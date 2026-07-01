#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_74(
        var_alphaav: f64,
        var_atatbot_d: f64,
        var_btat__blk1570: f64,
        var_btat__blk1570_dn11: f64,
        var_btat__blk1570_dn12: f64,
        var_btat__blk1570_dn6: f64,
        var_btat__blk1570_dn7: f64,
        var_btat__blk1570_dn8: f64,
        var_btat__blk1570_dn9: f64,
        var_cbbtbotd_i: f64,
        var_fbbtbot_d: f64,
        var_guard1589: f64,
        var_guard1590: f64,
        var_guard1678: f64,
        var_guard1682: f64,
        var_one_minus_pbot_d: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_pbrbotd_i: f64,
        var_twoatatoverthreebtat__blk1571: f64,
        var_twoatatoverthreebtat__blk1571_dn11: f64,
        var_twoatatoverthreebtat__blk1571_dn12: f64,
        var_twoatatoverthreebtat__blk1571_dn6: f64,
        var_twoatatoverthreebtat__blk1571_dn7: f64,
        var_twoatatoverthreebtat__blk1571_dn8: f64,
        var_twoatatoverthreebtat__blk1571_dn9: f64,
        var_umaxbeforelimiting__blk1572: f64,
        var_umaxbeforelimiting__blk1572_dn11: f64,
        var_umaxbeforelimiting__blk1572_dn12: f64,
        var_umaxbeforelimiting__blk1572_dn6: f64,
        var_umaxbeforelimiting__blk1572_dn7: f64,
        var_umaxbeforelimiting__blk1572_dn8: f64,
        var_umaxbeforelimiting__blk1572_dn9: f64,
        var_vav__blk1559: f64,
        var_vav__blk1559_dn11: f64,
        var_vav__blk1559_dn12: f64,
        var_vav__blk1559_dn7: f64,
        var_vav__blk1559_dn8: f64,
        var_vbbt__blk1558: f64,
        var_vbbt__blk1558_dn11: f64,
        var_vbbt__blk1558_dn12: f64,
        var_vbbt__blk1558_dn7: f64,
        var_vbbt__blk1558_dn8: f64,
        var_vbirbotd_i: f64,
        var_vbirbotinv_d: f64,
        var_vbrbotd_i: f64,
        var_vbrinvbot_d: f64,
        var_wdepnulrinvbot_d: f64,
        var_fmaxr__blk1585_slot: &mut f64,
        var_fmaxr__blk1585_dn11_slot: &mut f64,
        var_fmaxr__blk1585_dn12_slot: &mut f64,
        var_fmaxr__blk1585_dn6_slot: &mut f64,
        var_fmaxr__blk1585_dn7_slot: &mut f64,
        var_fmaxr__blk1585_dn8_slot: &mut f64,
        var_fmaxr__blk1585_dn9_slot: &mut f64,
        var_fmaxr__blk1585_rv_slot: &mut f64,
        var_guard1685_slot: &mut f64,
        var_guard1685_rv_slot: &mut f64,
        var_guard1686_slot: &mut f64,
        var_guard1686_rv_slot: &mut f64,
        var_guard1687_slot: &mut f64,
        var_guard1687_rv_slot: &mut f64,
        var_guard1688_slot: &mut f64,
        var_guard1688_rv_slot: &mut f64,
        var_guard1689_slot: &mut f64,
        var_guard1689_rv_slot: &mut f64,
        var_guard1690_slot: &mut f64,
        var_guard1690_rv_slot: &mut f64,
        var_guard1691_slot: &mut f64,
        var_guard1691_rv_slot: &mut f64,
        var_guard1692_slot: &mut f64,
        var_guard1692_rv_slot: &mut f64,
        var_guard1693_slot: &mut f64,
        var_guard1693_rv_slot: &mut f64,
        var_guard1694_slot: &mut f64,
        var_guard1694_rv_slot: &mut f64,
        var_guard1695_slot: &mut f64,
        var_guard1695_rv_slot: &mut f64,
        var_ktat__blk1578_slot: &mut f64,
        var_ktat__blk1578_dn11_slot: &mut f64,
        var_ktat__blk1578_dn12_slot: &mut f64,
        var_ktat__blk1578_dn6_slot: &mut f64,
        var_ktat__blk1578_dn7_slot: &mut f64,
        var_ktat__blk1578_dn8_slot: &mut f64,
        var_ktat__blk1578_dn9_slot: &mut f64,
        var_ktat__blk1578_rv_slot: &mut f64,
        var_ltat__blk1579_slot: &mut f64,
        var_ltat__blk1579_dn11_slot: &mut f64,
        var_ltat__blk1579_dn12_slot: &mut f64,
        var_ltat__blk1579_dn6_slot: &mut f64,
        var_ltat__blk1579_dn7_slot: &mut f64,
        var_ltat__blk1579_dn8_slot: &mut f64,
        var_ltat__blk1579_dn9_slot: &mut f64,
        var_ltat__blk1579_rv_slot: &mut f64,
        var_mtat__blk1580_slot: &mut f64,
        var_mtat__blk1580_dn11_slot: &mut f64,
        var_mtat__blk1580_dn12_slot: &mut f64,
        var_mtat__blk1580_dn6_slot: &mut f64,
        var_mtat__blk1580_dn7_slot: &mut f64,
        var_mtat__blk1580_dn8_slot: &mut f64,
        var_mtat__blk1580_dn9_slot: &mut f64,
        var_mtat__blk1580_rv_slot: &mut f64,
        var_sqrtumax__blk1574_slot: &mut f64,
        var_sqrtumax__blk1574_dn11_slot: &mut f64,
        var_sqrtumax__blk1574_dn12_slot: &mut f64,
        var_sqrtumax__blk1574_dn6_slot: &mut f64,
        var_sqrtumax__blk1574_dn7_slot: &mut f64,
        var_sqrtumax__blk1574_dn8_slot: &mut f64,
        var_sqrtumax__blk1574_dn9_slot: &mut f64,
        var_sqrtumax__blk1574_rv_slot: &mut f64,
        var_tmp__blk1560_slot: &mut f64,
        var_tmp__blk1560_dn11_slot: &mut f64,
        var_tmp__blk1560_dn12_slot: &mut f64,
        var_tmp__blk1560_dn6_slot: &mut f64,
        var_tmp__blk1560_dn7_slot: &mut f64,
        var_tmp__blk1560_dn8_slot: &mut f64,
        var_tmp__blk1560_dn9_slot: &mut f64,
        var_tmp__blk1560_rv_slot: &mut f64,
        var_umax__blk1573_slot: &mut f64,
        var_umax__blk1573_dn11_slot: &mut f64,
        var_umax__blk1573_dn12_slot: &mut f64,
        var_umax__blk1573_dn6_slot: &mut f64,
        var_umax__blk1573_dn7_slot: &mut f64,
        var_umax__blk1573_dn8_slot: &mut f64,
        var_umax__blk1573_dn9_slot: &mut f64,
        var_umax__blk1573_rv_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn11_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn12_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn6_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn7_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn8_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn9_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_rv_slot: &mut f64,
        var_xerfc__blk1581_slot: &mut f64,
        var_xerfc__blk1581_dn11_slot: &mut f64,
        var_xerfc__blk1581_dn12_slot: &mut f64,
        var_xerfc__blk1581_dn6_slot: &mut f64,
        var_xerfc__blk1581_dn7_slot: &mut f64,
        var_xerfc__blk1581_dn8_slot: &mut f64,
        var_xerfc__blk1581_dn9_slot: &mut f64,
        var_xerfc__blk1581_rv_slot: &mut f64,
        var_ysq__blk1542_slot: &mut f64,
        var_ysq__blk1542_dn11_slot: &mut f64,
        var_ysq__blk1542_dn12_slot: &mut f64,
        var_ysq__blk1542_dn6_slot: &mut f64,
        var_ysq__blk1542_dn7_slot: &mut f64,
        var_ysq__blk1542_dn8_slot: &mut f64,
        var_ysq__blk1542_dn9_slot: &mut f64,
        var_ysq__blk1542_rv_slot: &mut f64,
    ) {
        let mut var_fmaxr__blk1585: f64 = *var_fmaxr__blk1585_slot;
        let mut var_fmaxr__blk1585_dn11: f64 = *var_fmaxr__blk1585_dn11_slot;
        let mut var_fmaxr__blk1585_dn12: f64 = *var_fmaxr__blk1585_dn12_slot;
        let mut var_fmaxr__blk1585_dn6: f64 = *var_fmaxr__blk1585_dn6_slot;
        let mut var_fmaxr__blk1585_dn7: f64 = *var_fmaxr__blk1585_dn7_slot;
        let mut var_fmaxr__blk1585_dn8: f64 = *var_fmaxr__blk1585_dn8_slot;
        let mut var_fmaxr__blk1585_dn9: f64 = *var_fmaxr__blk1585_dn9_slot;
        let mut var_fmaxr__blk1585_rv: f64 = *var_fmaxr__blk1585_rv_slot;
        let mut var_guard1685: f64 = *var_guard1685_slot;
        let mut var_guard1685_rv: f64 = *var_guard1685_rv_slot;
        let mut var_guard1686: f64 = *var_guard1686_slot;
        let mut var_guard1686_rv: f64 = *var_guard1686_rv_slot;
        let mut var_guard1687: f64 = *var_guard1687_slot;
        let mut var_guard1687_rv: f64 = *var_guard1687_rv_slot;
        let mut var_guard1688: f64 = *var_guard1688_slot;
        let mut var_guard1688_rv: f64 = *var_guard1688_rv_slot;
        let mut var_guard1689: f64 = *var_guard1689_slot;
        let mut var_guard1689_rv: f64 = *var_guard1689_rv_slot;
        let mut var_guard1690: f64 = *var_guard1690_slot;
        let mut var_guard1690_rv: f64 = *var_guard1690_rv_slot;
        let mut var_guard1691: f64 = *var_guard1691_slot;
        let mut var_guard1691_rv: f64 = *var_guard1691_rv_slot;
        let mut var_guard1692: f64 = *var_guard1692_slot;
        let mut var_guard1692_rv: f64 = *var_guard1692_rv_slot;
        let mut var_guard1693: f64 = *var_guard1693_slot;
        let mut var_guard1693_rv: f64 = *var_guard1693_rv_slot;
        let mut var_guard1694: f64 = *var_guard1694_slot;
        let mut var_guard1694_rv: f64 = *var_guard1694_rv_slot;
        let mut var_guard1695: f64 = *var_guard1695_slot;
        let mut var_guard1695_rv: f64 = *var_guard1695_rv_slot;
        let mut var_ktat__blk1578: f64 = *var_ktat__blk1578_slot;
        let mut var_ktat__blk1578_dn11: f64 = *var_ktat__blk1578_dn11_slot;
        let mut var_ktat__blk1578_dn12: f64 = *var_ktat__blk1578_dn12_slot;
        let mut var_ktat__blk1578_dn6: f64 = *var_ktat__blk1578_dn6_slot;
        let mut var_ktat__blk1578_dn7: f64 = *var_ktat__blk1578_dn7_slot;
        let mut var_ktat__blk1578_dn8: f64 = *var_ktat__blk1578_dn8_slot;
        let mut var_ktat__blk1578_dn9: f64 = *var_ktat__blk1578_dn9_slot;
        let mut var_ktat__blk1578_rv: f64 = *var_ktat__blk1578_rv_slot;
        let mut var_ltat__blk1579: f64 = *var_ltat__blk1579_slot;
        let mut var_ltat__blk1579_dn11: f64 = *var_ltat__blk1579_dn11_slot;
        let mut var_ltat__blk1579_dn12: f64 = *var_ltat__blk1579_dn12_slot;
        let mut var_ltat__blk1579_dn6: f64 = *var_ltat__blk1579_dn6_slot;
        let mut var_ltat__blk1579_dn7: f64 = *var_ltat__blk1579_dn7_slot;
        let mut var_ltat__blk1579_dn8: f64 = *var_ltat__blk1579_dn8_slot;
        let mut var_ltat__blk1579_dn9: f64 = *var_ltat__blk1579_dn9_slot;
        let mut var_ltat__blk1579_rv: f64 = *var_ltat__blk1579_rv_slot;
        let mut var_mtat__blk1580: f64 = *var_mtat__blk1580_slot;
        let mut var_mtat__blk1580_dn11: f64 = *var_mtat__blk1580_dn11_slot;
        let mut var_mtat__blk1580_dn12: f64 = *var_mtat__blk1580_dn12_slot;
        let mut var_mtat__blk1580_dn6: f64 = *var_mtat__blk1580_dn6_slot;
        let mut var_mtat__blk1580_dn7: f64 = *var_mtat__blk1580_dn7_slot;
        let mut var_mtat__blk1580_dn8: f64 = *var_mtat__blk1580_dn8_slot;
        let mut var_mtat__blk1580_dn9: f64 = *var_mtat__blk1580_dn9_slot;
        let mut var_mtat__blk1580_rv: f64 = *var_mtat__blk1580_rv_slot;
        let mut var_sqrtumax__blk1574: f64 = *var_sqrtumax__blk1574_slot;
        let mut var_sqrtumax__blk1574_dn11: f64 = *var_sqrtumax__blk1574_dn11_slot;
        let mut var_sqrtumax__blk1574_dn12: f64 = *var_sqrtumax__blk1574_dn12_slot;
        let mut var_sqrtumax__blk1574_dn6: f64 = *var_sqrtumax__blk1574_dn6_slot;
        let mut var_sqrtumax__blk1574_dn7: f64 = *var_sqrtumax__blk1574_dn7_slot;
        let mut var_sqrtumax__blk1574_dn8: f64 = *var_sqrtumax__blk1574_dn8_slot;
        let mut var_sqrtumax__blk1574_dn9: f64 = *var_sqrtumax__blk1574_dn9_slot;
        let mut var_sqrtumax__blk1574_rv: f64 = *var_sqrtumax__blk1574_rv_slot;
        let mut var_tmp__blk1560: f64 = *var_tmp__blk1560_slot;
        let mut var_tmp__blk1560_dn11: f64 = *var_tmp__blk1560_dn11_slot;
        let mut var_tmp__blk1560_dn12: f64 = *var_tmp__blk1560_dn12_slot;
        let mut var_tmp__blk1560_dn6: f64 = *var_tmp__blk1560_dn6_slot;
        let mut var_tmp__blk1560_dn7: f64 = *var_tmp__blk1560_dn7_slot;
        let mut var_tmp__blk1560_dn8: f64 = *var_tmp__blk1560_dn8_slot;
        let mut var_tmp__blk1560_dn9: f64 = *var_tmp__blk1560_dn9_slot;
        let mut var_tmp__blk1560_rv: f64 = *var_tmp__blk1560_rv_slot;
        let mut var_umax__blk1573: f64 = *var_umax__blk1573_slot;
        let mut var_umax__blk1573_dn11: f64 = *var_umax__blk1573_dn11_slot;
        let mut var_umax__blk1573_dn12: f64 = *var_umax__blk1573_dn12_slot;
        let mut var_umax__blk1573_dn6: f64 = *var_umax__blk1573_dn6_slot;
        let mut var_umax__blk1573_dn7: f64 = *var_umax__blk1573_dn7_slot;
        let mut var_umax__blk1573_dn8: f64 = *var_umax__blk1573_dn8_slot;
        let mut var_umax__blk1573_dn9: f64 = *var_umax__blk1573_dn9_slot;
        let mut var_umax__blk1573_rv: f64 = *var_umax__blk1573_rv_slot;
        let mut var_umaxpoweronepointfive__blk1575: f64 = *var_umaxpoweronepointfive__blk1575_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn11: f64 = *var_umaxpoweronepointfive__blk1575_dn11_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn12: f64 = *var_umaxpoweronepointfive__blk1575_dn12_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn6: f64 = *var_umaxpoweronepointfive__blk1575_dn6_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn7: f64 = *var_umaxpoweronepointfive__blk1575_dn7_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn8: f64 = *var_umaxpoweronepointfive__blk1575_dn8_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn9: f64 = *var_umaxpoweronepointfive__blk1575_dn9_slot;
        let mut var_umaxpoweronepointfive__blk1575_rv: f64 = *var_umaxpoweronepointfive__blk1575_rv_slot;
        let mut var_xerfc__blk1581: f64 = *var_xerfc__blk1581_slot;
        let mut var_xerfc__blk1581_dn11: f64 = *var_xerfc__blk1581_dn11_slot;
        let mut var_xerfc__blk1581_dn12: f64 = *var_xerfc__blk1581_dn12_slot;
        let mut var_xerfc__blk1581_dn6: f64 = *var_xerfc__blk1581_dn6_slot;
        let mut var_xerfc__blk1581_dn7: f64 = *var_xerfc__blk1581_dn7_slot;
        let mut var_xerfc__blk1581_dn8: f64 = *var_xerfc__blk1581_dn8_slot;
        let mut var_xerfc__blk1581_dn9: f64 = *var_xerfc__blk1581_dn9_slot;
        let mut var_xerfc__blk1581_rv: f64 = *var_xerfc__blk1581_rv_slot;
        let mut var_ysq__blk1542: f64 = *var_ysq__blk1542_slot;
        let mut var_ysq__blk1542_dn11: f64 = *var_ysq__blk1542_dn11_slot;
        let mut var_ysq__blk1542_dn12: f64 = *var_ysq__blk1542_dn12_slot;
        let mut var_ysq__blk1542_dn6: f64 = *var_ysq__blk1542_dn6_slot;
        let mut var_ysq__blk1542_dn7: f64 = *var_ysq__blk1542_dn7_slot;
        let mut var_ysq__blk1542_dn8: f64 = *var_ysq__blk1542_dn8_slot;
        let mut var_ysq__blk1542_dn9: f64 = *var_ysq__blk1542_dn9_slot;
        let mut var_ysq__blk1542_rv: f64 = *var_ysq__blk1542_rv_slot;

        let (assign59710_e76771, assign59710_e76771_d_n6, assign59710_e76771_d_n7, assign59710_e76771_d_n8, assign59710_e76771_d_n9, assign59710_e76771_d_n11, assign59710_e76771_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1682 == 0.0)) {
        let assign59710_e76762: f64 = (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572);
        let assign59710_e76765: f64 = (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572);
        let assign59710_e76767: f64 = (assign59710_e76765 + 1.0);
        let assign59710_e76768: f64 = (assign59710_e76762 / assign59710_e76767);
        let assign59710_e76769: f64 = (assign59710_e76768).sqrt();
        (assign59710_e76769, ((((((var_umaxbeforelimiting__blk1572_dn6 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn6)) * assign59710_e76767) - (assign59710_e76762 * ((var_umaxbeforelimiting__blk1572_dn6 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn6)))) / (assign59710_e76767 * assign59710_e76767)) / (2.0 * assign59710_e76769)), ((((((var_umaxbeforelimiting__blk1572_dn7 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn7)) * assign59710_e76767) - (assign59710_e76762 * ((var_umaxbeforelimiting__blk1572_dn7 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn7)))) / (assign59710_e76767 * assign59710_e76767)) / (2.0 * assign59710_e76769)), ((((((var_umaxbeforelimiting__blk1572_dn8 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn8)) * assign59710_e76767) - (assign59710_e76762 * ((var_umaxbeforelimiting__blk1572_dn8 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn8)))) / (assign59710_e76767 * assign59710_e76767)) / (2.0 * assign59710_e76769)), ((((((var_umaxbeforelimiting__blk1572_dn9 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn9)) * assign59710_e76767) - (assign59710_e76762 * ((var_umaxbeforelimiting__blk1572_dn9 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn9)))) / (assign59710_e76767 * assign59710_e76767)) / (2.0 * assign59710_e76769)), ((((((var_umaxbeforelimiting__blk1572_dn11 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn11)) * assign59710_e76767) - (assign59710_e76762 * ((var_umaxbeforelimiting__blk1572_dn11 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn11)))) / (assign59710_e76767 * assign59710_e76767)) / (2.0 * assign59710_e76769)), ((((((var_umaxbeforelimiting__blk1572_dn12 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn12)) * assign59710_e76767) - (assign59710_e76762 * ((var_umaxbeforelimiting__blk1572_dn12 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn12)))) / (assign59710_e76767 * assign59710_e76767)) / (2.0 * assign59710_e76769)),)
    } else {
        (var_umax__blk1573, var_umax__blk1573_dn6, var_umax__blk1573_dn7, var_umax__blk1573_dn8, var_umax__blk1573_dn9, var_umax__blk1573_dn11, var_umax__blk1573_dn12,)
    }
};
        var_umax__blk1573 = assign59710_e76771;
        var_umax__blk1573_dn6 = assign59710_e76771_d_n6;
        var_umax__blk1573_dn7 = assign59710_e76771_d_n7;
        var_umax__blk1573_dn8 = assign59710_e76771_d_n8;
        var_umax__blk1573_dn9 = assign59710_e76771_d_n9;
        var_umax__blk1573_dn11 = assign59710_e76771_d_n11;
        var_umax__blk1573_dn12 = assign59710_e76771_d_n12;
        var_umax__blk1573_rv = 0.0;

        let (assign59720_e76785, assign59720_e76785_d_n6, assign59720_e76785_d_n7, assign59720_e76785_d_n8, assign59720_e76785_d_n9, assign59720_e76785_d_n11, assign59720_e76785_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1682 == 0.0)) {
        let assign59720_e76783: f64 = (var_umax__blk1573).sqrt();
        (assign59720_e76783, (var_umax__blk1573_dn6 / (2.0 * assign59720_e76783)), (var_umax__blk1573_dn7 / (2.0 * assign59720_e76783)), (var_umax__blk1573_dn8 / (2.0 * assign59720_e76783)), (var_umax__blk1573_dn9 / (2.0 * assign59720_e76783)), (var_umax__blk1573_dn11 / (2.0 * assign59720_e76783)), (var_umax__blk1573_dn12 / (2.0 * assign59720_e76783)),)
    } else {
        (var_sqrtumax__blk1574, var_sqrtumax__blk1574_dn6, var_sqrtumax__blk1574_dn7, var_sqrtumax__blk1574_dn8, var_sqrtumax__blk1574_dn9, var_sqrtumax__blk1574_dn11, var_sqrtumax__blk1574_dn12,)
    }
};
        var_sqrtumax__blk1574 = assign59720_e76785;
        var_sqrtumax__blk1574_dn6 = assign59720_e76785_d_n6;
        var_sqrtumax__blk1574_dn7 = assign59720_e76785_d_n7;
        var_sqrtumax__blk1574_dn8 = assign59720_e76785_d_n8;
        var_sqrtumax__blk1574_dn9 = assign59720_e76785_d_n9;
        var_sqrtumax__blk1574_dn11 = assign59720_e76785_d_n11;
        var_sqrtumax__blk1574_dn12 = assign59720_e76785_d_n12;
        var_sqrtumax__blk1574_rv = 0.0;

        let (assign59730_e76800, assign59730_e76800_d_n6, assign59730_e76800_d_n7, assign59730_e76800_d_n8, assign59730_e76800_d_n9, assign59730_e76800_d_n11, assign59730_e76800_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1682 == 0.0)) {
        let assign59730_e76798: f64 = (var_umax__blk1573 * var_sqrtumax__blk1574);
        (assign59730_e76798, ((var_umax__blk1573_dn6 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn6)), ((var_umax__blk1573_dn7 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn7)), ((var_umax__blk1573_dn8 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn8)), ((var_umax__blk1573_dn9 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn9)), ((var_umax__blk1573_dn11 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn11)), ((var_umax__blk1573_dn12 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn12)),)
    } else {
        (var_umaxpoweronepointfive__blk1575, var_umaxpoweronepointfive__blk1575_dn6, var_umaxpoweronepointfive__blk1575_dn7, var_umaxpoweronepointfive__blk1575_dn8, var_umaxpoweronepointfive__blk1575_dn9, var_umaxpoweronepointfive__blk1575_dn11, var_umaxpoweronepointfive__blk1575_dn12,)
    }
};
        var_umaxpoweronepointfive__blk1575 = assign59730_e76800;
        var_umaxpoweronepointfive__blk1575_dn6 = assign59730_e76800_d_n6;
        var_umaxpoweronepointfive__blk1575_dn7 = assign59730_e76800_d_n7;
        var_umaxpoweronepointfive__blk1575_dn8 = assign59730_e76800_d_n8;
        var_umaxpoweronepointfive__blk1575_dn9 = assign59730_e76800_d_n9;
        var_umaxpoweronepointfive__blk1575_dn11 = assign59730_e76800_d_n11;
        var_umaxpoweronepointfive__blk1575_dn12 = assign59730_e76800_d_n12;
        var_umaxpoweronepointfive__blk1575_rv = 0.0;

        let (assign59780_e76890, assign59780_e76890_d_n6, assign59780_e76890_d_n7, assign59780_e76890_d_n8, assign59780_e76890_d_n9, assign59780_e76890_d_n11, assign59780_e76890_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1682 == 0.0)) {
        let assign59780_e76886: f64 = (var_btat__blk1570 / var_sqrtumax__blk1574);
        let assign59780_e76887: f64 = (0.375 * assign59780_e76886);
        let assign59780_e76888: f64 = (assign59780_e76887).sqrt();
        (assign59780_e76888, ((0.375 * (((var_btat__blk1570_dn6 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn6)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign59780_e76888)), ((0.375 * (((var_btat__blk1570_dn7 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn7)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign59780_e76888)), ((0.375 * (((var_btat__blk1570_dn8 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn8)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign59780_e76888)), ((0.375 * (((var_btat__blk1570_dn9 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn9)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign59780_e76888)), ((0.375 * (((var_btat__blk1570_dn11 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn11)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign59780_e76888)), ((0.375 * (((var_btat__blk1570_dn12 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn12)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign59780_e76888)),)
    } else {
        (var_ktat__blk1578, var_ktat__blk1578_dn6, var_ktat__blk1578_dn7, var_ktat__blk1578_dn8, var_ktat__blk1578_dn9, var_ktat__blk1578_dn11, var_ktat__blk1578_dn12,)
    }
};
        var_ktat__blk1578 = assign59780_e76890;
        var_ktat__blk1578_dn6 = assign59780_e76890_d_n6;
        var_ktat__blk1578_dn7 = assign59780_e76890_d_n7;
        var_ktat__blk1578_dn8 = assign59780_e76890_d_n8;
        var_ktat__blk1578_dn9 = assign59780_e76890_d_n9;
        var_ktat__blk1578_dn11 = assign59780_e76890_d_n11;
        var_ktat__blk1578_dn12 = assign59780_e76890_d_n12;
        var_ktat__blk1578_rv = 0.0;

        let (assign59790_e76909, assign59790_e76909_d_n6, assign59790_e76909_d_n7, assign59790_e76909_d_n8, assign59790_e76909_d_n9, assign59790_e76909_d_n11, assign59790_e76909_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1682 == 0.0)) {
        let assign59790_e76904: f64 = (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574);
        let assign59790_e76905: f64 = (2.0 * assign59790_e76904);
        let assign59790_e76907: f64 = (assign59790_e76905 - var_umax__blk1573);
        (assign59790_e76907, ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn6 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn6))) - var_umax__blk1573_dn6), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn7 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn7))) - var_umax__blk1573_dn7), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn8 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn8))) - var_umax__blk1573_dn8), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn9 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn9))) - var_umax__blk1573_dn9), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn11 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn11))) - var_umax__blk1573_dn11), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn12 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn12))) - var_umax__blk1573_dn12),)
    } else {
        (var_ltat__blk1579, var_ltat__blk1579_dn6, var_ltat__blk1579_dn7, var_ltat__blk1579_dn8, var_ltat__blk1579_dn9, var_ltat__blk1579_dn11, var_ltat__blk1579_dn12,)
    }
};
        var_ltat__blk1579 = assign59790_e76909;
        var_ltat__blk1579_dn6 = assign59790_e76909_d_n6;
        var_ltat__blk1579_dn7 = assign59790_e76909_d_n7;
        var_ltat__blk1579_dn8 = assign59790_e76909_d_n8;
        var_ltat__blk1579_dn9 = assign59790_e76909_d_n9;
        var_ltat__blk1579_dn11 = assign59790_e76909_d_n11;
        var_ltat__blk1579_dn12 = assign59790_e76909_d_n12;
        var_ltat__blk1579_rv = 0.0;

        let (assign59800_e76936, assign59800_e76936_d_n6, assign59800_e76936_d_n7, assign59800_e76936_d_n8, assign59800_e76936_d_n9, assign59800_e76936_d_n11, assign59800_e76936_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1682 == 0.0)) {
        let assign59800_e76922: f64 = (var_atatbot_d * var_twoatatoverthreebtat__blk1571);
        let assign59800_e76924: f64 = (assign59800_e76922 * var_sqrtumax__blk1574);
        let assign59800_e76927: f64 = (var_atatbot_d * var_umax__blk1573);
        let assign59800_e76928: f64 = (assign59800_e76924 - assign59800_e76927);
        let assign59800_e76932: f64 = (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575);
        let assign59800_e76933: f64 = (0.5 * assign59800_e76932);
        let assign59800_e76934: f64 = (assign59800_e76928 + assign59800_e76933);
        (assign59800_e76934, (((((var_atatbot_d * var_twoatatoverthreebtat__blk1571_dn6) * var_sqrtumax__blk1574) + (assign59800_e76922 * var_sqrtumax__blk1574_dn6)) - (var_atatbot_d * var_umax__blk1573_dn6)) + (0.5 * ((var_btat__blk1570_dn6 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat__blk1571_dn7) * var_sqrtumax__blk1574) + (assign59800_e76922 * var_sqrtumax__blk1574_dn7)) - (var_atatbot_d * var_umax__blk1573_dn7)) + (0.5 * ((var_btat__blk1570_dn7 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat__blk1571_dn8) * var_sqrtumax__blk1574) + (assign59800_e76922 * var_sqrtumax__blk1574_dn8)) - (var_atatbot_d * var_umax__blk1573_dn8)) + (0.5 * ((var_btat__blk1570_dn8 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn8)))), (((((var_atatbot_d * var_twoatatoverthreebtat__blk1571_dn9) * var_sqrtumax__blk1574) + (assign59800_e76922 * var_sqrtumax__blk1574_dn9)) - (var_atatbot_d * var_umax__blk1573_dn9)) + (0.5 * ((var_btat__blk1570_dn9 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn9)))), (((((var_atatbot_d * var_twoatatoverthreebtat__blk1571_dn11) * var_sqrtumax__blk1574) + (assign59800_e76922 * var_sqrtumax__blk1574_dn11)) - (var_atatbot_d * var_umax__blk1573_dn11)) + (0.5 * ((var_btat__blk1570_dn11 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn11)))), (((((var_atatbot_d * var_twoatatoverthreebtat__blk1571_dn12) * var_sqrtumax__blk1574) + (assign59800_e76922 * var_sqrtumax__blk1574_dn12)) - (var_atatbot_d * var_umax__blk1573_dn12)) + (0.5 * ((var_btat__blk1570_dn12 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn12)))),)
    } else {
        (var_mtat__blk1580, var_mtat__blk1580_dn6, var_mtat__blk1580_dn7, var_mtat__blk1580_dn8, var_mtat__blk1580_dn9, var_mtat__blk1580_dn11, var_mtat__blk1580_dn12,)
    }
};
        var_mtat__blk1580 = assign59800_e76936;
        var_mtat__blk1580_dn6 = assign59800_e76936_d_n6;
        var_mtat__blk1580_dn7 = assign59800_e76936_d_n7;
        var_mtat__blk1580_dn8 = assign59800_e76936_d_n8;
        var_mtat__blk1580_dn9 = assign59800_e76936_d_n9;
        var_mtat__blk1580_dn11 = assign59800_e76936_d_n11;
        var_mtat__blk1580_dn12 = assign59800_e76936_d_n12;
        var_mtat__blk1580_rv = 0.0;

        let (assign59810_e76953, assign59810_e76953_d_n6, assign59810_e76953_d_n7, assign59810_e76953_d_n8, assign59810_e76953_d_n9, assign59810_e76953_d_n11, assign59810_e76953_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1682 == 0.0)) {
        let assign59810_e76949: f64 = (var_ltat__blk1579 - 1.0);
        let assign59810_e76951: f64 = (assign59810_e76949 * var_ktat__blk1578);
        (assign59810_e76951, ((var_ltat__blk1579_dn6 * var_ktat__blk1578) + (assign59810_e76949 * var_ktat__blk1578_dn6)), ((var_ltat__blk1579_dn7 * var_ktat__blk1578) + (assign59810_e76949 * var_ktat__blk1578_dn7)), ((var_ltat__blk1579_dn8 * var_ktat__blk1578) + (assign59810_e76949 * var_ktat__blk1578_dn8)), ((var_ltat__blk1579_dn9 * var_ktat__blk1578) + (assign59810_e76949 * var_ktat__blk1578_dn9)), ((var_ltat__blk1579_dn11 * var_ktat__blk1578) + (assign59810_e76949 * var_ktat__blk1578_dn11)), ((var_ltat__blk1579_dn12 * var_ktat__blk1578) + (assign59810_e76949 * var_ktat__blk1578_dn12)),)
    } else {
        (var_xerfc__blk1581, var_xerfc__blk1581_dn6, var_xerfc__blk1581_dn7, var_xerfc__blk1581_dn8, var_xerfc__blk1581_dn9, var_xerfc__blk1581_dn11, var_xerfc__blk1581_dn12,)
    }
};
        var_xerfc__blk1581 = assign59810_e76953;
        var_xerfc__blk1581_dn6 = assign59810_e76953_d_n6;
        var_xerfc__blk1581_dn7 = assign59810_e76953_d_n7;
        var_xerfc__blk1581_dn8 = assign59810_e76953_d_n8;
        var_xerfc__blk1581_dn9 = assign59810_e76953_d_n9;
        var_xerfc__blk1581_dn11 = assign59810_e76953_d_n11;
        var_xerfc__blk1581_dn12 = assign59810_e76953_d_n12;
        var_xerfc__blk1581_rv = 0.0;

        let (assign59820_e76968, assign59820_e76968_d_n6, assign59820_e76968_d_n7, assign59820_e76968_d_n8, assign59820_e76968_d_n9, assign59820_e76968_d_n11, assign59820_e76968_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1682 == 0.0)) {
        let assign59820_e76966: f64 = (var_xerfc__blk1581 * var_xerfc__blk1581);
        (assign59820_e76966, ((var_xerfc__blk1581_dn6 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn6)), ((var_xerfc__blk1581_dn7 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn7)), ((var_xerfc__blk1581_dn8 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn8)), ((var_xerfc__blk1581_dn9 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn9)), ((var_xerfc__blk1581_dn11 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn11)), ((var_xerfc__blk1581_dn12 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn12)),)
    } else {
        (var_ysq__blk1542, var_ysq__blk1542_dn6, var_ysq__blk1542_dn7, var_ysq__blk1542_dn8, var_ysq__blk1542_dn9, var_ysq__blk1542_dn11, var_ysq__blk1542_dn12,)
    }
};
        var_ysq__blk1542 = assign59820_e76968;
        var_ysq__blk1542_dn6 = assign59820_e76968_d_n6;
        var_ysq__blk1542_dn7 = assign59820_e76968_d_n7;
        var_ysq__blk1542_dn8 = assign59820_e76968_d_n8;
        var_ysq__blk1542_dn9 = assign59820_e76968_d_n9;
        var_ysq__blk1542_dn11 = assign59820_e76968_d_n11;
        var_ysq__blk1542_dn12 = assign59820_e76968_d_n12;
        var_ysq__blk1542_rv = 0.0;

        let assign59860_e77016: f64 = (-var_ysq__blk1542);
        let assign59860_e77018: f64 = (assign59860_e77016 + var_mtat__blk1580);
        let assign59860_e77020: f64 = (-230.25850929940458);
        let assign59860_e77021: f64 = if assign59860_e77018 > assign59860_e77020 { 1.0 } else { 0.0 };
        var_guard1685 = assign59860_e77021;
        var_guard1685_rv = 0.0;

        let (assign59870_e77040, assign59870_e77040_d_n6, assign59870_e77040_d_n7, assign59870_e77040_d_n8, assign59870_e77040_d_n9, assign59870_e77040_d_n11, assign59870_e77040_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1682 == 0.0)) && (var_guard1685 != 0.0)) {
        let assign59870_e77035: f64 = (-var_ysq__blk1542);
        let assign59870_e77037: f64 = (assign59870_e77035 + var_mtat__blk1580);
        let assign59870_e77038: f64 = (assign59870_e77037).exp();
        (assign59870_e77038, (assign59870_e77038 * ((-var_ysq__blk1542_dn6) + var_mtat__blk1580_dn6)), (assign59870_e77038 * ((-var_ysq__blk1542_dn7) + var_mtat__blk1580_dn7)), (assign59870_e77038 * ((-var_ysq__blk1542_dn8) + var_mtat__blk1580_dn8)), (assign59870_e77038 * ((-var_ysq__blk1542_dn9) + var_mtat__blk1580_dn9)), (assign59870_e77038 * ((-var_ysq__blk1542_dn11) + var_mtat__blk1580_dn11)), (assign59870_e77038 * ((-var_ysq__blk1542_dn12) + var_mtat__blk1580_dn12)),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign59870_e77040;
        var_tmp__blk1560_dn6 = assign59870_e77040_d_n6;
        var_tmp__blk1560_dn7 = assign59870_e77040_d_n7;
        var_tmp__blk1560_dn8 = assign59870_e77040_d_n8;
        var_tmp__blk1560_dn9 = assign59870_e77040_d_n9;
        var_tmp__blk1560_dn11 = assign59870_e77040_d_n11;
        var_tmp__blk1560_dn12 = assign59870_e77040_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let (assign59880_e77090, assign59880_e77090_d_n6, assign59880_e77090_d_n7, assign59880_e77090_d_n8, assign59880_e77090_d_n9, assign59880_e77090_d_n11, assign59880_e77090_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1682 == 0.0)) && (var_guard1685 == 0.0)) {
        let assign59880_e77057: f64 = (-230.25850929940458);
        let assign59880_e77059: f64 = (-var_ysq__blk1542);
        let assign59880_e77061: f64 = (assign59880_e77059 + var_mtat__blk1580);
        let assign59880_e77062: f64 = (assign59880_e77057 - assign59880_e77061);
        let assign59880_e77066: f64 = (-230.25850929940458);
        let assign59880_e77068: f64 = (-var_ysq__blk1542);
        let assign59880_e77070: f64 = (assign59880_e77068 + var_mtat__blk1580);
        let assign59880_e77071: f64 = (assign59880_e77066 - assign59880_e77070);
        let assign59880_e77074: f64 = (-230.25850929940458);
        let assign59880_e77076: f64 = (-var_ysq__blk1542);
        let assign59880_e77078: f64 = (assign59880_e77076 + var_mtat__blk1580);
        let assign59880_e77079: f64 = (assign59880_e77074 - assign59880_e77078);
        let assign59880_e77081: f64 = (assign59880_e77079 * 0.3333333333333333);
        let assign59880_e77082: f64 = (1.0 + assign59880_e77081);
        let assign59880_e77083: f64 = (assign59880_e77071 * assign59880_e77082);
        let assign59880_e77084: f64 = (0.5 * assign59880_e77083);
        let assign59880_e77085: f64 = (1.0 + assign59880_e77084);
        let assign59880_e77086: f64 = (assign59880_e77062 * assign59880_e77085);
        let assign59880_e77087: f64 = (1.0 + assign59880_e77086);
        let assign59880_e77088: f64 = (1e-100 / assign59880_e77087);
        (assign59880_e77088, (-((1e-100 * (((-((-var_ysq__blk1542_dn6) + var_mtat__blk1580_dn6)) * assign59880_e77085) + (assign59880_e77062 * (0.5 * (((-((-var_ysq__blk1542_dn6) + var_mtat__blk1580_dn6)) * assign59880_e77082) + (assign59880_e77071 * ((-((-var_ysq__blk1542_dn6) + var_mtat__blk1580_dn6)) * 0.3333333333333333))))))) / (assign59880_e77087 * assign59880_e77087))), (-((1e-100 * (((-((-var_ysq__blk1542_dn7) + var_mtat__blk1580_dn7)) * assign59880_e77085) + (assign59880_e77062 * (0.5 * (((-((-var_ysq__blk1542_dn7) + var_mtat__blk1580_dn7)) * assign59880_e77082) + (assign59880_e77071 * ((-((-var_ysq__blk1542_dn7) + var_mtat__blk1580_dn7)) * 0.3333333333333333))))))) / (assign59880_e77087 * assign59880_e77087))), (-((1e-100 * (((-((-var_ysq__blk1542_dn8) + var_mtat__blk1580_dn8)) * assign59880_e77085) + (assign59880_e77062 * (0.5 * (((-((-var_ysq__blk1542_dn8) + var_mtat__blk1580_dn8)) * assign59880_e77082) + (assign59880_e77071 * ((-((-var_ysq__blk1542_dn8) + var_mtat__blk1580_dn8)) * 0.3333333333333333))))))) / (assign59880_e77087 * assign59880_e77087))), (-((1e-100 * (((-((-var_ysq__blk1542_dn9) + var_mtat__blk1580_dn9)) * assign59880_e77085) + (assign59880_e77062 * (0.5 * (((-((-var_ysq__blk1542_dn9) + var_mtat__blk1580_dn9)) * assign59880_e77082) + (assign59880_e77071 * ((-((-var_ysq__blk1542_dn9) + var_mtat__blk1580_dn9)) * 0.3333333333333333))))))) / (assign59880_e77087 * assign59880_e77087))), (-((1e-100 * (((-((-var_ysq__blk1542_dn11) + var_mtat__blk1580_dn11)) * assign59880_e77085) + (assign59880_e77062 * (0.5 * (((-((-var_ysq__blk1542_dn11) + var_mtat__blk1580_dn11)) * assign59880_e77082) + (assign59880_e77071 * ((-((-var_ysq__blk1542_dn11) + var_mtat__blk1580_dn11)) * 0.3333333333333333))))))) / (assign59880_e77087 * assign59880_e77087))), (-((1e-100 * (((-((-var_ysq__blk1542_dn12) + var_mtat__blk1580_dn12)) * assign59880_e77085) + (assign59880_e77062 * (0.5 * (((-((-var_ysq__blk1542_dn12) + var_mtat__blk1580_dn12)) * assign59880_e77082) + (assign59880_e77071 * ((-((-var_ysq__blk1542_dn12) + var_mtat__blk1580_dn12)) * 0.3333333333333333))))))) / (assign59880_e77087 * assign59880_e77087))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign59880_e77090;
        var_tmp__blk1560_dn6 = assign59880_e77090_d_n6;
        var_tmp__blk1560_dn7 = assign59880_e77090_d_n7;
        var_tmp__blk1560_dn8 = assign59880_e77090_d_n8;
        var_tmp__blk1560_dn9 = assign59880_e77090_d_n9;
        var_tmp__blk1560_dn11 = assign59880_e77090_d_n11;
        var_tmp__blk1560_dn12 = assign59880_e77090_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let assign59900_e77124: f64 = if var_xerfc__blk1581 > 0.0 { 1.0 } else { 0.0 };
        var_guard1686 = assign59900_e77124;
        var_guard1686_rv = 0.0;

        let assign59920_e77142: f64 = (-230.25850929940458);
        let assign59920_e77143: f64 = if var_mtat__blk1580 > assign59920_e77142 { 1.0 } else { 0.0 };
        var_guard1687 = assign59920_e77143;
        var_guard1687_rv = 0.0;

        let (assign59930_e77162, assign59930_e77162_d_n6, assign59930_e77162_d_n7, assign59930_e77162_d_n8, assign59930_e77162_d_n9, assign59930_e77162_d_n11, assign59930_e77162_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1682 == 0.0)) && (var_guard1686 == 0.0)) && (var_guard1687 != 0.0)) {
        let assign59930_e77160: f64 = (var_mtat__blk1580).exp();
        (assign59930_e77160, (assign59930_e77160 * var_mtat__blk1580_dn6), (assign59930_e77160 * var_mtat__blk1580_dn7), (assign59930_e77160 * var_mtat__blk1580_dn8), (assign59930_e77160 * var_mtat__blk1580_dn9), (assign59930_e77160 * var_mtat__blk1580_dn11), (assign59930_e77160 * var_mtat__blk1580_dn12),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign59930_e77162;
        var_tmp__blk1560_dn6 = assign59930_e77162_d_n6;
        var_tmp__blk1560_dn7 = assign59930_e77162_d_n7;
        var_tmp__blk1560_dn8 = assign59930_e77162_d_n8;
        var_tmp__blk1560_dn9 = assign59930_e77162_d_n9;
        var_tmp__blk1560_dn11 = assign59930_e77162_d_n11;
        var_tmp__blk1560_dn12 = assign59930_e77162_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let (assign59940_e77206, assign59940_e77206_d_n6, assign59940_e77206_d_n7, assign59940_e77206_d_n8, assign59940_e77206_d_n9, assign59940_e77206_d_n11, assign59940_e77206_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1682 == 0.0)) && (var_guard1686 == 0.0)) && (var_guard1687 == 0.0)) {
        let assign59940_e77182: f64 = (-230.25850929940458);
        let assign59940_e77184: f64 = (assign59940_e77182 - var_mtat__blk1580);
        let assign59940_e77188: f64 = (-230.25850929940458);
        let assign59940_e77190: f64 = (assign59940_e77188 - var_mtat__blk1580);
        let assign59940_e77193: f64 = (-230.25850929940458);
        let assign59940_e77195: f64 = (assign59940_e77193 - var_mtat__blk1580);
        let assign59940_e77197: f64 = (assign59940_e77195 * 0.3333333333333333);
        let assign59940_e77198: f64 = (1.0 + assign59940_e77197);
        let assign59940_e77199: f64 = (assign59940_e77190 * assign59940_e77198);
        let assign59940_e77200: f64 = (0.5 * assign59940_e77199);
        let assign59940_e77201: f64 = (1.0 + assign59940_e77200);
        let assign59940_e77202: f64 = (assign59940_e77184 * assign59940_e77201);
        let assign59940_e77203: f64 = (1.0 + assign59940_e77202);
        let assign59940_e77204: f64 = (1e-100 / assign59940_e77203);
        (assign59940_e77204, (-((1e-100 * (((-var_mtat__blk1580_dn6) * assign59940_e77201) + (assign59940_e77184 * (0.5 * (((-var_mtat__blk1580_dn6) * assign59940_e77198) + (assign59940_e77190 * ((-var_mtat__blk1580_dn6) * 0.3333333333333333))))))) / (assign59940_e77203 * assign59940_e77203))), (-((1e-100 * (((-var_mtat__blk1580_dn7) * assign59940_e77201) + (assign59940_e77184 * (0.5 * (((-var_mtat__blk1580_dn7) * assign59940_e77198) + (assign59940_e77190 * ((-var_mtat__blk1580_dn7) * 0.3333333333333333))))))) / (assign59940_e77203 * assign59940_e77203))), (-((1e-100 * (((-var_mtat__blk1580_dn8) * assign59940_e77201) + (assign59940_e77184 * (0.5 * (((-var_mtat__blk1580_dn8) * assign59940_e77198) + (assign59940_e77190 * ((-var_mtat__blk1580_dn8) * 0.3333333333333333))))))) / (assign59940_e77203 * assign59940_e77203))), (-((1e-100 * (((-var_mtat__blk1580_dn9) * assign59940_e77201) + (assign59940_e77184 * (0.5 * (((-var_mtat__blk1580_dn9) * assign59940_e77198) + (assign59940_e77190 * ((-var_mtat__blk1580_dn9) * 0.3333333333333333))))))) / (assign59940_e77203 * assign59940_e77203))), (-((1e-100 * (((-var_mtat__blk1580_dn11) * assign59940_e77201) + (assign59940_e77184 * (0.5 * (((-var_mtat__blk1580_dn11) * assign59940_e77198) + (assign59940_e77190 * ((-var_mtat__blk1580_dn11) * 0.3333333333333333))))))) / (assign59940_e77203 * assign59940_e77203))), (-((1e-100 * (((-var_mtat__blk1580_dn12) * assign59940_e77201) + (assign59940_e77184 * (0.5 * (((-var_mtat__blk1580_dn12) * assign59940_e77198) + (assign59940_e77190 * ((-var_mtat__blk1580_dn12) * 0.3333333333333333))))))) / (assign59940_e77203 * assign59940_e77203))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign59940_e77206;
        var_tmp__blk1560_dn6 = assign59940_e77206_d_n6;
        var_tmp__blk1560_dn7 = assign59940_e77206_d_n7;
        var_tmp__blk1560_dn8 = assign59940_e77206_d_n8;
        var_tmp__blk1560_dn9 = assign59940_e77206_d_n9;
        var_tmp__blk1560_dn11 = assign59940_e77206_d_n11;
        var_tmp__blk1560_dn12 = assign59940_e77206_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let assign59980_e77269: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard1688 = assign59980_e77269;
        var_guard1688_rv = 0.0;

        let assign60000_e77284: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard1689 = assign60000_e77284;
        var_guard1689_rv = 0.0;

        let (assign60010_e77304, assign60010_e77304_d_n6, assign60010_e77304_d_n7, assign60010_e77304_d_n8, assign60010_e77304_d_n9, assign60010_e77304_d_n11, assign60010_e77304_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1688 == 0.0)) && (var_guard1689 != 0.0)) {
        let assign60010_e77299: f64 = (var_vbirbotd_i - var_vbbt__blk1558);
        let assign60010_e77301: f64 = (assign60010_e77299 * var_vbirbotinv_d);
        let assign60010_e77302: f64 = (assign60010_e77301).sqrt();
        (assign60010_e77302, 0.0, (((-var_vbbt__blk1558_dn7) * var_vbirbotinv_d) / (2.0 * assign60010_e77302)), (((-var_vbbt__blk1558_dn8) * var_vbirbotinv_d) / (2.0 * assign60010_e77302)), 0.0, (((-var_vbbt__blk1558_dn11) * var_vbirbotinv_d) / (2.0 * assign60010_e77302)), (((-var_vbbt__blk1558_dn12) * var_vbirbotinv_d) / (2.0 * assign60010_e77302)),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60010_e77304;
        var_tmp__blk1560_dn6 = assign60010_e77304_d_n6;
        var_tmp__blk1560_dn7 = assign60010_e77304_d_n7;
        var_tmp__blk1560_dn8 = assign60010_e77304_d_n8;
        var_tmp__blk1560_dn9 = assign60010_e77304_d_n9;
        var_tmp__blk1560_dn11 = assign60010_e77304_d_n11;
        var_tmp__blk1560_dn12 = assign60010_e77304_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let (assign60020_e77326, assign60020_e77326_d_n6, assign60020_e77326_d_n7, assign60020_e77326_d_n8, assign60020_e77326_d_n9, assign60020_e77326_d_n11, assign60020_e77326_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1688 == 0.0)) && (var_guard1689 == 0.0)) {
        let assign60020_e77320: f64 = (var_vbirbotd_i - var_vbbt__blk1558);
        let assign60020_e77322: f64 = (assign60020_e77320 * var_vbirbotinv_d);
        let assign60020_e77324: f64 = (assign60020_e77322).powf(var_pbotd_i);
        (assign60020_e77324, 0.0, if 0.0 == 0.0 && ((var_pbotd_i) as f64).is_finite() && ((var_pbotd_i) as f64).fract() == 0.0 { if var_pbotd_i == 0.0 { 0.0 } else { (var_pbotd_i * ((assign60020_e77322).powf(var_pbotd_i - 1.0) * ((-var_vbbt__blk1558_dn7) * var_vbirbotinv_d))) } } else { (assign60020_e77324 * (var_pbotd_i * (((-var_vbbt__blk1558_dn7) * var_vbirbotinv_d) / assign60020_e77322))) }, if 0.0 == 0.0 && ((var_pbotd_i) as f64).is_finite() && ((var_pbotd_i) as f64).fract() == 0.0 { if var_pbotd_i == 0.0 { 0.0 } else { (var_pbotd_i * ((assign60020_e77322).powf(var_pbotd_i - 1.0) * ((-var_vbbt__blk1558_dn8) * var_vbirbotinv_d))) } } else { (assign60020_e77324 * (var_pbotd_i * (((-var_vbbt__blk1558_dn8) * var_vbirbotinv_d) / assign60020_e77322))) }, 0.0, if 0.0 == 0.0 && ((var_pbotd_i) as f64).is_finite() && ((var_pbotd_i) as f64).fract() == 0.0 { if var_pbotd_i == 0.0 { 0.0 } else { (var_pbotd_i * ((assign60020_e77322).powf(var_pbotd_i - 1.0) * ((-var_vbbt__blk1558_dn11) * var_vbirbotinv_d))) } } else { (assign60020_e77324 * (var_pbotd_i * (((-var_vbbt__blk1558_dn11) * var_vbirbotinv_d) / assign60020_e77322))) }, if 0.0 == 0.0 && ((var_pbotd_i) as f64).is_finite() && ((var_pbotd_i) as f64).fract() == 0.0 { if var_pbotd_i == 0.0 { 0.0 } else { (var_pbotd_i * ((assign60020_e77322).powf(var_pbotd_i - 1.0) * ((-var_vbbt__blk1558_dn12) * var_vbirbotinv_d))) } } else { (assign60020_e77324 * (var_pbotd_i * (((-var_vbbt__blk1558_dn12) * var_vbirbotinv_d) / assign60020_e77322))) },)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60020_e77326;
        var_tmp__blk1560_dn6 = assign60020_e77326_d_n6;
        var_tmp__blk1560_dn7 = assign60020_e77326_d_n7;
        var_tmp__blk1560_dn8 = assign60020_e77326_d_n8;
        var_tmp__blk1560_dn9 = assign60020_e77326_d_n9;
        var_tmp__blk1560_dn11 = assign60020_e77326_d_n11;
        var_tmp__blk1560_dn12 = assign60020_e77326_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let (assign60030_e77347, assign60030_e77347_d_n6, assign60030_e77347_d_n7, assign60030_e77347_d_n8, assign60030_e77347_d_n9, assign60030_e77347_d_n11, assign60030_e77347_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1688 == 0.0)) {
        let assign60030_e77340: f64 = (var_vbirbotd_i - var_vbbt__blk1558);
        let assign60030_e77342: f64 = (assign60030_e77340 * var_wdepnulrinvbot_d);
        let assign60030_e77344: f64 = (assign60030_e77342 / var_tmp__blk1560);
        let assign60030_e77345: f64 = (var_one_over_one_minus_pbot_d * assign60030_e77344);
        (assign60030_e77345, (var_one_over_one_minus_pbot_d * (-((assign60030_e77342 * var_tmp__blk1560_dn6) / (var_tmp__blk1560 * var_tmp__blk1560)))), (var_one_over_one_minus_pbot_d * (((((-var_vbbt__blk1558_dn7) * var_wdepnulrinvbot_d) * var_tmp__blk1560) - (assign60030_e77342 * var_tmp__blk1560_dn7)) / (var_tmp__blk1560 * var_tmp__blk1560))), (var_one_over_one_minus_pbot_d * (((((-var_vbbt__blk1558_dn8) * var_wdepnulrinvbot_d) * var_tmp__blk1560) - (assign60030_e77342 * var_tmp__blk1560_dn8)) / (var_tmp__blk1560 * var_tmp__blk1560))), (var_one_over_one_minus_pbot_d * (-((assign60030_e77342 * var_tmp__blk1560_dn9) / (var_tmp__blk1560 * var_tmp__blk1560)))), (var_one_over_one_minus_pbot_d * (((((-var_vbbt__blk1558_dn11) * var_wdepnulrinvbot_d) * var_tmp__blk1560) - (assign60030_e77342 * var_tmp__blk1560_dn11)) / (var_tmp__blk1560 * var_tmp__blk1560))), (var_one_over_one_minus_pbot_d * (((((-var_vbbt__blk1558_dn12) * var_wdepnulrinvbot_d) * var_tmp__blk1560) - (assign60030_e77342 * var_tmp__blk1560_dn12)) / (var_tmp__blk1560 * var_tmp__blk1560))),)
    } else {
        (var_fmaxr__blk1585, var_fmaxr__blk1585_dn6, var_fmaxr__blk1585_dn7, var_fmaxr__blk1585_dn8, var_fmaxr__blk1585_dn9, var_fmaxr__blk1585_dn11, var_fmaxr__blk1585_dn12,)
    }
};
        var_fmaxr__blk1585 = assign60030_e77347;
        var_fmaxr__blk1585_dn6 = assign60030_e77347_d_n6;
        var_fmaxr__blk1585_dn7 = assign60030_e77347_d_n7;
        var_fmaxr__blk1585_dn8 = assign60030_e77347_d_n8;
        var_fmaxr__blk1585_dn9 = assign60030_e77347_d_n9;
        var_fmaxr__blk1585_dn11 = assign60030_e77347_d_n11;
        var_fmaxr__blk1585_dn12 = assign60030_e77347_d_n12;
        var_fmaxr__blk1585_rv = 0.0;

        let assign60040_e77349: f64 = (-var_fbbtbot_d);
        let assign60040_e77351: f64 = (assign60040_e77349 / var_fmaxr__blk1585);
        let assign60040_e77352: f64 = (assign60040_e77351).abs();
        let assign60040_e77354: f64 = if assign60040_e77352 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1690 = assign60040_e77354;
        var_guard1690_rv = 0.0;

        let (assign60050_e77373, assign60050_e77373_d_n6, assign60050_e77373_d_n7, assign60050_e77373_d_n8, assign60050_e77373_d_n9, assign60050_e77373_d_n11, assign60050_e77373_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1688 == 0.0)) && (var_guard1690 != 0.0)) {
        let assign60050_e77368: f64 = (-var_fbbtbot_d);
        let assign60050_e77370: f64 = (assign60050_e77368 / var_fmaxr__blk1585);
        let assign60050_e77371: f64 = (assign60050_e77370).exp();
        (assign60050_e77371, (assign60050_e77371 * (-((assign60050_e77368 * var_fmaxr__blk1585_dn6) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))), (assign60050_e77371 * (-((assign60050_e77368 * var_fmaxr__blk1585_dn7) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))), (assign60050_e77371 * (-((assign60050_e77368 * var_fmaxr__blk1585_dn8) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))), (assign60050_e77371 * (-((assign60050_e77368 * var_fmaxr__blk1585_dn9) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))), (assign60050_e77371 * (-((assign60050_e77368 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))), (assign60050_e77371 * (-((assign60050_e77368 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60050_e77373;
        var_tmp__blk1560_dn6 = assign60050_e77373_d_n6;
        var_tmp__blk1560_dn7 = assign60050_e77373_d_n7;
        var_tmp__blk1560_dn8 = assign60050_e77373_d_n8;
        var_tmp__blk1560_dn9 = assign60050_e77373_d_n9;
        var_tmp__blk1560_dn11 = assign60050_e77373_d_n11;
        var_tmp__blk1560_dn12 = assign60050_e77373_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let assign60060_e77375: f64 = (-var_fbbtbot_d);
        let assign60060_e77377: f64 = (assign60060_e77375 / var_fmaxr__blk1585);
        let assign60060_e77379: f64 = if assign60060_e77377 < 0.0 { 1.0 } else { 0.0 };
        var_guard1691 = assign60060_e77379;
        var_guard1691_rv = 0.0;

        let (assign60070_e77431, assign60070_e77431_d_n6, assign60070_e77431_d_n7, assign60070_e77431_d_n8, assign60070_e77431_d_n9, assign60070_e77431_d_n11, assign60070_e77431_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1688 == 0.0)) && (var_guard1690 == 0.0)) && (var_guard1691 != 0.0)) {
        let assign60070_e77398: f64 = (-230.25850929940458);
        let assign60070_e77400: f64 = (-var_fbbtbot_d);
        let assign60070_e77402: f64 = (assign60070_e77400 / var_fmaxr__blk1585);
        let assign60070_e77403: f64 = (assign60070_e77398 - assign60070_e77402);
        let assign60070_e77407: f64 = (-230.25850929940458);
        let assign60070_e77409: f64 = (-var_fbbtbot_d);
        let assign60070_e77411: f64 = (assign60070_e77409 / var_fmaxr__blk1585);
        let assign60070_e77412: f64 = (assign60070_e77407 - assign60070_e77411);
        let assign60070_e77415: f64 = (-230.25850929940458);
        let assign60070_e77417: f64 = (-var_fbbtbot_d);
        let assign60070_e77419: f64 = (assign60070_e77417 / var_fmaxr__blk1585);
        let assign60070_e77420: f64 = (assign60070_e77415 - assign60070_e77419);
        let assign60070_e77422: f64 = (assign60070_e77420 * 0.3333333333333333);
        let assign60070_e77423: f64 = (1.0 + assign60070_e77422);
        let assign60070_e77424: f64 = (assign60070_e77412 * assign60070_e77423);
        let assign60070_e77425: f64 = (0.5 * assign60070_e77424);
        let assign60070_e77426: f64 = (1.0 + assign60070_e77425);
        let assign60070_e77427: f64 = (assign60070_e77403 * assign60070_e77426);
        let assign60070_e77428: f64 = (1.0 + assign60070_e77427);
        let assign60070_e77429: f64 = (1e-100 / assign60070_e77428);
        (assign60070_e77429, (-((1e-100 * (((-(-((assign60070_e77400 * var_fmaxr__blk1585_dn6) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60070_e77426) + (assign60070_e77403 * (0.5 * (((-(-((assign60070_e77409 * var_fmaxr__blk1585_dn6) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60070_e77423) + (assign60070_e77412 * ((-(-((assign60070_e77417 * var_fmaxr__blk1585_dn6) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * 0.3333333333333333))))))) / (assign60070_e77428 * assign60070_e77428))), (-((1e-100 * (((-(-((assign60070_e77400 * var_fmaxr__blk1585_dn7) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60070_e77426) + (assign60070_e77403 * (0.5 * (((-(-((assign60070_e77409 * var_fmaxr__blk1585_dn7) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60070_e77423) + (assign60070_e77412 * ((-(-((assign60070_e77417 * var_fmaxr__blk1585_dn7) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * 0.3333333333333333))))))) / (assign60070_e77428 * assign60070_e77428))), (-((1e-100 * (((-(-((assign60070_e77400 * var_fmaxr__blk1585_dn8) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60070_e77426) + (assign60070_e77403 * (0.5 * (((-(-((assign60070_e77409 * var_fmaxr__blk1585_dn8) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60070_e77423) + (assign60070_e77412 * ((-(-((assign60070_e77417 * var_fmaxr__blk1585_dn8) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * 0.3333333333333333))))))) / (assign60070_e77428 * assign60070_e77428))), (-((1e-100 * (((-(-((assign60070_e77400 * var_fmaxr__blk1585_dn9) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60070_e77426) + (assign60070_e77403 * (0.5 * (((-(-((assign60070_e77409 * var_fmaxr__blk1585_dn9) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60070_e77423) + (assign60070_e77412 * ((-(-((assign60070_e77417 * var_fmaxr__blk1585_dn9) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * 0.3333333333333333))))))) / (assign60070_e77428 * assign60070_e77428))), (-((1e-100 * (((-(-((assign60070_e77400 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60070_e77426) + (assign60070_e77403 * (0.5 * (((-(-((assign60070_e77409 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60070_e77423) + (assign60070_e77412 * ((-(-((assign60070_e77417 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * 0.3333333333333333))))))) / (assign60070_e77428 * assign60070_e77428))), (-((1e-100 * (((-(-((assign60070_e77400 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60070_e77426) + (assign60070_e77403 * (0.5 * (((-(-((assign60070_e77409 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60070_e77423) + (assign60070_e77412 * ((-(-((assign60070_e77417 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * 0.3333333333333333))))))) / (assign60070_e77428 * assign60070_e77428))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60070_e77431;
        var_tmp__blk1560_dn6 = assign60070_e77431_d_n6;
        var_tmp__blk1560_dn7 = assign60070_e77431_d_n7;
        var_tmp__blk1560_dn8 = assign60070_e77431_d_n8;
        var_tmp__blk1560_dn9 = assign60070_e77431_d_n9;
        var_tmp__blk1560_dn11 = assign60070_e77431_d_n11;
        var_tmp__blk1560_dn12 = assign60070_e77431_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let (assign60080_e77481, assign60080_e77481_d_n6, assign60080_e77481_d_n7, assign60080_e77481_d_n8, assign60080_e77481_d_n9, assign60080_e77481_d_n11, assign60080_e77481_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1688 == 0.0)) && (var_guard1690 == 0.0)) && (var_guard1691 == 0.0)) {
        let assign60080_e77451: f64 = (-var_fbbtbot_d);
        let assign60080_e77453: f64 = (assign60080_e77451 / var_fmaxr__blk1585);
        let assign60080_e77455: f64 = (assign60080_e77453 - 230.25850929940458);
        let assign60080_e77459: f64 = (-var_fbbtbot_d);
        let assign60080_e77461: f64 = (assign60080_e77459 / var_fmaxr__blk1585);
        let assign60080_e77463: f64 = (assign60080_e77461 - 230.25850929940458);
        let assign60080_e77466: f64 = (-var_fbbtbot_d);
        let assign60080_e77468: f64 = (assign60080_e77466 / var_fmaxr__blk1585);
        let assign60080_e77470: f64 = (assign60080_e77468 - 230.25850929940458);
        let assign60080_e77472: f64 = (assign60080_e77470 * 0.3333333333333333);
        let assign60080_e77473: f64 = (1.0 + assign60080_e77472);
        let assign60080_e77474: f64 = (assign60080_e77463 * assign60080_e77473);
        let assign60080_e77475: f64 = (0.5 * assign60080_e77474);
        let assign60080_e77476: f64 = (1.0 + assign60080_e77475);
        let assign60080_e77477: f64 = (assign60080_e77455 * assign60080_e77476);
        let assign60080_e77478: f64 = (1.0 + assign60080_e77477);
        let assign60080_e77479: f64 = (1e100 * assign60080_e77478);
        (assign60080_e77479, (1e100 * (((-((assign60080_e77451 * var_fmaxr__blk1585_dn6) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60080_e77476) + (assign60080_e77455 * (0.5 * (((-((assign60080_e77459 * var_fmaxr__blk1585_dn6) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60080_e77473) + (assign60080_e77463 * ((-((assign60080_e77466 * var_fmaxr__blk1585_dn6) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))), (1e100 * (((-((assign60080_e77451 * var_fmaxr__blk1585_dn7) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60080_e77476) + (assign60080_e77455 * (0.5 * (((-((assign60080_e77459 * var_fmaxr__blk1585_dn7) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60080_e77473) + (assign60080_e77463 * ((-((assign60080_e77466 * var_fmaxr__blk1585_dn7) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))), (1e100 * (((-((assign60080_e77451 * var_fmaxr__blk1585_dn8) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60080_e77476) + (assign60080_e77455 * (0.5 * (((-((assign60080_e77459 * var_fmaxr__blk1585_dn8) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60080_e77473) + (assign60080_e77463 * ((-((assign60080_e77466 * var_fmaxr__blk1585_dn8) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))), (1e100 * (((-((assign60080_e77451 * var_fmaxr__blk1585_dn9) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60080_e77476) + (assign60080_e77455 * (0.5 * (((-((assign60080_e77459 * var_fmaxr__blk1585_dn9) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60080_e77473) + (assign60080_e77463 * ((-((assign60080_e77466 * var_fmaxr__blk1585_dn9) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))), (1e100 * (((-((assign60080_e77451 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60080_e77476) + (assign60080_e77455 * (0.5 * (((-((assign60080_e77459 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60080_e77473) + (assign60080_e77463 * ((-((assign60080_e77466 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))), (1e100 * (((-((assign60080_e77451 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60080_e77476) + (assign60080_e77455 * (0.5 * (((-((assign60080_e77459 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60080_e77473) + (assign60080_e77463 * ((-((assign60080_e77466 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60080_e77481;
        var_tmp__blk1560_dn6 = assign60080_e77481_d_n6;
        var_tmp__blk1560_dn7 = assign60080_e77481_d_n7;
        var_tmp__blk1560_dn8 = assign60080_e77481_d_n8;
        var_tmp__blk1560_dn9 = assign60080_e77481_d_n9;
        var_tmp__blk1560_dn11 = assign60080_e77481_d_n11;
        var_tmp__blk1560_dn12 = assign60080_e77481_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let assign60100_e77505: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard1692 = assign60100_e77505;
        var_guard1692_rv = 0.0;

        let assign60120_e77520: f64 = (-var_alphaav);
        let assign60120_e77522: f64 = (assign60120_e77520 * var_vbrbotd_i);
        let assign60120_e77523: f64 = if var_vav__blk1559 > assign60120_e77522 { 1.0 } else { 0.0 };
        var_guard1693 = assign60120_e77523;
        var_guard1693_rv = 0.0;

        let assign60130_e77526: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard1694 = assign60130_e77526;
        var_guard1694_rv = 0.0;

        let (assign60140_e77557, assign60140_e77557_d_n6, assign60140_e77557_d_n7, assign60140_e77557_d_n8, assign60140_e77557_d_n9, assign60140_e77557_d_n11, assign60140_e77557_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1692 == 0.0)) && (var_guard1693 != 0.0)) && (var_guard1694 != 0.0)) {
        let assign60140_e77543: f64 = (var_vav__blk1559 * var_vbrinvbot_d);
        let assign60140_e77546: f64 = (var_vav__blk1559 * var_vbrinvbot_d);
        let assign60140_e77547: f64 = (assign60140_e77543 * assign60140_e77546);
        let assign60140_e77550: f64 = (var_vav__blk1559 * var_vbrinvbot_d);
        let assign60140_e77551: f64 = (assign60140_e77547 * assign60140_e77550);
        let assign60140_e77554: f64 = (var_vav__blk1559 * var_vbrinvbot_d);
        let assign60140_e77555: f64 = (assign60140_e77551 * assign60140_e77554);
        (assign60140_e77555, 0.0, (((((((var_vav__blk1559_dn7 * var_vbrinvbot_d) * assign60140_e77546) + (assign60140_e77543 * (var_vav__blk1559_dn7 * var_vbrinvbot_d))) * assign60140_e77550) + (assign60140_e77547 * (var_vav__blk1559_dn7 * var_vbrinvbot_d))) * assign60140_e77554) + (assign60140_e77551 * (var_vav__blk1559_dn7 * var_vbrinvbot_d))), (((((((var_vav__blk1559_dn8 * var_vbrinvbot_d) * assign60140_e77546) + (assign60140_e77543 * (var_vav__blk1559_dn8 * var_vbrinvbot_d))) * assign60140_e77550) + (assign60140_e77547 * (var_vav__blk1559_dn8 * var_vbrinvbot_d))) * assign60140_e77554) + (assign60140_e77551 * (var_vav__blk1559_dn8 * var_vbrinvbot_d))), 0.0, (((((((var_vav__blk1559_dn11 * var_vbrinvbot_d) * assign60140_e77546) + (assign60140_e77543 * (var_vav__blk1559_dn11 * var_vbrinvbot_d))) * assign60140_e77550) + (assign60140_e77547 * (var_vav__blk1559_dn11 * var_vbrinvbot_d))) * assign60140_e77554) + (assign60140_e77551 * (var_vav__blk1559_dn11 * var_vbrinvbot_d))), (((((((var_vav__blk1559_dn12 * var_vbrinvbot_d) * assign60140_e77546) + (assign60140_e77543 * (var_vav__blk1559_dn12 * var_vbrinvbot_d))) * assign60140_e77550) + (assign60140_e77547 * (var_vav__blk1559_dn12 * var_vbrinvbot_d))) * assign60140_e77554) + (assign60140_e77551 * (var_vav__blk1559_dn12 * var_vbrinvbot_d))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60140_e77557;
        var_tmp__blk1560_dn6 = assign60140_e77557_d_n6;
        var_tmp__blk1560_dn7 = assign60140_e77557_d_n7;
        var_tmp__blk1560_dn8 = assign60140_e77557_d_n8;
        var_tmp__blk1560_dn9 = assign60140_e77557_d_n9;
        var_tmp__blk1560_dn11 = assign60140_e77557_d_n11;
        var_tmp__blk1560_dn12 = assign60140_e77557_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let (assign60150_e77580, assign60150_e77580_d_n6, assign60150_e77580_d_n7, assign60150_e77580_d_n8, assign60150_e77580_d_n9, assign60150_e77580_d_n11, assign60150_e77580_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1692 == 0.0)) && (var_guard1693 != 0.0)) && (var_guard1694 == 0.0)) {
        let assign60150_e77575: f64 = (var_vav__blk1559 * var_vbrinvbot_d);
        let assign60150_e77576: f64 = (assign60150_e77575).abs();
        let assign60150_e77578: f64 = (assign60150_e77576).powf(var_pbrbotd_i);
        (assign60150_e77578, 0.0, if 0.0 == 0.0 && ((var_pbrbotd_i) as f64).is_finite() && ((var_pbrbotd_i) as f64).fract() == 0.0 { if var_pbrbotd_i == 0.0 { 0.0 } else { (var_pbrbotd_i * ((assign60150_e77576).powf(var_pbrbotd_i - 1.0) * if assign60150_e77575 >= 0.0 { (var_vav__blk1559_dn7 * var_vbrinvbot_d) } else { (-(var_vav__blk1559_dn7 * var_vbrinvbot_d)) })) } } else { (assign60150_e77578 * (var_pbrbotd_i * (if assign60150_e77575 >= 0.0 { (var_vav__blk1559_dn7 * var_vbrinvbot_d) } else { (-(var_vav__blk1559_dn7 * var_vbrinvbot_d)) } / assign60150_e77576))) }, if 0.0 == 0.0 && ((var_pbrbotd_i) as f64).is_finite() && ((var_pbrbotd_i) as f64).fract() == 0.0 { if var_pbrbotd_i == 0.0 { 0.0 } else { (var_pbrbotd_i * ((assign60150_e77576).powf(var_pbrbotd_i - 1.0) * if assign60150_e77575 >= 0.0 { (var_vav__blk1559_dn8 * var_vbrinvbot_d) } else { (-(var_vav__blk1559_dn8 * var_vbrinvbot_d)) })) } } else { (assign60150_e77578 * (var_pbrbotd_i * (if assign60150_e77575 >= 0.0 { (var_vav__blk1559_dn8 * var_vbrinvbot_d) } else { (-(var_vav__blk1559_dn8 * var_vbrinvbot_d)) } / assign60150_e77576))) }, 0.0, if 0.0 == 0.0 && ((var_pbrbotd_i) as f64).is_finite() && ((var_pbrbotd_i) as f64).fract() == 0.0 { if var_pbrbotd_i == 0.0 { 0.0 } else { (var_pbrbotd_i * ((assign60150_e77576).powf(var_pbrbotd_i - 1.0) * if assign60150_e77575 >= 0.0 { (var_vav__blk1559_dn11 * var_vbrinvbot_d) } else { (-(var_vav__blk1559_dn11 * var_vbrinvbot_d)) })) } } else { (assign60150_e77578 * (var_pbrbotd_i * (if assign60150_e77575 >= 0.0 { (var_vav__blk1559_dn11 * var_vbrinvbot_d) } else { (-(var_vav__blk1559_dn11 * var_vbrinvbot_d)) } / assign60150_e77576))) }, if 0.0 == 0.0 && ((var_pbrbotd_i) as f64).is_finite() && ((var_pbrbotd_i) as f64).fract() == 0.0 { if var_pbrbotd_i == 0.0 { 0.0 } else { (var_pbrbotd_i * ((assign60150_e77576).powf(var_pbrbotd_i - 1.0) * if assign60150_e77575 >= 0.0 { (var_vav__blk1559_dn12 * var_vbrinvbot_d) } else { (-(var_vav__blk1559_dn12 * var_vbrinvbot_d)) })) } } else { (assign60150_e77578 * (var_pbrbotd_i * (if assign60150_e77575 >= 0.0 { (var_vav__blk1559_dn12 * var_vbrinvbot_d) } else { (-(var_vav__blk1559_dn12 * var_vbrinvbot_d)) } / assign60150_e77576))) },)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60150_e77580;
        var_tmp__blk1560_dn6 = assign60150_e77580_d_n6;
        var_tmp__blk1560_dn7 = assign60150_e77580_d_n7;
        var_tmp__blk1560_dn8 = assign60150_e77580_d_n8;
        var_tmp__blk1560_dn9 = assign60150_e77580_d_n9;
        var_tmp__blk1560_dn11 = assign60150_e77580_d_n11;
        var_tmp__blk1560_dn12 = assign60150_e77580_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let assign60190_e77646: f64 = if var_one_minus_pbot_d == 0.5 { 1.0 } else { 0.0 };
        var_guard1695 = assign60190_e77646;
        var_guard1695_rv = 0.0;

        *var_fmaxr__blk1585_slot = var_fmaxr__blk1585;
        *var_fmaxr__blk1585_dn11_slot = var_fmaxr__blk1585_dn11;
        *var_fmaxr__blk1585_dn12_slot = var_fmaxr__blk1585_dn12;
        *var_fmaxr__blk1585_dn6_slot = var_fmaxr__blk1585_dn6;
        *var_fmaxr__blk1585_dn7_slot = var_fmaxr__blk1585_dn7;
        *var_fmaxr__blk1585_dn8_slot = var_fmaxr__blk1585_dn8;
        *var_fmaxr__blk1585_dn9_slot = var_fmaxr__blk1585_dn9;
        *var_fmaxr__blk1585_rv_slot = var_fmaxr__blk1585_rv;
        *var_guard1685_slot = var_guard1685;
        *var_guard1685_rv_slot = var_guard1685_rv;
        *var_guard1686_slot = var_guard1686;
        *var_guard1686_rv_slot = var_guard1686_rv;
        *var_guard1687_slot = var_guard1687;
        *var_guard1687_rv_slot = var_guard1687_rv;
        *var_guard1688_slot = var_guard1688;
        *var_guard1688_rv_slot = var_guard1688_rv;
        *var_guard1689_slot = var_guard1689;
        *var_guard1689_rv_slot = var_guard1689_rv;
        *var_guard1690_slot = var_guard1690;
        *var_guard1690_rv_slot = var_guard1690_rv;
        *var_guard1691_slot = var_guard1691;
        *var_guard1691_rv_slot = var_guard1691_rv;
        *var_guard1692_slot = var_guard1692;
        *var_guard1692_rv_slot = var_guard1692_rv;
        *var_guard1693_slot = var_guard1693;
        *var_guard1693_rv_slot = var_guard1693_rv;
        *var_guard1694_slot = var_guard1694;
        *var_guard1694_rv_slot = var_guard1694_rv;
        *var_guard1695_slot = var_guard1695;
        *var_guard1695_rv_slot = var_guard1695_rv;
        *var_ktat__blk1578_slot = var_ktat__blk1578;
        *var_ktat__blk1578_dn11_slot = var_ktat__blk1578_dn11;
        *var_ktat__blk1578_dn12_slot = var_ktat__blk1578_dn12;
        *var_ktat__blk1578_dn6_slot = var_ktat__blk1578_dn6;
        *var_ktat__blk1578_dn7_slot = var_ktat__blk1578_dn7;
        *var_ktat__blk1578_dn8_slot = var_ktat__blk1578_dn8;
        *var_ktat__blk1578_dn9_slot = var_ktat__blk1578_dn9;
        *var_ktat__blk1578_rv_slot = var_ktat__blk1578_rv;
        *var_ltat__blk1579_slot = var_ltat__blk1579;
        *var_ltat__blk1579_dn11_slot = var_ltat__blk1579_dn11;
        *var_ltat__blk1579_dn12_slot = var_ltat__blk1579_dn12;
        *var_ltat__blk1579_dn6_slot = var_ltat__blk1579_dn6;
        *var_ltat__blk1579_dn7_slot = var_ltat__blk1579_dn7;
        *var_ltat__blk1579_dn8_slot = var_ltat__blk1579_dn8;
        *var_ltat__blk1579_dn9_slot = var_ltat__blk1579_dn9;
        *var_ltat__blk1579_rv_slot = var_ltat__blk1579_rv;
        *var_mtat__blk1580_slot = var_mtat__blk1580;
        *var_mtat__blk1580_dn11_slot = var_mtat__blk1580_dn11;
        *var_mtat__blk1580_dn12_slot = var_mtat__blk1580_dn12;
        *var_mtat__blk1580_dn6_slot = var_mtat__blk1580_dn6;
        *var_mtat__blk1580_dn7_slot = var_mtat__blk1580_dn7;
        *var_mtat__blk1580_dn8_slot = var_mtat__blk1580_dn8;
        *var_mtat__blk1580_dn9_slot = var_mtat__blk1580_dn9;
        *var_mtat__blk1580_rv_slot = var_mtat__blk1580_rv;
        *var_sqrtumax__blk1574_slot = var_sqrtumax__blk1574;
        *var_sqrtumax__blk1574_dn11_slot = var_sqrtumax__blk1574_dn11;
        *var_sqrtumax__blk1574_dn12_slot = var_sqrtumax__blk1574_dn12;
        *var_sqrtumax__blk1574_dn6_slot = var_sqrtumax__blk1574_dn6;
        *var_sqrtumax__blk1574_dn7_slot = var_sqrtumax__blk1574_dn7;
        *var_sqrtumax__blk1574_dn8_slot = var_sqrtumax__blk1574_dn8;
        *var_sqrtumax__blk1574_dn9_slot = var_sqrtumax__blk1574_dn9;
        *var_sqrtumax__blk1574_rv_slot = var_sqrtumax__blk1574_rv;
        *var_tmp__blk1560_slot = var_tmp__blk1560;
        *var_tmp__blk1560_dn11_slot = var_tmp__blk1560_dn11;
        *var_tmp__blk1560_dn12_slot = var_tmp__blk1560_dn12;
        *var_tmp__blk1560_dn6_slot = var_tmp__blk1560_dn6;
        *var_tmp__blk1560_dn7_slot = var_tmp__blk1560_dn7;
        *var_tmp__blk1560_dn8_slot = var_tmp__blk1560_dn8;
        *var_tmp__blk1560_dn9_slot = var_tmp__blk1560_dn9;
        *var_tmp__blk1560_rv_slot = var_tmp__blk1560_rv;
        *var_umax__blk1573_slot = var_umax__blk1573;
        *var_umax__blk1573_dn11_slot = var_umax__blk1573_dn11;
        *var_umax__blk1573_dn12_slot = var_umax__blk1573_dn12;
        *var_umax__blk1573_dn6_slot = var_umax__blk1573_dn6;
        *var_umax__blk1573_dn7_slot = var_umax__blk1573_dn7;
        *var_umax__blk1573_dn8_slot = var_umax__blk1573_dn8;
        *var_umax__blk1573_dn9_slot = var_umax__blk1573_dn9;
        *var_umax__blk1573_rv_slot = var_umax__blk1573_rv;
        *var_umaxpoweronepointfive__blk1575_slot = var_umaxpoweronepointfive__blk1575;
        *var_umaxpoweronepointfive__blk1575_dn11_slot = var_umaxpoweronepointfive__blk1575_dn11;
        *var_umaxpoweronepointfive__blk1575_dn12_slot = var_umaxpoweronepointfive__blk1575_dn12;
        *var_umaxpoweronepointfive__blk1575_dn6_slot = var_umaxpoweronepointfive__blk1575_dn6;
        *var_umaxpoweronepointfive__blk1575_dn7_slot = var_umaxpoweronepointfive__blk1575_dn7;
        *var_umaxpoweronepointfive__blk1575_dn8_slot = var_umaxpoweronepointfive__blk1575_dn8;
        *var_umaxpoweronepointfive__blk1575_dn9_slot = var_umaxpoweronepointfive__blk1575_dn9;
        *var_umaxpoweronepointfive__blk1575_rv_slot = var_umaxpoweronepointfive__blk1575_rv;
        *var_xerfc__blk1581_slot = var_xerfc__blk1581;
        *var_xerfc__blk1581_dn11_slot = var_xerfc__blk1581_dn11;
        *var_xerfc__blk1581_dn12_slot = var_xerfc__blk1581_dn12;
        *var_xerfc__blk1581_dn6_slot = var_xerfc__blk1581_dn6;
        *var_xerfc__blk1581_dn7_slot = var_xerfc__blk1581_dn7;
        *var_xerfc__blk1581_dn8_slot = var_xerfc__blk1581_dn8;
        *var_xerfc__blk1581_dn9_slot = var_xerfc__blk1581_dn9;
        *var_xerfc__blk1581_rv_slot = var_xerfc__blk1581_rv;
        *var_ysq__blk1542_slot = var_ysq__blk1542;
        *var_ysq__blk1542_dn11_slot = var_ysq__blk1542_dn11;
        *var_ysq__blk1542_dn12_slot = var_ysq__blk1542_dn12;
        *var_ysq__blk1542_dn6_slot = var_ysq__blk1542_dn6;
        *var_ysq__blk1542_dn7_slot = var_ysq__blk1542_dn7;
        *var_ysq__blk1542_dn8_slot = var_ysq__blk1542_dn8;
        *var_ysq__blk1542_dn9_slot = var_ysq__blk1542_dn9;
        *var_ysq__blk1542_rv_slot = var_ysq__blk1542_rv;
    }

    pub(super) fn stamp_reactive_block_75(
        p: &Parameters,
        var_atatsti_d: f64,
        var_btatpartsti_d: f64,
        var_cbbtstid_i: f64,
        var_csrhstid_i: f64,
        var_ctatstid_i: f64,
        var_guard1589: f64,
        var_guard1590: f64,
        var_guard1678: f64,
        var_guard1695: f64,
        var_lsdrain_i: f64,
        var_one_minus_pbot_d: f64,
        var_one_minus_psti_d: f64,
        var_pstid_i: f64,
        var_qpref2bot_d: f64,
        var_qprefbot_d: f64,
        var_vbbt__blk1558: f64,
        var_vbbt__blk1558_dn11: f64,
        var_vbbt__blk1558_dn12: f64,
        var_vbbt__blk1558_dn7: f64,
        var_vbbt__blk1558_dn8: f64,
        var_vbiinvbot_d: f64,
        var_vbirstid_i: f64,
        var_vbirstiinv_d: f64,
        var_vbisti_d: f64,
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
        var_wdepnulrsti_d: f64,
        var_btat__blk1570_slot: &mut f64,
        var_btat__blk1570_dn11_slot: &mut f64,
        var_btat__blk1570_dn12_slot: &mut f64,
        var_btat__blk1570_dn6_slot: &mut f64,
        var_btat__blk1570_dn7_slot: &mut f64,
        var_btat__blk1570_dn8_slot: &mut f64,
        var_btat__blk1570_dn9_slot: &mut f64,
        var_btat__blk1570_rv_slot: &mut f64,
        var_guard1696_slot: &mut f64,
        var_guard1696_rv_slot: &mut f64,
        var_guard1697_slot: &mut f64,
        var_guard1697_rv_slot: &mut f64,
        var_guard1699_slot: &mut f64,
        var_guard1699_rv_slot: &mut f64,
        var_guard1700_slot: &mut f64,
        var_guard1700_rv_slot: &mut f64,
        var_guard1703_slot: &mut f64,
        var_guard1703_rv_slot: &mut f64,
        var_guard1704_slot: &mut f64,
        var_guard1704_rv_slot: &mut f64,
        var_guard1705_slot: &mut f64,
        var_guard1705_rv_slot: &mut f64,
        var_guard1706_slot: &mut f64,
        var_guard1706_rv_slot: &mut f64,
        var_guard1707_slot: &mut f64,
        var_guard1707_rv_slot: &mut f64,
        var_ktat__blk1578_slot: &mut f64,
        var_ktat__blk1578_dn11_slot: &mut f64,
        var_ktat__blk1578_dn12_slot: &mut f64,
        var_ktat__blk1578_dn6_slot: &mut f64,
        var_ktat__blk1578_dn7_slot: &mut f64,
        var_ktat__blk1578_dn8_slot: &mut f64,
        var_ktat__blk1578_dn9_slot: &mut f64,
        var_ktat__blk1578_rv_slot: &mut f64,
        var_ltat__blk1579_slot: &mut f64,
        var_ltat__blk1579_dn11_slot: &mut f64,
        var_ltat__blk1579_dn12_slot: &mut f64,
        var_ltat__blk1579_dn6_slot: &mut f64,
        var_ltat__blk1579_dn7_slot: &mut f64,
        var_ltat__blk1579_dn8_slot: &mut f64,
        var_ltat__blk1579_dn9_slot: &mut f64,
        var_ltat__blk1579_rv_slot: &mut f64,
        var_mtat__blk1580_slot: &mut f64,
        var_mtat__blk1580_dn11_slot: &mut f64,
        var_mtat__blk1580_dn12_slot: &mut f64,
        var_mtat__blk1580_dn6_slot: &mut f64,
        var_mtat__blk1580_dn7_slot: &mut f64,
        var_mtat__blk1580_dn8_slot: &mut f64,
        var_mtat__blk1580_dn9_slot: &mut f64,
        var_mtat__blk1580_rv_slot: &mut f64,
        var_qjunbot_d_slot: &mut f64,
        var_qjunbot_d_dn11_slot: &mut f64,
        var_qjunbot_d_dn12_slot: &mut f64,
        var_qjunbot_d_dn6_slot: &mut f64,
        var_qjunbot_d_dn7_slot: &mut f64,
        var_qjunbot_d_dn8_slot: &mut f64,
        var_qjunbot_d_dn9_slot: &mut f64,
        var_qjunbot_d_rv_slot: &mut f64,
        var_qjunsti_d_slot: &mut f64,
        var_qjunsti_d_dn11_slot: &mut f64,
        var_qjunsti_d_dn12_slot: &mut f64,
        var_qjunsti_d_dn6_slot: &mut f64,
        var_qjunsti_d_dn7_slot: &mut f64,
        var_qjunsti_d_dn8_slot: &mut f64,
        var_qjunsti_d_dn9_slot: &mut f64,
        var_qjunsti_d_rv_slot: &mut f64,
        var_sqrtumax__blk1574_slot: &mut f64,
        var_sqrtumax__blk1574_dn11_slot: &mut f64,
        var_sqrtumax__blk1574_dn12_slot: &mut f64,
        var_sqrtumax__blk1574_dn6_slot: &mut f64,
        var_sqrtumax__blk1574_dn7_slot: &mut f64,
        var_sqrtumax__blk1574_dn8_slot: &mut f64,
        var_sqrtumax__blk1574_dn9_slot: &mut f64,
        var_sqrtumax__blk1574_rv_slot: &mut f64,
        var_tmp__blk1560_slot: &mut f64,
        var_tmp__blk1560_dn11_slot: &mut f64,
        var_tmp__blk1560_dn12_slot: &mut f64,
        var_tmp__blk1560_dn6_slot: &mut f64,
        var_tmp__blk1560_dn7_slot: &mut f64,
        var_tmp__blk1560_dn8_slot: &mut f64,
        var_tmp__blk1560_dn9_slot: &mut f64,
        var_tmp__blk1560_rv_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn11_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn12_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn6_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn7_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn8_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn9_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_rv_slot: &mut f64,
        var_umax__blk1573_slot: &mut f64,
        var_umax__blk1573_dn11_slot: &mut f64,
        var_umax__blk1573_dn12_slot: &mut f64,
        var_umax__blk1573_dn6_slot: &mut f64,
        var_umax__blk1573_dn7_slot: &mut f64,
        var_umax__blk1573_dn8_slot: &mut f64,
        var_umax__blk1573_dn9_slot: &mut f64,
        var_umax__blk1573_rv_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn11_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn12_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn6_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn7_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn8_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn9_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_rv_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn11_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn12_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn6_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn7_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn8_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn9_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_rv_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_dn11_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_dn12_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_dn7_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_rv_slot: &mut f64,
        var_wdep__blk1567_slot: &mut f64,
        var_wdep__blk1567_dn11_slot: &mut f64,
        var_wdep__blk1567_dn12_slot: &mut f64,
        var_wdep__blk1567_dn6_slot: &mut f64,
        var_wdep__blk1567_dn7_slot: &mut f64,
        var_wdep__blk1567_dn8_slot: &mut f64,
        var_wdep__blk1567_dn9_slot: &mut f64,
        var_wdep__blk1567_rv_slot: &mut f64,
        var_xerfc__blk1581_slot: &mut f64,
        var_xerfc__blk1581_dn11_slot: &mut f64,
        var_xerfc__blk1581_dn12_slot: &mut f64,
        var_xerfc__blk1581_dn6_slot: &mut f64,
        var_xerfc__blk1581_dn7_slot: &mut f64,
        var_xerfc__blk1581_dn8_slot: &mut f64,
        var_xerfc__blk1581_dn9_slot: &mut f64,
        var_xerfc__blk1581_rv_slot: &mut f64,
        var_ysq__blk1542_slot: &mut f64,
        var_ysq__blk1542_dn11_slot: &mut f64,
        var_ysq__blk1542_dn12_slot: &mut f64,
        var_ysq__blk1542_dn6_slot: &mut f64,
        var_ysq__blk1542_dn7_slot: &mut f64,
        var_ysq__blk1542_dn8_slot: &mut f64,
        var_ysq__blk1542_dn9_slot: &mut f64,
        var_ysq__blk1542_rv_slot: &mut f64,
    ) {
        let mut var_btat__blk1570: f64 = *var_btat__blk1570_slot;
        let mut var_btat__blk1570_dn11: f64 = *var_btat__blk1570_dn11_slot;
        let mut var_btat__blk1570_dn12: f64 = *var_btat__blk1570_dn12_slot;
        let mut var_btat__blk1570_dn6: f64 = *var_btat__blk1570_dn6_slot;
        let mut var_btat__blk1570_dn7: f64 = *var_btat__blk1570_dn7_slot;
        let mut var_btat__blk1570_dn8: f64 = *var_btat__blk1570_dn8_slot;
        let mut var_btat__blk1570_dn9: f64 = *var_btat__blk1570_dn9_slot;
        let mut var_btat__blk1570_rv: f64 = *var_btat__blk1570_rv_slot;
        let mut var_guard1696: f64 = *var_guard1696_slot;
        let mut var_guard1696_rv: f64 = *var_guard1696_rv_slot;
        let mut var_guard1697: f64 = *var_guard1697_slot;
        let mut var_guard1697_rv: f64 = *var_guard1697_rv_slot;
        let mut var_guard1699: f64 = *var_guard1699_slot;
        let mut var_guard1699_rv: f64 = *var_guard1699_rv_slot;
        let mut var_guard1700: f64 = *var_guard1700_slot;
        let mut var_guard1700_rv: f64 = *var_guard1700_rv_slot;
        let mut var_guard1703: f64 = *var_guard1703_slot;
        let mut var_guard1703_rv: f64 = *var_guard1703_rv_slot;
        let mut var_guard1704: f64 = *var_guard1704_slot;
        let mut var_guard1704_rv: f64 = *var_guard1704_rv_slot;
        let mut var_guard1705: f64 = *var_guard1705_slot;
        let mut var_guard1705_rv: f64 = *var_guard1705_rv_slot;
        let mut var_guard1706: f64 = *var_guard1706_slot;
        let mut var_guard1706_rv: f64 = *var_guard1706_rv_slot;
        let mut var_guard1707: f64 = *var_guard1707_slot;
        let mut var_guard1707_rv: f64 = *var_guard1707_rv_slot;
        let mut var_ktat__blk1578: f64 = *var_ktat__blk1578_slot;
        let mut var_ktat__blk1578_dn11: f64 = *var_ktat__blk1578_dn11_slot;
        let mut var_ktat__blk1578_dn12: f64 = *var_ktat__blk1578_dn12_slot;
        let mut var_ktat__blk1578_dn6: f64 = *var_ktat__blk1578_dn6_slot;
        let mut var_ktat__blk1578_dn7: f64 = *var_ktat__blk1578_dn7_slot;
        let mut var_ktat__blk1578_dn8: f64 = *var_ktat__blk1578_dn8_slot;
        let mut var_ktat__blk1578_dn9: f64 = *var_ktat__blk1578_dn9_slot;
        let mut var_ktat__blk1578_rv: f64 = *var_ktat__blk1578_rv_slot;
        let mut var_ltat__blk1579: f64 = *var_ltat__blk1579_slot;
        let mut var_ltat__blk1579_dn11: f64 = *var_ltat__blk1579_dn11_slot;
        let mut var_ltat__blk1579_dn12: f64 = *var_ltat__blk1579_dn12_slot;
        let mut var_ltat__blk1579_dn6: f64 = *var_ltat__blk1579_dn6_slot;
        let mut var_ltat__blk1579_dn7: f64 = *var_ltat__blk1579_dn7_slot;
        let mut var_ltat__blk1579_dn8: f64 = *var_ltat__blk1579_dn8_slot;
        let mut var_ltat__blk1579_dn9: f64 = *var_ltat__blk1579_dn9_slot;
        let mut var_ltat__blk1579_rv: f64 = *var_ltat__blk1579_rv_slot;
        let mut var_mtat__blk1580: f64 = *var_mtat__blk1580_slot;
        let mut var_mtat__blk1580_dn11: f64 = *var_mtat__blk1580_dn11_slot;
        let mut var_mtat__blk1580_dn12: f64 = *var_mtat__blk1580_dn12_slot;
        let mut var_mtat__blk1580_dn6: f64 = *var_mtat__blk1580_dn6_slot;
        let mut var_mtat__blk1580_dn7: f64 = *var_mtat__blk1580_dn7_slot;
        let mut var_mtat__blk1580_dn8: f64 = *var_mtat__blk1580_dn8_slot;
        let mut var_mtat__blk1580_dn9: f64 = *var_mtat__blk1580_dn9_slot;
        let mut var_mtat__blk1580_rv: f64 = *var_mtat__blk1580_rv_slot;
        let mut var_qjunbot_d: f64 = *var_qjunbot_d_slot;
        let mut var_qjunbot_d_dn11: f64 = *var_qjunbot_d_dn11_slot;
        let mut var_qjunbot_d_dn12: f64 = *var_qjunbot_d_dn12_slot;
        let mut var_qjunbot_d_dn6: f64 = *var_qjunbot_d_dn6_slot;
        let mut var_qjunbot_d_dn7: f64 = *var_qjunbot_d_dn7_slot;
        let mut var_qjunbot_d_dn8: f64 = *var_qjunbot_d_dn8_slot;
        let mut var_qjunbot_d_dn9: f64 = *var_qjunbot_d_dn9_slot;
        let mut var_qjunbot_d_rv: f64 = *var_qjunbot_d_rv_slot;
        let mut var_qjunsti_d: f64 = *var_qjunsti_d_slot;
        let mut var_qjunsti_d_dn11: f64 = *var_qjunsti_d_dn11_slot;
        let mut var_qjunsti_d_dn12: f64 = *var_qjunsti_d_dn12_slot;
        let mut var_qjunsti_d_dn6: f64 = *var_qjunsti_d_dn6_slot;
        let mut var_qjunsti_d_dn7: f64 = *var_qjunsti_d_dn7_slot;
        let mut var_qjunsti_d_dn8: f64 = *var_qjunsti_d_dn8_slot;
        let mut var_qjunsti_d_dn9: f64 = *var_qjunsti_d_dn9_slot;
        let mut var_qjunsti_d_rv: f64 = *var_qjunsti_d_rv_slot;
        let mut var_sqrtumax__blk1574: f64 = *var_sqrtumax__blk1574_slot;
        let mut var_sqrtumax__blk1574_dn11: f64 = *var_sqrtumax__blk1574_dn11_slot;
        let mut var_sqrtumax__blk1574_dn12: f64 = *var_sqrtumax__blk1574_dn12_slot;
        let mut var_sqrtumax__blk1574_dn6: f64 = *var_sqrtumax__blk1574_dn6_slot;
        let mut var_sqrtumax__blk1574_dn7: f64 = *var_sqrtumax__blk1574_dn7_slot;
        let mut var_sqrtumax__blk1574_dn8: f64 = *var_sqrtumax__blk1574_dn8_slot;
        let mut var_sqrtumax__blk1574_dn9: f64 = *var_sqrtumax__blk1574_dn9_slot;
        let mut var_sqrtumax__blk1574_rv: f64 = *var_sqrtumax__blk1574_rv_slot;
        let mut var_tmp__blk1560: f64 = *var_tmp__blk1560_slot;
        let mut var_tmp__blk1560_dn11: f64 = *var_tmp__blk1560_dn11_slot;
        let mut var_tmp__blk1560_dn12: f64 = *var_tmp__blk1560_dn12_slot;
        let mut var_tmp__blk1560_dn6: f64 = *var_tmp__blk1560_dn6_slot;
        let mut var_tmp__blk1560_dn7: f64 = *var_tmp__blk1560_dn7_slot;
        let mut var_tmp__blk1560_dn8: f64 = *var_tmp__blk1560_dn8_slot;
        let mut var_tmp__blk1560_dn9: f64 = *var_tmp__blk1560_dn9_slot;
        let mut var_tmp__blk1560_rv: f64 = *var_tmp__blk1560_rv_slot;
        let mut var_twoatatoverthreebtat__blk1571: f64 = *var_twoatatoverthreebtat__blk1571_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn11: f64 = *var_twoatatoverthreebtat__blk1571_dn11_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn12: f64 = *var_twoatatoverthreebtat__blk1571_dn12_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn6: f64 = *var_twoatatoverthreebtat__blk1571_dn6_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn7: f64 = *var_twoatatoverthreebtat__blk1571_dn7_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn8: f64 = *var_twoatatoverthreebtat__blk1571_dn8_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn9: f64 = *var_twoatatoverthreebtat__blk1571_dn9_slot;
        let mut var_twoatatoverthreebtat__blk1571_rv: f64 = *var_twoatatoverthreebtat__blk1571_rv_slot;
        let mut var_umax__blk1573: f64 = *var_umax__blk1573_slot;
        let mut var_umax__blk1573_dn11: f64 = *var_umax__blk1573_dn11_slot;
        let mut var_umax__blk1573_dn12: f64 = *var_umax__blk1573_dn12_slot;
        let mut var_umax__blk1573_dn6: f64 = *var_umax__blk1573_dn6_slot;
        let mut var_umax__blk1573_dn7: f64 = *var_umax__blk1573_dn7_slot;
        let mut var_umax__blk1573_dn8: f64 = *var_umax__blk1573_dn8_slot;
        let mut var_umax__blk1573_dn9: f64 = *var_umax__blk1573_dn9_slot;
        let mut var_umax__blk1573_rv: f64 = *var_umax__blk1573_rv_slot;
        let mut var_umaxbeforelimiting__blk1572: f64 = *var_umaxbeforelimiting__blk1572_slot;
        let mut var_umaxbeforelimiting__blk1572_dn11: f64 = *var_umaxbeforelimiting__blk1572_dn11_slot;
        let mut var_umaxbeforelimiting__blk1572_dn12: f64 = *var_umaxbeforelimiting__blk1572_dn12_slot;
        let mut var_umaxbeforelimiting__blk1572_dn6: f64 = *var_umaxbeforelimiting__blk1572_dn6_slot;
        let mut var_umaxbeforelimiting__blk1572_dn7: f64 = *var_umaxbeforelimiting__blk1572_dn7_slot;
        let mut var_umaxbeforelimiting__blk1572_dn8: f64 = *var_umaxbeforelimiting__blk1572_dn8_slot;
        let mut var_umaxbeforelimiting__blk1572_dn9: f64 = *var_umaxbeforelimiting__blk1572_dn9_slot;
        let mut var_umaxbeforelimiting__blk1572_rv: f64 = *var_umaxbeforelimiting__blk1572_rv_slot;
        let mut var_umaxpoweronepointfive__blk1575: f64 = *var_umaxpoweronepointfive__blk1575_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn11: f64 = *var_umaxpoweronepointfive__blk1575_dn11_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn12: f64 = *var_umaxpoweronepointfive__blk1575_dn12_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn6: f64 = *var_umaxpoweronepointfive__blk1575_dn6_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn7: f64 = *var_umaxpoweronepointfive__blk1575_dn7_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn8: f64 = *var_umaxpoweronepointfive__blk1575_dn8_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn9: f64 = *var_umaxpoweronepointfive__blk1575_dn9_slot;
        let mut var_umaxpoweronepointfive__blk1575_rv: f64 = *var_umaxpoweronepointfive__blk1575_rv_slot;
        let mut var_vbi_minus_vjsrh__blk1563: f64 = *var_vbi_minus_vjsrh__blk1563_slot;
        let mut var_vbi_minus_vjsrh__blk1563_dn11: f64 = *var_vbi_minus_vjsrh__blk1563_dn11_slot;
        let mut var_vbi_minus_vjsrh__blk1563_dn12: f64 = *var_vbi_minus_vjsrh__blk1563_dn12_slot;
        let mut var_vbi_minus_vjsrh__blk1563_dn7: f64 = *var_vbi_minus_vjsrh__blk1563_dn7_slot;
        let mut var_vbi_minus_vjsrh__blk1563_dn8: f64 = *var_vbi_minus_vjsrh__blk1563_dn8_slot;
        let mut var_vbi_minus_vjsrh__blk1563_rv: f64 = *var_vbi_minus_vjsrh__blk1563_rv_slot;
        let mut var_wdep__blk1567: f64 = *var_wdep__blk1567_slot;
        let mut var_wdep__blk1567_dn11: f64 = *var_wdep__blk1567_dn11_slot;
        let mut var_wdep__blk1567_dn12: f64 = *var_wdep__blk1567_dn12_slot;
        let mut var_wdep__blk1567_dn6: f64 = *var_wdep__blk1567_dn6_slot;
        let mut var_wdep__blk1567_dn7: f64 = *var_wdep__blk1567_dn7_slot;
        let mut var_wdep__blk1567_dn8: f64 = *var_wdep__blk1567_dn8_slot;
        let mut var_wdep__blk1567_dn9: f64 = *var_wdep__blk1567_dn9_slot;
        let mut var_wdep__blk1567_rv: f64 = *var_wdep__blk1567_rv_slot;
        let mut var_xerfc__blk1581: f64 = *var_xerfc__blk1581_slot;
        let mut var_xerfc__blk1581_dn11: f64 = *var_xerfc__blk1581_dn11_slot;
        let mut var_xerfc__blk1581_dn12: f64 = *var_xerfc__blk1581_dn12_slot;
        let mut var_xerfc__blk1581_dn6: f64 = *var_xerfc__blk1581_dn6_slot;
        let mut var_xerfc__blk1581_dn7: f64 = *var_xerfc__blk1581_dn7_slot;
        let mut var_xerfc__blk1581_dn8: f64 = *var_xerfc__blk1581_dn8_slot;
        let mut var_xerfc__blk1581_dn9: f64 = *var_xerfc__blk1581_dn9_slot;
        let mut var_xerfc__blk1581_rv: f64 = *var_xerfc__blk1581_rv_slot;
        let mut var_ysq__blk1542: f64 = *var_ysq__blk1542_slot;
        let mut var_ysq__blk1542_dn11: f64 = *var_ysq__blk1542_dn11_slot;
        let mut var_ysq__blk1542_dn12: f64 = *var_ysq__blk1542_dn12_slot;
        let mut var_ysq__blk1542_dn6: f64 = *var_ysq__blk1542_dn6_slot;
        let mut var_ysq__blk1542_dn7: f64 = *var_ysq__blk1542_dn7_slot;
        let mut var_ysq__blk1542_dn8: f64 = *var_ysq__blk1542_dn8_slot;
        let mut var_ysq__blk1542_dn9: f64 = *var_ysq__blk1542_dn9_slot;
        let mut var_ysq__blk1542_rv: f64 = *var_ysq__blk1542_rv_slot;

        let (assign60200_e77663, assign60200_e77663_d_n6, assign60200_e77663_d_n7, assign60200_e77663_d_n8, assign60200_e77663_d_n9, assign60200_e77663_d_n11, assign60200_e77663_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1695 != 0.0)) {
        let assign60200_e77659: f64 = (var_vj__blk1552 * var_vbiinvbot_d);
        let assign60200_e77660: f64 = (1.0 - assign60200_e77659);
        let assign60200_e77661: f64 = (assign60200_e77660).sqrt();
        (assign60200_e77661, 0.0, ((-(var_vj__blk1552_dn7 * var_vbiinvbot_d)) / (2.0 * assign60200_e77661)), ((-(var_vj__blk1552_dn8 * var_vbiinvbot_d)) / (2.0 * assign60200_e77661)), 0.0, ((-(var_vj__blk1552_dn11 * var_vbiinvbot_d)) / (2.0 * assign60200_e77661)), ((-(var_vj__blk1552_dn12 * var_vbiinvbot_d)) / (2.0 * assign60200_e77661)),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60200_e77663;
        var_tmp__blk1560_dn6 = assign60200_e77663_d_n6;
        var_tmp__blk1560_dn7 = assign60200_e77663_d_n7;
        var_tmp__blk1560_dn8 = assign60200_e77663_d_n8;
        var_tmp__blk1560_dn9 = assign60200_e77663_d_n9;
        var_tmp__blk1560_dn11 = assign60200_e77663_d_n11;
        var_tmp__blk1560_dn12 = assign60200_e77663_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let (assign60210_e77682, assign60210_e77682_d_n6, assign60210_e77682_d_n7, assign60210_e77682_d_n8, assign60210_e77682_d_n9, assign60210_e77682_d_n11, assign60210_e77682_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) && (var_guard1695 == 0.0)) {
        let assign60210_e77677: f64 = (var_vj__blk1552 * var_vbiinvbot_d);
        let assign60210_e77678: f64 = (1.0 - assign60210_e77677);
        let assign60210_e77680: f64 = (assign60210_e77678).powf(var_one_minus_pbot_d);
        (assign60210_e77680, 0.0, if 0.0 == 0.0 && ((var_one_minus_pbot_d) as f64).is_finite() && ((var_one_minus_pbot_d) as f64).fract() == 0.0 { if var_one_minus_pbot_d == 0.0 { 0.0 } else { (var_one_minus_pbot_d * ((assign60210_e77678).powf(var_one_minus_pbot_d - 1.0) * (-(var_vj__blk1552_dn7 * var_vbiinvbot_d)))) } } else { (assign60210_e77680 * (var_one_minus_pbot_d * ((-(var_vj__blk1552_dn7 * var_vbiinvbot_d)) / assign60210_e77678))) }, if 0.0 == 0.0 && ((var_one_minus_pbot_d) as f64).is_finite() && ((var_one_minus_pbot_d) as f64).fract() == 0.0 { if var_one_minus_pbot_d == 0.0 { 0.0 } else { (var_one_minus_pbot_d * ((assign60210_e77678).powf(var_one_minus_pbot_d - 1.0) * (-(var_vj__blk1552_dn8 * var_vbiinvbot_d)))) } } else { (assign60210_e77680 * (var_one_minus_pbot_d * ((-(var_vj__blk1552_dn8 * var_vbiinvbot_d)) / assign60210_e77678))) }, 0.0, if 0.0 == 0.0 && ((var_one_minus_pbot_d) as f64).is_finite() && ((var_one_minus_pbot_d) as f64).fract() == 0.0 { if var_one_minus_pbot_d == 0.0 { 0.0 } else { (var_one_minus_pbot_d * ((assign60210_e77678).powf(var_one_minus_pbot_d - 1.0) * (-(var_vj__blk1552_dn11 * var_vbiinvbot_d)))) } } else { (assign60210_e77680 * (var_one_minus_pbot_d * ((-(var_vj__blk1552_dn11 * var_vbiinvbot_d)) / assign60210_e77678))) }, if 0.0 == 0.0 && ((var_one_minus_pbot_d) as f64).is_finite() && ((var_one_minus_pbot_d) as f64).fract() == 0.0 { if var_one_minus_pbot_d == 0.0 { 0.0 } else { (var_one_minus_pbot_d * ((assign60210_e77678).powf(var_one_minus_pbot_d - 1.0) * (-(var_vj__blk1552_dn12 * var_vbiinvbot_d)))) } } else { (assign60210_e77680 * (var_one_minus_pbot_d * ((-(var_vj__blk1552_dn12 * var_vbiinvbot_d)) / assign60210_e77678))) },)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60210_e77682;
        var_tmp__blk1560_dn6 = assign60210_e77682_d_n6;
        var_tmp__blk1560_dn7 = assign60210_e77682_d_n7;
        var_tmp__blk1560_dn8 = assign60210_e77682_d_n8;
        var_tmp__blk1560_dn9 = assign60210_e77682_d_n9;
        var_tmp__blk1560_dn11 = assign60210_e77682_d_n11;
        var_tmp__blk1560_dn12 = assign60210_e77682_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let (assign60220_e77704, assign60220_e77704_d_n6, assign60220_e77704_d_n7, assign60220_e77704_d_n8, assign60220_e77704_d_n9, assign60220_e77704_d_n11, assign60220_e77704_d_n12,) = {
    if (((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1678 == 0.0)) {
        let assign60220_e77694: f64 = (1.0 - var_tmp__blk1560);
        let assign60220_e77695: f64 = (var_qprefbot_d * assign60220_e77694);
        let assign60220_e77699: f64 = (var_vjun_d - var_vj__blk1552);
        let assign60220_e77700: f64 = (var_qpref2bot_d * assign60220_e77699);
        let assign60220_e77701: f64 = (assign60220_e77695 + assign60220_e77700);
        let assign60220_e77702: f64 = (p.p30 * assign60220_e77701);
        (assign60220_e77702, (p.p30 * (var_qprefbot_d * (-var_tmp__blk1560_dn6))), (p.p30 * ((var_qprefbot_d * (-var_tmp__blk1560_dn7)) + (var_qpref2bot_d * (-var_vj__blk1552_dn7)))), (p.p30 * ((var_qprefbot_d * (-var_tmp__blk1560_dn8)) + (var_qpref2bot_d * (var_vjun_d_dn8 - var_vj__blk1552_dn8)))), (p.p30 * (var_qprefbot_d * (-var_tmp__blk1560_dn9))), (p.p30 * ((var_qprefbot_d * (-var_tmp__blk1560_dn11)) + (var_qpref2bot_d * (-var_vj__blk1552_dn11)))), (p.p30 * ((var_qprefbot_d * (-var_tmp__blk1560_dn12)) + (var_qpref2bot_d * (var_vjun_d_dn12 - var_vj__blk1552_dn12)))),)
    } else {
        (var_qjunbot_d, var_qjunbot_d_dn6, var_qjunbot_d_dn7, var_qjunbot_d_dn8, var_qjunbot_d_dn9, var_qjunbot_d_dn11, var_qjunbot_d_dn12,)
    }
};
        var_qjunbot_d = assign60220_e77704;
        var_qjunbot_d_dn6 = assign60220_e77704_d_n6;
        var_qjunbot_d_dn7 = assign60220_e77704_d_n7;
        var_qjunbot_d_dn8 = assign60220_e77704_d_n8;
        var_qjunbot_d_dn9 = assign60220_e77704_d_n9;
        var_qjunbot_d_dn11 = assign60220_e77704_d_n11;
        var_qjunbot_d_dn12 = assign60220_e77704_d_n12;
        var_qjunbot_d_rv = 0.0;

        let assign60230_e77707: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard1696 = assign60230_e77707;
        var_guard1696_rv = 0.0;

        let (assign60250_e77725, assign60250_e77725_d_n6, assign60250_e77725_d_n7, assign60250_e77725_d_n8, assign60250_e77725_d_n9, assign60250_e77725_d_n11, assign60250_e77725_d_n12,) = {
    if (((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qjunsti_d, var_qjunsti_d_dn6, var_qjunsti_d_dn7, var_qjunsti_d_dn8, var_qjunsti_d_dn9, var_qjunsti_d_dn11, var_qjunsti_d_dn12,)
    }
};
        var_qjunsti_d = assign60250_e77725;
        var_qjunsti_d_dn6 = assign60250_e77725_d_n6;
        var_qjunsti_d_dn7 = assign60250_e77725_d_n7;
        var_qjunsti_d_dn8 = assign60250_e77725_d_n8;
        var_qjunsti_d_dn9 = assign60250_e77725_d_n9;
        var_qjunsti_d_dn11 = assign60250_e77725_d_n11;
        var_qjunsti_d_dn12 = assign60250_e77725_d_n12;
        var_qjunsti_d_rv = 0.0;

        let assign60270_e77744: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard1697 = assign60270_e77744;
        var_guard1697_rv = 0.0;

        let (assign60290_e77771, assign60290_e77771_d_n7, assign60290_e77771_d_n8, assign60290_e77771_d_n11, assign60290_e77771_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1697 == 0.0)) {
        let assign60290_e77769: f64 = (var_vbisti_d - var_vjsrh__blk1557);
        (assign60290_e77769, (-var_vjsrh__blk1557_dn7), (-var_vjsrh__blk1557_dn8), (-var_vjsrh__blk1557_dn11), (-var_vjsrh__blk1557_dn12),)
    } else {
        (var_vbi_minus_vjsrh__blk1563, var_vbi_minus_vjsrh__blk1563_dn7, var_vbi_minus_vjsrh__blk1563_dn8, var_vbi_minus_vjsrh__blk1563_dn11, var_vbi_minus_vjsrh__blk1563_dn12,)
    }
};
        var_vbi_minus_vjsrh__blk1563 = assign60290_e77771;
        var_vbi_minus_vjsrh__blk1563_dn7 = assign60290_e77771_d_n7;
        var_vbi_minus_vjsrh__blk1563_dn8 = assign60290_e77771_d_n8;
        var_vbi_minus_vjsrh__blk1563_dn11 = assign60290_e77771_d_n11;
        var_vbi_minus_vjsrh__blk1563_dn12 = assign60290_e77771_d_n12;
        var_vbi_minus_vjsrh__blk1563_rv = 0.0;

        let assign60350_e77860: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard1699 = assign60350_e77860;
        var_guard1699_rv = 0.0;

        let (assign60360_e77878, assign60360_e77878_d_n6, assign60360_e77878_d_n7, assign60360_e77878_d_n8, assign60360_e77878_d_n9, assign60360_e77878_d_n11, assign60360_e77878_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1699 != 0.0)) {
        let assign60360_e77875: f64 = (var_vbi_minus_vjsrh__blk1563 * var_vbirstiinv_d);
        let assign60360_e77876: f64 = (assign60360_e77875).sqrt();
        (assign60360_e77876, 0.0, ((var_vbi_minus_vjsrh__blk1563_dn7 * var_vbirstiinv_d) / (2.0 * assign60360_e77876)), ((var_vbi_minus_vjsrh__blk1563_dn8 * var_vbirstiinv_d) / (2.0 * assign60360_e77876)), 0.0, ((var_vbi_minus_vjsrh__blk1563_dn11 * var_vbirstiinv_d) / (2.0 * assign60360_e77876)), ((var_vbi_minus_vjsrh__blk1563_dn12 * var_vbirstiinv_d) / (2.0 * assign60360_e77876)),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60360_e77878;
        var_tmp__blk1560_dn6 = assign60360_e77878_d_n6;
        var_tmp__blk1560_dn7 = assign60360_e77878_d_n7;
        var_tmp__blk1560_dn8 = assign60360_e77878_d_n8;
        var_tmp__blk1560_dn9 = assign60360_e77878_d_n9;
        var_tmp__blk1560_dn11 = assign60360_e77878_d_n11;
        var_tmp__blk1560_dn12 = assign60360_e77878_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let (assign60370_e77898, assign60370_e77898_d_n6, assign60370_e77898_d_n7, assign60370_e77898_d_n8, assign60370_e77898_d_n9, assign60370_e77898_d_n11, assign60370_e77898_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1699 == 0.0)) {
        let assign60370_e77894: f64 = (var_vbi_minus_vjsrh__blk1563 * var_vbirstiinv_d);
        let assign60370_e77896: f64 = (assign60370_e77894).powf(var_pstid_i);
        (assign60370_e77896, 0.0, if 0.0 == 0.0 && ((var_pstid_i) as f64).is_finite() && ((var_pstid_i) as f64).fract() == 0.0 { if var_pstid_i == 0.0 { 0.0 } else { (var_pstid_i * ((assign60370_e77894).powf(var_pstid_i - 1.0) * (var_vbi_minus_vjsrh__blk1563_dn7 * var_vbirstiinv_d))) } } else { (assign60370_e77896 * (var_pstid_i * ((var_vbi_minus_vjsrh__blk1563_dn7 * var_vbirstiinv_d) / assign60370_e77894))) }, if 0.0 == 0.0 && ((var_pstid_i) as f64).is_finite() && ((var_pstid_i) as f64).fract() == 0.0 { if var_pstid_i == 0.0 { 0.0 } else { (var_pstid_i * ((assign60370_e77894).powf(var_pstid_i - 1.0) * (var_vbi_minus_vjsrh__blk1563_dn8 * var_vbirstiinv_d))) } } else { (assign60370_e77896 * (var_pstid_i * ((var_vbi_minus_vjsrh__blk1563_dn8 * var_vbirstiinv_d) / assign60370_e77894))) }, 0.0, if 0.0 == 0.0 && ((var_pstid_i) as f64).is_finite() && ((var_pstid_i) as f64).fract() == 0.0 { if var_pstid_i == 0.0 { 0.0 } else { (var_pstid_i * ((assign60370_e77894).powf(var_pstid_i - 1.0) * (var_vbi_minus_vjsrh__blk1563_dn11 * var_vbirstiinv_d))) } } else { (assign60370_e77896 * (var_pstid_i * ((var_vbi_minus_vjsrh__blk1563_dn11 * var_vbirstiinv_d) / assign60370_e77894))) }, if 0.0 == 0.0 && ((var_pstid_i) as f64).is_finite() && ((var_pstid_i) as f64).fract() == 0.0 { if var_pstid_i == 0.0 { 0.0 } else { (var_pstid_i * ((assign60370_e77894).powf(var_pstid_i - 1.0) * (var_vbi_minus_vjsrh__blk1563_dn12 * var_vbirstiinv_d))) } } else { (assign60370_e77896 * (var_pstid_i * ((var_vbi_minus_vjsrh__blk1563_dn12 * var_vbirstiinv_d) / assign60370_e77894))) },)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60370_e77898;
        var_tmp__blk1560_dn6 = assign60370_e77898_d_n6;
        var_tmp__blk1560_dn7 = assign60370_e77898_d_n7;
        var_tmp__blk1560_dn8 = assign60370_e77898_d_n8;
        var_tmp__blk1560_dn9 = assign60370_e77898_d_n9;
        var_tmp__blk1560_dn11 = assign60370_e77898_d_n11;
        var_tmp__blk1560_dn12 = assign60370_e77898_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let (assign60380_e77913, assign60380_e77913_d_n6, assign60380_e77913_d_n7, assign60380_e77913_d_n8, assign60380_e77913_d_n9, assign60380_e77913_d_n11, assign60380_e77913_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1697 == 0.0)) {
        let assign60380_e77911: f64 = (var_wdepnulrsti_d * var_tmp__blk1560);
        (assign60380_e77911, (var_wdepnulrsti_d * var_tmp__blk1560_dn6), (var_wdepnulrsti_d * var_tmp__blk1560_dn7), (var_wdepnulrsti_d * var_tmp__blk1560_dn8), (var_wdepnulrsti_d * var_tmp__blk1560_dn9), (var_wdepnulrsti_d * var_tmp__blk1560_dn11), (var_wdepnulrsti_d * var_tmp__blk1560_dn12),)
    } else {
        (var_wdep__blk1567, var_wdep__blk1567_dn6, var_wdep__blk1567_dn7, var_wdep__blk1567_dn8, var_wdep__blk1567_dn9, var_wdep__blk1567_dn11, var_wdep__blk1567_dn12,)
    }
};
        var_wdep__blk1567 = assign60380_e77913;
        var_wdep__blk1567_dn6 = assign60380_e77913_d_n6;
        var_wdep__blk1567_dn7 = assign60380_e77913_d_n7;
        var_wdep__blk1567_dn8 = assign60380_e77913_d_n8;
        var_wdep__blk1567_dn9 = assign60380_e77913_d_n9;
        var_wdep__blk1567_dn11 = assign60380_e77913_d_n11;
        var_wdep__blk1567_dn12 = assign60380_e77913_d_n12;
        var_wdep__blk1567_rv = 0.0;

        let assign60410_e77952: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard1700 = assign60410_e77952;
        var_guard1700_rv = 0.0;

        let (assign60430_e77983, assign60430_e77983_d_n6, assign60430_e77983_d_n7, assign60430_e77983_d_n8, assign60430_e77983_d_n9, assign60430_e77983_d_n11, assign60430_e77983_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) {
        let assign60430_e77978: f64 = (var_wdep__blk1567 * var_one_minus_psti_d);
        let assign60430_e77980: f64 = (assign60430_e77978 / var_vbi_minus_vjsrh__blk1563);
        let assign60430_e77981: f64 = (var_btatpartsti_d * assign60430_e77980);
        (assign60430_e77981, (var_btatpartsti_d * ((var_wdep__blk1567_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh__blk1563)), (var_btatpartsti_d * ((((var_wdep__blk1567_dn7 * var_one_minus_psti_d) * var_vbi_minus_vjsrh__blk1563) - (assign60430_e77978 * var_vbi_minus_vjsrh__blk1563_dn7)) / (var_vbi_minus_vjsrh__blk1563 * var_vbi_minus_vjsrh__blk1563))), (var_btatpartsti_d * ((((var_wdep__blk1567_dn8 * var_one_minus_psti_d) * var_vbi_minus_vjsrh__blk1563) - (assign60430_e77978 * var_vbi_minus_vjsrh__blk1563_dn8)) / (var_vbi_minus_vjsrh__blk1563 * var_vbi_minus_vjsrh__blk1563))), (var_btatpartsti_d * ((var_wdep__blk1567_dn9 * var_one_minus_psti_d) / var_vbi_minus_vjsrh__blk1563)), (var_btatpartsti_d * ((((var_wdep__blk1567_dn11 * var_one_minus_psti_d) * var_vbi_minus_vjsrh__blk1563) - (assign60430_e77978 * var_vbi_minus_vjsrh__blk1563_dn11)) / (var_vbi_minus_vjsrh__blk1563 * var_vbi_minus_vjsrh__blk1563))), (var_btatpartsti_d * ((((var_wdep__blk1567_dn12 * var_one_minus_psti_d) * var_vbi_minus_vjsrh__blk1563) - (assign60430_e77978 * var_vbi_minus_vjsrh__blk1563_dn12)) / (var_vbi_minus_vjsrh__blk1563 * var_vbi_minus_vjsrh__blk1563))),)
    } else {
        (var_btat__blk1570, var_btat__blk1570_dn6, var_btat__blk1570_dn7, var_btat__blk1570_dn8, var_btat__blk1570_dn9, var_btat__blk1570_dn11, var_btat__blk1570_dn12,)
    }
};
        var_btat__blk1570 = assign60430_e77983;
        var_btat__blk1570_dn6 = assign60430_e77983_d_n6;
        var_btat__blk1570_dn7 = assign60430_e77983_d_n7;
        var_btat__blk1570_dn8 = assign60430_e77983_d_n8;
        var_btat__blk1570_dn9 = assign60430_e77983_d_n9;
        var_btat__blk1570_dn11 = assign60430_e77983_d_n11;
        var_btat__blk1570_dn12 = assign60430_e77983_d_n12;
        var_btat__blk1570_rv = 0.0;

        let (assign60440_e78000, assign60440_e78000_d_n6, assign60440_e78000_d_n7, assign60440_e78000_d_n8, assign60440_e78000_d_n9, assign60440_e78000_d_n11, assign60440_e78000_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) {
        let assign60440_e77996: f64 = (0.666666666666667 * var_atatsti_d);
        let assign60440_e77998: f64 = (assign60440_e77996 / var_btat__blk1570);
        (assign60440_e77998, (-((assign60440_e77996 * var_btat__blk1570_dn6) / (var_btat__blk1570 * var_btat__blk1570))), (-((assign60440_e77996 * var_btat__blk1570_dn7) / (var_btat__blk1570 * var_btat__blk1570))), (-((assign60440_e77996 * var_btat__blk1570_dn8) / (var_btat__blk1570 * var_btat__blk1570))), (-((assign60440_e77996 * var_btat__blk1570_dn9) / (var_btat__blk1570 * var_btat__blk1570))), (-((assign60440_e77996 * var_btat__blk1570_dn11) / (var_btat__blk1570 * var_btat__blk1570))), (-((assign60440_e77996 * var_btat__blk1570_dn12) / (var_btat__blk1570 * var_btat__blk1570))),)
    } else {
        (var_twoatatoverthreebtat__blk1571, var_twoatatoverthreebtat__blk1571_dn6, var_twoatatoverthreebtat__blk1571_dn7, var_twoatatoverthreebtat__blk1571_dn8, var_twoatatoverthreebtat__blk1571_dn9, var_twoatatoverthreebtat__blk1571_dn11, var_twoatatoverthreebtat__blk1571_dn12,)
    }
};
        var_twoatatoverthreebtat__blk1571 = assign60440_e78000;
        var_twoatatoverthreebtat__blk1571_dn6 = assign60440_e78000_d_n6;
        var_twoatatoverthreebtat__blk1571_dn7 = assign60440_e78000_d_n7;
        var_twoatatoverthreebtat__blk1571_dn8 = assign60440_e78000_d_n8;
        var_twoatatoverthreebtat__blk1571_dn9 = assign60440_e78000_d_n9;
        var_twoatatoverthreebtat__blk1571_dn11 = assign60440_e78000_d_n11;
        var_twoatatoverthreebtat__blk1571_dn12 = assign60440_e78000_d_n12;
        var_twoatatoverthreebtat__blk1571_rv = 0.0;

        let (assign60450_e78015, assign60450_e78015_d_n6, assign60450_e78015_d_n7, assign60450_e78015_d_n8, assign60450_e78015_d_n9, assign60450_e78015_d_n11, assign60450_e78015_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) {
        let assign60450_e78013: f64 = (var_twoatatoverthreebtat__blk1571 * var_twoatatoverthreebtat__blk1571);
        (assign60450_e78013, ((var_twoatatoverthreebtat__blk1571_dn6 * var_twoatatoverthreebtat__blk1571) + (var_twoatatoverthreebtat__blk1571 * var_twoatatoverthreebtat__blk1571_dn6)), ((var_twoatatoverthreebtat__blk1571_dn7 * var_twoatatoverthreebtat__blk1571) + (var_twoatatoverthreebtat__blk1571 * var_twoatatoverthreebtat__blk1571_dn7)), ((var_twoatatoverthreebtat__blk1571_dn8 * var_twoatatoverthreebtat__blk1571) + (var_twoatatoverthreebtat__blk1571 * var_twoatatoverthreebtat__blk1571_dn8)), ((var_twoatatoverthreebtat__blk1571_dn9 * var_twoatatoverthreebtat__blk1571) + (var_twoatatoverthreebtat__blk1571 * var_twoatatoverthreebtat__blk1571_dn9)), ((var_twoatatoverthreebtat__blk1571_dn11 * var_twoatatoverthreebtat__blk1571) + (var_twoatatoverthreebtat__blk1571 * var_twoatatoverthreebtat__blk1571_dn11)), ((var_twoatatoverthreebtat__blk1571_dn12 * var_twoatatoverthreebtat__blk1571) + (var_twoatatoverthreebtat__blk1571 * var_twoatatoverthreebtat__blk1571_dn12)),)
    } else {
        (var_umaxbeforelimiting__blk1572, var_umaxbeforelimiting__blk1572_dn6, var_umaxbeforelimiting__blk1572_dn7, var_umaxbeforelimiting__blk1572_dn8, var_umaxbeforelimiting__blk1572_dn9, var_umaxbeforelimiting__blk1572_dn11, var_umaxbeforelimiting__blk1572_dn12,)
    }
};
        var_umaxbeforelimiting__blk1572 = assign60450_e78015;
        var_umaxbeforelimiting__blk1572_dn6 = assign60450_e78015_d_n6;
        var_umaxbeforelimiting__blk1572_dn7 = assign60450_e78015_d_n7;
        var_umaxbeforelimiting__blk1572_dn8 = assign60450_e78015_d_n8;
        var_umaxbeforelimiting__blk1572_dn9 = assign60450_e78015_d_n9;
        var_umaxbeforelimiting__blk1572_dn11 = assign60450_e78015_d_n11;
        var_umaxbeforelimiting__blk1572_dn12 = assign60450_e78015_d_n12;
        var_umaxbeforelimiting__blk1572_rv = 0.0;

        let (assign60460_e78037, assign60460_e78037_d_n6, assign60460_e78037_d_n7, assign60460_e78037_d_n8, assign60460_e78037_d_n9, assign60460_e78037_d_n11, assign60460_e78037_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) {
        let assign60460_e78028: f64 = (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572);
        let assign60460_e78031: f64 = (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572);
        let assign60460_e78033: f64 = (assign60460_e78031 + 1.0);
        let assign60460_e78034: f64 = (assign60460_e78028 / assign60460_e78033);
        let assign60460_e78035: f64 = (assign60460_e78034).sqrt();
        (assign60460_e78035, ((((((var_umaxbeforelimiting__blk1572_dn6 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn6)) * assign60460_e78033) - (assign60460_e78028 * ((var_umaxbeforelimiting__blk1572_dn6 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn6)))) / (assign60460_e78033 * assign60460_e78033)) / (2.0 * assign60460_e78035)), ((((((var_umaxbeforelimiting__blk1572_dn7 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn7)) * assign60460_e78033) - (assign60460_e78028 * ((var_umaxbeforelimiting__blk1572_dn7 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn7)))) / (assign60460_e78033 * assign60460_e78033)) / (2.0 * assign60460_e78035)), ((((((var_umaxbeforelimiting__blk1572_dn8 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn8)) * assign60460_e78033) - (assign60460_e78028 * ((var_umaxbeforelimiting__blk1572_dn8 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn8)))) / (assign60460_e78033 * assign60460_e78033)) / (2.0 * assign60460_e78035)), ((((((var_umaxbeforelimiting__blk1572_dn9 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn9)) * assign60460_e78033) - (assign60460_e78028 * ((var_umaxbeforelimiting__blk1572_dn9 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn9)))) / (assign60460_e78033 * assign60460_e78033)) / (2.0 * assign60460_e78035)), ((((((var_umaxbeforelimiting__blk1572_dn11 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn11)) * assign60460_e78033) - (assign60460_e78028 * ((var_umaxbeforelimiting__blk1572_dn11 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn11)))) / (assign60460_e78033 * assign60460_e78033)) / (2.0 * assign60460_e78035)), ((((((var_umaxbeforelimiting__blk1572_dn12 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn12)) * assign60460_e78033) - (assign60460_e78028 * ((var_umaxbeforelimiting__blk1572_dn12 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn12)))) / (assign60460_e78033 * assign60460_e78033)) / (2.0 * assign60460_e78035)),)
    } else {
        (var_umax__blk1573, var_umax__blk1573_dn6, var_umax__blk1573_dn7, var_umax__blk1573_dn8, var_umax__blk1573_dn9, var_umax__blk1573_dn11, var_umax__blk1573_dn12,)
    }
};
        var_umax__blk1573 = assign60460_e78037;
        var_umax__blk1573_dn6 = assign60460_e78037_d_n6;
        var_umax__blk1573_dn7 = assign60460_e78037_d_n7;
        var_umax__blk1573_dn8 = assign60460_e78037_d_n8;
        var_umax__blk1573_dn9 = assign60460_e78037_d_n9;
        var_umax__blk1573_dn11 = assign60460_e78037_d_n11;
        var_umax__blk1573_dn12 = assign60460_e78037_d_n12;
        var_umax__blk1573_rv = 0.0;

        let (assign60470_e78051, assign60470_e78051_d_n6, assign60470_e78051_d_n7, assign60470_e78051_d_n8, assign60470_e78051_d_n9, assign60470_e78051_d_n11, assign60470_e78051_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) {
        let assign60470_e78049: f64 = (var_umax__blk1573).sqrt();
        (assign60470_e78049, (var_umax__blk1573_dn6 / (2.0 * assign60470_e78049)), (var_umax__blk1573_dn7 / (2.0 * assign60470_e78049)), (var_umax__blk1573_dn8 / (2.0 * assign60470_e78049)), (var_umax__blk1573_dn9 / (2.0 * assign60470_e78049)), (var_umax__blk1573_dn11 / (2.0 * assign60470_e78049)), (var_umax__blk1573_dn12 / (2.0 * assign60470_e78049)),)
    } else {
        (var_sqrtumax__blk1574, var_sqrtumax__blk1574_dn6, var_sqrtumax__blk1574_dn7, var_sqrtumax__blk1574_dn8, var_sqrtumax__blk1574_dn9, var_sqrtumax__blk1574_dn11, var_sqrtumax__blk1574_dn12,)
    }
};
        var_sqrtumax__blk1574 = assign60470_e78051;
        var_sqrtumax__blk1574_dn6 = assign60470_e78051_d_n6;
        var_sqrtumax__blk1574_dn7 = assign60470_e78051_d_n7;
        var_sqrtumax__blk1574_dn8 = assign60470_e78051_d_n8;
        var_sqrtumax__blk1574_dn9 = assign60470_e78051_d_n9;
        var_sqrtumax__blk1574_dn11 = assign60470_e78051_d_n11;
        var_sqrtumax__blk1574_dn12 = assign60470_e78051_d_n12;
        var_sqrtumax__blk1574_rv = 0.0;

        let (assign60480_e78066, assign60480_e78066_d_n6, assign60480_e78066_d_n7, assign60480_e78066_d_n8, assign60480_e78066_d_n9, assign60480_e78066_d_n11, assign60480_e78066_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) {
        let assign60480_e78064: f64 = (var_umax__blk1573 * var_sqrtumax__blk1574);
        (assign60480_e78064, ((var_umax__blk1573_dn6 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn6)), ((var_umax__blk1573_dn7 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn7)), ((var_umax__blk1573_dn8 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn8)), ((var_umax__blk1573_dn9 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn9)), ((var_umax__blk1573_dn11 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn11)), ((var_umax__blk1573_dn12 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn12)),)
    } else {
        (var_umaxpoweronepointfive__blk1575, var_umaxpoweronepointfive__blk1575_dn6, var_umaxpoweronepointfive__blk1575_dn7, var_umaxpoweronepointfive__blk1575_dn8, var_umaxpoweronepointfive__blk1575_dn9, var_umaxpoweronepointfive__blk1575_dn11, var_umaxpoweronepointfive__blk1575_dn12,)
    }
};
        var_umaxpoweronepointfive__blk1575 = assign60480_e78066;
        var_umaxpoweronepointfive__blk1575_dn6 = assign60480_e78066_d_n6;
        var_umaxpoweronepointfive__blk1575_dn7 = assign60480_e78066_d_n7;
        var_umaxpoweronepointfive__blk1575_dn8 = assign60480_e78066_d_n8;
        var_umaxpoweronepointfive__blk1575_dn9 = assign60480_e78066_d_n9;
        var_umaxpoweronepointfive__blk1575_dn11 = assign60480_e78066_d_n11;
        var_umaxpoweronepointfive__blk1575_dn12 = assign60480_e78066_d_n12;
        var_umaxpoweronepointfive__blk1575_rv = 0.0;

        let (assign60530_e78156, assign60530_e78156_d_n6, assign60530_e78156_d_n7, assign60530_e78156_d_n8, assign60530_e78156_d_n9, assign60530_e78156_d_n11, assign60530_e78156_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) {
        let assign60530_e78152: f64 = (var_btat__blk1570 / var_sqrtumax__blk1574);
        let assign60530_e78153: f64 = (0.375 * assign60530_e78152);
        let assign60530_e78154: f64 = (assign60530_e78153).sqrt();
        (assign60530_e78154, ((0.375 * (((var_btat__blk1570_dn6 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn6)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign60530_e78154)), ((0.375 * (((var_btat__blk1570_dn7 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn7)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign60530_e78154)), ((0.375 * (((var_btat__blk1570_dn8 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn8)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign60530_e78154)), ((0.375 * (((var_btat__blk1570_dn9 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn9)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign60530_e78154)), ((0.375 * (((var_btat__blk1570_dn11 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn11)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign60530_e78154)), ((0.375 * (((var_btat__blk1570_dn12 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn12)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign60530_e78154)),)
    } else {
        (var_ktat__blk1578, var_ktat__blk1578_dn6, var_ktat__blk1578_dn7, var_ktat__blk1578_dn8, var_ktat__blk1578_dn9, var_ktat__blk1578_dn11, var_ktat__blk1578_dn12,)
    }
};
        var_ktat__blk1578 = assign60530_e78156;
        var_ktat__blk1578_dn6 = assign60530_e78156_d_n6;
        var_ktat__blk1578_dn7 = assign60530_e78156_d_n7;
        var_ktat__blk1578_dn8 = assign60530_e78156_d_n8;
        var_ktat__blk1578_dn9 = assign60530_e78156_d_n9;
        var_ktat__blk1578_dn11 = assign60530_e78156_d_n11;
        var_ktat__blk1578_dn12 = assign60530_e78156_d_n12;
        var_ktat__blk1578_rv = 0.0;

        let (assign60540_e78175, assign60540_e78175_d_n6, assign60540_e78175_d_n7, assign60540_e78175_d_n8, assign60540_e78175_d_n9, assign60540_e78175_d_n11, assign60540_e78175_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) {
        let assign60540_e78170: f64 = (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574);
        let assign60540_e78171: f64 = (2.0 * assign60540_e78170);
        let assign60540_e78173: f64 = (assign60540_e78171 - var_umax__blk1573);
        (assign60540_e78173, ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn6 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn6))) - var_umax__blk1573_dn6), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn7 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn7))) - var_umax__blk1573_dn7), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn8 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn8))) - var_umax__blk1573_dn8), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn9 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn9))) - var_umax__blk1573_dn9), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn11 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn11))) - var_umax__blk1573_dn11), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn12 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn12))) - var_umax__blk1573_dn12),)
    } else {
        (var_ltat__blk1579, var_ltat__blk1579_dn6, var_ltat__blk1579_dn7, var_ltat__blk1579_dn8, var_ltat__blk1579_dn9, var_ltat__blk1579_dn11, var_ltat__blk1579_dn12,)
    }
};
        var_ltat__blk1579 = assign60540_e78175;
        var_ltat__blk1579_dn6 = assign60540_e78175_d_n6;
        var_ltat__blk1579_dn7 = assign60540_e78175_d_n7;
        var_ltat__blk1579_dn8 = assign60540_e78175_d_n8;
        var_ltat__blk1579_dn9 = assign60540_e78175_d_n9;
        var_ltat__blk1579_dn11 = assign60540_e78175_d_n11;
        var_ltat__blk1579_dn12 = assign60540_e78175_d_n12;
        var_ltat__blk1579_rv = 0.0;

        let (assign60550_e78202, assign60550_e78202_d_n6, assign60550_e78202_d_n7, assign60550_e78202_d_n8, assign60550_e78202_d_n9, assign60550_e78202_d_n11, assign60550_e78202_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) {
        let assign60550_e78188: f64 = (var_atatsti_d * var_twoatatoverthreebtat__blk1571);
        let assign60550_e78190: f64 = (assign60550_e78188 * var_sqrtumax__blk1574);
        let assign60550_e78193: f64 = (var_atatsti_d * var_umax__blk1573);
        let assign60550_e78194: f64 = (assign60550_e78190 - assign60550_e78193);
        let assign60550_e78198: f64 = (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575);
        let assign60550_e78199: f64 = (0.5 * assign60550_e78198);
        let assign60550_e78200: f64 = (assign60550_e78194 + assign60550_e78199);
        (assign60550_e78200, (((((var_atatsti_d * var_twoatatoverthreebtat__blk1571_dn6) * var_sqrtumax__blk1574) + (assign60550_e78188 * var_sqrtumax__blk1574_dn6)) - (var_atatsti_d * var_umax__blk1573_dn6)) + (0.5 * ((var_btat__blk1570_dn6 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat__blk1571_dn7) * var_sqrtumax__blk1574) + (assign60550_e78188 * var_sqrtumax__blk1574_dn7)) - (var_atatsti_d * var_umax__blk1573_dn7)) + (0.5 * ((var_btat__blk1570_dn7 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat__blk1571_dn8) * var_sqrtumax__blk1574) + (assign60550_e78188 * var_sqrtumax__blk1574_dn8)) - (var_atatsti_d * var_umax__blk1573_dn8)) + (0.5 * ((var_btat__blk1570_dn8 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn8)))), (((((var_atatsti_d * var_twoatatoverthreebtat__blk1571_dn9) * var_sqrtumax__blk1574) + (assign60550_e78188 * var_sqrtumax__blk1574_dn9)) - (var_atatsti_d * var_umax__blk1573_dn9)) + (0.5 * ((var_btat__blk1570_dn9 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn9)))), (((((var_atatsti_d * var_twoatatoverthreebtat__blk1571_dn11) * var_sqrtumax__blk1574) + (assign60550_e78188 * var_sqrtumax__blk1574_dn11)) - (var_atatsti_d * var_umax__blk1573_dn11)) + (0.5 * ((var_btat__blk1570_dn11 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn11)))), (((((var_atatsti_d * var_twoatatoverthreebtat__blk1571_dn12) * var_sqrtumax__blk1574) + (assign60550_e78188 * var_sqrtumax__blk1574_dn12)) - (var_atatsti_d * var_umax__blk1573_dn12)) + (0.5 * ((var_btat__blk1570_dn12 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn12)))),)
    } else {
        (var_mtat__blk1580, var_mtat__blk1580_dn6, var_mtat__blk1580_dn7, var_mtat__blk1580_dn8, var_mtat__blk1580_dn9, var_mtat__blk1580_dn11, var_mtat__blk1580_dn12,)
    }
};
        var_mtat__blk1580 = assign60550_e78202;
        var_mtat__blk1580_dn6 = assign60550_e78202_d_n6;
        var_mtat__blk1580_dn7 = assign60550_e78202_d_n7;
        var_mtat__blk1580_dn8 = assign60550_e78202_d_n8;
        var_mtat__blk1580_dn9 = assign60550_e78202_d_n9;
        var_mtat__blk1580_dn11 = assign60550_e78202_d_n11;
        var_mtat__blk1580_dn12 = assign60550_e78202_d_n12;
        var_mtat__blk1580_rv = 0.0;

        let (assign60560_e78219, assign60560_e78219_d_n6, assign60560_e78219_d_n7, assign60560_e78219_d_n8, assign60560_e78219_d_n9, assign60560_e78219_d_n11, assign60560_e78219_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) {
        let assign60560_e78215: f64 = (var_ltat__blk1579 - 1.0);
        let assign60560_e78217: f64 = (assign60560_e78215 * var_ktat__blk1578);
        (assign60560_e78217, ((var_ltat__blk1579_dn6 * var_ktat__blk1578) + (assign60560_e78215 * var_ktat__blk1578_dn6)), ((var_ltat__blk1579_dn7 * var_ktat__blk1578) + (assign60560_e78215 * var_ktat__blk1578_dn7)), ((var_ltat__blk1579_dn8 * var_ktat__blk1578) + (assign60560_e78215 * var_ktat__blk1578_dn8)), ((var_ltat__blk1579_dn9 * var_ktat__blk1578) + (assign60560_e78215 * var_ktat__blk1578_dn9)), ((var_ltat__blk1579_dn11 * var_ktat__blk1578) + (assign60560_e78215 * var_ktat__blk1578_dn11)), ((var_ltat__blk1579_dn12 * var_ktat__blk1578) + (assign60560_e78215 * var_ktat__blk1578_dn12)),)
    } else {
        (var_xerfc__blk1581, var_xerfc__blk1581_dn6, var_xerfc__blk1581_dn7, var_xerfc__blk1581_dn8, var_xerfc__blk1581_dn9, var_xerfc__blk1581_dn11, var_xerfc__blk1581_dn12,)
    }
};
        var_xerfc__blk1581 = assign60560_e78219;
        var_xerfc__blk1581_dn6 = assign60560_e78219_d_n6;
        var_xerfc__blk1581_dn7 = assign60560_e78219_d_n7;
        var_xerfc__blk1581_dn8 = assign60560_e78219_d_n8;
        var_xerfc__blk1581_dn9 = assign60560_e78219_d_n9;
        var_xerfc__blk1581_dn11 = assign60560_e78219_d_n11;
        var_xerfc__blk1581_dn12 = assign60560_e78219_d_n12;
        var_xerfc__blk1581_rv = 0.0;

        let (assign60570_e78234, assign60570_e78234_d_n6, assign60570_e78234_d_n7, assign60570_e78234_d_n8, assign60570_e78234_d_n9, assign60570_e78234_d_n11, assign60570_e78234_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) {
        let assign60570_e78232: f64 = (var_xerfc__blk1581 * var_xerfc__blk1581);
        (assign60570_e78232, ((var_xerfc__blk1581_dn6 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn6)), ((var_xerfc__blk1581_dn7 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn7)), ((var_xerfc__blk1581_dn8 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn8)), ((var_xerfc__blk1581_dn9 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn9)), ((var_xerfc__blk1581_dn11 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn11)), ((var_xerfc__blk1581_dn12 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn12)),)
    } else {
        (var_ysq__blk1542, var_ysq__blk1542_dn6, var_ysq__blk1542_dn7, var_ysq__blk1542_dn8, var_ysq__blk1542_dn9, var_ysq__blk1542_dn11, var_ysq__blk1542_dn12,)
    }
};
        var_ysq__blk1542 = assign60570_e78234;
        var_ysq__blk1542_dn6 = assign60570_e78234_d_n6;
        var_ysq__blk1542_dn7 = assign60570_e78234_d_n7;
        var_ysq__blk1542_dn8 = assign60570_e78234_d_n8;
        var_ysq__blk1542_dn9 = assign60570_e78234_d_n9;
        var_ysq__blk1542_dn11 = assign60570_e78234_d_n11;
        var_ysq__blk1542_dn12 = assign60570_e78234_d_n12;
        var_ysq__blk1542_rv = 0.0;

        let assign60610_e78282: f64 = (-var_ysq__blk1542);
        let assign60610_e78284: f64 = (assign60610_e78282 + var_mtat__blk1580);
        let assign60610_e78286: f64 = (-230.25850929940458);
        let assign60610_e78287: f64 = if assign60610_e78284 > assign60610_e78286 { 1.0 } else { 0.0 };
        var_guard1703 = assign60610_e78287;
        var_guard1703_rv = 0.0;

        let (assign60620_e78306, assign60620_e78306_d_n6, assign60620_e78306_d_n7, assign60620_e78306_d_n8, assign60620_e78306_d_n9, assign60620_e78306_d_n11, assign60620_e78306_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) && (var_guard1703 != 0.0)) {
        let assign60620_e78301: f64 = (-var_ysq__blk1542);
        let assign60620_e78303: f64 = (assign60620_e78301 + var_mtat__blk1580);
        let assign60620_e78304: f64 = (assign60620_e78303).exp();
        (assign60620_e78304, (assign60620_e78304 * ((-var_ysq__blk1542_dn6) + var_mtat__blk1580_dn6)), (assign60620_e78304 * ((-var_ysq__blk1542_dn7) + var_mtat__blk1580_dn7)), (assign60620_e78304 * ((-var_ysq__blk1542_dn8) + var_mtat__blk1580_dn8)), (assign60620_e78304 * ((-var_ysq__blk1542_dn9) + var_mtat__blk1580_dn9)), (assign60620_e78304 * ((-var_ysq__blk1542_dn11) + var_mtat__blk1580_dn11)), (assign60620_e78304 * ((-var_ysq__blk1542_dn12) + var_mtat__blk1580_dn12)),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60620_e78306;
        var_tmp__blk1560_dn6 = assign60620_e78306_d_n6;
        var_tmp__blk1560_dn7 = assign60620_e78306_d_n7;
        var_tmp__blk1560_dn8 = assign60620_e78306_d_n8;
        var_tmp__blk1560_dn9 = assign60620_e78306_d_n9;
        var_tmp__blk1560_dn11 = assign60620_e78306_d_n11;
        var_tmp__blk1560_dn12 = assign60620_e78306_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let (assign60630_e78356, assign60630_e78356_d_n6, assign60630_e78356_d_n7, assign60630_e78356_d_n8, assign60630_e78356_d_n9, assign60630_e78356_d_n11, assign60630_e78356_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) && (var_guard1703 == 0.0)) {
        let assign60630_e78323: f64 = (-230.25850929940458);
        let assign60630_e78325: f64 = (-var_ysq__blk1542);
        let assign60630_e78327: f64 = (assign60630_e78325 + var_mtat__blk1580);
        let assign60630_e78328: f64 = (assign60630_e78323 - assign60630_e78327);
        let assign60630_e78332: f64 = (-230.25850929940458);
        let assign60630_e78334: f64 = (-var_ysq__blk1542);
        let assign60630_e78336: f64 = (assign60630_e78334 + var_mtat__blk1580);
        let assign60630_e78337: f64 = (assign60630_e78332 - assign60630_e78336);
        let assign60630_e78340: f64 = (-230.25850929940458);
        let assign60630_e78342: f64 = (-var_ysq__blk1542);
        let assign60630_e78344: f64 = (assign60630_e78342 + var_mtat__blk1580);
        let assign60630_e78345: f64 = (assign60630_e78340 - assign60630_e78344);
        let assign60630_e78347: f64 = (assign60630_e78345 * 0.3333333333333333);
        let assign60630_e78348: f64 = (1.0 + assign60630_e78347);
        let assign60630_e78349: f64 = (assign60630_e78337 * assign60630_e78348);
        let assign60630_e78350: f64 = (0.5 * assign60630_e78349);
        let assign60630_e78351: f64 = (1.0 + assign60630_e78350);
        let assign60630_e78352: f64 = (assign60630_e78328 * assign60630_e78351);
        let assign60630_e78353: f64 = (1.0 + assign60630_e78352);
        let assign60630_e78354: f64 = (1e-100 / assign60630_e78353);
        (assign60630_e78354, (-((1e-100 * (((-((-var_ysq__blk1542_dn6) + var_mtat__blk1580_dn6)) * assign60630_e78351) + (assign60630_e78328 * (0.5 * (((-((-var_ysq__blk1542_dn6) + var_mtat__blk1580_dn6)) * assign60630_e78348) + (assign60630_e78337 * ((-((-var_ysq__blk1542_dn6) + var_mtat__blk1580_dn6)) * 0.3333333333333333))))))) / (assign60630_e78353 * assign60630_e78353))), (-((1e-100 * (((-((-var_ysq__blk1542_dn7) + var_mtat__blk1580_dn7)) * assign60630_e78351) + (assign60630_e78328 * (0.5 * (((-((-var_ysq__blk1542_dn7) + var_mtat__blk1580_dn7)) * assign60630_e78348) + (assign60630_e78337 * ((-((-var_ysq__blk1542_dn7) + var_mtat__blk1580_dn7)) * 0.3333333333333333))))))) / (assign60630_e78353 * assign60630_e78353))), (-((1e-100 * (((-((-var_ysq__blk1542_dn8) + var_mtat__blk1580_dn8)) * assign60630_e78351) + (assign60630_e78328 * (0.5 * (((-((-var_ysq__blk1542_dn8) + var_mtat__blk1580_dn8)) * assign60630_e78348) + (assign60630_e78337 * ((-((-var_ysq__blk1542_dn8) + var_mtat__blk1580_dn8)) * 0.3333333333333333))))))) / (assign60630_e78353 * assign60630_e78353))), (-((1e-100 * (((-((-var_ysq__blk1542_dn9) + var_mtat__blk1580_dn9)) * assign60630_e78351) + (assign60630_e78328 * (0.5 * (((-((-var_ysq__blk1542_dn9) + var_mtat__blk1580_dn9)) * assign60630_e78348) + (assign60630_e78337 * ((-((-var_ysq__blk1542_dn9) + var_mtat__blk1580_dn9)) * 0.3333333333333333))))))) / (assign60630_e78353 * assign60630_e78353))), (-((1e-100 * (((-((-var_ysq__blk1542_dn11) + var_mtat__blk1580_dn11)) * assign60630_e78351) + (assign60630_e78328 * (0.5 * (((-((-var_ysq__blk1542_dn11) + var_mtat__blk1580_dn11)) * assign60630_e78348) + (assign60630_e78337 * ((-((-var_ysq__blk1542_dn11) + var_mtat__blk1580_dn11)) * 0.3333333333333333))))))) / (assign60630_e78353 * assign60630_e78353))), (-((1e-100 * (((-((-var_ysq__blk1542_dn12) + var_mtat__blk1580_dn12)) * assign60630_e78351) + (assign60630_e78328 * (0.5 * (((-((-var_ysq__blk1542_dn12) + var_mtat__blk1580_dn12)) * assign60630_e78348) + (assign60630_e78337 * ((-((-var_ysq__blk1542_dn12) + var_mtat__blk1580_dn12)) * 0.3333333333333333))))))) / (assign60630_e78353 * assign60630_e78353))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60630_e78356;
        var_tmp__blk1560_dn6 = assign60630_e78356_d_n6;
        var_tmp__blk1560_dn7 = assign60630_e78356_d_n7;
        var_tmp__blk1560_dn8 = assign60630_e78356_d_n8;
        var_tmp__blk1560_dn9 = assign60630_e78356_d_n9;
        var_tmp__blk1560_dn11 = assign60630_e78356_d_n11;
        var_tmp__blk1560_dn12 = assign60630_e78356_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let assign60650_e78390: f64 = if var_xerfc__blk1581 > 0.0 { 1.0 } else { 0.0 };
        var_guard1704 = assign60650_e78390;
        var_guard1704_rv = 0.0;

        let assign60670_e78408: f64 = (-230.25850929940458);
        let assign60670_e78409: f64 = if var_mtat__blk1580 > assign60670_e78408 { 1.0 } else { 0.0 };
        var_guard1705 = assign60670_e78409;
        var_guard1705_rv = 0.0;

        let (assign60680_e78428, assign60680_e78428_d_n6, assign60680_e78428_d_n7, assign60680_e78428_d_n8, assign60680_e78428_d_n9, assign60680_e78428_d_n11, assign60680_e78428_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) && (var_guard1704 == 0.0)) && (var_guard1705 != 0.0)) {
        let assign60680_e78426: f64 = (var_mtat__blk1580).exp();
        (assign60680_e78426, (assign60680_e78426 * var_mtat__blk1580_dn6), (assign60680_e78426 * var_mtat__blk1580_dn7), (assign60680_e78426 * var_mtat__blk1580_dn8), (assign60680_e78426 * var_mtat__blk1580_dn9), (assign60680_e78426 * var_mtat__blk1580_dn11), (assign60680_e78426 * var_mtat__blk1580_dn12),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60680_e78428;
        var_tmp__blk1560_dn6 = assign60680_e78428_d_n6;
        var_tmp__blk1560_dn7 = assign60680_e78428_d_n7;
        var_tmp__blk1560_dn8 = assign60680_e78428_d_n8;
        var_tmp__blk1560_dn9 = assign60680_e78428_d_n9;
        var_tmp__blk1560_dn11 = assign60680_e78428_d_n11;
        var_tmp__blk1560_dn12 = assign60680_e78428_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let (assign60690_e78472, assign60690_e78472_d_n6, assign60690_e78472_d_n7, assign60690_e78472_d_n8, assign60690_e78472_d_n9, assign60690_e78472_d_n11, assign60690_e78472_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1700 == 0.0)) && (var_guard1704 == 0.0)) && (var_guard1705 == 0.0)) {
        let assign60690_e78448: f64 = (-230.25850929940458);
        let assign60690_e78450: f64 = (assign60690_e78448 - var_mtat__blk1580);
        let assign60690_e78454: f64 = (-230.25850929940458);
        let assign60690_e78456: f64 = (assign60690_e78454 - var_mtat__blk1580);
        let assign60690_e78459: f64 = (-230.25850929940458);
        let assign60690_e78461: f64 = (assign60690_e78459 - var_mtat__blk1580);
        let assign60690_e78463: f64 = (assign60690_e78461 * 0.3333333333333333);
        let assign60690_e78464: f64 = (1.0 + assign60690_e78463);
        let assign60690_e78465: f64 = (assign60690_e78456 * assign60690_e78464);
        let assign60690_e78466: f64 = (0.5 * assign60690_e78465);
        let assign60690_e78467: f64 = (1.0 + assign60690_e78466);
        let assign60690_e78468: f64 = (assign60690_e78450 * assign60690_e78467);
        let assign60690_e78469: f64 = (1.0 + assign60690_e78468);
        let assign60690_e78470: f64 = (1e-100 / assign60690_e78469);
        (assign60690_e78470, (-((1e-100 * (((-var_mtat__blk1580_dn6) * assign60690_e78467) + (assign60690_e78450 * (0.5 * (((-var_mtat__blk1580_dn6) * assign60690_e78464) + (assign60690_e78456 * ((-var_mtat__blk1580_dn6) * 0.3333333333333333))))))) / (assign60690_e78469 * assign60690_e78469))), (-((1e-100 * (((-var_mtat__blk1580_dn7) * assign60690_e78467) + (assign60690_e78450 * (0.5 * (((-var_mtat__blk1580_dn7) * assign60690_e78464) + (assign60690_e78456 * ((-var_mtat__blk1580_dn7) * 0.3333333333333333))))))) / (assign60690_e78469 * assign60690_e78469))), (-((1e-100 * (((-var_mtat__blk1580_dn8) * assign60690_e78467) + (assign60690_e78450 * (0.5 * (((-var_mtat__blk1580_dn8) * assign60690_e78464) + (assign60690_e78456 * ((-var_mtat__blk1580_dn8) * 0.3333333333333333))))))) / (assign60690_e78469 * assign60690_e78469))), (-((1e-100 * (((-var_mtat__blk1580_dn9) * assign60690_e78467) + (assign60690_e78450 * (0.5 * (((-var_mtat__blk1580_dn9) * assign60690_e78464) + (assign60690_e78456 * ((-var_mtat__blk1580_dn9) * 0.3333333333333333))))))) / (assign60690_e78469 * assign60690_e78469))), (-((1e-100 * (((-var_mtat__blk1580_dn11) * assign60690_e78467) + (assign60690_e78450 * (0.5 * (((-var_mtat__blk1580_dn11) * assign60690_e78464) + (assign60690_e78456 * ((-var_mtat__blk1580_dn11) * 0.3333333333333333))))))) / (assign60690_e78469 * assign60690_e78469))), (-((1e-100 * (((-var_mtat__blk1580_dn12) * assign60690_e78467) + (assign60690_e78450 * (0.5 * (((-var_mtat__blk1580_dn12) * assign60690_e78464) + (assign60690_e78456 * ((-var_mtat__blk1580_dn12) * 0.3333333333333333))))))) / (assign60690_e78469 * assign60690_e78469))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60690_e78472;
        var_tmp__blk1560_dn6 = assign60690_e78472_d_n6;
        var_tmp__blk1560_dn7 = assign60690_e78472_d_n7;
        var_tmp__blk1560_dn8 = assign60690_e78472_d_n8;
        var_tmp__blk1560_dn9 = assign60690_e78472_d_n9;
        var_tmp__blk1560_dn11 = assign60690_e78472_d_n11;
        var_tmp__blk1560_dn12 = assign60690_e78472_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let assign60730_e78535: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard1706 = assign60730_e78535;
        var_guard1706_rv = 0.0;

        let assign60750_e78550: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard1707 = assign60750_e78550;
        var_guard1707_rv = 0.0;

        let (assign60760_e78570, assign60760_e78570_d_n6, assign60760_e78570_d_n7, assign60760_e78570_d_n8, assign60760_e78570_d_n9, assign60760_e78570_d_n11, assign60760_e78570_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1706 == 0.0)) && (var_guard1707 != 0.0)) {
        let assign60760_e78565: f64 = (var_vbirstid_i - var_vbbt__blk1558);
        let assign60760_e78567: f64 = (assign60760_e78565 * var_vbirstiinv_d);
        let assign60760_e78568: f64 = (assign60760_e78567).sqrt();
        (assign60760_e78568, 0.0, (((-var_vbbt__blk1558_dn7) * var_vbirstiinv_d) / (2.0 * assign60760_e78568)), (((-var_vbbt__blk1558_dn8) * var_vbirstiinv_d) / (2.0 * assign60760_e78568)), 0.0, (((-var_vbbt__blk1558_dn11) * var_vbirstiinv_d) / (2.0 * assign60760_e78568)), (((-var_vbbt__blk1558_dn12) * var_vbirstiinv_d) / (2.0 * assign60760_e78568)),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60760_e78570;
        var_tmp__blk1560_dn6 = assign60760_e78570_d_n6;
        var_tmp__blk1560_dn7 = assign60760_e78570_d_n7;
        var_tmp__blk1560_dn8 = assign60760_e78570_d_n8;
        var_tmp__blk1560_dn9 = assign60760_e78570_d_n9;
        var_tmp__blk1560_dn11 = assign60760_e78570_d_n11;
        var_tmp__blk1560_dn12 = assign60760_e78570_d_n12;
        var_tmp__blk1560_rv = 0.0;

        *var_btat__blk1570_slot = var_btat__blk1570;
        *var_btat__blk1570_dn11_slot = var_btat__blk1570_dn11;
        *var_btat__blk1570_dn12_slot = var_btat__blk1570_dn12;
        *var_btat__blk1570_dn6_slot = var_btat__blk1570_dn6;
        *var_btat__blk1570_dn7_slot = var_btat__blk1570_dn7;
        *var_btat__blk1570_dn8_slot = var_btat__blk1570_dn8;
        *var_btat__blk1570_dn9_slot = var_btat__blk1570_dn9;
        *var_btat__blk1570_rv_slot = var_btat__blk1570_rv;
        *var_guard1696_slot = var_guard1696;
        *var_guard1696_rv_slot = var_guard1696_rv;
        *var_guard1697_slot = var_guard1697;
        *var_guard1697_rv_slot = var_guard1697_rv;
        *var_guard1699_slot = var_guard1699;
        *var_guard1699_rv_slot = var_guard1699_rv;
        *var_guard1700_slot = var_guard1700;
        *var_guard1700_rv_slot = var_guard1700_rv;
        *var_guard1703_slot = var_guard1703;
        *var_guard1703_rv_slot = var_guard1703_rv;
        *var_guard1704_slot = var_guard1704;
        *var_guard1704_rv_slot = var_guard1704_rv;
        *var_guard1705_slot = var_guard1705;
        *var_guard1705_rv_slot = var_guard1705_rv;
        *var_guard1706_slot = var_guard1706;
        *var_guard1706_rv_slot = var_guard1706_rv;
        *var_guard1707_slot = var_guard1707;
        *var_guard1707_rv_slot = var_guard1707_rv;
        *var_ktat__blk1578_slot = var_ktat__blk1578;
        *var_ktat__blk1578_dn11_slot = var_ktat__blk1578_dn11;
        *var_ktat__blk1578_dn12_slot = var_ktat__blk1578_dn12;
        *var_ktat__blk1578_dn6_slot = var_ktat__blk1578_dn6;
        *var_ktat__blk1578_dn7_slot = var_ktat__blk1578_dn7;
        *var_ktat__blk1578_dn8_slot = var_ktat__blk1578_dn8;
        *var_ktat__blk1578_dn9_slot = var_ktat__blk1578_dn9;
        *var_ktat__blk1578_rv_slot = var_ktat__blk1578_rv;
        *var_ltat__blk1579_slot = var_ltat__blk1579;
        *var_ltat__blk1579_dn11_slot = var_ltat__blk1579_dn11;
        *var_ltat__blk1579_dn12_slot = var_ltat__blk1579_dn12;
        *var_ltat__blk1579_dn6_slot = var_ltat__blk1579_dn6;
        *var_ltat__blk1579_dn7_slot = var_ltat__blk1579_dn7;
        *var_ltat__blk1579_dn8_slot = var_ltat__blk1579_dn8;
        *var_ltat__blk1579_dn9_slot = var_ltat__blk1579_dn9;
        *var_ltat__blk1579_rv_slot = var_ltat__blk1579_rv;
        *var_mtat__blk1580_slot = var_mtat__blk1580;
        *var_mtat__blk1580_dn11_slot = var_mtat__blk1580_dn11;
        *var_mtat__blk1580_dn12_slot = var_mtat__blk1580_dn12;
        *var_mtat__blk1580_dn6_slot = var_mtat__blk1580_dn6;
        *var_mtat__blk1580_dn7_slot = var_mtat__blk1580_dn7;
        *var_mtat__blk1580_dn8_slot = var_mtat__blk1580_dn8;
        *var_mtat__blk1580_dn9_slot = var_mtat__blk1580_dn9;
        *var_mtat__blk1580_rv_slot = var_mtat__blk1580_rv;
        *var_qjunbot_d_slot = var_qjunbot_d;
        *var_qjunbot_d_dn11_slot = var_qjunbot_d_dn11;
        *var_qjunbot_d_dn12_slot = var_qjunbot_d_dn12;
        *var_qjunbot_d_dn6_slot = var_qjunbot_d_dn6;
        *var_qjunbot_d_dn7_slot = var_qjunbot_d_dn7;
        *var_qjunbot_d_dn8_slot = var_qjunbot_d_dn8;
        *var_qjunbot_d_dn9_slot = var_qjunbot_d_dn9;
        *var_qjunbot_d_rv_slot = var_qjunbot_d_rv;
        *var_qjunsti_d_slot = var_qjunsti_d;
        *var_qjunsti_d_dn11_slot = var_qjunsti_d_dn11;
        *var_qjunsti_d_dn12_slot = var_qjunsti_d_dn12;
        *var_qjunsti_d_dn6_slot = var_qjunsti_d_dn6;
        *var_qjunsti_d_dn7_slot = var_qjunsti_d_dn7;
        *var_qjunsti_d_dn8_slot = var_qjunsti_d_dn8;
        *var_qjunsti_d_dn9_slot = var_qjunsti_d_dn9;
        *var_qjunsti_d_rv_slot = var_qjunsti_d_rv;
        *var_sqrtumax__blk1574_slot = var_sqrtumax__blk1574;
        *var_sqrtumax__blk1574_dn11_slot = var_sqrtumax__blk1574_dn11;
        *var_sqrtumax__blk1574_dn12_slot = var_sqrtumax__blk1574_dn12;
        *var_sqrtumax__blk1574_dn6_slot = var_sqrtumax__blk1574_dn6;
        *var_sqrtumax__blk1574_dn7_slot = var_sqrtumax__blk1574_dn7;
        *var_sqrtumax__blk1574_dn8_slot = var_sqrtumax__blk1574_dn8;
        *var_sqrtumax__blk1574_dn9_slot = var_sqrtumax__blk1574_dn9;
        *var_sqrtumax__blk1574_rv_slot = var_sqrtumax__blk1574_rv;
        *var_tmp__blk1560_slot = var_tmp__blk1560;
        *var_tmp__blk1560_dn11_slot = var_tmp__blk1560_dn11;
        *var_tmp__blk1560_dn12_slot = var_tmp__blk1560_dn12;
        *var_tmp__blk1560_dn6_slot = var_tmp__blk1560_dn6;
        *var_tmp__blk1560_dn7_slot = var_tmp__blk1560_dn7;
        *var_tmp__blk1560_dn8_slot = var_tmp__blk1560_dn8;
        *var_tmp__blk1560_dn9_slot = var_tmp__blk1560_dn9;
        *var_tmp__blk1560_rv_slot = var_tmp__blk1560_rv;
        *var_twoatatoverthreebtat__blk1571_slot = var_twoatatoverthreebtat__blk1571;
        *var_twoatatoverthreebtat__blk1571_dn11_slot = var_twoatatoverthreebtat__blk1571_dn11;
        *var_twoatatoverthreebtat__blk1571_dn12_slot = var_twoatatoverthreebtat__blk1571_dn12;
        *var_twoatatoverthreebtat__blk1571_dn6_slot = var_twoatatoverthreebtat__blk1571_dn6;
        *var_twoatatoverthreebtat__blk1571_dn7_slot = var_twoatatoverthreebtat__blk1571_dn7;
        *var_twoatatoverthreebtat__blk1571_dn8_slot = var_twoatatoverthreebtat__blk1571_dn8;
        *var_twoatatoverthreebtat__blk1571_dn9_slot = var_twoatatoverthreebtat__blk1571_dn9;
        *var_twoatatoverthreebtat__blk1571_rv_slot = var_twoatatoverthreebtat__blk1571_rv;
        *var_umax__blk1573_slot = var_umax__blk1573;
        *var_umax__blk1573_dn11_slot = var_umax__blk1573_dn11;
        *var_umax__blk1573_dn12_slot = var_umax__blk1573_dn12;
        *var_umax__blk1573_dn6_slot = var_umax__blk1573_dn6;
        *var_umax__blk1573_dn7_slot = var_umax__blk1573_dn7;
        *var_umax__blk1573_dn8_slot = var_umax__blk1573_dn8;
        *var_umax__blk1573_dn9_slot = var_umax__blk1573_dn9;
        *var_umax__blk1573_rv_slot = var_umax__blk1573_rv;
        *var_umaxbeforelimiting__blk1572_slot = var_umaxbeforelimiting__blk1572;
        *var_umaxbeforelimiting__blk1572_dn11_slot = var_umaxbeforelimiting__blk1572_dn11;
        *var_umaxbeforelimiting__blk1572_dn12_slot = var_umaxbeforelimiting__blk1572_dn12;
        *var_umaxbeforelimiting__blk1572_dn6_slot = var_umaxbeforelimiting__blk1572_dn6;
        *var_umaxbeforelimiting__blk1572_dn7_slot = var_umaxbeforelimiting__blk1572_dn7;
        *var_umaxbeforelimiting__blk1572_dn8_slot = var_umaxbeforelimiting__blk1572_dn8;
        *var_umaxbeforelimiting__blk1572_dn9_slot = var_umaxbeforelimiting__blk1572_dn9;
        *var_umaxbeforelimiting__blk1572_rv_slot = var_umaxbeforelimiting__blk1572_rv;
        *var_umaxpoweronepointfive__blk1575_slot = var_umaxpoweronepointfive__blk1575;
        *var_umaxpoweronepointfive__blk1575_dn11_slot = var_umaxpoweronepointfive__blk1575_dn11;
        *var_umaxpoweronepointfive__blk1575_dn12_slot = var_umaxpoweronepointfive__blk1575_dn12;
        *var_umaxpoweronepointfive__blk1575_dn6_slot = var_umaxpoweronepointfive__blk1575_dn6;
        *var_umaxpoweronepointfive__blk1575_dn7_slot = var_umaxpoweronepointfive__blk1575_dn7;
        *var_umaxpoweronepointfive__blk1575_dn8_slot = var_umaxpoweronepointfive__blk1575_dn8;
        *var_umaxpoweronepointfive__blk1575_dn9_slot = var_umaxpoweronepointfive__blk1575_dn9;
        *var_umaxpoweronepointfive__blk1575_rv_slot = var_umaxpoweronepointfive__blk1575_rv;
        *var_vbi_minus_vjsrh__blk1563_slot = var_vbi_minus_vjsrh__blk1563;
        *var_vbi_minus_vjsrh__blk1563_dn11_slot = var_vbi_minus_vjsrh__blk1563_dn11;
        *var_vbi_minus_vjsrh__blk1563_dn12_slot = var_vbi_minus_vjsrh__blk1563_dn12;
        *var_vbi_minus_vjsrh__blk1563_dn7_slot = var_vbi_minus_vjsrh__blk1563_dn7;
        *var_vbi_minus_vjsrh__blk1563_dn8_slot = var_vbi_minus_vjsrh__blk1563_dn8;
        *var_vbi_minus_vjsrh__blk1563_rv_slot = var_vbi_minus_vjsrh__blk1563_rv;
        *var_wdep__blk1567_slot = var_wdep__blk1567;
        *var_wdep__blk1567_dn11_slot = var_wdep__blk1567_dn11;
        *var_wdep__blk1567_dn12_slot = var_wdep__blk1567_dn12;
        *var_wdep__blk1567_dn6_slot = var_wdep__blk1567_dn6;
        *var_wdep__blk1567_dn7_slot = var_wdep__blk1567_dn7;
        *var_wdep__blk1567_dn8_slot = var_wdep__blk1567_dn8;
        *var_wdep__blk1567_dn9_slot = var_wdep__blk1567_dn9;
        *var_wdep__blk1567_rv_slot = var_wdep__blk1567_rv;
        *var_xerfc__blk1581_slot = var_xerfc__blk1581;
        *var_xerfc__blk1581_dn11_slot = var_xerfc__blk1581_dn11;
        *var_xerfc__blk1581_dn12_slot = var_xerfc__blk1581_dn12;
        *var_xerfc__blk1581_dn6_slot = var_xerfc__blk1581_dn6;
        *var_xerfc__blk1581_dn7_slot = var_xerfc__blk1581_dn7;
        *var_xerfc__blk1581_dn8_slot = var_xerfc__blk1581_dn8;
        *var_xerfc__blk1581_dn9_slot = var_xerfc__blk1581_dn9;
        *var_xerfc__blk1581_rv_slot = var_xerfc__blk1581_rv;
        *var_ysq__blk1542_slot = var_ysq__blk1542;
        *var_ysq__blk1542_dn11_slot = var_ysq__blk1542_dn11;
        *var_ysq__blk1542_dn12_slot = var_ysq__blk1542_dn12;
        *var_ysq__blk1542_dn6_slot = var_ysq__blk1542_dn6;
        *var_ysq__blk1542_dn7_slot = var_ysq__blk1542_dn7;
        *var_ysq__blk1542_dn8_slot = var_ysq__blk1542_dn8;
        *var_ysq__blk1542_dn9_slot = var_ysq__blk1542_dn9;
        *var_ysq__blk1542_rv_slot = var_ysq__blk1542_rv;
    }

    pub(super) fn stamp_reactive_block_76(
        p: &Parameters,
        var_alphaav: f64,
        var_atatgat_d: f64,
        var_btatpartgat_d: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_fbbtsti_d: f64,
        var_guard1589: f64,
        var_guard1590: f64,
        var_guard1696: f64,
        var_guard1706: f64,
        var_guard1707: f64,
        var_lgdrain_i: f64,
        var_one_minus_pgat_d: f64,
        var_one_minus_psti_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbrstid_i: f64,
        var_pgatd_i: f64,
        var_pstid_i: f64,
        var_qpref2sti_d: f64,
        var_qprefsti_d: f64,
        var_vav__blk1559: f64,
        var_vav__blk1559_dn11: f64,
        var_vav__blk1559_dn12: f64,
        var_vav__blk1559_dn7: f64,
        var_vav__blk1559_dn8: f64,
        var_vbbt__blk1558: f64,
        var_vbbt__blk1558_dn11: f64,
        var_vbbt__blk1558_dn12: f64,
        var_vbbt__blk1558_dn7: f64,
        var_vbbt__blk1558_dn8: f64,
        var_vbigat_d: f64,
        var_vbiinvsti_d: f64,
        var_vbirgatinv_d: f64,
        var_vbirstid_i: f64,
        var_vbirstiinv_d: f64,
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
        var_wdepnulrinvsti_d: f64,
        var_btat__blk1570_slot: &mut f64,
        var_btat__blk1570_dn11_slot: &mut f64,
        var_btat__blk1570_dn12_slot: &mut f64,
        var_btat__blk1570_dn6_slot: &mut f64,
        var_btat__blk1570_dn7_slot: &mut f64,
        var_btat__blk1570_dn8_slot: &mut f64,
        var_btat__blk1570_dn9_slot: &mut f64,
        var_btat__blk1570_rv_slot: &mut f64,
        var_fmaxr__blk1585_slot: &mut f64,
        var_fmaxr__blk1585_dn11_slot: &mut f64,
        var_fmaxr__blk1585_dn12_slot: &mut f64,
        var_fmaxr__blk1585_dn6_slot: &mut f64,
        var_fmaxr__blk1585_dn7_slot: &mut f64,
        var_fmaxr__blk1585_dn8_slot: &mut f64,
        var_fmaxr__blk1585_dn9_slot: &mut f64,
        var_fmaxr__blk1585_rv_slot: &mut f64,
        var_guard1708_slot: &mut f64,
        var_guard1708_rv_slot: &mut f64,
        var_guard1709_slot: &mut f64,
        var_guard1709_rv_slot: &mut f64,
        var_guard1710_slot: &mut f64,
        var_guard1710_rv_slot: &mut f64,
        var_guard1711_slot: &mut f64,
        var_guard1711_rv_slot: &mut f64,
        var_guard1712_slot: &mut f64,
        var_guard1712_rv_slot: &mut f64,
        var_guard1713_slot: &mut f64,
        var_guard1713_rv_slot: &mut f64,
        var_guard1714_slot: &mut f64,
        var_guard1714_rv_slot: &mut f64,
        var_guard1715_slot: &mut f64,
        var_guard1715_rv_slot: &mut f64,
        var_guard1717_slot: &mut f64,
        var_guard1717_rv_slot: &mut f64,
        var_guard1718_slot: &mut f64,
        var_guard1718_rv_slot: &mut f64,
        var_ktat__blk1578_slot: &mut f64,
        var_ktat__blk1578_dn11_slot: &mut f64,
        var_ktat__blk1578_dn12_slot: &mut f64,
        var_ktat__blk1578_dn6_slot: &mut f64,
        var_ktat__blk1578_dn7_slot: &mut f64,
        var_ktat__blk1578_dn8_slot: &mut f64,
        var_ktat__blk1578_dn9_slot: &mut f64,
        var_ktat__blk1578_rv_slot: &mut f64,
        var_ltat__blk1579_slot: &mut f64,
        var_ltat__blk1579_dn11_slot: &mut f64,
        var_ltat__blk1579_dn12_slot: &mut f64,
        var_ltat__blk1579_dn6_slot: &mut f64,
        var_ltat__blk1579_dn7_slot: &mut f64,
        var_ltat__blk1579_dn8_slot: &mut f64,
        var_ltat__blk1579_dn9_slot: &mut f64,
        var_ltat__blk1579_rv_slot: &mut f64,
        var_qjungat_d_slot: &mut f64,
        var_qjungat_d_dn11_slot: &mut f64,
        var_qjungat_d_dn12_slot: &mut f64,
        var_qjungat_d_dn6_slot: &mut f64,
        var_qjungat_d_dn7_slot: &mut f64,
        var_qjungat_d_dn8_slot: &mut f64,
        var_qjungat_d_dn9_slot: &mut f64,
        var_qjungat_d_rv_slot: &mut f64,
        var_qjunsti_d_slot: &mut f64,
        var_qjunsti_d_dn11_slot: &mut f64,
        var_qjunsti_d_dn12_slot: &mut f64,
        var_qjunsti_d_dn6_slot: &mut f64,
        var_qjunsti_d_dn7_slot: &mut f64,
        var_qjunsti_d_dn8_slot: &mut f64,
        var_qjunsti_d_dn9_slot: &mut f64,
        var_qjunsti_d_rv_slot: &mut f64,
        var_sqrtumax__blk1574_slot: &mut f64,
        var_sqrtumax__blk1574_dn11_slot: &mut f64,
        var_sqrtumax__blk1574_dn12_slot: &mut f64,
        var_sqrtumax__blk1574_dn6_slot: &mut f64,
        var_sqrtumax__blk1574_dn7_slot: &mut f64,
        var_sqrtumax__blk1574_dn8_slot: &mut f64,
        var_sqrtumax__blk1574_dn9_slot: &mut f64,
        var_sqrtumax__blk1574_rv_slot: &mut f64,
        var_tmp__blk1560_slot: &mut f64,
        var_tmp__blk1560_dn11_slot: &mut f64,
        var_tmp__blk1560_dn12_slot: &mut f64,
        var_tmp__blk1560_dn6_slot: &mut f64,
        var_tmp__blk1560_dn7_slot: &mut f64,
        var_tmp__blk1560_dn8_slot: &mut f64,
        var_tmp__blk1560_dn9_slot: &mut f64,
        var_tmp__blk1560_rv_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn11_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn12_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn6_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn7_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn8_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn9_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_rv_slot: &mut f64,
        var_umax__blk1573_slot: &mut f64,
        var_umax__blk1573_dn11_slot: &mut f64,
        var_umax__blk1573_dn12_slot: &mut f64,
        var_umax__blk1573_dn6_slot: &mut f64,
        var_umax__blk1573_dn7_slot: &mut f64,
        var_umax__blk1573_dn8_slot: &mut f64,
        var_umax__blk1573_dn9_slot: &mut f64,
        var_umax__blk1573_rv_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn11_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn12_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn6_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn7_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn8_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn9_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_rv_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn11_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn12_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn6_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn7_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn8_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn9_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_rv_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_dn11_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_dn12_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_dn7_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_rv_slot: &mut f64,
        var_wdep__blk1567_slot: &mut f64,
        var_wdep__blk1567_dn11_slot: &mut f64,
        var_wdep__blk1567_dn12_slot: &mut f64,
        var_wdep__blk1567_dn6_slot: &mut f64,
        var_wdep__blk1567_dn7_slot: &mut f64,
        var_wdep__blk1567_dn8_slot: &mut f64,
        var_wdep__blk1567_dn9_slot: &mut f64,
        var_wdep__blk1567_rv_slot: &mut f64,
    ) {
        let mut var_btat__blk1570: f64 = *var_btat__blk1570_slot;
        let mut var_btat__blk1570_dn11: f64 = *var_btat__blk1570_dn11_slot;
        let mut var_btat__blk1570_dn12: f64 = *var_btat__blk1570_dn12_slot;
        let mut var_btat__blk1570_dn6: f64 = *var_btat__blk1570_dn6_slot;
        let mut var_btat__blk1570_dn7: f64 = *var_btat__blk1570_dn7_slot;
        let mut var_btat__blk1570_dn8: f64 = *var_btat__blk1570_dn8_slot;
        let mut var_btat__blk1570_dn9: f64 = *var_btat__blk1570_dn9_slot;
        let mut var_btat__blk1570_rv: f64 = *var_btat__blk1570_rv_slot;
        let mut var_fmaxr__blk1585: f64 = *var_fmaxr__blk1585_slot;
        let mut var_fmaxr__blk1585_dn11: f64 = *var_fmaxr__blk1585_dn11_slot;
        let mut var_fmaxr__blk1585_dn12: f64 = *var_fmaxr__blk1585_dn12_slot;
        let mut var_fmaxr__blk1585_dn6: f64 = *var_fmaxr__blk1585_dn6_slot;
        let mut var_fmaxr__blk1585_dn7: f64 = *var_fmaxr__blk1585_dn7_slot;
        let mut var_fmaxr__blk1585_dn8: f64 = *var_fmaxr__blk1585_dn8_slot;
        let mut var_fmaxr__blk1585_dn9: f64 = *var_fmaxr__blk1585_dn9_slot;
        let mut var_fmaxr__blk1585_rv: f64 = *var_fmaxr__blk1585_rv_slot;
        let mut var_guard1708: f64 = *var_guard1708_slot;
        let mut var_guard1708_rv: f64 = *var_guard1708_rv_slot;
        let mut var_guard1709: f64 = *var_guard1709_slot;
        let mut var_guard1709_rv: f64 = *var_guard1709_rv_slot;
        let mut var_guard1710: f64 = *var_guard1710_slot;
        let mut var_guard1710_rv: f64 = *var_guard1710_rv_slot;
        let mut var_guard1711: f64 = *var_guard1711_slot;
        let mut var_guard1711_rv: f64 = *var_guard1711_rv_slot;
        let mut var_guard1712: f64 = *var_guard1712_slot;
        let mut var_guard1712_rv: f64 = *var_guard1712_rv_slot;
        let mut var_guard1713: f64 = *var_guard1713_slot;
        let mut var_guard1713_rv: f64 = *var_guard1713_rv_slot;
        let mut var_guard1714: f64 = *var_guard1714_slot;
        let mut var_guard1714_rv: f64 = *var_guard1714_rv_slot;
        let mut var_guard1715: f64 = *var_guard1715_slot;
        let mut var_guard1715_rv: f64 = *var_guard1715_rv_slot;
        let mut var_guard1717: f64 = *var_guard1717_slot;
        let mut var_guard1717_rv: f64 = *var_guard1717_rv_slot;
        let mut var_guard1718: f64 = *var_guard1718_slot;
        let mut var_guard1718_rv: f64 = *var_guard1718_rv_slot;
        let mut var_ktat__blk1578: f64 = *var_ktat__blk1578_slot;
        let mut var_ktat__blk1578_dn11: f64 = *var_ktat__blk1578_dn11_slot;
        let mut var_ktat__blk1578_dn12: f64 = *var_ktat__blk1578_dn12_slot;
        let mut var_ktat__blk1578_dn6: f64 = *var_ktat__blk1578_dn6_slot;
        let mut var_ktat__blk1578_dn7: f64 = *var_ktat__blk1578_dn7_slot;
        let mut var_ktat__blk1578_dn8: f64 = *var_ktat__blk1578_dn8_slot;
        let mut var_ktat__blk1578_dn9: f64 = *var_ktat__blk1578_dn9_slot;
        let mut var_ktat__blk1578_rv: f64 = *var_ktat__blk1578_rv_slot;
        let mut var_ltat__blk1579: f64 = *var_ltat__blk1579_slot;
        let mut var_ltat__blk1579_dn11: f64 = *var_ltat__blk1579_dn11_slot;
        let mut var_ltat__blk1579_dn12: f64 = *var_ltat__blk1579_dn12_slot;
        let mut var_ltat__blk1579_dn6: f64 = *var_ltat__blk1579_dn6_slot;
        let mut var_ltat__blk1579_dn7: f64 = *var_ltat__blk1579_dn7_slot;
        let mut var_ltat__blk1579_dn8: f64 = *var_ltat__blk1579_dn8_slot;
        let mut var_ltat__blk1579_dn9: f64 = *var_ltat__blk1579_dn9_slot;
        let mut var_ltat__blk1579_rv: f64 = *var_ltat__blk1579_rv_slot;
        let mut var_qjungat_d: f64 = *var_qjungat_d_slot;
        let mut var_qjungat_d_dn11: f64 = *var_qjungat_d_dn11_slot;
        let mut var_qjungat_d_dn12: f64 = *var_qjungat_d_dn12_slot;
        let mut var_qjungat_d_dn6: f64 = *var_qjungat_d_dn6_slot;
        let mut var_qjungat_d_dn7: f64 = *var_qjungat_d_dn7_slot;
        let mut var_qjungat_d_dn8: f64 = *var_qjungat_d_dn8_slot;
        let mut var_qjungat_d_dn9: f64 = *var_qjungat_d_dn9_slot;
        let mut var_qjungat_d_rv: f64 = *var_qjungat_d_rv_slot;
        let mut var_qjunsti_d: f64 = *var_qjunsti_d_slot;
        let mut var_qjunsti_d_dn11: f64 = *var_qjunsti_d_dn11_slot;
        let mut var_qjunsti_d_dn12: f64 = *var_qjunsti_d_dn12_slot;
        let mut var_qjunsti_d_dn6: f64 = *var_qjunsti_d_dn6_slot;
        let mut var_qjunsti_d_dn7: f64 = *var_qjunsti_d_dn7_slot;
        let mut var_qjunsti_d_dn8: f64 = *var_qjunsti_d_dn8_slot;
        let mut var_qjunsti_d_dn9: f64 = *var_qjunsti_d_dn9_slot;
        let mut var_qjunsti_d_rv: f64 = *var_qjunsti_d_rv_slot;
        let mut var_sqrtumax__blk1574: f64 = *var_sqrtumax__blk1574_slot;
        let mut var_sqrtumax__blk1574_dn11: f64 = *var_sqrtumax__blk1574_dn11_slot;
        let mut var_sqrtumax__blk1574_dn12: f64 = *var_sqrtumax__blk1574_dn12_slot;
        let mut var_sqrtumax__blk1574_dn6: f64 = *var_sqrtumax__blk1574_dn6_slot;
        let mut var_sqrtumax__blk1574_dn7: f64 = *var_sqrtumax__blk1574_dn7_slot;
        let mut var_sqrtumax__blk1574_dn8: f64 = *var_sqrtumax__blk1574_dn8_slot;
        let mut var_sqrtumax__blk1574_dn9: f64 = *var_sqrtumax__blk1574_dn9_slot;
        let mut var_sqrtumax__blk1574_rv: f64 = *var_sqrtumax__blk1574_rv_slot;
        let mut var_tmp__blk1560: f64 = *var_tmp__blk1560_slot;
        let mut var_tmp__blk1560_dn11: f64 = *var_tmp__blk1560_dn11_slot;
        let mut var_tmp__blk1560_dn12: f64 = *var_tmp__blk1560_dn12_slot;
        let mut var_tmp__blk1560_dn6: f64 = *var_tmp__blk1560_dn6_slot;
        let mut var_tmp__blk1560_dn7: f64 = *var_tmp__blk1560_dn7_slot;
        let mut var_tmp__blk1560_dn8: f64 = *var_tmp__blk1560_dn8_slot;
        let mut var_tmp__blk1560_dn9: f64 = *var_tmp__blk1560_dn9_slot;
        let mut var_tmp__blk1560_rv: f64 = *var_tmp__blk1560_rv_slot;
        let mut var_twoatatoverthreebtat__blk1571: f64 = *var_twoatatoverthreebtat__blk1571_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn11: f64 = *var_twoatatoverthreebtat__blk1571_dn11_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn12: f64 = *var_twoatatoverthreebtat__blk1571_dn12_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn6: f64 = *var_twoatatoverthreebtat__blk1571_dn6_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn7: f64 = *var_twoatatoverthreebtat__blk1571_dn7_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn8: f64 = *var_twoatatoverthreebtat__blk1571_dn8_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn9: f64 = *var_twoatatoverthreebtat__blk1571_dn9_slot;
        let mut var_twoatatoverthreebtat__blk1571_rv: f64 = *var_twoatatoverthreebtat__blk1571_rv_slot;
        let mut var_umax__blk1573: f64 = *var_umax__blk1573_slot;
        let mut var_umax__blk1573_dn11: f64 = *var_umax__blk1573_dn11_slot;
        let mut var_umax__blk1573_dn12: f64 = *var_umax__blk1573_dn12_slot;
        let mut var_umax__blk1573_dn6: f64 = *var_umax__blk1573_dn6_slot;
        let mut var_umax__blk1573_dn7: f64 = *var_umax__blk1573_dn7_slot;
        let mut var_umax__blk1573_dn8: f64 = *var_umax__blk1573_dn8_slot;
        let mut var_umax__blk1573_dn9: f64 = *var_umax__blk1573_dn9_slot;
        let mut var_umax__blk1573_rv: f64 = *var_umax__blk1573_rv_slot;
        let mut var_umaxbeforelimiting__blk1572: f64 = *var_umaxbeforelimiting__blk1572_slot;
        let mut var_umaxbeforelimiting__blk1572_dn11: f64 = *var_umaxbeforelimiting__blk1572_dn11_slot;
        let mut var_umaxbeforelimiting__blk1572_dn12: f64 = *var_umaxbeforelimiting__blk1572_dn12_slot;
        let mut var_umaxbeforelimiting__blk1572_dn6: f64 = *var_umaxbeforelimiting__blk1572_dn6_slot;
        let mut var_umaxbeforelimiting__blk1572_dn7: f64 = *var_umaxbeforelimiting__blk1572_dn7_slot;
        let mut var_umaxbeforelimiting__blk1572_dn8: f64 = *var_umaxbeforelimiting__blk1572_dn8_slot;
        let mut var_umaxbeforelimiting__blk1572_dn9: f64 = *var_umaxbeforelimiting__blk1572_dn9_slot;
        let mut var_umaxbeforelimiting__blk1572_rv: f64 = *var_umaxbeforelimiting__blk1572_rv_slot;
        let mut var_umaxpoweronepointfive__blk1575: f64 = *var_umaxpoweronepointfive__blk1575_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn11: f64 = *var_umaxpoweronepointfive__blk1575_dn11_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn12: f64 = *var_umaxpoweronepointfive__blk1575_dn12_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn6: f64 = *var_umaxpoweronepointfive__blk1575_dn6_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn7: f64 = *var_umaxpoweronepointfive__blk1575_dn7_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn8: f64 = *var_umaxpoweronepointfive__blk1575_dn8_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn9: f64 = *var_umaxpoweronepointfive__blk1575_dn9_slot;
        let mut var_umaxpoweronepointfive__blk1575_rv: f64 = *var_umaxpoweronepointfive__blk1575_rv_slot;
        let mut var_vbi_minus_vjsrh__blk1563: f64 = *var_vbi_minus_vjsrh__blk1563_slot;
        let mut var_vbi_minus_vjsrh__blk1563_dn11: f64 = *var_vbi_minus_vjsrh__blk1563_dn11_slot;
        let mut var_vbi_minus_vjsrh__blk1563_dn12: f64 = *var_vbi_minus_vjsrh__blk1563_dn12_slot;
        let mut var_vbi_minus_vjsrh__blk1563_dn7: f64 = *var_vbi_minus_vjsrh__blk1563_dn7_slot;
        let mut var_vbi_minus_vjsrh__blk1563_dn8: f64 = *var_vbi_minus_vjsrh__blk1563_dn8_slot;
        let mut var_vbi_minus_vjsrh__blk1563_rv: f64 = *var_vbi_minus_vjsrh__blk1563_rv_slot;
        let mut var_wdep__blk1567: f64 = *var_wdep__blk1567_slot;
        let mut var_wdep__blk1567_dn11: f64 = *var_wdep__blk1567_dn11_slot;
        let mut var_wdep__blk1567_dn12: f64 = *var_wdep__blk1567_dn12_slot;
        let mut var_wdep__blk1567_dn6: f64 = *var_wdep__blk1567_dn6_slot;
        let mut var_wdep__blk1567_dn7: f64 = *var_wdep__blk1567_dn7_slot;
        let mut var_wdep__blk1567_dn8: f64 = *var_wdep__blk1567_dn8_slot;
        let mut var_wdep__blk1567_dn9: f64 = *var_wdep__blk1567_dn9_slot;
        let mut var_wdep__blk1567_rv: f64 = *var_wdep__blk1567_rv_slot;

        let (assign60770_e78592, assign60770_e78592_d_n6, assign60770_e78592_d_n7, assign60770_e78592_d_n8, assign60770_e78592_d_n9, assign60770_e78592_d_n11, assign60770_e78592_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1706 == 0.0)) && (var_guard1707 == 0.0)) {
        let assign60770_e78586: f64 = (var_vbirstid_i - var_vbbt__blk1558);
        let assign60770_e78588: f64 = (assign60770_e78586 * var_vbirstiinv_d);
        let assign60770_e78590: f64 = (assign60770_e78588).powf(var_pstid_i);
        (assign60770_e78590, 0.0, if 0.0 == 0.0 && ((var_pstid_i) as f64).is_finite() && ((var_pstid_i) as f64).fract() == 0.0 { if var_pstid_i == 0.0 { 0.0 } else { (var_pstid_i * ((assign60770_e78588).powf(var_pstid_i - 1.0) * ((-var_vbbt__blk1558_dn7) * var_vbirstiinv_d))) } } else { (assign60770_e78590 * (var_pstid_i * (((-var_vbbt__blk1558_dn7) * var_vbirstiinv_d) / assign60770_e78588))) }, if 0.0 == 0.0 && ((var_pstid_i) as f64).is_finite() && ((var_pstid_i) as f64).fract() == 0.0 { if var_pstid_i == 0.0 { 0.0 } else { (var_pstid_i * ((assign60770_e78588).powf(var_pstid_i - 1.0) * ((-var_vbbt__blk1558_dn8) * var_vbirstiinv_d))) } } else { (assign60770_e78590 * (var_pstid_i * (((-var_vbbt__blk1558_dn8) * var_vbirstiinv_d) / assign60770_e78588))) }, 0.0, if 0.0 == 0.0 && ((var_pstid_i) as f64).is_finite() && ((var_pstid_i) as f64).fract() == 0.0 { if var_pstid_i == 0.0 { 0.0 } else { (var_pstid_i * ((assign60770_e78588).powf(var_pstid_i - 1.0) * ((-var_vbbt__blk1558_dn11) * var_vbirstiinv_d))) } } else { (assign60770_e78590 * (var_pstid_i * (((-var_vbbt__blk1558_dn11) * var_vbirstiinv_d) / assign60770_e78588))) }, if 0.0 == 0.0 && ((var_pstid_i) as f64).is_finite() && ((var_pstid_i) as f64).fract() == 0.0 { if var_pstid_i == 0.0 { 0.0 } else { (var_pstid_i * ((assign60770_e78588).powf(var_pstid_i - 1.0) * ((-var_vbbt__blk1558_dn12) * var_vbirstiinv_d))) } } else { (assign60770_e78590 * (var_pstid_i * (((-var_vbbt__blk1558_dn12) * var_vbirstiinv_d) / assign60770_e78588))) },)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60770_e78592;
        var_tmp__blk1560_dn6 = assign60770_e78592_d_n6;
        var_tmp__blk1560_dn7 = assign60770_e78592_d_n7;
        var_tmp__blk1560_dn8 = assign60770_e78592_d_n8;
        var_tmp__blk1560_dn9 = assign60770_e78592_d_n9;
        var_tmp__blk1560_dn11 = assign60770_e78592_d_n11;
        var_tmp__blk1560_dn12 = assign60770_e78592_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let (assign60780_e78613, assign60780_e78613_d_n6, assign60780_e78613_d_n7, assign60780_e78613_d_n8, assign60780_e78613_d_n9, assign60780_e78613_d_n11, assign60780_e78613_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1706 == 0.0)) {
        let assign60780_e78606: f64 = (var_vbirstid_i - var_vbbt__blk1558);
        let assign60780_e78608: f64 = (assign60780_e78606 * var_wdepnulrinvsti_d);
        let assign60780_e78610: f64 = (assign60780_e78608 / var_tmp__blk1560);
        let assign60780_e78611: f64 = (var_one_over_one_minus_psti_d * assign60780_e78610);
        (assign60780_e78611, (var_one_over_one_minus_psti_d * (-((assign60780_e78608 * var_tmp__blk1560_dn6) / (var_tmp__blk1560 * var_tmp__blk1560)))), (var_one_over_one_minus_psti_d * (((((-var_vbbt__blk1558_dn7) * var_wdepnulrinvsti_d) * var_tmp__blk1560) - (assign60780_e78608 * var_tmp__blk1560_dn7)) / (var_tmp__blk1560 * var_tmp__blk1560))), (var_one_over_one_minus_psti_d * (((((-var_vbbt__blk1558_dn8) * var_wdepnulrinvsti_d) * var_tmp__blk1560) - (assign60780_e78608 * var_tmp__blk1560_dn8)) / (var_tmp__blk1560 * var_tmp__blk1560))), (var_one_over_one_minus_psti_d * (-((assign60780_e78608 * var_tmp__blk1560_dn9) / (var_tmp__blk1560 * var_tmp__blk1560)))), (var_one_over_one_minus_psti_d * (((((-var_vbbt__blk1558_dn11) * var_wdepnulrinvsti_d) * var_tmp__blk1560) - (assign60780_e78608 * var_tmp__blk1560_dn11)) / (var_tmp__blk1560 * var_tmp__blk1560))), (var_one_over_one_minus_psti_d * (((((-var_vbbt__blk1558_dn12) * var_wdepnulrinvsti_d) * var_tmp__blk1560) - (assign60780_e78608 * var_tmp__blk1560_dn12)) / (var_tmp__blk1560 * var_tmp__blk1560))),)
    } else {
        (var_fmaxr__blk1585, var_fmaxr__blk1585_dn6, var_fmaxr__blk1585_dn7, var_fmaxr__blk1585_dn8, var_fmaxr__blk1585_dn9, var_fmaxr__blk1585_dn11, var_fmaxr__blk1585_dn12,)
    }
};
        var_fmaxr__blk1585 = assign60780_e78613;
        var_fmaxr__blk1585_dn6 = assign60780_e78613_d_n6;
        var_fmaxr__blk1585_dn7 = assign60780_e78613_d_n7;
        var_fmaxr__blk1585_dn8 = assign60780_e78613_d_n8;
        var_fmaxr__blk1585_dn9 = assign60780_e78613_d_n9;
        var_fmaxr__blk1585_dn11 = assign60780_e78613_d_n11;
        var_fmaxr__blk1585_dn12 = assign60780_e78613_d_n12;
        var_fmaxr__blk1585_rv = 0.0;

        let assign60790_e78615: f64 = (-var_fbbtsti_d);
        let assign60790_e78617: f64 = (assign60790_e78615 / var_fmaxr__blk1585);
        let assign60790_e78618: f64 = (assign60790_e78617).abs();
        let assign60790_e78620: f64 = if assign60790_e78618 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1708 = assign60790_e78620;
        var_guard1708_rv = 0.0;

        let (assign60800_e78639, assign60800_e78639_d_n6, assign60800_e78639_d_n7, assign60800_e78639_d_n8, assign60800_e78639_d_n9, assign60800_e78639_d_n11, assign60800_e78639_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1706 == 0.0)) && (var_guard1708 != 0.0)) {
        let assign60800_e78634: f64 = (-var_fbbtsti_d);
        let assign60800_e78636: f64 = (assign60800_e78634 / var_fmaxr__blk1585);
        let assign60800_e78637: f64 = (assign60800_e78636).exp();
        (assign60800_e78637, (assign60800_e78637 * (-((assign60800_e78634 * var_fmaxr__blk1585_dn6) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))), (assign60800_e78637 * (-((assign60800_e78634 * var_fmaxr__blk1585_dn7) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))), (assign60800_e78637 * (-((assign60800_e78634 * var_fmaxr__blk1585_dn8) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))), (assign60800_e78637 * (-((assign60800_e78634 * var_fmaxr__blk1585_dn9) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))), (assign60800_e78637 * (-((assign60800_e78634 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))), (assign60800_e78637 * (-((assign60800_e78634 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60800_e78639;
        var_tmp__blk1560_dn6 = assign60800_e78639_d_n6;
        var_tmp__blk1560_dn7 = assign60800_e78639_d_n7;
        var_tmp__blk1560_dn8 = assign60800_e78639_d_n8;
        var_tmp__blk1560_dn9 = assign60800_e78639_d_n9;
        var_tmp__blk1560_dn11 = assign60800_e78639_d_n11;
        var_tmp__blk1560_dn12 = assign60800_e78639_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let assign60810_e78641: f64 = (-var_fbbtsti_d);
        let assign60810_e78643: f64 = (assign60810_e78641 / var_fmaxr__blk1585);
        let assign60810_e78645: f64 = if assign60810_e78643 < 0.0 { 1.0 } else { 0.0 };
        var_guard1709 = assign60810_e78645;
        var_guard1709_rv = 0.0;

        let (assign60820_e78697, assign60820_e78697_d_n6, assign60820_e78697_d_n7, assign60820_e78697_d_n8, assign60820_e78697_d_n9, assign60820_e78697_d_n11, assign60820_e78697_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1706 == 0.0)) && (var_guard1708 == 0.0)) && (var_guard1709 != 0.0)) {
        let assign60820_e78664: f64 = (-230.25850929940458);
        let assign60820_e78666: f64 = (-var_fbbtsti_d);
        let assign60820_e78668: f64 = (assign60820_e78666 / var_fmaxr__blk1585);
        let assign60820_e78669: f64 = (assign60820_e78664 - assign60820_e78668);
        let assign60820_e78673: f64 = (-230.25850929940458);
        let assign60820_e78675: f64 = (-var_fbbtsti_d);
        let assign60820_e78677: f64 = (assign60820_e78675 / var_fmaxr__blk1585);
        let assign60820_e78678: f64 = (assign60820_e78673 - assign60820_e78677);
        let assign60820_e78681: f64 = (-230.25850929940458);
        let assign60820_e78683: f64 = (-var_fbbtsti_d);
        let assign60820_e78685: f64 = (assign60820_e78683 / var_fmaxr__blk1585);
        let assign60820_e78686: f64 = (assign60820_e78681 - assign60820_e78685);
        let assign60820_e78688: f64 = (assign60820_e78686 * 0.3333333333333333);
        let assign60820_e78689: f64 = (1.0 + assign60820_e78688);
        let assign60820_e78690: f64 = (assign60820_e78678 * assign60820_e78689);
        let assign60820_e78691: f64 = (0.5 * assign60820_e78690);
        let assign60820_e78692: f64 = (1.0 + assign60820_e78691);
        let assign60820_e78693: f64 = (assign60820_e78669 * assign60820_e78692);
        let assign60820_e78694: f64 = (1.0 + assign60820_e78693);
        let assign60820_e78695: f64 = (1e-100 / assign60820_e78694);
        (assign60820_e78695, (-((1e-100 * (((-(-((assign60820_e78666 * var_fmaxr__blk1585_dn6) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60820_e78692) + (assign60820_e78669 * (0.5 * (((-(-((assign60820_e78675 * var_fmaxr__blk1585_dn6) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60820_e78689) + (assign60820_e78678 * ((-(-((assign60820_e78683 * var_fmaxr__blk1585_dn6) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * 0.3333333333333333))))))) / (assign60820_e78694 * assign60820_e78694))), (-((1e-100 * (((-(-((assign60820_e78666 * var_fmaxr__blk1585_dn7) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60820_e78692) + (assign60820_e78669 * (0.5 * (((-(-((assign60820_e78675 * var_fmaxr__blk1585_dn7) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60820_e78689) + (assign60820_e78678 * ((-(-((assign60820_e78683 * var_fmaxr__blk1585_dn7) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * 0.3333333333333333))))))) / (assign60820_e78694 * assign60820_e78694))), (-((1e-100 * (((-(-((assign60820_e78666 * var_fmaxr__blk1585_dn8) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60820_e78692) + (assign60820_e78669 * (0.5 * (((-(-((assign60820_e78675 * var_fmaxr__blk1585_dn8) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60820_e78689) + (assign60820_e78678 * ((-(-((assign60820_e78683 * var_fmaxr__blk1585_dn8) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * 0.3333333333333333))))))) / (assign60820_e78694 * assign60820_e78694))), (-((1e-100 * (((-(-((assign60820_e78666 * var_fmaxr__blk1585_dn9) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60820_e78692) + (assign60820_e78669 * (0.5 * (((-(-((assign60820_e78675 * var_fmaxr__blk1585_dn9) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60820_e78689) + (assign60820_e78678 * ((-(-((assign60820_e78683 * var_fmaxr__blk1585_dn9) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * 0.3333333333333333))))))) / (assign60820_e78694 * assign60820_e78694))), (-((1e-100 * (((-(-((assign60820_e78666 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60820_e78692) + (assign60820_e78669 * (0.5 * (((-(-((assign60820_e78675 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60820_e78689) + (assign60820_e78678 * ((-(-((assign60820_e78683 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * 0.3333333333333333))))))) / (assign60820_e78694 * assign60820_e78694))), (-((1e-100 * (((-(-((assign60820_e78666 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60820_e78692) + (assign60820_e78669 * (0.5 * (((-(-((assign60820_e78675 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign60820_e78689) + (assign60820_e78678 * ((-(-((assign60820_e78683 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * 0.3333333333333333))))))) / (assign60820_e78694 * assign60820_e78694))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60820_e78697;
        var_tmp__blk1560_dn6 = assign60820_e78697_d_n6;
        var_tmp__blk1560_dn7 = assign60820_e78697_d_n7;
        var_tmp__blk1560_dn8 = assign60820_e78697_d_n8;
        var_tmp__blk1560_dn9 = assign60820_e78697_d_n9;
        var_tmp__blk1560_dn11 = assign60820_e78697_d_n11;
        var_tmp__blk1560_dn12 = assign60820_e78697_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let (assign60830_e78747, assign60830_e78747_d_n6, assign60830_e78747_d_n7, assign60830_e78747_d_n8, assign60830_e78747_d_n9, assign60830_e78747_d_n11, assign60830_e78747_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1706 == 0.0)) && (var_guard1708 == 0.0)) && (var_guard1709 == 0.0)) {
        let assign60830_e78717: f64 = (-var_fbbtsti_d);
        let assign60830_e78719: f64 = (assign60830_e78717 / var_fmaxr__blk1585);
        let assign60830_e78721: f64 = (assign60830_e78719 - 230.25850929940458);
        let assign60830_e78725: f64 = (-var_fbbtsti_d);
        let assign60830_e78727: f64 = (assign60830_e78725 / var_fmaxr__blk1585);
        let assign60830_e78729: f64 = (assign60830_e78727 - 230.25850929940458);
        let assign60830_e78732: f64 = (-var_fbbtsti_d);
        let assign60830_e78734: f64 = (assign60830_e78732 / var_fmaxr__blk1585);
        let assign60830_e78736: f64 = (assign60830_e78734 - 230.25850929940458);
        let assign60830_e78738: f64 = (assign60830_e78736 * 0.3333333333333333);
        let assign60830_e78739: f64 = (1.0 + assign60830_e78738);
        let assign60830_e78740: f64 = (assign60830_e78729 * assign60830_e78739);
        let assign60830_e78741: f64 = (0.5 * assign60830_e78740);
        let assign60830_e78742: f64 = (1.0 + assign60830_e78741);
        let assign60830_e78743: f64 = (assign60830_e78721 * assign60830_e78742);
        let assign60830_e78744: f64 = (1.0 + assign60830_e78743);
        let assign60830_e78745: f64 = (1e100 * assign60830_e78744);
        (assign60830_e78745, (1e100 * (((-((assign60830_e78717 * var_fmaxr__blk1585_dn6) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60830_e78742) + (assign60830_e78721 * (0.5 * (((-((assign60830_e78725 * var_fmaxr__blk1585_dn6) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60830_e78739) + (assign60830_e78729 * ((-((assign60830_e78732 * var_fmaxr__blk1585_dn6) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))), (1e100 * (((-((assign60830_e78717 * var_fmaxr__blk1585_dn7) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60830_e78742) + (assign60830_e78721 * (0.5 * (((-((assign60830_e78725 * var_fmaxr__blk1585_dn7) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60830_e78739) + (assign60830_e78729 * ((-((assign60830_e78732 * var_fmaxr__blk1585_dn7) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))), (1e100 * (((-((assign60830_e78717 * var_fmaxr__blk1585_dn8) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60830_e78742) + (assign60830_e78721 * (0.5 * (((-((assign60830_e78725 * var_fmaxr__blk1585_dn8) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60830_e78739) + (assign60830_e78729 * ((-((assign60830_e78732 * var_fmaxr__blk1585_dn8) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))), (1e100 * (((-((assign60830_e78717 * var_fmaxr__blk1585_dn9) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60830_e78742) + (assign60830_e78721 * (0.5 * (((-((assign60830_e78725 * var_fmaxr__blk1585_dn9) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60830_e78739) + (assign60830_e78729 * ((-((assign60830_e78732 * var_fmaxr__blk1585_dn9) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))), (1e100 * (((-((assign60830_e78717 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60830_e78742) + (assign60830_e78721 * (0.5 * (((-((assign60830_e78725 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60830_e78739) + (assign60830_e78729 * ((-((assign60830_e78732 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))), (1e100 * (((-((assign60830_e78717 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60830_e78742) + (assign60830_e78721 * (0.5 * (((-((assign60830_e78725 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign60830_e78739) + (assign60830_e78729 * ((-((assign60830_e78732 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60830_e78747;
        var_tmp__blk1560_dn6 = assign60830_e78747_d_n6;
        var_tmp__blk1560_dn7 = assign60830_e78747_d_n7;
        var_tmp__blk1560_dn8 = assign60830_e78747_d_n8;
        var_tmp__blk1560_dn9 = assign60830_e78747_d_n9;
        var_tmp__blk1560_dn11 = assign60830_e78747_d_n11;
        var_tmp__blk1560_dn12 = assign60830_e78747_d_n12;
        var_tmp__blk1560_rv = 0.0;

        let assign60850_e78771: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard1710 = assign60850_e78771;
        var_guard1710_rv = 0.0;

        let assign60870_e78786: f64 = (-var_alphaav);
        let assign60870_e78788: f64 = (assign60870_e78786 * var_vbrstid_i);
        let assign60870_e78789: f64 = if var_vav__blk1559 > assign60870_e78788 { 1.0 } else { 0.0 };
        var_guard1711 = assign60870_e78789;
        var_guard1711_rv = 0.0;

        let assign60880_e78792: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard1712 = assign60880_e78792;
        var_guard1712_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

        let assign60940_e78912: f64 = if var_one_minus_psti_d == 0.5 { 1.0 } else { 0.0 };
        var_guard1713 = assign60940_e78912;
        var_guard1713_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_qjunsti_d_rv = 0.0;

        let assign60980_e78973: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard1714 = assign60980_e78973;
        var_guard1714_rv = 0.0;

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
        var_qjungat_d_rv = 0.0;

        let assign61020_e79010: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard1715 = assign61020_e79010;
        var_guard1715_rv = 0.0;

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
        var_vbi_minus_vjsrh__blk1563_rv = 0.0;

        let assign61100_e79126: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard1717 = assign61100_e79126;
        var_guard1717_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_wdep__blk1567_rv = 0.0;

        let assign61160_e79218: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard1718 = assign61160_e79218;
        var_guard1718_rv = 0.0;

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
        var_btat__blk1570_rv = 0.0;

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
        var_twoatatoverthreebtat__blk1571_rv = 0.0;

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
        var_umaxbeforelimiting__blk1572_rv = 0.0;

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
        var_umax__blk1573_rv = 0.0;

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
        var_sqrtumax__blk1574_rv = 0.0;

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
        var_umaxpoweronepointfive__blk1575_rv = 0.0;

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
        var_ktat__blk1578_rv = 0.0;

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
        var_ltat__blk1579_rv = 0.0;

        *var_btat__blk1570_slot = var_btat__blk1570;
        *var_btat__blk1570_dn11_slot = var_btat__blk1570_dn11;
        *var_btat__blk1570_dn12_slot = var_btat__blk1570_dn12;
        *var_btat__blk1570_dn6_slot = var_btat__blk1570_dn6;
        *var_btat__blk1570_dn7_slot = var_btat__blk1570_dn7;
        *var_btat__blk1570_dn8_slot = var_btat__blk1570_dn8;
        *var_btat__blk1570_dn9_slot = var_btat__blk1570_dn9;
        *var_btat__blk1570_rv_slot = var_btat__blk1570_rv;
        *var_fmaxr__blk1585_slot = var_fmaxr__blk1585;
        *var_fmaxr__blk1585_dn11_slot = var_fmaxr__blk1585_dn11;
        *var_fmaxr__blk1585_dn12_slot = var_fmaxr__blk1585_dn12;
        *var_fmaxr__blk1585_dn6_slot = var_fmaxr__blk1585_dn6;
        *var_fmaxr__blk1585_dn7_slot = var_fmaxr__blk1585_dn7;
        *var_fmaxr__blk1585_dn8_slot = var_fmaxr__blk1585_dn8;
        *var_fmaxr__blk1585_dn9_slot = var_fmaxr__blk1585_dn9;
        *var_fmaxr__blk1585_rv_slot = var_fmaxr__blk1585_rv;
        *var_guard1708_slot = var_guard1708;
        *var_guard1708_rv_slot = var_guard1708_rv;
        *var_guard1709_slot = var_guard1709;
        *var_guard1709_rv_slot = var_guard1709_rv;
        *var_guard1710_slot = var_guard1710;
        *var_guard1710_rv_slot = var_guard1710_rv;
        *var_guard1711_slot = var_guard1711;
        *var_guard1711_rv_slot = var_guard1711_rv;
        *var_guard1712_slot = var_guard1712;
        *var_guard1712_rv_slot = var_guard1712_rv;
        *var_guard1713_slot = var_guard1713;
        *var_guard1713_rv_slot = var_guard1713_rv;
        *var_guard1714_slot = var_guard1714;
        *var_guard1714_rv_slot = var_guard1714_rv;
        *var_guard1715_slot = var_guard1715;
        *var_guard1715_rv_slot = var_guard1715_rv;
        *var_guard1717_slot = var_guard1717;
        *var_guard1717_rv_slot = var_guard1717_rv;
        *var_guard1718_slot = var_guard1718;
        *var_guard1718_rv_slot = var_guard1718_rv;
        *var_ktat__blk1578_slot = var_ktat__blk1578;
        *var_ktat__blk1578_dn11_slot = var_ktat__blk1578_dn11;
        *var_ktat__blk1578_dn12_slot = var_ktat__blk1578_dn12;
        *var_ktat__blk1578_dn6_slot = var_ktat__blk1578_dn6;
        *var_ktat__blk1578_dn7_slot = var_ktat__blk1578_dn7;
        *var_ktat__blk1578_dn8_slot = var_ktat__blk1578_dn8;
        *var_ktat__blk1578_dn9_slot = var_ktat__blk1578_dn9;
        *var_ktat__blk1578_rv_slot = var_ktat__blk1578_rv;
        *var_ltat__blk1579_slot = var_ltat__blk1579;
        *var_ltat__blk1579_dn11_slot = var_ltat__blk1579_dn11;
        *var_ltat__blk1579_dn12_slot = var_ltat__blk1579_dn12;
        *var_ltat__blk1579_dn6_slot = var_ltat__blk1579_dn6;
        *var_ltat__blk1579_dn7_slot = var_ltat__blk1579_dn7;
        *var_ltat__blk1579_dn8_slot = var_ltat__blk1579_dn8;
        *var_ltat__blk1579_dn9_slot = var_ltat__blk1579_dn9;
        *var_ltat__blk1579_rv_slot = var_ltat__blk1579_rv;
        *var_qjungat_d_slot = var_qjungat_d;
        *var_qjungat_d_dn11_slot = var_qjungat_d_dn11;
        *var_qjungat_d_dn12_slot = var_qjungat_d_dn12;
        *var_qjungat_d_dn6_slot = var_qjungat_d_dn6;
        *var_qjungat_d_dn7_slot = var_qjungat_d_dn7;
        *var_qjungat_d_dn8_slot = var_qjungat_d_dn8;
        *var_qjungat_d_dn9_slot = var_qjungat_d_dn9;
        *var_qjungat_d_rv_slot = var_qjungat_d_rv;
        *var_qjunsti_d_slot = var_qjunsti_d;
        *var_qjunsti_d_dn11_slot = var_qjunsti_d_dn11;
        *var_qjunsti_d_dn12_slot = var_qjunsti_d_dn12;
        *var_qjunsti_d_dn6_slot = var_qjunsti_d_dn6;
        *var_qjunsti_d_dn7_slot = var_qjunsti_d_dn7;
        *var_qjunsti_d_dn8_slot = var_qjunsti_d_dn8;
        *var_qjunsti_d_dn9_slot = var_qjunsti_d_dn9;
        *var_qjunsti_d_rv_slot = var_qjunsti_d_rv;
        *var_sqrtumax__blk1574_slot = var_sqrtumax__blk1574;
        *var_sqrtumax__blk1574_dn11_slot = var_sqrtumax__blk1574_dn11;
        *var_sqrtumax__blk1574_dn12_slot = var_sqrtumax__blk1574_dn12;
        *var_sqrtumax__blk1574_dn6_slot = var_sqrtumax__blk1574_dn6;
        *var_sqrtumax__blk1574_dn7_slot = var_sqrtumax__blk1574_dn7;
        *var_sqrtumax__blk1574_dn8_slot = var_sqrtumax__blk1574_dn8;
        *var_sqrtumax__blk1574_dn9_slot = var_sqrtumax__blk1574_dn9;
        *var_sqrtumax__blk1574_rv_slot = var_sqrtumax__blk1574_rv;
        *var_tmp__blk1560_slot = var_tmp__blk1560;
        *var_tmp__blk1560_dn11_slot = var_tmp__blk1560_dn11;
        *var_tmp__blk1560_dn12_slot = var_tmp__blk1560_dn12;
        *var_tmp__blk1560_dn6_slot = var_tmp__blk1560_dn6;
        *var_tmp__blk1560_dn7_slot = var_tmp__blk1560_dn7;
        *var_tmp__blk1560_dn8_slot = var_tmp__blk1560_dn8;
        *var_tmp__blk1560_dn9_slot = var_tmp__blk1560_dn9;
        *var_tmp__blk1560_rv_slot = var_tmp__blk1560_rv;
        *var_twoatatoverthreebtat__blk1571_slot = var_twoatatoverthreebtat__blk1571;
        *var_twoatatoverthreebtat__blk1571_dn11_slot = var_twoatatoverthreebtat__blk1571_dn11;
        *var_twoatatoverthreebtat__blk1571_dn12_slot = var_twoatatoverthreebtat__blk1571_dn12;
        *var_twoatatoverthreebtat__blk1571_dn6_slot = var_twoatatoverthreebtat__blk1571_dn6;
        *var_twoatatoverthreebtat__blk1571_dn7_slot = var_twoatatoverthreebtat__blk1571_dn7;
        *var_twoatatoverthreebtat__blk1571_dn8_slot = var_twoatatoverthreebtat__blk1571_dn8;
        *var_twoatatoverthreebtat__blk1571_dn9_slot = var_twoatatoverthreebtat__blk1571_dn9;
        *var_twoatatoverthreebtat__blk1571_rv_slot = var_twoatatoverthreebtat__blk1571_rv;
        *var_umax__blk1573_slot = var_umax__blk1573;
        *var_umax__blk1573_dn11_slot = var_umax__blk1573_dn11;
        *var_umax__blk1573_dn12_slot = var_umax__blk1573_dn12;
        *var_umax__blk1573_dn6_slot = var_umax__blk1573_dn6;
        *var_umax__blk1573_dn7_slot = var_umax__blk1573_dn7;
        *var_umax__blk1573_dn8_slot = var_umax__blk1573_dn8;
        *var_umax__blk1573_dn9_slot = var_umax__blk1573_dn9;
        *var_umax__blk1573_rv_slot = var_umax__blk1573_rv;
        *var_umaxbeforelimiting__blk1572_slot = var_umaxbeforelimiting__blk1572;
        *var_umaxbeforelimiting__blk1572_dn11_slot = var_umaxbeforelimiting__blk1572_dn11;
        *var_umaxbeforelimiting__blk1572_dn12_slot = var_umaxbeforelimiting__blk1572_dn12;
        *var_umaxbeforelimiting__blk1572_dn6_slot = var_umaxbeforelimiting__blk1572_dn6;
        *var_umaxbeforelimiting__blk1572_dn7_slot = var_umaxbeforelimiting__blk1572_dn7;
        *var_umaxbeforelimiting__blk1572_dn8_slot = var_umaxbeforelimiting__blk1572_dn8;
        *var_umaxbeforelimiting__blk1572_dn9_slot = var_umaxbeforelimiting__blk1572_dn9;
        *var_umaxbeforelimiting__blk1572_rv_slot = var_umaxbeforelimiting__blk1572_rv;
        *var_umaxpoweronepointfive__blk1575_slot = var_umaxpoweronepointfive__blk1575;
        *var_umaxpoweronepointfive__blk1575_dn11_slot = var_umaxpoweronepointfive__blk1575_dn11;
        *var_umaxpoweronepointfive__blk1575_dn12_slot = var_umaxpoweronepointfive__blk1575_dn12;
        *var_umaxpoweronepointfive__blk1575_dn6_slot = var_umaxpoweronepointfive__blk1575_dn6;
        *var_umaxpoweronepointfive__blk1575_dn7_slot = var_umaxpoweronepointfive__blk1575_dn7;
        *var_umaxpoweronepointfive__blk1575_dn8_slot = var_umaxpoweronepointfive__blk1575_dn8;
        *var_umaxpoweronepointfive__blk1575_dn9_slot = var_umaxpoweronepointfive__blk1575_dn9;
        *var_umaxpoweronepointfive__blk1575_rv_slot = var_umaxpoweronepointfive__blk1575_rv;
        *var_vbi_minus_vjsrh__blk1563_slot = var_vbi_minus_vjsrh__blk1563;
        *var_vbi_minus_vjsrh__blk1563_dn11_slot = var_vbi_minus_vjsrh__blk1563_dn11;
        *var_vbi_minus_vjsrh__blk1563_dn12_slot = var_vbi_minus_vjsrh__blk1563_dn12;
        *var_vbi_minus_vjsrh__blk1563_dn7_slot = var_vbi_minus_vjsrh__blk1563_dn7;
        *var_vbi_minus_vjsrh__blk1563_dn8_slot = var_vbi_minus_vjsrh__blk1563_dn8;
        *var_vbi_minus_vjsrh__blk1563_rv_slot = var_vbi_minus_vjsrh__blk1563_rv;
        *var_wdep__blk1567_slot = var_wdep__blk1567;
        *var_wdep__blk1567_dn11_slot = var_wdep__blk1567_dn11;
        *var_wdep__blk1567_dn12_slot = var_wdep__blk1567_dn12;
        *var_wdep__blk1567_dn6_slot = var_wdep__blk1567_dn6;
        *var_wdep__blk1567_dn7_slot = var_wdep__blk1567_dn7;
        *var_wdep__blk1567_dn8_slot = var_wdep__blk1567_dn8;
        *var_wdep__blk1567_dn9_slot = var_wdep__blk1567_dn9;
        *var_wdep__blk1567_rv_slot = var_wdep__blk1567_rv;
    }

    pub(super) fn stamp_reactive_block_77(
        var_alphaav: f64,
        var_anugatd_i: f64,
        var_atatgat_d: f64,
        var_btat__blk1570: f64,
        var_btat__blk1570_dn11: f64,
        var_btat__blk1570_dn12: f64,
        var_btat__blk1570_dn6: f64,
        var_btat__blk1570_dn7: f64,
        var_btat__blk1570_dn8: f64,
        var_btat__blk1570_dn9: f64,
        var_cbbtgatd_i: f64,
        var_fbbtgat_d: f64,
        var_fbbtgat_d_dn6: f64,
        var_fbbtgat_d_dn7: f64,
        var_fbbtgat_d_dn8: f64,
        var_fbbtgat_d_dn9: f64,
        var_guard1589: f64,
        var_guard1590: f64,
        var_guard1714: f64,
        var_guard1718: f64,
        var_ktat__blk1578: f64,
        var_ktat__blk1578_dn11: f64,
        var_ktat__blk1578_dn12: f64,
        var_ktat__blk1578_dn6: f64,
        var_ktat__blk1578_dn7: f64,
        var_ktat__blk1578_dn8: f64,
        var_ktat__blk1578_dn9: f64,
        var_ltat__blk1579: f64,
        var_ltat__blk1579_dn11: f64,
        var_ltat__blk1579_dn12: f64,
        var_ltat__blk1579_dn6: f64,
        var_ltat__blk1579_dn7: f64,
        var_ltat__blk1579_dn8: f64,
        var_ltat__blk1579_dn9: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_pbrgatd_i: f64,
        var_pgatd_i: f64,
        var_sqrtumax__blk1574: f64,
        var_sqrtumax__blk1574_dn11: f64,
        var_sqrtumax__blk1574_dn12: f64,
        var_sqrtumax__blk1574_dn6: f64,
        var_sqrtumax__blk1574_dn7: f64,
        var_sqrtumax__blk1574_dn8: f64,
        var_sqrtumax__blk1574_dn9: f64,
        var_swgat2nd_d: f64,
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
        var_umaxpoweronepointfive__blk1575: f64,
        var_umaxpoweronepointfive__blk1575_dn11: f64,
        var_umaxpoweronepointfive__blk1575_dn12: f64,
        var_umaxpoweronepointfive__blk1575_dn6: f64,
        var_umaxpoweronepointfive__blk1575_dn7: f64,
        var_umaxpoweronepointfive__blk1575_dn8: f64,
        var_umaxpoweronepointfive__blk1575_dn9: f64,
        var_vav__blk1559: f64,
        var_vav__blk1559_dn11: f64,
        var_vav__blk1559_dn12: f64,
        var_vav__blk1559_dn7: f64,
        var_vav__blk1559_dn8: f64,
        var_vbbt__blk1558: f64,
        var_vbbt__blk1558_dn11: f64,
        var_vbbt__blk1558_dn12: f64,
        var_vbbt__blk1558_dn7: f64,
        var_vbbt__blk1558_dn8: f64,
        var_vbirgatd_i: f64,
        var_vbirgatinv_d: f64,
        var_vbrgat_var_d: f64,
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
        var_wdepnulrinvgat_d: f64,
        var_fmaxr__blk1585_slot: &mut f64,
        var_fmaxr__blk1585_dn11_slot: &mut f64,
        var_fmaxr__blk1585_dn12_slot: &mut f64,
        var_fmaxr__blk1585_dn6_slot: &mut f64,
        var_fmaxr__blk1585_dn7_slot: &mut f64,
        var_fmaxr__blk1585_dn8_slot: &mut f64,
        var_fmaxr__blk1585_dn9_slot: &mut f64,
        var_fmaxr__blk1585_rv_slot: &mut f64,
        var_guard1721_slot: &mut f64,
        var_guard1721_rv_slot: &mut f64,
        var_guard1722_slot: &mut f64,
        var_guard1722_rv_slot: &mut f64,
        var_guard1723_slot: &mut f64,
        var_guard1723_rv_slot: &mut f64,
        var_guard1724_slot: &mut f64,
        var_guard1724_rv_slot: &mut f64,
        var_guard1725_slot: &mut f64,
        var_guard1725_rv_slot: &mut f64,
        var_guard1726_slot: &mut f64,
        var_guard1726_rv_slot: &mut f64,
        var_guard1727_slot: &mut f64,
        var_guard1727_rv_slot: &mut f64,
        var_guard1728_slot: &mut f64,
        var_guard1728_rv_slot: &mut f64,
        var_guard1729_slot: &mut f64,
        var_guard1729_rv_slot: &mut f64,
        var_guard1730_slot: &mut f64,
        var_guard1730_rv_slot: &mut f64,
        var_guard1731_slot: &mut f64,
        var_guard1731_rv_slot: &mut f64,
        var_h1__blk1545_slot: &mut f64,
        var_h1__blk1545_rv_slot: &mut f64,
        var_h2__blk1546_slot: &mut f64,
        var_h2__blk1546_rv_slot: &mut f64,
        var_h2d__blk1547_slot: &mut f64,
        var_h2d__blk1547_dn11_slot: &mut f64,
        var_h2d__blk1547_dn12_slot: &mut f64,
        var_h2d__blk1547_dn7_slot: &mut f64,
        var_h2d__blk1547_dn8_slot: &mut f64,
        var_h2d__blk1547_rv_slot: &mut f64,
        var_h3__blk1548_slot: &mut f64,
        var_h3__blk1548_dn11_slot: &mut f64,
        var_h3__blk1548_dn12_slot: &mut f64,
        var_h3__blk1548_dn7_slot: &mut f64,
        var_h3__blk1548_dn8_slot: &mut f64,
        var_h3__blk1548_rv_slot: &mut f64,
        var_mtat__blk1580_slot: &mut f64,
        var_mtat__blk1580_dn11_slot: &mut f64,
        var_mtat__blk1580_dn12_slot: &mut f64,
        var_mtat__blk1580_dn6_slot: &mut f64,
        var_mtat__blk1580_dn7_slot: &mut f64,
        var_mtat__blk1580_dn8_slot: &mut f64,
        var_mtat__blk1580_dn9_slot: &mut f64,
        var_mtat__blk1580_rv_slot: &mut f64,
        var_nu__blk1587_slot: &mut f64,
        var_nu__blk1587_dn11_slot: &mut f64,
        var_nu__blk1587_dn12_slot: &mut f64,
        var_nu__blk1587_dn7_slot: &mut f64,
        var_nu__blk1587_dn8_slot: &mut f64,
        var_nu__blk1587_rv_slot: &mut f64,
        var_tmp__blk1560_slot: &mut f64,
        var_tmp__blk1560_dn11_slot: &mut f64,
        var_tmp__blk1560_dn12_slot: &mut f64,
        var_tmp__blk1560_dn6_slot: &mut f64,
        var_tmp__blk1560_dn7_slot: &mut f64,
        var_tmp__blk1560_dn8_slot: &mut f64,
        var_tmp__blk1560_dn9_slot: &mut f64,
        var_tmp__blk1560_rv_slot: &mut f64,
        var_xerfc__blk1581_slot: &mut f64,
        var_xerfc__blk1581_dn11_slot: &mut f64,
        var_xerfc__blk1581_dn12_slot: &mut f64,
        var_xerfc__blk1581_dn6_slot: &mut f64,
        var_xerfc__blk1581_dn7_slot: &mut f64,
        var_xerfc__blk1581_dn8_slot: &mut f64,
        var_xerfc__blk1581_dn9_slot: &mut f64,
        var_xerfc__blk1581_rv_slot: &mut f64,
        var_ysq__blk1542_slot: &mut f64,
        var_ysq__blk1542_dn11_slot: &mut f64,
        var_ysq__blk1542_dn12_slot: &mut f64,
        var_ysq__blk1542_dn6_slot: &mut f64,
        var_ysq__blk1542_dn7_slot: &mut f64,
        var_ysq__blk1542_dn8_slot: &mut f64,
        var_ysq__blk1542_dn9_slot: &mut f64,
        var_ysq__blk1542_rv_slot: &mut f64,
    ) {
        let mut var_fmaxr__blk1585: f64 = *var_fmaxr__blk1585_slot;
        let mut var_fmaxr__blk1585_dn11: f64 = *var_fmaxr__blk1585_dn11_slot;
        let mut var_fmaxr__blk1585_dn12: f64 = *var_fmaxr__blk1585_dn12_slot;
        let mut var_fmaxr__blk1585_dn6: f64 = *var_fmaxr__blk1585_dn6_slot;
        let mut var_fmaxr__blk1585_dn7: f64 = *var_fmaxr__blk1585_dn7_slot;
        let mut var_fmaxr__blk1585_dn8: f64 = *var_fmaxr__blk1585_dn8_slot;
        let mut var_fmaxr__blk1585_dn9: f64 = *var_fmaxr__blk1585_dn9_slot;
        let mut var_fmaxr__blk1585_rv: f64 = *var_fmaxr__blk1585_rv_slot;
        let mut var_guard1721: f64 = *var_guard1721_slot;
        let mut var_guard1721_rv: f64 = *var_guard1721_rv_slot;
        let mut var_guard1722: f64 = *var_guard1722_slot;
        let mut var_guard1722_rv: f64 = *var_guard1722_rv_slot;
        let mut var_guard1723: f64 = *var_guard1723_slot;
        let mut var_guard1723_rv: f64 = *var_guard1723_rv_slot;
        let mut var_guard1724: f64 = *var_guard1724_slot;
        let mut var_guard1724_rv: f64 = *var_guard1724_rv_slot;
        let mut var_guard1725: f64 = *var_guard1725_slot;
        let mut var_guard1725_rv: f64 = *var_guard1725_rv_slot;
        let mut var_guard1726: f64 = *var_guard1726_slot;
        let mut var_guard1726_rv: f64 = *var_guard1726_rv_slot;
        let mut var_guard1727: f64 = *var_guard1727_slot;
        let mut var_guard1727_rv: f64 = *var_guard1727_rv_slot;
        let mut var_guard1728: f64 = *var_guard1728_slot;
        let mut var_guard1728_rv: f64 = *var_guard1728_rv_slot;
        let mut var_guard1729: f64 = *var_guard1729_slot;
        let mut var_guard1729_rv: f64 = *var_guard1729_rv_slot;
        let mut var_guard1730: f64 = *var_guard1730_slot;
        let mut var_guard1730_rv: f64 = *var_guard1730_rv_slot;
        let mut var_guard1731: f64 = *var_guard1731_slot;
        let mut var_guard1731_rv: f64 = *var_guard1731_rv_slot;
        let mut var_h1__blk1545: f64 = *var_h1__blk1545_slot;
        let mut var_h1__blk1545_rv: f64 = *var_h1__blk1545_rv_slot;
        let mut var_h2__blk1546: f64 = *var_h2__blk1546_slot;
        let mut var_h2__blk1546_rv: f64 = *var_h2__blk1546_rv_slot;
        let mut var_h2d__blk1547: f64 = *var_h2d__blk1547_slot;
        let mut var_h2d__blk1547_dn11: f64 = *var_h2d__blk1547_dn11_slot;
        let mut var_h2d__blk1547_dn12: f64 = *var_h2d__blk1547_dn12_slot;
        let mut var_h2d__blk1547_dn7: f64 = *var_h2d__blk1547_dn7_slot;
        let mut var_h2d__blk1547_dn8: f64 = *var_h2d__blk1547_dn8_slot;
        let mut var_h2d__blk1547_rv: f64 = *var_h2d__blk1547_rv_slot;
        let mut var_h3__blk1548: f64 = *var_h3__blk1548_slot;
        let mut var_h3__blk1548_dn11: f64 = *var_h3__blk1548_dn11_slot;
        let mut var_h3__blk1548_dn12: f64 = *var_h3__blk1548_dn12_slot;
        let mut var_h3__blk1548_dn7: f64 = *var_h3__blk1548_dn7_slot;
        let mut var_h3__blk1548_dn8: f64 = *var_h3__blk1548_dn8_slot;
        let mut var_h3__blk1548_rv: f64 = *var_h3__blk1548_rv_slot;
        let mut var_mtat__blk1580: f64 = *var_mtat__blk1580_slot;
        let mut var_mtat__blk1580_dn11: f64 = *var_mtat__blk1580_dn11_slot;
        let mut var_mtat__blk1580_dn12: f64 = *var_mtat__blk1580_dn12_slot;
        let mut var_mtat__blk1580_dn6: f64 = *var_mtat__blk1580_dn6_slot;
        let mut var_mtat__blk1580_dn7: f64 = *var_mtat__blk1580_dn7_slot;
        let mut var_mtat__blk1580_dn8: f64 = *var_mtat__blk1580_dn8_slot;
        let mut var_mtat__blk1580_dn9: f64 = *var_mtat__blk1580_dn9_slot;
        let mut var_mtat__blk1580_rv: f64 = *var_mtat__blk1580_rv_slot;
        let mut var_nu__blk1587: f64 = *var_nu__blk1587_slot;
        let mut var_nu__blk1587_dn11: f64 = *var_nu__blk1587_dn11_slot;
        let mut var_nu__blk1587_dn12: f64 = *var_nu__blk1587_dn12_slot;
        let mut var_nu__blk1587_dn7: f64 = *var_nu__blk1587_dn7_slot;
        let mut var_nu__blk1587_dn8: f64 = *var_nu__blk1587_dn8_slot;
        let mut var_nu__blk1587_rv: f64 = *var_nu__blk1587_rv_slot;
        let mut var_tmp__blk1560: f64 = *var_tmp__blk1560_slot;
        let mut var_tmp__blk1560_dn11: f64 = *var_tmp__blk1560_dn11_slot;
        let mut var_tmp__blk1560_dn12: f64 = *var_tmp__blk1560_dn12_slot;
        let mut var_tmp__blk1560_dn6: f64 = *var_tmp__blk1560_dn6_slot;
        let mut var_tmp__blk1560_dn7: f64 = *var_tmp__blk1560_dn7_slot;
        let mut var_tmp__blk1560_dn8: f64 = *var_tmp__blk1560_dn8_slot;
        let mut var_tmp__blk1560_dn9: f64 = *var_tmp__blk1560_dn9_slot;
        let mut var_tmp__blk1560_rv: f64 = *var_tmp__blk1560_rv_slot;
        let mut var_xerfc__blk1581: f64 = *var_xerfc__blk1581_slot;
        let mut var_xerfc__blk1581_dn11: f64 = *var_xerfc__blk1581_dn11_slot;
        let mut var_xerfc__blk1581_dn12: f64 = *var_xerfc__blk1581_dn12_slot;
        let mut var_xerfc__blk1581_dn6: f64 = *var_xerfc__blk1581_dn6_slot;
        let mut var_xerfc__blk1581_dn7: f64 = *var_xerfc__blk1581_dn7_slot;
        let mut var_xerfc__blk1581_dn8: f64 = *var_xerfc__blk1581_dn8_slot;
        let mut var_xerfc__blk1581_dn9: f64 = *var_xerfc__blk1581_dn9_slot;
        let mut var_xerfc__blk1581_rv: f64 = *var_xerfc__blk1581_rv_slot;
        let mut var_ysq__blk1542: f64 = *var_ysq__blk1542_slot;
        let mut var_ysq__blk1542_dn11: f64 = *var_ysq__blk1542_dn11_slot;
        let mut var_ysq__blk1542_dn12: f64 = *var_ysq__blk1542_dn12_slot;
        let mut var_ysq__blk1542_dn6: f64 = *var_ysq__blk1542_dn6_slot;
        let mut var_ysq__blk1542_dn7: f64 = *var_ysq__blk1542_dn7_slot;
        let mut var_ysq__blk1542_dn8: f64 = *var_ysq__blk1542_dn8_slot;
        let mut var_ysq__blk1542_dn9: f64 = *var_ysq__blk1542_dn9_slot;
        let mut var_ysq__blk1542_rv: f64 = *var_ysq__blk1542_rv_slot;

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
        var_mtat__blk1580_rv = 0.0;

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
        var_xerfc__blk1581_rv = 0.0;

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
        var_ysq__blk1542_rv = 0.0;

        let assign61360_e79548: f64 = (-var_ysq__blk1542);
        let assign61360_e79550: f64 = (assign61360_e79548 + var_mtat__blk1580);
        let assign61360_e79552: f64 = (-230.25850929940458);
        let assign61360_e79553: f64 = if assign61360_e79550 > assign61360_e79552 { 1.0 } else { 0.0 };
        var_guard1721 = assign61360_e79553;
        var_guard1721_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

        let assign61400_e79656: f64 = if var_xerfc__blk1581 > 0.0 { 1.0 } else { 0.0 };
        var_guard1722 = assign61400_e79656;
        var_guard1722_rv = 0.0;

        let assign61420_e79674: f64 = (-230.25850929940458);
        let assign61420_e79675: f64 = if var_mtat__blk1580 > assign61420_e79674 { 1.0 } else { 0.0 };
        var_guard1723 = assign61420_e79675;
        var_guard1723_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

        let assign61480_e79801: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard1724 = assign61480_e79801;
        var_guard1724_rv = 0.0;

        let assign61500_e79816: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard1725 = assign61500_e79816;
        var_guard1725_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_fmaxr__blk1585_rv = 0.0;

        let assign61540_e79881: f64 = (-var_fbbtgat_d);
        let assign61540_e79883: f64 = (assign61540_e79881 / var_fmaxr__blk1585);
        let assign61540_e79884: f64 = (assign61540_e79883).abs();
        let assign61540_e79886: f64 = if assign61540_e79884 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1726 = assign61540_e79886;
        var_guard1726_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

        let assign61560_e79907: f64 = (-var_fbbtgat_d);
        let assign61560_e79909: f64 = (assign61560_e79907 / var_fmaxr__blk1585);
        let assign61560_e79911: f64 = if assign61560_e79909 < 0.0 { 1.0 } else { 0.0 };
        var_guard1727 = assign61560_e79911;
        var_guard1727_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

        let assign61600_e80037: f64 = if var_vbrgat_var_d > 1000.0 { 1.0 } else { 0.0 };
        var_guard1728 = assign61600_e80037;
        var_guard1728_rv = 0.0;

        let assign61620_e80052: f64 = (-var_alphaav);
        let assign61620_e80054: f64 = (assign61620_e80052 * var_vbrgat_var_d);
        let assign61620_e80055: f64 = if var_vav__blk1559 > assign61620_e80054 { 1.0 } else { 0.0 };
        var_guard1729 = assign61620_e80055;
        var_guard1729_rv = 0.0;

        let assign61630_e80058: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard1730 = assign61630_e80058;
        var_guard1730_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

        let assign61690_e80178: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard1731 = assign61690_e80178;
        var_guard1731_rv = 0.0;

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
        var_nu__blk1587_rv = 0.0;

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
        var_h1__blk1545_rv = 0.0;

        let (assign61720_e80268,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61720_e80266: f64 = (var_vch_d / var_vfmin_d);
        (assign61720_e80266,)
    } else {
        (var_h2__blk1546,)
    }
};
        var_h2__blk1546 = assign61720_e80268;
        var_h2__blk1546_rv = 0.0;

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
        var_h2d__blk1547_rv = 0.0;

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
        var_h3__blk1548_rv = 0.0;

        *var_fmaxr__blk1585_slot = var_fmaxr__blk1585;
        *var_fmaxr__blk1585_dn11_slot = var_fmaxr__blk1585_dn11;
        *var_fmaxr__blk1585_dn12_slot = var_fmaxr__blk1585_dn12;
        *var_fmaxr__blk1585_dn6_slot = var_fmaxr__blk1585_dn6;
        *var_fmaxr__blk1585_dn7_slot = var_fmaxr__blk1585_dn7;
        *var_fmaxr__blk1585_dn8_slot = var_fmaxr__blk1585_dn8;
        *var_fmaxr__blk1585_dn9_slot = var_fmaxr__blk1585_dn9;
        *var_fmaxr__blk1585_rv_slot = var_fmaxr__blk1585_rv;
        *var_guard1721_slot = var_guard1721;
        *var_guard1721_rv_slot = var_guard1721_rv;
        *var_guard1722_slot = var_guard1722;
        *var_guard1722_rv_slot = var_guard1722_rv;
        *var_guard1723_slot = var_guard1723;
        *var_guard1723_rv_slot = var_guard1723_rv;
        *var_guard1724_slot = var_guard1724;
        *var_guard1724_rv_slot = var_guard1724_rv;
        *var_guard1725_slot = var_guard1725;
        *var_guard1725_rv_slot = var_guard1725_rv;
        *var_guard1726_slot = var_guard1726;
        *var_guard1726_rv_slot = var_guard1726_rv;
        *var_guard1727_slot = var_guard1727;
        *var_guard1727_rv_slot = var_guard1727_rv;
        *var_guard1728_slot = var_guard1728;
        *var_guard1728_rv_slot = var_guard1728_rv;
        *var_guard1729_slot = var_guard1729;
        *var_guard1729_rv_slot = var_guard1729_rv;
        *var_guard1730_slot = var_guard1730;
        *var_guard1730_rv_slot = var_guard1730_rv;
        *var_guard1731_slot = var_guard1731;
        *var_guard1731_rv_slot = var_guard1731_rv;
        *var_h1__blk1545_slot = var_h1__blk1545;
        *var_h1__blk1545_rv_slot = var_h1__blk1545_rv;
        *var_h2__blk1546_slot = var_h2__blk1546;
        *var_h2__blk1546_rv_slot = var_h2__blk1546_rv;
        *var_h2d__blk1547_slot = var_h2d__blk1547;
        *var_h2d__blk1547_dn11_slot = var_h2d__blk1547_dn11;
        *var_h2d__blk1547_dn12_slot = var_h2d__blk1547_dn12;
        *var_h2d__blk1547_dn7_slot = var_h2d__blk1547_dn7;
        *var_h2d__blk1547_dn8_slot = var_h2d__blk1547_dn8;
        *var_h2d__blk1547_rv_slot = var_h2d__blk1547_rv;
        *var_h3__blk1548_slot = var_h3__blk1548;
        *var_h3__blk1548_dn11_slot = var_h3__blk1548_dn11;
        *var_h3__blk1548_dn12_slot = var_h3__blk1548_dn12;
        *var_h3__blk1548_dn7_slot = var_h3__blk1548_dn7;
        *var_h3__blk1548_dn8_slot = var_h3__blk1548_dn8;
        *var_h3__blk1548_rv_slot = var_h3__blk1548_rv;
        *var_mtat__blk1580_slot = var_mtat__blk1580;
        *var_mtat__blk1580_dn11_slot = var_mtat__blk1580_dn11;
        *var_mtat__blk1580_dn12_slot = var_mtat__blk1580_dn12;
        *var_mtat__blk1580_dn6_slot = var_mtat__blk1580_dn6;
        *var_mtat__blk1580_dn7_slot = var_mtat__blk1580_dn7;
        *var_mtat__blk1580_dn8_slot = var_mtat__blk1580_dn8;
        *var_mtat__blk1580_dn9_slot = var_mtat__blk1580_dn9;
        *var_mtat__blk1580_rv_slot = var_mtat__blk1580_rv;
        *var_nu__blk1587_slot = var_nu__blk1587;
        *var_nu__blk1587_dn11_slot = var_nu__blk1587_dn11;
        *var_nu__blk1587_dn12_slot = var_nu__blk1587_dn12;
        *var_nu__blk1587_dn7_slot = var_nu__blk1587_dn7;
        *var_nu__blk1587_dn8_slot = var_nu__blk1587_dn8;
        *var_nu__blk1587_rv_slot = var_nu__blk1587_rv;
        *var_tmp__blk1560_slot = var_tmp__blk1560;
        *var_tmp__blk1560_dn11_slot = var_tmp__blk1560_dn11;
        *var_tmp__blk1560_dn12_slot = var_tmp__blk1560_dn12;
        *var_tmp__blk1560_dn6_slot = var_tmp__blk1560_dn6;
        *var_tmp__blk1560_dn7_slot = var_tmp__blk1560_dn7;
        *var_tmp__blk1560_dn8_slot = var_tmp__blk1560_dn8;
        *var_tmp__blk1560_dn9_slot = var_tmp__blk1560_dn9;
        *var_tmp__blk1560_rv_slot = var_tmp__blk1560_rv;
        *var_xerfc__blk1581_slot = var_xerfc__blk1581;
        *var_xerfc__blk1581_dn11_slot = var_xerfc__blk1581_dn11;
        *var_xerfc__blk1581_dn12_slot = var_xerfc__blk1581_dn12;
        *var_xerfc__blk1581_dn6_slot = var_xerfc__blk1581_dn6;
        *var_xerfc__blk1581_dn7_slot = var_xerfc__blk1581_dn7;
        *var_xerfc__blk1581_dn8_slot = var_xerfc__blk1581_dn8;
        *var_xerfc__blk1581_dn9_slot = var_xerfc__blk1581_dn9;
        *var_xerfc__blk1581_rv_slot = var_xerfc__blk1581_rv;
        *var_ysq__blk1542_slot = var_ysq__blk1542;
        *var_ysq__blk1542_dn11_slot = var_ysq__blk1542_dn11;
        *var_ysq__blk1542_dn12_slot = var_ysq__blk1542_dn12;
        *var_ysq__blk1542_dn6_slot = var_ysq__blk1542_dn6;
        *var_ysq__blk1542_dn7_slot = var_ysq__blk1542_dn7;
        *var_ysq__blk1542_dn8_slot = var_ysq__blk1542_dn8;
        *var_ysq__blk1542_dn9_slot = var_ysq__blk1542_dn9;
        *var_ysq__blk1542_rv_slot = var_ysq__blk1542_rv;
    }

    pub(super) fn stamp_reactive_block_78(
        p: &Parameters,
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_bet_i: f64,
        var_cox_qm: f64,
        var_cox_qm_dn4: f64,
        var_cox_qm_dn6: f64,
        var_cox_qm_dn7: f64,
        var_cox_qm_dn8: f64,
        var_cox_qm_dn9: f64,
        var_eta_p_ac: f64,
        var_eta_p_ac_dn4: f64,
        var_eta_p_ac_dn6: f64,
        var_eta_p_ac_dn7: f64,
        var_eta_p_ac_dn8: f64,
        var_eta_p_ac_dn9: f64,
        var_guard1589: f64,
        var_guard1590: f64,
        var_guard1714: f64,
        var_guard1731: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_nt: f64,
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
        var_sigvds: f64,
        var_vbiinvgat2nd_d: f64,
        var_vbiinvgat_d: f64,
        var_vch_d: f64,
        var_vfmin_d: f64,
        var_vj__blk1552: f64,
        var_vj__blk1552_dn11: f64,
        var_vj__blk1552_dn12: f64,
        var_vj__blk1552_dn7: f64,
        var_vj__blk1552_dn8: f64,
        var_vjun_d: f64,
        var_vjun_d_dn12: f64,
        var_vjun_d_dn8: f64,
        var_vtrgatd_i: f64,
        var_xg_dc: f64,
        var_cgeff_slot: &mut f64,
        var_cgeff_dn4_slot: &mut f64,
        var_cgeff_dn6_slot: &mut f64,
        var_cgeff_dn7_slot: &mut f64,
        var_cgeff_dn8_slot: &mut f64,
        var_cgeff_dn9_slot: &mut f64,
        var_cgeff_rv_slot: &mut f64,
        var_guard1732_slot: &mut f64,
        var_guard1732_rv_slot: &mut f64,
        var_guard1733_slot: &mut f64,
        var_guard1733_rv_slot: &mut f64,
        var_guard1734_slot: &mut f64,
        var_guard1734_rv_slot: &mut f64,
        var_guard1749_slot: &mut f64,
        var_guard1749_rv_slot: &mut f64,
        var_guard1782_slot: &mut f64,
        var_guard1782_rv_slot: &mut f64,
        var_guard1787_slot: &mut f64,
        var_guard1787_rv_slot: &mut f64,
        var_h1__blk1545_slot: &mut f64,
        var_h1__blk1545_rv_slot: &mut f64,
        var_h2__blk1546_slot: &mut f64,
        var_h2__blk1546_rv_slot: &mut f64,
        var_h2d__blk1547_slot: &mut f64,
        var_h2d__blk1547_dn11_slot: &mut f64,
        var_h2d__blk1547_dn12_slot: &mut f64,
        var_h2d__blk1547_dn7_slot: &mut f64,
        var_h2d__blk1547_dn8_slot: &mut f64,
        var_h2d__blk1547_rv_slot: &mut f64,
        var_h3__blk1548_slot: &mut f64,
        var_h3__blk1548_dn11_slot: &mut f64,
        var_h3__blk1548_dn12_slot: &mut f64,
        var_h3__blk1548_dn7_slot: &mut f64,
        var_h3__blk1548_dn8_slot: &mut f64,
        var_h3__blk1548_rv_slot: &mut f64,
        var_h4__blk1549_slot: &mut f64,
        var_h4__blk1549_dn11_slot: &mut f64,
        var_h4__blk1549_dn12_slot: &mut f64,
        var_h4__blk1549_dn7_slot: &mut f64,
        var_h4__blk1549_dn8_slot: &mut f64,
        var_h4__blk1549_rv_slot: &mut f64,
        var_h5__blk1550_slot: &mut f64,
        var_h5__blk1550_dn11_slot: &mut f64,
        var_h5__blk1550_dn12_slot: &mut f64,
        var_h5__blk1550_dn7_slot: &mut f64,
        var_h5__blk1550_dn8_slot: &mut f64,
        var_h5__blk1550_rv_slot: &mut f64,
        var_nu__blk1587_slot: &mut f64,
        var_nu__blk1587_dn11_slot: &mut f64,
        var_nu__blk1587_dn12_slot: &mut f64,
        var_nu__blk1587_dn7_slot: &mut f64,
        var_nu__blk1587_dn8_slot: &mut f64,
        var_nu__blk1587_rv_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn4_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qd_dn9_slot: &mut f64,
        var_qd_rv_slot: &mut f64,
        var_qfgd_slot: &mut f64,
        var_qfgd_dn6_slot: &mut f64,
        var_qfgd_dn7_slot: &mut f64,
        var_qfgd_dn8_slot: &mut f64,
        var_qfgd_rv_slot: &mut f64,
        var_qfgs_slot: &mut f64,
        var_qfgs_dn6_slot: &mut f64,
        var_qfgs_dn7_slot: &mut f64,
        var_qfgs_dn8_slot: &mut f64,
        var_qfgs_rv_slot: &mut f64,
        var_qjun_d_slot: &mut f64,
        var_qjun_d_dn11_slot: &mut f64,
        var_qjun_d_dn12_slot: &mut f64,
        var_qjun_d_dn6_slot: &mut f64,
        var_qjun_d_dn7_slot: &mut f64,
        var_qjun_d_dn8_slot: &mut f64,
        var_qjun_d_dn9_slot: &mut f64,
        var_qjun_d_rv_slot: &mut f64,
        var_qjun_s_slot: &mut f64,
        var_qjun_s_dn11_slot: &mut f64,
        var_qjun_s_dn12_slot: &mut f64,
        var_qjun_s_dn6_slot: &mut f64,
        var_qjun_s_dn7_slot: &mut f64,
        var_qjun_s_dn8_slot: &mut f64,
        var_qjun_s_dn9_slot: &mut f64,
        var_qjun_s_rv_slot: &mut f64,
        var_qjungat2nd_slot: &mut f64,
        var_qjungat2nd_dn11_slot: &mut f64,
        var_qjungat2nd_dn12_slot: &mut f64,
        var_qjungat2nd_dn6_slot: &mut f64,
        var_qjungat2nd_dn7_slot: &mut f64,
        var_qjungat2nd_dn8_slot: &mut f64,
        var_qjungat2nd_dn9_slot: &mut f64,
        var_qjungat2nd_rv_slot: &mut f64,
        var_qjungat_d_slot: &mut f64,
        var_qjungat_d_dn11_slot: &mut f64,
        var_qjungat_d_dn12_slot: &mut f64,
        var_qjungat_d_dn6_slot: &mut f64,
        var_qjungat_d_dn7_slot: &mut f64,
        var_qjungat_d_dn8_slot: &mut f64,
        var_qjungat_d_dn9_slot: &mut f64,
        var_qjungat_d_rv_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_dn4_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_qs_dn9_slot: &mut f64,
        var_qs_rv_slot: &mut f64,
        var_temp__blk1748_slot: &mut f64,
        var_temp__blk1748_dn4_slot: &mut f64,
        var_temp__blk1748_dn6_slot: &mut f64,
        var_temp__blk1748_dn7_slot: &mut f64,
        var_temp__blk1748_dn8_slot: &mut f64,
        var_temp__blk1748_dn9_slot: &mut f64,
        var_temp__blk1748_rv_slot: &mut f64,
        var_tmp__blk1560_slot: &mut f64,
        var_tmp__blk1560_dn11_slot: &mut f64,
        var_tmp__blk1560_dn12_slot: &mut f64,
        var_tmp__blk1560_dn6_slot: &mut f64,
        var_tmp__blk1560_dn7_slot: &mut f64,
        var_tmp__blk1560_dn8_slot: &mut f64,
        var_tmp__blk1560_dn9_slot: &mut f64,
        var_tmp__blk1560_rv_slot: &mut f64,
        var_vjtmp_slot: &mut f64,
        var_vjtmp_dn11_slot: &mut f64,
        var_vjtmp_dn12_slot: &mut f64,
        var_vjtmp_dn7_slot: &mut f64,
        var_vjtmp_dn8_slot: &mut f64,
        var_vjtmp_rv_slot: &mut f64,
    ) {
        let mut var_cgeff: f64 = *var_cgeff_slot;
        let mut var_cgeff_dn4: f64 = *var_cgeff_dn4_slot;
        let mut var_cgeff_dn6: f64 = *var_cgeff_dn6_slot;
        let mut var_cgeff_dn7: f64 = *var_cgeff_dn7_slot;
        let mut var_cgeff_dn8: f64 = *var_cgeff_dn8_slot;
        let mut var_cgeff_dn9: f64 = *var_cgeff_dn9_slot;
        let mut var_cgeff_rv: f64 = *var_cgeff_rv_slot;
        let mut var_guard1732: f64 = *var_guard1732_slot;
        let mut var_guard1732_rv: f64 = *var_guard1732_rv_slot;
        let mut var_guard1733: f64 = *var_guard1733_slot;
        let mut var_guard1733_rv: f64 = *var_guard1733_rv_slot;
        let mut var_guard1734: f64 = *var_guard1734_slot;
        let mut var_guard1734_rv: f64 = *var_guard1734_rv_slot;
        let mut var_guard1749: f64 = *var_guard1749_slot;
        let mut var_guard1749_rv: f64 = *var_guard1749_rv_slot;
        let mut var_guard1782: f64 = *var_guard1782_slot;
        let mut var_guard1782_rv: f64 = *var_guard1782_rv_slot;
        let mut var_guard1787: f64 = *var_guard1787_slot;
        let mut var_guard1787_rv: f64 = *var_guard1787_rv_slot;
        let mut var_h1__blk1545: f64 = *var_h1__blk1545_slot;
        let mut var_h1__blk1545_rv: f64 = *var_h1__blk1545_rv_slot;
        let mut var_h2__blk1546: f64 = *var_h2__blk1546_slot;
        let mut var_h2__blk1546_rv: f64 = *var_h2__blk1546_rv_slot;
        let mut var_h2d__blk1547: f64 = *var_h2d__blk1547_slot;
        let mut var_h2d__blk1547_dn11: f64 = *var_h2d__blk1547_dn11_slot;
        let mut var_h2d__blk1547_dn12: f64 = *var_h2d__blk1547_dn12_slot;
        let mut var_h2d__blk1547_dn7: f64 = *var_h2d__blk1547_dn7_slot;
        let mut var_h2d__blk1547_dn8: f64 = *var_h2d__blk1547_dn8_slot;
        let mut var_h2d__blk1547_rv: f64 = *var_h2d__blk1547_rv_slot;
        let mut var_h3__blk1548: f64 = *var_h3__blk1548_slot;
        let mut var_h3__blk1548_dn11: f64 = *var_h3__blk1548_dn11_slot;
        let mut var_h3__blk1548_dn12: f64 = *var_h3__blk1548_dn12_slot;
        let mut var_h3__blk1548_dn7: f64 = *var_h3__blk1548_dn7_slot;
        let mut var_h3__blk1548_dn8: f64 = *var_h3__blk1548_dn8_slot;
        let mut var_h3__blk1548_rv: f64 = *var_h3__blk1548_rv_slot;
        let mut var_h4__blk1549: f64 = *var_h4__blk1549_slot;
        let mut var_h4__blk1549_dn11: f64 = *var_h4__blk1549_dn11_slot;
        let mut var_h4__blk1549_dn12: f64 = *var_h4__blk1549_dn12_slot;
        let mut var_h4__blk1549_dn7: f64 = *var_h4__blk1549_dn7_slot;
        let mut var_h4__blk1549_dn8: f64 = *var_h4__blk1549_dn8_slot;
        let mut var_h4__blk1549_rv: f64 = *var_h4__blk1549_rv_slot;
        let mut var_h5__blk1550: f64 = *var_h5__blk1550_slot;
        let mut var_h5__blk1550_dn11: f64 = *var_h5__blk1550_dn11_slot;
        let mut var_h5__blk1550_dn12: f64 = *var_h5__blk1550_dn12_slot;
        let mut var_h5__blk1550_dn7: f64 = *var_h5__blk1550_dn7_slot;
        let mut var_h5__blk1550_dn8: f64 = *var_h5__blk1550_dn8_slot;
        let mut var_h5__blk1550_rv: f64 = *var_h5__blk1550_rv_slot;
        let mut var_nu__blk1587: f64 = *var_nu__blk1587_slot;
        let mut var_nu__blk1587_dn11: f64 = *var_nu__blk1587_dn11_slot;
        let mut var_nu__blk1587_dn12: f64 = *var_nu__blk1587_dn12_slot;
        let mut var_nu__blk1587_dn7: f64 = *var_nu__blk1587_dn7_slot;
        let mut var_nu__blk1587_dn8: f64 = *var_nu__blk1587_dn8_slot;
        let mut var_nu__blk1587_rv: f64 = *var_nu__blk1587_rv_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn4: f64 = *var_qd_dn4_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qd_dn9: f64 = *var_qd_dn9_slot;
        let mut var_qd_rv: f64 = *var_qd_rv_slot;
        let mut var_qfgd: f64 = *var_qfgd_slot;
        let mut var_qfgd_dn6: f64 = *var_qfgd_dn6_slot;
        let mut var_qfgd_dn7: f64 = *var_qfgd_dn7_slot;
        let mut var_qfgd_dn8: f64 = *var_qfgd_dn8_slot;
        let mut var_qfgd_rv: f64 = *var_qfgd_rv_slot;
        let mut var_qfgs: f64 = *var_qfgs_slot;
        let mut var_qfgs_dn6: f64 = *var_qfgs_dn6_slot;
        let mut var_qfgs_dn7: f64 = *var_qfgs_dn7_slot;
        let mut var_qfgs_dn8: f64 = *var_qfgs_dn8_slot;
        let mut var_qfgs_rv: f64 = *var_qfgs_rv_slot;
        let mut var_qjun_d: f64 = *var_qjun_d_slot;
        let mut var_qjun_d_dn11: f64 = *var_qjun_d_dn11_slot;
        let mut var_qjun_d_dn12: f64 = *var_qjun_d_dn12_slot;
        let mut var_qjun_d_dn6: f64 = *var_qjun_d_dn6_slot;
        let mut var_qjun_d_dn7: f64 = *var_qjun_d_dn7_slot;
        let mut var_qjun_d_dn8: f64 = *var_qjun_d_dn8_slot;
        let mut var_qjun_d_dn9: f64 = *var_qjun_d_dn9_slot;
        let mut var_qjun_d_rv: f64 = *var_qjun_d_rv_slot;
        let mut var_qjun_s: f64 = *var_qjun_s_slot;
        let mut var_qjun_s_dn11: f64 = *var_qjun_s_dn11_slot;
        let mut var_qjun_s_dn12: f64 = *var_qjun_s_dn12_slot;
        let mut var_qjun_s_dn6: f64 = *var_qjun_s_dn6_slot;
        let mut var_qjun_s_dn7: f64 = *var_qjun_s_dn7_slot;
        let mut var_qjun_s_dn8: f64 = *var_qjun_s_dn8_slot;
        let mut var_qjun_s_dn9: f64 = *var_qjun_s_dn9_slot;
        let mut var_qjun_s_rv: f64 = *var_qjun_s_rv_slot;
        let mut var_qjungat2nd: f64 = *var_qjungat2nd_slot;
        let mut var_qjungat2nd_dn11: f64 = *var_qjungat2nd_dn11_slot;
        let mut var_qjungat2nd_dn12: f64 = *var_qjungat2nd_dn12_slot;
        let mut var_qjungat2nd_dn6: f64 = *var_qjungat2nd_dn6_slot;
        let mut var_qjungat2nd_dn7: f64 = *var_qjungat2nd_dn7_slot;
        let mut var_qjungat2nd_dn8: f64 = *var_qjungat2nd_dn8_slot;
        let mut var_qjungat2nd_dn9: f64 = *var_qjungat2nd_dn9_slot;
        let mut var_qjungat2nd_rv: f64 = *var_qjungat2nd_rv_slot;
        let mut var_qjungat_d: f64 = *var_qjungat_d_slot;
        let mut var_qjungat_d_dn11: f64 = *var_qjungat_d_dn11_slot;
        let mut var_qjungat_d_dn12: f64 = *var_qjungat_d_dn12_slot;
        let mut var_qjungat_d_dn6: f64 = *var_qjungat_d_dn6_slot;
        let mut var_qjungat_d_dn7: f64 = *var_qjungat_d_dn7_slot;
        let mut var_qjungat_d_dn8: f64 = *var_qjungat_d_dn8_slot;
        let mut var_qjungat_d_dn9: f64 = *var_qjungat_d_dn9_slot;
        let mut var_qjungat_d_rv: f64 = *var_qjungat_d_rv_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_dn4: f64 = *var_qs_dn4_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_qs_dn9: f64 = *var_qs_dn9_slot;
        let mut var_qs_rv: f64 = *var_qs_rv_slot;
        let mut var_temp__blk1748: f64 = *var_temp__blk1748_slot;
        let mut var_temp__blk1748_dn4: f64 = *var_temp__blk1748_dn4_slot;
        let mut var_temp__blk1748_dn6: f64 = *var_temp__blk1748_dn6_slot;
        let mut var_temp__blk1748_dn7: f64 = *var_temp__blk1748_dn7_slot;
        let mut var_temp__blk1748_dn8: f64 = *var_temp__blk1748_dn8_slot;
        let mut var_temp__blk1748_dn9: f64 = *var_temp__blk1748_dn9_slot;
        let mut var_temp__blk1748_rv: f64 = *var_temp__blk1748_rv_slot;
        let mut var_tmp__blk1560: f64 = *var_tmp__blk1560_slot;
        let mut var_tmp__blk1560_dn11: f64 = *var_tmp__blk1560_dn11_slot;
        let mut var_tmp__blk1560_dn12: f64 = *var_tmp__blk1560_dn12_slot;
        let mut var_tmp__blk1560_dn6: f64 = *var_tmp__blk1560_dn6_slot;
        let mut var_tmp__blk1560_dn7: f64 = *var_tmp__blk1560_dn7_slot;
        let mut var_tmp__blk1560_dn8: f64 = *var_tmp__blk1560_dn8_slot;
        let mut var_tmp__blk1560_dn9: f64 = *var_tmp__blk1560_dn9_slot;
        let mut var_tmp__blk1560_rv: f64 = *var_tmp__blk1560_rv_slot;
        let mut var_vjtmp: f64 = *var_vjtmp_slot;
        let mut var_vjtmp_dn11: f64 = *var_vjtmp_dn11_slot;
        let mut var_vjtmp_dn12: f64 = *var_vjtmp_dn12_slot;
        let mut var_vjtmp_dn7: f64 = *var_vjtmp_dn7_slot;
        let mut var_vjtmp_dn8: f64 = *var_vjtmp_dn8_slot;
        let mut var_vjtmp_rv: f64 = *var_vjtmp_rv_slot;

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
        var_h4__blk1549_rv = 0.0;

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
        var_h5__blk1550_rv = 0.0;

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
        var_vjtmp_rv = 0.0;

        let assign61780_e80352: f64 = if var_one_minus_pgat_d == 0.5 { 1.0 } else { 0.0 };
        var_guard1732 = assign61780_e80352;
        var_guard1732_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_qjungat_d_rv = 0.0;

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
        var_nu__blk1587_rv = 0.0;

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
        var_h1__blk1545_rv = 0.0;

        let (assign61840_e80462,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61840_e80460: f64 = (var_vch_d / var_vfmin_d);
        (assign61840_e80460,)
    } else {
        (var_h2__blk1546,)
    }
};
        var_h2__blk1546 = assign61840_e80462;
        var_h2__blk1546_rv = 0.0;

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
        var_h2d__blk1547_rv = 0.0;

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
        var_h3__blk1548_rv = 0.0;

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
        var_h4__blk1549_rv = 0.0;

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
        var_h5__blk1550_rv = 0.0;

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
        var_vjtmp_rv = 0.0;

        let assign61900_e80546: f64 = if var_one_minus_pgat2nd_d == 0.5 { 1.0 } else { 0.0 };
        var_guard1733 = assign61900_e80546;
        var_guard1733_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_qjungat2nd_rv = 0.0;

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
        var_qjungat_d_rv = 0.0;

        let assign61950_e80627: f64 = if var_one_minus_pgat_d == 0.5 { 1.0 } else { 0.0 };
        var_guard1734 = assign61950_e80627;
        var_guard1734_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_tmp__blk1560_rv = 0.0;

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
        var_qjungat_d_rv = 0.0;

        let assign62240_e80805: f64 = (var_qg + var_qb);
        let assign62240_e80807: f64 = (assign62240_e80805 + var_qd);
        let assign62240_e80808: f64 = (-assign62240_e80807);
        var_qs = assign62240_e80808;
        var_qs_dn4 = (-((var_qg_dn4 + var_qb_dn4) + var_qd_dn4));
        var_qs_dn6 = (-((var_qg_dn6 + var_qb_dn6) + var_qd_dn6));
        var_qs_dn7 = (-((var_qg_dn7 + var_qb_dn7) + var_qd_dn7));
        var_qs_dn8 = (-((var_qg_dn8 + var_qb_dn8) + var_qd_dn8));
        var_qs_dn9 = (-((var_qg_dn9 + var_qb_dn9) + var_qd_dn9));
        var_qs_rv = 0.0;

        let assign62250_e80811: f64 = (var_qfgs + var_qgs_ov);
        var_qfgs = assign62250_e80811;
        var_qfgs_dn6 = (var_qfgs_dn6 + var_qgs_ov_dn6);
        var_qfgs_dn7 = (var_qfgs_dn7 + var_qgs_ov_dn7);
        var_qfgs_dn8 = (var_qfgs_dn8 + var_qgs_ov_dn8);
        var_qfgs_rv = 0.0;

        let assign62260_e80814: f64 = (var_qfgd + var_qgd_ov);
        var_qfgd = assign62260_e80814;
        var_qfgd_dn6 = (var_qfgd_dn6 + var_qgd_ov_dn6);
        var_qfgd_dn7 = (var_qfgd_dn7 + var_qgd_ov_dn7);
        var_qfgd_dn8 = (var_qfgd_dn8 + var_qgd_ov_dn8);
        var_qfgd_rv = 0.0;

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
        var_qjun_s_rv = 0.0;

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
        var_qjun_d_rv = 0.0;

        let assign62290_e80839: f64 = if var_sigvds < 0.0 { 1.0 } else { 0.0 };
        var_guard1749 = assign62290_e80839;
        var_guard1749_rv = 0.0;

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
        var_temp__blk1748_rv = 0.0;

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
        var_qd_rv = 0.0;

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
        var_qs_rv = 0.0;

        let assign62390_e80860: f64 = (var_cox_qm * var_eta_p_ac);
        var_cgeff = assign62390_e80860;
        var_cgeff_dn4 = ((var_cox_qm_dn4 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn4));
        var_cgeff_dn6 = ((var_cox_qm_dn6 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn6));
        var_cgeff_dn7 = ((var_cox_qm_dn7 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn7));
        var_cgeff_dn8 = ((var_cox_qm_dn8 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn8));
        var_cgeff_dn9 = ((var_cox_qm_dn9 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn9));
        var_cgeff_rv = 0.0;

        let assign62450_e80872: f64 = if ((var_xg_dc > 0.0) && (var_bet_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard1782 = assign62450_e80872;
        var_guard1782_rv = 0.0;

        let assign62760_e81238: f64 = if ((((p.p50 == 1.0) && (var_nt > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        var_guard1787 = assign62760_e81238;
        var_guard1787_rv = 0.0;

        *var_cgeff_slot = var_cgeff;
        *var_cgeff_dn4_slot = var_cgeff_dn4;
        *var_cgeff_dn6_slot = var_cgeff_dn6;
        *var_cgeff_dn7_slot = var_cgeff_dn7;
        *var_cgeff_dn8_slot = var_cgeff_dn8;
        *var_cgeff_dn9_slot = var_cgeff_dn9;
        *var_cgeff_rv_slot = var_cgeff_rv;
        *var_guard1732_slot = var_guard1732;
        *var_guard1732_rv_slot = var_guard1732_rv;
        *var_guard1733_slot = var_guard1733;
        *var_guard1733_rv_slot = var_guard1733_rv;
        *var_guard1734_slot = var_guard1734;
        *var_guard1734_rv_slot = var_guard1734_rv;
        *var_guard1749_slot = var_guard1749;
        *var_guard1749_rv_slot = var_guard1749_rv;
        *var_guard1782_slot = var_guard1782;
        *var_guard1782_rv_slot = var_guard1782_rv;
        *var_guard1787_slot = var_guard1787;
        *var_guard1787_rv_slot = var_guard1787_rv;
        *var_h1__blk1545_slot = var_h1__blk1545;
        *var_h1__blk1545_rv_slot = var_h1__blk1545_rv;
        *var_h2__blk1546_slot = var_h2__blk1546;
        *var_h2__blk1546_rv_slot = var_h2__blk1546_rv;
        *var_h2d__blk1547_slot = var_h2d__blk1547;
        *var_h2d__blk1547_dn11_slot = var_h2d__blk1547_dn11;
        *var_h2d__blk1547_dn12_slot = var_h2d__blk1547_dn12;
        *var_h2d__blk1547_dn7_slot = var_h2d__blk1547_dn7;
        *var_h2d__blk1547_dn8_slot = var_h2d__blk1547_dn8;
        *var_h2d__blk1547_rv_slot = var_h2d__blk1547_rv;
        *var_h3__blk1548_slot = var_h3__blk1548;
        *var_h3__blk1548_dn11_slot = var_h3__blk1548_dn11;
        *var_h3__blk1548_dn12_slot = var_h3__blk1548_dn12;
        *var_h3__blk1548_dn7_slot = var_h3__blk1548_dn7;
        *var_h3__blk1548_dn8_slot = var_h3__blk1548_dn8;
        *var_h3__blk1548_rv_slot = var_h3__blk1548_rv;
        *var_h4__blk1549_slot = var_h4__blk1549;
        *var_h4__blk1549_dn11_slot = var_h4__blk1549_dn11;
        *var_h4__blk1549_dn12_slot = var_h4__blk1549_dn12;
        *var_h4__blk1549_dn7_slot = var_h4__blk1549_dn7;
        *var_h4__blk1549_dn8_slot = var_h4__blk1549_dn8;
        *var_h4__blk1549_rv_slot = var_h4__blk1549_rv;
        *var_h5__blk1550_slot = var_h5__blk1550;
        *var_h5__blk1550_dn11_slot = var_h5__blk1550_dn11;
        *var_h5__blk1550_dn12_slot = var_h5__blk1550_dn12;
        *var_h5__blk1550_dn7_slot = var_h5__blk1550_dn7;
        *var_h5__blk1550_dn8_slot = var_h5__blk1550_dn8;
        *var_h5__blk1550_rv_slot = var_h5__blk1550_rv;
        *var_nu__blk1587_slot = var_nu__blk1587;
        *var_nu__blk1587_dn11_slot = var_nu__blk1587_dn11;
        *var_nu__blk1587_dn12_slot = var_nu__blk1587_dn12;
        *var_nu__blk1587_dn7_slot = var_nu__blk1587_dn7;
        *var_nu__blk1587_dn8_slot = var_nu__blk1587_dn8;
        *var_nu__blk1587_rv_slot = var_nu__blk1587_rv;
        *var_qd_slot = var_qd;
        *var_qd_dn4_slot = var_qd_dn4;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qd_dn9_slot = var_qd_dn9;
        *var_qd_rv_slot = var_qd_rv;
        *var_qfgd_slot = var_qfgd;
        *var_qfgd_dn6_slot = var_qfgd_dn6;
        *var_qfgd_dn7_slot = var_qfgd_dn7;
        *var_qfgd_dn8_slot = var_qfgd_dn8;
        *var_qfgd_rv_slot = var_qfgd_rv;
        *var_qfgs_slot = var_qfgs;
        *var_qfgs_dn6_slot = var_qfgs_dn6;
        *var_qfgs_dn7_slot = var_qfgs_dn7;
        *var_qfgs_dn8_slot = var_qfgs_dn8;
        *var_qfgs_rv_slot = var_qfgs_rv;
        *var_qjun_d_slot = var_qjun_d;
        *var_qjun_d_dn11_slot = var_qjun_d_dn11;
        *var_qjun_d_dn12_slot = var_qjun_d_dn12;
        *var_qjun_d_dn6_slot = var_qjun_d_dn6;
        *var_qjun_d_dn7_slot = var_qjun_d_dn7;
        *var_qjun_d_dn8_slot = var_qjun_d_dn8;
        *var_qjun_d_dn9_slot = var_qjun_d_dn9;
        *var_qjun_d_rv_slot = var_qjun_d_rv;
        *var_qjun_s_slot = var_qjun_s;
        *var_qjun_s_dn11_slot = var_qjun_s_dn11;
        *var_qjun_s_dn12_slot = var_qjun_s_dn12;
        *var_qjun_s_dn6_slot = var_qjun_s_dn6;
        *var_qjun_s_dn7_slot = var_qjun_s_dn7;
        *var_qjun_s_dn8_slot = var_qjun_s_dn8;
        *var_qjun_s_dn9_slot = var_qjun_s_dn9;
        *var_qjun_s_rv_slot = var_qjun_s_rv;
        *var_qjungat2nd_slot = var_qjungat2nd;
        *var_qjungat2nd_dn11_slot = var_qjungat2nd_dn11;
        *var_qjungat2nd_dn12_slot = var_qjungat2nd_dn12;
        *var_qjungat2nd_dn6_slot = var_qjungat2nd_dn6;
        *var_qjungat2nd_dn7_slot = var_qjungat2nd_dn7;
        *var_qjungat2nd_dn8_slot = var_qjungat2nd_dn8;
        *var_qjungat2nd_dn9_slot = var_qjungat2nd_dn9;
        *var_qjungat2nd_rv_slot = var_qjungat2nd_rv;
        *var_qjungat_d_slot = var_qjungat_d;
        *var_qjungat_d_dn11_slot = var_qjungat_d_dn11;
        *var_qjungat_d_dn12_slot = var_qjungat_d_dn12;
        *var_qjungat_d_dn6_slot = var_qjungat_d_dn6;
        *var_qjungat_d_dn7_slot = var_qjungat_d_dn7;
        *var_qjungat_d_dn8_slot = var_qjungat_d_dn8;
        *var_qjungat_d_dn9_slot = var_qjungat_d_dn9;
        *var_qjungat_d_rv_slot = var_qjungat_d_rv;
        *var_qs_slot = var_qs;
        *var_qs_dn4_slot = var_qs_dn4;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_qs_dn9_slot = var_qs_dn9;
        *var_qs_rv_slot = var_qs_rv;
        *var_temp__blk1748_slot = var_temp__blk1748;
        *var_temp__blk1748_dn4_slot = var_temp__blk1748_dn4;
        *var_temp__blk1748_dn6_slot = var_temp__blk1748_dn6;
        *var_temp__blk1748_dn7_slot = var_temp__blk1748_dn7;
        *var_temp__blk1748_dn8_slot = var_temp__blk1748_dn8;
        *var_temp__blk1748_dn9_slot = var_temp__blk1748_dn9;
        *var_temp__blk1748_rv_slot = var_temp__blk1748_rv;
        *var_tmp__blk1560_slot = var_tmp__blk1560;
        *var_tmp__blk1560_dn11_slot = var_tmp__blk1560_dn11;
        *var_tmp__blk1560_dn12_slot = var_tmp__blk1560_dn12;
        *var_tmp__blk1560_dn6_slot = var_tmp__blk1560_dn6;
        *var_tmp__blk1560_dn7_slot = var_tmp__blk1560_dn7;
        *var_tmp__blk1560_dn8_slot = var_tmp__blk1560_dn8;
        *var_tmp__blk1560_dn9_slot = var_tmp__blk1560_dn9;
        *var_tmp__blk1560_rv_slot = var_tmp__blk1560_rv;
        *var_vjtmp_slot = var_vjtmp;
        *var_vjtmp_dn11_slot = var_vjtmp_dn11;
        *var_vjtmp_dn12_slot = var_vjtmp_dn12;
        *var_vjtmp_dn7_slot = var_vjtmp_dn7;
        *var_vjtmp_dn8_slot = var_vjtmp_dn8;
        *var_vjtmp_rv_slot = var_vjtmp_rv;
    }

    pub(super) fn stamp_reactive_block_79(
        p: &Parameters,
        var_alpha_dc: f64,
        var_alpha_dc_dn4: f64,
        var_alpha_dc_dn6: f64,
        var_alpha_dc_dn7: f64,
        var_alpha_dc_dn8: f64,
        var_alpha_dc_dn9: f64,
        var_betnedge_i: f64,
        var_cox_over_q: f64,
        var_cox_qm: f64,
        var_cox_qm_dn4: f64,
        var_cox_qm_dn6: f64,
        var_cox_qm_dn7: f64,
        var_cox_qm_dn8: f64,
        var_cox_qm_dn9: f64,
        var_dsqredge: f64,
        var_dsqredge_dn4: f64,
        var_dsqredge_dn6: f64,
        var_dsqredge_dn7: f64,
        var_dsqredge_dn8: f64,
        var_dsqredge_dn9: f64,
        var_eta_p_ac: f64,
        var_eta_p_ac_dn4: f64,
        var_eta_p_ac_dn6: f64,
        var_eta_p_ac_dn7: f64,
        var_eta_p_ac_dn8: f64,
        var_eta_p_ac_dn9: f64,
        var_gfedge2: f64,
        var_gfedge2_dn4: f64,
        var_gmob_dl_ac: f64,
        var_gmob_dl_ac_dn4: f64,
        var_gmob_dl_ac_dn6: f64,
        var_gmob_dl_ac_dn7: f64,
        var_gmob_dl_ac_dn8: f64,
        var_gmob_dl_ac_dn9: f64,
        var_guard1782: f64,
        var_guard1787: f64,
        var_gvsat_ac: f64,
        var_gvsat_ac_dn4: f64,
        var_gvsat_ac_dn6: f64,
        var_gvsat_ac_dn7: f64,
        var_gvsat_ac_dn8: f64,
        var_gvsat_ac_dn9: f64,
        var_h_dc: f64,
        var_h_dc_dn4: f64,
        var_h_dc_dn6: f64,
        var_h_dc_dn7: f64,
        var_h_dc_dn8: f64,
        var_h_dc_dn9: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_xgedge: f64,
        var_cgeff_slot: &mut f64,
        var_cgeff_dn4_slot: &mut f64,
        var_cgeff_dn6_slot: &mut f64,
        var_cgeff_dn7_slot: &mut f64,
        var_cgeff_dn8_slot: &mut f64,
        var_cgeff_dn9_slot: &mut f64,
        var_cgeff_rv_slot: &mut f64,
        var_guard1791_slot: &mut f64,
        var_guard1791_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
    ) {
        let mut var_cgeff: f64 = *var_cgeff_slot;
        let mut var_cgeff_dn4: f64 = *var_cgeff_dn4_slot;
        let mut var_cgeff_dn6: f64 = *var_cgeff_dn6_slot;
        let mut var_cgeff_dn7: f64 = *var_cgeff_dn7_slot;
        let mut var_cgeff_dn8: f64 = *var_cgeff_dn8_slot;
        let mut var_cgeff_dn9: f64 = *var_cgeff_dn9_slot;
        let mut var_cgeff_rv: f64 = *var_cgeff_rv_slot;
        let mut var_guard1791: f64 = *var_guard1791_slot;
        let mut var_guard1791_rv: f64 = *var_guard1791_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;

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
        var_cgeff_rv = 0.0;

        let assign63070_e81549: f64 = if (((p.p46 != 0.0) && (var_betnedge_i > 0.0)) && (var_xgedge > 0.0)) { 1.0 } else { 0.0 };
        var_guard1791 = assign63070_e81549;
        var_guard1791_rv = 0.0;

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
        var_temp1_rv = 0.0;

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
        var_temp1_rv = 0.0;

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
        var_temp1_rv = 0.0;

        *var_cgeff_slot = var_cgeff;
        *var_cgeff_dn4_slot = var_cgeff_dn4;
        *var_cgeff_dn6_slot = var_cgeff_dn6;
        *var_cgeff_dn7_slot = var_cgeff_dn7;
        *var_cgeff_dn8_slot = var_cgeff_dn8;
        *var_cgeff_dn9_slot = var_cgeff_dn9;
        *var_cgeff_rv_slot = var_cgeff_rv;
        *var_guard1791_slot = var_guard1791;
        *var_guard1791_rv_slot = var_guard1791_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp1_rv_slot = var_temp1_rv;
    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        var_chnl_type: f64,
        var_gbulk: f64,
        var_gdrain: f64,
        var_ggate: f64,
        var_gjuns: f64,
        var_gsource: f64,
        var_guard1735: f64,
        var_guard1736: f64,
        var_guard1737: f64,
        var_guard1738: f64,
        var_guard1739: f64,
        var_guard1740: f64,
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
        var_i_gb: f64,
        var_i_gb_dn4: f64,
        var_i_gb_dn6: f64,
        var_i_gb_dn7: f64,
        var_i_gb_dn8: f64,
        var_i_gb_dn9: f64,
        var_i_gcd: f64,
        var_i_gcd_dn4: f64,
        var_i_gcd_dn6: f64,
        var_i_gcd_dn7: f64,
        var_i_gcd_dn8: f64,
        var_i_gcd_dn9: f64,
        var_i_gcs: f64,
        var_i_gcs_dn4: f64,
        var_i_gcs_dn6: f64,
        var_i_gcs_dn7: f64,
        var_i_gcs_dn8: f64,
        var_i_gcs_dn9: f64,
        var_i_gidl: f64,
        var_i_gidl_dn4: f64,
        var_i_gidl_dn6: f64,
        var_i_gidl_dn7: f64,
        var_i_gidl_dn8: f64,
        var_i_gidl_dn9: f64,
        var_i_gisl: f64,
        var_i_gisl_dn4: f64,
        var_i_gisl_dn6: f64,
        var_i_gisl_dn7: f64,
        var_i_gisl_dn8: f64,
        var_i_gisl_dn9: f64,
        var_igdov: f64,
        var_igdov_dn4: f64,
        var_igdov_dn6: f64,
        var_igdov_dn7: f64,
        var_igdov_dn8: f64,
        var_igdov_dn9: f64,
        var_igsov: f64,
        var_igsov_dn4: f64,
        var_igsov_dn6: f64,
        var_igsov_dn7: f64,
        var_igsov_dn8: f64,
        var_igsov_dn9: f64,
        var_iimpact: f64,
        var_iimpact_dn4: f64,
        var_iimpact_dn6: f64,
        var_iimpact_dn7: f64,
        var_iimpact_dn8: f64,
        var_iimpact_dn9: f64,
        var_ijun_d: f64,
        var_ijun_d_dn11: f64,
        var_ijun_d_dn12: f64,
        var_ijun_d_dn6: f64,
        var_ijun_d_dn7: f64,
        var_ijun_d_dn8: f64,
        var_ijun_d_dn9: f64,
        var_ijun_s: f64,
        var_ijun_s_dn11: f64,
        var_ijun_s_dn12: f64,
        var_ijun_s_dn6: f64,
        var_ijun_s_dn7: f64,
        var_ijun_s_dn8: f64,
        var_ijun_s_dn9: f64,
        var_mult_inst: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq0_e972, eq0_e972_d_n4, eq0_e972_d_n6, eq0_e972_d_n7, eq0_e972_d_n8, eq0_e972_d_n9,) = {
    if (var_guard1735 != 0.0) {
        let eq0_e966: f64 = (var_chnl_type * var_mult_inst);
        let eq0_e968: f64 = (eq0_e966 * p.p32);
        let eq0_e970: f64 = (eq0_e968 * var_iimpact);
        let eq0_e970_d_n4: f64 = (eq0_e968 * var_iimpact_dn4);
        let eq0_e970_d_n6: f64 = (eq0_e968 * var_iimpact_dn6);
        let eq0_e970_d_n7: f64 = (eq0_e968 * var_iimpact_dn7);
        let eq0_e970_d_n8: f64 = (eq0_e968 * var_iimpact_dn8);
        let eq0_e970_d_n9: f64 = (eq0_e968 * var_iimpact_dn9);
        (eq0_e970, eq0_e970_d_n4, eq0_e970_d_n6, eq0_e970_d_n7, eq0_e970_d_n8, eq0_e970_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e972;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq0_e972_d_n4), multiplicity * (eq0_e972_d_n6), multiplicity * (eq0_e972_d_n7), multiplicity * (eq0_e972_d_n8), multiplicity * (eq0_e972_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq1_e984, eq1_e984_d_n4, eq1_e984_d_n6, eq1_e984_d_n7, eq1_e984_d_n8, eq1_e984_d_n9,) = {
    if (var_guard1735 != 0.0) {
        let eq1_e976: f64 = (var_chnl_type * var_mult_inst);
        let eq1_e978: f64 = (eq1_e976 * p.p32);
        let eq1_e981: f64 = (var_i_ds + var_i_dsedge);
        let eq1_e981_d_n4: f64 = (var_i_ds_dn4 + var_i_dsedge_dn4);
        let eq1_e981_d_n6: f64 = (var_i_ds_dn6 + var_i_dsedge_dn6);
        let eq1_e981_d_n7: f64 = (var_i_ds_dn7 + var_i_dsedge_dn7);
        let eq1_e981_d_n8: f64 = (var_i_ds_dn8 + var_i_dsedge_dn8);
        let eq1_e981_d_n9: f64 = (var_i_ds_dn9 + var_i_dsedge_dn9);
        let eq1_e982: f64 = (eq1_e978 * eq1_e981);
        let eq1_e982_d_n4: f64 = (eq1_e978 * eq1_e981_d_n4);
        let eq1_e982_d_n6: f64 = (eq1_e978 * eq1_e981_d_n6);
        let eq1_e982_d_n7: f64 = (eq1_e978 * eq1_e981_d_n7);
        let eq1_e982_d_n8: f64 = (eq1_e978 * eq1_e981_d_n8);
        let eq1_e982_d_n9: f64 = (eq1_e978 * eq1_e981_d_n9);
        (eq1_e982, eq1_e982_d_n4, eq1_e982_d_n6, eq1_e982_d_n7, eq1_e982_d_n8, eq1_e982_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e984;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq1_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq1_e984_d_n4), multiplicity * (eq1_e984_d_n6), multiplicity * (eq1_e984_d_n7), multiplicity * (eq1_e984_d_n8), multiplicity * (eq1_e984_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq2_e994, eq2_e994_d_n4, eq2_e994_d_n6, eq2_e994_d_n7, eq2_e994_d_n8, eq2_e994_d_n9,) = {
    if (var_guard1735 != 0.0) {
        let eq2_e988: f64 = (var_chnl_type * var_mult_inst);
        let eq2_e990: f64 = (eq2_e988 * p.p32);
        let eq2_e992: f64 = (eq2_e990 * var_i_gcs);
        let eq2_e992_d_n4: f64 = (eq2_e990 * var_i_gcs_dn4);
        let eq2_e992_d_n6: f64 = (eq2_e990 * var_i_gcs_dn6);
        let eq2_e992_d_n7: f64 = (eq2_e990 * var_i_gcs_dn7);
        let eq2_e992_d_n8: f64 = (eq2_e990 * var_i_gcs_dn8);
        let eq2_e992_d_n9: f64 = (eq2_e990 * var_i_gcs_dn9);
        (eq2_e992, eq2_e992_d_n4, eq2_e992_d_n6, eq2_e992_d_n7, eq2_e992_d_n8, eq2_e992_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e994;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq2_e994_d_n4), multiplicity * (eq2_e994_d_n6), multiplicity * (eq2_e994_d_n7), multiplicity * (eq2_e994_d_n8), multiplicity * (eq2_e994_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq3_e1004, eq3_e1004_d_n4, eq3_e1004_d_n6, eq3_e1004_d_n7, eq3_e1004_d_n8, eq3_e1004_d_n9,) = {
    if (var_guard1735 != 0.0) {
        let eq3_e998: f64 = (var_chnl_type * var_mult_inst);
        let eq3_e1000: f64 = (eq3_e998 * p.p32);
        let eq3_e1002: f64 = (eq3_e1000 * var_i_gcd);
        let eq3_e1002_d_n4: f64 = (eq3_e1000 * var_i_gcd_dn4);
        let eq3_e1002_d_n6: f64 = (eq3_e1000 * var_i_gcd_dn6);
        let eq3_e1002_d_n7: f64 = (eq3_e1000 * var_i_gcd_dn7);
        let eq3_e1002_d_n8: f64 = (eq3_e1000 * var_i_gcd_dn8);
        let eq3_e1002_d_n9: f64 = (eq3_e1000 * var_i_gcd_dn9);
        (eq3_e1002, eq3_e1002_d_n4, eq3_e1002_d_n6, eq3_e1002_d_n7, eq3_e1002_d_n8, eq3_e1002_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e1004;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq3_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq3_e1004_d_n4), multiplicity * (eq3_e1004_d_n6), multiplicity * (eq3_e1004_d_n7), multiplicity * (eq3_e1004_d_n8), multiplicity * (eq3_e1004_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq4_e1015, eq4_e1015_d_n4, eq4_e1015_d_n6, eq4_e1015_d_n7, eq4_e1015_d_n8, eq4_e1015_d_n9,) = {
    if (var_guard1735 == 0.0) {
        let eq4_e1009: f64 = (var_chnl_type * var_mult_inst);
        let eq4_e1011: f64 = (eq4_e1009 * p.p32);
        let eq4_e1013: f64 = (eq4_e1011 * var_iimpact);
        let eq4_e1013_d_n4: f64 = (eq4_e1011 * var_iimpact_dn4);
        let eq4_e1013_d_n6: f64 = (eq4_e1011 * var_iimpact_dn6);
        let eq4_e1013_d_n7: f64 = (eq4_e1011 * var_iimpact_dn7);
        let eq4_e1013_d_n8: f64 = (eq4_e1011 * var_iimpact_dn8);
        let eq4_e1013_d_n9: f64 = (eq4_e1011 * var_iimpact_dn9);
        (eq4_e1013, eq4_e1013_d_n4, eq4_e1013_d_n6, eq4_e1013_d_n7, eq4_e1013_d_n8, eq4_e1013_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1015;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq4_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq4_e1015_d_n4), multiplicity * (eq4_e1015_d_n6), multiplicity * (eq4_e1015_d_n7), multiplicity * (eq4_e1015_d_n8), multiplicity * (eq4_e1015_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq5_e1028, eq5_e1028_d_n4, eq5_e1028_d_n6, eq5_e1028_d_n7, eq5_e1028_d_n8, eq5_e1028_d_n9,) = {
    if (var_guard1735 == 0.0) {
        let eq5_e1020: f64 = (var_chnl_type * var_mult_inst);
        let eq5_e1022: f64 = (eq5_e1020 * p.p32);
        let eq5_e1025: f64 = (var_i_ds + var_i_dsedge);
        let eq5_e1025_d_n4: f64 = (var_i_ds_dn4 + var_i_dsedge_dn4);
        let eq5_e1025_d_n6: f64 = (var_i_ds_dn6 + var_i_dsedge_dn6);
        let eq5_e1025_d_n7: f64 = (var_i_ds_dn7 + var_i_dsedge_dn7);
        let eq5_e1025_d_n8: f64 = (var_i_ds_dn8 + var_i_dsedge_dn8);
        let eq5_e1025_d_n9: f64 = (var_i_ds_dn9 + var_i_dsedge_dn9);
        let eq5_e1026: f64 = (eq5_e1022 * eq5_e1025);
        let eq5_e1026_d_n4: f64 = (eq5_e1022 * eq5_e1025_d_n4);
        let eq5_e1026_d_n6: f64 = (eq5_e1022 * eq5_e1025_d_n6);
        let eq5_e1026_d_n7: f64 = (eq5_e1022 * eq5_e1025_d_n7);
        let eq5_e1026_d_n8: f64 = (eq5_e1022 * eq5_e1025_d_n8);
        let eq5_e1026_d_n9: f64 = (eq5_e1022 * eq5_e1025_d_n9);
        (eq5_e1026, eq5_e1026_d_n4, eq5_e1026_d_n6, eq5_e1026_d_n7, eq5_e1026_d_n8, eq5_e1026_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1028;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq5_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq5_e1028_d_n4), multiplicity * (eq5_e1028_d_n6), multiplicity * (eq5_e1028_d_n7), multiplicity * (eq5_e1028_d_n8), multiplicity * (eq5_e1028_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq6_e1039, eq6_e1039_d_n4, eq6_e1039_d_n6, eq6_e1039_d_n7, eq6_e1039_d_n8, eq6_e1039_d_n9,) = {
    if (var_guard1735 == 0.0) {
        let eq6_e1033: f64 = (var_chnl_type * var_mult_inst);
        let eq6_e1035: f64 = (eq6_e1033 * p.p32);
        let eq6_e1037: f64 = (eq6_e1035 * var_i_gcs);
        let eq6_e1037_d_n4: f64 = (eq6_e1035 * var_i_gcs_dn4);
        let eq6_e1037_d_n6: f64 = (eq6_e1035 * var_i_gcs_dn6);
        let eq6_e1037_d_n7: f64 = (eq6_e1035 * var_i_gcs_dn7);
        let eq6_e1037_d_n8: f64 = (eq6_e1035 * var_i_gcs_dn8);
        let eq6_e1037_d_n9: f64 = (eq6_e1035 * var_i_gcs_dn9);
        (eq6_e1037, eq6_e1037_d_n4, eq6_e1037_d_n6, eq6_e1037_d_n7, eq6_e1037_d_n8, eq6_e1037_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1039;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq6_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq6_e1039_d_n4), multiplicity * (eq6_e1039_d_n6), multiplicity * (eq6_e1039_d_n7), multiplicity * (eq6_e1039_d_n8), multiplicity * (eq6_e1039_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq7_e1050, eq7_e1050_d_n4, eq7_e1050_d_n6, eq7_e1050_d_n7, eq7_e1050_d_n8, eq7_e1050_d_n9,) = {
    if (var_guard1735 == 0.0) {
        let eq7_e1044: f64 = (var_chnl_type * var_mult_inst);
        let eq7_e1046: f64 = (eq7_e1044 * p.p32);
        let eq7_e1048: f64 = (eq7_e1046 * var_i_gcd);
        let eq7_e1048_d_n4: f64 = (eq7_e1046 * var_i_gcd_dn4);
        let eq7_e1048_d_n6: f64 = (eq7_e1046 * var_i_gcd_dn6);
        let eq7_e1048_d_n7: f64 = (eq7_e1046 * var_i_gcd_dn7);
        let eq7_e1048_d_n8: f64 = (eq7_e1046 * var_i_gcd_dn8);
        let eq7_e1048_d_n9: f64 = (eq7_e1046 * var_i_gcd_dn9);
        (eq7_e1048, eq7_e1048_d_n4, eq7_e1048_d_n6, eq7_e1048_d_n7, eq7_e1048_d_n8, eq7_e1048_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1050;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq7_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq7_e1050_d_n4), multiplicity * (eq7_e1050_d_n6), multiplicity * (eq7_e1050_d_n7), multiplicity * (eq7_e1050_d_n8), multiplicity * (eq7_e1050_d_n9)],
            [],
            [],
            1.0,
        );
        let eq8_e1053: f64 = (var_chnl_type * var_mult_inst);
        let eq8_e1055: f64 = (eq8_e1053 * p.p32);
        let eq8_e1057: f64 = (eq8_e1055 * var_i_gb);
        let eq8_e1057_d_n4: f64 = (eq8_e1055 * var_i_gb_dn4);
        let eq8_e1057_d_n6: f64 = (eq8_e1055 * var_i_gb_dn6);
        let eq8_e1057_d_n7: f64 = (eq8_e1055 * var_i_gb_dn7);
        let eq8_e1057_d_n8: f64 = (eq8_e1055 * var_i_gb_dn8);
        let eq8_e1057_d_n9: f64 = (eq8_e1055 * var_i_gb_dn9);
        let eq8_value: f64 = eq8_e1057;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (eq8_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq8_e1057_d_n4), multiplicity * (eq8_e1057_d_n6), multiplicity * (eq8_e1057_d_n7), multiplicity * (eq8_e1057_d_n8), multiplicity * (eq8_e1057_d_n9)],
            [],
            [],
            1.0,
        );
        let eq9_e1060: f64 = (var_chnl_type * var_mult_inst);
        let eq9_e1062: f64 = (eq9_e1060 * p.p32);
        let eq9_e1064: f64 = (eq9_e1062 * var_igsov);
        let eq9_e1064_d_n4: f64 = (eq9_e1062 * var_igsov_dn4);
        let eq9_e1064_d_n6: f64 = (eq9_e1062 * var_igsov_dn6);
        let eq9_e1064_d_n7: f64 = (eq9_e1062 * var_igsov_dn7);
        let eq9_e1064_d_n8: f64 = (eq9_e1062 * var_igsov_dn8);
        let eq9_e1064_d_n9: f64 = (eq9_e1062 * var_igsov_dn9);
        let eq9_value: f64 = eq9_e1064;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq9_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq9_e1064_d_n4), multiplicity * (eq9_e1064_d_n6), multiplicity * (eq9_e1064_d_n7), multiplicity * (eq9_e1064_d_n8), multiplicity * (eq9_e1064_d_n9)],
            [],
            [],
            1.0,
        );
        let eq10_e1067: f64 = (var_chnl_type * var_mult_inst);
        let eq10_e1069: f64 = (eq10_e1067 * p.p32);
        let eq10_e1071: f64 = (eq10_e1069 * var_igdov);
        let eq10_e1071_d_n4: f64 = (eq10_e1069 * var_igdov_dn4);
        let eq10_e1071_d_n6: f64 = (eq10_e1069 * var_igdov_dn6);
        let eq10_e1071_d_n7: f64 = (eq10_e1069 * var_igdov_dn7);
        let eq10_e1071_d_n8: f64 = (eq10_e1069 * var_igdov_dn8);
        let eq10_e1071_d_n9: f64 = (eq10_e1069 * var_igdov_dn9);
        let eq10_value: f64 = eq10_e1071;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq10_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq10_e1071_d_n4), multiplicity * (eq10_e1071_d_n6), multiplicity * (eq10_e1071_d_n7), multiplicity * (eq10_e1071_d_n8), multiplicity * (eq10_e1071_d_n9)],
            [],
            [],
            1.0,
        );
        let eq11_e1074: f64 = (var_chnl_type * var_mult_inst);
        let eq11_e1076: f64 = (eq11_e1074 * p.p32);
        let eq11_e1078: f64 = (eq11_e1076 * var_i_gisl);
        let eq11_e1078_d_n4: f64 = (eq11_e1076 * var_i_gisl_dn4);
        let eq11_e1078_d_n6: f64 = (eq11_e1076 * var_i_gisl_dn6);
        let eq11_e1078_d_n7: f64 = (eq11_e1076 * var_i_gisl_dn7);
        let eq11_e1078_d_n8: f64 = (eq11_e1076 * var_i_gisl_dn8);
        let eq11_e1078_d_n9: f64 = (eq11_e1076 * var_i_gisl_dn9);
        let eq11_value: f64 = eq11_e1078;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq11_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq11_e1078_d_n4), multiplicity * (eq11_e1078_d_n6), multiplicity * (eq11_e1078_d_n7), multiplicity * (eq11_e1078_d_n8), multiplicity * (eq11_e1078_d_n9)],
            [],
            [],
            1.0,
        );
        let eq12_e1081: f64 = (var_chnl_type * var_mult_inst);
        let eq12_e1083: f64 = (eq12_e1081 * p.p32);
        let eq12_e1085: f64 = (eq12_e1083 * var_i_gidl);
        let eq12_e1085_d_n4: f64 = (eq12_e1083 * var_i_gidl_dn4);
        let eq12_e1085_d_n6: f64 = (eq12_e1083 * var_i_gidl_dn6);
        let eq12_e1085_d_n7: f64 = (eq12_e1083 * var_i_gidl_dn7);
        let eq12_e1085_d_n8: f64 = (eq12_e1083 * var_i_gidl_dn8);
        let eq12_e1085_d_n9: f64 = (eq12_e1083 * var_i_gidl_dn9);
        let eq12_value: f64 = eq12_e1085;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(9),
            multiplicity * (eq12_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq12_e1085_d_n4), multiplicity * (eq12_e1085_d_n6), multiplicity * (eq12_e1085_d_n7), multiplicity * (eq12_e1085_d_n8), multiplicity * (eq12_e1085_d_n9)],
            [],
            [],
            1.0,
        );
        let eq13_e1088: f64 = (var_chnl_type * var_mult_inst);
        let eq13_e1090: f64 = (eq13_e1088 * p.p32);
        let eq13_e1092: f64 = (eq13_e1090 * var_ijun_s);
        let eq13_e1092_d_n6: f64 = (eq13_e1090 * var_ijun_s_dn6);
        let eq13_e1092_d_n7: f64 = (eq13_e1090 * var_ijun_s_dn7);
        let eq13_e1092_d_n8: f64 = (eq13_e1090 * var_ijun_s_dn8);
        let eq13_e1092_d_n9: f64 = (eq13_e1090 * var_ijun_s_dn9);
        let eq13_e1092_d_n11: f64 = (eq13_e1090 * var_ijun_s_dn11);
        let eq13_e1092_d_n12: f64 = (eq13_e1090 * var_ijun_s_dn12);
        let eq13_value: f64 = eq13_e1092;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq13_value),
            [6, 7, 8, 9, 11, 12],
            [multiplicity * (eq13_e1092_d_n6), multiplicity * (eq13_e1092_d_n7), multiplicity * (eq13_e1092_d_n8), multiplicity * (eq13_e1092_d_n9), multiplicity * (eq13_e1092_d_n11), multiplicity * (eq13_e1092_d_n12)],
            [],
            [],
            1.0,
        );
        let eq14_e1095: f64 = (var_chnl_type * var_mult_inst);
        let eq14_e1097: f64 = (eq14_e1095 * p.p32);
        let eq14_e1099: f64 = (eq14_e1097 * var_ijun_d);
        let eq14_e1099_d_n6: f64 = (eq14_e1097 * var_ijun_d_dn6);
        let eq14_e1099_d_n7: f64 = (eq14_e1097 * var_ijun_d_dn7);
        let eq14_e1099_d_n8: f64 = (eq14_e1097 * var_ijun_d_dn8);
        let eq14_e1099_d_n9: f64 = (eq14_e1097 * var_ijun_d_dn9);
        let eq14_e1099_d_n11: f64 = (eq14_e1097 * var_ijun_d_dn11);
        let eq14_e1099_d_n12: f64 = (eq14_e1097 * var_ijun_d_dn12);
        let eq14_value: f64 = eq14_e1099;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(12),
            Some(8),
            multiplicity * (eq14_value),
            [6, 7, 8, 9, 11, 12],
            [multiplicity * (eq14_e1099_d_n6), multiplicity * (eq14_e1099_d_n7), multiplicity * (eq14_e1099_d_n8), multiplicity * (eq14_e1099_d_n9), multiplicity * (eq14_e1099_d_n11), multiplicity * (eq14_e1099_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq15_e1109, eq15_e1109_d_n1, eq15_e1109_d_n6,) = {
    if (var_guard1736 != 0.0) {
        let eq15_e1103: f64 = (var_mult_inst * p.p32);
        let eq15_e1105: f64 = (eq15_e1103 * var_ggate);
        let eq15_e1107: f64 = (eq15_e1105 * (nv1 - nv6));
        (eq15_e1107, eq15_e1105, (-eq15_e1105),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1109;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(6),
            multiplicity * (eq15_value),
            1,
            multiplicity * (eq15_e1109_d_n1),
            6,
            multiplicity * (eq15_e1109_d_n6),
        );
        let (eq17_e1124,) = {
    if (var_guard1736 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1124;
        stamper.stamp_potential_const_local(
            0,
            eq17_value,
        );
        let (eq18_e1134, eq18_e1134_d_n2, eq18_e1134_d_n7,) = {
    if (var_guard1737 != 0.0) {
        let eq18_e1128: f64 = (var_mult_inst * p.p32);
        let eq18_e1130: f64 = (eq18_e1128 * var_gsource);
        let eq18_e1132: f64 = (eq18_e1130 * (nv2 - nv7));
        (eq18_e1132, eq18_e1130, (-eq18_e1130),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1134;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(7),
            multiplicity * (eq18_value),
            2,
            multiplicity * (eq18_e1134_d_n2),
            7,
            multiplicity * (eq18_e1134_d_n7),
        );
        let (eq20_e1149,) = {
    if (var_guard1737 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1149;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
        let (eq21_e1159, eq21_e1159_d_n0, eq21_e1159_d_n8,) = {
    if (var_guard1738 != 0.0) {
        let eq21_e1153: f64 = (var_mult_inst * p.p32);
        let eq21_e1155: f64 = (eq21_e1153 * var_gdrain);
        let eq21_e1157: f64 = (eq21_e1155 * (nv0 - nv8));
        (eq21_e1157, eq21_e1155, (-eq21_e1155),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1159;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(8),
            multiplicity * (eq21_value),
            0,
            multiplicity * (eq21_e1159_d_n0),
            8,
            multiplicity * (eq21_e1159_d_n8),
        );
        let (eq23_e1174,) = {
    if (var_guard1738 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1174;
        stamper.stamp_potential_const_local(
            2,
            eq23_value,
        );
        let (eq24_e1184, eq24_e1184_d_n9, eq24_e1184_d_n10,) = {
    if (var_guard1739 != 0.0) {
        let eq24_e1178: f64 = (var_mult_inst * p.p32);
        let eq24_e1180: f64 = (eq24_e1178 * var_gbulk);
        let eq24_e1182: f64 = (eq24_e1180 * (nv9 - nv10));
        (eq24_e1182, eq24_e1180, (-eq24_e1180),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1184;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(10),
            multiplicity * (eq24_value),
            9,
            multiplicity * (eq24_e1184_d_n9),
            10,
            multiplicity * (eq24_e1184_d_n10),
        );
        let (eq26_e1199,) = {
    if (var_guard1739 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1199;
        stamper.stamp_potential_const_local(
            3,
            eq26_value,
        );
        let (eq27_e1209, eq27_e1209_d_n10, eq27_e1209_d_n11,) = {
    if (var_guard1740 != 0.0) {
        let eq27_e1203: f64 = (var_mult_inst * p.p32);
        let eq27_e1205: f64 = (eq27_e1203 * var_gjuns);
        let eq27_e1207: f64 = (eq27_e1205 * (nv11 - nv10));
        (eq27_e1207, (-eq27_e1205), eq27_e1205,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1209;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(10),
            multiplicity * (eq27_value),
            10,
            multiplicity * (eq27_e1209_d_n10),
            11,
            multiplicity * (eq27_e1209_d_n11),
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
        var_chnl_type: f64,
        var_cth_i: f64,
        var_gjund: f64,
        var_guard1740: f64,
        var_guard1741: f64,
        var_guard1742: f64,
        var_gwell: f64,
        var_mult_inst: f64,
        var_pdiss_1: f64,
        var_pdiss_1_dn0: f64,
        var_pdiss_1_dn2: f64,
        var_pdiss_1_dn4: f64,
        var_pdiss_1_dn6: f64,
        var_pdiss_1_dn7: f64,
        var_pdiss_1_dn8: f64,
        var_pdiss_1_dn9: f64,
        var_qb: f64,
        var_qb_dn4: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qb_dn9: f64,
        var_qd: f64,
        var_qd_dn4: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qd_dn8: f64,
        var_qd_dn9: f64,
        var_qfgd: f64,
        var_qfgd_dn6: f64,
        var_qfgd_dn7: f64,
        var_qfgd_dn8: f64,
        var_qfgs: f64,
        var_qfgs_dn6: f64,
        var_qfgs_dn7: f64,
        var_qfgs_dn8: f64,
        var_qg: f64,
        var_qg_dn4: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_qg_dn8: f64,
        var_qg_dn9: f64,
        var_qgb_ov: f64,
        var_qgb_ov_dn4: f64,
        var_qgb_ov_dn6: f64,
        var_qgb_ov_dn7: f64,
        var_qgb_ov_dn8: f64,
        var_qgb_ov_dn9: f64,
        var_qjun_d: f64,
        var_qjun_d_dn11: f64,
        var_qjun_d_dn12: f64,
        var_qjun_d_dn6: f64,
        var_qjun_d_dn7: f64,
        var_qjun_d_dn8: f64,
        var_qjun_d_dn9: f64,
        var_qjun_s: f64,
        var_qjun_s_dn11: f64,
        var_qjun_s_dn12: f64,
        var_qjun_s_dn6: f64,
        var_qjun_s_dn7: f64,
        var_qjun_s_dn8: f64,
        var_qjun_s_dn9: f64,
        var_rth_t: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq29_e1224,) = {
    if (var_guard1740 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1224;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
        );
        let (eq30_e1234, eq30_e1234_d_n10, eq30_e1234_d_n12,) = {
    if (var_guard1741 != 0.0) {
        let eq30_e1228: f64 = (var_mult_inst * p.p32);
        let eq30_e1230: f64 = (eq30_e1228 * var_gjund);
        let eq30_e1232: f64 = (eq30_e1230 * (nv12 - nv10));
        (eq30_e1232, (-eq30_e1230), eq30_e1230,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1234;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(10),
            multiplicity * (eq30_value),
            10,
            multiplicity * (eq30_e1234_d_n10),
            12,
            multiplicity * (eq30_e1234_d_n12),
        );
        let (eq32_e1249,) = {
    if (var_guard1741 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1249;
        stamper.stamp_potential_const_local(
            5,
            eq32_value,
        );
        let (eq33_e1259, eq33_e1259_d_n3, eq33_e1259_d_n10,) = {
    if (var_guard1742 != 0.0) {
        let eq33_e1253: f64 = (var_mult_inst * p.p32);
        let eq33_e1255: f64 = (eq33_e1253 * var_gwell);
        let eq33_e1257: f64 = (eq33_e1255 * (nv3 - nv10));
        (eq33_e1257, eq33_e1255, (-eq33_e1255),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1259;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(10),
            multiplicity * (eq33_value),
            3,
            multiplicity * (eq33_e1259_d_n3),
            10,
            multiplicity * (eq33_e1259_d_n10),
        );
        let (eq35_e1274,) = {
    if (var_guard1742 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1274;
        stamper.stamp_potential_const_local(
            6,
            eq35_value,
        );
        let eq38_e1286: f64 = (-var_mult_inst);
        let eq38_e1288: f64 = (eq38_e1286 * var_pdiss_1);
        let eq38_e1288_d_n0: f64 = (eq38_e1286 * var_pdiss_1_dn0);
        let eq38_e1288_d_n2: f64 = (eq38_e1286 * var_pdiss_1_dn2);
        let eq38_e1288_d_n4: f64 = (eq38_e1286 * var_pdiss_1_dn4);
        let eq38_e1288_d_n6: f64 = (eq38_e1286 * var_pdiss_1_dn6);
        let eq38_e1288_d_n7: f64 = (eq38_e1286 * var_pdiss_1_dn7);
        let eq38_e1288_d_n8: f64 = (eq38_e1286 * var_pdiss_1_dn8);
        let eq38_e1288_d_n9: f64 = (eq38_e1286 * var_pdiss_1_dn9);
        let eq38_value: f64 = eq38_e1288;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(4),
            None,
            multiplicity * (eq38_value),
            [0, 2, 4, 6, 7, 8, 9],
            [multiplicity * (eq38_e1288_d_n0), multiplicity * (eq38_e1288_d_n2), multiplicity * (eq38_e1288_d_n4), multiplicity * (eq38_e1288_d_n6), multiplicity * (eq38_e1288_d_n7), multiplicity * (eq38_e1288_d_n8), multiplicity * (eq38_e1288_d_n9)],
            [],
            [],
            1.0,
        );
        let eq39_e1291: f64 = (var_mult_inst * var_cth_i);
        let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));
        let eq39_e1294: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq39_e1293);
        let eq39_value: f64 = eq39_e1294;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq39_value),
            4,
            multiplicity * ((eq39_e1291 * ddt_scale)),
        );
        let eq40_e1297: f64 = (var_mult_inst * (nv4 - 0.0));
        let __rspice_inv_cse_0: f64 = 1.0 / var_rth_t;
        let eq40_e1299: f64 = (eq40_e1297 * __rspice_inv_cse_0);
        let eq40_e1299_d_n4: f64 = (var_mult_inst * __rspice_inv_cse_0);
        let eq40_value: f64 = eq40_e1299;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq40_value),
            4,
            multiplicity * (eq40_e1299_d_n4),
        );
        let eq41_e1302: f64 = (var_chnl_type * var_mult_inst);
        let eq41_e1304: f64 = (eq41_e1302 * p.p33);
        let eq41_e1306: f64 = (eq41_e1304 * var_qg);
        let eq41_e1306_d_n4: f64 = (eq41_e1304 * var_qg_dn4);
        let eq41_e1306_d_n6: f64 = (eq41_e1304 * var_qg_dn6);
        let eq41_e1306_d_n7: f64 = (eq41_e1304 * var_qg_dn7);
        let eq41_e1306_d_n8: f64 = (eq41_e1304 * var_qg_dn8);
        let eq41_e1306_d_n9: f64 = (eq41_e1304 * var_qg_dn9);
        let eq41_e1307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq41_e1306);
        let eq41_value: f64 = eq41_e1307;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq41_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq41_e1306_d_n4 * ddt_scale)), multiplicity * ((eq41_e1306_d_n6 * ddt_scale)), multiplicity * ((eq41_e1306_d_n7 * ddt_scale)), multiplicity * ((eq41_e1306_d_n8 * ddt_scale)), multiplicity * ((eq41_e1306_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq42_e1310: f64 = (var_chnl_type * var_mult_inst);
        let eq42_e1312: f64 = (eq42_e1310 * p.p33);
        let eq42_e1314: f64 = (eq42_e1312 * var_qb);
        let eq42_e1314_d_n4: f64 = (eq42_e1312 * var_qb_dn4);
        let eq42_e1314_d_n6: f64 = (eq42_e1312 * var_qb_dn6);
        let eq42_e1314_d_n7: f64 = (eq42_e1312 * var_qb_dn7);
        let eq42_e1314_d_n8: f64 = (eq42_e1312 * var_qb_dn8);
        let eq42_e1314_d_n9: f64 = (eq42_e1312 * var_qb_dn9);
        let eq42_e1315: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq42_e1314);
        let eq42_value: f64 = eq42_e1315;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq42_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq42_e1314_d_n4 * ddt_scale)), multiplicity * ((eq42_e1314_d_n6 * ddt_scale)), multiplicity * ((eq42_e1314_d_n7 * ddt_scale)), multiplicity * ((eq42_e1314_d_n8 * ddt_scale)), multiplicity * ((eq42_e1314_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq43_e1318: f64 = (var_chnl_type * var_mult_inst);
        let eq43_e1320: f64 = (eq43_e1318 * p.p33);
        let eq43_e1322: f64 = (eq43_e1320 * var_qd);
        let eq43_e1322_d_n4: f64 = (eq43_e1320 * var_qd_dn4);
        let eq43_e1322_d_n6: f64 = (eq43_e1320 * var_qd_dn6);
        let eq43_e1322_d_n7: f64 = (eq43_e1320 * var_qd_dn7);
        let eq43_e1322_d_n8: f64 = (eq43_e1320 * var_qd_dn8);
        let eq43_e1322_d_n9: f64 = (eq43_e1320 * var_qd_dn9);
        let eq43_e1323: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq43_e1322);
        let eq43_value: f64 = eq43_e1323;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq43_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq43_e1322_d_n4 * ddt_scale)), multiplicity * ((eq43_e1322_d_n6 * ddt_scale)), multiplicity * ((eq43_e1322_d_n7 * ddt_scale)), multiplicity * ((eq43_e1322_d_n8 * ddt_scale)), multiplicity * ((eq43_e1322_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq44_e1326: f64 = (var_chnl_type * var_mult_inst);
        let eq44_e1328: f64 = (eq44_e1326 * p.p33);
        let eq44_e1330: f64 = (eq44_e1328 * var_qfgs);
        let eq44_e1330_d_n6: f64 = (eq44_e1328 * var_qfgs_dn6);
        let eq44_e1330_d_n7: f64 = (eq44_e1328 * var_qfgs_dn7);
        let eq44_e1330_d_n8: f64 = (eq44_e1328 * var_qfgs_dn8);
        let eq44_e1331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq44_e1330);
        let eq44_value: f64 = eq44_e1331;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(7),
            multiplicity * (eq44_value),
            6,
            multiplicity * ((eq44_e1330_d_n6 * ddt_scale)),
            7,
            multiplicity * ((eq44_e1330_d_n7 * ddt_scale)),
            8,
            multiplicity * ((eq44_e1330_d_n8 * ddt_scale)),
        );
        let eq45_e1334: f64 = (var_chnl_type * var_mult_inst);
        let eq45_e1336: f64 = (eq45_e1334 * p.p33);
        let eq45_e1338: f64 = (eq45_e1336 * var_qfgd);
        let eq45_e1338_d_n6: f64 = (eq45_e1336 * var_qfgd_dn6);
        let eq45_e1338_d_n7: f64 = (eq45_e1336 * var_qfgd_dn7);
        let eq45_e1338_d_n8: f64 = (eq45_e1336 * var_qfgd_dn8);
        let eq45_e1339: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq45_e1338);
        let eq45_value: f64 = eq45_e1339;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(8),
            multiplicity * (eq45_value),
            6,
            multiplicity * ((eq45_e1338_d_n6 * ddt_scale)),
            7,
            multiplicity * ((eq45_e1338_d_n7 * ddt_scale)),
            8,
            multiplicity * ((eq45_e1338_d_n8 * ddt_scale)),
        );
        let eq46_e1342: f64 = (var_chnl_type * var_mult_inst);
        let eq46_e1344: f64 = (eq46_e1342 * p.p33);
        let eq46_e1346: f64 = (eq46_e1344 * var_qgb_ov);
        let eq46_e1346_d_n4: f64 = (eq46_e1344 * var_qgb_ov_dn4);
        let eq46_e1346_d_n6: f64 = (eq46_e1344 * var_qgb_ov_dn6);
        let eq46_e1346_d_n7: f64 = (eq46_e1344 * var_qgb_ov_dn7);
        let eq46_e1346_d_n8: f64 = (eq46_e1344 * var_qgb_ov_dn8);
        let eq46_e1346_d_n9: f64 = (eq46_e1344 * var_qgb_ov_dn9);
        let eq46_e1347: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq46_e1346);
        let eq46_value: f64 = eq46_e1347;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (eq46_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq46_e1346_d_n4 * ddt_scale)), multiplicity * ((eq46_e1346_d_n6 * ddt_scale)), multiplicity * ((eq46_e1346_d_n7 * ddt_scale)), multiplicity * ((eq46_e1346_d_n8 * ddt_scale)), multiplicity * ((eq46_e1346_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq47_e1350: f64 = (var_chnl_type * var_mult_inst);
        let eq47_e1352: f64 = (eq47_e1350 * p.p33);
        let eq47_e1354: f64 = (eq47_e1352 * var_qjun_s);
        let eq47_e1354_d_n6: f64 = (eq47_e1352 * var_qjun_s_dn6);
        let eq47_e1354_d_n7: f64 = (eq47_e1352 * var_qjun_s_dn7);
        let eq47_e1354_d_n8: f64 = (eq47_e1352 * var_qjun_s_dn8);
        let eq47_e1354_d_n9: f64 = (eq47_e1352 * var_qjun_s_dn9);
        let eq47_e1354_d_n11: f64 = (eq47_e1352 * var_qjun_s_dn11);
        let eq47_e1354_d_n12: f64 = (eq47_e1352 * var_qjun_s_dn12);
        let eq47_e1355: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq47_e1354);
        let eq47_value: f64 = eq47_e1355;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq47_value),
            [6, 7, 8, 9, 11, 12],
            [multiplicity * ((eq47_e1354_d_n6 * ddt_scale)), multiplicity * ((eq47_e1354_d_n7 * ddt_scale)), multiplicity * ((eq47_e1354_d_n8 * ddt_scale)), multiplicity * ((eq47_e1354_d_n9 * ddt_scale)), multiplicity * ((eq47_e1354_d_n11 * ddt_scale)), multiplicity * ((eq47_e1354_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq48_e1358: f64 = (var_chnl_type * var_mult_inst);
        let eq48_e1360: f64 = (eq48_e1358 * p.p33);
        let eq48_e1362: f64 = (eq48_e1360 * var_qjun_d);
        let eq48_e1362_d_n6: f64 = (eq48_e1360 * var_qjun_d_dn6);
        let eq48_e1362_d_n7: f64 = (eq48_e1360 * var_qjun_d_dn7);
        let eq48_e1362_d_n8: f64 = (eq48_e1360 * var_qjun_d_dn8);
        let eq48_e1362_d_n9: f64 = (eq48_e1360 * var_qjun_d_dn9);
        let eq48_e1362_d_n11: f64 = (eq48_e1360 * var_qjun_d_dn11);
        let eq48_e1362_d_n12: f64 = (eq48_e1360 * var_qjun_d_dn12);
        let eq48_e1363: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq48_e1362);
        let eq48_value: f64 = eq48_e1363;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(12),
            Some(8),
            multiplicity * (eq48_value),
            [6, 7, 8, 9, 11, 12],
            [multiplicity * ((eq48_e1362_d_n6 * ddt_scale)), multiplicity * ((eq48_e1362_d_n7 * ddt_scale)), multiplicity * ((eq48_e1362_d_n8 * ddt_scale)), multiplicity * ((eq48_e1362_d_n9 * ddt_scale)), multiplicity * ((eq48_e1362_d_n11 * ddt_scale)), multiplicity * ((eq48_e1362_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        var_cgeff: f64,
        var_cgeff_dn4: f64,
        var_cgeff_dn6: f64,
        var_cgeff_dn7: f64,
        var_cgeff_dn8: f64,
        var_cgeff_dn9: f64,
        var_mig: f64,
        var_mig_dn4: f64,
        var_mig_dn6: f64,
        var_mig_dn7: f64,
        var_mig_dn8: f64,
        var_mig_dn9: f64,
        var_mult_inst: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq50_e1371: f64 = ((nv5 - 0.0) / var_mig);
        let eq50_e1371_d_n4: f64 = (-(((nv5 - 0.0) * var_mig_dn4) / (var_mig * var_mig)));
        let eq50_e1371_d_n5: f64 = (1.0 / var_mig);
        let eq50_e1371_d_n6: f64 = (-(((nv5 - 0.0) * var_mig_dn6) / (var_mig * var_mig)));
        let eq50_e1371_d_n7: f64 = (-(((nv5 - 0.0) * var_mig_dn7) / (var_mig * var_mig)));
        let eq50_e1371_d_n8: f64 = (-(((nv5 - 0.0) * var_mig_dn8) / (var_mig * var_mig)));
        let eq50_e1371_d_n9: f64 = (-(((nv5 - 0.0) * var_mig_dn9) / (var_mig * var_mig)));
        let eq50_value: f64 = eq50_e1371;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq50_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq50_e1371_d_n4), multiplicity * (eq50_e1371_d_n5), multiplicity * (eq50_e1371_d_n6), multiplicity * (eq50_e1371_d_n7), multiplicity * (eq50_e1371_d_n8), multiplicity * (eq50_e1371_d_n9)],
            [],
            [],
            1.0,
        );
        let eq51_e1374: f64 = (var_cgeff * (nv5 - 0.0));
        let eq51_e1374_d_n4: f64 = (var_cgeff_dn4 * (nv5 - 0.0));
        let eq51_e1374_d_n6: f64 = (var_cgeff_dn6 * (nv5 - 0.0));
        let eq51_e1374_d_n7: f64 = (var_cgeff_dn7 * (nv5 - 0.0));
        let eq51_e1374_d_n8: f64 = (var_cgeff_dn8 * (nv5 - 0.0));
        let eq51_e1374_d_n9: f64 = (var_cgeff_dn9 * (nv5 - 0.0));
        let eq51_e1375: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq51_e1374);
        let eq51_value: f64 = eq51_e1375;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq51_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq51_e1374_d_n4 * ddt_scale)), multiplicity * ((var_cgeff * ddt_scale)), multiplicity * ((eq51_e1374_d_n6 * ddt_scale)), multiplicity * ((eq51_e1374_d_n7 * ddt_scale)), multiplicity * ((eq51_e1374_d_n8 * ddt_scale)), multiplicity * ((eq51_e1374_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq52_e1378: f64 = (var_mult_inst * p.p32);
        let eq52_e1379: f64 = (eq52_e1378).sqrt();
        let eq52_e1381: f64 = (eq52_e1379 * 0.5);
        let eq52_e1383: f64 = (eq52_e1381 * var_cgeff);
        let eq52_e1383_d_n4: f64 = (eq52_e1381 * var_cgeff_dn4);
        let eq52_e1383_d_n6: f64 = (eq52_e1381 * var_cgeff_dn6);
        let eq52_e1383_d_n7: f64 = (eq52_e1381 * var_cgeff_dn7);
        let eq52_e1383_d_n8: f64 = (eq52_e1381 * var_cgeff_dn8);
        let eq52_e1383_d_n9: f64 = (eq52_e1381 * var_cgeff_dn9);
        let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));
        let eq52_e1385_d_n4: f64 = (eq52_e1383_d_n4 * (nv5 - 0.0));
        let eq52_e1385_d_n6: f64 = (eq52_e1383_d_n6 * (nv5 - 0.0));
        let eq52_e1385_d_n7: f64 = (eq52_e1383_d_n7 * (nv5 - 0.0));
        let eq52_e1385_d_n8: f64 = (eq52_e1383_d_n8 * (nv5 - 0.0));
        let eq52_e1385_d_n9: f64 = (eq52_e1383_d_n9 * (nv5 - 0.0));
        let eq52_e1386: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e1385);
        let eq52_e1387: f64 = (-eq52_e1386);
        let eq52_e1387_d_n4: f64 = (-(eq52_e1385_d_n4 * ddt_scale));
        let eq52_e1387_d_n5: f64 = (-(eq52_e1383 * ddt_scale));
        let eq52_e1387_d_n6: f64 = (-(eq52_e1385_d_n6 * ddt_scale));
        let eq52_e1387_d_n7: f64 = (-(eq52_e1385_d_n7 * ddt_scale));
        let eq52_e1387_d_n8: f64 = (-(eq52_e1385_d_n8 * ddt_scale));
        let eq52_e1387_d_n9: f64 = (-(eq52_e1385_d_n9 * ddt_scale));
        let eq52_value: f64 = eq52_e1387;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq52_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq52_e1387_d_n4), multiplicity * (eq52_e1387_d_n5), multiplicity * (eq52_e1387_d_n6), multiplicity * (eq52_e1387_d_n7), multiplicity * (eq52_e1387_d_n8), multiplicity * (eq52_e1387_d_n9)],
            [],
            [],
            1.0,
        );
        let eq53_e1390: f64 = (var_mult_inst * p.p32);
        let eq53_e1391: f64 = (eq53_e1390).sqrt();
        let eq53_e1393: f64 = (eq53_e1391 * 0.5);
        let eq53_e1395: f64 = (eq53_e1393 * var_cgeff);
        let eq53_e1395_d_n4: f64 = (eq53_e1393 * var_cgeff_dn4);
        let eq53_e1395_d_n6: f64 = (eq53_e1393 * var_cgeff_dn6);
        let eq53_e1395_d_n7: f64 = (eq53_e1393 * var_cgeff_dn7);
        let eq53_e1395_d_n8: f64 = (eq53_e1393 * var_cgeff_dn8);
        let eq53_e1395_d_n9: f64 = (eq53_e1393 * var_cgeff_dn9);
        let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));
        let eq53_e1397_d_n4: f64 = (eq53_e1395_d_n4 * (nv5 - 0.0));
        let eq53_e1397_d_n6: f64 = (eq53_e1395_d_n6 * (nv5 - 0.0));
        let eq53_e1397_d_n7: f64 = (eq53_e1395_d_n7 * (nv5 - 0.0));
        let eq53_e1397_d_n8: f64 = (eq53_e1395_d_n8 * (nv5 - 0.0));
        let eq53_e1397_d_n9: f64 = (eq53_e1395_d_n9 * (nv5 - 0.0));
        let eq53_e1398: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq53_e1397);
        let eq53_e1399: f64 = (-eq53_e1398);
        let eq53_e1399_d_n4: f64 = (-(eq53_e1397_d_n4 * ddt_scale));
        let eq53_e1399_d_n5: f64 = (-(eq53_e1395 * ddt_scale));
        let eq53_e1399_d_n6: f64 = (-(eq53_e1397_d_n6 * ddt_scale));
        let eq53_e1399_d_n7: f64 = (-(eq53_e1397_d_n7 * ddt_scale));
        let eq53_e1399_d_n8: f64 = (-(eq53_e1397_d_n8 * ddt_scale));
        let eq53_e1399_d_n9: f64 = (-(eq53_e1397_d_n9 * ddt_scale));
        let eq53_value: f64 = eq53_e1399;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq53_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq53_e1399_d_n4), multiplicity * (eq53_e1399_d_n5), multiplicity * (eq53_e1399_d_n6), multiplicity * (eq53_e1399_d_n7), multiplicity * (eq53_e1399_d_n8), multiplicity * (eq53_e1399_d_n9)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_cgeff: f64,
        var_cgeff_dn4: f64,
        var_cgeff_dn6: f64,
        var_cgeff_dn7: f64,
        var_cgeff_dn8: f64,
        var_cgeff_dn9: f64,
        var_chnl_type: f64,
        var_cth_i: f64,
        var_mult_inst: f64,
        var_qb: f64,
        var_qb_dn4: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qb_dn9: f64,
        var_qd: f64,
        var_qd_dn4: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qd_dn8: f64,
        var_qd_dn9: f64,
        var_qfgd: f64,
        var_qfgd_dn6: f64,
        var_qfgd_dn7: f64,
        var_qfgd_dn8: f64,
        var_qfgs: f64,
        var_qfgs_dn6: f64,
        var_qfgs_dn7: f64,
        var_qfgs_dn8: f64,
        var_qg: f64,
        var_qg_dn4: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_qg_dn8: f64,
        var_qg_dn9: f64,
        var_qgb_ov: f64,
        var_qgb_ov_dn4: f64,
        var_qgb_ov_dn6: f64,
        var_qgb_ov_dn7: f64,
        var_qgb_ov_dn8: f64,
        var_qgb_ov_dn9: f64,
        var_qjun_d: f64,
        var_qjun_d_dn11: f64,
        var_qjun_d_dn12: f64,
        var_qjun_d_dn6: f64,
        var_qjun_d_dn7: f64,
        var_qjun_d_dn8: f64,
        var_qjun_d_dn9: f64,
        var_qjun_s: f64,
        var_qjun_s_dn11: f64,
        var_qjun_s_dn12: f64,
        var_qjun_s_dn6: f64,
        var_qjun_s_dn7: f64,
        var_qjun_s_dn8: f64,
        var_qjun_s_dn9: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq39_e1291: f64 = (var_mult_inst * var_cth_i);
        let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));
        let eq39_e1294_q: f64 = eq39_e1293;
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq39_e1291),
        );
        let eq41_e1302: f64 = (var_chnl_type * var_mult_inst);
        let eq41_e1304: f64 = (eq41_e1302 * p.p33);
        let eq41_e1306: f64 = (eq41_e1304 * var_qg);
        let eq41_e1306_d_n4: f64 = (eq41_e1304 * var_qg_dn4);
        let eq41_e1306_d_n6: f64 = (eq41_e1304 * var_qg_dn6);
        let eq41_e1306_d_n7: f64 = (eq41_e1304 * var_qg_dn7);
        let eq41_e1306_d_n8: f64 = (eq41_e1304 * var_qg_dn8);
        let eq41_e1306_d_n9: f64 = (eq41_e1304 * var_qg_dn9);
        let eq41_e1307_q: f64 = eq41_e1306;
        let eq41_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq41_e1306_d_n4, 0.0, eq41_e1306_d_n6, eq41_e1306_d_n7, eq41_e1306_d_n8, eq41_e1306_d_n9, 0.0, 0.0, 0.0];
        let eq41_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1310: f64 = (var_chnl_type * var_mult_inst);
        let eq42_e1312: f64 = (eq42_e1310 * p.p33);
        let eq42_e1314: f64 = (eq42_e1312 * var_qb);
        let eq42_e1314_d_n4: f64 = (eq42_e1312 * var_qb_dn4);
        let eq42_e1314_d_n6: f64 = (eq42_e1312 * var_qb_dn6);
        let eq42_e1314_d_n7: f64 = (eq42_e1312 * var_qb_dn7);
        let eq42_e1314_d_n8: f64 = (eq42_e1312 * var_qb_dn8);
        let eq42_e1314_d_n9: f64 = (eq42_e1312 * var_qb_dn9);
        let eq42_e1315_q: f64 = eq42_e1314;
        let eq42_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq42_e1314_d_n4, 0.0, eq42_e1314_d_n6, eq42_e1314_d_n7, eq42_e1314_d_n8, eq42_e1314_d_n9, 0.0, 0.0, 0.0];
        let eq42_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e1318: f64 = (var_chnl_type * var_mult_inst);
        let eq43_e1320: f64 = (eq43_e1318 * p.p33);
        let eq43_e1322: f64 = (eq43_e1320 * var_qd);
        let eq43_e1322_d_n4: f64 = (eq43_e1320 * var_qd_dn4);
        let eq43_e1322_d_n6: f64 = (eq43_e1320 * var_qd_dn6);
        let eq43_e1322_d_n7: f64 = (eq43_e1320 * var_qd_dn7);
        let eq43_e1322_d_n8: f64 = (eq43_e1320 * var_qd_dn8);
        let eq43_e1322_d_n9: f64 = (eq43_e1320 * var_qd_dn9);
        let eq43_e1323_q: f64 = eq43_e1322;
        let eq43_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq43_e1322_d_n4, 0.0, eq43_e1322_d_n6, eq43_e1322_d_n7, eq43_e1322_d_n8, eq43_e1322_d_n9, 0.0, 0.0, 0.0];
        let eq43_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let eq44_e1326: f64 = (var_chnl_type * var_mult_inst);
        let eq44_e1328: f64 = (eq44_e1326 * p.p33);
        let eq44_e1330: f64 = (eq44_e1328 * var_qfgs);
        let eq44_e1330_d_n6: f64 = (eq44_e1328 * var_qfgs_dn6);
        let eq44_e1330_d_n7: f64 = (eq44_e1328 * var_qfgs_dn7);
        let eq44_e1330_d_n8: f64 = (eq44_e1328 * var_qfgs_dn8);
        let eq44_e1331_q: f64 = eq44_e1330;
        stamper.stamp_current_reactive_node3(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes[6],
            multiplicity * (eq44_e1330_d_n6),
            nodes[7],
            multiplicity * (eq44_e1330_d_n7),
            nodes[8],
            multiplicity * (eq44_e1330_d_n8),
        );
        let eq45_e1334: f64 = (var_chnl_type * var_mult_inst);
        let eq45_e1336: f64 = (eq45_e1334 * p.p33);
        let eq45_e1338: f64 = (eq45_e1336 * var_qfgd);
        let eq45_e1338_d_n6: f64 = (eq45_e1336 * var_qfgd_dn6);
        let eq45_e1338_d_n7: f64 = (eq45_e1336 * var_qfgd_dn7);
        let eq45_e1338_d_n8: f64 = (eq45_e1336 * var_qfgd_dn8);
        let eq45_e1339_q: f64 = eq45_e1338;
        stamper.stamp_current_reactive_node3(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes[6],
            multiplicity * (eq45_e1338_d_n6),
            nodes[7],
            multiplicity * (eq45_e1338_d_n7),
            nodes[8],
            multiplicity * (eq45_e1338_d_n8),
        );
        let eq46_e1342: f64 = (var_chnl_type * var_mult_inst);
        let eq46_e1344: f64 = (eq46_e1342 * p.p33);
        let eq46_e1346: f64 = (eq46_e1344 * var_qgb_ov);
        let eq46_e1346_d_n4: f64 = (eq46_e1344 * var_qgb_ov_dn4);
        let eq46_e1346_d_n6: f64 = (eq46_e1344 * var_qgb_ov_dn6);
        let eq46_e1346_d_n7: f64 = (eq46_e1344 * var_qgb_ov_dn7);
        let eq46_e1346_d_n8: f64 = (eq46_e1344 * var_qgb_ov_dn8);
        let eq46_e1346_d_n9: f64 = (eq46_e1344 * var_qgb_ov_dn9);
        let eq46_e1347_q: f64 = eq46_e1346;
        let eq46_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq46_e1346_d_n4, 0.0, eq46_e1346_d_n6, eq46_e1346_d_n7, eq46_e1346_d_n8, eq46_e1346_d_n9, 0.0, 0.0, 0.0];
        let eq46_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[9]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let eq47_e1350: f64 = (var_chnl_type * var_mult_inst);
        let eq47_e1352: f64 = (eq47_e1350 * p.p33);
        let eq47_e1354: f64 = (eq47_e1352 * var_qjun_s);
        let eq47_e1354_d_n6: f64 = (eq47_e1352 * var_qjun_s_dn6);
        let eq47_e1354_d_n7: f64 = (eq47_e1352 * var_qjun_s_dn7);
        let eq47_e1354_d_n8: f64 = (eq47_e1352 * var_qjun_s_dn8);
        let eq47_e1354_d_n9: f64 = (eq47_e1352 * var_qjun_s_dn9);
        let eq47_e1354_d_n11: f64 = (eq47_e1352 * var_qjun_s_dn11);
        let eq47_e1354_d_n12: f64 = (eq47_e1352 * var_qjun_s_dn12);
        let eq47_e1355_q: f64 = eq47_e1354;
        let eq47_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq47_e1354_d_n6, eq47_e1354_d_n7, eq47_e1354_d_n8, eq47_e1354_d_n9, 0.0, eq47_e1354_d_n11, eq47_e1354_d_n12];
        let eq47_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let eq48_e1358: f64 = (var_chnl_type * var_mult_inst);
        let eq48_e1360: f64 = (eq48_e1358 * p.p33);
        let eq48_e1362: f64 = (eq48_e1360 * var_qjun_d);
        let eq48_e1362_d_n6: f64 = (eq48_e1360 * var_qjun_d_dn6);
        let eq48_e1362_d_n7: f64 = (eq48_e1360 * var_qjun_d_dn7);
        let eq48_e1362_d_n8: f64 = (eq48_e1360 * var_qjun_d_dn8);
        let eq48_e1362_d_n9: f64 = (eq48_e1360 * var_qjun_d_dn9);
        let eq48_e1362_d_n11: f64 = (eq48_e1360 * var_qjun_d_dn11);
        let eq48_e1362_d_n12: f64 = (eq48_e1360 * var_qjun_d_dn12);
        let eq48_e1363_q: f64 = eq48_e1362;
        let eq48_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq48_e1362_d_n6, eq48_e1362_d_n7, eq48_e1362_d_n8, eq48_e1362_d_n9, 0.0, eq48_e1362_d_n11, eq48_e1362_d_n12];
        let eq48_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[8]),
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let eq51_e1374: f64 = (var_cgeff * (nv5 - 0.0));
        let eq51_e1374_d_n4: f64 = (var_cgeff_dn4 * (nv5 - 0.0));
        let eq51_e1374_d_n6: f64 = (var_cgeff_dn6 * (nv5 - 0.0));
        let eq51_e1374_d_n7: f64 = (var_cgeff_dn7 * (nv5 - 0.0));
        let eq51_e1374_d_n8: f64 = (var_cgeff_dn8 * (nv5 - 0.0));
        let eq51_e1374_d_n9: f64 = (var_cgeff_dn9 * (nv5 - 0.0));
        let eq51_e1375_q: f64 = eq51_e1374;
        let eq51_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq51_e1374_d_n4, var_cgeff, eq51_e1374_d_n6, eq51_e1374_d_n7, eq51_e1374_d_n8, eq51_e1374_d_n9, 0.0, 0.0, 0.0];
        let eq51_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let eq52_e1378: f64 = (var_mult_inst * p.p32);
        let eq52_e1379: f64 = (eq52_e1378).sqrt();
        let eq52_e1381: f64 = (eq52_e1379 * 0.5);
        let eq52_e1383: f64 = (eq52_e1381 * var_cgeff);
        let eq52_e1383_d_n4: f64 = (eq52_e1381 * var_cgeff_dn4);
        let eq52_e1383_d_n6: f64 = (eq52_e1381 * var_cgeff_dn6);
        let eq52_e1383_d_n7: f64 = (eq52_e1381 * var_cgeff_dn7);
        let eq52_e1383_d_n8: f64 = (eq52_e1381 * var_cgeff_dn8);
        let eq52_e1383_d_n9: f64 = (eq52_e1381 * var_cgeff_dn9);
        let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));
        let eq52_e1385_d_n4: f64 = (eq52_e1383_d_n4 * (nv5 - 0.0));
        let eq52_e1385_d_n6: f64 = (eq52_e1383_d_n6 * (nv5 - 0.0));
        let eq52_e1385_d_n7: f64 = (eq52_e1383_d_n7 * (nv5 - 0.0));
        let eq52_e1385_d_n8: f64 = (eq52_e1383_d_n8 * (nv5 - 0.0));
        let eq52_e1385_d_n9: f64 = (eq52_e1383_d_n9 * (nv5 - 0.0));
        let eq52_e1386_q: f64 = eq52_e1385;
        let eq52_e1387: f64 = (-eq52_e1385);
        let eq52_e1387_q: f64 = (-eq52_e1386_q);
        let eq52_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, (-eq52_e1385_d_n4), (-eq52_e1383), (-eq52_e1385_d_n6), (-eq52_e1385_d_n7), (-eq52_e1385_d_n8), (-eq52_e1385_d_n9), 0.0, 0.0, 0.0];
        let eq52_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let eq53_e1390: f64 = (var_mult_inst * p.p32);
        let eq53_e1391: f64 = (eq53_e1390).sqrt();
        let eq53_e1393: f64 = (eq53_e1391 * 0.5);
        let eq53_e1395: f64 = (eq53_e1393 * var_cgeff);
        let eq53_e1395_d_n4: f64 = (eq53_e1393 * var_cgeff_dn4);
        let eq53_e1395_d_n6: f64 = (eq53_e1393 * var_cgeff_dn6);
        let eq53_e1395_d_n7: f64 = (eq53_e1393 * var_cgeff_dn7);
        let eq53_e1395_d_n8: f64 = (eq53_e1393 * var_cgeff_dn8);
        let eq53_e1395_d_n9: f64 = (eq53_e1393 * var_cgeff_dn9);
        let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));
        let eq53_e1397_d_n4: f64 = (eq53_e1395_d_n4 * (nv5 - 0.0));
        let eq53_e1397_d_n6: f64 = (eq53_e1395_d_n6 * (nv5 - 0.0));
        let eq53_e1397_d_n7: f64 = (eq53_e1395_d_n7 * (nv5 - 0.0));
        let eq53_e1397_d_n8: f64 = (eq53_e1395_d_n8 * (nv5 - 0.0));
        let eq53_e1397_d_n9: f64 = (eq53_e1395_d_n9 * (nv5 - 0.0));
        let eq53_e1398_q: f64 = eq53_e1397;
        let eq53_e1399: f64 = (-eq53_e1397);
        let eq53_e1399_q: f64 = (-eq53_e1398_q);
        let eq53_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, (-eq53_e1397_d_n4), (-eq53_e1395), (-eq53_e1397_d_n6), (-eq53_e1397_d_n7), (-eq53_e1397_d_n8), (-eq53_e1397_d_n9), 0.0, 0.0, 0.0];
        let eq53_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
