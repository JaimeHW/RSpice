#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_240(
        var_atatgat_d: f64,
        var_berfc: f64,
        var_btat__blk1553: f64,
        var_btat__blk1553_db0: f64,
        var_btat__blk1553_db1: f64,
        var_btat__blk1553_db2: f64,
        var_btat__blk1553_db3: f64,
        var_btat__blk1553_db4: f64,
        var_btat__blk1553_db5: f64,
        var_btat__blk1553_db6: f64,
        var_btat__blk1553_dn0: f64,
        var_btat__blk1553_dn1: f64,
        var_btat__blk1553_dn10: f64,
        var_btat__blk1553_dn11: f64,
        var_btat__blk1553_dn2: f64,
        var_btat__blk1553_dn3: f64,
        var_btat__blk1553_dn4: f64,
        var_btat__blk1553_dn5: f64,
        var_btat__blk1553_dn6: f64,
        var_btat__blk1553_dn7: f64,
        var_btat__blk1553_dn8: f64,
        var_btat__blk1553_dn9: f64,
        var_cerfc: f64,
        var_guard1572: f64,
        var_guard1573: f64,
        var_guard1697: f64,
        var_guard1701: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_perfc: f64,
        var_pgatd_i: f64,
        var_sqrtumax__blk1557: f64,
        var_sqrtumax__blk1557_db0: f64,
        var_sqrtumax__blk1557_db1: f64,
        var_sqrtumax__blk1557_db2: f64,
        var_sqrtumax__blk1557_db3: f64,
        var_sqrtumax__blk1557_db4: f64,
        var_sqrtumax__blk1557_db5: f64,
        var_sqrtumax__blk1557_db6: f64,
        var_sqrtumax__blk1557_dn0: f64,
        var_sqrtumax__blk1557_dn1: f64,
        var_sqrtumax__blk1557_dn10: f64,
        var_sqrtumax__blk1557_dn11: f64,
        var_sqrtumax__blk1557_dn2: f64,
        var_sqrtumax__blk1557_dn3: f64,
        var_sqrtumax__blk1557_dn4: f64,
        var_sqrtumax__blk1557_dn5: f64,
        var_sqrtumax__blk1557_dn6: f64,
        var_sqrtumax__blk1557_dn7: f64,
        var_sqrtumax__blk1557_dn8: f64,
        var_sqrtumax__blk1557_dn9: f64,
        var_twoatatoverthreebtat__blk1554: f64,
        var_twoatatoverthreebtat__blk1554_db0: f64,
        var_twoatatoverthreebtat__blk1554_db1: f64,
        var_twoatatoverthreebtat__blk1554_db2: f64,
        var_twoatatoverthreebtat__blk1554_db3: f64,
        var_twoatatoverthreebtat__blk1554_db4: f64,
        var_twoatatoverthreebtat__blk1554_db5: f64,
        var_twoatatoverthreebtat__blk1554_db6: f64,
        var_twoatatoverthreebtat__blk1554_dn0: f64,
        var_twoatatoverthreebtat__blk1554_dn1: f64,
        var_twoatatoverthreebtat__blk1554_dn10: f64,
        var_twoatatoverthreebtat__blk1554_dn11: f64,
        var_twoatatoverthreebtat__blk1554_dn2: f64,
        var_twoatatoverthreebtat__blk1554_dn3: f64,
        var_twoatatoverthreebtat__blk1554_dn4: f64,
        var_twoatatoverthreebtat__blk1554_dn5: f64,
        var_twoatatoverthreebtat__blk1554_dn6: f64,
        var_twoatatoverthreebtat__blk1554_dn7: f64,
        var_twoatatoverthreebtat__blk1554_dn8: f64,
        var_twoatatoverthreebtat__blk1554_dn9: f64,
        var_umax__blk1556: f64,
        var_umax__blk1556_db0: f64,
        var_umax__blk1556_db1: f64,
        var_umax__blk1556_db2: f64,
        var_umax__blk1556_db3: f64,
        var_umax__blk1556_db4: f64,
        var_umax__blk1556_db5: f64,
        var_umax__blk1556_db6: f64,
        var_umax__blk1556_dn0: f64,
        var_umax__blk1556_dn1: f64,
        var_umax__blk1556_dn10: f64,
        var_umax__blk1556_dn11: f64,
        var_umax__blk1556_dn2: f64,
        var_umax__blk1556_dn3: f64,
        var_umax__blk1556_dn4: f64,
        var_umax__blk1556_dn5: f64,
        var_umax__blk1556_dn6: f64,
        var_umax__blk1556_dn7: f64,
        var_umax__blk1556_dn8: f64,
        var_umax__blk1556_dn9: f64,
        var_wsrh__blk1549: f64,
        var_wsrh__blk1549_db0: f64,
        var_wsrh__blk1549_db1: f64,
        var_wsrh__blk1549_db2: f64,
        var_wsrh__blk1549_db3: f64,
        var_wsrh__blk1549_db4: f64,
        var_wsrh__blk1549_db5: f64,
        var_wsrh__blk1549_db6: f64,
        var_wsrh__blk1549_dn0: f64,
        var_wsrh__blk1549_dn1: f64,
        var_wsrh__blk1549_dn10: f64,
        var_wsrh__blk1549_dn11: f64,
        var_wsrh__blk1549_dn2: f64,
        var_wsrh__blk1549_dn3: f64,
        var_wsrh__blk1549_dn4: f64,
        var_wsrh__blk1549_dn5: f64,
        var_wsrh__blk1549_dn6: f64,
        var_wsrh__blk1549_dn7: f64,
        var_wsrh__blk1549_dn8: f64,
        var_wsrh__blk1549_dn9: f64,
        var_erfcpos__blk1527_slot: &mut f64,
        var_erfcpos__blk1527_db0_slot: &mut f64,
        var_erfcpos__blk1527_db1_slot: &mut f64,
        var_erfcpos__blk1527_db2_slot: &mut f64,
        var_erfcpos__blk1527_db3_slot: &mut f64,
        var_erfcpos__blk1527_db4_slot: &mut f64,
        var_erfcpos__blk1527_db5_slot: &mut f64,
        var_erfcpos__blk1527_db6_slot: &mut f64,
        var_erfcpos__blk1527_dn0_slot: &mut f64,
        var_erfcpos__blk1527_dn1_slot: &mut f64,
        var_erfcpos__blk1527_dn10_slot: &mut f64,
        var_erfcpos__blk1527_dn11_slot: &mut f64,
        var_erfcpos__blk1527_dn2_slot: &mut f64,
        var_erfcpos__blk1527_dn3_slot: &mut f64,
        var_erfcpos__blk1527_dn4_slot: &mut f64,
        var_erfcpos__blk1527_dn5_slot: &mut f64,
        var_erfcpos__blk1527_dn6_slot: &mut f64,
        var_erfcpos__blk1527_dn7_slot: &mut f64,
        var_erfcpos__blk1527_dn8_slot: &mut f64,
        var_erfcpos__blk1527_dn9_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_db0_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_db1_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_db2_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_db3_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_db4_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_db5_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_db6_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn0_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn1_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn10_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn11_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn2_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn3_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn4_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn5_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn6_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn7_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn8_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn9_slot: &mut f64,
        var_guard1702_slot: &mut f64,
        var_guard1703_slot: &mut f64,
        var_guard1704_slot: &mut f64,
        var_guard1705_slot: &mut f64,
        var_guard1706_slot: &mut f64,
        var_ktat__blk1561_slot: &mut f64,
        var_ktat__blk1561_db0_slot: &mut f64,
        var_ktat__blk1561_db1_slot: &mut f64,
        var_ktat__blk1561_db2_slot: &mut f64,
        var_ktat__blk1561_db3_slot: &mut f64,
        var_ktat__blk1561_db4_slot: &mut f64,
        var_ktat__blk1561_db5_slot: &mut f64,
        var_ktat__blk1561_db6_slot: &mut f64,
        var_ktat__blk1561_dn0_slot: &mut f64,
        var_ktat__blk1561_dn1_slot: &mut f64,
        var_ktat__blk1561_dn10_slot: &mut f64,
        var_ktat__blk1561_dn11_slot: &mut f64,
        var_ktat__blk1561_dn2_slot: &mut f64,
        var_ktat__blk1561_dn3_slot: &mut f64,
        var_ktat__blk1561_dn4_slot: &mut f64,
        var_ktat__blk1561_dn5_slot: &mut f64,
        var_ktat__blk1561_dn6_slot: &mut f64,
        var_ktat__blk1561_dn7_slot: &mut f64,
        var_ktat__blk1561_dn8_slot: &mut f64,
        var_ktat__blk1561_dn9_slot: &mut f64,
        var_ltat__blk1562_slot: &mut f64,
        var_ltat__blk1562_db0_slot: &mut f64,
        var_ltat__blk1562_db1_slot: &mut f64,
        var_ltat__blk1562_db2_slot: &mut f64,
        var_ltat__blk1562_db3_slot: &mut f64,
        var_ltat__blk1562_db4_slot: &mut f64,
        var_ltat__blk1562_db5_slot: &mut f64,
        var_ltat__blk1562_db6_slot: &mut f64,
        var_ltat__blk1562_dn0_slot: &mut f64,
        var_ltat__blk1562_dn1_slot: &mut f64,
        var_ltat__blk1562_dn10_slot: &mut f64,
        var_ltat__blk1562_dn11_slot: &mut f64,
        var_ltat__blk1562_dn2_slot: &mut f64,
        var_ltat__blk1562_dn3_slot: &mut f64,
        var_ltat__blk1562_dn4_slot: &mut f64,
        var_ltat__blk1562_dn5_slot: &mut f64,
        var_ltat__blk1562_dn6_slot: &mut f64,
        var_ltat__blk1562_dn7_slot: &mut f64,
        var_ltat__blk1562_dn8_slot: &mut f64,
        var_ltat__blk1562_dn9_slot: &mut f64,
        var_mtat__blk1563_slot: &mut f64,
        var_mtat__blk1563_db0_slot: &mut f64,
        var_mtat__blk1563_db1_slot: &mut f64,
        var_mtat__blk1563_db2_slot: &mut f64,
        var_mtat__blk1563_db3_slot: &mut f64,
        var_mtat__blk1563_db4_slot: &mut f64,
        var_mtat__blk1563_db5_slot: &mut f64,
        var_mtat__blk1563_db6_slot: &mut f64,
        var_mtat__blk1563_dn0_slot: &mut f64,
        var_mtat__blk1563_dn1_slot: &mut f64,
        var_mtat__blk1563_dn10_slot: &mut f64,
        var_mtat__blk1563_dn11_slot: &mut f64,
        var_mtat__blk1563_dn2_slot: &mut f64,
        var_mtat__blk1563_dn3_slot: &mut f64,
        var_mtat__blk1563_dn4_slot: &mut f64,
        var_mtat__blk1563_dn5_slot: &mut f64,
        var_mtat__blk1563_dn6_slot: &mut f64,
        var_mtat__blk1563_dn7_slot: &mut f64,
        var_mtat__blk1563_dn8_slot: &mut f64,
        var_mtat__blk1563_dn9_slot: &mut f64,
        var_terfc__blk1526_slot: &mut f64,
        var_terfc__blk1526_db0_slot: &mut f64,
        var_terfc__blk1526_db1_slot: &mut f64,
        var_terfc__blk1526_db2_slot: &mut f64,
        var_terfc__blk1526_db3_slot: &mut f64,
        var_terfc__blk1526_db4_slot: &mut f64,
        var_terfc__blk1526_db5_slot: &mut f64,
        var_terfc__blk1526_db6_slot: &mut f64,
        var_terfc__blk1526_dn0_slot: &mut f64,
        var_terfc__blk1526_dn1_slot: &mut f64,
        var_terfc__blk1526_dn10_slot: &mut f64,
        var_terfc__blk1526_dn11_slot: &mut f64,
        var_terfc__blk1526_dn2_slot: &mut f64,
        var_terfc__blk1526_dn3_slot: &mut f64,
        var_terfc__blk1526_dn4_slot: &mut f64,
        var_terfc__blk1526_dn5_slot: &mut f64,
        var_terfc__blk1526_dn6_slot: &mut f64,
        var_terfc__blk1526_dn7_slot: &mut f64,
        var_terfc__blk1526_dn8_slot: &mut f64,
        var_terfc__blk1526_dn9_slot: &mut f64,
        var_tmp__blk1543_slot: &mut f64,
        var_tmp__blk1543_db0_slot: &mut f64,
        var_tmp__blk1543_db1_slot: &mut f64,
        var_tmp__blk1543_db2_slot: &mut f64,
        var_tmp__blk1543_db3_slot: &mut f64,
        var_tmp__blk1543_db4_slot: &mut f64,
        var_tmp__blk1543_db5_slot: &mut f64,
        var_tmp__blk1543_db6_slot: &mut f64,
        var_tmp__blk1543_dn0_slot: &mut f64,
        var_tmp__blk1543_dn1_slot: &mut f64,
        var_tmp__blk1543_dn10_slot: &mut f64,
        var_tmp__blk1543_dn11_slot: &mut f64,
        var_tmp__blk1543_dn2_slot: &mut f64,
        var_tmp__blk1543_dn3_slot: &mut f64,
        var_tmp__blk1543_dn4_slot: &mut f64,
        var_tmp__blk1543_dn5_slot: &mut f64,
        var_tmp__blk1543_dn6_slot: &mut f64,
        var_tmp__blk1543_dn7_slot: &mut f64,
        var_tmp__blk1543_dn8_slot: &mut f64,
        var_tmp__blk1543_dn9_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_db0_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_db1_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_db2_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_db3_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_db4_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_db5_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_db6_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_dn0_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_dn1_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_dn10_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_dn11_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_dn2_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_dn3_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_dn4_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_dn5_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_dn6_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_dn7_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_dn8_slot: &mut f64,
        var_umaxpoweronepointfive__blk1558_dn9_slot: &mut f64,
        var_wgamma__blk1559_slot: &mut f64,
        var_wgamma__blk1559_db0_slot: &mut f64,
        var_wgamma__blk1559_db1_slot: &mut f64,
        var_wgamma__blk1559_db2_slot: &mut f64,
        var_wgamma__blk1559_db3_slot: &mut f64,
        var_wgamma__blk1559_db4_slot: &mut f64,
        var_wgamma__blk1559_db5_slot: &mut f64,
        var_wgamma__blk1559_db6_slot: &mut f64,
        var_wgamma__blk1559_dn0_slot: &mut f64,
        var_wgamma__blk1559_dn1_slot: &mut f64,
        var_wgamma__blk1559_dn10_slot: &mut f64,
        var_wgamma__blk1559_dn11_slot: &mut f64,
        var_wgamma__blk1559_dn2_slot: &mut f64,
        var_wgamma__blk1559_dn3_slot: &mut f64,
        var_wgamma__blk1559_dn4_slot: &mut f64,
        var_wgamma__blk1559_dn5_slot: &mut f64,
        var_wgamma__blk1559_dn6_slot: &mut f64,
        var_wgamma__blk1559_dn7_slot: &mut f64,
        var_wgamma__blk1559_dn8_slot: &mut f64,
        var_wgamma__blk1559_dn9_slot: &mut f64,
        var_wtat__blk1560_slot: &mut f64,
        var_wtat__blk1560_db0_slot: &mut f64,
        var_wtat__blk1560_db1_slot: &mut f64,
        var_wtat__blk1560_db2_slot: &mut f64,
        var_wtat__blk1560_db3_slot: &mut f64,
        var_wtat__blk1560_db4_slot: &mut f64,
        var_wtat__blk1560_db5_slot: &mut f64,
        var_wtat__blk1560_db6_slot: &mut f64,
        var_wtat__blk1560_dn0_slot: &mut f64,
        var_wtat__blk1560_dn1_slot: &mut f64,
        var_wtat__blk1560_dn10_slot: &mut f64,
        var_wtat__blk1560_dn11_slot: &mut f64,
        var_wtat__blk1560_dn2_slot: &mut f64,
        var_wtat__blk1560_dn3_slot: &mut f64,
        var_wtat__blk1560_dn4_slot: &mut f64,
        var_wtat__blk1560_dn5_slot: &mut f64,
        var_wtat__blk1560_dn6_slot: &mut f64,
        var_wtat__blk1560_dn7_slot: &mut f64,
        var_wtat__blk1560_dn8_slot: &mut f64,
        var_wtat__blk1560_dn9_slot: &mut f64,
        var_xerfc__blk1564_slot: &mut f64,
        var_xerfc__blk1564_db0_slot: &mut f64,
        var_xerfc__blk1564_db1_slot: &mut f64,
        var_xerfc__blk1564_db2_slot: &mut f64,
        var_xerfc__blk1564_db3_slot: &mut f64,
        var_xerfc__blk1564_db4_slot: &mut f64,
        var_xerfc__blk1564_db5_slot: &mut f64,
        var_xerfc__blk1564_db6_slot: &mut f64,
        var_xerfc__blk1564_dn0_slot: &mut f64,
        var_xerfc__blk1564_dn1_slot: &mut f64,
        var_xerfc__blk1564_dn10_slot: &mut f64,
        var_xerfc__blk1564_dn11_slot: &mut f64,
        var_xerfc__blk1564_dn2_slot: &mut f64,
        var_xerfc__blk1564_dn3_slot: &mut f64,
        var_xerfc__blk1564_dn4_slot: &mut f64,
        var_xerfc__blk1564_dn5_slot: &mut f64,
        var_xerfc__blk1564_dn6_slot: &mut f64,
        var_xerfc__blk1564_dn7_slot: &mut f64,
        var_xerfc__blk1564_dn8_slot: &mut f64,
        var_xerfc__blk1564_dn9_slot: &mut f64,
        var_ysq__blk1525_slot: &mut f64,
        var_ysq__blk1525_db0_slot: &mut f64,
        var_ysq__blk1525_db1_slot: &mut f64,
        var_ysq__blk1525_db2_slot: &mut f64,
        var_ysq__blk1525_db3_slot: &mut f64,
        var_ysq__blk1525_db4_slot: &mut f64,
        var_ysq__blk1525_db5_slot: &mut f64,
        var_ysq__blk1525_db6_slot: &mut f64,
        var_ysq__blk1525_dn0_slot: &mut f64,
        var_ysq__blk1525_dn1_slot: &mut f64,
        var_ysq__blk1525_dn10_slot: &mut f64,
        var_ysq__blk1525_dn11_slot: &mut f64,
        var_ysq__blk1525_dn2_slot: &mut f64,
        var_ysq__blk1525_dn3_slot: &mut f64,
        var_ysq__blk1525_dn4_slot: &mut f64,
        var_ysq__blk1525_dn5_slot: &mut f64,
        var_ysq__blk1525_dn6_slot: &mut f64,
        var_ysq__blk1525_dn7_slot: &mut f64,
        var_ysq__blk1525_dn8_slot: &mut f64,
        var_ysq__blk1525_dn9_slot: &mut f64,
    ) {
        let mut var_erfcpos__blk1527: f64 = *var_erfcpos__blk1527_slot;
        let mut var_erfcpos__blk1527_db0: f64 = *var_erfcpos__blk1527_db0_slot;
        let mut var_erfcpos__blk1527_db1: f64 = *var_erfcpos__blk1527_db1_slot;
        let mut var_erfcpos__blk1527_db2: f64 = *var_erfcpos__blk1527_db2_slot;
        let mut var_erfcpos__blk1527_db3: f64 = *var_erfcpos__blk1527_db3_slot;
        let mut var_erfcpos__blk1527_db4: f64 = *var_erfcpos__blk1527_db4_slot;
        let mut var_erfcpos__blk1527_db5: f64 = *var_erfcpos__blk1527_db5_slot;
        let mut var_erfcpos__blk1527_db6: f64 = *var_erfcpos__blk1527_db6_slot;
        let mut var_erfcpos__blk1527_dn0: f64 = *var_erfcpos__blk1527_dn0_slot;
        let mut var_erfcpos__blk1527_dn1: f64 = *var_erfcpos__blk1527_dn1_slot;
        let mut var_erfcpos__blk1527_dn10: f64 = *var_erfcpos__blk1527_dn10_slot;
        let mut var_erfcpos__blk1527_dn11: f64 = *var_erfcpos__blk1527_dn11_slot;
        let mut var_erfcpos__blk1527_dn2: f64 = *var_erfcpos__blk1527_dn2_slot;
        let mut var_erfcpos__blk1527_dn3: f64 = *var_erfcpos__blk1527_dn3_slot;
        let mut var_erfcpos__blk1527_dn4: f64 = *var_erfcpos__blk1527_dn4_slot;
        let mut var_erfcpos__blk1527_dn5: f64 = *var_erfcpos__blk1527_dn5_slot;
        let mut var_erfcpos__blk1527_dn6: f64 = *var_erfcpos__blk1527_dn6_slot;
        let mut var_erfcpos__blk1527_dn7: f64 = *var_erfcpos__blk1527_dn7_slot;
        let mut var_erfcpos__blk1527_dn8: f64 = *var_erfcpos__blk1527_dn8_slot;
        let mut var_erfcpos__blk1527_dn9: f64 = *var_erfcpos__blk1527_dn9_slot;
        let mut var_erfctimesexpmtat__blk1565: f64 = *var_erfctimesexpmtat__blk1565_slot;
        let mut var_erfctimesexpmtat__blk1565_db0: f64 = *var_erfctimesexpmtat__blk1565_db0_slot;
        let mut var_erfctimesexpmtat__blk1565_db1: f64 = *var_erfctimesexpmtat__blk1565_db1_slot;
        let mut var_erfctimesexpmtat__blk1565_db2: f64 = *var_erfctimesexpmtat__blk1565_db2_slot;
        let mut var_erfctimesexpmtat__blk1565_db3: f64 = *var_erfctimesexpmtat__blk1565_db3_slot;
        let mut var_erfctimesexpmtat__blk1565_db4: f64 = *var_erfctimesexpmtat__blk1565_db4_slot;
        let mut var_erfctimesexpmtat__blk1565_db5: f64 = *var_erfctimesexpmtat__blk1565_db5_slot;
        let mut var_erfctimesexpmtat__blk1565_db6: f64 = *var_erfctimesexpmtat__blk1565_db6_slot;
        let mut var_erfctimesexpmtat__blk1565_dn0: f64 = *var_erfctimesexpmtat__blk1565_dn0_slot;
        let mut var_erfctimesexpmtat__blk1565_dn1: f64 = *var_erfctimesexpmtat__blk1565_dn1_slot;
        let mut var_erfctimesexpmtat__blk1565_dn10: f64 = *var_erfctimesexpmtat__blk1565_dn10_slot;
        let mut var_erfctimesexpmtat__blk1565_dn11: f64 = *var_erfctimesexpmtat__blk1565_dn11_slot;
        let mut var_erfctimesexpmtat__blk1565_dn2: f64 = *var_erfctimesexpmtat__blk1565_dn2_slot;
        let mut var_erfctimesexpmtat__blk1565_dn3: f64 = *var_erfctimesexpmtat__blk1565_dn3_slot;
        let mut var_erfctimesexpmtat__blk1565_dn4: f64 = *var_erfctimesexpmtat__blk1565_dn4_slot;
        let mut var_erfctimesexpmtat__blk1565_dn5: f64 = *var_erfctimesexpmtat__blk1565_dn5_slot;
        let mut var_erfctimesexpmtat__blk1565_dn6: f64 = *var_erfctimesexpmtat__blk1565_dn6_slot;
        let mut var_erfctimesexpmtat__blk1565_dn7: f64 = *var_erfctimesexpmtat__blk1565_dn7_slot;
        let mut var_erfctimesexpmtat__blk1565_dn8: f64 = *var_erfctimesexpmtat__blk1565_dn8_slot;
        let mut var_erfctimesexpmtat__blk1565_dn9: f64 = *var_erfctimesexpmtat__blk1565_dn9_slot;
        let mut var_guard1702: f64 = *var_guard1702_slot;
        let mut var_guard1703: f64 = *var_guard1703_slot;
        let mut var_guard1704: f64 = *var_guard1704_slot;
        let mut var_guard1705: f64 = *var_guard1705_slot;
        let mut var_guard1706: f64 = *var_guard1706_slot;
        let mut var_ktat__blk1561: f64 = *var_ktat__blk1561_slot;
        let mut var_ktat__blk1561_db0: f64 = *var_ktat__blk1561_db0_slot;
        let mut var_ktat__blk1561_db1: f64 = *var_ktat__blk1561_db1_slot;
        let mut var_ktat__blk1561_db2: f64 = *var_ktat__blk1561_db2_slot;
        let mut var_ktat__blk1561_db3: f64 = *var_ktat__blk1561_db3_slot;
        let mut var_ktat__blk1561_db4: f64 = *var_ktat__blk1561_db4_slot;
        let mut var_ktat__blk1561_db5: f64 = *var_ktat__blk1561_db5_slot;
        let mut var_ktat__blk1561_db6: f64 = *var_ktat__blk1561_db6_slot;
        let mut var_ktat__blk1561_dn0: f64 = *var_ktat__blk1561_dn0_slot;
        let mut var_ktat__blk1561_dn1: f64 = *var_ktat__blk1561_dn1_slot;
        let mut var_ktat__blk1561_dn10: f64 = *var_ktat__blk1561_dn10_slot;
        let mut var_ktat__blk1561_dn11: f64 = *var_ktat__blk1561_dn11_slot;
        let mut var_ktat__blk1561_dn2: f64 = *var_ktat__blk1561_dn2_slot;
        let mut var_ktat__blk1561_dn3: f64 = *var_ktat__blk1561_dn3_slot;
        let mut var_ktat__blk1561_dn4: f64 = *var_ktat__blk1561_dn4_slot;
        let mut var_ktat__blk1561_dn5: f64 = *var_ktat__blk1561_dn5_slot;
        let mut var_ktat__blk1561_dn6: f64 = *var_ktat__blk1561_dn6_slot;
        let mut var_ktat__blk1561_dn7: f64 = *var_ktat__blk1561_dn7_slot;
        let mut var_ktat__blk1561_dn8: f64 = *var_ktat__blk1561_dn8_slot;
        let mut var_ktat__blk1561_dn9: f64 = *var_ktat__blk1561_dn9_slot;
        let mut var_ltat__blk1562: f64 = *var_ltat__blk1562_slot;
        let mut var_ltat__blk1562_db0: f64 = *var_ltat__blk1562_db0_slot;
        let mut var_ltat__blk1562_db1: f64 = *var_ltat__blk1562_db1_slot;
        let mut var_ltat__blk1562_db2: f64 = *var_ltat__blk1562_db2_slot;
        let mut var_ltat__blk1562_db3: f64 = *var_ltat__blk1562_db3_slot;
        let mut var_ltat__blk1562_db4: f64 = *var_ltat__blk1562_db4_slot;
        let mut var_ltat__blk1562_db5: f64 = *var_ltat__blk1562_db5_slot;
        let mut var_ltat__blk1562_db6: f64 = *var_ltat__blk1562_db6_slot;
        let mut var_ltat__blk1562_dn0: f64 = *var_ltat__blk1562_dn0_slot;
        let mut var_ltat__blk1562_dn1: f64 = *var_ltat__blk1562_dn1_slot;
        let mut var_ltat__blk1562_dn10: f64 = *var_ltat__blk1562_dn10_slot;
        let mut var_ltat__blk1562_dn11: f64 = *var_ltat__blk1562_dn11_slot;
        let mut var_ltat__blk1562_dn2: f64 = *var_ltat__blk1562_dn2_slot;
        let mut var_ltat__blk1562_dn3: f64 = *var_ltat__blk1562_dn3_slot;
        let mut var_ltat__blk1562_dn4: f64 = *var_ltat__blk1562_dn4_slot;
        let mut var_ltat__blk1562_dn5: f64 = *var_ltat__blk1562_dn5_slot;
        let mut var_ltat__blk1562_dn6: f64 = *var_ltat__blk1562_dn6_slot;
        let mut var_ltat__blk1562_dn7: f64 = *var_ltat__blk1562_dn7_slot;
        let mut var_ltat__blk1562_dn8: f64 = *var_ltat__blk1562_dn8_slot;
        let mut var_ltat__blk1562_dn9: f64 = *var_ltat__blk1562_dn9_slot;
        let mut var_mtat__blk1563: f64 = *var_mtat__blk1563_slot;
        let mut var_mtat__blk1563_db0: f64 = *var_mtat__blk1563_db0_slot;
        let mut var_mtat__blk1563_db1: f64 = *var_mtat__blk1563_db1_slot;
        let mut var_mtat__blk1563_db2: f64 = *var_mtat__blk1563_db2_slot;
        let mut var_mtat__blk1563_db3: f64 = *var_mtat__blk1563_db3_slot;
        let mut var_mtat__blk1563_db4: f64 = *var_mtat__blk1563_db4_slot;
        let mut var_mtat__blk1563_db5: f64 = *var_mtat__blk1563_db5_slot;
        let mut var_mtat__blk1563_db6: f64 = *var_mtat__blk1563_db6_slot;
        let mut var_mtat__blk1563_dn0: f64 = *var_mtat__blk1563_dn0_slot;
        let mut var_mtat__blk1563_dn1: f64 = *var_mtat__blk1563_dn1_slot;
        let mut var_mtat__blk1563_dn10: f64 = *var_mtat__blk1563_dn10_slot;
        let mut var_mtat__blk1563_dn11: f64 = *var_mtat__blk1563_dn11_slot;
        let mut var_mtat__blk1563_dn2: f64 = *var_mtat__blk1563_dn2_slot;
        let mut var_mtat__blk1563_dn3: f64 = *var_mtat__blk1563_dn3_slot;
        let mut var_mtat__blk1563_dn4: f64 = *var_mtat__blk1563_dn4_slot;
        let mut var_mtat__blk1563_dn5: f64 = *var_mtat__blk1563_dn5_slot;
        let mut var_mtat__blk1563_dn6: f64 = *var_mtat__blk1563_dn6_slot;
        let mut var_mtat__blk1563_dn7: f64 = *var_mtat__blk1563_dn7_slot;
        let mut var_mtat__blk1563_dn8: f64 = *var_mtat__blk1563_dn8_slot;
        let mut var_mtat__blk1563_dn9: f64 = *var_mtat__blk1563_dn9_slot;
        let mut var_terfc__blk1526: f64 = *var_terfc__blk1526_slot;
        let mut var_terfc__blk1526_db0: f64 = *var_terfc__blk1526_db0_slot;
        let mut var_terfc__blk1526_db1: f64 = *var_terfc__blk1526_db1_slot;
        let mut var_terfc__blk1526_db2: f64 = *var_terfc__blk1526_db2_slot;
        let mut var_terfc__blk1526_db3: f64 = *var_terfc__blk1526_db3_slot;
        let mut var_terfc__blk1526_db4: f64 = *var_terfc__blk1526_db4_slot;
        let mut var_terfc__blk1526_db5: f64 = *var_terfc__blk1526_db5_slot;
        let mut var_terfc__blk1526_db6: f64 = *var_terfc__blk1526_db6_slot;
        let mut var_terfc__blk1526_dn0: f64 = *var_terfc__blk1526_dn0_slot;
        let mut var_terfc__blk1526_dn1: f64 = *var_terfc__blk1526_dn1_slot;
        let mut var_terfc__blk1526_dn10: f64 = *var_terfc__blk1526_dn10_slot;
        let mut var_terfc__blk1526_dn11: f64 = *var_terfc__blk1526_dn11_slot;
        let mut var_terfc__blk1526_dn2: f64 = *var_terfc__blk1526_dn2_slot;
        let mut var_terfc__blk1526_dn3: f64 = *var_terfc__blk1526_dn3_slot;
        let mut var_terfc__blk1526_dn4: f64 = *var_terfc__blk1526_dn4_slot;
        let mut var_terfc__blk1526_dn5: f64 = *var_terfc__blk1526_dn5_slot;
        let mut var_terfc__blk1526_dn6: f64 = *var_terfc__blk1526_dn6_slot;
        let mut var_terfc__blk1526_dn7: f64 = *var_terfc__blk1526_dn7_slot;
        let mut var_terfc__blk1526_dn8: f64 = *var_terfc__blk1526_dn8_slot;
        let mut var_terfc__blk1526_dn9: f64 = *var_terfc__blk1526_dn9_slot;
        let mut var_tmp__blk1543: f64 = *var_tmp__blk1543_slot;
        let mut var_tmp__blk1543_db0: f64 = *var_tmp__blk1543_db0_slot;
        let mut var_tmp__blk1543_db1: f64 = *var_tmp__blk1543_db1_slot;
        let mut var_tmp__blk1543_db2: f64 = *var_tmp__blk1543_db2_slot;
        let mut var_tmp__blk1543_db3: f64 = *var_tmp__blk1543_db3_slot;
        let mut var_tmp__blk1543_db4: f64 = *var_tmp__blk1543_db4_slot;
        let mut var_tmp__blk1543_db5: f64 = *var_tmp__blk1543_db5_slot;
        let mut var_tmp__blk1543_db6: f64 = *var_tmp__blk1543_db6_slot;
        let mut var_tmp__blk1543_dn0: f64 = *var_tmp__blk1543_dn0_slot;
        let mut var_tmp__blk1543_dn1: f64 = *var_tmp__blk1543_dn1_slot;
        let mut var_tmp__blk1543_dn10: f64 = *var_tmp__blk1543_dn10_slot;
        let mut var_tmp__blk1543_dn11: f64 = *var_tmp__blk1543_dn11_slot;
        let mut var_tmp__blk1543_dn2: f64 = *var_tmp__blk1543_dn2_slot;
        let mut var_tmp__blk1543_dn3: f64 = *var_tmp__blk1543_dn3_slot;
        let mut var_tmp__blk1543_dn4: f64 = *var_tmp__blk1543_dn4_slot;
        let mut var_tmp__blk1543_dn5: f64 = *var_tmp__blk1543_dn5_slot;
        let mut var_tmp__blk1543_dn6: f64 = *var_tmp__blk1543_dn6_slot;
        let mut var_tmp__blk1543_dn7: f64 = *var_tmp__blk1543_dn7_slot;
        let mut var_tmp__blk1543_dn8: f64 = *var_tmp__blk1543_dn8_slot;
        let mut var_tmp__blk1543_dn9: f64 = *var_tmp__blk1543_dn9_slot;
        let mut var_umaxpoweronepointfive__blk1558: f64 = *var_umaxpoweronepointfive__blk1558_slot;
        let mut var_umaxpoweronepointfive__blk1558_db0: f64 = *var_umaxpoweronepointfive__blk1558_db0_slot;
        let mut var_umaxpoweronepointfive__blk1558_db1: f64 = *var_umaxpoweronepointfive__blk1558_db1_slot;
        let mut var_umaxpoweronepointfive__blk1558_db2: f64 = *var_umaxpoweronepointfive__blk1558_db2_slot;
        let mut var_umaxpoweronepointfive__blk1558_db3: f64 = *var_umaxpoweronepointfive__blk1558_db3_slot;
        let mut var_umaxpoweronepointfive__blk1558_db4: f64 = *var_umaxpoweronepointfive__blk1558_db4_slot;
        let mut var_umaxpoweronepointfive__blk1558_db5: f64 = *var_umaxpoweronepointfive__blk1558_db5_slot;
        let mut var_umaxpoweronepointfive__blk1558_db6: f64 = *var_umaxpoweronepointfive__blk1558_db6_slot;
        let mut var_umaxpoweronepointfive__blk1558_dn0: f64 = *var_umaxpoweronepointfive__blk1558_dn0_slot;
        let mut var_umaxpoweronepointfive__blk1558_dn1: f64 = *var_umaxpoweronepointfive__blk1558_dn1_slot;
        let mut var_umaxpoweronepointfive__blk1558_dn10: f64 = *var_umaxpoweronepointfive__blk1558_dn10_slot;
        let mut var_umaxpoweronepointfive__blk1558_dn11: f64 = *var_umaxpoweronepointfive__blk1558_dn11_slot;
        let mut var_umaxpoweronepointfive__blk1558_dn2: f64 = *var_umaxpoweronepointfive__blk1558_dn2_slot;
        let mut var_umaxpoweronepointfive__blk1558_dn3: f64 = *var_umaxpoweronepointfive__blk1558_dn3_slot;
        let mut var_umaxpoweronepointfive__blk1558_dn4: f64 = *var_umaxpoweronepointfive__blk1558_dn4_slot;
        let mut var_umaxpoweronepointfive__blk1558_dn5: f64 = *var_umaxpoweronepointfive__blk1558_dn5_slot;
        let mut var_umaxpoweronepointfive__blk1558_dn6: f64 = *var_umaxpoweronepointfive__blk1558_dn6_slot;
        let mut var_umaxpoweronepointfive__blk1558_dn7: f64 = *var_umaxpoweronepointfive__blk1558_dn7_slot;
        let mut var_umaxpoweronepointfive__blk1558_dn8: f64 = *var_umaxpoweronepointfive__blk1558_dn8_slot;
        let mut var_umaxpoweronepointfive__blk1558_dn9: f64 = *var_umaxpoweronepointfive__blk1558_dn9_slot;
        let mut var_wgamma__blk1559: f64 = *var_wgamma__blk1559_slot;
        let mut var_wgamma__blk1559_db0: f64 = *var_wgamma__blk1559_db0_slot;
        let mut var_wgamma__blk1559_db1: f64 = *var_wgamma__blk1559_db1_slot;
        let mut var_wgamma__blk1559_db2: f64 = *var_wgamma__blk1559_db2_slot;
        let mut var_wgamma__blk1559_db3: f64 = *var_wgamma__blk1559_db3_slot;
        let mut var_wgamma__blk1559_db4: f64 = *var_wgamma__blk1559_db4_slot;
        let mut var_wgamma__blk1559_db5: f64 = *var_wgamma__blk1559_db5_slot;
        let mut var_wgamma__blk1559_db6: f64 = *var_wgamma__blk1559_db6_slot;
        let mut var_wgamma__blk1559_dn0: f64 = *var_wgamma__blk1559_dn0_slot;
        let mut var_wgamma__blk1559_dn1: f64 = *var_wgamma__blk1559_dn1_slot;
        let mut var_wgamma__blk1559_dn10: f64 = *var_wgamma__blk1559_dn10_slot;
        let mut var_wgamma__blk1559_dn11: f64 = *var_wgamma__blk1559_dn11_slot;
        let mut var_wgamma__blk1559_dn2: f64 = *var_wgamma__blk1559_dn2_slot;
        let mut var_wgamma__blk1559_dn3: f64 = *var_wgamma__blk1559_dn3_slot;
        let mut var_wgamma__blk1559_dn4: f64 = *var_wgamma__blk1559_dn4_slot;
        let mut var_wgamma__blk1559_dn5: f64 = *var_wgamma__blk1559_dn5_slot;
        let mut var_wgamma__blk1559_dn6: f64 = *var_wgamma__blk1559_dn6_slot;
        let mut var_wgamma__blk1559_dn7: f64 = *var_wgamma__blk1559_dn7_slot;
        let mut var_wgamma__blk1559_dn8: f64 = *var_wgamma__blk1559_dn8_slot;
        let mut var_wgamma__blk1559_dn9: f64 = *var_wgamma__blk1559_dn9_slot;
        let mut var_wtat__blk1560: f64 = *var_wtat__blk1560_slot;
        let mut var_wtat__blk1560_db0: f64 = *var_wtat__blk1560_db0_slot;
        let mut var_wtat__blk1560_db1: f64 = *var_wtat__blk1560_db1_slot;
        let mut var_wtat__blk1560_db2: f64 = *var_wtat__blk1560_db2_slot;
        let mut var_wtat__blk1560_db3: f64 = *var_wtat__blk1560_db3_slot;
        let mut var_wtat__blk1560_db4: f64 = *var_wtat__blk1560_db4_slot;
        let mut var_wtat__blk1560_db5: f64 = *var_wtat__blk1560_db5_slot;
        let mut var_wtat__blk1560_db6: f64 = *var_wtat__blk1560_db6_slot;
        let mut var_wtat__blk1560_dn0: f64 = *var_wtat__blk1560_dn0_slot;
        let mut var_wtat__blk1560_dn1: f64 = *var_wtat__blk1560_dn1_slot;
        let mut var_wtat__blk1560_dn10: f64 = *var_wtat__blk1560_dn10_slot;
        let mut var_wtat__blk1560_dn11: f64 = *var_wtat__blk1560_dn11_slot;
        let mut var_wtat__blk1560_dn2: f64 = *var_wtat__blk1560_dn2_slot;
        let mut var_wtat__blk1560_dn3: f64 = *var_wtat__blk1560_dn3_slot;
        let mut var_wtat__blk1560_dn4: f64 = *var_wtat__blk1560_dn4_slot;
        let mut var_wtat__blk1560_dn5: f64 = *var_wtat__blk1560_dn5_slot;
        let mut var_wtat__blk1560_dn6: f64 = *var_wtat__blk1560_dn6_slot;
        let mut var_wtat__blk1560_dn7: f64 = *var_wtat__blk1560_dn7_slot;
        let mut var_wtat__blk1560_dn8: f64 = *var_wtat__blk1560_dn8_slot;
        let mut var_wtat__blk1560_dn9: f64 = *var_wtat__blk1560_dn9_slot;
        let mut var_xerfc__blk1564: f64 = *var_xerfc__blk1564_slot;
        let mut var_xerfc__blk1564_db0: f64 = *var_xerfc__blk1564_db0_slot;
        let mut var_xerfc__blk1564_db1: f64 = *var_xerfc__blk1564_db1_slot;
        let mut var_xerfc__blk1564_db2: f64 = *var_xerfc__blk1564_db2_slot;
        let mut var_xerfc__blk1564_db3: f64 = *var_xerfc__blk1564_db3_slot;
        let mut var_xerfc__blk1564_db4: f64 = *var_xerfc__blk1564_db4_slot;
        let mut var_xerfc__blk1564_db5: f64 = *var_xerfc__blk1564_db5_slot;
        let mut var_xerfc__blk1564_db6: f64 = *var_xerfc__blk1564_db6_slot;
        let mut var_xerfc__blk1564_dn0: f64 = *var_xerfc__blk1564_dn0_slot;
        let mut var_xerfc__blk1564_dn1: f64 = *var_xerfc__blk1564_dn1_slot;
        let mut var_xerfc__blk1564_dn10: f64 = *var_xerfc__blk1564_dn10_slot;
        let mut var_xerfc__blk1564_dn11: f64 = *var_xerfc__blk1564_dn11_slot;
        let mut var_xerfc__blk1564_dn2: f64 = *var_xerfc__blk1564_dn2_slot;
        let mut var_xerfc__blk1564_dn3: f64 = *var_xerfc__blk1564_dn3_slot;
        let mut var_xerfc__blk1564_dn4: f64 = *var_xerfc__blk1564_dn4_slot;
        let mut var_xerfc__blk1564_dn5: f64 = *var_xerfc__blk1564_dn5_slot;
        let mut var_xerfc__blk1564_dn6: f64 = *var_xerfc__blk1564_dn6_slot;
        let mut var_xerfc__blk1564_dn7: f64 = *var_xerfc__blk1564_dn7_slot;
        let mut var_xerfc__blk1564_dn8: f64 = *var_xerfc__blk1564_dn8_slot;
        let mut var_xerfc__blk1564_dn9: f64 = *var_xerfc__blk1564_dn9_slot;
        let mut var_ysq__blk1525: f64 = *var_ysq__blk1525_slot;
        let mut var_ysq__blk1525_db0: f64 = *var_ysq__blk1525_db0_slot;
        let mut var_ysq__blk1525_db1: f64 = *var_ysq__blk1525_db1_slot;
        let mut var_ysq__blk1525_db2: f64 = *var_ysq__blk1525_db2_slot;
        let mut var_ysq__blk1525_db3: f64 = *var_ysq__blk1525_db3_slot;
        let mut var_ysq__blk1525_db4: f64 = *var_ysq__blk1525_db4_slot;
        let mut var_ysq__blk1525_db5: f64 = *var_ysq__blk1525_db5_slot;
        let mut var_ysq__blk1525_db6: f64 = *var_ysq__blk1525_db6_slot;
        let mut var_ysq__blk1525_dn0: f64 = *var_ysq__blk1525_dn0_slot;
        let mut var_ysq__blk1525_dn1: f64 = *var_ysq__blk1525_dn1_slot;
        let mut var_ysq__blk1525_dn10: f64 = *var_ysq__blk1525_dn10_slot;
        let mut var_ysq__blk1525_dn11: f64 = *var_ysq__blk1525_dn11_slot;
        let mut var_ysq__blk1525_dn2: f64 = *var_ysq__blk1525_dn2_slot;
        let mut var_ysq__blk1525_dn3: f64 = *var_ysq__blk1525_dn3_slot;
        let mut var_ysq__blk1525_dn4: f64 = *var_ysq__blk1525_dn4_slot;
        let mut var_ysq__blk1525_dn5: f64 = *var_ysq__blk1525_dn5_slot;
        let mut var_ysq__blk1525_dn6: f64 = *var_ysq__blk1525_dn6_slot;
        let mut var_ysq__blk1525_dn7: f64 = *var_ysq__blk1525_dn7_slot;
        let mut var_ysq__blk1525_dn8: f64 = *var_ysq__blk1525_dn8_slot;
        let mut var_ysq__blk1525_dn9: f64 = *var_ysq__blk1525_dn9_slot;

        let (assign61050_e79106, assign61050_e79106_d_n0, assign61050_e79106_d_n1, assign61050_e79106_d_n2, assign61050_e79106_d_n3, assign61050_e79106_d_n4, assign61050_e79106_d_n5, assign61050_e79106_d_n6, assign61050_e79106_d_n7, assign61050_e79106_d_n8, assign61050_e79106_d_n9, assign61050_e79106_d_n10, assign61050_e79106_d_n11, assign61050_e79106_d_b0, assign61050_e79106_d_b1, assign61050_e79106_d_b2, assign61050_e79106_d_b3, assign61050_e79106_d_b4, assign61050_e79106_d_b5, assign61050_e79106_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) {
        let assign61050_e79104: f64 = (var_umax__blk1556 * var_sqrtumax__blk1557);
        (assign61050_e79104, ((var_umax__blk1556_dn0 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_dn0)), ((var_umax__blk1556_dn1 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_dn1)), ((var_umax__blk1556_dn2 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_dn2)), ((var_umax__blk1556_dn3 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_dn3)), ((var_umax__blk1556_dn4 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_dn4)), ((var_umax__blk1556_dn5 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_dn5)), ((var_umax__blk1556_dn6 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_dn6)), ((var_umax__blk1556_dn7 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_dn7)), ((var_umax__blk1556_dn8 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_dn8)), ((var_umax__blk1556_dn9 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_dn9)), ((var_umax__blk1556_dn10 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_dn10)), ((var_umax__blk1556_dn11 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_dn11)), ((var_umax__blk1556_db0 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_db0)), ((var_umax__blk1556_db1 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_db1)), ((var_umax__blk1556_db2 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_db2)), ((var_umax__blk1556_db3 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_db3)), ((var_umax__blk1556_db4 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_db4)), ((var_umax__blk1556_db5 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_db5)), ((var_umax__blk1556_db6 * var_sqrtumax__blk1557) + (var_umax__blk1556 * var_sqrtumax__blk1557_db6)),)
    } else {
        (var_umaxpoweronepointfive__blk1558, var_umaxpoweronepointfive__blk1558_dn0, var_umaxpoweronepointfive__blk1558_dn1, var_umaxpoweronepointfive__blk1558_dn2, var_umaxpoweronepointfive__blk1558_dn3, var_umaxpoweronepointfive__blk1558_dn4, var_umaxpoweronepointfive__blk1558_dn5, var_umaxpoweronepointfive__blk1558_dn6, var_umaxpoweronepointfive__blk1558_dn7, var_umaxpoweronepointfive__blk1558_dn8, var_umaxpoweronepointfive__blk1558_dn9, var_umaxpoweronepointfive__blk1558_dn10, var_umaxpoweronepointfive__blk1558_dn11, var_umaxpoweronepointfive__blk1558_db0, var_umaxpoweronepointfive__blk1558_db1, var_umaxpoweronepointfive__blk1558_db2, var_umaxpoweronepointfive__blk1558_db3, var_umaxpoweronepointfive__blk1558_db4, var_umaxpoweronepointfive__blk1558_db5, var_umaxpoweronepointfive__blk1558_db6,)
    }
};
        var_umaxpoweronepointfive__blk1558 = assign61050_e79106;
        var_umaxpoweronepointfive__blk1558_dn0 = assign61050_e79106_d_n0;
        var_umaxpoweronepointfive__blk1558_dn1 = assign61050_e79106_d_n1;
        var_umaxpoweronepointfive__blk1558_dn2 = assign61050_e79106_d_n2;
        var_umaxpoweronepointfive__blk1558_dn3 = assign61050_e79106_d_n3;
        var_umaxpoweronepointfive__blk1558_dn4 = assign61050_e79106_d_n4;
        var_umaxpoweronepointfive__blk1558_dn5 = assign61050_e79106_d_n5;
        var_umaxpoweronepointfive__blk1558_dn6 = assign61050_e79106_d_n6;
        var_umaxpoweronepointfive__blk1558_dn7 = assign61050_e79106_d_n7;
        var_umaxpoweronepointfive__blk1558_dn8 = assign61050_e79106_d_n8;
        var_umaxpoweronepointfive__blk1558_dn9 = assign61050_e79106_d_n9;
        var_umaxpoweronepointfive__blk1558_dn10 = assign61050_e79106_d_n10;
        var_umaxpoweronepointfive__blk1558_dn11 = assign61050_e79106_d_n11;
        var_umaxpoweronepointfive__blk1558_db0 = assign61050_e79106_d_b0;
        var_umaxpoweronepointfive__blk1558_db1 = assign61050_e79106_d_b1;
        var_umaxpoweronepointfive__blk1558_db2 = assign61050_e79106_d_b2;
        var_umaxpoweronepointfive__blk1558_db3 = assign61050_e79106_d_b3;
        var_umaxpoweronepointfive__blk1558_db4 = assign61050_e79106_d_b4;
        var_umaxpoweronepointfive__blk1558_db5 = assign61050_e79106_d_b5;
        var_umaxpoweronepointfive__blk1558_db6 = assign61050_e79106_d_b6;

        let assign61060_e79108: f64 = (-var_pgatd_i);
        let assign61060_e79110: f64 = (assign61060_e79108 * var_one_over_one_minus_pgat_d);
        let assign61060_e79112: f64 = (-1.0);
        let assign61060_e79113: f64 = if assign61060_e79110 == assign61060_e79112 { 1.0 } else { 0.0 };
        var_guard1702 = assign61060_e79113;

        let (assign61070_e79134, assign61070_e79134_d_n0, assign61070_e79134_d_n1, assign61070_e79134_d_n2, assign61070_e79134_d_n3, assign61070_e79134_d_n4, assign61070_e79134_d_n5, assign61070_e79134_d_n6, assign61070_e79134_d_n7, assign61070_e79134_d_n8, assign61070_e79134_d_n9, assign61070_e79134_d_n10, assign61070_e79134_d_n11, assign61070_e79134_d_b0, assign61070_e79134_d_b1, assign61070_e79134_d_b2, assign61070_e79134_d_b3, assign61070_e79134_d_b4, assign61070_e79134_d_b5, assign61070_e79134_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) && (var_guard1702 != 0.0)) {
        let assign61070_e79130: f64 = (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558);
        let assign61070_e79131: f64 = (1.0 + assign61070_e79130);
        let assign61070_e79132: f64 = (1.0 / assign61070_e79131);
        (assign61070_e79132, (-(((var_btat__blk1553_dn0 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn0)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_dn1 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn1)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_dn2 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn2)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_dn3 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn3)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_dn4 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn4)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_dn5 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn5)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_dn6 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn6)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_dn7 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn7)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_dn8 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn8)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_dn9 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn9)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_dn10 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn10)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_dn11 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn11)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_db0 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db0)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_db1 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db1)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_db2 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db2)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_db3 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db3)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_db4 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db4)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_db5 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db5)) / (assign61070_e79131 * assign61070_e79131))), (-(((var_btat__blk1553_db6 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db6)) / (assign61070_e79131 * assign61070_e79131))),)
    } else {
        (var_wgamma__blk1559, var_wgamma__blk1559_dn0, var_wgamma__blk1559_dn1, var_wgamma__blk1559_dn2, var_wgamma__blk1559_dn3, var_wgamma__blk1559_dn4, var_wgamma__blk1559_dn5, var_wgamma__blk1559_dn6, var_wgamma__blk1559_dn7, var_wgamma__blk1559_dn8, var_wgamma__blk1559_dn9, var_wgamma__blk1559_dn10, var_wgamma__blk1559_dn11, var_wgamma__blk1559_db0, var_wgamma__blk1559_db1, var_wgamma__blk1559_db2, var_wgamma__blk1559_db3, var_wgamma__blk1559_db4, var_wgamma__blk1559_db5, var_wgamma__blk1559_db6,)
    }
};
        var_wgamma__blk1559 = assign61070_e79134;
        var_wgamma__blk1559_dn0 = assign61070_e79134_d_n0;
        var_wgamma__blk1559_dn1 = assign61070_e79134_d_n1;
        var_wgamma__blk1559_dn2 = assign61070_e79134_d_n2;
        var_wgamma__blk1559_dn3 = assign61070_e79134_d_n3;
        var_wgamma__blk1559_dn4 = assign61070_e79134_d_n4;
        var_wgamma__blk1559_dn5 = assign61070_e79134_d_n5;
        var_wgamma__blk1559_dn6 = assign61070_e79134_d_n6;
        var_wgamma__blk1559_dn7 = assign61070_e79134_d_n7;
        var_wgamma__blk1559_dn8 = assign61070_e79134_d_n8;
        var_wgamma__blk1559_dn9 = assign61070_e79134_d_n9;
        var_wgamma__blk1559_dn10 = assign61070_e79134_d_n10;
        var_wgamma__blk1559_dn11 = assign61070_e79134_d_n11;
        var_wgamma__blk1559_db0 = assign61070_e79134_d_b0;
        var_wgamma__blk1559_db1 = assign61070_e79134_d_b1;
        var_wgamma__blk1559_db2 = assign61070_e79134_d_b2;
        var_wgamma__blk1559_db3 = assign61070_e79134_d_b3;
        var_wgamma__blk1559_db4 = assign61070_e79134_d_b4;
        var_wgamma__blk1559_db5 = assign61070_e79134_d_b5;
        var_wgamma__blk1559_db6 = assign61070_e79134_d_b6;

        let (assign61080_e79159, assign61080_e79159_d_n0, assign61080_e79159_d_n1, assign61080_e79159_d_n2, assign61080_e79159_d_n3, assign61080_e79159_d_n4, assign61080_e79159_d_n5, assign61080_e79159_d_n6, assign61080_e79159_d_n7, assign61080_e79159_d_n8, assign61080_e79159_d_n9, assign61080_e79159_d_n10, assign61080_e79159_d_n11, assign61080_e79159_d_b0, assign61080_e79159_d_b1, assign61080_e79159_d_b2, assign61080_e79159_d_b3, assign61080_e79159_d_b4, assign61080_e79159_d_b5, assign61080_e79159_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) && (var_guard1702 == 0.0)) {
        let assign61080_e79151: f64 = (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558);
        let assign61080_e79152: f64 = (1.0 + assign61080_e79151);
        let assign61080_e79154: f64 = (-var_pgatd_i);
        let assign61080_e79156: f64 = (assign61080_e79154 * var_one_over_one_minus_pgat_d);
        let assign61080_e79157: f64 = (assign61080_e79152).powf(assign61080_e79156);
        (assign61080_e79157, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_dn0 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn0)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_dn0 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn0)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_dn1 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn1)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_dn1 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn1)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_dn2 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn2)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_dn2 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn2)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_dn3 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn3)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_dn3 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn3)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_dn4 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn4)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_dn4 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn4)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_dn5 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn5)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_dn5 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn5)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_dn6 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn6)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_dn6 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn6)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_dn7 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn7)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_dn7 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn7)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_dn8 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn8)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_dn8 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn8)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_dn9 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn9)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_dn9 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn9)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_dn10 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn10)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_dn10 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn10)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_dn11 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn11)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_dn11 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn11)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_db0 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db0)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_db0 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db0)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_db1 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db1)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_db1 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db1)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_db2 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db2)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_db2 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db2)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_db3 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db3)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_db3 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db3)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_db4 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db4)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_db4 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db4)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_db5 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db5)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_db5 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db5)) / assign61080_e79152))) }, if 0.0 == 0.0 && ((assign61080_e79156) as f64).is_finite() && ((assign61080_e79156) as f64).fract() == 0.0 { if assign61080_e79156 == 0.0 { 0.0 } else { (assign61080_e79156 * ((assign61080_e79152).powf(assign61080_e79156 - 1.0) * ((var_btat__blk1553_db6 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db6)))) } } else { (assign61080_e79157 * (assign61080_e79156 * (((var_btat__blk1553_db6 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db6)) / assign61080_e79152))) },)
    } else {
        (var_wgamma__blk1559, var_wgamma__blk1559_dn0, var_wgamma__blk1559_dn1, var_wgamma__blk1559_dn2, var_wgamma__blk1559_dn3, var_wgamma__blk1559_dn4, var_wgamma__blk1559_dn5, var_wgamma__blk1559_dn6, var_wgamma__blk1559_dn7, var_wgamma__blk1559_dn8, var_wgamma__blk1559_dn9, var_wgamma__blk1559_dn10, var_wgamma__blk1559_dn11, var_wgamma__blk1559_db0, var_wgamma__blk1559_db1, var_wgamma__blk1559_db2, var_wgamma__blk1559_db3, var_wgamma__blk1559_db4, var_wgamma__blk1559_db5, var_wgamma__blk1559_db6,)
    }
};
        var_wgamma__blk1559 = assign61080_e79159;
        var_wgamma__blk1559_dn0 = assign61080_e79159_d_n0;
        var_wgamma__blk1559_dn1 = assign61080_e79159_d_n1;
        var_wgamma__blk1559_dn2 = assign61080_e79159_d_n2;
        var_wgamma__blk1559_dn3 = assign61080_e79159_d_n3;
        var_wgamma__blk1559_dn4 = assign61080_e79159_d_n4;
        var_wgamma__blk1559_dn5 = assign61080_e79159_d_n5;
        var_wgamma__blk1559_dn6 = assign61080_e79159_d_n6;
        var_wgamma__blk1559_dn7 = assign61080_e79159_d_n7;
        var_wgamma__blk1559_dn8 = assign61080_e79159_d_n8;
        var_wgamma__blk1559_dn9 = assign61080_e79159_d_n9;
        var_wgamma__blk1559_dn10 = assign61080_e79159_d_n10;
        var_wgamma__blk1559_dn11 = assign61080_e79159_d_n11;
        var_wgamma__blk1559_db0 = assign61080_e79159_d_b0;
        var_wgamma__blk1559_db1 = assign61080_e79159_d_b1;
        var_wgamma__blk1559_db2 = assign61080_e79159_d_b2;
        var_wgamma__blk1559_db3 = assign61080_e79159_d_b3;
        var_wgamma__blk1559_db4 = assign61080_e79159_d_b4;
        var_wgamma__blk1559_db5 = assign61080_e79159_d_b5;
        var_wgamma__blk1559_db6 = assign61080_e79159_d_b6;

        let (assign61090_e79178, assign61090_e79178_d_n0, assign61090_e79178_d_n1, assign61090_e79178_d_n2, assign61090_e79178_d_n3, assign61090_e79178_d_n4, assign61090_e79178_d_n5, assign61090_e79178_d_n6, assign61090_e79178_d_n7, assign61090_e79178_d_n8, assign61090_e79178_d_n9, assign61090_e79178_d_n10, assign61090_e79178_d_n11, assign61090_e79178_d_b0, assign61090_e79178_d_b1, assign61090_e79178_d_b2, assign61090_e79178_d_b3, assign61090_e79178_d_b4, assign61090_e79178_d_b5, assign61090_e79178_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) {
        let assign61090_e79172: f64 = (var_wsrh__blk1549 * var_wgamma__blk1559);
        let assign61090_e79175: f64 = (var_wsrh__blk1549 + var_wgamma__blk1559);
        let assign61090_e79176: f64 = (assign61090_e79172 / assign61090_e79175);
        (assign61090_e79176, (((((var_wsrh__blk1549_dn0 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_dn0)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_dn0 + var_wgamma__blk1559_dn0))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_dn1 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_dn1)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_dn1 + var_wgamma__blk1559_dn1))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_dn2 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_dn2)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_dn2 + var_wgamma__blk1559_dn2))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_dn3 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_dn3)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_dn3 + var_wgamma__blk1559_dn3))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_dn4 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_dn4)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_dn4 + var_wgamma__blk1559_dn4))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_dn5 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_dn5)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_dn5 + var_wgamma__blk1559_dn5))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_dn6 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_dn6)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_dn6 + var_wgamma__blk1559_dn6))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_dn7 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_dn7)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_dn7 + var_wgamma__blk1559_dn7))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_dn8 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_dn8)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_dn8 + var_wgamma__blk1559_dn8))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_dn9 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_dn9)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_dn9 + var_wgamma__blk1559_dn9))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_dn10 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_dn10)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_dn10 + var_wgamma__blk1559_dn10))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_dn11 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_dn11)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_dn11 + var_wgamma__blk1559_dn11))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_db0 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_db0)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_db0 + var_wgamma__blk1559_db0))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_db1 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_db1)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_db1 + var_wgamma__blk1559_db1))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_db2 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_db2)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_db2 + var_wgamma__blk1559_db2))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_db3 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_db3)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_db3 + var_wgamma__blk1559_db3))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_db4 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_db4)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_db4 + var_wgamma__blk1559_db4))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_db5 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_db5)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_db5 + var_wgamma__blk1559_db5))) / (assign61090_e79175 * assign61090_e79175)), (((((var_wsrh__blk1549_db6 * var_wgamma__blk1559) + (var_wsrh__blk1549 * var_wgamma__blk1559_db6)) * assign61090_e79175) - (assign61090_e79172 * (var_wsrh__blk1549_db6 + var_wgamma__blk1559_db6))) / (assign61090_e79175 * assign61090_e79175)),)
    } else {
        (var_wtat__blk1560, var_wtat__blk1560_dn0, var_wtat__blk1560_dn1, var_wtat__blk1560_dn2, var_wtat__blk1560_dn3, var_wtat__blk1560_dn4, var_wtat__blk1560_dn5, var_wtat__blk1560_dn6, var_wtat__blk1560_dn7, var_wtat__blk1560_dn8, var_wtat__blk1560_dn9, var_wtat__blk1560_dn10, var_wtat__blk1560_dn11, var_wtat__blk1560_db0, var_wtat__blk1560_db1, var_wtat__blk1560_db2, var_wtat__blk1560_db3, var_wtat__blk1560_db4, var_wtat__blk1560_db5, var_wtat__blk1560_db6,)
    }
};
        var_wtat__blk1560 = assign61090_e79178;
        var_wtat__blk1560_dn0 = assign61090_e79178_d_n0;
        var_wtat__blk1560_dn1 = assign61090_e79178_d_n1;
        var_wtat__blk1560_dn2 = assign61090_e79178_d_n2;
        var_wtat__blk1560_dn3 = assign61090_e79178_d_n3;
        var_wtat__blk1560_dn4 = assign61090_e79178_d_n4;
        var_wtat__blk1560_dn5 = assign61090_e79178_d_n5;
        var_wtat__blk1560_dn6 = assign61090_e79178_d_n6;
        var_wtat__blk1560_dn7 = assign61090_e79178_d_n7;
        var_wtat__blk1560_dn8 = assign61090_e79178_d_n8;
        var_wtat__blk1560_dn9 = assign61090_e79178_d_n9;
        var_wtat__blk1560_dn10 = assign61090_e79178_d_n10;
        var_wtat__blk1560_dn11 = assign61090_e79178_d_n11;
        var_wtat__blk1560_db0 = assign61090_e79178_d_b0;
        var_wtat__blk1560_db1 = assign61090_e79178_d_b1;
        var_wtat__blk1560_db2 = assign61090_e79178_d_b2;
        var_wtat__blk1560_db3 = assign61090_e79178_d_b3;
        var_wtat__blk1560_db4 = assign61090_e79178_d_b4;
        var_wtat__blk1560_db5 = assign61090_e79178_d_b5;
        var_wtat__blk1560_db6 = assign61090_e79178_d_b6;

        let (assign61100_e79196, assign61100_e79196_d_n0, assign61100_e79196_d_n1, assign61100_e79196_d_n2, assign61100_e79196_d_n3, assign61100_e79196_d_n4, assign61100_e79196_d_n5, assign61100_e79196_d_n6, assign61100_e79196_d_n7, assign61100_e79196_d_n8, assign61100_e79196_d_n9, assign61100_e79196_d_n10, assign61100_e79196_d_n11, assign61100_e79196_d_b0, assign61100_e79196_d_b1, assign61100_e79196_d_b2, assign61100_e79196_d_b3, assign61100_e79196_d_b4, assign61100_e79196_d_b5, assign61100_e79196_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) {
        let assign61100_e79192: f64 = (var_btat__blk1553 / var_sqrtumax__blk1557);
        let assign61100_e79193: f64 = (0.375 * assign61100_e79192);
        let assign61100_e79194: f64 = (assign61100_e79193).sqrt();
        (assign61100_e79194, ((0.375 * (((var_btat__blk1553_dn0 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_dn0)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_dn1 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_dn1)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_dn2 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_dn2)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_dn3 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_dn3)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_dn4 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_dn4)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_dn5 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_dn5)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_dn6 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_dn6)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_dn7 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_dn7)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_dn8 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_dn8)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_dn9 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_dn9)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_dn10 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_dn10)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_dn11 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_dn11)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_db0 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_db0)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_db1 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_db1)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_db2 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_db2)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_db3 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_db3)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_db4 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_db4)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_db5 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_db5)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)), ((0.375 * (((var_btat__blk1553_db6 * var_sqrtumax__blk1557) - (var_btat__blk1553 * var_sqrtumax__blk1557_db6)) / (var_sqrtumax__blk1557 * var_sqrtumax__blk1557))) / (2.0 * assign61100_e79194)),)
    } else {
        (var_ktat__blk1561, var_ktat__blk1561_dn0, var_ktat__blk1561_dn1, var_ktat__blk1561_dn2, var_ktat__blk1561_dn3, var_ktat__blk1561_dn4, var_ktat__blk1561_dn5, var_ktat__blk1561_dn6, var_ktat__blk1561_dn7, var_ktat__blk1561_dn8, var_ktat__blk1561_dn9, var_ktat__blk1561_dn10, var_ktat__blk1561_dn11, var_ktat__blk1561_db0, var_ktat__blk1561_db1, var_ktat__blk1561_db2, var_ktat__blk1561_db3, var_ktat__blk1561_db4, var_ktat__blk1561_db5, var_ktat__blk1561_db6,)
    }
};
        var_ktat__blk1561 = assign61100_e79196;
        var_ktat__blk1561_dn0 = assign61100_e79196_d_n0;
        var_ktat__blk1561_dn1 = assign61100_e79196_d_n1;
        var_ktat__blk1561_dn2 = assign61100_e79196_d_n2;
        var_ktat__blk1561_dn3 = assign61100_e79196_d_n3;
        var_ktat__blk1561_dn4 = assign61100_e79196_d_n4;
        var_ktat__blk1561_dn5 = assign61100_e79196_d_n5;
        var_ktat__blk1561_dn6 = assign61100_e79196_d_n6;
        var_ktat__blk1561_dn7 = assign61100_e79196_d_n7;
        var_ktat__blk1561_dn8 = assign61100_e79196_d_n8;
        var_ktat__blk1561_dn9 = assign61100_e79196_d_n9;
        var_ktat__blk1561_dn10 = assign61100_e79196_d_n10;
        var_ktat__blk1561_dn11 = assign61100_e79196_d_n11;
        var_ktat__blk1561_db0 = assign61100_e79196_d_b0;
        var_ktat__blk1561_db1 = assign61100_e79196_d_b1;
        var_ktat__blk1561_db2 = assign61100_e79196_d_b2;
        var_ktat__blk1561_db3 = assign61100_e79196_d_b3;
        var_ktat__blk1561_db4 = assign61100_e79196_d_b4;
        var_ktat__blk1561_db5 = assign61100_e79196_d_b5;
        var_ktat__blk1561_db6 = assign61100_e79196_d_b6;

        let (assign61110_e79215, assign61110_e79215_d_n0, assign61110_e79215_d_n1, assign61110_e79215_d_n2, assign61110_e79215_d_n3, assign61110_e79215_d_n4, assign61110_e79215_d_n5, assign61110_e79215_d_n6, assign61110_e79215_d_n7, assign61110_e79215_d_n8, assign61110_e79215_d_n9, assign61110_e79215_d_n10, assign61110_e79215_d_n11, assign61110_e79215_d_b0, assign61110_e79215_d_b1, assign61110_e79215_d_b2, assign61110_e79215_d_b3, assign61110_e79215_d_b4, assign61110_e79215_d_b5, assign61110_e79215_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) {
        let assign61110_e79210: f64 = (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557);
        let assign61110_e79211: f64 = (2.0 * assign61110_e79210);
        let assign61110_e79213: f64 = (assign61110_e79211 - var_umax__blk1556);
        (assign61110_e79213, ((2.0 * ((var_twoatatoverthreebtat__blk1554_dn0 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_dn0))) - var_umax__blk1556_dn0), ((2.0 * ((var_twoatatoverthreebtat__blk1554_dn1 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_dn1))) - var_umax__blk1556_dn1), ((2.0 * ((var_twoatatoverthreebtat__blk1554_dn2 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_dn2))) - var_umax__blk1556_dn2), ((2.0 * ((var_twoatatoverthreebtat__blk1554_dn3 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_dn3))) - var_umax__blk1556_dn3), ((2.0 * ((var_twoatatoverthreebtat__blk1554_dn4 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_dn4))) - var_umax__blk1556_dn4), ((2.0 * ((var_twoatatoverthreebtat__blk1554_dn5 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_dn5))) - var_umax__blk1556_dn5), ((2.0 * ((var_twoatatoverthreebtat__blk1554_dn6 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_dn6))) - var_umax__blk1556_dn6), ((2.0 * ((var_twoatatoverthreebtat__blk1554_dn7 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_dn7))) - var_umax__blk1556_dn7), ((2.0 * ((var_twoatatoverthreebtat__blk1554_dn8 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_dn8))) - var_umax__blk1556_dn8), ((2.0 * ((var_twoatatoverthreebtat__blk1554_dn9 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_dn9))) - var_umax__blk1556_dn9), ((2.0 * ((var_twoatatoverthreebtat__blk1554_dn10 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_dn10))) - var_umax__blk1556_dn10), ((2.0 * ((var_twoatatoverthreebtat__blk1554_dn11 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_dn11))) - var_umax__blk1556_dn11), ((2.0 * ((var_twoatatoverthreebtat__blk1554_db0 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_db0))) - var_umax__blk1556_db0), ((2.0 * ((var_twoatatoverthreebtat__blk1554_db1 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_db1))) - var_umax__blk1556_db1), ((2.0 * ((var_twoatatoverthreebtat__blk1554_db2 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_db2))) - var_umax__blk1556_db2), ((2.0 * ((var_twoatatoverthreebtat__blk1554_db3 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_db3))) - var_umax__blk1556_db3), ((2.0 * ((var_twoatatoverthreebtat__blk1554_db4 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_db4))) - var_umax__blk1556_db4), ((2.0 * ((var_twoatatoverthreebtat__blk1554_db5 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_db5))) - var_umax__blk1556_db5), ((2.0 * ((var_twoatatoverthreebtat__blk1554_db6 * var_sqrtumax__blk1557) + (var_twoatatoverthreebtat__blk1554 * var_sqrtumax__blk1557_db6))) - var_umax__blk1556_db6),)
    } else {
        (var_ltat__blk1562, var_ltat__blk1562_dn0, var_ltat__blk1562_dn1, var_ltat__blk1562_dn2, var_ltat__blk1562_dn3, var_ltat__blk1562_dn4, var_ltat__blk1562_dn5, var_ltat__blk1562_dn6, var_ltat__blk1562_dn7, var_ltat__blk1562_dn8, var_ltat__blk1562_dn9, var_ltat__blk1562_dn10, var_ltat__blk1562_dn11, var_ltat__blk1562_db0, var_ltat__blk1562_db1, var_ltat__blk1562_db2, var_ltat__blk1562_db3, var_ltat__blk1562_db4, var_ltat__blk1562_db5, var_ltat__blk1562_db6,)
    }
};
        var_ltat__blk1562 = assign61110_e79215;
        var_ltat__blk1562_dn0 = assign61110_e79215_d_n0;
        var_ltat__blk1562_dn1 = assign61110_e79215_d_n1;
        var_ltat__blk1562_dn2 = assign61110_e79215_d_n2;
        var_ltat__blk1562_dn3 = assign61110_e79215_d_n3;
        var_ltat__blk1562_dn4 = assign61110_e79215_d_n4;
        var_ltat__blk1562_dn5 = assign61110_e79215_d_n5;
        var_ltat__blk1562_dn6 = assign61110_e79215_d_n6;
        var_ltat__blk1562_dn7 = assign61110_e79215_d_n7;
        var_ltat__blk1562_dn8 = assign61110_e79215_d_n8;
        var_ltat__blk1562_dn9 = assign61110_e79215_d_n9;
        var_ltat__blk1562_dn10 = assign61110_e79215_d_n10;
        var_ltat__blk1562_dn11 = assign61110_e79215_d_n11;
        var_ltat__blk1562_db0 = assign61110_e79215_d_b0;
        var_ltat__blk1562_db1 = assign61110_e79215_d_b1;
        var_ltat__blk1562_db2 = assign61110_e79215_d_b2;
        var_ltat__blk1562_db3 = assign61110_e79215_d_b3;
        var_ltat__blk1562_db4 = assign61110_e79215_d_b4;
        var_ltat__blk1562_db5 = assign61110_e79215_d_b5;
        var_ltat__blk1562_db6 = assign61110_e79215_d_b6;

        let (assign61120_e79242, assign61120_e79242_d_n0, assign61120_e79242_d_n1, assign61120_e79242_d_n2, assign61120_e79242_d_n3, assign61120_e79242_d_n4, assign61120_e79242_d_n5, assign61120_e79242_d_n6, assign61120_e79242_d_n7, assign61120_e79242_d_n8, assign61120_e79242_d_n9, assign61120_e79242_d_n10, assign61120_e79242_d_n11, assign61120_e79242_d_b0, assign61120_e79242_d_b1, assign61120_e79242_d_b2, assign61120_e79242_d_b3, assign61120_e79242_d_b4, assign61120_e79242_d_b5, assign61120_e79242_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) {
        let assign61120_e79228: f64 = (var_atatgat_d * var_twoatatoverthreebtat__blk1554);
        let assign61120_e79230: f64 = (assign61120_e79228 * var_sqrtumax__blk1557);
        let assign61120_e79233: f64 = (var_atatgat_d * var_umax__blk1556);
        let assign61120_e79234: f64 = (assign61120_e79230 - assign61120_e79233);
        let assign61120_e79238: f64 = (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558);
        let assign61120_e79239: f64 = (0.5 * assign61120_e79238);
        let assign61120_e79240: f64 = (assign61120_e79234 + assign61120_e79239);
        (assign61120_e79240, (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_dn0) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_dn0)) - (var_atatgat_d * var_umax__blk1556_dn0)) + (0.5 * ((var_btat__blk1553_dn0 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn0)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_dn1) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_dn1)) - (var_atatgat_d * var_umax__blk1556_dn1)) + (0.5 * ((var_btat__blk1553_dn1 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn1)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_dn2) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_dn2)) - (var_atatgat_d * var_umax__blk1556_dn2)) + (0.5 * ((var_btat__blk1553_dn2 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn2)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_dn3) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_dn3)) - (var_atatgat_d * var_umax__blk1556_dn3)) + (0.5 * ((var_btat__blk1553_dn3 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn3)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_dn4) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_dn4)) - (var_atatgat_d * var_umax__blk1556_dn4)) + (0.5 * ((var_btat__blk1553_dn4 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn4)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_dn5) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_dn5)) - (var_atatgat_d * var_umax__blk1556_dn5)) + (0.5 * ((var_btat__blk1553_dn5 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn5)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_dn6) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_dn6)) - (var_atatgat_d * var_umax__blk1556_dn6)) + (0.5 * ((var_btat__blk1553_dn6 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_dn7) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_dn7)) - (var_atatgat_d * var_umax__blk1556_dn7)) + (0.5 * ((var_btat__blk1553_dn7 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_dn8) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_dn8)) - (var_atatgat_d * var_umax__blk1556_dn8)) + (0.5 * ((var_btat__blk1553_dn8 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn8)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_dn9) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_dn9)) - (var_atatgat_d * var_umax__blk1556_dn9)) + (0.5 * ((var_btat__blk1553_dn9 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn9)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_dn10) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_dn10)) - (var_atatgat_d * var_umax__blk1556_dn10)) + (0.5 * ((var_btat__blk1553_dn10 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn10)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_dn11) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_dn11)) - (var_atatgat_d * var_umax__blk1556_dn11)) + (0.5 * ((var_btat__blk1553_dn11 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_dn11)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_db0) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_db0)) - (var_atatgat_d * var_umax__blk1556_db0)) + (0.5 * ((var_btat__blk1553_db0 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db0)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_db1) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_db1)) - (var_atatgat_d * var_umax__blk1556_db1)) + (0.5 * ((var_btat__blk1553_db1 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db1)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_db2) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_db2)) - (var_atatgat_d * var_umax__blk1556_db2)) + (0.5 * ((var_btat__blk1553_db2 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db2)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_db3) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_db3)) - (var_atatgat_d * var_umax__blk1556_db3)) + (0.5 * ((var_btat__blk1553_db3 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db3)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_db4) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_db4)) - (var_atatgat_d * var_umax__blk1556_db4)) + (0.5 * ((var_btat__blk1553_db4 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db4)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_db5) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_db5)) - (var_atatgat_d * var_umax__blk1556_db5)) + (0.5 * ((var_btat__blk1553_db5 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db5)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1554_db6) * var_sqrtumax__blk1557) + (assign61120_e79228 * var_sqrtumax__blk1557_db6)) - (var_atatgat_d * var_umax__blk1556_db6)) + (0.5 * ((var_btat__blk1553_db6 * var_umaxpoweronepointfive__blk1558) + (var_btat__blk1553 * var_umaxpoweronepointfive__blk1558_db6)))),)
    } else {
        (var_mtat__blk1563, var_mtat__blk1563_dn0, var_mtat__blk1563_dn1, var_mtat__blk1563_dn2, var_mtat__blk1563_dn3, var_mtat__blk1563_dn4, var_mtat__blk1563_dn5, var_mtat__blk1563_dn6, var_mtat__blk1563_dn7, var_mtat__blk1563_dn8, var_mtat__blk1563_dn9, var_mtat__blk1563_dn10, var_mtat__blk1563_dn11, var_mtat__blk1563_db0, var_mtat__blk1563_db1, var_mtat__blk1563_db2, var_mtat__blk1563_db3, var_mtat__blk1563_db4, var_mtat__blk1563_db5, var_mtat__blk1563_db6,)
    }
};
        var_mtat__blk1563 = assign61120_e79242;
        var_mtat__blk1563_dn0 = assign61120_e79242_d_n0;
        var_mtat__blk1563_dn1 = assign61120_e79242_d_n1;
        var_mtat__blk1563_dn2 = assign61120_e79242_d_n2;
        var_mtat__blk1563_dn3 = assign61120_e79242_d_n3;
        var_mtat__blk1563_dn4 = assign61120_e79242_d_n4;
        var_mtat__blk1563_dn5 = assign61120_e79242_d_n5;
        var_mtat__blk1563_dn6 = assign61120_e79242_d_n6;
        var_mtat__blk1563_dn7 = assign61120_e79242_d_n7;
        var_mtat__blk1563_dn8 = assign61120_e79242_d_n8;
        var_mtat__blk1563_dn9 = assign61120_e79242_d_n9;
        var_mtat__blk1563_dn10 = assign61120_e79242_d_n10;
        var_mtat__blk1563_dn11 = assign61120_e79242_d_n11;
        var_mtat__blk1563_db0 = assign61120_e79242_d_b0;
        var_mtat__blk1563_db1 = assign61120_e79242_d_b1;
        var_mtat__blk1563_db2 = assign61120_e79242_d_b2;
        var_mtat__blk1563_db3 = assign61120_e79242_d_b3;
        var_mtat__blk1563_db4 = assign61120_e79242_d_b4;
        var_mtat__blk1563_db5 = assign61120_e79242_d_b5;
        var_mtat__blk1563_db6 = assign61120_e79242_d_b6;

        let (assign61130_e79259, assign61130_e79259_d_n0, assign61130_e79259_d_n1, assign61130_e79259_d_n2, assign61130_e79259_d_n3, assign61130_e79259_d_n4, assign61130_e79259_d_n5, assign61130_e79259_d_n6, assign61130_e79259_d_n7, assign61130_e79259_d_n8, assign61130_e79259_d_n9, assign61130_e79259_d_n10, assign61130_e79259_d_n11, assign61130_e79259_d_b0, assign61130_e79259_d_b1, assign61130_e79259_d_b2, assign61130_e79259_d_b3, assign61130_e79259_d_b4, assign61130_e79259_d_b5, assign61130_e79259_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) {
        let assign61130_e79255: f64 = (var_ltat__blk1562 - 1.0);
        let assign61130_e79257: f64 = (assign61130_e79255 * var_ktat__blk1561);
        (assign61130_e79257, ((var_ltat__blk1562_dn0 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_dn0)), ((var_ltat__blk1562_dn1 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_dn1)), ((var_ltat__blk1562_dn2 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_dn2)), ((var_ltat__blk1562_dn3 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_dn3)), ((var_ltat__blk1562_dn4 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_dn4)), ((var_ltat__blk1562_dn5 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_dn5)), ((var_ltat__blk1562_dn6 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_dn6)), ((var_ltat__blk1562_dn7 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_dn7)), ((var_ltat__blk1562_dn8 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_dn8)), ((var_ltat__blk1562_dn9 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_dn9)), ((var_ltat__blk1562_dn10 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_dn10)), ((var_ltat__blk1562_dn11 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_dn11)), ((var_ltat__blk1562_db0 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_db0)), ((var_ltat__blk1562_db1 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_db1)), ((var_ltat__blk1562_db2 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_db2)), ((var_ltat__blk1562_db3 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_db3)), ((var_ltat__blk1562_db4 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_db4)), ((var_ltat__blk1562_db5 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_db5)), ((var_ltat__blk1562_db6 * var_ktat__blk1561) + (assign61130_e79255 * var_ktat__blk1561_db6)),)
    } else {
        (var_xerfc__blk1564, var_xerfc__blk1564_dn0, var_xerfc__blk1564_dn1, var_xerfc__blk1564_dn2, var_xerfc__blk1564_dn3, var_xerfc__blk1564_dn4, var_xerfc__blk1564_dn5, var_xerfc__blk1564_dn6, var_xerfc__blk1564_dn7, var_xerfc__blk1564_dn8, var_xerfc__blk1564_dn9, var_xerfc__blk1564_dn10, var_xerfc__blk1564_dn11, var_xerfc__blk1564_db0, var_xerfc__blk1564_db1, var_xerfc__blk1564_db2, var_xerfc__blk1564_db3, var_xerfc__blk1564_db4, var_xerfc__blk1564_db5, var_xerfc__blk1564_db6,)
    }
};
        var_xerfc__blk1564 = assign61130_e79259;
        var_xerfc__blk1564_dn0 = assign61130_e79259_d_n0;
        var_xerfc__blk1564_dn1 = assign61130_e79259_d_n1;
        var_xerfc__blk1564_dn2 = assign61130_e79259_d_n2;
        var_xerfc__blk1564_dn3 = assign61130_e79259_d_n3;
        var_xerfc__blk1564_dn4 = assign61130_e79259_d_n4;
        var_xerfc__blk1564_dn5 = assign61130_e79259_d_n5;
        var_xerfc__blk1564_dn6 = assign61130_e79259_d_n6;
        var_xerfc__blk1564_dn7 = assign61130_e79259_d_n7;
        var_xerfc__blk1564_dn8 = assign61130_e79259_d_n8;
        var_xerfc__blk1564_dn9 = assign61130_e79259_d_n9;
        var_xerfc__blk1564_dn10 = assign61130_e79259_d_n10;
        var_xerfc__blk1564_dn11 = assign61130_e79259_d_n11;
        var_xerfc__blk1564_db0 = assign61130_e79259_d_b0;
        var_xerfc__blk1564_db1 = assign61130_e79259_d_b1;
        var_xerfc__blk1564_db2 = assign61130_e79259_d_b2;
        var_xerfc__blk1564_db3 = assign61130_e79259_d_b3;
        var_xerfc__blk1564_db4 = assign61130_e79259_d_b4;
        var_xerfc__blk1564_db5 = assign61130_e79259_d_b5;
        var_xerfc__blk1564_db6 = assign61130_e79259_d_b6;

        let (assign61140_e79274, assign61140_e79274_d_n0, assign61140_e79274_d_n1, assign61140_e79274_d_n2, assign61140_e79274_d_n3, assign61140_e79274_d_n4, assign61140_e79274_d_n5, assign61140_e79274_d_n6, assign61140_e79274_d_n7, assign61140_e79274_d_n8, assign61140_e79274_d_n9, assign61140_e79274_d_n10, assign61140_e79274_d_n11, assign61140_e79274_d_b0, assign61140_e79274_d_b1, assign61140_e79274_d_b2, assign61140_e79274_d_b3, assign61140_e79274_d_b4, assign61140_e79274_d_b5, assign61140_e79274_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) {
        let assign61140_e79272: f64 = (var_xerfc__blk1564 * var_xerfc__blk1564);
        (assign61140_e79272, ((var_xerfc__blk1564_dn0 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_dn0)), ((var_xerfc__blk1564_dn1 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_dn1)), ((var_xerfc__blk1564_dn2 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_dn2)), ((var_xerfc__blk1564_dn3 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_dn3)), ((var_xerfc__blk1564_dn4 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_dn4)), ((var_xerfc__blk1564_dn5 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_dn5)), ((var_xerfc__blk1564_dn6 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_dn6)), ((var_xerfc__blk1564_dn7 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_dn7)), ((var_xerfc__blk1564_dn8 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_dn8)), ((var_xerfc__blk1564_dn9 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_dn9)), ((var_xerfc__blk1564_dn10 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_dn10)), ((var_xerfc__blk1564_dn11 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_dn11)), ((var_xerfc__blk1564_db0 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_db0)), ((var_xerfc__blk1564_db1 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_db1)), ((var_xerfc__blk1564_db2 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_db2)), ((var_xerfc__blk1564_db3 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_db3)), ((var_xerfc__blk1564_db4 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_db4)), ((var_xerfc__blk1564_db5 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_db5)), ((var_xerfc__blk1564_db6 * var_xerfc__blk1564) + (var_xerfc__blk1564 * var_xerfc__blk1564_db6)),)
    } else {
        (var_ysq__blk1525, var_ysq__blk1525_dn0, var_ysq__blk1525_dn1, var_ysq__blk1525_dn2, var_ysq__blk1525_dn3, var_ysq__blk1525_dn4, var_ysq__blk1525_dn5, var_ysq__blk1525_dn6, var_ysq__blk1525_dn7, var_ysq__blk1525_dn8, var_ysq__blk1525_dn9, var_ysq__blk1525_dn10, var_ysq__blk1525_dn11, var_ysq__blk1525_db0, var_ysq__blk1525_db1, var_ysq__blk1525_db2, var_ysq__blk1525_db3, var_ysq__blk1525_db4, var_ysq__blk1525_db5, var_ysq__blk1525_db6,)
    }
};
        var_ysq__blk1525 = assign61140_e79274;
        var_ysq__blk1525_dn0 = assign61140_e79274_d_n0;
        var_ysq__blk1525_dn1 = assign61140_e79274_d_n1;
        var_ysq__blk1525_dn2 = assign61140_e79274_d_n2;
        var_ysq__blk1525_dn3 = assign61140_e79274_d_n3;
        var_ysq__blk1525_dn4 = assign61140_e79274_d_n4;
        var_ysq__blk1525_dn5 = assign61140_e79274_d_n5;
        var_ysq__blk1525_dn6 = assign61140_e79274_d_n6;
        var_ysq__blk1525_dn7 = assign61140_e79274_d_n7;
        var_ysq__blk1525_dn8 = assign61140_e79274_d_n8;
        var_ysq__blk1525_dn9 = assign61140_e79274_d_n9;
        var_ysq__blk1525_dn10 = assign61140_e79274_d_n10;
        var_ysq__blk1525_dn11 = assign61140_e79274_d_n11;
        var_ysq__blk1525_db0 = assign61140_e79274_d_b0;
        var_ysq__blk1525_db1 = assign61140_e79274_d_b1;
        var_ysq__blk1525_db2 = assign61140_e79274_d_b2;
        var_ysq__blk1525_db3 = assign61140_e79274_d_b3;
        var_ysq__blk1525_db4 = assign61140_e79274_d_b4;
        var_ysq__blk1525_db5 = assign61140_e79274_d_b5;
        var_ysq__blk1525_db6 = assign61140_e79274_d_b6;

        let assign61150_e79277: f64 = if var_xerfc__blk1564 > 0.0 { 1.0 } else { 0.0 };
        var_guard1703 = assign61150_e79277;

        let (assign61160_e79298, assign61160_e79298_d_n0, assign61160_e79298_d_n1, assign61160_e79298_d_n2, assign61160_e79298_d_n3, assign61160_e79298_d_n4, assign61160_e79298_d_n5, assign61160_e79298_d_n6, assign61160_e79298_d_n7, assign61160_e79298_d_n8, assign61160_e79298_d_n9, assign61160_e79298_d_n10, assign61160_e79298_d_n11, assign61160_e79298_d_b0, assign61160_e79298_d_b1, assign61160_e79298_d_b2, assign61160_e79298_d_b3, assign61160_e79298_d_b4, assign61160_e79298_d_b5, assign61160_e79298_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) && (var_guard1703 != 0.0)) {
        let assign61160_e79294: f64 = (var_perfc * var_xerfc__blk1564);
        let assign61160_e79295: f64 = (1.0 + assign61160_e79294);
        let assign61160_e79296: f64 = (1.0 / assign61160_e79295);
        (assign61160_e79296, (-((var_perfc * var_xerfc__blk1564_dn0) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_dn1) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_dn2) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_dn3) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_dn4) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_dn5) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_dn6) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_dn7) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_dn8) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_dn9) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_dn10) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_dn11) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_db0) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_db1) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_db2) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_db3) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_db4) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_db5) / (assign61160_e79295 * assign61160_e79295))), (-((var_perfc * var_xerfc__blk1564_db6) / (assign61160_e79295 * assign61160_e79295))),)
    } else {
        (var_terfc__blk1526, var_terfc__blk1526_dn0, var_terfc__blk1526_dn1, var_terfc__blk1526_dn2, var_terfc__blk1526_dn3, var_terfc__blk1526_dn4, var_terfc__blk1526_dn5, var_terfc__blk1526_dn6, var_terfc__blk1526_dn7, var_terfc__blk1526_dn8, var_terfc__blk1526_dn9, var_terfc__blk1526_dn10, var_terfc__blk1526_dn11, var_terfc__blk1526_db0, var_terfc__blk1526_db1, var_terfc__blk1526_db2, var_terfc__blk1526_db3, var_terfc__blk1526_db4, var_terfc__blk1526_db5, var_terfc__blk1526_db6,)
    }
};
        var_terfc__blk1526 = assign61160_e79298;
        var_terfc__blk1526_dn0 = assign61160_e79298_d_n0;
        var_terfc__blk1526_dn1 = assign61160_e79298_d_n1;
        var_terfc__blk1526_dn2 = assign61160_e79298_d_n2;
        var_terfc__blk1526_dn3 = assign61160_e79298_d_n3;
        var_terfc__blk1526_dn4 = assign61160_e79298_d_n4;
        var_terfc__blk1526_dn5 = assign61160_e79298_d_n5;
        var_terfc__blk1526_dn6 = assign61160_e79298_d_n6;
        var_terfc__blk1526_dn7 = assign61160_e79298_d_n7;
        var_terfc__blk1526_dn8 = assign61160_e79298_d_n8;
        var_terfc__blk1526_dn9 = assign61160_e79298_d_n9;
        var_terfc__blk1526_dn10 = assign61160_e79298_d_n10;
        var_terfc__blk1526_dn11 = assign61160_e79298_d_n11;
        var_terfc__blk1526_db0 = assign61160_e79298_d_b0;
        var_terfc__blk1526_db1 = assign61160_e79298_d_b1;
        var_terfc__blk1526_db2 = assign61160_e79298_d_b2;
        var_terfc__blk1526_db3 = assign61160_e79298_d_b3;
        var_terfc__blk1526_db4 = assign61160_e79298_d_b4;
        var_terfc__blk1526_db5 = assign61160_e79298_d_b5;
        var_terfc__blk1526_db6 = assign61160_e79298_d_b6;

        let (assign61170_e79320, assign61170_e79320_d_n0, assign61170_e79320_d_n1, assign61170_e79320_d_n2, assign61170_e79320_d_n3, assign61170_e79320_d_n4, assign61170_e79320_d_n5, assign61170_e79320_d_n6, assign61170_e79320_d_n7, assign61170_e79320_d_n8, assign61170_e79320_d_n9, assign61170_e79320_d_n10, assign61170_e79320_d_n11, assign61170_e79320_d_b0, assign61170_e79320_d_b1, assign61170_e79320_d_b2, assign61170_e79320_d_b3, assign61170_e79320_d_b4, assign61170_e79320_d_b5, assign61170_e79320_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) && (var_guard1703 == 0.0)) {
        let assign61170_e79316: f64 = (var_perfc * var_xerfc__blk1564);
        let assign61170_e79317: f64 = (1.0 - assign61170_e79316);
        let assign61170_e79318: f64 = (1.0 / assign61170_e79317);
        (assign61170_e79318, (-((-(var_perfc * var_xerfc__blk1564_dn0)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_dn1)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_dn2)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_dn3)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_dn4)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_dn5)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_dn6)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_dn7)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_dn8)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_dn9)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_dn10)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_dn11)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_db0)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_db1)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_db2)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_db3)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_db4)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_db5)) / (assign61170_e79317 * assign61170_e79317))), (-((-(var_perfc * var_xerfc__blk1564_db6)) / (assign61170_e79317 * assign61170_e79317))),)
    } else {
        (var_terfc__blk1526, var_terfc__blk1526_dn0, var_terfc__blk1526_dn1, var_terfc__blk1526_dn2, var_terfc__blk1526_dn3, var_terfc__blk1526_dn4, var_terfc__blk1526_dn5, var_terfc__blk1526_dn6, var_terfc__blk1526_dn7, var_terfc__blk1526_dn8, var_terfc__blk1526_dn9, var_terfc__blk1526_dn10, var_terfc__blk1526_dn11, var_terfc__blk1526_db0, var_terfc__blk1526_db1, var_terfc__blk1526_db2, var_terfc__blk1526_db3, var_terfc__blk1526_db4, var_terfc__blk1526_db5, var_terfc__blk1526_db6,)
    }
};
        var_terfc__blk1526 = assign61170_e79320;
        var_terfc__blk1526_dn0 = assign61170_e79320_d_n0;
        var_terfc__blk1526_dn1 = assign61170_e79320_d_n1;
        var_terfc__blk1526_dn2 = assign61170_e79320_d_n2;
        var_terfc__blk1526_dn3 = assign61170_e79320_d_n3;
        var_terfc__blk1526_dn4 = assign61170_e79320_d_n4;
        var_terfc__blk1526_dn5 = assign61170_e79320_d_n5;
        var_terfc__blk1526_dn6 = assign61170_e79320_d_n6;
        var_terfc__blk1526_dn7 = assign61170_e79320_d_n7;
        var_terfc__blk1526_dn8 = assign61170_e79320_d_n8;
        var_terfc__blk1526_dn9 = assign61170_e79320_d_n9;
        var_terfc__blk1526_dn10 = assign61170_e79320_d_n10;
        var_terfc__blk1526_dn11 = assign61170_e79320_d_n11;
        var_terfc__blk1526_db0 = assign61170_e79320_d_b0;
        var_terfc__blk1526_db1 = assign61170_e79320_d_b1;
        var_terfc__blk1526_db2 = assign61170_e79320_d_b2;
        var_terfc__blk1526_db3 = assign61170_e79320_d_b3;
        var_terfc__blk1526_db4 = assign61170_e79320_d_b4;
        var_terfc__blk1526_db5 = assign61170_e79320_d_b5;
        var_terfc__blk1526_db6 = assign61170_e79320_d_b6;

        let assign61180_e79322: f64 = (-var_ysq__blk1525);
        let assign61180_e79324: f64 = (assign61180_e79322 + var_mtat__blk1563);
        let assign61180_e79326: f64 = (-230.25850929940458);
        let assign61180_e79327: f64 = if assign61180_e79324 > assign61180_e79326 { 1.0 } else { 0.0 };
        var_guard1704 = assign61180_e79327;

        let (assign61190_e79346, assign61190_e79346_d_n0, assign61190_e79346_d_n1, assign61190_e79346_d_n2, assign61190_e79346_d_n3, assign61190_e79346_d_n4, assign61190_e79346_d_n5, assign61190_e79346_d_n6, assign61190_e79346_d_n7, assign61190_e79346_d_n8, assign61190_e79346_d_n9, assign61190_e79346_d_n10, assign61190_e79346_d_n11, assign61190_e79346_d_b0, assign61190_e79346_d_b1, assign61190_e79346_d_b2, assign61190_e79346_d_b3, assign61190_e79346_d_b4, assign61190_e79346_d_b5, assign61190_e79346_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) && (var_guard1704 != 0.0)) {
        let assign61190_e79341: f64 = (-var_ysq__blk1525);
        let assign61190_e79343: f64 = (assign61190_e79341 + var_mtat__blk1563);
        let assign61190_e79344: f64 = (assign61190_e79343).exp();
        (assign61190_e79344, (assign61190_e79344 * ((-var_ysq__blk1525_dn0) + var_mtat__blk1563_dn0)), (assign61190_e79344 * ((-var_ysq__blk1525_dn1) + var_mtat__blk1563_dn1)), (assign61190_e79344 * ((-var_ysq__blk1525_dn2) + var_mtat__blk1563_dn2)), (assign61190_e79344 * ((-var_ysq__blk1525_dn3) + var_mtat__blk1563_dn3)), (assign61190_e79344 * ((-var_ysq__blk1525_dn4) + var_mtat__blk1563_dn4)), (assign61190_e79344 * ((-var_ysq__blk1525_dn5) + var_mtat__blk1563_dn5)), (assign61190_e79344 * ((-var_ysq__blk1525_dn6) + var_mtat__blk1563_dn6)), (assign61190_e79344 * ((-var_ysq__blk1525_dn7) + var_mtat__blk1563_dn7)), (assign61190_e79344 * ((-var_ysq__blk1525_dn8) + var_mtat__blk1563_dn8)), (assign61190_e79344 * ((-var_ysq__blk1525_dn9) + var_mtat__blk1563_dn9)), (assign61190_e79344 * ((-var_ysq__blk1525_dn10) + var_mtat__blk1563_dn10)), (assign61190_e79344 * ((-var_ysq__blk1525_dn11) + var_mtat__blk1563_dn11)), (assign61190_e79344 * ((-var_ysq__blk1525_db0) + var_mtat__blk1563_db0)), (assign61190_e79344 * ((-var_ysq__blk1525_db1) + var_mtat__blk1563_db1)), (assign61190_e79344 * ((-var_ysq__blk1525_db2) + var_mtat__blk1563_db2)), (assign61190_e79344 * ((-var_ysq__blk1525_db3) + var_mtat__blk1563_db3)), (assign61190_e79344 * ((-var_ysq__blk1525_db4) + var_mtat__blk1563_db4)), (assign61190_e79344 * ((-var_ysq__blk1525_db5) + var_mtat__blk1563_db5)), (assign61190_e79344 * ((-var_ysq__blk1525_db6) + var_mtat__blk1563_db6)),)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61190_e79346;
        var_tmp__blk1543_dn0 = assign61190_e79346_d_n0;
        var_tmp__blk1543_dn1 = assign61190_e79346_d_n1;
        var_tmp__blk1543_dn2 = assign61190_e79346_d_n2;
        var_tmp__blk1543_dn3 = assign61190_e79346_d_n3;
        var_tmp__blk1543_dn4 = assign61190_e79346_d_n4;
        var_tmp__blk1543_dn5 = assign61190_e79346_d_n5;
        var_tmp__blk1543_dn6 = assign61190_e79346_d_n6;
        var_tmp__blk1543_dn7 = assign61190_e79346_d_n7;
        var_tmp__blk1543_dn8 = assign61190_e79346_d_n8;
        var_tmp__blk1543_dn9 = assign61190_e79346_d_n9;
        var_tmp__blk1543_dn10 = assign61190_e79346_d_n10;
        var_tmp__blk1543_dn11 = assign61190_e79346_d_n11;
        var_tmp__blk1543_db0 = assign61190_e79346_d_b0;
        var_tmp__blk1543_db1 = assign61190_e79346_d_b1;
        var_tmp__blk1543_db2 = assign61190_e79346_d_b2;
        var_tmp__blk1543_db3 = assign61190_e79346_d_b3;
        var_tmp__blk1543_db4 = assign61190_e79346_d_b4;
        var_tmp__blk1543_db5 = assign61190_e79346_d_b5;
        var_tmp__blk1543_db6 = assign61190_e79346_d_b6;

        let (assign61200_e79396, assign61200_e79396_d_n0, assign61200_e79396_d_n1, assign61200_e79396_d_n2, assign61200_e79396_d_n3, assign61200_e79396_d_n4, assign61200_e79396_d_n5, assign61200_e79396_d_n6, assign61200_e79396_d_n7, assign61200_e79396_d_n8, assign61200_e79396_d_n9, assign61200_e79396_d_n10, assign61200_e79396_d_n11, assign61200_e79396_d_b0, assign61200_e79396_d_b1, assign61200_e79396_d_b2, assign61200_e79396_d_b3, assign61200_e79396_d_b4, assign61200_e79396_d_b5, assign61200_e79396_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) && (var_guard1704 == 0.0)) {
        let assign61200_e79363: f64 = (-230.25850929940458);
        let assign61200_e79365: f64 = (-var_ysq__blk1525);
        let assign61200_e79367: f64 = (assign61200_e79365 + var_mtat__blk1563);
        let assign61200_e79368: f64 = (assign61200_e79363 - assign61200_e79367);
        let assign61200_e79372: f64 = (-230.25850929940458);
        let assign61200_e79374: f64 = (-var_ysq__blk1525);
        let assign61200_e79376: f64 = (assign61200_e79374 + var_mtat__blk1563);
        let assign61200_e79377: f64 = (assign61200_e79372 - assign61200_e79376);
        let assign61200_e79380: f64 = (-230.25850929940458);
        let assign61200_e79382: f64 = (-var_ysq__blk1525);
        let assign61200_e79384: f64 = (assign61200_e79382 + var_mtat__blk1563);
        let assign61200_e79385: f64 = (assign61200_e79380 - assign61200_e79384);
        let assign61200_e79387: f64 = (assign61200_e79385 * 0.3333333333333333);
        let assign61200_e79388: f64 = (1.0 + assign61200_e79387);
        let assign61200_e79389: f64 = (assign61200_e79377 * assign61200_e79388);
        let assign61200_e79390: f64 = (0.5 * assign61200_e79389);
        let assign61200_e79391: f64 = (1.0 + assign61200_e79390);
        let assign61200_e79392: f64 = (assign61200_e79368 * assign61200_e79391);
        let assign61200_e79393: f64 = (1.0 + assign61200_e79392);
        let assign61200_e79394: f64 = (1e-100 / assign61200_e79393);
        (assign61200_e79394, (-((1e-100 * (((-((-var_ysq__blk1525_dn0) + var_mtat__blk1563_dn0)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_dn0) + var_mtat__blk1563_dn0)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_dn0) + var_mtat__blk1563_dn0)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_dn1) + var_mtat__blk1563_dn1)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_dn1) + var_mtat__blk1563_dn1)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_dn1) + var_mtat__blk1563_dn1)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_dn2) + var_mtat__blk1563_dn2)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_dn2) + var_mtat__blk1563_dn2)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_dn2) + var_mtat__blk1563_dn2)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_dn3) + var_mtat__blk1563_dn3)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_dn3) + var_mtat__blk1563_dn3)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_dn3) + var_mtat__blk1563_dn3)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_dn4) + var_mtat__blk1563_dn4)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_dn4) + var_mtat__blk1563_dn4)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_dn4) + var_mtat__blk1563_dn4)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_dn5) + var_mtat__blk1563_dn5)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_dn5) + var_mtat__blk1563_dn5)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_dn5) + var_mtat__blk1563_dn5)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_dn6) + var_mtat__blk1563_dn6)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_dn6) + var_mtat__blk1563_dn6)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_dn6) + var_mtat__blk1563_dn6)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_dn7) + var_mtat__blk1563_dn7)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_dn7) + var_mtat__blk1563_dn7)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_dn7) + var_mtat__blk1563_dn7)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_dn8) + var_mtat__blk1563_dn8)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_dn8) + var_mtat__blk1563_dn8)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_dn8) + var_mtat__blk1563_dn8)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_dn9) + var_mtat__blk1563_dn9)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_dn9) + var_mtat__blk1563_dn9)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_dn9) + var_mtat__blk1563_dn9)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_dn10) + var_mtat__blk1563_dn10)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_dn10) + var_mtat__blk1563_dn10)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_dn10) + var_mtat__blk1563_dn10)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_dn11) + var_mtat__blk1563_dn11)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_dn11) + var_mtat__blk1563_dn11)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_dn11) + var_mtat__blk1563_dn11)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_db0) + var_mtat__blk1563_db0)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_db0) + var_mtat__blk1563_db0)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_db0) + var_mtat__blk1563_db0)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_db1) + var_mtat__blk1563_db1)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_db1) + var_mtat__blk1563_db1)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_db1) + var_mtat__blk1563_db1)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_db2) + var_mtat__blk1563_db2)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_db2) + var_mtat__blk1563_db2)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_db2) + var_mtat__blk1563_db2)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_db3) + var_mtat__blk1563_db3)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_db3) + var_mtat__blk1563_db3)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_db3) + var_mtat__blk1563_db3)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_db4) + var_mtat__blk1563_db4)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_db4) + var_mtat__blk1563_db4)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_db4) + var_mtat__blk1563_db4)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_db5) + var_mtat__blk1563_db5)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_db5) + var_mtat__blk1563_db5)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_db5) + var_mtat__blk1563_db5)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))), (-((1e-100 * (((-((-var_ysq__blk1525_db6) + var_mtat__blk1563_db6)) * assign61200_e79391) + (assign61200_e79368 * (0.5 * (((-((-var_ysq__blk1525_db6) + var_mtat__blk1563_db6)) * assign61200_e79388) + (assign61200_e79377 * ((-((-var_ysq__blk1525_db6) + var_mtat__blk1563_db6)) * 0.3333333333333333))))))) / (assign61200_e79393 * assign61200_e79393))),)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61200_e79396;
        var_tmp__blk1543_dn0 = assign61200_e79396_d_n0;
        var_tmp__blk1543_dn1 = assign61200_e79396_d_n1;
        var_tmp__blk1543_dn2 = assign61200_e79396_d_n2;
        var_tmp__blk1543_dn3 = assign61200_e79396_d_n3;
        var_tmp__blk1543_dn4 = assign61200_e79396_d_n4;
        var_tmp__blk1543_dn5 = assign61200_e79396_d_n5;
        var_tmp__blk1543_dn6 = assign61200_e79396_d_n6;
        var_tmp__blk1543_dn7 = assign61200_e79396_d_n7;
        var_tmp__blk1543_dn8 = assign61200_e79396_d_n8;
        var_tmp__blk1543_dn9 = assign61200_e79396_d_n9;
        var_tmp__blk1543_dn10 = assign61200_e79396_d_n10;
        var_tmp__blk1543_dn11 = assign61200_e79396_d_n11;
        var_tmp__blk1543_db0 = assign61200_e79396_d_b0;
        var_tmp__blk1543_db1 = assign61200_e79396_d_b1;
        var_tmp__blk1543_db2 = assign61200_e79396_d_b2;
        var_tmp__blk1543_db3 = assign61200_e79396_d_b3;
        var_tmp__blk1543_db4 = assign61200_e79396_d_b4;
        var_tmp__blk1543_db5 = assign61200_e79396_d_b5;
        var_tmp__blk1543_db6 = assign61200_e79396_d_b6;

        let (assign61210_e79427, assign61210_e79427_d_n0, assign61210_e79427_d_n1, assign61210_e79427_d_n2, assign61210_e79427_d_n3, assign61210_e79427_d_n4, assign61210_e79427_d_n5, assign61210_e79427_d_n6, assign61210_e79427_d_n7, assign61210_e79427_d_n8, assign61210_e79427_d_n9, assign61210_e79427_d_n10, assign61210_e79427_d_n11, assign61210_e79427_d_b0, assign61210_e79427_d_b1, assign61210_e79427_d_b2, assign61210_e79427_d_b3, assign61210_e79427_d_b4, assign61210_e79427_d_b5, assign61210_e79427_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) {
        let assign61210_e79409: f64 = (0.29214664 * var_terfc__blk1526);
        let assign61210_e79413: f64 = (var_terfc__blk1526 * var_terfc__blk1526);
        let assign61210_e79414: f64 = (var_berfc * assign61210_e79413);
        let assign61210_e79415: f64 = (assign61210_e79409 + assign61210_e79414);
        let assign61210_e79419: f64 = (var_terfc__blk1526 * var_terfc__blk1526);
        let assign61210_e79421: f64 = (assign61210_e79419 * var_terfc__blk1526);
        let assign61210_e79422: f64 = (var_cerfc * assign61210_e79421);
        let assign61210_e79423: f64 = (assign61210_e79415 + assign61210_e79422);
        let assign61210_e79425: f64 = (assign61210_e79423 * var_tmp__blk1543);
        (assign61210_e79425, (((((0.29214664 * var_terfc__blk1526_dn0) + (var_berfc * ((var_terfc__blk1526_dn0 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn0)))) + (var_cerfc * ((((var_terfc__blk1526_dn0 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn0)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_dn0)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_dn0)), (((((0.29214664 * var_terfc__blk1526_dn1) + (var_berfc * ((var_terfc__blk1526_dn1 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn1)))) + (var_cerfc * ((((var_terfc__blk1526_dn1 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn1)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_dn1)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_dn1)), (((((0.29214664 * var_terfc__blk1526_dn2) + (var_berfc * ((var_terfc__blk1526_dn2 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn2)))) + (var_cerfc * ((((var_terfc__blk1526_dn2 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn2)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_dn2)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_dn2)), (((((0.29214664 * var_terfc__blk1526_dn3) + (var_berfc * ((var_terfc__blk1526_dn3 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn3)))) + (var_cerfc * ((((var_terfc__blk1526_dn3 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn3)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_dn3)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_dn3)), (((((0.29214664 * var_terfc__blk1526_dn4) + (var_berfc * ((var_terfc__blk1526_dn4 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn4)))) + (var_cerfc * ((((var_terfc__blk1526_dn4 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn4)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_dn4)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_dn4)), (((((0.29214664 * var_terfc__blk1526_dn5) + (var_berfc * ((var_terfc__blk1526_dn5 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn5)))) + (var_cerfc * ((((var_terfc__blk1526_dn5 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn5)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_dn5)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_dn5)), (((((0.29214664 * var_terfc__blk1526_dn6) + (var_berfc * ((var_terfc__blk1526_dn6 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn6)))) + (var_cerfc * ((((var_terfc__blk1526_dn6 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn6)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_dn6)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_dn6)), (((((0.29214664 * var_terfc__blk1526_dn7) + (var_berfc * ((var_terfc__blk1526_dn7 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn7)))) + (var_cerfc * ((((var_terfc__blk1526_dn7 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn7)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_dn7)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_dn7)), (((((0.29214664 * var_terfc__blk1526_dn8) + (var_berfc * ((var_terfc__blk1526_dn8 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn8)))) + (var_cerfc * ((((var_terfc__blk1526_dn8 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn8)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_dn8)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_dn8)), (((((0.29214664 * var_terfc__blk1526_dn9) + (var_berfc * ((var_terfc__blk1526_dn9 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn9)))) + (var_cerfc * ((((var_terfc__blk1526_dn9 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn9)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_dn9)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_dn9)), (((((0.29214664 * var_terfc__blk1526_dn10) + (var_berfc * ((var_terfc__blk1526_dn10 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn10)))) + (var_cerfc * ((((var_terfc__blk1526_dn10 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn10)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_dn10)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_dn10)), (((((0.29214664 * var_terfc__blk1526_dn11) + (var_berfc * ((var_terfc__blk1526_dn11 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn11)))) + (var_cerfc * ((((var_terfc__blk1526_dn11 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_dn11)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_dn11)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_dn11)), (((((0.29214664 * var_terfc__blk1526_db0) + (var_berfc * ((var_terfc__blk1526_db0 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_db0)))) + (var_cerfc * ((((var_terfc__blk1526_db0 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_db0)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_db0)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_db0)), (((((0.29214664 * var_terfc__blk1526_db1) + (var_berfc * ((var_terfc__blk1526_db1 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_db1)))) + (var_cerfc * ((((var_terfc__blk1526_db1 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_db1)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_db1)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_db1)), (((((0.29214664 * var_terfc__blk1526_db2) + (var_berfc * ((var_terfc__blk1526_db2 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_db2)))) + (var_cerfc * ((((var_terfc__blk1526_db2 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_db2)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_db2)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_db2)), (((((0.29214664 * var_terfc__blk1526_db3) + (var_berfc * ((var_terfc__blk1526_db3 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_db3)))) + (var_cerfc * ((((var_terfc__blk1526_db3 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_db3)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_db3)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_db3)), (((((0.29214664 * var_terfc__blk1526_db4) + (var_berfc * ((var_terfc__blk1526_db4 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_db4)))) + (var_cerfc * ((((var_terfc__blk1526_db4 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_db4)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_db4)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_db4)), (((((0.29214664 * var_terfc__blk1526_db5) + (var_berfc * ((var_terfc__blk1526_db5 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_db5)))) + (var_cerfc * ((((var_terfc__blk1526_db5 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_db5)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_db5)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_db5)), (((((0.29214664 * var_terfc__blk1526_db6) + (var_berfc * ((var_terfc__blk1526_db6 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_db6)))) + (var_cerfc * ((((var_terfc__blk1526_db6 * var_terfc__blk1526) + (var_terfc__blk1526 * var_terfc__blk1526_db6)) * var_terfc__blk1526) + (assign61210_e79419 * var_terfc__blk1526_db6)))) * var_tmp__blk1543) + (assign61210_e79423 * var_tmp__blk1543_db6)),)
    } else {
        (var_erfcpos__blk1527, var_erfcpos__blk1527_dn0, var_erfcpos__blk1527_dn1, var_erfcpos__blk1527_dn2, var_erfcpos__blk1527_dn3, var_erfcpos__blk1527_dn4, var_erfcpos__blk1527_dn5, var_erfcpos__blk1527_dn6, var_erfcpos__blk1527_dn7, var_erfcpos__blk1527_dn8, var_erfcpos__blk1527_dn9, var_erfcpos__blk1527_dn10, var_erfcpos__blk1527_dn11, var_erfcpos__blk1527_db0, var_erfcpos__blk1527_db1, var_erfcpos__blk1527_db2, var_erfcpos__blk1527_db3, var_erfcpos__blk1527_db4, var_erfcpos__blk1527_db5, var_erfcpos__blk1527_db6,)
    }
};
        var_erfcpos__blk1527 = assign61210_e79427;
        var_erfcpos__blk1527_dn0 = assign61210_e79427_d_n0;
        var_erfcpos__blk1527_dn1 = assign61210_e79427_d_n1;
        var_erfcpos__blk1527_dn2 = assign61210_e79427_d_n2;
        var_erfcpos__blk1527_dn3 = assign61210_e79427_d_n3;
        var_erfcpos__blk1527_dn4 = assign61210_e79427_d_n4;
        var_erfcpos__blk1527_dn5 = assign61210_e79427_d_n5;
        var_erfcpos__blk1527_dn6 = assign61210_e79427_d_n6;
        var_erfcpos__blk1527_dn7 = assign61210_e79427_d_n7;
        var_erfcpos__blk1527_dn8 = assign61210_e79427_d_n8;
        var_erfcpos__blk1527_dn9 = assign61210_e79427_d_n9;
        var_erfcpos__blk1527_dn10 = assign61210_e79427_d_n10;
        var_erfcpos__blk1527_dn11 = assign61210_e79427_d_n11;
        var_erfcpos__blk1527_db0 = assign61210_e79427_d_b0;
        var_erfcpos__blk1527_db1 = assign61210_e79427_d_b1;
        var_erfcpos__blk1527_db2 = assign61210_e79427_d_b2;
        var_erfcpos__blk1527_db3 = assign61210_e79427_d_b3;
        var_erfcpos__blk1527_db4 = assign61210_e79427_d_b4;
        var_erfcpos__blk1527_db5 = assign61210_e79427_d_b5;
        var_erfcpos__blk1527_db6 = assign61210_e79427_d_b6;

        let assign61220_e79430: f64 = if var_xerfc__blk1564 > 0.0 { 1.0 } else { 0.0 };
        var_guard1705 = assign61220_e79430;

        let (assign61230_e79445, assign61230_e79445_d_n0, assign61230_e79445_d_n1, assign61230_e79445_d_n2, assign61230_e79445_d_n3, assign61230_e79445_d_n4, assign61230_e79445_d_n5, assign61230_e79445_d_n6, assign61230_e79445_d_n7, assign61230_e79445_d_n8, assign61230_e79445_d_n9, assign61230_e79445_d_n10, assign61230_e79445_d_n11, assign61230_e79445_d_b0, assign61230_e79445_d_b1, assign61230_e79445_d_b2, assign61230_e79445_d_b3, assign61230_e79445_d_b4, assign61230_e79445_d_b5, assign61230_e79445_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) && (var_guard1705 != 0.0)) {
        (var_erfcpos__blk1527, var_erfcpos__blk1527_dn0, var_erfcpos__blk1527_dn1, var_erfcpos__blk1527_dn2, var_erfcpos__blk1527_dn3, var_erfcpos__blk1527_dn4, var_erfcpos__blk1527_dn5, var_erfcpos__blk1527_dn6, var_erfcpos__blk1527_dn7, var_erfcpos__blk1527_dn8, var_erfcpos__blk1527_dn9, var_erfcpos__blk1527_dn10, var_erfcpos__blk1527_dn11, var_erfcpos__blk1527_db0, var_erfcpos__blk1527_db1, var_erfcpos__blk1527_db2, var_erfcpos__blk1527_db3, var_erfcpos__blk1527_db4, var_erfcpos__blk1527_db5, var_erfcpos__blk1527_db6,)
    } else {
        (var_erfctimesexpmtat__blk1565, var_erfctimesexpmtat__blk1565_dn0, var_erfctimesexpmtat__blk1565_dn1, var_erfctimesexpmtat__blk1565_dn2, var_erfctimesexpmtat__blk1565_dn3, var_erfctimesexpmtat__blk1565_dn4, var_erfctimesexpmtat__blk1565_dn5, var_erfctimesexpmtat__blk1565_dn6, var_erfctimesexpmtat__blk1565_dn7, var_erfctimesexpmtat__blk1565_dn8, var_erfctimesexpmtat__blk1565_dn9, var_erfctimesexpmtat__blk1565_dn10, var_erfctimesexpmtat__blk1565_dn11, var_erfctimesexpmtat__blk1565_db0, var_erfctimesexpmtat__blk1565_db1, var_erfctimesexpmtat__blk1565_db2, var_erfctimesexpmtat__blk1565_db3, var_erfctimesexpmtat__blk1565_db4, var_erfctimesexpmtat__blk1565_db5, var_erfctimesexpmtat__blk1565_db6,)
    }
};
        var_erfctimesexpmtat__blk1565 = assign61230_e79445;
        var_erfctimesexpmtat__blk1565_dn0 = assign61230_e79445_d_n0;
        var_erfctimesexpmtat__blk1565_dn1 = assign61230_e79445_d_n1;
        var_erfctimesexpmtat__blk1565_dn2 = assign61230_e79445_d_n2;
        var_erfctimesexpmtat__blk1565_dn3 = assign61230_e79445_d_n3;
        var_erfctimesexpmtat__blk1565_dn4 = assign61230_e79445_d_n4;
        var_erfctimesexpmtat__blk1565_dn5 = assign61230_e79445_d_n5;
        var_erfctimesexpmtat__blk1565_dn6 = assign61230_e79445_d_n6;
        var_erfctimesexpmtat__blk1565_dn7 = assign61230_e79445_d_n7;
        var_erfctimesexpmtat__blk1565_dn8 = assign61230_e79445_d_n8;
        var_erfctimesexpmtat__blk1565_dn9 = assign61230_e79445_d_n9;
        var_erfctimesexpmtat__blk1565_dn10 = assign61230_e79445_d_n10;
        var_erfctimesexpmtat__blk1565_dn11 = assign61230_e79445_d_n11;
        var_erfctimesexpmtat__blk1565_db0 = assign61230_e79445_d_b0;
        var_erfctimesexpmtat__blk1565_db1 = assign61230_e79445_d_b1;
        var_erfctimesexpmtat__blk1565_db2 = assign61230_e79445_d_b2;
        var_erfctimesexpmtat__blk1565_db3 = assign61230_e79445_d_b3;
        var_erfctimesexpmtat__blk1565_db4 = assign61230_e79445_d_b4;
        var_erfctimesexpmtat__blk1565_db5 = assign61230_e79445_d_b5;
        var_erfctimesexpmtat__blk1565_db6 = assign61230_e79445_d_b6;

        let assign61240_e79448: f64 = (-230.25850929940458);
        let assign61240_e79449: f64 = if var_mtat__blk1563 > assign61240_e79448 { 1.0 } else { 0.0 };
        var_guard1706 = assign61240_e79449;

        *var_erfcpos__blk1527_slot = var_erfcpos__blk1527;
        *var_erfcpos__blk1527_db0_slot = var_erfcpos__blk1527_db0;
        *var_erfcpos__blk1527_db1_slot = var_erfcpos__blk1527_db1;
        *var_erfcpos__blk1527_db2_slot = var_erfcpos__blk1527_db2;
        *var_erfcpos__blk1527_db3_slot = var_erfcpos__blk1527_db3;
        *var_erfcpos__blk1527_db4_slot = var_erfcpos__blk1527_db4;
        *var_erfcpos__blk1527_db5_slot = var_erfcpos__blk1527_db5;
        *var_erfcpos__blk1527_db6_slot = var_erfcpos__blk1527_db6;
        *var_erfcpos__blk1527_dn0_slot = var_erfcpos__blk1527_dn0;
        *var_erfcpos__blk1527_dn1_slot = var_erfcpos__blk1527_dn1;
        *var_erfcpos__blk1527_dn10_slot = var_erfcpos__blk1527_dn10;
        *var_erfcpos__blk1527_dn11_slot = var_erfcpos__blk1527_dn11;
        *var_erfcpos__blk1527_dn2_slot = var_erfcpos__blk1527_dn2;
        *var_erfcpos__blk1527_dn3_slot = var_erfcpos__blk1527_dn3;
        *var_erfcpos__blk1527_dn4_slot = var_erfcpos__blk1527_dn4;
        *var_erfcpos__blk1527_dn5_slot = var_erfcpos__blk1527_dn5;
        *var_erfcpos__blk1527_dn6_slot = var_erfcpos__blk1527_dn6;
        *var_erfcpos__blk1527_dn7_slot = var_erfcpos__blk1527_dn7;
        *var_erfcpos__blk1527_dn8_slot = var_erfcpos__blk1527_dn8;
        *var_erfcpos__blk1527_dn9_slot = var_erfcpos__blk1527_dn9;
        *var_erfctimesexpmtat__blk1565_slot = var_erfctimesexpmtat__blk1565;
        *var_erfctimesexpmtat__blk1565_db0_slot = var_erfctimesexpmtat__blk1565_db0;
        *var_erfctimesexpmtat__blk1565_db1_slot = var_erfctimesexpmtat__blk1565_db1;
        *var_erfctimesexpmtat__blk1565_db2_slot = var_erfctimesexpmtat__blk1565_db2;
        *var_erfctimesexpmtat__blk1565_db3_slot = var_erfctimesexpmtat__blk1565_db3;
        *var_erfctimesexpmtat__blk1565_db4_slot = var_erfctimesexpmtat__blk1565_db4;
        *var_erfctimesexpmtat__blk1565_db5_slot = var_erfctimesexpmtat__blk1565_db5;
        *var_erfctimesexpmtat__blk1565_db6_slot = var_erfctimesexpmtat__blk1565_db6;
        *var_erfctimesexpmtat__blk1565_dn0_slot = var_erfctimesexpmtat__blk1565_dn0;
        *var_erfctimesexpmtat__blk1565_dn1_slot = var_erfctimesexpmtat__blk1565_dn1;
        *var_erfctimesexpmtat__blk1565_dn10_slot = var_erfctimesexpmtat__blk1565_dn10;
        *var_erfctimesexpmtat__blk1565_dn11_slot = var_erfctimesexpmtat__blk1565_dn11;
        *var_erfctimesexpmtat__blk1565_dn2_slot = var_erfctimesexpmtat__blk1565_dn2;
        *var_erfctimesexpmtat__blk1565_dn3_slot = var_erfctimesexpmtat__blk1565_dn3;
        *var_erfctimesexpmtat__blk1565_dn4_slot = var_erfctimesexpmtat__blk1565_dn4;
        *var_erfctimesexpmtat__blk1565_dn5_slot = var_erfctimesexpmtat__blk1565_dn5;
        *var_erfctimesexpmtat__blk1565_dn6_slot = var_erfctimesexpmtat__blk1565_dn6;
        *var_erfctimesexpmtat__blk1565_dn7_slot = var_erfctimesexpmtat__blk1565_dn7;
        *var_erfctimesexpmtat__blk1565_dn8_slot = var_erfctimesexpmtat__blk1565_dn8;
        *var_erfctimesexpmtat__blk1565_dn9_slot = var_erfctimesexpmtat__blk1565_dn9;
        *var_guard1702_slot = var_guard1702;
        *var_guard1703_slot = var_guard1703;
        *var_guard1704_slot = var_guard1704;
        *var_guard1705_slot = var_guard1705;
        *var_guard1706_slot = var_guard1706;
        *var_ktat__blk1561_slot = var_ktat__blk1561;
        *var_ktat__blk1561_db0_slot = var_ktat__blk1561_db0;
        *var_ktat__blk1561_db1_slot = var_ktat__blk1561_db1;
        *var_ktat__blk1561_db2_slot = var_ktat__blk1561_db2;
        *var_ktat__blk1561_db3_slot = var_ktat__blk1561_db3;
        *var_ktat__blk1561_db4_slot = var_ktat__blk1561_db4;
        *var_ktat__blk1561_db5_slot = var_ktat__blk1561_db5;
        *var_ktat__blk1561_db6_slot = var_ktat__blk1561_db6;
        *var_ktat__blk1561_dn0_slot = var_ktat__blk1561_dn0;
        *var_ktat__blk1561_dn1_slot = var_ktat__blk1561_dn1;
        *var_ktat__blk1561_dn10_slot = var_ktat__blk1561_dn10;
        *var_ktat__blk1561_dn11_slot = var_ktat__blk1561_dn11;
        *var_ktat__blk1561_dn2_slot = var_ktat__blk1561_dn2;
        *var_ktat__blk1561_dn3_slot = var_ktat__blk1561_dn3;
        *var_ktat__blk1561_dn4_slot = var_ktat__blk1561_dn4;
        *var_ktat__blk1561_dn5_slot = var_ktat__blk1561_dn5;
        *var_ktat__blk1561_dn6_slot = var_ktat__blk1561_dn6;
        *var_ktat__blk1561_dn7_slot = var_ktat__blk1561_dn7;
        *var_ktat__blk1561_dn8_slot = var_ktat__blk1561_dn8;
        *var_ktat__blk1561_dn9_slot = var_ktat__blk1561_dn9;
        *var_ltat__blk1562_slot = var_ltat__blk1562;
        *var_ltat__blk1562_db0_slot = var_ltat__blk1562_db0;
        *var_ltat__blk1562_db1_slot = var_ltat__blk1562_db1;
        *var_ltat__blk1562_db2_slot = var_ltat__blk1562_db2;
        *var_ltat__blk1562_db3_slot = var_ltat__blk1562_db3;
        *var_ltat__blk1562_db4_slot = var_ltat__blk1562_db4;
        *var_ltat__blk1562_db5_slot = var_ltat__blk1562_db5;
        *var_ltat__blk1562_db6_slot = var_ltat__blk1562_db6;
        *var_ltat__blk1562_dn0_slot = var_ltat__blk1562_dn0;
        *var_ltat__blk1562_dn1_slot = var_ltat__blk1562_dn1;
        *var_ltat__blk1562_dn10_slot = var_ltat__blk1562_dn10;
        *var_ltat__blk1562_dn11_slot = var_ltat__blk1562_dn11;
        *var_ltat__blk1562_dn2_slot = var_ltat__blk1562_dn2;
        *var_ltat__blk1562_dn3_slot = var_ltat__blk1562_dn3;
        *var_ltat__blk1562_dn4_slot = var_ltat__blk1562_dn4;
        *var_ltat__blk1562_dn5_slot = var_ltat__blk1562_dn5;
        *var_ltat__blk1562_dn6_slot = var_ltat__blk1562_dn6;
        *var_ltat__blk1562_dn7_slot = var_ltat__blk1562_dn7;
        *var_ltat__blk1562_dn8_slot = var_ltat__blk1562_dn8;
        *var_ltat__blk1562_dn9_slot = var_ltat__blk1562_dn9;
        *var_mtat__blk1563_slot = var_mtat__blk1563;
        *var_mtat__blk1563_db0_slot = var_mtat__blk1563_db0;
        *var_mtat__blk1563_db1_slot = var_mtat__blk1563_db1;
        *var_mtat__blk1563_db2_slot = var_mtat__blk1563_db2;
        *var_mtat__blk1563_db3_slot = var_mtat__blk1563_db3;
        *var_mtat__blk1563_db4_slot = var_mtat__blk1563_db4;
        *var_mtat__blk1563_db5_slot = var_mtat__blk1563_db5;
        *var_mtat__blk1563_db6_slot = var_mtat__blk1563_db6;
        *var_mtat__blk1563_dn0_slot = var_mtat__blk1563_dn0;
        *var_mtat__blk1563_dn1_slot = var_mtat__blk1563_dn1;
        *var_mtat__blk1563_dn10_slot = var_mtat__blk1563_dn10;
        *var_mtat__blk1563_dn11_slot = var_mtat__blk1563_dn11;
        *var_mtat__blk1563_dn2_slot = var_mtat__blk1563_dn2;
        *var_mtat__blk1563_dn3_slot = var_mtat__blk1563_dn3;
        *var_mtat__blk1563_dn4_slot = var_mtat__blk1563_dn4;
        *var_mtat__blk1563_dn5_slot = var_mtat__blk1563_dn5;
        *var_mtat__blk1563_dn6_slot = var_mtat__blk1563_dn6;
        *var_mtat__blk1563_dn7_slot = var_mtat__blk1563_dn7;
        *var_mtat__blk1563_dn8_slot = var_mtat__blk1563_dn8;
        *var_mtat__blk1563_dn9_slot = var_mtat__blk1563_dn9;
        *var_terfc__blk1526_slot = var_terfc__blk1526;
        *var_terfc__blk1526_db0_slot = var_terfc__blk1526_db0;
        *var_terfc__blk1526_db1_slot = var_terfc__blk1526_db1;
        *var_terfc__blk1526_db2_slot = var_terfc__blk1526_db2;
        *var_terfc__blk1526_db3_slot = var_terfc__blk1526_db3;
        *var_terfc__blk1526_db4_slot = var_terfc__blk1526_db4;
        *var_terfc__blk1526_db5_slot = var_terfc__blk1526_db5;
        *var_terfc__blk1526_db6_slot = var_terfc__blk1526_db6;
        *var_terfc__blk1526_dn0_slot = var_terfc__blk1526_dn0;
        *var_terfc__blk1526_dn1_slot = var_terfc__blk1526_dn1;
        *var_terfc__blk1526_dn10_slot = var_terfc__blk1526_dn10;
        *var_terfc__blk1526_dn11_slot = var_terfc__blk1526_dn11;
        *var_terfc__blk1526_dn2_slot = var_terfc__blk1526_dn2;
        *var_terfc__blk1526_dn3_slot = var_terfc__blk1526_dn3;
        *var_terfc__blk1526_dn4_slot = var_terfc__blk1526_dn4;
        *var_terfc__blk1526_dn5_slot = var_terfc__blk1526_dn5;
        *var_terfc__blk1526_dn6_slot = var_terfc__blk1526_dn6;
        *var_terfc__blk1526_dn7_slot = var_terfc__blk1526_dn7;
        *var_terfc__blk1526_dn8_slot = var_terfc__blk1526_dn8;
        *var_terfc__blk1526_dn9_slot = var_terfc__blk1526_dn9;
        *var_tmp__blk1543_slot = var_tmp__blk1543;
        *var_tmp__blk1543_db0_slot = var_tmp__blk1543_db0;
        *var_tmp__blk1543_db1_slot = var_tmp__blk1543_db1;
        *var_tmp__blk1543_db2_slot = var_tmp__blk1543_db2;
        *var_tmp__blk1543_db3_slot = var_tmp__blk1543_db3;
        *var_tmp__blk1543_db4_slot = var_tmp__blk1543_db4;
        *var_tmp__blk1543_db5_slot = var_tmp__blk1543_db5;
        *var_tmp__blk1543_db6_slot = var_tmp__blk1543_db6;
        *var_tmp__blk1543_dn0_slot = var_tmp__blk1543_dn0;
        *var_tmp__blk1543_dn1_slot = var_tmp__blk1543_dn1;
        *var_tmp__blk1543_dn10_slot = var_tmp__blk1543_dn10;
        *var_tmp__blk1543_dn11_slot = var_tmp__blk1543_dn11;
        *var_tmp__blk1543_dn2_slot = var_tmp__blk1543_dn2;
        *var_tmp__blk1543_dn3_slot = var_tmp__blk1543_dn3;
        *var_tmp__blk1543_dn4_slot = var_tmp__blk1543_dn4;
        *var_tmp__blk1543_dn5_slot = var_tmp__blk1543_dn5;
        *var_tmp__blk1543_dn6_slot = var_tmp__blk1543_dn6;
        *var_tmp__blk1543_dn7_slot = var_tmp__blk1543_dn7;
        *var_tmp__blk1543_dn8_slot = var_tmp__blk1543_dn8;
        *var_tmp__blk1543_dn9_slot = var_tmp__blk1543_dn9;
        *var_umaxpoweronepointfive__blk1558_slot = var_umaxpoweronepointfive__blk1558;
        *var_umaxpoweronepointfive__blk1558_db0_slot = var_umaxpoweronepointfive__blk1558_db0;
        *var_umaxpoweronepointfive__blk1558_db1_slot = var_umaxpoweronepointfive__blk1558_db1;
        *var_umaxpoweronepointfive__blk1558_db2_slot = var_umaxpoweronepointfive__blk1558_db2;
        *var_umaxpoweronepointfive__blk1558_db3_slot = var_umaxpoweronepointfive__blk1558_db3;
        *var_umaxpoweronepointfive__blk1558_db4_slot = var_umaxpoweronepointfive__blk1558_db4;
        *var_umaxpoweronepointfive__blk1558_db5_slot = var_umaxpoweronepointfive__blk1558_db5;
        *var_umaxpoweronepointfive__blk1558_db6_slot = var_umaxpoweronepointfive__blk1558_db6;
        *var_umaxpoweronepointfive__blk1558_dn0_slot = var_umaxpoweronepointfive__blk1558_dn0;
        *var_umaxpoweronepointfive__blk1558_dn1_slot = var_umaxpoweronepointfive__blk1558_dn1;
        *var_umaxpoweronepointfive__blk1558_dn10_slot = var_umaxpoweronepointfive__blk1558_dn10;
        *var_umaxpoweronepointfive__blk1558_dn11_slot = var_umaxpoweronepointfive__blk1558_dn11;
        *var_umaxpoweronepointfive__blk1558_dn2_slot = var_umaxpoweronepointfive__blk1558_dn2;
        *var_umaxpoweronepointfive__blk1558_dn3_slot = var_umaxpoweronepointfive__blk1558_dn3;
        *var_umaxpoweronepointfive__blk1558_dn4_slot = var_umaxpoweronepointfive__blk1558_dn4;
        *var_umaxpoweronepointfive__blk1558_dn5_slot = var_umaxpoweronepointfive__blk1558_dn5;
        *var_umaxpoweronepointfive__blk1558_dn6_slot = var_umaxpoweronepointfive__blk1558_dn6;
        *var_umaxpoweronepointfive__blk1558_dn7_slot = var_umaxpoweronepointfive__blk1558_dn7;
        *var_umaxpoweronepointfive__blk1558_dn8_slot = var_umaxpoweronepointfive__blk1558_dn8;
        *var_umaxpoweronepointfive__blk1558_dn9_slot = var_umaxpoweronepointfive__blk1558_dn9;
        *var_wgamma__blk1559_slot = var_wgamma__blk1559;
        *var_wgamma__blk1559_db0_slot = var_wgamma__blk1559_db0;
        *var_wgamma__blk1559_db1_slot = var_wgamma__blk1559_db1;
        *var_wgamma__blk1559_db2_slot = var_wgamma__blk1559_db2;
        *var_wgamma__blk1559_db3_slot = var_wgamma__blk1559_db3;
        *var_wgamma__blk1559_db4_slot = var_wgamma__blk1559_db4;
        *var_wgamma__blk1559_db5_slot = var_wgamma__blk1559_db5;
        *var_wgamma__blk1559_db6_slot = var_wgamma__blk1559_db6;
        *var_wgamma__blk1559_dn0_slot = var_wgamma__blk1559_dn0;
        *var_wgamma__blk1559_dn1_slot = var_wgamma__blk1559_dn1;
        *var_wgamma__blk1559_dn10_slot = var_wgamma__blk1559_dn10;
        *var_wgamma__blk1559_dn11_slot = var_wgamma__blk1559_dn11;
        *var_wgamma__blk1559_dn2_slot = var_wgamma__blk1559_dn2;
        *var_wgamma__blk1559_dn3_slot = var_wgamma__blk1559_dn3;
        *var_wgamma__blk1559_dn4_slot = var_wgamma__blk1559_dn4;
        *var_wgamma__blk1559_dn5_slot = var_wgamma__blk1559_dn5;
        *var_wgamma__blk1559_dn6_slot = var_wgamma__blk1559_dn6;
        *var_wgamma__blk1559_dn7_slot = var_wgamma__blk1559_dn7;
        *var_wgamma__blk1559_dn8_slot = var_wgamma__blk1559_dn8;
        *var_wgamma__blk1559_dn9_slot = var_wgamma__blk1559_dn9;
        *var_wtat__blk1560_slot = var_wtat__blk1560;
        *var_wtat__blk1560_db0_slot = var_wtat__blk1560_db0;
        *var_wtat__blk1560_db1_slot = var_wtat__blk1560_db1;
        *var_wtat__blk1560_db2_slot = var_wtat__blk1560_db2;
        *var_wtat__blk1560_db3_slot = var_wtat__blk1560_db3;
        *var_wtat__blk1560_db4_slot = var_wtat__blk1560_db4;
        *var_wtat__blk1560_db5_slot = var_wtat__blk1560_db5;
        *var_wtat__blk1560_db6_slot = var_wtat__blk1560_db6;
        *var_wtat__blk1560_dn0_slot = var_wtat__blk1560_dn0;
        *var_wtat__blk1560_dn1_slot = var_wtat__blk1560_dn1;
        *var_wtat__blk1560_dn10_slot = var_wtat__blk1560_dn10;
        *var_wtat__blk1560_dn11_slot = var_wtat__blk1560_dn11;
        *var_wtat__blk1560_dn2_slot = var_wtat__blk1560_dn2;
        *var_wtat__blk1560_dn3_slot = var_wtat__blk1560_dn3;
        *var_wtat__blk1560_dn4_slot = var_wtat__blk1560_dn4;
        *var_wtat__blk1560_dn5_slot = var_wtat__blk1560_dn5;
        *var_wtat__blk1560_dn6_slot = var_wtat__blk1560_dn6;
        *var_wtat__blk1560_dn7_slot = var_wtat__blk1560_dn7;
        *var_wtat__blk1560_dn8_slot = var_wtat__blk1560_dn8;
        *var_wtat__blk1560_dn9_slot = var_wtat__blk1560_dn9;
        *var_xerfc__blk1564_slot = var_xerfc__blk1564;
        *var_xerfc__blk1564_db0_slot = var_xerfc__blk1564_db0;
        *var_xerfc__blk1564_db1_slot = var_xerfc__blk1564_db1;
        *var_xerfc__blk1564_db2_slot = var_xerfc__blk1564_db2;
        *var_xerfc__blk1564_db3_slot = var_xerfc__blk1564_db3;
        *var_xerfc__blk1564_db4_slot = var_xerfc__blk1564_db4;
        *var_xerfc__blk1564_db5_slot = var_xerfc__blk1564_db5;
        *var_xerfc__blk1564_db6_slot = var_xerfc__blk1564_db6;
        *var_xerfc__blk1564_dn0_slot = var_xerfc__blk1564_dn0;
        *var_xerfc__blk1564_dn1_slot = var_xerfc__blk1564_dn1;
        *var_xerfc__blk1564_dn10_slot = var_xerfc__blk1564_dn10;
        *var_xerfc__blk1564_dn11_slot = var_xerfc__blk1564_dn11;
        *var_xerfc__blk1564_dn2_slot = var_xerfc__blk1564_dn2;
        *var_xerfc__blk1564_dn3_slot = var_xerfc__blk1564_dn3;
        *var_xerfc__blk1564_dn4_slot = var_xerfc__blk1564_dn4;
        *var_xerfc__blk1564_dn5_slot = var_xerfc__blk1564_dn5;
        *var_xerfc__blk1564_dn6_slot = var_xerfc__blk1564_dn6;
        *var_xerfc__blk1564_dn7_slot = var_xerfc__blk1564_dn7;
        *var_xerfc__blk1564_dn8_slot = var_xerfc__blk1564_dn8;
        *var_xerfc__blk1564_dn9_slot = var_xerfc__blk1564_dn9;
        *var_ysq__blk1525_slot = var_ysq__blk1525;
        *var_ysq__blk1525_db0_slot = var_ysq__blk1525_db0;
        *var_ysq__blk1525_db1_slot = var_ysq__blk1525_db1;
        *var_ysq__blk1525_db2_slot = var_ysq__blk1525_db2;
        *var_ysq__blk1525_db3_slot = var_ysq__blk1525_db3;
        *var_ysq__blk1525_db4_slot = var_ysq__blk1525_db4;
        *var_ysq__blk1525_db5_slot = var_ysq__blk1525_db5;
        *var_ysq__blk1525_db6_slot = var_ysq__blk1525_db6;
        *var_ysq__blk1525_dn0_slot = var_ysq__blk1525_dn0;
        *var_ysq__blk1525_dn1_slot = var_ysq__blk1525_dn1;
        *var_ysq__blk1525_dn10_slot = var_ysq__blk1525_dn10;
        *var_ysq__blk1525_dn11_slot = var_ysq__blk1525_dn11;
        *var_ysq__blk1525_dn2_slot = var_ysq__blk1525_dn2;
        *var_ysq__blk1525_dn3_slot = var_ysq__blk1525_dn3;
        *var_ysq__blk1525_dn4_slot = var_ysq__blk1525_dn4;
        *var_ysq__blk1525_dn5_slot = var_ysq__blk1525_dn5;
        *var_ysq__blk1525_dn6_slot = var_ysq__blk1525_dn6;
        *var_ysq__blk1525_dn7_slot = var_ysq__blk1525_dn7;
        *var_ysq__blk1525_dn8_slot = var_ysq__blk1525_dn8;
        *var_ysq__blk1525_dn9_slot = var_ysq__blk1525_dn9;
    }

    pub(super) fn stamp_transient_block_241(
        var_alphaav: f64,
        var_asrh__blk1551: f64,
        var_asrh__blk1551_db0: f64,
        var_asrh__blk1551_db1: f64,
        var_asrh__blk1551_db2: f64,
        var_asrh__blk1551_db3: f64,
        var_asrh__blk1551_db4: f64,
        var_asrh__blk1551_db5: f64,
        var_asrh__blk1551_db6: f64,
        var_asrh__blk1551_dn0: f64,
        var_asrh__blk1551_dn1: f64,
        var_asrh__blk1551_dn10: f64,
        var_asrh__blk1551_dn11: f64,
        var_asrh__blk1551_dn2: f64,
        var_asrh__blk1551_dn3: f64,
        var_asrh__blk1551_dn4: f64,
        var_asrh__blk1551_dn5: f64,
        var_asrh__blk1551_dn6: f64,
        var_asrh__blk1551_dn7: f64,
        var_asrh__blk1551_dn8: f64,
        var_asrh__blk1551_dn9: f64,
        var_atatgat_d: f64,
        var_cbbtgatd_i: f64,
        var_ctatgatd_i: f64,
        var_erfcpos__blk1527: f64,
        var_erfcpos__blk1527_db0: f64,
        var_erfcpos__blk1527_db1: f64,
        var_erfcpos__blk1527_db2: f64,
        var_erfcpos__blk1527_db3: f64,
        var_erfcpos__blk1527_db4: f64,
        var_erfcpos__blk1527_db5: f64,
        var_erfcpos__blk1527_db6: f64,
        var_erfcpos__blk1527_dn0: f64,
        var_erfcpos__blk1527_dn1: f64,
        var_erfcpos__blk1527_dn10: f64,
        var_erfcpos__blk1527_dn11: f64,
        var_erfcpos__blk1527_dn2: f64,
        var_erfcpos__blk1527_dn3: f64,
        var_erfcpos__blk1527_dn4: f64,
        var_erfcpos__blk1527_dn5: f64,
        var_erfcpos__blk1527_dn6: f64,
        var_erfcpos__blk1527_dn7: f64,
        var_erfcpos__blk1527_dn8: f64,
        var_erfcpos__blk1527_dn9: f64,
        var_fbbtgat_d: f64,
        var_fbbtgat_d_db0: f64,
        var_fbbtgat_d_db1: f64,
        var_fbbtgat_d_db2: f64,
        var_fbbtgat_d_db3: f64,
        var_fbbtgat_d_db4: f64,
        var_fbbtgat_d_db5: f64,
        var_fbbtgat_d_db6: f64,
        var_fbbtgat_d_dn0: f64,
        var_fbbtgat_d_dn1: f64,
        var_fbbtgat_d_dn10: f64,
        var_fbbtgat_d_dn11: f64,
        var_fbbtgat_d_dn2: f64,
        var_fbbtgat_d_dn3: f64,
        var_fbbtgat_d_dn4: f64,
        var_fbbtgat_d_dn5: f64,
        var_fbbtgat_d_dn6: f64,
        var_fbbtgat_d_dn7: f64,
        var_fbbtgat_d_dn8: f64,
        var_fbbtgat_d_dn9: f64,
        var_guard1572: f64,
        var_guard1573: f64,
        var_guard1697: f64,
        var_guard1701: f64,
        var_guard1705: f64,
        var_guard1706: f64,
        var_ktat__blk1561: f64,
        var_ktat__blk1561_db0: f64,
        var_ktat__blk1561_db1: f64,
        var_ktat__blk1561_db2: f64,
        var_ktat__blk1561_db3: f64,
        var_ktat__blk1561_db4: f64,
        var_ktat__blk1561_db5: f64,
        var_ktat__blk1561_db6: f64,
        var_ktat__blk1561_dn0: f64,
        var_ktat__blk1561_dn1: f64,
        var_ktat__blk1561_dn10: f64,
        var_ktat__blk1561_dn11: f64,
        var_ktat__blk1561_dn2: f64,
        var_ktat__blk1561_dn3: f64,
        var_ktat__blk1561_dn4: f64,
        var_ktat__blk1561_dn5: f64,
        var_ktat__blk1561_dn6: f64,
        var_ktat__blk1561_dn7: f64,
        var_ktat__blk1561_dn8: f64,
        var_ktat__blk1561_dn9: f64,
        var_mtat__blk1563: f64,
        var_mtat__blk1563_db0: f64,
        var_mtat__blk1563_db1: f64,
        var_mtat__blk1563_db2: f64,
        var_mtat__blk1563_db3: f64,
        var_mtat__blk1563_db4: f64,
        var_mtat__blk1563_db5: f64,
        var_mtat__blk1563_db6: f64,
        var_mtat__blk1563_dn0: f64,
        var_mtat__blk1563_dn1: f64,
        var_mtat__blk1563_dn10: f64,
        var_mtat__blk1563_dn11: f64,
        var_mtat__blk1563_dn2: f64,
        var_mtat__blk1563_dn3: f64,
        var_mtat__blk1563_dn4: f64,
        var_mtat__blk1563_dn5: f64,
        var_mtat__blk1563_dn6: f64,
        var_mtat__blk1563_dn7: f64,
        var_mtat__blk1563_dn8: f64,
        var_mtat__blk1563_dn9: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_pbrgatd_i: f64,
        var_pgatd_i: f64,
        var_vav__blk1542: f64,
        var_vbbt__blk1541: f64,
        var_vbbt__blk1541_db0: f64,
        var_vbbt__blk1541_db1: f64,
        var_vbbt__blk1541_db2: f64,
        var_vbbt__blk1541_db3: f64,
        var_vbbt__blk1541_db4: f64,
        var_vbbt__blk1541_db5: f64,
        var_vbbt__blk1541_db6: f64,
        var_vbbt__blk1541_dn0: f64,
        var_vbbt__blk1541_dn1: f64,
        var_vbbt__blk1541_dn10: f64,
        var_vbbt__blk1541_dn11: f64,
        var_vbbt__blk1541_dn2: f64,
        var_vbbt__blk1541_dn3: f64,
        var_vbbt__blk1541_dn4: f64,
        var_vbbt__blk1541_dn5: f64,
        var_vbbt__blk1541_dn6: f64,
        var_vbbt__blk1541_dn7: f64,
        var_vbbt__blk1541_dn8: f64,
        var_vbbt__blk1541_dn9: f64,
        var_vbirgatd_i: f64,
        var_vbirgatinv_d: f64,
        var_vbrgat_var_d: f64,
        var_vjun_d: f64,
        var_vjun_d_db0: f64,
        var_vjun_d_db1: f64,
        var_vjun_d_db2: f64,
        var_vjun_d_db3: f64,
        var_vjun_d_db4: f64,
        var_vjun_d_db5: f64,
        var_vjun_d_db6: f64,
        var_vjun_d_dn0: f64,
        var_vjun_d_dn1: f64,
        var_vjun_d_dn10: f64,
        var_vjun_d_dn11: f64,
        var_vjun_d_dn2: f64,
        var_vjun_d_dn3: f64,
        var_vjun_d_dn4: f64,
        var_vjun_d_dn5: f64,
        var_vjun_d_dn6: f64,
        var_vjun_d_dn7: f64,
        var_vjun_d_dn8: f64,
        var_vjun_d_dn9: f64,
        var_wdepnulrinvgat_d: f64,
        var_wtat__blk1560: f64,
        var_wtat__blk1560_db0: f64,
        var_wtat__blk1560_db1: f64,
        var_wtat__blk1560_db2: f64,
        var_wtat__blk1560_db3: f64,
        var_wtat__blk1560_db4: f64,
        var_wtat__blk1560_db5: f64,
        var_wtat__blk1560_db6: f64,
        var_wtat__blk1560_dn0: f64,
        var_wtat__blk1560_dn1: f64,
        var_wtat__blk1560_dn10: f64,
        var_wtat__blk1560_dn11: f64,
        var_wtat__blk1560_dn2: f64,
        var_wtat__blk1560_dn3: f64,
        var_wtat__blk1560_dn4: f64,
        var_wtat__blk1560_dn5: f64,
        var_wtat__blk1560_dn6: f64,
        var_wtat__blk1560_dn7: f64,
        var_wtat__blk1560_dn8: f64,
        var_wtat__blk1560_dn9: f64,
        var_erfctimesexpmtat__blk1565_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_db0_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_db1_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_db2_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_db3_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_db4_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_db5_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_db6_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn0_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn1_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn10_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn11_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn2_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn3_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn4_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn5_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn6_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn7_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn8_slot: &mut f64,
        var_erfctimesexpmtat__blk1565_dn9_slot: &mut f64,
        var_fbreakdown__blk1569_slot: &mut f64,
        var_fbreakdown__blk1569_db0_slot: &mut f64,
        var_fbreakdown__blk1569_db1_slot: &mut f64,
        var_fbreakdown__blk1569_db2_slot: &mut f64,
        var_fbreakdown__blk1569_db3_slot: &mut f64,
        var_fbreakdown__blk1569_db4_slot: &mut f64,
        var_fbreakdown__blk1569_db5_slot: &mut f64,
        var_fbreakdown__blk1569_db6_slot: &mut f64,
        var_fbreakdown__blk1569_dn0_slot: &mut f64,
        var_fbreakdown__blk1569_dn1_slot: &mut f64,
        var_fbreakdown__blk1569_dn10_slot: &mut f64,
        var_fbreakdown__blk1569_dn11_slot: &mut f64,
        var_fbreakdown__blk1569_dn2_slot: &mut f64,
        var_fbreakdown__blk1569_dn3_slot: &mut f64,
        var_fbreakdown__blk1569_dn4_slot: &mut f64,
        var_fbreakdown__blk1569_dn5_slot: &mut f64,
        var_fbreakdown__blk1569_dn6_slot: &mut f64,
        var_fbreakdown__blk1569_dn7_slot: &mut f64,
        var_fbreakdown__blk1569_dn8_slot: &mut f64,
        var_fbreakdown__blk1569_dn9_slot: &mut f64,
        var_fmaxr__blk1568_slot: &mut f64,
        var_fmaxr__blk1568_db0_slot: &mut f64,
        var_fmaxr__blk1568_db1_slot: &mut f64,
        var_fmaxr__blk1568_db2_slot: &mut f64,
        var_fmaxr__blk1568_db3_slot: &mut f64,
        var_fmaxr__blk1568_db4_slot: &mut f64,
        var_fmaxr__blk1568_db5_slot: &mut f64,
        var_fmaxr__blk1568_db6_slot: &mut f64,
        var_fmaxr__blk1568_dn0_slot: &mut f64,
        var_fmaxr__blk1568_dn1_slot: &mut f64,
        var_fmaxr__blk1568_dn10_slot: &mut f64,
        var_fmaxr__blk1568_dn11_slot: &mut f64,
        var_fmaxr__blk1568_dn2_slot: &mut f64,
        var_fmaxr__blk1568_dn3_slot: &mut f64,
        var_fmaxr__blk1568_dn4_slot: &mut f64,
        var_fmaxr__blk1568_dn5_slot: &mut f64,
        var_fmaxr__blk1568_dn6_slot: &mut f64,
        var_fmaxr__blk1568_dn7_slot: &mut f64,
        var_fmaxr__blk1568_dn8_slot: &mut f64,
        var_fmaxr__blk1568_dn9_slot: &mut f64,
        var_gammamax__blk1566_slot: &mut f64,
        var_gammamax__blk1566_db0_slot: &mut f64,
        var_gammamax__blk1566_db1_slot: &mut f64,
        var_gammamax__blk1566_db2_slot: &mut f64,
        var_gammamax__blk1566_db3_slot: &mut f64,
        var_gammamax__blk1566_db4_slot: &mut f64,
        var_gammamax__blk1566_db5_slot: &mut f64,
        var_gammamax__blk1566_db6_slot: &mut f64,
        var_gammamax__blk1566_dn0_slot: &mut f64,
        var_gammamax__blk1566_dn1_slot: &mut f64,
        var_gammamax__blk1566_dn10_slot: &mut f64,
        var_gammamax__blk1566_dn11_slot: &mut f64,
        var_gammamax__blk1566_dn2_slot: &mut f64,
        var_gammamax__blk1566_dn3_slot: &mut f64,
        var_gammamax__blk1566_dn4_slot: &mut f64,
        var_gammamax__blk1566_dn5_slot: &mut f64,
        var_gammamax__blk1566_dn6_slot: &mut f64,
        var_gammamax__blk1566_dn7_slot: &mut f64,
        var_gammamax__blk1566_dn8_slot: &mut f64,
        var_gammamax__blk1566_dn9_slot: &mut f64,
        var_guard1707_slot: &mut f64,
        var_guard1708_slot: &mut f64,
        var_guard1709_slot: &mut f64,
        var_guard1710_slot: &mut f64,
        var_guard1711_slot: &mut f64,
        var_guard1712_slot: &mut f64,
        var_guard1713_slot: &mut f64,
        var_ibbt__blk1567_slot: &mut f64,
        var_ibbt__blk1567_db0_slot: &mut f64,
        var_ibbt__blk1567_db1_slot: &mut f64,
        var_ibbt__blk1567_db2_slot: &mut f64,
        var_ibbt__blk1567_db3_slot: &mut f64,
        var_ibbt__blk1567_db4_slot: &mut f64,
        var_ibbt__blk1567_db5_slot: &mut f64,
        var_ibbt__blk1567_db6_slot: &mut f64,
        var_ibbt__blk1567_dn0_slot: &mut f64,
        var_ibbt__blk1567_dn1_slot: &mut f64,
        var_ibbt__blk1567_dn10_slot: &mut f64,
        var_ibbt__blk1567_dn11_slot: &mut f64,
        var_ibbt__blk1567_dn2_slot: &mut f64,
        var_ibbt__blk1567_dn3_slot: &mut f64,
        var_ibbt__blk1567_dn4_slot: &mut f64,
        var_ibbt__blk1567_dn5_slot: &mut f64,
        var_ibbt__blk1567_dn6_slot: &mut f64,
        var_ibbt__blk1567_dn7_slot: &mut f64,
        var_ibbt__blk1567_dn8_slot: &mut f64,
        var_ibbt__blk1567_dn9_slot: &mut f64,
        var_itat__blk1552_slot: &mut f64,
        var_itat__blk1552_db0_slot: &mut f64,
        var_itat__blk1552_db1_slot: &mut f64,
        var_itat__blk1552_db2_slot: &mut f64,
        var_itat__blk1552_db3_slot: &mut f64,
        var_itat__blk1552_db4_slot: &mut f64,
        var_itat__blk1552_db5_slot: &mut f64,
        var_itat__blk1552_db6_slot: &mut f64,
        var_itat__blk1552_dn0_slot: &mut f64,
        var_itat__blk1552_dn1_slot: &mut f64,
        var_itat__blk1552_dn10_slot: &mut f64,
        var_itat__blk1552_dn11_slot: &mut f64,
        var_itat__blk1552_dn2_slot: &mut f64,
        var_itat__blk1552_dn3_slot: &mut f64,
        var_itat__blk1552_dn4_slot: &mut f64,
        var_itat__blk1552_dn5_slot: &mut f64,
        var_itat__blk1552_dn6_slot: &mut f64,
        var_itat__blk1552_dn7_slot: &mut f64,
        var_itat__blk1552_dn8_slot: &mut f64,
        var_itat__blk1552_dn9_slot: &mut f64,
        var_tmp__blk1543_slot: &mut f64,
        var_tmp__blk1543_db0_slot: &mut f64,
        var_tmp__blk1543_db1_slot: &mut f64,
        var_tmp__blk1543_db2_slot: &mut f64,
        var_tmp__blk1543_db3_slot: &mut f64,
        var_tmp__blk1543_db4_slot: &mut f64,
        var_tmp__blk1543_db5_slot: &mut f64,
        var_tmp__blk1543_db6_slot: &mut f64,
        var_tmp__blk1543_dn0_slot: &mut f64,
        var_tmp__blk1543_dn1_slot: &mut f64,
        var_tmp__blk1543_dn10_slot: &mut f64,
        var_tmp__blk1543_dn11_slot: &mut f64,
        var_tmp__blk1543_dn2_slot: &mut f64,
        var_tmp__blk1543_dn3_slot: &mut f64,
        var_tmp__blk1543_dn4_slot: &mut f64,
        var_tmp__blk1543_dn5_slot: &mut f64,
        var_tmp__blk1543_dn6_slot: &mut f64,
        var_tmp__blk1543_dn7_slot: &mut f64,
        var_tmp__blk1543_dn8_slot: &mut f64,
        var_tmp__blk1543_dn9_slot: &mut f64,
    ) {
        let mut var_erfctimesexpmtat__blk1565: f64 = *var_erfctimesexpmtat__blk1565_slot;
        let mut var_erfctimesexpmtat__blk1565_db0: f64 = *var_erfctimesexpmtat__blk1565_db0_slot;
        let mut var_erfctimesexpmtat__blk1565_db1: f64 = *var_erfctimesexpmtat__blk1565_db1_slot;
        let mut var_erfctimesexpmtat__blk1565_db2: f64 = *var_erfctimesexpmtat__blk1565_db2_slot;
        let mut var_erfctimesexpmtat__blk1565_db3: f64 = *var_erfctimesexpmtat__blk1565_db3_slot;
        let mut var_erfctimesexpmtat__blk1565_db4: f64 = *var_erfctimesexpmtat__blk1565_db4_slot;
        let mut var_erfctimesexpmtat__blk1565_db5: f64 = *var_erfctimesexpmtat__blk1565_db5_slot;
        let mut var_erfctimesexpmtat__blk1565_db6: f64 = *var_erfctimesexpmtat__blk1565_db6_slot;
        let mut var_erfctimesexpmtat__blk1565_dn0: f64 = *var_erfctimesexpmtat__blk1565_dn0_slot;
        let mut var_erfctimesexpmtat__blk1565_dn1: f64 = *var_erfctimesexpmtat__blk1565_dn1_slot;
        let mut var_erfctimesexpmtat__blk1565_dn10: f64 = *var_erfctimesexpmtat__blk1565_dn10_slot;
        let mut var_erfctimesexpmtat__blk1565_dn11: f64 = *var_erfctimesexpmtat__blk1565_dn11_slot;
        let mut var_erfctimesexpmtat__blk1565_dn2: f64 = *var_erfctimesexpmtat__blk1565_dn2_slot;
        let mut var_erfctimesexpmtat__blk1565_dn3: f64 = *var_erfctimesexpmtat__blk1565_dn3_slot;
        let mut var_erfctimesexpmtat__blk1565_dn4: f64 = *var_erfctimesexpmtat__blk1565_dn4_slot;
        let mut var_erfctimesexpmtat__blk1565_dn5: f64 = *var_erfctimesexpmtat__blk1565_dn5_slot;
        let mut var_erfctimesexpmtat__blk1565_dn6: f64 = *var_erfctimesexpmtat__blk1565_dn6_slot;
        let mut var_erfctimesexpmtat__blk1565_dn7: f64 = *var_erfctimesexpmtat__blk1565_dn7_slot;
        let mut var_erfctimesexpmtat__blk1565_dn8: f64 = *var_erfctimesexpmtat__blk1565_dn8_slot;
        let mut var_erfctimesexpmtat__blk1565_dn9: f64 = *var_erfctimesexpmtat__blk1565_dn9_slot;
        let mut var_fbreakdown__blk1569: f64 = *var_fbreakdown__blk1569_slot;
        let mut var_fbreakdown__blk1569_db0: f64 = *var_fbreakdown__blk1569_db0_slot;
        let mut var_fbreakdown__blk1569_db1: f64 = *var_fbreakdown__blk1569_db1_slot;
        let mut var_fbreakdown__blk1569_db2: f64 = *var_fbreakdown__blk1569_db2_slot;
        let mut var_fbreakdown__blk1569_db3: f64 = *var_fbreakdown__blk1569_db3_slot;
        let mut var_fbreakdown__blk1569_db4: f64 = *var_fbreakdown__blk1569_db4_slot;
        let mut var_fbreakdown__blk1569_db5: f64 = *var_fbreakdown__blk1569_db5_slot;
        let mut var_fbreakdown__blk1569_db6: f64 = *var_fbreakdown__blk1569_db6_slot;
        let mut var_fbreakdown__blk1569_dn0: f64 = *var_fbreakdown__blk1569_dn0_slot;
        let mut var_fbreakdown__blk1569_dn1: f64 = *var_fbreakdown__blk1569_dn1_slot;
        let mut var_fbreakdown__blk1569_dn10: f64 = *var_fbreakdown__blk1569_dn10_slot;
        let mut var_fbreakdown__blk1569_dn11: f64 = *var_fbreakdown__blk1569_dn11_slot;
        let mut var_fbreakdown__blk1569_dn2: f64 = *var_fbreakdown__blk1569_dn2_slot;
        let mut var_fbreakdown__blk1569_dn3: f64 = *var_fbreakdown__blk1569_dn3_slot;
        let mut var_fbreakdown__blk1569_dn4: f64 = *var_fbreakdown__blk1569_dn4_slot;
        let mut var_fbreakdown__blk1569_dn5: f64 = *var_fbreakdown__blk1569_dn5_slot;
        let mut var_fbreakdown__blk1569_dn6: f64 = *var_fbreakdown__blk1569_dn6_slot;
        let mut var_fbreakdown__blk1569_dn7: f64 = *var_fbreakdown__blk1569_dn7_slot;
        let mut var_fbreakdown__blk1569_dn8: f64 = *var_fbreakdown__blk1569_dn8_slot;
        let mut var_fbreakdown__blk1569_dn9: f64 = *var_fbreakdown__blk1569_dn9_slot;
        let mut var_fmaxr__blk1568: f64 = *var_fmaxr__blk1568_slot;
        let mut var_fmaxr__blk1568_db0: f64 = *var_fmaxr__blk1568_db0_slot;
        let mut var_fmaxr__blk1568_db1: f64 = *var_fmaxr__blk1568_db1_slot;
        let mut var_fmaxr__blk1568_db2: f64 = *var_fmaxr__blk1568_db2_slot;
        let mut var_fmaxr__blk1568_db3: f64 = *var_fmaxr__blk1568_db3_slot;
        let mut var_fmaxr__blk1568_db4: f64 = *var_fmaxr__blk1568_db4_slot;
        let mut var_fmaxr__blk1568_db5: f64 = *var_fmaxr__blk1568_db5_slot;
        let mut var_fmaxr__blk1568_db6: f64 = *var_fmaxr__blk1568_db6_slot;
        let mut var_fmaxr__blk1568_dn0: f64 = *var_fmaxr__blk1568_dn0_slot;
        let mut var_fmaxr__blk1568_dn1: f64 = *var_fmaxr__blk1568_dn1_slot;
        let mut var_fmaxr__blk1568_dn10: f64 = *var_fmaxr__blk1568_dn10_slot;
        let mut var_fmaxr__blk1568_dn11: f64 = *var_fmaxr__blk1568_dn11_slot;
        let mut var_fmaxr__blk1568_dn2: f64 = *var_fmaxr__blk1568_dn2_slot;
        let mut var_fmaxr__blk1568_dn3: f64 = *var_fmaxr__blk1568_dn3_slot;
        let mut var_fmaxr__blk1568_dn4: f64 = *var_fmaxr__blk1568_dn4_slot;
        let mut var_fmaxr__blk1568_dn5: f64 = *var_fmaxr__blk1568_dn5_slot;
        let mut var_fmaxr__blk1568_dn6: f64 = *var_fmaxr__blk1568_dn6_slot;
        let mut var_fmaxr__blk1568_dn7: f64 = *var_fmaxr__blk1568_dn7_slot;
        let mut var_fmaxr__blk1568_dn8: f64 = *var_fmaxr__blk1568_dn8_slot;
        let mut var_fmaxr__blk1568_dn9: f64 = *var_fmaxr__blk1568_dn9_slot;
        let mut var_gammamax__blk1566: f64 = *var_gammamax__blk1566_slot;
        let mut var_gammamax__blk1566_db0: f64 = *var_gammamax__blk1566_db0_slot;
        let mut var_gammamax__blk1566_db1: f64 = *var_gammamax__blk1566_db1_slot;
        let mut var_gammamax__blk1566_db2: f64 = *var_gammamax__blk1566_db2_slot;
        let mut var_gammamax__blk1566_db3: f64 = *var_gammamax__blk1566_db3_slot;
        let mut var_gammamax__blk1566_db4: f64 = *var_gammamax__blk1566_db4_slot;
        let mut var_gammamax__blk1566_db5: f64 = *var_gammamax__blk1566_db5_slot;
        let mut var_gammamax__blk1566_db6: f64 = *var_gammamax__blk1566_db6_slot;
        let mut var_gammamax__blk1566_dn0: f64 = *var_gammamax__blk1566_dn0_slot;
        let mut var_gammamax__blk1566_dn1: f64 = *var_gammamax__blk1566_dn1_slot;
        let mut var_gammamax__blk1566_dn10: f64 = *var_gammamax__blk1566_dn10_slot;
        let mut var_gammamax__blk1566_dn11: f64 = *var_gammamax__blk1566_dn11_slot;
        let mut var_gammamax__blk1566_dn2: f64 = *var_gammamax__blk1566_dn2_slot;
        let mut var_gammamax__blk1566_dn3: f64 = *var_gammamax__blk1566_dn3_slot;
        let mut var_gammamax__blk1566_dn4: f64 = *var_gammamax__blk1566_dn4_slot;
        let mut var_gammamax__blk1566_dn5: f64 = *var_gammamax__blk1566_dn5_slot;
        let mut var_gammamax__blk1566_dn6: f64 = *var_gammamax__blk1566_dn6_slot;
        let mut var_gammamax__blk1566_dn7: f64 = *var_gammamax__blk1566_dn7_slot;
        let mut var_gammamax__blk1566_dn8: f64 = *var_gammamax__blk1566_dn8_slot;
        let mut var_gammamax__blk1566_dn9: f64 = *var_gammamax__blk1566_dn9_slot;
        let mut var_guard1707: f64 = *var_guard1707_slot;
        let mut var_guard1708: f64 = *var_guard1708_slot;
        let mut var_guard1709: f64 = *var_guard1709_slot;
        let mut var_guard1710: f64 = *var_guard1710_slot;
        let mut var_guard1711: f64 = *var_guard1711_slot;
        let mut var_guard1712: f64 = *var_guard1712_slot;
        let mut var_guard1713: f64 = *var_guard1713_slot;
        let mut var_ibbt__blk1567: f64 = *var_ibbt__blk1567_slot;
        let mut var_ibbt__blk1567_db0: f64 = *var_ibbt__blk1567_db0_slot;
        let mut var_ibbt__blk1567_db1: f64 = *var_ibbt__blk1567_db1_slot;
        let mut var_ibbt__blk1567_db2: f64 = *var_ibbt__blk1567_db2_slot;
        let mut var_ibbt__blk1567_db3: f64 = *var_ibbt__blk1567_db3_slot;
        let mut var_ibbt__blk1567_db4: f64 = *var_ibbt__blk1567_db4_slot;
        let mut var_ibbt__blk1567_db5: f64 = *var_ibbt__blk1567_db5_slot;
        let mut var_ibbt__blk1567_db6: f64 = *var_ibbt__blk1567_db6_slot;
        let mut var_ibbt__blk1567_dn0: f64 = *var_ibbt__blk1567_dn0_slot;
        let mut var_ibbt__blk1567_dn1: f64 = *var_ibbt__blk1567_dn1_slot;
        let mut var_ibbt__blk1567_dn10: f64 = *var_ibbt__blk1567_dn10_slot;
        let mut var_ibbt__blk1567_dn11: f64 = *var_ibbt__blk1567_dn11_slot;
        let mut var_ibbt__blk1567_dn2: f64 = *var_ibbt__blk1567_dn2_slot;
        let mut var_ibbt__blk1567_dn3: f64 = *var_ibbt__blk1567_dn3_slot;
        let mut var_ibbt__blk1567_dn4: f64 = *var_ibbt__blk1567_dn4_slot;
        let mut var_ibbt__blk1567_dn5: f64 = *var_ibbt__blk1567_dn5_slot;
        let mut var_ibbt__blk1567_dn6: f64 = *var_ibbt__blk1567_dn6_slot;
        let mut var_ibbt__blk1567_dn7: f64 = *var_ibbt__blk1567_dn7_slot;
        let mut var_ibbt__blk1567_dn8: f64 = *var_ibbt__blk1567_dn8_slot;
        let mut var_ibbt__blk1567_dn9: f64 = *var_ibbt__blk1567_dn9_slot;
        let mut var_itat__blk1552: f64 = *var_itat__blk1552_slot;
        let mut var_itat__blk1552_db0: f64 = *var_itat__blk1552_db0_slot;
        let mut var_itat__blk1552_db1: f64 = *var_itat__blk1552_db1_slot;
        let mut var_itat__blk1552_db2: f64 = *var_itat__blk1552_db2_slot;
        let mut var_itat__blk1552_db3: f64 = *var_itat__blk1552_db3_slot;
        let mut var_itat__blk1552_db4: f64 = *var_itat__blk1552_db4_slot;
        let mut var_itat__blk1552_db5: f64 = *var_itat__blk1552_db5_slot;
        let mut var_itat__blk1552_db6: f64 = *var_itat__blk1552_db6_slot;
        let mut var_itat__blk1552_dn0: f64 = *var_itat__blk1552_dn0_slot;
        let mut var_itat__blk1552_dn1: f64 = *var_itat__blk1552_dn1_slot;
        let mut var_itat__blk1552_dn10: f64 = *var_itat__blk1552_dn10_slot;
        let mut var_itat__blk1552_dn11: f64 = *var_itat__blk1552_dn11_slot;
        let mut var_itat__blk1552_dn2: f64 = *var_itat__blk1552_dn2_slot;
        let mut var_itat__blk1552_dn3: f64 = *var_itat__blk1552_dn3_slot;
        let mut var_itat__blk1552_dn4: f64 = *var_itat__blk1552_dn4_slot;
        let mut var_itat__blk1552_dn5: f64 = *var_itat__blk1552_dn5_slot;
        let mut var_itat__blk1552_dn6: f64 = *var_itat__blk1552_dn6_slot;
        let mut var_itat__blk1552_dn7: f64 = *var_itat__blk1552_dn7_slot;
        let mut var_itat__blk1552_dn8: f64 = *var_itat__blk1552_dn8_slot;
        let mut var_itat__blk1552_dn9: f64 = *var_itat__blk1552_dn9_slot;
        let mut var_tmp__blk1543: f64 = *var_tmp__blk1543_slot;
        let mut var_tmp__blk1543_db0: f64 = *var_tmp__blk1543_db0_slot;
        let mut var_tmp__blk1543_db1: f64 = *var_tmp__blk1543_db1_slot;
        let mut var_tmp__blk1543_db2: f64 = *var_tmp__blk1543_db2_slot;
        let mut var_tmp__blk1543_db3: f64 = *var_tmp__blk1543_db3_slot;
        let mut var_tmp__blk1543_db4: f64 = *var_tmp__blk1543_db4_slot;
        let mut var_tmp__blk1543_db5: f64 = *var_tmp__blk1543_db5_slot;
        let mut var_tmp__blk1543_db6: f64 = *var_tmp__blk1543_db6_slot;
        let mut var_tmp__blk1543_dn0: f64 = *var_tmp__blk1543_dn0_slot;
        let mut var_tmp__blk1543_dn1: f64 = *var_tmp__blk1543_dn1_slot;
        let mut var_tmp__blk1543_dn10: f64 = *var_tmp__blk1543_dn10_slot;
        let mut var_tmp__blk1543_dn11: f64 = *var_tmp__blk1543_dn11_slot;
        let mut var_tmp__blk1543_dn2: f64 = *var_tmp__blk1543_dn2_slot;
        let mut var_tmp__blk1543_dn3: f64 = *var_tmp__blk1543_dn3_slot;
        let mut var_tmp__blk1543_dn4: f64 = *var_tmp__blk1543_dn4_slot;
        let mut var_tmp__blk1543_dn5: f64 = *var_tmp__blk1543_dn5_slot;
        let mut var_tmp__blk1543_dn6: f64 = *var_tmp__blk1543_dn6_slot;
        let mut var_tmp__blk1543_dn7: f64 = *var_tmp__blk1543_dn7_slot;
        let mut var_tmp__blk1543_dn8: f64 = *var_tmp__blk1543_dn8_slot;
        let mut var_tmp__blk1543_dn9: f64 = *var_tmp__blk1543_dn9_slot;

        let (assign61250_e79468, assign61250_e79468_d_n0, assign61250_e79468_d_n1, assign61250_e79468_d_n2, assign61250_e79468_d_n3, assign61250_e79468_d_n4, assign61250_e79468_d_n5, assign61250_e79468_d_n6, assign61250_e79468_d_n7, assign61250_e79468_d_n8, assign61250_e79468_d_n9, assign61250_e79468_d_n10, assign61250_e79468_d_n11, assign61250_e79468_d_b0, assign61250_e79468_d_b1, assign61250_e79468_d_b2, assign61250_e79468_d_b3, assign61250_e79468_d_b4, assign61250_e79468_d_b5, assign61250_e79468_d_b6,) = {
    if ((((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) && (var_guard1705 == 0.0)) && (var_guard1706 != 0.0)) {
        let assign61250_e79466: f64 = (var_mtat__blk1563).exp();
        (assign61250_e79466, (assign61250_e79466 * var_mtat__blk1563_dn0), (assign61250_e79466 * var_mtat__blk1563_dn1), (assign61250_e79466 * var_mtat__blk1563_dn2), (assign61250_e79466 * var_mtat__blk1563_dn3), (assign61250_e79466 * var_mtat__blk1563_dn4), (assign61250_e79466 * var_mtat__blk1563_dn5), (assign61250_e79466 * var_mtat__blk1563_dn6), (assign61250_e79466 * var_mtat__blk1563_dn7), (assign61250_e79466 * var_mtat__blk1563_dn8), (assign61250_e79466 * var_mtat__blk1563_dn9), (assign61250_e79466 * var_mtat__blk1563_dn10), (assign61250_e79466 * var_mtat__blk1563_dn11), (assign61250_e79466 * var_mtat__blk1563_db0), (assign61250_e79466 * var_mtat__blk1563_db1), (assign61250_e79466 * var_mtat__blk1563_db2), (assign61250_e79466 * var_mtat__blk1563_db3), (assign61250_e79466 * var_mtat__blk1563_db4), (assign61250_e79466 * var_mtat__blk1563_db5), (assign61250_e79466 * var_mtat__blk1563_db6),)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61250_e79468;
        var_tmp__blk1543_dn0 = assign61250_e79468_d_n0;
        var_tmp__blk1543_dn1 = assign61250_e79468_d_n1;
        var_tmp__blk1543_dn2 = assign61250_e79468_d_n2;
        var_tmp__blk1543_dn3 = assign61250_e79468_d_n3;
        var_tmp__blk1543_dn4 = assign61250_e79468_d_n4;
        var_tmp__blk1543_dn5 = assign61250_e79468_d_n5;
        var_tmp__blk1543_dn6 = assign61250_e79468_d_n6;
        var_tmp__blk1543_dn7 = assign61250_e79468_d_n7;
        var_tmp__blk1543_dn8 = assign61250_e79468_d_n8;
        var_tmp__blk1543_dn9 = assign61250_e79468_d_n9;
        var_tmp__blk1543_dn10 = assign61250_e79468_d_n10;
        var_tmp__blk1543_dn11 = assign61250_e79468_d_n11;
        var_tmp__blk1543_db0 = assign61250_e79468_d_b0;
        var_tmp__blk1543_db1 = assign61250_e79468_d_b1;
        var_tmp__blk1543_db2 = assign61250_e79468_d_b2;
        var_tmp__blk1543_db3 = assign61250_e79468_d_b3;
        var_tmp__blk1543_db4 = assign61250_e79468_d_b4;
        var_tmp__blk1543_db5 = assign61250_e79468_d_b5;
        var_tmp__blk1543_db6 = assign61250_e79468_d_b6;

        let (assign61260_e79512, assign61260_e79512_d_n0, assign61260_e79512_d_n1, assign61260_e79512_d_n2, assign61260_e79512_d_n3, assign61260_e79512_d_n4, assign61260_e79512_d_n5, assign61260_e79512_d_n6, assign61260_e79512_d_n7, assign61260_e79512_d_n8, assign61260_e79512_d_n9, assign61260_e79512_d_n10, assign61260_e79512_d_n11, assign61260_e79512_d_b0, assign61260_e79512_d_b1, assign61260_e79512_d_b2, assign61260_e79512_d_b3, assign61260_e79512_d_b4, assign61260_e79512_d_b5, assign61260_e79512_d_b6,) = {
    if ((((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) && (var_guard1705 == 0.0)) && (var_guard1706 == 0.0)) {
        let assign61260_e79488: f64 = (-230.25850929940458);
        let assign61260_e79490: f64 = (assign61260_e79488 - var_mtat__blk1563);
        let assign61260_e79494: f64 = (-230.25850929940458);
        let assign61260_e79496: f64 = (assign61260_e79494 - var_mtat__blk1563);
        let assign61260_e79499: f64 = (-230.25850929940458);
        let assign61260_e79501: f64 = (assign61260_e79499 - var_mtat__blk1563);
        let assign61260_e79503: f64 = (assign61260_e79501 * 0.3333333333333333);
        let assign61260_e79504: f64 = (1.0 + assign61260_e79503);
        let assign61260_e79505: f64 = (assign61260_e79496 * assign61260_e79504);
        let assign61260_e79506: f64 = (0.5 * assign61260_e79505);
        let assign61260_e79507: f64 = (1.0 + assign61260_e79506);
        let assign61260_e79508: f64 = (assign61260_e79490 * assign61260_e79507);
        let assign61260_e79509: f64 = (1.0 + assign61260_e79508);
        let assign61260_e79510: f64 = (1e-100 / assign61260_e79509);
        (assign61260_e79510, (-((1e-100 * (((-var_mtat__blk1563_dn0) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_dn0) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_dn0) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_dn1) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_dn1) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_dn1) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_dn2) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_dn2) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_dn2) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_dn3) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_dn3) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_dn3) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_dn4) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_dn4) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_dn4) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_dn5) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_dn5) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_dn5) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_dn6) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_dn6) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_dn6) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_dn7) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_dn7) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_dn7) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_dn8) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_dn8) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_dn8) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_dn9) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_dn9) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_dn9) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_dn10) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_dn10) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_dn10) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_dn11) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_dn11) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_dn11) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_db0) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_db0) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_db0) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_db1) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_db1) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_db1) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_db2) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_db2) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_db2) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_db3) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_db3) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_db3) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_db4) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_db4) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_db4) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_db5) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_db5) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_db5) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))), (-((1e-100 * (((-var_mtat__blk1563_db6) * assign61260_e79507) + (assign61260_e79490 * (0.5 * (((-var_mtat__blk1563_db6) * assign61260_e79504) + (assign61260_e79496 * ((-var_mtat__blk1563_db6) * 0.3333333333333333))))))) / (assign61260_e79509 * assign61260_e79509))),)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61260_e79512;
        var_tmp__blk1543_dn0 = assign61260_e79512_d_n0;
        var_tmp__blk1543_dn1 = assign61260_e79512_d_n1;
        var_tmp__blk1543_dn2 = assign61260_e79512_d_n2;
        var_tmp__blk1543_dn3 = assign61260_e79512_d_n3;
        var_tmp__blk1543_dn4 = assign61260_e79512_d_n4;
        var_tmp__blk1543_dn5 = assign61260_e79512_d_n5;
        var_tmp__blk1543_dn6 = assign61260_e79512_d_n6;
        var_tmp__blk1543_dn7 = assign61260_e79512_d_n7;
        var_tmp__blk1543_dn8 = assign61260_e79512_d_n8;
        var_tmp__blk1543_dn9 = assign61260_e79512_d_n9;
        var_tmp__blk1543_dn10 = assign61260_e79512_d_n10;
        var_tmp__blk1543_dn11 = assign61260_e79512_d_n11;
        var_tmp__blk1543_db0 = assign61260_e79512_d_b0;
        var_tmp__blk1543_db1 = assign61260_e79512_d_b1;
        var_tmp__blk1543_db2 = assign61260_e79512_d_b2;
        var_tmp__blk1543_db3 = assign61260_e79512_d_b3;
        var_tmp__blk1543_db4 = assign61260_e79512_d_b4;
        var_tmp__blk1543_db5 = assign61260_e79512_d_b5;
        var_tmp__blk1543_db6 = assign61260_e79512_d_b6;

        let (assign61270_e79532, assign61270_e79532_d_n0, assign61270_e79532_d_n1, assign61270_e79532_d_n2, assign61270_e79532_d_n3, assign61270_e79532_d_n4, assign61270_e79532_d_n5, assign61270_e79532_d_n6, assign61270_e79532_d_n7, assign61270_e79532_d_n8, assign61270_e79532_d_n9, assign61270_e79532_d_n10, assign61270_e79532_d_n11, assign61270_e79532_d_b0, assign61270_e79532_d_b1, assign61270_e79532_d_b2, assign61270_e79532_d_b3, assign61270_e79532_d_b4, assign61270_e79532_d_b5, assign61270_e79532_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) && (var_guard1705 == 0.0)) {
        let assign61270_e79528: f64 = (2.0 * var_tmp__blk1543);
        let assign61270_e79530: f64 = (assign61270_e79528 - var_erfcpos__blk1527);
        (assign61270_e79530, ((2.0 * var_tmp__blk1543_dn0) - var_erfcpos__blk1527_dn0), ((2.0 * var_tmp__blk1543_dn1) - var_erfcpos__blk1527_dn1), ((2.0 * var_tmp__blk1543_dn2) - var_erfcpos__blk1527_dn2), ((2.0 * var_tmp__blk1543_dn3) - var_erfcpos__blk1527_dn3), ((2.0 * var_tmp__blk1543_dn4) - var_erfcpos__blk1527_dn4), ((2.0 * var_tmp__blk1543_dn5) - var_erfcpos__blk1527_dn5), ((2.0 * var_tmp__blk1543_dn6) - var_erfcpos__blk1527_dn6), ((2.0 * var_tmp__blk1543_dn7) - var_erfcpos__blk1527_dn7), ((2.0 * var_tmp__blk1543_dn8) - var_erfcpos__blk1527_dn8), ((2.0 * var_tmp__blk1543_dn9) - var_erfcpos__blk1527_dn9), ((2.0 * var_tmp__blk1543_dn10) - var_erfcpos__blk1527_dn10), ((2.0 * var_tmp__blk1543_dn11) - var_erfcpos__blk1527_dn11), ((2.0 * var_tmp__blk1543_db0) - var_erfcpos__blk1527_db0), ((2.0 * var_tmp__blk1543_db1) - var_erfcpos__blk1527_db1), ((2.0 * var_tmp__blk1543_db2) - var_erfcpos__blk1527_db2), ((2.0 * var_tmp__blk1543_db3) - var_erfcpos__blk1527_db3), ((2.0 * var_tmp__blk1543_db4) - var_erfcpos__blk1527_db4), ((2.0 * var_tmp__blk1543_db5) - var_erfcpos__blk1527_db5), ((2.0 * var_tmp__blk1543_db6) - var_erfcpos__blk1527_db6),)
    } else {
        (var_erfctimesexpmtat__blk1565, var_erfctimesexpmtat__blk1565_dn0, var_erfctimesexpmtat__blk1565_dn1, var_erfctimesexpmtat__blk1565_dn2, var_erfctimesexpmtat__blk1565_dn3, var_erfctimesexpmtat__blk1565_dn4, var_erfctimesexpmtat__blk1565_dn5, var_erfctimesexpmtat__blk1565_dn6, var_erfctimesexpmtat__blk1565_dn7, var_erfctimesexpmtat__blk1565_dn8, var_erfctimesexpmtat__blk1565_dn9, var_erfctimesexpmtat__blk1565_dn10, var_erfctimesexpmtat__blk1565_dn11, var_erfctimesexpmtat__blk1565_db0, var_erfctimesexpmtat__blk1565_db1, var_erfctimesexpmtat__blk1565_db2, var_erfctimesexpmtat__blk1565_db3, var_erfctimesexpmtat__blk1565_db4, var_erfctimesexpmtat__blk1565_db5, var_erfctimesexpmtat__blk1565_db6,)
    }
};
        var_erfctimesexpmtat__blk1565 = assign61270_e79532;
        var_erfctimesexpmtat__blk1565_dn0 = assign61270_e79532_d_n0;
        var_erfctimesexpmtat__blk1565_dn1 = assign61270_e79532_d_n1;
        var_erfctimesexpmtat__blk1565_dn2 = assign61270_e79532_d_n2;
        var_erfctimesexpmtat__blk1565_dn3 = assign61270_e79532_d_n3;
        var_erfctimesexpmtat__blk1565_dn4 = assign61270_e79532_d_n4;
        var_erfctimesexpmtat__blk1565_dn5 = assign61270_e79532_d_n5;
        var_erfctimesexpmtat__blk1565_dn6 = assign61270_e79532_d_n6;
        var_erfctimesexpmtat__blk1565_dn7 = assign61270_e79532_d_n7;
        var_erfctimesexpmtat__blk1565_dn8 = assign61270_e79532_d_n8;
        var_erfctimesexpmtat__blk1565_dn9 = assign61270_e79532_d_n9;
        var_erfctimesexpmtat__blk1565_dn10 = assign61270_e79532_d_n10;
        var_erfctimesexpmtat__blk1565_dn11 = assign61270_e79532_d_n11;
        var_erfctimesexpmtat__blk1565_db0 = assign61270_e79532_d_b0;
        var_erfctimesexpmtat__blk1565_db1 = assign61270_e79532_d_b1;
        var_erfctimesexpmtat__blk1565_db2 = assign61270_e79532_d_b2;
        var_erfctimesexpmtat__blk1565_db3 = assign61270_e79532_d_b3;
        var_erfctimesexpmtat__blk1565_db4 = assign61270_e79532_d_b4;
        var_erfctimesexpmtat__blk1565_db5 = assign61270_e79532_d_b5;
        var_erfctimesexpmtat__blk1565_db6 = assign61270_e79532_d_b6;

        let (assign61280_e79553, assign61280_e79553_d_n0, assign61280_e79553_d_n1, assign61280_e79553_d_n2, assign61280_e79553_d_n3, assign61280_e79553_d_n4, assign61280_e79553_d_n5, assign61280_e79553_d_n6, assign61280_e79553_d_n7, assign61280_e79553_d_n8, assign61280_e79553_d_n9, assign61280_e79553_d_n10, assign61280_e79553_d_n11, assign61280_e79553_d_b0, assign61280_e79553_d_b1, assign61280_e79553_d_b2, assign61280_e79553_d_b3, assign61280_e79553_d_b4, assign61280_e79553_d_b5, assign61280_e79553_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) {
        let assign61280_e79545: f64 = (1.772453850905516 * 0.5);
        let assign61280_e79548: f64 = (var_atatgat_d * var_erfctimesexpmtat__blk1565);
        let assign61280_e79550: f64 = (assign61280_e79548 / var_ktat__blk1561);
        let assign61280_e79551: f64 = (assign61280_e79545 * assign61280_e79550);
        (assign61280_e79551, (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_dn0) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_dn0)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_dn1) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_dn1)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_dn2) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_dn2)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_dn3) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_dn3)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_dn4) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_dn4)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_dn5) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_dn5)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_dn6) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_dn6)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_dn7) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_dn7)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_dn8) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_dn8)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_dn9) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_dn9)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_dn10) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_dn10)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_dn11) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_dn11)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_db0) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_db0)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_db1) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_db1)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_db2) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_db2)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_db3) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_db3)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_db4) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_db4)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_db5) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_db5)) / (var_ktat__blk1561 * var_ktat__blk1561))), (assign61280_e79545 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1565_db6) * var_ktat__blk1561) - (assign61280_e79548 * var_ktat__blk1561_db6)) / (var_ktat__blk1561 * var_ktat__blk1561))),)
    } else {
        (var_gammamax__blk1566, var_gammamax__blk1566_dn0, var_gammamax__blk1566_dn1, var_gammamax__blk1566_dn2, var_gammamax__blk1566_dn3, var_gammamax__blk1566_dn4, var_gammamax__blk1566_dn5, var_gammamax__blk1566_dn6, var_gammamax__blk1566_dn7, var_gammamax__blk1566_dn8, var_gammamax__blk1566_dn9, var_gammamax__blk1566_dn10, var_gammamax__blk1566_dn11, var_gammamax__blk1566_db0, var_gammamax__blk1566_db1, var_gammamax__blk1566_db2, var_gammamax__blk1566_db3, var_gammamax__blk1566_db4, var_gammamax__blk1566_db5, var_gammamax__blk1566_db6,)
    }
};
        var_gammamax__blk1566 = assign61280_e79553;
        var_gammamax__blk1566_dn0 = assign61280_e79553_d_n0;
        var_gammamax__blk1566_dn1 = assign61280_e79553_d_n1;
        var_gammamax__blk1566_dn2 = assign61280_e79553_d_n2;
        var_gammamax__blk1566_dn3 = assign61280_e79553_d_n3;
        var_gammamax__blk1566_dn4 = assign61280_e79553_d_n4;
        var_gammamax__blk1566_dn5 = assign61280_e79553_d_n5;
        var_gammamax__blk1566_dn6 = assign61280_e79553_d_n6;
        var_gammamax__blk1566_dn7 = assign61280_e79553_d_n7;
        var_gammamax__blk1566_dn8 = assign61280_e79553_d_n8;
        var_gammamax__blk1566_dn9 = assign61280_e79553_d_n9;
        var_gammamax__blk1566_dn10 = assign61280_e79553_d_n10;
        var_gammamax__blk1566_dn11 = assign61280_e79553_d_n11;
        var_gammamax__blk1566_db0 = assign61280_e79553_d_b0;
        var_gammamax__blk1566_db1 = assign61280_e79553_d_b1;
        var_gammamax__blk1566_db2 = assign61280_e79553_d_b2;
        var_gammamax__blk1566_db3 = assign61280_e79553_d_b3;
        var_gammamax__blk1566_db4 = assign61280_e79553_d_b4;
        var_gammamax__blk1566_db5 = assign61280_e79553_d_b5;
        var_gammamax__blk1566_db6 = assign61280_e79553_d_b6;

        let (assign61290_e79572, assign61290_e79572_d_n0, assign61290_e79572_d_n1, assign61290_e79572_d_n2, assign61290_e79572_d_n3, assign61290_e79572_d_n4, assign61290_e79572_d_n5, assign61290_e79572_d_n6, assign61290_e79572_d_n7, assign61290_e79572_d_n8, assign61290_e79572_d_n9, assign61290_e79572_d_n10, assign61290_e79572_d_n11, assign61290_e79572_d_b0, assign61290_e79572_d_b1, assign61290_e79572_d_b2, assign61290_e79572_d_b3, assign61290_e79572_d_b4, assign61290_e79572_d_b5, assign61290_e79572_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1701 == 0.0)) {
        let assign61290_e79567: f64 = (var_asrh__blk1551 * var_gammamax__blk1566);
        let assign61290_e79569: f64 = (assign61290_e79567 * var_wtat__blk1560);
        let assign61290_e79570: f64 = (var_ctatgatd_i * assign61290_e79569);
        (assign61290_e79570, (var_ctatgatd_i * ((((var_asrh__blk1551_dn0 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_dn0)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_dn0))), (var_ctatgatd_i * ((((var_asrh__blk1551_dn1 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_dn1)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_dn1))), (var_ctatgatd_i * ((((var_asrh__blk1551_dn2 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_dn2)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_dn2))), (var_ctatgatd_i * ((((var_asrh__blk1551_dn3 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_dn3)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_dn3))), (var_ctatgatd_i * ((((var_asrh__blk1551_dn4 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_dn4)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_dn4))), (var_ctatgatd_i * ((((var_asrh__blk1551_dn5 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_dn5)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_dn5))), (var_ctatgatd_i * ((((var_asrh__blk1551_dn6 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_dn6)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_dn6))), (var_ctatgatd_i * ((((var_asrh__blk1551_dn7 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_dn7)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_dn7))), (var_ctatgatd_i * ((((var_asrh__blk1551_dn8 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_dn8)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_dn8))), (var_ctatgatd_i * ((((var_asrh__blk1551_dn9 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_dn9)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_dn9))), (var_ctatgatd_i * ((((var_asrh__blk1551_dn10 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_dn10)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_dn10))), (var_ctatgatd_i * ((((var_asrh__blk1551_dn11 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_dn11)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_dn11))), (var_ctatgatd_i * ((((var_asrh__blk1551_db0 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_db0)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_db0))), (var_ctatgatd_i * ((((var_asrh__blk1551_db1 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_db1)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_db1))), (var_ctatgatd_i * ((((var_asrh__blk1551_db2 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_db2)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_db2))), (var_ctatgatd_i * ((((var_asrh__blk1551_db3 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_db3)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_db3))), (var_ctatgatd_i * ((((var_asrh__blk1551_db4 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_db4)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_db4))), (var_ctatgatd_i * ((((var_asrh__blk1551_db5 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_db5)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_db5))), (var_ctatgatd_i * ((((var_asrh__blk1551_db6 * var_gammamax__blk1566) + (var_asrh__blk1551 * var_gammamax__blk1566_db6)) * var_wtat__blk1560) + (assign61290_e79567 * var_wtat__blk1560_db6))),)
    } else {
        (var_itat__blk1552, var_itat__blk1552_dn0, var_itat__blk1552_dn1, var_itat__blk1552_dn2, var_itat__blk1552_dn3, var_itat__blk1552_dn4, var_itat__blk1552_dn5, var_itat__blk1552_dn6, var_itat__blk1552_dn7, var_itat__blk1552_dn8, var_itat__blk1552_dn9, var_itat__blk1552_dn10, var_itat__blk1552_dn11, var_itat__blk1552_db0, var_itat__blk1552_db1, var_itat__blk1552_db2, var_itat__blk1552_db3, var_itat__blk1552_db4, var_itat__blk1552_db5, var_itat__blk1552_db6,)
    }
};
        var_itat__blk1552 = assign61290_e79572;
        var_itat__blk1552_dn0 = assign61290_e79572_d_n0;
        var_itat__blk1552_dn1 = assign61290_e79572_d_n1;
        var_itat__blk1552_dn2 = assign61290_e79572_d_n2;
        var_itat__blk1552_dn3 = assign61290_e79572_d_n3;
        var_itat__blk1552_dn4 = assign61290_e79572_d_n4;
        var_itat__blk1552_dn5 = assign61290_e79572_d_n5;
        var_itat__blk1552_dn6 = assign61290_e79572_d_n6;
        var_itat__blk1552_dn7 = assign61290_e79572_d_n7;
        var_itat__blk1552_dn8 = assign61290_e79572_d_n8;
        var_itat__blk1552_dn9 = assign61290_e79572_d_n9;
        var_itat__blk1552_dn10 = assign61290_e79572_d_n10;
        var_itat__blk1552_dn11 = assign61290_e79572_d_n11;
        var_itat__blk1552_db0 = assign61290_e79572_d_b0;
        var_itat__blk1552_db1 = assign61290_e79572_d_b1;
        var_itat__blk1552_db2 = assign61290_e79572_d_b2;
        var_itat__blk1552_db3 = assign61290_e79572_d_b3;
        var_itat__blk1552_db4 = assign61290_e79572_d_b4;
        var_itat__blk1552_db5 = assign61290_e79572_d_b5;
        var_itat__blk1552_db6 = assign61290_e79572_d_b6;

        let assign61300_e79575: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard1707 = assign61300_e79575;

        let (assign61310_e79587, assign61310_e79587_d_n0, assign61310_e79587_d_n1, assign61310_e79587_d_n2, assign61310_e79587_d_n3, assign61310_e79587_d_n4, assign61310_e79587_d_n5, assign61310_e79587_d_n6, assign61310_e79587_d_n7, assign61310_e79587_d_n8, assign61310_e79587_d_n9, assign61310_e79587_d_n10, assign61310_e79587_d_n11, assign61310_e79587_d_b0, assign61310_e79587_d_b1, assign61310_e79587_d_b2, assign61310_e79587_d_b3, assign61310_e79587_d_b4, assign61310_e79587_d_b5, assign61310_e79587_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1707 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt__blk1567, var_ibbt__blk1567_dn0, var_ibbt__blk1567_dn1, var_ibbt__blk1567_dn2, var_ibbt__blk1567_dn3, var_ibbt__blk1567_dn4, var_ibbt__blk1567_dn5, var_ibbt__blk1567_dn6, var_ibbt__blk1567_dn7, var_ibbt__blk1567_dn8, var_ibbt__blk1567_dn9, var_ibbt__blk1567_dn10, var_ibbt__blk1567_dn11, var_ibbt__blk1567_db0, var_ibbt__blk1567_db1, var_ibbt__blk1567_db2, var_ibbt__blk1567_db3, var_ibbt__blk1567_db4, var_ibbt__blk1567_db5, var_ibbt__blk1567_db6,)
    }
};
        var_ibbt__blk1567 = assign61310_e79587;
        var_ibbt__blk1567_dn0 = assign61310_e79587_d_n0;
        var_ibbt__blk1567_dn1 = assign61310_e79587_d_n1;
        var_ibbt__blk1567_dn2 = assign61310_e79587_d_n2;
        var_ibbt__blk1567_dn3 = assign61310_e79587_d_n3;
        var_ibbt__blk1567_dn4 = assign61310_e79587_d_n4;
        var_ibbt__blk1567_dn5 = assign61310_e79587_d_n5;
        var_ibbt__blk1567_dn6 = assign61310_e79587_d_n6;
        var_ibbt__blk1567_dn7 = assign61310_e79587_d_n7;
        var_ibbt__blk1567_dn8 = assign61310_e79587_d_n8;
        var_ibbt__blk1567_dn9 = assign61310_e79587_d_n9;
        var_ibbt__blk1567_dn10 = assign61310_e79587_d_n10;
        var_ibbt__blk1567_dn11 = assign61310_e79587_d_n11;
        var_ibbt__blk1567_db0 = assign61310_e79587_d_b0;
        var_ibbt__blk1567_db1 = assign61310_e79587_d_b1;
        var_ibbt__blk1567_db2 = assign61310_e79587_d_b2;
        var_ibbt__blk1567_db3 = assign61310_e79587_d_b3;
        var_ibbt__blk1567_db4 = assign61310_e79587_d_b4;
        var_ibbt__blk1567_db5 = assign61310_e79587_d_b5;
        var_ibbt__blk1567_db6 = assign61310_e79587_d_b6;

        let assign61320_e79590: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard1708 = assign61320_e79590;

        let (assign61330_e79610, assign61330_e79610_d_n0, assign61330_e79610_d_n1, assign61330_e79610_d_n2, assign61330_e79610_d_n3, assign61330_e79610_d_n4, assign61330_e79610_d_n5, assign61330_e79610_d_n6, assign61330_e79610_d_n7, assign61330_e79610_d_n8, assign61330_e79610_d_n9, assign61330_e79610_d_n10, assign61330_e79610_d_n11, assign61330_e79610_d_b0, assign61330_e79610_d_b1, assign61330_e79610_d_b2, assign61330_e79610_d_b3, assign61330_e79610_d_b4, assign61330_e79610_d_b5, assign61330_e79610_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1707 == 0.0)) && (var_guard1708 != 0.0)) {
        let assign61330_e79605: f64 = (var_vbirgatd_i - var_vbbt__blk1541);
        let assign61330_e79607: f64 = (assign61330_e79605 * var_vbirgatinv_d);
        let assign61330_e79608: f64 = (assign61330_e79607).sqrt();
        (assign61330_e79608, (((-var_vbbt__blk1541_dn0) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_dn1) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_dn2) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_dn3) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_dn4) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_dn5) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_dn6) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_dn7) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_dn8) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_dn9) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_dn10) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_dn11) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_db0) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_db1) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_db2) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_db3) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_db4) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_db5) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)), (((-var_vbbt__blk1541_db6) * var_vbirgatinv_d) / (2.0 * assign61330_e79608)),)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61330_e79610;
        var_tmp__blk1543_dn0 = assign61330_e79610_d_n0;
        var_tmp__blk1543_dn1 = assign61330_e79610_d_n1;
        var_tmp__blk1543_dn2 = assign61330_e79610_d_n2;
        var_tmp__blk1543_dn3 = assign61330_e79610_d_n3;
        var_tmp__blk1543_dn4 = assign61330_e79610_d_n4;
        var_tmp__blk1543_dn5 = assign61330_e79610_d_n5;
        var_tmp__blk1543_dn6 = assign61330_e79610_d_n6;
        var_tmp__blk1543_dn7 = assign61330_e79610_d_n7;
        var_tmp__blk1543_dn8 = assign61330_e79610_d_n8;
        var_tmp__blk1543_dn9 = assign61330_e79610_d_n9;
        var_tmp__blk1543_dn10 = assign61330_e79610_d_n10;
        var_tmp__blk1543_dn11 = assign61330_e79610_d_n11;
        var_tmp__blk1543_db0 = assign61330_e79610_d_b0;
        var_tmp__blk1543_db1 = assign61330_e79610_d_b1;
        var_tmp__blk1543_db2 = assign61330_e79610_d_b2;
        var_tmp__blk1543_db3 = assign61330_e79610_d_b3;
        var_tmp__blk1543_db4 = assign61330_e79610_d_b4;
        var_tmp__blk1543_db5 = assign61330_e79610_d_b5;
        var_tmp__blk1543_db6 = assign61330_e79610_d_b6;

        let (assign61340_e79632, assign61340_e79632_d_n0, assign61340_e79632_d_n1, assign61340_e79632_d_n2, assign61340_e79632_d_n3, assign61340_e79632_d_n4, assign61340_e79632_d_n5, assign61340_e79632_d_n6, assign61340_e79632_d_n7, assign61340_e79632_d_n8, assign61340_e79632_d_n9, assign61340_e79632_d_n10, assign61340_e79632_d_n11, assign61340_e79632_d_b0, assign61340_e79632_d_b1, assign61340_e79632_d_b2, assign61340_e79632_d_b3, assign61340_e79632_d_b4, assign61340_e79632_d_b5, assign61340_e79632_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1707 == 0.0)) && (var_guard1708 == 0.0)) {
        let assign61340_e79626: f64 = (var_vbirgatd_i - var_vbbt__blk1541);
        let assign61340_e79628: f64 = (assign61340_e79626 * var_vbirgatinv_d);
        let assign61340_e79630: f64 = (assign61340_e79628).powf(var_pgatd_i);
        (assign61340_e79630, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_dn0) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_dn0) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_dn1) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_dn1) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_dn2) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_dn2) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_dn3) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_dn3) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_dn4) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_dn4) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_dn5) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_dn5) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_dn6) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_dn6) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_dn7) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_dn7) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_dn8) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_dn8) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_dn9) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_dn9) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_dn10) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_dn10) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_dn11) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_dn11) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_db0) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_db0) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_db1) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_db1) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_db2) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_db2) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_db3) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_db3) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_db4) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_db4) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_db5) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_db5) * var_vbirgatinv_d) / assign61340_e79628))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61340_e79628).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1541_db6) * var_vbirgatinv_d))) } } else { (assign61340_e79630 * (var_pgatd_i * (((-var_vbbt__blk1541_db6) * var_vbirgatinv_d) / assign61340_e79628))) },)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61340_e79632;
        var_tmp__blk1543_dn0 = assign61340_e79632_d_n0;
        var_tmp__blk1543_dn1 = assign61340_e79632_d_n1;
        var_tmp__blk1543_dn2 = assign61340_e79632_d_n2;
        var_tmp__blk1543_dn3 = assign61340_e79632_d_n3;
        var_tmp__blk1543_dn4 = assign61340_e79632_d_n4;
        var_tmp__blk1543_dn5 = assign61340_e79632_d_n5;
        var_tmp__blk1543_dn6 = assign61340_e79632_d_n6;
        var_tmp__blk1543_dn7 = assign61340_e79632_d_n7;
        var_tmp__blk1543_dn8 = assign61340_e79632_d_n8;
        var_tmp__blk1543_dn9 = assign61340_e79632_d_n9;
        var_tmp__blk1543_dn10 = assign61340_e79632_d_n10;
        var_tmp__blk1543_dn11 = assign61340_e79632_d_n11;
        var_tmp__blk1543_db0 = assign61340_e79632_d_b0;
        var_tmp__blk1543_db1 = assign61340_e79632_d_b1;
        var_tmp__blk1543_db2 = assign61340_e79632_d_b2;
        var_tmp__blk1543_db3 = assign61340_e79632_d_b3;
        var_tmp__blk1543_db4 = assign61340_e79632_d_b4;
        var_tmp__blk1543_db5 = assign61340_e79632_d_b5;
        var_tmp__blk1543_db6 = assign61340_e79632_d_b6;

        let (assign61350_e79653, assign61350_e79653_d_n0, assign61350_e79653_d_n1, assign61350_e79653_d_n2, assign61350_e79653_d_n3, assign61350_e79653_d_n4, assign61350_e79653_d_n5, assign61350_e79653_d_n6, assign61350_e79653_d_n7, assign61350_e79653_d_n8, assign61350_e79653_d_n9, assign61350_e79653_d_n10, assign61350_e79653_d_n11, assign61350_e79653_d_b0, assign61350_e79653_d_b1, assign61350_e79653_d_b2, assign61350_e79653_d_b3, assign61350_e79653_d_b4, assign61350_e79653_d_b5, assign61350_e79653_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1707 == 0.0)) {
        let assign61350_e79646: f64 = (var_vbirgatd_i - var_vbbt__blk1541);
        let assign61350_e79648: f64 = (assign61350_e79646 * var_wdepnulrinvgat_d);
        let assign61350_e79650: f64 = (assign61350_e79648 / var_tmp__blk1543);
        let assign61350_e79651: f64 = (var_one_over_one_minus_pgat_d * assign61350_e79650);
        (assign61350_e79651, (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_dn0) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_dn0)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_dn1) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_dn1)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_dn2) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_dn2)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_dn3) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_dn3)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_dn4) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_dn4)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_dn5) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_dn5)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_dn6) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_dn6)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_dn7) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_dn7)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_dn8) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_dn8)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_dn9) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_dn9)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_dn10) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_dn10)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_dn11) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_dn11)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_db0) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_db0)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_db1) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_db1)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_db2) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_db2)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_db3) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_db3)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_db4) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_db4)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_db5) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_db5)) / (var_tmp__blk1543 * var_tmp__blk1543))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1541_db6) * var_wdepnulrinvgat_d) * var_tmp__blk1543) - (assign61350_e79648 * var_tmp__blk1543_db6)) / (var_tmp__blk1543 * var_tmp__blk1543))),)
    } else {
        (var_fmaxr__blk1568, var_fmaxr__blk1568_dn0, var_fmaxr__blk1568_dn1, var_fmaxr__blk1568_dn2, var_fmaxr__blk1568_dn3, var_fmaxr__blk1568_dn4, var_fmaxr__blk1568_dn5, var_fmaxr__blk1568_dn6, var_fmaxr__blk1568_dn7, var_fmaxr__blk1568_dn8, var_fmaxr__blk1568_dn9, var_fmaxr__blk1568_dn10, var_fmaxr__blk1568_dn11, var_fmaxr__blk1568_db0, var_fmaxr__blk1568_db1, var_fmaxr__blk1568_db2, var_fmaxr__blk1568_db3, var_fmaxr__blk1568_db4, var_fmaxr__blk1568_db5, var_fmaxr__blk1568_db6,)
    }
};
        var_fmaxr__blk1568 = assign61350_e79653;
        var_fmaxr__blk1568_dn0 = assign61350_e79653_d_n0;
        var_fmaxr__blk1568_dn1 = assign61350_e79653_d_n1;
        var_fmaxr__blk1568_dn2 = assign61350_e79653_d_n2;
        var_fmaxr__blk1568_dn3 = assign61350_e79653_d_n3;
        var_fmaxr__blk1568_dn4 = assign61350_e79653_d_n4;
        var_fmaxr__blk1568_dn5 = assign61350_e79653_d_n5;
        var_fmaxr__blk1568_dn6 = assign61350_e79653_d_n6;
        var_fmaxr__blk1568_dn7 = assign61350_e79653_d_n7;
        var_fmaxr__blk1568_dn8 = assign61350_e79653_d_n8;
        var_fmaxr__blk1568_dn9 = assign61350_e79653_d_n9;
        var_fmaxr__blk1568_dn10 = assign61350_e79653_d_n10;
        var_fmaxr__blk1568_dn11 = assign61350_e79653_d_n11;
        var_fmaxr__blk1568_db0 = assign61350_e79653_d_b0;
        var_fmaxr__blk1568_db1 = assign61350_e79653_d_b1;
        var_fmaxr__blk1568_db2 = assign61350_e79653_d_b2;
        var_fmaxr__blk1568_db3 = assign61350_e79653_d_b3;
        var_fmaxr__blk1568_db4 = assign61350_e79653_d_b4;
        var_fmaxr__blk1568_db5 = assign61350_e79653_d_b5;
        var_fmaxr__blk1568_db6 = assign61350_e79653_d_b6;

        let assign61360_e79655: f64 = (-var_fbbtgat_d);
        let assign61360_e79657: f64 = (assign61360_e79655 / var_fmaxr__blk1568);
        let assign61360_e79658: f64 = (assign61360_e79657).abs();
        let assign61360_e79660: f64 = if assign61360_e79658 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1709 = assign61360_e79660;

        let (assign61370_e79679, assign61370_e79679_d_n0, assign61370_e79679_d_n1, assign61370_e79679_d_n2, assign61370_e79679_d_n3, assign61370_e79679_d_n4, assign61370_e79679_d_n5, assign61370_e79679_d_n6, assign61370_e79679_d_n7, assign61370_e79679_d_n8, assign61370_e79679_d_n9, assign61370_e79679_d_n10, assign61370_e79679_d_n11, assign61370_e79679_d_b0, assign61370_e79679_d_b1, assign61370_e79679_d_b2, assign61370_e79679_d_b3, assign61370_e79679_d_b4, assign61370_e79679_d_b5, assign61370_e79679_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1707 == 0.0)) && (var_guard1709 != 0.0)) {
        let assign61370_e79674: f64 = (-var_fbbtgat_d);
        let assign61370_e79676: f64 = (assign61370_e79674 / var_fmaxr__blk1568);
        let assign61370_e79677: f64 = (assign61370_e79676).exp();
        (assign61370_e79677, (assign61370_e79677 * ((((-var_fbbtgat_d_dn0) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_dn0)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_dn1) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_dn1)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_dn2) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_dn2)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_dn3) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_dn3)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_dn4) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_dn4)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_dn5) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_dn5)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_dn6) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_dn6)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_dn7) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_dn7)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_dn8) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_dn8)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_dn9) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_dn9)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_dn10) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_dn10)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_dn11) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_dn11)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_db0) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_db0)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_db1) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_db1)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_db2) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_db2)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_db3) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_db3)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_db4) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_db4)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_db5) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_db5)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))), (assign61370_e79677 * ((((-var_fbbtgat_d_db6) * var_fmaxr__blk1568) - (assign61370_e79674 * var_fmaxr__blk1568_db6)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))),)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61370_e79679;
        var_tmp__blk1543_dn0 = assign61370_e79679_d_n0;
        var_tmp__blk1543_dn1 = assign61370_e79679_d_n1;
        var_tmp__blk1543_dn2 = assign61370_e79679_d_n2;
        var_tmp__blk1543_dn3 = assign61370_e79679_d_n3;
        var_tmp__blk1543_dn4 = assign61370_e79679_d_n4;
        var_tmp__blk1543_dn5 = assign61370_e79679_d_n5;
        var_tmp__blk1543_dn6 = assign61370_e79679_d_n6;
        var_tmp__blk1543_dn7 = assign61370_e79679_d_n7;
        var_tmp__blk1543_dn8 = assign61370_e79679_d_n8;
        var_tmp__blk1543_dn9 = assign61370_e79679_d_n9;
        var_tmp__blk1543_dn10 = assign61370_e79679_d_n10;
        var_tmp__blk1543_dn11 = assign61370_e79679_d_n11;
        var_tmp__blk1543_db0 = assign61370_e79679_d_b0;
        var_tmp__blk1543_db1 = assign61370_e79679_d_b1;
        var_tmp__blk1543_db2 = assign61370_e79679_d_b2;
        var_tmp__blk1543_db3 = assign61370_e79679_d_b3;
        var_tmp__blk1543_db4 = assign61370_e79679_d_b4;
        var_tmp__blk1543_db5 = assign61370_e79679_d_b5;
        var_tmp__blk1543_db6 = assign61370_e79679_d_b6;

        let assign61380_e79681: f64 = (-var_fbbtgat_d);
        let assign61380_e79683: f64 = (assign61380_e79681 / var_fmaxr__blk1568);
        let assign61380_e79685: f64 = if assign61380_e79683 < 0.0 { 1.0 } else { 0.0 };
        var_guard1710 = assign61380_e79685;

        let (assign61390_e79737, assign61390_e79737_d_n0, assign61390_e79737_d_n1, assign61390_e79737_d_n2, assign61390_e79737_d_n3, assign61390_e79737_d_n4, assign61390_e79737_d_n5, assign61390_e79737_d_n6, assign61390_e79737_d_n7, assign61390_e79737_d_n8, assign61390_e79737_d_n9, assign61390_e79737_d_n10, assign61390_e79737_d_n11, assign61390_e79737_d_b0, assign61390_e79737_d_b1, assign61390_e79737_d_b2, assign61390_e79737_d_b3, assign61390_e79737_d_b4, assign61390_e79737_d_b5, assign61390_e79737_d_b6,) = {
    if ((((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1707 == 0.0)) && (var_guard1709 == 0.0)) && (var_guard1710 != 0.0)) {
        let assign61390_e79704: f64 = (-230.25850929940458);
        let assign61390_e79706: f64 = (-var_fbbtgat_d);
        let assign61390_e79708: f64 = (assign61390_e79706 / var_fmaxr__blk1568);
        let assign61390_e79709: f64 = (assign61390_e79704 - assign61390_e79708);
        let assign61390_e79713: f64 = (-230.25850929940458);
        let assign61390_e79715: f64 = (-var_fbbtgat_d);
        let assign61390_e79717: f64 = (assign61390_e79715 / var_fmaxr__blk1568);
        let assign61390_e79718: f64 = (assign61390_e79713 - assign61390_e79717);
        let assign61390_e79721: f64 = (-230.25850929940458);
        let assign61390_e79723: f64 = (-var_fbbtgat_d);
        let assign61390_e79725: f64 = (assign61390_e79723 / var_fmaxr__blk1568);
        let assign61390_e79726: f64 = (assign61390_e79721 - assign61390_e79725);
        let assign61390_e79728: f64 = (assign61390_e79726 * 0.3333333333333333);
        let assign61390_e79729: f64 = (1.0 + assign61390_e79728);
        let assign61390_e79730: f64 = (assign61390_e79718 * assign61390_e79729);
        let assign61390_e79731: f64 = (0.5 * assign61390_e79730);
        let assign61390_e79732: f64 = (1.0 + assign61390_e79731);
        let assign61390_e79733: f64 = (assign61390_e79709 * assign61390_e79732);
        let assign61390_e79734: f64 = (1.0 + assign61390_e79733);
        let assign61390_e79735: f64 = (1e-100 / assign61390_e79734);
        (assign61390_e79735, (-((1e-100 * (((-((((-var_fbbtgat_d_dn0) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_dn0)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_dn0) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_dn0)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_dn0) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_dn0)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn1) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_dn1)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_dn1) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_dn1)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_dn1) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_dn1)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn2) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_dn2)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_dn2) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_dn2)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_dn2) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_dn2)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn3) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_dn3)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_dn3) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_dn3)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_dn3) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_dn3)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn4) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_dn4)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_dn4) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_dn4)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_dn4) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_dn4)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_dn5)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_dn5)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_dn5) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_dn5)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_dn6)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_dn6)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_dn6)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_dn7)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_dn7)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_dn7)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_dn8)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_dn8)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_dn8)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn9) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_dn9)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_dn9) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_dn9)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_dn9) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_dn9)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn10) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_dn10)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_dn10) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_dn10)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_dn10) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_dn10)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn11) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_dn11)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_dn11) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_dn11)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_dn11) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_dn11)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_db0) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_db0)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_db0) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_db0)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_db0) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_db0)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_db1) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_db1)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_db1) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_db1)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_db1) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_db1)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_db2) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_db2)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_db2) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_db2)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_db2) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_db2)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_db3) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_db3)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_db3) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_db3)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_db3) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_db3)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_db4) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_db4)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_db4) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_db4)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_db4) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_db4)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_db5) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_db5)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_db5) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_db5)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_db5) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_db5)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))), (-((1e-100 * (((-((((-var_fbbtgat_d_db6) * var_fmaxr__blk1568) - (assign61390_e79706 * var_fmaxr__blk1568_db6)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79732) + (assign61390_e79709 * (0.5 * (((-((((-var_fbbtgat_d_db6) * var_fmaxr__blk1568) - (assign61390_e79715 * var_fmaxr__blk1568_db6)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * assign61390_e79729) + (assign61390_e79718 * ((-((((-var_fbbtgat_d_db6) * var_fmaxr__blk1568) - (assign61390_e79723 * var_fmaxr__blk1568_db6)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568))) * 0.3333333333333333))))))) / (assign61390_e79734 * assign61390_e79734))),)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61390_e79737;
        var_tmp__blk1543_dn0 = assign61390_e79737_d_n0;
        var_tmp__blk1543_dn1 = assign61390_e79737_d_n1;
        var_tmp__blk1543_dn2 = assign61390_e79737_d_n2;
        var_tmp__blk1543_dn3 = assign61390_e79737_d_n3;
        var_tmp__blk1543_dn4 = assign61390_e79737_d_n4;
        var_tmp__blk1543_dn5 = assign61390_e79737_d_n5;
        var_tmp__blk1543_dn6 = assign61390_e79737_d_n6;
        var_tmp__blk1543_dn7 = assign61390_e79737_d_n7;
        var_tmp__blk1543_dn8 = assign61390_e79737_d_n8;
        var_tmp__blk1543_dn9 = assign61390_e79737_d_n9;
        var_tmp__blk1543_dn10 = assign61390_e79737_d_n10;
        var_tmp__blk1543_dn11 = assign61390_e79737_d_n11;
        var_tmp__blk1543_db0 = assign61390_e79737_d_b0;
        var_tmp__blk1543_db1 = assign61390_e79737_d_b1;
        var_tmp__blk1543_db2 = assign61390_e79737_d_b2;
        var_tmp__blk1543_db3 = assign61390_e79737_d_b3;
        var_tmp__blk1543_db4 = assign61390_e79737_d_b4;
        var_tmp__blk1543_db5 = assign61390_e79737_d_b5;
        var_tmp__blk1543_db6 = assign61390_e79737_d_b6;

        let (assign61400_e79787, assign61400_e79787_d_n0, assign61400_e79787_d_n1, assign61400_e79787_d_n2, assign61400_e79787_d_n3, assign61400_e79787_d_n4, assign61400_e79787_d_n5, assign61400_e79787_d_n6, assign61400_e79787_d_n7, assign61400_e79787_d_n8, assign61400_e79787_d_n9, assign61400_e79787_d_n10, assign61400_e79787_d_n11, assign61400_e79787_d_b0, assign61400_e79787_d_b1, assign61400_e79787_d_b2, assign61400_e79787_d_b3, assign61400_e79787_d_b4, assign61400_e79787_d_b5, assign61400_e79787_d_b6,) = {
    if ((((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1707 == 0.0)) && (var_guard1709 == 0.0)) && (var_guard1710 == 0.0)) {
        let assign61400_e79757: f64 = (-var_fbbtgat_d);
        let assign61400_e79759: f64 = (assign61400_e79757 / var_fmaxr__blk1568);
        let assign61400_e79761: f64 = (assign61400_e79759 - 230.25850929940458);
        let assign61400_e79765: f64 = (-var_fbbtgat_d);
        let assign61400_e79767: f64 = (assign61400_e79765 / var_fmaxr__blk1568);
        let assign61400_e79769: f64 = (assign61400_e79767 - 230.25850929940458);
        let assign61400_e79772: f64 = (-var_fbbtgat_d);
        let assign61400_e79774: f64 = (assign61400_e79772 / var_fmaxr__blk1568);
        let assign61400_e79776: f64 = (assign61400_e79774 - 230.25850929940458);
        let assign61400_e79778: f64 = (assign61400_e79776 * 0.3333333333333333);
        let assign61400_e79779: f64 = (1.0 + assign61400_e79778);
        let assign61400_e79780: f64 = (assign61400_e79769 * assign61400_e79779);
        let assign61400_e79781: f64 = (0.5 * assign61400_e79780);
        let assign61400_e79782: f64 = (1.0 + assign61400_e79781);
        let assign61400_e79783: f64 = (assign61400_e79761 * assign61400_e79782);
        let assign61400_e79784: f64 = (1.0 + assign61400_e79783);
        let assign61400_e79785: f64 = (1e100 * assign61400_e79784);
        (assign61400_e79785, (1e100 * ((((((-var_fbbtgat_d_dn0) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_dn0)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_dn0) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_dn0)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_dn0) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_dn0)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn1) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_dn1)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_dn1) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_dn1)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_dn1) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_dn1)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn2) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_dn2)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_dn2) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_dn2)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_dn2) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_dn2)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn3) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_dn3)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_dn3) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_dn3)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_dn3) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_dn3)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn4) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_dn4)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_dn4) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_dn4)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_dn4) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_dn4)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_dn5)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_dn5)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_dn5) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_dn5)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_dn6)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_dn6)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_dn6) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_dn6)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_dn7)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_dn7)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_dn7) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_dn7)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_dn8)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_dn8)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_dn8) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_dn8)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn9) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_dn9)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_dn9) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_dn9)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_dn9) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_dn9)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn10) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_dn10)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_dn10) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_dn10)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_dn10) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_dn10)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn11) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_dn11)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_dn11) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_dn11)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_dn11) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_dn11)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_db0) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_db0)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_db0) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_db0)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_db0) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_db0)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_db1) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_db1)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_db1) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_db1)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_db1) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_db1)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_db2) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_db2)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_db2) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_db2)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_db2) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_db2)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_db3) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_db3)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_db3) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_db3)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_db3) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_db3)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_db4) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_db4)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_db4) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_db4)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_db4) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_db4)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_db5) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_db5)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_db5) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_db5)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_db5) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_db5)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_db6) * var_fmaxr__blk1568) - (assign61400_e79757 * var_fmaxr__blk1568_db6)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79782) + (assign61400_e79761 * (0.5 * ((((((-var_fbbtgat_d_db6) * var_fmaxr__blk1568) - (assign61400_e79765 * var_fmaxr__blk1568_db6)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * assign61400_e79779) + (assign61400_e79769 * (((((-var_fbbtgat_d_db6) * var_fmaxr__blk1568) - (assign61400_e79772 * var_fmaxr__blk1568_db6)) / (var_fmaxr__blk1568 * var_fmaxr__blk1568)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61400_e79787;
        var_tmp__blk1543_dn0 = assign61400_e79787_d_n0;
        var_tmp__blk1543_dn1 = assign61400_e79787_d_n1;
        var_tmp__blk1543_dn2 = assign61400_e79787_d_n2;
        var_tmp__blk1543_dn3 = assign61400_e79787_d_n3;
        var_tmp__blk1543_dn4 = assign61400_e79787_d_n4;
        var_tmp__blk1543_dn5 = assign61400_e79787_d_n5;
        var_tmp__blk1543_dn6 = assign61400_e79787_d_n6;
        var_tmp__blk1543_dn7 = assign61400_e79787_d_n7;
        var_tmp__blk1543_dn8 = assign61400_e79787_d_n8;
        var_tmp__blk1543_dn9 = assign61400_e79787_d_n9;
        var_tmp__blk1543_dn10 = assign61400_e79787_d_n10;
        var_tmp__blk1543_dn11 = assign61400_e79787_d_n11;
        var_tmp__blk1543_db0 = assign61400_e79787_d_b0;
        var_tmp__blk1543_db1 = assign61400_e79787_d_b1;
        var_tmp__blk1543_db2 = assign61400_e79787_d_b2;
        var_tmp__blk1543_db3 = assign61400_e79787_d_b3;
        var_tmp__blk1543_db4 = assign61400_e79787_d_b4;
        var_tmp__blk1543_db5 = assign61400_e79787_d_b5;
        var_tmp__blk1543_db6 = assign61400_e79787_d_b6;

        let (assign61410_e79808, assign61410_e79808_d_n0, assign61410_e79808_d_n1, assign61410_e79808_d_n2, assign61410_e79808_d_n3, assign61410_e79808_d_n4, assign61410_e79808_d_n5, assign61410_e79808_d_n6, assign61410_e79808_d_n7, assign61410_e79808_d_n8, assign61410_e79808_d_n9, assign61410_e79808_d_n10, assign61410_e79808_d_n11, assign61410_e79808_d_b0, assign61410_e79808_d_b1, assign61410_e79808_d_b2, assign61410_e79808_d_b3, assign61410_e79808_d_b4, assign61410_e79808_d_b5, assign61410_e79808_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1707 == 0.0)) {
        let assign61410_e79801: f64 = (var_vjun_d * var_fmaxr__blk1568);
        let assign61410_e79803: f64 = (assign61410_e79801 * var_fmaxr__blk1568);
        let assign61410_e79805: f64 = (assign61410_e79803 * var_tmp__blk1543);
        let assign61410_e79806: f64 = (var_cbbtgatd_i * assign61410_e79805);
        (assign61410_e79806, (var_cbbtgatd_i * ((((((var_vjun_d_dn0 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_dn0)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_dn0)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_dn0))), (var_cbbtgatd_i * ((((((var_vjun_d_dn1 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_dn1)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_dn1)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_dn1))), (var_cbbtgatd_i * ((((((var_vjun_d_dn2 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_dn2)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_dn2)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_dn2))), (var_cbbtgatd_i * ((((((var_vjun_d_dn3 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_dn3)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_dn3)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_dn3))), (var_cbbtgatd_i * ((((((var_vjun_d_dn4 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_dn4)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_dn4)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_dn4))), (var_cbbtgatd_i * ((((((var_vjun_d_dn5 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_dn5)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_dn5)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_dn5))), (var_cbbtgatd_i * ((((((var_vjun_d_dn6 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_dn6)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_dn6)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_dn6))), (var_cbbtgatd_i * ((((((var_vjun_d_dn7 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_dn7)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_dn7)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_dn7))), (var_cbbtgatd_i * ((((((var_vjun_d_dn8 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_dn8)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_dn8)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_dn8))), (var_cbbtgatd_i * ((((((var_vjun_d_dn9 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_dn9)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_dn9)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_dn9))), (var_cbbtgatd_i * ((((((var_vjun_d_dn10 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_dn10)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_dn10)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_dn10))), (var_cbbtgatd_i * ((((((var_vjun_d_dn11 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_dn11)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_dn11)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_dn11))), (var_cbbtgatd_i * ((((((var_vjun_d_db0 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_db0)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_db0)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_db0))), (var_cbbtgatd_i * ((((((var_vjun_d_db1 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_db1)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_db1)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_db1))), (var_cbbtgatd_i * ((((((var_vjun_d_db2 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_db2)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_db2)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_db2))), (var_cbbtgatd_i * ((((((var_vjun_d_db3 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_db3)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_db3)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_db3))), (var_cbbtgatd_i * ((((((var_vjun_d_db4 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_db4)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_db4)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_db4))), (var_cbbtgatd_i * ((((((var_vjun_d_db5 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_db5)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_db5)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_db5))), (var_cbbtgatd_i * ((((((var_vjun_d_db6 * var_fmaxr__blk1568) + (var_vjun_d * var_fmaxr__blk1568_db6)) * var_fmaxr__blk1568) + (assign61410_e79801 * var_fmaxr__blk1568_db6)) * var_tmp__blk1543) + (assign61410_e79803 * var_tmp__blk1543_db6))),)
    } else {
        (var_ibbt__blk1567, var_ibbt__blk1567_dn0, var_ibbt__blk1567_dn1, var_ibbt__blk1567_dn2, var_ibbt__blk1567_dn3, var_ibbt__blk1567_dn4, var_ibbt__blk1567_dn5, var_ibbt__blk1567_dn6, var_ibbt__blk1567_dn7, var_ibbt__blk1567_dn8, var_ibbt__blk1567_dn9, var_ibbt__blk1567_dn10, var_ibbt__blk1567_dn11, var_ibbt__blk1567_db0, var_ibbt__blk1567_db1, var_ibbt__blk1567_db2, var_ibbt__blk1567_db3, var_ibbt__blk1567_db4, var_ibbt__blk1567_db5, var_ibbt__blk1567_db6,)
    }
};
        var_ibbt__blk1567 = assign61410_e79808;
        var_ibbt__blk1567_dn0 = assign61410_e79808_d_n0;
        var_ibbt__blk1567_dn1 = assign61410_e79808_d_n1;
        var_ibbt__blk1567_dn2 = assign61410_e79808_d_n2;
        var_ibbt__blk1567_dn3 = assign61410_e79808_d_n3;
        var_ibbt__blk1567_dn4 = assign61410_e79808_d_n4;
        var_ibbt__blk1567_dn5 = assign61410_e79808_d_n5;
        var_ibbt__blk1567_dn6 = assign61410_e79808_d_n6;
        var_ibbt__blk1567_dn7 = assign61410_e79808_d_n7;
        var_ibbt__blk1567_dn8 = assign61410_e79808_d_n8;
        var_ibbt__blk1567_dn9 = assign61410_e79808_d_n9;
        var_ibbt__blk1567_dn10 = assign61410_e79808_d_n10;
        var_ibbt__blk1567_dn11 = assign61410_e79808_d_n11;
        var_ibbt__blk1567_db0 = assign61410_e79808_d_b0;
        var_ibbt__blk1567_db1 = assign61410_e79808_d_b1;
        var_ibbt__blk1567_db2 = assign61410_e79808_d_b2;
        var_ibbt__blk1567_db3 = assign61410_e79808_d_b3;
        var_ibbt__blk1567_db4 = assign61410_e79808_d_b4;
        var_ibbt__blk1567_db5 = assign61410_e79808_d_b5;
        var_ibbt__blk1567_db6 = assign61410_e79808_d_b6;

        let assign61420_e79811: f64 = if var_vbrgat_var_d > 1000.0 { 1.0 } else { 0.0 };
        var_guard1711 = assign61420_e79811;

        let (assign61430_e79823, assign61430_e79823_d_n0, assign61430_e79823_d_n1, assign61430_e79823_d_n2, assign61430_e79823_d_n3, assign61430_e79823_d_n4, assign61430_e79823_d_n5, assign61430_e79823_d_n6, assign61430_e79823_d_n7, assign61430_e79823_d_n8, assign61430_e79823_d_n9, assign61430_e79823_d_n10, assign61430_e79823_d_n11, assign61430_e79823_d_b0, assign61430_e79823_d_b1, assign61430_e79823_d_b2, assign61430_e79823_d_b3, assign61430_e79823_d_b4, assign61430_e79823_d_b5, assign61430_e79823_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1711 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown__blk1569, var_fbreakdown__blk1569_dn0, var_fbreakdown__blk1569_dn1, var_fbreakdown__blk1569_dn2, var_fbreakdown__blk1569_dn3, var_fbreakdown__blk1569_dn4, var_fbreakdown__blk1569_dn5, var_fbreakdown__blk1569_dn6, var_fbreakdown__blk1569_dn7, var_fbreakdown__blk1569_dn8, var_fbreakdown__blk1569_dn9, var_fbreakdown__blk1569_dn10, var_fbreakdown__blk1569_dn11, var_fbreakdown__blk1569_db0, var_fbreakdown__blk1569_db1, var_fbreakdown__blk1569_db2, var_fbreakdown__blk1569_db3, var_fbreakdown__blk1569_db4, var_fbreakdown__blk1569_db5, var_fbreakdown__blk1569_db6,)
    }
};
        var_fbreakdown__blk1569 = assign61430_e79823;
        var_fbreakdown__blk1569_dn0 = assign61430_e79823_d_n0;
        var_fbreakdown__blk1569_dn1 = assign61430_e79823_d_n1;
        var_fbreakdown__blk1569_dn2 = assign61430_e79823_d_n2;
        var_fbreakdown__blk1569_dn3 = assign61430_e79823_d_n3;
        var_fbreakdown__blk1569_dn4 = assign61430_e79823_d_n4;
        var_fbreakdown__blk1569_dn5 = assign61430_e79823_d_n5;
        var_fbreakdown__blk1569_dn6 = assign61430_e79823_d_n6;
        var_fbreakdown__blk1569_dn7 = assign61430_e79823_d_n7;
        var_fbreakdown__blk1569_dn8 = assign61430_e79823_d_n8;
        var_fbreakdown__blk1569_dn9 = assign61430_e79823_d_n9;
        var_fbreakdown__blk1569_dn10 = assign61430_e79823_d_n10;
        var_fbreakdown__blk1569_dn11 = assign61430_e79823_d_n11;
        var_fbreakdown__blk1569_db0 = assign61430_e79823_d_b0;
        var_fbreakdown__blk1569_db1 = assign61430_e79823_d_b1;
        var_fbreakdown__blk1569_db2 = assign61430_e79823_d_b2;
        var_fbreakdown__blk1569_db3 = assign61430_e79823_d_b3;
        var_fbreakdown__blk1569_db4 = assign61430_e79823_d_b4;
        var_fbreakdown__blk1569_db5 = assign61430_e79823_d_b5;
        var_fbreakdown__blk1569_db6 = assign61430_e79823_d_b6;

        let assign61440_e79826: f64 = (-var_alphaav);
        let assign61440_e79828: f64 = (assign61440_e79826 * var_vbrgat_var_d);
        let assign61440_e79829: f64 = if var_vav__blk1542 > assign61440_e79828 { 1.0 } else { 0.0 };
        var_guard1712 = assign61440_e79829;

        let assign61450_e79832: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard1713 = assign61450_e79832;

        *var_erfctimesexpmtat__blk1565_slot = var_erfctimesexpmtat__blk1565;
        *var_erfctimesexpmtat__blk1565_db0_slot = var_erfctimesexpmtat__blk1565_db0;
        *var_erfctimesexpmtat__blk1565_db1_slot = var_erfctimesexpmtat__blk1565_db1;
        *var_erfctimesexpmtat__blk1565_db2_slot = var_erfctimesexpmtat__blk1565_db2;
        *var_erfctimesexpmtat__blk1565_db3_slot = var_erfctimesexpmtat__blk1565_db3;
        *var_erfctimesexpmtat__blk1565_db4_slot = var_erfctimesexpmtat__blk1565_db4;
        *var_erfctimesexpmtat__blk1565_db5_slot = var_erfctimesexpmtat__blk1565_db5;
        *var_erfctimesexpmtat__blk1565_db6_slot = var_erfctimesexpmtat__blk1565_db6;
        *var_erfctimesexpmtat__blk1565_dn0_slot = var_erfctimesexpmtat__blk1565_dn0;
        *var_erfctimesexpmtat__blk1565_dn1_slot = var_erfctimesexpmtat__blk1565_dn1;
        *var_erfctimesexpmtat__blk1565_dn10_slot = var_erfctimesexpmtat__blk1565_dn10;
        *var_erfctimesexpmtat__blk1565_dn11_slot = var_erfctimesexpmtat__blk1565_dn11;
        *var_erfctimesexpmtat__blk1565_dn2_slot = var_erfctimesexpmtat__blk1565_dn2;
        *var_erfctimesexpmtat__blk1565_dn3_slot = var_erfctimesexpmtat__blk1565_dn3;
        *var_erfctimesexpmtat__blk1565_dn4_slot = var_erfctimesexpmtat__blk1565_dn4;
        *var_erfctimesexpmtat__blk1565_dn5_slot = var_erfctimesexpmtat__blk1565_dn5;
        *var_erfctimesexpmtat__blk1565_dn6_slot = var_erfctimesexpmtat__blk1565_dn6;
        *var_erfctimesexpmtat__blk1565_dn7_slot = var_erfctimesexpmtat__blk1565_dn7;
        *var_erfctimesexpmtat__blk1565_dn8_slot = var_erfctimesexpmtat__blk1565_dn8;
        *var_erfctimesexpmtat__blk1565_dn9_slot = var_erfctimesexpmtat__blk1565_dn9;
        *var_fbreakdown__blk1569_slot = var_fbreakdown__blk1569;
        *var_fbreakdown__blk1569_db0_slot = var_fbreakdown__blk1569_db0;
        *var_fbreakdown__blk1569_db1_slot = var_fbreakdown__blk1569_db1;
        *var_fbreakdown__blk1569_db2_slot = var_fbreakdown__blk1569_db2;
        *var_fbreakdown__blk1569_db3_slot = var_fbreakdown__blk1569_db3;
        *var_fbreakdown__blk1569_db4_slot = var_fbreakdown__blk1569_db4;
        *var_fbreakdown__blk1569_db5_slot = var_fbreakdown__blk1569_db5;
        *var_fbreakdown__blk1569_db6_slot = var_fbreakdown__blk1569_db6;
        *var_fbreakdown__blk1569_dn0_slot = var_fbreakdown__blk1569_dn0;
        *var_fbreakdown__blk1569_dn1_slot = var_fbreakdown__blk1569_dn1;
        *var_fbreakdown__blk1569_dn10_slot = var_fbreakdown__blk1569_dn10;
        *var_fbreakdown__blk1569_dn11_slot = var_fbreakdown__blk1569_dn11;
        *var_fbreakdown__blk1569_dn2_slot = var_fbreakdown__blk1569_dn2;
        *var_fbreakdown__blk1569_dn3_slot = var_fbreakdown__blk1569_dn3;
        *var_fbreakdown__blk1569_dn4_slot = var_fbreakdown__blk1569_dn4;
        *var_fbreakdown__blk1569_dn5_slot = var_fbreakdown__blk1569_dn5;
        *var_fbreakdown__blk1569_dn6_slot = var_fbreakdown__blk1569_dn6;
        *var_fbreakdown__blk1569_dn7_slot = var_fbreakdown__blk1569_dn7;
        *var_fbreakdown__blk1569_dn8_slot = var_fbreakdown__blk1569_dn8;
        *var_fbreakdown__blk1569_dn9_slot = var_fbreakdown__blk1569_dn9;
        *var_fmaxr__blk1568_slot = var_fmaxr__blk1568;
        *var_fmaxr__blk1568_db0_slot = var_fmaxr__blk1568_db0;
        *var_fmaxr__blk1568_db1_slot = var_fmaxr__blk1568_db1;
        *var_fmaxr__blk1568_db2_slot = var_fmaxr__blk1568_db2;
        *var_fmaxr__blk1568_db3_slot = var_fmaxr__blk1568_db3;
        *var_fmaxr__blk1568_db4_slot = var_fmaxr__blk1568_db4;
        *var_fmaxr__blk1568_db5_slot = var_fmaxr__blk1568_db5;
        *var_fmaxr__blk1568_db6_slot = var_fmaxr__blk1568_db6;
        *var_fmaxr__blk1568_dn0_slot = var_fmaxr__blk1568_dn0;
        *var_fmaxr__blk1568_dn1_slot = var_fmaxr__blk1568_dn1;
        *var_fmaxr__blk1568_dn10_slot = var_fmaxr__blk1568_dn10;
        *var_fmaxr__blk1568_dn11_slot = var_fmaxr__blk1568_dn11;
        *var_fmaxr__blk1568_dn2_slot = var_fmaxr__blk1568_dn2;
        *var_fmaxr__blk1568_dn3_slot = var_fmaxr__blk1568_dn3;
        *var_fmaxr__blk1568_dn4_slot = var_fmaxr__blk1568_dn4;
        *var_fmaxr__blk1568_dn5_slot = var_fmaxr__blk1568_dn5;
        *var_fmaxr__blk1568_dn6_slot = var_fmaxr__blk1568_dn6;
        *var_fmaxr__blk1568_dn7_slot = var_fmaxr__blk1568_dn7;
        *var_fmaxr__blk1568_dn8_slot = var_fmaxr__blk1568_dn8;
        *var_fmaxr__blk1568_dn9_slot = var_fmaxr__blk1568_dn9;
        *var_gammamax__blk1566_slot = var_gammamax__blk1566;
        *var_gammamax__blk1566_db0_slot = var_gammamax__blk1566_db0;
        *var_gammamax__blk1566_db1_slot = var_gammamax__blk1566_db1;
        *var_gammamax__blk1566_db2_slot = var_gammamax__blk1566_db2;
        *var_gammamax__blk1566_db3_slot = var_gammamax__blk1566_db3;
        *var_gammamax__blk1566_db4_slot = var_gammamax__blk1566_db4;
        *var_gammamax__blk1566_db5_slot = var_gammamax__blk1566_db5;
        *var_gammamax__blk1566_db6_slot = var_gammamax__blk1566_db6;
        *var_gammamax__blk1566_dn0_slot = var_gammamax__blk1566_dn0;
        *var_gammamax__blk1566_dn1_slot = var_gammamax__blk1566_dn1;
        *var_gammamax__blk1566_dn10_slot = var_gammamax__blk1566_dn10;
        *var_gammamax__blk1566_dn11_slot = var_gammamax__blk1566_dn11;
        *var_gammamax__blk1566_dn2_slot = var_gammamax__blk1566_dn2;
        *var_gammamax__blk1566_dn3_slot = var_gammamax__blk1566_dn3;
        *var_gammamax__blk1566_dn4_slot = var_gammamax__blk1566_dn4;
        *var_gammamax__blk1566_dn5_slot = var_gammamax__blk1566_dn5;
        *var_gammamax__blk1566_dn6_slot = var_gammamax__blk1566_dn6;
        *var_gammamax__blk1566_dn7_slot = var_gammamax__blk1566_dn7;
        *var_gammamax__blk1566_dn8_slot = var_gammamax__blk1566_dn8;
        *var_gammamax__blk1566_dn9_slot = var_gammamax__blk1566_dn9;
        *var_guard1707_slot = var_guard1707;
        *var_guard1708_slot = var_guard1708;
        *var_guard1709_slot = var_guard1709;
        *var_guard1710_slot = var_guard1710;
        *var_guard1711_slot = var_guard1711;
        *var_guard1712_slot = var_guard1712;
        *var_guard1713_slot = var_guard1713;
        *var_ibbt__blk1567_slot = var_ibbt__blk1567;
        *var_ibbt__blk1567_db0_slot = var_ibbt__blk1567_db0;
        *var_ibbt__blk1567_db1_slot = var_ibbt__blk1567_db1;
        *var_ibbt__blk1567_db2_slot = var_ibbt__blk1567_db2;
        *var_ibbt__blk1567_db3_slot = var_ibbt__blk1567_db3;
        *var_ibbt__blk1567_db4_slot = var_ibbt__blk1567_db4;
        *var_ibbt__blk1567_db5_slot = var_ibbt__blk1567_db5;
        *var_ibbt__blk1567_db6_slot = var_ibbt__blk1567_db6;
        *var_ibbt__blk1567_dn0_slot = var_ibbt__blk1567_dn0;
        *var_ibbt__blk1567_dn1_slot = var_ibbt__blk1567_dn1;
        *var_ibbt__blk1567_dn10_slot = var_ibbt__blk1567_dn10;
        *var_ibbt__blk1567_dn11_slot = var_ibbt__blk1567_dn11;
        *var_ibbt__blk1567_dn2_slot = var_ibbt__blk1567_dn2;
        *var_ibbt__blk1567_dn3_slot = var_ibbt__blk1567_dn3;
        *var_ibbt__blk1567_dn4_slot = var_ibbt__blk1567_dn4;
        *var_ibbt__blk1567_dn5_slot = var_ibbt__blk1567_dn5;
        *var_ibbt__blk1567_dn6_slot = var_ibbt__blk1567_dn6;
        *var_ibbt__blk1567_dn7_slot = var_ibbt__blk1567_dn7;
        *var_ibbt__blk1567_dn8_slot = var_ibbt__blk1567_dn8;
        *var_ibbt__blk1567_dn9_slot = var_ibbt__blk1567_dn9;
        *var_itat__blk1552_slot = var_itat__blk1552;
        *var_itat__blk1552_db0_slot = var_itat__blk1552_db0;
        *var_itat__blk1552_db1_slot = var_itat__blk1552_db1;
        *var_itat__blk1552_db2_slot = var_itat__blk1552_db2;
        *var_itat__blk1552_db3_slot = var_itat__blk1552_db3;
        *var_itat__blk1552_db4_slot = var_itat__blk1552_db4;
        *var_itat__blk1552_db5_slot = var_itat__blk1552_db5;
        *var_itat__blk1552_db6_slot = var_itat__blk1552_db6;
        *var_itat__blk1552_dn0_slot = var_itat__blk1552_dn0;
        *var_itat__blk1552_dn1_slot = var_itat__blk1552_dn1;
        *var_itat__blk1552_dn10_slot = var_itat__blk1552_dn10;
        *var_itat__blk1552_dn11_slot = var_itat__blk1552_dn11;
        *var_itat__blk1552_dn2_slot = var_itat__blk1552_dn2;
        *var_itat__blk1552_dn3_slot = var_itat__blk1552_dn3;
        *var_itat__blk1552_dn4_slot = var_itat__blk1552_dn4;
        *var_itat__blk1552_dn5_slot = var_itat__blk1552_dn5;
        *var_itat__blk1552_dn6_slot = var_itat__blk1552_dn6;
        *var_itat__blk1552_dn7_slot = var_itat__blk1552_dn7;
        *var_itat__blk1552_dn8_slot = var_itat__blk1552_dn8;
        *var_itat__blk1552_dn9_slot = var_itat__blk1552_dn9;
        *var_tmp__blk1543_slot = var_tmp__blk1543;
        *var_tmp__blk1543_db0_slot = var_tmp__blk1543_db0;
        *var_tmp__blk1543_db1_slot = var_tmp__blk1543_db1;
        *var_tmp__blk1543_db2_slot = var_tmp__blk1543_db2;
        *var_tmp__blk1543_db3_slot = var_tmp__blk1543_db3;
        *var_tmp__blk1543_db4_slot = var_tmp__blk1543_db4;
        *var_tmp__blk1543_db5_slot = var_tmp__blk1543_db5;
        *var_tmp__blk1543_db6_slot = var_tmp__blk1543_db6;
        *var_tmp__blk1543_dn0_slot = var_tmp__blk1543_dn0;
        *var_tmp__blk1543_dn1_slot = var_tmp__blk1543_dn1;
        *var_tmp__blk1543_dn10_slot = var_tmp__blk1543_dn10;
        *var_tmp__blk1543_dn11_slot = var_tmp__blk1543_dn11;
        *var_tmp__blk1543_dn2_slot = var_tmp__blk1543_dn2;
        *var_tmp__blk1543_dn3_slot = var_tmp__blk1543_dn3;
        *var_tmp__blk1543_dn4_slot = var_tmp__blk1543_dn4;
        *var_tmp__blk1543_dn5_slot = var_tmp__blk1543_dn5;
        *var_tmp__blk1543_dn6_slot = var_tmp__blk1543_dn6;
        *var_tmp__blk1543_dn7_slot = var_tmp__blk1543_dn7;
        *var_tmp__blk1543_dn8_slot = var_tmp__blk1543_dn8;
        *var_tmp__blk1543_dn9_slot = var_tmp__blk1543_dn9;
    }

    pub(super) fn stamp_transient_block_242(
        p: &Parameters,
        var_alphaav: f64,
        var_anugatd_i: f64,
        var_fstopgat_d: f64,
        var_guard1572: f64,
        var_guard1573: f64,
        var_guard1697: f64,
        var_guard1711: f64,
        var_guard1712: f64,
        var_guard1713: f64,
        var_ibbt__blk1567: f64,
        var_ibbt__blk1567_db0: f64,
        var_ibbt__blk1567_db1: f64,
        var_ibbt__blk1567_db2: f64,
        var_ibbt__blk1567_db3: f64,
        var_ibbt__blk1567_db4: f64,
        var_ibbt__blk1567_db5: f64,
        var_ibbt__blk1567_db6: f64,
        var_ibbt__blk1567_dn0: f64,
        var_ibbt__blk1567_dn1: f64,
        var_ibbt__blk1567_dn10: f64,
        var_ibbt__blk1567_dn11: f64,
        var_ibbt__blk1567_dn2: f64,
        var_ibbt__blk1567_dn3: f64,
        var_ibbt__blk1567_dn4: f64,
        var_ibbt__blk1567_dn5: f64,
        var_ibbt__blk1567_dn6: f64,
        var_ibbt__blk1567_dn7: f64,
        var_ibbt__blk1567_dn8: f64,
        var_ibbt__blk1567_dn9: f64,
        var_id__blk1544: f64,
        var_id__blk1544_db0: f64,
        var_id__blk1544_db1: f64,
        var_id__blk1544_db2: f64,
        var_id__blk1544_db3: f64,
        var_id__blk1544_db4: f64,
        var_id__blk1544_db5: f64,
        var_id__blk1544_db6: f64,
        var_id__blk1544_dn0: f64,
        var_id__blk1544_dn1: f64,
        var_id__blk1544_dn10: f64,
        var_id__blk1544_dn11: f64,
        var_id__blk1544_dn2: f64,
        var_id__blk1544_dn3: f64,
        var_id__blk1544_dn4: f64,
        var_id__blk1544_dn5: f64,
        var_id__blk1544_dn6: f64,
        var_id__blk1544_dn7: f64,
        var_id__blk1544_dn8: f64,
        var_id__blk1544_dn9: f64,
        var_isrh__blk1545: f64,
        var_isrh__blk1545_db0: f64,
        var_isrh__blk1545_db1: f64,
        var_isrh__blk1545_db2: f64,
        var_isrh__blk1545_db3: f64,
        var_isrh__blk1545_db4: f64,
        var_isrh__blk1545_db5: f64,
        var_isrh__blk1545_db6: f64,
        var_isrh__blk1545_dn0: f64,
        var_isrh__blk1545_dn1: f64,
        var_isrh__blk1545_dn10: f64,
        var_isrh__blk1545_dn11: f64,
        var_isrh__blk1545_dn2: f64,
        var_isrh__blk1545_dn3: f64,
        var_isrh__blk1545_dn4: f64,
        var_isrh__blk1545_dn5: f64,
        var_isrh__blk1545_dn6: f64,
        var_isrh__blk1545_dn7: f64,
        var_isrh__blk1545_dn8: f64,
        var_isrh__blk1545_dn9: f64,
        var_itat__blk1552: f64,
        var_itat__blk1552_db0: f64,
        var_itat__blk1552_db1: f64,
        var_itat__blk1552_db2: f64,
        var_itat__blk1552_db3: f64,
        var_itat__blk1552_db4: f64,
        var_itat__blk1552_db5: f64,
        var_itat__blk1552_db6: f64,
        var_itat__blk1552_dn0: f64,
        var_itat__blk1552_dn1: f64,
        var_itat__blk1552_dn10: f64,
        var_itat__blk1552_dn11: f64,
        var_itat__blk1552_dn2: f64,
        var_itat__blk1552_dn3: f64,
        var_itat__blk1552_dn4: f64,
        var_itat__blk1552_dn5: f64,
        var_itat__blk1552_dn6: f64,
        var_itat__blk1552_dn7: f64,
        var_itat__blk1552_dn8: f64,
        var_itat__blk1552_dn9: f64,
        var_one_minus_pgat_d: f64,
        var_pbrgatd_i: f64,
        var_qpref2gat_d: f64,
        var_qprefgat_d: f64,
        var_slopegat_d: f64,
        var_slopegat_d_db0: f64,
        var_slopegat_d_db1: f64,
        var_slopegat_d_db2: f64,
        var_slopegat_d_db3: f64,
        var_slopegat_d_db4: f64,
        var_slopegat_d_db5: f64,
        var_slopegat_d_db6: f64,
        var_slopegat_d_dn0: f64,
        var_slopegat_d_dn1: f64,
        var_slopegat_d_dn10: f64,
        var_slopegat_d_dn11: f64,
        var_slopegat_d_dn2: f64,
        var_slopegat_d_dn3: f64,
        var_slopegat_d_dn4: f64,
        var_slopegat_d_dn5: f64,
        var_slopegat_d_dn6: f64,
        var_slopegat_d_dn7: f64,
        var_slopegat_d_dn8: f64,
        var_slopegat_d_dn9: f64,
        var_swgat2nd_d: f64,
        var_vav__blk1542: f64,
        var_vav__blk1542_db0: f64,
        var_vav__blk1542_db1: f64,
        var_vav__blk1542_db2: f64,
        var_vav__blk1542_db3: f64,
        var_vav__blk1542_db4: f64,
        var_vav__blk1542_db5: f64,
        var_vav__blk1542_db6: f64,
        var_vav__blk1542_dn0: f64,
        var_vav__blk1542_dn1: f64,
        var_vav__blk1542_dn10: f64,
        var_vav__blk1542_dn11: f64,
        var_vav__blk1542_dn2: f64,
        var_vav__blk1542_dn3: f64,
        var_vav__blk1542_dn4: f64,
        var_vav__blk1542_dn5: f64,
        var_vav__blk1542_dn6: f64,
        var_vav__blk1542_dn7: f64,
        var_vav__blk1542_dn8: f64,
        var_vav__blk1542_dn9: f64,
        var_vbiinvgat_d: f64,
        var_vbrgat_var_d: f64,
        var_vbrgat_var_d_db0: f64,
        var_vbrgat_var_d_db1: f64,
        var_vbrgat_var_d_db2: f64,
        var_vbrgat_var_d_db3: f64,
        var_vbrgat_var_d_db4: f64,
        var_vbrgat_var_d_db5: f64,
        var_vbrgat_var_d_db6: f64,
        var_vbrgat_var_d_dn0: f64,
        var_vbrgat_var_d_dn1: f64,
        var_vbrgat_var_d_dn10: f64,
        var_vbrgat_var_d_dn11: f64,
        var_vbrgat_var_d_dn2: f64,
        var_vbrgat_var_d_dn3: f64,
        var_vbrgat_var_d_dn4: f64,
        var_vbrgat_var_d_dn5: f64,
        var_vbrgat_var_d_dn6: f64,
        var_vbrgat_var_d_dn7: f64,
        var_vbrgat_var_d_dn8: f64,
        var_vbrgat_var_d_dn9: f64,
        var_vbrinvgat_d: f64,
        var_vbrinvgat_d_db0: f64,
        var_vbrinvgat_d_db1: f64,
        var_vbrinvgat_d_db2: f64,
        var_vbrinvgat_d_db3: f64,
        var_vbrinvgat_d_db4: f64,
        var_vbrinvgat_d_db5: f64,
        var_vbrinvgat_d_db6: f64,
        var_vbrinvgat_d_dn0: f64,
        var_vbrinvgat_d_dn1: f64,
        var_vbrinvgat_d_dn10: f64,
        var_vbrinvgat_d_dn11: f64,
        var_vbrinvgat_d_dn2: f64,
        var_vbrinvgat_d_dn3: f64,
        var_vbrinvgat_d_dn4: f64,
        var_vbrinvgat_d_dn5: f64,
        var_vbrinvgat_d_dn6: f64,
        var_vbrinvgat_d_dn7: f64,
        var_vbrinvgat_d_dn8: f64,
        var_vbrinvgat_d_dn9: f64,
        var_vch_d: f64,
        var_vfmin_d: f64,
        var_vjun_d: f64,
        var_vjun_d_db0: f64,
        var_vjun_d_db1: f64,
        var_vjun_d_db2: f64,
        var_vjun_d_db3: f64,
        var_vjun_d_db4: f64,
        var_vjun_d_db5: f64,
        var_vjun_d_db6: f64,
        var_vjun_d_dn0: f64,
        var_vjun_d_dn1: f64,
        var_vjun_d_dn10: f64,
        var_vjun_d_dn11: f64,
        var_vjun_d_dn2: f64,
        var_vjun_d_dn3: f64,
        var_vjun_d_dn4: f64,
        var_vjun_d_dn5: f64,
        var_vjun_d_dn6: f64,
        var_vjun_d_dn7: f64,
        var_vjun_d_dn8: f64,
        var_vjun_d_dn9: f64,
        var_vtrgatd_i: f64,
        var_fbreakdown__blk1569_slot: &mut f64,
        var_fbreakdown__blk1569_db0_slot: &mut f64,
        var_fbreakdown__blk1569_db1_slot: &mut f64,
        var_fbreakdown__blk1569_db2_slot: &mut f64,
        var_fbreakdown__blk1569_db3_slot: &mut f64,
        var_fbreakdown__blk1569_db4_slot: &mut f64,
        var_fbreakdown__blk1569_db5_slot: &mut f64,
        var_fbreakdown__blk1569_db6_slot: &mut f64,
        var_fbreakdown__blk1569_dn0_slot: &mut f64,
        var_fbreakdown__blk1569_dn1_slot: &mut f64,
        var_fbreakdown__blk1569_dn10_slot: &mut f64,
        var_fbreakdown__blk1569_dn11_slot: &mut f64,
        var_fbreakdown__blk1569_dn2_slot: &mut f64,
        var_fbreakdown__blk1569_dn3_slot: &mut f64,
        var_fbreakdown__blk1569_dn4_slot: &mut f64,
        var_fbreakdown__blk1569_dn5_slot: &mut f64,
        var_fbreakdown__blk1569_dn6_slot: &mut f64,
        var_fbreakdown__blk1569_dn7_slot: &mut f64,
        var_fbreakdown__blk1569_dn8_slot: &mut f64,
        var_fbreakdown__blk1569_dn9_slot: &mut f64,
        var_guard1714_slot: &mut f64,
        var_guard1715_slot: &mut f64,
        var_h1__blk1528_slot: &mut f64,
        var_h2__blk1529_slot: &mut f64,
        var_h2d__blk1530_slot: &mut f64,
        var_h2d__blk1530_db0_slot: &mut f64,
        var_h2d__blk1530_db1_slot: &mut f64,
        var_h2d__blk1530_db2_slot: &mut f64,
        var_h2d__blk1530_db3_slot: &mut f64,
        var_h2d__blk1530_db4_slot: &mut f64,
        var_h2d__blk1530_db5_slot: &mut f64,
        var_h2d__blk1530_db6_slot: &mut f64,
        var_h2d__blk1530_dn0_slot: &mut f64,
        var_h2d__blk1530_dn1_slot: &mut f64,
        var_h2d__blk1530_dn10_slot: &mut f64,
        var_h2d__blk1530_dn11_slot: &mut f64,
        var_h2d__blk1530_dn2_slot: &mut f64,
        var_h2d__blk1530_dn3_slot: &mut f64,
        var_h2d__blk1530_dn4_slot: &mut f64,
        var_h2d__blk1530_dn5_slot: &mut f64,
        var_h2d__blk1530_dn6_slot: &mut f64,
        var_h2d__blk1530_dn7_slot: &mut f64,
        var_h2d__blk1530_dn8_slot: &mut f64,
        var_h2d__blk1530_dn9_slot: &mut f64,
        var_h3__blk1531_slot: &mut f64,
        var_h3__blk1531_db0_slot: &mut f64,
        var_h3__blk1531_db1_slot: &mut f64,
        var_h3__blk1531_db2_slot: &mut f64,
        var_h3__blk1531_db3_slot: &mut f64,
        var_h3__blk1531_db4_slot: &mut f64,
        var_h3__blk1531_db5_slot: &mut f64,
        var_h3__blk1531_db6_slot: &mut f64,
        var_h3__blk1531_dn0_slot: &mut f64,
        var_h3__blk1531_dn1_slot: &mut f64,
        var_h3__blk1531_dn10_slot: &mut f64,
        var_h3__blk1531_dn11_slot: &mut f64,
        var_h3__blk1531_dn2_slot: &mut f64,
        var_h3__blk1531_dn3_slot: &mut f64,
        var_h3__blk1531_dn4_slot: &mut f64,
        var_h3__blk1531_dn5_slot: &mut f64,
        var_h3__blk1531_dn6_slot: &mut f64,
        var_h3__blk1531_dn7_slot: &mut f64,
        var_h3__blk1531_dn8_slot: &mut f64,
        var_h3__blk1531_dn9_slot: &mut f64,
        var_h4__blk1532_slot: &mut f64,
        var_h4__blk1532_db0_slot: &mut f64,
        var_h4__blk1532_db1_slot: &mut f64,
        var_h4__blk1532_db2_slot: &mut f64,
        var_h4__blk1532_db3_slot: &mut f64,
        var_h4__blk1532_db4_slot: &mut f64,
        var_h4__blk1532_db5_slot: &mut f64,
        var_h4__blk1532_db6_slot: &mut f64,
        var_h4__blk1532_dn0_slot: &mut f64,
        var_h4__blk1532_dn1_slot: &mut f64,
        var_h4__blk1532_dn10_slot: &mut f64,
        var_h4__blk1532_dn11_slot: &mut f64,
        var_h4__blk1532_dn2_slot: &mut f64,
        var_h4__blk1532_dn3_slot: &mut f64,
        var_h4__blk1532_dn4_slot: &mut f64,
        var_h4__blk1532_dn5_slot: &mut f64,
        var_h4__blk1532_dn6_slot: &mut f64,
        var_h4__blk1532_dn7_slot: &mut f64,
        var_h4__blk1532_dn8_slot: &mut f64,
        var_h4__blk1532_dn9_slot: &mut f64,
        var_h5__blk1533_slot: &mut f64,
        var_h5__blk1533_db0_slot: &mut f64,
        var_h5__blk1533_db1_slot: &mut f64,
        var_h5__blk1533_db2_slot: &mut f64,
        var_h5__blk1533_db3_slot: &mut f64,
        var_h5__blk1533_db4_slot: &mut f64,
        var_h5__blk1533_db5_slot: &mut f64,
        var_h5__blk1533_db6_slot: &mut f64,
        var_h5__blk1533_dn0_slot: &mut f64,
        var_h5__blk1533_dn1_slot: &mut f64,
        var_h5__blk1533_dn10_slot: &mut f64,
        var_h5__blk1533_dn11_slot: &mut f64,
        var_h5__blk1533_dn2_slot: &mut f64,
        var_h5__blk1533_dn3_slot: &mut f64,
        var_h5__blk1533_dn4_slot: &mut f64,
        var_h5__blk1533_dn5_slot: &mut f64,
        var_h5__blk1533_dn6_slot: &mut f64,
        var_h5__blk1533_dn7_slot: &mut f64,
        var_h5__blk1533_dn8_slot: &mut f64,
        var_h5__blk1533_dn9_slot: &mut f64,
        var_ijungat_d_slot: &mut f64,
        var_ijungat_d_db0_slot: &mut f64,
        var_ijungat_d_db1_slot: &mut f64,
        var_ijungat_d_db2_slot: &mut f64,
        var_ijungat_d_db3_slot: &mut f64,
        var_ijungat_d_db4_slot: &mut f64,
        var_ijungat_d_db5_slot: &mut f64,
        var_ijungat_d_db6_slot: &mut f64,
        var_ijungat_d_dn0_slot: &mut f64,
        var_ijungat_d_dn1_slot: &mut f64,
        var_ijungat_d_dn10_slot: &mut f64,
        var_ijungat_d_dn11_slot: &mut f64,
        var_ijungat_d_dn2_slot: &mut f64,
        var_ijungat_d_dn3_slot: &mut f64,
        var_ijungat_d_dn4_slot: &mut f64,
        var_ijungat_d_dn5_slot: &mut f64,
        var_ijungat_d_dn6_slot: &mut f64,
        var_ijungat_d_dn7_slot: &mut f64,
        var_ijungat_d_dn8_slot: &mut f64,
        var_ijungat_d_dn9_slot: &mut f64,
        var_nu__blk1570_slot: &mut f64,
        var_nu__blk1570_db0_slot: &mut f64,
        var_nu__blk1570_db1_slot: &mut f64,
        var_nu__blk1570_db2_slot: &mut f64,
        var_nu__blk1570_db3_slot: &mut f64,
        var_nu__blk1570_db4_slot: &mut f64,
        var_nu__blk1570_db5_slot: &mut f64,
        var_nu__blk1570_db6_slot: &mut f64,
        var_nu__blk1570_dn0_slot: &mut f64,
        var_nu__blk1570_dn1_slot: &mut f64,
        var_nu__blk1570_dn10_slot: &mut f64,
        var_nu__blk1570_dn11_slot: &mut f64,
        var_nu__blk1570_dn2_slot: &mut f64,
        var_nu__blk1570_dn3_slot: &mut f64,
        var_nu__blk1570_dn4_slot: &mut f64,
        var_nu__blk1570_dn5_slot: &mut f64,
        var_nu__blk1570_dn6_slot: &mut f64,
        var_nu__blk1570_dn7_slot: &mut f64,
        var_nu__blk1570_dn8_slot: &mut f64,
        var_nu__blk1570_dn9_slot: &mut f64,
        var_qjungat_d_slot: &mut f64,
        var_qjungat_d_db0_slot: &mut f64,
        var_qjungat_d_db1_slot: &mut f64,
        var_qjungat_d_db2_slot: &mut f64,
        var_qjungat_d_db3_slot: &mut f64,
        var_qjungat_d_db4_slot: &mut f64,
        var_qjungat_d_db5_slot: &mut f64,
        var_qjungat_d_db6_slot: &mut f64,
        var_qjungat_d_dn0_slot: &mut f64,
        var_qjungat_d_dn1_slot: &mut f64,
        var_qjungat_d_dn10_slot: &mut f64,
        var_qjungat_d_dn11_slot: &mut f64,
        var_qjungat_d_dn2_slot: &mut f64,
        var_qjungat_d_dn3_slot: &mut f64,
        var_qjungat_d_dn4_slot: &mut f64,
        var_qjungat_d_dn5_slot: &mut f64,
        var_qjungat_d_dn6_slot: &mut f64,
        var_qjungat_d_dn7_slot: &mut f64,
        var_qjungat_d_dn8_slot: &mut f64,
        var_qjungat_d_dn9_slot: &mut f64,
        var_tmp__blk1543_slot: &mut f64,
        var_tmp__blk1543_db0_slot: &mut f64,
        var_tmp__blk1543_db1_slot: &mut f64,
        var_tmp__blk1543_db2_slot: &mut f64,
        var_tmp__blk1543_db3_slot: &mut f64,
        var_tmp__blk1543_db4_slot: &mut f64,
        var_tmp__blk1543_db5_slot: &mut f64,
        var_tmp__blk1543_db6_slot: &mut f64,
        var_tmp__blk1543_dn0_slot: &mut f64,
        var_tmp__blk1543_dn1_slot: &mut f64,
        var_tmp__blk1543_dn10_slot: &mut f64,
        var_tmp__blk1543_dn11_slot: &mut f64,
        var_tmp__blk1543_dn2_slot: &mut f64,
        var_tmp__blk1543_dn3_slot: &mut f64,
        var_tmp__blk1543_dn4_slot: &mut f64,
        var_tmp__blk1543_dn5_slot: &mut f64,
        var_tmp__blk1543_dn6_slot: &mut f64,
        var_tmp__blk1543_dn7_slot: &mut f64,
        var_tmp__blk1543_dn8_slot: &mut f64,
        var_tmp__blk1543_dn9_slot: &mut f64,
        var_vjtmp_slot: &mut f64,
        var_vjtmp_db0_slot: &mut f64,
        var_vjtmp_db1_slot: &mut f64,
        var_vjtmp_db2_slot: &mut f64,
        var_vjtmp_db3_slot: &mut f64,
        var_vjtmp_db4_slot: &mut f64,
        var_vjtmp_db5_slot: &mut f64,
        var_vjtmp_db6_slot: &mut f64,
        var_vjtmp_dn0_slot: &mut f64,
        var_vjtmp_dn1_slot: &mut f64,
        var_vjtmp_dn10_slot: &mut f64,
        var_vjtmp_dn11_slot: &mut f64,
        var_vjtmp_dn2_slot: &mut f64,
        var_vjtmp_dn3_slot: &mut f64,
        var_vjtmp_dn4_slot: &mut f64,
        var_vjtmp_dn5_slot: &mut f64,
        var_vjtmp_dn6_slot: &mut f64,
        var_vjtmp_dn7_slot: &mut f64,
        var_vjtmp_dn8_slot: &mut f64,
        var_vjtmp_dn9_slot: &mut f64,
    ) {
        let mut var_fbreakdown__blk1569: f64 = *var_fbreakdown__blk1569_slot;
        let mut var_fbreakdown__blk1569_db0: f64 = *var_fbreakdown__blk1569_db0_slot;
        let mut var_fbreakdown__blk1569_db1: f64 = *var_fbreakdown__blk1569_db1_slot;
        let mut var_fbreakdown__blk1569_db2: f64 = *var_fbreakdown__blk1569_db2_slot;
        let mut var_fbreakdown__blk1569_db3: f64 = *var_fbreakdown__blk1569_db3_slot;
        let mut var_fbreakdown__blk1569_db4: f64 = *var_fbreakdown__blk1569_db4_slot;
        let mut var_fbreakdown__blk1569_db5: f64 = *var_fbreakdown__blk1569_db5_slot;
        let mut var_fbreakdown__blk1569_db6: f64 = *var_fbreakdown__blk1569_db6_slot;
        let mut var_fbreakdown__blk1569_dn0: f64 = *var_fbreakdown__blk1569_dn0_slot;
        let mut var_fbreakdown__blk1569_dn1: f64 = *var_fbreakdown__blk1569_dn1_slot;
        let mut var_fbreakdown__blk1569_dn10: f64 = *var_fbreakdown__blk1569_dn10_slot;
        let mut var_fbreakdown__blk1569_dn11: f64 = *var_fbreakdown__blk1569_dn11_slot;
        let mut var_fbreakdown__blk1569_dn2: f64 = *var_fbreakdown__blk1569_dn2_slot;
        let mut var_fbreakdown__blk1569_dn3: f64 = *var_fbreakdown__blk1569_dn3_slot;
        let mut var_fbreakdown__blk1569_dn4: f64 = *var_fbreakdown__blk1569_dn4_slot;
        let mut var_fbreakdown__blk1569_dn5: f64 = *var_fbreakdown__blk1569_dn5_slot;
        let mut var_fbreakdown__blk1569_dn6: f64 = *var_fbreakdown__blk1569_dn6_slot;
        let mut var_fbreakdown__blk1569_dn7: f64 = *var_fbreakdown__blk1569_dn7_slot;
        let mut var_fbreakdown__blk1569_dn8: f64 = *var_fbreakdown__blk1569_dn8_slot;
        let mut var_fbreakdown__blk1569_dn9: f64 = *var_fbreakdown__blk1569_dn9_slot;
        let mut var_guard1714: f64 = *var_guard1714_slot;
        let mut var_guard1715: f64 = *var_guard1715_slot;
        let mut var_h1__blk1528: f64 = *var_h1__blk1528_slot;
        let mut var_h2__blk1529: f64 = *var_h2__blk1529_slot;
        let mut var_h2d__blk1530: f64 = *var_h2d__blk1530_slot;
        let mut var_h2d__blk1530_db0: f64 = *var_h2d__blk1530_db0_slot;
        let mut var_h2d__blk1530_db1: f64 = *var_h2d__blk1530_db1_slot;
        let mut var_h2d__blk1530_db2: f64 = *var_h2d__blk1530_db2_slot;
        let mut var_h2d__blk1530_db3: f64 = *var_h2d__blk1530_db3_slot;
        let mut var_h2d__blk1530_db4: f64 = *var_h2d__blk1530_db4_slot;
        let mut var_h2d__blk1530_db5: f64 = *var_h2d__blk1530_db5_slot;
        let mut var_h2d__blk1530_db6: f64 = *var_h2d__blk1530_db6_slot;
        let mut var_h2d__blk1530_dn0: f64 = *var_h2d__blk1530_dn0_slot;
        let mut var_h2d__blk1530_dn1: f64 = *var_h2d__blk1530_dn1_slot;
        let mut var_h2d__blk1530_dn10: f64 = *var_h2d__blk1530_dn10_slot;
        let mut var_h2d__blk1530_dn11: f64 = *var_h2d__blk1530_dn11_slot;
        let mut var_h2d__blk1530_dn2: f64 = *var_h2d__blk1530_dn2_slot;
        let mut var_h2d__blk1530_dn3: f64 = *var_h2d__blk1530_dn3_slot;
        let mut var_h2d__blk1530_dn4: f64 = *var_h2d__blk1530_dn4_slot;
        let mut var_h2d__blk1530_dn5: f64 = *var_h2d__blk1530_dn5_slot;
        let mut var_h2d__blk1530_dn6: f64 = *var_h2d__blk1530_dn6_slot;
        let mut var_h2d__blk1530_dn7: f64 = *var_h2d__blk1530_dn7_slot;
        let mut var_h2d__blk1530_dn8: f64 = *var_h2d__blk1530_dn8_slot;
        let mut var_h2d__blk1530_dn9: f64 = *var_h2d__blk1530_dn9_slot;
        let mut var_h3__blk1531: f64 = *var_h3__blk1531_slot;
        let mut var_h3__blk1531_db0: f64 = *var_h3__blk1531_db0_slot;
        let mut var_h3__blk1531_db1: f64 = *var_h3__blk1531_db1_slot;
        let mut var_h3__blk1531_db2: f64 = *var_h3__blk1531_db2_slot;
        let mut var_h3__blk1531_db3: f64 = *var_h3__blk1531_db3_slot;
        let mut var_h3__blk1531_db4: f64 = *var_h3__blk1531_db4_slot;
        let mut var_h3__blk1531_db5: f64 = *var_h3__blk1531_db5_slot;
        let mut var_h3__blk1531_db6: f64 = *var_h3__blk1531_db6_slot;
        let mut var_h3__blk1531_dn0: f64 = *var_h3__blk1531_dn0_slot;
        let mut var_h3__blk1531_dn1: f64 = *var_h3__blk1531_dn1_slot;
        let mut var_h3__blk1531_dn10: f64 = *var_h3__blk1531_dn10_slot;
        let mut var_h3__blk1531_dn11: f64 = *var_h3__blk1531_dn11_slot;
        let mut var_h3__blk1531_dn2: f64 = *var_h3__blk1531_dn2_slot;
        let mut var_h3__blk1531_dn3: f64 = *var_h3__blk1531_dn3_slot;
        let mut var_h3__blk1531_dn4: f64 = *var_h3__blk1531_dn4_slot;
        let mut var_h3__blk1531_dn5: f64 = *var_h3__blk1531_dn5_slot;
        let mut var_h3__blk1531_dn6: f64 = *var_h3__blk1531_dn6_slot;
        let mut var_h3__blk1531_dn7: f64 = *var_h3__blk1531_dn7_slot;
        let mut var_h3__blk1531_dn8: f64 = *var_h3__blk1531_dn8_slot;
        let mut var_h3__blk1531_dn9: f64 = *var_h3__blk1531_dn9_slot;
        let mut var_h4__blk1532: f64 = *var_h4__blk1532_slot;
        let mut var_h4__blk1532_db0: f64 = *var_h4__blk1532_db0_slot;
        let mut var_h4__blk1532_db1: f64 = *var_h4__blk1532_db1_slot;
        let mut var_h4__blk1532_db2: f64 = *var_h4__blk1532_db2_slot;
        let mut var_h4__blk1532_db3: f64 = *var_h4__blk1532_db3_slot;
        let mut var_h4__blk1532_db4: f64 = *var_h4__blk1532_db4_slot;
        let mut var_h4__blk1532_db5: f64 = *var_h4__blk1532_db5_slot;
        let mut var_h4__blk1532_db6: f64 = *var_h4__blk1532_db6_slot;
        let mut var_h4__blk1532_dn0: f64 = *var_h4__blk1532_dn0_slot;
        let mut var_h4__blk1532_dn1: f64 = *var_h4__blk1532_dn1_slot;
        let mut var_h4__blk1532_dn10: f64 = *var_h4__blk1532_dn10_slot;
        let mut var_h4__blk1532_dn11: f64 = *var_h4__blk1532_dn11_slot;
        let mut var_h4__blk1532_dn2: f64 = *var_h4__blk1532_dn2_slot;
        let mut var_h4__blk1532_dn3: f64 = *var_h4__blk1532_dn3_slot;
        let mut var_h4__blk1532_dn4: f64 = *var_h4__blk1532_dn4_slot;
        let mut var_h4__blk1532_dn5: f64 = *var_h4__blk1532_dn5_slot;
        let mut var_h4__blk1532_dn6: f64 = *var_h4__blk1532_dn6_slot;
        let mut var_h4__blk1532_dn7: f64 = *var_h4__blk1532_dn7_slot;
        let mut var_h4__blk1532_dn8: f64 = *var_h4__blk1532_dn8_slot;
        let mut var_h4__blk1532_dn9: f64 = *var_h4__blk1532_dn9_slot;
        let mut var_h5__blk1533: f64 = *var_h5__blk1533_slot;
        let mut var_h5__blk1533_db0: f64 = *var_h5__blk1533_db0_slot;
        let mut var_h5__blk1533_db1: f64 = *var_h5__blk1533_db1_slot;
        let mut var_h5__blk1533_db2: f64 = *var_h5__blk1533_db2_slot;
        let mut var_h5__blk1533_db3: f64 = *var_h5__blk1533_db3_slot;
        let mut var_h5__blk1533_db4: f64 = *var_h5__blk1533_db4_slot;
        let mut var_h5__blk1533_db5: f64 = *var_h5__blk1533_db5_slot;
        let mut var_h5__blk1533_db6: f64 = *var_h5__blk1533_db6_slot;
        let mut var_h5__blk1533_dn0: f64 = *var_h5__blk1533_dn0_slot;
        let mut var_h5__blk1533_dn1: f64 = *var_h5__blk1533_dn1_slot;
        let mut var_h5__blk1533_dn10: f64 = *var_h5__blk1533_dn10_slot;
        let mut var_h5__blk1533_dn11: f64 = *var_h5__blk1533_dn11_slot;
        let mut var_h5__blk1533_dn2: f64 = *var_h5__blk1533_dn2_slot;
        let mut var_h5__blk1533_dn3: f64 = *var_h5__blk1533_dn3_slot;
        let mut var_h5__blk1533_dn4: f64 = *var_h5__blk1533_dn4_slot;
        let mut var_h5__blk1533_dn5: f64 = *var_h5__blk1533_dn5_slot;
        let mut var_h5__blk1533_dn6: f64 = *var_h5__blk1533_dn6_slot;
        let mut var_h5__blk1533_dn7: f64 = *var_h5__blk1533_dn7_slot;
        let mut var_h5__blk1533_dn8: f64 = *var_h5__blk1533_dn8_slot;
        let mut var_h5__blk1533_dn9: f64 = *var_h5__blk1533_dn9_slot;
        let mut var_ijungat_d: f64 = *var_ijungat_d_slot;
        let mut var_ijungat_d_db0: f64 = *var_ijungat_d_db0_slot;
        let mut var_ijungat_d_db1: f64 = *var_ijungat_d_db1_slot;
        let mut var_ijungat_d_db2: f64 = *var_ijungat_d_db2_slot;
        let mut var_ijungat_d_db3: f64 = *var_ijungat_d_db3_slot;
        let mut var_ijungat_d_db4: f64 = *var_ijungat_d_db4_slot;
        let mut var_ijungat_d_db5: f64 = *var_ijungat_d_db5_slot;
        let mut var_ijungat_d_db6: f64 = *var_ijungat_d_db6_slot;
        let mut var_ijungat_d_dn0: f64 = *var_ijungat_d_dn0_slot;
        let mut var_ijungat_d_dn1: f64 = *var_ijungat_d_dn1_slot;
        let mut var_ijungat_d_dn10: f64 = *var_ijungat_d_dn10_slot;
        let mut var_ijungat_d_dn11: f64 = *var_ijungat_d_dn11_slot;
        let mut var_ijungat_d_dn2: f64 = *var_ijungat_d_dn2_slot;
        let mut var_ijungat_d_dn3: f64 = *var_ijungat_d_dn3_slot;
        let mut var_ijungat_d_dn4: f64 = *var_ijungat_d_dn4_slot;
        let mut var_ijungat_d_dn5: f64 = *var_ijungat_d_dn5_slot;
        let mut var_ijungat_d_dn6: f64 = *var_ijungat_d_dn6_slot;
        let mut var_ijungat_d_dn7: f64 = *var_ijungat_d_dn7_slot;
        let mut var_ijungat_d_dn8: f64 = *var_ijungat_d_dn8_slot;
        let mut var_ijungat_d_dn9: f64 = *var_ijungat_d_dn9_slot;
        let mut var_nu__blk1570: f64 = *var_nu__blk1570_slot;
        let mut var_nu__blk1570_db0: f64 = *var_nu__blk1570_db0_slot;
        let mut var_nu__blk1570_db1: f64 = *var_nu__blk1570_db1_slot;
        let mut var_nu__blk1570_db2: f64 = *var_nu__blk1570_db2_slot;
        let mut var_nu__blk1570_db3: f64 = *var_nu__blk1570_db3_slot;
        let mut var_nu__blk1570_db4: f64 = *var_nu__blk1570_db4_slot;
        let mut var_nu__blk1570_db5: f64 = *var_nu__blk1570_db5_slot;
        let mut var_nu__blk1570_db6: f64 = *var_nu__blk1570_db6_slot;
        let mut var_nu__blk1570_dn0: f64 = *var_nu__blk1570_dn0_slot;
        let mut var_nu__blk1570_dn1: f64 = *var_nu__blk1570_dn1_slot;
        let mut var_nu__blk1570_dn10: f64 = *var_nu__blk1570_dn10_slot;
        let mut var_nu__blk1570_dn11: f64 = *var_nu__blk1570_dn11_slot;
        let mut var_nu__blk1570_dn2: f64 = *var_nu__blk1570_dn2_slot;
        let mut var_nu__blk1570_dn3: f64 = *var_nu__blk1570_dn3_slot;
        let mut var_nu__blk1570_dn4: f64 = *var_nu__blk1570_dn4_slot;
        let mut var_nu__blk1570_dn5: f64 = *var_nu__blk1570_dn5_slot;
        let mut var_nu__blk1570_dn6: f64 = *var_nu__blk1570_dn6_slot;
        let mut var_nu__blk1570_dn7: f64 = *var_nu__blk1570_dn7_slot;
        let mut var_nu__blk1570_dn8: f64 = *var_nu__blk1570_dn8_slot;
        let mut var_nu__blk1570_dn9: f64 = *var_nu__blk1570_dn9_slot;
        let mut var_qjungat_d: f64 = *var_qjungat_d_slot;
        let mut var_qjungat_d_db0: f64 = *var_qjungat_d_db0_slot;
        let mut var_qjungat_d_db1: f64 = *var_qjungat_d_db1_slot;
        let mut var_qjungat_d_db2: f64 = *var_qjungat_d_db2_slot;
        let mut var_qjungat_d_db3: f64 = *var_qjungat_d_db3_slot;
        let mut var_qjungat_d_db4: f64 = *var_qjungat_d_db4_slot;
        let mut var_qjungat_d_db5: f64 = *var_qjungat_d_db5_slot;
        let mut var_qjungat_d_db6: f64 = *var_qjungat_d_db6_slot;
        let mut var_qjungat_d_dn0: f64 = *var_qjungat_d_dn0_slot;
        let mut var_qjungat_d_dn1: f64 = *var_qjungat_d_dn1_slot;
        let mut var_qjungat_d_dn10: f64 = *var_qjungat_d_dn10_slot;
        let mut var_qjungat_d_dn11: f64 = *var_qjungat_d_dn11_slot;
        let mut var_qjungat_d_dn2: f64 = *var_qjungat_d_dn2_slot;
        let mut var_qjungat_d_dn3: f64 = *var_qjungat_d_dn3_slot;
        let mut var_qjungat_d_dn4: f64 = *var_qjungat_d_dn4_slot;
        let mut var_qjungat_d_dn5: f64 = *var_qjungat_d_dn5_slot;
        let mut var_qjungat_d_dn6: f64 = *var_qjungat_d_dn6_slot;
        let mut var_qjungat_d_dn7: f64 = *var_qjungat_d_dn7_slot;
        let mut var_qjungat_d_dn8: f64 = *var_qjungat_d_dn8_slot;
        let mut var_qjungat_d_dn9: f64 = *var_qjungat_d_dn9_slot;
        let mut var_tmp__blk1543: f64 = *var_tmp__blk1543_slot;
        let mut var_tmp__blk1543_db0: f64 = *var_tmp__blk1543_db0_slot;
        let mut var_tmp__blk1543_db1: f64 = *var_tmp__blk1543_db1_slot;
        let mut var_tmp__blk1543_db2: f64 = *var_tmp__blk1543_db2_slot;
        let mut var_tmp__blk1543_db3: f64 = *var_tmp__blk1543_db3_slot;
        let mut var_tmp__blk1543_db4: f64 = *var_tmp__blk1543_db4_slot;
        let mut var_tmp__blk1543_db5: f64 = *var_tmp__blk1543_db5_slot;
        let mut var_tmp__blk1543_db6: f64 = *var_tmp__blk1543_db6_slot;
        let mut var_tmp__blk1543_dn0: f64 = *var_tmp__blk1543_dn0_slot;
        let mut var_tmp__blk1543_dn1: f64 = *var_tmp__blk1543_dn1_slot;
        let mut var_tmp__blk1543_dn10: f64 = *var_tmp__blk1543_dn10_slot;
        let mut var_tmp__blk1543_dn11: f64 = *var_tmp__blk1543_dn11_slot;
        let mut var_tmp__blk1543_dn2: f64 = *var_tmp__blk1543_dn2_slot;
        let mut var_tmp__blk1543_dn3: f64 = *var_tmp__blk1543_dn3_slot;
        let mut var_tmp__blk1543_dn4: f64 = *var_tmp__blk1543_dn4_slot;
        let mut var_tmp__blk1543_dn5: f64 = *var_tmp__blk1543_dn5_slot;
        let mut var_tmp__blk1543_dn6: f64 = *var_tmp__blk1543_dn6_slot;
        let mut var_tmp__blk1543_dn7: f64 = *var_tmp__blk1543_dn7_slot;
        let mut var_tmp__blk1543_dn8: f64 = *var_tmp__blk1543_dn8_slot;
        let mut var_tmp__blk1543_dn9: f64 = *var_tmp__blk1543_dn9_slot;
        let mut var_vjtmp: f64 = *var_vjtmp_slot;
        let mut var_vjtmp_db0: f64 = *var_vjtmp_db0_slot;
        let mut var_vjtmp_db1: f64 = *var_vjtmp_db1_slot;
        let mut var_vjtmp_db2: f64 = *var_vjtmp_db2_slot;
        let mut var_vjtmp_db3: f64 = *var_vjtmp_db3_slot;
        let mut var_vjtmp_db4: f64 = *var_vjtmp_db4_slot;
        let mut var_vjtmp_db5: f64 = *var_vjtmp_db5_slot;
        let mut var_vjtmp_db6: f64 = *var_vjtmp_db6_slot;
        let mut var_vjtmp_dn0: f64 = *var_vjtmp_dn0_slot;
        let mut var_vjtmp_dn1: f64 = *var_vjtmp_dn1_slot;
        let mut var_vjtmp_dn10: f64 = *var_vjtmp_dn10_slot;
        let mut var_vjtmp_dn11: f64 = *var_vjtmp_dn11_slot;
        let mut var_vjtmp_dn2: f64 = *var_vjtmp_dn2_slot;
        let mut var_vjtmp_dn3: f64 = *var_vjtmp_dn3_slot;
        let mut var_vjtmp_dn4: f64 = *var_vjtmp_dn4_slot;
        let mut var_vjtmp_dn5: f64 = *var_vjtmp_dn5_slot;
        let mut var_vjtmp_dn6: f64 = *var_vjtmp_dn6_slot;
        let mut var_vjtmp_dn7: f64 = *var_vjtmp_dn7_slot;
        let mut var_vjtmp_dn8: f64 = *var_vjtmp_dn8_slot;
        let mut var_vjtmp_dn9: f64 = *var_vjtmp_dn9_slot;

        let (assign61460_e79863, assign61460_e79863_d_n0, assign61460_e79863_d_n1, assign61460_e79863_d_n2, assign61460_e79863_d_n3, assign61460_e79863_d_n4, assign61460_e79863_d_n5, assign61460_e79863_d_n6, assign61460_e79863_d_n7, assign61460_e79863_d_n8, assign61460_e79863_d_n9, assign61460_e79863_d_n10, assign61460_e79863_d_n11, assign61460_e79863_d_b0, assign61460_e79863_d_b1, assign61460_e79863_d_b2, assign61460_e79863_d_b3, assign61460_e79863_d_b4, assign61460_e79863_d_b5, assign61460_e79863_d_b6,) = {
    if ((((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1711 == 0.0)) && (var_guard1712 != 0.0)) && (var_guard1713 != 0.0)) {
        let assign61460_e79849: f64 = (var_vav__blk1542 * var_vbrinvgat_d);
        let assign61460_e79852: f64 = (var_vav__blk1542 * var_vbrinvgat_d);
        let assign61460_e79853: f64 = (assign61460_e79849 * assign61460_e79852);
        let assign61460_e79856: f64 = (var_vav__blk1542 * var_vbrinvgat_d);
        let assign61460_e79857: f64 = (assign61460_e79853 * assign61460_e79856);
        let assign61460_e79860: f64 = (var_vav__blk1542 * var_vbrinvgat_d);
        let assign61460_e79861: f64 = (assign61460_e79857 * assign61460_e79860);
        (assign61460_e79861, ((((((((var_vav__blk1542_dn0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn0)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_dn0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn0)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_dn0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn0)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_dn0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn0)))), ((((((((var_vav__blk1542_dn1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn1)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_dn1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn1)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_dn1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn1)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_dn1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn1)))), ((((((((var_vav__blk1542_dn2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn2)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_dn2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn2)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_dn2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn2)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_dn2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn2)))), ((((((((var_vav__blk1542_dn3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn3)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_dn3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn3)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_dn3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn3)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_dn3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn3)))), ((((((((var_vav__blk1542_dn4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn4)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_dn4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn4)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_dn4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn4)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_dn4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn4)))), ((((((((var_vav__blk1542_dn5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn5)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_dn5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn5)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_dn5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn5)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_dn5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn5)))), ((((((((var_vav__blk1542_dn6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn6)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_dn6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn6)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_dn6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn6)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_dn6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn6)))), ((((((((var_vav__blk1542_dn7 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn7)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_dn7 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn7)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_dn7 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn7)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_dn7 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn7)))), ((((((((var_vav__blk1542_dn8 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn8)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_dn8 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn8)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_dn8 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn8)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_dn8 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn8)))), ((((((((var_vav__blk1542_dn9 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn9)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_dn9 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn9)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_dn9 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn9)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_dn9 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn9)))), ((((((((var_vav__blk1542_dn10 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn10)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_dn10 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn10)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_dn10 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn10)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_dn10 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn10)))), ((((((((var_vav__blk1542_dn11 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn11)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_dn11 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn11)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_dn11 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn11)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_dn11 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn11)))), ((((((((var_vav__blk1542_db0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db0)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_db0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db0)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_db0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db0)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_db0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db0)))), ((((((((var_vav__blk1542_db1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db1)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_db1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db1)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_db1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db1)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_db1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db1)))), ((((((((var_vav__blk1542_db2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db2)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_db2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db2)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_db2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db2)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_db2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db2)))), ((((((((var_vav__blk1542_db3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db3)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_db3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db3)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_db3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db3)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_db3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db3)))), ((((((((var_vav__blk1542_db4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db4)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_db4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db4)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_db4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db4)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_db4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db4)))), ((((((((var_vav__blk1542_db5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db5)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_db5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db5)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_db5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db5)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_db5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db5)))), ((((((((var_vav__blk1542_db6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db6)) * assign61460_e79852) + (assign61460_e79849 * ((var_vav__blk1542_db6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db6)))) * assign61460_e79856) + (assign61460_e79853 * ((var_vav__blk1542_db6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db6)))) * assign61460_e79860) + (assign61460_e79857 * ((var_vav__blk1542_db6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db6)))),)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61460_e79863;
        var_tmp__blk1543_dn0 = assign61460_e79863_d_n0;
        var_tmp__blk1543_dn1 = assign61460_e79863_d_n1;
        var_tmp__blk1543_dn2 = assign61460_e79863_d_n2;
        var_tmp__blk1543_dn3 = assign61460_e79863_d_n3;
        var_tmp__blk1543_dn4 = assign61460_e79863_d_n4;
        var_tmp__blk1543_dn5 = assign61460_e79863_d_n5;
        var_tmp__blk1543_dn6 = assign61460_e79863_d_n6;
        var_tmp__blk1543_dn7 = assign61460_e79863_d_n7;
        var_tmp__blk1543_dn8 = assign61460_e79863_d_n8;
        var_tmp__blk1543_dn9 = assign61460_e79863_d_n9;
        var_tmp__blk1543_dn10 = assign61460_e79863_d_n10;
        var_tmp__blk1543_dn11 = assign61460_e79863_d_n11;
        var_tmp__blk1543_db0 = assign61460_e79863_d_b0;
        var_tmp__blk1543_db1 = assign61460_e79863_d_b1;
        var_tmp__blk1543_db2 = assign61460_e79863_d_b2;
        var_tmp__blk1543_db3 = assign61460_e79863_d_b3;
        var_tmp__blk1543_db4 = assign61460_e79863_d_b4;
        var_tmp__blk1543_db5 = assign61460_e79863_d_b5;
        var_tmp__blk1543_db6 = assign61460_e79863_d_b6;

        let (assign61470_e79886, assign61470_e79886_d_n0, assign61470_e79886_d_n1, assign61470_e79886_d_n2, assign61470_e79886_d_n3, assign61470_e79886_d_n4, assign61470_e79886_d_n5, assign61470_e79886_d_n6, assign61470_e79886_d_n7, assign61470_e79886_d_n8, assign61470_e79886_d_n9, assign61470_e79886_d_n10, assign61470_e79886_d_n11, assign61470_e79886_d_b0, assign61470_e79886_d_b1, assign61470_e79886_d_b2, assign61470_e79886_d_b3, assign61470_e79886_d_b4, assign61470_e79886_d_b5, assign61470_e79886_d_b6,) = {
    if ((((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1711 == 0.0)) && (var_guard1712 != 0.0)) && (var_guard1713 == 0.0)) {
        let assign61470_e79881: f64 = (var_vav__blk1542 * var_vbrinvgat_d);
        let assign61470_e79882: f64 = (assign61470_e79881).abs();
        let assign61470_e79884: f64 = (assign61470_e79882).powf(var_pbrgatd_i);
        (assign61470_e79884, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn0)) } else { (-((var_vav__blk1542_dn0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn0))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn0)) } else { (-((var_vav__blk1542_dn0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn0))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn1)) } else { (-((var_vav__blk1542_dn1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn1))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn1)) } else { (-((var_vav__blk1542_dn1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn1))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn2)) } else { (-((var_vav__blk1542_dn2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn2))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn2)) } else { (-((var_vav__blk1542_dn2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn2))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn3)) } else { (-((var_vav__blk1542_dn3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn3))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn3)) } else { (-((var_vav__blk1542_dn3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn3))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn4)) } else { (-((var_vav__blk1542_dn4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn4))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn4)) } else { (-((var_vav__blk1542_dn4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn4))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn5)) } else { (-((var_vav__blk1542_dn5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn5))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn5)) } else { (-((var_vav__blk1542_dn5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn5))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn6)) } else { (-((var_vav__blk1542_dn6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn6))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn6)) } else { (-((var_vav__blk1542_dn6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn6))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn7 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn7)) } else { (-((var_vav__blk1542_dn7 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn7))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn7 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn7)) } else { (-((var_vav__blk1542_dn7 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn7))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn8 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn8)) } else { (-((var_vav__blk1542_dn8 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn8))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn8 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn8)) } else { (-((var_vav__blk1542_dn8 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn8))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn9 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn9)) } else { (-((var_vav__blk1542_dn9 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn9))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn9 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn9)) } else { (-((var_vav__blk1542_dn9 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn9))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn10 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn10)) } else { (-((var_vav__blk1542_dn10 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn10))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn10 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn10)) } else { (-((var_vav__blk1542_dn10 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn10))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn11 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn11)) } else { (-((var_vav__blk1542_dn11 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn11))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_dn11 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn11)) } else { (-((var_vav__blk1542_dn11 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_dn11))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_db0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db0)) } else { (-((var_vav__blk1542_db0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db0))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_db0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db0)) } else { (-((var_vav__blk1542_db0 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db0))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_db1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db1)) } else { (-((var_vav__blk1542_db1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db1))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_db1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db1)) } else { (-((var_vav__blk1542_db1 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db1))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_db2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db2)) } else { (-((var_vav__blk1542_db2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db2))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_db2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db2)) } else { (-((var_vav__blk1542_db2 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db2))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_db3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db3)) } else { (-((var_vav__blk1542_db3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db3))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_db3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db3)) } else { (-((var_vav__blk1542_db3 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db3))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_db4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db4)) } else { (-((var_vav__blk1542_db4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db4))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_db4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db4)) } else { (-((var_vav__blk1542_db4 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db4))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_db5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db5)) } else { (-((var_vav__blk1542_db5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db5))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_db5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db5)) } else { (-((var_vav__blk1542_db5 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db5))) } / assign61470_e79882))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61470_e79882).powf(var_pbrgatd_i - 1.0) * if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_db6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db6)) } else { (-((var_vav__blk1542_db6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db6))) })) } } else { (assign61470_e79884 * (var_pbrgatd_i * (if assign61470_e79881 >= 0.0 { ((var_vav__blk1542_db6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db6)) } else { (-((var_vav__blk1542_db6 * var_vbrinvgat_d) + (var_vav__blk1542 * var_vbrinvgat_d_db6))) } / assign61470_e79882))) },)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61470_e79886;
        var_tmp__blk1543_dn0 = assign61470_e79886_d_n0;
        var_tmp__blk1543_dn1 = assign61470_e79886_d_n1;
        var_tmp__blk1543_dn2 = assign61470_e79886_d_n2;
        var_tmp__blk1543_dn3 = assign61470_e79886_d_n3;
        var_tmp__blk1543_dn4 = assign61470_e79886_d_n4;
        var_tmp__blk1543_dn5 = assign61470_e79886_d_n5;
        var_tmp__blk1543_dn6 = assign61470_e79886_d_n6;
        var_tmp__blk1543_dn7 = assign61470_e79886_d_n7;
        var_tmp__blk1543_dn8 = assign61470_e79886_d_n8;
        var_tmp__blk1543_dn9 = assign61470_e79886_d_n9;
        var_tmp__blk1543_dn10 = assign61470_e79886_d_n10;
        var_tmp__blk1543_dn11 = assign61470_e79886_d_n11;
        var_tmp__blk1543_db0 = assign61470_e79886_d_b0;
        var_tmp__blk1543_db1 = assign61470_e79886_d_b1;
        var_tmp__blk1543_db2 = assign61470_e79886_d_b2;
        var_tmp__blk1543_db3 = assign61470_e79886_d_b3;
        var_tmp__blk1543_db4 = assign61470_e79886_d_b4;
        var_tmp__blk1543_db5 = assign61470_e79886_d_b5;
        var_tmp__blk1543_db6 = assign61470_e79886_d_b6;

        let (assign61480_e79905, assign61480_e79905_d_n0, assign61480_e79905_d_n1, assign61480_e79905_d_n2, assign61480_e79905_d_n3, assign61480_e79905_d_n4, assign61480_e79905_d_n5, assign61480_e79905_d_n6, assign61480_e79905_d_n7, assign61480_e79905_d_n8, assign61480_e79905_d_n9, assign61480_e79905_d_n10, assign61480_e79905_d_n11, assign61480_e79905_d_b0, assign61480_e79905_d_b1, assign61480_e79905_d_b2, assign61480_e79905_d_b3, assign61480_e79905_d_b4, assign61480_e79905_d_b5, assign61480_e79905_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1711 == 0.0)) && (var_guard1712 != 0.0)) {
        let assign61480_e79902: f64 = (1.0 - var_tmp__blk1543);
        let assign61480_e79903: f64 = (1.0 / assign61480_e79902);
        (assign61480_e79903, (-((-var_tmp__blk1543_dn0) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_dn1) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_dn2) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_dn3) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_dn4) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_dn5) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_dn6) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_dn7) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_dn8) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_dn9) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_dn10) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_dn11) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_db0) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_db1) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_db2) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_db3) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_db4) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_db5) / (assign61480_e79902 * assign61480_e79902))), (-((-var_tmp__blk1543_db6) / (assign61480_e79902 * assign61480_e79902))),)
    } else {
        (var_fbreakdown__blk1569, var_fbreakdown__blk1569_dn0, var_fbreakdown__blk1569_dn1, var_fbreakdown__blk1569_dn2, var_fbreakdown__blk1569_dn3, var_fbreakdown__blk1569_dn4, var_fbreakdown__blk1569_dn5, var_fbreakdown__blk1569_dn6, var_fbreakdown__blk1569_dn7, var_fbreakdown__blk1569_dn8, var_fbreakdown__blk1569_dn9, var_fbreakdown__blk1569_dn10, var_fbreakdown__blk1569_dn11, var_fbreakdown__blk1569_db0, var_fbreakdown__blk1569_db1, var_fbreakdown__blk1569_db2, var_fbreakdown__blk1569_db3, var_fbreakdown__blk1569_db4, var_fbreakdown__blk1569_db5, var_fbreakdown__blk1569_db6,)
    }
};
        var_fbreakdown__blk1569 = assign61480_e79905;
        var_fbreakdown__blk1569_dn0 = assign61480_e79905_d_n0;
        var_fbreakdown__blk1569_dn1 = assign61480_e79905_d_n1;
        var_fbreakdown__blk1569_dn2 = assign61480_e79905_d_n2;
        var_fbreakdown__blk1569_dn3 = assign61480_e79905_d_n3;
        var_fbreakdown__blk1569_dn4 = assign61480_e79905_d_n4;
        var_fbreakdown__blk1569_dn5 = assign61480_e79905_d_n5;
        var_fbreakdown__blk1569_dn6 = assign61480_e79905_d_n6;
        var_fbreakdown__blk1569_dn7 = assign61480_e79905_d_n7;
        var_fbreakdown__blk1569_dn8 = assign61480_e79905_d_n8;
        var_fbreakdown__blk1569_dn9 = assign61480_e79905_d_n9;
        var_fbreakdown__blk1569_dn10 = assign61480_e79905_d_n10;
        var_fbreakdown__blk1569_dn11 = assign61480_e79905_d_n11;
        var_fbreakdown__blk1569_db0 = assign61480_e79905_d_b0;
        var_fbreakdown__blk1569_db1 = assign61480_e79905_d_b1;
        var_fbreakdown__blk1569_db2 = assign61480_e79905_d_b2;
        var_fbreakdown__blk1569_db3 = assign61480_e79905_d_b3;
        var_fbreakdown__blk1569_db4 = assign61480_e79905_d_b4;
        var_fbreakdown__blk1569_db5 = assign61480_e79905_d_b5;
        var_fbreakdown__blk1569_db6 = assign61480_e79905_d_b6;

        let (assign61490_e79929, assign61490_e79929_d_n0, assign61490_e79929_d_n1, assign61490_e79929_d_n2, assign61490_e79929_d_n3, assign61490_e79929_d_n4, assign61490_e79929_d_n5, assign61490_e79929_d_n6, assign61490_e79929_d_n7, assign61490_e79929_d_n8, assign61490_e79929_d_n9, assign61490_e79929_d_n10, assign61490_e79929_d_n11, assign61490_e79929_d_b0, assign61490_e79929_d_b1, assign61490_e79929_d_b2, assign61490_e79929_d_b3, assign61490_e79929_d_b4, assign61490_e79929_d_b5, assign61490_e79929_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1711 == 0.0)) && (var_guard1712 == 0.0)) {
        let assign61490_e79923: f64 = (var_alphaav * var_vbrgat_var_d);
        let assign61490_e79924: f64 = (var_vav__blk1542 + assign61490_e79923);
        let assign61490_e79926: f64 = (assign61490_e79924 * var_slopegat_d);
        let assign61490_e79927: f64 = (var_fstopgat_d + assign61490_e79926);
        (assign61490_e79927, (((var_vav__blk1542_dn0 + (var_alphaav * var_vbrgat_var_d_dn0)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_dn0)), (((var_vav__blk1542_dn1 + (var_alphaav * var_vbrgat_var_d_dn1)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_dn1)), (((var_vav__blk1542_dn2 + (var_alphaav * var_vbrgat_var_d_dn2)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_dn2)), (((var_vav__blk1542_dn3 + (var_alphaav * var_vbrgat_var_d_dn3)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_dn3)), (((var_vav__blk1542_dn4 + (var_alphaav * var_vbrgat_var_d_dn4)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_dn4)), (((var_vav__blk1542_dn5 + (var_alphaav * var_vbrgat_var_d_dn5)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_dn5)), (((var_vav__blk1542_dn6 + (var_alphaav * var_vbrgat_var_d_dn6)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_dn6)), (((var_vav__blk1542_dn7 + (var_alphaav * var_vbrgat_var_d_dn7)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_dn7)), (((var_vav__blk1542_dn8 + (var_alphaav * var_vbrgat_var_d_dn8)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_dn8)), (((var_vav__blk1542_dn9 + (var_alphaav * var_vbrgat_var_d_dn9)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_dn9)), (((var_vav__blk1542_dn10 + (var_alphaav * var_vbrgat_var_d_dn10)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_dn10)), (((var_vav__blk1542_dn11 + (var_alphaav * var_vbrgat_var_d_dn11)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_dn11)), (((var_vav__blk1542_db0 + (var_alphaav * var_vbrgat_var_d_db0)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_db0)), (((var_vav__blk1542_db1 + (var_alphaav * var_vbrgat_var_d_db1)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_db1)), (((var_vav__blk1542_db2 + (var_alphaav * var_vbrgat_var_d_db2)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_db2)), (((var_vav__blk1542_db3 + (var_alphaav * var_vbrgat_var_d_db3)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_db3)), (((var_vav__blk1542_db4 + (var_alphaav * var_vbrgat_var_d_db4)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_db4)), (((var_vav__blk1542_db5 + (var_alphaav * var_vbrgat_var_d_db5)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_db5)), (((var_vav__blk1542_db6 + (var_alphaav * var_vbrgat_var_d_db6)) * var_slopegat_d) + (assign61490_e79924 * var_slopegat_d_db6)),)
    } else {
        (var_fbreakdown__blk1569, var_fbreakdown__blk1569_dn0, var_fbreakdown__blk1569_dn1, var_fbreakdown__blk1569_dn2, var_fbreakdown__blk1569_dn3, var_fbreakdown__blk1569_dn4, var_fbreakdown__blk1569_dn5, var_fbreakdown__blk1569_dn6, var_fbreakdown__blk1569_dn7, var_fbreakdown__blk1569_dn8, var_fbreakdown__blk1569_dn9, var_fbreakdown__blk1569_dn10, var_fbreakdown__blk1569_dn11, var_fbreakdown__blk1569_db0, var_fbreakdown__blk1569_db1, var_fbreakdown__blk1569_db2, var_fbreakdown__blk1569_db3, var_fbreakdown__blk1569_db4, var_fbreakdown__blk1569_db5, var_fbreakdown__blk1569_db6,)
    }
};
        var_fbreakdown__blk1569 = assign61490_e79929;
        var_fbreakdown__blk1569_dn0 = assign61490_e79929_d_n0;
        var_fbreakdown__blk1569_dn1 = assign61490_e79929_d_n1;
        var_fbreakdown__blk1569_dn2 = assign61490_e79929_d_n2;
        var_fbreakdown__blk1569_dn3 = assign61490_e79929_d_n3;
        var_fbreakdown__blk1569_dn4 = assign61490_e79929_d_n4;
        var_fbreakdown__blk1569_dn5 = assign61490_e79929_d_n5;
        var_fbreakdown__blk1569_dn6 = assign61490_e79929_d_n6;
        var_fbreakdown__blk1569_dn7 = assign61490_e79929_d_n7;
        var_fbreakdown__blk1569_dn8 = assign61490_e79929_d_n8;
        var_fbreakdown__blk1569_dn9 = assign61490_e79929_d_n9;
        var_fbreakdown__blk1569_dn10 = assign61490_e79929_d_n10;
        var_fbreakdown__blk1569_dn11 = assign61490_e79929_d_n11;
        var_fbreakdown__blk1569_db0 = assign61490_e79929_d_b0;
        var_fbreakdown__blk1569_db1 = assign61490_e79929_d_b1;
        var_fbreakdown__blk1569_db2 = assign61490_e79929_d_b2;
        var_fbreakdown__blk1569_db3 = assign61490_e79929_d_b3;
        var_fbreakdown__blk1569_db4 = assign61490_e79929_d_b4;
        var_fbreakdown__blk1569_db5 = assign61490_e79929_d_b5;
        var_fbreakdown__blk1569_db6 = assign61490_e79929_d_b6;

        let (assign61500_e79949, assign61500_e79949_d_n0, assign61500_e79949_d_n1, assign61500_e79949_d_n2, assign61500_e79949_d_n3, assign61500_e79949_d_n4, assign61500_e79949_d_n5, assign61500_e79949_d_n6, assign61500_e79949_d_n7, assign61500_e79949_d_n8, assign61500_e79949_d_n9, assign61500_e79949_d_n10, assign61500_e79949_d_n11, assign61500_e79949_d_b0, assign61500_e79949_d_b1, assign61500_e79949_d_b2, assign61500_e79949_d_b3, assign61500_e79949_d_b4, assign61500_e79949_d_b5, assign61500_e79949_d_b6,) = {
    if (((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) {
        let assign61500_e79940: f64 = (var_id__blk1544 + var_isrh__blk1545);
        let assign61500_e79942: f64 = (assign61500_e79940 + var_itat__blk1552);
        let assign61500_e79944: f64 = (assign61500_e79942 + var_ibbt__blk1567);
        let assign61500_e79945: f64 = (p.p29 * assign61500_e79944);
        let assign61500_e79947: f64 = (assign61500_e79945 * var_fbreakdown__blk1569);
        (assign61500_e79947, (((p.p29 * (((var_id__blk1544_dn0 + var_isrh__blk1545_dn0) + var_itat__blk1552_dn0) + var_ibbt__blk1567_dn0)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_dn0)), (((p.p29 * (((var_id__blk1544_dn1 + var_isrh__blk1545_dn1) + var_itat__blk1552_dn1) + var_ibbt__blk1567_dn1)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_dn1)), (((p.p29 * (((var_id__blk1544_dn2 + var_isrh__blk1545_dn2) + var_itat__blk1552_dn2) + var_ibbt__blk1567_dn2)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_dn2)), (((p.p29 * (((var_id__blk1544_dn3 + var_isrh__blk1545_dn3) + var_itat__blk1552_dn3) + var_ibbt__blk1567_dn3)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_dn3)), (((p.p29 * (((var_id__blk1544_dn4 + var_isrh__blk1545_dn4) + var_itat__blk1552_dn4) + var_ibbt__blk1567_dn4)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_dn4)), (((p.p29 * (((var_id__blk1544_dn5 + var_isrh__blk1545_dn5) + var_itat__blk1552_dn5) + var_ibbt__blk1567_dn5)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_dn5)), (((p.p29 * (((var_id__blk1544_dn6 + var_isrh__blk1545_dn6) + var_itat__blk1552_dn6) + var_ibbt__blk1567_dn6)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_dn6)), (((p.p29 * (((var_id__blk1544_dn7 + var_isrh__blk1545_dn7) + var_itat__blk1552_dn7) + var_ibbt__blk1567_dn7)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_dn7)), (((p.p29 * (((var_id__blk1544_dn8 + var_isrh__blk1545_dn8) + var_itat__blk1552_dn8) + var_ibbt__blk1567_dn8)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_dn8)), (((p.p29 * (((var_id__blk1544_dn9 + var_isrh__blk1545_dn9) + var_itat__blk1552_dn9) + var_ibbt__blk1567_dn9)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_dn9)), (((p.p29 * (((var_id__blk1544_dn10 + var_isrh__blk1545_dn10) + var_itat__blk1552_dn10) + var_ibbt__blk1567_dn10)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_dn10)), (((p.p29 * (((var_id__blk1544_dn11 + var_isrh__blk1545_dn11) + var_itat__blk1552_dn11) + var_ibbt__blk1567_dn11)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_dn11)), (((p.p29 * (((var_id__blk1544_db0 + var_isrh__blk1545_db0) + var_itat__blk1552_db0) + var_ibbt__blk1567_db0)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_db0)), (((p.p29 * (((var_id__blk1544_db1 + var_isrh__blk1545_db1) + var_itat__blk1552_db1) + var_ibbt__blk1567_db1)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_db1)), (((p.p29 * (((var_id__blk1544_db2 + var_isrh__blk1545_db2) + var_itat__blk1552_db2) + var_ibbt__blk1567_db2)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_db2)), (((p.p29 * (((var_id__blk1544_db3 + var_isrh__blk1545_db3) + var_itat__blk1552_db3) + var_ibbt__blk1567_db3)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_db3)), (((p.p29 * (((var_id__blk1544_db4 + var_isrh__blk1545_db4) + var_itat__blk1552_db4) + var_ibbt__blk1567_db4)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_db4)), (((p.p29 * (((var_id__blk1544_db5 + var_isrh__blk1545_db5) + var_itat__blk1552_db5) + var_ibbt__blk1567_db5)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_db5)), (((p.p29 * (((var_id__blk1544_db6 + var_isrh__blk1545_db6) + var_itat__blk1552_db6) + var_ibbt__blk1567_db6)) * var_fbreakdown__blk1569) + (assign61500_e79945 * var_fbreakdown__blk1569_db6)),)
    } else {
        (var_ijungat_d, var_ijungat_d_dn0, var_ijungat_d_dn1, var_ijungat_d_dn2, var_ijungat_d_dn3, var_ijungat_d_dn4, var_ijungat_d_dn5, var_ijungat_d_dn6, var_ijungat_d_dn7, var_ijungat_d_dn8, var_ijungat_d_dn9, var_ijungat_d_dn10, var_ijungat_d_dn11, var_ijungat_d_db0, var_ijungat_d_db1, var_ijungat_d_db2, var_ijungat_d_db3, var_ijungat_d_db4, var_ijungat_d_db5, var_ijungat_d_db6,)
    }
};
        var_ijungat_d = assign61500_e79949;
        var_ijungat_d_dn0 = assign61500_e79949_d_n0;
        var_ijungat_d_dn1 = assign61500_e79949_d_n1;
        var_ijungat_d_dn2 = assign61500_e79949_d_n2;
        var_ijungat_d_dn3 = assign61500_e79949_d_n3;
        var_ijungat_d_dn4 = assign61500_e79949_d_n4;
        var_ijungat_d_dn5 = assign61500_e79949_d_n5;
        var_ijungat_d_dn6 = assign61500_e79949_d_n6;
        var_ijungat_d_dn7 = assign61500_e79949_d_n7;
        var_ijungat_d_dn8 = assign61500_e79949_d_n8;
        var_ijungat_d_dn9 = assign61500_e79949_d_n9;
        var_ijungat_d_dn10 = assign61500_e79949_d_n10;
        var_ijungat_d_dn11 = assign61500_e79949_d_n11;
        var_ijungat_d_db0 = assign61500_e79949_d_b0;
        var_ijungat_d_db1 = assign61500_e79949_d_b1;
        var_ijungat_d_db2 = assign61500_e79949_d_b2;
        var_ijungat_d_db3 = assign61500_e79949_d_b3;
        var_ijungat_d_db4 = assign61500_e79949_d_b4;
        var_ijungat_d_db5 = assign61500_e79949_d_b5;
        var_ijungat_d_db6 = assign61500_e79949_d_b6;

        let assign61510_e79952: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard1714 = assign61510_e79952;

        let (assign61520_e80012, assign61520_e80012_d_n0, assign61520_e80012_d_n1, assign61520_e80012_d_n2, assign61520_e80012_d_n3, assign61520_e80012_d_n4, assign61520_e80012_d_n5, assign61520_e80012_d_n6, assign61520_e80012_d_n7, assign61520_e80012_d_n8, assign61520_e80012_d_n9, assign61520_e80012_d_n10, assign61520_e80012_d_n11, assign61520_e80012_d_b0, assign61520_e80012_d_b1, assign61520_e80012_d_b2, assign61520_e80012_d_b3, assign61520_e80012_d_b4, assign61520_e80012_d_b5, assign61520_e80012_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let (assign61520_e80010, assign61520_e80010_d_n0, assign61520_e80010_d_n1, assign61520_e80010_d_n2, assign61520_e80010_d_n3, assign61520_e80010_d_n4, assign61520_e80010_d_n5, assign61520_e80010_d_n6, assign61520_e80010_d_n7, assign61520_e80010_d_n8, assign61520_e80010_d_n9, assign61520_e80010_d_n10, assign61520_e80010_d_n11, assign61520_e80010_d_b0, assign61520_e80010_d_b1, assign61520_e80010_d_b2, assign61520_e80010_d_b3, assign61520_e80010_d_b4, assign61520_e80010_d_b5, assign61520_e80010_d_b6,) = {
            if (var_vjun_d < var_vtrgatd_i) {
                let assign61520_e79967: f64 = (var_vjun_d - var_vtrgatd_i);
                let assign61520_e79969: f64 = (assign61520_e79967 / var_anugatd_i);
                let assign61520_e79971: f64 = (-37.0);
                let (assign61520_e79987, assign61520_e79987_d_n0, assign61520_e79987_d_n1, assign61520_e79987_d_n2, assign61520_e79987_d_n3, assign61520_e79987_d_n4, assign61520_e79987_d_n5, assign61520_e79987_d_n6, assign61520_e79987_d_n7, assign61520_e79987_d_n8, assign61520_e79987_d_n9, assign61520_e79987_d_n10, assign61520_e79987_d_n11, assign61520_e79987_d_b0, assign61520_e79987_d_b1, assign61520_e79987_d_b2, assign61520_e79987_d_b3, assign61520_e79987_d_b4, assign61520_e79987_d_b5, assign61520_e79987_d_b6,) = {
                    if (assign61520_e79969 < assign61520_e79971) {
                        (var_vtrgatd_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign61520_e79978: f64 = (var_vjun_d - var_vtrgatd_i);
                        let assign61520_e79980: f64 = (assign61520_e79978 / var_anugatd_i);
                        let assign61520_e79981: f64 = (assign61520_e79980).exp();
                        let assign61520_e79982: f64 = (1.0 + assign61520_e79981);
                        let assign61520_e79983: f64 = (assign61520_e79982).ln();
                        let assign61520_e79985: f64 = (assign61520_e79983 * var_anugatd_i);
                        let assign61520_e79986: f64 = (var_vtrgatd_i + assign61520_e79985);
                        (assign61520_e79986, (((assign61520_e79981 * (var_vjun_d_dn0 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_dn1 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_dn2 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_dn3 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_dn4 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_dn5 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_dn6 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_dn7 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_dn8 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_dn9 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_dn10 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_dn11 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_db0 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_db1 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_db2 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_db3 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_db4 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_db5 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i), (((assign61520_e79981 * (var_vjun_d_db6 / var_anugatd_i)) / assign61520_e79982) * var_anugatd_i),)
                    }
                };
                (assign61520_e79987, assign61520_e79987_d_n0, assign61520_e79987_d_n1, assign61520_e79987_d_n2, assign61520_e79987_d_n3, assign61520_e79987_d_n4, assign61520_e79987_d_n5, assign61520_e79987_d_n6, assign61520_e79987_d_n7, assign61520_e79987_d_n8, assign61520_e79987_d_n9, assign61520_e79987_d_n10, assign61520_e79987_d_n11, assign61520_e79987_d_b0, assign61520_e79987_d_b1, assign61520_e79987_d_b2, assign61520_e79987_d_b3, assign61520_e79987_d_b4, assign61520_e79987_d_b5, assign61520_e79987_d_b6,)
            } else {
                let assign61520_e79990: f64 = (var_vjun_d - var_vtrgatd_i);
                let assign61520_e79992: f64 = (assign61520_e79990 / var_anugatd_i);
                let (assign61520_e80009, assign61520_e80009_d_n0, assign61520_e80009_d_n1, assign61520_e80009_d_n2, assign61520_e80009_d_n3, assign61520_e80009_d_n4, assign61520_e80009_d_n5, assign61520_e80009_d_n6, assign61520_e80009_d_n7, assign61520_e80009_d_n8, assign61520_e80009_d_n9, assign61520_e80009_d_n10, assign61520_e80009_d_n11, assign61520_e80009_d_b0, assign61520_e80009_d_b1, assign61520_e80009_d_b2, assign61520_e80009_d_b3, assign61520_e80009_d_b4, assign61520_e80009_d_b5, assign61520_e80009_d_b6,) = {
                    if (assign61520_e79992 > 37.0) {
                        (var_vjun_d, var_vjun_d_dn0, var_vjun_d_dn1, var_vjun_d_dn2, var_vjun_d_dn3, var_vjun_d_dn4, var_vjun_d_dn5, var_vjun_d_dn6, var_vjun_d_dn7, var_vjun_d_dn8, var_vjun_d_dn9, var_vjun_d_dn10, var_vjun_d_dn11, var_vjun_d_db0, var_vjun_d_db1, var_vjun_d_db2, var_vjun_d_db3, var_vjun_d_db4, var_vjun_d_db5, var_vjun_d_db6,)
                    } else {
                        let assign61520_e80000: f64 = (var_vtrgatd_i - var_vjun_d);
                        let assign61520_e80002: f64 = (assign61520_e80000 / var_anugatd_i);
                        let assign61520_e80003: f64 = (assign61520_e80002).exp();
                        let assign61520_e80004: f64 = (1.0 + assign61520_e80003);
                        let assign61520_e80005: f64 = (assign61520_e80004).ln();
                        let assign61520_e80007: f64 = (assign61520_e80005 * var_anugatd_i);
                        let assign61520_e80008: f64 = (var_vjun_d + assign61520_e80007);
                        (assign61520_e80008, (var_vjun_d_dn0 + (((assign61520_e80003 * ((-var_vjun_d_dn0) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_dn1 + (((assign61520_e80003 * ((-var_vjun_d_dn1) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_dn2 + (((assign61520_e80003 * ((-var_vjun_d_dn2) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_dn3 + (((assign61520_e80003 * ((-var_vjun_d_dn3) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_dn4 + (((assign61520_e80003 * ((-var_vjun_d_dn4) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_dn5 + (((assign61520_e80003 * ((-var_vjun_d_dn5) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_dn6 + (((assign61520_e80003 * ((-var_vjun_d_dn6) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_dn7 + (((assign61520_e80003 * ((-var_vjun_d_dn7) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_dn8 + (((assign61520_e80003 * ((-var_vjun_d_dn8) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_dn9 + (((assign61520_e80003 * ((-var_vjun_d_dn9) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_dn10 + (((assign61520_e80003 * ((-var_vjun_d_dn10) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_dn11 + (((assign61520_e80003 * ((-var_vjun_d_dn11) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_db0 + (((assign61520_e80003 * ((-var_vjun_d_db0) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_db1 + (((assign61520_e80003 * ((-var_vjun_d_db1) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_db2 + (((assign61520_e80003 * ((-var_vjun_d_db2) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_db3 + (((assign61520_e80003 * ((-var_vjun_d_db3) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_db4 + (((assign61520_e80003 * ((-var_vjun_d_db4) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_db5 + (((assign61520_e80003 * ((-var_vjun_d_db5) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)), (var_vjun_d_db6 + (((assign61520_e80003 * ((-var_vjun_d_db6) / var_anugatd_i)) / assign61520_e80004) * var_anugatd_i)),)
                    }
                };
                (assign61520_e80009, assign61520_e80009_d_n0, assign61520_e80009_d_n1, assign61520_e80009_d_n2, assign61520_e80009_d_n3, assign61520_e80009_d_n4, assign61520_e80009_d_n5, assign61520_e80009_d_n6, assign61520_e80009_d_n7, assign61520_e80009_d_n8, assign61520_e80009_d_n9, assign61520_e80009_d_n10, assign61520_e80009_d_n11, assign61520_e80009_d_b0, assign61520_e80009_d_b1, assign61520_e80009_d_b2, assign61520_e80009_d_b3, assign61520_e80009_d_b4, assign61520_e80009_d_b5, assign61520_e80009_d_b6,)
            }
        };
        (assign61520_e80010, assign61520_e80010_d_n0, assign61520_e80010_d_n1, assign61520_e80010_d_n2, assign61520_e80010_d_n3, assign61520_e80010_d_n4, assign61520_e80010_d_n5, assign61520_e80010_d_n6, assign61520_e80010_d_n7, assign61520_e80010_d_n8, assign61520_e80010_d_n9, assign61520_e80010_d_n10, assign61520_e80010_d_n11, assign61520_e80010_d_b0, assign61520_e80010_d_b1, assign61520_e80010_d_b2, assign61520_e80010_d_b3, assign61520_e80010_d_b4, assign61520_e80010_d_b5, assign61520_e80010_d_b6,)
    } else {
        (var_nu__blk1570, var_nu__blk1570_dn0, var_nu__blk1570_dn1, var_nu__blk1570_dn2, var_nu__blk1570_dn3, var_nu__blk1570_dn4, var_nu__blk1570_dn5, var_nu__blk1570_dn6, var_nu__blk1570_dn7, var_nu__blk1570_dn8, var_nu__blk1570_dn9, var_nu__blk1570_dn10, var_nu__blk1570_dn11, var_nu__blk1570_db0, var_nu__blk1570_db1, var_nu__blk1570_db2, var_nu__blk1570_db3, var_nu__blk1570_db4, var_nu__blk1570_db5, var_nu__blk1570_db6,)
    }
};
        var_nu__blk1570 = assign61520_e80012;
        var_nu__blk1570_dn0 = assign61520_e80012_d_n0;
        var_nu__blk1570_dn1 = assign61520_e80012_d_n1;
        var_nu__blk1570_dn2 = assign61520_e80012_d_n2;
        var_nu__blk1570_dn3 = assign61520_e80012_d_n3;
        var_nu__blk1570_dn4 = assign61520_e80012_d_n4;
        var_nu__blk1570_dn5 = assign61520_e80012_d_n5;
        var_nu__blk1570_dn6 = assign61520_e80012_d_n6;
        var_nu__blk1570_dn7 = assign61520_e80012_d_n7;
        var_nu__blk1570_dn8 = assign61520_e80012_d_n8;
        var_nu__blk1570_dn9 = assign61520_e80012_d_n9;
        var_nu__blk1570_dn10 = assign61520_e80012_d_n10;
        var_nu__blk1570_dn11 = assign61520_e80012_d_n11;
        var_nu__blk1570_db0 = assign61520_e80012_d_b0;
        var_nu__blk1570_db1 = assign61520_e80012_d_b1;
        var_nu__blk1570_db2 = assign61520_e80012_d_b2;
        var_nu__blk1570_db3 = assign61520_e80012_d_b3;
        var_nu__blk1570_db4 = assign61520_e80012_d_b4;
        var_nu__blk1570_db5 = assign61520_e80012_d_b5;
        var_nu__blk1570_db6 = assign61520_e80012_d_b6;

        let (assign61530_e80028,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61530_e80024: f64 = (4.0 * var_vch_d);
        let assign61530_e80026: f64 = (assign61530_e80024 * var_vch_d);
        (assign61530_e80026,)
    } else {
        (var_h1__blk1528,)
    }
};
        var_h1__blk1528 = assign61530_e80028;

        let (assign61540_e80042,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61540_e80040: f64 = (var_vch_d / var_vfmin_d);
        (assign61540_e80040,)
    } else {
        (var_h2__blk1529,)
    }
};
        var_h2__blk1529 = assign61540_e80042;

        let (assign61550_e80058, assign61550_e80058_d_n0, assign61550_e80058_d_n1, assign61550_e80058_d_n2, assign61550_e80058_d_n3, assign61550_e80058_d_n4, assign61550_e80058_d_n5, assign61550_e80058_d_n6, assign61550_e80058_d_n7, assign61550_e80058_d_n8, assign61550_e80058_d_n9, assign61550_e80058_d_n10, assign61550_e80058_d_n11, assign61550_e80058_d_b0, assign61550_e80058_d_b1, assign61550_e80058_d_b2, assign61550_e80058_d_b3, assign61550_e80058_d_b4, assign61550_e80058_d_b5, assign61550_e80058_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61550_e80055: f64 = (var_vch_d * var_h2__blk1529);
        let assign61550_e80056: f64 = (var_nu__blk1570 + assign61550_e80055);
        (assign61550_e80056, var_nu__blk1570_dn0, var_nu__blk1570_dn1, var_nu__blk1570_dn2, var_nu__blk1570_dn3, var_nu__blk1570_dn4, var_nu__blk1570_dn5, var_nu__blk1570_dn6, var_nu__blk1570_dn7, var_nu__blk1570_dn8, var_nu__blk1570_dn9, var_nu__blk1570_dn10, var_nu__blk1570_dn11, var_nu__blk1570_db0, var_nu__blk1570_db1, var_nu__blk1570_db2, var_nu__blk1570_db3, var_nu__blk1570_db4, var_nu__blk1570_db5, var_nu__blk1570_db6,)
    } else {
        (var_h2d__blk1530, var_h2d__blk1530_dn0, var_h2d__blk1530_dn1, var_h2d__blk1530_dn2, var_h2d__blk1530_dn3, var_h2d__blk1530_dn4, var_h2d__blk1530_dn5, var_h2d__blk1530_dn6, var_h2d__blk1530_dn7, var_h2d__blk1530_dn8, var_h2d__blk1530_dn9, var_h2d__blk1530_dn10, var_h2d__blk1530_dn11, var_h2d__blk1530_db0, var_h2d__blk1530_db1, var_h2d__blk1530_db2, var_h2d__blk1530_db3, var_h2d__blk1530_db4, var_h2d__blk1530_db5, var_h2d__blk1530_db6,)
    }
};
        var_h2d__blk1530 = assign61550_e80058;
        var_h2d__blk1530_dn0 = assign61550_e80058_d_n0;
        var_h2d__blk1530_dn1 = assign61550_e80058_d_n1;
        var_h2d__blk1530_dn2 = assign61550_e80058_d_n2;
        var_h2d__blk1530_dn3 = assign61550_e80058_d_n3;
        var_h2d__blk1530_dn4 = assign61550_e80058_d_n4;
        var_h2d__blk1530_dn5 = assign61550_e80058_d_n5;
        var_h2d__blk1530_dn6 = assign61550_e80058_d_n6;
        var_h2d__blk1530_dn7 = assign61550_e80058_d_n7;
        var_h2d__blk1530_dn8 = assign61550_e80058_d_n8;
        var_h2d__blk1530_dn9 = assign61550_e80058_d_n9;
        var_h2d__blk1530_dn10 = assign61550_e80058_d_n10;
        var_h2d__blk1530_dn11 = assign61550_e80058_d_n11;
        var_h2d__blk1530_db0 = assign61550_e80058_d_b0;
        var_h2d__blk1530_db1 = assign61550_e80058_d_b1;
        var_h2d__blk1530_db2 = assign61550_e80058_d_b2;
        var_h2d__blk1530_db3 = assign61550_e80058_d_b3;
        var_h2d__blk1530_db4 = assign61550_e80058_d_b4;
        var_h2d__blk1530_db5 = assign61550_e80058_d_b5;
        var_h2d__blk1530_db6 = assign61550_e80058_d_b6;

        let (assign61560_e80072, assign61560_e80072_d_n0, assign61560_e80072_d_n1, assign61560_e80072_d_n2, assign61560_e80072_d_n3, assign61560_e80072_d_n4, assign61560_e80072_d_n5, assign61560_e80072_d_n6, assign61560_e80072_d_n7, assign61560_e80072_d_n8, assign61560_e80072_d_n9, assign61560_e80072_d_n10, assign61560_e80072_d_n11, assign61560_e80072_d_b0, assign61560_e80072_d_b1, assign61560_e80072_d_b2, assign61560_e80072_d_b3, assign61560_e80072_d_b4, assign61560_e80072_d_b5, assign61560_e80072_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61560_e80070: f64 = (var_vfmin_d + var_h2d__blk1530);
        (assign61560_e80070, var_h2d__blk1530_dn0, var_h2d__blk1530_dn1, var_h2d__blk1530_dn2, var_h2d__blk1530_dn3, var_h2d__blk1530_dn4, var_h2d__blk1530_dn5, var_h2d__blk1530_dn6, var_h2d__blk1530_dn7, var_h2d__blk1530_dn8, var_h2d__blk1530_dn9, var_h2d__blk1530_dn10, var_h2d__blk1530_dn11, var_h2d__blk1530_db0, var_h2d__blk1530_db1, var_h2d__blk1530_db2, var_h2d__blk1530_db3, var_h2d__blk1530_db4, var_h2d__blk1530_db5, var_h2d__blk1530_db6,)
    } else {
        (var_h3__blk1531, var_h3__blk1531_dn0, var_h3__blk1531_dn1, var_h3__blk1531_dn2, var_h3__blk1531_dn3, var_h3__blk1531_dn4, var_h3__blk1531_dn5, var_h3__blk1531_dn6, var_h3__blk1531_dn7, var_h3__blk1531_dn8, var_h3__blk1531_dn9, var_h3__blk1531_dn10, var_h3__blk1531_dn11, var_h3__blk1531_db0, var_h3__blk1531_db1, var_h3__blk1531_db2, var_h3__blk1531_db3, var_h3__blk1531_db4, var_h3__blk1531_db5, var_h3__blk1531_db6,)
    }
};
        var_h3__blk1531 = assign61560_e80072;
        var_h3__blk1531_dn0 = assign61560_e80072_d_n0;
        var_h3__blk1531_dn1 = assign61560_e80072_d_n1;
        var_h3__blk1531_dn2 = assign61560_e80072_d_n2;
        var_h3__blk1531_dn3 = assign61560_e80072_d_n3;
        var_h3__blk1531_dn4 = assign61560_e80072_d_n4;
        var_h3__blk1531_dn5 = assign61560_e80072_d_n5;
        var_h3__blk1531_dn6 = assign61560_e80072_d_n6;
        var_h3__blk1531_dn7 = assign61560_e80072_d_n7;
        var_h3__blk1531_dn8 = assign61560_e80072_d_n8;
        var_h3__blk1531_dn9 = assign61560_e80072_d_n9;
        var_h3__blk1531_dn10 = assign61560_e80072_d_n10;
        var_h3__blk1531_dn11 = assign61560_e80072_d_n11;
        var_h3__blk1531_db0 = assign61560_e80072_d_b0;
        var_h3__blk1531_db1 = assign61560_e80072_d_b1;
        var_h3__blk1531_db2 = assign61560_e80072_d_b2;
        var_h3__blk1531_db3 = assign61560_e80072_d_b3;
        var_h3__blk1531_db4 = assign61560_e80072_d_b4;
        var_h3__blk1531_db5 = assign61560_e80072_d_b5;
        var_h3__blk1531_db6 = assign61560_e80072_d_b6;

        let (assign61570_e80086, assign61570_e80086_d_n0, assign61570_e80086_d_n1, assign61570_e80086_d_n2, assign61570_e80086_d_n3, assign61570_e80086_d_n4, assign61570_e80086_d_n5, assign61570_e80086_d_n6, assign61570_e80086_d_n7, assign61570_e80086_d_n8, assign61570_e80086_d_n9, assign61570_e80086_d_n10, assign61570_e80086_d_n11, assign61570_e80086_d_b0, assign61570_e80086_d_b1, assign61570_e80086_d_b2, assign61570_e80086_d_b3, assign61570_e80086_d_b4, assign61570_e80086_d_b5, assign61570_e80086_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61570_e80084: f64 = (var_vfmin_d - var_h2d__blk1530);
        (assign61570_e80084, (-var_h2d__blk1530_dn0), (-var_h2d__blk1530_dn1), (-var_h2d__blk1530_dn2), (-var_h2d__blk1530_dn3), (-var_h2d__blk1530_dn4), (-var_h2d__blk1530_dn5), (-var_h2d__blk1530_dn6), (-var_h2d__blk1530_dn7), (-var_h2d__blk1530_dn8), (-var_h2d__blk1530_dn9), (-var_h2d__blk1530_dn10), (-var_h2d__blk1530_dn11), (-var_h2d__blk1530_db0), (-var_h2d__blk1530_db1), (-var_h2d__blk1530_db2), (-var_h2d__blk1530_db3), (-var_h2d__blk1530_db4), (-var_h2d__blk1530_db5), (-var_h2d__blk1530_db6),)
    } else {
        (var_h4__blk1532, var_h4__blk1532_dn0, var_h4__blk1532_dn1, var_h4__blk1532_dn2, var_h4__blk1532_dn3, var_h4__blk1532_dn4, var_h4__blk1532_dn5, var_h4__blk1532_dn6, var_h4__blk1532_dn7, var_h4__blk1532_dn8, var_h4__blk1532_dn9, var_h4__blk1532_dn10, var_h4__blk1532_dn11, var_h4__blk1532_db0, var_h4__blk1532_db1, var_h4__blk1532_db2, var_h4__blk1532_db3, var_h4__blk1532_db4, var_h4__blk1532_db5, var_h4__blk1532_db6,)
    }
};
        var_h4__blk1532 = assign61570_e80086;
        var_h4__blk1532_dn0 = assign61570_e80086_d_n0;
        var_h4__blk1532_dn1 = assign61570_e80086_d_n1;
        var_h4__blk1532_dn2 = assign61570_e80086_d_n2;
        var_h4__blk1532_dn3 = assign61570_e80086_d_n3;
        var_h4__blk1532_dn4 = assign61570_e80086_d_n4;
        var_h4__blk1532_dn5 = assign61570_e80086_d_n5;
        var_h4__blk1532_dn6 = assign61570_e80086_d_n6;
        var_h4__blk1532_dn7 = assign61570_e80086_d_n7;
        var_h4__blk1532_dn8 = assign61570_e80086_d_n8;
        var_h4__blk1532_dn9 = assign61570_e80086_d_n9;
        var_h4__blk1532_dn10 = assign61570_e80086_d_n10;
        var_h4__blk1532_dn11 = assign61570_e80086_d_n11;
        var_h4__blk1532_db0 = assign61570_e80086_d_b0;
        var_h4__blk1532_db1 = assign61570_e80086_d_b1;
        var_h4__blk1532_db2 = assign61570_e80086_d_b2;
        var_h4__blk1532_db3 = assign61570_e80086_d_b3;
        var_h4__blk1532_db4 = assign61570_e80086_d_b4;
        var_h4__blk1532_db5 = assign61570_e80086_d_b5;
        var_h4__blk1532_db6 = assign61570_e80086_d_b6;

        let (assign61580_e80103, assign61580_e80103_d_n0, assign61580_e80103_d_n1, assign61580_e80103_d_n2, assign61580_e80103_d_n3, assign61580_e80103_d_n4, assign61580_e80103_d_n5, assign61580_e80103_d_n6, assign61580_e80103_d_n7, assign61580_e80103_d_n8, assign61580_e80103_d_n9, assign61580_e80103_d_n10, assign61580_e80103_d_n11, assign61580_e80103_d_b0, assign61580_e80103_d_b1, assign61580_e80103_d_b2, assign61580_e80103_d_b3, assign61580_e80103_d_b4, assign61580_e80103_d_b5, assign61580_e80103_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61580_e80098: f64 = (var_h4__blk1532 * var_h4__blk1532);
        let assign61580_e80100: f64 = (assign61580_e80098 + var_h1__blk1528);
        let assign61580_e80101: f64 = (assign61580_e80100).sqrt();
        (assign61580_e80101, (((var_h4__blk1532_dn0 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn0)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_dn1 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn1)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_dn2 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn2)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_dn3 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn3)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_dn4 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn4)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_dn5 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn5)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_dn6 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn6)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_dn7 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn7)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_dn8 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn8)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_dn9 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn9)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_dn10 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn10)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_dn11 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn11)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_db0 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_db0)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_db1 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_db1)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_db2 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_db2)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_db3 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_db3)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_db4 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_db4)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_db5 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_db5)) / (2.0 * assign61580_e80101)), (((var_h4__blk1532_db6 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_db6)) / (2.0 * assign61580_e80101)),)
    } else {
        (var_h5__blk1533, var_h5__blk1533_dn0, var_h5__blk1533_dn1, var_h5__blk1533_dn2, var_h5__blk1533_dn3, var_h5__blk1533_dn4, var_h5__blk1533_dn5, var_h5__blk1533_dn6, var_h5__blk1533_dn7, var_h5__blk1533_dn8, var_h5__blk1533_dn9, var_h5__blk1533_dn10, var_h5__blk1533_dn11, var_h5__blk1533_db0, var_h5__blk1533_db1, var_h5__blk1533_db2, var_h5__blk1533_db3, var_h5__blk1533_db4, var_h5__blk1533_db5, var_h5__blk1533_db6,)
    }
};
        var_h5__blk1533 = assign61580_e80103;
        var_h5__blk1533_dn0 = assign61580_e80103_d_n0;
        var_h5__blk1533_dn1 = assign61580_e80103_d_n1;
        var_h5__blk1533_dn2 = assign61580_e80103_d_n2;
        var_h5__blk1533_dn3 = assign61580_e80103_d_n3;
        var_h5__blk1533_dn4 = assign61580_e80103_d_n4;
        var_h5__blk1533_dn5 = assign61580_e80103_d_n5;
        var_h5__blk1533_dn6 = assign61580_e80103_d_n6;
        var_h5__blk1533_dn7 = assign61580_e80103_d_n7;
        var_h5__blk1533_dn8 = assign61580_e80103_d_n8;
        var_h5__blk1533_dn9 = assign61580_e80103_d_n9;
        var_h5__blk1533_dn10 = assign61580_e80103_d_n10;
        var_h5__blk1533_dn11 = assign61580_e80103_d_n11;
        var_h5__blk1533_db0 = assign61580_e80103_d_b0;
        var_h5__blk1533_db1 = assign61580_e80103_d_b1;
        var_h5__blk1533_db2 = assign61580_e80103_d_b2;
        var_h5__blk1533_db3 = assign61580_e80103_d_b3;
        var_h5__blk1533_db4 = assign61580_e80103_d_b4;
        var_h5__blk1533_db5 = assign61580_e80103_d_b5;
        var_h5__blk1533_db6 = assign61580_e80103_d_b6;

        let (assign61590_e80123, assign61590_e80123_d_n0, assign61590_e80123_d_n1, assign61590_e80123_d_n2, assign61590_e80123_d_n3, assign61590_e80123_d_n4, assign61590_e80123_d_n5, assign61590_e80123_d_n6, assign61590_e80123_d_n7, assign61590_e80123_d_n8, assign61590_e80123_d_n9, assign61590_e80123_d_n10, assign61590_e80123_d_n11, assign61590_e80123_d_b0, assign61590_e80123_d_b1, assign61590_e80123_d_b2, assign61590_e80123_d_b3, assign61590_e80123_d_b4, assign61590_e80123_d_b5, assign61590_e80123_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61590_e80116: f64 = (var_nu__blk1570 * var_vfmin_d);
        let assign61590_e80119: f64 = (var_h3__blk1531 + var_h5__blk1533);
        let assign61590_e80120: f64 = (assign61590_e80116 / assign61590_e80119);
        let assign61590_e80121: f64 = (2.0 * assign61590_e80120);
        (assign61590_e80121, (2.0 * ((((var_nu__blk1570_dn0 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_dn0 + var_h5__blk1533_dn0))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_dn1 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_dn1 + var_h5__blk1533_dn1))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_dn2 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_dn2 + var_h5__blk1533_dn2))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_dn3 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_dn3 + var_h5__blk1533_dn3))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_dn4 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_dn4 + var_h5__blk1533_dn4))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_dn5 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_dn5 + var_h5__blk1533_dn5))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_dn6 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_dn6 + var_h5__blk1533_dn6))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_dn7 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_dn7 + var_h5__blk1533_dn7))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_dn8 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_dn8 + var_h5__blk1533_dn8))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_dn9 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_dn9 + var_h5__blk1533_dn9))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_dn10 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_dn10 + var_h5__blk1533_dn10))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_dn11 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_dn11 + var_h5__blk1533_dn11))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_db0 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_db0 + var_h5__blk1533_db0))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_db1 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_db1 + var_h5__blk1533_db1))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_db2 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_db2 + var_h5__blk1533_db2))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_db3 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_db3 + var_h5__blk1533_db3))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_db4 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_db4 + var_h5__blk1533_db4))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_db5 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_db5 + var_h5__blk1533_db5))) / (assign61590_e80119 * assign61590_e80119))), (2.0 * ((((var_nu__blk1570_db6 * var_vfmin_d) * assign61590_e80119) - (assign61590_e80116 * (var_h3__blk1531_db6 + var_h5__blk1533_db6))) / (assign61590_e80119 * assign61590_e80119))),)
    } else {
        (var_vjtmp, var_vjtmp_dn0, var_vjtmp_dn1, var_vjtmp_dn2, var_vjtmp_dn3, var_vjtmp_dn4, var_vjtmp_dn5, var_vjtmp_dn6, var_vjtmp_dn7, var_vjtmp_dn8, var_vjtmp_dn9, var_vjtmp_dn10, var_vjtmp_dn11, var_vjtmp_db0, var_vjtmp_db1, var_vjtmp_db2, var_vjtmp_db3, var_vjtmp_db4, var_vjtmp_db5, var_vjtmp_db6,)
    }
};
        var_vjtmp = assign61590_e80123;
        var_vjtmp_dn0 = assign61590_e80123_d_n0;
        var_vjtmp_dn1 = assign61590_e80123_d_n1;
        var_vjtmp_dn2 = assign61590_e80123_d_n2;
        var_vjtmp_dn3 = assign61590_e80123_d_n3;
        var_vjtmp_dn4 = assign61590_e80123_d_n4;
        var_vjtmp_dn5 = assign61590_e80123_d_n5;
        var_vjtmp_dn6 = assign61590_e80123_d_n6;
        var_vjtmp_dn7 = assign61590_e80123_d_n7;
        var_vjtmp_dn8 = assign61590_e80123_d_n8;
        var_vjtmp_dn9 = assign61590_e80123_d_n9;
        var_vjtmp_dn10 = assign61590_e80123_d_n10;
        var_vjtmp_dn11 = assign61590_e80123_d_n11;
        var_vjtmp_db0 = assign61590_e80123_d_b0;
        var_vjtmp_db1 = assign61590_e80123_d_b1;
        var_vjtmp_db2 = assign61590_e80123_d_b2;
        var_vjtmp_db3 = assign61590_e80123_d_b3;
        var_vjtmp_db4 = assign61590_e80123_d_b4;
        var_vjtmp_db5 = assign61590_e80123_d_b5;
        var_vjtmp_db6 = assign61590_e80123_d_b6;

        let assign61600_e80126: f64 = if var_one_minus_pgat_d == 0.5 { 1.0 } else { 0.0 };
        var_guard1715 = assign61600_e80126;

        let (assign61610_e80145, assign61610_e80145_d_n0, assign61610_e80145_d_n1, assign61610_e80145_d_n2, assign61610_e80145_d_n3, assign61610_e80145_d_n4, assign61610_e80145_d_n5, assign61610_e80145_d_n6, assign61610_e80145_d_n7, assign61610_e80145_d_n8, assign61610_e80145_d_n9, assign61610_e80145_d_n10, assign61610_e80145_d_n11, assign61610_e80145_d_b0, assign61610_e80145_d_b1, assign61610_e80145_d_b2, assign61610_e80145_d_b3, assign61610_e80145_d_b4, assign61610_e80145_d_b5, assign61610_e80145_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) && (var_guard1715 != 0.0)) {
        let assign61610_e80141: f64 = (var_vjtmp * var_vbiinvgat_d);
        let assign61610_e80142: f64 = (1.0 - assign61610_e80141);
        let assign61610_e80143: f64 = (assign61610_e80142).sqrt();
        (assign61610_e80143, ((-(var_vjtmp_dn0 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_dn1 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_dn2 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_dn3 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_dn4 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_dn5 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_dn6 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_dn7 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_dn8 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_dn9 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_dn10 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_dn11 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_db0 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_db1 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_db2 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_db3 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_db4 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_db5 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)), ((-(var_vjtmp_db6 * var_vbiinvgat_d)) / (2.0 * assign61610_e80143)),)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61610_e80145;
        var_tmp__blk1543_dn0 = assign61610_e80145_d_n0;
        var_tmp__blk1543_dn1 = assign61610_e80145_d_n1;
        var_tmp__blk1543_dn2 = assign61610_e80145_d_n2;
        var_tmp__blk1543_dn3 = assign61610_e80145_d_n3;
        var_tmp__blk1543_dn4 = assign61610_e80145_d_n4;
        var_tmp__blk1543_dn5 = assign61610_e80145_d_n5;
        var_tmp__blk1543_dn6 = assign61610_e80145_d_n6;
        var_tmp__blk1543_dn7 = assign61610_e80145_d_n7;
        var_tmp__blk1543_dn8 = assign61610_e80145_d_n8;
        var_tmp__blk1543_dn9 = assign61610_e80145_d_n9;
        var_tmp__blk1543_dn10 = assign61610_e80145_d_n10;
        var_tmp__blk1543_dn11 = assign61610_e80145_d_n11;
        var_tmp__blk1543_db0 = assign61610_e80145_d_b0;
        var_tmp__blk1543_db1 = assign61610_e80145_d_b1;
        var_tmp__blk1543_db2 = assign61610_e80145_d_b2;
        var_tmp__blk1543_db3 = assign61610_e80145_d_b3;
        var_tmp__blk1543_db4 = assign61610_e80145_d_b4;
        var_tmp__blk1543_db5 = assign61610_e80145_d_b5;
        var_tmp__blk1543_db6 = assign61610_e80145_d_b6;

        let (assign61620_e80166, assign61620_e80166_d_n0, assign61620_e80166_d_n1, assign61620_e80166_d_n2, assign61620_e80166_d_n3, assign61620_e80166_d_n4, assign61620_e80166_d_n5, assign61620_e80166_d_n6, assign61620_e80166_d_n7, assign61620_e80166_d_n8, assign61620_e80166_d_n9, assign61620_e80166_d_n10, assign61620_e80166_d_n11, assign61620_e80166_d_b0, assign61620_e80166_d_b1, assign61620_e80166_d_b2, assign61620_e80166_d_b3, assign61620_e80166_d_b4, assign61620_e80166_d_b5, assign61620_e80166_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) && (var_guard1715 == 0.0)) {
        let assign61620_e80161: f64 = (var_vjtmp * var_vbiinvgat_d);
        let assign61620_e80162: f64 = (1.0 - assign61620_e80161);
        let assign61620_e80164: f64 = (assign61620_e80162).powf(var_one_minus_pgat_d);
        (assign61620_e80164, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn0 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn0 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn1 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn1 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn2 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn2 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn3 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn3 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn4 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn4 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn5 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn5 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn6 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn6 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn7 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn7 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn8 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn8 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn9 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn9 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn10 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn10 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn11 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn11 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_db0 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_db0 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_db1 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_db1 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_db2 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_db2 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_db3 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_db3 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_db4 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_db4 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_db5 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_db5 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_db6 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_db6 * var_vbiinvgat_d)) / assign61620_e80162))) },)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61620_e80166;
        var_tmp__blk1543_dn0 = assign61620_e80166_d_n0;
        var_tmp__blk1543_dn1 = assign61620_e80166_d_n1;
        var_tmp__blk1543_dn2 = assign61620_e80166_d_n2;
        var_tmp__blk1543_dn3 = assign61620_e80166_d_n3;
        var_tmp__blk1543_dn4 = assign61620_e80166_d_n4;
        var_tmp__blk1543_dn5 = assign61620_e80166_d_n5;
        var_tmp__blk1543_dn6 = assign61620_e80166_d_n6;
        var_tmp__blk1543_dn7 = assign61620_e80166_d_n7;
        var_tmp__blk1543_dn8 = assign61620_e80166_d_n8;
        var_tmp__blk1543_dn9 = assign61620_e80166_d_n9;
        var_tmp__blk1543_dn10 = assign61620_e80166_d_n10;
        var_tmp__blk1543_dn11 = assign61620_e80166_d_n11;
        var_tmp__blk1543_db0 = assign61620_e80166_d_b0;
        var_tmp__blk1543_db1 = assign61620_e80166_d_b1;
        var_tmp__blk1543_db2 = assign61620_e80166_d_b2;
        var_tmp__blk1543_db3 = assign61620_e80166_d_b3;
        var_tmp__blk1543_db4 = assign61620_e80166_d_b4;
        var_tmp__blk1543_db5 = assign61620_e80166_d_b5;
        var_tmp__blk1543_db6 = assign61620_e80166_d_b6;

        let (assign61630_e80190, assign61630_e80190_d_n0, assign61630_e80190_d_n1, assign61630_e80190_d_n2, assign61630_e80190_d_n3, assign61630_e80190_d_n4, assign61630_e80190_d_n5, assign61630_e80190_d_n6, assign61630_e80190_d_n7, assign61630_e80190_d_n8, assign61630_e80190_d_n9, assign61630_e80190_d_n10, assign61630_e80190_d_n11, assign61630_e80190_d_b0, assign61630_e80190_d_b1, assign61630_e80190_d_b2, assign61630_e80190_d_b3, assign61630_e80190_d_b4, assign61630_e80190_d_b5, assign61630_e80190_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61630_e80180: f64 = (1.0 - var_tmp__blk1543);
        let assign61630_e80181: f64 = (var_qprefgat_d * assign61630_e80180);
        let assign61630_e80185: f64 = (var_nu__blk1570 - var_vjtmp);
        let assign61630_e80186: f64 = (var_qpref2gat_d * assign61630_e80185);
        let assign61630_e80187: f64 = (assign61630_e80181 + assign61630_e80186);
        let assign61630_e80188: f64 = (p.p30 * assign61630_e80187);
        (assign61630_e80188, (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn0)) + (var_qpref2gat_d * (var_nu__blk1570_dn0 - var_vjtmp_dn0)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn1)) + (var_qpref2gat_d * (var_nu__blk1570_dn1 - var_vjtmp_dn1)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn2)) + (var_qpref2gat_d * (var_nu__blk1570_dn2 - var_vjtmp_dn2)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn3)) + (var_qpref2gat_d * (var_nu__blk1570_dn3 - var_vjtmp_dn3)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn4)) + (var_qpref2gat_d * (var_nu__blk1570_dn4 - var_vjtmp_dn4)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn5)) + (var_qpref2gat_d * (var_nu__blk1570_dn5 - var_vjtmp_dn5)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn6)) + (var_qpref2gat_d * (var_nu__blk1570_dn6 - var_vjtmp_dn6)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn7)) + (var_qpref2gat_d * (var_nu__blk1570_dn7 - var_vjtmp_dn7)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn8)) + (var_qpref2gat_d * (var_nu__blk1570_dn8 - var_vjtmp_dn8)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn9)) + (var_qpref2gat_d * (var_nu__blk1570_dn9 - var_vjtmp_dn9)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn10)) + (var_qpref2gat_d * (var_nu__blk1570_dn10 - var_vjtmp_dn10)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn11)) + (var_qpref2gat_d * (var_nu__blk1570_dn11 - var_vjtmp_dn11)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_db0)) + (var_qpref2gat_d * (var_nu__blk1570_db0 - var_vjtmp_db0)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_db1)) + (var_qpref2gat_d * (var_nu__blk1570_db1 - var_vjtmp_db1)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_db2)) + (var_qpref2gat_d * (var_nu__blk1570_db2 - var_vjtmp_db2)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_db3)) + (var_qpref2gat_d * (var_nu__blk1570_db3 - var_vjtmp_db3)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_db4)) + (var_qpref2gat_d * (var_nu__blk1570_db4 - var_vjtmp_db4)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_db5)) + (var_qpref2gat_d * (var_nu__blk1570_db5 - var_vjtmp_db5)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_db6)) + (var_qpref2gat_d * (var_nu__blk1570_db6 - var_vjtmp_db6)))),)
    } else {
        (var_qjungat_d, var_qjungat_d_dn0, var_qjungat_d_dn1, var_qjungat_d_dn2, var_qjungat_d_dn3, var_qjungat_d_dn4, var_qjungat_d_dn5, var_qjungat_d_dn6, var_qjungat_d_dn7, var_qjungat_d_dn8, var_qjungat_d_dn9, var_qjungat_d_dn10, var_qjungat_d_dn11, var_qjungat_d_db0, var_qjungat_d_db1, var_qjungat_d_db2, var_qjungat_d_db3, var_qjungat_d_db4, var_qjungat_d_db5, var_qjungat_d_db6,)
    }
};
        var_qjungat_d = assign61630_e80190;
        var_qjungat_d_dn0 = assign61630_e80190_d_n0;
        var_qjungat_d_dn1 = assign61630_e80190_d_n1;
        var_qjungat_d_dn2 = assign61630_e80190_d_n2;
        var_qjungat_d_dn3 = assign61630_e80190_d_n3;
        var_qjungat_d_dn4 = assign61630_e80190_d_n4;
        var_qjungat_d_dn5 = assign61630_e80190_d_n5;
        var_qjungat_d_dn6 = assign61630_e80190_d_n6;
        var_qjungat_d_dn7 = assign61630_e80190_d_n7;
        var_qjungat_d_dn8 = assign61630_e80190_d_n8;
        var_qjungat_d_dn9 = assign61630_e80190_d_n9;
        var_qjungat_d_dn10 = assign61630_e80190_d_n10;
        var_qjungat_d_dn11 = assign61630_e80190_d_n11;
        var_qjungat_d_db0 = assign61630_e80190_d_b0;
        var_qjungat_d_db1 = assign61630_e80190_d_b1;
        var_qjungat_d_db2 = assign61630_e80190_d_b2;
        var_qjungat_d_db3 = assign61630_e80190_d_b3;
        var_qjungat_d_db4 = assign61630_e80190_d_b4;
        var_qjungat_d_db5 = assign61630_e80190_d_b5;
        var_qjungat_d_db6 = assign61630_e80190_d_b6;

        *var_fbreakdown__blk1569_slot = var_fbreakdown__blk1569;
        *var_fbreakdown__blk1569_db0_slot = var_fbreakdown__blk1569_db0;
        *var_fbreakdown__blk1569_db1_slot = var_fbreakdown__blk1569_db1;
        *var_fbreakdown__blk1569_db2_slot = var_fbreakdown__blk1569_db2;
        *var_fbreakdown__blk1569_db3_slot = var_fbreakdown__blk1569_db3;
        *var_fbreakdown__blk1569_db4_slot = var_fbreakdown__blk1569_db4;
        *var_fbreakdown__blk1569_db5_slot = var_fbreakdown__blk1569_db5;
        *var_fbreakdown__blk1569_db6_slot = var_fbreakdown__blk1569_db6;
        *var_fbreakdown__blk1569_dn0_slot = var_fbreakdown__blk1569_dn0;
        *var_fbreakdown__blk1569_dn1_slot = var_fbreakdown__blk1569_dn1;
        *var_fbreakdown__blk1569_dn10_slot = var_fbreakdown__blk1569_dn10;
        *var_fbreakdown__blk1569_dn11_slot = var_fbreakdown__blk1569_dn11;
        *var_fbreakdown__blk1569_dn2_slot = var_fbreakdown__blk1569_dn2;
        *var_fbreakdown__blk1569_dn3_slot = var_fbreakdown__blk1569_dn3;
        *var_fbreakdown__blk1569_dn4_slot = var_fbreakdown__blk1569_dn4;
        *var_fbreakdown__blk1569_dn5_slot = var_fbreakdown__blk1569_dn5;
        *var_fbreakdown__blk1569_dn6_slot = var_fbreakdown__blk1569_dn6;
        *var_fbreakdown__blk1569_dn7_slot = var_fbreakdown__blk1569_dn7;
        *var_fbreakdown__blk1569_dn8_slot = var_fbreakdown__blk1569_dn8;
        *var_fbreakdown__blk1569_dn9_slot = var_fbreakdown__blk1569_dn9;
        *var_guard1714_slot = var_guard1714;
        *var_guard1715_slot = var_guard1715;
        *var_h1__blk1528_slot = var_h1__blk1528;
        *var_h2__blk1529_slot = var_h2__blk1529;
        *var_h2d__blk1530_slot = var_h2d__blk1530;
        *var_h2d__blk1530_db0_slot = var_h2d__blk1530_db0;
        *var_h2d__blk1530_db1_slot = var_h2d__blk1530_db1;
        *var_h2d__blk1530_db2_slot = var_h2d__blk1530_db2;
        *var_h2d__blk1530_db3_slot = var_h2d__blk1530_db3;
        *var_h2d__blk1530_db4_slot = var_h2d__blk1530_db4;
        *var_h2d__blk1530_db5_slot = var_h2d__blk1530_db5;
        *var_h2d__blk1530_db6_slot = var_h2d__blk1530_db6;
        *var_h2d__blk1530_dn0_slot = var_h2d__blk1530_dn0;
        *var_h2d__blk1530_dn1_slot = var_h2d__blk1530_dn1;
        *var_h2d__blk1530_dn10_slot = var_h2d__blk1530_dn10;
        *var_h2d__blk1530_dn11_slot = var_h2d__blk1530_dn11;
        *var_h2d__blk1530_dn2_slot = var_h2d__blk1530_dn2;
        *var_h2d__blk1530_dn3_slot = var_h2d__blk1530_dn3;
        *var_h2d__blk1530_dn4_slot = var_h2d__blk1530_dn4;
        *var_h2d__blk1530_dn5_slot = var_h2d__blk1530_dn5;
        *var_h2d__blk1530_dn6_slot = var_h2d__blk1530_dn6;
        *var_h2d__blk1530_dn7_slot = var_h2d__blk1530_dn7;
        *var_h2d__blk1530_dn8_slot = var_h2d__blk1530_dn8;
        *var_h2d__blk1530_dn9_slot = var_h2d__blk1530_dn9;
        *var_h3__blk1531_slot = var_h3__blk1531;
        *var_h3__blk1531_db0_slot = var_h3__blk1531_db0;
        *var_h3__blk1531_db1_slot = var_h3__blk1531_db1;
        *var_h3__blk1531_db2_slot = var_h3__blk1531_db2;
        *var_h3__blk1531_db3_slot = var_h3__blk1531_db3;
        *var_h3__blk1531_db4_slot = var_h3__blk1531_db4;
        *var_h3__blk1531_db5_slot = var_h3__blk1531_db5;
        *var_h3__blk1531_db6_slot = var_h3__blk1531_db6;
        *var_h3__blk1531_dn0_slot = var_h3__blk1531_dn0;
        *var_h3__blk1531_dn1_slot = var_h3__blk1531_dn1;
        *var_h3__blk1531_dn10_slot = var_h3__blk1531_dn10;
        *var_h3__blk1531_dn11_slot = var_h3__blk1531_dn11;
        *var_h3__blk1531_dn2_slot = var_h3__blk1531_dn2;
        *var_h3__blk1531_dn3_slot = var_h3__blk1531_dn3;
        *var_h3__blk1531_dn4_slot = var_h3__blk1531_dn4;
        *var_h3__blk1531_dn5_slot = var_h3__blk1531_dn5;
        *var_h3__blk1531_dn6_slot = var_h3__blk1531_dn6;
        *var_h3__blk1531_dn7_slot = var_h3__blk1531_dn7;
        *var_h3__blk1531_dn8_slot = var_h3__blk1531_dn8;
        *var_h3__blk1531_dn9_slot = var_h3__blk1531_dn9;
        *var_h4__blk1532_slot = var_h4__blk1532;
        *var_h4__blk1532_db0_slot = var_h4__blk1532_db0;
        *var_h4__blk1532_db1_slot = var_h4__blk1532_db1;
        *var_h4__blk1532_db2_slot = var_h4__blk1532_db2;
        *var_h4__blk1532_db3_slot = var_h4__blk1532_db3;
        *var_h4__blk1532_db4_slot = var_h4__blk1532_db4;
        *var_h4__blk1532_db5_slot = var_h4__blk1532_db5;
        *var_h4__blk1532_db6_slot = var_h4__blk1532_db6;
        *var_h4__blk1532_dn0_slot = var_h4__blk1532_dn0;
        *var_h4__blk1532_dn1_slot = var_h4__blk1532_dn1;
        *var_h4__blk1532_dn10_slot = var_h4__blk1532_dn10;
        *var_h4__blk1532_dn11_slot = var_h4__blk1532_dn11;
        *var_h4__blk1532_dn2_slot = var_h4__blk1532_dn2;
        *var_h4__blk1532_dn3_slot = var_h4__blk1532_dn3;
        *var_h4__blk1532_dn4_slot = var_h4__blk1532_dn4;
        *var_h4__blk1532_dn5_slot = var_h4__blk1532_dn5;
        *var_h4__blk1532_dn6_slot = var_h4__blk1532_dn6;
        *var_h4__blk1532_dn7_slot = var_h4__blk1532_dn7;
        *var_h4__blk1532_dn8_slot = var_h4__blk1532_dn8;
        *var_h4__blk1532_dn9_slot = var_h4__blk1532_dn9;
        *var_h5__blk1533_slot = var_h5__blk1533;
        *var_h5__blk1533_db0_slot = var_h5__blk1533_db0;
        *var_h5__blk1533_db1_slot = var_h5__blk1533_db1;
        *var_h5__blk1533_db2_slot = var_h5__blk1533_db2;
        *var_h5__blk1533_db3_slot = var_h5__blk1533_db3;
        *var_h5__blk1533_db4_slot = var_h5__blk1533_db4;
        *var_h5__blk1533_db5_slot = var_h5__blk1533_db5;
        *var_h5__blk1533_db6_slot = var_h5__blk1533_db6;
        *var_h5__blk1533_dn0_slot = var_h5__blk1533_dn0;
        *var_h5__blk1533_dn1_slot = var_h5__blk1533_dn1;
        *var_h5__blk1533_dn10_slot = var_h5__blk1533_dn10;
        *var_h5__blk1533_dn11_slot = var_h5__blk1533_dn11;
        *var_h5__blk1533_dn2_slot = var_h5__blk1533_dn2;
        *var_h5__blk1533_dn3_slot = var_h5__blk1533_dn3;
        *var_h5__blk1533_dn4_slot = var_h5__blk1533_dn4;
        *var_h5__blk1533_dn5_slot = var_h5__blk1533_dn5;
        *var_h5__blk1533_dn6_slot = var_h5__blk1533_dn6;
        *var_h5__blk1533_dn7_slot = var_h5__blk1533_dn7;
        *var_h5__blk1533_dn8_slot = var_h5__blk1533_dn8;
        *var_h5__blk1533_dn9_slot = var_h5__blk1533_dn9;
        *var_ijungat_d_slot = var_ijungat_d;
        *var_ijungat_d_db0_slot = var_ijungat_d_db0;
        *var_ijungat_d_db1_slot = var_ijungat_d_db1;
        *var_ijungat_d_db2_slot = var_ijungat_d_db2;
        *var_ijungat_d_db3_slot = var_ijungat_d_db3;
        *var_ijungat_d_db4_slot = var_ijungat_d_db4;
        *var_ijungat_d_db5_slot = var_ijungat_d_db5;
        *var_ijungat_d_db6_slot = var_ijungat_d_db6;
        *var_ijungat_d_dn0_slot = var_ijungat_d_dn0;
        *var_ijungat_d_dn1_slot = var_ijungat_d_dn1;
        *var_ijungat_d_dn10_slot = var_ijungat_d_dn10;
        *var_ijungat_d_dn11_slot = var_ijungat_d_dn11;
        *var_ijungat_d_dn2_slot = var_ijungat_d_dn2;
        *var_ijungat_d_dn3_slot = var_ijungat_d_dn3;
        *var_ijungat_d_dn4_slot = var_ijungat_d_dn4;
        *var_ijungat_d_dn5_slot = var_ijungat_d_dn5;
        *var_ijungat_d_dn6_slot = var_ijungat_d_dn6;
        *var_ijungat_d_dn7_slot = var_ijungat_d_dn7;
        *var_ijungat_d_dn8_slot = var_ijungat_d_dn8;
        *var_ijungat_d_dn9_slot = var_ijungat_d_dn9;
        *var_nu__blk1570_slot = var_nu__blk1570;
        *var_nu__blk1570_db0_slot = var_nu__blk1570_db0;
        *var_nu__blk1570_db1_slot = var_nu__blk1570_db1;
        *var_nu__blk1570_db2_slot = var_nu__blk1570_db2;
        *var_nu__blk1570_db3_slot = var_nu__blk1570_db3;
        *var_nu__blk1570_db4_slot = var_nu__blk1570_db4;
        *var_nu__blk1570_db5_slot = var_nu__blk1570_db5;
        *var_nu__blk1570_db6_slot = var_nu__blk1570_db6;
        *var_nu__blk1570_dn0_slot = var_nu__blk1570_dn0;
        *var_nu__blk1570_dn1_slot = var_nu__blk1570_dn1;
        *var_nu__blk1570_dn10_slot = var_nu__blk1570_dn10;
        *var_nu__blk1570_dn11_slot = var_nu__blk1570_dn11;
        *var_nu__blk1570_dn2_slot = var_nu__blk1570_dn2;
        *var_nu__blk1570_dn3_slot = var_nu__blk1570_dn3;
        *var_nu__blk1570_dn4_slot = var_nu__blk1570_dn4;
        *var_nu__blk1570_dn5_slot = var_nu__blk1570_dn5;
        *var_nu__blk1570_dn6_slot = var_nu__blk1570_dn6;
        *var_nu__blk1570_dn7_slot = var_nu__blk1570_dn7;
        *var_nu__blk1570_dn8_slot = var_nu__blk1570_dn8;
        *var_nu__blk1570_dn9_slot = var_nu__blk1570_dn9;
        *var_qjungat_d_slot = var_qjungat_d;
        *var_qjungat_d_db0_slot = var_qjungat_d_db0;
        *var_qjungat_d_db1_slot = var_qjungat_d_db1;
        *var_qjungat_d_db2_slot = var_qjungat_d_db2;
        *var_qjungat_d_db3_slot = var_qjungat_d_db3;
        *var_qjungat_d_db4_slot = var_qjungat_d_db4;
        *var_qjungat_d_db5_slot = var_qjungat_d_db5;
        *var_qjungat_d_db6_slot = var_qjungat_d_db6;
        *var_qjungat_d_dn0_slot = var_qjungat_d_dn0;
        *var_qjungat_d_dn1_slot = var_qjungat_d_dn1;
        *var_qjungat_d_dn10_slot = var_qjungat_d_dn10;
        *var_qjungat_d_dn11_slot = var_qjungat_d_dn11;
        *var_qjungat_d_dn2_slot = var_qjungat_d_dn2;
        *var_qjungat_d_dn3_slot = var_qjungat_d_dn3;
        *var_qjungat_d_dn4_slot = var_qjungat_d_dn4;
        *var_qjungat_d_dn5_slot = var_qjungat_d_dn5;
        *var_qjungat_d_dn6_slot = var_qjungat_d_dn6;
        *var_qjungat_d_dn7_slot = var_qjungat_d_dn7;
        *var_qjungat_d_dn8_slot = var_qjungat_d_dn8;
        *var_qjungat_d_dn9_slot = var_qjungat_d_dn9;
        *var_tmp__blk1543_slot = var_tmp__blk1543;
        *var_tmp__blk1543_db0_slot = var_tmp__blk1543_db0;
        *var_tmp__blk1543_db1_slot = var_tmp__blk1543_db1;
        *var_tmp__blk1543_db2_slot = var_tmp__blk1543_db2;
        *var_tmp__blk1543_db3_slot = var_tmp__blk1543_db3;
        *var_tmp__blk1543_db4_slot = var_tmp__blk1543_db4;
        *var_tmp__blk1543_db5_slot = var_tmp__blk1543_db5;
        *var_tmp__blk1543_db6_slot = var_tmp__blk1543_db6;
        *var_tmp__blk1543_dn0_slot = var_tmp__blk1543_dn0;
        *var_tmp__blk1543_dn1_slot = var_tmp__blk1543_dn1;
        *var_tmp__blk1543_dn10_slot = var_tmp__blk1543_dn10;
        *var_tmp__blk1543_dn11_slot = var_tmp__blk1543_dn11;
        *var_tmp__blk1543_dn2_slot = var_tmp__blk1543_dn2;
        *var_tmp__blk1543_dn3_slot = var_tmp__blk1543_dn3;
        *var_tmp__blk1543_dn4_slot = var_tmp__blk1543_dn4;
        *var_tmp__blk1543_dn5_slot = var_tmp__blk1543_dn5;
        *var_tmp__blk1543_dn6_slot = var_tmp__blk1543_dn6;
        *var_tmp__blk1543_dn7_slot = var_tmp__blk1543_dn7;
        *var_tmp__blk1543_dn8_slot = var_tmp__blk1543_dn8;
        *var_tmp__blk1543_dn9_slot = var_tmp__blk1543_dn9;
        *var_vjtmp_slot = var_vjtmp;
        *var_vjtmp_db0_slot = var_vjtmp_db0;
        *var_vjtmp_db1_slot = var_vjtmp_db1;
        *var_vjtmp_db2_slot = var_vjtmp_db2;
        *var_vjtmp_db3_slot = var_vjtmp_db3;
        *var_vjtmp_db4_slot = var_vjtmp_db4;
        *var_vjtmp_db5_slot = var_vjtmp_db5;
        *var_vjtmp_db6_slot = var_vjtmp_db6;
        *var_vjtmp_dn0_slot = var_vjtmp_dn0;
        *var_vjtmp_dn1_slot = var_vjtmp_dn1;
        *var_vjtmp_dn10_slot = var_vjtmp_dn10;
        *var_vjtmp_dn11_slot = var_vjtmp_dn11;
        *var_vjtmp_dn2_slot = var_vjtmp_dn2;
        *var_vjtmp_dn3_slot = var_vjtmp_dn3;
        *var_vjtmp_dn4_slot = var_vjtmp_dn4;
        *var_vjtmp_dn5_slot = var_vjtmp_dn5;
        *var_vjtmp_dn6_slot = var_vjtmp_dn6;
        *var_vjtmp_dn7_slot = var_vjtmp_dn7;
        *var_vjtmp_dn8_slot = var_vjtmp_dn8;
        *var_vjtmp_dn9_slot = var_vjtmp_dn9;
    }

    pub(super) fn stamp_transient_block_243(
        p: &Parameters,
        var_abdrain_i: f64,
        var_guard1572: f64,
        var_guard1573: f64,
        var_guard1697: f64,
        var_guard1714: f64,
        var_ijunbot_d: f64,
        var_ijunbot_d_db0: f64,
        var_ijunbot_d_db1: f64,
        var_ijunbot_d_db2: f64,
        var_ijunbot_d_db3: f64,
        var_ijunbot_d_db4: f64,
        var_ijunbot_d_db5: f64,
        var_ijunbot_d_db6: f64,
        var_ijunbot_d_dn0: f64,
        var_ijunbot_d_dn1: f64,
        var_ijunbot_d_dn10: f64,
        var_ijunbot_d_dn11: f64,
        var_ijunbot_d_dn2: f64,
        var_ijunbot_d_dn3: f64,
        var_ijunbot_d_dn4: f64,
        var_ijunbot_d_dn5: f64,
        var_ijunbot_d_dn6: f64,
        var_ijunbot_d_dn7: f64,
        var_ijunbot_d_dn8: f64,
        var_ijunbot_d_dn9: f64,
        var_ijungat_d: f64,
        var_ijungat_d_db0: f64,
        var_ijungat_d_db1: f64,
        var_ijungat_d_db2: f64,
        var_ijungat_d_db3: f64,
        var_ijungat_d_db4: f64,
        var_ijungat_d_db5: f64,
        var_ijungat_d_db6: f64,
        var_ijungat_d_dn0: f64,
        var_ijungat_d_dn1: f64,
        var_ijungat_d_dn10: f64,
        var_ijungat_d_dn11: f64,
        var_ijungat_d_dn2: f64,
        var_ijungat_d_dn3: f64,
        var_ijungat_d_dn4: f64,
        var_ijungat_d_dn5: f64,
        var_ijungat_d_dn6: f64,
        var_ijungat_d_dn7: f64,
        var_ijungat_d_dn8: f64,
        var_ijungat_d_dn9: f64,
        var_ijunsti_d: f64,
        var_ijunsti_d_db0: f64,
        var_ijunsti_d_db1: f64,
        var_ijunsti_d_db2: f64,
        var_ijunsti_d_db3: f64,
        var_ijunsti_d_db4: f64,
        var_ijunsti_d_db5: f64,
        var_ijunsti_d_db6: f64,
        var_ijunsti_d_dn0: f64,
        var_ijunsti_d_dn1: f64,
        var_ijunsti_d_dn10: f64,
        var_ijunsti_d_dn11: f64,
        var_ijunsti_d_dn2: f64,
        var_ijunsti_d_dn3: f64,
        var_ijunsti_d_dn4: f64,
        var_ijunsti_d_dn5: f64,
        var_ijunsti_d_dn6: f64,
        var_ijunsti_d_dn7: f64,
        var_ijunsti_d_dn8: f64,
        var_ijunsti_d_dn9: f64,
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_one_minus_pgat2nd_d: f64,
        var_one_minus_pgat_d: f64,
        var_qb: f64,
        var_qb_db0: f64,
        var_qb_db1: f64,
        var_qb_db2: f64,
        var_qb_db3: f64,
        var_qb_db4: f64,
        var_qb_db5: f64,
        var_qb_db6: f64,
        var_qb_dn0: f64,
        var_qb_dn1: f64,
        var_qb_dn10: f64,
        var_qb_dn11: f64,
        var_qb_dn2: f64,
        var_qb_dn3: f64,
        var_qb_dn4: f64,
        var_qb_dn5: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qb_dn9: f64,
        var_qd: f64,
        var_qd_db0: f64,
        var_qd_db1: f64,
        var_qd_db2: f64,
        var_qd_db3: f64,
        var_qd_db4: f64,
        var_qd_db5: f64,
        var_qd_db6: f64,
        var_qd_dn0: f64,
        var_qd_dn1: f64,
        var_qd_dn10: f64,
        var_qd_dn11: f64,
        var_qd_dn2: f64,
        var_qd_dn3: f64,
        var_qd_dn4: f64,
        var_qd_dn5: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qd_dn8: f64,
        var_qd_dn9: f64,
        var_qg: f64,
        var_qg_db0: f64,
        var_qg_db1: f64,
        var_qg_db2: f64,
        var_qg_db3: f64,
        var_qg_db4: f64,
        var_qg_db5: f64,
        var_qg_db6: f64,
        var_qg_dn0: f64,
        var_qg_dn1: f64,
        var_qg_dn10: f64,
        var_qg_dn11: f64,
        var_qg_dn2: f64,
        var_qg_dn3: f64,
        var_qg_dn4: f64,
        var_qg_dn5: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_qg_dn8: f64,
        var_qg_dn9: f64,
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
        var_rwell_i: f64,
        var_sigvds: f64,
        var_vbiinvgat2nd_d: f64,
        var_vbiinvgat_d: f64,
        var_vch_d: f64,
        var_vfmin_d: f64,
        var_vj__blk1535: f64,
        var_vj__blk1535_db0: f64,
        var_vj__blk1535_db1: f64,
        var_vj__blk1535_db2: f64,
        var_vj__blk1535_db3: f64,
        var_vj__blk1535_db4: f64,
        var_vj__blk1535_db5: f64,
        var_vj__blk1535_db6: f64,
        var_vj__blk1535_dn0: f64,
        var_vj__blk1535_dn1: f64,
        var_vj__blk1535_dn10: f64,
        var_vj__blk1535_dn11: f64,
        var_vj__blk1535_dn2: f64,
        var_vj__blk1535_dn3: f64,
        var_vj__blk1535_dn4: f64,
        var_vj__blk1535_dn5: f64,
        var_vj__blk1535_dn6: f64,
        var_vj__blk1535_dn7: f64,
        var_vj__blk1535_dn8: f64,
        var_vj__blk1535_dn9: f64,
        var_vjun_d: f64,
        var_vjun_d_db0: f64,
        var_vjun_d_db1: f64,
        var_vjun_d_db2: f64,
        var_vjun_d_db3: f64,
        var_vjun_d_db4: f64,
        var_vjun_d_db5: f64,
        var_vjun_d_db6: f64,
        var_vjun_d_dn0: f64,
        var_vjun_d_dn1: f64,
        var_vjun_d_dn10: f64,
        var_vjun_d_dn11: f64,
        var_vjun_d_dn2: f64,
        var_vjun_d_dn3: f64,
        var_vjun_d_dn4: f64,
        var_vjun_d_dn5: f64,
        var_vjun_d_dn6: f64,
        var_vjun_d_dn7: f64,
        var_vjun_d_dn8: f64,
        var_vjun_d_dn9: f64,
        var_vtrgatd_i: f64,
        var_guard1716_slot: &mut f64,
        var_guard1717_slot: &mut f64,
        var_guard1718_slot: &mut f64,
        var_guard1719_slot: &mut f64,
        var_guard1720_slot: &mut f64,
        var_guard1721_slot: &mut f64,
        var_guard1722_slot: &mut f64,
        var_guard1723_slot: &mut f64,
        var_guard1724_slot: &mut f64,
        var_guard1725_slot: &mut f64,
        var_h1__blk1528_slot: &mut f64,
        var_h2__blk1529_slot: &mut f64,
        var_h2d__blk1530_slot: &mut f64,
        var_h2d__blk1530_db0_slot: &mut f64,
        var_h2d__blk1530_db1_slot: &mut f64,
        var_h2d__blk1530_db2_slot: &mut f64,
        var_h2d__blk1530_db3_slot: &mut f64,
        var_h2d__blk1530_db4_slot: &mut f64,
        var_h2d__blk1530_db5_slot: &mut f64,
        var_h2d__blk1530_db6_slot: &mut f64,
        var_h2d__blk1530_dn0_slot: &mut f64,
        var_h2d__blk1530_dn1_slot: &mut f64,
        var_h2d__blk1530_dn10_slot: &mut f64,
        var_h2d__blk1530_dn11_slot: &mut f64,
        var_h2d__blk1530_dn2_slot: &mut f64,
        var_h2d__blk1530_dn3_slot: &mut f64,
        var_h2d__blk1530_dn4_slot: &mut f64,
        var_h2d__blk1530_dn5_slot: &mut f64,
        var_h2d__blk1530_dn6_slot: &mut f64,
        var_h2d__blk1530_dn7_slot: &mut f64,
        var_h2d__blk1530_dn8_slot: &mut f64,
        var_h2d__blk1530_dn9_slot: &mut f64,
        var_h3__blk1531_slot: &mut f64,
        var_h3__blk1531_db0_slot: &mut f64,
        var_h3__blk1531_db1_slot: &mut f64,
        var_h3__blk1531_db2_slot: &mut f64,
        var_h3__blk1531_db3_slot: &mut f64,
        var_h3__blk1531_db4_slot: &mut f64,
        var_h3__blk1531_db5_slot: &mut f64,
        var_h3__blk1531_db6_slot: &mut f64,
        var_h3__blk1531_dn0_slot: &mut f64,
        var_h3__blk1531_dn1_slot: &mut f64,
        var_h3__blk1531_dn10_slot: &mut f64,
        var_h3__blk1531_dn11_slot: &mut f64,
        var_h3__blk1531_dn2_slot: &mut f64,
        var_h3__blk1531_dn3_slot: &mut f64,
        var_h3__blk1531_dn4_slot: &mut f64,
        var_h3__blk1531_dn5_slot: &mut f64,
        var_h3__blk1531_dn6_slot: &mut f64,
        var_h3__blk1531_dn7_slot: &mut f64,
        var_h3__blk1531_dn8_slot: &mut f64,
        var_h3__blk1531_dn9_slot: &mut f64,
        var_h4__blk1532_slot: &mut f64,
        var_h4__blk1532_db0_slot: &mut f64,
        var_h4__blk1532_db1_slot: &mut f64,
        var_h4__blk1532_db2_slot: &mut f64,
        var_h4__blk1532_db3_slot: &mut f64,
        var_h4__blk1532_db4_slot: &mut f64,
        var_h4__blk1532_db5_slot: &mut f64,
        var_h4__blk1532_db6_slot: &mut f64,
        var_h4__blk1532_dn0_slot: &mut f64,
        var_h4__blk1532_dn1_slot: &mut f64,
        var_h4__blk1532_dn10_slot: &mut f64,
        var_h4__blk1532_dn11_slot: &mut f64,
        var_h4__blk1532_dn2_slot: &mut f64,
        var_h4__blk1532_dn3_slot: &mut f64,
        var_h4__blk1532_dn4_slot: &mut f64,
        var_h4__blk1532_dn5_slot: &mut f64,
        var_h4__blk1532_dn6_slot: &mut f64,
        var_h4__blk1532_dn7_slot: &mut f64,
        var_h4__blk1532_dn8_slot: &mut f64,
        var_h4__blk1532_dn9_slot: &mut f64,
        var_h5__blk1533_slot: &mut f64,
        var_h5__blk1533_db0_slot: &mut f64,
        var_h5__blk1533_db1_slot: &mut f64,
        var_h5__blk1533_db2_slot: &mut f64,
        var_h5__blk1533_db3_slot: &mut f64,
        var_h5__blk1533_db4_slot: &mut f64,
        var_h5__blk1533_db5_slot: &mut f64,
        var_h5__blk1533_db6_slot: &mut f64,
        var_h5__blk1533_dn0_slot: &mut f64,
        var_h5__blk1533_dn1_slot: &mut f64,
        var_h5__blk1533_dn10_slot: &mut f64,
        var_h5__blk1533_dn11_slot: &mut f64,
        var_h5__blk1533_dn2_slot: &mut f64,
        var_h5__blk1533_dn3_slot: &mut f64,
        var_h5__blk1533_dn4_slot: &mut f64,
        var_h5__blk1533_dn5_slot: &mut f64,
        var_h5__blk1533_dn6_slot: &mut f64,
        var_h5__blk1533_dn7_slot: &mut f64,
        var_h5__blk1533_dn8_slot: &mut f64,
        var_h5__blk1533_dn9_slot: &mut f64,
        var_ijun_d_slot: &mut f64,
        var_ijun_d_db0_slot: &mut f64,
        var_ijun_d_db1_slot: &mut f64,
        var_ijun_d_db2_slot: &mut f64,
        var_ijun_d_db3_slot: &mut f64,
        var_ijun_d_db4_slot: &mut f64,
        var_ijun_d_db5_slot: &mut f64,
        var_ijun_d_db6_slot: &mut f64,
        var_ijun_d_dn0_slot: &mut f64,
        var_ijun_d_dn1_slot: &mut f64,
        var_ijun_d_dn10_slot: &mut f64,
        var_ijun_d_dn11_slot: &mut f64,
        var_ijun_d_dn2_slot: &mut f64,
        var_ijun_d_dn3_slot: &mut f64,
        var_ijun_d_dn4_slot: &mut f64,
        var_ijun_d_dn5_slot: &mut f64,
        var_ijun_d_dn6_slot: &mut f64,
        var_ijun_d_dn7_slot: &mut f64,
        var_ijun_d_dn8_slot: &mut f64,
        var_ijun_d_dn9_slot: &mut f64,
        var_nu__blk1570_slot: &mut f64,
        var_nu__blk1570_db0_slot: &mut f64,
        var_nu__blk1570_db1_slot: &mut f64,
        var_nu__blk1570_db2_slot: &mut f64,
        var_nu__blk1570_db3_slot: &mut f64,
        var_nu__blk1570_db4_slot: &mut f64,
        var_nu__blk1570_db5_slot: &mut f64,
        var_nu__blk1570_db6_slot: &mut f64,
        var_nu__blk1570_dn0_slot: &mut f64,
        var_nu__blk1570_dn1_slot: &mut f64,
        var_nu__blk1570_dn10_slot: &mut f64,
        var_nu__blk1570_dn11_slot: &mut f64,
        var_nu__blk1570_dn2_slot: &mut f64,
        var_nu__blk1570_dn3_slot: &mut f64,
        var_nu__blk1570_dn4_slot: &mut f64,
        var_nu__blk1570_dn5_slot: &mut f64,
        var_nu__blk1570_dn6_slot: &mut f64,
        var_nu__blk1570_dn7_slot: &mut f64,
        var_nu__blk1570_dn8_slot: &mut f64,
        var_nu__blk1570_dn9_slot: &mut f64,
        var_qjungat2nd_slot: &mut f64,
        var_qjungat2nd_db0_slot: &mut f64,
        var_qjungat2nd_db1_slot: &mut f64,
        var_qjungat2nd_db2_slot: &mut f64,
        var_qjungat2nd_db3_slot: &mut f64,
        var_qjungat2nd_db4_slot: &mut f64,
        var_qjungat2nd_db5_slot: &mut f64,
        var_qjungat2nd_db6_slot: &mut f64,
        var_qjungat2nd_dn0_slot: &mut f64,
        var_qjungat2nd_dn1_slot: &mut f64,
        var_qjungat2nd_dn10_slot: &mut f64,
        var_qjungat2nd_dn11_slot: &mut f64,
        var_qjungat2nd_dn2_slot: &mut f64,
        var_qjungat2nd_dn3_slot: &mut f64,
        var_qjungat2nd_dn4_slot: &mut f64,
        var_qjungat2nd_dn5_slot: &mut f64,
        var_qjungat2nd_dn6_slot: &mut f64,
        var_qjungat2nd_dn7_slot: &mut f64,
        var_qjungat2nd_dn8_slot: &mut f64,
        var_qjungat2nd_dn9_slot: &mut f64,
        var_qjungat_d_slot: &mut f64,
        var_qjungat_d_db0_slot: &mut f64,
        var_qjungat_d_db1_slot: &mut f64,
        var_qjungat_d_db2_slot: &mut f64,
        var_qjungat_d_db3_slot: &mut f64,
        var_qjungat_d_db4_slot: &mut f64,
        var_qjungat_d_db5_slot: &mut f64,
        var_qjungat_d_db6_slot: &mut f64,
        var_qjungat_d_dn0_slot: &mut f64,
        var_qjungat_d_dn1_slot: &mut f64,
        var_qjungat_d_dn10_slot: &mut f64,
        var_qjungat_d_dn11_slot: &mut f64,
        var_qjungat_d_dn2_slot: &mut f64,
        var_qjungat_d_dn3_slot: &mut f64,
        var_qjungat_d_dn4_slot: &mut f64,
        var_qjungat_d_dn5_slot: &mut f64,
        var_qjungat_d_dn6_slot: &mut f64,
        var_qjungat_d_dn7_slot: &mut f64,
        var_qjungat_d_dn8_slot: &mut f64,
        var_qjungat_d_dn9_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_db0_slot: &mut f64,
        var_qs_db1_slot: &mut f64,
        var_qs_db2_slot: &mut f64,
        var_qs_db3_slot: &mut f64,
        var_qs_db4_slot: &mut f64,
        var_qs_db5_slot: &mut f64,
        var_qs_db6_slot: &mut f64,
        var_qs_dn0_slot: &mut f64,
        var_qs_dn1_slot: &mut f64,
        var_qs_dn10_slot: &mut f64,
        var_qs_dn11_slot: &mut f64,
        var_qs_dn2_slot: &mut f64,
        var_qs_dn3_slot: &mut f64,
        var_qs_dn4_slot: &mut f64,
        var_qs_dn5_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_qs_dn9_slot: &mut f64,
        var_tmp__blk1543_slot: &mut f64,
        var_tmp__blk1543_db0_slot: &mut f64,
        var_tmp__blk1543_db1_slot: &mut f64,
        var_tmp__blk1543_db2_slot: &mut f64,
        var_tmp__blk1543_db3_slot: &mut f64,
        var_tmp__blk1543_db4_slot: &mut f64,
        var_tmp__blk1543_db5_slot: &mut f64,
        var_tmp__blk1543_db6_slot: &mut f64,
        var_tmp__blk1543_dn0_slot: &mut f64,
        var_tmp__blk1543_dn1_slot: &mut f64,
        var_tmp__blk1543_dn10_slot: &mut f64,
        var_tmp__blk1543_dn11_slot: &mut f64,
        var_tmp__blk1543_dn2_slot: &mut f64,
        var_tmp__blk1543_dn3_slot: &mut f64,
        var_tmp__blk1543_dn4_slot: &mut f64,
        var_tmp__blk1543_dn5_slot: &mut f64,
        var_tmp__blk1543_dn6_slot: &mut f64,
        var_tmp__blk1543_dn7_slot: &mut f64,
        var_tmp__blk1543_dn8_slot: &mut f64,
        var_tmp__blk1543_dn9_slot: &mut f64,
        var_vjtmp_slot: &mut f64,
        var_vjtmp_db0_slot: &mut f64,
        var_vjtmp_db1_slot: &mut f64,
        var_vjtmp_db2_slot: &mut f64,
        var_vjtmp_db3_slot: &mut f64,
        var_vjtmp_db4_slot: &mut f64,
        var_vjtmp_db5_slot: &mut f64,
        var_vjtmp_db6_slot: &mut f64,
        var_vjtmp_dn0_slot: &mut f64,
        var_vjtmp_dn1_slot: &mut f64,
        var_vjtmp_dn10_slot: &mut f64,
        var_vjtmp_dn11_slot: &mut f64,
        var_vjtmp_dn2_slot: &mut f64,
        var_vjtmp_dn3_slot: &mut f64,
        var_vjtmp_dn4_slot: &mut f64,
        var_vjtmp_dn5_slot: &mut f64,
        var_vjtmp_dn6_slot: &mut f64,
        var_vjtmp_dn7_slot: &mut f64,
        var_vjtmp_dn8_slot: &mut f64,
        var_vjtmp_dn9_slot: &mut f64,
    ) {
        let mut var_guard1716: f64 = *var_guard1716_slot;
        let mut var_guard1717: f64 = *var_guard1717_slot;
        let mut var_guard1718: f64 = *var_guard1718_slot;
        let mut var_guard1719: f64 = *var_guard1719_slot;
        let mut var_guard1720: f64 = *var_guard1720_slot;
        let mut var_guard1721: f64 = *var_guard1721_slot;
        let mut var_guard1722: f64 = *var_guard1722_slot;
        let mut var_guard1723: f64 = *var_guard1723_slot;
        let mut var_guard1724: f64 = *var_guard1724_slot;
        let mut var_guard1725: f64 = *var_guard1725_slot;
        let mut var_h1__blk1528: f64 = *var_h1__blk1528_slot;
        let mut var_h2__blk1529: f64 = *var_h2__blk1529_slot;
        let mut var_h2d__blk1530: f64 = *var_h2d__blk1530_slot;
        let mut var_h2d__blk1530_db0: f64 = *var_h2d__blk1530_db0_slot;
        let mut var_h2d__blk1530_db1: f64 = *var_h2d__blk1530_db1_slot;
        let mut var_h2d__blk1530_db2: f64 = *var_h2d__blk1530_db2_slot;
        let mut var_h2d__blk1530_db3: f64 = *var_h2d__blk1530_db3_slot;
        let mut var_h2d__blk1530_db4: f64 = *var_h2d__blk1530_db4_slot;
        let mut var_h2d__blk1530_db5: f64 = *var_h2d__blk1530_db5_slot;
        let mut var_h2d__blk1530_db6: f64 = *var_h2d__blk1530_db6_slot;
        let mut var_h2d__blk1530_dn0: f64 = *var_h2d__blk1530_dn0_slot;
        let mut var_h2d__blk1530_dn1: f64 = *var_h2d__blk1530_dn1_slot;
        let mut var_h2d__blk1530_dn10: f64 = *var_h2d__blk1530_dn10_slot;
        let mut var_h2d__blk1530_dn11: f64 = *var_h2d__blk1530_dn11_slot;
        let mut var_h2d__blk1530_dn2: f64 = *var_h2d__blk1530_dn2_slot;
        let mut var_h2d__blk1530_dn3: f64 = *var_h2d__blk1530_dn3_slot;
        let mut var_h2d__blk1530_dn4: f64 = *var_h2d__blk1530_dn4_slot;
        let mut var_h2d__blk1530_dn5: f64 = *var_h2d__blk1530_dn5_slot;
        let mut var_h2d__blk1530_dn6: f64 = *var_h2d__blk1530_dn6_slot;
        let mut var_h2d__blk1530_dn7: f64 = *var_h2d__blk1530_dn7_slot;
        let mut var_h2d__blk1530_dn8: f64 = *var_h2d__blk1530_dn8_slot;
        let mut var_h2d__blk1530_dn9: f64 = *var_h2d__blk1530_dn9_slot;
        let mut var_h3__blk1531: f64 = *var_h3__blk1531_slot;
        let mut var_h3__blk1531_db0: f64 = *var_h3__blk1531_db0_slot;
        let mut var_h3__blk1531_db1: f64 = *var_h3__blk1531_db1_slot;
        let mut var_h3__blk1531_db2: f64 = *var_h3__blk1531_db2_slot;
        let mut var_h3__blk1531_db3: f64 = *var_h3__blk1531_db3_slot;
        let mut var_h3__blk1531_db4: f64 = *var_h3__blk1531_db4_slot;
        let mut var_h3__blk1531_db5: f64 = *var_h3__blk1531_db5_slot;
        let mut var_h3__blk1531_db6: f64 = *var_h3__blk1531_db6_slot;
        let mut var_h3__blk1531_dn0: f64 = *var_h3__blk1531_dn0_slot;
        let mut var_h3__blk1531_dn1: f64 = *var_h3__blk1531_dn1_slot;
        let mut var_h3__blk1531_dn10: f64 = *var_h3__blk1531_dn10_slot;
        let mut var_h3__blk1531_dn11: f64 = *var_h3__blk1531_dn11_slot;
        let mut var_h3__blk1531_dn2: f64 = *var_h3__blk1531_dn2_slot;
        let mut var_h3__blk1531_dn3: f64 = *var_h3__blk1531_dn3_slot;
        let mut var_h3__blk1531_dn4: f64 = *var_h3__blk1531_dn4_slot;
        let mut var_h3__blk1531_dn5: f64 = *var_h3__blk1531_dn5_slot;
        let mut var_h3__blk1531_dn6: f64 = *var_h3__blk1531_dn6_slot;
        let mut var_h3__blk1531_dn7: f64 = *var_h3__blk1531_dn7_slot;
        let mut var_h3__blk1531_dn8: f64 = *var_h3__blk1531_dn8_slot;
        let mut var_h3__blk1531_dn9: f64 = *var_h3__blk1531_dn9_slot;
        let mut var_h4__blk1532: f64 = *var_h4__blk1532_slot;
        let mut var_h4__blk1532_db0: f64 = *var_h4__blk1532_db0_slot;
        let mut var_h4__blk1532_db1: f64 = *var_h4__blk1532_db1_slot;
        let mut var_h4__blk1532_db2: f64 = *var_h4__blk1532_db2_slot;
        let mut var_h4__blk1532_db3: f64 = *var_h4__blk1532_db3_slot;
        let mut var_h4__blk1532_db4: f64 = *var_h4__blk1532_db4_slot;
        let mut var_h4__blk1532_db5: f64 = *var_h4__blk1532_db5_slot;
        let mut var_h4__blk1532_db6: f64 = *var_h4__blk1532_db6_slot;
        let mut var_h4__blk1532_dn0: f64 = *var_h4__blk1532_dn0_slot;
        let mut var_h4__blk1532_dn1: f64 = *var_h4__blk1532_dn1_slot;
        let mut var_h4__blk1532_dn10: f64 = *var_h4__blk1532_dn10_slot;
        let mut var_h4__blk1532_dn11: f64 = *var_h4__blk1532_dn11_slot;
        let mut var_h4__blk1532_dn2: f64 = *var_h4__blk1532_dn2_slot;
        let mut var_h4__blk1532_dn3: f64 = *var_h4__blk1532_dn3_slot;
        let mut var_h4__blk1532_dn4: f64 = *var_h4__blk1532_dn4_slot;
        let mut var_h4__blk1532_dn5: f64 = *var_h4__blk1532_dn5_slot;
        let mut var_h4__blk1532_dn6: f64 = *var_h4__blk1532_dn6_slot;
        let mut var_h4__blk1532_dn7: f64 = *var_h4__blk1532_dn7_slot;
        let mut var_h4__blk1532_dn8: f64 = *var_h4__blk1532_dn8_slot;
        let mut var_h4__blk1532_dn9: f64 = *var_h4__blk1532_dn9_slot;
        let mut var_h5__blk1533: f64 = *var_h5__blk1533_slot;
        let mut var_h5__blk1533_db0: f64 = *var_h5__blk1533_db0_slot;
        let mut var_h5__blk1533_db1: f64 = *var_h5__blk1533_db1_slot;
        let mut var_h5__blk1533_db2: f64 = *var_h5__blk1533_db2_slot;
        let mut var_h5__blk1533_db3: f64 = *var_h5__blk1533_db3_slot;
        let mut var_h5__blk1533_db4: f64 = *var_h5__blk1533_db4_slot;
        let mut var_h5__blk1533_db5: f64 = *var_h5__blk1533_db5_slot;
        let mut var_h5__blk1533_db6: f64 = *var_h5__blk1533_db6_slot;
        let mut var_h5__blk1533_dn0: f64 = *var_h5__blk1533_dn0_slot;
        let mut var_h5__blk1533_dn1: f64 = *var_h5__blk1533_dn1_slot;
        let mut var_h5__blk1533_dn10: f64 = *var_h5__blk1533_dn10_slot;
        let mut var_h5__blk1533_dn11: f64 = *var_h5__blk1533_dn11_slot;
        let mut var_h5__blk1533_dn2: f64 = *var_h5__blk1533_dn2_slot;
        let mut var_h5__blk1533_dn3: f64 = *var_h5__blk1533_dn3_slot;
        let mut var_h5__blk1533_dn4: f64 = *var_h5__blk1533_dn4_slot;
        let mut var_h5__blk1533_dn5: f64 = *var_h5__blk1533_dn5_slot;
        let mut var_h5__blk1533_dn6: f64 = *var_h5__blk1533_dn6_slot;
        let mut var_h5__blk1533_dn7: f64 = *var_h5__blk1533_dn7_slot;
        let mut var_h5__blk1533_dn8: f64 = *var_h5__blk1533_dn8_slot;
        let mut var_h5__blk1533_dn9: f64 = *var_h5__blk1533_dn9_slot;
        let mut var_ijun_d: f64 = *var_ijun_d_slot;
        let mut var_ijun_d_db0: f64 = *var_ijun_d_db0_slot;
        let mut var_ijun_d_db1: f64 = *var_ijun_d_db1_slot;
        let mut var_ijun_d_db2: f64 = *var_ijun_d_db2_slot;
        let mut var_ijun_d_db3: f64 = *var_ijun_d_db3_slot;
        let mut var_ijun_d_db4: f64 = *var_ijun_d_db4_slot;
        let mut var_ijun_d_db5: f64 = *var_ijun_d_db5_slot;
        let mut var_ijun_d_db6: f64 = *var_ijun_d_db6_slot;
        let mut var_ijun_d_dn0: f64 = *var_ijun_d_dn0_slot;
        let mut var_ijun_d_dn1: f64 = *var_ijun_d_dn1_slot;
        let mut var_ijun_d_dn10: f64 = *var_ijun_d_dn10_slot;
        let mut var_ijun_d_dn11: f64 = *var_ijun_d_dn11_slot;
        let mut var_ijun_d_dn2: f64 = *var_ijun_d_dn2_slot;
        let mut var_ijun_d_dn3: f64 = *var_ijun_d_dn3_slot;
        let mut var_ijun_d_dn4: f64 = *var_ijun_d_dn4_slot;
        let mut var_ijun_d_dn5: f64 = *var_ijun_d_dn5_slot;
        let mut var_ijun_d_dn6: f64 = *var_ijun_d_dn6_slot;
        let mut var_ijun_d_dn7: f64 = *var_ijun_d_dn7_slot;
        let mut var_ijun_d_dn8: f64 = *var_ijun_d_dn8_slot;
        let mut var_ijun_d_dn9: f64 = *var_ijun_d_dn9_slot;
        let mut var_nu__blk1570: f64 = *var_nu__blk1570_slot;
        let mut var_nu__blk1570_db0: f64 = *var_nu__blk1570_db0_slot;
        let mut var_nu__blk1570_db1: f64 = *var_nu__blk1570_db1_slot;
        let mut var_nu__blk1570_db2: f64 = *var_nu__blk1570_db2_slot;
        let mut var_nu__blk1570_db3: f64 = *var_nu__blk1570_db3_slot;
        let mut var_nu__blk1570_db4: f64 = *var_nu__blk1570_db4_slot;
        let mut var_nu__blk1570_db5: f64 = *var_nu__blk1570_db5_slot;
        let mut var_nu__blk1570_db6: f64 = *var_nu__blk1570_db6_slot;
        let mut var_nu__blk1570_dn0: f64 = *var_nu__blk1570_dn0_slot;
        let mut var_nu__blk1570_dn1: f64 = *var_nu__blk1570_dn1_slot;
        let mut var_nu__blk1570_dn10: f64 = *var_nu__blk1570_dn10_slot;
        let mut var_nu__blk1570_dn11: f64 = *var_nu__blk1570_dn11_slot;
        let mut var_nu__blk1570_dn2: f64 = *var_nu__blk1570_dn2_slot;
        let mut var_nu__blk1570_dn3: f64 = *var_nu__blk1570_dn3_slot;
        let mut var_nu__blk1570_dn4: f64 = *var_nu__blk1570_dn4_slot;
        let mut var_nu__blk1570_dn5: f64 = *var_nu__blk1570_dn5_slot;
        let mut var_nu__blk1570_dn6: f64 = *var_nu__blk1570_dn6_slot;
        let mut var_nu__blk1570_dn7: f64 = *var_nu__blk1570_dn7_slot;
        let mut var_nu__blk1570_dn8: f64 = *var_nu__blk1570_dn8_slot;
        let mut var_nu__blk1570_dn9: f64 = *var_nu__blk1570_dn9_slot;
        let mut var_qjungat2nd: f64 = *var_qjungat2nd_slot;
        let mut var_qjungat2nd_db0: f64 = *var_qjungat2nd_db0_slot;
        let mut var_qjungat2nd_db1: f64 = *var_qjungat2nd_db1_slot;
        let mut var_qjungat2nd_db2: f64 = *var_qjungat2nd_db2_slot;
        let mut var_qjungat2nd_db3: f64 = *var_qjungat2nd_db3_slot;
        let mut var_qjungat2nd_db4: f64 = *var_qjungat2nd_db4_slot;
        let mut var_qjungat2nd_db5: f64 = *var_qjungat2nd_db5_slot;
        let mut var_qjungat2nd_db6: f64 = *var_qjungat2nd_db6_slot;
        let mut var_qjungat2nd_dn0: f64 = *var_qjungat2nd_dn0_slot;
        let mut var_qjungat2nd_dn1: f64 = *var_qjungat2nd_dn1_slot;
        let mut var_qjungat2nd_dn10: f64 = *var_qjungat2nd_dn10_slot;
        let mut var_qjungat2nd_dn11: f64 = *var_qjungat2nd_dn11_slot;
        let mut var_qjungat2nd_dn2: f64 = *var_qjungat2nd_dn2_slot;
        let mut var_qjungat2nd_dn3: f64 = *var_qjungat2nd_dn3_slot;
        let mut var_qjungat2nd_dn4: f64 = *var_qjungat2nd_dn4_slot;
        let mut var_qjungat2nd_dn5: f64 = *var_qjungat2nd_dn5_slot;
        let mut var_qjungat2nd_dn6: f64 = *var_qjungat2nd_dn6_slot;
        let mut var_qjungat2nd_dn7: f64 = *var_qjungat2nd_dn7_slot;
        let mut var_qjungat2nd_dn8: f64 = *var_qjungat2nd_dn8_slot;
        let mut var_qjungat2nd_dn9: f64 = *var_qjungat2nd_dn9_slot;
        let mut var_qjungat_d: f64 = *var_qjungat_d_slot;
        let mut var_qjungat_d_db0: f64 = *var_qjungat_d_db0_slot;
        let mut var_qjungat_d_db1: f64 = *var_qjungat_d_db1_slot;
        let mut var_qjungat_d_db2: f64 = *var_qjungat_d_db2_slot;
        let mut var_qjungat_d_db3: f64 = *var_qjungat_d_db3_slot;
        let mut var_qjungat_d_db4: f64 = *var_qjungat_d_db4_slot;
        let mut var_qjungat_d_db5: f64 = *var_qjungat_d_db5_slot;
        let mut var_qjungat_d_db6: f64 = *var_qjungat_d_db6_slot;
        let mut var_qjungat_d_dn0: f64 = *var_qjungat_d_dn0_slot;
        let mut var_qjungat_d_dn1: f64 = *var_qjungat_d_dn1_slot;
        let mut var_qjungat_d_dn10: f64 = *var_qjungat_d_dn10_slot;
        let mut var_qjungat_d_dn11: f64 = *var_qjungat_d_dn11_slot;
        let mut var_qjungat_d_dn2: f64 = *var_qjungat_d_dn2_slot;
        let mut var_qjungat_d_dn3: f64 = *var_qjungat_d_dn3_slot;
        let mut var_qjungat_d_dn4: f64 = *var_qjungat_d_dn4_slot;
        let mut var_qjungat_d_dn5: f64 = *var_qjungat_d_dn5_slot;
        let mut var_qjungat_d_dn6: f64 = *var_qjungat_d_dn6_slot;
        let mut var_qjungat_d_dn7: f64 = *var_qjungat_d_dn7_slot;
        let mut var_qjungat_d_dn8: f64 = *var_qjungat_d_dn8_slot;
        let mut var_qjungat_d_dn9: f64 = *var_qjungat_d_dn9_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_db0: f64 = *var_qs_db0_slot;
        let mut var_qs_db1: f64 = *var_qs_db1_slot;
        let mut var_qs_db2: f64 = *var_qs_db2_slot;
        let mut var_qs_db3: f64 = *var_qs_db3_slot;
        let mut var_qs_db4: f64 = *var_qs_db4_slot;
        let mut var_qs_db5: f64 = *var_qs_db5_slot;
        let mut var_qs_db6: f64 = *var_qs_db6_slot;
        let mut var_qs_dn0: f64 = *var_qs_dn0_slot;
        let mut var_qs_dn1: f64 = *var_qs_dn1_slot;
        let mut var_qs_dn10: f64 = *var_qs_dn10_slot;
        let mut var_qs_dn11: f64 = *var_qs_dn11_slot;
        let mut var_qs_dn2: f64 = *var_qs_dn2_slot;
        let mut var_qs_dn3: f64 = *var_qs_dn3_slot;
        let mut var_qs_dn4: f64 = *var_qs_dn4_slot;
        let mut var_qs_dn5: f64 = *var_qs_dn5_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_qs_dn9: f64 = *var_qs_dn9_slot;
        let mut var_tmp__blk1543: f64 = *var_tmp__blk1543_slot;
        let mut var_tmp__blk1543_db0: f64 = *var_tmp__blk1543_db0_slot;
        let mut var_tmp__blk1543_db1: f64 = *var_tmp__blk1543_db1_slot;
        let mut var_tmp__blk1543_db2: f64 = *var_tmp__blk1543_db2_slot;
        let mut var_tmp__blk1543_db3: f64 = *var_tmp__blk1543_db3_slot;
        let mut var_tmp__blk1543_db4: f64 = *var_tmp__blk1543_db4_slot;
        let mut var_tmp__blk1543_db5: f64 = *var_tmp__blk1543_db5_slot;
        let mut var_tmp__blk1543_db6: f64 = *var_tmp__blk1543_db6_slot;
        let mut var_tmp__blk1543_dn0: f64 = *var_tmp__blk1543_dn0_slot;
        let mut var_tmp__blk1543_dn1: f64 = *var_tmp__blk1543_dn1_slot;
        let mut var_tmp__blk1543_dn10: f64 = *var_tmp__blk1543_dn10_slot;
        let mut var_tmp__blk1543_dn11: f64 = *var_tmp__blk1543_dn11_slot;
        let mut var_tmp__blk1543_dn2: f64 = *var_tmp__blk1543_dn2_slot;
        let mut var_tmp__blk1543_dn3: f64 = *var_tmp__blk1543_dn3_slot;
        let mut var_tmp__blk1543_dn4: f64 = *var_tmp__blk1543_dn4_slot;
        let mut var_tmp__blk1543_dn5: f64 = *var_tmp__blk1543_dn5_slot;
        let mut var_tmp__blk1543_dn6: f64 = *var_tmp__blk1543_dn6_slot;
        let mut var_tmp__blk1543_dn7: f64 = *var_tmp__blk1543_dn7_slot;
        let mut var_tmp__blk1543_dn8: f64 = *var_tmp__blk1543_dn8_slot;
        let mut var_tmp__blk1543_dn9: f64 = *var_tmp__blk1543_dn9_slot;
        let mut var_vjtmp: f64 = *var_vjtmp_slot;
        let mut var_vjtmp_db0: f64 = *var_vjtmp_db0_slot;
        let mut var_vjtmp_db1: f64 = *var_vjtmp_db1_slot;
        let mut var_vjtmp_db2: f64 = *var_vjtmp_db2_slot;
        let mut var_vjtmp_db3: f64 = *var_vjtmp_db3_slot;
        let mut var_vjtmp_db4: f64 = *var_vjtmp_db4_slot;
        let mut var_vjtmp_db5: f64 = *var_vjtmp_db5_slot;
        let mut var_vjtmp_db6: f64 = *var_vjtmp_db6_slot;
        let mut var_vjtmp_dn0: f64 = *var_vjtmp_dn0_slot;
        let mut var_vjtmp_dn1: f64 = *var_vjtmp_dn1_slot;
        let mut var_vjtmp_dn10: f64 = *var_vjtmp_dn10_slot;
        let mut var_vjtmp_dn11: f64 = *var_vjtmp_dn11_slot;
        let mut var_vjtmp_dn2: f64 = *var_vjtmp_dn2_slot;
        let mut var_vjtmp_dn3: f64 = *var_vjtmp_dn3_slot;
        let mut var_vjtmp_dn4: f64 = *var_vjtmp_dn4_slot;
        let mut var_vjtmp_dn5: f64 = *var_vjtmp_dn5_slot;
        let mut var_vjtmp_dn6: f64 = *var_vjtmp_dn6_slot;
        let mut var_vjtmp_dn7: f64 = *var_vjtmp_dn7_slot;
        let mut var_vjtmp_dn8: f64 = *var_vjtmp_dn8_slot;
        let mut var_vjtmp_dn9: f64 = *var_vjtmp_dn9_slot;

        let (assign61640_e80206, assign61640_e80206_d_n0, assign61640_e80206_d_n1, assign61640_e80206_d_n2, assign61640_e80206_d_n3, assign61640_e80206_d_n4, assign61640_e80206_d_n5, assign61640_e80206_d_n6, assign61640_e80206_d_n7, assign61640_e80206_d_n8, assign61640_e80206_d_n9, assign61640_e80206_d_n10, assign61640_e80206_d_n11, assign61640_e80206_d_b0, assign61640_e80206_d_b1, assign61640_e80206_d_b2, assign61640_e80206_d_b3, assign61640_e80206_d_b4, assign61640_e80206_d_b5, assign61640_e80206_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61640_e80202: f64 = (var_vjun_d + var_vtrgatd_i);
        let assign61640_e80204: f64 = (assign61640_e80202 - var_nu__blk1570);
        (assign61640_e80204, (var_vjun_d_dn0 - var_nu__blk1570_dn0), (var_vjun_d_dn1 - var_nu__blk1570_dn1), (var_vjun_d_dn2 - var_nu__blk1570_dn2), (var_vjun_d_dn3 - var_nu__blk1570_dn3), (var_vjun_d_dn4 - var_nu__blk1570_dn4), (var_vjun_d_dn5 - var_nu__blk1570_dn5), (var_vjun_d_dn6 - var_nu__blk1570_dn6), (var_vjun_d_dn7 - var_nu__blk1570_dn7), (var_vjun_d_dn8 - var_nu__blk1570_dn8), (var_vjun_d_dn9 - var_nu__blk1570_dn9), (var_vjun_d_dn10 - var_nu__blk1570_dn10), (var_vjun_d_dn11 - var_nu__blk1570_dn11), (var_vjun_d_db0 - var_nu__blk1570_db0), (var_vjun_d_db1 - var_nu__blk1570_db1), (var_vjun_d_db2 - var_nu__blk1570_db2), (var_vjun_d_db3 - var_nu__blk1570_db3), (var_vjun_d_db4 - var_nu__blk1570_db4), (var_vjun_d_db5 - var_nu__blk1570_db5), (var_vjun_d_db6 - var_nu__blk1570_db6),)
    } else {
        (var_nu__blk1570, var_nu__blk1570_dn0, var_nu__blk1570_dn1, var_nu__blk1570_dn2, var_nu__blk1570_dn3, var_nu__blk1570_dn4, var_nu__blk1570_dn5, var_nu__blk1570_dn6, var_nu__blk1570_dn7, var_nu__blk1570_dn8, var_nu__blk1570_dn9, var_nu__blk1570_dn10, var_nu__blk1570_dn11, var_nu__blk1570_db0, var_nu__blk1570_db1, var_nu__blk1570_db2, var_nu__blk1570_db3, var_nu__blk1570_db4, var_nu__blk1570_db5, var_nu__blk1570_db6,)
    }
};
        var_nu__blk1570 = assign61640_e80206;
        var_nu__blk1570_dn0 = assign61640_e80206_d_n0;
        var_nu__blk1570_dn1 = assign61640_e80206_d_n1;
        var_nu__blk1570_dn2 = assign61640_e80206_d_n2;
        var_nu__blk1570_dn3 = assign61640_e80206_d_n3;
        var_nu__blk1570_dn4 = assign61640_e80206_d_n4;
        var_nu__blk1570_dn5 = assign61640_e80206_d_n5;
        var_nu__blk1570_dn6 = assign61640_e80206_d_n6;
        var_nu__blk1570_dn7 = assign61640_e80206_d_n7;
        var_nu__blk1570_dn8 = assign61640_e80206_d_n8;
        var_nu__blk1570_dn9 = assign61640_e80206_d_n9;
        var_nu__blk1570_dn10 = assign61640_e80206_d_n10;
        var_nu__blk1570_dn11 = assign61640_e80206_d_n11;
        var_nu__blk1570_db0 = assign61640_e80206_d_b0;
        var_nu__blk1570_db1 = assign61640_e80206_d_b1;
        var_nu__blk1570_db2 = assign61640_e80206_d_b2;
        var_nu__blk1570_db3 = assign61640_e80206_d_b3;
        var_nu__blk1570_db4 = assign61640_e80206_d_b4;
        var_nu__blk1570_db5 = assign61640_e80206_d_b5;
        var_nu__blk1570_db6 = assign61640_e80206_d_b6;

        let (assign61650_e80222,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61650_e80218: f64 = (4.0 * var_vch_d);
        let assign61650_e80220: f64 = (assign61650_e80218 * var_vch_d);
        (assign61650_e80220,)
    } else {
        (var_h1__blk1528,)
    }
};
        var_h1__blk1528 = assign61650_e80222;

        let (assign61660_e80236,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61660_e80234: f64 = (var_vch_d / var_vfmin_d);
        (assign61660_e80234,)
    } else {
        (var_h2__blk1529,)
    }
};
        var_h2__blk1529 = assign61660_e80236;

        let (assign61670_e80252, assign61670_e80252_d_n0, assign61670_e80252_d_n1, assign61670_e80252_d_n2, assign61670_e80252_d_n3, assign61670_e80252_d_n4, assign61670_e80252_d_n5, assign61670_e80252_d_n6, assign61670_e80252_d_n7, assign61670_e80252_d_n8, assign61670_e80252_d_n9, assign61670_e80252_d_n10, assign61670_e80252_d_n11, assign61670_e80252_d_b0, assign61670_e80252_d_b1, assign61670_e80252_d_b2, assign61670_e80252_d_b3, assign61670_e80252_d_b4, assign61670_e80252_d_b5, assign61670_e80252_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61670_e80249: f64 = (var_vch_d * var_h2__blk1529);
        let assign61670_e80250: f64 = (var_nu__blk1570 + assign61670_e80249);
        (assign61670_e80250, var_nu__blk1570_dn0, var_nu__blk1570_dn1, var_nu__blk1570_dn2, var_nu__blk1570_dn3, var_nu__blk1570_dn4, var_nu__blk1570_dn5, var_nu__blk1570_dn6, var_nu__blk1570_dn7, var_nu__blk1570_dn8, var_nu__blk1570_dn9, var_nu__blk1570_dn10, var_nu__blk1570_dn11, var_nu__blk1570_db0, var_nu__blk1570_db1, var_nu__blk1570_db2, var_nu__blk1570_db3, var_nu__blk1570_db4, var_nu__blk1570_db5, var_nu__blk1570_db6,)
    } else {
        (var_h2d__blk1530, var_h2d__blk1530_dn0, var_h2d__blk1530_dn1, var_h2d__blk1530_dn2, var_h2d__blk1530_dn3, var_h2d__blk1530_dn4, var_h2d__blk1530_dn5, var_h2d__blk1530_dn6, var_h2d__blk1530_dn7, var_h2d__blk1530_dn8, var_h2d__blk1530_dn9, var_h2d__blk1530_dn10, var_h2d__blk1530_dn11, var_h2d__blk1530_db0, var_h2d__blk1530_db1, var_h2d__blk1530_db2, var_h2d__blk1530_db3, var_h2d__blk1530_db4, var_h2d__blk1530_db5, var_h2d__blk1530_db6,)
    }
};
        var_h2d__blk1530 = assign61670_e80252;
        var_h2d__blk1530_dn0 = assign61670_e80252_d_n0;
        var_h2d__blk1530_dn1 = assign61670_e80252_d_n1;
        var_h2d__blk1530_dn2 = assign61670_e80252_d_n2;
        var_h2d__blk1530_dn3 = assign61670_e80252_d_n3;
        var_h2d__blk1530_dn4 = assign61670_e80252_d_n4;
        var_h2d__blk1530_dn5 = assign61670_e80252_d_n5;
        var_h2d__blk1530_dn6 = assign61670_e80252_d_n6;
        var_h2d__blk1530_dn7 = assign61670_e80252_d_n7;
        var_h2d__blk1530_dn8 = assign61670_e80252_d_n8;
        var_h2d__blk1530_dn9 = assign61670_e80252_d_n9;
        var_h2d__blk1530_dn10 = assign61670_e80252_d_n10;
        var_h2d__blk1530_dn11 = assign61670_e80252_d_n11;
        var_h2d__blk1530_db0 = assign61670_e80252_d_b0;
        var_h2d__blk1530_db1 = assign61670_e80252_d_b1;
        var_h2d__blk1530_db2 = assign61670_e80252_d_b2;
        var_h2d__blk1530_db3 = assign61670_e80252_d_b3;
        var_h2d__blk1530_db4 = assign61670_e80252_d_b4;
        var_h2d__blk1530_db5 = assign61670_e80252_d_b5;
        var_h2d__blk1530_db6 = assign61670_e80252_d_b6;

        let (assign61680_e80266, assign61680_e80266_d_n0, assign61680_e80266_d_n1, assign61680_e80266_d_n2, assign61680_e80266_d_n3, assign61680_e80266_d_n4, assign61680_e80266_d_n5, assign61680_e80266_d_n6, assign61680_e80266_d_n7, assign61680_e80266_d_n8, assign61680_e80266_d_n9, assign61680_e80266_d_n10, assign61680_e80266_d_n11, assign61680_e80266_d_b0, assign61680_e80266_d_b1, assign61680_e80266_d_b2, assign61680_e80266_d_b3, assign61680_e80266_d_b4, assign61680_e80266_d_b5, assign61680_e80266_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61680_e80264: f64 = (var_vfmin_d + var_h2d__blk1530);
        (assign61680_e80264, var_h2d__blk1530_dn0, var_h2d__blk1530_dn1, var_h2d__blk1530_dn2, var_h2d__blk1530_dn3, var_h2d__blk1530_dn4, var_h2d__blk1530_dn5, var_h2d__blk1530_dn6, var_h2d__blk1530_dn7, var_h2d__blk1530_dn8, var_h2d__blk1530_dn9, var_h2d__blk1530_dn10, var_h2d__blk1530_dn11, var_h2d__blk1530_db0, var_h2d__blk1530_db1, var_h2d__blk1530_db2, var_h2d__blk1530_db3, var_h2d__blk1530_db4, var_h2d__blk1530_db5, var_h2d__blk1530_db6,)
    } else {
        (var_h3__blk1531, var_h3__blk1531_dn0, var_h3__blk1531_dn1, var_h3__blk1531_dn2, var_h3__blk1531_dn3, var_h3__blk1531_dn4, var_h3__blk1531_dn5, var_h3__blk1531_dn6, var_h3__blk1531_dn7, var_h3__blk1531_dn8, var_h3__blk1531_dn9, var_h3__blk1531_dn10, var_h3__blk1531_dn11, var_h3__blk1531_db0, var_h3__blk1531_db1, var_h3__blk1531_db2, var_h3__blk1531_db3, var_h3__blk1531_db4, var_h3__blk1531_db5, var_h3__blk1531_db6,)
    }
};
        var_h3__blk1531 = assign61680_e80266;
        var_h3__blk1531_dn0 = assign61680_e80266_d_n0;
        var_h3__blk1531_dn1 = assign61680_e80266_d_n1;
        var_h3__blk1531_dn2 = assign61680_e80266_d_n2;
        var_h3__blk1531_dn3 = assign61680_e80266_d_n3;
        var_h3__blk1531_dn4 = assign61680_e80266_d_n4;
        var_h3__blk1531_dn5 = assign61680_e80266_d_n5;
        var_h3__blk1531_dn6 = assign61680_e80266_d_n6;
        var_h3__blk1531_dn7 = assign61680_e80266_d_n7;
        var_h3__blk1531_dn8 = assign61680_e80266_d_n8;
        var_h3__blk1531_dn9 = assign61680_e80266_d_n9;
        var_h3__blk1531_dn10 = assign61680_e80266_d_n10;
        var_h3__blk1531_dn11 = assign61680_e80266_d_n11;
        var_h3__blk1531_db0 = assign61680_e80266_d_b0;
        var_h3__blk1531_db1 = assign61680_e80266_d_b1;
        var_h3__blk1531_db2 = assign61680_e80266_d_b2;
        var_h3__blk1531_db3 = assign61680_e80266_d_b3;
        var_h3__blk1531_db4 = assign61680_e80266_d_b4;
        var_h3__blk1531_db5 = assign61680_e80266_d_b5;
        var_h3__blk1531_db6 = assign61680_e80266_d_b6;

        let (assign61690_e80280, assign61690_e80280_d_n0, assign61690_e80280_d_n1, assign61690_e80280_d_n2, assign61690_e80280_d_n3, assign61690_e80280_d_n4, assign61690_e80280_d_n5, assign61690_e80280_d_n6, assign61690_e80280_d_n7, assign61690_e80280_d_n8, assign61690_e80280_d_n9, assign61690_e80280_d_n10, assign61690_e80280_d_n11, assign61690_e80280_d_b0, assign61690_e80280_d_b1, assign61690_e80280_d_b2, assign61690_e80280_d_b3, assign61690_e80280_d_b4, assign61690_e80280_d_b5, assign61690_e80280_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61690_e80278: f64 = (var_vfmin_d - var_h2d__blk1530);
        (assign61690_e80278, (-var_h2d__blk1530_dn0), (-var_h2d__blk1530_dn1), (-var_h2d__blk1530_dn2), (-var_h2d__blk1530_dn3), (-var_h2d__blk1530_dn4), (-var_h2d__blk1530_dn5), (-var_h2d__blk1530_dn6), (-var_h2d__blk1530_dn7), (-var_h2d__blk1530_dn8), (-var_h2d__blk1530_dn9), (-var_h2d__blk1530_dn10), (-var_h2d__blk1530_dn11), (-var_h2d__blk1530_db0), (-var_h2d__blk1530_db1), (-var_h2d__blk1530_db2), (-var_h2d__blk1530_db3), (-var_h2d__blk1530_db4), (-var_h2d__blk1530_db5), (-var_h2d__blk1530_db6),)
    } else {
        (var_h4__blk1532, var_h4__blk1532_dn0, var_h4__blk1532_dn1, var_h4__blk1532_dn2, var_h4__blk1532_dn3, var_h4__blk1532_dn4, var_h4__blk1532_dn5, var_h4__blk1532_dn6, var_h4__blk1532_dn7, var_h4__blk1532_dn8, var_h4__blk1532_dn9, var_h4__blk1532_dn10, var_h4__blk1532_dn11, var_h4__blk1532_db0, var_h4__blk1532_db1, var_h4__blk1532_db2, var_h4__blk1532_db3, var_h4__blk1532_db4, var_h4__blk1532_db5, var_h4__blk1532_db6,)
    }
};
        var_h4__blk1532 = assign61690_e80280;
        var_h4__blk1532_dn0 = assign61690_e80280_d_n0;
        var_h4__blk1532_dn1 = assign61690_e80280_d_n1;
        var_h4__blk1532_dn2 = assign61690_e80280_d_n2;
        var_h4__blk1532_dn3 = assign61690_e80280_d_n3;
        var_h4__blk1532_dn4 = assign61690_e80280_d_n4;
        var_h4__blk1532_dn5 = assign61690_e80280_d_n5;
        var_h4__blk1532_dn6 = assign61690_e80280_d_n6;
        var_h4__blk1532_dn7 = assign61690_e80280_d_n7;
        var_h4__blk1532_dn8 = assign61690_e80280_d_n8;
        var_h4__blk1532_dn9 = assign61690_e80280_d_n9;
        var_h4__blk1532_dn10 = assign61690_e80280_d_n10;
        var_h4__blk1532_dn11 = assign61690_e80280_d_n11;
        var_h4__blk1532_db0 = assign61690_e80280_d_b0;
        var_h4__blk1532_db1 = assign61690_e80280_d_b1;
        var_h4__blk1532_db2 = assign61690_e80280_d_b2;
        var_h4__blk1532_db3 = assign61690_e80280_d_b3;
        var_h4__blk1532_db4 = assign61690_e80280_d_b4;
        var_h4__blk1532_db5 = assign61690_e80280_d_b5;
        var_h4__blk1532_db6 = assign61690_e80280_d_b6;

        let (assign61700_e80297, assign61700_e80297_d_n0, assign61700_e80297_d_n1, assign61700_e80297_d_n2, assign61700_e80297_d_n3, assign61700_e80297_d_n4, assign61700_e80297_d_n5, assign61700_e80297_d_n6, assign61700_e80297_d_n7, assign61700_e80297_d_n8, assign61700_e80297_d_n9, assign61700_e80297_d_n10, assign61700_e80297_d_n11, assign61700_e80297_d_b0, assign61700_e80297_d_b1, assign61700_e80297_d_b2, assign61700_e80297_d_b3, assign61700_e80297_d_b4, assign61700_e80297_d_b5, assign61700_e80297_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61700_e80292: f64 = (var_h4__blk1532 * var_h4__blk1532);
        let assign61700_e80294: f64 = (assign61700_e80292 + var_h1__blk1528);
        let assign61700_e80295: f64 = (assign61700_e80294).sqrt();
        (assign61700_e80295, (((var_h4__blk1532_dn0 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn0)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_dn1 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn1)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_dn2 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn2)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_dn3 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn3)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_dn4 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn4)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_dn5 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn5)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_dn6 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn6)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_dn7 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn7)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_dn8 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn8)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_dn9 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn9)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_dn10 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn10)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_dn11 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn11)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_db0 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_db0)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_db1 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_db1)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_db2 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_db2)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_db3 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_db3)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_db4 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_db4)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_db5 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_db5)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_db6 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_db6)) / (2.0 * assign61700_e80295)),)
    } else {
        (var_h5__blk1533, var_h5__blk1533_dn0, var_h5__blk1533_dn1, var_h5__blk1533_dn2, var_h5__blk1533_dn3, var_h5__blk1533_dn4, var_h5__blk1533_dn5, var_h5__blk1533_dn6, var_h5__blk1533_dn7, var_h5__blk1533_dn8, var_h5__blk1533_dn9, var_h5__blk1533_dn10, var_h5__blk1533_dn11, var_h5__blk1533_db0, var_h5__blk1533_db1, var_h5__blk1533_db2, var_h5__blk1533_db3, var_h5__blk1533_db4, var_h5__blk1533_db5, var_h5__blk1533_db6,)
    }
};
        var_h5__blk1533 = assign61700_e80297;
        var_h5__blk1533_dn0 = assign61700_e80297_d_n0;
        var_h5__blk1533_dn1 = assign61700_e80297_d_n1;
        var_h5__blk1533_dn2 = assign61700_e80297_d_n2;
        var_h5__blk1533_dn3 = assign61700_e80297_d_n3;
        var_h5__blk1533_dn4 = assign61700_e80297_d_n4;
        var_h5__blk1533_dn5 = assign61700_e80297_d_n5;
        var_h5__blk1533_dn6 = assign61700_e80297_d_n6;
        var_h5__blk1533_dn7 = assign61700_e80297_d_n7;
        var_h5__blk1533_dn8 = assign61700_e80297_d_n8;
        var_h5__blk1533_dn9 = assign61700_e80297_d_n9;
        var_h5__blk1533_dn10 = assign61700_e80297_d_n10;
        var_h5__blk1533_dn11 = assign61700_e80297_d_n11;
        var_h5__blk1533_db0 = assign61700_e80297_d_b0;
        var_h5__blk1533_db1 = assign61700_e80297_d_b1;
        var_h5__blk1533_db2 = assign61700_e80297_d_b2;
        var_h5__blk1533_db3 = assign61700_e80297_d_b3;
        var_h5__blk1533_db4 = assign61700_e80297_d_b4;
        var_h5__blk1533_db5 = assign61700_e80297_d_b5;
        var_h5__blk1533_db6 = assign61700_e80297_d_b6;

        let (assign61710_e80317, assign61710_e80317_d_n0, assign61710_e80317_d_n1, assign61710_e80317_d_n2, assign61710_e80317_d_n3, assign61710_e80317_d_n4, assign61710_e80317_d_n5, assign61710_e80317_d_n6, assign61710_e80317_d_n7, assign61710_e80317_d_n8, assign61710_e80317_d_n9, assign61710_e80317_d_n10, assign61710_e80317_d_n11, assign61710_e80317_d_b0, assign61710_e80317_d_b1, assign61710_e80317_d_b2, assign61710_e80317_d_b3, assign61710_e80317_d_b4, assign61710_e80317_d_b5, assign61710_e80317_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61710_e80310: f64 = (var_nu__blk1570 * var_vfmin_d);
        let assign61710_e80313: f64 = (var_h3__blk1531 + var_h5__blk1533);
        let assign61710_e80314: f64 = (assign61710_e80310 / assign61710_e80313);
        let assign61710_e80315: f64 = (2.0 * assign61710_e80314);
        (assign61710_e80315, (2.0 * ((((var_nu__blk1570_dn0 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn0 + var_h5__blk1533_dn0))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_dn1 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn1 + var_h5__blk1533_dn1))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_dn2 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn2 + var_h5__blk1533_dn2))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_dn3 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn3 + var_h5__blk1533_dn3))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_dn4 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn4 + var_h5__blk1533_dn4))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_dn5 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn5 + var_h5__blk1533_dn5))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_dn6 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn6 + var_h5__blk1533_dn6))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_dn7 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn7 + var_h5__blk1533_dn7))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_dn8 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn8 + var_h5__blk1533_dn8))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_dn9 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn9 + var_h5__blk1533_dn9))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_dn10 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn10 + var_h5__blk1533_dn10))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_dn11 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn11 + var_h5__blk1533_dn11))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_db0 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_db0 + var_h5__blk1533_db0))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_db1 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_db1 + var_h5__blk1533_db1))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_db2 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_db2 + var_h5__blk1533_db2))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_db3 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_db3 + var_h5__blk1533_db3))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_db4 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_db4 + var_h5__blk1533_db4))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_db5 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_db5 + var_h5__blk1533_db5))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_db6 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_db6 + var_h5__blk1533_db6))) / (assign61710_e80313 * assign61710_e80313))),)
    } else {
        (var_vjtmp, var_vjtmp_dn0, var_vjtmp_dn1, var_vjtmp_dn2, var_vjtmp_dn3, var_vjtmp_dn4, var_vjtmp_dn5, var_vjtmp_dn6, var_vjtmp_dn7, var_vjtmp_dn8, var_vjtmp_dn9, var_vjtmp_dn10, var_vjtmp_dn11, var_vjtmp_db0, var_vjtmp_db1, var_vjtmp_db2, var_vjtmp_db3, var_vjtmp_db4, var_vjtmp_db5, var_vjtmp_db6,)
    }
};
        var_vjtmp = assign61710_e80317;
        var_vjtmp_dn0 = assign61710_e80317_d_n0;
        var_vjtmp_dn1 = assign61710_e80317_d_n1;
        var_vjtmp_dn2 = assign61710_e80317_d_n2;
        var_vjtmp_dn3 = assign61710_e80317_d_n3;
        var_vjtmp_dn4 = assign61710_e80317_d_n4;
        var_vjtmp_dn5 = assign61710_e80317_d_n5;
        var_vjtmp_dn6 = assign61710_e80317_d_n6;
        var_vjtmp_dn7 = assign61710_e80317_d_n7;
        var_vjtmp_dn8 = assign61710_e80317_d_n8;
        var_vjtmp_dn9 = assign61710_e80317_d_n9;
        var_vjtmp_dn10 = assign61710_e80317_d_n10;
        var_vjtmp_dn11 = assign61710_e80317_d_n11;
        var_vjtmp_db0 = assign61710_e80317_d_b0;
        var_vjtmp_db1 = assign61710_e80317_d_b1;
        var_vjtmp_db2 = assign61710_e80317_d_b2;
        var_vjtmp_db3 = assign61710_e80317_d_b3;
        var_vjtmp_db4 = assign61710_e80317_d_b4;
        var_vjtmp_db5 = assign61710_e80317_d_b5;
        var_vjtmp_db6 = assign61710_e80317_d_b6;

        let assign61720_e80320: f64 = if var_one_minus_pgat2nd_d == 0.5 { 1.0 } else { 0.0 };
        var_guard1716 = assign61720_e80320;

        let (assign61730_e80339, assign61730_e80339_d_n0, assign61730_e80339_d_n1, assign61730_e80339_d_n2, assign61730_e80339_d_n3, assign61730_e80339_d_n4, assign61730_e80339_d_n5, assign61730_e80339_d_n6, assign61730_e80339_d_n7, assign61730_e80339_d_n8, assign61730_e80339_d_n9, assign61730_e80339_d_n10, assign61730_e80339_d_n11, assign61730_e80339_d_b0, assign61730_e80339_d_b1, assign61730_e80339_d_b2, assign61730_e80339_d_b3, assign61730_e80339_d_b4, assign61730_e80339_d_b5, assign61730_e80339_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) && (var_guard1716 != 0.0)) {
        let assign61730_e80335: f64 = (var_vjtmp * var_vbiinvgat2nd_d);
        let assign61730_e80336: f64 = (1.0 - assign61730_e80335);
        let assign61730_e80337: f64 = (assign61730_e80336).sqrt();
        (assign61730_e80337, ((-(var_vjtmp_dn0 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_dn1 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_dn2 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_dn3 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_dn4 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_dn5 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_dn6 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_dn7 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_dn8 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_dn9 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_dn10 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_dn11 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_db0 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_db1 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_db2 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_db3 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_db4 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_db5 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_db6 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)),)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61730_e80339;
        var_tmp__blk1543_dn0 = assign61730_e80339_d_n0;
        var_tmp__blk1543_dn1 = assign61730_e80339_d_n1;
        var_tmp__blk1543_dn2 = assign61730_e80339_d_n2;
        var_tmp__blk1543_dn3 = assign61730_e80339_d_n3;
        var_tmp__blk1543_dn4 = assign61730_e80339_d_n4;
        var_tmp__blk1543_dn5 = assign61730_e80339_d_n5;
        var_tmp__blk1543_dn6 = assign61730_e80339_d_n6;
        var_tmp__blk1543_dn7 = assign61730_e80339_d_n7;
        var_tmp__blk1543_dn8 = assign61730_e80339_d_n8;
        var_tmp__blk1543_dn9 = assign61730_e80339_d_n9;
        var_tmp__blk1543_dn10 = assign61730_e80339_d_n10;
        var_tmp__blk1543_dn11 = assign61730_e80339_d_n11;
        var_tmp__blk1543_db0 = assign61730_e80339_d_b0;
        var_tmp__blk1543_db1 = assign61730_e80339_d_b1;
        var_tmp__blk1543_db2 = assign61730_e80339_d_b2;
        var_tmp__blk1543_db3 = assign61730_e80339_d_b3;
        var_tmp__blk1543_db4 = assign61730_e80339_d_b4;
        var_tmp__blk1543_db5 = assign61730_e80339_d_b5;
        var_tmp__blk1543_db6 = assign61730_e80339_d_b6;

        let (assign61740_e80360, assign61740_e80360_d_n0, assign61740_e80360_d_n1, assign61740_e80360_d_n2, assign61740_e80360_d_n3, assign61740_e80360_d_n4, assign61740_e80360_d_n5, assign61740_e80360_d_n6, assign61740_e80360_d_n7, assign61740_e80360_d_n8, assign61740_e80360_d_n9, assign61740_e80360_d_n10, assign61740_e80360_d_n11, assign61740_e80360_d_b0, assign61740_e80360_d_b1, assign61740_e80360_d_b2, assign61740_e80360_d_b3, assign61740_e80360_d_b4, assign61740_e80360_d_b5, assign61740_e80360_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) && (var_guard1716 == 0.0)) {
        let assign61740_e80355: f64 = (var_vjtmp * var_vbiinvgat2nd_d);
        let assign61740_e80356: f64 = (1.0 - assign61740_e80355);
        let assign61740_e80358: f64 = (assign61740_e80356).powf(var_one_minus_pgat2nd_d);
        (assign61740_e80358, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn0 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn0 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn1 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn1 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn2 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn2 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn3 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn3 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn4 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn4 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn5 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn5 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn6 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn6 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn7 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn7 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn8 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn8 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn9 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn9 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn10 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn10 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn11 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn11 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_db0 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_db0 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_db1 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_db1 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_db2 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_db2 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_db3 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_db3 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_db4 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_db4 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_db5 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_db5 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_db6 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_db6 * var_vbiinvgat2nd_d)) / assign61740_e80356))) },)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61740_e80360;
        var_tmp__blk1543_dn0 = assign61740_e80360_d_n0;
        var_tmp__blk1543_dn1 = assign61740_e80360_d_n1;
        var_tmp__blk1543_dn2 = assign61740_e80360_d_n2;
        var_tmp__blk1543_dn3 = assign61740_e80360_d_n3;
        var_tmp__blk1543_dn4 = assign61740_e80360_d_n4;
        var_tmp__blk1543_dn5 = assign61740_e80360_d_n5;
        var_tmp__blk1543_dn6 = assign61740_e80360_d_n6;
        var_tmp__blk1543_dn7 = assign61740_e80360_d_n7;
        var_tmp__blk1543_dn8 = assign61740_e80360_d_n8;
        var_tmp__blk1543_dn9 = assign61740_e80360_d_n9;
        var_tmp__blk1543_dn10 = assign61740_e80360_d_n10;
        var_tmp__blk1543_dn11 = assign61740_e80360_d_n11;
        var_tmp__blk1543_db0 = assign61740_e80360_d_b0;
        var_tmp__blk1543_db1 = assign61740_e80360_d_b1;
        var_tmp__blk1543_db2 = assign61740_e80360_d_b2;
        var_tmp__blk1543_db3 = assign61740_e80360_d_b3;
        var_tmp__blk1543_db4 = assign61740_e80360_d_b4;
        var_tmp__blk1543_db5 = assign61740_e80360_d_b5;
        var_tmp__blk1543_db6 = assign61740_e80360_d_b6;

        let (assign61750_e80384, assign61750_e80384_d_n0, assign61750_e80384_d_n1, assign61750_e80384_d_n2, assign61750_e80384_d_n3, assign61750_e80384_d_n4, assign61750_e80384_d_n5, assign61750_e80384_d_n6, assign61750_e80384_d_n7, assign61750_e80384_d_n8, assign61750_e80384_d_n9, assign61750_e80384_d_n10, assign61750_e80384_d_n11, assign61750_e80384_d_b0, assign61750_e80384_d_b1, assign61750_e80384_d_b2, assign61750_e80384_d_b3, assign61750_e80384_d_b4, assign61750_e80384_d_b5, assign61750_e80384_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61750_e80374: f64 = (1.0 - var_tmp__blk1543);
        let assign61750_e80375: f64 = (var_qprefgat2nd_d * assign61750_e80374);
        let assign61750_e80379: f64 = (var_nu__blk1570 - var_vjtmp);
        let assign61750_e80380: f64 = (var_qpref2gat2nd_d * assign61750_e80379);
        let assign61750_e80381: f64 = (assign61750_e80375 + assign61750_e80380);
        let assign61750_e80382: f64 = (p.p30 * assign61750_e80381);
        (assign61750_e80382, (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn0)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn0 - var_vjtmp_dn0)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn1)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn1 - var_vjtmp_dn1)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn2)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn2 - var_vjtmp_dn2)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn3)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn3 - var_vjtmp_dn3)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn4)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn4 - var_vjtmp_dn4)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn5)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn5 - var_vjtmp_dn5)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn6)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn6 - var_vjtmp_dn6)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn7)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn7 - var_vjtmp_dn7)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn8)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn8 - var_vjtmp_dn8)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn9)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn9 - var_vjtmp_dn9)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn10)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn10 - var_vjtmp_dn10)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn11)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn11 - var_vjtmp_dn11)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_db0)) + (var_qpref2gat2nd_d * (var_nu__blk1570_db0 - var_vjtmp_db0)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_db1)) + (var_qpref2gat2nd_d * (var_nu__blk1570_db1 - var_vjtmp_db1)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_db2)) + (var_qpref2gat2nd_d * (var_nu__blk1570_db2 - var_vjtmp_db2)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_db3)) + (var_qpref2gat2nd_d * (var_nu__blk1570_db3 - var_vjtmp_db3)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_db4)) + (var_qpref2gat2nd_d * (var_nu__blk1570_db4 - var_vjtmp_db4)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_db5)) + (var_qpref2gat2nd_d * (var_nu__blk1570_db5 - var_vjtmp_db5)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_db6)) + (var_qpref2gat2nd_d * (var_nu__blk1570_db6 - var_vjtmp_db6)))),)
    } else {
        (var_qjungat2nd, var_qjungat2nd_dn0, var_qjungat2nd_dn1, var_qjungat2nd_dn2, var_qjungat2nd_dn3, var_qjungat2nd_dn4, var_qjungat2nd_dn5, var_qjungat2nd_dn6, var_qjungat2nd_dn7, var_qjungat2nd_dn8, var_qjungat2nd_dn9, var_qjungat2nd_dn10, var_qjungat2nd_dn11, var_qjungat2nd_db0, var_qjungat2nd_db1, var_qjungat2nd_db2, var_qjungat2nd_db3, var_qjungat2nd_db4, var_qjungat2nd_db5, var_qjungat2nd_db6,)
    }
};
        var_qjungat2nd = assign61750_e80384;
        var_qjungat2nd_dn0 = assign61750_e80384_d_n0;
        var_qjungat2nd_dn1 = assign61750_e80384_d_n1;
        var_qjungat2nd_dn2 = assign61750_e80384_d_n2;
        var_qjungat2nd_dn3 = assign61750_e80384_d_n3;
        var_qjungat2nd_dn4 = assign61750_e80384_d_n4;
        var_qjungat2nd_dn5 = assign61750_e80384_d_n5;
        var_qjungat2nd_dn6 = assign61750_e80384_d_n6;
        var_qjungat2nd_dn7 = assign61750_e80384_d_n7;
        var_qjungat2nd_dn8 = assign61750_e80384_d_n8;
        var_qjungat2nd_dn9 = assign61750_e80384_d_n9;
        var_qjungat2nd_dn10 = assign61750_e80384_d_n10;
        var_qjungat2nd_dn11 = assign61750_e80384_d_n11;
        var_qjungat2nd_db0 = assign61750_e80384_d_b0;
        var_qjungat2nd_db1 = assign61750_e80384_d_b1;
        var_qjungat2nd_db2 = assign61750_e80384_d_b2;
        var_qjungat2nd_db3 = assign61750_e80384_d_b3;
        var_qjungat2nd_db4 = assign61750_e80384_d_b4;
        var_qjungat2nd_db5 = assign61750_e80384_d_b5;
        var_qjungat2nd_db6 = assign61750_e80384_d_b6;

        let (assign61760_e80398, assign61760_e80398_d_n0, assign61760_e80398_d_n1, assign61760_e80398_d_n2, assign61760_e80398_d_n3, assign61760_e80398_d_n4, assign61760_e80398_d_n5, assign61760_e80398_d_n6, assign61760_e80398_d_n7, assign61760_e80398_d_n8, assign61760_e80398_d_n9, assign61760_e80398_d_n10, assign61760_e80398_d_n11, assign61760_e80398_d_b0, assign61760_e80398_d_b1, assign61760_e80398_d_b2, assign61760_e80398_d_b3, assign61760_e80398_d_b4, assign61760_e80398_d_b5, assign61760_e80398_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61760_e80396: f64 = (var_qjungat_d + var_qjungat2nd);
        (assign61760_e80396, (var_qjungat_d_dn0 + var_qjungat2nd_dn0), (var_qjungat_d_dn1 + var_qjungat2nd_dn1), (var_qjungat_d_dn2 + var_qjungat2nd_dn2), (var_qjungat_d_dn3 + var_qjungat2nd_dn3), (var_qjungat_d_dn4 + var_qjungat2nd_dn4), (var_qjungat_d_dn5 + var_qjungat2nd_dn5), (var_qjungat_d_dn6 + var_qjungat2nd_dn6), (var_qjungat_d_dn7 + var_qjungat2nd_dn7), (var_qjungat_d_dn8 + var_qjungat2nd_dn8), (var_qjungat_d_dn9 + var_qjungat2nd_dn9), (var_qjungat_d_dn10 + var_qjungat2nd_dn10), (var_qjungat_d_dn11 + var_qjungat2nd_dn11), (var_qjungat_d_db0 + var_qjungat2nd_db0), (var_qjungat_d_db1 + var_qjungat2nd_db1), (var_qjungat_d_db2 + var_qjungat2nd_db2), (var_qjungat_d_db3 + var_qjungat2nd_db3), (var_qjungat_d_db4 + var_qjungat2nd_db4), (var_qjungat_d_db5 + var_qjungat2nd_db5), (var_qjungat_d_db6 + var_qjungat2nd_db6),)
    } else {
        (var_qjungat_d, var_qjungat_d_dn0, var_qjungat_d_dn1, var_qjungat_d_dn2, var_qjungat_d_dn3, var_qjungat_d_dn4, var_qjungat_d_dn5, var_qjungat_d_dn6, var_qjungat_d_dn7, var_qjungat_d_dn8, var_qjungat_d_dn9, var_qjungat_d_dn10, var_qjungat_d_dn11, var_qjungat_d_db0, var_qjungat_d_db1, var_qjungat_d_db2, var_qjungat_d_db3, var_qjungat_d_db4, var_qjungat_d_db5, var_qjungat_d_db6,)
    }
};
        var_qjungat_d = assign61760_e80398;
        var_qjungat_d_dn0 = assign61760_e80398_d_n0;
        var_qjungat_d_dn1 = assign61760_e80398_d_n1;
        var_qjungat_d_dn2 = assign61760_e80398_d_n2;
        var_qjungat_d_dn3 = assign61760_e80398_d_n3;
        var_qjungat_d_dn4 = assign61760_e80398_d_n4;
        var_qjungat_d_dn5 = assign61760_e80398_d_n5;
        var_qjungat_d_dn6 = assign61760_e80398_d_n6;
        var_qjungat_d_dn7 = assign61760_e80398_d_n7;
        var_qjungat_d_dn8 = assign61760_e80398_d_n8;
        var_qjungat_d_dn9 = assign61760_e80398_d_n9;
        var_qjungat_d_dn10 = assign61760_e80398_d_n10;
        var_qjungat_d_dn11 = assign61760_e80398_d_n11;
        var_qjungat_d_db0 = assign61760_e80398_d_b0;
        var_qjungat_d_db1 = assign61760_e80398_d_b1;
        var_qjungat_d_db2 = assign61760_e80398_d_b2;
        var_qjungat_d_db3 = assign61760_e80398_d_b3;
        var_qjungat_d_db4 = assign61760_e80398_d_b4;
        var_qjungat_d_db5 = assign61760_e80398_d_b5;
        var_qjungat_d_db6 = assign61760_e80398_d_b6;

        let assign61770_e80401: f64 = if var_one_minus_pgat_d == 0.5 { 1.0 } else { 0.0 };
        var_guard1717 = assign61770_e80401;

        let (assign61780_e80421, assign61780_e80421_d_n0, assign61780_e80421_d_n1, assign61780_e80421_d_n2, assign61780_e80421_d_n3, assign61780_e80421_d_n4, assign61780_e80421_d_n5, assign61780_e80421_d_n6, assign61780_e80421_d_n7, assign61780_e80421_d_n8, assign61780_e80421_d_n9, assign61780_e80421_d_n10, assign61780_e80421_d_n11, assign61780_e80421_d_b0, assign61780_e80421_d_b1, assign61780_e80421_d_b2, assign61780_e80421_d_b3, assign61780_e80421_d_b4, assign61780_e80421_d_b5, assign61780_e80421_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1717 != 0.0)) {
        let assign61780_e80417: f64 = (var_vj__blk1535 * var_vbiinvgat_d);
        let assign61780_e80418: f64 = (1.0 - assign61780_e80417);
        let assign61780_e80419: f64 = (assign61780_e80418).sqrt();
        (assign61780_e80419, ((-(var_vj__blk1535_dn0 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_dn1 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_dn2 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_dn3 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_dn4 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_dn5 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_dn6 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_dn7 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_dn8 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_dn9 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_dn10 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_dn11 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_db0 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_db1 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_db2 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_db3 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_db4 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_db5 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_db6 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)),)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61780_e80421;
        var_tmp__blk1543_dn0 = assign61780_e80421_d_n0;
        var_tmp__blk1543_dn1 = assign61780_e80421_d_n1;
        var_tmp__blk1543_dn2 = assign61780_e80421_d_n2;
        var_tmp__blk1543_dn3 = assign61780_e80421_d_n3;
        var_tmp__blk1543_dn4 = assign61780_e80421_d_n4;
        var_tmp__blk1543_dn5 = assign61780_e80421_d_n5;
        var_tmp__blk1543_dn6 = assign61780_e80421_d_n6;
        var_tmp__blk1543_dn7 = assign61780_e80421_d_n7;
        var_tmp__blk1543_dn8 = assign61780_e80421_d_n8;
        var_tmp__blk1543_dn9 = assign61780_e80421_d_n9;
        var_tmp__blk1543_dn10 = assign61780_e80421_d_n10;
        var_tmp__blk1543_dn11 = assign61780_e80421_d_n11;
        var_tmp__blk1543_db0 = assign61780_e80421_d_b0;
        var_tmp__blk1543_db1 = assign61780_e80421_d_b1;
        var_tmp__blk1543_db2 = assign61780_e80421_d_b2;
        var_tmp__blk1543_db3 = assign61780_e80421_d_b3;
        var_tmp__blk1543_db4 = assign61780_e80421_d_b4;
        var_tmp__blk1543_db5 = assign61780_e80421_d_b5;
        var_tmp__blk1543_db6 = assign61780_e80421_d_b6;

        let (assign61790_e80443, assign61790_e80443_d_n0, assign61790_e80443_d_n1, assign61790_e80443_d_n2, assign61790_e80443_d_n3, assign61790_e80443_d_n4, assign61790_e80443_d_n5, assign61790_e80443_d_n6, assign61790_e80443_d_n7, assign61790_e80443_d_n8, assign61790_e80443_d_n9, assign61790_e80443_d_n10, assign61790_e80443_d_n11, assign61790_e80443_d_b0, assign61790_e80443_d_b1, assign61790_e80443_d_b2, assign61790_e80443_d_b3, assign61790_e80443_d_b4, assign61790_e80443_d_b5, assign61790_e80443_d_b6,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1717 == 0.0)) {
        let assign61790_e80438: f64 = (var_vj__blk1535 * var_vbiinvgat_d);
        let assign61790_e80439: f64 = (1.0 - assign61790_e80438);
        let assign61790_e80441: f64 = (assign61790_e80439).powf(var_one_minus_pgat_d);
        (assign61790_e80441, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn0 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn0 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn1 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn1 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn2 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn2 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn3 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn3 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn4 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn4 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn5 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn5 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn6 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn6 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn7 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn7 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn8 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn8 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn9 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn9 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn10 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn10 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn11 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn11 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_db0 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_db0 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_db1 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_db1 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_db2 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_db2 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_db3 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_db3 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_db4 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_db4 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_db5 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_db5 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_db6 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_db6 * var_vbiinvgat_d)) / assign61790_e80439))) },)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn0, var_tmp__blk1543_dn1, var_tmp__blk1543_dn2, var_tmp__blk1543_dn3, var_tmp__blk1543_dn4, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn9, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11, var_tmp__blk1543_db0, var_tmp__blk1543_db1, var_tmp__blk1543_db2, var_tmp__blk1543_db3, var_tmp__blk1543_db4, var_tmp__blk1543_db5, var_tmp__blk1543_db6,)
    }
};
        var_tmp__blk1543 = assign61790_e80443;
        var_tmp__blk1543_dn0 = assign61790_e80443_d_n0;
        var_tmp__blk1543_dn1 = assign61790_e80443_d_n1;
        var_tmp__blk1543_dn2 = assign61790_e80443_d_n2;
        var_tmp__blk1543_dn3 = assign61790_e80443_d_n3;
        var_tmp__blk1543_dn4 = assign61790_e80443_d_n4;
        var_tmp__blk1543_dn5 = assign61790_e80443_d_n5;
        var_tmp__blk1543_dn6 = assign61790_e80443_d_n6;
        var_tmp__blk1543_dn7 = assign61790_e80443_d_n7;
        var_tmp__blk1543_dn8 = assign61790_e80443_d_n8;
        var_tmp__blk1543_dn9 = assign61790_e80443_d_n9;
        var_tmp__blk1543_dn10 = assign61790_e80443_d_n10;
        var_tmp__blk1543_dn11 = assign61790_e80443_d_n11;
        var_tmp__blk1543_db0 = assign61790_e80443_d_b0;
        var_tmp__blk1543_db1 = assign61790_e80443_d_b1;
        var_tmp__blk1543_db2 = assign61790_e80443_d_b2;
        var_tmp__blk1543_db3 = assign61790_e80443_d_b3;
        var_tmp__blk1543_db4 = assign61790_e80443_d_b4;
        var_tmp__blk1543_db5 = assign61790_e80443_d_b5;
        var_tmp__blk1543_db6 = assign61790_e80443_d_b6;

        let (assign61800_e80468, assign61800_e80468_d_n0, assign61800_e80468_d_n1, assign61800_e80468_d_n2, assign61800_e80468_d_n3, assign61800_e80468_d_n4, assign61800_e80468_d_n5, assign61800_e80468_d_n6, assign61800_e80468_d_n7, assign61800_e80468_d_n8, assign61800_e80468_d_n9, assign61800_e80468_d_n10, assign61800_e80468_d_n11, assign61800_e80468_d_b0, assign61800_e80468_d_b1, assign61800_e80468_d_b2, assign61800_e80468_d_b3, assign61800_e80468_d_b4, assign61800_e80468_d_b5, assign61800_e80468_d_b6,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 == 0.0)) {
        let assign61800_e80458: f64 = (1.0 - var_tmp__blk1543);
        let assign61800_e80459: f64 = (var_qprefgat_d * assign61800_e80458);
        let assign61800_e80463: f64 = (var_vjun_d - var_vj__blk1535);
        let assign61800_e80464: f64 = (var_qpref2gat_d * assign61800_e80463);
        let assign61800_e80465: f64 = (assign61800_e80459 + assign61800_e80464);
        let assign61800_e80466: f64 = (p.p30 * assign61800_e80465);
        (assign61800_e80466, (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn0)) + (var_qpref2gat_d * (var_vjun_d_dn0 - var_vj__blk1535_dn0)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn1)) + (var_qpref2gat_d * (var_vjun_d_dn1 - var_vj__blk1535_dn1)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn2)) + (var_qpref2gat_d * (var_vjun_d_dn2 - var_vj__blk1535_dn2)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn3)) + (var_qpref2gat_d * (var_vjun_d_dn3 - var_vj__blk1535_dn3)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn4)) + (var_qpref2gat_d * (var_vjun_d_dn4 - var_vj__blk1535_dn4)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn5)) + (var_qpref2gat_d * (var_vjun_d_dn5 - var_vj__blk1535_dn5)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn6)) + (var_qpref2gat_d * (var_vjun_d_dn6 - var_vj__blk1535_dn6)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn7)) + (var_qpref2gat_d * (var_vjun_d_dn7 - var_vj__blk1535_dn7)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn8)) + (var_qpref2gat_d * (var_vjun_d_dn8 - var_vj__blk1535_dn8)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn9)) + (var_qpref2gat_d * (var_vjun_d_dn9 - var_vj__blk1535_dn9)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn10)) + (var_qpref2gat_d * (var_vjun_d_dn10 - var_vj__blk1535_dn10)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn11)) + (var_qpref2gat_d * (var_vjun_d_dn11 - var_vj__blk1535_dn11)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_db0)) + (var_qpref2gat_d * (var_vjun_d_db0 - var_vj__blk1535_db0)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_db1)) + (var_qpref2gat_d * (var_vjun_d_db1 - var_vj__blk1535_db1)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_db2)) + (var_qpref2gat_d * (var_vjun_d_db2 - var_vj__blk1535_db2)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_db3)) + (var_qpref2gat_d * (var_vjun_d_db3 - var_vj__blk1535_db3)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_db4)) + (var_qpref2gat_d * (var_vjun_d_db4 - var_vj__blk1535_db4)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_db5)) + (var_qpref2gat_d * (var_vjun_d_db5 - var_vj__blk1535_db5)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_db6)) + (var_qpref2gat_d * (var_vjun_d_db6 - var_vj__blk1535_db6)))),)
    } else {
        (var_qjungat_d, var_qjungat_d_dn0, var_qjungat_d_dn1, var_qjungat_d_dn2, var_qjungat_d_dn3, var_qjungat_d_dn4, var_qjungat_d_dn5, var_qjungat_d_dn6, var_qjungat_d_dn7, var_qjungat_d_dn8, var_qjungat_d_dn9, var_qjungat_d_dn10, var_qjungat_d_dn11, var_qjungat_d_db0, var_qjungat_d_db1, var_qjungat_d_db2, var_qjungat_d_db3, var_qjungat_d_db4, var_qjungat_d_db5, var_qjungat_d_db6,)
    }
};
        var_qjungat_d = assign61800_e80468;
        var_qjungat_d_dn0 = assign61800_e80468_d_n0;
        var_qjungat_d_dn1 = assign61800_e80468_d_n1;
        var_qjungat_d_dn2 = assign61800_e80468_d_n2;
        var_qjungat_d_dn3 = assign61800_e80468_d_n3;
        var_qjungat_d_dn4 = assign61800_e80468_d_n4;
        var_qjungat_d_dn5 = assign61800_e80468_d_n5;
        var_qjungat_d_dn6 = assign61800_e80468_d_n6;
        var_qjungat_d_dn7 = assign61800_e80468_d_n7;
        var_qjungat_d_dn8 = assign61800_e80468_d_n8;
        var_qjungat_d_dn9 = assign61800_e80468_d_n9;
        var_qjungat_d_dn10 = assign61800_e80468_d_n10;
        var_qjungat_d_dn11 = assign61800_e80468_d_n11;
        var_qjungat_d_db0 = assign61800_e80468_d_b0;
        var_qjungat_d_db1 = assign61800_e80468_d_b1;
        var_qjungat_d_db2 = assign61800_e80468_d_b2;
        var_qjungat_d_db3 = assign61800_e80468_d_b3;
        var_qjungat_d_db4 = assign61800_e80468_d_b4;
        var_qjungat_d_db5 = assign61800_e80468_d_b5;
        var_qjungat_d_db6 = assign61800_e80468_d_b6;

        let (assign61810_e80485, assign61810_e80485_d_n0, assign61810_e80485_d_n1, assign61810_e80485_d_n2, assign61810_e80485_d_n3, assign61810_e80485_d_n4, assign61810_e80485_d_n5, assign61810_e80485_d_n6, assign61810_e80485_d_n7, assign61810_e80485_d_n8, assign61810_e80485_d_n9, assign61810_e80485_d_n10, assign61810_e80485_d_n11, assign61810_e80485_d_b0, assign61810_e80485_d_b1, assign61810_e80485_d_b2, assign61810_e80485_d_b3, assign61810_e80485_d_b4, assign61810_e80485_d_b5, assign61810_e80485_d_b6,) = {
    if ((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) {
        let assign61810_e80475: f64 = (var_abdrain_i * var_ijunbot_d);
        let assign61810_e80478: f64 = (var_lsdrain_i * var_ijunsti_d);
        let assign61810_e80479: f64 = (assign61810_e80475 + assign61810_e80478);
        let assign61810_e80482: f64 = (var_lgdrain_i * var_ijungat_d);
        let assign61810_e80483: f64 = (assign61810_e80479 + assign61810_e80482);
        (assign61810_e80483, (((var_abdrain_i * var_ijunbot_d_dn0) + (var_lsdrain_i * var_ijunsti_d_dn0)) + (var_lgdrain_i * var_ijungat_d_dn0)), (((var_abdrain_i * var_ijunbot_d_dn1) + (var_lsdrain_i * var_ijunsti_d_dn1)) + (var_lgdrain_i * var_ijungat_d_dn1)), (((var_abdrain_i * var_ijunbot_d_dn2) + (var_lsdrain_i * var_ijunsti_d_dn2)) + (var_lgdrain_i * var_ijungat_d_dn2)), (((var_abdrain_i * var_ijunbot_d_dn3) + (var_lsdrain_i * var_ijunsti_d_dn3)) + (var_lgdrain_i * var_ijungat_d_dn3)), (((var_abdrain_i * var_ijunbot_d_dn4) + (var_lsdrain_i * var_ijunsti_d_dn4)) + (var_lgdrain_i * var_ijungat_d_dn4)), (((var_abdrain_i * var_ijunbot_d_dn5) + (var_lsdrain_i * var_ijunsti_d_dn5)) + (var_lgdrain_i * var_ijungat_d_dn5)), (((var_abdrain_i * var_ijunbot_d_dn6) + (var_lsdrain_i * var_ijunsti_d_dn6)) + (var_lgdrain_i * var_ijungat_d_dn6)), (((var_abdrain_i * var_ijunbot_d_dn7) + (var_lsdrain_i * var_ijunsti_d_dn7)) + (var_lgdrain_i * var_ijungat_d_dn7)), (((var_abdrain_i * var_ijunbot_d_dn8) + (var_lsdrain_i * var_ijunsti_d_dn8)) + (var_lgdrain_i * var_ijungat_d_dn8)), (((var_abdrain_i * var_ijunbot_d_dn9) + (var_lsdrain_i * var_ijunsti_d_dn9)) + (var_lgdrain_i * var_ijungat_d_dn9)), (((var_abdrain_i * var_ijunbot_d_dn10) + (var_lsdrain_i * var_ijunsti_d_dn10)) + (var_lgdrain_i * var_ijungat_d_dn10)), (((var_abdrain_i * var_ijunbot_d_dn11) + (var_lsdrain_i * var_ijunsti_d_dn11)) + (var_lgdrain_i * var_ijungat_d_dn11)), (((var_abdrain_i * var_ijunbot_d_db0) + (var_lsdrain_i * var_ijunsti_d_db0)) + (var_lgdrain_i * var_ijungat_d_db0)), (((var_abdrain_i * var_ijunbot_d_db1) + (var_lsdrain_i * var_ijunsti_d_db1)) + (var_lgdrain_i * var_ijungat_d_db1)), (((var_abdrain_i * var_ijunbot_d_db2) + (var_lsdrain_i * var_ijunsti_d_db2)) + (var_lgdrain_i * var_ijungat_d_db2)), (((var_abdrain_i * var_ijunbot_d_db3) + (var_lsdrain_i * var_ijunsti_d_db3)) + (var_lgdrain_i * var_ijungat_d_db3)), (((var_abdrain_i * var_ijunbot_d_db4) + (var_lsdrain_i * var_ijunsti_d_db4)) + (var_lgdrain_i * var_ijungat_d_db4)), (((var_abdrain_i * var_ijunbot_d_db5) + (var_lsdrain_i * var_ijunsti_d_db5)) + (var_lgdrain_i * var_ijungat_d_db5)), (((var_abdrain_i * var_ijunbot_d_db6) + (var_lsdrain_i * var_ijunsti_d_db6)) + (var_lgdrain_i * var_ijungat_d_db6)),)
    } else {
        (var_ijun_d, var_ijun_d_dn0, var_ijun_d_dn1, var_ijun_d_dn2, var_ijun_d_dn3, var_ijun_d_dn4, var_ijun_d_dn5, var_ijun_d_dn6, var_ijun_d_dn7, var_ijun_d_dn8, var_ijun_d_dn9, var_ijun_d_dn10, var_ijun_d_dn11, var_ijun_d_db0, var_ijun_d_db1, var_ijun_d_db2, var_ijun_d_db3, var_ijun_d_db4, var_ijun_d_db5, var_ijun_d_db6,)
    }
};
        var_ijun_d = assign61810_e80485;
        var_ijun_d_dn0 = assign61810_e80485_d_n0;
        var_ijun_d_dn1 = assign61810_e80485_d_n1;
        var_ijun_d_dn2 = assign61810_e80485_d_n2;
        var_ijun_d_dn3 = assign61810_e80485_d_n3;
        var_ijun_d_dn4 = assign61810_e80485_d_n4;
        var_ijun_d_dn5 = assign61810_e80485_d_n5;
        var_ijun_d_dn6 = assign61810_e80485_d_n6;
        var_ijun_d_dn7 = assign61810_e80485_d_n7;
        var_ijun_d_dn8 = assign61810_e80485_d_n8;
        var_ijun_d_dn9 = assign61810_e80485_d_n9;
        var_ijun_d_dn10 = assign61810_e80485_d_n10;
        var_ijun_d_dn11 = assign61810_e80485_d_n11;
        var_ijun_d_db0 = assign61810_e80485_d_b0;
        var_ijun_d_db1 = assign61810_e80485_d_b1;
        var_ijun_d_db2 = assign61810_e80485_d_b2;
        var_ijun_d_db3 = assign61810_e80485_d_b3;
        var_ijun_d_db4 = assign61810_e80485_d_b4;
        var_ijun_d_db5 = assign61810_e80485_d_b5;
        var_ijun_d_db6 = assign61810_e80485_d_b6;

        let assign61890_e80509: f64 = if var_sigvds > 0.0 { 1.0 } else { 0.0 };
        var_guard1718 = assign61890_e80509;

        let assign61900_e80512: f64 = if var_rg_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1719 = assign61900_e80512;

        let assign61910_e80515: f64 = if var_rse_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1720 = assign61910_e80515;

        let assign61920_e80518: f64 = if var_rde_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1721 = assign61920_e80518;

        let assign61930_e80521: f64 = if var_rbulk_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1722 = assign61930_e80521;

        let assign61940_e80524: f64 = if var_rjuns_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1723 = assign61940_e80524;

        let assign61950_e80527: f64 = if var_rjund_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1724 = assign61950_e80527;

        let assign61960_e80530: f64 = if var_rwell_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1725 = assign61960_e80530;

        let assign61970_e80533: f64 = (var_qg + var_qb);
        let assign61970_e80535: f64 = (assign61970_e80533 + var_qd);
        let assign61970_e80536: f64 = (-assign61970_e80535);
        var_qs = assign61970_e80536;
        var_qs_dn0 = (-((var_qg_dn0 + var_qb_dn0) + var_qd_dn0));
        var_qs_dn1 = (-((var_qg_dn1 + var_qb_dn1) + var_qd_dn1));
        var_qs_dn2 = (-((var_qg_dn2 + var_qb_dn2) + var_qd_dn2));
        var_qs_dn3 = (-((var_qg_dn3 + var_qb_dn3) + var_qd_dn3));
        var_qs_dn4 = (-((var_qg_dn4 + var_qb_dn4) + var_qd_dn4));
        var_qs_dn5 = (-((var_qg_dn5 + var_qb_dn5) + var_qd_dn5));
        var_qs_dn6 = (-((var_qg_dn6 + var_qb_dn6) + var_qd_dn6));
        var_qs_dn7 = (-((var_qg_dn7 + var_qb_dn7) + var_qd_dn7));
        var_qs_dn8 = (-((var_qg_dn8 + var_qb_dn8) + var_qd_dn8));
        var_qs_dn9 = (-((var_qg_dn9 + var_qb_dn9) + var_qd_dn9));
        var_qs_dn10 = (-((var_qg_dn10 + var_qb_dn10) + var_qd_dn10));
        var_qs_dn11 = (-((var_qg_dn11 + var_qb_dn11) + var_qd_dn11));
        var_qs_db0 = (-((var_qg_db0 + var_qb_db0) + var_qd_db0));
        var_qs_db1 = (-((var_qg_db1 + var_qb_db1) + var_qd_db1));
        var_qs_db2 = (-((var_qg_db2 + var_qb_db2) + var_qd_db2));
        var_qs_db3 = (-((var_qg_db3 + var_qb_db3) + var_qd_db3));
        var_qs_db4 = (-((var_qg_db4 + var_qb_db4) + var_qd_db4));
        var_qs_db5 = (-((var_qg_db5 + var_qb_db5) + var_qd_db5));
        var_qs_db6 = (-((var_qg_db6 + var_qb_db6) + var_qd_db6));

        *var_guard1716_slot = var_guard1716;
        *var_guard1717_slot = var_guard1717;
        *var_guard1718_slot = var_guard1718;
        *var_guard1719_slot = var_guard1719;
        *var_guard1720_slot = var_guard1720;
        *var_guard1721_slot = var_guard1721;
        *var_guard1722_slot = var_guard1722;
        *var_guard1723_slot = var_guard1723;
        *var_guard1724_slot = var_guard1724;
        *var_guard1725_slot = var_guard1725;
        *var_h1__blk1528_slot = var_h1__blk1528;
        *var_h2__blk1529_slot = var_h2__blk1529;
        *var_h2d__blk1530_slot = var_h2d__blk1530;
        *var_h2d__blk1530_db0_slot = var_h2d__blk1530_db0;
        *var_h2d__blk1530_db1_slot = var_h2d__blk1530_db1;
        *var_h2d__blk1530_db2_slot = var_h2d__blk1530_db2;
        *var_h2d__blk1530_db3_slot = var_h2d__blk1530_db3;
        *var_h2d__blk1530_db4_slot = var_h2d__blk1530_db4;
        *var_h2d__blk1530_db5_slot = var_h2d__blk1530_db5;
        *var_h2d__blk1530_db6_slot = var_h2d__blk1530_db6;
        *var_h2d__blk1530_dn0_slot = var_h2d__blk1530_dn0;
        *var_h2d__blk1530_dn1_slot = var_h2d__blk1530_dn1;
        *var_h2d__blk1530_dn10_slot = var_h2d__blk1530_dn10;
        *var_h2d__blk1530_dn11_slot = var_h2d__blk1530_dn11;
        *var_h2d__blk1530_dn2_slot = var_h2d__blk1530_dn2;
        *var_h2d__blk1530_dn3_slot = var_h2d__blk1530_dn3;
        *var_h2d__blk1530_dn4_slot = var_h2d__blk1530_dn4;
        *var_h2d__blk1530_dn5_slot = var_h2d__blk1530_dn5;
        *var_h2d__blk1530_dn6_slot = var_h2d__blk1530_dn6;
        *var_h2d__blk1530_dn7_slot = var_h2d__blk1530_dn7;
        *var_h2d__blk1530_dn8_slot = var_h2d__blk1530_dn8;
        *var_h2d__blk1530_dn9_slot = var_h2d__blk1530_dn9;
        *var_h3__blk1531_slot = var_h3__blk1531;
        *var_h3__blk1531_db0_slot = var_h3__blk1531_db0;
        *var_h3__blk1531_db1_slot = var_h3__blk1531_db1;
        *var_h3__blk1531_db2_slot = var_h3__blk1531_db2;
        *var_h3__blk1531_db3_slot = var_h3__blk1531_db3;
        *var_h3__blk1531_db4_slot = var_h3__blk1531_db4;
        *var_h3__blk1531_db5_slot = var_h3__blk1531_db5;
        *var_h3__blk1531_db6_slot = var_h3__blk1531_db6;
        *var_h3__blk1531_dn0_slot = var_h3__blk1531_dn0;
        *var_h3__blk1531_dn1_slot = var_h3__blk1531_dn1;
        *var_h3__blk1531_dn10_slot = var_h3__blk1531_dn10;
        *var_h3__blk1531_dn11_slot = var_h3__blk1531_dn11;
        *var_h3__blk1531_dn2_slot = var_h3__blk1531_dn2;
        *var_h3__blk1531_dn3_slot = var_h3__blk1531_dn3;
        *var_h3__blk1531_dn4_slot = var_h3__blk1531_dn4;
        *var_h3__blk1531_dn5_slot = var_h3__blk1531_dn5;
        *var_h3__blk1531_dn6_slot = var_h3__blk1531_dn6;
        *var_h3__blk1531_dn7_slot = var_h3__blk1531_dn7;
        *var_h3__blk1531_dn8_slot = var_h3__blk1531_dn8;
        *var_h3__blk1531_dn9_slot = var_h3__blk1531_dn9;
        *var_h4__blk1532_slot = var_h4__blk1532;
        *var_h4__blk1532_db0_slot = var_h4__blk1532_db0;
        *var_h4__blk1532_db1_slot = var_h4__blk1532_db1;
        *var_h4__blk1532_db2_slot = var_h4__blk1532_db2;
        *var_h4__blk1532_db3_slot = var_h4__blk1532_db3;
        *var_h4__blk1532_db4_slot = var_h4__blk1532_db4;
        *var_h4__blk1532_db5_slot = var_h4__blk1532_db5;
        *var_h4__blk1532_db6_slot = var_h4__blk1532_db6;
        *var_h4__blk1532_dn0_slot = var_h4__blk1532_dn0;
        *var_h4__blk1532_dn1_slot = var_h4__blk1532_dn1;
        *var_h4__blk1532_dn10_slot = var_h4__blk1532_dn10;
        *var_h4__blk1532_dn11_slot = var_h4__blk1532_dn11;
        *var_h4__blk1532_dn2_slot = var_h4__blk1532_dn2;
        *var_h4__blk1532_dn3_slot = var_h4__blk1532_dn3;
        *var_h4__blk1532_dn4_slot = var_h4__blk1532_dn4;
        *var_h4__blk1532_dn5_slot = var_h4__blk1532_dn5;
        *var_h4__blk1532_dn6_slot = var_h4__blk1532_dn6;
        *var_h4__blk1532_dn7_slot = var_h4__blk1532_dn7;
        *var_h4__blk1532_dn8_slot = var_h4__blk1532_dn8;
        *var_h4__blk1532_dn9_slot = var_h4__blk1532_dn9;
        *var_h5__blk1533_slot = var_h5__blk1533;
        *var_h5__blk1533_db0_slot = var_h5__blk1533_db0;
        *var_h5__blk1533_db1_slot = var_h5__blk1533_db1;
        *var_h5__blk1533_db2_slot = var_h5__blk1533_db2;
        *var_h5__blk1533_db3_slot = var_h5__blk1533_db3;
        *var_h5__blk1533_db4_slot = var_h5__blk1533_db4;
        *var_h5__blk1533_db5_slot = var_h5__blk1533_db5;
        *var_h5__blk1533_db6_slot = var_h5__blk1533_db6;
        *var_h5__blk1533_dn0_slot = var_h5__blk1533_dn0;
        *var_h5__blk1533_dn1_slot = var_h5__blk1533_dn1;
        *var_h5__blk1533_dn10_slot = var_h5__blk1533_dn10;
        *var_h5__blk1533_dn11_slot = var_h5__blk1533_dn11;
        *var_h5__blk1533_dn2_slot = var_h5__blk1533_dn2;
        *var_h5__blk1533_dn3_slot = var_h5__blk1533_dn3;
        *var_h5__blk1533_dn4_slot = var_h5__blk1533_dn4;
        *var_h5__blk1533_dn5_slot = var_h5__blk1533_dn5;
        *var_h5__blk1533_dn6_slot = var_h5__blk1533_dn6;
        *var_h5__blk1533_dn7_slot = var_h5__blk1533_dn7;
        *var_h5__blk1533_dn8_slot = var_h5__blk1533_dn8;
        *var_h5__blk1533_dn9_slot = var_h5__blk1533_dn9;
        *var_ijun_d_slot = var_ijun_d;
        *var_ijun_d_db0_slot = var_ijun_d_db0;
        *var_ijun_d_db1_slot = var_ijun_d_db1;
        *var_ijun_d_db2_slot = var_ijun_d_db2;
        *var_ijun_d_db3_slot = var_ijun_d_db3;
        *var_ijun_d_db4_slot = var_ijun_d_db4;
        *var_ijun_d_db5_slot = var_ijun_d_db5;
        *var_ijun_d_db6_slot = var_ijun_d_db6;
        *var_ijun_d_dn0_slot = var_ijun_d_dn0;
        *var_ijun_d_dn1_slot = var_ijun_d_dn1;
        *var_ijun_d_dn10_slot = var_ijun_d_dn10;
        *var_ijun_d_dn11_slot = var_ijun_d_dn11;
        *var_ijun_d_dn2_slot = var_ijun_d_dn2;
        *var_ijun_d_dn3_slot = var_ijun_d_dn3;
        *var_ijun_d_dn4_slot = var_ijun_d_dn4;
        *var_ijun_d_dn5_slot = var_ijun_d_dn5;
        *var_ijun_d_dn6_slot = var_ijun_d_dn6;
        *var_ijun_d_dn7_slot = var_ijun_d_dn7;
        *var_ijun_d_dn8_slot = var_ijun_d_dn8;
        *var_ijun_d_dn9_slot = var_ijun_d_dn9;
        *var_nu__blk1570_slot = var_nu__blk1570;
        *var_nu__blk1570_db0_slot = var_nu__blk1570_db0;
        *var_nu__blk1570_db1_slot = var_nu__blk1570_db1;
        *var_nu__blk1570_db2_slot = var_nu__blk1570_db2;
        *var_nu__blk1570_db3_slot = var_nu__blk1570_db3;
        *var_nu__blk1570_db4_slot = var_nu__blk1570_db4;
        *var_nu__blk1570_db5_slot = var_nu__blk1570_db5;
        *var_nu__blk1570_db6_slot = var_nu__blk1570_db6;
        *var_nu__blk1570_dn0_slot = var_nu__blk1570_dn0;
        *var_nu__blk1570_dn1_slot = var_nu__blk1570_dn1;
        *var_nu__blk1570_dn10_slot = var_nu__blk1570_dn10;
        *var_nu__blk1570_dn11_slot = var_nu__blk1570_dn11;
        *var_nu__blk1570_dn2_slot = var_nu__blk1570_dn2;
        *var_nu__blk1570_dn3_slot = var_nu__blk1570_dn3;
        *var_nu__blk1570_dn4_slot = var_nu__blk1570_dn4;
        *var_nu__blk1570_dn5_slot = var_nu__blk1570_dn5;
        *var_nu__blk1570_dn6_slot = var_nu__blk1570_dn6;
        *var_nu__blk1570_dn7_slot = var_nu__blk1570_dn7;
        *var_nu__blk1570_dn8_slot = var_nu__blk1570_dn8;
        *var_nu__blk1570_dn9_slot = var_nu__blk1570_dn9;
        *var_qjungat2nd_slot = var_qjungat2nd;
        *var_qjungat2nd_db0_slot = var_qjungat2nd_db0;
        *var_qjungat2nd_db1_slot = var_qjungat2nd_db1;
        *var_qjungat2nd_db2_slot = var_qjungat2nd_db2;
        *var_qjungat2nd_db3_slot = var_qjungat2nd_db3;
        *var_qjungat2nd_db4_slot = var_qjungat2nd_db4;
        *var_qjungat2nd_db5_slot = var_qjungat2nd_db5;
        *var_qjungat2nd_db6_slot = var_qjungat2nd_db6;
        *var_qjungat2nd_dn0_slot = var_qjungat2nd_dn0;
        *var_qjungat2nd_dn1_slot = var_qjungat2nd_dn1;
        *var_qjungat2nd_dn10_slot = var_qjungat2nd_dn10;
        *var_qjungat2nd_dn11_slot = var_qjungat2nd_dn11;
        *var_qjungat2nd_dn2_slot = var_qjungat2nd_dn2;
        *var_qjungat2nd_dn3_slot = var_qjungat2nd_dn3;
        *var_qjungat2nd_dn4_slot = var_qjungat2nd_dn4;
        *var_qjungat2nd_dn5_slot = var_qjungat2nd_dn5;
        *var_qjungat2nd_dn6_slot = var_qjungat2nd_dn6;
        *var_qjungat2nd_dn7_slot = var_qjungat2nd_dn7;
        *var_qjungat2nd_dn8_slot = var_qjungat2nd_dn8;
        *var_qjungat2nd_dn9_slot = var_qjungat2nd_dn9;
        *var_qjungat_d_slot = var_qjungat_d;
        *var_qjungat_d_db0_slot = var_qjungat_d_db0;
        *var_qjungat_d_db1_slot = var_qjungat_d_db1;
        *var_qjungat_d_db2_slot = var_qjungat_d_db2;
        *var_qjungat_d_db3_slot = var_qjungat_d_db3;
        *var_qjungat_d_db4_slot = var_qjungat_d_db4;
        *var_qjungat_d_db5_slot = var_qjungat_d_db5;
        *var_qjungat_d_db6_slot = var_qjungat_d_db6;
        *var_qjungat_d_dn0_slot = var_qjungat_d_dn0;
        *var_qjungat_d_dn1_slot = var_qjungat_d_dn1;
        *var_qjungat_d_dn10_slot = var_qjungat_d_dn10;
        *var_qjungat_d_dn11_slot = var_qjungat_d_dn11;
        *var_qjungat_d_dn2_slot = var_qjungat_d_dn2;
        *var_qjungat_d_dn3_slot = var_qjungat_d_dn3;
        *var_qjungat_d_dn4_slot = var_qjungat_d_dn4;
        *var_qjungat_d_dn5_slot = var_qjungat_d_dn5;
        *var_qjungat_d_dn6_slot = var_qjungat_d_dn6;
        *var_qjungat_d_dn7_slot = var_qjungat_d_dn7;
        *var_qjungat_d_dn8_slot = var_qjungat_d_dn8;
        *var_qjungat_d_dn9_slot = var_qjungat_d_dn9;
        *var_qs_slot = var_qs;
        *var_qs_db0_slot = var_qs_db0;
        *var_qs_db1_slot = var_qs_db1;
        *var_qs_db2_slot = var_qs_db2;
        *var_qs_db3_slot = var_qs_db3;
        *var_qs_db4_slot = var_qs_db4;
        *var_qs_db5_slot = var_qs_db5;
        *var_qs_db6_slot = var_qs_db6;
        *var_qs_dn0_slot = var_qs_dn0;
        *var_qs_dn1_slot = var_qs_dn1;
        *var_qs_dn10_slot = var_qs_dn10;
        *var_qs_dn11_slot = var_qs_dn11;
        *var_qs_dn2_slot = var_qs_dn2;
        *var_qs_dn3_slot = var_qs_dn3;
        *var_qs_dn4_slot = var_qs_dn4;
        *var_qs_dn5_slot = var_qs_dn5;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_qs_dn9_slot = var_qs_dn9;
        *var_tmp__blk1543_slot = var_tmp__blk1543;
        *var_tmp__blk1543_db0_slot = var_tmp__blk1543_db0;
        *var_tmp__blk1543_db1_slot = var_tmp__blk1543_db1;
        *var_tmp__blk1543_db2_slot = var_tmp__blk1543_db2;
        *var_tmp__blk1543_db3_slot = var_tmp__blk1543_db3;
        *var_tmp__blk1543_db4_slot = var_tmp__blk1543_db4;
        *var_tmp__blk1543_db5_slot = var_tmp__blk1543_db5;
        *var_tmp__blk1543_db6_slot = var_tmp__blk1543_db6;
        *var_tmp__blk1543_dn0_slot = var_tmp__blk1543_dn0;
        *var_tmp__blk1543_dn1_slot = var_tmp__blk1543_dn1;
        *var_tmp__blk1543_dn10_slot = var_tmp__blk1543_dn10;
        *var_tmp__blk1543_dn11_slot = var_tmp__blk1543_dn11;
        *var_tmp__blk1543_dn2_slot = var_tmp__blk1543_dn2;
        *var_tmp__blk1543_dn3_slot = var_tmp__blk1543_dn3;
        *var_tmp__blk1543_dn4_slot = var_tmp__blk1543_dn4;
        *var_tmp__blk1543_dn5_slot = var_tmp__blk1543_dn5;
        *var_tmp__blk1543_dn6_slot = var_tmp__blk1543_dn6;
        *var_tmp__blk1543_dn7_slot = var_tmp__blk1543_dn7;
        *var_tmp__blk1543_dn8_slot = var_tmp__blk1543_dn8;
        *var_tmp__blk1543_dn9_slot = var_tmp__blk1543_dn9;
        *var_vjtmp_slot = var_vjtmp;
        *var_vjtmp_db0_slot = var_vjtmp_db0;
        *var_vjtmp_db1_slot = var_vjtmp_db1;
        *var_vjtmp_db2_slot = var_vjtmp_db2;
        *var_vjtmp_db3_slot = var_vjtmp_db3;
        *var_vjtmp_db4_slot = var_vjtmp_db4;
        *var_vjtmp_db5_slot = var_vjtmp_db5;
        *var_vjtmp_db6_slot = var_vjtmp_db6;
        *var_vjtmp_dn0_slot = var_vjtmp_dn0;
        *var_vjtmp_dn1_slot = var_vjtmp_dn1;
        *var_vjtmp_dn10_slot = var_vjtmp_dn10;
        *var_vjtmp_dn11_slot = var_vjtmp_dn11;
        *var_vjtmp_dn2_slot = var_vjtmp_dn2;
        *var_vjtmp_dn3_slot = var_vjtmp_dn3;
        *var_vjtmp_dn4_slot = var_vjtmp_dn4;
        *var_vjtmp_dn5_slot = var_vjtmp_dn5;
        *var_vjtmp_dn6_slot = var_vjtmp_dn6;
        *var_vjtmp_dn7_slot = var_vjtmp_dn7;
        *var_vjtmp_dn8_slot = var_vjtmp_dn8;
        *var_vjtmp_dn9_slot = var_vjtmp_dn9;
    }

    pub(super) fn stamp_transient_block_244(
        p: &Parameters,
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_alpha_dc: f64,
        var_alpha_dc_db0: f64,
        var_alpha_dc_db1: f64,
        var_alpha_dc_db2: f64,
        var_alpha_dc_db3: f64,
        var_alpha_dc_db4: f64,
        var_alpha_dc_db5: f64,
        var_alpha_dc_db6: f64,
        var_alpha_dc_dn0: f64,
        var_alpha_dc_dn1: f64,
        var_alpha_dc_dn10: f64,
        var_alpha_dc_dn11: f64,
        var_alpha_dc_dn2: f64,
        var_alpha_dc_dn3: f64,
        var_alpha_dc_dn4: f64,
        var_alpha_dc_dn5: f64,
        var_alpha_dc_dn6: f64,
        var_alpha_dc_dn7: f64,
        var_alpha_dc_dn8: f64,
        var_alpha_dc_dn9: f64,
        var_bet_i: f64,
        var_cox_qm: f64,
        var_cox_qm_db0: f64,
        var_cox_qm_db1: f64,
        var_cox_qm_db2: f64,
        var_cox_qm_db3: f64,
        var_cox_qm_db4: f64,
        var_cox_qm_db5: f64,
        var_cox_qm_db6: f64,
        var_cox_qm_dn0: f64,
        var_cox_qm_dn1: f64,
        var_cox_qm_dn10: f64,
        var_cox_qm_dn11: f64,
        var_cox_qm_dn2: f64,
        var_cox_qm_dn3: f64,
        var_cox_qm_dn4: f64,
        var_cox_qm_dn5: f64,
        var_cox_qm_dn6: f64,
        var_cox_qm_dn7: f64,
        var_cox_qm_dn8: f64,
        var_cox_qm_dn9: f64,
        var_dps_dc: f64,
        var_dps_dc_db0: f64,
        var_dps_dc_db1: f64,
        var_dps_dc_db2: f64,
        var_dps_dc_db3: f64,
        var_dps_dc_db4: f64,
        var_dps_dc_db5: f64,
        var_dps_dc_db6: f64,
        var_dps_dc_dn0: f64,
        var_dps_dc_dn1: f64,
        var_dps_dc_dn10: f64,
        var_dps_dc_dn11: f64,
        var_dps_dc_dn2: f64,
        var_dps_dc_dn3: f64,
        var_dps_dc_dn4: f64,
        var_dps_dc_dn5: f64,
        var_dps_dc_dn6: f64,
        var_dps_dc_dn7: f64,
        var_dps_dc_dn8: f64,
        var_dps_dc_dn9: f64,
        var_eta_p_ac: f64,
        var_eta_p_ac_db0: f64,
        var_eta_p_ac_db1: f64,
        var_eta_p_ac_db2: f64,
        var_eta_p_ac_db3: f64,
        var_eta_p_ac_db4: f64,
        var_eta_p_ac_db5: f64,
        var_eta_p_ac_db6: f64,
        var_eta_p_ac_dn0: f64,
        var_eta_p_ac_dn1: f64,
        var_eta_p_ac_dn10: f64,
        var_eta_p_ac_dn11: f64,
        var_eta_p_ac_dn2: f64,
        var_eta_p_ac_dn3: f64,
        var_eta_p_ac_dn4: f64,
        var_eta_p_ac_dn5: f64,
        var_eta_p_ac_dn6: f64,
        var_eta_p_ac_dn7: f64,
        var_eta_p_ac_dn8: f64,
        var_eta_p_ac_dn9: f64,
        var_h_dc: f64,
        var_h_dc_db0: f64,
        var_h_dc_db1: f64,
        var_h_dc_db2: f64,
        var_h_dc_db3: f64,
        var_h_dc_db4: f64,
        var_h_dc_db5: f64,
        var_h_dc_db6: f64,
        var_h_dc_dn0: f64,
        var_h_dc_dn1: f64,
        var_h_dc_dn10: f64,
        var_h_dc_dn11: f64,
        var_h_dc_dn2: f64,
        var_h_dc_dn3: f64,
        var_h_dc_dn4: f64,
        var_h_dc_dn5: f64,
        var_h_dc_dn6: f64,
        var_h_dc_dn7: f64,
        var_h_dc_dn8: f64,
        var_h_dc_dn9: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_qgd_ov: f64,
        var_qgd_ov_db0: f64,
        var_qgd_ov_db1: f64,
        var_qgd_ov_db2: f64,
        var_qgd_ov_db3: f64,
        var_qgd_ov_db4: f64,
        var_qgd_ov_db5: f64,
        var_qgd_ov_db6: f64,
        var_qgd_ov_dn0: f64,
        var_qgd_ov_dn1: f64,
        var_qgd_ov_dn10: f64,
        var_qgd_ov_dn11: f64,
        var_qgd_ov_dn2: f64,
        var_qgd_ov_dn3: f64,
        var_qgd_ov_dn4: f64,
        var_qgd_ov_dn5: f64,
        var_qgd_ov_dn6: f64,
        var_qgd_ov_dn7: f64,
        var_qgd_ov_dn8: f64,
        var_qgd_ov_dn9: f64,
        var_qgs_ov: f64,
        var_qgs_ov_db0: f64,
        var_qgs_ov_db1: f64,
        var_qgs_ov_db2: f64,
        var_qgs_ov_db3: f64,
        var_qgs_ov_db4: f64,
        var_qgs_ov_db5: f64,
        var_qgs_ov_db6: f64,
        var_qgs_ov_dn0: f64,
        var_qgs_ov_dn1: f64,
        var_qgs_ov_dn10: f64,
        var_qgs_ov_dn11: f64,
        var_qgs_ov_dn2: f64,
        var_qgs_ov_dn3: f64,
        var_qgs_ov_dn4: f64,
        var_qgs_ov_dn5: f64,
        var_qgs_ov_dn6: f64,
        var_qgs_ov_dn7: f64,
        var_qgs_ov_dn8: f64,
        var_qgs_ov_dn9: f64,
        var_qim1_dc: f64,
        var_qim1_dc_db0: f64,
        var_qim1_dc_db1: f64,
        var_qim1_dc_db2: f64,
        var_qim1_dc_db3: f64,
        var_qim1_dc_db4: f64,
        var_qim1_dc_db5: f64,
        var_qim1_dc_db6: f64,
        var_qim1_dc_dn0: f64,
        var_qim1_dc_dn1: f64,
        var_qim1_dc_dn10: f64,
        var_qim1_dc_dn11: f64,
        var_qim1_dc_dn2: f64,
        var_qim1_dc_dn3: f64,
        var_qim1_dc_dn4: f64,
        var_qim1_dc_dn5: f64,
        var_qim1_dc_dn6: f64,
        var_qim1_dc_dn7: f64,
        var_qim1_dc_dn8: f64,
        var_qim1_dc_dn9: f64,
        var_qim_dc: f64,
        var_qim_dc_db0: f64,
        var_qim_dc_db1: f64,
        var_qim_dc_db2: f64,
        var_qim_dc_db3: f64,
        var_qim_dc_db4: f64,
        var_qim_dc_db5: f64,
        var_qim_dc_db6: f64,
        var_qim_dc_dn0: f64,
        var_qim_dc_dn1: f64,
        var_qim_dc_dn10: f64,
        var_qim_dc_dn11: f64,
        var_qim_dc_dn2: f64,
        var_qim_dc_dn3: f64,
        var_qim_dc_dn4: f64,
        var_qim_dc_dn5: f64,
        var_qim_dc_dn6: f64,
        var_qim_dc_dn7: f64,
        var_qim_dc_dn8: f64,
        var_qim_dc_dn9: f64,
        var_qjunbot_d: f64,
        var_qjunbot_d_db0: f64,
        var_qjunbot_d_db1: f64,
        var_qjunbot_d_db2: f64,
        var_qjunbot_d_db3: f64,
        var_qjunbot_d_db4: f64,
        var_qjunbot_d_db5: f64,
        var_qjunbot_d_db6: f64,
        var_qjunbot_d_dn0: f64,
        var_qjunbot_d_dn1: f64,
        var_qjunbot_d_dn10: f64,
        var_qjunbot_d_dn11: f64,
        var_qjunbot_d_dn2: f64,
        var_qjunbot_d_dn3: f64,
        var_qjunbot_d_dn4: f64,
        var_qjunbot_d_dn5: f64,
        var_qjunbot_d_dn6: f64,
        var_qjunbot_d_dn7: f64,
        var_qjunbot_d_dn8: f64,
        var_qjunbot_d_dn9: f64,
        var_qjunbot_s: f64,
        var_qjunbot_s_db0: f64,
        var_qjunbot_s_db1: f64,
        var_qjunbot_s_db2: f64,
        var_qjunbot_s_db3: f64,
        var_qjunbot_s_db4: f64,
        var_qjunbot_s_db5: f64,
        var_qjunbot_s_db6: f64,
        var_qjunbot_s_dn0: f64,
        var_qjunbot_s_dn1: f64,
        var_qjunbot_s_dn10: f64,
        var_qjunbot_s_dn11: f64,
        var_qjunbot_s_dn2: f64,
        var_qjunbot_s_dn3: f64,
        var_qjunbot_s_dn4: f64,
        var_qjunbot_s_dn5: f64,
        var_qjunbot_s_dn6: f64,
        var_qjunbot_s_dn7: f64,
        var_qjunbot_s_dn8: f64,
        var_qjunbot_s_dn9: f64,
        var_qjungat_d: f64,
        var_qjungat_d_db0: f64,
        var_qjungat_d_db1: f64,
        var_qjungat_d_db2: f64,
        var_qjungat_d_db3: f64,
        var_qjungat_d_db4: f64,
        var_qjungat_d_db5: f64,
        var_qjungat_d_db6: f64,
        var_qjungat_d_dn0: f64,
        var_qjungat_d_dn1: f64,
        var_qjungat_d_dn10: f64,
        var_qjungat_d_dn11: f64,
        var_qjungat_d_dn2: f64,
        var_qjungat_d_dn3: f64,
        var_qjungat_d_dn4: f64,
        var_qjungat_d_dn5: f64,
        var_qjungat_d_dn6: f64,
        var_qjungat_d_dn7: f64,
        var_qjungat_d_dn8: f64,
        var_qjungat_d_dn9: f64,
        var_qjungat_s: f64,
        var_qjungat_s_db0: f64,
        var_qjungat_s_db1: f64,
        var_qjungat_s_db2: f64,
        var_qjungat_s_db3: f64,
        var_qjungat_s_db4: f64,
        var_qjungat_s_db5: f64,
        var_qjungat_s_db6: f64,
        var_qjungat_s_dn0: f64,
        var_qjungat_s_dn1: f64,
        var_qjungat_s_dn10: f64,
        var_qjungat_s_dn11: f64,
        var_qjungat_s_dn2: f64,
        var_qjungat_s_dn3: f64,
        var_qjungat_s_dn4: f64,
        var_qjungat_s_dn5: f64,
        var_qjungat_s_dn6: f64,
        var_qjungat_s_dn7: f64,
        var_qjungat_s_dn8: f64,
        var_qjungat_s_dn9: f64,
        var_qjunsti_d: f64,
        var_qjunsti_d_db0: f64,
        var_qjunsti_d_db1: f64,
        var_qjunsti_d_db2: f64,
        var_qjunsti_d_db3: f64,
        var_qjunsti_d_db4: f64,
        var_qjunsti_d_db5: f64,
        var_qjunsti_d_db6: f64,
        var_qjunsti_d_dn0: f64,
        var_qjunsti_d_dn1: f64,
        var_qjunsti_d_dn10: f64,
        var_qjunsti_d_dn11: f64,
        var_qjunsti_d_dn2: f64,
        var_qjunsti_d_dn3: f64,
        var_qjunsti_d_dn4: f64,
        var_qjunsti_d_dn5: f64,
        var_qjunsti_d_dn6: f64,
        var_qjunsti_d_dn7: f64,
        var_qjunsti_d_dn8: f64,
        var_qjunsti_d_dn9: f64,
        var_qjunsti_s: f64,
        var_qjunsti_s_db0: f64,
        var_qjunsti_s_db1: f64,
        var_qjunsti_s_db2: f64,
        var_qjunsti_s_db3: f64,
        var_qjunsti_s_db4: f64,
        var_qjunsti_s_db5: f64,
        var_qjunsti_s_db6: f64,
        var_qjunsti_s_dn0: f64,
        var_qjunsti_s_dn1: f64,
        var_qjunsti_s_dn10: f64,
        var_qjunsti_s_dn11: f64,
        var_qjunsti_s_dn2: f64,
        var_qjunsti_s_dn3: f64,
        var_qjunsti_s_dn4: f64,
        var_qjunsti_s_dn5: f64,
        var_qjunsti_s_dn6: f64,
        var_qjunsti_s_dn7: f64,
        var_qjunsti_s_dn8: f64,
        var_qjunsti_s_dn9: f64,
        var_sigvds: f64,
        var_xg_dc: f64,
        var_c_igid_slot: &mut f64,
        var_c_igid_db0_slot: &mut f64,
        var_c_igid_db1_slot: &mut f64,
        var_c_igid_db2_slot: &mut f64,
        var_c_igid_db3_slot: &mut f64,
        var_c_igid_db4_slot: &mut f64,
        var_c_igid_db5_slot: &mut f64,
        var_c_igid_db6_slot: &mut f64,
        var_c_igid_dn0_slot: &mut f64,
        var_c_igid_dn1_slot: &mut f64,
        var_c_igid_dn10_slot: &mut f64,
        var_c_igid_dn11_slot: &mut f64,
        var_c_igid_dn2_slot: &mut f64,
        var_c_igid_dn3_slot: &mut f64,
        var_c_igid_dn4_slot: &mut f64,
        var_c_igid_dn5_slot: &mut f64,
        var_c_igid_dn6_slot: &mut f64,
        var_c_igid_dn7_slot: &mut f64,
        var_c_igid_dn8_slot: &mut f64,
        var_c_igid_dn9_slot: &mut f64,
        var_cgeff_slot: &mut f64,
        var_cgeff_db0_slot: &mut f64,
        var_cgeff_db1_slot: &mut f64,
        var_cgeff_db2_slot: &mut f64,
        var_cgeff_db3_slot: &mut f64,
        var_cgeff_db4_slot: &mut f64,
        var_cgeff_db5_slot: &mut f64,
        var_cgeff_db6_slot: &mut f64,
        var_cgeff_dn0_slot: &mut f64,
        var_cgeff_dn1_slot: &mut f64,
        var_cgeff_dn10_slot: &mut f64,
        var_cgeff_dn11_slot: &mut f64,
        var_cgeff_dn2_slot: &mut f64,
        var_cgeff_dn3_slot: &mut f64,
        var_cgeff_dn4_slot: &mut f64,
        var_cgeff_dn5_slot: &mut f64,
        var_cgeff_dn6_slot: &mut f64,
        var_cgeff_dn7_slot: &mut f64,
        var_cgeff_dn8_slot: &mut f64,
        var_cgeff_dn9_slot: &mut f64,
        var_guard1727_slot: &mut f64,
        var_guard1760_slot: &mut f64,
        var_guard1762_slot: &mut f64,
        var_h0_slot: &mut f64,
        var_h0_db0_slot: &mut f64,
        var_h0_db1_slot: &mut f64,
        var_h0_db2_slot: &mut f64,
        var_h0_db3_slot: &mut f64,
        var_h0_db4_slot: &mut f64,
        var_h0_db5_slot: &mut f64,
        var_h0_db6_slot: &mut f64,
        var_h0_dn0_slot: &mut f64,
        var_h0_dn1_slot: &mut f64,
        var_h0_dn10_slot: &mut f64,
        var_h0_dn11_slot: &mut f64,
        var_h0_dn2_slot: &mut f64,
        var_h0_dn3_slot: &mut f64,
        var_h0_dn4_slot: &mut f64,
        var_h0_dn5_slot: &mut f64,
        var_h0_dn6_slot: &mut f64,
        var_h0_dn7_slot: &mut f64,
        var_h0_dn8_slot: &mut f64,
        var_h0_dn9_slot: &mut f64,
        var_mid_slot: &mut f64,
        var_mid_db0_slot: &mut f64,
        var_mid_db1_slot: &mut f64,
        var_mid_db2_slot: &mut f64,
        var_mid_db3_slot: &mut f64,
        var_mid_db4_slot: &mut f64,
        var_mid_db5_slot: &mut f64,
        var_mid_db6_slot: &mut f64,
        var_mid_dn0_slot: &mut f64,
        var_mid_dn1_slot: &mut f64,
        var_mid_dn10_slot: &mut f64,
        var_mid_dn11_slot: &mut f64,
        var_mid_dn2_slot: &mut f64,
        var_mid_dn3_slot: &mut f64,
        var_mid_dn4_slot: &mut f64,
        var_mid_dn5_slot: &mut f64,
        var_mid_dn6_slot: &mut f64,
        var_mid_dn7_slot: &mut f64,
        var_mid_dn8_slot: &mut f64,
        var_mid_dn9_slot: &mut f64,
        var_mig_slot: &mut f64,
        var_mig_db0_slot: &mut f64,
        var_mig_db1_slot: &mut f64,
        var_mig_db2_slot: &mut f64,
        var_mig_db3_slot: &mut f64,
        var_mig_db4_slot: &mut f64,
        var_mig_db5_slot: &mut f64,
        var_mig_db6_slot: &mut f64,
        var_mig_dn0_slot: &mut f64,
        var_mig_dn1_slot: &mut f64,
        var_mig_dn10_slot: &mut f64,
        var_mig_dn11_slot: &mut f64,
        var_mig_dn2_slot: &mut f64,
        var_mig_dn3_slot: &mut f64,
        var_mig_dn4_slot: &mut f64,
        var_mig_dn5_slot: &mut f64,
        var_mig_dn6_slot: &mut f64,
        var_mig_dn7_slot: &mut f64,
        var_mig_dn8_slot: &mut f64,
        var_mig_dn9_slot: &mut f64,
        var_migid_slot: &mut f64,
        var_migid_db0_slot: &mut f64,
        var_migid_db1_slot: &mut f64,
        var_migid_db2_slot: &mut f64,
        var_migid_db3_slot: &mut f64,
        var_migid_db4_slot: &mut f64,
        var_migid_db5_slot: &mut f64,
        var_migid_db6_slot: &mut f64,
        var_migid_dn0_slot: &mut f64,
        var_migid_dn1_slot: &mut f64,
        var_migid_dn10_slot: &mut f64,
        var_migid_dn11_slot: &mut f64,
        var_migid_dn2_slot: &mut f64,
        var_migid_dn3_slot: &mut f64,
        var_migid_dn4_slot: &mut f64,
        var_migid_dn5_slot: &mut f64,
        var_migid_dn6_slot: &mut f64,
        var_migid_dn7_slot: &mut f64,
        var_migid_dn8_slot: &mut f64,
        var_migid_dn9_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_db0_slot: &mut f64,
        var_qd_db1_slot: &mut f64,
        var_qd_db2_slot: &mut f64,
        var_qd_db3_slot: &mut f64,
        var_qd_db4_slot: &mut f64,
        var_qd_db5_slot: &mut f64,
        var_qd_db6_slot: &mut f64,
        var_qd_dn0_slot: &mut f64,
        var_qd_dn1_slot: &mut f64,
        var_qd_dn10_slot: &mut f64,
        var_qd_dn11_slot: &mut f64,
        var_qd_dn2_slot: &mut f64,
        var_qd_dn3_slot: &mut f64,
        var_qd_dn4_slot: &mut f64,
        var_qd_dn5_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qd_dn9_slot: &mut f64,
        var_qfgd_slot: &mut f64,
        var_qfgd_db0_slot: &mut f64,
        var_qfgd_db1_slot: &mut f64,
        var_qfgd_db2_slot: &mut f64,
        var_qfgd_db3_slot: &mut f64,
        var_qfgd_db4_slot: &mut f64,
        var_qfgd_db5_slot: &mut f64,
        var_qfgd_db6_slot: &mut f64,
        var_qfgd_dn0_slot: &mut f64,
        var_qfgd_dn1_slot: &mut f64,
        var_qfgd_dn10_slot: &mut f64,
        var_qfgd_dn11_slot: &mut f64,
        var_qfgd_dn2_slot: &mut f64,
        var_qfgd_dn3_slot: &mut f64,
        var_qfgd_dn4_slot: &mut f64,
        var_qfgd_dn5_slot: &mut f64,
        var_qfgd_dn6_slot: &mut f64,
        var_qfgd_dn7_slot: &mut f64,
        var_qfgd_dn8_slot: &mut f64,
        var_qfgd_dn9_slot: &mut f64,
        var_qfgs_slot: &mut f64,
        var_qfgs_db0_slot: &mut f64,
        var_qfgs_db1_slot: &mut f64,
        var_qfgs_db2_slot: &mut f64,
        var_qfgs_db3_slot: &mut f64,
        var_qfgs_db4_slot: &mut f64,
        var_qfgs_db5_slot: &mut f64,
        var_qfgs_db6_slot: &mut f64,
        var_qfgs_dn0_slot: &mut f64,
        var_qfgs_dn1_slot: &mut f64,
        var_qfgs_dn10_slot: &mut f64,
        var_qfgs_dn11_slot: &mut f64,
        var_qfgs_dn2_slot: &mut f64,
        var_qfgs_dn3_slot: &mut f64,
        var_qfgs_dn4_slot: &mut f64,
        var_qfgs_dn5_slot: &mut f64,
        var_qfgs_dn6_slot: &mut f64,
        var_qfgs_dn7_slot: &mut f64,
        var_qfgs_dn8_slot: &mut f64,
        var_qfgs_dn9_slot: &mut f64,
        var_qjun_d_slot: &mut f64,
        var_qjun_d_db0_slot: &mut f64,
        var_qjun_d_db1_slot: &mut f64,
        var_qjun_d_db2_slot: &mut f64,
        var_qjun_d_db3_slot: &mut f64,
        var_qjun_d_db4_slot: &mut f64,
        var_qjun_d_db5_slot: &mut f64,
        var_qjun_d_db6_slot: &mut f64,
        var_qjun_d_dn0_slot: &mut f64,
        var_qjun_d_dn1_slot: &mut f64,
        var_qjun_d_dn10_slot: &mut f64,
        var_qjun_d_dn11_slot: &mut f64,
        var_qjun_d_dn2_slot: &mut f64,
        var_qjun_d_dn3_slot: &mut f64,
        var_qjun_d_dn4_slot: &mut f64,
        var_qjun_d_dn5_slot: &mut f64,
        var_qjun_d_dn6_slot: &mut f64,
        var_qjun_d_dn7_slot: &mut f64,
        var_qjun_d_dn8_slot: &mut f64,
        var_qjun_d_dn9_slot: &mut f64,
        var_qjun_s_slot: &mut f64,
        var_qjun_s_db0_slot: &mut f64,
        var_qjun_s_db1_slot: &mut f64,
        var_qjun_s_db2_slot: &mut f64,
        var_qjun_s_db3_slot: &mut f64,
        var_qjun_s_db4_slot: &mut f64,
        var_qjun_s_db5_slot: &mut f64,
        var_qjun_s_db6_slot: &mut f64,
        var_qjun_s_dn0_slot: &mut f64,
        var_qjun_s_dn1_slot: &mut f64,
        var_qjun_s_dn10_slot: &mut f64,
        var_qjun_s_dn11_slot: &mut f64,
        var_qjun_s_dn2_slot: &mut f64,
        var_qjun_s_dn3_slot: &mut f64,
        var_qjun_s_dn4_slot: &mut f64,
        var_qjun_s_dn5_slot: &mut f64,
        var_qjun_s_dn6_slot: &mut f64,
        var_qjun_s_dn7_slot: &mut f64,
        var_qjun_s_dn8_slot: &mut f64,
        var_qjun_s_dn9_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_db0_slot: &mut f64,
        var_qs_db1_slot: &mut f64,
        var_qs_db2_slot: &mut f64,
        var_qs_db3_slot: &mut f64,
        var_qs_db4_slot: &mut f64,
        var_qs_db5_slot: &mut f64,
        var_qs_db6_slot: &mut f64,
        var_qs_dn0_slot: &mut f64,
        var_qs_dn1_slot: &mut f64,
        var_qs_dn10_slot: &mut f64,
        var_qs_dn11_slot: &mut f64,
        var_qs_dn2_slot: &mut f64,
        var_qs_dn3_slot: &mut f64,
        var_qs_dn4_slot: &mut f64,
        var_qs_dn5_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_qs_dn9_slot: &mut f64,
        var_r_slot: &mut f64,
        var_r_db0_slot: &mut f64,
        var_r_db1_slot: &mut f64,
        var_r_db2_slot: &mut f64,
        var_r_db3_slot: &mut f64,
        var_r_db4_slot: &mut f64,
        var_r_db5_slot: &mut f64,
        var_r_db6_slot: &mut f64,
        var_r_dn0_slot: &mut f64,
        var_r_dn1_slot: &mut f64,
        var_r_dn10_slot: &mut f64,
        var_r_dn11_slot: &mut f64,
        var_r_dn2_slot: &mut f64,
        var_r_dn3_slot: &mut f64,
        var_r_dn4_slot: &mut f64,
        var_r_dn5_slot: &mut f64,
        var_r_dn6_slot: &mut f64,
        var_r_dn7_slot: &mut f64,
        var_r_dn8_slot: &mut f64,
        var_r_dn9_slot: &mut f64,
        var_sidexc_slot: &mut f64,
        var_sidexc_db0_slot: &mut f64,
        var_sidexc_db1_slot: &mut f64,
        var_sidexc_db2_slot: &mut f64,
        var_sidexc_db3_slot: &mut f64,
        var_sidexc_db4_slot: &mut f64,
        var_sidexc_db5_slot: &mut f64,
        var_sidexc_db6_slot: &mut f64,
        var_sidexc_dn0_slot: &mut f64,
        var_sidexc_dn1_slot: &mut f64,
        var_sidexc_dn10_slot: &mut f64,
        var_sidexc_dn11_slot: &mut f64,
        var_sidexc_dn2_slot: &mut f64,
        var_sidexc_dn3_slot: &mut f64,
        var_sidexc_dn4_slot: &mut f64,
        var_sidexc_dn5_slot: &mut f64,
        var_sidexc_dn6_slot: &mut f64,
        var_sidexc_dn7_slot: &mut f64,
        var_sidexc_dn8_slot: &mut f64,
        var_sidexc_dn9_slot: &mut f64,
        var_sqid_slot: &mut f64,
        var_sqid_db0_slot: &mut f64,
        var_sqid_db1_slot: &mut f64,
        var_sqid_db2_slot: &mut f64,
        var_sqid_db3_slot: &mut f64,
        var_sqid_db4_slot: &mut f64,
        var_sqid_db5_slot: &mut f64,
        var_sqid_db6_slot: &mut f64,
        var_sqid_dn0_slot: &mut f64,
        var_sqid_dn1_slot: &mut f64,
        var_sqid_dn10_slot: &mut f64,
        var_sqid_dn11_slot: &mut f64,
        var_sqid_dn2_slot: &mut f64,
        var_sqid_dn3_slot: &mut f64,
        var_sqid_dn4_slot: &mut f64,
        var_sqid_dn5_slot: &mut f64,
        var_sqid_dn6_slot: &mut f64,
        var_sqid_dn7_slot: &mut f64,
        var_sqid_dn8_slot: &mut f64,
        var_sqid_dn9_slot: &mut f64,
        var_sqig_slot: &mut f64,
        var_sqig_db0_slot: &mut f64,
        var_sqig_db1_slot: &mut f64,
        var_sqig_db2_slot: &mut f64,
        var_sqig_db3_slot: &mut f64,
        var_sqig_db4_slot: &mut f64,
        var_sqig_db5_slot: &mut f64,
        var_sqig_db6_slot: &mut f64,
        var_sqig_dn0_slot: &mut f64,
        var_sqig_dn1_slot: &mut f64,
        var_sqig_dn10_slot: &mut f64,
        var_sqig_dn11_slot: &mut f64,
        var_sqig_dn2_slot: &mut f64,
        var_sqig_dn3_slot: &mut f64,
        var_sqig_dn4_slot: &mut f64,
        var_sqig_dn5_slot: &mut f64,
        var_sqig_dn6_slot: &mut f64,
        var_sqig_dn7_slot: &mut f64,
        var_sqig_dn8_slot: &mut f64,
        var_sqig_dn9_slot: &mut f64,
        var_sqt2_slot: &mut f64,
        var_sqt2_db0_slot: &mut f64,
        var_sqt2_db1_slot: &mut f64,
        var_sqt2_db2_slot: &mut f64,
        var_sqt2_db3_slot: &mut f64,
        var_sqt2_db4_slot: &mut f64,
        var_sqt2_db5_slot: &mut f64,
        var_sqt2_db6_slot: &mut f64,
        var_sqt2_dn0_slot: &mut f64,
        var_sqt2_dn1_slot: &mut f64,
        var_sqt2_dn10_slot: &mut f64,
        var_sqt2_dn11_slot: &mut f64,
        var_sqt2_dn2_slot: &mut f64,
        var_sqt2_dn3_slot: &mut f64,
        var_sqt2_dn4_slot: &mut f64,
        var_sqt2_dn5_slot: &mut f64,
        var_sqt2_dn6_slot: &mut f64,
        var_sqt2_dn7_slot: &mut f64,
        var_sqt2_dn8_slot: &mut f64,
        var_sqt2_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_db0_slot: &mut f64,
        var_t1_db1_slot: &mut f64,
        var_t1_db2_slot: &mut f64,
        var_t1_db3_slot: &mut f64,
        var_t1_db4_slot: &mut f64,
        var_t1_db5_slot: &mut f64,
        var_t1_db6_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_db0_slot: &mut f64,
        var_t2_db1_slot: &mut f64,
        var_t2_db2_slot: &mut f64,
        var_t2_db3_slot: &mut f64,
        var_t2_db4_slot: &mut f64,
        var_t2_db5_slot: &mut f64,
        var_t2_db6_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_temp__blk1726_slot: &mut f64,
        var_temp__blk1726_db0_slot: &mut f64,
        var_temp__blk1726_db1_slot: &mut f64,
        var_temp__blk1726_db2_slot: &mut f64,
        var_temp__blk1726_db3_slot: &mut f64,
        var_temp__blk1726_db4_slot: &mut f64,
        var_temp__blk1726_db5_slot: &mut f64,
        var_temp__blk1726_db6_slot: &mut f64,
        var_temp__blk1726_dn0_slot: &mut f64,
        var_temp__blk1726_dn1_slot: &mut f64,
        var_temp__blk1726_dn10_slot: &mut f64,
        var_temp__blk1726_dn11_slot: &mut f64,
        var_temp__blk1726_dn2_slot: &mut f64,
        var_temp__blk1726_dn3_slot: &mut f64,
        var_temp__blk1726_dn4_slot: &mut f64,
        var_temp__blk1726_dn5_slot: &mut f64,
        var_temp__blk1726_dn6_slot: &mut f64,
        var_temp__blk1726_dn7_slot: &mut f64,
        var_temp__blk1726_dn8_slot: &mut f64,
        var_temp__blk1726_dn9_slot: &mut f64,
    ) {
        let mut var_c_igid: f64 = *var_c_igid_slot;
        let mut var_c_igid_db0: f64 = *var_c_igid_db0_slot;
        let mut var_c_igid_db1: f64 = *var_c_igid_db1_slot;
        let mut var_c_igid_db2: f64 = *var_c_igid_db2_slot;
        let mut var_c_igid_db3: f64 = *var_c_igid_db3_slot;
        let mut var_c_igid_db4: f64 = *var_c_igid_db4_slot;
        let mut var_c_igid_db5: f64 = *var_c_igid_db5_slot;
        let mut var_c_igid_db6: f64 = *var_c_igid_db6_slot;
        let mut var_c_igid_dn0: f64 = *var_c_igid_dn0_slot;
        let mut var_c_igid_dn1: f64 = *var_c_igid_dn1_slot;
        let mut var_c_igid_dn10: f64 = *var_c_igid_dn10_slot;
        let mut var_c_igid_dn11: f64 = *var_c_igid_dn11_slot;
        let mut var_c_igid_dn2: f64 = *var_c_igid_dn2_slot;
        let mut var_c_igid_dn3: f64 = *var_c_igid_dn3_slot;
        let mut var_c_igid_dn4: f64 = *var_c_igid_dn4_slot;
        let mut var_c_igid_dn5: f64 = *var_c_igid_dn5_slot;
        let mut var_c_igid_dn6: f64 = *var_c_igid_dn6_slot;
        let mut var_c_igid_dn7: f64 = *var_c_igid_dn7_slot;
        let mut var_c_igid_dn8: f64 = *var_c_igid_dn8_slot;
        let mut var_c_igid_dn9: f64 = *var_c_igid_dn9_slot;
        let mut var_cgeff: f64 = *var_cgeff_slot;
        let mut var_cgeff_db0: f64 = *var_cgeff_db0_slot;
        let mut var_cgeff_db1: f64 = *var_cgeff_db1_slot;
        let mut var_cgeff_db2: f64 = *var_cgeff_db2_slot;
        let mut var_cgeff_db3: f64 = *var_cgeff_db3_slot;
        let mut var_cgeff_db4: f64 = *var_cgeff_db4_slot;
        let mut var_cgeff_db5: f64 = *var_cgeff_db5_slot;
        let mut var_cgeff_db6: f64 = *var_cgeff_db6_slot;
        let mut var_cgeff_dn0: f64 = *var_cgeff_dn0_slot;
        let mut var_cgeff_dn1: f64 = *var_cgeff_dn1_slot;
        let mut var_cgeff_dn10: f64 = *var_cgeff_dn10_slot;
        let mut var_cgeff_dn11: f64 = *var_cgeff_dn11_slot;
        let mut var_cgeff_dn2: f64 = *var_cgeff_dn2_slot;
        let mut var_cgeff_dn3: f64 = *var_cgeff_dn3_slot;
        let mut var_cgeff_dn4: f64 = *var_cgeff_dn4_slot;
        let mut var_cgeff_dn5: f64 = *var_cgeff_dn5_slot;
        let mut var_cgeff_dn6: f64 = *var_cgeff_dn6_slot;
        let mut var_cgeff_dn7: f64 = *var_cgeff_dn7_slot;
        let mut var_cgeff_dn8: f64 = *var_cgeff_dn8_slot;
        let mut var_cgeff_dn9: f64 = *var_cgeff_dn9_slot;
        let mut var_guard1727: f64 = *var_guard1727_slot;
        let mut var_guard1760: f64 = *var_guard1760_slot;
        let mut var_guard1762: f64 = *var_guard1762_slot;
        let mut var_h0: f64 = *var_h0_slot;
        let mut var_h0_db0: f64 = *var_h0_db0_slot;
        let mut var_h0_db1: f64 = *var_h0_db1_slot;
        let mut var_h0_db2: f64 = *var_h0_db2_slot;
        let mut var_h0_db3: f64 = *var_h0_db3_slot;
        let mut var_h0_db4: f64 = *var_h0_db4_slot;
        let mut var_h0_db5: f64 = *var_h0_db5_slot;
        let mut var_h0_db6: f64 = *var_h0_db6_slot;
        let mut var_h0_dn0: f64 = *var_h0_dn0_slot;
        let mut var_h0_dn1: f64 = *var_h0_dn1_slot;
        let mut var_h0_dn10: f64 = *var_h0_dn10_slot;
        let mut var_h0_dn11: f64 = *var_h0_dn11_slot;
        let mut var_h0_dn2: f64 = *var_h0_dn2_slot;
        let mut var_h0_dn3: f64 = *var_h0_dn3_slot;
        let mut var_h0_dn4: f64 = *var_h0_dn4_slot;
        let mut var_h0_dn5: f64 = *var_h0_dn5_slot;
        let mut var_h0_dn6: f64 = *var_h0_dn6_slot;
        let mut var_h0_dn7: f64 = *var_h0_dn7_slot;
        let mut var_h0_dn8: f64 = *var_h0_dn8_slot;
        let mut var_h0_dn9: f64 = *var_h0_dn9_slot;
        let mut var_mid: f64 = *var_mid_slot;
        let mut var_mid_db0: f64 = *var_mid_db0_slot;
        let mut var_mid_db1: f64 = *var_mid_db1_slot;
        let mut var_mid_db2: f64 = *var_mid_db2_slot;
        let mut var_mid_db3: f64 = *var_mid_db3_slot;
        let mut var_mid_db4: f64 = *var_mid_db4_slot;
        let mut var_mid_db5: f64 = *var_mid_db5_slot;
        let mut var_mid_db6: f64 = *var_mid_db6_slot;
        let mut var_mid_dn0: f64 = *var_mid_dn0_slot;
        let mut var_mid_dn1: f64 = *var_mid_dn1_slot;
        let mut var_mid_dn10: f64 = *var_mid_dn10_slot;
        let mut var_mid_dn11: f64 = *var_mid_dn11_slot;
        let mut var_mid_dn2: f64 = *var_mid_dn2_slot;
        let mut var_mid_dn3: f64 = *var_mid_dn3_slot;
        let mut var_mid_dn4: f64 = *var_mid_dn4_slot;
        let mut var_mid_dn5: f64 = *var_mid_dn5_slot;
        let mut var_mid_dn6: f64 = *var_mid_dn6_slot;
        let mut var_mid_dn7: f64 = *var_mid_dn7_slot;
        let mut var_mid_dn8: f64 = *var_mid_dn8_slot;
        let mut var_mid_dn9: f64 = *var_mid_dn9_slot;
        let mut var_mig: f64 = *var_mig_slot;
        let mut var_mig_db0: f64 = *var_mig_db0_slot;
        let mut var_mig_db1: f64 = *var_mig_db1_slot;
        let mut var_mig_db2: f64 = *var_mig_db2_slot;
        let mut var_mig_db3: f64 = *var_mig_db3_slot;
        let mut var_mig_db4: f64 = *var_mig_db4_slot;
        let mut var_mig_db5: f64 = *var_mig_db5_slot;
        let mut var_mig_db6: f64 = *var_mig_db6_slot;
        let mut var_mig_dn0: f64 = *var_mig_dn0_slot;
        let mut var_mig_dn1: f64 = *var_mig_dn1_slot;
        let mut var_mig_dn10: f64 = *var_mig_dn10_slot;
        let mut var_mig_dn11: f64 = *var_mig_dn11_slot;
        let mut var_mig_dn2: f64 = *var_mig_dn2_slot;
        let mut var_mig_dn3: f64 = *var_mig_dn3_slot;
        let mut var_mig_dn4: f64 = *var_mig_dn4_slot;
        let mut var_mig_dn5: f64 = *var_mig_dn5_slot;
        let mut var_mig_dn6: f64 = *var_mig_dn6_slot;
        let mut var_mig_dn7: f64 = *var_mig_dn7_slot;
        let mut var_mig_dn8: f64 = *var_mig_dn8_slot;
        let mut var_mig_dn9: f64 = *var_mig_dn9_slot;
        let mut var_migid: f64 = *var_migid_slot;
        let mut var_migid_db0: f64 = *var_migid_db0_slot;
        let mut var_migid_db1: f64 = *var_migid_db1_slot;
        let mut var_migid_db2: f64 = *var_migid_db2_slot;
        let mut var_migid_db3: f64 = *var_migid_db3_slot;
        let mut var_migid_db4: f64 = *var_migid_db4_slot;
        let mut var_migid_db5: f64 = *var_migid_db5_slot;
        let mut var_migid_db6: f64 = *var_migid_db6_slot;
        let mut var_migid_dn0: f64 = *var_migid_dn0_slot;
        let mut var_migid_dn1: f64 = *var_migid_dn1_slot;
        let mut var_migid_dn10: f64 = *var_migid_dn10_slot;
        let mut var_migid_dn11: f64 = *var_migid_dn11_slot;
        let mut var_migid_dn2: f64 = *var_migid_dn2_slot;
        let mut var_migid_dn3: f64 = *var_migid_dn3_slot;
        let mut var_migid_dn4: f64 = *var_migid_dn4_slot;
        let mut var_migid_dn5: f64 = *var_migid_dn5_slot;
        let mut var_migid_dn6: f64 = *var_migid_dn6_slot;
        let mut var_migid_dn7: f64 = *var_migid_dn7_slot;
        let mut var_migid_dn8: f64 = *var_migid_dn8_slot;
        let mut var_migid_dn9: f64 = *var_migid_dn9_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_db0: f64 = *var_qd_db0_slot;
        let mut var_qd_db1: f64 = *var_qd_db1_slot;
        let mut var_qd_db2: f64 = *var_qd_db2_slot;
        let mut var_qd_db3: f64 = *var_qd_db3_slot;
        let mut var_qd_db4: f64 = *var_qd_db4_slot;
        let mut var_qd_db5: f64 = *var_qd_db5_slot;
        let mut var_qd_db6: f64 = *var_qd_db6_slot;
        let mut var_qd_dn0: f64 = *var_qd_dn0_slot;
        let mut var_qd_dn1: f64 = *var_qd_dn1_slot;
        let mut var_qd_dn10: f64 = *var_qd_dn10_slot;
        let mut var_qd_dn11: f64 = *var_qd_dn11_slot;
        let mut var_qd_dn2: f64 = *var_qd_dn2_slot;
        let mut var_qd_dn3: f64 = *var_qd_dn3_slot;
        let mut var_qd_dn4: f64 = *var_qd_dn4_slot;
        let mut var_qd_dn5: f64 = *var_qd_dn5_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qd_dn9: f64 = *var_qd_dn9_slot;
        let mut var_qfgd: f64 = *var_qfgd_slot;
        let mut var_qfgd_db0: f64 = *var_qfgd_db0_slot;
        let mut var_qfgd_db1: f64 = *var_qfgd_db1_slot;
        let mut var_qfgd_db2: f64 = *var_qfgd_db2_slot;
        let mut var_qfgd_db3: f64 = *var_qfgd_db3_slot;
        let mut var_qfgd_db4: f64 = *var_qfgd_db4_slot;
        let mut var_qfgd_db5: f64 = *var_qfgd_db5_slot;
        let mut var_qfgd_db6: f64 = *var_qfgd_db6_slot;
        let mut var_qfgd_dn0: f64 = *var_qfgd_dn0_slot;
        let mut var_qfgd_dn1: f64 = *var_qfgd_dn1_slot;
        let mut var_qfgd_dn10: f64 = *var_qfgd_dn10_slot;
        let mut var_qfgd_dn11: f64 = *var_qfgd_dn11_slot;
        let mut var_qfgd_dn2: f64 = *var_qfgd_dn2_slot;
        let mut var_qfgd_dn3: f64 = *var_qfgd_dn3_slot;
        let mut var_qfgd_dn4: f64 = *var_qfgd_dn4_slot;
        let mut var_qfgd_dn5: f64 = *var_qfgd_dn5_slot;
        let mut var_qfgd_dn6: f64 = *var_qfgd_dn6_slot;
        let mut var_qfgd_dn7: f64 = *var_qfgd_dn7_slot;
        let mut var_qfgd_dn8: f64 = *var_qfgd_dn8_slot;
        let mut var_qfgd_dn9: f64 = *var_qfgd_dn9_slot;
        let mut var_qfgs: f64 = *var_qfgs_slot;
        let mut var_qfgs_db0: f64 = *var_qfgs_db0_slot;
        let mut var_qfgs_db1: f64 = *var_qfgs_db1_slot;
        let mut var_qfgs_db2: f64 = *var_qfgs_db2_slot;
        let mut var_qfgs_db3: f64 = *var_qfgs_db3_slot;
        let mut var_qfgs_db4: f64 = *var_qfgs_db4_slot;
        let mut var_qfgs_db5: f64 = *var_qfgs_db5_slot;
        let mut var_qfgs_db6: f64 = *var_qfgs_db6_slot;
        let mut var_qfgs_dn0: f64 = *var_qfgs_dn0_slot;
        let mut var_qfgs_dn1: f64 = *var_qfgs_dn1_slot;
        let mut var_qfgs_dn10: f64 = *var_qfgs_dn10_slot;
        let mut var_qfgs_dn11: f64 = *var_qfgs_dn11_slot;
        let mut var_qfgs_dn2: f64 = *var_qfgs_dn2_slot;
        let mut var_qfgs_dn3: f64 = *var_qfgs_dn3_slot;
        let mut var_qfgs_dn4: f64 = *var_qfgs_dn4_slot;
        let mut var_qfgs_dn5: f64 = *var_qfgs_dn5_slot;
        let mut var_qfgs_dn6: f64 = *var_qfgs_dn6_slot;
        let mut var_qfgs_dn7: f64 = *var_qfgs_dn7_slot;
        let mut var_qfgs_dn8: f64 = *var_qfgs_dn8_slot;
        let mut var_qfgs_dn9: f64 = *var_qfgs_dn9_slot;
        let mut var_qjun_d: f64 = *var_qjun_d_slot;
        let mut var_qjun_d_db0: f64 = *var_qjun_d_db0_slot;
        let mut var_qjun_d_db1: f64 = *var_qjun_d_db1_slot;
        let mut var_qjun_d_db2: f64 = *var_qjun_d_db2_slot;
        let mut var_qjun_d_db3: f64 = *var_qjun_d_db3_slot;
        let mut var_qjun_d_db4: f64 = *var_qjun_d_db4_slot;
        let mut var_qjun_d_db5: f64 = *var_qjun_d_db5_slot;
        let mut var_qjun_d_db6: f64 = *var_qjun_d_db6_slot;
        let mut var_qjun_d_dn0: f64 = *var_qjun_d_dn0_slot;
        let mut var_qjun_d_dn1: f64 = *var_qjun_d_dn1_slot;
        let mut var_qjun_d_dn10: f64 = *var_qjun_d_dn10_slot;
        let mut var_qjun_d_dn11: f64 = *var_qjun_d_dn11_slot;
        let mut var_qjun_d_dn2: f64 = *var_qjun_d_dn2_slot;
        let mut var_qjun_d_dn3: f64 = *var_qjun_d_dn3_slot;
        let mut var_qjun_d_dn4: f64 = *var_qjun_d_dn4_slot;
        let mut var_qjun_d_dn5: f64 = *var_qjun_d_dn5_slot;
        let mut var_qjun_d_dn6: f64 = *var_qjun_d_dn6_slot;
        let mut var_qjun_d_dn7: f64 = *var_qjun_d_dn7_slot;
        let mut var_qjun_d_dn8: f64 = *var_qjun_d_dn8_slot;
        let mut var_qjun_d_dn9: f64 = *var_qjun_d_dn9_slot;
        let mut var_qjun_s: f64 = *var_qjun_s_slot;
        let mut var_qjun_s_db0: f64 = *var_qjun_s_db0_slot;
        let mut var_qjun_s_db1: f64 = *var_qjun_s_db1_slot;
        let mut var_qjun_s_db2: f64 = *var_qjun_s_db2_slot;
        let mut var_qjun_s_db3: f64 = *var_qjun_s_db3_slot;
        let mut var_qjun_s_db4: f64 = *var_qjun_s_db4_slot;
        let mut var_qjun_s_db5: f64 = *var_qjun_s_db5_slot;
        let mut var_qjun_s_db6: f64 = *var_qjun_s_db6_slot;
        let mut var_qjun_s_dn0: f64 = *var_qjun_s_dn0_slot;
        let mut var_qjun_s_dn1: f64 = *var_qjun_s_dn1_slot;
        let mut var_qjun_s_dn10: f64 = *var_qjun_s_dn10_slot;
        let mut var_qjun_s_dn11: f64 = *var_qjun_s_dn11_slot;
        let mut var_qjun_s_dn2: f64 = *var_qjun_s_dn2_slot;
        let mut var_qjun_s_dn3: f64 = *var_qjun_s_dn3_slot;
        let mut var_qjun_s_dn4: f64 = *var_qjun_s_dn4_slot;
        let mut var_qjun_s_dn5: f64 = *var_qjun_s_dn5_slot;
        let mut var_qjun_s_dn6: f64 = *var_qjun_s_dn6_slot;
        let mut var_qjun_s_dn7: f64 = *var_qjun_s_dn7_slot;
        let mut var_qjun_s_dn8: f64 = *var_qjun_s_dn8_slot;
        let mut var_qjun_s_dn9: f64 = *var_qjun_s_dn9_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_db0: f64 = *var_qs_db0_slot;
        let mut var_qs_db1: f64 = *var_qs_db1_slot;
        let mut var_qs_db2: f64 = *var_qs_db2_slot;
        let mut var_qs_db3: f64 = *var_qs_db3_slot;
        let mut var_qs_db4: f64 = *var_qs_db4_slot;
        let mut var_qs_db5: f64 = *var_qs_db5_slot;
        let mut var_qs_db6: f64 = *var_qs_db6_slot;
        let mut var_qs_dn0: f64 = *var_qs_dn0_slot;
        let mut var_qs_dn1: f64 = *var_qs_dn1_slot;
        let mut var_qs_dn10: f64 = *var_qs_dn10_slot;
        let mut var_qs_dn11: f64 = *var_qs_dn11_slot;
        let mut var_qs_dn2: f64 = *var_qs_dn2_slot;
        let mut var_qs_dn3: f64 = *var_qs_dn3_slot;
        let mut var_qs_dn4: f64 = *var_qs_dn4_slot;
        let mut var_qs_dn5: f64 = *var_qs_dn5_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_qs_dn9: f64 = *var_qs_dn9_slot;
        let mut var_r: f64 = *var_r_slot;
        let mut var_r_db0: f64 = *var_r_db0_slot;
        let mut var_r_db1: f64 = *var_r_db1_slot;
        let mut var_r_db2: f64 = *var_r_db2_slot;
        let mut var_r_db3: f64 = *var_r_db3_slot;
        let mut var_r_db4: f64 = *var_r_db4_slot;
        let mut var_r_db5: f64 = *var_r_db5_slot;
        let mut var_r_db6: f64 = *var_r_db6_slot;
        let mut var_r_dn0: f64 = *var_r_dn0_slot;
        let mut var_r_dn1: f64 = *var_r_dn1_slot;
        let mut var_r_dn10: f64 = *var_r_dn10_slot;
        let mut var_r_dn11: f64 = *var_r_dn11_slot;
        let mut var_r_dn2: f64 = *var_r_dn2_slot;
        let mut var_r_dn3: f64 = *var_r_dn3_slot;
        let mut var_r_dn4: f64 = *var_r_dn4_slot;
        let mut var_r_dn5: f64 = *var_r_dn5_slot;
        let mut var_r_dn6: f64 = *var_r_dn6_slot;
        let mut var_r_dn7: f64 = *var_r_dn7_slot;
        let mut var_r_dn8: f64 = *var_r_dn8_slot;
        let mut var_r_dn9: f64 = *var_r_dn9_slot;
        let mut var_sidexc: f64 = *var_sidexc_slot;
        let mut var_sidexc_db0: f64 = *var_sidexc_db0_slot;
        let mut var_sidexc_db1: f64 = *var_sidexc_db1_slot;
        let mut var_sidexc_db2: f64 = *var_sidexc_db2_slot;
        let mut var_sidexc_db3: f64 = *var_sidexc_db3_slot;
        let mut var_sidexc_db4: f64 = *var_sidexc_db4_slot;
        let mut var_sidexc_db5: f64 = *var_sidexc_db5_slot;
        let mut var_sidexc_db6: f64 = *var_sidexc_db6_slot;
        let mut var_sidexc_dn0: f64 = *var_sidexc_dn0_slot;
        let mut var_sidexc_dn1: f64 = *var_sidexc_dn1_slot;
        let mut var_sidexc_dn10: f64 = *var_sidexc_dn10_slot;
        let mut var_sidexc_dn11: f64 = *var_sidexc_dn11_slot;
        let mut var_sidexc_dn2: f64 = *var_sidexc_dn2_slot;
        let mut var_sidexc_dn3: f64 = *var_sidexc_dn3_slot;
        let mut var_sidexc_dn4: f64 = *var_sidexc_dn4_slot;
        let mut var_sidexc_dn5: f64 = *var_sidexc_dn5_slot;
        let mut var_sidexc_dn6: f64 = *var_sidexc_dn6_slot;
        let mut var_sidexc_dn7: f64 = *var_sidexc_dn7_slot;
        let mut var_sidexc_dn8: f64 = *var_sidexc_dn8_slot;
        let mut var_sidexc_dn9: f64 = *var_sidexc_dn9_slot;
        let mut var_sqid: f64 = *var_sqid_slot;
        let mut var_sqid_db0: f64 = *var_sqid_db0_slot;
        let mut var_sqid_db1: f64 = *var_sqid_db1_slot;
        let mut var_sqid_db2: f64 = *var_sqid_db2_slot;
        let mut var_sqid_db3: f64 = *var_sqid_db3_slot;
        let mut var_sqid_db4: f64 = *var_sqid_db4_slot;
        let mut var_sqid_db5: f64 = *var_sqid_db5_slot;
        let mut var_sqid_db6: f64 = *var_sqid_db6_slot;
        let mut var_sqid_dn0: f64 = *var_sqid_dn0_slot;
        let mut var_sqid_dn1: f64 = *var_sqid_dn1_slot;
        let mut var_sqid_dn10: f64 = *var_sqid_dn10_slot;
        let mut var_sqid_dn11: f64 = *var_sqid_dn11_slot;
        let mut var_sqid_dn2: f64 = *var_sqid_dn2_slot;
        let mut var_sqid_dn3: f64 = *var_sqid_dn3_slot;
        let mut var_sqid_dn4: f64 = *var_sqid_dn4_slot;
        let mut var_sqid_dn5: f64 = *var_sqid_dn5_slot;
        let mut var_sqid_dn6: f64 = *var_sqid_dn6_slot;
        let mut var_sqid_dn7: f64 = *var_sqid_dn7_slot;
        let mut var_sqid_dn8: f64 = *var_sqid_dn8_slot;
        let mut var_sqid_dn9: f64 = *var_sqid_dn9_slot;
        let mut var_sqig: f64 = *var_sqig_slot;
        let mut var_sqig_db0: f64 = *var_sqig_db0_slot;
        let mut var_sqig_db1: f64 = *var_sqig_db1_slot;
        let mut var_sqig_db2: f64 = *var_sqig_db2_slot;
        let mut var_sqig_db3: f64 = *var_sqig_db3_slot;
        let mut var_sqig_db4: f64 = *var_sqig_db4_slot;
        let mut var_sqig_db5: f64 = *var_sqig_db5_slot;
        let mut var_sqig_db6: f64 = *var_sqig_db6_slot;
        let mut var_sqig_dn0: f64 = *var_sqig_dn0_slot;
        let mut var_sqig_dn1: f64 = *var_sqig_dn1_slot;
        let mut var_sqig_dn10: f64 = *var_sqig_dn10_slot;
        let mut var_sqig_dn11: f64 = *var_sqig_dn11_slot;
        let mut var_sqig_dn2: f64 = *var_sqig_dn2_slot;
        let mut var_sqig_dn3: f64 = *var_sqig_dn3_slot;
        let mut var_sqig_dn4: f64 = *var_sqig_dn4_slot;
        let mut var_sqig_dn5: f64 = *var_sqig_dn5_slot;
        let mut var_sqig_dn6: f64 = *var_sqig_dn6_slot;
        let mut var_sqig_dn7: f64 = *var_sqig_dn7_slot;
        let mut var_sqig_dn8: f64 = *var_sqig_dn8_slot;
        let mut var_sqig_dn9: f64 = *var_sqig_dn9_slot;
        let mut var_sqt2: f64 = *var_sqt2_slot;
        let mut var_sqt2_db0: f64 = *var_sqt2_db0_slot;
        let mut var_sqt2_db1: f64 = *var_sqt2_db1_slot;
        let mut var_sqt2_db2: f64 = *var_sqt2_db2_slot;
        let mut var_sqt2_db3: f64 = *var_sqt2_db3_slot;
        let mut var_sqt2_db4: f64 = *var_sqt2_db4_slot;
        let mut var_sqt2_db5: f64 = *var_sqt2_db5_slot;
        let mut var_sqt2_db6: f64 = *var_sqt2_db6_slot;
        let mut var_sqt2_dn0: f64 = *var_sqt2_dn0_slot;
        let mut var_sqt2_dn1: f64 = *var_sqt2_dn1_slot;
        let mut var_sqt2_dn10: f64 = *var_sqt2_dn10_slot;
        let mut var_sqt2_dn11: f64 = *var_sqt2_dn11_slot;
        let mut var_sqt2_dn2: f64 = *var_sqt2_dn2_slot;
        let mut var_sqt2_dn3: f64 = *var_sqt2_dn3_slot;
        let mut var_sqt2_dn4: f64 = *var_sqt2_dn4_slot;
        let mut var_sqt2_dn5: f64 = *var_sqt2_dn5_slot;
        let mut var_sqt2_dn6: f64 = *var_sqt2_dn6_slot;
        let mut var_sqt2_dn7: f64 = *var_sqt2_dn7_slot;
        let mut var_sqt2_dn8: f64 = *var_sqt2_dn8_slot;
        let mut var_sqt2_dn9: f64 = *var_sqt2_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_db0: f64 = *var_t1_db0_slot;
        let mut var_t1_db1: f64 = *var_t1_db1_slot;
        let mut var_t1_db2: f64 = *var_t1_db2_slot;
        let mut var_t1_db3: f64 = *var_t1_db3_slot;
        let mut var_t1_db4: f64 = *var_t1_db4_slot;
        let mut var_t1_db5: f64 = *var_t1_db5_slot;
        let mut var_t1_db6: f64 = *var_t1_db6_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_db0: f64 = *var_t2_db0_slot;
        let mut var_t2_db1: f64 = *var_t2_db1_slot;
        let mut var_t2_db2: f64 = *var_t2_db2_slot;
        let mut var_t2_db3: f64 = *var_t2_db3_slot;
        let mut var_t2_db4: f64 = *var_t2_db4_slot;
        let mut var_t2_db5: f64 = *var_t2_db5_slot;
        let mut var_t2_db6: f64 = *var_t2_db6_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_temp__blk1726: f64 = *var_temp__blk1726_slot;
        let mut var_temp__blk1726_db0: f64 = *var_temp__blk1726_db0_slot;
        let mut var_temp__blk1726_db1: f64 = *var_temp__blk1726_db1_slot;
        let mut var_temp__blk1726_db2: f64 = *var_temp__blk1726_db2_slot;
        let mut var_temp__blk1726_db3: f64 = *var_temp__blk1726_db3_slot;
        let mut var_temp__blk1726_db4: f64 = *var_temp__blk1726_db4_slot;
        let mut var_temp__blk1726_db5: f64 = *var_temp__blk1726_db5_slot;
        let mut var_temp__blk1726_db6: f64 = *var_temp__blk1726_db6_slot;
        let mut var_temp__blk1726_dn0: f64 = *var_temp__blk1726_dn0_slot;
        let mut var_temp__blk1726_dn1: f64 = *var_temp__blk1726_dn1_slot;
        let mut var_temp__blk1726_dn10: f64 = *var_temp__blk1726_dn10_slot;
        let mut var_temp__blk1726_dn11: f64 = *var_temp__blk1726_dn11_slot;
        let mut var_temp__blk1726_dn2: f64 = *var_temp__blk1726_dn2_slot;
        let mut var_temp__blk1726_dn3: f64 = *var_temp__blk1726_dn3_slot;
        let mut var_temp__blk1726_dn4: f64 = *var_temp__blk1726_dn4_slot;
        let mut var_temp__blk1726_dn5: f64 = *var_temp__blk1726_dn5_slot;
        let mut var_temp__blk1726_dn6: f64 = *var_temp__blk1726_dn6_slot;
        let mut var_temp__blk1726_dn7: f64 = *var_temp__blk1726_dn7_slot;
        let mut var_temp__blk1726_dn8: f64 = *var_temp__blk1726_dn8_slot;
        let mut var_temp__blk1726_dn9: f64 = *var_temp__blk1726_dn9_slot;

        let assign61980_e80539: f64 = (var_qfgs + var_qgs_ov);
        var_qfgs = assign61980_e80539;
        var_qfgs_dn0 = (var_qfgs_dn0 + var_qgs_ov_dn0);
        var_qfgs_dn1 = (var_qfgs_dn1 + var_qgs_ov_dn1);
        var_qfgs_dn2 = (var_qfgs_dn2 + var_qgs_ov_dn2);
        var_qfgs_dn3 = (var_qfgs_dn3 + var_qgs_ov_dn3);
        var_qfgs_dn4 = (var_qfgs_dn4 + var_qgs_ov_dn4);
        var_qfgs_dn5 = (var_qfgs_dn5 + var_qgs_ov_dn5);
        var_qfgs_dn6 = (var_qfgs_dn6 + var_qgs_ov_dn6);
        var_qfgs_dn7 = (var_qfgs_dn7 + var_qgs_ov_dn7);
        var_qfgs_dn8 = (var_qfgs_dn8 + var_qgs_ov_dn8);
        var_qfgs_dn9 = (var_qfgs_dn9 + var_qgs_ov_dn9);
        var_qfgs_dn10 = (var_qfgs_dn10 + var_qgs_ov_dn10);
        var_qfgs_dn11 = (var_qfgs_dn11 + var_qgs_ov_dn11);
        var_qfgs_db0 = (var_qfgs_db0 + var_qgs_ov_db0);
        var_qfgs_db1 = (var_qfgs_db1 + var_qgs_ov_db1);
        var_qfgs_db2 = (var_qfgs_db2 + var_qgs_ov_db2);
        var_qfgs_db3 = (var_qfgs_db3 + var_qgs_ov_db3);
        var_qfgs_db4 = (var_qfgs_db4 + var_qgs_ov_db4);
        var_qfgs_db5 = (var_qfgs_db5 + var_qgs_ov_db5);
        var_qfgs_db6 = (var_qfgs_db6 + var_qgs_ov_db6);

        let assign61990_e80542: f64 = (var_qfgd + var_qgd_ov);
        var_qfgd = assign61990_e80542;
        var_qfgd_dn0 = (var_qfgd_dn0 + var_qgd_ov_dn0);
        var_qfgd_dn1 = (var_qfgd_dn1 + var_qgd_ov_dn1);
        var_qfgd_dn2 = (var_qfgd_dn2 + var_qgd_ov_dn2);
        var_qfgd_dn3 = (var_qfgd_dn3 + var_qgd_ov_dn3);
        var_qfgd_dn4 = (var_qfgd_dn4 + var_qgd_ov_dn4);
        var_qfgd_dn5 = (var_qfgd_dn5 + var_qgd_ov_dn5);
        var_qfgd_dn6 = (var_qfgd_dn6 + var_qgd_ov_dn6);
        var_qfgd_dn7 = (var_qfgd_dn7 + var_qgd_ov_dn7);
        var_qfgd_dn8 = (var_qfgd_dn8 + var_qgd_ov_dn8);
        var_qfgd_dn9 = (var_qfgd_dn9 + var_qgd_ov_dn9);
        var_qfgd_dn10 = (var_qfgd_dn10 + var_qgd_ov_dn10);
        var_qfgd_dn11 = (var_qfgd_dn11 + var_qgd_ov_dn11);
        var_qfgd_db0 = (var_qfgd_db0 + var_qgd_ov_db0);
        var_qfgd_db1 = (var_qfgd_db1 + var_qgd_ov_db1);
        var_qfgd_db2 = (var_qfgd_db2 + var_qgd_ov_db2);
        var_qfgd_db3 = (var_qfgd_db3 + var_qgd_ov_db3);
        var_qfgd_db4 = (var_qfgd_db4 + var_qgd_ov_db4);
        var_qfgd_db5 = (var_qfgd_db5 + var_qgd_ov_db5);
        var_qfgd_db6 = (var_qfgd_db6 + var_qgd_ov_db6);

        let assign62000_e80545: f64 = (var_absource_i * var_qjunbot_s);
        let assign62000_e80548: f64 = (var_lssource_i * var_qjunsti_s);
        let assign62000_e80549: f64 = (assign62000_e80545 + assign62000_e80548);
        let assign62000_e80552: f64 = (var_lgsource_i * var_qjungat_s);
        let assign62000_e80553: f64 = (assign62000_e80549 + assign62000_e80552);
        var_qjun_s = assign62000_e80553;
        var_qjun_s_dn0 = (((var_absource_i * var_qjunbot_s_dn0) + (var_lssource_i * var_qjunsti_s_dn0)) + (var_lgsource_i * var_qjungat_s_dn0));
        var_qjun_s_dn1 = (((var_absource_i * var_qjunbot_s_dn1) + (var_lssource_i * var_qjunsti_s_dn1)) + (var_lgsource_i * var_qjungat_s_dn1));
        var_qjun_s_dn2 = (((var_absource_i * var_qjunbot_s_dn2) + (var_lssource_i * var_qjunsti_s_dn2)) + (var_lgsource_i * var_qjungat_s_dn2));
        var_qjun_s_dn3 = (((var_absource_i * var_qjunbot_s_dn3) + (var_lssource_i * var_qjunsti_s_dn3)) + (var_lgsource_i * var_qjungat_s_dn3));
        var_qjun_s_dn4 = (((var_absource_i * var_qjunbot_s_dn4) + (var_lssource_i * var_qjunsti_s_dn4)) + (var_lgsource_i * var_qjungat_s_dn4));
        var_qjun_s_dn5 = (((var_absource_i * var_qjunbot_s_dn5) + (var_lssource_i * var_qjunsti_s_dn5)) + (var_lgsource_i * var_qjungat_s_dn5));
        var_qjun_s_dn6 = (((var_absource_i * var_qjunbot_s_dn6) + (var_lssource_i * var_qjunsti_s_dn6)) + (var_lgsource_i * var_qjungat_s_dn6));
        var_qjun_s_dn7 = (((var_absource_i * var_qjunbot_s_dn7) + (var_lssource_i * var_qjunsti_s_dn7)) + (var_lgsource_i * var_qjungat_s_dn7));
        var_qjun_s_dn8 = (((var_absource_i * var_qjunbot_s_dn8) + (var_lssource_i * var_qjunsti_s_dn8)) + (var_lgsource_i * var_qjungat_s_dn8));
        var_qjun_s_dn9 = (((var_absource_i * var_qjunbot_s_dn9) + (var_lssource_i * var_qjunsti_s_dn9)) + (var_lgsource_i * var_qjungat_s_dn9));
        var_qjun_s_dn10 = (((var_absource_i * var_qjunbot_s_dn10) + (var_lssource_i * var_qjunsti_s_dn10)) + (var_lgsource_i * var_qjungat_s_dn10));
        var_qjun_s_dn11 = (((var_absource_i * var_qjunbot_s_dn11) + (var_lssource_i * var_qjunsti_s_dn11)) + (var_lgsource_i * var_qjungat_s_dn11));
        var_qjun_s_db0 = (((var_absource_i * var_qjunbot_s_db0) + (var_lssource_i * var_qjunsti_s_db0)) + (var_lgsource_i * var_qjungat_s_db0));
        var_qjun_s_db1 = (((var_absource_i * var_qjunbot_s_db1) + (var_lssource_i * var_qjunsti_s_db1)) + (var_lgsource_i * var_qjungat_s_db1));
        var_qjun_s_db2 = (((var_absource_i * var_qjunbot_s_db2) + (var_lssource_i * var_qjunsti_s_db2)) + (var_lgsource_i * var_qjungat_s_db2));
        var_qjun_s_db3 = (((var_absource_i * var_qjunbot_s_db3) + (var_lssource_i * var_qjunsti_s_db3)) + (var_lgsource_i * var_qjungat_s_db3));
        var_qjun_s_db4 = (((var_absource_i * var_qjunbot_s_db4) + (var_lssource_i * var_qjunsti_s_db4)) + (var_lgsource_i * var_qjungat_s_db4));
        var_qjun_s_db5 = (((var_absource_i * var_qjunbot_s_db5) + (var_lssource_i * var_qjunsti_s_db5)) + (var_lgsource_i * var_qjungat_s_db5));
        var_qjun_s_db6 = (((var_absource_i * var_qjunbot_s_db6) + (var_lssource_i * var_qjunsti_s_db6)) + (var_lgsource_i * var_qjungat_s_db6));

        let assign62010_e80556: f64 = (var_abdrain_i * var_qjunbot_d);
        let assign62010_e80559: f64 = (var_lsdrain_i * var_qjunsti_d);
        let assign62010_e80560: f64 = (assign62010_e80556 + assign62010_e80559);
        let assign62010_e80563: f64 = (var_lgdrain_i * var_qjungat_d);
        let assign62010_e80564: f64 = (assign62010_e80560 + assign62010_e80563);
        var_qjun_d = assign62010_e80564;
        var_qjun_d_dn0 = (((var_abdrain_i * var_qjunbot_d_dn0) + (var_lsdrain_i * var_qjunsti_d_dn0)) + (var_lgdrain_i * var_qjungat_d_dn0));
        var_qjun_d_dn1 = (((var_abdrain_i * var_qjunbot_d_dn1) + (var_lsdrain_i * var_qjunsti_d_dn1)) + (var_lgdrain_i * var_qjungat_d_dn1));
        var_qjun_d_dn2 = (((var_abdrain_i * var_qjunbot_d_dn2) + (var_lsdrain_i * var_qjunsti_d_dn2)) + (var_lgdrain_i * var_qjungat_d_dn2));
        var_qjun_d_dn3 = (((var_abdrain_i * var_qjunbot_d_dn3) + (var_lsdrain_i * var_qjunsti_d_dn3)) + (var_lgdrain_i * var_qjungat_d_dn3));
        var_qjun_d_dn4 = (((var_abdrain_i * var_qjunbot_d_dn4) + (var_lsdrain_i * var_qjunsti_d_dn4)) + (var_lgdrain_i * var_qjungat_d_dn4));
        var_qjun_d_dn5 = (((var_abdrain_i * var_qjunbot_d_dn5) + (var_lsdrain_i * var_qjunsti_d_dn5)) + (var_lgdrain_i * var_qjungat_d_dn5));
        var_qjun_d_dn6 = (((var_abdrain_i * var_qjunbot_d_dn6) + (var_lsdrain_i * var_qjunsti_d_dn6)) + (var_lgdrain_i * var_qjungat_d_dn6));
        var_qjun_d_dn7 = (((var_abdrain_i * var_qjunbot_d_dn7) + (var_lsdrain_i * var_qjunsti_d_dn7)) + (var_lgdrain_i * var_qjungat_d_dn7));
        var_qjun_d_dn8 = (((var_abdrain_i * var_qjunbot_d_dn8) + (var_lsdrain_i * var_qjunsti_d_dn8)) + (var_lgdrain_i * var_qjungat_d_dn8));
        var_qjun_d_dn9 = (((var_abdrain_i * var_qjunbot_d_dn9) + (var_lsdrain_i * var_qjunsti_d_dn9)) + (var_lgdrain_i * var_qjungat_d_dn9));
        var_qjun_d_dn10 = (((var_abdrain_i * var_qjunbot_d_dn10) + (var_lsdrain_i * var_qjunsti_d_dn10)) + (var_lgdrain_i * var_qjungat_d_dn10));
        var_qjun_d_dn11 = (((var_abdrain_i * var_qjunbot_d_dn11) + (var_lsdrain_i * var_qjunsti_d_dn11)) + (var_lgdrain_i * var_qjungat_d_dn11));
        var_qjun_d_db0 = (((var_abdrain_i * var_qjunbot_d_db0) + (var_lsdrain_i * var_qjunsti_d_db0)) + (var_lgdrain_i * var_qjungat_d_db0));
        var_qjun_d_db1 = (((var_abdrain_i * var_qjunbot_d_db1) + (var_lsdrain_i * var_qjunsti_d_db1)) + (var_lgdrain_i * var_qjungat_d_db1));
        var_qjun_d_db2 = (((var_abdrain_i * var_qjunbot_d_db2) + (var_lsdrain_i * var_qjunsti_d_db2)) + (var_lgdrain_i * var_qjungat_d_db2));
        var_qjun_d_db3 = (((var_abdrain_i * var_qjunbot_d_db3) + (var_lsdrain_i * var_qjunsti_d_db3)) + (var_lgdrain_i * var_qjungat_d_db3));
        var_qjun_d_db4 = (((var_abdrain_i * var_qjunbot_d_db4) + (var_lsdrain_i * var_qjunsti_d_db4)) + (var_lgdrain_i * var_qjungat_d_db4));
        var_qjun_d_db5 = (((var_abdrain_i * var_qjunbot_d_db5) + (var_lsdrain_i * var_qjunsti_d_db5)) + (var_lgdrain_i * var_qjungat_d_db5));
        var_qjun_d_db6 = (((var_abdrain_i * var_qjunbot_d_db6) + (var_lsdrain_i * var_qjunsti_d_db6)) + (var_lgdrain_i * var_qjungat_d_db6));

        let assign62020_e80567: f64 = if var_sigvds < 0.0 { 1.0 } else { 0.0 };
        var_guard1727 = assign62020_e80567;

        let (assign62030_e80571, assign62030_e80571_d_n0, assign62030_e80571_d_n1, assign62030_e80571_d_n2, assign62030_e80571_d_n3, assign62030_e80571_d_n4, assign62030_e80571_d_n5, assign62030_e80571_d_n6, assign62030_e80571_d_n7, assign62030_e80571_d_n8, assign62030_e80571_d_n9, assign62030_e80571_d_n10, assign62030_e80571_d_n11, assign62030_e80571_d_b0, assign62030_e80571_d_b1, assign62030_e80571_d_b2, assign62030_e80571_d_b3, assign62030_e80571_d_b4, assign62030_e80571_d_b5, assign62030_e80571_d_b6,) = {
    if (var_guard1727 != 0.0) {
        (var_qd, var_qd_dn0, var_qd_dn1, var_qd_dn2, var_qd_dn3, var_qd_dn4, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_dn9, var_qd_dn10, var_qd_dn11, var_qd_db0, var_qd_db1, var_qd_db2, var_qd_db3, var_qd_db4, var_qd_db5, var_qd_db6,)
    } else {
        (var_temp__blk1726, var_temp__blk1726_dn0, var_temp__blk1726_dn1, var_temp__blk1726_dn2, var_temp__blk1726_dn3, var_temp__blk1726_dn4, var_temp__blk1726_dn5, var_temp__blk1726_dn6, var_temp__blk1726_dn7, var_temp__blk1726_dn8, var_temp__blk1726_dn9, var_temp__blk1726_dn10, var_temp__blk1726_dn11, var_temp__blk1726_db0, var_temp__blk1726_db1, var_temp__blk1726_db2, var_temp__blk1726_db3, var_temp__blk1726_db4, var_temp__blk1726_db5, var_temp__blk1726_db6,)
    }
};
        var_temp__blk1726 = assign62030_e80571;
        var_temp__blk1726_dn0 = assign62030_e80571_d_n0;
        var_temp__blk1726_dn1 = assign62030_e80571_d_n1;
        var_temp__blk1726_dn2 = assign62030_e80571_d_n2;
        var_temp__blk1726_dn3 = assign62030_e80571_d_n3;
        var_temp__blk1726_dn4 = assign62030_e80571_d_n4;
        var_temp__blk1726_dn5 = assign62030_e80571_d_n5;
        var_temp__blk1726_dn6 = assign62030_e80571_d_n6;
        var_temp__blk1726_dn7 = assign62030_e80571_d_n7;
        var_temp__blk1726_dn8 = assign62030_e80571_d_n8;
        var_temp__blk1726_dn9 = assign62030_e80571_d_n9;
        var_temp__blk1726_dn10 = assign62030_e80571_d_n10;
        var_temp__blk1726_dn11 = assign62030_e80571_d_n11;
        var_temp__blk1726_db0 = assign62030_e80571_d_b0;
        var_temp__blk1726_db1 = assign62030_e80571_d_b1;
        var_temp__blk1726_db2 = assign62030_e80571_d_b2;
        var_temp__blk1726_db3 = assign62030_e80571_d_b3;
        var_temp__blk1726_db4 = assign62030_e80571_d_b4;
        var_temp__blk1726_db5 = assign62030_e80571_d_b5;
        var_temp__blk1726_db6 = assign62030_e80571_d_b6;

        let (assign62040_e80575, assign62040_e80575_d_n0, assign62040_e80575_d_n1, assign62040_e80575_d_n2, assign62040_e80575_d_n3, assign62040_e80575_d_n4, assign62040_e80575_d_n5, assign62040_e80575_d_n6, assign62040_e80575_d_n7, assign62040_e80575_d_n8, assign62040_e80575_d_n9, assign62040_e80575_d_n10, assign62040_e80575_d_n11, assign62040_e80575_d_b0, assign62040_e80575_d_b1, assign62040_e80575_d_b2, assign62040_e80575_d_b3, assign62040_e80575_d_b4, assign62040_e80575_d_b5, assign62040_e80575_d_b6,) = {
    if (var_guard1727 != 0.0) {
        (var_qs, var_qs_dn0, var_qs_dn1, var_qs_dn2, var_qs_dn3, var_qs_dn4, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_dn9, var_qs_dn10, var_qs_dn11, var_qs_db0, var_qs_db1, var_qs_db2, var_qs_db3, var_qs_db4, var_qs_db5, var_qs_db6,)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn1, var_qd_dn2, var_qd_dn3, var_qd_dn4, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_dn9, var_qd_dn10, var_qd_dn11, var_qd_db0, var_qd_db1, var_qd_db2, var_qd_db3, var_qd_db4, var_qd_db5, var_qd_db6,)
    }
};
        var_qd = assign62040_e80575;
        var_qd_dn0 = assign62040_e80575_d_n0;
        var_qd_dn1 = assign62040_e80575_d_n1;
        var_qd_dn2 = assign62040_e80575_d_n2;
        var_qd_dn3 = assign62040_e80575_d_n3;
        var_qd_dn4 = assign62040_e80575_d_n4;
        var_qd_dn5 = assign62040_e80575_d_n5;
        var_qd_dn6 = assign62040_e80575_d_n6;
        var_qd_dn7 = assign62040_e80575_d_n7;
        var_qd_dn8 = assign62040_e80575_d_n8;
        var_qd_dn9 = assign62040_e80575_d_n9;
        var_qd_dn10 = assign62040_e80575_d_n10;
        var_qd_dn11 = assign62040_e80575_d_n11;
        var_qd_db0 = assign62040_e80575_d_b0;
        var_qd_db1 = assign62040_e80575_d_b1;
        var_qd_db2 = assign62040_e80575_d_b2;
        var_qd_db3 = assign62040_e80575_d_b3;
        var_qd_db4 = assign62040_e80575_d_b4;
        var_qd_db5 = assign62040_e80575_d_b5;
        var_qd_db6 = assign62040_e80575_d_b6;

        let (assign62050_e80579, assign62050_e80579_d_n0, assign62050_e80579_d_n1, assign62050_e80579_d_n2, assign62050_e80579_d_n3, assign62050_e80579_d_n4, assign62050_e80579_d_n5, assign62050_e80579_d_n6, assign62050_e80579_d_n7, assign62050_e80579_d_n8, assign62050_e80579_d_n9, assign62050_e80579_d_n10, assign62050_e80579_d_n11, assign62050_e80579_d_b0, assign62050_e80579_d_b1, assign62050_e80579_d_b2, assign62050_e80579_d_b3, assign62050_e80579_d_b4, assign62050_e80579_d_b5, assign62050_e80579_d_b6,) = {
    if (var_guard1727 != 0.0) {
        (var_temp__blk1726, var_temp__blk1726_dn0, var_temp__blk1726_dn1, var_temp__blk1726_dn2, var_temp__blk1726_dn3, var_temp__blk1726_dn4, var_temp__blk1726_dn5, var_temp__blk1726_dn6, var_temp__blk1726_dn7, var_temp__blk1726_dn8, var_temp__blk1726_dn9, var_temp__blk1726_dn10, var_temp__blk1726_dn11, var_temp__blk1726_db0, var_temp__blk1726_db1, var_temp__blk1726_db2, var_temp__blk1726_db3, var_temp__blk1726_db4, var_temp__blk1726_db5, var_temp__blk1726_db6,)
    } else {
        (var_qs, var_qs_dn0, var_qs_dn1, var_qs_dn2, var_qs_dn3, var_qs_dn4, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_dn9, var_qs_dn10, var_qs_dn11, var_qs_db0, var_qs_db1, var_qs_db2, var_qs_db3, var_qs_db4, var_qs_db5, var_qs_db6,)
    }
};
        var_qs = assign62050_e80579;
        var_qs_dn0 = assign62050_e80579_d_n0;
        var_qs_dn1 = assign62050_e80579_d_n1;
        var_qs_dn2 = assign62050_e80579_d_n2;
        var_qs_dn3 = assign62050_e80579_d_n3;
        var_qs_dn4 = assign62050_e80579_d_n4;
        var_qs_dn5 = assign62050_e80579_d_n5;
        var_qs_dn6 = assign62050_e80579_d_n6;
        var_qs_dn7 = assign62050_e80579_d_n7;
        var_qs_dn8 = assign62050_e80579_d_n8;
        var_qs_dn9 = assign62050_e80579_d_n9;
        var_qs_dn10 = assign62050_e80579_d_n10;
        var_qs_dn11 = assign62050_e80579_d_n11;
        var_qs_db0 = assign62050_e80579_d_b0;
        var_qs_db1 = assign62050_e80579_d_b1;
        var_qs_db2 = assign62050_e80579_d_b2;
        var_qs_db3 = assign62050_e80579_d_b3;
        var_qs_db4 = assign62050_e80579_d_b4;
        var_qs_db5 = assign62050_e80579_d_b5;
        var_qs_db6 = assign62050_e80579_d_b6;

        var_sidexc = 0.0;
        var_sidexc_dn0 = 0.0;
        var_sidexc_dn1 = 0.0;
        var_sidexc_dn2 = 0.0;
        var_sidexc_dn3 = 0.0;
        var_sidexc_dn4 = 0.0;
        var_sidexc_dn5 = 0.0;
        var_sidexc_dn6 = 0.0;
        var_sidexc_dn7 = 0.0;
        var_sidexc_dn8 = 0.0;
        var_sidexc_dn9 = 0.0;
        var_sidexc_dn10 = 0.0;
        var_sidexc_dn11 = 0.0;
        var_sidexc_db0 = 0.0;
        var_sidexc_db1 = 0.0;
        var_sidexc_db2 = 0.0;
        var_sidexc_db3 = 0.0;
        var_sidexc_db4 = 0.0;
        var_sidexc_db5 = 0.0;
        var_sidexc_db6 = 0.0;

        var_mid = 0.0;
        var_mid_dn0 = 0.0;
        var_mid_dn1 = 0.0;
        var_mid_dn2 = 0.0;
        var_mid_dn3 = 0.0;
        var_mid_dn4 = 0.0;
        var_mid_dn5 = 0.0;
        var_mid_dn6 = 0.0;
        var_mid_dn7 = 0.0;
        var_mid_dn8 = 0.0;
        var_mid_dn9 = 0.0;
        var_mid_dn10 = 0.0;
        var_mid_dn11 = 0.0;
        var_mid_db0 = 0.0;
        var_mid_db1 = 0.0;
        var_mid_db2 = 0.0;
        var_mid_db3 = 0.0;
        var_mid_db4 = 0.0;
        var_mid_db5 = 0.0;
        var_mid_db6 = 0.0;

        var_mig = 1e-40;
        var_mig_dn0 = 0.0;
        var_mig_dn1 = 0.0;
        var_mig_dn2 = 0.0;
        var_mig_dn3 = 0.0;
        var_mig_dn4 = 0.0;
        var_mig_dn5 = 0.0;
        var_mig_dn6 = 0.0;
        var_mig_dn7 = 0.0;
        var_mig_dn8 = 0.0;
        var_mig_dn9 = 0.0;
        var_mig_dn10 = 0.0;
        var_mig_dn11 = 0.0;
        var_mig_db0 = 0.0;
        var_mig_db1 = 0.0;
        var_mig_db2 = 0.0;
        var_mig_db3 = 0.0;
        var_mig_db4 = 0.0;
        var_mig_db5 = 0.0;
        var_mig_db6 = 0.0;

        var_migid = 0.0;
        var_migid_dn0 = 0.0;
        var_migid_dn1 = 0.0;
        var_migid_dn2 = 0.0;
        var_migid_dn3 = 0.0;
        var_migid_dn4 = 0.0;
        var_migid_dn5 = 0.0;
        var_migid_dn6 = 0.0;
        var_migid_dn7 = 0.0;
        var_migid_dn8 = 0.0;
        var_migid_dn9 = 0.0;
        var_migid_dn10 = 0.0;
        var_migid_dn11 = 0.0;
        var_migid_db0 = 0.0;
        var_migid_db1 = 0.0;
        var_migid_db2 = 0.0;
        var_migid_db3 = 0.0;
        var_migid_db4 = 0.0;
        var_migid_db5 = 0.0;
        var_migid_db6 = 0.0;

        var_c_igid = 0.0;
        var_c_igid_dn0 = 0.0;
        var_c_igid_dn1 = 0.0;
        var_c_igid_dn2 = 0.0;
        var_c_igid_dn3 = 0.0;
        var_c_igid_dn4 = 0.0;
        var_c_igid_dn5 = 0.0;
        var_c_igid_dn6 = 0.0;
        var_c_igid_dn7 = 0.0;
        var_c_igid_dn8 = 0.0;
        var_c_igid_dn9 = 0.0;
        var_c_igid_dn10 = 0.0;
        var_c_igid_dn11 = 0.0;
        var_c_igid_db0 = 0.0;
        var_c_igid_db1 = 0.0;
        var_c_igid_db2 = 0.0;
        var_c_igid_db3 = 0.0;
        var_c_igid_db4 = 0.0;
        var_c_igid_db5 = 0.0;
        var_c_igid_db6 = 0.0;

        let assign62120_e80588: f64 = (var_cox_qm * var_eta_p_ac);
        var_cgeff = assign62120_e80588;
        var_cgeff_dn0 = ((var_cox_qm_dn0 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn0));
        var_cgeff_dn1 = ((var_cox_qm_dn1 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn1));
        var_cgeff_dn2 = ((var_cox_qm_dn2 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn2));
        var_cgeff_dn3 = ((var_cox_qm_dn3 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn3));
        var_cgeff_dn4 = ((var_cox_qm_dn4 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn4));
        var_cgeff_dn5 = ((var_cox_qm_dn5 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn5));
        var_cgeff_dn6 = ((var_cox_qm_dn6 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn6));
        var_cgeff_dn7 = ((var_cox_qm_dn7 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn7));
        var_cgeff_dn8 = ((var_cox_qm_dn8 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn8));
        var_cgeff_dn9 = ((var_cox_qm_dn9 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn9));
        var_cgeff_dn10 = ((var_cox_qm_dn10 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn10));
        var_cgeff_dn11 = ((var_cox_qm_dn11 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn11));
        var_cgeff_db0 = ((var_cox_qm_db0 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_db0));
        var_cgeff_db1 = ((var_cox_qm_db1 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_db1));
        var_cgeff_db2 = ((var_cox_qm_db2 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_db2));
        var_cgeff_db3 = ((var_cox_qm_db3 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_db3));
        var_cgeff_db4 = ((var_cox_qm_db4 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_db4));
        var_cgeff_db5 = ((var_cox_qm_db5 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_db5));
        var_cgeff_db6 = ((var_cox_qm_db6 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_db6));

        var_sqid = 0.0;
        var_sqid_dn0 = 0.0;
        var_sqid_dn1 = 0.0;
        var_sqid_dn2 = 0.0;
        var_sqid_dn3 = 0.0;
        var_sqid_dn4 = 0.0;
        var_sqid_dn5 = 0.0;
        var_sqid_dn6 = 0.0;
        var_sqid_dn7 = 0.0;
        var_sqid_dn8 = 0.0;
        var_sqid_dn9 = 0.0;
        var_sqid_dn10 = 0.0;
        var_sqid_dn11 = 0.0;
        var_sqid_db0 = 0.0;
        var_sqid_db1 = 0.0;
        var_sqid_db2 = 0.0;
        var_sqid_db3 = 0.0;
        var_sqid_db4 = 0.0;
        var_sqid_db5 = 0.0;
        var_sqid_db6 = 0.0;

        var_sqig = 0.0;
        var_sqig_dn0 = 0.0;
        var_sqig_dn1 = 0.0;
        var_sqig_dn2 = 0.0;
        var_sqig_dn3 = 0.0;
        var_sqig_dn4 = 0.0;
        var_sqig_dn5 = 0.0;
        var_sqig_dn6 = 0.0;
        var_sqig_dn7 = 0.0;
        var_sqig_dn8 = 0.0;
        var_sqig_dn9 = 0.0;
        var_sqig_dn10 = 0.0;
        var_sqig_dn11 = 0.0;
        var_sqig_db0 = 0.0;
        var_sqig_db1 = 0.0;
        var_sqig_db2 = 0.0;
        var_sqig_db3 = 0.0;
        var_sqig_db4 = 0.0;
        var_sqig_db5 = 0.0;
        var_sqig_db6 = 0.0;

        let assign62180_e80600: f64 = if ((var_xg_dc > 0.0) && (var_bet_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard1760 = assign62180_e80600;

        let assign62270_e80706: f64 = if p.p32 > 0.0 { 1.0 } else { 0.0 };
        var_guard1762 = assign62270_e80706;

        let (assign62280_e80714, assign62280_e80714_d_n0, assign62280_e80714_d_n1, assign62280_e80714_d_n2, assign62280_e80714_d_n3, assign62280_e80714_d_n4, assign62280_e80714_d_n5, assign62280_e80714_d_n6, assign62280_e80714_d_n7, assign62280_e80714_d_n8, assign62280_e80714_d_n9, assign62280_e80714_d_n10, assign62280_e80714_d_n11, assign62280_e80714_d_b0, assign62280_e80714_d_b1, assign62280_e80714_d_b2, assign62280_e80714_d_b3, assign62280_e80714_d_b4, assign62280_e80714_d_b5, assign62280_e80714_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62280_e80712: f64 = (var_qim1_dc / var_alpha_dc);
        (assign62280_e80712, (((var_qim1_dc_dn0 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn0)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn1 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn1)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn2 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn2)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn3 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn3)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn4 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn4)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn5 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn5)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn6 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn6)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn7 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn7)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn8 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn8)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn9 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn9)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn10 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn10)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn11 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn11)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_db0 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_db0)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_db1 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_db1)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_db2 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_db2)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_db3 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_db3)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_db4 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_db4)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_db5 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_db5)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_db6 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_db6)) / (var_alpha_dc * var_alpha_dc)),)
    } else {
        (var_h0, var_h0_dn0, var_h0_dn1, var_h0_dn2, var_h0_dn3, var_h0_dn4, var_h0_dn5, var_h0_dn6, var_h0_dn7, var_h0_dn8, var_h0_dn9, var_h0_dn10, var_h0_dn11, var_h0_db0, var_h0_db1, var_h0_db2, var_h0_db3, var_h0_db4, var_h0_db5, var_h0_db6,)
    }
};
        var_h0 = assign62280_e80714;
        var_h0_dn0 = assign62280_e80714_d_n0;
        var_h0_dn1 = assign62280_e80714_d_n1;
        var_h0_dn2 = assign62280_e80714_d_n2;
        var_h0_dn3 = assign62280_e80714_d_n3;
        var_h0_dn4 = assign62280_e80714_d_n4;
        var_h0_dn5 = assign62280_e80714_d_n5;
        var_h0_dn6 = assign62280_e80714_d_n6;
        var_h0_dn7 = assign62280_e80714_d_n7;
        var_h0_dn8 = assign62280_e80714_d_n8;
        var_h0_dn9 = assign62280_e80714_d_n9;
        var_h0_dn10 = assign62280_e80714_d_n10;
        var_h0_dn11 = assign62280_e80714_d_n11;
        var_h0_db0 = assign62280_e80714_d_b0;
        var_h0_db1 = assign62280_e80714_d_b1;
        var_h0_db2 = assign62280_e80714_d_b2;
        var_h0_db3 = assign62280_e80714_d_b3;
        var_h0_db4 = assign62280_e80714_d_b4;
        var_h0_db5 = assign62280_e80714_d_b5;
        var_h0_db6 = assign62280_e80714_d_b6;

        let (assign62290_e80722, assign62290_e80722_d_n0, assign62290_e80722_d_n1, assign62290_e80722_d_n2, assign62290_e80722_d_n3, assign62290_e80722_d_n4, assign62290_e80722_d_n5, assign62290_e80722_d_n6, assign62290_e80722_d_n7, assign62290_e80722_d_n8, assign62290_e80722_d_n9, assign62290_e80722_d_n10, assign62290_e80722_d_n11, assign62290_e80722_d_b0, assign62290_e80722_d_b1, assign62290_e80722_d_b2, assign62290_e80722_d_b3, assign62290_e80722_d_b4, assign62290_e80722_d_b5, assign62290_e80722_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62290_e80720: f64 = (var_qim_dc / var_qim1_dc);
        (assign62290_e80720, (((var_qim_dc_dn0 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn0)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn1 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn1)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn2 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn2)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn3 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn3)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn4 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn4)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn5 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn5)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn6 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn6)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn7 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn7)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn8 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn8)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn9 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn9)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn10 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn10)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn11 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn11)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_db0 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_db0)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_db1 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_db1)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_db2 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_db2)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_db3 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_db3)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_db4 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_db4)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_db5 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_db5)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_db6 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_db6)) / (var_qim1_dc * var_qim1_dc)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4, var_t1_db5, var_t1_db6,)
    }
};
        var_t1 = assign62290_e80722;
        var_t1_dn0 = assign62290_e80722_d_n0;
        var_t1_dn1 = assign62290_e80722_d_n1;
        var_t1_dn2 = assign62290_e80722_d_n2;
        var_t1_dn3 = assign62290_e80722_d_n3;
        var_t1_dn4 = assign62290_e80722_d_n4;
        var_t1_dn5 = assign62290_e80722_d_n5;
        var_t1_dn6 = assign62290_e80722_d_n6;
        var_t1_dn7 = assign62290_e80722_d_n7;
        var_t1_dn8 = assign62290_e80722_d_n8;
        var_t1_dn9 = assign62290_e80722_d_n9;
        var_t1_dn10 = assign62290_e80722_d_n10;
        var_t1_dn11 = assign62290_e80722_d_n11;
        var_t1_db0 = assign62290_e80722_d_b0;
        var_t1_db1 = assign62290_e80722_d_b1;
        var_t1_db2 = assign62290_e80722_d_b2;
        var_t1_db3 = assign62290_e80722_d_b3;
        var_t1_db4 = assign62290_e80722_d_b4;
        var_t1_db5 = assign62290_e80722_d_b5;
        var_t1_db6 = assign62290_e80722_d_b6;

        let (assign62300_e80734, assign62300_e80734_d_n0, assign62300_e80734_d_n1, assign62300_e80734_d_n2, assign62300_e80734_d_n3, assign62300_e80734_d_n4, assign62300_e80734_d_n5, assign62300_e80734_d_n6, assign62300_e80734_d_n7, assign62300_e80734_d_n8, assign62300_e80734_d_n9, assign62300_e80734_d_n10, assign62300_e80734_d_n11, assign62300_e80734_d_b0, assign62300_e80734_d_b1, assign62300_e80734_d_b2, assign62300_e80734_d_b3, assign62300_e80734_d_b4, assign62300_e80734_d_b5, assign62300_e80734_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62300_e80728: f64 = (0.5 * 0.16666666666666666);
        let assign62300_e80731: f64 = (var_dps_dc / var_h0);
        let assign62300_e80732: f64 = (assign62300_e80728 * assign62300_e80731);
        (assign62300_e80732, (assign62300_e80728 * (((var_dps_dc_dn0 * var_h0) - (var_dps_dc * var_h0_dn0)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_dn1 * var_h0) - (var_dps_dc * var_h0_dn1)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_dn2 * var_h0) - (var_dps_dc * var_h0_dn2)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_dn3 * var_h0) - (var_dps_dc * var_h0_dn3)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_dn4 * var_h0) - (var_dps_dc * var_h0_dn4)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_dn5 * var_h0) - (var_dps_dc * var_h0_dn5)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_dn6 * var_h0) - (var_dps_dc * var_h0_dn6)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_dn7 * var_h0) - (var_dps_dc * var_h0_dn7)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_dn8 * var_h0) - (var_dps_dc * var_h0_dn8)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_dn9 * var_h0) - (var_dps_dc * var_h0_dn9)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_dn10 * var_h0) - (var_dps_dc * var_h0_dn10)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_dn11 * var_h0) - (var_dps_dc * var_h0_dn11)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_db0 * var_h0) - (var_dps_dc * var_h0_db0)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_db1 * var_h0) - (var_dps_dc * var_h0_db1)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_db2 * var_h0) - (var_dps_dc * var_h0_db2)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_db3 * var_h0) - (var_dps_dc * var_h0_db3)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_db4 * var_h0) - (var_dps_dc * var_h0_db4)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_db5 * var_h0) - (var_dps_dc * var_h0_db5)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_db6 * var_h0) - (var_dps_dc * var_h0_db6)) / (var_h0 * var_h0))),)
    } else {
        (var_sqt2, var_sqt2_dn0, var_sqt2_dn1, var_sqt2_dn2, var_sqt2_dn3, var_sqt2_dn4, var_sqt2_dn5, var_sqt2_dn6, var_sqt2_dn7, var_sqt2_dn8, var_sqt2_dn9, var_sqt2_dn10, var_sqt2_dn11, var_sqt2_db0, var_sqt2_db1, var_sqt2_db2, var_sqt2_db3, var_sqt2_db4, var_sqt2_db5, var_sqt2_db6,)
    }
};
        var_sqt2 = assign62300_e80734;
        var_sqt2_dn0 = assign62300_e80734_d_n0;
        var_sqt2_dn1 = assign62300_e80734_d_n1;
        var_sqt2_dn2 = assign62300_e80734_d_n2;
        var_sqt2_dn3 = assign62300_e80734_d_n3;
        var_sqt2_dn4 = assign62300_e80734_d_n4;
        var_sqt2_dn5 = assign62300_e80734_d_n5;
        var_sqt2_dn6 = assign62300_e80734_d_n6;
        var_sqt2_dn7 = assign62300_e80734_d_n7;
        var_sqt2_dn8 = assign62300_e80734_d_n8;
        var_sqt2_dn9 = assign62300_e80734_d_n9;
        var_sqt2_dn10 = assign62300_e80734_d_n10;
        var_sqt2_dn11 = assign62300_e80734_d_n11;
        var_sqt2_db0 = assign62300_e80734_d_b0;
        var_sqt2_db1 = assign62300_e80734_d_b1;
        var_sqt2_db2 = assign62300_e80734_d_b2;
        var_sqt2_db3 = assign62300_e80734_d_b3;
        var_sqt2_db4 = assign62300_e80734_d_b4;
        var_sqt2_db5 = assign62300_e80734_d_b5;
        var_sqt2_db6 = assign62300_e80734_d_b6;

        let (assign62310_e80742, assign62310_e80742_d_n0, assign62310_e80742_d_n1, assign62310_e80742_d_n2, assign62310_e80742_d_n3, assign62310_e80742_d_n4, assign62310_e80742_d_n5, assign62310_e80742_d_n6, assign62310_e80742_d_n7, assign62310_e80742_d_n8, assign62310_e80742_d_n9, assign62310_e80742_d_n10, assign62310_e80742_d_n11, assign62310_e80742_d_b0, assign62310_e80742_d_b1, assign62310_e80742_d_b2, assign62310_e80742_d_b3, assign62310_e80742_d_b4, assign62310_e80742_d_b5, assign62310_e80742_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62310_e80740: f64 = (var_sqt2 * var_sqt2);
        (assign62310_e80740, ((var_sqt2_dn0 * var_sqt2) + (var_sqt2 * var_sqt2_dn0)), ((var_sqt2_dn1 * var_sqt2) + (var_sqt2 * var_sqt2_dn1)), ((var_sqt2_dn2 * var_sqt2) + (var_sqt2 * var_sqt2_dn2)), ((var_sqt2_dn3 * var_sqt2) + (var_sqt2 * var_sqt2_dn3)), ((var_sqt2_dn4 * var_sqt2) + (var_sqt2 * var_sqt2_dn4)), ((var_sqt2_dn5 * var_sqt2) + (var_sqt2 * var_sqt2_dn5)), ((var_sqt2_dn6 * var_sqt2) + (var_sqt2 * var_sqt2_dn6)), ((var_sqt2_dn7 * var_sqt2) + (var_sqt2 * var_sqt2_dn7)), ((var_sqt2_dn8 * var_sqt2) + (var_sqt2 * var_sqt2_dn8)), ((var_sqt2_dn9 * var_sqt2) + (var_sqt2 * var_sqt2_dn9)), ((var_sqt2_dn10 * var_sqt2) + (var_sqt2 * var_sqt2_dn10)), ((var_sqt2_dn11 * var_sqt2) + (var_sqt2 * var_sqt2_dn11)), ((var_sqt2_db0 * var_sqt2) + (var_sqt2 * var_sqt2_db0)), ((var_sqt2_db1 * var_sqt2) + (var_sqt2 * var_sqt2_db1)), ((var_sqt2_db2 * var_sqt2) + (var_sqt2 * var_sqt2_db2)), ((var_sqt2_db3 * var_sqt2) + (var_sqt2 * var_sqt2_db3)), ((var_sqt2_db4 * var_sqt2) + (var_sqt2 * var_sqt2_db4)), ((var_sqt2_db5 * var_sqt2) + (var_sqt2 * var_sqt2_db5)), ((var_sqt2_db6 * var_sqt2) + (var_sqt2 * var_sqt2_db6)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_db0, var_t2_db1, var_t2_db2, var_t2_db3, var_t2_db4, var_t2_db5, var_t2_db6,)
    }
};
        var_t2 = assign62310_e80742;
        var_t2_dn0 = assign62310_e80742_d_n0;
        var_t2_dn1 = assign62310_e80742_d_n1;
        var_t2_dn2 = assign62310_e80742_d_n2;
        var_t2_dn3 = assign62310_e80742_d_n3;
        var_t2_dn4 = assign62310_e80742_d_n4;
        var_t2_dn5 = assign62310_e80742_d_n5;
        var_t2_dn6 = assign62310_e80742_d_n6;
        var_t2_dn7 = assign62310_e80742_d_n7;
        var_t2_dn8 = assign62310_e80742_d_n8;
        var_t2_dn9 = assign62310_e80742_d_n9;
        var_t2_dn10 = assign62310_e80742_d_n10;
        var_t2_dn11 = assign62310_e80742_d_n11;
        var_t2_db0 = assign62310_e80742_d_b0;
        var_t2_db1 = assign62310_e80742_d_b1;
        var_t2_db2 = assign62310_e80742_d_b2;
        var_t2_db3 = assign62310_e80742_d_b3;
        var_t2_db4 = assign62310_e80742_d_b4;
        var_t2_db5 = assign62310_e80742_d_b5;
        var_t2_db6 = assign62310_e80742_d_b6;

        let (assign62320_e80752, assign62320_e80752_d_n0, assign62320_e80752_d_n1, assign62320_e80752_d_n2, assign62320_e80752_d_n3, assign62320_e80752_d_n4, assign62320_e80752_d_n5, assign62320_e80752_d_n6, assign62320_e80752_d_n7, assign62320_e80752_d_n8, assign62320_e80752_d_n9, assign62320_e80752_d_n10, assign62320_e80752_d_n11, assign62320_e80752_d_b0, assign62320_e80752_d_b1, assign62320_e80752_d_b2, assign62320_e80752_d_b3, assign62320_e80752_d_b4, assign62320_e80752_d_b5, assign62320_e80752_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62320_e80748: f64 = (var_h0 / var_h_dc);
        let assign62320_e80750: f64 = (assign62320_e80748 - 1.0);
        (assign62320_e80750, (((var_h0_dn0 * var_h_dc) - (var_h0 * var_h_dc_dn0)) / (var_h_dc * var_h_dc)), (((var_h0_dn1 * var_h_dc) - (var_h0 * var_h_dc_dn1)) / (var_h_dc * var_h_dc)), (((var_h0_dn2 * var_h_dc) - (var_h0 * var_h_dc_dn2)) / (var_h_dc * var_h_dc)), (((var_h0_dn3 * var_h_dc) - (var_h0 * var_h_dc_dn3)) / (var_h_dc * var_h_dc)), (((var_h0_dn4 * var_h_dc) - (var_h0 * var_h_dc_dn4)) / (var_h_dc * var_h_dc)), (((var_h0_dn5 * var_h_dc) - (var_h0 * var_h_dc_dn5)) / (var_h_dc * var_h_dc)), (((var_h0_dn6 * var_h_dc) - (var_h0 * var_h_dc_dn6)) / (var_h_dc * var_h_dc)), (((var_h0_dn7 * var_h_dc) - (var_h0 * var_h_dc_dn7)) / (var_h_dc * var_h_dc)), (((var_h0_dn8 * var_h_dc) - (var_h0 * var_h_dc_dn8)) / (var_h_dc * var_h_dc)), (((var_h0_dn9 * var_h_dc) - (var_h0 * var_h_dc_dn9)) / (var_h_dc * var_h_dc)), (((var_h0_dn10 * var_h_dc) - (var_h0 * var_h_dc_dn10)) / (var_h_dc * var_h_dc)), (((var_h0_dn11 * var_h_dc) - (var_h0 * var_h_dc_dn11)) / (var_h_dc * var_h_dc)), (((var_h0_db0 * var_h_dc) - (var_h0 * var_h_dc_db0)) / (var_h_dc * var_h_dc)), (((var_h0_db1 * var_h_dc) - (var_h0 * var_h_dc_db1)) / (var_h_dc * var_h_dc)), (((var_h0_db2 * var_h_dc) - (var_h0 * var_h_dc_db2)) / (var_h_dc * var_h_dc)), (((var_h0_db3 * var_h_dc) - (var_h0 * var_h_dc_db3)) / (var_h_dc * var_h_dc)), (((var_h0_db4 * var_h_dc) - (var_h0 * var_h_dc_db4)) / (var_h_dc * var_h_dc)), (((var_h0_db5 * var_h_dc) - (var_h0 * var_h_dc_db5)) / (var_h_dc * var_h_dc)), (((var_h0_db6 * var_h_dc) - (var_h0 * var_h_dc_db6)) / (var_h_dc * var_h_dc)),)
    } else {
        (var_r, var_r_dn0, var_r_dn1, var_r_dn2, var_r_dn3, var_r_dn4, var_r_dn5, var_r_dn6, var_r_dn7, var_r_dn8, var_r_dn9, var_r_dn10, var_r_dn11, var_r_db0, var_r_db1, var_r_db2, var_r_db3, var_r_db4, var_r_db5, var_r_db6,)
    }
};
        var_r = assign62320_e80752;
        var_r_dn0 = assign62320_e80752_d_n0;
        var_r_dn1 = assign62320_e80752_d_n1;
        var_r_dn2 = assign62320_e80752_d_n2;
        var_r_dn3 = assign62320_e80752_d_n3;
        var_r_dn4 = assign62320_e80752_d_n4;
        var_r_dn5 = assign62320_e80752_d_n5;
        var_r_dn6 = assign62320_e80752_d_n6;
        var_r_dn7 = assign62320_e80752_d_n7;
        var_r_dn8 = assign62320_e80752_d_n8;
        var_r_dn9 = assign62320_e80752_d_n9;
        var_r_dn10 = assign62320_e80752_d_n10;
        var_r_dn11 = assign62320_e80752_d_n11;
        var_r_db0 = assign62320_e80752_d_b0;
        var_r_db1 = assign62320_e80752_d_b1;
        var_r_db2 = assign62320_e80752_d_b2;
        var_r_db3 = assign62320_e80752_d_b3;
        var_r_db4 = assign62320_e80752_d_b4;
        var_r_db5 = assign62320_e80752_d_b5;
        var_r_db6 = assign62320_e80752_d_b6;

        *var_c_igid_slot = var_c_igid;
        *var_c_igid_db0_slot = var_c_igid_db0;
        *var_c_igid_db1_slot = var_c_igid_db1;
        *var_c_igid_db2_slot = var_c_igid_db2;
        *var_c_igid_db3_slot = var_c_igid_db3;
        *var_c_igid_db4_slot = var_c_igid_db4;
        *var_c_igid_db5_slot = var_c_igid_db5;
        *var_c_igid_db6_slot = var_c_igid_db6;
        *var_c_igid_dn0_slot = var_c_igid_dn0;
        *var_c_igid_dn1_slot = var_c_igid_dn1;
        *var_c_igid_dn10_slot = var_c_igid_dn10;
        *var_c_igid_dn11_slot = var_c_igid_dn11;
        *var_c_igid_dn2_slot = var_c_igid_dn2;
        *var_c_igid_dn3_slot = var_c_igid_dn3;
        *var_c_igid_dn4_slot = var_c_igid_dn4;
        *var_c_igid_dn5_slot = var_c_igid_dn5;
        *var_c_igid_dn6_slot = var_c_igid_dn6;
        *var_c_igid_dn7_slot = var_c_igid_dn7;
        *var_c_igid_dn8_slot = var_c_igid_dn8;
        *var_c_igid_dn9_slot = var_c_igid_dn9;
        *var_cgeff_slot = var_cgeff;
        *var_cgeff_db0_slot = var_cgeff_db0;
        *var_cgeff_db1_slot = var_cgeff_db1;
        *var_cgeff_db2_slot = var_cgeff_db2;
        *var_cgeff_db3_slot = var_cgeff_db3;
        *var_cgeff_db4_slot = var_cgeff_db4;
        *var_cgeff_db5_slot = var_cgeff_db5;
        *var_cgeff_db6_slot = var_cgeff_db6;
        *var_cgeff_dn0_slot = var_cgeff_dn0;
        *var_cgeff_dn1_slot = var_cgeff_dn1;
        *var_cgeff_dn10_slot = var_cgeff_dn10;
        *var_cgeff_dn11_slot = var_cgeff_dn11;
        *var_cgeff_dn2_slot = var_cgeff_dn2;
        *var_cgeff_dn3_slot = var_cgeff_dn3;
        *var_cgeff_dn4_slot = var_cgeff_dn4;
        *var_cgeff_dn5_slot = var_cgeff_dn5;
        *var_cgeff_dn6_slot = var_cgeff_dn6;
        *var_cgeff_dn7_slot = var_cgeff_dn7;
        *var_cgeff_dn8_slot = var_cgeff_dn8;
        *var_cgeff_dn9_slot = var_cgeff_dn9;
        *var_guard1727_slot = var_guard1727;
        *var_guard1760_slot = var_guard1760;
        *var_guard1762_slot = var_guard1762;
        *var_h0_slot = var_h0;
        *var_h0_db0_slot = var_h0_db0;
        *var_h0_db1_slot = var_h0_db1;
        *var_h0_db2_slot = var_h0_db2;
        *var_h0_db3_slot = var_h0_db3;
        *var_h0_db4_slot = var_h0_db4;
        *var_h0_db5_slot = var_h0_db5;
        *var_h0_db6_slot = var_h0_db6;
        *var_h0_dn0_slot = var_h0_dn0;
        *var_h0_dn1_slot = var_h0_dn1;
        *var_h0_dn10_slot = var_h0_dn10;
        *var_h0_dn11_slot = var_h0_dn11;
        *var_h0_dn2_slot = var_h0_dn2;
        *var_h0_dn3_slot = var_h0_dn3;
        *var_h0_dn4_slot = var_h0_dn4;
        *var_h0_dn5_slot = var_h0_dn5;
        *var_h0_dn6_slot = var_h0_dn6;
        *var_h0_dn7_slot = var_h0_dn7;
        *var_h0_dn8_slot = var_h0_dn8;
        *var_h0_dn9_slot = var_h0_dn9;
        *var_mid_slot = var_mid;
        *var_mid_db0_slot = var_mid_db0;
        *var_mid_db1_slot = var_mid_db1;
        *var_mid_db2_slot = var_mid_db2;
        *var_mid_db3_slot = var_mid_db3;
        *var_mid_db4_slot = var_mid_db4;
        *var_mid_db5_slot = var_mid_db5;
        *var_mid_db6_slot = var_mid_db6;
        *var_mid_dn0_slot = var_mid_dn0;
        *var_mid_dn1_slot = var_mid_dn1;
        *var_mid_dn10_slot = var_mid_dn10;
        *var_mid_dn11_slot = var_mid_dn11;
        *var_mid_dn2_slot = var_mid_dn2;
        *var_mid_dn3_slot = var_mid_dn3;
        *var_mid_dn4_slot = var_mid_dn4;
        *var_mid_dn5_slot = var_mid_dn5;
        *var_mid_dn6_slot = var_mid_dn6;
        *var_mid_dn7_slot = var_mid_dn7;
        *var_mid_dn8_slot = var_mid_dn8;
        *var_mid_dn9_slot = var_mid_dn9;
        *var_mig_slot = var_mig;
        *var_mig_db0_slot = var_mig_db0;
        *var_mig_db1_slot = var_mig_db1;
        *var_mig_db2_slot = var_mig_db2;
        *var_mig_db3_slot = var_mig_db3;
        *var_mig_db4_slot = var_mig_db4;
        *var_mig_db5_slot = var_mig_db5;
        *var_mig_db6_slot = var_mig_db6;
        *var_mig_dn0_slot = var_mig_dn0;
        *var_mig_dn1_slot = var_mig_dn1;
        *var_mig_dn10_slot = var_mig_dn10;
        *var_mig_dn11_slot = var_mig_dn11;
        *var_mig_dn2_slot = var_mig_dn2;
        *var_mig_dn3_slot = var_mig_dn3;
        *var_mig_dn4_slot = var_mig_dn4;
        *var_mig_dn5_slot = var_mig_dn5;
        *var_mig_dn6_slot = var_mig_dn6;
        *var_mig_dn7_slot = var_mig_dn7;
        *var_mig_dn8_slot = var_mig_dn8;
        *var_mig_dn9_slot = var_mig_dn9;
        *var_migid_slot = var_migid;
        *var_migid_db0_slot = var_migid_db0;
        *var_migid_db1_slot = var_migid_db1;
        *var_migid_db2_slot = var_migid_db2;
        *var_migid_db3_slot = var_migid_db3;
        *var_migid_db4_slot = var_migid_db4;
        *var_migid_db5_slot = var_migid_db5;
        *var_migid_db6_slot = var_migid_db6;
        *var_migid_dn0_slot = var_migid_dn0;
        *var_migid_dn1_slot = var_migid_dn1;
        *var_migid_dn10_slot = var_migid_dn10;
        *var_migid_dn11_slot = var_migid_dn11;
        *var_migid_dn2_slot = var_migid_dn2;
        *var_migid_dn3_slot = var_migid_dn3;
        *var_migid_dn4_slot = var_migid_dn4;
        *var_migid_dn5_slot = var_migid_dn5;
        *var_migid_dn6_slot = var_migid_dn6;
        *var_migid_dn7_slot = var_migid_dn7;
        *var_migid_dn8_slot = var_migid_dn8;
        *var_migid_dn9_slot = var_migid_dn9;
        *var_qd_slot = var_qd;
        *var_qd_db0_slot = var_qd_db0;
        *var_qd_db1_slot = var_qd_db1;
        *var_qd_db2_slot = var_qd_db2;
        *var_qd_db3_slot = var_qd_db3;
        *var_qd_db4_slot = var_qd_db4;
        *var_qd_db5_slot = var_qd_db5;
        *var_qd_db6_slot = var_qd_db6;
        *var_qd_dn0_slot = var_qd_dn0;
        *var_qd_dn1_slot = var_qd_dn1;
        *var_qd_dn10_slot = var_qd_dn10;
        *var_qd_dn11_slot = var_qd_dn11;
        *var_qd_dn2_slot = var_qd_dn2;
        *var_qd_dn3_slot = var_qd_dn3;
        *var_qd_dn4_slot = var_qd_dn4;
        *var_qd_dn5_slot = var_qd_dn5;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qd_dn9_slot = var_qd_dn9;
        *var_qfgd_slot = var_qfgd;
        *var_qfgd_db0_slot = var_qfgd_db0;
        *var_qfgd_db1_slot = var_qfgd_db1;
        *var_qfgd_db2_slot = var_qfgd_db2;
        *var_qfgd_db3_slot = var_qfgd_db3;
        *var_qfgd_db4_slot = var_qfgd_db4;
        *var_qfgd_db5_slot = var_qfgd_db5;
        *var_qfgd_db6_slot = var_qfgd_db6;
        *var_qfgd_dn0_slot = var_qfgd_dn0;
        *var_qfgd_dn1_slot = var_qfgd_dn1;
        *var_qfgd_dn10_slot = var_qfgd_dn10;
        *var_qfgd_dn11_slot = var_qfgd_dn11;
        *var_qfgd_dn2_slot = var_qfgd_dn2;
        *var_qfgd_dn3_slot = var_qfgd_dn3;
        *var_qfgd_dn4_slot = var_qfgd_dn4;
        *var_qfgd_dn5_slot = var_qfgd_dn5;
        *var_qfgd_dn6_slot = var_qfgd_dn6;
        *var_qfgd_dn7_slot = var_qfgd_dn7;
        *var_qfgd_dn8_slot = var_qfgd_dn8;
        *var_qfgd_dn9_slot = var_qfgd_dn9;
        *var_qfgs_slot = var_qfgs;
        *var_qfgs_db0_slot = var_qfgs_db0;
        *var_qfgs_db1_slot = var_qfgs_db1;
        *var_qfgs_db2_slot = var_qfgs_db2;
        *var_qfgs_db3_slot = var_qfgs_db3;
        *var_qfgs_db4_slot = var_qfgs_db4;
        *var_qfgs_db5_slot = var_qfgs_db5;
        *var_qfgs_db6_slot = var_qfgs_db6;
        *var_qfgs_dn0_slot = var_qfgs_dn0;
        *var_qfgs_dn1_slot = var_qfgs_dn1;
        *var_qfgs_dn10_slot = var_qfgs_dn10;
        *var_qfgs_dn11_slot = var_qfgs_dn11;
        *var_qfgs_dn2_slot = var_qfgs_dn2;
        *var_qfgs_dn3_slot = var_qfgs_dn3;
        *var_qfgs_dn4_slot = var_qfgs_dn4;
        *var_qfgs_dn5_slot = var_qfgs_dn5;
        *var_qfgs_dn6_slot = var_qfgs_dn6;
        *var_qfgs_dn7_slot = var_qfgs_dn7;
        *var_qfgs_dn8_slot = var_qfgs_dn8;
        *var_qfgs_dn9_slot = var_qfgs_dn9;
        *var_qjun_d_slot = var_qjun_d;
        *var_qjun_d_db0_slot = var_qjun_d_db0;
        *var_qjun_d_db1_slot = var_qjun_d_db1;
        *var_qjun_d_db2_slot = var_qjun_d_db2;
        *var_qjun_d_db3_slot = var_qjun_d_db3;
        *var_qjun_d_db4_slot = var_qjun_d_db4;
        *var_qjun_d_db5_slot = var_qjun_d_db5;
        *var_qjun_d_db6_slot = var_qjun_d_db6;
        *var_qjun_d_dn0_slot = var_qjun_d_dn0;
        *var_qjun_d_dn1_slot = var_qjun_d_dn1;
        *var_qjun_d_dn10_slot = var_qjun_d_dn10;
        *var_qjun_d_dn11_slot = var_qjun_d_dn11;
        *var_qjun_d_dn2_slot = var_qjun_d_dn2;
        *var_qjun_d_dn3_slot = var_qjun_d_dn3;
        *var_qjun_d_dn4_slot = var_qjun_d_dn4;
        *var_qjun_d_dn5_slot = var_qjun_d_dn5;
        *var_qjun_d_dn6_slot = var_qjun_d_dn6;
        *var_qjun_d_dn7_slot = var_qjun_d_dn7;
        *var_qjun_d_dn8_slot = var_qjun_d_dn8;
        *var_qjun_d_dn9_slot = var_qjun_d_dn9;
        *var_qjun_s_slot = var_qjun_s;
        *var_qjun_s_db0_slot = var_qjun_s_db0;
        *var_qjun_s_db1_slot = var_qjun_s_db1;
        *var_qjun_s_db2_slot = var_qjun_s_db2;
        *var_qjun_s_db3_slot = var_qjun_s_db3;
        *var_qjun_s_db4_slot = var_qjun_s_db4;
        *var_qjun_s_db5_slot = var_qjun_s_db5;
        *var_qjun_s_db6_slot = var_qjun_s_db6;
        *var_qjun_s_dn0_slot = var_qjun_s_dn0;
        *var_qjun_s_dn1_slot = var_qjun_s_dn1;
        *var_qjun_s_dn10_slot = var_qjun_s_dn10;
        *var_qjun_s_dn11_slot = var_qjun_s_dn11;
        *var_qjun_s_dn2_slot = var_qjun_s_dn2;
        *var_qjun_s_dn3_slot = var_qjun_s_dn3;
        *var_qjun_s_dn4_slot = var_qjun_s_dn4;
        *var_qjun_s_dn5_slot = var_qjun_s_dn5;
        *var_qjun_s_dn6_slot = var_qjun_s_dn6;
        *var_qjun_s_dn7_slot = var_qjun_s_dn7;
        *var_qjun_s_dn8_slot = var_qjun_s_dn8;
        *var_qjun_s_dn9_slot = var_qjun_s_dn9;
        *var_qs_slot = var_qs;
        *var_qs_db0_slot = var_qs_db0;
        *var_qs_db1_slot = var_qs_db1;
        *var_qs_db2_slot = var_qs_db2;
        *var_qs_db3_slot = var_qs_db3;
        *var_qs_db4_slot = var_qs_db4;
        *var_qs_db5_slot = var_qs_db5;
        *var_qs_db6_slot = var_qs_db6;
        *var_qs_dn0_slot = var_qs_dn0;
        *var_qs_dn1_slot = var_qs_dn1;
        *var_qs_dn10_slot = var_qs_dn10;
        *var_qs_dn11_slot = var_qs_dn11;
        *var_qs_dn2_slot = var_qs_dn2;
        *var_qs_dn3_slot = var_qs_dn3;
        *var_qs_dn4_slot = var_qs_dn4;
        *var_qs_dn5_slot = var_qs_dn5;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_qs_dn9_slot = var_qs_dn9;
        *var_r_slot = var_r;
        *var_r_db0_slot = var_r_db0;
        *var_r_db1_slot = var_r_db1;
        *var_r_db2_slot = var_r_db2;
        *var_r_db3_slot = var_r_db3;
        *var_r_db4_slot = var_r_db4;
        *var_r_db5_slot = var_r_db5;
        *var_r_db6_slot = var_r_db6;
        *var_r_dn0_slot = var_r_dn0;
        *var_r_dn1_slot = var_r_dn1;
        *var_r_dn10_slot = var_r_dn10;
        *var_r_dn11_slot = var_r_dn11;
        *var_r_dn2_slot = var_r_dn2;
        *var_r_dn3_slot = var_r_dn3;
        *var_r_dn4_slot = var_r_dn4;
        *var_r_dn5_slot = var_r_dn5;
        *var_r_dn6_slot = var_r_dn6;
        *var_r_dn7_slot = var_r_dn7;
        *var_r_dn8_slot = var_r_dn8;
        *var_r_dn9_slot = var_r_dn9;
        *var_sidexc_slot = var_sidexc;
        *var_sidexc_db0_slot = var_sidexc_db0;
        *var_sidexc_db1_slot = var_sidexc_db1;
        *var_sidexc_db2_slot = var_sidexc_db2;
        *var_sidexc_db3_slot = var_sidexc_db3;
        *var_sidexc_db4_slot = var_sidexc_db4;
        *var_sidexc_db5_slot = var_sidexc_db5;
        *var_sidexc_db6_slot = var_sidexc_db6;
        *var_sidexc_dn0_slot = var_sidexc_dn0;
        *var_sidexc_dn1_slot = var_sidexc_dn1;
        *var_sidexc_dn10_slot = var_sidexc_dn10;
        *var_sidexc_dn11_slot = var_sidexc_dn11;
        *var_sidexc_dn2_slot = var_sidexc_dn2;
        *var_sidexc_dn3_slot = var_sidexc_dn3;
        *var_sidexc_dn4_slot = var_sidexc_dn4;
        *var_sidexc_dn5_slot = var_sidexc_dn5;
        *var_sidexc_dn6_slot = var_sidexc_dn6;
        *var_sidexc_dn7_slot = var_sidexc_dn7;
        *var_sidexc_dn8_slot = var_sidexc_dn8;
        *var_sidexc_dn9_slot = var_sidexc_dn9;
        *var_sqid_slot = var_sqid;
        *var_sqid_db0_slot = var_sqid_db0;
        *var_sqid_db1_slot = var_sqid_db1;
        *var_sqid_db2_slot = var_sqid_db2;
        *var_sqid_db3_slot = var_sqid_db3;
        *var_sqid_db4_slot = var_sqid_db4;
        *var_sqid_db5_slot = var_sqid_db5;
        *var_sqid_db6_slot = var_sqid_db6;
        *var_sqid_dn0_slot = var_sqid_dn0;
        *var_sqid_dn1_slot = var_sqid_dn1;
        *var_sqid_dn10_slot = var_sqid_dn10;
        *var_sqid_dn11_slot = var_sqid_dn11;
        *var_sqid_dn2_slot = var_sqid_dn2;
        *var_sqid_dn3_slot = var_sqid_dn3;
        *var_sqid_dn4_slot = var_sqid_dn4;
        *var_sqid_dn5_slot = var_sqid_dn5;
        *var_sqid_dn6_slot = var_sqid_dn6;
        *var_sqid_dn7_slot = var_sqid_dn7;
        *var_sqid_dn8_slot = var_sqid_dn8;
        *var_sqid_dn9_slot = var_sqid_dn9;
        *var_sqig_slot = var_sqig;
        *var_sqig_db0_slot = var_sqig_db0;
        *var_sqig_db1_slot = var_sqig_db1;
        *var_sqig_db2_slot = var_sqig_db2;
        *var_sqig_db3_slot = var_sqig_db3;
        *var_sqig_db4_slot = var_sqig_db4;
        *var_sqig_db5_slot = var_sqig_db5;
        *var_sqig_db6_slot = var_sqig_db6;
        *var_sqig_dn0_slot = var_sqig_dn0;
        *var_sqig_dn1_slot = var_sqig_dn1;
        *var_sqig_dn10_slot = var_sqig_dn10;
        *var_sqig_dn11_slot = var_sqig_dn11;
        *var_sqig_dn2_slot = var_sqig_dn2;
        *var_sqig_dn3_slot = var_sqig_dn3;
        *var_sqig_dn4_slot = var_sqig_dn4;
        *var_sqig_dn5_slot = var_sqig_dn5;
        *var_sqig_dn6_slot = var_sqig_dn6;
        *var_sqig_dn7_slot = var_sqig_dn7;
        *var_sqig_dn8_slot = var_sqig_dn8;
        *var_sqig_dn9_slot = var_sqig_dn9;
        *var_sqt2_slot = var_sqt2;
        *var_sqt2_db0_slot = var_sqt2_db0;
        *var_sqt2_db1_slot = var_sqt2_db1;
        *var_sqt2_db2_slot = var_sqt2_db2;
        *var_sqt2_db3_slot = var_sqt2_db3;
        *var_sqt2_db4_slot = var_sqt2_db4;
        *var_sqt2_db5_slot = var_sqt2_db5;
        *var_sqt2_db6_slot = var_sqt2_db6;
        *var_sqt2_dn0_slot = var_sqt2_dn0;
        *var_sqt2_dn1_slot = var_sqt2_dn1;
        *var_sqt2_dn10_slot = var_sqt2_dn10;
        *var_sqt2_dn11_slot = var_sqt2_dn11;
        *var_sqt2_dn2_slot = var_sqt2_dn2;
        *var_sqt2_dn3_slot = var_sqt2_dn3;
        *var_sqt2_dn4_slot = var_sqt2_dn4;
        *var_sqt2_dn5_slot = var_sqt2_dn5;
        *var_sqt2_dn6_slot = var_sqt2_dn6;
        *var_sqt2_dn7_slot = var_sqt2_dn7;
        *var_sqt2_dn8_slot = var_sqt2_dn8;
        *var_sqt2_dn9_slot = var_sqt2_dn9;
        *var_t1_slot = var_t1;
        *var_t1_db0_slot = var_t1_db0;
        *var_t1_db1_slot = var_t1_db1;
        *var_t1_db2_slot = var_t1_db2;
        *var_t1_db3_slot = var_t1_db3;
        *var_t1_db4_slot = var_t1_db4;
        *var_t1_db5_slot = var_t1_db5;
        *var_t1_db6_slot = var_t1_db6;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_db0_slot = var_t2_db0;
        *var_t2_db1_slot = var_t2_db1;
        *var_t2_db2_slot = var_t2_db2;
        *var_t2_db3_slot = var_t2_db3;
        *var_t2_db4_slot = var_t2_db4;
        *var_t2_db5_slot = var_t2_db5;
        *var_t2_db6_slot = var_t2_db6;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_temp__blk1726_slot = var_temp__blk1726;
        *var_temp__blk1726_db0_slot = var_temp__blk1726_db0;
        *var_temp__blk1726_db1_slot = var_temp__blk1726_db1;
        *var_temp__blk1726_db2_slot = var_temp__blk1726_db2;
        *var_temp__blk1726_db3_slot = var_temp__blk1726_db3;
        *var_temp__blk1726_db4_slot = var_temp__blk1726_db4;
        *var_temp__blk1726_db5_slot = var_temp__blk1726_db5;
        *var_temp__blk1726_db6_slot = var_temp__blk1726_db6;
        *var_temp__blk1726_dn0_slot = var_temp__blk1726_dn0;
        *var_temp__blk1726_dn1_slot = var_temp__blk1726_dn1;
        *var_temp__blk1726_dn10_slot = var_temp__blk1726_dn10;
        *var_temp__blk1726_dn11_slot = var_temp__blk1726_dn11;
        *var_temp__blk1726_dn2_slot = var_temp__blk1726_dn2;
        *var_temp__blk1726_dn3_slot = var_temp__blk1726_dn3;
        *var_temp__blk1726_dn4_slot = var_temp__blk1726_dn4;
        *var_temp__blk1726_dn5_slot = var_temp__blk1726_dn5;
        *var_temp__blk1726_dn6_slot = var_temp__blk1726_dn6;
        *var_temp__blk1726_dn7_slot = var_temp__blk1726_dn7;
        *var_temp__blk1726_dn8_slot = var_temp__blk1726_dn8;
        *var_temp__blk1726_dn9_slot = var_temp__blk1726_dn9;
    }

    pub(super) fn stamp_transient_block_245(
        p: &Parameters,
        var_bet_i: f64,
        var_chnl_type: f64,
        var_dps_dc: f64,
        var_dps_dc_db0: f64,
        var_dps_dc_db1: f64,
        var_dps_dc_db2: f64,
        var_dps_dc_db3: f64,
        var_dps_dc_db4: f64,
        var_dps_dc_db5: f64,
        var_dps_dc_db6: f64,
        var_dps_dc_dn0: f64,
        var_dps_dc_dn1: f64,
        var_dps_dc_dn10: f64,
        var_dps_dc_dn11: f64,
        var_dps_dc_dn2: f64,
        var_dps_dc_dn3: f64,
        var_dps_dc_dn4: f64,
        var_dps_dc_dn5: f64,
        var_dps_dc_dn6: f64,
        var_dps_dc_dn7: f64,
        var_dps_dc_dn8: f64,
        var_dps_dc_dn9: f64,
        var_fac_exc: f64,
        var_fntexc_i: f64,
        var_gmob_dc: f64,
        var_gmob_dc_db0: f64,
        var_gmob_dc_db1: f64,
        var_gmob_dc_db2: f64,
        var_gmob_dc_db3: f64,
        var_gmob_dc_db4: f64,
        var_gmob_dc_db5: f64,
        var_gmob_dc_db6: f64,
        var_gmob_dc_dn0: f64,
        var_gmob_dc_dn1: f64,
        var_gmob_dc_dn10: f64,
        var_gmob_dc_dn11: f64,
        var_gmob_dc_dn2: f64,
        var_gmob_dc_dn3: f64,
        var_gmob_dc_dn4: f64,
        var_gmob_dc_dn5: f64,
        var_gmob_dc_dn6: f64,
        var_gmob_dc_dn7: f64,
        var_gmob_dc_dn8: f64,
        var_gmob_dc_dn9: f64,
        var_guard1760: f64,
        var_guard1762: f64,
        var_gvsatinv_dc: f64,
        var_gvsatinv_dc_db0: f64,
        var_gvsatinv_dc_db1: f64,
        var_gvsatinv_dc_db2: f64,
        var_gvsatinv_dc_db3: f64,
        var_gvsatinv_dc_db4: f64,
        var_gvsatinv_dc_db5: f64,
        var_gvsatinv_dc_db6: f64,
        var_gvsatinv_dc_dn0: f64,
        var_gvsatinv_dc_dn1: f64,
        var_gvsatinv_dc_dn10: f64,
        var_gvsatinv_dc_dn11: f64,
        var_gvsatinv_dc_dn2: f64,
        var_gvsatinv_dc_dn3: f64,
        var_gvsatinv_dc_dn4: f64,
        var_gvsatinv_dc_dn5: f64,
        var_gvsatinv_dc_dn6: f64,
        var_gvsatinv_dc_dn7: f64,
        var_gvsatinv_dc_dn8: f64,
        var_gvsatinv_dc_dn9: f64,
        var_i_ds: f64,
        var_i_ds_db0: f64,
        var_i_ds_db1: f64,
        var_i_ds_db2: f64,
        var_i_ds_db3: f64,
        var_i_ds_db4: f64,
        var_i_ds_db5: f64,
        var_i_ds_db6: f64,
        var_i_ds_dn0: f64,
        var_i_ds_dn1: f64,
        var_i_ds_dn10: f64,
        var_i_ds_dn11: f64,
        var_i_ds_dn2: f64,
        var_i_ds_dn3: f64,
        var_i_ds_dn4: f64,
        var_i_ds_dn5: f64,
        var_i_ds_dn6: f64,
        var_i_ds_dn7: f64,
        var_i_ds_dn8: f64,
        var_i_ds_dn9: f64,
        var_nt: f64,
        var_nt0: f64,
        var_qim1_dc: f64,
        var_qim1_dc_db0: f64,
        var_qim1_dc_db1: f64,
        var_qim1_dc_db2: f64,
        var_qim1_dc_db3: f64,
        var_qim1_dc_db4: f64,
        var_qim1_dc_db5: f64,
        var_qim1_dc_db6: f64,
        var_qim1_dc_dn0: f64,
        var_qim1_dc_dn1: f64,
        var_qim1_dc_dn10: f64,
        var_qim1_dc_dn11: f64,
        var_qim1_dc_dn2: f64,
        var_qim1_dc_dn3: f64,
        var_qim1_dc_dn4: f64,
        var_qim1_dc_dn5: f64,
        var_qim1_dc_dn6: f64,
        var_qim1_dc_dn7: f64,
        var_qim1_dc_dn8: f64,
        var_qim1_dc_dn9: f64,
        var_r: f64,
        var_r_db0: f64,
        var_r_db1: f64,
        var_r_db2: f64,
        var_r_db3: f64,
        var_r_db4: f64,
        var_r_db5: f64,
        var_r_db6: f64,
        var_r_dn0: f64,
        var_r_dn1: f64,
        var_r_dn10: f64,
        var_r_dn11: f64,
        var_r_dn2: f64,
        var_r_dn3: f64,
        var_r_dn4: f64,
        var_r_dn5: f64,
        var_r_dn6: f64,
        var_r_dn7: f64,
        var_r_dn8: f64,
        var_r_dn9: f64,
        var_t1: f64,
        var_t1_db0: f64,
        var_t1_db1: f64,
        var_t1_db2: f64,
        var_t1_db3: f64,
        var_t1_db4: f64,
        var_t1_db5: f64,
        var_t1_db6: f64,
        var_t1_dn0: f64,
        var_t1_dn1: f64,
        var_t1_dn10: f64,
        var_t1_dn11: f64,
        var_t1_dn2: f64,
        var_t1_dn3: f64,
        var_t1_dn4: f64,
        var_t1_dn5: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_t1_dn9: f64,
        var_t2: f64,
        var_t2_db0: f64,
        var_t2_db1: f64,
        var_t2_db2: f64,
        var_t2_db3: f64,
        var_t2_db4: f64,
        var_t2_db5: f64,
        var_t2_db6: f64,
        var_t2_dn0: f64,
        var_t2_dn1: f64,
        var_t2_dn10: f64,
        var_t2_dn11: f64,
        var_t2_dn2: f64,
        var_t2_dn3: f64,
        var_t2_dn4: f64,
        var_t2_dn5: f64,
        var_t2_dn6: f64,
        var_t2_dn7: f64,
        var_t2_dn8: f64,
        var_t2_dn9: f64,
        var_thesateff_dc: f64,
        var_thesateff_dc_db0: f64,
        var_thesateff_dc_db1: f64,
        var_thesateff_dc_db2: f64,
        var_thesateff_dc_db3: f64,
        var_thesateff_dc_db4: f64,
        var_thesateff_dc_db5: f64,
        var_thesateff_dc_db6: f64,
        var_thesateff_dc_dn0: f64,
        var_thesateff_dc_dn1: f64,
        var_thesateff_dc_dn10: f64,
        var_thesateff_dc_dn11: f64,
        var_thesateff_dc_dn2: f64,
        var_thesateff_dc_dn3: f64,
        var_thesateff_dc_dn4: f64,
        var_thesateff_dc_dn5: f64,
        var_thesateff_dc_dn6: f64,
        var_thesateff_dc_dn7: f64,
        var_thesateff_dc_dn8: f64,
        var_thesateff_dc_dn9: f64,
        var_vdse_dc: f64,
        var_vdse_dc_db0: f64,
        var_vdse_dc_db1: f64,
        var_vdse_dc_db2: f64,
        var_vdse_dc_db3: f64,
        var_vdse_dc_db4: f64,
        var_vdse_dc_db5: f64,
        var_vdse_dc_db6: f64,
        var_vdse_dc_dn0: f64,
        var_vdse_dc_dn1: f64,
        var_vdse_dc_dn10: f64,
        var_vdse_dc_dn11: f64,
        var_vdse_dc_dn2: f64,
        var_vdse_dc_dn3: f64,
        var_vdse_dc_dn4: f64,
        var_vdse_dc_dn5: f64,
        var_vdse_dc_dn6: f64,
        var_vdse_dc_dn7: f64,
        var_vdse_dc_dn8: f64,
        var_vdse_dc_dn9: f64,
        var_g_ideal_slot: &mut f64,
        var_g_ideal_db0_slot: &mut f64,
        var_g_ideal_db1_slot: &mut f64,
        var_g_ideal_db2_slot: &mut f64,
        var_g_ideal_db3_slot: &mut f64,
        var_g_ideal_db4_slot: &mut f64,
        var_g_ideal_db5_slot: &mut f64,
        var_g_ideal_db6_slot: &mut f64,
        var_g_ideal_dn0_slot: &mut f64,
        var_g_ideal_dn1_slot: &mut f64,
        var_g_ideal_dn10_slot: &mut f64,
        var_g_ideal_dn11_slot: &mut f64,
        var_g_ideal_dn2_slot: &mut f64,
        var_g_ideal_dn3_slot: &mut f64,
        var_g_ideal_dn4_slot: &mut f64,
        var_g_ideal_dn5_slot: &mut f64,
        var_g_ideal_dn6_slot: &mut f64,
        var_g_ideal_dn7_slot: &mut f64,
        var_g_ideal_dn8_slot: &mut f64,
        var_g_ideal_dn9_slot: &mut f64,
        var_gfac_slot: &mut f64,
        var_gfac_db0_slot: &mut f64,
        var_gfac_db1_slot: &mut f64,
        var_gfac_db2_slot: &mut f64,
        var_gfac_db3_slot: &mut f64,
        var_gfac_db4_slot: &mut f64,
        var_gfac_db5_slot: &mut f64,
        var_gfac_db6_slot: &mut f64,
        var_gfac_dn0_slot: &mut f64,
        var_gfac_dn1_slot: &mut f64,
        var_gfac_dn10_slot: &mut f64,
        var_gfac_dn11_slot: &mut f64,
        var_gfac_dn2_slot: &mut f64,
        var_gfac_dn3_slot: &mut f64,
        var_gfac_dn4_slot: &mut f64,
        var_gfac_dn5_slot: &mut f64,
        var_gfac_dn6_slot: &mut f64,
        var_gfac_dn7_slot: &mut f64,
        var_gfac_dn8_slot: &mut f64,
        var_gfac_dn9_slot: &mut f64,
        var_guard1763_slot: &mut f64,
        var_guard1764_slot: &mut f64,
        var_guard1765_slot: &mut f64,
        var_gvsat_exc_slot: &mut f64,
        var_gvsat_exc_db0_slot: &mut f64,
        var_gvsat_exc_db1_slot: &mut f64,
        var_gvsat_exc_db2_slot: &mut f64,
        var_gvsat_exc_db3_slot: &mut f64,
        var_gvsat_exc_db4_slot: &mut f64,
        var_gvsat_exc_db5_slot: &mut f64,
        var_gvsat_exc_db6_slot: &mut f64,
        var_gvsat_exc_dn0_slot: &mut f64,
        var_gvsat_exc_dn1_slot: &mut f64,
        var_gvsat_exc_dn10_slot: &mut f64,
        var_gvsat_exc_dn11_slot: &mut f64,
        var_gvsat_exc_dn2_slot: &mut f64,
        var_gvsat_exc_dn3_slot: &mut f64,
        var_gvsat_exc_dn4_slot: &mut f64,
        var_gvsat_exc_dn5_slot: &mut f64,
        var_gvsat_exc_dn6_slot: &mut f64,
        var_gvsat_exc_dn7_slot: &mut f64,
        var_gvsat_exc_dn8_slot: &mut f64,
        var_gvsat_exc_dn9_slot: &mut f64,
        var_lc_slot: &mut f64,
        var_lc_db0_slot: &mut f64,
        var_lc_db1_slot: &mut f64,
        var_lc_db2_slot: &mut f64,
        var_lc_db3_slot: &mut f64,
        var_lc_db4_slot: &mut f64,
        var_lc_db5_slot: &mut f64,
        var_lc_db6_slot: &mut f64,
        var_lc_dn0_slot: &mut f64,
        var_lc_dn1_slot: &mut f64,
        var_lc_dn10_slot: &mut f64,
        var_lc_dn11_slot: &mut f64,
        var_lc_dn2_slot: &mut f64,
        var_lc_dn3_slot: &mut f64,
        var_lc_dn4_slot: &mut f64,
        var_lc_dn5_slot: &mut f64,
        var_lc_dn6_slot: &mut f64,
        var_lc_dn7_slot: &mut f64,
        var_lc_dn8_slot: &mut f64,
        var_lc_dn9_slot: &mut f64,
        var_lcinv2_slot: &mut f64,
        var_lcinv2_db0_slot: &mut f64,
        var_lcinv2_db1_slot: &mut f64,
        var_lcinv2_db2_slot: &mut f64,
        var_lcinv2_db3_slot: &mut f64,
        var_lcinv2_db4_slot: &mut f64,
        var_lcinv2_db5_slot: &mut f64,
        var_lcinv2_db6_slot: &mut f64,
        var_lcinv2_dn0_slot: &mut f64,
        var_lcinv2_dn1_slot: &mut f64,
        var_lcinv2_dn10_slot: &mut f64,
        var_lcinv2_dn11_slot: &mut f64,
        var_lcinv2_dn2_slot: &mut f64,
        var_lcinv2_dn3_slot: &mut f64,
        var_lcinv2_dn4_slot: &mut f64,
        var_lcinv2_dn5_slot: &mut f64,
        var_lcinv2_dn6_slot: &mut f64,
        var_lcinv2_dn7_slot: &mut f64,
        var_lcinv2_dn8_slot: &mut f64,
        var_lcinv2_dn9_slot: &mut f64,
        var_mid_slot: &mut f64,
        var_mid_db0_slot: &mut f64,
        var_mid_db1_slot: &mut f64,
        var_mid_db2_slot: &mut f64,
        var_mid_db3_slot: &mut f64,
        var_mid_db4_slot: &mut f64,
        var_mid_db5_slot: &mut f64,
        var_mid_db6_slot: &mut f64,
        var_mid_dn0_slot: &mut f64,
        var_mid_dn1_slot: &mut f64,
        var_mid_dn10_slot: &mut f64,
        var_mid_dn11_slot: &mut f64,
        var_mid_dn2_slot: &mut f64,
        var_mid_dn3_slot: &mut f64,
        var_mid_dn4_slot: &mut f64,
        var_mid_dn5_slot: &mut f64,
        var_mid_dn6_slot: &mut f64,
        var_mid_dn7_slot: &mut f64,
        var_mid_dn8_slot: &mut f64,
        var_mid_dn9_slot: &mut f64,
        var_mig_slot: &mut f64,
        var_mig_db0_slot: &mut f64,
        var_mig_db1_slot: &mut f64,
        var_mig_db2_slot: &mut f64,
        var_mig_db3_slot: &mut f64,
        var_mig_db4_slot: &mut f64,
        var_mig_db5_slot: &mut f64,
        var_mig_db6_slot: &mut f64,
        var_mig_dn0_slot: &mut f64,
        var_mig_dn1_slot: &mut f64,
        var_mig_dn10_slot: &mut f64,
        var_mig_dn11_slot: &mut f64,
        var_mig_dn2_slot: &mut f64,
        var_mig_dn3_slot: &mut f64,
        var_mig_dn4_slot: &mut f64,
        var_mig_dn5_slot: &mut f64,
        var_mig_dn6_slot: &mut f64,
        var_mig_dn7_slot: &mut f64,
        var_mig_dn8_slot: &mut f64,
        var_mig_dn9_slot: &mut f64,
        var_sidexc_slot: &mut f64,
        var_sidexc_db0_slot: &mut f64,
        var_sidexc_db1_slot: &mut f64,
        var_sidexc_db2_slot: &mut f64,
        var_sidexc_db3_slot: &mut f64,
        var_sidexc_db4_slot: &mut f64,
        var_sidexc_db5_slot: &mut f64,
        var_sidexc_db6_slot: &mut f64,
        var_sidexc_dn0_slot: &mut f64,
        var_sidexc_dn1_slot: &mut f64,
        var_sidexc_dn10_slot: &mut f64,
        var_sidexc_dn11_slot: &mut f64,
        var_sidexc_dn2_slot: &mut f64,
        var_sidexc_dn3_slot: &mut f64,
        var_sidexc_dn4_slot: &mut f64,
        var_sidexc_dn5_slot: &mut f64,
        var_sidexc_dn6_slot: &mut f64,
        var_sidexc_dn7_slot: &mut f64,
        var_sidexc_dn8_slot: &mut f64,
        var_sidexc_dn9_slot: &mut f64,
        var_sqid_slot: &mut f64,
        var_sqid_db0_slot: &mut f64,
        var_sqid_db1_slot: &mut f64,
        var_sqid_db2_slot: &mut f64,
        var_sqid_db3_slot: &mut f64,
        var_sqid_db4_slot: &mut f64,
        var_sqid_db5_slot: &mut f64,
        var_sqid_db6_slot: &mut f64,
        var_sqid_dn0_slot: &mut f64,
        var_sqid_dn1_slot: &mut f64,
        var_sqid_dn10_slot: &mut f64,
        var_sqid_dn11_slot: &mut f64,
        var_sqid_dn2_slot: &mut f64,
        var_sqid_dn3_slot: &mut f64,
        var_sqid_dn4_slot: &mut f64,
        var_sqid_dn5_slot: &mut f64,
        var_sqid_dn6_slot: &mut f64,
        var_sqid_dn7_slot: &mut f64,
        var_sqid_dn8_slot: &mut f64,
        var_sqid_dn9_slot: &mut f64,
        var_thesat1_exc_slot: &mut f64,
        var_thesat1_exc_db0_slot: &mut f64,
        var_thesat1_exc_db1_slot: &mut f64,
        var_thesat1_exc_db2_slot: &mut f64,
        var_thesat1_exc_db3_slot: &mut f64,
        var_thesat1_exc_db4_slot: &mut f64,
        var_thesat1_exc_db5_slot: &mut f64,
        var_thesat1_exc_db6_slot: &mut f64,
        var_thesat1_exc_dn0_slot: &mut f64,
        var_thesat1_exc_dn1_slot: &mut f64,
        var_thesat1_exc_dn10_slot: &mut f64,
        var_thesat1_exc_dn11_slot: &mut f64,
        var_thesat1_exc_dn2_slot: &mut f64,
        var_thesat1_exc_dn3_slot: &mut f64,
        var_thesat1_exc_dn4_slot: &mut f64,
        var_thesat1_exc_dn5_slot: &mut f64,
        var_thesat1_exc_dn6_slot: &mut f64,
        var_thesat1_exc_dn7_slot: &mut f64,
        var_thesat1_exc_dn8_slot: &mut f64,
        var_thesat1_exc_dn9_slot: &mut f64,
        var_zsat_exc_slot: &mut f64,
        var_zsat_exc_db0_slot: &mut f64,
        var_zsat_exc_db1_slot: &mut f64,
        var_zsat_exc_db2_slot: &mut f64,
        var_zsat_exc_db3_slot: &mut f64,
        var_zsat_exc_db4_slot: &mut f64,
        var_zsat_exc_db5_slot: &mut f64,
        var_zsat_exc_db6_slot: &mut f64,
        var_zsat_exc_dn0_slot: &mut f64,
        var_zsat_exc_dn1_slot: &mut f64,
        var_zsat_exc_dn10_slot: &mut f64,
        var_zsat_exc_dn11_slot: &mut f64,
        var_zsat_exc_dn2_slot: &mut f64,
        var_zsat_exc_dn3_slot: &mut f64,
        var_zsat_exc_dn4_slot: &mut f64,
        var_zsat_exc_dn5_slot: &mut f64,
        var_zsat_exc_dn6_slot: &mut f64,
        var_zsat_exc_dn7_slot: &mut f64,
        var_zsat_exc_dn8_slot: &mut f64,
        var_zsat_exc_dn9_slot: &mut f64,
    ) {
        let mut var_g_ideal: f64 = *var_g_ideal_slot;
        let mut var_g_ideal_db0: f64 = *var_g_ideal_db0_slot;
        let mut var_g_ideal_db1: f64 = *var_g_ideal_db1_slot;
        let mut var_g_ideal_db2: f64 = *var_g_ideal_db2_slot;
        let mut var_g_ideal_db3: f64 = *var_g_ideal_db3_slot;
        let mut var_g_ideal_db4: f64 = *var_g_ideal_db4_slot;
        let mut var_g_ideal_db5: f64 = *var_g_ideal_db5_slot;
        let mut var_g_ideal_db6: f64 = *var_g_ideal_db6_slot;
        let mut var_g_ideal_dn0: f64 = *var_g_ideal_dn0_slot;
        let mut var_g_ideal_dn1: f64 = *var_g_ideal_dn1_slot;
        let mut var_g_ideal_dn10: f64 = *var_g_ideal_dn10_slot;
        let mut var_g_ideal_dn11: f64 = *var_g_ideal_dn11_slot;
        let mut var_g_ideal_dn2: f64 = *var_g_ideal_dn2_slot;
        let mut var_g_ideal_dn3: f64 = *var_g_ideal_dn3_slot;
        let mut var_g_ideal_dn4: f64 = *var_g_ideal_dn4_slot;
        let mut var_g_ideal_dn5: f64 = *var_g_ideal_dn5_slot;
        let mut var_g_ideal_dn6: f64 = *var_g_ideal_dn6_slot;
        let mut var_g_ideal_dn7: f64 = *var_g_ideal_dn7_slot;
        let mut var_g_ideal_dn8: f64 = *var_g_ideal_dn8_slot;
        let mut var_g_ideal_dn9: f64 = *var_g_ideal_dn9_slot;
        let mut var_gfac: f64 = *var_gfac_slot;
        let mut var_gfac_db0: f64 = *var_gfac_db0_slot;
        let mut var_gfac_db1: f64 = *var_gfac_db1_slot;
        let mut var_gfac_db2: f64 = *var_gfac_db2_slot;
        let mut var_gfac_db3: f64 = *var_gfac_db3_slot;
        let mut var_gfac_db4: f64 = *var_gfac_db4_slot;
        let mut var_gfac_db5: f64 = *var_gfac_db5_slot;
        let mut var_gfac_db6: f64 = *var_gfac_db6_slot;
        let mut var_gfac_dn0: f64 = *var_gfac_dn0_slot;
        let mut var_gfac_dn1: f64 = *var_gfac_dn1_slot;
        let mut var_gfac_dn10: f64 = *var_gfac_dn10_slot;
        let mut var_gfac_dn11: f64 = *var_gfac_dn11_slot;
        let mut var_gfac_dn2: f64 = *var_gfac_dn2_slot;
        let mut var_gfac_dn3: f64 = *var_gfac_dn3_slot;
        let mut var_gfac_dn4: f64 = *var_gfac_dn4_slot;
        let mut var_gfac_dn5: f64 = *var_gfac_dn5_slot;
        let mut var_gfac_dn6: f64 = *var_gfac_dn6_slot;
        let mut var_gfac_dn7: f64 = *var_gfac_dn7_slot;
        let mut var_gfac_dn8: f64 = *var_gfac_dn8_slot;
        let mut var_gfac_dn9: f64 = *var_gfac_dn9_slot;
        let mut var_guard1763: f64 = *var_guard1763_slot;
        let mut var_guard1764: f64 = *var_guard1764_slot;
        let mut var_guard1765: f64 = *var_guard1765_slot;
        let mut var_gvsat_exc: f64 = *var_gvsat_exc_slot;
        let mut var_gvsat_exc_db0: f64 = *var_gvsat_exc_db0_slot;
        let mut var_gvsat_exc_db1: f64 = *var_gvsat_exc_db1_slot;
        let mut var_gvsat_exc_db2: f64 = *var_gvsat_exc_db2_slot;
        let mut var_gvsat_exc_db3: f64 = *var_gvsat_exc_db3_slot;
        let mut var_gvsat_exc_db4: f64 = *var_gvsat_exc_db4_slot;
        let mut var_gvsat_exc_db5: f64 = *var_gvsat_exc_db5_slot;
        let mut var_gvsat_exc_db6: f64 = *var_gvsat_exc_db6_slot;
        let mut var_gvsat_exc_dn0: f64 = *var_gvsat_exc_dn0_slot;
        let mut var_gvsat_exc_dn1: f64 = *var_gvsat_exc_dn1_slot;
        let mut var_gvsat_exc_dn10: f64 = *var_gvsat_exc_dn10_slot;
        let mut var_gvsat_exc_dn11: f64 = *var_gvsat_exc_dn11_slot;
        let mut var_gvsat_exc_dn2: f64 = *var_gvsat_exc_dn2_slot;
        let mut var_gvsat_exc_dn3: f64 = *var_gvsat_exc_dn3_slot;
        let mut var_gvsat_exc_dn4: f64 = *var_gvsat_exc_dn4_slot;
        let mut var_gvsat_exc_dn5: f64 = *var_gvsat_exc_dn5_slot;
        let mut var_gvsat_exc_dn6: f64 = *var_gvsat_exc_dn6_slot;
        let mut var_gvsat_exc_dn7: f64 = *var_gvsat_exc_dn7_slot;
        let mut var_gvsat_exc_dn8: f64 = *var_gvsat_exc_dn8_slot;
        let mut var_gvsat_exc_dn9: f64 = *var_gvsat_exc_dn9_slot;
        let mut var_lc: f64 = *var_lc_slot;
        let mut var_lc_db0: f64 = *var_lc_db0_slot;
        let mut var_lc_db1: f64 = *var_lc_db1_slot;
        let mut var_lc_db2: f64 = *var_lc_db2_slot;
        let mut var_lc_db3: f64 = *var_lc_db3_slot;
        let mut var_lc_db4: f64 = *var_lc_db4_slot;
        let mut var_lc_db5: f64 = *var_lc_db5_slot;
        let mut var_lc_db6: f64 = *var_lc_db6_slot;
        let mut var_lc_dn0: f64 = *var_lc_dn0_slot;
        let mut var_lc_dn1: f64 = *var_lc_dn1_slot;
        let mut var_lc_dn10: f64 = *var_lc_dn10_slot;
        let mut var_lc_dn11: f64 = *var_lc_dn11_slot;
        let mut var_lc_dn2: f64 = *var_lc_dn2_slot;
        let mut var_lc_dn3: f64 = *var_lc_dn3_slot;
        let mut var_lc_dn4: f64 = *var_lc_dn4_slot;
        let mut var_lc_dn5: f64 = *var_lc_dn5_slot;
        let mut var_lc_dn6: f64 = *var_lc_dn6_slot;
        let mut var_lc_dn7: f64 = *var_lc_dn7_slot;
        let mut var_lc_dn8: f64 = *var_lc_dn8_slot;
        let mut var_lc_dn9: f64 = *var_lc_dn9_slot;
        let mut var_lcinv2: f64 = *var_lcinv2_slot;
        let mut var_lcinv2_db0: f64 = *var_lcinv2_db0_slot;
        let mut var_lcinv2_db1: f64 = *var_lcinv2_db1_slot;
        let mut var_lcinv2_db2: f64 = *var_lcinv2_db2_slot;
        let mut var_lcinv2_db3: f64 = *var_lcinv2_db3_slot;
        let mut var_lcinv2_db4: f64 = *var_lcinv2_db4_slot;
        let mut var_lcinv2_db5: f64 = *var_lcinv2_db5_slot;
        let mut var_lcinv2_db6: f64 = *var_lcinv2_db6_slot;
        let mut var_lcinv2_dn0: f64 = *var_lcinv2_dn0_slot;
        let mut var_lcinv2_dn1: f64 = *var_lcinv2_dn1_slot;
        let mut var_lcinv2_dn10: f64 = *var_lcinv2_dn10_slot;
        let mut var_lcinv2_dn11: f64 = *var_lcinv2_dn11_slot;
        let mut var_lcinv2_dn2: f64 = *var_lcinv2_dn2_slot;
        let mut var_lcinv2_dn3: f64 = *var_lcinv2_dn3_slot;
        let mut var_lcinv2_dn4: f64 = *var_lcinv2_dn4_slot;
        let mut var_lcinv2_dn5: f64 = *var_lcinv2_dn5_slot;
        let mut var_lcinv2_dn6: f64 = *var_lcinv2_dn6_slot;
        let mut var_lcinv2_dn7: f64 = *var_lcinv2_dn7_slot;
        let mut var_lcinv2_dn8: f64 = *var_lcinv2_dn8_slot;
        let mut var_lcinv2_dn9: f64 = *var_lcinv2_dn9_slot;
        let mut var_mid: f64 = *var_mid_slot;
        let mut var_mid_db0: f64 = *var_mid_db0_slot;
        let mut var_mid_db1: f64 = *var_mid_db1_slot;
        let mut var_mid_db2: f64 = *var_mid_db2_slot;
        let mut var_mid_db3: f64 = *var_mid_db3_slot;
        let mut var_mid_db4: f64 = *var_mid_db4_slot;
        let mut var_mid_db5: f64 = *var_mid_db5_slot;
        let mut var_mid_db6: f64 = *var_mid_db6_slot;
        let mut var_mid_dn0: f64 = *var_mid_dn0_slot;
        let mut var_mid_dn1: f64 = *var_mid_dn1_slot;
        let mut var_mid_dn10: f64 = *var_mid_dn10_slot;
        let mut var_mid_dn11: f64 = *var_mid_dn11_slot;
        let mut var_mid_dn2: f64 = *var_mid_dn2_slot;
        let mut var_mid_dn3: f64 = *var_mid_dn3_slot;
        let mut var_mid_dn4: f64 = *var_mid_dn4_slot;
        let mut var_mid_dn5: f64 = *var_mid_dn5_slot;
        let mut var_mid_dn6: f64 = *var_mid_dn6_slot;
        let mut var_mid_dn7: f64 = *var_mid_dn7_slot;
        let mut var_mid_dn8: f64 = *var_mid_dn8_slot;
        let mut var_mid_dn9: f64 = *var_mid_dn9_slot;
        let mut var_mig: f64 = *var_mig_slot;
        let mut var_mig_db0: f64 = *var_mig_db0_slot;
        let mut var_mig_db1: f64 = *var_mig_db1_slot;
        let mut var_mig_db2: f64 = *var_mig_db2_slot;
        let mut var_mig_db3: f64 = *var_mig_db3_slot;
        let mut var_mig_db4: f64 = *var_mig_db4_slot;
        let mut var_mig_db5: f64 = *var_mig_db5_slot;
        let mut var_mig_db6: f64 = *var_mig_db6_slot;
        let mut var_mig_dn0: f64 = *var_mig_dn0_slot;
        let mut var_mig_dn1: f64 = *var_mig_dn1_slot;
        let mut var_mig_dn10: f64 = *var_mig_dn10_slot;
        let mut var_mig_dn11: f64 = *var_mig_dn11_slot;
        let mut var_mig_dn2: f64 = *var_mig_dn2_slot;
        let mut var_mig_dn3: f64 = *var_mig_dn3_slot;
        let mut var_mig_dn4: f64 = *var_mig_dn4_slot;
        let mut var_mig_dn5: f64 = *var_mig_dn5_slot;
        let mut var_mig_dn6: f64 = *var_mig_dn6_slot;
        let mut var_mig_dn7: f64 = *var_mig_dn7_slot;
        let mut var_mig_dn8: f64 = *var_mig_dn8_slot;
        let mut var_mig_dn9: f64 = *var_mig_dn9_slot;
        let mut var_sidexc: f64 = *var_sidexc_slot;
        let mut var_sidexc_db0: f64 = *var_sidexc_db0_slot;
        let mut var_sidexc_db1: f64 = *var_sidexc_db1_slot;
        let mut var_sidexc_db2: f64 = *var_sidexc_db2_slot;
        let mut var_sidexc_db3: f64 = *var_sidexc_db3_slot;
        let mut var_sidexc_db4: f64 = *var_sidexc_db4_slot;
        let mut var_sidexc_db5: f64 = *var_sidexc_db5_slot;
        let mut var_sidexc_db6: f64 = *var_sidexc_db6_slot;
        let mut var_sidexc_dn0: f64 = *var_sidexc_dn0_slot;
        let mut var_sidexc_dn1: f64 = *var_sidexc_dn1_slot;
        let mut var_sidexc_dn10: f64 = *var_sidexc_dn10_slot;
        let mut var_sidexc_dn11: f64 = *var_sidexc_dn11_slot;
        let mut var_sidexc_dn2: f64 = *var_sidexc_dn2_slot;
        let mut var_sidexc_dn3: f64 = *var_sidexc_dn3_slot;
        let mut var_sidexc_dn4: f64 = *var_sidexc_dn4_slot;
        let mut var_sidexc_dn5: f64 = *var_sidexc_dn5_slot;
        let mut var_sidexc_dn6: f64 = *var_sidexc_dn6_slot;
        let mut var_sidexc_dn7: f64 = *var_sidexc_dn7_slot;
        let mut var_sidexc_dn8: f64 = *var_sidexc_dn8_slot;
        let mut var_sidexc_dn9: f64 = *var_sidexc_dn9_slot;
        let mut var_sqid: f64 = *var_sqid_slot;
        let mut var_sqid_db0: f64 = *var_sqid_db0_slot;
        let mut var_sqid_db1: f64 = *var_sqid_db1_slot;
        let mut var_sqid_db2: f64 = *var_sqid_db2_slot;
        let mut var_sqid_db3: f64 = *var_sqid_db3_slot;
        let mut var_sqid_db4: f64 = *var_sqid_db4_slot;
        let mut var_sqid_db5: f64 = *var_sqid_db5_slot;
        let mut var_sqid_db6: f64 = *var_sqid_db6_slot;
        let mut var_sqid_dn0: f64 = *var_sqid_dn0_slot;
        let mut var_sqid_dn1: f64 = *var_sqid_dn1_slot;
        let mut var_sqid_dn10: f64 = *var_sqid_dn10_slot;
        let mut var_sqid_dn11: f64 = *var_sqid_dn11_slot;
        let mut var_sqid_dn2: f64 = *var_sqid_dn2_slot;
        let mut var_sqid_dn3: f64 = *var_sqid_dn3_slot;
        let mut var_sqid_dn4: f64 = *var_sqid_dn4_slot;
        let mut var_sqid_dn5: f64 = *var_sqid_dn5_slot;
        let mut var_sqid_dn6: f64 = *var_sqid_dn6_slot;
        let mut var_sqid_dn7: f64 = *var_sqid_dn7_slot;
        let mut var_sqid_dn8: f64 = *var_sqid_dn8_slot;
        let mut var_sqid_dn9: f64 = *var_sqid_dn9_slot;
        let mut var_thesat1_exc: f64 = *var_thesat1_exc_slot;
        let mut var_thesat1_exc_db0: f64 = *var_thesat1_exc_db0_slot;
        let mut var_thesat1_exc_db1: f64 = *var_thesat1_exc_db1_slot;
        let mut var_thesat1_exc_db2: f64 = *var_thesat1_exc_db2_slot;
        let mut var_thesat1_exc_db3: f64 = *var_thesat1_exc_db3_slot;
        let mut var_thesat1_exc_db4: f64 = *var_thesat1_exc_db4_slot;
        let mut var_thesat1_exc_db5: f64 = *var_thesat1_exc_db5_slot;
        let mut var_thesat1_exc_db6: f64 = *var_thesat1_exc_db6_slot;
        let mut var_thesat1_exc_dn0: f64 = *var_thesat1_exc_dn0_slot;
        let mut var_thesat1_exc_dn1: f64 = *var_thesat1_exc_dn1_slot;
        let mut var_thesat1_exc_dn10: f64 = *var_thesat1_exc_dn10_slot;
        let mut var_thesat1_exc_dn11: f64 = *var_thesat1_exc_dn11_slot;
        let mut var_thesat1_exc_dn2: f64 = *var_thesat1_exc_dn2_slot;
        let mut var_thesat1_exc_dn3: f64 = *var_thesat1_exc_dn3_slot;
        let mut var_thesat1_exc_dn4: f64 = *var_thesat1_exc_dn4_slot;
        let mut var_thesat1_exc_dn5: f64 = *var_thesat1_exc_dn5_slot;
        let mut var_thesat1_exc_dn6: f64 = *var_thesat1_exc_dn6_slot;
        let mut var_thesat1_exc_dn7: f64 = *var_thesat1_exc_dn7_slot;
        let mut var_thesat1_exc_dn8: f64 = *var_thesat1_exc_dn8_slot;
        let mut var_thesat1_exc_dn9: f64 = *var_thesat1_exc_dn9_slot;
        let mut var_zsat_exc: f64 = *var_zsat_exc_slot;
        let mut var_zsat_exc_db0: f64 = *var_zsat_exc_db0_slot;
        let mut var_zsat_exc_db1: f64 = *var_zsat_exc_db1_slot;
        let mut var_zsat_exc_db2: f64 = *var_zsat_exc_db2_slot;
        let mut var_zsat_exc_db3: f64 = *var_zsat_exc_db3_slot;
        let mut var_zsat_exc_db4: f64 = *var_zsat_exc_db4_slot;
        let mut var_zsat_exc_db5: f64 = *var_zsat_exc_db5_slot;
        let mut var_zsat_exc_db6: f64 = *var_zsat_exc_db6_slot;
        let mut var_zsat_exc_dn0: f64 = *var_zsat_exc_dn0_slot;
        let mut var_zsat_exc_dn1: f64 = *var_zsat_exc_dn1_slot;
        let mut var_zsat_exc_dn10: f64 = *var_zsat_exc_dn10_slot;
        let mut var_zsat_exc_dn11: f64 = *var_zsat_exc_dn11_slot;
        let mut var_zsat_exc_dn2: f64 = *var_zsat_exc_dn2_slot;
        let mut var_zsat_exc_dn3: f64 = *var_zsat_exc_dn3_slot;
        let mut var_zsat_exc_dn4: f64 = *var_zsat_exc_dn4_slot;
        let mut var_zsat_exc_dn5: f64 = *var_zsat_exc_dn5_slot;
        let mut var_zsat_exc_dn6: f64 = *var_zsat_exc_dn6_slot;
        let mut var_zsat_exc_dn7: f64 = *var_zsat_exc_dn7_slot;
        let mut var_zsat_exc_dn8: f64 = *var_zsat_exc_dn8_slot;
        let mut var_zsat_exc_dn9: f64 = *var_zsat_exc_dn9_slot;

        let (assign62330_e80775, assign62330_e80775_d_n0, assign62330_e80775_d_n1, assign62330_e80775_d_n2, assign62330_e80775_d_n3, assign62330_e80775_d_n4, assign62330_e80775_d_n5, assign62330_e80775_d_n6, assign62330_e80775_d_n7, assign62330_e80775_d_n8, assign62330_e80775_d_n9, assign62330_e80775_d_n10, assign62330_e80775_d_n11, assign62330_e80775_d_b0, assign62330_e80775_d_b1, assign62330_e80775_d_b2, assign62330_e80775_d_b3, assign62330_e80775_d_b4, assign62330_e80775_d_b5, assign62330_e80775_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62330_e80760: f64 = (var_r * var_t2);
        let assign62330_e80761: f64 = (12.0 * assign62330_e80760);
        let assign62330_e80762: f64 = (1.0 - assign62330_e80761);
        let (assign62330_e80773, assign62330_e80773_d_n0, assign62330_e80773_d_n1, assign62330_e80773_d_n2, assign62330_e80773_d_n3, assign62330_e80773_d_n4, assign62330_e80773_d_n5, assign62330_e80773_d_n6, assign62330_e80773_d_n7, assign62330_e80773_d_n8, assign62330_e80773_d_n9, assign62330_e80773_d_n10, assign62330_e80773_d_n11, assign62330_e80773_d_b0, assign62330_e80773_d_b1, assign62330_e80773_d_b2, assign62330_e80773_d_b3, assign62330_e80773_d_b4, assign62330_e80773_d_b5, assign62330_e80773_d_b6,) = {
            if (assign62330_e80762 > 1e-20) {
                let assign62330_e80769: f64 = (var_r * var_t2);
                let assign62330_e80770: f64 = (12.0 * assign62330_e80769);
                let assign62330_e80771: f64 = (1.0 - assign62330_e80770);
                (assign62330_e80771, (-(12.0 * ((var_r_dn0 * var_t2) + (var_r * var_t2_dn0)))), (-(12.0 * ((var_r_dn1 * var_t2) + (var_r * var_t2_dn1)))), (-(12.0 * ((var_r_dn2 * var_t2) + (var_r * var_t2_dn2)))), (-(12.0 * ((var_r_dn3 * var_t2) + (var_r * var_t2_dn3)))), (-(12.0 * ((var_r_dn4 * var_t2) + (var_r * var_t2_dn4)))), (-(12.0 * ((var_r_dn5 * var_t2) + (var_r * var_t2_dn5)))), (-(12.0 * ((var_r_dn6 * var_t2) + (var_r * var_t2_dn6)))), (-(12.0 * ((var_r_dn7 * var_t2) + (var_r * var_t2_dn7)))), (-(12.0 * ((var_r_dn8 * var_t2) + (var_r * var_t2_dn8)))), (-(12.0 * ((var_r_dn9 * var_t2) + (var_r * var_t2_dn9)))), (-(12.0 * ((var_r_dn10 * var_t2) + (var_r * var_t2_dn10)))), (-(12.0 * ((var_r_dn11 * var_t2) + (var_r * var_t2_dn11)))), (-(12.0 * ((var_r_db0 * var_t2) + (var_r * var_t2_db0)))), (-(12.0 * ((var_r_db1 * var_t2) + (var_r * var_t2_db1)))), (-(12.0 * ((var_r_db2 * var_t2) + (var_r * var_t2_db2)))), (-(12.0 * ((var_r_db3 * var_t2) + (var_r * var_t2_db3)))), (-(12.0 * ((var_r_db4 * var_t2) + (var_r * var_t2_db4)))), (-(12.0 * ((var_r_db5 * var_t2) + (var_r * var_t2_db5)))), (-(12.0 * ((var_r_db6 * var_t2) + (var_r * var_t2_db6)))),)
            } else {
                (1e-20, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62330_e80773, assign62330_e80773_d_n0, assign62330_e80773_d_n1, assign62330_e80773_d_n2, assign62330_e80773_d_n3, assign62330_e80773_d_n4, assign62330_e80773_d_n5, assign62330_e80773_d_n6, assign62330_e80773_d_n7, assign62330_e80773_d_n8, assign62330_e80773_d_n9, assign62330_e80773_d_n10, assign62330_e80773_d_n11, assign62330_e80773_d_b0, assign62330_e80773_d_b1, assign62330_e80773_d_b2, assign62330_e80773_d_b3, assign62330_e80773_d_b4, assign62330_e80773_d_b5, assign62330_e80773_d_b6,)
    } else {
        (var_lc, var_lc_dn0, var_lc_dn1, var_lc_dn2, var_lc_dn3, var_lc_dn4, var_lc_dn5, var_lc_dn6, var_lc_dn7, var_lc_dn8, var_lc_dn9, var_lc_dn10, var_lc_dn11, var_lc_db0, var_lc_db1, var_lc_db2, var_lc_db3, var_lc_db4, var_lc_db5, var_lc_db6,)
    }
};
        var_lc = assign62330_e80775;
        var_lc_dn0 = assign62330_e80775_d_n0;
        var_lc_dn1 = assign62330_e80775_d_n1;
        var_lc_dn2 = assign62330_e80775_d_n2;
        var_lc_dn3 = assign62330_e80775_d_n3;
        var_lc_dn4 = assign62330_e80775_d_n4;
        var_lc_dn5 = assign62330_e80775_d_n5;
        var_lc_dn6 = assign62330_e80775_d_n6;
        var_lc_dn7 = assign62330_e80775_d_n7;
        var_lc_dn8 = assign62330_e80775_d_n8;
        var_lc_dn9 = assign62330_e80775_d_n9;
        var_lc_dn10 = assign62330_e80775_d_n10;
        var_lc_dn11 = assign62330_e80775_d_n11;
        var_lc_db0 = assign62330_e80775_d_b0;
        var_lc_db1 = assign62330_e80775_d_b1;
        var_lc_db2 = assign62330_e80775_d_b2;
        var_lc_db3 = assign62330_e80775_d_b3;
        var_lc_db4 = assign62330_e80775_d_b4;
        var_lc_db5 = assign62330_e80775_d_b5;
        var_lc_db6 = assign62330_e80775_d_b6;

        let (assign62340_e80785, assign62340_e80785_d_n0, assign62340_e80785_d_n1, assign62340_e80785_d_n2, assign62340_e80785_d_n3, assign62340_e80785_d_n4, assign62340_e80785_d_n5, assign62340_e80785_d_n6, assign62340_e80785_d_n7, assign62340_e80785_d_n8, assign62340_e80785_d_n9, assign62340_e80785_d_n10, assign62340_e80785_d_n11, assign62340_e80785_d_b0, assign62340_e80785_d_b1, assign62340_e80785_d_b2, assign62340_e80785_d_b3, assign62340_e80785_d_b4, assign62340_e80785_d_b5, assign62340_e80785_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62340_e80782: f64 = (var_lc * var_lc);
        let assign62340_e80783: f64 = (1.0 / assign62340_e80782);
        (assign62340_e80783, (-(((var_lc_dn0 * var_lc) + (var_lc * var_lc_dn0)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_dn1 * var_lc) + (var_lc * var_lc_dn1)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_dn2 * var_lc) + (var_lc * var_lc_dn2)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_dn3 * var_lc) + (var_lc * var_lc_dn3)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_dn4 * var_lc) + (var_lc * var_lc_dn4)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_dn5 * var_lc) + (var_lc * var_lc_dn5)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_dn6 * var_lc) + (var_lc * var_lc_dn6)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_dn7 * var_lc) + (var_lc * var_lc_dn7)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_dn8 * var_lc) + (var_lc * var_lc_dn8)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_dn9 * var_lc) + (var_lc * var_lc_dn9)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_dn10 * var_lc) + (var_lc * var_lc_dn10)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_dn11 * var_lc) + (var_lc * var_lc_dn11)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_db0 * var_lc) + (var_lc * var_lc_db0)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_db1 * var_lc) + (var_lc * var_lc_db1)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_db2 * var_lc) + (var_lc * var_lc_db2)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_db3 * var_lc) + (var_lc * var_lc_db3)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_db4 * var_lc) + (var_lc * var_lc_db4)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_db5 * var_lc) + (var_lc * var_lc_db5)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_db6 * var_lc) + (var_lc * var_lc_db6)) / (assign62340_e80782 * assign62340_e80782))),)
    } else {
        (var_lcinv2, var_lcinv2_dn0, var_lcinv2_dn1, var_lcinv2_dn2, var_lcinv2_dn3, var_lcinv2_dn4, var_lcinv2_dn5, var_lcinv2_dn6, var_lcinv2_dn7, var_lcinv2_dn8, var_lcinv2_dn9, var_lcinv2_dn10, var_lcinv2_dn11, var_lcinv2_db0, var_lcinv2_db1, var_lcinv2_db2, var_lcinv2_db3, var_lcinv2_db4, var_lcinv2_db5, var_lcinv2_db6,)
    }
};
        var_lcinv2 = assign62340_e80785;
        var_lcinv2_dn0 = assign62340_e80785_d_n0;
        var_lcinv2_dn1 = assign62340_e80785_d_n1;
        var_lcinv2_dn2 = assign62340_e80785_d_n2;
        var_lcinv2_dn3 = assign62340_e80785_d_n3;
        var_lcinv2_dn4 = assign62340_e80785_d_n4;
        var_lcinv2_dn5 = assign62340_e80785_d_n5;
        var_lcinv2_dn6 = assign62340_e80785_d_n6;
        var_lcinv2_dn7 = assign62340_e80785_d_n7;
        var_lcinv2_dn8 = assign62340_e80785_d_n8;
        var_lcinv2_dn9 = assign62340_e80785_d_n9;
        var_lcinv2_dn10 = assign62340_e80785_d_n10;
        var_lcinv2_dn11 = assign62340_e80785_d_n11;
        var_lcinv2_db0 = assign62340_e80785_d_b0;
        var_lcinv2_db1 = assign62340_e80785_d_b1;
        var_lcinv2_db2 = assign62340_e80785_d_b2;
        var_lcinv2_db3 = assign62340_e80785_d_b3;
        var_lcinv2_db4 = assign62340_e80785_d_b4;
        var_lcinv2_db5 = assign62340_e80785_d_b5;
        var_lcinv2_db6 = assign62340_e80785_d_b6;

        let (assign62350_e80795, assign62350_e80795_d_n0, assign62350_e80795_d_n1, assign62350_e80795_d_n2, assign62350_e80795_d_n3, assign62350_e80795_d_n4, assign62350_e80795_d_n5, assign62350_e80795_d_n6, assign62350_e80795_d_n7, assign62350_e80795_d_n8, assign62350_e80795_d_n9, assign62350_e80795_d_n10, assign62350_e80795_d_n11, assign62350_e80795_d_b0, assign62350_e80795_d_b1, assign62350_e80795_d_b2, assign62350_e80795_d_b3, assign62350_e80795_d_b4, assign62350_e80795_d_b5, assign62350_e80795_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62350_e80791: f64 = (var_bet_i * var_qim1_dc);
        let assign62350_e80793: f64 = (assign62350_e80791 * var_gvsatinv_dc);
        (assign62350_e80793, (((var_bet_i * var_qim1_dc_dn0) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn0)), (((var_bet_i * var_qim1_dc_dn1) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn1)), (((var_bet_i * var_qim1_dc_dn2) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn2)), (((var_bet_i * var_qim1_dc_dn3) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn3)), (((var_bet_i * var_qim1_dc_dn4) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn4)), (((var_bet_i * var_qim1_dc_dn5) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn5)), (((var_bet_i * var_qim1_dc_dn6) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn6)), (((var_bet_i * var_qim1_dc_dn7) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn7)), (((var_bet_i * var_qim1_dc_dn8) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn8)), (((var_bet_i * var_qim1_dc_dn9) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn9)), (((var_bet_i * var_qim1_dc_dn10) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn10)), (((var_bet_i * var_qim1_dc_dn11) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn11)), (((var_bet_i * var_qim1_dc_db0) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_db0)), (((var_bet_i * var_qim1_dc_db1) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_db1)), (((var_bet_i * var_qim1_dc_db2) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_db2)), (((var_bet_i * var_qim1_dc_db3) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_db3)), (((var_bet_i * var_qim1_dc_db4) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_db4)), (((var_bet_i * var_qim1_dc_db5) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_db5)), (((var_bet_i * var_qim1_dc_db6) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_db6)),)
    } else {
        (var_g_ideal, var_g_ideal_dn0, var_g_ideal_dn1, var_g_ideal_dn2, var_g_ideal_dn3, var_g_ideal_dn4, var_g_ideal_dn5, var_g_ideal_dn6, var_g_ideal_dn7, var_g_ideal_dn8, var_g_ideal_dn9, var_g_ideal_dn10, var_g_ideal_dn11, var_g_ideal_db0, var_g_ideal_db1, var_g_ideal_db2, var_g_ideal_db3, var_g_ideal_db4, var_g_ideal_db5, var_g_ideal_db6,)
    }
};
        var_g_ideal = assign62350_e80795;
        var_g_ideal_dn0 = assign62350_e80795_d_n0;
        var_g_ideal_dn1 = assign62350_e80795_d_n1;
        var_g_ideal_dn2 = assign62350_e80795_d_n2;
        var_g_ideal_dn3 = assign62350_e80795_d_n3;
        var_g_ideal_dn4 = assign62350_e80795_d_n4;
        var_g_ideal_dn5 = assign62350_e80795_d_n5;
        var_g_ideal_dn6 = assign62350_e80795_d_n6;
        var_g_ideal_dn7 = assign62350_e80795_d_n7;
        var_g_ideal_dn8 = assign62350_e80795_d_n8;
        var_g_ideal_dn9 = assign62350_e80795_d_n9;
        var_g_ideal_dn10 = assign62350_e80795_d_n10;
        var_g_ideal_dn11 = assign62350_e80795_d_n11;
        var_g_ideal_db0 = assign62350_e80795_d_b0;
        var_g_ideal_db1 = assign62350_e80795_d_b1;
        var_g_ideal_db2 = assign62350_e80795_d_b2;
        var_g_ideal_db3 = assign62350_e80795_d_b3;
        var_g_ideal_db4 = assign62350_e80795_d_b4;
        var_g_ideal_db5 = assign62350_e80795_d_b5;
        var_g_ideal_db6 = assign62350_e80795_d_b6;

        let (assign62360_e80815, assign62360_e80815_d_n0, assign62360_e80815_d_n1, assign62360_e80815_d_n2, assign62360_e80815_d_n3, assign62360_e80815_d_n4, assign62360_e80815_d_n5, assign62360_e80815_d_n6, assign62360_e80815_d_n7, assign62360_e80815_d_n8, assign62360_e80815_d_n9, assign62360_e80815_d_n10, assign62360_e80815_d_n11, assign62360_e80815_d_b0, assign62360_e80815_d_b1, assign62360_e80815_d_b2, assign62360_e80815_d_b3, assign62360_e80815_d_b4, assign62360_e80815_d_b5, assign62360_e80815_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62360_e80802: f64 = (12.0 * var_t2);
        let assign62360_e80803: f64 = (var_t1 + assign62360_e80802);
        let assign62360_e80807: f64 = (1.0 + var_t1);
        let assign62360_e80809: f64 = (assign62360_e80807 * var_t2);
        let assign62360_e80811: f64 = (assign62360_e80809 * var_r);
        let assign62360_e80812: f64 = (24.0 * assign62360_e80811);
        let assign62360_e80813: f64 = (assign62360_e80803 - assign62360_e80812);
        (assign62360_e80813, ((var_t1_dn0 + (12.0 * var_t2_dn0)) - (24.0 * ((((var_t1_dn0 * var_t2) + (assign62360_e80807 * var_t2_dn0)) * var_r) + (assign62360_e80809 * var_r_dn0)))), ((var_t1_dn1 + (12.0 * var_t2_dn1)) - (24.0 * ((((var_t1_dn1 * var_t2) + (assign62360_e80807 * var_t2_dn1)) * var_r) + (assign62360_e80809 * var_r_dn1)))), ((var_t1_dn2 + (12.0 * var_t2_dn2)) - (24.0 * ((((var_t1_dn2 * var_t2) + (assign62360_e80807 * var_t2_dn2)) * var_r) + (assign62360_e80809 * var_r_dn2)))), ((var_t1_dn3 + (12.0 * var_t2_dn3)) - (24.0 * ((((var_t1_dn3 * var_t2) + (assign62360_e80807 * var_t2_dn3)) * var_r) + (assign62360_e80809 * var_r_dn3)))), ((var_t1_dn4 + (12.0 * var_t2_dn4)) - (24.0 * ((((var_t1_dn4 * var_t2) + (assign62360_e80807 * var_t2_dn4)) * var_r) + (assign62360_e80809 * var_r_dn4)))), ((var_t1_dn5 + (12.0 * var_t2_dn5)) - (24.0 * ((((var_t1_dn5 * var_t2) + (assign62360_e80807 * var_t2_dn5)) * var_r) + (assign62360_e80809 * var_r_dn5)))), ((var_t1_dn6 + (12.0 * var_t2_dn6)) - (24.0 * ((((var_t1_dn6 * var_t2) + (assign62360_e80807 * var_t2_dn6)) * var_r) + (assign62360_e80809 * var_r_dn6)))), ((var_t1_dn7 + (12.0 * var_t2_dn7)) - (24.0 * ((((var_t1_dn7 * var_t2) + (assign62360_e80807 * var_t2_dn7)) * var_r) + (assign62360_e80809 * var_r_dn7)))), ((var_t1_dn8 + (12.0 * var_t2_dn8)) - (24.0 * ((((var_t1_dn8 * var_t2) + (assign62360_e80807 * var_t2_dn8)) * var_r) + (assign62360_e80809 * var_r_dn8)))), ((var_t1_dn9 + (12.0 * var_t2_dn9)) - (24.0 * ((((var_t1_dn9 * var_t2) + (assign62360_e80807 * var_t2_dn9)) * var_r) + (assign62360_e80809 * var_r_dn9)))), ((var_t1_dn10 + (12.0 * var_t2_dn10)) - (24.0 * ((((var_t1_dn10 * var_t2) + (assign62360_e80807 * var_t2_dn10)) * var_r) + (assign62360_e80809 * var_r_dn10)))), ((var_t1_dn11 + (12.0 * var_t2_dn11)) - (24.0 * ((((var_t1_dn11 * var_t2) + (assign62360_e80807 * var_t2_dn11)) * var_r) + (assign62360_e80809 * var_r_dn11)))), ((var_t1_db0 + (12.0 * var_t2_db0)) - (24.0 * ((((var_t1_db0 * var_t2) + (assign62360_e80807 * var_t2_db0)) * var_r) + (assign62360_e80809 * var_r_db0)))), ((var_t1_db1 + (12.0 * var_t2_db1)) - (24.0 * ((((var_t1_db1 * var_t2) + (assign62360_e80807 * var_t2_db1)) * var_r) + (assign62360_e80809 * var_r_db1)))), ((var_t1_db2 + (12.0 * var_t2_db2)) - (24.0 * ((((var_t1_db2 * var_t2) + (assign62360_e80807 * var_t2_db2)) * var_r) + (assign62360_e80809 * var_r_db2)))), ((var_t1_db3 + (12.0 * var_t2_db3)) - (24.0 * ((((var_t1_db3 * var_t2) + (assign62360_e80807 * var_t2_db3)) * var_r) + (assign62360_e80809 * var_r_db3)))), ((var_t1_db4 + (12.0 * var_t2_db4)) - (24.0 * ((((var_t1_db4 * var_t2) + (assign62360_e80807 * var_t2_db4)) * var_r) + (assign62360_e80809 * var_r_db4)))), ((var_t1_db5 + (12.0 * var_t2_db5)) - (24.0 * ((((var_t1_db5 * var_t2) + (assign62360_e80807 * var_t2_db5)) * var_r) + (assign62360_e80809 * var_r_db5)))), ((var_t1_db6 + (12.0 * var_t2_db6)) - (24.0 * ((((var_t1_db6 * var_t2) + (assign62360_e80807 * var_t2_db6)) * var_r) + (assign62360_e80809 * var_r_db6)))),)
    } else {
        (var_mid, var_mid_dn0, var_mid_dn1, var_mid_dn2, var_mid_dn3, var_mid_dn4, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn9, var_mid_dn10, var_mid_dn11, var_mid_db0, var_mid_db1, var_mid_db2, var_mid_db3, var_mid_db4, var_mid_db5, var_mid_db6,)
    }
};
        var_mid = assign62360_e80815;
        var_mid_dn0 = assign62360_e80815_d_n0;
        var_mid_dn1 = assign62360_e80815_d_n1;
        var_mid_dn2 = assign62360_e80815_d_n2;
        var_mid_dn3 = assign62360_e80815_d_n3;
        var_mid_dn4 = assign62360_e80815_d_n4;
        var_mid_dn5 = assign62360_e80815_d_n5;
        var_mid_dn6 = assign62360_e80815_d_n6;
        var_mid_dn7 = assign62360_e80815_d_n7;
        var_mid_dn8 = assign62360_e80815_d_n8;
        var_mid_dn9 = assign62360_e80815_d_n9;
        var_mid_dn10 = assign62360_e80815_d_n10;
        var_mid_dn11 = assign62360_e80815_d_n11;
        var_mid_db0 = assign62360_e80815_d_b0;
        var_mid_db1 = assign62360_e80815_d_b1;
        var_mid_db2 = assign62360_e80815_d_b2;
        var_mid_db3 = assign62360_e80815_d_b3;
        var_mid_db4 = assign62360_e80815_d_b4;
        var_mid_db5 = assign62360_e80815_d_b5;
        var_mid_db6 = assign62360_e80815_d_b6;

        let (assign62370_e80826, assign62370_e80826_d_n0, assign62370_e80826_d_n1, assign62370_e80826_d_n2, assign62370_e80826_d_n3, assign62370_e80826_d_n4, assign62370_e80826_d_n5, assign62370_e80826_d_n6, assign62370_e80826_d_n7, assign62370_e80826_d_n8, assign62370_e80826_d_n9, assign62370_e80826_d_n10, assign62370_e80826_d_n11, assign62370_e80826_d_b0, assign62370_e80826_d_b1, assign62370_e80826_d_b2, assign62370_e80826_d_b3, assign62370_e80826_d_b4, assign62370_e80826_d_b5, assign62370_e80826_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let (assign62370_e80824, assign62370_e80824_d_n0, assign62370_e80824_d_n1, assign62370_e80824_d_n2, assign62370_e80824_d_n3, assign62370_e80824_d_n4, assign62370_e80824_d_n5, assign62370_e80824_d_n6, assign62370_e80824_d_n7, assign62370_e80824_d_n8, assign62370_e80824_d_n9, assign62370_e80824_d_n10, assign62370_e80824_d_n11, assign62370_e80824_d_b0, assign62370_e80824_d_b1, assign62370_e80824_d_b2, assign62370_e80824_d_b3, assign62370_e80824_d_b4, assign62370_e80824_d_b5, assign62370_e80824_d_b6,) = {
            if (var_mid > 1e-40) {
                (var_mid, var_mid_dn0, var_mid_dn1, var_mid_dn2, var_mid_dn3, var_mid_dn4, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn9, var_mid_dn10, var_mid_dn11, var_mid_db0, var_mid_db1, var_mid_db2, var_mid_db3, var_mid_db4, var_mid_db5, var_mid_db6,)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62370_e80824, assign62370_e80824_d_n0, assign62370_e80824_d_n1, assign62370_e80824_d_n2, assign62370_e80824_d_n3, assign62370_e80824_d_n4, assign62370_e80824_d_n5, assign62370_e80824_d_n6, assign62370_e80824_d_n7, assign62370_e80824_d_n8, assign62370_e80824_d_n9, assign62370_e80824_d_n10, assign62370_e80824_d_n11, assign62370_e80824_d_b0, assign62370_e80824_d_b1, assign62370_e80824_d_b2, assign62370_e80824_d_b3, assign62370_e80824_d_b4, assign62370_e80824_d_b5, assign62370_e80824_d_b6,)
    } else {
        (var_mid, var_mid_dn0, var_mid_dn1, var_mid_dn2, var_mid_dn3, var_mid_dn4, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn9, var_mid_dn10, var_mid_dn11, var_mid_db0, var_mid_db1, var_mid_db2, var_mid_db3, var_mid_db4, var_mid_db5, var_mid_db6,)
    }
};
        var_mid = assign62370_e80826;
        var_mid_dn0 = assign62370_e80826_d_n0;
        var_mid_dn1 = assign62370_e80826_d_n1;
        var_mid_dn2 = assign62370_e80826_d_n2;
        var_mid_dn3 = assign62370_e80826_d_n3;
        var_mid_dn4 = assign62370_e80826_d_n4;
        var_mid_dn5 = assign62370_e80826_d_n5;
        var_mid_dn6 = assign62370_e80826_d_n6;
        var_mid_dn7 = assign62370_e80826_d_n7;
        var_mid_dn8 = assign62370_e80826_d_n8;
        var_mid_dn9 = assign62370_e80826_d_n9;
        var_mid_dn10 = assign62370_e80826_d_n10;
        var_mid_dn11 = assign62370_e80826_d_n11;
        var_mid_db0 = assign62370_e80826_d_b0;
        var_mid_db1 = assign62370_e80826_d_b1;
        var_mid_db2 = assign62370_e80826_d_b2;
        var_mid_db3 = assign62370_e80826_d_b3;
        var_mid_db4 = assign62370_e80826_d_b4;
        var_mid_db5 = assign62370_e80826_d_b5;
        var_mid_db6 = assign62370_e80826_d_b6;

        let (assign62380_e80836, assign62380_e80836_d_n0, assign62380_e80836_d_n1, assign62380_e80836_d_n2, assign62380_e80836_d_n3, assign62380_e80836_d_n4, assign62380_e80836_d_n5, assign62380_e80836_d_n6, assign62380_e80836_d_n7, assign62380_e80836_d_n8, assign62380_e80836_d_n9, assign62380_e80836_d_n10, assign62380_e80836_d_n11, assign62380_e80836_d_b0, assign62380_e80836_d_b1, assign62380_e80836_d_b2, assign62380_e80836_d_b3, assign62380_e80836_d_b4, assign62380_e80836_d_b5, assign62380_e80836_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62380_e80832: f64 = (var_g_ideal * var_lcinv2);
        let assign62380_e80834: f64 = (assign62380_e80832 * var_mid);
        (assign62380_e80834, ((((var_g_ideal_dn0 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn0)) * var_mid) + (assign62380_e80832 * var_mid_dn0)), ((((var_g_ideal_dn1 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn1)) * var_mid) + (assign62380_e80832 * var_mid_dn1)), ((((var_g_ideal_dn2 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn2)) * var_mid) + (assign62380_e80832 * var_mid_dn2)), ((((var_g_ideal_dn3 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn3)) * var_mid) + (assign62380_e80832 * var_mid_dn3)), ((((var_g_ideal_dn4 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn4)) * var_mid) + (assign62380_e80832 * var_mid_dn4)), ((((var_g_ideal_dn5 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn5)) * var_mid) + (assign62380_e80832 * var_mid_dn5)), ((((var_g_ideal_dn6 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn6)) * var_mid) + (assign62380_e80832 * var_mid_dn6)), ((((var_g_ideal_dn7 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn7)) * var_mid) + (assign62380_e80832 * var_mid_dn7)), ((((var_g_ideal_dn8 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn8)) * var_mid) + (assign62380_e80832 * var_mid_dn8)), ((((var_g_ideal_dn9 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn9)) * var_mid) + (assign62380_e80832 * var_mid_dn9)), ((((var_g_ideal_dn10 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn10)) * var_mid) + (assign62380_e80832 * var_mid_dn10)), ((((var_g_ideal_dn11 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn11)) * var_mid) + (assign62380_e80832 * var_mid_dn11)), ((((var_g_ideal_db0 * var_lcinv2) + (var_g_ideal * var_lcinv2_db0)) * var_mid) + (assign62380_e80832 * var_mid_db0)), ((((var_g_ideal_db1 * var_lcinv2) + (var_g_ideal * var_lcinv2_db1)) * var_mid) + (assign62380_e80832 * var_mid_db1)), ((((var_g_ideal_db2 * var_lcinv2) + (var_g_ideal * var_lcinv2_db2)) * var_mid) + (assign62380_e80832 * var_mid_db2)), ((((var_g_ideal_db3 * var_lcinv2) + (var_g_ideal * var_lcinv2_db3)) * var_mid) + (assign62380_e80832 * var_mid_db3)), ((((var_g_ideal_db4 * var_lcinv2) + (var_g_ideal * var_lcinv2_db4)) * var_mid) + (assign62380_e80832 * var_mid_db4)), ((((var_g_ideal_db5 * var_lcinv2) + (var_g_ideal * var_lcinv2_db5)) * var_mid) + (assign62380_e80832 * var_mid_db5)), ((((var_g_ideal_db6 * var_lcinv2) + (var_g_ideal * var_lcinv2_db6)) * var_mid) + (assign62380_e80832 * var_mid_db6)),)
    } else {
        (var_mid, var_mid_dn0, var_mid_dn1, var_mid_dn2, var_mid_dn3, var_mid_dn4, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn9, var_mid_dn10, var_mid_dn11, var_mid_db0, var_mid_db1, var_mid_db2, var_mid_db3, var_mid_db4, var_mid_db5, var_mid_db6,)
    }
};
        var_mid = assign62380_e80836;
        var_mid_dn0 = assign62380_e80836_d_n0;
        var_mid_dn1 = assign62380_e80836_d_n1;
        var_mid_dn2 = assign62380_e80836_d_n2;
        var_mid_dn3 = assign62380_e80836_d_n3;
        var_mid_dn4 = assign62380_e80836_d_n4;
        var_mid_dn5 = assign62380_e80836_d_n5;
        var_mid_dn6 = assign62380_e80836_d_n6;
        var_mid_dn7 = assign62380_e80836_d_n7;
        var_mid_dn8 = assign62380_e80836_d_n8;
        var_mid_dn9 = assign62380_e80836_d_n9;
        var_mid_dn10 = assign62380_e80836_d_n10;
        var_mid_dn11 = assign62380_e80836_d_n11;
        var_mid_db0 = assign62380_e80836_d_b0;
        var_mid_db1 = assign62380_e80836_d_b1;
        var_mid_db2 = assign62380_e80836_d_b2;
        var_mid_db3 = assign62380_e80836_d_b3;
        var_mid_db4 = assign62380_e80836_d_b4;
        var_mid_db5 = assign62380_e80836_d_b5;
        var_mid_db6 = assign62380_e80836_d_b6;

        let assign62390_e80839: f64 = if var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1763 = assign62390_e80839;

        let (assign62400_e80849, assign62400_e80849_d_n0, assign62400_e80849_d_n1, assign62400_e80849_d_n2, assign62400_e80849_d_n3, assign62400_e80849_d_n4, assign62400_e80849_d_n5, assign62400_e80849_d_n6, assign62400_e80849_d_n7, assign62400_e80849_d_n8, assign62400_e80849_d_n9, assign62400_e80849_d_n10, assign62400_e80849_d_n11, assign62400_e80849_d_b0, assign62400_e80849_d_b1, assign62400_e80849_d_b2, assign62400_e80849_d_b3, assign62400_e80849_d_b4, assign62400_e80849_d_b5, assign62400_e80849_d_b6,) = {
    if (((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) && (var_guard1763 != 0.0)) {
        let assign62400_e80847: f64 = (var_thesateff_dc / var_gmob_dc);
        (assign62400_e80847, (((var_thesateff_dc_dn0 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn0)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn1 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn1)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn2 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn2)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn3 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn3)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn4 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn4)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn5 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn5)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn6 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn6)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn7 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn7)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn8 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn8)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn9 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn9)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn10 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn10)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn11 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn11)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_db0 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_db0)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_db1 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_db1)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_db2 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_db2)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_db3 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_db3)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_db4 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_db4)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_db5 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_db5)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_db6 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_db6)) / (var_gmob_dc * var_gmob_dc)),)
    } else {
        (var_thesat1_exc, var_thesat1_exc_dn0, var_thesat1_exc_dn1, var_thesat1_exc_dn2, var_thesat1_exc_dn3, var_thesat1_exc_dn4, var_thesat1_exc_dn5, var_thesat1_exc_dn6, var_thesat1_exc_dn7, var_thesat1_exc_dn8, var_thesat1_exc_dn9, var_thesat1_exc_dn10, var_thesat1_exc_dn11, var_thesat1_exc_db0, var_thesat1_exc_db1, var_thesat1_exc_db2, var_thesat1_exc_db3, var_thesat1_exc_db4, var_thesat1_exc_db5, var_thesat1_exc_db6,)
    }
};
        var_thesat1_exc = assign62400_e80849;
        var_thesat1_exc_dn0 = assign62400_e80849_d_n0;
        var_thesat1_exc_dn1 = assign62400_e80849_d_n1;
        var_thesat1_exc_dn2 = assign62400_e80849_d_n2;
        var_thesat1_exc_dn3 = assign62400_e80849_d_n3;
        var_thesat1_exc_dn4 = assign62400_e80849_d_n4;
        var_thesat1_exc_dn5 = assign62400_e80849_d_n5;
        var_thesat1_exc_dn6 = assign62400_e80849_d_n6;
        var_thesat1_exc_dn7 = assign62400_e80849_d_n7;
        var_thesat1_exc_dn8 = assign62400_e80849_d_n8;
        var_thesat1_exc_dn9 = assign62400_e80849_d_n9;
        var_thesat1_exc_dn10 = assign62400_e80849_d_n10;
        var_thesat1_exc_dn11 = assign62400_e80849_d_n11;
        var_thesat1_exc_db0 = assign62400_e80849_d_b0;
        var_thesat1_exc_db1 = assign62400_e80849_d_b1;
        var_thesat1_exc_db2 = assign62400_e80849_d_b2;
        var_thesat1_exc_db3 = assign62400_e80849_d_b3;
        var_thesat1_exc_db4 = assign62400_e80849_d_b4;
        var_thesat1_exc_db5 = assign62400_e80849_d_b5;
        var_thesat1_exc_db6 = assign62400_e80849_d_b6;

        let (assign62410_e80863, assign62410_e80863_d_n0, assign62410_e80863_d_n1, assign62410_e80863_d_n2, assign62410_e80863_d_n3, assign62410_e80863_d_n4, assign62410_e80863_d_n5, assign62410_e80863_d_n6, assign62410_e80863_d_n7, assign62410_e80863_d_n8, assign62410_e80863_d_n9, assign62410_e80863_d_n10, assign62410_e80863_d_n11, assign62410_e80863_d_b0, assign62410_e80863_d_b1, assign62410_e80863_d_b2, assign62410_e80863_d_b3, assign62410_e80863_d_b4, assign62410_e80863_d_b5, assign62410_e80863_d_b6,) = {
    if (((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) && (var_guard1763 != 0.0)) {
        let assign62410_e80857: f64 = (var_thesat1_exc * var_thesat1_exc);
        let assign62410_e80859: f64 = (assign62410_e80857 * var_dps_dc);
        let assign62410_e80861: f64 = (assign62410_e80859 * var_dps_dc);
        (assign62410_e80861, ((((((var_thesat1_exc_dn0 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn0)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn0)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn0)), ((((((var_thesat1_exc_dn1 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn1)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn1)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn1)), ((((((var_thesat1_exc_dn2 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn2)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn2)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn2)), ((((((var_thesat1_exc_dn3 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn3)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn3)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn3)), ((((((var_thesat1_exc_dn4 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn4)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn4)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn4)), ((((((var_thesat1_exc_dn5 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn5)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn5)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn5)), ((((((var_thesat1_exc_dn6 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn6)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn6)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn6)), ((((((var_thesat1_exc_dn7 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn7)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn7)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn7)), ((((((var_thesat1_exc_dn8 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn8)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn8)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn8)), ((((((var_thesat1_exc_dn9 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn9)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn9)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn9)), ((((((var_thesat1_exc_dn10 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn10)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn10)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn10)), ((((((var_thesat1_exc_dn11 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn11)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn11)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn11)), ((((((var_thesat1_exc_db0 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_db0)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_db0)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_db0)), ((((((var_thesat1_exc_db1 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_db1)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_db1)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_db1)), ((((((var_thesat1_exc_db2 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_db2)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_db2)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_db2)), ((((((var_thesat1_exc_db3 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_db3)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_db3)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_db3)), ((((((var_thesat1_exc_db4 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_db4)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_db4)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_db4)), ((((((var_thesat1_exc_db5 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_db5)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_db5)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_db5)), ((((((var_thesat1_exc_db6 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_db6)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_db6)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_db6)),)
    } else {
        (var_zsat_exc, var_zsat_exc_dn0, var_zsat_exc_dn1, var_zsat_exc_dn2, var_zsat_exc_dn3, var_zsat_exc_dn4, var_zsat_exc_dn5, var_zsat_exc_dn6, var_zsat_exc_dn7, var_zsat_exc_dn8, var_zsat_exc_dn9, var_zsat_exc_dn10, var_zsat_exc_dn11, var_zsat_exc_db0, var_zsat_exc_db1, var_zsat_exc_db2, var_zsat_exc_db3, var_zsat_exc_db4, var_zsat_exc_db5, var_zsat_exc_db6,)
    }
};
        var_zsat_exc = assign62410_e80863;
        var_zsat_exc_dn0 = assign62410_e80863_d_n0;
        var_zsat_exc_dn1 = assign62410_e80863_d_n1;
        var_zsat_exc_dn2 = assign62410_e80863_d_n2;
        var_zsat_exc_dn3 = assign62410_e80863_d_n3;
        var_zsat_exc_dn4 = assign62410_e80863_d_n4;
        var_zsat_exc_dn5 = assign62410_e80863_d_n5;
        var_zsat_exc_dn6 = assign62410_e80863_d_n6;
        var_zsat_exc_dn7 = assign62410_e80863_d_n7;
        var_zsat_exc_dn8 = assign62410_e80863_d_n8;
        var_zsat_exc_dn9 = assign62410_e80863_d_n9;
        var_zsat_exc_dn10 = assign62410_e80863_d_n10;
        var_zsat_exc_dn11 = assign62410_e80863_d_n11;
        var_zsat_exc_db0 = assign62410_e80863_d_b0;
        var_zsat_exc_db1 = assign62410_e80863_d_b1;
        var_zsat_exc_db2 = assign62410_e80863_d_b2;
        var_zsat_exc_db3 = assign62410_e80863_d_b3;
        var_zsat_exc_db4 = assign62410_e80863_d_b4;
        var_zsat_exc_db5 = assign62410_e80863_d_b5;
        var_zsat_exc_db6 = assign62410_e80863_d_b6;

        let assign62420_e80866: f64 = (-1.0);
        let assign62420_e80867: f64 = if var_chnl_type == assign62420_e80866 { 1.0 } else { 0.0 };
        var_guard1764 = assign62420_e80867;

        let (assign62430_e80883, assign62430_e80883_d_n0, assign62430_e80883_d_n1, assign62430_e80883_d_n2, assign62430_e80883_d_n3, assign62430_e80883_d_n4, assign62430_e80883_d_n5, assign62430_e80883_d_n6, assign62430_e80883_d_n7, assign62430_e80883_d_n8, assign62430_e80883_d_n9, assign62430_e80883_d_n10, assign62430_e80883_d_n11, assign62430_e80883_d_b0, assign62430_e80883_d_b1, assign62430_e80883_d_b2, assign62430_e80883_d_b3, assign62430_e80883_d_b4, assign62430_e80883_d_b5, assign62430_e80883_d_b6,) = {
    if ((((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) && (var_guard1763 != 0.0)) && (var_guard1764 != 0.0)) {
        let assign62430_e80879: f64 = (var_thesat1_exc * var_dps_dc);
        let assign62430_e80880: f64 = (1.0 + assign62430_e80879);
        let assign62430_e80881: f64 = (var_zsat_exc / assign62430_e80880);
        (assign62430_e80881, (((var_zsat_exc_dn0 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn0 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn0)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_dn1 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn1 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn1)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_dn2 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn2 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn2)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_dn3 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn3 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn3)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_dn4 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn4 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn4)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_dn5 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn5 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn5)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_dn6 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn6 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn6)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_dn7 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn7 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn7)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_dn8 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn8 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn8)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_dn9 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn9 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn9)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_dn10 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn10 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn10)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_dn11 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn11 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn11)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_db0 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_db0 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_db0)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_db1 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_db1 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_db1)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_db2 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_db2 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_db2)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_db3 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_db3 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_db3)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_db4 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_db4 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_db4)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_db5 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_db5 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_db5)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_db6 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_db6 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_db6)))) / (assign62430_e80880 * assign62430_e80880)),)
    } else {
        (var_zsat_exc, var_zsat_exc_dn0, var_zsat_exc_dn1, var_zsat_exc_dn2, var_zsat_exc_dn3, var_zsat_exc_dn4, var_zsat_exc_dn5, var_zsat_exc_dn6, var_zsat_exc_dn7, var_zsat_exc_dn8, var_zsat_exc_dn9, var_zsat_exc_dn10, var_zsat_exc_dn11, var_zsat_exc_db0, var_zsat_exc_db1, var_zsat_exc_db2, var_zsat_exc_db3, var_zsat_exc_db4, var_zsat_exc_db5, var_zsat_exc_db6,)
    }
};
        var_zsat_exc = assign62430_e80883;
        var_zsat_exc_dn0 = assign62430_e80883_d_n0;
        var_zsat_exc_dn1 = assign62430_e80883_d_n1;
        var_zsat_exc_dn2 = assign62430_e80883_d_n2;
        var_zsat_exc_dn3 = assign62430_e80883_d_n3;
        var_zsat_exc_dn4 = assign62430_e80883_d_n4;
        var_zsat_exc_dn5 = assign62430_e80883_d_n5;
        var_zsat_exc_dn6 = assign62430_e80883_d_n6;
        var_zsat_exc_dn7 = assign62430_e80883_d_n7;
        var_zsat_exc_dn8 = assign62430_e80883_d_n8;
        var_zsat_exc_dn9 = assign62430_e80883_d_n9;
        var_zsat_exc_dn10 = assign62430_e80883_d_n10;
        var_zsat_exc_dn11 = assign62430_e80883_d_n11;
        var_zsat_exc_db0 = assign62430_e80883_d_b0;
        var_zsat_exc_db1 = assign62430_e80883_d_b1;
        var_zsat_exc_db2 = assign62430_e80883_d_b2;
        var_zsat_exc_db3 = assign62430_e80883_d_b3;
        var_zsat_exc_db4 = assign62430_e80883_d_b4;
        var_zsat_exc_db5 = assign62430_e80883_d_b5;
        var_zsat_exc_db6 = assign62430_e80883_d_b6;

        let (assign62440_e80902, assign62440_e80902_d_n0, assign62440_e80902_d_n1, assign62440_e80902_d_n2, assign62440_e80902_d_n3, assign62440_e80902_d_n4, assign62440_e80902_d_n5, assign62440_e80902_d_n6, assign62440_e80902_d_n7, assign62440_e80902_d_n8, assign62440_e80902_d_n9, assign62440_e80902_d_n10, assign62440_e80902_d_n11, assign62440_e80902_d_b0, assign62440_e80902_d_b1, assign62440_e80902_d_b2, assign62440_e80902_d_b3, assign62440_e80902_d_b4, assign62440_e80902_d_b5, assign62440_e80902_d_b6,) = {
    if (((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) && (var_guard1763 != 0.0)) {
        let assign62440_e80895: f64 = (2.0 * var_zsat_exc);
        let assign62440_e80896: f64 = (1.0 + assign62440_e80895);
        let assign62440_e80897: f64 = (assign62440_e80896).sqrt();
        let assign62440_e80898: f64 = (1.0 + assign62440_e80897);
        let assign62440_e80899: f64 = (var_gmob_dc * assign62440_e80898);
        let assign62440_e80900: f64 = (0.5 * assign62440_e80899);
        (assign62440_e80900, (0.5 * ((var_gmob_dc_dn0 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn0) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_dn1 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn1) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_dn2 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn2) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_dn3 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn3) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_dn4 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn4) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_dn5 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn5) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_dn6 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn6) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_dn7 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn7) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_dn8 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn8) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_dn9 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn9) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_dn10 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn10) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_dn11 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn11) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_db0 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_db0) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_db1 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_db1) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_db2 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_db2) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_db3 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_db3) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_db4 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_db4) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_db5 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_db5) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_db6 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_db6) / (2.0 * assign62440_e80897))))),)
    } else {
        (var_gvsat_exc, var_gvsat_exc_dn0, var_gvsat_exc_dn1, var_gvsat_exc_dn2, var_gvsat_exc_dn3, var_gvsat_exc_dn4, var_gvsat_exc_dn5, var_gvsat_exc_dn6, var_gvsat_exc_dn7, var_gvsat_exc_dn8, var_gvsat_exc_dn9, var_gvsat_exc_dn10, var_gvsat_exc_dn11, var_gvsat_exc_db0, var_gvsat_exc_db1, var_gvsat_exc_db2, var_gvsat_exc_db3, var_gvsat_exc_db4, var_gvsat_exc_db5, var_gvsat_exc_db6,)
    }
};
        var_gvsat_exc = assign62440_e80902;
        var_gvsat_exc_dn0 = assign62440_e80902_d_n0;
        var_gvsat_exc_dn1 = assign62440_e80902_d_n1;
        var_gvsat_exc_dn2 = assign62440_e80902_d_n2;
        var_gvsat_exc_dn3 = assign62440_e80902_d_n3;
        var_gvsat_exc_dn4 = assign62440_e80902_d_n4;
        var_gvsat_exc_dn5 = assign62440_e80902_d_n5;
        var_gvsat_exc_dn6 = assign62440_e80902_d_n6;
        var_gvsat_exc_dn7 = assign62440_e80902_d_n7;
        var_gvsat_exc_dn8 = assign62440_e80902_d_n8;
        var_gvsat_exc_dn9 = assign62440_e80902_d_n9;
        var_gvsat_exc_dn10 = assign62440_e80902_d_n10;
        var_gvsat_exc_dn11 = assign62440_e80902_d_n11;
        var_gvsat_exc_db0 = assign62440_e80902_d_b0;
        var_gvsat_exc_db1 = assign62440_e80902_d_b1;
        var_gvsat_exc_db2 = assign62440_e80902_d_b2;
        var_gvsat_exc_db3 = assign62440_e80902_d_b3;
        var_gvsat_exc_db4 = assign62440_e80902_d_b4;
        var_gvsat_exc_db5 = assign62440_e80902_d_b5;
        var_gvsat_exc_db6 = assign62440_e80902_d_b6;

        let (assign62450_e80914, assign62450_e80914_d_n0, assign62450_e80914_d_n1, assign62450_e80914_d_n2, assign62450_e80914_d_n3, assign62450_e80914_d_n4, assign62450_e80914_d_n5, assign62450_e80914_d_n6, assign62450_e80914_d_n7, assign62450_e80914_d_n8, assign62450_e80914_d_n9, assign62450_e80914_d_n10, assign62450_e80914_d_n11, assign62450_e80914_d_b0, assign62450_e80914_d_b1, assign62450_e80914_d_b2, assign62450_e80914_d_b3, assign62450_e80914_d_b4, assign62450_e80914_d_b5, assign62450_e80914_d_b6,) = {
    if (((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) && (var_guard1763 != 0.0)) {
        let assign62450_e80911: f64 = (var_gvsat_exc * var_lc);
        let assign62450_e80912: f64 = (var_gmob_dc / assign62450_e80911);
        (assign62450_e80912, (((var_gmob_dc_dn0 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn0 * var_lc) + (var_gvsat_exc * var_lc_dn0)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_dn1 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn1 * var_lc) + (var_gvsat_exc * var_lc_dn1)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_dn2 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn2 * var_lc) + (var_gvsat_exc * var_lc_dn2)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_dn3 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn3 * var_lc) + (var_gvsat_exc * var_lc_dn3)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_dn4 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn4 * var_lc) + (var_gvsat_exc * var_lc_dn4)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_dn5 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn5 * var_lc) + (var_gvsat_exc * var_lc_dn5)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_dn6 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn6 * var_lc) + (var_gvsat_exc * var_lc_dn6)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_dn7 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn7 * var_lc) + (var_gvsat_exc * var_lc_dn7)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_dn8 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn8 * var_lc) + (var_gvsat_exc * var_lc_dn8)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_dn9 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn9 * var_lc) + (var_gvsat_exc * var_lc_dn9)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_dn10 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn10 * var_lc) + (var_gvsat_exc * var_lc_dn10)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_dn11 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn11 * var_lc) + (var_gvsat_exc * var_lc_dn11)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_db0 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_db0 * var_lc) + (var_gvsat_exc * var_lc_db0)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_db1 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_db1 * var_lc) + (var_gvsat_exc * var_lc_db1)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_db2 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_db2 * var_lc) + (var_gvsat_exc * var_lc_db2)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_db3 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_db3 * var_lc) + (var_gvsat_exc * var_lc_db3)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_db4 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_db4 * var_lc) + (var_gvsat_exc * var_lc_db4)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_db5 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_db5 * var_lc) + (var_gvsat_exc * var_lc_db5)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_db6 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_db6 * var_lc) + (var_gvsat_exc * var_lc_db6)))) / (assign62450_e80911 * assign62450_e80911)),)
    } else {
        (var_gfac, var_gfac_dn0, var_gfac_dn1, var_gfac_dn2, var_gfac_dn3, var_gfac_dn4, var_gfac_dn5, var_gfac_dn6, var_gfac_dn7, var_gfac_dn8, var_gfac_dn9, var_gfac_dn10, var_gfac_dn11, var_gfac_db0, var_gfac_db1, var_gfac_db2, var_gfac_db3, var_gfac_db4, var_gfac_db5, var_gfac_db6,)
    }
};
        var_gfac = assign62450_e80914;
        var_gfac_dn0 = assign62450_e80914_d_n0;
        var_gfac_dn1 = assign62450_e80914_d_n1;
        var_gfac_dn2 = assign62450_e80914_d_n2;
        var_gfac_dn3 = assign62450_e80914_d_n3;
        var_gfac_dn4 = assign62450_e80914_d_n4;
        var_gfac_dn5 = assign62450_e80914_d_n5;
        var_gfac_dn6 = assign62450_e80914_d_n6;
        var_gfac_dn7 = assign62450_e80914_d_n7;
        var_gfac_dn8 = assign62450_e80914_d_n8;
        var_gfac_dn9 = assign62450_e80914_d_n9;
        var_gfac_dn10 = assign62450_e80914_d_n10;
        var_gfac_dn11 = assign62450_e80914_d_n11;
        var_gfac_db0 = assign62450_e80914_d_b0;
        var_gfac_db1 = assign62450_e80914_d_b1;
        var_gfac_db2 = assign62450_e80914_d_b2;
        var_gfac_db3 = assign62450_e80914_d_b3;
        var_gfac_db4 = assign62450_e80914_d_b4;
        var_gfac_db5 = assign62450_e80914_d_b5;
        var_gfac_db6 = assign62450_e80914_d_b6;

        let (assign62460_e80930, assign62460_e80930_d_n0, assign62460_e80930_d_n1, assign62460_e80930_d_n2, assign62460_e80930_d_n3, assign62460_e80930_d_n4, assign62460_e80930_d_n5, assign62460_e80930_d_n6, assign62460_e80930_d_n7, assign62460_e80930_d_n8, assign62460_e80930_d_n9, assign62460_e80930_d_n10, assign62460_e80930_d_n11, assign62460_e80930_d_b0, assign62460_e80930_d_b1, assign62460_e80930_d_b2, assign62460_e80930_d_b3, assign62460_e80930_d_b4, assign62460_e80930_d_b5, assign62460_e80930_d_b6,) = {
    if (((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) && (var_guard1763 != 0.0)) {
        let assign62460_e80922: f64 = (var_fac_exc * var_i_ds);
        let assign62460_e80924: f64 = (assign62460_e80922 * var_vdse_dc);
        let assign62460_e80926: f64 = (assign62460_e80924 * var_gfac);
        let assign62460_e80928: f64 = (assign62460_e80926 * var_gfac);
        (assign62460_e80928, (((((((var_fac_exc * var_i_ds_dn0) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn0)) * var_gfac) + (assign62460_e80924 * var_gfac_dn0)) * var_gfac) + (assign62460_e80926 * var_gfac_dn0)), (((((((var_fac_exc * var_i_ds_dn1) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn1)) * var_gfac) + (assign62460_e80924 * var_gfac_dn1)) * var_gfac) + (assign62460_e80926 * var_gfac_dn1)), (((((((var_fac_exc * var_i_ds_dn2) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn2)) * var_gfac) + (assign62460_e80924 * var_gfac_dn2)) * var_gfac) + (assign62460_e80926 * var_gfac_dn2)), (((((((var_fac_exc * var_i_ds_dn3) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn3)) * var_gfac) + (assign62460_e80924 * var_gfac_dn3)) * var_gfac) + (assign62460_e80926 * var_gfac_dn3)), (((((((var_fac_exc * var_i_ds_dn4) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn4)) * var_gfac) + (assign62460_e80924 * var_gfac_dn4)) * var_gfac) + (assign62460_e80926 * var_gfac_dn4)), (((((((var_fac_exc * var_i_ds_dn5) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn5)) * var_gfac) + (assign62460_e80924 * var_gfac_dn5)) * var_gfac) + (assign62460_e80926 * var_gfac_dn5)), (((((((var_fac_exc * var_i_ds_dn6) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn6)) * var_gfac) + (assign62460_e80924 * var_gfac_dn6)) * var_gfac) + (assign62460_e80926 * var_gfac_dn6)), (((((((var_fac_exc * var_i_ds_dn7) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn7)) * var_gfac) + (assign62460_e80924 * var_gfac_dn7)) * var_gfac) + (assign62460_e80926 * var_gfac_dn7)), (((((((var_fac_exc * var_i_ds_dn8) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn8)) * var_gfac) + (assign62460_e80924 * var_gfac_dn8)) * var_gfac) + (assign62460_e80926 * var_gfac_dn8)), (((((((var_fac_exc * var_i_ds_dn9) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn9)) * var_gfac) + (assign62460_e80924 * var_gfac_dn9)) * var_gfac) + (assign62460_e80926 * var_gfac_dn9)), (((((((var_fac_exc * var_i_ds_dn10) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn10)) * var_gfac) + (assign62460_e80924 * var_gfac_dn10)) * var_gfac) + (assign62460_e80926 * var_gfac_dn10)), (((((((var_fac_exc * var_i_ds_dn11) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn11)) * var_gfac) + (assign62460_e80924 * var_gfac_dn11)) * var_gfac) + (assign62460_e80926 * var_gfac_dn11)), (((((((var_fac_exc * var_i_ds_db0) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_db0)) * var_gfac) + (assign62460_e80924 * var_gfac_db0)) * var_gfac) + (assign62460_e80926 * var_gfac_db0)), (((((((var_fac_exc * var_i_ds_db1) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_db1)) * var_gfac) + (assign62460_e80924 * var_gfac_db1)) * var_gfac) + (assign62460_e80926 * var_gfac_db1)), (((((((var_fac_exc * var_i_ds_db2) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_db2)) * var_gfac) + (assign62460_e80924 * var_gfac_db2)) * var_gfac) + (assign62460_e80926 * var_gfac_db2)), (((((((var_fac_exc * var_i_ds_db3) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_db3)) * var_gfac) + (assign62460_e80924 * var_gfac_db3)) * var_gfac) + (assign62460_e80926 * var_gfac_db3)), (((((((var_fac_exc * var_i_ds_db4) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_db4)) * var_gfac) + (assign62460_e80924 * var_gfac_db4)) * var_gfac) + (assign62460_e80926 * var_gfac_db4)), (((((((var_fac_exc * var_i_ds_db5) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_db5)) * var_gfac) + (assign62460_e80924 * var_gfac_db5)) * var_gfac) + (assign62460_e80926 * var_gfac_db5)), (((((((var_fac_exc * var_i_ds_db6) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_db6)) * var_gfac) + (assign62460_e80924 * var_gfac_db6)) * var_gfac) + (assign62460_e80926 * var_gfac_db6)),)
    } else {
        (var_sidexc, var_sidexc_dn0, var_sidexc_dn1, var_sidexc_dn2, var_sidexc_dn3, var_sidexc_dn4, var_sidexc_dn5, var_sidexc_dn6, var_sidexc_dn7, var_sidexc_dn8, var_sidexc_dn9, var_sidexc_dn10, var_sidexc_dn11, var_sidexc_db0, var_sidexc_db1, var_sidexc_db2, var_sidexc_db3, var_sidexc_db4, var_sidexc_db5, var_sidexc_db6,)
    }
};
        var_sidexc = assign62460_e80930;
        var_sidexc_dn0 = assign62460_e80930_d_n0;
        var_sidexc_dn1 = assign62460_e80930_d_n1;
        var_sidexc_dn2 = assign62460_e80930_d_n2;
        var_sidexc_dn3 = assign62460_e80930_d_n3;
        var_sidexc_dn4 = assign62460_e80930_d_n4;
        var_sidexc_dn5 = assign62460_e80930_d_n5;
        var_sidexc_dn6 = assign62460_e80930_d_n6;
        var_sidexc_dn7 = assign62460_e80930_d_n7;
        var_sidexc_dn8 = assign62460_e80930_d_n8;
        var_sidexc_dn9 = assign62460_e80930_d_n9;
        var_sidexc_dn10 = assign62460_e80930_d_n10;
        var_sidexc_dn11 = assign62460_e80930_d_n11;
        var_sidexc_db0 = assign62460_e80930_d_b0;
        var_sidexc_db1 = assign62460_e80930_d_b1;
        var_sidexc_db2 = assign62460_e80930_d_b2;
        var_sidexc_db3 = assign62460_e80930_d_b3;
        var_sidexc_db4 = assign62460_e80930_d_b4;
        var_sidexc_db5 = assign62460_e80930_d_b5;
        var_sidexc_db6 = assign62460_e80930_d_b6;

        let (assign62470_e80942, assign62470_e80942_d_n0, assign62470_e80942_d_n1, assign62470_e80942_d_n2, assign62470_e80942_d_n3, assign62470_e80942_d_n4, assign62470_e80942_d_n5, assign62470_e80942_d_n6, assign62470_e80942_d_n7, assign62470_e80942_d_n8, assign62470_e80942_d_n9, assign62470_e80942_d_n10, assign62470_e80942_d_n11, assign62470_e80942_d_b0, assign62470_e80942_d_b1, assign62470_e80942_d_b2, assign62470_e80942_d_b3, assign62470_e80942_d_b4, assign62470_e80942_d_b5, assign62470_e80942_d_b6,) = {
    if (((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) && (var_guard1763 != 0.0)) {
        let assign62470_e80939: f64 = (var_sidexc / var_nt0);
        let assign62470_e80940: f64 = (var_mid + assign62470_e80939);
        (assign62470_e80940, (var_mid_dn0 + (var_sidexc_dn0 / var_nt0)), (var_mid_dn1 + (var_sidexc_dn1 / var_nt0)), (var_mid_dn2 + (var_sidexc_dn2 / var_nt0)), (var_mid_dn3 + (var_sidexc_dn3 / var_nt0)), (var_mid_dn4 + (var_sidexc_dn4 / var_nt0)), (var_mid_dn5 + (var_sidexc_dn5 / var_nt0)), (var_mid_dn6 + (var_sidexc_dn6 / var_nt0)), (var_mid_dn7 + (var_sidexc_dn7 / var_nt0)), (var_mid_dn8 + (var_sidexc_dn8 / var_nt0)), (var_mid_dn9 + (var_sidexc_dn9 / var_nt0)), (var_mid_dn10 + (var_sidexc_dn10 / var_nt0)), (var_mid_dn11 + (var_sidexc_dn11 / var_nt0)), (var_mid_db0 + (var_sidexc_db0 / var_nt0)), (var_mid_db1 + (var_sidexc_db1 / var_nt0)), (var_mid_db2 + (var_sidexc_db2 / var_nt0)), (var_mid_db3 + (var_sidexc_db3 / var_nt0)), (var_mid_db4 + (var_sidexc_db4 / var_nt0)), (var_mid_db5 + (var_sidexc_db5 / var_nt0)), (var_mid_db6 + (var_sidexc_db6 / var_nt0)),)
    } else {
        (var_mid, var_mid_dn0, var_mid_dn1, var_mid_dn2, var_mid_dn3, var_mid_dn4, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn9, var_mid_dn10, var_mid_dn11, var_mid_db0, var_mid_db1, var_mid_db2, var_mid_db3, var_mid_db4, var_mid_db5, var_mid_db6,)
    }
};
        var_mid = assign62470_e80942;
        var_mid_dn0 = assign62470_e80942_d_n0;
        var_mid_dn1 = assign62470_e80942_d_n1;
        var_mid_dn2 = assign62470_e80942_d_n2;
        var_mid_dn3 = assign62470_e80942_d_n3;
        var_mid_dn4 = assign62470_e80942_d_n4;
        var_mid_dn5 = assign62470_e80942_d_n5;
        var_mid_dn6 = assign62470_e80942_d_n6;
        var_mid_dn7 = assign62470_e80942_d_n7;
        var_mid_dn8 = assign62470_e80942_d_n8;
        var_mid_dn9 = assign62470_e80942_d_n9;
        var_mid_dn10 = assign62470_e80942_d_n10;
        var_mid_dn11 = assign62470_e80942_d_n11;
        var_mid_db0 = assign62470_e80942_d_b0;
        var_mid_db1 = assign62470_e80942_d_b1;
        var_mid_db2 = assign62470_e80942_d_b2;
        var_mid_db3 = assign62470_e80942_d_b3;
        var_mid_db4 = assign62470_e80942_d_b4;
        var_mid_db5 = assign62470_e80942_d_b5;
        var_mid_db6 = assign62470_e80942_d_b6;

        let (assign62480_e80951, assign62480_e80951_d_n0, assign62480_e80951_d_n1, assign62480_e80951_d_n2, assign62480_e80951_d_n3, assign62480_e80951_d_n4, assign62480_e80951_d_n5, assign62480_e80951_d_n6, assign62480_e80951_d_n7, assign62480_e80951_d_n8, assign62480_e80951_d_n9, assign62480_e80951_d_n10, assign62480_e80951_d_n11, assign62480_e80951_d_b0, assign62480_e80951_d_b1, assign62480_e80951_d_b2, assign62480_e80951_d_b3, assign62480_e80951_d_b4, assign62480_e80951_d_b5, assign62480_e80951_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62480_e80948: f64 = (var_nt * var_mid);
        let assign62480_e80949: f64 = (assign62480_e80948).sqrt();
        (assign62480_e80949, ((var_nt * var_mid_dn0) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_dn1) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_dn2) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_dn3) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_dn4) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_dn5) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_dn6) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_dn7) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_dn8) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_dn9) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_dn10) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_dn11) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_db0) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_db1) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_db2) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_db3) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_db4) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_db5) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_db6) / (2.0 * assign62480_e80949)),)
    } else {
        (var_sqid, var_sqid_dn0, var_sqid_dn1, var_sqid_dn2, var_sqid_dn3, var_sqid_dn4, var_sqid_dn5, var_sqid_dn6, var_sqid_dn7, var_sqid_dn8, var_sqid_dn9, var_sqid_dn10, var_sqid_dn11, var_sqid_db0, var_sqid_db1, var_sqid_db2, var_sqid_db3, var_sqid_db4, var_sqid_db5, var_sqid_db6,)
    }
};
        var_sqid = assign62480_e80951;
        var_sqid_dn0 = assign62480_e80951_d_n0;
        var_sqid_dn1 = assign62480_e80951_d_n1;
        var_sqid_dn2 = assign62480_e80951_d_n2;
        var_sqid_dn3 = assign62480_e80951_d_n3;
        var_sqid_dn4 = assign62480_e80951_d_n4;
        var_sqid_dn5 = assign62480_e80951_d_n5;
        var_sqid_dn6 = assign62480_e80951_d_n6;
        var_sqid_dn7 = assign62480_e80951_d_n7;
        var_sqid_dn8 = assign62480_e80951_d_n8;
        var_sqid_dn9 = assign62480_e80951_d_n9;
        var_sqid_dn10 = assign62480_e80951_d_n10;
        var_sqid_dn11 = assign62480_e80951_d_n11;
        var_sqid_db0 = assign62480_e80951_d_b0;
        var_sqid_db1 = assign62480_e80951_d_b1;
        var_sqid_db2 = assign62480_e80951_d_b2;
        var_sqid_db3 = assign62480_e80951_d_b3;
        var_sqid_db4 = assign62480_e80951_d_b4;
        var_sqid_db5 = assign62480_e80951_d_b5;
        var_sqid_db6 = assign62480_e80951_d_b6;

        let assign62490_e80966: f64 = if ((((p.p50 == 1.0) && (var_nt > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        var_guard1765 = assign62490_e80966;

        let (assign62500_e80998, assign62500_e80998_d_n0, assign62500_e80998_d_n1, assign62500_e80998_d_n2, assign62500_e80998_d_n3, assign62500_e80998_d_n4, assign62500_e80998_d_n5, assign62500_e80998_d_n6, assign62500_e80998_d_n7, assign62500_e80998_d_n8, assign62500_e80998_d_n9, assign62500_e80998_d_n10, assign62500_e80998_d_n11, assign62500_e80998_d_b0, assign62500_e80998_d_b1, assign62500_e80998_d_b2, assign62500_e80998_d_b3, assign62500_e80998_d_b4, assign62500_e80998_d_b5, assign62500_e80998_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let assign62500_e80972: f64 = (var_t1 / 12.0);
        let assign62500_e80976: f64 = (var_t1 + 0.2);
        let assign62500_e80979: f64 = (12.0 * var_t2);
        let assign62500_e80980: f64 = (assign62500_e80976 - assign62500_e80979);
        let assign62500_e80981: f64 = (var_t2 * assign62500_e80980);
        let assign62500_e80982: f64 = (assign62500_e80972 - assign62500_e80981);
        let assign62500_e80987: f64 = (var_t1 + 1.0);
        let assign62500_e80990: f64 = (12.0 * var_t2);
        let assign62500_e80991: f64 = (assign62500_e80987 - assign62500_e80990);
        let assign62500_e80992: f64 = (var_t2 * assign62500_e80991);
        let assign62500_e80994: f64 = (assign62500_e80992 * var_r);
        let assign62500_e80995: f64 = (1.6 * assign62500_e80994);
        let assign62500_e80996: f64 = (assign62500_e80982 - assign62500_e80995);
        (assign62500_e80996, (((var_t1_dn0 / 12.0) - ((var_t2_dn0 * assign62500_e80980) + (var_t2 * (var_t1_dn0 - (12.0 * var_t2_dn0))))) - (1.6 * ((((var_t2_dn0 * assign62500_e80991) + (var_t2 * (var_t1_dn0 - (12.0 * var_t2_dn0)))) * var_r) + (assign62500_e80992 * var_r_dn0)))), (((var_t1_dn1 / 12.0) - ((var_t2_dn1 * assign62500_e80980) + (var_t2 * (var_t1_dn1 - (12.0 * var_t2_dn1))))) - (1.6 * ((((var_t2_dn1 * assign62500_e80991) + (var_t2 * (var_t1_dn1 - (12.0 * var_t2_dn1)))) * var_r) + (assign62500_e80992 * var_r_dn1)))), (((var_t1_dn2 / 12.0) - ((var_t2_dn2 * assign62500_e80980) + (var_t2 * (var_t1_dn2 - (12.0 * var_t2_dn2))))) - (1.6 * ((((var_t2_dn2 * assign62500_e80991) + (var_t2 * (var_t1_dn2 - (12.0 * var_t2_dn2)))) * var_r) + (assign62500_e80992 * var_r_dn2)))), (((var_t1_dn3 / 12.0) - ((var_t2_dn3 * assign62500_e80980) + (var_t2 * (var_t1_dn3 - (12.0 * var_t2_dn3))))) - (1.6 * ((((var_t2_dn3 * assign62500_e80991) + (var_t2 * (var_t1_dn3 - (12.0 * var_t2_dn3)))) * var_r) + (assign62500_e80992 * var_r_dn3)))), (((var_t1_dn4 / 12.0) - ((var_t2_dn4 * assign62500_e80980) + (var_t2 * (var_t1_dn4 - (12.0 * var_t2_dn4))))) - (1.6 * ((((var_t2_dn4 * assign62500_e80991) + (var_t2 * (var_t1_dn4 - (12.0 * var_t2_dn4)))) * var_r) + (assign62500_e80992 * var_r_dn4)))), (((var_t1_dn5 / 12.0) - ((var_t2_dn5 * assign62500_e80980) + (var_t2 * (var_t1_dn5 - (12.0 * var_t2_dn5))))) - (1.6 * ((((var_t2_dn5 * assign62500_e80991) + (var_t2 * (var_t1_dn5 - (12.0 * var_t2_dn5)))) * var_r) + (assign62500_e80992 * var_r_dn5)))), (((var_t1_dn6 / 12.0) - ((var_t2_dn6 * assign62500_e80980) + (var_t2 * (var_t1_dn6 - (12.0 * var_t2_dn6))))) - (1.6 * ((((var_t2_dn6 * assign62500_e80991) + (var_t2 * (var_t1_dn6 - (12.0 * var_t2_dn6)))) * var_r) + (assign62500_e80992 * var_r_dn6)))), (((var_t1_dn7 / 12.0) - ((var_t2_dn7 * assign62500_e80980) + (var_t2 * (var_t1_dn7 - (12.0 * var_t2_dn7))))) - (1.6 * ((((var_t2_dn7 * assign62500_e80991) + (var_t2 * (var_t1_dn7 - (12.0 * var_t2_dn7)))) * var_r) + (assign62500_e80992 * var_r_dn7)))), (((var_t1_dn8 / 12.0) - ((var_t2_dn8 * assign62500_e80980) + (var_t2 * (var_t1_dn8 - (12.0 * var_t2_dn8))))) - (1.6 * ((((var_t2_dn8 * assign62500_e80991) + (var_t2 * (var_t1_dn8 - (12.0 * var_t2_dn8)))) * var_r) + (assign62500_e80992 * var_r_dn8)))), (((var_t1_dn9 / 12.0) - ((var_t2_dn9 * assign62500_e80980) + (var_t2 * (var_t1_dn9 - (12.0 * var_t2_dn9))))) - (1.6 * ((((var_t2_dn9 * assign62500_e80991) + (var_t2 * (var_t1_dn9 - (12.0 * var_t2_dn9)))) * var_r) + (assign62500_e80992 * var_r_dn9)))), (((var_t1_dn10 / 12.0) - ((var_t2_dn10 * assign62500_e80980) + (var_t2 * (var_t1_dn10 - (12.0 * var_t2_dn10))))) - (1.6 * ((((var_t2_dn10 * assign62500_e80991) + (var_t2 * (var_t1_dn10 - (12.0 * var_t2_dn10)))) * var_r) + (assign62500_e80992 * var_r_dn10)))), (((var_t1_dn11 / 12.0) - ((var_t2_dn11 * assign62500_e80980) + (var_t2 * (var_t1_dn11 - (12.0 * var_t2_dn11))))) - (1.6 * ((((var_t2_dn11 * assign62500_e80991) + (var_t2 * (var_t1_dn11 - (12.0 * var_t2_dn11)))) * var_r) + (assign62500_e80992 * var_r_dn11)))), (((var_t1_db0 / 12.0) - ((var_t2_db0 * assign62500_e80980) + (var_t2 * (var_t1_db0 - (12.0 * var_t2_db0))))) - (1.6 * ((((var_t2_db0 * assign62500_e80991) + (var_t2 * (var_t1_db0 - (12.0 * var_t2_db0)))) * var_r) + (assign62500_e80992 * var_r_db0)))), (((var_t1_db1 / 12.0) - ((var_t2_db1 * assign62500_e80980) + (var_t2 * (var_t1_db1 - (12.0 * var_t2_db1))))) - (1.6 * ((((var_t2_db1 * assign62500_e80991) + (var_t2 * (var_t1_db1 - (12.0 * var_t2_db1)))) * var_r) + (assign62500_e80992 * var_r_db1)))), (((var_t1_db2 / 12.0) - ((var_t2_db2 * assign62500_e80980) + (var_t2 * (var_t1_db2 - (12.0 * var_t2_db2))))) - (1.6 * ((((var_t2_db2 * assign62500_e80991) + (var_t2 * (var_t1_db2 - (12.0 * var_t2_db2)))) * var_r) + (assign62500_e80992 * var_r_db2)))), (((var_t1_db3 / 12.0) - ((var_t2_db3 * assign62500_e80980) + (var_t2 * (var_t1_db3 - (12.0 * var_t2_db3))))) - (1.6 * ((((var_t2_db3 * assign62500_e80991) + (var_t2 * (var_t1_db3 - (12.0 * var_t2_db3)))) * var_r) + (assign62500_e80992 * var_r_db3)))), (((var_t1_db4 / 12.0) - ((var_t2_db4 * assign62500_e80980) + (var_t2 * (var_t1_db4 - (12.0 * var_t2_db4))))) - (1.6 * ((((var_t2_db4 * assign62500_e80991) + (var_t2 * (var_t1_db4 - (12.0 * var_t2_db4)))) * var_r) + (assign62500_e80992 * var_r_db4)))), (((var_t1_db5 / 12.0) - ((var_t2_db5 * assign62500_e80980) + (var_t2 * (var_t1_db5 - (12.0 * var_t2_db5))))) - (1.6 * ((((var_t2_db5 * assign62500_e80991) + (var_t2 * (var_t1_db5 - (12.0 * var_t2_db5)))) * var_r) + (assign62500_e80992 * var_r_db5)))), (((var_t1_db6 / 12.0) - ((var_t2_db6 * assign62500_e80980) + (var_t2 * (var_t1_db6 - (12.0 * var_t2_db6))))) - (1.6 * ((((var_t2_db6 * assign62500_e80991) + (var_t2 * (var_t1_db6 - (12.0 * var_t2_db6)))) * var_r) + (assign62500_e80992 * var_r_db6)))),)
    } else {
        (var_mig, var_mig_dn0, var_mig_dn1, var_mig_dn2, var_mig_dn3, var_mig_dn4, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn9, var_mig_dn10, var_mig_dn11, var_mig_db0, var_mig_db1, var_mig_db2, var_mig_db3, var_mig_db4, var_mig_db5, var_mig_db6,)
    }
};
        var_mig = assign62500_e80998;
        var_mig_dn0 = assign62500_e80998_d_n0;
        var_mig_dn1 = assign62500_e80998_d_n1;
        var_mig_dn2 = assign62500_e80998_d_n2;
        var_mig_dn3 = assign62500_e80998_d_n3;
        var_mig_dn4 = assign62500_e80998_d_n4;
        var_mig_dn5 = assign62500_e80998_d_n5;
        var_mig_dn6 = assign62500_e80998_d_n6;
        var_mig_dn7 = assign62500_e80998_d_n7;
        var_mig_dn8 = assign62500_e80998_d_n8;
        var_mig_dn9 = assign62500_e80998_d_n9;
        var_mig_dn10 = assign62500_e80998_d_n10;
        var_mig_dn11 = assign62500_e80998_d_n11;
        var_mig_db0 = assign62500_e80998_d_b0;
        var_mig_db1 = assign62500_e80998_d_b1;
        var_mig_db2 = assign62500_e80998_d_b2;
        var_mig_db3 = assign62500_e80998_d_b3;
        var_mig_db4 = assign62500_e80998_d_b4;
        var_mig_db5 = assign62500_e80998_d_b5;
        var_mig_db6 = assign62500_e80998_d_b6;

        *var_g_ideal_slot = var_g_ideal;
        *var_g_ideal_db0_slot = var_g_ideal_db0;
        *var_g_ideal_db1_slot = var_g_ideal_db1;
        *var_g_ideal_db2_slot = var_g_ideal_db2;
        *var_g_ideal_db3_slot = var_g_ideal_db3;
        *var_g_ideal_db4_slot = var_g_ideal_db4;
        *var_g_ideal_db5_slot = var_g_ideal_db5;
        *var_g_ideal_db6_slot = var_g_ideal_db6;
        *var_g_ideal_dn0_slot = var_g_ideal_dn0;
        *var_g_ideal_dn1_slot = var_g_ideal_dn1;
        *var_g_ideal_dn10_slot = var_g_ideal_dn10;
        *var_g_ideal_dn11_slot = var_g_ideal_dn11;
        *var_g_ideal_dn2_slot = var_g_ideal_dn2;
        *var_g_ideal_dn3_slot = var_g_ideal_dn3;
        *var_g_ideal_dn4_slot = var_g_ideal_dn4;
        *var_g_ideal_dn5_slot = var_g_ideal_dn5;
        *var_g_ideal_dn6_slot = var_g_ideal_dn6;
        *var_g_ideal_dn7_slot = var_g_ideal_dn7;
        *var_g_ideal_dn8_slot = var_g_ideal_dn8;
        *var_g_ideal_dn9_slot = var_g_ideal_dn9;
        *var_gfac_slot = var_gfac;
        *var_gfac_db0_slot = var_gfac_db0;
        *var_gfac_db1_slot = var_gfac_db1;
        *var_gfac_db2_slot = var_gfac_db2;
        *var_gfac_db3_slot = var_gfac_db3;
        *var_gfac_db4_slot = var_gfac_db4;
        *var_gfac_db5_slot = var_gfac_db5;
        *var_gfac_db6_slot = var_gfac_db6;
        *var_gfac_dn0_slot = var_gfac_dn0;
        *var_gfac_dn1_slot = var_gfac_dn1;
        *var_gfac_dn10_slot = var_gfac_dn10;
        *var_gfac_dn11_slot = var_gfac_dn11;
        *var_gfac_dn2_slot = var_gfac_dn2;
        *var_gfac_dn3_slot = var_gfac_dn3;
        *var_gfac_dn4_slot = var_gfac_dn4;
        *var_gfac_dn5_slot = var_gfac_dn5;
        *var_gfac_dn6_slot = var_gfac_dn6;
        *var_gfac_dn7_slot = var_gfac_dn7;
        *var_gfac_dn8_slot = var_gfac_dn8;
        *var_gfac_dn9_slot = var_gfac_dn9;
        *var_guard1763_slot = var_guard1763;
        *var_guard1764_slot = var_guard1764;
        *var_guard1765_slot = var_guard1765;
        *var_gvsat_exc_slot = var_gvsat_exc;
        *var_gvsat_exc_db0_slot = var_gvsat_exc_db0;
        *var_gvsat_exc_db1_slot = var_gvsat_exc_db1;
        *var_gvsat_exc_db2_slot = var_gvsat_exc_db2;
        *var_gvsat_exc_db3_slot = var_gvsat_exc_db3;
        *var_gvsat_exc_db4_slot = var_gvsat_exc_db4;
        *var_gvsat_exc_db5_slot = var_gvsat_exc_db5;
        *var_gvsat_exc_db6_slot = var_gvsat_exc_db6;
        *var_gvsat_exc_dn0_slot = var_gvsat_exc_dn0;
        *var_gvsat_exc_dn1_slot = var_gvsat_exc_dn1;
        *var_gvsat_exc_dn10_slot = var_gvsat_exc_dn10;
        *var_gvsat_exc_dn11_slot = var_gvsat_exc_dn11;
        *var_gvsat_exc_dn2_slot = var_gvsat_exc_dn2;
        *var_gvsat_exc_dn3_slot = var_gvsat_exc_dn3;
        *var_gvsat_exc_dn4_slot = var_gvsat_exc_dn4;
        *var_gvsat_exc_dn5_slot = var_gvsat_exc_dn5;
        *var_gvsat_exc_dn6_slot = var_gvsat_exc_dn6;
        *var_gvsat_exc_dn7_slot = var_gvsat_exc_dn7;
        *var_gvsat_exc_dn8_slot = var_gvsat_exc_dn8;
        *var_gvsat_exc_dn9_slot = var_gvsat_exc_dn9;
        *var_lc_slot = var_lc;
        *var_lc_db0_slot = var_lc_db0;
        *var_lc_db1_slot = var_lc_db1;
        *var_lc_db2_slot = var_lc_db2;
        *var_lc_db3_slot = var_lc_db3;
        *var_lc_db4_slot = var_lc_db4;
        *var_lc_db5_slot = var_lc_db5;
        *var_lc_db6_slot = var_lc_db6;
        *var_lc_dn0_slot = var_lc_dn0;
        *var_lc_dn1_slot = var_lc_dn1;
        *var_lc_dn10_slot = var_lc_dn10;
        *var_lc_dn11_slot = var_lc_dn11;
        *var_lc_dn2_slot = var_lc_dn2;
        *var_lc_dn3_slot = var_lc_dn3;
        *var_lc_dn4_slot = var_lc_dn4;
        *var_lc_dn5_slot = var_lc_dn5;
        *var_lc_dn6_slot = var_lc_dn6;
        *var_lc_dn7_slot = var_lc_dn7;
        *var_lc_dn8_slot = var_lc_dn8;
        *var_lc_dn9_slot = var_lc_dn9;
        *var_lcinv2_slot = var_lcinv2;
        *var_lcinv2_db0_slot = var_lcinv2_db0;
        *var_lcinv2_db1_slot = var_lcinv2_db1;
        *var_lcinv2_db2_slot = var_lcinv2_db2;
        *var_lcinv2_db3_slot = var_lcinv2_db3;
        *var_lcinv2_db4_slot = var_lcinv2_db4;
        *var_lcinv2_db5_slot = var_lcinv2_db5;
        *var_lcinv2_db6_slot = var_lcinv2_db6;
        *var_lcinv2_dn0_slot = var_lcinv2_dn0;
        *var_lcinv2_dn1_slot = var_lcinv2_dn1;
        *var_lcinv2_dn10_slot = var_lcinv2_dn10;
        *var_lcinv2_dn11_slot = var_lcinv2_dn11;
        *var_lcinv2_dn2_slot = var_lcinv2_dn2;
        *var_lcinv2_dn3_slot = var_lcinv2_dn3;
        *var_lcinv2_dn4_slot = var_lcinv2_dn4;
        *var_lcinv2_dn5_slot = var_lcinv2_dn5;
        *var_lcinv2_dn6_slot = var_lcinv2_dn6;
        *var_lcinv2_dn7_slot = var_lcinv2_dn7;
        *var_lcinv2_dn8_slot = var_lcinv2_dn8;
        *var_lcinv2_dn9_slot = var_lcinv2_dn9;
        *var_mid_slot = var_mid;
        *var_mid_db0_slot = var_mid_db0;
        *var_mid_db1_slot = var_mid_db1;
        *var_mid_db2_slot = var_mid_db2;
        *var_mid_db3_slot = var_mid_db3;
        *var_mid_db4_slot = var_mid_db4;
        *var_mid_db5_slot = var_mid_db5;
        *var_mid_db6_slot = var_mid_db6;
        *var_mid_dn0_slot = var_mid_dn0;
        *var_mid_dn1_slot = var_mid_dn1;
        *var_mid_dn10_slot = var_mid_dn10;
        *var_mid_dn11_slot = var_mid_dn11;
        *var_mid_dn2_slot = var_mid_dn2;
        *var_mid_dn3_slot = var_mid_dn3;
        *var_mid_dn4_slot = var_mid_dn4;
        *var_mid_dn5_slot = var_mid_dn5;
        *var_mid_dn6_slot = var_mid_dn6;
        *var_mid_dn7_slot = var_mid_dn7;
        *var_mid_dn8_slot = var_mid_dn8;
        *var_mid_dn9_slot = var_mid_dn9;
        *var_mig_slot = var_mig;
        *var_mig_db0_slot = var_mig_db0;
        *var_mig_db1_slot = var_mig_db1;
        *var_mig_db2_slot = var_mig_db2;
        *var_mig_db3_slot = var_mig_db3;
        *var_mig_db4_slot = var_mig_db4;
        *var_mig_db5_slot = var_mig_db5;
        *var_mig_db6_slot = var_mig_db6;
        *var_mig_dn0_slot = var_mig_dn0;
        *var_mig_dn1_slot = var_mig_dn1;
        *var_mig_dn10_slot = var_mig_dn10;
        *var_mig_dn11_slot = var_mig_dn11;
        *var_mig_dn2_slot = var_mig_dn2;
        *var_mig_dn3_slot = var_mig_dn3;
        *var_mig_dn4_slot = var_mig_dn4;
        *var_mig_dn5_slot = var_mig_dn5;
        *var_mig_dn6_slot = var_mig_dn6;
        *var_mig_dn7_slot = var_mig_dn7;
        *var_mig_dn8_slot = var_mig_dn8;
        *var_mig_dn9_slot = var_mig_dn9;
        *var_sidexc_slot = var_sidexc;
        *var_sidexc_db0_slot = var_sidexc_db0;
        *var_sidexc_db1_slot = var_sidexc_db1;
        *var_sidexc_db2_slot = var_sidexc_db2;
        *var_sidexc_db3_slot = var_sidexc_db3;
        *var_sidexc_db4_slot = var_sidexc_db4;
        *var_sidexc_db5_slot = var_sidexc_db5;
        *var_sidexc_db6_slot = var_sidexc_db6;
        *var_sidexc_dn0_slot = var_sidexc_dn0;
        *var_sidexc_dn1_slot = var_sidexc_dn1;
        *var_sidexc_dn10_slot = var_sidexc_dn10;
        *var_sidexc_dn11_slot = var_sidexc_dn11;
        *var_sidexc_dn2_slot = var_sidexc_dn2;
        *var_sidexc_dn3_slot = var_sidexc_dn3;
        *var_sidexc_dn4_slot = var_sidexc_dn4;
        *var_sidexc_dn5_slot = var_sidexc_dn5;
        *var_sidexc_dn6_slot = var_sidexc_dn6;
        *var_sidexc_dn7_slot = var_sidexc_dn7;
        *var_sidexc_dn8_slot = var_sidexc_dn8;
        *var_sidexc_dn9_slot = var_sidexc_dn9;
        *var_sqid_slot = var_sqid;
        *var_sqid_db0_slot = var_sqid_db0;
        *var_sqid_db1_slot = var_sqid_db1;
        *var_sqid_db2_slot = var_sqid_db2;
        *var_sqid_db3_slot = var_sqid_db3;
        *var_sqid_db4_slot = var_sqid_db4;
        *var_sqid_db5_slot = var_sqid_db5;
        *var_sqid_db6_slot = var_sqid_db6;
        *var_sqid_dn0_slot = var_sqid_dn0;
        *var_sqid_dn1_slot = var_sqid_dn1;
        *var_sqid_dn10_slot = var_sqid_dn10;
        *var_sqid_dn11_slot = var_sqid_dn11;
        *var_sqid_dn2_slot = var_sqid_dn2;
        *var_sqid_dn3_slot = var_sqid_dn3;
        *var_sqid_dn4_slot = var_sqid_dn4;
        *var_sqid_dn5_slot = var_sqid_dn5;
        *var_sqid_dn6_slot = var_sqid_dn6;
        *var_sqid_dn7_slot = var_sqid_dn7;
        *var_sqid_dn8_slot = var_sqid_dn8;
        *var_sqid_dn9_slot = var_sqid_dn9;
        *var_thesat1_exc_slot = var_thesat1_exc;
        *var_thesat1_exc_db0_slot = var_thesat1_exc_db0;
        *var_thesat1_exc_db1_slot = var_thesat1_exc_db1;
        *var_thesat1_exc_db2_slot = var_thesat1_exc_db2;
        *var_thesat1_exc_db3_slot = var_thesat1_exc_db3;
        *var_thesat1_exc_db4_slot = var_thesat1_exc_db4;
        *var_thesat1_exc_db5_slot = var_thesat1_exc_db5;
        *var_thesat1_exc_db6_slot = var_thesat1_exc_db6;
        *var_thesat1_exc_dn0_slot = var_thesat1_exc_dn0;
        *var_thesat1_exc_dn1_slot = var_thesat1_exc_dn1;
        *var_thesat1_exc_dn10_slot = var_thesat1_exc_dn10;
        *var_thesat1_exc_dn11_slot = var_thesat1_exc_dn11;
        *var_thesat1_exc_dn2_slot = var_thesat1_exc_dn2;
        *var_thesat1_exc_dn3_slot = var_thesat1_exc_dn3;
        *var_thesat1_exc_dn4_slot = var_thesat1_exc_dn4;
        *var_thesat1_exc_dn5_slot = var_thesat1_exc_dn5;
        *var_thesat1_exc_dn6_slot = var_thesat1_exc_dn6;
        *var_thesat1_exc_dn7_slot = var_thesat1_exc_dn7;
        *var_thesat1_exc_dn8_slot = var_thesat1_exc_dn8;
        *var_thesat1_exc_dn9_slot = var_thesat1_exc_dn9;
        *var_zsat_exc_slot = var_zsat_exc;
        *var_zsat_exc_db0_slot = var_zsat_exc_db0;
        *var_zsat_exc_db1_slot = var_zsat_exc_db1;
        *var_zsat_exc_db2_slot = var_zsat_exc_db2;
        *var_zsat_exc_db3_slot = var_zsat_exc_db3;
        *var_zsat_exc_db4_slot = var_zsat_exc_db4;
        *var_zsat_exc_db5_slot = var_zsat_exc_db5;
        *var_zsat_exc_db6_slot = var_zsat_exc_db6;
        *var_zsat_exc_dn0_slot = var_zsat_exc_dn0;
        *var_zsat_exc_dn1_slot = var_zsat_exc_dn1;
        *var_zsat_exc_dn10_slot = var_zsat_exc_dn10;
        *var_zsat_exc_dn11_slot = var_zsat_exc_dn11;
        *var_zsat_exc_dn2_slot = var_zsat_exc_dn2;
        *var_zsat_exc_dn3_slot = var_zsat_exc_dn3;
        *var_zsat_exc_dn4_slot = var_zsat_exc_dn4;
        *var_zsat_exc_dn5_slot = var_zsat_exc_dn5;
        *var_zsat_exc_dn6_slot = var_zsat_exc_dn6;
        *var_zsat_exc_dn7_slot = var_zsat_exc_dn7;
        *var_zsat_exc_dn8_slot = var_zsat_exc_dn8;
        *var_zsat_exc_dn9_slot = var_zsat_exc_dn9;
    }

    pub(super) fn stamp_transient_block_246(
        p: &Parameters,
        var_alpha_dc: f64,
        var_alpha_dc_db0: f64,
        var_alpha_dc_db1: f64,
        var_alpha_dc_db2: f64,
        var_alpha_dc_db3: f64,
        var_alpha_dc_db4: f64,
        var_alpha_dc_db5: f64,
        var_alpha_dc_db6: f64,
        var_alpha_dc_dn0: f64,
        var_alpha_dc_dn1: f64,
        var_alpha_dc_dn10: f64,
        var_alpha_dc_dn11: f64,
        var_alpha_dc_dn2: f64,
        var_alpha_dc_dn3: f64,
        var_alpha_dc_dn4: f64,
        var_alpha_dc_dn5: f64,
        var_alpha_dc_dn6: f64,
        var_alpha_dc_dn7: f64,
        var_alpha_dc_dn8: f64,
        var_alpha_dc_dn9: f64,
        var_betnedge_i: f64,
        var_cox_over_q: f64,
        var_cox_qm: f64,
        var_cox_qm_db0: f64,
        var_cox_qm_db1: f64,
        var_cox_qm_db2: f64,
        var_cox_qm_db3: f64,
        var_cox_qm_db4: f64,
        var_cox_qm_db5: f64,
        var_cox_qm_db6: f64,
        var_cox_qm_dn0: f64,
        var_cox_qm_dn1: f64,
        var_cox_qm_dn10: f64,
        var_cox_qm_dn11: f64,
        var_cox_qm_dn2: f64,
        var_cox_qm_dn3: f64,
        var_cox_qm_dn4: f64,
        var_cox_qm_dn5: f64,
        var_cox_qm_dn6: f64,
        var_cox_qm_dn7: f64,
        var_cox_qm_dn8: f64,
        var_cox_qm_dn9: f64,
        var_dsqredge: f64,
        var_dsqredge_db0: f64,
        var_dsqredge_db1: f64,
        var_dsqredge_db2: f64,
        var_dsqredge_db3: f64,
        var_dsqredge_db4: f64,
        var_dsqredge_db5: f64,
        var_dsqredge_db6: f64,
        var_dsqredge_dn0: f64,
        var_dsqredge_dn1: f64,
        var_dsqredge_dn10: f64,
        var_dsqredge_dn11: f64,
        var_dsqredge_dn2: f64,
        var_dsqredge_dn3: f64,
        var_dsqredge_dn4: f64,
        var_dsqredge_dn5: f64,
        var_dsqredge_dn6: f64,
        var_dsqredge_dn7: f64,
        var_dsqredge_dn8: f64,
        var_dsqredge_dn9: f64,
        var_eta_p_ac: f64,
        var_eta_p_ac_db0: f64,
        var_eta_p_ac_db1: f64,
        var_eta_p_ac_db2: f64,
        var_eta_p_ac_db3: f64,
        var_eta_p_ac_db4: f64,
        var_eta_p_ac_db5: f64,
        var_eta_p_ac_db6: f64,
        var_eta_p_ac_dn0: f64,
        var_eta_p_ac_dn1: f64,
        var_eta_p_ac_dn10: f64,
        var_eta_p_ac_dn11: f64,
        var_eta_p_ac_dn2: f64,
        var_eta_p_ac_dn3: f64,
        var_eta_p_ac_dn4: f64,
        var_eta_p_ac_dn5: f64,
        var_eta_p_ac_dn6: f64,
        var_eta_p_ac_dn7: f64,
        var_eta_p_ac_dn8: f64,
        var_eta_p_ac_dn9: f64,
        var_fntexc_i: f64,
        var_g_ideal: f64,
        var_g_ideal_db0: f64,
        var_g_ideal_db1: f64,
        var_g_ideal_db2: f64,
        var_g_ideal_db3: f64,
        var_g_ideal_db4: f64,
        var_g_ideal_db5: f64,
        var_g_ideal_db6: f64,
        var_g_ideal_dn0: f64,
        var_g_ideal_dn1: f64,
        var_g_ideal_dn10: f64,
        var_g_ideal_dn11: f64,
        var_g_ideal_dn2: f64,
        var_g_ideal_dn3: f64,
        var_g_ideal_dn4: f64,
        var_g_ideal_dn5: f64,
        var_g_ideal_dn6: f64,
        var_g_ideal_dn7: f64,
        var_g_ideal_dn8: f64,
        var_g_ideal_dn9: f64,
        var_gfedge2: f64,
        var_gmob_dl_ac: f64,
        var_gmob_dl_ac_db0: f64,
        var_gmob_dl_ac_db1: f64,
        var_gmob_dl_ac_db2: f64,
        var_gmob_dl_ac_db3: f64,
        var_gmob_dl_ac_db4: f64,
        var_gmob_dl_ac_db5: f64,
        var_gmob_dl_ac_db6: f64,
        var_gmob_dl_ac_dn0: f64,
        var_gmob_dl_ac_dn1: f64,
        var_gmob_dl_ac_dn10: f64,
        var_gmob_dl_ac_dn11: f64,
        var_gmob_dl_ac_dn2: f64,
        var_gmob_dl_ac_dn3: f64,
        var_gmob_dl_ac_dn4: f64,
        var_gmob_dl_ac_dn5: f64,
        var_gmob_dl_ac_dn6: f64,
        var_gmob_dl_ac_dn7: f64,
        var_gmob_dl_ac_dn8: f64,
        var_gmob_dl_ac_dn9: f64,
        var_guard1760: f64,
        var_guard1765: f64,
        var_gvsat_ac: f64,
        var_gvsat_ac_db0: f64,
        var_gvsat_ac_db1: f64,
        var_gvsat_ac_db2: f64,
        var_gvsat_ac_db3: f64,
        var_gvsat_ac_db4: f64,
        var_gvsat_ac_db5: f64,
        var_gvsat_ac_db6: f64,
        var_gvsat_ac_dn0: f64,
        var_gvsat_ac_dn1: f64,
        var_gvsat_ac_dn10: f64,
        var_gvsat_ac_dn11: f64,
        var_gvsat_ac_dn2: f64,
        var_gvsat_ac_dn3: f64,
        var_gvsat_ac_dn4: f64,
        var_gvsat_ac_dn5: f64,
        var_gvsat_ac_dn6: f64,
        var_gvsat_ac_dn7: f64,
        var_gvsat_ac_dn8: f64,
        var_gvsat_ac_dn9: f64,
        var_h_dc: f64,
        var_h_dc_db0: f64,
        var_h_dc_db1: f64,
        var_h_dc_db2: f64,
        var_h_dc_db3: f64,
        var_h_dc_db4: f64,
        var_h_dc_db5: f64,
        var_h_dc_db6: f64,
        var_h_dc_dn0: f64,
        var_h_dc_dn1: f64,
        var_h_dc_dn10: f64,
        var_h_dc_dn11: f64,
        var_h_dc_dn2: f64,
        var_h_dc_dn3: f64,
        var_h_dc_dn4: f64,
        var_h_dc_dn5: f64,
        var_h_dc_dn6: f64,
        var_h_dc_dn7: f64,
        var_h_dc_dn8: f64,
        var_h_dc_dn9: f64,
        var_lcinv2: f64,
        var_lcinv2_db0: f64,
        var_lcinv2_db1: f64,
        var_lcinv2_db2: f64,
        var_lcinv2_db3: f64,
        var_lcinv2_db4: f64,
        var_lcinv2_db5: f64,
        var_lcinv2_db6: f64,
        var_lcinv2_dn0: f64,
        var_lcinv2_dn1: f64,
        var_lcinv2_dn10: f64,
        var_lcinv2_dn11: f64,
        var_lcinv2_dn2: f64,
        var_lcinv2_dn3: f64,
        var_lcinv2_dn4: f64,
        var_lcinv2_dn5: f64,
        var_lcinv2_dn6: f64,
        var_lcinv2_dn7: f64,
        var_lcinv2_dn8: f64,
        var_lcinv2_dn9: f64,
        var_nt: f64,
        var_nt0: f64,
        var_phit: f64,
        var_r: f64,
        var_r_db0: f64,
        var_r_db1: f64,
        var_r_db2: f64,
        var_r_db3: f64,
        var_r_db4: f64,
        var_r_db5: f64,
        var_r_db6: f64,
        var_r_dn0: f64,
        var_r_dn1: f64,
        var_r_dn10: f64,
        var_r_dn11: f64,
        var_r_dn2: f64,
        var_r_dn3: f64,
        var_r_dn4: f64,
        var_r_dn5: f64,
        var_r_dn6: f64,
        var_r_dn7: f64,
        var_r_dn8: f64,
        var_r_dn9: f64,
        var_sidexc: f64,
        var_sidexc_db0: f64,
        var_sidexc_db1: f64,
        var_sidexc_db2: f64,
        var_sidexc_db3: f64,
        var_sidexc_db4: f64,
        var_sidexc_db5: f64,
        var_sidexc_db6: f64,
        var_sidexc_dn0: f64,
        var_sidexc_dn1: f64,
        var_sidexc_dn10: f64,
        var_sidexc_dn11: f64,
        var_sidexc_dn2: f64,
        var_sidexc_dn3: f64,
        var_sidexc_dn4: f64,
        var_sidexc_dn5: f64,
        var_sidexc_dn6: f64,
        var_sidexc_dn7: f64,
        var_sidexc_dn8: f64,
        var_sidexc_dn9: f64,
        var_sqid: f64,
        var_sqid_db0: f64,
        var_sqid_db1: f64,
        var_sqid_db2: f64,
        var_sqid_db3: f64,
        var_sqid_db4: f64,
        var_sqid_db5: f64,
        var_sqid_db6: f64,
        var_sqid_dn0: f64,
        var_sqid_dn1: f64,
        var_sqid_dn10: f64,
        var_sqid_dn11: f64,
        var_sqid_dn2: f64,
        var_sqid_dn3: f64,
        var_sqid_dn4: f64,
        var_sqid_dn5: f64,
        var_sqid_dn6: f64,
        var_sqid_dn7: f64,
        var_sqid_dn8: f64,
        var_sqid_dn9: f64,
        var_sqt2: f64,
        var_sqt2_db0: f64,
        var_sqt2_db1: f64,
        var_sqt2_db2: f64,
        var_sqt2_db3: f64,
        var_sqt2_db4: f64,
        var_sqt2_db5: f64,
        var_sqt2_db6: f64,
        var_sqt2_dn0: f64,
        var_sqt2_dn1: f64,
        var_sqt2_dn10: f64,
        var_sqt2_dn11: f64,
        var_sqt2_dn2: f64,
        var_sqt2_dn3: f64,
        var_sqt2_dn4: f64,
        var_sqt2_dn5: f64,
        var_sqt2_dn6: f64,
        var_sqt2_dn7: f64,
        var_sqt2_dn8: f64,
        var_sqt2_dn9: f64,
        var_t1: f64,
        var_t1_db0: f64,
        var_t1_db1: f64,
        var_t1_db2: f64,
        var_t1_db3: f64,
        var_t1_db4: f64,
        var_t1_db5: f64,
        var_t1_db6: f64,
        var_t1_dn0: f64,
        var_t1_dn1: f64,
        var_t1_dn10: f64,
        var_t1_dn11: f64,
        var_t1_dn2: f64,
        var_t1_dn3: f64,
        var_t1_dn4: f64,
        var_t1_dn5: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_t1_dn9: f64,
        var_t2: f64,
        var_t2_db0: f64,
        var_t2_db1: f64,
        var_t2_db2: f64,
        var_t2_db3: f64,
        var_t2_db4: f64,
        var_t2_db5: f64,
        var_t2_db6: f64,
        var_t2_dn0: f64,
        var_t2_dn1: f64,
        var_t2_dn10: f64,
        var_t2_dn11: f64,
        var_t2_dn2: f64,
        var_t2_dn3: f64,
        var_t2_dn4: f64,
        var_t2_dn5: f64,
        var_t2_dn6: f64,
        var_t2_dn7: f64,
        var_t2_dn8: f64,
        var_t2_dn9: f64,
        var_xgedge: f64,
        var_c_igid_slot: &mut f64,
        var_c_igid_db0_slot: &mut f64,
        var_c_igid_db1_slot: &mut f64,
        var_c_igid_db2_slot: &mut f64,
        var_c_igid_db3_slot: &mut f64,
        var_c_igid_db4_slot: &mut f64,
        var_c_igid_db5_slot: &mut f64,
        var_c_igid_db6_slot: &mut f64,
        var_c_igid_dn0_slot: &mut f64,
        var_c_igid_dn1_slot: &mut f64,
        var_c_igid_dn10_slot: &mut f64,
        var_c_igid_dn11_slot: &mut f64,
        var_c_igid_dn2_slot: &mut f64,
        var_c_igid_dn3_slot: &mut f64,
        var_c_igid_dn4_slot: &mut f64,
        var_c_igid_dn5_slot: &mut f64,
        var_c_igid_dn6_slot: &mut f64,
        var_c_igid_dn7_slot: &mut f64,
        var_c_igid_dn8_slot: &mut f64,
        var_c_igid_dn9_slot: &mut f64,
        var_cgeff_slot: &mut f64,
        var_cgeff_db0_slot: &mut f64,
        var_cgeff_db1_slot: &mut f64,
        var_cgeff_db2_slot: &mut f64,
        var_cgeff_db3_slot: &mut f64,
        var_cgeff_db4_slot: &mut f64,
        var_cgeff_db5_slot: &mut f64,
        var_cgeff_db6_slot: &mut f64,
        var_cgeff_dn0_slot: &mut f64,
        var_cgeff_dn1_slot: &mut f64,
        var_cgeff_dn10_slot: &mut f64,
        var_cgeff_dn11_slot: &mut f64,
        var_cgeff_dn2_slot: &mut f64,
        var_cgeff_dn3_slot: &mut f64,
        var_cgeff_dn4_slot: &mut f64,
        var_cgeff_dn5_slot: &mut f64,
        var_cgeff_dn6_slot: &mut f64,
        var_cgeff_dn7_slot: &mut f64,
        var_cgeff_dn8_slot: &mut f64,
        var_cgeff_dn9_slot: &mut f64,
        var_guard1766_slot: &mut f64,
        var_guard1767_slot: &mut f64,
        var_guard1769_slot: &mut f64,
        var_mig_slot: &mut f64,
        var_mig_db0_slot: &mut f64,
        var_mig_db1_slot: &mut f64,
        var_mig_db2_slot: &mut f64,
        var_mig_db3_slot: &mut f64,
        var_mig_db4_slot: &mut f64,
        var_mig_db5_slot: &mut f64,
        var_mig_db6_slot: &mut f64,
        var_mig_dn0_slot: &mut f64,
        var_mig_dn1_slot: &mut f64,
        var_mig_dn10_slot: &mut f64,
        var_mig_dn11_slot: &mut f64,
        var_mig_dn2_slot: &mut f64,
        var_mig_dn3_slot: &mut f64,
        var_mig_dn4_slot: &mut f64,
        var_mig_dn5_slot: &mut f64,
        var_mig_dn6_slot: &mut f64,
        var_mig_dn7_slot: &mut f64,
        var_mig_dn8_slot: &mut f64,
        var_mig_dn9_slot: &mut f64,
        var_migid_slot: &mut f64,
        var_migid0_slot: &mut f64,
        var_migid0_db0_slot: &mut f64,
        var_migid0_db1_slot: &mut f64,
        var_migid0_db2_slot: &mut f64,
        var_migid0_db3_slot: &mut f64,
        var_migid0_db4_slot: &mut f64,
        var_migid0_db5_slot: &mut f64,
        var_migid0_db6_slot: &mut f64,
        var_migid0_dn0_slot: &mut f64,
        var_migid0_dn1_slot: &mut f64,
        var_migid0_dn10_slot: &mut f64,
        var_migid0_dn11_slot: &mut f64,
        var_migid0_dn2_slot: &mut f64,
        var_migid0_dn3_slot: &mut f64,
        var_migid0_dn4_slot: &mut f64,
        var_migid0_dn5_slot: &mut f64,
        var_migid0_dn6_slot: &mut f64,
        var_migid0_dn7_slot: &mut f64,
        var_migid0_dn8_slot: &mut f64,
        var_migid0_dn9_slot: &mut f64,
        var_migid_db0_slot: &mut f64,
        var_migid_db1_slot: &mut f64,
        var_migid_db2_slot: &mut f64,
        var_migid_db3_slot: &mut f64,
        var_migid_db4_slot: &mut f64,
        var_migid_db5_slot: &mut f64,
        var_migid_db6_slot: &mut f64,
        var_migid_dn0_slot: &mut f64,
        var_migid_dn1_slot: &mut f64,
        var_migid_dn10_slot: &mut f64,
        var_migid_dn11_slot: &mut f64,
        var_migid_dn2_slot: &mut f64,
        var_migid_dn3_slot: &mut f64,
        var_migid_dn4_slot: &mut f64,
        var_migid_dn5_slot: &mut f64,
        var_migid_dn6_slot: &mut f64,
        var_migid_dn7_slot: &mut f64,
        var_migid_dn8_slot: &mut f64,
        var_migid_dn9_slot: &mut f64,
        var_sqig_slot: &mut f64,
        var_sqig_db0_slot: &mut f64,
        var_sqig_db1_slot: &mut f64,
        var_sqig_db2_slot: &mut f64,
        var_sqig_db3_slot: &mut f64,
        var_sqig_db4_slot: &mut f64,
        var_sqig_db5_slot: &mut f64,
        var_sqig_db6_slot: &mut f64,
        var_sqig_dn0_slot: &mut f64,
        var_sqig_dn1_slot: &mut f64,
        var_sqig_dn10_slot: &mut f64,
        var_sqig_dn11_slot: &mut f64,
        var_sqig_dn2_slot: &mut f64,
        var_sqig_dn3_slot: &mut f64,
        var_sqig_dn4_slot: &mut f64,
        var_sqig_dn5_slot: &mut f64,
        var_sqig_dn6_slot: &mut f64,
        var_sqig_dn7_slot: &mut f64,
        var_sqig_dn8_slot: &mut f64,
        var_sqig_dn9_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_db0_slot: &mut f64,
        var_temp1_db1_slot: &mut f64,
        var_temp1_db2_slot: &mut f64,
        var_temp1_db3_slot: &mut f64,
        var_temp1_db4_slot: &mut f64,
        var_temp1_db5_slot: &mut f64,
        var_temp1_db6_slot: &mut f64,
        var_temp1_dn0_slot: &mut f64,
        var_temp1_dn1_slot: &mut f64,
        var_temp1_dn10_slot: &mut f64,
        var_temp1_dn11_slot: &mut f64,
        var_temp1_dn2_slot: &mut f64,
        var_temp1_dn3_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
    ) {
        let mut var_c_igid: f64 = *var_c_igid_slot;
        let mut var_c_igid_db0: f64 = *var_c_igid_db0_slot;
        let mut var_c_igid_db1: f64 = *var_c_igid_db1_slot;
        let mut var_c_igid_db2: f64 = *var_c_igid_db2_slot;
        let mut var_c_igid_db3: f64 = *var_c_igid_db3_slot;
        let mut var_c_igid_db4: f64 = *var_c_igid_db4_slot;
        let mut var_c_igid_db5: f64 = *var_c_igid_db5_slot;
        let mut var_c_igid_db6: f64 = *var_c_igid_db6_slot;
        let mut var_c_igid_dn0: f64 = *var_c_igid_dn0_slot;
        let mut var_c_igid_dn1: f64 = *var_c_igid_dn1_slot;
        let mut var_c_igid_dn10: f64 = *var_c_igid_dn10_slot;
        let mut var_c_igid_dn11: f64 = *var_c_igid_dn11_slot;
        let mut var_c_igid_dn2: f64 = *var_c_igid_dn2_slot;
        let mut var_c_igid_dn3: f64 = *var_c_igid_dn3_slot;
        let mut var_c_igid_dn4: f64 = *var_c_igid_dn4_slot;
        let mut var_c_igid_dn5: f64 = *var_c_igid_dn5_slot;
        let mut var_c_igid_dn6: f64 = *var_c_igid_dn6_slot;
        let mut var_c_igid_dn7: f64 = *var_c_igid_dn7_slot;
        let mut var_c_igid_dn8: f64 = *var_c_igid_dn8_slot;
        let mut var_c_igid_dn9: f64 = *var_c_igid_dn9_slot;
        let mut var_cgeff: f64 = *var_cgeff_slot;
        let mut var_cgeff_db0: f64 = *var_cgeff_db0_slot;
        let mut var_cgeff_db1: f64 = *var_cgeff_db1_slot;
        let mut var_cgeff_db2: f64 = *var_cgeff_db2_slot;
        let mut var_cgeff_db3: f64 = *var_cgeff_db3_slot;
        let mut var_cgeff_db4: f64 = *var_cgeff_db4_slot;
        let mut var_cgeff_db5: f64 = *var_cgeff_db5_slot;
        let mut var_cgeff_db6: f64 = *var_cgeff_db6_slot;
        let mut var_cgeff_dn0: f64 = *var_cgeff_dn0_slot;
        let mut var_cgeff_dn1: f64 = *var_cgeff_dn1_slot;
        let mut var_cgeff_dn10: f64 = *var_cgeff_dn10_slot;
        let mut var_cgeff_dn11: f64 = *var_cgeff_dn11_slot;
        let mut var_cgeff_dn2: f64 = *var_cgeff_dn2_slot;
        let mut var_cgeff_dn3: f64 = *var_cgeff_dn3_slot;
        let mut var_cgeff_dn4: f64 = *var_cgeff_dn4_slot;
        let mut var_cgeff_dn5: f64 = *var_cgeff_dn5_slot;
        let mut var_cgeff_dn6: f64 = *var_cgeff_dn6_slot;
        let mut var_cgeff_dn7: f64 = *var_cgeff_dn7_slot;
        let mut var_cgeff_dn8: f64 = *var_cgeff_dn8_slot;
        let mut var_cgeff_dn9: f64 = *var_cgeff_dn9_slot;
        let mut var_guard1766: f64 = *var_guard1766_slot;
        let mut var_guard1767: f64 = *var_guard1767_slot;
        let mut var_guard1769: f64 = *var_guard1769_slot;
        let mut var_mig: f64 = *var_mig_slot;
        let mut var_mig_db0: f64 = *var_mig_db0_slot;
        let mut var_mig_db1: f64 = *var_mig_db1_slot;
        let mut var_mig_db2: f64 = *var_mig_db2_slot;
        let mut var_mig_db3: f64 = *var_mig_db3_slot;
        let mut var_mig_db4: f64 = *var_mig_db4_slot;
        let mut var_mig_db5: f64 = *var_mig_db5_slot;
        let mut var_mig_db6: f64 = *var_mig_db6_slot;
        let mut var_mig_dn0: f64 = *var_mig_dn0_slot;
        let mut var_mig_dn1: f64 = *var_mig_dn1_slot;
        let mut var_mig_dn10: f64 = *var_mig_dn10_slot;
        let mut var_mig_dn11: f64 = *var_mig_dn11_slot;
        let mut var_mig_dn2: f64 = *var_mig_dn2_slot;
        let mut var_mig_dn3: f64 = *var_mig_dn3_slot;
        let mut var_mig_dn4: f64 = *var_mig_dn4_slot;
        let mut var_mig_dn5: f64 = *var_mig_dn5_slot;
        let mut var_mig_dn6: f64 = *var_mig_dn6_slot;
        let mut var_mig_dn7: f64 = *var_mig_dn7_slot;
        let mut var_mig_dn8: f64 = *var_mig_dn8_slot;
        let mut var_mig_dn9: f64 = *var_mig_dn9_slot;
        let mut var_migid: f64 = *var_migid_slot;
        let mut var_migid0: f64 = *var_migid0_slot;
        let mut var_migid0_db0: f64 = *var_migid0_db0_slot;
        let mut var_migid0_db1: f64 = *var_migid0_db1_slot;
        let mut var_migid0_db2: f64 = *var_migid0_db2_slot;
        let mut var_migid0_db3: f64 = *var_migid0_db3_slot;
        let mut var_migid0_db4: f64 = *var_migid0_db4_slot;
        let mut var_migid0_db5: f64 = *var_migid0_db5_slot;
        let mut var_migid0_db6: f64 = *var_migid0_db6_slot;
        let mut var_migid0_dn0: f64 = *var_migid0_dn0_slot;
        let mut var_migid0_dn1: f64 = *var_migid0_dn1_slot;
        let mut var_migid0_dn10: f64 = *var_migid0_dn10_slot;
        let mut var_migid0_dn11: f64 = *var_migid0_dn11_slot;
        let mut var_migid0_dn2: f64 = *var_migid0_dn2_slot;
        let mut var_migid0_dn3: f64 = *var_migid0_dn3_slot;
        let mut var_migid0_dn4: f64 = *var_migid0_dn4_slot;
        let mut var_migid0_dn5: f64 = *var_migid0_dn5_slot;
        let mut var_migid0_dn6: f64 = *var_migid0_dn6_slot;
        let mut var_migid0_dn7: f64 = *var_migid0_dn7_slot;
        let mut var_migid0_dn8: f64 = *var_migid0_dn8_slot;
        let mut var_migid0_dn9: f64 = *var_migid0_dn9_slot;
        let mut var_migid_db0: f64 = *var_migid_db0_slot;
        let mut var_migid_db1: f64 = *var_migid_db1_slot;
        let mut var_migid_db2: f64 = *var_migid_db2_slot;
        let mut var_migid_db3: f64 = *var_migid_db3_slot;
        let mut var_migid_db4: f64 = *var_migid_db4_slot;
        let mut var_migid_db5: f64 = *var_migid_db5_slot;
        let mut var_migid_db6: f64 = *var_migid_db6_slot;
        let mut var_migid_dn0: f64 = *var_migid_dn0_slot;
        let mut var_migid_dn1: f64 = *var_migid_dn1_slot;
        let mut var_migid_dn10: f64 = *var_migid_dn10_slot;
        let mut var_migid_dn11: f64 = *var_migid_dn11_slot;
        let mut var_migid_dn2: f64 = *var_migid_dn2_slot;
        let mut var_migid_dn3: f64 = *var_migid_dn3_slot;
        let mut var_migid_dn4: f64 = *var_migid_dn4_slot;
        let mut var_migid_dn5: f64 = *var_migid_dn5_slot;
        let mut var_migid_dn6: f64 = *var_migid_dn6_slot;
        let mut var_migid_dn7: f64 = *var_migid_dn7_slot;
        let mut var_migid_dn8: f64 = *var_migid_dn8_slot;
        let mut var_migid_dn9: f64 = *var_migid_dn9_slot;
        let mut var_sqig: f64 = *var_sqig_slot;
        let mut var_sqig_db0: f64 = *var_sqig_db0_slot;
        let mut var_sqig_db1: f64 = *var_sqig_db1_slot;
        let mut var_sqig_db2: f64 = *var_sqig_db2_slot;
        let mut var_sqig_db3: f64 = *var_sqig_db3_slot;
        let mut var_sqig_db4: f64 = *var_sqig_db4_slot;
        let mut var_sqig_db5: f64 = *var_sqig_db5_slot;
        let mut var_sqig_db6: f64 = *var_sqig_db6_slot;
        let mut var_sqig_dn0: f64 = *var_sqig_dn0_slot;
        let mut var_sqig_dn1: f64 = *var_sqig_dn1_slot;
        let mut var_sqig_dn10: f64 = *var_sqig_dn10_slot;
        let mut var_sqig_dn11: f64 = *var_sqig_dn11_slot;
        let mut var_sqig_dn2: f64 = *var_sqig_dn2_slot;
        let mut var_sqig_dn3: f64 = *var_sqig_dn3_slot;
        let mut var_sqig_dn4: f64 = *var_sqig_dn4_slot;
        let mut var_sqig_dn5: f64 = *var_sqig_dn5_slot;
        let mut var_sqig_dn6: f64 = *var_sqig_dn6_slot;
        let mut var_sqig_dn7: f64 = *var_sqig_dn7_slot;
        let mut var_sqig_dn8: f64 = *var_sqig_dn8_slot;
        let mut var_sqig_dn9: f64 = *var_sqig_dn9_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_db0: f64 = *var_temp1_db0_slot;
        let mut var_temp1_db1: f64 = *var_temp1_db1_slot;
        let mut var_temp1_db2: f64 = *var_temp1_db2_slot;
        let mut var_temp1_db3: f64 = *var_temp1_db3_slot;
        let mut var_temp1_db4: f64 = *var_temp1_db4_slot;
        let mut var_temp1_db5: f64 = *var_temp1_db5_slot;
        let mut var_temp1_db6: f64 = *var_temp1_db6_slot;
        let mut var_temp1_dn0: f64 = *var_temp1_dn0_slot;
        let mut var_temp1_dn1: f64 = *var_temp1_dn1_slot;
        let mut var_temp1_dn10: f64 = *var_temp1_dn10_slot;
        let mut var_temp1_dn11: f64 = *var_temp1_dn11_slot;
        let mut var_temp1_dn2: f64 = *var_temp1_dn2_slot;
        let mut var_temp1_dn3: f64 = *var_temp1_dn3_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;

        let (assign62510_e81009, assign62510_e81009_d_n0, assign62510_e81009_d_n1, assign62510_e81009_d_n2, assign62510_e81009_d_n3, assign62510_e81009_d_n4, assign62510_e81009_d_n5, assign62510_e81009_d_n6, assign62510_e81009_d_n7, assign62510_e81009_d_n8, assign62510_e81009_d_n9, assign62510_e81009_d_n10, assign62510_e81009_d_n11, assign62510_e81009_d_b0, assign62510_e81009_d_b1, assign62510_e81009_d_b2, assign62510_e81009_d_b3, assign62510_e81009_d_b4, assign62510_e81009_d_b5, assign62510_e81009_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let (assign62510_e81007, assign62510_e81007_d_n0, assign62510_e81007_d_n1, assign62510_e81007_d_n2, assign62510_e81007_d_n3, assign62510_e81007_d_n4, assign62510_e81007_d_n5, assign62510_e81007_d_n6, assign62510_e81007_d_n7, assign62510_e81007_d_n8, assign62510_e81007_d_n9, assign62510_e81007_d_n10, assign62510_e81007_d_n11, assign62510_e81007_d_b0, assign62510_e81007_d_b1, assign62510_e81007_d_b2, assign62510_e81007_d_b3, assign62510_e81007_d_b4, assign62510_e81007_d_b5, assign62510_e81007_d_b6,) = {
            if (var_mig > 1e-40) {
                (var_mig, var_mig_dn0, var_mig_dn1, var_mig_dn2, var_mig_dn3, var_mig_dn4, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn9, var_mig_dn10, var_mig_dn11, var_mig_db0, var_mig_db1, var_mig_db2, var_mig_db3, var_mig_db4, var_mig_db5, var_mig_db6,)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62510_e81007, assign62510_e81007_d_n0, assign62510_e81007_d_n1, assign62510_e81007_d_n2, assign62510_e81007_d_n3, assign62510_e81007_d_n4, assign62510_e81007_d_n5, assign62510_e81007_d_n6, assign62510_e81007_d_n7, assign62510_e81007_d_n8, assign62510_e81007_d_n9, assign62510_e81007_d_n10, assign62510_e81007_d_n11, assign62510_e81007_d_b0, assign62510_e81007_d_b1, assign62510_e81007_d_b2, assign62510_e81007_d_b3, assign62510_e81007_d_b4, assign62510_e81007_d_b5, assign62510_e81007_d_b6,)
    } else {
        (var_mig, var_mig_dn0, var_mig_dn1, var_mig_dn2, var_mig_dn3, var_mig_dn4, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn9, var_mig_dn10, var_mig_dn11, var_mig_db0, var_mig_db1, var_mig_db2, var_mig_db3, var_mig_db4, var_mig_db5, var_mig_db6,)
    }
};
        var_mig = assign62510_e81009;
        var_mig_dn0 = assign62510_e81009_d_n0;
        var_mig_dn1 = assign62510_e81009_d_n1;
        var_mig_dn2 = assign62510_e81009_d_n2;
        var_mig_dn3 = assign62510_e81009_d_n3;
        var_mig_dn4 = assign62510_e81009_d_n4;
        var_mig_dn5 = assign62510_e81009_d_n5;
        var_mig_dn6 = assign62510_e81009_d_n6;
        var_mig_dn7 = assign62510_e81009_d_n7;
        var_mig_dn8 = assign62510_e81009_d_n8;
        var_mig_dn9 = assign62510_e81009_d_n9;
        var_mig_dn10 = assign62510_e81009_d_n10;
        var_mig_dn11 = assign62510_e81009_d_n11;
        var_mig_db0 = assign62510_e81009_d_b0;
        var_mig_db1 = assign62510_e81009_d_b1;
        var_mig_db2 = assign62510_e81009_d_b2;
        var_mig_db3 = assign62510_e81009_d_b3;
        var_mig_db4 = assign62510_e81009_d_b4;
        var_mig_db5 = assign62510_e81009_d_b5;
        var_mig_db6 = assign62510_e81009_d_b6;

        let (assign62520_e81019, assign62520_e81019_d_n0, assign62520_e81019_d_n1, assign62520_e81019_d_n2, assign62520_e81019_d_n3, assign62520_e81019_d_n4, assign62520_e81019_d_n5, assign62520_e81019_d_n6, assign62520_e81019_d_n7, assign62520_e81019_d_n8, assign62520_e81019_d_n9, assign62520_e81019_d_n10, assign62520_e81019_d_n11, assign62520_e81019_d_b0, assign62520_e81019_d_b1, assign62520_e81019_d_b2, assign62520_e81019_d_b3, assign62520_e81019_d_b4, assign62520_e81019_d_b5, assign62520_e81019_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let assign62520_e81015: f64 = (var_lcinv2 / var_g_ideal);
        let assign62520_e81017: f64 = (assign62520_e81015 * var_mig);
        (assign62520_e81017, (((((var_lcinv2_dn0 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn0)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn0)), (((((var_lcinv2_dn1 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn1)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn1)), (((((var_lcinv2_dn2 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn2)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn2)), (((((var_lcinv2_dn3 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn3)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn3)), (((((var_lcinv2_dn4 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn4)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn4)), (((((var_lcinv2_dn5 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn5)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn5)), (((((var_lcinv2_dn6 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn6)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn6)), (((((var_lcinv2_dn7 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn7)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn7)), (((((var_lcinv2_dn8 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn8)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn8)), (((((var_lcinv2_dn9 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn9)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn9)), (((((var_lcinv2_dn10 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn10)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn10)), (((((var_lcinv2_dn11 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn11)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn11)), (((((var_lcinv2_db0 * var_g_ideal) - (var_lcinv2 * var_g_ideal_db0)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_db0)), (((((var_lcinv2_db1 * var_g_ideal) - (var_lcinv2 * var_g_ideal_db1)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_db1)), (((((var_lcinv2_db2 * var_g_ideal) - (var_lcinv2 * var_g_ideal_db2)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_db2)), (((((var_lcinv2_db3 * var_g_ideal) - (var_lcinv2 * var_g_ideal_db3)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_db3)), (((((var_lcinv2_db4 * var_g_ideal) - (var_lcinv2 * var_g_ideal_db4)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_db4)), (((((var_lcinv2_db5 * var_g_ideal) - (var_lcinv2 * var_g_ideal_db5)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_db5)), (((((var_lcinv2_db6 * var_g_ideal) - (var_lcinv2 * var_g_ideal_db6)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_db6)),)
    } else {
        (var_mig, var_mig_dn0, var_mig_dn1, var_mig_dn2, var_mig_dn3, var_mig_dn4, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn9, var_mig_dn10, var_mig_dn11, var_mig_db0, var_mig_db1, var_mig_db2, var_mig_db3, var_mig_db4, var_mig_db5, var_mig_db6,)
    }
};
        var_mig = assign62520_e81019;
        var_mig_dn0 = assign62520_e81019_d_n0;
        var_mig_dn1 = assign62520_e81019_d_n1;
        var_mig_dn2 = assign62520_e81019_d_n2;
        var_mig_dn3 = assign62520_e81019_d_n3;
        var_mig_dn4 = assign62520_e81019_d_n4;
        var_mig_dn5 = assign62520_e81019_d_n5;
        var_mig_dn6 = assign62520_e81019_d_n6;
        var_mig_dn7 = assign62520_e81019_d_n7;
        var_mig_dn8 = assign62520_e81019_d_n8;
        var_mig_dn9 = assign62520_e81019_d_n9;
        var_mig_dn10 = assign62520_e81019_d_n10;
        var_mig_dn11 = assign62520_e81019_d_n11;
        var_mig_db0 = assign62520_e81019_d_b0;
        var_mig_db1 = assign62520_e81019_d_b1;
        var_mig_db2 = assign62520_e81019_d_b2;
        var_mig_db3 = assign62520_e81019_d_b3;
        var_mig_db4 = assign62520_e81019_d_b4;
        var_mig_db5 = assign62520_e81019_d_b5;
        var_mig_db6 = assign62520_e81019_d_b6;

        let (assign62530_e81047, assign62530_e81047_d_n0, assign62530_e81047_d_n1, assign62530_e81047_d_n2, assign62530_e81047_d_n3, assign62530_e81047_d_n4, assign62530_e81047_d_n5, assign62530_e81047_d_n6, assign62530_e81047_d_n7, assign62530_e81047_d_n8, assign62530_e81047_d_n9, assign62530_e81047_d_n10, assign62530_e81047_d_n11, assign62530_e81047_d_b0, assign62530_e81047_d_b1, assign62530_e81047_d_b2, assign62530_e81047_d_b3, assign62530_e81047_d_b4, assign62530_e81047_d_b5, assign62530_e81047_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let assign62530_e81025: f64 = (var_lcinv2 * var_sqt2);
        let assign62530_e81029: f64 = (12.0 * var_t2);
        let assign62530_e81030: f64 = (1.0 - assign62530_e81029);
        let assign62530_e81034: f64 = (19.2 * var_t2);
        let assign62530_e81035: f64 = (var_t1 + assign62530_e81034);
        let assign62530_e81039: f64 = (var_t1 * var_t2);
        let assign62530_e81040: f64 = (12.0 * assign62530_e81039);
        let assign62530_e81041: f64 = (assign62530_e81035 - assign62530_e81040);
        let assign62530_e81043: f64 = (assign62530_e81041 * var_r);
        let assign62530_e81044: f64 = (assign62530_e81030 - assign62530_e81043);
        let assign62530_e81045: f64 = (assign62530_e81025 * assign62530_e81044);
        (assign62530_e81045, ((((var_lcinv2_dn0 * var_sqt2) + (var_lcinv2 * var_sqt2_dn0)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn0)) - ((((var_t1_dn0 + (19.2 * var_t2_dn0)) - (12.0 * ((var_t1_dn0 * var_t2) + (var_t1 * var_t2_dn0)))) * var_r) + (assign62530_e81041 * var_r_dn0))))), ((((var_lcinv2_dn1 * var_sqt2) + (var_lcinv2 * var_sqt2_dn1)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn1)) - ((((var_t1_dn1 + (19.2 * var_t2_dn1)) - (12.0 * ((var_t1_dn1 * var_t2) + (var_t1 * var_t2_dn1)))) * var_r) + (assign62530_e81041 * var_r_dn1))))), ((((var_lcinv2_dn2 * var_sqt2) + (var_lcinv2 * var_sqt2_dn2)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn2)) - ((((var_t1_dn2 + (19.2 * var_t2_dn2)) - (12.0 * ((var_t1_dn2 * var_t2) + (var_t1 * var_t2_dn2)))) * var_r) + (assign62530_e81041 * var_r_dn2))))), ((((var_lcinv2_dn3 * var_sqt2) + (var_lcinv2 * var_sqt2_dn3)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn3)) - ((((var_t1_dn3 + (19.2 * var_t2_dn3)) - (12.0 * ((var_t1_dn3 * var_t2) + (var_t1 * var_t2_dn3)))) * var_r) + (assign62530_e81041 * var_r_dn3))))), ((((var_lcinv2_dn4 * var_sqt2) + (var_lcinv2 * var_sqt2_dn4)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn4)) - ((((var_t1_dn4 + (19.2 * var_t2_dn4)) - (12.0 * ((var_t1_dn4 * var_t2) + (var_t1 * var_t2_dn4)))) * var_r) + (assign62530_e81041 * var_r_dn4))))), ((((var_lcinv2_dn5 * var_sqt2) + (var_lcinv2 * var_sqt2_dn5)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn5)) - ((((var_t1_dn5 + (19.2 * var_t2_dn5)) - (12.0 * ((var_t1_dn5 * var_t2) + (var_t1 * var_t2_dn5)))) * var_r) + (assign62530_e81041 * var_r_dn5))))), ((((var_lcinv2_dn6 * var_sqt2) + (var_lcinv2 * var_sqt2_dn6)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn6)) - ((((var_t1_dn6 + (19.2 * var_t2_dn6)) - (12.0 * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)))) * var_r) + (assign62530_e81041 * var_r_dn6))))), ((((var_lcinv2_dn7 * var_sqt2) + (var_lcinv2 * var_sqt2_dn7)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn7)) - ((((var_t1_dn7 + (19.2 * var_t2_dn7)) - (12.0 * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)))) * var_r) + (assign62530_e81041 * var_r_dn7))))), ((((var_lcinv2_dn8 * var_sqt2) + (var_lcinv2 * var_sqt2_dn8)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn8)) - ((((var_t1_dn8 + (19.2 * var_t2_dn8)) - (12.0 * ((var_t1_dn8 * var_t2) + (var_t1 * var_t2_dn8)))) * var_r) + (assign62530_e81041 * var_r_dn8))))), ((((var_lcinv2_dn9 * var_sqt2) + (var_lcinv2 * var_sqt2_dn9)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn9)) - ((((var_t1_dn9 + (19.2 * var_t2_dn9)) - (12.0 * ((var_t1_dn9 * var_t2) + (var_t1 * var_t2_dn9)))) * var_r) + (assign62530_e81041 * var_r_dn9))))), ((((var_lcinv2_dn10 * var_sqt2) + (var_lcinv2 * var_sqt2_dn10)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn10)) - ((((var_t1_dn10 + (19.2 * var_t2_dn10)) - (12.0 * ((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10)))) * var_r) + (assign62530_e81041 * var_r_dn10))))), ((((var_lcinv2_dn11 * var_sqt2) + (var_lcinv2 * var_sqt2_dn11)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn11)) - ((((var_t1_dn11 + (19.2 * var_t2_dn11)) - (12.0 * ((var_t1_dn11 * var_t2) + (var_t1 * var_t2_dn11)))) * var_r) + (assign62530_e81041 * var_r_dn11))))), ((((var_lcinv2_db0 * var_sqt2) + (var_lcinv2 * var_sqt2_db0)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_db0)) - ((((var_t1_db0 + (19.2 * var_t2_db0)) - (12.0 * ((var_t1_db0 * var_t2) + (var_t1 * var_t2_db0)))) * var_r) + (assign62530_e81041 * var_r_db0))))), ((((var_lcinv2_db1 * var_sqt2) + (var_lcinv2 * var_sqt2_db1)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_db1)) - ((((var_t1_db1 + (19.2 * var_t2_db1)) - (12.0 * ((var_t1_db1 * var_t2) + (var_t1 * var_t2_db1)))) * var_r) + (assign62530_e81041 * var_r_db1))))), ((((var_lcinv2_db2 * var_sqt2) + (var_lcinv2 * var_sqt2_db2)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_db2)) - ((((var_t1_db2 + (19.2 * var_t2_db2)) - (12.0 * ((var_t1_db2 * var_t2) + (var_t1 * var_t2_db2)))) * var_r) + (assign62530_e81041 * var_r_db2))))), ((((var_lcinv2_db3 * var_sqt2) + (var_lcinv2 * var_sqt2_db3)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_db3)) - ((((var_t1_db3 + (19.2 * var_t2_db3)) - (12.0 * ((var_t1_db3 * var_t2) + (var_t1 * var_t2_db3)))) * var_r) + (assign62530_e81041 * var_r_db3))))), ((((var_lcinv2_db4 * var_sqt2) + (var_lcinv2 * var_sqt2_db4)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_db4)) - ((((var_t1_db4 + (19.2 * var_t2_db4)) - (12.0 * ((var_t1_db4 * var_t2) + (var_t1 * var_t2_db4)))) * var_r) + (assign62530_e81041 * var_r_db4))))), ((((var_lcinv2_db5 * var_sqt2) + (var_lcinv2 * var_sqt2_db5)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_db5)) - ((((var_t1_db5 + (19.2 * var_t2_db5)) - (12.0 * ((var_t1_db5 * var_t2) + (var_t1 * var_t2_db5)))) * var_r) + (assign62530_e81041 * var_r_db5))))), ((((var_lcinv2_db6 * var_sqt2) + (var_lcinv2 * var_sqt2_db6)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_db6)) - ((((var_t1_db6 + (19.2 * var_t2_db6)) - (12.0 * ((var_t1_db6 * var_t2) + (var_t1 * var_t2_db6)))) * var_r) + (assign62530_e81041 * var_r_db6))))),)
    } else {
        (var_migid0, var_migid0_dn0, var_migid0_dn1, var_migid0_dn2, var_migid0_dn3, var_migid0_dn4, var_migid0_dn5, var_migid0_dn6, var_migid0_dn7, var_migid0_dn8, var_migid0_dn9, var_migid0_dn10, var_migid0_dn11, var_migid0_db0, var_migid0_db1, var_migid0_db2, var_migid0_db3, var_migid0_db4, var_migid0_db5, var_migid0_db6,)
    }
};
        var_migid0 = assign62530_e81047;
        var_migid0_dn0 = assign62530_e81047_d_n0;
        var_migid0_dn1 = assign62530_e81047_d_n1;
        var_migid0_dn2 = assign62530_e81047_d_n2;
        var_migid0_dn3 = assign62530_e81047_d_n3;
        var_migid0_dn4 = assign62530_e81047_d_n4;
        var_migid0_dn5 = assign62530_e81047_d_n5;
        var_migid0_dn6 = assign62530_e81047_d_n6;
        var_migid0_dn7 = assign62530_e81047_d_n7;
        var_migid0_dn8 = assign62530_e81047_d_n8;
        var_migid0_dn9 = assign62530_e81047_d_n9;
        var_migid0_dn10 = assign62530_e81047_d_n10;
        var_migid0_dn11 = assign62530_e81047_d_n11;
        var_migid0_db0 = assign62530_e81047_d_b0;
        var_migid0_db1 = assign62530_e81047_d_b1;
        var_migid0_db2 = assign62530_e81047_d_b2;
        var_migid0_db3 = assign62530_e81047_d_b3;
        var_migid0_db4 = assign62530_e81047_d_b4;
        var_migid0_db5 = assign62530_e81047_d_b5;
        var_migid0_db6 = assign62530_e81047_d_b6;

        let (assign62540_e81063, assign62540_e81063_d_n0, assign62540_e81063_d_n1, assign62540_e81063_d_n2, assign62540_e81063_d_n3, assign62540_e81063_d_n4, assign62540_e81063_d_n5, assign62540_e81063_d_n6, assign62540_e81063_d_n7, assign62540_e81063_d_n8, assign62540_e81063_d_n9, assign62540_e81063_d_n10, assign62540_e81063_d_n11, assign62540_e81063_d_b0, assign62540_e81063_d_b1, assign62540_e81063_d_b2, assign62540_e81063_d_b3, assign62540_e81063_d_b4, assign62540_e81063_d_b5, assign62540_e81063_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let assign62540_e81053: f64 = (var_gvsat_ac * var_gvsat_ac);
        let assign62540_e81055: f64 = (assign62540_e81053 * var_cox_qm);
        let assign62540_e81057: f64 = (assign62540_e81055 * var_eta_p_ac);
        let assign62540_e81060: f64 = (var_gmob_dl_ac * var_gmob_dl_ac);
        let assign62540_e81061: f64 = (assign62540_e81057 / assign62540_e81060);
        (assign62540_e81061, (((((((((var_gvsat_ac_dn0 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn0)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn0)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn0)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn0 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn0)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_dn1 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn1)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn1)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn1)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn1 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn1)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_dn2 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn2)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn2)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn2)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn2 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn2)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_dn3 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn3)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn3)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn3)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn3 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn3)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_dn4 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn4)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn4)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn4)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn4 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn4)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_dn5 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn5)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn5)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn5)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn5 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn5)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_dn6 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn6)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn6)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn6)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn6 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn6)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_dn7 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn7)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn7)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn7)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn7 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn7)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_dn8 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn8)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn8)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn8)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn8 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn8)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_dn9 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn9)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn9)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn9)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn9 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn9)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_dn10 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn10)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn10)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn10)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn10 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn10)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_dn11 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn11)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn11)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn11)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn11 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn11)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_db0 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_db0)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_db0)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_db0)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_db0 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_db0)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_db1 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_db1)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_db1)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_db1)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_db1 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_db1)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_db2 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_db2)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_db2)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_db2)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_db2 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_db2)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_db3 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_db3)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_db3)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_db3)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_db3 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_db3)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_db4 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_db4)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_db4)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_db4)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_db4 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_db4)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_db5 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_db5)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_db5)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_db5)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_db5 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_db5)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_db6 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_db6)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_db6)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_db6)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_db6 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_db6)))) / (assign62540_e81060 * assign62540_e81060)),)
    } else {
        (var_cgeff, var_cgeff_dn0, var_cgeff_dn1, var_cgeff_dn2, var_cgeff_dn3, var_cgeff_dn4, var_cgeff_dn5, var_cgeff_dn6, var_cgeff_dn7, var_cgeff_dn8, var_cgeff_dn9, var_cgeff_dn10, var_cgeff_dn11, var_cgeff_db0, var_cgeff_db1, var_cgeff_db2, var_cgeff_db3, var_cgeff_db4, var_cgeff_db5, var_cgeff_db6,)
    }
};
        var_cgeff = assign62540_e81063;
        var_cgeff_dn0 = assign62540_e81063_d_n0;
        var_cgeff_dn1 = assign62540_e81063_d_n1;
        var_cgeff_dn2 = assign62540_e81063_d_n2;
        var_cgeff_dn3 = assign62540_e81063_d_n3;
        var_cgeff_dn4 = assign62540_e81063_d_n4;
        var_cgeff_dn5 = assign62540_e81063_d_n5;
        var_cgeff_dn6 = assign62540_e81063_d_n6;
        var_cgeff_dn7 = assign62540_e81063_d_n7;
        var_cgeff_dn8 = assign62540_e81063_d_n8;
        var_cgeff_dn9 = assign62540_e81063_d_n9;
        var_cgeff_dn10 = assign62540_e81063_d_n10;
        var_cgeff_dn11 = assign62540_e81063_d_n11;
        var_cgeff_db0 = assign62540_e81063_d_b0;
        var_cgeff_db1 = assign62540_e81063_d_b1;
        var_cgeff_db2 = assign62540_e81063_d_b2;
        var_cgeff_db3 = assign62540_e81063_d_b3;
        var_cgeff_db4 = assign62540_e81063_d_b4;
        var_cgeff_db5 = assign62540_e81063_d_b5;
        var_cgeff_db6 = assign62540_e81063_d_b6;

        let assign62550_e81066: f64 = if var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1766 = assign62550_e81066;

        let (assign62560_e81090, assign62560_e81090_d_n0, assign62560_e81090_d_n1, assign62560_e81090_d_n2, assign62560_e81090_d_n3, assign62560_e81090_d_n4, assign62560_e81090_d_n5, assign62560_e81090_d_n6, assign62560_e81090_d_n7, assign62560_e81090_d_n8, assign62560_e81090_d_n9, assign62560_e81090_d_n10, assign62560_e81090_d_n11, assign62560_e81090_d_b0, assign62560_e81090_d_b1, assign62560_e81090_d_b2, assign62560_e81090_d_b3, assign62560_e81090_d_b4, assign62560_e81090_d_b5, assign62560_e81090_d_b6,) = {
    if (((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) && (var_guard1766 != 0.0)) {
        let assign62560_e81077: f64 = (12.0 * var_t2);
        let assign62560_e81078: f64 = (1.0 + assign62560_e81077);
        let assign62560_e81079: f64 = (var_sidexc * assign62560_e81078);
        let assign62560_e81082: f64 = (12.0 * var_g_ideal);
        let assign62560_e81084: f64 = (assign62560_e81082 * var_g_ideal);
        let assign62560_e81086: f64 = (assign62560_e81084 * var_nt0);
        let assign62560_e81087: f64 = (assign62560_e81079 / assign62560_e81086);
        let assign62560_e81088: f64 = (var_mig + assign62560_e81087);
        (assign62560_e81088, (var_mig_dn0 + (((((var_sidexc_dn0 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn0))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn0) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn0)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_dn1 + (((((var_sidexc_dn1 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn1))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn1) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn1)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_dn2 + (((((var_sidexc_dn2 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn2))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn2) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn2)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_dn3 + (((((var_sidexc_dn3 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn3))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn3) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn3)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_dn4 + (((((var_sidexc_dn4 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn4))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn4) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn4)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_dn5 + (((((var_sidexc_dn5 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn5))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn5) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn5)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_dn6 + (((((var_sidexc_dn6 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn6))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn6) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn6)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_dn7 + (((((var_sidexc_dn7 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn7))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn7) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn7)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_dn8 + (((((var_sidexc_dn8 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn8))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn8) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn8)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_dn9 + (((((var_sidexc_dn9 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn9))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn9) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn9)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_dn10 + (((((var_sidexc_dn10 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn10))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn10) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn10)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_dn11 + (((((var_sidexc_dn11 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn11))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn11) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn11)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_db0 + (((((var_sidexc_db0 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_db0))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_db0) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_db0)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_db1 + (((((var_sidexc_db1 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_db1))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_db1) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_db1)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_db2 + (((((var_sidexc_db2 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_db2))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_db2) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_db2)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_db3 + (((((var_sidexc_db3 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_db3))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_db3) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_db3)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_db4 + (((((var_sidexc_db4 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_db4))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_db4) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_db4)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_db5 + (((((var_sidexc_db5 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_db5))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_db5) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_db5)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_db6 + (((((var_sidexc_db6 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_db6))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_db6) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_db6)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))),)
    } else {
        (var_mig, var_mig_dn0, var_mig_dn1, var_mig_dn2, var_mig_dn3, var_mig_dn4, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn9, var_mig_dn10, var_mig_dn11, var_mig_db0, var_mig_db1, var_mig_db2, var_mig_db3, var_mig_db4, var_mig_db5, var_mig_db6,)
    }
};
        var_mig = assign62560_e81090;
        var_mig_dn0 = assign62560_e81090_d_n0;
        var_mig_dn1 = assign62560_e81090_d_n1;
        var_mig_dn2 = assign62560_e81090_d_n2;
        var_mig_dn3 = assign62560_e81090_d_n3;
        var_mig_dn4 = assign62560_e81090_d_n4;
        var_mig_dn5 = assign62560_e81090_d_n5;
        var_mig_dn6 = assign62560_e81090_d_n6;
        var_mig_dn7 = assign62560_e81090_d_n7;
        var_mig_dn8 = assign62560_e81090_d_n8;
        var_mig_dn9 = assign62560_e81090_d_n9;
        var_mig_dn10 = assign62560_e81090_d_n10;
        var_mig_dn11 = assign62560_e81090_d_n11;
        var_mig_db0 = assign62560_e81090_d_b0;
        var_mig_db1 = assign62560_e81090_d_b1;
        var_mig_db2 = assign62560_e81090_d_b2;
        var_mig_db3 = assign62560_e81090_d_b3;
        var_mig_db4 = assign62560_e81090_d_b4;
        var_mig_db5 = assign62560_e81090_d_b5;
        var_mig_db6 = assign62560_e81090_d_b6;

        let (assign62570_e81110, assign62570_e81110_d_n0, assign62570_e81110_d_n1, assign62570_e81110_d_n2, assign62570_e81110_d_n3, assign62570_e81110_d_n4, assign62570_e81110_d_n5, assign62570_e81110_d_n6, assign62570_e81110_d_n7, assign62570_e81110_d_n8, assign62570_e81110_d_n9, assign62570_e81110_d_n10, assign62570_e81110_d_n11, assign62570_e81110_d_b0, assign62570_e81110_d_b1, assign62570_e81110_d_b2, assign62570_e81110_d_b3, assign62570_e81110_d_b4, assign62570_e81110_d_b5, assign62570_e81110_d_b6,) = {
    if (((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) && (var_guard1766 != 0.0)) {
        let assign62570_e81099: f64 = (var_sidexc * var_sqt2);
        let assign62570_e81102: f64 = (1.0 + var_r);
        let assign62570_e81103: f64 = (assign62570_e81099 * assign62570_e81102);
        let assign62570_e81106: f64 = (var_g_ideal * var_nt0);
        let assign62570_e81107: f64 = (assign62570_e81103 / assign62570_e81106);
        let assign62570_e81108: f64 = (var_migid0 - assign62570_e81107);
        (assign62570_e81108, (var_migid0_dn0 - (((((((var_sidexc_dn0 * var_sqt2) + (var_sidexc * var_sqt2_dn0)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn0)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn0 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_dn1 - (((((((var_sidexc_dn1 * var_sqt2) + (var_sidexc * var_sqt2_dn1)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn1)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn1 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_dn2 - (((((((var_sidexc_dn2 * var_sqt2) + (var_sidexc * var_sqt2_dn2)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn2)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn2 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_dn3 - (((((((var_sidexc_dn3 * var_sqt2) + (var_sidexc * var_sqt2_dn3)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn3)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn3 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_dn4 - (((((((var_sidexc_dn4 * var_sqt2) + (var_sidexc * var_sqt2_dn4)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn4)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn4 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_dn5 - (((((((var_sidexc_dn5 * var_sqt2) + (var_sidexc * var_sqt2_dn5)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn5)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn5 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_dn6 - (((((((var_sidexc_dn6 * var_sqt2) + (var_sidexc * var_sqt2_dn6)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn6)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn6 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_dn7 - (((((((var_sidexc_dn7 * var_sqt2) + (var_sidexc * var_sqt2_dn7)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn7)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn7 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_dn8 - (((((((var_sidexc_dn8 * var_sqt2) + (var_sidexc * var_sqt2_dn8)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn8)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn8 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_dn9 - (((((((var_sidexc_dn9 * var_sqt2) + (var_sidexc * var_sqt2_dn9)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn9)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn9 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_dn10 - (((((((var_sidexc_dn10 * var_sqt2) + (var_sidexc * var_sqt2_dn10)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn10)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn10 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_dn11 - (((((((var_sidexc_dn11 * var_sqt2) + (var_sidexc * var_sqt2_dn11)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn11)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn11 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_db0 - (((((((var_sidexc_db0 * var_sqt2) + (var_sidexc * var_sqt2_db0)) * assign62570_e81102) + (assign62570_e81099 * var_r_db0)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_db0 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_db1 - (((((((var_sidexc_db1 * var_sqt2) + (var_sidexc * var_sqt2_db1)) * assign62570_e81102) + (assign62570_e81099 * var_r_db1)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_db1 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_db2 - (((((((var_sidexc_db2 * var_sqt2) + (var_sidexc * var_sqt2_db2)) * assign62570_e81102) + (assign62570_e81099 * var_r_db2)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_db2 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_db3 - (((((((var_sidexc_db3 * var_sqt2) + (var_sidexc * var_sqt2_db3)) * assign62570_e81102) + (assign62570_e81099 * var_r_db3)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_db3 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_db4 - (((((((var_sidexc_db4 * var_sqt2) + (var_sidexc * var_sqt2_db4)) * assign62570_e81102) + (assign62570_e81099 * var_r_db4)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_db4 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_db5 - (((((((var_sidexc_db5 * var_sqt2) + (var_sidexc * var_sqt2_db5)) * assign62570_e81102) + (assign62570_e81099 * var_r_db5)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_db5 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_db6 - (((((((var_sidexc_db6 * var_sqt2) + (var_sidexc * var_sqt2_db6)) * assign62570_e81102) + (assign62570_e81099 * var_r_db6)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_db6 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))),)
    } else {
        (var_migid0, var_migid0_dn0, var_migid0_dn1, var_migid0_dn2, var_migid0_dn3, var_migid0_dn4, var_migid0_dn5, var_migid0_dn6, var_migid0_dn7, var_migid0_dn8, var_migid0_dn9, var_migid0_dn10, var_migid0_dn11, var_migid0_db0, var_migid0_db1, var_migid0_db2, var_migid0_db3, var_migid0_db4, var_migid0_db5, var_migid0_db6,)
    }
};
        var_migid0 = assign62570_e81110;
        var_migid0_dn0 = assign62570_e81110_d_n0;
        var_migid0_dn1 = assign62570_e81110_d_n1;
        var_migid0_dn2 = assign62570_e81110_d_n2;
        var_migid0_dn3 = assign62570_e81110_d_n3;
        var_migid0_dn4 = assign62570_e81110_d_n4;
        var_migid0_dn5 = assign62570_e81110_d_n5;
        var_migid0_dn6 = assign62570_e81110_d_n6;
        var_migid0_dn7 = assign62570_e81110_d_n7;
        var_migid0_dn8 = assign62570_e81110_d_n8;
        var_migid0_dn9 = assign62570_e81110_d_n9;
        var_migid0_dn10 = assign62570_e81110_d_n10;
        var_migid0_dn11 = assign62570_e81110_d_n11;
        var_migid0_db0 = assign62570_e81110_d_b0;
        var_migid0_db1 = assign62570_e81110_d_b1;
        var_migid0_db2 = assign62570_e81110_d_b2;
        var_migid0_db3 = assign62570_e81110_d_b3;
        var_migid0_db4 = assign62570_e81110_d_b4;
        var_migid0_db5 = assign62570_e81110_d_b5;
        var_migid0_db6 = assign62570_e81110_d_b6;

        let (assign62580_e81119, assign62580_e81119_d_n0, assign62580_e81119_d_n1, assign62580_e81119_d_n2, assign62580_e81119_d_n3, assign62580_e81119_d_n4, assign62580_e81119_d_n5, assign62580_e81119_d_n6, assign62580_e81119_d_n7, assign62580_e81119_d_n8, assign62580_e81119_d_n9, assign62580_e81119_d_n10, assign62580_e81119_d_n11, assign62580_e81119_d_b0, assign62580_e81119_d_b1, assign62580_e81119_d_b2, assign62580_e81119_d_b3, assign62580_e81119_d_b4, assign62580_e81119_d_b5, assign62580_e81119_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let assign62580_e81116: f64 = (var_nt / var_mig);
        let assign62580_e81117: f64 = (assign62580_e81116).sqrt();
        (assign62580_e81117, ((-((var_nt * var_mig_dn0) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_dn1) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_dn2) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_dn3) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_dn4) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_dn5) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_dn6) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_dn7) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_dn8) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_dn9) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_dn10) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_dn11) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_db0) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_db1) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_db2) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_db3) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_db4) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_db5) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_db6) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)),)
    } else {
        (var_sqig, var_sqig_dn0, var_sqig_dn1, var_sqig_dn2, var_sqig_dn3, var_sqig_dn4, var_sqig_dn5, var_sqig_dn6, var_sqig_dn7, var_sqig_dn8, var_sqig_dn9, var_sqig_dn10, var_sqig_dn11, var_sqig_db0, var_sqig_db1, var_sqig_db2, var_sqig_db3, var_sqig_db4, var_sqig_db5, var_sqig_db6,)
    }
};
        var_sqig = assign62580_e81119;
        var_sqig_dn0 = assign62580_e81119_d_n0;
        var_sqig_dn1 = assign62580_e81119_d_n1;
        var_sqig_dn2 = assign62580_e81119_d_n2;
        var_sqig_dn3 = assign62580_e81119_d_n3;
        var_sqig_dn4 = assign62580_e81119_d_n4;
        var_sqig_dn5 = assign62580_e81119_d_n5;
        var_sqig_dn6 = assign62580_e81119_d_n6;
        var_sqig_dn7 = assign62580_e81119_d_n7;
        var_sqig_dn8 = assign62580_e81119_d_n8;
        var_sqig_dn9 = assign62580_e81119_d_n9;
        var_sqig_dn10 = assign62580_e81119_d_n10;
        var_sqig_dn11 = assign62580_e81119_d_n11;
        var_sqig_db0 = assign62580_e81119_d_b0;
        var_sqig_db1 = assign62580_e81119_d_b1;
        var_sqig_db2 = assign62580_e81119_d_b2;
        var_sqig_db3 = assign62580_e81119_d_b3;
        var_sqig_db4 = assign62580_e81119_d_b4;
        var_sqig_db5 = assign62580_e81119_d_b5;
        var_sqig_db6 = assign62580_e81119_d_b6;

        let assign62590_e81122: f64 = if var_sqid <= 0.0 { 1.0 } else { 0.0 };
        var_guard1767 = assign62590_e81122;

        let (assign62600_e81130, assign62600_e81130_d_n0, assign62600_e81130_d_n1, assign62600_e81130_d_n2, assign62600_e81130_d_n3, assign62600_e81130_d_n4, assign62600_e81130_d_n5, assign62600_e81130_d_n6, assign62600_e81130_d_n7, assign62600_e81130_d_n8, assign62600_e81130_d_n9, assign62600_e81130_d_n10, assign62600_e81130_d_n11, assign62600_e81130_d_b0, assign62600_e81130_d_b1, assign62600_e81130_d_b2, assign62600_e81130_d_b3, assign62600_e81130_d_b4, assign62600_e81130_d_b5, assign62600_e81130_d_b6,) = {
    if (((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) && (var_guard1767 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_c_igid, var_c_igid_dn0, var_c_igid_dn1, var_c_igid_dn2, var_c_igid_dn3, var_c_igid_dn4, var_c_igid_dn5, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8, var_c_igid_dn9, var_c_igid_dn10, var_c_igid_dn11, var_c_igid_db0, var_c_igid_db1, var_c_igid_db2, var_c_igid_db3, var_c_igid_db4, var_c_igid_db5, var_c_igid_db6,)
    }
};
        var_c_igid = assign62600_e81130;
        var_c_igid_dn0 = assign62600_e81130_d_n0;
        var_c_igid_dn1 = assign62600_e81130_d_n1;
        var_c_igid_dn2 = assign62600_e81130_d_n2;
        var_c_igid_dn3 = assign62600_e81130_d_n3;
        var_c_igid_dn4 = assign62600_e81130_d_n4;
        var_c_igid_dn5 = assign62600_e81130_d_n5;
        var_c_igid_dn6 = assign62600_e81130_d_n6;
        var_c_igid_dn7 = assign62600_e81130_d_n7;
        var_c_igid_dn8 = assign62600_e81130_d_n8;
        var_c_igid_dn9 = assign62600_e81130_d_n9;
        var_c_igid_dn10 = assign62600_e81130_d_n10;
        var_c_igid_dn11 = assign62600_e81130_d_n11;
        var_c_igid_db0 = assign62600_e81130_d_b0;
        var_c_igid_db1 = assign62600_e81130_d_b1;
        var_c_igid_db2 = assign62600_e81130_d_b2;
        var_c_igid_db3 = assign62600_e81130_d_b3;
        var_c_igid_db4 = assign62600_e81130_d_b4;
        var_c_igid_db5 = assign62600_e81130_d_b5;
        var_c_igid_db6 = assign62600_e81130_d_b6;

        let (assign62610_e81143, assign62610_e81143_d_n0, assign62610_e81143_d_n1, assign62610_e81143_d_n2, assign62610_e81143_d_n3, assign62610_e81143_d_n4, assign62610_e81143_d_n5, assign62610_e81143_d_n6, assign62610_e81143_d_n7, assign62610_e81143_d_n8, assign62610_e81143_d_n9, assign62610_e81143_d_n10, assign62610_e81143_d_n11, assign62610_e81143_d_b0, assign62610_e81143_d_b1, assign62610_e81143_d_b2, assign62610_e81143_d_b3, assign62610_e81143_d_b4, assign62610_e81143_d_b5, assign62610_e81143_d_b6,) = {
    if (((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) && (var_guard1767 == 0.0)) {
        let assign62610_e81139: f64 = (var_migid0 * var_sqig);
        let assign62610_e81141: f64 = (assign62610_e81139 / var_sqid);
        (assign62610_e81141, (((((var_migid0_dn0 * var_sqig) + (var_migid0 * var_sqig_dn0)) * var_sqid) - (assign62610_e81139 * var_sqid_dn0)) / (var_sqid * var_sqid)), (((((var_migid0_dn1 * var_sqig) + (var_migid0 * var_sqig_dn1)) * var_sqid) - (assign62610_e81139 * var_sqid_dn1)) / (var_sqid * var_sqid)), (((((var_migid0_dn2 * var_sqig) + (var_migid0 * var_sqig_dn2)) * var_sqid) - (assign62610_e81139 * var_sqid_dn2)) / (var_sqid * var_sqid)), (((((var_migid0_dn3 * var_sqig) + (var_migid0 * var_sqig_dn3)) * var_sqid) - (assign62610_e81139 * var_sqid_dn3)) / (var_sqid * var_sqid)), (((((var_migid0_dn4 * var_sqig) + (var_migid0 * var_sqig_dn4)) * var_sqid) - (assign62610_e81139 * var_sqid_dn4)) / (var_sqid * var_sqid)), (((((var_migid0_dn5 * var_sqig) + (var_migid0 * var_sqig_dn5)) * var_sqid) - (assign62610_e81139 * var_sqid_dn5)) / (var_sqid * var_sqid)), (((((var_migid0_dn6 * var_sqig) + (var_migid0 * var_sqig_dn6)) * var_sqid) - (assign62610_e81139 * var_sqid_dn6)) / (var_sqid * var_sqid)), (((((var_migid0_dn7 * var_sqig) + (var_migid0 * var_sqig_dn7)) * var_sqid) - (assign62610_e81139 * var_sqid_dn7)) / (var_sqid * var_sqid)), (((((var_migid0_dn8 * var_sqig) + (var_migid0 * var_sqig_dn8)) * var_sqid) - (assign62610_e81139 * var_sqid_dn8)) / (var_sqid * var_sqid)), (((((var_migid0_dn9 * var_sqig) + (var_migid0 * var_sqig_dn9)) * var_sqid) - (assign62610_e81139 * var_sqid_dn9)) / (var_sqid * var_sqid)), (((((var_migid0_dn10 * var_sqig) + (var_migid0 * var_sqig_dn10)) * var_sqid) - (assign62610_e81139 * var_sqid_dn10)) / (var_sqid * var_sqid)), (((((var_migid0_dn11 * var_sqig) + (var_migid0 * var_sqig_dn11)) * var_sqid) - (assign62610_e81139 * var_sqid_dn11)) / (var_sqid * var_sqid)), (((((var_migid0_db0 * var_sqig) + (var_migid0 * var_sqig_db0)) * var_sqid) - (assign62610_e81139 * var_sqid_db0)) / (var_sqid * var_sqid)), (((((var_migid0_db1 * var_sqig) + (var_migid0 * var_sqig_db1)) * var_sqid) - (assign62610_e81139 * var_sqid_db1)) / (var_sqid * var_sqid)), (((((var_migid0_db2 * var_sqig) + (var_migid0 * var_sqig_db2)) * var_sqid) - (assign62610_e81139 * var_sqid_db2)) / (var_sqid * var_sqid)), (((((var_migid0_db3 * var_sqig) + (var_migid0 * var_sqig_db3)) * var_sqid) - (assign62610_e81139 * var_sqid_db3)) / (var_sqid * var_sqid)), (((((var_migid0_db4 * var_sqig) + (var_migid0 * var_sqig_db4)) * var_sqid) - (assign62610_e81139 * var_sqid_db4)) / (var_sqid * var_sqid)), (((((var_migid0_db5 * var_sqig) + (var_migid0 * var_sqig_db5)) * var_sqid) - (assign62610_e81139 * var_sqid_db5)) / (var_sqid * var_sqid)), (((((var_migid0_db6 * var_sqig) + (var_migid0 * var_sqig_db6)) * var_sqid) - (assign62610_e81139 * var_sqid_db6)) / (var_sqid * var_sqid)),)
    } else {
        (var_c_igid, var_c_igid_dn0, var_c_igid_dn1, var_c_igid_dn2, var_c_igid_dn3, var_c_igid_dn4, var_c_igid_dn5, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8, var_c_igid_dn9, var_c_igid_dn10, var_c_igid_dn11, var_c_igid_db0, var_c_igid_db1, var_c_igid_db2, var_c_igid_db3, var_c_igid_db4, var_c_igid_db5, var_c_igid_db6,)
    }
};
        var_c_igid = assign62610_e81143;
        var_c_igid_dn0 = assign62610_e81143_d_n0;
        var_c_igid_dn1 = assign62610_e81143_d_n1;
        var_c_igid_dn2 = assign62610_e81143_d_n2;
        var_c_igid_dn3 = assign62610_e81143_d_n3;
        var_c_igid_dn4 = assign62610_e81143_d_n4;
        var_c_igid_dn5 = assign62610_e81143_d_n5;
        var_c_igid_dn6 = assign62610_e81143_d_n6;
        var_c_igid_dn7 = assign62610_e81143_d_n7;
        var_c_igid_dn8 = assign62610_e81143_d_n8;
        var_c_igid_dn9 = assign62610_e81143_d_n9;
        var_c_igid_dn10 = assign62610_e81143_d_n10;
        var_c_igid_dn11 = assign62610_e81143_d_n11;
        var_c_igid_db0 = assign62610_e81143_d_b0;
        var_c_igid_db1 = assign62610_e81143_d_b1;
        var_c_igid_db2 = assign62610_e81143_d_b2;
        var_c_igid_db3 = assign62610_e81143_d_b3;
        var_c_igid_db4 = assign62610_e81143_d_b4;
        var_c_igid_db5 = assign62610_e81143_d_b5;
        var_c_igid_db6 = assign62610_e81143_d_b6;

        let (assign62620_e81159, assign62620_e81159_d_n0, assign62620_e81159_d_n1, assign62620_e81159_d_n2, assign62620_e81159_d_n3, assign62620_e81159_d_n4, assign62620_e81159_d_n5, assign62620_e81159_d_n6, assign62620_e81159_d_n7, assign62620_e81159_d_n8, assign62620_e81159_d_n9, assign62620_e81159_d_n10, assign62620_e81159_d_n11, assign62620_e81159_d_b0, assign62620_e81159_d_b1, assign62620_e81159_d_b2, assign62620_e81159_d_b3, assign62620_e81159_d_b4, assign62620_e81159_d_b5, assign62620_e81159_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let (assign62620_e81157, assign62620_e81157_d_n0, assign62620_e81157_d_n1, assign62620_e81157_d_n2, assign62620_e81157_d_n3, assign62620_e81157_d_n4, assign62620_e81157_d_n5, assign62620_e81157_d_n6, assign62620_e81157_d_n7, assign62620_e81157_d_n8, assign62620_e81157_d_n9, assign62620_e81157_d_n10, assign62620_e81157_d_n11, assign62620_e81157_d_b0, assign62620_e81157_d_b1, assign62620_e81157_d_b2, assign62620_e81157_d_b3, assign62620_e81157_d_b4, assign62620_e81157_d_b5, assign62620_e81157_d_b6,) = {
            if (var_c_igid > 0.0) {
                let (assign62620_e81155, assign62620_e81155_d_n0, assign62620_e81155_d_n1, assign62620_e81155_d_n2, assign62620_e81155_d_n3, assign62620_e81155_d_n4, assign62620_e81155_d_n5, assign62620_e81155_d_n6, assign62620_e81155_d_n7, assign62620_e81155_d_n8, assign62620_e81155_d_n9, assign62620_e81155_d_n10, assign62620_e81155_d_n11, assign62620_e81155_d_b0, assign62620_e81155_d_b1, assign62620_e81155_d_b2, assign62620_e81155_d_b3, assign62620_e81155_d_b4, assign62620_e81155_d_b5, assign62620_e81155_d_b6,) = {
                    if (var_c_igid < 1.0) {
                        (var_c_igid, var_c_igid_dn0, var_c_igid_dn1, var_c_igid_dn2, var_c_igid_dn3, var_c_igid_dn4, var_c_igid_dn5, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8, var_c_igid_dn9, var_c_igid_dn10, var_c_igid_dn11, var_c_igid_db0, var_c_igid_db1, var_c_igid_db2, var_c_igid_db3, var_c_igid_db4, var_c_igid_db5, var_c_igid_db6,)
                    } else {
                        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign62620_e81155, assign62620_e81155_d_n0, assign62620_e81155_d_n1, assign62620_e81155_d_n2, assign62620_e81155_d_n3, assign62620_e81155_d_n4, assign62620_e81155_d_n5, assign62620_e81155_d_n6, assign62620_e81155_d_n7, assign62620_e81155_d_n8, assign62620_e81155_d_n9, assign62620_e81155_d_n10, assign62620_e81155_d_n11, assign62620_e81155_d_b0, assign62620_e81155_d_b1, assign62620_e81155_d_b2, assign62620_e81155_d_b3, assign62620_e81155_d_b4, assign62620_e81155_d_b5, assign62620_e81155_d_b6,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62620_e81157, assign62620_e81157_d_n0, assign62620_e81157_d_n1, assign62620_e81157_d_n2, assign62620_e81157_d_n3, assign62620_e81157_d_n4, assign62620_e81157_d_n5, assign62620_e81157_d_n6, assign62620_e81157_d_n7, assign62620_e81157_d_n8, assign62620_e81157_d_n9, assign62620_e81157_d_n10, assign62620_e81157_d_n11, assign62620_e81157_d_b0, assign62620_e81157_d_b1, assign62620_e81157_d_b2, assign62620_e81157_d_b3, assign62620_e81157_d_b4, assign62620_e81157_d_b5, assign62620_e81157_d_b6,)
    } else {
        (var_c_igid, var_c_igid_dn0, var_c_igid_dn1, var_c_igid_dn2, var_c_igid_dn3, var_c_igid_dn4, var_c_igid_dn5, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8, var_c_igid_dn9, var_c_igid_dn10, var_c_igid_dn11, var_c_igid_db0, var_c_igid_db1, var_c_igid_db2, var_c_igid_db3, var_c_igid_db4, var_c_igid_db5, var_c_igid_db6,)
    }
};
        var_c_igid = assign62620_e81159;
        var_c_igid_dn0 = assign62620_e81159_d_n0;
        var_c_igid_dn1 = assign62620_e81159_d_n1;
        var_c_igid_dn2 = assign62620_e81159_d_n2;
        var_c_igid_dn3 = assign62620_e81159_d_n3;
        var_c_igid_dn4 = assign62620_e81159_d_n4;
        var_c_igid_dn5 = assign62620_e81159_d_n5;
        var_c_igid_dn6 = assign62620_e81159_d_n6;
        var_c_igid_dn7 = assign62620_e81159_d_n7;
        var_c_igid_dn8 = assign62620_e81159_d_n8;
        var_c_igid_dn9 = assign62620_e81159_d_n9;
        var_c_igid_dn10 = assign62620_e81159_d_n10;
        var_c_igid_dn11 = assign62620_e81159_d_n11;
        var_c_igid_db0 = assign62620_e81159_d_b0;
        var_c_igid_db1 = assign62620_e81159_d_b1;
        var_c_igid_db2 = assign62620_e81159_d_b2;
        var_c_igid_db3 = assign62620_e81159_d_b3;
        var_c_igid_db4 = assign62620_e81159_d_b4;
        var_c_igid_db5 = assign62620_e81159_d_b5;
        var_c_igid_db6 = assign62620_e81159_d_b6;

        let (assign62630_e81169, assign62630_e81169_d_n0, assign62630_e81169_d_n1, assign62630_e81169_d_n2, assign62630_e81169_d_n3, assign62630_e81169_d_n4, assign62630_e81169_d_n5, assign62630_e81169_d_n6, assign62630_e81169_d_n7, assign62630_e81169_d_n8, assign62630_e81169_d_n9, assign62630_e81169_d_n10, assign62630_e81169_d_n11, assign62630_e81169_d_b0, assign62630_e81169_d_b1, assign62630_e81169_d_b2, assign62630_e81169_d_b3, assign62630_e81169_d_b4, assign62630_e81169_d_b5, assign62630_e81169_d_b6,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let assign62630_e81165: f64 = (var_c_igid * var_sqid);
        let assign62630_e81167: f64 = (assign62630_e81165 / var_sqig);
        (assign62630_e81167, (((((var_c_igid_dn0 * var_sqid) + (var_c_igid * var_sqid_dn0)) * var_sqig) - (assign62630_e81165 * var_sqig_dn0)) / (var_sqig * var_sqig)), (((((var_c_igid_dn1 * var_sqid) + (var_c_igid * var_sqid_dn1)) * var_sqig) - (assign62630_e81165 * var_sqig_dn1)) / (var_sqig * var_sqig)), (((((var_c_igid_dn2 * var_sqid) + (var_c_igid * var_sqid_dn2)) * var_sqig) - (assign62630_e81165 * var_sqig_dn2)) / (var_sqig * var_sqig)), (((((var_c_igid_dn3 * var_sqid) + (var_c_igid * var_sqid_dn3)) * var_sqig) - (assign62630_e81165 * var_sqig_dn3)) / (var_sqig * var_sqig)), (((((var_c_igid_dn4 * var_sqid) + (var_c_igid * var_sqid_dn4)) * var_sqig) - (assign62630_e81165 * var_sqig_dn4)) / (var_sqig * var_sqig)), (((((var_c_igid_dn5 * var_sqid) + (var_c_igid * var_sqid_dn5)) * var_sqig) - (assign62630_e81165 * var_sqig_dn5)) / (var_sqig * var_sqig)), (((((var_c_igid_dn6 * var_sqid) + (var_c_igid * var_sqid_dn6)) * var_sqig) - (assign62630_e81165 * var_sqig_dn6)) / (var_sqig * var_sqig)), (((((var_c_igid_dn7 * var_sqid) + (var_c_igid * var_sqid_dn7)) * var_sqig) - (assign62630_e81165 * var_sqig_dn7)) / (var_sqig * var_sqig)), (((((var_c_igid_dn8 * var_sqid) + (var_c_igid * var_sqid_dn8)) * var_sqig) - (assign62630_e81165 * var_sqig_dn8)) / (var_sqig * var_sqig)), (((((var_c_igid_dn9 * var_sqid) + (var_c_igid * var_sqid_dn9)) * var_sqig) - (assign62630_e81165 * var_sqig_dn9)) / (var_sqig * var_sqig)), (((((var_c_igid_dn10 * var_sqid) + (var_c_igid * var_sqid_dn10)) * var_sqig) - (assign62630_e81165 * var_sqig_dn10)) / (var_sqig * var_sqig)), (((((var_c_igid_dn11 * var_sqid) + (var_c_igid * var_sqid_dn11)) * var_sqig) - (assign62630_e81165 * var_sqig_dn11)) / (var_sqig * var_sqig)), (((((var_c_igid_db0 * var_sqid) + (var_c_igid * var_sqid_db0)) * var_sqig) - (assign62630_e81165 * var_sqig_db0)) / (var_sqig * var_sqig)), (((((var_c_igid_db1 * var_sqid) + (var_c_igid * var_sqid_db1)) * var_sqig) - (assign62630_e81165 * var_sqig_db1)) / (var_sqig * var_sqig)), (((((var_c_igid_db2 * var_sqid) + (var_c_igid * var_sqid_db2)) * var_sqig) - (assign62630_e81165 * var_sqig_db2)) / (var_sqig * var_sqig)), (((((var_c_igid_db3 * var_sqid) + (var_c_igid * var_sqid_db3)) * var_sqig) - (assign62630_e81165 * var_sqig_db3)) / (var_sqig * var_sqig)), (((((var_c_igid_db4 * var_sqid) + (var_c_igid * var_sqid_db4)) * var_sqig) - (assign62630_e81165 * var_sqig_db4)) / (var_sqig * var_sqig)), (((((var_c_igid_db5 * var_sqid) + (var_c_igid * var_sqid_db5)) * var_sqig) - (assign62630_e81165 * var_sqig_db5)) / (var_sqig * var_sqig)), (((((var_c_igid_db6 * var_sqid) + (var_c_igid * var_sqid_db6)) * var_sqig) - (assign62630_e81165 * var_sqig_db6)) / (var_sqig * var_sqig)),)
    } else {
        (var_migid, var_migid_dn0, var_migid_dn1, var_migid_dn2, var_migid_dn3, var_migid_dn4, var_migid_dn5, var_migid_dn6, var_migid_dn7, var_migid_dn8, var_migid_dn9, var_migid_dn10, var_migid_dn11, var_migid_db0, var_migid_db1, var_migid_db2, var_migid_db3, var_migid_db4, var_migid_db5, var_migid_db6,)
    }
};
        var_migid = assign62630_e81169;
        var_migid_dn0 = assign62630_e81169_d_n0;
        var_migid_dn1 = assign62630_e81169_d_n1;
        var_migid_dn2 = assign62630_e81169_d_n2;
        var_migid_dn3 = assign62630_e81169_d_n3;
        var_migid_dn4 = assign62630_e81169_d_n4;
        var_migid_dn5 = assign62630_e81169_d_n5;
        var_migid_dn6 = assign62630_e81169_d_n6;
        var_migid_dn7 = assign62630_e81169_d_n7;
        var_migid_dn8 = assign62630_e81169_d_n8;
        var_migid_dn9 = assign62630_e81169_d_n9;
        var_migid_dn10 = assign62630_e81169_d_n10;
        var_migid_dn11 = assign62630_e81169_d_n11;
        var_migid_db0 = assign62630_e81169_d_b0;
        var_migid_db1 = assign62630_e81169_d_b1;
        var_migid_db2 = assign62630_e81169_d_b2;
        var_migid_db3 = assign62630_e81169_d_b3;
        var_migid_db4 = assign62630_e81169_d_b4;
        var_migid_db5 = assign62630_e81169_d_b5;
        var_migid_db6 = assign62630_e81169_d_b6;

        let assign62800_e81277: f64 = if (((p.p46 != 0.0) && (var_betnedge_i > 0.0)) && (var_xgedge > 0.0)) { 1.0 } else { 0.0 };
        var_guard1769 = assign62800_e81277;

        let (assign62810_e81285, assign62810_e81285_d_n0, assign62810_e81285_d_n1, assign62810_e81285_d_n2, assign62810_e81285_d_n3, assign62810_e81285_d_n4, assign62810_e81285_d_n5, assign62810_e81285_d_n6, assign62810_e81285_d_n7, assign62810_e81285_d_n8, assign62810_e81285_d_n9, assign62810_e81285_d_n10, assign62810_e81285_d_n11, assign62810_e81285_d_b0, assign62810_e81285_d_b1, assign62810_e81285_d_b2, assign62810_e81285_d_b3, assign62810_e81285_d_b4, assign62810_e81285_d_b5, assign62810_e81285_d_b6,) = {
    if (var_guard1769 != 0.0) {
        let assign62810_e81281: f64 = (4.0 * var_dsqredge);
        let assign62810_e81283: f64 = (assign62810_e81281 / var_gfedge2);
        (assign62810_e81283, ((4.0 * var_dsqredge_dn0) / var_gfedge2), ((4.0 * var_dsqredge_dn1) / var_gfedge2), ((4.0 * var_dsqredge_dn2) / var_gfedge2), ((4.0 * var_dsqredge_dn3) / var_gfedge2), ((4.0 * var_dsqredge_dn4) / var_gfedge2), ((4.0 * var_dsqredge_dn5) / var_gfedge2), ((4.0 * var_dsqredge_dn6) / var_gfedge2), ((4.0 * var_dsqredge_dn7) / var_gfedge2), ((4.0 * var_dsqredge_dn8) / var_gfedge2), ((4.0 * var_dsqredge_dn9) / var_gfedge2), ((4.0 * var_dsqredge_dn10) / var_gfedge2), ((4.0 * var_dsqredge_dn11) / var_gfedge2), ((4.0 * var_dsqredge_db0) / var_gfedge2), ((4.0 * var_dsqredge_db1) / var_gfedge2), ((4.0 * var_dsqredge_db2) / var_gfedge2), ((4.0 * var_dsqredge_db3) / var_gfedge2), ((4.0 * var_dsqredge_db4) / var_gfedge2), ((4.0 * var_dsqredge_db5) / var_gfedge2), ((4.0 * var_dsqredge_db6) / var_gfedge2),)
    } else {
        (var_temp1, var_temp1_dn0, var_temp1_dn1, var_temp1_dn2, var_temp1_dn3, var_temp1_dn4, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9, var_temp1_dn10, var_temp1_dn11, var_temp1_db0, var_temp1_db1, var_temp1_db2, var_temp1_db3, var_temp1_db4, var_temp1_db5, var_temp1_db6,)
    }
};
        var_temp1 = assign62810_e81285;
        var_temp1_dn0 = assign62810_e81285_d_n0;
        var_temp1_dn1 = assign62810_e81285_d_n1;
        var_temp1_dn2 = assign62810_e81285_d_n2;
        var_temp1_dn3 = assign62810_e81285_d_n3;
        var_temp1_dn4 = assign62810_e81285_d_n4;
        var_temp1_dn5 = assign62810_e81285_d_n5;
        var_temp1_dn6 = assign62810_e81285_d_n6;
        var_temp1_dn7 = assign62810_e81285_d_n7;
        var_temp1_dn8 = assign62810_e81285_d_n8;
        var_temp1_dn9 = assign62810_e81285_d_n9;
        var_temp1_dn10 = assign62810_e81285_d_n10;
        var_temp1_dn11 = assign62810_e81285_d_n11;
        var_temp1_db0 = assign62810_e81285_d_b0;
        var_temp1_db1 = assign62810_e81285_d_b1;
        var_temp1_db2 = assign62810_e81285_d_b2;
        var_temp1_db3 = assign62810_e81285_d_b3;
        var_temp1_db4 = assign62810_e81285_d_b4;
        var_temp1_db5 = assign62810_e81285_d_b5;
        var_temp1_db6 = assign62810_e81285_d_b6;

        let (assign62830_e81305, assign62830_e81305_d_n0, assign62830_e81305_d_n1, assign62830_e81305_d_n2, assign62830_e81305_d_n3, assign62830_e81305_d_n4, assign62830_e81305_d_n5, assign62830_e81305_d_n6, assign62830_e81305_d_n7, assign62830_e81305_d_n8, assign62830_e81305_d_n9, assign62830_e81305_d_n10, assign62830_e81305_d_n11, assign62830_e81305_d_b0, assign62830_e81305_d_b1, assign62830_e81305_d_b2, assign62830_e81305_d_b3, assign62830_e81305_d_b4, assign62830_e81305_d_b5, assign62830_e81305_d_b6,) = {
    if (var_guard1769 != 0.0) {
        let assign62830_e81303: f64 = (var_cox_over_q * var_phit);
        (assign62830_e81303, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn0, var_temp1_dn1, var_temp1_dn2, var_temp1_dn3, var_temp1_dn4, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9, var_temp1_dn10, var_temp1_dn11, var_temp1_db0, var_temp1_db1, var_temp1_db2, var_temp1_db3, var_temp1_db4, var_temp1_db5, var_temp1_db6,)
    }
};
        var_temp1 = assign62830_e81305;
        var_temp1_dn0 = assign62830_e81305_d_n0;
        var_temp1_dn1 = assign62830_e81305_d_n1;
        var_temp1_dn2 = assign62830_e81305_d_n2;
        var_temp1_dn3 = assign62830_e81305_d_n3;
        var_temp1_dn4 = assign62830_e81305_d_n4;
        var_temp1_dn5 = assign62830_e81305_d_n5;
        var_temp1_dn6 = assign62830_e81305_d_n6;
        var_temp1_dn7 = assign62830_e81305_d_n7;
        var_temp1_dn8 = assign62830_e81305_d_n8;
        var_temp1_dn9 = assign62830_e81305_d_n9;
        var_temp1_dn10 = assign62830_e81305_d_n10;
        var_temp1_dn11 = assign62830_e81305_d_n11;
        var_temp1_db0 = assign62830_e81305_d_b0;
        var_temp1_db1 = assign62830_e81305_d_b1;
        var_temp1_db2 = assign62830_e81305_d_b2;
        var_temp1_db3 = assign62830_e81305_d_b3;
        var_temp1_db4 = assign62830_e81305_d_b4;
        var_temp1_db5 = assign62830_e81305_d_b5;
        var_temp1_db6 = assign62830_e81305_d_b6;

        let (assign62960_e81445, assign62960_e81445_d_n0, assign62960_e81445_d_n1, assign62960_e81445_d_n2, assign62960_e81445_d_n3, assign62960_e81445_d_n4, assign62960_e81445_d_n5, assign62960_e81445_d_n6, assign62960_e81445_d_n7, assign62960_e81445_d_n8, assign62960_e81445_d_n9, assign62960_e81445_d_n10, assign62960_e81445_d_n11, assign62960_e81445_d_b0, assign62960_e81445_d_b1, assign62960_e81445_d_b2, assign62960_e81445_d_b3, assign62960_e81445_d_b4, assign62960_e81445_d_b5, assign62960_e81445_d_b6,) = {
    if (var_guard1769 != 0.0) {
        let assign62960_e81443: f64 = (var_alpha_dc * var_h_dc);
        (assign62960_e81443, ((var_alpha_dc_dn0 * var_h_dc) + (var_alpha_dc * var_h_dc_dn0)), ((var_alpha_dc_dn1 * var_h_dc) + (var_alpha_dc * var_h_dc_dn1)), ((var_alpha_dc_dn2 * var_h_dc) + (var_alpha_dc * var_h_dc_dn2)), ((var_alpha_dc_dn3 * var_h_dc) + (var_alpha_dc * var_h_dc_dn3)), ((var_alpha_dc_dn4 * var_h_dc) + (var_alpha_dc * var_h_dc_dn4)), ((var_alpha_dc_dn5 * var_h_dc) + (var_alpha_dc * var_h_dc_dn5)), ((var_alpha_dc_dn6 * var_h_dc) + (var_alpha_dc * var_h_dc_dn6)), ((var_alpha_dc_dn7 * var_h_dc) + (var_alpha_dc * var_h_dc_dn7)), ((var_alpha_dc_dn8 * var_h_dc) + (var_alpha_dc * var_h_dc_dn8)), ((var_alpha_dc_dn9 * var_h_dc) + (var_alpha_dc * var_h_dc_dn9)), ((var_alpha_dc_dn10 * var_h_dc) + (var_alpha_dc * var_h_dc_dn10)), ((var_alpha_dc_dn11 * var_h_dc) + (var_alpha_dc * var_h_dc_dn11)), ((var_alpha_dc_db0 * var_h_dc) + (var_alpha_dc * var_h_dc_db0)), ((var_alpha_dc_db1 * var_h_dc) + (var_alpha_dc * var_h_dc_db1)), ((var_alpha_dc_db2 * var_h_dc) + (var_alpha_dc * var_h_dc_db2)), ((var_alpha_dc_db3 * var_h_dc) + (var_alpha_dc * var_h_dc_db3)), ((var_alpha_dc_db4 * var_h_dc) + (var_alpha_dc * var_h_dc_db4)), ((var_alpha_dc_db5 * var_h_dc) + (var_alpha_dc * var_h_dc_db5)), ((var_alpha_dc_db6 * var_h_dc) + (var_alpha_dc * var_h_dc_db6)),)
    } else {
        (var_temp1, var_temp1_dn0, var_temp1_dn1, var_temp1_dn2, var_temp1_dn3, var_temp1_dn4, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9, var_temp1_dn10, var_temp1_dn11, var_temp1_db0, var_temp1_db1, var_temp1_db2, var_temp1_db3, var_temp1_db4, var_temp1_db5, var_temp1_db6,)
    }
};
        var_temp1 = assign62960_e81445;
        var_temp1_dn0 = assign62960_e81445_d_n0;
        var_temp1_dn1 = assign62960_e81445_d_n1;
        var_temp1_dn2 = assign62960_e81445_d_n2;
        var_temp1_dn3 = assign62960_e81445_d_n3;
        var_temp1_dn4 = assign62960_e81445_d_n4;
        var_temp1_dn5 = assign62960_e81445_d_n5;
        var_temp1_dn6 = assign62960_e81445_d_n6;
        var_temp1_dn7 = assign62960_e81445_d_n7;
        var_temp1_dn8 = assign62960_e81445_d_n8;
        var_temp1_dn9 = assign62960_e81445_d_n9;
        var_temp1_dn10 = assign62960_e81445_d_n10;
        var_temp1_dn11 = assign62960_e81445_d_n11;
        var_temp1_db0 = assign62960_e81445_d_b0;
        var_temp1_db1 = assign62960_e81445_d_b1;
        var_temp1_db2 = assign62960_e81445_d_b2;
        var_temp1_db3 = assign62960_e81445_d_b3;
        var_temp1_db4 = assign62960_e81445_d_b4;
        var_temp1_db5 = assign62960_e81445_d_b5;
        var_temp1_db6 = assign62960_e81445_d_b6;

        *var_c_igid_slot = var_c_igid;
        *var_c_igid_db0_slot = var_c_igid_db0;
        *var_c_igid_db1_slot = var_c_igid_db1;
        *var_c_igid_db2_slot = var_c_igid_db2;
        *var_c_igid_db3_slot = var_c_igid_db3;
        *var_c_igid_db4_slot = var_c_igid_db4;
        *var_c_igid_db5_slot = var_c_igid_db5;
        *var_c_igid_db6_slot = var_c_igid_db6;
        *var_c_igid_dn0_slot = var_c_igid_dn0;
        *var_c_igid_dn1_slot = var_c_igid_dn1;
        *var_c_igid_dn10_slot = var_c_igid_dn10;
        *var_c_igid_dn11_slot = var_c_igid_dn11;
        *var_c_igid_dn2_slot = var_c_igid_dn2;
        *var_c_igid_dn3_slot = var_c_igid_dn3;
        *var_c_igid_dn4_slot = var_c_igid_dn4;
        *var_c_igid_dn5_slot = var_c_igid_dn5;
        *var_c_igid_dn6_slot = var_c_igid_dn6;
        *var_c_igid_dn7_slot = var_c_igid_dn7;
        *var_c_igid_dn8_slot = var_c_igid_dn8;
        *var_c_igid_dn9_slot = var_c_igid_dn9;
        *var_cgeff_slot = var_cgeff;
        *var_cgeff_db0_slot = var_cgeff_db0;
        *var_cgeff_db1_slot = var_cgeff_db1;
        *var_cgeff_db2_slot = var_cgeff_db2;
        *var_cgeff_db3_slot = var_cgeff_db3;
        *var_cgeff_db4_slot = var_cgeff_db4;
        *var_cgeff_db5_slot = var_cgeff_db5;
        *var_cgeff_db6_slot = var_cgeff_db6;
        *var_cgeff_dn0_slot = var_cgeff_dn0;
        *var_cgeff_dn1_slot = var_cgeff_dn1;
        *var_cgeff_dn10_slot = var_cgeff_dn10;
        *var_cgeff_dn11_slot = var_cgeff_dn11;
        *var_cgeff_dn2_slot = var_cgeff_dn2;
        *var_cgeff_dn3_slot = var_cgeff_dn3;
        *var_cgeff_dn4_slot = var_cgeff_dn4;
        *var_cgeff_dn5_slot = var_cgeff_dn5;
        *var_cgeff_dn6_slot = var_cgeff_dn6;
        *var_cgeff_dn7_slot = var_cgeff_dn7;
        *var_cgeff_dn8_slot = var_cgeff_dn8;
        *var_cgeff_dn9_slot = var_cgeff_dn9;
        *var_guard1766_slot = var_guard1766;
        *var_guard1767_slot = var_guard1767;
        *var_guard1769_slot = var_guard1769;
        *var_mig_slot = var_mig;
        *var_mig_db0_slot = var_mig_db0;
        *var_mig_db1_slot = var_mig_db1;
        *var_mig_db2_slot = var_mig_db2;
        *var_mig_db3_slot = var_mig_db3;
        *var_mig_db4_slot = var_mig_db4;
        *var_mig_db5_slot = var_mig_db5;
        *var_mig_db6_slot = var_mig_db6;
        *var_mig_dn0_slot = var_mig_dn0;
        *var_mig_dn1_slot = var_mig_dn1;
        *var_mig_dn10_slot = var_mig_dn10;
        *var_mig_dn11_slot = var_mig_dn11;
        *var_mig_dn2_slot = var_mig_dn2;
        *var_mig_dn3_slot = var_mig_dn3;
        *var_mig_dn4_slot = var_mig_dn4;
        *var_mig_dn5_slot = var_mig_dn5;
        *var_mig_dn6_slot = var_mig_dn6;
        *var_mig_dn7_slot = var_mig_dn7;
        *var_mig_dn8_slot = var_mig_dn8;
        *var_mig_dn9_slot = var_mig_dn9;
        *var_migid_slot = var_migid;
        *var_migid0_slot = var_migid0;
        *var_migid0_db0_slot = var_migid0_db0;
        *var_migid0_db1_slot = var_migid0_db1;
        *var_migid0_db2_slot = var_migid0_db2;
        *var_migid0_db3_slot = var_migid0_db3;
        *var_migid0_db4_slot = var_migid0_db4;
        *var_migid0_db5_slot = var_migid0_db5;
        *var_migid0_db6_slot = var_migid0_db6;
        *var_migid0_dn0_slot = var_migid0_dn0;
        *var_migid0_dn1_slot = var_migid0_dn1;
        *var_migid0_dn10_slot = var_migid0_dn10;
        *var_migid0_dn11_slot = var_migid0_dn11;
        *var_migid0_dn2_slot = var_migid0_dn2;
        *var_migid0_dn3_slot = var_migid0_dn3;
        *var_migid0_dn4_slot = var_migid0_dn4;
        *var_migid0_dn5_slot = var_migid0_dn5;
        *var_migid0_dn6_slot = var_migid0_dn6;
        *var_migid0_dn7_slot = var_migid0_dn7;
        *var_migid0_dn8_slot = var_migid0_dn8;
        *var_migid0_dn9_slot = var_migid0_dn9;
        *var_migid_db0_slot = var_migid_db0;
        *var_migid_db1_slot = var_migid_db1;
        *var_migid_db2_slot = var_migid_db2;
        *var_migid_db3_slot = var_migid_db3;
        *var_migid_db4_slot = var_migid_db4;
        *var_migid_db5_slot = var_migid_db5;
        *var_migid_db6_slot = var_migid_db6;
        *var_migid_dn0_slot = var_migid_dn0;
        *var_migid_dn1_slot = var_migid_dn1;
        *var_migid_dn10_slot = var_migid_dn10;
        *var_migid_dn11_slot = var_migid_dn11;
        *var_migid_dn2_slot = var_migid_dn2;
        *var_migid_dn3_slot = var_migid_dn3;
        *var_migid_dn4_slot = var_migid_dn4;
        *var_migid_dn5_slot = var_migid_dn5;
        *var_migid_dn6_slot = var_migid_dn6;
        *var_migid_dn7_slot = var_migid_dn7;
        *var_migid_dn8_slot = var_migid_dn8;
        *var_migid_dn9_slot = var_migid_dn9;
        *var_sqig_slot = var_sqig;
        *var_sqig_db0_slot = var_sqig_db0;
        *var_sqig_db1_slot = var_sqig_db1;
        *var_sqig_db2_slot = var_sqig_db2;
        *var_sqig_db3_slot = var_sqig_db3;
        *var_sqig_db4_slot = var_sqig_db4;
        *var_sqig_db5_slot = var_sqig_db5;
        *var_sqig_db6_slot = var_sqig_db6;
        *var_sqig_dn0_slot = var_sqig_dn0;
        *var_sqig_dn1_slot = var_sqig_dn1;
        *var_sqig_dn10_slot = var_sqig_dn10;
        *var_sqig_dn11_slot = var_sqig_dn11;
        *var_sqig_dn2_slot = var_sqig_dn2;
        *var_sqig_dn3_slot = var_sqig_dn3;
        *var_sqig_dn4_slot = var_sqig_dn4;
        *var_sqig_dn5_slot = var_sqig_dn5;
        *var_sqig_dn6_slot = var_sqig_dn6;
        *var_sqig_dn7_slot = var_sqig_dn7;
        *var_sqig_dn8_slot = var_sqig_dn8;
        *var_sqig_dn9_slot = var_sqig_dn9;
        *var_temp1_slot = var_temp1;
        *var_temp1_db0_slot = var_temp1_db0;
        *var_temp1_db1_slot = var_temp1_db1;
        *var_temp1_db2_slot = var_temp1_db2;
        *var_temp1_db3_slot = var_temp1_db3;
        *var_temp1_db4_slot = var_temp1_db4;
        *var_temp1_db5_slot = var_temp1_db5;
        *var_temp1_db6_slot = var_temp1_db6;
        *var_temp1_dn0_slot = var_temp1_dn0;
        *var_temp1_dn1_slot = var_temp1_dn1;
        *var_temp1_dn10_slot = var_temp1_dn10;
        *var_temp1_dn11_slot = var_temp1_dn11;
        *var_temp1_dn2_slot = var_temp1_dn2;
        *var_temp1_dn3_slot = var_temp1_dn3;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[984] = (p.p37 >= 0.0);
        s.store_scalar(984, if s.b[984] { 1.0 } else { 0.0 });

        if s.b[984] {
            s.store_scalar(0, 1.0);
        }

        if (!s.b[984]) {
            s.store_scalar(0, (-1.0));
        }

        s.store_scalar(761, (8.8541878176e-12 * 11.8));

        s.store_scalar(344, (273.15 + p.p38));

        s.store_scalar(468, 0.0);

        s.b[985] = (p.p920 > 0.5);
        s.store_scalar(985, if s.b[985] { 1.0 } else { 0.0 });

        if s.b[985] {
            s.store_scalar(468, 1.0);
        }

        if (!s.b[985]) {
            s.store_scalar(468, 0.0);
        }

        s.store_scalar(358, (273.15 + p.p816));

        s.store_scalar(361, (1.3806505e-23 / 1.6021918e-19));

        s.store_scalar(362, (s.v[361] * s.v[358]));

        s.store_scalar(363, (1.0 / s.v[362]));

        s.store_scalar(369, ((-((0.000702 * s.v[358]) * s.v[358])) / (1108.0 + s.v[358])));

        s.store_scalar(372, (p.p827 + s.v[369]));

        s.store_scalar(373, (p.p828 + s.v[369]));

        s.store_scalar(374, (p.p829 + s.v[369]));

        s.store_scalar(402, (1.0 - p.p824));

        s.store_scalar(403, (1.0 - p.p825));

        s.store_scalar(404, (1.0 - p.p826));

        s.store_scalar(405, (1.0 / s.v[402]));

        s.store_scalar(406, (1.0 / s.v[403]));

        s.store_scalar(407, (1.0 / s.v[404]));

        s.store_scalar(417, (s.v[761] / p.p818));

        s.store_scalar(418, ((p.p836 * s.v[761]) / p.p819));

        s.store_scalar(419, ((p.p837 * s.v[761]) / p.p820));

        s.store_scalar(420, (1.0 / s.v[417]));

        s.store_scalar(421, (1.0 / s.v[418]));

        s.store_scalar(422, (1.0 / s.v[419]));

        s.store_scalar(423, (1.0 / p.p821));

        s.store_scalar(424, (1.0 / p.p822));

        s.store_scalar(425, (1.0 / p.p823));

        s.store_scalar(438, (1.0 - (1.0 / p.p817)));

        s.store_scalar(442, (1.0 / p.p853));

        s.store_scalar(443, (1.0 / p.p854));

        s.store_scalar(444, (1.0 / p.p855));

        s.b[986] = ((((p.p859 != 1.0) || (p.p860 != 1.0)) || (p.p861 != 1.0)) || (p.p862 != 1.0));
        s.store_scalar(986, if s.b[986] { 1.0 } else { 0.0 });

        if s.b[986] {
            s.store_scalar(467, 1.0);
        }

        if (!s.b[986]) {
            s.store_scalar(467, 0.0);
        }

        s.b[987] = (s.v[467] == 1.0);
        s.store_scalar(987, if s.b[987] { 1.0 } else { 0.0 });

        if s.b[987] {
            s.store_scalar(451, (if ((p.p820 * p.p859) > 1e-18) { (p.p820 * p.p859) } else { 1e-18 }));
        }

        if s.b[987] {
            s.store_scalar(452, (if ((p.p823 * p.p860) > 0.05) { (p.p823 * p.p860) } else { 0.05 }));
        }

        if s.b[987] {
            s.store_scalar(453, (if ((if ((p.p826 * p.p861) > 0.05) { (p.p826 * p.p861) } else { 0.05 }) < 0.95) { (if ((p.p826 * p.p861) > 0.05) { (p.p826 * p.p861) } else { 0.05 }) } else { 0.95 }));
        }

        if s.b[987] {
            s.store_scalar(454, (p.p829 * p.p862));
            s.store_offset(456, 454, s.v[369]);
            s.store_sub_from_scalar(461, 1.0, 453);
            s.store_div_from_scalar(462, 1.0, 461);
        }

        s.b[988] = (p.p44 == 0.0);
        s.store_scalar(988, if s.b[988] { 1.0 } else { 0.0 });

        if s.b[988] {
            s.store_scalar(499, p.p818);
            s.store_scalar(500, p.p819);
            s.store_scalar(501, p.p820);
            s.store_scalar(502, p.p821);
            s.store_scalar(503, p.p822);
            s.store_scalar(504, p.p823);
            s.store_scalar(505, p.p824);
            s.store_scalar(506, p.p825);
            s.store_scalar(507, p.p826);
            s.store_scalar(508, p.p827);
            s.store_scalar(509, p.p828);
            s.store_scalar(510, p.p829);
            s.store_scalar(511, p.p830);
            s.store_scalar(512, p.p831);
            s.store_scalar(513, p.p832);
            s.store_scalar(516, p.p833);
            s.store_scalar(517, p.p834);
            s.store_scalar(518, p.p835);
            s.store_scalar(514, p.p836);
            s.store_scalar(515, p.p837);
            s.store_scalar(519, p.p838);
            s.store_scalar(520, p.p839);
            s.store_scalar(521, p.p840);
            s.store_scalar(522, p.p841);
            s.store_scalar(523, p.p842);
            s.store_scalar(524, p.p843);
            s.store_scalar(525, p.p844);
            s.store_scalar(526, p.p845);
            s.store_scalar(527, p.p846);
            s.store_scalar(528, p.p847);
            s.store_scalar(529, p.p848);
            s.store_scalar(530, p.p849);
            s.store_scalar(531, p.p850);
            s.store_scalar(532, p.p851);
            s.store_scalar(533, p.p852);
            s.store_scalar(534, p.p853);
            s.store_scalar(535, p.p854);
            s.store_scalar(536, p.p855);
            s.store_scalar(537, p.p856);
            s.store_scalar(538, p.p857);
            s.store_scalar(539, p.p858);
            s.store_scalar(547, p.p922);
            s.store_scalar(630, p.p865);
            s.store_scalar(631, p.p866);
            s.store_scalar(632, p.p867);
            s.store_scalar(633, p.p868);
            s.store_scalar(540, p.p859);
            s.store_scalar(541, p.p860);
            s.store_scalar(542, p.p861);
            s.store_scalar(543, p.p862);
            s.store_scalar(544, p.p863);
            s.store_scalar(545, p.p864);
        }

        if (!s.b[988]) {
            s.store_scalar(499, p.p869);
            s.store_scalar(500, p.p870);
            s.store_scalar(501, p.p871);
            s.store_scalar(502, p.p872);
            s.store_scalar(503, p.p873);
            s.store_scalar(504, p.p874);
            s.store_scalar(505, p.p875);
            s.store_scalar(506, p.p876);
            s.store_scalar(507, p.p877);
            s.store_scalar(508, p.p878);
            s.store_scalar(509, p.p879);
            s.store_scalar(510, p.p880);
            s.store_scalar(511, p.p881);
            s.store_scalar(512, p.p882);
            s.store_scalar(513, p.p883);
            s.store_scalar(516, p.p884);
            s.store_scalar(517, p.p885);
            s.store_scalar(518, p.p886);
            s.store_scalar(514, p.p887);
            s.store_scalar(515, p.p888);
            s.store_scalar(519, p.p889);
            s.store_scalar(520, p.p890);
            s.store_scalar(521, p.p891);
            s.store_scalar(522, p.p892);
            s.store_scalar(523, p.p893);
            s.store_scalar(524, p.p894);
            s.store_scalar(525, p.p895);
            s.store_scalar(526, p.p896);
            s.store_scalar(527, p.p897);
            s.store_scalar(528, p.p898);
            s.store_scalar(529, p.p899);
            s.store_scalar(530, p.p900);
            s.store_scalar(531, p.p901);
            s.store_scalar(532, p.p902);
            s.store_scalar(533, p.p903);
            s.store_scalar(534, p.p904);
            s.store_scalar(535, p.p905);
            s.store_scalar(536, p.p906);
            s.store_scalar(537, p.p907);
            s.store_scalar(538, p.p908);
            s.store_scalar(539, p.p909);
            s.store_scalar(547, p.p924);
            s.store_scalar(630, p.p916);
            s.store_scalar(631, p.p917);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (!s.b[988]) {
            s.store_scalar(632, p.p918);
            s.store_scalar(633, p.p919);
            s.store_scalar(540, p.p910);
            s.store_scalar(541, p.p911);
            s.store_scalar(542, p.p912);
            s.store_scalar(543, p.p913);
            s.store_scalar(544, p.p914);
            s.store_scalar(545, p.p915);
        }

        s.store_offset(548, 508, s.v[369]);

        s.store_offset(549, 509, s.v[369]);

        s.store_offset(550, 510, s.v[369]);

        s.store_sub_from_scalar(569, 1.0, 505);

        s.store_sub_from_scalar(570, 1.0, 506);

        s.store_sub_from_scalar(571, 1.0, 507);

        s.store_div_from_scalar(572, 1.0, 569);

        s.store_div_from_scalar(573, 1.0, 570);

        s.store_div_from_scalar(574, 1.0, 571);

        s.store_div_from_scalar(584, s.v[761], 499);

        s.store_div_scaled_inputs_indices(585, 514, s.v[761], 500, 1.0);

        s.store_div_scaled_inputs_indices(586, 515, s.v[761], 501, 1.0);

        s.store_div_from_scalar(587, 1.0, 584);

        s.store_div_from_scalar(588, 1.0, 585);

        s.store_div_from_scalar(589, 1.0, 586);

        s.store_div_from_scalar(590, 1.0, 502);

        s.store_div_from_scalar(591, 1.0, 503);

        s.store_div_from_scalar(592, 1.0, 504);

        s.store_div_from_scalar(608, 1.0, 534);

        s.store_div_from_scalar(609, 1.0, 535);

        s.store_div_from_scalar(610, 1.0, 536);

        s.b[989] = ((((s.v[540] != 1.0) || (s.v[541] != 1.0)) || (s.v[542] != 1.0)) || (s.v[543] != 1.0));
        s.store_scalar(989, if s.b[989] { 1.0 } else { 0.0 });

        if s.b[989] {
            s.store_scalar(629, 1.0);
        }

        if (!s.b[989]) {
            s.store_scalar(629, 0.0);
        }

        s.b[990] = (s.v[629] == 1.0);
        s.store_scalar(990, if s.b[990] { 1.0 } else { 0.0 });

        if s.b[990] {
            if ((s.v[501] * s.v[540]) > 1e-18) {
                s.store_mul(614, 501, 540);
            } else {
                s.store_scalar(614, 1e-18);
            }
        }

        if s.b[990] {
            if ((s.v[504] * s.v[541]) > 0.05) {
                s.store_mul(615, 504, 541);
            } else {
                s.store_scalar(615, 0.05);
            }
        }

        if s.b[990] {
            if ((if ((s.v[507] * s.v[542]) > 0.05) { (s.v[507] * s.v[542]) } else { 0.05 }) < 0.95) {
                if ((s.v[507] * s.v[542]) > 0.05) {
                    s.store_mul(616, 507, 542);
                } else {
                    s.store_scalar(616, 0.05);
                }
            } else {
                s.store_scalar(616, 0.95);
            }
        }

        if s.b[990] {
            s.store_mul(617, 510, 543);
            s.store_offset(619, 617, s.v[369]);
            s.store_sub_from_scalar(624, 1.0, 616);
            s.store_div_from_scalar(625, 1.0, 624);
        }

        s.store_scalar(345, ((ctx_temp + p.p55) + p.p35));

        s.store_scalar(346, (s.v[345] / s.v[344]));

        s.store_scalar(347, (s.v[345] - s.v[344]));

        s.store_scalar(348, ((s.v[345] * 1.3806505e-23) / 1.6021918e-19));

        s.store_scalar(349, (1.0 / s.v[348]));

        s.store_scalar(350, s.v[345]);

        s.store_scalar(351, (s.v[350] * s.v[350]));

        s.store_scalar(352, (s.v[350] - s.v[344]));

        s.store_scalar(353, (s.v[344] / s.v[350]));

        s.store_scalar(354, ((s.v[353]) as f64).ln());

        s.store_scalar(709, ((s.v[350] * 1.3806505e-23) / 1.6021918e-19));

        s.store_scalar(355, (1.0 / s.v[709]));

        s.store_scalar(356, ((1.179 - (9.025e-5 * s.v[350])) - (3.05e-7 * s.v[351])));

        s.store_scalar(357, ((((1.045 + (0.00045 * s.v[350])) * ((0.523 + (0.0014 * s.v[350])) - (1.48e-6 * s.v[351]))) * s.v[351]) / 90000.0));

        if (!(s.v[357] > 0.001)) {
            s.store_scalar(357, 0.001);
        }

        s.store_scalar(359, (((ctx_temp + p.p55) + p.p35)).max((273.15 + (-250.0))));

        s.store_scalar(360, (s.v[359] / s.v[358]));

        s.store_scalar(364, (s.v[361] * s.v[359]));

        s.store_scalar(365, (1.0 / s.v[364]));

        s.store_scalar(370, ((-((0.000702 * s.v[359]) * s.v[359])) / (1108.0 + s.v[359])));

        s.store_scalar(375, (p.p827 + s.v[370]));

        s.store_scalar(376, (p.p828 + s.v[370]));

        s.store_scalar(377, (p.p829 + s.v[370]));

        s.store_scalar(378, (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[372] * s.v[363]) - (s.v[375] * s.v[365])))) as f64).exp()));

        s.store_scalar(379, (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[373] * s.v[363]) - (s.v[376] * s.v[365])))) as f64).exp()));

        s.store_scalar(380, (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[374] * s.v[363]) - (s.v[377] * s.v[365])))) as f64).exp()));

        s.store_scalar(381, ((p.p830 * s.v[378]) * s.v[378]));

        s.store_scalar(382, ((p.p831 * s.v[379]) * s.v[379]));

        s.store_scalar(383, ((p.p832 * s.v[380]) * s.v[380]));

        s.store_scalar(384, ((p.p821 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[378]) as f64).ln())));

        s.store_scalar(385, ((p.p822 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[379]) as f64).ln())));

        s.store_scalar(386, ((p.p823 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[380]) as f64).ln())));

        s.store_scalar(387, (s.v[384] + (s.v[364] * (((1.0 + ((((0.05 - s.v[384]) * s.v[365])) as f64).exp())) as f64).ln())));

        s.store_scalar(388, (s.v[385] + (s.v[364] * (((1.0 + ((((0.05 - s.v[385]) * s.v[365])) as f64).exp())) as f64).ln())));

        s.store_scalar(389, (s.v[386] + (s.v[364] * (((1.0 + ((((0.05 - s.v[386]) * s.v[365])) as f64).exp())) as f64).ln())));

        s.store_scalar(399, (1.0 / s.v[387]));

        s.store_scalar(400, (1.0 / s.v[388]));

        s.store_scalar(401, (1.0 / s.v[389]));

        s.store_scalar(408, (p.p818 * (((p.p821 * s.v[399])) as f64).powf(p.p824)));

        s.store_scalar(409, (p.p819 * (((p.p822 * s.v[400])) as f64).powf(p.p825)));

        s.store_scalar(410, (p.p820 * (((p.p823 * s.v[401])) as f64).powf(p.p826)));

        s.store_scalar(411, ((s.v[408] * s.v[387]) * s.v[405]));

        s.store_scalar(412, ((s.v[409] * s.v[388]) * s.v[406]));

        s.store_scalar(413, ((s.v[410] * s.v[389]) * s.v[407]));

        s.store_scalar(414, (2.0 * s.v[408]));

        s.store_scalar(415, (2.0 * s.v[409]));

        s.store_scalar(416, (2.0 * s.v[410]));

        s.store_scalar(426, ((0.5 * s.v[375])).max(s.v[364]));

        s.store_scalar(427, ((0.5 * s.v[376])).max(s.v[364]));

        s.store_scalar(428, ((0.5 * s.v[377])).max(s.v[364]));

        s.store_scalar(429, (s.v[426] * s.v[365]));

        s.store_scalar(430, (s.v[427] * s.v[365]));

        s.store_scalar(431, (s.v[428] * s.v[365]));

        s.store_scalar(432, (((((((32.0 * p.p841) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[426] * s.v[426]) * s.v[426]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));

        s.store_scalar(433, (((((((32.0 * p.p842) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[427] * s.v[427]) * s.v[427]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));

        s.store_scalar(434, (((((((32.0 * p.p843) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[428] * s.v[428]) * s.v[428]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));

        s.store_scalar(435, (p.p847 * (1.0 + (p.p850 * (s.v[359] - s.v[358])))));

        s.store_scalar(436, (p.p848 * (1.0 + (p.p851 * (s.v[359] - s.v[358])))));

        s.store_scalar(437, (p.p849 * (1.0 + (p.p852 * (s.v[359] - s.v[358])))));

        if (!(s.v[435] > 0.0)) {
            s.store_scalar(435, 0.0);
        }

        if (!(s.v[436] > 0.0)) {
            s.store_scalar(436, 0.0);
        }

        if (!(s.v[437] > 0.0)) {
            s.store_scalar(437, 0.0);
        }

        s.b[1010] = (s.v[467] == 1.0);
        s.store_scalar(1010, if s.b[1010] { 1.0 } else { 0.0 });

        if s.b[1010] {
            s.store_offset(455, 454, s.v[370]);
            s.store_scale_ad(457, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(456), s.v[363], s.ad_value(455), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(458, 452, s.v[360], A::ln(s.ad_value(457)), (2.0 * s.v[364]));
            s.store_add_scaled_inputs_ad_rhs(459, 458, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(458), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);
            s.store_div_from_scalar(460, 1.0, 459);
            s.store_mul_pow_ad_rhs(463, 451, A::mul(s.ad_value(452), s.ad_value(460)), s.ad_value(453));
            s.store_mul3_lhs(464, 463, 459, 462);
            s.store_scale(465, 463, 2.0);
        }

        s.store_offset(551, 508, s.v[370]);

        s.store_offset(552, 509, s.v[370]);

        s.store_offset(553, 510, s.v[370]);

        s.store_scale_ad(554, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(548), s.v[363], s.ad_value(551), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));

        s.store_scale_ad(555, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(549), s.v[363], s.ad_value(552), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));

        s.store_scale_ad(556, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(550), s.v[363], s.ad_value(553), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));

        s.store_mul3_lhs(557, 511, 554, 554);

        s.store_mul3_lhs(558, 512, 555, 555);

        s.store_mul3_lhs(559, 513, 556, 556);

        s.store_sub_scaled_inputs_ad_rhs(560, 502, s.v[360], A::ln(s.ad_value(554)), (2.0 * s.v[364]));

        s.store_sub_scaled_inputs_ad_rhs(561, 503, s.v[360], A::ln(s.ad_value(555)), (2.0 * s.v[364]));

        s.store_sub_scaled_inputs_ad_rhs(562, 504, s.v[360], A::ln(s.ad_value(556)), (2.0 * s.v[364]));

        s.store_add_scaled_inputs_ad_rhs(563, 560, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(560), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);

        s.store_add_scaled_inputs_ad_rhs(564, 561, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(561), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);

        s.store_add_scaled_inputs_ad_rhs(565, 562, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(562), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);

        s.store_div_from_scalar(566, 1.0, 563);

        s.store_div_from_scalar(567, 1.0, 564);

        s.store_div_from_scalar(568, 1.0, 565);

        s.store_mul_pow_ad_rhs(575, 499, A::mul(s.ad_value(502), s.ad_value(566)), s.ad_value(505));

        s.store_mul_pow_ad_rhs(576, 500, A::mul(s.ad_value(503), s.ad_value(567)), s.ad_value(506));

        s.store_mul_pow_ad_rhs(577, 501, A::mul(s.ad_value(504), s.ad_value(568)), s.ad_value(507));

        s.store_mul3_lhs(578, 575, 563, 572);

        s.store_mul3_lhs(579, 576, 564, 573);

        s.store_mul3_lhs(580, 577, 565, 574);

        s.store_scale(581, 575, 2.0);

        s.store_scale(582, 576, 2.0);

        s.store_scale(583, 577, 2.0);

        s.store_max_with_scalar_ad(593, A::scale(s.ad_value(551), 0.5), s.v[364]);

        s.store_max_with_scalar_ad(594, A::scale(s.ad_value(552), 0.5), s.v[364]);

        s.store_max_with_scalar_ad(595, A::scale(s.ad_value(553), 0.5), s.v[364]);

        s.store_scale(596, 593, s.v[365]);

        s.store_scale(597, 594, s.v[365]);

        s.store_scale(598, 595, s.v[365]);

        s.store_scaled_sqrt_ad(599, A::mul3_scaled_output(s.ad_value(522), A::square(s.ad_value(593)), s.ad_value(593), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(600, A::mul3_scaled_output(s.ad_value(523), A::square(s.ad_value(594)), s.ad_value(594), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(601, A::mul3_scaled_output(s.ad_value(524), A::square(s.ad_value(595)), s.ad_value(595), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_mul_scale_offset_rhs(602, 528, 531, (s.v[359] - s.v[358]), 1.0);

        s.store_mul_scale_offset_rhs(603, 529, 532, (s.v[359] - s.v[358]), 1.0);

        s.store_mul_scale_offset_rhs(604, 530, 533, (s.v[359] - s.v[358]), 1.0);

        if (!(s.v[602] > 0.0)) {
            s.store_scalar(602, 0.0);
        }

        if (!(s.v[603] > 0.0)) {
            s.store_scalar(603, 0.0);
        }

        if (!(s.v[604] > 0.0)) {
            s.store_scalar(604, 0.0);
        }

        s.b[1011] = (s.v[629] == 1.0);
        s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });

        if s.b[1011] {
            s.store_offset(618, 617, s.v[370]);
            s.store_scale_ad(620, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(619), s.v[363], s.ad_value(618), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(621, 615, s.v[360], A::ln(s.ad_value(620)), (2.0 * s.v[364]));
            s.store_add_scaled_inputs_ad_rhs(622, 621, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(621), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);
            s.store_div_from_scalar(623, 1.0, 622);
            s.store_mul_pow_ad_rhs(626, 614, A::mul(s.ad_value(615), s.ad_value(623)), s.ad_value(616));
            s.store_mul3_lhs(627, 626, 622, 625);
            s.store_scale(628, 626, 2.0);
        }

        s.store_scalar(1, 1.0);

        s.store_scalar(2, 1.0);

        s.store_scalar(306, 0.0);

        s.store_scalar(307, 0.0);

        s.store_scalar(3, p.p0);

        s.store_scalar(4, p.p1);

        s.store_scalar(5, p.p2);

        s.store_scalar(6, p.p3);

        s.store_scalar(7, p.p4);

        s.store_scalar(8, p.p8);

        s.store_scalar(640, p.p19);

        s.store_scalar(641, p.p20);

        s.store_scalar(642, p.p21);

        s.store_scalar(667, p.p22);

        s.store_scalar(668, p.p23);

        s.store_scalar(669, p.p24);

        s.store_scalar(643, p.p25);

        s.store_scalar(644, p.p26);

        s.store_scalar(670, p.p27);

        s.store_scalar(671, p.p28);

        s.store_scalar(10, p.p14);

        s.b[1012] = (p.p39 > 0.0);
        s.store_scalar(1012, if s.b[1012] { 1.0 } else { 0.0 });

        if s.b[1012] {
            s.store_scalar(1, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));
        }

        if s.b[1012] {
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

        s.store_scalar(302, (1e-6 / s.v[3]));

        s.store_div_from_scalar(303, 1e-6, 4);

        s.store_offset_scaled(304, 303, ((p.p188) * ((p.p186 * (1.0 + (p.p187 * s.v[302]))))), (p.p186 * (1.0 + (p.p187 * s.v[302]))));

        s.store_offset_scaled(305, 303, ((p.p192) * ((p.p190 * (1.0 + (p.p191 * s.v[302]))))), (p.p190 * (1.0 + (p.p191 * s.v[302]))));

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (((s.v[3] + s.v[304]) - (2.0 * p.p189)) > 1e-9) {
            s.store_offset(306, 304, ((s.v[3]) + ((-(2.0 * p.p189)))));
        } else {
            s.store_scalar(306, 1e-9);
        }

        if (((s.v[4] + s.v[305]) - (2.0 * p.p193)) > 1e-9) {
            s.store_offset_add(307, 4, 305, (-(2.0 * p.p193)));
        } else {
            s.store_scalar(307, 1e-9);
        }

        s.store_div_from_scalar(308, 1e-6, 306);

        s.store_square(309, 308);

        s.store_div_from_scalar(310, 1e-6, 307);

        s.store_div_from_scalar(311, 1.0, 310);

        s.store_mul(312, 308, 310);

        s.store_div_from_scalar(313, 1.0, 312);

        if ((((s.v[3] + s.v[304]) - (2.0 * p.p189)) + p.p194) > 1e-9) {
            s.store_offset(314, 304, ((((s.v[3]) + ((-(2.0 * p.p189))))) + (p.p194)));
        } else {
            s.store_scalar(314, 1e-9);
        }

        if ((((s.v[4] + s.v[305]) - (2.0 * p.p193)) + p.p195) > 1e-9) {
            s.store_offset_add(315, 4, 305, (((-(2.0 * p.p193))) + (p.p195)));
        } else {
            s.store_scalar(315, 1e-9);
        }

        s.store_scale(316, 315, 1000000.0);

        if (((s.v[3] + s.v[304]) + p.p194) > 1e-9) {
            s.store_offset(317, 304, ((s.v[3]) + (p.p194)));
        } else {
            s.store_scalar(317, 1e-9);
        }

        if (((s.v[4] + s.v[305]) + p.p195) > 1e-9) {
            s.store_offset_add(318, 4, 305, p.p195);
        } else {
            s.store_scalar(318, 1e-9);
        }

        s.store_scale(319, 317, 1000000.0);

        s.store_scale(320, 318, 1000000.0);

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

        s.b[1013] = param_given[121];
        s.store_scalar(1013, if s.b[1013] { 1.0 } else { 0.0 });

        if s.b[1013] {
            s.store_scalar(105, p.p121);
        }

        s.store_scalar(106, p.p120);

        s.b[1014] = param_given[122];
        s.store_scalar(1014, if s.b[1014] { 1.0 } else { 0.0 });

        if s.b[1014] {
            s.store_scalar(106, p.p122);
        }

        s.copy_ad(107, 105);

        s.b[1015] = param_given[123];
        s.store_scalar(1015, if s.b[1015] { 1.0 } else { 0.0 });

        if s.b[1015] {
            s.store_scalar(107, p.p123);
        }

        s.copy_ad(108, 106);

        s.b[1016] = param_given[124];
        s.store_scalar(1016, if s.b[1016] { 1.0 } else { 0.0 });

        if s.b[1016] {
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

        s.b[1017] = param_given[137];
        s.store_scalar(1017, if s.b[1017] { 1.0 } else { 0.0 });

        if s.b[1017] {
            s.store_scalar(121, p.p137);
        }

        s.store_scalar(122, p.p103);

        s.b[1018] = param_given[138];
        s.store_scalar(1018, if s.b[1018] { 1.0 } else { 0.0 });

        if s.b[1018] {
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

        s.b[1019] = (p.p39 > 0.0);
        s.store_scalar(1019, if s.b[1019] { 1.0 } else { 0.0 });

        if s.b[1019] {
            s.store_add_scaled_inputs3_offset_mixed_aii(40, A::powf(s.ad_value(308), p.p198), p.p197, 310, p.p199, 312, p.p200, p.p196);
            s.store_add_scaled_inputs3_offset_indices(41, 308, p.p202, 310, p.p203, 312, p.p204, p.p201);
            s.store_scalar(42, p.p205);
            s.store_scalar(43, p.p206);
            s.store_scalar(44, p.p207);
        }

        if s.b[1019] {
            s.store_scale_ad(325, {
                if ((1.0 + ((p.p209 * s.v[310]) * (((1.0 + (s.v[307] / p.p210))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(310), p.p209, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p210), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p208);
        }

        if s.b[1019] {
            s.store_scale_ad(326, {
                if ((1.0 + ((p.p212 * s.v[310]) * (((1.0 + (s.v[307] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(310), p.p212, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p213), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p211);
        }

        if s.b[1019] {
            s.store_scale_ad(327, {
                if ((1.0 + ((p.p215 * s.v[310]) * (((1.0 + (s.v[307] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(310), p.p215, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p213), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p214);
        }

        s.b[1020] = (s.v[306] > (2.0 * s.v[327]));
        s.store_scalar(1020, if s.b[1020] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1020]) {
            s.store_scalar(328, 75000000000.0);
            s.store_sub_ad(329, A::sqrt(A::add_scaled_inputs(s.ad_value(325), 1.0, s.ad_value(326), 0.5)), A::sqrt(s.ad_value(325)));
            s.store_add_scaled_product_mixed_aia(330, A::sqrt(s.ad_value(325)), 1.0, 328, A::ln(A::offset(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(327), 2.0, s.ad_value(306), 1.0), A::exp(A::div(s.ad_value(329), s.ad_value(328))), (-1.0)), 1.0)), 1.0);
            s.store_square(330, 330);
        }

        s.b[1021] = (s.v[306] >= s.v[327]);
        s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });

        if ((s.b[1019] && (!s.b[1020])) && s.b[1021]) {
            s.store_add_ad_rhs(330, 325, A::div_scaled_product(s.ad_value(326), s.ad_value(327), 1.0, s.ad_value(306), 1.0));
        }

        if ((s.b[1019] && (!s.b[1020])) && (!s.b[1021])) {
            s.store_add_ad_rhs(330, 325, A::mul_sub_from_scalar_rhs(s.ad_value(326), 2.0, A::div(s.ad_value(306), s.ad_value(327))));
        }

        if s.b[1019] {
            s.store_mul_sub_scaled_inputs_rhs(45, 330, A::sub_from_scalar(1.0, A::scale(s.ad_value(308), p.p216)), 1.0, s.ad_value(309), p.p217);
            s.store_add_scaled_inputs3_offset_mixed_aii(46, A::powf(s.ad_value(308), p.p220), p.p219, 310, p.p221, 312, p.p222, p.p218);
            s.store_scalar(47, p.p223);
            s.store_scalar(48, p.p224);
            s.store_add_scaled_inputs3_offset_mixed_aii(49, A::powf(s.ad_value(308), p.p227), p.p226, 310, p.p228, 312, p.p229, p.p225);
        }

        if s.b[1019] {
            s.store_scale_ad(50, {
                if (1e-6 > (1.0 + (p.p231 * s.v[308]))) {
                    A::constant(1e-6)
                } else {
                    A::scale_offset(s.ad_value(308), p.p231, 1.0)
                }
            }, p.p230);
        }

        if s.b[1019] {
            s.store_scalar(55, p.p232);
            s.store_scalar(56, p.p233);
            s.store_scalar(57, p.p236);
            s.store_scalar(58, p.p237);
            s.store_mul3_ad(51, A::scale_offset(A::powf(s.ad_value(308), p.p240), p.p239, p.p238), A::scale_offset(s.ad_value(310), p.p241, 1.0), A::scale_offset(s.ad_value(312), p.p242, 1.0));
            s.store_scalar(52, p.p244);
            s.store_scalar(53, p.p243);
            s.store_scalar(54, p.p245);
            s.store_scaled_mul_scale_offset_rhs_ad(62, A::powf(s.ad_value(308), p.p247), 310, p.p248, 1.0, p.p246);
            s.store_scalar(63, p.p250);
            s.store_scalar(64, p.p249);
            s.store_scaled_mul_scale_offset_rhs_ad(59, A::powf(s.ad_value(308), p.p252), 310, p.p253, 1.0, p.p251);
            s.store_scalar(60, p.p255);
            s.store_scalar(61, p.p254);
            s.store_offset_scaled(331, 310, ((p.p258) * (p.p257)), p.p257);
        }

        if s.b[1019] {
            s.store_scale_ad(332, {
                if ((1.0 + (p.p260 * s.v[310])) > 0.001) {
                    A::scale_offset(s.ad_value(310), p.p260, 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p259);
        }

        if s.b[1019] {
            s.store_add_ad(333, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(331), s.ad_value(332), 1.0, s.ad_value(306), 1.0), 1.0, A::exp_div_scaled_inputs(s.ad_value(306), -1.0, s.ad_value(332), 1.0)), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p261 * p.p262), s.ad_value(306)), 1.0, A::exp_scaled_input(s.ad_value(306), (-1.0 / (p.p262)))));
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1019] {
            if (s.v[333] > 1e-15) {
            } else {
                s.store_scalar(333, 1e-15);
            }
        }

        if s.b[1019] {
            s.store_add_scaled_product_mixed_aia(334, A::scale_offset(s.ad_value(310), p.p263, 1.0), 1.0, 310, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p265), 1.0)), p.p264);
            s.store_mul_div_scaled_inputs_mixed_iia(65, 334, 307, p.p256, A::mul(s.ad_value(333), s.ad_value(306)), 1.0);
            s.store_add_scaled_inputs3_offset_indices(66, 308, p.p267, 310, p.p268, 312, p.p269, p.p266);
            s.store_offset_scaled(67, 310, ((p.p271) * (p.p270)), p.p270);
            s.store_scalar(68, p.p272);
            s.store_scalar(69, p.p273);
            s.store_scalar(70, p.p274);
            s.store_mul3_ad(71, A::scale_offset(A::powf(s.ad_value(308), p.p277), p.p276, p.p275), A::scale_offset(s.ad_value(310), p.p278, 1.0), A::scale_offset(s.ad_value(312), p.p279, 1.0));
            s.store_scalar(72, p.p280);
            s.store_scalar(73, p.p281);
            s.store_scalar(74, p.p282);
            s.store_mul3_ad_scaled_output(75, A::scale_offset(s.ad_value(308), p.p284, 1.0), A::scale_offset(s.ad_value(310), p.p285, 1.0), A::scale_offset(s.ad_value(312), p.p286, 1.0), p.p283);
            s.store_scalar(76, p.p287);
            s.store_scalar(77, p.p288);
            s.store_mul_scale_offset_rhs(78, 310, 310, ((p.p290) * (p.p289)), p.p289);
            s.store_scalar(79, p.p291);
            s.store_scalar(80, p.p292);
            s.store_scalar(81, p.p293);
            s.store_mul3_ad(82, A::offset(A::mul(A::div_scaled_inputs(s.ad_value(334), p.p295, s.ad_value(333), 1.0), A::powf(s.ad_value(308), p.p296)), p.p294), A::scale_offset(s.ad_value(310), p.p297, 1.0), A::scale_offset(s.ad_value(312), p.p298, 1.0));
            s.store_add_scaled_inputs3_offset_indices(83, 308, p.p300, 310, p.p301, 312, p.p302, p.p299);
            s.store_scalar(84, p.p303);
            s.store_scalar(85, p.p304);
            s.store_scalar(86, p.p305);
            s.store_div_from_scalar_offset_scaled_input(87, p.p306, 308, p.p307, 1.0);
            s.store_scaled_mul_scale_offset_rhs_ad(88, A::powf(s.ad_value(308), p.p309), 310, p.p310, 1.0, p.p308);
            s.store_powf(335, 308, p.p312);
            s.store_div_scaled_product_offset_denominator(89, s.ad_value(335), A::scale_offset(s.ad_value(310), p.p314, 1.0), p.p311, A::mul_scaled_lhs(s.ad_value(308), p.p313, s.ad_value(335)), 1.0, 1.0);
            s.store_powf(335, 308, p.p316);
            s.store_div_scaled_product_offset_denominator(90, s.ad_value(335), A::scale_offset(s.ad_value(310), p.p318, 1.0), p.p315, A::mul_scaled_lhs(s.ad_value(308), p.p317, s.ad_value(335)), 1.0, 1.0);
            s.store_scalar(91, p.p319);
            s.store_scaled_mul_scale_offset_inputs(92, 308, p.p321, 1.0, 310, p.p322, 1.0, p.p320);
            s.store_scalar(93, p.p323);
            s.store_scalar(94, p.p324);
            s.store_scaled_mul_scale_offset_inputs(95, 308, p.p326, 1.0, 310, p.p327, 1.0, p.p325);
            s.store_scaled_mul_scale_offset_inputs(96, 308, p.p329, 1.0, 310, p.p330, 1.0, p.p328);
            s.store_scalar(97, p.p331);
            s.store_scalar(98, p.p332);
            s.store_div_from_scalar(99, p.p333, 312);
            s.store_div_from_scalar_scaled_input(100, (p.p334 * p.p234), 310, 1e-6);
            s.store_div_from_scalar_scaled_input(101, (p.p335 * p.p235), 310, 1e-6);
            s.store_scalar(102, p.p336);
            s.store_scalar(103, p.p337);
            s.store_scalar(104, p.p338);
            s.store_scalar(105, p.p337);
        }

        s.b[1022] = param_given[339];
        s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1022]) {
            s.store_scalar(105, p.p339);
        }

        if s.b[1019] {
            s.store_scalar(106, p.p338);
        }

        s.b[1023] = param_given[340];
        s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1023]) {
            s.store_scalar(106, p.p340);
        }

        if s.b[1019] {
            s.copy_ad(107, 105);
        }

        s.b[1024] = param_given[341];
        s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1024]) {
            s.store_scalar(107, p.p341);
        }

        if s.b[1019] {
            s.copy_ad(108, 106);
        }

        s.b[1025] = param_given[342];
        s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1025]) {
            s.store_scalar(108, p.p342);
        }

        if s.b[1019] {
            s.store_scalar(109, p.p343);
            s.store_div_from_scalar_scaled_input(110, (p.p344 * p.p234), 310, 1e-6);
            s.store_div_from_scalar_scaled_input(111, (p.p345 * p.p235), 310, 1e-6);
            s.store_scalar(112, p.p346);
            s.store_scalar(113, p.p347);
            s.store_scalar(114, p.p348);
            s.store_scalar(115, p.p349);
            s.store_scalar(116, p.p350);
            s.store_scalar(117, p.p351);
            s.store_scaled_mul(118, 315, 314, ((8.8541878176e-12 * p.p207) * 1.0 / (p.p206)));
            s.store_scale(125, 315, ((8.8541878176e-12 * p.p207) * (p.p234 * 1.0 / (p.p232))));
            s.store_scale(126, 315, ((8.8541878176e-12 * p.p207) * (p.p235 * 1.0 / (p.p233))));
            s.store_add_scaled_inputs3_offset_mixed_aii(119, A::powf(s.ad_value(308), p.p354), p.p353, 310, p.p355, 312, p.p356, p.p352);
            s.store_add_scaled_inputs3_offset_indices(120, 308, p.p358, 310, p.p359, 312, p.p360, p.p357);
            s.store_scalar(32, p.p294);
        }

        s.b[1026] = param_given[361];
        s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1026]) {
            s.store_scalar(32, p.p361);
        }

        if s.b[1019] {
            s.store_scalar(33, p.p295);
        }

        s.b[1027] = param_given[362];
        s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1027]) {
            s.store_scalar(33, p.p362);
        }

        if s.b[1019] {
            s.store_scalar(34, p.p296);
        }

        s.b[1028] = param_given[363];
        s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1028]) {
            s.store_scalar(34, p.p363);
        }

        if s.b[1019] {
            s.store_scalar(35, p.p297);
        }

        s.b[1029] = param_given[364];
        s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1029]) {
            s.store_scalar(35, p.p364);
        }

        if s.b[1019] {
            s.store_scalar(36, p.p298);
        }

        s.b[1030] = param_given[365];
        s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1030]) {
            s.store_scalar(36, p.p365);
        }

        if s.b[1019] {
            s.store_mul3_ad(121, A::add_scaled_product(s.ad_value(32), 1.0, A::div_scaled_product(s.ad_value(33), s.ad_value(334), 1.0, s.ad_value(333), 1.0), A::pow(s.ad_value(308), s.ad_value(34)), 1.0), A::offset(A::mul(s.ad_value(35), s.ad_value(310)), 1.0), A::offset(A::mul(s.ad_value(36), s.ad_value(312)), 1.0));
            s.store_scalar(37, p.p306);
        }

        s.b[1031] = param_given[366];
        s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1031]) {
            s.store_scalar(37, p.p366);
        }

        if s.b[1019] {
            s.store_scalar(38, p.p307);
        }

        s.b[1032] = param_given[367];
        s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1032]) {
            s.store_scalar(38, p.p367);
        }

        if s.b[1019] {
            s.store_div_scaled_value_offset_denominator(122, s.ad_value(37), 1.0, A::mul(s.ad_value(38), s.ad_value(308)), 1.0, 1.0);
            s.store_scaled_mul_scale_offset_rhs_ad(123, A::powf(s.ad_value(308), p.p369), 310, p.p370, 1.0, p.p368);
            s.store_powf(335, 308, p.p372);
            s.store_div_scaled_product_offset_denominator(124, s.ad_value(335), A::scale_offset(s.ad_value(310), p.p374, 1.0), p.p371, A::mul_scaled_lhs(s.ad_value(308), p.p373, s.ad_value(335)), 1.0, 1.0);
            s.store_scalar(127, p.p375);
            s.store_scalar(128, p.p376);
            s.store_scalar(129, p.p377);
            s.store_scale(130, 319, p.p378);
            s.store_scale(131, 316, p.p379);
            s.store_scale(132, 316, p.p380);
            s.store_scalar(133, p.p381);
            s.store_scalar(134, p.p382);
            s.store_scalar(135, p.p383);
            s.store_scalar(136, p.p384);
            s.store_scale(137, 320, p.p385);
            s.store_scale(138, 320, p.p386);
            s.store_sub_from_scalar_ad(1001, 1.0, A::div_from_scalar((2.0 * p.p393), s.ad_value(306)));
            s.store_scalar(139, p.p387);
            s.store_offset_scaled(338, 307, p.p396, (2.0 * p.p395));
            s.store_scalar(145, p.p397);
            s.store_add_scaled_inputs3_offset_indices(146, 308, p.p399, 310, p.p400, 312, p.p401, p.p398);
            s.store_add_scaled_inputs3_offset_mixed_aii(147, A::powf(s.ad_value(308), p.p404), p.p403, 310, p.p405, 312, p.p406, p.p402);
            s.store_mul3_ad_scaled_output(148, A::scale_offset(A::powf(s.ad_value(308), p.p409), p.p408, 1.0), A::scale_offset(s.ad_value(310), p.p410, 1.0), A::scale_offset(s.ad_value(312), p.p411, 1.0), p.p407);
            s.store_offset_scaled_ad(149, A::powf(s.ad_value(308), p.p414), p.p413, p.p412);
            s.store_offset_ad(341, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p415 * p.p416), s.ad_value(306)), 1.0, A::exp_scaled_input(s.ad_value(306), (-1.0 / (p.p416)))), 1.0);
        }

        if s.b[1019] {
            if (s.v[341] > 1e-15) {
            } else {
                s.store_scalar(341, 1e-15);
            }
        }

        if s.b[1019] {
            s.store_mul_div_scaled_inputs_mixed_aia(150, A::scale_offset(s.ad_value(310), p.p417, 1.0), 338, p.p256, A::mul(s.ad_value(341), s.ad_value(306)), 1.0);
            s.store_add_scaled_inputs3_offset_indices(151, 308, p.p419, 310, p.p420, 312, p.p421, p.p418);
            s.store_scaled_mul_scale_offset_rhs_ad(152, A::powf(s.ad_value(308), p.p423), 310, p.p424, 1.0, p.p422);
            s.store_scalar(153, p.p425);
            s.store_scalar(154, p.p426);
            s.store_scaled_mul_scale_offset_rhs_ad(155, A::powf(s.ad_value(308), p.p428), 310, p.p429, 1.0, p.p427);
            s.store_scalar(156, p.p431);
            s.store_scalar(157, p.p430);
            s.store_add_scaled_inputs3_offset_indices(342, 308, p.p808, 310, p.p809, 312, p.p810, p.p807);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1019] {
            s.store_add_scaled_inputs3_offset_indices(343, 308, p.p812, 310, p.p813, 312, p.p814, p.p811);
        }

        s.b[1034] = (((param_given[448] || param_given[449]) || param_given[450]) || param_given[451]);
        s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1034]) {
            s.store_add_scaled_inputs3_offset_indices(40, 308, p.p449, 310, p.p450, 312, p.p451, p.p448);
        }

        s.b[1035] = (((param_given[452] || param_given[453]) || param_given[454]) || param_given[455]);
        s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1035]) {
            s.store_add_scaled_inputs3_offset_indices(41, 308, p.p453, 310, p.p454, 312, p.p455, p.p452);
        }

        s.b[1036] = (((param_given[456] || param_given[457]) || param_given[458]) || param_given[459]);
        s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1036]) {
            s.store_add_scaled_inputs3_offset_indices(45, 308, p.p457, 310, p.p458, 312, p.p459, p.p456);
        }

        s.b[1037] = (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]);
        s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1037]) {
            s.store_add_scaled_inputs3_offset_indices(46, 308, p.p461, 310, p.p462, 312, p.p463, p.p460);
        }

        s.b[1038] = (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]);
        s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1038]) {
            s.store_add_scaled_inputs3_offset_indices(47, 308, p.p465, 310, p.p466, 312, p.p467, p.p464);
        }

        s.b[1039] = (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]);
        s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1039]) {
            s.store_add_scaled_inputs3_offset_indices(49, 308, p.p469, 310, p.p470, 312, p.p471, p.p468);
        }

        s.b[1040] = (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]);
        s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1040]) {
            s.store_add_scaled_inputs3_offset_indices(50, 308, p.p473, 310, p.p474, 312, p.p475, p.p472);
        }

        s.b[1041] = (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]);
        s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1041]) {
            s.store_add_scaled_inputs3_offset_indices(57, 308, p.p477, 310, p.p478, 312, p.p479, p.p476);
        }

        s.b[1042] = (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]);
        s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1042]) {
            s.store_add_scaled_inputs3_offset_indices(58, 308, p.p481, 310, p.p482, 312, p.p483, p.p480);
        }

        s.b[1043] = (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]);
        s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1043]) {
            s.store_add_scaled_inputs3_offset_indices(51, 308, p.p485, 310, p.p486, 312, p.p487, p.p484);
        }

        s.b[1044] = (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]);
        s.store_scalar(1044, if s.b[1044] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1044]) {
            s.store_add_scaled_inputs3_offset_indices(52, 308, p.p493, 310, p.p494, 312, p.p495, p.p492);
        }

        s.b[1045] = (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]);
        s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1045]) {
            s.store_add_scaled_inputs3_offset_indices(53, 308, p.p489, 310, p.p490, 312, p.p491, p.p488);
        }

        s.b[1046] = (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]);
        s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1046]) {
            s.store_add_scaled_inputs3_offset_indices(54, 308, p.p497, 310, p.p498, 312, p.p499, p.p496);
        }

        s.b[1047] = (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]);
        s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1047]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(62, 309, s.ad_value(308), p.p501, s.ad_value(310), p.p502, s.ad_value(312), p.p503, p.p500);
        }

        s.b[1048] = (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]);
        s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1048]) {
            s.store_add_scaled_inputs3_offset_indices(63, 308, p.p509, 310, p.p510, 312, p.p511, p.p508);
        }

        s.b[1049] = (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]);
        s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1049]) {
            s.store_add_scaled_inputs3_offset_indices(64, 308, p.p505, 310, p.p506, 312, p.p507, p.p504);
        }

        s.b[1050] = (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]);
        s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1050]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(59, 309, s.ad_value(308), p.p513, s.ad_value(310), p.p514, s.ad_value(312), p.p515, p.p512);
        }

        s.b[1051] = (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]);
        s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1051]) {
            s.store_add_scaled_inputs3_offset_indices(60, 308, p.p521, 310, p.p522, 312, p.p523, p.p520);
        }

        s.b[1052] = (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]);
        s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1052]) {
            s.store_add_scaled_inputs3_offset_indices(61, 308, p.p517, 310, p.p518, 312, p.p519, p.p516);
        }

        s.b[1053] = (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]);
        s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1053]) {
            s.store_mul_div_scaled_inputs_mixed_aii(65, A::add_scaled_inputs3_offset(s.ad_value(308), p.p525, s.ad_value(310), p.p526, s.ad_value(312), p.p527, p.p524), 307, 1.0, 306, 1.0);
        }

        s.b[1054] = (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]);
        s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1054]) {
            s.store_add_scaled_inputs3_offset_indices(66, 308, p.p529, 310, p.p530, 312, p.p531, p.p528);
        }

        s.b[1055] = (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]);
        s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1055]) {
            s.store_add_scaled_inputs3_offset_indices(67, 308, p.p533, 310, p.p534, 312, p.p535, p.p532);
        }

        s.b[1056] = (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]);
        s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1056]) {
            s.store_add_scaled_inputs3_offset_indices(69, 308, p.p537, 310, p.p538, 312, p.p539, p.p536);
        }

        s.b[1057] = (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]);
        s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1057]) {
            s.store_add_scaled_inputs3_offset_indices(71, 308, p.p541, 310, p.p542, 312, p.p543, p.p540);
        }

        s.b[1058] = (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]);
        s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1058]) {
            s.store_add_scaled_inputs3_offset_indices(73, 308, p.p545, 310, p.p546, 312, p.p547, p.p544);
        }

        s.b[1059] = (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]);
        s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1059]) {
            s.store_add_scaled_inputs3_offset_indices(75, 308, p.p549, 310, p.p550, 312, p.p551, p.p548);
        }

        s.b[1060] = (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]);
        s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1060]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(78, 310, s.ad_value(308), p.p553, s.ad_value(310), p.p554, s.ad_value(312), p.p555, p.p552);
        }

        s.b[1061] = (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]);
        s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1061]) {
            s.store_add_scaled_inputs3_offset_indices(79, 308, p.p557, 310, p.p558, 312, p.p559, p.p556);
        }

        s.b[1062] = (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]);
        s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1062]) {
            s.store_add_scaled_inputs3_offset_indices(80, 308, p.p561, 310, p.p562, 312, p.p563, p.p560);
        }

        s.b[1063] = (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]);
        s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1063]) {
            s.store_add_scaled_inputs3_offset_indices(81, 308, p.p565, 310, p.p566, 312, p.p567, p.p564);
        }

        s.b[1064] = (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]);
        s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1064]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(82, 308, s.ad_value(308), p.p569, s.ad_value(310), p.p570, s.ad_value(312), p.p571, p.p568);
        }

        s.b[1065] = (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]);
        s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1065]) {
            s.store_add_scaled_inputs3_offset_indices(83, 308, p.p573, 310, p.p574, 312, p.p575, p.p572);
        }

        s.b[1066] = (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]);
        s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1066]) {
            s.store_add_scaled_inputs3_offset_indices(84, 308, p.p577, 310, p.p578, 312, p.p579, p.p576);
        }

        s.b[1067] = (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]);
        s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1067]) {
            s.store_add_scaled_inputs3_offset_indices(85, 308, p.p581, 310, p.p582, 312, p.p583, p.p580);
        }

        s.b[1068] = (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]);
        s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1068]) {
            s.store_add_scaled_inputs3_offset_indices(87, 308, p.p585, 310, p.p586, 312, p.p587, p.p584);
        }

        s.b[1069] = (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]);
        s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1069]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(88, 308, s.ad_value(308), p.p589, s.ad_value(310), p.p590, s.ad_value(312), p.p591, p.p588);
        }

        s.b[1070] = (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]);
        s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1070]) {
            s.store_add_scaled_inputs3_offset_indices(89, 308, p.p593, 310, p.p594, 312, p.p595, p.p592);
        }

        s.b[1071] = (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]);
        s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1071]) {
            s.store_add_scaled_inputs3_offset_indices(90, 308, p.p597, 310, p.p598, 312, p.p599, p.p596);
        }

        s.b[1072] = (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]);
        s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1072]) {
            s.store_add_scaled_inputs3_offset_indices(92, 308, p.p601, 310, p.p602, 312, p.p603, p.p600);
        }

        s.b[1073] = (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]);
        s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1073]) {
            s.store_add_scaled_inputs3_offset_indices(94, 308, p.p605, 310, p.p606, 312, p.p607, p.p604);
        }

        s.b[1074] = (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]);
        s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1074]) {
            s.store_add_scaled_inputs3_offset_indices(95, 308, p.p609, 310, p.p610, 312, p.p611, p.p608);
        }

        s.b[1075] = (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]);
        s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1075]) {
            s.store_add_scaled_inputs3_offset_indices(96, 308, p.p613, 310, p.p614, 312, p.p615, p.p612);
        }

        s.b[1076] = (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]);
        s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1076]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(99, 313, s.ad_value(308), p.p617, s.ad_value(310), p.p618, s.ad_value(312), p.p619, p.p616);
        }

        s.b[1077] = (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]);
        s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1077]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(100, 311, s.ad_value(308), p.p621, s.ad_value(310), p.p622, s.ad_value(312), p.p623, p.p620);
        }

        s.b[1078] = (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]);
        s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1078]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(101, 311, s.ad_value(308), p.p625, s.ad_value(310), p.p626, s.ad_value(312), p.p627, p.p624);
        }

        s.b[1079] = (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]);
        s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1079]) {
            s.store_add_scaled_inputs3_offset_indices(102, 308, p.p629, 310, p.p630, 312, p.p631, p.p628);
        }

        s.b[1080] = (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]);
        s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1080]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(110, 311, s.ad_value(308), p.p633, s.ad_value(310), p.p634, s.ad_value(312), p.p635, p.p632);
        }

        s.b[1081] = (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]);
        s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1081]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(111, 311, s.ad_value(308), p.p637, s.ad_value(310), p.p638, s.ad_value(312), p.p639, p.p636);
        }

        s.b[1082] = (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]);
        s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1082]) {
            s.store_add_scaled_inputs3_offset_indices(114, 308, p.p641, 310, p.p642, 312, p.p643, p.p640);
        }

        s.b[1083] = (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]);
        s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1083]) {
            s.store_add_scaled_inputs3_offset_indices(115, 308, p.p645, 310, p.p646, 312, p.p647, p.p644);
        }

        s.b[1084] = (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]);
        s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1084]) {
            s.store_mul_ad_affine_product_rhs(118, 316, s.ad_value(314), A::add_scaled_inputs3_offset(s.ad_value(308), p.p649, s.ad_value(310), p.p650, s.ad_value(312), p.p651, p.p648), 1.0 / (1e-6), 0.0);
        }

        s.b[1085] = (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]);
        s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1085]) {
            s.store_add_scaled_inputs3_offset_indices(119, 308, p.p653, 310, p.p654, 312, p.p655, p.p652);
        }

        s.b[1086] = (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]);
        s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1086]) {
            s.store_add_scaled_inputs3_offset_indices(120, 308, p.p657, 310, p.p658, 312, p.p659, p.p656);
        }

        s.b[1087] = (((((((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) || param_given[568]) || param_given[569]) || param_given[570]) || param_given[571]);
        s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1087]) {
            s.store_scalar(28, p.p568);
        }

        s.b[1088] = param_given[660];
        s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });

        if ((s.b[1019] && s.b[1087]) && s.b[1088]) {
            s.store_scalar(28, p.p660);
        }

        if (s.b[1019] && s.b[1087]) {
            s.store_scalar(29, p.p569);
        }

        s.b[1089] = param_given[661];
        s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });

        if ((s.b[1019] && s.b[1087]) && s.b[1089]) {
            s.store_scalar(29, p.p661);
        }

        if (s.b[1019] && s.b[1087]) {
            s.store_scalar(30, p.p570);
        }

        s.b[1090] = param_given[662];
        s.store_scalar(1090, if s.b[1090] { 1.0 } else { 0.0 });

        if ((s.b[1019] && s.b[1087]) && s.b[1090]) {
            s.store_scalar(30, p.p662);
        }

        if (s.b[1019] && s.b[1087]) {
            s.store_scalar(31, p.p571);
        }

        s.b[1091] = param_given[663];
        s.store_scalar(1091, if s.b[1091] { 1.0 } else { 0.0 });

        if ((s.b[1019] && s.b[1087]) && s.b[1091]) {
            s.store_scalar(31, p.p663);
        }

        if (s.b[1019] && s.b[1087]) {
            s.store_mul_ad_rhs(121, 308, A::add_scaled_value_products3(s.ad_value(28), 1.0, s.ad_value(29), s.ad_value(308), 1.0, s.ad_value(30), s.ad_value(310), 1.0, s.ad_value(31), s.ad_value(312), 1.0));
        }

        s.b[1092] = (((((((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) || param_given[584]) || param_given[585]) || param_given[586]) || param_given[587]);
        s.store_scalar(1092, if s.b[1092] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1092]) {
            s.store_scalar(28, p.p584);
        }

        s.b[1093] = param_given[664];
        s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });

        if ((s.b[1019] && s.b[1092]) && s.b[1093]) {
            s.store_scalar(28, p.p664);
        }

        if (s.b[1019] && s.b[1092]) {
            s.store_scalar(29, p.p585);
        }

        s.b[1094] = param_given[665];
        s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });

        if ((s.b[1019] && s.b[1092]) && s.b[1094]) {
            s.store_scalar(29, p.p665);
        }

        if (s.b[1019] && s.b[1092]) {
            s.store_scalar(30, p.p586);
        }

        s.b[1095] = param_given[666];
        s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });

        if ((s.b[1019] && s.b[1092]) && s.b[1095]) {
            s.store_scalar(30, p.p666);
        }

        if (s.b[1019] && s.b[1092]) {
            s.store_scalar(31, p.p587);
        }

        s.b[1096] = param_given[667];
        s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });

        if ((s.b[1019] && s.b[1092]) && s.b[1096]) {
            s.store_scalar(31, p.p667);
        }

        if (s.b[1019] && s.b[1092]) {
            s.store_add_scaled_value_products3_indices(122, 28, 1.0, 29, 308, 1.0, 30, 310, 1.0, 31, 312, 1.0);
        }

        s.b[1097] = (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]);
        s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1097]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(123, 308, s.ad_value(308), p.p669, s.ad_value(310), p.p670, s.ad_value(312), p.p671, p.p668);
        }

        s.b[1098] = (((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]);
        s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1098]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(124, 308, s.ad_value(308), p.p673, s.ad_value(310), p.p674, s.ad_value(312), p.p675, p.p672);
        }

        s.b[1099] = (((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]);
        s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1099]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(125, 316, s.ad_value(308), p.p677, s.ad_value(310), p.p678, s.ad_value(312), p.p679, p.p676);
        }

        s.b[1100] = (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]);
        s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1100]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(126, 316, s.ad_value(308), p.p681, s.ad_value(310), p.p682, s.ad_value(312), p.p683, p.p680);
        }

        s.b[1101] = (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]);
        s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1101]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(130, 319, s.ad_value(308), p.p685, s.ad_value(310), p.p686, s.ad_value(312), p.p687, p.p684);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1102] = (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]);
        s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1102]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(131, 316, s.ad_value(308), p.p689, s.ad_value(310), p.p690, s.ad_value(312), p.p691, p.p688);
        }

        s.b[1103] = (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]);
        s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1103]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(132, 316, s.ad_value(308), p.p693, s.ad_value(310), p.p694, s.ad_value(312), p.p695, p.p692);
        }

        s.b[1104] = (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]);
        s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1104]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 320, s.ad_value(308), p.p697, s.ad_value(310), p.p698, s.ad_value(312), p.p699, p.p696);
        }

        s.b[1105] = (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]);
        s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1105]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(138, 320, s.ad_value(308), p.p701, s.ad_value(310), p.p702, s.ad_value(312), p.p703, p.p700);
        }

        s.b[1110] = (((param_given[720] || param_given[721]) || param_given[722]) || param_given[723]);
        s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1110]) {
            s.store_add_scaled_inputs3_offset_indices(145, 308, p.p721, 310, p.p722, 312, p.p723, p.p720);
        }

        s.b[1111] = (((param_given[724] || param_given[725]) || param_given[726]) || param_given[727]);
        s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1111]) {
            s.store_add_scaled_inputs3_offset_indices(146, 308, p.p725, 310, p.p726, 312, p.p727, p.p724);
        }

        s.b[1112] = (((param_given[728] || param_given[729]) || param_given[730]) || param_given[731]);
        s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1112]) {
            s.store_add_scaled_inputs3_offset_indices(147, 308, p.p729, 310, p.p730, 312, p.p731, p.p728);
        }

        s.b[1113] = (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]);
        s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1113]) {
            s.store_add_scaled_inputs3_offset_indices(148, 308, p.p733, 310, p.p734, 312, p.p735, p.p732);
        }

        s.b[1114] = (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]);
        s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1114]) {
            s.store_add_scaled_inputs3_offset_indices(149, 308, p.p737, 310, p.p738, 312, p.p739, p.p736);
        }

        s.b[1115] = (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]);
        s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1115]) {
            s.store_mul_div_scaled_inputs_mixed_aii(150, A::add_scaled_inputs3_offset(s.ad_value(308), p.p741, s.ad_value(310), p.p742, s.ad_value(312), p.p743, p.p740), 338, 1.0, 306, 1.0);
        }

        s.b[1116] = (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]);
        s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1116]) {
            s.store_add_scaled_inputs3_offset_indices(151, 308, p.p745, 310, p.p746, 312, p.p747, p.p744);
        }

        s.b[1117] = (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]);
        s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1117]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(152, 309, s.ad_value(308), p.p749, s.ad_value(310), p.p750, s.ad_value(312), p.p751, p.p748);
        }

        s.b[1118] = (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]);
        s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1118]) {
            s.store_add_scaled_inputs3_offset_indices(153, 308, p.p753, 310, p.p754, 312, p.p755, p.p752);
        }

        s.b[1119] = (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]);
        s.store_scalar(1119, if s.b[1119] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1119]) {
            s.store_add_scaled_inputs3_offset_indices(154, 308, p.p757, 310, p.p758, 312, p.p759, p.p756);
        }

        s.b[1120] = (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]);
        s.store_scalar(1120, if s.b[1120] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1120]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(155, 309, s.ad_value(308), p.p761, s.ad_value(310), p.p762, s.ad_value(312), p.p763, p.p760);
        }

        s.b[1121] = (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]);
        s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1121]) {
            s.store_add_scaled_inputs3_offset_indices(156, 308, p.p769, 310, p.p770, 312, p.p771, p.p768);
        }

        s.b[1122] = (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]);
        s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1122]) {
            s.store_add_scaled_inputs3_offset_indices(157, 308, p.p765, 310, p.p766, 312, p.p767, p.p764);
        }

        if s.b[1019] {
            s.store_scalar(1008, 0.0);
            s.store_scalar(1009, 0.0);
            s.store_scalar(1007, 0.0);
            s.store_scalar(39, p.p788);
        }

        s.b[1126] = param_given[789];
        s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });

        if (s.b[1019] && s.b[1126]) {
            s.store_scalar(39, p.p789);
        }

        s.b[1127] = (((s.v[5] > 0.0) && (s.v[6] > 0.0)) && ((s.v[1] == 1.0) || ((s.v[1] > 1.0) && (s.v[7] > 0.0))));
        s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });

        let mut assign9160_loop_guard: usize = 0;
        while {
            let assign9160_cond_e8969: f64 = (s.v[1] - 0.5);
            let assign9160_cond_e8971: f64 = if ((s.b[1019] && s.b[1127]) && (s.v[1007] < assign9160_cond_e8969)) { 1.0 } else { 0.0 };
            assign9160_cond_e8971 != 0.0
        } {
            assign9160_loop_guard += 1;
            assert!(assign9160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1019] && s.b[1127]) {
                s.store_add_ad_rhs(1008, 1008, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1007), (s.v[7] + s.v[3]), (s.v[5] + (0.5 * s.v[3])))));
                s.store_add_ad_rhs(1009, 1009, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1007), (s.v[7] + s.v[3]), (s.v[6] + (0.5 * s.v[3])))));
                s.store_offset(1007, 1007, 1.0);
            }
        }

        if (s.b[1019] && s.b[1127]) {
            s.store_mul(992, 1008, 2);
            s.store_mul(993, 1009, 2);
            s.store_scalar(994, (1.0 / (p.p784 + (0.5 * s.v[3]))));
            s.store_scalar(995, (1.0 / (p.p785 + (0.5 * s.v[3]))));
        }

        if (s.b[1019] && s.b[1127]) {
            if ((s.v[3] + s.v[304]) > 1e-9) {
                s.store_offset(1005, 304, s.v[3]);
            } else {
                s.store_scalar(1005, 1e-9);
            }
        }

        if (s.b[1019] && s.b[1127]) {
            if (((s.v[4] + s.v[305]) + p.p786) > 1e-9) {
                s.store_offset_add(1006, 4, 305, p.p786);
            } else {
                s.store_scalar(1006, 1e-9);
            }
        }

        if (s.b[1019] && s.b[1127]) {
            s.store_div_from_scalar_powf_ad(1003, 1.0, s.ad_value(1005), p.p794);
            s.store_div_from_scalar_powf_ad(1004, 1.0, s.ad_value(1006), p.p795);
            s.store_add_scaled_inputs_product_first_ad(996, A::scale_offset(s.ad_value(1003), p.p791, 1.0), (1.0 + (p.p790 * (s.v[346] - 1.0))), 1004, (p.p792 * (1.0 + (p.p790 * (s.v[346] - 1.0)))), 1003, 1004, (p.p793 * (1.0 + (p.p790 * (s.v[346] - 1.0)))));
            s.store_div_scaled_inputs2_indices(997, 992, p.p787, 993, p.p787, 996, 1.0);
            s.store_div_scaled_inputs2_indices(998, 994, p.p787, 995, p.p787, 996, 1.0);
            s.store_div_from_scalar_powf_ad(1003, 1.0, s.ad_value(1005), p.p800);
            s.store_div_from_scalar_powf_ad(1004, 1.0, s.ad_value(1006), p.p801);
            s.store_add_scaled_inputs_product_first_ad(999, A::scale_offset(s.ad_value(1003), p.p797, 1.0), 1.0, 1004, p.p798, 1003, 1004, p.p799);
            s.store_add_scaled_inputs4_indices(1001, 992, 1.0, 993, 1.0, 994, -1.0, 995, -1.0);
            s.store_div_scaled_offset_numerator(1002, s.ad_value(997), 1.0, 1.0, A::offset(s.ad_value(998), 1.0), 1.0);
            s.store_mul(65, 65, 1002);
            s.store_div_scaled_product3_mixed_iiaa(82, 82, 1002, A::scale_offset(s.ad_value(998), p.p788, 1.0), 1.0, A::scale_offset(s.ad_value(997), p.p788, 1.0), 1.0);
            s.store_div_scaled_product3_mixed_iiaa(121, 121, 1002, A::offset(A::mul(s.ad_value(39), s.ad_value(998)), 1.0), 1.0, A::offset(A::mul(s.ad_value(39), s.ad_value(997)), 1.0), 1.0);
            s.store_mul(150, 150, 1002);
            s.store_div_scaled_inputs_indices(1002, 1001, p.p796, 999, 1.0);
            s.store_add(40, 40, 1002);
            s.store_add(145, 145, 1002);
            s.store_div_scaled_inputs_mixed_ia(1002, 1001, p.p802, A::powf(s.ad_value(999), p.p803), 1.0);
            s.store_add(62, 62, 1002);
            s.store_add(155, 155, 1002);
        }

        s.b[1128] = ((((s.v[11] > 0.0) || (s.v[12] > 0.0)) || (s.v[13] > 0.0)) || (s.v[8] > 0.0));
        s.store_scalar(1128, if s.b[1128] { 1.0 } else { 0.0 });

        s.b[1129] = (((s.v[11] == 0.0) && (s.v[12] == 0.0)) && (s.v[13] == 0.0));
        s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });

        if ((s.b[1019] && s.b[1128]) && s.b[1129]) {
            s.store_offset(1001, 4, s.v[8]);
            s.store_scalar(1002, (1.0 / p.p804));
            s.store_div_from_scalar_scaled_input(11, (p.p804 * p.p804), 1001, s.v[8]);
            s.store_div_scaled_add_product(12, A::exp_scaled_input(s.ad_value(1002), ((-10.0) * s.v[8])), ((0.1 * s.v[8]) + (0.01 * p.p804)), A::scale_offset(s.ad_value(1001), 0.1, (0.01 * p.p804)), A::exp(A::mul_scaled_lhs(s.ad_value(1001), (-10.0), s.ad_value(1002))), (-1.0), s.ad_value(4), 1.0);
            s.store_div_scaled_add_product(13, A::exp_scaled_input(s.ad_value(1002), ((-20.0) * s.v[8])), ((0.05 * s.v[8]) + (0.0025 * p.p804)), A::scale_offset(s.ad_value(1001), 0.05, (0.0025 * p.p804)), A::exp(A::mul_scaled_lhs(s.ad_value(1001), (-20.0), s.ad_value(1002))), (-1.0), s.ad_value(4), 1.0);
        }

        if (s.b[1019] && s.b[1128]) {
            s.store_add_scaled_inputs3_indices(1001, 11, 1.0, 12, p.p805, 13, p.p806);
            s.store_add_scaled_product_indices(40, 40, 1.0, 342, 1001, 1.0);
            s.store_mul_offset_ad_rhs(65, 65, A::mul(s.ad_value(343), s.ad_value(1001)), 1.0);
            s.store_add_scaled_product_indices(145, 145, 1.0, 342, 1001, 1.0);
            s.store_mul_offset_ad_rhs(150, 150, A::mul(s.ad_value(343), s.ad_value(1001)), 1.0);
        }

        s.copy_ad(172, 40);

        s.copy_ad(173, 41);

        s.copy_ad(174, 42);

        s.copy_ad(176, 43);

        s.copy_ad(177, 44);

        if (s.v[45] > 1e20) {
            if (s.v[45] < 1e26) {
                s.copy_ad(178, 45);
            } else {
                s.store_scalar(178, 1e26);
            }
        } else {
            s.store_scalar(178, 1e20);
        }

        if (s.v[46] > 0.01) {
            s.copy_ad(179, 46);
        } else {
            s.store_scalar(179, 0.01);
        }

        if (s.v[47] > 0.0) {
            s.copy_ad(180, 47);
        } else {
            s.store_scalar(180, 0.0);
        }

        s.copy_ad(181, 48);

        s.copy_ad(182, 49);

        if (s.v[50] > 0.0) {
            s.copy_ad(183, 50);
        } else {
            s.store_scalar(183, 0.0);
        }

        s.copy_ad(187, 55);

        s.copy_ad(188, 56);

        if (s.v[57] > 1e23) {
            if (s.v[57] < 1e27) {
                s.copy_ad(189, 57);
            } else {
                s.store_scalar(189, 1e27);
            }
        } else {
            s.store_scalar(189, 1e23);
        }

        if (s.v[58] > 1e23) {
            if (s.v[58] < 1e27) {
                s.copy_ad(190, 58);
            } else {
                s.store_scalar(190, 1e27);
            }
        } else {
            s.store_scalar(190, 1e23);
        }

        if (s.v[51] > 0.0) {
            s.copy_ad(184, 51);
        } else {
            s.store_scalar(184, 0.0);
        }

        if (s.v[53] > 0.0) {
            if (s.v[53] < 0.5) {
                s.copy_ad(186, 53);
            } else {
                s.store_scalar(186, 0.5);
            }
        } else {
            s.store_scalar(186, 0.0);
        }

        if (s.v[52] > 0.0) {
            if (s.v[52] < 1.0) {
                s.copy_ad(185, 52);
            } else {
                s.store_scalar(185, 1.0);
            }
        } else {
            s.store_scalar(185, 0.0);
        }

        s.copy_ad(175, 54);

        if (s.v[62] > 0.0) {
            s.copy_ad(191, 62);
        } else {
            s.store_scalar(191, 0.0);
        }

        if (s.v[64] > 0.0) {
            if (s.v[64] < 1.0) {
                s.copy_ad(193, 64);
            } else {
                s.store_scalar(193, 1.0);
            }
        } else {
            s.store_scalar(193, 0.0);
        }

        if (s.v[63] > 0.0) {
            s.copy_ad(192, 63);
        } else {
            s.store_scalar(192, 0.0);
        }

        if (s.v[59] > 0.0) {
            s.copy_ad(194, 59);
        } else {
            s.store_scalar(194, 0.0);
        }

        if (s.v[61] > 0.0) {
            if (s.v[61] < 1.0) {
                s.copy_ad(195, 61);
            } else {
                s.store_scalar(195, 1.0);
            }
        } else {
            s.store_scalar(195, 0.0);
        }

        if (s.v[60] > 0.0) {
            s.copy_ad(196, 60);
        } else {
            s.store_scalar(196, 0.0);
        }

        if (s.v[65] > 0.0) {
            s.copy_ad(197, 65);
        } else {
            s.store_scalar(197, 0.0);
        }

        s.copy_ad(198, 66);

        if (s.v[67] > 0.0) {
            s.copy_ad(199, 67);
        } else {
            s.store_scalar(199, 0.0);
        }

        s.copy_ad(200, 68);

        if (s.v[69] > 0.0) {
            s.copy_ad(201, 69);
        } else {
            s.store_scalar(201, 0.0);
        }

        s.copy_ad(202, 70);

        if (s.v[71] > 0.0) {
            s.copy_ad(203, 71);
        } else {
            s.store_scalar(203, 0.0);
        }

        s.copy_ad(204, 72);

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.v[73] > 0.0) {
            s.copy_ad(205, 73);
        } else {
            s.store_scalar(205, 0.0);
        }

        s.copy_ad(206, 74);

        if (s.v[75] > 0.0) {
            s.copy_ad(207, 75);
        } else {
            s.store_scalar(207, 0.0);
        }

        s.copy_ad(208, 76);

        s.copy_ad(209, 77);

        if (s.v[78] > 0.0) {
            s.copy_ad(210, 78);
        } else {
            s.store_scalar(210, 0.0);
        }

        s.copy_ad(211, 79);

        if (s.v[80] > (-0.5)) {
            if (s.v[80] < 1.0) {
                s.copy_ad(212, 80);
            } else {
                s.store_scalar(212, 1.0);
            }
        } else {
            s.store_scalar(212, (-0.5));
        }

        if (s.v[81] > (-0.5)) {
            s.copy_ad(213, 81);
        } else {
            s.store_scalar(213, (-0.5));
        }

        if (s.v[82] > 0.0) {
            s.copy_ad(214, 82);
        } else {
            s.store_scalar(214, 0.0);
        }

        s.copy_ad(215, 83);

        if (s.v[84] > (-0.5)) {
            if (s.v[84] < 1.0) {
                s.copy_ad(216, 84);
            } else {
                s.store_scalar(216, 1.0);
            }
        } else {
            s.store_scalar(216, (-0.5));
        }

        if (s.v[85] > (-0.5)) {
            s.copy_ad(217, 85);
        } else {
            s.store_scalar(217, (-0.5));
        }

        if (s.v[86] > 0.01) {
            s.copy_ad(218, 86);
        } else {
            s.store_scalar(218, 0.01);
        }

        if (s.v[87] > 2.0) {
            s.copy_ad(219, 87);
        } else {
            s.store_scalar(219, 2.0);
        }

        if (s.v[88] > 0.0) {
            s.copy_ad(220, 88);
        } else {
            s.store_scalar(220, 0.0);
        }

        if (s.v[89] > 0.0) {
            s.copy_ad(221, 89);
        } else {
            s.store_scalar(221, 0.0);
        }

        if (s.v[90] > 0.0) {
            s.copy_ad(222, 90);
        } else {
            s.store_scalar(222, 0.0);
        }

        s.copy_ad(223, 91);

        if (s.v[92] > 0.0) {
            s.copy_ad(224, 92);
        } else {
            s.store_scalar(224, 0.0);
        }

        s.copy_ad(225, 93);

        s.copy_ad(226, 94);

        if (s.v[95] > 0.0) {
            s.copy_ad(227, 95);
        } else {
            s.store_scalar(227, 0.0);
        }

        if (s.v[96] > 0.0) {
            s.copy_ad(228, 96);
        } else {
            s.store_scalar(228, 0.0);
        }

        if (s.v[97] > 1e-12) {
            s.copy_ad(229, 97);
        } else {
            s.store_scalar(229, 1e-12);
        }

        s.copy_ad(230, 98);

        if (s.v[99] > 0.0) {
            s.copy_ad(231, 99);
        } else {
            s.store_scalar(231, 0.0);
        }

        if (s.v[100] > 0.0) {
            s.copy_ad(232, 100);
        } else {
            s.store_scalar(232, 0.0);
        }

        if (s.v[101] > 0.0) {
            s.copy_ad(233, 101);
        } else {
            s.store_scalar(233, 0.0);
        }

        s.copy_ad(234, 102);

        s.copy_ad(235, 103);

        s.copy_ad(236, 104);

        s.copy_ad(237, 105);

        s.copy_ad(238, 106);

        s.copy_ad(239, 107);

        s.copy_ad(240, 108);

        s.copy_ad(241, 109);

        if (s.v[110] > 0.0) {
            s.copy_ad(242, 110);
        } else {
            s.store_scalar(242, 0.0);
        }

        if (s.v[111] > 0.0) {
            s.copy_ad(243, 111);
        } else {
            s.store_scalar(243, 0.0);
        }

        s.copy_ad(244, 112);

        s.copy_ad(245, 113);

        s.copy_ad(246, 114);

        s.copy_ad(247, 115);

        s.copy_ad(248, 116);

        s.copy_ad(249, 117);

        if (s.v[118] > 0.0) {
            s.copy_ad(250, 118);
        } else {
            s.store_scalar(250, 0.0);
        }

        s.copy_ad(251, 119);

        if (s.v[120] > 0.0) {
            s.copy_ad(252, 120);
        } else {
            s.store_scalar(252, 0.0);
        }

        if (s.v[121] > 0.0) {
            s.copy_ad(253, 121);
        } else {
            s.store_scalar(253, 0.0);
        }

        if (s.v[122] > 2.0) {
            s.copy_ad(254, 122);
        } else {
            s.store_scalar(254, 2.0);
        }

        s.copy_ad(255, 123);

        if (s.v[124] > 0.0) {
            s.copy_ad(256, 124);
        } else {
            s.store_scalar(256, 0.0);
        }

        if (s.v[125] > 0.0) {
            s.copy_ad(257, 125);
        } else {
            s.store_scalar(257, 0.0);
        }

        if (s.v[126] > 0.0) {
            s.copy_ad(258, 126);
        } else {
            s.store_scalar(258, 0.0);
        }

        s.copy_ad(259, 127);

        s.copy_ad(260, 128);

        s.copy_ad(261, 129);

        if (s.v[130] > 0.0) {
            s.copy_ad(262, 130);
        } else {
            s.store_scalar(262, 0.0);
        }

        if (s.v[131] > 0.0) {
            s.copy_ad(263, 131);
        } else {
            s.store_scalar(263, 0.0);
        }

        if (s.v[132] > 0.0) {
            s.copy_ad(264, 132);
        } else {
            s.store_scalar(264, 0.0);
        }

        s.copy_ad(265, 133);

        s.copy_ad(266, 134);

        s.copy_ad(267, 135);

        s.copy_ad(268, 136);

        if (s.v[137] > 0.0) {
            s.copy_ad(269, 137);
        } else {
            s.store_scalar(269, 0.0);
        }

        if (s.v[138] > 0.0) {
            s.copy_ad(270, 138);
        } else {
            s.store_scalar(270, 0.0);
        }

        s.copy_ad(271, 139);

        s.copy_ad(277, 145);

        s.copy_ad(278, 146);

        s.copy_ad(279, 147);

        if (s.v[148] > 1e20) {
            if (s.v[148] < 1e26) {
                s.copy_ad(280, 148);
            } else {
                s.store_scalar(280, 1e26);
            }
        } else {
            s.store_scalar(280, 1e20);
        }

        if (s.v[149] > 0.0) {
            s.copy_ad(281, 149);
        } else {
            s.store_scalar(281, 0.0);
        }

        if (s.v[150] > 0.0) {
            s.copy_ad(282, 150);
        } else {
            s.store_scalar(282, 0.0);
        }

        s.copy_ad(283, 151);

        if (s.v[152] > 0.0) {
            s.copy_ad(284, 152);
        } else {
            s.store_scalar(284, 0.0);
        }

        if (s.v[153] > 0.0) {
            if (s.v[153] < 1.0) {
                s.copy_ad(285, 153);
            } else {
                s.store_scalar(285, 1.0);
            }
        } else {
            s.store_scalar(285, 0.0);
        }

        if (s.v[154] > 0.0) {
            s.copy_ad(286, 154);
        } else {
            s.store_scalar(286, 0.0);
        }

        if (s.v[155] > 0.0) {
            s.copy_ad(287, 155);
        } else {
            s.store_scalar(287, 0.0);
        }

        if (s.v[157] > 0.0) {
            if (s.v[157] < 1.0) {
                s.copy_ad(289, 157);
            } else {
                s.store_scalar(289, 1.0);
            }
        } else {
            s.store_scalar(289, 0.0);
        }

        if (s.v[156] > 0.0) {
            s.copy_ad(288, 156);
        } else {
            s.store_scalar(288, 0.0);
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
            s.copy_ad(188, 187);
            s.copy_ad(190, 189);
            s.copy_ad(243, 242);
            s.copy_ad(245, 244);
            s.copy_ad(247, 246);
            s.copy_ad(249, 248);
            s.copy_ad(233, 232);
            s.copy_ad(239, 237);
            s.copy_ad(240, 238);
            s.copy_ad(258, 257);
            s.copy_ad(260, 259);
            s.copy_ad(264, 263);
            s.copy_ad(270, 269);
        }

        s.store_scale(762, 177, 8.8541878176e-12);

        s.store_div(763, 762, 176);

        s.store_square(764, 176);

        s.store_scale(765, 763, 6.241449993689894e18);

        s.store_mul(766, 252, 178);

        if (s.v[766] > 1e20) {
            if (s.v[766] < 1e26) {
            } else {
                s.store_scalar(766, 1e26);
            }
        } else {
            s.store_scalar(766, 1e20);
        }

        s.store_scalar(767, 0.0);

        s.b[1131] = (p.p51 > 0.0);
        s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });

        if s.b[1131] {
            s.store_scale_ad(767, A::powf(s.ad_value(763), 0.6666666666666666), ((0.4 * 5.951993) * p.p51));
        }

        s.b[1132] = (s.v[0] == (-1.0));
        s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });

        if (s.b[1131] && s.b[1132]) {
            s.store_scale(767, 767, (7.448711 / 5.951993));
        }

        s.store_scale(768, 763, (1e-8 * 1.0 / (s.v[761])));

        s.store_scale(769, 209, 0.5);

        s.store_scalar(770, 0.5);

        s.b[1133] = (s.v[0] == (-1.0));
        s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });

        if s.b[1133] {
            s.store_scale(769, 209, 0.3333333333333333);
            s.store_scalar(770, 0.3333333333333333);
        }

        s.store_offset_pow_from_scalar_ad(1000, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(219)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(771, s.ad_value(1000), (-1.0), A::offset(s.ad_value(1000), (-1.0)), 1.0, {
            if ((4.0 * s.v[1000]) > 0.0001) {
                A::scale(s.ad_value(1000), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_offset_pow_from_scalar_ad(1000, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(254)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(772, s.ad_value(1000), (-1.0), A::offset(s.ad_value(1000), (-1.0)), 1.0, {
            if ((4.0 * s.v[1000]) > 0.0001) {
                A::scale(s.ad_value(1000), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_div_from_scalar(773, 1.0, 223);

        s.store_div(774, 762, 187);

        s.store_div(775, 762, 188);

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_div_ad_lhs(776, A::sqrt_scaled_input(s.ad_value(189), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[349])), 774);

        s.store_div_ad_lhs(777, A::sqrt_scaled_input(s.ad_value(190), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[349])), 775);

        s.store_square(778, 776);

        s.store_square(779, 777);

        s.store_offset_div_ad(780, A::ln(A::offset(A::exp_scaled_input(s.ad_value(261), (0.005 * s.v[349])), (-1.0))), s.ad_value(261), (-((((((0.005 * s.v[349])) as f64).exp() - 1.0)) as f64).ln()));

        s.store_add_ad_lhs(781, A::ln_scaled_input(s.ad_value(776), 0.5), 780);

        s.store_add_ad_lhs(782, A::ln_scaled_input(s.ad_value(777), 0.5), 780);

        s.store_div_from_scalar(814, 1.0, 776);

        s.store_offset_scaled(815, 776, 3.1, 8.5);

        s.store_square(783, 815);

        s.store_scale(816, 815, 0.5);

        s.b[1134] = (s.v[814] < 0.06);
        s.store_scalar(1134, if s.b[1134] { 1.0 } else { 0.0 });

        if s.b[1134] {
            s.store_scale(784, 814, 64.0);
        }

        s.b[1135] = (s.v[814] <= 0.45);
        s.store_scalar(1135, if s.b[1135] { 1.0 } else { 0.0 });

        if ((!s.b[1134]) && s.b[1135]) {
            s.store_offset_scaled(784, 814, 22.0, 3.0);
        }

        s.b[1136] = (s.v[814] <= 1.6);
        s.store_scalar(1136, if s.b[1136] { 1.0 } else { 0.0 });

        if (((!s.b[1134]) && (!s.b[1135])) && s.b[1136]) {
            s.store_offset_scaled(784, 814, (-7.2), 15.5);
        }

        if (((!s.b[1134]) && (!s.b[1135])) && (!s.b[1136])) {
            s.copy_ad(784, 776);
        }

        s.store_add_scaled_inputs_product_right_ad(785, 816, 1.0, 778, 0.5, 776, A::sqrt(A::add_scaled_inputs3(s.ad_value(816), 1.0, s.ad_value(778), 0.25, s.ad_value(784), 1.0)), (-1.0));

        s.store_div_from_scalar(814, 1.0, 777);

        s.store_offset_scaled(815, 777, 3.1, 8.5);

        s.store_square(786, 815);

        s.store_scale(816, 815, 0.5);

        s.b[1137] = (s.v[814] < 0.06);
        s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });

        if s.b[1137] {
            s.store_scale(787, 814, 64.0);
        }

        s.b[1138] = (s.v[814] <= 0.45);
        s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });

        if ((!s.b[1137]) && s.b[1138]) {
            s.store_offset_scaled(787, 814, 22.0, 3.0);
        }

        s.b[1139] = (s.v[814] <= 1.6);
        s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });

        if (((!s.b[1137]) && (!s.b[1138])) && s.b[1139]) {
            s.store_offset_scaled(787, 814, (-7.2), 15.5);
        }

        if (((!s.b[1137]) && (!s.b[1138])) && (!s.b[1139])) {
            s.copy_ad(787, 777);
        }

        s.store_add_scaled_inputs_product_right_ad(788, 816, 1.0, 779, 0.5, 777, A::sqrt(A::add_scaled_inputs3(s.ad_value(816), 1.0, s.ad_value(779), 0.25, s.ad_value(787), 1.0)), (-1.0));

        s.store_add_scaled_inputs_ad(722, A::offset(s.ad_value(182), s.v[356]), 1.0, A::ln_scaled_input(A::mul(s.ad_value(178), A::powf(s.ad_value(357), (-0.75))), 4e-26), (2.0 * s.v[709]));

        if (!(s.v[722] > 0.05)) {
            s.store_scalar(722, 0.05);
        }

        s.store_div_ad_lhs(723, A::sqrt_scaled_input(s.ad_value(178), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[355])), 763);

        s.store_scalar(724, 0.0);

        s.store_scalar(725, 0.0);

        s.b[1140] = (s.v[183] > 0.0);
        s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });

        if s.b[1140] {
            s.store_div_from_scalar(726, 80000000.0, 764);
        }

        if s.b[1140] {
            if (s.v[183] > s.v[726]) {
                s.copy_ad(725, 183);
            } else {
                s.copy_ad(725, 726);
            }
        }

        if s.b[1140] {
            if (5e24 > s.v[725]) {
                s.store_scalar(725, 5e24);
            } else {
            }
        }

        if s.b[1140] {
            s.store_div_scaled_product_indices(724, 763, 763, (2.0 * s.v[709]), 725, (1.6021918e-19 * s.v[761]));
        }

        s.store_scalar(727, ((100.0 * s.v[709]) * s.v[709]));

        s.b[1141] = (p.p51 > 0.0);
        s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });

        if s.b[1141] {
            s.store_sqrt_ad(728, A::mul3_scaled_output(s.ad_value(723), s.ad_value(723), s.ad_value(722), s.v[709]));
            s.store_mul_scaled_powf_rhs(729, 767, 0.75, 728, 0.6666666666666666);
            s.store_add(722, 722, 729);
            s.store_mul_offset_ad_rhs(723, 723, A::div_scaled_inputs(s.ad_value(729), (2.0 * 0.6666666666666666), s.ad_value(728), 1.0), 1.0);
        }

        s.store_sqrt(730, 722);

        s.store_scale(731, 722, 0.95);

        s.store_scaled_mul(732, 722, 722, 0.0025);

        s.copy_ad(733, 732);

        s.store_scaled_sqrt(734, 733, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(735, 731, 0.5, 734, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(731), s.ad_value(734))), s.ad_value(732)), (-0.5));

        s.store_scaled_offset(736, 722, s.v[356], 0.5);

        s.store_sub_ad_lhs(737, A::sqrt(A::add(s.ad_value(180), s.ad_value(722))), 730);

        s.store_add_scaled_inputs3_sqrt_first_mixed_aii(738, A::add_scaled_inputs3(s.ad_value(180), 1.0, s.ad_value(181), 1.0, s.ad_value(722), 1.0), 1.0, 730, (-1.0), 737, -1.0);

        s.store_add_scaled_inputs3_offset_mixed_iia(739, 182, 1.0, 251, 1.0, A::ln_scaled_input(A::mul(s.ad_value(766), A::powf(s.ad_value(357), (-0.75))), 4e-26), (2.0 * s.v[709]), s.v[356]);

        if (!(s.v[739] > 0.05)) {
            s.store_scalar(739, 0.05);
        }

        s.store_div_ad_lhs(740, A::sqrt_scaled_input(s.ad_value(766), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[355])), 763);

        s.b[1142] = (p.p51 > 0.0);
        s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });

        if s.b[1142] {
            s.store_sqrt_ad(728, A::mul3_scaled_output(s.ad_value(740), s.ad_value(740), s.ad_value(739), s.v[709]));
            s.store_mul_scaled_powf_rhs(729, 767, 0.75, 728, 0.6666666666666666);
            s.store_add(739, 739, 729);
            s.store_mul_offset_ad_rhs(740, 740, A::div_scaled_inputs(s.ad_value(729), (2.0 * 0.6666666666666666), s.ad_value(728), 1.0), 1.0);
        }

        s.store_scale(741, 739, 0.95);

        s.store_scaled_mul(742, 739, 739, 0.0025);

        s.copy_ad(743, 742);

        s.store_scaled_sqrt(734, 743, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(744, 741, 0.5, 734, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(741), s.ad_value(734))), s.ad_value(742)), (-0.5));

        s.store_offset_add_scaled_product(694, s.ad_value(172), 1.0, s.ad_value(173), A::scale_offset(s.ad_value(174), s.v[352], 1.0), s.v[352], s.v[17]);

        s.store_exp_scaled_input(745, 175, s.v[354]);

        s.store_mul(695, 184, 745);

        s.store_scale(696, 185, 1.0 / (s.v[353]));

        s.store_exp_scaled_input(746, 198, s.v[354]);

        s.store_mul(697, 197, 746);

        s.store_scaled_mul(710, 697, 763, s.v[16]);

        s.store_mul_ad_rhs(699, 201, A::exp_scaled_input(s.ad_value(202), s.v[354]));

        s.store_exp_scaled_input(747, 200, s.v[354]);

        s.store_mul(698, 199, 747);

        s.store_mul_ad_rhs(701, 205, A::exp_scaled_input(s.ad_value(206), s.v[354]));

        s.store_exp_scaled_input(748, 204, s.v[354]);

        s.store_mul(700, 203, 748);

        s.store_exp_scaled_input(749, 208, s.v[354]);

        s.store_mul(702, 207, 749);

        s.store_exp_scaled_input(750, 211, s.v[354]);

        s.store_mul(703, 210, 750);

        s.store_scaled_mul(751, 710, 703, 2.0);

        s.store_exp_scaled_input(752, 215, s.v[354]);

        s.store_mul(714, 214, 752);

        s.store_mul(715, 253, 752);

        s.store_mul_ad_rhs(706, 225, A::exp_scaled_input(s.ad_value(226), (-s.v[354])));

        s.store_scale(713, 271, (4.0 * (1.3806505e-23 * s.v[350])));

        s.b[1143] = ((p.p46 != 0.0) && (s.v[282] > 0.0));
        s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });

        if s.b[1143] {
            s.store_offset_add_scaled_inputs_indices(707, 277, 1.0, 278, s.v[352], s.v[19]);
            s.store_exp_scaled_input(753, 283, s.v[354]);
            s.store_mul(708, 282, 753);
            s.store_scaled_mul(711, 708, 763, s.v[18]);
            s.store_offset_scaled(717, 281, ((s.v[353]) * (s.v[709])), s.v[709]);
            s.store_add_scaled_product_mixed_aia(754, A::offset(s.ad_value(279), s.v[356]), 1.0, 717, A::ln_scaled_input(A::mul(s.ad_value(280), A::powf(s.ad_value(357), (-0.75))), 4e-26), 2.0);
        }

        if s.b[1143] {
            if (s.v[754] > 0.05) {
            } else {
                s.store_scalar(754, 0.05);
            }
        }

        if s.b[1143] {
            s.store_div_ad_lhs(755, A::sqrt_scaled_input(s.ad_value(280), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[355])), 763);
            s.store_square(718, 755);
            s.store_ln(719, 718);
            s.store_scale(756, 754, 0.95);
            s.store_scaled_mul(757, 754, 754, 0.0025);
            s.copy_ad(758, 757);
            s.store_scaled_sqrt(759, 758, 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(760, 756, 0.5, 759, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(756), s.ad_value(759))), s.ad_value(757)), (-0.5));
        }

        if (!s.b[1143]) {
            s.store_scalar(707, 0.0);
            s.store_scalar(753, 1.0);
            s.store_scalar(708, 0.0);
            s.store_scalar(711, 0.0);
            s.store_scalar(717, s.v[709]);
            s.store_scalar(754, 0.0);
            s.store_scalar(755, 1.0);
            s.store_scalar(718, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(756, 0.0);
            s.store_scalar(757, 0.0);
            s.store_scalar(758, 0.0);
            s.store_scalar(759, 0.0);
            s.store_scalar(760, 0.0);
        }

        s.store_div_from_scalar(789, 1.0, 241);

        s.store_scaled_sqrt_scaled_input(790, 241, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(791, 790, 176);

        s.store_mul(792, 790, 187);

        s.store_mul(793, 790, 188);

        s.store_scalar(794, 0.0);

        s.b[1144] = (s.v[236] < 0.0);
        s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });

        if s.b[1144] {
            s.store_div_scaled_inputs_indices(794, 235, (-0.495), 236, 1.0);
        }

        s.store_scalar(795, 0.0);

        s.b[1145] = (s.v[238] < 0.0);
        s.store_scalar(1145, if s.b[1145] { 1.0 } else { 0.0 });

        if s.b[1145] {
            s.store_div_scaled_inputs_indices(795, 237, (-0.495), 238, 1.0);
        }

        s.b[1146] = (s.v[240] < 0.0);
        s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });

        if s.b[1146] {
            s.store_div_scaled_inputs_indices(796, 239, (-0.495), 240, 1.0);
        }

        s.store_pow_from_scalar_ad(797, s.v[346], s.ad_value(234));

        s.store_mul(231, 231, 797);

        s.store_mul(232, 232, 797);

        s.store_mul(233, 233, 797);

        if ((1.0 + (s.v[246] * s.v[347])) > 0.0) {
            s.store_offset_scaled(790, 246, s.v[347], 1.0);
        } else {
            s.store_scalar(790, 0.0);
        }

        s.store_mul(704, 244, 790);

        s.store_scaled_mul(800, 704, 187, 500000000.0);

        if ((1.0 + (s.v[247] * s.v[347])) > 0.0) {
            s.store_offset_scaled(790, 247, s.v[347], 1.0);
        } else {
            s.store_scalar(790, 0.0);
        }

        s.store_mul(705, 245, 790);

        s.store_scaled_mul(801, 705, 188, 500000000.0);

        s.store_scalar(802, 0.0);

        s.b[1147] = (s.v[267] > 1e-10);
        s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });

        if s.b[1147] {
            s.store_div_from_scalar(802, 0.75, 267);
        }

        s.store_square(803, 268);

        s.store_scale(20, 2, s.v[640]);

        s.store_scale(21, 2, s.v[641]);

        s.store_scale(22, 2, s.v[642]);

        s.store_scale(23, 2, s.v[667]);

        s.store_scale(24, 2, s.v[668]);

        s.store_scale(25, 2, s.v[669]);

        s.store_scalar(26, 0.0);

        s.b[1155] = (p.p43 == 3.0);
        s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });

        if s.b[1155] {
            s.store_scalar(26, 1.0);
        }

        s.copy_ad(27, 307);

        s.b[1156] = (p.p39 == 0.0);
        s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });

        if s.b[1156] {
            s.store_scalar(27, (if (s.v[10] > 0.0) { s.v[10] } else { 0.0 }));
        }

        s.b[1157] = ((p.p43 == 2.0) || (p.p43 == 3.0));
        s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });

        if s.b[1157] {
            s.store_scale(20, 2, s.v[643]);
            s.store_add_scaled_product_indices(21, 2, s.v[644], 26, 27, (-1.0));
            s.copy_ad(22, 27);
            s.store_scale(23, 2, s.v[670]);
            s.store_add_scaled_product_indices(24, 2, s.v[671], 26, 27, (-1.0));
            s.copy_ad(25, 27);
        }

        s.b[1158] = (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0));
        s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_8(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1158] {
            if (s.v[20] > 0.0) {
                s.copy_ad(640, 20);
            } else {
                s.store_scalar(640, 0.0);
            }
        }

        if s.b[1158] {
            if (s.v[21] > 0.0) {
                s.copy_ad(641, 21);
            } else {
                s.store_scalar(641, 0.0);
            }
        }

        if s.b[1158] {
            if (s.v[22] > 0.0) {
                s.copy_ad(642, 22);
            } else {
                s.store_scalar(642, 0.0);
            }
        }

        if s.b[1158] {
            if (s.v[23] > 0.0) {
                s.copy_ad(667, 23);
            } else {
                s.store_scalar(667, 0.0);
            }
        }

        if s.b[1158] {
            if (s.v[24] > 0.0) {
                s.copy_ad(668, 24);
            } else {
                s.store_scalar(668, 0.0);
            }
        }

        if s.b[1158] {
            if (s.v[25] > 0.0) {
                s.copy_ad(669, 25);
            } else {
                s.store_scalar(669, 0.0);
            }
        }

        if (!s.b[1158]) {
            s.store_scalar(640, 0.0);
            s.store_scalar(641, 0.0);
            s.store_scalar(642, 0.0);
            s.store_scalar(667, 0.0);
            s.store_scalar(668, 0.0);
            s.store_scalar(669, 0.0);
        }

        s.store_scalar(650, 0.0);

        s.store_scalar(677, 0.0);

        s.store_scalar(652, 0.0);

        s.store_scalar(679, 0.0);

        s.store_scalar(651, 0.0);

        s.store_scalar(678, 0.0);

        s.store_scalar(653, 0.0);

        s.store_scalar(680, 0.0);

        s.store_scalar(648, 0.0);

        s.store_scalar(675, 0.0);

        s.store_scalar(649, 0.0);

        s.store_scalar(676, 0.0);

        s.store_scalar(645, 1.0);

        s.store_scalar(672, 1.0);

        s.store_scalar(646, 1.0);

        s.store_scalar(673, 1.0);

        s.store_scalar(647, 1.0);

        s.store_scalar(674, 1.0);

        s.store_scalar(495, 0.0);

        s.b[1159] = (p.p43 > 0.0);
        s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });

        s.b[1160] = ((s.v[381] * s.v[640]) > 0.0);
        s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1160]) {
            s.store_scaled_ln_ad(448, A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(640), s.v[381])), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1160])) {
            s.store_scalar(448, 100000000.0);
        }

        s.b[1161] = ((s.v[382] * s.v[641]) > 0.0);
        s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1161]) {
            s.store_scaled_ln_ad(449, A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(641), s.v[382])), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1161])) {
            s.store_scalar(449, 100000000.0);
        }

        s.b[1162] = ((s.v[383] * s.v[642]) > 0.0);
        s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1162]) {
            s.store_scaled_ln_ad(450, A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(642), s.v[383])), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1162])) {
            s.store_scalar(450, 100000000.0);
        }

        if s.b[1159] {
            s.store_min3(648, 448, 449, 450);
        }

        s.b[1163] = ((((s.v[648] * s.v[365])) as f64).abs() < 230.25850929940458);
        s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1163]) {
            s.store_exp_scaled_input(649, 648, s.v[365]);
        }

        s.b[1164] = ((s.v[648] * s.v[365]) < 0.0);
        s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });

        if ((s.b[1159] && (!s.b[1163])) && s.b[1164]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(649, 1e-100, (-230.25850929940458), A::scale(s.ad_value(648), s.v[365]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((s.b[1159] && (!s.b[1163])) && (!s.b[1164])) {
            s.store_scaled_offset_ad(649, A::mul_offset_rhs(A::scale_offset(s.ad_value(648), s.v[365], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(648), s.v[365], (-230.25850929940458)), A::scale_offset(s.ad_value(648), ((s.v[365]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1159] {
            s.store_scalar(390, s.v[387]);
            s.store_scalar(391, s.v[388]);
            s.store_scalar(392, s.v[389]);
            s.store_scalar(393, p.p824);
            s.store_scalar(394, p.p825);
            s.store_scalar(395, p.p826);
            s.store_scalar(396, p.p821);
            s.store_scalar(397, p.p822);
            s.store_scalar(398, p.p823);
        }

        s.b[1165] = (s.v[640] == 0.0);
        s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1165]) {
            s.store_scalar(390, (s.v[388] + s.v[389]));
            s.store_scalar(393, (0.9 * (p.p825).min(p.p826)));
            s.store_scalar(396, (p.p822 + p.p823));
        }

        s.b[1166] = (s.v[641] == 0.0);
        s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1166]) {
            s.store_scalar(391, (s.v[387] + s.v[389]));
            s.store_scalar(394, (0.9 * (p.p824).min(p.p826)));
            s.store_scalar(397, (p.p821 + p.p823));
        }

        s.b[1167] = (s.v[642] == 0.0);
        s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1167]) {
            s.store_scalar(392, (s.v[387] + s.v[388]));
            s.store_scalar(395, (0.9 * (p.p824).min(p.p825)));
            s.store_scalar(398, (p.p821 + p.p822));
        }

        if s.b[1159] {
            s.store_min3(650, 390, 391, 392);
            s.store_scale(651, 650, 0.1);
            s.store_max3(371, 393, 394, 395);
            s.store_mul_sub_from_scalar_ad_rhs(652, 650, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(371))));
            s.store_offset_min_ad(653, A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398), (-0.05));
        }

        s.b[1168] = ((s.v[557] * s.v[667]) > 0.0);
        s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1168]) {
            s.store_scaled_ln_ad(448, A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(557), s.ad_value(667))), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1168])) {
            s.store_scalar(448, 100000000.0);
        }

        s.b[1169] = ((s.v[558] * s.v[668]) > 0.0);
        s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1169]) {
            s.store_scaled_ln_ad(449, A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(558), s.ad_value(668))), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1169])) {
            s.store_scalar(449, 100000000.0);
        }

        s.b[1170] = ((s.v[559] * s.v[669]) > 0.0);
        s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1170]) {
            s.store_scaled_ln_ad(450, A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(559), s.ad_value(669))), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1170])) {
            s.store_scalar(450, 100000000.0);
        }

        if s.b[1159] {
            s.store_min3(675, 448, 449, 450);
        }

        s.b[1171] = ((((s.v[675] * s.v[365])) as f64).abs() < 230.25850929940458);
        s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1171]) {
            s.store_exp_scaled_input(676, 675, s.v[365]);
        }

        s.b[1172] = ((s.v[675] * s.v[365]) < 0.0);
        s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });

        if ((s.b[1159] && (!s.b[1171])) && s.b[1172]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(676, 1e-100, (-230.25850929940458), A::scale(s.ad_value(675), s.v[365]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((s.b[1159] && (!s.b[1171])) && (!s.b[1172])) {
            s.store_scaled_offset_ad(676, A::mul_offset_rhs(A::scale_offset(s.ad_value(675), s.v[365], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(675), s.v[365], (-230.25850929940458)), A::scale_offset(s.ad_value(675), ((s.v[365]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1159] {
            s.copy_ad(390, 563);
            s.copy_ad(391, 564);
            s.copy_ad(392, 565);
            s.copy_ad(393, 505);
            s.copy_ad(394, 506);
            s.copy_ad(395, 507);
            s.copy_ad(396, 502);
            s.copy_ad(397, 503);
            s.copy_ad(398, 504);
        }

        s.b[1173] = (s.v[667] == 0.0);
        s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1173]) {
            s.store_add(390, 564, 565);
            s.store_scale_ad(393, A::min(s.ad_value(506), s.ad_value(507)), 0.9);
            s.store_add(396, 503, 504);
        }

        s.b[1174] = (s.v[668] == 0.0);
        s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1174]) {
            s.store_add(391, 563, 565);
            s.store_scale_ad(394, A::min(s.ad_value(505), s.ad_value(507)), 0.9);
            s.store_add(397, 502, 504);
        }

        s.b[1175] = (s.v[669] == 0.0);
        s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1175]) {
            s.store_add(392, 563, 564);
            s.store_scale_ad(395, A::min(s.ad_value(505), s.ad_value(506)), 0.9);
            s.store_add(398, 502, 503);
        }

        if s.b[1159] {
            s.store_min3(677, 390, 391, 392);
            s.store_scale(678, 677, 0.1);
            s.store_max3(371, 393, 394, 395);
            s.store_mul_sub_from_scalar_ad_rhs(679, 677, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(371))));
            s.store_offset_min_ad(680, A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398), (-0.05));
        }

        s.b[1176] = (s.v[468] == 1.0);
        s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });

        if (s.b[1159] && s.b[1176]) {
            s.store_add_scaled_inputs3_indices(495, 640, (s.v[408] * p.p922), 641, (s.v[409] * p.p922), 642, (s.v[410] * p.p922));
        }

        s.b[1511] = ((s.v[640] * s.v[408]) <= s.v[495]);
        s.store_scalar(1511, if s.b[1511] { 1.0 } else { 0.0 });

        if ((s.b[1159] && s.b[1176]) && s.b[1511]) {
            s.store_scalar(645, 0.0);
        }

        s.b[1512] = ((s.v[641] * s.v[409]) <= s.v[495]);
        s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });

        if ((s.b[1159] && s.b[1176]) && s.b[1512]) {
            s.store_scalar(646, 0.0);
        }

        s.b[1513] = ((s.v[642] * s.v[410]) <= s.v[495]);
        s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });

        if ((s.b[1159] && s.b[1176]) && s.b[1513]) {
            s.store_scalar(647, 0.0);
        }

        if (s.b[1159] && s.b[1176]) {
            s.store_mul_ad_rhs(495, 547, A::add_scaled_products3(s.ad_value(667), s.ad_value(575), 1.0, s.ad_value(668), s.ad_value(576), 1.0, s.ad_value(669), s.ad_value(577), 1.0));
        }

        s.b[1801] = ((s.v[667] * s.v[575]) <= s.v[495]);
        s.store_scalar(1801, if s.b[1801] { 1.0 } else { 0.0 });

        if ((s.b[1159] && s.b[1176]) && s.b[1801]) {
            s.store_scalar(672, 0.0);
        }

        s.b[1802] = ((s.v[668] * s.v[576]) <= s.v[495]);
        s.store_scalar(1802, if s.b[1802] { 1.0 } else { 0.0 });

        if ((s.b[1159] && s.b[1176]) && s.b[1802]) {
            s.store_scalar(673, 0.0);
        }

        s.b[1803] = ((s.v[669] * s.v[577]) <= s.v[495]);
        s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });

        if ((s.b[1159] && s.b[1176]) && s.b[1803]) {
            s.store_scalar(674, 0.0);
        }

        s.store_scalar(1919, 0.0);

        s.store_scalar(1920, 0.0);

        s.store_scalar(1921, 0.0);

        s.b[1994] = (s.v[0] == 1.0);
        s.store_scalar(1994, if s.b[1994] { 1.0 } else { 0.0 });

        if s.b[1994] {
            s.store_voltage(819, ctx, nodes, Some(5), Some(6));
            s.store_voltage(820, ctx, nodes, Some(7), Some(6));
            s.store_voltage(821, ctx, nodes, Some(6), Some(8));
        }

    }
}
