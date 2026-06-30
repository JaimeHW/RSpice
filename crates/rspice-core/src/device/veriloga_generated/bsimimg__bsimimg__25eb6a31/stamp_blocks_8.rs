#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_82(
        p: &Parameters,
        var_aigbinv_i: f64,
        var_aigbinv_i_db0: f64,
        var_aigbinv_i_db1: f64,
        var_aigbinv_i_db2: f64,
        var_aigbinv_i_db3: f64,
        var_aigbinv_i_db4: f64,
        var_aigbinv_i_dn0: f64,
        var_aigbinv_i_dn1: f64,
        var_aigbinv_i_dn2: f64,
        var_aigbinv_i_dn3: f64,
        var_aigbinv_i_dn4: f64,
        var_aigbinv_i_dn5: f64,
        var_aigbinv_i_dn6: f64,
        var_aigbinv_i_dn7: f64,
        var_aigbinv_i_dn8: f64,
        var_bigbinv_i: f64,
        var_bigbinv_i_db0: f64,
        var_bigbinv_i_db1: f64,
        var_bigbinv_i_db2: f64,
        var_bigbinv_i_db3: f64,
        var_bigbinv_i_db4: f64,
        var_bigbinv_i_dn0: f64,
        var_bigbinv_i_dn1: f64,
        var_bigbinv_i_dn2: f64,
        var_bigbinv_i_dn3: f64,
        var_bigbinv_i_dn4: f64,
        var_bigbinv_i_dn5: f64,
        var_bigbinv_i_dn6: f64,
        var_bigbinv_i_dn7: f64,
        var_bigbinv_i_dn8: f64,
        var_cigbinv_i: f64,
        var_cigbinv_i_db0: f64,
        var_cigbinv_i_db1: f64,
        var_cigbinv_i_db2: f64,
        var_cigbinv_i_db3: f64,
        var_cigbinv_i_db4: f64,
        var_cigbinv_i_dn0: f64,
        var_cigbinv_i_dn1: f64,
        var_cigbinv_i_dn2: f64,
        var_cigbinv_i_dn3: f64,
        var_cigbinv_i_dn4: f64,
        var_cigbinv_i_dn5: f64,
        var_cigbinv_i_dn6: f64,
        var_cigbinv_i_dn7: f64,
        var_cigbinv_i_dn8: f64,
        var_deltaphi1: f64,
        var_deltaphi1_db0: f64,
        var_deltaphi1_db1: f64,
        var_deltaphi1_db2: f64,
        var_deltaphi1_db3: f64,
        var_deltaphi1_db4: f64,
        var_deltaphi1_dn0: f64,
        var_deltaphi1_dn1: f64,
        var_deltaphi1_dn2: f64,
        var_deltaphi1_dn3: f64,
        var_deltaphi1_dn4: f64,
        var_deltaphi1_dn5: f64,
        var_deltaphi1_dn6: f64,
        var_deltaphi1_dn7: f64,
        var_deltaphi1_dn8: f64,
        var_eigbinv_i: f64,
        var_eigbinv_i_db0: f64,
        var_eigbinv_i_db1: f64,
        var_eigbinv_i_db2: f64,
        var_eigbinv_i_db3: f64,
        var_eigbinv_i_db4: f64,
        var_eigbinv_i_dn0: f64,
        var_eigbinv_i_dn1: f64,
        var_eigbinv_i_dn2: f64,
        var_eigbinv_i_dn3: f64,
        var_eigbinv_i_dn4: f64,
        var_eigbinv_i_dn5: f64,
        var_eigbinv_i_dn6: f64,
        var_eigbinv_i_dn7: f64,
        var_eigbinv_i_dn8: f64,
        var_guard121: f64,
        var_nigbacc_i: f64,
        var_nigbacc_i_db0: f64,
        var_nigbacc_i_db1: f64,
        var_nigbacc_i_db2: f64,
        var_nigbacc_i_db3: f64,
        var_nigbacc_i_db4: f64,
        var_nigbacc_i_dn0: f64,
        var_nigbacc_i_dn1: f64,
        var_nigbacc_i_dn2: f64,
        var_nigbacc_i_dn3: f64,
        var_nigbacc_i_dn4: f64,
        var_nigbacc_i_dn5: f64,
        var_nigbacc_i_dn6: f64,
        var_nigbacc_i_dn7: f64,
        var_nigbacc_i_dn8: f64,
        var_nigbinv_i: f64,
        var_nigbinv_i_db0: f64,
        var_nigbinv_i_db1: f64,
        var_nigbinv_i_db2: f64,
        var_nigbinv_i_db3: f64,
        var_nigbinv_i_db4: f64,
        var_nigbinv_i_dn0: f64,
        var_nigbinv_i_dn1: f64,
        var_nigbinv_i_dn2: f64,
        var_nigbinv_i_dn3: f64,
        var_nigbinv_i_dn4: f64,
        var_nigbinv_i_dn5: f64,
        var_nigbinv_i_dn6: f64,
        var_nigbinv_i_dn7: f64,
        var_nigbinv_i_dn8: f64,
        var_phib: f64,
        var_phib_db0: f64,
        var_phib_db1: f64,
        var_phib_db2: f64,
        var_phib_db3: f64,
        var_phib_db4: f64,
        var_phib_dn0: f64,
        var_phib_dn1: f64,
        var_phib_dn2: f64,
        var_phib_dn3: f64,
        var_phib_dn4: f64,
        var_phib_dn5: f64,
        var_phib_dn6: f64,
        var_phib_dn7: f64,
        var_phib_dn8: f64,
        var_qia: f64,
        var_qia_db0: f64,
        var_qia_db1: f64,
        var_qia_db2: f64,
        var_qia_db3: f64,
        var_qia_db4: f64,
        var_qia_dn0: f64,
        var_qia_dn1: f64,
        var_qia_dn2: f64,
        var_qia_dn3: f64,
        var_qia_dn4: f64,
        var_qia_dn5: f64,
        var_qia_dn6: f64,
        var_qia_dn7: f64,
        var_qia_dn8: f64,
        var_vgbg: f64,
        var_vgbg_db0: f64,
        var_vgbg_db1: f64,
        var_vgbg_db2: f64,
        var_vgbg_db3: f64,
        var_vgbg_db4: f64,
        var_vgbg_dn0: f64,
        var_vgbg_dn1: f64,
        var_vgbg_dn2: f64,
        var_vgbg_dn3: f64,
        var_vgbg_dn4: f64,
        var_vgbg_dn5: f64,
        var_vgbg_dn6: f64,
        var_vgbg_dn7: f64,
        var_vgbg_dn8: f64,
        var_vtm: f64,
        var_vtm_db0: f64,
        var_vtm_db1: f64,
        var_vtm_db2: f64,
        var_vtm_db3: f64,
        var_vtm_db4: f64,
        var_vtm_dn0: f64,
        var_vtm_dn1: f64,
        var_vtm_dn2: f64,
        var_vtm_dn3: f64,
        var_vtm_dn4: f64,
        var_vtm_dn5: f64,
        var_vtm_dn6: f64,
        var_vtm_dn7: f64,
        var_vtm_dn8: f64,
        var_guard122_slot: &mut f64,
        var_guard122_db0_slot: &mut f64,
        var_guard122_db1_slot: &mut f64,
        var_guard122_db2_slot: &mut f64,
        var_guard122_db3_slot: &mut f64,
        var_guard122_db4_slot: &mut f64,
        var_guard122_dn0_slot: &mut f64,
        var_guard122_dn1_slot: &mut f64,
        var_guard122_dn2_slot: &mut f64,
        var_guard122_dn3_slot: &mut f64,
        var_guard122_dn4_slot: &mut f64,
        var_guard122_dn5_slot: &mut f64,
        var_guard122_dn6_slot: &mut f64,
        var_guard122_dn7_slot: &mut f64,
        var_guard122_dn8_slot: &mut f64,
        var_guard122_rdb0_slot: &mut f64,
        var_guard122_rdb1_slot: &mut f64,
        var_guard122_rdb2_slot: &mut f64,
        var_guard122_rdb3_slot: &mut f64,
        var_guard122_rdb4_slot: &mut f64,
        var_guard122_rdn0_slot: &mut f64,
        var_guard122_rdn1_slot: &mut f64,
        var_guard122_rdn2_slot: &mut f64,
        var_guard122_rdn3_slot: &mut f64,
        var_guard122_rdn4_slot: &mut f64,
        var_guard122_rdn5_slot: &mut f64,
        var_guard122_rdn6_slot: &mut f64,
        var_guard122_rdn7_slot: &mut f64,
        var_guard122_rdn8_slot: &mut f64,
        var_guard122_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_db0_slot: &mut f64,
        var_t0_db1_slot: &mut f64,
        var_t0_db2_slot: &mut f64,
        var_t0_db3_slot: &mut f64,
        var_t0_db4_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rdb0_slot: &mut f64,
        var_t0_rdb1_slot: &mut f64,
        var_t0_rdb2_slot: &mut f64,
        var_t0_rdb3_slot: &mut f64,
        var_t0_rdb4_slot: &mut f64,
        var_t0_rdn0_slot: &mut f64,
        var_t0_rdn1_slot: &mut f64,
        var_t0_rdn2_slot: &mut f64,
        var_t0_rdn3_slot: &mut f64,
        var_t0_rdn4_slot: &mut f64,
        var_t0_rdn5_slot: &mut f64,
        var_t0_rdn6_slot: &mut f64,
        var_t0_rdn7_slot: &mut f64,
        var_t0_rdn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_db0_slot: &mut f64,
        var_t1_db1_slot: &mut f64,
        var_t1_db2_slot: &mut f64,
        var_t1_db3_slot: &mut f64,
        var_t1_db4_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rdb0_slot: &mut f64,
        var_t1_rdb1_slot: &mut f64,
        var_t1_rdb2_slot: &mut f64,
        var_t1_rdb3_slot: &mut f64,
        var_t1_rdb4_slot: &mut f64,
        var_t1_rdn0_slot: &mut f64,
        var_t1_rdn1_slot: &mut f64,
        var_t1_rdn2_slot: &mut f64,
        var_t1_rdn3_slot: &mut f64,
        var_t1_rdn4_slot: &mut f64,
        var_t1_rdn5_slot: &mut f64,
        var_t1_rdn6_slot: &mut f64,
        var_t1_rdn7_slot: &mut f64,
        var_t1_rdn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_db0_slot: &mut f64,
        var_t2_db1_slot: &mut f64,
        var_t2_db2_slot: &mut f64,
        var_t2_db3_slot: &mut f64,
        var_t2_db4_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rdb0_slot: &mut f64,
        var_t2_rdb1_slot: &mut f64,
        var_t2_rdb2_slot: &mut f64,
        var_t2_rdb3_slot: &mut f64,
        var_t2_rdb4_slot: &mut f64,
        var_t2_rdn0_slot: &mut f64,
        var_t2_rdn1_slot: &mut f64,
        var_t2_rdn2_slot: &mut f64,
        var_t2_rdn3_slot: &mut f64,
        var_t2_rdn4_slot: &mut f64,
        var_t2_rdn5_slot: &mut f64,
        var_t2_rdn6_slot: &mut f64,
        var_t2_rdn7_slot: &mut f64,
        var_t2_rdn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_db0_slot: &mut f64,
        var_t3_db1_slot: &mut f64,
        var_t3_db2_slot: &mut f64,
        var_t3_db3_slot: &mut f64,
        var_t3_db4_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn1_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rdb0_slot: &mut f64,
        var_t3_rdb1_slot: &mut f64,
        var_t3_rdb2_slot: &mut f64,
        var_t3_rdb3_slot: &mut f64,
        var_t3_rdb4_slot: &mut f64,
        var_t3_rdn0_slot: &mut f64,
        var_t3_rdn1_slot: &mut f64,
        var_t3_rdn2_slot: &mut f64,
        var_t3_rdn3_slot: &mut f64,
        var_t3_rdn4_slot: &mut f64,
        var_t3_rdn5_slot: &mut f64,
        var_t3_rdn6_slot: &mut f64,
        var_t3_rdn7_slot: &mut f64,
        var_t3_rdn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_db0_slot: &mut f64,
        var_t4_db1_slot: &mut f64,
        var_t4_db2_slot: &mut f64,
        var_t4_db3_slot: &mut f64,
        var_t4_db4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn1_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_rdb0_slot: &mut f64,
        var_t4_rdb1_slot: &mut f64,
        var_t4_rdb2_slot: &mut f64,
        var_t4_rdb3_slot: &mut f64,
        var_t4_rdb4_slot: &mut f64,
        var_t4_rdn0_slot: &mut f64,
        var_t4_rdn1_slot: &mut f64,
        var_t4_rdn2_slot: &mut f64,
        var_t4_rdn3_slot: &mut f64,
        var_t4_rdn4_slot: &mut f64,
        var_t4_rdn5_slot: &mut f64,
        var_t4_rdn6_slot: &mut f64,
        var_t4_rdn7_slot: &mut f64,
        var_t4_rdn8_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_db0_slot: &mut f64,
        var_t5_db1_slot: &mut f64,
        var_t5_db2_slot: &mut f64,
        var_t5_db3_slot: &mut f64,
        var_t5_db4_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn1_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_rdb0_slot: &mut f64,
        var_t5_rdb1_slot: &mut f64,
        var_t5_rdb2_slot: &mut f64,
        var_t5_rdb3_slot: &mut f64,
        var_t5_rdb4_slot: &mut f64,
        var_t5_rdn0_slot: &mut f64,
        var_t5_rdn1_slot: &mut f64,
        var_t5_rdn2_slot: &mut f64,
        var_t5_rdn3_slot: &mut f64,
        var_t5_rdn4_slot: &mut f64,
        var_t5_rdn5_slot: &mut f64,
        var_t5_rdn6_slot: &mut f64,
        var_t5_rdn7_slot: &mut f64,
        var_t5_rdn8_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_db0_slot: &mut f64,
        var_t6_db1_slot: &mut f64,
        var_t6_db2_slot: &mut f64,
        var_t6_db3_slot: &mut f64,
        var_t6_db4_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn1_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_rdb0_slot: &mut f64,
        var_t6_rdb1_slot: &mut f64,
        var_t6_rdb2_slot: &mut f64,
        var_t6_rdb3_slot: &mut f64,
        var_t6_rdb4_slot: &mut f64,
        var_t6_rdn0_slot: &mut f64,
        var_t6_rdn1_slot: &mut f64,
        var_t6_rdn2_slot: &mut f64,
        var_t6_rdn3_slot: &mut f64,
        var_t6_rdn4_slot: &mut f64,
        var_t6_rdn5_slot: &mut f64,
        var_t6_rdn6_slot: &mut f64,
        var_t6_rdn7_slot: &mut f64,
        var_t6_rdn8_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_vfbzb_slot: &mut f64,
        var_vfbzb_db0_slot: &mut f64,
        var_vfbzb_db1_slot: &mut f64,
        var_vfbzb_db2_slot: &mut f64,
        var_vfbzb_db3_slot: &mut f64,
        var_vfbzb_db4_slot: &mut f64,
        var_vfbzb_dn0_slot: &mut f64,
        var_vfbzb_dn1_slot: &mut f64,
        var_vfbzb_dn2_slot: &mut f64,
        var_vfbzb_dn3_slot: &mut f64,
        var_vfbzb_dn4_slot: &mut f64,
        var_vfbzb_dn5_slot: &mut f64,
        var_vfbzb_dn6_slot: &mut f64,
        var_vfbzb_dn7_slot: &mut f64,
        var_vfbzb_dn8_slot: &mut f64,
        var_vfbzb_rdb0_slot: &mut f64,
        var_vfbzb_rdb1_slot: &mut f64,
        var_vfbzb_rdb2_slot: &mut f64,
        var_vfbzb_rdb3_slot: &mut f64,
        var_vfbzb_rdb4_slot: &mut f64,
        var_vfbzb_rdn0_slot: &mut f64,
        var_vfbzb_rdn1_slot: &mut f64,
        var_vfbzb_rdn2_slot: &mut f64,
        var_vfbzb_rdn3_slot: &mut f64,
        var_vfbzb_rdn4_slot: &mut f64,
        var_vfbzb_rdn5_slot: &mut f64,
        var_vfbzb_rdn6_slot: &mut f64,
        var_vfbzb_rdn7_slot: &mut f64,
        var_vfbzb_rdn8_slot: &mut f64,
        var_vfbzb_rv_slot: &mut f64,
        var_voxacc_slot: &mut f64,
        var_voxacc_db0_slot: &mut f64,
        var_voxacc_db1_slot: &mut f64,
        var_voxacc_db2_slot: &mut f64,
        var_voxacc_db3_slot: &mut f64,
        var_voxacc_db4_slot: &mut f64,
        var_voxacc_dn0_slot: &mut f64,
        var_voxacc_dn1_slot: &mut f64,
        var_voxacc_dn2_slot: &mut f64,
        var_voxacc_dn3_slot: &mut f64,
        var_voxacc_dn4_slot: &mut f64,
        var_voxacc_dn5_slot: &mut f64,
        var_voxacc_dn6_slot: &mut f64,
        var_voxacc_dn7_slot: &mut f64,
        var_voxacc_dn8_slot: &mut f64,
        var_voxacc_rdb0_slot: &mut f64,
        var_voxacc_rdb1_slot: &mut f64,
        var_voxacc_rdb2_slot: &mut f64,
        var_voxacc_rdb3_slot: &mut f64,
        var_voxacc_rdb4_slot: &mut f64,
        var_voxacc_rdn0_slot: &mut f64,
        var_voxacc_rdn1_slot: &mut f64,
        var_voxacc_rdn2_slot: &mut f64,
        var_voxacc_rdn3_slot: &mut f64,
        var_voxacc_rdn4_slot: &mut f64,
        var_voxacc_rdn5_slot: &mut f64,
        var_voxacc_rdn6_slot: &mut f64,
        var_voxacc_rdn7_slot: &mut f64,
        var_voxacc_rdn8_slot: &mut f64,
        var_voxacc_rv_slot: &mut f64,
    ) {
        let mut var_guard122: f64 = *var_guard122_slot;
        let mut var_guard122_db0: f64 = *var_guard122_db0_slot;
        let mut var_guard122_db1: f64 = *var_guard122_db1_slot;
        let mut var_guard122_db2: f64 = *var_guard122_db2_slot;
        let mut var_guard122_db3: f64 = *var_guard122_db3_slot;
        let mut var_guard122_db4: f64 = *var_guard122_db4_slot;
        let mut var_guard122_dn0: f64 = *var_guard122_dn0_slot;
        let mut var_guard122_dn1: f64 = *var_guard122_dn1_slot;
        let mut var_guard122_dn2: f64 = *var_guard122_dn2_slot;
        let mut var_guard122_dn3: f64 = *var_guard122_dn3_slot;
        let mut var_guard122_dn4: f64 = *var_guard122_dn4_slot;
        let mut var_guard122_dn5: f64 = *var_guard122_dn5_slot;
        let mut var_guard122_dn6: f64 = *var_guard122_dn6_slot;
        let mut var_guard122_dn7: f64 = *var_guard122_dn7_slot;
        let mut var_guard122_dn8: f64 = *var_guard122_dn8_slot;
        let mut var_guard122_rdb0: f64 = *var_guard122_rdb0_slot;
        let mut var_guard122_rdb1: f64 = *var_guard122_rdb1_slot;
        let mut var_guard122_rdb2: f64 = *var_guard122_rdb2_slot;
        let mut var_guard122_rdb3: f64 = *var_guard122_rdb3_slot;
        let mut var_guard122_rdb4: f64 = *var_guard122_rdb4_slot;
        let mut var_guard122_rdn0: f64 = *var_guard122_rdn0_slot;
        let mut var_guard122_rdn1: f64 = *var_guard122_rdn1_slot;
        let mut var_guard122_rdn2: f64 = *var_guard122_rdn2_slot;
        let mut var_guard122_rdn3: f64 = *var_guard122_rdn3_slot;
        let mut var_guard122_rdn4: f64 = *var_guard122_rdn4_slot;
        let mut var_guard122_rdn5: f64 = *var_guard122_rdn5_slot;
        let mut var_guard122_rdn6: f64 = *var_guard122_rdn6_slot;
        let mut var_guard122_rdn7: f64 = *var_guard122_rdn7_slot;
        let mut var_guard122_rdn8: f64 = *var_guard122_rdn8_slot;
        let mut var_guard122_rv: f64 = *var_guard122_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_db0: f64 = *var_t0_db0_slot;
        let mut var_t0_db1: f64 = *var_t0_db1_slot;
        let mut var_t0_db2: f64 = *var_t0_db2_slot;
        let mut var_t0_db3: f64 = *var_t0_db3_slot;
        let mut var_t0_db4: f64 = *var_t0_db4_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rdb0: f64 = *var_t0_rdb0_slot;
        let mut var_t0_rdb1: f64 = *var_t0_rdb1_slot;
        let mut var_t0_rdb2: f64 = *var_t0_rdb2_slot;
        let mut var_t0_rdb3: f64 = *var_t0_rdb3_slot;
        let mut var_t0_rdb4: f64 = *var_t0_rdb4_slot;
        let mut var_t0_rdn0: f64 = *var_t0_rdn0_slot;
        let mut var_t0_rdn1: f64 = *var_t0_rdn1_slot;
        let mut var_t0_rdn2: f64 = *var_t0_rdn2_slot;
        let mut var_t0_rdn3: f64 = *var_t0_rdn3_slot;
        let mut var_t0_rdn4: f64 = *var_t0_rdn4_slot;
        let mut var_t0_rdn5: f64 = *var_t0_rdn5_slot;
        let mut var_t0_rdn6: f64 = *var_t0_rdn6_slot;
        let mut var_t0_rdn7: f64 = *var_t0_rdn7_slot;
        let mut var_t0_rdn8: f64 = *var_t0_rdn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_db0: f64 = *var_t1_db0_slot;
        let mut var_t1_db1: f64 = *var_t1_db1_slot;
        let mut var_t1_db2: f64 = *var_t1_db2_slot;
        let mut var_t1_db3: f64 = *var_t1_db3_slot;
        let mut var_t1_db4: f64 = *var_t1_db4_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rdb0: f64 = *var_t1_rdb0_slot;
        let mut var_t1_rdb1: f64 = *var_t1_rdb1_slot;
        let mut var_t1_rdb2: f64 = *var_t1_rdb2_slot;
        let mut var_t1_rdb3: f64 = *var_t1_rdb3_slot;
        let mut var_t1_rdb4: f64 = *var_t1_rdb4_slot;
        let mut var_t1_rdn0: f64 = *var_t1_rdn0_slot;
        let mut var_t1_rdn1: f64 = *var_t1_rdn1_slot;
        let mut var_t1_rdn2: f64 = *var_t1_rdn2_slot;
        let mut var_t1_rdn3: f64 = *var_t1_rdn3_slot;
        let mut var_t1_rdn4: f64 = *var_t1_rdn4_slot;
        let mut var_t1_rdn5: f64 = *var_t1_rdn5_slot;
        let mut var_t1_rdn6: f64 = *var_t1_rdn6_slot;
        let mut var_t1_rdn7: f64 = *var_t1_rdn7_slot;
        let mut var_t1_rdn8: f64 = *var_t1_rdn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_db0: f64 = *var_t2_db0_slot;
        let mut var_t2_db1: f64 = *var_t2_db1_slot;
        let mut var_t2_db2: f64 = *var_t2_db2_slot;
        let mut var_t2_db3: f64 = *var_t2_db3_slot;
        let mut var_t2_db4: f64 = *var_t2_db4_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rdb0: f64 = *var_t2_rdb0_slot;
        let mut var_t2_rdb1: f64 = *var_t2_rdb1_slot;
        let mut var_t2_rdb2: f64 = *var_t2_rdb2_slot;
        let mut var_t2_rdb3: f64 = *var_t2_rdb3_slot;
        let mut var_t2_rdb4: f64 = *var_t2_rdb4_slot;
        let mut var_t2_rdn0: f64 = *var_t2_rdn0_slot;
        let mut var_t2_rdn1: f64 = *var_t2_rdn1_slot;
        let mut var_t2_rdn2: f64 = *var_t2_rdn2_slot;
        let mut var_t2_rdn3: f64 = *var_t2_rdn3_slot;
        let mut var_t2_rdn4: f64 = *var_t2_rdn4_slot;
        let mut var_t2_rdn5: f64 = *var_t2_rdn5_slot;
        let mut var_t2_rdn6: f64 = *var_t2_rdn6_slot;
        let mut var_t2_rdn7: f64 = *var_t2_rdn7_slot;
        let mut var_t2_rdn8: f64 = *var_t2_rdn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_db0: f64 = *var_t3_db0_slot;
        let mut var_t3_db1: f64 = *var_t3_db1_slot;
        let mut var_t3_db2: f64 = *var_t3_db2_slot;
        let mut var_t3_db3: f64 = *var_t3_db3_slot;
        let mut var_t3_db4: f64 = *var_t3_db4_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn1: f64 = *var_t3_dn1_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rdb0: f64 = *var_t3_rdb0_slot;
        let mut var_t3_rdb1: f64 = *var_t3_rdb1_slot;
        let mut var_t3_rdb2: f64 = *var_t3_rdb2_slot;
        let mut var_t3_rdb3: f64 = *var_t3_rdb3_slot;
        let mut var_t3_rdb4: f64 = *var_t3_rdb4_slot;
        let mut var_t3_rdn0: f64 = *var_t3_rdn0_slot;
        let mut var_t3_rdn1: f64 = *var_t3_rdn1_slot;
        let mut var_t3_rdn2: f64 = *var_t3_rdn2_slot;
        let mut var_t3_rdn3: f64 = *var_t3_rdn3_slot;
        let mut var_t3_rdn4: f64 = *var_t3_rdn4_slot;
        let mut var_t3_rdn5: f64 = *var_t3_rdn5_slot;
        let mut var_t3_rdn6: f64 = *var_t3_rdn6_slot;
        let mut var_t3_rdn7: f64 = *var_t3_rdn7_slot;
        let mut var_t3_rdn8: f64 = *var_t3_rdn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_db0: f64 = *var_t4_db0_slot;
        let mut var_t4_db1: f64 = *var_t4_db1_slot;
        let mut var_t4_db2: f64 = *var_t4_db2_slot;
        let mut var_t4_db3: f64 = *var_t4_db3_slot;
        let mut var_t4_db4: f64 = *var_t4_db4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn1: f64 = *var_t4_dn1_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_rdb0: f64 = *var_t4_rdb0_slot;
        let mut var_t4_rdb1: f64 = *var_t4_rdb1_slot;
        let mut var_t4_rdb2: f64 = *var_t4_rdb2_slot;
        let mut var_t4_rdb3: f64 = *var_t4_rdb3_slot;
        let mut var_t4_rdb4: f64 = *var_t4_rdb4_slot;
        let mut var_t4_rdn0: f64 = *var_t4_rdn0_slot;
        let mut var_t4_rdn1: f64 = *var_t4_rdn1_slot;
        let mut var_t4_rdn2: f64 = *var_t4_rdn2_slot;
        let mut var_t4_rdn3: f64 = *var_t4_rdn3_slot;
        let mut var_t4_rdn4: f64 = *var_t4_rdn4_slot;
        let mut var_t4_rdn5: f64 = *var_t4_rdn5_slot;
        let mut var_t4_rdn6: f64 = *var_t4_rdn6_slot;
        let mut var_t4_rdn7: f64 = *var_t4_rdn7_slot;
        let mut var_t4_rdn8: f64 = *var_t4_rdn8_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_db0: f64 = *var_t5_db0_slot;
        let mut var_t5_db1: f64 = *var_t5_db1_slot;
        let mut var_t5_db2: f64 = *var_t5_db2_slot;
        let mut var_t5_db3: f64 = *var_t5_db3_slot;
        let mut var_t5_db4: f64 = *var_t5_db4_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn1: f64 = *var_t5_dn1_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_rdb0: f64 = *var_t5_rdb0_slot;
        let mut var_t5_rdb1: f64 = *var_t5_rdb1_slot;
        let mut var_t5_rdb2: f64 = *var_t5_rdb2_slot;
        let mut var_t5_rdb3: f64 = *var_t5_rdb3_slot;
        let mut var_t5_rdb4: f64 = *var_t5_rdb4_slot;
        let mut var_t5_rdn0: f64 = *var_t5_rdn0_slot;
        let mut var_t5_rdn1: f64 = *var_t5_rdn1_slot;
        let mut var_t5_rdn2: f64 = *var_t5_rdn2_slot;
        let mut var_t5_rdn3: f64 = *var_t5_rdn3_slot;
        let mut var_t5_rdn4: f64 = *var_t5_rdn4_slot;
        let mut var_t5_rdn5: f64 = *var_t5_rdn5_slot;
        let mut var_t5_rdn6: f64 = *var_t5_rdn6_slot;
        let mut var_t5_rdn7: f64 = *var_t5_rdn7_slot;
        let mut var_t5_rdn8: f64 = *var_t5_rdn8_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_db0: f64 = *var_t6_db0_slot;
        let mut var_t6_db1: f64 = *var_t6_db1_slot;
        let mut var_t6_db2: f64 = *var_t6_db2_slot;
        let mut var_t6_db3: f64 = *var_t6_db3_slot;
        let mut var_t6_db4: f64 = *var_t6_db4_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn1: f64 = *var_t6_dn1_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_rdb0: f64 = *var_t6_rdb0_slot;
        let mut var_t6_rdb1: f64 = *var_t6_rdb1_slot;
        let mut var_t6_rdb2: f64 = *var_t6_rdb2_slot;
        let mut var_t6_rdb3: f64 = *var_t6_rdb3_slot;
        let mut var_t6_rdb4: f64 = *var_t6_rdb4_slot;
        let mut var_t6_rdn0: f64 = *var_t6_rdn0_slot;
        let mut var_t6_rdn1: f64 = *var_t6_rdn1_slot;
        let mut var_t6_rdn2: f64 = *var_t6_rdn2_slot;
        let mut var_t6_rdn3: f64 = *var_t6_rdn3_slot;
        let mut var_t6_rdn4: f64 = *var_t6_rdn4_slot;
        let mut var_t6_rdn5: f64 = *var_t6_rdn5_slot;
        let mut var_t6_rdn6: f64 = *var_t6_rdn6_slot;
        let mut var_t6_rdn7: f64 = *var_t6_rdn7_slot;
        let mut var_t6_rdn8: f64 = *var_t6_rdn8_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_vfbzb: f64 = *var_vfbzb_slot;
        let mut var_vfbzb_db0: f64 = *var_vfbzb_db0_slot;
        let mut var_vfbzb_db1: f64 = *var_vfbzb_db1_slot;
        let mut var_vfbzb_db2: f64 = *var_vfbzb_db2_slot;
        let mut var_vfbzb_db3: f64 = *var_vfbzb_db3_slot;
        let mut var_vfbzb_db4: f64 = *var_vfbzb_db4_slot;
        let mut var_vfbzb_dn0: f64 = *var_vfbzb_dn0_slot;
        let mut var_vfbzb_dn1: f64 = *var_vfbzb_dn1_slot;
        let mut var_vfbzb_dn2: f64 = *var_vfbzb_dn2_slot;
        let mut var_vfbzb_dn3: f64 = *var_vfbzb_dn3_slot;
        let mut var_vfbzb_dn4: f64 = *var_vfbzb_dn4_slot;
        let mut var_vfbzb_dn5: f64 = *var_vfbzb_dn5_slot;
        let mut var_vfbzb_dn6: f64 = *var_vfbzb_dn6_slot;
        let mut var_vfbzb_dn7: f64 = *var_vfbzb_dn7_slot;
        let mut var_vfbzb_dn8: f64 = *var_vfbzb_dn8_slot;
        let mut var_vfbzb_rdb0: f64 = *var_vfbzb_rdb0_slot;
        let mut var_vfbzb_rdb1: f64 = *var_vfbzb_rdb1_slot;
        let mut var_vfbzb_rdb2: f64 = *var_vfbzb_rdb2_slot;
        let mut var_vfbzb_rdb3: f64 = *var_vfbzb_rdb3_slot;
        let mut var_vfbzb_rdb4: f64 = *var_vfbzb_rdb4_slot;
        let mut var_vfbzb_rdn0: f64 = *var_vfbzb_rdn0_slot;
        let mut var_vfbzb_rdn1: f64 = *var_vfbzb_rdn1_slot;
        let mut var_vfbzb_rdn2: f64 = *var_vfbzb_rdn2_slot;
        let mut var_vfbzb_rdn3: f64 = *var_vfbzb_rdn3_slot;
        let mut var_vfbzb_rdn4: f64 = *var_vfbzb_rdn4_slot;
        let mut var_vfbzb_rdn5: f64 = *var_vfbzb_rdn5_slot;
        let mut var_vfbzb_rdn6: f64 = *var_vfbzb_rdn6_slot;
        let mut var_vfbzb_rdn7: f64 = *var_vfbzb_rdn7_slot;
        let mut var_vfbzb_rdn8: f64 = *var_vfbzb_rdn8_slot;
        let mut var_vfbzb_rv: f64 = *var_vfbzb_rv_slot;
        let mut var_voxacc: f64 = *var_voxacc_slot;
        let mut var_voxacc_db0: f64 = *var_voxacc_db0_slot;
        let mut var_voxacc_db1: f64 = *var_voxacc_db1_slot;
        let mut var_voxacc_db2: f64 = *var_voxacc_db2_slot;
        let mut var_voxacc_db3: f64 = *var_voxacc_db3_slot;
        let mut var_voxacc_db4: f64 = *var_voxacc_db4_slot;
        let mut var_voxacc_dn0: f64 = *var_voxacc_dn0_slot;
        let mut var_voxacc_dn1: f64 = *var_voxacc_dn1_slot;
        let mut var_voxacc_dn2: f64 = *var_voxacc_dn2_slot;
        let mut var_voxacc_dn3: f64 = *var_voxacc_dn3_slot;
        let mut var_voxacc_dn4: f64 = *var_voxacc_dn4_slot;
        let mut var_voxacc_dn5: f64 = *var_voxacc_dn5_slot;
        let mut var_voxacc_dn6: f64 = *var_voxacc_dn6_slot;
        let mut var_voxacc_dn7: f64 = *var_voxacc_dn7_slot;
        let mut var_voxacc_dn8: f64 = *var_voxacc_dn8_slot;
        let mut var_voxacc_rdb0: f64 = *var_voxacc_rdb0_slot;
        let mut var_voxacc_rdb1: f64 = *var_voxacc_rdb1_slot;
        let mut var_voxacc_rdb2: f64 = *var_voxacc_rdb2_slot;
        let mut var_voxacc_rdb3: f64 = *var_voxacc_rdb3_slot;
        let mut var_voxacc_rdb4: f64 = *var_voxacc_rdb4_slot;
        let mut var_voxacc_rdn0: f64 = *var_voxacc_rdn0_slot;
        let mut var_voxacc_rdn1: f64 = *var_voxacc_rdn1_slot;
        let mut var_voxacc_rdn2: f64 = *var_voxacc_rdn2_slot;
        let mut var_voxacc_rdn3: f64 = *var_voxacc_rdn3_slot;
        let mut var_voxacc_rdn4: f64 = *var_voxacc_rdn4_slot;
        let mut var_voxacc_rdn5: f64 = *var_voxacc_rdn5_slot;
        let mut var_voxacc_rdn6: f64 = *var_voxacc_rdn6_slot;
        let mut var_voxacc_rdn7: f64 = *var_voxacc_rdn7_slot;
        let mut var_voxacc_rdn8: f64 = *var_voxacc_rdn8_slot;
        let mut var_voxacc_rv: f64 = *var_voxacc_rv_slot;

        let (assign11830_e10981, assign11830_e10981_d_n0, assign11830_e10981_d_n1, assign11830_e10981_d_n2, assign11830_e10981_d_n3, assign11830_e10981_d_n4, assign11830_e10981_d_n5, assign11830_e10981_d_n6, assign11830_e10981_d_n7, assign11830_e10981_d_n8, assign11830_e10981_d_b0, assign11830_e10981_d_b1, assign11830_e10981_d_b2, assign11830_e10981_d_b3, assign11830_e10981_d_b4,) = {
    if (var_guard121 != 0.0) {
        let assign11830_e10975: f64 = (var_qia - var_eigbinv_i);
        let assign11830_e10977: f64 = (assign11830_e10975 / var_nigbinv_i);
        let assign11830_e10979: f64 = (assign11830_e10977 / var_vtm);
        (assign11830_e10979, (((((((var_qia_dn0 - var_eigbinv_i_dn0) * var_nigbinv_i) - (assign11830_e10975 * var_nigbinv_i_dn0)) / (var_nigbinv_i * var_nigbinv_i)) * var_vtm) - (assign11830_e10977 * var_vtm_dn0)) / (var_vtm * var_vtm)), (((((((var_qia_dn1 - var_eigbinv_i_dn1) * var_nigbinv_i) - (assign11830_e10975 * var_nigbinv_i_dn1)) / (var_nigbinv_i * var_nigbinv_i)) * var_vtm) - (assign11830_e10977 * var_vtm_dn1)) / (var_vtm * var_vtm)), (((((((var_qia_dn2 - var_eigbinv_i_dn2) * var_nigbinv_i) - (assign11830_e10975 * var_nigbinv_i_dn2)) / (var_nigbinv_i * var_nigbinv_i)) * var_vtm) - (assign11830_e10977 * var_vtm_dn2)) / (var_vtm * var_vtm)), (((((((var_qia_dn3 - var_eigbinv_i_dn3) * var_nigbinv_i) - (assign11830_e10975 * var_nigbinv_i_dn3)) / (var_nigbinv_i * var_nigbinv_i)) * var_vtm) - (assign11830_e10977 * var_vtm_dn3)) / (var_vtm * var_vtm)), (((((((var_qia_dn4 - var_eigbinv_i_dn4) * var_nigbinv_i) - (assign11830_e10975 * var_nigbinv_i_dn4)) / (var_nigbinv_i * var_nigbinv_i)) * var_vtm) - (assign11830_e10977 * var_vtm_dn4)) / (var_vtm * var_vtm)), (((((((var_qia_dn5 - var_eigbinv_i_dn5) * var_nigbinv_i) - (assign11830_e10975 * var_nigbinv_i_dn5)) / (var_nigbinv_i * var_nigbinv_i)) * var_vtm) - (assign11830_e10977 * var_vtm_dn5)) / (var_vtm * var_vtm)), (((((((var_qia_dn6 - var_eigbinv_i_dn6) * var_nigbinv_i) - (assign11830_e10975 * var_nigbinv_i_dn6)) / (var_nigbinv_i * var_nigbinv_i)) * var_vtm) - (assign11830_e10977 * var_vtm_dn6)) / (var_vtm * var_vtm)), (((((((var_qia_dn7 - var_eigbinv_i_dn7) * var_nigbinv_i) - (assign11830_e10975 * var_nigbinv_i_dn7)) / (var_nigbinv_i * var_nigbinv_i)) * var_vtm) - (assign11830_e10977 * var_vtm_dn7)) / (var_vtm * var_vtm)), (((((((var_qia_dn8 - var_eigbinv_i_dn8) * var_nigbinv_i) - (assign11830_e10975 * var_nigbinv_i_dn8)) / (var_nigbinv_i * var_nigbinv_i)) * var_vtm) - (assign11830_e10977 * var_vtm_dn8)) / (var_vtm * var_vtm)), (((((((var_qia_db0 - var_eigbinv_i_db0) * var_nigbinv_i) - (assign11830_e10975 * var_nigbinv_i_db0)) / (var_nigbinv_i * var_nigbinv_i)) * var_vtm) - (assign11830_e10977 * var_vtm_db0)) / (var_vtm * var_vtm)), (((((((var_qia_db1 - var_eigbinv_i_db1) * var_nigbinv_i) - (assign11830_e10975 * var_nigbinv_i_db1)) / (var_nigbinv_i * var_nigbinv_i)) * var_vtm) - (assign11830_e10977 * var_vtm_db1)) / (var_vtm * var_vtm)), (((((((var_qia_db2 - var_eigbinv_i_db2) * var_nigbinv_i) - (assign11830_e10975 * var_nigbinv_i_db2)) / (var_nigbinv_i * var_nigbinv_i)) * var_vtm) - (assign11830_e10977 * var_vtm_db2)) / (var_vtm * var_vtm)), (((((((var_qia_db3 - var_eigbinv_i_db3) * var_nigbinv_i) - (assign11830_e10975 * var_nigbinv_i_db3)) / (var_nigbinv_i * var_nigbinv_i)) * var_vtm) - (assign11830_e10977 * var_vtm_db3)) / (var_vtm * var_vtm)), (((((((var_qia_db4 - var_eigbinv_i_db4) * var_nigbinv_i) - (assign11830_e10975 * var_nigbinv_i_db4)) / (var_nigbinv_i * var_nigbinv_i)) * var_vtm) - (assign11830_e10977 * var_vtm_db4)) / (var_vtm * var_vtm)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4,)
    }
};
        var_t1 = assign11830_e10981;
        var_t1_dn0 = assign11830_e10981_d_n0;
        var_t1_dn1 = assign11830_e10981_d_n1;
        var_t1_dn2 = assign11830_e10981_d_n2;
        var_t1_dn3 = assign11830_e10981_d_n3;
        var_t1_dn4 = assign11830_e10981_d_n4;
        var_t1_dn5 = assign11830_e10981_d_n5;
        var_t1_dn6 = assign11830_e10981_d_n6;
        var_t1_dn7 = assign11830_e10981_d_n7;
        var_t1_dn8 = assign11830_e10981_d_n8;
        var_t1_db0 = assign11830_e10981_d_b0;
        var_t1_db1 = assign11830_e10981_d_b1;
        var_t1_db2 = assign11830_e10981_d_b2;
        var_t1_db3 = assign11830_e10981_d_b3;
        var_t1_db4 = assign11830_e10981_d_b4;
        var_t1_rv = 0.0;
        var_t1_rdn0 = 0.0;
        var_t1_rdn1 = 0.0;
        var_t1_rdn2 = 0.0;
        var_t1_rdn3 = 0.0;
        var_t1_rdn4 = 0.0;
        var_t1_rdn5 = 0.0;
        var_t1_rdn6 = 0.0;
        var_t1_rdn7 = 0.0;
        var_t1_rdn8 = 0.0;
        var_t1_rdb0 = 0.0;
        var_t1_rdb1 = 0.0;
        var_t1_rdb2 = 0.0;
        var_t1_rdb3 = 0.0;
        var_t1_rdb4 = 0.0;

        let (assign11850_e11003, assign11850_e11003_d_n0, assign11850_e11003_d_n1, assign11850_e11003_d_n2, assign11850_e11003_d_n3, assign11850_e11003_d_n4, assign11850_e11003_d_n5, assign11850_e11003_d_n6, assign11850_e11003_d_n7, assign11850_e11003_d_n8, assign11850_e11003_d_b0, assign11850_e11003_d_b1, assign11850_e11003_d_b2, assign11850_e11003_d_b3, assign11850_e11003_d_b4,) = {
    if (var_guard121 != 0.0) {
        let assign11850_e11000: f64 = (var_bigbinv_i * var_qia);
        let assign11850_e11001: f64 = (var_aigbinv_i - assign11850_e11000);
        (assign11850_e11001, (var_aigbinv_i_dn0 - ((var_bigbinv_i_dn0 * var_qia) + (var_bigbinv_i * var_qia_dn0))), (var_aigbinv_i_dn1 - ((var_bigbinv_i_dn1 * var_qia) + (var_bigbinv_i * var_qia_dn1))), (var_aigbinv_i_dn2 - ((var_bigbinv_i_dn2 * var_qia) + (var_bigbinv_i * var_qia_dn2))), (var_aigbinv_i_dn3 - ((var_bigbinv_i_dn3 * var_qia) + (var_bigbinv_i * var_qia_dn3))), (var_aigbinv_i_dn4 - ((var_bigbinv_i_dn4 * var_qia) + (var_bigbinv_i * var_qia_dn4))), (var_aigbinv_i_dn5 - ((var_bigbinv_i_dn5 * var_qia) + (var_bigbinv_i * var_qia_dn5))), (var_aigbinv_i_dn6 - ((var_bigbinv_i_dn6 * var_qia) + (var_bigbinv_i * var_qia_dn6))), (var_aigbinv_i_dn7 - ((var_bigbinv_i_dn7 * var_qia) + (var_bigbinv_i * var_qia_dn7))), (var_aigbinv_i_dn8 - ((var_bigbinv_i_dn8 * var_qia) + (var_bigbinv_i * var_qia_dn8))), (var_aigbinv_i_db0 - ((var_bigbinv_i_db0 * var_qia) + (var_bigbinv_i * var_qia_db0))), (var_aigbinv_i_db1 - ((var_bigbinv_i_db1 * var_qia) + (var_bigbinv_i * var_qia_db1))), (var_aigbinv_i_db2 - ((var_bigbinv_i_db2 * var_qia) + (var_bigbinv_i * var_qia_db2))), (var_aigbinv_i_db3 - ((var_bigbinv_i_db3 * var_qia) + (var_bigbinv_i * var_qia_db3))), (var_aigbinv_i_db4 - ((var_bigbinv_i_db4 * var_qia) + (var_bigbinv_i * var_qia_db4))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_db0, var_t2_db1, var_t2_db2, var_t2_db3, var_t2_db4,)
    }
};
        var_t2 = assign11850_e11003;
        var_t2_dn0 = assign11850_e11003_d_n0;
        var_t2_dn1 = assign11850_e11003_d_n1;
        var_t2_dn2 = assign11850_e11003_d_n2;
        var_t2_dn3 = assign11850_e11003_d_n3;
        var_t2_dn4 = assign11850_e11003_d_n4;
        var_t2_dn5 = assign11850_e11003_d_n5;
        var_t2_dn6 = assign11850_e11003_d_n6;
        var_t2_dn7 = assign11850_e11003_d_n7;
        var_t2_dn8 = assign11850_e11003_d_n8;
        var_t2_db0 = assign11850_e11003_d_b0;
        var_t2_db1 = assign11850_e11003_d_b1;
        var_t2_db2 = assign11850_e11003_d_b2;
        var_t2_db3 = assign11850_e11003_d_b3;
        var_t2_db4 = assign11850_e11003_d_b4;
        var_t2_rv = 0.0;
        var_t2_rdn0 = 0.0;
        var_t2_rdn1 = 0.0;
        var_t2_rdn2 = 0.0;
        var_t2_rdn3 = 0.0;
        var_t2_rdn4 = 0.0;
        var_t2_rdn5 = 0.0;
        var_t2_rdn6 = 0.0;
        var_t2_rdn7 = 0.0;
        var_t2_rdn8 = 0.0;
        var_t2_rdb0 = 0.0;
        var_t2_rdb1 = 0.0;
        var_t2_rdb2 = 0.0;
        var_t2_rdb3 = 0.0;
        var_t2_rdb4 = 0.0;

        let (assign11860_e11011, assign11860_e11011_d_n0, assign11860_e11011_d_n1, assign11860_e11011_d_n2, assign11860_e11011_d_n3, assign11860_e11011_d_n4, assign11860_e11011_d_n5, assign11860_e11011_d_n6, assign11860_e11011_d_n7, assign11860_e11011_d_n8, assign11860_e11011_d_b0, assign11860_e11011_d_b1, assign11860_e11011_d_b2, assign11860_e11011_d_b3, assign11860_e11011_d_b4,) = {
    if (var_guard121 != 0.0) {
        let assign11860_e11008: f64 = (var_cigbinv_i * var_qia);
        let assign11860_e11009: f64 = (1.0 + assign11860_e11008);
        (assign11860_e11009, ((var_cigbinv_i_dn0 * var_qia) + (var_cigbinv_i * var_qia_dn0)), ((var_cigbinv_i_dn1 * var_qia) + (var_cigbinv_i * var_qia_dn1)), ((var_cigbinv_i_dn2 * var_qia) + (var_cigbinv_i * var_qia_dn2)), ((var_cigbinv_i_dn3 * var_qia) + (var_cigbinv_i * var_qia_dn3)), ((var_cigbinv_i_dn4 * var_qia) + (var_cigbinv_i * var_qia_dn4)), ((var_cigbinv_i_dn5 * var_qia) + (var_cigbinv_i * var_qia_dn5)), ((var_cigbinv_i_dn6 * var_qia) + (var_cigbinv_i * var_qia_dn6)), ((var_cigbinv_i_dn7 * var_qia) + (var_cigbinv_i * var_qia_dn7)), ((var_cigbinv_i_dn8 * var_qia) + (var_cigbinv_i * var_qia_dn8)), ((var_cigbinv_i_db0 * var_qia) + (var_cigbinv_i * var_qia_db0)), ((var_cigbinv_i_db1 * var_qia) + (var_cigbinv_i * var_qia_db1)), ((var_cigbinv_i_db2 * var_qia) + (var_cigbinv_i * var_qia_db2)), ((var_cigbinv_i_db3 * var_qia) + (var_cigbinv_i * var_qia_db3)), ((var_cigbinv_i_db4 * var_qia) + (var_cigbinv_i * var_qia_db4)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn1, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_db0, var_t3_db1, var_t3_db2, var_t3_db3, var_t3_db4,)
    }
};
        var_t3 = assign11860_e11011;
        var_t3_dn0 = assign11860_e11011_d_n0;
        var_t3_dn1 = assign11860_e11011_d_n1;
        var_t3_dn2 = assign11860_e11011_d_n2;
        var_t3_dn3 = assign11860_e11011_d_n3;
        var_t3_dn4 = assign11860_e11011_d_n4;
        var_t3_dn5 = assign11860_e11011_d_n5;
        var_t3_dn6 = assign11860_e11011_d_n6;
        var_t3_dn7 = assign11860_e11011_d_n7;
        var_t3_dn8 = assign11860_e11011_d_n8;
        var_t3_db0 = assign11860_e11011_d_b0;
        var_t3_db1 = assign11860_e11011_d_b1;
        var_t3_db2 = assign11860_e11011_d_b2;
        var_t3_db3 = assign11860_e11011_d_b3;
        var_t3_db4 = assign11860_e11011_d_b4;
        var_t3_rv = 0.0;
        var_t3_rdn0 = 0.0;
        var_t3_rdn1 = 0.0;
        var_t3_rdn2 = 0.0;
        var_t3_rdn3 = 0.0;
        var_t3_rdn4 = 0.0;
        var_t3_rdn5 = 0.0;
        var_t3_rdn6 = 0.0;
        var_t3_rdn7 = 0.0;
        var_t3_rdn8 = 0.0;
        var_t3_rdb0 = 0.0;
        var_t3_rdb1 = 0.0;
        var_t3_rdb2 = 0.0;
        var_t3_rdb3 = 0.0;
        var_t3_rdb4 = 0.0;

        let (assign11870_e11022, assign11870_e11022_d_n0, assign11870_e11022_d_n1, assign11870_e11022_d_n2, assign11870_e11022_d_n3, assign11870_e11022_d_n4, assign11870_e11022_d_n5, assign11870_e11022_d_n6, assign11870_e11022_d_n7, assign11870_e11022_d_n8, assign11870_e11022_d_b0, assign11870_e11022_d_b1, assign11870_e11022_d_b2, assign11870_e11022_d_b3, assign11870_e11022_d_b4,) = {
    if (var_guard121 != 0.0) {
        let assign11870_e11014: f64 = (-982222000000.0);
        let assign11870_e11016: f64 = (assign11870_e11014 * p.p99);
        let assign11870_e11018: f64 = (assign11870_e11016 * var_t2);
        let assign11870_e11020: f64 = (assign11870_e11018 * var_t3);
        (assign11870_e11020, (((assign11870_e11016 * var_t2_dn0) * var_t3) + (assign11870_e11018 * var_t3_dn0)), (((assign11870_e11016 * var_t2_dn1) * var_t3) + (assign11870_e11018 * var_t3_dn1)), (((assign11870_e11016 * var_t2_dn2) * var_t3) + (assign11870_e11018 * var_t3_dn2)), (((assign11870_e11016 * var_t2_dn3) * var_t3) + (assign11870_e11018 * var_t3_dn3)), (((assign11870_e11016 * var_t2_dn4) * var_t3) + (assign11870_e11018 * var_t3_dn4)), (((assign11870_e11016 * var_t2_dn5) * var_t3) + (assign11870_e11018 * var_t3_dn5)), (((assign11870_e11016 * var_t2_dn6) * var_t3) + (assign11870_e11018 * var_t3_dn6)), (((assign11870_e11016 * var_t2_dn7) * var_t3) + (assign11870_e11018 * var_t3_dn7)), (((assign11870_e11016 * var_t2_dn8) * var_t3) + (assign11870_e11018 * var_t3_dn8)), (((assign11870_e11016 * var_t2_db0) * var_t3) + (assign11870_e11018 * var_t3_db0)), (((assign11870_e11016 * var_t2_db1) * var_t3) + (assign11870_e11018 * var_t3_db1)), (((assign11870_e11016 * var_t2_db2) * var_t3) + (assign11870_e11018 * var_t3_db2)), (((assign11870_e11016 * var_t2_db3) * var_t3) + (assign11870_e11018 * var_t3_db3)), (((assign11870_e11016 * var_t2_db4) * var_t3) + (assign11870_e11018 * var_t3_db4)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn1, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_db0, var_t4_db1, var_t4_db2, var_t4_db3, var_t4_db4,)
    }
};
        var_t4 = assign11870_e11022;
        var_t4_dn0 = assign11870_e11022_d_n0;
        var_t4_dn1 = assign11870_e11022_d_n1;
        var_t4_dn2 = assign11870_e11022_d_n2;
        var_t4_dn3 = assign11870_e11022_d_n3;
        var_t4_dn4 = assign11870_e11022_d_n4;
        var_t4_dn5 = assign11870_e11022_d_n5;
        var_t4_dn6 = assign11870_e11022_d_n6;
        var_t4_dn7 = assign11870_e11022_d_n7;
        var_t4_dn8 = assign11870_e11022_d_n8;
        var_t4_db0 = assign11870_e11022_d_b0;
        var_t4_db1 = assign11870_e11022_d_b1;
        var_t4_db2 = assign11870_e11022_d_b2;
        var_t4_db3 = assign11870_e11022_d_b3;
        var_t4_db4 = assign11870_e11022_d_b4;
        var_t4_rv = 0.0;
        var_t4_rdn0 = 0.0;
        var_t4_rdn1 = 0.0;
        var_t4_rdn2 = 0.0;
        var_t4_rdn3 = 0.0;
        var_t4_rdn4 = 0.0;
        var_t4_rdn5 = 0.0;
        var_t4_rdn6 = 0.0;
        var_t4_rdn7 = 0.0;
        var_t4_rdn8 = 0.0;
        var_t4_rdb0 = 0.0;
        var_t4_rdb1 = 0.0;
        var_t4_rdb2 = 0.0;
        var_t4_rdb3 = 0.0;
        var_t4_rdb4 = 0.0;

        let (assign11880_e11027, assign11880_e11027_d_n0, assign11880_e11027_d_n1, assign11880_e11027_d_n2, assign11880_e11027_d_n3, assign11880_e11027_d_n4, assign11880_e11027_d_n5, assign11880_e11027_d_n6, assign11880_e11027_d_n7, assign11880_e11027_d_n8, assign11880_e11027_d_b0, assign11880_e11027_d_b1, assign11880_e11027_d_b2, assign11880_e11027_d_b3, assign11880_e11027_d_b4,) = {
    if (var_guard121 != 0.0) {
        let assign11880_e11025: f64 = { let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign11880_e11025, ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn0), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn1), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn2), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn3), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn4), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn5), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn6), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn7), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn8), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_db0), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_db1), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_db2), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_db3), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_db4),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn1, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_db0, var_t5_db1, var_t5_db2, var_t5_db3, var_t5_db4,)
    }
};
        var_t5 = assign11880_e11027;
        var_t5_dn0 = assign11880_e11027_d_n0;
        var_t5_dn1 = assign11880_e11027_d_n1;
        var_t5_dn2 = assign11880_e11027_d_n2;
        var_t5_dn3 = assign11880_e11027_d_n3;
        var_t5_dn4 = assign11880_e11027_d_n4;
        var_t5_dn5 = assign11880_e11027_d_n5;
        var_t5_dn6 = assign11880_e11027_d_n6;
        var_t5_dn7 = assign11880_e11027_d_n7;
        var_t5_dn8 = assign11880_e11027_d_n8;
        var_t5_db0 = assign11880_e11027_d_b0;
        var_t5_db1 = assign11880_e11027_d_b1;
        var_t5_db2 = assign11880_e11027_d_b2;
        var_t5_db3 = assign11880_e11027_d_b3;
        var_t5_db4 = assign11880_e11027_d_b4;
        var_t5_rv = 0.0;
        var_t5_rdn0 = 0.0;
        var_t5_rdn1 = 0.0;
        var_t5_rdn2 = 0.0;
        var_t5_rdn3 = 0.0;
        var_t5_rdn4 = 0.0;
        var_t5_rdn5 = 0.0;
        var_t5_rdn6 = 0.0;
        var_t5_rdn7 = 0.0;
        var_t5_rdn8 = 0.0;
        var_t5_rdb0 = 0.0;
        var_t5_rdb1 = 0.0;
        var_t5_rdb2 = 0.0;
        var_t5_rdb3 = 0.0;
        var_t5_rdb4 = 0.0;

        let (assign11890_e11031, assign11890_e11031_d_n0, assign11890_e11031_d_n1, assign11890_e11031_d_n2, assign11890_e11031_d_n3, assign11890_e11031_d_n4, assign11890_e11031_d_n5, assign11890_e11031_d_n6, assign11890_e11031_d_n7, assign11890_e11031_d_n8, assign11890_e11031_d_b0, assign11890_e11031_d_b1, assign11890_e11031_d_b2, assign11890_e11031_d_b3, assign11890_e11031_d_b4,) = {
    if (var_guard121 != 0.0) {
        (3.75956e-7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn1, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_db0, var_t6_db1, var_t6_db2, var_t6_db3, var_t6_db4,)
    }
};
        var_t6 = assign11890_e11031;
        var_t6_dn0 = assign11890_e11031_d_n0;
        var_t6_dn1 = assign11890_e11031_d_n1;
        var_t6_dn2 = assign11890_e11031_d_n2;
        var_t6_dn3 = assign11890_e11031_d_n3;
        var_t6_dn4 = assign11890_e11031_d_n4;
        var_t6_dn5 = assign11890_e11031_d_n5;
        var_t6_dn6 = assign11890_e11031_d_n6;
        var_t6_dn7 = assign11890_e11031_d_n7;
        var_t6_dn8 = assign11890_e11031_d_n8;
        var_t6_db0 = assign11890_e11031_d_b0;
        var_t6_db1 = assign11890_e11031_d_b1;
        var_t6_db2 = assign11890_e11031_d_b2;
        var_t6_db3 = assign11890_e11031_d_b3;
        var_t6_db4 = assign11890_e11031_d_b4;
        var_t6_rv = 0.0;
        var_t6_rdn0 = 0.0;
        var_t6_rdn1 = 0.0;
        var_t6_rdn2 = 0.0;
        var_t6_rdn3 = 0.0;
        var_t6_rdn4 = 0.0;
        var_t6_rdn5 = 0.0;
        var_t6_rdn6 = 0.0;
        var_t6_rdn7 = 0.0;
        var_t6_rdn8 = 0.0;
        var_t6_rdb0 = 0.0;
        var_t6_rdb1 = 0.0;
        var_t6_rdb2 = 0.0;
        var_t6_rdb3 = 0.0;
        var_t6_rdb4 = 0.0;

        let (assign11920_e11059, assign11920_e11059_d_n0, assign11920_e11059_d_n1, assign11920_e11059_d_n2, assign11920_e11059_d_n3, assign11920_e11059_d_n4, assign11920_e11059_d_n5, assign11920_e11059_d_n6, assign11920_e11059_d_n7, assign11920_e11059_d_n8, assign11920_e11059_d_b0, assign11920_e11059_d_b1, assign11920_e11059_d_b2, assign11920_e11059_d_b3, assign11920_e11059_d_b4,) = {
    if (var_guard121 != 0.0) {
        let assign11920_e11057: f64 = (var_deltaphi1 - var_phib);
        (assign11920_e11057, (var_deltaphi1_dn0 - var_phib_dn0), (var_deltaphi1_dn1 - var_phib_dn1), (var_deltaphi1_dn2 - var_phib_dn2), (var_deltaphi1_dn3 - var_phib_dn3), (var_deltaphi1_dn4 - var_phib_dn4), (var_deltaphi1_dn5 - var_phib_dn5), (var_deltaphi1_dn6 - var_phib_dn6), (var_deltaphi1_dn7 - var_phib_dn7), (var_deltaphi1_dn8 - var_phib_dn8), (var_deltaphi1_db0 - var_phib_db0), (var_deltaphi1_db1 - var_phib_db1), (var_deltaphi1_db2 - var_phib_db2), (var_deltaphi1_db3 - var_phib_db3), (var_deltaphi1_db4 - var_phib_db4),)
    } else {
        (var_vfbzb, var_vfbzb_dn0, var_vfbzb_dn1, var_vfbzb_dn2, var_vfbzb_dn3, var_vfbzb_dn4, var_vfbzb_dn5, var_vfbzb_dn6, var_vfbzb_dn7, var_vfbzb_dn8, var_vfbzb_db0, var_vfbzb_db1, var_vfbzb_db2, var_vfbzb_db3, var_vfbzb_db4,)
    }
};
        var_vfbzb = assign11920_e11059;
        var_vfbzb_dn0 = assign11920_e11059_d_n0;
        var_vfbzb_dn1 = assign11920_e11059_d_n1;
        var_vfbzb_dn2 = assign11920_e11059_d_n2;
        var_vfbzb_dn3 = assign11920_e11059_d_n3;
        var_vfbzb_dn4 = assign11920_e11059_d_n4;
        var_vfbzb_dn5 = assign11920_e11059_d_n5;
        var_vfbzb_dn6 = assign11920_e11059_d_n6;
        var_vfbzb_dn7 = assign11920_e11059_d_n7;
        var_vfbzb_dn8 = assign11920_e11059_d_n8;
        var_vfbzb_db0 = assign11920_e11059_d_b0;
        var_vfbzb_db1 = assign11920_e11059_d_b1;
        var_vfbzb_db2 = assign11920_e11059_d_b2;
        var_vfbzb_db3 = assign11920_e11059_d_b3;
        var_vfbzb_db4 = assign11920_e11059_d_b4;
        var_vfbzb_rv = 0.0;
        var_vfbzb_rdn0 = 0.0;
        var_vfbzb_rdn1 = 0.0;
        var_vfbzb_rdn2 = 0.0;
        var_vfbzb_rdn3 = 0.0;
        var_vfbzb_rdn4 = 0.0;
        var_vfbzb_rdn5 = 0.0;
        var_vfbzb_rdn6 = 0.0;
        var_vfbzb_rdn7 = 0.0;
        var_vfbzb_rdn8 = 0.0;
        var_vfbzb_rdb0 = 0.0;
        var_vfbzb_rdb1 = 0.0;
        var_vfbzb_rdb2 = 0.0;
        var_vfbzb_rdb3 = 0.0;
        var_vfbzb_rdb4 = 0.0;

        let (assign11930_e11065, assign11930_e11065_d_n0, assign11930_e11065_d_n1, assign11930_e11065_d_n2, assign11930_e11065_d_n3, assign11930_e11065_d_n4, assign11930_e11065_d_n5, assign11930_e11065_d_n6, assign11930_e11065_d_n7, assign11930_e11065_d_n8, assign11930_e11065_d_b0, assign11930_e11065_d_b1, assign11930_e11065_d_b2, assign11930_e11065_d_b3, assign11930_e11065_d_b4,) = {
    if (var_guard121 != 0.0) {
        let assign11930_e11063: f64 = (var_vfbzb - var_vgbg);
        (assign11930_e11063, (var_vfbzb_dn0 - var_vgbg_dn0), (var_vfbzb_dn1 - var_vgbg_dn1), (var_vfbzb_dn2 - var_vgbg_dn2), (var_vfbzb_dn3 - var_vgbg_dn3), (var_vfbzb_dn4 - var_vgbg_dn4), (var_vfbzb_dn5 - var_vgbg_dn5), (var_vfbzb_dn6 - var_vgbg_dn6), (var_vfbzb_dn7 - var_vgbg_dn7), (var_vfbzb_dn8 - var_vgbg_dn8), (var_vfbzb_db0 - var_vgbg_db0), (var_vfbzb_db1 - var_vgbg_db1), (var_vfbzb_db2 - var_vgbg_db2), (var_vfbzb_db3 - var_vgbg_db3), (var_vfbzb_db4 - var_vgbg_db4),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4,)
    }
};
        var_t0 = assign11930_e11065;
        var_t0_dn0 = assign11930_e11065_d_n0;
        var_t0_dn1 = assign11930_e11065_d_n1;
        var_t0_dn2 = assign11930_e11065_d_n2;
        var_t0_dn3 = assign11930_e11065_d_n3;
        var_t0_dn4 = assign11930_e11065_d_n4;
        var_t0_dn5 = assign11930_e11065_d_n5;
        var_t0_dn6 = assign11930_e11065_d_n6;
        var_t0_dn7 = assign11930_e11065_d_n7;
        var_t0_dn8 = assign11930_e11065_d_n8;
        var_t0_db0 = assign11930_e11065_d_b0;
        var_t0_db1 = assign11930_e11065_d_b1;
        var_t0_db2 = assign11930_e11065_d_b2;
        var_t0_db3 = assign11930_e11065_d_b3;
        var_t0_db4 = assign11930_e11065_d_b4;
        var_t0_rv = 0.0;
        var_t0_rdn0 = 0.0;
        var_t0_rdn1 = 0.0;
        var_t0_rdn2 = 0.0;
        var_t0_rdn3 = 0.0;
        var_t0_rdn4 = 0.0;
        var_t0_rdn5 = 0.0;
        var_t0_rdn6 = 0.0;
        var_t0_rdn7 = 0.0;
        var_t0_rdn8 = 0.0;
        var_t0_rdb0 = 0.0;
        var_t0_rdb1 = 0.0;
        var_t0_rdb2 = 0.0;
        var_t0_rdb3 = 0.0;
        var_t0_rdb4 = 0.0;

        let (assign11940_e11073, assign11940_e11073_d_n0, assign11940_e11073_d_n1, assign11940_e11073_d_n2, assign11940_e11073_d_n3, assign11940_e11073_d_n4, assign11940_e11073_d_n5, assign11940_e11073_d_n6, assign11940_e11073_d_n7, assign11940_e11073_d_n8, assign11940_e11073_d_b0, assign11940_e11073_d_b1, assign11940_e11073_d_b2, assign11940_e11073_d_b3, assign11940_e11073_d_b4,) = {
    if (var_guard121 != 0.0) {
        let assign11940_e11069: f64 = (var_t0 / var_nigbacc_i);
        let assign11940_e11071: f64 = (assign11940_e11069 / var_vtm);
        (assign11940_e11071, ((((((var_t0_dn0 * var_nigbacc_i) - (var_t0 * var_nigbacc_i_dn0)) / (var_nigbacc_i * var_nigbacc_i)) * var_vtm) - (assign11940_e11069 * var_vtm_dn0)) / (var_vtm * var_vtm)), ((((((var_t0_dn1 * var_nigbacc_i) - (var_t0 * var_nigbacc_i_dn1)) / (var_nigbacc_i * var_nigbacc_i)) * var_vtm) - (assign11940_e11069 * var_vtm_dn1)) / (var_vtm * var_vtm)), ((((((var_t0_dn2 * var_nigbacc_i) - (var_t0 * var_nigbacc_i_dn2)) / (var_nigbacc_i * var_nigbacc_i)) * var_vtm) - (assign11940_e11069 * var_vtm_dn2)) / (var_vtm * var_vtm)), ((((((var_t0_dn3 * var_nigbacc_i) - (var_t0 * var_nigbacc_i_dn3)) / (var_nigbacc_i * var_nigbacc_i)) * var_vtm) - (assign11940_e11069 * var_vtm_dn3)) / (var_vtm * var_vtm)), ((((((var_t0_dn4 * var_nigbacc_i) - (var_t0 * var_nigbacc_i_dn4)) / (var_nigbacc_i * var_nigbacc_i)) * var_vtm) - (assign11940_e11069 * var_vtm_dn4)) / (var_vtm * var_vtm)), ((((((var_t0_dn5 * var_nigbacc_i) - (var_t0 * var_nigbacc_i_dn5)) / (var_nigbacc_i * var_nigbacc_i)) * var_vtm) - (assign11940_e11069 * var_vtm_dn5)) / (var_vtm * var_vtm)), ((((((var_t0_dn6 * var_nigbacc_i) - (var_t0 * var_nigbacc_i_dn6)) / (var_nigbacc_i * var_nigbacc_i)) * var_vtm) - (assign11940_e11069 * var_vtm_dn6)) / (var_vtm * var_vtm)), ((((((var_t0_dn7 * var_nigbacc_i) - (var_t0 * var_nigbacc_i_dn7)) / (var_nigbacc_i * var_nigbacc_i)) * var_vtm) - (assign11940_e11069 * var_vtm_dn7)) / (var_vtm * var_vtm)), ((((((var_t0_dn8 * var_nigbacc_i) - (var_t0 * var_nigbacc_i_dn8)) / (var_nigbacc_i * var_nigbacc_i)) * var_vtm) - (assign11940_e11069 * var_vtm_dn8)) / (var_vtm * var_vtm)), ((((((var_t0_db0 * var_nigbacc_i) - (var_t0 * var_nigbacc_i_db0)) / (var_nigbacc_i * var_nigbacc_i)) * var_vtm) - (assign11940_e11069 * var_vtm_db0)) / (var_vtm * var_vtm)), ((((((var_t0_db1 * var_nigbacc_i) - (var_t0 * var_nigbacc_i_db1)) / (var_nigbacc_i * var_nigbacc_i)) * var_vtm) - (assign11940_e11069 * var_vtm_db1)) / (var_vtm * var_vtm)), ((((((var_t0_db2 * var_nigbacc_i) - (var_t0 * var_nigbacc_i_db2)) / (var_nigbacc_i * var_nigbacc_i)) * var_vtm) - (assign11940_e11069 * var_vtm_db2)) / (var_vtm * var_vtm)), ((((((var_t0_db3 * var_nigbacc_i) - (var_t0 * var_nigbacc_i_db3)) / (var_nigbacc_i * var_nigbacc_i)) * var_vtm) - (assign11940_e11069 * var_vtm_db3)) / (var_vtm * var_vtm)), ((((((var_t0_db4 * var_nigbacc_i) - (var_t0 * var_nigbacc_i_db4)) / (var_nigbacc_i * var_nigbacc_i)) * var_vtm) - (assign11940_e11069 * var_vtm_db4)) / (var_vtm * var_vtm)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4,)
    }
};
        var_t1 = assign11940_e11073;
        var_t1_dn0 = assign11940_e11073_d_n0;
        var_t1_dn1 = assign11940_e11073_d_n1;
        var_t1_dn2 = assign11940_e11073_d_n2;
        var_t1_dn3 = assign11940_e11073_d_n3;
        var_t1_dn4 = assign11940_e11073_d_n4;
        var_t1_dn5 = assign11940_e11073_d_n5;
        var_t1_dn6 = assign11940_e11073_d_n6;
        var_t1_dn7 = assign11940_e11073_d_n7;
        var_t1_dn8 = assign11940_e11073_d_n8;
        var_t1_db0 = assign11940_e11073_d_b0;
        var_t1_db1 = assign11940_e11073_d_b1;
        var_t1_db2 = assign11940_e11073_d_b2;
        var_t1_db3 = assign11940_e11073_d_b3;
        var_t1_db4 = assign11940_e11073_d_b4;
        var_t1_rv = 0.0;
        var_t1_rdn0 = 0.0;
        var_t1_rdn1 = 0.0;
        var_t1_rdn2 = 0.0;
        var_t1_rdn3 = 0.0;
        var_t1_rdn4 = 0.0;
        var_t1_rdn5 = 0.0;
        var_t1_rdn6 = 0.0;
        var_t1_rdn7 = 0.0;
        var_t1_rdn8 = 0.0;
        var_t1_rdb0 = 0.0;
        var_t1_rdb1 = 0.0;
        var_t1_rdb2 = 0.0;
        var_t1_rdb3 = 0.0;
        var_t1_rdb4 = 0.0;

        let assign11960_e11090: f64 = if var_vfbzb <= 0.0 { 1.0 } else { 0.0 };
        var_guard122 = assign11960_e11090;
        var_guard122_dn0 = 0.0;
        var_guard122_dn1 = 0.0;
        var_guard122_dn2 = 0.0;
        var_guard122_dn3 = 0.0;
        var_guard122_dn4 = 0.0;
        var_guard122_dn5 = 0.0;
        var_guard122_dn6 = 0.0;
        var_guard122_dn7 = 0.0;
        var_guard122_dn8 = 0.0;
        var_guard122_db0 = 0.0;
        var_guard122_db1 = 0.0;
        var_guard122_db2 = 0.0;
        var_guard122_db3 = 0.0;
        var_guard122_db4 = 0.0;
        var_guard122_rv = 0.0;
        var_guard122_rdn0 = 0.0;
        var_guard122_rdn1 = 0.0;
        var_guard122_rdn2 = 0.0;
        var_guard122_rdn3 = 0.0;
        var_guard122_rdn4 = 0.0;
        var_guard122_rdn5 = 0.0;
        var_guard122_rdn6 = 0.0;
        var_guard122_rdn7 = 0.0;
        var_guard122_rdn8 = 0.0;
        var_guard122_rdb0 = 0.0;
        var_guard122_rdb1 = 0.0;
        var_guard122_rdb2 = 0.0;
        var_guard122_rdb3 = 0.0;
        var_guard122_rdb4 = 0.0;

        let (assign11970_e11113, assign11970_e11113_d_n0, assign11970_e11113_d_n1, assign11970_e11113_d_n2, assign11970_e11113_d_n3, assign11970_e11113_d_n4, assign11970_e11113_d_n5, assign11970_e11113_d_n6, assign11970_e11113_d_n7, assign11970_e11113_d_n8, assign11970_e11113_d_b0, assign11970_e11113_d_b1, assign11970_e11113_d_b2, assign11970_e11113_d_b3, assign11970_e11113_d_b4,) = {
    if ((var_guard121 != 0.0) && (var_guard122 != 0.0)) {
        let assign11970_e11097: f64 = (var_t0 - 0.02);
        let assign11970_e11100: f64 = (var_t0 - 0.02);
        let assign11970_e11103: f64 = (var_t0 - 0.02);
        let assign11970_e11104: f64 = (assign11970_e11100 * assign11970_e11103);
        let assign11970_e11107: f64 = (0.08 * var_vfbzb);
        let assign11970_e11108: f64 = (assign11970_e11104 - assign11970_e11107);
        let assign11970_e11109: f64 = (assign11970_e11108).sqrt();
        let assign11970_e11110: f64 = (assign11970_e11097 + assign11970_e11109);
        let assign11970_e11111: f64 = (0.5 * assign11970_e11110);
        (assign11970_e11111, (0.5 * (var_t0_dn0 + ((((var_t0_dn0 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn0)) - (0.08 * var_vfbzb_dn0)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_dn1 + ((((var_t0_dn1 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn1)) - (0.08 * var_vfbzb_dn1)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_dn2 + ((((var_t0_dn2 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn2)) - (0.08 * var_vfbzb_dn2)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_dn3 + ((((var_t0_dn3 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn3)) - (0.08 * var_vfbzb_dn3)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_dn4 + ((((var_t0_dn4 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn4)) - (0.08 * var_vfbzb_dn4)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_dn5 + ((((var_t0_dn5 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn5)) - (0.08 * var_vfbzb_dn5)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_dn6 + ((((var_t0_dn6 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn6)) - (0.08 * var_vfbzb_dn6)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_dn7 + ((((var_t0_dn7 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn7)) - (0.08 * var_vfbzb_dn7)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_dn8 + ((((var_t0_dn8 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn8)) - (0.08 * var_vfbzb_dn8)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_db0 + ((((var_t0_db0 * assign11970_e11103) + (assign11970_e11100 * var_t0_db0)) - (0.08 * var_vfbzb_db0)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_db1 + ((((var_t0_db1 * assign11970_e11103) + (assign11970_e11100 * var_t0_db1)) - (0.08 * var_vfbzb_db1)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_db2 + ((((var_t0_db2 * assign11970_e11103) + (assign11970_e11100 * var_t0_db2)) - (0.08 * var_vfbzb_db2)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_db3 + ((((var_t0_db3 * assign11970_e11103) + (assign11970_e11100 * var_t0_db3)) - (0.08 * var_vfbzb_db3)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_db4 + ((((var_t0_db4 * assign11970_e11103) + (assign11970_e11100 * var_t0_db4)) - (0.08 * var_vfbzb_db4)) / (2.0 * assign11970_e11109)))),)
    } else {
        (var_voxacc, var_voxacc_dn0, var_voxacc_dn1, var_voxacc_dn2, var_voxacc_dn3, var_voxacc_dn4, var_voxacc_dn5, var_voxacc_dn6, var_voxacc_dn7, var_voxacc_dn8, var_voxacc_db0, var_voxacc_db1, var_voxacc_db2, var_voxacc_db3, var_voxacc_db4,)
    }
};
        var_voxacc = assign11970_e11113;
        var_voxacc_dn0 = assign11970_e11113_d_n0;
        var_voxacc_dn1 = assign11970_e11113_d_n1;
        var_voxacc_dn2 = assign11970_e11113_d_n2;
        var_voxacc_dn3 = assign11970_e11113_d_n3;
        var_voxacc_dn4 = assign11970_e11113_d_n4;
        var_voxacc_dn5 = assign11970_e11113_d_n5;
        var_voxacc_dn6 = assign11970_e11113_d_n6;
        var_voxacc_dn7 = assign11970_e11113_d_n7;
        var_voxacc_dn8 = assign11970_e11113_d_n8;
        var_voxacc_db0 = assign11970_e11113_d_b0;
        var_voxacc_db1 = assign11970_e11113_d_b1;
        var_voxacc_db2 = assign11970_e11113_d_b2;
        var_voxacc_db3 = assign11970_e11113_d_b3;
        var_voxacc_db4 = assign11970_e11113_d_b4;
        var_voxacc_rv = 0.0;
        var_voxacc_rdn0 = 0.0;
        var_voxacc_rdn1 = 0.0;
        var_voxacc_rdn2 = 0.0;
        var_voxacc_rdn3 = 0.0;
        var_voxacc_rdn4 = 0.0;
        var_voxacc_rdn5 = 0.0;
        var_voxacc_rdn6 = 0.0;
        var_voxacc_rdn7 = 0.0;
        var_voxacc_rdn8 = 0.0;
        var_voxacc_rdb0 = 0.0;
        var_voxacc_rdb1 = 0.0;
        var_voxacc_rdb2 = 0.0;
        var_voxacc_rdb3 = 0.0;
        var_voxacc_rdb4 = 0.0;

        let (assign11980_e11137, assign11980_e11137_d_n0, assign11980_e11137_d_n1, assign11980_e11137_d_n2, assign11980_e11137_d_n3, assign11980_e11137_d_n4, assign11980_e11137_d_n5, assign11980_e11137_d_n6, assign11980_e11137_d_n7, assign11980_e11137_d_n8, assign11980_e11137_d_b0, assign11980_e11137_d_b1, assign11980_e11137_d_b2, assign11980_e11137_d_b3, assign11980_e11137_d_b4,) = {
    if ((var_guard121 != 0.0) && (var_guard122 == 0.0)) {
        let assign11980_e11121: f64 = (var_t0 - 0.02);
        let assign11980_e11124: f64 = (var_t0 - 0.02);
        let assign11980_e11127: f64 = (var_t0 - 0.02);
        let assign11980_e11128: f64 = (assign11980_e11124 * assign11980_e11127);
        let assign11980_e11131: f64 = (0.08 * var_vfbzb);
        let assign11980_e11132: f64 = (assign11980_e11128 + assign11980_e11131);
        let assign11980_e11133: f64 = (assign11980_e11132).sqrt();
        let assign11980_e11134: f64 = (assign11980_e11121 + assign11980_e11133);
        let assign11980_e11135: f64 = (0.5 * assign11980_e11134);
        (assign11980_e11135, (0.5 * (var_t0_dn0 + ((((var_t0_dn0 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn0)) + (0.08 * var_vfbzb_dn0)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_dn1 + ((((var_t0_dn1 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn1)) + (0.08 * var_vfbzb_dn1)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_dn2 + ((((var_t0_dn2 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn2)) + (0.08 * var_vfbzb_dn2)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_dn3 + ((((var_t0_dn3 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn3)) + (0.08 * var_vfbzb_dn3)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_dn4 + ((((var_t0_dn4 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn4)) + (0.08 * var_vfbzb_dn4)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_dn5 + ((((var_t0_dn5 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn5)) + (0.08 * var_vfbzb_dn5)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_dn6 + ((((var_t0_dn6 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn6)) + (0.08 * var_vfbzb_dn6)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_dn7 + ((((var_t0_dn7 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn7)) + (0.08 * var_vfbzb_dn7)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_dn8 + ((((var_t0_dn8 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn8)) + (0.08 * var_vfbzb_dn8)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_db0 + ((((var_t0_db0 * assign11980_e11127) + (assign11980_e11124 * var_t0_db0)) + (0.08 * var_vfbzb_db0)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_db1 + ((((var_t0_db1 * assign11980_e11127) + (assign11980_e11124 * var_t0_db1)) + (0.08 * var_vfbzb_db1)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_db2 + ((((var_t0_db2 * assign11980_e11127) + (assign11980_e11124 * var_t0_db2)) + (0.08 * var_vfbzb_db2)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_db3 + ((((var_t0_db3 * assign11980_e11127) + (assign11980_e11124 * var_t0_db3)) + (0.08 * var_vfbzb_db3)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_db4 + ((((var_t0_db4 * assign11980_e11127) + (assign11980_e11124 * var_t0_db4)) + (0.08 * var_vfbzb_db4)) / (2.0 * assign11980_e11133)))),)
    } else {
        (var_voxacc, var_voxacc_dn0, var_voxacc_dn1, var_voxacc_dn2, var_voxacc_dn3, var_voxacc_dn4, var_voxacc_dn5, var_voxacc_dn6, var_voxacc_dn7, var_voxacc_dn8, var_voxacc_db0, var_voxacc_db1, var_voxacc_db2, var_voxacc_db3, var_voxacc_db4,)
    }
};
        var_voxacc = assign11980_e11137;
        var_voxacc_dn0 = assign11980_e11137_d_n0;
        var_voxacc_dn1 = assign11980_e11137_d_n1;
        var_voxacc_dn2 = assign11980_e11137_d_n2;
        var_voxacc_dn3 = assign11980_e11137_d_n3;
        var_voxacc_dn4 = assign11980_e11137_d_n4;
        var_voxacc_dn5 = assign11980_e11137_d_n5;
        var_voxacc_dn6 = assign11980_e11137_d_n6;
        var_voxacc_dn7 = assign11980_e11137_d_n7;
        var_voxacc_dn8 = assign11980_e11137_d_n8;
        var_voxacc_db0 = assign11980_e11137_d_b0;
        var_voxacc_db1 = assign11980_e11137_d_b1;
        var_voxacc_db2 = assign11980_e11137_d_b2;
        var_voxacc_db3 = assign11980_e11137_d_b3;
        var_voxacc_db4 = assign11980_e11137_d_b4;
        var_voxacc_rv = 0.0;
        var_voxacc_rdn0 = 0.0;
        var_voxacc_rdn1 = 0.0;
        var_voxacc_rdn2 = 0.0;
        var_voxacc_rdn3 = 0.0;
        var_voxacc_rdn4 = 0.0;
        var_voxacc_rdn5 = 0.0;
        var_voxacc_rdn6 = 0.0;
        var_voxacc_rdn7 = 0.0;
        var_voxacc_rdn8 = 0.0;
        var_voxacc_rdb0 = 0.0;
        var_voxacc_rdb1 = 0.0;
        var_voxacc_rdb2 = 0.0;
        var_voxacc_rdb3 = 0.0;
        var_voxacc_rdb4 = 0.0;


        *var_guard122_slot = var_guard122;
        *var_guard122_db0_slot = var_guard122_db0;
        *var_guard122_db1_slot = var_guard122_db1;
        *var_guard122_db2_slot = var_guard122_db2;
        *var_guard122_db3_slot = var_guard122_db3;
        *var_guard122_db4_slot = var_guard122_db4;
        *var_guard122_dn0_slot = var_guard122_dn0;
        *var_guard122_dn1_slot = var_guard122_dn1;
        *var_guard122_dn2_slot = var_guard122_dn2;
        *var_guard122_dn3_slot = var_guard122_dn3;
        *var_guard122_dn4_slot = var_guard122_dn4;
        *var_guard122_dn5_slot = var_guard122_dn5;
        *var_guard122_dn6_slot = var_guard122_dn6;
        *var_guard122_dn7_slot = var_guard122_dn7;
        *var_guard122_dn8_slot = var_guard122_dn8;
        *var_guard122_rdb0_slot = var_guard122_rdb0;
        *var_guard122_rdb1_slot = var_guard122_rdb1;
        *var_guard122_rdb2_slot = var_guard122_rdb2;
        *var_guard122_rdb3_slot = var_guard122_rdb3;
        *var_guard122_rdb4_slot = var_guard122_rdb4;
        *var_guard122_rdn0_slot = var_guard122_rdn0;
        *var_guard122_rdn1_slot = var_guard122_rdn1;
        *var_guard122_rdn2_slot = var_guard122_rdn2;
        *var_guard122_rdn3_slot = var_guard122_rdn3;
        *var_guard122_rdn4_slot = var_guard122_rdn4;
        *var_guard122_rdn5_slot = var_guard122_rdn5;
        *var_guard122_rdn6_slot = var_guard122_rdn6;
        *var_guard122_rdn7_slot = var_guard122_rdn7;
        *var_guard122_rdn8_slot = var_guard122_rdn8;
        *var_guard122_rv_slot = var_guard122_rv;
        *var_t0_slot = var_t0;
        *var_t0_db0_slot = var_t0_db0;
        *var_t0_db1_slot = var_t0_db1;
        *var_t0_db2_slot = var_t0_db2;
        *var_t0_db3_slot = var_t0_db3;
        *var_t0_db4_slot = var_t0_db4;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rdb0_slot = var_t0_rdb0;
        *var_t0_rdb1_slot = var_t0_rdb1;
        *var_t0_rdb2_slot = var_t0_rdb2;
        *var_t0_rdb3_slot = var_t0_rdb3;
        *var_t0_rdb4_slot = var_t0_rdb4;
        *var_t0_rdn0_slot = var_t0_rdn0;
        *var_t0_rdn1_slot = var_t0_rdn1;
        *var_t0_rdn2_slot = var_t0_rdn2;
        *var_t0_rdn3_slot = var_t0_rdn3;
        *var_t0_rdn4_slot = var_t0_rdn4;
        *var_t0_rdn5_slot = var_t0_rdn5;
        *var_t0_rdn6_slot = var_t0_rdn6;
        *var_t0_rdn7_slot = var_t0_rdn7;
        *var_t0_rdn8_slot = var_t0_rdn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_db0_slot = var_t1_db0;
        *var_t1_db1_slot = var_t1_db1;
        *var_t1_db2_slot = var_t1_db2;
        *var_t1_db3_slot = var_t1_db3;
        *var_t1_db4_slot = var_t1_db4;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rdb0_slot = var_t1_rdb0;
        *var_t1_rdb1_slot = var_t1_rdb1;
        *var_t1_rdb2_slot = var_t1_rdb2;
        *var_t1_rdb3_slot = var_t1_rdb3;
        *var_t1_rdb4_slot = var_t1_rdb4;
        *var_t1_rdn0_slot = var_t1_rdn0;
        *var_t1_rdn1_slot = var_t1_rdn1;
        *var_t1_rdn2_slot = var_t1_rdn2;
        *var_t1_rdn3_slot = var_t1_rdn3;
        *var_t1_rdn4_slot = var_t1_rdn4;
        *var_t1_rdn5_slot = var_t1_rdn5;
        *var_t1_rdn6_slot = var_t1_rdn6;
        *var_t1_rdn7_slot = var_t1_rdn7;
        *var_t1_rdn8_slot = var_t1_rdn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_db0_slot = var_t2_db0;
        *var_t2_db1_slot = var_t2_db1;
        *var_t2_db2_slot = var_t2_db2;
        *var_t2_db3_slot = var_t2_db3;
        *var_t2_db4_slot = var_t2_db4;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rdb0_slot = var_t2_rdb0;
        *var_t2_rdb1_slot = var_t2_rdb1;
        *var_t2_rdb2_slot = var_t2_rdb2;
        *var_t2_rdb3_slot = var_t2_rdb3;
        *var_t2_rdb4_slot = var_t2_rdb4;
        *var_t2_rdn0_slot = var_t2_rdn0;
        *var_t2_rdn1_slot = var_t2_rdn1;
        *var_t2_rdn2_slot = var_t2_rdn2;
        *var_t2_rdn3_slot = var_t2_rdn3;
        *var_t2_rdn4_slot = var_t2_rdn4;
        *var_t2_rdn5_slot = var_t2_rdn5;
        *var_t2_rdn6_slot = var_t2_rdn6;
        *var_t2_rdn7_slot = var_t2_rdn7;
        *var_t2_rdn8_slot = var_t2_rdn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_db0_slot = var_t3_db0;
        *var_t3_db1_slot = var_t3_db1;
        *var_t3_db2_slot = var_t3_db2;
        *var_t3_db3_slot = var_t3_db3;
        *var_t3_db4_slot = var_t3_db4;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn1_slot = var_t3_dn1;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rdb0_slot = var_t3_rdb0;
        *var_t3_rdb1_slot = var_t3_rdb1;
        *var_t3_rdb2_slot = var_t3_rdb2;
        *var_t3_rdb3_slot = var_t3_rdb3;
        *var_t3_rdb4_slot = var_t3_rdb4;
        *var_t3_rdn0_slot = var_t3_rdn0;
        *var_t3_rdn1_slot = var_t3_rdn1;
        *var_t3_rdn2_slot = var_t3_rdn2;
        *var_t3_rdn3_slot = var_t3_rdn3;
        *var_t3_rdn4_slot = var_t3_rdn4;
        *var_t3_rdn5_slot = var_t3_rdn5;
        *var_t3_rdn6_slot = var_t3_rdn6;
        *var_t3_rdn7_slot = var_t3_rdn7;
        *var_t3_rdn8_slot = var_t3_rdn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_db0_slot = var_t4_db0;
        *var_t4_db1_slot = var_t4_db1;
        *var_t4_db2_slot = var_t4_db2;
        *var_t4_db3_slot = var_t4_db3;
        *var_t4_db4_slot = var_t4_db4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn1_slot = var_t4_dn1;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_rdb0_slot = var_t4_rdb0;
        *var_t4_rdb1_slot = var_t4_rdb1;
        *var_t4_rdb2_slot = var_t4_rdb2;
        *var_t4_rdb3_slot = var_t4_rdb3;
        *var_t4_rdb4_slot = var_t4_rdb4;
        *var_t4_rdn0_slot = var_t4_rdn0;
        *var_t4_rdn1_slot = var_t4_rdn1;
        *var_t4_rdn2_slot = var_t4_rdn2;
        *var_t4_rdn3_slot = var_t4_rdn3;
        *var_t4_rdn4_slot = var_t4_rdn4;
        *var_t4_rdn5_slot = var_t4_rdn5;
        *var_t4_rdn6_slot = var_t4_rdn6;
        *var_t4_rdn7_slot = var_t4_rdn7;
        *var_t4_rdn8_slot = var_t4_rdn8;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_db0_slot = var_t5_db0;
        *var_t5_db1_slot = var_t5_db1;
        *var_t5_db2_slot = var_t5_db2;
        *var_t5_db3_slot = var_t5_db3;
        *var_t5_db4_slot = var_t5_db4;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn1_slot = var_t5_dn1;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_rdb0_slot = var_t5_rdb0;
        *var_t5_rdb1_slot = var_t5_rdb1;
        *var_t5_rdb2_slot = var_t5_rdb2;
        *var_t5_rdb3_slot = var_t5_rdb3;
        *var_t5_rdb4_slot = var_t5_rdb4;
        *var_t5_rdn0_slot = var_t5_rdn0;
        *var_t5_rdn1_slot = var_t5_rdn1;
        *var_t5_rdn2_slot = var_t5_rdn2;
        *var_t5_rdn3_slot = var_t5_rdn3;
        *var_t5_rdn4_slot = var_t5_rdn4;
        *var_t5_rdn5_slot = var_t5_rdn5;
        *var_t5_rdn6_slot = var_t5_rdn6;
        *var_t5_rdn7_slot = var_t5_rdn7;
        *var_t5_rdn8_slot = var_t5_rdn8;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_db0_slot = var_t6_db0;
        *var_t6_db1_slot = var_t6_db1;
        *var_t6_db2_slot = var_t6_db2;
        *var_t6_db3_slot = var_t6_db3;
        *var_t6_db4_slot = var_t6_db4;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn1_slot = var_t6_dn1;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_rdb0_slot = var_t6_rdb0;
        *var_t6_rdb1_slot = var_t6_rdb1;
        *var_t6_rdb2_slot = var_t6_rdb2;
        *var_t6_rdb3_slot = var_t6_rdb3;
        *var_t6_rdb4_slot = var_t6_rdb4;
        *var_t6_rdn0_slot = var_t6_rdn0;
        *var_t6_rdn1_slot = var_t6_rdn1;
        *var_t6_rdn2_slot = var_t6_rdn2;
        *var_t6_rdn3_slot = var_t6_rdn3;
        *var_t6_rdn4_slot = var_t6_rdn4;
        *var_t6_rdn5_slot = var_t6_rdn5;
        *var_t6_rdn6_slot = var_t6_rdn6;
        *var_t6_rdn7_slot = var_t6_rdn7;
        *var_t6_rdn8_slot = var_t6_rdn8;
        *var_t6_rv_slot = var_t6_rv;
        *var_vfbzb_slot = var_vfbzb;
        *var_vfbzb_db0_slot = var_vfbzb_db0;
        *var_vfbzb_db1_slot = var_vfbzb_db1;
        *var_vfbzb_db2_slot = var_vfbzb_db2;
        *var_vfbzb_db3_slot = var_vfbzb_db3;
        *var_vfbzb_db4_slot = var_vfbzb_db4;
        *var_vfbzb_dn0_slot = var_vfbzb_dn0;
        *var_vfbzb_dn1_slot = var_vfbzb_dn1;
        *var_vfbzb_dn2_slot = var_vfbzb_dn2;
        *var_vfbzb_dn3_slot = var_vfbzb_dn3;
        *var_vfbzb_dn4_slot = var_vfbzb_dn4;
        *var_vfbzb_dn5_slot = var_vfbzb_dn5;
        *var_vfbzb_dn6_slot = var_vfbzb_dn6;
        *var_vfbzb_dn7_slot = var_vfbzb_dn7;
        *var_vfbzb_dn8_slot = var_vfbzb_dn8;
        *var_vfbzb_rdb0_slot = var_vfbzb_rdb0;
        *var_vfbzb_rdb1_slot = var_vfbzb_rdb1;
        *var_vfbzb_rdb2_slot = var_vfbzb_rdb2;
        *var_vfbzb_rdb3_slot = var_vfbzb_rdb3;
        *var_vfbzb_rdb4_slot = var_vfbzb_rdb4;
        *var_vfbzb_rdn0_slot = var_vfbzb_rdn0;
        *var_vfbzb_rdn1_slot = var_vfbzb_rdn1;
        *var_vfbzb_rdn2_slot = var_vfbzb_rdn2;
        *var_vfbzb_rdn3_slot = var_vfbzb_rdn3;
        *var_vfbzb_rdn4_slot = var_vfbzb_rdn4;
        *var_vfbzb_rdn5_slot = var_vfbzb_rdn5;
        *var_vfbzb_rdn6_slot = var_vfbzb_rdn6;
        *var_vfbzb_rdn7_slot = var_vfbzb_rdn7;
        *var_vfbzb_rdn8_slot = var_vfbzb_rdn8;
        *var_vfbzb_rv_slot = var_vfbzb_rv;
        *var_voxacc_slot = var_voxacc;
        *var_voxacc_db0_slot = var_voxacc_db0;
        *var_voxacc_db1_slot = var_voxacc_db1;
        *var_voxacc_db2_slot = var_voxacc_db2;
        *var_voxacc_db3_slot = var_voxacc_db3;
        *var_voxacc_db4_slot = var_voxacc_db4;
        *var_voxacc_dn0_slot = var_voxacc_dn0;
        *var_voxacc_dn1_slot = var_voxacc_dn1;
        *var_voxacc_dn2_slot = var_voxacc_dn2;
        *var_voxacc_dn3_slot = var_voxacc_dn3;
        *var_voxacc_dn4_slot = var_voxacc_dn4;
        *var_voxacc_dn5_slot = var_voxacc_dn5;
        *var_voxacc_dn6_slot = var_voxacc_dn6;
        *var_voxacc_dn7_slot = var_voxacc_dn7;
        *var_voxacc_dn8_slot = var_voxacc_dn8;
        *var_voxacc_rdb0_slot = var_voxacc_rdb0;
        *var_voxacc_rdb1_slot = var_voxacc_rdb1;
        *var_voxacc_rdb2_slot = var_voxacc_rdb2;
        *var_voxacc_rdb3_slot = var_voxacc_rdb3;
        *var_voxacc_rdb4_slot = var_voxacc_rdb4;
        *var_voxacc_rdn0_slot = var_voxacc_rdn0;
        *var_voxacc_rdn1_slot = var_voxacc_rdn1;
        *var_voxacc_rdn2_slot = var_voxacc_rdn2;
        *var_voxacc_rdn3_slot = var_voxacc_rdn3;
        *var_voxacc_rdn4_slot = var_voxacc_rdn4;
        *var_voxacc_rdn5_slot = var_voxacc_rdn5;
        *var_voxacc_rdn6_slot = var_voxacc_rdn6;
        *var_voxacc_rdn7_slot = var_voxacc_rdn7;
        *var_voxacc_rdn8_slot = var_voxacc_rdn8;
        *var_voxacc_rv_slot = var_voxacc_rv;
    }

    pub(super) fn stamp_reactive_block_83(
        p: &Parameters,
        var_aigbacc_i: f64,
        var_aigbacc_i_db0: f64,
        var_aigbacc_i_db1: f64,
        var_aigbacc_i_db2: f64,
        var_aigbacc_i_db3: f64,
        var_aigbacc_i_db4: f64,
        var_aigbacc_i_dn0: f64,
        var_aigbacc_i_dn1: f64,
        var_aigbacc_i_dn2: f64,
        var_aigbacc_i_dn3: f64,
        var_aigbacc_i_dn4: f64,
        var_aigbacc_i_dn5: f64,
        var_aigbacc_i_dn6: f64,
        var_aigbacc_i_dn7: f64,
        var_aigbacc_i_dn8: f64,
        var_aigc_i: f64,
        var_aigc_i_db0: f64,
        var_aigc_i_db1: f64,
        var_aigc_i_db2: f64,
        var_aigc_i_db3: f64,
        var_aigc_i_db4: f64,
        var_aigc_i_dn0: f64,
        var_aigc_i_dn1: f64,
        var_aigc_i_dn2: f64,
        var_aigc_i_dn3: f64,
        var_aigc_i_dn4: f64,
        var_aigc_i_dn5: f64,
        var_aigc_i_dn6: f64,
        var_aigc_i_dn7: f64,
        var_aigc_i_dn8: f64,
        var_bechvb: f64,
        var_bechvb_db0: f64,
        var_bechvb_db1: f64,
        var_bechvb_db2: f64,
        var_bechvb_db3: f64,
        var_bechvb_db4: f64,
        var_bechvb_dn0: f64,
        var_bechvb_dn1: f64,
        var_bechvb_dn2: f64,
        var_bechvb_dn3: f64,
        var_bechvb_dn4: f64,
        var_bechvb_dn5: f64,
        var_bechvb_dn6: f64,
        var_bechvb_dn7: f64,
        var_bechvb_dn8: f64,
        var_bigbacc_i: f64,
        var_bigbacc_i_db0: f64,
        var_bigbacc_i_db1: f64,
        var_bigbacc_i_db2: f64,
        var_bigbacc_i_db3: f64,
        var_bigbacc_i_db4: f64,
        var_bigbacc_i_dn0: f64,
        var_bigbacc_i_dn1: f64,
        var_bigbacc_i_dn2: f64,
        var_bigbacc_i_dn3: f64,
        var_bigbacc_i_dn4: f64,
        var_bigbacc_i_dn5: f64,
        var_bigbacc_i_dn6: f64,
        var_bigbacc_i_dn7: f64,
        var_bigbacc_i_dn8: f64,
        var_bigc_i: f64,
        var_bigc_i_db0: f64,
        var_bigc_i_db1: f64,
        var_bigc_i_db2: f64,
        var_bigc_i_db3: f64,
        var_bigc_i_db4: f64,
        var_bigc_i_dn0: f64,
        var_bigc_i_dn1: f64,
        var_bigc_i_dn2: f64,
        var_bigc_i_dn3: f64,
        var_bigc_i_dn4: f64,
        var_bigc_i_dn5: f64,
        var_bigc_i_dn6: f64,
        var_bigc_i_dn7: f64,
        var_bigc_i_dn8: f64,
        var_cigbacc_i: f64,
        var_cigbacc_i_db0: f64,
        var_cigbacc_i_db1: f64,
        var_cigbacc_i_db2: f64,
        var_cigbacc_i_db3: f64,
        var_cigbacc_i_db4: f64,
        var_cigbacc_i_dn0: f64,
        var_cigbacc_i_dn1: f64,
        var_cigbacc_i_dn2: f64,
        var_cigbacc_i_dn3: f64,
        var_cigbacc_i_dn4: f64,
        var_cigbacc_i_dn5: f64,
        var_cigbacc_i_dn6: f64,
        var_cigbacc_i_dn7: f64,
        var_cigbacc_i_dn8: f64,
        var_cigc_i: f64,
        var_cigc_i_db0: f64,
        var_cigc_i_db1: f64,
        var_cigc_i_db2: f64,
        var_cigc_i_db3: f64,
        var_cigc_i_db4: f64,
        var_cigc_i_dn0: f64,
        var_cigc_i_dn1: f64,
        var_cigc_i_dn2: f64,
        var_cigc_i_dn3: f64,
        var_cigc_i_dn4: f64,
        var_cigc_i_dn5: f64,
        var_cigc_i_dn6: f64,
        var_cigc_i_dn7: f64,
        var_cigc_i_dn8: f64,
        var_digc_i: f64,
        var_digc_i_db0: f64,
        var_digc_i_db1: f64,
        var_digc_i_db2: f64,
        var_digc_i_db3: f64,
        var_digc_i_db4: f64,
        var_digc_i_dn0: f64,
        var_digc_i_dn1: f64,
        var_digc_i_dn2: f64,
        var_digc_i_dn3: f64,
        var_digc_i_dn4: f64,
        var_digc_i_dn5: f64,
        var_digc_i_dn6: f64,
        var_digc_i_dn7: f64,
        var_digc_i_dn8: f64,
        var_guard121: f64,
        var_phifs: f64,
        var_phifs_db0: f64,
        var_phifs_db1: f64,
        var_phifs_db2: f64,
        var_phifs_db3: f64,
        var_phifs_db4: f64,
        var_phifs_dn0: f64,
        var_phifs_dn1: f64,
        var_phifs_dn2: f64,
        var_phifs_dn3: f64,
        var_phifs_dn4: f64,
        var_phifs_dn5: f64,
        var_phifs_dn6: f64,
        var_phifs_dn7: f64,
        var_phifs_dn8: f64,
        var_qia: f64,
        var_qia_db0: f64,
        var_qia_db1: f64,
        var_qia_db2: f64,
        var_qia_db3: f64,
        var_qia_db4: f64,
        var_qia_dn0: f64,
        var_qia_dn1: f64,
        var_qia_dn2: f64,
        var_qia_dn3: f64,
        var_qia_dn4: f64,
        var_qia_dn5: f64,
        var_qia_dn6: f64,
        var_qia_dn7: f64,
        var_qia_dn8: f64,
        var_vbgd_noswap: f64,
        var_vbgd_noswap_db0: f64,
        var_vbgd_noswap_db1: f64,
        var_vbgd_noswap_db2: f64,
        var_vbgd_noswap_db3: f64,
        var_vbgd_noswap_db4: f64,
        var_vbgd_noswap_dn0: f64,
        var_vbgd_noswap_dn1: f64,
        var_vbgd_noswap_dn2: f64,
        var_vbgd_noswap_dn3: f64,
        var_vbgd_noswap_dn4: f64,
        var_vbgd_noswap_dn5: f64,
        var_vbgd_noswap_dn6: f64,
        var_vbgd_noswap_dn7: f64,
        var_vbgd_noswap_dn8: f64,
        var_vbgs_noswap: f64,
        var_vbgs_noswap_db0: f64,
        var_vbgs_noswap_db1: f64,
        var_vbgs_noswap_db2: f64,
        var_vbgs_noswap_db3: f64,
        var_vbgs_noswap_db4: f64,
        var_vbgs_noswap_dn0: f64,
        var_vbgs_noswap_dn1: f64,
        var_vbgs_noswap_dn2: f64,
        var_vbgs_noswap_dn3: f64,
        var_vbgs_noswap_dn4: f64,
        var_vbgs_noswap_dn5: f64,
        var_vbgs_noswap_dn6: f64,
        var_vbgs_noswap_dn7: f64,
        var_vbgs_noswap_dn8: f64,
        var_vds_noswap: f64,
        var_vds_noswap_db0: f64,
        var_vds_noswap_db1: f64,
        var_vds_noswap_db2: f64,
        var_vds_noswap_db3: f64,
        var_vds_noswap_db4: f64,
        var_vds_noswap_dn0: f64,
        var_vds_noswap_dn1: f64,
        var_vds_noswap_dn2: f64,
        var_vds_noswap_dn3: f64,
        var_vds_noswap_dn4: f64,
        var_vds_noswap_dn5: f64,
        var_vds_noswap_dn6: f64,
        var_vds_noswap_dn7: f64,
        var_vds_noswap_dn8: f64,
        var_vdsx: f64,
        var_vdsx_db0: f64,
        var_vdsx_db1: f64,
        var_vdsx_db2: f64,
        var_vdsx_db3: f64,
        var_vdsx_db4: f64,
        var_vdsx_dn0: f64,
        var_vdsx_dn1: f64,
        var_vdsx_dn2: f64,
        var_vdsx_dn3: f64,
        var_vdsx_dn4: f64,
        var_vdsx_dn5: f64,
        var_vdsx_dn6: f64,
        var_vdsx_dn7: f64,
        var_vdsx_dn8: f64,
        var_vgbg: f64,
        var_vgbg_db0: f64,
        var_vgbg_db1: f64,
        var_vgbg_db2: f64,
        var_vgbg_db3: f64,
        var_vgbg_db4: f64,
        var_vgbg_dn0: f64,
        var_vgbg_dn1: f64,
        var_vgbg_dn2: f64,
        var_vgbg_dn3: f64,
        var_vgbg_dn4: f64,
        var_vgbg_dn5: f64,
        var_vgbg_dn6: f64,
        var_vgbg_dn7: f64,
        var_vgbg_dn8: f64,
        var_vgfb1: f64,
        var_vgfb1_db0: f64,
        var_vgfb1_db1: f64,
        var_vgfb1_db2: f64,
        var_vgfb1_db3: f64,
        var_vgfb1_db4: f64,
        var_vgfb1_dn0: f64,
        var_vgfb1_dn1: f64,
        var_vgfb1_dn2: f64,
        var_vgfb1_dn3: f64,
        var_vgfb1_dn4: f64,
        var_vgfb1_dn5: f64,
        var_vgfb1_dn6: f64,
        var_vgfb1_dn7: f64,
        var_vgfb1_dn8: f64,
        var_voxacc: f64,
        var_voxacc_db0: f64,
        var_voxacc_db1: f64,
        var_voxacc_db2: f64,
        var_voxacc_db3: f64,
        var_voxacc_db4: f64,
        var_voxacc_dn0: f64,
        var_voxacc_dn1: f64,
        var_voxacc_dn2: f64,
        var_voxacc_dn3: f64,
        var_voxacc_dn4: f64,
        var_voxacc_dn5: f64,
        var_voxacc_dn6: f64,
        var_voxacc_dn7: f64,
        var_voxacc_dn8: f64,
        var_vtm: f64,
        var_vtm_db0: f64,
        var_vtm_db1: f64,
        var_vtm_db2: f64,
        var_vtm_db3: f64,
        var_vtm_db4: f64,
        var_vtm_dn0: f64,
        var_vtm_dn1: f64,
        var_vtm_dn2: f64,
        var_vtm_dn3: f64,
        var_vtm_dn4: f64,
        var_vtm_dn5: f64,
        var_vtm_dn6: f64,
        var_vtm_dn7: f64,
        var_vtm_dn8: f64,
        var_guard123_slot: &mut f64,
        var_guard123_db0_slot: &mut f64,
        var_guard123_db1_slot: &mut f64,
        var_guard123_db2_slot: &mut f64,
        var_guard123_db3_slot: &mut f64,
        var_guard123_db4_slot: &mut f64,
        var_guard123_dn0_slot: &mut f64,
        var_guard123_dn1_slot: &mut f64,
        var_guard123_dn2_slot: &mut f64,
        var_guard123_dn3_slot: &mut f64,
        var_guard123_dn4_slot: &mut f64,
        var_guard123_dn5_slot: &mut f64,
        var_guard123_dn6_slot: &mut f64,
        var_guard123_dn7_slot: &mut f64,
        var_guard123_dn8_slot: &mut f64,
        var_guard123_rdb0_slot: &mut f64,
        var_guard123_rdb1_slot: &mut f64,
        var_guard123_rdb2_slot: &mut f64,
        var_guard123_rdb3_slot: &mut f64,
        var_guard123_rdb4_slot: &mut f64,
        var_guard123_rdn0_slot: &mut f64,
        var_guard123_rdn1_slot: &mut f64,
        var_guard123_rdn2_slot: &mut f64,
        var_guard123_rdn3_slot: &mut f64,
        var_guard123_rdn4_slot: &mut f64,
        var_guard123_rdn5_slot: &mut f64,
        var_guard123_rdn6_slot: &mut f64,
        var_guard123_rdn7_slot: &mut f64,
        var_guard123_rdn8_slot: &mut f64,
        var_guard123_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_db0_slot: &mut f64,
        var_t0_db1_slot: &mut f64,
        var_t0_db2_slot: &mut f64,
        var_t0_db3_slot: &mut f64,
        var_t0_db4_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rdb0_slot: &mut f64,
        var_t0_rdb1_slot: &mut f64,
        var_t0_rdb2_slot: &mut f64,
        var_t0_rdb3_slot: &mut f64,
        var_t0_rdb4_slot: &mut f64,
        var_t0_rdn0_slot: &mut f64,
        var_t0_rdn1_slot: &mut f64,
        var_t0_rdn2_slot: &mut f64,
        var_t0_rdn3_slot: &mut f64,
        var_t0_rdn4_slot: &mut f64,
        var_t0_rdn5_slot: &mut f64,
        var_t0_rdn6_slot: &mut f64,
        var_t0_rdn7_slot: &mut f64,
        var_t0_rdn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_db0_slot: &mut f64,
        var_t1_db1_slot: &mut f64,
        var_t1_db2_slot: &mut f64,
        var_t1_db3_slot: &mut f64,
        var_t1_db4_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rdb0_slot: &mut f64,
        var_t1_rdb1_slot: &mut f64,
        var_t1_rdb2_slot: &mut f64,
        var_t1_rdb3_slot: &mut f64,
        var_t1_rdb4_slot: &mut f64,
        var_t1_rdn0_slot: &mut f64,
        var_t1_rdn1_slot: &mut f64,
        var_t1_rdn2_slot: &mut f64,
        var_t1_rdn3_slot: &mut f64,
        var_t1_rdn4_slot: &mut f64,
        var_t1_rdn5_slot: &mut f64,
        var_t1_rdn6_slot: &mut f64,
        var_t1_rdn7_slot: &mut f64,
        var_t1_rdn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_db0_slot: &mut f64,
        var_t2_db1_slot: &mut f64,
        var_t2_db2_slot: &mut f64,
        var_t2_db3_slot: &mut f64,
        var_t2_db4_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rdb0_slot: &mut f64,
        var_t2_rdb1_slot: &mut f64,
        var_t2_rdb2_slot: &mut f64,
        var_t2_rdb3_slot: &mut f64,
        var_t2_rdb4_slot: &mut f64,
        var_t2_rdn0_slot: &mut f64,
        var_t2_rdn1_slot: &mut f64,
        var_t2_rdn2_slot: &mut f64,
        var_t2_rdn3_slot: &mut f64,
        var_t2_rdn4_slot: &mut f64,
        var_t2_rdn5_slot: &mut f64,
        var_t2_rdn6_slot: &mut f64,
        var_t2_rdn7_slot: &mut f64,
        var_t2_rdn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_db0_slot: &mut f64,
        var_t3_db1_slot: &mut f64,
        var_t3_db2_slot: &mut f64,
        var_t3_db3_slot: &mut f64,
        var_t3_db4_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn1_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rdb0_slot: &mut f64,
        var_t3_rdb1_slot: &mut f64,
        var_t3_rdb2_slot: &mut f64,
        var_t3_rdb3_slot: &mut f64,
        var_t3_rdb4_slot: &mut f64,
        var_t3_rdn0_slot: &mut f64,
        var_t3_rdn1_slot: &mut f64,
        var_t3_rdn2_slot: &mut f64,
        var_t3_rdn3_slot: &mut f64,
        var_t3_rdn4_slot: &mut f64,
        var_t3_rdn5_slot: &mut f64,
        var_t3_rdn6_slot: &mut f64,
        var_t3_rdn7_slot: &mut f64,
        var_t3_rdn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_db0_slot: &mut f64,
        var_t4_db1_slot: &mut f64,
        var_t4_db2_slot: &mut f64,
        var_t4_db3_slot: &mut f64,
        var_t4_db4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn1_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_rdb0_slot: &mut f64,
        var_t4_rdb1_slot: &mut f64,
        var_t4_rdb2_slot: &mut f64,
        var_t4_rdb3_slot: &mut f64,
        var_t4_rdb4_slot: &mut f64,
        var_t4_rdn0_slot: &mut f64,
        var_t4_rdn1_slot: &mut f64,
        var_t4_rdn2_slot: &mut f64,
        var_t4_rdn3_slot: &mut f64,
        var_t4_rdn4_slot: &mut f64,
        var_t4_rdn5_slot: &mut f64,
        var_t4_rdn6_slot: &mut f64,
        var_t4_rdn7_slot: &mut f64,
        var_t4_rdn8_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_db0_slot: &mut f64,
        var_t5_db1_slot: &mut f64,
        var_t5_db2_slot: &mut f64,
        var_t5_db3_slot: &mut f64,
        var_t5_db4_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn1_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_rdb0_slot: &mut f64,
        var_t5_rdb1_slot: &mut f64,
        var_t5_rdb2_slot: &mut f64,
        var_t5_rdb3_slot: &mut f64,
        var_t5_rdb4_slot: &mut f64,
        var_t5_rdn0_slot: &mut f64,
        var_t5_rdn1_slot: &mut f64,
        var_t5_rdn2_slot: &mut f64,
        var_t5_rdn3_slot: &mut f64,
        var_t5_rdn4_slot: &mut f64,
        var_t5_rdn5_slot: &mut f64,
        var_t5_rdn6_slot: &mut f64,
        var_t5_rdn7_slot: &mut f64,
        var_t5_rdn8_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_db0_slot: &mut f64,
        var_t6_db1_slot: &mut f64,
        var_t6_db2_slot: &mut f64,
        var_t6_db3_slot: &mut f64,
        var_t6_db4_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn1_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_rdb0_slot: &mut f64,
        var_t6_rdb1_slot: &mut f64,
        var_t6_rdb2_slot: &mut f64,
        var_t6_rdb3_slot: &mut f64,
        var_t6_rdb4_slot: &mut f64,
        var_t6_rdn0_slot: &mut f64,
        var_t6_rdn1_slot: &mut f64,
        var_t6_rdn2_slot: &mut f64,
        var_t6_rdn3_slot: &mut f64,
        var_t6_rdn4_slot: &mut f64,
        var_t6_rdn5_slot: &mut f64,
        var_t6_rdn6_slot: &mut f64,
        var_t6_rdn7_slot: &mut f64,
        var_t6_rdn8_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
    ) {
        let mut var_guard123: f64 = *var_guard123_slot;
        let mut var_guard123_db0: f64 = *var_guard123_db0_slot;
        let mut var_guard123_db1: f64 = *var_guard123_db1_slot;
        let mut var_guard123_db2: f64 = *var_guard123_db2_slot;
        let mut var_guard123_db3: f64 = *var_guard123_db3_slot;
        let mut var_guard123_db4: f64 = *var_guard123_db4_slot;
        let mut var_guard123_dn0: f64 = *var_guard123_dn0_slot;
        let mut var_guard123_dn1: f64 = *var_guard123_dn1_slot;
        let mut var_guard123_dn2: f64 = *var_guard123_dn2_slot;
        let mut var_guard123_dn3: f64 = *var_guard123_dn3_slot;
        let mut var_guard123_dn4: f64 = *var_guard123_dn4_slot;
        let mut var_guard123_dn5: f64 = *var_guard123_dn5_slot;
        let mut var_guard123_dn6: f64 = *var_guard123_dn6_slot;
        let mut var_guard123_dn7: f64 = *var_guard123_dn7_slot;
        let mut var_guard123_dn8: f64 = *var_guard123_dn8_slot;
        let mut var_guard123_rdb0: f64 = *var_guard123_rdb0_slot;
        let mut var_guard123_rdb1: f64 = *var_guard123_rdb1_slot;
        let mut var_guard123_rdb2: f64 = *var_guard123_rdb2_slot;
        let mut var_guard123_rdb3: f64 = *var_guard123_rdb3_slot;
        let mut var_guard123_rdb4: f64 = *var_guard123_rdb4_slot;
        let mut var_guard123_rdn0: f64 = *var_guard123_rdn0_slot;
        let mut var_guard123_rdn1: f64 = *var_guard123_rdn1_slot;
        let mut var_guard123_rdn2: f64 = *var_guard123_rdn2_slot;
        let mut var_guard123_rdn3: f64 = *var_guard123_rdn3_slot;
        let mut var_guard123_rdn4: f64 = *var_guard123_rdn4_slot;
        let mut var_guard123_rdn5: f64 = *var_guard123_rdn5_slot;
        let mut var_guard123_rdn6: f64 = *var_guard123_rdn6_slot;
        let mut var_guard123_rdn7: f64 = *var_guard123_rdn7_slot;
        let mut var_guard123_rdn8: f64 = *var_guard123_rdn8_slot;
        let mut var_guard123_rv: f64 = *var_guard123_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_db0: f64 = *var_t0_db0_slot;
        let mut var_t0_db1: f64 = *var_t0_db1_slot;
        let mut var_t0_db2: f64 = *var_t0_db2_slot;
        let mut var_t0_db3: f64 = *var_t0_db3_slot;
        let mut var_t0_db4: f64 = *var_t0_db4_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rdb0: f64 = *var_t0_rdb0_slot;
        let mut var_t0_rdb1: f64 = *var_t0_rdb1_slot;
        let mut var_t0_rdb2: f64 = *var_t0_rdb2_slot;
        let mut var_t0_rdb3: f64 = *var_t0_rdb3_slot;
        let mut var_t0_rdb4: f64 = *var_t0_rdb4_slot;
        let mut var_t0_rdn0: f64 = *var_t0_rdn0_slot;
        let mut var_t0_rdn1: f64 = *var_t0_rdn1_slot;
        let mut var_t0_rdn2: f64 = *var_t0_rdn2_slot;
        let mut var_t0_rdn3: f64 = *var_t0_rdn3_slot;
        let mut var_t0_rdn4: f64 = *var_t0_rdn4_slot;
        let mut var_t0_rdn5: f64 = *var_t0_rdn5_slot;
        let mut var_t0_rdn6: f64 = *var_t0_rdn6_slot;
        let mut var_t0_rdn7: f64 = *var_t0_rdn7_slot;
        let mut var_t0_rdn8: f64 = *var_t0_rdn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_db0: f64 = *var_t1_db0_slot;
        let mut var_t1_db1: f64 = *var_t1_db1_slot;
        let mut var_t1_db2: f64 = *var_t1_db2_slot;
        let mut var_t1_db3: f64 = *var_t1_db3_slot;
        let mut var_t1_db4: f64 = *var_t1_db4_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rdb0: f64 = *var_t1_rdb0_slot;
        let mut var_t1_rdb1: f64 = *var_t1_rdb1_slot;
        let mut var_t1_rdb2: f64 = *var_t1_rdb2_slot;
        let mut var_t1_rdb3: f64 = *var_t1_rdb3_slot;
        let mut var_t1_rdb4: f64 = *var_t1_rdb4_slot;
        let mut var_t1_rdn0: f64 = *var_t1_rdn0_slot;
        let mut var_t1_rdn1: f64 = *var_t1_rdn1_slot;
        let mut var_t1_rdn2: f64 = *var_t1_rdn2_slot;
        let mut var_t1_rdn3: f64 = *var_t1_rdn3_slot;
        let mut var_t1_rdn4: f64 = *var_t1_rdn4_slot;
        let mut var_t1_rdn5: f64 = *var_t1_rdn5_slot;
        let mut var_t1_rdn6: f64 = *var_t1_rdn6_slot;
        let mut var_t1_rdn7: f64 = *var_t1_rdn7_slot;
        let mut var_t1_rdn8: f64 = *var_t1_rdn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_db0: f64 = *var_t2_db0_slot;
        let mut var_t2_db1: f64 = *var_t2_db1_slot;
        let mut var_t2_db2: f64 = *var_t2_db2_slot;
        let mut var_t2_db3: f64 = *var_t2_db3_slot;
        let mut var_t2_db4: f64 = *var_t2_db4_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rdb0: f64 = *var_t2_rdb0_slot;
        let mut var_t2_rdb1: f64 = *var_t2_rdb1_slot;
        let mut var_t2_rdb2: f64 = *var_t2_rdb2_slot;
        let mut var_t2_rdb3: f64 = *var_t2_rdb3_slot;
        let mut var_t2_rdb4: f64 = *var_t2_rdb4_slot;
        let mut var_t2_rdn0: f64 = *var_t2_rdn0_slot;
        let mut var_t2_rdn1: f64 = *var_t2_rdn1_slot;
        let mut var_t2_rdn2: f64 = *var_t2_rdn2_slot;
        let mut var_t2_rdn3: f64 = *var_t2_rdn3_slot;
        let mut var_t2_rdn4: f64 = *var_t2_rdn4_slot;
        let mut var_t2_rdn5: f64 = *var_t2_rdn5_slot;
        let mut var_t2_rdn6: f64 = *var_t2_rdn6_slot;
        let mut var_t2_rdn7: f64 = *var_t2_rdn7_slot;
        let mut var_t2_rdn8: f64 = *var_t2_rdn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_db0: f64 = *var_t3_db0_slot;
        let mut var_t3_db1: f64 = *var_t3_db1_slot;
        let mut var_t3_db2: f64 = *var_t3_db2_slot;
        let mut var_t3_db3: f64 = *var_t3_db3_slot;
        let mut var_t3_db4: f64 = *var_t3_db4_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn1: f64 = *var_t3_dn1_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rdb0: f64 = *var_t3_rdb0_slot;
        let mut var_t3_rdb1: f64 = *var_t3_rdb1_slot;
        let mut var_t3_rdb2: f64 = *var_t3_rdb2_slot;
        let mut var_t3_rdb3: f64 = *var_t3_rdb3_slot;
        let mut var_t3_rdb4: f64 = *var_t3_rdb4_slot;
        let mut var_t3_rdn0: f64 = *var_t3_rdn0_slot;
        let mut var_t3_rdn1: f64 = *var_t3_rdn1_slot;
        let mut var_t3_rdn2: f64 = *var_t3_rdn2_slot;
        let mut var_t3_rdn3: f64 = *var_t3_rdn3_slot;
        let mut var_t3_rdn4: f64 = *var_t3_rdn4_slot;
        let mut var_t3_rdn5: f64 = *var_t3_rdn5_slot;
        let mut var_t3_rdn6: f64 = *var_t3_rdn6_slot;
        let mut var_t3_rdn7: f64 = *var_t3_rdn7_slot;
        let mut var_t3_rdn8: f64 = *var_t3_rdn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_db0: f64 = *var_t4_db0_slot;
        let mut var_t4_db1: f64 = *var_t4_db1_slot;
        let mut var_t4_db2: f64 = *var_t4_db2_slot;
        let mut var_t4_db3: f64 = *var_t4_db3_slot;
        let mut var_t4_db4: f64 = *var_t4_db4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn1: f64 = *var_t4_dn1_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_rdb0: f64 = *var_t4_rdb0_slot;
        let mut var_t4_rdb1: f64 = *var_t4_rdb1_slot;
        let mut var_t4_rdb2: f64 = *var_t4_rdb2_slot;
        let mut var_t4_rdb3: f64 = *var_t4_rdb3_slot;
        let mut var_t4_rdb4: f64 = *var_t4_rdb4_slot;
        let mut var_t4_rdn0: f64 = *var_t4_rdn0_slot;
        let mut var_t4_rdn1: f64 = *var_t4_rdn1_slot;
        let mut var_t4_rdn2: f64 = *var_t4_rdn2_slot;
        let mut var_t4_rdn3: f64 = *var_t4_rdn3_slot;
        let mut var_t4_rdn4: f64 = *var_t4_rdn4_slot;
        let mut var_t4_rdn5: f64 = *var_t4_rdn5_slot;
        let mut var_t4_rdn6: f64 = *var_t4_rdn6_slot;
        let mut var_t4_rdn7: f64 = *var_t4_rdn7_slot;
        let mut var_t4_rdn8: f64 = *var_t4_rdn8_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_db0: f64 = *var_t5_db0_slot;
        let mut var_t5_db1: f64 = *var_t5_db1_slot;
        let mut var_t5_db2: f64 = *var_t5_db2_slot;
        let mut var_t5_db3: f64 = *var_t5_db3_slot;
        let mut var_t5_db4: f64 = *var_t5_db4_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn1: f64 = *var_t5_dn1_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_rdb0: f64 = *var_t5_rdb0_slot;
        let mut var_t5_rdb1: f64 = *var_t5_rdb1_slot;
        let mut var_t5_rdb2: f64 = *var_t5_rdb2_slot;
        let mut var_t5_rdb3: f64 = *var_t5_rdb3_slot;
        let mut var_t5_rdb4: f64 = *var_t5_rdb4_slot;
        let mut var_t5_rdn0: f64 = *var_t5_rdn0_slot;
        let mut var_t5_rdn1: f64 = *var_t5_rdn1_slot;
        let mut var_t5_rdn2: f64 = *var_t5_rdn2_slot;
        let mut var_t5_rdn3: f64 = *var_t5_rdn3_slot;
        let mut var_t5_rdn4: f64 = *var_t5_rdn4_slot;
        let mut var_t5_rdn5: f64 = *var_t5_rdn5_slot;
        let mut var_t5_rdn6: f64 = *var_t5_rdn6_slot;
        let mut var_t5_rdn7: f64 = *var_t5_rdn7_slot;
        let mut var_t5_rdn8: f64 = *var_t5_rdn8_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_db0: f64 = *var_t6_db0_slot;
        let mut var_t6_db1: f64 = *var_t6_db1_slot;
        let mut var_t6_db2: f64 = *var_t6_db2_slot;
        let mut var_t6_db3: f64 = *var_t6_db3_slot;
        let mut var_t6_db4: f64 = *var_t6_db4_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn1: f64 = *var_t6_dn1_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_rdb0: f64 = *var_t6_rdb0_slot;
        let mut var_t6_rdb1: f64 = *var_t6_rdb1_slot;
        let mut var_t6_rdb2: f64 = *var_t6_rdb2_slot;
        let mut var_t6_rdb3: f64 = *var_t6_rdb3_slot;
        let mut var_t6_rdb4: f64 = *var_t6_rdb4_slot;
        let mut var_t6_rdn0: f64 = *var_t6_rdn0_slot;
        let mut var_t6_rdn1: f64 = *var_t6_rdn1_slot;
        let mut var_t6_rdn2: f64 = *var_t6_rdn2_slot;
        let mut var_t6_rdn3: f64 = *var_t6_rdn3_slot;
        let mut var_t6_rdn4: f64 = *var_t6_rdn4_slot;
        let mut var_t6_rdn5: f64 = *var_t6_rdn5_slot;
        let mut var_t6_rdn6: f64 = *var_t6_rdn6_slot;
        let mut var_t6_rdn7: f64 = *var_t6_rdn7_slot;
        let mut var_t6_rdn8: f64 = *var_t6_rdn8_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;

        let (assign11990_e11145, assign11990_e11145_d_n0, assign11990_e11145_d_n1, assign11990_e11145_d_n2, assign11990_e11145_d_n3, assign11990_e11145_d_n4, assign11990_e11145_d_n5, assign11990_e11145_d_n6, assign11990_e11145_d_n7, assign11990_e11145_d_n8, assign11990_e11145_d_b0, assign11990_e11145_d_b1, assign11990_e11145_d_b2, assign11990_e11145_d_b3, assign11990_e11145_d_b4,) = {
    if (var_guard121 != 0.0) {
        let assign11990_e11142: f64 = (var_bigbacc_i * var_voxacc);
        let assign11990_e11143: f64 = (var_aigbacc_i - assign11990_e11142);
        (assign11990_e11143, (var_aigbacc_i_dn0 - ((var_bigbacc_i_dn0 * var_voxacc) + (var_bigbacc_i * var_voxacc_dn0))), (var_aigbacc_i_dn1 - ((var_bigbacc_i_dn1 * var_voxacc) + (var_bigbacc_i * var_voxacc_dn1))), (var_aigbacc_i_dn2 - ((var_bigbacc_i_dn2 * var_voxacc) + (var_bigbacc_i * var_voxacc_dn2))), (var_aigbacc_i_dn3 - ((var_bigbacc_i_dn3 * var_voxacc) + (var_bigbacc_i * var_voxacc_dn3))), (var_aigbacc_i_dn4 - ((var_bigbacc_i_dn4 * var_voxacc) + (var_bigbacc_i * var_voxacc_dn4))), (var_aigbacc_i_dn5 - ((var_bigbacc_i_dn5 * var_voxacc) + (var_bigbacc_i * var_voxacc_dn5))), (var_aigbacc_i_dn6 - ((var_bigbacc_i_dn6 * var_voxacc) + (var_bigbacc_i * var_voxacc_dn6))), (var_aigbacc_i_dn7 - ((var_bigbacc_i_dn7 * var_voxacc) + (var_bigbacc_i * var_voxacc_dn7))), (var_aigbacc_i_dn8 - ((var_bigbacc_i_dn8 * var_voxacc) + (var_bigbacc_i * var_voxacc_dn8))), (var_aigbacc_i_db0 - ((var_bigbacc_i_db0 * var_voxacc) + (var_bigbacc_i * var_voxacc_db0))), (var_aigbacc_i_db1 - ((var_bigbacc_i_db1 * var_voxacc) + (var_bigbacc_i * var_voxacc_db1))), (var_aigbacc_i_db2 - ((var_bigbacc_i_db2 * var_voxacc) + (var_bigbacc_i * var_voxacc_db2))), (var_aigbacc_i_db3 - ((var_bigbacc_i_db3 * var_voxacc) + (var_bigbacc_i * var_voxacc_db3))), (var_aigbacc_i_db4 - ((var_bigbacc_i_db4 * var_voxacc) + (var_bigbacc_i * var_voxacc_db4))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_db0, var_t2_db1, var_t2_db2, var_t2_db3, var_t2_db4,)
    }
};
        var_t2 = assign11990_e11145;
        var_t2_dn0 = assign11990_e11145_d_n0;
        var_t2_dn1 = assign11990_e11145_d_n1;
        var_t2_dn2 = assign11990_e11145_d_n2;
        var_t2_dn3 = assign11990_e11145_d_n3;
        var_t2_dn4 = assign11990_e11145_d_n4;
        var_t2_dn5 = assign11990_e11145_d_n5;
        var_t2_dn6 = assign11990_e11145_d_n6;
        var_t2_dn7 = assign11990_e11145_d_n7;
        var_t2_dn8 = assign11990_e11145_d_n8;
        var_t2_db0 = assign11990_e11145_d_b0;
        var_t2_db1 = assign11990_e11145_d_b1;
        var_t2_db2 = assign11990_e11145_d_b2;
        var_t2_db3 = assign11990_e11145_d_b3;
        var_t2_db4 = assign11990_e11145_d_b4;
        var_t2_rv = 0.0;
        var_t2_rdn0 = 0.0;
        var_t2_rdn1 = 0.0;
        var_t2_rdn2 = 0.0;
        var_t2_rdn3 = 0.0;
        var_t2_rdn4 = 0.0;
        var_t2_rdn5 = 0.0;
        var_t2_rdn6 = 0.0;
        var_t2_rdn7 = 0.0;
        var_t2_rdn8 = 0.0;
        var_t2_rdb0 = 0.0;
        var_t2_rdb1 = 0.0;
        var_t2_rdb2 = 0.0;
        var_t2_rdb3 = 0.0;
        var_t2_rdb4 = 0.0;

        let (assign12000_e11153, assign12000_e11153_d_n0, assign12000_e11153_d_n1, assign12000_e11153_d_n2, assign12000_e11153_d_n3, assign12000_e11153_d_n4, assign12000_e11153_d_n5, assign12000_e11153_d_n6, assign12000_e11153_d_n7, assign12000_e11153_d_n8, assign12000_e11153_d_b0, assign12000_e11153_d_b1, assign12000_e11153_d_b2, assign12000_e11153_d_b3, assign12000_e11153_d_b4,) = {
    if (var_guard121 != 0.0) {
        let assign12000_e11150: f64 = (var_cigbacc_i * var_voxacc);
        let assign12000_e11151: f64 = (1.0 + assign12000_e11150);
        (assign12000_e11151, ((var_cigbacc_i_dn0 * var_voxacc) + (var_cigbacc_i * var_voxacc_dn0)), ((var_cigbacc_i_dn1 * var_voxacc) + (var_cigbacc_i * var_voxacc_dn1)), ((var_cigbacc_i_dn2 * var_voxacc) + (var_cigbacc_i * var_voxacc_dn2)), ((var_cigbacc_i_dn3 * var_voxacc) + (var_cigbacc_i * var_voxacc_dn3)), ((var_cigbacc_i_dn4 * var_voxacc) + (var_cigbacc_i * var_voxacc_dn4)), ((var_cigbacc_i_dn5 * var_voxacc) + (var_cigbacc_i * var_voxacc_dn5)), ((var_cigbacc_i_dn6 * var_voxacc) + (var_cigbacc_i * var_voxacc_dn6)), ((var_cigbacc_i_dn7 * var_voxacc) + (var_cigbacc_i * var_voxacc_dn7)), ((var_cigbacc_i_dn8 * var_voxacc) + (var_cigbacc_i * var_voxacc_dn8)), ((var_cigbacc_i_db0 * var_voxacc) + (var_cigbacc_i * var_voxacc_db0)), ((var_cigbacc_i_db1 * var_voxacc) + (var_cigbacc_i * var_voxacc_db1)), ((var_cigbacc_i_db2 * var_voxacc) + (var_cigbacc_i * var_voxacc_db2)), ((var_cigbacc_i_db3 * var_voxacc) + (var_cigbacc_i * var_voxacc_db3)), ((var_cigbacc_i_db4 * var_voxacc) + (var_cigbacc_i * var_voxacc_db4)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn1, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_db0, var_t3_db1, var_t3_db2, var_t3_db3, var_t3_db4,)
    }
};
        var_t3 = assign12000_e11153;
        var_t3_dn0 = assign12000_e11153_d_n0;
        var_t3_dn1 = assign12000_e11153_d_n1;
        var_t3_dn2 = assign12000_e11153_d_n2;
        var_t3_dn3 = assign12000_e11153_d_n3;
        var_t3_dn4 = assign12000_e11153_d_n4;
        var_t3_dn5 = assign12000_e11153_d_n5;
        var_t3_dn6 = assign12000_e11153_d_n6;
        var_t3_dn7 = assign12000_e11153_d_n7;
        var_t3_dn8 = assign12000_e11153_d_n8;
        var_t3_db0 = assign12000_e11153_d_b0;
        var_t3_db1 = assign12000_e11153_d_b1;
        var_t3_db2 = assign12000_e11153_d_b2;
        var_t3_db3 = assign12000_e11153_d_b3;
        var_t3_db4 = assign12000_e11153_d_b4;
        var_t3_rv = 0.0;
        var_t3_rdn0 = 0.0;
        var_t3_rdn1 = 0.0;
        var_t3_rdn2 = 0.0;
        var_t3_rdn3 = 0.0;
        var_t3_rdn4 = 0.0;
        var_t3_rdn5 = 0.0;
        var_t3_rdn6 = 0.0;
        var_t3_rdn7 = 0.0;
        var_t3_rdn8 = 0.0;
        var_t3_rdb0 = 0.0;
        var_t3_rdb1 = 0.0;
        var_t3_rdb2 = 0.0;
        var_t3_rdb3 = 0.0;
        var_t3_rdb4 = 0.0;

        let (assign12010_e11164, assign12010_e11164_d_n0, assign12010_e11164_d_n1, assign12010_e11164_d_n2, assign12010_e11164_d_n3, assign12010_e11164_d_n4, assign12010_e11164_d_n5, assign12010_e11164_d_n6, assign12010_e11164_d_n7, assign12010_e11164_d_n8, assign12010_e11164_d_b0, assign12010_e11164_d_b1, assign12010_e11164_d_b2, assign12010_e11164_d_b3, assign12010_e11164_d_b4,) = {
    if (var_guard121 != 0.0) {
        let assign12010_e11156: f64 = (-745669000000.0);
        let assign12010_e11158: f64 = (assign12010_e11156 * p.p99);
        let assign12010_e11160: f64 = (assign12010_e11158 * var_t2);
        let assign12010_e11162: f64 = (assign12010_e11160 * var_t3);
        (assign12010_e11162, (((assign12010_e11158 * var_t2_dn0) * var_t3) + (assign12010_e11160 * var_t3_dn0)), (((assign12010_e11158 * var_t2_dn1) * var_t3) + (assign12010_e11160 * var_t3_dn1)), (((assign12010_e11158 * var_t2_dn2) * var_t3) + (assign12010_e11160 * var_t3_dn2)), (((assign12010_e11158 * var_t2_dn3) * var_t3) + (assign12010_e11160 * var_t3_dn3)), (((assign12010_e11158 * var_t2_dn4) * var_t3) + (assign12010_e11160 * var_t3_dn4)), (((assign12010_e11158 * var_t2_dn5) * var_t3) + (assign12010_e11160 * var_t3_dn5)), (((assign12010_e11158 * var_t2_dn6) * var_t3) + (assign12010_e11160 * var_t3_dn6)), (((assign12010_e11158 * var_t2_dn7) * var_t3) + (assign12010_e11160 * var_t3_dn7)), (((assign12010_e11158 * var_t2_dn8) * var_t3) + (assign12010_e11160 * var_t3_dn8)), (((assign12010_e11158 * var_t2_db0) * var_t3) + (assign12010_e11160 * var_t3_db0)), (((assign12010_e11158 * var_t2_db1) * var_t3) + (assign12010_e11160 * var_t3_db1)), (((assign12010_e11158 * var_t2_db2) * var_t3) + (assign12010_e11160 * var_t3_db2)), (((assign12010_e11158 * var_t2_db3) * var_t3) + (assign12010_e11160 * var_t3_db3)), (((assign12010_e11158 * var_t2_db4) * var_t3) + (assign12010_e11160 * var_t3_db4)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn1, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_db0, var_t4_db1, var_t4_db2, var_t4_db3, var_t4_db4,)
    }
};
        var_t4 = assign12010_e11164;
        var_t4_dn0 = assign12010_e11164_d_n0;
        var_t4_dn1 = assign12010_e11164_d_n1;
        var_t4_dn2 = assign12010_e11164_d_n2;
        var_t4_dn3 = assign12010_e11164_d_n3;
        var_t4_dn4 = assign12010_e11164_d_n4;
        var_t4_dn5 = assign12010_e11164_d_n5;
        var_t4_dn6 = assign12010_e11164_d_n6;
        var_t4_dn7 = assign12010_e11164_d_n7;
        var_t4_dn8 = assign12010_e11164_d_n8;
        var_t4_db0 = assign12010_e11164_d_b0;
        var_t4_db1 = assign12010_e11164_d_b1;
        var_t4_db2 = assign12010_e11164_d_b2;
        var_t4_db3 = assign12010_e11164_d_b3;
        var_t4_db4 = assign12010_e11164_d_b4;
        var_t4_rv = 0.0;
        var_t4_rdn0 = 0.0;
        var_t4_rdn1 = 0.0;
        var_t4_rdn2 = 0.0;
        var_t4_rdn3 = 0.0;
        var_t4_rdn4 = 0.0;
        var_t4_rdn5 = 0.0;
        var_t4_rdn6 = 0.0;
        var_t4_rdn7 = 0.0;
        var_t4_rdn8 = 0.0;
        var_t4_rdb0 = 0.0;
        var_t4_rdb1 = 0.0;
        var_t4_rdb2 = 0.0;
        var_t4_rdb3 = 0.0;
        var_t4_rdb4 = 0.0;

        let (assign12020_e11169, assign12020_e11169_d_n0, assign12020_e11169_d_n1, assign12020_e11169_d_n2, assign12020_e11169_d_n3, assign12020_e11169_d_n4, assign12020_e11169_d_n5, assign12020_e11169_d_n6, assign12020_e11169_d_n7, assign12020_e11169_d_n8, assign12020_e11169_d_b0, assign12020_e11169_d_b1, assign12020_e11169_d_b2, assign12020_e11169_d_b3, assign12020_e11169_d_b4,) = {
    if (var_guard121 != 0.0) {
        let assign12020_e11167: f64 = { let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12020_e11167, ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn0), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn1), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn2), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn3), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn4), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn5), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn6), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn7), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn8), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_db0), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_db1), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_db2), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_db3), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_db4),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn1, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_db0, var_t5_db1, var_t5_db2, var_t5_db3, var_t5_db4,)
    }
};
        var_t5 = assign12020_e11169;
        var_t5_dn0 = assign12020_e11169_d_n0;
        var_t5_dn1 = assign12020_e11169_d_n1;
        var_t5_dn2 = assign12020_e11169_d_n2;
        var_t5_dn3 = assign12020_e11169_d_n3;
        var_t5_dn4 = assign12020_e11169_d_n4;
        var_t5_dn5 = assign12020_e11169_d_n5;
        var_t5_dn6 = assign12020_e11169_d_n6;
        var_t5_dn7 = assign12020_e11169_d_n7;
        var_t5_dn8 = assign12020_e11169_d_n8;
        var_t5_db0 = assign12020_e11169_d_b0;
        var_t5_db1 = assign12020_e11169_d_b1;
        var_t5_db2 = assign12020_e11169_d_b2;
        var_t5_db3 = assign12020_e11169_d_b3;
        var_t5_db4 = assign12020_e11169_d_b4;
        var_t5_rv = 0.0;
        var_t5_rdn0 = 0.0;
        var_t5_rdn1 = 0.0;
        var_t5_rdn2 = 0.0;
        var_t5_rdn3 = 0.0;
        var_t5_rdn4 = 0.0;
        var_t5_rdn5 = 0.0;
        var_t5_rdn6 = 0.0;
        var_t5_rdn7 = 0.0;
        var_t5_rdn8 = 0.0;
        var_t5_rdb0 = 0.0;
        var_t5_rdb1 = 0.0;
        var_t5_rdb2 = 0.0;
        var_t5_rdb3 = 0.0;
        var_t5_rdb4 = 0.0;

        let (assign12030_e11173, assign12030_e11173_d_n0, assign12030_e11173_d_n1, assign12030_e11173_d_n2, assign12030_e11173_d_n3, assign12030_e11173_d_n4, assign12030_e11173_d_n5, assign12030_e11173_d_n6, assign12030_e11173_d_n7, assign12030_e11173_d_n8, assign12030_e11173_d_b0, assign12030_e11173_d_b1, assign12030_e11173_d_b2, assign12030_e11173_d_b3, assign12030_e11173_d_b4,) = {
    if (var_guard121 != 0.0) {
        (4.97232e-7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn1, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_db0, var_t6_db1, var_t6_db2, var_t6_db3, var_t6_db4,)
    }
};
        var_t6 = assign12030_e11173;
        var_t6_dn0 = assign12030_e11173_d_n0;
        var_t6_dn1 = assign12030_e11173_d_n1;
        var_t6_dn2 = assign12030_e11173_d_n2;
        var_t6_dn3 = assign12030_e11173_d_n3;
        var_t6_dn4 = assign12030_e11173_d_n4;
        var_t6_dn5 = assign12030_e11173_d_n5;
        var_t6_dn6 = assign12030_e11173_d_n6;
        var_t6_dn7 = assign12030_e11173_d_n7;
        var_t6_dn8 = assign12030_e11173_d_n8;
        var_t6_db0 = assign12030_e11173_d_b0;
        var_t6_db1 = assign12030_e11173_d_b1;
        var_t6_db2 = assign12030_e11173_d_b2;
        var_t6_db3 = assign12030_e11173_d_b3;
        var_t6_db4 = assign12030_e11173_d_b4;
        var_t6_rv = 0.0;
        var_t6_rdn0 = 0.0;
        var_t6_rdn1 = 0.0;
        var_t6_rdn2 = 0.0;
        var_t6_rdn3 = 0.0;
        var_t6_rdn4 = 0.0;
        var_t6_rdn5 = 0.0;
        var_t6_rdn6 = 0.0;
        var_t6_rdn7 = 0.0;
        var_t6_rdn8 = 0.0;
        var_t6_rdb0 = 0.0;
        var_t6_rdb1 = 0.0;
        var_t6_rdb2 = 0.0;
        var_t6_rdb3 = 0.0;
        var_t6_rdb4 = 0.0;

        let assign12060_e11198: f64 = (0.6 * var_vds_noswap);
        let assign12060_e11200: f64 = (assign12060_e11198 / var_vtm);
        let assign12060_e11201: f64 = (assign12060_e11200).tanh();
        var_t0 = assign12060_e11201;
        var_t0_dn0 = (((((0.6 * var_vds_noswap_dn0) * var_vtm) - (assign12060_e11198 * var_vtm_dn0)) / (var_vtm * var_vtm)) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_dn1 = (((((0.6 * var_vds_noswap_dn1) * var_vtm) - (assign12060_e11198 * var_vtm_dn1)) / (var_vtm * var_vtm)) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_dn2 = (((((0.6 * var_vds_noswap_dn2) * var_vtm) - (assign12060_e11198 * var_vtm_dn2)) / (var_vtm * var_vtm)) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_dn3 = (((((0.6 * var_vds_noswap_dn3) * var_vtm) - (assign12060_e11198 * var_vtm_dn3)) / (var_vtm * var_vtm)) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_dn4 = (((((0.6 * var_vds_noswap_dn4) * var_vtm) - (assign12060_e11198 * var_vtm_dn4)) / (var_vtm * var_vtm)) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_dn5 = (((((0.6 * var_vds_noswap_dn5) * var_vtm) - (assign12060_e11198 * var_vtm_dn5)) / (var_vtm * var_vtm)) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_dn6 = (((((0.6 * var_vds_noswap_dn6) * var_vtm) - (assign12060_e11198 * var_vtm_dn6)) / (var_vtm * var_vtm)) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_dn7 = (((((0.6 * var_vds_noswap_dn7) * var_vtm) - (assign12060_e11198 * var_vtm_dn7)) / (var_vtm * var_vtm)) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_dn8 = (((((0.6 * var_vds_noswap_dn8) * var_vtm) - (assign12060_e11198 * var_vtm_dn8)) / (var_vtm * var_vtm)) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_db0 = (((((0.6 * var_vds_noswap_db0) * var_vtm) - (assign12060_e11198 * var_vtm_db0)) / (var_vtm * var_vtm)) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_db1 = (((((0.6 * var_vds_noswap_db1) * var_vtm) - (assign12060_e11198 * var_vtm_db1)) / (var_vtm * var_vtm)) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_db2 = (((((0.6 * var_vds_noswap_db2) * var_vtm) - (assign12060_e11198 * var_vtm_db2)) / (var_vtm * var_vtm)) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_db3 = (((((0.6 * var_vds_noswap_db3) * var_vtm) - (assign12060_e11198 * var_vtm_db3)) / (var_vtm * var_vtm)) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_db4 = (((((0.6 * var_vds_noswap_db4) * var_vtm) - (assign12060_e11198 * var_vtm_db4)) / (var_vtm * var_vtm)) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_rv = 0.0;
        var_t0_rdn0 = 0.0;
        var_t0_rdn1 = 0.0;
        var_t0_rdn2 = 0.0;
        var_t0_rdn3 = 0.0;
        var_t0_rdn4 = 0.0;
        var_t0_rdn5 = 0.0;
        var_t0_rdn6 = 0.0;
        var_t0_rdn7 = 0.0;
        var_t0_rdn8 = 0.0;
        var_t0_rdb0 = 0.0;
        var_t0_rdb1 = 0.0;
        var_t0_rdb2 = 0.0;
        var_t0_rdb3 = 0.0;
        var_t0_rdb4 = 0.0;

        let assign12110_e11222: f64 = if p.p16 != 0.0 { 1.0 } else { 0.0 };
        var_guard123 = assign12110_e11222;
        var_guard123_dn0 = 0.0;
        var_guard123_dn1 = 0.0;
        var_guard123_dn2 = 0.0;
        var_guard123_dn3 = 0.0;
        var_guard123_dn4 = 0.0;
        var_guard123_dn5 = 0.0;
        var_guard123_dn6 = 0.0;
        var_guard123_dn7 = 0.0;
        var_guard123_dn8 = 0.0;
        var_guard123_db0 = 0.0;
        var_guard123_db1 = 0.0;
        var_guard123_db2 = 0.0;
        var_guard123_db3 = 0.0;
        var_guard123_db4 = 0.0;
        var_guard123_rv = 0.0;
        var_guard123_rdn0 = 0.0;
        var_guard123_rdn1 = 0.0;
        var_guard123_rdn2 = 0.0;
        var_guard123_rdn3 = 0.0;
        var_guard123_rdn4 = 0.0;
        var_guard123_rdn5 = 0.0;
        var_guard123_rdn6 = 0.0;
        var_guard123_rdn7 = 0.0;
        var_guard123_rdn8 = 0.0;
        var_guard123_rdb0 = 0.0;
        var_guard123_rdb1 = 0.0;
        var_guard123_rdb2 = 0.0;
        var_guard123_rdb3 = 0.0;
        var_guard123_rdb4 = 0.0;

        let (assign12120_e11234, assign12120_e11234_d_n0, assign12120_e11234_d_n1, assign12120_e11234_d_n2, assign12120_e11234_d_n3, assign12120_e11234_d_n4, assign12120_e11234_d_n5, assign12120_e11234_d_n6, assign12120_e11234_d_n7, assign12120_e11234_d_n8, assign12120_e11234_d_b0, assign12120_e11234_d_b1, assign12120_e11234_d_b2, assign12120_e11234_d_b3, assign12120_e11234_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12120_e11229: f64 = (var_digc_i * var_phifs);
        let assign12120_e11230: f64 = (var_vgfb1 - assign12120_e11229);
        let assign12120_e11231: f64 = (var_bigc_i * assign12120_e11230);
        let assign12120_e11232: f64 = (var_aigc_i - assign12120_e11231);
        (assign12120_e11232, (var_aigc_i_dn0 - ((var_bigc_i_dn0 * assign12120_e11230) + (var_bigc_i * (var_vgfb1_dn0 - ((var_digc_i_dn0 * var_phifs) + (var_digc_i * var_phifs_dn0)))))), (var_aigc_i_dn1 - ((var_bigc_i_dn1 * assign12120_e11230) + (var_bigc_i * (var_vgfb1_dn1 - ((var_digc_i_dn1 * var_phifs) + (var_digc_i * var_phifs_dn1)))))), (var_aigc_i_dn2 - ((var_bigc_i_dn2 * assign12120_e11230) + (var_bigc_i * (var_vgfb1_dn2 - ((var_digc_i_dn2 * var_phifs) + (var_digc_i * var_phifs_dn2)))))), (var_aigc_i_dn3 - ((var_bigc_i_dn3 * assign12120_e11230) + (var_bigc_i * (var_vgfb1_dn3 - ((var_digc_i_dn3 * var_phifs) + (var_digc_i * var_phifs_dn3)))))), (var_aigc_i_dn4 - ((var_bigc_i_dn4 * assign12120_e11230) + (var_bigc_i * (var_vgfb1_dn4 - ((var_digc_i_dn4 * var_phifs) + (var_digc_i * var_phifs_dn4)))))), (var_aigc_i_dn5 - ((var_bigc_i_dn5 * assign12120_e11230) + (var_bigc_i * (var_vgfb1_dn5 - ((var_digc_i_dn5 * var_phifs) + (var_digc_i * var_phifs_dn5)))))), (var_aigc_i_dn6 - ((var_bigc_i_dn6 * assign12120_e11230) + (var_bigc_i * (var_vgfb1_dn6 - ((var_digc_i_dn6 * var_phifs) + (var_digc_i * var_phifs_dn6)))))), (var_aigc_i_dn7 - ((var_bigc_i_dn7 * assign12120_e11230) + (var_bigc_i * (var_vgfb1_dn7 - ((var_digc_i_dn7 * var_phifs) + (var_digc_i * var_phifs_dn7)))))), (var_aigc_i_dn8 - ((var_bigc_i_dn8 * assign12120_e11230) + (var_bigc_i * (var_vgfb1_dn8 - ((var_digc_i_dn8 * var_phifs) + (var_digc_i * var_phifs_dn8)))))), (var_aigc_i_db0 - ((var_bigc_i_db0 * assign12120_e11230) + (var_bigc_i * (var_vgfb1_db0 - ((var_digc_i_db0 * var_phifs) + (var_digc_i * var_phifs_db0)))))), (var_aigc_i_db1 - ((var_bigc_i_db1 * assign12120_e11230) + (var_bigc_i * (var_vgfb1_db1 - ((var_digc_i_db1 * var_phifs) + (var_digc_i * var_phifs_db1)))))), (var_aigc_i_db2 - ((var_bigc_i_db2 * assign12120_e11230) + (var_bigc_i * (var_vgfb1_db2 - ((var_digc_i_db2 * var_phifs) + (var_digc_i * var_phifs_db2)))))), (var_aigc_i_db3 - ((var_bigc_i_db3 * assign12120_e11230) + (var_bigc_i * (var_vgfb1_db3 - ((var_digc_i_db3 * var_phifs) + (var_digc_i * var_phifs_db3)))))), (var_aigc_i_db4 - ((var_bigc_i_db4 * assign12120_e11230) + (var_bigc_i * (var_vgfb1_db4 - ((var_digc_i_db4 * var_phifs) + (var_digc_i * var_phifs_db4)))))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4,)
    }
};
        var_t1 = assign12120_e11234;
        var_t1_dn0 = assign12120_e11234_d_n0;
        var_t1_dn1 = assign12120_e11234_d_n1;
        var_t1_dn2 = assign12120_e11234_d_n2;
        var_t1_dn3 = assign12120_e11234_d_n3;
        var_t1_dn4 = assign12120_e11234_d_n4;
        var_t1_dn5 = assign12120_e11234_d_n5;
        var_t1_dn6 = assign12120_e11234_d_n6;
        var_t1_dn7 = assign12120_e11234_d_n7;
        var_t1_dn8 = assign12120_e11234_d_n8;
        var_t1_db0 = assign12120_e11234_d_b0;
        var_t1_db1 = assign12120_e11234_d_b1;
        var_t1_db2 = assign12120_e11234_d_b2;
        var_t1_db3 = assign12120_e11234_d_b3;
        var_t1_db4 = assign12120_e11234_d_b4;
        var_t1_rv = 0.0;
        var_t1_rdn0 = 0.0;
        var_t1_rdn1 = 0.0;
        var_t1_rdn2 = 0.0;
        var_t1_rdn3 = 0.0;
        var_t1_rdn4 = 0.0;
        var_t1_rdn5 = 0.0;
        var_t1_rdn6 = 0.0;
        var_t1_rdn7 = 0.0;
        var_t1_rdn8 = 0.0;
        var_t1_rdb0 = 0.0;
        var_t1_rdb1 = 0.0;
        var_t1_rdb2 = 0.0;
        var_t1_rdb3 = 0.0;
        var_t1_rdb4 = 0.0;

        let (assign12130_e11246, assign12130_e11246_d_n0, assign12130_e11246_d_n1, assign12130_e11246_d_n2, assign12130_e11246_d_n3, assign12130_e11246_d_n4, assign12130_e11246_d_n5, assign12130_e11246_d_n6, assign12130_e11246_d_n7, assign12130_e11246_d_n8, assign12130_e11246_d_b0, assign12130_e11246_d_b1, assign12130_e11246_d_b2, assign12130_e11246_d_b3, assign12130_e11246_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12130_e11241: f64 = (var_digc_i * var_phifs);
        let assign12130_e11242: f64 = (var_vgfb1 - assign12130_e11241);
        let assign12130_e11243: f64 = (var_cigc_i * assign12130_e11242);
        let assign12130_e11244: f64 = (1.0 + assign12130_e11243);
        (assign12130_e11244, ((var_cigc_i_dn0 * assign12130_e11242) + (var_cigc_i * (var_vgfb1_dn0 - ((var_digc_i_dn0 * var_phifs) + (var_digc_i * var_phifs_dn0))))), ((var_cigc_i_dn1 * assign12130_e11242) + (var_cigc_i * (var_vgfb1_dn1 - ((var_digc_i_dn1 * var_phifs) + (var_digc_i * var_phifs_dn1))))), ((var_cigc_i_dn2 * assign12130_e11242) + (var_cigc_i * (var_vgfb1_dn2 - ((var_digc_i_dn2 * var_phifs) + (var_digc_i * var_phifs_dn2))))), ((var_cigc_i_dn3 * assign12130_e11242) + (var_cigc_i * (var_vgfb1_dn3 - ((var_digc_i_dn3 * var_phifs) + (var_digc_i * var_phifs_dn3))))), ((var_cigc_i_dn4 * assign12130_e11242) + (var_cigc_i * (var_vgfb1_dn4 - ((var_digc_i_dn4 * var_phifs) + (var_digc_i * var_phifs_dn4))))), ((var_cigc_i_dn5 * assign12130_e11242) + (var_cigc_i * (var_vgfb1_dn5 - ((var_digc_i_dn5 * var_phifs) + (var_digc_i * var_phifs_dn5))))), ((var_cigc_i_dn6 * assign12130_e11242) + (var_cigc_i * (var_vgfb1_dn6 - ((var_digc_i_dn6 * var_phifs) + (var_digc_i * var_phifs_dn6))))), ((var_cigc_i_dn7 * assign12130_e11242) + (var_cigc_i * (var_vgfb1_dn7 - ((var_digc_i_dn7 * var_phifs) + (var_digc_i * var_phifs_dn7))))), ((var_cigc_i_dn8 * assign12130_e11242) + (var_cigc_i * (var_vgfb1_dn8 - ((var_digc_i_dn8 * var_phifs) + (var_digc_i * var_phifs_dn8))))), ((var_cigc_i_db0 * assign12130_e11242) + (var_cigc_i * (var_vgfb1_db0 - ((var_digc_i_db0 * var_phifs) + (var_digc_i * var_phifs_db0))))), ((var_cigc_i_db1 * assign12130_e11242) + (var_cigc_i * (var_vgfb1_db1 - ((var_digc_i_db1 * var_phifs) + (var_digc_i * var_phifs_db1))))), ((var_cigc_i_db2 * assign12130_e11242) + (var_cigc_i * (var_vgfb1_db2 - ((var_digc_i_db2 * var_phifs) + (var_digc_i * var_phifs_db2))))), ((var_cigc_i_db3 * assign12130_e11242) + (var_cigc_i * (var_vgfb1_db3 - ((var_digc_i_db3 * var_phifs) + (var_digc_i * var_phifs_db3))))), ((var_cigc_i_db4 * assign12130_e11242) + (var_cigc_i * (var_vgfb1_db4 - ((var_digc_i_db4 * var_phifs) + (var_digc_i * var_phifs_db4))))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_db0, var_t2_db1, var_t2_db2, var_t2_db3, var_t2_db4,)
    }
};
        var_t2 = assign12130_e11246;
        var_t2_dn0 = assign12130_e11246_d_n0;
        var_t2_dn1 = assign12130_e11246_d_n1;
        var_t2_dn2 = assign12130_e11246_d_n2;
        var_t2_dn3 = assign12130_e11246_d_n3;
        var_t2_dn4 = assign12130_e11246_d_n4;
        var_t2_dn5 = assign12130_e11246_d_n5;
        var_t2_dn6 = assign12130_e11246_d_n6;
        var_t2_dn7 = assign12130_e11246_d_n7;
        var_t2_dn8 = assign12130_e11246_d_n8;
        var_t2_db0 = assign12130_e11246_d_b0;
        var_t2_db1 = assign12130_e11246_d_b1;
        var_t2_db2 = assign12130_e11246_d_b2;
        var_t2_db3 = assign12130_e11246_d_b3;
        var_t2_db4 = assign12130_e11246_d_b4;
        var_t2_rv = 0.0;
        var_t2_rdn0 = 0.0;
        var_t2_rdn1 = 0.0;
        var_t2_rdn2 = 0.0;
        var_t2_rdn3 = 0.0;
        var_t2_rdn4 = 0.0;
        var_t2_rdn5 = 0.0;
        var_t2_rdn6 = 0.0;
        var_t2_rdn7 = 0.0;
        var_t2_rdn8 = 0.0;
        var_t2_rdb0 = 0.0;
        var_t2_rdb1 = 0.0;
        var_t2_rdb2 = 0.0;
        var_t2_rdb3 = 0.0;
        var_t2_rdb4 = 0.0;

        let (assign12140_e11257, assign12140_e11257_d_n0, assign12140_e11257_d_n1, assign12140_e11257_d_n2, assign12140_e11257_d_n3, assign12140_e11257_d_n4, assign12140_e11257_d_n5, assign12140_e11257_d_n6, assign12140_e11257_d_n7, assign12140_e11257_d_n8, assign12140_e11257_d_b0, assign12140_e11257_d_b1, assign12140_e11257_d_b2, assign12140_e11257_d_b3, assign12140_e11257_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12140_e11249: f64 = (-var_bechvb);
        let assign12140_e11251: f64 = (assign12140_e11249 * p.p99);
        let assign12140_e11253: f64 = (assign12140_e11251 * var_t1);
        let assign12140_e11255: f64 = (assign12140_e11253 * var_t2);
        (assign12140_e11255, ((((((-var_bechvb_dn0) * p.p99) * var_t1) + (assign12140_e11251 * var_t1_dn0)) * var_t2) + (assign12140_e11253 * var_t2_dn0)), ((((((-var_bechvb_dn1) * p.p99) * var_t1) + (assign12140_e11251 * var_t1_dn1)) * var_t2) + (assign12140_e11253 * var_t2_dn1)), ((((((-var_bechvb_dn2) * p.p99) * var_t1) + (assign12140_e11251 * var_t1_dn2)) * var_t2) + (assign12140_e11253 * var_t2_dn2)), ((((((-var_bechvb_dn3) * p.p99) * var_t1) + (assign12140_e11251 * var_t1_dn3)) * var_t2) + (assign12140_e11253 * var_t2_dn3)), ((((((-var_bechvb_dn4) * p.p99) * var_t1) + (assign12140_e11251 * var_t1_dn4)) * var_t2) + (assign12140_e11253 * var_t2_dn4)), ((((((-var_bechvb_dn5) * p.p99) * var_t1) + (assign12140_e11251 * var_t1_dn5)) * var_t2) + (assign12140_e11253 * var_t2_dn5)), ((((((-var_bechvb_dn6) * p.p99) * var_t1) + (assign12140_e11251 * var_t1_dn6)) * var_t2) + (assign12140_e11253 * var_t2_dn6)), ((((((-var_bechvb_dn7) * p.p99) * var_t1) + (assign12140_e11251 * var_t1_dn7)) * var_t2) + (assign12140_e11253 * var_t2_dn7)), ((((((-var_bechvb_dn8) * p.p99) * var_t1) + (assign12140_e11251 * var_t1_dn8)) * var_t2) + (assign12140_e11253 * var_t2_dn8)), ((((((-var_bechvb_db0) * p.p99) * var_t1) + (assign12140_e11251 * var_t1_db0)) * var_t2) + (assign12140_e11253 * var_t2_db0)), ((((((-var_bechvb_db1) * p.p99) * var_t1) + (assign12140_e11251 * var_t1_db1)) * var_t2) + (assign12140_e11253 * var_t2_db1)), ((((((-var_bechvb_db2) * p.p99) * var_t1) + (assign12140_e11251 * var_t1_db2)) * var_t2) + (assign12140_e11253 * var_t2_db2)), ((((((-var_bechvb_db3) * p.p99) * var_t1) + (assign12140_e11251 * var_t1_db3)) * var_t2) + (assign12140_e11253 * var_t2_db3)), ((((((-var_bechvb_db4) * p.p99) * var_t1) + (assign12140_e11251 * var_t1_db4)) * var_t2) + (assign12140_e11253 * var_t2_db4)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn1, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_db0, var_t3_db1, var_t3_db2, var_t3_db3, var_t3_db4,)
    }
};
        var_t3 = assign12140_e11257;
        var_t3_dn0 = assign12140_e11257_d_n0;
        var_t3_dn1 = assign12140_e11257_d_n1;
        var_t3_dn2 = assign12140_e11257_d_n2;
        var_t3_dn3 = assign12140_e11257_d_n3;
        var_t3_dn4 = assign12140_e11257_d_n4;
        var_t3_dn5 = assign12140_e11257_d_n5;
        var_t3_dn6 = assign12140_e11257_d_n6;
        var_t3_dn7 = assign12140_e11257_d_n7;
        var_t3_dn8 = assign12140_e11257_d_n8;
        var_t3_db0 = assign12140_e11257_d_b0;
        var_t3_db1 = assign12140_e11257_d_b1;
        var_t3_db2 = assign12140_e11257_d_b2;
        var_t3_db3 = assign12140_e11257_d_b3;
        var_t3_db4 = assign12140_e11257_d_b4;
        var_t3_rv = 0.0;
        var_t3_rdn0 = 0.0;
        var_t3_rdn1 = 0.0;
        var_t3_rdn2 = 0.0;
        var_t3_rdn3 = 0.0;
        var_t3_rdn4 = 0.0;
        var_t3_rdn5 = 0.0;
        var_t3_rdn6 = 0.0;
        var_t3_rdn7 = 0.0;
        var_t3_rdn8 = 0.0;
        var_t3_rdb0 = 0.0;
        var_t3_rdb1 = 0.0;
        var_t3_rdb2 = 0.0;
        var_t3_rdb3 = 0.0;
        var_t3_rdb4 = 0.0;

        let (assign12150_e11264, assign12150_e11264_d_n0, assign12150_e11264_d_n1, assign12150_e11264_d_n2, assign12150_e11264_d_n3, assign12150_e11264_d_n4, assign12150_e11264_d_n5, assign12150_e11264_d_n6, assign12150_e11264_d_n7, assign12150_e11264_d_n8, assign12150_e11264_d_b0, assign12150_e11264_d_b1, assign12150_e11264_d_b2, assign12150_e11264_d_b3, assign12150_e11264_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12150_e11261: f64 = { let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign12150_e11262: f64 = (var_qia * assign12150_e11261);
        (assign12150_e11262, ((var_qia_dn0 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn0))), ((var_qia_dn1 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn1))), ((var_qia_dn2 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn2))), ((var_qia_dn3 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn3))), ((var_qia_dn4 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn4))), ((var_qia_dn5 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn5))), ((var_qia_dn6 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn6))), ((var_qia_dn7 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn7))), ((var_qia_dn8 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn8))), ((var_qia_db0 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db0))), ((var_qia_db1 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db1))), ((var_qia_db2 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db2))), ((var_qia_db3 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db3))), ((var_qia_db4 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db4))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn1, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_db0, var_t4_db1, var_t4_db2, var_t4_db3, var_t4_db4,)
    }
};
        var_t4 = assign12150_e11264;
        var_t4_dn0 = assign12150_e11264_d_n0;
        var_t4_dn1 = assign12150_e11264_d_n1;
        var_t4_dn2 = assign12150_e11264_d_n2;
        var_t4_dn3 = assign12150_e11264_d_n3;
        var_t4_dn4 = assign12150_e11264_d_n4;
        var_t4_dn5 = assign12150_e11264_d_n5;
        var_t4_dn6 = assign12150_e11264_d_n6;
        var_t4_dn7 = assign12150_e11264_d_n7;
        var_t4_dn8 = assign12150_e11264_d_n8;
        var_t4_db0 = assign12150_e11264_d_b0;
        var_t4_db1 = assign12150_e11264_d_b1;
        var_t4_db2 = assign12150_e11264_d_b2;
        var_t4_db3 = assign12150_e11264_d_b3;
        var_t4_db4 = assign12150_e11264_d_b4;
        var_t4_rv = 0.0;
        var_t4_rdn0 = 0.0;
        var_t4_rdn1 = 0.0;
        var_t4_rdn2 = 0.0;
        var_t4_rdn3 = 0.0;
        var_t4_rdn4 = 0.0;
        var_t4_rdn5 = 0.0;
        var_t4_rdn6 = 0.0;
        var_t4_rdn7 = 0.0;
        var_t4_rdn8 = 0.0;
        var_t4_rdb0 = 0.0;
        var_t4_rdb1 = 0.0;
        var_t4_rdb2 = 0.0;
        var_t4_rdb3 = 0.0;
        var_t4_rdb4 = 0.0;

        let (assign12160_e11278, assign12160_e11278_d_n0, assign12160_e11278_d_n1, assign12160_e11278_d_n2, assign12160_e11278_d_n3, assign12160_e11278_d_n4, assign12160_e11278_d_n5, assign12160_e11278_d_n6, assign12160_e11278_d_n7, assign12160_e11278_d_n8, assign12160_e11278_d_b0, assign12160_e11278_d_b1, assign12160_e11278_d_b2, assign12160_e11278_d_b3, assign12160_e11278_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12160_e11269: f64 = (0.5 * var_vdsx);
        let assign12160_e11270: f64 = (var_vgbg + assign12160_e11269);
        let assign12160_e11274: f64 = (var_vbgs_noswap + var_vbgd_noswap);
        let assign12160_e11275: f64 = (0.5 * assign12160_e11274);
        let assign12160_e11276: f64 = (assign12160_e11270 + assign12160_e11275);
        (assign12160_e11276, ((var_vgbg_dn0 + (0.5 * var_vdsx_dn0)) + (0.5 * (var_vbgs_noswap_dn0 + var_vbgd_noswap_dn0))), ((var_vgbg_dn1 + (0.5 * var_vdsx_dn1)) + (0.5 * (var_vbgs_noswap_dn1 + var_vbgd_noswap_dn1))), ((var_vgbg_dn2 + (0.5 * var_vdsx_dn2)) + (0.5 * (var_vbgs_noswap_dn2 + var_vbgd_noswap_dn2))), ((var_vgbg_dn3 + (0.5 * var_vdsx_dn3)) + (0.5 * (var_vbgs_noswap_dn3 + var_vbgd_noswap_dn3))), ((var_vgbg_dn4 + (0.5 * var_vdsx_dn4)) + (0.5 * (var_vbgs_noswap_dn4 + var_vbgd_noswap_dn4))), ((var_vgbg_dn5 + (0.5 * var_vdsx_dn5)) + (0.5 * (var_vbgs_noswap_dn5 + var_vbgd_noswap_dn5))), ((var_vgbg_dn6 + (0.5 * var_vdsx_dn6)) + (0.5 * (var_vbgs_noswap_dn6 + var_vbgd_noswap_dn6))), ((var_vgbg_dn7 + (0.5 * var_vdsx_dn7)) + (0.5 * (var_vbgs_noswap_dn7 + var_vbgd_noswap_dn7))), ((var_vgbg_dn8 + (0.5 * var_vdsx_dn8)) + (0.5 * (var_vbgs_noswap_dn8 + var_vbgd_noswap_dn8))), ((var_vgbg_db0 + (0.5 * var_vdsx_db0)) + (0.5 * (var_vbgs_noswap_db0 + var_vbgd_noswap_db0))), ((var_vgbg_db1 + (0.5 * var_vdsx_db1)) + (0.5 * (var_vbgs_noswap_db1 + var_vbgd_noswap_db1))), ((var_vgbg_db2 + (0.5 * var_vdsx_db2)) + (0.5 * (var_vbgs_noswap_db2 + var_vbgd_noswap_db2))), ((var_vgbg_db3 + (0.5 * var_vdsx_db3)) + (0.5 * (var_vbgs_noswap_db3 + var_vbgd_noswap_db3))), ((var_vgbg_db4 + (0.5 * var_vdsx_db4)) + (0.5 * (var_vbgs_noswap_db4 + var_vbgd_noswap_db4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn1, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_db0, var_t5_db1, var_t5_db2, var_t5_db3, var_t5_db4,)
    }
};
        var_t5 = assign12160_e11278;
        var_t5_dn0 = assign12160_e11278_d_n0;
        var_t5_dn1 = assign12160_e11278_d_n1;
        var_t5_dn2 = assign12160_e11278_d_n2;
        var_t5_dn3 = assign12160_e11278_d_n3;
        var_t5_dn4 = assign12160_e11278_d_n4;
        var_t5_dn5 = assign12160_e11278_d_n5;
        var_t5_dn6 = assign12160_e11278_d_n6;
        var_t5_dn7 = assign12160_e11278_d_n7;
        var_t5_dn8 = assign12160_e11278_d_n8;
        var_t5_db0 = assign12160_e11278_d_b0;
        var_t5_db1 = assign12160_e11278_d_b1;
        var_t5_db2 = assign12160_e11278_d_b2;
        var_t5_db3 = assign12160_e11278_d_b3;
        var_t5_db4 = assign12160_e11278_d_b4;
        var_t5_rv = 0.0;
        var_t5_rdn0 = 0.0;
        var_t5_rdn1 = 0.0;
        var_t5_rdn2 = 0.0;
        var_t5_rdn3 = 0.0;
        var_t5_rdn4 = 0.0;
        var_t5_rdn5 = 0.0;
        var_t5_rdn6 = 0.0;
        var_t5_rdn7 = 0.0;
        var_t5_rdn8 = 0.0;
        var_t5_rdb0 = 0.0;
        var_t5_rdb1 = 0.0;
        var_t5_rdb2 = 0.0;
        var_t5_rdb3 = 0.0;
        var_t5_rdb4 = 0.0;


        *var_guard123_slot = var_guard123;
        *var_guard123_db0_slot = var_guard123_db0;
        *var_guard123_db1_slot = var_guard123_db1;
        *var_guard123_db2_slot = var_guard123_db2;
        *var_guard123_db3_slot = var_guard123_db3;
        *var_guard123_db4_slot = var_guard123_db4;
        *var_guard123_dn0_slot = var_guard123_dn0;
        *var_guard123_dn1_slot = var_guard123_dn1;
        *var_guard123_dn2_slot = var_guard123_dn2;
        *var_guard123_dn3_slot = var_guard123_dn3;
        *var_guard123_dn4_slot = var_guard123_dn4;
        *var_guard123_dn5_slot = var_guard123_dn5;
        *var_guard123_dn6_slot = var_guard123_dn6;
        *var_guard123_dn7_slot = var_guard123_dn7;
        *var_guard123_dn8_slot = var_guard123_dn8;
        *var_guard123_rdb0_slot = var_guard123_rdb0;
        *var_guard123_rdb1_slot = var_guard123_rdb1;
        *var_guard123_rdb2_slot = var_guard123_rdb2;
        *var_guard123_rdb3_slot = var_guard123_rdb3;
        *var_guard123_rdb4_slot = var_guard123_rdb4;
        *var_guard123_rdn0_slot = var_guard123_rdn0;
        *var_guard123_rdn1_slot = var_guard123_rdn1;
        *var_guard123_rdn2_slot = var_guard123_rdn2;
        *var_guard123_rdn3_slot = var_guard123_rdn3;
        *var_guard123_rdn4_slot = var_guard123_rdn4;
        *var_guard123_rdn5_slot = var_guard123_rdn5;
        *var_guard123_rdn6_slot = var_guard123_rdn6;
        *var_guard123_rdn7_slot = var_guard123_rdn7;
        *var_guard123_rdn8_slot = var_guard123_rdn8;
        *var_guard123_rv_slot = var_guard123_rv;
        *var_t0_slot = var_t0;
        *var_t0_db0_slot = var_t0_db0;
        *var_t0_db1_slot = var_t0_db1;
        *var_t0_db2_slot = var_t0_db2;
        *var_t0_db3_slot = var_t0_db3;
        *var_t0_db4_slot = var_t0_db4;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rdb0_slot = var_t0_rdb0;
        *var_t0_rdb1_slot = var_t0_rdb1;
        *var_t0_rdb2_slot = var_t0_rdb2;
        *var_t0_rdb3_slot = var_t0_rdb3;
        *var_t0_rdb4_slot = var_t0_rdb4;
        *var_t0_rdn0_slot = var_t0_rdn0;
        *var_t0_rdn1_slot = var_t0_rdn1;
        *var_t0_rdn2_slot = var_t0_rdn2;
        *var_t0_rdn3_slot = var_t0_rdn3;
        *var_t0_rdn4_slot = var_t0_rdn4;
        *var_t0_rdn5_slot = var_t0_rdn5;
        *var_t0_rdn6_slot = var_t0_rdn6;
        *var_t0_rdn7_slot = var_t0_rdn7;
        *var_t0_rdn8_slot = var_t0_rdn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_db0_slot = var_t1_db0;
        *var_t1_db1_slot = var_t1_db1;
        *var_t1_db2_slot = var_t1_db2;
        *var_t1_db3_slot = var_t1_db3;
        *var_t1_db4_slot = var_t1_db4;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rdb0_slot = var_t1_rdb0;
        *var_t1_rdb1_slot = var_t1_rdb1;
        *var_t1_rdb2_slot = var_t1_rdb2;
        *var_t1_rdb3_slot = var_t1_rdb3;
        *var_t1_rdb4_slot = var_t1_rdb4;
        *var_t1_rdn0_slot = var_t1_rdn0;
        *var_t1_rdn1_slot = var_t1_rdn1;
        *var_t1_rdn2_slot = var_t1_rdn2;
        *var_t1_rdn3_slot = var_t1_rdn3;
        *var_t1_rdn4_slot = var_t1_rdn4;
        *var_t1_rdn5_slot = var_t1_rdn5;
        *var_t1_rdn6_slot = var_t1_rdn6;
        *var_t1_rdn7_slot = var_t1_rdn7;
        *var_t1_rdn8_slot = var_t1_rdn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_db0_slot = var_t2_db0;
        *var_t2_db1_slot = var_t2_db1;
        *var_t2_db2_slot = var_t2_db2;
        *var_t2_db3_slot = var_t2_db3;
        *var_t2_db4_slot = var_t2_db4;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rdb0_slot = var_t2_rdb0;
        *var_t2_rdb1_slot = var_t2_rdb1;
        *var_t2_rdb2_slot = var_t2_rdb2;
        *var_t2_rdb3_slot = var_t2_rdb3;
        *var_t2_rdb4_slot = var_t2_rdb4;
        *var_t2_rdn0_slot = var_t2_rdn0;
        *var_t2_rdn1_slot = var_t2_rdn1;
        *var_t2_rdn2_slot = var_t2_rdn2;
        *var_t2_rdn3_slot = var_t2_rdn3;
        *var_t2_rdn4_slot = var_t2_rdn4;
        *var_t2_rdn5_slot = var_t2_rdn5;
        *var_t2_rdn6_slot = var_t2_rdn6;
        *var_t2_rdn7_slot = var_t2_rdn7;
        *var_t2_rdn8_slot = var_t2_rdn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_db0_slot = var_t3_db0;
        *var_t3_db1_slot = var_t3_db1;
        *var_t3_db2_slot = var_t3_db2;
        *var_t3_db3_slot = var_t3_db3;
        *var_t3_db4_slot = var_t3_db4;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn1_slot = var_t3_dn1;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rdb0_slot = var_t3_rdb0;
        *var_t3_rdb1_slot = var_t3_rdb1;
        *var_t3_rdb2_slot = var_t3_rdb2;
        *var_t3_rdb3_slot = var_t3_rdb3;
        *var_t3_rdb4_slot = var_t3_rdb4;
        *var_t3_rdn0_slot = var_t3_rdn0;
        *var_t3_rdn1_slot = var_t3_rdn1;
        *var_t3_rdn2_slot = var_t3_rdn2;
        *var_t3_rdn3_slot = var_t3_rdn3;
        *var_t3_rdn4_slot = var_t3_rdn4;
        *var_t3_rdn5_slot = var_t3_rdn5;
        *var_t3_rdn6_slot = var_t3_rdn6;
        *var_t3_rdn7_slot = var_t3_rdn7;
        *var_t3_rdn8_slot = var_t3_rdn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_db0_slot = var_t4_db0;
        *var_t4_db1_slot = var_t4_db1;
        *var_t4_db2_slot = var_t4_db2;
        *var_t4_db3_slot = var_t4_db3;
        *var_t4_db4_slot = var_t4_db4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn1_slot = var_t4_dn1;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_rdb0_slot = var_t4_rdb0;
        *var_t4_rdb1_slot = var_t4_rdb1;
        *var_t4_rdb2_slot = var_t4_rdb2;
        *var_t4_rdb3_slot = var_t4_rdb3;
        *var_t4_rdb4_slot = var_t4_rdb4;
        *var_t4_rdn0_slot = var_t4_rdn0;
        *var_t4_rdn1_slot = var_t4_rdn1;
        *var_t4_rdn2_slot = var_t4_rdn2;
        *var_t4_rdn3_slot = var_t4_rdn3;
        *var_t4_rdn4_slot = var_t4_rdn4;
        *var_t4_rdn5_slot = var_t4_rdn5;
        *var_t4_rdn6_slot = var_t4_rdn6;
        *var_t4_rdn7_slot = var_t4_rdn7;
        *var_t4_rdn8_slot = var_t4_rdn8;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_db0_slot = var_t5_db0;
        *var_t5_db1_slot = var_t5_db1;
        *var_t5_db2_slot = var_t5_db2;
        *var_t5_db3_slot = var_t5_db3;
        *var_t5_db4_slot = var_t5_db4;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn1_slot = var_t5_dn1;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_rdb0_slot = var_t5_rdb0;
        *var_t5_rdb1_slot = var_t5_rdb1;
        *var_t5_rdb2_slot = var_t5_rdb2;
        *var_t5_rdb3_slot = var_t5_rdb3;
        *var_t5_rdb4_slot = var_t5_rdb4;
        *var_t5_rdn0_slot = var_t5_rdn0;
        *var_t5_rdn1_slot = var_t5_rdn1;
        *var_t5_rdn2_slot = var_t5_rdn2;
        *var_t5_rdn3_slot = var_t5_rdn3;
        *var_t5_rdn4_slot = var_t5_rdn4;
        *var_t5_rdn5_slot = var_t5_rdn5;
        *var_t5_rdn6_slot = var_t5_rdn6;
        *var_t5_rdn7_slot = var_t5_rdn7;
        *var_t5_rdn8_slot = var_t5_rdn8;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_db0_slot = var_t6_db0;
        *var_t6_db1_slot = var_t6_db1;
        *var_t6_db2_slot = var_t6_db2;
        *var_t6_db3_slot = var_t6_db3;
        *var_t6_db4_slot = var_t6_db4;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn1_slot = var_t6_dn1;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_rdb0_slot = var_t6_rdb0;
        *var_t6_rdb1_slot = var_t6_rdb1;
        *var_t6_rdb2_slot = var_t6_rdb2;
        *var_t6_rdb3_slot = var_t6_rdb3;
        *var_t6_rdb4_slot = var_t6_rdb4;
        *var_t6_rdn0_slot = var_t6_rdn0;
        *var_t6_rdn1_slot = var_t6_rdn1;
        *var_t6_rdn2_slot = var_t6_rdn2;
        *var_t6_rdn3_slot = var_t6_rdn3;
        *var_t6_rdn4_slot = var_t6_rdn4;
        *var_t6_rdn5_slot = var_t6_rdn5;
        *var_t6_rdn6_slot = var_t6_rdn6;
        *var_t6_rdn7_slot = var_t6_rdn7;
        *var_t6_rdn8_slot = var_t6_rdn8;
        *var_t6_rv_slot = var_t6_rv;
    }

    pub(super) fn stamp_reactive_block_84(
        p: &Parameters,
        var_aigs_i: f64,
        var_aigs_i_db0: f64,
        var_aigs_i_db1: f64,
        var_aigs_i_db2: f64,
        var_aigs_i_db3: f64,
        var_aigs_i_db4: f64,
        var_aigs_i_dn0: f64,
        var_aigs_i_dn1: f64,
        var_aigs_i_dn2: f64,
        var_aigs_i_dn3: f64,
        var_aigs_i_dn4: f64,
        var_aigs_i_dn5: f64,
        var_aigs_i_dn6: f64,
        var_aigs_i_dn7: f64,
        var_aigs_i_dn8: f64,
        var_bechvb: f64,
        var_bechvb_db0: f64,
        var_bechvb_db1: f64,
        var_bechvb_db2: f64,
        var_bechvb_db3: f64,
        var_bechvb_db4: f64,
        var_bechvb_dn0: f64,
        var_bechvb_dn1: f64,
        var_bechvb_dn2: f64,
        var_bechvb_dn3: f64,
        var_bechvb_dn4: f64,
        var_bechvb_dn5: f64,
        var_bechvb_dn6: f64,
        var_bechvb_dn7: f64,
        var_bechvb_dn8: f64,
        var_bigs_i: f64,
        var_bigs_i_db0: f64,
        var_bigs_i_db1: f64,
        var_bigs_i_db2: f64,
        var_bigs_i_db3: f64,
        var_bigs_i_db4: f64,
        var_bigs_i_dn0: f64,
        var_bigs_i_dn1: f64,
        var_bigs_i_dn2: f64,
        var_bigs_i_dn3: f64,
        var_bigs_i_dn4: f64,
        var_bigs_i_dn5: f64,
        var_bigs_i_dn6: f64,
        var_bigs_i_dn7: f64,
        var_bigs_i_dn8: f64,
        var_cigs_i: f64,
        var_cigs_i_db0: f64,
        var_cigs_i_db1: f64,
        var_cigs_i_db2: f64,
        var_cigs_i_db3: f64,
        var_cigs_i_db4: f64,
        var_cigs_i_dn0: f64,
        var_cigs_i_dn1: f64,
        var_cigs_i_dn2: f64,
        var_cigs_i_dn3: f64,
        var_cigs_i_dn4: f64,
        var_cigs_i_dn5: f64,
        var_cigs_i_dn6: f64,
        var_cigs_i_dn7: f64,
        var_cigs_i_dn8: f64,
        var_digs_i: f64,
        var_digs_i_db0: f64,
        var_digs_i_db1: f64,
        var_digs_i_db2: f64,
        var_digs_i_db3: f64,
        var_digs_i_db4: f64,
        var_digs_i_dn0: f64,
        var_digs_i_dn1: f64,
        var_digs_i_dn2: f64,
        var_digs_i_dn3: f64,
        var_digs_i_dn4: f64,
        var_digs_i_dn5: f64,
        var_digs_i_dn6: f64,
        var_digs_i_dn7: f64,
        var_digs_i_dn8: f64,
        var_gamma0: f64,
        var_gamma0_db0: f64,
        var_gamma0_db1: f64,
        var_gamma0_db2: f64,
        var_gamma0_db3: f64,
        var_gamma0_db4: f64,
        var_gamma0_dn0: f64,
        var_gamma0_dn1: f64,
        var_gamma0_dn2: f64,
        var_gamma0_dn3: f64,
        var_gamma0_dn4: f64,
        var_gamma0_dn5: f64,
        var_gamma0_dn6: f64,
        var_gamma0_dn7: f64,
        var_gamma0_dn8: f64,
        var_guard123: f64,
        var_pigcd_i: f64,
        var_pigcd_i_db0: f64,
        var_pigcd_i_db1: f64,
        var_pigcd_i_db2: f64,
        var_pigcd_i_db3: f64,
        var_pigcd_i_db4: f64,
        var_pigcd_i_dn0: f64,
        var_pigcd_i_dn1: f64,
        var_pigcd_i_dn2: f64,
        var_pigcd_i_dn3: f64,
        var_pigcd_i_dn4: f64,
        var_pigcd_i_dn5: f64,
        var_pigcd_i_dn6: f64,
        var_pigcd_i_dn7: f64,
        var_pigcd_i_dn8: f64,
        var_poxedge_i: f64,
        var_poxedge_i_db0: f64,
        var_poxedge_i_db1: f64,
        var_poxedge_i_db2: f64,
        var_poxedge_i_db3: f64,
        var_poxedge_i_db4: f64,
        var_poxedge_i_dn0: f64,
        var_poxedge_i_dn1: f64,
        var_poxedge_i_dn2: f64,
        var_poxedge_i_dn3: f64,
        var_poxedge_i_dn4: f64,
        var_poxedge_i_dn5: f64,
        var_poxedge_i_dn6: f64,
        var_poxedge_i_dn7: f64,
        var_poxedge_i_dn8: f64,
        var_vbgs: f64,
        var_vbgs_db0: f64,
        var_vbgs_db1: f64,
        var_vbgs_db2: f64,
        var_vbgs_db3: f64,
        var_vbgs_db4: f64,
        var_vbgs_dn0: f64,
        var_vbgs_dn1: f64,
        var_vbgs_dn2: f64,
        var_vbgs_dn3: f64,
        var_vbgs_dn4: f64,
        var_vbgs_dn5: f64,
        var_vbgs_dn6: f64,
        var_vbgs_dn7: f64,
        var_vbgs_dn8: f64,
        var_vdseff: f64,
        var_vdseff_db0: f64,
        var_vdseff_db1: f64,
        var_vdseff_db2: f64,
        var_vdseff_db3: f64,
        var_vdseff_db4: f64,
        var_vdseff_dn0: f64,
        var_vdseff_dn1: f64,
        var_vdseff_dn2: f64,
        var_vdseff_dn3: f64,
        var_vdseff_dn4: f64,
        var_vdseff_dn5: f64,
        var_vdseff_dn6: f64,
        var_vdseff_dn7: f64,
        var_vdseff_dn8: f64,
        var_vfbsd: f64,
        var_vfbsd_bg: f64,
        var_vfbsd_bg_db0: f64,
        var_vfbsd_bg_db1: f64,
        var_vfbsd_bg_db2: f64,
        var_vfbsd_bg_db3: f64,
        var_vfbsd_bg_db4: f64,
        var_vfbsd_bg_dn0: f64,
        var_vfbsd_bg_dn1: f64,
        var_vfbsd_bg_dn2: f64,
        var_vfbsd_bg_dn3: f64,
        var_vfbsd_bg_dn4: f64,
        var_vfbsd_bg_dn5: f64,
        var_vfbsd_bg_dn6: f64,
        var_vfbsd_bg_dn7: f64,
        var_vfbsd_bg_dn8: f64,
        var_vfbsd_db0: f64,
        var_vfbsd_db1: f64,
        var_vfbsd_db2: f64,
        var_vfbsd_db3: f64,
        var_vfbsd_db4: f64,
        var_vfbsd_dn0: f64,
        var_vfbsd_dn1: f64,
        var_vfbsd_dn2: f64,
        var_vfbsd_dn3: f64,
        var_vfbsd_dn4: f64,
        var_vfbsd_dn5: f64,
        var_vfbsd_dn6: f64,
        var_vfbsd_dn7: f64,
        var_vfbsd_dn8: f64,
        var_vgs_noswap: f64,
        var_vgs_noswap_db0: f64,
        var_vgs_noswap_db1: f64,
        var_vgs_noswap_db2: f64,
        var_vgs_noswap_db3: f64,
        var_vgs_noswap_db4: f64,
        var_vgs_noswap_dn0: f64,
        var_vgs_noswap_dn1: f64,
        var_vgs_noswap_dn2: f64,
        var_vgs_noswap_dn3: f64,
        var_vgs_noswap_dn4: f64,
        var_vgs_noswap_dn5: f64,
        var_vgs_noswap_dn6: f64,
        var_vgs_noswap_dn7: f64,
        var_vgs_noswap_dn8: f64,
        var_t0_slot: &mut f64,
        var_t0_db0_slot: &mut f64,
        var_t0_db1_slot: &mut f64,
        var_t0_db2_slot: &mut f64,
        var_t0_db3_slot: &mut f64,
        var_t0_db4_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rdb0_slot: &mut f64,
        var_t0_rdb1_slot: &mut f64,
        var_t0_rdb2_slot: &mut f64,
        var_t0_rdb3_slot: &mut f64,
        var_t0_rdb4_slot: &mut f64,
        var_t0_rdn0_slot: &mut f64,
        var_t0_rdn1_slot: &mut f64,
        var_t0_rdn2_slot: &mut f64,
        var_t0_rdn3_slot: &mut f64,
        var_t0_rdn4_slot: &mut f64,
        var_t0_rdn5_slot: &mut f64,
        var_t0_rdn6_slot: &mut f64,
        var_t0_rdn7_slot: &mut f64,
        var_t0_rdn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_db0_slot: &mut f64,
        var_t1_db1_slot: &mut f64,
        var_t1_db2_slot: &mut f64,
        var_t1_db3_slot: &mut f64,
        var_t1_db4_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_exp_slot: &mut f64,
        var_t1_exp_db0_slot: &mut f64,
        var_t1_exp_db1_slot: &mut f64,
        var_t1_exp_db2_slot: &mut f64,
        var_t1_exp_db3_slot: &mut f64,
        var_t1_exp_db4_slot: &mut f64,
        var_t1_exp_dn0_slot: &mut f64,
        var_t1_exp_dn1_slot: &mut f64,
        var_t1_exp_dn2_slot: &mut f64,
        var_t1_exp_dn3_slot: &mut f64,
        var_t1_exp_dn4_slot: &mut f64,
        var_t1_exp_dn5_slot: &mut f64,
        var_t1_exp_dn6_slot: &mut f64,
        var_t1_exp_dn7_slot: &mut f64,
        var_t1_exp_dn8_slot: &mut f64,
        var_t1_exp_rdb0_slot: &mut f64,
        var_t1_exp_rdb1_slot: &mut f64,
        var_t1_exp_rdb2_slot: &mut f64,
        var_t1_exp_rdb3_slot: &mut f64,
        var_t1_exp_rdb4_slot: &mut f64,
        var_t1_exp_rdn0_slot: &mut f64,
        var_t1_exp_rdn1_slot: &mut f64,
        var_t1_exp_rdn2_slot: &mut f64,
        var_t1_exp_rdn3_slot: &mut f64,
        var_t1_exp_rdn4_slot: &mut f64,
        var_t1_exp_rdn5_slot: &mut f64,
        var_t1_exp_rdn6_slot: &mut f64,
        var_t1_exp_rdn7_slot: &mut f64,
        var_t1_exp_rdn8_slot: &mut f64,
        var_t1_exp_rv_slot: &mut f64,
        var_t1_rdb0_slot: &mut f64,
        var_t1_rdb1_slot: &mut f64,
        var_t1_rdb2_slot: &mut f64,
        var_t1_rdb3_slot: &mut f64,
        var_t1_rdb4_slot: &mut f64,
        var_t1_rdn0_slot: &mut f64,
        var_t1_rdn1_slot: &mut f64,
        var_t1_rdn2_slot: &mut f64,
        var_t1_rdn3_slot: &mut f64,
        var_t1_rdn4_slot: &mut f64,
        var_t1_rdn5_slot: &mut f64,
        var_t1_rdn6_slot: &mut f64,
        var_t1_rdn7_slot: &mut f64,
        var_t1_rdn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_db0_slot: &mut f64,
        var_t2_db1_slot: &mut f64,
        var_t2_db2_slot: &mut f64,
        var_t2_db3_slot: &mut f64,
        var_t2_db4_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rdb0_slot: &mut f64,
        var_t2_rdb1_slot: &mut f64,
        var_t2_rdb2_slot: &mut f64,
        var_t2_rdb3_slot: &mut f64,
        var_t2_rdb4_slot: &mut f64,
        var_t2_rdn0_slot: &mut f64,
        var_t2_rdn1_slot: &mut f64,
        var_t2_rdn2_slot: &mut f64,
        var_t2_rdn3_slot: &mut f64,
        var_t2_rdn4_slot: &mut f64,
        var_t2_rdn5_slot: &mut f64,
        var_t2_rdn6_slot: &mut f64,
        var_t2_rdn7_slot: &mut f64,
        var_t2_rdn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_db0_slot: &mut f64,
        var_t3_db1_slot: &mut f64,
        var_t3_db2_slot: &mut f64,
        var_t3_db3_slot: &mut f64,
        var_t3_db4_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn1_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rdb0_slot: &mut f64,
        var_t3_rdb1_slot: &mut f64,
        var_t3_rdb2_slot: &mut f64,
        var_t3_rdb3_slot: &mut f64,
        var_t3_rdb4_slot: &mut f64,
        var_t3_rdn0_slot: &mut f64,
        var_t3_rdn1_slot: &mut f64,
        var_t3_rdn2_slot: &mut f64,
        var_t3_rdn3_slot: &mut f64,
        var_t3_rdn4_slot: &mut f64,
        var_t3_rdn5_slot: &mut f64,
        var_t3_rdn6_slot: &mut f64,
        var_t3_rdn7_slot: &mut f64,
        var_t3_rdn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_db0_slot: &mut f64,
        var_t4_db1_slot: &mut f64,
        var_t4_db2_slot: &mut f64,
        var_t4_db3_slot: &mut f64,
        var_t4_db4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn1_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_rdb0_slot: &mut f64,
        var_t4_rdb1_slot: &mut f64,
        var_t4_rdb2_slot: &mut f64,
        var_t4_rdb3_slot: &mut f64,
        var_t4_rdb4_slot: &mut f64,
        var_t4_rdn0_slot: &mut f64,
        var_t4_rdn1_slot: &mut f64,
        var_t4_rdn2_slot: &mut f64,
        var_t4_rdn3_slot: &mut f64,
        var_t4_rdn4_slot: &mut f64,
        var_t4_rdn5_slot: &mut f64,
        var_t4_rdn6_slot: &mut f64,
        var_t4_rdn7_slot: &mut f64,
        var_t4_rdn8_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_db0_slot: &mut f64,
        var_t5_db1_slot: &mut f64,
        var_t5_db2_slot: &mut f64,
        var_t5_db3_slot: &mut f64,
        var_t5_db4_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn1_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_rdb0_slot: &mut f64,
        var_t5_rdb1_slot: &mut f64,
        var_t5_rdb2_slot: &mut f64,
        var_t5_rdb3_slot: &mut f64,
        var_t5_rdb4_slot: &mut f64,
        var_t5_rdn0_slot: &mut f64,
        var_t5_rdn1_slot: &mut f64,
        var_t5_rdn2_slot: &mut f64,
        var_t5_rdn3_slot: &mut f64,
        var_t5_rdn4_slot: &mut f64,
        var_t5_rdn5_slot: &mut f64,
        var_t5_rdn6_slot: &mut f64,
        var_t5_rdn7_slot: &mut f64,
        var_t5_rdn8_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_vdseffx_slot: &mut f64,
        var_vdseffx_db0_slot: &mut f64,
        var_vdseffx_db1_slot: &mut f64,
        var_vdseffx_db2_slot: &mut f64,
        var_vdseffx_db3_slot: &mut f64,
        var_vdseffx_db4_slot: &mut f64,
        var_vdseffx_dn0_slot: &mut f64,
        var_vdseffx_dn1_slot: &mut f64,
        var_vdseffx_dn2_slot: &mut f64,
        var_vdseffx_dn3_slot: &mut f64,
        var_vdseffx_dn4_slot: &mut f64,
        var_vdseffx_dn5_slot: &mut f64,
        var_vdseffx_dn6_slot: &mut f64,
        var_vdseffx_dn7_slot: &mut f64,
        var_vdseffx_dn8_slot: &mut f64,
        var_vdseffx_rdb0_slot: &mut f64,
        var_vdseffx_rdb1_slot: &mut f64,
        var_vdseffx_rdb2_slot: &mut f64,
        var_vdseffx_rdb3_slot: &mut f64,
        var_vdseffx_rdb4_slot: &mut f64,
        var_vdseffx_rdn0_slot: &mut f64,
        var_vdseffx_rdn1_slot: &mut f64,
        var_vdseffx_rdn2_slot: &mut f64,
        var_vdseffx_rdn3_slot: &mut f64,
        var_vdseffx_rdn4_slot: &mut f64,
        var_vdseffx_rdn5_slot: &mut f64,
        var_vdseffx_rdn6_slot: &mut f64,
        var_vdseffx_rdn7_slot: &mut f64,
        var_vdseffx_rdn8_slot: &mut f64,
        var_vdseffx_rv_slot: &mut f64,
        var_vfgs_eff_slot: &mut f64,
        var_vfgs_eff_db0_slot: &mut f64,
        var_vfgs_eff_db1_slot: &mut f64,
        var_vfgs_eff_db2_slot: &mut f64,
        var_vfgs_eff_db3_slot: &mut f64,
        var_vfgs_eff_db4_slot: &mut f64,
        var_vfgs_eff_dn0_slot: &mut f64,
        var_vfgs_eff_dn1_slot: &mut f64,
        var_vfgs_eff_dn2_slot: &mut f64,
        var_vfgs_eff_dn3_slot: &mut f64,
        var_vfgs_eff_dn4_slot: &mut f64,
        var_vfgs_eff_dn5_slot: &mut f64,
        var_vfgs_eff_dn6_slot: &mut f64,
        var_vfgs_eff_dn7_slot: &mut f64,
        var_vfgs_eff_dn8_slot: &mut f64,
        var_vfgs_eff_rdb0_slot: &mut f64,
        var_vfgs_eff_rdb1_slot: &mut f64,
        var_vfgs_eff_rdb2_slot: &mut f64,
        var_vfgs_eff_rdb3_slot: &mut f64,
        var_vfgs_eff_rdb4_slot: &mut f64,
        var_vfgs_eff_rdn0_slot: &mut f64,
        var_vfgs_eff_rdn1_slot: &mut f64,
        var_vfgs_eff_rdn2_slot: &mut f64,
        var_vfgs_eff_rdn3_slot: &mut f64,
        var_vfgs_eff_rdn4_slot: &mut f64,
        var_vfgs_eff_rdn5_slot: &mut f64,
        var_vfgs_eff_rdn6_slot: &mut f64,
        var_vfgs_eff_rdn7_slot: &mut f64,
        var_vfgs_eff_rdn8_slot: &mut f64,
        var_vfgs_eff_rv_slot: &mut f64,
    ) {
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_db0: f64 = *var_t0_db0_slot;
        let mut var_t0_db1: f64 = *var_t0_db1_slot;
        let mut var_t0_db2: f64 = *var_t0_db2_slot;
        let mut var_t0_db3: f64 = *var_t0_db3_slot;
        let mut var_t0_db4: f64 = *var_t0_db4_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rdb0: f64 = *var_t0_rdb0_slot;
        let mut var_t0_rdb1: f64 = *var_t0_rdb1_slot;
        let mut var_t0_rdb2: f64 = *var_t0_rdb2_slot;
        let mut var_t0_rdb3: f64 = *var_t0_rdb3_slot;
        let mut var_t0_rdb4: f64 = *var_t0_rdb4_slot;
        let mut var_t0_rdn0: f64 = *var_t0_rdn0_slot;
        let mut var_t0_rdn1: f64 = *var_t0_rdn1_slot;
        let mut var_t0_rdn2: f64 = *var_t0_rdn2_slot;
        let mut var_t0_rdn3: f64 = *var_t0_rdn3_slot;
        let mut var_t0_rdn4: f64 = *var_t0_rdn4_slot;
        let mut var_t0_rdn5: f64 = *var_t0_rdn5_slot;
        let mut var_t0_rdn6: f64 = *var_t0_rdn6_slot;
        let mut var_t0_rdn7: f64 = *var_t0_rdn7_slot;
        let mut var_t0_rdn8: f64 = *var_t0_rdn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_db0: f64 = *var_t1_db0_slot;
        let mut var_t1_db1: f64 = *var_t1_db1_slot;
        let mut var_t1_db2: f64 = *var_t1_db2_slot;
        let mut var_t1_db3: f64 = *var_t1_db3_slot;
        let mut var_t1_db4: f64 = *var_t1_db4_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_exp: f64 = *var_t1_exp_slot;
        let mut var_t1_exp_db0: f64 = *var_t1_exp_db0_slot;
        let mut var_t1_exp_db1: f64 = *var_t1_exp_db1_slot;
        let mut var_t1_exp_db2: f64 = *var_t1_exp_db2_slot;
        let mut var_t1_exp_db3: f64 = *var_t1_exp_db3_slot;
        let mut var_t1_exp_db4: f64 = *var_t1_exp_db4_slot;
        let mut var_t1_exp_dn0: f64 = *var_t1_exp_dn0_slot;
        let mut var_t1_exp_dn1: f64 = *var_t1_exp_dn1_slot;
        let mut var_t1_exp_dn2: f64 = *var_t1_exp_dn2_slot;
        let mut var_t1_exp_dn3: f64 = *var_t1_exp_dn3_slot;
        let mut var_t1_exp_dn4: f64 = *var_t1_exp_dn4_slot;
        let mut var_t1_exp_dn5: f64 = *var_t1_exp_dn5_slot;
        let mut var_t1_exp_dn6: f64 = *var_t1_exp_dn6_slot;
        let mut var_t1_exp_dn7: f64 = *var_t1_exp_dn7_slot;
        let mut var_t1_exp_dn8: f64 = *var_t1_exp_dn8_slot;
        let mut var_t1_exp_rdb0: f64 = *var_t1_exp_rdb0_slot;
        let mut var_t1_exp_rdb1: f64 = *var_t1_exp_rdb1_slot;
        let mut var_t1_exp_rdb2: f64 = *var_t1_exp_rdb2_slot;
        let mut var_t1_exp_rdb3: f64 = *var_t1_exp_rdb3_slot;
        let mut var_t1_exp_rdb4: f64 = *var_t1_exp_rdb4_slot;
        let mut var_t1_exp_rdn0: f64 = *var_t1_exp_rdn0_slot;
        let mut var_t1_exp_rdn1: f64 = *var_t1_exp_rdn1_slot;
        let mut var_t1_exp_rdn2: f64 = *var_t1_exp_rdn2_slot;
        let mut var_t1_exp_rdn3: f64 = *var_t1_exp_rdn3_slot;
        let mut var_t1_exp_rdn4: f64 = *var_t1_exp_rdn4_slot;
        let mut var_t1_exp_rdn5: f64 = *var_t1_exp_rdn5_slot;
        let mut var_t1_exp_rdn6: f64 = *var_t1_exp_rdn6_slot;
        let mut var_t1_exp_rdn7: f64 = *var_t1_exp_rdn7_slot;
        let mut var_t1_exp_rdn8: f64 = *var_t1_exp_rdn8_slot;
        let mut var_t1_exp_rv: f64 = *var_t1_exp_rv_slot;
        let mut var_t1_rdb0: f64 = *var_t1_rdb0_slot;
        let mut var_t1_rdb1: f64 = *var_t1_rdb1_slot;
        let mut var_t1_rdb2: f64 = *var_t1_rdb2_slot;
        let mut var_t1_rdb3: f64 = *var_t1_rdb3_slot;
        let mut var_t1_rdb4: f64 = *var_t1_rdb4_slot;
        let mut var_t1_rdn0: f64 = *var_t1_rdn0_slot;
        let mut var_t1_rdn1: f64 = *var_t1_rdn1_slot;
        let mut var_t1_rdn2: f64 = *var_t1_rdn2_slot;
        let mut var_t1_rdn3: f64 = *var_t1_rdn3_slot;
        let mut var_t1_rdn4: f64 = *var_t1_rdn4_slot;
        let mut var_t1_rdn5: f64 = *var_t1_rdn5_slot;
        let mut var_t1_rdn6: f64 = *var_t1_rdn6_slot;
        let mut var_t1_rdn7: f64 = *var_t1_rdn7_slot;
        let mut var_t1_rdn8: f64 = *var_t1_rdn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_db0: f64 = *var_t2_db0_slot;
        let mut var_t2_db1: f64 = *var_t2_db1_slot;
        let mut var_t2_db2: f64 = *var_t2_db2_slot;
        let mut var_t2_db3: f64 = *var_t2_db3_slot;
        let mut var_t2_db4: f64 = *var_t2_db4_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rdb0: f64 = *var_t2_rdb0_slot;
        let mut var_t2_rdb1: f64 = *var_t2_rdb1_slot;
        let mut var_t2_rdb2: f64 = *var_t2_rdb2_slot;
        let mut var_t2_rdb3: f64 = *var_t2_rdb3_slot;
        let mut var_t2_rdb4: f64 = *var_t2_rdb4_slot;
        let mut var_t2_rdn0: f64 = *var_t2_rdn0_slot;
        let mut var_t2_rdn1: f64 = *var_t2_rdn1_slot;
        let mut var_t2_rdn2: f64 = *var_t2_rdn2_slot;
        let mut var_t2_rdn3: f64 = *var_t2_rdn3_slot;
        let mut var_t2_rdn4: f64 = *var_t2_rdn4_slot;
        let mut var_t2_rdn5: f64 = *var_t2_rdn5_slot;
        let mut var_t2_rdn6: f64 = *var_t2_rdn6_slot;
        let mut var_t2_rdn7: f64 = *var_t2_rdn7_slot;
        let mut var_t2_rdn8: f64 = *var_t2_rdn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_db0: f64 = *var_t3_db0_slot;
        let mut var_t3_db1: f64 = *var_t3_db1_slot;
        let mut var_t3_db2: f64 = *var_t3_db2_slot;
        let mut var_t3_db3: f64 = *var_t3_db3_slot;
        let mut var_t3_db4: f64 = *var_t3_db4_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn1: f64 = *var_t3_dn1_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rdb0: f64 = *var_t3_rdb0_slot;
        let mut var_t3_rdb1: f64 = *var_t3_rdb1_slot;
        let mut var_t3_rdb2: f64 = *var_t3_rdb2_slot;
        let mut var_t3_rdb3: f64 = *var_t3_rdb3_slot;
        let mut var_t3_rdb4: f64 = *var_t3_rdb4_slot;
        let mut var_t3_rdn0: f64 = *var_t3_rdn0_slot;
        let mut var_t3_rdn1: f64 = *var_t3_rdn1_slot;
        let mut var_t3_rdn2: f64 = *var_t3_rdn2_slot;
        let mut var_t3_rdn3: f64 = *var_t3_rdn3_slot;
        let mut var_t3_rdn4: f64 = *var_t3_rdn4_slot;
        let mut var_t3_rdn5: f64 = *var_t3_rdn5_slot;
        let mut var_t3_rdn6: f64 = *var_t3_rdn6_slot;
        let mut var_t3_rdn7: f64 = *var_t3_rdn7_slot;
        let mut var_t3_rdn8: f64 = *var_t3_rdn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_db0: f64 = *var_t4_db0_slot;
        let mut var_t4_db1: f64 = *var_t4_db1_slot;
        let mut var_t4_db2: f64 = *var_t4_db2_slot;
        let mut var_t4_db3: f64 = *var_t4_db3_slot;
        let mut var_t4_db4: f64 = *var_t4_db4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn1: f64 = *var_t4_dn1_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_rdb0: f64 = *var_t4_rdb0_slot;
        let mut var_t4_rdb1: f64 = *var_t4_rdb1_slot;
        let mut var_t4_rdb2: f64 = *var_t4_rdb2_slot;
        let mut var_t4_rdb3: f64 = *var_t4_rdb3_slot;
        let mut var_t4_rdb4: f64 = *var_t4_rdb4_slot;
        let mut var_t4_rdn0: f64 = *var_t4_rdn0_slot;
        let mut var_t4_rdn1: f64 = *var_t4_rdn1_slot;
        let mut var_t4_rdn2: f64 = *var_t4_rdn2_slot;
        let mut var_t4_rdn3: f64 = *var_t4_rdn3_slot;
        let mut var_t4_rdn4: f64 = *var_t4_rdn4_slot;
        let mut var_t4_rdn5: f64 = *var_t4_rdn5_slot;
        let mut var_t4_rdn6: f64 = *var_t4_rdn6_slot;
        let mut var_t4_rdn7: f64 = *var_t4_rdn7_slot;
        let mut var_t4_rdn8: f64 = *var_t4_rdn8_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_db0: f64 = *var_t5_db0_slot;
        let mut var_t5_db1: f64 = *var_t5_db1_slot;
        let mut var_t5_db2: f64 = *var_t5_db2_slot;
        let mut var_t5_db3: f64 = *var_t5_db3_slot;
        let mut var_t5_db4: f64 = *var_t5_db4_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn1: f64 = *var_t5_dn1_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_rdb0: f64 = *var_t5_rdb0_slot;
        let mut var_t5_rdb1: f64 = *var_t5_rdb1_slot;
        let mut var_t5_rdb2: f64 = *var_t5_rdb2_slot;
        let mut var_t5_rdb3: f64 = *var_t5_rdb3_slot;
        let mut var_t5_rdb4: f64 = *var_t5_rdb4_slot;
        let mut var_t5_rdn0: f64 = *var_t5_rdn0_slot;
        let mut var_t5_rdn1: f64 = *var_t5_rdn1_slot;
        let mut var_t5_rdn2: f64 = *var_t5_rdn2_slot;
        let mut var_t5_rdn3: f64 = *var_t5_rdn3_slot;
        let mut var_t5_rdn4: f64 = *var_t5_rdn4_slot;
        let mut var_t5_rdn5: f64 = *var_t5_rdn5_slot;
        let mut var_t5_rdn6: f64 = *var_t5_rdn6_slot;
        let mut var_t5_rdn7: f64 = *var_t5_rdn7_slot;
        let mut var_t5_rdn8: f64 = *var_t5_rdn8_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_vdseffx: f64 = *var_vdseffx_slot;
        let mut var_vdseffx_db0: f64 = *var_vdseffx_db0_slot;
        let mut var_vdseffx_db1: f64 = *var_vdseffx_db1_slot;
        let mut var_vdseffx_db2: f64 = *var_vdseffx_db2_slot;
        let mut var_vdseffx_db3: f64 = *var_vdseffx_db3_slot;
        let mut var_vdseffx_db4: f64 = *var_vdseffx_db4_slot;
        let mut var_vdseffx_dn0: f64 = *var_vdseffx_dn0_slot;
        let mut var_vdseffx_dn1: f64 = *var_vdseffx_dn1_slot;
        let mut var_vdseffx_dn2: f64 = *var_vdseffx_dn2_slot;
        let mut var_vdseffx_dn3: f64 = *var_vdseffx_dn3_slot;
        let mut var_vdseffx_dn4: f64 = *var_vdseffx_dn4_slot;
        let mut var_vdseffx_dn5: f64 = *var_vdseffx_dn5_slot;
        let mut var_vdseffx_dn6: f64 = *var_vdseffx_dn6_slot;
        let mut var_vdseffx_dn7: f64 = *var_vdseffx_dn7_slot;
        let mut var_vdseffx_dn8: f64 = *var_vdseffx_dn8_slot;
        let mut var_vdseffx_rdb0: f64 = *var_vdseffx_rdb0_slot;
        let mut var_vdseffx_rdb1: f64 = *var_vdseffx_rdb1_slot;
        let mut var_vdseffx_rdb2: f64 = *var_vdseffx_rdb2_slot;
        let mut var_vdseffx_rdb3: f64 = *var_vdseffx_rdb3_slot;
        let mut var_vdseffx_rdb4: f64 = *var_vdseffx_rdb4_slot;
        let mut var_vdseffx_rdn0: f64 = *var_vdseffx_rdn0_slot;
        let mut var_vdseffx_rdn1: f64 = *var_vdseffx_rdn1_slot;
        let mut var_vdseffx_rdn2: f64 = *var_vdseffx_rdn2_slot;
        let mut var_vdseffx_rdn3: f64 = *var_vdseffx_rdn3_slot;
        let mut var_vdseffx_rdn4: f64 = *var_vdseffx_rdn4_slot;
        let mut var_vdseffx_rdn5: f64 = *var_vdseffx_rdn5_slot;
        let mut var_vdseffx_rdn6: f64 = *var_vdseffx_rdn6_slot;
        let mut var_vdseffx_rdn7: f64 = *var_vdseffx_rdn7_slot;
        let mut var_vdseffx_rdn8: f64 = *var_vdseffx_rdn8_slot;
        let mut var_vdseffx_rv: f64 = *var_vdseffx_rv_slot;
        let mut var_vfgs_eff: f64 = *var_vfgs_eff_slot;
        let mut var_vfgs_eff_db0: f64 = *var_vfgs_eff_db0_slot;
        let mut var_vfgs_eff_db1: f64 = *var_vfgs_eff_db1_slot;
        let mut var_vfgs_eff_db2: f64 = *var_vfgs_eff_db2_slot;
        let mut var_vfgs_eff_db3: f64 = *var_vfgs_eff_db3_slot;
        let mut var_vfgs_eff_db4: f64 = *var_vfgs_eff_db4_slot;
        let mut var_vfgs_eff_dn0: f64 = *var_vfgs_eff_dn0_slot;
        let mut var_vfgs_eff_dn1: f64 = *var_vfgs_eff_dn1_slot;
        let mut var_vfgs_eff_dn2: f64 = *var_vfgs_eff_dn2_slot;
        let mut var_vfgs_eff_dn3: f64 = *var_vfgs_eff_dn3_slot;
        let mut var_vfgs_eff_dn4: f64 = *var_vfgs_eff_dn4_slot;
        let mut var_vfgs_eff_dn5: f64 = *var_vfgs_eff_dn5_slot;
        let mut var_vfgs_eff_dn6: f64 = *var_vfgs_eff_dn6_slot;
        let mut var_vfgs_eff_dn7: f64 = *var_vfgs_eff_dn7_slot;
        let mut var_vfgs_eff_dn8: f64 = *var_vfgs_eff_dn8_slot;
        let mut var_vfgs_eff_rdb0: f64 = *var_vfgs_eff_rdb0_slot;
        let mut var_vfgs_eff_rdb1: f64 = *var_vfgs_eff_rdb1_slot;
        let mut var_vfgs_eff_rdb2: f64 = *var_vfgs_eff_rdb2_slot;
        let mut var_vfgs_eff_rdb3: f64 = *var_vfgs_eff_rdb3_slot;
        let mut var_vfgs_eff_rdb4: f64 = *var_vfgs_eff_rdb4_slot;
        let mut var_vfgs_eff_rdn0: f64 = *var_vfgs_eff_rdn0_slot;
        let mut var_vfgs_eff_rdn1: f64 = *var_vfgs_eff_rdn1_slot;
        let mut var_vfgs_eff_rdn2: f64 = *var_vfgs_eff_rdn2_slot;
        let mut var_vfgs_eff_rdn3: f64 = *var_vfgs_eff_rdn3_slot;
        let mut var_vfgs_eff_rdn4: f64 = *var_vfgs_eff_rdn4_slot;
        let mut var_vfgs_eff_rdn5: f64 = *var_vfgs_eff_rdn5_slot;
        let mut var_vfgs_eff_rdn6: f64 = *var_vfgs_eff_rdn6_slot;
        let mut var_vfgs_eff_rdn7: f64 = *var_vfgs_eff_rdn7_slot;
        let mut var_vfgs_eff_rdn8: f64 = *var_vfgs_eff_rdn8_slot;
        let mut var_vfgs_eff_rv: f64 = *var_vfgs_eff_rv_slot;

        let (assign12180_e11305, assign12180_e11305_d_n0, assign12180_e11305_d_n1, assign12180_e11305_d_n2, assign12180_e11305_d_n3, assign12180_e11305_d_n4, assign12180_e11305_d_n5, assign12180_e11305_d_n6, assign12180_e11305_d_n7, assign12180_e11305_d_n8, assign12180_e11305_d_b0, assign12180_e11305_d_b1, assign12180_e11305_d_b2, assign12180_e11305_d_b3, assign12180_e11305_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12180_e11298: f64 = (var_vdseff * var_vdseff);
        let assign12180_e11300: f64 = (assign12180_e11298 + 0.01);
        let assign12180_e11301: f64 = (assign12180_e11300).sqrt();
        let assign12180_e11303: f64 = (assign12180_e11301 - 0.1);
        (assign12180_e11303, (((var_vdseff_dn0 * var_vdseff) + (var_vdseff * var_vdseff_dn0)) / (2.0 * assign12180_e11301)), (((var_vdseff_dn1 * var_vdseff) + (var_vdseff * var_vdseff_dn1)) / (2.0 * assign12180_e11301)), (((var_vdseff_dn2 * var_vdseff) + (var_vdseff * var_vdseff_dn2)) / (2.0 * assign12180_e11301)), (((var_vdseff_dn3 * var_vdseff) + (var_vdseff * var_vdseff_dn3)) / (2.0 * assign12180_e11301)), (((var_vdseff_dn4 * var_vdseff) + (var_vdseff * var_vdseff_dn4)) / (2.0 * assign12180_e11301)), (((var_vdseff_dn5 * var_vdseff) + (var_vdseff * var_vdseff_dn5)) / (2.0 * assign12180_e11301)), (((var_vdseff_dn6 * var_vdseff) + (var_vdseff * var_vdseff_dn6)) / (2.0 * assign12180_e11301)), (((var_vdseff_dn7 * var_vdseff) + (var_vdseff * var_vdseff_dn7)) / (2.0 * assign12180_e11301)), (((var_vdseff_dn8 * var_vdseff) + (var_vdseff * var_vdseff_dn8)) / (2.0 * assign12180_e11301)), (((var_vdseff_db0 * var_vdseff) + (var_vdseff * var_vdseff_db0)) / (2.0 * assign12180_e11301)), (((var_vdseff_db1 * var_vdseff) + (var_vdseff * var_vdseff_db1)) / (2.0 * assign12180_e11301)), (((var_vdseff_db2 * var_vdseff) + (var_vdseff * var_vdseff_db2)) / (2.0 * assign12180_e11301)), (((var_vdseff_db3 * var_vdseff) + (var_vdseff * var_vdseff_db3)) / (2.0 * assign12180_e11301)), (((var_vdseff_db4 * var_vdseff) + (var_vdseff * var_vdseff_db4)) / (2.0 * assign12180_e11301)),)
    } else {
        (var_vdseffx, var_vdseffx_dn0, var_vdseffx_dn1, var_vdseffx_dn2, var_vdseffx_dn3, var_vdseffx_dn4, var_vdseffx_dn5, var_vdseffx_dn6, var_vdseffx_dn7, var_vdseffx_dn8, var_vdseffx_db0, var_vdseffx_db1, var_vdseffx_db2, var_vdseffx_db3, var_vdseffx_db4,)
    }
};
        var_vdseffx = assign12180_e11305;
        var_vdseffx_dn0 = assign12180_e11305_d_n0;
        var_vdseffx_dn1 = assign12180_e11305_d_n1;
        var_vdseffx_dn2 = assign12180_e11305_d_n2;
        var_vdseffx_dn3 = assign12180_e11305_d_n3;
        var_vdseffx_dn4 = assign12180_e11305_d_n4;
        var_vdseffx_dn5 = assign12180_e11305_d_n5;
        var_vdseffx_dn6 = assign12180_e11305_d_n6;
        var_vdseffx_dn7 = assign12180_e11305_d_n7;
        var_vdseffx_dn8 = assign12180_e11305_d_n8;
        var_vdseffx_db0 = assign12180_e11305_d_b0;
        var_vdseffx_db1 = assign12180_e11305_d_b1;
        var_vdseffx_db2 = assign12180_e11305_d_b2;
        var_vdseffx_db3 = assign12180_e11305_d_b3;
        var_vdseffx_db4 = assign12180_e11305_d_b4;
        var_vdseffx_rv = 0.0;
        var_vdseffx_rdn0 = 0.0;
        var_vdseffx_rdn1 = 0.0;
        var_vdseffx_rdn2 = 0.0;
        var_vdseffx_rdn3 = 0.0;
        var_vdseffx_rdn4 = 0.0;
        var_vdseffx_rdn5 = 0.0;
        var_vdseffx_rdn6 = 0.0;
        var_vdseffx_rdn7 = 0.0;
        var_vdseffx_rdn8 = 0.0;
        var_vdseffx_rdb0 = 0.0;
        var_vdseffx_rdb1 = 0.0;
        var_vdseffx_rdb2 = 0.0;
        var_vdseffx_rdb3 = 0.0;
        var_vdseffx_rdb4 = 0.0;

        let (assign12190_e11311, assign12190_e11311_d_n0, assign12190_e11311_d_n1, assign12190_e11311_d_n2, assign12190_e11311_d_n3, assign12190_e11311_d_n4, assign12190_e11311_d_n5, assign12190_e11311_d_n6, assign12190_e11311_d_n7, assign12190_e11311_d_n8, assign12190_e11311_d_b0, assign12190_e11311_d_b1, assign12190_e11311_d_b2, assign12190_e11311_d_b3, assign12190_e11311_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12190_e11309: f64 = (var_pigcd_i * var_vdseffx);
        (assign12190_e11309, ((var_pigcd_i_dn0 * var_vdseffx) + (var_pigcd_i * var_vdseffx_dn0)), ((var_pigcd_i_dn1 * var_vdseffx) + (var_pigcd_i * var_vdseffx_dn1)), ((var_pigcd_i_dn2 * var_vdseffx) + (var_pigcd_i * var_vdseffx_dn2)), ((var_pigcd_i_dn3 * var_vdseffx) + (var_pigcd_i * var_vdseffx_dn3)), ((var_pigcd_i_dn4 * var_vdseffx) + (var_pigcd_i * var_vdseffx_dn4)), ((var_pigcd_i_dn5 * var_vdseffx) + (var_pigcd_i * var_vdseffx_dn5)), ((var_pigcd_i_dn6 * var_vdseffx) + (var_pigcd_i * var_vdseffx_dn6)), ((var_pigcd_i_dn7 * var_vdseffx) + (var_pigcd_i * var_vdseffx_dn7)), ((var_pigcd_i_dn8 * var_vdseffx) + (var_pigcd_i * var_vdseffx_dn8)), ((var_pigcd_i_db0 * var_vdseffx) + (var_pigcd_i * var_vdseffx_db0)), ((var_pigcd_i_db1 * var_vdseffx) + (var_pigcd_i * var_vdseffx_db1)), ((var_pigcd_i_db2 * var_vdseffx) + (var_pigcd_i * var_vdseffx_db2)), ((var_pigcd_i_db3 * var_vdseffx) + (var_pigcd_i * var_vdseffx_db3)), ((var_pigcd_i_db4 * var_vdseffx) + (var_pigcd_i * var_vdseffx_db4)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4,)
    }
};
        var_t1 = assign12190_e11311;
        var_t1_dn0 = assign12190_e11311_d_n0;
        var_t1_dn1 = assign12190_e11311_d_n1;
        var_t1_dn2 = assign12190_e11311_d_n2;
        var_t1_dn3 = assign12190_e11311_d_n3;
        var_t1_dn4 = assign12190_e11311_d_n4;
        var_t1_dn5 = assign12190_e11311_d_n5;
        var_t1_dn6 = assign12190_e11311_d_n6;
        var_t1_dn7 = assign12190_e11311_d_n7;
        var_t1_dn8 = assign12190_e11311_d_n8;
        var_t1_db0 = assign12190_e11311_d_b0;
        var_t1_db1 = assign12190_e11311_d_b1;
        var_t1_db2 = assign12190_e11311_d_b2;
        var_t1_db3 = assign12190_e11311_d_b3;
        var_t1_db4 = assign12190_e11311_d_b4;
        var_t1_rv = 0.0;
        var_t1_rdn0 = 0.0;
        var_t1_rdn1 = 0.0;
        var_t1_rdn2 = 0.0;
        var_t1_rdn3 = 0.0;
        var_t1_rdn4 = 0.0;
        var_t1_rdn5 = 0.0;
        var_t1_rdn6 = 0.0;
        var_t1_rdn7 = 0.0;
        var_t1_rdn8 = 0.0;
        var_t1_rdb0 = 0.0;
        var_t1_rdb1 = 0.0;
        var_t1_rdb2 = 0.0;
        var_t1_rdb3 = 0.0;
        var_t1_rdb4 = 0.0;

        let (assign12200_e11317, assign12200_e11317_d_n0, assign12200_e11317_d_n1, assign12200_e11317_d_n2, assign12200_e11317_d_n3, assign12200_e11317_d_n4, assign12200_e11317_d_n5, assign12200_e11317_d_n6, assign12200_e11317_d_n7, assign12200_e11317_d_n8, assign12200_e11317_d_b0, assign12200_e11317_d_b1, assign12200_e11317_d_b2, assign12200_e11317_d_b3, assign12200_e11317_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12200_e11314: f64 = (-var_t1);
        let assign12200_e11315: f64 = { let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12200_e11315, ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn0)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn1)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn2)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn3)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn4)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn5)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn6)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn7)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn8)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_db0)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_db1)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_db2)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_db3)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_db4)),)
    } else {
        (var_t1_exp, var_t1_exp_dn0, var_t1_exp_dn1, var_t1_exp_dn2, var_t1_exp_dn3, var_t1_exp_dn4, var_t1_exp_dn5, var_t1_exp_dn6, var_t1_exp_dn7, var_t1_exp_dn8, var_t1_exp_db0, var_t1_exp_db1, var_t1_exp_db2, var_t1_exp_db3, var_t1_exp_db4,)
    }
};
        var_t1_exp = assign12200_e11317;
        var_t1_exp_dn0 = assign12200_e11317_d_n0;
        var_t1_exp_dn1 = assign12200_e11317_d_n1;
        var_t1_exp_dn2 = assign12200_e11317_d_n2;
        var_t1_exp_dn3 = assign12200_e11317_d_n3;
        var_t1_exp_dn4 = assign12200_e11317_d_n4;
        var_t1_exp_dn5 = assign12200_e11317_d_n5;
        var_t1_exp_dn6 = assign12200_e11317_d_n6;
        var_t1_exp_dn7 = assign12200_e11317_d_n7;
        var_t1_exp_dn8 = assign12200_e11317_d_n8;
        var_t1_exp_db0 = assign12200_e11317_d_b0;
        var_t1_exp_db1 = assign12200_e11317_d_b1;
        var_t1_exp_db2 = assign12200_e11317_d_b2;
        var_t1_exp_db3 = assign12200_e11317_d_b3;
        var_t1_exp_db4 = assign12200_e11317_d_b4;
        var_t1_exp_rv = 0.0;
        var_t1_exp_rdn0 = 0.0;
        var_t1_exp_rdn1 = 0.0;
        var_t1_exp_rdn2 = 0.0;
        var_t1_exp_rdn3 = 0.0;
        var_t1_exp_rdn4 = 0.0;
        var_t1_exp_rdn5 = 0.0;
        var_t1_exp_rdn6 = 0.0;
        var_t1_exp_rdn7 = 0.0;
        var_t1_exp_rdn8 = 0.0;
        var_t1_exp_rdb0 = 0.0;
        var_t1_exp_rdb1 = 0.0;
        var_t1_exp_rdb2 = 0.0;
        var_t1_exp_rdb3 = 0.0;
        var_t1_exp_rdb4 = 0.0;

        let (assign12210_e11327, assign12210_e11327_d_n0, assign12210_e11327_d_n1, assign12210_e11327_d_n2, assign12210_e11327_d_n3, assign12210_e11327_d_n4, assign12210_e11327_d_n5, assign12210_e11327_d_n6, assign12210_e11327_d_n7, assign12210_e11327_d_n8, assign12210_e11327_d_b0, assign12210_e11327_d_b1, assign12210_e11327_d_b2, assign12210_e11327_d_b3, assign12210_e11327_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12210_e11321: f64 = (var_t1 + var_t1_exp);
        let assign12210_e11323: f64 = (assign12210_e11321 - 1.0);
        let assign12210_e11325: f64 = (assign12210_e11323 + 0.0001);
        (assign12210_e11325, (var_t1_dn0 + var_t1_exp_dn0), (var_t1_dn1 + var_t1_exp_dn1), (var_t1_dn2 + var_t1_exp_dn2), (var_t1_dn3 + var_t1_exp_dn3), (var_t1_dn4 + var_t1_exp_dn4), (var_t1_dn5 + var_t1_exp_dn5), (var_t1_dn6 + var_t1_exp_dn6), (var_t1_dn7 + var_t1_exp_dn7), (var_t1_dn8 + var_t1_exp_dn8), (var_t1_db0 + var_t1_exp_db0), (var_t1_db1 + var_t1_exp_db1), (var_t1_db2 + var_t1_exp_db2), (var_t1_db3 + var_t1_exp_db3), (var_t1_db4 + var_t1_exp_db4),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn1, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_db0, var_t3_db1, var_t3_db2, var_t3_db3, var_t3_db4,)
    }
};
        var_t3 = assign12210_e11327;
        var_t3_dn0 = assign12210_e11327_d_n0;
        var_t3_dn1 = assign12210_e11327_d_n1;
        var_t3_dn2 = assign12210_e11327_d_n2;
        var_t3_dn3 = assign12210_e11327_d_n3;
        var_t3_dn4 = assign12210_e11327_d_n4;
        var_t3_dn5 = assign12210_e11327_d_n5;
        var_t3_dn6 = assign12210_e11327_d_n6;
        var_t3_dn7 = assign12210_e11327_d_n7;
        var_t3_dn8 = assign12210_e11327_d_n8;
        var_t3_db0 = assign12210_e11327_d_b0;
        var_t3_db1 = assign12210_e11327_d_b1;
        var_t3_db2 = assign12210_e11327_d_b2;
        var_t3_db3 = assign12210_e11327_d_b3;
        var_t3_db4 = assign12210_e11327_d_b4;
        var_t3_rv = 0.0;
        var_t3_rdn0 = 0.0;
        var_t3_rdn1 = 0.0;
        var_t3_rdn2 = 0.0;
        var_t3_rdn3 = 0.0;
        var_t3_rdn4 = 0.0;
        var_t3_rdn5 = 0.0;
        var_t3_rdn6 = 0.0;
        var_t3_rdn7 = 0.0;
        var_t3_rdn8 = 0.0;
        var_t3_rdb0 = 0.0;
        var_t3_rdb1 = 0.0;
        var_t3_rdb2 = 0.0;
        var_t3_rdb3 = 0.0;
        var_t3_rdb4 = 0.0;

        let (assign12220_e11339, assign12220_e11339_d_n0, assign12220_e11339_d_n1, assign12220_e11339_d_n2, assign12220_e11339_d_n3, assign12220_e11339_d_n4, assign12220_e11339_d_n5, assign12220_e11339_d_n6, assign12220_e11339_d_n7, assign12220_e11339_d_n8, assign12220_e11339_d_b0, assign12220_e11339_d_b1, assign12220_e11339_d_b2, assign12220_e11339_d_b3, assign12220_e11339_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12220_e11332: f64 = (var_t1 + 1.0);
        let assign12220_e11334: f64 = (assign12220_e11332 * var_t1_exp);
        let assign12220_e11335: f64 = (1.0 - assign12220_e11334);
        let assign12220_e11337: f64 = (assign12220_e11335 + 0.0001);
        (assign12220_e11337, (-((var_t1_dn0 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn0))), (-((var_t1_dn1 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn1))), (-((var_t1_dn2 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn2))), (-((var_t1_dn3 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn3))), (-((var_t1_dn4 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn4))), (-((var_t1_dn5 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn5))), (-((var_t1_dn6 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn6))), (-((var_t1_dn7 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn7))), (-((var_t1_dn8 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn8))), (-((var_t1_db0 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_db0))), (-((var_t1_db1 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_db1))), (-((var_t1_db2 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_db2))), (-((var_t1_db3 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_db3))), (-((var_t1_db4 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_db4))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn1, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_db0, var_t4_db1, var_t4_db2, var_t4_db3, var_t4_db4,)
    }
};
        var_t4 = assign12220_e11339;
        var_t4_dn0 = assign12220_e11339_d_n0;
        var_t4_dn1 = assign12220_e11339_d_n1;
        var_t4_dn2 = assign12220_e11339_d_n2;
        var_t4_dn3 = assign12220_e11339_d_n3;
        var_t4_dn4 = assign12220_e11339_d_n4;
        var_t4_dn5 = assign12220_e11339_d_n5;
        var_t4_dn6 = assign12220_e11339_d_n6;
        var_t4_dn7 = assign12220_e11339_d_n7;
        var_t4_dn8 = assign12220_e11339_d_n8;
        var_t4_db0 = assign12220_e11339_d_b0;
        var_t4_db1 = assign12220_e11339_d_b1;
        var_t4_db2 = assign12220_e11339_d_b2;
        var_t4_db3 = assign12220_e11339_d_b3;
        var_t4_db4 = assign12220_e11339_d_b4;
        var_t4_rv = 0.0;
        var_t4_rdn0 = 0.0;
        var_t4_rdn1 = 0.0;
        var_t4_rdn2 = 0.0;
        var_t4_rdn3 = 0.0;
        var_t4_rdn4 = 0.0;
        var_t4_rdn5 = 0.0;
        var_t4_rdn6 = 0.0;
        var_t4_rdn7 = 0.0;
        var_t4_rdn8 = 0.0;
        var_t4_rdb0 = 0.0;
        var_t4_rdb1 = 0.0;
        var_t4_rdb2 = 0.0;
        var_t4_rdb3 = 0.0;
        var_t4_rdb4 = 0.0;

        let (assign12230_e11347, assign12230_e11347_d_n0, assign12230_e11347_d_n1, assign12230_e11347_d_n2, assign12230_e11347_d_n3, assign12230_e11347_d_n4, assign12230_e11347_d_n5, assign12230_e11347_d_n6, assign12230_e11347_d_n7, assign12230_e11347_d_n8, assign12230_e11347_d_b0, assign12230_e11347_d_b1, assign12230_e11347_d_b2, assign12230_e11347_d_b3, assign12230_e11347_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12230_e11343: f64 = (var_t1 * var_t1);
        let assign12230_e11345: f64 = (assign12230_e11343 + 0.0002);
        (assign12230_e11345, ((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)), ((var_t1_dn1 * var_t1) + (var_t1 * var_t1_dn1)), ((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)), ((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3)), ((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)), ((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)), ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)), ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)), ((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)), ((var_t1_db0 * var_t1) + (var_t1 * var_t1_db0)), ((var_t1_db1 * var_t1) + (var_t1 * var_t1_db1)), ((var_t1_db2 * var_t1) + (var_t1 * var_t1_db2)), ((var_t1_db3 * var_t1) + (var_t1 * var_t1_db3)), ((var_t1_db4 * var_t1) + (var_t1 * var_t1_db4)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn1, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_db0, var_t5_db1, var_t5_db2, var_t5_db3, var_t5_db4,)
    }
};
        var_t5 = assign12230_e11347;
        var_t5_dn0 = assign12230_e11347_d_n0;
        var_t5_dn1 = assign12230_e11347_d_n1;
        var_t5_dn2 = assign12230_e11347_d_n2;
        var_t5_dn3 = assign12230_e11347_d_n3;
        var_t5_dn4 = assign12230_e11347_d_n4;
        var_t5_dn5 = assign12230_e11347_d_n5;
        var_t5_dn6 = assign12230_e11347_d_n6;
        var_t5_dn7 = assign12230_e11347_d_n7;
        var_t5_dn8 = assign12230_e11347_d_n8;
        var_t5_db0 = assign12230_e11347_d_b0;
        var_t5_db1 = assign12230_e11347_d_b1;
        var_t5_db2 = assign12230_e11347_d_b2;
        var_t5_db3 = assign12230_e11347_d_b3;
        var_t5_db4 = assign12230_e11347_d_b4;
        var_t5_rv = 0.0;
        var_t5_rdn0 = 0.0;
        var_t5_rdn1 = 0.0;
        var_t5_rdn2 = 0.0;
        var_t5_rdn3 = 0.0;
        var_t5_rdn4 = 0.0;
        var_t5_rdn5 = 0.0;
        var_t5_rdn6 = 0.0;
        var_t5_rdn7 = 0.0;
        var_t5_rdn8 = 0.0;
        var_t5_rdb0 = 0.0;
        var_t5_rdb1 = 0.0;
        var_t5_rdb2 = 0.0;
        var_t5_rdb3 = 0.0;
        var_t5_rdb4 = 0.0;

        let (assign12260_e11377, assign12260_e11377_d_n0, assign12260_e11377_d_n1, assign12260_e11377_d_n2, assign12260_e11377_d_n3, assign12260_e11377_d_n4, assign12260_e11377_d_n5, assign12260_e11377_d_n6, assign12260_e11377_d_n7, assign12260_e11377_d_n8, assign12260_e11377_d_b0, assign12260_e11377_d_b1, assign12260_e11377_d_b2, assign12260_e11377_d_b3, assign12260_e11377_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12260_e11367: f64 = (var_vgs_noswap - var_vfbsd);
        let assign12260_e11370: f64 = (var_digs_i * var_gamma0);
        let assign12260_e11373: f64 = (var_vbgs - var_vfbsd_bg);
        let assign12260_e11374: f64 = (assign12260_e11370 * assign12260_e11373);
        let assign12260_e11375: f64 = (assign12260_e11367 + assign12260_e11374);
        (assign12260_e11375, ((var_vgs_noswap_dn0 - var_vfbsd_dn0) + ((((var_digs_i_dn0 * var_gamma0) + (var_digs_i * var_gamma0_dn0)) * assign12260_e11373) + (assign12260_e11370 * (var_vbgs_dn0 - var_vfbsd_bg_dn0)))), ((var_vgs_noswap_dn1 - var_vfbsd_dn1) + ((((var_digs_i_dn1 * var_gamma0) + (var_digs_i * var_gamma0_dn1)) * assign12260_e11373) + (assign12260_e11370 * (var_vbgs_dn1 - var_vfbsd_bg_dn1)))), ((var_vgs_noswap_dn2 - var_vfbsd_dn2) + ((((var_digs_i_dn2 * var_gamma0) + (var_digs_i * var_gamma0_dn2)) * assign12260_e11373) + (assign12260_e11370 * (var_vbgs_dn2 - var_vfbsd_bg_dn2)))), ((var_vgs_noswap_dn3 - var_vfbsd_dn3) + ((((var_digs_i_dn3 * var_gamma0) + (var_digs_i * var_gamma0_dn3)) * assign12260_e11373) + (assign12260_e11370 * (var_vbgs_dn3 - var_vfbsd_bg_dn3)))), ((var_vgs_noswap_dn4 - var_vfbsd_dn4) + ((((var_digs_i_dn4 * var_gamma0) + (var_digs_i * var_gamma0_dn4)) * assign12260_e11373) + (assign12260_e11370 * (var_vbgs_dn4 - var_vfbsd_bg_dn4)))), ((var_vgs_noswap_dn5 - var_vfbsd_dn5) + ((((var_digs_i_dn5 * var_gamma0) + (var_digs_i * var_gamma0_dn5)) * assign12260_e11373) + (assign12260_e11370 * (var_vbgs_dn5 - var_vfbsd_bg_dn5)))), ((var_vgs_noswap_dn6 - var_vfbsd_dn6) + ((((var_digs_i_dn6 * var_gamma0) + (var_digs_i * var_gamma0_dn6)) * assign12260_e11373) + (assign12260_e11370 * (var_vbgs_dn6 - var_vfbsd_bg_dn6)))), ((var_vgs_noswap_dn7 - var_vfbsd_dn7) + ((((var_digs_i_dn7 * var_gamma0) + (var_digs_i * var_gamma0_dn7)) * assign12260_e11373) + (assign12260_e11370 * (var_vbgs_dn7 - var_vfbsd_bg_dn7)))), ((var_vgs_noswap_dn8 - var_vfbsd_dn8) + ((((var_digs_i_dn8 * var_gamma0) + (var_digs_i * var_gamma0_dn8)) * assign12260_e11373) + (assign12260_e11370 * (var_vbgs_dn8 - var_vfbsd_bg_dn8)))), ((var_vgs_noswap_db0 - var_vfbsd_db0) + ((((var_digs_i_db0 * var_gamma0) + (var_digs_i * var_gamma0_db0)) * assign12260_e11373) + (assign12260_e11370 * (var_vbgs_db0 - var_vfbsd_bg_db0)))), ((var_vgs_noswap_db1 - var_vfbsd_db1) + ((((var_digs_i_db1 * var_gamma0) + (var_digs_i * var_gamma0_db1)) * assign12260_e11373) + (assign12260_e11370 * (var_vbgs_db1 - var_vfbsd_bg_db1)))), ((var_vgs_noswap_db2 - var_vfbsd_db2) + ((((var_digs_i_db2 * var_gamma0) + (var_digs_i * var_gamma0_db2)) * assign12260_e11373) + (assign12260_e11370 * (var_vbgs_db2 - var_vfbsd_bg_db2)))), ((var_vgs_noswap_db3 - var_vfbsd_db3) + ((((var_digs_i_db3 * var_gamma0) + (var_digs_i * var_gamma0_db3)) * assign12260_e11373) + (assign12260_e11370 * (var_vbgs_db3 - var_vfbsd_bg_db3)))), ((var_vgs_noswap_db4 - var_vfbsd_db4) + ((((var_digs_i_db4 * var_gamma0) + (var_digs_i * var_gamma0_db4)) * assign12260_e11373) + (assign12260_e11370 * (var_vbgs_db4 - var_vfbsd_bg_db4)))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4,)
    }
};
        var_t0 = assign12260_e11377;
        var_t0_dn0 = assign12260_e11377_d_n0;
        var_t0_dn1 = assign12260_e11377_d_n1;
        var_t0_dn2 = assign12260_e11377_d_n2;
        var_t0_dn3 = assign12260_e11377_d_n3;
        var_t0_dn4 = assign12260_e11377_d_n4;
        var_t0_dn5 = assign12260_e11377_d_n5;
        var_t0_dn6 = assign12260_e11377_d_n6;
        var_t0_dn7 = assign12260_e11377_d_n7;
        var_t0_dn8 = assign12260_e11377_d_n8;
        var_t0_db0 = assign12260_e11377_d_b0;
        var_t0_db1 = assign12260_e11377_d_b1;
        var_t0_db2 = assign12260_e11377_d_b2;
        var_t0_db3 = assign12260_e11377_d_b3;
        var_t0_db4 = assign12260_e11377_d_b4;
        var_t0_rv = 0.0;
        var_t0_rdn0 = 0.0;
        var_t0_rdn1 = 0.0;
        var_t0_rdn2 = 0.0;
        var_t0_rdn3 = 0.0;
        var_t0_rdn4 = 0.0;
        var_t0_rdn5 = 0.0;
        var_t0_rdn6 = 0.0;
        var_t0_rdn7 = 0.0;
        var_t0_rdn8 = 0.0;
        var_t0_rdb0 = 0.0;
        var_t0_rdb1 = 0.0;
        var_t0_rdb2 = 0.0;
        var_t0_rdb3 = 0.0;
        var_t0_rdb4 = 0.0;

        let (assign12270_e11386, assign12270_e11386_d_n0, assign12270_e11386_d_n1, assign12270_e11386_d_n2, assign12270_e11386_d_n3, assign12270_e11386_d_n4, assign12270_e11386_d_n5, assign12270_e11386_d_n6, assign12270_e11386_d_n7, assign12270_e11386_d_n8, assign12270_e11386_d_b0, assign12270_e11386_d_b1, assign12270_e11386_d_b2, assign12270_e11386_d_b3, assign12270_e11386_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12270_e11381: f64 = (var_t0 * var_t0);
        let assign12270_e11383: f64 = (assign12270_e11381 + 0.0001);
        let assign12270_e11384: f64 = (assign12270_e11383).sqrt();
        (assign12270_e11384, (((var_t0_dn0 * var_t0) + (var_t0 * var_t0_dn0)) / (2.0 * assign12270_e11384)), (((var_t0_dn1 * var_t0) + (var_t0 * var_t0_dn1)) / (2.0 * assign12270_e11384)), (((var_t0_dn2 * var_t0) + (var_t0 * var_t0_dn2)) / (2.0 * assign12270_e11384)), (((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)) / (2.0 * assign12270_e11384)), (((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)) / (2.0 * assign12270_e11384)), (((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)) / (2.0 * assign12270_e11384)), (((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)) / (2.0 * assign12270_e11384)), (((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)) / (2.0 * assign12270_e11384)), (((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)) / (2.0 * assign12270_e11384)), (((var_t0_db0 * var_t0) + (var_t0 * var_t0_db0)) / (2.0 * assign12270_e11384)), (((var_t0_db1 * var_t0) + (var_t0 * var_t0_db1)) / (2.0 * assign12270_e11384)), (((var_t0_db2 * var_t0) + (var_t0 * var_t0_db2)) / (2.0 * assign12270_e11384)), (((var_t0_db3 * var_t0) + (var_t0 * var_t0_db3)) / (2.0 * assign12270_e11384)), (((var_t0_db4 * var_t0) + (var_t0 * var_t0_db4)) / (2.0 * assign12270_e11384)),)
    } else {
        (var_vfgs_eff, var_vfgs_eff_dn0, var_vfgs_eff_dn1, var_vfgs_eff_dn2, var_vfgs_eff_dn3, var_vfgs_eff_dn4, var_vfgs_eff_dn5, var_vfgs_eff_dn6, var_vfgs_eff_dn7, var_vfgs_eff_dn8, var_vfgs_eff_db0, var_vfgs_eff_db1, var_vfgs_eff_db2, var_vfgs_eff_db3, var_vfgs_eff_db4,)
    }
};
        var_vfgs_eff = assign12270_e11386;
        var_vfgs_eff_dn0 = assign12270_e11386_d_n0;
        var_vfgs_eff_dn1 = assign12270_e11386_d_n1;
        var_vfgs_eff_dn2 = assign12270_e11386_d_n2;
        var_vfgs_eff_dn3 = assign12270_e11386_d_n3;
        var_vfgs_eff_dn4 = assign12270_e11386_d_n4;
        var_vfgs_eff_dn5 = assign12270_e11386_d_n5;
        var_vfgs_eff_dn6 = assign12270_e11386_d_n6;
        var_vfgs_eff_dn7 = assign12270_e11386_d_n7;
        var_vfgs_eff_dn8 = assign12270_e11386_d_n8;
        var_vfgs_eff_db0 = assign12270_e11386_d_b0;
        var_vfgs_eff_db1 = assign12270_e11386_d_b1;
        var_vfgs_eff_db2 = assign12270_e11386_d_b2;
        var_vfgs_eff_db3 = assign12270_e11386_d_b3;
        var_vfgs_eff_db4 = assign12270_e11386_d_b4;
        var_vfgs_eff_rv = 0.0;
        var_vfgs_eff_rdn0 = 0.0;
        var_vfgs_eff_rdn1 = 0.0;
        var_vfgs_eff_rdn2 = 0.0;
        var_vfgs_eff_rdn3 = 0.0;
        var_vfgs_eff_rdn4 = 0.0;
        var_vfgs_eff_rdn5 = 0.0;
        var_vfgs_eff_rdn6 = 0.0;
        var_vfgs_eff_rdn7 = 0.0;
        var_vfgs_eff_rdn8 = 0.0;
        var_vfgs_eff_rdb0 = 0.0;
        var_vfgs_eff_rdb1 = 0.0;
        var_vfgs_eff_rdb2 = 0.0;
        var_vfgs_eff_rdb3 = 0.0;
        var_vfgs_eff_rdb4 = 0.0;

        let (assign12280_e11394, assign12280_e11394_d_n0, assign12280_e11394_d_n1, assign12280_e11394_d_n2, assign12280_e11394_d_n3, assign12280_e11394_d_n4, assign12280_e11394_d_n5, assign12280_e11394_d_n6, assign12280_e11394_d_n7, assign12280_e11394_d_n8, assign12280_e11394_d_b0, assign12280_e11394_d_b1, assign12280_e11394_d_b2, assign12280_e11394_d_b3, assign12280_e11394_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12280_e11391: f64 = (var_bigs_i * var_vfgs_eff);
        let assign12280_e11392: f64 = (var_aigs_i - assign12280_e11391);
        (assign12280_e11392, (var_aigs_i_dn0 - ((var_bigs_i_dn0 * var_vfgs_eff) + (var_bigs_i * var_vfgs_eff_dn0))), (var_aigs_i_dn1 - ((var_bigs_i_dn1 * var_vfgs_eff) + (var_bigs_i * var_vfgs_eff_dn1))), (var_aigs_i_dn2 - ((var_bigs_i_dn2 * var_vfgs_eff) + (var_bigs_i * var_vfgs_eff_dn2))), (var_aigs_i_dn3 - ((var_bigs_i_dn3 * var_vfgs_eff) + (var_bigs_i * var_vfgs_eff_dn3))), (var_aigs_i_dn4 - ((var_bigs_i_dn4 * var_vfgs_eff) + (var_bigs_i * var_vfgs_eff_dn4))), (var_aigs_i_dn5 - ((var_bigs_i_dn5 * var_vfgs_eff) + (var_bigs_i * var_vfgs_eff_dn5))), (var_aigs_i_dn6 - ((var_bigs_i_dn6 * var_vfgs_eff) + (var_bigs_i * var_vfgs_eff_dn6))), (var_aigs_i_dn7 - ((var_bigs_i_dn7 * var_vfgs_eff) + (var_bigs_i * var_vfgs_eff_dn7))), (var_aigs_i_dn8 - ((var_bigs_i_dn8 * var_vfgs_eff) + (var_bigs_i * var_vfgs_eff_dn8))), (var_aigs_i_db0 - ((var_bigs_i_db0 * var_vfgs_eff) + (var_bigs_i * var_vfgs_eff_db0))), (var_aigs_i_db1 - ((var_bigs_i_db1 * var_vfgs_eff) + (var_bigs_i * var_vfgs_eff_db1))), (var_aigs_i_db2 - ((var_bigs_i_db2 * var_vfgs_eff) + (var_bigs_i * var_vfgs_eff_db2))), (var_aigs_i_db3 - ((var_bigs_i_db3 * var_vfgs_eff) + (var_bigs_i * var_vfgs_eff_db3))), (var_aigs_i_db4 - ((var_bigs_i_db4 * var_vfgs_eff) + (var_bigs_i * var_vfgs_eff_db4))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4,)
    }
};
        var_t1 = assign12280_e11394;
        var_t1_dn0 = assign12280_e11394_d_n0;
        var_t1_dn1 = assign12280_e11394_d_n1;
        var_t1_dn2 = assign12280_e11394_d_n2;
        var_t1_dn3 = assign12280_e11394_d_n3;
        var_t1_dn4 = assign12280_e11394_d_n4;
        var_t1_dn5 = assign12280_e11394_d_n5;
        var_t1_dn6 = assign12280_e11394_d_n6;
        var_t1_dn7 = assign12280_e11394_d_n7;
        var_t1_dn8 = assign12280_e11394_d_n8;
        var_t1_db0 = assign12280_e11394_d_b0;
        var_t1_db1 = assign12280_e11394_d_b1;
        var_t1_db2 = assign12280_e11394_d_b2;
        var_t1_db3 = assign12280_e11394_d_b3;
        var_t1_db4 = assign12280_e11394_d_b4;
        var_t1_rv = 0.0;
        var_t1_rdn0 = 0.0;
        var_t1_rdn1 = 0.0;
        var_t1_rdn2 = 0.0;
        var_t1_rdn3 = 0.0;
        var_t1_rdn4 = 0.0;
        var_t1_rdn5 = 0.0;
        var_t1_rdn6 = 0.0;
        var_t1_rdn7 = 0.0;
        var_t1_rdn8 = 0.0;
        var_t1_rdb0 = 0.0;
        var_t1_rdb1 = 0.0;
        var_t1_rdb2 = 0.0;
        var_t1_rdb3 = 0.0;
        var_t1_rdb4 = 0.0;

        let (assign12290_e11402, assign12290_e11402_d_n0, assign12290_e11402_d_n1, assign12290_e11402_d_n2, assign12290_e11402_d_n3, assign12290_e11402_d_n4, assign12290_e11402_d_n5, assign12290_e11402_d_n6, assign12290_e11402_d_n7, assign12290_e11402_d_n8, assign12290_e11402_d_b0, assign12290_e11402_d_b1, assign12290_e11402_d_b2, assign12290_e11402_d_b3, assign12290_e11402_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12290_e11399: f64 = (var_cigs_i * var_vfgs_eff);
        let assign12290_e11400: f64 = (1.0 + assign12290_e11399);
        (assign12290_e11400, ((var_cigs_i_dn0 * var_vfgs_eff) + (var_cigs_i * var_vfgs_eff_dn0)), ((var_cigs_i_dn1 * var_vfgs_eff) + (var_cigs_i * var_vfgs_eff_dn1)), ((var_cigs_i_dn2 * var_vfgs_eff) + (var_cigs_i * var_vfgs_eff_dn2)), ((var_cigs_i_dn3 * var_vfgs_eff) + (var_cigs_i * var_vfgs_eff_dn3)), ((var_cigs_i_dn4 * var_vfgs_eff) + (var_cigs_i * var_vfgs_eff_dn4)), ((var_cigs_i_dn5 * var_vfgs_eff) + (var_cigs_i * var_vfgs_eff_dn5)), ((var_cigs_i_dn6 * var_vfgs_eff) + (var_cigs_i * var_vfgs_eff_dn6)), ((var_cigs_i_dn7 * var_vfgs_eff) + (var_cigs_i * var_vfgs_eff_dn7)), ((var_cigs_i_dn8 * var_vfgs_eff) + (var_cigs_i * var_vfgs_eff_dn8)), ((var_cigs_i_db0 * var_vfgs_eff) + (var_cigs_i * var_vfgs_eff_db0)), ((var_cigs_i_db1 * var_vfgs_eff) + (var_cigs_i * var_vfgs_eff_db1)), ((var_cigs_i_db2 * var_vfgs_eff) + (var_cigs_i * var_vfgs_eff_db2)), ((var_cigs_i_db3 * var_vfgs_eff) + (var_cigs_i * var_vfgs_eff_db3)), ((var_cigs_i_db4 * var_vfgs_eff) + (var_cigs_i * var_vfgs_eff_db4)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_db0, var_t2_db1, var_t2_db2, var_t2_db3, var_t2_db4,)
    }
};
        var_t2 = assign12290_e11402;
        var_t2_dn0 = assign12290_e11402_d_n0;
        var_t2_dn1 = assign12290_e11402_d_n1;
        var_t2_dn2 = assign12290_e11402_d_n2;
        var_t2_dn3 = assign12290_e11402_d_n3;
        var_t2_dn4 = assign12290_e11402_d_n4;
        var_t2_dn5 = assign12290_e11402_d_n5;
        var_t2_dn6 = assign12290_e11402_d_n6;
        var_t2_dn7 = assign12290_e11402_d_n7;
        var_t2_dn8 = assign12290_e11402_d_n8;
        var_t2_db0 = assign12290_e11402_d_b0;
        var_t2_db1 = assign12290_e11402_d_b1;
        var_t2_db2 = assign12290_e11402_d_b2;
        var_t2_db3 = assign12290_e11402_d_b3;
        var_t2_db4 = assign12290_e11402_d_b4;
        var_t2_rv = 0.0;
        var_t2_rdn0 = 0.0;
        var_t2_rdn1 = 0.0;
        var_t2_rdn2 = 0.0;
        var_t2_rdn3 = 0.0;
        var_t2_rdn4 = 0.0;
        var_t2_rdn5 = 0.0;
        var_t2_rdn6 = 0.0;
        var_t2_rdn7 = 0.0;
        var_t2_rdn8 = 0.0;
        var_t2_rdb0 = 0.0;
        var_t2_rdb1 = 0.0;
        var_t2_rdb2 = 0.0;
        var_t2_rdb3 = 0.0;
        var_t2_rdb4 = 0.0;

        let (assign12300_e11415, assign12300_e11415_d_n0, assign12300_e11415_d_n1, assign12300_e11415_d_n2, assign12300_e11415_d_n3, assign12300_e11415_d_n4, assign12300_e11415_d_n5, assign12300_e11415_d_n6, assign12300_e11415_d_n7, assign12300_e11415_d_n8, assign12300_e11415_d_b0, assign12300_e11415_d_b1, assign12300_e11415_d_b2, assign12300_e11415_d_b3, assign12300_e11415_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12300_e11405: f64 = (-var_bechvb);
        let assign12300_e11407: f64 = (assign12300_e11405 * p.p99);
        let assign12300_e11409: f64 = (assign12300_e11407 * var_poxedge_i);
        let assign12300_e11411: f64 = (assign12300_e11409 * var_t1);
        let assign12300_e11413: f64 = (assign12300_e11411 * var_t2);
        (assign12300_e11413, ((((((((-var_bechvb_dn0) * p.p99) * var_poxedge_i) + (assign12300_e11407 * var_poxedge_i_dn0)) * var_t1) + (assign12300_e11409 * var_t1_dn0)) * var_t2) + (assign12300_e11411 * var_t2_dn0)), ((((((((-var_bechvb_dn1) * p.p99) * var_poxedge_i) + (assign12300_e11407 * var_poxedge_i_dn1)) * var_t1) + (assign12300_e11409 * var_t1_dn1)) * var_t2) + (assign12300_e11411 * var_t2_dn1)), ((((((((-var_bechvb_dn2) * p.p99) * var_poxedge_i) + (assign12300_e11407 * var_poxedge_i_dn2)) * var_t1) + (assign12300_e11409 * var_t1_dn2)) * var_t2) + (assign12300_e11411 * var_t2_dn2)), ((((((((-var_bechvb_dn3) * p.p99) * var_poxedge_i) + (assign12300_e11407 * var_poxedge_i_dn3)) * var_t1) + (assign12300_e11409 * var_t1_dn3)) * var_t2) + (assign12300_e11411 * var_t2_dn3)), ((((((((-var_bechvb_dn4) * p.p99) * var_poxedge_i) + (assign12300_e11407 * var_poxedge_i_dn4)) * var_t1) + (assign12300_e11409 * var_t1_dn4)) * var_t2) + (assign12300_e11411 * var_t2_dn4)), ((((((((-var_bechvb_dn5) * p.p99) * var_poxedge_i) + (assign12300_e11407 * var_poxedge_i_dn5)) * var_t1) + (assign12300_e11409 * var_t1_dn5)) * var_t2) + (assign12300_e11411 * var_t2_dn5)), ((((((((-var_bechvb_dn6) * p.p99) * var_poxedge_i) + (assign12300_e11407 * var_poxedge_i_dn6)) * var_t1) + (assign12300_e11409 * var_t1_dn6)) * var_t2) + (assign12300_e11411 * var_t2_dn6)), ((((((((-var_bechvb_dn7) * p.p99) * var_poxedge_i) + (assign12300_e11407 * var_poxedge_i_dn7)) * var_t1) + (assign12300_e11409 * var_t1_dn7)) * var_t2) + (assign12300_e11411 * var_t2_dn7)), ((((((((-var_bechvb_dn8) * p.p99) * var_poxedge_i) + (assign12300_e11407 * var_poxedge_i_dn8)) * var_t1) + (assign12300_e11409 * var_t1_dn8)) * var_t2) + (assign12300_e11411 * var_t2_dn8)), ((((((((-var_bechvb_db0) * p.p99) * var_poxedge_i) + (assign12300_e11407 * var_poxedge_i_db0)) * var_t1) + (assign12300_e11409 * var_t1_db0)) * var_t2) + (assign12300_e11411 * var_t2_db0)), ((((((((-var_bechvb_db1) * p.p99) * var_poxedge_i) + (assign12300_e11407 * var_poxedge_i_db1)) * var_t1) + (assign12300_e11409 * var_t1_db1)) * var_t2) + (assign12300_e11411 * var_t2_db1)), ((((((((-var_bechvb_db2) * p.p99) * var_poxedge_i) + (assign12300_e11407 * var_poxedge_i_db2)) * var_t1) + (assign12300_e11409 * var_t1_db2)) * var_t2) + (assign12300_e11411 * var_t2_db2)), ((((((((-var_bechvb_db3) * p.p99) * var_poxedge_i) + (assign12300_e11407 * var_poxedge_i_db3)) * var_t1) + (assign12300_e11409 * var_t1_db3)) * var_t2) + (assign12300_e11411 * var_t2_db3)), ((((((((-var_bechvb_db4) * p.p99) * var_poxedge_i) + (assign12300_e11407 * var_poxedge_i_db4)) * var_t1) + (assign12300_e11409 * var_t1_db4)) * var_t2) + (assign12300_e11411 * var_t2_db4)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn1, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_db0, var_t3_db1, var_t3_db2, var_t3_db3, var_t3_db4,)
    }
};
        var_t3 = assign12300_e11415;
        var_t3_dn0 = assign12300_e11415_d_n0;
        var_t3_dn1 = assign12300_e11415_d_n1;
        var_t3_dn2 = assign12300_e11415_d_n2;
        var_t3_dn3 = assign12300_e11415_d_n3;
        var_t3_dn4 = assign12300_e11415_d_n4;
        var_t3_dn5 = assign12300_e11415_d_n5;
        var_t3_dn6 = assign12300_e11415_d_n6;
        var_t3_dn7 = assign12300_e11415_d_n7;
        var_t3_dn8 = assign12300_e11415_d_n8;
        var_t3_db0 = assign12300_e11415_d_b0;
        var_t3_db1 = assign12300_e11415_d_b1;
        var_t3_db2 = assign12300_e11415_d_b2;
        var_t3_db3 = assign12300_e11415_d_b3;
        var_t3_db4 = assign12300_e11415_d_b4;
        var_t3_rv = 0.0;
        var_t3_rdn0 = 0.0;
        var_t3_rdn1 = 0.0;
        var_t3_rdn2 = 0.0;
        var_t3_rdn3 = 0.0;
        var_t3_rdn4 = 0.0;
        var_t3_rdn5 = 0.0;
        var_t3_rdn6 = 0.0;
        var_t3_rdn7 = 0.0;
        var_t3_rdn8 = 0.0;
        var_t3_rdb0 = 0.0;
        var_t3_rdb1 = 0.0;
        var_t3_rdb2 = 0.0;
        var_t3_rdb3 = 0.0;
        var_t3_rdb4 = 0.0;

        let (assign12310_e11420, assign12310_e11420_d_n0, assign12310_e11420_d_n1, assign12310_e11420_d_n2, assign12310_e11420_d_n3, assign12310_e11420_d_n4, assign12310_e11420_d_n5, assign12310_e11420_d_n6, assign12310_e11420_d_n7, assign12310_e11420_d_n8, assign12310_e11420_d_b0, assign12310_e11420_d_b1, assign12310_e11420_d_b2, assign12310_e11420_d_b3, assign12310_e11420_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12310_e11418: f64 = { let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12310_e11418, ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn0), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn1), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn2), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn3), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn4), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn5), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn6), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn7), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn8), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db0), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db1), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db2), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db3), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db4),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn1, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_db0, var_t4_db1, var_t4_db2, var_t4_db3, var_t4_db4,)
    }
};
        var_t4 = assign12310_e11420;
        var_t4_dn0 = assign12310_e11420_d_n0;
        var_t4_dn1 = assign12310_e11420_d_n1;
        var_t4_dn2 = assign12310_e11420_d_n2;
        var_t4_dn3 = assign12310_e11420_d_n3;
        var_t4_dn4 = assign12310_e11420_d_n4;
        var_t4_dn5 = assign12310_e11420_d_n5;
        var_t4_dn6 = assign12310_e11420_d_n6;
        var_t4_dn7 = assign12310_e11420_d_n7;
        var_t4_dn8 = assign12310_e11420_d_n8;
        var_t4_db0 = assign12310_e11420_d_b0;
        var_t4_db1 = assign12310_e11420_d_b1;
        var_t4_db2 = assign12310_e11420_d_b2;
        var_t4_db3 = assign12310_e11420_d_b3;
        var_t4_db4 = assign12310_e11420_d_b4;
        var_t4_rv = 0.0;
        var_t4_rdn0 = 0.0;
        var_t4_rdn1 = 0.0;
        var_t4_rdn2 = 0.0;
        var_t4_rdn3 = 0.0;
        var_t4_rdn4 = 0.0;
        var_t4_rdn5 = 0.0;
        var_t4_rdn6 = 0.0;
        var_t4_rdn7 = 0.0;
        var_t4_rdn8 = 0.0;
        var_t4_rdb0 = 0.0;
        var_t4_rdb1 = 0.0;
        var_t4_rdb2 = 0.0;
        var_t4_rdb3 = 0.0;
        var_t4_rdb4 = 0.0;


        *var_t0_slot = var_t0;
        *var_t0_db0_slot = var_t0_db0;
        *var_t0_db1_slot = var_t0_db1;
        *var_t0_db2_slot = var_t0_db2;
        *var_t0_db3_slot = var_t0_db3;
        *var_t0_db4_slot = var_t0_db4;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rdb0_slot = var_t0_rdb0;
        *var_t0_rdb1_slot = var_t0_rdb1;
        *var_t0_rdb2_slot = var_t0_rdb2;
        *var_t0_rdb3_slot = var_t0_rdb3;
        *var_t0_rdb4_slot = var_t0_rdb4;
        *var_t0_rdn0_slot = var_t0_rdn0;
        *var_t0_rdn1_slot = var_t0_rdn1;
        *var_t0_rdn2_slot = var_t0_rdn2;
        *var_t0_rdn3_slot = var_t0_rdn3;
        *var_t0_rdn4_slot = var_t0_rdn4;
        *var_t0_rdn5_slot = var_t0_rdn5;
        *var_t0_rdn6_slot = var_t0_rdn6;
        *var_t0_rdn7_slot = var_t0_rdn7;
        *var_t0_rdn8_slot = var_t0_rdn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_db0_slot = var_t1_db0;
        *var_t1_db1_slot = var_t1_db1;
        *var_t1_db2_slot = var_t1_db2;
        *var_t1_db3_slot = var_t1_db3;
        *var_t1_db4_slot = var_t1_db4;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_exp_slot = var_t1_exp;
        *var_t1_exp_db0_slot = var_t1_exp_db0;
        *var_t1_exp_db1_slot = var_t1_exp_db1;
        *var_t1_exp_db2_slot = var_t1_exp_db2;
        *var_t1_exp_db3_slot = var_t1_exp_db3;
        *var_t1_exp_db4_slot = var_t1_exp_db4;
        *var_t1_exp_dn0_slot = var_t1_exp_dn0;
        *var_t1_exp_dn1_slot = var_t1_exp_dn1;
        *var_t1_exp_dn2_slot = var_t1_exp_dn2;
        *var_t1_exp_dn3_slot = var_t1_exp_dn3;
        *var_t1_exp_dn4_slot = var_t1_exp_dn4;
        *var_t1_exp_dn5_slot = var_t1_exp_dn5;
        *var_t1_exp_dn6_slot = var_t1_exp_dn6;
        *var_t1_exp_dn7_slot = var_t1_exp_dn7;
        *var_t1_exp_dn8_slot = var_t1_exp_dn8;
        *var_t1_exp_rdb0_slot = var_t1_exp_rdb0;
        *var_t1_exp_rdb1_slot = var_t1_exp_rdb1;
        *var_t1_exp_rdb2_slot = var_t1_exp_rdb2;
        *var_t1_exp_rdb3_slot = var_t1_exp_rdb3;
        *var_t1_exp_rdb4_slot = var_t1_exp_rdb4;
        *var_t1_exp_rdn0_slot = var_t1_exp_rdn0;
        *var_t1_exp_rdn1_slot = var_t1_exp_rdn1;
        *var_t1_exp_rdn2_slot = var_t1_exp_rdn2;
        *var_t1_exp_rdn3_slot = var_t1_exp_rdn3;
        *var_t1_exp_rdn4_slot = var_t1_exp_rdn4;
        *var_t1_exp_rdn5_slot = var_t1_exp_rdn5;
        *var_t1_exp_rdn6_slot = var_t1_exp_rdn6;
        *var_t1_exp_rdn7_slot = var_t1_exp_rdn7;
        *var_t1_exp_rdn8_slot = var_t1_exp_rdn8;
        *var_t1_exp_rv_slot = var_t1_exp_rv;
        *var_t1_rdb0_slot = var_t1_rdb0;
        *var_t1_rdb1_slot = var_t1_rdb1;
        *var_t1_rdb2_slot = var_t1_rdb2;
        *var_t1_rdb3_slot = var_t1_rdb3;
        *var_t1_rdb4_slot = var_t1_rdb4;
        *var_t1_rdn0_slot = var_t1_rdn0;
        *var_t1_rdn1_slot = var_t1_rdn1;
        *var_t1_rdn2_slot = var_t1_rdn2;
        *var_t1_rdn3_slot = var_t1_rdn3;
        *var_t1_rdn4_slot = var_t1_rdn4;
        *var_t1_rdn5_slot = var_t1_rdn5;
        *var_t1_rdn6_slot = var_t1_rdn6;
        *var_t1_rdn7_slot = var_t1_rdn7;
        *var_t1_rdn8_slot = var_t1_rdn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_db0_slot = var_t2_db0;
        *var_t2_db1_slot = var_t2_db1;
        *var_t2_db2_slot = var_t2_db2;
        *var_t2_db3_slot = var_t2_db3;
        *var_t2_db4_slot = var_t2_db4;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rdb0_slot = var_t2_rdb0;
        *var_t2_rdb1_slot = var_t2_rdb1;
        *var_t2_rdb2_slot = var_t2_rdb2;
        *var_t2_rdb3_slot = var_t2_rdb3;
        *var_t2_rdb4_slot = var_t2_rdb4;
        *var_t2_rdn0_slot = var_t2_rdn0;
        *var_t2_rdn1_slot = var_t2_rdn1;
        *var_t2_rdn2_slot = var_t2_rdn2;
        *var_t2_rdn3_slot = var_t2_rdn3;
        *var_t2_rdn4_slot = var_t2_rdn4;
        *var_t2_rdn5_slot = var_t2_rdn5;
        *var_t2_rdn6_slot = var_t2_rdn6;
        *var_t2_rdn7_slot = var_t2_rdn7;
        *var_t2_rdn8_slot = var_t2_rdn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_db0_slot = var_t3_db0;
        *var_t3_db1_slot = var_t3_db1;
        *var_t3_db2_slot = var_t3_db2;
        *var_t3_db3_slot = var_t3_db3;
        *var_t3_db4_slot = var_t3_db4;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn1_slot = var_t3_dn1;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rdb0_slot = var_t3_rdb0;
        *var_t3_rdb1_slot = var_t3_rdb1;
        *var_t3_rdb2_slot = var_t3_rdb2;
        *var_t3_rdb3_slot = var_t3_rdb3;
        *var_t3_rdb4_slot = var_t3_rdb4;
        *var_t3_rdn0_slot = var_t3_rdn0;
        *var_t3_rdn1_slot = var_t3_rdn1;
        *var_t3_rdn2_slot = var_t3_rdn2;
        *var_t3_rdn3_slot = var_t3_rdn3;
        *var_t3_rdn4_slot = var_t3_rdn4;
        *var_t3_rdn5_slot = var_t3_rdn5;
        *var_t3_rdn6_slot = var_t3_rdn6;
        *var_t3_rdn7_slot = var_t3_rdn7;
        *var_t3_rdn8_slot = var_t3_rdn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_db0_slot = var_t4_db0;
        *var_t4_db1_slot = var_t4_db1;
        *var_t4_db2_slot = var_t4_db2;
        *var_t4_db3_slot = var_t4_db3;
        *var_t4_db4_slot = var_t4_db4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn1_slot = var_t4_dn1;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_rdb0_slot = var_t4_rdb0;
        *var_t4_rdb1_slot = var_t4_rdb1;
        *var_t4_rdb2_slot = var_t4_rdb2;
        *var_t4_rdb3_slot = var_t4_rdb3;
        *var_t4_rdb4_slot = var_t4_rdb4;
        *var_t4_rdn0_slot = var_t4_rdn0;
        *var_t4_rdn1_slot = var_t4_rdn1;
        *var_t4_rdn2_slot = var_t4_rdn2;
        *var_t4_rdn3_slot = var_t4_rdn3;
        *var_t4_rdn4_slot = var_t4_rdn4;
        *var_t4_rdn5_slot = var_t4_rdn5;
        *var_t4_rdn6_slot = var_t4_rdn6;
        *var_t4_rdn7_slot = var_t4_rdn7;
        *var_t4_rdn8_slot = var_t4_rdn8;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_db0_slot = var_t5_db0;
        *var_t5_db1_slot = var_t5_db1;
        *var_t5_db2_slot = var_t5_db2;
        *var_t5_db3_slot = var_t5_db3;
        *var_t5_db4_slot = var_t5_db4;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn1_slot = var_t5_dn1;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_rdb0_slot = var_t5_rdb0;
        *var_t5_rdb1_slot = var_t5_rdb1;
        *var_t5_rdb2_slot = var_t5_rdb2;
        *var_t5_rdb3_slot = var_t5_rdb3;
        *var_t5_rdb4_slot = var_t5_rdb4;
        *var_t5_rdn0_slot = var_t5_rdn0;
        *var_t5_rdn1_slot = var_t5_rdn1;
        *var_t5_rdn2_slot = var_t5_rdn2;
        *var_t5_rdn3_slot = var_t5_rdn3;
        *var_t5_rdn4_slot = var_t5_rdn4;
        *var_t5_rdn5_slot = var_t5_rdn5;
        *var_t5_rdn6_slot = var_t5_rdn6;
        *var_t5_rdn7_slot = var_t5_rdn7;
        *var_t5_rdn8_slot = var_t5_rdn8;
        *var_t5_rv_slot = var_t5_rv;
        *var_vdseffx_slot = var_vdseffx;
        *var_vdseffx_db0_slot = var_vdseffx_db0;
        *var_vdseffx_db1_slot = var_vdseffx_db1;
        *var_vdseffx_db2_slot = var_vdseffx_db2;
        *var_vdseffx_db3_slot = var_vdseffx_db3;
        *var_vdseffx_db4_slot = var_vdseffx_db4;
        *var_vdseffx_dn0_slot = var_vdseffx_dn0;
        *var_vdseffx_dn1_slot = var_vdseffx_dn1;
        *var_vdseffx_dn2_slot = var_vdseffx_dn2;
        *var_vdseffx_dn3_slot = var_vdseffx_dn3;
        *var_vdseffx_dn4_slot = var_vdseffx_dn4;
        *var_vdseffx_dn5_slot = var_vdseffx_dn5;
        *var_vdseffx_dn6_slot = var_vdseffx_dn6;
        *var_vdseffx_dn7_slot = var_vdseffx_dn7;
        *var_vdseffx_dn8_slot = var_vdseffx_dn8;
        *var_vdseffx_rdb0_slot = var_vdseffx_rdb0;
        *var_vdseffx_rdb1_slot = var_vdseffx_rdb1;
        *var_vdseffx_rdb2_slot = var_vdseffx_rdb2;
        *var_vdseffx_rdb3_slot = var_vdseffx_rdb3;
        *var_vdseffx_rdb4_slot = var_vdseffx_rdb4;
        *var_vdseffx_rdn0_slot = var_vdseffx_rdn0;
        *var_vdseffx_rdn1_slot = var_vdseffx_rdn1;
        *var_vdseffx_rdn2_slot = var_vdseffx_rdn2;
        *var_vdseffx_rdn3_slot = var_vdseffx_rdn3;
        *var_vdseffx_rdn4_slot = var_vdseffx_rdn4;
        *var_vdseffx_rdn5_slot = var_vdseffx_rdn5;
        *var_vdseffx_rdn6_slot = var_vdseffx_rdn6;
        *var_vdseffx_rdn7_slot = var_vdseffx_rdn7;
        *var_vdseffx_rdn8_slot = var_vdseffx_rdn8;
        *var_vdseffx_rv_slot = var_vdseffx_rv;
        *var_vfgs_eff_slot = var_vfgs_eff;
        *var_vfgs_eff_db0_slot = var_vfgs_eff_db0;
        *var_vfgs_eff_db1_slot = var_vfgs_eff_db1;
        *var_vfgs_eff_db2_slot = var_vfgs_eff_db2;
        *var_vfgs_eff_db3_slot = var_vfgs_eff_db3;
        *var_vfgs_eff_db4_slot = var_vfgs_eff_db4;
        *var_vfgs_eff_dn0_slot = var_vfgs_eff_dn0;
        *var_vfgs_eff_dn1_slot = var_vfgs_eff_dn1;
        *var_vfgs_eff_dn2_slot = var_vfgs_eff_dn2;
        *var_vfgs_eff_dn3_slot = var_vfgs_eff_dn3;
        *var_vfgs_eff_dn4_slot = var_vfgs_eff_dn4;
        *var_vfgs_eff_dn5_slot = var_vfgs_eff_dn5;
        *var_vfgs_eff_dn6_slot = var_vfgs_eff_dn6;
        *var_vfgs_eff_dn7_slot = var_vfgs_eff_dn7;
        *var_vfgs_eff_dn8_slot = var_vfgs_eff_dn8;
        *var_vfgs_eff_rdb0_slot = var_vfgs_eff_rdb0;
        *var_vfgs_eff_rdb1_slot = var_vfgs_eff_rdb1;
        *var_vfgs_eff_rdb2_slot = var_vfgs_eff_rdb2;
        *var_vfgs_eff_rdb3_slot = var_vfgs_eff_rdb3;
        *var_vfgs_eff_rdb4_slot = var_vfgs_eff_rdb4;
        *var_vfgs_eff_rdn0_slot = var_vfgs_eff_rdn0;
        *var_vfgs_eff_rdn1_slot = var_vfgs_eff_rdn1;
        *var_vfgs_eff_rdn2_slot = var_vfgs_eff_rdn2;
        *var_vfgs_eff_rdn3_slot = var_vfgs_eff_rdn3;
        *var_vfgs_eff_rdn4_slot = var_vfgs_eff_rdn4;
        *var_vfgs_eff_rdn5_slot = var_vfgs_eff_rdn5;
        *var_vfgs_eff_rdn6_slot = var_vfgs_eff_rdn6;
        *var_vfgs_eff_rdn7_slot = var_vfgs_eff_rdn7;
        *var_vfgs_eff_rdn8_slot = var_vfgs_eff_rdn8;
        *var_vfgs_eff_rv_slot = var_vfgs_eff_rv;
    }

    pub(super) fn stamp_reactive_block_85(
        p: &Parameters,
        var_agidl_i: f64,
        var_aigd_i: f64,
        var_aigd_i_db0: f64,
        var_aigd_i_db1: f64,
        var_aigd_i_db2: f64,
        var_aigd_i_db3: f64,
        var_aigd_i_db4: f64,
        var_aigd_i_dn0: f64,
        var_aigd_i_dn1: f64,
        var_aigd_i_dn2: f64,
        var_aigd_i_dn3: f64,
        var_aigd_i_dn4: f64,
        var_aigd_i_dn5: f64,
        var_aigd_i_dn6: f64,
        var_aigd_i_dn7: f64,
        var_aigd_i_dn8: f64,
        var_bechvb: f64,
        var_bechvb_db0: f64,
        var_bechvb_db1: f64,
        var_bechvb_db2: f64,
        var_bechvb_db3: f64,
        var_bechvb_db4: f64,
        var_bechvb_dn0: f64,
        var_bechvb_dn1: f64,
        var_bechvb_dn2: f64,
        var_bechvb_dn3: f64,
        var_bechvb_dn4: f64,
        var_bechvb_dn5: f64,
        var_bechvb_dn6: f64,
        var_bechvb_dn7: f64,
        var_bechvb_dn8: f64,
        var_bgidl_t: f64,
        var_bigd_i: f64,
        var_bigd_i_db0: f64,
        var_bigd_i_db1: f64,
        var_bigd_i_db2: f64,
        var_bigd_i_db3: f64,
        var_bigd_i_db4: f64,
        var_bigd_i_dn0: f64,
        var_bigd_i_dn1: f64,
        var_bigd_i_dn2: f64,
        var_bigd_i_dn3: f64,
        var_bigd_i_dn4: f64,
        var_bigd_i_dn5: f64,
        var_bigd_i_dn6: f64,
        var_bigd_i_dn7: f64,
        var_bigd_i_dn8: f64,
        var_cigd_i: f64,
        var_cigd_i_db0: f64,
        var_cigd_i_db1: f64,
        var_cigd_i_db2: f64,
        var_cigd_i_db3: f64,
        var_cigd_i_db4: f64,
        var_cigd_i_dn0: f64,
        var_cigd_i_dn1: f64,
        var_cigd_i_dn2: f64,
        var_cigd_i_dn3: f64,
        var_cigd_i_dn4: f64,
        var_cigd_i_dn5: f64,
        var_cigd_i_dn6: f64,
        var_cigd_i_dn7: f64,
        var_cigd_i_dn8: f64,
        var_digd_i: f64,
        var_digd_i_db0: f64,
        var_digd_i_db1: f64,
        var_digd_i_db2: f64,
        var_digd_i_db3: f64,
        var_digd_i_db4: f64,
        var_digd_i_dn0: f64,
        var_digd_i_dn1: f64,
        var_digd_i_dn2: f64,
        var_digd_i_dn3: f64,
        var_digd_i_dn4: f64,
        var_digd_i_dn5: f64,
        var_digd_i_dn6: f64,
        var_digd_i_dn7: f64,
        var_digd_i_dn8: f64,
        var_egidl_i: f64,
        var_egidl_i_db0: f64,
        var_egidl_i_db1: f64,
        var_egidl_i_db2: f64,
        var_egidl_i_db3: f64,
        var_egidl_i_db4: f64,
        var_egidl_i_dn0: f64,
        var_egidl_i_dn1: f64,
        var_egidl_i_dn2: f64,
        var_egidl_i_dn3: f64,
        var_egidl_i_dn4: f64,
        var_egidl_i_dn5: f64,
        var_egidl_i_dn6: f64,
        var_egidl_i_dn7: f64,
        var_egidl_i_dn8: f64,
        var_epsratio: f64,
        var_epsratio_db0: f64,
        var_epsratio_db1: f64,
        var_epsratio_db2: f64,
        var_epsratio_db3: f64,
        var_epsratio_db4: f64,
        var_epsratio_dn0: f64,
        var_epsratio_dn1: f64,
        var_epsratio_dn2: f64,
        var_epsratio_dn3: f64,
        var_epsratio_dn4: f64,
        var_epsratio_dn5: f64,
        var_epsratio_dn6: f64,
        var_epsratio_dn7: f64,
        var_epsratio_dn8: f64,
        var_gamma0: f64,
        var_gamma0_db0: f64,
        var_gamma0_db1: f64,
        var_gamma0_db2: f64,
        var_gamma0_db3: f64,
        var_gamma0_db4: f64,
        var_gamma0_dn0: f64,
        var_gamma0_dn1: f64,
        var_gamma0_dn2: f64,
        var_gamma0_dn3: f64,
        var_gamma0_dn4: f64,
        var_gamma0_dn5: f64,
        var_gamma0_dn6: f64,
        var_gamma0_dn7: f64,
        var_gamma0_dn8: f64,
        var_guard123: f64,
        var_poxedge_i: f64,
        var_poxedge_i_db0: f64,
        var_poxedge_i_db1: f64,
        var_poxedge_i_db2: f64,
        var_poxedge_i_db3: f64,
        var_poxedge_i_db4: f64,
        var_poxedge_i_dn0: f64,
        var_poxedge_i_dn1: f64,
        var_poxedge_i_dn2: f64,
        var_poxedge_i_dn3: f64,
        var_poxedge_i_dn4: f64,
        var_poxedge_i_dn5: f64,
        var_poxedge_i_dn6: f64,
        var_poxedge_i_dn7: f64,
        var_poxedge_i_dn8: f64,
        var_vbegidl_i: f64,
        var_vbegidl_i_db0: f64,
        var_vbegidl_i_db1: f64,
        var_vbegidl_i_db2: f64,
        var_vbegidl_i_db3: f64,
        var_vbegidl_i_db4: f64,
        var_vbegidl_i_dn0: f64,
        var_vbegidl_i_dn1: f64,
        var_vbegidl_i_dn2: f64,
        var_vbegidl_i_dn3: f64,
        var_vbegidl_i_dn4: f64,
        var_vbegidl_i_dn5: f64,
        var_vbegidl_i_dn6: f64,
        var_vbegidl_i_dn7: f64,
        var_vbegidl_i_dn8: f64,
        var_vbgidl_i: f64,
        var_vbgidl_i_db0: f64,
        var_vbgidl_i_db1: f64,
        var_vbgidl_i_db2: f64,
        var_vbgidl_i_db3: f64,
        var_vbgidl_i_db4: f64,
        var_vbgidl_i_dn0: f64,
        var_vbgidl_i_dn1: f64,
        var_vbgidl_i_dn2: f64,
        var_vbgidl_i_dn3: f64,
        var_vbgidl_i_dn4: f64,
        var_vbgidl_i_dn5: f64,
        var_vbgidl_i_dn6: f64,
        var_vbgidl_i_dn7: f64,
        var_vbgidl_i_dn8: f64,
        var_vbgs: f64,
        var_vbgs_db0: f64,
        var_vbgs_db1: f64,
        var_vbgs_db2: f64,
        var_vbgs_db3: f64,
        var_vbgs_db4: f64,
        var_vbgs_dn0: f64,
        var_vbgs_dn1: f64,
        var_vbgs_dn2: f64,
        var_vbgs_dn3: f64,
        var_vbgs_dn4: f64,
        var_vbgs_dn5: f64,
        var_vbgs_dn6: f64,
        var_vbgs_dn7: f64,
        var_vbgs_dn8: f64,
        var_vfbsd: f64,
        var_vfbsd_bg: f64,
        var_vfbsd_bg_db0: f64,
        var_vfbsd_bg_db1: f64,
        var_vfbsd_bg_db2: f64,
        var_vfbsd_bg_db3: f64,
        var_vfbsd_bg_db4: f64,
        var_vfbsd_bg_dn0: f64,
        var_vfbsd_bg_dn1: f64,
        var_vfbsd_bg_dn2: f64,
        var_vfbsd_bg_dn3: f64,
        var_vfbsd_bg_dn4: f64,
        var_vfbsd_bg_dn5: f64,
        var_vfbsd_bg_dn6: f64,
        var_vfbsd_bg_dn7: f64,
        var_vfbsd_bg_dn8: f64,
        var_vfbsd_db0: f64,
        var_vfbsd_db1: f64,
        var_vfbsd_db2: f64,
        var_vfbsd_db3: f64,
        var_vfbsd_db4: f64,
        var_vfbsd_dn0: f64,
        var_vfbsd_dn1: f64,
        var_vfbsd_dn2: f64,
        var_vfbsd_dn3: f64,
        var_vfbsd_dn4: f64,
        var_vfbsd_dn5: f64,
        var_vfbsd_dn6: f64,
        var_vfbsd_dn7: f64,
        var_vfbsd_dn8: f64,
        var_vgd_noswap: f64,
        var_vgd_noswap_db0: f64,
        var_vgd_noswap_db1: f64,
        var_vgd_noswap_db2: f64,
        var_vgd_noswap_db3: f64,
        var_vgd_noswap_db4: f64,
        var_vgd_noswap_dn0: f64,
        var_vgd_noswap_dn1: f64,
        var_vgd_noswap_dn2: f64,
        var_vgd_noswap_dn3: f64,
        var_vgd_noswap_dn4: f64,
        var_vgd_noswap_dn5: f64,
        var_vgd_noswap_dn6: f64,
        var_vgd_noswap_dn7: f64,
        var_vgd_noswap_dn8: f64,
        var_guard126_slot: &mut f64,
        var_guard126_db0_slot: &mut f64,
        var_guard126_db1_slot: &mut f64,
        var_guard126_db2_slot: &mut f64,
        var_guard126_db3_slot: &mut f64,
        var_guard126_db4_slot: &mut f64,
        var_guard126_dn0_slot: &mut f64,
        var_guard126_dn1_slot: &mut f64,
        var_guard126_dn2_slot: &mut f64,
        var_guard126_dn3_slot: &mut f64,
        var_guard126_dn4_slot: &mut f64,
        var_guard126_dn5_slot: &mut f64,
        var_guard126_dn6_slot: &mut f64,
        var_guard126_dn7_slot: &mut f64,
        var_guard126_dn8_slot: &mut f64,
        var_guard126_rdb0_slot: &mut f64,
        var_guard126_rdb1_slot: &mut f64,
        var_guard126_rdb2_slot: &mut f64,
        var_guard126_rdb3_slot: &mut f64,
        var_guard126_rdb4_slot: &mut f64,
        var_guard126_rdn0_slot: &mut f64,
        var_guard126_rdn1_slot: &mut f64,
        var_guard126_rdn2_slot: &mut f64,
        var_guard126_rdn3_slot: &mut f64,
        var_guard126_rdn4_slot: &mut f64,
        var_guard126_rdn5_slot: &mut f64,
        var_guard126_rdn6_slot: &mut f64,
        var_guard126_rdn7_slot: &mut f64,
        var_guard126_rdn8_slot: &mut f64,
        var_guard126_rv_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard127_db0_slot: &mut f64,
        var_guard127_db1_slot: &mut f64,
        var_guard127_db2_slot: &mut f64,
        var_guard127_db3_slot: &mut f64,
        var_guard127_db4_slot: &mut f64,
        var_guard127_dn0_slot: &mut f64,
        var_guard127_dn1_slot: &mut f64,
        var_guard127_dn2_slot: &mut f64,
        var_guard127_dn3_slot: &mut f64,
        var_guard127_dn4_slot: &mut f64,
        var_guard127_dn5_slot: &mut f64,
        var_guard127_dn6_slot: &mut f64,
        var_guard127_dn7_slot: &mut f64,
        var_guard127_dn8_slot: &mut f64,
        var_guard127_rdb0_slot: &mut f64,
        var_guard127_rdb1_slot: &mut f64,
        var_guard127_rdb2_slot: &mut f64,
        var_guard127_rdb3_slot: &mut f64,
        var_guard127_rdb4_slot: &mut f64,
        var_guard127_rdn0_slot: &mut f64,
        var_guard127_rdn1_slot: &mut f64,
        var_guard127_rdn2_slot: &mut f64,
        var_guard127_rdn3_slot: &mut f64,
        var_guard127_rdn4_slot: &mut f64,
        var_guard127_rdn5_slot: &mut f64,
        var_guard127_rdn6_slot: &mut f64,
        var_guard127_rdn7_slot: &mut f64,
        var_guard127_rdn8_slot: &mut f64,
        var_guard127_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_db0_slot: &mut f64,
        var_t0_db1_slot: &mut f64,
        var_t0_db2_slot: &mut f64,
        var_t0_db3_slot: &mut f64,
        var_t0_db4_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rdb0_slot: &mut f64,
        var_t0_rdb1_slot: &mut f64,
        var_t0_rdb2_slot: &mut f64,
        var_t0_rdb3_slot: &mut f64,
        var_t0_rdb4_slot: &mut f64,
        var_t0_rdn0_slot: &mut f64,
        var_t0_rdn1_slot: &mut f64,
        var_t0_rdn2_slot: &mut f64,
        var_t0_rdn3_slot: &mut f64,
        var_t0_rdn4_slot: &mut f64,
        var_t0_rdn5_slot: &mut f64,
        var_t0_rdn6_slot: &mut f64,
        var_t0_rdn7_slot: &mut f64,
        var_t0_rdn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_db0_slot: &mut f64,
        var_t1_db1_slot: &mut f64,
        var_t1_db2_slot: &mut f64,
        var_t1_db3_slot: &mut f64,
        var_t1_db4_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rdb0_slot: &mut f64,
        var_t1_rdb1_slot: &mut f64,
        var_t1_rdb2_slot: &mut f64,
        var_t1_rdb3_slot: &mut f64,
        var_t1_rdb4_slot: &mut f64,
        var_t1_rdn0_slot: &mut f64,
        var_t1_rdn1_slot: &mut f64,
        var_t1_rdn2_slot: &mut f64,
        var_t1_rdn3_slot: &mut f64,
        var_t1_rdn4_slot: &mut f64,
        var_t1_rdn5_slot: &mut f64,
        var_t1_rdn6_slot: &mut f64,
        var_t1_rdn7_slot: &mut f64,
        var_t1_rdn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_db0_slot: &mut f64,
        var_t2_db1_slot: &mut f64,
        var_t2_db2_slot: &mut f64,
        var_t2_db3_slot: &mut f64,
        var_t2_db4_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rdb0_slot: &mut f64,
        var_t2_rdb1_slot: &mut f64,
        var_t2_rdb2_slot: &mut f64,
        var_t2_rdb3_slot: &mut f64,
        var_t2_rdb4_slot: &mut f64,
        var_t2_rdn0_slot: &mut f64,
        var_t2_rdn1_slot: &mut f64,
        var_t2_rdn2_slot: &mut f64,
        var_t2_rdn3_slot: &mut f64,
        var_t2_rdn4_slot: &mut f64,
        var_t2_rdn5_slot: &mut f64,
        var_t2_rdn6_slot: &mut f64,
        var_t2_rdn7_slot: &mut f64,
        var_t2_rdn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_db0_slot: &mut f64,
        var_t3_db1_slot: &mut f64,
        var_t3_db2_slot: &mut f64,
        var_t3_db3_slot: &mut f64,
        var_t3_db4_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn1_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rdb0_slot: &mut f64,
        var_t3_rdb1_slot: &mut f64,
        var_t3_rdb2_slot: &mut f64,
        var_t3_rdb3_slot: &mut f64,
        var_t3_rdb4_slot: &mut f64,
        var_t3_rdn0_slot: &mut f64,
        var_t3_rdn1_slot: &mut f64,
        var_t3_rdn2_slot: &mut f64,
        var_t3_rdn3_slot: &mut f64,
        var_t3_rdn4_slot: &mut f64,
        var_t3_rdn5_slot: &mut f64,
        var_t3_rdn6_slot: &mut f64,
        var_t3_rdn7_slot: &mut f64,
        var_t3_rdn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_db0_slot: &mut f64,
        var_t4_db1_slot: &mut f64,
        var_t4_db2_slot: &mut f64,
        var_t4_db3_slot: &mut f64,
        var_t4_db4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn1_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_rdb0_slot: &mut f64,
        var_t4_rdb1_slot: &mut f64,
        var_t4_rdb2_slot: &mut f64,
        var_t4_rdb3_slot: &mut f64,
        var_t4_rdb4_slot: &mut f64,
        var_t4_rdn0_slot: &mut f64,
        var_t4_rdn1_slot: &mut f64,
        var_t4_rdn2_slot: &mut f64,
        var_t4_rdn3_slot: &mut f64,
        var_t4_rdn4_slot: &mut f64,
        var_t4_rdn5_slot: &mut f64,
        var_t4_rdn6_slot: &mut f64,
        var_t4_rdn7_slot: &mut f64,
        var_t4_rdn8_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_db0_slot: &mut f64,
        var_t6_db1_slot: &mut f64,
        var_t6_db2_slot: &mut f64,
        var_t6_db3_slot: &mut f64,
        var_t6_db4_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn1_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_rdb0_slot: &mut f64,
        var_t6_rdb1_slot: &mut f64,
        var_t6_rdb2_slot: &mut f64,
        var_t6_rdb3_slot: &mut f64,
        var_t6_rdb4_slot: &mut f64,
        var_t6_rdn0_slot: &mut f64,
        var_t6_rdn1_slot: &mut f64,
        var_t6_rdn2_slot: &mut f64,
        var_t6_rdn3_slot: &mut f64,
        var_t6_rdn4_slot: &mut f64,
        var_t6_rdn5_slot: &mut f64,
        var_t6_rdn6_slot: &mut f64,
        var_t6_rdn7_slot: &mut f64,
        var_t6_rdn8_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_vfgd_eff_slot: &mut f64,
        var_vfgd_eff_db0_slot: &mut f64,
        var_vfgd_eff_db1_slot: &mut f64,
        var_vfgd_eff_db2_slot: &mut f64,
        var_vfgd_eff_db3_slot: &mut f64,
        var_vfgd_eff_db4_slot: &mut f64,
        var_vfgd_eff_dn0_slot: &mut f64,
        var_vfgd_eff_dn1_slot: &mut f64,
        var_vfgd_eff_dn2_slot: &mut f64,
        var_vfgd_eff_dn3_slot: &mut f64,
        var_vfgd_eff_dn4_slot: &mut f64,
        var_vfgd_eff_dn5_slot: &mut f64,
        var_vfgd_eff_dn6_slot: &mut f64,
        var_vfgd_eff_dn7_slot: &mut f64,
        var_vfgd_eff_dn8_slot: &mut f64,
        var_vfgd_eff_rdb0_slot: &mut f64,
        var_vfgd_eff_rdb1_slot: &mut f64,
        var_vfgd_eff_rdb2_slot: &mut f64,
        var_vfgd_eff_rdb3_slot: &mut f64,
        var_vfgd_eff_rdb4_slot: &mut f64,
        var_vfgd_eff_rdn0_slot: &mut f64,
        var_vfgd_eff_rdn1_slot: &mut f64,
        var_vfgd_eff_rdn2_slot: &mut f64,
        var_vfgd_eff_rdn3_slot: &mut f64,
        var_vfgd_eff_rdn4_slot: &mut f64,
        var_vfgd_eff_rdn5_slot: &mut f64,
        var_vfgd_eff_rdn6_slot: &mut f64,
        var_vfgd_eff_rdn7_slot: &mut f64,
        var_vfgd_eff_rdn8_slot: &mut f64,
        var_vfgd_eff_rv_slot: &mut f64,
    ) {
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard126_db0: f64 = *var_guard126_db0_slot;
        let mut var_guard126_db1: f64 = *var_guard126_db1_slot;
        let mut var_guard126_db2: f64 = *var_guard126_db2_slot;
        let mut var_guard126_db3: f64 = *var_guard126_db3_slot;
        let mut var_guard126_db4: f64 = *var_guard126_db4_slot;
        let mut var_guard126_dn0: f64 = *var_guard126_dn0_slot;
        let mut var_guard126_dn1: f64 = *var_guard126_dn1_slot;
        let mut var_guard126_dn2: f64 = *var_guard126_dn2_slot;
        let mut var_guard126_dn3: f64 = *var_guard126_dn3_slot;
        let mut var_guard126_dn4: f64 = *var_guard126_dn4_slot;
        let mut var_guard126_dn5: f64 = *var_guard126_dn5_slot;
        let mut var_guard126_dn6: f64 = *var_guard126_dn6_slot;
        let mut var_guard126_dn7: f64 = *var_guard126_dn7_slot;
        let mut var_guard126_dn8: f64 = *var_guard126_dn8_slot;
        let mut var_guard126_rdb0: f64 = *var_guard126_rdb0_slot;
        let mut var_guard126_rdb1: f64 = *var_guard126_rdb1_slot;
        let mut var_guard126_rdb2: f64 = *var_guard126_rdb2_slot;
        let mut var_guard126_rdb3: f64 = *var_guard126_rdb3_slot;
        let mut var_guard126_rdb4: f64 = *var_guard126_rdb4_slot;
        let mut var_guard126_rdn0: f64 = *var_guard126_rdn0_slot;
        let mut var_guard126_rdn1: f64 = *var_guard126_rdn1_slot;
        let mut var_guard126_rdn2: f64 = *var_guard126_rdn2_slot;
        let mut var_guard126_rdn3: f64 = *var_guard126_rdn3_slot;
        let mut var_guard126_rdn4: f64 = *var_guard126_rdn4_slot;
        let mut var_guard126_rdn5: f64 = *var_guard126_rdn5_slot;
        let mut var_guard126_rdn6: f64 = *var_guard126_rdn6_slot;
        let mut var_guard126_rdn7: f64 = *var_guard126_rdn7_slot;
        let mut var_guard126_rdn8: f64 = *var_guard126_rdn8_slot;
        let mut var_guard126_rv: f64 = *var_guard126_rv_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard127_db0: f64 = *var_guard127_db0_slot;
        let mut var_guard127_db1: f64 = *var_guard127_db1_slot;
        let mut var_guard127_db2: f64 = *var_guard127_db2_slot;
        let mut var_guard127_db3: f64 = *var_guard127_db3_slot;
        let mut var_guard127_db4: f64 = *var_guard127_db4_slot;
        let mut var_guard127_dn0: f64 = *var_guard127_dn0_slot;
        let mut var_guard127_dn1: f64 = *var_guard127_dn1_slot;
        let mut var_guard127_dn2: f64 = *var_guard127_dn2_slot;
        let mut var_guard127_dn3: f64 = *var_guard127_dn3_slot;
        let mut var_guard127_dn4: f64 = *var_guard127_dn4_slot;
        let mut var_guard127_dn5: f64 = *var_guard127_dn5_slot;
        let mut var_guard127_dn6: f64 = *var_guard127_dn6_slot;
        let mut var_guard127_dn7: f64 = *var_guard127_dn7_slot;
        let mut var_guard127_dn8: f64 = *var_guard127_dn8_slot;
        let mut var_guard127_rdb0: f64 = *var_guard127_rdb0_slot;
        let mut var_guard127_rdb1: f64 = *var_guard127_rdb1_slot;
        let mut var_guard127_rdb2: f64 = *var_guard127_rdb2_slot;
        let mut var_guard127_rdb3: f64 = *var_guard127_rdb3_slot;
        let mut var_guard127_rdb4: f64 = *var_guard127_rdb4_slot;
        let mut var_guard127_rdn0: f64 = *var_guard127_rdn0_slot;
        let mut var_guard127_rdn1: f64 = *var_guard127_rdn1_slot;
        let mut var_guard127_rdn2: f64 = *var_guard127_rdn2_slot;
        let mut var_guard127_rdn3: f64 = *var_guard127_rdn3_slot;
        let mut var_guard127_rdn4: f64 = *var_guard127_rdn4_slot;
        let mut var_guard127_rdn5: f64 = *var_guard127_rdn5_slot;
        let mut var_guard127_rdn6: f64 = *var_guard127_rdn6_slot;
        let mut var_guard127_rdn7: f64 = *var_guard127_rdn7_slot;
        let mut var_guard127_rdn8: f64 = *var_guard127_rdn8_slot;
        let mut var_guard127_rv: f64 = *var_guard127_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_db0: f64 = *var_t0_db0_slot;
        let mut var_t0_db1: f64 = *var_t0_db1_slot;
        let mut var_t0_db2: f64 = *var_t0_db2_slot;
        let mut var_t0_db3: f64 = *var_t0_db3_slot;
        let mut var_t0_db4: f64 = *var_t0_db4_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rdb0: f64 = *var_t0_rdb0_slot;
        let mut var_t0_rdb1: f64 = *var_t0_rdb1_slot;
        let mut var_t0_rdb2: f64 = *var_t0_rdb2_slot;
        let mut var_t0_rdb3: f64 = *var_t0_rdb3_slot;
        let mut var_t0_rdb4: f64 = *var_t0_rdb4_slot;
        let mut var_t0_rdn0: f64 = *var_t0_rdn0_slot;
        let mut var_t0_rdn1: f64 = *var_t0_rdn1_slot;
        let mut var_t0_rdn2: f64 = *var_t0_rdn2_slot;
        let mut var_t0_rdn3: f64 = *var_t0_rdn3_slot;
        let mut var_t0_rdn4: f64 = *var_t0_rdn4_slot;
        let mut var_t0_rdn5: f64 = *var_t0_rdn5_slot;
        let mut var_t0_rdn6: f64 = *var_t0_rdn6_slot;
        let mut var_t0_rdn7: f64 = *var_t0_rdn7_slot;
        let mut var_t0_rdn8: f64 = *var_t0_rdn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_db0: f64 = *var_t1_db0_slot;
        let mut var_t1_db1: f64 = *var_t1_db1_slot;
        let mut var_t1_db2: f64 = *var_t1_db2_slot;
        let mut var_t1_db3: f64 = *var_t1_db3_slot;
        let mut var_t1_db4: f64 = *var_t1_db4_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rdb0: f64 = *var_t1_rdb0_slot;
        let mut var_t1_rdb1: f64 = *var_t1_rdb1_slot;
        let mut var_t1_rdb2: f64 = *var_t1_rdb2_slot;
        let mut var_t1_rdb3: f64 = *var_t1_rdb3_slot;
        let mut var_t1_rdb4: f64 = *var_t1_rdb4_slot;
        let mut var_t1_rdn0: f64 = *var_t1_rdn0_slot;
        let mut var_t1_rdn1: f64 = *var_t1_rdn1_slot;
        let mut var_t1_rdn2: f64 = *var_t1_rdn2_slot;
        let mut var_t1_rdn3: f64 = *var_t1_rdn3_slot;
        let mut var_t1_rdn4: f64 = *var_t1_rdn4_slot;
        let mut var_t1_rdn5: f64 = *var_t1_rdn5_slot;
        let mut var_t1_rdn6: f64 = *var_t1_rdn6_slot;
        let mut var_t1_rdn7: f64 = *var_t1_rdn7_slot;
        let mut var_t1_rdn8: f64 = *var_t1_rdn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_db0: f64 = *var_t2_db0_slot;
        let mut var_t2_db1: f64 = *var_t2_db1_slot;
        let mut var_t2_db2: f64 = *var_t2_db2_slot;
        let mut var_t2_db3: f64 = *var_t2_db3_slot;
        let mut var_t2_db4: f64 = *var_t2_db4_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rdb0: f64 = *var_t2_rdb0_slot;
        let mut var_t2_rdb1: f64 = *var_t2_rdb1_slot;
        let mut var_t2_rdb2: f64 = *var_t2_rdb2_slot;
        let mut var_t2_rdb3: f64 = *var_t2_rdb3_slot;
        let mut var_t2_rdb4: f64 = *var_t2_rdb4_slot;
        let mut var_t2_rdn0: f64 = *var_t2_rdn0_slot;
        let mut var_t2_rdn1: f64 = *var_t2_rdn1_slot;
        let mut var_t2_rdn2: f64 = *var_t2_rdn2_slot;
        let mut var_t2_rdn3: f64 = *var_t2_rdn3_slot;
        let mut var_t2_rdn4: f64 = *var_t2_rdn4_slot;
        let mut var_t2_rdn5: f64 = *var_t2_rdn5_slot;
        let mut var_t2_rdn6: f64 = *var_t2_rdn6_slot;
        let mut var_t2_rdn7: f64 = *var_t2_rdn7_slot;
        let mut var_t2_rdn8: f64 = *var_t2_rdn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_db0: f64 = *var_t3_db0_slot;
        let mut var_t3_db1: f64 = *var_t3_db1_slot;
        let mut var_t3_db2: f64 = *var_t3_db2_slot;
        let mut var_t3_db3: f64 = *var_t3_db3_slot;
        let mut var_t3_db4: f64 = *var_t3_db4_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn1: f64 = *var_t3_dn1_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rdb0: f64 = *var_t3_rdb0_slot;
        let mut var_t3_rdb1: f64 = *var_t3_rdb1_slot;
        let mut var_t3_rdb2: f64 = *var_t3_rdb2_slot;
        let mut var_t3_rdb3: f64 = *var_t3_rdb3_slot;
        let mut var_t3_rdb4: f64 = *var_t3_rdb4_slot;
        let mut var_t3_rdn0: f64 = *var_t3_rdn0_slot;
        let mut var_t3_rdn1: f64 = *var_t3_rdn1_slot;
        let mut var_t3_rdn2: f64 = *var_t3_rdn2_slot;
        let mut var_t3_rdn3: f64 = *var_t3_rdn3_slot;
        let mut var_t3_rdn4: f64 = *var_t3_rdn4_slot;
        let mut var_t3_rdn5: f64 = *var_t3_rdn5_slot;
        let mut var_t3_rdn6: f64 = *var_t3_rdn6_slot;
        let mut var_t3_rdn7: f64 = *var_t3_rdn7_slot;
        let mut var_t3_rdn8: f64 = *var_t3_rdn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_db0: f64 = *var_t4_db0_slot;
        let mut var_t4_db1: f64 = *var_t4_db1_slot;
        let mut var_t4_db2: f64 = *var_t4_db2_slot;
        let mut var_t4_db3: f64 = *var_t4_db3_slot;
        let mut var_t4_db4: f64 = *var_t4_db4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn1: f64 = *var_t4_dn1_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_rdb0: f64 = *var_t4_rdb0_slot;
        let mut var_t4_rdb1: f64 = *var_t4_rdb1_slot;
        let mut var_t4_rdb2: f64 = *var_t4_rdb2_slot;
        let mut var_t4_rdb3: f64 = *var_t4_rdb3_slot;
        let mut var_t4_rdb4: f64 = *var_t4_rdb4_slot;
        let mut var_t4_rdn0: f64 = *var_t4_rdn0_slot;
        let mut var_t4_rdn1: f64 = *var_t4_rdn1_slot;
        let mut var_t4_rdn2: f64 = *var_t4_rdn2_slot;
        let mut var_t4_rdn3: f64 = *var_t4_rdn3_slot;
        let mut var_t4_rdn4: f64 = *var_t4_rdn4_slot;
        let mut var_t4_rdn5: f64 = *var_t4_rdn5_slot;
        let mut var_t4_rdn6: f64 = *var_t4_rdn6_slot;
        let mut var_t4_rdn7: f64 = *var_t4_rdn7_slot;
        let mut var_t4_rdn8: f64 = *var_t4_rdn8_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_db0: f64 = *var_t6_db0_slot;
        let mut var_t6_db1: f64 = *var_t6_db1_slot;
        let mut var_t6_db2: f64 = *var_t6_db2_slot;
        let mut var_t6_db3: f64 = *var_t6_db3_slot;
        let mut var_t6_db4: f64 = *var_t6_db4_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn1: f64 = *var_t6_dn1_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_rdb0: f64 = *var_t6_rdb0_slot;
        let mut var_t6_rdb1: f64 = *var_t6_rdb1_slot;
        let mut var_t6_rdb2: f64 = *var_t6_rdb2_slot;
        let mut var_t6_rdb3: f64 = *var_t6_rdb3_slot;
        let mut var_t6_rdb4: f64 = *var_t6_rdb4_slot;
        let mut var_t6_rdn0: f64 = *var_t6_rdn0_slot;
        let mut var_t6_rdn1: f64 = *var_t6_rdn1_slot;
        let mut var_t6_rdn2: f64 = *var_t6_rdn2_slot;
        let mut var_t6_rdn3: f64 = *var_t6_rdn3_slot;
        let mut var_t6_rdn4: f64 = *var_t6_rdn4_slot;
        let mut var_t6_rdn5: f64 = *var_t6_rdn5_slot;
        let mut var_t6_rdn6: f64 = *var_t6_rdn6_slot;
        let mut var_t6_rdn7: f64 = *var_t6_rdn7_slot;
        let mut var_t6_rdn8: f64 = *var_t6_rdn8_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_vfgd_eff: f64 = *var_vfgd_eff_slot;
        let mut var_vfgd_eff_db0: f64 = *var_vfgd_eff_db0_slot;
        let mut var_vfgd_eff_db1: f64 = *var_vfgd_eff_db1_slot;
        let mut var_vfgd_eff_db2: f64 = *var_vfgd_eff_db2_slot;
        let mut var_vfgd_eff_db3: f64 = *var_vfgd_eff_db3_slot;
        let mut var_vfgd_eff_db4: f64 = *var_vfgd_eff_db4_slot;
        let mut var_vfgd_eff_dn0: f64 = *var_vfgd_eff_dn0_slot;
        let mut var_vfgd_eff_dn1: f64 = *var_vfgd_eff_dn1_slot;
        let mut var_vfgd_eff_dn2: f64 = *var_vfgd_eff_dn2_slot;
        let mut var_vfgd_eff_dn3: f64 = *var_vfgd_eff_dn3_slot;
        let mut var_vfgd_eff_dn4: f64 = *var_vfgd_eff_dn4_slot;
        let mut var_vfgd_eff_dn5: f64 = *var_vfgd_eff_dn5_slot;
        let mut var_vfgd_eff_dn6: f64 = *var_vfgd_eff_dn6_slot;
        let mut var_vfgd_eff_dn7: f64 = *var_vfgd_eff_dn7_slot;
        let mut var_vfgd_eff_dn8: f64 = *var_vfgd_eff_dn8_slot;
        let mut var_vfgd_eff_rdb0: f64 = *var_vfgd_eff_rdb0_slot;
        let mut var_vfgd_eff_rdb1: f64 = *var_vfgd_eff_rdb1_slot;
        let mut var_vfgd_eff_rdb2: f64 = *var_vfgd_eff_rdb2_slot;
        let mut var_vfgd_eff_rdb3: f64 = *var_vfgd_eff_rdb3_slot;
        let mut var_vfgd_eff_rdb4: f64 = *var_vfgd_eff_rdb4_slot;
        let mut var_vfgd_eff_rdn0: f64 = *var_vfgd_eff_rdn0_slot;
        let mut var_vfgd_eff_rdn1: f64 = *var_vfgd_eff_rdn1_slot;
        let mut var_vfgd_eff_rdn2: f64 = *var_vfgd_eff_rdn2_slot;
        let mut var_vfgd_eff_rdn3: f64 = *var_vfgd_eff_rdn3_slot;
        let mut var_vfgd_eff_rdn4: f64 = *var_vfgd_eff_rdn4_slot;
        let mut var_vfgd_eff_rdn5: f64 = *var_vfgd_eff_rdn5_slot;
        let mut var_vfgd_eff_rdn6: f64 = *var_vfgd_eff_rdn6_slot;
        let mut var_vfgd_eff_rdn7: f64 = *var_vfgd_eff_rdn7_slot;
        let mut var_vfgd_eff_rdn8: f64 = *var_vfgd_eff_rdn8_slot;
        let mut var_vfgd_eff_rv: f64 = *var_vfgd_eff_rv_slot;

        let (assign12350_e11466, assign12350_e11466_d_n0, assign12350_e11466_d_n1, assign12350_e11466_d_n2, assign12350_e11466_d_n3, assign12350_e11466_d_n4, assign12350_e11466_d_n5, assign12350_e11466_d_n6, assign12350_e11466_d_n7, assign12350_e11466_d_n8, assign12350_e11466_d_b0, assign12350_e11466_d_b1, assign12350_e11466_d_b2, assign12350_e11466_d_b3, assign12350_e11466_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12350_e11456: f64 = (var_vgd_noswap - var_vfbsd);
        let assign12350_e11459: f64 = (var_digd_i * var_gamma0);
        let assign12350_e11462: f64 = (var_vbgs - var_vfbsd_bg);
        let assign12350_e11463: f64 = (assign12350_e11459 * assign12350_e11462);
        let assign12350_e11464: f64 = (assign12350_e11456 + assign12350_e11463);
        (assign12350_e11464, ((var_vgd_noswap_dn0 - var_vfbsd_dn0) + ((((var_digd_i_dn0 * var_gamma0) + (var_digd_i * var_gamma0_dn0)) * assign12350_e11462) + (assign12350_e11459 * (var_vbgs_dn0 - var_vfbsd_bg_dn0)))), ((var_vgd_noswap_dn1 - var_vfbsd_dn1) + ((((var_digd_i_dn1 * var_gamma0) + (var_digd_i * var_gamma0_dn1)) * assign12350_e11462) + (assign12350_e11459 * (var_vbgs_dn1 - var_vfbsd_bg_dn1)))), ((var_vgd_noswap_dn2 - var_vfbsd_dn2) + ((((var_digd_i_dn2 * var_gamma0) + (var_digd_i * var_gamma0_dn2)) * assign12350_e11462) + (assign12350_e11459 * (var_vbgs_dn2 - var_vfbsd_bg_dn2)))), ((var_vgd_noswap_dn3 - var_vfbsd_dn3) + ((((var_digd_i_dn3 * var_gamma0) + (var_digd_i * var_gamma0_dn3)) * assign12350_e11462) + (assign12350_e11459 * (var_vbgs_dn3 - var_vfbsd_bg_dn3)))), ((var_vgd_noswap_dn4 - var_vfbsd_dn4) + ((((var_digd_i_dn4 * var_gamma0) + (var_digd_i * var_gamma0_dn4)) * assign12350_e11462) + (assign12350_e11459 * (var_vbgs_dn4 - var_vfbsd_bg_dn4)))), ((var_vgd_noswap_dn5 - var_vfbsd_dn5) + ((((var_digd_i_dn5 * var_gamma0) + (var_digd_i * var_gamma0_dn5)) * assign12350_e11462) + (assign12350_e11459 * (var_vbgs_dn5 - var_vfbsd_bg_dn5)))), ((var_vgd_noswap_dn6 - var_vfbsd_dn6) + ((((var_digd_i_dn6 * var_gamma0) + (var_digd_i * var_gamma0_dn6)) * assign12350_e11462) + (assign12350_e11459 * (var_vbgs_dn6 - var_vfbsd_bg_dn6)))), ((var_vgd_noswap_dn7 - var_vfbsd_dn7) + ((((var_digd_i_dn7 * var_gamma0) + (var_digd_i * var_gamma0_dn7)) * assign12350_e11462) + (assign12350_e11459 * (var_vbgs_dn7 - var_vfbsd_bg_dn7)))), ((var_vgd_noswap_dn8 - var_vfbsd_dn8) + ((((var_digd_i_dn8 * var_gamma0) + (var_digd_i * var_gamma0_dn8)) * assign12350_e11462) + (assign12350_e11459 * (var_vbgs_dn8 - var_vfbsd_bg_dn8)))), ((var_vgd_noswap_db0 - var_vfbsd_db0) + ((((var_digd_i_db0 * var_gamma0) + (var_digd_i * var_gamma0_db0)) * assign12350_e11462) + (assign12350_e11459 * (var_vbgs_db0 - var_vfbsd_bg_db0)))), ((var_vgd_noswap_db1 - var_vfbsd_db1) + ((((var_digd_i_db1 * var_gamma0) + (var_digd_i * var_gamma0_db1)) * assign12350_e11462) + (assign12350_e11459 * (var_vbgs_db1 - var_vfbsd_bg_db1)))), ((var_vgd_noswap_db2 - var_vfbsd_db2) + ((((var_digd_i_db2 * var_gamma0) + (var_digd_i * var_gamma0_db2)) * assign12350_e11462) + (assign12350_e11459 * (var_vbgs_db2 - var_vfbsd_bg_db2)))), ((var_vgd_noswap_db3 - var_vfbsd_db3) + ((((var_digd_i_db3 * var_gamma0) + (var_digd_i * var_gamma0_db3)) * assign12350_e11462) + (assign12350_e11459 * (var_vbgs_db3 - var_vfbsd_bg_db3)))), ((var_vgd_noswap_db4 - var_vfbsd_db4) + ((((var_digd_i_db4 * var_gamma0) + (var_digd_i * var_gamma0_db4)) * assign12350_e11462) + (assign12350_e11459 * (var_vbgs_db4 - var_vfbsd_bg_db4)))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4,)
    }
};
        var_t0 = assign12350_e11466;
        var_t0_dn0 = assign12350_e11466_d_n0;
        var_t0_dn1 = assign12350_e11466_d_n1;
        var_t0_dn2 = assign12350_e11466_d_n2;
        var_t0_dn3 = assign12350_e11466_d_n3;
        var_t0_dn4 = assign12350_e11466_d_n4;
        var_t0_dn5 = assign12350_e11466_d_n5;
        var_t0_dn6 = assign12350_e11466_d_n6;
        var_t0_dn7 = assign12350_e11466_d_n7;
        var_t0_dn8 = assign12350_e11466_d_n8;
        var_t0_db0 = assign12350_e11466_d_b0;
        var_t0_db1 = assign12350_e11466_d_b1;
        var_t0_db2 = assign12350_e11466_d_b2;
        var_t0_db3 = assign12350_e11466_d_b3;
        var_t0_db4 = assign12350_e11466_d_b4;
        var_t0_rv = 0.0;
        var_t0_rdn0 = 0.0;
        var_t0_rdn1 = 0.0;
        var_t0_rdn2 = 0.0;
        var_t0_rdn3 = 0.0;
        var_t0_rdn4 = 0.0;
        var_t0_rdn5 = 0.0;
        var_t0_rdn6 = 0.0;
        var_t0_rdn7 = 0.0;
        var_t0_rdn8 = 0.0;
        var_t0_rdb0 = 0.0;
        var_t0_rdb1 = 0.0;
        var_t0_rdb2 = 0.0;
        var_t0_rdb3 = 0.0;
        var_t0_rdb4 = 0.0;

        let (assign12360_e11475, assign12360_e11475_d_n0, assign12360_e11475_d_n1, assign12360_e11475_d_n2, assign12360_e11475_d_n3, assign12360_e11475_d_n4, assign12360_e11475_d_n5, assign12360_e11475_d_n6, assign12360_e11475_d_n7, assign12360_e11475_d_n8, assign12360_e11475_d_b0, assign12360_e11475_d_b1, assign12360_e11475_d_b2, assign12360_e11475_d_b3, assign12360_e11475_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12360_e11470: f64 = (var_t0 * var_t0);
        let assign12360_e11472: f64 = (assign12360_e11470 + 0.0001);
        let assign12360_e11473: f64 = (assign12360_e11472).sqrt();
        (assign12360_e11473, (((var_t0_dn0 * var_t0) + (var_t0 * var_t0_dn0)) / (2.0 * assign12360_e11473)), (((var_t0_dn1 * var_t0) + (var_t0 * var_t0_dn1)) / (2.0 * assign12360_e11473)), (((var_t0_dn2 * var_t0) + (var_t0 * var_t0_dn2)) / (2.0 * assign12360_e11473)), (((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)) / (2.0 * assign12360_e11473)), (((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)) / (2.0 * assign12360_e11473)), (((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)) / (2.0 * assign12360_e11473)), (((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)) / (2.0 * assign12360_e11473)), (((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)) / (2.0 * assign12360_e11473)), (((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)) / (2.0 * assign12360_e11473)), (((var_t0_db0 * var_t0) + (var_t0 * var_t0_db0)) / (2.0 * assign12360_e11473)), (((var_t0_db1 * var_t0) + (var_t0 * var_t0_db1)) / (2.0 * assign12360_e11473)), (((var_t0_db2 * var_t0) + (var_t0 * var_t0_db2)) / (2.0 * assign12360_e11473)), (((var_t0_db3 * var_t0) + (var_t0 * var_t0_db3)) / (2.0 * assign12360_e11473)), (((var_t0_db4 * var_t0) + (var_t0 * var_t0_db4)) / (2.0 * assign12360_e11473)),)
    } else {
        (var_vfgd_eff, var_vfgd_eff_dn0, var_vfgd_eff_dn1, var_vfgd_eff_dn2, var_vfgd_eff_dn3, var_vfgd_eff_dn4, var_vfgd_eff_dn5, var_vfgd_eff_dn6, var_vfgd_eff_dn7, var_vfgd_eff_dn8, var_vfgd_eff_db0, var_vfgd_eff_db1, var_vfgd_eff_db2, var_vfgd_eff_db3, var_vfgd_eff_db4,)
    }
};
        var_vfgd_eff = assign12360_e11475;
        var_vfgd_eff_dn0 = assign12360_e11475_d_n0;
        var_vfgd_eff_dn1 = assign12360_e11475_d_n1;
        var_vfgd_eff_dn2 = assign12360_e11475_d_n2;
        var_vfgd_eff_dn3 = assign12360_e11475_d_n3;
        var_vfgd_eff_dn4 = assign12360_e11475_d_n4;
        var_vfgd_eff_dn5 = assign12360_e11475_d_n5;
        var_vfgd_eff_dn6 = assign12360_e11475_d_n6;
        var_vfgd_eff_dn7 = assign12360_e11475_d_n7;
        var_vfgd_eff_dn8 = assign12360_e11475_d_n8;
        var_vfgd_eff_db0 = assign12360_e11475_d_b0;
        var_vfgd_eff_db1 = assign12360_e11475_d_b1;
        var_vfgd_eff_db2 = assign12360_e11475_d_b2;
        var_vfgd_eff_db3 = assign12360_e11475_d_b3;
        var_vfgd_eff_db4 = assign12360_e11475_d_b4;
        var_vfgd_eff_rv = 0.0;
        var_vfgd_eff_rdn0 = 0.0;
        var_vfgd_eff_rdn1 = 0.0;
        var_vfgd_eff_rdn2 = 0.0;
        var_vfgd_eff_rdn3 = 0.0;
        var_vfgd_eff_rdn4 = 0.0;
        var_vfgd_eff_rdn5 = 0.0;
        var_vfgd_eff_rdn6 = 0.0;
        var_vfgd_eff_rdn7 = 0.0;
        var_vfgd_eff_rdn8 = 0.0;
        var_vfgd_eff_rdb0 = 0.0;
        var_vfgd_eff_rdb1 = 0.0;
        var_vfgd_eff_rdb2 = 0.0;
        var_vfgd_eff_rdb3 = 0.0;
        var_vfgd_eff_rdb4 = 0.0;

        let (assign12370_e11483, assign12370_e11483_d_n0, assign12370_e11483_d_n1, assign12370_e11483_d_n2, assign12370_e11483_d_n3, assign12370_e11483_d_n4, assign12370_e11483_d_n5, assign12370_e11483_d_n6, assign12370_e11483_d_n7, assign12370_e11483_d_n8, assign12370_e11483_d_b0, assign12370_e11483_d_b1, assign12370_e11483_d_b2, assign12370_e11483_d_b3, assign12370_e11483_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12370_e11480: f64 = (var_bigd_i * var_vfgd_eff);
        let assign12370_e11481: f64 = (var_aigd_i - assign12370_e11480);
        (assign12370_e11481, (var_aigd_i_dn0 - ((var_bigd_i_dn0 * var_vfgd_eff) + (var_bigd_i * var_vfgd_eff_dn0))), (var_aigd_i_dn1 - ((var_bigd_i_dn1 * var_vfgd_eff) + (var_bigd_i * var_vfgd_eff_dn1))), (var_aigd_i_dn2 - ((var_bigd_i_dn2 * var_vfgd_eff) + (var_bigd_i * var_vfgd_eff_dn2))), (var_aigd_i_dn3 - ((var_bigd_i_dn3 * var_vfgd_eff) + (var_bigd_i * var_vfgd_eff_dn3))), (var_aigd_i_dn4 - ((var_bigd_i_dn4 * var_vfgd_eff) + (var_bigd_i * var_vfgd_eff_dn4))), (var_aigd_i_dn5 - ((var_bigd_i_dn5 * var_vfgd_eff) + (var_bigd_i * var_vfgd_eff_dn5))), (var_aigd_i_dn6 - ((var_bigd_i_dn6 * var_vfgd_eff) + (var_bigd_i * var_vfgd_eff_dn6))), (var_aigd_i_dn7 - ((var_bigd_i_dn7 * var_vfgd_eff) + (var_bigd_i * var_vfgd_eff_dn7))), (var_aigd_i_dn8 - ((var_bigd_i_dn8 * var_vfgd_eff) + (var_bigd_i * var_vfgd_eff_dn8))), (var_aigd_i_db0 - ((var_bigd_i_db0 * var_vfgd_eff) + (var_bigd_i * var_vfgd_eff_db0))), (var_aigd_i_db1 - ((var_bigd_i_db1 * var_vfgd_eff) + (var_bigd_i * var_vfgd_eff_db1))), (var_aigd_i_db2 - ((var_bigd_i_db2 * var_vfgd_eff) + (var_bigd_i * var_vfgd_eff_db2))), (var_aigd_i_db3 - ((var_bigd_i_db3 * var_vfgd_eff) + (var_bigd_i * var_vfgd_eff_db3))), (var_aigd_i_db4 - ((var_bigd_i_db4 * var_vfgd_eff) + (var_bigd_i * var_vfgd_eff_db4))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4,)
    }
};
        var_t1 = assign12370_e11483;
        var_t1_dn0 = assign12370_e11483_d_n0;
        var_t1_dn1 = assign12370_e11483_d_n1;
        var_t1_dn2 = assign12370_e11483_d_n2;
        var_t1_dn3 = assign12370_e11483_d_n3;
        var_t1_dn4 = assign12370_e11483_d_n4;
        var_t1_dn5 = assign12370_e11483_d_n5;
        var_t1_dn6 = assign12370_e11483_d_n6;
        var_t1_dn7 = assign12370_e11483_d_n7;
        var_t1_dn8 = assign12370_e11483_d_n8;
        var_t1_db0 = assign12370_e11483_d_b0;
        var_t1_db1 = assign12370_e11483_d_b1;
        var_t1_db2 = assign12370_e11483_d_b2;
        var_t1_db3 = assign12370_e11483_d_b3;
        var_t1_db4 = assign12370_e11483_d_b4;
        var_t1_rv = 0.0;
        var_t1_rdn0 = 0.0;
        var_t1_rdn1 = 0.0;
        var_t1_rdn2 = 0.0;
        var_t1_rdn3 = 0.0;
        var_t1_rdn4 = 0.0;
        var_t1_rdn5 = 0.0;
        var_t1_rdn6 = 0.0;
        var_t1_rdn7 = 0.0;
        var_t1_rdn8 = 0.0;
        var_t1_rdb0 = 0.0;
        var_t1_rdb1 = 0.0;
        var_t1_rdb2 = 0.0;
        var_t1_rdb3 = 0.0;
        var_t1_rdb4 = 0.0;

        let (assign12380_e11491, assign12380_e11491_d_n0, assign12380_e11491_d_n1, assign12380_e11491_d_n2, assign12380_e11491_d_n3, assign12380_e11491_d_n4, assign12380_e11491_d_n5, assign12380_e11491_d_n6, assign12380_e11491_d_n7, assign12380_e11491_d_n8, assign12380_e11491_d_b0, assign12380_e11491_d_b1, assign12380_e11491_d_b2, assign12380_e11491_d_b3, assign12380_e11491_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12380_e11488: f64 = (var_cigd_i * var_vfgd_eff);
        let assign12380_e11489: f64 = (1.0 + assign12380_e11488);
        (assign12380_e11489, ((var_cigd_i_dn0 * var_vfgd_eff) + (var_cigd_i * var_vfgd_eff_dn0)), ((var_cigd_i_dn1 * var_vfgd_eff) + (var_cigd_i * var_vfgd_eff_dn1)), ((var_cigd_i_dn2 * var_vfgd_eff) + (var_cigd_i * var_vfgd_eff_dn2)), ((var_cigd_i_dn3 * var_vfgd_eff) + (var_cigd_i * var_vfgd_eff_dn3)), ((var_cigd_i_dn4 * var_vfgd_eff) + (var_cigd_i * var_vfgd_eff_dn4)), ((var_cigd_i_dn5 * var_vfgd_eff) + (var_cigd_i * var_vfgd_eff_dn5)), ((var_cigd_i_dn6 * var_vfgd_eff) + (var_cigd_i * var_vfgd_eff_dn6)), ((var_cigd_i_dn7 * var_vfgd_eff) + (var_cigd_i * var_vfgd_eff_dn7)), ((var_cigd_i_dn8 * var_vfgd_eff) + (var_cigd_i * var_vfgd_eff_dn8)), ((var_cigd_i_db0 * var_vfgd_eff) + (var_cigd_i * var_vfgd_eff_db0)), ((var_cigd_i_db1 * var_vfgd_eff) + (var_cigd_i * var_vfgd_eff_db1)), ((var_cigd_i_db2 * var_vfgd_eff) + (var_cigd_i * var_vfgd_eff_db2)), ((var_cigd_i_db3 * var_vfgd_eff) + (var_cigd_i * var_vfgd_eff_db3)), ((var_cigd_i_db4 * var_vfgd_eff) + (var_cigd_i * var_vfgd_eff_db4)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_db0, var_t2_db1, var_t2_db2, var_t2_db3, var_t2_db4,)
    }
};
        var_t2 = assign12380_e11491;
        var_t2_dn0 = assign12380_e11491_d_n0;
        var_t2_dn1 = assign12380_e11491_d_n1;
        var_t2_dn2 = assign12380_e11491_d_n2;
        var_t2_dn3 = assign12380_e11491_d_n3;
        var_t2_dn4 = assign12380_e11491_d_n4;
        var_t2_dn5 = assign12380_e11491_d_n5;
        var_t2_dn6 = assign12380_e11491_d_n6;
        var_t2_dn7 = assign12380_e11491_d_n7;
        var_t2_dn8 = assign12380_e11491_d_n8;
        var_t2_db0 = assign12380_e11491_d_b0;
        var_t2_db1 = assign12380_e11491_d_b1;
        var_t2_db2 = assign12380_e11491_d_b2;
        var_t2_db3 = assign12380_e11491_d_b3;
        var_t2_db4 = assign12380_e11491_d_b4;
        var_t2_rv = 0.0;
        var_t2_rdn0 = 0.0;
        var_t2_rdn1 = 0.0;
        var_t2_rdn2 = 0.0;
        var_t2_rdn3 = 0.0;
        var_t2_rdn4 = 0.0;
        var_t2_rdn5 = 0.0;
        var_t2_rdn6 = 0.0;
        var_t2_rdn7 = 0.0;
        var_t2_rdn8 = 0.0;
        var_t2_rdb0 = 0.0;
        var_t2_rdb1 = 0.0;
        var_t2_rdb2 = 0.0;
        var_t2_rdb3 = 0.0;
        var_t2_rdb4 = 0.0;

        let (assign12390_e11504, assign12390_e11504_d_n0, assign12390_e11504_d_n1, assign12390_e11504_d_n2, assign12390_e11504_d_n3, assign12390_e11504_d_n4, assign12390_e11504_d_n5, assign12390_e11504_d_n6, assign12390_e11504_d_n7, assign12390_e11504_d_n8, assign12390_e11504_d_b0, assign12390_e11504_d_b1, assign12390_e11504_d_b2, assign12390_e11504_d_b3, assign12390_e11504_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12390_e11494: f64 = (-var_bechvb);
        let assign12390_e11496: f64 = (assign12390_e11494 * p.p99);
        let assign12390_e11498: f64 = (assign12390_e11496 * var_poxedge_i);
        let assign12390_e11500: f64 = (assign12390_e11498 * var_t1);
        let assign12390_e11502: f64 = (assign12390_e11500 * var_t2);
        (assign12390_e11502, ((((((((-var_bechvb_dn0) * p.p99) * var_poxedge_i) + (assign12390_e11496 * var_poxedge_i_dn0)) * var_t1) + (assign12390_e11498 * var_t1_dn0)) * var_t2) + (assign12390_e11500 * var_t2_dn0)), ((((((((-var_bechvb_dn1) * p.p99) * var_poxedge_i) + (assign12390_e11496 * var_poxedge_i_dn1)) * var_t1) + (assign12390_e11498 * var_t1_dn1)) * var_t2) + (assign12390_e11500 * var_t2_dn1)), ((((((((-var_bechvb_dn2) * p.p99) * var_poxedge_i) + (assign12390_e11496 * var_poxedge_i_dn2)) * var_t1) + (assign12390_e11498 * var_t1_dn2)) * var_t2) + (assign12390_e11500 * var_t2_dn2)), ((((((((-var_bechvb_dn3) * p.p99) * var_poxedge_i) + (assign12390_e11496 * var_poxedge_i_dn3)) * var_t1) + (assign12390_e11498 * var_t1_dn3)) * var_t2) + (assign12390_e11500 * var_t2_dn3)), ((((((((-var_bechvb_dn4) * p.p99) * var_poxedge_i) + (assign12390_e11496 * var_poxedge_i_dn4)) * var_t1) + (assign12390_e11498 * var_t1_dn4)) * var_t2) + (assign12390_e11500 * var_t2_dn4)), ((((((((-var_bechvb_dn5) * p.p99) * var_poxedge_i) + (assign12390_e11496 * var_poxedge_i_dn5)) * var_t1) + (assign12390_e11498 * var_t1_dn5)) * var_t2) + (assign12390_e11500 * var_t2_dn5)), ((((((((-var_bechvb_dn6) * p.p99) * var_poxedge_i) + (assign12390_e11496 * var_poxedge_i_dn6)) * var_t1) + (assign12390_e11498 * var_t1_dn6)) * var_t2) + (assign12390_e11500 * var_t2_dn6)), ((((((((-var_bechvb_dn7) * p.p99) * var_poxedge_i) + (assign12390_e11496 * var_poxedge_i_dn7)) * var_t1) + (assign12390_e11498 * var_t1_dn7)) * var_t2) + (assign12390_e11500 * var_t2_dn7)), ((((((((-var_bechvb_dn8) * p.p99) * var_poxedge_i) + (assign12390_e11496 * var_poxedge_i_dn8)) * var_t1) + (assign12390_e11498 * var_t1_dn8)) * var_t2) + (assign12390_e11500 * var_t2_dn8)), ((((((((-var_bechvb_db0) * p.p99) * var_poxedge_i) + (assign12390_e11496 * var_poxedge_i_db0)) * var_t1) + (assign12390_e11498 * var_t1_db0)) * var_t2) + (assign12390_e11500 * var_t2_db0)), ((((((((-var_bechvb_db1) * p.p99) * var_poxedge_i) + (assign12390_e11496 * var_poxedge_i_db1)) * var_t1) + (assign12390_e11498 * var_t1_db1)) * var_t2) + (assign12390_e11500 * var_t2_db1)), ((((((((-var_bechvb_db2) * p.p99) * var_poxedge_i) + (assign12390_e11496 * var_poxedge_i_db2)) * var_t1) + (assign12390_e11498 * var_t1_db2)) * var_t2) + (assign12390_e11500 * var_t2_db2)), ((((((((-var_bechvb_db3) * p.p99) * var_poxedge_i) + (assign12390_e11496 * var_poxedge_i_db3)) * var_t1) + (assign12390_e11498 * var_t1_db3)) * var_t2) + (assign12390_e11500 * var_t2_db3)), ((((((((-var_bechvb_db4) * p.p99) * var_poxedge_i) + (assign12390_e11496 * var_poxedge_i_db4)) * var_t1) + (assign12390_e11498 * var_t1_db4)) * var_t2) + (assign12390_e11500 * var_t2_db4)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn1, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_db0, var_t3_db1, var_t3_db2, var_t3_db3, var_t3_db4,)
    }
};
        var_t3 = assign12390_e11504;
        var_t3_dn0 = assign12390_e11504_d_n0;
        var_t3_dn1 = assign12390_e11504_d_n1;
        var_t3_dn2 = assign12390_e11504_d_n2;
        var_t3_dn3 = assign12390_e11504_d_n3;
        var_t3_dn4 = assign12390_e11504_d_n4;
        var_t3_dn5 = assign12390_e11504_d_n5;
        var_t3_dn6 = assign12390_e11504_d_n6;
        var_t3_dn7 = assign12390_e11504_d_n7;
        var_t3_dn8 = assign12390_e11504_d_n8;
        var_t3_db0 = assign12390_e11504_d_b0;
        var_t3_db1 = assign12390_e11504_d_b1;
        var_t3_db2 = assign12390_e11504_d_b2;
        var_t3_db3 = assign12390_e11504_d_b3;
        var_t3_db4 = assign12390_e11504_d_b4;
        var_t3_rv = 0.0;
        var_t3_rdn0 = 0.0;
        var_t3_rdn1 = 0.0;
        var_t3_rdn2 = 0.0;
        var_t3_rdn3 = 0.0;
        var_t3_rdn4 = 0.0;
        var_t3_rdn5 = 0.0;
        var_t3_rdn6 = 0.0;
        var_t3_rdn7 = 0.0;
        var_t3_rdn8 = 0.0;
        var_t3_rdb0 = 0.0;
        var_t3_rdb1 = 0.0;
        var_t3_rdb2 = 0.0;
        var_t3_rdb3 = 0.0;
        var_t3_rdb4 = 0.0;

        let (assign12400_e11509, assign12400_e11509_d_n0, assign12400_e11509_d_n1, assign12400_e11509_d_n2, assign12400_e11509_d_n3, assign12400_e11509_d_n4, assign12400_e11509_d_n5, assign12400_e11509_d_n6, assign12400_e11509_d_n7, assign12400_e11509_d_n8, assign12400_e11509_d_b0, assign12400_e11509_d_b1, assign12400_e11509_d_b2, assign12400_e11509_d_b3, assign12400_e11509_d_b4,) = {
    if (var_guard123 != 0.0) {
        let assign12400_e11507: f64 = { let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12400_e11507, ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn0), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn1), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn2), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn3), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn4), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn5), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn6), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn7), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn8), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db0), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db1), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db2), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db3), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_db4),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn1, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_db0, var_t4_db1, var_t4_db2, var_t4_db3, var_t4_db4,)
    }
};
        var_t4 = assign12400_e11509;
        var_t4_dn0 = assign12400_e11509_d_n0;
        var_t4_dn1 = assign12400_e11509_d_n1;
        var_t4_dn2 = assign12400_e11509_d_n2;
        var_t4_dn3 = assign12400_e11509_d_n3;
        var_t4_dn4 = assign12400_e11509_d_n4;
        var_t4_dn5 = assign12400_e11509_d_n5;
        var_t4_dn6 = assign12400_e11509_d_n6;
        var_t4_dn7 = assign12400_e11509_d_n7;
        var_t4_dn8 = assign12400_e11509_d_n8;
        var_t4_db0 = assign12400_e11509_d_b0;
        var_t4_db1 = assign12400_e11509_d_b1;
        var_t4_db2 = assign12400_e11509_d_b2;
        var_t4_db3 = assign12400_e11509_d_b3;
        var_t4_db4 = assign12400_e11509_d_b4;
        var_t4_rv = 0.0;
        var_t4_rdn0 = 0.0;
        var_t4_rdn1 = 0.0;
        var_t4_rdn2 = 0.0;
        var_t4_rdn3 = 0.0;
        var_t4_rdn4 = 0.0;
        var_t4_rdn5 = 0.0;
        var_t4_rdn6 = 0.0;
        var_t4_rdn7 = 0.0;
        var_t4_rdn8 = 0.0;
        var_t4_rdb0 = 0.0;
        var_t4_rdb1 = 0.0;
        var_t4_rdb2 = 0.0;
        var_t4_rdb3 = 0.0;
        var_t4_rdb4 = 0.0;

        let assign12460_e11546: f64 = if p.p15 != 0.0 { 1.0 } else { 0.0 };
        var_guard126 = assign12460_e11546;
        var_guard126_dn0 = 0.0;
        var_guard126_dn1 = 0.0;
        var_guard126_dn2 = 0.0;
        var_guard126_dn3 = 0.0;
        var_guard126_dn4 = 0.0;
        var_guard126_dn5 = 0.0;
        var_guard126_dn6 = 0.0;
        var_guard126_dn7 = 0.0;
        var_guard126_dn8 = 0.0;
        var_guard126_db0 = 0.0;
        var_guard126_db1 = 0.0;
        var_guard126_db2 = 0.0;
        var_guard126_db3 = 0.0;
        var_guard126_db4 = 0.0;
        var_guard126_rv = 0.0;
        var_guard126_rdn0 = 0.0;
        var_guard126_rdn1 = 0.0;
        var_guard126_rdn2 = 0.0;
        var_guard126_rdn3 = 0.0;
        var_guard126_rdn4 = 0.0;
        var_guard126_rdn5 = 0.0;
        var_guard126_rdn6 = 0.0;
        var_guard126_rdn7 = 0.0;
        var_guard126_rdn8 = 0.0;
        var_guard126_rdb0 = 0.0;
        var_guard126_rdb1 = 0.0;
        var_guard126_rdb2 = 0.0;
        var_guard126_rdb3 = 0.0;
        var_guard126_rdb4 = 0.0;

        let (assign12470_e11552, assign12470_e11552_d_n0, assign12470_e11552_d_n1, assign12470_e11552_d_n2, assign12470_e11552_d_n3, assign12470_e11552_d_n4, assign12470_e11552_d_n5, assign12470_e11552_d_n6, assign12470_e11552_d_n7, assign12470_e11552_d_n8, assign12470_e11552_d_b0, assign12470_e11552_d_b1, assign12470_e11552_d_b2, assign12470_e11552_d_b3, assign12470_e11552_d_b4,) = {
    if (var_guard126 != 0.0) {
        let assign12470_e11550: f64 = (var_epsratio * p.p45);
        (assign12470_e11550, (var_epsratio_dn0 * p.p45), (var_epsratio_dn1 * p.p45), (var_epsratio_dn2 * p.p45), (var_epsratio_dn3 * p.p45), (var_epsratio_dn4 * p.p45), (var_epsratio_dn5 * p.p45), (var_epsratio_dn6 * p.p45), (var_epsratio_dn7 * p.p45), (var_epsratio_dn8 * p.p45), (var_epsratio_db0 * p.p45), (var_epsratio_db1 * p.p45), (var_epsratio_db2 * p.p45), (var_epsratio_db3 * p.p45), (var_epsratio_db4 * p.p45),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4,)
    }
};
        var_t0 = assign12470_e11552;
        var_t0_dn0 = assign12470_e11552_d_n0;
        var_t0_dn1 = assign12470_e11552_d_n1;
        var_t0_dn2 = assign12470_e11552_d_n2;
        var_t0_dn3 = assign12470_e11552_d_n3;
        var_t0_dn4 = assign12470_e11552_d_n4;
        var_t0_dn5 = assign12470_e11552_d_n5;
        var_t0_dn6 = assign12470_e11552_d_n6;
        var_t0_dn7 = assign12470_e11552_d_n7;
        var_t0_dn8 = assign12470_e11552_d_n8;
        var_t0_db0 = assign12470_e11552_d_b0;
        var_t0_db1 = assign12470_e11552_d_b1;
        var_t0_db2 = assign12470_e11552_d_b2;
        var_t0_db3 = assign12470_e11552_d_b3;
        var_t0_db4 = assign12470_e11552_d_b4;
        var_t0_rv = 0.0;
        var_t0_rdn0 = 0.0;
        var_t0_rdn1 = 0.0;
        var_t0_rdn2 = 0.0;
        var_t0_rdn3 = 0.0;
        var_t0_rdn4 = 0.0;
        var_t0_rdn5 = 0.0;
        var_t0_rdn6 = 0.0;
        var_t0_rdn7 = 0.0;
        var_t0_rdn8 = 0.0;
        var_t0_rdb0 = 0.0;
        var_t0_rdb1 = 0.0;
        var_t0_rdb2 = 0.0;
        var_t0_rdb3 = 0.0;
        var_t0_rdb4 = 0.0;

        let assign12480_e11559: f64 = if ((var_agidl_i <= 0.0) || (var_bgidl_t <= 0.0)) { 1.0 } else { 0.0 };
        var_guard127 = assign12480_e11559;
        var_guard127_dn0 = 0.0;
        var_guard127_dn1 = 0.0;
        var_guard127_dn2 = 0.0;
        var_guard127_dn3 = 0.0;
        var_guard127_dn4 = 0.0;
        var_guard127_dn5 = 0.0;
        var_guard127_dn6 = 0.0;
        var_guard127_dn7 = 0.0;
        var_guard127_dn8 = 0.0;
        var_guard127_db0 = 0.0;
        var_guard127_db1 = 0.0;
        var_guard127_db2 = 0.0;
        var_guard127_db3 = 0.0;
        var_guard127_db4 = 0.0;
        var_guard127_rv = 0.0;
        var_guard127_rdn0 = 0.0;
        var_guard127_rdn1 = 0.0;
        var_guard127_rdn2 = 0.0;
        var_guard127_rdn3 = 0.0;
        var_guard127_rdn4 = 0.0;
        var_guard127_rdn5 = 0.0;
        var_guard127_rdn6 = 0.0;
        var_guard127_rdn7 = 0.0;
        var_guard127_rdn8 = 0.0;
        var_guard127_rdb0 = 0.0;
        var_guard127_rdb1 = 0.0;
        var_guard127_rdb2 = 0.0;
        var_guard127_rdb3 = 0.0;
        var_guard127_rdb4 = 0.0;

        let (assign12490_e11565, assign12490_e11565_d_n0, assign12490_e11565_d_n1, assign12490_e11565_d_n2, assign12490_e11565_d_n3, assign12490_e11565_d_n4, assign12490_e11565_d_n5, assign12490_e11565_d_n6, assign12490_e11565_d_n7, assign12490_e11565_d_n8, assign12490_e11565_d_b0, assign12490_e11565_d_b1, assign12490_e11565_d_b2, assign12490_e11565_d_b3, assign12490_e11565_d_b4,) = {
    if ((var_guard126 != 0.0) && (var_guard127 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn1, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_db0, var_t6_db1, var_t6_db2, var_t6_db3, var_t6_db4,)
    }
};
        var_t6 = assign12490_e11565;
        var_t6_dn0 = assign12490_e11565_d_n0;
        var_t6_dn1 = assign12490_e11565_d_n1;
        var_t6_dn2 = assign12490_e11565_d_n2;
        var_t6_dn3 = assign12490_e11565_d_n3;
        var_t6_dn4 = assign12490_e11565_d_n4;
        var_t6_dn5 = assign12490_e11565_d_n5;
        var_t6_dn6 = assign12490_e11565_d_n6;
        var_t6_dn7 = assign12490_e11565_d_n7;
        var_t6_dn8 = assign12490_e11565_d_n8;
        var_t6_db0 = assign12490_e11565_d_b0;
        var_t6_db1 = assign12490_e11565_d_b1;
        var_t6_db2 = assign12490_e11565_d_b2;
        var_t6_db3 = assign12490_e11565_d_b3;
        var_t6_db4 = assign12490_e11565_d_b4;
        var_t6_rv = 0.0;
        var_t6_rdn0 = 0.0;
        var_t6_rdn1 = 0.0;
        var_t6_rdn2 = 0.0;
        var_t6_rdn3 = 0.0;
        var_t6_rdn4 = 0.0;
        var_t6_rdn5 = 0.0;
        var_t6_rdn6 = 0.0;
        var_t6_rdn7 = 0.0;
        var_t6_rdn8 = 0.0;
        var_t6_rdb0 = 0.0;
        var_t6_rdb1 = 0.0;
        var_t6_rdb2 = 0.0;
        var_t6_rdb3 = 0.0;
        var_t6_rdb4 = 0.0;

        let (assign12500_e11589, assign12500_e11589_d_n0, assign12500_e11589_d_n1, assign12500_e11589_d_n2, assign12500_e11589_d_n3, assign12500_e11589_d_n4, assign12500_e11589_d_n5, assign12500_e11589_d_n6, assign12500_e11589_d_n7, assign12500_e11589_d_n8, assign12500_e11589_d_b0, assign12500_e11589_d_b1, assign12500_e11589_d_b2, assign12500_e11589_d_b3, assign12500_e11589_d_b4,) = {
    if ((var_guard126 != 0.0) && (var_guard127 == 0.0)) {
        let assign12500_e11571: f64 = (-var_vgd_noswap);
        let assign12500_e11573: f64 = (assign12500_e11571 - var_egidl_i);
        let assign12500_e11575: f64 = (assign12500_e11573 + var_vfbsd);
        let assign12500_e11578: f64 = (var_vbgidl_i * var_gamma0);
        let assign12500_e11581: f64 = (var_vbgs - var_vfbsd_bg);
        let assign12500_e11583: f64 = (assign12500_e11581 - var_vbegidl_i);
        let assign12500_e11584: f64 = (assign12500_e11578 * assign12500_e11583);
        let assign12500_e11585: f64 = (assign12500_e11575 + assign12500_e11584);
        let assign12500_e11587: f64 = (assign12500_e11585 / var_t0);
        (assign12500_e11587, (((((((-var_vgd_noswap_dn0) - var_egidl_i_dn0) + var_vfbsd_dn0) + ((((var_vbgidl_i_dn0 * var_gamma0) + (var_vbgidl_i * var_gamma0_dn0)) * assign12500_e11583) + (assign12500_e11578 * ((var_vbgs_dn0 - var_vfbsd_bg_dn0) - var_vbegidl_i_dn0)))) * var_t0) - (assign12500_e11585 * var_t0_dn0)) / (var_t0 * var_t0)), (((((((-var_vgd_noswap_dn1) - var_egidl_i_dn1) + var_vfbsd_dn1) + ((((var_vbgidl_i_dn1 * var_gamma0) + (var_vbgidl_i * var_gamma0_dn1)) * assign12500_e11583) + (assign12500_e11578 * ((var_vbgs_dn1 - var_vfbsd_bg_dn1) - var_vbegidl_i_dn1)))) * var_t0) - (assign12500_e11585 * var_t0_dn1)) / (var_t0 * var_t0)), (((((((-var_vgd_noswap_dn2) - var_egidl_i_dn2) + var_vfbsd_dn2) + ((((var_vbgidl_i_dn2 * var_gamma0) + (var_vbgidl_i * var_gamma0_dn2)) * assign12500_e11583) + (assign12500_e11578 * ((var_vbgs_dn2 - var_vfbsd_bg_dn2) - var_vbegidl_i_dn2)))) * var_t0) - (assign12500_e11585 * var_t0_dn2)) / (var_t0 * var_t0)), (((((((-var_vgd_noswap_dn3) - var_egidl_i_dn3) + var_vfbsd_dn3) + ((((var_vbgidl_i_dn3 * var_gamma0) + (var_vbgidl_i * var_gamma0_dn3)) * assign12500_e11583) + (assign12500_e11578 * ((var_vbgs_dn3 - var_vfbsd_bg_dn3) - var_vbegidl_i_dn3)))) * var_t0) - (assign12500_e11585 * var_t0_dn3)) / (var_t0 * var_t0)), (((((((-var_vgd_noswap_dn4) - var_egidl_i_dn4) + var_vfbsd_dn4) + ((((var_vbgidl_i_dn4 * var_gamma0) + (var_vbgidl_i * var_gamma0_dn4)) * assign12500_e11583) + (assign12500_e11578 * ((var_vbgs_dn4 - var_vfbsd_bg_dn4) - var_vbegidl_i_dn4)))) * var_t0) - (assign12500_e11585 * var_t0_dn4)) / (var_t0 * var_t0)), (((((((-var_vgd_noswap_dn5) - var_egidl_i_dn5) + var_vfbsd_dn5) + ((((var_vbgidl_i_dn5 * var_gamma0) + (var_vbgidl_i * var_gamma0_dn5)) * assign12500_e11583) + (assign12500_e11578 * ((var_vbgs_dn5 - var_vfbsd_bg_dn5) - var_vbegidl_i_dn5)))) * var_t0) - (assign12500_e11585 * var_t0_dn5)) / (var_t0 * var_t0)), (((((((-var_vgd_noswap_dn6) - var_egidl_i_dn6) + var_vfbsd_dn6) + ((((var_vbgidl_i_dn6 * var_gamma0) + (var_vbgidl_i * var_gamma0_dn6)) * assign12500_e11583) + (assign12500_e11578 * ((var_vbgs_dn6 - var_vfbsd_bg_dn6) - var_vbegidl_i_dn6)))) * var_t0) - (assign12500_e11585 * var_t0_dn6)) / (var_t0 * var_t0)), (((((((-var_vgd_noswap_dn7) - var_egidl_i_dn7) + var_vfbsd_dn7) + ((((var_vbgidl_i_dn7 * var_gamma0) + (var_vbgidl_i * var_gamma0_dn7)) * assign12500_e11583) + (assign12500_e11578 * ((var_vbgs_dn7 - var_vfbsd_bg_dn7) - var_vbegidl_i_dn7)))) * var_t0) - (assign12500_e11585 * var_t0_dn7)) / (var_t0 * var_t0)), (((((((-var_vgd_noswap_dn8) - var_egidl_i_dn8) + var_vfbsd_dn8) + ((((var_vbgidl_i_dn8 * var_gamma0) + (var_vbgidl_i * var_gamma0_dn8)) * assign12500_e11583) + (assign12500_e11578 * ((var_vbgs_dn8 - var_vfbsd_bg_dn8) - var_vbegidl_i_dn8)))) * var_t0) - (assign12500_e11585 * var_t0_dn8)) / (var_t0 * var_t0)), (((((((-var_vgd_noswap_db0) - var_egidl_i_db0) + var_vfbsd_db0) + ((((var_vbgidl_i_db0 * var_gamma0) + (var_vbgidl_i * var_gamma0_db0)) * assign12500_e11583) + (assign12500_e11578 * ((var_vbgs_db0 - var_vfbsd_bg_db0) - var_vbegidl_i_db0)))) * var_t0) - (assign12500_e11585 * var_t0_db0)) / (var_t0 * var_t0)), (((((((-var_vgd_noswap_db1) - var_egidl_i_db1) + var_vfbsd_db1) + ((((var_vbgidl_i_db1 * var_gamma0) + (var_vbgidl_i * var_gamma0_db1)) * assign12500_e11583) + (assign12500_e11578 * ((var_vbgs_db1 - var_vfbsd_bg_db1) - var_vbegidl_i_db1)))) * var_t0) - (assign12500_e11585 * var_t0_db1)) / (var_t0 * var_t0)), (((((((-var_vgd_noswap_db2) - var_egidl_i_db2) + var_vfbsd_db2) + ((((var_vbgidl_i_db2 * var_gamma0) + (var_vbgidl_i * var_gamma0_db2)) * assign12500_e11583) + (assign12500_e11578 * ((var_vbgs_db2 - var_vfbsd_bg_db2) - var_vbegidl_i_db2)))) * var_t0) - (assign12500_e11585 * var_t0_db2)) / (var_t0 * var_t0)), (((((((-var_vgd_noswap_db3) - var_egidl_i_db3) + var_vfbsd_db3) + ((((var_vbgidl_i_db3 * var_gamma0) + (var_vbgidl_i * var_gamma0_db3)) * assign12500_e11583) + (assign12500_e11578 * ((var_vbgs_db3 - var_vfbsd_bg_db3) - var_vbegidl_i_db3)))) * var_t0) - (assign12500_e11585 * var_t0_db3)) / (var_t0 * var_t0)), (((((((-var_vgd_noswap_db4) - var_egidl_i_db4) + var_vfbsd_db4) + ((((var_vbgidl_i_db4 * var_gamma0) + (var_vbgidl_i * var_gamma0_db4)) * assign12500_e11583) + (assign12500_e11578 * ((var_vbgs_db4 - var_vfbsd_bg_db4) - var_vbegidl_i_db4)))) * var_t0) - (assign12500_e11585 * var_t0_db4)) / (var_t0 * var_t0)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4,)
    }
};
        var_t1 = assign12500_e11589;
        var_t1_dn0 = assign12500_e11589_d_n0;
        var_t1_dn1 = assign12500_e11589_d_n1;
        var_t1_dn2 = assign12500_e11589_d_n2;
        var_t1_dn3 = assign12500_e11589_d_n3;
        var_t1_dn4 = assign12500_e11589_d_n4;
        var_t1_dn5 = assign12500_e11589_d_n5;
        var_t1_dn6 = assign12500_e11589_d_n6;
        var_t1_dn7 = assign12500_e11589_d_n7;
        var_t1_dn8 = assign12500_e11589_d_n8;
        var_t1_db0 = assign12500_e11589_d_b0;
        var_t1_db1 = assign12500_e11589_d_b1;
        var_t1_db2 = assign12500_e11589_d_b2;
        var_t1_db3 = assign12500_e11589_d_b3;
        var_t1_db4 = assign12500_e11589_d_b4;
        var_t1_rv = 0.0;
        var_t1_rdn0 = 0.0;
        var_t1_rdn1 = 0.0;
        var_t1_rdn2 = 0.0;
        var_t1_rdn3 = 0.0;
        var_t1_rdn4 = 0.0;
        var_t1_rdn5 = 0.0;
        var_t1_rdn6 = 0.0;
        var_t1_rdn7 = 0.0;
        var_t1_rdn8 = 0.0;
        var_t1_rdb0 = 0.0;
        var_t1_rdb1 = 0.0;
        var_t1_rdb2 = 0.0;
        var_t1_rdb3 = 0.0;
        var_t1_rdb4 = 0.0;

        let (assign12510_e11609, assign12510_e11609_d_n0, assign12510_e11609_d_n1, assign12510_e11609_d_n2, assign12510_e11609_d_n3, assign12510_e11609_d_n4, assign12510_e11609_d_n5, assign12510_e11609_d_n6, assign12510_e11609_d_n7, assign12510_e11609_d_n8, assign12510_e11609_d_b0, assign12510_e11609_d_b1, assign12510_e11609_d_b2, assign12510_e11609_d_b3, assign12510_e11609_d_b4,) = {
    if ((var_guard126 != 0.0) && (var_guard127 == 0.0)) {
        let assign12510_e11598: f64 = (var_t1 * var_t1);
        let assign12510_e11601: f64 = (4.0 * 0.01);
        let assign12510_e11603: f64 = (assign12510_e11601 * 0.01);
        let assign12510_e11604: f64 = (assign12510_e11598 + assign12510_e11603);
        let assign12510_e11605: f64 = (assign12510_e11604).sqrt();
        let assign12510_e11606: f64 = (var_t1 + assign12510_e11605);
        let assign12510_e11607: f64 = (0.5 * assign12510_e11606);
        (assign12510_e11607, (0.5 * (var_t1_dn0 + (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_dn1 + (((var_t1_dn1 * var_t1) + (var_t1 * var_t1_dn1)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_dn2 + (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_dn3 + (((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_dn4 + (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_dn5 + (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_dn6 + (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_dn7 + (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_dn8 + (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_db0 + (((var_t1_db0 * var_t1) + (var_t1 * var_t1_db0)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_db1 + (((var_t1_db1 * var_t1) + (var_t1 * var_t1_db1)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_db2 + (((var_t1_db2 * var_t1) + (var_t1 * var_t1_db2)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_db3 + (((var_t1_db3 * var_t1) + (var_t1 * var_t1_db3)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_db4 + (((var_t1_db4 * var_t1) + (var_t1 * var_t1_db4)) / (2.0 * assign12510_e11605)))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4,)
    }
};
        var_t1 = assign12510_e11609;
        var_t1_dn0 = assign12510_e11609_d_n0;
        var_t1_dn1 = assign12510_e11609_d_n1;
        var_t1_dn2 = assign12510_e11609_d_n2;
        var_t1_dn3 = assign12510_e11609_d_n3;
        var_t1_dn4 = assign12510_e11609_d_n4;
        var_t1_dn5 = assign12510_e11609_d_n5;
        var_t1_dn6 = assign12510_e11609_d_n6;
        var_t1_dn7 = assign12510_e11609_d_n7;
        var_t1_dn8 = assign12510_e11609_d_n8;
        var_t1_db0 = assign12510_e11609_d_b0;
        var_t1_db1 = assign12510_e11609_d_b1;
        var_t1_db2 = assign12510_e11609_d_b2;
        var_t1_db3 = assign12510_e11609_d_b3;
        var_t1_db4 = assign12510_e11609_d_b4;
        var_t1_rv = 0.0;
        var_t1_rdn0 = 0.0;
        var_t1_rdn1 = 0.0;
        var_t1_rdn2 = 0.0;
        var_t1_rdn3 = 0.0;
        var_t1_rdn4 = 0.0;
        var_t1_rdn5 = 0.0;
        var_t1_rdn6 = 0.0;
        var_t1_rdn7 = 0.0;
        var_t1_rdn8 = 0.0;
        var_t1_rdb0 = 0.0;
        var_t1_rdb1 = 0.0;
        var_t1_rdb2 = 0.0;
        var_t1_rdb3 = 0.0;
        var_t1_rdb4 = 0.0;


        *var_guard126_slot = var_guard126;
        *var_guard126_db0_slot = var_guard126_db0;
        *var_guard126_db1_slot = var_guard126_db1;
        *var_guard126_db2_slot = var_guard126_db2;
        *var_guard126_db3_slot = var_guard126_db3;
        *var_guard126_db4_slot = var_guard126_db4;
        *var_guard126_dn0_slot = var_guard126_dn0;
        *var_guard126_dn1_slot = var_guard126_dn1;
        *var_guard126_dn2_slot = var_guard126_dn2;
        *var_guard126_dn3_slot = var_guard126_dn3;
        *var_guard126_dn4_slot = var_guard126_dn4;
        *var_guard126_dn5_slot = var_guard126_dn5;
        *var_guard126_dn6_slot = var_guard126_dn6;
        *var_guard126_dn7_slot = var_guard126_dn7;
        *var_guard126_dn8_slot = var_guard126_dn8;
        *var_guard126_rdb0_slot = var_guard126_rdb0;
        *var_guard126_rdb1_slot = var_guard126_rdb1;
        *var_guard126_rdb2_slot = var_guard126_rdb2;
        *var_guard126_rdb3_slot = var_guard126_rdb3;
        *var_guard126_rdb4_slot = var_guard126_rdb4;
        *var_guard126_rdn0_slot = var_guard126_rdn0;
        *var_guard126_rdn1_slot = var_guard126_rdn1;
        *var_guard126_rdn2_slot = var_guard126_rdn2;
        *var_guard126_rdn3_slot = var_guard126_rdn3;
        *var_guard126_rdn4_slot = var_guard126_rdn4;
        *var_guard126_rdn5_slot = var_guard126_rdn5;
        *var_guard126_rdn6_slot = var_guard126_rdn6;
        *var_guard126_rdn7_slot = var_guard126_rdn7;
        *var_guard126_rdn8_slot = var_guard126_rdn8;
        *var_guard126_rv_slot = var_guard126_rv;
        *var_guard127_slot = var_guard127;
        *var_guard127_db0_slot = var_guard127_db0;
        *var_guard127_db1_slot = var_guard127_db1;
        *var_guard127_db2_slot = var_guard127_db2;
        *var_guard127_db3_slot = var_guard127_db3;
        *var_guard127_db4_slot = var_guard127_db4;
        *var_guard127_dn0_slot = var_guard127_dn0;
        *var_guard127_dn1_slot = var_guard127_dn1;
        *var_guard127_dn2_slot = var_guard127_dn2;
        *var_guard127_dn3_slot = var_guard127_dn3;
        *var_guard127_dn4_slot = var_guard127_dn4;
        *var_guard127_dn5_slot = var_guard127_dn5;
        *var_guard127_dn6_slot = var_guard127_dn6;
        *var_guard127_dn7_slot = var_guard127_dn7;
        *var_guard127_dn8_slot = var_guard127_dn8;
        *var_guard127_rdb0_slot = var_guard127_rdb0;
        *var_guard127_rdb1_slot = var_guard127_rdb1;
        *var_guard127_rdb2_slot = var_guard127_rdb2;
        *var_guard127_rdb3_slot = var_guard127_rdb3;
        *var_guard127_rdb4_slot = var_guard127_rdb4;
        *var_guard127_rdn0_slot = var_guard127_rdn0;
        *var_guard127_rdn1_slot = var_guard127_rdn1;
        *var_guard127_rdn2_slot = var_guard127_rdn2;
        *var_guard127_rdn3_slot = var_guard127_rdn3;
        *var_guard127_rdn4_slot = var_guard127_rdn4;
        *var_guard127_rdn5_slot = var_guard127_rdn5;
        *var_guard127_rdn6_slot = var_guard127_rdn6;
        *var_guard127_rdn7_slot = var_guard127_rdn7;
        *var_guard127_rdn8_slot = var_guard127_rdn8;
        *var_guard127_rv_slot = var_guard127_rv;
        *var_t0_slot = var_t0;
        *var_t0_db0_slot = var_t0_db0;
        *var_t0_db1_slot = var_t0_db1;
        *var_t0_db2_slot = var_t0_db2;
        *var_t0_db3_slot = var_t0_db3;
        *var_t0_db4_slot = var_t0_db4;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rdb0_slot = var_t0_rdb0;
        *var_t0_rdb1_slot = var_t0_rdb1;
        *var_t0_rdb2_slot = var_t0_rdb2;
        *var_t0_rdb3_slot = var_t0_rdb3;
        *var_t0_rdb4_slot = var_t0_rdb4;
        *var_t0_rdn0_slot = var_t0_rdn0;
        *var_t0_rdn1_slot = var_t0_rdn1;
        *var_t0_rdn2_slot = var_t0_rdn2;
        *var_t0_rdn3_slot = var_t0_rdn3;
        *var_t0_rdn4_slot = var_t0_rdn4;
        *var_t0_rdn5_slot = var_t0_rdn5;
        *var_t0_rdn6_slot = var_t0_rdn6;
        *var_t0_rdn7_slot = var_t0_rdn7;
        *var_t0_rdn8_slot = var_t0_rdn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_db0_slot = var_t1_db0;
        *var_t1_db1_slot = var_t1_db1;
        *var_t1_db2_slot = var_t1_db2;
        *var_t1_db3_slot = var_t1_db3;
        *var_t1_db4_slot = var_t1_db4;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rdb0_slot = var_t1_rdb0;
        *var_t1_rdb1_slot = var_t1_rdb1;
        *var_t1_rdb2_slot = var_t1_rdb2;
        *var_t1_rdb3_slot = var_t1_rdb3;
        *var_t1_rdb4_slot = var_t1_rdb4;
        *var_t1_rdn0_slot = var_t1_rdn0;
        *var_t1_rdn1_slot = var_t1_rdn1;
        *var_t1_rdn2_slot = var_t1_rdn2;
        *var_t1_rdn3_slot = var_t1_rdn3;
        *var_t1_rdn4_slot = var_t1_rdn4;
        *var_t1_rdn5_slot = var_t1_rdn5;
        *var_t1_rdn6_slot = var_t1_rdn6;
        *var_t1_rdn7_slot = var_t1_rdn7;
        *var_t1_rdn8_slot = var_t1_rdn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_db0_slot = var_t2_db0;
        *var_t2_db1_slot = var_t2_db1;
        *var_t2_db2_slot = var_t2_db2;
        *var_t2_db3_slot = var_t2_db3;
        *var_t2_db4_slot = var_t2_db4;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rdb0_slot = var_t2_rdb0;
        *var_t2_rdb1_slot = var_t2_rdb1;
        *var_t2_rdb2_slot = var_t2_rdb2;
        *var_t2_rdb3_slot = var_t2_rdb3;
        *var_t2_rdb4_slot = var_t2_rdb4;
        *var_t2_rdn0_slot = var_t2_rdn0;
        *var_t2_rdn1_slot = var_t2_rdn1;
        *var_t2_rdn2_slot = var_t2_rdn2;
        *var_t2_rdn3_slot = var_t2_rdn3;
        *var_t2_rdn4_slot = var_t2_rdn4;
        *var_t2_rdn5_slot = var_t2_rdn5;
        *var_t2_rdn6_slot = var_t2_rdn6;
        *var_t2_rdn7_slot = var_t2_rdn7;
        *var_t2_rdn8_slot = var_t2_rdn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_db0_slot = var_t3_db0;
        *var_t3_db1_slot = var_t3_db1;
        *var_t3_db2_slot = var_t3_db2;
        *var_t3_db3_slot = var_t3_db3;
        *var_t3_db4_slot = var_t3_db4;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn1_slot = var_t3_dn1;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rdb0_slot = var_t3_rdb0;
        *var_t3_rdb1_slot = var_t3_rdb1;
        *var_t3_rdb2_slot = var_t3_rdb2;
        *var_t3_rdb3_slot = var_t3_rdb3;
        *var_t3_rdb4_slot = var_t3_rdb4;
        *var_t3_rdn0_slot = var_t3_rdn0;
        *var_t3_rdn1_slot = var_t3_rdn1;
        *var_t3_rdn2_slot = var_t3_rdn2;
        *var_t3_rdn3_slot = var_t3_rdn3;
        *var_t3_rdn4_slot = var_t3_rdn4;
        *var_t3_rdn5_slot = var_t3_rdn5;
        *var_t3_rdn6_slot = var_t3_rdn6;
        *var_t3_rdn7_slot = var_t3_rdn7;
        *var_t3_rdn8_slot = var_t3_rdn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_db0_slot = var_t4_db0;
        *var_t4_db1_slot = var_t4_db1;
        *var_t4_db2_slot = var_t4_db2;
        *var_t4_db3_slot = var_t4_db3;
        *var_t4_db4_slot = var_t4_db4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn1_slot = var_t4_dn1;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_rdb0_slot = var_t4_rdb0;
        *var_t4_rdb1_slot = var_t4_rdb1;
        *var_t4_rdb2_slot = var_t4_rdb2;
        *var_t4_rdb3_slot = var_t4_rdb3;
        *var_t4_rdb4_slot = var_t4_rdb4;
        *var_t4_rdn0_slot = var_t4_rdn0;
        *var_t4_rdn1_slot = var_t4_rdn1;
        *var_t4_rdn2_slot = var_t4_rdn2;
        *var_t4_rdn3_slot = var_t4_rdn3;
        *var_t4_rdn4_slot = var_t4_rdn4;
        *var_t4_rdn5_slot = var_t4_rdn5;
        *var_t4_rdn6_slot = var_t4_rdn6;
        *var_t4_rdn7_slot = var_t4_rdn7;
        *var_t4_rdn8_slot = var_t4_rdn8;
        *var_t4_rv_slot = var_t4_rv;
        *var_t6_slot = var_t6;
        *var_t6_db0_slot = var_t6_db0;
        *var_t6_db1_slot = var_t6_db1;
        *var_t6_db2_slot = var_t6_db2;
        *var_t6_db3_slot = var_t6_db3;
        *var_t6_db4_slot = var_t6_db4;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn1_slot = var_t6_dn1;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_rdb0_slot = var_t6_rdb0;
        *var_t6_rdb1_slot = var_t6_rdb1;
        *var_t6_rdb2_slot = var_t6_rdb2;
        *var_t6_rdb3_slot = var_t6_rdb3;
        *var_t6_rdb4_slot = var_t6_rdb4;
        *var_t6_rdn0_slot = var_t6_rdn0;
        *var_t6_rdn1_slot = var_t6_rdn1;
        *var_t6_rdn2_slot = var_t6_rdn2;
        *var_t6_rdn3_slot = var_t6_rdn3;
        *var_t6_rdn4_slot = var_t6_rdn4;
        *var_t6_rdn5_slot = var_t6_rdn5;
        *var_t6_rdn6_slot = var_t6_rdn6;
        *var_t6_rdn7_slot = var_t6_rdn7;
        *var_t6_rdn8_slot = var_t6_rdn8;
        *var_t6_rv_slot = var_t6_rv;
        *var_vfgd_eff_slot = var_vfgd_eff;
        *var_vfgd_eff_db0_slot = var_vfgd_eff_db0;
        *var_vfgd_eff_db1_slot = var_vfgd_eff_db1;
        *var_vfgd_eff_db2_slot = var_vfgd_eff_db2;
        *var_vfgd_eff_db3_slot = var_vfgd_eff_db3;
        *var_vfgd_eff_db4_slot = var_vfgd_eff_db4;
        *var_vfgd_eff_dn0_slot = var_vfgd_eff_dn0;
        *var_vfgd_eff_dn1_slot = var_vfgd_eff_dn1;
        *var_vfgd_eff_dn2_slot = var_vfgd_eff_dn2;
        *var_vfgd_eff_dn3_slot = var_vfgd_eff_dn3;
        *var_vfgd_eff_dn4_slot = var_vfgd_eff_dn4;
        *var_vfgd_eff_dn5_slot = var_vfgd_eff_dn5;
        *var_vfgd_eff_dn6_slot = var_vfgd_eff_dn6;
        *var_vfgd_eff_dn7_slot = var_vfgd_eff_dn7;
        *var_vfgd_eff_dn8_slot = var_vfgd_eff_dn8;
        *var_vfgd_eff_rdb0_slot = var_vfgd_eff_rdb0;
        *var_vfgd_eff_rdb1_slot = var_vfgd_eff_rdb1;
        *var_vfgd_eff_rdb2_slot = var_vfgd_eff_rdb2;
        *var_vfgd_eff_rdb3_slot = var_vfgd_eff_rdb3;
        *var_vfgd_eff_rdb4_slot = var_vfgd_eff_rdb4;
        *var_vfgd_eff_rdn0_slot = var_vfgd_eff_rdn0;
        *var_vfgd_eff_rdn1_slot = var_vfgd_eff_rdn1;
        *var_vfgd_eff_rdn2_slot = var_vfgd_eff_rdn2;
        *var_vfgd_eff_rdn3_slot = var_vfgd_eff_rdn3;
        *var_vfgd_eff_rdn4_slot = var_vfgd_eff_rdn4;
        *var_vfgd_eff_rdn5_slot = var_vfgd_eff_rdn5;
        *var_vfgd_eff_rdn6_slot = var_vfgd_eff_rdn6;
        *var_vfgd_eff_rdn7_slot = var_vfgd_eff_rdn7;
        *var_vfgd_eff_rdn8_slot = var_vfgd_eff_rdn8;
        *var_vfgd_eff_rv_slot = var_vfgd_eff_rv;
    }

    pub(super) fn stamp_reactive_block_86(
        p: &Parameters,
        var_agidl_i: f64,
        var_agidl_i_db0: f64,
        var_agidl_i_db1: f64,
        var_agidl_i_db2: f64,
        var_agidl_i_db3: f64,
        var_agidl_i_db4: f64,
        var_agidl_i_dn0: f64,
        var_agidl_i_dn1: f64,
        var_agidl_i_dn2: f64,
        var_agidl_i_dn3: f64,
        var_agidl_i_dn4: f64,
        var_agidl_i_dn5: f64,
        var_agidl_i_dn6: f64,
        var_agidl_i_dn7: f64,
        var_agidl_i_dn8: f64,
        var_agisl_i: f64,
        var_agisl_i_db0: f64,
        var_agisl_i_db1: f64,
        var_agisl_i_db2: f64,
        var_agisl_i_db3: f64,
        var_agisl_i_db4: f64,
        var_agisl_i_dn0: f64,
        var_agisl_i_dn1: f64,
        var_agisl_i_dn2: f64,
        var_agisl_i_dn3: f64,
        var_agisl_i_dn4: f64,
        var_agisl_i_dn5: f64,
        var_agisl_i_dn6: f64,
        var_agisl_i_dn7: f64,
        var_agisl_i_dn8: f64,
        var_bgidl_t: f64,
        var_bgidl_t_db0: f64,
        var_bgidl_t_db1: f64,
        var_bgidl_t_db2: f64,
        var_bgidl_t_db3: f64,
        var_bgidl_t_db4: f64,
        var_bgidl_t_dn0: f64,
        var_bgidl_t_dn1: f64,
        var_bgidl_t_dn2: f64,
        var_bgidl_t_dn3: f64,
        var_bgidl_t_dn4: f64,
        var_bgidl_t_dn5: f64,
        var_bgidl_t_dn6: f64,
        var_bgidl_t_dn7: f64,
        var_bgidl_t_dn8: f64,
        var_bgisl_t: f64,
        var_bgisl_t_db0: f64,
        var_bgisl_t_db1: f64,
        var_bgisl_t_db2: f64,
        var_bgisl_t_db3: f64,
        var_bgisl_t_db4: f64,
        var_bgisl_t_dn0: f64,
        var_bgisl_t_dn1: f64,
        var_bgisl_t_dn2: f64,
        var_bgisl_t_dn3: f64,
        var_bgisl_t_dn4: f64,
        var_bgisl_t_dn5: f64,
        var_bgisl_t_dn6: f64,
        var_bgisl_t_dn7: f64,
        var_bgisl_t_dn8: f64,
        var_egisl_i: f64,
        var_egisl_i_db0: f64,
        var_egisl_i_db1: f64,
        var_egisl_i_db2: f64,
        var_egisl_i_db3: f64,
        var_egisl_i_db4: f64,
        var_egisl_i_dn0: f64,
        var_egisl_i_dn1: f64,
        var_egisl_i_dn2: f64,
        var_egisl_i_dn3: f64,
        var_egisl_i_dn4: f64,
        var_egisl_i_dn5: f64,
        var_egisl_i_dn6: f64,
        var_egisl_i_dn7: f64,
        var_egisl_i_dn8: f64,
        var_gamma0: f64,
        var_gamma0_db0: f64,
        var_gamma0_db1: f64,
        var_gamma0_db2: f64,
        var_gamma0_db3: f64,
        var_gamma0_db4: f64,
        var_gamma0_dn0: f64,
        var_gamma0_dn1: f64,
        var_gamma0_dn2: f64,
        var_gamma0_dn3: f64,
        var_gamma0_dn4: f64,
        var_gamma0_dn5: f64,
        var_gamma0_dn6: f64,
        var_gamma0_dn7: f64,
        var_gamma0_dn8: f64,
        var_guard126: f64,
        var_guard127: f64,
        var_pgidl_i: f64,
        var_pgidl_i_db0: f64,
        var_pgidl_i_db1: f64,
        var_pgidl_i_db2: f64,
        var_pgidl_i_db3: f64,
        var_pgidl_i_db4: f64,
        var_pgidl_i_dn0: f64,
        var_pgidl_i_dn1: f64,
        var_pgidl_i_dn2: f64,
        var_pgidl_i_dn3: f64,
        var_pgidl_i_dn4: f64,
        var_pgidl_i_dn5: f64,
        var_pgidl_i_dn6: f64,
        var_pgidl_i_dn7: f64,
        var_pgidl_i_dn8: f64,
        var_pgisl_i: f64,
        var_pgisl_i_db0: f64,
        var_pgisl_i_db1: f64,
        var_pgisl_i_db2: f64,
        var_pgisl_i_db3: f64,
        var_pgisl_i_db4: f64,
        var_pgisl_i_dn0: f64,
        var_pgisl_i_dn1: f64,
        var_pgisl_i_dn2: f64,
        var_pgisl_i_dn3: f64,
        var_pgisl_i_dn4: f64,
        var_pgisl_i_dn5: f64,
        var_pgisl_i_dn6: f64,
        var_pgisl_i_dn7: f64,
        var_pgisl_i_dn8: f64,
        var_t0: f64,
        var_t0_db0: f64,
        var_t0_db1: f64,
        var_t0_db2: f64,
        var_t0_db3: f64,
        var_t0_db4: f64,
        var_t0_dn0: f64,
        var_t0_dn1: f64,
        var_t0_dn2: f64,
        var_t0_dn3: f64,
        var_t0_dn4: f64,
        var_t0_dn5: f64,
        var_t0_dn6: f64,
        var_t0_dn7: f64,
        var_t0_dn8: f64,
        var_utotal: f64,
        var_utotal_db0: f64,
        var_utotal_db1: f64,
        var_utotal_db2: f64,
        var_utotal_db3: f64,
        var_utotal_db4: f64,
        var_utotal_dn0: f64,
        var_utotal_dn1: f64,
        var_utotal_dn2: f64,
        var_utotal_dn3: f64,
        var_utotal_dn4: f64,
        var_utotal_dn5: f64,
        var_utotal_dn6: f64,
        var_utotal_dn7: f64,
        var_utotal_dn8: f64,
        var_vbegisl_i: f64,
        var_vbegisl_i_db0: f64,
        var_vbegisl_i_db1: f64,
        var_vbegisl_i_db2: f64,
        var_vbegisl_i_db3: f64,
        var_vbegisl_i_db4: f64,
        var_vbegisl_i_dn0: f64,
        var_vbegisl_i_dn1: f64,
        var_vbegisl_i_dn2: f64,
        var_vbegisl_i_dn3: f64,
        var_vbegisl_i_dn4: f64,
        var_vbegisl_i_dn5: f64,
        var_vbegisl_i_dn6: f64,
        var_vbegisl_i_dn7: f64,
        var_vbegisl_i_dn8: f64,
        var_vbgisl_i: f64,
        var_vbgisl_i_db0: f64,
        var_vbgisl_i_db1: f64,
        var_vbgisl_i_db2: f64,
        var_vbgisl_i_db3: f64,
        var_vbgisl_i_db4: f64,
        var_vbgisl_i_dn0: f64,
        var_vbgisl_i_dn1: f64,
        var_vbgisl_i_dn2: f64,
        var_vbgisl_i_dn3: f64,
        var_vbgisl_i_dn4: f64,
        var_vbgisl_i_dn5: f64,
        var_vbgisl_i_dn6: f64,
        var_vbgisl_i_dn7: f64,
        var_vbgisl_i_dn8: f64,
        var_vbgs: f64,
        var_vbgs_db0: f64,
        var_vbgs_db1: f64,
        var_vbgs_db2: f64,
        var_vbgs_db3: f64,
        var_vbgs_db4: f64,
        var_vbgs_dn0: f64,
        var_vbgs_dn1: f64,
        var_vbgs_dn2: f64,
        var_vbgs_dn3: f64,
        var_vbgs_dn4: f64,
        var_vbgs_dn5: f64,
        var_vbgs_dn6: f64,
        var_vbgs_dn7: f64,
        var_vbgs_dn8: f64,
        var_vds_noswap: f64,
        var_vds_noswap_db0: f64,
        var_vds_noswap_db1: f64,
        var_vds_noswap_db2: f64,
        var_vds_noswap_db3: f64,
        var_vds_noswap_db4: f64,
        var_vds_noswap_dn0: f64,
        var_vds_noswap_dn1: f64,
        var_vds_noswap_dn2: f64,
        var_vds_noswap_dn3: f64,
        var_vds_noswap_dn4: f64,
        var_vds_noswap_dn5: f64,
        var_vds_noswap_dn6: f64,
        var_vds_noswap_dn7: f64,
        var_vds_noswap_dn8: f64,
        var_vfbsd: f64,
        var_vfbsd_bg: f64,
        var_vfbsd_bg_db0: f64,
        var_vfbsd_bg_db1: f64,
        var_vfbsd_bg_db2: f64,
        var_vfbsd_bg_db3: f64,
        var_vfbsd_bg_db4: f64,
        var_vfbsd_bg_dn0: f64,
        var_vfbsd_bg_dn1: f64,
        var_vfbsd_bg_dn2: f64,
        var_vfbsd_bg_dn3: f64,
        var_vfbsd_bg_dn4: f64,
        var_vfbsd_bg_dn5: f64,
        var_vfbsd_bg_dn6: f64,
        var_vfbsd_bg_dn7: f64,
        var_vfbsd_bg_dn8: f64,
        var_vfbsd_db0: f64,
        var_vfbsd_db1: f64,
        var_vfbsd_db2: f64,
        var_vfbsd_db3: f64,
        var_vfbsd_db4: f64,
        var_vfbsd_dn0: f64,
        var_vfbsd_dn1: f64,
        var_vfbsd_dn2: f64,
        var_vfbsd_dn3: f64,
        var_vfbsd_dn4: f64,
        var_vfbsd_dn5: f64,
        var_vfbsd_dn6: f64,
        var_vfbsd_dn7: f64,
        var_vfbsd_dn8: f64,
        var_vgs_noswap: f64,
        var_vgs_noswap_db0: f64,
        var_vgs_noswap_db1: f64,
        var_vgs_noswap_db2: f64,
        var_vgs_noswap_db3: f64,
        var_vgs_noswap_db4: f64,
        var_vgs_noswap_dn0: f64,
        var_vgs_noswap_dn1: f64,
        var_vgs_noswap_dn2: f64,
        var_vgs_noswap_dn3: f64,
        var_vgs_noswap_dn4: f64,
        var_vgs_noswap_dn5: f64,
        var_vgs_noswap_dn6: f64,
        var_vgs_noswap_dn7: f64,
        var_vgs_noswap_dn8: f64,
        var_vsat_t: f64,
        var_vsat_t_db0: f64,
        var_vsat_t_db1: f64,
        var_vsat_t_db2: f64,
        var_vsat_t_db3: f64,
        var_vsat_t_db4: f64,
        var_vsat_t_dn0: f64,
        var_vsat_t_dn1: f64,
        var_vsat_t_dn2: f64,
        var_vsat_t_dn3: f64,
        var_vsat_t_dn4: f64,
        var_vsat_t_dn5: f64,
        var_vsat_t_dn6: f64,
        var_vsat_t_dn7: f64,
        var_vsat_t_dn8: f64,
        var_weff: f64,
        var_weff_db0: f64,
        var_weff_db1: f64,
        var_weff_db2: f64,
        var_weff_db3: f64,
        var_weff_db4: f64,
        var_weff_dn0: f64,
        var_weff_dn1: f64,
        var_weff_dn2: f64,
        var_weff_dn3: f64,
        var_weff_dn4: f64,
        var_weff_dn5: f64,
        var_weff_dn6: f64,
        var_weff_dn7: f64,
        var_weff_dn8: f64,
        var_esatnoi_slot: &mut f64,
        var_esatnoi_db0_slot: &mut f64,
        var_esatnoi_db1_slot: &mut f64,
        var_esatnoi_db2_slot: &mut f64,
        var_esatnoi_db3_slot: &mut f64,
        var_esatnoi_db4_slot: &mut f64,
        var_esatnoi_dn0_slot: &mut f64,
        var_esatnoi_dn1_slot: &mut f64,
        var_esatnoi_dn2_slot: &mut f64,
        var_esatnoi_dn3_slot: &mut f64,
        var_esatnoi_dn4_slot: &mut f64,
        var_esatnoi_dn5_slot: &mut f64,
        var_esatnoi_dn6_slot: &mut f64,
        var_esatnoi_dn7_slot: &mut f64,
        var_esatnoi_dn8_slot: &mut f64,
        var_esatnoi_rdb0_slot: &mut f64,
        var_esatnoi_rdb1_slot: &mut f64,
        var_esatnoi_rdb2_slot: &mut f64,
        var_esatnoi_rdb3_slot: &mut f64,
        var_esatnoi_rdb4_slot: &mut f64,
        var_esatnoi_rdn0_slot: &mut f64,
        var_esatnoi_rdn1_slot: &mut f64,
        var_esatnoi_rdn2_slot: &mut f64,
        var_esatnoi_rdn3_slot: &mut f64,
        var_esatnoi_rdn4_slot: &mut f64,
        var_esatnoi_rdn5_slot: &mut f64,
        var_esatnoi_rdn6_slot: &mut f64,
        var_esatnoi_rdn7_slot: &mut f64,
        var_esatnoi_rdn8_slot: &mut f64,
        var_esatnoi_rv_slot: &mut f64,
        var_guard129_slot: &mut f64,
        var_guard129_db0_slot: &mut f64,
        var_guard129_db1_slot: &mut f64,
        var_guard129_db2_slot: &mut f64,
        var_guard129_db3_slot: &mut f64,
        var_guard129_db4_slot: &mut f64,
        var_guard129_dn0_slot: &mut f64,
        var_guard129_dn1_slot: &mut f64,
        var_guard129_dn2_slot: &mut f64,
        var_guard129_dn3_slot: &mut f64,
        var_guard129_dn4_slot: &mut f64,
        var_guard129_dn5_slot: &mut f64,
        var_guard129_dn6_slot: &mut f64,
        var_guard129_dn7_slot: &mut f64,
        var_guard129_dn8_slot: &mut f64,
        var_guard129_rdb0_slot: &mut f64,
        var_guard129_rdb1_slot: &mut f64,
        var_guard129_rdb2_slot: &mut f64,
        var_guard129_rdb3_slot: &mut f64,
        var_guard129_rdb4_slot: &mut f64,
        var_guard129_rdn0_slot: &mut f64,
        var_guard129_rdn1_slot: &mut f64,
        var_guard129_rdn2_slot: &mut f64,
        var_guard129_rdn3_slot: &mut f64,
        var_guard129_rdn4_slot: &mut f64,
        var_guard129_rdn5_slot: &mut f64,
        var_guard129_rdn6_slot: &mut f64,
        var_guard129_rdn7_slot: &mut f64,
        var_guard129_rdn8_slot: &mut f64,
        var_guard129_rv_slot: &mut f64,
        var_guard131_slot: &mut f64,
        var_guard131_db0_slot: &mut f64,
        var_guard131_db1_slot: &mut f64,
        var_guard131_db2_slot: &mut f64,
        var_guard131_db3_slot: &mut f64,
        var_guard131_db4_slot: &mut f64,
        var_guard131_dn0_slot: &mut f64,
        var_guard131_dn1_slot: &mut f64,
        var_guard131_dn2_slot: &mut f64,
        var_guard131_dn3_slot: &mut f64,
        var_guard131_dn4_slot: &mut f64,
        var_guard131_dn5_slot: &mut f64,
        var_guard131_dn6_slot: &mut f64,
        var_guard131_dn7_slot: &mut f64,
        var_guard131_dn8_slot: &mut f64,
        var_guard131_rdb0_slot: &mut f64,
        var_guard131_rdb1_slot: &mut f64,
        var_guard131_rdb2_slot: &mut f64,
        var_guard131_rdb3_slot: &mut f64,
        var_guard131_rdb4_slot: &mut f64,
        var_guard131_rdn0_slot: &mut f64,
        var_guard131_rdn1_slot: &mut f64,
        var_guard131_rdn2_slot: &mut f64,
        var_guard131_rdn3_slot: &mut f64,
        var_guard131_rdn4_slot: &mut f64,
        var_guard131_rdn5_slot: &mut f64,
        var_guard131_rdn6_slot: &mut f64,
        var_guard131_rdn7_slot: &mut f64,
        var_guard131_rdn8_slot: &mut f64,
        var_guard131_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_db0_slot: &mut f64,
        var_t1_db1_slot: &mut f64,
        var_t1_db2_slot: &mut f64,
        var_t1_db3_slot: &mut f64,
        var_t1_db4_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rdb0_slot: &mut f64,
        var_t1_rdb1_slot: &mut f64,
        var_t1_rdb2_slot: &mut f64,
        var_t1_rdb3_slot: &mut f64,
        var_t1_rdb4_slot: &mut f64,
        var_t1_rdn0_slot: &mut f64,
        var_t1_rdn1_slot: &mut f64,
        var_t1_rdn2_slot: &mut f64,
        var_t1_rdn3_slot: &mut f64,
        var_t1_rdn4_slot: &mut f64,
        var_t1_rdn5_slot: &mut f64,
        var_t1_rdn6_slot: &mut f64,
        var_t1_rdn7_slot: &mut f64,
        var_t1_rdn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_db0_slot: &mut f64,
        var_t2_db1_slot: &mut f64,
        var_t2_db2_slot: &mut f64,
        var_t2_db3_slot: &mut f64,
        var_t2_db4_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rdb0_slot: &mut f64,
        var_t2_rdb1_slot: &mut f64,
        var_t2_rdb2_slot: &mut f64,
        var_t2_rdb3_slot: &mut f64,
        var_t2_rdb4_slot: &mut f64,
        var_t2_rdn0_slot: &mut f64,
        var_t2_rdn1_slot: &mut f64,
        var_t2_rdn2_slot: &mut f64,
        var_t2_rdn3_slot: &mut f64,
        var_t2_rdn4_slot: &mut f64,
        var_t2_rdn5_slot: &mut f64,
        var_t2_rdn6_slot: &mut f64,
        var_t2_rdn7_slot: &mut f64,
        var_t2_rdn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_db0_slot: &mut f64,
        var_t3_db1_slot: &mut f64,
        var_t3_db2_slot: &mut f64,
        var_t3_db3_slot: &mut f64,
        var_t3_db4_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn1_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rdb0_slot: &mut f64,
        var_t3_rdb1_slot: &mut f64,
        var_t3_rdb2_slot: &mut f64,
        var_t3_rdb3_slot: &mut f64,
        var_t3_rdb4_slot: &mut f64,
        var_t3_rdn0_slot: &mut f64,
        var_t3_rdn1_slot: &mut f64,
        var_t3_rdn2_slot: &mut f64,
        var_t3_rdn3_slot: &mut f64,
        var_t3_rdn4_slot: &mut f64,
        var_t3_rdn5_slot: &mut f64,
        var_t3_rdn6_slot: &mut f64,
        var_t3_rdn7_slot: &mut f64,
        var_t3_rdn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_db0_slot: &mut f64,
        var_t6_db1_slot: &mut f64,
        var_t6_db2_slot: &mut f64,
        var_t6_db3_slot: &mut f64,
        var_t6_db4_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn1_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_rdb0_slot: &mut f64,
        var_t6_rdb1_slot: &mut f64,
        var_t6_rdb2_slot: &mut f64,
        var_t6_rdb3_slot: &mut f64,
        var_t6_rdb4_slot: &mut f64,
        var_t6_rdn0_slot: &mut f64,
        var_t6_rdn1_slot: &mut f64,
        var_t6_rdn2_slot: &mut f64,
        var_t6_rdn3_slot: &mut f64,
        var_t6_rdn4_slot: &mut f64,
        var_t6_rdn5_slot: &mut f64,
        var_t6_rdn6_slot: &mut f64,
        var_t6_rdn7_slot: &mut f64,
        var_t6_rdn8_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
    ) {
        let mut var_esatnoi: f64 = *var_esatnoi_slot;
        let mut var_esatnoi_db0: f64 = *var_esatnoi_db0_slot;
        let mut var_esatnoi_db1: f64 = *var_esatnoi_db1_slot;
        let mut var_esatnoi_db2: f64 = *var_esatnoi_db2_slot;
        let mut var_esatnoi_db3: f64 = *var_esatnoi_db3_slot;
        let mut var_esatnoi_db4: f64 = *var_esatnoi_db4_slot;
        let mut var_esatnoi_dn0: f64 = *var_esatnoi_dn0_slot;
        let mut var_esatnoi_dn1: f64 = *var_esatnoi_dn1_slot;
        let mut var_esatnoi_dn2: f64 = *var_esatnoi_dn2_slot;
        let mut var_esatnoi_dn3: f64 = *var_esatnoi_dn3_slot;
        let mut var_esatnoi_dn4: f64 = *var_esatnoi_dn4_slot;
        let mut var_esatnoi_dn5: f64 = *var_esatnoi_dn5_slot;
        let mut var_esatnoi_dn6: f64 = *var_esatnoi_dn6_slot;
        let mut var_esatnoi_dn7: f64 = *var_esatnoi_dn7_slot;
        let mut var_esatnoi_dn8: f64 = *var_esatnoi_dn8_slot;
        let mut var_esatnoi_rdb0: f64 = *var_esatnoi_rdb0_slot;
        let mut var_esatnoi_rdb1: f64 = *var_esatnoi_rdb1_slot;
        let mut var_esatnoi_rdb2: f64 = *var_esatnoi_rdb2_slot;
        let mut var_esatnoi_rdb3: f64 = *var_esatnoi_rdb3_slot;
        let mut var_esatnoi_rdb4: f64 = *var_esatnoi_rdb4_slot;
        let mut var_esatnoi_rdn0: f64 = *var_esatnoi_rdn0_slot;
        let mut var_esatnoi_rdn1: f64 = *var_esatnoi_rdn1_slot;
        let mut var_esatnoi_rdn2: f64 = *var_esatnoi_rdn2_slot;
        let mut var_esatnoi_rdn3: f64 = *var_esatnoi_rdn3_slot;
        let mut var_esatnoi_rdn4: f64 = *var_esatnoi_rdn4_slot;
        let mut var_esatnoi_rdn5: f64 = *var_esatnoi_rdn5_slot;
        let mut var_esatnoi_rdn6: f64 = *var_esatnoi_rdn6_slot;
        let mut var_esatnoi_rdn7: f64 = *var_esatnoi_rdn7_slot;
        let mut var_esatnoi_rdn8: f64 = *var_esatnoi_rdn8_slot;
        let mut var_esatnoi_rv: f64 = *var_esatnoi_rv_slot;
        let mut var_guard129: f64 = *var_guard129_slot;
        let mut var_guard129_db0: f64 = *var_guard129_db0_slot;
        let mut var_guard129_db1: f64 = *var_guard129_db1_slot;
        let mut var_guard129_db2: f64 = *var_guard129_db2_slot;
        let mut var_guard129_db3: f64 = *var_guard129_db3_slot;
        let mut var_guard129_db4: f64 = *var_guard129_db4_slot;
        let mut var_guard129_dn0: f64 = *var_guard129_dn0_slot;
        let mut var_guard129_dn1: f64 = *var_guard129_dn1_slot;
        let mut var_guard129_dn2: f64 = *var_guard129_dn2_slot;
        let mut var_guard129_dn3: f64 = *var_guard129_dn3_slot;
        let mut var_guard129_dn4: f64 = *var_guard129_dn4_slot;
        let mut var_guard129_dn5: f64 = *var_guard129_dn5_slot;
        let mut var_guard129_dn6: f64 = *var_guard129_dn6_slot;
        let mut var_guard129_dn7: f64 = *var_guard129_dn7_slot;
        let mut var_guard129_dn8: f64 = *var_guard129_dn8_slot;
        let mut var_guard129_rdb0: f64 = *var_guard129_rdb0_slot;
        let mut var_guard129_rdb1: f64 = *var_guard129_rdb1_slot;
        let mut var_guard129_rdb2: f64 = *var_guard129_rdb2_slot;
        let mut var_guard129_rdb3: f64 = *var_guard129_rdb3_slot;
        let mut var_guard129_rdb4: f64 = *var_guard129_rdb4_slot;
        let mut var_guard129_rdn0: f64 = *var_guard129_rdn0_slot;
        let mut var_guard129_rdn1: f64 = *var_guard129_rdn1_slot;
        let mut var_guard129_rdn2: f64 = *var_guard129_rdn2_slot;
        let mut var_guard129_rdn3: f64 = *var_guard129_rdn3_slot;
        let mut var_guard129_rdn4: f64 = *var_guard129_rdn4_slot;
        let mut var_guard129_rdn5: f64 = *var_guard129_rdn5_slot;
        let mut var_guard129_rdn6: f64 = *var_guard129_rdn6_slot;
        let mut var_guard129_rdn7: f64 = *var_guard129_rdn7_slot;
        let mut var_guard129_rdn8: f64 = *var_guard129_rdn8_slot;
        let mut var_guard129_rv: f64 = *var_guard129_rv_slot;
        let mut var_guard131: f64 = *var_guard131_slot;
        let mut var_guard131_db0: f64 = *var_guard131_db0_slot;
        let mut var_guard131_db1: f64 = *var_guard131_db1_slot;
        let mut var_guard131_db2: f64 = *var_guard131_db2_slot;
        let mut var_guard131_db3: f64 = *var_guard131_db3_slot;
        let mut var_guard131_db4: f64 = *var_guard131_db4_slot;
        let mut var_guard131_dn0: f64 = *var_guard131_dn0_slot;
        let mut var_guard131_dn1: f64 = *var_guard131_dn1_slot;
        let mut var_guard131_dn2: f64 = *var_guard131_dn2_slot;
        let mut var_guard131_dn3: f64 = *var_guard131_dn3_slot;
        let mut var_guard131_dn4: f64 = *var_guard131_dn4_slot;
        let mut var_guard131_dn5: f64 = *var_guard131_dn5_slot;
        let mut var_guard131_dn6: f64 = *var_guard131_dn6_slot;
        let mut var_guard131_dn7: f64 = *var_guard131_dn7_slot;
        let mut var_guard131_dn8: f64 = *var_guard131_dn8_slot;
        let mut var_guard131_rdb0: f64 = *var_guard131_rdb0_slot;
        let mut var_guard131_rdb1: f64 = *var_guard131_rdb1_slot;
        let mut var_guard131_rdb2: f64 = *var_guard131_rdb2_slot;
        let mut var_guard131_rdb3: f64 = *var_guard131_rdb3_slot;
        let mut var_guard131_rdb4: f64 = *var_guard131_rdb4_slot;
        let mut var_guard131_rdn0: f64 = *var_guard131_rdn0_slot;
        let mut var_guard131_rdn1: f64 = *var_guard131_rdn1_slot;
        let mut var_guard131_rdn2: f64 = *var_guard131_rdn2_slot;
        let mut var_guard131_rdn3: f64 = *var_guard131_rdn3_slot;
        let mut var_guard131_rdn4: f64 = *var_guard131_rdn4_slot;
        let mut var_guard131_rdn5: f64 = *var_guard131_rdn5_slot;
        let mut var_guard131_rdn6: f64 = *var_guard131_rdn6_slot;
        let mut var_guard131_rdn7: f64 = *var_guard131_rdn7_slot;
        let mut var_guard131_rdn8: f64 = *var_guard131_rdn8_slot;
        let mut var_guard131_rv: f64 = *var_guard131_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_db0: f64 = *var_t1_db0_slot;
        let mut var_t1_db1: f64 = *var_t1_db1_slot;
        let mut var_t1_db2: f64 = *var_t1_db2_slot;
        let mut var_t1_db3: f64 = *var_t1_db3_slot;
        let mut var_t1_db4: f64 = *var_t1_db4_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rdb0: f64 = *var_t1_rdb0_slot;
        let mut var_t1_rdb1: f64 = *var_t1_rdb1_slot;
        let mut var_t1_rdb2: f64 = *var_t1_rdb2_slot;
        let mut var_t1_rdb3: f64 = *var_t1_rdb3_slot;
        let mut var_t1_rdb4: f64 = *var_t1_rdb4_slot;
        let mut var_t1_rdn0: f64 = *var_t1_rdn0_slot;
        let mut var_t1_rdn1: f64 = *var_t1_rdn1_slot;
        let mut var_t1_rdn2: f64 = *var_t1_rdn2_slot;
        let mut var_t1_rdn3: f64 = *var_t1_rdn3_slot;
        let mut var_t1_rdn4: f64 = *var_t1_rdn4_slot;
        let mut var_t1_rdn5: f64 = *var_t1_rdn5_slot;
        let mut var_t1_rdn6: f64 = *var_t1_rdn6_slot;
        let mut var_t1_rdn7: f64 = *var_t1_rdn7_slot;
        let mut var_t1_rdn8: f64 = *var_t1_rdn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_db0: f64 = *var_t2_db0_slot;
        let mut var_t2_db1: f64 = *var_t2_db1_slot;
        let mut var_t2_db2: f64 = *var_t2_db2_slot;
        let mut var_t2_db3: f64 = *var_t2_db3_slot;
        let mut var_t2_db4: f64 = *var_t2_db4_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rdb0: f64 = *var_t2_rdb0_slot;
        let mut var_t2_rdb1: f64 = *var_t2_rdb1_slot;
        let mut var_t2_rdb2: f64 = *var_t2_rdb2_slot;
        let mut var_t2_rdb3: f64 = *var_t2_rdb3_slot;
        let mut var_t2_rdb4: f64 = *var_t2_rdb4_slot;
        let mut var_t2_rdn0: f64 = *var_t2_rdn0_slot;
        let mut var_t2_rdn1: f64 = *var_t2_rdn1_slot;
        let mut var_t2_rdn2: f64 = *var_t2_rdn2_slot;
        let mut var_t2_rdn3: f64 = *var_t2_rdn3_slot;
        let mut var_t2_rdn4: f64 = *var_t2_rdn4_slot;
        let mut var_t2_rdn5: f64 = *var_t2_rdn5_slot;
        let mut var_t2_rdn6: f64 = *var_t2_rdn6_slot;
        let mut var_t2_rdn7: f64 = *var_t2_rdn7_slot;
        let mut var_t2_rdn8: f64 = *var_t2_rdn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_db0: f64 = *var_t3_db0_slot;
        let mut var_t3_db1: f64 = *var_t3_db1_slot;
        let mut var_t3_db2: f64 = *var_t3_db2_slot;
        let mut var_t3_db3: f64 = *var_t3_db3_slot;
        let mut var_t3_db4: f64 = *var_t3_db4_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn1: f64 = *var_t3_dn1_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rdb0: f64 = *var_t3_rdb0_slot;
        let mut var_t3_rdb1: f64 = *var_t3_rdb1_slot;
        let mut var_t3_rdb2: f64 = *var_t3_rdb2_slot;
        let mut var_t3_rdb3: f64 = *var_t3_rdb3_slot;
        let mut var_t3_rdb4: f64 = *var_t3_rdb4_slot;
        let mut var_t3_rdn0: f64 = *var_t3_rdn0_slot;
        let mut var_t3_rdn1: f64 = *var_t3_rdn1_slot;
        let mut var_t3_rdn2: f64 = *var_t3_rdn2_slot;
        let mut var_t3_rdn3: f64 = *var_t3_rdn3_slot;
        let mut var_t3_rdn4: f64 = *var_t3_rdn4_slot;
        let mut var_t3_rdn5: f64 = *var_t3_rdn5_slot;
        let mut var_t3_rdn6: f64 = *var_t3_rdn6_slot;
        let mut var_t3_rdn7: f64 = *var_t3_rdn7_slot;
        let mut var_t3_rdn8: f64 = *var_t3_rdn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_db0: f64 = *var_t6_db0_slot;
        let mut var_t6_db1: f64 = *var_t6_db1_slot;
        let mut var_t6_db2: f64 = *var_t6_db2_slot;
        let mut var_t6_db3: f64 = *var_t6_db3_slot;
        let mut var_t6_db4: f64 = *var_t6_db4_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn1: f64 = *var_t6_dn1_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_rdb0: f64 = *var_t6_rdb0_slot;
        let mut var_t6_rdb1: f64 = *var_t6_rdb1_slot;
        let mut var_t6_rdb2: f64 = *var_t6_rdb2_slot;
        let mut var_t6_rdb3: f64 = *var_t6_rdb3_slot;
        let mut var_t6_rdb4: f64 = *var_t6_rdb4_slot;
        let mut var_t6_rdn0: f64 = *var_t6_rdn0_slot;
        let mut var_t6_rdn1: f64 = *var_t6_rdn1_slot;
        let mut var_t6_rdn2: f64 = *var_t6_rdn2_slot;
        let mut var_t6_rdn3: f64 = *var_t6_rdn3_slot;
        let mut var_t6_rdn4: f64 = *var_t6_rdn4_slot;
        let mut var_t6_rdn5: f64 = *var_t6_rdn5_slot;
        let mut var_t6_rdn6: f64 = *var_t6_rdn6_slot;
        let mut var_t6_rdn7: f64 = *var_t6_rdn7_slot;
        let mut var_t6_rdn8: f64 = *var_t6_rdn8_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;

        let (assign12520_e11620, assign12520_e11620_d_n0, assign12520_e11620_d_n1, assign12520_e11620_d_n2, assign12520_e11620_d_n3, assign12520_e11620_d_n4, assign12520_e11620_d_n5, assign12520_e11620_d_n6, assign12520_e11620_d_n7, assign12520_e11620_d_n8, assign12520_e11620_d_b0, assign12520_e11620_d_b1, assign12520_e11620_d_b2, assign12520_e11620_d_b3, assign12520_e11620_d_b4,) = {
    if ((var_guard126 != 0.0) && (var_guard127 == 0.0)) {
        let assign12520_e11617: f64 = (var_t1 + 0.001);
        let assign12520_e11618: f64 = (var_bgidl_t / assign12520_e11617);
        (assign12520_e11618, (((var_bgidl_t_dn0 * assign12520_e11617) - (var_bgidl_t * var_t1_dn0)) / (assign12520_e11617 * assign12520_e11617)), (((var_bgidl_t_dn1 * assign12520_e11617) - (var_bgidl_t * var_t1_dn1)) / (assign12520_e11617 * assign12520_e11617)), (((var_bgidl_t_dn2 * assign12520_e11617) - (var_bgidl_t * var_t1_dn2)) / (assign12520_e11617 * assign12520_e11617)), (((var_bgidl_t_dn3 * assign12520_e11617) - (var_bgidl_t * var_t1_dn3)) / (assign12520_e11617 * assign12520_e11617)), (((var_bgidl_t_dn4 * assign12520_e11617) - (var_bgidl_t * var_t1_dn4)) / (assign12520_e11617 * assign12520_e11617)), (((var_bgidl_t_dn5 * assign12520_e11617) - (var_bgidl_t * var_t1_dn5)) / (assign12520_e11617 * assign12520_e11617)), (((var_bgidl_t_dn6 * assign12520_e11617) - (var_bgidl_t * var_t1_dn6)) / (assign12520_e11617 * assign12520_e11617)), (((var_bgidl_t_dn7 * assign12520_e11617) - (var_bgidl_t * var_t1_dn7)) / (assign12520_e11617 * assign12520_e11617)), (((var_bgidl_t_dn8 * assign12520_e11617) - (var_bgidl_t * var_t1_dn8)) / (assign12520_e11617 * assign12520_e11617)), (((var_bgidl_t_db0 * assign12520_e11617) - (var_bgidl_t * var_t1_db0)) / (assign12520_e11617 * assign12520_e11617)), (((var_bgidl_t_db1 * assign12520_e11617) - (var_bgidl_t * var_t1_db1)) / (assign12520_e11617 * assign12520_e11617)), (((var_bgidl_t_db2 * assign12520_e11617) - (var_bgidl_t * var_t1_db2)) / (assign12520_e11617 * assign12520_e11617)), (((var_bgidl_t_db3 * assign12520_e11617) - (var_bgidl_t * var_t1_db3)) / (assign12520_e11617 * assign12520_e11617)), (((var_bgidl_t_db4 * assign12520_e11617) - (var_bgidl_t * var_t1_db4)) / (assign12520_e11617 * assign12520_e11617)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_db0, var_t2_db1, var_t2_db2, var_t2_db3, var_t2_db4,)
    }
};
        var_t2 = assign12520_e11620;
        var_t2_dn0 = assign12520_e11620_d_n0;
        var_t2_dn1 = assign12520_e11620_d_n1;
        var_t2_dn2 = assign12520_e11620_d_n2;
        var_t2_dn3 = assign12520_e11620_d_n3;
        var_t2_dn4 = assign12520_e11620_d_n4;
        var_t2_dn5 = assign12520_e11620_d_n5;
        var_t2_dn6 = assign12520_e11620_d_n6;
        var_t2_dn7 = assign12520_e11620_d_n7;
        var_t2_dn8 = assign12520_e11620_d_n8;
        var_t2_db0 = assign12520_e11620_d_b0;
        var_t2_db1 = assign12520_e11620_d_b1;
        var_t2_db2 = assign12520_e11620_d_b2;
        var_t2_db3 = assign12520_e11620_d_b3;
        var_t2_db4 = assign12520_e11620_d_b4;
        var_t2_rv = 0.0;
        var_t2_rdn0 = 0.0;
        var_t2_rdn1 = 0.0;
        var_t2_rdn2 = 0.0;
        var_t2_rdn3 = 0.0;
        var_t2_rdn4 = 0.0;
        var_t2_rdn5 = 0.0;
        var_t2_rdn6 = 0.0;
        var_t2_rdn7 = 0.0;
        var_t2_rdn8 = 0.0;
        var_t2_rdb0 = 0.0;
        var_t2_rdb1 = 0.0;
        var_t2_rdb2 = 0.0;
        var_t2_rdb3 = 0.0;
        var_t2_rdb4 = 0.0;

        let (assign12530_e11633, assign12530_e11633_d_n0, assign12530_e11633_d_n1, assign12530_e11633_d_n2, assign12530_e11633_d_n3, assign12530_e11633_d_n4, assign12530_e11633_d_n5, assign12530_e11633_d_n6, assign12530_e11633_d_n7, assign12530_e11633_d_n8, assign12530_e11633_d_b0, assign12530_e11633_d_b1, assign12530_e11633_d_b2, assign12530_e11633_d_b3, assign12530_e11633_d_b4,) = {
    if ((var_guard126 != 0.0) && (var_guard127 == 0.0)) {
        let assign12530_e11628: f64 = (var_t1).max(1e-38);
        let assign12530_e11629: f64 = (assign12530_e11628).ln();
        let assign12530_e11630: f64 = (var_pgidl_i * assign12530_e11629);
        let assign12530_e11631: f64 = { let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12530_e11631, ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgidl_i_dn0 * assign12530_e11629) + (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn0 } else { 0.0 } / assign12530_e11628)))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgidl_i_dn1 * assign12530_e11629) + (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn1 } else { 0.0 } / assign12530_e11628)))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgidl_i_dn2 * assign12530_e11629) + (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn2 } else { 0.0 } / assign12530_e11628)))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgidl_i_dn3 * assign12530_e11629) + (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn3 } else { 0.0 } / assign12530_e11628)))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgidl_i_dn4 * assign12530_e11629) + (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn4 } else { 0.0 } / assign12530_e11628)))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgidl_i_dn5 * assign12530_e11629) + (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn5 } else { 0.0 } / assign12530_e11628)))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgidl_i_dn6 * assign12530_e11629) + (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn6 } else { 0.0 } / assign12530_e11628)))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgidl_i_dn7 * assign12530_e11629) + (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn7 } else { 0.0 } / assign12530_e11628)))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgidl_i_dn8 * assign12530_e11629) + (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn8 } else { 0.0 } / assign12530_e11628)))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgidl_i_db0 * assign12530_e11629) + (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_db0 } else { 0.0 } / assign12530_e11628)))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgidl_i_db1 * assign12530_e11629) + (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_db1 } else { 0.0 } / assign12530_e11628)))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgidl_i_db2 * assign12530_e11629) + (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_db2 } else { 0.0 } / assign12530_e11628)))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgidl_i_db3 * assign12530_e11629) + (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_db3 } else { 0.0 } / assign12530_e11628)))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgidl_i_db4 * assign12530_e11629) + (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_db4 } else { 0.0 } / assign12530_e11628)))),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn1, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_db0, var_t3_db1, var_t3_db2, var_t3_db3, var_t3_db4,)
    }
};
        var_t3 = assign12530_e11633;
        var_t3_dn0 = assign12530_e11633_d_n0;
        var_t3_dn1 = assign12530_e11633_d_n1;
        var_t3_dn2 = assign12530_e11633_d_n2;
        var_t3_dn3 = assign12530_e11633_d_n3;
        var_t3_dn4 = assign12530_e11633_d_n4;
        var_t3_dn5 = assign12530_e11633_d_n5;
        var_t3_dn6 = assign12530_e11633_d_n6;
        var_t3_dn7 = assign12530_e11633_d_n7;
        var_t3_dn8 = assign12530_e11633_d_n8;
        var_t3_db0 = assign12530_e11633_d_b0;
        var_t3_db1 = assign12530_e11633_d_b1;
        var_t3_db2 = assign12530_e11633_d_b2;
        var_t3_db3 = assign12530_e11633_d_b3;
        var_t3_db4 = assign12530_e11633_d_b4;
        var_t3_rv = 0.0;
        var_t3_rdn0 = 0.0;
        var_t3_rdn1 = 0.0;
        var_t3_rdn2 = 0.0;
        var_t3_rdn3 = 0.0;
        var_t3_rdn4 = 0.0;
        var_t3_rdn5 = 0.0;
        var_t3_rdn6 = 0.0;
        var_t3_rdn7 = 0.0;
        var_t3_rdn8 = 0.0;
        var_t3_rdb0 = 0.0;
        var_t3_rdb1 = 0.0;
        var_t3_rdb2 = 0.0;
        var_t3_rdb3 = 0.0;
        var_t3_rdb4 = 0.0;

        let (assign12540_e11650, assign12540_e11650_d_n0, assign12540_e11650_d_n1, assign12540_e11650_d_n2, assign12540_e11650_d_n3, assign12540_e11650_d_n4, assign12540_e11650_d_n5, assign12540_e11650_d_n6, assign12540_e11650_d_n7, assign12540_e11650_d_n8, assign12540_e11650_d_b0, assign12540_e11650_d_b1, assign12540_e11650_d_b2, assign12540_e11650_d_b3, assign12540_e11650_d_b4,) = {
    if ((var_guard126 != 0.0) && (var_guard127 == 0.0)) {
        let assign12540_e11640: f64 = (var_agidl_i * var_weff);
        let assign12540_e11642: f64 = (assign12540_e11640 * var_t3);
        let assign12540_e11644: f64 = (-var_t2);
        let assign12540_e11645: f64 = { let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign12540_e11646: f64 = (assign12540_e11642 * assign12540_e11645);
        let assign12540_e11648: f64 = (assign12540_e11646 * var_vds_noswap);
        (assign12540_e11648, ((((((((var_agidl_i_dn0 * var_weff) + (var_agidl_i * var_weff_dn0)) * var_t3) + (assign12540_e11640 * var_t3_dn0)) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn0)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_dn0)), ((((((((var_agidl_i_dn1 * var_weff) + (var_agidl_i * var_weff_dn1)) * var_t3) + (assign12540_e11640 * var_t3_dn1)) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn1)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_dn1)), ((((((((var_agidl_i_dn2 * var_weff) + (var_agidl_i * var_weff_dn2)) * var_t3) + (assign12540_e11640 * var_t3_dn2)) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn2)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_dn2)), ((((((((var_agidl_i_dn3 * var_weff) + (var_agidl_i * var_weff_dn3)) * var_t3) + (assign12540_e11640 * var_t3_dn3)) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn3)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_dn3)), ((((((((var_agidl_i_dn4 * var_weff) + (var_agidl_i * var_weff_dn4)) * var_t3) + (assign12540_e11640 * var_t3_dn4)) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn4)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_dn4)), ((((((((var_agidl_i_dn5 * var_weff) + (var_agidl_i * var_weff_dn5)) * var_t3) + (assign12540_e11640 * var_t3_dn5)) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn5)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_dn5)), ((((((((var_agidl_i_dn6 * var_weff) + (var_agidl_i * var_weff_dn6)) * var_t3) + (assign12540_e11640 * var_t3_dn6)) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn6)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_dn6)), ((((((((var_agidl_i_dn7 * var_weff) + (var_agidl_i * var_weff_dn7)) * var_t3) + (assign12540_e11640 * var_t3_dn7)) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn7)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_dn7)), ((((((((var_agidl_i_dn8 * var_weff) + (var_agidl_i * var_weff_dn8)) * var_t3) + (assign12540_e11640 * var_t3_dn8)) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn8)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_dn8)), ((((((((var_agidl_i_db0 * var_weff) + (var_agidl_i * var_weff_db0)) * var_t3) + (assign12540_e11640 * var_t3_db0)) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_db0)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_db0)), ((((((((var_agidl_i_db1 * var_weff) + (var_agidl_i * var_weff_db1)) * var_t3) + (assign12540_e11640 * var_t3_db1)) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_db1)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_db1)), ((((((((var_agidl_i_db2 * var_weff) + (var_agidl_i * var_weff_db2)) * var_t3) + (assign12540_e11640 * var_t3_db2)) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_db2)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_db2)), ((((((((var_agidl_i_db3 * var_weff) + (var_agidl_i * var_weff_db3)) * var_t3) + (assign12540_e11640 * var_t3_db3)) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_db3)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_db3)), ((((((((var_agidl_i_db4 * var_weff) + (var_agidl_i * var_weff_db4)) * var_t3) + (assign12540_e11640 * var_t3_db4)) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_db4)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_db4)),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn1, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_db0, var_t6_db1, var_t6_db2, var_t6_db3, var_t6_db4,)
    }
};
        var_t6 = assign12540_e11650;
        var_t6_dn0 = assign12540_e11650_d_n0;
        var_t6_dn1 = assign12540_e11650_d_n1;
        var_t6_dn2 = assign12540_e11650_d_n2;
        var_t6_dn3 = assign12540_e11650_d_n3;
        var_t6_dn4 = assign12540_e11650_d_n4;
        var_t6_dn5 = assign12540_e11650_d_n5;
        var_t6_dn6 = assign12540_e11650_d_n6;
        var_t6_dn7 = assign12540_e11650_d_n7;
        var_t6_dn8 = assign12540_e11650_d_n8;
        var_t6_db0 = assign12540_e11650_d_b0;
        var_t6_db1 = assign12540_e11650_d_b1;
        var_t6_db2 = assign12540_e11650_d_b2;
        var_t6_db3 = assign12540_e11650_d_b3;
        var_t6_db4 = assign12540_e11650_d_b4;
        var_t6_rv = 0.0;
        var_t6_rdn0 = 0.0;
        var_t6_rdn1 = 0.0;
        var_t6_rdn2 = 0.0;
        var_t6_rdn3 = 0.0;
        var_t6_rdn4 = 0.0;
        var_t6_rdn5 = 0.0;
        var_t6_rdn6 = 0.0;
        var_t6_rdn7 = 0.0;
        var_t6_rdn8 = 0.0;
        var_t6_rdb0 = 0.0;
        var_t6_rdb1 = 0.0;
        var_t6_rdb2 = 0.0;
        var_t6_rdb3 = 0.0;
        var_t6_rdb4 = 0.0;

        let assign12580_e11673: f64 = if ((var_agisl_i <= 0.0) || (var_bgisl_t <= 0.0)) { 1.0 } else { 0.0 };
        var_guard129 = assign12580_e11673;
        var_guard129_dn0 = 0.0;
        var_guard129_dn1 = 0.0;
        var_guard129_dn2 = 0.0;
        var_guard129_dn3 = 0.0;
        var_guard129_dn4 = 0.0;
        var_guard129_dn5 = 0.0;
        var_guard129_dn6 = 0.0;
        var_guard129_dn7 = 0.0;
        var_guard129_dn8 = 0.0;
        var_guard129_db0 = 0.0;
        var_guard129_db1 = 0.0;
        var_guard129_db2 = 0.0;
        var_guard129_db3 = 0.0;
        var_guard129_db4 = 0.0;
        var_guard129_rv = 0.0;
        var_guard129_rdn0 = 0.0;
        var_guard129_rdn1 = 0.0;
        var_guard129_rdn2 = 0.0;
        var_guard129_rdn3 = 0.0;
        var_guard129_rdn4 = 0.0;
        var_guard129_rdn5 = 0.0;
        var_guard129_rdn6 = 0.0;
        var_guard129_rdn7 = 0.0;
        var_guard129_rdn8 = 0.0;
        var_guard129_rdb0 = 0.0;
        var_guard129_rdb1 = 0.0;
        var_guard129_rdb2 = 0.0;
        var_guard129_rdb3 = 0.0;
        var_guard129_rdb4 = 0.0;

        let (assign12590_e11679, assign12590_e11679_d_n0, assign12590_e11679_d_n1, assign12590_e11679_d_n2, assign12590_e11679_d_n3, assign12590_e11679_d_n4, assign12590_e11679_d_n5, assign12590_e11679_d_n6, assign12590_e11679_d_n7, assign12590_e11679_d_n8, assign12590_e11679_d_b0, assign12590_e11679_d_b1, assign12590_e11679_d_b2, assign12590_e11679_d_b3, assign12590_e11679_d_b4,) = {
    if ((var_guard126 != 0.0) && (var_guard129 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn1, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_db0, var_t6_db1, var_t6_db2, var_t6_db3, var_t6_db4,)
    }
};
        var_t6 = assign12590_e11679;
        var_t6_dn0 = assign12590_e11679_d_n0;
        var_t6_dn1 = assign12590_e11679_d_n1;
        var_t6_dn2 = assign12590_e11679_d_n2;
        var_t6_dn3 = assign12590_e11679_d_n3;
        var_t6_dn4 = assign12590_e11679_d_n4;
        var_t6_dn5 = assign12590_e11679_d_n5;
        var_t6_dn6 = assign12590_e11679_d_n6;
        var_t6_dn7 = assign12590_e11679_d_n7;
        var_t6_dn8 = assign12590_e11679_d_n8;
        var_t6_db0 = assign12590_e11679_d_b0;
        var_t6_db1 = assign12590_e11679_d_b1;
        var_t6_db2 = assign12590_e11679_d_b2;
        var_t6_db3 = assign12590_e11679_d_b3;
        var_t6_db4 = assign12590_e11679_d_b4;
        var_t6_rv = 0.0;
        var_t6_rdn0 = 0.0;
        var_t6_rdn1 = 0.0;
        var_t6_rdn2 = 0.0;
        var_t6_rdn3 = 0.0;
        var_t6_rdn4 = 0.0;
        var_t6_rdn5 = 0.0;
        var_t6_rdn6 = 0.0;
        var_t6_rdn7 = 0.0;
        var_t6_rdn8 = 0.0;
        var_t6_rdb0 = 0.0;
        var_t6_rdb1 = 0.0;
        var_t6_rdb2 = 0.0;
        var_t6_rdb3 = 0.0;
        var_t6_rdb4 = 0.0;

        let (assign12600_e11703, assign12600_e11703_d_n0, assign12600_e11703_d_n1, assign12600_e11703_d_n2, assign12600_e11703_d_n3, assign12600_e11703_d_n4, assign12600_e11703_d_n5, assign12600_e11703_d_n6, assign12600_e11703_d_n7, assign12600_e11703_d_n8, assign12600_e11703_d_b0, assign12600_e11703_d_b1, assign12600_e11703_d_b2, assign12600_e11703_d_b3, assign12600_e11703_d_b4,) = {
    if ((var_guard126 != 0.0) && (var_guard129 == 0.0)) {
        let assign12600_e11685: f64 = (-var_vgs_noswap);
        let assign12600_e11687: f64 = (assign12600_e11685 - var_egisl_i);
        let assign12600_e11689: f64 = (assign12600_e11687 + var_vfbsd);
        let assign12600_e11692: f64 = (var_vbgisl_i * var_gamma0);
        let assign12600_e11695: f64 = (var_vbgs - var_vfbsd_bg);
        let assign12600_e11697: f64 = (assign12600_e11695 - var_vbegisl_i);
        let assign12600_e11698: f64 = (assign12600_e11692 * assign12600_e11697);
        let assign12600_e11699: f64 = (assign12600_e11689 + assign12600_e11698);
        let assign12600_e11701: f64 = (assign12600_e11699 / var_t0);
        (assign12600_e11701, (((((((-var_vgs_noswap_dn0) - var_egisl_i_dn0) + var_vfbsd_dn0) + ((((var_vbgisl_i_dn0 * var_gamma0) + (var_vbgisl_i * var_gamma0_dn0)) * assign12600_e11697) + (assign12600_e11692 * ((var_vbgs_dn0 - var_vfbsd_bg_dn0) - var_vbegisl_i_dn0)))) * var_t0) - (assign12600_e11699 * var_t0_dn0)) / (var_t0 * var_t0)), (((((((-var_vgs_noswap_dn1) - var_egisl_i_dn1) + var_vfbsd_dn1) + ((((var_vbgisl_i_dn1 * var_gamma0) + (var_vbgisl_i * var_gamma0_dn1)) * assign12600_e11697) + (assign12600_e11692 * ((var_vbgs_dn1 - var_vfbsd_bg_dn1) - var_vbegisl_i_dn1)))) * var_t0) - (assign12600_e11699 * var_t0_dn1)) / (var_t0 * var_t0)), (((((((-var_vgs_noswap_dn2) - var_egisl_i_dn2) + var_vfbsd_dn2) + ((((var_vbgisl_i_dn2 * var_gamma0) + (var_vbgisl_i * var_gamma0_dn2)) * assign12600_e11697) + (assign12600_e11692 * ((var_vbgs_dn2 - var_vfbsd_bg_dn2) - var_vbegisl_i_dn2)))) * var_t0) - (assign12600_e11699 * var_t0_dn2)) / (var_t0 * var_t0)), (((((((-var_vgs_noswap_dn3) - var_egisl_i_dn3) + var_vfbsd_dn3) + ((((var_vbgisl_i_dn3 * var_gamma0) + (var_vbgisl_i * var_gamma0_dn3)) * assign12600_e11697) + (assign12600_e11692 * ((var_vbgs_dn3 - var_vfbsd_bg_dn3) - var_vbegisl_i_dn3)))) * var_t0) - (assign12600_e11699 * var_t0_dn3)) / (var_t0 * var_t0)), (((((((-var_vgs_noswap_dn4) - var_egisl_i_dn4) + var_vfbsd_dn4) + ((((var_vbgisl_i_dn4 * var_gamma0) + (var_vbgisl_i * var_gamma0_dn4)) * assign12600_e11697) + (assign12600_e11692 * ((var_vbgs_dn4 - var_vfbsd_bg_dn4) - var_vbegisl_i_dn4)))) * var_t0) - (assign12600_e11699 * var_t0_dn4)) / (var_t0 * var_t0)), (((((((-var_vgs_noswap_dn5) - var_egisl_i_dn5) + var_vfbsd_dn5) + ((((var_vbgisl_i_dn5 * var_gamma0) + (var_vbgisl_i * var_gamma0_dn5)) * assign12600_e11697) + (assign12600_e11692 * ((var_vbgs_dn5 - var_vfbsd_bg_dn5) - var_vbegisl_i_dn5)))) * var_t0) - (assign12600_e11699 * var_t0_dn5)) / (var_t0 * var_t0)), (((((((-var_vgs_noswap_dn6) - var_egisl_i_dn6) + var_vfbsd_dn6) + ((((var_vbgisl_i_dn6 * var_gamma0) + (var_vbgisl_i * var_gamma0_dn6)) * assign12600_e11697) + (assign12600_e11692 * ((var_vbgs_dn6 - var_vfbsd_bg_dn6) - var_vbegisl_i_dn6)))) * var_t0) - (assign12600_e11699 * var_t0_dn6)) / (var_t0 * var_t0)), (((((((-var_vgs_noswap_dn7) - var_egisl_i_dn7) + var_vfbsd_dn7) + ((((var_vbgisl_i_dn7 * var_gamma0) + (var_vbgisl_i * var_gamma0_dn7)) * assign12600_e11697) + (assign12600_e11692 * ((var_vbgs_dn7 - var_vfbsd_bg_dn7) - var_vbegisl_i_dn7)))) * var_t0) - (assign12600_e11699 * var_t0_dn7)) / (var_t0 * var_t0)), (((((((-var_vgs_noswap_dn8) - var_egisl_i_dn8) + var_vfbsd_dn8) + ((((var_vbgisl_i_dn8 * var_gamma0) + (var_vbgisl_i * var_gamma0_dn8)) * assign12600_e11697) + (assign12600_e11692 * ((var_vbgs_dn8 - var_vfbsd_bg_dn8) - var_vbegisl_i_dn8)))) * var_t0) - (assign12600_e11699 * var_t0_dn8)) / (var_t0 * var_t0)), (((((((-var_vgs_noswap_db0) - var_egisl_i_db0) + var_vfbsd_db0) + ((((var_vbgisl_i_db0 * var_gamma0) + (var_vbgisl_i * var_gamma0_db0)) * assign12600_e11697) + (assign12600_e11692 * ((var_vbgs_db0 - var_vfbsd_bg_db0) - var_vbegisl_i_db0)))) * var_t0) - (assign12600_e11699 * var_t0_db0)) / (var_t0 * var_t0)), (((((((-var_vgs_noswap_db1) - var_egisl_i_db1) + var_vfbsd_db1) + ((((var_vbgisl_i_db1 * var_gamma0) + (var_vbgisl_i * var_gamma0_db1)) * assign12600_e11697) + (assign12600_e11692 * ((var_vbgs_db1 - var_vfbsd_bg_db1) - var_vbegisl_i_db1)))) * var_t0) - (assign12600_e11699 * var_t0_db1)) / (var_t0 * var_t0)), (((((((-var_vgs_noswap_db2) - var_egisl_i_db2) + var_vfbsd_db2) + ((((var_vbgisl_i_db2 * var_gamma0) + (var_vbgisl_i * var_gamma0_db2)) * assign12600_e11697) + (assign12600_e11692 * ((var_vbgs_db2 - var_vfbsd_bg_db2) - var_vbegisl_i_db2)))) * var_t0) - (assign12600_e11699 * var_t0_db2)) / (var_t0 * var_t0)), (((((((-var_vgs_noswap_db3) - var_egisl_i_db3) + var_vfbsd_db3) + ((((var_vbgisl_i_db3 * var_gamma0) + (var_vbgisl_i * var_gamma0_db3)) * assign12600_e11697) + (assign12600_e11692 * ((var_vbgs_db3 - var_vfbsd_bg_db3) - var_vbegisl_i_db3)))) * var_t0) - (assign12600_e11699 * var_t0_db3)) / (var_t0 * var_t0)), (((((((-var_vgs_noswap_db4) - var_egisl_i_db4) + var_vfbsd_db4) + ((((var_vbgisl_i_db4 * var_gamma0) + (var_vbgisl_i * var_gamma0_db4)) * assign12600_e11697) + (assign12600_e11692 * ((var_vbgs_db4 - var_vfbsd_bg_db4) - var_vbegisl_i_db4)))) * var_t0) - (assign12600_e11699 * var_t0_db4)) / (var_t0 * var_t0)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4,)
    }
};
        var_t1 = assign12600_e11703;
        var_t1_dn0 = assign12600_e11703_d_n0;
        var_t1_dn1 = assign12600_e11703_d_n1;
        var_t1_dn2 = assign12600_e11703_d_n2;
        var_t1_dn3 = assign12600_e11703_d_n3;
        var_t1_dn4 = assign12600_e11703_d_n4;
        var_t1_dn5 = assign12600_e11703_d_n5;
        var_t1_dn6 = assign12600_e11703_d_n6;
        var_t1_dn7 = assign12600_e11703_d_n7;
        var_t1_dn8 = assign12600_e11703_d_n8;
        var_t1_db0 = assign12600_e11703_d_b0;
        var_t1_db1 = assign12600_e11703_d_b1;
        var_t1_db2 = assign12600_e11703_d_b2;
        var_t1_db3 = assign12600_e11703_d_b3;
        var_t1_db4 = assign12600_e11703_d_b4;
        var_t1_rv = 0.0;
        var_t1_rdn0 = 0.0;
        var_t1_rdn1 = 0.0;
        var_t1_rdn2 = 0.0;
        var_t1_rdn3 = 0.0;
        var_t1_rdn4 = 0.0;
        var_t1_rdn5 = 0.0;
        var_t1_rdn6 = 0.0;
        var_t1_rdn7 = 0.0;
        var_t1_rdn8 = 0.0;
        var_t1_rdb0 = 0.0;
        var_t1_rdb1 = 0.0;
        var_t1_rdb2 = 0.0;
        var_t1_rdb3 = 0.0;
        var_t1_rdb4 = 0.0;

        let (assign12610_e11723, assign12610_e11723_d_n0, assign12610_e11723_d_n1, assign12610_e11723_d_n2, assign12610_e11723_d_n3, assign12610_e11723_d_n4, assign12610_e11723_d_n5, assign12610_e11723_d_n6, assign12610_e11723_d_n7, assign12610_e11723_d_n8, assign12610_e11723_d_b0, assign12610_e11723_d_b1, assign12610_e11723_d_b2, assign12610_e11723_d_b3, assign12610_e11723_d_b4,) = {
    if ((var_guard126 != 0.0) && (var_guard129 == 0.0)) {
        let assign12610_e11712: f64 = (var_t1 * var_t1);
        let assign12610_e11715: f64 = (4.0 * 0.01);
        let assign12610_e11717: f64 = (assign12610_e11715 * 0.01);
        let assign12610_e11718: f64 = (assign12610_e11712 + assign12610_e11717);
        let assign12610_e11719: f64 = (assign12610_e11718).sqrt();
        let assign12610_e11720: f64 = (var_t1 + assign12610_e11719);
        let assign12610_e11721: f64 = (0.5 * assign12610_e11720);
        (assign12610_e11721, (0.5 * (var_t1_dn0 + (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_dn1 + (((var_t1_dn1 * var_t1) + (var_t1 * var_t1_dn1)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_dn2 + (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_dn3 + (((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_dn4 + (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_dn5 + (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_dn6 + (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_dn7 + (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_dn8 + (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_db0 + (((var_t1_db0 * var_t1) + (var_t1 * var_t1_db0)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_db1 + (((var_t1_db1 * var_t1) + (var_t1 * var_t1_db1)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_db2 + (((var_t1_db2 * var_t1) + (var_t1 * var_t1_db2)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_db3 + (((var_t1_db3 * var_t1) + (var_t1 * var_t1_db3)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_db4 + (((var_t1_db4 * var_t1) + (var_t1 * var_t1_db4)) / (2.0 * assign12610_e11719)))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4,)
    }
};
        var_t1 = assign12610_e11723;
        var_t1_dn0 = assign12610_e11723_d_n0;
        var_t1_dn1 = assign12610_e11723_d_n1;
        var_t1_dn2 = assign12610_e11723_d_n2;
        var_t1_dn3 = assign12610_e11723_d_n3;
        var_t1_dn4 = assign12610_e11723_d_n4;
        var_t1_dn5 = assign12610_e11723_d_n5;
        var_t1_dn6 = assign12610_e11723_d_n6;
        var_t1_dn7 = assign12610_e11723_d_n7;
        var_t1_dn8 = assign12610_e11723_d_n8;
        var_t1_db0 = assign12610_e11723_d_b0;
        var_t1_db1 = assign12610_e11723_d_b1;
        var_t1_db2 = assign12610_e11723_d_b2;
        var_t1_db3 = assign12610_e11723_d_b3;
        var_t1_db4 = assign12610_e11723_d_b4;
        var_t1_rv = 0.0;
        var_t1_rdn0 = 0.0;
        var_t1_rdn1 = 0.0;
        var_t1_rdn2 = 0.0;
        var_t1_rdn3 = 0.0;
        var_t1_rdn4 = 0.0;
        var_t1_rdn5 = 0.0;
        var_t1_rdn6 = 0.0;
        var_t1_rdn7 = 0.0;
        var_t1_rdn8 = 0.0;
        var_t1_rdb0 = 0.0;
        var_t1_rdb1 = 0.0;
        var_t1_rdb2 = 0.0;
        var_t1_rdb3 = 0.0;
        var_t1_rdb4 = 0.0;

        let (assign12620_e11734, assign12620_e11734_d_n0, assign12620_e11734_d_n1, assign12620_e11734_d_n2, assign12620_e11734_d_n3, assign12620_e11734_d_n4, assign12620_e11734_d_n5, assign12620_e11734_d_n6, assign12620_e11734_d_n7, assign12620_e11734_d_n8, assign12620_e11734_d_b0, assign12620_e11734_d_b1, assign12620_e11734_d_b2, assign12620_e11734_d_b3, assign12620_e11734_d_b4,) = {
    if ((var_guard126 != 0.0) && (var_guard129 == 0.0)) {
        let assign12620_e11731: f64 = (var_t1 + 0.001);
        let assign12620_e11732: f64 = (var_bgisl_t / assign12620_e11731);
        (assign12620_e11732, (((var_bgisl_t_dn0 * assign12620_e11731) - (var_bgisl_t * var_t1_dn0)) / (assign12620_e11731 * assign12620_e11731)), (((var_bgisl_t_dn1 * assign12620_e11731) - (var_bgisl_t * var_t1_dn1)) / (assign12620_e11731 * assign12620_e11731)), (((var_bgisl_t_dn2 * assign12620_e11731) - (var_bgisl_t * var_t1_dn2)) / (assign12620_e11731 * assign12620_e11731)), (((var_bgisl_t_dn3 * assign12620_e11731) - (var_bgisl_t * var_t1_dn3)) / (assign12620_e11731 * assign12620_e11731)), (((var_bgisl_t_dn4 * assign12620_e11731) - (var_bgisl_t * var_t1_dn4)) / (assign12620_e11731 * assign12620_e11731)), (((var_bgisl_t_dn5 * assign12620_e11731) - (var_bgisl_t * var_t1_dn5)) / (assign12620_e11731 * assign12620_e11731)), (((var_bgisl_t_dn6 * assign12620_e11731) - (var_bgisl_t * var_t1_dn6)) / (assign12620_e11731 * assign12620_e11731)), (((var_bgisl_t_dn7 * assign12620_e11731) - (var_bgisl_t * var_t1_dn7)) / (assign12620_e11731 * assign12620_e11731)), (((var_bgisl_t_dn8 * assign12620_e11731) - (var_bgisl_t * var_t1_dn8)) / (assign12620_e11731 * assign12620_e11731)), (((var_bgisl_t_db0 * assign12620_e11731) - (var_bgisl_t * var_t1_db0)) / (assign12620_e11731 * assign12620_e11731)), (((var_bgisl_t_db1 * assign12620_e11731) - (var_bgisl_t * var_t1_db1)) / (assign12620_e11731 * assign12620_e11731)), (((var_bgisl_t_db2 * assign12620_e11731) - (var_bgisl_t * var_t1_db2)) / (assign12620_e11731 * assign12620_e11731)), (((var_bgisl_t_db3 * assign12620_e11731) - (var_bgisl_t * var_t1_db3)) / (assign12620_e11731 * assign12620_e11731)), (((var_bgisl_t_db4 * assign12620_e11731) - (var_bgisl_t * var_t1_db4)) / (assign12620_e11731 * assign12620_e11731)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_db0, var_t2_db1, var_t2_db2, var_t2_db3, var_t2_db4,)
    }
};
        var_t2 = assign12620_e11734;
        var_t2_dn0 = assign12620_e11734_d_n0;
        var_t2_dn1 = assign12620_e11734_d_n1;
        var_t2_dn2 = assign12620_e11734_d_n2;
        var_t2_dn3 = assign12620_e11734_d_n3;
        var_t2_dn4 = assign12620_e11734_d_n4;
        var_t2_dn5 = assign12620_e11734_d_n5;
        var_t2_dn6 = assign12620_e11734_d_n6;
        var_t2_dn7 = assign12620_e11734_d_n7;
        var_t2_dn8 = assign12620_e11734_d_n8;
        var_t2_db0 = assign12620_e11734_d_b0;
        var_t2_db1 = assign12620_e11734_d_b1;
        var_t2_db2 = assign12620_e11734_d_b2;
        var_t2_db3 = assign12620_e11734_d_b3;
        var_t2_db4 = assign12620_e11734_d_b4;
        var_t2_rv = 0.0;
        var_t2_rdn0 = 0.0;
        var_t2_rdn1 = 0.0;
        var_t2_rdn2 = 0.0;
        var_t2_rdn3 = 0.0;
        var_t2_rdn4 = 0.0;
        var_t2_rdn5 = 0.0;
        var_t2_rdn6 = 0.0;
        var_t2_rdn7 = 0.0;
        var_t2_rdn8 = 0.0;
        var_t2_rdb0 = 0.0;
        var_t2_rdb1 = 0.0;
        var_t2_rdb2 = 0.0;
        var_t2_rdb3 = 0.0;
        var_t2_rdb4 = 0.0;

        let (assign12630_e11747, assign12630_e11747_d_n0, assign12630_e11747_d_n1, assign12630_e11747_d_n2, assign12630_e11747_d_n3, assign12630_e11747_d_n4, assign12630_e11747_d_n5, assign12630_e11747_d_n6, assign12630_e11747_d_n7, assign12630_e11747_d_n8, assign12630_e11747_d_b0, assign12630_e11747_d_b1, assign12630_e11747_d_b2, assign12630_e11747_d_b3, assign12630_e11747_d_b4,) = {
    if ((var_guard126 != 0.0) && (var_guard129 == 0.0)) {
        let assign12630_e11742: f64 = (var_t1).max(1e-38);
        let assign12630_e11743: f64 = (assign12630_e11742).ln();
        let assign12630_e11744: f64 = (var_pgisl_i * assign12630_e11743);
        let assign12630_e11745: f64 = { let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12630_e11745, ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgisl_i_dn0 * assign12630_e11743) + (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn0 } else { 0.0 } / assign12630_e11742)))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgisl_i_dn1 * assign12630_e11743) + (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn1 } else { 0.0 } / assign12630_e11742)))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgisl_i_dn2 * assign12630_e11743) + (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn2 } else { 0.0 } / assign12630_e11742)))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgisl_i_dn3 * assign12630_e11743) + (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn3 } else { 0.0 } / assign12630_e11742)))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgisl_i_dn4 * assign12630_e11743) + (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn4 } else { 0.0 } / assign12630_e11742)))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgisl_i_dn5 * assign12630_e11743) + (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn5 } else { 0.0 } / assign12630_e11742)))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgisl_i_dn6 * assign12630_e11743) + (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn6 } else { 0.0 } / assign12630_e11742)))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgisl_i_dn7 * assign12630_e11743) + (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn7 } else { 0.0 } / assign12630_e11742)))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgisl_i_dn8 * assign12630_e11743) + (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn8 } else { 0.0 } / assign12630_e11742)))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgisl_i_db0 * assign12630_e11743) + (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_db0 } else { 0.0 } / assign12630_e11742)))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgisl_i_db1 * assign12630_e11743) + (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_db1 } else { 0.0 } / assign12630_e11742)))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgisl_i_db2 * assign12630_e11743) + (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_db2 } else { 0.0 } / assign12630_e11742)))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgisl_i_db3 * assign12630_e11743) + (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_db3 } else { 0.0 } / assign12630_e11742)))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_pgisl_i_db4 * assign12630_e11743) + (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_db4 } else { 0.0 } / assign12630_e11742)))),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn1, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_db0, var_t3_db1, var_t3_db2, var_t3_db3, var_t3_db4,)
    }
};
        var_t3 = assign12630_e11747;
        var_t3_dn0 = assign12630_e11747_d_n0;
        var_t3_dn1 = assign12630_e11747_d_n1;
        var_t3_dn2 = assign12630_e11747_d_n2;
        var_t3_dn3 = assign12630_e11747_d_n3;
        var_t3_dn4 = assign12630_e11747_d_n4;
        var_t3_dn5 = assign12630_e11747_d_n5;
        var_t3_dn6 = assign12630_e11747_d_n6;
        var_t3_dn7 = assign12630_e11747_d_n7;
        var_t3_dn8 = assign12630_e11747_d_n8;
        var_t3_db0 = assign12630_e11747_d_b0;
        var_t3_db1 = assign12630_e11747_d_b1;
        var_t3_db2 = assign12630_e11747_d_b2;
        var_t3_db3 = assign12630_e11747_d_b3;
        var_t3_db4 = assign12630_e11747_d_b4;
        var_t3_rv = 0.0;
        var_t3_rdn0 = 0.0;
        var_t3_rdn1 = 0.0;
        var_t3_rdn2 = 0.0;
        var_t3_rdn3 = 0.0;
        var_t3_rdn4 = 0.0;
        var_t3_rdn5 = 0.0;
        var_t3_rdn6 = 0.0;
        var_t3_rdn7 = 0.0;
        var_t3_rdn8 = 0.0;
        var_t3_rdb0 = 0.0;
        var_t3_rdb1 = 0.0;
        var_t3_rdb2 = 0.0;
        var_t3_rdb3 = 0.0;
        var_t3_rdb4 = 0.0;

        let (assign12640_e11765, assign12640_e11765_d_n0, assign12640_e11765_d_n1, assign12640_e11765_d_n2, assign12640_e11765_d_n3, assign12640_e11765_d_n4, assign12640_e11765_d_n5, assign12640_e11765_d_n6, assign12640_e11765_d_n7, assign12640_e11765_d_n8, assign12640_e11765_d_b0, assign12640_e11765_d_b1, assign12640_e11765_d_b2, assign12640_e11765_d_b3, assign12640_e11765_d_b4,) = {
    if ((var_guard126 != 0.0) && (var_guard129 == 0.0)) {
        let assign12640_e11753: f64 = (-var_vds_noswap);
        let assign12640_e11755: f64 = (assign12640_e11753 * var_agisl_i);
        let assign12640_e11757: f64 = (assign12640_e11755 * var_weff);
        let assign12640_e11759: f64 = (assign12640_e11757 * var_t3);
        let assign12640_e11761: f64 = (-var_t2);
        let assign12640_e11762: f64 = { let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign12640_e11763: f64 = (assign12640_e11759 * assign12640_e11762);
        (assign12640_e11763, (((((((((-var_vds_noswap_dn0) * var_agisl_i) + (assign12640_e11753 * var_agisl_i_dn0)) * var_weff) + (assign12640_e11755 * var_weff_dn0)) * var_t3) + (assign12640_e11757 * var_t3_dn0)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn0)))), (((((((((-var_vds_noswap_dn1) * var_agisl_i) + (assign12640_e11753 * var_agisl_i_dn1)) * var_weff) + (assign12640_e11755 * var_weff_dn1)) * var_t3) + (assign12640_e11757 * var_t3_dn1)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn1)))), (((((((((-var_vds_noswap_dn2) * var_agisl_i) + (assign12640_e11753 * var_agisl_i_dn2)) * var_weff) + (assign12640_e11755 * var_weff_dn2)) * var_t3) + (assign12640_e11757 * var_t3_dn2)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn2)))), (((((((((-var_vds_noswap_dn3) * var_agisl_i) + (assign12640_e11753 * var_agisl_i_dn3)) * var_weff) + (assign12640_e11755 * var_weff_dn3)) * var_t3) + (assign12640_e11757 * var_t3_dn3)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn3)))), (((((((((-var_vds_noswap_dn4) * var_agisl_i) + (assign12640_e11753 * var_agisl_i_dn4)) * var_weff) + (assign12640_e11755 * var_weff_dn4)) * var_t3) + (assign12640_e11757 * var_t3_dn4)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn4)))), (((((((((-var_vds_noswap_dn5) * var_agisl_i) + (assign12640_e11753 * var_agisl_i_dn5)) * var_weff) + (assign12640_e11755 * var_weff_dn5)) * var_t3) + (assign12640_e11757 * var_t3_dn5)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn5)))), (((((((((-var_vds_noswap_dn6) * var_agisl_i) + (assign12640_e11753 * var_agisl_i_dn6)) * var_weff) + (assign12640_e11755 * var_weff_dn6)) * var_t3) + (assign12640_e11757 * var_t3_dn6)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn6)))), (((((((((-var_vds_noswap_dn7) * var_agisl_i) + (assign12640_e11753 * var_agisl_i_dn7)) * var_weff) + (assign12640_e11755 * var_weff_dn7)) * var_t3) + (assign12640_e11757 * var_t3_dn7)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn7)))), (((((((((-var_vds_noswap_dn8) * var_agisl_i) + (assign12640_e11753 * var_agisl_i_dn8)) * var_weff) + (assign12640_e11755 * var_weff_dn8)) * var_t3) + (assign12640_e11757 * var_t3_dn8)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn8)))), (((((((((-var_vds_noswap_db0) * var_agisl_i) + (assign12640_e11753 * var_agisl_i_db0)) * var_weff) + (assign12640_e11755 * var_weff_db0)) * var_t3) + (assign12640_e11757 * var_t3_db0)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_db0)))), (((((((((-var_vds_noswap_db1) * var_agisl_i) + (assign12640_e11753 * var_agisl_i_db1)) * var_weff) + (assign12640_e11755 * var_weff_db1)) * var_t3) + (assign12640_e11757 * var_t3_db1)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_db1)))), (((((((((-var_vds_noswap_db2) * var_agisl_i) + (assign12640_e11753 * var_agisl_i_db2)) * var_weff) + (assign12640_e11755 * var_weff_db2)) * var_t3) + (assign12640_e11757 * var_t3_db2)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_db2)))), (((((((((-var_vds_noswap_db3) * var_agisl_i) + (assign12640_e11753 * var_agisl_i_db3)) * var_weff) + (assign12640_e11755 * var_weff_db3)) * var_t3) + (assign12640_e11757 * var_t3_db3)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_db3)))), (((((((((-var_vds_noswap_db4) * var_agisl_i) + (assign12640_e11753 * var_agisl_i_db4)) * var_weff) + (assign12640_e11755 * var_weff_db4)) * var_t3) + (assign12640_e11757 * var_t3_db4)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_db4)))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn1, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_db0, var_t6_db1, var_t6_db2, var_t6_db3, var_t6_db4,)
    }
};
        var_t6 = assign12640_e11765;
        var_t6_dn0 = assign12640_e11765_d_n0;
        var_t6_dn1 = assign12640_e11765_d_n1;
        var_t6_dn2 = assign12640_e11765_d_n2;
        var_t6_dn3 = assign12640_e11765_d_n3;
        var_t6_dn4 = assign12640_e11765_d_n4;
        var_t6_dn5 = assign12640_e11765_d_n5;
        var_t6_dn6 = assign12640_e11765_d_n6;
        var_t6_dn7 = assign12640_e11765_d_n7;
        var_t6_dn8 = assign12640_e11765_d_n8;
        var_t6_db0 = assign12640_e11765_d_b0;
        var_t6_db1 = assign12640_e11765_d_b1;
        var_t6_db2 = assign12640_e11765_d_b2;
        var_t6_db3 = assign12640_e11765_d_b3;
        var_t6_db4 = assign12640_e11765_d_b4;
        var_t6_rv = 0.0;
        var_t6_rdn0 = 0.0;
        var_t6_rdn1 = 0.0;
        var_t6_rdn2 = 0.0;
        var_t6_rdn3 = 0.0;
        var_t6_rdn4 = 0.0;
        var_t6_rdn5 = 0.0;
        var_t6_rdn6 = 0.0;
        var_t6_rdn7 = 0.0;
        var_t6_rdn8 = 0.0;
        var_t6_rdb0 = 0.0;
        var_t6_rdb1 = 0.0;
        var_t6_rdb2 = 0.0;
        var_t6_rdb3 = 0.0;
        var_t6_rdb4 = 0.0;

        let assign12680_e11784: f64 = (2.0 * var_vsat_t);
        let assign12680_e11786: f64 = (assign12680_e11784 / var_utotal);
        var_esatnoi = assign12680_e11786;
        var_esatnoi_dn0 = ((((2.0 * var_vsat_t_dn0) * var_utotal) - (assign12680_e11784 * var_utotal_dn0)) / (var_utotal * var_utotal));
        var_esatnoi_dn1 = ((((2.0 * var_vsat_t_dn1) * var_utotal) - (assign12680_e11784 * var_utotal_dn1)) / (var_utotal * var_utotal));
        var_esatnoi_dn2 = ((((2.0 * var_vsat_t_dn2) * var_utotal) - (assign12680_e11784 * var_utotal_dn2)) / (var_utotal * var_utotal));
        var_esatnoi_dn3 = ((((2.0 * var_vsat_t_dn3) * var_utotal) - (assign12680_e11784 * var_utotal_dn3)) / (var_utotal * var_utotal));
        var_esatnoi_dn4 = ((((2.0 * var_vsat_t_dn4) * var_utotal) - (assign12680_e11784 * var_utotal_dn4)) / (var_utotal * var_utotal));
        var_esatnoi_dn5 = ((((2.0 * var_vsat_t_dn5) * var_utotal) - (assign12680_e11784 * var_utotal_dn5)) / (var_utotal * var_utotal));
        var_esatnoi_dn6 = ((((2.0 * var_vsat_t_dn6) * var_utotal) - (assign12680_e11784 * var_utotal_dn6)) / (var_utotal * var_utotal));
        var_esatnoi_dn7 = ((((2.0 * var_vsat_t_dn7) * var_utotal) - (assign12680_e11784 * var_utotal_dn7)) / (var_utotal * var_utotal));
        var_esatnoi_dn8 = ((((2.0 * var_vsat_t_dn8) * var_utotal) - (assign12680_e11784 * var_utotal_dn8)) / (var_utotal * var_utotal));
        var_esatnoi_db0 = ((((2.0 * var_vsat_t_db0) * var_utotal) - (assign12680_e11784 * var_utotal_db0)) / (var_utotal * var_utotal));
        var_esatnoi_db1 = ((((2.0 * var_vsat_t_db1) * var_utotal) - (assign12680_e11784 * var_utotal_db1)) / (var_utotal * var_utotal));
        var_esatnoi_db2 = ((((2.0 * var_vsat_t_db2) * var_utotal) - (assign12680_e11784 * var_utotal_db2)) / (var_utotal * var_utotal));
        var_esatnoi_db3 = ((((2.0 * var_vsat_t_db3) * var_utotal) - (assign12680_e11784 * var_utotal_db3)) / (var_utotal * var_utotal));
        var_esatnoi_db4 = ((((2.0 * var_vsat_t_db4) * var_utotal) - (assign12680_e11784 * var_utotal_db4)) / (var_utotal * var_utotal));
        var_esatnoi_rv = 0.0;
        var_esatnoi_rdn0 = 0.0;
        var_esatnoi_rdn1 = 0.0;
        var_esatnoi_rdn2 = 0.0;
        var_esatnoi_rdn3 = 0.0;
        var_esatnoi_rdn4 = 0.0;
        var_esatnoi_rdn5 = 0.0;
        var_esatnoi_rdn6 = 0.0;
        var_esatnoi_rdn7 = 0.0;
        var_esatnoi_rdn8 = 0.0;
        var_esatnoi_rdb0 = 0.0;
        var_esatnoi_rdb1 = 0.0;
        var_esatnoi_rdb2 = 0.0;
        var_esatnoi_rdb3 = 0.0;
        var_esatnoi_rdb4 = 0.0;

        let assign12690_e11797: f64 = if (((p.p288 > 0.0) || (p.p289 > 0.0)) || (p.p290 > 0.0)) { 1.0 } else { 0.0 };
        var_guard131 = assign12690_e11797;
        var_guard131_dn0 = 0.0;
        var_guard131_dn1 = 0.0;
        var_guard131_dn2 = 0.0;
        var_guard131_dn3 = 0.0;
        var_guard131_dn4 = 0.0;
        var_guard131_dn5 = 0.0;
        var_guard131_dn6 = 0.0;
        var_guard131_dn7 = 0.0;
        var_guard131_dn8 = 0.0;
        var_guard131_db0 = 0.0;
        var_guard131_db1 = 0.0;
        var_guard131_db2 = 0.0;
        var_guard131_db3 = 0.0;
        var_guard131_db4 = 0.0;
        var_guard131_rv = 0.0;
        var_guard131_rdn0 = 0.0;
        var_guard131_rdn1 = 0.0;
        var_guard131_rdn2 = 0.0;
        var_guard131_rdn3 = 0.0;
        var_guard131_rdn4 = 0.0;
        var_guard131_rdn5 = 0.0;
        var_guard131_rdn6 = 0.0;
        var_guard131_rdn7 = 0.0;
        var_guard131_rdn8 = 0.0;
        var_guard131_rdb0 = 0.0;
        var_guard131_rdb1 = 0.0;
        var_guard131_rdb2 = 0.0;
        var_guard131_rdb3 = 0.0;
        var_guard131_rdb4 = 0.0;


        *var_esatnoi_slot = var_esatnoi;
        *var_esatnoi_db0_slot = var_esatnoi_db0;
        *var_esatnoi_db1_slot = var_esatnoi_db1;
        *var_esatnoi_db2_slot = var_esatnoi_db2;
        *var_esatnoi_db3_slot = var_esatnoi_db3;
        *var_esatnoi_db4_slot = var_esatnoi_db4;
        *var_esatnoi_dn0_slot = var_esatnoi_dn0;
        *var_esatnoi_dn1_slot = var_esatnoi_dn1;
        *var_esatnoi_dn2_slot = var_esatnoi_dn2;
        *var_esatnoi_dn3_slot = var_esatnoi_dn3;
        *var_esatnoi_dn4_slot = var_esatnoi_dn4;
        *var_esatnoi_dn5_slot = var_esatnoi_dn5;
        *var_esatnoi_dn6_slot = var_esatnoi_dn6;
        *var_esatnoi_dn7_slot = var_esatnoi_dn7;
        *var_esatnoi_dn8_slot = var_esatnoi_dn8;
        *var_esatnoi_rdb0_slot = var_esatnoi_rdb0;
        *var_esatnoi_rdb1_slot = var_esatnoi_rdb1;
        *var_esatnoi_rdb2_slot = var_esatnoi_rdb2;
        *var_esatnoi_rdb3_slot = var_esatnoi_rdb3;
        *var_esatnoi_rdb4_slot = var_esatnoi_rdb4;
        *var_esatnoi_rdn0_slot = var_esatnoi_rdn0;
        *var_esatnoi_rdn1_slot = var_esatnoi_rdn1;
        *var_esatnoi_rdn2_slot = var_esatnoi_rdn2;
        *var_esatnoi_rdn3_slot = var_esatnoi_rdn3;
        *var_esatnoi_rdn4_slot = var_esatnoi_rdn4;
        *var_esatnoi_rdn5_slot = var_esatnoi_rdn5;
        *var_esatnoi_rdn6_slot = var_esatnoi_rdn6;
        *var_esatnoi_rdn7_slot = var_esatnoi_rdn7;
        *var_esatnoi_rdn8_slot = var_esatnoi_rdn8;
        *var_esatnoi_rv_slot = var_esatnoi_rv;
        *var_guard129_slot = var_guard129;
        *var_guard129_db0_slot = var_guard129_db0;
        *var_guard129_db1_slot = var_guard129_db1;
        *var_guard129_db2_slot = var_guard129_db2;
        *var_guard129_db3_slot = var_guard129_db3;
        *var_guard129_db4_slot = var_guard129_db4;
        *var_guard129_dn0_slot = var_guard129_dn0;
        *var_guard129_dn1_slot = var_guard129_dn1;
        *var_guard129_dn2_slot = var_guard129_dn2;
        *var_guard129_dn3_slot = var_guard129_dn3;
        *var_guard129_dn4_slot = var_guard129_dn4;
        *var_guard129_dn5_slot = var_guard129_dn5;
        *var_guard129_dn6_slot = var_guard129_dn6;
        *var_guard129_dn7_slot = var_guard129_dn7;
        *var_guard129_dn8_slot = var_guard129_dn8;
        *var_guard129_rdb0_slot = var_guard129_rdb0;
        *var_guard129_rdb1_slot = var_guard129_rdb1;
        *var_guard129_rdb2_slot = var_guard129_rdb2;
        *var_guard129_rdb3_slot = var_guard129_rdb3;
        *var_guard129_rdb4_slot = var_guard129_rdb4;
        *var_guard129_rdn0_slot = var_guard129_rdn0;
        *var_guard129_rdn1_slot = var_guard129_rdn1;
        *var_guard129_rdn2_slot = var_guard129_rdn2;
        *var_guard129_rdn3_slot = var_guard129_rdn3;
        *var_guard129_rdn4_slot = var_guard129_rdn4;
        *var_guard129_rdn5_slot = var_guard129_rdn5;
        *var_guard129_rdn6_slot = var_guard129_rdn6;
        *var_guard129_rdn7_slot = var_guard129_rdn7;
        *var_guard129_rdn8_slot = var_guard129_rdn8;
        *var_guard129_rv_slot = var_guard129_rv;
        *var_guard131_slot = var_guard131;
        *var_guard131_db0_slot = var_guard131_db0;
        *var_guard131_db1_slot = var_guard131_db1;
        *var_guard131_db2_slot = var_guard131_db2;
        *var_guard131_db3_slot = var_guard131_db3;
        *var_guard131_db4_slot = var_guard131_db4;
        *var_guard131_dn0_slot = var_guard131_dn0;
        *var_guard131_dn1_slot = var_guard131_dn1;
        *var_guard131_dn2_slot = var_guard131_dn2;
        *var_guard131_dn3_slot = var_guard131_dn3;
        *var_guard131_dn4_slot = var_guard131_dn4;
        *var_guard131_dn5_slot = var_guard131_dn5;
        *var_guard131_dn6_slot = var_guard131_dn6;
        *var_guard131_dn7_slot = var_guard131_dn7;
        *var_guard131_dn8_slot = var_guard131_dn8;
        *var_guard131_rdb0_slot = var_guard131_rdb0;
        *var_guard131_rdb1_slot = var_guard131_rdb1;
        *var_guard131_rdb2_slot = var_guard131_rdb2;
        *var_guard131_rdb3_slot = var_guard131_rdb3;
        *var_guard131_rdb4_slot = var_guard131_rdb4;
        *var_guard131_rdn0_slot = var_guard131_rdn0;
        *var_guard131_rdn1_slot = var_guard131_rdn1;
        *var_guard131_rdn2_slot = var_guard131_rdn2;
        *var_guard131_rdn3_slot = var_guard131_rdn3;
        *var_guard131_rdn4_slot = var_guard131_rdn4;
        *var_guard131_rdn5_slot = var_guard131_rdn5;
        *var_guard131_rdn6_slot = var_guard131_rdn6;
        *var_guard131_rdn7_slot = var_guard131_rdn7;
        *var_guard131_rdn8_slot = var_guard131_rdn8;
        *var_guard131_rv_slot = var_guard131_rv;
        *var_t1_slot = var_t1;
        *var_t1_db0_slot = var_t1_db0;
        *var_t1_db1_slot = var_t1_db1;
        *var_t1_db2_slot = var_t1_db2;
        *var_t1_db3_slot = var_t1_db3;
        *var_t1_db4_slot = var_t1_db4;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rdb0_slot = var_t1_rdb0;
        *var_t1_rdb1_slot = var_t1_rdb1;
        *var_t1_rdb2_slot = var_t1_rdb2;
        *var_t1_rdb3_slot = var_t1_rdb3;
        *var_t1_rdb4_slot = var_t1_rdb4;
        *var_t1_rdn0_slot = var_t1_rdn0;
        *var_t1_rdn1_slot = var_t1_rdn1;
        *var_t1_rdn2_slot = var_t1_rdn2;
        *var_t1_rdn3_slot = var_t1_rdn3;
        *var_t1_rdn4_slot = var_t1_rdn4;
        *var_t1_rdn5_slot = var_t1_rdn5;
        *var_t1_rdn6_slot = var_t1_rdn6;
        *var_t1_rdn7_slot = var_t1_rdn7;
        *var_t1_rdn8_slot = var_t1_rdn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_db0_slot = var_t2_db0;
        *var_t2_db1_slot = var_t2_db1;
        *var_t2_db2_slot = var_t2_db2;
        *var_t2_db3_slot = var_t2_db3;
        *var_t2_db4_slot = var_t2_db4;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rdb0_slot = var_t2_rdb0;
        *var_t2_rdb1_slot = var_t2_rdb1;
        *var_t2_rdb2_slot = var_t2_rdb2;
        *var_t2_rdb3_slot = var_t2_rdb3;
        *var_t2_rdb4_slot = var_t2_rdb4;
        *var_t2_rdn0_slot = var_t2_rdn0;
        *var_t2_rdn1_slot = var_t2_rdn1;
        *var_t2_rdn2_slot = var_t2_rdn2;
        *var_t2_rdn3_slot = var_t2_rdn3;
        *var_t2_rdn4_slot = var_t2_rdn4;
        *var_t2_rdn5_slot = var_t2_rdn5;
        *var_t2_rdn6_slot = var_t2_rdn6;
        *var_t2_rdn7_slot = var_t2_rdn7;
        *var_t2_rdn8_slot = var_t2_rdn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_db0_slot = var_t3_db0;
        *var_t3_db1_slot = var_t3_db1;
        *var_t3_db2_slot = var_t3_db2;
        *var_t3_db3_slot = var_t3_db3;
        *var_t3_db4_slot = var_t3_db4;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn1_slot = var_t3_dn1;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rdb0_slot = var_t3_rdb0;
        *var_t3_rdb1_slot = var_t3_rdb1;
        *var_t3_rdb2_slot = var_t3_rdb2;
        *var_t3_rdb3_slot = var_t3_rdb3;
        *var_t3_rdb4_slot = var_t3_rdb4;
        *var_t3_rdn0_slot = var_t3_rdn0;
        *var_t3_rdn1_slot = var_t3_rdn1;
        *var_t3_rdn2_slot = var_t3_rdn2;
        *var_t3_rdn3_slot = var_t3_rdn3;
        *var_t3_rdn4_slot = var_t3_rdn4;
        *var_t3_rdn5_slot = var_t3_rdn5;
        *var_t3_rdn6_slot = var_t3_rdn6;
        *var_t3_rdn7_slot = var_t3_rdn7;
        *var_t3_rdn8_slot = var_t3_rdn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_t6_slot = var_t6;
        *var_t6_db0_slot = var_t6_db0;
        *var_t6_db1_slot = var_t6_db1;
        *var_t6_db2_slot = var_t6_db2;
        *var_t6_db3_slot = var_t6_db3;
        *var_t6_db4_slot = var_t6_db4;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn1_slot = var_t6_dn1;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_rdb0_slot = var_t6_rdb0;
        *var_t6_rdb1_slot = var_t6_rdb1;
        *var_t6_rdb2_slot = var_t6_rdb2;
        *var_t6_rdb3_slot = var_t6_rdb3;
        *var_t6_rdb4_slot = var_t6_rdb4;
        *var_t6_rdn0_slot = var_t6_rdn0;
        *var_t6_rdn1_slot = var_t6_rdn1;
        *var_t6_rdn2_slot = var_t6_rdn2;
        *var_t6_rdn3_slot = var_t6_rdn3;
        *var_t6_rdn4_slot = var_t6_rdn4;
        *var_t6_rdn5_slot = var_t6_rdn5;
        *var_t6_rdn6_slot = var_t6_rdn6;
        *var_t6_rdn7_slot = var_t6_rdn7;
        *var_t6_rdn8_slot = var_t6_rdn8;
        *var_t6_rv_slot = var_t6_rv;
    }

    pub(super) fn stamp_reactive_block_87(
        p: &Parameters,
        var_diffvds: f64,
        var_diffvds_db0: f64,
        var_diffvds_db1: f64,
        var_diffvds_db2: f64,
        var_diffvds_db3: f64,
        var_diffvds_db4: f64,
        var_diffvds_dn0: f64,
        var_diffvds_dn1: f64,
        var_diffvds_dn2: f64,
        var_diffvds_dn3: f64,
        var_diffvds_dn4: f64,
        var_diffvds_dn5: f64,
        var_diffvds_dn6: f64,
        var_diffvds_dn7: f64,
        var_diffvds_dn8: f64,
        var_esatnoi: f64,
        var_esatnoi_db0: f64,
        var_esatnoi_db1: f64,
        var_esatnoi_db2: f64,
        var_esatnoi_db3: f64,
        var_esatnoi_db4: f64,
        var_esatnoi_dn0: f64,
        var_esatnoi_dn1: f64,
        var_esatnoi_dn2: f64,
        var_esatnoi_dn3: f64,
        var_esatnoi_dn4: f64,
        var_esatnoi_dn5: f64,
        var_esatnoi_dn6: f64,
        var_esatnoi_dn7: f64,
        var_esatnoi_dn8: f64,
        var_guard131: f64,
        var_leff: f64,
        var_leff_db0: f64,
        var_leff_db1: f64,
        var_leff_db2: f64,
        var_leff_db3: f64,
        var_leff_db4: f64,
        var_leff_dn0: f64,
        var_leff_dn1: f64,
        var_leff_dn2: f64,
        var_leff_dn3: f64,
        var_leff_dn4: f64,
        var_leff_dn5: f64,
        var_leff_dn6: f64,
        var_leff_dn7: f64,
        var_leff_dn8: f64,
        var_lintnoi_i: f64,
        var_lintnoi_i_db0: f64,
        var_lintnoi_i_db1: f64,
        var_lintnoi_i_db2: f64,
        var_lintnoi_i_db3: f64,
        var_lintnoi_i_db4: f64,
        var_lintnoi_i_dn0: f64,
        var_lintnoi_i_dn1: f64,
        var_lintnoi_i_dn2: f64,
        var_lintnoi_i_dn3: f64,
        var_lintnoi_i_dn4: f64,
        var_lintnoi_i_dn5: f64,
        var_lintnoi_i_dn6: f64,
        var_lintnoi_i_dn7: f64,
        var_lintnoi_i_dn8: f64,
        var_litl: f64,
        var_litl_db0: f64,
        var_litl_db1: f64,
        var_litl_db2: f64,
        var_litl_db3: f64,
        var_litl_db4: f64,
        var_litl_dn0: f64,
        var_litl_dn1: f64,
        var_litl_dn2: f64,
        var_litl_dn3: f64,
        var_litl_dn4: f64,
        var_litl_dn5: f64,
        var_litl_dn6: f64,
        var_litl_dn7: f64,
        var_litl_dn8: f64,
        var_mpower_i: f64,
        var_mpower_i_db0: f64,
        var_mpower_i_db1: f64,
        var_mpower_i_db2: f64,
        var_mpower_i_db3: f64,
        var_mpower_i_db4: f64,
        var_mpower_i_dn0: f64,
        var_mpower_i_dn1: f64,
        var_mpower_i_dn2: f64,
        var_mpower_i_dn3: f64,
        var_mpower_i_dn4: f64,
        var_mpower_i_dn5: f64,
        var_mpower_i_dn6: f64,
        var_mpower_i_dn7: f64,
        var_mpower_i_dn8: f64,
        var_noia2_i: f64,
        var_noia2_i_db0: f64,
        var_noia2_i_db1: f64,
        var_noia2_i_db2: f64,
        var_noia2_i_db3: f64,
        var_noia2_i_db4: f64,
        var_noia2_i_dn0: f64,
        var_noia2_i_dn1: f64,
        var_noia2_i_dn2: f64,
        var_noia2_i_dn3: f64,
        var_noia2_i_dn4: f64,
        var_noia2_i_dn5: f64,
        var_noia2_i_dn6: f64,
        var_noia2_i_dn7: f64,
        var_noia2_i_dn8: f64,
        var_qia2: f64,
        var_qia2_db0: f64,
        var_qia2_db1: f64,
        var_qia2_db2: f64,
        var_qia2_db3: f64,
        var_qia2_db4: f64,
        var_qia2_dn0: f64,
        var_qia2_dn1: f64,
        var_qia2_dn2: f64,
        var_qia2_dn3: f64,
        var_qia2_dn4: f64,
        var_qia2_dn5: f64,
        var_qia2_dn6: f64,
        var_qia2_dn7: f64,
        var_qia2_dn8: f64,
        var_qsref_i: f64,
        var_qsref_i_db0: f64,
        var_qsref_i_db1: f64,
        var_qsref_i_db2: f64,
        var_qsref_i_db3: f64,
        var_qsref_i_db4: f64,
        var_qsref_i_dn0: f64,
        var_qsref_i_dn1: f64,
        var_qsref_i_dn2: f64,
        var_qsref_i_dn3: f64,
        var_qsref_i_dn4: f64,
        var_qsref_i_dn5: f64,
        var_qsref_i_dn6: f64,
        var_qsref_i_dn7: f64,
        var_qsref_i_dn8: f64,
        var_delclm_slot: &mut f64,
        var_delclm_db0_slot: &mut f64,
        var_delclm_db1_slot: &mut f64,
        var_delclm_db2_slot: &mut f64,
        var_delclm_db3_slot: &mut f64,
        var_delclm_db4_slot: &mut f64,
        var_delclm_dn0_slot: &mut f64,
        var_delclm_dn1_slot: &mut f64,
        var_delclm_dn2_slot: &mut f64,
        var_delclm_dn3_slot: &mut f64,
        var_delclm_dn4_slot: &mut f64,
        var_delclm_dn5_slot: &mut f64,
        var_delclm_dn6_slot: &mut f64,
        var_delclm_dn7_slot: &mut f64,
        var_delclm_dn8_slot: &mut f64,
        var_delclm_rdb0_slot: &mut f64,
        var_delclm_rdb1_slot: &mut f64,
        var_delclm_rdb2_slot: &mut f64,
        var_delclm_rdb3_slot: &mut f64,
        var_delclm_rdb4_slot: &mut f64,
        var_delclm_rdn0_slot: &mut f64,
        var_delclm_rdn1_slot: &mut f64,
        var_delclm_rdn2_slot: &mut f64,
        var_delclm_rdn3_slot: &mut f64,
        var_delclm_rdn4_slot: &mut f64,
        var_delclm_rdn5_slot: &mut f64,
        var_delclm_rdn6_slot: &mut f64,
        var_delclm_rdn7_slot: &mut f64,
        var_delclm_rdn8_slot: &mut f64,
        var_delclm_rv_slot: &mut f64,
        var_guard132_slot: &mut f64,
        var_guard132_db0_slot: &mut f64,
        var_guard132_db1_slot: &mut f64,
        var_guard132_db2_slot: &mut f64,
        var_guard132_db3_slot: &mut f64,
        var_guard132_db4_slot: &mut f64,
        var_guard132_dn0_slot: &mut f64,
        var_guard132_dn1_slot: &mut f64,
        var_guard132_dn2_slot: &mut f64,
        var_guard132_dn3_slot: &mut f64,
        var_guard132_dn4_slot: &mut f64,
        var_guard132_dn5_slot: &mut f64,
        var_guard132_dn6_slot: &mut f64,
        var_guard132_dn7_slot: &mut f64,
        var_guard132_dn8_slot: &mut f64,
        var_guard132_rdb0_slot: &mut f64,
        var_guard132_rdb1_slot: &mut f64,
        var_guard132_rdb2_slot: &mut f64,
        var_guard132_rdb3_slot: &mut f64,
        var_guard132_rdb4_slot: &mut f64,
        var_guard132_rdn0_slot: &mut f64,
        var_guard132_rdn1_slot: &mut f64,
        var_guard132_rdn2_slot: &mut f64,
        var_guard132_rdn3_slot: &mut f64,
        var_guard132_rdn4_slot: &mut f64,
        var_guard132_rdn5_slot: &mut f64,
        var_guard132_rdn6_slot: &mut f64,
        var_guard132_rdn7_slot: &mut f64,
        var_guard132_rdn8_slot: &mut f64,
        var_guard132_rv_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard133_db0_slot: &mut f64,
        var_guard133_db1_slot: &mut f64,
        var_guard133_db2_slot: &mut f64,
        var_guard133_db3_slot: &mut f64,
        var_guard133_db4_slot: &mut f64,
        var_guard133_dn0_slot: &mut f64,
        var_guard133_dn1_slot: &mut f64,
        var_guard133_dn2_slot: &mut f64,
        var_guard133_dn3_slot: &mut f64,
        var_guard133_dn4_slot: &mut f64,
        var_guard133_dn5_slot: &mut f64,
        var_guard133_dn6_slot: &mut f64,
        var_guard133_dn7_slot: &mut f64,
        var_guard133_dn8_slot: &mut f64,
        var_guard133_rdb0_slot: &mut f64,
        var_guard133_rdb1_slot: &mut f64,
        var_guard133_rdb2_slot: &mut f64,
        var_guard133_rdb3_slot: &mut f64,
        var_guard133_rdb4_slot: &mut f64,
        var_guard133_rdn0_slot: &mut f64,
        var_guard133_rdn1_slot: &mut f64,
        var_guard133_rdn2_slot: &mut f64,
        var_guard133_rdn3_slot: &mut f64,
        var_guard133_rdn4_slot: &mut f64,
        var_guard133_rdn5_slot: &mut f64,
        var_guard133_rdn6_slot: &mut f64,
        var_guard133_rdn7_slot: &mut f64,
        var_guard133_rdn8_slot: &mut f64,
        var_guard133_rv_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_guard134_db0_slot: &mut f64,
        var_guard134_db1_slot: &mut f64,
        var_guard134_db2_slot: &mut f64,
        var_guard134_db3_slot: &mut f64,
        var_guard134_db4_slot: &mut f64,
        var_guard134_dn0_slot: &mut f64,
        var_guard134_dn1_slot: &mut f64,
        var_guard134_dn2_slot: &mut f64,
        var_guard134_dn3_slot: &mut f64,
        var_guard134_dn4_slot: &mut f64,
        var_guard134_dn5_slot: &mut f64,
        var_guard134_dn6_slot: &mut f64,
        var_guard134_dn7_slot: &mut f64,
        var_guard134_dn8_slot: &mut f64,
        var_guard134_rdb0_slot: &mut f64,
        var_guard134_rdb1_slot: &mut f64,
        var_guard134_rdb2_slot: &mut f64,
        var_guard134_rdb3_slot: &mut f64,
        var_guard134_rdb4_slot: &mut f64,
        var_guard134_rdn0_slot: &mut f64,
        var_guard134_rdn1_slot: &mut f64,
        var_guard134_rdn2_slot: &mut f64,
        var_guard134_rdn3_slot: &mut f64,
        var_guard134_rdn4_slot: &mut f64,
        var_guard134_rdn5_slot: &mut f64,
        var_guard134_rdn6_slot: &mut f64,
        var_guard134_rdn7_slot: &mut f64,
        var_guard134_rdn8_slot: &mut f64,
        var_guard134_rv_slot: &mut f64,
        var_leffnoi_slot: &mut f64,
        var_leffnoi_db0_slot: &mut f64,
        var_leffnoi_db1_slot: &mut f64,
        var_leffnoi_db2_slot: &mut f64,
        var_leffnoi_db3_slot: &mut f64,
        var_leffnoi_db4_slot: &mut f64,
        var_leffnoi_dn0_slot: &mut f64,
        var_leffnoi_dn1_slot: &mut f64,
        var_leffnoi_dn2_slot: &mut f64,
        var_leffnoi_dn3_slot: &mut f64,
        var_leffnoi_dn4_slot: &mut f64,
        var_leffnoi_dn5_slot: &mut f64,
        var_leffnoi_dn6_slot: &mut f64,
        var_leffnoi_dn7_slot: &mut f64,
        var_leffnoi_dn8_slot: &mut f64,
        var_leffnoi_rdb0_slot: &mut f64,
        var_leffnoi_rdb1_slot: &mut f64,
        var_leffnoi_rdb2_slot: &mut f64,
        var_leffnoi_rdb3_slot: &mut f64,
        var_leffnoi_rdb4_slot: &mut f64,
        var_leffnoi_rdn0_slot: &mut f64,
        var_leffnoi_rdn1_slot: &mut f64,
        var_leffnoi_rdn2_slot: &mut f64,
        var_leffnoi_rdn3_slot: &mut f64,
        var_leffnoi_rdn4_slot: &mut f64,
        var_leffnoi_rdn5_slot: &mut f64,
        var_leffnoi_rdn6_slot: &mut f64,
        var_leffnoi_rdn7_slot: &mut f64,
        var_leffnoi_rdn8_slot: &mut f64,
        var_leffnoi_rv_slot: &mut f64,
        var_leffnoisq_slot: &mut f64,
        var_leffnoisq_db0_slot: &mut f64,
        var_leffnoisq_db1_slot: &mut f64,
        var_leffnoisq_db2_slot: &mut f64,
        var_leffnoisq_db3_slot: &mut f64,
        var_leffnoisq_db4_slot: &mut f64,
        var_leffnoisq_dn0_slot: &mut f64,
        var_leffnoisq_dn1_slot: &mut f64,
        var_leffnoisq_dn2_slot: &mut f64,
        var_leffnoisq_dn3_slot: &mut f64,
        var_leffnoisq_dn4_slot: &mut f64,
        var_leffnoisq_dn5_slot: &mut f64,
        var_leffnoisq_dn6_slot: &mut f64,
        var_leffnoisq_dn7_slot: &mut f64,
        var_leffnoisq_dn8_slot: &mut f64,
        var_leffnoisq_rdb0_slot: &mut f64,
        var_leffnoisq_rdb1_slot: &mut f64,
        var_leffnoisq_rdb2_slot: &mut f64,
        var_leffnoisq_rdb3_slot: &mut f64,
        var_leffnoisq_rdb4_slot: &mut f64,
        var_leffnoisq_rdn0_slot: &mut f64,
        var_leffnoisq_rdn1_slot: &mut f64,
        var_leffnoisq_rdn2_slot: &mut f64,
        var_leffnoisq_rdn3_slot: &mut f64,
        var_leffnoisq_rdn4_slot: &mut f64,
        var_leffnoisq_rdn5_slot: &mut f64,
        var_leffnoisq_rdn6_slot: &mut f64,
        var_leffnoisq_rdn7_slot: &mut f64,
        var_leffnoisq_rdn8_slot: &mut f64,
        var_leffnoisq_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_db0_slot: &mut f64,
        var_t0_db1_slot: &mut f64,
        var_t0_db2_slot: &mut f64,
        var_t0_db3_slot: &mut f64,
        var_t0_db4_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rdb0_slot: &mut f64,
        var_t0_rdb1_slot: &mut f64,
        var_t0_rdb2_slot: &mut f64,
        var_t0_rdb3_slot: &mut f64,
        var_t0_rdb4_slot: &mut f64,
        var_t0_rdn0_slot: &mut f64,
        var_t0_rdn1_slot: &mut f64,
        var_t0_rdn2_slot: &mut f64,
        var_t0_rdn3_slot: &mut f64,
        var_t0_rdn4_slot: &mut f64,
        var_t0_rdn5_slot: &mut f64,
        var_t0_rdn6_slot: &mut f64,
        var_t0_rdn7_slot: &mut f64,
        var_t0_rdn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_db0_slot: &mut f64,
        var_t1_db1_slot: &mut f64,
        var_t1_db2_slot: &mut f64,
        var_t1_db3_slot: &mut f64,
        var_t1_db4_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rdb0_slot: &mut f64,
        var_t1_rdb1_slot: &mut f64,
        var_t1_rdb2_slot: &mut f64,
        var_t1_rdb3_slot: &mut f64,
        var_t1_rdb4_slot: &mut f64,
        var_t1_rdn0_slot: &mut f64,
        var_t1_rdn1_slot: &mut f64,
        var_t1_rdn2_slot: &mut f64,
        var_t1_rdn3_slot: &mut f64,
        var_t1_rdn4_slot: &mut f64,
        var_t1_rdn5_slot: &mut f64,
        var_t1_rdn6_slot: &mut f64,
        var_t1_rdn7_slot: &mut f64,
        var_t1_rdn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_db0_slot: &mut f64,
        var_t2_db1_slot: &mut f64,
        var_t2_db2_slot: &mut f64,
        var_t2_db3_slot: &mut f64,
        var_t2_db4_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rdb0_slot: &mut f64,
        var_t2_rdb1_slot: &mut f64,
        var_t2_rdb2_slot: &mut f64,
        var_t2_rdb3_slot: &mut f64,
        var_t2_rdb4_slot: &mut f64,
        var_t2_rdn0_slot: &mut f64,
        var_t2_rdn1_slot: &mut f64,
        var_t2_rdn2_slot: &mut f64,
        var_t2_rdn3_slot: &mut f64,
        var_t2_rdn4_slot: &mut f64,
        var_t2_rdn5_slot: &mut f64,
        var_t2_rdn6_slot: &mut f64,
        var_t2_rdn7_slot: &mut f64,
        var_t2_rdn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_db0_slot: &mut f64,
        var_t3_db1_slot: &mut f64,
        var_t3_db2_slot: &mut f64,
        var_t3_db3_slot: &mut f64,
        var_t3_db4_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn1_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rdb0_slot: &mut f64,
        var_t3_rdb1_slot: &mut f64,
        var_t3_rdb2_slot: &mut f64,
        var_t3_rdb3_slot: &mut f64,
        var_t3_rdb4_slot: &mut f64,
        var_t3_rdn0_slot: &mut f64,
        var_t3_rdn1_slot: &mut f64,
        var_t3_rdn2_slot: &mut f64,
        var_t3_rdn3_slot: &mut f64,
        var_t3_rdn4_slot: &mut f64,
        var_t3_rdn5_slot: &mut f64,
        var_t3_rdn6_slot: &mut f64,
        var_t3_rdn7_slot: &mut f64,
        var_t3_rdn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_db0_slot: &mut f64,
        var_t4_db1_slot: &mut f64,
        var_t4_db2_slot: &mut f64,
        var_t4_db3_slot: &mut f64,
        var_t4_db4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn1_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_rdb0_slot: &mut f64,
        var_t4_rdb1_slot: &mut f64,
        var_t4_rdb2_slot: &mut f64,
        var_t4_rdb3_slot: &mut f64,
        var_t4_rdb4_slot: &mut f64,
        var_t4_rdn0_slot: &mut f64,
        var_t4_rdn1_slot: &mut f64,
        var_t4_rdn2_slot: &mut f64,
        var_t4_rdn3_slot: &mut f64,
        var_t4_rdn4_slot: &mut f64,
        var_t4_rdn5_slot: &mut f64,
        var_t4_rdn6_slot: &mut f64,
        var_t4_rdn7_slot: &mut f64,
        var_t4_rdn8_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
    ) {
        let mut var_delclm: f64 = *var_delclm_slot;
        let mut var_delclm_db0: f64 = *var_delclm_db0_slot;
        let mut var_delclm_db1: f64 = *var_delclm_db1_slot;
        let mut var_delclm_db2: f64 = *var_delclm_db2_slot;
        let mut var_delclm_db3: f64 = *var_delclm_db3_slot;
        let mut var_delclm_db4: f64 = *var_delclm_db4_slot;
        let mut var_delclm_dn0: f64 = *var_delclm_dn0_slot;
        let mut var_delclm_dn1: f64 = *var_delclm_dn1_slot;
        let mut var_delclm_dn2: f64 = *var_delclm_dn2_slot;
        let mut var_delclm_dn3: f64 = *var_delclm_dn3_slot;
        let mut var_delclm_dn4: f64 = *var_delclm_dn4_slot;
        let mut var_delclm_dn5: f64 = *var_delclm_dn5_slot;
        let mut var_delclm_dn6: f64 = *var_delclm_dn6_slot;
        let mut var_delclm_dn7: f64 = *var_delclm_dn7_slot;
        let mut var_delclm_dn8: f64 = *var_delclm_dn8_slot;
        let mut var_delclm_rdb0: f64 = *var_delclm_rdb0_slot;
        let mut var_delclm_rdb1: f64 = *var_delclm_rdb1_slot;
        let mut var_delclm_rdb2: f64 = *var_delclm_rdb2_slot;
        let mut var_delclm_rdb3: f64 = *var_delclm_rdb3_slot;
        let mut var_delclm_rdb4: f64 = *var_delclm_rdb4_slot;
        let mut var_delclm_rdn0: f64 = *var_delclm_rdn0_slot;
        let mut var_delclm_rdn1: f64 = *var_delclm_rdn1_slot;
        let mut var_delclm_rdn2: f64 = *var_delclm_rdn2_slot;
        let mut var_delclm_rdn3: f64 = *var_delclm_rdn3_slot;
        let mut var_delclm_rdn4: f64 = *var_delclm_rdn4_slot;
        let mut var_delclm_rdn5: f64 = *var_delclm_rdn5_slot;
        let mut var_delclm_rdn6: f64 = *var_delclm_rdn6_slot;
        let mut var_delclm_rdn7: f64 = *var_delclm_rdn7_slot;
        let mut var_delclm_rdn8: f64 = *var_delclm_rdn8_slot;
        let mut var_delclm_rv: f64 = *var_delclm_rv_slot;
        let mut var_guard132: f64 = *var_guard132_slot;
        let mut var_guard132_db0: f64 = *var_guard132_db0_slot;
        let mut var_guard132_db1: f64 = *var_guard132_db1_slot;
        let mut var_guard132_db2: f64 = *var_guard132_db2_slot;
        let mut var_guard132_db3: f64 = *var_guard132_db3_slot;
        let mut var_guard132_db4: f64 = *var_guard132_db4_slot;
        let mut var_guard132_dn0: f64 = *var_guard132_dn0_slot;
        let mut var_guard132_dn1: f64 = *var_guard132_dn1_slot;
        let mut var_guard132_dn2: f64 = *var_guard132_dn2_slot;
        let mut var_guard132_dn3: f64 = *var_guard132_dn3_slot;
        let mut var_guard132_dn4: f64 = *var_guard132_dn4_slot;
        let mut var_guard132_dn5: f64 = *var_guard132_dn5_slot;
        let mut var_guard132_dn6: f64 = *var_guard132_dn6_slot;
        let mut var_guard132_dn7: f64 = *var_guard132_dn7_slot;
        let mut var_guard132_dn8: f64 = *var_guard132_dn8_slot;
        let mut var_guard132_rdb0: f64 = *var_guard132_rdb0_slot;
        let mut var_guard132_rdb1: f64 = *var_guard132_rdb1_slot;
        let mut var_guard132_rdb2: f64 = *var_guard132_rdb2_slot;
        let mut var_guard132_rdb3: f64 = *var_guard132_rdb3_slot;
        let mut var_guard132_rdb4: f64 = *var_guard132_rdb4_slot;
        let mut var_guard132_rdn0: f64 = *var_guard132_rdn0_slot;
        let mut var_guard132_rdn1: f64 = *var_guard132_rdn1_slot;
        let mut var_guard132_rdn2: f64 = *var_guard132_rdn2_slot;
        let mut var_guard132_rdn3: f64 = *var_guard132_rdn3_slot;
        let mut var_guard132_rdn4: f64 = *var_guard132_rdn4_slot;
        let mut var_guard132_rdn5: f64 = *var_guard132_rdn5_slot;
        let mut var_guard132_rdn6: f64 = *var_guard132_rdn6_slot;
        let mut var_guard132_rdn7: f64 = *var_guard132_rdn7_slot;
        let mut var_guard132_rdn8: f64 = *var_guard132_rdn8_slot;
        let mut var_guard132_rv: f64 = *var_guard132_rv_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard133_db0: f64 = *var_guard133_db0_slot;
        let mut var_guard133_db1: f64 = *var_guard133_db1_slot;
        let mut var_guard133_db2: f64 = *var_guard133_db2_slot;
        let mut var_guard133_db3: f64 = *var_guard133_db3_slot;
        let mut var_guard133_db4: f64 = *var_guard133_db4_slot;
        let mut var_guard133_dn0: f64 = *var_guard133_dn0_slot;
        let mut var_guard133_dn1: f64 = *var_guard133_dn1_slot;
        let mut var_guard133_dn2: f64 = *var_guard133_dn2_slot;
        let mut var_guard133_dn3: f64 = *var_guard133_dn3_slot;
        let mut var_guard133_dn4: f64 = *var_guard133_dn4_slot;
        let mut var_guard133_dn5: f64 = *var_guard133_dn5_slot;
        let mut var_guard133_dn6: f64 = *var_guard133_dn6_slot;
        let mut var_guard133_dn7: f64 = *var_guard133_dn7_slot;
        let mut var_guard133_dn8: f64 = *var_guard133_dn8_slot;
        let mut var_guard133_rdb0: f64 = *var_guard133_rdb0_slot;
        let mut var_guard133_rdb1: f64 = *var_guard133_rdb1_slot;
        let mut var_guard133_rdb2: f64 = *var_guard133_rdb2_slot;
        let mut var_guard133_rdb3: f64 = *var_guard133_rdb3_slot;
        let mut var_guard133_rdb4: f64 = *var_guard133_rdb4_slot;
        let mut var_guard133_rdn0: f64 = *var_guard133_rdn0_slot;
        let mut var_guard133_rdn1: f64 = *var_guard133_rdn1_slot;
        let mut var_guard133_rdn2: f64 = *var_guard133_rdn2_slot;
        let mut var_guard133_rdn3: f64 = *var_guard133_rdn3_slot;
        let mut var_guard133_rdn4: f64 = *var_guard133_rdn4_slot;
        let mut var_guard133_rdn5: f64 = *var_guard133_rdn5_slot;
        let mut var_guard133_rdn6: f64 = *var_guard133_rdn6_slot;
        let mut var_guard133_rdn7: f64 = *var_guard133_rdn7_slot;
        let mut var_guard133_rdn8: f64 = *var_guard133_rdn8_slot;
        let mut var_guard133_rv: f64 = *var_guard133_rv_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_guard134_db0: f64 = *var_guard134_db0_slot;
        let mut var_guard134_db1: f64 = *var_guard134_db1_slot;
        let mut var_guard134_db2: f64 = *var_guard134_db2_slot;
        let mut var_guard134_db3: f64 = *var_guard134_db3_slot;
        let mut var_guard134_db4: f64 = *var_guard134_db4_slot;
        let mut var_guard134_dn0: f64 = *var_guard134_dn0_slot;
        let mut var_guard134_dn1: f64 = *var_guard134_dn1_slot;
        let mut var_guard134_dn2: f64 = *var_guard134_dn2_slot;
        let mut var_guard134_dn3: f64 = *var_guard134_dn3_slot;
        let mut var_guard134_dn4: f64 = *var_guard134_dn4_slot;
        let mut var_guard134_dn5: f64 = *var_guard134_dn5_slot;
        let mut var_guard134_dn6: f64 = *var_guard134_dn6_slot;
        let mut var_guard134_dn7: f64 = *var_guard134_dn7_slot;
        let mut var_guard134_dn8: f64 = *var_guard134_dn8_slot;
        let mut var_guard134_rdb0: f64 = *var_guard134_rdb0_slot;
        let mut var_guard134_rdb1: f64 = *var_guard134_rdb1_slot;
        let mut var_guard134_rdb2: f64 = *var_guard134_rdb2_slot;
        let mut var_guard134_rdb3: f64 = *var_guard134_rdb3_slot;
        let mut var_guard134_rdb4: f64 = *var_guard134_rdb4_slot;
        let mut var_guard134_rdn0: f64 = *var_guard134_rdn0_slot;
        let mut var_guard134_rdn1: f64 = *var_guard134_rdn1_slot;
        let mut var_guard134_rdn2: f64 = *var_guard134_rdn2_slot;
        let mut var_guard134_rdn3: f64 = *var_guard134_rdn3_slot;
        let mut var_guard134_rdn4: f64 = *var_guard134_rdn4_slot;
        let mut var_guard134_rdn5: f64 = *var_guard134_rdn5_slot;
        let mut var_guard134_rdn6: f64 = *var_guard134_rdn6_slot;
        let mut var_guard134_rdn7: f64 = *var_guard134_rdn7_slot;
        let mut var_guard134_rdn8: f64 = *var_guard134_rdn8_slot;
        let mut var_guard134_rv: f64 = *var_guard134_rv_slot;
        let mut var_leffnoi: f64 = *var_leffnoi_slot;
        let mut var_leffnoi_db0: f64 = *var_leffnoi_db0_slot;
        let mut var_leffnoi_db1: f64 = *var_leffnoi_db1_slot;
        let mut var_leffnoi_db2: f64 = *var_leffnoi_db2_slot;
        let mut var_leffnoi_db3: f64 = *var_leffnoi_db3_slot;
        let mut var_leffnoi_db4: f64 = *var_leffnoi_db4_slot;
        let mut var_leffnoi_dn0: f64 = *var_leffnoi_dn0_slot;
        let mut var_leffnoi_dn1: f64 = *var_leffnoi_dn1_slot;
        let mut var_leffnoi_dn2: f64 = *var_leffnoi_dn2_slot;
        let mut var_leffnoi_dn3: f64 = *var_leffnoi_dn3_slot;
        let mut var_leffnoi_dn4: f64 = *var_leffnoi_dn4_slot;
        let mut var_leffnoi_dn5: f64 = *var_leffnoi_dn5_slot;
        let mut var_leffnoi_dn6: f64 = *var_leffnoi_dn6_slot;
        let mut var_leffnoi_dn7: f64 = *var_leffnoi_dn7_slot;
        let mut var_leffnoi_dn8: f64 = *var_leffnoi_dn8_slot;
        let mut var_leffnoi_rdb0: f64 = *var_leffnoi_rdb0_slot;
        let mut var_leffnoi_rdb1: f64 = *var_leffnoi_rdb1_slot;
        let mut var_leffnoi_rdb2: f64 = *var_leffnoi_rdb2_slot;
        let mut var_leffnoi_rdb3: f64 = *var_leffnoi_rdb3_slot;
        let mut var_leffnoi_rdb4: f64 = *var_leffnoi_rdb4_slot;
        let mut var_leffnoi_rdn0: f64 = *var_leffnoi_rdn0_slot;
        let mut var_leffnoi_rdn1: f64 = *var_leffnoi_rdn1_slot;
        let mut var_leffnoi_rdn2: f64 = *var_leffnoi_rdn2_slot;
        let mut var_leffnoi_rdn3: f64 = *var_leffnoi_rdn3_slot;
        let mut var_leffnoi_rdn4: f64 = *var_leffnoi_rdn4_slot;
        let mut var_leffnoi_rdn5: f64 = *var_leffnoi_rdn5_slot;
        let mut var_leffnoi_rdn6: f64 = *var_leffnoi_rdn6_slot;
        let mut var_leffnoi_rdn7: f64 = *var_leffnoi_rdn7_slot;
        let mut var_leffnoi_rdn8: f64 = *var_leffnoi_rdn8_slot;
        let mut var_leffnoi_rv: f64 = *var_leffnoi_rv_slot;
        let mut var_leffnoisq: f64 = *var_leffnoisq_slot;
        let mut var_leffnoisq_db0: f64 = *var_leffnoisq_db0_slot;
        let mut var_leffnoisq_db1: f64 = *var_leffnoisq_db1_slot;
        let mut var_leffnoisq_db2: f64 = *var_leffnoisq_db2_slot;
        let mut var_leffnoisq_db3: f64 = *var_leffnoisq_db3_slot;
        let mut var_leffnoisq_db4: f64 = *var_leffnoisq_db4_slot;
        let mut var_leffnoisq_dn0: f64 = *var_leffnoisq_dn0_slot;
        let mut var_leffnoisq_dn1: f64 = *var_leffnoisq_dn1_slot;
        let mut var_leffnoisq_dn2: f64 = *var_leffnoisq_dn2_slot;
        let mut var_leffnoisq_dn3: f64 = *var_leffnoisq_dn3_slot;
        let mut var_leffnoisq_dn4: f64 = *var_leffnoisq_dn4_slot;
        let mut var_leffnoisq_dn5: f64 = *var_leffnoisq_dn5_slot;
        let mut var_leffnoisq_dn6: f64 = *var_leffnoisq_dn6_slot;
        let mut var_leffnoisq_dn7: f64 = *var_leffnoisq_dn7_slot;
        let mut var_leffnoisq_dn8: f64 = *var_leffnoisq_dn8_slot;
        let mut var_leffnoisq_rdb0: f64 = *var_leffnoisq_rdb0_slot;
        let mut var_leffnoisq_rdb1: f64 = *var_leffnoisq_rdb1_slot;
        let mut var_leffnoisq_rdb2: f64 = *var_leffnoisq_rdb2_slot;
        let mut var_leffnoisq_rdb3: f64 = *var_leffnoisq_rdb3_slot;
        let mut var_leffnoisq_rdb4: f64 = *var_leffnoisq_rdb4_slot;
        let mut var_leffnoisq_rdn0: f64 = *var_leffnoisq_rdn0_slot;
        let mut var_leffnoisq_rdn1: f64 = *var_leffnoisq_rdn1_slot;
        let mut var_leffnoisq_rdn2: f64 = *var_leffnoisq_rdn2_slot;
        let mut var_leffnoisq_rdn3: f64 = *var_leffnoisq_rdn3_slot;
        let mut var_leffnoisq_rdn4: f64 = *var_leffnoisq_rdn4_slot;
        let mut var_leffnoisq_rdn5: f64 = *var_leffnoisq_rdn5_slot;
        let mut var_leffnoisq_rdn6: f64 = *var_leffnoisq_rdn6_slot;
        let mut var_leffnoisq_rdn7: f64 = *var_leffnoisq_rdn7_slot;
        let mut var_leffnoisq_rdn8: f64 = *var_leffnoisq_rdn8_slot;
        let mut var_leffnoisq_rv: f64 = *var_leffnoisq_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_db0: f64 = *var_t0_db0_slot;
        let mut var_t0_db1: f64 = *var_t0_db1_slot;
        let mut var_t0_db2: f64 = *var_t0_db2_slot;
        let mut var_t0_db3: f64 = *var_t0_db3_slot;
        let mut var_t0_db4: f64 = *var_t0_db4_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rdb0: f64 = *var_t0_rdb0_slot;
        let mut var_t0_rdb1: f64 = *var_t0_rdb1_slot;
        let mut var_t0_rdb2: f64 = *var_t0_rdb2_slot;
        let mut var_t0_rdb3: f64 = *var_t0_rdb3_slot;
        let mut var_t0_rdb4: f64 = *var_t0_rdb4_slot;
        let mut var_t0_rdn0: f64 = *var_t0_rdn0_slot;
        let mut var_t0_rdn1: f64 = *var_t0_rdn1_slot;
        let mut var_t0_rdn2: f64 = *var_t0_rdn2_slot;
        let mut var_t0_rdn3: f64 = *var_t0_rdn3_slot;
        let mut var_t0_rdn4: f64 = *var_t0_rdn4_slot;
        let mut var_t0_rdn5: f64 = *var_t0_rdn5_slot;
        let mut var_t0_rdn6: f64 = *var_t0_rdn6_slot;
        let mut var_t0_rdn7: f64 = *var_t0_rdn7_slot;
        let mut var_t0_rdn8: f64 = *var_t0_rdn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_db0: f64 = *var_t1_db0_slot;
        let mut var_t1_db1: f64 = *var_t1_db1_slot;
        let mut var_t1_db2: f64 = *var_t1_db2_slot;
        let mut var_t1_db3: f64 = *var_t1_db3_slot;
        let mut var_t1_db4: f64 = *var_t1_db4_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rdb0: f64 = *var_t1_rdb0_slot;
        let mut var_t1_rdb1: f64 = *var_t1_rdb1_slot;
        let mut var_t1_rdb2: f64 = *var_t1_rdb2_slot;
        let mut var_t1_rdb3: f64 = *var_t1_rdb3_slot;
        let mut var_t1_rdb4: f64 = *var_t1_rdb4_slot;
        let mut var_t1_rdn0: f64 = *var_t1_rdn0_slot;
        let mut var_t1_rdn1: f64 = *var_t1_rdn1_slot;
        let mut var_t1_rdn2: f64 = *var_t1_rdn2_slot;
        let mut var_t1_rdn3: f64 = *var_t1_rdn3_slot;
        let mut var_t1_rdn4: f64 = *var_t1_rdn4_slot;
        let mut var_t1_rdn5: f64 = *var_t1_rdn5_slot;
        let mut var_t1_rdn6: f64 = *var_t1_rdn6_slot;
        let mut var_t1_rdn7: f64 = *var_t1_rdn7_slot;
        let mut var_t1_rdn8: f64 = *var_t1_rdn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_db0: f64 = *var_t2_db0_slot;
        let mut var_t2_db1: f64 = *var_t2_db1_slot;
        let mut var_t2_db2: f64 = *var_t2_db2_slot;
        let mut var_t2_db3: f64 = *var_t2_db3_slot;
        let mut var_t2_db4: f64 = *var_t2_db4_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rdb0: f64 = *var_t2_rdb0_slot;
        let mut var_t2_rdb1: f64 = *var_t2_rdb1_slot;
        let mut var_t2_rdb2: f64 = *var_t2_rdb2_slot;
        let mut var_t2_rdb3: f64 = *var_t2_rdb3_slot;
        let mut var_t2_rdb4: f64 = *var_t2_rdb4_slot;
        let mut var_t2_rdn0: f64 = *var_t2_rdn0_slot;
        let mut var_t2_rdn1: f64 = *var_t2_rdn1_slot;
        let mut var_t2_rdn2: f64 = *var_t2_rdn2_slot;
        let mut var_t2_rdn3: f64 = *var_t2_rdn3_slot;
        let mut var_t2_rdn4: f64 = *var_t2_rdn4_slot;
        let mut var_t2_rdn5: f64 = *var_t2_rdn5_slot;
        let mut var_t2_rdn6: f64 = *var_t2_rdn6_slot;
        let mut var_t2_rdn7: f64 = *var_t2_rdn7_slot;
        let mut var_t2_rdn8: f64 = *var_t2_rdn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_db0: f64 = *var_t3_db0_slot;
        let mut var_t3_db1: f64 = *var_t3_db1_slot;
        let mut var_t3_db2: f64 = *var_t3_db2_slot;
        let mut var_t3_db3: f64 = *var_t3_db3_slot;
        let mut var_t3_db4: f64 = *var_t3_db4_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn1: f64 = *var_t3_dn1_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rdb0: f64 = *var_t3_rdb0_slot;
        let mut var_t3_rdb1: f64 = *var_t3_rdb1_slot;
        let mut var_t3_rdb2: f64 = *var_t3_rdb2_slot;
        let mut var_t3_rdb3: f64 = *var_t3_rdb3_slot;
        let mut var_t3_rdb4: f64 = *var_t3_rdb4_slot;
        let mut var_t3_rdn0: f64 = *var_t3_rdn0_slot;
        let mut var_t3_rdn1: f64 = *var_t3_rdn1_slot;
        let mut var_t3_rdn2: f64 = *var_t3_rdn2_slot;
        let mut var_t3_rdn3: f64 = *var_t3_rdn3_slot;
        let mut var_t3_rdn4: f64 = *var_t3_rdn4_slot;
        let mut var_t3_rdn5: f64 = *var_t3_rdn5_slot;
        let mut var_t3_rdn6: f64 = *var_t3_rdn6_slot;
        let mut var_t3_rdn7: f64 = *var_t3_rdn7_slot;
        let mut var_t3_rdn8: f64 = *var_t3_rdn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_db0: f64 = *var_t4_db0_slot;
        let mut var_t4_db1: f64 = *var_t4_db1_slot;
        let mut var_t4_db2: f64 = *var_t4_db2_slot;
        let mut var_t4_db3: f64 = *var_t4_db3_slot;
        let mut var_t4_db4: f64 = *var_t4_db4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn1: f64 = *var_t4_dn1_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_rdb0: f64 = *var_t4_rdb0_slot;
        let mut var_t4_rdb1: f64 = *var_t4_rdb1_slot;
        let mut var_t4_rdb2: f64 = *var_t4_rdb2_slot;
        let mut var_t4_rdb3: f64 = *var_t4_rdb3_slot;
        let mut var_t4_rdb4: f64 = *var_t4_rdb4_slot;
        let mut var_t4_rdn0: f64 = *var_t4_rdn0_slot;
        let mut var_t4_rdn1: f64 = *var_t4_rdn1_slot;
        let mut var_t4_rdn2: f64 = *var_t4_rdn2_slot;
        let mut var_t4_rdn3: f64 = *var_t4_rdn3_slot;
        let mut var_t4_rdn4: f64 = *var_t4_rdn4_slot;
        let mut var_t4_rdn5: f64 = *var_t4_rdn5_slot;
        let mut var_t4_rdn6: f64 = *var_t4_rdn6_slot;
        let mut var_t4_rdn7: f64 = *var_t4_rdn7_slot;
        let mut var_t4_rdn8: f64 = *var_t4_rdn8_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;

        let (assign12700_e11805, assign12700_e11805_d_n0, assign12700_e11805_d_n1, assign12700_e11805_d_n2, assign12700_e11805_d_n3, assign12700_e11805_d_n4, assign12700_e11805_d_n5, assign12700_e11805_d_n6, assign12700_e11805_d_n7, assign12700_e11805_d_n8, assign12700_e11805_d_b0, assign12700_e11805_d_b1, assign12700_e11805_d_b2, assign12700_e11805_d_b3, assign12700_e11805_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12700_e11802: f64 = (2.0 * var_lintnoi_i);
        let assign12700_e11803: f64 = (var_leff - assign12700_e11802);
        (assign12700_e11803, (var_leff_dn0 - (2.0 * var_lintnoi_i_dn0)), (var_leff_dn1 - (2.0 * var_lintnoi_i_dn1)), (var_leff_dn2 - (2.0 * var_lintnoi_i_dn2)), (var_leff_dn3 - (2.0 * var_lintnoi_i_dn3)), (var_leff_dn4 - (2.0 * var_lintnoi_i_dn4)), (var_leff_dn5 - (2.0 * var_lintnoi_i_dn5)), (var_leff_dn6 - (2.0 * var_lintnoi_i_dn6)), (var_leff_dn7 - (2.0 * var_lintnoi_i_dn7)), (var_leff_dn8 - (2.0 * var_lintnoi_i_dn8)), (var_leff_db0 - (2.0 * var_lintnoi_i_db0)), (var_leff_db1 - (2.0 * var_lintnoi_i_db1)), (var_leff_db2 - (2.0 * var_lintnoi_i_db2)), (var_leff_db3 - (2.0 * var_lintnoi_i_db3)), (var_leff_db4 - (2.0 * var_lintnoi_i_db4)),)
    } else {
        (var_leffnoi, var_leffnoi_dn0, var_leffnoi_dn1, var_leffnoi_dn2, var_leffnoi_dn3, var_leffnoi_dn4, var_leffnoi_dn5, var_leffnoi_dn6, var_leffnoi_dn7, var_leffnoi_dn8, var_leffnoi_db0, var_leffnoi_db1, var_leffnoi_db2, var_leffnoi_db3, var_leffnoi_db4,)
    }
};
        var_leffnoi = assign12700_e11805;
        var_leffnoi_dn0 = assign12700_e11805_d_n0;
        var_leffnoi_dn1 = assign12700_e11805_d_n1;
        var_leffnoi_dn2 = assign12700_e11805_d_n2;
        var_leffnoi_dn3 = assign12700_e11805_d_n3;
        var_leffnoi_dn4 = assign12700_e11805_d_n4;
        var_leffnoi_dn5 = assign12700_e11805_d_n5;
        var_leffnoi_dn6 = assign12700_e11805_d_n6;
        var_leffnoi_dn7 = assign12700_e11805_d_n7;
        var_leffnoi_dn8 = assign12700_e11805_d_n8;
        var_leffnoi_db0 = assign12700_e11805_d_b0;
        var_leffnoi_db1 = assign12700_e11805_d_b1;
        var_leffnoi_db2 = assign12700_e11805_d_b2;
        var_leffnoi_db3 = assign12700_e11805_d_b3;
        var_leffnoi_db4 = assign12700_e11805_d_b4;
        var_leffnoi_rv = 0.0;
        var_leffnoi_rdn0 = 0.0;
        var_leffnoi_rdn1 = 0.0;
        var_leffnoi_rdn2 = 0.0;
        var_leffnoi_rdn3 = 0.0;
        var_leffnoi_rdn4 = 0.0;
        var_leffnoi_rdn5 = 0.0;
        var_leffnoi_rdn6 = 0.0;
        var_leffnoi_rdn7 = 0.0;
        var_leffnoi_rdn8 = 0.0;
        var_leffnoi_rdb0 = 0.0;
        var_leffnoi_rdb1 = 0.0;
        var_leffnoi_rdb2 = 0.0;
        var_leffnoi_rdb3 = 0.0;
        var_leffnoi_rdb4 = 0.0;

        let (assign12710_e11811, assign12710_e11811_d_n0, assign12710_e11811_d_n1, assign12710_e11811_d_n2, assign12710_e11811_d_n3, assign12710_e11811_d_n4, assign12710_e11811_d_n5, assign12710_e11811_d_n6, assign12710_e11811_d_n7, assign12710_e11811_d_n8, assign12710_e11811_d_b0, assign12710_e11811_d_b1, assign12710_e11811_d_b2, assign12710_e11811_d_b3, assign12710_e11811_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12710_e11809: f64 = (var_leffnoi * var_leffnoi);
        (assign12710_e11809, ((var_leffnoi_dn0 * var_leffnoi) + (var_leffnoi * var_leffnoi_dn0)), ((var_leffnoi_dn1 * var_leffnoi) + (var_leffnoi * var_leffnoi_dn1)), ((var_leffnoi_dn2 * var_leffnoi) + (var_leffnoi * var_leffnoi_dn2)), ((var_leffnoi_dn3 * var_leffnoi) + (var_leffnoi * var_leffnoi_dn3)), ((var_leffnoi_dn4 * var_leffnoi) + (var_leffnoi * var_leffnoi_dn4)), ((var_leffnoi_dn5 * var_leffnoi) + (var_leffnoi * var_leffnoi_dn5)), ((var_leffnoi_dn6 * var_leffnoi) + (var_leffnoi * var_leffnoi_dn6)), ((var_leffnoi_dn7 * var_leffnoi) + (var_leffnoi * var_leffnoi_dn7)), ((var_leffnoi_dn8 * var_leffnoi) + (var_leffnoi * var_leffnoi_dn8)), ((var_leffnoi_db0 * var_leffnoi) + (var_leffnoi * var_leffnoi_db0)), ((var_leffnoi_db1 * var_leffnoi) + (var_leffnoi * var_leffnoi_db1)), ((var_leffnoi_db2 * var_leffnoi) + (var_leffnoi * var_leffnoi_db2)), ((var_leffnoi_db3 * var_leffnoi) + (var_leffnoi * var_leffnoi_db3)), ((var_leffnoi_db4 * var_leffnoi) + (var_leffnoi * var_leffnoi_db4)),)
    } else {
        (var_leffnoisq, var_leffnoisq_dn0, var_leffnoisq_dn1, var_leffnoisq_dn2, var_leffnoisq_dn3, var_leffnoisq_dn4, var_leffnoisq_dn5, var_leffnoisq_dn6, var_leffnoisq_dn7, var_leffnoisq_dn8, var_leffnoisq_db0, var_leffnoisq_db1, var_leffnoisq_db2, var_leffnoisq_db3, var_leffnoisq_db4,)
    }
};
        var_leffnoisq = assign12710_e11811;
        var_leffnoisq_dn0 = assign12710_e11811_d_n0;
        var_leffnoisq_dn1 = assign12710_e11811_d_n1;
        var_leffnoisq_dn2 = assign12710_e11811_d_n2;
        var_leffnoisq_dn3 = assign12710_e11811_d_n3;
        var_leffnoisq_dn4 = assign12710_e11811_d_n4;
        var_leffnoisq_dn5 = assign12710_e11811_d_n5;
        var_leffnoisq_dn6 = assign12710_e11811_d_n6;
        var_leffnoisq_dn7 = assign12710_e11811_d_n7;
        var_leffnoisq_dn8 = assign12710_e11811_d_n8;
        var_leffnoisq_db0 = assign12710_e11811_d_b0;
        var_leffnoisq_db1 = assign12710_e11811_d_b1;
        var_leffnoisq_db2 = assign12710_e11811_d_b2;
        var_leffnoisq_db3 = assign12710_e11811_d_b3;
        var_leffnoisq_db4 = assign12710_e11811_d_b4;
        var_leffnoisq_rv = 0.0;
        var_leffnoisq_rdn0 = 0.0;
        var_leffnoisq_rdn1 = 0.0;
        var_leffnoisq_rdn2 = 0.0;
        var_leffnoisq_rdn3 = 0.0;
        var_leffnoisq_rdn4 = 0.0;
        var_leffnoisq_rdn5 = 0.0;
        var_leffnoisq_rdn6 = 0.0;
        var_leffnoisq_rdn7 = 0.0;
        var_leffnoisq_rdn8 = 0.0;
        var_leffnoisq_rdb0 = 0.0;
        var_leffnoisq_rdb1 = 0.0;
        var_leffnoisq_rdb2 = 0.0;
        var_leffnoisq_rdb3 = 0.0;
        var_leffnoisq_rdb4 = 0.0;

        let assign12720_e11814: f64 = if p.p287 <= 0.0 { 1.0 } else { 0.0 };
        var_guard132 = assign12720_e11814;
        var_guard132_dn0 = 0.0;
        var_guard132_dn1 = 0.0;
        var_guard132_dn2 = 0.0;
        var_guard132_dn3 = 0.0;
        var_guard132_dn4 = 0.0;
        var_guard132_dn5 = 0.0;
        var_guard132_dn6 = 0.0;
        var_guard132_dn7 = 0.0;
        var_guard132_dn8 = 0.0;
        var_guard132_db0 = 0.0;
        var_guard132_db1 = 0.0;
        var_guard132_db2 = 0.0;
        var_guard132_db3 = 0.0;
        var_guard132_db4 = 0.0;
        var_guard132_rv = 0.0;
        var_guard132_rdn0 = 0.0;
        var_guard132_rdn1 = 0.0;
        var_guard132_rdn2 = 0.0;
        var_guard132_rdn3 = 0.0;
        var_guard132_rdn4 = 0.0;
        var_guard132_rdn5 = 0.0;
        var_guard132_rdn6 = 0.0;
        var_guard132_rdn7 = 0.0;
        var_guard132_rdn8 = 0.0;
        var_guard132_rdb0 = 0.0;
        var_guard132_rdb1 = 0.0;
        var_guard132_rdb2 = 0.0;
        var_guard132_rdb3 = 0.0;
        var_guard132_rdb4 = 0.0;

        let (assign12730_e11820, assign12730_e11820_d_n0, assign12730_e11820_d_n1, assign12730_e11820_d_n2, assign12730_e11820_d_n3, assign12730_e11820_d_n4, assign12730_e11820_d_n5, assign12730_e11820_d_n6, assign12730_e11820_d_n7, assign12730_e11820_d_n8, assign12730_e11820_d_b0, assign12730_e11820_d_b1, assign12730_e11820_d_b2, assign12730_e11820_d_b3, assign12730_e11820_d_b4,) = {
    if ((var_guard131 != 0.0) && (var_guard132 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_delclm, var_delclm_dn0, var_delclm_dn1, var_delclm_dn2, var_delclm_dn3, var_delclm_dn4, var_delclm_dn5, var_delclm_dn6, var_delclm_dn7, var_delclm_dn8, var_delclm_db0, var_delclm_db1, var_delclm_db2, var_delclm_db3, var_delclm_db4,)
    }
};
        var_delclm = assign12730_e11820;
        var_delclm_dn0 = assign12730_e11820_d_n0;
        var_delclm_dn1 = assign12730_e11820_d_n1;
        var_delclm_dn2 = assign12730_e11820_d_n2;
        var_delclm_dn3 = assign12730_e11820_d_n3;
        var_delclm_dn4 = assign12730_e11820_d_n4;
        var_delclm_dn5 = assign12730_e11820_d_n5;
        var_delclm_dn6 = assign12730_e11820_d_n6;
        var_delclm_dn7 = assign12730_e11820_d_n7;
        var_delclm_dn8 = assign12730_e11820_d_n8;
        var_delclm_db0 = assign12730_e11820_d_b0;
        var_delclm_db1 = assign12730_e11820_d_b1;
        var_delclm_db2 = assign12730_e11820_d_b2;
        var_delclm_db3 = assign12730_e11820_d_b3;
        var_delclm_db4 = assign12730_e11820_d_b4;
        var_delclm_rv = 0.0;
        var_delclm_rdn0 = 0.0;
        var_delclm_rdn1 = 0.0;
        var_delclm_rdn2 = 0.0;
        var_delclm_rdn3 = 0.0;
        var_delclm_rdn4 = 0.0;
        var_delclm_rdn5 = 0.0;
        var_delclm_rdn6 = 0.0;
        var_delclm_rdn7 = 0.0;
        var_delclm_rdn8 = 0.0;
        var_delclm_rdb0 = 0.0;
        var_delclm_rdb1 = 0.0;
        var_delclm_rdb2 = 0.0;
        var_delclm_rdb3 = 0.0;
        var_delclm_rdb4 = 0.0;

        let (assign12740_e11833, assign12740_e11833_d_n0, assign12740_e11833_d_n1, assign12740_e11833_d_n2, assign12740_e11833_d_n3, assign12740_e11833_d_n4, assign12740_e11833_d_n5, assign12740_e11833_d_n6, assign12740_e11833_d_n7, assign12740_e11833_d_n8, assign12740_e11833_d_b0, assign12740_e11833_d_b1, assign12740_e11833_d_b2, assign12740_e11833_d_b3, assign12740_e11833_d_b4,) = {
    if ((var_guard131 != 0.0) && (var_guard132 == 0.0)) {
        let assign12740_e11827: f64 = (var_diffvds / var_litl);
        let assign12740_e11829: f64 = (assign12740_e11827 + p.p287);
        let assign12740_e11831: f64 = (assign12740_e11829 / var_esatnoi);
        (assign12740_e11831, ((((((var_diffvds_dn0 * var_litl) - (var_diffvds * var_litl_dn0)) / (var_litl * var_litl)) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn0)) / (var_esatnoi * var_esatnoi)), ((((((var_diffvds_dn1 * var_litl) - (var_diffvds * var_litl_dn1)) / (var_litl * var_litl)) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn1)) / (var_esatnoi * var_esatnoi)), ((((((var_diffvds_dn2 * var_litl) - (var_diffvds * var_litl_dn2)) / (var_litl * var_litl)) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn2)) / (var_esatnoi * var_esatnoi)), ((((((var_diffvds_dn3 * var_litl) - (var_diffvds * var_litl_dn3)) / (var_litl * var_litl)) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn3)) / (var_esatnoi * var_esatnoi)), ((((((var_diffvds_dn4 * var_litl) - (var_diffvds * var_litl_dn4)) / (var_litl * var_litl)) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn4)) / (var_esatnoi * var_esatnoi)), ((((((var_diffvds_dn5 * var_litl) - (var_diffvds * var_litl_dn5)) / (var_litl * var_litl)) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn5)) / (var_esatnoi * var_esatnoi)), ((((((var_diffvds_dn6 * var_litl) - (var_diffvds * var_litl_dn6)) / (var_litl * var_litl)) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn6)) / (var_esatnoi * var_esatnoi)), ((((((var_diffvds_dn7 * var_litl) - (var_diffvds * var_litl_dn7)) / (var_litl * var_litl)) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn7)) / (var_esatnoi * var_esatnoi)), ((((((var_diffvds_dn8 * var_litl) - (var_diffvds * var_litl_dn8)) / (var_litl * var_litl)) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn8)) / (var_esatnoi * var_esatnoi)), ((((((var_diffvds_db0 * var_litl) - (var_diffvds * var_litl_db0)) / (var_litl * var_litl)) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_db0)) / (var_esatnoi * var_esatnoi)), ((((((var_diffvds_db1 * var_litl) - (var_diffvds * var_litl_db1)) / (var_litl * var_litl)) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_db1)) / (var_esatnoi * var_esatnoi)), ((((((var_diffvds_db2 * var_litl) - (var_diffvds * var_litl_db2)) / (var_litl * var_litl)) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_db2)) / (var_esatnoi * var_esatnoi)), ((((((var_diffvds_db3 * var_litl) - (var_diffvds * var_litl_db3)) / (var_litl * var_litl)) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_db3)) / (var_esatnoi * var_esatnoi)), ((((((var_diffvds_db4 * var_litl) - (var_diffvds * var_litl_db4)) / (var_litl * var_litl)) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_db4)) / (var_esatnoi * var_esatnoi)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4,)
    }
};
        var_t0 = assign12740_e11833;
        var_t0_dn0 = assign12740_e11833_d_n0;
        var_t0_dn1 = assign12740_e11833_d_n1;
        var_t0_dn2 = assign12740_e11833_d_n2;
        var_t0_dn3 = assign12740_e11833_d_n3;
        var_t0_dn4 = assign12740_e11833_d_n4;
        var_t0_dn5 = assign12740_e11833_d_n5;
        var_t0_dn6 = assign12740_e11833_d_n6;
        var_t0_dn7 = assign12740_e11833_d_n7;
        var_t0_dn8 = assign12740_e11833_d_n8;
        var_t0_db0 = assign12740_e11833_d_b0;
        var_t0_db1 = assign12740_e11833_d_b1;
        var_t0_db2 = assign12740_e11833_d_b2;
        var_t0_db3 = assign12740_e11833_d_b3;
        var_t0_db4 = assign12740_e11833_d_b4;
        var_t0_rv = 0.0;
        var_t0_rdn0 = 0.0;
        var_t0_rdn1 = 0.0;
        var_t0_rdn2 = 0.0;
        var_t0_rdn3 = 0.0;
        var_t0_rdn4 = 0.0;
        var_t0_rdn5 = 0.0;
        var_t0_rdn6 = 0.0;
        var_t0_rdn7 = 0.0;
        var_t0_rdn8 = 0.0;
        var_t0_rdb0 = 0.0;
        var_t0_rdb1 = 0.0;
        var_t0_rdb2 = 0.0;
        var_t0_rdb3 = 0.0;
        var_t0_rdb4 = 0.0;

        let (assign12750_e11845, assign12750_e11845_d_n0, assign12750_e11845_d_n1, assign12750_e11845_d_n2, assign12750_e11845_d_n3, assign12750_e11845_d_n4, assign12750_e11845_d_n5, assign12750_e11845_d_n6, assign12750_e11845_d_n7, assign12750_e11845_d_n8, assign12750_e11845_d_b0, assign12750_e11845_d_b1, assign12750_e11845_d_b2, assign12750_e11845_d_b3, assign12750_e11845_d_b4,) = {
    if ((var_guard131 != 0.0) && (var_guard132 == 0.0)) {
        let assign12750_e11841: f64 = (var_t0).max(1e-38);
        let assign12750_e11842: f64 = (assign12750_e11841).ln();
        let assign12750_e11843: f64 = (var_litl * assign12750_e11842);
        (assign12750_e11843, ((var_litl_dn0 * assign12750_e11842) + (var_litl * (if var_t0 >= 1e-38 { var_t0_dn0 } else { 0.0 } / assign12750_e11841))), ((var_litl_dn1 * assign12750_e11842) + (var_litl * (if var_t0 >= 1e-38 { var_t0_dn1 } else { 0.0 } / assign12750_e11841))), ((var_litl_dn2 * assign12750_e11842) + (var_litl * (if var_t0 >= 1e-38 { var_t0_dn2 } else { 0.0 } / assign12750_e11841))), ((var_litl_dn3 * assign12750_e11842) + (var_litl * (if var_t0 >= 1e-38 { var_t0_dn3 } else { 0.0 } / assign12750_e11841))), ((var_litl_dn4 * assign12750_e11842) + (var_litl * (if var_t0 >= 1e-38 { var_t0_dn4 } else { 0.0 } / assign12750_e11841))), ((var_litl_dn5 * assign12750_e11842) + (var_litl * (if var_t0 >= 1e-38 { var_t0_dn5 } else { 0.0 } / assign12750_e11841))), ((var_litl_dn6 * assign12750_e11842) + (var_litl * (if var_t0 >= 1e-38 { var_t0_dn6 } else { 0.0 } / assign12750_e11841))), ((var_litl_dn7 * assign12750_e11842) + (var_litl * (if var_t0 >= 1e-38 { var_t0_dn7 } else { 0.0 } / assign12750_e11841))), ((var_litl_dn8 * assign12750_e11842) + (var_litl * (if var_t0 >= 1e-38 { var_t0_dn8 } else { 0.0 } / assign12750_e11841))), ((var_litl_db0 * assign12750_e11842) + (var_litl * (if var_t0 >= 1e-38 { var_t0_db0 } else { 0.0 } / assign12750_e11841))), ((var_litl_db1 * assign12750_e11842) + (var_litl * (if var_t0 >= 1e-38 { var_t0_db1 } else { 0.0 } / assign12750_e11841))), ((var_litl_db2 * assign12750_e11842) + (var_litl * (if var_t0 >= 1e-38 { var_t0_db2 } else { 0.0 } / assign12750_e11841))), ((var_litl_db3 * assign12750_e11842) + (var_litl * (if var_t0 >= 1e-38 { var_t0_db3 } else { 0.0 } / assign12750_e11841))), ((var_litl_db4 * assign12750_e11842) + (var_litl * (if var_t0 >= 1e-38 { var_t0_db4 } else { 0.0 } / assign12750_e11841))),)
    } else {
        (var_delclm, var_delclm_dn0, var_delclm_dn1, var_delclm_dn2, var_delclm_dn3, var_delclm_dn4, var_delclm_dn5, var_delclm_dn6, var_delclm_dn7, var_delclm_dn8, var_delclm_db0, var_delclm_db1, var_delclm_db2, var_delclm_db3, var_delclm_db4,)
    }
};
        var_delclm = assign12750_e11845;
        var_delclm_dn0 = assign12750_e11845_d_n0;
        var_delclm_dn1 = assign12750_e11845_d_n1;
        var_delclm_dn2 = assign12750_e11845_d_n2;
        var_delclm_dn3 = assign12750_e11845_d_n3;
        var_delclm_dn4 = assign12750_e11845_d_n4;
        var_delclm_dn5 = assign12750_e11845_d_n5;
        var_delclm_dn6 = assign12750_e11845_d_n6;
        var_delclm_dn7 = assign12750_e11845_d_n7;
        var_delclm_dn8 = assign12750_e11845_d_n8;
        var_delclm_db0 = assign12750_e11845_d_b0;
        var_delclm_db1 = assign12750_e11845_d_b1;
        var_delclm_db2 = assign12750_e11845_d_b2;
        var_delclm_db3 = assign12750_e11845_d_b3;
        var_delclm_db4 = assign12750_e11845_d_b4;
        var_delclm_rv = 0.0;
        var_delclm_rdn0 = 0.0;
        var_delclm_rdn1 = 0.0;
        var_delclm_rdn2 = 0.0;
        var_delclm_rdn3 = 0.0;
        var_delclm_rdn4 = 0.0;
        var_delclm_rdn5 = 0.0;
        var_delclm_rdn6 = 0.0;
        var_delclm_rdn7 = 0.0;
        var_delclm_rdn8 = 0.0;
        var_delclm_rdb0 = 0.0;
        var_delclm_rdb1 = 0.0;
        var_delclm_rdb2 = 0.0;
        var_delclm_rdb3 = 0.0;
        var_delclm_rdb4 = 0.0;

        let assign12760_e11848: f64 = if var_delclm < 0.0 { 1.0 } else { 0.0 };
        var_guard133 = assign12760_e11848;
        var_guard133_dn0 = 0.0;
        var_guard133_dn1 = 0.0;
        var_guard133_dn2 = 0.0;
        var_guard133_dn3 = 0.0;
        var_guard133_dn4 = 0.0;
        var_guard133_dn5 = 0.0;
        var_guard133_dn6 = 0.0;
        var_guard133_dn7 = 0.0;
        var_guard133_dn8 = 0.0;
        var_guard133_db0 = 0.0;
        var_guard133_db1 = 0.0;
        var_guard133_db2 = 0.0;
        var_guard133_db3 = 0.0;
        var_guard133_db4 = 0.0;
        var_guard133_rv = 0.0;
        var_guard133_rdn0 = 0.0;
        var_guard133_rdn1 = 0.0;
        var_guard133_rdn2 = 0.0;
        var_guard133_rdn3 = 0.0;
        var_guard133_rdn4 = 0.0;
        var_guard133_rdn5 = 0.0;
        var_guard133_rdn6 = 0.0;
        var_guard133_rdn7 = 0.0;
        var_guard133_rdn8 = 0.0;
        var_guard133_rdb0 = 0.0;
        var_guard133_rdb1 = 0.0;
        var_guard133_rdb2 = 0.0;
        var_guard133_rdb3 = 0.0;
        var_guard133_rdb4 = 0.0;

        let (assign12770_e11857, assign12770_e11857_d_n0, assign12770_e11857_d_n1, assign12770_e11857_d_n2, assign12770_e11857_d_n3, assign12770_e11857_d_n4, assign12770_e11857_d_n5, assign12770_e11857_d_n6, assign12770_e11857_d_n7, assign12770_e11857_d_n8, assign12770_e11857_d_b0, assign12770_e11857_d_b1, assign12770_e11857_d_b2, assign12770_e11857_d_b3, assign12770_e11857_d_b4,) = {
    if (((var_guard131 != 0.0) && (var_guard132 == 0.0)) && (var_guard133 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_delclm, var_delclm_dn0, var_delclm_dn1, var_delclm_dn2, var_delclm_dn3, var_delclm_dn4, var_delclm_dn5, var_delclm_dn6, var_delclm_dn7, var_delclm_dn8, var_delclm_db0, var_delclm_db1, var_delclm_db2, var_delclm_db3, var_delclm_db4,)
    }
};
        var_delclm = assign12770_e11857;
        var_delclm_dn0 = assign12770_e11857_d_n0;
        var_delclm_dn1 = assign12770_e11857_d_n1;
        var_delclm_dn2 = assign12770_e11857_d_n2;
        var_delclm_dn3 = assign12770_e11857_d_n3;
        var_delclm_dn4 = assign12770_e11857_d_n4;
        var_delclm_dn5 = assign12770_e11857_d_n5;
        var_delclm_dn6 = assign12770_e11857_d_n6;
        var_delclm_dn7 = assign12770_e11857_d_n7;
        var_delclm_dn8 = assign12770_e11857_d_n8;
        var_delclm_db0 = assign12770_e11857_d_b0;
        var_delclm_db1 = assign12770_e11857_d_b1;
        var_delclm_db2 = assign12770_e11857_d_b2;
        var_delclm_db3 = assign12770_e11857_d_b3;
        var_delclm_db4 = assign12770_e11857_d_b4;
        var_delclm_rv = 0.0;
        var_delclm_rdn0 = 0.0;
        var_delclm_rdn1 = 0.0;
        var_delclm_rdn2 = 0.0;
        var_delclm_rdn3 = 0.0;
        var_delclm_rdn4 = 0.0;
        var_delclm_rdn5 = 0.0;
        var_delclm_rdn6 = 0.0;
        var_delclm_rdn7 = 0.0;
        var_delclm_rdn8 = 0.0;
        var_delclm_rdb0 = 0.0;
        var_delclm_rdb1 = 0.0;
        var_delclm_rdb2 = 0.0;
        var_delclm_rdb3 = 0.0;
        var_delclm_rdb4 = 0.0;

        let assign12780_e11860: f64 = if p.p22 == 1.0 { 1.0 } else { 0.0 };
        var_guard134 = assign12780_e11860;
        var_guard134_dn0 = 0.0;
        var_guard134_dn1 = 0.0;
        var_guard134_dn2 = 0.0;
        var_guard134_dn3 = 0.0;
        var_guard134_dn4 = 0.0;
        var_guard134_dn5 = 0.0;
        var_guard134_dn6 = 0.0;
        var_guard134_dn7 = 0.0;
        var_guard134_dn8 = 0.0;
        var_guard134_db0 = 0.0;
        var_guard134_db1 = 0.0;
        var_guard134_db2 = 0.0;
        var_guard134_db3 = 0.0;
        var_guard134_db4 = 0.0;
        var_guard134_rv = 0.0;
        var_guard134_rdn0 = 0.0;
        var_guard134_rdn1 = 0.0;
        var_guard134_rdn2 = 0.0;
        var_guard134_rdn3 = 0.0;
        var_guard134_rdn4 = 0.0;
        var_guard134_rdn5 = 0.0;
        var_guard134_rdn6 = 0.0;
        var_guard134_rdn7 = 0.0;
        var_guard134_rdn8 = 0.0;
        var_guard134_rdb0 = 0.0;
        var_guard134_rdb1 = 0.0;
        var_guard134_rdb2 = 0.0;
        var_guard134_rdb3 = 0.0;
        var_guard134_rdb4 = 0.0;

        let (assign12790_e11868, assign12790_e11868_d_n0, assign12790_e11868_d_n1, assign12790_e11868_d_n2, assign12790_e11868_d_n3, assign12790_e11868_d_n4, assign12790_e11868_d_n5, assign12790_e11868_d_n6, assign12790_e11868_d_n7, assign12790_e11868_d_n8, assign12790_e11868_d_b0, assign12790_e11868_d_b1, assign12790_e11868_d_b2, assign12790_e11868_d_b3, assign12790_e11868_d_b4,) = {
    if ((var_guard131 != 0.0) && (var_guard134 != 0.0)) {
        let assign12790_e11866: f64 = (var_qia2 / var_qsref_i);
        (assign12790_e11866, (((var_qia2_dn0 * var_qsref_i) - (var_qia2 * var_qsref_i_dn0)) / (var_qsref_i * var_qsref_i)), (((var_qia2_dn1 * var_qsref_i) - (var_qia2 * var_qsref_i_dn1)) / (var_qsref_i * var_qsref_i)), (((var_qia2_dn2 * var_qsref_i) - (var_qia2 * var_qsref_i_dn2)) / (var_qsref_i * var_qsref_i)), (((var_qia2_dn3 * var_qsref_i) - (var_qia2 * var_qsref_i_dn3)) / (var_qsref_i * var_qsref_i)), (((var_qia2_dn4 * var_qsref_i) - (var_qia2 * var_qsref_i_dn4)) / (var_qsref_i * var_qsref_i)), (((var_qia2_dn5 * var_qsref_i) - (var_qia2 * var_qsref_i_dn5)) / (var_qsref_i * var_qsref_i)), (((var_qia2_dn6 * var_qsref_i) - (var_qia2 * var_qsref_i_dn6)) / (var_qsref_i * var_qsref_i)), (((var_qia2_dn7 * var_qsref_i) - (var_qia2 * var_qsref_i_dn7)) / (var_qsref_i * var_qsref_i)), (((var_qia2_dn8 * var_qsref_i) - (var_qia2 * var_qsref_i_dn8)) / (var_qsref_i * var_qsref_i)), (((var_qia2_db0 * var_qsref_i) - (var_qia2 * var_qsref_i_db0)) / (var_qsref_i * var_qsref_i)), (((var_qia2_db1 * var_qsref_i) - (var_qia2 * var_qsref_i_db1)) / (var_qsref_i * var_qsref_i)), (((var_qia2_db2 * var_qsref_i) - (var_qia2 * var_qsref_i_db2)) / (var_qsref_i * var_qsref_i)), (((var_qia2_db3 * var_qsref_i) - (var_qia2 * var_qsref_i_db3)) / (var_qsref_i * var_qsref_i)), (((var_qia2_db4 * var_qsref_i) - (var_qia2 * var_qsref_i_db4)) / (var_qsref_i * var_qsref_i)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4,)
    }
};
        var_t1 = assign12790_e11868;
        var_t1_dn0 = assign12790_e11868_d_n0;
        var_t1_dn1 = assign12790_e11868_d_n1;
        var_t1_dn2 = assign12790_e11868_d_n2;
        var_t1_dn3 = assign12790_e11868_d_n3;
        var_t1_dn4 = assign12790_e11868_d_n4;
        var_t1_dn5 = assign12790_e11868_d_n5;
        var_t1_dn6 = assign12790_e11868_d_n6;
        var_t1_dn7 = assign12790_e11868_d_n7;
        var_t1_dn8 = assign12790_e11868_d_n8;
        var_t1_db0 = assign12790_e11868_d_b0;
        var_t1_db1 = assign12790_e11868_d_b1;
        var_t1_db2 = assign12790_e11868_d_b2;
        var_t1_db3 = assign12790_e11868_d_b3;
        var_t1_db4 = assign12790_e11868_d_b4;
        var_t1_rv = 0.0;
        var_t1_rdn0 = 0.0;
        var_t1_rdn1 = 0.0;
        var_t1_rdn2 = 0.0;
        var_t1_rdn3 = 0.0;
        var_t1_rdn4 = 0.0;
        var_t1_rdn5 = 0.0;
        var_t1_rdn6 = 0.0;
        var_t1_rdn7 = 0.0;
        var_t1_rdn8 = 0.0;
        var_t1_rdb0 = 0.0;
        var_t1_rdb1 = 0.0;
        var_t1_rdb2 = 0.0;
        var_t1_rdb3 = 0.0;
        var_t1_rdb4 = 0.0;

        let (assign12800_e11878, assign12800_e11878_d_n0, assign12800_e11878_d_n1, assign12800_e11878_d_n2, assign12800_e11878_d_n3, assign12800_e11878_d_n4, assign12800_e11878_d_n5, assign12800_e11878_d_n6, assign12800_e11878_d_n7, assign12800_e11878_d_n8, assign12800_e11878_d_b0, assign12800_e11878_d_b1, assign12800_e11878_d_b2, assign12800_e11878_d_b3, assign12800_e11878_d_b4,) = {
    if ((var_guard131 != 0.0) && (var_guard134 != 0.0)) {
        let assign12800_e11875: f64 = (var_t1).powf(var_mpower_i);
        let assign12800_e11876: f64 = (1.0 + assign12800_e11875);
        (assign12800_e11876, if var_mpower_i_dn0 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn0)) } } else { (assign12800_e11875 * ((var_mpower_i_dn0 * (var_t1).ln()) + (var_mpower_i * (var_t1_dn0 / var_t1)))) }, if var_mpower_i_dn1 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn1)) } } else { (assign12800_e11875 * ((var_mpower_i_dn1 * (var_t1).ln()) + (var_mpower_i * (var_t1_dn1 / var_t1)))) }, if var_mpower_i_dn2 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn2)) } } else { (assign12800_e11875 * ((var_mpower_i_dn2 * (var_t1).ln()) + (var_mpower_i * (var_t1_dn2 / var_t1)))) }, if var_mpower_i_dn3 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn3)) } } else { (assign12800_e11875 * ((var_mpower_i_dn3 * (var_t1).ln()) + (var_mpower_i * (var_t1_dn3 / var_t1)))) }, if var_mpower_i_dn4 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn4)) } } else { (assign12800_e11875 * ((var_mpower_i_dn4 * (var_t1).ln()) + (var_mpower_i * (var_t1_dn4 / var_t1)))) }, if var_mpower_i_dn5 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn5)) } } else { (assign12800_e11875 * ((var_mpower_i_dn5 * (var_t1).ln()) + (var_mpower_i * (var_t1_dn5 / var_t1)))) }, if var_mpower_i_dn6 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn6)) } } else { (assign12800_e11875 * ((var_mpower_i_dn6 * (var_t1).ln()) + (var_mpower_i * (var_t1_dn6 / var_t1)))) }, if var_mpower_i_dn7 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn7)) } } else { (assign12800_e11875 * ((var_mpower_i_dn7 * (var_t1).ln()) + (var_mpower_i * (var_t1_dn7 / var_t1)))) }, if var_mpower_i_dn8 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn8)) } } else { (assign12800_e11875 * ((var_mpower_i_dn8 * (var_t1).ln()) + (var_mpower_i * (var_t1_dn8 / var_t1)))) }, if var_mpower_i_db0 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_db0)) } } else { (assign12800_e11875 * ((var_mpower_i_db0 * (var_t1).ln()) + (var_mpower_i * (var_t1_db0 / var_t1)))) }, if var_mpower_i_db1 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_db1)) } } else { (assign12800_e11875 * ((var_mpower_i_db1 * (var_t1).ln()) + (var_mpower_i * (var_t1_db1 / var_t1)))) }, if var_mpower_i_db2 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_db2)) } } else { (assign12800_e11875 * ((var_mpower_i_db2 * (var_t1).ln()) + (var_mpower_i * (var_t1_db2 / var_t1)))) }, if var_mpower_i_db3 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_db3)) } } else { (assign12800_e11875 * ((var_mpower_i_db3 * (var_t1).ln()) + (var_mpower_i * (var_t1_db3 / var_t1)))) }, if var_mpower_i_db4 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_db4)) } } else { (assign12800_e11875 * ((var_mpower_i_db4 * (var_t1).ln()) + (var_mpower_i * (var_t1_db4 / var_t1)))) },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_db0, var_t2_db1, var_t2_db2, var_t2_db3, var_t2_db4,)
    }
};
        var_t2 = assign12800_e11878;
        var_t2_dn0 = assign12800_e11878_d_n0;
        var_t2_dn1 = assign12800_e11878_d_n1;
        var_t2_dn2 = assign12800_e11878_d_n2;
        var_t2_dn3 = assign12800_e11878_d_n3;
        var_t2_dn4 = assign12800_e11878_d_n4;
        var_t2_dn5 = assign12800_e11878_d_n5;
        var_t2_dn6 = assign12800_e11878_d_n6;
        var_t2_dn7 = assign12800_e11878_d_n7;
        var_t2_dn8 = assign12800_e11878_d_n8;
        var_t2_db0 = assign12800_e11878_d_b0;
        var_t2_db1 = assign12800_e11878_d_b1;
        var_t2_db2 = assign12800_e11878_d_b2;
        var_t2_db3 = assign12800_e11878_d_b3;
        var_t2_db4 = assign12800_e11878_d_b4;
        var_t2_rv = 0.0;
        var_t2_rdn0 = 0.0;
        var_t2_rdn1 = 0.0;
        var_t2_rdn2 = 0.0;
        var_t2_rdn3 = 0.0;
        var_t2_rdn4 = 0.0;
        var_t2_rdn5 = 0.0;
        var_t2_rdn6 = 0.0;
        var_t2_rdn7 = 0.0;
        var_t2_rdn8 = 0.0;
        var_t2_rdb0 = 0.0;
        var_t2_rdb1 = 0.0;
        var_t2_rdb2 = 0.0;
        var_t2_rdb3 = 0.0;
        var_t2_rdb4 = 0.0;

        let (assign12810_e11886, assign12810_e11886_d_n0, assign12810_e11886_d_n1, assign12810_e11886_d_n2, assign12810_e11886_d_n3, assign12810_e11886_d_n4, assign12810_e11886_d_n5, assign12810_e11886_d_n6, assign12810_e11886_d_n7, assign12810_e11886_d_n8, assign12810_e11886_d_b0, assign12810_e11886_d_b1, assign12810_e11886_d_b2, assign12810_e11886_d_b3, assign12810_e11886_d_b4,) = {
    if ((var_guard131 != 0.0) && (var_guard134 != 0.0)) {
        let assign12810_e11884: f64 = (var_noia2_i / var_t2);
        (assign12810_e11884, (((var_noia2_i_dn0 * var_t2) - (var_noia2_i * var_t2_dn0)) / (var_t2 * var_t2)), (((var_noia2_i_dn1 * var_t2) - (var_noia2_i * var_t2_dn1)) / (var_t2 * var_t2)), (((var_noia2_i_dn2 * var_t2) - (var_noia2_i * var_t2_dn2)) / (var_t2 * var_t2)), (((var_noia2_i_dn3 * var_t2) - (var_noia2_i * var_t2_dn3)) / (var_t2 * var_t2)), (((var_noia2_i_dn4 * var_t2) - (var_noia2_i * var_t2_dn4)) / (var_t2 * var_t2)), (((var_noia2_i_dn5 * var_t2) - (var_noia2_i * var_t2_dn5)) / (var_t2 * var_t2)), (((var_noia2_i_dn6 * var_t2) - (var_noia2_i * var_t2_dn6)) / (var_t2 * var_t2)), (((var_noia2_i_dn7 * var_t2) - (var_noia2_i * var_t2_dn7)) / (var_t2 * var_t2)), (((var_noia2_i_dn8 * var_t2) - (var_noia2_i * var_t2_dn8)) / (var_t2 * var_t2)), (((var_noia2_i_db0 * var_t2) - (var_noia2_i * var_t2_db0)) / (var_t2 * var_t2)), (((var_noia2_i_db1 * var_t2) - (var_noia2_i * var_t2_db1)) / (var_t2 * var_t2)), (((var_noia2_i_db2 * var_t2) - (var_noia2_i * var_t2_db2)) / (var_t2 * var_t2)), (((var_noia2_i_db3 * var_t2) - (var_noia2_i * var_t2_db3)) / (var_t2 * var_t2)), (((var_noia2_i_db4 * var_t2) - (var_noia2_i * var_t2_db4)) / (var_t2 * var_t2)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn1, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_db0, var_t3_db1, var_t3_db2, var_t3_db3, var_t3_db4,)
    }
};
        var_t3 = assign12810_e11886;
        var_t3_dn0 = assign12810_e11886_d_n0;
        var_t3_dn1 = assign12810_e11886_d_n1;
        var_t3_dn2 = assign12810_e11886_d_n2;
        var_t3_dn3 = assign12810_e11886_d_n3;
        var_t3_dn4 = assign12810_e11886_d_n4;
        var_t3_dn5 = assign12810_e11886_d_n5;
        var_t3_dn6 = assign12810_e11886_d_n6;
        var_t3_dn7 = assign12810_e11886_d_n7;
        var_t3_dn8 = assign12810_e11886_d_n8;
        var_t3_db0 = assign12810_e11886_d_b0;
        var_t3_db1 = assign12810_e11886_d_b1;
        var_t3_db2 = assign12810_e11886_d_b2;
        var_t3_db3 = assign12810_e11886_d_b3;
        var_t3_db4 = assign12810_e11886_d_b4;
        var_t3_rv = 0.0;
        var_t3_rdn0 = 0.0;
        var_t3_rdn1 = 0.0;
        var_t3_rdn2 = 0.0;
        var_t3_rdn3 = 0.0;
        var_t3_rdn4 = 0.0;
        var_t3_rdn5 = 0.0;
        var_t3_rdn6 = 0.0;
        var_t3_rdn7 = 0.0;
        var_t3_rdn8 = 0.0;
        var_t3_rdb0 = 0.0;
        var_t3_rdb1 = 0.0;
        var_t3_rdb2 = 0.0;
        var_t3_rdb3 = 0.0;
        var_t3_rdb4 = 0.0;

        let (assign12820_e11894, assign12820_e11894_d_n0, assign12820_e11894_d_n1, assign12820_e11894_d_n2, assign12820_e11894_d_n3, assign12820_e11894_d_n4, assign12820_e11894_d_n5, assign12820_e11894_d_n6, assign12820_e11894_d_n7, assign12820_e11894_d_n8, assign12820_e11894_d_b0, assign12820_e11894_d_b1, assign12820_e11894_d_b2, assign12820_e11894_d_b3, assign12820_e11894_d_b4,) = {
    if ((var_guard131 != 0.0) && (var_guard134 != 0.0)) {
        let assign12820_e11892: f64 = (var_t3 / p.p288);
        (assign12820_e11892, (var_t3_dn0 / p.p288), (var_t3_dn1 / p.p288), (var_t3_dn2 / p.p288), (var_t3_dn3 / p.p288), (var_t3_dn4 / p.p288), (var_t3_dn5 / p.p288), (var_t3_dn6 / p.p288), (var_t3_dn7 / p.p288), (var_t3_dn8 / p.p288), (var_t3_db0 / p.p288), (var_t3_db1 / p.p288), (var_t3_db2 / p.p288), (var_t3_db3 / p.p288), (var_t3_db4 / p.p288),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn1, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_db0, var_t4_db1, var_t4_db2, var_t4_db3, var_t4_db4,)
    }
};
        var_t4 = assign12820_e11894;
        var_t4_dn0 = assign12820_e11894_d_n0;
        var_t4_dn1 = assign12820_e11894_d_n1;
        var_t4_dn2 = assign12820_e11894_d_n2;
        var_t4_dn3 = assign12820_e11894_d_n3;
        var_t4_dn4 = assign12820_e11894_d_n4;
        var_t4_dn5 = assign12820_e11894_d_n5;
        var_t4_dn6 = assign12820_e11894_d_n6;
        var_t4_dn7 = assign12820_e11894_d_n7;
        var_t4_dn8 = assign12820_e11894_d_n8;
        var_t4_db0 = assign12820_e11894_d_b0;
        var_t4_db1 = assign12820_e11894_d_b1;
        var_t4_db2 = assign12820_e11894_d_b2;
        var_t4_db3 = assign12820_e11894_d_b3;
        var_t4_db4 = assign12820_e11894_d_b4;
        var_t4_rv = 0.0;
        var_t4_rdn0 = 0.0;
        var_t4_rdn1 = 0.0;
        var_t4_rdn2 = 0.0;
        var_t4_rdn3 = 0.0;
        var_t4_rdn4 = 0.0;
        var_t4_rdn5 = 0.0;
        var_t4_rdn6 = 0.0;
        var_t4_rdn7 = 0.0;
        var_t4_rdn8 = 0.0;
        var_t4_rdb0 = 0.0;
        var_t4_rdb1 = 0.0;
        var_t4_rdb2 = 0.0;
        var_t4_rdb3 = 0.0;
        var_t4_rdb4 = 0.0;


        *var_delclm_slot = var_delclm;
        *var_delclm_db0_slot = var_delclm_db0;
        *var_delclm_db1_slot = var_delclm_db1;
        *var_delclm_db2_slot = var_delclm_db2;
        *var_delclm_db3_slot = var_delclm_db3;
        *var_delclm_db4_slot = var_delclm_db4;
        *var_delclm_dn0_slot = var_delclm_dn0;
        *var_delclm_dn1_slot = var_delclm_dn1;
        *var_delclm_dn2_slot = var_delclm_dn2;
        *var_delclm_dn3_slot = var_delclm_dn3;
        *var_delclm_dn4_slot = var_delclm_dn4;
        *var_delclm_dn5_slot = var_delclm_dn5;
        *var_delclm_dn6_slot = var_delclm_dn6;
        *var_delclm_dn7_slot = var_delclm_dn7;
        *var_delclm_dn8_slot = var_delclm_dn8;
        *var_delclm_rdb0_slot = var_delclm_rdb0;
        *var_delclm_rdb1_slot = var_delclm_rdb1;
        *var_delclm_rdb2_slot = var_delclm_rdb2;
        *var_delclm_rdb3_slot = var_delclm_rdb3;
        *var_delclm_rdb4_slot = var_delclm_rdb4;
        *var_delclm_rdn0_slot = var_delclm_rdn0;
        *var_delclm_rdn1_slot = var_delclm_rdn1;
        *var_delclm_rdn2_slot = var_delclm_rdn2;
        *var_delclm_rdn3_slot = var_delclm_rdn3;
        *var_delclm_rdn4_slot = var_delclm_rdn4;
        *var_delclm_rdn5_slot = var_delclm_rdn5;
        *var_delclm_rdn6_slot = var_delclm_rdn6;
        *var_delclm_rdn7_slot = var_delclm_rdn7;
        *var_delclm_rdn8_slot = var_delclm_rdn8;
        *var_delclm_rv_slot = var_delclm_rv;
        *var_guard132_slot = var_guard132;
        *var_guard132_db0_slot = var_guard132_db0;
        *var_guard132_db1_slot = var_guard132_db1;
        *var_guard132_db2_slot = var_guard132_db2;
        *var_guard132_db3_slot = var_guard132_db3;
        *var_guard132_db4_slot = var_guard132_db4;
        *var_guard132_dn0_slot = var_guard132_dn0;
        *var_guard132_dn1_slot = var_guard132_dn1;
        *var_guard132_dn2_slot = var_guard132_dn2;
        *var_guard132_dn3_slot = var_guard132_dn3;
        *var_guard132_dn4_slot = var_guard132_dn4;
        *var_guard132_dn5_slot = var_guard132_dn5;
        *var_guard132_dn6_slot = var_guard132_dn6;
        *var_guard132_dn7_slot = var_guard132_dn7;
        *var_guard132_dn8_slot = var_guard132_dn8;
        *var_guard132_rdb0_slot = var_guard132_rdb0;
        *var_guard132_rdb1_slot = var_guard132_rdb1;
        *var_guard132_rdb2_slot = var_guard132_rdb2;
        *var_guard132_rdb3_slot = var_guard132_rdb3;
        *var_guard132_rdb4_slot = var_guard132_rdb4;
        *var_guard132_rdn0_slot = var_guard132_rdn0;
        *var_guard132_rdn1_slot = var_guard132_rdn1;
        *var_guard132_rdn2_slot = var_guard132_rdn2;
        *var_guard132_rdn3_slot = var_guard132_rdn3;
        *var_guard132_rdn4_slot = var_guard132_rdn4;
        *var_guard132_rdn5_slot = var_guard132_rdn5;
        *var_guard132_rdn6_slot = var_guard132_rdn6;
        *var_guard132_rdn7_slot = var_guard132_rdn7;
        *var_guard132_rdn8_slot = var_guard132_rdn8;
        *var_guard132_rv_slot = var_guard132_rv;
        *var_guard133_slot = var_guard133;
        *var_guard133_db0_slot = var_guard133_db0;
        *var_guard133_db1_slot = var_guard133_db1;
        *var_guard133_db2_slot = var_guard133_db2;
        *var_guard133_db3_slot = var_guard133_db3;
        *var_guard133_db4_slot = var_guard133_db4;
        *var_guard133_dn0_slot = var_guard133_dn0;
        *var_guard133_dn1_slot = var_guard133_dn1;
        *var_guard133_dn2_slot = var_guard133_dn2;
        *var_guard133_dn3_slot = var_guard133_dn3;
        *var_guard133_dn4_slot = var_guard133_dn4;
        *var_guard133_dn5_slot = var_guard133_dn5;
        *var_guard133_dn6_slot = var_guard133_dn6;
        *var_guard133_dn7_slot = var_guard133_dn7;
        *var_guard133_dn8_slot = var_guard133_dn8;
        *var_guard133_rdb0_slot = var_guard133_rdb0;
        *var_guard133_rdb1_slot = var_guard133_rdb1;
        *var_guard133_rdb2_slot = var_guard133_rdb2;
        *var_guard133_rdb3_slot = var_guard133_rdb3;
        *var_guard133_rdb4_slot = var_guard133_rdb4;
        *var_guard133_rdn0_slot = var_guard133_rdn0;
        *var_guard133_rdn1_slot = var_guard133_rdn1;
        *var_guard133_rdn2_slot = var_guard133_rdn2;
        *var_guard133_rdn3_slot = var_guard133_rdn3;
        *var_guard133_rdn4_slot = var_guard133_rdn4;
        *var_guard133_rdn5_slot = var_guard133_rdn5;
        *var_guard133_rdn6_slot = var_guard133_rdn6;
        *var_guard133_rdn7_slot = var_guard133_rdn7;
        *var_guard133_rdn8_slot = var_guard133_rdn8;
        *var_guard133_rv_slot = var_guard133_rv;
        *var_guard134_slot = var_guard134;
        *var_guard134_db0_slot = var_guard134_db0;
        *var_guard134_db1_slot = var_guard134_db1;
        *var_guard134_db2_slot = var_guard134_db2;
        *var_guard134_db3_slot = var_guard134_db3;
        *var_guard134_db4_slot = var_guard134_db4;
        *var_guard134_dn0_slot = var_guard134_dn0;
        *var_guard134_dn1_slot = var_guard134_dn1;
        *var_guard134_dn2_slot = var_guard134_dn2;
        *var_guard134_dn3_slot = var_guard134_dn3;
        *var_guard134_dn4_slot = var_guard134_dn4;
        *var_guard134_dn5_slot = var_guard134_dn5;
        *var_guard134_dn6_slot = var_guard134_dn6;
        *var_guard134_dn7_slot = var_guard134_dn7;
        *var_guard134_dn8_slot = var_guard134_dn8;
        *var_guard134_rdb0_slot = var_guard134_rdb0;
        *var_guard134_rdb1_slot = var_guard134_rdb1;
        *var_guard134_rdb2_slot = var_guard134_rdb2;
        *var_guard134_rdb3_slot = var_guard134_rdb3;
        *var_guard134_rdb4_slot = var_guard134_rdb4;
        *var_guard134_rdn0_slot = var_guard134_rdn0;
        *var_guard134_rdn1_slot = var_guard134_rdn1;
        *var_guard134_rdn2_slot = var_guard134_rdn2;
        *var_guard134_rdn3_slot = var_guard134_rdn3;
        *var_guard134_rdn4_slot = var_guard134_rdn4;
        *var_guard134_rdn5_slot = var_guard134_rdn5;
        *var_guard134_rdn6_slot = var_guard134_rdn6;
        *var_guard134_rdn7_slot = var_guard134_rdn7;
        *var_guard134_rdn8_slot = var_guard134_rdn8;
        *var_guard134_rv_slot = var_guard134_rv;
        *var_leffnoi_slot = var_leffnoi;
        *var_leffnoi_db0_slot = var_leffnoi_db0;
        *var_leffnoi_db1_slot = var_leffnoi_db1;
        *var_leffnoi_db2_slot = var_leffnoi_db2;
        *var_leffnoi_db3_slot = var_leffnoi_db3;
        *var_leffnoi_db4_slot = var_leffnoi_db4;
        *var_leffnoi_dn0_slot = var_leffnoi_dn0;
        *var_leffnoi_dn1_slot = var_leffnoi_dn1;
        *var_leffnoi_dn2_slot = var_leffnoi_dn2;
        *var_leffnoi_dn3_slot = var_leffnoi_dn3;
        *var_leffnoi_dn4_slot = var_leffnoi_dn4;
        *var_leffnoi_dn5_slot = var_leffnoi_dn5;
        *var_leffnoi_dn6_slot = var_leffnoi_dn6;
        *var_leffnoi_dn7_slot = var_leffnoi_dn7;
        *var_leffnoi_dn8_slot = var_leffnoi_dn8;
        *var_leffnoi_rdb0_slot = var_leffnoi_rdb0;
        *var_leffnoi_rdb1_slot = var_leffnoi_rdb1;
        *var_leffnoi_rdb2_slot = var_leffnoi_rdb2;
        *var_leffnoi_rdb3_slot = var_leffnoi_rdb3;
        *var_leffnoi_rdb4_slot = var_leffnoi_rdb4;
        *var_leffnoi_rdn0_slot = var_leffnoi_rdn0;
        *var_leffnoi_rdn1_slot = var_leffnoi_rdn1;
        *var_leffnoi_rdn2_slot = var_leffnoi_rdn2;
        *var_leffnoi_rdn3_slot = var_leffnoi_rdn3;
        *var_leffnoi_rdn4_slot = var_leffnoi_rdn4;
        *var_leffnoi_rdn5_slot = var_leffnoi_rdn5;
        *var_leffnoi_rdn6_slot = var_leffnoi_rdn6;
        *var_leffnoi_rdn7_slot = var_leffnoi_rdn7;
        *var_leffnoi_rdn8_slot = var_leffnoi_rdn8;
        *var_leffnoi_rv_slot = var_leffnoi_rv;
        *var_leffnoisq_slot = var_leffnoisq;
        *var_leffnoisq_db0_slot = var_leffnoisq_db0;
        *var_leffnoisq_db1_slot = var_leffnoisq_db1;
        *var_leffnoisq_db2_slot = var_leffnoisq_db2;
        *var_leffnoisq_db3_slot = var_leffnoisq_db3;
        *var_leffnoisq_db4_slot = var_leffnoisq_db4;
        *var_leffnoisq_dn0_slot = var_leffnoisq_dn0;
        *var_leffnoisq_dn1_slot = var_leffnoisq_dn1;
        *var_leffnoisq_dn2_slot = var_leffnoisq_dn2;
        *var_leffnoisq_dn3_slot = var_leffnoisq_dn3;
        *var_leffnoisq_dn4_slot = var_leffnoisq_dn4;
        *var_leffnoisq_dn5_slot = var_leffnoisq_dn5;
        *var_leffnoisq_dn6_slot = var_leffnoisq_dn6;
        *var_leffnoisq_dn7_slot = var_leffnoisq_dn7;
        *var_leffnoisq_dn8_slot = var_leffnoisq_dn8;
        *var_leffnoisq_rdb0_slot = var_leffnoisq_rdb0;
        *var_leffnoisq_rdb1_slot = var_leffnoisq_rdb1;
        *var_leffnoisq_rdb2_slot = var_leffnoisq_rdb2;
        *var_leffnoisq_rdb3_slot = var_leffnoisq_rdb3;
        *var_leffnoisq_rdb4_slot = var_leffnoisq_rdb4;
        *var_leffnoisq_rdn0_slot = var_leffnoisq_rdn0;
        *var_leffnoisq_rdn1_slot = var_leffnoisq_rdn1;
        *var_leffnoisq_rdn2_slot = var_leffnoisq_rdn2;
        *var_leffnoisq_rdn3_slot = var_leffnoisq_rdn3;
        *var_leffnoisq_rdn4_slot = var_leffnoisq_rdn4;
        *var_leffnoisq_rdn5_slot = var_leffnoisq_rdn5;
        *var_leffnoisq_rdn6_slot = var_leffnoisq_rdn6;
        *var_leffnoisq_rdn7_slot = var_leffnoisq_rdn7;
        *var_leffnoisq_rdn8_slot = var_leffnoisq_rdn8;
        *var_leffnoisq_rv_slot = var_leffnoisq_rv;
        *var_t0_slot = var_t0;
        *var_t0_db0_slot = var_t0_db0;
        *var_t0_db1_slot = var_t0_db1;
        *var_t0_db2_slot = var_t0_db2;
        *var_t0_db3_slot = var_t0_db3;
        *var_t0_db4_slot = var_t0_db4;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rdb0_slot = var_t0_rdb0;
        *var_t0_rdb1_slot = var_t0_rdb1;
        *var_t0_rdb2_slot = var_t0_rdb2;
        *var_t0_rdb3_slot = var_t0_rdb3;
        *var_t0_rdb4_slot = var_t0_rdb4;
        *var_t0_rdn0_slot = var_t0_rdn0;
        *var_t0_rdn1_slot = var_t0_rdn1;
        *var_t0_rdn2_slot = var_t0_rdn2;
        *var_t0_rdn3_slot = var_t0_rdn3;
        *var_t0_rdn4_slot = var_t0_rdn4;
        *var_t0_rdn5_slot = var_t0_rdn5;
        *var_t0_rdn6_slot = var_t0_rdn6;
        *var_t0_rdn7_slot = var_t0_rdn7;
        *var_t0_rdn8_slot = var_t0_rdn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_db0_slot = var_t1_db0;
        *var_t1_db1_slot = var_t1_db1;
        *var_t1_db2_slot = var_t1_db2;
        *var_t1_db3_slot = var_t1_db3;
        *var_t1_db4_slot = var_t1_db4;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rdb0_slot = var_t1_rdb0;
        *var_t1_rdb1_slot = var_t1_rdb1;
        *var_t1_rdb2_slot = var_t1_rdb2;
        *var_t1_rdb3_slot = var_t1_rdb3;
        *var_t1_rdb4_slot = var_t1_rdb4;
        *var_t1_rdn0_slot = var_t1_rdn0;
        *var_t1_rdn1_slot = var_t1_rdn1;
        *var_t1_rdn2_slot = var_t1_rdn2;
        *var_t1_rdn3_slot = var_t1_rdn3;
        *var_t1_rdn4_slot = var_t1_rdn4;
        *var_t1_rdn5_slot = var_t1_rdn5;
        *var_t1_rdn6_slot = var_t1_rdn6;
        *var_t1_rdn7_slot = var_t1_rdn7;
        *var_t1_rdn8_slot = var_t1_rdn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_db0_slot = var_t2_db0;
        *var_t2_db1_slot = var_t2_db1;
        *var_t2_db2_slot = var_t2_db2;
        *var_t2_db3_slot = var_t2_db3;
        *var_t2_db4_slot = var_t2_db4;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rdb0_slot = var_t2_rdb0;
        *var_t2_rdb1_slot = var_t2_rdb1;
        *var_t2_rdb2_slot = var_t2_rdb2;
        *var_t2_rdb3_slot = var_t2_rdb3;
        *var_t2_rdb4_slot = var_t2_rdb4;
        *var_t2_rdn0_slot = var_t2_rdn0;
        *var_t2_rdn1_slot = var_t2_rdn1;
        *var_t2_rdn2_slot = var_t2_rdn2;
        *var_t2_rdn3_slot = var_t2_rdn3;
        *var_t2_rdn4_slot = var_t2_rdn4;
        *var_t2_rdn5_slot = var_t2_rdn5;
        *var_t2_rdn6_slot = var_t2_rdn6;
        *var_t2_rdn7_slot = var_t2_rdn7;
        *var_t2_rdn8_slot = var_t2_rdn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_db0_slot = var_t3_db0;
        *var_t3_db1_slot = var_t3_db1;
        *var_t3_db2_slot = var_t3_db2;
        *var_t3_db3_slot = var_t3_db3;
        *var_t3_db4_slot = var_t3_db4;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn1_slot = var_t3_dn1;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rdb0_slot = var_t3_rdb0;
        *var_t3_rdb1_slot = var_t3_rdb1;
        *var_t3_rdb2_slot = var_t3_rdb2;
        *var_t3_rdb3_slot = var_t3_rdb3;
        *var_t3_rdb4_slot = var_t3_rdb4;
        *var_t3_rdn0_slot = var_t3_rdn0;
        *var_t3_rdn1_slot = var_t3_rdn1;
        *var_t3_rdn2_slot = var_t3_rdn2;
        *var_t3_rdn3_slot = var_t3_rdn3;
        *var_t3_rdn4_slot = var_t3_rdn4;
        *var_t3_rdn5_slot = var_t3_rdn5;
        *var_t3_rdn6_slot = var_t3_rdn6;
        *var_t3_rdn7_slot = var_t3_rdn7;
        *var_t3_rdn8_slot = var_t3_rdn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_db0_slot = var_t4_db0;
        *var_t4_db1_slot = var_t4_db1;
        *var_t4_db2_slot = var_t4_db2;
        *var_t4_db3_slot = var_t4_db3;
        *var_t4_db4_slot = var_t4_db4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn1_slot = var_t4_dn1;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_rdb0_slot = var_t4_rdb0;
        *var_t4_rdb1_slot = var_t4_rdb1;
        *var_t4_rdb2_slot = var_t4_rdb2;
        *var_t4_rdb3_slot = var_t4_rdb3;
        *var_t4_rdb4_slot = var_t4_rdb4;
        *var_t4_rdn0_slot = var_t4_rdn0;
        *var_t4_rdn1_slot = var_t4_rdn1;
        *var_t4_rdn2_slot = var_t4_rdn2;
        *var_t4_rdn3_slot = var_t4_rdn3;
        *var_t4_rdn4_slot = var_t4_rdn4;
        *var_t4_rdn5_slot = var_t4_rdn5;
        *var_t4_rdn6_slot = var_t4_rdn6;
        *var_t4_rdn7_slot = var_t4_rdn7;
        *var_t4_rdn8_slot = var_t4_rdn8;
        *var_t4_rv_slot = var_t4_rv;
    }

    pub(super) fn stamp_reactive_block_88(
        p: &Parameters,
        var_cit_i: f64,
        var_cit_i_db0: f64,
        var_cit_i_db1: f64,
        var_cit_i_db2: f64,
        var_cit_i_db3: f64,
        var_cit_i_db4: f64,
        var_cit_i_dn0: f64,
        var_cit_i_dn1: f64,
        var_cit_i_dn2: f64,
        var_cit_i_dn3: f64,
        var_cit_i_dn4: f64,
        var_cit_i_dn5: f64,
        var_cit_i_dn6: f64,
        var_cit_i_dn7: f64,
        var_cit_i_dn8: f64,
        var_coxeff: f64,
        var_coxeff_db0: f64,
        var_coxeff_db1: f64,
        var_coxeff_db2: f64,
        var_coxeff_db3: f64,
        var_coxeff_db4: f64,
        var_coxeff_dn0: f64,
        var_coxeff_dn1: f64,
        var_coxeff_dn2: f64,
        var_coxeff_dn3: f64,
        var_coxeff_dn4: f64,
        var_coxeff_dn5: f64,
        var_coxeff_dn6: f64,
        var_coxeff_dn7: f64,
        var_coxeff_dn8: f64,
        var_guard131: f64,
        var_guard134: f64,
        var_ids: f64,
        var_ids_db0: f64,
        var_ids_db1: f64,
        var_ids_db2: f64,
        var_ids_db3: f64,
        var_ids_db4: f64,
        var_ids_dn0: f64,
        var_ids_dn1: f64,
        var_ids_dn2: f64,
        var_ids_dn3: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_leffnoisq: f64,
        var_leffnoisq_db0: f64,
        var_leffnoisq_db1: f64,
        var_leffnoisq_db2: f64,
        var_leffnoisq_db3: f64,
        var_leffnoisq_db4: f64,
        var_leffnoisq_dn0: f64,
        var_leffnoisq_dn1: f64,
        var_leffnoisq_dn2: f64,
        var_leffnoisq_dn3: f64,
        var_leffnoisq_dn4: f64,
        var_leffnoisq_dn5: f64,
        var_leffnoisq_dn6: f64,
        var_leffnoisq_dn7: f64,
        var_leffnoisq_dn8: f64,
        var_qid: f64,
        var_qid_db0: f64,
        var_qid_db1: f64,
        var_qid_db2: f64,
        var_qid_db3: f64,
        var_qid_db4: f64,
        var_qid_dn0: f64,
        var_qid_dn1: f64,
        var_qid_dn2: f64,
        var_qid_dn3: f64,
        var_qid_dn4: f64,
        var_qid_dn5: f64,
        var_qid_dn6: f64,
        var_qid_dn7: f64,
        var_qid_dn8: f64,
        var_qis: f64,
        var_qis_db0: f64,
        var_qis_db1: f64,
        var_qis_db2: f64,
        var_qis_db3: f64,
        var_qis_db4: f64,
        var_qis_dn0: f64,
        var_qis_dn1: f64,
        var_qis_dn2: f64,
        var_qis_dn3: f64,
        var_qis_dn4: f64,
        var_qis_dn5: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_utotal: f64,
        var_utotal_db0: f64,
        var_utotal_db1: f64,
        var_utotal_db2: f64,
        var_utotal_db3: f64,
        var_utotal_db4: f64,
        var_utotal_dn0: f64,
        var_utotal_dn1: f64,
        var_utotal_dn2: f64,
        var_utotal_dn3: f64,
        var_utotal_dn4: f64,
        var_utotal_dn5: f64,
        var_utotal_dn6: f64,
        var_utotal_dn7: f64,
        var_utotal_dn8: f64,
        var_vtm: f64,
        var_vtm_db0: f64,
        var_vtm_db1: f64,
        var_vtm_db2: f64,
        var_vtm_db3: f64,
        var_vtm_db4: f64,
        var_vtm_dn0: f64,
        var_vtm_dn1: f64,
        var_vtm_dn2: f64,
        var_vtm_dn3: f64,
        var_vtm_dn4: f64,
        var_vtm_dn5: f64,
        var_vtm_dn6: f64,
        var_vtm_dn7: f64,
        var_vtm_dn8: f64,
        var_n0_slot: &mut f64,
        var_n0_db0_slot: &mut f64,
        var_n0_db1_slot: &mut f64,
        var_n0_db2_slot: &mut f64,
        var_n0_db3_slot: &mut f64,
        var_n0_db4_slot: &mut f64,
        var_n0_dn0_slot: &mut f64,
        var_n0_dn1_slot: &mut f64,
        var_n0_dn2_slot: &mut f64,
        var_n0_dn3_slot: &mut f64,
        var_n0_dn4_slot: &mut f64,
        var_n0_dn5_slot: &mut f64,
        var_n0_dn6_slot: &mut f64,
        var_n0_dn7_slot: &mut f64,
        var_n0_dn8_slot: &mut f64,
        var_n0_rdb0_slot: &mut f64,
        var_n0_rdb1_slot: &mut f64,
        var_n0_rdb2_slot: &mut f64,
        var_n0_rdb3_slot: &mut f64,
        var_n0_rdb4_slot: &mut f64,
        var_n0_rdn0_slot: &mut f64,
        var_n0_rdn1_slot: &mut f64,
        var_n0_rdn2_slot: &mut f64,
        var_n0_rdn3_slot: &mut f64,
        var_n0_rdn4_slot: &mut f64,
        var_n0_rdn5_slot: &mut f64,
        var_n0_rdn6_slot: &mut f64,
        var_n0_rdn7_slot: &mut f64,
        var_n0_rdn8_slot: &mut f64,
        var_n0_rv_slot: &mut f64,
        var_nl_slot: &mut f64,
        var_nl_db0_slot: &mut f64,
        var_nl_db1_slot: &mut f64,
        var_nl_db2_slot: &mut f64,
        var_nl_db3_slot: &mut f64,
        var_nl_db4_slot: &mut f64,
        var_nl_dn0_slot: &mut f64,
        var_nl_dn1_slot: &mut f64,
        var_nl_dn2_slot: &mut f64,
        var_nl_dn3_slot: &mut f64,
        var_nl_dn4_slot: &mut f64,
        var_nl_dn5_slot: &mut f64,
        var_nl_dn6_slot: &mut f64,
        var_nl_dn7_slot: &mut f64,
        var_nl_dn8_slot: &mut f64,
        var_nl_rdb0_slot: &mut f64,
        var_nl_rdb1_slot: &mut f64,
        var_nl_rdb2_slot: &mut f64,
        var_nl_rdb3_slot: &mut f64,
        var_nl_rdb4_slot: &mut f64,
        var_nl_rdn0_slot: &mut f64,
        var_nl_rdn1_slot: &mut f64,
        var_nl_rdn2_slot: &mut f64,
        var_nl_rdn3_slot: &mut f64,
        var_nl_rdn4_slot: &mut f64,
        var_nl_rdn5_slot: &mut f64,
        var_nl_rdn6_slot: &mut f64,
        var_nl_rdn7_slot: &mut f64,
        var_nl_rdn8_slot: &mut f64,
        var_nl_rv_slot: &mut f64,
        var_noiaeff_slot: &mut f64,
        var_noiaeff_db0_slot: &mut f64,
        var_noiaeff_db1_slot: &mut f64,
        var_noiaeff_db2_slot: &mut f64,
        var_noiaeff_db3_slot: &mut f64,
        var_noiaeff_db4_slot: &mut f64,
        var_noiaeff_dn0_slot: &mut f64,
        var_noiaeff_dn1_slot: &mut f64,
        var_noiaeff_dn2_slot: &mut f64,
        var_noiaeff_dn3_slot: &mut f64,
        var_noiaeff_dn4_slot: &mut f64,
        var_noiaeff_dn5_slot: &mut f64,
        var_noiaeff_dn6_slot: &mut f64,
        var_noiaeff_dn7_slot: &mut f64,
        var_noiaeff_dn8_slot: &mut f64,
        var_noiaeff_rdb0_slot: &mut f64,
        var_noiaeff_rdb1_slot: &mut f64,
        var_noiaeff_rdb2_slot: &mut f64,
        var_noiaeff_rdb3_slot: &mut f64,
        var_noiaeff_rdb4_slot: &mut f64,
        var_noiaeff_rdn0_slot: &mut f64,
        var_noiaeff_rdn1_slot: &mut f64,
        var_noiaeff_rdn2_slot: &mut f64,
        var_noiaeff_rdn3_slot: &mut f64,
        var_noiaeff_rdn4_slot: &mut f64,
        var_noiaeff_rdn5_slot: &mut f64,
        var_noiaeff_rdn6_slot: &mut f64,
        var_noiaeff_rdn7_slot: &mut f64,
        var_noiaeff_rdn8_slot: &mut f64,
        var_noiaeff_rv_slot: &mut f64,
        var_nstar_slot: &mut f64,
        var_nstar_db0_slot: &mut f64,
        var_nstar_db1_slot: &mut f64,
        var_nstar_db2_slot: &mut f64,
        var_nstar_db3_slot: &mut f64,
        var_nstar_db4_slot: &mut f64,
        var_nstar_dn0_slot: &mut f64,
        var_nstar_dn1_slot: &mut f64,
        var_nstar_dn2_slot: &mut f64,
        var_nstar_dn3_slot: &mut f64,
        var_nstar_dn4_slot: &mut f64,
        var_nstar_dn5_slot: &mut f64,
        var_nstar_dn6_slot: &mut f64,
        var_nstar_dn7_slot: &mut f64,
        var_nstar_dn8_slot: &mut f64,
        var_nstar_rdb0_slot: &mut f64,
        var_nstar_rdb1_slot: &mut f64,
        var_nstar_rdb2_slot: &mut f64,
        var_nstar_rdb3_slot: &mut f64,
        var_nstar_rdb4_slot: &mut f64,
        var_nstar_rdn0_slot: &mut f64,
        var_nstar_rdn1_slot: &mut f64,
        var_nstar_rdn2_slot: &mut f64,
        var_nstar_rdn3_slot: &mut f64,
        var_nstar_rdn4_slot: &mut f64,
        var_nstar_rdn5_slot: &mut f64,
        var_nstar_rdn6_slot: &mut f64,
        var_nstar_rdn7_slot: &mut f64,
        var_nstar_rdn8_slot: &mut f64,
        var_nstar_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_db0_slot: &mut f64,
        var_t1_db1_slot: &mut f64,
        var_t1_db2_slot: &mut f64,
        var_t1_db3_slot: &mut f64,
        var_t1_db4_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rdb0_slot: &mut f64,
        var_t1_rdb1_slot: &mut f64,
        var_t1_rdb2_slot: &mut f64,
        var_t1_rdb3_slot: &mut f64,
        var_t1_rdb4_slot: &mut f64,
        var_t1_rdn0_slot: &mut f64,
        var_t1_rdn1_slot: &mut f64,
        var_t1_rdn2_slot: &mut f64,
        var_t1_rdn3_slot: &mut f64,
        var_t1_rdn4_slot: &mut f64,
        var_t1_rdn5_slot: &mut f64,
        var_t1_rdn6_slot: &mut f64,
        var_t1_rdn7_slot: &mut f64,
        var_t1_rdn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_db0_slot: &mut f64,
        var_t2_db1_slot: &mut f64,
        var_t2_db2_slot: &mut f64,
        var_t2_db3_slot: &mut f64,
        var_t2_db4_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rdb0_slot: &mut f64,
        var_t2_rdb1_slot: &mut f64,
        var_t2_rdb2_slot: &mut f64,
        var_t2_rdb3_slot: &mut f64,
        var_t2_rdb4_slot: &mut f64,
        var_t2_rdn0_slot: &mut f64,
        var_t2_rdn1_slot: &mut f64,
        var_t2_rdn2_slot: &mut f64,
        var_t2_rdn3_slot: &mut f64,
        var_t2_rdn4_slot: &mut f64,
        var_t2_rdn5_slot: &mut f64,
        var_t2_rdn6_slot: &mut f64,
        var_t2_rdn7_slot: &mut f64,
        var_t2_rdn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_db0_slot: &mut f64,
        var_t3_db1_slot: &mut f64,
        var_t3_db2_slot: &mut f64,
        var_t3_db3_slot: &mut f64,
        var_t3_db4_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn1_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rdb0_slot: &mut f64,
        var_t3_rdb1_slot: &mut f64,
        var_t3_rdb2_slot: &mut f64,
        var_t3_rdb3_slot: &mut f64,
        var_t3_rdb4_slot: &mut f64,
        var_t3_rdn0_slot: &mut f64,
        var_t3_rdn1_slot: &mut f64,
        var_t3_rdn2_slot: &mut f64,
        var_t3_rdn3_slot: &mut f64,
        var_t3_rdn4_slot: &mut f64,
        var_t3_rdn5_slot: &mut f64,
        var_t3_rdn6_slot: &mut f64,
        var_t3_rdn7_slot: &mut f64,
        var_t3_rdn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_db0_slot: &mut f64,
        var_t4_db1_slot: &mut f64,
        var_t4_db2_slot: &mut f64,
        var_t4_db3_slot: &mut f64,
        var_t4_db4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn1_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_rdb0_slot: &mut f64,
        var_t4_rdb1_slot: &mut f64,
        var_t4_rdb2_slot: &mut f64,
        var_t4_rdb3_slot: &mut f64,
        var_t4_rdb4_slot: &mut f64,
        var_t4_rdn0_slot: &mut f64,
        var_t4_rdn1_slot: &mut f64,
        var_t4_rdn2_slot: &mut f64,
        var_t4_rdn3_slot: &mut f64,
        var_t4_rdn4_slot: &mut f64,
        var_t4_rdn5_slot: &mut f64,
        var_t4_rdn6_slot: &mut f64,
        var_t4_rdn7_slot: &mut f64,
        var_t4_rdn8_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_db0_slot: &mut f64,
        var_t5_db1_slot: &mut f64,
        var_t5_db2_slot: &mut f64,
        var_t5_db3_slot: &mut f64,
        var_t5_db4_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn1_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_rdb0_slot: &mut f64,
        var_t5_rdb1_slot: &mut f64,
        var_t5_rdb2_slot: &mut f64,
        var_t5_rdb3_slot: &mut f64,
        var_t5_rdb4_slot: &mut f64,
        var_t5_rdn0_slot: &mut f64,
        var_t5_rdn1_slot: &mut f64,
        var_t5_rdn2_slot: &mut f64,
        var_t5_rdn3_slot: &mut f64,
        var_t5_rdn4_slot: &mut f64,
        var_t5_rdn5_slot: &mut f64,
        var_t5_rdn6_slot: &mut f64,
        var_t5_rdn7_slot: &mut f64,
        var_t5_rdn8_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_db0_slot: &mut f64,
        var_t6_db1_slot: &mut f64,
        var_t6_db2_slot: &mut f64,
        var_t6_db3_slot: &mut f64,
        var_t6_db4_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn1_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_rdb0_slot: &mut f64,
        var_t6_rdb1_slot: &mut f64,
        var_t6_rdb2_slot: &mut f64,
        var_t6_rdb3_slot: &mut f64,
        var_t6_rdb4_slot: &mut f64,
        var_t6_rdn0_slot: &mut f64,
        var_t6_rdn1_slot: &mut f64,
        var_t6_rdn2_slot: &mut f64,
        var_t6_rdn3_slot: &mut f64,
        var_t6_rdn4_slot: &mut f64,
        var_t6_rdn5_slot: &mut f64,
        var_t6_rdn6_slot: &mut f64,
        var_t6_rdn7_slot: &mut f64,
        var_t6_rdn8_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
    ) {
        let mut var_n0: f64 = *var_n0_slot;
        let mut var_n0_db0: f64 = *var_n0_db0_slot;
        let mut var_n0_db1: f64 = *var_n0_db1_slot;
        let mut var_n0_db2: f64 = *var_n0_db2_slot;
        let mut var_n0_db3: f64 = *var_n0_db3_slot;
        let mut var_n0_db4: f64 = *var_n0_db4_slot;
        let mut var_n0_dn0: f64 = *var_n0_dn0_slot;
        let mut var_n0_dn1: f64 = *var_n0_dn1_slot;
        let mut var_n0_dn2: f64 = *var_n0_dn2_slot;
        let mut var_n0_dn3: f64 = *var_n0_dn3_slot;
        let mut var_n0_dn4: f64 = *var_n0_dn4_slot;
        let mut var_n0_dn5: f64 = *var_n0_dn5_slot;
        let mut var_n0_dn6: f64 = *var_n0_dn6_slot;
        let mut var_n0_dn7: f64 = *var_n0_dn7_slot;
        let mut var_n0_dn8: f64 = *var_n0_dn8_slot;
        let mut var_n0_rdb0: f64 = *var_n0_rdb0_slot;
        let mut var_n0_rdb1: f64 = *var_n0_rdb1_slot;
        let mut var_n0_rdb2: f64 = *var_n0_rdb2_slot;
        let mut var_n0_rdb3: f64 = *var_n0_rdb3_slot;
        let mut var_n0_rdb4: f64 = *var_n0_rdb4_slot;
        let mut var_n0_rdn0: f64 = *var_n0_rdn0_slot;
        let mut var_n0_rdn1: f64 = *var_n0_rdn1_slot;
        let mut var_n0_rdn2: f64 = *var_n0_rdn2_slot;
        let mut var_n0_rdn3: f64 = *var_n0_rdn3_slot;
        let mut var_n0_rdn4: f64 = *var_n0_rdn4_slot;
        let mut var_n0_rdn5: f64 = *var_n0_rdn5_slot;
        let mut var_n0_rdn6: f64 = *var_n0_rdn6_slot;
        let mut var_n0_rdn7: f64 = *var_n0_rdn7_slot;
        let mut var_n0_rdn8: f64 = *var_n0_rdn8_slot;
        let mut var_n0_rv: f64 = *var_n0_rv_slot;
        let mut var_nl: f64 = *var_nl_slot;
        let mut var_nl_db0: f64 = *var_nl_db0_slot;
        let mut var_nl_db1: f64 = *var_nl_db1_slot;
        let mut var_nl_db2: f64 = *var_nl_db2_slot;
        let mut var_nl_db3: f64 = *var_nl_db3_slot;
        let mut var_nl_db4: f64 = *var_nl_db4_slot;
        let mut var_nl_dn0: f64 = *var_nl_dn0_slot;
        let mut var_nl_dn1: f64 = *var_nl_dn1_slot;
        let mut var_nl_dn2: f64 = *var_nl_dn2_slot;
        let mut var_nl_dn3: f64 = *var_nl_dn3_slot;
        let mut var_nl_dn4: f64 = *var_nl_dn4_slot;
        let mut var_nl_dn5: f64 = *var_nl_dn5_slot;
        let mut var_nl_dn6: f64 = *var_nl_dn6_slot;
        let mut var_nl_dn7: f64 = *var_nl_dn7_slot;
        let mut var_nl_dn8: f64 = *var_nl_dn8_slot;
        let mut var_nl_rdb0: f64 = *var_nl_rdb0_slot;
        let mut var_nl_rdb1: f64 = *var_nl_rdb1_slot;
        let mut var_nl_rdb2: f64 = *var_nl_rdb2_slot;
        let mut var_nl_rdb3: f64 = *var_nl_rdb3_slot;
        let mut var_nl_rdb4: f64 = *var_nl_rdb4_slot;
        let mut var_nl_rdn0: f64 = *var_nl_rdn0_slot;
        let mut var_nl_rdn1: f64 = *var_nl_rdn1_slot;
        let mut var_nl_rdn2: f64 = *var_nl_rdn2_slot;
        let mut var_nl_rdn3: f64 = *var_nl_rdn3_slot;
        let mut var_nl_rdn4: f64 = *var_nl_rdn4_slot;
        let mut var_nl_rdn5: f64 = *var_nl_rdn5_slot;
        let mut var_nl_rdn6: f64 = *var_nl_rdn6_slot;
        let mut var_nl_rdn7: f64 = *var_nl_rdn7_slot;
        let mut var_nl_rdn8: f64 = *var_nl_rdn8_slot;
        let mut var_nl_rv: f64 = *var_nl_rv_slot;
        let mut var_noiaeff: f64 = *var_noiaeff_slot;
        let mut var_noiaeff_db0: f64 = *var_noiaeff_db0_slot;
        let mut var_noiaeff_db1: f64 = *var_noiaeff_db1_slot;
        let mut var_noiaeff_db2: f64 = *var_noiaeff_db2_slot;
        let mut var_noiaeff_db3: f64 = *var_noiaeff_db3_slot;
        let mut var_noiaeff_db4: f64 = *var_noiaeff_db4_slot;
        let mut var_noiaeff_dn0: f64 = *var_noiaeff_dn0_slot;
        let mut var_noiaeff_dn1: f64 = *var_noiaeff_dn1_slot;
        let mut var_noiaeff_dn2: f64 = *var_noiaeff_dn2_slot;
        let mut var_noiaeff_dn3: f64 = *var_noiaeff_dn3_slot;
        let mut var_noiaeff_dn4: f64 = *var_noiaeff_dn4_slot;
        let mut var_noiaeff_dn5: f64 = *var_noiaeff_dn5_slot;
        let mut var_noiaeff_dn6: f64 = *var_noiaeff_dn6_slot;
        let mut var_noiaeff_dn7: f64 = *var_noiaeff_dn7_slot;
        let mut var_noiaeff_dn8: f64 = *var_noiaeff_dn8_slot;
        let mut var_noiaeff_rdb0: f64 = *var_noiaeff_rdb0_slot;
        let mut var_noiaeff_rdb1: f64 = *var_noiaeff_rdb1_slot;
        let mut var_noiaeff_rdb2: f64 = *var_noiaeff_rdb2_slot;
        let mut var_noiaeff_rdb3: f64 = *var_noiaeff_rdb3_slot;
        let mut var_noiaeff_rdb4: f64 = *var_noiaeff_rdb4_slot;
        let mut var_noiaeff_rdn0: f64 = *var_noiaeff_rdn0_slot;
        let mut var_noiaeff_rdn1: f64 = *var_noiaeff_rdn1_slot;
        let mut var_noiaeff_rdn2: f64 = *var_noiaeff_rdn2_slot;
        let mut var_noiaeff_rdn3: f64 = *var_noiaeff_rdn3_slot;
        let mut var_noiaeff_rdn4: f64 = *var_noiaeff_rdn4_slot;
        let mut var_noiaeff_rdn5: f64 = *var_noiaeff_rdn5_slot;
        let mut var_noiaeff_rdn6: f64 = *var_noiaeff_rdn6_slot;
        let mut var_noiaeff_rdn7: f64 = *var_noiaeff_rdn7_slot;
        let mut var_noiaeff_rdn8: f64 = *var_noiaeff_rdn8_slot;
        let mut var_noiaeff_rv: f64 = *var_noiaeff_rv_slot;
        let mut var_nstar: f64 = *var_nstar_slot;
        let mut var_nstar_db0: f64 = *var_nstar_db0_slot;
        let mut var_nstar_db1: f64 = *var_nstar_db1_slot;
        let mut var_nstar_db2: f64 = *var_nstar_db2_slot;
        let mut var_nstar_db3: f64 = *var_nstar_db3_slot;
        let mut var_nstar_db4: f64 = *var_nstar_db4_slot;
        let mut var_nstar_dn0: f64 = *var_nstar_dn0_slot;
        let mut var_nstar_dn1: f64 = *var_nstar_dn1_slot;
        let mut var_nstar_dn2: f64 = *var_nstar_dn2_slot;
        let mut var_nstar_dn3: f64 = *var_nstar_dn3_slot;
        let mut var_nstar_dn4: f64 = *var_nstar_dn4_slot;
        let mut var_nstar_dn5: f64 = *var_nstar_dn5_slot;
        let mut var_nstar_dn6: f64 = *var_nstar_dn6_slot;
        let mut var_nstar_dn7: f64 = *var_nstar_dn7_slot;
        let mut var_nstar_dn8: f64 = *var_nstar_dn8_slot;
        let mut var_nstar_rdb0: f64 = *var_nstar_rdb0_slot;
        let mut var_nstar_rdb1: f64 = *var_nstar_rdb1_slot;
        let mut var_nstar_rdb2: f64 = *var_nstar_rdb2_slot;
        let mut var_nstar_rdb3: f64 = *var_nstar_rdb3_slot;
        let mut var_nstar_rdb4: f64 = *var_nstar_rdb4_slot;
        let mut var_nstar_rdn0: f64 = *var_nstar_rdn0_slot;
        let mut var_nstar_rdn1: f64 = *var_nstar_rdn1_slot;
        let mut var_nstar_rdn2: f64 = *var_nstar_rdn2_slot;
        let mut var_nstar_rdn3: f64 = *var_nstar_rdn3_slot;
        let mut var_nstar_rdn4: f64 = *var_nstar_rdn4_slot;
        let mut var_nstar_rdn5: f64 = *var_nstar_rdn5_slot;
        let mut var_nstar_rdn6: f64 = *var_nstar_rdn6_slot;
        let mut var_nstar_rdn7: f64 = *var_nstar_rdn7_slot;
        let mut var_nstar_rdn8: f64 = *var_nstar_rdn8_slot;
        let mut var_nstar_rv: f64 = *var_nstar_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_db0: f64 = *var_t1_db0_slot;
        let mut var_t1_db1: f64 = *var_t1_db1_slot;
        let mut var_t1_db2: f64 = *var_t1_db2_slot;
        let mut var_t1_db3: f64 = *var_t1_db3_slot;
        let mut var_t1_db4: f64 = *var_t1_db4_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rdb0: f64 = *var_t1_rdb0_slot;
        let mut var_t1_rdb1: f64 = *var_t1_rdb1_slot;
        let mut var_t1_rdb2: f64 = *var_t1_rdb2_slot;
        let mut var_t1_rdb3: f64 = *var_t1_rdb3_slot;
        let mut var_t1_rdb4: f64 = *var_t1_rdb4_slot;
        let mut var_t1_rdn0: f64 = *var_t1_rdn0_slot;
        let mut var_t1_rdn1: f64 = *var_t1_rdn1_slot;
        let mut var_t1_rdn2: f64 = *var_t1_rdn2_slot;
        let mut var_t1_rdn3: f64 = *var_t1_rdn3_slot;
        let mut var_t1_rdn4: f64 = *var_t1_rdn4_slot;
        let mut var_t1_rdn5: f64 = *var_t1_rdn5_slot;
        let mut var_t1_rdn6: f64 = *var_t1_rdn6_slot;
        let mut var_t1_rdn7: f64 = *var_t1_rdn7_slot;
        let mut var_t1_rdn8: f64 = *var_t1_rdn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_db0: f64 = *var_t2_db0_slot;
        let mut var_t2_db1: f64 = *var_t2_db1_slot;
        let mut var_t2_db2: f64 = *var_t2_db2_slot;
        let mut var_t2_db3: f64 = *var_t2_db3_slot;
        let mut var_t2_db4: f64 = *var_t2_db4_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rdb0: f64 = *var_t2_rdb0_slot;
        let mut var_t2_rdb1: f64 = *var_t2_rdb1_slot;
        let mut var_t2_rdb2: f64 = *var_t2_rdb2_slot;
        let mut var_t2_rdb3: f64 = *var_t2_rdb3_slot;
        let mut var_t2_rdb4: f64 = *var_t2_rdb4_slot;
        let mut var_t2_rdn0: f64 = *var_t2_rdn0_slot;
        let mut var_t2_rdn1: f64 = *var_t2_rdn1_slot;
        let mut var_t2_rdn2: f64 = *var_t2_rdn2_slot;
        let mut var_t2_rdn3: f64 = *var_t2_rdn3_slot;
        let mut var_t2_rdn4: f64 = *var_t2_rdn4_slot;
        let mut var_t2_rdn5: f64 = *var_t2_rdn5_slot;
        let mut var_t2_rdn6: f64 = *var_t2_rdn6_slot;
        let mut var_t2_rdn7: f64 = *var_t2_rdn7_slot;
        let mut var_t2_rdn8: f64 = *var_t2_rdn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_db0: f64 = *var_t3_db0_slot;
        let mut var_t3_db1: f64 = *var_t3_db1_slot;
        let mut var_t3_db2: f64 = *var_t3_db2_slot;
        let mut var_t3_db3: f64 = *var_t3_db3_slot;
        let mut var_t3_db4: f64 = *var_t3_db4_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn1: f64 = *var_t3_dn1_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rdb0: f64 = *var_t3_rdb0_slot;
        let mut var_t3_rdb1: f64 = *var_t3_rdb1_slot;
        let mut var_t3_rdb2: f64 = *var_t3_rdb2_slot;
        let mut var_t3_rdb3: f64 = *var_t3_rdb3_slot;
        let mut var_t3_rdb4: f64 = *var_t3_rdb4_slot;
        let mut var_t3_rdn0: f64 = *var_t3_rdn0_slot;
        let mut var_t3_rdn1: f64 = *var_t3_rdn1_slot;
        let mut var_t3_rdn2: f64 = *var_t3_rdn2_slot;
        let mut var_t3_rdn3: f64 = *var_t3_rdn3_slot;
        let mut var_t3_rdn4: f64 = *var_t3_rdn4_slot;
        let mut var_t3_rdn5: f64 = *var_t3_rdn5_slot;
        let mut var_t3_rdn6: f64 = *var_t3_rdn6_slot;
        let mut var_t3_rdn7: f64 = *var_t3_rdn7_slot;
        let mut var_t3_rdn8: f64 = *var_t3_rdn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_db0: f64 = *var_t4_db0_slot;
        let mut var_t4_db1: f64 = *var_t4_db1_slot;
        let mut var_t4_db2: f64 = *var_t4_db2_slot;
        let mut var_t4_db3: f64 = *var_t4_db3_slot;
        let mut var_t4_db4: f64 = *var_t4_db4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn1: f64 = *var_t4_dn1_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_rdb0: f64 = *var_t4_rdb0_slot;
        let mut var_t4_rdb1: f64 = *var_t4_rdb1_slot;
        let mut var_t4_rdb2: f64 = *var_t4_rdb2_slot;
        let mut var_t4_rdb3: f64 = *var_t4_rdb3_slot;
        let mut var_t4_rdb4: f64 = *var_t4_rdb4_slot;
        let mut var_t4_rdn0: f64 = *var_t4_rdn0_slot;
        let mut var_t4_rdn1: f64 = *var_t4_rdn1_slot;
        let mut var_t4_rdn2: f64 = *var_t4_rdn2_slot;
        let mut var_t4_rdn3: f64 = *var_t4_rdn3_slot;
        let mut var_t4_rdn4: f64 = *var_t4_rdn4_slot;
        let mut var_t4_rdn5: f64 = *var_t4_rdn5_slot;
        let mut var_t4_rdn6: f64 = *var_t4_rdn6_slot;
        let mut var_t4_rdn7: f64 = *var_t4_rdn7_slot;
        let mut var_t4_rdn8: f64 = *var_t4_rdn8_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_db0: f64 = *var_t5_db0_slot;
        let mut var_t5_db1: f64 = *var_t5_db1_slot;
        let mut var_t5_db2: f64 = *var_t5_db2_slot;
        let mut var_t5_db3: f64 = *var_t5_db3_slot;
        let mut var_t5_db4: f64 = *var_t5_db4_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn1: f64 = *var_t5_dn1_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_rdb0: f64 = *var_t5_rdb0_slot;
        let mut var_t5_rdb1: f64 = *var_t5_rdb1_slot;
        let mut var_t5_rdb2: f64 = *var_t5_rdb2_slot;
        let mut var_t5_rdb3: f64 = *var_t5_rdb3_slot;
        let mut var_t5_rdb4: f64 = *var_t5_rdb4_slot;
        let mut var_t5_rdn0: f64 = *var_t5_rdn0_slot;
        let mut var_t5_rdn1: f64 = *var_t5_rdn1_slot;
        let mut var_t5_rdn2: f64 = *var_t5_rdn2_slot;
        let mut var_t5_rdn3: f64 = *var_t5_rdn3_slot;
        let mut var_t5_rdn4: f64 = *var_t5_rdn4_slot;
        let mut var_t5_rdn5: f64 = *var_t5_rdn5_slot;
        let mut var_t5_rdn6: f64 = *var_t5_rdn6_slot;
        let mut var_t5_rdn7: f64 = *var_t5_rdn7_slot;
        let mut var_t5_rdn8: f64 = *var_t5_rdn8_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_db0: f64 = *var_t6_db0_slot;
        let mut var_t6_db1: f64 = *var_t6_db1_slot;
        let mut var_t6_db2: f64 = *var_t6_db2_slot;
        let mut var_t6_db3: f64 = *var_t6_db3_slot;
        let mut var_t6_db4: f64 = *var_t6_db4_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn1: f64 = *var_t6_dn1_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_rdb0: f64 = *var_t6_rdb0_slot;
        let mut var_t6_rdb1: f64 = *var_t6_rdb1_slot;
        let mut var_t6_rdb2: f64 = *var_t6_rdb2_slot;
        let mut var_t6_rdb3: f64 = *var_t6_rdb3_slot;
        let mut var_t6_rdb4: f64 = *var_t6_rdb4_slot;
        let mut var_t6_rdn0: f64 = *var_t6_rdn0_slot;
        let mut var_t6_rdn1: f64 = *var_t6_rdn1_slot;
        let mut var_t6_rdn2: f64 = *var_t6_rdn2_slot;
        let mut var_t6_rdn3: f64 = *var_t6_rdn3_slot;
        let mut var_t6_rdn4: f64 = *var_t6_rdn4_slot;
        let mut var_t6_rdn5: f64 = *var_t6_rdn5_slot;
        let mut var_t6_rdn6: f64 = *var_t6_rdn6_slot;
        let mut var_t6_rdn7: f64 = *var_t6_rdn7_slot;
        let mut var_t6_rdn8: f64 = *var_t6_rdn8_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;

        let (assign12830_e11919, assign12830_e11919_d_n0, assign12830_e11919_d_n1, assign12830_e11919_d_n2, assign12830_e11919_d_n3, assign12830_e11919_d_n4, assign12830_e11919_d_n5, assign12830_e11919_d_n6, assign12830_e11919_d_n7, assign12830_e11919_d_n8, assign12830_e11919_d_b0, assign12830_e11919_d_b1, assign12830_e11919_d_b2, assign12830_e11919_d_b3, assign12830_e11919_d_b4,) = {
    if ((var_guard131 != 0.0) && (var_guard134 != 0.0)) {
        let assign12830_e11901: f64 = (var_t4 + 1.0);
        let assign12830_e11904: f64 = (var_t4 - 1.0);
        let assign12830_e11907: f64 = (var_t4 - 1.0);
        let assign12830_e11908: f64 = (assign12830_e11904 * assign12830_e11907);
        let assign12830_e11911: f64 = (0.25 * p.p292);
        let assign12830_e11913: f64 = (assign12830_e11911 * p.p292);
        let assign12830_e11914: f64 = (assign12830_e11908 + assign12830_e11913);
        let assign12830_e11915: f64 = (assign12830_e11914).sqrt();
        let assign12830_e11916: f64 = (assign12830_e11901 + assign12830_e11915);
        let assign12830_e11917: f64 = (0.5 * assign12830_e11916);
        (assign12830_e11917, (0.5 * (var_t4_dn0 + (((var_t4_dn0 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn0)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_dn1 + (((var_t4_dn1 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn1)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_dn2 + (((var_t4_dn2 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn2)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_dn3 + (((var_t4_dn3 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn3)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_dn4 + (((var_t4_dn4 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn4)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_dn5 + (((var_t4_dn5 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn5)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_dn6 + (((var_t4_dn6 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn6)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_dn7 + (((var_t4_dn7 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn7)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_dn8 + (((var_t4_dn8 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn8)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_db0 + (((var_t4_db0 * assign12830_e11907) + (assign12830_e11904 * var_t4_db0)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_db1 + (((var_t4_db1 * assign12830_e11907) + (assign12830_e11904 * var_t4_db1)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_db2 + (((var_t4_db2 * assign12830_e11907) + (assign12830_e11904 * var_t4_db2)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_db3 + (((var_t4_db3 * assign12830_e11907) + (assign12830_e11904 * var_t4_db3)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_db4 + (((var_t4_db4 * assign12830_e11907) + (assign12830_e11904 * var_t4_db4)) / (2.0 * assign12830_e11915)))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn1, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_db0, var_t5_db1, var_t5_db2, var_t5_db3, var_t5_db4,)
    }
};
        var_t5 = assign12830_e11919;
        var_t5_dn0 = assign12830_e11919_d_n0;
        var_t5_dn1 = assign12830_e11919_d_n1;
        var_t5_dn2 = assign12830_e11919_d_n2;
        var_t5_dn3 = assign12830_e11919_d_n3;
        var_t5_dn4 = assign12830_e11919_d_n4;
        var_t5_dn5 = assign12830_e11919_d_n5;
        var_t5_dn6 = assign12830_e11919_d_n6;
        var_t5_dn7 = assign12830_e11919_d_n7;
        var_t5_dn8 = assign12830_e11919_d_n8;
        var_t5_db0 = assign12830_e11919_d_b0;
        var_t5_db1 = assign12830_e11919_d_b1;
        var_t5_db2 = assign12830_e11919_d_b2;
        var_t5_db3 = assign12830_e11919_d_b3;
        var_t5_db4 = assign12830_e11919_d_b4;
        var_t5_rv = 0.0;
        var_t5_rdn0 = 0.0;
        var_t5_rdn1 = 0.0;
        var_t5_rdn2 = 0.0;
        var_t5_rdn3 = 0.0;
        var_t5_rdn4 = 0.0;
        var_t5_rdn5 = 0.0;
        var_t5_rdn6 = 0.0;
        var_t5_rdn7 = 0.0;
        var_t5_rdn8 = 0.0;
        var_t5_rdb0 = 0.0;
        var_t5_rdb1 = 0.0;
        var_t5_rdb2 = 0.0;
        var_t5_rdb3 = 0.0;
        var_t5_rdb4 = 0.0;

        let (assign12840_e11927, assign12840_e11927_d_n0, assign12840_e11927_d_n1, assign12840_e11927_d_n2, assign12840_e11927_d_n3, assign12840_e11927_d_n4, assign12840_e11927_d_n5, assign12840_e11927_d_n6, assign12840_e11927_d_n7, assign12840_e11927_d_n8, assign12840_e11927_d_b0, assign12840_e11927_d_b1, assign12840_e11927_d_b2, assign12840_e11927_d_b3, assign12840_e11927_d_b4,) = {
    if ((var_guard131 != 0.0) && (var_guard134 != 0.0)) {
        let assign12840_e11925: f64 = (p.p288 * var_t5);
        (assign12840_e11925, (p.p288 * var_t5_dn0), (p.p288 * var_t5_dn1), (p.p288 * var_t5_dn2), (p.p288 * var_t5_dn3), (p.p288 * var_t5_dn4), (p.p288 * var_t5_dn5), (p.p288 * var_t5_dn6), (p.p288 * var_t5_dn7), (p.p288 * var_t5_dn8), (p.p288 * var_t5_db0), (p.p288 * var_t5_db1), (p.p288 * var_t5_db2), (p.p288 * var_t5_db3), (p.p288 * var_t5_db4),)
    } else {
        (var_noiaeff, var_noiaeff_dn0, var_noiaeff_dn1, var_noiaeff_dn2, var_noiaeff_dn3, var_noiaeff_dn4, var_noiaeff_dn5, var_noiaeff_dn6, var_noiaeff_dn7, var_noiaeff_dn8, var_noiaeff_db0, var_noiaeff_db1, var_noiaeff_db2, var_noiaeff_db3, var_noiaeff_db4,)
    }
};
        var_noiaeff = assign12840_e11927;
        var_noiaeff_dn0 = assign12840_e11927_d_n0;
        var_noiaeff_dn1 = assign12840_e11927_d_n1;
        var_noiaeff_dn2 = assign12840_e11927_d_n2;
        var_noiaeff_dn3 = assign12840_e11927_d_n3;
        var_noiaeff_dn4 = assign12840_e11927_d_n4;
        var_noiaeff_dn5 = assign12840_e11927_d_n5;
        var_noiaeff_dn6 = assign12840_e11927_d_n6;
        var_noiaeff_dn7 = assign12840_e11927_d_n7;
        var_noiaeff_dn8 = assign12840_e11927_d_n8;
        var_noiaeff_db0 = assign12840_e11927_d_b0;
        var_noiaeff_db1 = assign12840_e11927_d_b1;
        var_noiaeff_db2 = assign12840_e11927_d_b2;
        var_noiaeff_db3 = assign12840_e11927_d_b3;
        var_noiaeff_db4 = assign12840_e11927_d_b4;
        var_noiaeff_rv = 0.0;
        var_noiaeff_rdn0 = 0.0;
        var_noiaeff_rdn1 = 0.0;
        var_noiaeff_rdn2 = 0.0;
        var_noiaeff_rdn3 = 0.0;
        var_noiaeff_rdn4 = 0.0;
        var_noiaeff_rdn5 = 0.0;
        var_noiaeff_rdn6 = 0.0;
        var_noiaeff_rdn7 = 0.0;
        var_noiaeff_rdn8 = 0.0;
        var_noiaeff_rdb0 = 0.0;
        var_noiaeff_rdb1 = 0.0;
        var_noiaeff_rdb2 = 0.0;
        var_noiaeff_rdb3 = 0.0;
        var_noiaeff_rdb4 = 0.0;

        let (assign12850_e11934, assign12850_e11934_d_n0, assign12850_e11934_d_n1, assign12850_e11934_d_n2, assign12850_e11934_d_n3, assign12850_e11934_d_n4, assign12850_e11934_d_n5, assign12850_e11934_d_n6, assign12850_e11934_d_n7, assign12850_e11934_d_n8, assign12850_e11934_d_b0, assign12850_e11934_d_b1, assign12850_e11934_d_b2, assign12850_e11934_d_b3, assign12850_e11934_d_b4,) = {
    if ((var_guard131 != 0.0) && (var_guard134 == 0.0)) {
        (p.p288, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_noiaeff, var_noiaeff_dn0, var_noiaeff_dn1, var_noiaeff_dn2, var_noiaeff_dn3, var_noiaeff_dn4, var_noiaeff_dn5, var_noiaeff_dn6, var_noiaeff_dn7, var_noiaeff_dn8, var_noiaeff_db0, var_noiaeff_db1, var_noiaeff_db2, var_noiaeff_db3, var_noiaeff_db4,)
    }
};
        var_noiaeff = assign12850_e11934;
        var_noiaeff_dn0 = assign12850_e11934_d_n0;
        var_noiaeff_dn1 = assign12850_e11934_d_n1;
        var_noiaeff_dn2 = assign12850_e11934_d_n2;
        var_noiaeff_dn3 = assign12850_e11934_d_n3;
        var_noiaeff_dn4 = assign12850_e11934_d_n4;
        var_noiaeff_dn5 = assign12850_e11934_d_n5;
        var_noiaeff_dn6 = assign12850_e11934_d_n6;
        var_noiaeff_dn7 = assign12850_e11934_d_n7;
        var_noiaeff_dn8 = assign12850_e11934_d_n8;
        var_noiaeff_db0 = assign12850_e11934_d_b0;
        var_noiaeff_db1 = assign12850_e11934_d_b1;
        var_noiaeff_db2 = assign12850_e11934_d_b2;
        var_noiaeff_db3 = assign12850_e11934_d_b3;
        var_noiaeff_db4 = assign12850_e11934_d_b4;
        var_noiaeff_rv = 0.0;
        var_noiaeff_rdn0 = 0.0;
        var_noiaeff_rdn1 = 0.0;
        var_noiaeff_rdn2 = 0.0;
        var_noiaeff_rdn3 = 0.0;
        var_noiaeff_rdn4 = 0.0;
        var_noiaeff_rdn5 = 0.0;
        var_noiaeff_rdn6 = 0.0;
        var_noiaeff_rdn7 = 0.0;
        var_noiaeff_rdn8 = 0.0;
        var_noiaeff_rdb0 = 0.0;
        var_noiaeff_rdb1 = 0.0;
        var_noiaeff_rdb2 = 0.0;
        var_noiaeff_rdb3 = 0.0;
        var_noiaeff_rdb4 = 0.0;

        let (assign12860_e11949, assign12860_e11949_d_n0, assign12860_e11949_d_n1, assign12860_e11949_d_n2, assign12860_e11949_d_n3, assign12860_e11949_d_n4, assign12860_e11949_d_n5, assign12860_e11949_d_n6, assign12860_e11949_d_n7, assign12860_e11949_d_n8, assign12860_e11949_d_b0, assign12860_e11949_d_b1, assign12860_e11949_d_b2, assign12860_e11949_d_b3, assign12860_e11949_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12860_e11938: f64 = (1.60219e-19 * 1.60219e-19);
        let assign12860_e11940: f64 = (assign12860_e11938 * 1.60219e-19);
        let assign12860_e11942: f64 = (assign12860_e11940 * var_vtm);
        let assign12860_e11944: f64 = (var_ids).abs();
        let assign12860_e11945: f64 = (assign12860_e11942 * assign12860_e11944);
        let assign12860_e11947: f64 = (assign12860_e11945 * var_utotal);
        (assign12860_e11947, (((((assign12860_e11940 * var_vtm_dn0) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn0 } else { (-var_ids_dn0) })) * var_utotal) + (assign12860_e11945 * var_utotal_dn0)), (((((assign12860_e11940 * var_vtm_dn1) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn1 } else { (-var_ids_dn1) })) * var_utotal) + (assign12860_e11945 * var_utotal_dn1)), (((((assign12860_e11940 * var_vtm_dn2) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn2 } else { (-var_ids_dn2) })) * var_utotal) + (assign12860_e11945 * var_utotal_dn2)), (((((assign12860_e11940 * var_vtm_dn3) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn3 } else { (-var_ids_dn3) })) * var_utotal) + (assign12860_e11945 * var_utotal_dn3)), (((((assign12860_e11940 * var_vtm_dn4) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn4 } else { (-var_ids_dn4) })) * var_utotal) + (assign12860_e11945 * var_utotal_dn4)), (((((assign12860_e11940 * var_vtm_dn5) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn5 } else { (-var_ids_dn5) })) * var_utotal) + (assign12860_e11945 * var_utotal_dn5)), (((((assign12860_e11940 * var_vtm_dn6) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn6 } else { (-var_ids_dn6) })) * var_utotal) + (assign12860_e11945 * var_utotal_dn6)), (((((assign12860_e11940 * var_vtm_dn7) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn7 } else { (-var_ids_dn7) })) * var_utotal) + (assign12860_e11945 * var_utotal_dn7)), (((((assign12860_e11940 * var_vtm_dn8) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn8 } else { (-var_ids_dn8) })) * var_utotal) + (assign12860_e11945 * var_utotal_dn8)), (((((assign12860_e11940 * var_vtm_db0) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_db0 } else { (-var_ids_db0) })) * var_utotal) + (assign12860_e11945 * var_utotal_db0)), (((((assign12860_e11940 * var_vtm_db1) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_db1 } else { (-var_ids_db1) })) * var_utotal) + (assign12860_e11945 * var_utotal_db1)), (((((assign12860_e11940 * var_vtm_db2) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_db2 } else { (-var_ids_db2) })) * var_utotal) + (assign12860_e11945 * var_utotal_db2)), (((((assign12860_e11940 * var_vtm_db3) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_db3 } else { (-var_ids_db3) })) * var_utotal) + (assign12860_e11945 * var_utotal_db3)), (((((assign12860_e11940 * var_vtm_db4) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_db4 } else { (-var_ids_db4) })) * var_utotal) + (assign12860_e11945 * var_utotal_db4)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4,)
    }
};
        var_t1 = assign12860_e11949;
        var_t1_dn0 = assign12860_e11949_d_n0;
        var_t1_dn1 = assign12860_e11949_d_n1;
        var_t1_dn2 = assign12860_e11949_d_n2;
        var_t1_dn3 = assign12860_e11949_d_n3;
        var_t1_dn4 = assign12860_e11949_d_n4;
        var_t1_dn5 = assign12860_e11949_d_n5;
        var_t1_dn6 = assign12860_e11949_d_n6;
        var_t1_dn7 = assign12860_e11949_d_n7;
        var_t1_dn8 = assign12860_e11949_d_n8;
        var_t1_db0 = assign12860_e11949_d_b0;
        var_t1_db1 = assign12860_e11949_d_b1;
        var_t1_db2 = assign12860_e11949_d_b2;
        var_t1_db3 = assign12860_e11949_d_b3;
        var_t1_db4 = assign12860_e11949_d_b4;
        var_t1_rv = 0.0;
        var_t1_rdn0 = 0.0;
        var_t1_rdn1 = 0.0;
        var_t1_rdn2 = 0.0;
        var_t1_rdn3 = 0.0;
        var_t1_rdn4 = 0.0;
        var_t1_rdn5 = 0.0;
        var_t1_rdn6 = 0.0;
        var_t1_rdn7 = 0.0;
        var_t1_rdn8 = 0.0;
        var_t1_rdb0 = 0.0;
        var_t1_rdb1 = 0.0;
        var_t1_rdb2 = 0.0;
        var_t1_rdb3 = 0.0;
        var_t1_rdb4 = 0.0;

        let (assign12870_e11957, assign12870_e11957_d_n0, assign12870_e11957_d_n1, assign12870_e11957_d_n2, assign12870_e11957_d_n3, assign12870_e11957_d_n4, assign12870_e11957_d_n5, assign12870_e11957_d_n6, assign12870_e11957_d_n7, assign12870_e11957_d_n8, assign12870_e11957_d_b0, assign12870_e11957_d_b1, assign12870_e11957_d_b2, assign12870_e11957_d_b3, assign12870_e11957_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12870_e11953: f64 = (10000000000.0 * var_coxeff);
        let assign12870_e11955: f64 = (assign12870_e11953 * var_leffnoisq);
        (assign12870_e11955, (((10000000000.0 * var_coxeff_dn0) * var_leffnoisq) + (assign12870_e11953 * var_leffnoisq_dn0)), (((10000000000.0 * var_coxeff_dn1) * var_leffnoisq) + (assign12870_e11953 * var_leffnoisq_dn1)), (((10000000000.0 * var_coxeff_dn2) * var_leffnoisq) + (assign12870_e11953 * var_leffnoisq_dn2)), (((10000000000.0 * var_coxeff_dn3) * var_leffnoisq) + (assign12870_e11953 * var_leffnoisq_dn3)), (((10000000000.0 * var_coxeff_dn4) * var_leffnoisq) + (assign12870_e11953 * var_leffnoisq_dn4)), (((10000000000.0 * var_coxeff_dn5) * var_leffnoisq) + (assign12870_e11953 * var_leffnoisq_dn5)), (((10000000000.0 * var_coxeff_dn6) * var_leffnoisq) + (assign12870_e11953 * var_leffnoisq_dn6)), (((10000000000.0 * var_coxeff_dn7) * var_leffnoisq) + (assign12870_e11953 * var_leffnoisq_dn7)), (((10000000000.0 * var_coxeff_dn8) * var_leffnoisq) + (assign12870_e11953 * var_leffnoisq_dn8)), (((10000000000.0 * var_coxeff_db0) * var_leffnoisq) + (assign12870_e11953 * var_leffnoisq_db0)), (((10000000000.0 * var_coxeff_db1) * var_leffnoisq) + (assign12870_e11953 * var_leffnoisq_db1)), (((10000000000.0 * var_coxeff_db2) * var_leffnoisq) + (assign12870_e11953 * var_leffnoisq_db2)), (((10000000000.0 * var_coxeff_db3) * var_leffnoisq) + (assign12870_e11953 * var_leffnoisq_db3)), (((10000000000.0 * var_coxeff_db4) * var_leffnoisq) + (assign12870_e11953 * var_leffnoisq_db4)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_db0, var_t2_db1, var_t2_db2, var_t2_db3, var_t2_db4,)
    }
};
        var_t2 = assign12870_e11957;
        var_t2_dn0 = assign12870_e11957_d_n0;
        var_t2_dn1 = assign12870_e11957_d_n1;
        var_t2_dn2 = assign12870_e11957_d_n2;
        var_t2_dn3 = assign12870_e11957_d_n3;
        var_t2_dn4 = assign12870_e11957_d_n4;
        var_t2_dn5 = assign12870_e11957_d_n5;
        var_t2_dn6 = assign12870_e11957_d_n6;
        var_t2_dn7 = assign12870_e11957_d_n7;
        var_t2_dn8 = assign12870_e11957_d_n8;
        var_t2_db0 = assign12870_e11957_d_b0;
        var_t2_db1 = assign12870_e11957_d_b1;
        var_t2_db2 = assign12870_e11957_d_b2;
        var_t2_db3 = assign12870_e11957_d_b3;
        var_t2_db4 = assign12870_e11957_d_b4;
        var_t2_rv = 0.0;
        var_t2_rdn0 = 0.0;
        var_t2_rdn1 = 0.0;
        var_t2_rdn2 = 0.0;
        var_t2_rdn3 = 0.0;
        var_t2_rdn4 = 0.0;
        var_t2_rdn5 = 0.0;
        var_t2_rdn6 = 0.0;
        var_t2_rdn7 = 0.0;
        var_t2_rdn8 = 0.0;
        var_t2_rdb0 = 0.0;
        var_t2_rdb1 = 0.0;
        var_t2_rdb2 = 0.0;
        var_t2_rdb3 = 0.0;
        var_t2_rdb4 = 0.0;

        let (assign12880_e11965, assign12880_e11965_d_n0, assign12880_e11965_d_n1, assign12880_e11965_d_n2, assign12880_e11965_d_n3, assign12880_e11965_d_n4, assign12880_e11965_d_n5, assign12880_e11965_d_n6, assign12880_e11965_d_n7, assign12880_e11965_d_n8, assign12880_e11965_d_b0, assign12880_e11965_d_b1, assign12880_e11965_d_b2, assign12880_e11965_d_b3, assign12880_e11965_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12880_e11961: f64 = (var_coxeff * var_qis);
        let assign12880_e11963: f64 = (assign12880_e11961 / 1.60219e-19);
        (assign12880_e11963, (((var_coxeff_dn0 * var_qis) + (var_coxeff * var_qis_dn0)) / 1.60219e-19), (((var_coxeff_dn1 * var_qis) + (var_coxeff * var_qis_dn1)) / 1.60219e-19), (((var_coxeff_dn2 * var_qis) + (var_coxeff * var_qis_dn2)) / 1.60219e-19), (((var_coxeff_dn3 * var_qis) + (var_coxeff * var_qis_dn3)) / 1.60219e-19), (((var_coxeff_dn4 * var_qis) + (var_coxeff * var_qis_dn4)) / 1.60219e-19), (((var_coxeff_dn5 * var_qis) + (var_coxeff * var_qis_dn5)) / 1.60219e-19), (((var_coxeff_dn6 * var_qis) + (var_coxeff * var_qis_dn6)) / 1.60219e-19), (((var_coxeff_dn7 * var_qis) + (var_coxeff * var_qis_dn7)) / 1.60219e-19), (((var_coxeff_dn8 * var_qis) + (var_coxeff * var_qis_dn8)) / 1.60219e-19), (((var_coxeff_db0 * var_qis) + (var_coxeff * var_qis_db0)) / 1.60219e-19), (((var_coxeff_db1 * var_qis) + (var_coxeff * var_qis_db1)) / 1.60219e-19), (((var_coxeff_db2 * var_qis) + (var_coxeff * var_qis_db2)) / 1.60219e-19), (((var_coxeff_db3 * var_qis) + (var_coxeff * var_qis_db3)) / 1.60219e-19), (((var_coxeff_db4 * var_qis) + (var_coxeff * var_qis_db4)) / 1.60219e-19),)
    } else {
        (var_n0, var_n0_dn0, var_n0_dn1, var_n0_dn2, var_n0_dn3, var_n0_dn4, var_n0_dn5, var_n0_dn6, var_n0_dn7, var_n0_dn8, var_n0_db0, var_n0_db1, var_n0_db2, var_n0_db3, var_n0_db4,)
    }
};
        var_n0 = assign12880_e11965;
        var_n0_dn0 = assign12880_e11965_d_n0;
        var_n0_dn1 = assign12880_e11965_d_n1;
        var_n0_dn2 = assign12880_e11965_d_n2;
        var_n0_dn3 = assign12880_e11965_d_n3;
        var_n0_dn4 = assign12880_e11965_d_n4;
        var_n0_dn5 = assign12880_e11965_d_n5;
        var_n0_dn6 = assign12880_e11965_d_n6;
        var_n0_dn7 = assign12880_e11965_d_n7;
        var_n0_dn8 = assign12880_e11965_d_n8;
        var_n0_db0 = assign12880_e11965_d_b0;
        var_n0_db1 = assign12880_e11965_d_b1;
        var_n0_db2 = assign12880_e11965_d_b2;
        var_n0_db3 = assign12880_e11965_d_b3;
        var_n0_db4 = assign12880_e11965_d_b4;
        var_n0_rv = 0.0;
        var_n0_rdn0 = 0.0;
        var_n0_rdn1 = 0.0;
        var_n0_rdn2 = 0.0;
        var_n0_rdn3 = 0.0;
        var_n0_rdn4 = 0.0;
        var_n0_rdn5 = 0.0;
        var_n0_rdn6 = 0.0;
        var_n0_rdn7 = 0.0;
        var_n0_rdn8 = 0.0;
        var_n0_rdb0 = 0.0;
        var_n0_rdb1 = 0.0;
        var_n0_rdb2 = 0.0;
        var_n0_rdb3 = 0.0;
        var_n0_rdb4 = 0.0;

        let (assign12890_e11973, assign12890_e11973_d_n0, assign12890_e11973_d_n1, assign12890_e11973_d_n2, assign12890_e11973_d_n3, assign12890_e11973_d_n4, assign12890_e11973_d_n5, assign12890_e11973_d_n6, assign12890_e11973_d_n7, assign12890_e11973_d_n8, assign12890_e11973_d_b0, assign12890_e11973_d_b1, assign12890_e11973_d_b2, assign12890_e11973_d_b3, assign12890_e11973_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12890_e11969: f64 = (var_coxeff * var_qid);
        let assign12890_e11971: f64 = (assign12890_e11969 / 1.60219e-19);
        (assign12890_e11971, (((var_coxeff_dn0 * var_qid) + (var_coxeff * var_qid_dn0)) / 1.60219e-19), (((var_coxeff_dn1 * var_qid) + (var_coxeff * var_qid_dn1)) / 1.60219e-19), (((var_coxeff_dn2 * var_qid) + (var_coxeff * var_qid_dn2)) / 1.60219e-19), (((var_coxeff_dn3 * var_qid) + (var_coxeff * var_qid_dn3)) / 1.60219e-19), (((var_coxeff_dn4 * var_qid) + (var_coxeff * var_qid_dn4)) / 1.60219e-19), (((var_coxeff_dn5 * var_qid) + (var_coxeff * var_qid_dn5)) / 1.60219e-19), (((var_coxeff_dn6 * var_qid) + (var_coxeff * var_qid_dn6)) / 1.60219e-19), (((var_coxeff_dn7 * var_qid) + (var_coxeff * var_qid_dn7)) / 1.60219e-19), (((var_coxeff_dn8 * var_qid) + (var_coxeff * var_qid_dn8)) / 1.60219e-19), (((var_coxeff_db0 * var_qid) + (var_coxeff * var_qid_db0)) / 1.60219e-19), (((var_coxeff_db1 * var_qid) + (var_coxeff * var_qid_db1)) / 1.60219e-19), (((var_coxeff_db2 * var_qid) + (var_coxeff * var_qid_db2)) / 1.60219e-19), (((var_coxeff_db3 * var_qid) + (var_coxeff * var_qid_db3)) / 1.60219e-19), (((var_coxeff_db4 * var_qid) + (var_coxeff * var_qid_db4)) / 1.60219e-19),)
    } else {
        (var_nl, var_nl_dn0, var_nl_dn1, var_nl_dn2, var_nl_dn3, var_nl_dn4, var_nl_dn5, var_nl_dn6, var_nl_dn7, var_nl_dn8, var_nl_db0, var_nl_db1, var_nl_db2, var_nl_db3, var_nl_db4,)
    }
};
        var_nl = assign12890_e11973;
        var_nl_dn0 = assign12890_e11973_d_n0;
        var_nl_dn1 = assign12890_e11973_d_n1;
        var_nl_dn2 = assign12890_e11973_d_n2;
        var_nl_dn3 = assign12890_e11973_d_n3;
        var_nl_dn4 = assign12890_e11973_d_n4;
        var_nl_dn5 = assign12890_e11973_d_n5;
        var_nl_dn6 = assign12890_e11973_d_n6;
        var_nl_dn7 = assign12890_e11973_d_n7;
        var_nl_dn8 = assign12890_e11973_d_n8;
        var_nl_db0 = assign12890_e11973_d_b0;
        var_nl_db1 = assign12890_e11973_d_b1;
        var_nl_db2 = assign12890_e11973_d_b2;
        var_nl_db3 = assign12890_e11973_d_b3;
        var_nl_db4 = assign12890_e11973_d_b4;
        var_nl_rv = 0.0;
        var_nl_rdn0 = 0.0;
        var_nl_rdn1 = 0.0;
        var_nl_rdn2 = 0.0;
        var_nl_rdn3 = 0.0;
        var_nl_rdn4 = 0.0;
        var_nl_rdn5 = 0.0;
        var_nl_rdn6 = 0.0;
        var_nl_rdn7 = 0.0;
        var_nl_rdn8 = 0.0;
        var_nl_rdb0 = 0.0;
        var_nl_rdb1 = 0.0;
        var_nl_rdb2 = 0.0;
        var_nl_rdb3 = 0.0;
        var_nl_rdb4 = 0.0;

        let (assign12900_e11983, assign12900_e11983_d_n0, assign12900_e11983_d_n1, assign12900_e11983_d_n2, assign12900_e11983_d_n3, assign12900_e11983_d_n4, assign12900_e11983_d_n5, assign12900_e11983_d_n6, assign12900_e11983_d_n7, assign12900_e11983_d_n8, assign12900_e11983_d_b0, assign12900_e11983_d_b1, assign12900_e11983_d_b2, assign12900_e11983_d_b3, assign12900_e11983_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12900_e11977: f64 = (var_vtm / 1.60219e-19);
        let assign12900_e11980: f64 = (var_coxeff + var_cit_i);
        let assign12900_e11981: f64 = (assign12900_e11977 * assign12900_e11980);
        (assign12900_e11981, (((var_vtm_dn0 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * (var_coxeff_dn0 + var_cit_i_dn0))), (((var_vtm_dn1 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * (var_coxeff_dn1 + var_cit_i_dn1))), (((var_vtm_dn2 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * (var_coxeff_dn2 + var_cit_i_dn2))), (((var_vtm_dn3 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * (var_coxeff_dn3 + var_cit_i_dn3))), (((var_vtm_dn4 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * (var_coxeff_dn4 + var_cit_i_dn4))), (((var_vtm_dn5 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * (var_coxeff_dn5 + var_cit_i_dn5))), (((var_vtm_dn6 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * (var_coxeff_dn6 + var_cit_i_dn6))), (((var_vtm_dn7 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * (var_coxeff_dn7 + var_cit_i_dn7))), (((var_vtm_dn8 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * (var_coxeff_dn8 + var_cit_i_dn8))), (((var_vtm_db0 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * (var_coxeff_db0 + var_cit_i_db0))), (((var_vtm_db1 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * (var_coxeff_db1 + var_cit_i_db1))), (((var_vtm_db2 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * (var_coxeff_db2 + var_cit_i_db2))), (((var_vtm_db3 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * (var_coxeff_db3 + var_cit_i_db3))), (((var_vtm_db4 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * (var_coxeff_db4 + var_cit_i_db4))),)
    } else {
        (var_nstar, var_nstar_dn0, var_nstar_dn1, var_nstar_dn2, var_nstar_dn3, var_nstar_dn4, var_nstar_dn5, var_nstar_dn6, var_nstar_dn7, var_nstar_dn8, var_nstar_db0, var_nstar_db1, var_nstar_db2, var_nstar_db3, var_nstar_db4,)
    }
};
        var_nstar = assign12900_e11983;
        var_nstar_dn0 = assign12900_e11983_d_n0;
        var_nstar_dn1 = assign12900_e11983_d_n1;
        var_nstar_dn2 = assign12900_e11983_d_n2;
        var_nstar_dn3 = assign12900_e11983_d_n3;
        var_nstar_dn4 = assign12900_e11983_d_n4;
        var_nstar_dn5 = assign12900_e11983_d_n5;
        var_nstar_dn6 = assign12900_e11983_d_n6;
        var_nstar_dn7 = assign12900_e11983_d_n7;
        var_nstar_dn8 = assign12900_e11983_d_n8;
        var_nstar_db0 = assign12900_e11983_d_b0;
        var_nstar_db1 = assign12900_e11983_d_b1;
        var_nstar_db2 = assign12900_e11983_d_b2;
        var_nstar_db3 = assign12900_e11983_d_b3;
        var_nstar_db4 = assign12900_e11983_d_b4;
        var_nstar_rv = 0.0;
        var_nstar_rdn0 = 0.0;
        var_nstar_rdn1 = 0.0;
        var_nstar_rdn2 = 0.0;
        var_nstar_rdn3 = 0.0;
        var_nstar_rdn4 = 0.0;
        var_nstar_rdn5 = 0.0;
        var_nstar_rdn6 = 0.0;
        var_nstar_rdn7 = 0.0;
        var_nstar_rdn8 = 0.0;
        var_nstar_rdb0 = 0.0;
        var_nstar_rdb1 = 0.0;
        var_nstar_rdb2 = 0.0;
        var_nstar_rdb3 = 0.0;
        var_nstar_rdb4 = 0.0;

        let (assign12910_e11998, assign12910_e11998_d_n0, assign12910_e11998_d_n1, assign12910_e11998_d_n2, assign12910_e11998_d_n3, assign12910_e11998_d_n4, assign12910_e11998_d_n5, assign12910_e11998_d_n6, assign12910_e11998_d_n7, assign12910_e11998_d_n8, assign12910_e11998_d_b0, assign12910_e11998_d_b1, assign12910_e11998_d_b2, assign12910_e11998_d_b3, assign12910_e11998_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12910_e11988: f64 = (var_n0 + var_nstar);
        let assign12910_e11991: f64 = (var_nl + var_nstar);
        let assign12910_e11992: f64 = (assign12910_e11988 / assign12910_e11991);
        let assign12910_e11994: f64 = (assign12910_e11992).max(1e-38);
        let assign12910_e11995: f64 = (assign12910_e11994).ln();
        let assign12910_e11996: f64 = (var_noiaeff * assign12910_e11995);
        (assign12910_e11996, ((var_noiaeff_dn0 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn0 + var_nstar_dn0) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn0 + var_nstar_dn0))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_dn1 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn1 + var_nstar_dn1) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn1 + var_nstar_dn1))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_dn2 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn2 + var_nstar_dn2) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn2 + var_nstar_dn2))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_dn3 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn3 + var_nstar_dn3) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn3 + var_nstar_dn3))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_dn4 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn4 + var_nstar_dn4) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn4 + var_nstar_dn4))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_dn5 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn5 + var_nstar_dn5) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn5 + var_nstar_dn5))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_dn6 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn6 + var_nstar_dn6) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn6 + var_nstar_dn6))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_dn7 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn7 + var_nstar_dn7) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn7 + var_nstar_dn7))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_dn8 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn8 + var_nstar_dn8) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn8 + var_nstar_dn8))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_db0 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_db0 + var_nstar_db0) * assign12910_e11991) - (assign12910_e11988 * (var_nl_db0 + var_nstar_db0))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_db1 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_db1 + var_nstar_db1) * assign12910_e11991) - (assign12910_e11988 * (var_nl_db1 + var_nstar_db1))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_db2 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_db2 + var_nstar_db2) * assign12910_e11991) - (assign12910_e11988 * (var_nl_db2 + var_nstar_db2))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_db3 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_db3 + var_nstar_db3) * assign12910_e11991) - (assign12910_e11988 * (var_nl_db3 + var_nstar_db3))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_db4 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_db4 + var_nstar_db4) * assign12910_e11991) - (assign12910_e11988 * (var_nl_db4 + var_nstar_db4))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn1, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_db0, var_t3_db1, var_t3_db2, var_t3_db3, var_t3_db4,)
    }
};
        var_t3 = assign12910_e11998;
        var_t3_dn0 = assign12910_e11998_d_n0;
        var_t3_dn1 = assign12910_e11998_d_n1;
        var_t3_dn2 = assign12910_e11998_d_n2;
        var_t3_dn3 = assign12910_e11998_d_n3;
        var_t3_dn4 = assign12910_e11998_d_n4;
        var_t3_dn5 = assign12910_e11998_d_n5;
        var_t3_dn6 = assign12910_e11998_d_n6;
        var_t3_dn7 = assign12910_e11998_d_n7;
        var_t3_dn8 = assign12910_e11998_d_n8;
        var_t3_db0 = assign12910_e11998_d_b0;
        var_t3_db1 = assign12910_e11998_d_b1;
        var_t3_db2 = assign12910_e11998_d_b2;
        var_t3_db3 = assign12910_e11998_d_b3;
        var_t3_db4 = assign12910_e11998_d_b4;
        var_t3_rv = 0.0;
        var_t3_rdn0 = 0.0;
        var_t3_rdn1 = 0.0;
        var_t3_rdn2 = 0.0;
        var_t3_rdn3 = 0.0;
        var_t3_rdn4 = 0.0;
        var_t3_rdn5 = 0.0;
        var_t3_rdn6 = 0.0;
        var_t3_rdn7 = 0.0;
        var_t3_rdn8 = 0.0;
        var_t3_rdb0 = 0.0;
        var_t3_rdb1 = 0.0;
        var_t3_rdb2 = 0.0;
        var_t3_rdb3 = 0.0;
        var_t3_rdb4 = 0.0;

        let (assign12920_e12006, assign12920_e12006_d_n0, assign12920_e12006_d_n1, assign12920_e12006_d_n2, assign12920_e12006_d_n3, assign12920_e12006_d_n4, assign12920_e12006_d_n5, assign12920_e12006_d_n6, assign12920_e12006_d_n7, assign12920_e12006_d_n8, assign12920_e12006_d_b0, assign12920_e12006_d_b1, assign12920_e12006_d_b2, assign12920_e12006_d_b3, assign12920_e12006_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12920_e12003: f64 = (var_n0 - var_nl);
        let assign12920_e12004: f64 = (p.p289 * assign12920_e12003);
        (assign12920_e12004, (p.p289 * (var_n0_dn0 - var_nl_dn0)), (p.p289 * (var_n0_dn1 - var_nl_dn1)), (p.p289 * (var_n0_dn2 - var_nl_dn2)), (p.p289 * (var_n0_dn3 - var_nl_dn3)), (p.p289 * (var_n0_dn4 - var_nl_dn4)), (p.p289 * (var_n0_dn5 - var_nl_dn5)), (p.p289 * (var_n0_dn6 - var_nl_dn6)), (p.p289 * (var_n0_dn7 - var_nl_dn7)), (p.p289 * (var_n0_dn8 - var_nl_dn8)), (p.p289 * (var_n0_db0 - var_nl_db0)), (p.p289 * (var_n0_db1 - var_nl_db1)), (p.p289 * (var_n0_db2 - var_nl_db2)), (p.p289 * (var_n0_db3 - var_nl_db3)), (p.p289 * (var_n0_db4 - var_nl_db4)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn1, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_db0, var_t4_db1, var_t4_db2, var_t4_db3, var_t4_db4,)
    }
};
        var_t4 = assign12920_e12006;
        var_t4_dn0 = assign12920_e12006_d_n0;
        var_t4_dn1 = assign12920_e12006_d_n1;
        var_t4_dn2 = assign12920_e12006_d_n2;
        var_t4_dn3 = assign12920_e12006_d_n3;
        var_t4_dn4 = assign12920_e12006_d_n4;
        var_t4_dn5 = assign12920_e12006_d_n5;
        var_t4_dn6 = assign12920_e12006_d_n6;
        var_t4_dn7 = assign12920_e12006_d_n7;
        var_t4_dn8 = assign12920_e12006_d_n8;
        var_t4_db0 = assign12920_e12006_d_b0;
        var_t4_db1 = assign12920_e12006_d_b1;
        var_t4_db2 = assign12920_e12006_d_b2;
        var_t4_db3 = assign12920_e12006_d_b3;
        var_t4_db4 = assign12920_e12006_d_b4;
        var_t4_rv = 0.0;
        var_t4_rdn0 = 0.0;
        var_t4_rdn1 = 0.0;
        var_t4_rdn2 = 0.0;
        var_t4_rdn3 = 0.0;
        var_t4_rdn4 = 0.0;
        var_t4_rdn5 = 0.0;
        var_t4_rdn6 = 0.0;
        var_t4_rdn7 = 0.0;
        var_t4_rdn8 = 0.0;
        var_t4_rdb0 = 0.0;
        var_t4_rdb1 = 0.0;
        var_t4_rdb2 = 0.0;
        var_t4_rdb3 = 0.0;
        var_t4_rdb4 = 0.0;

        let (assign12930_e12020, assign12930_e12020_d_n0, assign12930_e12020_d_n1, assign12930_e12020_d_n2, assign12930_e12020_d_n3, assign12930_e12020_d_n4, assign12930_e12020_d_n5, assign12930_e12020_d_n6, assign12930_e12020_d_n7, assign12930_e12020_d_n8, assign12930_e12020_d_b0, assign12930_e12020_d_b1, assign12930_e12020_d_b2, assign12930_e12020_d_b3, assign12930_e12020_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12930_e12010: f64 = (0.5 * p.p290);
        let assign12930_e12013: f64 = (var_n0 * var_n0);
        let assign12930_e12016: f64 = (var_nl * var_nl);
        let assign12930_e12017: f64 = (assign12930_e12013 - assign12930_e12016);
        let assign12930_e12018: f64 = (assign12930_e12010 * assign12930_e12017);
        (assign12930_e12018, (assign12930_e12010 * (((var_n0_dn0 * var_n0) + (var_n0 * var_n0_dn0)) - ((var_nl_dn0 * var_nl) + (var_nl * var_nl_dn0)))), (assign12930_e12010 * (((var_n0_dn1 * var_n0) + (var_n0 * var_n0_dn1)) - ((var_nl_dn1 * var_nl) + (var_nl * var_nl_dn1)))), (assign12930_e12010 * (((var_n0_dn2 * var_n0) + (var_n0 * var_n0_dn2)) - ((var_nl_dn2 * var_nl) + (var_nl * var_nl_dn2)))), (assign12930_e12010 * (((var_n0_dn3 * var_n0) + (var_n0 * var_n0_dn3)) - ((var_nl_dn3 * var_nl) + (var_nl * var_nl_dn3)))), (assign12930_e12010 * (((var_n0_dn4 * var_n0) + (var_n0 * var_n0_dn4)) - ((var_nl_dn4 * var_nl) + (var_nl * var_nl_dn4)))), (assign12930_e12010 * (((var_n0_dn5 * var_n0) + (var_n0 * var_n0_dn5)) - ((var_nl_dn5 * var_nl) + (var_nl * var_nl_dn5)))), (assign12930_e12010 * (((var_n0_dn6 * var_n0) + (var_n0 * var_n0_dn6)) - ((var_nl_dn6 * var_nl) + (var_nl * var_nl_dn6)))), (assign12930_e12010 * (((var_n0_dn7 * var_n0) + (var_n0 * var_n0_dn7)) - ((var_nl_dn7 * var_nl) + (var_nl * var_nl_dn7)))), (assign12930_e12010 * (((var_n0_dn8 * var_n0) + (var_n0 * var_n0_dn8)) - ((var_nl_dn8 * var_nl) + (var_nl * var_nl_dn8)))), (assign12930_e12010 * (((var_n0_db0 * var_n0) + (var_n0 * var_n0_db0)) - ((var_nl_db0 * var_nl) + (var_nl * var_nl_db0)))), (assign12930_e12010 * (((var_n0_db1 * var_n0) + (var_n0 * var_n0_db1)) - ((var_nl_db1 * var_nl) + (var_nl * var_nl_db1)))), (assign12930_e12010 * (((var_n0_db2 * var_n0) + (var_n0 * var_n0_db2)) - ((var_nl_db2 * var_nl) + (var_nl * var_nl_db2)))), (assign12930_e12010 * (((var_n0_db3 * var_n0) + (var_n0 * var_n0_db3)) - ((var_nl_db3 * var_nl) + (var_nl * var_nl_db3)))), (assign12930_e12010 * (((var_n0_db4 * var_n0) + (var_n0 * var_n0_db4)) - ((var_nl_db4 * var_nl) + (var_nl * var_nl_db4)))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn1, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_db0, var_t5_db1, var_t5_db2, var_t5_db3, var_t5_db4,)
    }
};
        var_t5 = assign12930_e12020;
        var_t5_dn0 = assign12930_e12020_d_n0;
        var_t5_dn1 = assign12930_e12020_d_n1;
        var_t5_dn2 = assign12930_e12020_d_n2;
        var_t5_dn3 = assign12930_e12020_d_n3;
        var_t5_dn4 = assign12930_e12020_d_n4;
        var_t5_dn5 = assign12930_e12020_d_n5;
        var_t5_dn6 = assign12930_e12020_d_n6;
        var_t5_dn7 = assign12930_e12020_d_n7;
        var_t5_dn8 = assign12930_e12020_d_n8;
        var_t5_db0 = assign12930_e12020_d_b0;
        var_t5_db1 = assign12930_e12020_d_b1;
        var_t5_db2 = assign12930_e12020_d_b2;
        var_t5_db3 = assign12930_e12020_d_b3;
        var_t5_db4 = assign12930_e12020_d_b4;
        var_t5_rv = 0.0;
        var_t5_rdn0 = 0.0;
        var_t5_rdn1 = 0.0;
        var_t5_rdn2 = 0.0;
        var_t5_rdn3 = 0.0;
        var_t5_rdn4 = 0.0;
        var_t5_rdn5 = 0.0;
        var_t5_rdn6 = 0.0;
        var_t5_rdn7 = 0.0;
        var_t5_rdn8 = 0.0;
        var_t5_rdb0 = 0.0;
        var_t5_rdb1 = 0.0;
        var_t5_rdb2 = 0.0;
        var_t5_rdb3 = 0.0;
        var_t5_rdb4 = 0.0;

        let (assign12940_e12030, assign12940_e12030_d_n0, assign12940_e12030_d_n1, assign12940_e12030_d_n2, assign12940_e12030_d_n3, assign12940_e12030_d_n4, assign12940_e12030_d_n5, assign12940_e12030_d_n6, assign12940_e12030_d_n7, assign12940_e12030_d_n8, assign12940_e12030_d_b0, assign12940_e12030_d_b1, assign12940_e12030_d_b2, assign12940_e12030_d_b3, assign12940_e12030_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12940_e12024: f64 = (1.60219e-19 * var_vtm);
        let assign12940_e12026: f64 = (assign12940_e12024 * var_ids);
        let assign12940_e12028: f64 = (assign12940_e12026 * var_ids);
        (assign12940_e12028, (((((1.60219e-19 * var_vtm_dn0) * var_ids) + (assign12940_e12024 * var_ids_dn0)) * var_ids) + (assign12940_e12026 * var_ids_dn0)), (((((1.60219e-19 * var_vtm_dn1) * var_ids) + (assign12940_e12024 * var_ids_dn1)) * var_ids) + (assign12940_e12026 * var_ids_dn1)), (((((1.60219e-19 * var_vtm_dn2) * var_ids) + (assign12940_e12024 * var_ids_dn2)) * var_ids) + (assign12940_e12026 * var_ids_dn2)), (((((1.60219e-19 * var_vtm_dn3) * var_ids) + (assign12940_e12024 * var_ids_dn3)) * var_ids) + (assign12940_e12026 * var_ids_dn3)), (((((1.60219e-19 * var_vtm_dn4) * var_ids) + (assign12940_e12024 * var_ids_dn4)) * var_ids) + (assign12940_e12026 * var_ids_dn4)), (((((1.60219e-19 * var_vtm_dn5) * var_ids) + (assign12940_e12024 * var_ids_dn5)) * var_ids) + (assign12940_e12026 * var_ids_dn5)), (((((1.60219e-19 * var_vtm_dn6) * var_ids) + (assign12940_e12024 * var_ids_dn6)) * var_ids) + (assign12940_e12026 * var_ids_dn6)), (((((1.60219e-19 * var_vtm_dn7) * var_ids) + (assign12940_e12024 * var_ids_dn7)) * var_ids) + (assign12940_e12026 * var_ids_dn7)), (((((1.60219e-19 * var_vtm_dn8) * var_ids) + (assign12940_e12024 * var_ids_dn8)) * var_ids) + (assign12940_e12026 * var_ids_dn8)), (((((1.60219e-19 * var_vtm_db0) * var_ids) + (assign12940_e12024 * var_ids_db0)) * var_ids) + (assign12940_e12026 * var_ids_db0)), (((((1.60219e-19 * var_vtm_db1) * var_ids) + (assign12940_e12024 * var_ids_db1)) * var_ids) + (assign12940_e12026 * var_ids_db1)), (((((1.60219e-19 * var_vtm_db2) * var_ids) + (assign12940_e12024 * var_ids_db2)) * var_ids) + (assign12940_e12026 * var_ids_db2)), (((((1.60219e-19 * var_vtm_db3) * var_ids) + (assign12940_e12024 * var_ids_db3)) * var_ids) + (assign12940_e12026 * var_ids_db3)), (((((1.60219e-19 * var_vtm_db4) * var_ids) + (assign12940_e12024 * var_ids_db4)) * var_ids) + (assign12940_e12026 * var_ids_db4)),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn1, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_db0, var_t6_db1, var_t6_db2, var_t6_db3, var_t6_db4,)
    }
};
        var_t6 = assign12940_e12030;
        var_t6_dn0 = assign12940_e12030_d_n0;
        var_t6_dn1 = assign12940_e12030_d_n1;
        var_t6_dn2 = assign12940_e12030_d_n2;
        var_t6_dn3 = assign12940_e12030_d_n3;
        var_t6_dn4 = assign12940_e12030_d_n4;
        var_t6_dn5 = assign12940_e12030_d_n5;
        var_t6_dn6 = assign12940_e12030_d_n6;
        var_t6_dn7 = assign12940_e12030_d_n7;
        var_t6_dn8 = assign12940_e12030_d_n8;
        var_t6_db0 = assign12940_e12030_d_b0;
        var_t6_db1 = assign12940_e12030_d_b1;
        var_t6_db2 = assign12940_e12030_d_b2;
        var_t6_db3 = assign12940_e12030_d_b3;
        var_t6_db4 = assign12940_e12030_d_b4;
        var_t6_rv = 0.0;
        var_t6_rdn0 = 0.0;
        var_t6_rdn1 = 0.0;
        var_t6_rdn2 = 0.0;
        var_t6_rdn3 = 0.0;
        var_t6_rdn4 = 0.0;
        var_t6_rdn5 = 0.0;
        var_t6_rdn6 = 0.0;
        var_t6_rdn7 = 0.0;
        var_t6_rdn8 = 0.0;
        var_t6_rdb0 = 0.0;
        var_t6_rdb1 = 0.0;
        var_t6_rdb2 = 0.0;
        var_t6_rdb3 = 0.0;
        var_t6_rdb4 = 0.0;


        *var_n0_slot = var_n0;
        *var_n0_db0_slot = var_n0_db0;
        *var_n0_db1_slot = var_n0_db1;
        *var_n0_db2_slot = var_n0_db2;
        *var_n0_db3_slot = var_n0_db3;
        *var_n0_db4_slot = var_n0_db4;
        *var_n0_dn0_slot = var_n0_dn0;
        *var_n0_dn1_slot = var_n0_dn1;
        *var_n0_dn2_slot = var_n0_dn2;
        *var_n0_dn3_slot = var_n0_dn3;
        *var_n0_dn4_slot = var_n0_dn4;
        *var_n0_dn5_slot = var_n0_dn5;
        *var_n0_dn6_slot = var_n0_dn6;
        *var_n0_dn7_slot = var_n0_dn7;
        *var_n0_dn8_slot = var_n0_dn8;
        *var_n0_rdb0_slot = var_n0_rdb0;
        *var_n0_rdb1_slot = var_n0_rdb1;
        *var_n0_rdb2_slot = var_n0_rdb2;
        *var_n0_rdb3_slot = var_n0_rdb3;
        *var_n0_rdb4_slot = var_n0_rdb4;
        *var_n0_rdn0_slot = var_n0_rdn0;
        *var_n0_rdn1_slot = var_n0_rdn1;
        *var_n0_rdn2_slot = var_n0_rdn2;
        *var_n0_rdn3_slot = var_n0_rdn3;
        *var_n0_rdn4_slot = var_n0_rdn4;
        *var_n0_rdn5_slot = var_n0_rdn5;
        *var_n0_rdn6_slot = var_n0_rdn6;
        *var_n0_rdn7_slot = var_n0_rdn7;
        *var_n0_rdn8_slot = var_n0_rdn8;
        *var_n0_rv_slot = var_n0_rv;
        *var_nl_slot = var_nl;
        *var_nl_db0_slot = var_nl_db0;
        *var_nl_db1_slot = var_nl_db1;
        *var_nl_db2_slot = var_nl_db2;
        *var_nl_db3_slot = var_nl_db3;
        *var_nl_db4_slot = var_nl_db4;
        *var_nl_dn0_slot = var_nl_dn0;
        *var_nl_dn1_slot = var_nl_dn1;
        *var_nl_dn2_slot = var_nl_dn2;
        *var_nl_dn3_slot = var_nl_dn3;
        *var_nl_dn4_slot = var_nl_dn4;
        *var_nl_dn5_slot = var_nl_dn5;
        *var_nl_dn6_slot = var_nl_dn6;
        *var_nl_dn7_slot = var_nl_dn7;
        *var_nl_dn8_slot = var_nl_dn8;
        *var_nl_rdb0_slot = var_nl_rdb0;
        *var_nl_rdb1_slot = var_nl_rdb1;
        *var_nl_rdb2_slot = var_nl_rdb2;
        *var_nl_rdb3_slot = var_nl_rdb3;
        *var_nl_rdb4_slot = var_nl_rdb4;
        *var_nl_rdn0_slot = var_nl_rdn0;
        *var_nl_rdn1_slot = var_nl_rdn1;
        *var_nl_rdn2_slot = var_nl_rdn2;
        *var_nl_rdn3_slot = var_nl_rdn3;
        *var_nl_rdn4_slot = var_nl_rdn4;
        *var_nl_rdn5_slot = var_nl_rdn5;
        *var_nl_rdn6_slot = var_nl_rdn6;
        *var_nl_rdn7_slot = var_nl_rdn7;
        *var_nl_rdn8_slot = var_nl_rdn8;
        *var_nl_rv_slot = var_nl_rv;
        *var_noiaeff_slot = var_noiaeff;
        *var_noiaeff_db0_slot = var_noiaeff_db0;
        *var_noiaeff_db1_slot = var_noiaeff_db1;
        *var_noiaeff_db2_slot = var_noiaeff_db2;
        *var_noiaeff_db3_slot = var_noiaeff_db3;
        *var_noiaeff_db4_slot = var_noiaeff_db4;
        *var_noiaeff_dn0_slot = var_noiaeff_dn0;
        *var_noiaeff_dn1_slot = var_noiaeff_dn1;
        *var_noiaeff_dn2_slot = var_noiaeff_dn2;
        *var_noiaeff_dn3_slot = var_noiaeff_dn3;
        *var_noiaeff_dn4_slot = var_noiaeff_dn4;
        *var_noiaeff_dn5_slot = var_noiaeff_dn5;
        *var_noiaeff_dn6_slot = var_noiaeff_dn6;
        *var_noiaeff_dn7_slot = var_noiaeff_dn7;
        *var_noiaeff_dn8_slot = var_noiaeff_dn8;
        *var_noiaeff_rdb0_slot = var_noiaeff_rdb0;
        *var_noiaeff_rdb1_slot = var_noiaeff_rdb1;
        *var_noiaeff_rdb2_slot = var_noiaeff_rdb2;
        *var_noiaeff_rdb3_slot = var_noiaeff_rdb3;
        *var_noiaeff_rdb4_slot = var_noiaeff_rdb4;
        *var_noiaeff_rdn0_slot = var_noiaeff_rdn0;
        *var_noiaeff_rdn1_slot = var_noiaeff_rdn1;
        *var_noiaeff_rdn2_slot = var_noiaeff_rdn2;
        *var_noiaeff_rdn3_slot = var_noiaeff_rdn3;
        *var_noiaeff_rdn4_slot = var_noiaeff_rdn4;
        *var_noiaeff_rdn5_slot = var_noiaeff_rdn5;
        *var_noiaeff_rdn6_slot = var_noiaeff_rdn6;
        *var_noiaeff_rdn7_slot = var_noiaeff_rdn7;
        *var_noiaeff_rdn8_slot = var_noiaeff_rdn8;
        *var_noiaeff_rv_slot = var_noiaeff_rv;
        *var_nstar_slot = var_nstar;
        *var_nstar_db0_slot = var_nstar_db0;
        *var_nstar_db1_slot = var_nstar_db1;
        *var_nstar_db2_slot = var_nstar_db2;
        *var_nstar_db3_slot = var_nstar_db3;
        *var_nstar_db4_slot = var_nstar_db4;
        *var_nstar_dn0_slot = var_nstar_dn0;
        *var_nstar_dn1_slot = var_nstar_dn1;
        *var_nstar_dn2_slot = var_nstar_dn2;
        *var_nstar_dn3_slot = var_nstar_dn3;
        *var_nstar_dn4_slot = var_nstar_dn4;
        *var_nstar_dn5_slot = var_nstar_dn5;
        *var_nstar_dn6_slot = var_nstar_dn6;
        *var_nstar_dn7_slot = var_nstar_dn7;
        *var_nstar_dn8_slot = var_nstar_dn8;
        *var_nstar_rdb0_slot = var_nstar_rdb0;
        *var_nstar_rdb1_slot = var_nstar_rdb1;
        *var_nstar_rdb2_slot = var_nstar_rdb2;
        *var_nstar_rdb3_slot = var_nstar_rdb3;
        *var_nstar_rdb4_slot = var_nstar_rdb4;
        *var_nstar_rdn0_slot = var_nstar_rdn0;
        *var_nstar_rdn1_slot = var_nstar_rdn1;
        *var_nstar_rdn2_slot = var_nstar_rdn2;
        *var_nstar_rdn3_slot = var_nstar_rdn3;
        *var_nstar_rdn4_slot = var_nstar_rdn4;
        *var_nstar_rdn5_slot = var_nstar_rdn5;
        *var_nstar_rdn6_slot = var_nstar_rdn6;
        *var_nstar_rdn7_slot = var_nstar_rdn7;
        *var_nstar_rdn8_slot = var_nstar_rdn8;
        *var_nstar_rv_slot = var_nstar_rv;
        *var_t1_slot = var_t1;
        *var_t1_db0_slot = var_t1_db0;
        *var_t1_db1_slot = var_t1_db1;
        *var_t1_db2_slot = var_t1_db2;
        *var_t1_db3_slot = var_t1_db3;
        *var_t1_db4_slot = var_t1_db4;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rdb0_slot = var_t1_rdb0;
        *var_t1_rdb1_slot = var_t1_rdb1;
        *var_t1_rdb2_slot = var_t1_rdb2;
        *var_t1_rdb3_slot = var_t1_rdb3;
        *var_t1_rdb4_slot = var_t1_rdb4;
        *var_t1_rdn0_slot = var_t1_rdn0;
        *var_t1_rdn1_slot = var_t1_rdn1;
        *var_t1_rdn2_slot = var_t1_rdn2;
        *var_t1_rdn3_slot = var_t1_rdn3;
        *var_t1_rdn4_slot = var_t1_rdn4;
        *var_t1_rdn5_slot = var_t1_rdn5;
        *var_t1_rdn6_slot = var_t1_rdn6;
        *var_t1_rdn7_slot = var_t1_rdn7;
        *var_t1_rdn8_slot = var_t1_rdn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_db0_slot = var_t2_db0;
        *var_t2_db1_slot = var_t2_db1;
        *var_t2_db2_slot = var_t2_db2;
        *var_t2_db3_slot = var_t2_db3;
        *var_t2_db4_slot = var_t2_db4;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rdb0_slot = var_t2_rdb0;
        *var_t2_rdb1_slot = var_t2_rdb1;
        *var_t2_rdb2_slot = var_t2_rdb2;
        *var_t2_rdb3_slot = var_t2_rdb3;
        *var_t2_rdb4_slot = var_t2_rdb4;
        *var_t2_rdn0_slot = var_t2_rdn0;
        *var_t2_rdn1_slot = var_t2_rdn1;
        *var_t2_rdn2_slot = var_t2_rdn2;
        *var_t2_rdn3_slot = var_t2_rdn3;
        *var_t2_rdn4_slot = var_t2_rdn4;
        *var_t2_rdn5_slot = var_t2_rdn5;
        *var_t2_rdn6_slot = var_t2_rdn6;
        *var_t2_rdn7_slot = var_t2_rdn7;
        *var_t2_rdn8_slot = var_t2_rdn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_db0_slot = var_t3_db0;
        *var_t3_db1_slot = var_t3_db1;
        *var_t3_db2_slot = var_t3_db2;
        *var_t3_db3_slot = var_t3_db3;
        *var_t3_db4_slot = var_t3_db4;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn1_slot = var_t3_dn1;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rdb0_slot = var_t3_rdb0;
        *var_t3_rdb1_slot = var_t3_rdb1;
        *var_t3_rdb2_slot = var_t3_rdb2;
        *var_t3_rdb3_slot = var_t3_rdb3;
        *var_t3_rdb4_slot = var_t3_rdb4;
        *var_t3_rdn0_slot = var_t3_rdn0;
        *var_t3_rdn1_slot = var_t3_rdn1;
        *var_t3_rdn2_slot = var_t3_rdn2;
        *var_t3_rdn3_slot = var_t3_rdn3;
        *var_t3_rdn4_slot = var_t3_rdn4;
        *var_t3_rdn5_slot = var_t3_rdn5;
        *var_t3_rdn6_slot = var_t3_rdn6;
        *var_t3_rdn7_slot = var_t3_rdn7;
        *var_t3_rdn8_slot = var_t3_rdn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_db0_slot = var_t4_db0;
        *var_t4_db1_slot = var_t4_db1;
        *var_t4_db2_slot = var_t4_db2;
        *var_t4_db3_slot = var_t4_db3;
        *var_t4_db4_slot = var_t4_db4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn1_slot = var_t4_dn1;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_rdb0_slot = var_t4_rdb0;
        *var_t4_rdb1_slot = var_t4_rdb1;
        *var_t4_rdb2_slot = var_t4_rdb2;
        *var_t4_rdb3_slot = var_t4_rdb3;
        *var_t4_rdb4_slot = var_t4_rdb4;
        *var_t4_rdn0_slot = var_t4_rdn0;
        *var_t4_rdn1_slot = var_t4_rdn1;
        *var_t4_rdn2_slot = var_t4_rdn2;
        *var_t4_rdn3_slot = var_t4_rdn3;
        *var_t4_rdn4_slot = var_t4_rdn4;
        *var_t4_rdn5_slot = var_t4_rdn5;
        *var_t4_rdn6_slot = var_t4_rdn6;
        *var_t4_rdn7_slot = var_t4_rdn7;
        *var_t4_rdn8_slot = var_t4_rdn8;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_db0_slot = var_t5_db0;
        *var_t5_db1_slot = var_t5_db1;
        *var_t5_db2_slot = var_t5_db2;
        *var_t5_db3_slot = var_t5_db3;
        *var_t5_db4_slot = var_t5_db4;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn1_slot = var_t5_dn1;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_rdb0_slot = var_t5_rdb0;
        *var_t5_rdb1_slot = var_t5_rdb1;
        *var_t5_rdb2_slot = var_t5_rdb2;
        *var_t5_rdb3_slot = var_t5_rdb3;
        *var_t5_rdb4_slot = var_t5_rdb4;
        *var_t5_rdn0_slot = var_t5_rdn0;
        *var_t5_rdn1_slot = var_t5_rdn1;
        *var_t5_rdn2_slot = var_t5_rdn2;
        *var_t5_rdn3_slot = var_t5_rdn3;
        *var_t5_rdn4_slot = var_t5_rdn4;
        *var_t5_rdn5_slot = var_t5_rdn5;
        *var_t5_rdn6_slot = var_t5_rdn6;
        *var_t5_rdn7_slot = var_t5_rdn7;
        *var_t5_rdn8_slot = var_t5_rdn8;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_db0_slot = var_t6_db0;
        *var_t6_db1_slot = var_t6_db1;
        *var_t6_db2_slot = var_t6_db2;
        *var_t6_db3_slot = var_t6_db3;
        *var_t6_db4_slot = var_t6_db4;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn1_slot = var_t6_dn1;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_rdb0_slot = var_t6_rdb0;
        *var_t6_rdb1_slot = var_t6_rdb1;
        *var_t6_rdb2_slot = var_t6_rdb2;
        *var_t6_rdb3_slot = var_t6_rdb3;
        *var_t6_rdb4_slot = var_t6_rdb4;
        *var_t6_rdn0_slot = var_t6_rdn0;
        *var_t6_rdn1_slot = var_t6_rdn1;
        *var_t6_rdn2_slot = var_t6_rdn2;
        *var_t6_rdn3_slot = var_t6_rdn3;
        *var_t6_rdn4_slot = var_t6_rdn4;
        *var_t6_rdn5_slot = var_t6_rdn5;
        *var_t6_rdn6_slot = var_t6_rdn6;
        *var_t6_rdn7_slot = var_t6_rdn7;
        *var_t6_rdn8_slot = var_t6_rdn8;
        *var_t6_rv_slot = var_t6_rv;
    }

    pub(super) fn stamp_reactive_block_89(
        p: &Parameters,
        var_delclm: f64,
        var_delclm_db0: f64,
        var_delclm_db1: f64,
        var_delclm_db2: f64,
        var_delclm_db3: f64,
        var_delclm_db4: f64,
        var_delclm_dn0: f64,
        var_delclm_dn1: f64,
        var_delclm_dn2: f64,
        var_delclm_dn3: f64,
        var_delclm_dn4: f64,
        var_delclm_dn5: f64,
        var_delclm_dn6: f64,
        var_delclm_dn7: f64,
        var_delclm_dn8: f64,
        var_devsign: f64,
        var_guard131: f64,
        var_ids: f64,
        var_ids_db0: f64,
        var_ids_db1: f64,
        var_ids_db2: f64,
        var_ids_db3: f64,
        var_ids_db4: f64,
        var_ids_dn0: f64,
        var_ids_dn1: f64,
        var_ids_dn2: f64,
        var_ids_dn3: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_leffnoi: f64,
        var_leffnoi_db0: f64,
        var_leffnoi_db1: f64,
        var_leffnoi_db2: f64,
        var_leffnoi_db3: f64,
        var_leffnoi_db4: f64,
        var_leffnoi_dn0: f64,
        var_leffnoi_dn1: f64,
        var_leffnoi_dn2: f64,
        var_leffnoi_dn3: f64,
        var_leffnoi_dn4: f64,
        var_leffnoi_dn5: f64,
        var_leffnoi_dn6: f64,
        var_leffnoi_dn7: f64,
        var_leffnoi_dn8: f64,
        var_leffnoisq: f64,
        var_leffnoisq_db0: f64,
        var_leffnoisq_db1: f64,
        var_leffnoisq_db2: f64,
        var_leffnoisq_db3: f64,
        var_leffnoisq_db4: f64,
        var_leffnoisq_dn0: f64,
        var_leffnoisq_dn1: f64,
        var_leffnoisq_dn2: f64,
        var_leffnoisq_dn3: f64,
        var_leffnoisq_dn4: f64,
        var_leffnoisq_dn5: f64,
        var_leffnoisq_dn6: f64,
        var_leffnoisq_dn7: f64,
        var_leffnoisq_dn8: f64,
        var_nl: f64,
        var_nl_db0: f64,
        var_nl_db1: f64,
        var_nl_db2: f64,
        var_nl_db3: f64,
        var_nl_db4: f64,
        var_nl_dn0: f64,
        var_nl_dn1: f64,
        var_nl_dn2: f64,
        var_nl_dn3: f64,
        var_nl_dn4: f64,
        var_nl_dn5: f64,
        var_nl_dn6: f64,
        var_nl_dn7: f64,
        var_nl_dn8: f64,
        var_noiaeff: f64,
        var_noiaeff_db0: f64,
        var_noiaeff_db1: f64,
        var_noiaeff_db2: f64,
        var_noiaeff_db3: f64,
        var_noiaeff_db4: f64,
        var_noiaeff_dn0: f64,
        var_noiaeff_dn1: f64,
        var_noiaeff_dn2: f64,
        var_noiaeff_dn3: f64,
        var_noiaeff_dn4: f64,
        var_noiaeff_dn5: f64,
        var_noiaeff_dn6: f64,
        var_noiaeff_dn7: f64,
        var_noiaeff_dn8: f64,
        var_nstar: f64,
        var_nstar_db0: f64,
        var_nstar_db1: f64,
        var_nstar_db2: f64,
        var_nstar_db3: f64,
        var_nstar_db4: f64,
        var_nstar_dn0: f64,
        var_nstar_dn1: f64,
        var_nstar_dn2: f64,
        var_nstar_dn3: f64,
        var_nstar_dn4: f64,
        var_nstar_dn5: f64,
        var_nstar_dn6: f64,
        var_nstar_dn7: f64,
        var_nstar_dn8: f64,
        var_qbg: f64,
        var_qbg_db0: f64,
        var_qbg_db1: f64,
        var_qbg_db2: f64,
        var_qbg_db3: f64,
        var_qbg_db4: f64,
        var_qbg_dn0: f64,
        var_qbg_dn1: f64,
        var_qbg_dn2: f64,
        var_qbg_dn3: f64,
        var_qbg_dn4: f64,
        var_qbg_dn5: f64,
        var_qbg_dn6: f64,
        var_qbg_dn7: f64,
        var_qbg_dn8: f64,
        var_qd: f64,
        var_qd_db0: f64,
        var_qd_db1: f64,
        var_qd_db2: f64,
        var_qd_db3: f64,
        var_qd_db4: f64,
        var_qd_dn0: f64,
        var_qd_dn1: f64,
        var_qd_dn2: f64,
        var_qd_dn3: f64,
        var_qd_dn4: f64,
        var_qd_dn5: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qd_dn8: f64,
        var_qfg: f64,
        var_qfg_db0: f64,
        var_qfg_db1: f64,
        var_qfg_db2: f64,
        var_qfg_db3: f64,
        var_qfg_db4: f64,
        var_qfg_dn0: f64,
        var_qfg_dn1: f64,
        var_qfg_dn2: f64,
        var_qfg_dn3: f64,
        var_qfg_dn4: f64,
        var_qfg_dn5: f64,
        var_qfg_dn6: f64,
        var_qfg_dn7: f64,
        var_qfg_dn8: f64,
        var_qs: f64,
        var_qs_db0: f64,
        var_qs_db1: f64,
        var_qs_db2: f64,
        var_qs_db3: f64,
        var_qs_db4: f64,
        var_qs_dn0: f64,
        var_qs_dn1: f64,
        var_qs_dn2: f64,
        var_qs_dn3: f64,
        var_qs_dn4: f64,
        var_qs_dn5: f64,
        var_qs_dn6: f64,
        var_qs_dn7: f64,
        var_qs_dn8: f64,
        var_sigvds: f64,
        var_t2: f64,
        var_t2_db0: f64,
        var_t2_db1: f64,
        var_t2_db2: f64,
        var_t2_db3: f64,
        var_t2_db4: f64,
        var_t2_dn0: f64,
        var_t2_dn1: f64,
        var_t2_dn2: f64,
        var_t2_dn3: f64,
        var_t2_dn4: f64,
        var_t2_dn5: f64,
        var_t2_dn6: f64,
        var_t2_dn7: f64,
        var_t2_dn8: f64,
        var_t3: f64,
        var_t3_db0: f64,
        var_t3_db1: f64,
        var_t3_db2: f64,
        var_t3_db3: f64,
        var_t3_db4: f64,
        var_t3_dn0: f64,
        var_t3_dn1: f64,
        var_t3_dn2: f64,
        var_t3_dn3: f64,
        var_t3_dn4: f64,
        var_t3_dn5: f64,
        var_t3_dn6: f64,
        var_t3_dn7: f64,
        var_t3_dn8: f64,
        var_t4: f64,
        var_t4_db0: f64,
        var_t4_db1: f64,
        var_t4_db2: f64,
        var_t4_db3: f64,
        var_t4_db4: f64,
        var_t4_dn0: f64,
        var_t4_dn1: f64,
        var_t4_dn2: f64,
        var_t4_dn3: f64,
        var_t4_dn4: f64,
        var_t4_dn5: f64,
        var_t4_dn6: f64,
        var_t4_dn7: f64,
        var_t4_dn8: f64,
        var_t5: f64,
        var_t5_db0: f64,
        var_t5_db1: f64,
        var_t5_db2: f64,
        var_t5_db3: f64,
        var_t5_db4: f64,
        var_t5_dn0: f64,
        var_t5_dn1: f64,
        var_t5_dn2: f64,
        var_t5_dn3: f64,
        var_t5_dn4: f64,
        var_t5_dn5: f64,
        var_t5_dn6: f64,
        var_t5_dn7: f64,
        var_t5_dn8: f64,
        var_t6: f64,
        var_t6_db0: f64,
        var_t6_db1: f64,
        var_t6_db2: f64,
        var_t6_db3: f64,
        var_t6_db4: f64,
        var_t6_dn0: f64,
        var_t6_dn1: f64,
        var_t6_dn2: f64,
        var_t6_dn3: f64,
        var_t6_dn4: f64,
        var_t6_dn5: f64,
        var_t6_dn6: f64,
        var_t6_dn7: f64,
        var_t6_dn8: f64,
        var_vtm: f64,
        var_vtm_db0: f64,
        var_vtm_db1: f64,
        var_vtm_db2: f64,
        var_vtm_db3: f64,
        var_vtm_db4: f64,
        var_vtm_dn0: f64,
        var_vtm_dn1: f64,
        var_vtm_dn2: f64,
        var_vtm_dn3: f64,
        var_vtm_dn4: f64,
        var_vtm_dn5: f64,
        var_vtm_dn6: f64,
        var_vtm_dn7: f64,
        var_vtm_dn8: f64,
        var_weff: f64,
        var_weff_db0: f64,
        var_weff_db1: f64,
        var_weff_db2: f64,
        var_weff_db3: f64,
        var_weff_db4: f64,
        var_weff_dn0: f64,
        var_weff_dn1: f64,
        var_weff_dn2: f64,
        var_weff_dn3: f64,
        var_weff_dn4: f64,
        var_weff_dn5: f64,
        var_weff_dn6: f64,
        var_weff_dn7: f64,
        var_weff_dn8: f64,
        var_guard136_slot: &mut f64,
        var_guard136_db0_slot: &mut f64,
        var_guard136_db1_slot: &mut f64,
        var_guard136_db2_slot: &mut f64,
        var_guard136_db3_slot: &mut f64,
        var_guard136_db4_slot: &mut f64,
        var_guard136_dn0_slot: &mut f64,
        var_guard136_dn1_slot: &mut f64,
        var_guard136_dn2_slot: &mut f64,
        var_guard136_dn3_slot: &mut f64,
        var_guard136_dn4_slot: &mut f64,
        var_guard136_dn5_slot: &mut f64,
        var_guard136_dn6_slot: &mut f64,
        var_guard136_dn7_slot: &mut f64,
        var_guard136_dn8_slot: &mut f64,
        var_guard136_rdb0_slot: &mut f64,
        var_guard136_rdb1_slot: &mut f64,
        var_guard136_rdb2_slot: &mut f64,
        var_guard136_rdb3_slot: &mut f64,
        var_guard136_rdb4_slot: &mut f64,
        var_guard136_rdn0_slot: &mut f64,
        var_guard136_rdn1_slot: &mut f64,
        var_guard136_rdn2_slot: &mut f64,
        var_guard136_rdn3_slot: &mut f64,
        var_guard136_rdn4_slot: &mut f64,
        var_guard136_rdn5_slot: &mut f64,
        var_guard136_rdn6_slot: &mut f64,
        var_guard136_rdn7_slot: &mut f64,
        var_guard136_rdn8_slot: &mut f64,
        var_guard136_rv_slot: &mut f64,
        var_qbgi_slot: &mut f64,
        var_qbgi_db0_slot: &mut f64,
        var_qbgi_db1_slot: &mut f64,
        var_qbgi_db2_slot: &mut f64,
        var_qbgi_db3_slot: &mut f64,
        var_qbgi_db4_slot: &mut f64,
        var_qbgi_dn0_slot: &mut f64,
        var_qbgi_dn1_slot: &mut f64,
        var_qbgi_dn2_slot: &mut f64,
        var_qbgi_dn3_slot: &mut f64,
        var_qbgi_dn4_slot: &mut f64,
        var_qbgi_dn5_slot: &mut f64,
        var_qbgi_dn6_slot: &mut f64,
        var_qbgi_dn7_slot: &mut f64,
        var_qbgi_dn8_slot: &mut f64,
        var_qbgi_rdb0_slot: &mut f64,
        var_qbgi_rdb1_slot: &mut f64,
        var_qbgi_rdb2_slot: &mut f64,
        var_qbgi_rdb3_slot: &mut f64,
        var_qbgi_rdb4_slot: &mut f64,
        var_qbgi_rdn0_slot: &mut f64,
        var_qbgi_rdn1_slot: &mut f64,
        var_qbgi_rdn2_slot: &mut f64,
        var_qbgi_rdn3_slot: &mut f64,
        var_qbgi_rdn4_slot: &mut f64,
        var_qbgi_rdn5_slot: &mut f64,
        var_qbgi_rdn6_slot: &mut f64,
        var_qbgi_rdn7_slot: &mut f64,
        var_qbgi_rdn8_slot: &mut f64,
        var_qbgi_rv_slot: &mut f64,
        var_qdi_slot: &mut f64,
        var_qdi_db0_slot: &mut f64,
        var_qdi_db1_slot: &mut f64,
        var_qdi_db2_slot: &mut f64,
        var_qdi_db3_slot: &mut f64,
        var_qdi_db4_slot: &mut f64,
        var_qdi_dn0_slot: &mut f64,
        var_qdi_dn1_slot: &mut f64,
        var_qdi_dn2_slot: &mut f64,
        var_qdi_dn3_slot: &mut f64,
        var_qdi_dn4_slot: &mut f64,
        var_qdi_dn5_slot: &mut f64,
        var_qdi_dn6_slot: &mut f64,
        var_qdi_dn7_slot: &mut f64,
        var_qdi_dn8_slot: &mut f64,
        var_qdi_rdb0_slot: &mut f64,
        var_qdi_rdb1_slot: &mut f64,
        var_qdi_rdb2_slot: &mut f64,
        var_qdi_rdb3_slot: &mut f64,
        var_qdi_rdb4_slot: &mut f64,
        var_qdi_rdn0_slot: &mut f64,
        var_qdi_rdn1_slot: &mut f64,
        var_qdi_rdn2_slot: &mut f64,
        var_qdi_rdn3_slot: &mut f64,
        var_qdi_rdn4_slot: &mut f64,
        var_qdi_rdn5_slot: &mut f64,
        var_qdi_rdn6_slot: &mut f64,
        var_qdi_rdn7_slot: &mut f64,
        var_qdi_rdn8_slot: &mut f64,
        var_qdi_rv_slot: &mut f64,
        var_qfgi_slot: &mut f64,
        var_qfgi_db0_slot: &mut f64,
        var_qfgi_db1_slot: &mut f64,
        var_qfgi_db2_slot: &mut f64,
        var_qfgi_db3_slot: &mut f64,
        var_qfgi_db4_slot: &mut f64,
        var_qfgi_dn0_slot: &mut f64,
        var_qfgi_dn1_slot: &mut f64,
        var_qfgi_dn2_slot: &mut f64,
        var_qfgi_dn3_slot: &mut f64,
        var_qfgi_dn4_slot: &mut f64,
        var_qfgi_dn5_slot: &mut f64,
        var_qfgi_dn6_slot: &mut f64,
        var_qfgi_dn7_slot: &mut f64,
        var_qfgi_dn8_slot: &mut f64,
        var_qfgi_rdb0_slot: &mut f64,
        var_qfgi_rdb1_slot: &mut f64,
        var_qfgi_rdb2_slot: &mut f64,
        var_qfgi_rdb3_slot: &mut f64,
        var_qfgi_rdb4_slot: &mut f64,
        var_qfgi_rdn0_slot: &mut f64,
        var_qfgi_rdn1_slot: &mut f64,
        var_qfgi_rdn2_slot: &mut f64,
        var_qfgi_rdn3_slot: &mut f64,
        var_qfgi_rdn4_slot: &mut f64,
        var_qfgi_rdn5_slot: &mut f64,
        var_qfgi_rdn6_slot: &mut f64,
        var_qfgi_rdn7_slot: &mut f64,
        var_qfgi_rdn8_slot: &mut f64,
        var_qfgi_rv_slot: &mut f64,
        var_qsi_slot: &mut f64,
        var_qsi_db0_slot: &mut f64,
        var_qsi_db1_slot: &mut f64,
        var_qsi_db2_slot: &mut f64,
        var_qsi_db3_slot: &mut f64,
        var_qsi_db4_slot: &mut f64,
        var_qsi_dn0_slot: &mut f64,
        var_qsi_dn1_slot: &mut f64,
        var_qsi_dn2_slot: &mut f64,
        var_qsi_dn3_slot: &mut f64,
        var_qsi_dn4_slot: &mut f64,
        var_qsi_dn5_slot: &mut f64,
        var_qsi_dn6_slot: &mut f64,
        var_qsi_dn7_slot: &mut f64,
        var_qsi_dn8_slot: &mut f64,
        var_qsi_rdb0_slot: &mut f64,
        var_qsi_rdb1_slot: &mut f64,
        var_qsi_rdb2_slot: &mut f64,
        var_qsi_rdb3_slot: &mut f64,
        var_qsi_rdb4_slot: &mut f64,
        var_qsi_rdn0_slot: &mut f64,
        var_qsi_rdn1_slot: &mut f64,
        var_qsi_rdn2_slot: &mut f64,
        var_qsi_rdn3_slot: &mut f64,
        var_qsi_rdn4_slot: &mut f64,
        var_qsi_rdn5_slot: &mut f64,
        var_qsi_rdn6_slot: &mut f64,
        var_qsi_rdn7_slot: &mut f64,
        var_qsi_rdn8_slot: &mut f64,
        var_qsi_rv_slot: &mut f64,
        var_ssi_slot: &mut f64,
        var_ssi_db0_slot: &mut f64,
        var_ssi_db1_slot: &mut f64,
        var_ssi_db2_slot: &mut f64,
        var_ssi_db3_slot: &mut f64,
        var_ssi_db4_slot: &mut f64,
        var_ssi_dn0_slot: &mut f64,
        var_ssi_dn1_slot: &mut f64,
        var_ssi_dn2_slot: &mut f64,
        var_ssi_dn3_slot: &mut f64,
        var_ssi_dn4_slot: &mut f64,
        var_ssi_dn5_slot: &mut f64,
        var_ssi_dn6_slot: &mut f64,
        var_ssi_dn7_slot: &mut f64,
        var_ssi_dn8_slot: &mut f64,
        var_ssi_rdb0_slot: &mut f64,
        var_ssi_rdb1_slot: &mut f64,
        var_ssi_rdb2_slot: &mut f64,
        var_ssi_rdb3_slot: &mut f64,
        var_ssi_rdb4_slot: &mut f64,
        var_ssi_rdn0_slot: &mut f64,
        var_ssi_rdn1_slot: &mut f64,
        var_ssi_rdn2_slot: &mut f64,
        var_ssi_rdn3_slot: &mut f64,
        var_ssi_rdn4_slot: &mut f64,
        var_ssi_rdn5_slot: &mut f64,
        var_ssi_rdn6_slot: &mut f64,
        var_ssi_rdn7_slot: &mut f64,
        var_ssi_rdn8_slot: &mut f64,
        var_ssi_rv_slot: &mut f64,
        var_swi_slot: &mut f64,
        var_swi_db0_slot: &mut f64,
        var_swi_db1_slot: &mut f64,
        var_swi_db2_slot: &mut f64,
        var_swi_db3_slot: &mut f64,
        var_swi_db4_slot: &mut f64,
        var_swi_dn0_slot: &mut f64,
        var_swi_dn1_slot: &mut f64,
        var_swi_dn2_slot: &mut f64,
        var_swi_dn3_slot: &mut f64,
        var_swi_dn4_slot: &mut f64,
        var_swi_dn5_slot: &mut f64,
        var_swi_dn6_slot: &mut f64,
        var_swi_dn7_slot: &mut f64,
        var_swi_dn8_slot: &mut f64,
        var_swi_rdb0_slot: &mut f64,
        var_swi_rdb1_slot: &mut f64,
        var_swi_rdb2_slot: &mut f64,
        var_swi_rdb3_slot: &mut f64,
        var_swi_rdb4_slot: &mut f64,
        var_swi_rdn0_slot: &mut f64,
        var_swi_rdn1_slot: &mut f64,
        var_swi_rdn2_slot: &mut f64,
        var_swi_rdn3_slot: &mut f64,
        var_swi_rdn4_slot: &mut f64,
        var_swi_rdn5_slot: &mut f64,
        var_swi_rdn6_slot: &mut f64,
        var_swi_rdn7_slot: &mut f64,
        var_swi_rdn8_slot: &mut f64,
        var_swi_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t10_slot: &mut f64,
        var_t10_db0_slot: &mut f64,
        var_t10_db1_slot: &mut f64,
        var_t10_db2_slot: &mut f64,
        var_t10_db3_slot: &mut f64,
        var_t10_db4_slot: &mut f64,
        var_t10_dn0_slot: &mut f64,
        var_t10_dn1_slot: &mut f64,
        var_t10_dn2_slot: &mut f64,
        var_t10_dn3_slot: &mut f64,
        var_t10_dn4_slot: &mut f64,
        var_t10_dn5_slot: &mut f64,
        var_t10_dn6_slot: &mut f64,
        var_t10_dn7_slot: &mut f64,
        var_t10_dn8_slot: &mut f64,
        var_t10_rdb0_slot: &mut f64,
        var_t10_rdb1_slot: &mut f64,
        var_t10_rdb2_slot: &mut f64,
        var_t10_rdb3_slot: &mut f64,
        var_t10_rdb4_slot: &mut f64,
        var_t10_rdn0_slot: &mut f64,
        var_t10_rdn1_slot: &mut f64,
        var_t10_rdn2_slot: &mut f64,
        var_t10_rdn3_slot: &mut f64,
        var_t10_rdn4_slot: &mut f64,
        var_t10_rdn5_slot: &mut f64,
        var_t10_rdn6_slot: &mut f64,
        var_t10_rdn7_slot: &mut f64,
        var_t10_rdn8_slot: &mut f64,
        var_t10_rv_slot: &mut f64,
        var_t11_slot: &mut f64,
        var_t11_db0_slot: &mut f64,
        var_t11_db1_slot: &mut f64,
        var_t11_db2_slot: &mut f64,
        var_t11_db3_slot: &mut f64,
        var_t11_db4_slot: &mut f64,
        var_t11_dn0_slot: &mut f64,
        var_t11_dn1_slot: &mut f64,
        var_t11_dn2_slot: &mut f64,
        var_t11_dn3_slot: &mut f64,
        var_t11_dn4_slot: &mut f64,
        var_t11_dn5_slot: &mut f64,
        var_t11_dn6_slot: &mut f64,
        var_t11_dn7_slot: &mut f64,
        var_t11_dn8_slot: &mut f64,
        var_t11_rdb0_slot: &mut f64,
        var_t11_rdb1_slot: &mut f64,
        var_t11_rdb2_slot: &mut f64,
        var_t11_rdb3_slot: &mut f64,
        var_t11_rdb4_slot: &mut f64,
        var_t11_rdn0_slot: &mut f64,
        var_t11_rdn1_slot: &mut f64,
        var_t11_rdn2_slot: &mut f64,
        var_t11_rdn3_slot: &mut f64,
        var_t11_rdn4_slot: &mut f64,
        var_t11_rdn5_slot: &mut f64,
        var_t11_rdn6_slot: &mut f64,
        var_t11_rdn7_slot: &mut f64,
        var_t11_rdn8_slot: &mut f64,
        var_t11_rv_slot: &mut f64,
        var_t1_db0_slot: &mut f64,
        var_t1_db1_slot: &mut f64,
        var_t1_db2_slot: &mut f64,
        var_t1_db3_slot: &mut f64,
        var_t1_db4_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rdb0_slot: &mut f64,
        var_t1_rdb1_slot: &mut f64,
        var_t1_rdb2_slot: &mut f64,
        var_t1_rdb3_slot: &mut f64,
        var_t1_rdb4_slot: &mut f64,
        var_t1_rdn0_slot: &mut f64,
        var_t1_rdn1_slot: &mut f64,
        var_t1_rdn2_slot: &mut f64,
        var_t1_rdn3_slot: &mut f64,
        var_t1_rdn4_slot: &mut f64,
        var_t1_rdn5_slot: &mut f64,
        var_t1_rdn6_slot: &mut f64,
        var_t1_rdn7_slot: &mut f64,
        var_t1_rdn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_db0_slot: &mut f64,
        var_t7_db1_slot: &mut f64,
        var_t7_db2_slot: &mut f64,
        var_t7_db3_slot: &mut f64,
        var_t7_db4_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn1_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn3_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_rdb0_slot: &mut f64,
        var_t7_rdb1_slot: &mut f64,
        var_t7_rdb2_slot: &mut f64,
        var_t7_rdb3_slot: &mut f64,
        var_t7_rdb4_slot: &mut f64,
        var_t7_rdn0_slot: &mut f64,
        var_t7_rdn1_slot: &mut f64,
        var_t7_rdn2_slot: &mut f64,
        var_t7_rdn3_slot: &mut f64,
        var_t7_rdn4_slot: &mut f64,
        var_t7_rdn5_slot: &mut f64,
        var_t7_rdn6_slot: &mut f64,
        var_t7_rdn7_slot: &mut f64,
        var_t7_rdn8_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_db0_slot: &mut f64,
        var_t8_db1_slot: &mut f64,
        var_t8_db2_slot: &mut f64,
        var_t8_db3_slot: &mut f64,
        var_t8_db4_slot: &mut f64,
        var_t8_dn0_slot: &mut f64,
        var_t8_dn1_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn3_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_rdb0_slot: &mut f64,
        var_t8_rdb1_slot: &mut f64,
        var_t8_rdb2_slot: &mut f64,
        var_t8_rdb3_slot: &mut f64,
        var_t8_rdb4_slot: &mut f64,
        var_t8_rdn0_slot: &mut f64,
        var_t8_rdn1_slot: &mut f64,
        var_t8_rdn2_slot: &mut f64,
        var_t8_rdn3_slot: &mut f64,
        var_t8_rdn4_slot: &mut f64,
        var_t8_rdn5_slot: &mut f64,
        var_t8_rdn6_slot: &mut f64,
        var_t8_rdn7_slot: &mut f64,
        var_t8_rdn8_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
        var_t9_slot: &mut f64,
        var_t9_db0_slot: &mut f64,
        var_t9_db1_slot: &mut f64,
        var_t9_db2_slot: &mut f64,
        var_t9_db3_slot: &mut f64,
        var_t9_db4_slot: &mut f64,
        var_t9_dn0_slot: &mut f64,
        var_t9_dn1_slot: &mut f64,
        var_t9_dn2_slot: &mut f64,
        var_t9_dn3_slot: &mut f64,
        var_t9_dn4_slot: &mut f64,
        var_t9_dn5_slot: &mut f64,
        var_t9_dn6_slot: &mut f64,
        var_t9_dn7_slot: &mut f64,
        var_t9_dn8_slot: &mut f64,
        var_t9_rdb0_slot: &mut f64,
        var_t9_rdb1_slot: &mut f64,
        var_t9_rdb2_slot: &mut f64,
        var_t9_rdb3_slot: &mut f64,
        var_t9_rdb4_slot: &mut f64,
        var_t9_rdn0_slot: &mut f64,
        var_t9_rdn1_slot: &mut f64,
        var_t9_rdn2_slot: &mut f64,
        var_t9_rdn3_slot: &mut f64,
        var_t9_rdn4_slot: &mut f64,
        var_t9_rdn5_slot: &mut f64,
        var_t9_rdn6_slot: &mut f64,
        var_t9_rdn7_slot: &mut f64,
        var_t9_rdn8_slot: &mut f64,
        var_t9_rv_slot: &mut f64,
    ) {
        let mut var_guard136: f64 = *var_guard136_slot;
        let mut var_guard136_db0: f64 = *var_guard136_db0_slot;
        let mut var_guard136_db1: f64 = *var_guard136_db1_slot;
        let mut var_guard136_db2: f64 = *var_guard136_db2_slot;
        let mut var_guard136_db3: f64 = *var_guard136_db3_slot;
        let mut var_guard136_db4: f64 = *var_guard136_db4_slot;
        let mut var_guard136_dn0: f64 = *var_guard136_dn0_slot;
        let mut var_guard136_dn1: f64 = *var_guard136_dn1_slot;
        let mut var_guard136_dn2: f64 = *var_guard136_dn2_slot;
        let mut var_guard136_dn3: f64 = *var_guard136_dn3_slot;
        let mut var_guard136_dn4: f64 = *var_guard136_dn4_slot;
        let mut var_guard136_dn5: f64 = *var_guard136_dn5_slot;
        let mut var_guard136_dn6: f64 = *var_guard136_dn6_slot;
        let mut var_guard136_dn7: f64 = *var_guard136_dn7_slot;
        let mut var_guard136_dn8: f64 = *var_guard136_dn8_slot;
        let mut var_guard136_rdb0: f64 = *var_guard136_rdb0_slot;
        let mut var_guard136_rdb1: f64 = *var_guard136_rdb1_slot;
        let mut var_guard136_rdb2: f64 = *var_guard136_rdb2_slot;
        let mut var_guard136_rdb3: f64 = *var_guard136_rdb3_slot;
        let mut var_guard136_rdb4: f64 = *var_guard136_rdb4_slot;
        let mut var_guard136_rdn0: f64 = *var_guard136_rdn0_slot;
        let mut var_guard136_rdn1: f64 = *var_guard136_rdn1_slot;
        let mut var_guard136_rdn2: f64 = *var_guard136_rdn2_slot;
        let mut var_guard136_rdn3: f64 = *var_guard136_rdn3_slot;
        let mut var_guard136_rdn4: f64 = *var_guard136_rdn4_slot;
        let mut var_guard136_rdn5: f64 = *var_guard136_rdn5_slot;
        let mut var_guard136_rdn6: f64 = *var_guard136_rdn6_slot;
        let mut var_guard136_rdn7: f64 = *var_guard136_rdn7_slot;
        let mut var_guard136_rdn8: f64 = *var_guard136_rdn8_slot;
        let mut var_guard136_rv: f64 = *var_guard136_rv_slot;
        let mut var_qbgi: f64 = *var_qbgi_slot;
        let mut var_qbgi_db0: f64 = *var_qbgi_db0_slot;
        let mut var_qbgi_db1: f64 = *var_qbgi_db1_slot;
        let mut var_qbgi_db2: f64 = *var_qbgi_db2_slot;
        let mut var_qbgi_db3: f64 = *var_qbgi_db3_slot;
        let mut var_qbgi_db4: f64 = *var_qbgi_db4_slot;
        let mut var_qbgi_dn0: f64 = *var_qbgi_dn0_slot;
        let mut var_qbgi_dn1: f64 = *var_qbgi_dn1_slot;
        let mut var_qbgi_dn2: f64 = *var_qbgi_dn2_slot;
        let mut var_qbgi_dn3: f64 = *var_qbgi_dn3_slot;
        let mut var_qbgi_dn4: f64 = *var_qbgi_dn4_slot;
        let mut var_qbgi_dn5: f64 = *var_qbgi_dn5_slot;
        let mut var_qbgi_dn6: f64 = *var_qbgi_dn6_slot;
        let mut var_qbgi_dn7: f64 = *var_qbgi_dn7_slot;
        let mut var_qbgi_dn8: f64 = *var_qbgi_dn8_slot;
        let mut var_qbgi_rdb0: f64 = *var_qbgi_rdb0_slot;
        let mut var_qbgi_rdb1: f64 = *var_qbgi_rdb1_slot;
        let mut var_qbgi_rdb2: f64 = *var_qbgi_rdb2_slot;
        let mut var_qbgi_rdb3: f64 = *var_qbgi_rdb3_slot;
        let mut var_qbgi_rdb4: f64 = *var_qbgi_rdb4_slot;
        let mut var_qbgi_rdn0: f64 = *var_qbgi_rdn0_slot;
        let mut var_qbgi_rdn1: f64 = *var_qbgi_rdn1_slot;
        let mut var_qbgi_rdn2: f64 = *var_qbgi_rdn2_slot;
        let mut var_qbgi_rdn3: f64 = *var_qbgi_rdn3_slot;
        let mut var_qbgi_rdn4: f64 = *var_qbgi_rdn4_slot;
        let mut var_qbgi_rdn5: f64 = *var_qbgi_rdn5_slot;
        let mut var_qbgi_rdn6: f64 = *var_qbgi_rdn6_slot;
        let mut var_qbgi_rdn7: f64 = *var_qbgi_rdn7_slot;
        let mut var_qbgi_rdn8: f64 = *var_qbgi_rdn8_slot;
        let mut var_qbgi_rv: f64 = *var_qbgi_rv_slot;
        let mut var_qdi: f64 = *var_qdi_slot;
        let mut var_qdi_db0: f64 = *var_qdi_db0_slot;
        let mut var_qdi_db1: f64 = *var_qdi_db1_slot;
        let mut var_qdi_db2: f64 = *var_qdi_db2_slot;
        let mut var_qdi_db3: f64 = *var_qdi_db3_slot;
        let mut var_qdi_db4: f64 = *var_qdi_db4_slot;
        let mut var_qdi_dn0: f64 = *var_qdi_dn0_slot;
        let mut var_qdi_dn1: f64 = *var_qdi_dn1_slot;
        let mut var_qdi_dn2: f64 = *var_qdi_dn2_slot;
        let mut var_qdi_dn3: f64 = *var_qdi_dn3_slot;
        let mut var_qdi_dn4: f64 = *var_qdi_dn4_slot;
        let mut var_qdi_dn5: f64 = *var_qdi_dn5_slot;
        let mut var_qdi_dn6: f64 = *var_qdi_dn6_slot;
        let mut var_qdi_dn7: f64 = *var_qdi_dn7_slot;
        let mut var_qdi_dn8: f64 = *var_qdi_dn8_slot;
        let mut var_qdi_rdb0: f64 = *var_qdi_rdb0_slot;
        let mut var_qdi_rdb1: f64 = *var_qdi_rdb1_slot;
        let mut var_qdi_rdb2: f64 = *var_qdi_rdb2_slot;
        let mut var_qdi_rdb3: f64 = *var_qdi_rdb3_slot;
        let mut var_qdi_rdb4: f64 = *var_qdi_rdb4_slot;
        let mut var_qdi_rdn0: f64 = *var_qdi_rdn0_slot;
        let mut var_qdi_rdn1: f64 = *var_qdi_rdn1_slot;
        let mut var_qdi_rdn2: f64 = *var_qdi_rdn2_slot;
        let mut var_qdi_rdn3: f64 = *var_qdi_rdn3_slot;
        let mut var_qdi_rdn4: f64 = *var_qdi_rdn4_slot;
        let mut var_qdi_rdn5: f64 = *var_qdi_rdn5_slot;
        let mut var_qdi_rdn6: f64 = *var_qdi_rdn6_slot;
        let mut var_qdi_rdn7: f64 = *var_qdi_rdn7_slot;
        let mut var_qdi_rdn8: f64 = *var_qdi_rdn8_slot;
        let mut var_qdi_rv: f64 = *var_qdi_rv_slot;
        let mut var_qfgi: f64 = *var_qfgi_slot;
        let mut var_qfgi_db0: f64 = *var_qfgi_db0_slot;
        let mut var_qfgi_db1: f64 = *var_qfgi_db1_slot;
        let mut var_qfgi_db2: f64 = *var_qfgi_db2_slot;
        let mut var_qfgi_db3: f64 = *var_qfgi_db3_slot;
        let mut var_qfgi_db4: f64 = *var_qfgi_db4_slot;
        let mut var_qfgi_dn0: f64 = *var_qfgi_dn0_slot;
        let mut var_qfgi_dn1: f64 = *var_qfgi_dn1_slot;
        let mut var_qfgi_dn2: f64 = *var_qfgi_dn2_slot;
        let mut var_qfgi_dn3: f64 = *var_qfgi_dn3_slot;
        let mut var_qfgi_dn4: f64 = *var_qfgi_dn4_slot;
        let mut var_qfgi_dn5: f64 = *var_qfgi_dn5_slot;
        let mut var_qfgi_dn6: f64 = *var_qfgi_dn6_slot;
        let mut var_qfgi_dn7: f64 = *var_qfgi_dn7_slot;
        let mut var_qfgi_dn8: f64 = *var_qfgi_dn8_slot;
        let mut var_qfgi_rdb0: f64 = *var_qfgi_rdb0_slot;
        let mut var_qfgi_rdb1: f64 = *var_qfgi_rdb1_slot;
        let mut var_qfgi_rdb2: f64 = *var_qfgi_rdb2_slot;
        let mut var_qfgi_rdb3: f64 = *var_qfgi_rdb3_slot;
        let mut var_qfgi_rdb4: f64 = *var_qfgi_rdb4_slot;
        let mut var_qfgi_rdn0: f64 = *var_qfgi_rdn0_slot;
        let mut var_qfgi_rdn1: f64 = *var_qfgi_rdn1_slot;
        let mut var_qfgi_rdn2: f64 = *var_qfgi_rdn2_slot;
        let mut var_qfgi_rdn3: f64 = *var_qfgi_rdn3_slot;
        let mut var_qfgi_rdn4: f64 = *var_qfgi_rdn4_slot;
        let mut var_qfgi_rdn5: f64 = *var_qfgi_rdn5_slot;
        let mut var_qfgi_rdn6: f64 = *var_qfgi_rdn6_slot;
        let mut var_qfgi_rdn7: f64 = *var_qfgi_rdn7_slot;
        let mut var_qfgi_rdn8: f64 = *var_qfgi_rdn8_slot;
        let mut var_qfgi_rv: f64 = *var_qfgi_rv_slot;
        let mut var_qsi: f64 = *var_qsi_slot;
        let mut var_qsi_db0: f64 = *var_qsi_db0_slot;
        let mut var_qsi_db1: f64 = *var_qsi_db1_slot;
        let mut var_qsi_db2: f64 = *var_qsi_db2_slot;
        let mut var_qsi_db3: f64 = *var_qsi_db3_slot;
        let mut var_qsi_db4: f64 = *var_qsi_db4_slot;
        let mut var_qsi_dn0: f64 = *var_qsi_dn0_slot;
        let mut var_qsi_dn1: f64 = *var_qsi_dn1_slot;
        let mut var_qsi_dn2: f64 = *var_qsi_dn2_slot;
        let mut var_qsi_dn3: f64 = *var_qsi_dn3_slot;
        let mut var_qsi_dn4: f64 = *var_qsi_dn4_slot;
        let mut var_qsi_dn5: f64 = *var_qsi_dn5_slot;
        let mut var_qsi_dn6: f64 = *var_qsi_dn6_slot;
        let mut var_qsi_dn7: f64 = *var_qsi_dn7_slot;
        let mut var_qsi_dn8: f64 = *var_qsi_dn8_slot;
        let mut var_qsi_rdb0: f64 = *var_qsi_rdb0_slot;
        let mut var_qsi_rdb1: f64 = *var_qsi_rdb1_slot;
        let mut var_qsi_rdb2: f64 = *var_qsi_rdb2_slot;
        let mut var_qsi_rdb3: f64 = *var_qsi_rdb3_slot;
        let mut var_qsi_rdb4: f64 = *var_qsi_rdb4_slot;
        let mut var_qsi_rdn0: f64 = *var_qsi_rdn0_slot;
        let mut var_qsi_rdn1: f64 = *var_qsi_rdn1_slot;
        let mut var_qsi_rdn2: f64 = *var_qsi_rdn2_slot;
        let mut var_qsi_rdn3: f64 = *var_qsi_rdn3_slot;
        let mut var_qsi_rdn4: f64 = *var_qsi_rdn4_slot;
        let mut var_qsi_rdn5: f64 = *var_qsi_rdn5_slot;
        let mut var_qsi_rdn6: f64 = *var_qsi_rdn6_slot;
        let mut var_qsi_rdn7: f64 = *var_qsi_rdn7_slot;
        let mut var_qsi_rdn8: f64 = *var_qsi_rdn8_slot;
        let mut var_qsi_rv: f64 = *var_qsi_rv_slot;
        let mut var_ssi: f64 = *var_ssi_slot;
        let mut var_ssi_db0: f64 = *var_ssi_db0_slot;
        let mut var_ssi_db1: f64 = *var_ssi_db1_slot;
        let mut var_ssi_db2: f64 = *var_ssi_db2_slot;
        let mut var_ssi_db3: f64 = *var_ssi_db3_slot;
        let mut var_ssi_db4: f64 = *var_ssi_db4_slot;
        let mut var_ssi_dn0: f64 = *var_ssi_dn0_slot;
        let mut var_ssi_dn1: f64 = *var_ssi_dn1_slot;
        let mut var_ssi_dn2: f64 = *var_ssi_dn2_slot;
        let mut var_ssi_dn3: f64 = *var_ssi_dn3_slot;
        let mut var_ssi_dn4: f64 = *var_ssi_dn4_slot;
        let mut var_ssi_dn5: f64 = *var_ssi_dn5_slot;
        let mut var_ssi_dn6: f64 = *var_ssi_dn6_slot;
        let mut var_ssi_dn7: f64 = *var_ssi_dn7_slot;
        let mut var_ssi_dn8: f64 = *var_ssi_dn8_slot;
        let mut var_ssi_rdb0: f64 = *var_ssi_rdb0_slot;
        let mut var_ssi_rdb1: f64 = *var_ssi_rdb1_slot;
        let mut var_ssi_rdb2: f64 = *var_ssi_rdb2_slot;
        let mut var_ssi_rdb3: f64 = *var_ssi_rdb3_slot;
        let mut var_ssi_rdb4: f64 = *var_ssi_rdb4_slot;
        let mut var_ssi_rdn0: f64 = *var_ssi_rdn0_slot;
        let mut var_ssi_rdn1: f64 = *var_ssi_rdn1_slot;
        let mut var_ssi_rdn2: f64 = *var_ssi_rdn2_slot;
        let mut var_ssi_rdn3: f64 = *var_ssi_rdn3_slot;
        let mut var_ssi_rdn4: f64 = *var_ssi_rdn4_slot;
        let mut var_ssi_rdn5: f64 = *var_ssi_rdn5_slot;
        let mut var_ssi_rdn6: f64 = *var_ssi_rdn6_slot;
        let mut var_ssi_rdn7: f64 = *var_ssi_rdn7_slot;
        let mut var_ssi_rdn8: f64 = *var_ssi_rdn8_slot;
        let mut var_ssi_rv: f64 = *var_ssi_rv_slot;
        let mut var_swi: f64 = *var_swi_slot;
        let mut var_swi_db0: f64 = *var_swi_db0_slot;
        let mut var_swi_db1: f64 = *var_swi_db1_slot;
        let mut var_swi_db2: f64 = *var_swi_db2_slot;
        let mut var_swi_db3: f64 = *var_swi_db3_slot;
        let mut var_swi_db4: f64 = *var_swi_db4_slot;
        let mut var_swi_dn0: f64 = *var_swi_dn0_slot;
        let mut var_swi_dn1: f64 = *var_swi_dn1_slot;
        let mut var_swi_dn2: f64 = *var_swi_dn2_slot;
        let mut var_swi_dn3: f64 = *var_swi_dn3_slot;
        let mut var_swi_dn4: f64 = *var_swi_dn4_slot;
        let mut var_swi_dn5: f64 = *var_swi_dn5_slot;
        let mut var_swi_dn6: f64 = *var_swi_dn6_slot;
        let mut var_swi_dn7: f64 = *var_swi_dn7_slot;
        let mut var_swi_dn8: f64 = *var_swi_dn8_slot;
        let mut var_swi_rdb0: f64 = *var_swi_rdb0_slot;
        let mut var_swi_rdb1: f64 = *var_swi_rdb1_slot;
        let mut var_swi_rdb2: f64 = *var_swi_rdb2_slot;
        let mut var_swi_rdb3: f64 = *var_swi_rdb3_slot;
        let mut var_swi_rdb4: f64 = *var_swi_rdb4_slot;
        let mut var_swi_rdn0: f64 = *var_swi_rdn0_slot;
        let mut var_swi_rdn1: f64 = *var_swi_rdn1_slot;
        let mut var_swi_rdn2: f64 = *var_swi_rdn2_slot;
        let mut var_swi_rdn3: f64 = *var_swi_rdn3_slot;
        let mut var_swi_rdn4: f64 = *var_swi_rdn4_slot;
        let mut var_swi_rdn5: f64 = *var_swi_rdn5_slot;
        let mut var_swi_rdn6: f64 = *var_swi_rdn6_slot;
        let mut var_swi_rdn7: f64 = *var_swi_rdn7_slot;
        let mut var_swi_rdn8: f64 = *var_swi_rdn8_slot;
        let mut var_swi_rv: f64 = *var_swi_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t10: f64 = *var_t10_slot;
        let mut var_t10_db0: f64 = *var_t10_db0_slot;
        let mut var_t10_db1: f64 = *var_t10_db1_slot;
        let mut var_t10_db2: f64 = *var_t10_db2_slot;
        let mut var_t10_db3: f64 = *var_t10_db3_slot;
        let mut var_t10_db4: f64 = *var_t10_db4_slot;
        let mut var_t10_dn0: f64 = *var_t10_dn0_slot;
        let mut var_t10_dn1: f64 = *var_t10_dn1_slot;
        let mut var_t10_dn2: f64 = *var_t10_dn2_slot;
        let mut var_t10_dn3: f64 = *var_t10_dn3_slot;
        let mut var_t10_dn4: f64 = *var_t10_dn4_slot;
        let mut var_t10_dn5: f64 = *var_t10_dn5_slot;
        let mut var_t10_dn6: f64 = *var_t10_dn6_slot;
        let mut var_t10_dn7: f64 = *var_t10_dn7_slot;
        let mut var_t10_dn8: f64 = *var_t10_dn8_slot;
        let mut var_t10_rdb0: f64 = *var_t10_rdb0_slot;
        let mut var_t10_rdb1: f64 = *var_t10_rdb1_slot;
        let mut var_t10_rdb2: f64 = *var_t10_rdb2_slot;
        let mut var_t10_rdb3: f64 = *var_t10_rdb3_slot;
        let mut var_t10_rdb4: f64 = *var_t10_rdb4_slot;
        let mut var_t10_rdn0: f64 = *var_t10_rdn0_slot;
        let mut var_t10_rdn1: f64 = *var_t10_rdn1_slot;
        let mut var_t10_rdn2: f64 = *var_t10_rdn2_slot;
        let mut var_t10_rdn3: f64 = *var_t10_rdn3_slot;
        let mut var_t10_rdn4: f64 = *var_t10_rdn4_slot;
        let mut var_t10_rdn5: f64 = *var_t10_rdn5_slot;
        let mut var_t10_rdn6: f64 = *var_t10_rdn6_slot;
        let mut var_t10_rdn7: f64 = *var_t10_rdn7_slot;
        let mut var_t10_rdn8: f64 = *var_t10_rdn8_slot;
        let mut var_t10_rv: f64 = *var_t10_rv_slot;
        let mut var_t11: f64 = *var_t11_slot;
        let mut var_t11_db0: f64 = *var_t11_db0_slot;
        let mut var_t11_db1: f64 = *var_t11_db1_slot;
        let mut var_t11_db2: f64 = *var_t11_db2_slot;
        let mut var_t11_db3: f64 = *var_t11_db3_slot;
        let mut var_t11_db4: f64 = *var_t11_db4_slot;
        let mut var_t11_dn0: f64 = *var_t11_dn0_slot;
        let mut var_t11_dn1: f64 = *var_t11_dn1_slot;
        let mut var_t11_dn2: f64 = *var_t11_dn2_slot;
        let mut var_t11_dn3: f64 = *var_t11_dn3_slot;
        let mut var_t11_dn4: f64 = *var_t11_dn4_slot;
        let mut var_t11_dn5: f64 = *var_t11_dn5_slot;
        let mut var_t11_dn6: f64 = *var_t11_dn6_slot;
        let mut var_t11_dn7: f64 = *var_t11_dn7_slot;
        let mut var_t11_dn8: f64 = *var_t11_dn8_slot;
        let mut var_t11_rdb0: f64 = *var_t11_rdb0_slot;
        let mut var_t11_rdb1: f64 = *var_t11_rdb1_slot;
        let mut var_t11_rdb2: f64 = *var_t11_rdb2_slot;
        let mut var_t11_rdb3: f64 = *var_t11_rdb3_slot;
        let mut var_t11_rdb4: f64 = *var_t11_rdb4_slot;
        let mut var_t11_rdn0: f64 = *var_t11_rdn0_slot;
        let mut var_t11_rdn1: f64 = *var_t11_rdn1_slot;
        let mut var_t11_rdn2: f64 = *var_t11_rdn2_slot;
        let mut var_t11_rdn3: f64 = *var_t11_rdn3_slot;
        let mut var_t11_rdn4: f64 = *var_t11_rdn4_slot;
        let mut var_t11_rdn5: f64 = *var_t11_rdn5_slot;
        let mut var_t11_rdn6: f64 = *var_t11_rdn6_slot;
        let mut var_t11_rdn7: f64 = *var_t11_rdn7_slot;
        let mut var_t11_rdn8: f64 = *var_t11_rdn8_slot;
        let mut var_t11_rv: f64 = *var_t11_rv_slot;
        let mut var_t1_db0: f64 = *var_t1_db0_slot;
        let mut var_t1_db1: f64 = *var_t1_db1_slot;
        let mut var_t1_db2: f64 = *var_t1_db2_slot;
        let mut var_t1_db3: f64 = *var_t1_db3_slot;
        let mut var_t1_db4: f64 = *var_t1_db4_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rdb0: f64 = *var_t1_rdb0_slot;
        let mut var_t1_rdb1: f64 = *var_t1_rdb1_slot;
        let mut var_t1_rdb2: f64 = *var_t1_rdb2_slot;
        let mut var_t1_rdb3: f64 = *var_t1_rdb3_slot;
        let mut var_t1_rdb4: f64 = *var_t1_rdb4_slot;
        let mut var_t1_rdn0: f64 = *var_t1_rdn0_slot;
        let mut var_t1_rdn1: f64 = *var_t1_rdn1_slot;
        let mut var_t1_rdn2: f64 = *var_t1_rdn2_slot;
        let mut var_t1_rdn3: f64 = *var_t1_rdn3_slot;
        let mut var_t1_rdn4: f64 = *var_t1_rdn4_slot;
        let mut var_t1_rdn5: f64 = *var_t1_rdn5_slot;
        let mut var_t1_rdn6: f64 = *var_t1_rdn6_slot;
        let mut var_t1_rdn7: f64 = *var_t1_rdn7_slot;
        let mut var_t1_rdn8: f64 = *var_t1_rdn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_db0: f64 = *var_t7_db0_slot;
        let mut var_t7_db1: f64 = *var_t7_db1_slot;
        let mut var_t7_db2: f64 = *var_t7_db2_slot;
        let mut var_t7_db3: f64 = *var_t7_db3_slot;
        let mut var_t7_db4: f64 = *var_t7_db4_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn1: f64 = *var_t7_dn1_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn3: f64 = *var_t7_dn3_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_rdb0: f64 = *var_t7_rdb0_slot;
        let mut var_t7_rdb1: f64 = *var_t7_rdb1_slot;
        let mut var_t7_rdb2: f64 = *var_t7_rdb2_slot;
        let mut var_t7_rdb3: f64 = *var_t7_rdb3_slot;
        let mut var_t7_rdb4: f64 = *var_t7_rdb4_slot;
        let mut var_t7_rdn0: f64 = *var_t7_rdn0_slot;
        let mut var_t7_rdn1: f64 = *var_t7_rdn1_slot;
        let mut var_t7_rdn2: f64 = *var_t7_rdn2_slot;
        let mut var_t7_rdn3: f64 = *var_t7_rdn3_slot;
        let mut var_t7_rdn4: f64 = *var_t7_rdn4_slot;
        let mut var_t7_rdn5: f64 = *var_t7_rdn5_slot;
        let mut var_t7_rdn6: f64 = *var_t7_rdn6_slot;
        let mut var_t7_rdn7: f64 = *var_t7_rdn7_slot;
        let mut var_t7_rdn8: f64 = *var_t7_rdn8_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_db0: f64 = *var_t8_db0_slot;
        let mut var_t8_db1: f64 = *var_t8_db1_slot;
        let mut var_t8_db2: f64 = *var_t8_db2_slot;
        let mut var_t8_db3: f64 = *var_t8_db3_slot;
        let mut var_t8_db4: f64 = *var_t8_db4_slot;
        let mut var_t8_dn0: f64 = *var_t8_dn0_slot;
        let mut var_t8_dn1: f64 = *var_t8_dn1_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn3: f64 = *var_t8_dn3_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_rdb0: f64 = *var_t8_rdb0_slot;
        let mut var_t8_rdb1: f64 = *var_t8_rdb1_slot;
        let mut var_t8_rdb2: f64 = *var_t8_rdb2_slot;
        let mut var_t8_rdb3: f64 = *var_t8_rdb3_slot;
        let mut var_t8_rdb4: f64 = *var_t8_rdb4_slot;
        let mut var_t8_rdn0: f64 = *var_t8_rdn0_slot;
        let mut var_t8_rdn1: f64 = *var_t8_rdn1_slot;
        let mut var_t8_rdn2: f64 = *var_t8_rdn2_slot;
        let mut var_t8_rdn3: f64 = *var_t8_rdn3_slot;
        let mut var_t8_rdn4: f64 = *var_t8_rdn4_slot;
        let mut var_t8_rdn5: f64 = *var_t8_rdn5_slot;
        let mut var_t8_rdn6: f64 = *var_t8_rdn6_slot;
        let mut var_t8_rdn7: f64 = *var_t8_rdn7_slot;
        let mut var_t8_rdn8: f64 = *var_t8_rdn8_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;
        let mut var_t9: f64 = *var_t9_slot;
        let mut var_t9_db0: f64 = *var_t9_db0_slot;
        let mut var_t9_db1: f64 = *var_t9_db1_slot;
        let mut var_t9_db2: f64 = *var_t9_db2_slot;
        let mut var_t9_db3: f64 = *var_t9_db3_slot;
        let mut var_t9_db4: f64 = *var_t9_db4_slot;
        let mut var_t9_dn0: f64 = *var_t9_dn0_slot;
        let mut var_t9_dn1: f64 = *var_t9_dn1_slot;
        let mut var_t9_dn2: f64 = *var_t9_dn2_slot;
        let mut var_t9_dn3: f64 = *var_t9_dn3_slot;
        let mut var_t9_dn4: f64 = *var_t9_dn4_slot;
        let mut var_t9_dn5: f64 = *var_t9_dn5_slot;
        let mut var_t9_dn6: f64 = *var_t9_dn6_slot;
        let mut var_t9_dn7: f64 = *var_t9_dn7_slot;
        let mut var_t9_dn8: f64 = *var_t9_dn8_slot;
        let mut var_t9_rdb0: f64 = *var_t9_rdb0_slot;
        let mut var_t9_rdb1: f64 = *var_t9_rdb1_slot;
        let mut var_t9_rdb2: f64 = *var_t9_rdb2_slot;
        let mut var_t9_rdb3: f64 = *var_t9_rdb3_slot;
        let mut var_t9_rdb4: f64 = *var_t9_rdb4_slot;
        let mut var_t9_rdn0: f64 = *var_t9_rdn0_slot;
        let mut var_t9_rdn1: f64 = *var_t9_rdn1_slot;
        let mut var_t9_rdn2: f64 = *var_t9_rdn2_slot;
        let mut var_t9_rdn3: f64 = *var_t9_rdn3_slot;
        let mut var_t9_rdn4: f64 = *var_t9_rdn4_slot;
        let mut var_t9_rdn5: f64 = *var_t9_rdn5_slot;
        let mut var_t9_rdn6: f64 = *var_t9_rdn6_slot;
        let mut var_t9_rdn7: f64 = *var_t9_rdn7_slot;
        let mut var_t9_rdn8: f64 = *var_t9_rdn8_slot;
        let mut var_t9_rv: f64 = *var_t9_rv_slot;

        let (assign12950_e12040, assign12950_e12040_d_n0, assign12950_e12040_d_n1, assign12950_e12040_d_n2, assign12950_e12040_d_n3, assign12950_e12040_d_n4, assign12950_e12040_d_n5, assign12950_e12040_d_n6, assign12950_e12040_d_n7, assign12950_e12040_d_n8, assign12950_e12040_d_b0, assign12950_e12040_d_b1, assign12950_e12040_d_b2, assign12950_e12040_d_b3, assign12950_e12040_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12950_e12034: f64 = (10000000000.0 * var_leffnoisq);
        let assign12950_e12036: f64 = (assign12950_e12034 * var_weff);
        let assign12950_e12038: f64 = (assign12950_e12036 * p.p2);
        (assign12950_e12038, ((((10000000000.0 * var_leffnoisq_dn0) * var_weff) + (assign12950_e12034 * var_weff_dn0)) * p.p2), ((((10000000000.0 * var_leffnoisq_dn1) * var_weff) + (assign12950_e12034 * var_weff_dn1)) * p.p2), ((((10000000000.0 * var_leffnoisq_dn2) * var_weff) + (assign12950_e12034 * var_weff_dn2)) * p.p2), ((((10000000000.0 * var_leffnoisq_dn3) * var_weff) + (assign12950_e12034 * var_weff_dn3)) * p.p2), ((((10000000000.0 * var_leffnoisq_dn4) * var_weff) + (assign12950_e12034 * var_weff_dn4)) * p.p2), ((((10000000000.0 * var_leffnoisq_dn5) * var_weff) + (assign12950_e12034 * var_weff_dn5)) * p.p2), ((((10000000000.0 * var_leffnoisq_dn6) * var_weff) + (assign12950_e12034 * var_weff_dn6)) * p.p2), ((((10000000000.0 * var_leffnoisq_dn7) * var_weff) + (assign12950_e12034 * var_weff_dn7)) * p.p2), ((((10000000000.0 * var_leffnoisq_dn8) * var_weff) + (assign12950_e12034 * var_weff_dn8)) * p.p2), ((((10000000000.0 * var_leffnoisq_db0) * var_weff) + (assign12950_e12034 * var_weff_db0)) * p.p2), ((((10000000000.0 * var_leffnoisq_db1) * var_weff) + (assign12950_e12034 * var_weff_db1)) * p.p2), ((((10000000000.0 * var_leffnoisq_db2) * var_weff) + (assign12950_e12034 * var_weff_db2)) * p.p2), ((((10000000000.0 * var_leffnoisq_db3) * var_weff) + (assign12950_e12034 * var_weff_db3)) * p.p2), ((((10000000000.0 * var_leffnoisq_db4) * var_weff) + (assign12950_e12034 * var_weff_db4)) * p.p2),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn1, var_t7_dn2, var_t7_dn3, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_db0, var_t7_db1, var_t7_db2, var_t7_db3, var_t7_db4,)
    }
};
        var_t7 = assign12950_e12040;
        var_t7_dn0 = assign12950_e12040_d_n0;
        var_t7_dn1 = assign12950_e12040_d_n1;
        var_t7_dn2 = assign12950_e12040_d_n2;
        var_t7_dn3 = assign12950_e12040_d_n3;
        var_t7_dn4 = assign12950_e12040_d_n4;
        var_t7_dn5 = assign12950_e12040_d_n5;
        var_t7_dn6 = assign12950_e12040_d_n6;
        var_t7_dn7 = assign12950_e12040_d_n7;
        var_t7_dn8 = assign12950_e12040_d_n8;
        var_t7_db0 = assign12950_e12040_d_b0;
        var_t7_db1 = assign12950_e12040_d_b1;
        var_t7_db2 = assign12950_e12040_d_b2;
        var_t7_db3 = assign12950_e12040_d_b3;
        var_t7_db4 = assign12950_e12040_d_b4;
        var_t7_rv = 0.0;
        var_t7_rdn0 = 0.0;
        var_t7_rdn1 = 0.0;
        var_t7_rdn2 = 0.0;
        var_t7_rdn3 = 0.0;
        var_t7_rdn4 = 0.0;
        var_t7_rdn5 = 0.0;
        var_t7_rdn6 = 0.0;
        var_t7_rdn7 = 0.0;
        var_t7_rdn8 = 0.0;
        var_t7_rdb0 = 0.0;
        var_t7_rdb1 = 0.0;
        var_t7_rdb2 = 0.0;
        var_t7_rdb3 = 0.0;
        var_t7_rdb4 = 0.0;

        let (assign12960_e12054, assign12960_e12054_d_n0, assign12960_e12054_d_n1, assign12960_e12054_d_n2, assign12960_e12054_d_n3, assign12960_e12054_d_n4, assign12960_e12054_d_n5, assign12960_e12054_d_n6, assign12960_e12054_d_n7, assign12960_e12054_d_n8, assign12960_e12054_d_b0, assign12960_e12054_d_b1, assign12960_e12054_d_b2, assign12960_e12054_d_b3, assign12960_e12054_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12960_e12045: f64 = (p.p289 * var_nl);
        let assign12960_e12046: f64 = (var_noiaeff + assign12960_e12045);
        let assign12960_e12049: f64 = (p.p290 * var_nl);
        let assign12960_e12051: f64 = (assign12960_e12049 * var_nl);
        let assign12960_e12052: f64 = (assign12960_e12046 + assign12960_e12051);
        (assign12960_e12052, ((var_noiaeff_dn0 + (p.p289 * var_nl_dn0)) + (((p.p290 * var_nl_dn0) * var_nl) + (assign12960_e12049 * var_nl_dn0))), ((var_noiaeff_dn1 + (p.p289 * var_nl_dn1)) + (((p.p290 * var_nl_dn1) * var_nl) + (assign12960_e12049 * var_nl_dn1))), ((var_noiaeff_dn2 + (p.p289 * var_nl_dn2)) + (((p.p290 * var_nl_dn2) * var_nl) + (assign12960_e12049 * var_nl_dn2))), ((var_noiaeff_dn3 + (p.p289 * var_nl_dn3)) + (((p.p290 * var_nl_dn3) * var_nl) + (assign12960_e12049 * var_nl_dn3))), ((var_noiaeff_dn4 + (p.p289 * var_nl_dn4)) + (((p.p290 * var_nl_dn4) * var_nl) + (assign12960_e12049 * var_nl_dn4))), ((var_noiaeff_dn5 + (p.p289 * var_nl_dn5)) + (((p.p290 * var_nl_dn5) * var_nl) + (assign12960_e12049 * var_nl_dn5))), ((var_noiaeff_dn6 + (p.p289 * var_nl_dn6)) + (((p.p290 * var_nl_dn6) * var_nl) + (assign12960_e12049 * var_nl_dn6))), ((var_noiaeff_dn7 + (p.p289 * var_nl_dn7)) + (((p.p290 * var_nl_dn7) * var_nl) + (assign12960_e12049 * var_nl_dn7))), ((var_noiaeff_dn8 + (p.p289 * var_nl_dn8)) + (((p.p290 * var_nl_dn8) * var_nl) + (assign12960_e12049 * var_nl_dn8))), ((var_noiaeff_db0 + (p.p289 * var_nl_db0)) + (((p.p290 * var_nl_db0) * var_nl) + (assign12960_e12049 * var_nl_db0))), ((var_noiaeff_db1 + (p.p289 * var_nl_db1)) + (((p.p290 * var_nl_db1) * var_nl) + (assign12960_e12049 * var_nl_db1))), ((var_noiaeff_db2 + (p.p289 * var_nl_db2)) + (((p.p290 * var_nl_db2) * var_nl) + (assign12960_e12049 * var_nl_db2))), ((var_noiaeff_db3 + (p.p289 * var_nl_db3)) + (((p.p290 * var_nl_db3) * var_nl) + (assign12960_e12049 * var_nl_db3))), ((var_noiaeff_db4 + (p.p289 * var_nl_db4)) + (((p.p290 * var_nl_db4) * var_nl) + (assign12960_e12049 * var_nl_db4))),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn1, var_t8_dn2, var_t8_dn3, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_db0, var_t8_db1, var_t8_db2, var_t8_db3, var_t8_db4,)
    }
};
        var_t8 = assign12960_e12054;
        var_t8_dn0 = assign12960_e12054_d_n0;
        var_t8_dn1 = assign12960_e12054_d_n1;
        var_t8_dn2 = assign12960_e12054_d_n2;
        var_t8_dn3 = assign12960_e12054_d_n3;
        var_t8_dn4 = assign12960_e12054_d_n4;
        var_t8_dn5 = assign12960_e12054_d_n5;
        var_t8_dn6 = assign12960_e12054_d_n6;
        var_t8_dn7 = assign12960_e12054_d_n7;
        var_t8_dn8 = assign12960_e12054_d_n8;
        var_t8_db0 = assign12960_e12054_d_b0;
        var_t8_db1 = assign12960_e12054_d_b1;
        var_t8_db2 = assign12960_e12054_d_b2;
        var_t8_db3 = assign12960_e12054_d_b3;
        var_t8_db4 = assign12960_e12054_d_b4;
        var_t8_rv = 0.0;
        var_t8_rdn0 = 0.0;
        var_t8_rdn1 = 0.0;
        var_t8_rdn2 = 0.0;
        var_t8_rdn3 = 0.0;
        var_t8_rdn4 = 0.0;
        var_t8_rdn5 = 0.0;
        var_t8_rdn6 = 0.0;
        var_t8_rdn7 = 0.0;
        var_t8_rdn8 = 0.0;
        var_t8_rdb0 = 0.0;
        var_t8_rdb1 = 0.0;
        var_t8_rdb2 = 0.0;
        var_t8_rdb3 = 0.0;
        var_t8_rdb4 = 0.0;

        let (assign12970_e12064, assign12970_e12064_d_n0, assign12970_e12064_d_n1, assign12970_e12064_d_n2, assign12970_e12064_d_n3, assign12970_e12064_d_n4, assign12970_e12064_d_n5, assign12970_e12064_d_n6, assign12970_e12064_d_n7, assign12970_e12064_d_n8, assign12970_e12064_d_b0, assign12970_e12064_d_b1, assign12970_e12064_d_b2, assign12970_e12064_d_b3, assign12970_e12064_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12970_e12058: f64 = (var_nl + var_nstar);
        let assign12970_e12061: f64 = (var_nl + var_nstar);
        let assign12970_e12062: f64 = (assign12970_e12058 * assign12970_e12061);
        (assign12970_e12062, (((var_nl_dn0 + var_nstar_dn0) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn0 + var_nstar_dn0))), (((var_nl_dn1 + var_nstar_dn1) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn1 + var_nstar_dn1))), (((var_nl_dn2 + var_nstar_dn2) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn2 + var_nstar_dn2))), (((var_nl_dn3 + var_nstar_dn3) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn3 + var_nstar_dn3))), (((var_nl_dn4 + var_nstar_dn4) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn4 + var_nstar_dn4))), (((var_nl_dn5 + var_nstar_dn5) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn5 + var_nstar_dn5))), (((var_nl_dn6 + var_nstar_dn6) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn6 + var_nstar_dn6))), (((var_nl_dn7 + var_nstar_dn7) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn7 + var_nstar_dn7))), (((var_nl_dn8 + var_nstar_dn8) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn8 + var_nstar_dn8))), (((var_nl_db0 + var_nstar_db0) * assign12970_e12061) + (assign12970_e12058 * (var_nl_db0 + var_nstar_db0))), (((var_nl_db1 + var_nstar_db1) * assign12970_e12061) + (assign12970_e12058 * (var_nl_db1 + var_nstar_db1))), (((var_nl_db2 + var_nstar_db2) * assign12970_e12061) + (assign12970_e12058 * (var_nl_db2 + var_nstar_db2))), (((var_nl_db3 + var_nstar_db3) * assign12970_e12061) + (assign12970_e12058 * (var_nl_db3 + var_nstar_db3))), (((var_nl_db4 + var_nstar_db4) * assign12970_e12061) + (assign12970_e12058 * (var_nl_db4 + var_nstar_db4))),)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn1, var_t9_dn2, var_t9_dn3, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_db0, var_t9_db1, var_t9_db2, var_t9_db3, var_t9_db4,)
    }
};
        var_t9 = assign12970_e12064;
        var_t9_dn0 = assign12970_e12064_d_n0;
        var_t9_dn1 = assign12970_e12064_d_n1;
        var_t9_dn2 = assign12970_e12064_d_n2;
        var_t9_dn3 = assign12970_e12064_d_n3;
        var_t9_dn4 = assign12970_e12064_d_n4;
        var_t9_dn5 = assign12970_e12064_d_n5;
        var_t9_dn6 = assign12970_e12064_d_n6;
        var_t9_dn7 = assign12970_e12064_d_n7;
        var_t9_dn8 = assign12970_e12064_d_n8;
        var_t9_db0 = assign12970_e12064_d_b0;
        var_t9_db1 = assign12970_e12064_d_b1;
        var_t9_db2 = assign12970_e12064_d_b2;
        var_t9_db3 = assign12970_e12064_d_b3;
        var_t9_db4 = assign12970_e12064_d_b4;
        var_t9_rv = 0.0;
        var_t9_rdn0 = 0.0;
        var_t9_rdn1 = 0.0;
        var_t9_rdn2 = 0.0;
        var_t9_rdn3 = 0.0;
        var_t9_rdn4 = 0.0;
        var_t9_rdn5 = 0.0;
        var_t9_rdn6 = 0.0;
        var_t9_rdn7 = 0.0;
        var_t9_rdn8 = 0.0;
        var_t9_rdb0 = 0.0;
        var_t9_rdb1 = 0.0;
        var_t9_rdb2 = 0.0;
        var_t9_rdb3 = 0.0;
        var_t9_rdb4 = 0.0;

        let (assign12980_e12086, assign12980_e12086_d_n0, assign12980_e12086_d_n1, assign12980_e12086_d_n2, assign12980_e12086_d_n3, assign12980_e12086_d_n4, assign12980_e12086_d_n5, assign12980_e12086_d_n6, assign12980_e12086_d_n7, assign12980_e12086_d_n8, assign12980_e12086_d_b0, assign12980_e12086_d_b1, assign12980_e12086_d_b2, assign12980_e12086_d_b3, assign12980_e12086_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12980_e12068: f64 = (var_t1 / var_t2);
        let assign12980_e12071: f64 = (var_t3 + var_t4);
        let assign12980_e12073: f64 = (assign12980_e12071 + var_t5);
        let assign12980_e12074: f64 = (assign12980_e12068 * assign12980_e12073);
        let assign12980_e12077: f64 = (var_t6 / var_t7);
        let assign12980_e12079: f64 = (assign12980_e12077 * var_delclm);
        let assign12980_e12081: f64 = (assign12980_e12079 * var_t8);
        let assign12980_e12083: f64 = (assign12980_e12081 / var_t9);
        let assign12980_e12084: f64 = (assign12980_e12074 + assign12980_e12083);
        (assign12980_e12084, ((((((var_t1_dn0 * var_t2) - (var_t1 * var_t2_dn0)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn0 + var_t4_dn0) + var_t5_dn0))) + ((((((((((var_t6_dn0 * var_t7) - (var_t6 * var_t7_dn0)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn0)) * var_t8) + (assign12980_e12079 * var_t8_dn0)) * var_t9) - (assign12980_e12081 * var_t9_dn0)) / (var_t9 * var_t9))), ((((((var_t1_dn1 * var_t2) - (var_t1 * var_t2_dn1)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn1 + var_t4_dn1) + var_t5_dn1))) + ((((((((((var_t6_dn1 * var_t7) - (var_t6 * var_t7_dn1)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn1)) * var_t8) + (assign12980_e12079 * var_t8_dn1)) * var_t9) - (assign12980_e12081 * var_t9_dn1)) / (var_t9 * var_t9))), ((((((var_t1_dn2 * var_t2) - (var_t1 * var_t2_dn2)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn2 + var_t4_dn2) + var_t5_dn2))) + ((((((((((var_t6_dn2 * var_t7) - (var_t6 * var_t7_dn2)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn2)) * var_t8) + (assign12980_e12079 * var_t8_dn2)) * var_t9) - (assign12980_e12081 * var_t9_dn2)) / (var_t9 * var_t9))), ((((((var_t1_dn3 * var_t2) - (var_t1 * var_t2_dn3)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn3 + var_t4_dn3) + var_t5_dn3))) + ((((((((((var_t6_dn3 * var_t7) - (var_t6 * var_t7_dn3)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn3)) * var_t8) + (assign12980_e12079 * var_t8_dn3)) * var_t9) - (assign12980_e12081 * var_t9_dn3)) / (var_t9 * var_t9))), ((((((var_t1_dn4 * var_t2) - (var_t1 * var_t2_dn4)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn4 + var_t4_dn4) + var_t5_dn4))) + ((((((((((var_t6_dn4 * var_t7) - (var_t6 * var_t7_dn4)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn4)) * var_t8) + (assign12980_e12079 * var_t8_dn4)) * var_t9) - (assign12980_e12081 * var_t9_dn4)) / (var_t9 * var_t9))), ((((((var_t1_dn5 * var_t2) - (var_t1 * var_t2_dn5)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn5 + var_t4_dn5) + var_t5_dn5))) + ((((((((((var_t6_dn5 * var_t7) - (var_t6 * var_t7_dn5)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn5)) * var_t8) + (assign12980_e12079 * var_t8_dn5)) * var_t9) - (assign12980_e12081 * var_t9_dn5)) / (var_t9 * var_t9))), ((((((var_t1_dn6 * var_t2) - (var_t1 * var_t2_dn6)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn6 + var_t4_dn6) + var_t5_dn6))) + ((((((((((var_t6_dn6 * var_t7) - (var_t6 * var_t7_dn6)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn6)) * var_t8) + (assign12980_e12079 * var_t8_dn6)) * var_t9) - (assign12980_e12081 * var_t9_dn6)) / (var_t9 * var_t9))), ((((((var_t1_dn7 * var_t2) - (var_t1 * var_t2_dn7)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn7 + var_t4_dn7) + var_t5_dn7))) + ((((((((((var_t6_dn7 * var_t7) - (var_t6 * var_t7_dn7)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn7)) * var_t8) + (assign12980_e12079 * var_t8_dn7)) * var_t9) - (assign12980_e12081 * var_t9_dn7)) / (var_t9 * var_t9))), ((((((var_t1_dn8 * var_t2) - (var_t1 * var_t2_dn8)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn8 + var_t4_dn8) + var_t5_dn8))) + ((((((((((var_t6_dn8 * var_t7) - (var_t6 * var_t7_dn8)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn8)) * var_t8) + (assign12980_e12079 * var_t8_dn8)) * var_t9) - (assign12980_e12081 * var_t9_dn8)) / (var_t9 * var_t9))), ((((((var_t1_db0 * var_t2) - (var_t1 * var_t2_db0)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_db0 + var_t4_db0) + var_t5_db0))) + ((((((((((var_t6_db0 * var_t7) - (var_t6 * var_t7_db0)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_db0)) * var_t8) + (assign12980_e12079 * var_t8_db0)) * var_t9) - (assign12980_e12081 * var_t9_db0)) / (var_t9 * var_t9))), ((((((var_t1_db1 * var_t2) - (var_t1 * var_t2_db1)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_db1 + var_t4_db1) + var_t5_db1))) + ((((((((((var_t6_db1 * var_t7) - (var_t6 * var_t7_db1)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_db1)) * var_t8) + (assign12980_e12079 * var_t8_db1)) * var_t9) - (assign12980_e12081 * var_t9_db1)) / (var_t9 * var_t9))), ((((((var_t1_db2 * var_t2) - (var_t1 * var_t2_db2)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_db2 + var_t4_db2) + var_t5_db2))) + ((((((((((var_t6_db2 * var_t7) - (var_t6 * var_t7_db2)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_db2)) * var_t8) + (assign12980_e12079 * var_t8_db2)) * var_t9) - (assign12980_e12081 * var_t9_db2)) / (var_t9 * var_t9))), ((((((var_t1_db3 * var_t2) - (var_t1 * var_t2_db3)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_db3 + var_t4_db3) + var_t5_db3))) + ((((((((((var_t6_db3 * var_t7) - (var_t6 * var_t7_db3)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_db3)) * var_t8) + (assign12980_e12079 * var_t8_db3)) * var_t9) - (assign12980_e12081 * var_t9_db3)) / (var_t9 * var_t9))), ((((((var_t1_db4 * var_t2) - (var_t1 * var_t2_db4)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_db4 + var_t4_db4) + var_t5_db4))) + ((((((((((var_t6_db4 * var_t7) - (var_t6 * var_t7_db4)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_db4)) * var_t8) + (assign12980_e12079 * var_t8_db4)) * var_t9) - (assign12980_e12081 * var_t9_db4)) / (var_t9 * var_t9))),)
    } else {
        (var_ssi, var_ssi_dn0, var_ssi_dn1, var_ssi_dn2, var_ssi_dn3, var_ssi_dn4, var_ssi_dn5, var_ssi_dn6, var_ssi_dn7, var_ssi_dn8, var_ssi_db0, var_ssi_db1, var_ssi_db2, var_ssi_db3, var_ssi_db4,)
    }
};
        var_ssi = assign12980_e12086;
        var_ssi_dn0 = assign12980_e12086_d_n0;
        var_ssi_dn1 = assign12980_e12086_d_n1;
        var_ssi_dn2 = assign12980_e12086_d_n2;
        var_ssi_dn3 = assign12980_e12086_d_n3;
        var_ssi_dn4 = assign12980_e12086_d_n4;
        var_ssi_dn5 = assign12980_e12086_d_n5;
        var_ssi_dn6 = assign12980_e12086_d_n6;
        var_ssi_dn7 = assign12980_e12086_d_n7;
        var_ssi_dn8 = assign12980_e12086_d_n8;
        var_ssi_db0 = assign12980_e12086_d_b0;
        var_ssi_db1 = assign12980_e12086_d_b1;
        var_ssi_db2 = assign12980_e12086_d_b2;
        var_ssi_db3 = assign12980_e12086_d_b3;
        var_ssi_db4 = assign12980_e12086_d_b4;
        var_ssi_rv = 0.0;
        var_ssi_rdn0 = 0.0;
        var_ssi_rdn1 = 0.0;
        var_ssi_rdn2 = 0.0;
        var_ssi_rdn3 = 0.0;
        var_ssi_rdn4 = 0.0;
        var_ssi_rdn5 = 0.0;
        var_ssi_rdn6 = 0.0;
        var_ssi_rdn7 = 0.0;
        var_ssi_rdn8 = 0.0;
        var_ssi_rdb0 = 0.0;
        var_ssi_rdb1 = 0.0;
        var_ssi_rdb2 = 0.0;
        var_ssi_rdb3 = 0.0;
        var_ssi_rdb4 = 0.0;

        let (assign12990_e12094, assign12990_e12094_d_n0, assign12990_e12094_d_n1, assign12990_e12094_d_n2, assign12990_e12094_d_n3, assign12990_e12094_d_n4, assign12990_e12094_d_n5, assign12990_e12094_d_n6, assign12990_e12094_d_n7, assign12990_e12094_d_n8, assign12990_e12094_d_b0, assign12990_e12094_d_b1, assign12990_e12094_d_b2, assign12990_e12094_d_b3, assign12990_e12094_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign12990_e12090: f64 = (var_noiaeff * 1.60219e-19);
        let assign12990_e12092: f64 = (assign12990_e12090 * var_vtm);
        (assign12990_e12092, (((var_noiaeff_dn0 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_dn0)), (((var_noiaeff_dn1 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_dn1)), (((var_noiaeff_dn2 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_dn2)), (((var_noiaeff_dn3 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_dn3)), (((var_noiaeff_dn4 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_dn4)), (((var_noiaeff_dn5 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_dn5)), (((var_noiaeff_dn6 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_dn6)), (((var_noiaeff_dn7 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_dn7)), (((var_noiaeff_dn8 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_dn8)), (((var_noiaeff_db0 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_db0)), (((var_noiaeff_db1 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_db1)), (((var_noiaeff_db2 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_db2)), (((var_noiaeff_db3 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_db3)), (((var_noiaeff_db4 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_db4)),)
    } else {
        (var_t10, var_t10_dn0, var_t10_dn1, var_t10_dn2, var_t10_dn3, var_t10_dn4, var_t10_dn5, var_t10_dn6, var_t10_dn7, var_t10_dn8, var_t10_db0, var_t10_db1, var_t10_db2, var_t10_db3, var_t10_db4,)
    }
};
        var_t10 = assign12990_e12094;
        var_t10_dn0 = assign12990_e12094_d_n0;
        var_t10_dn1 = assign12990_e12094_d_n1;
        var_t10_dn2 = assign12990_e12094_d_n2;
        var_t10_dn3 = assign12990_e12094_d_n3;
        var_t10_dn4 = assign12990_e12094_d_n4;
        var_t10_dn5 = assign12990_e12094_d_n5;
        var_t10_dn6 = assign12990_e12094_d_n6;
        var_t10_dn7 = assign12990_e12094_d_n7;
        var_t10_dn8 = assign12990_e12094_d_n8;
        var_t10_db0 = assign12990_e12094_d_b0;
        var_t10_db1 = assign12990_e12094_d_b1;
        var_t10_db2 = assign12990_e12094_d_b2;
        var_t10_db3 = assign12990_e12094_d_b3;
        var_t10_db4 = assign12990_e12094_d_b4;
        var_t10_rv = 0.0;
        var_t10_rdn0 = 0.0;
        var_t10_rdn1 = 0.0;
        var_t10_rdn2 = 0.0;
        var_t10_rdn3 = 0.0;
        var_t10_rdn4 = 0.0;
        var_t10_rdn5 = 0.0;
        var_t10_rdn6 = 0.0;
        var_t10_rdn7 = 0.0;
        var_t10_rdn8 = 0.0;
        var_t10_rdb0 = 0.0;
        var_t10_rdb1 = 0.0;
        var_t10_rdb2 = 0.0;
        var_t10_rdb3 = 0.0;
        var_t10_rdb4 = 0.0;

        let (assign13000_e12108, assign13000_e12108_d_n0, assign13000_e12108_d_n1, assign13000_e12108_d_n2, assign13000_e12108_d_n3, assign13000_e12108_d_n4, assign13000_e12108_d_n5, assign13000_e12108_d_n6, assign13000_e12108_d_n7, assign13000_e12108_d_n8, assign13000_e12108_d_b0, assign13000_e12108_d_b1, assign13000_e12108_d_b2, assign13000_e12108_d_b3, assign13000_e12108_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign13000_e12098: f64 = (var_weff * p.p2);
        let assign13000_e12100: f64 = (assign13000_e12098 * var_leffnoi);
        let assign13000_e12102: f64 = (assign13000_e12100 * 10000000000.0);
        let assign13000_e12104: f64 = (assign13000_e12102 * var_nstar);
        let assign13000_e12106: f64 = (assign13000_e12104 * var_nstar);
        (assign13000_e12106, ((((((((var_weff_dn0 * p.p2) * var_leffnoi) + (assign13000_e12098 * var_leffnoi_dn0)) * 10000000000.0) * var_nstar) + (assign13000_e12102 * var_nstar_dn0)) * var_nstar) + (assign13000_e12104 * var_nstar_dn0)), ((((((((var_weff_dn1 * p.p2) * var_leffnoi) + (assign13000_e12098 * var_leffnoi_dn1)) * 10000000000.0) * var_nstar) + (assign13000_e12102 * var_nstar_dn1)) * var_nstar) + (assign13000_e12104 * var_nstar_dn1)), ((((((((var_weff_dn2 * p.p2) * var_leffnoi) + (assign13000_e12098 * var_leffnoi_dn2)) * 10000000000.0) * var_nstar) + (assign13000_e12102 * var_nstar_dn2)) * var_nstar) + (assign13000_e12104 * var_nstar_dn2)), ((((((((var_weff_dn3 * p.p2) * var_leffnoi) + (assign13000_e12098 * var_leffnoi_dn3)) * 10000000000.0) * var_nstar) + (assign13000_e12102 * var_nstar_dn3)) * var_nstar) + (assign13000_e12104 * var_nstar_dn3)), ((((((((var_weff_dn4 * p.p2) * var_leffnoi) + (assign13000_e12098 * var_leffnoi_dn4)) * 10000000000.0) * var_nstar) + (assign13000_e12102 * var_nstar_dn4)) * var_nstar) + (assign13000_e12104 * var_nstar_dn4)), ((((((((var_weff_dn5 * p.p2) * var_leffnoi) + (assign13000_e12098 * var_leffnoi_dn5)) * 10000000000.0) * var_nstar) + (assign13000_e12102 * var_nstar_dn5)) * var_nstar) + (assign13000_e12104 * var_nstar_dn5)), ((((((((var_weff_dn6 * p.p2) * var_leffnoi) + (assign13000_e12098 * var_leffnoi_dn6)) * 10000000000.0) * var_nstar) + (assign13000_e12102 * var_nstar_dn6)) * var_nstar) + (assign13000_e12104 * var_nstar_dn6)), ((((((((var_weff_dn7 * p.p2) * var_leffnoi) + (assign13000_e12098 * var_leffnoi_dn7)) * 10000000000.0) * var_nstar) + (assign13000_e12102 * var_nstar_dn7)) * var_nstar) + (assign13000_e12104 * var_nstar_dn7)), ((((((((var_weff_dn8 * p.p2) * var_leffnoi) + (assign13000_e12098 * var_leffnoi_dn8)) * 10000000000.0) * var_nstar) + (assign13000_e12102 * var_nstar_dn8)) * var_nstar) + (assign13000_e12104 * var_nstar_dn8)), ((((((((var_weff_db0 * p.p2) * var_leffnoi) + (assign13000_e12098 * var_leffnoi_db0)) * 10000000000.0) * var_nstar) + (assign13000_e12102 * var_nstar_db0)) * var_nstar) + (assign13000_e12104 * var_nstar_db0)), ((((((((var_weff_db1 * p.p2) * var_leffnoi) + (assign13000_e12098 * var_leffnoi_db1)) * 10000000000.0) * var_nstar) + (assign13000_e12102 * var_nstar_db1)) * var_nstar) + (assign13000_e12104 * var_nstar_db1)), ((((((((var_weff_db2 * p.p2) * var_leffnoi) + (assign13000_e12098 * var_leffnoi_db2)) * 10000000000.0) * var_nstar) + (assign13000_e12102 * var_nstar_db2)) * var_nstar) + (assign13000_e12104 * var_nstar_db2)), ((((((((var_weff_db3 * p.p2) * var_leffnoi) + (assign13000_e12098 * var_leffnoi_db3)) * 10000000000.0) * var_nstar) + (assign13000_e12102 * var_nstar_db3)) * var_nstar) + (assign13000_e12104 * var_nstar_db3)), ((((((((var_weff_db4 * p.p2) * var_leffnoi) + (assign13000_e12098 * var_leffnoi_db4)) * 10000000000.0) * var_nstar) + (assign13000_e12102 * var_nstar_db4)) * var_nstar) + (assign13000_e12104 * var_nstar_db4)),)
    } else {
        (var_t11, var_t11_dn0, var_t11_dn1, var_t11_dn2, var_t11_dn3, var_t11_dn4, var_t11_dn5, var_t11_dn6, var_t11_dn7, var_t11_dn8, var_t11_db0, var_t11_db1, var_t11_db2, var_t11_db3, var_t11_db4,)
    }
};
        var_t11 = assign13000_e12108;
        var_t11_dn0 = assign13000_e12108_d_n0;
        var_t11_dn1 = assign13000_e12108_d_n1;
        var_t11_dn2 = assign13000_e12108_d_n2;
        var_t11_dn3 = assign13000_e12108_d_n3;
        var_t11_dn4 = assign13000_e12108_d_n4;
        var_t11_dn5 = assign13000_e12108_d_n5;
        var_t11_dn6 = assign13000_e12108_d_n6;
        var_t11_dn7 = assign13000_e12108_d_n7;
        var_t11_dn8 = assign13000_e12108_d_n8;
        var_t11_db0 = assign13000_e12108_d_b0;
        var_t11_db1 = assign13000_e12108_d_b1;
        var_t11_db2 = assign13000_e12108_d_b2;
        var_t11_db3 = assign13000_e12108_d_b3;
        var_t11_db4 = assign13000_e12108_d_b4;
        var_t11_rv = 0.0;
        var_t11_rdn0 = 0.0;
        var_t11_rdn1 = 0.0;
        var_t11_rdn2 = 0.0;
        var_t11_rdn3 = 0.0;
        var_t11_rdn4 = 0.0;
        var_t11_rdn5 = 0.0;
        var_t11_rdn6 = 0.0;
        var_t11_rdn7 = 0.0;
        var_t11_rdn8 = 0.0;
        var_t11_rdb0 = 0.0;
        var_t11_rdb1 = 0.0;
        var_t11_rdb2 = 0.0;
        var_t11_rdb3 = 0.0;
        var_t11_rdb4 = 0.0;

        let (assign13010_e12118, assign13010_e12118_d_n0, assign13010_e12118_d_n1, assign13010_e12118_d_n2, assign13010_e12118_d_n3, assign13010_e12118_d_n4, assign13010_e12118_d_n5, assign13010_e12118_d_n6, assign13010_e12118_d_n7, assign13010_e12118_d_n8, assign13010_e12118_d_b0, assign13010_e12118_d_b1, assign13010_e12118_d_b2, assign13010_e12118_d_b3, assign13010_e12118_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign13010_e12112: f64 = (var_t10 / var_t11);
        let assign13010_e12114: f64 = (assign13010_e12112 * var_ids);
        let assign13010_e12116: f64 = (assign13010_e12114 * var_ids);
        (assign13010_e12116, (((((((var_t10_dn0 * var_t11) - (var_t10 * var_t11_dn0)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn0)) * var_ids) + (assign13010_e12114 * var_ids_dn0)), (((((((var_t10_dn1 * var_t11) - (var_t10 * var_t11_dn1)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn1)) * var_ids) + (assign13010_e12114 * var_ids_dn1)), (((((((var_t10_dn2 * var_t11) - (var_t10 * var_t11_dn2)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn2)) * var_ids) + (assign13010_e12114 * var_ids_dn2)), (((((((var_t10_dn3 * var_t11) - (var_t10 * var_t11_dn3)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn3)) * var_ids) + (assign13010_e12114 * var_ids_dn3)), (((((((var_t10_dn4 * var_t11) - (var_t10 * var_t11_dn4)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn4)) * var_ids) + (assign13010_e12114 * var_ids_dn4)), (((((((var_t10_dn5 * var_t11) - (var_t10 * var_t11_dn5)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn5)) * var_ids) + (assign13010_e12114 * var_ids_dn5)), (((((((var_t10_dn6 * var_t11) - (var_t10 * var_t11_dn6)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn6)) * var_ids) + (assign13010_e12114 * var_ids_dn6)), (((((((var_t10_dn7 * var_t11) - (var_t10 * var_t11_dn7)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn7)) * var_ids) + (assign13010_e12114 * var_ids_dn7)), (((((((var_t10_dn8 * var_t11) - (var_t10 * var_t11_dn8)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn8)) * var_ids) + (assign13010_e12114 * var_ids_dn8)), (((((((var_t10_db0 * var_t11) - (var_t10 * var_t11_db0)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_db0)) * var_ids) + (assign13010_e12114 * var_ids_db0)), (((((((var_t10_db1 * var_t11) - (var_t10 * var_t11_db1)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_db1)) * var_ids) + (assign13010_e12114 * var_ids_db1)), (((((((var_t10_db2 * var_t11) - (var_t10 * var_t11_db2)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_db2)) * var_ids) + (assign13010_e12114 * var_ids_db2)), (((((((var_t10_db3 * var_t11) - (var_t10 * var_t11_db3)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_db3)) * var_ids) + (assign13010_e12114 * var_ids_db3)), (((((((var_t10_db4 * var_t11) - (var_t10 * var_t11_db4)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_db4)) * var_ids) + (assign13010_e12114 * var_ids_db4)),)
    } else {
        (var_swi, var_swi_dn0, var_swi_dn1, var_swi_dn2, var_swi_dn3, var_swi_dn4, var_swi_dn5, var_swi_dn6, var_swi_dn7, var_swi_dn8, var_swi_db0, var_swi_db1, var_swi_db2, var_swi_db3, var_swi_db4,)
    }
};
        var_swi = assign13010_e12118;
        var_swi_dn0 = assign13010_e12118_d_n0;
        var_swi_dn1 = assign13010_e12118_d_n1;
        var_swi_dn2 = assign13010_e12118_d_n2;
        var_swi_dn3 = assign13010_e12118_d_n3;
        var_swi_dn4 = assign13010_e12118_d_n4;
        var_swi_dn5 = assign13010_e12118_d_n5;
        var_swi_dn6 = assign13010_e12118_d_n6;
        var_swi_dn7 = assign13010_e12118_d_n7;
        var_swi_dn8 = assign13010_e12118_d_n8;
        var_swi_db0 = assign13010_e12118_d_b0;
        var_swi_db1 = assign13010_e12118_d_b1;
        var_swi_db2 = assign13010_e12118_d_b2;
        var_swi_db3 = assign13010_e12118_d_b3;
        var_swi_db4 = assign13010_e12118_d_b4;
        var_swi_rv = 0.0;
        var_swi_rdn0 = 0.0;
        var_swi_rdn1 = 0.0;
        var_swi_rdn2 = 0.0;
        var_swi_rdn3 = 0.0;
        var_swi_rdn4 = 0.0;
        var_swi_rdn5 = 0.0;
        var_swi_rdn6 = 0.0;
        var_swi_rdn7 = 0.0;
        var_swi_rdn8 = 0.0;
        var_swi_rdb0 = 0.0;
        var_swi_rdb1 = 0.0;
        var_swi_rdb2 = 0.0;
        var_swi_rdb3 = 0.0;
        var_swi_rdb4 = 0.0;

        let (assign13020_e12124, assign13020_e12124_d_n0, assign13020_e12124_d_n1, assign13020_e12124_d_n2, assign13020_e12124_d_n3, assign13020_e12124_d_n4, assign13020_e12124_d_n5, assign13020_e12124_d_n6, assign13020_e12124_d_n7, assign13020_e12124_d_n8, assign13020_e12124_d_b0, assign13020_e12124_d_b1, assign13020_e12124_d_b2, assign13020_e12124_d_b3, assign13020_e12124_d_b4,) = {
    if (var_guard131 != 0.0) {
        let assign13020_e12122: f64 = (var_swi + var_ssi);
        (assign13020_e12122, (var_swi_dn0 + var_ssi_dn0), (var_swi_dn1 + var_ssi_dn1), (var_swi_dn2 + var_ssi_dn2), (var_swi_dn3 + var_ssi_dn3), (var_swi_dn4 + var_ssi_dn4), (var_swi_dn5 + var_ssi_dn5), (var_swi_dn6 + var_ssi_dn6), (var_swi_dn7 + var_ssi_dn7), (var_swi_dn8 + var_ssi_dn8), (var_swi_db0 + var_ssi_db0), (var_swi_db1 + var_ssi_db1), (var_swi_db2 + var_ssi_db2), (var_swi_db3 + var_ssi_db3), (var_swi_db4 + var_ssi_db4),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4,)
    }
};
        var_t1 = assign13020_e12124;
        var_t1_dn0 = assign13020_e12124_d_n0;
        var_t1_dn1 = assign13020_e12124_d_n1;
        var_t1_dn2 = assign13020_e12124_d_n2;
        var_t1_dn3 = assign13020_e12124_d_n3;
        var_t1_dn4 = assign13020_e12124_d_n4;
        var_t1_dn5 = assign13020_e12124_d_n5;
        var_t1_dn6 = assign13020_e12124_d_n6;
        var_t1_dn7 = assign13020_e12124_d_n7;
        var_t1_dn8 = assign13020_e12124_d_n8;
        var_t1_db0 = assign13020_e12124_d_b0;
        var_t1_db1 = assign13020_e12124_d_b1;
        var_t1_db2 = assign13020_e12124_d_b2;
        var_t1_db3 = assign13020_e12124_d_b3;
        var_t1_db4 = assign13020_e12124_d_b4;
        var_t1_rv = 0.0;
        var_t1_rdn0 = 0.0;
        var_t1_rdn1 = 0.0;
        var_t1_rdn2 = 0.0;
        var_t1_rdn3 = 0.0;
        var_t1_rdn4 = 0.0;
        var_t1_rdn5 = 0.0;
        var_t1_rdn6 = 0.0;
        var_t1_rdn7 = 0.0;
        var_t1_rdn8 = 0.0;
        var_t1_rdb0 = 0.0;
        var_t1_rdb1 = 0.0;
        var_t1_rdb2 = 0.0;
        var_t1_rdb3 = 0.0;
        var_t1_rdb4 = 0.0;

        let assign13070_e12152: f64 = (var_devsign * p.p2);
        let assign13070_e12154: f64 = (assign13070_e12152 * var_qfg);
        var_qfgi = assign13070_e12154;
        var_qfgi_dn0 = (assign13070_e12152 * var_qfg_dn0);
        var_qfgi_dn1 = (assign13070_e12152 * var_qfg_dn1);
        var_qfgi_dn2 = (assign13070_e12152 * var_qfg_dn2);
        var_qfgi_dn3 = (assign13070_e12152 * var_qfg_dn3);
        var_qfgi_dn4 = (assign13070_e12152 * var_qfg_dn4);
        var_qfgi_dn5 = (assign13070_e12152 * var_qfg_dn5);
        var_qfgi_dn6 = (assign13070_e12152 * var_qfg_dn6);
        var_qfgi_dn7 = (assign13070_e12152 * var_qfg_dn7);
        var_qfgi_dn8 = (assign13070_e12152 * var_qfg_dn8);
        var_qfgi_db0 = (assign13070_e12152 * var_qfg_db0);
        var_qfgi_db1 = (assign13070_e12152 * var_qfg_db1);
        var_qfgi_db2 = (assign13070_e12152 * var_qfg_db2);
        var_qfgi_db3 = (assign13070_e12152 * var_qfg_db3);
        var_qfgi_db4 = (assign13070_e12152 * var_qfg_db4);
        var_qfgi_rv = 0.0;
        var_qfgi_rdn0 = 0.0;
        var_qfgi_rdn1 = 0.0;
        var_qfgi_rdn2 = 0.0;
        var_qfgi_rdn3 = 0.0;
        var_qfgi_rdn4 = 0.0;
        var_qfgi_rdn5 = 0.0;
        var_qfgi_rdn6 = 0.0;
        var_qfgi_rdn7 = 0.0;
        var_qfgi_rdn8 = 0.0;
        var_qfgi_rdb0 = 0.0;
        var_qfgi_rdb1 = 0.0;
        var_qfgi_rdb2 = 0.0;
        var_qfgi_rdb3 = 0.0;
        var_qfgi_rdb4 = 0.0;

        let assign13080_e12157: f64 = (p.p2 * var_qbg);
        var_qbgi = assign13080_e12157;
        var_qbgi_dn0 = (p.p2 * var_qbg_dn0);
        var_qbgi_dn1 = (p.p2 * var_qbg_dn1);
        var_qbgi_dn2 = (p.p2 * var_qbg_dn2);
        var_qbgi_dn3 = (p.p2 * var_qbg_dn3);
        var_qbgi_dn4 = (p.p2 * var_qbg_dn4);
        var_qbgi_dn5 = (p.p2 * var_qbg_dn5);
        var_qbgi_dn6 = (p.p2 * var_qbg_dn6);
        var_qbgi_dn7 = (p.p2 * var_qbg_dn7);
        var_qbgi_dn8 = (p.p2 * var_qbg_dn8);
        var_qbgi_db0 = (p.p2 * var_qbg_db0);
        var_qbgi_db1 = (p.p2 * var_qbg_db1);
        var_qbgi_db2 = (p.p2 * var_qbg_db2);
        var_qbgi_db3 = (p.p2 * var_qbg_db3);
        var_qbgi_db4 = (p.p2 * var_qbg_db4);
        var_qbgi_rv = 0.0;
        var_qbgi_rdn0 = 0.0;
        var_qbgi_rdn1 = 0.0;
        var_qbgi_rdn2 = 0.0;
        var_qbgi_rdn3 = 0.0;
        var_qbgi_rdn4 = 0.0;
        var_qbgi_rdn5 = 0.0;
        var_qbgi_rdn6 = 0.0;
        var_qbgi_rdn7 = 0.0;
        var_qbgi_rdn8 = 0.0;
        var_qbgi_rdb0 = 0.0;
        var_qbgi_rdb1 = 0.0;
        var_qbgi_rdb2 = 0.0;
        var_qbgi_rdb3 = 0.0;
        var_qbgi_rdb4 = 0.0;

        let assign13090_e12160: f64 = if var_sigvds > 0.0 { 1.0 } else { 0.0 };
        var_guard136 = assign13090_e12160;
        var_guard136_dn0 = 0.0;
        var_guard136_dn1 = 0.0;
        var_guard136_dn2 = 0.0;
        var_guard136_dn3 = 0.0;
        var_guard136_dn4 = 0.0;
        var_guard136_dn5 = 0.0;
        var_guard136_dn6 = 0.0;
        var_guard136_dn7 = 0.0;
        var_guard136_dn8 = 0.0;
        var_guard136_db0 = 0.0;
        var_guard136_db1 = 0.0;
        var_guard136_db2 = 0.0;
        var_guard136_db3 = 0.0;
        var_guard136_db4 = 0.0;
        var_guard136_rv = 0.0;
        var_guard136_rdn0 = 0.0;
        var_guard136_rdn1 = 0.0;
        var_guard136_rdn2 = 0.0;
        var_guard136_rdn3 = 0.0;
        var_guard136_rdn4 = 0.0;
        var_guard136_rdn5 = 0.0;
        var_guard136_rdn6 = 0.0;
        var_guard136_rdn7 = 0.0;
        var_guard136_rdn8 = 0.0;
        var_guard136_rdb0 = 0.0;
        var_guard136_rdb1 = 0.0;
        var_guard136_rdb2 = 0.0;
        var_guard136_rdb3 = 0.0;
        var_guard136_rdb4 = 0.0;

        let (assign13100_e12166, assign13100_e12166_d_n0, assign13100_e12166_d_n1, assign13100_e12166_d_n2, assign13100_e12166_d_n3, assign13100_e12166_d_n4, assign13100_e12166_d_n5, assign13100_e12166_d_n6, assign13100_e12166_d_n7, assign13100_e12166_d_n8, assign13100_e12166_d_b0, assign13100_e12166_d_b1, assign13100_e12166_d_b2, assign13100_e12166_d_b3, assign13100_e12166_d_b4,) = {
    if (var_guard136 != 0.0) {
        let assign13100_e12164: f64 = (p.p2 * var_qs);
        (assign13100_e12164, (p.p2 * var_qs_dn0), (p.p2 * var_qs_dn1), (p.p2 * var_qs_dn2), (p.p2 * var_qs_dn3), (p.p2 * var_qs_dn4), (p.p2 * var_qs_dn5), (p.p2 * var_qs_dn6), (p.p2 * var_qs_dn7), (p.p2 * var_qs_dn8), (p.p2 * var_qs_db0), (p.p2 * var_qs_db1), (p.p2 * var_qs_db2), (p.p2 * var_qs_db3), (p.p2 * var_qs_db4),)
    } else {
        (var_qsi, var_qsi_dn0, var_qsi_dn1, var_qsi_dn2, var_qsi_dn3, var_qsi_dn4, var_qsi_dn5, var_qsi_dn6, var_qsi_dn7, var_qsi_dn8, var_qsi_db0, var_qsi_db1, var_qsi_db2, var_qsi_db3, var_qsi_db4,)
    }
};
        var_qsi = assign13100_e12166;
        var_qsi_dn0 = assign13100_e12166_d_n0;
        var_qsi_dn1 = assign13100_e12166_d_n1;
        var_qsi_dn2 = assign13100_e12166_d_n2;
        var_qsi_dn3 = assign13100_e12166_d_n3;
        var_qsi_dn4 = assign13100_e12166_d_n4;
        var_qsi_dn5 = assign13100_e12166_d_n5;
        var_qsi_dn6 = assign13100_e12166_d_n6;
        var_qsi_dn7 = assign13100_e12166_d_n7;
        var_qsi_dn8 = assign13100_e12166_d_n8;
        var_qsi_db0 = assign13100_e12166_d_b0;
        var_qsi_db1 = assign13100_e12166_d_b1;
        var_qsi_db2 = assign13100_e12166_d_b2;
        var_qsi_db3 = assign13100_e12166_d_b3;
        var_qsi_db4 = assign13100_e12166_d_b4;
        var_qsi_rv = 0.0;
        var_qsi_rdn0 = 0.0;
        var_qsi_rdn1 = 0.0;
        var_qsi_rdn2 = 0.0;
        var_qsi_rdn3 = 0.0;
        var_qsi_rdn4 = 0.0;
        var_qsi_rdn5 = 0.0;
        var_qsi_rdn6 = 0.0;
        var_qsi_rdn7 = 0.0;
        var_qsi_rdn8 = 0.0;
        var_qsi_rdb0 = 0.0;
        var_qsi_rdb1 = 0.0;
        var_qsi_rdb2 = 0.0;
        var_qsi_rdb3 = 0.0;
        var_qsi_rdb4 = 0.0;

        let (assign13110_e12172, assign13110_e12172_d_n0, assign13110_e12172_d_n1, assign13110_e12172_d_n2, assign13110_e12172_d_n3, assign13110_e12172_d_n4, assign13110_e12172_d_n5, assign13110_e12172_d_n6, assign13110_e12172_d_n7, assign13110_e12172_d_n8, assign13110_e12172_d_b0, assign13110_e12172_d_b1, assign13110_e12172_d_b2, assign13110_e12172_d_b3, assign13110_e12172_d_b4,) = {
    if (var_guard136 != 0.0) {
        let assign13110_e12170: f64 = (p.p2 * var_qd);
        (assign13110_e12170, (p.p2 * var_qd_dn0), (p.p2 * var_qd_dn1), (p.p2 * var_qd_dn2), (p.p2 * var_qd_dn3), (p.p2 * var_qd_dn4), (p.p2 * var_qd_dn5), (p.p2 * var_qd_dn6), (p.p2 * var_qd_dn7), (p.p2 * var_qd_dn8), (p.p2 * var_qd_db0), (p.p2 * var_qd_db1), (p.p2 * var_qd_db2), (p.p2 * var_qd_db3), (p.p2 * var_qd_db4),)
    } else {
        (var_qdi, var_qdi_dn0, var_qdi_dn1, var_qdi_dn2, var_qdi_dn3, var_qdi_dn4, var_qdi_dn5, var_qdi_dn6, var_qdi_dn7, var_qdi_dn8, var_qdi_db0, var_qdi_db1, var_qdi_db2, var_qdi_db3, var_qdi_db4,)
    }
};
        var_qdi = assign13110_e12172;
        var_qdi_dn0 = assign13110_e12172_d_n0;
        var_qdi_dn1 = assign13110_e12172_d_n1;
        var_qdi_dn2 = assign13110_e12172_d_n2;
        var_qdi_dn3 = assign13110_e12172_d_n3;
        var_qdi_dn4 = assign13110_e12172_d_n4;
        var_qdi_dn5 = assign13110_e12172_d_n5;
        var_qdi_dn6 = assign13110_e12172_d_n6;
        var_qdi_dn7 = assign13110_e12172_d_n7;
        var_qdi_dn8 = assign13110_e12172_d_n8;
        var_qdi_db0 = assign13110_e12172_d_b0;
        var_qdi_db1 = assign13110_e12172_d_b1;
        var_qdi_db2 = assign13110_e12172_d_b2;
        var_qdi_db3 = assign13110_e12172_d_b3;
        var_qdi_db4 = assign13110_e12172_d_b4;
        var_qdi_rv = 0.0;
        var_qdi_rdn0 = 0.0;
        var_qdi_rdn1 = 0.0;
        var_qdi_rdn2 = 0.0;
        var_qdi_rdn3 = 0.0;
        var_qdi_rdn4 = 0.0;
        var_qdi_rdn5 = 0.0;
        var_qdi_rdn6 = 0.0;
        var_qdi_rdn7 = 0.0;
        var_qdi_rdn8 = 0.0;
        var_qdi_rdb0 = 0.0;
        var_qdi_rdb1 = 0.0;
        var_qdi_rdb2 = 0.0;
        var_qdi_rdb3 = 0.0;
        var_qdi_rdb4 = 0.0;


        *var_guard136_slot = var_guard136;
        *var_guard136_db0_slot = var_guard136_db0;
        *var_guard136_db1_slot = var_guard136_db1;
        *var_guard136_db2_slot = var_guard136_db2;
        *var_guard136_db3_slot = var_guard136_db3;
        *var_guard136_db4_slot = var_guard136_db4;
        *var_guard136_dn0_slot = var_guard136_dn0;
        *var_guard136_dn1_slot = var_guard136_dn1;
        *var_guard136_dn2_slot = var_guard136_dn2;
        *var_guard136_dn3_slot = var_guard136_dn3;
        *var_guard136_dn4_slot = var_guard136_dn4;
        *var_guard136_dn5_slot = var_guard136_dn5;
        *var_guard136_dn6_slot = var_guard136_dn6;
        *var_guard136_dn7_slot = var_guard136_dn7;
        *var_guard136_dn8_slot = var_guard136_dn8;
        *var_guard136_rdb0_slot = var_guard136_rdb0;
        *var_guard136_rdb1_slot = var_guard136_rdb1;
        *var_guard136_rdb2_slot = var_guard136_rdb2;
        *var_guard136_rdb3_slot = var_guard136_rdb3;
        *var_guard136_rdb4_slot = var_guard136_rdb4;
        *var_guard136_rdn0_slot = var_guard136_rdn0;
        *var_guard136_rdn1_slot = var_guard136_rdn1;
        *var_guard136_rdn2_slot = var_guard136_rdn2;
        *var_guard136_rdn3_slot = var_guard136_rdn3;
        *var_guard136_rdn4_slot = var_guard136_rdn4;
        *var_guard136_rdn5_slot = var_guard136_rdn5;
        *var_guard136_rdn6_slot = var_guard136_rdn6;
        *var_guard136_rdn7_slot = var_guard136_rdn7;
        *var_guard136_rdn8_slot = var_guard136_rdn8;
        *var_guard136_rv_slot = var_guard136_rv;
        *var_qbgi_slot = var_qbgi;
        *var_qbgi_db0_slot = var_qbgi_db0;
        *var_qbgi_db1_slot = var_qbgi_db1;
        *var_qbgi_db2_slot = var_qbgi_db2;
        *var_qbgi_db3_slot = var_qbgi_db3;
        *var_qbgi_db4_slot = var_qbgi_db4;
        *var_qbgi_dn0_slot = var_qbgi_dn0;
        *var_qbgi_dn1_slot = var_qbgi_dn1;
        *var_qbgi_dn2_slot = var_qbgi_dn2;
        *var_qbgi_dn3_slot = var_qbgi_dn3;
        *var_qbgi_dn4_slot = var_qbgi_dn4;
        *var_qbgi_dn5_slot = var_qbgi_dn5;
        *var_qbgi_dn6_slot = var_qbgi_dn6;
        *var_qbgi_dn7_slot = var_qbgi_dn7;
        *var_qbgi_dn8_slot = var_qbgi_dn8;
        *var_qbgi_rdb0_slot = var_qbgi_rdb0;
        *var_qbgi_rdb1_slot = var_qbgi_rdb1;
        *var_qbgi_rdb2_slot = var_qbgi_rdb2;
        *var_qbgi_rdb3_slot = var_qbgi_rdb3;
        *var_qbgi_rdb4_slot = var_qbgi_rdb4;
        *var_qbgi_rdn0_slot = var_qbgi_rdn0;
        *var_qbgi_rdn1_slot = var_qbgi_rdn1;
        *var_qbgi_rdn2_slot = var_qbgi_rdn2;
        *var_qbgi_rdn3_slot = var_qbgi_rdn3;
        *var_qbgi_rdn4_slot = var_qbgi_rdn4;
        *var_qbgi_rdn5_slot = var_qbgi_rdn5;
        *var_qbgi_rdn6_slot = var_qbgi_rdn6;
        *var_qbgi_rdn7_slot = var_qbgi_rdn7;
        *var_qbgi_rdn8_slot = var_qbgi_rdn8;
        *var_qbgi_rv_slot = var_qbgi_rv;
        *var_qdi_slot = var_qdi;
        *var_qdi_db0_slot = var_qdi_db0;
        *var_qdi_db1_slot = var_qdi_db1;
        *var_qdi_db2_slot = var_qdi_db2;
        *var_qdi_db3_slot = var_qdi_db3;
        *var_qdi_db4_slot = var_qdi_db4;
        *var_qdi_dn0_slot = var_qdi_dn0;
        *var_qdi_dn1_slot = var_qdi_dn1;
        *var_qdi_dn2_slot = var_qdi_dn2;
        *var_qdi_dn3_slot = var_qdi_dn3;
        *var_qdi_dn4_slot = var_qdi_dn4;
        *var_qdi_dn5_slot = var_qdi_dn5;
        *var_qdi_dn6_slot = var_qdi_dn6;
        *var_qdi_dn7_slot = var_qdi_dn7;
        *var_qdi_dn8_slot = var_qdi_dn8;
        *var_qdi_rdb0_slot = var_qdi_rdb0;
        *var_qdi_rdb1_slot = var_qdi_rdb1;
        *var_qdi_rdb2_slot = var_qdi_rdb2;
        *var_qdi_rdb3_slot = var_qdi_rdb3;
        *var_qdi_rdb4_slot = var_qdi_rdb4;
        *var_qdi_rdn0_slot = var_qdi_rdn0;
        *var_qdi_rdn1_slot = var_qdi_rdn1;
        *var_qdi_rdn2_slot = var_qdi_rdn2;
        *var_qdi_rdn3_slot = var_qdi_rdn3;
        *var_qdi_rdn4_slot = var_qdi_rdn4;
        *var_qdi_rdn5_slot = var_qdi_rdn5;
        *var_qdi_rdn6_slot = var_qdi_rdn6;
        *var_qdi_rdn7_slot = var_qdi_rdn7;
        *var_qdi_rdn8_slot = var_qdi_rdn8;
        *var_qdi_rv_slot = var_qdi_rv;
        *var_qfgi_slot = var_qfgi;
        *var_qfgi_db0_slot = var_qfgi_db0;
        *var_qfgi_db1_slot = var_qfgi_db1;
        *var_qfgi_db2_slot = var_qfgi_db2;
        *var_qfgi_db3_slot = var_qfgi_db3;
        *var_qfgi_db4_slot = var_qfgi_db4;
        *var_qfgi_dn0_slot = var_qfgi_dn0;
        *var_qfgi_dn1_slot = var_qfgi_dn1;
        *var_qfgi_dn2_slot = var_qfgi_dn2;
        *var_qfgi_dn3_slot = var_qfgi_dn3;
        *var_qfgi_dn4_slot = var_qfgi_dn4;
        *var_qfgi_dn5_slot = var_qfgi_dn5;
        *var_qfgi_dn6_slot = var_qfgi_dn6;
        *var_qfgi_dn7_slot = var_qfgi_dn7;
        *var_qfgi_dn8_slot = var_qfgi_dn8;
        *var_qfgi_rdb0_slot = var_qfgi_rdb0;
        *var_qfgi_rdb1_slot = var_qfgi_rdb1;
        *var_qfgi_rdb2_slot = var_qfgi_rdb2;
        *var_qfgi_rdb3_slot = var_qfgi_rdb3;
        *var_qfgi_rdb4_slot = var_qfgi_rdb4;
        *var_qfgi_rdn0_slot = var_qfgi_rdn0;
        *var_qfgi_rdn1_slot = var_qfgi_rdn1;
        *var_qfgi_rdn2_slot = var_qfgi_rdn2;
        *var_qfgi_rdn3_slot = var_qfgi_rdn3;
        *var_qfgi_rdn4_slot = var_qfgi_rdn4;
        *var_qfgi_rdn5_slot = var_qfgi_rdn5;
        *var_qfgi_rdn6_slot = var_qfgi_rdn6;
        *var_qfgi_rdn7_slot = var_qfgi_rdn7;
        *var_qfgi_rdn8_slot = var_qfgi_rdn8;
        *var_qfgi_rv_slot = var_qfgi_rv;
        *var_qsi_slot = var_qsi;
        *var_qsi_db0_slot = var_qsi_db0;
        *var_qsi_db1_slot = var_qsi_db1;
        *var_qsi_db2_slot = var_qsi_db2;
        *var_qsi_db3_slot = var_qsi_db3;
        *var_qsi_db4_slot = var_qsi_db4;
        *var_qsi_dn0_slot = var_qsi_dn0;
        *var_qsi_dn1_slot = var_qsi_dn1;
        *var_qsi_dn2_slot = var_qsi_dn2;
        *var_qsi_dn3_slot = var_qsi_dn3;
        *var_qsi_dn4_slot = var_qsi_dn4;
        *var_qsi_dn5_slot = var_qsi_dn5;
        *var_qsi_dn6_slot = var_qsi_dn6;
        *var_qsi_dn7_slot = var_qsi_dn7;
        *var_qsi_dn8_slot = var_qsi_dn8;
        *var_qsi_rdb0_slot = var_qsi_rdb0;
        *var_qsi_rdb1_slot = var_qsi_rdb1;
        *var_qsi_rdb2_slot = var_qsi_rdb2;
        *var_qsi_rdb3_slot = var_qsi_rdb3;
        *var_qsi_rdb4_slot = var_qsi_rdb4;
        *var_qsi_rdn0_slot = var_qsi_rdn0;
        *var_qsi_rdn1_slot = var_qsi_rdn1;
        *var_qsi_rdn2_slot = var_qsi_rdn2;
        *var_qsi_rdn3_slot = var_qsi_rdn3;
        *var_qsi_rdn4_slot = var_qsi_rdn4;
        *var_qsi_rdn5_slot = var_qsi_rdn5;
        *var_qsi_rdn6_slot = var_qsi_rdn6;
        *var_qsi_rdn7_slot = var_qsi_rdn7;
        *var_qsi_rdn8_slot = var_qsi_rdn8;
        *var_qsi_rv_slot = var_qsi_rv;
        *var_ssi_slot = var_ssi;
        *var_ssi_db0_slot = var_ssi_db0;
        *var_ssi_db1_slot = var_ssi_db1;
        *var_ssi_db2_slot = var_ssi_db2;
        *var_ssi_db3_slot = var_ssi_db3;
        *var_ssi_db4_slot = var_ssi_db4;
        *var_ssi_dn0_slot = var_ssi_dn0;
        *var_ssi_dn1_slot = var_ssi_dn1;
        *var_ssi_dn2_slot = var_ssi_dn2;
        *var_ssi_dn3_slot = var_ssi_dn3;
        *var_ssi_dn4_slot = var_ssi_dn4;
        *var_ssi_dn5_slot = var_ssi_dn5;
        *var_ssi_dn6_slot = var_ssi_dn6;
        *var_ssi_dn7_slot = var_ssi_dn7;
        *var_ssi_dn8_slot = var_ssi_dn8;
        *var_ssi_rdb0_slot = var_ssi_rdb0;
        *var_ssi_rdb1_slot = var_ssi_rdb1;
        *var_ssi_rdb2_slot = var_ssi_rdb2;
        *var_ssi_rdb3_slot = var_ssi_rdb3;
        *var_ssi_rdb4_slot = var_ssi_rdb4;
        *var_ssi_rdn0_slot = var_ssi_rdn0;
        *var_ssi_rdn1_slot = var_ssi_rdn1;
        *var_ssi_rdn2_slot = var_ssi_rdn2;
        *var_ssi_rdn3_slot = var_ssi_rdn3;
        *var_ssi_rdn4_slot = var_ssi_rdn4;
        *var_ssi_rdn5_slot = var_ssi_rdn5;
        *var_ssi_rdn6_slot = var_ssi_rdn6;
        *var_ssi_rdn7_slot = var_ssi_rdn7;
        *var_ssi_rdn8_slot = var_ssi_rdn8;
        *var_ssi_rv_slot = var_ssi_rv;
        *var_swi_slot = var_swi;
        *var_swi_db0_slot = var_swi_db0;
        *var_swi_db1_slot = var_swi_db1;
        *var_swi_db2_slot = var_swi_db2;
        *var_swi_db3_slot = var_swi_db3;
        *var_swi_db4_slot = var_swi_db4;
        *var_swi_dn0_slot = var_swi_dn0;
        *var_swi_dn1_slot = var_swi_dn1;
        *var_swi_dn2_slot = var_swi_dn2;
        *var_swi_dn3_slot = var_swi_dn3;
        *var_swi_dn4_slot = var_swi_dn4;
        *var_swi_dn5_slot = var_swi_dn5;
        *var_swi_dn6_slot = var_swi_dn6;
        *var_swi_dn7_slot = var_swi_dn7;
        *var_swi_dn8_slot = var_swi_dn8;
        *var_swi_rdb0_slot = var_swi_rdb0;
        *var_swi_rdb1_slot = var_swi_rdb1;
        *var_swi_rdb2_slot = var_swi_rdb2;
        *var_swi_rdb3_slot = var_swi_rdb3;
        *var_swi_rdb4_slot = var_swi_rdb4;
        *var_swi_rdn0_slot = var_swi_rdn0;
        *var_swi_rdn1_slot = var_swi_rdn1;
        *var_swi_rdn2_slot = var_swi_rdn2;
        *var_swi_rdn3_slot = var_swi_rdn3;
        *var_swi_rdn4_slot = var_swi_rdn4;
        *var_swi_rdn5_slot = var_swi_rdn5;
        *var_swi_rdn6_slot = var_swi_rdn6;
        *var_swi_rdn7_slot = var_swi_rdn7;
        *var_swi_rdn8_slot = var_swi_rdn8;
        *var_swi_rv_slot = var_swi_rv;
        *var_t1_slot = var_t1;
        *var_t10_slot = var_t10;
        *var_t10_db0_slot = var_t10_db0;
        *var_t10_db1_slot = var_t10_db1;
        *var_t10_db2_slot = var_t10_db2;
        *var_t10_db3_slot = var_t10_db3;
        *var_t10_db4_slot = var_t10_db4;
        *var_t10_dn0_slot = var_t10_dn0;
        *var_t10_dn1_slot = var_t10_dn1;
        *var_t10_dn2_slot = var_t10_dn2;
        *var_t10_dn3_slot = var_t10_dn3;
        *var_t10_dn4_slot = var_t10_dn4;
        *var_t10_dn5_slot = var_t10_dn5;
        *var_t10_dn6_slot = var_t10_dn6;
        *var_t10_dn7_slot = var_t10_dn7;
        *var_t10_dn8_slot = var_t10_dn8;
        *var_t10_rdb0_slot = var_t10_rdb0;
        *var_t10_rdb1_slot = var_t10_rdb1;
        *var_t10_rdb2_slot = var_t10_rdb2;
        *var_t10_rdb3_slot = var_t10_rdb3;
        *var_t10_rdb4_slot = var_t10_rdb4;
        *var_t10_rdn0_slot = var_t10_rdn0;
        *var_t10_rdn1_slot = var_t10_rdn1;
        *var_t10_rdn2_slot = var_t10_rdn2;
        *var_t10_rdn3_slot = var_t10_rdn3;
        *var_t10_rdn4_slot = var_t10_rdn4;
        *var_t10_rdn5_slot = var_t10_rdn5;
        *var_t10_rdn6_slot = var_t10_rdn6;
        *var_t10_rdn7_slot = var_t10_rdn7;
        *var_t10_rdn8_slot = var_t10_rdn8;
        *var_t10_rv_slot = var_t10_rv;
        *var_t11_slot = var_t11;
        *var_t11_db0_slot = var_t11_db0;
        *var_t11_db1_slot = var_t11_db1;
        *var_t11_db2_slot = var_t11_db2;
        *var_t11_db3_slot = var_t11_db3;
        *var_t11_db4_slot = var_t11_db4;
        *var_t11_dn0_slot = var_t11_dn0;
        *var_t11_dn1_slot = var_t11_dn1;
        *var_t11_dn2_slot = var_t11_dn2;
        *var_t11_dn3_slot = var_t11_dn3;
        *var_t11_dn4_slot = var_t11_dn4;
        *var_t11_dn5_slot = var_t11_dn5;
        *var_t11_dn6_slot = var_t11_dn6;
        *var_t11_dn7_slot = var_t11_dn7;
        *var_t11_dn8_slot = var_t11_dn8;
        *var_t11_rdb0_slot = var_t11_rdb0;
        *var_t11_rdb1_slot = var_t11_rdb1;
        *var_t11_rdb2_slot = var_t11_rdb2;
        *var_t11_rdb3_slot = var_t11_rdb3;
        *var_t11_rdb4_slot = var_t11_rdb4;
        *var_t11_rdn0_slot = var_t11_rdn0;
        *var_t11_rdn1_slot = var_t11_rdn1;
        *var_t11_rdn2_slot = var_t11_rdn2;
        *var_t11_rdn3_slot = var_t11_rdn3;
        *var_t11_rdn4_slot = var_t11_rdn4;
        *var_t11_rdn5_slot = var_t11_rdn5;
        *var_t11_rdn6_slot = var_t11_rdn6;
        *var_t11_rdn7_slot = var_t11_rdn7;
        *var_t11_rdn8_slot = var_t11_rdn8;
        *var_t11_rv_slot = var_t11_rv;
        *var_t1_db0_slot = var_t1_db0;
        *var_t1_db1_slot = var_t1_db1;
        *var_t1_db2_slot = var_t1_db2;
        *var_t1_db3_slot = var_t1_db3;
        *var_t1_db4_slot = var_t1_db4;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rdb0_slot = var_t1_rdb0;
        *var_t1_rdb1_slot = var_t1_rdb1;
        *var_t1_rdb2_slot = var_t1_rdb2;
        *var_t1_rdb3_slot = var_t1_rdb3;
        *var_t1_rdb4_slot = var_t1_rdb4;
        *var_t1_rdn0_slot = var_t1_rdn0;
        *var_t1_rdn1_slot = var_t1_rdn1;
        *var_t1_rdn2_slot = var_t1_rdn2;
        *var_t1_rdn3_slot = var_t1_rdn3;
        *var_t1_rdn4_slot = var_t1_rdn4;
        *var_t1_rdn5_slot = var_t1_rdn5;
        *var_t1_rdn6_slot = var_t1_rdn6;
        *var_t1_rdn7_slot = var_t1_rdn7;
        *var_t1_rdn8_slot = var_t1_rdn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t7_slot = var_t7;
        *var_t7_db0_slot = var_t7_db0;
        *var_t7_db1_slot = var_t7_db1;
        *var_t7_db2_slot = var_t7_db2;
        *var_t7_db3_slot = var_t7_db3;
        *var_t7_db4_slot = var_t7_db4;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn1_slot = var_t7_dn1;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn3_slot = var_t7_dn3;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_rdb0_slot = var_t7_rdb0;
        *var_t7_rdb1_slot = var_t7_rdb1;
        *var_t7_rdb2_slot = var_t7_rdb2;
        *var_t7_rdb3_slot = var_t7_rdb3;
        *var_t7_rdb4_slot = var_t7_rdb4;
        *var_t7_rdn0_slot = var_t7_rdn0;
        *var_t7_rdn1_slot = var_t7_rdn1;
        *var_t7_rdn2_slot = var_t7_rdn2;
        *var_t7_rdn3_slot = var_t7_rdn3;
        *var_t7_rdn4_slot = var_t7_rdn4;
        *var_t7_rdn5_slot = var_t7_rdn5;
        *var_t7_rdn6_slot = var_t7_rdn6;
        *var_t7_rdn7_slot = var_t7_rdn7;
        *var_t7_rdn8_slot = var_t7_rdn8;
        *var_t7_rv_slot = var_t7_rv;
        *var_t8_slot = var_t8;
        *var_t8_db0_slot = var_t8_db0;
        *var_t8_db1_slot = var_t8_db1;
        *var_t8_db2_slot = var_t8_db2;
        *var_t8_db3_slot = var_t8_db3;
        *var_t8_db4_slot = var_t8_db4;
        *var_t8_dn0_slot = var_t8_dn0;
        *var_t8_dn1_slot = var_t8_dn1;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn3_slot = var_t8_dn3;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_rdb0_slot = var_t8_rdb0;
        *var_t8_rdb1_slot = var_t8_rdb1;
        *var_t8_rdb2_slot = var_t8_rdb2;
        *var_t8_rdb3_slot = var_t8_rdb3;
        *var_t8_rdb4_slot = var_t8_rdb4;
        *var_t8_rdn0_slot = var_t8_rdn0;
        *var_t8_rdn1_slot = var_t8_rdn1;
        *var_t8_rdn2_slot = var_t8_rdn2;
        *var_t8_rdn3_slot = var_t8_rdn3;
        *var_t8_rdn4_slot = var_t8_rdn4;
        *var_t8_rdn5_slot = var_t8_rdn5;
        *var_t8_rdn6_slot = var_t8_rdn6;
        *var_t8_rdn7_slot = var_t8_rdn7;
        *var_t8_rdn8_slot = var_t8_rdn8;
        *var_t8_rv_slot = var_t8_rv;
        *var_t9_slot = var_t9;
        *var_t9_db0_slot = var_t9_db0;
        *var_t9_db1_slot = var_t9_db1;
        *var_t9_db2_slot = var_t9_db2;
        *var_t9_db3_slot = var_t9_db3;
        *var_t9_db4_slot = var_t9_db4;
        *var_t9_dn0_slot = var_t9_dn0;
        *var_t9_dn1_slot = var_t9_dn1;
        *var_t9_dn2_slot = var_t9_dn2;
        *var_t9_dn3_slot = var_t9_dn3;
        *var_t9_dn4_slot = var_t9_dn4;
        *var_t9_dn5_slot = var_t9_dn5;
        *var_t9_dn6_slot = var_t9_dn6;
        *var_t9_dn7_slot = var_t9_dn7;
        *var_t9_dn8_slot = var_t9_dn8;
        *var_t9_rdb0_slot = var_t9_rdb0;
        *var_t9_rdb1_slot = var_t9_rdb1;
        *var_t9_rdb2_slot = var_t9_rdb2;
        *var_t9_rdb3_slot = var_t9_rdb3;
        *var_t9_rdb4_slot = var_t9_rdb4;
        *var_t9_rdn0_slot = var_t9_rdn0;
        *var_t9_rdn1_slot = var_t9_rdn1;
        *var_t9_rdn2_slot = var_t9_rdn2;
        *var_t9_rdn3_slot = var_t9_rdn3;
        *var_t9_rdn4_slot = var_t9_rdn4;
        *var_t9_rdn5_slot = var_t9_rdn5;
        *var_t9_rdn6_slot = var_t9_rdn6;
        *var_t9_rdn7_slot = var_t9_rdn7;
        *var_t9_rdn8_slot = var_t9_rdn8;
        *var_t9_rv_slot = var_t9_rv;
    }

    pub(super) fn stamp_reactive_block_90(
        p: &Parameters,
        var_guard136: f64,
        var_leff: f64,
        var_leff_db0: f64,
        var_leff_db1: f64,
        var_leff_db2: f64,
        var_leff_db3: f64,
        var_leff_db4: f64,
        var_leff_dn0: f64,
        var_leff_dn1: f64,
        var_leff_dn2: f64,
        var_leff_dn3: f64,
        var_leff_dn4: f64,
        var_leff_dn5: f64,
        var_leff_dn6: f64,
        var_leff_dn7: f64,
        var_leff_dn8: f64,
        var_qdbg: f64,
        var_qdbg_db0: f64,
        var_qdbg_db1: f64,
        var_qdbg_db2: f64,
        var_qdbg_db3: f64,
        var_qdbg_db4: f64,
        var_qdbg_dn0: f64,
        var_qdbg_dn1: f64,
        var_qdbg_dn2: f64,
        var_qdbg_dn3: f64,
        var_qdbg_dn4: f64,
        var_qdbg_dn5: f64,
        var_qdbg_dn6: f64,
        var_qdbg_dn7: f64,
        var_qdbg_dn8: f64,
        var_qfgi: f64,
        var_qfgi_db0: f64,
        var_qfgi_db1: f64,
        var_qfgi_db2: f64,
        var_qfgi_db3: f64,
        var_qfgi_db4: f64,
        var_qfgi_dn0: f64,
        var_qfgi_dn1: f64,
        var_qfgi_dn2: f64,
        var_qfgi_dn3: f64,
        var_qfgi_dn4: f64,
        var_qfgi_dn5: f64,
        var_qfgi_dn6: f64,
        var_qfgi_dn7: f64,
        var_qfgi_dn8: f64,
        var_qsbg: f64,
        var_qsbg_db0: f64,
        var_qsbg_db1: f64,
        var_qsbg_db2: f64,
        var_qsbg_db3: f64,
        var_qsbg_db4: f64,
        var_qsbg_dn0: f64,
        var_qsbg_dn1: f64,
        var_qsbg_dn2: f64,
        var_qsbg_dn3: f64,
        var_qsbg_dn4: f64,
        var_qsbg_dn5: f64,
        var_qsbg_dn6: f64,
        var_qsbg_dn7: f64,
        var_qsbg_dn8: f64,
        var_rdsi: f64,
        var_rdsi_db0: f64,
        var_rdsi_db1: f64,
        var_rdsi_db2: f64,
        var_rdsi_db3: f64,
        var_rdsi_db4: f64,
        var_rdsi_dn0: f64,
        var_rdsi_dn1: f64,
        var_rdsi_dn2: f64,
        var_rdsi_dn3: f64,
        var_rdsi_dn4: f64,
        var_rdsi_dn5: f64,
        var_rdsi_dn6: f64,
        var_rdsi_dn7: f64,
        var_rdsi_dn8: f64,
        var_utotal: f64,
        var_utotal_db0: f64,
        var_utotal_db1: f64,
        var_utotal_db2: f64,
        var_utotal_db3: f64,
        var_utotal_db4: f64,
        var_utotal_dn0: f64,
        var_utotal_dn1: f64,
        var_utotal_dn2: f64,
        var_utotal_dn3: f64,
        var_utotal_dn4: f64,
        var_utotal_dn5: f64,
        var_utotal_dn6: f64,
        var_utotal_dn7: f64,
        var_utotal_dn8: f64,
        var_qbg_slot: &mut f64,
        var_qbg_db0_slot: &mut f64,
        var_qbg_db1_slot: &mut f64,
        var_qbg_db2_slot: &mut f64,
        var_qbg_db3_slot: &mut f64,
        var_qbg_db4_slot: &mut f64,
        var_qbg_dn0_slot: &mut f64,
        var_qbg_dn1_slot: &mut f64,
        var_qbg_dn2_slot: &mut f64,
        var_qbg_dn3_slot: &mut f64,
        var_qbg_dn4_slot: &mut f64,
        var_qbg_dn5_slot: &mut f64,
        var_qbg_dn6_slot: &mut f64,
        var_qbg_dn7_slot: &mut f64,
        var_qbg_dn8_slot: &mut f64,
        var_qbg_rdb0_slot: &mut f64,
        var_qbg_rdb1_slot: &mut f64,
        var_qbg_rdb2_slot: &mut f64,
        var_qbg_rdb3_slot: &mut f64,
        var_qbg_rdb4_slot: &mut f64,
        var_qbg_rdn0_slot: &mut f64,
        var_qbg_rdn1_slot: &mut f64,
        var_qbg_rdn2_slot: &mut f64,
        var_qbg_rdn3_slot: &mut f64,
        var_qbg_rdn4_slot: &mut f64,
        var_qbg_rdn5_slot: &mut f64,
        var_qbg_rdn6_slot: &mut f64,
        var_qbg_rdn7_slot: &mut f64,
        var_qbg_rdn8_slot: &mut f64,
        var_qbg_rv_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_db0_slot: &mut f64,
        var_qd_db1_slot: &mut f64,
        var_qd_db2_slot: &mut f64,
        var_qd_db3_slot: &mut f64,
        var_qd_db4_slot: &mut f64,
        var_qd_dn0_slot: &mut f64,
        var_qd_dn1_slot: &mut f64,
        var_qd_dn2_slot: &mut f64,
        var_qd_dn3_slot: &mut f64,
        var_qd_dn4_slot: &mut f64,
        var_qd_dn5_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qd_rdb0_slot: &mut f64,
        var_qd_rdb1_slot: &mut f64,
        var_qd_rdb2_slot: &mut f64,
        var_qd_rdb3_slot: &mut f64,
        var_qd_rdb4_slot: &mut f64,
        var_qd_rdn0_slot: &mut f64,
        var_qd_rdn1_slot: &mut f64,
        var_qd_rdn2_slot: &mut f64,
        var_qd_rdn3_slot: &mut f64,
        var_qd_rdn4_slot: &mut f64,
        var_qd_rdn5_slot: &mut f64,
        var_qd_rdn6_slot: &mut f64,
        var_qd_rdn7_slot: &mut f64,
        var_qd_rdn8_slot: &mut f64,
        var_qd_rv_slot: &mut f64,
        var_qdi_slot: &mut f64,
        var_qdi_db0_slot: &mut f64,
        var_qdi_db1_slot: &mut f64,
        var_qdi_db2_slot: &mut f64,
        var_qdi_db3_slot: &mut f64,
        var_qdi_db4_slot: &mut f64,
        var_qdi_dn0_slot: &mut f64,
        var_qdi_dn1_slot: &mut f64,
        var_qdi_dn2_slot: &mut f64,
        var_qdi_dn3_slot: &mut f64,
        var_qdi_dn4_slot: &mut f64,
        var_qdi_dn5_slot: &mut f64,
        var_qdi_dn6_slot: &mut f64,
        var_qdi_dn7_slot: &mut f64,
        var_qdi_dn8_slot: &mut f64,
        var_qdi_rdb0_slot: &mut f64,
        var_qdi_rdb1_slot: &mut f64,
        var_qdi_rdb2_slot: &mut f64,
        var_qdi_rdb3_slot: &mut f64,
        var_qdi_rdb4_slot: &mut f64,
        var_qdi_rdn0_slot: &mut f64,
        var_qdi_rdn1_slot: &mut f64,
        var_qdi_rdn2_slot: &mut f64,
        var_qdi_rdn3_slot: &mut f64,
        var_qdi_rdn4_slot: &mut f64,
        var_qdi_rdn5_slot: &mut f64,
        var_qdi_rdn6_slot: &mut f64,
        var_qdi_rdn7_slot: &mut f64,
        var_qdi_rdn8_slot: &mut f64,
        var_qdi_rv_slot: &mut f64,
        var_qfg_slot: &mut f64,
        var_qfg_db0_slot: &mut f64,
        var_qfg_db1_slot: &mut f64,
        var_qfg_db2_slot: &mut f64,
        var_qfg_db3_slot: &mut f64,
        var_qfg_db4_slot: &mut f64,
        var_qfg_dn0_slot: &mut f64,
        var_qfg_dn1_slot: &mut f64,
        var_qfg_dn2_slot: &mut f64,
        var_qfg_dn3_slot: &mut f64,
        var_qfg_dn4_slot: &mut f64,
        var_qfg_dn5_slot: &mut f64,
        var_qfg_dn6_slot: &mut f64,
        var_qfg_dn7_slot: &mut f64,
        var_qfg_dn8_slot: &mut f64,
        var_qfg_rdb0_slot: &mut f64,
        var_qfg_rdb1_slot: &mut f64,
        var_qfg_rdb2_slot: &mut f64,
        var_qfg_rdb3_slot: &mut f64,
        var_qfg_rdb4_slot: &mut f64,
        var_qfg_rdn0_slot: &mut f64,
        var_qfg_rdn1_slot: &mut f64,
        var_qfg_rdn2_slot: &mut f64,
        var_qfg_rdn3_slot: &mut f64,
        var_qfg_rdn4_slot: &mut f64,
        var_qfg_rdn5_slot: &mut f64,
        var_qfg_rdn6_slot: &mut f64,
        var_qfg_rdn7_slot: &mut f64,
        var_qfg_rdn8_slot: &mut f64,
        var_qfg_rv_slot: &mut f64,
        var_qfgd_parasitic_slot: &mut f64,
        var_qfgd_parasitic_db0_slot: &mut f64,
        var_qfgd_parasitic_db1_slot: &mut f64,
        var_qfgd_parasitic_db2_slot: &mut f64,
        var_qfgd_parasitic_db3_slot: &mut f64,
        var_qfgd_parasitic_db4_slot: &mut f64,
        var_qfgd_parasitic_dn0_slot: &mut f64,
        var_qfgd_parasitic_dn1_slot: &mut f64,
        var_qfgd_parasitic_dn2_slot: &mut f64,
        var_qfgd_parasitic_dn3_slot: &mut f64,
        var_qfgd_parasitic_dn4_slot: &mut f64,
        var_qfgd_parasitic_dn5_slot: &mut f64,
        var_qfgd_parasitic_dn6_slot: &mut f64,
        var_qfgd_parasitic_dn7_slot: &mut f64,
        var_qfgd_parasitic_dn8_slot: &mut f64,
        var_qfgd_parasitic_rdb0_slot: &mut f64,
        var_qfgd_parasitic_rdb1_slot: &mut f64,
        var_qfgd_parasitic_rdb2_slot: &mut f64,
        var_qfgd_parasitic_rdb3_slot: &mut f64,
        var_qfgd_parasitic_rdb4_slot: &mut f64,
        var_qfgd_parasitic_rdn0_slot: &mut f64,
        var_qfgd_parasitic_rdn1_slot: &mut f64,
        var_qfgd_parasitic_rdn2_slot: &mut f64,
        var_qfgd_parasitic_rdn3_slot: &mut f64,
        var_qfgd_parasitic_rdn4_slot: &mut f64,
        var_qfgd_parasitic_rdn5_slot: &mut f64,
        var_qfgd_parasitic_rdn6_slot: &mut f64,
        var_qfgd_parasitic_rdn7_slot: &mut f64,
        var_qfgd_parasitic_rdn8_slot: &mut f64,
        var_qfgd_parasitic_rv_slot: &mut f64,
        var_qfgs_parasitic_slot: &mut f64,
        var_qfgs_parasitic_db0_slot: &mut f64,
        var_qfgs_parasitic_db1_slot: &mut f64,
        var_qfgs_parasitic_db2_slot: &mut f64,
        var_qfgs_parasitic_db3_slot: &mut f64,
        var_qfgs_parasitic_db4_slot: &mut f64,
        var_qfgs_parasitic_dn0_slot: &mut f64,
        var_qfgs_parasitic_dn1_slot: &mut f64,
        var_qfgs_parasitic_dn2_slot: &mut f64,
        var_qfgs_parasitic_dn3_slot: &mut f64,
        var_qfgs_parasitic_dn4_slot: &mut f64,
        var_qfgs_parasitic_dn5_slot: &mut f64,
        var_qfgs_parasitic_dn6_slot: &mut f64,
        var_qfgs_parasitic_dn7_slot: &mut f64,
        var_qfgs_parasitic_dn8_slot: &mut f64,
        var_qfgs_parasitic_rdb0_slot: &mut f64,
        var_qfgs_parasitic_rdb1_slot: &mut f64,
        var_qfgs_parasitic_rdb2_slot: &mut f64,
        var_qfgs_parasitic_rdb3_slot: &mut f64,
        var_qfgs_parasitic_rdb4_slot: &mut f64,
        var_qfgs_parasitic_rdn0_slot: &mut f64,
        var_qfgs_parasitic_rdn1_slot: &mut f64,
        var_qfgs_parasitic_rdn2_slot: &mut f64,
        var_qfgs_parasitic_rdn3_slot: &mut f64,
        var_qfgs_parasitic_rdn4_slot: &mut f64,
        var_qfgs_parasitic_rdn5_slot: &mut f64,
        var_qfgs_parasitic_rdn6_slot: &mut f64,
        var_qfgs_parasitic_rdn7_slot: &mut f64,
        var_qfgs_parasitic_rdn8_slot: &mut f64,
        var_qfgs_parasitic_rv_slot: &mut f64,
        var_qinv_slot: &mut f64,
        var_qinv_db0_slot: &mut f64,
        var_qinv_db1_slot: &mut f64,
        var_qinv_db2_slot: &mut f64,
        var_qinv_db3_slot: &mut f64,
        var_qinv_db4_slot: &mut f64,
        var_qinv_dn0_slot: &mut f64,
        var_qinv_dn1_slot: &mut f64,
        var_qinv_dn2_slot: &mut f64,
        var_qinv_dn3_slot: &mut f64,
        var_qinv_dn4_slot: &mut f64,
        var_qinv_dn5_slot: &mut f64,
        var_qinv_dn6_slot: &mut f64,
        var_qinv_dn7_slot: &mut f64,
        var_qinv_dn8_slot: &mut f64,
        var_qinv_rdb0_slot: &mut f64,
        var_qinv_rdb1_slot: &mut f64,
        var_qinv_rdb2_slot: &mut f64,
        var_qinv_rdb3_slot: &mut f64,
        var_qinv_rdb4_slot: &mut f64,
        var_qinv_rdn0_slot: &mut f64,
        var_qinv_rdn1_slot: &mut f64,
        var_qinv_rdn2_slot: &mut f64,
        var_qinv_rdn3_slot: &mut f64,
        var_qinv_rdn4_slot: &mut f64,
        var_qinv_rdn5_slot: &mut f64,
        var_qinv_rdn6_slot: &mut f64,
        var_qinv_rdn7_slot: &mut f64,
        var_qinv_rdn8_slot: &mut f64,
        var_qinv_rv_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_db0_slot: &mut f64,
        var_qs_db1_slot: &mut f64,
        var_qs_db2_slot: &mut f64,
        var_qs_db3_slot: &mut f64,
        var_qs_db4_slot: &mut f64,
        var_qs_dn0_slot: &mut f64,
        var_qs_dn1_slot: &mut f64,
        var_qs_dn2_slot: &mut f64,
        var_qs_dn3_slot: &mut f64,
        var_qs_dn4_slot: &mut f64,
        var_qs_dn5_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_qs_rdb0_slot: &mut f64,
        var_qs_rdb1_slot: &mut f64,
        var_qs_rdb2_slot: &mut f64,
        var_qs_rdb3_slot: &mut f64,
        var_qs_rdb4_slot: &mut f64,
        var_qs_rdn0_slot: &mut f64,
        var_qs_rdn1_slot: &mut f64,
        var_qs_rdn2_slot: &mut f64,
        var_qs_rdn3_slot: &mut f64,
        var_qs_rdn4_slot: &mut f64,
        var_qs_rdn5_slot: &mut f64,
        var_qs_rdn6_slot: &mut f64,
        var_qs_rdn7_slot: &mut f64,
        var_qs_rdn8_slot: &mut f64,
        var_qs_rv_slot: &mut f64,
        var_qsi_slot: &mut f64,
        var_qsi_db0_slot: &mut f64,
        var_qsi_db1_slot: &mut f64,
        var_qsi_db2_slot: &mut f64,
        var_qsi_db3_slot: &mut f64,
        var_qsi_db4_slot: &mut f64,
        var_qsi_dn0_slot: &mut f64,
        var_qsi_dn1_slot: &mut f64,
        var_qsi_dn2_slot: &mut f64,
        var_qsi_dn3_slot: &mut f64,
        var_qsi_dn4_slot: &mut f64,
        var_qsi_dn5_slot: &mut f64,
        var_qsi_dn6_slot: &mut f64,
        var_qsi_dn7_slot: &mut f64,
        var_qsi_dn8_slot: &mut f64,
        var_qsi_rdb0_slot: &mut f64,
        var_qsi_rdb1_slot: &mut f64,
        var_qsi_rdb2_slot: &mut f64,
        var_qsi_rdb3_slot: &mut f64,
        var_qsi_rdb4_slot: &mut f64,
        var_qsi_rdn0_slot: &mut f64,
        var_qsi_rdn1_slot: &mut f64,
        var_qsi_rdn2_slot: &mut f64,
        var_qsi_rdn3_slot: &mut f64,
        var_qsi_rdn4_slot: &mut f64,
        var_qsi_rdn5_slot: &mut f64,
        var_qsi_rdn6_slot: &mut f64,
        var_qsi_rdn7_slot: &mut f64,
        var_qsi_rdn8_slot: &mut f64,
        var_qsi_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_db0_slot: &mut f64,
        var_t0_db1_slot: &mut f64,
        var_t0_db2_slot: &mut f64,
        var_t0_db3_slot: &mut f64,
        var_t0_db4_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rdb0_slot: &mut f64,
        var_t0_rdb1_slot: &mut f64,
        var_t0_rdb2_slot: &mut f64,
        var_t0_rdb3_slot: &mut f64,
        var_t0_rdb4_slot: &mut f64,
        var_t0_rdn0_slot: &mut f64,
        var_t0_rdn1_slot: &mut f64,
        var_t0_rdn2_slot: &mut f64,
        var_t0_rdn3_slot: &mut f64,
        var_t0_rdn4_slot: &mut f64,
        var_t0_rdn5_slot: &mut f64,
        var_t0_rdn6_slot: &mut f64,
        var_t0_rdn7_slot: &mut f64,
        var_t0_rdn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_db0_slot: &mut f64,
        var_t1_db1_slot: &mut f64,
        var_t1_db2_slot: &mut f64,
        var_t1_db3_slot: &mut f64,
        var_t1_db4_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rdb0_slot: &mut f64,
        var_t1_rdb1_slot: &mut f64,
        var_t1_rdb2_slot: &mut f64,
        var_t1_rdb3_slot: &mut f64,
        var_t1_rdb4_slot: &mut f64,
        var_t1_rdn0_slot: &mut f64,
        var_t1_rdn1_slot: &mut f64,
        var_t1_rdn2_slot: &mut f64,
        var_t1_rdn3_slot: &mut f64,
        var_t1_rdn4_slot: &mut f64,
        var_t1_rdn5_slot: &mut f64,
        var_t1_rdn6_slot: &mut f64,
        var_t1_rdn7_slot: &mut f64,
        var_t1_rdn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
    ) {
        let mut var_qbg: f64 = *var_qbg_slot;
        let mut var_qbg_db0: f64 = *var_qbg_db0_slot;
        let mut var_qbg_db1: f64 = *var_qbg_db1_slot;
        let mut var_qbg_db2: f64 = *var_qbg_db2_slot;
        let mut var_qbg_db3: f64 = *var_qbg_db3_slot;
        let mut var_qbg_db4: f64 = *var_qbg_db4_slot;
        let mut var_qbg_dn0: f64 = *var_qbg_dn0_slot;
        let mut var_qbg_dn1: f64 = *var_qbg_dn1_slot;
        let mut var_qbg_dn2: f64 = *var_qbg_dn2_slot;
        let mut var_qbg_dn3: f64 = *var_qbg_dn3_slot;
        let mut var_qbg_dn4: f64 = *var_qbg_dn4_slot;
        let mut var_qbg_dn5: f64 = *var_qbg_dn5_slot;
        let mut var_qbg_dn6: f64 = *var_qbg_dn6_slot;
        let mut var_qbg_dn7: f64 = *var_qbg_dn7_slot;
        let mut var_qbg_dn8: f64 = *var_qbg_dn8_slot;
        let mut var_qbg_rdb0: f64 = *var_qbg_rdb0_slot;
        let mut var_qbg_rdb1: f64 = *var_qbg_rdb1_slot;
        let mut var_qbg_rdb2: f64 = *var_qbg_rdb2_slot;
        let mut var_qbg_rdb3: f64 = *var_qbg_rdb3_slot;
        let mut var_qbg_rdb4: f64 = *var_qbg_rdb4_slot;
        let mut var_qbg_rdn0: f64 = *var_qbg_rdn0_slot;
        let mut var_qbg_rdn1: f64 = *var_qbg_rdn1_slot;
        let mut var_qbg_rdn2: f64 = *var_qbg_rdn2_slot;
        let mut var_qbg_rdn3: f64 = *var_qbg_rdn3_slot;
        let mut var_qbg_rdn4: f64 = *var_qbg_rdn4_slot;
        let mut var_qbg_rdn5: f64 = *var_qbg_rdn5_slot;
        let mut var_qbg_rdn6: f64 = *var_qbg_rdn6_slot;
        let mut var_qbg_rdn7: f64 = *var_qbg_rdn7_slot;
        let mut var_qbg_rdn8: f64 = *var_qbg_rdn8_slot;
        let mut var_qbg_rv: f64 = *var_qbg_rv_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_db0: f64 = *var_qd_db0_slot;
        let mut var_qd_db1: f64 = *var_qd_db1_slot;
        let mut var_qd_db2: f64 = *var_qd_db2_slot;
        let mut var_qd_db3: f64 = *var_qd_db3_slot;
        let mut var_qd_db4: f64 = *var_qd_db4_slot;
        let mut var_qd_dn0: f64 = *var_qd_dn0_slot;
        let mut var_qd_dn1: f64 = *var_qd_dn1_slot;
        let mut var_qd_dn2: f64 = *var_qd_dn2_slot;
        let mut var_qd_dn3: f64 = *var_qd_dn3_slot;
        let mut var_qd_dn4: f64 = *var_qd_dn4_slot;
        let mut var_qd_dn5: f64 = *var_qd_dn5_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qd_rdb0: f64 = *var_qd_rdb0_slot;
        let mut var_qd_rdb1: f64 = *var_qd_rdb1_slot;
        let mut var_qd_rdb2: f64 = *var_qd_rdb2_slot;
        let mut var_qd_rdb3: f64 = *var_qd_rdb3_slot;
        let mut var_qd_rdb4: f64 = *var_qd_rdb4_slot;
        let mut var_qd_rdn0: f64 = *var_qd_rdn0_slot;
        let mut var_qd_rdn1: f64 = *var_qd_rdn1_slot;
        let mut var_qd_rdn2: f64 = *var_qd_rdn2_slot;
        let mut var_qd_rdn3: f64 = *var_qd_rdn3_slot;
        let mut var_qd_rdn4: f64 = *var_qd_rdn4_slot;
        let mut var_qd_rdn5: f64 = *var_qd_rdn5_slot;
        let mut var_qd_rdn6: f64 = *var_qd_rdn6_slot;
        let mut var_qd_rdn7: f64 = *var_qd_rdn7_slot;
        let mut var_qd_rdn8: f64 = *var_qd_rdn8_slot;
        let mut var_qd_rv: f64 = *var_qd_rv_slot;
        let mut var_qdi: f64 = *var_qdi_slot;
        let mut var_qdi_db0: f64 = *var_qdi_db0_slot;
        let mut var_qdi_db1: f64 = *var_qdi_db1_slot;
        let mut var_qdi_db2: f64 = *var_qdi_db2_slot;
        let mut var_qdi_db3: f64 = *var_qdi_db3_slot;
        let mut var_qdi_db4: f64 = *var_qdi_db4_slot;
        let mut var_qdi_dn0: f64 = *var_qdi_dn0_slot;
        let mut var_qdi_dn1: f64 = *var_qdi_dn1_slot;
        let mut var_qdi_dn2: f64 = *var_qdi_dn2_slot;
        let mut var_qdi_dn3: f64 = *var_qdi_dn3_slot;
        let mut var_qdi_dn4: f64 = *var_qdi_dn4_slot;
        let mut var_qdi_dn5: f64 = *var_qdi_dn5_slot;
        let mut var_qdi_dn6: f64 = *var_qdi_dn6_slot;
        let mut var_qdi_dn7: f64 = *var_qdi_dn7_slot;
        let mut var_qdi_dn8: f64 = *var_qdi_dn8_slot;
        let mut var_qdi_rdb0: f64 = *var_qdi_rdb0_slot;
        let mut var_qdi_rdb1: f64 = *var_qdi_rdb1_slot;
        let mut var_qdi_rdb2: f64 = *var_qdi_rdb2_slot;
        let mut var_qdi_rdb3: f64 = *var_qdi_rdb3_slot;
        let mut var_qdi_rdb4: f64 = *var_qdi_rdb4_slot;
        let mut var_qdi_rdn0: f64 = *var_qdi_rdn0_slot;
        let mut var_qdi_rdn1: f64 = *var_qdi_rdn1_slot;
        let mut var_qdi_rdn2: f64 = *var_qdi_rdn2_slot;
        let mut var_qdi_rdn3: f64 = *var_qdi_rdn3_slot;
        let mut var_qdi_rdn4: f64 = *var_qdi_rdn4_slot;
        let mut var_qdi_rdn5: f64 = *var_qdi_rdn5_slot;
        let mut var_qdi_rdn6: f64 = *var_qdi_rdn6_slot;
        let mut var_qdi_rdn7: f64 = *var_qdi_rdn7_slot;
        let mut var_qdi_rdn8: f64 = *var_qdi_rdn8_slot;
        let mut var_qdi_rv: f64 = *var_qdi_rv_slot;
        let mut var_qfg: f64 = *var_qfg_slot;
        let mut var_qfg_db0: f64 = *var_qfg_db0_slot;
        let mut var_qfg_db1: f64 = *var_qfg_db1_slot;
        let mut var_qfg_db2: f64 = *var_qfg_db2_slot;
        let mut var_qfg_db3: f64 = *var_qfg_db3_slot;
        let mut var_qfg_db4: f64 = *var_qfg_db4_slot;
        let mut var_qfg_dn0: f64 = *var_qfg_dn0_slot;
        let mut var_qfg_dn1: f64 = *var_qfg_dn1_slot;
        let mut var_qfg_dn2: f64 = *var_qfg_dn2_slot;
        let mut var_qfg_dn3: f64 = *var_qfg_dn3_slot;
        let mut var_qfg_dn4: f64 = *var_qfg_dn4_slot;
        let mut var_qfg_dn5: f64 = *var_qfg_dn5_slot;
        let mut var_qfg_dn6: f64 = *var_qfg_dn6_slot;
        let mut var_qfg_dn7: f64 = *var_qfg_dn7_slot;
        let mut var_qfg_dn8: f64 = *var_qfg_dn8_slot;
        let mut var_qfg_rdb0: f64 = *var_qfg_rdb0_slot;
        let mut var_qfg_rdb1: f64 = *var_qfg_rdb1_slot;
        let mut var_qfg_rdb2: f64 = *var_qfg_rdb2_slot;
        let mut var_qfg_rdb3: f64 = *var_qfg_rdb3_slot;
        let mut var_qfg_rdb4: f64 = *var_qfg_rdb4_slot;
        let mut var_qfg_rdn0: f64 = *var_qfg_rdn0_slot;
        let mut var_qfg_rdn1: f64 = *var_qfg_rdn1_slot;
        let mut var_qfg_rdn2: f64 = *var_qfg_rdn2_slot;
        let mut var_qfg_rdn3: f64 = *var_qfg_rdn3_slot;
        let mut var_qfg_rdn4: f64 = *var_qfg_rdn4_slot;
        let mut var_qfg_rdn5: f64 = *var_qfg_rdn5_slot;
        let mut var_qfg_rdn6: f64 = *var_qfg_rdn6_slot;
        let mut var_qfg_rdn7: f64 = *var_qfg_rdn7_slot;
        let mut var_qfg_rdn8: f64 = *var_qfg_rdn8_slot;
        let mut var_qfg_rv: f64 = *var_qfg_rv_slot;
        let mut var_qfgd_parasitic: f64 = *var_qfgd_parasitic_slot;
        let mut var_qfgd_parasitic_db0: f64 = *var_qfgd_parasitic_db0_slot;
        let mut var_qfgd_parasitic_db1: f64 = *var_qfgd_parasitic_db1_slot;
        let mut var_qfgd_parasitic_db2: f64 = *var_qfgd_parasitic_db2_slot;
        let mut var_qfgd_parasitic_db3: f64 = *var_qfgd_parasitic_db3_slot;
        let mut var_qfgd_parasitic_db4: f64 = *var_qfgd_parasitic_db4_slot;
        let mut var_qfgd_parasitic_dn0: f64 = *var_qfgd_parasitic_dn0_slot;
        let mut var_qfgd_parasitic_dn1: f64 = *var_qfgd_parasitic_dn1_slot;
        let mut var_qfgd_parasitic_dn2: f64 = *var_qfgd_parasitic_dn2_slot;
        let mut var_qfgd_parasitic_dn3: f64 = *var_qfgd_parasitic_dn3_slot;
        let mut var_qfgd_parasitic_dn4: f64 = *var_qfgd_parasitic_dn4_slot;
        let mut var_qfgd_parasitic_dn5: f64 = *var_qfgd_parasitic_dn5_slot;
        let mut var_qfgd_parasitic_dn6: f64 = *var_qfgd_parasitic_dn6_slot;
        let mut var_qfgd_parasitic_dn7: f64 = *var_qfgd_parasitic_dn7_slot;
        let mut var_qfgd_parasitic_dn8: f64 = *var_qfgd_parasitic_dn8_slot;
        let mut var_qfgd_parasitic_rdb0: f64 = *var_qfgd_parasitic_rdb0_slot;
        let mut var_qfgd_parasitic_rdb1: f64 = *var_qfgd_parasitic_rdb1_slot;
        let mut var_qfgd_parasitic_rdb2: f64 = *var_qfgd_parasitic_rdb2_slot;
        let mut var_qfgd_parasitic_rdb3: f64 = *var_qfgd_parasitic_rdb3_slot;
        let mut var_qfgd_parasitic_rdb4: f64 = *var_qfgd_parasitic_rdb4_slot;
        let mut var_qfgd_parasitic_rdn0: f64 = *var_qfgd_parasitic_rdn0_slot;
        let mut var_qfgd_parasitic_rdn1: f64 = *var_qfgd_parasitic_rdn1_slot;
        let mut var_qfgd_parasitic_rdn2: f64 = *var_qfgd_parasitic_rdn2_slot;
        let mut var_qfgd_parasitic_rdn3: f64 = *var_qfgd_parasitic_rdn3_slot;
        let mut var_qfgd_parasitic_rdn4: f64 = *var_qfgd_parasitic_rdn4_slot;
        let mut var_qfgd_parasitic_rdn5: f64 = *var_qfgd_parasitic_rdn5_slot;
        let mut var_qfgd_parasitic_rdn6: f64 = *var_qfgd_parasitic_rdn6_slot;
        let mut var_qfgd_parasitic_rdn7: f64 = *var_qfgd_parasitic_rdn7_slot;
        let mut var_qfgd_parasitic_rdn8: f64 = *var_qfgd_parasitic_rdn8_slot;
        let mut var_qfgd_parasitic_rv: f64 = *var_qfgd_parasitic_rv_slot;
        let mut var_qfgs_parasitic: f64 = *var_qfgs_parasitic_slot;
        let mut var_qfgs_parasitic_db0: f64 = *var_qfgs_parasitic_db0_slot;
        let mut var_qfgs_parasitic_db1: f64 = *var_qfgs_parasitic_db1_slot;
        let mut var_qfgs_parasitic_db2: f64 = *var_qfgs_parasitic_db2_slot;
        let mut var_qfgs_parasitic_db3: f64 = *var_qfgs_parasitic_db3_slot;
        let mut var_qfgs_parasitic_db4: f64 = *var_qfgs_parasitic_db4_slot;
        let mut var_qfgs_parasitic_dn0: f64 = *var_qfgs_parasitic_dn0_slot;
        let mut var_qfgs_parasitic_dn1: f64 = *var_qfgs_parasitic_dn1_slot;
        let mut var_qfgs_parasitic_dn2: f64 = *var_qfgs_parasitic_dn2_slot;
        let mut var_qfgs_parasitic_dn3: f64 = *var_qfgs_parasitic_dn3_slot;
        let mut var_qfgs_parasitic_dn4: f64 = *var_qfgs_parasitic_dn4_slot;
        let mut var_qfgs_parasitic_dn5: f64 = *var_qfgs_parasitic_dn5_slot;
        let mut var_qfgs_parasitic_dn6: f64 = *var_qfgs_parasitic_dn6_slot;
        let mut var_qfgs_parasitic_dn7: f64 = *var_qfgs_parasitic_dn7_slot;
        let mut var_qfgs_parasitic_dn8: f64 = *var_qfgs_parasitic_dn8_slot;
        let mut var_qfgs_parasitic_rdb0: f64 = *var_qfgs_parasitic_rdb0_slot;
        let mut var_qfgs_parasitic_rdb1: f64 = *var_qfgs_parasitic_rdb1_slot;
        let mut var_qfgs_parasitic_rdb2: f64 = *var_qfgs_parasitic_rdb2_slot;
        let mut var_qfgs_parasitic_rdb3: f64 = *var_qfgs_parasitic_rdb3_slot;
        let mut var_qfgs_parasitic_rdb4: f64 = *var_qfgs_parasitic_rdb4_slot;
        let mut var_qfgs_parasitic_rdn0: f64 = *var_qfgs_parasitic_rdn0_slot;
        let mut var_qfgs_parasitic_rdn1: f64 = *var_qfgs_parasitic_rdn1_slot;
        let mut var_qfgs_parasitic_rdn2: f64 = *var_qfgs_parasitic_rdn2_slot;
        let mut var_qfgs_parasitic_rdn3: f64 = *var_qfgs_parasitic_rdn3_slot;
        let mut var_qfgs_parasitic_rdn4: f64 = *var_qfgs_parasitic_rdn4_slot;
        let mut var_qfgs_parasitic_rdn5: f64 = *var_qfgs_parasitic_rdn5_slot;
        let mut var_qfgs_parasitic_rdn6: f64 = *var_qfgs_parasitic_rdn6_slot;
        let mut var_qfgs_parasitic_rdn7: f64 = *var_qfgs_parasitic_rdn7_slot;
        let mut var_qfgs_parasitic_rdn8: f64 = *var_qfgs_parasitic_rdn8_slot;
        let mut var_qfgs_parasitic_rv: f64 = *var_qfgs_parasitic_rv_slot;
        let mut var_qinv: f64 = *var_qinv_slot;
        let mut var_qinv_db0: f64 = *var_qinv_db0_slot;
        let mut var_qinv_db1: f64 = *var_qinv_db1_slot;
        let mut var_qinv_db2: f64 = *var_qinv_db2_slot;
        let mut var_qinv_db3: f64 = *var_qinv_db3_slot;
        let mut var_qinv_db4: f64 = *var_qinv_db4_slot;
        let mut var_qinv_dn0: f64 = *var_qinv_dn0_slot;
        let mut var_qinv_dn1: f64 = *var_qinv_dn1_slot;
        let mut var_qinv_dn2: f64 = *var_qinv_dn2_slot;
        let mut var_qinv_dn3: f64 = *var_qinv_dn3_slot;
        let mut var_qinv_dn4: f64 = *var_qinv_dn4_slot;
        let mut var_qinv_dn5: f64 = *var_qinv_dn5_slot;
        let mut var_qinv_dn6: f64 = *var_qinv_dn6_slot;
        let mut var_qinv_dn7: f64 = *var_qinv_dn7_slot;
        let mut var_qinv_dn8: f64 = *var_qinv_dn8_slot;
        let mut var_qinv_rdb0: f64 = *var_qinv_rdb0_slot;
        let mut var_qinv_rdb1: f64 = *var_qinv_rdb1_slot;
        let mut var_qinv_rdb2: f64 = *var_qinv_rdb2_slot;
        let mut var_qinv_rdb3: f64 = *var_qinv_rdb3_slot;
        let mut var_qinv_rdb4: f64 = *var_qinv_rdb4_slot;
        let mut var_qinv_rdn0: f64 = *var_qinv_rdn0_slot;
        let mut var_qinv_rdn1: f64 = *var_qinv_rdn1_slot;
        let mut var_qinv_rdn2: f64 = *var_qinv_rdn2_slot;
        let mut var_qinv_rdn3: f64 = *var_qinv_rdn3_slot;
        let mut var_qinv_rdn4: f64 = *var_qinv_rdn4_slot;
        let mut var_qinv_rdn5: f64 = *var_qinv_rdn5_slot;
        let mut var_qinv_rdn6: f64 = *var_qinv_rdn6_slot;
        let mut var_qinv_rdn7: f64 = *var_qinv_rdn7_slot;
        let mut var_qinv_rdn8: f64 = *var_qinv_rdn8_slot;
        let mut var_qinv_rv: f64 = *var_qinv_rv_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_db0: f64 = *var_qs_db0_slot;
        let mut var_qs_db1: f64 = *var_qs_db1_slot;
        let mut var_qs_db2: f64 = *var_qs_db2_slot;
        let mut var_qs_db3: f64 = *var_qs_db3_slot;
        let mut var_qs_db4: f64 = *var_qs_db4_slot;
        let mut var_qs_dn0: f64 = *var_qs_dn0_slot;
        let mut var_qs_dn1: f64 = *var_qs_dn1_slot;
        let mut var_qs_dn2: f64 = *var_qs_dn2_slot;
        let mut var_qs_dn3: f64 = *var_qs_dn3_slot;
        let mut var_qs_dn4: f64 = *var_qs_dn4_slot;
        let mut var_qs_dn5: f64 = *var_qs_dn5_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_qs_rdb0: f64 = *var_qs_rdb0_slot;
        let mut var_qs_rdb1: f64 = *var_qs_rdb1_slot;
        let mut var_qs_rdb2: f64 = *var_qs_rdb2_slot;
        let mut var_qs_rdb3: f64 = *var_qs_rdb3_slot;
        let mut var_qs_rdb4: f64 = *var_qs_rdb4_slot;
        let mut var_qs_rdn0: f64 = *var_qs_rdn0_slot;
        let mut var_qs_rdn1: f64 = *var_qs_rdn1_slot;
        let mut var_qs_rdn2: f64 = *var_qs_rdn2_slot;
        let mut var_qs_rdn3: f64 = *var_qs_rdn3_slot;
        let mut var_qs_rdn4: f64 = *var_qs_rdn4_slot;
        let mut var_qs_rdn5: f64 = *var_qs_rdn5_slot;
        let mut var_qs_rdn6: f64 = *var_qs_rdn6_slot;
        let mut var_qs_rdn7: f64 = *var_qs_rdn7_slot;
        let mut var_qs_rdn8: f64 = *var_qs_rdn8_slot;
        let mut var_qs_rv: f64 = *var_qs_rv_slot;
        let mut var_qsi: f64 = *var_qsi_slot;
        let mut var_qsi_db0: f64 = *var_qsi_db0_slot;
        let mut var_qsi_db1: f64 = *var_qsi_db1_slot;
        let mut var_qsi_db2: f64 = *var_qsi_db2_slot;
        let mut var_qsi_db3: f64 = *var_qsi_db3_slot;
        let mut var_qsi_db4: f64 = *var_qsi_db4_slot;
        let mut var_qsi_dn0: f64 = *var_qsi_dn0_slot;
        let mut var_qsi_dn1: f64 = *var_qsi_dn1_slot;
        let mut var_qsi_dn2: f64 = *var_qsi_dn2_slot;
        let mut var_qsi_dn3: f64 = *var_qsi_dn3_slot;
        let mut var_qsi_dn4: f64 = *var_qsi_dn4_slot;
        let mut var_qsi_dn5: f64 = *var_qsi_dn5_slot;
        let mut var_qsi_dn6: f64 = *var_qsi_dn6_slot;
        let mut var_qsi_dn7: f64 = *var_qsi_dn7_slot;
        let mut var_qsi_dn8: f64 = *var_qsi_dn8_slot;
        let mut var_qsi_rdb0: f64 = *var_qsi_rdb0_slot;
        let mut var_qsi_rdb1: f64 = *var_qsi_rdb1_slot;
        let mut var_qsi_rdb2: f64 = *var_qsi_rdb2_slot;
        let mut var_qsi_rdb3: f64 = *var_qsi_rdb3_slot;
        let mut var_qsi_rdb4: f64 = *var_qsi_rdb4_slot;
        let mut var_qsi_rdn0: f64 = *var_qsi_rdn0_slot;
        let mut var_qsi_rdn1: f64 = *var_qsi_rdn1_slot;
        let mut var_qsi_rdn2: f64 = *var_qsi_rdn2_slot;
        let mut var_qsi_rdn3: f64 = *var_qsi_rdn3_slot;
        let mut var_qsi_rdn4: f64 = *var_qsi_rdn4_slot;
        let mut var_qsi_rdn5: f64 = *var_qsi_rdn5_slot;
        let mut var_qsi_rdn6: f64 = *var_qsi_rdn6_slot;
        let mut var_qsi_rdn7: f64 = *var_qsi_rdn7_slot;
        let mut var_qsi_rdn8: f64 = *var_qsi_rdn8_slot;
        let mut var_qsi_rv: f64 = *var_qsi_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_db0: f64 = *var_t0_db0_slot;
        let mut var_t0_db1: f64 = *var_t0_db1_slot;
        let mut var_t0_db2: f64 = *var_t0_db2_slot;
        let mut var_t0_db3: f64 = *var_t0_db3_slot;
        let mut var_t0_db4: f64 = *var_t0_db4_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rdb0: f64 = *var_t0_rdb0_slot;
        let mut var_t0_rdb1: f64 = *var_t0_rdb1_slot;
        let mut var_t0_rdb2: f64 = *var_t0_rdb2_slot;
        let mut var_t0_rdb3: f64 = *var_t0_rdb3_slot;
        let mut var_t0_rdb4: f64 = *var_t0_rdb4_slot;
        let mut var_t0_rdn0: f64 = *var_t0_rdn0_slot;
        let mut var_t0_rdn1: f64 = *var_t0_rdn1_slot;
        let mut var_t0_rdn2: f64 = *var_t0_rdn2_slot;
        let mut var_t0_rdn3: f64 = *var_t0_rdn3_slot;
        let mut var_t0_rdn4: f64 = *var_t0_rdn4_slot;
        let mut var_t0_rdn5: f64 = *var_t0_rdn5_slot;
        let mut var_t0_rdn6: f64 = *var_t0_rdn6_slot;
        let mut var_t0_rdn7: f64 = *var_t0_rdn7_slot;
        let mut var_t0_rdn8: f64 = *var_t0_rdn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_db0: f64 = *var_t1_db0_slot;
        let mut var_t1_db1: f64 = *var_t1_db1_slot;
        let mut var_t1_db2: f64 = *var_t1_db2_slot;
        let mut var_t1_db3: f64 = *var_t1_db3_slot;
        let mut var_t1_db4: f64 = *var_t1_db4_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rdb0: f64 = *var_t1_rdb0_slot;
        let mut var_t1_rdb1: f64 = *var_t1_rdb1_slot;
        let mut var_t1_rdb2: f64 = *var_t1_rdb2_slot;
        let mut var_t1_rdb3: f64 = *var_t1_rdb3_slot;
        let mut var_t1_rdb4: f64 = *var_t1_rdb4_slot;
        let mut var_t1_rdn0: f64 = *var_t1_rdn0_slot;
        let mut var_t1_rdn1: f64 = *var_t1_rdn1_slot;
        let mut var_t1_rdn2: f64 = *var_t1_rdn2_slot;
        let mut var_t1_rdn3: f64 = *var_t1_rdn3_slot;
        let mut var_t1_rdn4: f64 = *var_t1_rdn4_slot;
        let mut var_t1_rdn5: f64 = *var_t1_rdn5_slot;
        let mut var_t1_rdn6: f64 = *var_t1_rdn6_slot;
        let mut var_t1_rdn7: f64 = *var_t1_rdn7_slot;
        let mut var_t1_rdn8: f64 = *var_t1_rdn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;

        let (assign13120_e12182, assign13120_e12182_d_n0, assign13120_e12182_d_n1, assign13120_e12182_d_n2, assign13120_e12182_d_n3, assign13120_e12182_d_n4, assign13120_e12182_d_n5, assign13120_e12182_d_n6, assign13120_e12182_d_n7, assign13120_e12182_d_n8, assign13120_e12182_d_b0, assign13120_e12182_d_b1, assign13120_e12182_d_b2, assign13120_e12182_d_b3, assign13120_e12182_d_b4,) = {
    if (var_guard136 != 0.0) {
        let assign13120_e12177: f64 = (var_qs - var_qfgs_parasitic);
        let assign13120_e12178: f64 = (p.p2 * assign13120_e12177);
        let assign13120_e12180: f64 = (assign13120_e12178 + var_qsbg);
        (assign13120_e12180, ((p.p2 * (var_qs_dn0 - var_qfgs_parasitic_dn0)) + var_qsbg_dn0), ((p.p2 * (var_qs_dn1 - var_qfgs_parasitic_dn1)) + var_qsbg_dn1), ((p.p2 * (var_qs_dn2 - var_qfgs_parasitic_dn2)) + var_qsbg_dn2), ((p.p2 * (var_qs_dn3 - var_qfgs_parasitic_dn3)) + var_qsbg_dn3), ((p.p2 * (var_qs_dn4 - var_qfgs_parasitic_dn4)) + var_qsbg_dn4), ((p.p2 * (var_qs_dn5 - var_qfgs_parasitic_dn5)) + var_qsbg_dn5), ((p.p2 * (var_qs_dn6 - var_qfgs_parasitic_dn6)) + var_qsbg_dn6), ((p.p2 * (var_qs_dn7 - var_qfgs_parasitic_dn7)) + var_qsbg_dn7), ((p.p2 * (var_qs_dn8 - var_qfgs_parasitic_dn8)) + var_qsbg_dn8), ((p.p2 * (var_qs_db0 - var_qfgs_parasitic_db0)) + var_qsbg_db0), ((p.p2 * (var_qs_db1 - var_qfgs_parasitic_db1)) + var_qsbg_db1), ((p.p2 * (var_qs_db2 - var_qfgs_parasitic_db2)) + var_qsbg_db2), ((p.p2 * (var_qs_db3 - var_qfgs_parasitic_db3)) + var_qsbg_db3), ((p.p2 * (var_qs_db4 - var_qfgs_parasitic_db4)) + var_qsbg_db4),)
    } else {
        (var_qs, var_qs_dn0, var_qs_dn1, var_qs_dn2, var_qs_dn3, var_qs_dn4, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_db0, var_qs_db1, var_qs_db2, var_qs_db3, var_qs_db4,)
    }
};
        var_qs = assign13120_e12182;
        var_qs_dn0 = assign13120_e12182_d_n0;
        var_qs_dn1 = assign13120_e12182_d_n1;
        var_qs_dn2 = assign13120_e12182_d_n2;
        var_qs_dn3 = assign13120_e12182_d_n3;
        var_qs_dn4 = assign13120_e12182_d_n4;
        var_qs_dn5 = assign13120_e12182_d_n5;
        var_qs_dn6 = assign13120_e12182_d_n6;
        var_qs_dn7 = assign13120_e12182_d_n7;
        var_qs_dn8 = assign13120_e12182_d_n8;
        var_qs_db0 = assign13120_e12182_d_b0;
        var_qs_db1 = assign13120_e12182_d_b1;
        var_qs_db2 = assign13120_e12182_d_b2;
        var_qs_db3 = assign13120_e12182_d_b3;
        var_qs_db4 = assign13120_e12182_d_b4;
        var_qs_rv = 0.0;
        var_qs_rdn0 = 0.0;
        var_qs_rdn1 = 0.0;
        var_qs_rdn2 = 0.0;
        var_qs_rdn3 = 0.0;
        var_qs_rdn4 = 0.0;
        var_qs_rdn5 = 0.0;
        var_qs_rdn6 = 0.0;
        var_qs_rdn7 = 0.0;
        var_qs_rdn8 = 0.0;
        var_qs_rdb0 = 0.0;
        var_qs_rdb1 = 0.0;
        var_qs_rdb2 = 0.0;
        var_qs_rdb3 = 0.0;
        var_qs_rdb4 = 0.0;

        let (assign13130_e12192, assign13130_e12192_d_n0, assign13130_e12192_d_n1, assign13130_e12192_d_n2, assign13130_e12192_d_n3, assign13130_e12192_d_n4, assign13130_e12192_d_n5, assign13130_e12192_d_n6, assign13130_e12192_d_n7, assign13130_e12192_d_n8, assign13130_e12192_d_b0, assign13130_e12192_d_b1, assign13130_e12192_d_b2, assign13130_e12192_d_b3, assign13130_e12192_d_b4,) = {
    if (var_guard136 != 0.0) {
        let assign13130_e12187: f64 = (var_qd - var_qfgd_parasitic);
        let assign13130_e12188: f64 = (p.p2 * assign13130_e12187);
        let assign13130_e12190: f64 = (assign13130_e12188 + var_qdbg);
        (assign13130_e12190, ((p.p2 * (var_qd_dn0 - var_qfgd_parasitic_dn0)) + var_qdbg_dn0), ((p.p2 * (var_qd_dn1 - var_qfgd_parasitic_dn1)) + var_qdbg_dn1), ((p.p2 * (var_qd_dn2 - var_qfgd_parasitic_dn2)) + var_qdbg_dn2), ((p.p2 * (var_qd_dn3 - var_qfgd_parasitic_dn3)) + var_qdbg_dn3), ((p.p2 * (var_qd_dn4 - var_qfgd_parasitic_dn4)) + var_qdbg_dn4), ((p.p2 * (var_qd_dn5 - var_qfgd_parasitic_dn5)) + var_qdbg_dn5), ((p.p2 * (var_qd_dn6 - var_qfgd_parasitic_dn6)) + var_qdbg_dn6), ((p.p2 * (var_qd_dn7 - var_qfgd_parasitic_dn7)) + var_qdbg_dn7), ((p.p2 * (var_qd_dn8 - var_qfgd_parasitic_dn8)) + var_qdbg_dn8), ((p.p2 * (var_qd_db0 - var_qfgd_parasitic_db0)) + var_qdbg_db0), ((p.p2 * (var_qd_db1 - var_qfgd_parasitic_db1)) + var_qdbg_db1), ((p.p2 * (var_qd_db2 - var_qfgd_parasitic_db2)) + var_qdbg_db2), ((p.p2 * (var_qd_db3 - var_qfgd_parasitic_db3)) + var_qdbg_db3), ((p.p2 * (var_qd_db4 - var_qfgd_parasitic_db4)) + var_qdbg_db4),)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn1, var_qd_dn2, var_qd_dn3, var_qd_dn4, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_db0, var_qd_db1, var_qd_db2, var_qd_db3, var_qd_db4,)
    }
};
        var_qd = assign13130_e12192;
        var_qd_dn0 = assign13130_e12192_d_n0;
        var_qd_dn1 = assign13130_e12192_d_n1;
        var_qd_dn2 = assign13130_e12192_d_n2;
        var_qd_dn3 = assign13130_e12192_d_n3;
        var_qd_dn4 = assign13130_e12192_d_n4;
        var_qd_dn5 = assign13130_e12192_d_n5;
        var_qd_dn6 = assign13130_e12192_d_n6;
        var_qd_dn7 = assign13130_e12192_d_n7;
        var_qd_dn8 = assign13130_e12192_d_n8;
        var_qd_db0 = assign13130_e12192_d_b0;
        var_qd_db1 = assign13130_e12192_d_b1;
        var_qd_db2 = assign13130_e12192_d_b2;
        var_qd_db3 = assign13130_e12192_d_b3;
        var_qd_db4 = assign13130_e12192_d_b4;
        var_qd_rv = 0.0;
        var_qd_rdn0 = 0.0;
        var_qd_rdn1 = 0.0;
        var_qd_rdn2 = 0.0;
        var_qd_rdn3 = 0.0;
        var_qd_rdn4 = 0.0;
        var_qd_rdn5 = 0.0;
        var_qd_rdn6 = 0.0;
        var_qd_rdn7 = 0.0;
        var_qd_rdn8 = 0.0;
        var_qd_rdb0 = 0.0;
        var_qd_rdb1 = 0.0;
        var_qd_rdb2 = 0.0;
        var_qd_rdb3 = 0.0;
        var_qd_rdb4 = 0.0;

        let (assign13140_e12199, assign13140_e12199_d_n0, assign13140_e12199_d_n1, assign13140_e12199_d_n2, assign13140_e12199_d_n3, assign13140_e12199_d_n4, assign13140_e12199_d_n5, assign13140_e12199_d_n6, assign13140_e12199_d_n7, assign13140_e12199_d_n8, assign13140_e12199_d_b0, assign13140_e12199_d_b1, assign13140_e12199_d_b2, assign13140_e12199_d_b3, assign13140_e12199_d_b4,) = {
    if (var_guard136 == 0.0) {
        let assign13140_e12197: f64 = (p.p2 * var_qd);
        (assign13140_e12197, (p.p2 * var_qd_dn0), (p.p2 * var_qd_dn1), (p.p2 * var_qd_dn2), (p.p2 * var_qd_dn3), (p.p2 * var_qd_dn4), (p.p2 * var_qd_dn5), (p.p2 * var_qd_dn6), (p.p2 * var_qd_dn7), (p.p2 * var_qd_dn8), (p.p2 * var_qd_db0), (p.p2 * var_qd_db1), (p.p2 * var_qd_db2), (p.p2 * var_qd_db3), (p.p2 * var_qd_db4),)
    } else {
        (var_qsi, var_qsi_dn0, var_qsi_dn1, var_qsi_dn2, var_qsi_dn3, var_qsi_dn4, var_qsi_dn5, var_qsi_dn6, var_qsi_dn7, var_qsi_dn8, var_qsi_db0, var_qsi_db1, var_qsi_db2, var_qsi_db3, var_qsi_db4,)
    }
};
        var_qsi = assign13140_e12199;
        var_qsi_dn0 = assign13140_e12199_d_n0;
        var_qsi_dn1 = assign13140_e12199_d_n1;
        var_qsi_dn2 = assign13140_e12199_d_n2;
        var_qsi_dn3 = assign13140_e12199_d_n3;
        var_qsi_dn4 = assign13140_e12199_d_n4;
        var_qsi_dn5 = assign13140_e12199_d_n5;
        var_qsi_dn6 = assign13140_e12199_d_n6;
        var_qsi_dn7 = assign13140_e12199_d_n7;
        var_qsi_dn8 = assign13140_e12199_d_n8;
        var_qsi_db0 = assign13140_e12199_d_b0;
        var_qsi_db1 = assign13140_e12199_d_b1;
        var_qsi_db2 = assign13140_e12199_d_b2;
        var_qsi_db3 = assign13140_e12199_d_b3;
        var_qsi_db4 = assign13140_e12199_d_b4;
        var_qsi_rv = 0.0;
        var_qsi_rdn0 = 0.0;
        var_qsi_rdn1 = 0.0;
        var_qsi_rdn2 = 0.0;
        var_qsi_rdn3 = 0.0;
        var_qsi_rdn4 = 0.0;
        var_qsi_rdn5 = 0.0;
        var_qsi_rdn6 = 0.0;
        var_qsi_rdn7 = 0.0;
        var_qsi_rdn8 = 0.0;
        var_qsi_rdb0 = 0.0;
        var_qsi_rdb1 = 0.0;
        var_qsi_rdb2 = 0.0;
        var_qsi_rdb3 = 0.0;
        var_qsi_rdb4 = 0.0;

        let (assign13150_e12206, assign13150_e12206_d_n0, assign13150_e12206_d_n1, assign13150_e12206_d_n2, assign13150_e12206_d_n3, assign13150_e12206_d_n4, assign13150_e12206_d_n5, assign13150_e12206_d_n6, assign13150_e12206_d_n7, assign13150_e12206_d_n8, assign13150_e12206_d_b0, assign13150_e12206_d_b1, assign13150_e12206_d_b2, assign13150_e12206_d_b3, assign13150_e12206_d_b4,) = {
    if (var_guard136 == 0.0) {
        let assign13150_e12204: f64 = (p.p2 * var_qs);
        (assign13150_e12204, (p.p2 * var_qs_dn0), (p.p2 * var_qs_dn1), (p.p2 * var_qs_dn2), (p.p2 * var_qs_dn3), (p.p2 * var_qs_dn4), (p.p2 * var_qs_dn5), (p.p2 * var_qs_dn6), (p.p2 * var_qs_dn7), (p.p2 * var_qs_dn8), (p.p2 * var_qs_db0), (p.p2 * var_qs_db1), (p.p2 * var_qs_db2), (p.p2 * var_qs_db3), (p.p2 * var_qs_db4),)
    } else {
        (var_qdi, var_qdi_dn0, var_qdi_dn1, var_qdi_dn2, var_qdi_dn3, var_qdi_dn4, var_qdi_dn5, var_qdi_dn6, var_qdi_dn7, var_qdi_dn8, var_qdi_db0, var_qdi_db1, var_qdi_db2, var_qdi_db3, var_qdi_db4,)
    }
};
        var_qdi = assign13150_e12206;
        var_qdi_dn0 = assign13150_e12206_d_n0;
        var_qdi_dn1 = assign13150_e12206_d_n1;
        var_qdi_dn2 = assign13150_e12206_d_n2;
        var_qdi_dn3 = assign13150_e12206_d_n3;
        var_qdi_dn4 = assign13150_e12206_d_n4;
        var_qdi_dn5 = assign13150_e12206_d_n5;
        var_qdi_dn6 = assign13150_e12206_d_n6;
        var_qdi_dn7 = assign13150_e12206_d_n7;
        var_qdi_dn8 = assign13150_e12206_d_n8;
        var_qdi_db0 = assign13150_e12206_d_b0;
        var_qdi_db1 = assign13150_e12206_d_b1;
        var_qdi_db2 = assign13150_e12206_d_b2;
        var_qdi_db3 = assign13150_e12206_d_b3;
        var_qdi_db4 = assign13150_e12206_d_b4;
        var_qdi_rv = 0.0;
        var_qdi_rdn0 = 0.0;
        var_qdi_rdn1 = 0.0;
        var_qdi_rdn2 = 0.0;
        var_qdi_rdn3 = 0.0;
        var_qdi_rdn4 = 0.0;
        var_qdi_rdn5 = 0.0;
        var_qdi_rdn6 = 0.0;
        var_qdi_rdn7 = 0.0;
        var_qdi_rdn8 = 0.0;
        var_qdi_rdb0 = 0.0;
        var_qdi_rdb1 = 0.0;
        var_qdi_rdb2 = 0.0;
        var_qdi_rdb3 = 0.0;
        var_qdi_rdb4 = 0.0;

        let (assign13160_e12217, assign13160_e12217_d_n0, assign13160_e12217_d_n1, assign13160_e12217_d_n2, assign13160_e12217_d_n3, assign13160_e12217_d_n4, assign13160_e12217_d_n5, assign13160_e12217_d_n6, assign13160_e12217_d_n7, assign13160_e12217_d_n8, assign13160_e12217_d_b0, assign13160_e12217_d_b1, assign13160_e12217_d_b2, assign13160_e12217_d_b3, assign13160_e12217_d_b4,) = {
    if (var_guard136 == 0.0) {
        let assign13160_e12212: f64 = (var_qd - var_qfgs_parasitic);
        let assign13160_e12213: f64 = (p.p2 * assign13160_e12212);
        let assign13160_e12215: f64 = (assign13160_e12213 + var_qsbg);
        (assign13160_e12215, ((p.p2 * (var_qd_dn0 - var_qfgs_parasitic_dn0)) + var_qsbg_dn0), ((p.p2 * (var_qd_dn1 - var_qfgs_parasitic_dn1)) + var_qsbg_dn1), ((p.p2 * (var_qd_dn2 - var_qfgs_parasitic_dn2)) + var_qsbg_dn2), ((p.p2 * (var_qd_dn3 - var_qfgs_parasitic_dn3)) + var_qsbg_dn3), ((p.p2 * (var_qd_dn4 - var_qfgs_parasitic_dn4)) + var_qsbg_dn4), ((p.p2 * (var_qd_dn5 - var_qfgs_parasitic_dn5)) + var_qsbg_dn5), ((p.p2 * (var_qd_dn6 - var_qfgs_parasitic_dn6)) + var_qsbg_dn6), ((p.p2 * (var_qd_dn7 - var_qfgs_parasitic_dn7)) + var_qsbg_dn7), ((p.p2 * (var_qd_dn8 - var_qfgs_parasitic_dn8)) + var_qsbg_dn8), ((p.p2 * (var_qd_db0 - var_qfgs_parasitic_db0)) + var_qsbg_db0), ((p.p2 * (var_qd_db1 - var_qfgs_parasitic_db1)) + var_qsbg_db1), ((p.p2 * (var_qd_db2 - var_qfgs_parasitic_db2)) + var_qsbg_db2), ((p.p2 * (var_qd_db3 - var_qfgs_parasitic_db3)) + var_qsbg_db3), ((p.p2 * (var_qd_db4 - var_qfgs_parasitic_db4)) + var_qsbg_db4),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4,)
    }
};
        var_t0 = assign13160_e12217;
        var_t0_dn0 = assign13160_e12217_d_n0;
        var_t0_dn1 = assign13160_e12217_d_n1;
        var_t0_dn2 = assign13160_e12217_d_n2;
        var_t0_dn3 = assign13160_e12217_d_n3;
        var_t0_dn4 = assign13160_e12217_d_n4;
        var_t0_dn5 = assign13160_e12217_d_n5;
        var_t0_dn6 = assign13160_e12217_d_n6;
        var_t0_dn7 = assign13160_e12217_d_n7;
        var_t0_dn8 = assign13160_e12217_d_n8;
        var_t0_db0 = assign13160_e12217_d_b0;
        var_t0_db1 = assign13160_e12217_d_b1;
        var_t0_db2 = assign13160_e12217_d_b2;
        var_t0_db3 = assign13160_e12217_d_b3;
        var_t0_db4 = assign13160_e12217_d_b4;
        var_t0_rv = 0.0;
        var_t0_rdn0 = 0.0;
        var_t0_rdn1 = 0.0;
        var_t0_rdn2 = 0.0;
        var_t0_rdn3 = 0.0;
        var_t0_rdn4 = 0.0;
        var_t0_rdn5 = 0.0;
        var_t0_rdn6 = 0.0;
        var_t0_rdn7 = 0.0;
        var_t0_rdn8 = 0.0;
        var_t0_rdb0 = 0.0;
        var_t0_rdb1 = 0.0;
        var_t0_rdb2 = 0.0;
        var_t0_rdb3 = 0.0;
        var_t0_rdb4 = 0.0;

        let (assign13170_e12228, assign13170_e12228_d_n0, assign13170_e12228_d_n1, assign13170_e12228_d_n2, assign13170_e12228_d_n3, assign13170_e12228_d_n4, assign13170_e12228_d_n5, assign13170_e12228_d_n6, assign13170_e12228_d_n7, assign13170_e12228_d_n8, assign13170_e12228_d_b0, assign13170_e12228_d_b1, assign13170_e12228_d_b2, assign13170_e12228_d_b3, assign13170_e12228_d_b4,) = {
    if (var_guard136 == 0.0) {
        let assign13170_e12223: f64 = (var_qs - var_qfgd_parasitic);
        let assign13170_e12224: f64 = (p.p2 * assign13170_e12223);
        let assign13170_e12226: f64 = (assign13170_e12224 + var_qdbg);
        (assign13170_e12226, ((p.p2 * (var_qs_dn0 - var_qfgd_parasitic_dn0)) + var_qdbg_dn0), ((p.p2 * (var_qs_dn1 - var_qfgd_parasitic_dn1)) + var_qdbg_dn1), ((p.p2 * (var_qs_dn2 - var_qfgd_parasitic_dn2)) + var_qdbg_dn2), ((p.p2 * (var_qs_dn3 - var_qfgd_parasitic_dn3)) + var_qdbg_dn3), ((p.p2 * (var_qs_dn4 - var_qfgd_parasitic_dn4)) + var_qdbg_dn4), ((p.p2 * (var_qs_dn5 - var_qfgd_parasitic_dn5)) + var_qdbg_dn5), ((p.p2 * (var_qs_dn6 - var_qfgd_parasitic_dn6)) + var_qdbg_dn6), ((p.p2 * (var_qs_dn7 - var_qfgd_parasitic_dn7)) + var_qdbg_dn7), ((p.p2 * (var_qs_dn8 - var_qfgd_parasitic_dn8)) + var_qdbg_dn8), ((p.p2 * (var_qs_db0 - var_qfgd_parasitic_db0)) + var_qdbg_db0), ((p.p2 * (var_qs_db1 - var_qfgd_parasitic_db1)) + var_qdbg_db1), ((p.p2 * (var_qs_db2 - var_qfgd_parasitic_db2)) + var_qdbg_db2), ((p.p2 * (var_qs_db3 - var_qfgd_parasitic_db3)) + var_qdbg_db3), ((p.p2 * (var_qs_db4 - var_qfgd_parasitic_db4)) + var_qdbg_db4),)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn1, var_qd_dn2, var_qd_dn3, var_qd_dn4, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_db0, var_qd_db1, var_qd_db2, var_qd_db3, var_qd_db4,)
    }
};
        var_qd = assign13170_e12228;
        var_qd_dn0 = assign13170_e12228_d_n0;
        var_qd_dn1 = assign13170_e12228_d_n1;
        var_qd_dn2 = assign13170_e12228_d_n2;
        var_qd_dn3 = assign13170_e12228_d_n3;
        var_qd_dn4 = assign13170_e12228_d_n4;
        var_qd_dn5 = assign13170_e12228_d_n5;
        var_qd_dn6 = assign13170_e12228_d_n6;
        var_qd_dn7 = assign13170_e12228_d_n7;
        var_qd_dn8 = assign13170_e12228_d_n8;
        var_qd_db0 = assign13170_e12228_d_b0;
        var_qd_db1 = assign13170_e12228_d_b1;
        var_qd_db2 = assign13170_e12228_d_b2;
        var_qd_db3 = assign13170_e12228_d_b3;
        var_qd_db4 = assign13170_e12228_d_b4;
        var_qd_rv = 0.0;
        var_qd_rdn0 = 0.0;
        var_qd_rdn1 = 0.0;
        var_qd_rdn2 = 0.0;
        var_qd_rdn3 = 0.0;
        var_qd_rdn4 = 0.0;
        var_qd_rdn5 = 0.0;
        var_qd_rdn6 = 0.0;
        var_qd_rdn7 = 0.0;
        var_qd_rdn8 = 0.0;
        var_qd_rdb0 = 0.0;
        var_qd_rdb1 = 0.0;
        var_qd_rdb2 = 0.0;
        var_qd_rdb3 = 0.0;
        var_qd_rdb4 = 0.0;

        let (assign13180_e12233, assign13180_e12233_d_n0, assign13180_e12233_d_n1, assign13180_e12233_d_n2, assign13180_e12233_d_n3, assign13180_e12233_d_n4, assign13180_e12233_d_n5, assign13180_e12233_d_n6, assign13180_e12233_d_n7, assign13180_e12233_d_n8, assign13180_e12233_d_b0, assign13180_e12233_d_b1, assign13180_e12233_d_b2, assign13180_e12233_d_b3, assign13180_e12233_d_b4,) = {
    if (var_guard136 == 0.0) {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4,)
    } else {
        (var_qs, var_qs_dn0, var_qs_dn1, var_qs_dn2, var_qs_dn3, var_qs_dn4, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_db0, var_qs_db1, var_qs_db2, var_qs_db3, var_qs_db4,)
    }
};
        var_qs = assign13180_e12233;
        var_qs_dn0 = assign13180_e12233_d_n0;
        var_qs_dn1 = assign13180_e12233_d_n1;
        var_qs_dn2 = assign13180_e12233_d_n2;
        var_qs_dn3 = assign13180_e12233_d_n3;
        var_qs_dn4 = assign13180_e12233_d_n4;
        var_qs_dn5 = assign13180_e12233_d_n5;
        var_qs_dn6 = assign13180_e12233_d_n6;
        var_qs_dn7 = assign13180_e12233_d_n7;
        var_qs_dn8 = assign13180_e12233_d_n8;
        var_qs_db0 = assign13180_e12233_d_b0;
        var_qs_db1 = assign13180_e12233_d_b1;
        var_qs_db2 = assign13180_e12233_d_b2;
        var_qs_db3 = assign13180_e12233_d_b3;
        var_qs_db4 = assign13180_e12233_d_b4;
        var_qs_rv = 0.0;
        var_qs_rdn0 = 0.0;
        var_qs_rdn1 = 0.0;
        var_qs_rdn2 = 0.0;
        var_qs_rdn3 = 0.0;
        var_qs_rdn4 = 0.0;
        var_qs_rdn5 = 0.0;
        var_qs_rdn6 = 0.0;
        var_qs_rdn7 = 0.0;
        var_qs_rdn8 = 0.0;
        var_qs_rdb0 = 0.0;
        var_qs_rdb1 = 0.0;
        var_qs_rdb2 = 0.0;
        var_qs_rdb3 = 0.0;
        var_qs_rdb4 = 0.0;

        let assign13190_e12238: f64 = (var_qfgs_parasitic + var_qfgd_parasitic);
        let assign13190_e12239: f64 = (p.p2 * assign13190_e12238);
        let assign13190_e12240: f64 = (var_qfgi + assign13190_e12239);
        var_qfg = assign13190_e12240;
        var_qfg_dn0 = (var_qfgi_dn0 + (p.p2 * (var_qfgs_parasitic_dn0 + var_qfgd_parasitic_dn0)));
        var_qfg_dn1 = (var_qfgi_dn1 + (p.p2 * (var_qfgs_parasitic_dn1 + var_qfgd_parasitic_dn1)));
        var_qfg_dn2 = (var_qfgi_dn2 + (p.p2 * (var_qfgs_parasitic_dn2 + var_qfgd_parasitic_dn2)));
        var_qfg_dn3 = (var_qfgi_dn3 + (p.p2 * (var_qfgs_parasitic_dn3 + var_qfgd_parasitic_dn3)));
        var_qfg_dn4 = (var_qfgi_dn4 + (p.p2 * (var_qfgs_parasitic_dn4 + var_qfgd_parasitic_dn4)));
        var_qfg_dn5 = (var_qfgi_dn5 + (p.p2 * (var_qfgs_parasitic_dn5 + var_qfgd_parasitic_dn5)));
        var_qfg_dn6 = (var_qfgi_dn6 + (p.p2 * (var_qfgs_parasitic_dn6 + var_qfgd_parasitic_dn6)));
        var_qfg_dn7 = (var_qfgi_dn7 + (p.p2 * (var_qfgs_parasitic_dn7 + var_qfgd_parasitic_dn7)));
        var_qfg_dn8 = (var_qfgi_dn8 + (p.p2 * (var_qfgs_parasitic_dn8 + var_qfgd_parasitic_dn8)));
        var_qfg_db0 = (var_qfgi_db0 + (p.p2 * (var_qfgs_parasitic_db0 + var_qfgd_parasitic_db0)));
        var_qfg_db1 = (var_qfgi_db1 + (p.p2 * (var_qfgs_parasitic_db1 + var_qfgd_parasitic_db1)));
        var_qfg_db2 = (var_qfgi_db2 + (p.p2 * (var_qfgs_parasitic_db2 + var_qfgd_parasitic_db2)));
        var_qfg_db3 = (var_qfgi_db3 + (p.p2 * (var_qfgs_parasitic_db3 + var_qfgd_parasitic_db3)));
        var_qfg_db4 = (var_qfgi_db4 + (p.p2 * (var_qfgs_parasitic_db4 + var_qfgd_parasitic_db4)));
        var_qfg_rv = 0.0;
        var_qfg_rdn0 = 0.0;
        var_qfg_rdn1 = 0.0;
        var_qfg_rdn2 = 0.0;
        var_qfg_rdn3 = 0.0;
        var_qfg_rdn4 = 0.0;
        var_qfg_rdn5 = 0.0;
        var_qfg_rdn6 = 0.0;
        var_qfg_rdn7 = 0.0;
        var_qfg_rdn8 = 0.0;
        var_qfg_rdb0 = 0.0;
        var_qfg_rdb1 = 0.0;
        var_qfg_rdb2 = 0.0;
        var_qfg_rdb3 = 0.0;
        var_qfg_rdb4 = 0.0;

        let assign13200_e12243: f64 = (p.p2 * var_qbg);
        let assign13200_e12245: f64 = (assign13200_e12243 - var_qsbg);
        let assign13200_e12247: f64 = (assign13200_e12245 - var_qdbg);
        var_qbg = assign13200_e12247;
        var_qbg_dn0 = (((p.p2 * var_qbg_dn0) - var_qsbg_dn0) - var_qdbg_dn0);
        var_qbg_dn1 = (((p.p2 * var_qbg_dn1) - var_qsbg_dn1) - var_qdbg_dn1);
        var_qbg_dn2 = (((p.p2 * var_qbg_dn2) - var_qsbg_dn2) - var_qdbg_dn2);
        var_qbg_dn3 = (((p.p2 * var_qbg_dn3) - var_qsbg_dn3) - var_qdbg_dn3);
        var_qbg_dn4 = (((p.p2 * var_qbg_dn4) - var_qsbg_dn4) - var_qdbg_dn4);
        var_qbg_dn5 = (((p.p2 * var_qbg_dn5) - var_qsbg_dn5) - var_qdbg_dn5);
        var_qbg_dn6 = (((p.p2 * var_qbg_dn6) - var_qsbg_dn6) - var_qdbg_dn6);
        var_qbg_dn7 = (((p.p2 * var_qbg_dn7) - var_qsbg_dn7) - var_qdbg_dn7);
        var_qbg_dn8 = (((p.p2 * var_qbg_dn8) - var_qsbg_dn8) - var_qdbg_dn8);
        var_qbg_db0 = (((p.p2 * var_qbg_db0) - var_qsbg_db0) - var_qdbg_db0);
        var_qbg_db1 = (((p.p2 * var_qbg_db1) - var_qsbg_db1) - var_qdbg_db1);
        var_qbg_db2 = (((p.p2 * var_qbg_db2) - var_qsbg_db2) - var_qdbg_db2);
        var_qbg_db3 = (((p.p2 * var_qbg_db3) - var_qsbg_db3) - var_qdbg_db3);
        var_qbg_db4 = (((p.p2 * var_qbg_db4) - var_qsbg_db4) - var_qdbg_db4);
        var_qbg_rv = 0.0;
        var_qbg_rdn0 = 0.0;
        var_qbg_rdn1 = 0.0;
        var_qbg_rdn2 = 0.0;
        var_qbg_rdn3 = 0.0;
        var_qbg_rdn4 = 0.0;
        var_qbg_rdn5 = 0.0;
        var_qbg_rdn6 = 0.0;
        var_qbg_rdn7 = 0.0;
        var_qbg_rdn8 = 0.0;
        var_qbg_rdb0 = 0.0;
        var_qbg_rdb1 = 0.0;
        var_qbg_rdb2 = 0.0;
        var_qbg_rdb3 = 0.0;
        var_qbg_rdb4 = 0.0;

        let assign13210_e12250: f64 = (p.p2 * var_qfgs_parasitic);
        var_qfgs_parasitic = assign13210_e12250;
        var_qfgs_parasitic_dn0 = (p.p2 * var_qfgs_parasitic_dn0);
        var_qfgs_parasitic_dn1 = (p.p2 * var_qfgs_parasitic_dn1);
        var_qfgs_parasitic_dn2 = (p.p2 * var_qfgs_parasitic_dn2);
        var_qfgs_parasitic_dn3 = (p.p2 * var_qfgs_parasitic_dn3);
        var_qfgs_parasitic_dn4 = (p.p2 * var_qfgs_parasitic_dn4);
        var_qfgs_parasitic_dn5 = (p.p2 * var_qfgs_parasitic_dn5);
        var_qfgs_parasitic_dn6 = (p.p2 * var_qfgs_parasitic_dn6);
        var_qfgs_parasitic_dn7 = (p.p2 * var_qfgs_parasitic_dn7);
        var_qfgs_parasitic_dn8 = (p.p2 * var_qfgs_parasitic_dn8);
        var_qfgs_parasitic_db0 = (p.p2 * var_qfgs_parasitic_db0);
        var_qfgs_parasitic_db1 = (p.p2 * var_qfgs_parasitic_db1);
        var_qfgs_parasitic_db2 = (p.p2 * var_qfgs_parasitic_db2);
        var_qfgs_parasitic_db3 = (p.p2 * var_qfgs_parasitic_db3);
        var_qfgs_parasitic_db4 = (p.p2 * var_qfgs_parasitic_db4);
        var_qfgs_parasitic_rv = 0.0;
        var_qfgs_parasitic_rdn0 = 0.0;
        var_qfgs_parasitic_rdn1 = 0.0;
        var_qfgs_parasitic_rdn2 = 0.0;
        var_qfgs_parasitic_rdn3 = 0.0;
        var_qfgs_parasitic_rdn4 = 0.0;
        var_qfgs_parasitic_rdn5 = 0.0;
        var_qfgs_parasitic_rdn6 = 0.0;
        var_qfgs_parasitic_rdn7 = 0.0;
        var_qfgs_parasitic_rdn8 = 0.0;
        var_qfgs_parasitic_rdb0 = 0.0;
        var_qfgs_parasitic_rdb1 = 0.0;
        var_qfgs_parasitic_rdb2 = 0.0;
        var_qfgs_parasitic_rdb3 = 0.0;
        var_qfgs_parasitic_rdb4 = 0.0;

        let assign13220_e12253: f64 = (p.p2 * var_qfgd_parasitic);
        var_qfgd_parasitic = assign13220_e12253;
        var_qfgd_parasitic_dn0 = (p.p2 * var_qfgd_parasitic_dn0);
        var_qfgd_parasitic_dn1 = (p.p2 * var_qfgd_parasitic_dn1);
        var_qfgd_parasitic_dn2 = (p.p2 * var_qfgd_parasitic_dn2);
        var_qfgd_parasitic_dn3 = (p.p2 * var_qfgd_parasitic_dn3);
        var_qfgd_parasitic_dn4 = (p.p2 * var_qfgd_parasitic_dn4);
        var_qfgd_parasitic_dn5 = (p.p2 * var_qfgd_parasitic_dn5);
        var_qfgd_parasitic_dn6 = (p.p2 * var_qfgd_parasitic_dn6);
        var_qfgd_parasitic_dn7 = (p.p2 * var_qfgd_parasitic_dn7);
        var_qfgd_parasitic_dn8 = (p.p2 * var_qfgd_parasitic_dn8);
        var_qfgd_parasitic_db0 = (p.p2 * var_qfgd_parasitic_db0);
        var_qfgd_parasitic_db1 = (p.p2 * var_qfgd_parasitic_db1);
        var_qfgd_parasitic_db2 = (p.p2 * var_qfgd_parasitic_db2);
        var_qfgd_parasitic_db3 = (p.p2 * var_qfgd_parasitic_db3);
        var_qfgd_parasitic_db4 = (p.p2 * var_qfgd_parasitic_db4);
        var_qfgd_parasitic_rv = 0.0;
        var_qfgd_parasitic_rdn0 = 0.0;
        var_qfgd_parasitic_rdn1 = 0.0;
        var_qfgd_parasitic_rdn2 = 0.0;
        var_qfgd_parasitic_rdn3 = 0.0;
        var_qfgd_parasitic_rdn4 = 0.0;
        var_qfgd_parasitic_rdn5 = 0.0;
        var_qfgd_parasitic_rdn6 = 0.0;
        var_qfgd_parasitic_rdn7 = 0.0;
        var_qfgd_parasitic_rdn8 = 0.0;
        var_qfgd_parasitic_rdb0 = 0.0;
        var_qfgd_parasitic_rdb1 = 0.0;
        var_qfgd_parasitic_rdb2 = 0.0;
        var_qfgd_parasitic_rdb3 = 0.0;
        var_qfgd_parasitic_rdb4 = 0.0;

        let assign13230_e12256: f64 = (var_qsi + var_qdi);
        let assign13230_e12257: f64 = (-assign13230_e12256);
        var_qinv = assign13230_e12257;
        var_qinv_dn0 = (-(var_qsi_dn0 + var_qdi_dn0));
        var_qinv_dn1 = (-(var_qsi_dn1 + var_qdi_dn1));
        var_qinv_dn2 = (-(var_qsi_dn2 + var_qdi_dn2));
        var_qinv_dn3 = (-(var_qsi_dn3 + var_qdi_dn3));
        var_qinv_dn4 = (-(var_qsi_dn4 + var_qdi_dn4));
        var_qinv_dn5 = (-(var_qsi_dn5 + var_qdi_dn5));
        var_qinv_dn6 = (-(var_qsi_dn6 + var_qdi_dn6));
        var_qinv_dn7 = (-(var_qsi_dn7 + var_qdi_dn7));
        var_qinv_dn8 = (-(var_qsi_dn8 + var_qdi_dn8));
        var_qinv_db0 = (-(var_qsi_db0 + var_qdi_db0));
        var_qinv_db1 = (-(var_qsi_db1 + var_qdi_db1));
        var_qinv_db2 = (-(var_qsi_db2 + var_qdi_db2));
        var_qinv_db3 = (-(var_qsi_db3 + var_qdi_db3));
        var_qinv_db4 = (-(var_qsi_db4 + var_qdi_db4));
        var_qinv_rv = 0.0;
        var_qinv_rdn0 = 0.0;
        var_qinv_rdn1 = 0.0;
        var_qinv_rdn2 = 0.0;
        var_qinv_rdn3 = 0.0;
        var_qinv_rdn4 = 0.0;
        var_qinv_rdn5 = 0.0;
        var_qinv_rdn6 = 0.0;
        var_qinv_rdn7 = 0.0;
        var_qinv_rdn8 = 0.0;
        var_qinv_rdb0 = 0.0;
        var_qinv_rdb1 = 0.0;
        var_qinv_rdb2 = 0.0;
        var_qinv_rdb3 = 0.0;
        var_qinv_rdb4 = 0.0;

        let assign13240_e12260: f64 = (var_utotal * var_qinv);
        var_t0 = assign13240_e12260;
        var_t0_dn0 = ((var_utotal_dn0 * var_qinv) + (var_utotal * var_qinv_dn0));
        var_t0_dn1 = ((var_utotal_dn1 * var_qinv) + (var_utotal * var_qinv_dn1));
        var_t0_dn2 = ((var_utotal_dn2 * var_qinv) + (var_utotal * var_qinv_dn2));
        var_t0_dn3 = ((var_utotal_dn3 * var_qinv) + (var_utotal * var_qinv_dn3));
        var_t0_dn4 = ((var_utotal_dn4 * var_qinv) + (var_utotal * var_qinv_dn4));
        var_t0_dn5 = ((var_utotal_dn5 * var_qinv) + (var_utotal * var_qinv_dn5));
        var_t0_dn6 = ((var_utotal_dn6 * var_qinv) + (var_utotal * var_qinv_dn6));
        var_t0_dn7 = ((var_utotal_dn7 * var_qinv) + (var_utotal * var_qinv_dn7));
        var_t0_dn8 = ((var_utotal_dn8 * var_qinv) + (var_utotal * var_qinv_dn8));
        var_t0_db0 = ((var_utotal_db0 * var_qinv) + (var_utotal * var_qinv_db0));
        var_t0_db1 = ((var_utotal_db1 * var_qinv) + (var_utotal * var_qinv_db1));
        var_t0_db2 = ((var_utotal_db2 * var_qinv) + (var_utotal * var_qinv_db2));
        var_t0_db3 = ((var_utotal_db3 * var_qinv) + (var_utotal * var_qinv_db3));
        var_t0_db4 = ((var_utotal_db4 * var_qinv) + (var_utotal * var_qinv_db4));
        var_t0_rv = 0.0;
        var_t0_rdn0 = 0.0;
        var_t0_rdn1 = 0.0;
        var_t0_rdn2 = 0.0;
        var_t0_rdn3 = 0.0;
        var_t0_rdn4 = 0.0;
        var_t0_rdn5 = 0.0;
        var_t0_rdn6 = 0.0;
        var_t0_rdn7 = 0.0;
        var_t0_rdn8 = 0.0;
        var_t0_rdb0 = 0.0;
        var_t0_rdb1 = 0.0;
        var_t0_rdb2 = 0.0;
        var_t0_rdb3 = 0.0;
        var_t0_rdb4 = 0.0;

        let assign13250_e12263: f64 = (var_t0 * var_rdsi);
        let assign13250_e12266: f64 = (var_leff * var_leff);
        let assign13250_e12267: f64 = (assign13250_e12263 + assign13250_e12266);
        var_t1 = assign13250_e12267;
        var_t1_dn0 = (((var_t0_dn0 * var_rdsi) + (var_t0 * var_rdsi_dn0)) + ((var_leff_dn0 * var_leff) + (var_leff * var_leff_dn0)));
        var_t1_dn1 = (((var_t0_dn1 * var_rdsi) + (var_t0 * var_rdsi_dn1)) + ((var_leff_dn1 * var_leff) + (var_leff * var_leff_dn1)));
        var_t1_dn2 = (((var_t0_dn2 * var_rdsi) + (var_t0 * var_rdsi_dn2)) + ((var_leff_dn2 * var_leff) + (var_leff * var_leff_dn2)));
        var_t1_dn3 = (((var_t0_dn3 * var_rdsi) + (var_t0 * var_rdsi_dn3)) + ((var_leff_dn3 * var_leff) + (var_leff * var_leff_dn3)));
        var_t1_dn4 = (((var_t0_dn4 * var_rdsi) + (var_t0 * var_rdsi_dn4)) + ((var_leff_dn4 * var_leff) + (var_leff * var_leff_dn4)));
        var_t1_dn5 = (((var_t0_dn5 * var_rdsi) + (var_t0 * var_rdsi_dn5)) + ((var_leff_dn5 * var_leff) + (var_leff * var_leff_dn5)));
        var_t1_dn6 = (((var_t0_dn6 * var_rdsi) + (var_t0 * var_rdsi_dn6)) + ((var_leff_dn6 * var_leff) + (var_leff * var_leff_dn6)));
        var_t1_dn7 = (((var_t0_dn7 * var_rdsi) + (var_t0 * var_rdsi_dn7)) + ((var_leff_dn7 * var_leff) + (var_leff * var_leff_dn7)));
        var_t1_dn8 = (((var_t0_dn8 * var_rdsi) + (var_t0 * var_rdsi_dn8)) + ((var_leff_dn8 * var_leff) + (var_leff * var_leff_dn8)));
        var_t1_db0 = (((var_t0_db0 * var_rdsi) + (var_t0 * var_rdsi_db0)) + ((var_leff_db0 * var_leff) + (var_leff * var_leff_db0)));
        var_t1_db1 = (((var_t0_db1 * var_rdsi) + (var_t0 * var_rdsi_db1)) + ((var_leff_db1 * var_leff) + (var_leff * var_leff_db1)));
        var_t1_db2 = (((var_t0_db2 * var_rdsi) + (var_t0 * var_rdsi_db2)) + ((var_leff_db2 * var_leff) + (var_leff * var_leff_db2)));
        var_t1_db3 = (((var_t0_db3 * var_rdsi) + (var_t0 * var_rdsi_db3)) + ((var_leff_db3 * var_leff) + (var_leff * var_leff_db3)));
        var_t1_db4 = (((var_t0_db4 * var_rdsi) + (var_t0 * var_rdsi_db4)) + ((var_leff_db4 * var_leff) + (var_leff * var_leff_db4)));
        var_t1_rv = 0.0;
        var_t1_rdn0 = 0.0;
        var_t1_rdn1 = 0.0;
        var_t1_rdn2 = 0.0;
        var_t1_rdn3 = 0.0;
        var_t1_rdn4 = 0.0;
        var_t1_rdn5 = 0.0;
        var_t1_rdn6 = 0.0;
        var_t1_rdn7 = 0.0;
        var_t1_rdn8 = 0.0;
        var_t1_rdb0 = 0.0;
        var_t1_rdb1 = 0.0;
        var_t1_rdb2 = 0.0;
        var_t1_rdb3 = 0.0;
        var_t1_rdb4 = 0.0;


        *var_qbg_slot = var_qbg;
        *var_qbg_db0_slot = var_qbg_db0;
        *var_qbg_db1_slot = var_qbg_db1;
        *var_qbg_db2_slot = var_qbg_db2;
        *var_qbg_db3_slot = var_qbg_db3;
        *var_qbg_db4_slot = var_qbg_db4;
        *var_qbg_dn0_slot = var_qbg_dn0;
        *var_qbg_dn1_slot = var_qbg_dn1;
        *var_qbg_dn2_slot = var_qbg_dn2;
        *var_qbg_dn3_slot = var_qbg_dn3;
        *var_qbg_dn4_slot = var_qbg_dn4;
        *var_qbg_dn5_slot = var_qbg_dn5;
        *var_qbg_dn6_slot = var_qbg_dn6;
        *var_qbg_dn7_slot = var_qbg_dn7;
        *var_qbg_dn8_slot = var_qbg_dn8;
        *var_qbg_rdb0_slot = var_qbg_rdb0;
        *var_qbg_rdb1_slot = var_qbg_rdb1;
        *var_qbg_rdb2_slot = var_qbg_rdb2;
        *var_qbg_rdb3_slot = var_qbg_rdb3;
        *var_qbg_rdb4_slot = var_qbg_rdb4;
        *var_qbg_rdn0_slot = var_qbg_rdn0;
        *var_qbg_rdn1_slot = var_qbg_rdn1;
        *var_qbg_rdn2_slot = var_qbg_rdn2;
        *var_qbg_rdn3_slot = var_qbg_rdn3;
        *var_qbg_rdn4_slot = var_qbg_rdn4;
        *var_qbg_rdn5_slot = var_qbg_rdn5;
        *var_qbg_rdn6_slot = var_qbg_rdn6;
        *var_qbg_rdn7_slot = var_qbg_rdn7;
        *var_qbg_rdn8_slot = var_qbg_rdn8;
        *var_qbg_rv_slot = var_qbg_rv;
        *var_qd_slot = var_qd;
        *var_qd_db0_slot = var_qd_db0;
        *var_qd_db1_slot = var_qd_db1;
        *var_qd_db2_slot = var_qd_db2;
        *var_qd_db3_slot = var_qd_db3;
        *var_qd_db4_slot = var_qd_db4;
        *var_qd_dn0_slot = var_qd_dn0;
        *var_qd_dn1_slot = var_qd_dn1;
        *var_qd_dn2_slot = var_qd_dn2;
        *var_qd_dn3_slot = var_qd_dn3;
        *var_qd_dn4_slot = var_qd_dn4;
        *var_qd_dn5_slot = var_qd_dn5;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qd_rdb0_slot = var_qd_rdb0;
        *var_qd_rdb1_slot = var_qd_rdb1;
        *var_qd_rdb2_slot = var_qd_rdb2;
        *var_qd_rdb3_slot = var_qd_rdb3;
        *var_qd_rdb4_slot = var_qd_rdb4;
        *var_qd_rdn0_slot = var_qd_rdn0;
        *var_qd_rdn1_slot = var_qd_rdn1;
        *var_qd_rdn2_slot = var_qd_rdn2;
        *var_qd_rdn3_slot = var_qd_rdn3;
        *var_qd_rdn4_slot = var_qd_rdn4;
        *var_qd_rdn5_slot = var_qd_rdn5;
        *var_qd_rdn6_slot = var_qd_rdn6;
        *var_qd_rdn7_slot = var_qd_rdn7;
        *var_qd_rdn8_slot = var_qd_rdn8;
        *var_qd_rv_slot = var_qd_rv;
        *var_qdi_slot = var_qdi;
        *var_qdi_db0_slot = var_qdi_db0;
        *var_qdi_db1_slot = var_qdi_db1;
        *var_qdi_db2_slot = var_qdi_db2;
        *var_qdi_db3_slot = var_qdi_db3;
        *var_qdi_db4_slot = var_qdi_db4;
        *var_qdi_dn0_slot = var_qdi_dn0;
        *var_qdi_dn1_slot = var_qdi_dn1;
        *var_qdi_dn2_slot = var_qdi_dn2;
        *var_qdi_dn3_slot = var_qdi_dn3;
        *var_qdi_dn4_slot = var_qdi_dn4;
        *var_qdi_dn5_slot = var_qdi_dn5;
        *var_qdi_dn6_slot = var_qdi_dn6;
        *var_qdi_dn7_slot = var_qdi_dn7;
        *var_qdi_dn8_slot = var_qdi_dn8;
        *var_qdi_rdb0_slot = var_qdi_rdb0;
        *var_qdi_rdb1_slot = var_qdi_rdb1;
        *var_qdi_rdb2_slot = var_qdi_rdb2;
        *var_qdi_rdb3_slot = var_qdi_rdb3;
        *var_qdi_rdb4_slot = var_qdi_rdb4;
        *var_qdi_rdn0_slot = var_qdi_rdn0;
        *var_qdi_rdn1_slot = var_qdi_rdn1;
        *var_qdi_rdn2_slot = var_qdi_rdn2;
        *var_qdi_rdn3_slot = var_qdi_rdn3;
        *var_qdi_rdn4_slot = var_qdi_rdn4;
        *var_qdi_rdn5_slot = var_qdi_rdn5;
        *var_qdi_rdn6_slot = var_qdi_rdn6;
        *var_qdi_rdn7_slot = var_qdi_rdn7;
        *var_qdi_rdn8_slot = var_qdi_rdn8;
        *var_qdi_rv_slot = var_qdi_rv;
        *var_qfg_slot = var_qfg;
        *var_qfg_db0_slot = var_qfg_db0;
        *var_qfg_db1_slot = var_qfg_db1;
        *var_qfg_db2_slot = var_qfg_db2;
        *var_qfg_db3_slot = var_qfg_db3;
        *var_qfg_db4_slot = var_qfg_db4;
        *var_qfg_dn0_slot = var_qfg_dn0;
        *var_qfg_dn1_slot = var_qfg_dn1;
        *var_qfg_dn2_slot = var_qfg_dn2;
        *var_qfg_dn3_slot = var_qfg_dn3;
        *var_qfg_dn4_slot = var_qfg_dn4;
        *var_qfg_dn5_slot = var_qfg_dn5;
        *var_qfg_dn6_slot = var_qfg_dn6;
        *var_qfg_dn7_slot = var_qfg_dn7;
        *var_qfg_dn8_slot = var_qfg_dn8;
        *var_qfg_rdb0_slot = var_qfg_rdb0;
        *var_qfg_rdb1_slot = var_qfg_rdb1;
        *var_qfg_rdb2_slot = var_qfg_rdb2;
        *var_qfg_rdb3_slot = var_qfg_rdb3;
        *var_qfg_rdb4_slot = var_qfg_rdb4;
        *var_qfg_rdn0_slot = var_qfg_rdn0;
        *var_qfg_rdn1_slot = var_qfg_rdn1;
        *var_qfg_rdn2_slot = var_qfg_rdn2;
        *var_qfg_rdn3_slot = var_qfg_rdn3;
        *var_qfg_rdn4_slot = var_qfg_rdn4;
        *var_qfg_rdn5_slot = var_qfg_rdn5;
        *var_qfg_rdn6_slot = var_qfg_rdn6;
        *var_qfg_rdn7_slot = var_qfg_rdn7;
        *var_qfg_rdn8_slot = var_qfg_rdn8;
        *var_qfg_rv_slot = var_qfg_rv;
        *var_qfgd_parasitic_slot = var_qfgd_parasitic;
        *var_qfgd_parasitic_db0_slot = var_qfgd_parasitic_db0;
        *var_qfgd_parasitic_db1_slot = var_qfgd_parasitic_db1;
        *var_qfgd_parasitic_db2_slot = var_qfgd_parasitic_db2;
        *var_qfgd_parasitic_db3_slot = var_qfgd_parasitic_db3;
        *var_qfgd_parasitic_db4_slot = var_qfgd_parasitic_db4;
        *var_qfgd_parasitic_dn0_slot = var_qfgd_parasitic_dn0;
        *var_qfgd_parasitic_dn1_slot = var_qfgd_parasitic_dn1;
        *var_qfgd_parasitic_dn2_slot = var_qfgd_parasitic_dn2;
        *var_qfgd_parasitic_dn3_slot = var_qfgd_parasitic_dn3;
        *var_qfgd_parasitic_dn4_slot = var_qfgd_parasitic_dn4;
        *var_qfgd_parasitic_dn5_slot = var_qfgd_parasitic_dn5;
        *var_qfgd_parasitic_dn6_slot = var_qfgd_parasitic_dn6;
        *var_qfgd_parasitic_dn7_slot = var_qfgd_parasitic_dn7;
        *var_qfgd_parasitic_dn8_slot = var_qfgd_parasitic_dn8;
        *var_qfgd_parasitic_rdb0_slot = var_qfgd_parasitic_rdb0;
        *var_qfgd_parasitic_rdb1_slot = var_qfgd_parasitic_rdb1;
        *var_qfgd_parasitic_rdb2_slot = var_qfgd_parasitic_rdb2;
        *var_qfgd_parasitic_rdb3_slot = var_qfgd_parasitic_rdb3;
        *var_qfgd_parasitic_rdb4_slot = var_qfgd_parasitic_rdb4;
        *var_qfgd_parasitic_rdn0_slot = var_qfgd_parasitic_rdn0;
        *var_qfgd_parasitic_rdn1_slot = var_qfgd_parasitic_rdn1;
        *var_qfgd_parasitic_rdn2_slot = var_qfgd_parasitic_rdn2;
        *var_qfgd_parasitic_rdn3_slot = var_qfgd_parasitic_rdn3;
        *var_qfgd_parasitic_rdn4_slot = var_qfgd_parasitic_rdn4;
        *var_qfgd_parasitic_rdn5_slot = var_qfgd_parasitic_rdn5;
        *var_qfgd_parasitic_rdn6_slot = var_qfgd_parasitic_rdn6;
        *var_qfgd_parasitic_rdn7_slot = var_qfgd_parasitic_rdn7;
        *var_qfgd_parasitic_rdn8_slot = var_qfgd_parasitic_rdn8;
        *var_qfgd_parasitic_rv_slot = var_qfgd_parasitic_rv;
        *var_qfgs_parasitic_slot = var_qfgs_parasitic;
        *var_qfgs_parasitic_db0_slot = var_qfgs_parasitic_db0;
        *var_qfgs_parasitic_db1_slot = var_qfgs_parasitic_db1;
        *var_qfgs_parasitic_db2_slot = var_qfgs_parasitic_db2;
        *var_qfgs_parasitic_db3_slot = var_qfgs_parasitic_db3;
        *var_qfgs_parasitic_db4_slot = var_qfgs_parasitic_db4;
        *var_qfgs_parasitic_dn0_slot = var_qfgs_parasitic_dn0;
        *var_qfgs_parasitic_dn1_slot = var_qfgs_parasitic_dn1;
        *var_qfgs_parasitic_dn2_slot = var_qfgs_parasitic_dn2;
        *var_qfgs_parasitic_dn3_slot = var_qfgs_parasitic_dn3;
        *var_qfgs_parasitic_dn4_slot = var_qfgs_parasitic_dn4;
        *var_qfgs_parasitic_dn5_slot = var_qfgs_parasitic_dn5;
        *var_qfgs_parasitic_dn6_slot = var_qfgs_parasitic_dn6;
        *var_qfgs_parasitic_dn7_slot = var_qfgs_parasitic_dn7;
        *var_qfgs_parasitic_dn8_slot = var_qfgs_parasitic_dn8;
        *var_qfgs_parasitic_rdb0_slot = var_qfgs_parasitic_rdb0;
        *var_qfgs_parasitic_rdb1_slot = var_qfgs_parasitic_rdb1;
        *var_qfgs_parasitic_rdb2_slot = var_qfgs_parasitic_rdb2;
        *var_qfgs_parasitic_rdb3_slot = var_qfgs_parasitic_rdb3;
        *var_qfgs_parasitic_rdb4_slot = var_qfgs_parasitic_rdb4;
        *var_qfgs_parasitic_rdn0_slot = var_qfgs_parasitic_rdn0;
        *var_qfgs_parasitic_rdn1_slot = var_qfgs_parasitic_rdn1;
        *var_qfgs_parasitic_rdn2_slot = var_qfgs_parasitic_rdn2;
        *var_qfgs_parasitic_rdn3_slot = var_qfgs_parasitic_rdn3;
        *var_qfgs_parasitic_rdn4_slot = var_qfgs_parasitic_rdn4;
        *var_qfgs_parasitic_rdn5_slot = var_qfgs_parasitic_rdn5;
        *var_qfgs_parasitic_rdn6_slot = var_qfgs_parasitic_rdn6;
        *var_qfgs_parasitic_rdn7_slot = var_qfgs_parasitic_rdn7;
        *var_qfgs_parasitic_rdn8_slot = var_qfgs_parasitic_rdn8;
        *var_qfgs_parasitic_rv_slot = var_qfgs_parasitic_rv;
        *var_qinv_slot = var_qinv;
        *var_qinv_db0_slot = var_qinv_db0;
        *var_qinv_db1_slot = var_qinv_db1;
        *var_qinv_db2_slot = var_qinv_db2;
        *var_qinv_db3_slot = var_qinv_db3;
        *var_qinv_db4_slot = var_qinv_db4;
        *var_qinv_dn0_slot = var_qinv_dn0;
        *var_qinv_dn1_slot = var_qinv_dn1;
        *var_qinv_dn2_slot = var_qinv_dn2;
        *var_qinv_dn3_slot = var_qinv_dn3;
        *var_qinv_dn4_slot = var_qinv_dn4;
        *var_qinv_dn5_slot = var_qinv_dn5;
        *var_qinv_dn6_slot = var_qinv_dn6;
        *var_qinv_dn7_slot = var_qinv_dn7;
        *var_qinv_dn8_slot = var_qinv_dn8;
        *var_qinv_rdb0_slot = var_qinv_rdb0;
        *var_qinv_rdb1_slot = var_qinv_rdb1;
        *var_qinv_rdb2_slot = var_qinv_rdb2;
        *var_qinv_rdb3_slot = var_qinv_rdb3;
        *var_qinv_rdb4_slot = var_qinv_rdb4;
        *var_qinv_rdn0_slot = var_qinv_rdn0;
        *var_qinv_rdn1_slot = var_qinv_rdn1;
        *var_qinv_rdn2_slot = var_qinv_rdn2;
        *var_qinv_rdn3_slot = var_qinv_rdn3;
        *var_qinv_rdn4_slot = var_qinv_rdn4;
        *var_qinv_rdn5_slot = var_qinv_rdn5;
        *var_qinv_rdn6_slot = var_qinv_rdn6;
        *var_qinv_rdn7_slot = var_qinv_rdn7;
        *var_qinv_rdn8_slot = var_qinv_rdn8;
        *var_qinv_rv_slot = var_qinv_rv;
        *var_qs_slot = var_qs;
        *var_qs_db0_slot = var_qs_db0;
        *var_qs_db1_slot = var_qs_db1;
        *var_qs_db2_slot = var_qs_db2;
        *var_qs_db3_slot = var_qs_db3;
        *var_qs_db4_slot = var_qs_db4;
        *var_qs_dn0_slot = var_qs_dn0;
        *var_qs_dn1_slot = var_qs_dn1;
        *var_qs_dn2_slot = var_qs_dn2;
        *var_qs_dn3_slot = var_qs_dn3;
        *var_qs_dn4_slot = var_qs_dn4;
        *var_qs_dn5_slot = var_qs_dn5;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_qs_rdb0_slot = var_qs_rdb0;
        *var_qs_rdb1_slot = var_qs_rdb1;
        *var_qs_rdb2_slot = var_qs_rdb2;
        *var_qs_rdb3_slot = var_qs_rdb3;
        *var_qs_rdb4_slot = var_qs_rdb4;
        *var_qs_rdn0_slot = var_qs_rdn0;
        *var_qs_rdn1_slot = var_qs_rdn1;
        *var_qs_rdn2_slot = var_qs_rdn2;
        *var_qs_rdn3_slot = var_qs_rdn3;
        *var_qs_rdn4_slot = var_qs_rdn4;
        *var_qs_rdn5_slot = var_qs_rdn5;
        *var_qs_rdn6_slot = var_qs_rdn6;
        *var_qs_rdn7_slot = var_qs_rdn7;
        *var_qs_rdn8_slot = var_qs_rdn8;
        *var_qs_rv_slot = var_qs_rv;
        *var_qsi_slot = var_qsi;
        *var_qsi_db0_slot = var_qsi_db0;
        *var_qsi_db1_slot = var_qsi_db1;
        *var_qsi_db2_slot = var_qsi_db2;
        *var_qsi_db3_slot = var_qsi_db3;
        *var_qsi_db4_slot = var_qsi_db4;
        *var_qsi_dn0_slot = var_qsi_dn0;
        *var_qsi_dn1_slot = var_qsi_dn1;
        *var_qsi_dn2_slot = var_qsi_dn2;
        *var_qsi_dn3_slot = var_qsi_dn3;
        *var_qsi_dn4_slot = var_qsi_dn4;
        *var_qsi_dn5_slot = var_qsi_dn5;
        *var_qsi_dn6_slot = var_qsi_dn6;
        *var_qsi_dn7_slot = var_qsi_dn7;
        *var_qsi_dn8_slot = var_qsi_dn8;
        *var_qsi_rdb0_slot = var_qsi_rdb0;
        *var_qsi_rdb1_slot = var_qsi_rdb1;
        *var_qsi_rdb2_slot = var_qsi_rdb2;
        *var_qsi_rdb3_slot = var_qsi_rdb3;
        *var_qsi_rdb4_slot = var_qsi_rdb4;
        *var_qsi_rdn0_slot = var_qsi_rdn0;
        *var_qsi_rdn1_slot = var_qsi_rdn1;
        *var_qsi_rdn2_slot = var_qsi_rdn2;
        *var_qsi_rdn3_slot = var_qsi_rdn3;
        *var_qsi_rdn4_slot = var_qsi_rdn4;
        *var_qsi_rdn5_slot = var_qsi_rdn5;
        *var_qsi_rdn6_slot = var_qsi_rdn6;
        *var_qsi_rdn7_slot = var_qsi_rdn7;
        *var_qsi_rdn8_slot = var_qsi_rdn8;
        *var_qsi_rv_slot = var_qsi_rv;
        *var_t0_slot = var_t0;
        *var_t0_db0_slot = var_t0_db0;
        *var_t0_db1_slot = var_t0_db1;
        *var_t0_db2_slot = var_t0_db2;
        *var_t0_db3_slot = var_t0_db3;
        *var_t0_db4_slot = var_t0_db4;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rdb0_slot = var_t0_rdb0;
        *var_t0_rdb1_slot = var_t0_rdb1;
        *var_t0_rdb2_slot = var_t0_rdb2;
        *var_t0_rdb3_slot = var_t0_rdb3;
        *var_t0_rdb4_slot = var_t0_rdb4;
        *var_t0_rdn0_slot = var_t0_rdn0;
        *var_t0_rdn1_slot = var_t0_rdn1;
        *var_t0_rdn2_slot = var_t0_rdn2;
        *var_t0_rdn3_slot = var_t0_rdn3;
        *var_t0_rdn4_slot = var_t0_rdn4;
        *var_t0_rdn5_slot = var_t0_rdn5;
        *var_t0_rdn6_slot = var_t0_rdn6;
        *var_t0_rdn7_slot = var_t0_rdn7;
        *var_t0_rdn8_slot = var_t0_rdn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_db0_slot = var_t1_db0;
        *var_t1_db1_slot = var_t1_db1;
        *var_t1_db2_slot = var_t1_db2;
        *var_t1_db3_slot = var_t1_db3;
        *var_t1_db4_slot = var_t1_db4;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rdb0_slot = var_t1_rdb0;
        *var_t1_rdb1_slot = var_t1_rdb1;
        *var_t1_rdb2_slot = var_t1_rdb2;
        *var_t1_rdb3_slot = var_t1_rdb3;
        *var_t1_rdb4_slot = var_t1_rdb4;
        *var_t1_rdn0_slot = var_t1_rdn0;
        *var_t1_rdn1_slot = var_t1_rdn1;
        *var_t1_rdn2_slot = var_t1_rdn2;
        *var_t1_rdn3_slot = var_t1_rdn3;
        *var_t1_rdn4_slot = var_t1_rdn4;
        *var_t1_rdn5_slot = var_t1_rdn5;
        *var_t1_rdn6_slot = var_t1_rdn6;
        *var_t1_rdn7_slot = var_t1_rdn7;
        *var_t1_rdn8_slot = var_t1_rdn8;
        *var_t1_rv_slot = var_t1_rv;
    }

    pub(super) fn stamp_reactive_block_91(
        p: &Parameters,
        var_cox1: f64,
        var_cox1_db0: f64,
        var_cox1_db1: f64,
        var_cox1_db2: f64,
        var_cox1_db3: f64,
        var_cox1_db4: f64,
        var_cox1_dn0: f64,
        var_cox1_dn1: f64,
        var_cox1_dn2: f64,
        var_cox1_dn3: f64,
        var_cox1_dn4: f64,
        var_cox1_dn5: f64,
        var_cox1_dn6: f64,
        var_cox1_dn7: f64,
        var_cox1_dn8: f64,
        var_leff: f64,
        var_leff_db0: f64,
        var_leff_db1: f64,
        var_leff_db2: f64,
        var_leff_db3: f64,
        var_leff_db4: f64,
        var_leff_dn0: f64,
        var_leff_dn1: f64,
        var_leff_dn2: f64,
        var_leff_dn3: f64,
        var_leff_dn4: f64,
        var_leff_dn5: f64,
        var_leff_dn6: f64,
        var_leff_dn7: f64,
        var_leff_dn8: f64,
        var_utotal: f64,
        var_utotal_db0: f64,
        var_utotal_db1: f64,
        var_utotal_db2: f64,
        var_utotal_db3: f64,
        var_utotal_db4: f64,
        var_utotal_dn0: f64,
        var_utotal_dn1: f64,
        var_utotal_dn2: f64,
        var_utotal_dn3: f64,
        var_utotal_dn4: f64,
        var_utotal_dn5: f64,
        var_utotal_dn6: f64,
        var_utotal_dn7: f64,
        var_utotal_dn8: f64,
        var_weff: f64,
        var_weff_db0: f64,
        var_weff_db1: f64,
        var_weff_db2: f64,
        var_weff_db3: f64,
        var_weff_db4: f64,
        var_weff_dn0: f64,
        var_weff_dn1: f64,
        var_weff_dn2: f64,
        var_weff_dn3: f64,
        var_weff_dn4: f64,
        var_weff_dn5: f64,
        var_weff_dn6: f64,
        var_weff_dn7: f64,
        var_weff_dn8: f64,
        var_xrcrg1_i: f64,
        var_guard137_slot: &mut f64,
        var_guard137_db0_slot: &mut f64,
        var_guard137_db1_slot: &mut f64,
        var_guard137_db2_slot: &mut f64,
        var_guard137_db3_slot: &mut f64,
        var_guard137_db4_slot: &mut f64,
        var_guard137_dn0_slot: &mut f64,
        var_guard137_dn1_slot: &mut f64,
        var_guard137_dn2_slot: &mut f64,
        var_guard137_dn3_slot: &mut f64,
        var_guard137_dn4_slot: &mut f64,
        var_guard137_dn5_slot: &mut f64,
        var_guard137_dn6_slot: &mut f64,
        var_guard137_dn7_slot: &mut f64,
        var_guard137_dn8_slot: &mut f64,
        var_guard137_rdb0_slot: &mut f64,
        var_guard137_rdb1_slot: &mut f64,
        var_guard137_rdb2_slot: &mut f64,
        var_guard137_rdb3_slot: &mut f64,
        var_guard137_rdb4_slot: &mut f64,
        var_guard137_rdn0_slot: &mut f64,
        var_guard137_rdn1_slot: &mut f64,
        var_guard137_rdn2_slot: &mut f64,
        var_guard137_rdn3_slot: &mut f64,
        var_guard137_rdn4_slot: &mut f64,
        var_guard137_rdn5_slot: &mut f64,
        var_guard137_rdn6_slot: &mut f64,
        var_guard137_rdn7_slot: &mut f64,
        var_guard137_rdn8_slot: &mut f64,
        var_guard137_rv_slot: &mut f64,
        var_guard147_slot: &mut f64,
        var_guard147_db0_slot: &mut f64,
        var_guard147_db1_slot: &mut f64,
        var_guard147_db2_slot: &mut f64,
        var_guard147_db3_slot: &mut f64,
        var_guard147_db4_slot: &mut f64,
        var_guard147_dn0_slot: &mut f64,
        var_guard147_dn1_slot: &mut f64,
        var_guard147_dn2_slot: &mut f64,
        var_guard147_dn3_slot: &mut f64,
        var_guard147_dn4_slot: &mut f64,
        var_guard147_dn5_slot: &mut f64,
        var_guard147_dn6_slot: &mut f64,
        var_guard147_dn7_slot: &mut f64,
        var_guard147_dn8_slot: &mut f64,
        var_guard147_rdb0_slot: &mut f64,
        var_guard147_rdb1_slot: &mut f64,
        var_guard147_rdb2_slot: &mut f64,
        var_guard147_rdb3_slot: &mut f64,
        var_guard147_rdb4_slot: &mut f64,
        var_guard147_rdn0_slot: &mut f64,
        var_guard147_rdn1_slot: &mut f64,
        var_guard147_rdn2_slot: &mut f64,
        var_guard147_rdn3_slot: &mut f64,
        var_guard147_rdn4_slot: &mut f64,
        var_guard147_rdn5_slot: &mut f64,
        var_guard147_rdn6_slot: &mut f64,
        var_guard147_rdn7_slot: &mut f64,
        var_guard147_rdn8_slot: &mut f64,
        var_guard147_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_db0_slot: &mut f64,
        var_t0_db1_slot: &mut f64,
        var_t0_db2_slot: &mut f64,
        var_t0_db3_slot: &mut f64,
        var_t0_db4_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rdb0_slot: &mut f64,
        var_t0_rdb1_slot: &mut f64,
        var_t0_rdb2_slot: &mut f64,
        var_t0_rdb3_slot: &mut f64,
        var_t0_rdb4_slot: &mut f64,
        var_t0_rdn0_slot: &mut f64,
        var_t0_rdn1_slot: &mut f64,
        var_t0_rdn2_slot: &mut f64,
        var_t0_rdn3_slot: &mut f64,
        var_t0_rdn4_slot: &mut f64,
        var_t0_rdn5_slot: &mut f64,
        var_t0_rdn6_slot: &mut f64,
        var_t0_rdn7_slot: &mut f64,
        var_t0_rdn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
    ) {
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard137_db0: f64 = *var_guard137_db0_slot;
        let mut var_guard137_db1: f64 = *var_guard137_db1_slot;
        let mut var_guard137_db2: f64 = *var_guard137_db2_slot;
        let mut var_guard137_db3: f64 = *var_guard137_db3_slot;
        let mut var_guard137_db4: f64 = *var_guard137_db4_slot;
        let mut var_guard137_dn0: f64 = *var_guard137_dn0_slot;
        let mut var_guard137_dn1: f64 = *var_guard137_dn1_slot;
        let mut var_guard137_dn2: f64 = *var_guard137_dn2_slot;
        let mut var_guard137_dn3: f64 = *var_guard137_dn3_slot;
        let mut var_guard137_dn4: f64 = *var_guard137_dn4_slot;
        let mut var_guard137_dn5: f64 = *var_guard137_dn5_slot;
        let mut var_guard137_dn6: f64 = *var_guard137_dn6_slot;
        let mut var_guard137_dn7: f64 = *var_guard137_dn7_slot;
        let mut var_guard137_dn8: f64 = *var_guard137_dn8_slot;
        let mut var_guard137_rdb0: f64 = *var_guard137_rdb0_slot;
        let mut var_guard137_rdb1: f64 = *var_guard137_rdb1_slot;
        let mut var_guard137_rdb2: f64 = *var_guard137_rdb2_slot;
        let mut var_guard137_rdb3: f64 = *var_guard137_rdb3_slot;
        let mut var_guard137_rdb4: f64 = *var_guard137_rdb4_slot;
        let mut var_guard137_rdn0: f64 = *var_guard137_rdn0_slot;
        let mut var_guard137_rdn1: f64 = *var_guard137_rdn1_slot;
        let mut var_guard137_rdn2: f64 = *var_guard137_rdn2_slot;
        let mut var_guard137_rdn3: f64 = *var_guard137_rdn3_slot;
        let mut var_guard137_rdn4: f64 = *var_guard137_rdn4_slot;
        let mut var_guard137_rdn5: f64 = *var_guard137_rdn5_slot;
        let mut var_guard137_rdn6: f64 = *var_guard137_rdn6_slot;
        let mut var_guard137_rdn7: f64 = *var_guard137_rdn7_slot;
        let mut var_guard137_rdn8: f64 = *var_guard137_rdn8_slot;
        let mut var_guard137_rv: f64 = *var_guard137_rv_slot;
        let mut var_guard147: f64 = *var_guard147_slot;
        let mut var_guard147_db0: f64 = *var_guard147_db0_slot;
        let mut var_guard147_db1: f64 = *var_guard147_db1_slot;
        let mut var_guard147_db2: f64 = *var_guard147_db2_slot;
        let mut var_guard147_db3: f64 = *var_guard147_db3_slot;
        let mut var_guard147_db4: f64 = *var_guard147_db4_slot;
        let mut var_guard147_dn0: f64 = *var_guard147_dn0_slot;
        let mut var_guard147_dn1: f64 = *var_guard147_dn1_slot;
        let mut var_guard147_dn2: f64 = *var_guard147_dn2_slot;
        let mut var_guard147_dn3: f64 = *var_guard147_dn3_slot;
        let mut var_guard147_dn4: f64 = *var_guard147_dn4_slot;
        let mut var_guard147_dn5: f64 = *var_guard147_dn5_slot;
        let mut var_guard147_dn6: f64 = *var_guard147_dn6_slot;
        let mut var_guard147_dn7: f64 = *var_guard147_dn7_slot;
        let mut var_guard147_dn8: f64 = *var_guard147_dn8_slot;
        let mut var_guard147_rdb0: f64 = *var_guard147_rdb0_slot;
        let mut var_guard147_rdb1: f64 = *var_guard147_rdb1_slot;
        let mut var_guard147_rdb2: f64 = *var_guard147_rdb2_slot;
        let mut var_guard147_rdb3: f64 = *var_guard147_rdb3_slot;
        let mut var_guard147_rdb4: f64 = *var_guard147_rdb4_slot;
        let mut var_guard147_rdn0: f64 = *var_guard147_rdn0_slot;
        let mut var_guard147_rdn1: f64 = *var_guard147_rdn1_slot;
        let mut var_guard147_rdn2: f64 = *var_guard147_rdn2_slot;
        let mut var_guard147_rdn3: f64 = *var_guard147_rdn3_slot;
        let mut var_guard147_rdn4: f64 = *var_guard147_rdn4_slot;
        let mut var_guard147_rdn5: f64 = *var_guard147_rdn5_slot;
        let mut var_guard147_rdn6: f64 = *var_guard147_rdn6_slot;
        let mut var_guard147_rdn7: f64 = *var_guard147_rdn7_slot;
        let mut var_guard147_rdn8: f64 = *var_guard147_rdn8_slot;
        let mut var_guard147_rv: f64 = *var_guard147_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_db0: f64 = *var_t0_db0_slot;
        let mut var_t0_db1: f64 = *var_t0_db1_slot;
        let mut var_t0_db2: f64 = *var_t0_db2_slot;
        let mut var_t0_db3: f64 = *var_t0_db3_slot;
        let mut var_t0_db4: f64 = *var_t0_db4_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rdb0: f64 = *var_t0_rdb0_slot;
        let mut var_t0_rdb1: f64 = *var_t0_rdb1_slot;
        let mut var_t0_rdb2: f64 = *var_t0_rdb2_slot;
        let mut var_t0_rdb3: f64 = *var_t0_rdb3_slot;
        let mut var_t0_rdb4: f64 = *var_t0_rdb4_slot;
        let mut var_t0_rdn0: f64 = *var_t0_rdn0_slot;
        let mut var_t0_rdn1: f64 = *var_t0_rdn1_slot;
        let mut var_t0_rdn2: f64 = *var_t0_rdn2_slot;
        let mut var_t0_rdn3: f64 = *var_t0_rdn3_slot;
        let mut var_t0_rdn4: f64 = *var_t0_rdn4_slot;
        let mut var_t0_rdn5: f64 = *var_t0_rdn5_slot;
        let mut var_t0_rdn6: f64 = *var_t0_rdn6_slot;
        let mut var_t0_rdn7: f64 = *var_t0_rdn7_slot;
        let mut var_t0_rdn8: f64 = *var_t0_rdn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;

        let assign13290_e12287: f64 = if ((p.p20 == 1.0) && (var_xrcrg1_i != 0.0)) { 1.0 } else { 0.0 };
        var_guard137 = assign13290_e12287;
        var_guard137_dn0 = 0.0;
        var_guard137_dn1 = 0.0;
        var_guard137_dn2 = 0.0;
        var_guard137_dn3 = 0.0;
        var_guard137_dn4 = 0.0;
        var_guard137_dn5 = 0.0;
        var_guard137_dn6 = 0.0;
        var_guard137_dn7 = 0.0;
        var_guard137_dn8 = 0.0;
        var_guard137_db0 = 0.0;
        var_guard137_db1 = 0.0;
        var_guard137_db2 = 0.0;
        var_guard137_db3 = 0.0;
        var_guard137_db4 = 0.0;
        var_guard137_rv = 0.0;
        var_guard137_rdn0 = 0.0;
        var_guard137_rdn1 = 0.0;
        var_guard137_rdn2 = 0.0;
        var_guard137_rdn3 = 0.0;
        var_guard137_rdn4 = 0.0;
        var_guard137_rdn5 = 0.0;
        var_guard137_rdn6 = 0.0;
        var_guard137_rdn7 = 0.0;
        var_guard137_rdn8 = 0.0;
        var_guard137_rdb0 = 0.0;
        var_guard137_rdb1 = 0.0;
        var_guard137_rdb2 = 0.0;
        var_guard137_rdb3 = 0.0;
        var_guard137_rdb4 = 0.0;

        let (assign13300_e12297, assign13300_e12297_d_n0, assign13300_e12297_d_n1, assign13300_e12297_d_n2, assign13300_e12297_d_n3, assign13300_e12297_d_n4, assign13300_e12297_d_n5, assign13300_e12297_d_n6, assign13300_e12297_d_n7, assign13300_e12297_d_n8, assign13300_e12297_d_b0, assign13300_e12297_d_b1, assign13300_e12297_d_b2, assign13300_e12297_d_b3, assign13300_e12297_d_b4,) = {
    if (var_guard137 != 0.0) {
        let assign13300_e12291: f64 = (var_utotal * var_cox1);
        let assign13300_e12293: f64 = (assign13300_e12291 * var_weff);
        let assign13300_e12295: f64 = (assign13300_e12293 / var_leff);
        (assign13300_e12295, (((((((var_utotal_dn0 * var_cox1) + (var_utotal * var_cox1_dn0)) * var_weff) + (assign13300_e12291 * var_weff_dn0)) * var_leff) - (assign13300_e12293 * var_leff_dn0)) / (var_leff * var_leff)), (((((((var_utotal_dn1 * var_cox1) + (var_utotal * var_cox1_dn1)) * var_weff) + (assign13300_e12291 * var_weff_dn1)) * var_leff) - (assign13300_e12293 * var_leff_dn1)) / (var_leff * var_leff)), (((((((var_utotal_dn2 * var_cox1) + (var_utotal * var_cox1_dn2)) * var_weff) + (assign13300_e12291 * var_weff_dn2)) * var_leff) - (assign13300_e12293 * var_leff_dn2)) / (var_leff * var_leff)), (((((((var_utotal_dn3 * var_cox1) + (var_utotal * var_cox1_dn3)) * var_weff) + (assign13300_e12291 * var_weff_dn3)) * var_leff) - (assign13300_e12293 * var_leff_dn3)) / (var_leff * var_leff)), (((((((var_utotal_dn4 * var_cox1) + (var_utotal * var_cox1_dn4)) * var_weff) + (assign13300_e12291 * var_weff_dn4)) * var_leff) - (assign13300_e12293 * var_leff_dn4)) / (var_leff * var_leff)), (((((((var_utotal_dn5 * var_cox1) + (var_utotal * var_cox1_dn5)) * var_weff) + (assign13300_e12291 * var_weff_dn5)) * var_leff) - (assign13300_e12293 * var_leff_dn5)) / (var_leff * var_leff)), (((((((var_utotal_dn6 * var_cox1) + (var_utotal * var_cox1_dn6)) * var_weff) + (assign13300_e12291 * var_weff_dn6)) * var_leff) - (assign13300_e12293 * var_leff_dn6)) / (var_leff * var_leff)), (((((((var_utotal_dn7 * var_cox1) + (var_utotal * var_cox1_dn7)) * var_weff) + (assign13300_e12291 * var_weff_dn7)) * var_leff) - (assign13300_e12293 * var_leff_dn7)) / (var_leff * var_leff)), (((((((var_utotal_dn8 * var_cox1) + (var_utotal * var_cox1_dn8)) * var_weff) + (assign13300_e12291 * var_weff_dn8)) * var_leff) - (assign13300_e12293 * var_leff_dn8)) / (var_leff * var_leff)), (((((((var_utotal_db0 * var_cox1) + (var_utotal * var_cox1_db0)) * var_weff) + (assign13300_e12291 * var_weff_db0)) * var_leff) - (assign13300_e12293 * var_leff_db0)) / (var_leff * var_leff)), (((((((var_utotal_db1 * var_cox1) + (var_utotal * var_cox1_db1)) * var_weff) + (assign13300_e12291 * var_weff_db1)) * var_leff) - (assign13300_e12293 * var_leff_db1)) / (var_leff * var_leff)), (((((((var_utotal_db2 * var_cox1) + (var_utotal * var_cox1_db2)) * var_weff) + (assign13300_e12291 * var_weff_db2)) * var_leff) - (assign13300_e12293 * var_leff_db2)) / (var_leff * var_leff)), (((((((var_utotal_db3 * var_cox1) + (var_utotal * var_cox1_db3)) * var_weff) + (assign13300_e12291 * var_weff_db3)) * var_leff) - (assign13300_e12293 * var_leff_db3)) / (var_leff * var_leff)), (((((((var_utotal_db4 * var_cox1) + (var_utotal * var_cox1_db4)) * var_weff) + (assign13300_e12291 * var_weff_db4)) * var_leff) - (assign13300_e12293 * var_leff_db4)) / (var_leff * var_leff)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4,)
    }
};
        var_t0 = assign13300_e12297;
        var_t0_dn0 = assign13300_e12297_d_n0;
        var_t0_dn1 = assign13300_e12297_d_n1;
        var_t0_dn2 = assign13300_e12297_d_n2;
        var_t0_dn3 = assign13300_e12297_d_n3;
        var_t0_dn4 = assign13300_e12297_d_n4;
        var_t0_dn5 = assign13300_e12297_d_n5;
        var_t0_dn6 = assign13300_e12297_d_n6;
        var_t0_dn7 = assign13300_e12297_d_n7;
        var_t0_dn8 = assign13300_e12297_d_n8;
        var_t0_db0 = assign13300_e12297_d_b0;
        var_t0_db1 = assign13300_e12297_d_b1;
        var_t0_db2 = assign13300_e12297_d_b2;
        var_t0_db3 = assign13300_e12297_d_b3;
        var_t0_db4 = assign13300_e12297_d_b4;
        var_t0_rv = 0.0;
        var_t0_rdn0 = 0.0;
        var_t0_rdn1 = 0.0;
        var_t0_rdn2 = 0.0;
        var_t0_rdn3 = 0.0;
        var_t0_rdn4 = 0.0;
        var_t0_rdn5 = 0.0;
        var_t0_rdn6 = 0.0;
        var_t0_rdn7 = 0.0;
        var_t0_rdn8 = 0.0;
        var_t0_rdb0 = 0.0;
        var_t0_rdb1 = 0.0;
        var_t0_rdb2 = 0.0;
        var_t0_rdb3 = 0.0;
        var_t0_rdb4 = 0.0;

        let assign13510_e12396: f64 = if ((p.p18 != 0.0) && (p.p310 > 0.0)) { 1.0 } else { 0.0 };
        var_guard147 = assign13510_e12396;
        var_guard147_dn0 = 0.0;
        var_guard147_dn1 = 0.0;
        var_guard147_dn2 = 0.0;
        var_guard147_dn3 = 0.0;
        var_guard147_dn4 = 0.0;
        var_guard147_dn5 = 0.0;
        var_guard147_dn6 = 0.0;
        var_guard147_dn7 = 0.0;
        var_guard147_dn8 = 0.0;
        var_guard147_db0 = 0.0;
        var_guard147_db1 = 0.0;
        var_guard147_db2 = 0.0;
        var_guard147_db3 = 0.0;
        var_guard147_db4 = 0.0;
        var_guard147_rv = 0.0;
        var_guard147_rdn0 = 0.0;
        var_guard147_rdn1 = 0.0;
        var_guard147_rdn2 = 0.0;
        var_guard147_rdn3 = 0.0;
        var_guard147_rdn4 = 0.0;
        var_guard147_rdn5 = 0.0;
        var_guard147_rdn6 = 0.0;
        var_guard147_rdn7 = 0.0;
        var_guard147_rdn8 = 0.0;
        var_guard147_rdb0 = 0.0;
        var_guard147_rdb1 = 0.0;
        var_guard147_rdb2 = 0.0;
        var_guard147_rdb3 = 0.0;
        var_guard147_rdb4 = 0.0;


        *var_guard137_slot = var_guard137;
        *var_guard137_db0_slot = var_guard137_db0;
        *var_guard137_db1_slot = var_guard137_db1;
        *var_guard137_db2_slot = var_guard137_db2;
        *var_guard137_db3_slot = var_guard137_db3;
        *var_guard137_db4_slot = var_guard137_db4;
        *var_guard137_dn0_slot = var_guard137_dn0;
        *var_guard137_dn1_slot = var_guard137_dn1;
        *var_guard137_dn2_slot = var_guard137_dn2;
        *var_guard137_dn3_slot = var_guard137_dn3;
        *var_guard137_dn4_slot = var_guard137_dn4;
        *var_guard137_dn5_slot = var_guard137_dn5;
        *var_guard137_dn6_slot = var_guard137_dn6;
        *var_guard137_dn7_slot = var_guard137_dn7;
        *var_guard137_dn8_slot = var_guard137_dn8;
        *var_guard137_rdb0_slot = var_guard137_rdb0;
        *var_guard137_rdb1_slot = var_guard137_rdb1;
        *var_guard137_rdb2_slot = var_guard137_rdb2;
        *var_guard137_rdb3_slot = var_guard137_rdb3;
        *var_guard137_rdb4_slot = var_guard137_rdb4;
        *var_guard137_rdn0_slot = var_guard137_rdn0;
        *var_guard137_rdn1_slot = var_guard137_rdn1;
        *var_guard137_rdn2_slot = var_guard137_rdn2;
        *var_guard137_rdn3_slot = var_guard137_rdn3;
        *var_guard137_rdn4_slot = var_guard137_rdn4;
        *var_guard137_rdn5_slot = var_guard137_rdn5;
        *var_guard137_rdn6_slot = var_guard137_rdn6;
        *var_guard137_rdn7_slot = var_guard137_rdn7;
        *var_guard137_rdn8_slot = var_guard137_rdn8;
        *var_guard137_rv_slot = var_guard137_rv;
        *var_guard147_slot = var_guard147;
        *var_guard147_db0_slot = var_guard147_db0;
        *var_guard147_db1_slot = var_guard147_db1;
        *var_guard147_db2_slot = var_guard147_db2;
        *var_guard147_db3_slot = var_guard147_db3;
        *var_guard147_db4_slot = var_guard147_db4;
        *var_guard147_dn0_slot = var_guard147_dn0;
        *var_guard147_dn1_slot = var_guard147_dn1;
        *var_guard147_dn2_slot = var_guard147_dn2;
        *var_guard147_dn3_slot = var_guard147_dn3;
        *var_guard147_dn4_slot = var_guard147_dn4;
        *var_guard147_dn5_slot = var_guard147_dn5;
        *var_guard147_dn6_slot = var_guard147_dn6;
        *var_guard147_dn7_slot = var_guard147_dn7;
        *var_guard147_dn8_slot = var_guard147_dn8;
        *var_guard147_rdb0_slot = var_guard147_rdb0;
        *var_guard147_rdb1_slot = var_guard147_rdb1;
        *var_guard147_rdb2_slot = var_guard147_rdb2;
        *var_guard147_rdb3_slot = var_guard147_rdb3;
        *var_guard147_rdb4_slot = var_guard147_rdb4;
        *var_guard147_rdn0_slot = var_guard147_rdn0;
        *var_guard147_rdn1_slot = var_guard147_rdn1;
        *var_guard147_rdn2_slot = var_guard147_rdn2;
        *var_guard147_rdn3_slot = var_guard147_rdn3;
        *var_guard147_rdn4_slot = var_guard147_rdn4;
        *var_guard147_rdn5_slot = var_guard147_rdn5;
        *var_guard147_rdn6_slot = var_guard147_rdn6;
        *var_guard147_rdn7_slot = var_guard147_rdn7;
        *var_guard147_rdn8_slot = var_guard147_rdn8;
        *var_guard147_rv_slot = var_guard147_rv;
        *var_t0_slot = var_t0;
        *var_t0_db0_slot = var_t0_db0;
        *var_t0_db1_slot = var_t0_db1;
        *var_t0_db2_slot = var_t0_db2;
        *var_t0_db3_slot = var_t0_db3;
        *var_t0_db4_slot = var_t0_db4;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rdb0_slot = var_t0_rdb0;
        *var_t0_rdb1_slot = var_t0_rdb1;
        *var_t0_rdb2_slot = var_t0_rdb2;
        *var_t0_rdb3_slot = var_t0_rdb3;
        *var_t0_rdb4_slot = var_t0_rdb4;
        *var_t0_rdn0_slot = var_t0_rdn0;
        *var_t0_rdn1_slot = var_t0_rdn1;
        *var_t0_rdn2_slot = var_t0_rdn2;
        *var_t0_rdn3_slot = var_t0_rdn3;
        *var_t0_rdn4_slot = var_t0_rdn4;
        *var_t0_rdn5_slot = var_t0_rdn5;
        *var_t0_rdn6_slot = var_t0_rdn6;
        *var_t0_rdn7_slot = var_t0_rdn7;
        *var_t0_rdn8_slot = var_t0_rdn8;
        *var_t0_rv_slot = var_t0_rv;
    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        var_devsign: f64,
        var_guard138: f64,
        var_ids: f64,
        var_ids_db0: f64,
        var_ids_db1: f64,
        var_ids_db2: f64,
        var_ids_db3: f64,
        var_ids_db4: f64,
        var_ids_dn0: f64,
        var_ids_dn1: f64,
        var_ids_dn2: f64,
        var_ids_dn3: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_igbd: f64,
        var_igbd_db0: f64,
        var_igbd_db1: f64,
        var_igbd_db2: f64,
        var_igbd_db3: f64,
        var_igbd_db4: f64,
        var_igbd_dn0: f64,
        var_igbd_dn1: f64,
        var_igbd_dn2: f64,
        var_igbd_dn3: f64,
        var_igbd_dn4: f64,
        var_igbd_dn5: f64,
        var_igbd_dn6: f64,
        var_igbd_dn7: f64,
        var_igbd_dn8: f64,
        var_igbs: f64,
        var_igbs_db0: f64,
        var_igbs_db1: f64,
        var_igbs_db2: f64,
        var_igbs_db3: f64,
        var_igbs_db4: f64,
        var_igbs_dn0: f64,
        var_igbs_dn1: f64,
        var_igbs_dn2: f64,
        var_igbs_dn3: f64,
        var_igbs_dn4: f64,
        var_igbs_dn5: f64,
        var_igbs_dn6: f64,
        var_igbs_dn7: f64,
        var_igbs_dn8: f64,
        var_igcd: f64,
        var_igcd_db0: f64,
        var_igcd_db1: f64,
        var_igcd_db2: f64,
        var_igcd_db3: f64,
        var_igcd_db4: f64,
        var_igcd_dn0: f64,
        var_igcd_dn1: f64,
        var_igcd_dn2: f64,
        var_igcd_dn3: f64,
        var_igcd_dn4: f64,
        var_igcd_dn5: f64,
        var_igcd_dn6: f64,
        var_igcd_dn7: f64,
        var_igcd_dn8: f64,
        var_igcs: f64,
        var_igcs_db0: f64,
        var_igcs_db1: f64,
        var_igcs_db2: f64,
        var_igcs_db3: f64,
        var_igcs_db4: f64,
        var_igcs_dn0: f64,
        var_igcs_dn1: f64,
        var_igcs_dn2: f64,
        var_igcs_dn3: f64,
        var_igcs_dn4: f64,
        var_igcs_dn5: f64,
        var_igcs_dn6: f64,
        var_igcs_dn7: f64,
        var_igcs_dn8: f64,
        var_igd: f64,
        var_igd_db0: f64,
        var_igd_db1: f64,
        var_igd_db2: f64,
        var_igd_db3: f64,
        var_igd_db4: f64,
        var_igd_dn0: f64,
        var_igd_dn1: f64,
        var_igd_dn2: f64,
        var_igd_dn3: f64,
        var_igd_dn4: f64,
        var_igd_dn5: f64,
        var_igd_dn6: f64,
        var_igd_dn7: f64,
        var_igd_dn8: f64,
        var_igidl: f64,
        var_igidl_db0: f64,
        var_igidl_db1: f64,
        var_igidl_db2: f64,
        var_igidl_db3: f64,
        var_igidl_db4: f64,
        var_igidl_dn0: f64,
        var_igidl_dn1: f64,
        var_igidl_dn2: f64,
        var_igidl_dn3: f64,
        var_igidl_dn4: f64,
        var_igidl_dn5: f64,
        var_igidl_dn6: f64,
        var_igidl_dn7: f64,
        var_igidl_dn8: f64,
        var_igisl: f64,
        var_igisl_db0: f64,
        var_igisl_db1: f64,
        var_igisl_db2: f64,
        var_igisl_db3: f64,
        var_igisl_db4: f64,
        var_igisl_dn0: f64,
        var_igisl_dn1: f64,
        var_igisl_dn2: f64,
        var_igisl_dn3: f64,
        var_igisl_dn4: f64,
        var_igisl_dn5: f64,
        var_igisl_dn6: f64,
        var_igisl_dn7: f64,
        var_igisl_dn8: f64,
        var_igs: f64,
        var_igs_db0: f64,
        var_igs_db1: f64,
        var_igs_db2: f64,
        var_igs_db3: f64,
        var_igs_db4: f64,
        var_igs_dn0: f64,
        var_igs_dn1: f64,
        var_igs_dn2: f64,
        var_igs_dn3: f64,
        var_igs_dn4: f64,
        var_igs_dn5: f64,
        var_igs_dn6: f64,
        var_igs_dn7: f64,
        var_igs_dn8: f64,
        var_iii: f64,
        var_iii_db0: f64,
        var_iii_db1: f64,
        var_iii_db2: f64,
        var_iii_db3: f64,
        var_iii_db4: f64,
        var_iii_dn0: f64,
        var_iii_dn1: f64,
        var_iii_dn2: f64,
        var_iii_dn3: f64,
        var_iii_dn4: f64,
        var_iii_dn5: f64,
        var_iii_dn6: f64,
        var_iii_dn7: f64,
        var_iii_dn8: f64,
        var_qdi: f64,
        var_qdi_db0: f64,
        var_qdi_db1: f64,
        var_qdi_db2: f64,
        var_qdi_db3: f64,
        var_qdi_db4: f64,
        var_qdi_dn0: f64,
        var_qdi_dn1: f64,
        var_qdi_dn2: f64,
        var_qdi_dn3: f64,
        var_qdi_dn4: f64,
        var_qdi_dn5: f64,
        var_qdi_dn6: f64,
        var_qdi_dn7: f64,
        var_qdi_dn8: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq0_e787, eq0_e787_d_n0, eq0_e787_d_n1, eq0_e787_d_n2, eq0_e787_d_n3, eq0_e787_d_n4, eq0_e787_d_n5, eq0_e787_d_n6, eq0_e787_d_n7, eq0_e787_d_n8, eq0_e787_d_b0, eq0_e787_d_b1, eq0_e787_d_b2, eq0_e787_d_b3, eq0_e787_d_b4,) = {
    if (var_guard138 != 0.0) {
        let eq0_e779: f64 = (var_devsign * var_ids);
        let eq0_e779_d_n0: f64 = (var_devsign * var_ids_dn0);
        let eq0_e779_d_n1: f64 = (var_devsign * var_ids_dn1);
        let eq0_e779_d_n2: f64 = (var_devsign * var_ids_dn2);
        let eq0_e779_d_n3: f64 = (var_devsign * var_ids_dn3);
        let eq0_e779_d_n4: f64 = (var_devsign * var_ids_dn4);
        let eq0_e779_d_n5: f64 = (var_devsign * var_ids_dn5);
        let eq0_e779_d_n6: f64 = (var_devsign * var_ids_dn6);
        let eq0_e779_d_n7: f64 = (var_devsign * var_ids_dn7);
        let eq0_e779_d_n8: f64 = (var_devsign * var_ids_dn8);
        let eq0_e779_d_b0: f64 = (var_devsign * var_ids_db0);
        let eq0_e779_d_b1: f64 = (var_devsign * var_ids_db1);
        let eq0_e779_d_b2: f64 = (var_devsign * var_ids_db2);
        let eq0_e779_d_b3: f64 = (var_devsign * var_ids_db3);
        let eq0_e779_d_b4: f64 = (var_devsign * var_ids_db4);
        let eq0_e782: f64 = 1e-12;
        let eq0_e784: f64 = (eq0_e782 * (nv5 - nv6));
        let eq0_e785: f64 = (eq0_e779 + eq0_e784);
        let eq0_e785_d_n5: f64 = (eq0_e779_d_n5 + eq0_e782);
        let eq0_e785_d_n6: f64 = (eq0_e779_d_n6 + (-eq0_e782));
        (eq0_e785, eq0_e779_d_n0, eq0_e779_d_n1, eq0_e779_d_n2, eq0_e779_d_n3, eq0_e779_d_n4, eq0_e785_d_n5, eq0_e785_d_n6, eq0_e779_d_n7, eq0_e779_d_n8, eq0_e779_d_b0, eq0_e779_d_b1, eq0_e779_d_b2, eq0_e779_d_b3, eq0_e779_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e787;
        let eq0_node_derivatives: [f64; 9] = [eq0_e787_d_n0, eq0_e787_d_n1, eq0_e787_d_n2, eq0_e787_d_n3, eq0_e787_d_n4, eq0_e787_d_n5, eq0_e787_d_n6, eq0_e787_d_n7, eq0_e787_d_n8];
        let eq0_branch_derivatives: [f64; 5] = [eq0_e787_d_b0, eq0_e787_d_b1, eq0_e787_d_b2, eq0_e787_d_b3, eq0_e787_d_b4];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e795, eq1_e795_d_n0, eq1_e795_d_n1, eq1_e795_d_n2, eq1_e795_d_n3, eq1_e795_d_n4, eq1_e795_d_n5, eq1_e795_d_n6, eq1_e795_d_n7, eq1_e795_d_n8, eq1_e795_d_b0, eq1_e795_d_b1, eq1_e795_d_b2, eq1_e795_d_b3, eq1_e795_d_b4,) = {
    if (var_guard138 != 0.0) {
        let eq1_e792: f64 = (var_igidl + var_iii);
        let eq1_e792_d_n0: f64 = (var_igidl_dn0 + var_iii_dn0);
        let eq1_e792_d_n1: f64 = (var_igidl_dn1 + var_iii_dn1);
        let eq1_e792_d_n2: f64 = (var_igidl_dn2 + var_iii_dn2);
        let eq1_e792_d_n3: f64 = (var_igidl_dn3 + var_iii_dn3);
        let eq1_e792_d_n4: f64 = (var_igidl_dn4 + var_iii_dn4);
        let eq1_e792_d_n5: f64 = (var_igidl_dn5 + var_iii_dn5);
        let eq1_e792_d_n6: f64 = (var_igidl_dn6 + var_iii_dn6);
        let eq1_e792_d_n7: f64 = (var_igidl_dn7 + var_iii_dn7);
        let eq1_e792_d_n8: f64 = (var_igidl_dn8 + var_iii_dn8);
        let eq1_e792_d_b0: f64 = (var_igidl_db0 + var_iii_db0);
        let eq1_e792_d_b1: f64 = (var_igidl_db1 + var_iii_db1);
        let eq1_e792_d_b2: f64 = (var_igidl_db2 + var_iii_db2);
        let eq1_e792_d_b3: f64 = (var_igidl_db3 + var_iii_db3);
        let eq1_e792_d_b4: f64 = (var_igidl_db4 + var_iii_db4);
        let eq1_e793: f64 = (var_devsign * eq1_e792);
        let eq1_e793_d_n0: f64 = (var_devsign * eq1_e792_d_n0);
        let eq1_e793_d_n1: f64 = (var_devsign * eq1_e792_d_n1);
        let eq1_e793_d_n2: f64 = (var_devsign * eq1_e792_d_n2);
        let eq1_e793_d_n3: f64 = (var_devsign * eq1_e792_d_n3);
        let eq1_e793_d_n4: f64 = (var_devsign * eq1_e792_d_n4);
        let eq1_e793_d_n5: f64 = (var_devsign * eq1_e792_d_n5);
        let eq1_e793_d_n6: f64 = (var_devsign * eq1_e792_d_n6);
        let eq1_e793_d_n7: f64 = (var_devsign * eq1_e792_d_n7);
        let eq1_e793_d_n8: f64 = (var_devsign * eq1_e792_d_n8);
        let eq1_e793_d_b0: f64 = (var_devsign * eq1_e792_d_b0);
        let eq1_e793_d_b1: f64 = (var_devsign * eq1_e792_d_b1);
        let eq1_e793_d_b2: f64 = (var_devsign * eq1_e792_d_b2);
        let eq1_e793_d_b3: f64 = (var_devsign * eq1_e792_d_b3);
        let eq1_e793_d_b4: f64 = (var_devsign * eq1_e792_d_b4);
        (eq1_e793, eq1_e793_d_n0, eq1_e793_d_n1, eq1_e793_d_n2, eq1_e793_d_n3, eq1_e793_d_n4, eq1_e793_d_n5, eq1_e793_d_n6, eq1_e793_d_n7, eq1_e793_d_n8, eq1_e793_d_b0, eq1_e793_d_b1, eq1_e793_d_b2, eq1_e793_d_b3, eq1_e793_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e795;
        let eq1_node_derivatives: [f64; 9] = [eq1_e795_d_n0, eq1_e795_d_n1, eq1_e795_d_n2, eq1_e795_d_n3, eq1_e795_d_n4, eq1_e795_d_n5, eq1_e795_d_n6, eq1_e795_d_n7, eq1_e795_d_n8];
        let eq1_branch_derivatives: [f64; 5] = [eq1_e795_d_b0, eq1_e795_d_b1, eq1_e795_d_b2, eq1_e795_d_b3, eq1_e795_d_b4];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e801, eq2_e801_d_n0, eq2_e801_d_n1, eq2_e801_d_n2, eq2_e801_d_n3, eq2_e801_d_n4, eq2_e801_d_n5, eq2_e801_d_n6, eq2_e801_d_n7, eq2_e801_d_n8, eq2_e801_d_b0, eq2_e801_d_b1, eq2_e801_d_b2, eq2_e801_d_b3, eq2_e801_d_b4,) = {
    if (var_guard138 != 0.0) {
        let eq2_e799: f64 = (var_devsign * var_igisl);
        let eq2_e799_d_n0: f64 = (var_devsign * var_igisl_dn0);
        let eq2_e799_d_n1: f64 = (var_devsign * var_igisl_dn1);
        let eq2_e799_d_n2: f64 = (var_devsign * var_igisl_dn2);
        let eq2_e799_d_n3: f64 = (var_devsign * var_igisl_dn3);
        let eq2_e799_d_n4: f64 = (var_devsign * var_igisl_dn4);
        let eq2_e799_d_n5: f64 = (var_devsign * var_igisl_dn5);
        let eq2_e799_d_n6: f64 = (var_devsign * var_igisl_dn6);
        let eq2_e799_d_n7: f64 = (var_devsign * var_igisl_dn7);
        let eq2_e799_d_n8: f64 = (var_devsign * var_igisl_dn8);
        let eq2_e799_d_b0: f64 = (var_devsign * var_igisl_db0);
        let eq2_e799_d_b1: f64 = (var_devsign * var_igisl_db1);
        let eq2_e799_d_b2: f64 = (var_devsign * var_igisl_db2);
        let eq2_e799_d_b3: f64 = (var_devsign * var_igisl_db3);
        let eq2_e799_d_b4: f64 = (var_devsign * var_igisl_db4);
        (eq2_e799, eq2_e799_d_n0, eq2_e799_d_n1, eq2_e799_d_n2, eq2_e799_d_n3, eq2_e799_d_n4, eq2_e799_d_n5, eq2_e799_d_n6, eq2_e799_d_n7, eq2_e799_d_n8, eq2_e799_d_b0, eq2_e799_d_b1, eq2_e799_d_b2, eq2_e799_d_b3, eq2_e799_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e801;
        let eq2_node_derivatives: [f64; 9] = [eq2_e801_d_n0, eq2_e801_d_n1, eq2_e801_d_n2, eq2_e801_d_n3, eq2_e801_d_n4, eq2_e801_d_n5, eq2_e801_d_n6, eq2_e801_d_n7, eq2_e801_d_n8];
        let eq2_branch_derivatives: [f64; 5] = [eq2_e801_d_b0, eq2_e801_d_b1, eq2_e801_d_b2, eq2_e801_d_b3, eq2_e801_d_b4];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e809, eq3_e809_d_n0, eq3_e809_d_n1, eq3_e809_d_n2, eq3_e809_d_n3, eq3_e809_d_n4, eq3_e809_d_n5, eq3_e809_d_n6, eq3_e809_d_n7, eq3_e809_d_n8, eq3_e809_d_b0, eq3_e809_d_b1, eq3_e809_d_b2, eq3_e809_d_b3, eq3_e809_d_b4,) = {
    if (var_guard138 != 0.0) {
        let eq3_e806: f64 = (var_igcs + var_igs);
        let eq3_e806_d_n0: f64 = (var_igcs_dn0 + var_igs_dn0);
        let eq3_e806_d_n1: f64 = (var_igcs_dn1 + var_igs_dn1);
        let eq3_e806_d_n2: f64 = (var_igcs_dn2 + var_igs_dn2);
        let eq3_e806_d_n3: f64 = (var_igcs_dn3 + var_igs_dn3);
        let eq3_e806_d_n4: f64 = (var_igcs_dn4 + var_igs_dn4);
        let eq3_e806_d_n5: f64 = (var_igcs_dn5 + var_igs_dn5);
        let eq3_e806_d_n6: f64 = (var_igcs_dn6 + var_igs_dn6);
        let eq3_e806_d_n7: f64 = (var_igcs_dn7 + var_igs_dn7);
        let eq3_e806_d_n8: f64 = (var_igcs_dn8 + var_igs_dn8);
        let eq3_e806_d_b0: f64 = (var_igcs_db0 + var_igs_db0);
        let eq3_e806_d_b1: f64 = (var_igcs_db1 + var_igs_db1);
        let eq3_e806_d_b2: f64 = (var_igcs_db2 + var_igs_db2);
        let eq3_e806_d_b3: f64 = (var_igcs_db3 + var_igs_db3);
        let eq3_e806_d_b4: f64 = (var_igcs_db4 + var_igs_db4);
        let eq3_e807: f64 = (var_devsign * eq3_e806);
        let eq3_e807_d_n0: f64 = (var_devsign * eq3_e806_d_n0);
        let eq3_e807_d_n1: f64 = (var_devsign * eq3_e806_d_n1);
        let eq3_e807_d_n2: f64 = (var_devsign * eq3_e806_d_n2);
        let eq3_e807_d_n3: f64 = (var_devsign * eq3_e806_d_n3);
        let eq3_e807_d_n4: f64 = (var_devsign * eq3_e806_d_n4);
        let eq3_e807_d_n5: f64 = (var_devsign * eq3_e806_d_n5);
        let eq3_e807_d_n6: f64 = (var_devsign * eq3_e806_d_n6);
        let eq3_e807_d_n7: f64 = (var_devsign * eq3_e806_d_n7);
        let eq3_e807_d_n8: f64 = (var_devsign * eq3_e806_d_n8);
        let eq3_e807_d_b0: f64 = (var_devsign * eq3_e806_d_b0);
        let eq3_e807_d_b1: f64 = (var_devsign * eq3_e806_d_b1);
        let eq3_e807_d_b2: f64 = (var_devsign * eq3_e806_d_b2);
        let eq3_e807_d_b3: f64 = (var_devsign * eq3_e806_d_b3);
        let eq3_e807_d_b4: f64 = (var_devsign * eq3_e806_d_b4);
        (eq3_e807, eq3_e807_d_n0, eq3_e807_d_n1, eq3_e807_d_n2, eq3_e807_d_n3, eq3_e807_d_n4, eq3_e807_d_n5, eq3_e807_d_n6, eq3_e807_d_n7, eq3_e807_d_n8, eq3_e807_d_b0, eq3_e807_d_b1, eq3_e807_d_b2, eq3_e807_d_b3, eq3_e807_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e809;
        let eq3_node_derivatives: [f64; 9] = [eq3_e809_d_n0, eq3_e809_d_n1, eq3_e809_d_n2, eq3_e809_d_n3, eq3_e809_d_n4, eq3_e809_d_n5, eq3_e809_d_n6, eq3_e809_d_n7, eq3_e809_d_n8];
        let eq3_branch_derivatives: [f64; 5] = [eq3_e809_d_b0, eq3_e809_d_b1, eq3_e809_d_b2, eq3_e809_d_b3, eq3_e809_d_b4];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e817, eq4_e817_d_n0, eq4_e817_d_n1, eq4_e817_d_n2, eq4_e817_d_n3, eq4_e817_d_n4, eq4_e817_d_n5, eq4_e817_d_n6, eq4_e817_d_n7, eq4_e817_d_n8, eq4_e817_d_b0, eq4_e817_d_b1, eq4_e817_d_b2, eq4_e817_d_b3, eq4_e817_d_b4,) = {
    if (var_guard138 != 0.0) {
        let eq4_e814: f64 = (var_igcd + var_igd);
        let eq4_e814_d_n0: f64 = (var_igcd_dn0 + var_igd_dn0);
        let eq4_e814_d_n1: f64 = (var_igcd_dn1 + var_igd_dn1);
        let eq4_e814_d_n2: f64 = (var_igcd_dn2 + var_igd_dn2);
        let eq4_e814_d_n3: f64 = (var_igcd_dn3 + var_igd_dn3);
        let eq4_e814_d_n4: f64 = (var_igcd_dn4 + var_igd_dn4);
        let eq4_e814_d_n5: f64 = (var_igcd_dn5 + var_igd_dn5);
        let eq4_e814_d_n6: f64 = (var_igcd_dn6 + var_igd_dn6);
        let eq4_e814_d_n7: f64 = (var_igcd_dn7 + var_igd_dn7);
        let eq4_e814_d_n8: f64 = (var_igcd_dn8 + var_igd_dn8);
        let eq4_e814_d_b0: f64 = (var_igcd_db0 + var_igd_db0);
        let eq4_e814_d_b1: f64 = (var_igcd_db1 + var_igd_db1);
        let eq4_e814_d_b2: f64 = (var_igcd_db2 + var_igd_db2);
        let eq4_e814_d_b3: f64 = (var_igcd_db3 + var_igd_db3);
        let eq4_e814_d_b4: f64 = (var_igcd_db4 + var_igd_db4);
        let eq4_e815: f64 = (var_devsign * eq4_e814);
        let eq4_e815_d_n0: f64 = (var_devsign * eq4_e814_d_n0);
        let eq4_e815_d_n1: f64 = (var_devsign * eq4_e814_d_n1);
        let eq4_e815_d_n2: f64 = (var_devsign * eq4_e814_d_n2);
        let eq4_e815_d_n3: f64 = (var_devsign * eq4_e814_d_n3);
        let eq4_e815_d_n4: f64 = (var_devsign * eq4_e814_d_n4);
        let eq4_e815_d_n5: f64 = (var_devsign * eq4_e814_d_n5);
        let eq4_e815_d_n6: f64 = (var_devsign * eq4_e814_d_n6);
        let eq4_e815_d_n7: f64 = (var_devsign * eq4_e814_d_n7);
        let eq4_e815_d_n8: f64 = (var_devsign * eq4_e814_d_n8);
        let eq4_e815_d_b0: f64 = (var_devsign * eq4_e814_d_b0);
        let eq4_e815_d_b1: f64 = (var_devsign * eq4_e814_d_b1);
        let eq4_e815_d_b2: f64 = (var_devsign * eq4_e814_d_b2);
        let eq4_e815_d_b3: f64 = (var_devsign * eq4_e814_d_b3);
        let eq4_e815_d_b4: f64 = (var_devsign * eq4_e814_d_b4);
        (eq4_e815, eq4_e815_d_n0, eq4_e815_d_n1, eq4_e815_d_n2, eq4_e815_d_n3, eq4_e815_d_n4, eq4_e815_d_n5, eq4_e815_d_n6, eq4_e815_d_n7, eq4_e815_d_n8, eq4_e815_d_b0, eq4_e815_d_b1, eq4_e815_d_b2, eq4_e815_d_b3, eq4_e815_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e817;
        let eq4_node_derivatives: [f64; 9] = [eq4_e817_d_n0, eq4_e817_d_n1, eq4_e817_d_n2, eq4_e817_d_n3, eq4_e817_d_n4, eq4_e817_d_n5, eq4_e817_d_n6, eq4_e817_d_n7, eq4_e817_d_n8];
        let eq4_branch_derivatives: [f64; 5] = [eq4_e817_d_b0, eq4_e817_d_b1, eq4_e817_d_b2, eq4_e817_d_b3, eq4_e817_d_b4];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e830, eq5_e830_d_n0, eq5_e830_d_n1, eq5_e830_d_n2, eq5_e830_d_n3, eq5_e830_d_n4, eq5_e830_d_n5, eq5_e830_d_n6, eq5_e830_d_n7, eq5_e830_d_n8, eq5_e830_d_b0, eq5_e830_d_b1, eq5_e830_d_b2, eq5_e830_d_b3, eq5_e830_d_b4,) = {
    if (var_guard138 == 0.0) {
        let eq5_e822: f64 = (var_devsign * var_ids);
        let eq5_e822_d_n0: f64 = (var_devsign * var_ids_dn0);
        let eq5_e822_d_n1: f64 = (var_devsign * var_ids_dn1);
        let eq5_e822_d_n2: f64 = (var_devsign * var_ids_dn2);
        let eq5_e822_d_n3: f64 = (var_devsign * var_ids_dn3);
        let eq5_e822_d_n4: f64 = (var_devsign * var_ids_dn4);
        let eq5_e822_d_n5: f64 = (var_devsign * var_ids_dn5);
        let eq5_e822_d_n6: f64 = (var_devsign * var_ids_dn6);
        let eq5_e822_d_n7: f64 = (var_devsign * var_ids_dn7);
        let eq5_e822_d_n8: f64 = (var_devsign * var_ids_dn8);
        let eq5_e822_d_b0: f64 = (var_devsign * var_ids_db0);
        let eq5_e822_d_b1: f64 = (var_devsign * var_ids_db1);
        let eq5_e822_d_b2: f64 = (var_devsign * var_ids_db2);
        let eq5_e822_d_b3: f64 = (var_devsign * var_ids_db3);
        let eq5_e822_d_b4: f64 = (var_devsign * var_ids_db4);
        let eq5_e825: f64 = 1e-12;
        let eq5_e827: f64 = (eq5_e825 * (nv6 - nv5));
        let eq5_e828: f64 = (eq5_e822 + eq5_e827);
        let eq5_e828_d_n5: f64 = (eq5_e822_d_n5 + (-eq5_e825));
        let eq5_e828_d_n6: f64 = (eq5_e822_d_n6 + eq5_e825);
        (eq5_e828, eq5_e822_d_n0, eq5_e822_d_n1, eq5_e822_d_n2, eq5_e822_d_n3, eq5_e822_d_n4, eq5_e828_d_n5, eq5_e828_d_n6, eq5_e822_d_n7, eq5_e822_d_n8, eq5_e822_d_b0, eq5_e822_d_b1, eq5_e822_d_b2, eq5_e822_d_b3, eq5_e822_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e830;
        let eq5_node_derivatives: [f64; 9] = [eq5_e830_d_n0, eq5_e830_d_n1, eq5_e830_d_n2, eq5_e830_d_n3, eq5_e830_d_n4, eq5_e830_d_n5, eq5_e830_d_n6, eq5_e830_d_n7, eq5_e830_d_n8];
        let eq5_branch_derivatives: [f64; 5] = [eq5_e830_d_b0, eq5_e830_d_b1, eq5_e830_d_b2, eq5_e830_d_b3, eq5_e830_d_b4];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e839, eq6_e839_d_n0, eq6_e839_d_n1, eq6_e839_d_n2, eq6_e839_d_n3, eq6_e839_d_n4, eq6_e839_d_n5, eq6_e839_d_n6, eq6_e839_d_n7, eq6_e839_d_n8, eq6_e839_d_b0, eq6_e839_d_b1, eq6_e839_d_b2, eq6_e839_d_b3, eq6_e839_d_b4,) = {
    if (var_guard138 == 0.0) {
        let eq6_e836: f64 = (var_igidl + var_iii);
        let eq6_e836_d_n0: f64 = (var_igidl_dn0 + var_iii_dn0);
        let eq6_e836_d_n1: f64 = (var_igidl_dn1 + var_iii_dn1);
        let eq6_e836_d_n2: f64 = (var_igidl_dn2 + var_iii_dn2);
        let eq6_e836_d_n3: f64 = (var_igidl_dn3 + var_iii_dn3);
        let eq6_e836_d_n4: f64 = (var_igidl_dn4 + var_iii_dn4);
        let eq6_e836_d_n5: f64 = (var_igidl_dn5 + var_iii_dn5);
        let eq6_e836_d_n6: f64 = (var_igidl_dn6 + var_iii_dn6);
        let eq6_e836_d_n7: f64 = (var_igidl_dn7 + var_iii_dn7);
        let eq6_e836_d_n8: f64 = (var_igidl_dn8 + var_iii_dn8);
        let eq6_e836_d_b0: f64 = (var_igidl_db0 + var_iii_db0);
        let eq6_e836_d_b1: f64 = (var_igidl_db1 + var_iii_db1);
        let eq6_e836_d_b2: f64 = (var_igidl_db2 + var_iii_db2);
        let eq6_e836_d_b3: f64 = (var_igidl_db3 + var_iii_db3);
        let eq6_e836_d_b4: f64 = (var_igidl_db4 + var_iii_db4);
        let eq6_e837: f64 = (var_devsign * eq6_e836);
        let eq6_e837_d_n0: f64 = (var_devsign * eq6_e836_d_n0);
        let eq6_e837_d_n1: f64 = (var_devsign * eq6_e836_d_n1);
        let eq6_e837_d_n2: f64 = (var_devsign * eq6_e836_d_n2);
        let eq6_e837_d_n3: f64 = (var_devsign * eq6_e836_d_n3);
        let eq6_e837_d_n4: f64 = (var_devsign * eq6_e836_d_n4);
        let eq6_e837_d_n5: f64 = (var_devsign * eq6_e836_d_n5);
        let eq6_e837_d_n6: f64 = (var_devsign * eq6_e836_d_n6);
        let eq6_e837_d_n7: f64 = (var_devsign * eq6_e836_d_n7);
        let eq6_e837_d_n8: f64 = (var_devsign * eq6_e836_d_n8);
        let eq6_e837_d_b0: f64 = (var_devsign * eq6_e836_d_b0);
        let eq6_e837_d_b1: f64 = (var_devsign * eq6_e836_d_b1);
        let eq6_e837_d_b2: f64 = (var_devsign * eq6_e836_d_b2);
        let eq6_e837_d_b3: f64 = (var_devsign * eq6_e836_d_b3);
        let eq6_e837_d_b4: f64 = (var_devsign * eq6_e836_d_b4);
        (eq6_e837, eq6_e837_d_n0, eq6_e837_d_n1, eq6_e837_d_n2, eq6_e837_d_n3, eq6_e837_d_n4, eq6_e837_d_n5, eq6_e837_d_n6, eq6_e837_d_n7, eq6_e837_d_n8, eq6_e837_d_b0, eq6_e837_d_b1, eq6_e837_d_b2, eq6_e837_d_b3, eq6_e837_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e839;
        let eq6_node_derivatives: [f64; 9] = [eq6_e839_d_n0, eq6_e839_d_n1, eq6_e839_d_n2, eq6_e839_d_n3, eq6_e839_d_n4, eq6_e839_d_n5, eq6_e839_d_n6, eq6_e839_d_n7, eq6_e839_d_n8];
        let eq6_branch_derivatives: [f64; 5] = [eq6_e839_d_b0, eq6_e839_d_b1, eq6_e839_d_b2, eq6_e839_d_b3, eq6_e839_d_b4];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e846, eq7_e846_d_n0, eq7_e846_d_n1, eq7_e846_d_n2, eq7_e846_d_n3, eq7_e846_d_n4, eq7_e846_d_n5, eq7_e846_d_n6, eq7_e846_d_n7, eq7_e846_d_n8, eq7_e846_d_b0, eq7_e846_d_b1, eq7_e846_d_b2, eq7_e846_d_b3, eq7_e846_d_b4,) = {
    if (var_guard138 == 0.0) {
        let eq7_e844: f64 = (var_devsign * var_igisl);
        let eq7_e844_d_n0: f64 = (var_devsign * var_igisl_dn0);
        let eq7_e844_d_n1: f64 = (var_devsign * var_igisl_dn1);
        let eq7_e844_d_n2: f64 = (var_devsign * var_igisl_dn2);
        let eq7_e844_d_n3: f64 = (var_devsign * var_igisl_dn3);
        let eq7_e844_d_n4: f64 = (var_devsign * var_igisl_dn4);
        let eq7_e844_d_n5: f64 = (var_devsign * var_igisl_dn5);
        let eq7_e844_d_n6: f64 = (var_devsign * var_igisl_dn6);
        let eq7_e844_d_n7: f64 = (var_devsign * var_igisl_dn7);
        let eq7_e844_d_n8: f64 = (var_devsign * var_igisl_dn8);
        let eq7_e844_d_b0: f64 = (var_devsign * var_igisl_db0);
        let eq7_e844_d_b1: f64 = (var_devsign * var_igisl_db1);
        let eq7_e844_d_b2: f64 = (var_devsign * var_igisl_db2);
        let eq7_e844_d_b3: f64 = (var_devsign * var_igisl_db3);
        let eq7_e844_d_b4: f64 = (var_devsign * var_igisl_db4);
        (eq7_e844, eq7_e844_d_n0, eq7_e844_d_n1, eq7_e844_d_n2, eq7_e844_d_n3, eq7_e844_d_n4, eq7_e844_d_n5, eq7_e844_d_n6, eq7_e844_d_n7, eq7_e844_d_n8, eq7_e844_d_b0, eq7_e844_d_b1, eq7_e844_d_b2, eq7_e844_d_b3, eq7_e844_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e846;
        let eq7_node_derivatives: [f64; 9] = [eq7_e846_d_n0, eq7_e846_d_n1, eq7_e846_d_n2, eq7_e846_d_n3, eq7_e846_d_n4, eq7_e846_d_n5, eq7_e846_d_n6, eq7_e846_d_n7, eq7_e846_d_n8];
        let eq7_branch_derivatives: [f64; 5] = [eq7_e846_d_b0, eq7_e846_d_b1, eq7_e846_d_b2, eq7_e846_d_b3, eq7_e846_d_b4];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e855, eq8_e855_d_n0, eq8_e855_d_n1, eq8_e855_d_n2, eq8_e855_d_n3, eq8_e855_d_n4, eq8_e855_d_n5, eq8_e855_d_n6, eq8_e855_d_n7, eq8_e855_d_n8, eq8_e855_d_b0, eq8_e855_d_b1, eq8_e855_d_b2, eq8_e855_d_b3, eq8_e855_d_b4,) = {
    if (var_guard138 == 0.0) {
        let eq8_e852: f64 = (var_igcs + var_igs);
        let eq8_e852_d_n0: f64 = (var_igcs_dn0 + var_igs_dn0);
        let eq8_e852_d_n1: f64 = (var_igcs_dn1 + var_igs_dn1);
        let eq8_e852_d_n2: f64 = (var_igcs_dn2 + var_igs_dn2);
        let eq8_e852_d_n3: f64 = (var_igcs_dn3 + var_igs_dn3);
        let eq8_e852_d_n4: f64 = (var_igcs_dn4 + var_igs_dn4);
        let eq8_e852_d_n5: f64 = (var_igcs_dn5 + var_igs_dn5);
        let eq8_e852_d_n6: f64 = (var_igcs_dn6 + var_igs_dn6);
        let eq8_e852_d_n7: f64 = (var_igcs_dn7 + var_igs_dn7);
        let eq8_e852_d_n8: f64 = (var_igcs_dn8 + var_igs_dn8);
        let eq8_e852_d_b0: f64 = (var_igcs_db0 + var_igs_db0);
        let eq8_e852_d_b1: f64 = (var_igcs_db1 + var_igs_db1);
        let eq8_e852_d_b2: f64 = (var_igcs_db2 + var_igs_db2);
        let eq8_e852_d_b3: f64 = (var_igcs_db3 + var_igs_db3);
        let eq8_e852_d_b4: f64 = (var_igcs_db4 + var_igs_db4);
        let eq8_e853: f64 = (var_devsign * eq8_e852);
        let eq8_e853_d_n0: f64 = (var_devsign * eq8_e852_d_n0);
        let eq8_e853_d_n1: f64 = (var_devsign * eq8_e852_d_n1);
        let eq8_e853_d_n2: f64 = (var_devsign * eq8_e852_d_n2);
        let eq8_e853_d_n3: f64 = (var_devsign * eq8_e852_d_n3);
        let eq8_e853_d_n4: f64 = (var_devsign * eq8_e852_d_n4);
        let eq8_e853_d_n5: f64 = (var_devsign * eq8_e852_d_n5);
        let eq8_e853_d_n6: f64 = (var_devsign * eq8_e852_d_n6);
        let eq8_e853_d_n7: f64 = (var_devsign * eq8_e852_d_n7);
        let eq8_e853_d_n8: f64 = (var_devsign * eq8_e852_d_n8);
        let eq8_e853_d_b0: f64 = (var_devsign * eq8_e852_d_b0);
        let eq8_e853_d_b1: f64 = (var_devsign * eq8_e852_d_b1);
        let eq8_e853_d_b2: f64 = (var_devsign * eq8_e852_d_b2);
        let eq8_e853_d_b3: f64 = (var_devsign * eq8_e852_d_b3);
        let eq8_e853_d_b4: f64 = (var_devsign * eq8_e852_d_b4);
        (eq8_e853, eq8_e853_d_n0, eq8_e853_d_n1, eq8_e853_d_n2, eq8_e853_d_n3, eq8_e853_d_n4, eq8_e853_d_n5, eq8_e853_d_n6, eq8_e853_d_n7, eq8_e853_d_n8, eq8_e853_d_b0, eq8_e853_d_b1, eq8_e853_d_b2, eq8_e853_d_b3, eq8_e853_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e855;
        let eq8_node_derivatives: [f64; 9] = [eq8_e855_d_n0, eq8_e855_d_n1, eq8_e855_d_n2, eq8_e855_d_n3, eq8_e855_d_n4, eq8_e855_d_n5, eq8_e855_d_n6, eq8_e855_d_n7, eq8_e855_d_n8];
        let eq8_branch_derivatives: [f64; 5] = [eq8_e855_d_b0, eq8_e855_d_b1, eq8_e855_d_b2, eq8_e855_d_b3, eq8_e855_d_b4];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e864, eq9_e864_d_n0, eq9_e864_d_n1, eq9_e864_d_n2, eq9_e864_d_n3, eq9_e864_d_n4, eq9_e864_d_n5, eq9_e864_d_n6, eq9_e864_d_n7, eq9_e864_d_n8, eq9_e864_d_b0, eq9_e864_d_b1, eq9_e864_d_b2, eq9_e864_d_b3, eq9_e864_d_b4,) = {
    if (var_guard138 == 0.0) {
        let eq9_e861: f64 = (var_igcd + var_igd);
        let eq9_e861_d_n0: f64 = (var_igcd_dn0 + var_igd_dn0);
        let eq9_e861_d_n1: f64 = (var_igcd_dn1 + var_igd_dn1);
        let eq9_e861_d_n2: f64 = (var_igcd_dn2 + var_igd_dn2);
        let eq9_e861_d_n3: f64 = (var_igcd_dn3 + var_igd_dn3);
        let eq9_e861_d_n4: f64 = (var_igcd_dn4 + var_igd_dn4);
        let eq9_e861_d_n5: f64 = (var_igcd_dn5 + var_igd_dn5);
        let eq9_e861_d_n6: f64 = (var_igcd_dn6 + var_igd_dn6);
        let eq9_e861_d_n7: f64 = (var_igcd_dn7 + var_igd_dn7);
        let eq9_e861_d_n8: f64 = (var_igcd_dn8 + var_igd_dn8);
        let eq9_e861_d_b0: f64 = (var_igcd_db0 + var_igd_db0);
        let eq9_e861_d_b1: f64 = (var_igcd_db1 + var_igd_db1);
        let eq9_e861_d_b2: f64 = (var_igcd_db2 + var_igd_db2);
        let eq9_e861_d_b3: f64 = (var_igcd_db3 + var_igd_db3);
        let eq9_e861_d_b4: f64 = (var_igcd_db4 + var_igd_db4);
        let eq9_e862: f64 = (var_devsign * eq9_e861);
        let eq9_e862_d_n0: f64 = (var_devsign * eq9_e861_d_n0);
        let eq9_e862_d_n1: f64 = (var_devsign * eq9_e861_d_n1);
        let eq9_e862_d_n2: f64 = (var_devsign * eq9_e861_d_n2);
        let eq9_e862_d_n3: f64 = (var_devsign * eq9_e861_d_n3);
        let eq9_e862_d_n4: f64 = (var_devsign * eq9_e861_d_n4);
        let eq9_e862_d_n5: f64 = (var_devsign * eq9_e861_d_n5);
        let eq9_e862_d_n6: f64 = (var_devsign * eq9_e861_d_n6);
        let eq9_e862_d_n7: f64 = (var_devsign * eq9_e861_d_n7);
        let eq9_e862_d_n8: f64 = (var_devsign * eq9_e861_d_n8);
        let eq9_e862_d_b0: f64 = (var_devsign * eq9_e861_d_b0);
        let eq9_e862_d_b1: f64 = (var_devsign * eq9_e861_d_b1);
        let eq9_e862_d_b2: f64 = (var_devsign * eq9_e861_d_b2);
        let eq9_e862_d_b3: f64 = (var_devsign * eq9_e861_d_b3);
        let eq9_e862_d_b4: f64 = (var_devsign * eq9_e861_d_b4);
        (eq9_e862, eq9_e862_d_n0, eq9_e862_d_n1, eq9_e862_d_n2, eq9_e862_d_n3, eq9_e862_d_n4, eq9_e862_d_n5, eq9_e862_d_n6, eq9_e862_d_n7, eq9_e862_d_n8, eq9_e862_d_b0, eq9_e862_d_b1, eq9_e862_d_b2, eq9_e862_d_b3, eq9_e862_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e864;
        let eq9_node_derivatives: [f64; 9] = [eq9_e864_d_n0, eq9_e864_d_n1, eq9_e864_d_n2, eq9_e864_d_n3, eq9_e864_d_n4, eq9_e864_d_n5, eq9_e864_d_n6, eq9_e864_d_n7, eq9_e864_d_n8];
        let eq9_branch_derivatives: [f64; 5] = [eq9_e864_d_b0, eq9_e864_d_b1, eq9_e864_d_b2, eq9_e864_d_b3, eq9_e864_d_b4];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e867: f64 = (var_devsign * var_igbs);
        let eq10_e867_d_n0: f64 = (var_devsign * var_igbs_dn0);
        let eq10_e867_d_n1: f64 = (var_devsign * var_igbs_dn1);
        let eq10_e867_d_n2: f64 = (var_devsign * var_igbs_dn2);
        let eq10_e867_d_n3: f64 = (var_devsign * var_igbs_dn3);
        let eq10_e867_d_n4: f64 = (var_devsign * var_igbs_dn4);
        let eq10_e867_d_n5: f64 = (var_devsign * var_igbs_dn5);
        let eq10_e867_d_n6: f64 = (var_devsign * var_igbs_dn6);
        let eq10_e867_d_n7: f64 = (var_devsign * var_igbs_dn7);
        let eq10_e867_d_n8: f64 = (var_devsign * var_igbs_dn8);
        let eq10_e867_d_b0: f64 = (var_devsign * var_igbs_db0);
        let eq10_e867_d_b1: f64 = (var_devsign * var_igbs_db1);
        let eq10_e867_d_b2: f64 = (var_devsign * var_igbs_db2);
        let eq10_e867_d_b3: f64 = (var_devsign * var_igbs_db3);
        let eq10_e867_d_b4: f64 = (var_devsign * var_igbs_db4);
        let eq10_value: f64 = eq10_e867;
        let eq10_node_derivatives: [f64; 9] = [eq10_e867_d_n0, eq10_e867_d_n1, eq10_e867_d_n2, eq10_e867_d_n3, eq10_e867_d_n4, eq10_e867_d_n5, eq10_e867_d_n6, eq10_e867_d_n7, eq10_e867_d_n8];
        let eq10_branch_derivatives: [f64; 5] = [eq10_e867_d_b0, eq10_e867_d_b1, eq10_e867_d_b2, eq10_e867_d_b3, eq10_e867_d_b4];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e870: f64 = (var_devsign * var_igbd);
        let eq11_e870_d_n0: f64 = (var_devsign * var_igbd_dn0);
        let eq11_e870_d_n1: f64 = (var_devsign * var_igbd_dn1);
        let eq11_e870_d_n2: f64 = (var_devsign * var_igbd_dn2);
        let eq11_e870_d_n3: f64 = (var_devsign * var_igbd_dn3);
        let eq11_e870_d_n4: f64 = (var_devsign * var_igbd_dn4);
        let eq11_e870_d_n5: f64 = (var_devsign * var_igbd_dn5);
        let eq11_e870_d_n6: f64 = (var_devsign * var_igbd_dn6);
        let eq11_e870_d_n7: f64 = (var_devsign * var_igbd_dn7);
        let eq11_e870_d_n8: f64 = (var_devsign * var_igbd_dn8);
        let eq11_e870_d_b0: f64 = (var_devsign * var_igbd_db0);
        let eq11_e870_d_b1: f64 = (var_devsign * var_igbd_db1);
        let eq11_e870_d_b2: f64 = (var_devsign * var_igbd_db2);
        let eq11_e870_d_b3: f64 = (var_devsign * var_igbd_db3);
        let eq11_e870_d_b4: f64 = (var_devsign * var_igbd_db4);
        let eq11_value: f64 = eq11_e870;
        let eq11_node_derivatives: [f64; 9] = [eq11_e870_d_n0, eq11_e870_d_n1, eq11_e870_d_n2, eq11_e870_d_n3, eq11_e870_d_n4, eq11_e870_d_n5, eq11_e870_d_n6, eq11_e870_d_n7, eq11_e870_d_n8];
        let eq11_branch_derivatives: [f64; 5] = [eq11_e870_d_b0, eq11_e870_d_b1, eq11_e870_d_b2, eq11_e870_d_b3, eq11_e870_d_b4];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e873: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, var_qdi);
        let eq12_e874: f64 = (var_devsign * eq12_e873);
        let eq12_e874_d_n0: f64 = (var_devsign * (var_qdi_dn0 * ddt_scale));
        let eq12_e874_d_n1: f64 = (var_devsign * (var_qdi_dn1 * ddt_scale));
        let eq12_e874_d_n2: f64 = (var_devsign * (var_qdi_dn2 * ddt_scale));
        let eq12_e874_d_n3: f64 = (var_devsign * (var_qdi_dn3 * ddt_scale));
        let eq12_e874_d_n4: f64 = (var_devsign * (var_qdi_dn4 * ddt_scale));
        let eq12_e874_d_n5: f64 = (var_devsign * (var_qdi_dn5 * ddt_scale));
        let eq12_e874_d_n6: f64 = (var_devsign * (var_qdi_dn6 * ddt_scale));
        let eq12_e874_d_n7: f64 = (var_devsign * (var_qdi_dn7 * ddt_scale));
        let eq12_e874_d_n8: f64 = (var_devsign * (var_qdi_dn8 * ddt_scale));
        let eq12_e874_d_b0: f64 = (var_devsign * (var_qdi_db0 * ddt_scale));
        let eq12_e874_d_b1: f64 = (var_devsign * (var_qdi_db1 * ddt_scale));
        let eq12_e874_d_b2: f64 = (var_devsign * (var_qdi_db2 * ddt_scale));
        let eq12_e874_d_b3: f64 = (var_devsign * (var_qdi_db3 * ddt_scale));
        let eq12_e874_d_b4: f64 = (var_devsign * (var_qdi_db4 * ddt_scale));
        let eq12_value: f64 = eq12_e874;
        let eq12_node_derivatives: [f64; 9] = [eq12_e874_d_n0, eq12_e874_d_n1, eq12_e874_d_n2, eq12_e874_d_n3, eq12_e874_d_n4, eq12_e874_d_n5, eq12_e874_d_n6, eq12_e874_d_n7, eq12_e874_d_n8];
        let eq12_branch_derivatives: [f64; 5] = [eq12_e874_d_b0, eq12_e874_d_b1, eq12_e874_d_b2, eq12_e874_d_b3, eq12_e874_d_b4];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        var_cth: f64,
        var_devsign: f64,
        var_gcrg: f64,
        var_gcrg_db0: f64,
        var_gcrg_db1: f64,
        var_gcrg_db2: f64,
        var_gcrg_db3: f64,
        var_gcrg_db4: f64,
        var_gcrg_dn0: f64,
        var_gcrg_dn1: f64,
        var_gcrg_dn2: f64,
        var_gcrg_dn3: f64,
        var_gcrg_dn4: f64,
        var_gcrg_dn5: f64,
        var_gcrg_dn6: f64,
        var_gcrg_dn7: f64,
        var_gcrg_dn8: f64,
        var_gdpr: f64,
        var_gdpr_db0: f64,
        var_gdpr_db1: f64,
        var_gdpr_db2: f64,
        var_gdpr_db3: f64,
        var_gdpr_db4: f64,
        var_gdpr_dn0: f64,
        var_gdpr_dn1: f64,
        var_gdpr_dn2: f64,
        var_gdpr_dn3: f64,
        var_gdpr_dn4: f64,
        var_gdpr_dn5: f64,
        var_gdpr_dn6: f64,
        var_gdpr_dn7: f64,
        var_gdpr_dn8: f64,
        var_gspr: f64,
        var_gspr_db0: f64,
        var_gspr_db1: f64,
        var_gspr_db2: f64,
        var_gspr_db3: f64,
        var_gspr_db4: f64,
        var_gspr_dn0: f64,
        var_gspr_dn1: f64,
        var_gspr_dn2: f64,
        var_gspr_dn3: f64,
        var_gspr_dn4: f64,
        var_gspr_dn5: f64,
        var_gspr_dn6: f64,
        var_gspr_dn7: f64,
        var_gspr_dn8: f64,
        var_gth: f64,
        var_guard139: f64,
        var_guard140: f64,
        var_guard147: f64,
        var_guard148: f64,
        var_ids: f64,
        var_ids_db0: f64,
        var_ids_db1: f64,
        var_ids_db2: f64,
        var_ids_db3: f64,
        var_ids_db4: f64,
        var_ids_dn0: f64,
        var_ids_dn1: f64,
        var_ids_dn2: f64,
        var_ids_dn3: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_qbgi: f64,
        var_qbgi_db0: f64,
        var_qbgi_db1: f64,
        var_qbgi_db2: f64,
        var_qbgi_db3: f64,
        var_qbgi_db4: f64,
        var_qbgi_dn0: f64,
        var_qbgi_dn1: f64,
        var_qbgi_dn2: f64,
        var_qbgi_dn3: f64,
        var_qbgi_dn4: f64,
        var_qbgi_dn5: f64,
        var_qbgi_dn6: f64,
        var_qbgi_dn7: f64,
        var_qbgi_dn8: f64,
        var_qdbg: f64,
        var_qdbg_db0: f64,
        var_qdbg_db1: f64,
        var_qdbg_db2: f64,
        var_qdbg_db3: f64,
        var_qdbg_db4: f64,
        var_qdbg_dn0: f64,
        var_qdbg_dn1: f64,
        var_qdbg_dn2: f64,
        var_qdbg_dn3: f64,
        var_qdbg_dn4: f64,
        var_qdbg_dn5: f64,
        var_qdbg_dn6: f64,
        var_qdbg_dn7: f64,
        var_qdbg_dn8: f64,
        var_qfgd_parasitic: f64,
        var_qfgd_parasitic_db0: f64,
        var_qfgd_parasitic_db1: f64,
        var_qfgd_parasitic_db2: f64,
        var_qfgd_parasitic_db3: f64,
        var_qfgd_parasitic_db4: f64,
        var_qfgd_parasitic_dn0: f64,
        var_qfgd_parasitic_dn1: f64,
        var_qfgd_parasitic_dn2: f64,
        var_qfgd_parasitic_dn3: f64,
        var_qfgd_parasitic_dn4: f64,
        var_qfgd_parasitic_dn5: f64,
        var_qfgd_parasitic_dn6: f64,
        var_qfgd_parasitic_dn7: f64,
        var_qfgd_parasitic_dn8: f64,
        var_qfgi: f64,
        var_qfgi_db0: f64,
        var_qfgi_db1: f64,
        var_qfgi_db2: f64,
        var_qfgi_db3: f64,
        var_qfgi_db4: f64,
        var_qfgi_dn0: f64,
        var_qfgi_dn1: f64,
        var_qfgi_dn2: f64,
        var_qfgi_dn3: f64,
        var_qfgi_dn4: f64,
        var_qfgi_dn5: f64,
        var_qfgi_dn6: f64,
        var_qfgi_dn7: f64,
        var_qfgi_dn8: f64,
        var_qfgs_parasitic: f64,
        var_qfgs_parasitic_db0: f64,
        var_qfgs_parasitic_db1: f64,
        var_qfgs_parasitic_db2: f64,
        var_qfgs_parasitic_db3: f64,
        var_qfgs_parasitic_db4: f64,
        var_qfgs_parasitic_dn0: f64,
        var_qfgs_parasitic_dn1: f64,
        var_qfgs_parasitic_dn2: f64,
        var_qfgs_parasitic_dn3: f64,
        var_qfgs_parasitic_dn4: f64,
        var_qfgs_parasitic_dn5: f64,
        var_qfgs_parasitic_dn6: f64,
        var_qfgs_parasitic_dn7: f64,
        var_qfgs_parasitic_dn8: f64,
        var_qsbg: f64,
        var_qsbg_db0: f64,
        var_qsbg_db1: f64,
        var_qsbg_db2: f64,
        var_qsbg_db3: f64,
        var_qsbg_db4: f64,
        var_qsbg_dn0: f64,
        var_qsbg_dn1: f64,
        var_qsbg_dn2: f64,
        var_qsbg_dn3: f64,
        var_qsbg_dn4: f64,
        var_qsbg_dn5: f64,
        var_qsbg_dn6: f64,
        var_qsbg_dn7: f64,
        var_qsbg_dn8: f64,
        var_rdrain: f64,
        var_rdrain_db0: f64,
        var_rdrain_db1: f64,
        var_rdrain_db2: f64,
        var_rdrain_db3: f64,
        var_rdrain_db4: f64,
        var_rdrain_dn0: f64,
        var_rdrain_dn1: f64,
        var_rdrain_dn2: f64,
        var_rdrain_dn3: f64,
        var_rdrain_dn4: f64,
        var_rdrain_dn5: f64,
        var_rdrain_dn6: f64,
        var_rdrain_dn7: f64,
        var_rdrain_dn8: f64,
        var_rsource: f64,
        var_rsource_db0: f64,
        var_rsource_db1: f64,
        var_rsource_db2: f64,
        var_rsource_db3: f64,
        var_rsource_db4: f64,
        var_rsource_dn0: f64,
        var_rsource_dn1: f64,
        var_rsource_dn2: f64,
        var_rsource_dn3: f64,
        var_rsource_dn4: f64,
        var_rsource_dn5: f64,
        var_rsource_dn6: f64,
        var_rsource_dn7: f64,
        var_rsource_dn8: f64,
        var_sigvds: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let eq13_e876: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, var_qfgi);
        let eq13_value: f64 = eq13_e876;
        let eq13_node_derivatives: [f64; 9] = [(var_qfgi_dn0 * ddt_scale), (var_qfgi_dn1 * ddt_scale), (var_qfgi_dn2 * ddt_scale), (var_qfgi_dn3 * ddt_scale), (var_qfgi_dn4 * ddt_scale), (var_qfgi_dn5 * ddt_scale), (var_qfgi_dn6 * ddt_scale), (var_qfgi_dn7 * ddt_scale), (var_qfgi_dn8 * ddt_scale)];
        let eq13_branch_derivatives: [f64; 5] = [(var_qfgi_db0 * ddt_scale), (var_qfgi_db1 * ddt_scale), (var_qfgi_db2 * ddt_scale), (var_qfgi_db3 * ddt_scale), (var_qfgi_db4 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e879: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_qbgi);
        let eq14_e880: f64 = (var_devsign * eq14_e879);
        let eq14_e880_d_n0: f64 = (var_devsign * (var_qbgi_dn0 * ddt_scale));
        let eq14_e880_d_n1: f64 = (var_devsign * (var_qbgi_dn1 * ddt_scale));
        let eq14_e880_d_n2: f64 = (var_devsign * (var_qbgi_dn2 * ddt_scale));
        let eq14_e880_d_n3: f64 = (var_devsign * (var_qbgi_dn3 * ddt_scale));
        let eq14_e880_d_n4: f64 = (var_devsign * (var_qbgi_dn4 * ddt_scale));
        let eq14_e880_d_n5: f64 = (var_devsign * (var_qbgi_dn5 * ddt_scale));
        let eq14_e880_d_n6: f64 = (var_devsign * (var_qbgi_dn6 * ddt_scale));
        let eq14_e880_d_n7: f64 = (var_devsign * (var_qbgi_dn7 * ddt_scale));
        let eq14_e880_d_n8: f64 = (var_devsign * (var_qbgi_dn8 * ddt_scale));
        let eq14_e880_d_b0: f64 = (var_devsign * (var_qbgi_db0 * ddt_scale));
        let eq14_e880_d_b1: f64 = (var_devsign * (var_qbgi_db1 * ddt_scale));
        let eq14_e880_d_b2: f64 = (var_devsign * (var_qbgi_db2 * ddt_scale));
        let eq14_e880_d_b3: f64 = (var_devsign * (var_qbgi_db3 * ddt_scale));
        let eq14_e880_d_b4: f64 = (var_devsign * (var_qbgi_db4 * ddt_scale));
        let eq14_value: f64 = eq14_e880;
        let eq14_node_derivatives: [f64; 9] = [eq14_e880_d_n0, eq14_e880_d_n1, eq14_e880_d_n2, eq14_e880_d_n3, eq14_e880_d_n4, eq14_e880_d_n5, eq14_e880_d_n6, eq14_e880_d_n7, eq14_e880_d_n8];
        let eq14_branch_derivatives: [f64; 5] = [eq14_e880_d_b0, eq14_e880_d_b1, eq14_e880_d_b2, eq14_e880_d_b3, eq14_e880_d_b4];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(6),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_e882: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, var_qfgs_parasitic);
        let eq15_value: f64 = eq15_e882;
        let eq15_node_derivatives: [f64; 9] = [(var_qfgs_parasitic_dn0 * ddt_scale), (var_qfgs_parasitic_dn1 * ddt_scale), (var_qfgs_parasitic_dn2 * ddt_scale), (var_qfgs_parasitic_dn3 * ddt_scale), (var_qfgs_parasitic_dn4 * ddt_scale), (var_qfgs_parasitic_dn5 * ddt_scale), (var_qfgs_parasitic_dn6 * ddt_scale), (var_qfgs_parasitic_dn7 * ddt_scale), (var_qfgs_parasitic_dn8 * ddt_scale)];
        let eq15_branch_derivatives: [f64; 5] = [(var_qfgs_parasitic_db0 * ddt_scale), (var_qfgs_parasitic_db1 * ddt_scale), (var_qfgs_parasitic_db2 * ddt_scale), (var_qfgs_parasitic_db3 * ddt_scale), (var_qfgs_parasitic_db4 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq16_e884: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, var_qfgd_parasitic);
        let eq16_value: f64 = eq16_e884;
        let eq16_node_derivatives: [f64; 9] = [(var_qfgd_parasitic_dn0 * ddt_scale), (var_qfgd_parasitic_dn1 * ddt_scale), (var_qfgd_parasitic_dn2 * ddt_scale), (var_qfgd_parasitic_dn3 * ddt_scale), (var_qfgd_parasitic_dn4 * ddt_scale), (var_qfgd_parasitic_dn5 * ddt_scale), (var_qfgd_parasitic_dn6 * ddt_scale), (var_qfgd_parasitic_dn7 * ddt_scale), (var_qfgd_parasitic_dn8 * ddt_scale)];
        let eq16_branch_derivatives: [f64; 5] = [(var_qfgd_parasitic_db0 * ddt_scale), (var_qfgd_parasitic_db1 * ddt_scale), (var_qfgd_parasitic_db2 * ddt_scale), (var_qfgd_parasitic_db3 * ddt_scale), (var_qfgd_parasitic_db4 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let eq17_e887: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, var_qsbg);
        let eq17_e888: f64 = (var_devsign * eq17_e887);
        let eq17_e888_d_n0: f64 = (var_devsign * (var_qsbg_dn0 * ddt_scale));
        let eq17_e888_d_n1: f64 = (var_devsign * (var_qsbg_dn1 * ddt_scale));
        let eq17_e888_d_n2: f64 = (var_devsign * (var_qsbg_dn2 * ddt_scale));
        let eq17_e888_d_n3: f64 = (var_devsign * (var_qsbg_dn3 * ddt_scale));
        let eq17_e888_d_n4: f64 = (var_devsign * (var_qsbg_dn4 * ddt_scale));
        let eq17_e888_d_n5: f64 = (var_devsign * (var_qsbg_dn5 * ddt_scale));
        let eq17_e888_d_n6: f64 = (var_devsign * (var_qsbg_dn6 * ddt_scale));
        let eq17_e888_d_n7: f64 = (var_devsign * (var_qsbg_dn7 * ddt_scale));
        let eq17_e888_d_n8: f64 = (var_devsign * (var_qsbg_dn8 * ddt_scale));
        let eq17_e888_d_b0: f64 = (var_devsign * (var_qsbg_db0 * ddt_scale));
        let eq17_e888_d_b1: f64 = (var_devsign * (var_qsbg_db1 * ddt_scale));
        let eq17_e888_d_b2: f64 = (var_devsign * (var_qsbg_db2 * ddt_scale));
        let eq17_e888_d_b3: f64 = (var_devsign * (var_qsbg_db3 * ddt_scale));
        let eq17_e888_d_b4: f64 = (var_devsign * (var_qsbg_db4 * ddt_scale));
        let eq17_value: f64 = eq17_e888;
        let eq17_node_derivatives: [f64; 9] = [eq17_e888_d_n0, eq17_e888_d_n1, eq17_e888_d_n2, eq17_e888_d_n3, eq17_e888_d_n4, eq17_e888_d_n5, eq17_e888_d_n6, eq17_e888_d_n7, eq17_e888_d_n8];
        let eq17_branch_derivatives: [f64; 5] = [eq17_e888_d_b0, eq17_e888_d_b1, eq17_e888_d_b2, eq17_e888_d_b3, eq17_e888_d_b4];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e891: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, var_qdbg);
        let eq18_e892: f64 = (var_devsign * eq18_e891);
        let eq18_e892_d_n0: f64 = (var_devsign * (var_qdbg_dn0 * ddt_scale));
        let eq18_e892_d_n1: f64 = (var_devsign * (var_qdbg_dn1 * ddt_scale));
        let eq18_e892_d_n2: f64 = (var_devsign * (var_qdbg_dn2 * ddt_scale));
        let eq18_e892_d_n3: f64 = (var_devsign * (var_qdbg_dn3 * ddt_scale));
        let eq18_e892_d_n4: f64 = (var_devsign * (var_qdbg_dn4 * ddt_scale));
        let eq18_e892_d_n5: f64 = (var_devsign * (var_qdbg_dn5 * ddt_scale));
        let eq18_e892_d_n6: f64 = (var_devsign * (var_qdbg_dn6 * ddt_scale));
        let eq18_e892_d_n7: f64 = (var_devsign * (var_qdbg_dn7 * ddt_scale));
        let eq18_e892_d_n8: f64 = (var_devsign * (var_qdbg_dn8 * ddt_scale));
        let eq18_e892_d_b0: f64 = (var_devsign * (var_qdbg_db0 * ddt_scale));
        let eq18_e892_d_b1: f64 = (var_devsign * (var_qdbg_db1 * ddt_scale));
        let eq18_e892_d_b2: f64 = (var_devsign * (var_qdbg_db2 * ddt_scale));
        let eq18_e892_d_b3: f64 = (var_devsign * (var_qdbg_db3 * ddt_scale));
        let eq18_e892_d_b4: f64 = (var_devsign * (var_qdbg_db4 * ddt_scale));
        let eq18_value: f64 = eq18_e892;
        let eq18_node_derivatives: [f64; 9] = [eq18_e892_d_n0, eq18_e892_d_n1, eq18_e892_d_n2, eq18_e892_d_n3, eq18_e892_d_n4, eq18_e892_d_n5, eq18_e892_d_n6, eq18_e892_d_n7, eq18_e892_d_n8];
        let eq18_branch_derivatives: [f64; 5] = [eq18_e892_d_b0, eq18_e892_d_b1, eq18_e892_d_b2, eq18_e892_d_b3, eq18_e892_d_b4];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq21_e907, eq21_e907_d_n0, eq21_e907_d_n1, eq21_e907_d_n2, eq21_e907_d_n3, eq21_e907_d_n4, eq21_e907_d_n5, eq21_e907_d_n6, eq21_e907_d_n7, eq21_e907_d_n8, eq21_e907_d_b0, eq21_e907_d_b1, eq21_e907_d_b2, eq21_e907_d_b3, eq21_e907_d_b4,) = {
    if (var_guard139 == 0.0) {
        let eq21_e905: f64 = ((nv0 - nv5) * var_gdpr);
        let eq21_e905_d_n0: f64 = (var_gdpr + ((nv0 - nv5) * var_gdpr_dn0));
        let eq21_e905_d_n1: f64 = ((nv0 - nv5) * var_gdpr_dn1);
        let eq21_e905_d_n2: f64 = ((nv0 - nv5) * var_gdpr_dn2);
        let eq21_e905_d_n3: f64 = ((nv0 - nv5) * var_gdpr_dn3);
        let eq21_e905_d_n4: f64 = ((nv0 - nv5) * var_gdpr_dn4);
        let eq21_e905_d_n5: f64 = ((-var_gdpr) + ((nv0 - nv5) * var_gdpr_dn5));
        let eq21_e905_d_n6: f64 = ((nv0 - nv5) * var_gdpr_dn6);
        let eq21_e905_d_n7: f64 = ((nv0 - nv5) * var_gdpr_dn7);
        let eq21_e905_d_n8: f64 = ((nv0 - nv5) * var_gdpr_dn8);
        let eq21_e905_d_b0: f64 = ((nv0 - nv5) * var_gdpr_db0);
        let eq21_e905_d_b1: f64 = ((nv0 - nv5) * var_gdpr_db1);
        let eq21_e905_d_b2: f64 = ((nv0 - nv5) * var_gdpr_db2);
        let eq21_e905_d_b3: f64 = ((nv0 - nv5) * var_gdpr_db3);
        let eq21_e905_d_b4: f64 = ((nv0 - nv5) * var_gdpr_db4);
        (eq21_e905, eq21_e905_d_n0, eq21_e905_d_n1, eq21_e905_d_n2, eq21_e905_d_n3, eq21_e905_d_n4, eq21_e905_d_n5, eq21_e905_d_n6, eq21_e905_d_n7, eq21_e905_d_n8, eq21_e905_d_b0, eq21_e905_d_b1, eq21_e905_d_b2, eq21_e905_d_b3, eq21_e905_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e907;
        let eq21_node_derivatives: [f64; 9] = [eq21_e907_d_n0, eq21_e907_d_n1, eq21_e907_d_n2, eq21_e907_d_n3, eq21_e907_d_n4, eq21_e907_d_n5, eq21_e907_d_n6, eq21_e907_d_n7, eq21_e907_d_n8];
        let eq21_branch_derivatives: [f64; 5] = [eq21_e907_d_b0, eq21_e907_d_b1, eq21_e907_d_b2, eq21_e907_d_b3, eq21_e907_d_b4];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(5),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e914, eq22_e914_d_n0, eq22_e914_d_n1, eq22_e914_d_n2, eq22_e914_d_n3, eq22_e914_d_n4, eq22_e914_d_n5, eq22_e914_d_n6, eq22_e914_d_n7, eq22_e914_d_n8, eq22_e914_d_b0, eq22_e914_d_b1, eq22_e914_d_b2, eq22_e914_d_b3, eq22_e914_d_b4,) = {
    if (var_guard139 == 0.0) {
        let eq22_e912: f64 = ((nv2 - nv6) * var_gspr);
        let eq22_e912_d_n0: f64 = ((nv2 - nv6) * var_gspr_dn0);
        let eq22_e912_d_n1: f64 = ((nv2 - nv6) * var_gspr_dn1);
        let eq22_e912_d_n2: f64 = (var_gspr + ((nv2 - nv6) * var_gspr_dn2));
        let eq22_e912_d_n3: f64 = ((nv2 - nv6) * var_gspr_dn3);
        let eq22_e912_d_n4: f64 = ((nv2 - nv6) * var_gspr_dn4);
        let eq22_e912_d_n5: f64 = ((nv2 - nv6) * var_gspr_dn5);
        let eq22_e912_d_n6: f64 = ((-var_gspr) + ((nv2 - nv6) * var_gspr_dn6));
        let eq22_e912_d_n7: f64 = ((nv2 - nv6) * var_gspr_dn7);
        let eq22_e912_d_n8: f64 = ((nv2 - nv6) * var_gspr_dn8);
        let eq22_e912_d_b0: f64 = ((nv2 - nv6) * var_gspr_db0);
        let eq22_e912_d_b1: f64 = ((nv2 - nv6) * var_gspr_db1);
        let eq22_e912_d_b2: f64 = ((nv2 - nv6) * var_gspr_db2);
        let eq22_e912_d_b3: f64 = ((nv2 - nv6) * var_gspr_db3);
        let eq22_e912_d_b4: f64 = ((nv2 - nv6) * var_gspr_db4);
        (eq22_e912, eq22_e912_d_n0, eq22_e912_d_n1, eq22_e912_d_n2, eq22_e912_d_n3, eq22_e912_d_n4, eq22_e912_d_n5, eq22_e912_d_n6, eq22_e912_d_n7, eq22_e912_d_n8, eq22_e912_d_b0, eq22_e912_d_b1, eq22_e912_d_b2, eq22_e912_d_b3, eq22_e912_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e914;
        let eq22_node_derivatives: [f64; 9] = [eq22_e914_d_n0, eq22_e914_d_n1, eq22_e914_d_n2, eq22_e914_d_n3, eq22_e914_d_n4, eq22_e914_d_n5, eq22_e914_d_n6, eq22_e914_d_n7, eq22_e914_d_n8];
        let eq22_branch_derivatives: [f64; 5] = [eq22_e914_d_b0, eq22_e914_d_b1, eq22_e914_d_b2, eq22_e914_d_b3, eq22_e914_d_b4];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(6),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let (eq25_e938, eq25_e938_d_n0, eq25_e938_d_n1, eq25_e938_d_n2, eq25_e938_d_n3, eq25_e938_d_n4, eq25_e938_d_n5, eq25_e938_d_n6, eq25_e938_d_n7, eq25_e938_d_n8, eq25_e938_d_b0, eq25_e938_d_b1, eq25_e938_d_b2, eq25_e938_d_b3, eq25_e938_d_b4,) = {
    if (var_guard140 != 0.0) {
        let eq25_e936: f64 = ((nv7 - nv8) * var_gcrg);
        let eq25_e936_d_n0: f64 = ((nv7 - nv8) * var_gcrg_dn0);
        let eq25_e936_d_n1: f64 = ((nv7 - nv8) * var_gcrg_dn1);
        let eq25_e936_d_n2: f64 = ((nv7 - nv8) * var_gcrg_dn2);
        let eq25_e936_d_n3: f64 = ((nv7 - nv8) * var_gcrg_dn3);
        let eq25_e936_d_n4: f64 = ((nv7 - nv8) * var_gcrg_dn4);
        let eq25_e936_d_n5: f64 = ((nv7 - nv8) * var_gcrg_dn5);
        let eq25_e936_d_n6: f64 = ((nv7 - nv8) * var_gcrg_dn6);
        let eq25_e936_d_n7: f64 = (var_gcrg + ((nv7 - nv8) * var_gcrg_dn7));
        let eq25_e936_d_n8: f64 = ((-var_gcrg) + ((nv7 - nv8) * var_gcrg_dn8));
        let eq25_e936_d_b0: f64 = ((nv7 - nv8) * var_gcrg_db0);
        let eq25_e936_d_b1: f64 = ((nv7 - nv8) * var_gcrg_db1);
        let eq25_e936_d_b2: f64 = ((nv7 - nv8) * var_gcrg_db2);
        let eq25_e936_d_b3: f64 = ((nv7 - nv8) * var_gcrg_db3);
        let eq25_e936_d_b4: f64 = ((nv7 - nv8) * var_gcrg_db4);
        (eq25_e936, eq25_e936_d_n0, eq25_e936_d_n1, eq25_e936_d_n2, eq25_e936_d_n3, eq25_e936_d_n4, eq25_e936_d_n5, eq25_e936_d_n6, eq25_e936_d_n7, eq25_e936_d_n8, eq25_e936_d_b0, eq25_e936_d_b1, eq25_e936_d_b2, eq25_e936_d_b3, eq25_e936_d_b4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e938;
        let eq25_node_derivatives: [f64; 9] = [eq25_e938_d_n0, eq25_e938_d_n1, eq25_e938_d_n2, eq25_e938_d_n3, eq25_e938_d_n4, eq25_e938_d_n5, eq25_e938_d_n6, eq25_e938_d_n7, eq25_e938_d_n8];
        let eq25_branch_derivatives: [f64; 5] = [eq25_e938_d_b0, eq25_e938_d_b1, eq25_e938_d_b2, eq25_e938_d_b3, eq25_e938_d_b4];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq38_e1079, eq38_e1079_d_n0, eq38_e1079_d_n1, eq38_e1079_d_n2, eq38_e1079_d_n3, eq38_e1079_d_n4, eq38_e1079_d_n5, eq38_e1079_d_n6, eq38_e1079_d_n7, eq38_e1079_d_n8, eq38_e1079_d_b0, eq38_e1079_d_b1, eq38_e1079_d_b2, eq38_e1079_d_b3, eq38_e1079_d_b4,) = {
    if ((var_guard147 != 0.0) && (var_guard148 != 0.0)) {
        let eq38_e1060: f64 = (var_devsign * var_sigvds);
        let eq38_e1062: f64 = (eq38_e1060 * (nv5 - nv6));
        let eq38_e1064: f64 = (eq38_e1062 * var_ids);
        let eq38_e1064_d_n0: f64 = (eq38_e1062 * var_ids_dn0);
        let eq38_e1064_d_n1: f64 = (eq38_e1062 * var_ids_dn1);
        let eq38_e1064_d_n2: f64 = (eq38_e1062 * var_ids_dn2);
        let eq38_e1064_d_n3: f64 = (eq38_e1062 * var_ids_dn3);
        let eq38_e1064_d_n4: f64 = (eq38_e1062 * var_ids_dn4);
        let eq38_e1064_d_n5: f64 = ((eq38_e1060 * var_ids) + (eq38_e1062 * var_ids_dn5));
        let eq38_e1064_d_n6: f64 = (((-eq38_e1060) * var_ids) + (eq38_e1062 * var_ids_dn6));
        let eq38_e1064_d_n7: f64 = (eq38_e1062 * var_ids_dn7);
        let eq38_e1064_d_n8: f64 = (eq38_e1062 * var_ids_dn8);
        let eq38_e1064_d_b0: f64 = (eq38_e1062 * var_ids_db0);
        let eq38_e1064_d_b1: f64 = (eq38_e1062 * var_ids_db1);
        let eq38_e1064_d_b2: f64 = (eq38_e1062 * var_ids_db2);
        let eq38_e1064_d_b3: f64 = (eq38_e1062 * var_ids_db3);
        let eq38_e1064_d_b4: f64 = (eq38_e1062 * var_ids_db4);
        let eq38_e1067: f64 = ((nv0 - nv5) * (nv0 - nv5));
        let eq38_e1067_d_n0: f64 = ((nv0 - nv5) + (nv0 - nv5));
        let eq38_e1067_d_n5: f64 = ((-(nv0 - nv5)) + (-(nv0 - nv5)));
        let eq38_e1069: f64 = (eq38_e1067 / var_rdrain);
        let eq38_e1069_d_n0: f64 = (((eq38_e1067_d_n0 * var_rdrain) - (eq38_e1067 * var_rdrain_dn0)) / (var_rdrain * var_rdrain));
        let eq38_e1069_d_n1: f64 = (-((eq38_e1067 * var_rdrain_dn1) / (var_rdrain * var_rdrain)));
        let eq38_e1069_d_n2: f64 = (-((eq38_e1067 * var_rdrain_dn2) / (var_rdrain * var_rdrain)));
        let eq38_e1069_d_n3: f64 = (-((eq38_e1067 * var_rdrain_dn3) / (var_rdrain * var_rdrain)));
        let eq38_e1069_d_n4: f64 = (-((eq38_e1067 * var_rdrain_dn4) / (var_rdrain * var_rdrain)));
        let eq38_e1069_d_n5: f64 = (((eq38_e1067_d_n5 * var_rdrain) - (eq38_e1067 * var_rdrain_dn5)) / (var_rdrain * var_rdrain));
        let eq38_e1069_d_n6: f64 = (-((eq38_e1067 * var_rdrain_dn6) / (var_rdrain * var_rdrain)));
        let eq38_e1069_d_n7: f64 = (-((eq38_e1067 * var_rdrain_dn7) / (var_rdrain * var_rdrain)));
        let eq38_e1069_d_n8: f64 = (-((eq38_e1067 * var_rdrain_dn8) / (var_rdrain * var_rdrain)));
        let eq38_e1069_d_b0: f64 = (-((eq38_e1067 * var_rdrain_db0) / (var_rdrain * var_rdrain)));
        let eq38_e1069_d_b1: f64 = (-((eq38_e1067 * var_rdrain_db1) / (var_rdrain * var_rdrain)));
        let eq38_e1069_d_b2: f64 = (-((eq38_e1067 * var_rdrain_db2) / (var_rdrain * var_rdrain)));
        let eq38_e1069_d_b3: f64 = (-((eq38_e1067 * var_rdrain_db3) / (var_rdrain * var_rdrain)));
        let eq38_e1069_d_b4: f64 = (-((eq38_e1067 * var_rdrain_db4) / (var_rdrain * var_rdrain)));
        let eq38_e1070: f64 = (eq38_e1064 + eq38_e1069);
        let eq38_e1070_d_n0: f64 = (eq38_e1064_d_n0 + eq38_e1069_d_n0);
        let eq38_e1070_d_n1: f64 = (eq38_e1064_d_n1 + eq38_e1069_d_n1);
        let eq38_e1070_d_n2: f64 = (eq38_e1064_d_n2 + eq38_e1069_d_n2);
        let eq38_e1070_d_n3: f64 = (eq38_e1064_d_n3 + eq38_e1069_d_n3);
        let eq38_e1070_d_n4: f64 = (eq38_e1064_d_n4 + eq38_e1069_d_n4);
        let eq38_e1070_d_n5: f64 = (eq38_e1064_d_n5 + eq38_e1069_d_n5);
        let eq38_e1070_d_n6: f64 = (eq38_e1064_d_n6 + eq38_e1069_d_n6);
        let eq38_e1070_d_n7: f64 = (eq38_e1064_d_n7 + eq38_e1069_d_n7);
        let eq38_e1070_d_n8: f64 = (eq38_e1064_d_n8 + eq38_e1069_d_n8);
        let eq38_e1070_d_b0: f64 = (eq38_e1064_d_b0 + eq38_e1069_d_b0);
        let eq38_e1070_d_b1: f64 = (eq38_e1064_d_b1 + eq38_e1069_d_b1);
        let eq38_e1070_d_b2: f64 = (eq38_e1064_d_b2 + eq38_e1069_d_b2);
        let eq38_e1070_d_b3: f64 = (eq38_e1064_d_b3 + eq38_e1069_d_b3);
        let eq38_e1070_d_b4: f64 = (eq38_e1064_d_b4 + eq38_e1069_d_b4);
        let eq38_e1073: f64 = ((nv2 - nv6) * (nv2 - nv6));
        let eq38_e1073_d_n2: f64 = ((nv2 - nv6) + (nv2 - nv6));
        let eq38_e1073_d_n6: f64 = ((-(nv2 - nv6)) + (-(nv2 - nv6)));
        let eq38_e1075: f64 = (eq38_e1073 / var_rsource);
        let eq38_e1075_d_n0: f64 = (-((eq38_e1073 * var_rsource_dn0) / (var_rsource * var_rsource)));
        let eq38_e1075_d_n1: f64 = (-((eq38_e1073 * var_rsource_dn1) / (var_rsource * var_rsource)));
        let eq38_e1075_d_n2: f64 = (((eq38_e1073_d_n2 * var_rsource) - (eq38_e1073 * var_rsource_dn2)) / (var_rsource * var_rsource));
        let eq38_e1075_d_n3: f64 = (-((eq38_e1073 * var_rsource_dn3) / (var_rsource * var_rsource)));
        let eq38_e1075_d_n4: f64 = (-((eq38_e1073 * var_rsource_dn4) / (var_rsource * var_rsource)));
        let eq38_e1075_d_n5: f64 = (-((eq38_e1073 * var_rsource_dn5) / (var_rsource * var_rsource)));
        let eq38_e1075_d_n6: f64 = (((eq38_e1073_d_n6 * var_rsource) - (eq38_e1073 * var_rsource_dn6)) / (var_rsource * var_rsource));
        let eq38_e1075_d_n7: f64 = (-((eq38_e1073 * var_rsource_dn7) / (var_rsource * var_rsource)));
        let eq38_e1075_d_n8: f64 = (-((eq38_e1073 * var_rsource_dn8) / (var_rsource * var_rsource)));
        let eq38_e1075_d_b0: f64 = (-((eq38_e1073 * var_rsource_db0) / (var_rsource * var_rsource)));
        let eq38_e1075_d_b1: f64 = (-((eq38_e1073 * var_rsource_db1) / (var_rsource * var_rsource)));
        let eq38_e1075_d_b2: f64 = (-((eq38_e1073 * var_rsource_db2) / (var_rsource * var_rsource)));
        let eq38_e1075_d_b3: f64 = (-((eq38_e1073 * var_rsource_db3) / (var_rsource * var_rsource)));
        let eq38_e1075_d_b4: f64 = (-((eq38_e1073 * var_rsource_db4) / (var_rsource * var_rsource)));
        let eq38_e1076: f64 = (eq38_e1070 + eq38_e1075);
        let eq38_e1076_d_n0: f64 = (eq38_e1070_d_n0 + eq38_e1075_d_n0);
        let eq38_e1076_d_n1: f64 = (eq38_e1070_d_n1 + eq38_e1075_d_n1);
        let eq38_e1076_d_n2: f64 = (eq38_e1070_d_n2 + eq38_e1075_d_n2);
        let eq38_e1076_d_n3: f64 = (eq38_e1070_d_n3 + eq38_e1075_d_n3);
        let eq38_e1076_d_n4: f64 = (eq38_e1070_d_n4 + eq38_e1075_d_n4);
        let eq38_e1076_d_n5: f64 = (eq38_e1070_d_n5 + eq38_e1075_d_n5);
        let eq38_e1076_d_n6: f64 = (eq38_e1070_d_n6 + eq38_e1075_d_n6);
        let eq38_e1076_d_n7: f64 = (eq38_e1070_d_n7 + eq38_e1075_d_n7);
        let eq38_e1076_d_n8: f64 = (eq38_e1070_d_n8 + eq38_e1075_d_n8);
        let eq38_e1076_d_b0: f64 = (eq38_e1070_d_b0 + eq38_e1075_d_b0);
        let eq38_e1076_d_b1: f64 = (eq38_e1070_d_b1 + eq38_e1075_d_b1);
        let eq38_e1076_d_b2: f64 = (eq38_e1070_d_b2 + eq38_e1075_d_b2);
        let eq38_e1076_d_b3: f64 = (eq38_e1070_d_b3 + eq38_e1075_d_b3);
        let eq38_e1076_d_b4: f64 = (eq38_e1070_d_b4 + eq38_e1075_d_b4);
        let eq38_e1077: f64 = (-eq38_e1076);
        (eq38_e1077, (-eq38_e1076_d_n0), (-eq38_e1076_d_n1), (-eq38_e1076_d_n2), (-eq38_e1076_d_n3), (-eq38_e1076_d_n4), (-eq38_e1076_d_n5), (-eq38_e1076_d_n6), (-eq38_e1076_d_n7), (-eq38_e1076_d_n8), (-eq38_e1076_d_b0), (-eq38_e1076_d_b1), (-eq38_e1076_d_b2), (-eq38_e1076_d_b3), (-eq38_e1076_d_b4),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e1079;
        let eq38_node_derivatives: [f64; 9] = [eq38_e1079_d_n0, eq38_e1079_d_n1, eq38_e1079_d_n2, eq38_e1079_d_n3, eq38_e1079_d_n4, eq38_e1079_d_n5, eq38_e1079_d_n6, eq38_e1079_d_n7, eq38_e1079_d_n8];
        let eq38_branch_derivatives: [f64; 5] = [eq38_e1079_d_b0, eq38_e1079_d_b1, eq38_e1079_d_b2, eq38_e1079_d_b3, eq38_e1079_d_b4];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let (eq39_e1093, eq39_e1093_d_n0, eq39_e1093_d_n1, eq39_e1093_d_n2, eq39_e1093_d_n3, eq39_e1093_d_n4, eq39_e1093_d_n5, eq39_e1093_d_n6, eq39_e1093_d_n7, eq39_e1093_d_n8, eq39_e1093_d_b0, eq39_e1093_d_b1, eq39_e1093_d_b2, eq39_e1093_d_b3, eq39_e1093_d_b4,) = {
    if ((var_guard147 != 0.0) && (var_guard148 == 0.0)) {
        let eq39_e1086: f64 = (var_devsign * var_sigvds);
        let eq39_e1088: f64 = (eq39_e1086 * (nv5 - nv6));
        let eq39_e1090: f64 = (eq39_e1088 * var_ids);
        let eq39_e1090_d_n0: f64 = (eq39_e1088 * var_ids_dn0);
        let eq39_e1090_d_n1: f64 = (eq39_e1088 * var_ids_dn1);
        let eq39_e1090_d_n2: f64 = (eq39_e1088 * var_ids_dn2);
        let eq39_e1090_d_n3: f64 = (eq39_e1088 * var_ids_dn3);
        let eq39_e1090_d_n4: f64 = (eq39_e1088 * var_ids_dn4);
        let eq39_e1090_d_n5: f64 = ((eq39_e1086 * var_ids) + (eq39_e1088 * var_ids_dn5));
        let eq39_e1090_d_n6: f64 = (((-eq39_e1086) * var_ids) + (eq39_e1088 * var_ids_dn6));
        let eq39_e1090_d_n7: f64 = (eq39_e1088 * var_ids_dn7);
        let eq39_e1090_d_n8: f64 = (eq39_e1088 * var_ids_dn8);
        let eq39_e1090_d_b0: f64 = (eq39_e1088 * var_ids_db0);
        let eq39_e1090_d_b1: f64 = (eq39_e1088 * var_ids_db1);
        let eq39_e1090_d_b2: f64 = (eq39_e1088 * var_ids_db2);
        let eq39_e1090_d_b3: f64 = (eq39_e1088 * var_ids_db3);
        let eq39_e1090_d_b4: f64 = (eq39_e1088 * var_ids_db4);
        let eq39_e1091: f64 = (-eq39_e1090);
        (eq39_e1091, (-eq39_e1090_d_n0), (-eq39_e1090_d_n1), (-eq39_e1090_d_n2), (-eq39_e1090_d_n3), (-eq39_e1090_d_n4), (-eq39_e1090_d_n5), (-eq39_e1090_d_n6), (-eq39_e1090_d_n7), (-eq39_e1090_d_n8), (-eq39_e1090_d_b0), (-eq39_e1090_d_b1), (-eq39_e1090_d_b2), (-eq39_e1090_d_b3), (-eq39_e1090_d_b4),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e1093;
        let eq39_node_derivatives: [f64; 9] = [eq39_e1093_d_n0, eq39_e1093_d_n1, eq39_e1093_d_n2, eq39_e1093_d_n3, eq39_e1093_d_n4, eq39_e1093_d_n5, eq39_e1093_d_n6, eq39_e1093_d_n7, eq39_e1093_d_n8];
        let eq39_branch_derivatives: [f64; 5] = [eq39_e1093_d_b0, eq39_e1093_d_b1, eq39_e1093_d_b2, eq39_e1093_d_b3, eq39_e1093_d_b4];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e1099, eq40_e1099_d_n4,) = {
    if (var_guard147 != 0.0) {
        let eq40_e1097: f64 = ((nv4 - 0.0) * var_gth);
        (eq40_e1097, var_gth,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e1099;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq40_value),
            4,
            multiplicity * (eq40_e1099_d_n4),
        );
        let (eq41_e1106, eq41_e1106_d_n4,) = {
    if (var_guard147 != 0.0) {
        let eq41_e1103: f64 = ((nv4 - 0.0) * var_cth);
        let eq41_e1104: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq41_e1103);
        (eq41_e1104, (var_cth * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e1106;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq41_value),
            4,
            multiplicity * (eq41_e1106_d_n4),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_cth: f64,
        var_cth_db0: f64,
        var_cth_db1: f64,
        var_cth_db2: f64,
        var_cth_db3: f64,
        var_cth_db4: f64,
        var_cth_dn0: f64,
        var_cth_dn1: f64,
        var_cth_dn2: f64,
        var_cth_dn3: f64,
        var_cth_dn4: f64,
        var_cth_dn5: f64,
        var_cth_dn6: f64,
        var_cth_dn7: f64,
        var_cth_dn8: f64,
        var_devsign: f64,
        var_guard147: f64,
        var_qbgi: f64,
        var_qbgi_db0: f64,
        var_qbgi_db1: f64,
        var_qbgi_db2: f64,
        var_qbgi_db3: f64,
        var_qbgi_db4: f64,
        var_qbgi_dn0: f64,
        var_qbgi_dn1: f64,
        var_qbgi_dn2: f64,
        var_qbgi_dn3: f64,
        var_qbgi_dn4: f64,
        var_qbgi_dn5: f64,
        var_qbgi_dn6: f64,
        var_qbgi_dn7: f64,
        var_qbgi_dn8: f64,
        var_qdbg: f64,
        var_qdbg_db0: f64,
        var_qdbg_db1: f64,
        var_qdbg_db2: f64,
        var_qdbg_db3: f64,
        var_qdbg_db4: f64,
        var_qdbg_dn0: f64,
        var_qdbg_dn1: f64,
        var_qdbg_dn2: f64,
        var_qdbg_dn3: f64,
        var_qdbg_dn4: f64,
        var_qdbg_dn5: f64,
        var_qdbg_dn6: f64,
        var_qdbg_dn7: f64,
        var_qdbg_dn8: f64,
        var_qdi: f64,
        var_qdi_db0: f64,
        var_qdi_db1: f64,
        var_qdi_db2: f64,
        var_qdi_db3: f64,
        var_qdi_db4: f64,
        var_qdi_dn0: f64,
        var_qdi_dn1: f64,
        var_qdi_dn2: f64,
        var_qdi_dn3: f64,
        var_qdi_dn4: f64,
        var_qdi_dn5: f64,
        var_qdi_dn6: f64,
        var_qdi_dn7: f64,
        var_qdi_dn8: f64,
        var_qfgd_parasitic: f64,
        var_qfgd_parasitic_db0: f64,
        var_qfgd_parasitic_db1: f64,
        var_qfgd_parasitic_db2: f64,
        var_qfgd_parasitic_db3: f64,
        var_qfgd_parasitic_db4: f64,
        var_qfgd_parasitic_dn0: f64,
        var_qfgd_parasitic_dn1: f64,
        var_qfgd_parasitic_dn2: f64,
        var_qfgd_parasitic_dn3: f64,
        var_qfgd_parasitic_dn4: f64,
        var_qfgd_parasitic_dn5: f64,
        var_qfgd_parasitic_dn6: f64,
        var_qfgd_parasitic_dn7: f64,
        var_qfgd_parasitic_dn8: f64,
        var_qfgi: f64,
        var_qfgi_db0: f64,
        var_qfgi_db1: f64,
        var_qfgi_db2: f64,
        var_qfgi_db3: f64,
        var_qfgi_db4: f64,
        var_qfgi_dn0: f64,
        var_qfgi_dn1: f64,
        var_qfgi_dn2: f64,
        var_qfgi_dn3: f64,
        var_qfgi_dn4: f64,
        var_qfgi_dn5: f64,
        var_qfgi_dn6: f64,
        var_qfgi_dn7: f64,
        var_qfgi_dn8: f64,
        var_qfgs_parasitic: f64,
        var_qfgs_parasitic_db0: f64,
        var_qfgs_parasitic_db1: f64,
        var_qfgs_parasitic_db2: f64,
        var_qfgs_parasitic_db3: f64,
        var_qfgs_parasitic_db4: f64,
        var_qfgs_parasitic_dn0: f64,
        var_qfgs_parasitic_dn1: f64,
        var_qfgs_parasitic_dn2: f64,
        var_qfgs_parasitic_dn3: f64,
        var_qfgs_parasitic_dn4: f64,
        var_qfgs_parasitic_dn5: f64,
        var_qfgs_parasitic_dn6: f64,
        var_qfgs_parasitic_dn7: f64,
        var_qfgs_parasitic_dn8: f64,
        var_qsbg: f64,
        var_qsbg_db0: f64,
        var_qsbg_db1: f64,
        var_qsbg_db2: f64,
        var_qsbg_db3: f64,
        var_qsbg_db4: f64,
        var_qsbg_dn0: f64,
        var_qsbg_dn1: f64,
        var_qsbg_dn2: f64,
        var_qsbg_dn3: f64,
        var_qsbg_dn4: f64,
        var_qsbg_dn5: f64,
        var_qsbg_dn6: f64,
        var_qsbg_dn7: f64,
        var_qsbg_dn8: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq12_e873_q: f64 = var_qdi;
        let eq12_e874: f64 = (var_devsign * var_qdi);
        let eq12_e874_d_n0: f64 = (var_devsign * var_qdi_dn0);
        let eq12_e874_d_n1: f64 = (var_devsign * var_qdi_dn1);
        let eq12_e874_d_n2: f64 = (var_devsign * var_qdi_dn2);
        let eq12_e874_d_n3: f64 = (var_devsign * var_qdi_dn3);
        let eq12_e874_d_n4: f64 = (var_devsign * var_qdi_dn4);
        let eq12_e874_d_n5: f64 = (var_devsign * var_qdi_dn5);
        let eq12_e874_d_n6: f64 = (var_devsign * var_qdi_dn6);
        let eq12_e874_d_n7: f64 = (var_devsign * var_qdi_dn7);
        let eq12_e874_d_n8: f64 = (var_devsign * var_qdi_dn8);
        let eq12_e874_d_b0: f64 = (var_devsign * var_qdi_db0);
        let eq12_e874_d_b1: f64 = (var_devsign * var_qdi_db1);
        let eq12_e874_d_b2: f64 = (var_devsign * var_qdi_db2);
        let eq12_e874_d_b3: f64 = (var_devsign * var_qdi_db3);
        let eq12_e874_d_b4: f64 = (var_devsign * var_qdi_db4);
        let eq12_e874_q: f64 = (var_devsign * eq12_e873_q);
        let eq12_reactive_node_derivatives: [f64; 9] = [eq12_e874_d_n0, eq12_e874_d_n1, eq12_e874_d_n2, eq12_e874_d_n3, eq12_e874_d_n4, eq12_e874_d_n5, eq12_e874_d_n6, eq12_e874_d_n7, eq12_e874_d_n8];
        let eq12_reactive_branch_derivatives: [f64; 5] = [eq12_e874_d_b0, eq12_e874_d_b1, eq12_e874_d_b2, eq12_e874_d_b3, eq12_e874_d_b4];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e876_q: f64 = var_qfgi;
        let eq13_reactive_node_derivatives: [f64; 9] = [var_qfgi_dn0, var_qfgi_dn1, var_qfgi_dn2, var_qfgi_dn3, var_qfgi_dn4, var_qfgi_dn5, var_qfgi_dn6, var_qfgi_dn7, var_qfgi_dn8];
        let eq13_reactive_branch_derivatives: [f64; 5] = [var_qfgi_db0, var_qfgi_db1, var_qfgi_db2, var_qfgi_db3, var_qfgi_db4];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e879_q: f64 = var_qbgi;
        let eq14_e880: f64 = (var_devsign * var_qbgi);
        let eq14_e880_d_n0: f64 = (var_devsign * var_qbgi_dn0);
        let eq14_e880_d_n1: f64 = (var_devsign * var_qbgi_dn1);
        let eq14_e880_d_n2: f64 = (var_devsign * var_qbgi_dn2);
        let eq14_e880_d_n3: f64 = (var_devsign * var_qbgi_dn3);
        let eq14_e880_d_n4: f64 = (var_devsign * var_qbgi_dn4);
        let eq14_e880_d_n5: f64 = (var_devsign * var_qbgi_dn5);
        let eq14_e880_d_n6: f64 = (var_devsign * var_qbgi_dn6);
        let eq14_e880_d_n7: f64 = (var_devsign * var_qbgi_dn7);
        let eq14_e880_d_n8: f64 = (var_devsign * var_qbgi_dn8);
        let eq14_e880_d_b0: f64 = (var_devsign * var_qbgi_db0);
        let eq14_e880_d_b1: f64 = (var_devsign * var_qbgi_db1);
        let eq14_e880_d_b2: f64 = (var_devsign * var_qbgi_db2);
        let eq14_e880_d_b3: f64 = (var_devsign * var_qbgi_db3);
        let eq14_e880_d_b4: f64 = (var_devsign * var_qbgi_db4);
        let eq14_e880_q: f64 = (var_devsign * eq14_e879_q);
        let eq14_reactive_node_derivatives: [f64; 9] = [eq14_e880_d_n0, eq14_e880_d_n1, eq14_e880_d_n2, eq14_e880_d_n3, eq14_e880_d_n4, eq14_e880_d_n5, eq14_e880_d_n6, eq14_e880_d_n7, eq14_e880_d_n8];
        let eq14_reactive_branch_derivatives: [f64; 5] = [eq14_e880_d_b0, eq14_e880_d_b1, eq14_e880_d_b2, eq14_e880_d_b3, eq14_e880_d_b4];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e882_q: f64 = var_qfgs_parasitic;
        let eq15_reactive_node_derivatives: [f64; 9] = [var_qfgs_parasitic_dn0, var_qfgs_parasitic_dn1, var_qfgs_parasitic_dn2, var_qfgs_parasitic_dn3, var_qfgs_parasitic_dn4, var_qfgs_parasitic_dn5, var_qfgs_parasitic_dn6, var_qfgs_parasitic_dn7, var_qfgs_parasitic_dn8];
        let eq15_reactive_branch_derivatives: [f64; 5] = [var_qfgs_parasitic_db0, var_qfgs_parasitic_db1, var_qfgs_parasitic_db2, var_qfgs_parasitic_db3, var_qfgs_parasitic_db4];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e884_q: f64 = var_qfgd_parasitic;
        let eq16_reactive_node_derivatives: [f64; 9] = [var_qfgd_parasitic_dn0, var_qfgd_parasitic_dn1, var_qfgd_parasitic_dn2, var_qfgd_parasitic_dn3, var_qfgd_parasitic_dn4, var_qfgd_parasitic_dn5, var_qfgd_parasitic_dn6, var_qfgd_parasitic_dn7, var_qfgd_parasitic_dn8];
        let eq16_reactive_branch_derivatives: [f64; 5] = [var_qfgd_parasitic_db0, var_qfgd_parasitic_db1, var_qfgd_parasitic_db2, var_qfgd_parasitic_db3, var_qfgd_parasitic_db4];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e887_q: f64 = var_qsbg;
        let eq17_e888: f64 = (var_devsign * var_qsbg);
        let eq17_e888_d_n0: f64 = (var_devsign * var_qsbg_dn0);
        let eq17_e888_d_n1: f64 = (var_devsign * var_qsbg_dn1);
        let eq17_e888_d_n2: f64 = (var_devsign * var_qsbg_dn2);
        let eq17_e888_d_n3: f64 = (var_devsign * var_qsbg_dn3);
        let eq17_e888_d_n4: f64 = (var_devsign * var_qsbg_dn4);
        let eq17_e888_d_n5: f64 = (var_devsign * var_qsbg_dn5);
        let eq17_e888_d_n6: f64 = (var_devsign * var_qsbg_dn6);
        let eq17_e888_d_n7: f64 = (var_devsign * var_qsbg_dn7);
        let eq17_e888_d_n8: f64 = (var_devsign * var_qsbg_dn8);
        let eq17_e888_d_b0: f64 = (var_devsign * var_qsbg_db0);
        let eq17_e888_d_b1: f64 = (var_devsign * var_qsbg_db1);
        let eq17_e888_d_b2: f64 = (var_devsign * var_qsbg_db2);
        let eq17_e888_d_b3: f64 = (var_devsign * var_qsbg_db3);
        let eq17_e888_d_b4: f64 = (var_devsign * var_qsbg_db4);
        let eq17_e888_q: f64 = (var_devsign * eq17_e887_q);
        let eq17_reactive_node_derivatives: [f64; 9] = [eq17_e888_d_n0, eq17_e888_d_n1, eq17_e888_d_n2, eq17_e888_d_n3, eq17_e888_d_n4, eq17_e888_d_n5, eq17_e888_d_n6, eq17_e888_d_n7, eq17_e888_d_n8];
        let eq17_reactive_branch_derivatives: [f64; 5] = [eq17_e888_d_b0, eq17_e888_d_b1, eq17_e888_d_b2, eq17_e888_d_b3, eq17_e888_d_b4];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e891_q: f64 = var_qdbg;
        let eq18_e892: f64 = (var_devsign * var_qdbg);
        let eq18_e892_d_n0: f64 = (var_devsign * var_qdbg_dn0);
        let eq18_e892_d_n1: f64 = (var_devsign * var_qdbg_dn1);
        let eq18_e892_d_n2: f64 = (var_devsign * var_qdbg_dn2);
        let eq18_e892_d_n3: f64 = (var_devsign * var_qdbg_dn3);
        let eq18_e892_d_n4: f64 = (var_devsign * var_qdbg_dn4);
        let eq18_e892_d_n5: f64 = (var_devsign * var_qdbg_dn5);
        let eq18_e892_d_n6: f64 = (var_devsign * var_qdbg_dn6);
        let eq18_e892_d_n7: f64 = (var_devsign * var_qdbg_dn7);
        let eq18_e892_d_n8: f64 = (var_devsign * var_qdbg_dn8);
        let eq18_e892_d_b0: f64 = (var_devsign * var_qdbg_db0);
        let eq18_e892_d_b1: f64 = (var_devsign * var_qdbg_db1);
        let eq18_e892_d_b2: f64 = (var_devsign * var_qdbg_db2);
        let eq18_e892_d_b3: f64 = (var_devsign * var_qdbg_db3);
        let eq18_e892_d_b4: f64 = (var_devsign * var_qdbg_db4);
        let eq18_e892_q: f64 = (var_devsign * eq18_e891_q);
        let eq18_reactive_node_derivatives: [f64; 9] = [eq18_e892_d_n0, eq18_e892_d_n1, eq18_e892_d_n2, eq18_e892_d_n3, eq18_e892_d_n4, eq18_e892_d_n5, eq18_e892_d_n6, eq18_e892_d_n7, eq18_e892_d_n8];
        let eq18_reactive_branch_derivatives: [f64; 5] = [eq18_e892_d_b0, eq18_e892_d_b1, eq18_e892_d_b2, eq18_e892_d_b3, eq18_e892_d_b4];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq41_e1106, eq41_e1106_d_n0, eq41_e1106_d_n1, eq41_e1106_d_n2, eq41_e1106_d_n3, eq41_e1106_d_n4, eq41_e1106_d_n5, eq41_e1106_d_n6, eq41_e1106_d_n7, eq41_e1106_d_n8, eq41_e1106_d_b0, eq41_e1106_d_b1, eq41_e1106_d_b2, eq41_e1106_d_b3, eq41_e1106_d_b4, eq41_e1106_q,) = {
    if (var_guard147 != 0.0) {
        let eq41_e1103: f64 = ((nv4 - 0.0) * var_cth);
        let eq41_e1103_d_n0: f64 = ((nv4 - 0.0) * var_cth_dn0);
        let eq41_e1103_d_n1: f64 = ((nv4 - 0.0) * var_cth_dn1);
        let eq41_e1103_d_n2: f64 = ((nv4 - 0.0) * var_cth_dn2);
        let eq41_e1103_d_n3: f64 = ((nv4 - 0.0) * var_cth_dn3);
        let eq41_e1103_d_n4: f64 = (var_cth + ((nv4 - 0.0) * var_cth_dn4));
        let eq41_e1103_d_n5: f64 = ((nv4 - 0.0) * var_cth_dn5);
        let eq41_e1103_d_n6: f64 = ((nv4 - 0.0) * var_cth_dn6);
        let eq41_e1103_d_n7: f64 = ((nv4 - 0.0) * var_cth_dn7);
        let eq41_e1103_d_n8: f64 = ((nv4 - 0.0) * var_cth_dn8);
        let eq41_e1103_d_b0: f64 = ((nv4 - 0.0) * var_cth_db0);
        let eq41_e1103_d_b1: f64 = ((nv4 - 0.0) * var_cth_db1);
        let eq41_e1103_d_b2: f64 = ((nv4 - 0.0) * var_cth_db2);
        let eq41_e1103_d_b3: f64 = ((nv4 - 0.0) * var_cth_db3);
        let eq41_e1103_d_b4: f64 = ((nv4 - 0.0) * var_cth_db4);
        let eq41_e1104_q: f64 = eq41_e1103;
        (eq41_e1103, eq41_e1103_d_n0, eq41_e1103_d_n1, eq41_e1103_d_n2, eq41_e1103_d_n3, eq41_e1103_d_n4, eq41_e1103_d_n5, eq41_e1103_d_n6, eq41_e1103_d_n7, eq41_e1103_d_n8, eq41_e1103_d_b0, eq41_e1103_d_b1, eq41_e1103_d_b2, eq41_e1103_d_b3, eq41_e1103_d_b4, eq41_e1104_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_reactive_node_derivatives: [f64; 9] = [eq41_e1106_d_n0, eq41_e1106_d_n1, eq41_e1106_d_n2, eq41_e1106_d_n3, eq41_e1106_d_n4, eq41_e1106_d_n5, eq41_e1106_d_n6, eq41_e1106_d_n7, eq41_e1106_d_n8];
        let eq41_reactive_branch_derivatives: [f64; 5] = [eq41_e1106_d_b0, eq41_e1106_d_b1, eq41_e1106_d_b2, eq41_e1106_d_b3, eq41_e1106_d_b4];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
