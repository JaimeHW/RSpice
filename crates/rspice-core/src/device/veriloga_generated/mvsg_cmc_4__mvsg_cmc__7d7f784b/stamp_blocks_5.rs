#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
        p: &Parameters,
        var_fn61_calc_iq__alpha_phit: f64,
        var_fn61_calc_iq__alpha_phit_db0: f64,
        var_fn61_calc_iq__alpha_phit_db1: f64,
        var_fn61_calc_iq__alpha_phit_db10: f64,
        var_fn61_calc_iq__alpha_phit_db11: f64,
        var_fn61_calc_iq__alpha_phit_db12: f64,
        var_fn61_calc_iq__alpha_phit_db13: f64,
        var_fn61_calc_iq__alpha_phit_db14: f64,
        var_fn61_calc_iq__alpha_phit_db15: f64,
        var_fn61_calc_iq__alpha_phit_db16: f64,
        var_fn61_calc_iq__alpha_phit_db17: f64,
        var_fn61_calc_iq__alpha_phit_db18: f64,
        var_fn61_calc_iq__alpha_phit_db19: f64,
        var_fn61_calc_iq__alpha_phit_db2: f64,
        var_fn61_calc_iq__alpha_phit_db20: f64,
        var_fn61_calc_iq__alpha_phit_db21: f64,
        var_fn61_calc_iq__alpha_phit_db22: f64,
        var_fn61_calc_iq__alpha_phit_db23: f64,
        var_fn61_calc_iq__alpha_phit_db24: f64,
        var_fn61_calc_iq__alpha_phit_db25: f64,
        var_fn61_calc_iq__alpha_phit_db26: f64,
        var_fn61_calc_iq__alpha_phit_db27: f64,
        var_fn61_calc_iq__alpha_phit_db28: f64,
        var_fn61_calc_iq__alpha_phit_db29: f64,
        var_fn61_calc_iq__alpha_phit_db3: f64,
        var_fn61_calc_iq__alpha_phit_db30: f64,
        var_fn61_calc_iq__alpha_phit_db31: f64,
        var_fn61_calc_iq__alpha_phit_db32: f64,
        var_fn61_calc_iq__alpha_phit_db33: f64,
        var_fn61_calc_iq__alpha_phit_db34: f64,
        var_fn61_calc_iq__alpha_phit_db35: f64,
        var_fn61_calc_iq__alpha_phit_db4: f64,
        var_fn61_calc_iq__alpha_phit_db5: f64,
        var_fn61_calc_iq__alpha_phit_db6: f64,
        var_fn61_calc_iq__alpha_phit_db7: f64,
        var_fn61_calc_iq__alpha_phit_db8: f64,
        var_fn61_calc_iq__alpha_phit_db9: f64,
        var_fn61_calc_iq__alpha_phit_dn0: f64,
        var_fn61_calc_iq__alpha_phit_dn1: f64,
        var_fn61_calc_iq__alpha_phit_dn10: f64,
        var_fn61_calc_iq__alpha_phit_dn11: f64,
        var_fn61_calc_iq__alpha_phit_dn12: f64,
        var_fn61_calc_iq__alpha_phit_dn13: f64,
        var_fn61_calc_iq__alpha_phit_dn14: f64,
        var_fn61_calc_iq__alpha_phit_dn15: f64,
        var_fn61_calc_iq__alpha_phit_dn16: f64,
        var_fn61_calc_iq__alpha_phit_dn17: f64,
        var_fn61_calc_iq__alpha_phit_dn18: f64,
        var_fn61_calc_iq__alpha_phit_dn19: f64,
        var_fn61_calc_iq__alpha_phit_dn2: f64,
        var_fn61_calc_iq__alpha_phit_dn20: f64,
        var_fn61_calc_iq__alpha_phit_dn21: f64,
        var_fn61_calc_iq__alpha_phit_dn22: f64,
        var_fn61_calc_iq__alpha_phit_dn23: f64,
        var_fn61_calc_iq__alpha_phit_dn24: f64,
        var_fn61_calc_iq__alpha_phit_dn25: f64,
        var_fn61_calc_iq__alpha_phit_dn26: f64,
        var_fn61_calc_iq__alpha_phit_dn27: f64,
        var_fn61_calc_iq__alpha_phit_dn28: f64,
        var_fn61_calc_iq__alpha_phit_dn29: f64,
        var_fn61_calc_iq__alpha_phit_dn3: f64,
        var_fn61_calc_iq__alpha_phit_dn4: f64,
        var_fn61_calc_iq__alpha_phit_dn5: f64,
        var_fn61_calc_iq__alpha_phit_dn6: f64,
        var_fn61_calc_iq__alpha_phit_dn7: f64,
        var_fn61_calc_iq__alpha_phit_dn8: f64,
        var_fn61_calc_iq__alpha_phit_dn9: f64,
        var_fn61_calc_iq__cb: f64,
        var_fn61_calc_iq__cb_db0: f64,
        var_fn61_calc_iq__cb_db1: f64,
        var_fn61_calc_iq__cb_db10: f64,
        var_fn61_calc_iq__cb_db11: f64,
        var_fn61_calc_iq__cb_db12: f64,
        var_fn61_calc_iq__cb_db13: f64,
        var_fn61_calc_iq__cb_db14: f64,
        var_fn61_calc_iq__cb_db15: f64,
        var_fn61_calc_iq__cb_db16: f64,
        var_fn61_calc_iq__cb_db17: f64,
        var_fn61_calc_iq__cb_db18: f64,
        var_fn61_calc_iq__cb_db19: f64,
        var_fn61_calc_iq__cb_db2: f64,
        var_fn61_calc_iq__cb_db20: f64,
        var_fn61_calc_iq__cb_db21: f64,
        var_fn61_calc_iq__cb_db22: f64,
        var_fn61_calc_iq__cb_db23: f64,
        var_fn61_calc_iq__cb_db24: f64,
        var_fn61_calc_iq__cb_db25: f64,
        var_fn61_calc_iq__cb_db26: f64,
        var_fn61_calc_iq__cb_db27: f64,
        var_fn61_calc_iq__cb_db28: f64,
        var_fn61_calc_iq__cb_db29: f64,
        var_fn61_calc_iq__cb_db3: f64,
        var_fn61_calc_iq__cb_db30: f64,
        var_fn61_calc_iq__cb_db31: f64,
        var_fn61_calc_iq__cb_db32: f64,
        var_fn61_calc_iq__cb_db33: f64,
        var_fn61_calc_iq__cb_db34: f64,
        var_fn61_calc_iq__cb_db35: f64,
        var_fn61_calc_iq__cb_db4: f64,
        var_fn61_calc_iq__cb_db5: f64,
        var_fn61_calc_iq__cb_db6: f64,
        var_fn61_calc_iq__cb_db7: f64,
        var_fn61_calc_iq__cb_db8: f64,
        var_fn61_calc_iq__cb_db9: f64,
        var_fn61_calc_iq__cb_dn0: f64,
        var_fn61_calc_iq__cb_dn1: f64,
        var_fn61_calc_iq__cb_dn10: f64,
        var_fn61_calc_iq__cb_dn11: f64,
        var_fn61_calc_iq__cb_dn12: f64,
        var_fn61_calc_iq__cb_dn13: f64,
        var_fn61_calc_iq__cb_dn14: f64,
        var_fn61_calc_iq__cb_dn15: f64,
        var_fn61_calc_iq__cb_dn16: f64,
        var_fn61_calc_iq__cb_dn17: f64,
        var_fn61_calc_iq__cb_dn18: f64,
        var_fn61_calc_iq__cb_dn19: f64,
        var_fn61_calc_iq__cb_dn2: f64,
        var_fn61_calc_iq__cb_dn20: f64,
        var_fn61_calc_iq__cb_dn21: f64,
        var_fn61_calc_iq__cb_dn22: f64,
        var_fn61_calc_iq__cb_dn23: f64,
        var_fn61_calc_iq__cb_dn24: f64,
        var_fn61_calc_iq__cb_dn25: f64,
        var_fn61_calc_iq__cb_dn26: f64,
        var_fn61_calc_iq__cb_dn27: f64,
        var_fn61_calc_iq__cb_dn28: f64,
        var_fn61_calc_iq__cb_dn29: f64,
        var_fn61_calc_iq__cb_dn3: f64,
        var_fn61_calc_iq__cb_dn4: f64,
        var_fn61_calc_iq__cb_dn5: f64,
        var_fn61_calc_iq__cb_dn6: f64,
        var_fn61_calc_iq__cb_dn7: f64,
        var_fn61_calc_iq__cb_dn8: f64,
        var_fn61_calc_iq__cb_dn9: f64,
        var_fn61_calc_iq__etab: f64,
        var_fn61_calc_iq__etab_db0: f64,
        var_fn61_calc_iq__etab_db1: f64,
        var_fn61_calc_iq__etab_db10: f64,
        var_fn61_calc_iq__etab_db11: f64,
        var_fn61_calc_iq__etab_db12: f64,
        var_fn61_calc_iq__etab_db13: f64,
        var_fn61_calc_iq__etab_db14: f64,
        var_fn61_calc_iq__etab_db15: f64,
        var_fn61_calc_iq__etab_db16: f64,
        var_fn61_calc_iq__etab_db17: f64,
        var_fn61_calc_iq__etab_db18: f64,
        var_fn61_calc_iq__etab_db19: f64,
        var_fn61_calc_iq__etab_db2: f64,
        var_fn61_calc_iq__etab_db20: f64,
        var_fn61_calc_iq__etab_db21: f64,
        var_fn61_calc_iq__etab_db22: f64,
        var_fn61_calc_iq__etab_db23: f64,
        var_fn61_calc_iq__etab_db24: f64,
        var_fn61_calc_iq__etab_db25: f64,
        var_fn61_calc_iq__etab_db26: f64,
        var_fn61_calc_iq__etab_db27: f64,
        var_fn61_calc_iq__etab_db28: f64,
        var_fn61_calc_iq__etab_db29: f64,
        var_fn61_calc_iq__etab_db3: f64,
        var_fn61_calc_iq__etab_db30: f64,
        var_fn61_calc_iq__etab_db31: f64,
        var_fn61_calc_iq__etab_db32: f64,
        var_fn61_calc_iq__etab_db33: f64,
        var_fn61_calc_iq__etab_db34: f64,
        var_fn61_calc_iq__etab_db35: f64,
        var_fn61_calc_iq__etab_db4: f64,
        var_fn61_calc_iq__etab_db5: f64,
        var_fn61_calc_iq__etab_db6: f64,
        var_fn61_calc_iq__etab_db7: f64,
        var_fn61_calc_iq__etab_db8: f64,
        var_fn61_calc_iq__etab_db9: f64,
        var_fn61_calc_iq__etab_dn0: f64,
        var_fn61_calc_iq__etab_dn1: f64,
        var_fn61_calc_iq__etab_dn10: f64,
        var_fn61_calc_iq__etab_dn11: f64,
        var_fn61_calc_iq__etab_dn12: f64,
        var_fn61_calc_iq__etab_dn13: f64,
        var_fn61_calc_iq__etab_dn14: f64,
        var_fn61_calc_iq__etab_dn15: f64,
        var_fn61_calc_iq__etab_dn16: f64,
        var_fn61_calc_iq__etab_dn17: f64,
        var_fn61_calc_iq__etab_dn18: f64,
        var_fn61_calc_iq__etab_dn19: f64,
        var_fn61_calc_iq__etab_dn2: f64,
        var_fn61_calc_iq__etab_dn20: f64,
        var_fn61_calc_iq__etab_dn21: f64,
        var_fn61_calc_iq__etab_dn22: f64,
        var_fn61_calc_iq__etab_dn23: f64,
        var_fn61_calc_iq__etab_dn24: f64,
        var_fn61_calc_iq__etab_dn25: f64,
        var_fn61_calc_iq__etab_dn26: f64,
        var_fn61_calc_iq__etab_dn27: f64,
        var_fn61_calc_iq__etab_dn28: f64,
        var_fn61_calc_iq__etab_dn29: f64,
        var_fn61_calc_iq__etab_dn3: f64,
        var_fn61_calc_iq__etab_dn4: f64,
        var_fn61_calc_iq__etab_dn5: f64,
        var_fn61_calc_iq__etab_dn6: f64,
        var_fn61_calc_iq__etab_dn7: f64,
        var_fn61_calc_iq__etab_dn8: f64,
        var_fn61_calc_iq__etab_dn9: f64,
        var_fn61_calc_iq__ngf: f64,
        var_fn61_calc_iq__qgsflag: f64,
        var_fn61_calc_iq__trapfracdl: f64,
        var_fn61_calc_iq__two_n_phit0: f64,
        var_fn61_calc_iq__two_n_phit0_db0: f64,
        var_fn61_calc_iq__two_n_phit0_db1: f64,
        var_fn61_calc_iq__two_n_phit0_db10: f64,
        var_fn61_calc_iq__two_n_phit0_db11: f64,
        var_fn61_calc_iq__two_n_phit0_db12: f64,
        var_fn61_calc_iq__two_n_phit0_db13: f64,
        var_fn61_calc_iq__two_n_phit0_db14: f64,
        var_fn61_calc_iq__two_n_phit0_db15: f64,
        var_fn61_calc_iq__two_n_phit0_db16: f64,
        var_fn61_calc_iq__two_n_phit0_db17: f64,
        var_fn61_calc_iq__two_n_phit0_db18: f64,
        var_fn61_calc_iq__two_n_phit0_db19: f64,
        var_fn61_calc_iq__two_n_phit0_db2: f64,
        var_fn61_calc_iq__two_n_phit0_db20: f64,
        var_fn61_calc_iq__two_n_phit0_db21: f64,
        var_fn61_calc_iq__two_n_phit0_db22: f64,
        var_fn61_calc_iq__two_n_phit0_db23: f64,
        var_fn61_calc_iq__two_n_phit0_db24: f64,
        var_fn61_calc_iq__two_n_phit0_db25: f64,
        var_fn61_calc_iq__two_n_phit0_db26: f64,
        var_fn61_calc_iq__two_n_phit0_db27: f64,
        var_fn61_calc_iq__two_n_phit0_db28: f64,
        var_fn61_calc_iq__two_n_phit0_db29: f64,
        var_fn61_calc_iq__two_n_phit0_db3: f64,
        var_fn61_calc_iq__two_n_phit0_db30: f64,
        var_fn61_calc_iq__two_n_phit0_db31: f64,
        var_fn61_calc_iq__two_n_phit0_db32: f64,
        var_fn61_calc_iq__two_n_phit0_db33: f64,
        var_fn61_calc_iq__two_n_phit0_db34: f64,
        var_fn61_calc_iq__two_n_phit0_db35: f64,
        var_fn61_calc_iq__two_n_phit0_db4: f64,
        var_fn61_calc_iq__two_n_phit0_db5: f64,
        var_fn61_calc_iq__two_n_phit0_db6: f64,
        var_fn61_calc_iq__two_n_phit0_db7: f64,
        var_fn61_calc_iq__two_n_phit0_db8: f64,
        var_fn61_calc_iq__two_n_phit0_db9: f64,
        var_fn61_calc_iq__two_n_phit0_dn0: f64,
        var_fn61_calc_iq__two_n_phit0_dn1: f64,
        var_fn61_calc_iq__two_n_phit0_dn10: f64,
        var_fn61_calc_iq__two_n_phit0_dn11: f64,
        var_fn61_calc_iq__two_n_phit0_dn12: f64,
        var_fn61_calc_iq__two_n_phit0_dn13: f64,
        var_fn61_calc_iq__two_n_phit0_dn14: f64,
        var_fn61_calc_iq__two_n_phit0_dn15: f64,
        var_fn61_calc_iq__two_n_phit0_dn16: f64,
        var_fn61_calc_iq__two_n_phit0_dn17: f64,
        var_fn61_calc_iq__two_n_phit0_dn18: f64,
        var_fn61_calc_iq__two_n_phit0_dn19: f64,
        var_fn61_calc_iq__two_n_phit0_dn2: f64,
        var_fn61_calc_iq__two_n_phit0_dn20: f64,
        var_fn61_calc_iq__two_n_phit0_dn21: f64,
        var_fn61_calc_iq__two_n_phit0_dn22: f64,
        var_fn61_calc_iq__two_n_phit0_dn23: f64,
        var_fn61_calc_iq__two_n_phit0_dn24: f64,
        var_fn61_calc_iq__two_n_phit0_dn25: f64,
        var_fn61_calc_iq__two_n_phit0_dn26: f64,
        var_fn61_calc_iq__two_n_phit0_dn27: f64,
        var_fn61_calc_iq__two_n_phit0_dn28: f64,
        var_fn61_calc_iq__two_n_phit0_dn29: f64,
        var_fn61_calc_iq__two_n_phit0_dn3: f64,
        var_fn61_calc_iq__two_n_phit0_dn4: f64,
        var_fn61_calc_iq__two_n_phit0_dn5: f64,
        var_fn61_calc_iq__two_n_phit0_dn6: f64,
        var_fn61_calc_iq__two_n_phit0_dn7: f64,
        var_fn61_calc_iq__two_n_phit0_dn8: f64,
        var_fn61_calc_iq__two_n_phit0_dn9: f64,
        var_fn61_calc_iq__type: f64,
        var_fn61_calc_iq__vgsin: f64,
        var_fn61_calc_iq__vgsin_db0: f64,
        var_fn61_calc_iq__vgsin_db1: f64,
        var_fn61_calc_iq__vgsin_db10: f64,
        var_fn61_calc_iq__vgsin_db11: f64,
        var_fn61_calc_iq__vgsin_db12: f64,
        var_fn61_calc_iq__vgsin_db13: f64,
        var_fn61_calc_iq__vgsin_db14: f64,
        var_fn61_calc_iq__vgsin_db15: f64,
        var_fn61_calc_iq__vgsin_db16: f64,
        var_fn61_calc_iq__vgsin_db17: f64,
        var_fn61_calc_iq__vgsin_db18: f64,
        var_fn61_calc_iq__vgsin_db19: f64,
        var_fn61_calc_iq__vgsin_db2: f64,
        var_fn61_calc_iq__vgsin_db20: f64,
        var_fn61_calc_iq__vgsin_db21: f64,
        var_fn61_calc_iq__vgsin_db22: f64,
        var_fn61_calc_iq__vgsin_db23: f64,
        var_fn61_calc_iq__vgsin_db24: f64,
        var_fn61_calc_iq__vgsin_db25: f64,
        var_fn61_calc_iq__vgsin_db26: f64,
        var_fn61_calc_iq__vgsin_db27: f64,
        var_fn61_calc_iq__vgsin_db28: f64,
        var_fn61_calc_iq__vgsin_db29: f64,
        var_fn61_calc_iq__vgsin_db3: f64,
        var_fn61_calc_iq__vgsin_db30: f64,
        var_fn61_calc_iq__vgsin_db31: f64,
        var_fn61_calc_iq__vgsin_db32: f64,
        var_fn61_calc_iq__vgsin_db33: f64,
        var_fn61_calc_iq__vgsin_db34: f64,
        var_fn61_calc_iq__vgsin_db35: f64,
        var_fn61_calc_iq__vgsin_db4: f64,
        var_fn61_calc_iq__vgsin_db5: f64,
        var_fn61_calc_iq__vgsin_db6: f64,
        var_fn61_calc_iq__vgsin_db7: f64,
        var_fn61_calc_iq__vgsin_db8: f64,
        var_fn61_calc_iq__vgsin_db9: f64,
        var_fn61_calc_iq__vgsin_dn0: f64,
        var_fn61_calc_iq__vgsin_dn1: f64,
        var_fn61_calc_iq__vgsin_dn10: f64,
        var_fn61_calc_iq__vgsin_dn11: f64,
        var_fn61_calc_iq__vgsin_dn12: f64,
        var_fn61_calc_iq__vgsin_dn13: f64,
        var_fn61_calc_iq__vgsin_dn14: f64,
        var_fn61_calc_iq__vgsin_dn15: f64,
        var_fn61_calc_iq__vgsin_dn16: f64,
        var_fn61_calc_iq__vgsin_dn17: f64,
        var_fn61_calc_iq__vgsin_dn18: f64,
        var_fn61_calc_iq__vgsin_dn19: f64,
        var_fn61_calc_iq__vgsin_dn2: f64,
        var_fn61_calc_iq__vgsin_dn20: f64,
        var_fn61_calc_iq__vgsin_dn21: f64,
        var_fn61_calc_iq__vgsin_dn22: f64,
        var_fn61_calc_iq__vgsin_dn23: f64,
        var_fn61_calc_iq__vgsin_dn24: f64,
        var_fn61_calc_iq__vgsin_dn25: f64,
        var_fn61_calc_iq__vgsin_dn26: f64,
        var_fn61_calc_iq__vgsin_dn27: f64,
        var_fn61_calc_iq__vgsin_dn28: f64,
        var_fn61_calc_iq__vgsin_dn29: f64,
        var_fn61_calc_iq__vgsin_dn3: f64,
        var_fn61_calc_iq__vgsin_dn4: f64,
        var_fn61_calc_iq__vgsin_dn5: f64,
        var_fn61_calc_iq__vgsin_dn6: f64,
        var_fn61_calc_iq__vgsin_dn7: f64,
        var_fn61_calc_iq__vgsin_dn8: f64,
        var_fn61_calc_iq__vgsin_dn9: f64,
        var_fn61_calc_iq__vtof: f64,
        var_fn61_calc_iq__vtof_db0: f64,
        var_fn61_calc_iq__vtof_db1: f64,
        var_fn61_calc_iq__vtof_db10: f64,
        var_fn61_calc_iq__vtof_db11: f64,
        var_fn61_calc_iq__vtof_db12: f64,
        var_fn61_calc_iq__vtof_db13: f64,
        var_fn61_calc_iq__vtof_db14: f64,
        var_fn61_calc_iq__vtof_db15: f64,
        var_fn61_calc_iq__vtof_db16: f64,
        var_fn61_calc_iq__vtof_db17: f64,
        var_fn61_calc_iq__vtof_db18: f64,
        var_fn61_calc_iq__vtof_db19: f64,
        var_fn61_calc_iq__vtof_db2: f64,
        var_fn61_calc_iq__vtof_db20: f64,
        var_fn61_calc_iq__vtof_db21: f64,
        var_fn61_calc_iq__vtof_db22: f64,
        var_fn61_calc_iq__vtof_db23: f64,
        var_fn61_calc_iq__vtof_db24: f64,
        var_fn61_calc_iq__vtof_db25: f64,
        var_fn61_calc_iq__vtof_db26: f64,
        var_fn61_calc_iq__vtof_db27: f64,
        var_fn61_calc_iq__vtof_db28: f64,
        var_fn61_calc_iq__vtof_db29: f64,
        var_fn61_calc_iq__vtof_db3: f64,
        var_fn61_calc_iq__vtof_db30: f64,
        var_fn61_calc_iq__vtof_db31: f64,
        var_fn61_calc_iq__vtof_db32: f64,
        var_fn61_calc_iq__vtof_db33: f64,
        var_fn61_calc_iq__vtof_db34: f64,
        var_fn61_calc_iq__vtof_db35: f64,
        var_fn61_calc_iq__vtof_db4: f64,
        var_fn61_calc_iq__vtof_db5: f64,
        var_fn61_calc_iq__vtof_db6: f64,
        var_fn61_calc_iq__vtof_db7: f64,
        var_fn61_calc_iq__vtof_db8: f64,
        var_fn61_calc_iq__vtof_db9: f64,
        var_fn61_calc_iq__vtof_dn0: f64,
        var_fn61_calc_iq__vtof_dn1: f64,
        var_fn61_calc_iq__vtof_dn10: f64,
        var_fn61_calc_iq__vtof_dn11: f64,
        var_fn61_calc_iq__vtof_dn12: f64,
        var_fn61_calc_iq__vtof_dn13: f64,
        var_fn61_calc_iq__vtof_dn14: f64,
        var_fn61_calc_iq__vtof_dn15: f64,
        var_fn61_calc_iq__vtof_dn16: f64,
        var_fn61_calc_iq__vtof_dn17: f64,
        var_fn61_calc_iq__vtof_dn18: f64,
        var_fn61_calc_iq__vtof_dn19: f64,
        var_fn61_calc_iq__vtof_dn2: f64,
        var_fn61_calc_iq__vtof_dn20: f64,
        var_fn61_calc_iq__vtof_dn21: f64,
        var_fn61_calc_iq__vtof_dn22: f64,
        var_fn61_calc_iq__vtof_dn23: f64,
        var_fn61_calc_iq__vtof_dn24: f64,
        var_fn61_calc_iq__vtof_dn25: f64,
        var_fn61_calc_iq__vtof_dn26: f64,
        var_fn61_calc_iq__vtof_dn27: f64,
        var_fn61_calc_iq__vtof_dn28: f64,
        var_fn61_calc_iq__vtof_dn29: f64,
        var_fn61_calc_iq__vtof_dn3: f64,
        var_fn61_calc_iq__vtof_dn4: f64,
        var_fn61_calc_iq__vtof_dn5: f64,
        var_fn61_calc_iq__vtof_dn6: f64,
        var_fn61_calc_iq__vtof_dn7: f64,
        var_fn61_calc_iq__vtof_dn8: f64,
        var_fn61_calc_iq__vtof_dn9: f64,
        var_fn61_calc_iq__w: f64,
        var_guard60: f64,
        var_guard87: f64,
        var_guard90: f64,
        var_guard91: f64,
        var_fn61_calc_iq__etags_slot: &mut f64,
        var_fn61_calc_iq__etags_db0_slot: &mut f64,
        var_fn61_calc_iq__etags_db1_slot: &mut f64,
        var_fn61_calc_iq__etags_db10_slot: &mut f64,
        var_fn61_calc_iq__etags_db11_slot: &mut f64,
        var_fn61_calc_iq__etags_db12_slot: &mut f64,
        var_fn61_calc_iq__etags_db13_slot: &mut f64,
        var_fn61_calc_iq__etags_db14_slot: &mut f64,
        var_fn61_calc_iq__etags_db15_slot: &mut f64,
        var_fn61_calc_iq__etags_db16_slot: &mut f64,
        var_fn61_calc_iq__etags_db17_slot: &mut f64,
        var_fn61_calc_iq__etags_db18_slot: &mut f64,
        var_fn61_calc_iq__etags_db19_slot: &mut f64,
        var_fn61_calc_iq__etags_db2_slot: &mut f64,
        var_fn61_calc_iq__etags_db20_slot: &mut f64,
        var_fn61_calc_iq__etags_db21_slot: &mut f64,
        var_fn61_calc_iq__etags_db22_slot: &mut f64,
        var_fn61_calc_iq__etags_db23_slot: &mut f64,
        var_fn61_calc_iq__etags_db24_slot: &mut f64,
        var_fn61_calc_iq__etags_db25_slot: &mut f64,
        var_fn61_calc_iq__etags_db26_slot: &mut f64,
        var_fn61_calc_iq__etags_db27_slot: &mut f64,
        var_fn61_calc_iq__etags_db28_slot: &mut f64,
        var_fn61_calc_iq__etags_db29_slot: &mut f64,
        var_fn61_calc_iq__etags_db3_slot: &mut f64,
        var_fn61_calc_iq__etags_db30_slot: &mut f64,
        var_fn61_calc_iq__etags_db31_slot: &mut f64,
        var_fn61_calc_iq__etags_db32_slot: &mut f64,
        var_fn61_calc_iq__etags_db33_slot: &mut f64,
        var_fn61_calc_iq__etags_db34_slot: &mut f64,
        var_fn61_calc_iq__etags_db35_slot: &mut f64,
        var_fn61_calc_iq__etags_db4_slot: &mut f64,
        var_fn61_calc_iq__etags_db5_slot: &mut f64,
        var_fn61_calc_iq__etags_db6_slot: &mut f64,
        var_fn61_calc_iq__etags_db7_slot: &mut f64,
        var_fn61_calc_iq__etags_db8_slot: &mut f64,
        var_fn61_calc_iq__etags_db9_slot: &mut f64,
        var_fn61_calc_iq__etags_dn0_slot: &mut f64,
        var_fn61_calc_iq__etags_dn1_slot: &mut f64,
        var_fn61_calc_iq__etags_dn10_slot: &mut f64,
        var_fn61_calc_iq__etags_dn11_slot: &mut f64,
        var_fn61_calc_iq__etags_dn12_slot: &mut f64,
        var_fn61_calc_iq__etags_dn13_slot: &mut f64,
        var_fn61_calc_iq__etags_dn14_slot: &mut f64,
        var_fn61_calc_iq__etags_dn15_slot: &mut f64,
        var_fn61_calc_iq__etags_dn16_slot: &mut f64,
        var_fn61_calc_iq__etags_dn17_slot: &mut f64,
        var_fn61_calc_iq__etags_dn18_slot: &mut f64,
        var_fn61_calc_iq__etags_dn19_slot: &mut f64,
        var_fn61_calc_iq__etags_dn2_slot: &mut f64,
        var_fn61_calc_iq__etags_dn20_slot: &mut f64,
        var_fn61_calc_iq__etags_dn21_slot: &mut f64,
        var_fn61_calc_iq__etags_dn22_slot: &mut f64,
        var_fn61_calc_iq__etags_dn23_slot: &mut f64,
        var_fn61_calc_iq__etags_dn24_slot: &mut f64,
        var_fn61_calc_iq__etags_dn25_slot: &mut f64,
        var_fn61_calc_iq__etags_dn26_slot: &mut f64,
        var_fn61_calc_iq__etags_dn27_slot: &mut f64,
        var_fn61_calc_iq__etags_dn28_slot: &mut f64,
        var_fn61_calc_iq__etags_dn29_slot: &mut f64,
        var_fn61_calc_iq__etags_dn3_slot: &mut f64,
        var_fn61_calc_iq__etags_dn4_slot: &mut f64,
        var_fn61_calc_iq__etags_dn5_slot: &mut f64,
        var_fn61_calc_iq__etags_dn6_slot: &mut f64,
        var_fn61_calc_iq__etags_dn7_slot: &mut f64,
        var_fn61_calc_iq__etags_dn8_slot: &mut f64,
        var_fn61_calc_iq__etags_dn9_slot: &mut f64,
        var_fn61_calc_iq__exparg_slot: &mut f64,
        var_fn61_calc_iq__exparg_db0_slot: &mut f64,
        var_fn61_calc_iq__exparg_db1_slot: &mut f64,
        var_fn61_calc_iq__exparg_db10_slot: &mut f64,
        var_fn61_calc_iq__exparg_db11_slot: &mut f64,
        var_fn61_calc_iq__exparg_db12_slot: &mut f64,
        var_fn61_calc_iq__exparg_db13_slot: &mut f64,
        var_fn61_calc_iq__exparg_db14_slot: &mut f64,
        var_fn61_calc_iq__exparg_db15_slot: &mut f64,
        var_fn61_calc_iq__exparg_db16_slot: &mut f64,
        var_fn61_calc_iq__exparg_db17_slot: &mut f64,
        var_fn61_calc_iq__exparg_db18_slot: &mut f64,
        var_fn61_calc_iq__exparg_db19_slot: &mut f64,
        var_fn61_calc_iq__exparg_db2_slot: &mut f64,
        var_fn61_calc_iq__exparg_db20_slot: &mut f64,
        var_fn61_calc_iq__exparg_db21_slot: &mut f64,
        var_fn61_calc_iq__exparg_db22_slot: &mut f64,
        var_fn61_calc_iq__exparg_db23_slot: &mut f64,
        var_fn61_calc_iq__exparg_db24_slot: &mut f64,
        var_fn61_calc_iq__exparg_db25_slot: &mut f64,
        var_fn61_calc_iq__exparg_db26_slot: &mut f64,
        var_fn61_calc_iq__exparg_db27_slot: &mut f64,
        var_fn61_calc_iq__exparg_db28_slot: &mut f64,
        var_fn61_calc_iq__exparg_db29_slot: &mut f64,
        var_fn61_calc_iq__exparg_db3_slot: &mut f64,
        var_fn61_calc_iq__exparg_db30_slot: &mut f64,
        var_fn61_calc_iq__exparg_db31_slot: &mut f64,
        var_fn61_calc_iq__exparg_db32_slot: &mut f64,
        var_fn61_calc_iq__exparg_db33_slot: &mut f64,
        var_fn61_calc_iq__exparg_db34_slot: &mut f64,
        var_fn61_calc_iq__exparg_db35_slot: &mut f64,
        var_fn61_calc_iq__exparg_db4_slot: &mut f64,
        var_fn61_calc_iq__exparg_db5_slot: &mut f64,
        var_fn61_calc_iq__exparg_db6_slot: &mut f64,
        var_fn61_calc_iq__exparg_db7_slot: &mut f64,
        var_fn61_calc_iq__exparg_db8_slot: &mut f64,
        var_fn61_calc_iq__exparg_db9_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn0_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn1_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn10_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn11_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn12_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn13_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn14_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn15_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn16_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn17_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn18_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn19_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn2_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn20_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn21_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn22_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn23_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn24_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn25_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn26_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn27_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn28_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn29_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn3_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn4_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn5_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn6_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn7_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn8_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn9_slot: &mut f64,
        var_fn61_calc_iq__qbout_slot: &mut f64,
        var_fn61_calc_iq__qbout_db0_slot: &mut f64,
        var_fn61_calc_iq__qbout_db1_slot: &mut f64,
        var_fn61_calc_iq__qbout_db10_slot: &mut f64,
        var_fn61_calc_iq__qbout_db11_slot: &mut f64,
        var_fn61_calc_iq__qbout_db12_slot: &mut f64,
        var_fn61_calc_iq__qbout_db13_slot: &mut f64,
        var_fn61_calc_iq__qbout_db14_slot: &mut f64,
        var_fn61_calc_iq__qbout_db15_slot: &mut f64,
        var_fn61_calc_iq__qbout_db16_slot: &mut f64,
        var_fn61_calc_iq__qbout_db17_slot: &mut f64,
        var_fn61_calc_iq__qbout_db18_slot: &mut f64,
        var_fn61_calc_iq__qbout_db19_slot: &mut f64,
        var_fn61_calc_iq__qbout_db2_slot: &mut f64,
        var_fn61_calc_iq__qbout_db20_slot: &mut f64,
        var_fn61_calc_iq__qbout_db21_slot: &mut f64,
        var_fn61_calc_iq__qbout_db22_slot: &mut f64,
        var_fn61_calc_iq__qbout_db23_slot: &mut f64,
        var_fn61_calc_iq__qbout_db24_slot: &mut f64,
        var_fn61_calc_iq__qbout_db25_slot: &mut f64,
        var_fn61_calc_iq__qbout_db26_slot: &mut f64,
        var_fn61_calc_iq__qbout_db27_slot: &mut f64,
        var_fn61_calc_iq__qbout_db28_slot: &mut f64,
        var_fn61_calc_iq__qbout_db29_slot: &mut f64,
        var_fn61_calc_iq__qbout_db3_slot: &mut f64,
        var_fn61_calc_iq__qbout_db30_slot: &mut f64,
        var_fn61_calc_iq__qbout_db31_slot: &mut f64,
        var_fn61_calc_iq__qbout_db32_slot: &mut f64,
        var_fn61_calc_iq__qbout_db33_slot: &mut f64,
        var_fn61_calc_iq__qbout_db34_slot: &mut f64,
        var_fn61_calc_iq__qbout_db35_slot: &mut f64,
        var_fn61_calc_iq__qbout_db4_slot: &mut f64,
        var_fn61_calc_iq__qbout_db5_slot: &mut f64,
        var_fn61_calc_iq__qbout_db6_slot: &mut f64,
        var_fn61_calc_iq__qbout_db7_slot: &mut f64,
        var_fn61_calc_iq__qbout_db8_slot: &mut f64,
        var_fn61_calc_iq__qbout_db9_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn0_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn1_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn10_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn11_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn12_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn13_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn14_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn15_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn16_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn17_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn18_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn19_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn2_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn20_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn21_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn22_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn23_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn24_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn25_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn26_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn27_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn28_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn29_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn3_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn4_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn5_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn6_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn7_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn8_slot: &mut f64,
        var_fn61_calc_iq__qbout_dn9_slot: &mut f64,
        var_fn61_calc_iq__qcout_slot: &mut f64,
        var_fn61_calc_iq__qcout_db0_slot: &mut f64,
        var_fn61_calc_iq__qcout_db1_slot: &mut f64,
        var_fn61_calc_iq__qcout_db10_slot: &mut f64,
        var_fn61_calc_iq__qcout_db11_slot: &mut f64,
        var_fn61_calc_iq__qcout_db12_slot: &mut f64,
        var_fn61_calc_iq__qcout_db13_slot: &mut f64,
        var_fn61_calc_iq__qcout_db14_slot: &mut f64,
        var_fn61_calc_iq__qcout_db15_slot: &mut f64,
        var_fn61_calc_iq__qcout_db16_slot: &mut f64,
        var_fn61_calc_iq__qcout_db17_slot: &mut f64,
        var_fn61_calc_iq__qcout_db18_slot: &mut f64,
        var_fn61_calc_iq__qcout_db19_slot: &mut f64,
        var_fn61_calc_iq__qcout_db2_slot: &mut f64,
        var_fn61_calc_iq__qcout_db20_slot: &mut f64,
        var_fn61_calc_iq__qcout_db21_slot: &mut f64,
        var_fn61_calc_iq__qcout_db22_slot: &mut f64,
        var_fn61_calc_iq__qcout_db23_slot: &mut f64,
        var_fn61_calc_iq__qcout_db24_slot: &mut f64,
        var_fn61_calc_iq__qcout_db25_slot: &mut f64,
        var_fn61_calc_iq__qcout_db26_slot: &mut f64,
        var_fn61_calc_iq__qcout_db27_slot: &mut f64,
        var_fn61_calc_iq__qcout_db28_slot: &mut f64,
        var_fn61_calc_iq__qcout_db29_slot: &mut f64,
        var_fn61_calc_iq__qcout_db3_slot: &mut f64,
        var_fn61_calc_iq__qcout_db30_slot: &mut f64,
        var_fn61_calc_iq__qcout_db31_slot: &mut f64,
        var_fn61_calc_iq__qcout_db32_slot: &mut f64,
        var_fn61_calc_iq__qcout_db33_slot: &mut f64,
        var_fn61_calc_iq__qcout_db34_slot: &mut f64,
        var_fn61_calc_iq__qcout_db35_slot: &mut f64,
        var_fn61_calc_iq__qcout_db4_slot: &mut f64,
        var_fn61_calc_iq__qcout_db5_slot: &mut f64,
        var_fn61_calc_iq__qcout_db6_slot: &mut f64,
        var_fn61_calc_iq__qcout_db7_slot: &mut f64,
        var_fn61_calc_iq__qcout_db8_slot: &mut f64,
        var_fn61_calc_iq__qcout_db9_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn0_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn1_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn10_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn11_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn12_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn13_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn14_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn15_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn16_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn17_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn18_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn19_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn2_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn20_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn21_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn22_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn23_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn24_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn25_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn26_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn27_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn28_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn29_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn3_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn4_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn5_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn6_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn7_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn8_slot: &mut f64,
        var_fn61_calc_iq__qcout_dn9_slot: &mut f64,
        var_guard92_slot: &mut f64,
        var_guard93_slot: &mut f64,
    ) {
        let mut var_fn61_calc_iq__etags: f64 = *var_fn61_calc_iq__etags_slot;
        let mut var_fn61_calc_iq__etags_db0: f64 = *var_fn61_calc_iq__etags_db0_slot;
        let mut var_fn61_calc_iq__etags_db1: f64 = *var_fn61_calc_iq__etags_db1_slot;
        let mut var_fn61_calc_iq__etags_db10: f64 = *var_fn61_calc_iq__etags_db10_slot;
        let mut var_fn61_calc_iq__etags_db11: f64 = *var_fn61_calc_iq__etags_db11_slot;
        let mut var_fn61_calc_iq__etags_db12: f64 = *var_fn61_calc_iq__etags_db12_slot;
        let mut var_fn61_calc_iq__etags_db13: f64 = *var_fn61_calc_iq__etags_db13_slot;
        let mut var_fn61_calc_iq__etags_db14: f64 = *var_fn61_calc_iq__etags_db14_slot;
        let mut var_fn61_calc_iq__etags_db15: f64 = *var_fn61_calc_iq__etags_db15_slot;
        let mut var_fn61_calc_iq__etags_db16: f64 = *var_fn61_calc_iq__etags_db16_slot;
        let mut var_fn61_calc_iq__etags_db17: f64 = *var_fn61_calc_iq__etags_db17_slot;
        let mut var_fn61_calc_iq__etags_db18: f64 = *var_fn61_calc_iq__etags_db18_slot;
        let mut var_fn61_calc_iq__etags_db19: f64 = *var_fn61_calc_iq__etags_db19_slot;
        let mut var_fn61_calc_iq__etags_db2: f64 = *var_fn61_calc_iq__etags_db2_slot;
        let mut var_fn61_calc_iq__etags_db20: f64 = *var_fn61_calc_iq__etags_db20_slot;
        let mut var_fn61_calc_iq__etags_db21: f64 = *var_fn61_calc_iq__etags_db21_slot;
        let mut var_fn61_calc_iq__etags_db22: f64 = *var_fn61_calc_iq__etags_db22_slot;
        let mut var_fn61_calc_iq__etags_db23: f64 = *var_fn61_calc_iq__etags_db23_slot;
        let mut var_fn61_calc_iq__etags_db24: f64 = *var_fn61_calc_iq__etags_db24_slot;
        let mut var_fn61_calc_iq__etags_db25: f64 = *var_fn61_calc_iq__etags_db25_slot;
        let mut var_fn61_calc_iq__etags_db26: f64 = *var_fn61_calc_iq__etags_db26_slot;
        let mut var_fn61_calc_iq__etags_db27: f64 = *var_fn61_calc_iq__etags_db27_slot;
        let mut var_fn61_calc_iq__etags_db28: f64 = *var_fn61_calc_iq__etags_db28_slot;
        let mut var_fn61_calc_iq__etags_db29: f64 = *var_fn61_calc_iq__etags_db29_slot;
        let mut var_fn61_calc_iq__etags_db3: f64 = *var_fn61_calc_iq__etags_db3_slot;
        let mut var_fn61_calc_iq__etags_db30: f64 = *var_fn61_calc_iq__etags_db30_slot;
        let mut var_fn61_calc_iq__etags_db31: f64 = *var_fn61_calc_iq__etags_db31_slot;
        let mut var_fn61_calc_iq__etags_db32: f64 = *var_fn61_calc_iq__etags_db32_slot;
        let mut var_fn61_calc_iq__etags_db33: f64 = *var_fn61_calc_iq__etags_db33_slot;
        let mut var_fn61_calc_iq__etags_db34: f64 = *var_fn61_calc_iq__etags_db34_slot;
        let mut var_fn61_calc_iq__etags_db35: f64 = *var_fn61_calc_iq__etags_db35_slot;
        let mut var_fn61_calc_iq__etags_db4: f64 = *var_fn61_calc_iq__etags_db4_slot;
        let mut var_fn61_calc_iq__etags_db5: f64 = *var_fn61_calc_iq__etags_db5_slot;
        let mut var_fn61_calc_iq__etags_db6: f64 = *var_fn61_calc_iq__etags_db6_slot;
        let mut var_fn61_calc_iq__etags_db7: f64 = *var_fn61_calc_iq__etags_db7_slot;
        let mut var_fn61_calc_iq__etags_db8: f64 = *var_fn61_calc_iq__etags_db8_slot;
        let mut var_fn61_calc_iq__etags_db9: f64 = *var_fn61_calc_iq__etags_db9_slot;
        let mut var_fn61_calc_iq__etags_dn0: f64 = *var_fn61_calc_iq__etags_dn0_slot;
        let mut var_fn61_calc_iq__etags_dn1: f64 = *var_fn61_calc_iq__etags_dn1_slot;
        let mut var_fn61_calc_iq__etags_dn10: f64 = *var_fn61_calc_iq__etags_dn10_slot;
        let mut var_fn61_calc_iq__etags_dn11: f64 = *var_fn61_calc_iq__etags_dn11_slot;
        let mut var_fn61_calc_iq__etags_dn12: f64 = *var_fn61_calc_iq__etags_dn12_slot;
        let mut var_fn61_calc_iq__etags_dn13: f64 = *var_fn61_calc_iq__etags_dn13_slot;
        let mut var_fn61_calc_iq__etags_dn14: f64 = *var_fn61_calc_iq__etags_dn14_slot;
        let mut var_fn61_calc_iq__etags_dn15: f64 = *var_fn61_calc_iq__etags_dn15_slot;
        let mut var_fn61_calc_iq__etags_dn16: f64 = *var_fn61_calc_iq__etags_dn16_slot;
        let mut var_fn61_calc_iq__etags_dn17: f64 = *var_fn61_calc_iq__etags_dn17_slot;
        let mut var_fn61_calc_iq__etags_dn18: f64 = *var_fn61_calc_iq__etags_dn18_slot;
        let mut var_fn61_calc_iq__etags_dn19: f64 = *var_fn61_calc_iq__etags_dn19_slot;
        let mut var_fn61_calc_iq__etags_dn2: f64 = *var_fn61_calc_iq__etags_dn2_slot;
        let mut var_fn61_calc_iq__etags_dn20: f64 = *var_fn61_calc_iq__etags_dn20_slot;
        let mut var_fn61_calc_iq__etags_dn21: f64 = *var_fn61_calc_iq__etags_dn21_slot;
        let mut var_fn61_calc_iq__etags_dn22: f64 = *var_fn61_calc_iq__etags_dn22_slot;
        let mut var_fn61_calc_iq__etags_dn23: f64 = *var_fn61_calc_iq__etags_dn23_slot;
        let mut var_fn61_calc_iq__etags_dn24: f64 = *var_fn61_calc_iq__etags_dn24_slot;
        let mut var_fn61_calc_iq__etags_dn25: f64 = *var_fn61_calc_iq__etags_dn25_slot;
        let mut var_fn61_calc_iq__etags_dn26: f64 = *var_fn61_calc_iq__etags_dn26_slot;
        let mut var_fn61_calc_iq__etags_dn27: f64 = *var_fn61_calc_iq__etags_dn27_slot;
        let mut var_fn61_calc_iq__etags_dn28: f64 = *var_fn61_calc_iq__etags_dn28_slot;
        let mut var_fn61_calc_iq__etags_dn29: f64 = *var_fn61_calc_iq__etags_dn29_slot;
        let mut var_fn61_calc_iq__etags_dn3: f64 = *var_fn61_calc_iq__etags_dn3_slot;
        let mut var_fn61_calc_iq__etags_dn4: f64 = *var_fn61_calc_iq__etags_dn4_slot;
        let mut var_fn61_calc_iq__etags_dn5: f64 = *var_fn61_calc_iq__etags_dn5_slot;
        let mut var_fn61_calc_iq__etags_dn6: f64 = *var_fn61_calc_iq__etags_dn6_slot;
        let mut var_fn61_calc_iq__etags_dn7: f64 = *var_fn61_calc_iq__etags_dn7_slot;
        let mut var_fn61_calc_iq__etags_dn8: f64 = *var_fn61_calc_iq__etags_dn8_slot;
        let mut var_fn61_calc_iq__etags_dn9: f64 = *var_fn61_calc_iq__etags_dn9_slot;
        let mut var_fn61_calc_iq__exparg: f64 = *var_fn61_calc_iq__exparg_slot;
        let mut var_fn61_calc_iq__exparg_db0: f64 = *var_fn61_calc_iq__exparg_db0_slot;
        let mut var_fn61_calc_iq__exparg_db1: f64 = *var_fn61_calc_iq__exparg_db1_slot;
        let mut var_fn61_calc_iq__exparg_db10: f64 = *var_fn61_calc_iq__exparg_db10_slot;
        let mut var_fn61_calc_iq__exparg_db11: f64 = *var_fn61_calc_iq__exparg_db11_slot;
        let mut var_fn61_calc_iq__exparg_db12: f64 = *var_fn61_calc_iq__exparg_db12_slot;
        let mut var_fn61_calc_iq__exparg_db13: f64 = *var_fn61_calc_iq__exparg_db13_slot;
        let mut var_fn61_calc_iq__exparg_db14: f64 = *var_fn61_calc_iq__exparg_db14_slot;
        let mut var_fn61_calc_iq__exparg_db15: f64 = *var_fn61_calc_iq__exparg_db15_slot;
        let mut var_fn61_calc_iq__exparg_db16: f64 = *var_fn61_calc_iq__exparg_db16_slot;
        let mut var_fn61_calc_iq__exparg_db17: f64 = *var_fn61_calc_iq__exparg_db17_slot;
        let mut var_fn61_calc_iq__exparg_db18: f64 = *var_fn61_calc_iq__exparg_db18_slot;
        let mut var_fn61_calc_iq__exparg_db19: f64 = *var_fn61_calc_iq__exparg_db19_slot;
        let mut var_fn61_calc_iq__exparg_db2: f64 = *var_fn61_calc_iq__exparg_db2_slot;
        let mut var_fn61_calc_iq__exparg_db20: f64 = *var_fn61_calc_iq__exparg_db20_slot;
        let mut var_fn61_calc_iq__exparg_db21: f64 = *var_fn61_calc_iq__exparg_db21_slot;
        let mut var_fn61_calc_iq__exparg_db22: f64 = *var_fn61_calc_iq__exparg_db22_slot;
        let mut var_fn61_calc_iq__exparg_db23: f64 = *var_fn61_calc_iq__exparg_db23_slot;
        let mut var_fn61_calc_iq__exparg_db24: f64 = *var_fn61_calc_iq__exparg_db24_slot;
        let mut var_fn61_calc_iq__exparg_db25: f64 = *var_fn61_calc_iq__exparg_db25_slot;
        let mut var_fn61_calc_iq__exparg_db26: f64 = *var_fn61_calc_iq__exparg_db26_slot;
        let mut var_fn61_calc_iq__exparg_db27: f64 = *var_fn61_calc_iq__exparg_db27_slot;
        let mut var_fn61_calc_iq__exparg_db28: f64 = *var_fn61_calc_iq__exparg_db28_slot;
        let mut var_fn61_calc_iq__exparg_db29: f64 = *var_fn61_calc_iq__exparg_db29_slot;
        let mut var_fn61_calc_iq__exparg_db3: f64 = *var_fn61_calc_iq__exparg_db3_slot;
        let mut var_fn61_calc_iq__exparg_db30: f64 = *var_fn61_calc_iq__exparg_db30_slot;
        let mut var_fn61_calc_iq__exparg_db31: f64 = *var_fn61_calc_iq__exparg_db31_slot;
        let mut var_fn61_calc_iq__exparg_db32: f64 = *var_fn61_calc_iq__exparg_db32_slot;
        let mut var_fn61_calc_iq__exparg_db33: f64 = *var_fn61_calc_iq__exparg_db33_slot;
        let mut var_fn61_calc_iq__exparg_db34: f64 = *var_fn61_calc_iq__exparg_db34_slot;
        let mut var_fn61_calc_iq__exparg_db35: f64 = *var_fn61_calc_iq__exparg_db35_slot;
        let mut var_fn61_calc_iq__exparg_db4: f64 = *var_fn61_calc_iq__exparg_db4_slot;
        let mut var_fn61_calc_iq__exparg_db5: f64 = *var_fn61_calc_iq__exparg_db5_slot;
        let mut var_fn61_calc_iq__exparg_db6: f64 = *var_fn61_calc_iq__exparg_db6_slot;
        let mut var_fn61_calc_iq__exparg_db7: f64 = *var_fn61_calc_iq__exparg_db7_slot;
        let mut var_fn61_calc_iq__exparg_db8: f64 = *var_fn61_calc_iq__exparg_db8_slot;
        let mut var_fn61_calc_iq__exparg_db9: f64 = *var_fn61_calc_iq__exparg_db9_slot;
        let mut var_fn61_calc_iq__exparg_dn0: f64 = *var_fn61_calc_iq__exparg_dn0_slot;
        let mut var_fn61_calc_iq__exparg_dn1: f64 = *var_fn61_calc_iq__exparg_dn1_slot;
        let mut var_fn61_calc_iq__exparg_dn10: f64 = *var_fn61_calc_iq__exparg_dn10_slot;
        let mut var_fn61_calc_iq__exparg_dn11: f64 = *var_fn61_calc_iq__exparg_dn11_slot;
        let mut var_fn61_calc_iq__exparg_dn12: f64 = *var_fn61_calc_iq__exparg_dn12_slot;
        let mut var_fn61_calc_iq__exparg_dn13: f64 = *var_fn61_calc_iq__exparg_dn13_slot;
        let mut var_fn61_calc_iq__exparg_dn14: f64 = *var_fn61_calc_iq__exparg_dn14_slot;
        let mut var_fn61_calc_iq__exparg_dn15: f64 = *var_fn61_calc_iq__exparg_dn15_slot;
        let mut var_fn61_calc_iq__exparg_dn16: f64 = *var_fn61_calc_iq__exparg_dn16_slot;
        let mut var_fn61_calc_iq__exparg_dn17: f64 = *var_fn61_calc_iq__exparg_dn17_slot;
        let mut var_fn61_calc_iq__exparg_dn18: f64 = *var_fn61_calc_iq__exparg_dn18_slot;
        let mut var_fn61_calc_iq__exparg_dn19: f64 = *var_fn61_calc_iq__exparg_dn19_slot;
        let mut var_fn61_calc_iq__exparg_dn2: f64 = *var_fn61_calc_iq__exparg_dn2_slot;
        let mut var_fn61_calc_iq__exparg_dn20: f64 = *var_fn61_calc_iq__exparg_dn20_slot;
        let mut var_fn61_calc_iq__exparg_dn21: f64 = *var_fn61_calc_iq__exparg_dn21_slot;
        let mut var_fn61_calc_iq__exparg_dn22: f64 = *var_fn61_calc_iq__exparg_dn22_slot;
        let mut var_fn61_calc_iq__exparg_dn23: f64 = *var_fn61_calc_iq__exparg_dn23_slot;
        let mut var_fn61_calc_iq__exparg_dn24: f64 = *var_fn61_calc_iq__exparg_dn24_slot;
        let mut var_fn61_calc_iq__exparg_dn25: f64 = *var_fn61_calc_iq__exparg_dn25_slot;
        let mut var_fn61_calc_iq__exparg_dn26: f64 = *var_fn61_calc_iq__exparg_dn26_slot;
        let mut var_fn61_calc_iq__exparg_dn27: f64 = *var_fn61_calc_iq__exparg_dn27_slot;
        let mut var_fn61_calc_iq__exparg_dn28: f64 = *var_fn61_calc_iq__exparg_dn28_slot;
        let mut var_fn61_calc_iq__exparg_dn29: f64 = *var_fn61_calc_iq__exparg_dn29_slot;
        let mut var_fn61_calc_iq__exparg_dn3: f64 = *var_fn61_calc_iq__exparg_dn3_slot;
        let mut var_fn61_calc_iq__exparg_dn4: f64 = *var_fn61_calc_iq__exparg_dn4_slot;
        let mut var_fn61_calc_iq__exparg_dn5: f64 = *var_fn61_calc_iq__exparg_dn5_slot;
        let mut var_fn61_calc_iq__exparg_dn6: f64 = *var_fn61_calc_iq__exparg_dn6_slot;
        let mut var_fn61_calc_iq__exparg_dn7: f64 = *var_fn61_calc_iq__exparg_dn7_slot;
        let mut var_fn61_calc_iq__exparg_dn8: f64 = *var_fn61_calc_iq__exparg_dn8_slot;
        let mut var_fn61_calc_iq__exparg_dn9: f64 = *var_fn61_calc_iq__exparg_dn9_slot;
        let mut var_fn61_calc_iq__qbout: f64 = *var_fn61_calc_iq__qbout_slot;
        let mut var_fn61_calc_iq__qbout_db0: f64 = *var_fn61_calc_iq__qbout_db0_slot;
        let mut var_fn61_calc_iq__qbout_db1: f64 = *var_fn61_calc_iq__qbout_db1_slot;
        let mut var_fn61_calc_iq__qbout_db10: f64 = *var_fn61_calc_iq__qbout_db10_slot;
        let mut var_fn61_calc_iq__qbout_db11: f64 = *var_fn61_calc_iq__qbout_db11_slot;
        let mut var_fn61_calc_iq__qbout_db12: f64 = *var_fn61_calc_iq__qbout_db12_slot;
        let mut var_fn61_calc_iq__qbout_db13: f64 = *var_fn61_calc_iq__qbout_db13_slot;
        let mut var_fn61_calc_iq__qbout_db14: f64 = *var_fn61_calc_iq__qbout_db14_slot;
        let mut var_fn61_calc_iq__qbout_db15: f64 = *var_fn61_calc_iq__qbout_db15_slot;
        let mut var_fn61_calc_iq__qbout_db16: f64 = *var_fn61_calc_iq__qbout_db16_slot;
        let mut var_fn61_calc_iq__qbout_db17: f64 = *var_fn61_calc_iq__qbout_db17_slot;
        let mut var_fn61_calc_iq__qbout_db18: f64 = *var_fn61_calc_iq__qbout_db18_slot;
        let mut var_fn61_calc_iq__qbout_db19: f64 = *var_fn61_calc_iq__qbout_db19_slot;
        let mut var_fn61_calc_iq__qbout_db2: f64 = *var_fn61_calc_iq__qbout_db2_slot;
        let mut var_fn61_calc_iq__qbout_db20: f64 = *var_fn61_calc_iq__qbout_db20_slot;
        let mut var_fn61_calc_iq__qbout_db21: f64 = *var_fn61_calc_iq__qbout_db21_slot;
        let mut var_fn61_calc_iq__qbout_db22: f64 = *var_fn61_calc_iq__qbout_db22_slot;
        let mut var_fn61_calc_iq__qbout_db23: f64 = *var_fn61_calc_iq__qbout_db23_slot;
        let mut var_fn61_calc_iq__qbout_db24: f64 = *var_fn61_calc_iq__qbout_db24_slot;
        let mut var_fn61_calc_iq__qbout_db25: f64 = *var_fn61_calc_iq__qbout_db25_slot;
        let mut var_fn61_calc_iq__qbout_db26: f64 = *var_fn61_calc_iq__qbout_db26_slot;
        let mut var_fn61_calc_iq__qbout_db27: f64 = *var_fn61_calc_iq__qbout_db27_slot;
        let mut var_fn61_calc_iq__qbout_db28: f64 = *var_fn61_calc_iq__qbout_db28_slot;
        let mut var_fn61_calc_iq__qbout_db29: f64 = *var_fn61_calc_iq__qbout_db29_slot;
        let mut var_fn61_calc_iq__qbout_db3: f64 = *var_fn61_calc_iq__qbout_db3_slot;
        let mut var_fn61_calc_iq__qbout_db30: f64 = *var_fn61_calc_iq__qbout_db30_slot;
        let mut var_fn61_calc_iq__qbout_db31: f64 = *var_fn61_calc_iq__qbout_db31_slot;
        let mut var_fn61_calc_iq__qbout_db32: f64 = *var_fn61_calc_iq__qbout_db32_slot;
        let mut var_fn61_calc_iq__qbout_db33: f64 = *var_fn61_calc_iq__qbout_db33_slot;
        let mut var_fn61_calc_iq__qbout_db34: f64 = *var_fn61_calc_iq__qbout_db34_slot;
        let mut var_fn61_calc_iq__qbout_db35: f64 = *var_fn61_calc_iq__qbout_db35_slot;
        let mut var_fn61_calc_iq__qbout_db4: f64 = *var_fn61_calc_iq__qbout_db4_slot;
        let mut var_fn61_calc_iq__qbout_db5: f64 = *var_fn61_calc_iq__qbout_db5_slot;
        let mut var_fn61_calc_iq__qbout_db6: f64 = *var_fn61_calc_iq__qbout_db6_slot;
        let mut var_fn61_calc_iq__qbout_db7: f64 = *var_fn61_calc_iq__qbout_db7_slot;
        let mut var_fn61_calc_iq__qbout_db8: f64 = *var_fn61_calc_iq__qbout_db8_slot;
        let mut var_fn61_calc_iq__qbout_db9: f64 = *var_fn61_calc_iq__qbout_db9_slot;
        let mut var_fn61_calc_iq__qbout_dn0: f64 = *var_fn61_calc_iq__qbout_dn0_slot;
        let mut var_fn61_calc_iq__qbout_dn1: f64 = *var_fn61_calc_iq__qbout_dn1_slot;
        let mut var_fn61_calc_iq__qbout_dn10: f64 = *var_fn61_calc_iq__qbout_dn10_slot;
        let mut var_fn61_calc_iq__qbout_dn11: f64 = *var_fn61_calc_iq__qbout_dn11_slot;
        let mut var_fn61_calc_iq__qbout_dn12: f64 = *var_fn61_calc_iq__qbout_dn12_slot;
        let mut var_fn61_calc_iq__qbout_dn13: f64 = *var_fn61_calc_iq__qbout_dn13_slot;
        let mut var_fn61_calc_iq__qbout_dn14: f64 = *var_fn61_calc_iq__qbout_dn14_slot;
        let mut var_fn61_calc_iq__qbout_dn15: f64 = *var_fn61_calc_iq__qbout_dn15_slot;
        let mut var_fn61_calc_iq__qbout_dn16: f64 = *var_fn61_calc_iq__qbout_dn16_slot;
        let mut var_fn61_calc_iq__qbout_dn17: f64 = *var_fn61_calc_iq__qbout_dn17_slot;
        let mut var_fn61_calc_iq__qbout_dn18: f64 = *var_fn61_calc_iq__qbout_dn18_slot;
        let mut var_fn61_calc_iq__qbout_dn19: f64 = *var_fn61_calc_iq__qbout_dn19_slot;
        let mut var_fn61_calc_iq__qbout_dn2: f64 = *var_fn61_calc_iq__qbout_dn2_slot;
        let mut var_fn61_calc_iq__qbout_dn20: f64 = *var_fn61_calc_iq__qbout_dn20_slot;
        let mut var_fn61_calc_iq__qbout_dn21: f64 = *var_fn61_calc_iq__qbout_dn21_slot;
        let mut var_fn61_calc_iq__qbout_dn22: f64 = *var_fn61_calc_iq__qbout_dn22_slot;
        let mut var_fn61_calc_iq__qbout_dn23: f64 = *var_fn61_calc_iq__qbout_dn23_slot;
        let mut var_fn61_calc_iq__qbout_dn24: f64 = *var_fn61_calc_iq__qbout_dn24_slot;
        let mut var_fn61_calc_iq__qbout_dn25: f64 = *var_fn61_calc_iq__qbout_dn25_slot;
        let mut var_fn61_calc_iq__qbout_dn26: f64 = *var_fn61_calc_iq__qbout_dn26_slot;
        let mut var_fn61_calc_iq__qbout_dn27: f64 = *var_fn61_calc_iq__qbout_dn27_slot;
        let mut var_fn61_calc_iq__qbout_dn28: f64 = *var_fn61_calc_iq__qbout_dn28_slot;
        let mut var_fn61_calc_iq__qbout_dn29: f64 = *var_fn61_calc_iq__qbout_dn29_slot;
        let mut var_fn61_calc_iq__qbout_dn3: f64 = *var_fn61_calc_iq__qbout_dn3_slot;
        let mut var_fn61_calc_iq__qbout_dn4: f64 = *var_fn61_calc_iq__qbout_dn4_slot;
        let mut var_fn61_calc_iq__qbout_dn5: f64 = *var_fn61_calc_iq__qbout_dn5_slot;
        let mut var_fn61_calc_iq__qbout_dn6: f64 = *var_fn61_calc_iq__qbout_dn6_slot;
        let mut var_fn61_calc_iq__qbout_dn7: f64 = *var_fn61_calc_iq__qbout_dn7_slot;
        let mut var_fn61_calc_iq__qbout_dn8: f64 = *var_fn61_calc_iq__qbout_dn8_slot;
        let mut var_fn61_calc_iq__qbout_dn9: f64 = *var_fn61_calc_iq__qbout_dn9_slot;
        let mut var_fn61_calc_iq__qcout: f64 = *var_fn61_calc_iq__qcout_slot;
        let mut var_fn61_calc_iq__qcout_db0: f64 = *var_fn61_calc_iq__qcout_db0_slot;
        let mut var_fn61_calc_iq__qcout_db1: f64 = *var_fn61_calc_iq__qcout_db1_slot;
        let mut var_fn61_calc_iq__qcout_db10: f64 = *var_fn61_calc_iq__qcout_db10_slot;
        let mut var_fn61_calc_iq__qcout_db11: f64 = *var_fn61_calc_iq__qcout_db11_slot;
        let mut var_fn61_calc_iq__qcout_db12: f64 = *var_fn61_calc_iq__qcout_db12_slot;
        let mut var_fn61_calc_iq__qcout_db13: f64 = *var_fn61_calc_iq__qcout_db13_slot;
        let mut var_fn61_calc_iq__qcout_db14: f64 = *var_fn61_calc_iq__qcout_db14_slot;
        let mut var_fn61_calc_iq__qcout_db15: f64 = *var_fn61_calc_iq__qcout_db15_slot;
        let mut var_fn61_calc_iq__qcout_db16: f64 = *var_fn61_calc_iq__qcout_db16_slot;
        let mut var_fn61_calc_iq__qcout_db17: f64 = *var_fn61_calc_iq__qcout_db17_slot;
        let mut var_fn61_calc_iq__qcout_db18: f64 = *var_fn61_calc_iq__qcout_db18_slot;
        let mut var_fn61_calc_iq__qcout_db19: f64 = *var_fn61_calc_iq__qcout_db19_slot;
        let mut var_fn61_calc_iq__qcout_db2: f64 = *var_fn61_calc_iq__qcout_db2_slot;
        let mut var_fn61_calc_iq__qcout_db20: f64 = *var_fn61_calc_iq__qcout_db20_slot;
        let mut var_fn61_calc_iq__qcout_db21: f64 = *var_fn61_calc_iq__qcout_db21_slot;
        let mut var_fn61_calc_iq__qcout_db22: f64 = *var_fn61_calc_iq__qcout_db22_slot;
        let mut var_fn61_calc_iq__qcout_db23: f64 = *var_fn61_calc_iq__qcout_db23_slot;
        let mut var_fn61_calc_iq__qcout_db24: f64 = *var_fn61_calc_iq__qcout_db24_slot;
        let mut var_fn61_calc_iq__qcout_db25: f64 = *var_fn61_calc_iq__qcout_db25_slot;
        let mut var_fn61_calc_iq__qcout_db26: f64 = *var_fn61_calc_iq__qcout_db26_slot;
        let mut var_fn61_calc_iq__qcout_db27: f64 = *var_fn61_calc_iq__qcout_db27_slot;
        let mut var_fn61_calc_iq__qcout_db28: f64 = *var_fn61_calc_iq__qcout_db28_slot;
        let mut var_fn61_calc_iq__qcout_db29: f64 = *var_fn61_calc_iq__qcout_db29_slot;
        let mut var_fn61_calc_iq__qcout_db3: f64 = *var_fn61_calc_iq__qcout_db3_slot;
        let mut var_fn61_calc_iq__qcout_db30: f64 = *var_fn61_calc_iq__qcout_db30_slot;
        let mut var_fn61_calc_iq__qcout_db31: f64 = *var_fn61_calc_iq__qcout_db31_slot;
        let mut var_fn61_calc_iq__qcout_db32: f64 = *var_fn61_calc_iq__qcout_db32_slot;
        let mut var_fn61_calc_iq__qcout_db33: f64 = *var_fn61_calc_iq__qcout_db33_slot;
        let mut var_fn61_calc_iq__qcout_db34: f64 = *var_fn61_calc_iq__qcout_db34_slot;
        let mut var_fn61_calc_iq__qcout_db35: f64 = *var_fn61_calc_iq__qcout_db35_slot;
        let mut var_fn61_calc_iq__qcout_db4: f64 = *var_fn61_calc_iq__qcout_db4_slot;
        let mut var_fn61_calc_iq__qcout_db5: f64 = *var_fn61_calc_iq__qcout_db5_slot;
        let mut var_fn61_calc_iq__qcout_db6: f64 = *var_fn61_calc_iq__qcout_db6_slot;
        let mut var_fn61_calc_iq__qcout_db7: f64 = *var_fn61_calc_iq__qcout_db7_slot;
        let mut var_fn61_calc_iq__qcout_db8: f64 = *var_fn61_calc_iq__qcout_db8_slot;
        let mut var_fn61_calc_iq__qcout_db9: f64 = *var_fn61_calc_iq__qcout_db9_slot;
        let mut var_fn61_calc_iq__qcout_dn0: f64 = *var_fn61_calc_iq__qcout_dn0_slot;
        let mut var_fn61_calc_iq__qcout_dn1: f64 = *var_fn61_calc_iq__qcout_dn1_slot;
        let mut var_fn61_calc_iq__qcout_dn10: f64 = *var_fn61_calc_iq__qcout_dn10_slot;
        let mut var_fn61_calc_iq__qcout_dn11: f64 = *var_fn61_calc_iq__qcout_dn11_slot;
        let mut var_fn61_calc_iq__qcout_dn12: f64 = *var_fn61_calc_iq__qcout_dn12_slot;
        let mut var_fn61_calc_iq__qcout_dn13: f64 = *var_fn61_calc_iq__qcout_dn13_slot;
        let mut var_fn61_calc_iq__qcout_dn14: f64 = *var_fn61_calc_iq__qcout_dn14_slot;
        let mut var_fn61_calc_iq__qcout_dn15: f64 = *var_fn61_calc_iq__qcout_dn15_slot;
        let mut var_fn61_calc_iq__qcout_dn16: f64 = *var_fn61_calc_iq__qcout_dn16_slot;
        let mut var_fn61_calc_iq__qcout_dn17: f64 = *var_fn61_calc_iq__qcout_dn17_slot;
        let mut var_fn61_calc_iq__qcout_dn18: f64 = *var_fn61_calc_iq__qcout_dn18_slot;
        let mut var_fn61_calc_iq__qcout_dn19: f64 = *var_fn61_calc_iq__qcout_dn19_slot;
        let mut var_fn61_calc_iq__qcout_dn2: f64 = *var_fn61_calc_iq__qcout_dn2_slot;
        let mut var_fn61_calc_iq__qcout_dn20: f64 = *var_fn61_calc_iq__qcout_dn20_slot;
        let mut var_fn61_calc_iq__qcout_dn21: f64 = *var_fn61_calc_iq__qcout_dn21_slot;
        let mut var_fn61_calc_iq__qcout_dn22: f64 = *var_fn61_calc_iq__qcout_dn22_slot;
        let mut var_fn61_calc_iq__qcout_dn23: f64 = *var_fn61_calc_iq__qcout_dn23_slot;
        let mut var_fn61_calc_iq__qcout_dn24: f64 = *var_fn61_calc_iq__qcout_dn24_slot;
        let mut var_fn61_calc_iq__qcout_dn25: f64 = *var_fn61_calc_iq__qcout_dn25_slot;
        let mut var_fn61_calc_iq__qcout_dn26: f64 = *var_fn61_calc_iq__qcout_dn26_slot;
        let mut var_fn61_calc_iq__qcout_dn27: f64 = *var_fn61_calc_iq__qcout_dn27_slot;
        let mut var_fn61_calc_iq__qcout_dn28: f64 = *var_fn61_calc_iq__qcout_dn28_slot;
        let mut var_fn61_calc_iq__qcout_dn29: f64 = *var_fn61_calc_iq__qcout_dn29_slot;
        let mut var_fn61_calc_iq__qcout_dn3: f64 = *var_fn61_calc_iq__qcout_dn3_slot;
        let mut var_fn61_calc_iq__qcout_dn4: f64 = *var_fn61_calc_iq__qcout_dn4_slot;
        let mut var_fn61_calc_iq__qcout_dn5: f64 = *var_fn61_calc_iq__qcout_dn5_slot;
        let mut var_fn61_calc_iq__qcout_dn6: f64 = *var_fn61_calc_iq__qcout_dn6_slot;
        let mut var_fn61_calc_iq__qcout_dn7: f64 = *var_fn61_calc_iq__qcout_dn7_slot;
        let mut var_fn61_calc_iq__qcout_dn8: f64 = *var_fn61_calc_iq__qcout_dn8_slot;
        let mut var_fn61_calc_iq__qcout_dn9: f64 = *var_fn61_calc_iq__qcout_dn9_slot;
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_guard93: f64 = *var_guard93_slot;

        let (assign7100_e8544, assign7100_e8544_d_n0, assign7100_e8544_d_n1, assign7100_e8544_d_n2, assign7100_e8544_d_n3, assign7100_e8544_d_n4, assign7100_e8544_d_n5, assign7100_e8544_d_n6, assign7100_e8544_d_n7, assign7100_e8544_d_n8, assign7100_e8544_d_n9, assign7100_e8544_d_n10, assign7100_e8544_d_n11, assign7100_e8544_d_n12, assign7100_e8544_d_n13, assign7100_e8544_d_n14, assign7100_e8544_d_n15, assign7100_e8544_d_n16, assign7100_e8544_d_n17, assign7100_e8544_d_n18, assign7100_e8544_d_n19, assign7100_e8544_d_n20, assign7100_e8544_d_n21, assign7100_e8544_d_n22, assign7100_e8544_d_n23, assign7100_e8544_d_n24, assign7100_e8544_d_n25, assign7100_e8544_d_n26, assign7100_e8544_d_n27, assign7100_e8544_d_n28, assign7100_e8544_d_n29, assign7100_e8544_d_b0, assign7100_e8544_d_b1, assign7100_e8544_d_b2, assign7100_e8544_d_b3, assign7100_e8544_d_b4, assign7100_e8544_d_b5, assign7100_e8544_d_b6, assign7100_e8544_d_b7, assign7100_e8544_d_b8, assign7100_e8544_d_b9, assign7100_e8544_d_b10, assign7100_e8544_d_b11, assign7100_e8544_d_b12, assign7100_e8544_d_b13, assign7100_e8544_d_b14, assign7100_e8544_d_b15, assign7100_e8544_d_b16, assign7100_e8544_d_b17, assign7100_e8544_d_b18, assign7100_e8544_d_b19, assign7100_e8544_d_b20, assign7100_e8544_d_b21, assign7100_e8544_d_b22, assign7100_e8544_d_b23, assign7100_e8544_d_b24, assign7100_e8544_d_b25, assign7100_e8544_d_b26, assign7100_e8544_d_b27, assign7100_e8544_d_b28, assign7100_e8544_d_b29, assign7100_e8544_d_b30, assign7100_e8544_d_b31, assign7100_e8544_d_b32, assign7100_e8544_d_b33, assign7100_e8544_d_b34, assign7100_e8544_d_b35,) = {
    if ((((var_guard60 != 0.0) && (var_guard87 != 0.0)) && (var_guard90 == 0.0)) && (var_guard91 != 0.0)) {
        let assign7100_e8542: f64 = (var_fn61_calc_iq__etab).exp();
        (assign7100_e8542, (assign7100_e8542 * var_fn61_calc_iq__etab_dn0), (assign7100_e8542 * var_fn61_calc_iq__etab_dn1), (assign7100_e8542 * var_fn61_calc_iq__etab_dn2), (assign7100_e8542 * var_fn61_calc_iq__etab_dn3), (assign7100_e8542 * var_fn61_calc_iq__etab_dn4), (assign7100_e8542 * var_fn61_calc_iq__etab_dn5), (assign7100_e8542 * var_fn61_calc_iq__etab_dn6), (assign7100_e8542 * var_fn61_calc_iq__etab_dn7), (assign7100_e8542 * var_fn61_calc_iq__etab_dn8), (assign7100_e8542 * var_fn61_calc_iq__etab_dn9), (assign7100_e8542 * var_fn61_calc_iq__etab_dn10), (assign7100_e8542 * var_fn61_calc_iq__etab_dn11), (assign7100_e8542 * var_fn61_calc_iq__etab_dn12), (assign7100_e8542 * var_fn61_calc_iq__etab_dn13), (assign7100_e8542 * var_fn61_calc_iq__etab_dn14), (assign7100_e8542 * var_fn61_calc_iq__etab_dn15), (assign7100_e8542 * var_fn61_calc_iq__etab_dn16), (assign7100_e8542 * var_fn61_calc_iq__etab_dn17), (assign7100_e8542 * var_fn61_calc_iq__etab_dn18), (assign7100_e8542 * var_fn61_calc_iq__etab_dn19), (assign7100_e8542 * var_fn61_calc_iq__etab_dn20), (assign7100_e8542 * var_fn61_calc_iq__etab_dn21), (assign7100_e8542 * var_fn61_calc_iq__etab_dn22), (assign7100_e8542 * var_fn61_calc_iq__etab_dn23), (assign7100_e8542 * var_fn61_calc_iq__etab_dn24), (assign7100_e8542 * var_fn61_calc_iq__etab_dn25), (assign7100_e8542 * var_fn61_calc_iq__etab_dn26), (assign7100_e8542 * var_fn61_calc_iq__etab_dn27), (assign7100_e8542 * var_fn61_calc_iq__etab_dn28), (assign7100_e8542 * var_fn61_calc_iq__etab_dn29), (assign7100_e8542 * var_fn61_calc_iq__etab_db0), (assign7100_e8542 * var_fn61_calc_iq__etab_db1), (assign7100_e8542 * var_fn61_calc_iq__etab_db2), (assign7100_e8542 * var_fn61_calc_iq__etab_db3), (assign7100_e8542 * var_fn61_calc_iq__etab_db4), (assign7100_e8542 * var_fn61_calc_iq__etab_db5), (assign7100_e8542 * var_fn61_calc_iq__etab_db6), (assign7100_e8542 * var_fn61_calc_iq__etab_db7), (assign7100_e8542 * var_fn61_calc_iq__etab_db8), (assign7100_e8542 * var_fn61_calc_iq__etab_db9), (assign7100_e8542 * var_fn61_calc_iq__etab_db10), (assign7100_e8542 * var_fn61_calc_iq__etab_db11), (assign7100_e8542 * var_fn61_calc_iq__etab_db12), (assign7100_e8542 * var_fn61_calc_iq__etab_db13), (assign7100_e8542 * var_fn61_calc_iq__etab_db14), (assign7100_e8542 * var_fn61_calc_iq__etab_db15), (assign7100_e8542 * var_fn61_calc_iq__etab_db16), (assign7100_e8542 * var_fn61_calc_iq__etab_db17), (assign7100_e8542 * var_fn61_calc_iq__etab_db18), (assign7100_e8542 * var_fn61_calc_iq__etab_db19), (assign7100_e8542 * var_fn61_calc_iq__etab_db20), (assign7100_e8542 * var_fn61_calc_iq__etab_db21), (assign7100_e8542 * var_fn61_calc_iq__etab_db22), (assign7100_e8542 * var_fn61_calc_iq__etab_db23), (assign7100_e8542 * var_fn61_calc_iq__etab_db24), (assign7100_e8542 * var_fn61_calc_iq__etab_db25), (assign7100_e8542 * var_fn61_calc_iq__etab_db26), (assign7100_e8542 * var_fn61_calc_iq__etab_db27), (assign7100_e8542 * var_fn61_calc_iq__etab_db28), (assign7100_e8542 * var_fn61_calc_iq__etab_db29), (assign7100_e8542 * var_fn61_calc_iq__etab_db30), (assign7100_e8542 * var_fn61_calc_iq__etab_db31), (assign7100_e8542 * var_fn61_calc_iq__etab_db32), (assign7100_e8542 * var_fn61_calc_iq__etab_db33), (assign7100_e8542 * var_fn61_calc_iq__etab_db34), (assign7100_e8542 * var_fn61_calc_iq__etab_db35),)
    } else {
        (var_fn61_calc_iq__exparg, var_fn61_calc_iq__exparg_dn0, var_fn61_calc_iq__exparg_dn1, var_fn61_calc_iq__exparg_dn2, var_fn61_calc_iq__exparg_dn3, var_fn61_calc_iq__exparg_dn4, var_fn61_calc_iq__exparg_dn5, var_fn61_calc_iq__exparg_dn6, var_fn61_calc_iq__exparg_dn7, var_fn61_calc_iq__exparg_dn8, var_fn61_calc_iq__exparg_dn9, var_fn61_calc_iq__exparg_dn10, var_fn61_calc_iq__exparg_dn11, var_fn61_calc_iq__exparg_dn12, var_fn61_calc_iq__exparg_dn13, var_fn61_calc_iq__exparg_dn14, var_fn61_calc_iq__exparg_dn15, var_fn61_calc_iq__exparg_dn16, var_fn61_calc_iq__exparg_dn17, var_fn61_calc_iq__exparg_dn18, var_fn61_calc_iq__exparg_dn19, var_fn61_calc_iq__exparg_dn20, var_fn61_calc_iq__exparg_dn21, var_fn61_calc_iq__exparg_dn22, var_fn61_calc_iq__exparg_dn23, var_fn61_calc_iq__exparg_dn24, var_fn61_calc_iq__exparg_dn25, var_fn61_calc_iq__exparg_dn26, var_fn61_calc_iq__exparg_dn27, var_fn61_calc_iq__exparg_dn28, var_fn61_calc_iq__exparg_dn29, var_fn61_calc_iq__exparg_db0, var_fn61_calc_iq__exparg_db1, var_fn61_calc_iq__exparg_db2, var_fn61_calc_iq__exparg_db3, var_fn61_calc_iq__exparg_db4, var_fn61_calc_iq__exparg_db5, var_fn61_calc_iq__exparg_db6, var_fn61_calc_iq__exparg_db7, var_fn61_calc_iq__exparg_db8, var_fn61_calc_iq__exparg_db9, var_fn61_calc_iq__exparg_db10, var_fn61_calc_iq__exparg_db11, var_fn61_calc_iq__exparg_db12, var_fn61_calc_iq__exparg_db13, var_fn61_calc_iq__exparg_db14, var_fn61_calc_iq__exparg_db15, var_fn61_calc_iq__exparg_db16, var_fn61_calc_iq__exparg_db17, var_fn61_calc_iq__exparg_db18, var_fn61_calc_iq__exparg_db19, var_fn61_calc_iq__exparg_db20, var_fn61_calc_iq__exparg_db21, var_fn61_calc_iq__exparg_db22, var_fn61_calc_iq__exparg_db23, var_fn61_calc_iq__exparg_db24, var_fn61_calc_iq__exparg_db25, var_fn61_calc_iq__exparg_db26, var_fn61_calc_iq__exparg_db27, var_fn61_calc_iq__exparg_db28, var_fn61_calc_iq__exparg_db29, var_fn61_calc_iq__exparg_db30, var_fn61_calc_iq__exparg_db31, var_fn61_calc_iq__exparg_db32, var_fn61_calc_iq__exparg_db33, var_fn61_calc_iq__exparg_db34, var_fn61_calc_iq__exparg_db35,)
    }
};
        var_fn61_calc_iq__exparg = assign7100_e8544;
        var_fn61_calc_iq__exparg_dn0 = assign7100_e8544_d_n0;
        var_fn61_calc_iq__exparg_dn1 = assign7100_e8544_d_n1;
        var_fn61_calc_iq__exparg_dn2 = assign7100_e8544_d_n2;
        var_fn61_calc_iq__exparg_dn3 = assign7100_e8544_d_n3;
        var_fn61_calc_iq__exparg_dn4 = assign7100_e8544_d_n4;
        var_fn61_calc_iq__exparg_dn5 = assign7100_e8544_d_n5;
        var_fn61_calc_iq__exparg_dn6 = assign7100_e8544_d_n6;
        var_fn61_calc_iq__exparg_dn7 = assign7100_e8544_d_n7;
        var_fn61_calc_iq__exparg_dn8 = assign7100_e8544_d_n8;
        var_fn61_calc_iq__exparg_dn9 = assign7100_e8544_d_n9;
        var_fn61_calc_iq__exparg_dn10 = assign7100_e8544_d_n10;
        var_fn61_calc_iq__exparg_dn11 = assign7100_e8544_d_n11;
        var_fn61_calc_iq__exparg_dn12 = assign7100_e8544_d_n12;
        var_fn61_calc_iq__exparg_dn13 = assign7100_e8544_d_n13;
        var_fn61_calc_iq__exparg_dn14 = assign7100_e8544_d_n14;
        var_fn61_calc_iq__exparg_dn15 = assign7100_e8544_d_n15;
        var_fn61_calc_iq__exparg_dn16 = assign7100_e8544_d_n16;
        var_fn61_calc_iq__exparg_dn17 = assign7100_e8544_d_n17;
        var_fn61_calc_iq__exparg_dn18 = assign7100_e8544_d_n18;
        var_fn61_calc_iq__exparg_dn19 = assign7100_e8544_d_n19;
        var_fn61_calc_iq__exparg_dn20 = assign7100_e8544_d_n20;
        var_fn61_calc_iq__exparg_dn21 = assign7100_e8544_d_n21;
        var_fn61_calc_iq__exparg_dn22 = assign7100_e8544_d_n22;
        var_fn61_calc_iq__exparg_dn23 = assign7100_e8544_d_n23;
        var_fn61_calc_iq__exparg_dn24 = assign7100_e8544_d_n24;
        var_fn61_calc_iq__exparg_dn25 = assign7100_e8544_d_n25;
        var_fn61_calc_iq__exparg_dn26 = assign7100_e8544_d_n26;
        var_fn61_calc_iq__exparg_dn27 = assign7100_e8544_d_n27;
        var_fn61_calc_iq__exparg_dn28 = assign7100_e8544_d_n28;
        var_fn61_calc_iq__exparg_dn29 = assign7100_e8544_d_n29;
        var_fn61_calc_iq__exparg_db0 = assign7100_e8544_d_b0;
        var_fn61_calc_iq__exparg_db1 = assign7100_e8544_d_b1;
        var_fn61_calc_iq__exparg_db2 = assign7100_e8544_d_b2;
        var_fn61_calc_iq__exparg_db3 = assign7100_e8544_d_b3;
        var_fn61_calc_iq__exparg_db4 = assign7100_e8544_d_b4;
        var_fn61_calc_iq__exparg_db5 = assign7100_e8544_d_b5;
        var_fn61_calc_iq__exparg_db6 = assign7100_e8544_d_b6;
        var_fn61_calc_iq__exparg_db7 = assign7100_e8544_d_b7;
        var_fn61_calc_iq__exparg_db8 = assign7100_e8544_d_b8;
        var_fn61_calc_iq__exparg_db9 = assign7100_e8544_d_b9;
        var_fn61_calc_iq__exparg_db10 = assign7100_e8544_d_b10;
        var_fn61_calc_iq__exparg_db11 = assign7100_e8544_d_b11;
        var_fn61_calc_iq__exparg_db12 = assign7100_e8544_d_b12;
        var_fn61_calc_iq__exparg_db13 = assign7100_e8544_d_b13;
        var_fn61_calc_iq__exparg_db14 = assign7100_e8544_d_b14;
        var_fn61_calc_iq__exparg_db15 = assign7100_e8544_d_b15;
        var_fn61_calc_iq__exparg_db16 = assign7100_e8544_d_b16;
        var_fn61_calc_iq__exparg_db17 = assign7100_e8544_d_b17;
        var_fn61_calc_iq__exparg_db18 = assign7100_e8544_d_b18;
        var_fn61_calc_iq__exparg_db19 = assign7100_e8544_d_b19;
        var_fn61_calc_iq__exparg_db20 = assign7100_e8544_d_b20;
        var_fn61_calc_iq__exparg_db21 = assign7100_e8544_d_b21;
        var_fn61_calc_iq__exparg_db22 = assign7100_e8544_d_b22;
        var_fn61_calc_iq__exparg_db23 = assign7100_e8544_d_b23;
        var_fn61_calc_iq__exparg_db24 = assign7100_e8544_d_b24;
        var_fn61_calc_iq__exparg_db25 = assign7100_e8544_d_b25;
        var_fn61_calc_iq__exparg_db26 = assign7100_e8544_d_b26;
        var_fn61_calc_iq__exparg_db27 = assign7100_e8544_d_b27;
        var_fn61_calc_iq__exparg_db28 = assign7100_e8544_d_b28;
        var_fn61_calc_iq__exparg_db29 = assign7100_e8544_d_b29;
        var_fn61_calc_iq__exparg_db30 = assign7100_e8544_d_b30;
        var_fn61_calc_iq__exparg_db31 = assign7100_e8544_d_b31;
        var_fn61_calc_iq__exparg_db32 = assign7100_e8544_d_b32;
        var_fn61_calc_iq__exparg_db33 = assign7100_e8544_d_b33;
        var_fn61_calc_iq__exparg_db34 = assign7100_e8544_d_b34;
        var_fn61_calc_iq__exparg_db35 = assign7100_e8544_d_b35;

        let (assign7110_e8560, assign7110_e8560_d_n0, assign7110_e8560_d_n1, assign7110_e8560_d_n2, assign7110_e8560_d_n3, assign7110_e8560_d_n4, assign7110_e8560_d_n5, assign7110_e8560_d_n6, assign7110_e8560_d_n7, assign7110_e8560_d_n8, assign7110_e8560_d_n9, assign7110_e8560_d_n10, assign7110_e8560_d_n11, assign7110_e8560_d_n12, assign7110_e8560_d_n13, assign7110_e8560_d_n14, assign7110_e8560_d_n15, assign7110_e8560_d_n16, assign7110_e8560_d_n17, assign7110_e8560_d_n18, assign7110_e8560_d_n19, assign7110_e8560_d_n20, assign7110_e8560_d_n21, assign7110_e8560_d_n22, assign7110_e8560_d_n23, assign7110_e8560_d_n24, assign7110_e8560_d_n25, assign7110_e8560_d_n26, assign7110_e8560_d_n27, assign7110_e8560_d_n28, assign7110_e8560_d_n29, assign7110_e8560_d_b0, assign7110_e8560_d_b1, assign7110_e8560_d_b2, assign7110_e8560_d_b3, assign7110_e8560_d_b4, assign7110_e8560_d_b5, assign7110_e8560_d_b6, assign7110_e8560_d_b7, assign7110_e8560_d_b8, assign7110_e8560_d_b9, assign7110_e8560_d_b10, assign7110_e8560_d_b11, assign7110_e8560_d_b12, assign7110_e8560_d_b13, assign7110_e8560_d_b14, assign7110_e8560_d_b15, assign7110_e8560_d_b16, assign7110_e8560_d_b17, assign7110_e8560_d_b18, assign7110_e8560_d_b19, assign7110_e8560_d_b20, assign7110_e8560_d_b21, assign7110_e8560_d_b22, assign7110_e8560_d_b23, assign7110_e8560_d_b24, assign7110_e8560_d_b25, assign7110_e8560_d_b26, assign7110_e8560_d_b27, assign7110_e8560_d_b28, assign7110_e8560_d_b29, assign7110_e8560_d_b30, assign7110_e8560_d_b31, assign7110_e8560_d_b32, assign7110_e8560_d_b33, assign7110_e8560_d_b34, assign7110_e8560_d_b35,) = {
    if ((((var_guard60 != 0.0) && (var_guard87 != 0.0)) && (var_guard90 == 0.0)) && (var_guard91 == 0.0)) {
        let assign7110_e8556: f64 = (var_fn61_calc_iq__etab).exp();
        let assign7110_e8557: f64 = (1.0 + assign7110_e8556);
        let assign7110_e8558: f64 = (assign7110_e8557).ln();
        (assign7110_e8558, ((assign7110_e8556 * var_fn61_calc_iq__etab_dn0) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn1) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn2) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn3) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn4) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn5) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn6) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn7) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn8) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn9) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn10) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn11) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn12) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn13) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn14) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn15) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn16) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn17) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn18) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn19) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn20) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn21) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn22) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn23) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn24) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn25) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn26) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn27) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn28) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_dn29) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db0) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db1) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db2) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db3) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db4) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db5) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db6) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db7) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db8) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db9) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db10) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db11) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db12) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db13) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db14) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db15) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db16) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db17) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db18) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db19) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db20) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db21) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db22) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db23) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db24) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db25) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db26) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db27) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db28) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db29) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db30) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db31) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db32) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db33) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db34) / assign7110_e8557), ((assign7110_e8556 * var_fn61_calc_iq__etab_db35) / assign7110_e8557),)
    } else {
        (var_fn61_calc_iq__exparg, var_fn61_calc_iq__exparg_dn0, var_fn61_calc_iq__exparg_dn1, var_fn61_calc_iq__exparg_dn2, var_fn61_calc_iq__exparg_dn3, var_fn61_calc_iq__exparg_dn4, var_fn61_calc_iq__exparg_dn5, var_fn61_calc_iq__exparg_dn6, var_fn61_calc_iq__exparg_dn7, var_fn61_calc_iq__exparg_dn8, var_fn61_calc_iq__exparg_dn9, var_fn61_calc_iq__exparg_dn10, var_fn61_calc_iq__exparg_dn11, var_fn61_calc_iq__exparg_dn12, var_fn61_calc_iq__exparg_dn13, var_fn61_calc_iq__exparg_dn14, var_fn61_calc_iq__exparg_dn15, var_fn61_calc_iq__exparg_dn16, var_fn61_calc_iq__exparg_dn17, var_fn61_calc_iq__exparg_dn18, var_fn61_calc_iq__exparg_dn19, var_fn61_calc_iq__exparg_dn20, var_fn61_calc_iq__exparg_dn21, var_fn61_calc_iq__exparg_dn22, var_fn61_calc_iq__exparg_dn23, var_fn61_calc_iq__exparg_dn24, var_fn61_calc_iq__exparg_dn25, var_fn61_calc_iq__exparg_dn26, var_fn61_calc_iq__exparg_dn27, var_fn61_calc_iq__exparg_dn28, var_fn61_calc_iq__exparg_dn29, var_fn61_calc_iq__exparg_db0, var_fn61_calc_iq__exparg_db1, var_fn61_calc_iq__exparg_db2, var_fn61_calc_iq__exparg_db3, var_fn61_calc_iq__exparg_db4, var_fn61_calc_iq__exparg_db5, var_fn61_calc_iq__exparg_db6, var_fn61_calc_iq__exparg_db7, var_fn61_calc_iq__exparg_db8, var_fn61_calc_iq__exparg_db9, var_fn61_calc_iq__exparg_db10, var_fn61_calc_iq__exparg_db11, var_fn61_calc_iq__exparg_db12, var_fn61_calc_iq__exparg_db13, var_fn61_calc_iq__exparg_db14, var_fn61_calc_iq__exparg_db15, var_fn61_calc_iq__exparg_db16, var_fn61_calc_iq__exparg_db17, var_fn61_calc_iq__exparg_db18, var_fn61_calc_iq__exparg_db19, var_fn61_calc_iq__exparg_db20, var_fn61_calc_iq__exparg_db21, var_fn61_calc_iq__exparg_db22, var_fn61_calc_iq__exparg_db23, var_fn61_calc_iq__exparg_db24, var_fn61_calc_iq__exparg_db25, var_fn61_calc_iq__exparg_db26, var_fn61_calc_iq__exparg_db27, var_fn61_calc_iq__exparg_db28, var_fn61_calc_iq__exparg_db29, var_fn61_calc_iq__exparg_db30, var_fn61_calc_iq__exparg_db31, var_fn61_calc_iq__exparg_db32, var_fn61_calc_iq__exparg_db33, var_fn61_calc_iq__exparg_db34, var_fn61_calc_iq__exparg_db35,)
    }
};
        var_fn61_calc_iq__exparg = assign7110_e8560;
        var_fn61_calc_iq__exparg_dn0 = assign7110_e8560_d_n0;
        var_fn61_calc_iq__exparg_dn1 = assign7110_e8560_d_n1;
        var_fn61_calc_iq__exparg_dn2 = assign7110_e8560_d_n2;
        var_fn61_calc_iq__exparg_dn3 = assign7110_e8560_d_n3;
        var_fn61_calc_iq__exparg_dn4 = assign7110_e8560_d_n4;
        var_fn61_calc_iq__exparg_dn5 = assign7110_e8560_d_n5;
        var_fn61_calc_iq__exparg_dn6 = assign7110_e8560_d_n6;
        var_fn61_calc_iq__exparg_dn7 = assign7110_e8560_d_n7;
        var_fn61_calc_iq__exparg_dn8 = assign7110_e8560_d_n8;
        var_fn61_calc_iq__exparg_dn9 = assign7110_e8560_d_n9;
        var_fn61_calc_iq__exparg_dn10 = assign7110_e8560_d_n10;
        var_fn61_calc_iq__exparg_dn11 = assign7110_e8560_d_n11;
        var_fn61_calc_iq__exparg_dn12 = assign7110_e8560_d_n12;
        var_fn61_calc_iq__exparg_dn13 = assign7110_e8560_d_n13;
        var_fn61_calc_iq__exparg_dn14 = assign7110_e8560_d_n14;
        var_fn61_calc_iq__exparg_dn15 = assign7110_e8560_d_n15;
        var_fn61_calc_iq__exparg_dn16 = assign7110_e8560_d_n16;
        var_fn61_calc_iq__exparg_dn17 = assign7110_e8560_d_n17;
        var_fn61_calc_iq__exparg_dn18 = assign7110_e8560_d_n18;
        var_fn61_calc_iq__exparg_dn19 = assign7110_e8560_d_n19;
        var_fn61_calc_iq__exparg_dn20 = assign7110_e8560_d_n20;
        var_fn61_calc_iq__exparg_dn21 = assign7110_e8560_d_n21;
        var_fn61_calc_iq__exparg_dn22 = assign7110_e8560_d_n22;
        var_fn61_calc_iq__exparg_dn23 = assign7110_e8560_d_n23;
        var_fn61_calc_iq__exparg_dn24 = assign7110_e8560_d_n24;
        var_fn61_calc_iq__exparg_dn25 = assign7110_e8560_d_n25;
        var_fn61_calc_iq__exparg_dn26 = assign7110_e8560_d_n26;
        var_fn61_calc_iq__exparg_dn27 = assign7110_e8560_d_n27;
        var_fn61_calc_iq__exparg_dn28 = assign7110_e8560_d_n28;
        var_fn61_calc_iq__exparg_dn29 = assign7110_e8560_d_n29;
        var_fn61_calc_iq__exparg_db0 = assign7110_e8560_d_b0;
        var_fn61_calc_iq__exparg_db1 = assign7110_e8560_d_b1;
        var_fn61_calc_iq__exparg_db2 = assign7110_e8560_d_b2;
        var_fn61_calc_iq__exparg_db3 = assign7110_e8560_d_b3;
        var_fn61_calc_iq__exparg_db4 = assign7110_e8560_d_b4;
        var_fn61_calc_iq__exparg_db5 = assign7110_e8560_d_b5;
        var_fn61_calc_iq__exparg_db6 = assign7110_e8560_d_b6;
        var_fn61_calc_iq__exparg_db7 = assign7110_e8560_d_b7;
        var_fn61_calc_iq__exparg_db8 = assign7110_e8560_d_b8;
        var_fn61_calc_iq__exparg_db9 = assign7110_e8560_d_b9;
        var_fn61_calc_iq__exparg_db10 = assign7110_e8560_d_b10;
        var_fn61_calc_iq__exparg_db11 = assign7110_e8560_d_b11;
        var_fn61_calc_iq__exparg_db12 = assign7110_e8560_d_b12;
        var_fn61_calc_iq__exparg_db13 = assign7110_e8560_d_b13;
        var_fn61_calc_iq__exparg_db14 = assign7110_e8560_d_b14;
        var_fn61_calc_iq__exparg_db15 = assign7110_e8560_d_b15;
        var_fn61_calc_iq__exparg_db16 = assign7110_e8560_d_b16;
        var_fn61_calc_iq__exparg_db17 = assign7110_e8560_d_b17;
        var_fn61_calc_iq__exparg_db18 = assign7110_e8560_d_b18;
        var_fn61_calc_iq__exparg_db19 = assign7110_e8560_d_b19;
        var_fn61_calc_iq__exparg_db20 = assign7110_e8560_d_b20;
        var_fn61_calc_iq__exparg_db21 = assign7110_e8560_d_b21;
        var_fn61_calc_iq__exparg_db22 = assign7110_e8560_d_b22;
        var_fn61_calc_iq__exparg_db23 = assign7110_e8560_d_b23;
        var_fn61_calc_iq__exparg_db24 = assign7110_e8560_d_b24;
        var_fn61_calc_iq__exparg_db25 = assign7110_e8560_d_b25;
        var_fn61_calc_iq__exparg_db26 = assign7110_e8560_d_b26;
        var_fn61_calc_iq__exparg_db27 = assign7110_e8560_d_b27;
        var_fn61_calc_iq__exparg_db28 = assign7110_e8560_d_b28;
        var_fn61_calc_iq__exparg_db29 = assign7110_e8560_d_b29;
        var_fn61_calc_iq__exparg_db30 = assign7110_e8560_d_b30;
        var_fn61_calc_iq__exparg_db31 = assign7110_e8560_d_b31;
        var_fn61_calc_iq__exparg_db32 = assign7110_e8560_d_b32;
        var_fn61_calc_iq__exparg_db33 = assign7110_e8560_d_b33;
        var_fn61_calc_iq__exparg_db34 = assign7110_e8560_d_b34;
        var_fn61_calc_iq__exparg_db35 = assign7110_e8560_d_b35;

        let (assign7120_e8578, assign7120_e8578_d_n0, assign7120_e8578_d_n1, assign7120_e8578_d_n2, assign7120_e8578_d_n3, assign7120_e8578_d_n4, assign7120_e8578_d_n5, assign7120_e8578_d_n6, assign7120_e8578_d_n7, assign7120_e8578_d_n8, assign7120_e8578_d_n9, assign7120_e8578_d_n10, assign7120_e8578_d_n11, assign7120_e8578_d_n12, assign7120_e8578_d_n13, assign7120_e8578_d_n14, assign7120_e8578_d_n15, assign7120_e8578_d_n16, assign7120_e8578_d_n17, assign7120_e8578_d_n18, assign7120_e8578_d_n19, assign7120_e8578_d_n20, assign7120_e8578_d_n21, assign7120_e8578_d_n22, assign7120_e8578_d_n23, assign7120_e8578_d_n24, assign7120_e8578_d_n25, assign7120_e8578_d_n26, assign7120_e8578_d_n27, assign7120_e8578_d_n28, assign7120_e8578_d_n29, assign7120_e8578_d_b0, assign7120_e8578_d_b1, assign7120_e8578_d_b2, assign7120_e8578_d_b3, assign7120_e8578_d_b4, assign7120_e8578_d_b5, assign7120_e8578_d_b6, assign7120_e8578_d_b7, assign7120_e8578_d_b8, assign7120_e8578_d_b9, assign7120_e8578_d_b10, assign7120_e8578_d_b11, assign7120_e8578_d_b12, assign7120_e8578_d_b13, assign7120_e8578_d_b14, assign7120_e8578_d_b15, assign7120_e8578_d_b16, assign7120_e8578_d_b17, assign7120_e8578_d_b18, assign7120_e8578_d_b19, assign7120_e8578_d_b20, assign7120_e8578_d_b21, assign7120_e8578_d_b22, assign7120_e8578_d_b23, assign7120_e8578_d_b24, assign7120_e8578_d_b25, assign7120_e8578_d_b26, assign7120_e8578_d_b27, assign7120_e8578_d_b28, assign7120_e8578_d_b29, assign7120_e8578_d_b30, assign7120_e8578_d_b31, assign7120_e8578_d_b32, assign7120_e8578_d_b33, assign7120_e8578_d_b34, assign7120_e8578_d_b35,) = {
    if ((var_guard60 != 0.0) && (var_guard87 != 0.0)) {
        let assign7120_e8566: f64 = (var_fn61_calc_iq__w * var_fn61_calc_iq__ngf);
        let assign7120_e8568: f64 = (assign7120_e8566 * var_fn61_calc_iq__type);
        let assign7120_e8570: f64 = (assign7120_e8568 * var_fn61_calc_iq__cb);
        let assign7120_e8572: f64 = (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0);
        let assign7120_e8574: f64 = (assign7120_e8572 * var_fn61_calc_iq__exparg);
        let assign7120_e8576: f64 = (assign7120_e8574 * var_fn61_calc_iq__trapfracdl);
        (assign7120_e8576, ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn0) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn0)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn0)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn1) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn1)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn1)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn2) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn2)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn2)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn3) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn3)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn3)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn4) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn4)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn4)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn5) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn5)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn5)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn6) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn6)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn6)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn7) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn7)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn7)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn8) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn8)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn8)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn9) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn9)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn9)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn10) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn10)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn10)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn11) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn11)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn11)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn12) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn12)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn12)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn13) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn13)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn13)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn14) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn14)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn14)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn15) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn15)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn15)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn16) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn16)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn16)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn17) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn17)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn17)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn18) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn18)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn18)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn19) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn19)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn19)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn20) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn20)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn20)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn21) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn21)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn21)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn22) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn22)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn22)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn23) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn23)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn23)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn24) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn24)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn24)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn25) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn25)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn25)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn26) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn26)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn26)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn27) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn27)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn27)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn28) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn28)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn28)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_dn29) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_dn29)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_dn29)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db0) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db0)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db0)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db1) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db1)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db1)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db2) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db2)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db2)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db3) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db3)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db3)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db4) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db4)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db4)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db5) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db5)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db5)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db6) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db6)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db6)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db7) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db7)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db7)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db8) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db8)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db8)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db9) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db9)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db9)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db10) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db10)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db10)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db11) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db11)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db11)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db12) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db12)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db12)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db13) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db13)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db13)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db14) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db14)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db14)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db15) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db15)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db15)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db16) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db16)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db16)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db17) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db17)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db17)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db18) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db18)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db18)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db19) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db19)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db19)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db20) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db20)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db20)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db21) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db21)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db21)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db22) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db22)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db22)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db23) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db23)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db23)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db24) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db24)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db24)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db25) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db25)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db25)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db26) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db26)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db26)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db27) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db27)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db27)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db28) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db28)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db28)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db29) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db29)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db29)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db30) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db30)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db30)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db31) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db31)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db31)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db32) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db32)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db32)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db33) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db33)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db33)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db34) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db34)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db34)) * var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * var_fn61_calc_iq__cb_db35) * var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * var_fn61_calc_iq__two_n_phit0_db35)) * var_fn61_calc_iq__exparg) + (assign7120_e8572 * var_fn61_calc_iq__exparg_db35)) * var_fn61_calc_iq__trapfracdl),)
    } else {
        (var_fn61_calc_iq__qbout, var_fn61_calc_iq__qbout_dn0, var_fn61_calc_iq__qbout_dn1, var_fn61_calc_iq__qbout_dn2, var_fn61_calc_iq__qbout_dn3, var_fn61_calc_iq__qbout_dn4, var_fn61_calc_iq__qbout_dn5, var_fn61_calc_iq__qbout_dn6, var_fn61_calc_iq__qbout_dn7, var_fn61_calc_iq__qbout_dn8, var_fn61_calc_iq__qbout_dn9, var_fn61_calc_iq__qbout_dn10, var_fn61_calc_iq__qbout_dn11, var_fn61_calc_iq__qbout_dn12, var_fn61_calc_iq__qbout_dn13, var_fn61_calc_iq__qbout_dn14, var_fn61_calc_iq__qbout_dn15, var_fn61_calc_iq__qbout_dn16, var_fn61_calc_iq__qbout_dn17, var_fn61_calc_iq__qbout_dn18, var_fn61_calc_iq__qbout_dn19, var_fn61_calc_iq__qbout_dn20, var_fn61_calc_iq__qbout_dn21, var_fn61_calc_iq__qbout_dn22, var_fn61_calc_iq__qbout_dn23, var_fn61_calc_iq__qbout_dn24, var_fn61_calc_iq__qbout_dn25, var_fn61_calc_iq__qbout_dn26, var_fn61_calc_iq__qbout_dn27, var_fn61_calc_iq__qbout_dn28, var_fn61_calc_iq__qbout_dn29, var_fn61_calc_iq__qbout_db0, var_fn61_calc_iq__qbout_db1, var_fn61_calc_iq__qbout_db2, var_fn61_calc_iq__qbout_db3, var_fn61_calc_iq__qbout_db4, var_fn61_calc_iq__qbout_db5, var_fn61_calc_iq__qbout_db6, var_fn61_calc_iq__qbout_db7, var_fn61_calc_iq__qbout_db8, var_fn61_calc_iq__qbout_db9, var_fn61_calc_iq__qbout_db10, var_fn61_calc_iq__qbout_db11, var_fn61_calc_iq__qbout_db12, var_fn61_calc_iq__qbout_db13, var_fn61_calc_iq__qbout_db14, var_fn61_calc_iq__qbout_db15, var_fn61_calc_iq__qbout_db16, var_fn61_calc_iq__qbout_db17, var_fn61_calc_iq__qbout_db18, var_fn61_calc_iq__qbout_db19, var_fn61_calc_iq__qbout_db20, var_fn61_calc_iq__qbout_db21, var_fn61_calc_iq__qbout_db22, var_fn61_calc_iq__qbout_db23, var_fn61_calc_iq__qbout_db24, var_fn61_calc_iq__qbout_db25, var_fn61_calc_iq__qbout_db26, var_fn61_calc_iq__qbout_db27, var_fn61_calc_iq__qbout_db28, var_fn61_calc_iq__qbout_db29, var_fn61_calc_iq__qbout_db30, var_fn61_calc_iq__qbout_db31, var_fn61_calc_iq__qbout_db32, var_fn61_calc_iq__qbout_db33, var_fn61_calc_iq__qbout_db34, var_fn61_calc_iq__qbout_db35,)
    }
};
        var_fn61_calc_iq__qbout = assign7120_e8578;
        var_fn61_calc_iq__qbout_dn0 = assign7120_e8578_d_n0;
        var_fn61_calc_iq__qbout_dn1 = assign7120_e8578_d_n1;
        var_fn61_calc_iq__qbout_dn2 = assign7120_e8578_d_n2;
        var_fn61_calc_iq__qbout_dn3 = assign7120_e8578_d_n3;
        var_fn61_calc_iq__qbout_dn4 = assign7120_e8578_d_n4;
        var_fn61_calc_iq__qbout_dn5 = assign7120_e8578_d_n5;
        var_fn61_calc_iq__qbout_dn6 = assign7120_e8578_d_n6;
        var_fn61_calc_iq__qbout_dn7 = assign7120_e8578_d_n7;
        var_fn61_calc_iq__qbout_dn8 = assign7120_e8578_d_n8;
        var_fn61_calc_iq__qbout_dn9 = assign7120_e8578_d_n9;
        var_fn61_calc_iq__qbout_dn10 = assign7120_e8578_d_n10;
        var_fn61_calc_iq__qbout_dn11 = assign7120_e8578_d_n11;
        var_fn61_calc_iq__qbout_dn12 = assign7120_e8578_d_n12;
        var_fn61_calc_iq__qbout_dn13 = assign7120_e8578_d_n13;
        var_fn61_calc_iq__qbout_dn14 = assign7120_e8578_d_n14;
        var_fn61_calc_iq__qbout_dn15 = assign7120_e8578_d_n15;
        var_fn61_calc_iq__qbout_dn16 = assign7120_e8578_d_n16;
        var_fn61_calc_iq__qbout_dn17 = assign7120_e8578_d_n17;
        var_fn61_calc_iq__qbout_dn18 = assign7120_e8578_d_n18;
        var_fn61_calc_iq__qbout_dn19 = assign7120_e8578_d_n19;
        var_fn61_calc_iq__qbout_dn20 = assign7120_e8578_d_n20;
        var_fn61_calc_iq__qbout_dn21 = assign7120_e8578_d_n21;
        var_fn61_calc_iq__qbout_dn22 = assign7120_e8578_d_n22;
        var_fn61_calc_iq__qbout_dn23 = assign7120_e8578_d_n23;
        var_fn61_calc_iq__qbout_dn24 = assign7120_e8578_d_n24;
        var_fn61_calc_iq__qbout_dn25 = assign7120_e8578_d_n25;
        var_fn61_calc_iq__qbout_dn26 = assign7120_e8578_d_n26;
        var_fn61_calc_iq__qbout_dn27 = assign7120_e8578_d_n27;
        var_fn61_calc_iq__qbout_dn28 = assign7120_e8578_d_n28;
        var_fn61_calc_iq__qbout_dn29 = assign7120_e8578_d_n29;
        var_fn61_calc_iq__qbout_db0 = assign7120_e8578_d_b0;
        var_fn61_calc_iq__qbout_db1 = assign7120_e8578_d_b1;
        var_fn61_calc_iq__qbout_db2 = assign7120_e8578_d_b2;
        var_fn61_calc_iq__qbout_db3 = assign7120_e8578_d_b3;
        var_fn61_calc_iq__qbout_db4 = assign7120_e8578_d_b4;
        var_fn61_calc_iq__qbout_db5 = assign7120_e8578_d_b5;
        var_fn61_calc_iq__qbout_db6 = assign7120_e8578_d_b6;
        var_fn61_calc_iq__qbout_db7 = assign7120_e8578_d_b7;
        var_fn61_calc_iq__qbout_db8 = assign7120_e8578_d_b8;
        var_fn61_calc_iq__qbout_db9 = assign7120_e8578_d_b9;
        var_fn61_calc_iq__qbout_db10 = assign7120_e8578_d_b10;
        var_fn61_calc_iq__qbout_db11 = assign7120_e8578_d_b11;
        var_fn61_calc_iq__qbout_db12 = assign7120_e8578_d_b12;
        var_fn61_calc_iq__qbout_db13 = assign7120_e8578_d_b13;
        var_fn61_calc_iq__qbout_db14 = assign7120_e8578_d_b14;
        var_fn61_calc_iq__qbout_db15 = assign7120_e8578_d_b15;
        var_fn61_calc_iq__qbout_db16 = assign7120_e8578_d_b16;
        var_fn61_calc_iq__qbout_db17 = assign7120_e8578_d_b17;
        var_fn61_calc_iq__qbout_db18 = assign7120_e8578_d_b18;
        var_fn61_calc_iq__qbout_db19 = assign7120_e8578_d_b19;
        var_fn61_calc_iq__qbout_db20 = assign7120_e8578_d_b20;
        var_fn61_calc_iq__qbout_db21 = assign7120_e8578_d_b21;
        var_fn61_calc_iq__qbout_db22 = assign7120_e8578_d_b22;
        var_fn61_calc_iq__qbout_db23 = assign7120_e8578_d_b23;
        var_fn61_calc_iq__qbout_db24 = assign7120_e8578_d_b24;
        var_fn61_calc_iq__qbout_db25 = assign7120_e8578_d_b25;
        var_fn61_calc_iq__qbout_db26 = assign7120_e8578_d_b26;
        var_fn61_calc_iq__qbout_db27 = assign7120_e8578_d_b27;
        var_fn61_calc_iq__qbout_db28 = assign7120_e8578_d_b28;
        var_fn61_calc_iq__qbout_db29 = assign7120_e8578_d_b29;
        var_fn61_calc_iq__qbout_db30 = assign7120_e8578_d_b30;
        var_fn61_calc_iq__qbout_db31 = assign7120_e8578_d_b31;
        var_fn61_calc_iq__qbout_db32 = assign7120_e8578_d_b32;
        var_fn61_calc_iq__qbout_db33 = assign7120_e8578_d_b33;
        var_fn61_calc_iq__qbout_db34 = assign7120_e8578_d_b34;
        var_fn61_calc_iq__qbout_db35 = assign7120_e8578_d_b35;

        let (assign7130_e8585, assign7130_e8585_d_n0, assign7130_e8585_d_n1, assign7130_e8585_d_n2, assign7130_e8585_d_n3, assign7130_e8585_d_n4, assign7130_e8585_d_n5, assign7130_e8585_d_n6, assign7130_e8585_d_n7, assign7130_e8585_d_n8, assign7130_e8585_d_n9, assign7130_e8585_d_n10, assign7130_e8585_d_n11, assign7130_e8585_d_n12, assign7130_e8585_d_n13, assign7130_e8585_d_n14, assign7130_e8585_d_n15, assign7130_e8585_d_n16, assign7130_e8585_d_n17, assign7130_e8585_d_n18, assign7130_e8585_d_n19, assign7130_e8585_d_n20, assign7130_e8585_d_n21, assign7130_e8585_d_n22, assign7130_e8585_d_n23, assign7130_e8585_d_n24, assign7130_e8585_d_n25, assign7130_e8585_d_n26, assign7130_e8585_d_n27, assign7130_e8585_d_n28, assign7130_e8585_d_n29, assign7130_e8585_d_b0, assign7130_e8585_d_b1, assign7130_e8585_d_b2, assign7130_e8585_d_b3, assign7130_e8585_d_b4, assign7130_e8585_d_b5, assign7130_e8585_d_b6, assign7130_e8585_d_b7, assign7130_e8585_d_b8, assign7130_e8585_d_b9, assign7130_e8585_d_b10, assign7130_e8585_d_b11, assign7130_e8585_d_b12, assign7130_e8585_d_b13, assign7130_e8585_d_b14, assign7130_e8585_d_b15, assign7130_e8585_d_b16, assign7130_e8585_d_b17, assign7130_e8585_d_b18, assign7130_e8585_d_b19, assign7130_e8585_d_b20, assign7130_e8585_d_b21, assign7130_e8585_d_b22, assign7130_e8585_d_b23, assign7130_e8585_d_b24, assign7130_e8585_d_b25, assign7130_e8585_d_b26, assign7130_e8585_d_b27, assign7130_e8585_d_b28, assign7130_e8585_d_b29, assign7130_e8585_d_b30, assign7130_e8585_d_b31, assign7130_e8585_d_b32, assign7130_e8585_d_b33, assign7130_e8585_d_b34, assign7130_e8585_d_b35,) = {
    if ((var_guard60 != 0.0) && (var_guard87 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn61_calc_iq__qcout, var_fn61_calc_iq__qcout_dn0, var_fn61_calc_iq__qcout_dn1, var_fn61_calc_iq__qcout_dn2, var_fn61_calc_iq__qcout_dn3, var_fn61_calc_iq__qcout_dn4, var_fn61_calc_iq__qcout_dn5, var_fn61_calc_iq__qcout_dn6, var_fn61_calc_iq__qcout_dn7, var_fn61_calc_iq__qcout_dn8, var_fn61_calc_iq__qcout_dn9, var_fn61_calc_iq__qcout_dn10, var_fn61_calc_iq__qcout_dn11, var_fn61_calc_iq__qcout_dn12, var_fn61_calc_iq__qcout_dn13, var_fn61_calc_iq__qcout_dn14, var_fn61_calc_iq__qcout_dn15, var_fn61_calc_iq__qcout_dn16, var_fn61_calc_iq__qcout_dn17, var_fn61_calc_iq__qcout_dn18, var_fn61_calc_iq__qcout_dn19, var_fn61_calc_iq__qcout_dn20, var_fn61_calc_iq__qcout_dn21, var_fn61_calc_iq__qcout_dn22, var_fn61_calc_iq__qcout_dn23, var_fn61_calc_iq__qcout_dn24, var_fn61_calc_iq__qcout_dn25, var_fn61_calc_iq__qcout_dn26, var_fn61_calc_iq__qcout_dn27, var_fn61_calc_iq__qcout_dn28, var_fn61_calc_iq__qcout_dn29, var_fn61_calc_iq__qcout_db0, var_fn61_calc_iq__qcout_db1, var_fn61_calc_iq__qcout_db2, var_fn61_calc_iq__qcout_db3, var_fn61_calc_iq__qcout_db4, var_fn61_calc_iq__qcout_db5, var_fn61_calc_iq__qcout_db6, var_fn61_calc_iq__qcout_db7, var_fn61_calc_iq__qcout_db8, var_fn61_calc_iq__qcout_db9, var_fn61_calc_iq__qcout_db10, var_fn61_calc_iq__qcout_db11, var_fn61_calc_iq__qcout_db12, var_fn61_calc_iq__qcout_db13, var_fn61_calc_iq__qcout_db14, var_fn61_calc_iq__qcout_db15, var_fn61_calc_iq__qcout_db16, var_fn61_calc_iq__qcout_db17, var_fn61_calc_iq__qcout_db18, var_fn61_calc_iq__qcout_db19, var_fn61_calc_iq__qcout_db20, var_fn61_calc_iq__qcout_db21, var_fn61_calc_iq__qcout_db22, var_fn61_calc_iq__qcout_db23, var_fn61_calc_iq__qcout_db24, var_fn61_calc_iq__qcout_db25, var_fn61_calc_iq__qcout_db26, var_fn61_calc_iq__qcout_db27, var_fn61_calc_iq__qcout_db28, var_fn61_calc_iq__qcout_db29, var_fn61_calc_iq__qcout_db30, var_fn61_calc_iq__qcout_db31, var_fn61_calc_iq__qcout_db32, var_fn61_calc_iq__qcout_db33, var_fn61_calc_iq__qcout_db34, var_fn61_calc_iq__qcout_db35,)
    }
};
        var_fn61_calc_iq__qcout = assign7130_e8585;
        var_fn61_calc_iq__qcout_dn0 = assign7130_e8585_d_n0;
        var_fn61_calc_iq__qcout_dn1 = assign7130_e8585_d_n1;
        var_fn61_calc_iq__qcout_dn2 = assign7130_e8585_d_n2;
        var_fn61_calc_iq__qcout_dn3 = assign7130_e8585_d_n3;
        var_fn61_calc_iq__qcout_dn4 = assign7130_e8585_d_n4;
        var_fn61_calc_iq__qcout_dn5 = assign7130_e8585_d_n5;
        var_fn61_calc_iq__qcout_dn6 = assign7130_e8585_d_n6;
        var_fn61_calc_iq__qcout_dn7 = assign7130_e8585_d_n7;
        var_fn61_calc_iq__qcout_dn8 = assign7130_e8585_d_n8;
        var_fn61_calc_iq__qcout_dn9 = assign7130_e8585_d_n9;
        var_fn61_calc_iq__qcout_dn10 = assign7130_e8585_d_n10;
        var_fn61_calc_iq__qcout_dn11 = assign7130_e8585_d_n11;
        var_fn61_calc_iq__qcout_dn12 = assign7130_e8585_d_n12;
        var_fn61_calc_iq__qcout_dn13 = assign7130_e8585_d_n13;
        var_fn61_calc_iq__qcout_dn14 = assign7130_e8585_d_n14;
        var_fn61_calc_iq__qcout_dn15 = assign7130_e8585_d_n15;
        var_fn61_calc_iq__qcout_dn16 = assign7130_e8585_d_n16;
        var_fn61_calc_iq__qcout_dn17 = assign7130_e8585_d_n17;
        var_fn61_calc_iq__qcout_dn18 = assign7130_e8585_d_n18;
        var_fn61_calc_iq__qcout_dn19 = assign7130_e8585_d_n19;
        var_fn61_calc_iq__qcout_dn20 = assign7130_e8585_d_n20;
        var_fn61_calc_iq__qcout_dn21 = assign7130_e8585_d_n21;
        var_fn61_calc_iq__qcout_dn22 = assign7130_e8585_d_n22;
        var_fn61_calc_iq__qcout_dn23 = assign7130_e8585_d_n23;
        var_fn61_calc_iq__qcout_dn24 = assign7130_e8585_d_n24;
        var_fn61_calc_iq__qcout_dn25 = assign7130_e8585_d_n25;
        var_fn61_calc_iq__qcout_dn26 = assign7130_e8585_d_n26;
        var_fn61_calc_iq__qcout_dn27 = assign7130_e8585_d_n27;
        var_fn61_calc_iq__qcout_dn28 = assign7130_e8585_d_n28;
        var_fn61_calc_iq__qcout_dn29 = assign7130_e8585_d_n29;
        var_fn61_calc_iq__qcout_db0 = assign7130_e8585_d_b0;
        var_fn61_calc_iq__qcout_db1 = assign7130_e8585_d_b1;
        var_fn61_calc_iq__qcout_db2 = assign7130_e8585_d_b2;
        var_fn61_calc_iq__qcout_db3 = assign7130_e8585_d_b3;
        var_fn61_calc_iq__qcout_db4 = assign7130_e8585_d_b4;
        var_fn61_calc_iq__qcout_db5 = assign7130_e8585_d_b5;
        var_fn61_calc_iq__qcout_db6 = assign7130_e8585_d_b6;
        var_fn61_calc_iq__qcout_db7 = assign7130_e8585_d_b7;
        var_fn61_calc_iq__qcout_db8 = assign7130_e8585_d_b8;
        var_fn61_calc_iq__qcout_db9 = assign7130_e8585_d_b9;
        var_fn61_calc_iq__qcout_db10 = assign7130_e8585_d_b10;
        var_fn61_calc_iq__qcout_db11 = assign7130_e8585_d_b11;
        var_fn61_calc_iq__qcout_db12 = assign7130_e8585_d_b12;
        var_fn61_calc_iq__qcout_db13 = assign7130_e8585_d_b13;
        var_fn61_calc_iq__qcout_db14 = assign7130_e8585_d_b14;
        var_fn61_calc_iq__qcout_db15 = assign7130_e8585_d_b15;
        var_fn61_calc_iq__qcout_db16 = assign7130_e8585_d_b16;
        var_fn61_calc_iq__qcout_db17 = assign7130_e8585_d_b17;
        var_fn61_calc_iq__qcout_db18 = assign7130_e8585_d_b18;
        var_fn61_calc_iq__qcout_db19 = assign7130_e8585_d_b19;
        var_fn61_calc_iq__qcout_db20 = assign7130_e8585_d_b20;
        var_fn61_calc_iq__qcout_db21 = assign7130_e8585_d_b21;
        var_fn61_calc_iq__qcout_db22 = assign7130_e8585_d_b22;
        var_fn61_calc_iq__qcout_db23 = assign7130_e8585_d_b23;
        var_fn61_calc_iq__qcout_db24 = assign7130_e8585_d_b24;
        var_fn61_calc_iq__qcout_db25 = assign7130_e8585_d_b25;
        var_fn61_calc_iq__qcout_db26 = assign7130_e8585_d_b26;
        var_fn61_calc_iq__qcout_db27 = assign7130_e8585_d_b27;
        var_fn61_calc_iq__qcout_db28 = assign7130_e8585_d_b28;
        var_fn61_calc_iq__qcout_db29 = assign7130_e8585_d_b29;
        var_fn61_calc_iq__qcout_db30 = assign7130_e8585_d_b30;
        var_fn61_calc_iq__qcout_db31 = assign7130_e8585_d_b31;
        var_fn61_calc_iq__qcout_db32 = assign7130_e8585_d_b32;
        var_fn61_calc_iq__qcout_db33 = assign7130_e8585_d_b33;
        var_fn61_calc_iq__qcout_db34 = assign7130_e8585_d_b34;
        var_fn61_calc_iq__qcout_db35 = assign7130_e8585_d_b35;

        let (assign7140_e8592, assign7140_e8592_d_n0, assign7140_e8592_d_n1, assign7140_e8592_d_n2, assign7140_e8592_d_n3, assign7140_e8592_d_n4, assign7140_e8592_d_n5, assign7140_e8592_d_n6, assign7140_e8592_d_n7, assign7140_e8592_d_n8, assign7140_e8592_d_n9, assign7140_e8592_d_n10, assign7140_e8592_d_n11, assign7140_e8592_d_n12, assign7140_e8592_d_n13, assign7140_e8592_d_n14, assign7140_e8592_d_n15, assign7140_e8592_d_n16, assign7140_e8592_d_n17, assign7140_e8592_d_n18, assign7140_e8592_d_n19, assign7140_e8592_d_n20, assign7140_e8592_d_n21, assign7140_e8592_d_n22, assign7140_e8592_d_n23, assign7140_e8592_d_n24, assign7140_e8592_d_n25, assign7140_e8592_d_n26, assign7140_e8592_d_n27, assign7140_e8592_d_n28, assign7140_e8592_d_n29, assign7140_e8592_d_b0, assign7140_e8592_d_b1, assign7140_e8592_d_b2, assign7140_e8592_d_b3, assign7140_e8592_d_b4, assign7140_e8592_d_b5, assign7140_e8592_d_b6, assign7140_e8592_d_b7, assign7140_e8592_d_b8, assign7140_e8592_d_b9, assign7140_e8592_d_b10, assign7140_e8592_d_b11, assign7140_e8592_d_b12, assign7140_e8592_d_b13, assign7140_e8592_d_b14, assign7140_e8592_d_b15, assign7140_e8592_d_b16, assign7140_e8592_d_b17, assign7140_e8592_d_b18, assign7140_e8592_d_b19, assign7140_e8592_d_b20, assign7140_e8592_d_b21, assign7140_e8592_d_b22, assign7140_e8592_d_b23, assign7140_e8592_d_b24, assign7140_e8592_d_b25, assign7140_e8592_d_b26, assign7140_e8592_d_b27, assign7140_e8592_d_b28, assign7140_e8592_d_b29, assign7140_e8592_d_b30, assign7140_e8592_d_b31, assign7140_e8592_d_b32, assign7140_e8592_d_b33, assign7140_e8592_d_b34, assign7140_e8592_d_b35,) = {
    if ((var_guard60 != 0.0) && (var_guard87 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn61_calc_iq__qbout, var_fn61_calc_iq__qbout_dn0, var_fn61_calc_iq__qbout_dn1, var_fn61_calc_iq__qbout_dn2, var_fn61_calc_iq__qbout_dn3, var_fn61_calc_iq__qbout_dn4, var_fn61_calc_iq__qbout_dn5, var_fn61_calc_iq__qbout_dn6, var_fn61_calc_iq__qbout_dn7, var_fn61_calc_iq__qbout_dn8, var_fn61_calc_iq__qbout_dn9, var_fn61_calc_iq__qbout_dn10, var_fn61_calc_iq__qbout_dn11, var_fn61_calc_iq__qbout_dn12, var_fn61_calc_iq__qbout_dn13, var_fn61_calc_iq__qbout_dn14, var_fn61_calc_iq__qbout_dn15, var_fn61_calc_iq__qbout_dn16, var_fn61_calc_iq__qbout_dn17, var_fn61_calc_iq__qbout_dn18, var_fn61_calc_iq__qbout_dn19, var_fn61_calc_iq__qbout_dn20, var_fn61_calc_iq__qbout_dn21, var_fn61_calc_iq__qbout_dn22, var_fn61_calc_iq__qbout_dn23, var_fn61_calc_iq__qbout_dn24, var_fn61_calc_iq__qbout_dn25, var_fn61_calc_iq__qbout_dn26, var_fn61_calc_iq__qbout_dn27, var_fn61_calc_iq__qbout_dn28, var_fn61_calc_iq__qbout_dn29, var_fn61_calc_iq__qbout_db0, var_fn61_calc_iq__qbout_db1, var_fn61_calc_iq__qbout_db2, var_fn61_calc_iq__qbout_db3, var_fn61_calc_iq__qbout_db4, var_fn61_calc_iq__qbout_db5, var_fn61_calc_iq__qbout_db6, var_fn61_calc_iq__qbout_db7, var_fn61_calc_iq__qbout_db8, var_fn61_calc_iq__qbout_db9, var_fn61_calc_iq__qbout_db10, var_fn61_calc_iq__qbout_db11, var_fn61_calc_iq__qbout_db12, var_fn61_calc_iq__qbout_db13, var_fn61_calc_iq__qbout_db14, var_fn61_calc_iq__qbout_db15, var_fn61_calc_iq__qbout_db16, var_fn61_calc_iq__qbout_db17, var_fn61_calc_iq__qbout_db18, var_fn61_calc_iq__qbout_db19, var_fn61_calc_iq__qbout_db20, var_fn61_calc_iq__qbout_db21, var_fn61_calc_iq__qbout_db22, var_fn61_calc_iq__qbout_db23, var_fn61_calc_iq__qbout_db24, var_fn61_calc_iq__qbout_db25, var_fn61_calc_iq__qbout_db26, var_fn61_calc_iq__qbout_db27, var_fn61_calc_iq__qbout_db28, var_fn61_calc_iq__qbout_db29, var_fn61_calc_iq__qbout_db30, var_fn61_calc_iq__qbout_db31, var_fn61_calc_iq__qbout_db32, var_fn61_calc_iq__qbout_db33, var_fn61_calc_iq__qbout_db34, var_fn61_calc_iq__qbout_db35,)
    }
};
        var_fn61_calc_iq__qbout = assign7140_e8592;
        var_fn61_calc_iq__qbout_dn0 = assign7140_e8592_d_n0;
        var_fn61_calc_iq__qbout_dn1 = assign7140_e8592_d_n1;
        var_fn61_calc_iq__qbout_dn2 = assign7140_e8592_d_n2;
        var_fn61_calc_iq__qbout_dn3 = assign7140_e8592_d_n3;
        var_fn61_calc_iq__qbout_dn4 = assign7140_e8592_d_n4;
        var_fn61_calc_iq__qbout_dn5 = assign7140_e8592_d_n5;
        var_fn61_calc_iq__qbout_dn6 = assign7140_e8592_d_n6;
        var_fn61_calc_iq__qbout_dn7 = assign7140_e8592_d_n7;
        var_fn61_calc_iq__qbout_dn8 = assign7140_e8592_d_n8;
        var_fn61_calc_iq__qbout_dn9 = assign7140_e8592_d_n9;
        var_fn61_calc_iq__qbout_dn10 = assign7140_e8592_d_n10;
        var_fn61_calc_iq__qbout_dn11 = assign7140_e8592_d_n11;
        var_fn61_calc_iq__qbout_dn12 = assign7140_e8592_d_n12;
        var_fn61_calc_iq__qbout_dn13 = assign7140_e8592_d_n13;
        var_fn61_calc_iq__qbout_dn14 = assign7140_e8592_d_n14;
        var_fn61_calc_iq__qbout_dn15 = assign7140_e8592_d_n15;
        var_fn61_calc_iq__qbout_dn16 = assign7140_e8592_d_n16;
        var_fn61_calc_iq__qbout_dn17 = assign7140_e8592_d_n17;
        var_fn61_calc_iq__qbout_dn18 = assign7140_e8592_d_n18;
        var_fn61_calc_iq__qbout_dn19 = assign7140_e8592_d_n19;
        var_fn61_calc_iq__qbout_dn20 = assign7140_e8592_d_n20;
        var_fn61_calc_iq__qbout_dn21 = assign7140_e8592_d_n21;
        var_fn61_calc_iq__qbout_dn22 = assign7140_e8592_d_n22;
        var_fn61_calc_iq__qbout_dn23 = assign7140_e8592_d_n23;
        var_fn61_calc_iq__qbout_dn24 = assign7140_e8592_d_n24;
        var_fn61_calc_iq__qbout_dn25 = assign7140_e8592_d_n25;
        var_fn61_calc_iq__qbout_dn26 = assign7140_e8592_d_n26;
        var_fn61_calc_iq__qbout_dn27 = assign7140_e8592_d_n27;
        var_fn61_calc_iq__qbout_dn28 = assign7140_e8592_d_n28;
        var_fn61_calc_iq__qbout_dn29 = assign7140_e8592_d_n29;
        var_fn61_calc_iq__qbout_db0 = assign7140_e8592_d_b0;
        var_fn61_calc_iq__qbout_db1 = assign7140_e8592_d_b1;
        var_fn61_calc_iq__qbout_db2 = assign7140_e8592_d_b2;
        var_fn61_calc_iq__qbout_db3 = assign7140_e8592_d_b3;
        var_fn61_calc_iq__qbout_db4 = assign7140_e8592_d_b4;
        var_fn61_calc_iq__qbout_db5 = assign7140_e8592_d_b5;
        var_fn61_calc_iq__qbout_db6 = assign7140_e8592_d_b6;
        var_fn61_calc_iq__qbout_db7 = assign7140_e8592_d_b7;
        var_fn61_calc_iq__qbout_db8 = assign7140_e8592_d_b8;
        var_fn61_calc_iq__qbout_db9 = assign7140_e8592_d_b9;
        var_fn61_calc_iq__qbout_db10 = assign7140_e8592_d_b10;
        var_fn61_calc_iq__qbout_db11 = assign7140_e8592_d_b11;
        var_fn61_calc_iq__qbout_db12 = assign7140_e8592_d_b12;
        var_fn61_calc_iq__qbout_db13 = assign7140_e8592_d_b13;
        var_fn61_calc_iq__qbout_db14 = assign7140_e8592_d_b14;
        var_fn61_calc_iq__qbout_db15 = assign7140_e8592_d_b15;
        var_fn61_calc_iq__qbout_db16 = assign7140_e8592_d_b16;
        var_fn61_calc_iq__qbout_db17 = assign7140_e8592_d_b17;
        var_fn61_calc_iq__qbout_db18 = assign7140_e8592_d_b18;
        var_fn61_calc_iq__qbout_db19 = assign7140_e8592_d_b19;
        var_fn61_calc_iq__qbout_db20 = assign7140_e8592_d_b20;
        var_fn61_calc_iq__qbout_db21 = assign7140_e8592_d_b21;
        var_fn61_calc_iq__qbout_db22 = assign7140_e8592_d_b22;
        var_fn61_calc_iq__qbout_db23 = assign7140_e8592_d_b23;
        var_fn61_calc_iq__qbout_db24 = assign7140_e8592_d_b24;
        var_fn61_calc_iq__qbout_db25 = assign7140_e8592_d_b25;
        var_fn61_calc_iq__qbout_db26 = assign7140_e8592_d_b26;
        var_fn61_calc_iq__qbout_db27 = assign7140_e8592_d_b27;
        var_fn61_calc_iq__qbout_db28 = assign7140_e8592_d_b28;
        var_fn61_calc_iq__qbout_db29 = assign7140_e8592_d_b29;
        var_fn61_calc_iq__qbout_db30 = assign7140_e8592_d_b30;
        var_fn61_calc_iq__qbout_db31 = assign7140_e8592_d_b31;
        var_fn61_calc_iq__qbout_db32 = assign7140_e8592_d_b32;
        var_fn61_calc_iq__qbout_db33 = assign7140_e8592_d_b33;
        var_fn61_calc_iq__qbout_db34 = assign7140_e8592_d_b34;
        var_fn61_calc_iq__qbout_db35 = assign7140_e8592_d_b35;

        let assign7150_e8595: f64 = if var_fn61_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        var_guard92 = assign7150_e8595;

        let (assign7160_e8611, assign7160_e8611_d_n0, assign7160_e8611_d_n1, assign7160_e8611_d_n2, assign7160_e8611_d_n3, assign7160_e8611_d_n4, assign7160_e8611_d_n5, assign7160_e8611_d_n6, assign7160_e8611_d_n7, assign7160_e8611_d_n8, assign7160_e8611_d_n9, assign7160_e8611_d_n10, assign7160_e8611_d_n11, assign7160_e8611_d_n12, assign7160_e8611_d_n13, assign7160_e8611_d_n14, assign7160_e8611_d_n15, assign7160_e8611_d_n16, assign7160_e8611_d_n17, assign7160_e8611_d_n18, assign7160_e8611_d_n19, assign7160_e8611_d_n20, assign7160_e8611_d_n21, assign7160_e8611_d_n22, assign7160_e8611_d_n23, assign7160_e8611_d_n24, assign7160_e8611_d_n25, assign7160_e8611_d_n26, assign7160_e8611_d_n27, assign7160_e8611_d_n28, assign7160_e8611_d_n29, assign7160_e8611_d_b0, assign7160_e8611_d_b1, assign7160_e8611_d_b2, assign7160_e8611_d_b3, assign7160_e8611_d_b4, assign7160_e8611_d_b5, assign7160_e8611_d_b6, assign7160_e8611_d_b7, assign7160_e8611_d_b8, assign7160_e8611_d_b9, assign7160_e8611_d_b10, assign7160_e8611_d_b11, assign7160_e8611_d_b12, assign7160_e8611_d_b13, assign7160_e8611_d_b14, assign7160_e8611_d_b15, assign7160_e8611_d_b16, assign7160_e8611_d_b17, assign7160_e8611_d_b18, assign7160_e8611_d_b19, assign7160_e8611_d_b20, assign7160_e8611_d_b21, assign7160_e8611_d_b22, assign7160_e8611_d_b23, assign7160_e8611_d_b24, assign7160_e8611_d_b25, assign7160_e8611_d_b26, assign7160_e8611_d_b27, assign7160_e8611_d_b28, assign7160_e8611_d_b29, assign7160_e8611_d_b30, assign7160_e8611_d_b31, assign7160_e8611_d_b32, assign7160_e8611_d_b33, assign7160_e8611_d_b34, assign7160_e8611_d_b35,) = {
    if ((var_guard60 != 0.0) && (var_guard92 != 0.0)) {
        let assign7160_e8603: f64 = (p.p51 * 0.5);
        let assign7160_e8605: f64 = (assign7160_e8603 * var_fn61_calc_iq__alpha_phit);
        let assign7160_e8606: f64 = (var_fn61_calc_iq__vtof - assign7160_e8605);
        let assign7160_e8607: f64 = (var_fn61_calc_iq__vgsin - assign7160_e8606);
        let assign7160_e8609: f64 = (assign7160_e8607 / var_fn61_calc_iq__two_n_phit0);
        (assign7160_e8609, ((((var_fn61_calc_iq__vgsin_dn0 - (var_fn61_calc_iq__vtof_dn0 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn0))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn0)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn1 - (var_fn61_calc_iq__vtof_dn1 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn1))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn1)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn2 - (var_fn61_calc_iq__vtof_dn2 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn2))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn2)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn3 - (var_fn61_calc_iq__vtof_dn3 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn3))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn3)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn4 - (var_fn61_calc_iq__vtof_dn4 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn4))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn4)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn5 - (var_fn61_calc_iq__vtof_dn5 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn5))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn5)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn6 - (var_fn61_calc_iq__vtof_dn6 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn6))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn6)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn7 - (var_fn61_calc_iq__vtof_dn7 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn7))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn7)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn8 - (var_fn61_calc_iq__vtof_dn8 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn8))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn8)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn9 - (var_fn61_calc_iq__vtof_dn9 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn9))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn9)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn10 - (var_fn61_calc_iq__vtof_dn10 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn10))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn10)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn11 - (var_fn61_calc_iq__vtof_dn11 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn11))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn11)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn12 - (var_fn61_calc_iq__vtof_dn12 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn12))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn12)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn13 - (var_fn61_calc_iq__vtof_dn13 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn13))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn13)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn14 - (var_fn61_calc_iq__vtof_dn14 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn14))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn14)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn15 - (var_fn61_calc_iq__vtof_dn15 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn15))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn15)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn16 - (var_fn61_calc_iq__vtof_dn16 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn16))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn16)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn17 - (var_fn61_calc_iq__vtof_dn17 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn17))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn17)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn18 - (var_fn61_calc_iq__vtof_dn18 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn18))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn18)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn19 - (var_fn61_calc_iq__vtof_dn19 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn19))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn19)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn20 - (var_fn61_calc_iq__vtof_dn20 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn20))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn20)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn21 - (var_fn61_calc_iq__vtof_dn21 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn21))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn21)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn22 - (var_fn61_calc_iq__vtof_dn22 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn22))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn22)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn23 - (var_fn61_calc_iq__vtof_dn23 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn23))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn23)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn24 - (var_fn61_calc_iq__vtof_dn24 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn24))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn24)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn25 - (var_fn61_calc_iq__vtof_dn25 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn25))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn25)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn26 - (var_fn61_calc_iq__vtof_dn26 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn26))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn26)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn27 - (var_fn61_calc_iq__vtof_dn27 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn27))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn27)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn28 - (var_fn61_calc_iq__vtof_dn28 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn28))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn28)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_dn29 - (var_fn61_calc_iq__vtof_dn29 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_dn29))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_dn29)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db0 - (var_fn61_calc_iq__vtof_db0 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db0))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db0)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db1 - (var_fn61_calc_iq__vtof_db1 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db1))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db1)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db2 - (var_fn61_calc_iq__vtof_db2 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db2))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db2)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db3 - (var_fn61_calc_iq__vtof_db3 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db3))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db3)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db4 - (var_fn61_calc_iq__vtof_db4 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db4))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db4)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db5 - (var_fn61_calc_iq__vtof_db5 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db5))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db5)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db6 - (var_fn61_calc_iq__vtof_db6 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db6))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db6)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db7 - (var_fn61_calc_iq__vtof_db7 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db7))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db7)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db8 - (var_fn61_calc_iq__vtof_db8 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db8))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db8)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db9 - (var_fn61_calc_iq__vtof_db9 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db9))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db9)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db10 - (var_fn61_calc_iq__vtof_db10 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db10))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db10)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db11 - (var_fn61_calc_iq__vtof_db11 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db11))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db11)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db12 - (var_fn61_calc_iq__vtof_db12 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db12))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db12)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db13 - (var_fn61_calc_iq__vtof_db13 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db13))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db13)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db14 - (var_fn61_calc_iq__vtof_db14 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db14))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db14)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db15 - (var_fn61_calc_iq__vtof_db15 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db15))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db15)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db16 - (var_fn61_calc_iq__vtof_db16 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db16))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db16)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db17 - (var_fn61_calc_iq__vtof_db17 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db17))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db17)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db18 - (var_fn61_calc_iq__vtof_db18 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db18))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db18)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db19 - (var_fn61_calc_iq__vtof_db19 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db19))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db19)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db20 - (var_fn61_calc_iq__vtof_db20 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db20))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db20)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db21 - (var_fn61_calc_iq__vtof_db21 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db21))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db21)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db22 - (var_fn61_calc_iq__vtof_db22 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db22))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db22)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db23 - (var_fn61_calc_iq__vtof_db23 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db23))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db23)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db24 - (var_fn61_calc_iq__vtof_db24 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db24))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db24)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db25 - (var_fn61_calc_iq__vtof_db25 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db25))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db25)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db26 - (var_fn61_calc_iq__vtof_db26 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db26))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db26)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db27 - (var_fn61_calc_iq__vtof_db27 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db27))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db27)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db28 - (var_fn61_calc_iq__vtof_db28 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db28))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db28)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db29 - (var_fn61_calc_iq__vtof_db29 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db29))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db29)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db30 - (var_fn61_calc_iq__vtof_db30 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db30))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db30)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db31 - (var_fn61_calc_iq__vtof_db31 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db31))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db31)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db32 - (var_fn61_calc_iq__vtof_db32 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db32))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db32)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db33 - (var_fn61_calc_iq__vtof_db33 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db33))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db33)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db34 - (var_fn61_calc_iq__vtof_db34 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db34))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db34)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)), ((((var_fn61_calc_iq__vgsin_db35 - (var_fn61_calc_iq__vtof_db35 - (assign7160_e8603 * var_fn61_calc_iq__alpha_phit_db35))) * var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * var_fn61_calc_iq__two_n_phit0_db35)) / (var_fn61_calc_iq__two_n_phit0 * var_fn61_calc_iq__two_n_phit0)),)
    } else {
        (var_fn61_calc_iq__etags, var_fn61_calc_iq__etags_dn0, var_fn61_calc_iq__etags_dn1, var_fn61_calc_iq__etags_dn2, var_fn61_calc_iq__etags_dn3, var_fn61_calc_iq__etags_dn4, var_fn61_calc_iq__etags_dn5, var_fn61_calc_iq__etags_dn6, var_fn61_calc_iq__etags_dn7, var_fn61_calc_iq__etags_dn8, var_fn61_calc_iq__etags_dn9, var_fn61_calc_iq__etags_dn10, var_fn61_calc_iq__etags_dn11, var_fn61_calc_iq__etags_dn12, var_fn61_calc_iq__etags_dn13, var_fn61_calc_iq__etags_dn14, var_fn61_calc_iq__etags_dn15, var_fn61_calc_iq__etags_dn16, var_fn61_calc_iq__etags_dn17, var_fn61_calc_iq__etags_dn18, var_fn61_calc_iq__etags_dn19, var_fn61_calc_iq__etags_dn20, var_fn61_calc_iq__etags_dn21, var_fn61_calc_iq__etags_dn22, var_fn61_calc_iq__etags_dn23, var_fn61_calc_iq__etags_dn24, var_fn61_calc_iq__etags_dn25, var_fn61_calc_iq__etags_dn26, var_fn61_calc_iq__etags_dn27, var_fn61_calc_iq__etags_dn28, var_fn61_calc_iq__etags_dn29, var_fn61_calc_iq__etags_db0, var_fn61_calc_iq__etags_db1, var_fn61_calc_iq__etags_db2, var_fn61_calc_iq__etags_db3, var_fn61_calc_iq__etags_db4, var_fn61_calc_iq__etags_db5, var_fn61_calc_iq__etags_db6, var_fn61_calc_iq__etags_db7, var_fn61_calc_iq__etags_db8, var_fn61_calc_iq__etags_db9, var_fn61_calc_iq__etags_db10, var_fn61_calc_iq__etags_db11, var_fn61_calc_iq__etags_db12, var_fn61_calc_iq__etags_db13, var_fn61_calc_iq__etags_db14, var_fn61_calc_iq__etags_db15, var_fn61_calc_iq__etags_db16, var_fn61_calc_iq__etags_db17, var_fn61_calc_iq__etags_db18, var_fn61_calc_iq__etags_db19, var_fn61_calc_iq__etags_db20, var_fn61_calc_iq__etags_db21, var_fn61_calc_iq__etags_db22, var_fn61_calc_iq__etags_db23, var_fn61_calc_iq__etags_db24, var_fn61_calc_iq__etags_db25, var_fn61_calc_iq__etags_db26, var_fn61_calc_iq__etags_db27, var_fn61_calc_iq__etags_db28, var_fn61_calc_iq__etags_db29, var_fn61_calc_iq__etags_db30, var_fn61_calc_iq__etags_db31, var_fn61_calc_iq__etags_db32, var_fn61_calc_iq__etags_db33, var_fn61_calc_iq__etags_db34, var_fn61_calc_iq__etags_db35,)
    }
};
        var_fn61_calc_iq__etags = assign7160_e8611;
        var_fn61_calc_iq__etags_dn0 = assign7160_e8611_d_n0;
        var_fn61_calc_iq__etags_dn1 = assign7160_e8611_d_n1;
        var_fn61_calc_iq__etags_dn2 = assign7160_e8611_d_n2;
        var_fn61_calc_iq__etags_dn3 = assign7160_e8611_d_n3;
        var_fn61_calc_iq__etags_dn4 = assign7160_e8611_d_n4;
        var_fn61_calc_iq__etags_dn5 = assign7160_e8611_d_n5;
        var_fn61_calc_iq__etags_dn6 = assign7160_e8611_d_n6;
        var_fn61_calc_iq__etags_dn7 = assign7160_e8611_d_n7;
        var_fn61_calc_iq__etags_dn8 = assign7160_e8611_d_n8;
        var_fn61_calc_iq__etags_dn9 = assign7160_e8611_d_n9;
        var_fn61_calc_iq__etags_dn10 = assign7160_e8611_d_n10;
        var_fn61_calc_iq__etags_dn11 = assign7160_e8611_d_n11;
        var_fn61_calc_iq__etags_dn12 = assign7160_e8611_d_n12;
        var_fn61_calc_iq__etags_dn13 = assign7160_e8611_d_n13;
        var_fn61_calc_iq__etags_dn14 = assign7160_e8611_d_n14;
        var_fn61_calc_iq__etags_dn15 = assign7160_e8611_d_n15;
        var_fn61_calc_iq__etags_dn16 = assign7160_e8611_d_n16;
        var_fn61_calc_iq__etags_dn17 = assign7160_e8611_d_n17;
        var_fn61_calc_iq__etags_dn18 = assign7160_e8611_d_n18;
        var_fn61_calc_iq__etags_dn19 = assign7160_e8611_d_n19;
        var_fn61_calc_iq__etags_dn20 = assign7160_e8611_d_n20;
        var_fn61_calc_iq__etags_dn21 = assign7160_e8611_d_n21;
        var_fn61_calc_iq__etags_dn22 = assign7160_e8611_d_n22;
        var_fn61_calc_iq__etags_dn23 = assign7160_e8611_d_n23;
        var_fn61_calc_iq__etags_dn24 = assign7160_e8611_d_n24;
        var_fn61_calc_iq__etags_dn25 = assign7160_e8611_d_n25;
        var_fn61_calc_iq__etags_dn26 = assign7160_e8611_d_n26;
        var_fn61_calc_iq__etags_dn27 = assign7160_e8611_d_n27;
        var_fn61_calc_iq__etags_dn28 = assign7160_e8611_d_n28;
        var_fn61_calc_iq__etags_dn29 = assign7160_e8611_d_n29;
        var_fn61_calc_iq__etags_db0 = assign7160_e8611_d_b0;
        var_fn61_calc_iq__etags_db1 = assign7160_e8611_d_b1;
        var_fn61_calc_iq__etags_db2 = assign7160_e8611_d_b2;
        var_fn61_calc_iq__etags_db3 = assign7160_e8611_d_b3;
        var_fn61_calc_iq__etags_db4 = assign7160_e8611_d_b4;
        var_fn61_calc_iq__etags_db5 = assign7160_e8611_d_b5;
        var_fn61_calc_iq__etags_db6 = assign7160_e8611_d_b6;
        var_fn61_calc_iq__etags_db7 = assign7160_e8611_d_b7;
        var_fn61_calc_iq__etags_db8 = assign7160_e8611_d_b8;
        var_fn61_calc_iq__etags_db9 = assign7160_e8611_d_b9;
        var_fn61_calc_iq__etags_db10 = assign7160_e8611_d_b10;
        var_fn61_calc_iq__etags_db11 = assign7160_e8611_d_b11;
        var_fn61_calc_iq__etags_db12 = assign7160_e8611_d_b12;
        var_fn61_calc_iq__etags_db13 = assign7160_e8611_d_b13;
        var_fn61_calc_iq__etags_db14 = assign7160_e8611_d_b14;
        var_fn61_calc_iq__etags_db15 = assign7160_e8611_d_b15;
        var_fn61_calc_iq__etags_db16 = assign7160_e8611_d_b16;
        var_fn61_calc_iq__etags_db17 = assign7160_e8611_d_b17;
        var_fn61_calc_iq__etags_db18 = assign7160_e8611_d_b18;
        var_fn61_calc_iq__etags_db19 = assign7160_e8611_d_b19;
        var_fn61_calc_iq__etags_db20 = assign7160_e8611_d_b20;
        var_fn61_calc_iq__etags_db21 = assign7160_e8611_d_b21;
        var_fn61_calc_iq__etags_db22 = assign7160_e8611_d_b22;
        var_fn61_calc_iq__etags_db23 = assign7160_e8611_d_b23;
        var_fn61_calc_iq__etags_db24 = assign7160_e8611_d_b24;
        var_fn61_calc_iq__etags_db25 = assign7160_e8611_d_b25;
        var_fn61_calc_iq__etags_db26 = assign7160_e8611_d_b26;
        var_fn61_calc_iq__etags_db27 = assign7160_e8611_d_b27;
        var_fn61_calc_iq__etags_db28 = assign7160_e8611_d_b28;
        var_fn61_calc_iq__etags_db29 = assign7160_e8611_d_b29;
        var_fn61_calc_iq__etags_db30 = assign7160_e8611_d_b30;
        var_fn61_calc_iq__etags_db31 = assign7160_e8611_d_b31;
        var_fn61_calc_iq__etags_db32 = assign7160_e8611_d_b32;
        var_fn61_calc_iq__etags_db33 = assign7160_e8611_d_b33;
        var_fn61_calc_iq__etags_db34 = assign7160_e8611_d_b34;
        var_fn61_calc_iq__etags_db35 = assign7160_e8611_d_b35;

        let assign7170_e8614: f64 = if var_fn61_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        var_guard93 = assign7170_e8614;


        *var_fn61_calc_iq__etags_slot = var_fn61_calc_iq__etags;
        *var_fn61_calc_iq__etags_db0_slot = var_fn61_calc_iq__etags_db0;
        *var_fn61_calc_iq__etags_db1_slot = var_fn61_calc_iq__etags_db1;
        *var_fn61_calc_iq__etags_db10_slot = var_fn61_calc_iq__etags_db10;
        *var_fn61_calc_iq__etags_db11_slot = var_fn61_calc_iq__etags_db11;
        *var_fn61_calc_iq__etags_db12_slot = var_fn61_calc_iq__etags_db12;
        *var_fn61_calc_iq__etags_db13_slot = var_fn61_calc_iq__etags_db13;
        *var_fn61_calc_iq__etags_db14_slot = var_fn61_calc_iq__etags_db14;
        *var_fn61_calc_iq__etags_db15_slot = var_fn61_calc_iq__etags_db15;
        *var_fn61_calc_iq__etags_db16_slot = var_fn61_calc_iq__etags_db16;
        *var_fn61_calc_iq__etags_db17_slot = var_fn61_calc_iq__etags_db17;
        *var_fn61_calc_iq__etags_db18_slot = var_fn61_calc_iq__etags_db18;
        *var_fn61_calc_iq__etags_db19_slot = var_fn61_calc_iq__etags_db19;
        *var_fn61_calc_iq__etags_db2_slot = var_fn61_calc_iq__etags_db2;
        *var_fn61_calc_iq__etags_db20_slot = var_fn61_calc_iq__etags_db20;
        *var_fn61_calc_iq__etags_db21_slot = var_fn61_calc_iq__etags_db21;
        *var_fn61_calc_iq__etags_db22_slot = var_fn61_calc_iq__etags_db22;
        *var_fn61_calc_iq__etags_db23_slot = var_fn61_calc_iq__etags_db23;
        *var_fn61_calc_iq__etags_db24_slot = var_fn61_calc_iq__etags_db24;
        *var_fn61_calc_iq__etags_db25_slot = var_fn61_calc_iq__etags_db25;
        *var_fn61_calc_iq__etags_db26_slot = var_fn61_calc_iq__etags_db26;
        *var_fn61_calc_iq__etags_db27_slot = var_fn61_calc_iq__etags_db27;
        *var_fn61_calc_iq__etags_db28_slot = var_fn61_calc_iq__etags_db28;
        *var_fn61_calc_iq__etags_db29_slot = var_fn61_calc_iq__etags_db29;
        *var_fn61_calc_iq__etags_db3_slot = var_fn61_calc_iq__etags_db3;
        *var_fn61_calc_iq__etags_db30_slot = var_fn61_calc_iq__etags_db30;
        *var_fn61_calc_iq__etags_db31_slot = var_fn61_calc_iq__etags_db31;
        *var_fn61_calc_iq__etags_db32_slot = var_fn61_calc_iq__etags_db32;
        *var_fn61_calc_iq__etags_db33_slot = var_fn61_calc_iq__etags_db33;
        *var_fn61_calc_iq__etags_db34_slot = var_fn61_calc_iq__etags_db34;
        *var_fn61_calc_iq__etags_db35_slot = var_fn61_calc_iq__etags_db35;
        *var_fn61_calc_iq__etags_db4_slot = var_fn61_calc_iq__etags_db4;
        *var_fn61_calc_iq__etags_db5_slot = var_fn61_calc_iq__etags_db5;
        *var_fn61_calc_iq__etags_db6_slot = var_fn61_calc_iq__etags_db6;
        *var_fn61_calc_iq__etags_db7_slot = var_fn61_calc_iq__etags_db7;
        *var_fn61_calc_iq__etags_db8_slot = var_fn61_calc_iq__etags_db8;
        *var_fn61_calc_iq__etags_db9_slot = var_fn61_calc_iq__etags_db9;
        *var_fn61_calc_iq__etags_dn0_slot = var_fn61_calc_iq__etags_dn0;
        *var_fn61_calc_iq__etags_dn1_slot = var_fn61_calc_iq__etags_dn1;
        *var_fn61_calc_iq__etags_dn10_slot = var_fn61_calc_iq__etags_dn10;
        *var_fn61_calc_iq__etags_dn11_slot = var_fn61_calc_iq__etags_dn11;
        *var_fn61_calc_iq__etags_dn12_slot = var_fn61_calc_iq__etags_dn12;
        *var_fn61_calc_iq__etags_dn13_slot = var_fn61_calc_iq__etags_dn13;
        *var_fn61_calc_iq__etags_dn14_slot = var_fn61_calc_iq__etags_dn14;
        *var_fn61_calc_iq__etags_dn15_slot = var_fn61_calc_iq__etags_dn15;
        *var_fn61_calc_iq__etags_dn16_slot = var_fn61_calc_iq__etags_dn16;
        *var_fn61_calc_iq__etags_dn17_slot = var_fn61_calc_iq__etags_dn17;
        *var_fn61_calc_iq__etags_dn18_slot = var_fn61_calc_iq__etags_dn18;
        *var_fn61_calc_iq__etags_dn19_slot = var_fn61_calc_iq__etags_dn19;
        *var_fn61_calc_iq__etags_dn2_slot = var_fn61_calc_iq__etags_dn2;
        *var_fn61_calc_iq__etags_dn20_slot = var_fn61_calc_iq__etags_dn20;
        *var_fn61_calc_iq__etags_dn21_slot = var_fn61_calc_iq__etags_dn21;
        *var_fn61_calc_iq__etags_dn22_slot = var_fn61_calc_iq__etags_dn22;
        *var_fn61_calc_iq__etags_dn23_slot = var_fn61_calc_iq__etags_dn23;
        *var_fn61_calc_iq__etags_dn24_slot = var_fn61_calc_iq__etags_dn24;
        *var_fn61_calc_iq__etags_dn25_slot = var_fn61_calc_iq__etags_dn25;
        *var_fn61_calc_iq__etags_dn26_slot = var_fn61_calc_iq__etags_dn26;
        *var_fn61_calc_iq__etags_dn27_slot = var_fn61_calc_iq__etags_dn27;
        *var_fn61_calc_iq__etags_dn28_slot = var_fn61_calc_iq__etags_dn28;
        *var_fn61_calc_iq__etags_dn29_slot = var_fn61_calc_iq__etags_dn29;
        *var_fn61_calc_iq__etags_dn3_slot = var_fn61_calc_iq__etags_dn3;
        *var_fn61_calc_iq__etags_dn4_slot = var_fn61_calc_iq__etags_dn4;
        *var_fn61_calc_iq__etags_dn5_slot = var_fn61_calc_iq__etags_dn5;
        *var_fn61_calc_iq__etags_dn6_slot = var_fn61_calc_iq__etags_dn6;
        *var_fn61_calc_iq__etags_dn7_slot = var_fn61_calc_iq__etags_dn7;
        *var_fn61_calc_iq__etags_dn8_slot = var_fn61_calc_iq__etags_dn8;
        *var_fn61_calc_iq__etags_dn9_slot = var_fn61_calc_iq__etags_dn9;
        *var_fn61_calc_iq__exparg_slot = var_fn61_calc_iq__exparg;
        *var_fn61_calc_iq__exparg_db0_slot = var_fn61_calc_iq__exparg_db0;
        *var_fn61_calc_iq__exparg_db1_slot = var_fn61_calc_iq__exparg_db1;
        *var_fn61_calc_iq__exparg_db10_slot = var_fn61_calc_iq__exparg_db10;
        *var_fn61_calc_iq__exparg_db11_slot = var_fn61_calc_iq__exparg_db11;
        *var_fn61_calc_iq__exparg_db12_slot = var_fn61_calc_iq__exparg_db12;
        *var_fn61_calc_iq__exparg_db13_slot = var_fn61_calc_iq__exparg_db13;
        *var_fn61_calc_iq__exparg_db14_slot = var_fn61_calc_iq__exparg_db14;
        *var_fn61_calc_iq__exparg_db15_slot = var_fn61_calc_iq__exparg_db15;
        *var_fn61_calc_iq__exparg_db16_slot = var_fn61_calc_iq__exparg_db16;
        *var_fn61_calc_iq__exparg_db17_slot = var_fn61_calc_iq__exparg_db17;
        *var_fn61_calc_iq__exparg_db18_slot = var_fn61_calc_iq__exparg_db18;
        *var_fn61_calc_iq__exparg_db19_slot = var_fn61_calc_iq__exparg_db19;
        *var_fn61_calc_iq__exparg_db2_slot = var_fn61_calc_iq__exparg_db2;
        *var_fn61_calc_iq__exparg_db20_slot = var_fn61_calc_iq__exparg_db20;
        *var_fn61_calc_iq__exparg_db21_slot = var_fn61_calc_iq__exparg_db21;
        *var_fn61_calc_iq__exparg_db22_slot = var_fn61_calc_iq__exparg_db22;
        *var_fn61_calc_iq__exparg_db23_slot = var_fn61_calc_iq__exparg_db23;
        *var_fn61_calc_iq__exparg_db24_slot = var_fn61_calc_iq__exparg_db24;
        *var_fn61_calc_iq__exparg_db25_slot = var_fn61_calc_iq__exparg_db25;
        *var_fn61_calc_iq__exparg_db26_slot = var_fn61_calc_iq__exparg_db26;
        *var_fn61_calc_iq__exparg_db27_slot = var_fn61_calc_iq__exparg_db27;
        *var_fn61_calc_iq__exparg_db28_slot = var_fn61_calc_iq__exparg_db28;
        *var_fn61_calc_iq__exparg_db29_slot = var_fn61_calc_iq__exparg_db29;
        *var_fn61_calc_iq__exparg_db3_slot = var_fn61_calc_iq__exparg_db3;
        *var_fn61_calc_iq__exparg_db30_slot = var_fn61_calc_iq__exparg_db30;
        *var_fn61_calc_iq__exparg_db31_slot = var_fn61_calc_iq__exparg_db31;
        *var_fn61_calc_iq__exparg_db32_slot = var_fn61_calc_iq__exparg_db32;
        *var_fn61_calc_iq__exparg_db33_slot = var_fn61_calc_iq__exparg_db33;
        *var_fn61_calc_iq__exparg_db34_slot = var_fn61_calc_iq__exparg_db34;
        *var_fn61_calc_iq__exparg_db35_slot = var_fn61_calc_iq__exparg_db35;
        *var_fn61_calc_iq__exparg_db4_slot = var_fn61_calc_iq__exparg_db4;
        *var_fn61_calc_iq__exparg_db5_slot = var_fn61_calc_iq__exparg_db5;
        *var_fn61_calc_iq__exparg_db6_slot = var_fn61_calc_iq__exparg_db6;
        *var_fn61_calc_iq__exparg_db7_slot = var_fn61_calc_iq__exparg_db7;
        *var_fn61_calc_iq__exparg_db8_slot = var_fn61_calc_iq__exparg_db8;
        *var_fn61_calc_iq__exparg_db9_slot = var_fn61_calc_iq__exparg_db9;
        *var_fn61_calc_iq__exparg_dn0_slot = var_fn61_calc_iq__exparg_dn0;
        *var_fn61_calc_iq__exparg_dn1_slot = var_fn61_calc_iq__exparg_dn1;
        *var_fn61_calc_iq__exparg_dn10_slot = var_fn61_calc_iq__exparg_dn10;
        *var_fn61_calc_iq__exparg_dn11_slot = var_fn61_calc_iq__exparg_dn11;
        *var_fn61_calc_iq__exparg_dn12_slot = var_fn61_calc_iq__exparg_dn12;
        *var_fn61_calc_iq__exparg_dn13_slot = var_fn61_calc_iq__exparg_dn13;
        *var_fn61_calc_iq__exparg_dn14_slot = var_fn61_calc_iq__exparg_dn14;
        *var_fn61_calc_iq__exparg_dn15_slot = var_fn61_calc_iq__exparg_dn15;
        *var_fn61_calc_iq__exparg_dn16_slot = var_fn61_calc_iq__exparg_dn16;
        *var_fn61_calc_iq__exparg_dn17_slot = var_fn61_calc_iq__exparg_dn17;
        *var_fn61_calc_iq__exparg_dn18_slot = var_fn61_calc_iq__exparg_dn18;
        *var_fn61_calc_iq__exparg_dn19_slot = var_fn61_calc_iq__exparg_dn19;
        *var_fn61_calc_iq__exparg_dn2_slot = var_fn61_calc_iq__exparg_dn2;
        *var_fn61_calc_iq__exparg_dn20_slot = var_fn61_calc_iq__exparg_dn20;
        *var_fn61_calc_iq__exparg_dn21_slot = var_fn61_calc_iq__exparg_dn21;
        *var_fn61_calc_iq__exparg_dn22_slot = var_fn61_calc_iq__exparg_dn22;
        *var_fn61_calc_iq__exparg_dn23_slot = var_fn61_calc_iq__exparg_dn23;
        *var_fn61_calc_iq__exparg_dn24_slot = var_fn61_calc_iq__exparg_dn24;
        *var_fn61_calc_iq__exparg_dn25_slot = var_fn61_calc_iq__exparg_dn25;
        *var_fn61_calc_iq__exparg_dn26_slot = var_fn61_calc_iq__exparg_dn26;
        *var_fn61_calc_iq__exparg_dn27_slot = var_fn61_calc_iq__exparg_dn27;
        *var_fn61_calc_iq__exparg_dn28_slot = var_fn61_calc_iq__exparg_dn28;
        *var_fn61_calc_iq__exparg_dn29_slot = var_fn61_calc_iq__exparg_dn29;
        *var_fn61_calc_iq__exparg_dn3_slot = var_fn61_calc_iq__exparg_dn3;
        *var_fn61_calc_iq__exparg_dn4_slot = var_fn61_calc_iq__exparg_dn4;
        *var_fn61_calc_iq__exparg_dn5_slot = var_fn61_calc_iq__exparg_dn5;
        *var_fn61_calc_iq__exparg_dn6_slot = var_fn61_calc_iq__exparg_dn6;
        *var_fn61_calc_iq__exparg_dn7_slot = var_fn61_calc_iq__exparg_dn7;
        *var_fn61_calc_iq__exparg_dn8_slot = var_fn61_calc_iq__exparg_dn8;
        *var_fn61_calc_iq__exparg_dn9_slot = var_fn61_calc_iq__exparg_dn9;
        *var_fn61_calc_iq__qbout_slot = var_fn61_calc_iq__qbout;
        *var_fn61_calc_iq__qbout_db0_slot = var_fn61_calc_iq__qbout_db0;
        *var_fn61_calc_iq__qbout_db1_slot = var_fn61_calc_iq__qbout_db1;
        *var_fn61_calc_iq__qbout_db10_slot = var_fn61_calc_iq__qbout_db10;
        *var_fn61_calc_iq__qbout_db11_slot = var_fn61_calc_iq__qbout_db11;
        *var_fn61_calc_iq__qbout_db12_slot = var_fn61_calc_iq__qbout_db12;
        *var_fn61_calc_iq__qbout_db13_slot = var_fn61_calc_iq__qbout_db13;
        *var_fn61_calc_iq__qbout_db14_slot = var_fn61_calc_iq__qbout_db14;
        *var_fn61_calc_iq__qbout_db15_slot = var_fn61_calc_iq__qbout_db15;
        *var_fn61_calc_iq__qbout_db16_slot = var_fn61_calc_iq__qbout_db16;
        *var_fn61_calc_iq__qbout_db17_slot = var_fn61_calc_iq__qbout_db17;
        *var_fn61_calc_iq__qbout_db18_slot = var_fn61_calc_iq__qbout_db18;
        *var_fn61_calc_iq__qbout_db19_slot = var_fn61_calc_iq__qbout_db19;
        *var_fn61_calc_iq__qbout_db2_slot = var_fn61_calc_iq__qbout_db2;
        *var_fn61_calc_iq__qbout_db20_slot = var_fn61_calc_iq__qbout_db20;
        *var_fn61_calc_iq__qbout_db21_slot = var_fn61_calc_iq__qbout_db21;
        *var_fn61_calc_iq__qbout_db22_slot = var_fn61_calc_iq__qbout_db22;
        *var_fn61_calc_iq__qbout_db23_slot = var_fn61_calc_iq__qbout_db23;
        *var_fn61_calc_iq__qbout_db24_slot = var_fn61_calc_iq__qbout_db24;
        *var_fn61_calc_iq__qbout_db25_slot = var_fn61_calc_iq__qbout_db25;
        *var_fn61_calc_iq__qbout_db26_slot = var_fn61_calc_iq__qbout_db26;
        *var_fn61_calc_iq__qbout_db27_slot = var_fn61_calc_iq__qbout_db27;
        *var_fn61_calc_iq__qbout_db28_slot = var_fn61_calc_iq__qbout_db28;
        *var_fn61_calc_iq__qbout_db29_slot = var_fn61_calc_iq__qbout_db29;
        *var_fn61_calc_iq__qbout_db3_slot = var_fn61_calc_iq__qbout_db3;
        *var_fn61_calc_iq__qbout_db30_slot = var_fn61_calc_iq__qbout_db30;
        *var_fn61_calc_iq__qbout_db31_slot = var_fn61_calc_iq__qbout_db31;
        *var_fn61_calc_iq__qbout_db32_slot = var_fn61_calc_iq__qbout_db32;
        *var_fn61_calc_iq__qbout_db33_slot = var_fn61_calc_iq__qbout_db33;
        *var_fn61_calc_iq__qbout_db34_slot = var_fn61_calc_iq__qbout_db34;
        *var_fn61_calc_iq__qbout_db35_slot = var_fn61_calc_iq__qbout_db35;
        *var_fn61_calc_iq__qbout_db4_slot = var_fn61_calc_iq__qbout_db4;
        *var_fn61_calc_iq__qbout_db5_slot = var_fn61_calc_iq__qbout_db5;
        *var_fn61_calc_iq__qbout_db6_slot = var_fn61_calc_iq__qbout_db6;
        *var_fn61_calc_iq__qbout_db7_slot = var_fn61_calc_iq__qbout_db7;
        *var_fn61_calc_iq__qbout_db8_slot = var_fn61_calc_iq__qbout_db8;
        *var_fn61_calc_iq__qbout_db9_slot = var_fn61_calc_iq__qbout_db9;
        *var_fn61_calc_iq__qbout_dn0_slot = var_fn61_calc_iq__qbout_dn0;
        *var_fn61_calc_iq__qbout_dn1_slot = var_fn61_calc_iq__qbout_dn1;
        *var_fn61_calc_iq__qbout_dn10_slot = var_fn61_calc_iq__qbout_dn10;
        *var_fn61_calc_iq__qbout_dn11_slot = var_fn61_calc_iq__qbout_dn11;
        *var_fn61_calc_iq__qbout_dn12_slot = var_fn61_calc_iq__qbout_dn12;
        *var_fn61_calc_iq__qbout_dn13_slot = var_fn61_calc_iq__qbout_dn13;
        *var_fn61_calc_iq__qbout_dn14_slot = var_fn61_calc_iq__qbout_dn14;
        *var_fn61_calc_iq__qbout_dn15_slot = var_fn61_calc_iq__qbout_dn15;
        *var_fn61_calc_iq__qbout_dn16_slot = var_fn61_calc_iq__qbout_dn16;
        *var_fn61_calc_iq__qbout_dn17_slot = var_fn61_calc_iq__qbout_dn17;
        *var_fn61_calc_iq__qbout_dn18_slot = var_fn61_calc_iq__qbout_dn18;
        *var_fn61_calc_iq__qbout_dn19_slot = var_fn61_calc_iq__qbout_dn19;
        *var_fn61_calc_iq__qbout_dn2_slot = var_fn61_calc_iq__qbout_dn2;
        *var_fn61_calc_iq__qbout_dn20_slot = var_fn61_calc_iq__qbout_dn20;
        *var_fn61_calc_iq__qbout_dn21_slot = var_fn61_calc_iq__qbout_dn21;
        *var_fn61_calc_iq__qbout_dn22_slot = var_fn61_calc_iq__qbout_dn22;
        *var_fn61_calc_iq__qbout_dn23_slot = var_fn61_calc_iq__qbout_dn23;
        *var_fn61_calc_iq__qbout_dn24_slot = var_fn61_calc_iq__qbout_dn24;
        *var_fn61_calc_iq__qbout_dn25_slot = var_fn61_calc_iq__qbout_dn25;
        *var_fn61_calc_iq__qbout_dn26_slot = var_fn61_calc_iq__qbout_dn26;
        *var_fn61_calc_iq__qbout_dn27_slot = var_fn61_calc_iq__qbout_dn27;
        *var_fn61_calc_iq__qbout_dn28_slot = var_fn61_calc_iq__qbout_dn28;
        *var_fn61_calc_iq__qbout_dn29_slot = var_fn61_calc_iq__qbout_dn29;
        *var_fn61_calc_iq__qbout_dn3_slot = var_fn61_calc_iq__qbout_dn3;
        *var_fn61_calc_iq__qbout_dn4_slot = var_fn61_calc_iq__qbout_dn4;
        *var_fn61_calc_iq__qbout_dn5_slot = var_fn61_calc_iq__qbout_dn5;
        *var_fn61_calc_iq__qbout_dn6_slot = var_fn61_calc_iq__qbout_dn6;
        *var_fn61_calc_iq__qbout_dn7_slot = var_fn61_calc_iq__qbout_dn7;
        *var_fn61_calc_iq__qbout_dn8_slot = var_fn61_calc_iq__qbout_dn8;
        *var_fn61_calc_iq__qbout_dn9_slot = var_fn61_calc_iq__qbout_dn9;
        *var_fn61_calc_iq__qcout_slot = var_fn61_calc_iq__qcout;
        *var_fn61_calc_iq__qcout_db0_slot = var_fn61_calc_iq__qcout_db0;
        *var_fn61_calc_iq__qcout_db1_slot = var_fn61_calc_iq__qcout_db1;
        *var_fn61_calc_iq__qcout_db10_slot = var_fn61_calc_iq__qcout_db10;
        *var_fn61_calc_iq__qcout_db11_slot = var_fn61_calc_iq__qcout_db11;
        *var_fn61_calc_iq__qcout_db12_slot = var_fn61_calc_iq__qcout_db12;
        *var_fn61_calc_iq__qcout_db13_slot = var_fn61_calc_iq__qcout_db13;
        *var_fn61_calc_iq__qcout_db14_slot = var_fn61_calc_iq__qcout_db14;
        *var_fn61_calc_iq__qcout_db15_slot = var_fn61_calc_iq__qcout_db15;
        *var_fn61_calc_iq__qcout_db16_slot = var_fn61_calc_iq__qcout_db16;
        *var_fn61_calc_iq__qcout_db17_slot = var_fn61_calc_iq__qcout_db17;
        *var_fn61_calc_iq__qcout_db18_slot = var_fn61_calc_iq__qcout_db18;
        *var_fn61_calc_iq__qcout_db19_slot = var_fn61_calc_iq__qcout_db19;
        *var_fn61_calc_iq__qcout_db2_slot = var_fn61_calc_iq__qcout_db2;
        *var_fn61_calc_iq__qcout_db20_slot = var_fn61_calc_iq__qcout_db20;
        *var_fn61_calc_iq__qcout_db21_slot = var_fn61_calc_iq__qcout_db21;
        *var_fn61_calc_iq__qcout_db22_slot = var_fn61_calc_iq__qcout_db22;
        *var_fn61_calc_iq__qcout_db23_slot = var_fn61_calc_iq__qcout_db23;
        *var_fn61_calc_iq__qcout_db24_slot = var_fn61_calc_iq__qcout_db24;
        *var_fn61_calc_iq__qcout_db25_slot = var_fn61_calc_iq__qcout_db25;
        *var_fn61_calc_iq__qcout_db26_slot = var_fn61_calc_iq__qcout_db26;
        *var_fn61_calc_iq__qcout_db27_slot = var_fn61_calc_iq__qcout_db27;
        *var_fn61_calc_iq__qcout_db28_slot = var_fn61_calc_iq__qcout_db28;
        *var_fn61_calc_iq__qcout_db29_slot = var_fn61_calc_iq__qcout_db29;
        *var_fn61_calc_iq__qcout_db3_slot = var_fn61_calc_iq__qcout_db3;
        *var_fn61_calc_iq__qcout_db30_slot = var_fn61_calc_iq__qcout_db30;
        *var_fn61_calc_iq__qcout_db31_slot = var_fn61_calc_iq__qcout_db31;
        *var_fn61_calc_iq__qcout_db32_slot = var_fn61_calc_iq__qcout_db32;
        *var_fn61_calc_iq__qcout_db33_slot = var_fn61_calc_iq__qcout_db33;
        *var_fn61_calc_iq__qcout_db34_slot = var_fn61_calc_iq__qcout_db34;
        *var_fn61_calc_iq__qcout_db35_slot = var_fn61_calc_iq__qcout_db35;
        *var_fn61_calc_iq__qcout_db4_slot = var_fn61_calc_iq__qcout_db4;
        *var_fn61_calc_iq__qcout_db5_slot = var_fn61_calc_iq__qcout_db5;
        *var_fn61_calc_iq__qcout_db6_slot = var_fn61_calc_iq__qcout_db6;
        *var_fn61_calc_iq__qcout_db7_slot = var_fn61_calc_iq__qcout_db7;
        *var_fn61_calc_iq__qcout_db8_slot = var_fn61_calc_iq__qcout_db8;
        *var_fn61_calc_iq__qcout_db9_slot = var_fn61_calc_iq__qcout_db9;
        *var_fn61_calc_iq__qcout_dn0_slot = var_fn61_calc_iq__qcout_dn0;
        *var_fn61_calc_iq__qcout_dn1_slot = var_fn61_calc_iq__qcout_dn1;
        *var_fn61_calc_iq__qcout_dn10_slot = var_fn61_calc_iq__qcout_dn10;
        *var_fn61_calc_iq__qcout_dn11_slot = var_fn61_calc_iq__qcout_dn11;
        *var_fn61_calc_iq__qcout_dn12_slot = var_fn61_calc_iq__qcout_dn12;
        *var_fn61_calc_iq__qcout_dn13_slot = var_fn61_calc_iq__qcout_dn13;
        *var_fn61_calc_iq__qcout_dn14_slot = var_fn61_calc_iq__qcout_dn14;
        *var_fn61_calc_iq__qcout_dn15_slot = var_fn61_calc_iq__qcout_dn15;
        *var_fn61_calc_iq__qcout_dn16_slot = var_fn61_calc_iq__qcout_dn16;
        *var_fn61_calc_iq__qcout_dn17_slot = var_fn61_calc_iq__qcout_dn17;
        *var_fn61_calc_iq__qcout_dn18_slot = var_fn61_calc_iq__qcout_dn18;
        *var_fn61_calc_iq__qcout_dn19_slot = var_fn61_calc_iq__qcout_dn19;
        *var_fn61_calc_iq__qcout_dn2_slot = var_fn61_calc_iq__qcout_dn2;
        *var_fn61_calc_iq__qcout_dn20_slot = var_fn61_calc_iq__qcout_dn20;
        *var_fn61_calc_iq__qcout_dn21_slot = var_fn61_calc_iq__qcout_dn21;
        *var_fn61_calc_iq__qcout_dn22_slot = var_fn61_calc_iq__qcout_dn22;
        *var_fn61_calc_iq__qcout_dn23_slot = var_fn61_calc_iq__qcout_dn23;
        *var_fn61_calc_iq__qcout_dn24_slot = var_fn61_calc_iq__qcout_dn24;
        *var_fn61_calc_iq__qcout_dn25_slot = var_fn61_calc_iq__qcout_dn25;
        *var_fn61_calc_iq__qcout_dn26_slot = var_fn61_calc_iq__qcout_dn26;
        *var_fn61_calc_iq__qcout_dn27_slot = var_fn61_calc_iq__qcout_dn27;
        *var_fn61_calc_iq__qcout_dn28_slot = var_fn61_calc_iq__qcout_dn28;
        *var_fn61_calc_iq__qcout_dn29_slot = var_fn61_calc_iq__qcout_dn29;
        *var_fn61_calc_iq__qcout_dn3_slot = var_fn61_calc_iq__qcout_dn3;
        *var_fn61_calc_iq__qcout_dn4_slot = var_fn61_calc_iq__qcout_dn4;
        *var_fn61_calc_iq__qcout_dn5_slot = var_fn61_calc_iq__qcout_dn5;
        *var_fn61_calc_iq__qcout_dn6_slot = var_fn61_calc_iq__qcout_dn6;
        *var_fn61_calc_iq__qcout_dn7_slot = var_fn61_calc_iq__qcout_dn7;
        *var_fn61_calc_iq__qcout_dn8_slot = var_fn61_calc_iq__qcout_dn8;
        *var_fn61_calc_iq__qcout_dn9_slot = var_fn61_calc_iq__qcout_dn9;
        *var_guard92_slot = var_guard92;
        *var_guard93_slot = var_guard93;
    }

    pub(super) fn stamp_transient_block_81(
        var_fn61_calc_iq__cs: f64,
        var_fn61_calc_iq__etags: f64,
        var_fn61_calc_iq__etags_db0: f64,
        var_fn61_calc_iq__etags_db1: f64,
        var_fn61_calc_iq__etags_db10: f64,
        var_fn61_calc_iq__etags_db11: f64,
        var_fn61_calc_iq__etags_db12: f64,
        var_fn61_calc_iq__etags_db13: f64,
        var_fn61_calc_iq__etags_db14: f64,
        var_fn61_calc_iq__etags_db15: f64,
        var_fn61_calc_iq__etags_db16: f64,
        var_fn61_calc_iq__etags_db17: f64,
        var_fn61_calc_iq__etags_db18: f64,
        var_fn61_calc_iq__etags_db19: f64,
        var_fn61_calc_iq__etags_db2: f64,
        var_fn61_calc_iq__etags_db20: f64,
        var_fn61_calc_iq__etags_db21: f64,
        var_fn61_calc_iq__etags_db22: f64,
        var_fn61_calc_iq__etags_db23: f64,
        var_fn61_calc_iq__etags_db24: f64,
        var_fn61_calc_iq__etags_db25: f64,
        var_fn61_calc_iq__etags_db26: f64,
        var_fn61_calc_iq__etags_db27: f64,
        var_fn61_calc_iq__etags_db28: f64,
        var_fn61_calc_iq__etags_db29: f64,
        var_fn61_calc_iq__etags_db3: f64,
        var_fn61_calc_iq__etags_db30: f64,
        var_fn61_calc_iq__etags_db31: f64,
        var_fn61_calc_iq__etags_db32: f64,
        var_fn61_calc_iq__etags_db33: f64,
        var_fn61_calc_iq__etags_db34: f64,
        var_fn61_calc_iq__etags_db35: f64,
        var_fn61_calc_iq__etags_db4: f64,
        var_fn61_calc_iq__etags_db5: f64,
        var_fn61_calc_iq__etags_db6: f64,
        var_fn61_calc_iq__etags_db7: f64,
        var_fn61_calc_iq__etags_db8: f64,
        var_fn61_calc_iq__etags_db9: f64,
        var_fn61_calc_iq__etags_dn0: f64,
        var_fn61_calc_iq__etags_dn1: f64,
        var_fn61_calc_iq__etags_dn10: f64,
        var_fn61_calc_iq__etags_dn11: f64,
        var_fn61_calc_iq__etags_dn12: f64,
        var_fn61_calc_iq__etags_dn13: f64,
        var_fn61_calc_iq__etags_dn14: f64,
        var_fn61_calc_iq__etags_dn15: f64,
        var_fn61_calc_iq__etags_dn16: f64,
        var_fn61_calc_iq__etags_dn17: f64,
        var_fn61_calc_iq__etags_dn18: f64,
        var_fn61_calc_iq__etags_dn19: f64,
        var_fn61_calc_iq__etags_dn2: f64,
        var_fn61_calc_iq__etags_dn20: f64,
        var_fn61_calc_iq__etags_dn21: f64,
        var_fn61_calc_iq__etags_dn22: f64,
        var_fn61_calc_iq__etags_dn23: f64,
        var_fn61_calc_iq__etags_dn24: f64,
        var_fn61_calc_iq__etags_dn25: f64,
        var_fn61_calc_iq__etags_dn26: f64,
        var_fn61_calc_iq__etags_dn27: f64,
        var_fn61_calc_iq__etags_dn28: f64,
        var_fn61_calc_iq__etags_dn29: f64,
        var_fn61_calc_iq__etags_dn3: f64,
        var_fn61_calc_iq__etags_dn4: f64,
        var_fn61_calc_iq__etags_dn5: f64,
        var_fn61_calc_iq__etags_dn6: f64,
        var_fn61_calc_iq__etags_dn7: f64,
        var_fn61_calc_iq__etags_dn8: f64,
        var_fn61_calc_iq__etags_dn9: f64,
        var_fn61_calc_iq__idsout: f64,
        var_fn61_calc_iq__idsout_db0: f64,
        var_fn61_calc_iq__idsout_db1: f64,
        var_fn61_calc_iq__idsout_db10: f64,
        var_fn61_calc_iq__idsout_db11: f64,
        var_fn61_calc_iq__idsout_db12: f64,
        var_fn61_calc_iq__idsout_db13: f64,
        var_fn61_calc_iq__idsout_db14: f64,
        var_fn61_calc_iq__idsout_db15: f64,
        var_fn61_calc_iq__idsout_db16: f64,
        var_fn61_calc_iq__idsout_db17: f64,
        var_fn61_calc_iq__idsout_db18: f64,
        var_fn61_calc_iq__idsout_db19: f64,
        var_fn61_calc_iq__idsout_db2: f64,
        var_fn61_calc_iq__idsout_db20: f64,
        var_fn61_calc_iq__idsout_db21: f64,
        var_fn61_calc_iq__idsout_db22: f64,
        var_fn61_calc_iq__idsout_db23: f64,
        var_fn61_calc_iq__idsout_db24: f64,
        var_fn61_calc_iq__idsout_db25: f64,
        var_fn61_calc_iq__idsout_db26: f64,
        var_fn61_calc_iq__idsout_db27: f64,
        var_fn61_calc_iq__idsout_db28: f64,
        var_fn61_calc_iq__idsout_db29: f64,
        var_fn61_calc_iq__idsout_db3: f64,
        var_fn61_calc_iq__idsout_db30: f64,
        var_fn61_calc_iq__idsout_db31: f64,
        var_fn61_calc_iq__idsout_db32: f64,
        var_fn61_calc_iq__idsout_db33: f64,
        var_fn61_calc_iq__idsout_db34: f64,
        var_fn61_calc_iq__idsout_db35: f64,
        var_fn61_calc_iq__idsout_db4: f64,
        var_fn61_calc_iq__idsout_db5: f64,
        var_fn61_calc_iq__idsout_db6: f64,
        var_fn61_calc_iq__idsout_db7: f64,
        var_fn61_calc_iq__idsout_db8: f64,
        var_fn61_calc_iq__idsout_db9: f64,
        var_fn61_calc_iq__idsout_dn0: f64,
        var_fn61_calc_iq__idsout_dn1: f64,
        var_fn61_calc_iq__idsout_dn10: f64,
        var_fn61_calc_iq__idsout_dn11: f64,
        var_fn61_calc_iq__idsout_dn12: f64,
        var_fn61_calc_iq__idsout_dn13: f64,
        var_fn61_calc_iq__idsout_dn14: f64,
        var_fn61_calc_iq__idsout_dn15: f64,
        var_fn61_calc_iq__idsout_dn16: f64,
        var_fn61_calc_iq__idsout_dn17: f64,
        var_fn61_calc_iq__idsout_dn18: f64,
        var_fn61_calc_iq__idsout_dn19: f64,
        var_fn61_calc_iq__idsout_dn2: f64,
        var_fn61_calc_iq__idsout_dn20: f64,
        var_fn61_calc_iq__idsout_dn21: f64,
        var_fn61_calc_iq__idsout_dn22: f64,
        var_fn61_calc_iq__idsout_dn23: f64,
        var_fn61_calc_iq__idsout_dn24: f64,
        var_fn61_calc_iq__idsout_dn25: f64,
        var_fn61_calc_iq__idsout_dn26: f64,
        var_fn61_calc_iq__idsout_dn27: f64,
        var_fn61_calc_iq__idsout_dn28: f64,
        var_fn61_calc_iq__idsout_dn29: f64,
        var_fn61_calc_iq__idsout_dn3: f64,
        var_fn61_calc_iq__idsout_dn4: f64,
        var_fn61_calc_iq__idsout_dn5: f64,
        var_fn61_calc_iq__idsout_dn6: f64,
        var_fn61_calc_iq__idsout_dn7: f64,
        var_fn61_calc_iq__idsout_dn8: f64,
        var_fn61_calc_iq__idsout_dn9: f64,
        var_fn61_calc_iq__ngf: f64,
        var_fn61_calc_iq__trapfracdl: f64,
        var_fn61_calc_iq__two_n_phit0: f64,
        var_fn61_calc_iq__two_n_phit0_db0: f64,
        var_fn61_calc_iq__two_n_phit0_db1: f64,
        var_fn61_calc_iq__two_n_phit0_db10: f64,
        var_fn61_calc_iq__two_n_phit0_db11: f64,
        var_fn61_calc_iq__two_n_phit0_db12: f64,
        var_fn61_calc_iq__two_n_phit0_db13: f64,
        var_fn61_calc_iq__two_n_phit0_db14: f64,
        var_fn61_calc_iq__two_n_phit0_db15: f64,
        var_fn61_calc_iq__two_n_phit0_db16: f64,
        var_fn61_calc_iq__two_n_phit0_db17: f64,
        var_fn61_calc_iq__two_n_phit0_db18: f64,
        var_fn61_calc_iq__two_n_phit0_db19: f64,
        var_fn61_calc_iq__two_n_phit0_db2: f64,
        var_fn61_calc_iq__two_n_phit0_db20: f64,
        var_fn61_calc_iq__two_n_phit0_db21: f64,
        var_fn61_calc_iq__two_n_phit0_db22: f64,
        var_fn61_calc_iq__two_n_phit0_db23: f64,
        var_fn61_calc_iq__two_n_phit0_db24: f64,
        var_fn61_calc_iq__two_n_phit0_db25: f64,
        var_fn61_calc_iq__two_n_phit0_db26: f64,
        var_fn61_calc_iq__two_n_phit0_db27: f64,
        var_fn61_calc_iq__two_n_phit0_db28: f64,
        var_fn61_calc_iq__two_n_phit0_db29: f64,
        var_fn61_calc_iq__two_n_phit0_db3: f64,
        var_fn61_calc_iq__two_n_phit0_db30: f64,
        var_fn61_calc_iq__two_n_phit0_db31: f64,
        var_fn61_calc_iq__two_n_phit0_db32: f64,
        var_fn61_calc_iq__two_n_phit0_db33: f64,
        var_fn61_calc_iq__two_n_phit0_db34: f64,
        var_fn61_calc_iq__two_n_phit0_db35: f64,
        var_fn61_calc_iq__two_n_phit0_db4: f64,
        var_fn61_calc_iq__two_n_phit0_db5: f64,
        var_fn61_calc_iq__two_n_phit0_db6: f64,
        var_fn61_calc_iq__two_n_phit0_db7: f64,
        var_fn61_calc_iq__two_n_phit0_db8: f64,
        var_fn61_calc_iq__two_n_phit0_db9: f64,
        var_fn61_calc_iq__two_n_phit0_dn0: f64,
        var_fn61_calc_iq__two_n_phit0_dn1: f64,
        var_fn61_calc_iq__two_n_phit0_dn10: f64,
        var_fn61_calc_iq__two_n_phit0_dn11: f64,
        var_fn61_calc_iq__two_n_phit0_dn12: f64,
        var_fn61_calc_iq__two_n_phit0_dn13: f64,
        var_fn61_calc_iq__two_n_phit0_dn14: f64,
        var_fn61_calc_iq__two_n_phit0_dn15: f64,
        var_fn61_calc_iq__two_n_phit0_dn16: f64,
        var_fn61_calc_iq__two_n_phit0_dn17: f64,
        var_fn61_calc_iq__two_n_phit0_dn18: f64,
        var_fn61_calc_iq__two_n_phit0_dn19: f64,
        var_fn61_calc_iq__two_n_phit0_dn2: f64,
        var_fn61_calc_iq__two_n_phit0_dn20: f64,
        var_fn61_calc_iq__two_n_phit0_dn21: f64,
        var_fn61_calc_iq__two_n_phit0_dn22: f64,
        var_fn61_calc_iq__two_n_phit0_dn23: f64,
        var_fn61_calc_iq__two_n_phit0_dn24: f64,
        var_fn61_calc_iq__two_n_phit0_dn25: f64,
        var_fn61_calc_iq__two_n_phit0_dn26: f64,
        var_fn61_calc_iq__two_n_phit0_dn27: f64,
        var_fn61_calc_iq__two_n_phit0_dn28: f64,
        var_fn61_calc_iq__two_n_phit0_dn29: f64,
        var_fn61_calc_iq__two_n_phit0_dn3: f64,
        var_fn61_calc_iq__two_n_phit0_dn4: f64,
        var_fn61_calc_iq__two_n_phit0_dn5: f64,
        var_fn61_calc_iq__two_n_phit0_dn6: f64,
        var_fn61_calc_iq__two_n_phit0_dn7: f64,
        var_fn61_calc_iq__two_n_phit0_dn8: f64,
        var_fn61_calc_iq__two_n_phit0_dn9: f64,
        var_fn61_calc_iq__type: f64,
        var_fn61_calc_iq__w: f64,
        var_guard60: f64,
        var_guard92: f64,
        var_guard93: f64,
        var_fn61_calc_iq__exparg_slot: &mut f64,
        var_fn61_calc_iq__exparg_db0_slot: &mut f64,
        var_fn61_calc_iq__exparg_db1_slot: &mut f64,
        var_fn61_calc_iq__exparg_db10_slot: &mut f64,
        var_fn61_calc_iq__exparg_db11_slot: &mut f64,
        var_fn61_calc_iq__exparg_db12_slot: &mut f64,
        var_fn61_calc_iq__exparg_db13_slot: &mut f64,
        var_fn61_calc_iq__exparg_db14_slot: &mut f64,
        var_fn61_calc_iq__exparg_db15_slot: &mut f64,
        var_fn61_calc_iq__exparg_db16_slot: &mut f64,
        var_fn61_calc_iq__exparg_db17_slot: &mut f64,
        var_fn61_calc_iq__exparg_db18_slot: &mut f64,
        var_fn61_calc_iq__exparg_db19_slot: &mut f64,
        var_fn61_calc_iq__exparg_db2_slot: &mut f64,
        var_fn61_calc_iq__exparg_db20_slot: &mut f64,
        var_fn61_calc_iq__exparg_db21_slot: &mut f64,
        var_fn61_calc_iq__exparg_db22_slot: &mut f64,
        var_fn61_calc_iq__exparg_db23_slot: &mut f64,
        var_fn61_calc_iq__exparg_db24_slot: &mut f64,
        var_fn61_calc_iq__exparg_db25_slot: &mut f64,
        var_fn61_calc_iq__exparg_db26_slot: &mut f64,
        var_fn61_calc_iq__exparg_db27_slot: &mut f64,
        var_fn61_calc_iq__exparg_db28_slot: &mut f64,
        var_fn61_calc_iq__exparg_db29_slot: &mut f64,
        var_fn61_calc_iq__exparg_db3_slot: &mut f64,
        var_fn61_calc_iq__exparg_db30_slot: &mut f64,
        var_fn61_calc_iq__exparg_db31_slot: &mut f64,
        var_fn61_calc_iq__exparg_db32_slot: &mut f64,
        var_fn61_calc_iq__exparg_db33_slot: &mut f64,
        var_fn61_calc_iq__exparg_db34_slot: &mut f64,
        var_fn61_calc_iq__exparg_db35_slot: &mut f64,
        var_fn61_calc_iq__exparg_db4_slot: &mut f64,
        var_fn61_calc_iq__exparg_db5_slot: &mut f64,
        var_fn61_calc_iq__exparg_db6_slot: &mut f64,
        var_fn61_calc_iq__exparg_db7_slot: &mut f64,
        var_fn61_calc_iq__exparg_db8_slot: &mut f64,
        var_fn61_calc_iq__exparg_db9_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn0_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn1_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn10_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn11_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn12_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn13_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn14_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn15_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn16_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn17_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn18_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn19_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn2_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn20_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn21_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn22_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn23_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn24_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn25_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn26_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn27_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn28_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn29_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn3_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn4_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn5_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn6_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn7_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn8_slot: &mut f64,
        var_fn61_calc_iq__exparg_dn9_slot: &mut f64,
        var_fn61_calc_iq__qsout_slot: &mut f64,
        var_fn61_calc_iq__qsout_db0_slot: &mut f64,
        var_fn61_calc_iq__qsout_db1_slot: &mut f64,
        var_fn61_calc_iq__qsout_db10_slot: &mut f64,
        var_fn61_calc_iq__qsout_db11_slot: &mut f64,
        var_fn61_calc_iq__qsout_db12_slot: &mut f64,
        var_fn61_calc_iq__qsout_db13_slot: &mut f64,
        var_fn61_calc_iq__qsout_db14_slot: &mut f64,
        var_fn61_calc_iq__qsout_db15_slot: &mut f64,
        var_fn61_calc_iq__qsout_db16_slot: &mut f64,
        var_fn61_calc_iq__qsout_db17_slot: &mut f64,
        var_fn61_calc_iq__qsout_db18_slot: &mut f64,
        var_fn61_calc_iq__qsout_db19_slot: &mut f64,
        var_fn61_calc_iq__qsout_db2_slot: &mut f64,
        var_fn61_calc_iq__qsout_db20_slot: &mut f64,
        var_fn61_calc_iq__qsout_db21_slot: &mut f64,
        var_fn61_calc_iq__qsout_db22_slot: &mut f64,
        var_fn61_calc_iq__qsout_db23_slot: &mut f64,
        var_fn61_calc_iq__qsout_db24_slot: &mut f64,
        var_fn61_calc_iq__qsout_db25_slot: &mut f64,
        var_fn61_calc_iq__qsout_db26_slot: &mut f64,
        var_fn61_calc_iq__qsout_db27_slot: &mut f64,
        var_fn61_calc_iq__qsout_db28_slot: &mut f64,
        var_fn61_calc_iq__qsout_db29_slot: &mut f64,
        var_fn61_calc_iq__qsout_db3_slot: &mut f64,
        var_fn61_calc_iq__qsout_db30_slot: &mut f64,
        var_fn61_calc_iq__qsout_db31_slot: &mut f64,
        var_fn61_calc_iq__qsout_db32_slot: &mut f64,
        var_fn61_calc_iq__qsout_db33_slot: &mut f64,
        var_fn61_calc_iq__qsout_db34_slot: &mut f64,
        var_fn61_calc_iq__qsout_db35_slot: &mut f64,
        var_fn61_calc_iq__qsout_db4_slot: &mut f64,
        var_fn61_calc_iq__qsout_db5_slot: &mut f64,
        var_fn61_calc_iq__qsout_db6_slot: &mut f64,
        var_fn61_calc_iq__qsout_db7_slot: &mut f64,
        var_fn61_calc_iq__qsout_db8_slot: &mut f64,
        var_fn61_calc_iq__qsout_db9_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn0_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn1_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn10_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn11_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn12_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn13_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn14_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn15_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn16_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn17_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn18_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn19_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn2_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn20_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn21_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn22_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn23_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn24_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn25_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn26_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn27_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn28_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn29_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn3_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn4_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn5_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn6_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn7_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn8_slot: &mut f64,
        var_fn61_calc_iq__qsout_dn9_slot: &mut f64,
        var_fn61_calc_iq__return_slot: &mut f64,
        var_fn61_calc_iq__return_db0_slot: &mut f64,
        var_fn61_calc_iq__return_db1_slot: &mut f64,
        var_fn61_calc_iq__return_db10_slot: &mut f64,
        var_fn61_calc_iq__return_db11_slot: &mut f64,
        var_fn61_calc_iq__return_db12_slot: &mut f64,
        var_fn61_calc_iq__return_db13_slot: &mut f64,
        var_fn61_calc_iq__return_db14_slot: &mut f64,
        var_fn61_calc_iq__return_db15_slot: &mut f64,
        var_fn61_calc_iq__return_db16_slot: &mut f64,
        var_fn61_calc_iq__return_db17_slot: &mut f64,
        var_fn61_calc_iq__return_db18_slot: &mut f64,
        var_fn61_calc_iq__return_db19_slot: &mut f64,
        var_fn61_calc_iq__return_db2_slot: &mut f64,
        var_fn61_calc_iq__return_db20_slot: &mut f64,
        var_fn61_calc_iq__return_db21_slot: &mut f64,
        var_fn61_calc_iq__return_db22_slot: &mut f64,
        var_fn61_calc_iq__return_db23_slot: &mut f64,
        var_fn61_calc_iq__return_db24_slot: &mut f64,
        var_fn61_calc_iq__return_db25_slot: &mut f64,
        var_fn61_calc_iq__return_db26_slot: &mut f64,
        var_fn61_calc_iq__return_db27_slot: &mut f64,
        var_fn61_calc_iq__return_db28_slot: &mut f64,
        var_fn61_calc_iq__return_db29_slot: &mut f64,
        var_fn61_calc_iq__return_db3_slot: &mut f64,
        var_fn61_calc_iq__return_db30_slot: &mut f64,
        var_fn61_calc_iq__return_db31_slot: &mut f64,
        var_fn61_calc_iq__return_db32_slot: &mut f64,
        var_fn61_calc_iq__return_db33_slot: &mut f64,
        var_fn61_calc_iq__return_db34_slot: &mut f64,
        var_fn61_calc_iq__return_db35_slot: &mut f64,
        var_fn61_calc_iq__return_db4_slot: &mut f64,
        var_fn61_calc_iq__return_db5_slot: &mut f64,
        var_fn61_calc_iq__return_db6_slot: &mut f64,
        var_fn61_calc_iq__return_db7_slot: &mut f64,
        var_fn61_calc_iq__return_db8_slot: &mut f64,
        var_fn61_calc_iq__return_db9_slot: &mut f64,
        var_fn61_calc_iq__return_dn0_slot: &mut f64,
        var_fn61_calc_iq__return_dn1_slot: &mut f64,
        var_fn61_calc_iq__return_dn10_slot: &mut f64,
        var_fn61_calc_iq__return_dn11_slot: &mut f64,
        var_fn61_calc_iq__return_dn12_slot: &mut f64,
        var_fn61_calc_iq__return_dn13_slot: &mut f64,
        var_fn61_calc_iq__return_dn14_slot: &mut f64,
        var_fn61_calc_iq__return_dn15_slot: &mut f64,
        var_fn61_calc_iq__return_dn16_slot: &mut f64,
        var_fn61_calc_iq__return_dn17_slot: &mut f64,
        var_fn61_calc_iq__return_dn18_slot: &mut f64,
        var_fn61_calc_iq__return_dn19_slot: &mut f64,
        var_fn61_calc_iq__return_dn2_slot: &mut f64,
        var_fn61_calc_iq__return_dn20_slot: &mut f64,
        var_fn61_calc_iq__return_dn21_slot: &mut f64,
        var_fn61_calc_iq__return_dn22_slot: &mut f64,
        var_fn61_calc_iq__return_dn23_slot: &mut f64,
        var_fn61_calc_iq__return_dn24_slot: &mut f64,
        var_fn61_calc_iq__return_dn25_slot: &mut f64,
        var_fn61_calc_iq__return_dn26_slot: &mut f64,
        var_fn61_calc_iq__return_dn27_slot: &mut f64,
        var_fn61_calc_iq__return_dn28_slot: &mut f64,
        var_fn61_calc_iq__return_dn29_slot: &mut f64,
        var_fn61_calc_iq__return_dn3_slot: &mut f64,
        var_fn61_calc_iq__return_dn4_slot: &mut f64,
        var_fn61_calc_iq__return_dn5_slot: &mut f64,
        var_fn61_calc_iq__return_dn6_slot: &mut f64,
        var_fn61_calc_iq__return_dn7_slot: &mut f64,
        var_fn61_calc_iq__return_dn8_slot: &mut f64,
        var_fn61_calc_iq__return_dn9_slot: &mut f64,
        var_guard94_slot: &mut f64,
    ) {
        let mut var_fn61_calc_iq__exparg: f64 = *var_fn61_calc_iq__exparg_slot;
        let mut var_fn61_calc_iq__exparg_db0: f64 = *var_fn61_calc_iq__exparg_db0_slot;
        let mut var_fn61_calc_iq__exparg_db1: f64 = *var_fn61_calc_iq__exparg_db1_slot;
        let mut var_fn61_calc_iq__exparg_db10: f64 = *var_fn61_calc_iq__exparg_db10_slot;
        let mut var_fn61_calc_iq__exparg_db11: f64 = *var_fn61_calc_iq__exparg_db11_slot;
        let mut var_fn61_calc_iq__exparg_db12: f64 = *var_fn61_calc_iq__exparg_db12_slot;
        let mut var_fn61_calc_iq__exparg_db13: f64 = *var_fn61_calc_iq__exparg_db13_slot;
        let mut var_fn61_calc_iq__exparg_db14: f64 = *var_fn61_calc_iq__exparg_db14_slot;
        let mut var_fn61_calc_iq__exparg_db15: f64 = *var_fn61_calc_iq__exparg_db15_slot;
        let mut var_fn61_calc_iq__exparg_db16: f64 = *var_fn61_calc_iq__exparg_db16_slot;
        let mut var_fn61_calc_iq__exparg_db17: f64 = *var_fn61_calc_iq__exparg_db17_slot;
        let mut var_fn61_calc_iq__exparg_db18: f64 = *var_fn61_calc_iq__exparg_db18_slot;
        let mut var_fn61_calc_iq__exparg_db19: f64 = *var_fn61_calc_iq__exparg_db19_slot;
        let mut var_fn61_calc_iq__exparg_db2: f64 = *var_fn61_calc_iq__exparg_db2_slot;
        let mut var_fn61_calc_iq__exparg_db20: f64 = *var_fn61_calc_iq__exparg_db20_slot;
        let mut var_fn61_calc_iq__exparg_db21: f64 = *var_fn61_calc_iq__exparg_db21_slot;
        let mut var_fn61_calc_iq__exparg_db22: f64 = *var_fn61_calc_iq__exparg_db22_slot;
        let mut var_fn61_calc_iq__exparg_db23: f64 = *var_fn61_calc_iq__exparg_db23_slot;
        let mut var_fn61_calc_iq__exparg_db24: f64 = *var_fn61_calc_iq__exparg_db24_slot;
        let mut var_fn61_calc_iq__exparg_db25: f64 = *var_fn61_calc_iq__exparg_db25_slot;
        let mut var_fn61_calc_iq__exparg_db26: f64 = *var_fn61_calc_iq__exparg_db26_slot;
        let mut var_fn61_calc_iq__exparg_db27: f64 = *var_fn61_calc_iq__exparg_db27_slot;
        let mut var_fn61_calc_iq__exparg_db28: f64 = *var_fn61_calc_iq__exparg_db28_slot;
        let mut var_fn61_calc_iq__exparg_db29: f64 = *var_fn61_calc_iq__exparg_db29_slot;
        let mut var_fn61_calc_iq__exparg_db3: f64 = *var_fn61_calc_iq__exparg_db3_slot;
        let mut var_fn61_calc_iq__exparg_db30: f64 = *var_fn61_calc_iq__exparg_db30_slot;
        let mut var_fn61_calc_iq__exparg_db31: f64 = *var_fn61_calc_iq__exparg_db31_slot;
        let mut var_fn61_calc_iq__exparg_db32: f64 = *var_fn61_calc_iq__exparg_db32_slot;
        let mut var_fn61_calc_iq__exparg_db33: f64 = *var_fn61_calc_iq__exparg_db33_slot;
        let mut var_fn61_calc_iq__exparg_db34: f64 = *var_fn61_calc_iq__exparg_db34_slot;
        let mut var_fn61_calc_iq__exparg_db35: f64 = *var_fn61_calc_iq__exparg_db35_slot;
        let mut var_fn61_calc_iq__exparg_db4: f64 = *var_fn61_calc_iq__exparg_db4_slot;
        let mut var_fn61_calc_iq__exparg_db5: f64 = *var_fn61_calc_iq__exparg_db5_slot;
        let mut var_fn61_calc_iq__exparg_db6: f64 = *var_fn61_calc_iq__exparg_db6_slot;
        let mut var_fn61_calc_iq__exparg_db7: f64 = *var_fn61_calc_iq__exparg_db7_slot;
        let mut var_fn61_calc_iq__exparg_db8: f64 = *var_fn61_calc_iq__exparg_db8_slot;
        let mut var_fn61_calc_iq__exparg_db9: f64 = *var_fn61_calc_iq__exparg_db9_slot;
        let mut var_fn61_calc_iq__exparg_dn0: f64 = *var_fn61_calc_iq__exparg_dn0_slot;
        let mut var_fn61_calc_iq__exparg_dn1: f64 = *var_fn61_calc_iq__exparg_dn1_slot;
        let mut var_fn61_calc_iq__exparg_dn10: f64 = *var_fn61_calc_iq__exparg_dn10_slot;
        let mut var_fn61_calc_iq__exparg_dn11: f64 = *var_fn61_calc_iq__exparg_dn11_slot;
        let mut var_fn61_calc_iq__exparg_dn12: f64 = *var_fn61_calc_iq__exparg_dn12_slot;
        let mut var_fn61_calc_iq__exparg_dn13: f64 = *var_fn61_calc_iq__exparg_dn13_slot;
        let mut var_fn61_calc_iq__exparg_dn14: f64 = *var_fn61_calc_iq__exparg_dn14_slot;
        let mut var_fn61_calc_iq__exparg_dn15: f64 = *var_fn61_calc_iq__exparg_dn15_slot;
        let mut var_fn61_calc_iq__exparg_dn16: f64 = *var_fn61_calc_iq__exparg_dn16_slot;
        let mut var_fn61_calc_iq__exparg_dn17: f64 = *var_fn61_calc_iq__exparg_dn17_slot;
        let mut var_fn61_calc_iq__exparg_dn18: f64 = *var_fn61_calc_iq__exparg_dn18_slot;
        let mut var_fn61_calc_iq__exparg_dn19: f64 = *var_fn61_calc_iq__exparg_dn19_slot;
        let mut var_fn61_calc_iq__exparg_dn2: f64 = *var_fn61_calc_iq__exparg_dn2_slot;
        let mut var_fn61_calc_iq__exparg_dn20: f64 = *var_fn61_calc_iq__exparg_dn20_slot;
        let mut var_fn61_calc_iq__exparg_dn21: f64 = *var_fn61_calc_iq__exparg_dn21_slot;
        let mut var_fn61_calc_iq__exparg_dn22: f64 = *var_fn61_calc_iq__exparg_dn22_slot;
        let mut var_fn61_calc_iq__exparg_dn23: f64 = *var_fn61_calc_iq__exparg_dn23_slot;
        let mut var_fn61_calc_iq__exparg_dn24: f64 = *var_fn61_calc_iq__exparg_dn24_slot;
        let mut var_fn61_calc_iq__exparg_dn25: f64 = *var_fn61_calc_iq__exparg_dn25_slot;
        let mut var_fn61_calc_iq__exparg_dn26: f64 = *var_fn61_calc_iq__exparg_dn26_slot;
        let mut var_fn61_calc_iq__exparg_dn27: f64 = *var_fn61_calc_iq__exparg_dn27_slot;
        let mut var_fn61_calc_iq__exparg_dn28: f64 = *var_fn61_calc_iq__exparg_dn28_slot;
        let mut var_fn61_calc_iq__exparg_dn29: f64 = *var_fn61_calc_iq__exparg_dn29_slot;
        let mut var_fn61_calc_iq__exparg_dn3: f64 = *var_fn61_calc_iq__exparg_dn3_slot;
        let mut var_fn61_calc_iq__exparg_dn4: f64 = *var_fn61_calc_iq__exparg_dn4_slot;
        let mut var_fn61_calc_iq__exparg_dn5: f64 = *var_fn61_calc_iq__exparg_dn5_slot;
        let mut var_fn61_calc_iq__exparg_dn6: f64 = *var_fn61_calc_iq__exparg_dn6_slot;
        let mut var_fn61_calc_iq__exparg_dn7: f64 = *var_fn61_calc_iq__exparg_dn7_slot;
        let mut var_fn61_calc_iq__exparg_dn8: f64 = *var_fn61_calc_iq__exparg_dn8_slot;
        let mut var_fn61_calc_iq__exparg_dn9: f64 = *var_fn61_calc_iq__exparg_dn9_slot;
        let mut var_fn61_calc_iq__qsout: f64 = *var_fn61_calc_iq__qsout_slot;
        let mut var_fn61_calc_iq__qsout_db0: f64 = *var_fn61_calc_iq__qsout_db0_slot;
        let mut var_fn61_calc_iq__qsout_db1: f64 = *var_fn61_calc_iq__qsout_db1_slot;
        let mut var_fn61_calc_iq__qsout_db10: f64 = *var_fn61_calc_iq__qsout_db10_slot;
        let mut var_fn61_calc_iq__qsout_db11: f64 = *var_fn61_calc_iq__qsout_db11_slot;
        let mut var_fn61_calc_iq__qsout_db12: f64 = *var_fn61_calc_iq__qsout_db12_slot;
        let mut var_fn61_calc_iq__qsout_db13: f64 = *var_fn61_calc_iq__qsout_db13_slot;
        let mut var_fn61_calc_iq__qsout_db14: f64 = *var_fn61_calc_iq__qsout_db14_slot;
        let mut var_fn61_calc_iq__qsout_db15: f64 = *var_fn61_calc_iq__qsout_db15_slot;
        let mut var_fn61_calc_iq__qsout_db16: f64 = *var_fn61_calc_iq__qsout_db16_slot;
        let mut var_fn61_calc_iq__qsout_db17: f64 = *var_fn61_calc_iq__qsout_db17_slot;
        let mut var_fn61_calc_iq__qsout_db18: f64 = *var_fn61_calc_iq__qsout_db18_slot;
        let mut var_fn61_calc_iq__qsout_db19: f64 = *var_fn61_calc_iq__qsout_db19_slot;
        let mut var_fn61_calc_iq__qsout_db2: f64 = *var_fn61_calc_iq__qsout_db2_slot;
        let mut var_fn61_calc_iq__qsout_db20: f64 = *var_fn61_calc_iq__qsout_db20_slot;
        let mut var_fn61_calc_iq__qsout_db21: f64 = *var_fn61_calc_iq__qsout_db21_slot;
        let mut var_fn61_calc_iq__qsout_db22: f64 = *var_fn61_calc_iq__qsout_db22_slot;
        let mut var_fn61_calc_iq__qsout_db23: f64 = *var_fn61_calc_iq__qsout_db23_slot;
        let mut var_fn61_calc_iq__qsout_db24: f64 = *var_fn61_calc_iq__qsout_db24_slot;
        let mut var_fn61_calc_iq__qsout_db25: f64 = *var_fn61_calc_iq__qsout_db25_slot;
        let mut var_fn61_calc_iq__qsout_db26: f64 = *var_fn61_calc_iq__qsout_db26_slot;
        let mut var_fn61_calc_iq__qsout_db27: f64 = *var_fn61_calc_iq__qsout_db27_slot;
        let mut var_fn61_calc_iq__qsout_db28: f64 = *var_fn61_calc_iq__qsout_db28_slot;
        let mut var_fn61_calc_iq__qsout_db29: f64 = *var_fn61_calc_iq__qsout_db29_slot;
        let mut var_fn61_calc_iq__qsout_db3: f64 = *var_fn61_calc_iq__qsout_db3_slot;
        let mut var_fn61_calc_iq__qsout_db30: f64 = *var_fn61_calc_iq__qsout_db30_slot;
        let mut var_fn61_calc_iq__qsout_db31: f64 = *var_fn61_calc_iq__qsout_db31_slot;
        let mut var_fn61_calc_iq__qsout_db32: f64 = *var_fn61_calc_iq__qsout_db32_slot;
        let mut var_fn61_calc_iq__qsout_db33: f64 = *var_fn61_calc_iq__qsout_db33_slot;
        let mut var_fn61_calc_iq__qsout_db34: f64 = *var_fn61_calc_iq__qsout_db34_slot;
        let mut var_fn61_calc_iq__qsout_db35: f64 = *var_fn61_calc_iq__qsout_db35_slot;
        let mut var_fn61_calc_iq__qsout_db4: f64 = *var_fn61_calc_iq__qsout_db4_slot;
        let mut var_fn61_calc_iq__qsout_db5: f64 = *var_fn61_calc_iq__qsout_db5_slot;
        let mut var_fn61_calc_iq__qsout_db6: f64 = *var_fn61_calc_iq__qsout_db6_slot;
        let mut var_fn61_calc_iq__qsout_db7: f64 = *var_fn61_calc_iq__qsout_db7_slot;
        let mut var_fn61_calc_iq__qsout_db8: f64 = *var_fn61_calc_iq__qsout_db8_slot;
        let mut var_fn61_calc_iq__qsout_db9: f64 = *var_fn61_calc_iq__qsout_db9_slot;
        let mut var_fn61_calc_iq__qsout_dn0: f64 = *var_fn61_calc_iq__qsout_dn0_slot;
        let mut var_fn61_calc_iq__qsout_dn1: f64 = *var_fn61_calc_iq__qsout_dn1_slot;
        let mut var_fn61_calc_iq__qsout_dn10: f64 = *var_fn61_calc_iq__qsout_dn10_slot;
        let mut var_fn61_calc_iq__qsout_dn11: f64 = *var_fn61_calc_iq__qsout_dn11_slot;
        let mut var_fn61_calc_iq__qsout_dn12: f64 = *var_fn61_calc_iq__qsout_dn12_slot;
        let mut var_fn61_calc_iq__qsout_dn13: f64 = *var_fn61_calc_iq__qsout_dn13_slot;
        let mut var_fn61_calc_iq__qsout_dn14: f64 = *var_fn61_calc_iq__qsout_dn14_slot;
        let mut var_fn61_calc_iq__qsout_dn15: f64 = *var_fn61_calc_iq__qsout_dn15_slot;
        let mut var_fn61_calc_iq__qsout_dn16: f64 = *var_fn61_calc_iq__qsout_dn16_slot;
        let mut var_fn61_calc_iq__qsout_dn17: f64 = *var_fn61_calc_iq__qsout_dn17_slot;
        let mut var_fn61_calc_iq__qsout_dn18: f64 = *var_fn61_calc_iq__qsout_dn18_slot;
        let mut var_fn61_calc_iq__qsout_dn19: f64 = *var_fn61_calc_iq__qsout_dn19_slot;
        let mut var_fn61_calc_iq__qsout_dn2: f64 = *var_fn61_calc_iq__qsout_dn2_slot;
        let mut var_fn61_calc_iq__qsout_dn20: f64 = *var_fn61_calc_iq__qsout_dn20_slot;
        let mut var_fn61_calc_iq__qsout_dn21: f64 = *var_fn61_calc_iq__qsout_dn21_slot;
        let mut var_fn61_calc_iq__qsout_dn22: f64 = *var_fn61_calc_iq__qsout_dn22_slot;
        let mut var_fn61_calc_iq__qsout_dn23: f64 = *var_fn61_calc_iq__qsout_dn23_slot;
        let mut var_fn61_calc_iq__qsout_dn24: f64 = *var_fn61_calc_iq__qsout_dn24_slot;
        let mut var_fn61_calc_iq__qsout_dn25: f64 = *var_fn61_calc_iq__qsout_dn25_slot;
        let mut var_fn61_calc_iq__qsout_dn26: f64 = *var_fn61_calc_iq__qsout_dn26_slot;
        let mut var_fn61_calc_iq__qsout_dn27: f64 = *var_fn61_calc_iq__qsout_dn27_slot;
        let mut var_fn61_calc_iq__qsout_dn28: f64 = *var_fn61_calc_iq__qsout_dn28_slot;
        let mut var_fn61_calc_iq__qsout_dn29: f64 = *var_fn61_calc_iq__qsout_dn29_slot;
        let mut var_fn61_calc_iq__qsout_dn3: f64 = *var_fn61_calc_iq__qsout_dn3_slot;
        let mut var_fn61_calc_iq__qsout_dn4: f64 = *var_fn61_calc_iq__qsout_dn4_slot;
        let mut var_fn61_calc_iq__qsout_dn5: f64 = *var_fn61_calc_iq__qsout_dn5_slot;
        let mut var_fn61_calc_iq__qsout_dn6: f64 = *var_fn61_calc_iq__qsout_dn6_slot;
        let mut var_fn61_calc_iq__qsout_dn7: f64 = *var_fn61_calc_iq__qsout_dn7_slot;
        let mut var_fn61_calc_iq__qsout_dn8: f64 = *var_fn61_calc_iq__qsout_dn8_slot;
        let mut var_fn61_calc_iq__qsout_dn9: f64 = *var_fn61_calc_iq__qsout_dn9_slot;
        let mut var_fn61_calc_iq__return: f64 = *var_fn61_calc_iq__return_slot;
        let mut var_fn61_calc_iq__return_db0: f64 = *var_fn61_calc_iq__return_db0_slot;
        let mut var_fn61_calc_iq__return_db1: f64 = *var_fn61_calc_iq__return_db1_slot;
        let mut var_fn61_calc_iq__return_db10: f64 = *var_fn61_calc_iq__return_db10_slot;
        let mut var_fn61_calc_iq__return_db11: f64 = *var_fn61_calc_iq__return_db11_slot;
        let mut var_fn61_calc_iq__return_db12: f64 = *var_fn61_calc_iq__return_db12_slot;
        let mut var_fn61_calc_iq__return_db13: f64 = *var_fn61_calc_iq__return_db13_slot;
        let mut var_fn61_calc_iq__return_db14: f64 = *var_fn61_calc_iq__return_db14_slot;
        let mut var_fn61_calc_iq__return_db15: f64 = *var_fn61_calc_iq__return_db15_slot;
        let mut var_fn61_calc_iq__return_db16: f64 = *var_fn61_calc_iq__return_db16_slot;
        let mut var_fn61_calc_iq__return_db17: f64 = *var_fn61_calc_iq__return_db17_slot;
        let mut var_fn61_calc_iq__return_db18: f64 = *var_fn61_calc_iq__return_db18_slot;
        let mut var_fn61_calc_iq__return_db19: f64 = *var_fn61_calc_iq__return_db19_slot;
        let mut var_fn61_calc_iq__return_db2: f64 = *var_fn61_calc_iq__return_db2_slot;
        let mut var_fn61_calc_iq__return_db20: f64 = *var_fn61_calc_iq__return_db20_slot;
        let mut var_fn61_calc_iq__return_db21: f64 = *var_fn61_calc_iq__return_db21_slot;
        let mut var_fn61_calc_iq__return_db22: f64 = *var_fn61_calc_iq__return_db22_slot;
        let mut var_fn61_calc_iq__return_db23: f64 = *var_fn61_calc_iq__return_db23_slot;
        let mut var_fn61_calc_iq__return_db24: f64 = *var_fn61_calc_iq__return_db24_slot;
        let mut var_fn61_calc_iq__return_db25: f64 = *var_fn61_calc_iq__return_db25_slot;
        let mut var_fn61_calc_iq__return_db26: f64 = *var_fn61_calc_iq__return_db26_slot;
        let mut var_fn61_calc_iq__return_db27: f64 = *var_fn61_calc_iq__return_db27_slot;
        let mut var_fn61_calc_iq__return_db28: f64 = *var_fn61_calc_iq__return_db28_slot;
        let mut var_fn61_calc_iq__return_db29: f64 = *var_fn61_calc_iq__return_db29_slot;
        let mut var_fn61_calc_iq__return_db3: f64 = *var_fn61_calc_iq__return_db3_slot;
        let mut var_fn61_calc_iq__return_db30: f64 = *var_fn61_calc_iq__return_db30_slot;
        let mut var_fn61_calc_iq__return_db31: f64 = *var_fn61_calc_iq__return_db31_slot;
        let mut var_fn61_calc_iq__return_db32: f64 = *var_fn61_calc_iq__return_db32_slot;
        let mut var_fn61_calc_iq__return_db33: f64 = *var_fn61_calc_iq__return_db33_slot;
        let mut var_fn61_calc_iq__return_db34: f64 = *var_fn61_calc_iq__return_db34_slot;
        let mut var_fn61_calc_iq__return_db35: f64 = *var_fn61_calc_iq__return_db35_slot;
        let mut var_fn61_calc_iq__return_db4: f64 = *var_fn61_calc_iq__return_db4_slot;
        let mut var_fn61_calc_iq__return_db5: f64 = *var_fn61_calc_iq__return_db5_slot;
        let mut var_fn61_calc_iq__return_db6: f64 = *var_fn61_calc_iq__return_db6_slot;
        let mut var_fn61_calc_iq__return_db7: f64 = *var_fn61_calc_iq__return_db7_slot;
        let mut var_fn61_calc_iq__return_db8: f64 = *var_fn61_calc_iq__return_db8_slot;
        let mut var_fn61_calc_iq__return_db9: f64 = *var_fn61_calc_iq__return_db9_slot;
        let mut var_fn61_calc_iq__return_dn0: f64 = *var_fn61_calc_iq__return_dn0_slot;
        let mut var_fn61_calc_iq__return_dn1: f64 = *var_fn61_calc_iq__return_dn1_slot;
        let mut var_fn61_calc_iq__return_dn10: f64 = *var_fn61_calc_iq__return_dn10_slot;
        let mut var_fn61_calc_iq__return_dn11: f64 = *var_fn61_calc_iq__return_dn11_slot;
        let mut var_fn61_calc_iq__return_dn12: f64 = *var_fn61_calc_iq__return_dn12_slot;
        let mut var_fn61_calc_iq__return_dn13: f64 = *var_fn61_calc_iq__return_dn13_slot;
        let mut var_fn61_calc_iq__return_dn14: f64 = *var_fn61_calc_iq__return_dn14_slot;
        let mut var_fn61_calc_iq__return_dn15: f64 = *var_fn61_calc_iq__return_dn15_slot;
        let mut var_fn61_calc_iq__return_dn16: f64 = *var_fn61_calc_iq__return_dn16_slot;
        let mut var_fn61_calc_iq__return_dn17: f64 = *var_fn61_calc_iq__return_dn17_slot;
        let mut var_fn61_calc_iq__return_dn18: f64 = *var_fn61_calc_iq__return_dn18_slot;
        let mut var_fn61_calc_iq__return_dn19: f64 = *var_fn61_calc_iq__return_dn19_slot;
        let mut var_fn61_calc_iq__return_dn2: f64 = *var_fn61_calc_iq__return_dn2_slot;
        let mut var_fn61_calc_iq__return_dn20: f64 = *var_fn61_calc_iq__return_dn20_slot;
        let mut var_fn61_calc_iq__return_dn21: f64 = *var_fn61_calc_iq__return_dn21_slot;
        let mut var_fn61_calc_iq__return_dn22: f64 = *var_fn61_calc_iq__return_dn22_slot;
        let mut var_fn61_calc_iq__return_dn23: f64 = *var_fn61_calc_iq__return_dn23_slot;
        let mut var_fn61_calc_iq__return_dn24: f64 = *var_fn61_calc_iq__return_dn24_slot;
        let mut var_fn61_calc_iq__return_dn25: f64 = *var_fn61_calc_iq__return_dn25_slot;
        let mut var_fn61_calc_iq__return_dn26: f64 = *var_fn61_calc_iq__return_dn26_slot;
        let mut var_fn61_calc_iq__return_dn27: f64 = *var_fn61_calc_iq__return_dn27_slot;
        let mut var_fn61_calc_iq__return_dn28: f64 = *var_fn61_calc_iq__return_dn28_slot;
        let mut var_fn61_calc_iq__return_dn29: f64 = *var_fn61_calc_iq__return_dn29_slot;
        let mut var_fn61_calc_iq__return_dn3: f64 = *var_fn61_calc_iq__return_dn3_slot;
        let mut var_fn61_calc_iq__return_dn4: f64 = *var_fn61_calc_iq__return_dn4_slot;
        let mut var_fn61_calc_iq__return_dn5: f64 = *var_fn61_calc_iq__return_dn5_slot;
        let mut var_fn61_calc_iq__return_dn6: f64 = *var_fn61_calc_iq__return_dn6_slot;
        let mut var_fn61_calc_iq__return_dn7: f64 = *var_fn61_calc_iq__return_dn7_slot;
        let mut var_fn61_calc_iq__return_dn8: f64 = *var_fn61_calc_iq__return_dn8_slot;
        let mut var_fn61_calc_iq__return_dn9: f64 = *var_fn61_calc_iq__return_dn9_slot;
        let mut var_guard94: f64 = *var_guard94_slot;

        let (assign7180_e8622, assign7180_e8622_d_n0, assign7180_e8622_d_n1, assign7180_e8622_d_n2, assign7180_e8622_d_n3, assign7180_e8622_d_n4, assign7180_e8622_d_n5, assign7180_e8622_d_n6, assign7180_e8622_d_n7, assign7180_e8622_d_n8, assign7180_e8622_d_n9, assign7180_e8622_d_n10, assign7180_e8622_d_n11, assign7180_e8622_d_n12, assign7180_e8622_d_n13, assign7180_e8622_d_n14, assign7180_e8622_d_n15, assign7180_e8622_d_n16, assign7180_e8622_d_n17, assign7180_e8622_d_n18, assign7180_e8622_d_n19, assign7180_e8622_d_n20, assign7180_e8622_d_n21, assign7180_e8622_d_n22, assign7180_e8622_d_n23, assign7180_e8622_d_n24, assign7180_e8622_d_n25, assign7180_e8622_d_n26, assign7180_e8622_d_n27, assign7180_e8622_d_n28, assign7180_e8622_d_n29, assign7180_e8622_d_b0, assign7180_e8622_d_b1, assign7180_e8622_d_b2, assign7180_e8622_d_b3, assign7180_e8622_d_b4, assign7180_e8622_d_b5, assign7180_e8622_d_b6, assign7180_e8622_d_b7, assign7180_e8622_d_b8, assign7180_e8622_d_b9, assign7180_e8622_d_b10, assign7180_e8622_d_b11, assign7180_e8622_d_b12, assign7180_e8622_d_b13, assign7180_e8622_d_b14, assign7180_e8622_d_b15, assign7180_e8622_d_b16, assign7180_e8622_d_b17, assign7180_e8622_d_b18, assign7180_e8622_d_b19, assign7180_e8622_d_b20, assign7180_e8622_d_b21, assign7180_e8622_d_b22, assign7180_e8622_d_b23, assign7180_e8622_d_b24, assign7180_e8622_d_b25, assign7180_e8622_d_b26, assign7180_e8622_d_b27, assign7180_e8622_d_b28, assign7180_e8622_d_b29, assign7180_e8622_d_b30, assign7180_e8622_d_b31, assign7180_e8622_d_b32, assign7180_e8622_d_b33, assign7180_e8622_d_b34, assign7180_e8622_d_b35,) = {
    if (((var_guard60 != 0.0) && (var_guard92 != 0.0)) && (var_guard93 != 0.0)) {
        (var_fn61_calc_iq__etags, var_fn61_calc_iq__etags_dn0, var_fn61_calc_iq__etags_dn1, var_fn61_calc_iq__etags_dn2, var_fn61_calc_iq__etags_dn3, var_fn61_calc_iq__etags_dn4, var_fn61_calc_iq__etags_dn5, var_fn61_calc_iq__etags_dn6, var_fn61_calc_iq__etags_dn7, var_fn61_calc_iq__etags_dn8, var_fn61_calc_iq__etags_dn9, var_fn61_calc_iq__etags_dn10, var_fn61_calc_iq__etags_dn11, var_fn61_calc_iq__etags_dn12, var_fn61_calc_iq__etags_dn13, var_fn61_calc_iq__etags_dn14, var_fn61_calc_iq__etags_dn15, var_fn61_calc_iq__etags_dn16, var_fn61_calc_iq__etags_dn17, var_fn61_calc_iq__etags_dn18, var_fn61_calc_iq__etags_dn19, var_fn61_calc_iq__etags_dn20, var_fn61_calc_iq__etags_dn21, var_fn61_calc_iq__etags_dn22, var_fn61_calc_iq__etags_dn23, var_fn61_calc_iq__etags_dn24, var_fn61_calc_iq__etags_dn25, var_fn61_calc_iq__etags_dn26, var_fn61_calc_iq__etags_dn27, var_fn61_calc_iq__etags_dn28, var_fn61_calc_iq__etags_dn29, var_fn61_calc_iq__etags_db0, var_fn61_calc_iq__etags_db1, var_fn61_calc_iq__etags_db2, var_fn61_calc_iq__etags_db3, var_fn61_calc_iq__etags_db4, var_fn61_calc_iq__etags_db5, var_fn61_calc_iq__etags_db6, var_fn61_calc_iq__etags_db7, var_fn61_calc_iq__etags_db8, var_fn61_calc_iq__etags_db9, var_fn61_calc_iq__etags_db10, var_fn61_calc_iq__etags_db11, var_fn61_calc_iq__etags_db12, var_fn61_calc_iq__etags_db13, var_fn61_calc_iq__etags_db14, var_fn61_calc_iq__etags_db15, var_fn61_calc_iq__etags_db16, var_fn61_calc_iq__etags_db17, var_fn61_calc_iq__etags_db18, var_fn61_calc_iq__etags_db19, var_fn61_calc_iq__etags_db20, var_fn61_calc_iq__etags_db21, var_fn61_calc_iq__etags_db22, var_fn61_calc_iq__etags_db23, var_fn61_calc_iq__etags_db24, var_fn61_calc_iq__etags_db25, var_fn61_calc_iq__etags_db26, var_fn61_calc_iq__etags_db27, var_fn61_calc_iq__etags_db28, var_fn61_calc_iq__etags_db29, var_fn61_calc_iq__etags_db30, var_fn61_calc_iq__etags_db31, var_fn61_calc_iq__etags_db32, var_fn61_calc_iq__etags_db33, var_fn61_calc_iq__etags_db34, var_fn61_calc_iq__etags_db35,)
    } else {
        (var_fn61_calc_iq__exparg, var_fn61_calc_iq__exparg_dn0, var_fn61_calc_iq__exparg_dn1, var_fn61_calc_iq__exparg_dn2, var_fn61_calc_iq__exparg_dn3, var_fn61_calc_iq__exparg_dn4, var_fn61_calc_iq__exparg_dn5, var_fn61_calc_iq__exparg_dn6, var_fn61_calc_iq__exparg_dn7, var_fn61_calc_iq__exparg_dn8, var_fn61_calc_iq__exparg_dn9, var_fn61_calc_iq__exparg_dn10, var_fn61_calc_iq__exparg_dn11, var_fn61_calc_iq__exparg_dn12, var_fn61_calc_iq__exparg_dn13, var_fn61_calc_iq__exparg_dn14, var_fn61_calc_iq__exparg_dn15, var_fn61_calc_iq__exparg_dn16, var_fn61_calc_iq__exparg_dn17, var_fn61_calc_iq__exparg_dn18, var_fn61_calc_iq__exparg_dn19, var_fn61_calc_iq__exparg_dn20, var_fn61_calc_iq__exparg_dn21, var_fn61_calc_iq__exparg_dn22, var_fn61_calc_iq__exparg_dn23, var_fn61_calc_iq__exparg_dn24, var_fn61_calc_iq__exparg_dn25, var_fn61_calc_iq__exparg_dn26, var_fn61_calc_iq__exparg_dn27, var_fn61_calc_iq__exparg_dn28, var_fn61_calc_iq__exparg_dn29, var_fn61_calc_iq__exparg_db0, var_fn61_calc_iq__exparg_db1, var_fn61_calc_iq__exparg_db2, var_fn61_calc_iq__exparg_db3, var_fn61_calc_iq__exparg_db4, var_fn61_calc_iq__exparg_db5, var_fn61_calc_iq__exparg_db6, var_fn61_calc_iq__exparg_db7, var_fn61_calc_iq__exparg_db8, var_fn61_calc_iq__exparg_db9, var_fn61_calc_iq__exparg_db10, var_fn61_calc_iq__exparg_db11, var_fn61_calc_iq__exparg_db12, var_fn61_calc_iq__exparg_db13, var_fn61_calc_iq__exparg_db14, var_fn61_calc_iq__exparg_db15, var_fn61_calc_iq__exparg_db16, var_fn61_calc_iq__exparg_db17, var_fn61_calc_iq__exparg_db18, var_fn61_calc_iq__exparg_db19, var_fn61_calc_iq__exparg_db20, var_fn61_calc_iq__exparg_db21, var_fn61_calc_iq__exparg_db22, var_fn61_calc_iq__exparg_db23, var_fn61_calc_iq__exparg_db24, var_fn61_calc_iq__exparg_db25, var_fn61_calc_iq__exparg_db26, var_fn61_calc_iq__exparg_db27, var_fn61_calc_iq__exparg_db28, var_fn61_calc_iq__exparg_db29, var_fn61_calc_iq__exparg_db30, var_fn61_calc_iq__exparg_db31, var_fn61_calc_iq__exparg_db32, var_fn61_calc_iq__exparg_db33, var_fn61_calc_iq__exparg_db34, var_fn61_calc_iq__exparg_db35,)
    }
};
        var_fn61_calc_iq__exparg = assign7180_e8622;
        var_fn61_calc_iq__exparg_dn0 = assign7180_e8622_d_n0;
        var_fn61_calc_iq__exparg_dn1 = assign7180_e8622_d_n1;
        var_fn61_calc_iq__exparg_dn2 = assign7180_e8622_d_n2;
        var_fn61_calc_iq__exparg_dn3 = assign7180_e8622_d_n3;
        var_fn61_calc_iq__exparg_dn4 = assign7180_e8622_d_n4;
        var_fn61_calc_iq__exparg_dn5 = assign7180_e8622_d_n5;
        var_fn61_calc_iq__exparg_dn6 = assign7180_e8622_d_n6;
        var_fn61_calc_iq__exparg_dn7 = assign7180_e8622_d_n7;
        var_fn61_calc_iq__exparg_dn8 = assign7180_e8622_d_n8;
        var_fn61_calc_iq__exparg_dn9 = assign7180_e8622_d_n9;
        var_fn61_calc_iq__exparg_dn10 = assign7180_e8622_d_n10;
        var_fn61_calc_iq__exparg_dn11 = assign7180_e8622_d_n11;
        var_fn61_calc_iq__exparg_dn12 = assign7180_e8622_d_n12;
        var_fn61_calc_iq__exparg_dn13 = assign7180_e8622_d_n13;
        var_fn61_calc_iq__exparg_dn14 = assign7180_e8622_d_n14;
        var_fn61_calc_iq__exparg_dn15 = assign7180_e8622_d_n15;
        var_fn61_calc_iq__exparg_dn16 = assign7180_e8622_d_n16;
        var_fn61_calc_iq__exparg_dn17 = assign7180_e8622_d_n17;
        var_fn61_calc_iq__exparg_dn18 = assign7180_e8622_d_n18;
        var_fn61_calc_iq__exparg_dn19 = assign7180_e8622_d_n19;
        var_fn61_calc_iq__exparg_dn20 = assign7180_e8622_d_n20;
        var_fn61_calc_iq__exparg_dn21 = assign7180_e8622_d_n21;
        var_fn61_calc_iq__exparg_dn22 = assign7180_e8622_d_n22;
        var_fn61_calc_iq__exparg_dn23 = assign7180_e8622_d_n23;
        var_fn61_calc_iq__exparg_dn24 = assign7180_e8622_d_n24;
        var_fn61_calc_iq__exparg_dn25 = assign7180_e8622_d_n25;
        var_fn61_calc_iq__exparg_dn26 = assign7180_e8622_d_n26;
        var_fn61_calc_iq__exparg_dn27 = assign7180_e8622_d_n27;
        var_fn61_calc_iq__exparg_dn28 = assign7180_e8622_d_n28;
        var_fn61_calc_iq__exparg_dn29 = assign7180_e8622_d_n29;
        var_fn61_calc_iq__exparg_db0 = assign7180_e8622_d_b0;
        var_fn61_calc_iq__exparg_db1 = assign7180_e8622_d_b1;
        var_fn61_calc_iq__exparg_db2 = assign7180_e8622_d_b2;
        var_fn61_calc_iq__exparg_db3 = assign7180_e8622_d_b3;
        var_fn61_calc_iq__exparg_db4 = assign7180_e8622_d_b4;
        var_fn61_calc_iq__exparg_db5 = assign7180_e8622_d_b5;
        var_fn61_calc_iq__exparg_db6 = assign7180_e8622_d_b6;
        var_fn61_calc_iq__exparg_db7 = assign7180_e8622_d_b7;
        var_fn61_calc_iq__exparg_db8 = assign7180_e8622_d_b8;
        var_fn61_calc_iq__exparg_db9 = assign7180_e8622_d_b9;
        var_fn61_calc_iq__exparg_db10 = assign7180_e8622_d_b10;
        var_fn61_calc_iq__exparg_db11 = assign7180_e8622_d_b11;
        var_fn61_calc_iq__exparg_db12 = assign7180_e8622_d_b12;
        var_fn61_calc_iq__exparg_db13 = assign7180_e8622_d_b13;
        var_fn61_calc_iq__exparg_db14 = assign7180_e8622_d_b14;
        var_fn61_calc_iq__exparg_db15 = assign7180_e8622_d_b15;
        var_fn61_calc_iq__exparg_db16 = assign7180_e8622_d_b16;
        var_fn61_calc_iq__exparg_db17 = assign7180_e8622_d_b17;
        var_fn61_calc_iq__exparg_db18 = assign7180_e8622_d_b18;
        var_fn61_calc_iq__exparg_db19 = assign7180_e8622_d_b19;
        var_fn61_calc_iq__exparg_db20 = assign7180_e8622_d_b20;
        var_fn61_calc_iq__exparg_db21 = assign7180_e8622_d_b21;
        var_fn61_calc_iq__exparg_db22 = assign7180_e8622_d_b22;
        var_fn61_calc_iq__exparg_db23 = assign7180_e8622_d_b23;
        var_fn61_calc_iq__exparg_db24 = assign7180_e8622_d_b24;
        var_fn61_calc_iq__exparg_db25 = assign7180_e8622_d_b25;
        var_fn61_calc_iq__exparg_db26 = assign7180_e8622_d_b26;
        var_fn61_calc_iq__exparg_db27 = assign7180_e8622_d_b27;
        var_fn61_calc_iq__exparg_db28 = assign7180_e8622_d_b28;
        var_fn61_calc_iq__exparg_db29 = assign7180_e8622_d_b29;
        var_fn61_calc_iq__exparg_db30 = assign7180_e8622_d_b30;
        var_fn61_calc_iq__exparg_db31 = assign7180_e8622_d_b31;
        var_fn61_calc_iq__exparg_db32 = assign7180_e8622_d_b32;
        var_fn61_calc_iq__exparg_db33 = assign7180_e8622_d_b33;
        var_fn61_calc_iq__exparg_db34 = assign7180_e8622_d_b34;
        var_fn61_calc_iq__exparg_db35 = assign7180_e8622_d_b35;

        let assign7190_e8625: f64 = (-50.0);
        let assign7190_e8626: f64 = if var_fn61_calc_iq__etags < assign7190_e8625 { 1.0 } else { 0.0 };
        var_guard94 = assign7190_e8626;

        let (assign7200_e8638, assign7200_e8638_d_n0, assign7200_e8638_d_n1, assign7200_e8638_d_n2, assign7200_e8638_d_n3, assign7200_e8638_d_n4, assign7200_e8638_d_n5, assign7200_e8638_d_n6, assign7200_e8638_d_n7, assign7200_e8638_d_n8, assign7200_e8638_d_n9, assign7200_e8638_d_n10, assign7200_e8638_d_n11, assign7200_e8638_d_n12, assign7200_e8638_d_n13, assign7200_e8638_d_n14, assign7200_e8638_d_n15, assign7200_e8638_d_n16, assign7200_e8638_d_n17, assign7200_e8638_d_n18, assign7200_e8638_d_n19, assign7200_e8638_d_n20, assign7200_e8638_d_n21, assign7200_e8638_d_n22, assign7200_e8638_d_n23, assign7200_e8638_d_n24, assign7200_e8638_d_n25, assign7200_e8638_d_n26, assign7200_e8638_d_n27, assign7200_e8638_d_n28, assign7200_e8638_d_n29, assign7200_e8638_d_b0, assign7200_e8638_d_b1, assign7200_e8638_d_b2, assign7200_e8638_d_b3, assign7200_e8638_d_b4, assign7200_e8638_d_b5, assign7200_e8638_d_b6, assign7200_e8638_d_b7, assign7200_e8638_d_b8, assign7200_e8638_d_b9, assign7200_e8638_d_b10, assign7200_e8638_d_b11, assign7200_e8638_d_b12, assign7200_e8638_d_b13, assign7200_e8638_d_b14, assign7200_e8638_d_b15, assign7200_e8638_d_b16, assign7200_e8638_d_b17, assign7200_e8638_d_b18, assign7200_e8638_d_b19, assign7200_e8638_d_b20, assign7200_e8638_d_b21, assign7200_e8638_d_b22, assign7200_e8638_d_b23, assign7200_e8638_d_b24, assign7200_e8638_d_b25, assign7200_e8638_d_b26, assign7200_e8638_d_b27, assign7200_e8638_d_b28, assign7200_e8638_d_b29, assign7200_e8638_d_b30, assign7200_e8638_d_b31, assign7200_e8638_d_b32, assign7200_e8638_d_b33, assign7200_e8638_d_b34, assign7200_e8638_d_b35,) = {
    if ((((var_guard60 != 0.0) && (var_guard92 != 0.0)) && (var_guard93 == 0.0)) && (var_guard94 != 0.0)) {
        let assign7200_e8636: f64 = (var_fn61_calc_iq__etags).exp();
        (assign7200_e8636, (assign7200_e8636 * var_fn61_calc_iq__etags_dn0), (assign7200_e8636 * var_fn61_calc_iq__etags_dn1), (assign7200_e8636 * var_fn61_calc_iq__etags_dn2), (assign7200_e8636 * var_fn61_calc_iq__etags_dn3), (assign7200_e8636 * var_fn61_calc_iq__etags_dn4), (assign7200_e8636 * var_fn61_calc_iq__etags_dn5), (assign7200_e8636 * var_fn61_calc_iq__etags_dn6), (assign7200_e8636 * var_fn61_calc_iq__etags_dn7), (assign7200_e8636 * var_fn61_calc_iq__etags_dn8), (assign7200_e8636 * var_fn61_calc_iq__etags_dn9), (assign7200_e8636 * var_fn61_calc_iq__etags_dn10), (assign7200_e8636 * var_fn61_calc_iq__etags_dn11), (assign7200_e8636 * var_fn61_calc_iq__etags_dn12), (assign7200_e8636 * var_fn61_calc_iq__etags_dn13), (assign7200_e8636 * var_fn61_calc_iq__etags_dn14), (assign7200_e8636 * var_fn61_calc_iq__etags_dn15), (assign7200_e8636 * var_fn61_calc_iq__etags_dn16), (assign7200_e8636 * var_fn61_calc_iq__etags_dn17), (assign7200_e8636 * var_fn61_calc_iq__etags_dn18), (assign7200_e8636 * var_fn61_calc_iq__etags_dn19), (assign7200_e8636 * var_fn61_calc_iq__etags_dn20), (assign7200_e8636 * var_fn61_calc_iq__etags_dn21), (assign7200_e8636 * var_fn61_calc_iq__etags_dn22), (assign7200_e8636 * var_fn61_calc_iq__etags_dn23), (assign7200_e8636 * var_fn61_calc_iq__etags_dn24), (assign7200_e8636 * var_fn61_calc_iq__etags_dn25), (assign7200_e8636 * var_fn61_calc_iq__etags_dn26), (assign7200_e8636 * var_fn61_calc_iq__etags_dn27), (assign7200_e8636 * var_fn61_calc_iq__etags_dn28), (assign7200_e8636 * var_fn61_calc_iq__etags_dn29), (assign7200_e8636 * var_fn61_calc_iq__etags_db0), (assign7200_e8636 * var_fn61_calc_iq__etags_db1), (assign7200_e8636 * var_fn61_calc_iq__etags_db2), (assign7200_e8636 * var_fn61_calc_iq__etags_db3), (assign7200_e8636 * var_fn61_calc_iq__etags_db4), (assign7200_e8636 * var_fn61_calc_iq__etags_db5), (assign7200_e8636 * var_fn61_calc_iq__etags_db6), (assign7200_e8636 * var_fn61_calc_iq__etags_db7), (assign7200_e8636 * var_fn61_calc_iq__etags_db8), (assign7200_e8636 * var_fn61_calc_iq__etags_db9), (assign7200_e8636 * var_fn61_calc_iq__etags_db10), (assign7200_e8636 * var_fn61_calc_iq__etags_db11), (assign7200_e8636 * var_fn61_calc_iq__etags_db12), (assign7200_e8636 * var_fn61_calc_iq__etags_db13), (assign7200_e8636 * var_fn61_calc_iq__etags_db14), (assign7200_e8636 * var_fn61_calc_iq__etags_db15), (assign7200_e8636 * var_fn61_calc_iq__etags_db16), (assign7200_e8636 * var_fn61_calc_iq__etags_db17), (assign7200_e8636 * var_fn61_calc_iq__etags_db18), (assign7200_e8636 * var_fn61_calc_iq__etags_db19), (assign7200_e8636 * var_fn61_calc_iq__etags_db20), (assign7200_e8636 * var_fn61_calc_iq__etags_db21), (assign7200_e8636 * var_fn61_calc_iq__etags_db22), (assign7200_e8636 * var_fn61_calc_iq__etags_db23), (assign7200_e8636 * var_fn61_calc_iq__etags_db24), (assign7200_e8636 * var_fn61_calc_iq__etags_db25), (assign7200_e8636 * var_fn61_calc_iq__etags_db26), (assign7200_e8636 * var_fn61_calc_iq__etags_db27), (assign7200_e8636 * var_fn61_calc_iq__etags_db28), (assign7200_e8636 * var_fn61_calc_iq__etags_db29), (assign7200_e8636 * var_fn61_calc_iq__etags_db30), (assign7200_e8636 * var_fn61_calc_iq__etags_db31), (assign7200_e8636 * var_fn61_calc_iq__etags_db32), (assign7200_e8636 * var_fn61_calc_iq__etags_db33), (assign7200_e8636 * var_fn61_calc_iq__etags_db34), (assign7200_e8636 * var_fn61_calc_iq__etags_db35),)
    } else {
        (var_fn61_calc_iq__exparg, var_fn61_calc_iq__exparg_dn0, var_fn61_calc_iq__exparg_dn1, var_fn61_calc_iq__exparg_dn2, var_fn61_calc_iq__exparg_dn3, var_fn61_calc_iq__exparg_dn4, var_fn61_calc_iq__exparg_dn5, var_fn61_calc_iq__exparg_dn6, var_fn61_calc_iq__exparg_dn7, var_fn61_calc_iq__exparg_dn8, var_fn61_calc_iq__exparg_dn9, var_fn61_calc_iq__exparg_dn10, var_fn61_calc_iq__exparg_dn11, var_fn61_calc_iq__exparg_dn12, var_fn61_calc_iq__exparg_dn13, var_fn61_calc_iq__exparg_dn14, var_fn61_calc_iq__exparg_dn15, var_fn61_calc_iq__exparg_dn16, var_fn61_calc_iq__exparg_dn17, var_fn61_calc_iq__exparg_dn18, var_fn61_calc_iq__exparg_dn19, var_fn61_calc_iq__exparg_dn20, var_fn61_calc_iq__exparg_dn21, var_fn61_calc_iq__exparg_dn22, var_fn61_calc_iq__exparg_dn23, var_fn61_calc_iq__exparg_dn24, var_fn61_calc_iq__exparg_dn25, var_fn61_calc_iq__exparg_dn26, var_fn61_calc_iq__exparg_dn27, var_fn61_calc_iq__exparg_dn28, var_fn61_calc_iq__exparg_dn29, var_fn61_calc_iq__exparg_db0, var_fn61_calc_iq__exparg_db1, var_fn61_calc_iq__exparg_db2, var_fn61_calc_iq__exparg_db3, var_fn61_calc_iq__exparg_db4, var_fn61_calc_iq__exparg_db5, var_fn61_calc_iq__exparg_db6, var_fn61_calc_iq__exparg_db7, var_fn61_calc_iq__exparg_db8, var_fn61_calc_iq__exparg_db9, var_fn61_calc_iq__exparg_db10, var_fn61_calc_iq__exparg_db11, var_fn61_calc_iq__exparg_db12, var_fn61_calc_iq__exparg_db13, var_fn61_calc_iq__exparg_db14, var_fn61_calc_iq__exparg_db15, var_fn61_calc_iq__exparg_db16, var_fn61_calc_iq__exparg_db17, var_fn61_calc_iq__exparg_db18, var_fn61_calc_iq__exparg_db19, var_fn61_calc_iq__exparg_db20, var_fn61_calc_iq__exparg_db21, var_fn61_calc_iq__exparg_db22, var_fn61_calc_iq__exparg_db23, var_fn61_calc_iq__exparg_db24, var_fn61_calc_iq__exparg_db25, var_fn61_calc_iq__exparg_db26, var_fn61_calc_iq__exparg_db27, var_fn61_calc_iq__exparg_db28, var_fn61_calc_iq__exparg_db29, var_fn61_calc_iq__exparg_db30, var_fn61_calc_iq__exparg_db31, var_fn61_calc_iq__exparg_db32, var_fn61_calc_iq__exparg_db33, var_fn61_calc_iq__exparg_db34, var_fn61_calc_iq__exparg_db35,)
    }
};
        var_fn61_calc_iq__exparg = assign7200_e8638;
        var_fn61_calc_iq__exparg_dn0 = assign7200_e8638_d_n0;
        var_fn61_calc_iq__exparg_dn1 = assign7200_e8638_d_n1;
        var_fn61_calc_iq__exparg_dn2 = assign7200_e8638_d_n2;
        var_fn61_calc_iq__exparg_dn3 = assign7200_e8638_d_n3;
        var_fn61_calc_iq__exparg_dn4 = assign7200_e8638_d_n4;
        var_fn61_calc_iq__exparg_dn5 = assign7200_e8638_d_n5;
        var_fn61_calc_iq__exparg_dn6 = assign7200_e8638_d_n6;
        var_fn61_calc_iq__exparg_dn7 = assign7200_e8638_d_n7;
        var_fn61_calc_iq__exparg_dn8 = assign7200_e8638_d_n8;
        var_fn61_calc_iq__exparg_dn9 = assign7200_e8638_d_n9;
        var_fn61_calc_iq__exparg_dn10 = assign7200_e8638_d_n10;
        var_fn61_calc_iq__exparg_dn11 = assign7200_e8638_d_n11;
        var_fn61_calc_iq__exparg_dn12 = assign7200_e8638_d_n12;
        var_fn61_calc_iq__exparg_dn13 = assign7200_e8638_d_n13;
        var_fn61_calc_iq__exparg_dn14 = assign7200_e8638_d_n14;
        var_fn61_calc_iq__exparg_dn15 = assign7200_e8638_d_n15;
        var_fn61_calc_iq__exparg_dn16 = assign7200_e8638_d_n16;
        var_fn61_calc_iq__exparg_dn17 = assign7200_e8638_d_n17;
        var_fn61_calc_iq__exparg_dn18 = assign7200_e8638_d_n18;
        var_fn61_calc_iq__exparg_dn19 = assign7200_e8638_d_n19;
        var_fn61_calc_iq__exparg_dn20 = assign7200_e8638_d_n20;
        var_fn61_calc_iq__exparg_dn21 = assign7200_e8638_d_n21;
        var_fn61_calc_iq__exparg_dn22 = assign7200_e8638_d_n22;
        var_fn61_calc_iq__exparg_dn23 = assign7200_e8638_d_n23;
        var_fn61_calc_iq__exparg_dn24 = assign7200_e8638_d_n24;
        var_fn61_calc_iq__exparg_dn25 = assign7200_e8638_d_n25;
        var_fn61_calc_iq__exparg_dn26 = assign7200_e8638_d_n26;
        var_fn61_calc_iq__exparg_dn27 = assign7200_e8638_d_n27;
        var_fn61_calc_iq__exparg_dn28 = assign7200_e8638_d_n28;
        var_fn61_calc_iq__exparg_dn29 = assign7200_e8638_d_n29;
        var_fn61_calc_iq__exparg_db0 = assign7200_e8638_d_b0;
        var_fn61_calc_iq__exparg_db1 = assign7200_e8638_d_b1;
        var_fn61_calc_iq__exparg_db2 = assign7200_e8638_d_b2;
        var_fn61_calc_iq__exparg_db3 = assign7200_e8638_d_b3;
        var_fn61_calc_iq__exparg_db4 = assign7200_e8638_d_b4;
        var_fn61_calc_iq__exparg_db5 = assign7200_e8638_d_b5;
        var_fn61_calc_iq__exparg_db6 = assign7200_e8638_d_b6;
        var_fn61_calc_iq__exparg_db7 = assign7200_e8638_d_b7;
        var_fn61_calc_iq__exparg_db8 = assign7200_e8638_d_b8;
        var_fn61_calc_iq__exparg_db9 = assign7200_e8638_d_b9;
        var_fn61_calc_iq__exparg_db10 = assign7200_e8638_d_b10;
        var_fn61_calc_iq__exparg_db11 = assign7200_e8638_d_b11;
        var_fn61_calc_iq__exparg_db12 = assign7200_e8638_d_b12;
        var_fn61_calc_iq__exparg_db13 = assign7200_e8638_d_b13;
        var_fn61_calc_iq__exparg_db14 = assign7200_e8638_d_b14;
        var_fn61_calc_iq__exparg_db15 = assign7200_e8638_d_b15;
        var_fn61_calc_iq__exparg_db16 = assign7200_e8638_d_b16;
        var_fn61_calc_iq__exparg_db17 = assign7200_e8638_d_b17;
        var_fn61_calc_iq__exparg_db18 = assign7200_e8638_d_b18;
        var_fn61_calc_iq__exparg_db19 = assign7200_e8638_d_b19;
        var_fn61_calc_iq__exparg_db20 = assign7200_e8638_d_b20;
        var_fn61_calc_iq__exparg_db21 = assign7200_e8638_d_b21;
        var_fn61_calc_iq__exparg_db22 = assign7200_e8638_d_b22;
        var_fn61_calc_iq__exparg_db23 = assign7200_e8638_d_b23;
        var_fn61_calc_iq__exparg_db24 = assign7200_e8638_d_b24;
        var_fn61_calc_iq__exparg_db25 = assign7200_e8638_d_b25;
        var_fn61_calc_iq__exparg_db26 = assign7200_e8638_d_b26;
        var_fn61_calc_iq__exparg_db27 = assign7200_e8638_d_b27;
        var_fn61_calc_iq__exparg_db28 = assign7200_e8638_d_b28;
        var_fn61_calc_iq__exparg_db29 = assign7200_e8638_d_b29;
        var_fn61_calc_iq__exparg_db30 = assign7200_e8638_d_b30;
        var_fn61_calc_iq__exparg_db31 = assign7200_e8638_d_b31;
        var_fn61_calc_iq__exparg_db32 = assign7200_e8638_d_b32;
        var_fn61_calc_iq__exparg_db33 = assign7200_e8638_d_b33;
        var_fn61_calc_iq__exparg_db34 = assign7200_e8638_d_b34;
        var_fn61_calc_iq__exparg_db35 = assign7200_e8638_d_b35;

        let (assign7210_e8654, assign7210_e8654_d_n0, assign7210_e8654_d_n1, assign7210_e8654_d_n2, assign7210_e8654_d_n3, assign7210_e8654_d_n4, assign7210_e8654_d_n5, assign7210_e8654_d_n6, assign7210_e8654_d_n7, assign7210_e8654_d_n8, assign7210_e8654_d_n9, assign7210_e8654_d_n10, assign7210_e8654_d_n11, assign7210_e8654_d_n12, assign7210_e8654_d_n13, assign7210_e8654_d_n14, assign7210_e8654_d_n15, assign7210_e8654_d_n16, assign7210_e8654_d_n17, assign7210_e8654_d_n18, assign7210_e8654_d_n19, assign7210_e8654_d_n20, assign7210_e8654_d_n21, assign7210_e8654_d_n22, assign7210_e8654_d_n23, assign7210_e8654_d_n24, assign7210_e8654_d_n25, assign7210_e8654_d_n26, assign7210_e8654_d_n27, assign7210_e8654_d_n28, assign7210_e8654_d_n29, assign7210_e8654_d_b0, assign7210_e8654_d_b1, assign7210_e8654_d_b2, assign7210_e8654_d_b3, assign7210_e8654_d_b4, assign7210_e8654_d_b5, assign7210_e8654_d_b6, assign7210_e8654_d_b7, assign7210_e8654_d_b8, assign7210_e8654_d_b9, assign7210_e8654_d_b10, assign7210_e8654_d_b11, assign7210_e8654_d_b12, assign7210_e8654_d_b13, assign7210_e8654_d_b14, assign7210_e8654_d_b15, assign7210_e8654_d_b16, assign7210_e8654_d_b17, assign7210_e8654_d_b18, assign7210_e8654_d_b19, assign7210_e8654_d_b20, assign7210_e8654_d_b21, assign7210_e8654_d_b22, assign7210_e8654_d_b23, assign7210_e8654_d_b24, assign7210_e8654_d_b25, assign7210_e8654_d_b26, assign7210_e8654_d_b27, assign7210_e8654_d_b28, assign7210_e8654_d_b29, assign7210_e8654_d_b30, assign7210_e8654_d_b31, assign7210_e8654_d_b32, assign7210_e8654_d_b33, assign7210_e8654_d_b34, assign7210_e8654_d_b35,) = {
    if ((((var_guard60 != 0.0) && (var_guard92 != 0.0)) && (var_guard93 == 0.0)) && (var_guard94 == 0.0)) {
        let assign7210_e8650: f64 = (var_fn61_calc_iq__etags).exp();
        let assign7210_e8651: f64 = (1.0 + assign7210_e8650);
        let assign7210_e8652: f64 = (assign7210_e8651).ln();
        (assign7210_e8652, ((assign7210_e8650 * var_fn61_calc_iq__etags_dn0) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn1) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn2) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn3) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn4) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn5) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn6) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn7) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn8) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn9) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn10) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn11) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn12) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn13) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn14) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn15) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn16) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn17) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn18) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn19) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn20) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn21) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn22) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn23) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn24) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn25) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn26) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn27) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn28) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_dn29) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db0) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db1) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db2) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db3) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db4) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db5) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db6) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db7) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db8) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db9) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db10) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db11) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db12) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db13) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db14) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db15) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db16) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db17) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db18) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db19) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db20) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db21) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db22) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db23) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db24) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db25) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db26) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db27) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db28) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db29) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db30) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db31) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db32) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db33) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db34) / assign7210_e8651), ((assign7210_e8650 * var_fn61_calc_iq__etags_db35) / assign7210_e8651),)
    } else {
        (var_fn61_calc_iq__exparg, var_fn61_calc_iq__exparg_dn0, var_fn61_calc_iq__exparg_dn1, var_fn61_calc_iq__exparg_dn2, var_fn61_calc_iq__exparg_dn3, var_fn61_calc_iq__exparg_dn4, var_fn61_calc_iq__exparg_dn5, var_fn61_calc_iq__exparg_dn6, var_fn61_calc_iq__exparg_dn7, var_fn61_calc_iq__exparg_dn8, var_fn61_calc_iq__exparg_dn9, var_fn61_calc_iq__exparg_dn10, var_fn61_calc_iq__exparg_dn11, var_fn61_calc_iq__exparg_dn12, var_fn61_calc_iq__exparg_dn13, var_fn61_calc_iq__exparg_dn14, var_fn61_calc_iq__exparg_dn15, var_fn61_calc_iq__exparg_dn16, var_fn61_calc_iq__exparg_dn17, var_fn61_calc_iq__exparg_dn18, var_fn61_calc_iq__exparg_dn19, var_fn61_calc_iq__exparg_dn20, var_fn61_calc_iq__exparg_dn21, var_fn61_calc_iq__exparg_dn22, var_fn61_calc_iq__exparg_dn23, var_fn61_calc_iq__exparg_dn24, var_fn61_calc_iq__exparg_dn25, var_fn61_calc_iq__exparg_dn26, var_fn61_calc_iq__exparg_dn27, var_fn61_calc_iq__exparg_dn28, var_fn61_calc_iq__exparg_dn29, var_fn61_calc_iq__exparg_db0, var_fn61_calc_iq__exparg_db1, var_fn61_calc_iq__exparg_db2, var_fn61_calc_iq__exparg_db3, var_fn61_calc_iq__exparg_db4, var_fn61_calc_iq__exparg_db5, var_fn61_calc_iq__exparg_db6, var_fn61_calc_iq__exparg_db7, var_fn61_calc_iq__exparg_db8, var_fn61_calc_iq__exparg_db9, var_fn61_calc_iq__exparg_db10, var_fn61_calc_iq__exparg_db11, var_fn61_calc_iq__exparg_db12, var_fn61_calc_iq__exparg_db13, var_fn61_calc_iq__exparg_db14, var_fn61_calc_iq__exparg_db15, var_fn61_calc_iq__exparg_db16, var_fn61_calc_iq__exparg_db17, var_fn61_calc_iq__exparg_db18, var_fn61_calc_iq__exparg_db19, var_fn61_calc_iq__exparg_db20, var_fn61_calc_iq__exparg_db21, var_fn61_calc_iq__exparg_db22, var_fn61_calc_iq__exparg_db23, var_fn61_calc_iq__exparg_db24, var_fn61_calc_iq__exparg_db25, var_fn61_calc_iq__exparg_db26, var_fn61_calc_iq__exparg_db27, var_fn61_calc_iq__exparg_db28, var_fn61_calc_iq__exparg_db29, var_fn61_calc_iq__exparg_db30, var_fn61_calc_iq__exparg_db31, var_fn61_calc_iq__exparg_db32, var_fn61_calc_iq__exparg_db33, var_fn61_calc_iq__exparg_db34, var_fn61_calc_iq__exparg_db35,)
    }
};
        var_fn61_calc_iq__exparg = assign7210_e8654;
        var_fn61_calc_iq__exparg_dn0 = assign7210_e8654_d_n0;
        var_fn61_calc_iq__exparg_dn1 = assign7210_e8654_d_n1;
        var_fn61_calc_iq__exparg_dn2 = assign7210_e8654_d_n2;
        var_fn61_calc_iq__exparg_dn3 = assign7210_e8654_d_n3;
        var_fn61_calc_iq__exparg_dn4 = assign7210_e8654_d_n4;
        var_fn61_calc_iq__exparg_dn5 = assign7210_e8654_d_n5;
        var_fn61_calc_iq__exparg_dn6 = assign7210_e8654_d_n6;
        var_fn61_calc_iq__exparg_dn7 = assign7210_e8654_d_n7;
        var_fn61_calc_iq__exparg_dn8 = assign7210_e8654_d_n8;
        var_fn61_calc_iq__exparg_dn9 = assign7210_e8654_d_n9;
        var_fn61_calc_iq__exparg_dn10 = assign7210_e8654_d_n10;
        var_fn61_calc_iq__exparg_dn11 = assign7210_e8654_d_n11;
        var_fn61_calc_iq__exparg_dn12 = assign7210_e8654_d_n12;
        var_fn61_calc_iq__exparg_dn13 = assign7210_e8654_d_n13;
        var_fn61_calc_iq__exparg_dn14 = assign7210_e8654_d_n14;
        var_fn61_calc_iq__exparg_dn15 = assign7210_e8654_d_n15;
        var_fn61_calc_iq__exparg_dn16 = assign7210_e8654_d_n16;
        var_fn61_calc_iq__exparg_dn17 = assign7210_e8654_d_n17;
        var_fn61_calc_iq__exparg_dn18 = assign7210_e8654_d_n18;
        var_fn61_calc_iq__exparg_dn19 = assign7210_e8654_d_n19;
        var_fn61_calc_iq__exparg_dn20 = assign7210_e8654_d_n20;
        var_fn61_calc_iq__exparg_dn21 = assign7210_e8654_d_n21;
        var_fn61_calc_iq__exparg_dn22 = assign7210_e8654_d_n22;
        var_fn61_calc_iq__exparg_dn23 = assign7210_e8654_d_n23;
        var_fn61_calc_iq__exparg_dn24 = assign7210_e8654_d_n24;
        var_fn61_calc_iq__exparg_dn25 = assign7210_e8654_d_n25;
        var_fn61_calc_iq__exparg_dn26 = assign7210_e8654_d_n26;
        var_fn61_calc_iq__exparg_dn27 = assign7210_e8654_d_n27;
        var_fn61_calc_iq__exparg_dn28 = assign7210_e8654_d_n28;
        var_fn61_calc_iq__exparg_dn29 = assign7210_e8654_d_n29;
        var_fn61_calc_iq__exparg_db0 = assign7210_e8654_d_b0;
        var_fn61_calc_iq__exparg_db1 = assign7210_e8654_d_b1;
        var_fn61_calc_iq__exparg_db2 = assign7210_e8654_d_b2;
        var_fn61_calc_iq__exparg_db3 = assign7210_e8654_d_b3;
        var_fn61_calc_iq__exparg_db4 = assign7210_e8654_d_b4;
        var_fn61_calc_iq__exparg_db5 = assign7210_e8654_d_b5;
        var_fn61_calc_iq__exparg_db6 = assign7210_e8654_d_b6;
        var_fn61_calc_iq__exparg_db7 = assign7210_e8654_d_b7;
        var_fn61_calc_iq__exparg_db8 = assign7210_e8654_d_b8;
        var_fn61_calc_iq__exparg_db9 = assign7210_e8654_d_b9;
        var_fn61_calc_iq__exparg_db10 = assign7210_e8654_d_b10;
        var_fn61_calc_iq__exparg_db11 = assign7210_e8654_d_b11;
        var_fn61_calc_iq__exparg_db12 = assign7210_e8654_d_b12;
        var_fn61_calc_iq__exparg_db13 = assign7210_e8654_d_b13;
        var_fn61_calc_iq__exparg_db14 = assign7210_e8654_d_b14;
        var_fn61_calc_iq__exparg_db15 = assign7210_e8654_d_b15;
        var_fn61_calc_iq__exparg_db16 = assign7210_e8654_d_b16;
        var_fn61_calc_iq__exparg_db17 = assign7210_e8654_d_b17;
        var_fn61_calc_iq__exparg_db18 = assign7210_e8654_d_b18;
        var_fn61_calc_iq__exparg_db19 = assign7210_e8654_d_b19;
        var_fn61_calc_iq__exparg_db20 = assign7210_e8654_d_b20;
        var_fn61_calc_iq__exparg_db21 = assign7210_e8654_d_b21;
        var_fn61_calc_iq__exparg_db22 = assign7210_e8654_d_b22;
        var_fn61_calc_iq__exparg_db23 = assign7210_e8654_d_b23;
        var_fn61_calc_iq__exparg_db24 = assign7210_e8654_d_b24;
        var_fn61_calc_iq__exparg_db25 = assign7210_e8654_d_b25;
        var_fn61_calc_iq__exparg_db26 = assign7210_e8654_d_b26;
        var_fn61_calc_iq__exparg_db27 = assign7210_e8654_d_b27;
        var_fn61_calc_iq__exparg_db28 = assign7210_e8654_d_b28;
        var_fn61_calc_iq__exparg_db29 = assign7210_e8654_d_b29;
        var_fn61_calc_iq__exparg_db30 = assign7210_e8654_d_b30;
        var_fn61_calc_iq__exparg_db31 = assign7210_e8654_d_b31;
        var_fn61_calc_iq__exparg_db32 = assign7210_e8654_d_b32;
        var_fn61_calc_iq__exparg_db33 = assign7210_e8654_d_b33;
        var_fn61_calc_iq__exparg_db34 = assign7210_e8654_d_b34;
        var_fn61_calc_iq__exparg_db35 = assign7210_e8654_d_b35;

        let (assign7220_e8672, assign7220_e8672_d_n0, assign7220_e8672_d_n1, assign7220_e8672_d_n2, assign7220_e8672_d_n3, assign7220_e8672_d_n4, assign7220_e8672_d_n5, assign7220_e8672_d_n6, assign7220_e8672_d_n7, assign7220_e8672_d_n8, assign7220_e8672_d_n9, assign7220_e8672_d_n10, assign7220_e8672_d_n11, assign7220_e8672_d_n12, assign7220_e8672_d_n13, assign7220_e8672_d_n14, assign7220_e8672_d_n15, assign7220_e8672_d_n16, assign7220_e8672_d_n17, assign7220_e8672_d_n18, assign7220_e8672_d_n19, assign7220_e8672_d_n20, assign7220_e8672_d_n21, assign7220_e8672_d_n22, assign7220_e8672_d_n23, assign7220_e8672_d_n24, assign7220_e8672_d_n25, assign7220_e8672_d_n26, assign7220_e8672_d_n27, assign7220_e8672_d_n28, assign7220_e8672_d_n29, assign7220_e8672_d_b0, assign7220_e8672_d_b1, assign7220_e8672_d_b2, assign7220_e8672_d_b3, assign7220_e8672_d_b4, assign7220_e8672_d_b5, assign7220_e8672_d_b6, assign7220_e8672_d_b7, assign7220_e8672_d_b8, assign7220_e8672_d_b9, assign7220_e8672_d_b10, assign7220_e8672_d_b11, assign7220_e8672_d_b12, assign7220_e8672_d_b13, assign7220_e8672_d_b14, assign7220_e8672_d_b15, assign7220_e8672_d_b16, assign7220_e8672_d_b17, assign7220_e8672_d_b18, assign7220_e8672_d_b19, assign7220_e8672_d_b20, assign7220_e8672_d_b21, assign7220_e8672_d_b22, assign7220_e8672_d_b23, assign7220_e8672_d_b24, assign7220_e8672_d_b25, assign7220_e8672_d_b26, assign7220_e8672_d_b27, assign7220_e8672_d_b28, assign7220_e8672_d_b29, assign7220_e8672_d_b30, assign7220_e8672_d_b31, assign7220_e8672_d_b32, assign7220_e8672_d_b33, assign7220_e8672_d_b34, assign7220_e8672_d_b35,) = {
    if ((var_guard60 != 0.0) && (var_guard92 != 0.0)) {
        let assign7220_e8660: f64 = (var_fn61_calc_iq__w * var_fn61_calc_iq__ngf);
        let assign7220_e8662: f64 = (assign7220_e8660 * var_fn61_calc_iq__type);
        let assign7220_e8664: f64 = (assign7220_e8662 * var_fn61_calc_iq__cs);
        let assign7220_e8666: f64 = (assign7220_e8664 * var_fn61_calc_iq__two_n_phit0);
        let assign7220_e8668: f64 = (assign7220_e8666 * var_fn61_calc_iq__exparg);
        let assign7220_e8670: f64 = (assign7220_e8668 * var_fn61_calc_iq__trapfracdl);
        (assign7220_e8670, ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn0) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn0)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn1) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn1)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn2) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn2)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn3) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn3)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn4) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn4)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn5) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn5)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn6) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn6)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn7) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn7)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn8) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn8)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn9) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn9)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn10) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn10)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn11) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn11)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn12) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn12)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn13) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn13)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn14) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn14)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn15) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn15)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn16) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn16)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn17) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn17)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn18) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn18)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn19) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn19)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn20) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn20)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn21) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn21)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn22) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn22)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn23) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn23)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn24) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn24)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn25) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn25)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn26) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn26)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn27) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn27)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn28) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn28)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_dn29) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_dn29)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db0) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db0)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db1) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db1)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db2) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db2)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db3) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db3)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db4) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db4)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db5) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db5)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db6) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db6)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db7) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db7)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db8) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db8)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db9) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db9)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db10) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db10)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db11) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db11)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db12) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db12)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db13) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db13)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db14) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db14)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db15) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db15)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db16) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db16)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db17) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db17)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db18) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db18)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db19) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db19)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db20) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db20)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db21) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db21)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db22) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db22)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db23) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db23)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db24) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db24)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db25) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db25)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db26) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db26)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db27) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db27)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db28) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db28)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db29) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db29)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db30) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db30)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db31) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db31)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db32) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db32)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db33) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db33)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db34) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db34)) * var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * var_fn61_calc_iq__two_n_phit0_db35) * var_fn61_calc_iq__exparg) + (assign7220_e8666 * var_fn61_calc_iq__exparg_db35)) * var_fn61_calc_iq__trapfracdl),)
    } else {
        (var_fn61_calc_iq__qsout, var_fn61_calc_iq__qsout_dn0, var_fn61_calc_iq__qsout_dn1, var_fn61_calc_iq__qsout_dn2, var_fn61_calc_iq__qsout_dn3, var_fn61_calc_iq__qsout_dn4, var_fn61_calc_iq__qsout_dn5, var_fn61_calc_iq__qsout_dn6, var_fn61_calc_iq__qsout_dn7, var_fn61_calc_iq__qsout_dn8, var_fn61_calc_iq__qsout_dn9, var_fn61_calc_iq__qsout_dn10, var_fn61_calc_iq__qsout_dn11, var_fn61_calc_iq__qsout_dn12, var_fn61_calc_iq__qsout_dn13, var_fn61_calc_iq__qsout_dn14, var_fn61_calc_iq__qsout_dn15, var_fn61_calc_iq__qsout_dn16, var_fn61_calc_iq__qsout_dn17, var_fn61_calc_iq__qsout_dn18, var_fn61_calc_iq__qsout_dn19, var_fn61_calc_iq__qsout_dn20, var_fn61_calc_iq__qsout_dn21, var_fn61_calc_iq__qsout_dn22, var_fn61_calc_iq__qsout_dn23, var_fn61_calc_iq__qsout_dn24, var_fn61_calc_iq__qsout_dn25, var_fn61_calc_iq__qsout_dn26, var_fn61_calc_iq__qsout_dn27, var_fn61_calc_iq__qsout_dn28, var_fn61_calc_iq__qsout_dn29, var_fn61_calc_iq__qsout_db0, var_fn61_calc_iq__qsout_db1, var_fn61_calc_iq__qsout_db2, var_fn61_calc_iq__qsout_db3, var_fn61_calc_iq__qsout_db4, var_fn61_calc_iq__qsout_db5, var_fn61_calc_iq__qsout_db6, var_fn61_calc_iq__qsout_db7, var_fn61_calc_iq__qsout_db8, var_fn61_calc_iq__qsout_db9, var_fn61_calc_iq__qsout_db10, var_fn61_calc_iq__qsout_db11, var_fn61_calc_iq__qsout_db12, var_fn61_calc_iq__qsout_db13, var_fn61_calc_iq__qsout_db14, var_fn61_calc_iq__qsout_db15, var_fn61_calc_iq__qsout_db16, var_fn61_calc_iq__qsout_db17, var_fn61_calc_iq__qsout_db18, var_fn61_calc_iq__qsout_db19, var_fn61_calc_iq__qsout_db20, var_fn61_calc_iq__qsout_db21, var_fn61_calc_iq__qsout_db22, var_fn61_calc_iq__qsout_db23, var_fn61_calc_iq__qsout_db24, var_fn61_calc_iq__qsout_db25, var_fn61_calc_iq__qsout_db26, var_fn61_calc_iq__qsout_db27, var_fn61_calc_iq__qsout_db28, var_fn61_calc_iq__qsout_db29, var_fn61_calc_iq__qsout_db30, var_fn61_calc_iq__qsout_db31, var_fn61_calc_iq__qsout_db32, var_fn61_calc_iq__qsout_db33, var_fn61_calc_iq__qsout_db34, var_fn61_calc_iq__qsout_db35,)
    }
};
        var_fn61_calc_iq__qsout = assign7220_e8672;
        var_fn61_calc_iq__qsout_dn0 = assign7220_e8672_d_n0;
        var_fn61_calc_iq__qsout_dn1 = assign7220_e8672_d_n1;
        var_fn61_calc_iq__qsout_dn2 = assign7220_e8672_d_n2;
        var_fn61_calc_iq__qsout_dn3 = assign7220_e8672_d_n3;
        var_fn61_calc_iq__qsout_dn4 = assign7220_e8672_d_n4;
        var_fn61_calc_iq__qsout_dn5 = assign7220_e8672_d_n5;
        var_fn61_calc_iq__qsout_dn6 = assign7220_e8672_d_n6;
        var_fn61_calc_iq__qsout_dn7 = assign7220_e8672_d_n7;
        var_fn61_calc_iq__qsout_dn8 = assign7220_e8672_d_n8;
        var_fn61_calc_iq__qsout_dn9 = assign7220_e8672_d_n9;
        var_fn61_calc_iq__qsout_dn10 = assign7220_e8672_d_n10;
        var_fn61_calc_iq__qsout_dn11 = assign7220_e8672_d_n11;
        var_fn61_calc_iq__qsout_dn12 = assign7220_e8672_d_n12;
        var_fn61_calc_iq__qsout_dn13 = assign7220_e8672_d_n13;
        var_fn61_calc_iq__qsout_dn14 = assign7220_e8672_d_n14;
        var_fn61_calc_iq__qsout_dn15 = assign7220_e8672_d_n15;
        var_fn61_calc_iq__qsout_dn16 = assign7220_e8672_d_n16;
        var_fn61_calc_iq__qsout_dn17 = assign7220_e8672_d_n17;
        var_fn61_calc_iq__qsout_dn18 = assign7220_e8672_d_n18;
        var_fn61_calc_iq__qsout_dn19 = assign7220_e8672_d_n19;
        var_fn61_calc_iq__qsout_dn20 = assign7220_e8672_d_n20;
        var_fn61_calc_iq__qsout_dn21 = assign7220_e8672_d_n21;
        var_fn61_calc_iq__qsout_dn22 = assign7220_e8672_d_n22;
        var_fn61_calc_iq__qsout_dn23 = assign7220_e8672_d_n23;
        var_fn61_calc_iq__qsout_dn24 = assign7220_e8672_d_n24;
        var_fn61_calc_iq__qsout_dn25 = assign7220_e8672_d_n25;
        var_fn61_calc_iq__qsout_dn26 = assign7220_e8672_d_n26;
        var_fn61_calc_iq__qsout_dn27 = assign7220_e8672_d_n27;
        var_fn61_calc_iq__qsout_dn28 = assign7220_e8672_d_n28;
        var_fn61_calc_iq__qsout_dn29 = assign7220_e8672_d_n29;
        var_fn61_calc_iq__qsout_db0 = assign7220_e8672_d_b0;
        var_fn61_calc_iq__qsout_db1 = assign7220_e8672_d_b1;
        var_fn61_calc_iq__qsout_db2 = assign7220_e8672_d_b2;
        var_fn61_calc_iq__qsout_db3 = assign7220_e8672_d_b3;
        var_fn61_calc_iq__qsout_db4 = assign7220_e8672_d_b4;
        var_fn61_calc_iq__qsout_db5 = assign7220_e8672_d_b5;
        var_fn61_calc_iq__qsout_db6 = assign7220_e8672_d_b6;
        var_fn61_calc_iq__qsout_db7 = assign7220_e8672_d_b7;
        var_fn61_calc_iq__qsout_db8 = assign7220_e8672_d_b8;
        var_fn61_calc_iq__qsout_db9 = assign7220_e8672_d_b9;
        var_fn61_calc_iq__qsout_db10 = assign7220_e8672_d_b10;
        var_fn61_calc_iq__qsout_db11 = assign7220_e8672_d_b11;
        var_fn61_calc_iq__qsout_db12 = assign7220_e8672_d_b12;
        var_fn61_calc_iq__qsout_db13 = assign7220_e8672_d_b13;
        var_fn61_calc_iq__qsout_db14 = assign7220_e8672_d_b14;
        var_fn61_calc_iq__qsout_db15 = assign7220_e8672_d_b15;
        var_fn61_calc_iq__qsout_db16 = assign7220_e8672_d_b16;
        var_fn61_calc_iq__qsout_db17 = assign7220_e8672_d_b17;
        var_fn61_calc_iq__qsout_db18 = assign7220_e8672_d_b18;
        var_fn61_calc_iq__qsout_db19 = assign7220_e8672_d_b19;
        var_fn61_calc_iq__qsout_db20 = assign7220_e8672_d_b20;
        var_fn61_calc_iq__qsout_db21 = assign7220_e8672_d_b21;
        var_fn61_calc_iq__qsout_db22 = assign7220_e8672_d_b22;
        var_fn61_calc_iq__qsout_db23 = assign7220_e8672_d_b23;
        var_fn61_calc_iq__qsout_db24 = assign7220_e8672_d_b24;
        var_fn61_calc_iq__qsout_db25 = assign7220_e8672_d_b25;
        var_fn61_calc_iq__qsout_db26 = assign7220_e8672_d_b26;
        var_fn61_calc_iq__qsout_db27 = assign7220_e8672_d_b27;
        var_fn61_calc_iq__qsout_db28 = assign7220_e8672_d_b28;
        var_fn61_calc_iq__qsout_db29 = assign7220_e8672_d_b29;
        var_fn61_calc_iq__qsout_db30 = assign7220_e8672_d_b30;
        var_fn61_calc_iq__qsout_db31 = assign7220_e8672_d_b31;
        var_fn61_calc_iq__qsout_db32 = assign7220_e8672_d_b32;
        var_fn61_calc_iq__qsout_db33 = assign7220_e8672_d_b33;
        var_fn61_calc_iq__qsout_db34 = assign7220_e8672_d_b34;
        var_fn61_calc_iq__qsout_db35 = assign7220_e8672_d_b35;

        let (assign7230_e8679, assign7230_e8679_d_n0, assign7230_e8679_d_n1, assign7230_e8679_d_n2, assign7230_e8679_d_n3, assign7230_e8679_d_n4, assign7230_e8679_d_n5, assign7230_e8679_d_n6, assign7230_e8679_d_n7, assign7230_e8679_d_n8, assign7230_e8679_d_n9, assign7230_e8679_d_n10, assign7230_e8679_d_n11, assign7230_e8679_d_n12, assign7230_e8679_d_n13, assign7230_e8679_d_n14, assign7230_e8679_d_n15, assign7230_e8679_d_n16, assign7230_e8679_d_n17, assign7230_e8679_d_n18, assign7230_e8679_d_n19, assign7230_e8679_d_n20, assign7230_e8679_d_n21, assign7230_e8679_d_n22, assign7230_e8679_d_n23, assign7230_e8679_d_n24, assign7230_e8679_d_n25, assign7230_e8679_d_n26, assign7230_e8679_d_n27, assign7230_e8679_d_n28, assign7230_e8679_d_n29, assign7230_e8679_d_b0, assign7230_e8679_d_b1, assign7230_e8679_d_b2, assign7230_e8679_d_b3, assign7230_e8679_d_b4, assign7230_e8679_d_b5, assign7230_e8679_d_b6, assign7230_e8679_d_b7, assign7230_e8679_d_b8, assign7230_e8679_d_b9, assign7230_e8679_d_b10, assign7230_e8679_d_b11, assign7230_e8679_d_b12, assign7230_e8679_d_b13, assign7230_e8679_d_b14, assign7230_e8679_d_b15, assign7230_e8679_d_b16, assign7230_e8679_d_b17, assign7230_e8679_d_b18, assign7230_e8679_d_b19, assign7230_e8679_d_b20, assign7230_e8679_d_b21, assign7230_e8679_d_b22, assign7230_e8679_d_b23, assign7230_e8679_d_b24, assign7230_e8679_d_b25, assign7230_e8679_d_b26, assign7230_e8679_d_b27, assign7230_e8679_d_b28, assign7230_e8679_d_b29, assign7230_e8679_d_b30, assign7230_e8679_d_b31, assign7230_e8679_d_b32, assign7230_e8679_d_b33, assign7230_e8679_d_b34, assign7230_e8679_d_b35,) = {
    if ((var_guard60 != 0.0) && (var_guard92 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn61_calc_iq__qsout, var_fn61_calc_iq__qsout_dn0, var_fn61_calc_iq__qsout_dn1, var_fn61_calc_iq__qsout_dn2, var_fn61_calc_iq__qsout_dn3, var_fn61_calc_iq__qsout_dn4, var_fn61_calc_iq__qsout_dn5, var_fn61_calc_iq__qsout_dn6, var_fn61_calc_iq__qsout_dn7, var_fn61_calc_iq__qsout_dn8, var_fn61_calc_iq__qsout_dn9, var_fn61_calc_iq__qsout_dn10, var_fn61_calc_iq__qsout_dn11, var_fn61_calc_iq__qsout_dn12, var_fn61_calc_iq__qsout_dn13, var_fn61_calc_iq__qsout_dn14, var_fn61_calc_iq__qsout_dn15, var_fn61_calc_iq__qsout_dn16, var_fn61_calc_iq__qsout_dn17, var_fn61_calc_iq__qsout_dn18, var_fn61_calc_iq__qsout_dn19, var_fn61_calc_iq__qsout_dn20, var_fn61_calc_iq__qsout_dn21, var_fn61_calc_iq__qsout_dn22, var_fn61_calc_iq__qsout_dn23, var_fn61_calc_iq__qsout_dn24, var_fn61_calc_iq__qsout_dn25, var_fn61_calc_iq__qsout_dn26, var_fn61_calc_iq__qsout_dn27, var_fn61_calc_iq__qsout_dn28, var_fn61_calc_iq__qsout_dn29, var_fn61_calc_iq__qsout_db0, var_fn61_calc_iq__qsout_db1, var_fn61_calc_iq__qsout_db2, var_fn61_calc_iq__qsout_db3, var_fn61_calc_iq__qsout_db4, var_fn61_calc_iq__qsout_db5, var_fn61_calc_iq__qsout_db6, var_fn61_calc_iq__qsout_db7, var_fn61_calc_iq__qsout_db8, var_fn61_calc_iq__qsout_db9, var_fn61_calc_iq__qsout_db10, var_fn61_calc_iq__qsout_db11, var_fn61_calc_iq__qsout_db12, var_fn61_calc_iq__qsout_db13, var_fn61_calc_iq__qsout_db14, var_fn61_calc_iq__qsout_db15, var_fn61_calc_iq__qsout_db16, var_fn61_calc_iq__qsout_db17, var_fn61_calc_iq__qsout_db18, var_fn61_calc_iq__qsout_db19, var_fn61_calc_iq__qsout_db20, var_fn61_calc_iq__qsout_db21, var_fn61_calc_iq__qsout_db22, var_fn61_calc_iq__qsout_db23, var_fn61_calc_iq__qsout_db24, var_fn61_calc_iq__qsout_db25, var_fn61_calc_iq__qsout_db26, var_fn61_calc_iq__qsout_db27, var_fn61_calc_iq__qsout_db28, var_fn61_calc_iq__qsout_db29, var_fn61_calc_iq__qsout_db30, var_fn61_calc_iq__qsout_db31, var_fn61_calc_iq__qsout_db32, var_fn61_calc_iq__qsout_db33, var_fn61_calc_iq__qsout_db34, var_fn61_calc_iq__qsout_db35,)
    }
};
        var_fn61_calc_iq__qsout = assign7230_e8679;
        var_fn61_calc_iq__qsout_dn0 = assign7230_e8679_d_n0;
        var_fn61_calc_iq__qsout_dn1 = assign7230_e8679_d_n1;
        var_fn61_calc_iq__qsout_dn2 = assign7230_e8679_d_n2;
        var_fn61_calc_iq__qsout_dn3 = assign7230_e8679_d_n3;
        var_fn61_calc_iq__qsout_dn4 = assign7230_e8679_d_n4;
        var_fn61_calc_iq__qsout_dn5 = assign7230_e8679_d_n5;
        var_fn61_calc_iq__qsout_dn6 = assign7230_e8679_d_n6;
        var_fn61_calc_iq__qsout_dn7 = assign7230_e8679_d_n7;
        var_fn61_calc_iq__qsout_dn8 = assign7230_e8679_d_n8;
        var_fn61_calc_iq__qsout_dn9 = assign7230_e8679_d_n9;
        var_fn61_calc_iq__qsout_dn10 = assign7230_e8679_d_n10;
        var_fn61_calc_iq__qsout_dn11 = assign7230_e8679_d_n11;
        var_fn61_calc_iq__qsout_dn12 = assign7230_e8679_d_n12;
        var_fn61_calc_iq__qsout_dn13 = assign7230_e8679_d_n13;
        var_fn61_calc_iq__qsout_dn14 = assign7230_e8679_d_n14;
        var_fn61_calc_iq__qsout_dn15 = assign7230_e8679_d_n15;
        var_fn61_calc_iq__qsout_dn16 = assign7230_e8679_d_n16;
        var_fn61_calc_iq__qsout_dn17 = assign7230_e8679_d_n17;
        var_fn61_calc_iq__qsout_dn18 = assign7230_e8679_d_n18;
        var_fn61_calc_iq__qsout_dn19 = assign7230_e8679_d_n19;
        var_fn61_calc_iq__qsout_dn20 = assign7230_e8679_d_n20;
        var_fn61_calc_iq__qsout_dn21 = assign7230_e8679_d_n21;
        var_fn61_calc_iq__qsout_dn22 = assign7230_e8679_d_n22;
        var_fn61_calc_iq__qsout_dn23 = assign7230_e8679_d_n23;
        var_fn61_calc_iq__qsout_dn24 = assign7230_e8679_d_n24;
        var_fn61_calc_iq__qsout_dn25 = assign7230_e8679_d_n25;
        var_fn61_calc_iq__qsout_dn26 = assign7230_e8679_d_n26;
        var_fn61_calc_iq__qsout_dn27 = assign7230_e8679_d_n27;
        var_fn61_calc_iq__qsout_dn28 = assign7230_e8679_d_n28;
        var_fn61_calc_iq__qsout_dn29 = assign7230_e8679_d_n29;
        var_fn61_calc_iq__qsout_db0 = assign7230_e8679_d_b0;
        var_fn61_calc_iq__qsout_db1 = assign7230_e8679_d_b1;
        var_fn61_calc_iq__qsout_db2 = assign7230_e8679_d_b2;
        var_fn61_calc_iq__qsout_db3 = assign7230_e8679_d_b3;
        var_fn61_calc_iq__qsout_db4 = assign7230_e8679_d_b4;
        var_fn61_calc_iq__qsout_db5 = assign7230_e8679_d_b5;
        var_fn61_calc_iq__qsout_db6 = assign7230_e8679_d_b6;
        var_fn61_calc_iq__qsout_db7 = assign7230_e8679_d_b7;
        var_fn61_calc_iq__qsout_db8 = assign7230_e8679_d_b8;
        var_fn61_calc_iq__qsout_db9 = assign7230_e8679_d_b9;
        var_fn61_calc_iq__qsout_db10 = assign7230_e8679_d_b10;
        var_fn61_calc_iq__qsout_db11 = assign7230_e8679_d_b11;
        var_fn61_calc_iq__qsout_db12 = assign7230_e8679_d_b12;
        var_fn61_calc_iq__qsout_db13 = assign7230_e8679_d_b13;
        var_fn61_calc_iq__qsout_db14 = assign7230_e8679_d_b14;
        var_fn61_calc_iq__qsout_db15 = assign7230_e8679_d_b15;
        var_fn61_calc_iq__qsout_db16 = assign7230_e8679_d_b16;
        var_fn61_calc_iq__qsout_db17 = assign7230_e8679_d_b17;
        var_fn61_calc_iq__qsout_db18 = assign7230_e8679_d_b18;
        var_fn61_calc_iq__qsout_db19 = assign7230_e8679_d_b19;
        var_fn61_calc_iq__qsout_db20 = assign7230_e8679_d_b20;
        var_fn61_calc_iq__qsout_db21 = assign7230_e8679_d_b21;
        var_fn61_calc_iq__qsout_db22 = assign7230_e8679_d_b22;
        var_fn61_calc_iq__qsout_db23 = assign7230_e8679_d_b23;
        var_fn61_calc_iq__qsout_db24 = assign7230_e8679_d_b24;
        var_fn61_calc_iq__qsout_db25 = assign7230_e8679_d_b25;
        var_fn61_calc_iq__qsout_db26 = assign7230_e8679_d_b26;
        var_fn61_calc_iq__qsout_db27 = assign7230_e8679_d_b27;
        var_fn61_calc_iq__qsout_db28 = assign7230_e8679_d_b28;
        var_fn61_calc_iq__qsout_db29 = assign7230_e8679_d_b29;
        var_fn61_calc_iq__qsout_db30 = assign7230_e8679_d_b30;
        var_fn61_calc_iq__qsout_db31 = assign7230_e8679_d_b31;
        var_fn61_calc_iq__qsout_db32 = assign7230_e8679_d_b32;
        var_fn61_calc_iq__qsout_db33 = assign7230_e8679_d_b33;
        var_fn61_calc_iq__qsout_db34 = assign7230_e8679_d_b34;
        var_fn61_calc_iq__qsout_db35 = assign7230_e8679_d_b35;

        let (assign7240_e8683, assign7240_e8683_d_n0, assign7240_e8683_d_n1, assign7240_e8683_d_n2, assign7240_e8683_d_n3, assign7240_e8683_d_n4, assign7240_e8683_d_n5, assign7240_e8683_d_n6, assign7240_e8683_d_n7, assign7240_e8683_d_n8, assign7240_e8683_d_n9, assign7240_e8683_d_n10, assign7240_e8683_d_n11, assign7240_e8683_d_n12, assign7240_e8683_d_n13, assign7240_e8683_d_n14, assign7240_e8683_d_n15, assign7240_e8683_d_n16, assign7240_e8683_d_n17, assign7240_e8683_d_n18, assign7240_e8683_d_n19, assign7240_e8683_d_n20, assign7240_e8683_d_n21, assign7240_e8683_d_n22, assign7240_e8683_d_n23, assign7240_e8683_d_n24, assign7240_e8683_d_n25, assign7240_e8683_d_n26, assign7240_e8683_d_n27, assign7240_e8683_d_n28, assign7240_e8683_d_n29, assign7240_e8683_d_b0, assign7240_e8683_d_b1, assign7240_e8683_d_b2, assign7240_e8683_d_b3, assign7240_e8683_d_b4, assign7240_e8683_d_b5, assign7240_e8683_d_b6, assign7240_e8683_d_b7, assign7240_e8683_d_b8, assign7240_e8683_d_b9, assign7240_e8683_d_b10, assign7240_e8683_d_b11, assign7240_e8683_d_b12, assign7240_e8683_d_b13, assign7240_e8683_d_b14, assign7240_e8683_d_b15, assign7240_e8683_d_b16, assign7240_e8683_d_b17, assign7240_e8683_d_b18, assign7240_e8683_d_b19, assign7240_e8683_d_b20, assign7240_e8683_d_b21, assign7240_e8683_d_b22, assign7240_e8683_d_b23, assign7240_e8683_d_b24, assign7240_e8683_d_b25, assign7240_e8683_d_b26, assign7240_e8683_d_b27, assign7240_e8683_d_b28, assign7240_e8683_d_b29, assign7240_e8683_d_b30, assign7240_e8683_d_b31, assign7240_e8683_d_b32, assign7240_e8683_d_b33, assign7240_e8683_d_b34, assign7240_e8683_d_b35,) = {
    if (var_guard60 != 0.0) {
        (var_fn61_calc_iq__idsout, var_fn61_calc_iq__idsout_dn0, var_fn61_calc_iq__idsout_dn1, var_fn61_calc_iq__idsout_dn2, var_fn61_calc_iq__idsout_dn3, var_fn61_calc_iq__idsout_dn4, var_fn61_calc_iq__idsout_dn5, var_fn61_calc_iq__idsout_dn6, var_fn61_calc_iq__idsout_dn7, var_fn61_calc_iq__idsout_dn8, var_fn61_calc_iq__idsout_dn9, var_fn61_calc_iq__idsout_dn10, var_fn61_calc_iq__idsout_dn11, var_fn61_calc_iq__idsout_dn12, var_fn61_calc_iq__idsout_dn13, var_fn61_calc_iq__idsout_dn14, var_fn61_calc_iq__idsout_dn15, var_fn61_calc_iq__idsout_dn16, var_fn61_calc_iq__idsout_dn17, var_fn61_calc_iq__idsout_dn18, var_fn61_calc_iq__idsout_dn19, var_fn61_calc_iq__idsout_dn20, var_fn61_calc_iq__idsout_dn21, var_fn61_calc_iq__idsout_dn22, var_fn61_calc_iq__idsout_dn23, var_fn61_calc_iq__idsout_dn24, var_fn61_calc_iq__idsout_dn25, var_fn61_calc_iq__idsout_dn26, var_fn61_calc_iq__idsout_dn27, var_fn61_calc_iq__idsout_dn28, var_fn61_calc_iq__idsout_dn29, var_fn61_calc_iq__idsout_db0, var_fn61_calc_iq__idsout_db1, var_fn61_calc_iq__idsout_db2, var_fn61_calc_iq__idsout_db3, var_fn61_calc_iq__idsout_db4, var_fn61_calc_iq__idsout_db5, var_fn61_calc_iq__idsout_db6, var_fn61_calc_iq__idsout_db7, var_fn61_calc_iq__idsout_db8, var_fn61_calc_iq__idsout_db9, var_fn61_calc_iq__idsout_db10, var_fn61_calc_iq__idsout_db11, var_fn61_calc_iq__idsout_db12, var_fn61_calc_iq__idsout_db13, var_fn61_calc_iq__idsout_db14, var_fn61_calc_iq__idsout_db15, var_fn61_calc_iq__idsout_db16, var_fn61_calc_iq__idsout_db17, var_fn61_calc_iq__idsout_db18, var_fn61_calc_iq__idsout_db19, var_fn61_calc_iq__idsout_db20, var_fn61_calc_iq__idsout_db21, var_fn61_calc_iq__idsout_db22, var_fn61_calc_iq__idsout_db23, var_fn61_calc_iq__idsout_db24, var_fn61_calc_iq__idsout_db25, var_fn61_calc_iq__idsout_db26, var_fn61_calc_iq__idsout_db27, var_fn61_calc_iq__idsout_db28, var_fn61_calc_iq__idsout_db29, var_fn61_calc_iq__idsout_db30, var_fn61_calc_iq__idsout_db31, var_fn61_calc_iq__idsout_db32, var_fn61_calc_iq__idsout_db33, var_fn61_calc_iq__idsout_db34, var_fn61_calc_iq__idsout_db35,)
    } else {
        (var_fn61_calc_iq__return, var_fn61_calc_iq__return_dn0, var_fn61_calc_iq__return_dn1, var_fn61_calc_iq__return_dn2, var_fn61_calc_iq__return_dn3, var_fn61_calc_iq__return_dn4, var_fn61_calc_iq__return_dn5, var_fn61_calc_iq__return_dn6, var_fn61_calc_iq__return_dn7, var_fn61_calc_iq__return_dn8, var_fn61_calc_iq__return_dn9, var_fn61_calc_iq__return_dn10, var_fn61_calc_iq__return_dn11, var_fn61_calc_iq__return_dn12, var_fn61_calc_iq__return_dn13, var_fn61_calc_iq__return_dn14, var_fn61_calc_iq__return_dn15, var_fn61_calc_iq__return_dn16, var_fn61_calc_iq__return_dn17, var_fn61_calc_iq__return_dn18, var_fn61_calc_iq__return_dn19, var_fn61_calc_iq__return_dn20, var_fn61_calc_iq__return_dn21, var_fn61_calc_iq__return_dn22, var_fn61_calc_iq__return_dn23, var_fn61_calc_iq__return_dn24, var_fn61_calc_iq__return_dn25, var_fn61_calc_iq__return_dn26, var_fn61_calc_iq__return_dn27, var_fn61_calc_iq__return_dn28, var_fn61_calc_iq__return_dn29, var_fn61_calc_iq__return_db0, var_fn61_calc_iq__return_db1, var_fn61_calc_iq__return_db2, var_fn61_calc_iq__return_db3, var_fn61_calc_iq__return_db4, var_fn61_calc_iq__return_db5, var_fn61_calc_iq__return_db6, var_fn61_calc_iq__return_db7, var_fn61_calc_iq__return_db8, var_fn61_calc_iq__return_db9, var_fn61_calc_iq__return_db10, var_fn61_calc_iq__return_db11, var_fn61_calc_iq__return_db12, var_fn61_calc_iq__return_db13, var_fn61_calc_iq__return_db14, var_fn61_calc_iq__return_db15, var_fn61_calc_iq__return_db16, var_fn61_calc_iq__return_db17, var_fn61_calc_iq__return_db18, var_fn61_calc_iq__return_db19, var_fn61_calc_iq__return_db20, var_fn61_calc_iq__return_db21, var_fn61_calc_iq__return_db22, var_fn61_calc_iq__return_db23, var_fn61_calc_iq__return_db24, var_fn61_calc_iq__return_db25, var_fn61_calc_iq__return_db26, var_fn61_calc_iq__return_db27, var_fn61_calc_iq__return_db28, var_fn61_calc_iq__return_db29, var_fn61_calc_iq__return_db30, var_fn61_calc_iq__return_db31, var_fn61_calc_iq__return_db32, var_fn61_calc_iq__return_db33, var_fn61_calc_iq__return_db34, var_fn61_calc_iq__return_db35,)
    }
};
        var_fn61_calc_iq__return = assign7240_e8683;
        var_fn61_calc_iq__return_dn0 = assign7240_e8683_d_n0;
        var_fn61_calc_iq__return_dn1 = assign7240_e8683_d_n1;
        var_fn61_calc_iq__return_dn2 = assign7240_e8683_d_n2;
        var_fn61_calc_iq__return_dn3 = assign7240_e8683_d_n3;
        var_fn61_calc_iq__return_dn4 = assign7240_e8683_d_n4;
        var_fn61_calc_iq__return_dn5 = assign7240_e8683_d_n5;
        var_fn61_calc_iq__return_dn6 = assign7240_e8683_d_n6;
        var_fn61_calc_iq__return_dn7 = assign7240_e8683_d_n7;
        var_fn61_calc_iq__return_dn8 = assign7240_e8683_d_n8;
        var_fn61_calc_iq__return_dn9 = assign7240_e8683_d_n9;
        var_fn61_calc_iq__return_dn10 = assign7240_e8683_d_n10;
        var_fn61_calc_iq__return_dn11 = assign7240_e8683_d_n11;
        var_fn61_calc_iq__return_dn12 = assign7240_e8683_d_n12;
        var_fn61_calc_iq__return_dn13 = assign7240_e8683_d_n13;
        var_fn61_calc_iq__return_dn14 = assign7240_e8683_d_n14;
        var_fn61_calc_iq__return_dn15 = assign7240_e8683_d_n15;
        var_fn61_calc_iq__return_dn16 = assign7240_e8683_d_n16;
        var_fn61_calc_iq__return_dn17 = assign7240_e8683_d_n17;
        var_fn61_calc_iq__return_dn18 = assign7240_e8683_d_n18;
        var_fn61_calc_iq__return_dn19 = assign7240_e8683_d_n19;
        var_fn61_calc_iq__return_dn20 = assign7240_e8683_d_n20;
        var_fn61_calc_iq__return_dn21 = assign7240_e8683_d_n21;
        var_fn61_calc_iq__return_dn22 = assign7240_e8683_d_n22;
        var_fn61_calc_iq__return_dn23 = assign7240_e8683_d_n23;
        var_fn61_calc_iq__return_dn24 = assign7240_e8683_d_n24;
        var_fn61_calc_iq__return_dn25 = assign7240_e8683_d_n25;
        var_fn61_calc_iq__return_dn26 = assign7240_e8683_d_n26;
        var_fn61_calc_iq__return_dn27 = assign7240_e8683_d_n27;
        var_fn61_calc_iq__return_dn28 = assign7240_e8683_d_n28;
        var_fn61_calc_iq__return_dn29 = assign7240_e8683_d_n29;
        var_fn61_calc_iq__return_db0 = assign7240_e8683_d_b0;
        var_fn61_calc_iq__return_db1 = assign7240_e8683_d_b1;
        var_fn61_calc_iq__return_db2 = assign7240_e8683_d_b2;
        var_fn61_calc_iq__return_db3 = assign7240_e8683_d_b3;
        var_fn61_calc_iq__return_db4 = assign7240_e8683_d_b4;
        var_fn61_calc_iq__return_db5 = assign7240_e8683_d_b5;
        var_fn61_calc_iq__return_db6 = assign7240_e8683_d_b6;
        var_fn61_calc_iq__return_db7 = assign7240_e8683_d_b7;
        var_fn61_calc_iq__return_db8 = assign7240_e8683_d_b8;
        var_fn61_calc_iq__return_db9 = assign7240_e8683_d_b9;
        var_fn61_calc_iq__return_db10 = assign7240_e8683_d_b10;
        var_fn61_calc_iq__return_db11 = assign7240_e8683_d_b11;
        var_fn61_calc_iq__return_db12 = assign7240_e8683_d_b12;
        var_fn61_calc_iq__return_db13 = assign7240_e8683_d_b13;
        var_fn61_calc_iq__return_db14 = assign7240_e8683_d_b14;
        var_fn61_calc_iq__return_db15 = assign7240_e8683_d_b15;
        var_fn61_calc_iq__return_db16 = assign7240_e8683_d_b16;
        var_fn61_calc_iq__return_db17 = assign7240_e8683_d_b17;
        var_fn61_calc_iq__return_db18 = assign7240_e8683_d_b18;
        var_fn61_calc_iq__return_db19 = assign7240_e8683_d_b19;
        var_fn61_calc_iq__return_db20 = assign7240_e8683_d_b20;
        var_fn61_calc_iq__return_db21 = assign7240_e8683_d_b21;
        var_fn61_calc_iq__return_db22 = assign7240_e8683_d_b22;
        var_fn61_calc_iq__return_db23 = assign7240_e8683_d_b23;
        var_fn61_calc_iq__return_db24 = assign7240_e8683_d_b24;
        var_fn61_calc_iq__return_db25 = assign7240_e8683_d_b25;
        var_fn61_calc_iq__return_db26 = assign7240_e8683_d_b26;
        var_fn61_calc_iq__return_db27 = assign7240_e8683_d_b27;
        var_fn61_calc_iq__return_db28 = assign7240_e8683_d_b28;
        var_fn61_calc_iq__return_db29 = assign7240_e8683_d_b29;
        var_fn61_calc_iq__return_db30 = assign7240_e8683_d_b30;
        var_fn61_calc_iq__return_db31 = assign7240_e8683_d_b31;
        var_fn61_calc_iq__return_db32 = assign7240_e8683_d_b32;
        var_fn61_calc_iq__return_db33 = assign7240_e8683_d_b33;
        var_fn61_calc_iq__return_db34 = assign7240_e8683_d_b34;
        var_fn61_calc_iq__return_db35 = assign7240_e8683_d_b35;


        *var_fn61_calc_iq__exparg_slot = var_fn61_calc_iq__exparg;
        *var_fn61_calc_iq__exparg_db0_slot = var_fn61_calc_iq__exparg_db0;
        *var_fn61_calc_iq__exparg_db1_slot = var_fn61_calc_iq__exparg_db1;
        *var_fn61_calc_iq__exparg_db10_slot = var_fn61_calc_iq__exparg_db10;
        *var_fn61_calc_iq__exparg_db11_slot = var_fn61_calc_iq__exparg_db11;
        *var_fn61_calc_iq__exparg_db12_slot = var_fn61_calc_iq__exparg_db12;
        *var_fn61_calc_iq__exparg_db13_slot = var_fn61_calc_iq__exparg_db13;
        *var_fn61_calc_iq__exparg_db14_slot = var_fn61_calc_iq__exparg_db14;
        *var_fn61_calc_iq__exparg_db15_slot = var_fn61_calc_iq__exparg_db15;
        *var_fn61_calc_iq__exparg_db16_slot = var_fn61_calc_iq__exparg_db16;
        *var_fn61_calc_iq__exparg_db17_slot = var_fn61_calc_iq__exparg_db17;
        *var_fn61_calc_iq__exparg_db18_slot = var_fn61_calc_iq__exparg_db18;
        *var_fn61_calc_iq__exparg_db19_slot = var_fn61_calc_iq__exparg_db19;
        *var_fn61_calc_iq__exparg_db2_slot = var_fn61_calc_iq__exparg_db2;
        *var_fn61_calc_iq__exparg_db20_slot = var_fn61_calc_iq__exparg_db20;
        *var_fn61_calc_iq__exparg_db21_slot = var_fn61_calc_iq__exparg_db21;
        *var_fn61_calc_iq__exparg_db22_slot = var_fn61_calc_iq__exparg_db22;
        *var_fn61_calc_iq__exparg_db23_slot = var_fn61_calc_iq__exparg_db23;
        *var_fn61_calc_iq__exparg_db24_slot = var_fn61_calc_iq__exparg_db24;
        *var_fn61_calc_iq__exparg_db25_slot = var_fn61_calc_iq__exparg_db25;
        *var_fn61_calc_iq__exparg_db26_slot = var_fn61_calc_iq__exparg_db26;
        *var_fn61_calc_iq__exparg_db27_slot = var_fn61_calc_iq__exparg_db27;
        *var_fn61_calc_iq__exparg_db28_slot = var_fn61_calc_iq__exparg_db28;
        *var_fn61_calc_iq__exparg_db29_slot = var_fn61_calc_iq__exparg_db29;
        *var_fn61_calc_iq__exparg_db3_slot = var_fn61_calc_iq__exparg_db3;
        *var_fn61_calc_iq__exparg_db30_slot = var_fn61_calc_iq__exparg_db30;
        *var_fn61_calc_iq__exparg_db31_slot = var_fn61_calc_iq__exparg_db31;
        *var_fn61_calc_iq__exparg_db32_slot = var_fn61_calc_iq__exparg_db32;
        *var_fn61_calc_iq__exparg_db33_slot = var_fn61_calc_iq__exparg_db33;
        *var_fn61_calc_iq__exparg_db34_slot = var_fn61_calc_iq__exparg_db34;
        *var_fn61_calc_iq__exparg_db35_slot = var_fn61_calc_iq__exparg_db35;
        *var_fn61_calc_iq__exparg_db4_slot = var_fn61_calc_iq__exparg_db4;
        *var_fn61_calc_iq__exparg_db5_slot = var_fn61_calc_iq__exparg_db5;
        *var_fn61_calc_iq__exparg_db6_slot = var_fn61_calc_iq__exparg_db6;
        *var_fn61_calc_iq__exparg_db7_slot = var_fn61_calc_iq__exparg_db7;
        *var_fn61_calc_iq__exparg_db8_slot = var_fn61_calc_iq__exparg_db8;
        *var_fn61_calc_iq__exparg_db9_slot = var_fn61_calc_iq__exparg_db9;
        *var_fn61_calc_iq__exparg_dn0_slot = var_fn61_calc_iq__exparg_dn0;
        *var_fn61_calc_iq__exparg_dn1_slot = var_fn61_calc_iq__exparg_dn1;
        *var_fn61_calc_iq__exparg_dn10_slot = var_fn61_calc_iq__exparg_dn10;
        *var_fn61_calc_iq__exparg_dn11_slot = var_fn61_calc_iq__exparg_dn11;
        *var_fn61_calc_iq__exparg_dn12_slot = var_fn61_calc_iq__exparg_dn12;
        *var_fn61_calc_iq__exparg_dn13_slot = var_fn61_calc_iq__exparg_dn13;
        *var_fn61_calc_iq__exparg_dn14_slot = var_fn61_calc_iq__exparg_dn14;
        *var_fn61_calc_iq__exparg_dn15_slot = var_fn61_calc_iq__exparg_dn15;
        *var_fn61_calc_iq__exparg_dn16_slot = var_fn61_calc_iq__exparg_dn16;
        *var_fn61_calc_iq__exparg_dn17_slot = var_fn61_calc_iq__exparg_dn17;
        *var_fn61_calc_iq__exparg_dn18_slot = var_fn61_calc_iq__exparg_dn18;
        *var_fn61_calc_iq__exparg_dn19_slot = var_fn61_calc_iq__exparg_dn19;
        *var_fn61_calc_iq__exparg_dn2_slot = var_fn61_calc_iq__exparg_dn2;
        *var_fn61_calc_iq__exparg_dn20_slot = var_fn61_calc_iq__exparg_dn20;
        *var_fn61_calc_iq__exparg_dn21_slot = var_fn61_calc_iq__exparg_dn21;
        *var_fn61_calc_iq__exparg_dn22_slot = var_fn61_calc_iq__exparg_dn22;
        *var_fn61_calc_iq__exparg_dn23_slot = var_fn61_calc_iq__exparg_dn23;
        *var_fn61_calc_iq__exparg_dn24_slot = var_fn61_calc_iq__exparg_dn24;
        *var_fn61_calc_iq__exparg_dn25_slot = var_fn61_calc_iq__exparg_dn25;
        *var_fn61_calc_iq__exparg_dn26_slot = var_fn61_calc_iq__exparg_dn26;
        *var_fn61_calc_iq__exparg_dn27_slot = var_fn61_calc_iq__exparg_dn27;
        *var_fn61_calc_iq__exparg_dn28_slot = var_fn61_calc_iq__exparg_dn28;
        *var_fn61_calc_iq__exparg_dn29_slot = var_fn61_calc_iq__exparg_dn29;
        *var_fn61_calc_iq__exparg_dn3_slot = var_fn61_calc_iq__exparg_dn3;
        *var_fn61_calc_iq__exparg_dn4_slot = var_fn61_calc_iq__exparg_dn4;
        *var_fn61_calc_iq__exparg_dn5_slot = var_fn61_calc_iq__exparg_dn5;
        *var_fn61_calc_iq__exparg_dn6_slot = var_fn61_calc_iq__exparg_dn6;
        *var_fn61_calc_iq__exparg_dn7_slot = var_fn61_calc_iq__exparg_dn7;
        *var_fn61_calc_iq__exparg_dn8_slot = var_fn61_calc_iq__exparg_dn8;
        *var_fn61_calc_iq__exparg_dn9_slot = var_fn61_calc_iq__exparg_dn9;
        *var_fn61_calc_iq__qsout_slot = var_fn61_calc_iq__qsout;
        *var_fn61_calc_iq__qsout_db0_slot = var_fn61_calc_iq__qsout_db0;
        *var_fn61_calc_iq__qsout_db1_slot = var_fn61_calc_iq__qsout_db1;
        *var_fn61_calc_iq__qsout_db10_slot = var_fn61_calc_iq__qsout_db10;
        *var_fn61_calc_iq__qsout_db11_slot = var_fn61_calc_iq__qsout_db11;
        *var_fn61_calc_iq__qsout_db12_slot = var_fn61_calc_iq__qsout_db12;
        *var_fn61_calc_iq__qsout_db13_slot = var_fn61_calc_iq__qsout_db13;
        *var_fn61_calc_iq__qsout_db14_slot = var_fn61_calc_iq__qsout_db14;
        *var_fn61_calc_iq__qsout_db15_slot = var_fn61_calc_iq__qsout_db15;
        *var_fn61_calc_iq__qsout_db16_slot = var_fn61_calc_iq__qsout_db16;
        *var_fn61_calc_iq__qsout_db17_slot = var_fn61_calc_iq__qsout_db17;
        *var_fn61_calc_iq__qsout_db18_slot = var_fn61_calc_iq__qsout_db18;
        *var_fn61_calc_iq__qsout_db19_slot = var_fn61_calc_iq__qsout_db19;
        *var_fn61_calc_iq__qsout_db2_slot = var_fn61_calc_iq__qsout_db2;
        *var_fn61_calc_iq__qsout_db20_slot = var_fn61_calc_iq__qsout_db20;
        *var_fn61_calc_iq__qsout_db21_slot = var_fn61_calc_iq__qsout_db21;
        *var_fn61_calc_iq__qsout_db22_slot = var_fn61_calc_iq__qsout_db22;
        *var_fn61_calc_iq__qsout_db23_slot = var_fn61_calc_iq__qsout_db23;
        *var_fn61_calc_iq__qsout_db24_slot = var_fn61_calc_iq__qsout_db24;
        *var_fn61_calc_iq__qsout_db25_slot = var_fn61_calc_iq__qsout_db25;
        *var_fn61_calc_iq__qsout_db26_slot = var_fn61_calc_iq__qsout_db26;
        *var_fn61_calc_iq__qsout_db27_slot = var_fn61_calc_iq__qsout_db27;
        *var_fn61_calc_iq__qsout_db28_slot = var_fn61_calc_iq__qsout_db28;
        *var_fn61_calc_iq__qsout_db29_slot = var_fn61_calc_iq__qsout_db29;
        *var_fn61_calc_iq__qsout_db3_slot = var_fn61_calc_iq__qsout_db3;
        *var_fn61_calc_iq__qsout_db30_slot = var_fn61_calc_iq__qsout_db30;
        *var_fn61_calc_iq__qsout_db31_slot = var_fn61_calc_iq__qsout_db31;
        *var_fn61_calc_iq__qsout_db32_slot = var_fn61_calc_iq__qsout_db32;
        *var_fn61_calc_iq__qsout_db33_slot = var_fn61_calc_iq__qsout_db33;
        *var_fn61_calc_iq__qsout_db34_slot = var_fn61_calc_iq__qsout_db34;
        *var_fn61_calc_iq__qsout_db35_slot = var_fn61_calc_iq__qsout_db35;
        *var_fn61_calc_iq__qsout_db4_slot = var_fn61_calc_iq__qsout_db4;
        *var_fn61_calc_iq__qsout_db5_slot = var_fn61_calc_iq__qsout_db5;
        *var_fn61_calc_iq__qsout_db6_slot = var_fn61_calc_iq__qsout_db6;
        *var_fn61_calc_iq__qsout_db7_slot = var_fn61_calc_iq__qsout_db7;
        *var_fn61_calc_iq__qsout_db8_slot = var_fn61_calc_iq__qsout_db8;
        *var_fn61_calc_iq__qsout_db9_slot = var_fn61_calc_iq__qsout_db9;
        *var_fn61_calc_iq__qsout_dn0_slot = var_fn61_calc_iq__qsout_dn0;
        *var_fn61_calc_iq__qsout_dn1_slot = var_fn61_calc_iq__qsout_dn1;
        *var_fn61_calc_iq__qsout_dn10_slot = var_fn61_calc_iq__qsout_dn10;
        *var_fn61_calc_iq__qsout_dn11_slot = var_fn61_calc_iq__qsout_dn11;
        *var_fn61_calc_iq__qsout_dn12_slot = var_fn61_calc_iq__qsout_dn12;
        *var_fn61_calc_iq__qsout_dn13_slot = var_fn61_calc_iq__qsout_dn13;
        *var_fn61_calc_iq__qsout_dn14_slot = var_fn61_calc_iq__qsout_dn14;
        *var_fn61_calc_iq__qsout_dn15_slot = var_fn61_calc_iq__qsout_dn15;
        *var_fn61_calc_iq__qsout_dn16_slot = var_fn61_calc_iq__qsout_dn16;
        *var_fn61_calc_iq__qsout_dn17_slot = var_fn61_calc_iq__qsout_dn17;
        *var_fn61_calc_iq__qsout_dn18_slot = var_fn61_calc_iq__qsout_dn18;
        *var_fn61_calc_iq__qsout_dn19_slot = var_fn61_calc_iq__qsout_dn19;
        *var_fn61_calc_iq__qsout_dn2_slot = var_fn61_calc_iq__qsout_dn2;
        *var_fn61_calc_iq__qsout_dn20_slot = var_fn61_calc_iq__qsout_dn20;
        *var_fn61_calc_iq__qsout_dn21_slot = var_fn61_calc_iq__qsout_dn21;
        *var_fn61_calc_iq__qsout_dn22_slot = var_fn61_calc_iq__qsout_dn22;
        *var_fn61_calc_iq__qsout_dn23_slot = var_fn61_calc_iq__qsout_dn23;
        *var_fn61_calc_iq__qsout_dn24_slot = var_fn61_calc_iq__qsout_dn24;
        *var_fn61_calc_iq__qsout_dn25_slot = var_fn61_calc_iq__qsout_dn25;
        *var_fn61_calc_iq__qsout_dn26_slot = var_fn61_calc_iq__qsout_dn26;
        *var_fn61_calc_iq__qsout_dn27_slot = var_fn61_calc_iq__qsout_dn27;
        *var_fn61_calc_iq__qsout_dn28_slot = var_fn61_calc_iq__qsout_dn28;
        *var_fn61_calc_iq__qsout_dn29_slot = var_fn61_calc_iq__qsout_dn29;
        *var_fn61_calc_iq__qsout_dn3_slot = var_fn61_calc_iq__qsout_dn3;
        *var_fn61_calc_iq__qsout_dn4_slot = var_fn61_calc_iq__qsout_dn4;
        *var_fn61_calc_iq__qsout_dn5_slot = var_fn61_calc_iq__qsout_dn5;
        *var_fn61_calc_iq__qsout_dn6_slot = var_fn61_calc_iq__qsout_dn6;
        *var_fn61_calc_iq__qsout_dn7_slot = var_fn61_calc_iq__qsout_dn7;
        *var_fn61_calc_iq__qsout_dn8_slot = var_fn61_calc_iq__qsout_dn8;
        *var_fn61_calc_iq__qsout_dn9_slot = var_fn61_calc_iq__qsout_dn9;
        *var_fn61_calc_iq__return_slot = var_fn61_calc_iq__return;
        *var_fn61_calc_iq__return_db0_slot = var_fn61_calc_iq__return_db0;
        *var_fn61_calc_iq__return_db1_slot = var_fn61_calc_iq__return_db1;
        *var_fn61_calc_iq__return_db10_slot = var_fn61_calc_iq__return_db10;
        *var_fn61_calc_iq__return_db11_slot = var_fn61_calc_iq__return_db11;
        *var_fn61_calc_iq__return_db12_slot = var_fn61_calc_iq__return_db12;
        *var_fn61_calc_iq__return_db13_slot = var_fn61_calc_iq__return_db13;
        *var_fn61_calc_iq__return_db14_slot = var_fn61_calc_iq__return_db14;
        *var_fn61_calc_iq__return_db15_slot = var_fn61_calc_iq__return_db15;
        *var_fn61_calc_iq__return_db16_slot = var_fn61_calc_iq__return_db16;
        *var_fn61_calc_iq__return_db17_slot = var_fn61_calc_iq__return_db17;
        *var_fn61_calc_iq__return_db18_slot = var_fn61_calc_iq__return_db18;
        *var_fn61_calc_iq__return_db19_slot = var_fn61_calc_iq__return_db19;
        *var_fn61_calc_iq__return_db2_slot = var_fn61_calc_iq__return_db2;
        *var_fn61_calc_iq__return_db20_slot = var_fn61_calc_iq__return_db20;
        *var_fn61_calc_iq__return_db21_slot = var_fn61_calc_iq__return_db21;
        *var_fn61_calc_iq__return_db22_slot = var_fn61_calc_iq__return_db22;
        *var_fn61_calc_iq__return_db23_slot = var_fn61_calc_iq__return_db23;
        *var_fn61_calc_iq__return_db24_slot = var_fn61_calc_iq__return_db24;
        *var_fn61_calc_iq__return_db25_slot = var_fn61_calc_iq__return_db25;
        *var_fn61_calc_iq__return_db26_slot = var_fn61_calc_iq__return_db26;
        *var_fn61_calc_iq__return_db27_slot = var_fn61_calc_iq__return_db27;
        *var_fn61_calc_iq__return_db28_slot = var_fn61_calc_iq__return_db28;
        *var_fn61_calc_iq__return_db29_slot = var_fn61_calc_iq__return_db29;
        *var_fn61_calc_iq__return_db3_slot = var_fn61_calc_iq__return_db3;
        *var_fn61_calc_iq__return_db30_slot = var_fn61_calc_iq__return_db30;
        *var_fn61_calc_iq__return_db31_slot = var_fn61_calc_iq__return_db31;
        *var_fn61_calc_iq__return_db32_slot = var_fn61_calc_iq__return_db32;
        *var_fn61_calc_iq__return_db33_slot = var_fn61_calc_iq__return_db33;
        *var_fn61_calc_iq__return_db34_slot = var_fn61_calc_iq__return_db34;
        *var_fn61_calc_iq__return_db35_slot = var_fn61_calc_iq__return_db35;
        *var_fn61_calc_iq__return_db4_slot = var_fn61_calc_iq__return_db4;
        *var_fn61_calc_iq__return_db5_slot = var_fn61_calc_iq__return_db5;
        *var_fn61_calc_iq__return_db6_slot = var_fn61_calc_iq__return_db6;
        *var_fn61_calc_iq__return_db7_slot = var_fn61_calc_iq__return_db7;
        *var_fn61_calc_iq__return_db8_slot = var_fn61_calc_iq__return_db8;
        *var_fn61_calc_iq__return_db9_slot = var_fn61_calc_iq__return_db9;
        *var_fn61_calc_iq__return_dn0_slot = var_fn61_calc_iq__return_dn0;
        *var_fn61_calc_iq__return_dn1_slot = var_fn61_calc_iq__return_dn1;
        *var_fn61_calc_iq__return_dn10_slot = var_fn61_calc_iq__return_dn10;
        *var_fn61_calc_iq__return_dn11_slot = var_fn61_calc_iq__return_dn11;
        *var_fn61_calc_iq__return_dn12_slot = var_fn61_calc_iq__return_dn12;
        *var_fn61_calc_iq__return_dn13_slot = var_fn61_calc_iq__return_dn13;
        *var_fn61_calc_iq__return_dn14_slot = var_fn61_calc_iq__return_dn14;
        *var_fn61_calc_iq__return_dn15_slot = var_fn61_calc_iq__return_dn15;
        *var_fn61_calc_iq__return_dn16_slot = var_fn61_calc_iq__return_dn16;
        *var_fn61_calc_iq__return_dn17_slot = var_fn61_calc_iq__return_dn17;
        *var_fn61_calc_iq__return_dn18_slot = var_fn61_calc_iq__return_dn18;
        *var_fn61_calc_iq__return_dn19_slot = var_fn61_calc_iq__return_dn19;
        *var_fn61_calc_iq__return_dn2_slot = var_fn61_calc_iq__return_dn2;
        *var_fn61_calc_iq__return_dn20_slot = var_fn61_calc_iq__return_dn20;
        *var_fn61_calc_iq__return_dn21_slot = var_fn61_calc_iq__return_dn21;
        *var_fn61_calc_iq__return_dn22_slot = var_fn61_calc_iq__return_dn22;
        *var_fn61_calc_iq__return_dn23_slot = var_fn61_calc_iq__return_dn23;
        *var_fn61_calc_iq__return_dn24_slot = var_fn61_calc_iq__return_dn24;
        *var_fn61_calc_iq__return_dn25_slot = var_fn61_calc_iq__return_dn25;
        *var_fn61_calc_iq__return_dn26_slot = var_fn61_calc_iq__return_dn26;
        *var_fn61_calc_iq__return_dn27_slot = var_fn61_calc_iq__return_dn27;
        *var_fn61_calc_iq__return_dn28_slot = var_fn61_calc_iq__return_dn28;
        *var_fn61_calc_iq__return_dn29_slot = var_fn61_calc_iq__return_dn29;
        *var_fn61_calc_iq__return_dn3_slot = var_fn61_calc_iq__return_dn3;
        *var_fn61_calc_iq__return_dn4_slot = var_fn61_calc_iq__return_dn4;
        *var_fn61_calc_iq__return_dn5_slot = var_fn61_calc_iq__return_dn5;
        *var_fn61_calc_iq__return_dn6_slot = var_fn61_calc_iq__return_dn6;
        *var_fn61_calc_iq__return_dn7_slot = var_fn61_calc_iq__return_dn7;
        *var_fn61_calc_iq__return_dn8_slot = var_fn61_calc_iq__return_dn8;
        *var_fn61_calc_iq__return_dn9_slot = var_fn61_calc_iq__return_dn9;
        *var_guard94_slot = var_guard94;
    }

    pub(super) fn stamp_transient_block_82(
        var_fn61_calc_iq__idsout: f64,
        var_fn61_calc_iq__idsout_db0: f64,
        var_fn61_calc_iq__idsout_db1: f64,
        var_fn61_calc_iq__idsout_db10: f64,
        var_fn61_calc_iq__idsout_db11: f64,
        var_fn61_calc_iq__idsout_db12: f64,
        var_fn61_calc_iq__idsout_db13: f64,
        var_fn61_calc_iq__idsout_db14: f64,
        var_fn61_calc_iq__idsout_db15: f64,
        var_fn61_calc_iq__idsout_db16: f64,
        var_fn61_calc_iq__idsout_db17: f64,
        var_fn61_calc_iq__idsout_db18: f64,
        var_fn61_calc_iq__idsout_db19: f64,
        var_fn61_calc_iq__idsout_db2: f64,
        var_fn61_calc_iq__idsout_db20: f64,
        var_fn61_calc_iq__idsout_db21: f64,
        var_fn61_calc_iq__idsout_db22: f64,
        var_fn61_calc_iq__idsout_db23: f64,
        var_fn61_calc_iq__idsout_db24: f64,
        var_fn61_calc_iq__idsout_db25: f64,
        var_fn61_calc_iq__idsout_db26: f64,
        var_fn61_calc_iq__idsout_db27: f64,
        var_fn61_calc_iq__idsout_db28: f64,
        var_fn61_calc_iq__idsout_db29: f64,
        var_fn61_calc_iq__idsout_db3: f64,
        var_fn61_calc_iq__idsout_db30: f64,
        var_fn61_calc_iq__idsout_db31: f64,
        var_fn61_calc_iq__idsout_db32: f64,
        var_fn61_calc_iq__idsout_db33: f64,
        var_fn61_calc_iq__idsout_db34: f64,
        var_fn61_calc_iq__idsout_db35: f64,
        var_fn61_calc_iq__idsout_db4: f64,
        var_fn61_calc_iq__idsout_db5: f64,
        var_fn61_calc_iq__idsout_db6: f64,
        var_fn61_calc_iq__idsout_db7: f64,
        var_fn61_calc_iq__idsout_db8: f64,
        var_fn61_calc_iq__idsout_db9: f64,
        var_fn61_calc_iq__idsout_dn0: f64,
        var_fn61_calc_iq__idsout_dn1: f64,
        var_fn61_calc_iq__idsout_dn10: f64,
        var_fn61_calc_iq__idsout_dn11: f64,
        var_fn61_calc_iq__idsout_dn12: f64,
        var_fn61_calc_iq__idsout_dn13: f64,
        var_fn61_calc_iq__idsout_dn14: f64,
        var_fn61_calc_iq__idsout_dn15: f64,
        var_fn61_calc_iq__idsout_dn16: f64,
        var_fn61_calc_iq__idsout_dn17: f64,
        var_fn61_calc_iq__idsout_dn18: f64,
        var_fn61_calc_iq__idsout_dn19: f64,
        var_fn61_calc_iq__idsout_dn2: f64,
        var_fn61_calc_iq__idsout_dn20: f64,
        var_fn61_calc_iq__idsout_dn21: f64,
        var_fn61_calc_iq__idsout_dn22: f64,
        var_fn61_calc_iq__idsout_dn23: f64,
        var_fn61_calc_iq__idsout_dn24: f64,
        var_fn61_calc_iq__idsout_dn25: f64,
        var_fn61_calc_iq__idsout_dn26: f64,
        var_fn61_calc_iq__idsout_dn27: f64,
        var_fn61_calc_iq__idsout_dn28: f64,
        var_fn61_calc_iq__idsout_dn29: f64,
        var_fn61_calc_iq__idsout_dn3: f64,
        var_fn61_calc_iq__idsout_dn4: f64,
        var_fn61_calc_iq__idsout_dn5: f64,
        var_fn61_calc_iq__idsout_dn6: f64,
        var_fn61_calc_iq__idsout_dn7: f64,
        var_fn61_calc_iq__idsout_dn8: f64,
        var_fn61_calc_iq__idsout_dn9: f64,
        var_fn61_calc_iq__qbout: f64,
        var_fn61_calc_iq__qbout_db0: f64,
        var_fn61_calc_iq__qbout_db1: f64,
        var_fn61_calc_iq__qbout_db10: f64,
        var_fn61_calc_iq__qbout_db11: f64,
        var_fn61_calc_iq__qbout_db12: f64,
        var_fn61_calc_iq__qbout_db13: f64,
        var_fn61_calc_iq__qbout_db14: f64,
        var_fn61_calc_iq__qbout_db15: f64,
        var_fn61_calc_iq__qbout_db16: f64,
        var_fn61_calc_iq__qbout_db17: f64,
        var_fn61_calc_iq__qbout_db18: f64,
        var_fn61_calc_iq__qbout_db19: f64,
        var_fn61_calc_iq__qbout_db2: f64,
        var_fn61_calc_iq__qbout_db20: f64,
        var_fn61_calc_iq__qbout_db21: f64,
        var_fn61_calc_iq__qbout_db22: f64,
        var_fn61_calc_iq__qbout_db23: f64,
        var_fn61_calc_iq__qbout_db24: f64,
        var_fn61_calc_iq__qbout_db25: f64,
        var_fn61_calc_iq__qbout_db26: f64,
        var_fn61_calc_iq__qbout_db27: f64,
        var_fn61_calc_iq__qbout_db28: f64,
        var_fn61_calc_iq__qbout_db29: f64,
        var_fn61_calc_iq__qbout_db3: f64,
        var_fn61_calc_iq__qbout_db30: f64,
        var_fn61_calc_iq__qbout_db31: f64,
        var_fn61_calc_iq__qbout_db32: f64,
        var_fn61_calc_iq__qbout_db33: f64,
        var_fn61_calc_iq__qbout_db34: f64,
        var_fn61_calc_iq__qbout_db35: f64,
        var_fn61_calc_iq__qbout_db4: f64,
        var_fn61_calc_iq__qbout_db5: f64,
        var_fn61_calc_iq__qbout_db6: f64,
        var_fn61_calc_iq__qbout_db7: f64,
        var_fn61_calc_iq__qbout_db8: f64,
        var_fn61_calc_iq__qbout_db9: f64,
        var_fn61_calc_iq__qbout_dn0: f64,
        var_fn61_calc_iq__qbout_dn1: f64,
        var_fn61_calc_iq__qbout_dn10: f64,
        var_fn61_calc_iq__qbout_dn11: f64,
        var_fn61_calc_iq__qbout_dn12: f64,
        var_fn61_calc_iq__qbout_dn13: f64,
        var_fn61_calc_iq__qbout_dn14: f64,
        var_fn61_calc_iq__qbout_dn15: f64,
        var_fn61_calc_iq__qbout_dn16: f64,
        var_fn61_calc_iq__qbout_dn17: f64,
        var_fn61_calc_iq__qbout_dn18: f64,
        var_fn61_calc_iq__qbout_dn19: f64,
        var_fn61_calc_iq__qbout_dn2: f64,
        var_fn61_calc_iq__qbout_dn20: f64,
        var_fn61_calc_iq__qbout_dn21: f64,
        var_fn61_calc_iq__qbout_dn22: f64,
        var_fn61_calc_iq__qbout_dn23: f64,
        var_fn61_calc_iq__qbout_dn24: f64,
        var_fn61_calc_iq__qbout_dn25: f64,
        var_fn61_calc_iq__qbout_dn26: f64,
        var_fn61_calc_iq__qbout_dn27: f64,
        var_fn61_calc_iq__qbout_dn28: f64,
        var_fn61_calc_iq__qbout_dn29: f64,
        var_fn61_calc_iq__qbout_dn3: f64,
        var_fn61_calc_iq__qbout_dn4: f64,
        var_fn61_calc_iq__qbout_dn5: f64,
        var_fn61_calc_iq__qbout_dn6: f64,
        var_fn61_calc_iq__qbout_dn7: f64,
        var_fn61_calc_iq__qbout_dn8: f64,
        var_fn61_calc_iq__qbout_dn9: f64,
        var_fn61_calc_iq__qcout: f64,
        var_fn61_calc_iq__qcout_db0: f64,
        var_fn61_calc_iq__qcout_db1: f64,
        var_fn61_calc_iq__qcout_db10: f64,
        var_fn61_calc_iq__qcout_db11: f64,
        var_fn61_calc_iq__qcout_db12: f64,
        var_fn61_calc_iq__qcout_db13: f64,
        var_fn61_calc_iq__qcout_db14: f64,
        var_fn61_calc_iq__qcout_db15: f64,
        var_fn61_calc_iq__qcout_db16: f64,
        var_fn61_calc_iq__qcout_db17: f64,
        var_fn61_calc_iq__qcout_db18: f64,
        var_fn61_calc_iq__qcout_db19: f64,
        var_fn61_calc_iq__qcout_db2: f64,
        var_fn61_calc_iq__qcout_db20: f64,
        var_fn61_calc_iq__qcout_db21: f64,
        var_fn61_calc_iq__qcout_db22: f64,
        var_fn61_calc_iq__qcout_db23: f64,
        var_fn61_calc_iq__qcout_db24: f64,
        var_fn61_calc_iq__qcout_db25: f64,
        var_fn61_calc_iq__qcout_db26: f64,
        var_fn61_calc_iq__qcout_db27: f64,
        var_fn61_calc_iq__qcout_db28: f64,
        var_fn61_calc_iq__qcout_db29: f64,
        var_fn61_calc_iq__qcout_db3: f64,
        var_fn61_calc_iq__qcout_db30: f64,
        var_fn61_calc_iq__qcout_db31: f64,
        var_fn61_calc_iq__qcout_db32: f64,
        var_fn61_calc_iq__qcout_db33: f64,
        var_fn61_calc_iq__qcout_db34: f64,
        var_fn61_calc_iq__qcout_db35: f64,
        var_fn61_calc_iq__qcout_db4: f64,
        var_fn61_calc_iq__qcout_db5: f64,
        var_fn61_calc_iq__qcout_db6: f64,
        var_fn61_calc_iq__qcout_db7: f64,
        var_fn61_calc_iq__qcout_db8: f64,
        var_fn61_calc_iq__qcout_db9: f64,
        var_fn61_calc_iq__qcout_dn0: f64,
        var_fn61_calc_iq__qcout_dn1: f64,
        var_fn61_calc_iq__qcout_dn10: f64,
        var_fn61_calc_iq__qcout_dn11: f64,
        var_fn61_calc_iq__qcout_dn12: f64,
        var_fn61_calc_iq__qcout_dn13: f64,
        var_fn61_calc_iq__qcout_dn14: f64,
        var_fn61_calc_iq__qcout_dn15: f64,
        var_fn61_calc_iq__qcout_dn16: f64,
        var_fn61_calc_iq__qcout_dn17: f64,
        var_fn61_calc_iq__qcout_dn18: f64,
        var_fn61_calc_iq__qcout_dn19: f64,
        var_fn61_calc_iq__qcout_dn2: f64,
        var_fn61_calc_iq__qcout_dn20: f64,
        var_fn61_calc_iq__qcout_dn21: f64,
        var_fn61_calc_iq__qcout_dn22: f64,
        var_fn61_calc_iq__qcout_dn23: f64,
        var_fn61_calc_iq__qcout_dn24: f64,
        var_fn61_calc_iq__qcout_dn25: f64,
        var_fn61_calc_iq__qcout_dn26: f64,
        var_fn61_calc_iq__qcout_dn27: f64,
        var_fn61_calc_iq__qcout_dn28: f64,
        var_fn61_calc_iq__qcout_dn29: f64,
        var_fn61_calc_iq__qcout_dn3: f64,
        var_fn61_calc_iq__qcout_dn4: f64,
        var_fn61_calc_iq__qcout_dn5: f64,
        var_fn61_calc_iq__qcout_dn6: f64,
        var_fn61_calc_iq__qcout_dn7: f64,
        var_fn61_calc_iq__qcout_dn8: f64,
        var_fn61_calc_iq__qcout_dn9: f64,
        var_fn61_calc_iq__qgdout: f64,
        var_fn61_calc_iq__qgdout_db0: f64,
        var_fn61_calc_iq__qgdout_db1: f64,
        var_fn61_calc_iq__qgdout_db10: f64,
        var_fn61_calc_iq__qgdout_db11: f64,
        var_fn61_calc_iq__qgdout_db12: f64,
        var_fn61_calc_iq__qgdout_db13: f64,
        var_fn61_calc_iq__qgdout_db14: f64,
        var_fn61_calc_iq__qgdout_db15: f64,
        var_fn61_calc_iq__qgdout_db16: f64,
        var_fn61_calc_iq__qgdout_db17: f64,
        var_fn61_calc_iq__qgdout_db18: f64,
        var_fn61_calc_iq__qgdout_db19: f64,
        var_fn61_calc_iq__qgdout_db2: f64,
        var_fn61_calc_iq__qgdout_db20: f64,
        var_fn61_calc_iq__qgdout_db21: f64,
        var_fn61_calc_iq__qgdout_db22: f64,
        var_fn61_calc_iq__qgdout_db23: f64,
        var_fn61_calc_iq__qgdout_db24: f64,
        var_fn61_calc_iq__qgdout_db25: f64,
        var_fn61_calc_iq__qgdout_db26: f64,
        var_fn61_calc_iq__qgdout_db27: f64,
        var_fn61_calc_iq__qgdout_db28: f64,
        var_fn61_calc_iq__qgdout_db29: f64,
        var_fn61_calc_iq__qgdout_db3: f64,
        var_fn61_calc_iq__qgdout_db30: f64,
        var_fn61_calc_iq__qgdout_db31: f64,
        var_fn61_calc_iq__qgdout_db32: f64,
        var_fn61_calc_iq__qgdout_db33: f64,
        var_fn61_calc_iq__qgdout_db34: f64,
        var_fn61_calc_iq__qgdout_db35: f64,
        var_fn61_calc_iq__qgdout_db4: f64,
        var_fn61_calc_iq__qgdout_db5: f64,
        var_fn61_calc_iq__qgdout_db6: f64,
        var_fn61_calc_iq__qgdout_db7: f64,
        var_fn61_calc_iq__qgdout_db8: f64,
        var_fn61_calc_iq__qgdout_db9: f64,
        var_fn61_calc_iq__qgdout_dn0: f64,
        var_fn61_calc_iq__qgdout_dn1: f64,
        var_fn61_calc_iq__qgdout_dn10: f64,
        var_fn61_calc_iq__qgdout_dn11: f64,
        var_fn61_calc_iq__qgdout_dn12: f64,
        var_fn61_calc_iq__qgdout_dn13: f64,
        var_fn61_calc_iq__qgdout_dn14: f64,
        var_fn61_calc_iq__qgdout_dn15: f64,
        var_fn61_calc_iq__qgdout_dn16: f64,
        var_fn61_calc_iq__qgdout_dn17: f64,
        var_fn61_calc_iq__qgdout_dn18: f64,
        var_fn61_calc_iq__qgdout_dn19: f64,
        var_fn61_calc_iq__qgdout_dn2: f64,
        var_fn61_calc_iq__qgdout_dn20: f64,
        var_fn61_calc_iq__qgdout_dn21: f64,
        var_fn61_calc_iq__qgdout_dn22: f64,
        var_fn61_calc_iq__qgdout_dn23: f64,
        var_fn61_calc_iq__qgdout_dn24: f64,
        var_fn61_calc_iq__qgdout_dn25: f64,
        var_fn61_calc_iq__qgdout_dn26: f64,
        var_fn61_calc_iq__qgdout_dn27: f64,
        var_fn61_calc_iq__qgdout_dn28: f64,
        var_fn61_calc_iq__qgdout_dn29: f64,
        var_fn61_calc_iq__qgdout_dn3: f64,
        var_fn61_calc_iq__qgdout_dn4: f64,
        var_fn61_calc_iq__qgdout_dn5: f64,
        var_fn61_calc_iq__qgdout_dn6: f64,
        var_fn61_calc_iq__qgdout_dn7: f64,
        var_fn61_calc_iq__qgdout_dn8: f64,
        var_fn61_calc_iq__qgdout_dn9: f64,
        var_fn61_calc_iq__qgsout: f64,
        var_fn61_calc_iq__qgsout_db0: f64,
        var_fn61_calc_iq__qgsout_db1: f64,
        var_fn61_calc_iq__qgsout_db10: f64,
        var_fn61_calc_iq__qgsout_db11: f64,
        var_fn61_calc_iq__qgsout_db12: f64,
        var_fn61_calc_iq__qgsout_db13: f64,
        var_fn61_calc_iq__qgsout_db14: f64,
        var_fn61_calc_iq__qgsout_db15: f64,
        var_fn61_calc_iq__qgsout_db16: f64,
        var_fn61_calc_iq__qgsout_db17: f64,
        var_fn61_calc_iq__qgsout_db18: f64,
        var_fn61_calc_iq__qgsout_db19: f64,
        var_fn61_calc_iq__qgsout_db2: f64,
        var_fn61_calc_iq__qgsout_db20: f64,
        var_fn61_calc_iq__qgsout_db21: f64,
        var_fn61_calc_iq__qgsout_db22: f64,
        var_fn61_calc_iq__qgsout_db23: f64,
        var_fn61_calc_iq__qgsout_db24: f64,
        var_fn61_calc_iq__qgsout_db25: f64,
        var_fn61_calc_iq__qgsout_db26: f64,
        var_fn61_calc_iq__qgsout_db27: f64,
        var_fn61_calc_iq__qgsout_db28: f64,
        var_fn61_calc_iq__qgsout_db29: f64,
        var_fn61_calc_iq__qgsout_db3: f64,
        var_fn61_calc_iq__qgsout_db30: f64,
        var_fn61_calc_iq__qgsout_db31: f64,
        var_fn61_calc_iq__qgsout_db32: f64,
        var_fn61_calc_iq__qgsout_db33: f64,
        var_fn61_calc_iq__qgsout_db34: f64,
        var_fn61_calc_iq__qgsout_db35: f64,
        var_fn61_calc_iq__qgsout_db4: f64,
        var_fn61_calc_iq__qgsout_db5: f64,
        var_fn61_calc_iq__qgsout_db6: f64,
        var_fn61_calc_iq__qgsout_db7: f64,
        var_fn61_calc_iq__qgsout_db8: f64,
        var_fn61_calc_iq__qgsout_db9: f64,
        var_fn61_calc_iq__qgsout_dn0: f64,
        var_fn61_calc_iq__qgsout_dn1: f64,
        var_fn61_calc_iq__qgsout_dn10: f64,
        var_fn61_calc_iq__qgsout_dn11: f64,
        var_fn61_calc_iq__qgsout_dn12: f64,
        var_fn61_calc_iq__qgsout_dn13: f64,
        var_fn61_calc_iq__qgsout_dn14: f64,
        var_fn61_calc_iq__qgsout_dn15: f64,
        var_fn61_calc_iq__qgsout_dn16: f64,
        var_fn61_calc_iq__qgsout_dn17: f64,
        var_fn61_calc_iq__qgsout_dn18: f64,
        var_fn61_calc_iq__qgsout_dn19: f64,
        var_fn61_calc_iq__qgsout_dn2: f64,
        var_fn61_calc_iq__qgsout_dn20: f64,
        var_fn61_calc_iq__qgsout_dn21: f64,
        var_fn61_calc_iq__qgsout_dn22: f64,
        var_fn61_calc_iq__qgsout_dn23: f64,
        var_fn61_calc_iq__qgsout_dn24: f64,
        var_fn61_calc_iq__qgsout_dn25: f64,
        var_fn61_calc_iq__qgsout_dn26: f64,
        var_fn61_calc_iq__qgsout_dn27: f64,
        var_fn61_calc_iq__qgsout_dn28: f64,
        var_fn61_calc_iq__qgsout_dn29: f64,
        var_fn61_calc_iq__qgsout_dn3: f64,
        var_fn61_calc_iq__qgsout_dn4: f64,
        var_fn61_calc_iq__qgsout_dn5: f64,
        var_fn61_calc_iq__qgsout_dn6: f64,
        var_fn61_calc_iq__qgsout_dn7: f64,
        var_fn61_calc_iq__qgsout_dn8: f64,
        var_fn61_calc_iq__qgsout_dn9: f64,
        var_fn61_calc_iq__qsout: f64,
        var_fn61_calc_iq__qsout_db0: f64,
        var_fn61_calc_iq__qsout_db1: f64,
        var_fn61_calc_iq__qsout_db10: f64,
        var_fn61_calc_iq__qsout_db11: f64,
        var_fn61_calc_iq__qsout_db12: f64,
        var_fn61_calc_iq__qsout_db13: f64,
        var_fn61_calc_iq__qsout_db14: f64,
        var_fn61_calc_iq__qsout_db15: f64,
        var_fn61_calc_iq__qsout_db16: f64,
        var_fn61_calc_iq__qsout_db17: f64,
        var_fn61_calc_iq__qsout_db18: f64,
        var_fn61_calc_iq__qsout_db19: f64,
        var_fn61_calc_iq__qsout_db2: f64,
        var_fn61_calc_iq__qsout_db20: f64,
        var_fn61_calc_iq__qsout_db21: f64,
        var_fn61_calc_iq__qsout_db22: f64,
        var_fn61_calc_iq__qsout_db23: f64,
        var_fn61_calc_iq__qsout_db24: f64,
        var_fn61_calc_iq__qsout_db25: f64,
        var_fn61_calc_iq__qsout_db26: f64,
        var_fn61_calc_iq__qsout_db27: f64,
        var_fn61_calc_iq__qsout_db28: f64,
        var_fn61_calc_iq__qsout_db29: f64,
        var_fn61_calc_iq__qsout_db3: f64,
        var_fn61_calc_iq__qsout_db30: f64,
        var_fn61_calc_iq__qsout_db31: f64,
        var_fn61_calc_iq__qsout_db32: f64,
        var_fn61_calc_iq__qsout_db33: f64,
        var_fn61_calc_iq__qsout_db34: f64,
        var_fn61_calc_iq__qsout_db35: f64,
        var_fn61_calc_iq__qsout_db4: f64,
        var_fn61_calc_iq__qsout_db5: f64,
        var_fn61_calc_iq__qsout_db6: f64,
        var_fn61_calc_iq__qsout_db7: f64,
        var_fn61_calc_iq__qsout_db8: f64,
        var_fn61_calc_iq__qsout_db9: f64,
        var_fn61_calc_iq__qsout_dn0: f64,
        var_fn61_calc_iq__qsout_dn1: f64,
        var_fn61_calc_iq__qsout_dn10: f64,
        var_fn61_calc_iq__qsout_dn11: f64,
        var_fn61_calc_iq__qsout_dn12: f64,
        var_fn61_calc_iq__qsout_dn13: f64,
        var_fn61_calc_iq__qsout_dn14: f64,
        var_fn61_calc_iq__qsout_dn15: f64,
        var_fn61_calc_iq__qsout_dn16: f64,
        var_fn61_calc_iq__qsout_dn17: f64,
        var_fn61_calc_iq__qsout_dn18: f64,
        var_fn61_calc_iq__qsout_dn19: f64,
        var_fn61_calc_iq__qsout_dn2: f64,
        var_fn61_calc_iq__qsout_dn20: f64,
        var_fn61_calc_iq__qsout_dn21: f64,
        var_fn61_calc_iq__qsout_dn22: f64,
        var_fn61_calc_iq__qsout_dn23: f64,
        var_fn61_calc_iq__qsout_dn24: f64,
        var_fn61_calc_iq__qsout_dn25: f64,
        var_fn61_calc_iq__qsout_dn26: f64,
        var_fn61_calc_iq__qsout_dn27: f64,
        var_fn61_calc_iq__qsout_dn28: f64,
        var_fn61_calc_iq__qsout_dn29: f64,
        var_fn61_calc_iq__qsout_dn3: f64,
        var_fn61_calc_iq__qsout_dn4: f64,
        var_fn61_calc_iq__qsout_dn5: f64,
        var_fn61_calc_iq__qsout_dn6: f64,
        var_fn61_calc_iq__qsout_dn7: f64,
        var_fn61_calc_iq__qsout_dn8: f64,
        var_fn61_calc_iq__qsout_dn9: f64,
        var_guard60: f64,
        var_idsfp3_slot: &mut f64,
        var_idsfp3_db0_slot: &mut f64,
        var_idsfp3_db1_slot: &mut f64,
        var_idsfp3_db10_slot: &mut f64,
        var_idsfp3_db11_slot: &mut f64,
        var_idsfp3_db12_slot: &mut f64,
        var_idsfp3_db13_slot: &mut f64,
        var_idsfp3_db14_slot: &mut f64,
        var_idsfp3_db15_slot: &mut f64,
        var_idsfp3_db16_slot: &mut f64,
        var_idsfp3_db17_slot: &mut f64,
        var_idsfp3_db18_slot: &mut f64,
        var_idsfp3_db19_slot: &mut f64,
        var_idsfp3_db2_slot: &mut f64,
        var_idsfp3_db20_slot: &mut f64,
        var_idsfp3_db21_slot: &mut f64,
        var_idsfp3_db22_slot: &mut f64,
        var_idsfp3_db23_slot: &mut f64,
        var_idsfp3_db24_slot: &mut f64,
        var_idsfp3_db25_slot: &mut f64,
        var_idsfp3_db26_slot: &mut f64,
        var_idsfp3_db27_slot: &mut f64,
        var_idsfp3_db28_slot: &mut f64,
        var_idsfp3_db29_slot: &mut f64,
        var_idsfp3_db3_slot: &mut f64,
        var_idsfp3_db30_slot: &mut f64,
        var_idsfp3_db31_slot: &mut f64,
        var_idsfp3_db32_slot: &mut f64,
        var_idsfp3_db33_slot: &mut f64,
        var_idsfp3_db34_slot: &mut f64,
        var_idsfp3_db35_slot: &mut f64,
        var_idsfp3_db4_slot: &mut f64,
        var_idsfp3_db5_slot: &mut f64,
        var_idsfp3_db6_slot: &mut f64,
        var_idsfp3_db7_slot: &mut f64,
        var_idsfp3_db8_slot: &mut f64,
        var_idsfp3_db9_slot: &mut f64,
        var_idsfp3_dn0_slot: &mut f64,
        var_idsfp3_dn1_slot: &mut f64,
        var_idsfp3_dn10_slot: &mut f64,
        var_idsfp3_dn11_slot: &mut f64,
        var_idsfp3_dn12_slot: &mut f64,
        var_idsfp3_dn13_slot: &mut f64,
        var_idsfp3_dn14_slot: &mut f64,
        var_idsfp3_dn15_slot: &mut f64,
        var_idsfp3_dn16_slot: &mut f64,
        var_idsfp3_dn17_slot: &mut f64,
        var_idsfp3_dn18_slot: &mut f64,
        var_idsfp3_dn19_slot: &mut f64,
        var_idsfp3_dn2_slot: &mut f64,
        var_idsfp3_dn20_slot: &mut f64,
        var_idsfp3_dn21_slot: &mut f64,
        var_idsfp3_dn22_slot: &mut f64,
        var_idsfp3_dn23_slot: &mut f64,
        var_idsfp3_dn24_slot: &mut f64,
        var_idsfp3_dn25_slot: &mut f64,
        var_idsfp3_dn26_slot: &mut f64,
        var_idsfp3_dn27_slot: &mut f64,
        var_idsfp3_dn28_slot: &mut f64,
        var_idsfp3_dn29_slot: &mut f64,
        var_idsfp3_dn3_slot: &mut f64,
        var_idsfp3_dn4_slot: &mut f64,
        var_idsfp3_dn5_slot: &mut f64,
        var_idsfp3_dn6_slot: &mut f64,
        var_idsfp3_dn7_slot: &mut f64,
        var_idsfp3_dn8_slot: &mut f64,
        var_idsfp3_dn9_slot: &mut f64,
        var_qbfp3_slot: &mut f64,
        var_qbfp3_db0_slot: &mut f64,
        var_qbfp3_db1_slot: &mut f64,
        var_qbfp3_db10_slot: &mut f64,
        var_qbfp3_db11_slot: &mut f64,
        var_qbfp3_db12_slot: &mut f64,
        var_qbfp3_db13_slot: &mut f64,
        var_qbfp3_db14_slot: &mut f64,
        var_qbfp3_db15_slot: &mut f64,
        var_qbfp3_db16_slot: &mut f64,
        var_qbfp3_db17_slot: &mut f64,
        var_qbfp3_db18_slot: &mut f64,
        var_qbfp3_db19_slot: &mut f64,
        var_qbfp3_db2_slot: &mut f64,
        var_qbfp3_db20_slot: &mut f64,
        var_qbfp3_db21_slot: &mut f64,
        var_qbfp3_db22_slot: &mut f64,
        var_qbfp3_db23_slot: &mut f64,
        var_qbfp3_db24_slot: &mut f64,
        var_qbfp3_db25_slot: &mut f64,
        var_qbfp3_db26_slot: &mut f64,
        var_qbfp3_db27_slot: &mut f64,
        var_qbfp3_db28_slot: &mut f64,
        var_qbfp3_db29_slot: &mut f64,
        var_qbfp3_db3_slot: &mut f64,
        var_qbfp3_db30_slot: &mut f64,
        var_qbfp3_db31_slot: &mut f64,
        var_qbfp3_db32_slot: &mut f64,
        var_qbfp3_db33_slot: &mut f64,
        var_qbfp3_db34_slot: &mut f64,
        var_qbfp3_db35_slot: &mut f64,
        var_qbfp3_db4_slot: &mut f64,
        var_qbfp3_db5_slot: &mut f64,
        var_qbfp3_db6_slot: &mut f64,
        var_qbfp3_db7_slot: &mut f64,
        var_qbfp3_db8_slot: &mut f64,
        var_qbfp3_db9_slot: &mut f64,
        var_qbfp3_dn0_slot: &mut f64,
        var_qbfp3_dn1_slot: &mut f64,
        var_qbfp3_dn10_slot: &mut f64,
        var_qbfp3_dn11_slot: &mut f64,
        var_qbfp3_dn12_slot: &mut f64,
        var_qbfp3_dn13_slot: &mut f64,
        var_qbfp3_dn14_slot: &mut f64,
        var_qbfp3_dn15_slot: &mut f64,
        var_qbfp3_dn16_slot: &mut f64,
        var_qbfp3_dn17_slot: &mut f64,
        var_qbfp3_dn18_slot: &mut f64,
        var_qbfp3_dn19_slot: &mut f64,
        var_qbfp3_dn2_slot: &mut f64,
        var_qbfp3_dn20_slot: &mut f64,
        var_qbfp3_dn21_slot: &mut f64,
        var_qbfp3_dn22_slot: &mut f64,
        var_qbfp3_dn23_slot: &mut f64,
        var_qbfp3_dn24_slot: &mut f64,
        var_qbfp3_dn25_slot: &mut f64,
        var_qbfp3_dn26_slot: &mut f64,
        var_qbfp3_dn27_slot: &mut f64,
        var_qbfp3_dn28_slot: &mut f64,
        var_qbfp3_dn29_slot: &mut f64,
        var_qbfp3_dn3_slot: &mut f64,
        var_qbfp3_dn4_slot: &mut f64,
        var_qbfp3_dn5_slot: &mut f64,
        var_qbfp3_dn6_slot: &mut f64,
        var_qbfp3_dn7_slot: &mut f64,
        var_qbfp3_dn8_slot: &mut f64,
        var_qbfp3_dn9_slot: &mut f64,
        var_qcfp3_slot: &mut f64,
        var_qcfp3_db0_slot: &mut f64,
        var_qcfp3_db1_slot: &mut f64,
        var_qcfp3_db10_slot: &mut f64,
        var_qcfp3_db11_slot: &mut f64,
        var_qcfp3_db12_slot: &mut f64,
        var_qcfp3_db13_slot: &mut f64,
        var_qcfp3_db14_slot: &mut f64,
        var_qcfp3_db15_slot: &mut f64,
        var_qcfp3_db16_slot: &mut f64,
        var_qcfp3_db17_slot: &mut f64,
        var_qcfp3_db18_slot: &mut f64,
        var_qcfp3_db19_slot: &mut f64,
        var_qcfp3_db2_slot: &mut f64,
        var_qcfp3_db20_slot: &mut f64,
        var_qcfp3_db21_slot: &mut f64,
        var_qcfp3_db22_slot: &mut f64,
        var_qcfp3_db23_slot: &mut f64,
        var_qcfp3_db24_slot: &mut f64,
        var_qcfp3_db25_slot: &mut f64,
        var_qcfp3_db26_slot: &mut f64,
        var_qcfp3_db27_slot: &mut f64,
        var_qcfp3_db28_slot: &mut f64,
        var_qcfp3_db29_slot: &mut f64,
        var_qcfp3_db3_slot: &mut f64,
        var_qcfp3_db30_slot: &mut f64,
        var_qcfp3_db31_slot: &mut f64,
        var_qcfp3_db32_slot: &mut f64,
        var_qcfp3_db33_slot: &mut f64,
        var_qcfp3_db34_slot: &mut f64,
        var_qcfp3_db35_slot: &mut f64,
        var_qcfp3_db4_slot: &mut f64,
        var_qcfp3_db5_slot: &mut f64,
        var_qcfp3_db6_slot: &mut f64,
        var_qcfp3_db7_slot: &mut f64,
        var_qcfp3_db8_slot: &mut f64,
        var_qcfp3_db9_slot: &mut f64,
        var_qcfp3_dn0_slot: &mut f64,
        var_qcfp3_dn1_slot: &mut f64,
        var_qcfp3_dn10_slot: &mut f64,
        var_qcfp3_dn11_slot: &mut f64,
        var_qcfp3_dn12_slot: &mut f64,
        var_qcfp3_dn13_slot: &mut f64,
        var_qcfp3_dn14_slot: &mut f64,
        var_qcfp3_dn15_slot: &mut f64,
        var_qcfp3_dn16_slot: &mut f64,
        var_qcfp3_dn17_slot: &mut f64,
        var_qcfp3_dn18_slot: &mut f64,
        var_qcfp3_dn19_slot: &mut f64,
        var_qcfp3_dn2_slot: &mut f64,
        var_qcfp3_dn20_slot: &mut f64,
        var_qcfp3_dn21_slot: &mut f64,
        var_qcfp3_dn22_slot: &mut f64,
        var_qcfp3_dn23_slot: &mut f64,
        var_qcfp3_dn24_slot: &mut f64,
        var_qcfp3_dn25_slot: &mut f64,
        var_qcfp3_dn26_slot: &mut f64,
        var_qcfp3_dn27_slot: &mut f64,
        var_qcfp3_dn28_slot: &mut f64,
        var_qcfp3_dn29_slot: &mut f64,
        var_qcfp3_dn3_slot: &mut f64,
        var_qcfp3_dn4_slot: &mut f64,
        var_qcfp3_dn5_slot: &mut f64,
        var_qcfp3_dn6_slot: &mut f64,
        var_qcfp3_dn7_slot: &mut f64,
        var_qcfp3_dn8_slot: &mut f64,
        var_qcfp3_dn9_slot: &mut f64,
        var_qgdfp3_slot: &mut f64,
        var_qgdfp3_db0_slot: &mut f64,
        var_qgdfp3_db1_slot: &mut f64,
        var_qgdfp3_db10_slot: &mut f64,
        var_qgdfp3_db11_slot: &mut f64,
        var_qgdfp3_db12_slot: &mut f64,
        var_qgdfp3_db13_slot: &mut f64,
        var_qgdfp3_db14_slot: &mut f64,
        var_qgdfp3_db15_slot: &mut f64,
        var_qgdfp3_db16_slot: &mut f64,
        var_qgdfp3_db17_slot: &mut f64,
        var_qgdfp3_db18_slot: &mut f64,
        var_qgdfp3_db19_slot: &mut f64,
        var_qgdfp3_db2_slot: &mut f64,
        var_qgdfp3_db20_slot: &mut f64,
        var_qgdfp3_db21_slot: &mut f64,
        var_qgdfp3_db22_slot: &mut f64,
        var_qgdfp3_db23_slot: &mut f64,
        var_qgdfp3_db24_slot: &mut f64,
        var_qgdfp3_db25_slot: &mut f64,
        var_qgdfp3_db26_slot: &mut f64,
        var_qgdfp3_db27_slot: &mut f64,
        var_qgdfp3_db28_slot: &mut f64,
        var_qgdfp3_db29_slot: &mut f64,
        var_qgdfp3_db3_slot: &mut f64,
        var_qgdfp3_db30_slot: &mut f64,
        var_qgdfp3_db31_slot: &mut f64,
        var_qgdfp3_db32_slot: &mut f64,
        var_qgdfp3_db33_slot: &mut f64,
        var_qgdfp3_db34_slot: &mut f64,
        var_qgdfp3_db35_slot: &mut f64,
        var_qgdfp3_db4_slot: &mut f64,
        var_qgdfp3_db5_slot: &mut f64,
        var_qgdfp3_db6_slot: &mut f64,
        var_qgdfp3_db7_slot: &mut f64,
        var_qgdfp3_db8_slot: &mut f64,
        var_qgdfp3_db9_slot: &mut f64,
        var_qgdfp3_dn0_slot: &mut f64,
        var_qgdfp3_dn1_slot: &mut f64,
        var_qgdfp3_dn10_slot: &mut f64,
        var_qgdfp3_dn11_slot: &mut f64,
        var_qgdfp3_dn12_slot: &mut f64,
        var_qgdfp3_dn13_slot: &mut f64,
        var_qgdfp3_dn14_slot: &mut f64,
        var_qgdfp3_dn15_slot: &mut f64,
        var_qgdfp3_dn16_slot: &mut f64,
        var_qgdfp3_dn17_slot: &mut f64,
        var_qgdfp3_dn18_slot: &mut f64,
        var_qgdfp3_dn19_slot: &mut f64,
        var_qgdfp3_dn2_slot: &mut f64,
        var_qgdfp3_dn20_slot: &mut f64,
        var_qgdfp3_dn21_slot: &mut f64,
        var_qgdfp3_dn22_slot: &mut f64,
        var_qgdfp3_dn23_slot: &mut f64,
        var_qgdfp3_dn24_slot: &mut f64,
        var_qgdfp3_dn25_slot: &mut f64,
        var_qgdfp3_dn26_slot: &mut f64,
        var_qgdfp3_dn27_slot: &mut f64,
        var_qgdfp3_dn28_slot: &mut f64,
        var_qgdfp3_dn29_slot: &mut f64,
        var_qgdfp3_dn3_slot: &mut f64,
        var_qgdfp3_dn4_slot: &mut f64,
        var_qgdfp3_dn5_slot: &mut f64,
        var_qgdfp3_dn6_slot: &mut f64,
        var_qgdfp3_dn7_slot: &mut f64,
        var_qgdfp3_dn8_slot: &mut f64,
        var_qgdfp3_dn9_slot: &mut f64,
        var_qgsfp3_slot: &mut f64,
        var_qgsfp3_db0_slot: &mut f64,
        var_qgsfp3_db1_slot: &mut f64,
        var_qgsfp3_db10_slot: &mut f64,
        var_qgsfp3_db11_slot: &mut f64,
        var_qgsfp3_db12_slot: &mut f64,
        var_qgsfp3_db13_slot: &mut f64,
        var_qgsfp3_db14_slot: &mut f64,
        var_qgsfp3_db15_slot: &mut f64,
        var_qgsfp3_db16_slot: &mut f64,
        var_qgsfp3_db17_slot: &mut f64,
        var_qgsfp3_db18_slot: &mut f64,
        var_qgsfp3_db19_slot: &mut f64,
        var_qgsfp3_db2_slot: &mut f64,
        var_qgsfp3_db20_slot: &mut f64,
        var_qgsfp3_db21_slot: &mut f64,
        var_qgsfp3_db22_slot: &mut f64,
        var_qgsfp3_db23_slot: &mut f64,
        var_qgsfp3_db24_slot: &mut f64,
        var_qgsfp3_db25_slot: &mut f64,
        var_qgsfp3_db26_slot: &mut f64,
        var_qgsfp3_db27_slot: &mut f64,
        var_qgsfp3_db28_slot: &mut f64,
        var_qgsfp3_db29_slot: &mut f64,
        var_qgsfp3_db3_slot: &mut f64,
        var_qgsfp3_db30_slot: &mut f64,
        var_qgsfp3_db31_slot: &mut f64,
        var_qgsfp3_db32_slot: &mut f64,
        var_qgsfp3_db33_slot: &mut f64,
        var_qgsfp3_db34_slot: &mut f64,
        var_qgsfp3_db35_slot: &mut f64,
        var_qgsfp3_db4_slot: &mut f64,
        var_qgsfp3_db5_slot: &mut f64,
        var_qgsfp3_db6_slot: &mut f64,
        var_qgsfp3_db7_slot: &mut f64,
        var_qgsfp3_db8_slot: &mut f64,
        var_qgsfp3_db9_slot: &mut f64,
        var_qgsfp3_dn0_slot: &mut f64,
        var_qgsfp3_dn1_slot: &mut f64,
        var_qgsfp3_dn10_slot: &mut f64,
        var_qgsfp3_dn11_slot: &mut f64,
        var_qgsfp3_dn12_slot: &mut f64,
        var_qgsfp3_dn13_slot: &mut f64,
        var_qgsfp3_dn14_slot: &mut f64,
        var_qgsfp3_dn15_slot: &mut f64,
        var_qgsfp3_dn16_slot: &mut f64,
        var_qgsfp3_dn17_slot: &mut f64,
        var_qgsfp3_dn18_slot: &mut f64,
        var_qgsfp3_dn19_slot: &mut f64,
        var_qgsfp3_dn2_slot: &mut f64,
        var_qgsfp3_dn20_slot: &mut f64,
        var_qgsfp3_dn21_slot: &mut f64,
        var_qgsfp3_dn22_slot: &mut f64,
        var_qgsfp3_dn23_slot: &mut f64,
        var_qgsfp3_dn24_slot: &mut f64,
        var_qgsfp3_dn25_slot: &mut f64,
        var_qgsfp3_dn26_slot: &mut f64,
        var_qgsfp3_dn27_slot: &mut f64,
        var_qgsfp3_dn28_slot: &mut f64,
        var_qgsfp3_dn29_slot: &mut f64,
        var_qgsfp3_dn3_slot: &mut f64,
        var_qgsfp3_dn4_slot: &mut f64,
        var_qgsfp3_dn5_slot: &mut f64,
        var_qgsfp3_dn6_slot: &mut f64,
        var_qgsfp3_dn7_slot: &mut f64,
        var_qgsfp3_dn8_slot: &mut f64,
        var_qgsfp3_dn9_slot: &mut f64,
        var_qsfp3_slot: &mut f64,
        var_qsfp3_db0_slot: &mut f64,
        var_qsfp3_db1_slot: &mut f64,
        var_qsfp3_db10_slot: &mut f64,
        var_qsfp3_db11_slot: &mut f64,
        var_qsfp3_db12_slot: &mut f64,
        var_qsfp3_db13_slot: &mut f64,
        var_qsfp3_db14_slot: &mut f64,
        var_qsfp3_db15_slot: &mut f64,
        var_qsfp3_db16_slot: &mut f64,
        var_qsfp3_db17_slot: &mut f64,
        var_qsfp3_db18_slot: &mut f64,
        var_qsfp3_db19_slot: &mut f64,
        var_qsfp3_db2_slot: &mut f64,
        var_qsfp3_db20_slot: &mut f64,
        var_qsfp3_db21_slot: &mut f64,
        var_qsfp3_db22_slot: &mut f64,
        var_qsfp3_db23_slot: &mut f64,
        var_qsfp3_db24_slot: &mut f64,
        var_qsfp3_db25_slot: &mut f64,
        var_qsfp3_db26_slot: &mut f64,
        var_qsfp3_db27_slot: &mut f64,
        var_qsfp3_db28_slot: &mut f64,
        var_qsfp3_db29_slot: &mut f64,
        var_qsfp3_db3_slot: &mut f64,
        var_qsfp3_db30_slot: &mut f64,
        var_qsfp3_db31_slot: &mut f64,
        var_qsfp3_db32_slot: &mut f64,
        var_qsfp3_db33_slot: &mut f64,
        var_qsfp3_db34_slot: &mut f64,
        var_qsfp3_db35_slot: &mut f64,
        var_qsfp3_db4_slot: &mut f64,
        var_qsfp3_db5_slot: &mut f64,
        var_qsfp3_db6_slot: &mut f64,
        var_qsfp3_db7_slot: &mut f64,
        var_qsfp3_db8_slot: &mut f64,
        var_qsfp3_db9_slot: &mut f64,
        var_qsfp3_dn0_slot: &mut f64,
        var_qsfp3_dn1_slot: &mut f64,
        var_qsfp3_dn10_slot: &mut f64,
        var_qsfp3_dn11_slot: &mut f64,
        var_qsfp3_dn12_slot: &mut f64,
        var_qsfp3_dn13_slot: &mut f64,
        var_qsfp3_dn14_slot: &mut f64,
        var_qsfp3_dn15_slot: &mut f64,
        var_qsfp3_dn16_slot: &mut f64,
        var_qsfp3_dn17_slot: &mut f64,
        var_qsfp3_dn18_slot: &mut f64,
        var_qsfp3_dn19_slot: &mut f64,
        var_qsfp3_dn2_slot: &mut f64,
        var_qsfp3_dn20_slot: &mut f64,
        var_qsfp3_dn21_slot: &mut f64,
        var_qsfp3_dn22_slot: &mut f64,
        var_qsfp3_dn23_slot: &mut f64,
        var_qsfp3_dn24_slot: &mut f64,
        var_qsfp3_dn25_slot: &mut f64,
        var_qsfp3_dn26_slot: &mut f64,
        var_qsfp3_dn27_slot: &mut f64,
        var_qsfp3_dn28_slot: &mut f64,
        var_qsfp3_dn29_slot: &mut f64,
        var_qsfp3_dn3_slot: &mut f64,
        var_qsfp3_dn4_slot: &mut f64,
        var_qsfp3_dn5_slot: &mut f64,
        var_qsfp3_dn6_slot: &mut f64,
        var_qsfp3_dn7_slot: &mut f64,
        var_qsfp3_dn8_slot: &mut f64,
        var_qsfp3_dn9_slot: &mut f64,
    ) {
        let mut var_idsfp3: f64 = *var_idsfp3_slot;
        let mut var_idsfp3_db0: f64 = *var_idsfp3_db0_slot;
        let mut var_idsfp3_db1: f64 = *var_idsfp3_db1_slot;
        let mut var_idsfp3_db10: f64 = *var_idsfp3_db10_slot;
        let mut var_idsfp3_db11: f64 = *var_idsfp3_db11_slot;
        let mut var_idsfp3_db12: f64 = *var_idsfp3_db12_slot;
        let mut var_idsfp3_db13: f64 = *var_idsfp3_db13_slot;
        let mut var_idsfp3_db14: f64 = *var_idsfp3_db14_slot;
        let mut var_idsfp3_db15: f64 = *var_idsfp3_db15_slot;
        let mut var_idsfp3_db16: f64 = *var_idsfp3_db16_slot;
        let mut var_idsfp3_db17: f64 = *var_idsfp3_db17_slot;
        let mut var_idsfp3_db18: f64 = *var_idsfp3_db18_slot;
        let mut var_idsfp3_db19: f64 = *var_idsfp3_db19_slot;
        let mut var_idsfp3_db2: f64 = *var_idsfp3_db2_slot;
        let mut var_idsfp3_db20: f64 = *var_idsfp3_db20_slot;
        let mut var_idsfp3_db21: f64 = *var_idsfp3_db21_slot;
        let mut var_idsfp3_db22: f64 = *var_idsfp3_db22_slot;
        let mut var_idsfp3_db23: f64 = *var_idsfp3_db23_slot;
        let mut var_idsfp3_db24: f64 = *var_idsfp3_db24_slot;
        let mut var_idsfp3_db25: f64 = *var_idsfp3_db25_slot;
        let mut var_idsfp3_db26: f64 = *var_idsfp3_db26_slot;
        let mut var_idsfp3_db27: f64 = *var_idsfp3_db27_slot;
        let mut var_idsfp3_db28: f64 = *var_idsfp3_db28_slot;
        let mut var_idsfp3_db29: f64 = *var_idsfp3_db29_slot;
        let mut var_idsfp3_db3: f64 = *var_idsfp3_db3_slot;
        let mut var_idsfp3_db30: f64 = *var_idsfp3_db30_slot;
        let mut var_idsfp3_db31: f64 = *var_idsfp3_db31_slot;
        let mut var_idsfp3_db32: f64 = *var_idsfp3_db32_slot;
        let mut var_idsfp3_db33: f64 = *var_idsfp3_db33_slot;
        let mut var_idsfp3_db34: f64 = *var_idsfp3_db34_slot;
        let mut var_idsfp3_db35: f64 = *var_idsfp3_db35_slot;
        let mut var_idsfp3_db4: f64 = *var_idsfp3_db4_slot;
        let mut var_idsfp3_db5: f64 = *var_idsfp3_db5_slot;
        let mut var_idsfp3_db6: f64 = *var_idsfp3_db6_slot;
        let mut var_idsfp3_db7: f64 = *var_idsfp3_db7_slot;
        let mut var_idsfp3_db8: f64 = *var_idsfp3_db8_slot;
        let mut var_idsfp3_db9: f64 = *var_idsfp3_db9_slot;
        let mut var_idsfp3_dn0: f64 = *var_idsfp3_dn0_slot;
        let mut var_idsfp3_dn1: f64 = *var_idsfp3_dn1_slot;
        let mut var_idsfp3_dn10: f64 = *var_idsfp3_dn10_slot;
        let mut var_idsfp3_dn11: f64 = *var_idsfp3_dn11_slot;
        let mut var_idsfp3_dn12: f64 = *var_idsfp3_dn12_slot;
        let mut var_idsfp3_dn13: f64 = *var_idsfp3_dn13_slot;
        let mut var_idsfp3_dn14: f64 = *var_idsfp3_dn14_slot;
        let mut var_idsfp3_dn15: f64 = *var_idsfp3_dn15_slot;
        let mut var_idsfp3_dn16: f64 = *var_idsfp3_dn16_slot;
        let mut var_idsfp3_dn17: f64 = *var_idsfp3_dn17_slot;
        let mut var_idsfp3_dn18: f64 = *var_idsfp3_dn18_slot;
        let mut var_idsfp3_dn19: f64 = *var_idsfp3_dn19_slot;
        let mut var_idsfp3_dn2: f64 = *var_idsfp3_dn2_slot;
        let mut var_idsfp3_dn20: f64 = *var_idsfp3_dn20_slot;
        let mut var_idsfp3_dn21: f64 = *var_idsfp3_dn21_slot;
        let mut var_idsfp3_dn22: f64 = *var_idsfp3_dn22_slot;
        let mut var_idsfp3_dn23: f64 = *var_idsfp3_dn23_slot;
        let mut var_idsfp3_dn24: f64 = *var_idsfp3_dn24_slot;
        let mut var_idsfp3_dn25: f64 = *var_idsfp3_dn25_slot;
        let mut var_idsfp3_dn26: f64 = *var_idsfp3_dn26_slot;
        let mut var_idsfp3_dn27: f64 = *var_idsfp3_dn27_slot;
        let mut var_idsfp3_dn28: f64 = *var_idsfp3_dn28_slot;
        let mut var_idsfp3_dn29: f64 = *var_idsfp3_dn29_slot;
        let mut var_idsfp3_dn3: f64 = *var_idsfp3_dn3_slot;
        let mut var_idsfp3_dn4: f64 = *var_idsfp3_dn4_slot;
        let mut var_idsfp3_dn5: f64 = *var_idsfp3_dn5_slot;
        let mut var_idsfp3_dn6: f64 = *var_idsfp3_dn6_slot;
        let mut var_idsfp3_dn7: f64 = *var_idsfp3_dn7_slot;
        let mut var_idsfp3_dn8: f64 = *var_idsfp3_dn8_slot;
        let mut var_idsfp3_dn9: f64 = *var_idsfp3_dn9_slot;
        let mut var_qbfp3: f64 = *var_qbfp3_slot;
        let mut var_qbfp3_db0: f64 = *var_qbfp3_db0_slot;
        let mut var_qbfp3_db1: f64 = *var_qbfp3_db1_slot;
        let mut var_qbfp3_db10: f64 = *var_qbfp3_db10_slot;
        let mut var_qbfp3_db11: f64 = *var_qbfp3_db11_slot;
        let mut var_qbfp3_db12: f64 = *var_qbfp3_db12_slot;
        let mut var_qbfp3_db13: f64 = *var_qbfp3_db13_slot;
        let mut var_qbfp3_db14: f64 = *var_qbfp3_db14_slot;
        let mut var_qbfp3_db15: f64 = *var_qbfp3_db15_slot;
        let mut var_qbfp3_db16: f64 = *var_qbfp3_db16_slot;
        let mut var_qbfp3_db17: f64 = *var_qbfp3_db17_slot;
        let mut var_qbfp3_db18: f64 = *var_qbfp3_db18_slot;
        let mut var_qbfp3_db19: f64 = *var_qbfp3_db19_slot;
        let mut var_qbfp3_db2: f64 = *var_qbfp3_db2_slot;
        let mut var_qbfp3_db20: f64 = *var_qbfp3_db20_slot;
        let mut var_qbfp3_db21: f64 = *var_qbfp3_db21_slot;
        let mut var_qbfp3_db22: f64 = *var_qbfp3_db22_slot;
        let mut var_qbfp3_db23: f64 = *var_qbfp3_db23_slot;
        let mut var_qbfp3_db24: f64 = *var_qbfp3_db24_slot;
        let mut var_qbfp3_db25: f64 = *var_qbfp3_db25_slot;
        let mut var_qbfp3_db26: f64 = *var_qbfp3_db26_slot;
        let mut var_qbfp3_db27: f64 = *var_qbfp3_db27_slot;
        let mut var_qbfp3_db28: f64 = *var_qbfp3_db28_slot;
        let mut var_qbfp3_db29: f64 = *var_qbfp3_db29_slot;
        let mut var_qbfp3_db3: f64 = *var_qbfp3_db3_slot;
        let mut var_qbfp3_db30: f64 = *var_qbfp3_db30_slot;
        let mut var_qbfp3_db31: f64 = *var_qbfp3_db31_slot;
        let mut var_qbfp3_db32: f64 = *var_qbfp3_db32_slot;
        let mut var_qbfp3_db33: f64 = *var_qbfp3_db33_slot;
        let mut var_qbfp3_db34: f64 = *var_qbfp3_db34_slot;
        let mut var_qbfp3_db35: f64 = *var_qbfp3_db35_slot;
        let mut var_qbfp3_db4: f64 = *var_qbfp3_db4_slot;
        let mut var_qbfp3_db5: f64 = *var_qbfp3_db5_slot;
        let mut var_qbfp3_db6: f64 = *var_qbfp3_db6_slot;
        let mut var_qbfp3_db7: f64 = *var_qbfp3_db7_slot;
        let mut var_qbfp3_db8: f64 = *var_qbfp3_db8_slot;
        let mut var_qbfp3_db9: f64 = *var_qbfp3_db9_slot;
        let mut var_qbfp3_dn0: f64 = *var_qbfp3_dn0_slot;
        let mut var_qbfp3_dn1: f64 = *var_qbfp3_dn1_slot;
        let mut var_qbfp3_dn10: f64 = *var_qbfp3_dn10_slot;
        let mut var_qbfp3_dn11: f64 = *var_qbfp3_dn11_slot;
        let mut var_qbfp3_dn12: f64 = *var_qbfp3_dn12_slot;
        let mut var_qbfp3_dn13: f64 = *var_qbfp3_dn13_slot;
        let mut var_qbfp3_dn14: f64 = *var_qbfp3_dn14_slot;
        let mut var_qbfp3_dn15: f64 = *var_qbfp3_dn15_slot;
        let mut var_qbfp3_dn16: f64 = *var_qbfp3_dn16_slot;
        let mut var_qbfp3_dn17: f64 = *var_qbfp3_dn17_slot;
        let mut var_qbfp3_dn18: f64 = *var_qbfp3_dn18_slot;
        let mut var_qbfp3_dn19: f64 = *var_qbfp3_dn19_slot;
        let mut var_qbfp3_dn2: f64 = *var_qbfp3_dn2_slot;
        let mut var_qbfp3_dn20: f64 = *var_qbfp3_dn20_slot;
        let mut var_qbfp3_dn21: f64 = *var_qbfp3_dn21_slot;
        let mut var_qbfp3_dn22: f64 = *var_qbfp3_dn22_slot;
        let mut var_qbfp3_dn23: f64 = *var_qbfp3_dn23_slot;
        let mut var_qbfp3_dn24: f64 = *var_qbfp3_dn24_slot;
        let mut var_qbfp3_dn25: f64 = *var_qbfp3_dn25_slot;
        let mut var_qbfp3_dn26: f64 = *var_qbfp3_dn26_slot;
        let mut var_qbfp3_dn27: f64 = *var_qbfp3_dn27_slot;
        let mut var_qbfp3_dn28: f64 = *var_qbfp3_dn28_slot;
        let mut var_qbfp3_dn29: f64 = *var_qbfp3_dn29_slot;
        let mut var_qbfp3_dn3: f64 = *var_qbfp3_dn3_slot;
        let mut var_qbfp3_dn4: f64 = *var_qbfp3_dn4_slot;
        let mut var_qbfp3_dn5: f64 = *var_qbfp3_dn5_slot;
        let mut var_qbfp3_dn6: f64 = *var_qbfp3_dn6_slot;
        let mut var_qbfp3_dn7: f64 = *var_qbfp3_dn7_slot;
        let mut var_qbfp3_dn8: f64 = *var_qbfp3_dn8_slot;
        let mut var_qbfp3_dn9: f64 = *var_qbfp3_dn9_slot;
        let mut var_qcfp3: f64 = *var_qcfp3_slot;
        let mut var_qcfp3_db0: f64 = *var_qcfp3_db0_slot;
        let mut var_qcfp3_db1: f64 = *var_qcfp3_db1_slot;
        let mut var_qcfp3_db10: f64 = *var_qcfp3_db10_slot;
        let mut var_qcfp3_db11: f64 = *var_qcfp3_db11_slot;
        let mut var_qcfp3_db12: f64 = *var_qcfp3_db12_slot;
        let mut var_qcfp3_db13: f64 = *var_qcfp3_db13_slot;
        let mut var_qcfp3_db14: f64 = *var_qcfp3_db14_slot;
        let mut var_qcfp3_db15: f64 = *var_qcfp3_db15_slot;
        let mut var_qcfp3_db16: f64 = *var_qcfp3_db16_slot;
        let mut var_qcfp3_db17: f64 = *var_qcfp3_db17_slot;
        let mut var_qcfp3_db18: f64 = *var_qcfp3_db18_slot;
        let mut var_qcfp3_db19: f64 = *var_qcfp3_db19_slot;
        let mut var_qcfp3_db2: f64 = *var_qcfp3_db2_slot;
        let mut var_qcfp3_db20: f64 = *var_qcfp3_db20_slot;
        let mut var_qcfp3_db21: f64 = *var_qcfp3_db21_slot;
        let mut var_qcfp3_db22: f64 = *var_qcfp3_db22_slot;
        let mut var_qcfp3_db23: f64 = *var_qcfp3_db23_slot;
        let mut var_qcfp3_db24: f64 = *var_qcfp3_db24_slot;
        let mut var_qcfp3_db25: f64 = *var_qcfp3_db25_slot;
        let mut var_qcfp3_db26: f64 = *var_qcfp3_db26_slot;
        let mut var_qcfp3_db27: f64 = *var_qcfp3_db27_slot;
        let mut var_qcfp3_db28: f64 = *var_qcfp3_db28_slot;
        let mut var_qcfp3_db29: f64 = *var_qcfp3_db29_slot;
        let mut var_qcfp3_db3: f64 = *var_qcfp3_db3_slot;
        let mut var_qcfp3_db30: f64 = *var_qcfp3_db30_slot;
        let mut var_qcfp3_db31: f64 = *var_qcfp3_db31_slot;
        let mut var_qcfp3_db32: f64 = *var_qcfp3_db32_slot;
        let mut var_qcfp3_db33: f64 = *var_qcfp3_db33_slot;
        let mut var_qcfp3_db34: f64 = *var_qcfp3_db34_slot;
        let mut var_qcfp3_db35: f64 = *var_qcfp3_db35_slot;
        let mut var_qcfp3_db4: f64 = *var_qcfp3_db4_slot;
        let mut var_qcfp3_db5: f64 = *var_qcfp3_db5_slot;
        let mut var_qcfp3_db6: f64 = *var_qcfp3_db6_slot;
        let mut var_qcfp3_db7: f64 = *var_qcfp3_db7_slot;
        let mut var_qcfp3_db8: f64 = *var_qcfp3_db8_slot;
        let mut var_qcfp3_db9: f64 = *var_qcfp3_db9_slot;
        let mut var_qcfp3_dn0: f64 = *var_qcfp3_dn0_slot;
        let mut var_qcfp3_dn1: f64 = *var_qcfp3_dn1_slot;
        let mut var_qcfp3_dn10: f64 = *var_qcfp3_dn10_slot;
        let mut var_qcfp3_dn11: f64 = *var_qcfp3_dn11_slot;
        let mut var_qcfp3_dn12: f64 = *var_qcfp3_dn12_slot;
        let mut var_qcfp3_dn13: f64 = *var_qcfp3_dn13_slot;
        let mut var_qcfp3_dn14: f64 = *var_qcfp3_dn14_slot;
        let mut var_qcfp3_dn15: f64 = *var_qcfp3_dn15_slot;
        let mut var_qcfp3_dn16: f64 = *var_qcfp3_dn16_slot;
        let mut var_qcfp3_dn17: f64 = *var_qcfp3_dn17_slot;
        let mut var_qcfp3_dn18: f64 = *var_qcfp3_dn18_slot;
        let mut var_qcfp3_dn19: f64 = *var_qcfp3_dn19_slot;
        let mut var_qcfp3_dn2: f64 = *var_qcfp3_dn2_slot;
        let mut var_qcfp3_dn20: f64 = *var_qcfp3_dn20_slot;
        let mut var_qcfp3_dn21: f64 = *var_qcfp3_dn21_slot;
        let mut var_qcfp3_dn22: f64 = *var_qcfp3_dn22_slot;
        let mut var_qcfp3_dn23: f64 = *var_qcfp3_dn23_slot;
        let mut var_qcfp3_dn24: f64 = *var_qcfp3_dn24_slot;
        let mut var_qcfp3_dn25: f64 = *var_qcfp3_dn25_slot;
        let mut var_qcfp3_dn26: f64 = *var_qcfp3_dn26_slot;
        let mut var_qcfp3_dn27: f64 = *var_qcfp3_dn27_slot;
        let mut var_qcfp3_dn28: f64 = *var_qcfp3_dn28_slot;
        let mut var_qcfp3_dn29: f64 = *var_qcfp3_dn29_slot;
        let mut var_qcfp3_dn3: f64 = *var_qcfp3_dn3_slot;
        let mut var_qcfp3_dn4: f64 = *var_qcfp3_dn4_slot;
        let mut var_qcfp3_dn5: f64 = *var_qcfp3_dn5_slot;
        let mut var_qcfp3_dn6: f64 = *var_qcfp3_dn6_slot;
        let mut var_qcfp3_dn7: f64 = *var_qcfp3_dn7_slot;
        let mut var_qcfp3_dn8: f64 = *var_qcfp3_dn8_slot;
        let mut var_qcfp3_dn9: f64 = *var_qcfp3_dn9_slot;
        let mut var_qgdfp3: f64 = *var_qgdfp3_slot;
        let mut var_qgdfp3_db0: f64 = *var_qgdfp3_db0_slot;
        let mut var_qgdfp3_db1: f64 = *var_qgdfp3_db1_slot;
        let mut var_qgdfp3_db10: f64 = *var_qgdfp3_db10_slot;
        let mut var_qgdfp3_db11: f64 = *var_qgdfp3_db11_slot;
        let mut var_qgdfp3_db12: f64 = *var_qgdfp3_db12_slot;
        let mut var_qgdfp3_db13: f64 = *var_qgdfp3_db13_slot;
        let mut var_qgdfp3_db14: f64 = *var_qgdfp3_db14_slot;
        let mut var_qgdfp3_db15: f64 = *var_qgdfp3_db15_slot;
        let mut var_qgdfp3_db16: f64 = *var_qgdfp3_db16_slot;
        let mut var_qgdfp3_db17: f64 = *var_qgdfp3_db17_slot;
        let mut var_qgdfp3_db18: f64 = *var_qgdfp3_db18_slot;
        let mut var_qgdfp3_db19: f64 = *var_qgdfp3_db19_slot;
        let mut var_qgdfp3_db2: f64 = *var_qgdfp3_db2_slot;
        let mut var_qgdfp3_db20: f64 = *var_qgdfp3_db20_slot;
        let mut var_qgdfp3_db21: f64 = *var_qgdfp3_db21_slot;
        let mut var_qgdfp3_db22: f64 = *var_qgdfp3_db22_slot;
        let mut var_qgdfp3_db23: f64 = *var_qgdfp3_db23_slot;
        let mut var_qgdfp3_db24: f64 = *var_qgdfp3_db24_slot;
        let mut var_qgdfp3_db25: f64 = *var_qgdfp3_db25_slot;
        let mut var_qgdfp3_db26: f64 = *var_qgdfp3_db26_slot;
        let mut var_qgdfp3_db27: f64 = *var_qgdfp3_db27_slot;
        let mut var_qgdfp3_db28: f64 = *var_qgdfp3_db28_slot;
        let mut var_qgdfp3_db29: f64 = *var_qgdfp3_db29_slot;
        let mut var_qgdfp3_db3: f64 = *var_qgdfp3_db3_slot;
        let mut var_qgdfp3_db30: f64 = *var_qgdfp3_db30_slot;
        let mut var_qgdfp3_db31: f64 = *var_qgdfp3_db31_slot;
        let mut var_qgdfp3_db32: f64 = *var_qgdfp3_db32_slot;
        let mut var_qgdfp3_db33: f64 = *var_qgdfp3_db33_slot;
        let mut var_qgdfp3_db34: f64 = *var_qgdfp3_db34_slot;
        let mut var_qgdfp3_db35: f64 = *var_qgdfp3_db35_slot;
        let mut var_qgdfp3_db4: f64 = *var_qgdfp3_db4_slot;
        let mut var_qgdfp3_db5: f64 = *var_qgdfp3_db5_slot;
        let mut var_qgdfp3_db6: f64 = *var_qgdfp3_db6_slot;
        let mut var_qgdfp3_db7: f64 = *var_qgdfp3_db7_slot;
        let mut var_qgdfp3_db8: f64 = *var_qgdfp3_db8_slot;
        let mut var_qgdfp3_db9: f64 = *var_qgdfp3_db9_slot;
        let mut var_qgdfp3_dn0: f64 = *var_qgdfp3_dn0_slot;
        let mut var_qgdfp3_dn1: f64 = *var_qgdfp3_dn1_slot;
        let mut var_qgdfp3_dn10: f64 = *var_qgdfp3_dn10_slot;
        let mut var_qgdfp3_dn11: f64 = *var_qgdfp3_dn11_slot;
        let mut var_qgdfp3_dn12: f64 = *var_qgdfp3_dn12_slot;
        let mut var_qgdfp3_dn13: f64 = *var_qgdfp3_dn13_slot;
        let mut var_qgdfp3_dn14: f64 = *var_qgdfp3_dn14_slot;
        let mut var_qgdfp3_dn15: f64 = *var_qgdfp3_dn15_slot;
        let mut var_qgdfp3_dn16: f64 = *var_qgdfp3_dn16_slot;
        let mut var_qgdfp3_dn17: f64 = *var_qgdfp3_dn17_slot;
        let mut var_qgdfp3_dn18: f64 = *var_qgdfp3_dn18_slot;
        let mut var_qgdfp3_dn19: f64 = *var_qgdfp3_dn19_slot;
        let mut var_qgdfp3_dn2: f64 = *var_qgdfp3_dn2_slot;
        let mut var_qgdfp3_dn20: f64 = *var_qgdfp3_dn20_slot;
        let mut var_qgdfp3_dn21: f64 = *var_qgdfp3_dn21_slot;
        let mut var_qgdfp3_dn22: f64 = *var_qgdfp3_dn22_slot;
        let mut var_qgdfp3_dn23: f64 = *var_qgdfp3_dn23_slot;
        let mut var_qgdfp3_dn24: f64 = *var_qgdfp3_dn24_slot;
        let mut var_qgdfp3_dn25: f64 = *var_qgdfp3_dn25_slot;
        let mut var_qgdfp3_dn26: f64 = *var_qgdfp3_dn26_slot;
        let mut var_qgdfp3_dn27: f64 = *var_qgdfp3_dn27_slot;
        let mut var_qgdfp3_dn28: f64 = *var_qgdfp3_dn28_slot;
        let mut var_qgdfp3_dn29: f64 = *var_qgdfp3_dn29_slot;
        let mut var_qgdfp3_dn3: f64 = *var_qgdfp3_dn3_slot;
        let mut var_qgdfp3_dn4: f64 = *var_qgdfp3_dn4_slot;
        let mut var_qgdfp3_dn5: f64 = *var_qgdfp3_dn5_slot;
        let mut var_qgdfp3_dn6: f64 = *var_qgdfp3_dn6_slot;
        let mut var_qgdfp3_dn7: f64 = *var_qgdfp3_dn7_slot;
        let mut var_qgdfp3_dn8: f64 = *var_qgdfp3_dn8_slot;
        let mut var_qgdfp3_dn9: f64 = *var_qgdfp3_dn9_slot;
        let mut var_qgsfp3: f64 = *var_qgsfp3_slot;
        let mut var_qgsfp3_db0: f64 = *var_qgsfp3_db0_slot;
        let mut var_qgsfp3_db1: f64 = *var_qgsfp3_db1_slot;
        let mut var_qgsfp3_db10: f64 = *var_qgsfp3_db10_slot;
        let mut var_qgsfp3_db11: f64 = *var_qgsfp3_db11_slot;
        let mut var_qgsfp3_db12: f64 = *var_qgsfp3_db12_slot;
        let mut var_qgsfp3_db13: f64 = *var_qgsfp3_db13_slot;
        let mut var_qgsfp3_db14: f64 = *var_qgsfp3_db14_slot;
        let mut var_qgsfp3_db15: f64 = *var_qgsfp3_db15_slot;
        let mut var_qgsfp3_db16: f64 = *var_qgsfp3_db16_slot;
        let mut var_qgsfp3_db17: f64 = *var_qgsfp3_db17_slot;
        let mut var_qgsfp3_db18: f64 = *var_qgsfp3_db18_slot;
        let mut var_qgsfp3_db19: f64 = *var_qgsfp3_db19_slot;
        let mut var_qgsfp3_db2: f64 = *var_qgsfp3_db2_slot;
        let mut var_qgsfp3_db20: f64 = *var_qgsfp3_db20_slot;
        let mut var_qgsfp3_db21: f64 = *var_qgsfp3_db21_slot;
        let mut var_qgsfp3_db22: f64 = *var_qgsfp3_db22_slot;
        let mut var_qgsfp3_db23: f64 = *var_qgsfp3_db23_slot;
        let mut var_qgsfp3_db24: f64 = *var_qgsfp3_db24_slot;
        let mut var_qgsfp3_db25: f64 = *var_qgsfp3_db25_slot;
        let mut var_qgsfp3_db26: f64 = *var_qgsfp3_db26_slot;
        let mut var_qgsfp3_db27: f64 = *var_qgsfp3_db27_slot;
        let mut var_qgsfp3_db28: f64 = *var_qgsfp3_db28_slot;
        let mut var_qgsfp3_db29: f64 = *var_qgsfp3_db29_slot;
        let mut var_qgsfp3_db3: f64 = *var_qgsfp3_db3_slot;
        let mut var_qgsfp3_db30: f64 = *var_qgsfp3_db30_slot;
        let mut var_qgsfp3_db31: f64 = *var_qgsfp3_db31_slot;
        let mut var_qgsfp3_db32: f64 = *var_qgsfp3_db32_slot;
        let mut var_qgsfp3_db33: f64 = *var_qgsfp3_db33_slot;
        let mut var_qgsfp3_db34: f64 = *var_qgsfp3_db34_slot;
        let mut var_qgsfp3_db35: f64 = *var_qgsfp3_db35_slot;
        let mut var_qgsfp3_db4: f64 = *var_qgsfp3_db4_slot;
        let mut var_qgsfp3_db5: f64 = *var_qgsfp3_db5_slot;
        let mut var_qgsfp3_db6: f64 = *var_qgsfp3_db6_slot;
        let mut var_qgsfp3_db7: f64 = *var_qgsfp3_db7_slot;
        let mut var_qgsfp3_db8: f64 = *var_qgsfp3_db8_slot;
        let mut var_qgsfp3_db9: f64 = *var_qgsfp3_db9_slot;
        let mut var_qgsfp3_dn0: f64 = *var_qgsfp3_dn0_slot;
        let mut var_qgsfp3_dn1: f64 = *var_qgsfp3_dn1_slot;
        let mut var_qgsfp3_dn10: f64 = *var_qgsfp3_dn10_slot;
        let mut var_qgsfp3_dn11: f64 = *var_qgsfp3_dn11_slot;
        let mut var_qgsfp3_dn12: f64 = *var_qgsfp3_dn12_slot;
        let mut var_qgsfp3_dn13: f64 = *var_qgsfp3_dn13_slot;
        let mut var_qgsfp3_dn14: f64 = *var_qgsfp3_dn14_slot;
        let mut var_qgsfp3_dn15: f64 = *var_qgsfp3_dn15_slot;
        let mut var_qgsfp3_dn16: f64 = *var_qgsfp3_dn16_slot;
        let mut var_qgsfp3_dn17: f64 = *var_qgsfp3_dn17_slot;
        let mut var_qgsfp3_dn18: f64 = *var_qgsfp3_dn18_slot;
        let mut var_qgsfp3_dn19: f64 = *var_qgsfp3_dn19_slot;
        let mut var_qgsfp3_dn2: f64 = *var_qgsfp3_dn2_slot;
        let mut var_qgsfp3_dn20: f64 = *var_qgsfp3_dn20_slot;
        let mut var_qgsfp3_dn21: f64 = *var_qgsfp3_dn21_slot;
        let mut var_qgsfp3_dn22: f64 = *var_qgsfp3_dn22_slot;
        let mut var_qgsfp3_dn23: f64 = *var_qgsfp3_dn23_slot;
        let mut var_qgsfp3_dn24: f64 = *var_qgsfp3_dn24_slot;
        let mut var_qgsfp3_dn25: f64 = *var_qgsfp3_dn25_slot;
        let mut var_qgsfp3_dn26: f64 = *var_qgsfp3_dn26_slot;
        let mut var_qgsfp3_dn27: f64 = *var_qgsfp3_dn27_slot;
        let mut var_qgsfp3_dn28: f64 = *var_qgsfp3_dn28_slot;
        let mut var_qgsfp3_dn29: f64 = *var_qgsfp3_dn29_slot;
        let mut var_qgsfp3_dn3: f64 = *var_qgsfp3_dn3_slot;
        let mut var_qgsfp3_dn4: f64 = *var_qgsfp3_dn4_slot;
        let mut var_qgsfp3_dn5: f64 = *var_qgsfp3_dn5_slot;
        let mut var_qgsfp3_dn6: f64 = *var_qgsfp3_dn6_slot;
        let mut var_qgsfp3_dn7: f64 = *var_qgsfp3_dn7_slot;
        let mut var_qgsfp3_dn8: f64 = *var_qgsfp3_dn8_slot;
        let mut var_qgsfp3_dn9: f64 = *var_qgsfp3_dn9_slot;
        let mut var_qsfp3: f64 = *var_qsfp3_slot;
        let mut var_qsfp3_db0: f64 = *var_qsfp3_db0_slot;
        let mut var_qsfp3_db1: f64 = *var_qsfp3_db1_slot;
        let mut var_qsfp3_db10: f64 = *var_qsfp3_db10_slot;
        let mut var_qsfp3_db11: f64 = *var_qsfp3_db11_slot;
        let mut var_qsfp3_db12: f64 = *var_qsfp3_db12_slot;
        let mut var_qsfp3_db13: f64 = *var_qsfp3_db13_slot;
        let mut var_qsfp3_db14: f64 = *var_qsfp3_db14_slot;
        let mut var_qsfp3_db15: f64 = *var_qsfp3_db15_slot;
        let mut var_qsfp3_db16: f64 = *var_qsfp3_db16_slot;
        let mut var_qsfp3_db17: f64 = *var_qsfp3_db17_slot;
        let mut var_qsfp3_db18: f64 = *var_qsfp3_db18_slot;
        let mut var_qsfp3_db19: f64 = *var_qsfp3_db19_slot;
        let mut var_qsfp3_db2: f64 = *var_qsfp3_db2_slot;
        let mut var_qsfp3_db20: f64 = *var_qsfp3_db20_slot;
        let mut var_qsfp3_db21: f64 = *var_qsfp3_db21_slot;
        let mut var_qsfp3_db22: f64 = *var_qsfp3_db22_slot;
        let mut var_qsfp3_db23: f64 = *var_qsfp3_db23_slot;
        let mut var_qsfp3_db24: f64 = *var_qsfp3_db24_slot;
        let mut var_qsfp3_db25: f64 = *var_qsfp3_db25_slot;
        let mut var_qsfp3_db26: f64 = *var_qsfp3_db26_slot;
        let mut var_qsfp3_db27: f64 = *var_qsfp3_db27_slot;
        let mut var_qsfp3_db28: f64 = *var_qsfp3_db28_slot;
        let mut var_qsfp3_db29: f64 = *var_qsfp3_db29_slot;
        let mut var_qsfp3_db3: f64 = *var_qsfp3_db3_slot;
        let mut var_qsfp3_db30: f64 = *var_qsfp3_db30_slot;
        let mut var_qsfp3_db31: f64 = *var_qsfp3_db31_slot;
        let mut var_qsfp3_db32: f64 = *var_qsfp3_db32_slot;
        let mut var_qsfp3_db33: f64 = *var_qsfp3_db33_slot;
        let mut var_qsfp3_db34: f64 = *var_qsfp3_db34_slot;
        let mut var_qsfp3_db35: f64 = *var_qsfp3_db35_slot;
        let mut var_qsfp3_db4: f64 = *var_qsfp3_db4_slot;
        let mut var_qsfp3_db5: f64 = *var_qsfp3_db5_slot;
        let mut var_qsfp3_db6: f64 = *var_qsfp3_db6_slot;
        let mut var_qsfp3_db7: f64 = *var_qsfp3_db7_slot;
        let mut var_qsfp3_db8: f64 = *var_qsfp3_db8_slot;
        let mut var_qsfp3_db9: f64 = *var_qsfp3_db9_slot;
        let mut var_qsfp3_dn0: f64 = *var_qsfp3_dn0_slot;
        let mut var_qsfp3_dn1: f64 = *var_qsfp3_dn1_slot;
        let mut var_qsfp3_dn10: f64 = *var_qsfp3_dn10_slot;
        let mut var_qsfp3_dn11: f64 = *var_qsfp3_dn11_slot;
        let mut var_qsfp3_dn12: f64 = *var_qsfp3_dn12_slot;
        let mut var_qsfp3_dn13: f64 = *var_qsfp3_dn13_slot;
        let mut var_qsfp3_dn14: f64 = *var_qsfp3_dn14_slot;
        let mut var_qsfp3_dn15: f64 = *var_qsfp3_dn15_slot;
        let mut var_qsfp3_dn16: f64 = *var_qsfp3_dn16_slot;
        let mut var_qsfp3_dn17: f64 = *var_qsfp3_dn17_slot;
        let mut var_qsfp3_dn18: f64 = *var_qsfp3_dn18_slot;
        let mut var_qsfp3_dn19: f64 = *var_qsfp3_dn19_slot;
        let mut var_qsfp3_dn2: f64 = *var_qsfp3_dn2_slot;
        let mut var_qsfp3_dn20: f64 = *var_qsfp3_dn20_slot;
        let mut var_qsfp3_dn21: f64 = *var_qsfp3_dn21_slot;
        let mut var_qsfp3_dn22: f64 = *var_qsfp3_dn22_slot;
        let mut var_qsfp3_dn23: f64 = *var_qsfp3_dn23_slot;
        let mut var_qsfp3_dn24: f64 = *var_qsfp3_dn24_slot;
        let mut var_qsfp3_dn25: f64 = *var_qsfp3_dn25_slot;
        let mut var_qsfp3_dn26: f64 = *var_qsfp3_dn26_slot;
        let mut var_qsfp3_dn27: f64 = *var_qsfp3_dn27_slot;
        let mut var_qsfp3_dn28: f64 = *var_qsfp3_dn28_slot;
        let mut var_qsfp3_dn29: f64 = *var_qsfp3_dn29_slot;
        let mut var_qsfp3_dn3: f64 = *var_qsfp3_dn3_slot;
        let mut var_qsfp3_dn4: f64 = *var_qsfp3_dn4_slot;
        let mut var_qsfp3_dn5: f64 = *var_qsfp3_dn5_slot;
        let mut var_qsfp3_dn6: f64 = *var_qsfp3_dn6_slot;
        let mut var_qsfp3_dn7: f64 = *var_qsfp3_dn7_slot;
        let mut var_qsfp3_dn8: f64 = *var_qsfp3_dn8_slot;
        let mut var_qsfp3_dn9: f64 = *var_qsfp3_dn9_slot;

        let (assign7250_e8687, assign7250_e8687_d_n0, assign7250_e8687_d_n1, assign7250_e8687_d_n2, assign7250_e8687_d_n3, assign7250_e8687_d_n4, assign7250_e8687_d_n5, assign7250_e8687_d_n6, assign7250_e8687_d_n7, assign7250_e8687_d_n8, assign7250_e8687_d_n9, assign7250_e8687_d_n10, assign7250_e8687_d_n11, assign7250_e8687_d_n12, assign7250_e8687_d_n13, assign7250_e8687_d_n14, assign7250_e8687_d_n15, assign7250_e8687_d_n16, assign7250_e8687_d_n17, assign7250_e8687_d_n18, assign7250_e8687_d_n19, assign7250_e8687_d_n20, assign7250_e8687_d_n21, assign7250_e8687_d_n22, assign7250_e8687_d_n23, assign7250_e8687_d_n24, assign7250_e8687_d_n25, assign7250_e8687_d_n26, assign7250_e8687_d_n27, assign7250_e8687_d_n28, assign7250_e8687_d_n29, assign7250_e8687_d_b0, assign7250_e8687_d_b1, assign7250_e8687_d_b2, assign7250_e8687_d_b3, assign7250_e8687_d_b4, assign7250_e8687_d_b5, assign7250_e8687_d_b6, assign7250_e8687_d_b7, assign7250_e8687_d_b8, assign7250_e8687_d_b9, assign7250_e8687_d_b10, assign7250_e8687_d_b11, assign7250_e8687_d_b12, assign7250_e8687_d_b13, assign7250_e8687_d_b14, assign7250_e8687_d_b15, assign7250_e8687_d_b16, assign7250_e8687_d_b17, assign7250_e8687_d_b18, assign7250_e8687_d_b19, assign7250_e8687_d_b20, assign7250_e8687_d_b21, assign7250_e8687_d_b22, assign7250_e8687_d_b23, assign7250_e8687_d_b24, assign7250_e8687_d_b25, assign7250_e8687_d_b26, assign7250_e8687_d_b27, assign7250_e8687_d_b28, assign7250_e8687_d_b29, assign7250_e8687_d_b30, assign7250_e8687_d_b31, assign7250_e8687_d_b32, assign7250_e8687_d_b33, assign7250_e8687_d_b34, assign7250_e8687_d_b35,) = {
    if (var_guard60 != 0.0) {
        (var_fn61_calc_iq__idsout, var_fn61_calc_iq__idsout_dn0, var_fn61_calc_iq__idsout_dn1, var_fn61_calc_iq__idsout_dn2, var_fn61_calc_iq__idsout_dn3, var_fn61_calc_iq__idsout_dn4, var_fn61_calc_iq__idsout_dn5, var_fn61_calc_iq__idsout_dn6, var_fn61_calc_iq__idsout_dn7, var_fn61_calc_iq__idsout_dn8, var_fn61_calc_iq__idsout_dn9, var_fn61_calc_iq__idsout_dn10, var_fn61_calc_iq__idsout_dn11, var_fn61_calc_iq__idsout_dn12, var_fn61_calc_iq__idsout_dn13, var_fn61_calc_iq__idsout_dn14, var_fn61_calc_iq__idsout_dn15, var_fn61_calc_iq__idsout_dn16, var_fn61_calc_iq__idsout_dn17, var_fn61_calc_iq__idsout_dn18, var_fn61_calc_iq__idsout_dn19, var_fn61_calc_iq__idsout_dn20, var_fn61_calc_iq__idsout_dn21, var_fn61_calc_iq__idsout_dn22, var_fn61_calc_iq__idsout_dn23, var_fn61_calc_iq__idsout_dn24, var_fn61_calc_iq__idsout_dn25, var_fn61_calc_iq__idsout_dn26, var_fn61_calc_iq__idsout_dn27, var_fn61_calc_iq__idsout_dn28, var_fn61_calc_iq__idsout_dn29, var_fn61_calc_iq__idsout_db0, var_fn61_calc_iq__idsout_db1, var_fn61_calc_iq__idsout_db2, var_fn61_calc_iq__idsout_db3, var_fn61_calc_iq__idsout_db4, var_fn61_calc_iq__idsout_db5, var_fn61_calc_iq__idsout_db6, var_fn61_calc_iq__idsout_db7, var_fn61_calc_iq__idsout_db8, var_fn61_calc_iq__idsout_db9, var_fn61_calc_iq__idsout_db10, var_fn61_calc_iq__idsout_db11, var_fn61_calc_iq__idsout_db12, var_fn61_calc_iq__idsout_db13, var_fn61_calc_iq__idsout_db14, var_fn61_calc_iq__idsout_db15, var_fn61_calc_iq__idsout_db16, var_fn61_calc_iq__idsout_db17, var_fn61_calc_iq__idsout_db18, var_fn61_calc_iq__idsout_db19, var_fn61_calc_iq__idsout_db20, var_fn61_calc_iq__idsout_db21, var_fn61_calc_iq__idsout_db22, var_fn61_calc_iq__idsout_db23, var_fn61_calc_iq__idsout_db24, var_fn61_calc_iq__idsout_db25, var_fn61_calc_iq__idsout_db26, var_fn61_calc_iq__idsout_db27, var_fn61_calc_iq__idsout_db28, var_fn61_calc_iq__idsout_db29, var_fn61_calc_iq__idsout_db30, var_fn61_calc_iq__idsout_db31, var_fn61_calc_iq__idsout_db32, var_fn61_calc_iq__idsout_db33, var_fn61_calc_iq__idsout_db34, var_fn61_calc_iq__idsout_db35,)
    } else {
        (var_idsfp3, var_idsfp3_dn0, var_idsfp3_dn1, var_idsfp3_dn2, var_idsfp3_dn3, var_idsfp3_dn4, var_idsfp3_dn5, var_idsfp3_dn6, var_idsfp3_dn7, var_idsfp3_dn8, var_idsfp3_dn9, var_idsfp3_dn10, var_idsfp3_dn11, var_idsfp3_dn12, var_idsfp3_dn13, var_idsfp3_dn14, var_idsfp3_dn15, var_idsfp3_dn16, var_idsfp3_dn17, var_idsfp3_dn18, var_idsfp3_dn19, var_idsfp3_dn20, var_idsfp3_dn21, var_idsfp3_dn22, var_idsfp3_dn23, var_idsfp3_dn24, var_idsfp3_dn25, var_idsfp3_dn26, var_idsfp3_dn27, var_idsfp3_dn28, var_idsfp3_dn29, var_idsfp3_db0, var_idsfp3_db1, var_idsfp3_db2, var_idsfp3_db3, var_idsfp3_db4, var_idsfp3_db5, var_idsfp3_db6, var_idsfp3_db7, var_idsfp3_db8, var_idsfp3_db9, var_idsfp3_db10, var_idsfp3_db11, var_idsfp3_db12, var_idsfp3_db13, var_idsfp3_db14, var_idsfp3_db15, var_idsfp3_db16, var_idsfp3_db17, var_idsfp3_db18, var_idsfp3_db19, var_idsfp3_db20, var_idsfp3_db21, var_idsfp3_db22, var_idsfp3_db23, var_idsfp3_db24, var_idsfp3_db25, var_idsfp3_db26, var_idsfp3_db27, var_idsfp3_db28, var_idsfp3_db29, var_idsfp3_db30, var_idsfp3_db31, var_idsfp3_db32, var_idsfp3_db33, var_idsfp3_db34, var_idsfp3_db35,)
    }
};
        var_idsfp3 = assign7250_e8687;
        var_idsfp3_dn0 = assign7250_e8687_d_n0;
        var_idsfp3_dn1 = assign7250_e8687_d_n1;
        var_idsfp3_dn2 = assign7250_e8687_d_n2;
        var_idsfp3_dn3 = assign7250_e8687_d_n3;
        var_idsfp3_dn4 = assign7250_e8687_d_n4;
        var_idsfp3_dn5 = assign7250_e8687_d_n5;
        var_idsfp3_dn6 = assign7250_e8687_d_n6;
        var_idsfp3_dn7 = assign7250_e8687_d_n7;
        var_idsfp3_dn8 = assign7250_e8687_d_n8;
        var_idsfp3_dn9 = assign7250_e8687_d_n9;
        var_idsfp3_dn10 = assign7250_e8687_d_n10;
        var_idsfp3_dn11 = assign7250_e8687_d_n11;
        var_idsfp3_dn12 = assign7250_e8687_d_n12;
        var_idsfp3_dn13 = assign7250_e8687_d_n13;
        var_idsfp3_dn14 = assign7250_e8687_d_n14;
        var_idsfp3_dn15 = assign7250_e8687_d_n15;
        var_idsfp3_dn16 = assign7250_e8687_d_n16;
        var_idsfp3_dn17 = assign7250_e8687_d_n17;
        var_idsfp3_dn18 = assign7250_e8687_d_n18;
        var_idsfp3_dn19 = assign7250_e8687_d_n19;
        var_idsfp3_dn20 = assign7250_e8687_d_n20;
        var_idsfp3_dn21 = assign7250_e8687_d_n21;
        var_idsfp3_dn22 = assign7250_e8687_d_n22;
        var_idsfp3_dn23 = assign7250_e8687_d_n23;
        var_idsfp3_dn24 = assign7250_e8687_d_n24;
        var_idsfp3_dn25 = assign7250_e8687_d_n25;
        var_idsfp3_dn26 = assign7250_e8687_d_n26;
        var_idsfp3_dn27 = assign7250_e8687_d_n27;
        var_idsfp3_dn28 = assign7250_e8687_d_n28;
        var_idsfp3_dn29 = assign7250_e8687_d_n29;
        var_idsfp3_db0 = assign7250_e8687_d_b0;
        var_idsfp3_db1 = assign7250_e8687_d_b1;
        var_idsfp3_db2 = assign7250_e8687_d_b2;
        var_idsfp3_db3 = assign7250_e8687_d_b3;
        var_idsfp3_db4 = assign7250_e8687_d_b4;
        var_idsfp3_db5 = assign7250_e8687_d_b5;
        var_idsfp3_db6 = assign7250_e8687_d_b6;
        var_idsfp3_db7 = assign7250_e8687_d_b7;
        var_idsfp3_db8 = assign7250_e8687_d_b8;
        var_idsfp3_db9 = assign7250_e8687_d_b9;
        var_idsfp3_db10 = assign7250_e8687_d_b10;
        var_idsfp3_db11 = assign7250_e8687_d_b11;
        var_idsfp3_db12 = assign7250_e8687_d_b12;
        var_idsfp3_db13 = assign7250_e8687_d_b13;
        var_idsfp3_db14 = assign7250_e8687_d_b14;
        var_idsfp3_db15 = assign7250_e8687_d_b15;
        var_idsfp3_db16 = assign7250_e8687_d_b16;
        var_idsfp3_db17 = assign7250_e8687_d_b17;
        var_idsfp3_db18 = assign7250_e8687_d_b18;
        var_idsfp3_db19 = assign7250_e8687_d_b19;
        var_idsfp3_db20 = assign7250_e8687_d_b20;
        var_idsfp3_db21 = assign7250_e8687_d_b21;
        var_idsfp3_db22 = assign7250_e8687_d_b22;
        var_idsfp3_db23 = assign7250_e8687_d_b23;
        var_idsfp3_db24 = assign7250_e8687_d_b24;
        var_idsfp3_db25 = assign7250_e8687_d_b25;
        var_idsfp3_db26 = assign7250_e8687_d_b26;
        var_idsfp3_db27 = assign7250_e8687_d_b27;
        var_idsfp3_db28 = assign7250_e8687_d_b28;
        var_idsfp3_db29 = assign7250_e8687_d_b29;
        var_idsfp3_db30 = assign7250_e8687_d_b30;
        var_idsfp3_db31 = assign7250_e8687_d_b31;
        var_idsfp3_db32 = assign7250_e8687_d_b32;
        var_idsfp3_db33 = assign7250_e8687_d_b33;
        var_idsfp3_db34 = assign7250_e8687_d_b34;
        var_idsfp3_db35 = assign7250_e8687_d_b35;

        let (assign7260_e8691, assign7260_e8691_d_n0, assign7260_e8691_d_n1, assign7260_e8691_d_n2, assign7260_e8691_d_n3, assign7260_e8691_d_n4, assign7260_e8691_d_n5, assign7260_e8691_d_n6, assign7260_e8691_d_n7, assign7260_e8691_d_n8, assign7260_e8691_d_n9, assign7260_e8691_d_n10, assign7260_e8691_d_n11, assign7260_e8691_d_n12, assign7260_e8691_d_n13, assign7260_e8691_d_n14, assign7260_e8691_d_n15, assign7260_e8691_d_n16, assign7260_e8691_d_n17, assign7260_e8691_d_n18, assign7260_e8691_d_n19, assign7260_e8691_d_n20, assign7260_e8691_d_n21, assign7260_e8691_d_n22, assign7260_e8691_d_n23, assign7260_e8691_d_n24, assign7260_e8691_d_n25, assign7260_e8691_d_n26, assign7260_e8691_d_n27, assign7260_e8691_d_n28, assign7260_e8691_d_n29, assign7260_e8691_d_b0, assign7260_e8691_d_b1, assign7260_e8691_d_b2, assign7260_e8691_d_b3, assign7260_e8691_d_b4, assign7260_e8691_d_b5, assign7260_e8691_d_b6, assign7260_e8691_d_b7, assign7260_e8691_d_b8, assign7260_e8691_d_b9, assign7260_e8691_d_b10, assign7260_e8691_d_b11, assign7260_e8691_d_b12, assign7260_e8691_d_b13, assign7260_e8691_d_b14, assign7260_e8691_d_b15, assign7260_e8691_d_b16, assign7260_e8691_d_b17, assign7260_e8691_d_b18, assign7260_e8691_d_b19, assign7260_e8691_d_b20, assign7260_e8691_d_b21, assign7260_e8691_d_b22, assign7260_e8691_d_b23, assign7260_e8691_d_b24, assign7260_e8691_d_b25, assign7260_e8691_d_b26, assign7260_e8691_d_b27, assign7260_e8691_d_b28, assign7260_e8691_d_b29, assign7260_e8691_d_b30, assign7260_e8691_d_b31, assign7260_e8691_d_b32, assign7260_e8691_d_b33, assign7260_e8691_d_b34, assign7260_e8691_d_b35,) = {
    if (var_guard60 != 0.0) {
        (var_fn61_calc_iq__qgsout, var_fn61_calc_iq__qgsout_dn0, var_fn61_calc_iq__qgsout_dn1, var_fn61_calc_iq__qgsout_dn2, var_fn61_calc_iq__qgsout_dn3, var_fn61_calc_iq__qgsout_dn4, var_fn61_calc_iq__qgsout_dn5, var_fn61_calc_iq__qgsout_dn6, var_fn61_calc_iq__qgsout_dn7, var_fn61_calc_iq__qgsout_dn8, var_fn61_calc_iq__qgsout_dn9, var_fn61_calc_iq__qgsout_dn10, var_fn61_calc_iq__qgsout_dn11, var_fn61_calc_iq__qgsout_dn12, var_fn61_calc_iq__qgsout_dn13, var_fn61_calc_iq__qgsout_dn14, var_fn61_calc_iq__qgsout_dn15, var_fn61_calc_iq__qgsout_dn16, var_fn61_calc_iq__qgsout_dn17, var_fn61_calc_iq__qgsout_dn18, var_fn61_calc_iq__qgsout_dn19, var_fn61_calc_iq__qgsout_dn20, var_fn61_calc_iq__qgsout_dn21, var_fn61_calc_iq__qgsout_dn22, var_fn61_calc_iq__qgsout_dn23, var_fn61_calc_iq__qgsout_dn24, var_fn61_calc_iq__qgsout_dn25, var_fn61_calc_iq__qgsout_dn26, var_fn61_calc_iq__qgsout_dn27, var_fn61_calc_iq__qgsout_dn28, var_fn61_calc_iq__qgsout_dn29, var_fn61_calc_iq__qgsout_db0, var_fn61_calc_iq__qgsout_db1, var_fn61_calc_iq__qgsout_db2, var_fn61_calc_iq__qgsout_db3, var_fn61_calc_iq__qgsout_db4, var_fn61_calc_iq__qgsout_db5, var_fn61_calc_iq__qgsout_db6, var_fn61_calc_iq__qgsout_db7, var_fn61_calc_iq__qgsout_db8, var_fn61_calc_iq__qgsout_db9, var_fn61_calc_iq__qgsout_db10, var_fn61_calc_iq__qgsout_db11, var_fn61_calc_iq__qgsout_db12, var_fn61_calc_iq__qgsout_db13, var_fn61_calc_iq__qgsout_db14, var_fn61_calc_iq__qgsout_db15, var_fn61_calc_iq__qgsout_db16, var_fn61_calc_iq__qgsout_db17, var_fn61_calc_iq__qgsout_db18, var_fn61_calc_iq__qgsout_db19, var_fn61_calc_iq__qgsout_db20, var_fn61_calc_iq__qgsout_db21, var_fn61_calc_iq__qgsout_db22, var_fn61_calc_iq__qgsout_db23, var_fn61_calc_iq__qgsout_db24, var_fn61_calc_iq__qgsout_db25, var_fn61_calc_iq__qgsout_db26, var_fn61_calc_iq__qgsout_db27, var_fn61_calc_iq__qgsout_db28, var_fn61_calc_iq__qgsout_db29, var_fn61_calc_iq__qgsout_db30, var_fn61_calc_iq__qgsout_db31, var_fn61_calc_iq__qgsout_db32, var_fn61_calc_iq__qgsout_db33, var_fn61_calc_iq__qgsout_db34, var_fn61_calc_iq__qgsout_db35,)
    } else {
        (var_qgsfp3, var_qgsfp3_dn0, var_qgsfp3_dn1, var_qgsfp3_dn2, var_qgsfp3_dn3, var_qgsfp3_dn4, var_qgsfp3_dn5, var_qgsfp3_dn6, var_qgsfp3_dn7, var_qgsfp3_dn8, var_qgsfp3_dn9, var_qgsfp3_dn10, var_qgsfp3_dn11, var_qgsfp3_dn12, var_qgsfp3_dn13, var_qgsfp3_dn14, var_qgsfp3_dn15, var_qgsfp3_dn16, var_qgsfp3_dn17, var_qgsfp3_dn18, var_qgsfp3_dn19, var_qgsfp3_dn20, var_qgsfp3_dn21, var_qgsfp3_dn22, var_qgsfp3_dn23, var_qgsfp3_dn24, var_qgsfp3_dn25, var_qgsfp3_dn26, var_qgsfp3_dn27, var_qgsfp3_dn28, var_qgsfp3_dn29, var_qgsfp3_db0, var_qgsfp3_db1, var_qgsfp3_db2, var_qgsfp3_db3, var_qgsfp3_db4, var_qgsfp3_db5, var_qgsfp3_db6, var_qgsfp3_db7, var_qgsfp3_db8, var_qgsfp3_db9, var_qgsfp3_db10, var_qgsfp3_db11, var_qgsfp3_db12, var_qgsfp3_db13, var_qgsfp3_db14, var_qgsfp3_db15, var_qgsfp3_db16, var_qgsfp3_db17, var_qgsfp3_db18, var_qgsfp3_db19, var_qgsfp3_db20, var_qgsfp3_db21, var_qgsfp3_db22, var_qgsfp3_db23, var_qgsfp3_db24, var_qgsfp3_db25, var_qgsfp3_db26, var_qgsfp3_db27, var_qgsfp3_db28, var_qgsfp3_db29, var_qgsfp3_db30, var_qgsfp3_db31, var_qgsfp3_db32, var_qgsfp3_db33, var_qgsfp3_db34, var_qgsfp3_db35,)
    }
};
        var_qgsfp3 = assign7260_e8691;
        var_qgsfp3_dn0 = assign7260_e8691_d_n0;
        var_qgsfp3_dn1 = assign7260_e8691_d_n1;
        var_qgsfp3_dn2 = assign7260_e8691_d_n2;
        var_qgsfp3_dn3 = assign7260_e8691_d_n3;
        var_qgsfp3_dn4 = assign7260_e8691_d_n4;
        var_qgsfp3_dn5 = assign7260_e8691_d_n5;
        var_qgsfp3_dn6 = assign7260_e8691_d_n6;
        var_qgsfp3_dn7 = assign7260_e8691_d_n7;
        var_qgsfp3_dn8 = assign7260_e8691_d_n8;
        var_qgsfp3_dn9 = assign7260_e8691_d_n9;
        var_qgsfp3_dn10 = assign7260_e8691_d_n10;
        var_qgsfp3_dn11 = assign7260_e8691_d_n11;
        var_qgsfp3_dn12 = assign7260_e8691_d_n12;
        var_qgsfp3_dn13 = assign7260_e8691_d_n13;
        var_qgsfp3_dn14 = assign7260_e8691_d_n14;
        var_qgsfp3_dn15 = assign7260_e8691_d_n15;
        var_qgsfp3_dn16 = assign7260_e8691_d_n16;
        var_qgsfp3_dn17 = assign7260_e8691_d_n17;
        var_qgsfp3_dn18 = assign7260_e8691_d_n18;
        var_qgsfp3_dn19 = assign7260_e8691_d_n19;
        var_qgsfp3_dn20 = assign7260_e8691_d_n20;
        var_qgsfp3_dn21 = assign7260_e8691_d_n21;
        var_qgsfp3_dn22 = assign7260_e8691_d_n22;
        var_qgsfp3_dn23 = assign7260_e8691_d_n23;
        var_qgsfp3_dn24 = assign7260_e8691_d_n24;
        var_qgsfp3_dn25 = assign7260_e8691_d_n25;
        var_qgsfp3_dn26 = assign7260_e8691_d_n26;
        var_qgsfp3_dn27 = assign7260_e8691_d_n27;
        var_qgsfp3_dn28 = assign7260_e8691_d_n28;
        var_qgsfp3_dn29 = assign7260_e8691_d_n29;
        var_qgsfp3_db0 = assign7260_e8691_d_b0;
        var_qgsfp3_db1 = assign7260_e8691_d_b1;
        var_qgsfp3_db2 = assign7260_e8691_d_b2;
        var_qgsfp3_db3 = assign7260_e8691_d_b3;
        var_qgsfp3_db4 = assign7260_e8691_d_b4;
        var_qgsfp3_db5 = assign7260_e8691_d_b5;
        var_qgsfp3_db6 = assign7260_e8691_d_b6;
        var_qgsfp3_db7 = assign7260_e8691_d_b7;
        var_qgsfp3_db8 = assign7260_e8691_d_b8;
        var_qgsfp3_db9 = assign7260_e8691_d_b9;
        var_qgsfp3_db10 = assign7260_e8691_d_b10;
        var_qgsfp3_db11 = assign7260_e8691_d_b11;
        var_qgsfp3_db12 = assign7260_e8691_d_b12;
        var_qgsfp3_db13 = assign7260_e8691_d_b13;
        var_qgsfp3_db14 = assign7260_e8691_d_b14;
        var_qgsfp3_db15 = assign7260_e8691_d_b15;
        var_qgsfp3_db16 = assign7260_e8691_d_b16;
        var_qgsfp3_db17 = assign7260_e8691_d_b17;
        var_qgsfp3_db18 = assign7260_e8691_d_b18;
        var_qgsfp3_db19 = assign7260_e8691_d_b19;
        var_qgsfp3_db20 = assign7260_e8691_d_b20;
        var_qgsfp3_db21 = assign7260_e8691_d_b21;
        var_qgsfp3_db22 = assign7260_e8691_d_b22;
        var_qgsfp3_db23 = assign7260_e8691_d_b23;
        var_qgsfp3_db24 = assign7260_e8691_d_b24;
        var_qgsfp3_db25 = assign7260_e8691_d_b25;
        var_qgsfp3_db26 = assign7260_e8691_d_b26;
        var_qgsfp3_db27 = assign7260_e8691_d_b27;
        var_qgsfp3_db28 = assign7260_e8691_d_b28;
        var_qgsfp3_db29 = assign7260_e8691_d_b29;
        var_qgsfp3_db30 = assign7260_e8691_d_b30;
        var_qgsfp3_db31 = assign7260_e8691_d_b31;
        var_qgsfp3_db32 = assign7260_e8691_d_b32;
        var_qgsfp3_db33 = assign7260_e8691_d_b33;
        var_qgsfp3_db34 = assign7260_e8691_d_b34;
        var_qgsfp3_db35 = assign7260_e8691_d_b35;

        let (assign7270_e8695, assign7270_e8695_d_n0, assign7270_e8695_d_n1, assign7270_e8695_d_n2, assign7270_e8695_d_n3, assign7270_e8695_d_n4, assign7270_e8695_d_n5, assign7270_e8695_d_n6, assign7270_e8695_d_n7, assign7270_e8695_d_n8, assign7270_e8695_d_n9, assign7270_e8695_d_n10, assign7270_e8695_d_n11, assign7270_e8695_d_n12, assign7270_e8695_d_n13, assign7270_e8695_d_n14, assign7270_e8695_d_n15, assign7270_e8695_d_n16, assign7270_e8695_d_n17, assign7270_e8695_d_n18, assign7270_e8695_d_n19, assign7270_e8695_d_n20, assign7270_e8695_d_n21, assign7270_e8695_d_n22, assign7270_e8695_d_n23, assign7270_e8695_d_n24, assign7270_e8695_d_n25, assign7270_e8695_d_n26, assign7270_e8695_d_n27, assign7270_e8695_d_n28, assign7270_e8695_d_n29, assign7270_e8695_d_b0, assign7270_e8695_d_b1, assign7270_e8695_d_b2, assign7270_e8695_d_b3, assign7270_e8695_d_b4, assign7270_e8695_d_b5, assign7270_e8695_d_b6, assign7270_e8695_d_b7, assign7270_e8695_d_b8, assign7270_e8695_d_b9, assign7270_e8695_d_b10, assign7270_e8695_d_b11, assign7270_e8695_d_b12, assign7270_e8695_d_b13, assign7270_e8695_d_b14, assign7270_e8695_d_b15, assign7270_e8695_d_b16, assign7270_e8695_d_b17, assign7270_e8695_d_b18, assign7270_e8695_d_b19, assign7270_e8695_d_b20, assign7270_e8695_d_b21, assign7270_e8695_d_b22, assign7270_e8695_d_b23, assign7270_e8695_d_b24, assign7270_e8695_d_b25, assign7270_e8695_d_b26, assign7270_e8695_d_b27, assign7270_e8695_d_b28, assign7270_e8695_d_b29, assign7270_e8695_d_b30, assign7270_e8695_d_b31, assign7270_e8695_d_b32, assign7270_e8695_d_b33, assign7270_e8695_d_b34, assign7270_e8695_d_b35,) = {
    if (var_guard60 != 0.0) {
        (var_fn61_calc_iq__qgdout, var_fn61_calc_iq__qgdout_dn0, var_fn61_calc_iq__qgdout_dn1, var_fn61_calc_iq__qgdout_dn2, var_fn61_calc_iq__qgdout_dn3, var_fn61_calc_iq__qgdout_dn4, var_fn61_calc_iq__qgdout_dn5, var_fn61_calc_iq__qgdout_dn6, var_fn61_calc_iq__qgdout_dn7, var_fn61_calc_iq__qgdout_dn8, var_fn61_calc_iq__qgdout_dn9, var_fn61_calc_iq__qgdout_dn10, var_fn61_calc_iq__qgdout_dn11, var_fn61_calc_iq__qgdout_dn12, var_fn61_calc_iq__qgdout_dn13, var_fn61_calc_iq__qgdout_dn14, var_fn61_calc_iq__qgdout_dn15, var_fn61_calc_iq__qgdout_dn16, var_fn61_calc_iq__qgdout_dn17, var_fn61_calc_iq__qgdout_dn18, var_fn61_calc_iq__qgdout_dn19, var_fn61_calc_iq__qgdout_dn20, var_fn61_calc_iq__qgdout_dn21, var_fn61_calc_iq__qgdout_dn22, var_fn61_calc_iq__qgdout_dn23, var_fn61_calc_iq__qgdout_dn24, var_fn61_calc_iq__qgdout_dn25, var_fn61_calc_iq__qgdout_dn26, var_fn61_calc_iq__qgdout_dn27, var_fn61_calc_iq__qgdout_dn28, var_fn61_calc_iq__qgdout_dn29, var_fn61_calc_iq__qgdout_db0, var_fn61_calc_iq__qgdout_db1, var_fn61_calc_iq__qgdout_db2, var_fn61_calc_iq__qgdout_db3, var_fn61_calc_iq__qgdout_db4, var_fn61_calc_iq__qgdout_db5, var_fn61_calc_iq__qgdout_db6, var_fn61_calc_iq__qgdout_db7, var_fn61_calc_iq__qgdout_db8, var_fn61_calc_iq__qgdout_db9, var_fn61_calc_iq__qgdout_db10, var_fn61_calc_iq__qgdout_db11, var_fn61_calc_iq__qgdout_db12, var_fn61_calc_iq__qgdout_db13, var_fn61_calc_iq__qgdout_db14, var_fn61_calc_iq__qgdout_db15, var_fn61_calc_iq__qgdout_db16, var_fn61_calc_iq__qgdout_db17, var_fn61_calc_iq__qgdout_db18, var_fn61_calc_iq__qgdout_db19, var_fn61_calc_iq__qgdout_db20, var_fn61_calc_iq__qgdout_db21, var_fn61_calc_iq__qgdout_db22, var_fn61_calc_iq__qgdout_db23, var_fn61_calc_iq__qgdout_db24, var_fn61_calc_iq__qgdout_db25, var_fn61_calc_iq__qgdout_db26, var_fn61_calc_iq__qgdout_db27, var_fn61_calc_iq__qgdout_db28, var_fn61_calc_iq__qgdout_db29, var_fn61_calc_iq__qgdout_db30, var_fn61_calc_iq__qgdout_db31, var_fn61_calc_iq__qgdout_db32, var_fn61_calc_iq__qgdout_db33, var_fn61_calc_iq__qgdout_db34, var_fn61_calc_iq__qgdout_db35,)
    } else {
        (var_qgdfp3, var_qgdfp3_dn0, var_qgdfp3_dn1, var_qgdfp3_dn2, var_qgdfp3_dn3, var_qgdfp3_dn4, var_qgdfp3_dn5, var_qgdfp3_dn6, var_qgdfp3_dn7, var_qgdfp3_dn8, var_qgdfp3_dn9, var_qgdfp3_dn10, var_qgdfp3_dn11, var_qgdfp3_dn12, var_qgdfp3_dn13, var_qgdfp3_dn14, var_qgdfp3_dn15, var_qgdfp3_dn16, var_qgdfp3_dn17, var_qgdfp3_dn18, var_qgdfp3_dn19, var_qgdfp3_dn20, var_qgdfp3_dn21, var_qgdfp3_dn22, var_qgdfp3_dn23, var_qgdfp3_dn24, var_qgdfp3_dn25, var_qgdfp3_dn26, var_qgdfp3_dn27, var_qgdfp3_dn28, var_qgdfp3_dn29, var_qgdfp3_db0, var_qgdfp3_db1, var_qgdfp3_db2, var_qgdfp3_db3, var_qgdfp3_db4, var_qgdfp3_db5, var_qgdfp3_db6, var_qgdfp3_db7, var_qgdfp3_db8, var_qgdfp3_db9, var_qgdfp3_db10, var_qgdfp3_db11, var_qgdfp3_db12, var_qgdfp3_db13, var_qgdfp3_db14, var_qgdfp3_db15, var_qgdfp3_db16, var_qgdfp3_db17, var_qgdfp3_db18, var_qgdfp3_db19, var_qgdfp3_db20, var_qgdfp3_db21, var_qgdfp3_db22, var_qgdfp3_db23, var_qgdfp3_db24, var_qgdfp3_db25, var_qgdfp3_db26, var_qgdfp3_db27, var_qgdfp3_db28, var_qgdfp3_db29, var_qgdfp3_db30, var_qgdfp3_db31, var_qgdfp3_db32, var_qgdfp3_db33, var_qgdfp3_db34, var_qgdfp3_db35,)
    }
};
        var_qgdfp3 = assign7270_e8695;
        var_qgdfp3_dn0 = assign7270_e8695_d_n0;
        var_qgdfp3_dn1 = assign7270_e8695_d_n1;
        var_qgdfp3_dn2 = assign7270_e8695_d_n2;
        var_qgdfp3_dn3 = assign7270_e8695_d_n3;
        var_qgdfp3_dn4 = assign7270_e8695_d_n4;
        var_qgdfp3_dn5 = assign7270_e8695_d_n5;
        var_qgdfp3_dn6 = assign7270_e8695_d_n6;
        var_qgdfp3_dn7 = assign7270_e8695_d_n7;
        var_qgdfp3_dn8 = assign7270_e8695_d_n8;
        var_qgdfp3_dn9 = assign7270_e8695_d_n9;
        var_qgdfp3_dn10 = assign7270_e8695_d_n10;
        var_qgdfp3_dn11 = assign7270_e8695_d_n11;
        var_qgdfp3_dn12 = assign7270_e8695_d_n12;
        var_qgdfp3_dn13 = assign7270_e8695_d_n13;
        var_qgdfp3_dn14 = assign7270_e8695_d_n14;
        var_qgdfp3_dn15 = assign7270_e8695_d_n15;
        var_qgdfp3_dn16 = assign7270_e8695_d_n16;
        var_qgdfp3_dn17 = assign7270_e8695_d_n17;
        var_qgdfp3_dn18 = assign7270_e8695_d_n18;
        var_qgdfp3_dn19 = assign7270_e8695_d_n19;
        var_qgdfp3_dn20 = assign7270_e8695_d_n20;
        var_qgdfp3_dn21 = assign7270_e8695_d_n21;
        var_qgdfp3_dn22 = assign7270_e8695_d_n22;
        var_qgdfp3_dn23 = assign7270_e8695_d_n23;
        var_qgdfp3_dn24 = assign7270_e8695_d_n24;
        var_qgdfp3_dn25 = assign7270_e8695_d_n25;
        var_qgdfp3_dn26 = assign7270_e8695_d_n26;
        var_qgdfp3_dn27 = assign7270_e8695_d_n27;
        var_qgdfp3_dn28 = assign7270_e8695_d_n28;
        var_qgdfp3_dn29 = assign7270_e8695_d_n29;
        var_qgdfp3_db0 = assign7270_e8695_d_b0;
        var_qgdfp3_db1 = assign7270_e8695_d_b1;
        var_qgdfp3_db2 = assign7270_e8695_d_b2;
        var_qgdfp3_db3 = assign7270_e8695_d_b3;
        var_qgdfp3_db4 = assign7270_e8695_d_b4;
        var_qgdfp3_db5 = assign7270_e8695_d_b5;
        var_qgdfp3_db6 = assign7270_e8695_d_b6;
        var_qgdfp3_db7 = assign7270_e8695_d_b7;
        var_qgdfp3_db8 = assign7270_e8695_d_b8;
        var_qgdfp3_db9 = assign7270_e8695_d_b9;
        var_qgdfp3_db10 = assign7270_e8695_d_b10;
        var_qgdfp3_db11 = assign7270_e8695_d_b11;
        var_qgdfp3_db12 = assign7270_e8695_d_b12;
        var_qgdfp3_db13 = assign7270_e8695_d_b13;
        var_qgdfp3_db14 = assign7270_e8695_d_b14;
        var_qgdfp3_db15 = assign7270_e8695_d_b15;
        var_qgdfp3_db16 = assign7270_e8695_d_b16;
        var_qgdfp3_db17 = assign7270_e8695_d_b17;
        var_qgdfp3_db18 = assign7270_e8695_d_b18;
        var_qgdfp3_db19 = assign7270_e8695_d_b19;
        var_qgdfp3_db20 = assign7270_e8695_d_b20;
        var_qgdfp3_db21 = assign7270_e8695_d_b21;
        var_qgdfp3_db22 = assign7270_e8695_d_b22;
        var_qgdfp3_db23 = assign7270_e8695_d_b23;
        var_qgdfp3_db24 = assign7270_e8695_d_b24;
        var_qgdfp3_db25 = assign7270_e8695_d_b25;
        var_qgdfp3_db26 = assign7270_e8695_d_b26;
        var_qgdfp3_db27 = assign7270_e8695_d_b27;
        var_qgdfp3_db28 = assign7270_e8695_d_b28;
        var_qgdfp3_db29 = assign7270_e8695_d_b29;
        var_qgdfp3_db30 = assign7270_e8695_d_b30;
        var_qgdfp3_db31 = assign7270_e8695_d_b31;
        var_qgdfp3_db32 = assign7270_e8695_d_b32;
        var_qgdfp3_db33 = assign7270_e8695_d_b33;
        var_qgdfp3_db34 = assign7270_e8695_d_b34;
        var_qgdfp3_db35 = assign7270_e8695_d_b35;

        let (assign7280_e8699, assign7280_e8699_d_n0, assign7280_e8699_d_n1, assign7280_e8699_d_n2, assign7280_e8699_d_n3, assign7280_e8699_d_n4, assign7280_e8699_d_n5, assign7280_e8699_d_n6, assign7280_e8699_d_n7, assign7280_e8699_d_n8, assign7280_e8699_d_n9, assign7280_e8699_d_n10, assign7280_e8699_d_n11, assign7280_e8699_d_n12, assign7280_e8699_d_n13, assign7280_e8699_d_n14, assign7280_e8699_d_n15, assign7280_e8699_d_n16, assign7280_e8699_d_n17, assign7280_e8699_d_n18, assign7280_e8699_d_n19, assign7280_e8699_d_n20, assign7280_e8699_d_n21, assign7280_e8699_d_n22, assign7280_e8699_d_n23, assign7280_e8699_d_n24, assign7280_e8699_d_n25, assign7280_e8699_d_n26, assign7280_e8699_d_n27, assign7280_e8699_d_n28, assign7280_e8699_d_n29, assign7280_e8699_d_b0, assign7280_e8699_d_b1, assign7280_e8699_d_b2, assign7280_e8699_d_b3, assign7280_e8699_d_b4, assign7280_e8699_d_b5, assign7280_e8699_d_b6, assign7280_e8699_d_b7, assign7280_e8699_d_b8, assign7280_e8699_d_b9, assign7280_e8699_d_b10, assign7280_e8699_d_b11, assign7280_e8699_d_b12, assign7280_e8699_d_b13, assign7280_e8699_d_b14, assign7280_e8699_d_b15, assign7280_e8699_d_b16, assign7280_e8699_d_b17, assign7280_e8699_d_b18, assign7280_e8699_d_b19, assign7280_e8699_d_b20, assign7280_e8699_d_b21, assign7280_e8699_d_b22, assign7280_e8699_d_b23, assign7280_e8699_d_b24, assign7280_e8699_d_b25, assign7280_e8699_d_b26, assign7280_e8699_d_b27, assign7280_e8699_d_b28, assign7280_e8699_d_b29, assign7280_e8699_d_b30, assign7280_e8699_d_b31, assign7280_e8699_d_b32, assign7280_e8699_d_b33, assign7280_e8699_d_b34, assign7280_e8699_d_b35,) = {
    if (var_guard60 != 0.0) {
        (var_fn61_calc_iq__qcout, var_fn61_calc_iq__qcout_dn0, var_fn61_calc_iq__qcout_dn1, var_fn61_calc_iq__qcout_dn2, var_fn61_calc_iq__qcout_dn3, var_fn61_calc_iq__qcout_dn4, var_fn61_calc_iq__qcout_dn5, var_fn61_calc_iq__qcout_dn6, var_fn61_calc_iq__qcout_dn7, var_fn61_calc_iq__qcout_dn8, var_fn61_calc_iq__qcout_dn9, var_fn61_calc_iq__qcout_dn10, var_fn61_calc_iq__qcout_dn11, var_fn61_calc_iq__qcout_dn12, var_fn61_calc_iq__qcout_dn13, var_fn61_calc_iq__qcout_dn14, var_fn61_calc_iq__qcout_dn15, var_fn61_calc_iq__qcout_dn16, var_fn61_calc_iq__qcout_dn17, var_fn61_calc_iq__qcout_dn18, var_fn61_calc_iq__qcout_dn19, var_fn61_calc_iq__qcout_dn20, var_fn61_calc_iq__qcout_dn21, var_fn61_calc_iq__qcout_dn22, var_fn61_calc_iq__qcout_dn23, var_fn61_calc_iq__qcout_dn24, var_fn61_calc_iq__qcout_dn25, var_fn61_calc_iq__qcout_dn26, var_fn61_calc_iq__qcout_dn27, var_fn61_calc_iq__qcout_dn28, var_fn61_calc_iq__qcout_dn29, var_fn61_calc_iq__qcout_db0, var_fn61_calc_iq__qcout_db1, var_fn61_calc_iq__qcout_db2, var_fn61_calc_iq__qcout_db3, var_fn61_calc_iq__qcout_db4, var_fn61_calc_iq__qcout_db5, var_fn61_calc_iq__qcout_db6, var_fn61_calc_iq__qcout_db7, var_fn61_calc_iq__qcout_db8, var_fn61_calc_iq__qcout_db9, var_fn61_calc_iq__qcout_db10, var_fn61_calc_iq__qcout_db11, var_fn61_calc_iq__qcout_db12, var_fn61_calc_iq__qcout_db13, var_fn61_calc_iq__qcout_db14, var_fn61_calc_iq__qcout_db15, var_fn61_calc_iq__qcout_db16, var_fn61_calc_iq__qcout_db17, var_fn61_calc_iq__qcout_db18, var_fn61_calc_iq__qcout_db19, var_fn61_calc_iq__qcout_db20, var_fn61_calc_iq__qcout_db21, var_fn61_calc_iq__qcout_db22, var_fn61_calc_iq__qcout_db23, var_fn61_calc_iq__qcout_db24, var_fn61_calc_iq__qcout_db25, var_fn61_calc_iq__qcout_db26, var_fn61_calc_iq__qcout_db27, var_fn61_calc_iq__qcout_db28, var_fn61_calc_iq__qcout_db29, var_fn61_calc_iq__qcout_db30, var_fn61_calc_iq__qcout_db31, var_fn61_calc_iq__qcout_db32, var_fn61_calc_iq__qcout_db33, var_fn61_calc_iq__qcout_db34, var_fn61_calc_iq__qcout_db35,)
    } else {
        (var_qcfp3, var_qcfp3_dn0, var_qcfp3_dn1, var_qcfp3_dn2, var_qcfp3_dn3, var_qcfp3_dn4, var_qcfp3_dn5, var_qcfp3_dn6, var_qcfp3_dn7, var_qcfp3_dn8, var_qcfp3_dn9, var_qcfp3_dn10, var_qcfp3_dn11, var_qcfp3_dn12, var_qcfp3_dn13, var_qcfp3_dn14, var_qcfp3_dn15, var_qcfp3_dn16, var_qcfp3_dn17, var_qcfp3_dn18, var_qcfp3_dn19, var_qcfp3_dn20, var_qcfp3_dn21, var_qcfp3_dn22, var_qcfp3_dn23, var_qcfp3_dn24, var_qcfp3_dn25, var_qcfp3_dn26, var_qcfp3_dn27, var_qcfp3_dn28, var_qcfp3_dn29, var_qcfp3_db0, var_qcfp3_db1, var_qcfp3_db2, var_qcfp3_db3, var_qcfp3_db4, var_qcfp3_db5, var_qcfp3_db6, var_qcfp3_db7, var_qcfp3_db8, var_qcfp3_db9, var_qcfp3_db10, var_qcfp3_db11, var_qcfp3_db12, var_qcfp3_db13, var_qcfp3_db14, var_qcfp3_db15, var_qcfp3_db16, var_qcfp3_db17, var_qcfp3_db18, var_qcfp3_db19, var_qcfp3_db20, var_qcfp3_db21, var_qcfp3_db22, var_qcfp3_db23, var_qcfp3_db24, var_qcfp3_db25, var_qcfp3_db26, var_qcfp3_db27, var_qcfp3_db28, var_qcfp3_db29, var_qcfp3_db30, var_qcfp3_db31, var_qcfp3_db32, var_qcfp3_db33, var_qcfp3_db34, var_qcfp3_db35,)
    }
};
        var_qcfp3 = assign7280_e8699;
        var_qcfp3_dn0 = assign7280_e8699_d_n0;
        var_qcfp3_dn1 = assign7280_e8699_d_n1;
        var_qcfp3_dn2 = assign7280_e8699_d_n2;
        var_qcfp3_dn3 = assign7280_e8699_d_n3;
        var_qcfp3_dn4 = assign7280_e8699_d_n4;
        var_qcfp3_dn5 = assign7280_e8699_d_n5;
        var_qcfp3_dn6 = assign7280_e8699_d_n6;
        var_qcfp3_dn7 = assign7280_e8699_d_n7;
        var_qcfp3_dn8 = assign7280_e8699_d_n8;
        var_qcfp3_dn9 = assign7280_e8699_d_n9;
        var_qcfp3_dn10 = assign7280_e8699_d_n10;
        var_qcfp3_dn11 = assign7280_e8699_d_n11;
        var_qcfp3_dn12 = assign7280_e8699_d_n12;
        var_qcfp3_dn13 = assign7280_e8699_d_n13;
        var_qcfp3_dn14 = assign7280_e8699_d_n14;
        var_qcfp3_dn15 = assign7280_e8699_d_n15;
        var_qcfp3_dn16 = assign7280_e8699_d_n16;
        var_qcfp3_dn17 = assign7280_e8699_d_n17;
        var_qcfp3_dn18 = assign7280_e8699_d_n18;
        var_qcfp3_dn19 = assign7280_e8699_d_n19;
        var_qcfp3_dn20 = assign7280_e8699_d_n20;
        var_qcfp3_dn21 = assign7280_e8699_d_n21;
        var_qcfp3_dn22 = assign7280_e8699_d_n22;
        var_qcfp3_dn23 = assign7280_e8699_d_n23;
        var_qcfp3_dn24 = assign7280_e8699_d_n24;
        var_qcfp3_dn25 = assign7280_e8699_d_n25;
        var_qcfp3_dn26 = assign7280_e8699_d_n26;
        var_qcfp3_dn27 = assign7280_e8699_d_n27;
        var_qcfp3_dn28 = assign7280_e8699_d_n28;
        var_qcfp3_dn29 = assign7280_e8699_d_n29;
        var_qcfp3_db0 = assign7280_e8699_d_b0;
        var_qcfp3_db1 = assign7280_e8699_d_b1;
        var_qcfp3_db2 = assign7280_e8699_d_b2;
        var_qcfp3_db3 = assign7280_e8699_d_b3;
        var_qcfp3_db4 = assign7280_e8699_d_b4;
        var_qcfp3_db5 = assign7280_e8699_d_b5;
        var_qcfp3_db6 = assign7280_e8699_d_b6;
        var_qcfp3_db7 = assign7280_e8699_d_b7;
        var_qcfp3_db8 = assign7280_e8699_d_b8;
        var_qcfp3_db9 = assign7280_e8699_d_b9;
        var_qcfp3_db10 = assign7280_e8699_d_b10;
        var_qcfp3_db11 = assign7280_e8699_d_b11;
        var_qcfp3_db12 = assign7280_e8699_d_b12;
        var_qcfp3_db13 = assign7280_e8699_d_b13;
        var_qcfp3_db14 = assign7280_e8699_d_b14;
        var_qcfp3_db15 = assign7280_e8699_d_b15;
        var_qcfp3_db16 = assign7280_e8699_d_b16;
        var_qcfp3_db17 = assign7280_e8699_d_b17;
        var_qcfp3_db18 = assign7280_e8699_d_b18;
        var_qcfp3_db19 = assign7280_e8699_d_b19;
        var_qcfp3_db20 = assign7280_e8699_d_b20;
        var_qcfp3_db21 = assign7280_e8699_d_b21;
        var_qcfp3_db22 = assign7280_e8699_d_b22;
        var_qcfp3_db23 = assign7280_e8699_d_b23;
        var_qcfp3_db24 = assign7280_e8699_d_b24;
        var_qcfp3_db25 = assign7280_e8699_d_b25;
        var_qcfp3_db26 = assign7280_e8699_d_b26;
        var_qcfp3_db27 = assign7280_e8699_d_b27;
        var_qcfp3_db28 = assign7280_e8699_d_b28;
        var_qcfp3_db29 = assign7280_e8699_d_b29;
        var_qcfp3_db30 = assign7280_e8699_d_b30;
        var_qcfp3_db31 = assign7280_e8699_d_b31;
        var_qcfp3_db32 = assign7280_e8699_d_b32;
        var_qcfp3_db33 = assign7280_e8699_d_b33;
        var_qcfp3_db34 = assign7280_e8699_d_b34;
        var_qcfp3_db35 = assign7280_e8699_d_b35;

        let (assign7290_e8703, assign7290_e8703_d_n0, assign7290_e8703_d_n1, assign7290_e8703_d_n2, assign7290_e8703_d_n3, assign7290_e8703_d_n4, assign7290_e8703_d_n5, assign7290_e8703_d_n6, assign7290_e8703_d_n7, assign7290_e8703_d_n8, assign7290_e8703_d_n9, assign7290_e8703_d_n10, assign7290_e8703_d_n11, assign7290_e8703_d_n12, assign7290_e8703_d_n13, assign7290_e8703_d_n14, assign7290_e8703_d_n15, assign7290_e8703_d_n16, assign7290_e8703_d_n17, assign7290_e8703_d_n18, assign7290_e8703_d_n19, assign7290_e8703_d_n20, assign7290_e8703_d_n21, assign7290_e8703_d_n22, assign7290_e8703_d_n23, assign7290_e8703_d_n24, assign7290_e8703_d_n25, assign7290_e8703_d_n26, assign7290_e8703_d_n27, assign7290_e8703_d_n28, assign7290_e8703_d_n29, assign7290_e8703_d_b0, assign7290_e8703_d_b1, assign7290_e8703_d_b2, assign7290_e8703_d_b3, assign7290_e8703_d_b4, assign7290_e8703_d_b5, assign7290_e8703_d_b6, assign7290_e8703_d_b7, assign7290_e8703_d_b8, assign7290_e8703_d_b9, assign7290_e8703_d_b10, assign7290_e8703_d_b11, assign7290_e8703_d_b12, assign7290_e8703_d_b13, assign7290_e8703_d_b14, assign7290_e8703_d_b15, assign7290_e8703_d_b16, assign7290_e8703_d_b17, assign7290_e8703_d_b18, assign7290_e8703_d_b19, assign7290_e8703_d_b20, assign7290_e8703_d_b21, assign7290_e8703_d_b22, assign7290_e8703_d_b23, assign7290_e8703_d_b24, assign7290_e8703_d_b25, assign7290_e8703_d_b26, assign7290_e8703_d_b27, assign7290_e8703_d_b28, assign7290_e8703_d_b29, assign7290_e8703_d_b30, assign7290_e8703_d_b31, assign7290_e8703_d_b32, assign7290_e8703_d_b33, assign7290_e8703_d_b34, assign7290_e8703_d_b35,) = {
    if (var_guard60 != 0.0) {
        (var_fn61_calc_iq__qbout, var_fn61_calc_iq__qbout_dn0, var_fn61_calc_iq__qbout_dn1, var_fn61_calc_iq__qbout_dn2, var_fn61_calc_iq__qbout_dn3, var_fn61_calc_iq__qbout_dn4, var_fn61_calc_iq__qbout_dn5, var_fn61_calc_iq__qbout_dn6, var_fn61_calc_iq__qbout_dn7, var_fn61_calc_iq__qbout_dn8, var_fn61_calc_iq__qbout_dn9, var_fn61_calc_iq__qbout_dn10, var_fn61_calc_iq__qbout_dn11, var_fn61_calc_iq__qbout_dn12, var_fn61_calc_iq__qbout_dn13, var_fn61_calc_iq__qbout_dn14, var_fn61_calc_iq__qbout_dn15, var_fn61_calc_iq__qbout_dn16, var_fn61_calc_iq__qbout_dn17, var_fn61_calc_iq__qbout_dn18, var_fn61_calc_iq__qbout_dn19, var_fn61_calc_iq__qbout_dn20, var_fn61_calc_iq__qbout_dn21, var_fn61_calc_iq__qbout_dn22, var_fn61_calc_iq__qbout_dn23, var_fn61_calc_iq__qbout_dn24, var_fn61_calc_iq__qbout_dn25, var_fn61_calc_iq__qbout_dn26, var_fn61_calc_iq__qbout_dn27, var_fn61_calc_iq__qbout_dn28, var_fn61_calc_iq__qbout_dn29, var_fn61_calc_iq__qbout_db0, var_fn61_calc_iq__qbout_db1, var_fn61_calc_iq__qbout_db2, var_fn61_calc_iq__qbout_db3, var_fn61_calc_iq__qbout_db4, var_fn61_calc_iq__qbout_db5, var_fn61_calc_iq__qbout_db6, var_fn61_calc_iq__qbout_db7, var_fn61_calc_iq__qbout_db8, var_fn61_calc_iq__qbout_db9, var_fn61_calc_iq__qbout_db10, var_fn61_calc_iq__qbout_db11, var_fn61_calc_iq__qbout_db12, var_fn61_calc_iq__qbout_db13, var_fn61_calc_iq__qbout_db14, var_fn61_calc_iq__qbout_db15, var_fn61_calc_iq__qbout_db16, var_fn61_calc_iq__qbout_db17, var_fn61_calc_iq__qbout_db18, var_fn61_calc_iq__qbout_db19, var_fn61_calc_iq__qbout_db20, var_fn61_calc_iq__qbout_db21, var_fn61_calc_iq__qbout_db22, var_fn61_calc_iq__qbout_db23, var_fn61_calc_iq__qbout_db24, var_fn61_calc_iq__qbout_db25, var_fn61_calc_iq__qbout_db26, var_fn61_calc_iq__qbout_db27, var_fn61_calc_iq__qbout_db28, var_fn61_calc_iq__qbout_db29, var_fn61_calc_iq__qbout_db30, var_fn61_calc_iq__qbout_db31, var_fn61_calc_iq__qbout_db32, var_fn61_calc_iq__qbout_db33, var_fn61_calc_iq__qbout_db34, var_fn61_calc_iq__qbout_db35,)
    } else {
        (var_qbfp3, var_qbfp3_dn0, var_qbfp3_dn1, var_qbfp3_dn2, var_qbfp3_dn3, var_qbfp3_dn4, var_qbfp3_dn5, var_qbfp3_dn6, var_qbfp3_dn7, var_qbfp3_dn8, var_qbfp3_dn9, var_qbfp3_dn10, var_qbfp3_dn11, var_qbfp3_dn12, var_qbfp3_dn13, var_qbfp3_dn14, var_qbfp3_dn15, var_qbfp3_dn16, var_qbfp3_dn17, var_qbfp3_dn18, var_qbfp3_dn19, var_qbfp3_dn20, var_qbfp3_dn21, var_qbfp3_dn22, var_qbfp3_dn23, var_qbfp3_dn24, var_qbfp3_dn25, var_qbfp3_dn26, var_qbfp3_dn27, var_qbfp3_dn28, var_qbfp3_dn29, var_qbfp3_db0, var_qbfp3_db1, var_qbfp3_db2, var_qbfp3_db3, var_qbfp3_db4, var_qbfp3_db5, var_qbfp3_db6, var_qbfp3_db7, var_qbfp3_db8, var_qbfp3_db9, var_qbfp3_db10, var_qbfp3_db11, var_qbfp3_db12, var_qbfp3_db13, var_qbfp3_db14, var_qbfp3_db15, var_qbfp3_db16, var_qbfp3_db17, var_qbfp3_db18, var_qbfp3_db19, var_qbfp3_db20, var_qbfp3_db21, var_qbfp3_db22, var_qbfp3_db23, var_qbfp3_db24, var_qbfp3_db25, var_qbfp3_db26, var_qbfp3_db27, var_qbfp3_db28, var_qbfp3_db29, var_qbfp3_db30, var_qbfp3_db31, var_qbfp3_db32, var_qbfp3_db33, var_qbfp3_db34, var_qbfp3_db35,)
    }
};
        var_qbfp3 = assign7290_e8703;
        var_qbfp3_dn0 = assign7290_e8703_d_n0;
        var_qbfp3_dn1 = assign7290_e8703_d_n1;
        var_qbfp3_dn2 = assign7290_e8703_d_n2;
        var_qbfp3_dn3 = assign7290_e8703_d_n3;
        var_qbfp3_dn4 = assign7290_e8703_d_n4;
        var_qbfp3_dn5 = assign7290_e8703_d_n5;
        var_qbfp3_dn6 = assign7290_e8703_d_n6;
        var_qbfp3_dn7 = assign7290_e8703_d_n7;
        var_qbfp3_dn8 = assign7290_e8703_d_n8;
        var_qbfp3_dn9 = assign7290_e8703_d_n9;
        var_qbfp3_dn10 = assign7290_e8703_d_n10;
        var_qbfp3_dn11 = assign7290_e8703_d_n11;
        var_qbfp3_dn12 = assign7290_e8703_d_n12;
        var_qbfp3_dn13 = assign7290_e8703_d_n13;
        var_qbfp3_dn14 = assign7290_e8703_d_n14;
        var_qbfp3_dn15 = assign7290_e8703_d_n15;
        var_qbfp3_dn16 = assign7290_e8703_d_n16;
        var_qbfp3_dn17 = assign7290_e8703_d_n17;
        var_qbfp3_dn18 = assign7290_e8703_d_n18;
        var_qbfp3_dn19 = assign7290_e8703_d_n19;
        var_qbfp3_dn20 = assign7290_e8703_d_n20;
        var_qbfp3_dn21 = assign7290_e8703_d_n21;
        var_qbfp3_dn22 = assign7290_e8703_d_n22;
        var_qbfp3_dn23 = assign7290_e8703_d_n23;
        var_qbfp3_dn24 = assign7290_e8703_d_n24;
        var_qbfp3_dn25 = assign7290_e8703_d_n25;
        var_qbfp3_dn26 = assign7290_e8703_d_n26;
        var_qbfp3_dn27 = assign7290_e8703_d_n27;
        var_qbfp3_dn28 = assign7290_e8703_d_n28;
        var_qbfp3_dn29 = assign7290_e8703_d_n29;
        var_qbfp3_db0 = assign7290_e8703_d_b0;
        var_qbfp3_db1 = assign7290_e8703_d_b1;
        var_qbfp3_db2 = assign7290_e8703_d_b2;
        var_qbfp3_db3 = assign7290_e8703_d_b3;
        var_qbfp3_db4 = assign7290_e8703_d_b4;
        var_qbfp3_db5 = assign7290_e8703_d_b5;
        var_qbfp3_db6 = assign7290_e8703_d_b6;
        var_qbfp3_db7 = assign7290_e8703_d_b7;
        var_qbfp3_db8 = assign7290_e8703_d_b8;
        var_qbfp3_db9 = assign7290_e8703_d_b9;
        var_qbfp3_db10 = assign7290_e8703_d_b10;
        var_qbfp3_db11 = assign7290_e8703_d_b11;
        var_qbfp3_db12 = assign7290_e8703_d_b12;
        var_qbfp3_db13 = assign7290_e8703_d_b13;
        var_qbfp3_db14 = assign7290_e8703_d_b14;
        var_qbfp3_db15 = assign7290_e8703_d_b15;
        var_qbfp3_db16 = assign7290_e8703_d_b16;
        var_qbfp3_db17 = assign7290_e8703_d_b17;
        var_qbfp3_db18 = assign7290_e8703_d_b18;
        var_qbfp3_db19 = assign7290_e8703_d_b19;
        var_qbfp3_db20 = assign7290_e8703_d_b20;
        var_qbfp3_db21 = assign7290_e8703_d_b21;
        var_qbfp3_db22 = assign7290_e8703_d_b22;
        var_qbfp3_db23 = assign7290_e8703_d_b23;
        var_qbfp3_db24 = assign7290_e8703_d_b24;
        var_qbfp3_db25 = assign7290_e8703_d_b25;
        var_qbfp3_db26 = assign7290_e8703_d_b26;
        var_qbfp3_db27 = assign7290_e8703_d_b27;
        var_qbfp3_db28 = assign7290_e8703_d_b28;
        var_qbfp3_db29 = assign7290_e8703_d_b29;
        var_qbfp3_db30 = assign7290_e8703_d_b30;
        var_qbfp3_db31 = assign7290_e8703_d_b31;
        var_qbfp3_db32 = assign7290_e8703_d_b32;
        var_qbfp3_db33 = assign7290_e8703_d_b33;
        var_qbfp3_db34 = assign7290_e8703_d_b34;
        var_qbfp3_db35 = assign7290_e8703_d_b35;

        let (assign7300_e8707, assign7300_e8707_d_n0, assign7300_e8707_d_n1, assign7300_e8707_d_n2, assign7300_e8707_d_n3, assign7300_e8707_d_n4, assign7300_e8707_d_n5, assign7300_e8707_d_n6, assign7300_e8707_d_n7, assign7300_e8707_d_n8, assign7300_e8707_d_n9, assign7300_e8707_d_n10, assign7300_e8707_d_n11, assign7300_e8707_d_n12, assign7300_e8707_d_n13, assign7300_e8707_d_n14, assign7300_e8707_d_n15, assign7300_e8707_d_n16, assign7300_e8707_d_n17, assign7300_e8707_d_n18, assign7300_e8707_d_n19, assign7300_e8707_d_n20, assign7300_e8707_d_n21, assign7300_e8707_d_n22, assign7300_e8707_d_n23, assign7300_e8707_d_n24, assign7300_e8707_d_n25, assign7300_e8707_d_n26, assign7300_e8707_d_n27, assign7300_e8707_d_n28, assign7300_e8707_d_n29, assign7300_e8707_d_b0, assign7300_e8707_d_b1, assign7300_e8707_d_b2, assign7300_e8707_d_b3, assign7300_e8707_d_b4, assign7300_e8707_d_b5, assign7300_e8707_d_b6, assign7300_e8707_d_b7, assign7300_e8707_d_b8, assign7300_e8707_d_b9, assign7300_e8707_d_b10, assign7300_e8707_d_b11, assign7300_e8707_d_b12, assign7300_e8707_d_b13, assign7300_e8707_d_b14, assign7300_e8707_d_b15, assign7300_e8707_d_b16, assign7300_e8707_d_b17, assign7300_e8707_d_b18, assign7300_e8707_d_b19, assign7300_e8707_d_b20, assign7300_e8707_d_b21, assign7300_e8707_d_b22, assign7300_e8707_d_b23, assign7300_e8707_d_b24, assign7300_e8707_d_b25, assign7300_e8707_d_b26, assign7300_e8707_d_b27, assign7300_e8707_d_b28, assign7300_e8707_d_b29, assign7300_e8707_d_b30, assign7300_e8707_d_b31, assign7300_e8707_d_b32, assign7300_e8707_d_b33, assign7300_e8707_d_b34, assign7300_e8707_d_b35,) = {
    if (var_guard60 != 0.0) {
        (var_fn61_calc_iq__qsout, var_fn61_calc_iq__qsout_dn0, var_fn61_calc_iq__qsout_dn1, var_fn61_calc_iq__qsout_dn2, var_fn61_calc_iq__qsout_dn3, var_fn61_calc_iq__qsout_dn4, var_fn61_calc_iq__qsout_dn5, var_fn61_calc_iq__qsout_dn6, var_fn61_calc_iq__qsout_dn7, var_fn61_calc_iq__qsout_dn8, var_fn61_calc_iq__qsout_dn9, var_fn61_calc_iq__qsout_dn10, var_fn61_calc_iq__qsout_dn11, var_fn61_calc_iq__qsout_dn12, var_fn61_calc_iq__qsout_dn13, var_fn61_calc_iq__qsout_dn14, var_fn61_calc_iq__qsout_dn15, var_fn61_calc_iq__qsout_dn16, var_fn61_calc_iq__qsout_dn17, var_fn61_calc_iq__qsout_dn18, var_fn61_calc_iq__qsout_dn19, var_fn61_calc_iq__qsout_dn20, var_fn61_calc_iq__qsout_dn21, var_fn61_calc_iq__qsout_dn22, var_fn61_calc_iq__qsout_dn23, var_fn61_calc_iq__qsout_dn24, var_fn61_calc_iq__qsout_dn25, var_fn61_calc_iq__qsout_dn26, var_fn61_calc_iq__qsout_dn27, var_fn61_calc_iq__qsout_dn28, var_fn61_calc_iq__qsout_dn29, var_fn61_calc_iq__qsout_db0, var_fn61_calc_iq__qsout_db1, var_fn61_calc_iq__qsout_db2, var_fn61_calc_iq__qsout_db3, var_fn61_calc_iq__qsout_db4, var_fn61_calc_iq__qsout_db5, var_fn61_calc_iq__qsout_db6, var_fn61_calc_iq__qsout_db7, var_fn61_calc_iq__qsout_db8, var_fn61_calc_iq__qsout_db9, var_fn61_calc_iq__qsout_db10, var_fn61_calc_iq__qsout_db11, var_fn61_calc_iq__qsout_db12, var_fn61_calc_iq__qsout_db13, var_fn61_calc_iq__qsout_db14, var_fn61_calc_iq__qsout_db15, var_fn61_calc_iq__qsout_db16, var_fn61_calc_iq__qsout_db17, var_fn61_calc_iq__qsout_db18, var_fn61_calc_iq__qsout_db19, var_fn61_calc_iq__qsout_db20, var_fn61_calc_iq__qsout_db21, var_fn61_calc_iq__qsout_db22, var_fn61_calc_iq__qsout_db23, var_fn61_calc_iq__qsout_db24, var_fn61_calc_iq__qsout_db25, var_fn61_calc_iq__qsout_db26, var_fn61_calc_iq__qsout_db27, var_fn61_calc_iq__qsout_db28, var_fn61_calc_iq__qsout_db29, var_fn61_calc_iq__qsout_db30, var_fn61_calc_iq__qsout_db31, var_fn61_calc_iq__qsout_db32, var_fn61_calc_iq__qsout_db33, var_fn61_calc_iq__qsout_db34, var_fn61_calc_iq__qsout_db35,)
    } else {
        (var_qsfp3, var_qsfp3_dn0, var_qsfp3_dn1, var_qsfp3_dn2, var_qsfp3_dn3, var_qsfp3_dn4, var_qsfp3_dn5, var_qsfp3_dn6, var_qsfp3_dn7, var_qsfp3_dn8, var_qsfp3_dn9, var_qsfp3_dn10, var_qsfp3_dn11, var_qsfp3_dn12, var_qsfp3_dn13, var_qsfp3_dn14, var_qsfp3_dn15, var_qsfp3_dn16, var_qsfp3_dn17, var_qsfp3_dn18, var_qsfp3_dn19, var_qsfp3_dn20, var_qsfp3_dn21, var_qsfp3_dn22, var_qsfp3_dn23, var_qsfp3_dn24, var_qsfp3_dn25, var_qsfp3_dn26, var_qsfp3_dn27, var_qsfp3_dn28, var_qsfp3_dn29, var_qsfp3_db0, var_qsfp3_db1, var_qsfp3_db2, var_qsfp3_db3, var_qsfp3_db4, var_qsfp3_db5, var_qsfp3_db6, var_qsfp3_db7, var_qsfp3_db8, var_qsfp3_db9, var_qsfp3_db10, var_qsfp3_db11, var_qsfp3_db12, var_qsfp3_db13, var_qsfp3_db14, var_qsfp3_db15, var_qsfp3_db16, var_qsfp3_db17, var_qsfp3_db18, var_qsfp3_db19, var_qsfp3_db20, var_qsfp3_db21, var_qsfp3_db22, var_qsfp3_db23, var_qsfp3_db24, var_qsfp3_db25, var_qsfp3_db26, var_qsfp3_db27, var_qsfp3_db28, var_qsfp3_db29, var_qsfp3_db30, var_qsfp3_db31, var_qsfp3_db32, var_qsfp3_db33, var_qsfp3_db34, var_qsfp3_db35,)
    }
};
        var_qsfp3 = assign7300_e8707;
        var_qsfp3_dn0 = assign7300_e8707_d_n0;
        var_qsfp3_dn1 = assign7300_e8707_d_n1;
        var_qsfp3_dn2 = assign7300_e8707_d_n2;
        var_qsfp3_dn3 = assign7300_e8707_d_n3;
        var_qsfp3_dn4 = assign7300_e8707_d_n4;
        var_qsfp3_dn5 = assign7300_e8707_d_n5;
        var_qsfp3_dn6 = assign7300_e8707_d_n6;
        var_qsfp3_dn7 = assign7300_e8707_d_n7;
        var_qsfp3_dn8 = assign7300_e8707_d_n8;
        var_qsfp3_dn9 = assign7300_e8707_d_n9;
        var_qsfp3_dn10 = assign7300_e8707_d_n10;
        var_qsfp3_dn11 = assign7300_e8707_d_n11;
        var_qsfp3_dn12 = assign7300_e8707_d_n12;
        var_qsfp3_dn13 = assign7300_e8707_d_n13;
        var_qsfp3_dn14 = assign7300_e8707_d_n14;
        var_qsfp3_dn15 = assign7300_e8707_d_n15;
        var_qsfp3_dn16 = assign7300_e8707_d_n16;
        var_qsfp3_dn17 = assign7300_e8707_d_n17;
        var_qsfp3_dn18 = assign7300_e8707_d_n18;
        var_qsfp3_dn19 = assign7300_e8707_d_n19;
        var_qsfp3_dn20 = assign7300_e8707_d_n20;
        var_qsfp3_dn21 = assign7300_e8707_d_n21;
        var_qsfp3_dn22 = assign7300_e8707_d_n22;
        var_qsfp3_dn23 = assign7300_e8707_d_n23;
        var_qsfp3_dn24 = assign7300_e8707_d_n24;
        var_qsfp3_dn25 = assign7300_e8707_d_n25;
        var_qsfp3_dn26 = assign7300_e8707_d_n26;
        var_qsfp3_dn27 = assign7300_e8707_d_n27;
        var_qsfp3_dn28 = assign7300_e8707_d_n28;
        var_qsfp3_dn29 = assign7300_e8707_d_n29;
        var_qsfp3_db0 = assign7300_e8707_d_b0;
        var_qsfp3_db1 = assign7300_e8707_d_b1;
        var_qsfp3_db2 = assign7300_e8707_d_b2;
        var_qsfp3_db3 = assign7300_e8707_d_b3;
        var_qsfp3_db4 = assign7300_e8707_d_b4;
        var_qsfp3_db5 = assign7300_e8707_d_b5;
        var_qsfp3_db6 = assign7300_e8707_d_b6;
        var_qsfp3_db7 = assign7300_e8707_d_b7;
        var_qsfp3_db8 = assign7300_e8707_d_b8;
        var_qsfp3_db9 = assign7300_e8707_d_b9;
        var_qsfp3_db10 = assign7300_e8707_d_b10;
        var_qsfp3_db11 = assign7300_e8707_d_b11;
        var_qsfp3_db12 = assign7300_e8707_d_b12;
        var_qsfp3_db13 = assign7300_e8707_d_b13;
        var_qsfp3_db14 = assign7300_e8707_d_b14;
        var_qsfp3_db15 = assign7300_e8707_d_b15;
        var_qsfp3_db16 = assign7300_e8707_d_b16;
        var_qsfp3_db17 = assign7300_e8707_d_b17;
        var_qsfp3_db18 = assign7300_e8707_d_b18;
        var_qsfp3_db19 = assign7300_e8707_d_b19;
        var_qsfp3_db20 = assign7300_e8707_d_b20;
        var_qsfp3_db21 = assign7300_e8707_d_b21;
        var_qsfp3_db22 = assign7300_e8707_d_b22;
        var_qsfp3_db23 = assign7300_e8707_d_b23;
        var_qsfp3_db24 = assign7300_e8707_d_b24;
        var_qsfp3_db25 = assign7300_e8707_d_b25;
        var_qsfp3_db26 = assign7300_e8707_d_b26;
        var_qsfp3_db27 = assign7300_e8707_d_b27;
        var_qsfp3_db28 = assign7300_e8707_d_b28;
        var_qsfp3_db29 = assign7300_e8707_d_b29;
        var_qsfp3_db30 = assign7300_e8707_d_b30;
        var_qsfp3_db31 = assign7300_e8707_d_b31;
        var_qsfp3_db32 = assign7300_e8707_d_b32;
        var_qsfp3_db33 = assign7300_e8707_d_b33;
        var_qsfp3_db34 = assign7300_e8707_d_b34;
        var_qsfp3_db35 = assign7300_e8707_d_b35;


        *var_idsfp3_slot = var_idsfp3;
        *var_idsfp3_db0_slot = var_idsfp3_db0;
        *var_idsfp3_db1_slot = var_idsfp3_db1;
        *var_idsfp3_db10_slot = var_idsfp3_db10;
        *var_idsfp3_db11_slot = var_idsfp3_db11;
        *var_idsfp3_db12_slot = var_idsfp3_db12;
        *var_idsfp3_db13_slot = var_idsfp3_db13;
        *var_idsfp3_db14_slot = var_idsfp3_db14;
        *var_idsfp3_db15_slot = var_idsfp3_db15;
        *var_idsfp3_db16_slot = var_idsfp3_db16;
        *var_idsfp3_db17_slot = var_idsfp3_db17;
        *var_idsfp3_db18_slot = var_idsfp3_db18;
        *var_idsfp3_db19_slot = var_idsfp3_db19;
        *var_idsfp3_db2_slot = var_idsfp3_db2;
        *var_idsfp3_db20_slot = var_idsfp3_db20;
        *var_idsfp3_db21_slot = var_idsfp3_db21;
        *var_idsfp3_db22_slot = var_idsfp3_db22;
        *var_idsfp3_db23_slot = var_idsfp3_db23;
        *var_idsfp3_db24_slot = var_idsfp3_db24;
        *var_idsfp3_db25_slot = var_idsfp3_db25;
        *var_idsfp3_db26_slot = var_idsfp3_db26;
        *var_idsfp3_db27_slot = var_idsfp3_db27;
        *var_idsfp3_db28_slot = var_idsfp3_db28;
        *var_idsfp3_db29_slot = var_idsfp3_db29;
        *var_idsfp3_db3_slot = var_idsfp3_db3;
        *var_idsfp3_db30_slot = var_idsfp3_db30;
        *var_idsfp3_db31_slot = var_idsfp3_db31;
        *var_idsfp3_db32_slot = var_idsfp3_db32;
        *var_idsfp3_db33_slot = var_idsfp3_db33;
        *var_idsfp3_db34_slot = var_idsfp3_db34;
        *var_idsfp3_db35_slot = var_idsfp3_db35;
        *var_idsfp3_db4_slot = var_idsfp3_db4;
        *var_idsfp3_db5_slot = var_idsfp3_db5;
        *var_idsfp3_db6_slot = var_idsfp3_db6;
        *var_idsfp3_db7_slot = var_idsfp3_db7;
        *var_idsfp3_db8_slot = var_idsfp3_db8;
        *var_idsfp3_db9_slot = var_idsfp3_db9;
        *var_idsfp3_dn0_slot = var_idsfp3_dn0;
        *var_idsfp3_dn1_slot = var_idsfp3_dn1;
        *var_idsfp3_dn10_slot = var_idsfp3_dn10;
        *var_idsfp3_dn11_slot = var_idsfp3_dn11;
        *var_idsfp3_dn12_slot = var_idsfp3_dn12;
        *var_idsfp3_dn13_slot = var_idsfp3_dn13;
        *var_idsfp3_dn14_slot = var_idsfp3_dn14;
        *var_idsfp3_dn15_slot = var_idsfp3_dn15;
        *var_idsfp3_dn16_slot = var_idsfp3_dn16;
        *var_idsfp3_dn17_slot = var_idsfp3_dn17;
        *var_idsfp3_dn18_slot = var_idsfp3_dn18;
        *var_idsfp3_dn19_slot = var_idsfp3_dn19;
        *var_idsfp3_dn2_slot = var_idsfp3_dn2;
        *var_idsfp3_dn20_slot = var_idsfp3_dn20;
        *var_idsfp3_dn21_slot = var_idsfp3_dn21;
        *var_idsfp3_dn22_slot = var_idsfp3_dn22;
        *var_idsfp3_dn23_slot = var_idsfp3_dn23;
        *var_idsfp3_dn24_slot = var_idsfp3_dn24;
        *var_idsfp3_dn25_slot = var_idsfp3_dn25;
        *var_idsfp3_dn26_slot = var_idsfp3_dn26;
        *var_idsfp3_dn27_slot = var_idsfp3_dn27;
        *var_idsfp3_dn28_slot = var_idsfp3_dn28;
        *var_idsfp3_dn29_slot = var_idsfp3_dn29;
        *var_idsfp3_dn3_slot = var_idsfp3_dn3;
        *var_idsfp3_dn4_slot = var_idsfp3_dn4;
        *var_idsfp3_dn5_slot = var_idsfp3_dn5;
        *var_idsfp3_dn6_slot = var_idsfp3_dn6;
        *var_idsfp3_dn7_slot = var_idsfp3_dn7;
        *var_idsfp3_dn8_slot = var_idsfp3_dn8;
        *var_idsfp3_dn9_slot = var_idsfp3_dn9;
        *var_qbfp3_slot = var_qbfp3;
        *var_qbfp3_db0_slot = var_qbfp3_db0;
        *var_qbfp3_db1_slot = var_qbfp3_db1;
        *var_qbfp3_db10_slot = var_qbfp3_db10;
        *var_qbfp3_db11_slot = var_qbfp3_db11;
        *var_qbfp3_db12_slot = var_qbfp3_db12;
        *var_qbfp3_db13_slot = var_qbfp3_db13;
        *var_qbfp3_db14_slot = var_qbfp3_db14;
        *var_qbfp3_db15_slot = var_qbfp3_db15;
        *var_qbfp3_db16_slot = var_qbfp3_db16;
        *var_qbfp3_db17_slot = var_qbfp3_db17;
        *var_qbfp3_db18_slot = var_qbfp3_db18;
        *var_qbfp3_db19_slot = var_qbfp3_db19;
        *var_qbfp3_db2_slot = var_qbfp3_db2;
        *var_qbfp3_db20_slot = var_qbfp3_db20;
        *var_qbfp3_db21_slot = var_qbfp3_db21;
        *var_qbfp3_db22_slot = var_qbfp3_db22;
        *var_qbfp3_db23_slot = var_qbfp3_db23;
        *var_qbfp3_db24_slot = var_qbfp3_db24;
        *var_qbfp3_db25_slot = var_qbfp3_db25;
        *var_qbfp3_db26_slot = var_qbfp3_db26;
        *var_qbfp3_db27_slot = var_qbfp3_db27;
        *var_qbfp3_db28_slot = var_qbfp3_db28;
        *var_qbfp3_db29_slot = var_qbfp3_db29;
        *var_qbfp3_db3_slot = var_qbfp3_db3;
        *var_qbfp3_db30_slot = var_qbfp3_db30;
        *var_qbfp3_db31_slot = var_qbfp3_db31;
        *var_qbfp3_db32_slot = var_qbfp3_db32;
        *var_qbfp3_db33_slot = var_qbfp3_db33;
        *var_qbfp3_db34_slot = var_qbfp3_db34;
        *var_qbfp3_db35_slot = var_qbfp3_db35;
        *var_qbfp3_db4_slot = var_qbfp3_db4;
        *var_qbfp3_db5_slot = var_qbfp3_db5;
        *var_qbfp3_db6_slot = var_qbfp3_db6;
        *var_qbfp3_db7_slot = var_qbfp3_db7;
        *var_qbfp3_db8_slot = var_qbfp3_db8;
        *var_qbfp3_db9_slot = var_qbfp3_db9;
        *var_qbfp3_dn0_slot = var_qbfp3_dn0;
        *var_qbfp3_dn1_slot = var_qbfp3_dn1;
        *var_qbfp3_dn10_slot = var_qbfp3_dn10;
        *var_qbfp3_dn11_slot = var_qbfp3_dn11;
        *var_qbfp3_dn12_slot = var_qbfp3_dn12;
        *var_qbfp3_dn13_slot = var_qbfp3_dn13;
        *var_qbfp3_dn14_slot = var_qbfp3_dn14;
        *var_qbfp3_dn15_slot = var_qbfp3_dn15;
        *var_qbfp3_dn16_slot = var_qbfp3_dn16;
        *var_qbfp3_dn17_slot = var_qbfp3_dn17;
        *var_qbfp3_dn18_slot = var_qbfp3_dn18;
        *var_qbfp3_dn19_slot = var_qbfp3_dn19;
        *var_qbfp3_dn2_slot = var_qbfp3_dn2;
        *var_qbfp3_dn20_slot = var_qbfp3_dn20;
        *var_qbfp3_dn21_slot = var_qbfp3_dn21;
        *var_qbfp3_dn22_slot = var_qbfp3_dn22;
        *var_qbfp3_dn23_slot = var_qbfp3_dn23;
        *var_qbfp3_dn24_slot = var_qbfp3_dn24;
        *var_qbfp3_dn25_slot = var_qbfp3_dn25;
        *var_qbfp3_dn26_slot = var_qbfp3_dn26;
        *var_qbfp3_dn27_slot = var_qbfp3_dn27;
        *var_qbfp3_dn28_slot = var_qbfp3_dn28;
        *var_qbfp3_dn29_slot = var_qbfp3_dn29;
        *var_qbfp3_dn3_slot = var_qbfp3_dn3;
        *var_qbfp3_dn4_slot = var_qbfp3_dn4;
        *var_qbfp3_dn5_slot = var_qbfp3_dn5;
        *var_qbfp3_dn6_slot = var_qbfp3_dn6;
        *var_qbfp3_dn7_slot = var_qbfp3_dn7;
        *var_qbfp3_dn8_slot = var_qbfp3_dn8;
        *var_qbfp3_dn9_slot = var_qbfp3_dn9;
        *var_qcfp3_slot = var_qcfp3;
        *var_qcfp3_db0_slot = var_qcfp3_db0;
        *var_qcfp3_db1_slot = var_qcfp3_db1;
        *var_qcfp3_db10_slot = var_qcfp3_db10;
        *var_qcfp3_db11_slot = var_qcfp3_db11;
        *var_qcfp3_db12_slot = var_qcfp3_db12;
        *var_qcfp3_db13_slot = var_qcfp3_db13;
        *var_qcfp3_db14_slot = var_qcfp3_db14;
        *var_qcfp3_db15_slot = var_qcfp3_db15;
        *var_qcfp3_db16_slot = var_qcfp3_db16;
        *var_qcfp3_db17_slot = var_qcfp3_db17;
        *var_qcfp3_db18_slot = var_qcfp3_db18;
        *var_qcfp3_db19_slot = var_qcfp3_db19;
        *var_qcfp3_db2_slot = var_qcfp3_db2;
        *var_qcfp3_db20_slot = var_qcfp3_db20;
        *var_qcfp3_db21_slot = var_qcfp3_db21;
        *var_qcfp3_db22_slot = var_qcfp3_db22;
        *var_qcfp3_db23_slot = var_qcfp3_db23;
        *var_qcfp3_db24_slot = var_qcfp3_db24;
        *var_qcfp3_db25_slot = var_qcfp3_db25;
        *var_qcfp3_db26_slot = var_qcfp3_db26;
        *var_qcfp3_db27_slot = var_qcfp3_db27;
        *var_qcfp3_db28_slot = var_qcfp3_db28;
        *var_qcfp3_db29_slot = var_qcfp3_db29;
        *var_qcfp3_db3_slot = var_qcfp3_db3;
        *var_qcfp3_db30_slot = var_qcfp3_db30;
        *var_qcfp3_db31_slot = var_qcfp3_db31;
        *var_qcfp3_db32_slot = var_qcfp3_db32;
        *var_qcfp3_db33_slot = var_qcfp3_db33;
        *var_qcfp3_db34_slot = var_qcfp3_db34;
        *var_qcfp3_db35_slot = var_qcfp3_db35;
        *var_qcfp3_db4_slot = var_qcfp3_db4;
        *var_qcfp3_db5_slot = var_qcfp3_db5;
        *var_qcfp3_db6_slot = var_qcfp3_db6;
        *var_qcfp3_db7_slot = var_qcfp3_db7;
        *var_qcfp3_db8_slot = var_qcfp3_db8;
        *var_qcfp3_db9_slot = var_qcfp3_db9;
        *var_qcfp3_dn0_slot = var_qcfp3_dn0;
        *var_qcfp3_dn1_slot = var_qcfp3_dn1;
        *var_qcfp3_dn10_slot = var_qcfp3_dn10;
        *var_qcfp3_dn11_slot = var_qcfp3_dn11;
        *var_qcfp3_dn12_slot = var_qcfp3_dn12;
        *var_qcfp3_dn13_slot = var_qcfp3_dn13;
        *var_qcfp3_dn14_slot = var_qcfp3_dn14;
        *var_qcfp3_dn15_slot = var_qcfp3_dn15;
        *var_qcfp3_dn16_slot = var_qcfp3_dn16;
        *var_qcfp3_dn17_slot = var_qcfp3_dn17;
        *var_qcfp3_dn18_slot = var_qcfp3_dn18;
        *var_qcfp3_dn19_slot = var_qcfp3_dn19;
        *var_qcfp3_dn2_slot = var_qcfp3_dn2;
        *var_qcfp3_dn20_slot = var_qcfp3_dn20;
        *var_qcfp3_dn21_slot = var_qcfp3_dn21;
        *var_qcfp3_dn22_slot = var_qcfp3_dn22;
        *var_qcfp3_dn23_slot = var_qcfp3_dn23;
        *var_qcfp3_dn24_slot = var_qcfp3_dn24;
        *var_qcfp3_dn25_slot = var_qcfp3_dn25;
        *var_qcfp3_dn26_slot = var_qcfp3_dn26;
        *var_qcfp3_dn27_slot = var_qcfp3_dn27;
        *var_qcfp3_dn28_slot = var_qcfp3_dn28;
        *var_qcfp3_dn29_slot = var_qcfp3_dn29;
        *var_qcfp3_dn3_slot = var_qcfp3_dn3;
        *var_qcfp3_dn4_slot = var_qcfp3_dn4;
        *var_qcfp3_dn5_slot = var_qcfp3_dn5;
        *var_qcfp3_dn6_slot = var_qcfp3_dn6;
        *var_qcfp3_dn7_slot = var_qcfp3_dn7;
        *var_qcfp3_dn8_slot = var_qcfp3_dn8;
        *var_qcfp3_dn9_slot = var_qcfp3_dn9;
        *var_qgdfp3_slot = var_qgdfp3;
        *var_qgdfp3_db0_slot = var_qgdfp3_db0;
        *var_qgdfp3_db1_slot = var_qgdfp3_db1;
        *var_qgdfp3_db10_slot = var_qgdfp3_db10;
        *var_qgdfp3_db11_slot = var_qgdfp3_db11;
        *var_qgdfp3_db12_slot = var_qgdfp3_db12;
        *var_qgdfp3_db13_slot = var_qgdfp3_db13;
        *var_qgdfp3_db14_slot = var_qgdfp3_db14;
        *var_qgdfp3_db15_slot = var_qgdfp3_db15;
        *var_qgdfp3_db16_slot = var_qgdfp3_db16;
        *var_qgdfp3_db17_slot = var_qgdfp3_db17;
        *var_qgdfp3_db18_slot = var_qgdfp3_db18;
        *var_qgdfp3_db19_slot = var_qgdfp3_db19;
        *var_qgdfp3_db2_slot = var_qgdfp3_db2;
        *var_qgdfp3_db20_slot = var_qgdfp3_db20;
        *var_qgdfp3_db21_slot = var_qgdfp3_db21;
        *var_qgdfp3_db22_slot = var_qgdfp3_db22;
        *var_qgdfp3_db23_slot = var_qgdfp3_db23;
        *var_qgdfp3_db24_slot = var_qgdfp3_db24;
        *var_qgdfp3_db25_slot = var_qgdfp3_db25;
        *var_qgdfp3_db26_slot = var_qgdfp3_db26;
        *var_qgdfp3_db27_slot = var_qgdfp3_db27;
        *var_qgdfp3_db28_slot = var_qgdfp3_db28;
        *var_qgdfp3_db29_slot = var_qgdfp3_db29;
        *var_qgdfp3_db3_slot = var_qgdfp3_db3;
        *var_qgdfp3_db30_slot = var_qgdfp3_db30;
        *var_qgdfp3_db31_slot = var_qgdfp3_db31;
        *var_qgdfp3_db32_slot = var_qgdfp3_db32;
        *var_qgdfp3_db33_slot = var_qgdfp3_db33;
        *var_qgdfp3_db34_slot = var_qgdfp3_db34;
        *var_qgdfp3_db35_slot = var_qgdfp3_db35;
        *var_qgdfp3_db4_slot = var_qgdfp3_db4;
        *var_qgdfp3_db5_slot = var_qgdfp3_db5;
        *var_qgdfp3_db6_slot = var_qgdfp3_db6;
        *var_qgdfp3_db7_slot = var_qgdfp3_db7;
        *var_qgdfp3_db8_slot = var_qgdfp3_db8;
        *var_qgdfp3_db9_slot = var_qgdfp3_db9;
        *var_qgdfp3_dn0_slot = var_qgdfp3_dn0;
        *var_qgdfp3_dn1_slot = var_qgdfp3_dn1;
        *var_qgdfp3_dn10_slot = var_qgdfp3_dn10;
        *var_qgdfp3_dn11_slot = var_qgdfp3_dn11;
        *var_qgdfp3_dn12_slot = var_qgdfp3_dn12;
        *var_qgdfp3_dn13_slot = var_qgdfp3_dn13;
        *var_qgdfp3_dn14_slot = var_qgdfp3_dn14;
        *var_qgdfp3_dn15_slot = var_qgdfp3_dn15;
        *var_qgdfp3_dn16_slot = var_qgdfp3_dn16;
        *var_qgdfp3_dn17_slot = var_qgdfp3_dn17;
        *var_qgdfp3_dn18_slot = var_qgdfp3_dn18;
        *var_qgdfp3_dn19_slot = var_qgdfp3_dn19;
        *var_qgdfp3_dn2_slot = var_qgdfp3_dn2;
        *var_qgdfp3_dn20_slot = var_qgdfp3_dn20;
        *var_qgdfp3_dn21_slot = var_qgdfp3_dn21;
        *var_qgdfp3_dn22_slot = var_qgdfp3_dn22;
        *var_qgdfp3_dn23_slot = var_qgdfp3_dn23;
        *var_qgdfp3_dn24_slot = var_qgdfp3_dn24;
        *var_qgdfp3_dn25_slot = var_qgdfp3_dn25;
        *var_qgdfp3_dn26_slot = var_qgdfp3_dn26;
        *var_qgdfp3_dn27_slot = var_qgdfp3_dn27;
        *var_qgdfp3_dn28_slot = var_qgdfp3_dn28;
        *var_qgdfp3_dn29_slot = var_qgdfp3_dn29;
        *var_qgdfp3_dn3_slot = var_qgdfp3_dn3;
        *var_qgdfp3_dn4_slot = var_qgdfp3_dn4;
        *var_qgdfp3_dn5_slot = var_qgdfp3_dn5;
        *var_qgdfp3_dn6_slot = var_qgdfp3_dn6;
        *var_qgdfp3_dn7_slot = var_qgdfp3_dn7;
        *var_qgdfp3_dn8_slot = var_qgdfp3_dn8;
        *var_qgdfp3_dn9_slot = var_qgdfp3_dn9;
        *var_qgsfp3_slot = var_qgsfp3;
        *var_qgsfp3_db0_slot = var_qgsfp3_db0;
        *var_qgsfp3_db1_slot = var_qgsfp3_db1;
        *var_qgsfp3_db10_slot = var_qgsfp3_db10;
        *var_qgsfp3_db11_slot = var_qgsfp3_db11;
        *var_qgsfp3_db12_slot = var_qgsfp3_db12;
        *var_qgsfp3_db13_slot = var_qgsfp3_db13;
        *var_qgsfp3_db14_slot = var_qgsfp3_db14;
        *var_qgsfp3_db15_slot = var_qgsfp3_db15;
        *var_qgsfp3_db16_slot = var_qgsfp3_db16;
        *var_qgsfp3_db17_slot = var_qgsfp3_db17;
        *var_qgsfp3_db18_slot = var_qgsfp3_db18;
        *var_qgsfp3_db19_slot = var_qgsfp3_db19;
        *var_qgsfp3_db2_slot = var_qgsfp3_db2;
        *var_qgsfp3_db20_slot = var_qgsfp3_db20;
        *var_qgsfp3_db21_slot = var_qgsfp3_db21;
        *var_qgsfp3_db22_slot = var_qgsfp3_db22;
        *var_qgsfp3_db23_slot = var_qgsfp3_db23;
        *var_qgsfp3_db24_slot = var_qgsfp3_db24;
        *var_qgsfp3_db25_slot = var_qgsfp3_db25;
        *var_qgsfp3_db26_slot = var_qgsfp3_db26;
        *var_qgsfp3_db27_slot = var_qgsfp3_db27;
        *var_qgsfp3_db28_slot = var_qgsfp3_db28;
        *var_qgsfp3_db29_slot = var_qgsfp3_db29;
        *var_qgsfp3_db3_slot = var_qgsfp3_db3;
        *var_qgsfp3_db30_slot = var_qgsfp3_db30;
        *var_qgsfp3_db31_slot = var_qgsfp3_db31;
        *var_qgsfp3_db32_slot = var_qgsfp3_db32;
        *var_qgsfp3_db33_slot = var_qgsfp3_db33;
        *var_qgsfp3_db34_slot = var_qgsfp3_db34;
        *var_qgsfp3_db35_slot = var_qgsfp3_db35;
        *var_qgsfp3_db4_slot = var_qgsfp3_db4;
        *var_qgsfp3_db5_slot = var_qgsfp3_db5;
        *var_qgsfp3_db6_slot = var_qgsfp3_db6;
        *var_qgsfp3_db7_slot = var_qgsfp3_db7;
        *var_qgsfp3_db8_slot = var_qgsfp3_db8;
        *var_qgsfp3_db9_slot = var_qgsfp3_db9;
        *var_qgsfp3_dn0_slot = var_qgsfp3_dn0;
        *var_qgsfp3_dn1_slot = var_qgsfp3_dn1;
        *var_qgsfp3_dn10_slot = var_qgsfp3_dn10;
        *var_qgsfp3_dn11_slot = var_qgsfp3_dn11;
        *var_qgsfp3_dn12_slot = var_qgsfp3_dn12;
        *var_qgsfp3_dn13_slot = var_qgsfp3_dn13;
        *var_qgsfp3_dn14_slot = var_qgsfp3_dn14;
        *var_qgsfp3_dn15_slot = var_qgsfp3_dn15;
        *var_qgsfp3_dn16_slot = var_qgsfp3_dn16;
        *var_qgsfp3_dn17_slot = var_qgsfp3_dn17;
        *var_qgsfp3_dn18_slot = var_qgsfp3_dn18;
        *var_qgsfp3_dn19_slot = var_qgsfp3_dn19;
        *var_qgsfp3_dn2_slot = var_qgsfp3_dn2;
        *var_qgsfp3_dn20_slot = var_qgsfp3_dn20;
        *var_qgsfp3_dn21_slot = var_qgsfp3_dn21;
        *var_qgsfp3_dn22_slot = var_qgsfp3_dn22;
        *var_qgsfp3_dn23_slot = var_qgsfp3_dn23;
        *var_qgsfp3_dn24_slot = var_qgsfp3_dn24;
        *var_qgsfp3_dn25_slot = var_qgsfp3_dn25;
        *var_qgsfp3_dn26_slot = var_qgsfp3_dn26;
        *var_qgsfp3_dn27_slot = var_qgsfp3_dn27;
        *var_qgsfp3_dn28_slot = var_qgsfp3_dn28;
        *var_qgsfp3_dn29_slot = var_qgsfp3_dn29;
        *var_qgsfp3_dn3_slot = var_qgsfp3_dn3;
        *var_qgsfp3_dn4_slot = var_qgsfp3_dn4;
        *var_qgsfp3_dn5_slot = var_qgsfp3_dn5;
        *var_qgsfp3_dn6_slot = var_qgsfp3_dn6;
        *var_qgsfp3_dn7_slot = var_qgsfp3_dn7;
        *var_qgsfp3_dn8_slot = var_qgsfp3_dn8;
        *var_qgsfp3_dn9_slot = var_qgsfp3_dn9;
        *var_qsfp3_slot = var_qsfp3;
        *var_qsfp3_db0_slot = var_qsfp3_db0;
        *var_qsfp3_db1_slot = var_qsfp3_db1;
        *var_qsfp3_db10_slot = var_qsfp3_db10;
        *var_qsfp3_db11_slot = var_qsfp3_db11;
        *var_qsfp3_db12_slot = var_qsfp3_db12;
        *var_qsfp3_db13_slot = var_qsfp3_db13;
        *var_qsfp3_db14_slot = var_qsfp3_db14;
        *var_qsfp3_db15_slot = var_qsfp3_db15;
        *var_qsfp3_db16_slot = var_qsfp3_db16;
        *var_qsfp3_db17_slot = var_qsfp3_db17;
        *var_qsfp3_db18_slot = var_qsfp3_db18;
        *var_qsfp3_db19_slot = var_qsfp3_db19;
        *var_qsfp3_db2_slot = var_qsfp3_db2;
        *var_qsfp3_db20_slot = var_qsfp3_db20;
        *var_qsfp3_db21_slot = var_qsfp3_db21;
        *var_qsfp3_db22_slot = var_qsfp3_db22;
        *var_qsfp3_db23_slot = var_qsfp3_db23;
        *var_qsfp3_db24_slot = var_qsfp3_db24;
        *var_qsfp3_db25_slot = var_qsfp3_db25;
        *var_qsfp3_db26_slot = var_qsfp3_db26;
        *var_qsfp3_db27_slot = var_qsfp3_db27;
        *var_qsfp3_db28_slot = var_qsfp3_db28;
        *var_qsfp3_db29_slot = var_qsfp3_db29;
        *var_qsfp3_db3_slot = var_qsfp3_db3;
        *var_qsfp3_db30_slot = var_qsfp3_db30;
        *var_qsfp3_db31_slot = var_qsfp3_db31;
        *var_qsfp3_db32_slot = var_qsfp3_db32;
        *var_qsfp3_db33_slot = var_qsfp3_db33;
        *var_qsfp3_db34_slot = var_qsfp3_db34;
        *var_qsfp3_db35_slot = var_qsfp3_db35;
        *var_qsfp3_db4_slot = var_qsfp3_db4;
        *var_qsfp3_db5_slot = var_qsfp3_db5;
        *var_qsfp3_db6_slot = var_qsfp3_db6;
        *var_qsfp3_db7_slot = var_qsfp3_db7;
        *var_qsfp3_db8_slot = var_qsfp3_db8;
        *var_qsfp3_db9_slot = var_qsfp3_db9;
        *var_qsfp3_dn0_slot = var_qsfp3_dn0;
        *var_qsfp3_dn1_slot = var_qsfp3_dn1;
        *var_qsfp3_dn10_slot = var_qsfp3_dn10;
        *var_qsfp3_dn11_slot = var_qsfp3_dn11;
        *var_qsfp3_dn12_slot = var_qsfp3_dn12;
        *var_qsfp3_dn13_slot = var_qsfp3_dn13;
        *var_qsfp3_dn14_slot = var_qsfp3_dn14;
        *var_qsfp3_dn15_slot = var_qsfp3_dn15;
        *var_qsfp3_dn16_slot = var_qsfp3_dn16;
        *var_qsfp3_dn17_slot = var_qsfp3_dn17;
        *var_qsfp3_dn18_slot = var_qsfp3_dn18;
        *var_qsfp3_dn19_slot = var_qsfp3_dn19;
        *var_qsfp3_dn2_slot = var_qsfp3_dn2;
        *var_qsfp3_dn20_slot = var_qsfp3_dn20;
        *var_qsfp3_dn21_slot = var_qsfp3_dn21;
        *var_qsfp3_dn22_slot = var_qsfp3_dn22;
        *var_qsfp3_dn23_slot = var_qsfp3_dn23;
        *var_qsfp3_dn24_slot = var_qsfp3_dn24;
        *var_qsfp3_dn25_slot = var_qsfp3_dn25;
        *var_qsfp3_dn26_slot = var_qsfp3_dn26;
        *var_qsfp3_dn27_slot = var_qsfp3_dn27;
        *var_qsfp3_dn28_slot = var_qsfp3_dn28;
        *var_qsfp3_dn29_slot = var_qsfp3_dn29;
        *var_qsfp3_dn3_slot = var_qsfp3_dn3;
        *var_qsfp3_dn4_slot = var_qsfp3_dn4;
        *var_qsfp3_dn5_slot = var_qsfp3_dn5;
        *var_qsfp3_dn6_slot = var_qsfp3_dn6;
        *var_qsfp3_dn7_slot = var_qsfp3_dn7;
        *var_qsfp3_dn8_slot = var_qsfp3_dn8;
        *var_qsfp3_dn9_slot = var_qsfp3_dn9;
    }

    pub(super) fn stamp_transient_block_83(
        s: &mut Scratch,
        p: &Parameters,
        var_fn61_calc_iq__return: f64,
        var_fn61_calc_iq__return_db0: f64,
        var_fn61_calc_iq__return_db1: f64,
        var_fn61_calc_iq__return_db10: f64,
        var_fn61_calc_iq__return_db11: f64,
        var_fn61_calc_iq__return_db12: f64,
        var_fn61_calc_iq__return_db13: f64,
        var_fn61_calc_iq__return_db14: f64,
        var_fn61_calc_iq__return_db15: f64,
        var_fn61_calc_iq__return_db16: f64,
        var_fn61_calc_iq__return_db17: f64,
        var_fn61_calc_iq__return_db18: f64,
        var_fn61_calc_iq__return_db19: f64,
        var_fn61_calc_iq__return_db2: f64,
        var_fn61_calc_iq__return_db20: f64,
        var_fn61_calc_iq__return_db21: f64,
        var_fn61_calc_iq__return_db22: f64,
        var_fn61_calc_iq__return_db23: f64,
        var_fn61_calc_iq__return_db24: f64,
        var_fn61_calc_iq__return_db25: f64,
        var_fn61_calc_iq__return_db26: f64,
        var_fn61_calc_iq__return_db27: f64,
        var_fn61_calc_iq__return_db28: f64,
        var_fn61_calc_iq__return_db29: f64,
        var_fn61_calc_iq__return_db3: f64,
        var_fn61_calc_iq__return_db30: f64,
        var_fn61_calc_iq__return_db31: f64,
        var_fn61_calc_iq__return_db32: f64,
        var_fn61_calc_iq__return_db33: f64,
        var_fn61_calc_iq__return_db34: f64,
        var_fn61_calc_iq__return_db35: f64,
        var_fn61_calc_iq__return_db4: f64,
        var_fn61_calc_iq__return_db5: f64,
        var_fn61_calc_iq__return_db6: f64,
        var_fn61_calc_iq__return_db7: f64,
        var_fn61_calc_iq__return_db8: f64,
        var_fn61_calc_iq__return_db9: f64,
        var_fn61_calc_iq__return_dn0: f64,
        var_fn61_calc_iq__return_dn1: f64,
        var_fn61_calc_iq__return_dn10: f64,
        var_fn61_calc_iq__return_dn11: f64,
        var_fn61_calc_iq__return_dn12: f64,
        var_fn61_calc_iq__return_dn13: f64,
        var_fn61_calc_iq__return_dn14: f64,
        var_fn61_calc_iq__return_dn15: f64,
        var_fn61_calc_iq__return_dn16: f64,
        var_fn61_calc_iq__return_dn17: f64,
        var_fn61_calc_iq__return_dn18: f64,
        var_fn61_calc_iq__return_dn19: f64,
        var_fn61_calc_iq__return_dn2: f64,
        var_fn61_calc_iq__return_dn20: f64,
        var_fn61_calc_iq__return_dn21: f64,
        var_fn61_calc_iq__return_dn22: f64,
        var_fn61_calc_iq__return_dn23: f64,
        var_fn61_calc_iq__return_dn24: f64,
        var_fn61_calc_iq__return_dn25: f64,
        var_fn61_calc_iq__return_dn26: f64,
        var_fn61_calc_iq__return_dn27: f64,
        var_fn61_calc_iq__return_dn28: f64,
        var_fn61_calc_iq__return_dn29: f64,
        var_fn61_calc_iq__return_dn3: f64,
        var_fn61_calc_iq__return_dn4: f64,
        var_fn61_calc_iq__return_dn5: f64,
        var_fn61_calc_iq__return_dn6: f64,
        var_fn61_calc_iq__return_dn7: f64,
        var_fn61_calc_iq__return_dn8: f64,
        var_fn61_calc_iq__return_dn9: f64,
        var_guard60: f64,
        var_tnomk: f64,
        var_guard95_slot: &mut f64,
        var_idsfp3_slot: &mut f64,
        var_idsfp3_db0_slot: &mut f64,
        var_idsfp3_db1_slot: &mut f64,
        var_idsfp3_db10_slot: &mut f64,
        var_idsfp3_db11_slot: &mut f64,
        var_idsfp3_db12_slot: &mut f64,
        var_idsfp3_db13_slot: &mut f64,
        var_idsfp3_db14_slot: &mut f64,
        var_idsfp3_db15_slot: &mut f64,
        var_idsfp3_db16_slot: &mut f64,
        var_idsfp3_db17_slot: &mut f64,
        var_idsfp3_db18_slot: &mut f64,
        var_idsfp3_db19_slot: &mut f64,
        var_idsfp3_db2_slot: &mut f64,
        var_idsfp3_db20_slot: &mut f64,
        var_idsfp3_db21_slot: &mut f64,
        var_idsfp3_db22_slot: &mut f64,
        var_idsfp3_db23_slot: &mut f64,
        var_idsfp3_db24_slot: &mut f64,
        var_idsfp3_db25_slot: &mut f64,
        var_idsfp3_db26_slot: &mut f64,
        var_idsfp3_db27_slot: &mut f64,
        var_idsfp3_db28_slot: &mut f64,
        var_idsfp3_db29_slot: &mut f64,
        var_idsfp3_db3_slot: &mut f64,
        var_idsfp3_db30_slot: &mut f64,
        var_idsfp3_db31_slot: &mut f64,
        var_idsfp3_db32_slot: &mut f64,
        var_idsfp3_db33_slot: &mut f64,
        var_idsfp3_db34_slot: &mut f64,
        var_idsfp3_db35_slot: &mut f64,
        var_idsfp3_db4_slot: &mut f64,
        var_idsfp3_db5_slot: &mut f64,
        var_idsfp3_db6_slot: &mut f64,
        var_idsfp3_db7_slot: &mut f64,
        var_idsfp3_db8_slot: &mut f64,
        var_idsfp3_db9_slot: &mut f64,
        var_idsfp3_dn0_slot: &mut f64,
        var_idsfp3_dn1_slot: &mut f64,
        var_idsfp3_dn10_slot: &mut f64,
        var_idsfp3_dn11_slot: &mut f64,
        var_idsfp3_dn12_slot: &mut f64,
        var_idsfp3_dn13_slot: &mut f64,
        var_idsfp3_dn14_slot: &mut f64,
        var_idsfp3_dn15_slot: &mut f64,
        var_idsfp3_dn16_slot: &mut f64,
        var_idsfp3_dn17_slot: &mut f64,
        var_idsfp3_dn18_slot: &mut f64,
        var_idsfp3_dn19_slot: &mut f64,
        var_idsfp3_dn2_slot: &mut f64,
        var_idsfp3_dn20_slot: &mut f64,
        var_idsfp3_dn21_slot: &mut f64,
        var_idsfp3_dn22_slot: &mut f64,
        var_idsfp3_dn23_slot: &mut f64,
        var_idsfp3_dn24_slot: &mut f64,
        var_idsfp3_dn25_slot: &mut f64,
        var_idsfp3_dn26_slot: &mut f64,
        var_idsfp3_dn27_slot: &mut f64,
        var_idsfp3_dn28_slot: &mut f64,
        var_idsfp3_dn29_slot: &mut f64,
        var_idsfp3_dn3_slot: &mut f64,
        var_idsfp3_dn4_slot: &mut f64,
        var_idsfp3_dn5_slot: &mut f64,
        var_idsfp3_dn6_slot: &mut f64,
        var_idsfp3_dn7_slot: &mut f64,
        var_idsfp3_dn8_slot: &mut f64,
        var_idsfp3_dn9_slot: &mut f64,
    ) {
        let mut var_guard95: f64 = *var_guard95_slot;
        let mut var_idsfp3: f64 = *var_idsfp3_slot;
        let mut var_idsfp3_db0: f64 = *var_idsfp3_db0_slot;
        let mut var_idsfp3_db1: f64 = *var_idsfp3_db1_slot;
        let mut var_idsfp3_db10: f64 = *var_idsfp3_db10_slot;
        let mut var_idsfp3_db11: f64 = *var_idsfp3_db11_slot;
        let mut var_idsfp3_db12: f64 = *var_idsfp3_db12_slot;
        let mut var_idsfp3_db13: f64 = *var_idsfp3_db13_slot;
        let mut var_idsfp3_db14: f64 = *var_idsfp3_db14_slot;
        let mut var_idsfp3_db15: f64 = *var_idsfp3_db15_slot;
        let mut var_idsfp3_db16: f64 = *var_idsfp3_db16_slot;
        let mut var_idsfp3_db17: f64 = *var_idsfp3_db17_slot;
        let mut var_idsfp3_db18: f64 = *var_idsfp3_db18_slot;
        let mut var_idsfp3_db19: f64 = *var_idsfp3_db19_slot;
        let mut var_idsfp3_db2: f64 = *var_idsfp3_db2_slot;
        let mut var_idsfp3_db20: f64 = *var_idsfp3_db20_slot;
        let mut var_idsfp3_db21: f64 = *var_idsfp3_db21_slot;
        let mut var_idsfp3_db22: f64 = *var_idsfp3_db22_slot;
        let mut var_idsfp3_db23: f64 = *var_idsfp3_db23_slot;
        let mut var_idsfp3_db24: f64 = *var_idsfp3_db24_slot;
        let mut var_idsfp3_db25: f64 = *var_idsfp3_db25_slot;
        let mut var_idsfp3_db26: f64 = *var_idsfp3_db26_slot;
        let mut var_idsfp3_db27: f64 = *var_idsfp3_db27_slot;
        let mut var_idsfp3_db28: f64 = *var_idsfp3_db28_slot;
        let mut var_idsfp3_db29: f64 = *var_idsfp3_db29_slot;
        let mut var_idsfp3_db3: f64 = *var_idsfp3_db3_slot;
        let mut var_idsfp3_db30: f64 = *var_idsfp3_db30_slot;
        let mut var_idsfp3_db31: f64 = *var_idsfp3_db31_slot;
        let mut var_idsfp3_db32: f64 = *var_idsfp3_db32_slot;
        let mut var_idsfp3_db33: f64 = *var_idsfp3_db33_slot;
        let mut var_idsfp3_db34: f64 = *var_idsfp3_db34_slot;
        let mut var_idsfp3_db35: f64 = *var_idsfp3_db35_slot;
        let mut var_idsfp3_db4: f64 = *var_idsfp3_db4_slot;
        let mut var_idsfp3_db5: f64 = *var_idsfp3_db5_slot;
        let mut var_idsfp3_db6: f64 = *var_idsfp3_db6_slot;
        let mut var_idsfp3_db7: f64 = *var_idsfp3_db7_slot;
        let mut var_idsfp3_db8: f64 = *var_idsfp3_db8_slot;
        let mut var_idsfp3_db9: f64 = *var_idsfp3_db9_slot;
        let mut var_idsfp3_dn0: f64 = *var_idsfp3_dn0_slot;
        let mut var_idsfp3_dn1: f64 = *var_idsfp3_dn1_slot;
        let mut var_idsfp3_dn10: f64 = *var_idsfp3_dn10_slot;
        let mut var_idsfp3_dn11: f64 = *var_idsfp3_dn11_slot;
        let mut var_idsfp3_dn12: f64 = *var_idsfp3_dn12_slot;
        let mut var_idsfp3_dn13: f64 = *var_idsfp3_dn13_slot;
        let mut var_idsfp3_dn14: f64 = *var_idsfp3_dn14_slot;
        let mut var_idsfp3_dn15: f64 = *var_idsfp3_dn15_slot;
        let mut var_idsfp3_dn16: f64 = *var_idsfp3_dn16_slot;
        let mut var_idsfp3_dn17: f64 = *var_idsfp3_dn17_slot;
        let mut var_idsfp3_dn18: f64 = *var_idsfp3_dn18_slot;
        let mut var_idsfp3_dn19: f64 = *var_idsfp3_dn19_slot;
        let mut var_idsfp3_dn2: f64 = *var_idsfp3_dn2_slot;
        let mut var_idsfp3_dn20: f64 = *var_idsfp3_dn20_slot;
        let mut var_idsfp3_dn21: f64 = *var_idsfp3_dn21_slot;
        let mut var_idsfp3_dn22: f64 = *var_idsfp3_dn22_slot;
        let mut var_idsfp3_dn23: f64 = *var_idsfp3_dn23_slot;
        let mut var_idsfp3_dn24: f64 = *var_idsfp3_dn24_slot;
        let mut var_idsfp3_dn25: f64 = *var_idsfp3_dn25_slot;
        let mut var_idsfp3_dn26: f64 = *var_idsfp3_dn26_slot;
        let mut var_idsfp3_dn27: f64 = *var_idsfp3_dn27_slot;
        let mut var_idsfp3_dn28: f64 = *var_idsfp3_dn28_slot;
        let mut var_idsfp3_dn29: f64 = *var_idsfp3_dn29_slot;
        let mut var_idsfp3_dn3: f64 = *var_idsfp3_dn3_slot;
        let mut var_idsfp3_dn4: f64 = *var_idsfp3_dn4_slot;
        let mut var_idsfp3_dn5: f64 = *var_idsfp3_dn5_slot;
        let mut var_idsfp3_dn6: f64 = *var_idsfp3_dn6_slot;
        let mut var_idsfp3_dn7: f64 = *var_idsfp3_dn7_slot;
        let mut var_idsfp3_dn8: f64 = *var_idsfp3_dn8_slot;
        let mut var_idsfp3_dn9: f64 = *var_idsfp3_dn9_slot;

        let (assign7330_e8719, assign7330_e8719_d_n0, assign7330_e8719_d_n1, assign7330_e8719_d_n2, assign7330_e8719_d_n3, assign7330_e8719_d_n4, assign7330_e8719_d_n5, assign7330_e8719_d_n6, assign7330_e8719_d_n7, assign7330_e8719_d_n8, assign7330_e8719_d_n9, assign7330_e8719_d_n10, assign7330_e8719_d_n11, assign7330_e8719_d_n12, assign7330_e8719_d_n13, assign7330_e8719_d_n14, assign7330_e8719_d_n15, assign7330_e8719_d_n16, assign7330_e8719_d_n17, assign7330_e8719_d_n18, assign7330_e8719_d_n19, assign7330_e8719_d_n20, assign7330_e8719_d_n21, assign7330_e8719_d_n22, assign7330_e8719_d_n23, assign7330_e8719_d_n24, assign7330_e8719_d_n25, assign7330_e8719_d_n26, assign7330_e8719_d_n27, assign7330_e8719_d_n28, assign7330_e8719_d_n29, assign7330_e8719_d_b0, assign7330_e8719_d_b1, assign7330_e8719_d_b2, assign7330_e8719_d_b3, assign7330_e8719_d_b4, assign7330_e8719_d_b5, assign7330_e8719_d_b6, assign7330_e8719_d_b7, assign7330_e8719_d_b8, assign7330_e8719_d_b9, assign7330_e8719_d_b10, assign7330_e8719_d_b11, assign7330_e8719_d_b12, assign7330_e8719_d_b13, assign7330_e8719_d_b14, assign7330_e8719_d_b15, assign7330_e8719_d_b16, assign7330_e8719_d_b17, assign7330_e8719_d_b18, assign7330_e8719_d_b19, assign7330_e8719_d_b20, assign7330_e8719_d_b21, assign7330_e8719_d_b22, assign7330_e8719_d_b23, assign7330_e8719_d_b24, assign7330_e8719_d_b25, assign7330_e8719_d_b26, assign7330_e8719_d_b27, assign7330_e8719_d_b28, assign7330_e8719_d_b29, assign7330_e8719_d_b30, assign7330_e8719_d_b31, assign7330_e8719_d_b32, assign7330_e8719_d_b33, assign7330_e8719_d_b34, assign7330_e8719_d_b35,) = {
    if (var_guard60 != 0.0) {
        (var_fn61_calc_iq__return, var_fn61_calc_iq__return_dn0, var_fn61_calc_iq__return_dn1, var_fn61_calc_iq__return_dn2, var_fn61_calc_iq__return_dn3, var_fn61_calc_iq__return_dn4, var_fn61_calc_iq__return_dn5, var_fn61_calc_iq__return_dn6, var_fn61_calc_iq__return_dn7, var_fn61_calc_iq__return_dn8, var_fn61_calc_iq__return_dn9, var_fn61_calc_iq__return_dn10, var_fn61_calc_iq__return_dn11, var_fn61_calc_iq__return_dn12, var_fn61_calc_iq__return_dn13, var_fn61_calc_iq__return_dn14, var_fn61_calc_iq__return_dn15, var_fn61_calc_iq__return_dn16, var_fn61_calc_iq__return_dn17, var_fn61_calc_iq__return_dn18, var_fn61_calc_iq__return_dn19, var_fn61_calc_iq__return_dn20, var_fn61_calc_iq__return_dn21, var_fn61_calc_iq__return_dn22, var_fn61_calc_iq__return_dn23, var_fn61_calc_iq__return_dn24, var_fn61_calc_iq__return_dn25, var_fn61_calc_iq__return_dn26, var_fn61_calc_iq__return_dn27, var_fn61_calc_iq__return_dn28, var_fn61_calc_iq__return_dn29, var_fn61_calc_iq__return_db0, var_fn61_calc_iq__return_db1, var_fn61_calc_iq__return_db2, var_fn61_calc_iq__return_db3, var_fn61_calc_iq__return_db4, var_fn61_calc_iq__return_db5, var_fn61_calc_iq__return_db6, var_fn61_calc_iq__return_db7, var_fn61_calc_iq__return_db8, var_fn61_calc_iq__return_db9, var_fn61_calc_iq__return_db10, var_fn61_calc_iq__return_db11, var_fn61_calc_iq__return_db12, var_fn61_calc_iq__return_db13, var_fn61_calc_iq__return_db14, var_fn61_calc_iq__return_db15, var_fn61_calc_iq__return_db16, var_fn61_calc_iq__return_db17, var_fn61_calc_iq__return_db18, var_fn61_calc_iq__return_db19, var_fn61_calc_iq__return_db20, var_fn61_calc_iq__return_db21, var_fn61_calc_iq__return_db22, var_fn61_calc_iq__return_db23, var_fn61_calc_iq__return_db24, var_fn61_calc_iq__return_db25, var_fn61_calc_iq__return_db26, var_fn61_calc_iq__return_db27, var_fn61_calc_iq__return_db28, var_fn61_calc_iq__return_db29, var_fn61_calc_iq__return_db30, var_fn61_calc_iq__return_db31, var_fn61_calc_iq__return_db32, var_fn61_calc_iq__return_db33, var_fn61_calc_iq__return_db34, var_fn61_calc_iq__return_db35,)
    } else {
        (var_idsfp3, var_idsfp3_dn0, var_idsfp3_dn1, var_idsfp3_dn2, var_idsfp3_dn3, var_idsfp3_dn4, var_idsfp3_dn5, var_idsfp3_dn6, var_idsfp3_dn7, var_idsfp3_dn8, var_idsfp3_dn9, var_idsfp3_dn10, var_idsfp3_dn11, var_idsfp3_dn12, var_idsfp3_dn13, var_idsfp3_dn14, var_idsfp3_dn15, var_idsfp3_dn16, var_idsfp3_dn17, var_idsfp3_dn18, var_idsfp3_dn19, var_idsfp3_dn20, var_idsfp3_dn21, var_idsfp3_dn22, var_idsfp3_dn23, var_idsfp3_dn24, var_idsfp3_dn25, var_idsfp3_dn26, var_idsfp3_dn27, var_idsfp3_dn28, var_idsfp3_dn29, var_idsfp3_db0, var_idsfp3_db1, var_idsfp3_db2, var_idsfp3_db3, var_idsfp3_db4, var_idsfp3_db5, var_idsfp3_db6, var_idsfp3_db7, var_idsfp3_db8, var_idsfp3_db9, var_idsfp3_db10, var_idsfp3_db11, var_idsfp3_db12, var_idsfp3_db13, var_idsfp3_db14, var_idsfp3_db15, var_idsfp3_db16, var_idsfp3_db17, var_idsfp3_db18, var_idsfp3_db19, var_idsfp3_db20, var_idsfp3_db21, var_idsfp3_db22, var_idsfp3_db23, var_idsfp3_db24, var_idsfp3_db25, var_idsfp3_db26, var_idsfp3_db27, var_idsfp3_db28, var_idsfp3_db29, var_idsfp3_db30, var_idsfp3_db31, var_idsfp3_db32, var_idsfp3_db33, var_idsfp3_db34, var_idsfp3_db35,)
    }
};
        var_idsfp3 = assign7330_e8719;
        var_idsfp3_dn0 = assign7330_e8719_d_n0;
        var_idsfp3_dn1 = assign7330_e8719_d_n1;
        var_idsfp3_dn2 = assign7330_e8719_d_n2;
        var_idsfp3_dn3 = assign7330_e8719_d_n3;
        var_idsfp3_dn4 = assign7330_e8719_d_n4;
        var_idsfp3_dn5 = assign7330_e8719_d_n5;
        var_idsfp3_dn6 = assign7330_e8719_d_n6;
        var_idsfp3_dn7 = assign7330_e8719_d_n7;
        var_idsfp3_dn8 = assign7330_e8719_d_n8;
        var_idsfp3_dn9 = assign7330_e8719_d_n9;
        var_idsfp3_dn10 = assign7330_e8719_d_n10;
        var_idsfp3_dn11 = assign7330_e8719_d_n11;
        var_idsfp3_dn12 = assign7330_e8719_d_n12;
        var_idsfp3_dn13 = assign7330_e8719_d_n13;
        var_idsfp3_dn14 = assign7330_e8719_d_n14;
        var_idsfp3_dn15 = assign7330_e8719_d_n15;
        var_idsfp3_dn16 = assign7330_e8719_d_n16;
        var_idsfp3_dn17 = assign7330_e8719_d_n17;
        var_idsfp3_dn18 = assign7330_e8719_d_n18;
        var_idsfp3_dn19 = assign7330_e8719_d_n19;
        var_idsfp3_dn20 = assign7330_e8719_d_n20;
        var_idsfp3_dn21 = assign7330_e8719_d_n21;
        var_idsfp3_dn22 = assign7330_e8719_d_n22;
        var_idsfp3_dn23 = assign7330_e8719_d_n23;
        var_idsfp3_dn24 = assign7330_e8719_d_n24;
        var_idsfp3_dn25 = assign7330_e8719_d_n25;
        var_idsfp3_dn26 = assign7330_e8719_d_n26;
        var_idsfp3_dn27 = assign7330_e8719_d_n27;
        var_idsfp3_dn28 = assign7330_e8719_d_n28;
        var_idsfp3_dn29 = assign7330_e8719_d_n29;
        var_idsfp3_db0 = assign7330_e8719_d_b0;
        var_idsfp3_db1 = assign7330_e8719_d_b1;
        var_idsfp3_db2 = assign7330_e8719_d_b2;
        var_idsfp3_db3 = assign7330_e8719_d_b3;
        var_idsfp3_db4 = assign7330_e8719_d_b4;
        var_idsfp3_db5 = assign7330_e8719_d_b5;
        var_idsfp3_db6 = assign7330_e8719_d_b6;
        var_idsfp3_db7 = assign7330_e8719_d_b7;
        var_idsfp3_db8 = assign7330_e8719_d_b8;
        var_idsfp3_db9 = assign7330_e8719_d_b9;
        var_idsfp3_db10 = assign7330_e8719_d_b10;
        var_idsfp3_db11 = assign7330_e8719_d_b11;
        var_idsfp3_db12 = assign7330_e8719_d_b12;
        var_idsfp3_db13 = assign7330_e8719_d_b13;
        var_idsfp3_db14 = assign7330_e8719_d_b14;
        var_idsfp3_db15 = assign7330_e8719_d_b15;
        var_idsfp3_db16 = assign7330_e8719_d_b16;
        var_idsfp3_db17 = assign7330_e8719_d_b17;
        var_idsfp3_db18 = assign7330_e8719_d_b18;
        var_idsfp3_db19 = assign7330_e8719_d_b19;
        var_idsfp3_db20 = assign7330_e8719_d_b20;
        var_idsfp3_db21 = assign7330_e8719_d_b21;
        var_idsfp3_db22 = assign7330_e8719_d_b22;
        var_idsfp3_db23 = assign7330_e8719_d_b23;
        var_idsfp3_db24 = assign7330_e8719_d_b24;
        var_idsfp3_db25 = assign7330_e8719_d_b25;
        var_idsfp3_db26 = assign7330_e8719_d_b26;
        var_idsfp3_db27 = assign7330_e8719_d_b27;
        var_idsfp3_db28 = assign7330_e8719_d_b28;
        var_idsfp3_db29 = assign7330_e8719_d_b29;
        var_idsfp3_db30 = assign7330_e8719_d_b30;
        var_idsfp3_db31 = assign7330_e8719_d_b31;
        var_idsfp3_db32 = assign7330_e8719_d_b32;
        var_idsfp3_db33 = assign7330_e8719_d_b33;
        var_idsfp3_db34 = assign7330_e8719_d_b34;
        var_idsfp3_db35 = assign7330_e8719_d_b35;

        let assign7340_e8722: f64 = if p.p210 == 1.0 { 1.0 } else { 0.0 };
        var_guard95 = assign7340_e8722;

        s.store_scalar(196, 0.0);

        s.store_scalar(197, 0.0);

        s.store_scalar(198, 0.0);

        s.store_scalar(199, 0.0);

        s.store_scalar(200, 0.0);

        s.store_scalar(201, 0.0);

        s.b[614] = (p.p189 > p.p354);
        s.store_scalar(614, if s.b[614] { 1.0 } else { 0.0 });

        if s.b[614] {
            s.store_scalar(615, 0.0);
            s.store_scalar(616, 0.0);
            s.store_scalar(617, 0.0);
            s.store_scalar(618, 0.0);
            s.store_scalar(619, 0.0);
            s.store_scalar(620, 0.0);
            s.store_scalar(621, 0.0);
            s.store_scalar(622, 0.0);
            s.store_scalar(623, 0.0);
            s.copy_ad(624, 90);
            s.copy_ad(625, 91);
        }

        let (assign7550_e8781,) = {
    if s.b[614] {
        (p.p195,)
    } else {
        (s.v[626],)
    }
};
        s.store_scalar(626, assign7550_e8781);

        if s.b[614] {
            s.copy_ad(627, 92);
            s.copy_ad(628, 93);
        }

        let (assign7580_e8793,) = {
    if s.b[614] {
        (p.p193,)
    } else {
        (s.v[629],)
    }
};
        s.store_scalar(629, assign7580_e8793);

        if s.b[614] {
            s.copy_ad(630, 111);
            s.store_scalar(631, var_tnomk);
            s.copy_ad(632, 113);
            s.store_scalar(633, p.p0);
            s.store_scalar(634, p.p189);
            s.copy_ad(635, 35);
            s.store_scalar(636, p.p194);
            s.copy_ad(637, 36);
            s.copy_ad(638, 37);
            s.store_scalar(639, p.p190);
            s.store_scalar(640, p.p204);
            s.store_scalar(641, p.p203);
            s.store_scalar(642, 0.0);
            s.store_scalar(643, p.p205);
            s.store_scalar(644, p.p209);
            s.store_scalar(645, p.p200);
            s.store_scalar(646, p.p201);
            s.store_scalar(647, p.p202);
            s.store_scalar(648, p.p208);
            s.store_scalar(649, p.p207);
            s.store_scalar(650, p.p206);
            s.store_scalar(651, p.p39);
            s.store_scalar(652, p.p47);
            s.store_scalar(653, p.p45);
            s.store_scalar(654, p.p42);
            s.store_scalar(655, p.p2);
            s.store_scalar(656, p.p6);
            s.store_scalar(657, 1.0);
            s.store_scalar(658, 0.0);
            s.store_scalar(659, 0.0);
            s.store_scalar(660, 0.0);
            s.store_scalar(661, 0.0);
            s.store_scalar(662, 0.0);
            s.store_scalar(663, 0.0);
            s.store_scalar(664, 0.0);
            s.store_scalar(665, 0.0);
            s.store_scalar(666, 0.0);
            s.store_scalar(667, 0.0);
            s.store_scalar(668, 0.0);
            s.store_scalar(669, 0.0);
            s.store_scalar(670, 0.0);
            s.store_scalar(671, 0.0);
            s.store_scalar(672, 0.0);
            s.store_scalar(673, 0.0);
            s.store_scalar(674, 0.0);
            s.store_scalar(675, 0.0);
            s.store_scalar(676, 0.0);
            s.store_scalar(677, 0.0);
            s.store_scalar(678, 0.0);
            s.store_scalar(679, 0.0);
            s.store_scalar(680, 0.0);
            s.store_scalar(681, 0.0);
            s.store_scalar(682, 0.0);
            s.store_scalar(683, 0.0);
            s.store_scalar(684, 0.0);
            s.store_scalar(685, 0.0);
            s.store_scalar(686, 0.0);
            s.store_scalar(687, 0.0);
            s.store_scalar(688, 0.0);
            s.store_scalar(689, 0.0);
            s.store_scalar(690, 0.0);
            s.store_scalar(691, 0.0);
            s.store_scalar(692, 0.0);
            s.store_scalar(693, 0.0);
            s.store_scalar(694, 0.0);
            s.store_scalar(695, 0.0);
            s.store_scalar(696, 0.0);
            s.store_scalar(697, 0.0);
            s.store_scalar(698, 0.0);
            s.store_scalar(699, 0.0);
            s.store_scalar(700, 0.0);
            s.store_scalar(701, 0.0);
            s.store_scalar(702, 0.0);
            s.store_scalar(703, 0.0);
            s.store_scalar(704, 0.0);
            s.store_scalar(705, 0.0);
            s.store_scalar(706, 0.0);
            s.store_scalar(707, 0.0);
            s.store_scalar(708, 0.0);
            s.store_scalar(709, 0.0);
            s.store_scalar(710, 0.0);
            s.store_scalar(711, 0.0);
            s.store_scalar(712, 0.0);
            s.store_scalar(713, 0.0);
            s.store_scalar(714, 0.0);
            s.store_scalar(715, 0.0);
            s.store_scalar(716, 0.0);
        }


        *var_guard95_slot = var_guard95;
        *var_idsfp3_slot = var_idsfp3;
        *var_idsfp3_db0_slot = var_idsfp3_db0;
        *var_idsfp3_db1_slot = var_idsfp3_db1;
        *var_idsfp3_db10_slot = var_idsfp3_db10;
        *var_idsfp3_db11_slot = var_idsfp3_db11;
        *var_idsfp3_db12_slot = var_idsfp3_db12;
        *var_idsfp3_db13_slot = var_idsfp3_db13;
        *var_idsfp3_db14_slot = var_idsfp3_db14;
        *var_idsfp3_db15_slot = var_idsfp3_db15;
        *var_idsfp3_db16_slot = var_idsfp3_db16;
        *var_idsfp3_db17_slot = var_idsfp3_db17;
        *var_idsfp3_db18_slot = var_idsfp3_db18;
        *var_idsfp3_db19_slot = var_idsfp3_db19;
        *var_idsfp3_db2_slot = var_idsfp3_db2;
        *var_idsfp3_db20_slot = var_idsfp3_db20;
        *var_idsfp3_db21_slot = var_idsfp3_db21;
        *var_idsfp3_db22_slot = var_idsfp3_db22;
        *var_idsfp3_db23_slot = var_idsfp3_db23;
        *var_idsfp3_db24_slot = var_idsfp3_db24;
        *var_idsfp3_db25_slot = var_idsfp3_db25;
        *var_idsfp3_db26_slot = var_idsfp3_db26;
        *var_idsfp3_db27_slot = var_idsfp3_db27;
        *var_idsfp3_db28_slot = var_idsfp3_db28;
        *var_idsfp3_db29_slot = var_idsfp3_db29;
        *var_idsfp3_db3_slot = var_idsfp3_db3;
        *var_idsfp3_db30_slot = var_idsfp3_db30;
        *var_idsfp3_db31_slot = var_idsfp3_db31;
        *var_idsfp3_db32_slot = var_idsfp3_db32;
        *var_idsfp3_db33_slot = var_idsfp3_db33;
        *var_idsfp3_db34_slot = var_idsfp3_db34;
        *var_idsfp3_db35_slot = var_idsfp3_db35;
        *var_idsfp3_db4_slot = var_idsfp3_db4;
        *var_idsfp3_db5_slot = var_idsfp3_db5;
        *var_idsfp3_db6_slot = var_idsfp3_db6;
        *var_idsfp3_db7_slot = var_idsfp3_db7;
        *var_idsfp3_db8_slot = var_idsfp3_db8;
        *var_idsfp3_db9_slot = var_idsfp3_db9;
        *var_idsfp3_dn0_slot = var_idsfp3_dn0;
        *var_idsfp3_dn1_slot = var_idsfp3_dn1;
        *var_idsfp3_dn10_slot = var_idsfp3_dn10;
        *var_idsfp3_dn11_slot = var_idsfp3_dn11;
        *var_idsfp3_dn12_slot = var_idsfp3_dn12;
        *var_idsfp3_dn13_slot = var_idsfp3_dn13;
        *var_idsfp3_dn14_slot = var_idsfp3_dn14;
        *var_idsfp3_dn15_slot = var_idsfp3_dn15;
        *var_idsfp3_dn16_slot = var_idsfp3_dn16;
        *var_idsfp3_dn17_slot = var_idsfp3_dn17;
        *var_idsfp3_dn18_slot = var_idsfp3_dn18;
        *var_idsfp3_dn19_slot = var_idsfp3_dn19;
        *var_idsfp3_dn2_slot = var_idsfp3_dn2;
        *var_idsfp3_dn20_slot = var_idsfp3_dn20;
        *var_idsfp3_dn21_slot = var_idsfp3_dn21;
        *var_idsfp3_dn22_slot = var_idsfp3_dn22;
        *var_idsfp3_dn23_slot = var_idsfp3_dn23;
        *var_idsfp3_dn24_slot = var_idsfp3_dn24;
        *var_idsfp3_dn25_slot = var_idsfp3_dn25;
        *var_idsfp3_dn26_slot = var_idsfp3_dn26;
        *var_idsfp3_dn27_slot = var_idsfp3_dn27;
        *var_idsfp3_dn28_slot = var_idsfp3_dn28;
        *var_idsfp3_dn29_slot = var_idsfp3_dn29;
        *var_idsfp3_dn3_slot = var_idsfp3_dn3;
        *var_idsfp3_dn4_slot = var_idsfp3_dn4;
        *var_idsfp3_dn5_slot = var_idsfp3_dn5;
        *var_idsfp3_dn6_slot = var_idsfp3_dn6;
        *var_idsfp3_dn7_slot = var_idsfp3_dn7;
        *var_idsfp3_dn8_slot = var_idsfp3_dn8;
        *var_idsfp3_dn9_slot = var_idsfp3_dn9;
    }

    pub(super) fn stamp_transient_block_84(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[614] {
            s.store_scalar(717, 0.0);
            s.store_scalar(718, 0.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(721, 0.0);
            s.store_scalar(722, 0.0);
            s.store_scalar(723, 0.0);
            s.store_scalar(724, 0.0);
            s.store_scalar(725, 0.0);
            s.store_scalar(726, 0.0);
        }

        if s.b[614] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(723, 625, A::tanh_scaled_input(s.ad_value(625), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(723, 625, p.p53);
                } else {
                    s.store_scalar(723, 0.0);
                }
            }
        }

        if s.b[614] {
            s.store_sub(724, 624, 625);
            s.store_mul(658, 644, 632);
            s.store_add_scaled_product_value_ad(660, A::div_scaled_inputs(s.ad_value(640), 1.0, s.ad_value(632), 2.302585092994046), 1.0, 643, 723, 1.0);
            s.store_add_scaled_product_right_sub(661, 639, 1.0, 650, 630, 631, 1.0);
            s.store_pow_ad(679, A::div(s.ad_value(630), s.ad_value(631)), s.ad_value(652));
        }

        s.b[727] = (s.v[651] != 0.0);
        s.store_scalar(727, if s.b[727] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[727]) {
            s.store_div_ad_rhs(662, 723, A::pow(A::offset(A::pow(A::div(s.ad_value(723), s.ad_value(651)), s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647))));
        }

        if (s.b[614] && (!s.b[727])) {
            s.store_scalar(662, 0.0);
        }

        if s.b[614] {
            s.store_mul_add_scaled_product_rhs(659, 723, s.ad_value(641), 1.0, s.ad_value(662), s.ad_value(642), (-1.0));
            s.store_sub(622, 661, 659);
            s.store_scaled_mul(664, 660, 632, 2.0);
            s.store_mul(665, 635, 664);
            s.store_sub_scaled_inputs(722, 622, 1.0, 658, (p.p51 * 0.5));
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_mixed_aii(721, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(624), s.ad_value(724)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 722, (-1.0), 658, 1.0);
        }

        s.b[728] = (s.v[721] > 50.0);
        s.store_scalar(728, if s.b[728] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[728]) {
            s.store_scalar(680, 0.0);
        }

        s.b[729] = (s.v[721] < (-50.0));
        s.store_scalar(729, if s.b[729] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[728])) && s.b[729]) {
            s.store_scalar(680, 1.0);
        }

        if ((s.b[614] && (!s.b[728])) && (!s.b[729])) {
            s.store_div_from_scalar_offset_ad(680, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_mixed_aai(681, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(624), s.ad_value(724)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(680), (-(p.p51 * 0.1))), (-1.0), 664, 1.0);
        }

        s.b[730] = (s.v[681] > 50.0);
        s.store_scalar(730, if s.b[730] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[730]) {
            s.store_mul(682, 665, 681);
        }

        s.b[731] = (s.v[681] < (-50.0));
        s.store_scalar(731, if s.b[731] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[730])) && s.b[731]) {
            s.store_mul_exp_rhs(682, 665, 681);
        }

        if ((s.b[614] && (!s.b[730])) && (!s.b[731])) {
            s.store_mul_ln_one_plus_exp_rhs(682, 665, 681);
        }

        if s.b[614] {
            s.store_div_ad_rhs(668, 646, A::mul_offset_rhs(s.ad_value(679), A::div_scaled_product(s.ad_value(648), s.ad_value(682), 1.0, s.ad_value(635), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(669, 645, A::div_scaled_offset_numerator(A::mul(s.ad_value(653), s.ad_value(631)), 1.0, 1.0, A::offset(A::mul(s.ad_value(653), s.ad_value(630)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(654), s.ad_value(723), 1.0, s.ad_value(634), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(649), s.ad_value(682), 1.0, s.ad_value(635), 1.0), 1.0), 1.0);
            s.store_add_ad(670, A::div_scaled_product3(s.ad_value(680), s.ad_value(632), s.ad_value(668), 2.0, s.ad_value(634), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(680), s.ad_value(669)));
            s.store_div_scaled_product_indices(686, 669, 634, 1.0, 668, 1.0);
            s.store_add_scaled_product_right_ad(687, 686, (-1.0), 686, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(682), 2.0, s.ad_value(635), s.ad_value(686), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(688, A::mul_sub_from_scalar_rhs(s.ad_value(686), 1.0, s.ad_value(680)), 1.0, 664, 680, 1.0);
            s.store_add_scaled_product_value_ad(623, A::mul_sub_from_scalar_rhs(s.ad_value(687), 1.0, s.ad_value(680)), 1.0, 664, 680, 1.0);
        }

        if s.b[614] {
            s.store_div_from_scalar_pow_ad(689, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(625), s.ad_value(623)), 0.5, A::div(s.ad_value(625), s.ad_value(623)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(625), s.ad_value(623))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(625), s.ad_value(623)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(625), s.ad_value(623))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul(690, 625, 689);
        }

        if s.b[614] {
            s.store_div_from_scalar_pow_ad(691, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul_neg_lhs(692, 625, 691);
            s.store_div_scaled_inputs2_indices(721, 624, 1.0, 722, (-1.0), 658, 1.0);
        }

        s.b[732] = (s.v[721] > 50.0);
        s.store_scalar(732, if s.b[732] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[732]) {
            s.store_scalar(663, 0.0);
        }

        s.b[733] = (s.v[721] < (-50.0));
        s.store_scalar(733, if s.b[733] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[732])) && s.b[733]) {
            s.store_scalar(663, 1.0);
        }

        if ((s.b[614] && (!s.b[732])) && (!s.b[733])) {
            s.store_div_from_scalar_offset_ad(663, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3_mixed_iiai(666, 724, 1.0, 692, (-1.0), A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(663), (-(p.p51 * 0.1))), -1.0, 664, 1.0);
        }

        s.b[734] = (s.v[666] > 50.0);
        s.store_scalar(734, if s.b[734] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[734]) {
            s.store_mul(667, 665, 666);
        }

        s.b[735] = (s.v[666] < (-50.0));
        s.store_scalar(735, if s.b[735] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[734])) && s.b[735]) {
            s.store_mul_exp_rhs(667, 665, 666);
        }

        if ((s.b[614] && (!s.b[734])) && (!s.b[735])) {
            s.store_mul_ln_one_plus_exp_rhs(667, 665, 666);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_indices(721, 724, 1.0, 722, (-1.0), 658, 1.0);
        }

        s.b[736] = (s.v[721] > 50.0);
        s.store_scalar(736, if s.b[736] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[736]) {
            s.store_scalar(693, 0.0);
        }

        s.b[737] = (s.v[721] < (-50.0));
        s.store_scalar(737, if s.b[737] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[736])) && s.b[737]) {
            s.store_scalar(693, 1.0);
        }

        if ((s.b[614] && (!s.b[736])) && (!s.b[737])) {
            s.store_div_from_scalar_offset_ad(693, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3_mixed_iiai(694, 624, 1.0, 690, (-1.0), A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(693), (-(p.p51 * 0.1))), -1.0, 664, 1.0);
        }

        s.b[738] = (s.v[694] > 50.0);
        s.store_scalar(738, if s.b[738] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[738]) {
            s.store_mul(695, 665, 694);
        }

        s.b[739] = (s.v[694] < (-50.0));
        s.store_scalar(739, if s.b[739] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[738])) && s.b[739]) {
            s.store_mul_exp_rhs(695, 665, 694);
        }

        if ((s.b[614] && (!s.b[738])) && (!s.b[739])) {
            s.store_mul_ln_one_plus_exp_rhs(695, 665, 694);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_indices(696, 667, 1.0, 695, (-1.0), 635, 1.0);
            s.store_div(722, 696, 688);
        }

        if s.b[614] {
            s.store_div_ad_rhs(697, 722, A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(722), A::tanh_scaled_input(s.ad_value(722), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(s.ad_value(722), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647))));
        }

        if s.b[614] {
            s.store_mul(698, 670, 697);
            s.store_mul_product3_mixed_iaai(616, 657, A::mul3(s.ad_value(656), s.ad_value(633), s.ad_value(655)), A::add(s.ad_value(667), s.ad_value(695)), 698, 0.5);
            s.store_div_scaled_inputs_indices(671, 640, 1.0, 632, 2.302585092994046);
            s.store_scaled_mul(673, 671, 632, 2.0);
            s.store_mul(674, 635, 673);
            s.store_sub_scaled_inputs(726, 661, 1.0, 658, (p.p51 * 0.5));
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_mixed_aii(725, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(624), s.ad_value(724)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 726, (-1.0), 658, 1.0);
        }

        s.b[740] = (s.v[725] > 50.0);
        s.store_scalar(740, if s.b[740] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[740]) {
            s.store_scalar(683, 0.0);
        }

        s.b[741] = (s.v[725] < (-50.0));
        s.store_scalar(741, if s.b[741] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[740])) && s.b[741]) {
            s.store_scalar(683, 1.0);
        }

        if ((s.b[614] && (!s.b[740])) && (!s.b[741])) {
            s.store_div_from_scalar_offset_ad(683, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_mixed_aai(684, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(624), s.ad_value(724)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(683), (-(p.p51 * 0.1))), (-1.0), 673, 1.0);
        }

        s.b[742] = (s.v[684] > 50.0);
        s.store_scalar(742, if s.b[742] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[742]) {
            s.store_mul(685, 674, 684);
        }

        s.b[743] = (s.v[684] < (-50.0));
        s.store_scalar(743, if s.b[743] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[742])) && s.b[743]) {
            s.store_mul_exp_rhs(685, 674, 684);
        }

        if ((s.b[614] && (!s.b[742])) && (!s.b[743])) {
            s.store_mul_ln_one_plus_exp_rhs(685, 674, 684);
        }

        if s.b[614] {
            s.store_div(677, 646, 679);
            s.store_mul_div_scaled_offset_numerator_rhs(678, 645, A::mul(s.ad_value(653), s.ad_value(631)), 1.0, 1.0, A::offset(A::mul(s.ad_value(653), s.ad_value(630)), 1.0), 1.0);
            s.store_div_scaled_product_indices(699, 678, 634, 1.0, 677, 1.0);
            s.store_add_scaled_product_right_ad(700, 699, (-1.0), 699, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(685), 2.0, s.ad_value(635), s.ad_value(699), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(701, A::mul_sub_from_scalar_rhs(s.ad_value(700), 1.0, s.ad_value(683)), 1.0, 673, 683, 1.0);
        }

        if s.b[614] {
            s.store_div_from_scalar_pow_ad(702, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(625), s.ad_value(701)), 0.5, A::div(s.ad_value(625), s.ad_value(701)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(625), s.ad_value(701))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(625), s.ad_value(701)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(625), s.ad_value(701))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul(703, 625, 702);
        }

        if s.b[614] {
            s.store_div_from_scalar_pow_ad(704, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul_neg_lhs(705, 625, 704);
            s.store_div_scaled_inputs2_indices(725, 624, 1.0, 726, (-1.0), 658, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_85(
        s: &mut Scratch,
        p: &Parameters,
        var_tnomk: f64,
    ) {
        s.b[744] = (s.v[725] > 50.0);
        s.store_scalar(744, if s.b[744] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[744]) {
            s.store_scalar(672, 0.0);
        }

        s.b[745] = (s.v[725] < (-50.0));
        s.store_scalar(745, if s.b[745] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[744])) && s.b[745]) {
            s.store_scalar(672, 1.0);
        }

        if ((s.b[614] && (!s.b[744])) && (!s.b[745])) {
            s.store_div_from_scalar_offset_ad(672, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3_mixed_iiai(675, 724, 1.0, 705, (-1.0), A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(672), (-(p.p51 * 0.1))), -1.0, 673, 1.0);
        }

        s.b[746] = (s.v[675] > 50.0);
        s.store_scalar(746, if s.b[746] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[746]) {
            s.store_mul(676, 674, 675);
        }

        s.b[747] = (s.v[675] < (-50.0));
        s.store_scalar(747, if s.b[747] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[746])) && s.b[747]) {
            s.store_mul_exp_rhs(676, 674, 675);
        }

        if ((s.b[614] && (!s.b[746])) && (!s.b[747])) {
            s.store_mul_ln_one_plus_exp_rhs(676, 674, 675);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_indices(725, 724, 1.0, 726, (-1.0), 658, 1.0);
        }

        s.b[748] = (s.v[725] > 50.0);
        s.store_scalar(748, if s.b[748] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[748]) {
            s.store_scalar(706, 0.0);
        }

        s.b[749] = (s.v[725] < (-50.0));
        s.store_scalar(749, if s.b[749] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[748])) && s.b[749]) {
            s.store_scalar(706, 1.0);
        }

        if ((s.b[614] && (!s.b[748])) && (!s.b[749])) {
            s.store_div_from_scalar_offset_ad(706, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3_mixed_iiai(707, 624, 1.0, 703, (-1.0), A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(706), (-(p.p51 * 0.1))), -1.0, 673, 1.0);
        }

        s.b[750] = (s.v[707] > 50.0);
        s.store_scalar(750, if s.b[750] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[750]) {
            s.store_mul(708, 674, 707);
        }

        s.b[751] = (s.v[707] < (-50.0));
        s.store_scalar(751, if s.b[751] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[750])) && s.b[751]) {
            s.store_mul_exp_rhs(708, 674, 707);
        }

        if ((s.b[614] && (!s.b[750])) && (!s.b[751])) {
            s.store_mul_ln_one_plus_exp_rhs(708, 674, 707);
        }

        if s.b[614] {
            s.store_offset_square(709, 676, 1e-38);
            s.store_offset_mul(710, 709, 676, 1e-57);
            s.store_offset_square(711, 708, 1e-38);
            s.store_offset_mul(712, 711, 708, 1e-57);
            s.store_offset_mul(713, 676, 708, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(714, 709, (2.0 / 3.0), 711, (2.0 / 3.0), 713, (2.0 / 3.0), A::offset(A::add(s.ad_value(676), s.ad_value(708)), 2e-19), 1.0);
            s.store_div_ad(715, A::add_scaled_inputs_products(s.ad_value(710), (2.0 * 2.0), s.ad_value(712), (3.0 * 2.0), s.ad_value(709), s.ad_value(708), (4.0 * 2.0), s.ad_value(711), s.ad_value(676), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(709), 15.0, s.ad_value(711), 15.0, s.ad_value(713), (2.0 * 15.0)));
            s.store_sub(716, 714, 715);
            s.copy_ad(717, 715);
            s.store_mul_product3_mixed_iaii(617, 657, A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(634)), 656, 716, 1.0);
            s.store_mul_product3_mixed_iaii(618, 657, A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(634)), 656, 717, 1.0);
        }

        s.b[752] = (s.v[626] == 1.0);
        s.store_scalar(752, if s.b[752] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[752]) {
            s.store_div_scaled_inputs3_indices(718, 627, 1.0, 661, -1.0, 658, (-(-(p.p51 * 0.5))), 673, 1.0);
        }

        s.b[753] = (s.v[718] > 50.0);
        s.store_scalar(753, if s.b[753] { 1.0 } else { 0.0 });

        if ((s.b[614] && s.b[752]) && s.b[753]) {
            s.copy_ad(721, 718);
        }

        s.b[754] = (s.v[718] < (-50.0));
        s.store_scalar(754, if s.b[754] { 1.0 } else { 0.0 });

        if (((s.b[614] && s.b[752]) && (!s.b[753])) && s.b[754]) {
            s.store_exp(721, 718);
        }

        if (((s.b[614] && s.b[752]) && (!s.b[753])) && (!s.b[754])) {
            s.store_ln_one_plus_exp(721, 718);
        }

        if (s.b[614] && s.b[752]) {
            s.store_mul_ad_product_lhs_mixed_ai(619, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(637), s.ad_value(673)), 721, 657);
            s.store_div_scaled_inputs3_indices(719, 628, 1.0, 661, -1.0, 658, (-(-(p.p51 * 0.5))), 673, 1.0);
        }

        s.b[755] = (s.v[719] > 50.0);
        s.store_scalar(755, if s.b[755] { 1.0 } else { 0.0 });

        if ((s.b[614] && s.b[752]) && s.b[755]) {
            s.copy_ad(721, 719);
        }

        s.b[756] = (s.v[719] < (-50.0));
        s.store_scalar(756, if s.b[756] { 1.0 } else { 0.0 });

        if (((s.b[614] && s.b[752]) && (!s.b[755])) && s.b[756]) {
            s.store_exp(721, 719);
        }

        if (((s.b[614] && s.b[752]) && (!s.b[755])) && (!s.b[756])) {
            s.store_ln_one_plus_exp(721, 719);
        }

        if (s.b[614] && s.b[752]) {
            s.store_mul_ad_product_lhs_mixed_ai(620, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(638), s.ad_value(673)), 721, 657);
        }

        if (s.b[614] && (!s.b[752])) {
            s.store_scalar(619, 0.0);
            s.store_scalar(620, 0.0);
        }

        s.b[757] = (s.v[629] == 1.0);
        s.store_scalar(757, if s.b[757] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[757]) {
            s.store_div_scaled_inputs3_indices(720, 624, 1.0, 661, -1.0, 658, (-(-(p.p51 * 0.5))), 673, 1.0);
        }

        s.b[758] = (s.v[720] > 50.0);
        s.store_scalar(758, if s.b[758] { 1.0 } else { 0.0 });

        if ((s.b[614] && s.b[757]) && s.b[758]) {
            s.copy_ad(721, 720);
        }

        s.b[759] = (s.v[720] < (-50.0));
        s.store_scalar(759, if s.b[759] { 1.0 } else { 0.0 });

        if (((s.b[614] && s.b[757]) && (!s.b[758])) && s.b[759]) {
            s.store_exp(721, 720);
        }

        if (((s.b[614] && s.b[757]) && (!s.b[758])) && (!s.b[759])) {
            s.store_ln_one_plus_exp(721, 720);
        }

        if (s.b[614] && s.b[757]) {
            s.store_mul_ad_product_lhs_mixed_ai(621, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(636), s.ad_value(673)), 721, 657);
        }

        if (s.b[614] && (!s.b[757])) {
            s.store_scalar(621, 0.0);
        }

        if s.b[614] {
            s.copy_ad(615, 616);
            s.copy_ad(196, 616);
            s.copy_ad(197, 617);
            s.copy_ad(198, 618);
            s.copy_ad(199, 619);
            s.copy_ad(200, 620);
            s.copy_ad(201, 621);
            s.copy_ad(196, 615);
        }

        s.b[760] = (p.p188 == 1.0);
        s.store_scalar(760, if s.b[760] { 1.0 } else { 0.0 });

        s.store_scalar(190, 0.0);

        s.store_scalar(191, 0.0);

        s.store_scalar(192, 0.0);

        s.store_scalar(193, 0.0);

        s.store_scalar(194, 0.0);

        s.store_scalar(195, 0.0);

        s.b[761] = (p.p167 > p.p354);
        s.store_scalar(761, if s.b[761] { 1.0 } else { 0.0 });

        if s.b[761] {
            s.store_scalar(762, 0.0);
            s.store_scalar(763, 0.0);
            s.store_scalar(764, 0.0);
            s.store_scalar(765, 0.0);
            s.store_scalar(766, 0.0);
            s.store_scalar(767, 0.0);
            s.store_scalar(768, 0.0);
            s.store_scalar(769, 0.0);
            s.store_scalar(770, 0.0);
            s.copy_ad(771, 84);
            s.copy_ad(772, 85);
        }

        let (assign10390_e11205,) = {
    if s.b[761] {
        (p.p173,)
    } else {
        (s.v[773],)
    }
};
        s.store_scalar(773, assign10390_e11205);

        if s.b[761] {
            s.copy_ad(774, 86);
            s.copy_ad(775, 87);
        }

        let (assign10420_e11217,) = {
    if s.b[761] {
        (p.p171,)
    } else {
        (s.v[776],)
    }
};
        s.store_scalar(776, assign10420_e11217);

        if s.b[761] {
            s.copy_ad(777, 111);
            s.store_scalar(778, var_tnomk);
            s.copy_ad(779, 113);
            s.store_scalar(780, p.p0);
            s.store_scalar(781, p.p167);
            s.copy_ad(782, 32);
            s.store_scalar(783, p.p172);
            s.copy_ad(784, 33);
            s.copy_ad(785, 34);
            s.store_scalar(786, p.p168);
            s.store_scalar(787, p.p182);
            s.store_scalar(788, p.p181);
            s.store_scalar(789, 0.0);
            s.store_scalar(790, p.p183);
            s.store_scalar(791, p.p187);
            s.store_scalar(792, p.p178);
            s.store_scalar(793, p.p179);
            s.store_scalar(794, p.p180);
            s.store_scalar(795, p.p186);
            s.store_scalar(796, p.p185);
            s.store_scalar(797, p.p184);
            s.store_scalar(798, p.p39);
            s.store_scalar(799, p.p47);
            s.store_scalar(800, p.p45);
            s.store_scalar(801, p.p42);
            s.store_scalar(802, p.p2);
            s.store_scalar(803, p.p6);
            s.store_scalar(804, 1.0);
            s.store_scalar(805, 0.0);
            s.store_scalar(806, 0.0);
            s.store_scalar(807, 0.0);
            s.store_scalar(808, 0.0);
            s.store_scalar(809, 0.0);
            s.store_scalar(810, 0.0);
            s.store_scalar(811, 0.0);
            s.store_scalar(812, 0.0);
            s.store_scalar(813, 0.0);
            s.store_scalar(814, 0.0);
            s.store_scalar(815, 0.0);
            s.store_scalar(816, 0.0);
            s.store_scalar(817, 0.0);
            s.store_scalar(818, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_86(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[761] {
            s.store_scalar(819, 0.0);
            s.store_scalar(820, 0.0);
            s.store_scalar(821, 0.0);
            s.store_scalar(822, 0.0);
            s.store_scalar(823, 0.0);
            s.store_scalar(824, 0.0);
            s.store_scalar(825, 0.0);
            s.store_scalar(826, 0.0);
            s.store_scalar(827, 0.0);
            s.store_scalar(828, 0.0);
            s.store_scalar(829, 0.0);
            s.store_scalar(830, 0.0);
            s.store_scalar(831, 0.0);
            s.store_scalar(832, 0.0);
            s.store_scalar(833, 0.0);
            s.store_scalar(834, 0.0);
            s.store_scalar(835, 0.0);
            s.store_scalar(836, 0.0);
            s.store_scalar(837, 0.0);
            s.store_scalar(838, 0.0);
            s.store_scalar(839, 0.0);
            s.store_scalar(840, 0.0);
            s.store_scalar(841, 0.0);
            s.store_scalar(842, 0.0);
            s.store_scalar(843, 0.0);
            s.store_scalar(844, 0.0);
            s.store_scalar(845, 0.0);
            s.store_scalar(846, 0.0);
            s.store_scalar(847, 0.0);
            s.store_scalar(848, 0.0);
            s.store_scalar(849, 0.0);
            s.store_scalar(850, 0.0);
            s.store_scalar(851, 0.0);
            s.store_scalar(852, 0.0);
            s.store_scalar(853, 0.0);
            s.store_scalar(854, 0.0);
            s.store_scalar(855, 0.0);
            s.store_scalar(856, 0.0);
            s.store_scalar(857, 0.0);
            s.store_scalar(858, 0.0);
            s.store_scalar(859, 0.0);
            s.store_scalar(860, 0.0);
            s.store_scalar(861, 0.0);
            s.store_scalar(862, 0.0);
            s.store_scalar(863, 0.0);
            s.store_scalar(864, 0.0);
            s.store_scalar(865, 0.0);
            s.store_scalar(866, 0.0);
            s.store_scalar(867, 0.0);
            s.store_scalar(868, 0.0);
            s.store_scalar(869, 0.0);
            s.store_scalar(870, 0.0);
            s.store_scalar(871, 0.0);
            s.store_scalar(872, 0.0);
            s.store_scalar(873, 0.0);
        }

        if s.b[761] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(870, 772, A::tanh_scaled_input(s.ad_value(772), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(870, 772, p.p53);
                } else {
                    s.store_scalar(870, 0.0);
                }
            }
        }

        if s.b[761] {
            s.store_sub(871, 771, 772);
            s.store_mul(805, 791, 779);
            s.store_add_scaled_product_value_ad(807, A::div_scaled_inputs(s.ad_value(787), 1.0, s.ad_value(779), 2.302585092994046), 1.0, 790, 870, 1.0);
            s.store_add_scaled_product_right_sub(808, 786, 1.0, 797, 777, 778, 1.0);
            s.store_pow_ad(826, A::div(s.ad_value(777), s.ad_value(778)), s.ad_value(799));
        }

        s.b[874] = (s.v[798] != 0.0);
        s.store_scalar(874, if s.b[874] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[874]) {
            s.store_div_ad_rhs(809, 870, A::pow(A::offset(A::pow(A::div(s.ad_value(870), s.ad_value(798)), s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794))));
        }

        if (s.b[761] && (!s.b[874])) {
            s.store_scalar(809, 0.0);
        }

        if s.b[761] {
            s.store_mul_add_scaled_product_rhs(806, 870, s.ad_value(788), 1.0, s.ad_value(809), s.ad_value(789), (-1.0));
            s.store_sub(769, 808, 806);
            s.store_scaled_mul(811, 807, 779, 2.0);
            s.store_mul(812, 782, 811);
            s.store_sub_scaled_inputs(869, 769, 1.0, 805, (p.p51 * 0.5));
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_mixed_aii(868, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(771), s.ad_value(871)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 869, (-1.0), 805, 1.0);
        }

        s.b[875] = (s.v[868] > 50.0);
        s.store_scalar(875, if s.b[875] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[875]) {
            s.store_scalar(827, 0.0);
        }

        s.b[876] = (s.v[868] < (-50.0));
        s.store_scalar(876, if s.b[876] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[875])) && s.b[876]) {
            s.store_scalar(827, 1.0);
        }

        if ((s.b[761] && (!s.b[875])) && (!s.b[876])) {
            s.store_div_from_scalar_offset_ad(827, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_mixed_aai(828, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(771), s.ad_value(871)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(827), (-(p.p51 * 0.1))), (-1.0), 811, 1.0);
        }

        s.b[877] = (s.v[828] > 50.0);
        s.store_scalar(877, if s.b[877] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[877]) {
            s.store_mul(829, 812, 828);
        }

        s.b[878] = (s.v[828] < (-50.0));
        s.store_scalar(878, if s.b[878] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[877])) && s.b[878]) {
            s.store_mul_exp_rhs(829, 812, 828);
        }

        if ((s.b[761] && (!s.b[877])) && (!s.b[878])) {
            s.store_mul_ln_one_plus_exp_rhs(829, 812, 828);
        }

        if s.b[761] {
            s.store_div_ad_rhs(815, 793, A::mul_offset_rhs(s.ad_value(826), A::div_scaled_product(s.ad_value(795), s.ad_value(829), 1.0, s.ad_value(782), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(816, 792, A::div_scaled_offset_numerator(A::mul(s.ad_value(800), s.ad_value(778)), 1.0, 1.0, A::offset(A::mul(s.ad_value(800), s.ad_value(777)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(801), s.ad_value(870), 1.0, s.ad_value(781), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(796), s.ad_value(829), 1.0, s.ad_value(782), 1.0), 1.0), 1.0);
            s.store_add_ad(817, A::div_scaled_product3(s.ad_value(827), s.ad_value(779), s.ad_value(815), 2.0, s.ad_value(781), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(827), s.ad_value(816)));
            s.store_div_scaled_product_indices(833, 816, 781, 1.0, 815, 1.0);
            s.store_add_scaled_product_right_ad(834, 833, (-1.0), 833, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(829), 2.0, s.ad_value(782), s.ad_value(833), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(835, A::mul_sub_from_scalar_rhs(s.ad_value(833), 1.0, s.ad_value(827)), 1.0, 811, 827, 1.0);
            s.store_add_scaled_product_value_ad(770, A::mul_sub_from_scalar_rhs(s.ad_value(834), 1.0, s.ad_value(827)), 1.0, 811, 827, 1.0);
        }

        if s.b[761] {
            s.store_div_from_scalar_pow_ad(836, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(772), s.ad_value(770)), 0.5, A::div(s.ad_value(772), s.ad_value(770)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(772), s.ad_value(770))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(772), s.ad_value(770)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(772), s.ad_value(770))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul(837, 772, 836);
        }

        if s.b[761] {
            s.store_div_from_scalar_pow_ad(838, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul_neg_lhs(839, 772, 838);
            s.store_div_scaled_inputs2_indices(868, 771, 1.0, 869, (-1.0), 805, 1.0);
        }

        s.b[879] = (s.v[868] > 50.0);
        s.store_scalar(879, if s.b[879] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[879]) {
            s.store_scalar(810, 0.0);
        }

        s.b[880] = (s.v[868] < (-50.0));
        s.store_scalar(880, if s.b[880] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[879])) && s.b[880]) {
            s.store_scalar(810, 1.0);
        }

        if ((s.b[761] && (!s.b[879])) && (!s.b[880])) {
            s.store_div_from_scalar_offset_ad(810, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3_mixed_iiai(813, 871, 1.0, 839, (-1.0), A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(810), (-(p.p51 * 0.1))), -1.0, 811, 1.0);
        }

        s.b[881] = (s.v[813] > 50.0);
        s.store_scalar(881, if s.b[881] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[881]) {
            s.store_mul(814, 812, 813);
        }

        s.b[882] = (s.v[813] < (-50.0));
        s.store_scalar(882, if s.b[882] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[881])) && s.b[882]) {
            s.store_mul_exp_rhs(814, 812, 813);
        }

        if ((s.b[761] && (!s.b[881])) && (!s.b[882])) {
            s.store_mul_ln_one_plus_exp_rhs(814, 812, 813);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_indices(868, 871, 1.0, 869, (-1.0), 805, 1.0);
        }

        s.b[883] = (s.v[868] > 50.0);
        s.store_scalar(883, if s.b[883] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[883]) {
            s.store_scalar(840, 0.0);
        }

        s.b[884] = (s.v[868] < (-50.0));
        s.store_scalar(884, if s.b[884] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[883])) && s.b[884]) {
            s.store_scalar(840, 1.0);
        }

        if ((s.b[761] && (!s.b[883])) && (!s.b[884])) {
            s.store_div_from_scalar_offset_ad(840, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3_mixed_iiai(841, 771, 1.0, 837, (-1.0), A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(840), (-(p.p51 * 0.1))), -1.0, 811, 1.0);
        }

        s.b[885] = (s.v[841] > 50.0);
        s.store_scalar(885, if s.b[885] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[885]) {
            s.store_mul(842, 812, 841);
        }

        s.b[886] = (s.v[841] < (-50.0));
        s.store_scalar(886, if s.b[886] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[885])) && s.b[886]) {
            s.store_mul_exp_rhs(842, 812, 841);
        }

        if ((s.b[761] && (!s.b[885])) && (!s.b[886])) {
            s.store_mul_ln_one_plus_exp_rhs(842, 812, 841);
        }

    }

    pub(super) fn stamp_transient_block_87(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[761] {
            s.store_div_scaled_inputs2_indices(843, 814, 1.0, 842, (-1.0), 782, 1.0);
            s.store_div(869, 843, 835);
        }

        if s.b[761] {
            s.store_div_ad_rhs(844, 869, A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(869), A::tanh_scaled_input(s.ad_value(869), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(s.ad_value(869), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794))));
        }

        if s.b[761] {
            s.store_mul(845, 817, 844);
            s.store_mul_product3_mixed_iaai(763, 804, A::mul3(s.ad_value(803), s.ad_value(780), s.ad_value(802)), A::add(s.ad_value(814), s.ad_value(842)), 845, 0.5);
            s.store_div_scaled_inputs_indices(818, 787, 1.0, 779, 2.302585092994046);
            s.store_scaled_mul(820, 818, 779, 2.0);
            s.store_mul(821, 782, 820);
            s.store_sub_scaled_inputs(873, 808, 1.0, 805, (p.p51 * 0.5));
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_mixed_aii(872, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(771), s.ad_value(871)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 873, (-1.0), 805, 1.0);
        }

        s.b[887] = (s.v[872] > 50.0);
        s.store_scalar(887, if s.b[887] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[887]) {
            s.store_scalar(830, 0.0);
        }

        s.b[888] = (s.v[872] < (-50.0));
        s.store_scalar(888, if s.b[888] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[887])) && s.b[888]) {
            s.store_scalar(830, 1.0);
        }

        if ((s.b[761] && (!s.b[887])) && (!s.b[888])) {
            s.store_div_from_scalar_offset_ad(830, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_mixed_aai(831, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(771), s.ad_value(871)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(830), (-(p.p51 * 0.1))), (-1.0), 820, 1.0);
        }

        s.b[889] = (s.v[831] > 50.0);
        s.store_scalar(889, if s.b[889] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[889]) {
            s.store_mul(832, 821, 831);
        }

        s.b[890] = (s.v[831] < (-50.0));
        s.store_scalar(890, if s.b[890] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[889])) && s.b[890]) {
            s.store_mul_exp_rhs(832, 821, 831);
        }

        if ((s.b[761] && (!s.b[889])) && (!s.b[890])) {
            s.store_mul_ln_one_plus_exp_rhs(832, 821, 831);
        }

        if s.b[761] {
            s.store_div(824, 793, 826);
            s.store_mul_div_scaled_offset_numerator_rhs(825, 792, A::mul(s.ad_value(800), s.ad_value(778)), 1.0, 1.0, A::offset(A::mul(s.ad_value(800), s.ad_value(777)), 1.0), 1.0);
            s.store_div_scaled_product_indices(846, 825, 781, 1.0, 824, 1.0);
            s.store_add_scaled_product_right_ad(847, 846, (-1.0), 846, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(832), 2.0, s.ad_value(782), s.ad_value(846), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(848, A::mul_sub_from_scalar_rhs(s.ad_value(847), 1.0, s.ad_value(830)), 1.0, 820, 830, 1.0);
        }

        if s.b[761] {
            s.store_div_from_scalar_pow_ad(849, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(772), s.ad_value(848)), 0.5, A::div(s.ad_value(772), s.ad_value(848)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(772), s.ad_value(848))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(772), s.ad_value(848)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(772), s.ad_value(848))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul(850, 772, 849);
        }

        if s.b[761] {
            s.store_div_from_scalar_pow_ad(851, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul_neg_lhs(852, 772, 851);
            s.store_div_scaled_inputs2_indices(872, 771, 1.0, 873, (-1.0), 805, 1.0);
        }

        s.b[891] = (s.v[872] > 50.0);
        s.store_scalar(891, if s.b[891] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[891]) {
            s.store_scalar(819, 0.0);
        }

        s.b[892] = (s.v[872] < (-50.0));
        s.store_scalar(892, if s.b[892] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[891])) && s.b[892]) {
            s.store_scalar(819, 1.0);
        }

        if ((s.b[761] && (!s.b[891])) && (!s.b[892])) {
            s.store_div_from_scalar_offset_ad(819, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3_mixed_iiai(822, 871, 1.0, 852, (-1.0), A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(819), (-(p.p51 * 0.1))), -1.0, 820, 1.0);
        }

        s.b[893] = (s.v[822] > 50.0);
        s.store_scalar(893, if s.b[893] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[893]) {
            s.store_mul(823, 821, 822);
        }

        s.b[894] = (s.v[822] < (-50.0));
        s.store_scalar(894, if s.b[894] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[893])) && s.b[894]) {
            s.store_mul_exp_rhs(823, 821, 822);
        }

        if ((s.b[761] && (!s.b[893])) && (!s.b[894])) {
            s.store_mul_ln_one_plus_exp_rhs(823, 821, 822);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_indices(872, 871, 1.0, 873, (-1.0), 805, 1.0);
        }

        s.b[895] = (s.v[872] > 50.0);
        s.store_scalar(895, if s.b[895] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[895]) {
            s.store_scalar(853, 0.0);
        }

        s.b[896] = (s.v[872] < (-50.0));
        s.store_scalar(896, if s.b[896] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[895])) && s.b[896]) {
            s.store_scalar(853, 1.0);
        }

        if ((s.b[761] && (!s.b[895])) && (!s.b[896])) {
            s.store_div_from_scalar_offset_ad(853, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3_mixed_iiai(854, 771, 1.0, 850, (-1.0), A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(853), (-(p.p51 * 0.1))), -1.0, 820, 1.0);
        }

        s.b[897] = (s.v[854] > 50.0);
        s.store_scalar(897, if s.b[897] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[897]) {
            s.store_mul(855, 821, 854);
        }

        s.b[898] = (s.v[854] < (-50.0));
        s.store_scalar(898, if s.b[898] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[897])) && s.b[898]) {
            s.store_mul_exp_rhs(855, 821, 854);
        }

        if ((s.b[761] && (!s.b[897])) && (!s.b[898])) {
            s.store_mul_ln_one_plus_exp_rhs(855, 821, 854);
        }

        if s.b[761] {
            s.store_offset_square(856, 823, 1e-38);
            s.store_offset_mul(857, 856, 823, 1e-57);
            s.store_offset_square(858, 855, 1e-38);
            s.store_offset_mul(859, 858, 855, 1e-57);
            s.store_offset_mul(860, 823, 855, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(861, 856, (2.0 / 3.0), 858, (2.0 / 3.0), 860, (2.0 / 3.0), A::offset(A::add(s.ad_value(823), s.ad_value(855)), 2e-19), 1.0);
            s.store_div_ad(862, A::add_scaled_inputs_products(s.ad_value(857), (2.0 * 2.0), s.ad_value(859), (3.0 * 2.0), s.ad_value(856), s.ad_value(855), (4.0 * 2.0), s.ad_value(858), s.ad_value(823), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(856), 15.0, s.ad_value(858), 15.0, s.ad_value(860), (2.0 * 15.0)));
            s.store_sub(863, 861, 862);
            s.copy_ad(864, 862);
            s.store_mul_product3_mixed_iaii(764, 804, A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(781)), 803, 863, 1.0);
            s.store_mul_product3_mixed_iaii(765, 804, A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(781)), 803, 864, 1.0);
        }

        s.b[899] = (s.v[773] == 1.0);
        s.store_scalar(899, if s.b[899] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[899]) {
            s.store_div_scaled_inputs3_indices(865, 774, 1.0, 808, -1.0, 805, (-(-(p.p51 * 0.5))), 820, 1.0);
        }

        s.b[900] = (s.v[865] > 50.0);
        s.store_scalar(900, if s.b[900] { 1.0 } else { 0.0 });

        if ((s.b[761] && s.b[899]) && s.b[900]) {
            s.copy_ad(868, 865);
        }

        s.b[901] = (s.v[865] < (-50.0));
        s.store_scalar(901, if s.b[901] { 1.0 } else { 0.0 });

        if (((s.b[761] && s.b[899]) && (!s.b[900])) && s.b[901]) {
            s.store_exp(868, 865);
        }

        if (((s.b[761] && s.b[899]) && (!s.b[900])) && (!s.b[901])) {
            s.store_ln_one_plus_exp(868, 865);
        }

        if (s.b[761] && s.b[899]) {
            s.store_mul_ad_product_lhs_mixed_ai(766, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(784), s.ad_value(820)), 868, 804);
            s.store_div_scaled_inputs3_indices(866, 775, 1.0, 808, -1.0, 805, (-(-(p.p51 * 0.5))), 820, 1.0);
        }

        s.b[902] = (s.v[866] > 50.0);
        s.store_scalar(902, if s.b[902] { 1.0 } else { 0.0 });

        if ((s.b[761] && s.b[899]) && s.b[902]) {
            s.copy_ad(868, 866);
        }

        s.b[903] = (s.v[866] < (-50.0));
        s.store_scalar(903, if s.b[903] { 1.0 } else { 0.0 });

        if (((s.b[761] && s.b[899]) && (!s.b[902])) && s.b[903]) {
            s.store_exp(868, 866);
        }

        if (((s.b[761] && s.b[899]) && (!s.b[902])) && (!s.b[903])) {
            s.store_ln_one_plus_exp(868, 866);
        }

        if (s.b[761] && s.b[899]) {
            s.store_mul_ad_product_lhs_mixed_ai(767, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(785), s.ad_value(820)), 868, 804);
        }

        if (s.b[761] && (!s.b[899])) {
            s.store_scalar(766, 0.0);
            s.store_scalar(767, 0.0);
        }

        s.b[904] = (s.v[776] == 1.0);
        s.store_scalar(904, if s.b[904] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[904]) {
            s.store_div_scaled_inputs3_indices(867, 771, 1.0, 808, -1.0, 805, (-(-(p.p51 * 0.5))), 820, 1.0);
        }

        s.b[905] = (s.v[867] > 50.0);
        s.store_scalar(905, if s.b[905] { 1.0 } else { 0.0 });

        if ((s.b[761] && s.b[904]) && s.b[905]) {
            s.copy_ad(868, 867);
        }

        s.b[906] = (s.v[867] < (-50.0));
        s.store_scalar(906, if s.b[906] { 1.0 } else { 0.0 });

        if (((s.b[761] && s.b[904]) && (!s.b[905])) && s.b[906]) {
            s.store_exp(868, 867);
        }

        if (((s.b[761] && s.b[904]) && (!s.b[905])) && (!s.b[906])) {
            s.store_ln_one_plus_exp(868, 867);
        }

        if (s.b[761] && s.b[904]) {
            s.store_mul_ad_product_lhs_mixed_ai(768, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(783), s.ad_value(820)), 868, 804);
        }

        if (s.b[761] && (!s.b[904])) {
            s.store_scalar(768, 0.0);
        }

        if s.b[761] {
            s.copy_ad(762, 763);
            s.copy_ad(190, 763);
            s.copy_ad(191, 764);
            s.copy_ad(192, 765);
            s.copy_ad(193, 766);
            s.copy_ad(194, 767);
            s.copy_ad(195, 768);
            s.copy_ad(190, 762);
        }

        s.b[907] = (p.p166 == 1.0);
        s.store_scalar(907, if s.b[907] { 1.0 } else { 0.0 });

        s.store_scalar(166, 0.0);

        s.store_scalar(167, 0.0);

        s.store_scalar(168, 0.0);

        s.store_scalar(169, 0.0);

        s.store_scalar(170, 0.0);

        s.store_scalar(171, 0.0);

        s.b[908] = (p.p79 > p.p354);
        s.store_scalar(908, if s.b[908] { 1.0 } else { 0.0 });

        if s.b[908] {
            s.store_scalar(909, 0.0);
            s.store_scalar(910, 0.0);
            s.store_scalar(911, 0.0);
            s.store_scalar(912, 0.0);
            s.store_scalar(913, 0.0);
            s.store_scalar(914, 0.0);
            s.store_scalar(915, 0.0);
            s.store_scalar(916, 0.0);
            s.store_scalar(917, 0.0);
            s.copy_ad(918, 60);
            s.copy_ad(919, 61);
        }

        let (assign13230_e13629,) = {
    if s.b[908] {
        (p.p85,)
    } else {
        (s.v[920],)
    }
};
        s.store_scalar(920, assign13230_e13629);

        if s.b[908] {
            s.copy_ad(921, 62);
        }

    }

    pub(super) fn stamp_transient_block_88(
        s: &mut Scratch,
        p: &Parameters,
        var_tnomk: f64,
    ) {
        if s.b[908] {
            s.copy_ad(922, 63);
        }

        let (assign13260_e13641,) = {
    if s.b[908] {
        (p.p83,)
    } else {
        (s.v[923],)
    }
};
        s.store_scalar(923, assign13260_e13641);

        if s.b[908] {
            s.copy_ad(924, 111);
            s.store_scalar(925, var_tnomk);
            s.copy_ad(926, 113);
            s.store_scalar(927, p.p0);
            s.store_scalar(928, p.p79);
            s.copy_ad(929, 20);
            s.store_scalar(930, p.p84);
            s.copy_ad(931, 21);
            s.copy_ad(932, 22);
            s.store_scalar(933, p.p80);
            s.store_scalar(934, p.p94);
            s.store_scalar(935, p.p93);
            s.store_scalar(936, 0.0);
            s.store_scalar(937, p.p95);
            s.store_scalar(938, p.p99);
            s.store_scalar(939, p.p90);
            s.store_scalar(940, p.p91);
            s.store_scalar(941, p.p92);
            s.store_scalar(942, p.p98);
            s.store_scalar(943, p.p97);
            s.store_scalar(944, p.p96);
            s.store_scalar(945, p.p39);
            s.store_scalar(946, p.p47);
            s.store_scalar(947, p.p45);
            s.store_scalar(948, p.p42);
            s.store_scalar(949, p.p2);
            s.store_scalar(950, p.p6);
            s.store_scalar(951, 1.0);
            s.store_scalar(952, 0.0);
            s.store_scalar(953, 0.0);
            s.store_scalar(954, 0.0);
            s.store_scalar(955, 0.0);
            s.store_scalar(956, 0.0);
            s.store_scalar(957, 0.0);
            s.store_scalar(958, 0.0);
            s.store_scalar(959, 0.0);
            s.store_scalar(960, 0.0);
            s.store_scalar(961, 0.0);
            s.store_scalar(962, 0.0);
            s.store_scalar(963, 0.0);
            s.store_scalar(964, 0.0);
            s.store_scalar(965, 0.0);
            s.store_scalar(966, 0.0);
            s.store_scalar(967, 0.0);
            s.store_scalar(968, 0.0);
            s.store_scalar(969, 0.0);
            s.store_scalar(970, 0.0);
            s.store_scalar(971, 0.0);
            s.store_scalar(972, 0.0);
            s.store_scalar(973, 0.0);
            s.store_scalar(974, 0.0);
            s.store_scalar(975, 0.0);
            s.store_scalar(976, 0.0);
            s.store_scalar(977, 0.0);
            s.store_scalar(978, 0.0);
            s.store_scalar(979, 0.0);
            s.store_scalar(980, 0.0);
            s.store_scalar(981, 0.0);
            s.store_scalar(982, 0.0);
            s.store_scalar(983, 0.0);
            s.store_scalar(984, 0.0);
            s.store_scalar(985, 0.0);
            s.store_scalar(986, 0.0);
            s.store_scalar(987, 0.0);
            s.store_scalar(988, 0.0);
            s.store_scalar(989, 0.0);
            s.store_scalar(990, 0.0);
            s.store_scalar(991, 0.0);
            s.store_scalar(992, 0.0);
            s.store_scalar(993, 0.0);
            s.store_scalar(994, 0.0);
            s.store_scalar(995, 0.0);
            s.store_scalar(996, 0.0);
            s.store_scalar(997, 0.0);
            s.store_scalar(998, 0.0);
            s.store_scalar(999, 0.0);
            s.store_scalar(1000, 0.0);
            s.store_scalar(1001, 0.0);
            s.store_scalar(1002, 0.0);
            s.store_scalar(1003, 0.0);
            s.store_scalar(1004, 0.0);
            s.store_scalar(1005, 0.0);
            s.store_scalar(1006, 0.0);
            s.store_scalar(1007, 0.0);
            s.store_scalar(1008, 0.0);
            s.store_scalar(1009, 0.0);
            s.store_scalar(1010, 0.0);
            s.store_scalar(1011, 0.0);
            s.store_scalar(1012, 0.0);
            s.store_scalar(1013, 0.0);
            s.store_scalar(1014, 0.0);
            s.store_scalar(1015, 0.0);
            s.store_scalar(1016, 0.0);
            s.store_scalar(1017, 0.0);
            s.store_scalar(1018, 0.0);
            s.store_scalar(1019, 0.0);
            s.store_scalar(1020, 0.0);
        }

        if s.b[908] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1017, 919, A::tanh_scaled_input(s.ad_value(919), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1017, 919, p.p53);
                } else {
                    s.store_scalar(1017, 0.0);
                }
            }
        }

        if s.b[908] {
            s.store_sub(1018, 918, 919);
            s.store_mul(952, 938, 926);
            s.store_add_scaled_product_value_ad(954, A::div_scaled_inputs(s.ad_value(934), 1.0, s.ad_value(926), 2.302585092994046), 1.0, 937, 1017, 1.0);
            s.store_add_scaled_product_right_sub(955, 933, 1.0, 944, 924, 925, 1.0);
            s.store_pow_ad(973, A::div(s.ad_value(924), s.ad_value(925)), s.ad_value(946));
        }

        s.b[1021] = (s.v[945] != 0.0);
        s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1021]) {
            s.store_div_ad_rhs(956, 1017, A::pow(A::offset(A::pow(A::div(s.ad_value(1017), s.ad_value(945)), s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941))));
        }

        if (s.b[908] && (!s.b[1021])) {
            s.store_scalar(956, 0.0);
        }

        if s.b[908] {
            s.store_mul_add_scaled_product_rhs(953, 1017, s.ad_value(935), 1.0, s.ad_value(956), s.ad_value(936), (-1.0));
            s.store_sub(916, 955, 953);
            s.store_scaled_mul(958, 954, 926, 2.0);
            s.store_mul(959, 929, 958);
            s.store_sub_scaled_inputs(1016, 916, 1.0, 952, (p.p51 * 0.5));
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aii(1015, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1016, (-1.0), 952, 1.0);
        }

        s.b[1022] = (s.v[1015] > 50.0);
        s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1022]) {
            s.store_scalar(974, 0.0);
        }

        s.b[1023] = (s.v[1015] < (-50.0));
        s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1022])) && s.b[1023]) {
            s.store_scalar(974, 1.0);
        }

        if ((s.b[908] && (!s.b[1022])) && (!s.b[1023])) {
            s.store_div_from_scalar_offset_ad(974, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_89(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aai(975, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(974), (-(p.p51 * 0.1))), (-1.0), 958, 1.0);
        }

        s.b[1024] = (s.v[975] > 50.0);
        s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1024]) {
            s.store_mul(976, 959, 975);
        }

        s.b[1025] = (s.v[975] < (-50.0));
        s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1024])) && s.b[1025]) {
            s.store_mul_exp_rhs(976, 959, 975);
        }

        if ((s.b[908] && (!s.b[1024])) && (!s.b[1025])) {
            s.store_mul_ln_one_plus_exp_rhs(976, 959, 975);
        }

        if s.b[908] {
            s.store_div_ad_rhs(962, 940, A::mul_offset_rhs(s.ad_value(973), A::div_scaled_product(s.ad_value(942), s.ad_value(976), 1.0, s.ad_value(929), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(963, 939, A::div_scaled_offset_numerator(A::mul(s.ad_value(947), s.ad_value(925)), 1.0, 1.0, A::offset(A::mul(s.ad_value(947), s.ad_value(924)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(948), s.ad_value(1017), 1.0, s.ad_value(928), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(943), s.ad_value(976), 1.0, s.ad_value(929), 1.0), 1.0), 1.0);
            s.store_add_ad(964, A::div_scaled_product3(s.ad_value(974), s.ad_value(926), s.ad_value(962), 2.0, s.ad_value(928), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(974), s.ad_value(963)));
            s.store_div_scaled_product_indices(980, 963, 928, 1.0, 962, 1.0);
            s.store_add_scaled_product_right_ad(981, 980, (-1.0), 980, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(976), 2.0, s.ad_value(929), s.ad_value(980), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(982, A::mul_sub_from_scalar_rhs(s.ad_value(980), 1.0, s.ad_value(974)), 1.0, 958, 974, 1.0);
            s.store_add_scaled_product_value_ad(917, A::mul_sub_from_scalar_rhs(s.ad_value(981), 1.0, s.ad_value(974)), 1.0, 958, 974, 1.0);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(983, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(919), s.ad_value(917)), 0.5, A::div(s.ad_value(919), s.ad_value(917)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(919), s.ad_value(917))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(919), s.ad_value(917)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(919), s.ad_value(917))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul(984, 919, 983);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(985, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul_neg_lhs(986, 919, 985);
            s.store_div_scaled_inputs2_indices(1015, 918, 1.0, 1016, (-1.0), 952, 1.0);
        }

        s.b[1026] = (s.v[1015] > 50.0);
        s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1026]) {
            s.store_scalar(957, 0.0);
        }

        s.b[1027] = (s.v[1015] < (-50.0));
        s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1026])) && s.b[1027]) {
            s.store_scalar(957, 1.0);
        }

        if ((s.b[908] && (!s.b[1026])) && (!s.b[1027])) {
            s.store_div_from_scalar_offset_ad(957, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3_mixed_iiai(960, 1018, 1.0, 986, (-1.0), A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(957), (-(p.p51 * 0.1))), -1.0, 958, 1.0);
        }

        s.b[1028] = (s.v[960] > 50.0);
        s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1028]) {
            s.store_mul(961, 959, 960);
        }

        s.b[1029] = (s.v[960] < (-50.0));
        s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1028])) && s.b[1029]) {
            s.store_mul_exp_rhs(961, 959, 960);
        }

        if ((s.b[908] && (!s.b[1028])) && (!s.b[1029])) {
            s.store_mul_ln_one_plus_exp_rhs(961, 959, 960);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_indices(1015, 1018, 1.0, 1016, (-1.0), 952, 1.0);
        }

        s.b[1030] = (s.v[1015] > 50.0);
        s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1030]) {
            s.store_scalar(987, 0.0);
        }

        s.b[1031] = (s.v[1015] < (-50.0));
        s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1030])) && s.b[1031]) {
            s.store_scalar(987, 1.0);
        }

        if ((s.b[908] && (!s.b[1030])) && (!s.b[1031])) {
            s.store_div_from_scalar_offset_ad(987, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3_mixed_iiai(988, 918, 1.0, 984, (-1.0), A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(987), (-(p.p51 * 0.1))), -1.0, 958, 1.0);
        }

        s.b[1032] = (s.v[988] > 50.0);
        s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1032]) {
            s.store_mul(989, 959, 988);
        }

        s.b[1033] = (s.v[988] < (-50.0));
        s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1032])) && s.b[1033]) {
            s.store_mul_exp_rhs(989, 959, 988);
        }

        if ((s.b[908] && (!s.b[1032])) && (!s.b[1033])) {
            s.store_mul_ln_one_plus_exp_rhs(989, 959, 988);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_indices(990, 961, 1.0, 989, (-1.0), 929, 1.0);
            s.store_div(1016, 990, 982);
        }

        if s.b[908] {
            s.store_div_ad_rhs(991, 1016, A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(1016), A::tanh_scaled_input(s.ad_value(1016), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(s.ad_value(1016), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941))));
        }

        if s.b[908] {
            s.store_mul(992, 964, 991);
            s.store_mul_product3_mixed_iaai(910, 951, A::mul3(s.ad_value(950), s.ad_value(927), s.ad_value(949)), A::add(s.ad_value(961), s.ad_value(989)), 992, 0.5);
            s.store_div_scaled_inputs_indices(965, 934, 1.0, 926, 2.302585092994046);
            s.store_scaled_mul(967, 965, 926, 2.0);
            s.store_mul(968, 929, 967);
            s.store_sub_scaled_inputs(1020, 955, 1.0, 952, (p.p51 * 0.5));
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aii(1019, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1020, (-1.0), 952, 1.0);
        }

        s.b[1034] = (s.v[1019] > 50.0);
        s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1034]) {
            s.store_scalar(977, 0.0);
        }

        s.b[1035] = (s.v[1019] < (-50.0));
        s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1034])) && s.b[1035]) {
            s.store_scalar(977, 1.0);
        }

        if ((s.b[908] && (!s.b[1034])) && (!s.b[1035])) {
            s.store_div_from_scalar_offset_ad(977, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aai(978, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(977), (-(p.p51 * 0.1))), (-1.0), 967, 1.0);
        }

        s.b[1036] = (s.v[978] > 50.0);
        s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1036]) {
            s.store_mul(979, 968, 978);
        }

        s.b[1037] = (s.v[978] < (-50.0));
        s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1036])) && s.b[1037]) {
            s.store_mul_exp_rhs(979, 968, 978);
        }

        if ((s.b[908] && (!s.b[1036])) && (!s.b[1037])) {
            s.store_mul_ln_one_plus_exp_rhs(979, 968, 978);
        }

        if s.b[908] {
            s.store_div(971, 940, 973);
            s.store_mul_div_scaled_offset_numerator_rhs(972, 939, A::mul(s.ad_value(947), s.ad_value(925)), 1.0, 1.0, A::offset(A::mul(s.ad_value(947), s.ad_value(924)), 1.0), 1.0);
            s.store_div_scaled_product_indices(993, 972, 928, 1.0, 971, 1.0);
            s.store_add_scaled_product_right_ad(994, 993, (-1.0), 993, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(979), 2.0, s.ad_value(929), s.ad_value(993), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(995, A::mul_sub_from_scalar_rhs(s.ad_value(994), 1.0, s.ad_value(977)), 1.0, 967, 977, 1.0);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(996, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(919), s.ad_value(995)), 0.5, A::div(s.ad_value(919), s.ad_value(995)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(919), s.ad_value(995))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(919), s.ad_value(995)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(919), s.ad_value(995))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul(997, 919, 996);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(998, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul_neg_lhs(999, 919, 998);
            s.store_div_scaled_inputs2_indices(1019, 918, 1.0, 1020, (-1.0), 952, 1.0);
        }

        s.b[1038] = (s.v[1019] > 50.0);
        s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1038]) {
            s.store_scalar(966, 0.0);
        }

        s.b[1039] = (s.v[1019] < (-50.0));
        s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1038])) && s.b[1039]) {
            s.store_scalar(966, 1.0);
        }

        if ((s.b[908] && (!s.b[1038])) && (!s.b[1039])) {
            s.store_div_from_scalar_offset_ad(966, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3_mixed_iiai(969, 1018, 1.0, 999, (-1.0), A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(966), (-(p.p51 * 0.1))), -1.0, 967, 1.0);
        }

        s.b[1040] = (s.v[969] > 50.0);
        s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1040]) {
            s.store_mul(970, 968, 969);
        }

        s.b[1041] = (s.v[969] < (-50.0));
        s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1040])) && s.b[1041]) {
            s.store_mul_exp_rhs(970, 968, 969);
        }

        if ((s.b[908] && (!s.b[1040])) && (!s.b[1041])) {
            s.store_mul_ln_one_plus_exp_rhs(970, 968, 969);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_indices(1019, 1018, 1.0, 1020, (-1.0), 952, 1.0);
        }

        s.b[1042] = (s.v[1019] > 50.0);
        s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1042]) {
            s.store_scalar(1000, 0.0);
        }

        s.b[1043] = (s.v[1019] < (-50.0));
        s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1042])) && s.b[1043]) {
            s.store_scalar(1000, 1.0);
        }

        if ((s.b[908] && (!s.b[1042])) && (!s.b[1043])) {
            s.store_div_from_scalar_offset_ad(1000, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3_mixed_iiai(1001, 918, 1.0, 997, (-1.0), A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(1000), (-(p.p51 * 0.1))), -1.0, 967, 1.0);
        }

        s.b[1044] = (s.v[1001] > 50.0);
        s.store_scalar(1044, if s.b[1044] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1044]) {
            s.store_mul(1002, 968, 1001);
        }

        s.b[1045] = (s.v[1001] < (-50.0));
        s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1044])) && s.b[1045]) {
            s.store_mul_exp_rhs(1002, 968, 1001);
        }

        if ((s.b[908] && (!s.b[1044])) && (!s.b[1045])) {
            s.store_mul_ln_one_plus_exp_rhs(1002, 968, 1001);
        }

        if s.b[908] {
            s.store_offset_square(1003, 970, 1e-38);
            s.store_offset_mul(1004, 1003, 970, 1e-57);
            s.store_offset_square(1005, 1002, 1e-38);
            s.store_offset_mul(1006, 1005, 1002, 1e-57);
            s.store_offset_mul(1007, 970, 1002, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1008, 1003, (2.0 / 3.0), 1005, (2.0 / 3.0), 1007, (2.0 / 3.0), A::offset(A::add(s.ad_value(970), s.ad_value(1002)), 2e-19), 1.0);
            s.store_div_ad(1009, A::add_scaled_inputs_products(s.ad_value(1004), (2.0 * 2.0), s.ad_value(1006), (3.0 * 2.0), s.ad_value(1003), s.ad_value(1002), (4.0 * 2.0), s.ad_value(1005), s.ad_value(970), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1003), 15.0, s.ad_value(1005), 15.0, s.ad_value(1007), (2.0 * 15.0)));
            s.store_sub(1010, 1008, 1009);
            s.copy_ad(1011, 1009);
            s.store_mul_product3_mixed_iaii(911, 951, A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(928)), 950, 1010, 1.0);
            s.store_mul_product3_mixed_iaii(912, 951, A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(928)), 950, 1011, 1.0);
        }

        s.b[1046] = (s.v[920] == 1.0);
        s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1046]) {
            s.store_div_scaled_inputs3_indices(1012, 921, 1.0, 955, -1.0, 952, (-(-(p.p51 * 0.5))), 967, 1.0);
        }

        s.b[1047] = (s.v[1012] > 50.0);
        s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });

        if ((s.b[908] && s.b[1046]) && s.b[1047]) {
            s.copy_ad(1015, 1012);
        }

    }

    pub(super) fn stamp_transient_block_90(
        s: &mut Scratch,
        p: &Parameters,
        var_tnomk: f64,
    ) {
        s.b[1048] = (s.v[1012] < (-50.0));
        s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });

        if (((s.b[908] && s.b[1046]) && (!s.b[1047])) && s.b[1048]) {
            s.store_exp(1015, 1012);
        }

        if (((s.b[908] && s.b[1046]) && (!s.b[1047])) && (!s.b[1048])) {
            s.store_ln_one_plus_exp(1015, 1012);
        }

        if (s.b[908] && s.b[1046]) {
            s.store_mul_ad_product_lhs_mixed_ai(913, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(931), s.ad_value(967)), 1015, 951);
            s.store_div_scaled_inputs3_indices(1013, 922, 1.0, 955, -1.0, 952, (-(-(p.p51 * 0.5))), 967, 1.0);
        }

        s.b[1049] = (s.v[1013] > 50.0);
        s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });

        if ((s.b[908] && s.b[1046]) && s.b[1049]) {
            s.copy_ad(1015, 1013);
        }

        s.b[1050] = (s.v[1013] < (-50.0));
        s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });

        if (((s.b[908] && s.b[1046]) && (!s.b[1049])) && s.b[1050]) {
            s.store_exp(1015, 1013);
        }

        if (((s.b[908] && s.b[1046]) && (!s.b[1049])) && (!s.b[1050])) {
            s.store_ln_one_plus_exp(1015, 1013);
        }

        if (s.b[908] && s.b[1046]) {
            s.store_mul_ad_product_lhs_mixed_ai(914, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(932), s.ad_value(967)), 1015, 951);
        }

        if (s.b[908] && (!s.b[1046])) {
            s.store_scalar(913, 0.0);
            s.store_scalar(914, 0.0);
        }

        s.b[1051] = (s.v[923] == 1.0);
        s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1051]) {
            s.store_div_scaled_inputs3_indices(1014, 918, 1.0, 955, -1.0, 952, (-(-(p.p51 * 0.5))), 967, 1.0);
        }

        s.b[1052] = (s.v[1014] > 50.0);
        s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });

        if ((s.b[908] && s.b[1051]) && s.b[1052]) {
            s.copy_ad(1015, 1014);
        }

        s.b[1053] = (s.v[1014] < (-50.0));
        s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });

        if (((s.b[908] && s.b[1051]) && (!s.b[1052])) && s.b[1053]) {
            s.store_exp(1015, 1014);
        }

        if (((s.b[908] && s.b[1051]) && (!s.b[1052])) && (!s.b[1053])) {
            s.store_ln_one_plus_exp(1015, 1014);
        }

        if (s.b[908] && s.b[1051]) {
            s.store_mul_ad_product_lhs_mixed_ai(915, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(930), s.ad_value(967)), 1015, 951);
        }

        if (s.b[908] && (!s.b[1051])) {
            s.store_scalar(915, 0.0);
        }

        if s.b[908] {
            s.copy_ad(909, 910);
            s.copy_ad(166, 910);
            s.copy_ad(167, 911);
            s.copy_ad(168, 912);
            s.copy_ad(169, 913);
            s.copy_ad(170, 914);
            s.copy_ad(171, 915);
            s.copy_ad(166, 909);
        }

        s.b[1054] = (p.p78 == 1.0);
        s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });

        s.store_scalar(172, 0.0);

        s.store_scalar(173, 0.0);

        s.store_scalar(174, 0.0);

        s.store_scalar(175, 0.0);

        s.store_scalar(176, 0.0);

        s.store_scalar(177, 0.0);

        s.b[1055] = (p.p101 > p.p354);
        s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });

        if s.b[1055] {
            s.store_scalar(1056, 0.0);
            s.store_scalar(1057, 0.0);
            s.store_scalar(1058, 0.0);
            s.store_scalar(1059, 0.0);
            s.store_scalar(1060, 0.0);
            s.store_scalar(1061, 0.0);
            s.store_scalar(1062, 0.0);
            s.store_scalar(1063, 0.0);
            s.store_scalar(1064, 0.0);
            s.copy_ad(1065, 66);
            s.copy_ad(1066, 67);
        }

        let (assign16070_e16053,) = {
    if s.b[1055] {
        (p.p107,)
    } else {
        (s.v[1067],)
    }
};
        s.store_scalar(1067, assign16070_e16053);

        if s.b[1055] {
            s.copy_ad(1068, 68);
            s.copy_ad(1069, 69);
        }

        let (assign16100_e16065,) = {
    if s.b[1055] {
        (p.p105,)
    } else {
        (s.v[1070],)
    }
};
        s.store_scalar(1070, assign16100_e16065);

        if s.b[1055] {
            s.copy_ad(1071, 111);
            s.store_scalar(1072, var_tnomk);
            s.copy_ad(1073, 113);
            s.store_scalar(1074, p.p0);
            s.store_scalar(1075, p.p101);
            s.copy_ad(1076, 23);
            s.store_scalar(1077, p.p106);
            s.copy_ad(1078, 24);
            s.copy_ad(1079, 25);
            s.store_scalar(1080, p.p102);
            s.store_scalar(1081, p.p116);
            s.store_scalar(1082, p.p115);
            s.store_scalar(1083, 0.0);
            s.store_scalar(1084, p.p117);
            s.store_scalar(1085, p.p121);
            s.store_scalar(1086, p.p112);
            s.store_scalar(1087, p.p113);
            s.store_scalar(1088, p.p114);
            s.store_scalar(1089, p.p120);
            s.store_scalar(1090, p.p119);
            s.store_scalar(1091, p.p118);
            s.store_scalar(1092, p.p39);
            s.store_scalar(1093, p.p47);
            s.store_scalar(1094, p.p45);
            s.store_scalar(1095, p.p42);
            s.store_scalar(1096, p.p2);
            s.store_scalar(1097, p.p6);
            s.store_scalar(1098, 1.0);
            s.store_scalar(1099, 0.0);
            s.store_scalar(1100, 0.0);
            s.store_scalar(1101, 0.0);
            s.store_scalar(1102, 0.0);
            s.store_scalar(1103, 0.0);
            s.store_scalar(1104, 0.0);
            s.store_scalar(1105, 0.0);
            s.store_scalar(1106, 0.0);
            s.store_scalar(1107, 0.0);
            s.store_scalar(1108, 0.0);
            s.store_scalar(1109, 0.0);
            s.store_scalar(1110, 0.0);
            s.store_scalar(1111, 0.0);
            s.store_scalar(1112, 0.0);
            s.store_scalar(1113, 0.0);
            s.store_scalar(1114, 0.0);
            s.store_scalar(1115, 0.0);
            s.store_scalar(1116, 0.0);
            s.store_scalar(1117, 0.0);
            s.store_scalar(1118, 0.0);
            s.store_scalar(1119, 0.0);
            s.store_scalar(1120, 0.0);
            s.store_scalar(1121, 0.0);
            s.store_scalar(1122, 0.0);
            s.store_scalar(1123, 0.0);
            s.store_scalar(1124, 0.0);
            s.store_scalar(1125, 0.0);
            s.store_scalar(1126, 0.0);
            s.store_scalar(1127, 0.0);
            s.store_scalar(1128, 0.0);
            s.store_scalar(1129, 0.0);
            s.store_scalar(1130, 0.0);
            s.store_scalar(1131, 0.0);
            s.store_scalar(1132, 0.0);
            s.store_scalar(1133, 0.0);
            s.store_scalar(1134, 0.0);
            s.store_scalar(1135, 0.0);
            s.store_scalar(1136, 0.0);
            s.store_scalar(1137, 0.0);
            s.store_scalar(1138, 0.0);
            s.store_scalar(1139, 0.0);
            s.store_scalar(1140, 0.0);
            s.store_scalar(1141, 0.0);
            s.store_scalar(1142, 0.0);
            s.store_scalar(1143, 0.0);
            s.store_scalar(1144, 0.0);
            s.store_scalar(1145, 0.0);
            s.store_scalar(1146, 0.0);
            s.store_scalar(1147, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_91(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1055] {
            s.store_scalar(1148, 0.0);
            s.store_scalar(1149, 0.0);
            s.store_scalar(1150, 0.0);
            s.store_scalar(1151, 0.0);
            s.store_scalar(1152, 0.0);
            s.store_scalar(1153, 0.0);
            s.store_scalar(1154, 0.0);
            s.store_scalar(1155, 0.0);
            s.store_scalar(1156, 0.0);
            s.store_scalar(1157, 0.0);
            s.store_scalar(1158, 0.0);
            s.store_scalar(1159, 0.0);
            s.store_scalar(1160, 0.0);
            s.store_scalar(1161, 0.0);
            s.store_scalar(1162, 0.0);
            s.store_scalar(1163, 0.0);
            s.store_scalar(1164, 0.0);
            s.store_scalar(1165, 0.0);
            s.store_scalar(1166, 0.0);
            s.store_scalar(1167, 0.0);
        }

        if s.b[1055] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1164, 1066, A::tanh_scaled_input(s.ad_value(1066), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1164, 1066, p.p53);
                } else {
                    s.store_scalar(1164, 0.0);
                }
            }
        }

        if s.b[1055] {
            s.store_sub(1165, 1065, 1066);
            s.store_mul(1099, 1085, 1073);
            s.store_add_scaled_product_value_ad(1101, A::div_scaled_inputs(s.ad_value(1081), 1.0, s.ad_value(1073), 2.302585092994046), 1.0, 1084, 1164, 1.0);
            s.store_add_scaled_product_right_sub(1102, 1080, 1.0, 1091, 1071, 1072, 1.0);
            s.store_pow_ad(1120, A::div(s.ad_value(1071), s.ad_value(1072)), s.ad_value(1093));
        }

        s.b[1168] = (s.v[1092] != 0.0);
        s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1168]) {
            s.store_div_ad_rhs(1103, 1164, A::pow(A::offset(A::pow(A::div(s.ad_value(1164), s.ad_value(1092)), s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088))));
        }

        if (s.b[1055] && (!s.b[1168])) {
            s.store_scalar(1103, 0.0);
        }

        if s.b[1055] {
            s.store_mul_add_scaled_product_rhs(1100, 1164, s.ad_value(1082), 1.0, s.ad_value(1103), s.ad_value(1083), (-1.0));
            s.store_sub(1063, 1102, 1100);
            s.store_scaled_mul(1105, 1101, 1073, 2.0);
            s.store_mul(1106, 1076, 1105);
            s.store_sub_scaled_inputs(1163, 1063, 1.0, 1099, (p.p51 * 0.5));
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_mixed_aii(1162, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1163, (-1.0), 1099, 1.0);
        }

        s.b[1169] = (s.v[1162] > 50.0);
        s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1169]) {
            s.store_scalar(1121, 0.0);
        }

        s.b[1170] = (s.v[1162] < (-50.0));
        s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1169])) && s.b[1170]) {
            s.store_scalar(1121, 1.0);
        }

        if ((s.b[1055] && (!s.b[1169])) && (!s.b[1170])) {
            s.store_div_from_scalar_offset_ad(1121, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_mixed_aai(1122, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1121), (-(p.p51 * 0.1))), (-1.0), 1105, 1.0);
        }

        s.b[1171] = (s.v[1122] > 50.0);
        s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1171]) {
            s.store_mul(1123, 1106, 1122);
        }

        s.b[1172] = (s.v[1122] < (-50.0));
        s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1171])) && s.b[1172]) {
            s.store_mul_exp_rhs(1123, 1106, 1122);
        }

        if ((s.b[1055] && (!s.b[1171])) && (!s.b[1172])) {
            s.store_mul_ln_one_plus_exp_rhs(1123, 1106, 1122);
        }

        if s.b[1055] {
            s.store_div_ad_rhs(1109, 1087, A::mul_offset_rhs(s.ad_value(1120), A::div_scaled_product(s.ad_value(1089), s.ad_value(1123), 1.0, s.ad_value(1076), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1110, 1086, A::div_scaled_offset_numerator(A::mul(s.ad_value(1094), s.ad_value(1072)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1094), s.ad_value(1071)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1095), s.ad_value(1164), 1.0, s.ad_value(1075), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1090), s.ad_value(1123), 1.0, s.ad_value(1076), 1.0), 1.0), 1.0);
            s.store_add_ad(1111, A::div_scaled_product3(s.ad_value(1121), s.ad_value(1073), s.ad_value(1109), 2.0, s.ad_value(1075), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1121), s.ad_value(1110)));
            s.store_div_scaled_product_indices(1127, 1110, 1075, 1.0, 1109, 1.0);
            s.store_add_scaled_product_right_ad(1128, 1127, (-1.0), 1127, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1123), 2.0, s.ad_value(1076), s.ad_value(1127), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1129, A::mul_sub_from_scalar_rhs(s.ad_value(1127), 1.0, s.ad_value(1121)), 1.0, 1105, 1121, 1.0);
            s.store_add_scaled_product_value_ad(1064, A::mul_sub_from_scalar_rhs(s.ad_value(1128), 1.0, s.ad_value(1121)), 1.0, 1105, 1121, 1.0);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1130, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1066), s.ad_value(1064)), 0.5, A::div(s.ad_value(1066), s.ad_value(1064)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1066), s.ad_value(1064))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1066), s.ad_value(1064)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1066), s.ad_value(1064))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul(1131, 1066, 1130);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1132, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul_neg_lhs(1133, 1066, 1132);
            s.store_div_scaled_inputs2_indices(1162, 1065, 1.0, 1163, (-1.0), 1099, 1.0);
        }

        s.b[1173] = (s.v[1162] > 50.0);
        s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1173]) {
            s.store_scalar(1104, 0.0);
        }

        s.b[1174] = (s.v[1162] < (-50.0));
        s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1173])) && s.b[1174]) {
            s.store_scalar(1104, 1.0);
        }

        if ((s.b[1055] && (!s.b[1173])) && (!s.b[1174])) {
            s.store_div_from_scalar_offset_ad(1104, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3_mixed_iiai(1107, 1165, 1.0, 1133, (-1.0), A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1104), (-(p.p51 * 0.1))), -1.0, 1105, 1.0);
        }

        s.b[1175] = (s.v[1107] > 50.0);
        s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1175]) {
            s.store_mul(1108, 1106, 1107);
        }

        s.b[1176] = (s.v[1107] < (-50.0));
        s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1175])) && s.b[1176]) {
            s.store_mul_exp_rhs(1108, 1106, 1107);
        }

        if ((s.b[1055] && (!s.b[1175])) && (!s.b[1176])) {
            s.store_mul_ln_one_plus_exp_rhs(1108, 1106, 1107);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_indices(1162, 1165, 1.0, 1163, (-1.0), 1099, 1.0);
        }

        s.b[1177] = (s.v[1162] > 50.0);
        s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1177]) {
            s.store_scalar(1134, 0.0);
        }

        s.b[1178] = (s.v[1162] < (-50.0));
        s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1177])) && s.b[1178]) {
            s.store_scalar(1134, 1.0);
        }

        if ((s.b[1055] && (!s.b[1177])) && (!s.b[1178])) {
            s.store_div_from_scalar_offset_ad(1134, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3_mixed_iiai(1135, 1065, 1.0, 1131, (-1.0), A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1134), (-(p.p51 * 0.1))), -1.0, 1105, 1.0);
        }

        s.b[1179] = (s.v[1135] > 50.0);
        s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1179]) {
            s.store_mul(1136, 1106, 1135);
        }

        s.b[1180] = (s.v[1135] < (-50.0));
        s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1179])) && s.b[1180]) {
            s.store_mul_exp_rhs(1136, 1106, 1135);
        }

        if ((s.b[1055] && (!s.b[1179])) && (!s.b[1180])) {
            s.store_mul_ln_one_plus_exp_rhs(1136, 1106, 1135);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_indices(1137, 1108, 1.0, 1136, (-1.0), 1076, 1.0);
            s.store_div(1163, 1137, 1129);
        }

        if s.b[1055] {
            s.store_div_ad_rhs(1138, 1163, A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(1163), A::tanh_scaled_input(s.ad_value(1163), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(s.ad_value(1163), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088))));
        }

        if s.b[1055] {
            s.store_mul(1139, 1111, 1138);
            s.store_mul_product3_mixed_iaai(1057, 1098, A::mul3(s.ad_value(1097), s.ad_value(1074), s.ad_value(1096)), A::add(s.ad_value(1108), s.ad_value(1136)), 1139, 0.5);
            s.store_div_scaled_inputs_indices(1112, 1081, 1.0, 1073, 2.302585092994046);
            s.store_scaled_mul(1114, 1112, 1073, 2.0);
            s.store_mul(1115, 1076, 1114);
            s.store_sub_scaled_inputs(1167, 1102, 1.0, 1099, (p.p51 * 0.5));
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_mixed_aii(1166, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1167, (-1.0), 1099, 1.0);
        }

        s.b[1181] = (s.v[1166] > 50.0);
        s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1181]) {
            s.store_scalar(1124, 0.0);
        }

        s.b[1182] = (s.v[1166] < (-50.0));
        s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1181])) && s.b[1182]) {
            s.store_scalar(1124, 1.0);
        }

        if ((s.b[1055] && (!s.b[1181])) && (!s.b[1182])) {
            s.store_div_from_scalar_offset_ad(1124, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_mixed_aai(1125, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1124), (-(p.p51 * 0.1))), (-1.0), 1114, 1.0);
        }

        s.b[1183] = (s.v[1125] > 50.0);
        s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1183]) {
            s.store_mul(1126, 1115, 1125);
        }

        s.b[1184] = (s.v[1125] < (-50.0));
        s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1183])) && s.b[1184]) {
            s.store_mul_exp_rhs(1126, 1115, 1125);
        }

        if ((s.b[1055] && (!s.b[1183])) && (!s.b[1184])) {
            s.store_mul_ln_one_plus_exp_rhs(1126, 1115, 1125);
        }

        if s.b[1055] {
            s.store_div(1118, 1087, 1120);
            s.store_mul_div_scaled_offset_numerator_rhs(1119, 1086, A::mul(s.ad_value(1094), s.ad_value(1072)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1094), s.ad_value(1071)), 1.0), 1.0);
            s.store_div_scaled_product_indices(1140, 1119, 1075, 1.0, 1118, 1.0);
            s.store_add_scaled_product_right_ad(1141, 1140, (-1.0), 1140, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1126), 2.0, s.ad_value(1076), s.ad_value(1140), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1142, A::mul_sub_from_scalar_rhs(s.ad_value(1141), 1.0, s.ad_value(1124)), 1.0, 1114, 1124, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_92(
        s: &mut Scratch,
        p: &Parameters,
        var_tnomk: f64,
    ) {
        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1143, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1066), s.ad_value(1142)), 0.5, A::div(s.ad_value(1066), s.ad_value(1142)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1066), s.ad_value(1142))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1066), s.ad_value(1142)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1066), s.ad_value(1142))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul(1144, 1066, 1143);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1145, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul_neg_lhs(1146, 1066, 1145);
            s.store_div_scaled_inputs2_indices(1166, 1065, 1.0, 1167, (-1.0), 1099, 1.0);
        }

        s.b[1185] = (s.v[1166] > 50.0);
        s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1185]) {
            s.store_scalar(1113, 0.0);
        }

        s.b[1186] = (s.v[1166] < (-50.0));
        s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1185])) && s.b[1186]) {
            s.store_scalar(1113, 1.0);
        }

        if ((s.b[1055] && (!s.b[1185])) && (!s.b[1186])) {
            s.store_div_from_scalar_offset_ad(1113, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3_mixed_iiai(1116, 1165, 1.0, 1146, (-1.0), A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1113), (-(p.p51 * 0.1))), -1.0, 1114, 1.0);
        }

        s.b[1187] = (s.v[1116] > 50.0);
        s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1187]) {
            s.store_mul(1117, 1115, 1116);
        }

        s.b[1188] = (s.v[1116] < (-50.0));
        s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1187])) && s.b[1188]) {
            s.store_mul_exp_rhs(1117, 1115, 1116);
        }

        if ((s.b[1055] && (!s.b[1187])) && (!s.b[1188])) {
            s.store_mul_ln_one_plus_exp_rhs(1117, 1115, 1116);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_indices(1166, 1165, 1.0, 1167, (-1.0), 1099, 1.0);
        }

        s.b[1189] = (s.v[1166] > 50.0);
        s.store_scalar(1189, if s.b[1189] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1189]) {
            s.store_scalar(1147, 0.0);
        }

        s.b[1190] = (s.v[1166] < (-50.0));
        s.store_scalar(1190, if s.b[1190] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1189])) && s.b[1190]) {
            s.store_scalar(1147, 1.0);
        }

        if ((s.b[1055] && (!s.b[1189])) && (!s.b[1190])) {
            s.store_div_from_scalar_offset_ad(1147, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3_mixed_iiai(1148, 1065, 1.0, 1144, (-1.0), A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1147), (-(p.p51 * 0.1))), -1.0, 1114, 1.0);
        }

        s.b[1191] = (s.v[1148] > 50.0);
        s.store_scalar(1191, if s.b[1191] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1191]) {
            s.store_mul(1149, 1115, 1148);
        }

        s.b[1192] = (s.v[1148] < (-50.0));
        s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1191])) && s.b[1192]) {
            s.store_mul_exp_rhs(1149, 1115, 1148);
        }

        if ((s.b[1055] && (!s.b[1191])) && (!s.b[1192])) {
            s.store_mul_ln_one_plus_exp_rhs(1149, 1115, 1148);
        }

        if s.b[1055] {
            s.store_offset_square(1150, 1117, 1e-38);
            s.store_offset_mul(1151, 1150, 1117, 1e-57);
            s.store_offset_square(1152, 1149, 1e-38);
            s.store_offset_mul(1153, 1152, 1149, 1e-57);
            s.store_offset_mul(1154, 1117, 1149, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1155, 1150, (2.0 / 3.0), 1152, (2.0 / 3.0), 1154, (2.0 / 3.0), A::offset(A::add(s.ad_value(1117), s.ad_value(1149)), 2e-19), 1.0);
            s.store_div_ad(1156, A::add_scaled_inputs_products(s.ad_value(1151), (2.0 * 2.0), s.ad_value(1153), (3.0 * 2.0), s.ad_value(1150), s.ad_value(1149), (4.0 * 2.0), s.ad_value(1152), s.ad_value(1117), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1150), 15.0, s.ad_value(1152), 15.0, s.ad_value(1154), (2.0 * 15.0)));
            s.store_sub(1157, 1155, 1156);
            s.copy_ad(1158, 1156);
            s.store_mul_product3_mixed_iaii(1058, 1098, A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1075)), 1097, 1157, 1.0);
            s.store_mul_product3_mixed_iaii(1059, 1098, A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1075)), 1097, 1158, 1.0);
        }

        s.b[1193] = (s.v[1067] == 1.0);
        s.store_scalar(1193, if s.b[1193] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1193]) {
            s.store_div_scaled_inputs3_indices(1159, 1068, 1.0, 1102, -1.0, 1099, (-(-(p.p51 * 0.5))), 1114, 1.0);
        }

        s.b[1194] = (s.v[1159] > 50.0);
        s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });

        if ((s.b[1055] && s.b[1193]) && s.b[1194]) {
            s.copy_ad(1162, 1159);
        }

        s.b[1195] = (s.v[1159] < (-50.0));
        s.store_scalar(1195, if s.b[1195] { 1.0 } else { 0.0 });

        if (((s.b[1055] && s.b[1193]) && (!s.b[1194])) && s.b[1195]) {
            s.store_exp(1162, 1159);
        }

        if (((s.b[1055] && s.b[1193]) && (!s.b[1194])) && (!s.b[1195])) {
            s.store_ln_one_plus_exp(1162, 1159);
        }

        if (s.b[1055] && s.b[1193]) {
            s.store_mul_ad_product_lhs_mixed_ai(1060, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1078), s.ad_value(1114)), 1162, 1098);
            s.store_div_scaled_inputs3_indices(1160, 1069, 1.0, 1102, -1.0, 1099, (-(-(p.p51 * 0.5))), 1114, 1.0);
        }

        s.b[1196] = (s.v[1160] > 50.0);
        s.store_scalar(1196, if s.b[1196] { 1.0 } else { 0.0 });

        if ((s.b[1055] && s.b[1193]) && s.b[1196]) {
            s.copy_ad(1162, 1160);
        }

        s.b[1197] = (s.v[1160] < (-50.0));
        s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });

        if (((s.b[1055] && s.b[1193]) && (!s.b[1196])) && s.b[1197]) {
            s.store_exp(1162, 1160);
        }

        if (((s.b[1055] && s.b[1193]) && (!s.b[1196])) && (!s.b[1197])) {
            s.store_ln_one_plus_exp(1162, 1160);
        }

        if (s.b[1055] && s.b[1193]) {
            s.store_mul_ad_product_lhs_mixed_ai(1061, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1079), s.ad_value(1114)), 1162, 1098);
        }

        if (s.b[1055] && (!s.b[1193])) {
            s.store_scalar(1060, 0.0);
            s.store_scalar(1061, 0.0);
        }

        s.b[1198] = (s.v[1070] == 1.0);
        s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1198]) {
            s.store_div_scaled_inputs3_indices(1161, 1065, 1.0, 1102, -1.0, 1099, (-(-(p.p51 * 0.5))), 1114, 1.0);
        }

        s.b[1199] = (s.v[1161] > 50.0);
        s.store_scalar(1199, if s.b[1199] { 1.0 } else { 0.0 });

        if ((s.b[1055] && s.b[1198]) && s.b[1199]) {
            s.copy_ad(1162, 1161);
        }

        s.b[1200] = (s.v[1161] < (-50.0));
        s.store_scalar(1200, if s.b[1200] { 1.0 } else { 0.0 });

        if (((s.b[1055] && s.b[1198]) && (!s.b[1199])) && s.b[1200]) {
            s.store_exp(1162, 1161);
        }

        if (((s.b[1055] && s.b[1198]) && (!s.b[1199])) && (!s.b[1200])) {
            s.store_ln_one_plus_exp(1162, 1161);
        }

        if (s.b[1055] && s.b[1198]) {
            s.store_mul_ad_product_lhs_mixed_ai(1062, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1077), s.ad_value(1114)), 1162, 1098);
        }

        if (s.b[1055] && (!s.b[1198])) {
            s.store_scalar(1062, 0.0);
        }

        if s.b[1055] {
            s.copy_ad(1056, 1057);
            s.copy_ad(172, 1057);
            s.copy_ad(173, 1058);
            s.copy_ad(174, 1059);
            s.copy_ad(175, 1060);
            s.copy_ad(176, 1061);
            s.copy_ad(177, 1062);
            s.copy_ad(172, 1056);
        }

        s.b[1201] = (p.p100 == 1.0);
        s.store_scalar(1201, if s.b[1201] { 1.0 } else { 0.0 });

        s.store_scalar(178, 0.0);

        s.store_scalar(179, 0.0);

        s.store_scalar(180, 0.0);

        s.store_scalar(181, 0.0);

        s.store_scalar(182, 0.0);

        s.store_scalar(183, 0.0);

        s.b[1202] = (p.p123 > p.p354);
        s.store_scalar(1202, if s.b[1202] { 1.0 } else { 0.0 });

        if s.b[1202] {
            s.store_scalar(1203, 0.0);
            s.store_scalar(1204, 0.0);
            s.store_scalar(1205, 0.0);
            s.store_scalar(1206, 0.0);
            s.store_scalar(1207, 0.0);
            s.store_scalar(1208, 0.0);
            s.store_scalar(1209, 0.0);
            s.store_scalar(1210, 0.0);
            s.store_scalar(1211, 0.0);
            s.copy_ad(1212, 72);
            s.copy_ad(1213, 73);
        }

        let (assign18910_e18477,) = {
    if s.b[1202] {
        (p.p129,)
    } else {
        (s.v[1214],)
    }
};
        s.store_scalar(1214, assign18910_e18477);

        if s.b[1202] {
            s.copy_ad(1215, 74);
            s.copy_ad(1216, 75);
        }

        let (assign18940_e18489,) = {
    if s.b[1202] {
        (p.p127,)
    } else {
        (s.v[1217],)
    }
};
        s.store_scalar(1217, assign18940_e18489);

        if s.b[1202] {
            s.copy_ad(1218, 111);
            s.store_scalar(1219, var_tnomk);
            s.copy_ad(1220, 113);
            s.store_scalar(1221, p.p0);
            s.store_scalar(1222, p.p123);
            s.copy_ad(1223, 26);
            s.store_scalar(1224, p.p128);
            s.copy_ad(1225, 27);
            s.copy_ad(1226, 28);
            s.store_scalar(1227, p.p124);
            s.store_scalar(1228, p.p138);
            s.store_scalar(1229, p.p137);
            s.store_scalar(1230, 0.0);
            s.store_scalar(1231, p.p139);
            s.store_scalar(1232, p.p143);
            s.store_scalar(1233, p.p134);
            s.store_scalar(1234, p.p135);
            s.store_scalar(1235, p.p136);
            s.store_scalar(1236, p.p142);
            s.store_scalar(1237, p.p141);
            s.store_scalar(1238, p.p140);
            s.store_scalar(1239, p.p39);
            s.store_scalar(1240, p.p47);
            s.store_scalar(1241, p.p45);
            s.store_scalar(1242, p.p42);
            s.store_scalar(1243, p.p2);
            s.store_scalar(1244, p.p6);
            s.store_scalar(1245, 1.0);
            s.store_scalar(1246, 0.0);
            s.store_scalar(1247, 0.0);
            s.store_scalar(1248, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_93(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1202] {
            s.store_scalar(1249, 0.0);
            s.store_scalar(1250, 0.0);
            s.store_scalar(1251, 0.0);
            s.store_scalar(1252, 0.0);
            s.store_scalar(1253, 0.0);
            s.store_scalar(1254, 0.0);
            s.store_scalar(1255, 0.0);
            s.store_scalar(1256, 0.0);
            s.store_scalar(1257, 0.0);
            s.store_scalar(1258, 0.0);
            s.store_scalar(1259, 0.0);
            s.store_scalar(1260, 0.0);
            s.store_scalar(1261, 0.0);
            s.store_scalar(1262, 0.0);
            s.store_scalar(1263, 0.0);
            s.store_scalar(1264, 0.0);
            s.store_scalar(1265, 0.0);
            s.store_scalar(1266, 0.0);
            s.store_scalar(1267, 0.0);
            s.store_scalar(1268, 0.0);
            s.store_scalar(1269, 0.0);
            s.store_scalar(1270, 0.0);
            s.store_scalar(1271, 0.0);
            s.store_scalar(1272, 0.0);
            s.store_scalar(1273, 0.0);
            s.store_scalar(1274, 0.0);
            s.store_scalar(1275, 0.0);
            s.store_scalar(1276, 0.0);
            s.store_scalar(1277, 0.0);
            s.store_scalar(1278, 0.0);
            s.store_scalar(1279, 0.0);
            s.store_scalar(1280, 0.0);
            s.store_scalar(1281, 0.0);
            s.store_scalar(1282, 0.0);
            s.store_scalar(1283, 0.0);
            s.store_scalar(1284, 0.0);
            s.store_scalar(1285, 0.0);
            s.store_scalar(1286, 0.0);
            s.store_scalar(1287, 0.0);
            s.store_scalar(1288, 0.0);
            s.store_scalar(1289, 0.0);
            s.store_scalar(1290, 0.0);
            s.store_scalar(1291, 0.0);
            s.store_scalar(1292, 0.0);
            s.store_scalar(1293, 0.0);
            s.store_scalar(1294, 0.0);
            s.store_scalar(1295, 0.0);
            s.store_scalar(1296, 0.0);
            s.store_scalar(1297, 0.0);
            s.store_scalar(1298, 0.0);
            s.store_scalar(1299, 0.0);
            s.store_scalar(1300, 0.0);
            s.store_scalar(1301, 0.0);
            s.store_scalar(1302, 0.0);
            s.store_scalar(1303, 0.0);
            s.store_scalar(1304, 0.0);
            s.store_scalar(1305, 0.0);
            s.store_scalar(1306, 0.0);
            s.store_scalar(1307, 0.0);
            s.store_scalar(1308, 0.0);
            s.store_scalar(1309, 0.0);
            s.store_scalar(1310, 0.0);
            s.store_scalar(1311, 0.0);
            s.store_scalar(1312, 0.0);
            s.store_scalar(1313, 0.0);
            s.store_scalar(1314, 0.0);
        }

        if s.b[1202] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1311, 1213, A::tanh_scaled_input(s.ad_value(1213), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1311, 1213, p.p53);
                } else {
                    s.store_scalar(1311, 0.0);
                }
            }
        }

        if s.b[1202] {
            s.store_sub(1312, 1212, 1213);
            s.store_mul(1246, 1232, 1220);
            s.store_add_scaled_product_value_ad(1248, A::div_scaled_inputs(s.ad_value(1228), 1.0, s.ad_value(1220), 2.302585092994046), 1.0, 1231, 1311, 1.0);
            s.store_add_scaled_product_right_sub(1249, 1227, 1.0, 1238, 1218, 1219, 1.0);
            s.store_pow_ad(1267, A::div(s.ad_value(1218), s.ad_value(1219)), s.ad_value(1240));
        }

        s.b[1315] = (s.v[1239] != 0.0);
        s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1315]) {
            s.store_div_ad_rhs(1250, 1311, A::pow(A::offset(A::pow(A::div(s.ad_value(1311), s.ad_value(1239)), s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235))));
        }

        if (s.b[1202] && (!s.b[1315])) {
            s.store_scalar(1250, 0.0);
        }

        if s.b[1202] {
            s.store_mul_add_scaled_product_rhs(1247, 1311, s.ad_value(1229), 1.0, s.ad_value(1250), s.ad_value(1230), (-1.0));
            s.store_sub(1210, 1249, 1247);
            s.store_scaled_mul(1252, 1248, 1220, 2.0);
            s.store_mul(1253, 1223, 1252);
            s.store_sub_scaled_inputs(1310, 1210, 1.0, 1246, (p.p51 * 0.5));
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_mixed_aii(1309, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1310, (-1.0), 1246, 1.0);
        }

        s.b[1316] = (s.v[1309] > 50.0);
        s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1316]) {
            s.store_scalar(1268, 0.0);
        }

        s.b[1317] = (s.v[1309] < (-50.0));
        s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1316])) && s.b[1317]) {
            s.store_scalar(1268, 1.0);
        }

        if ((s.b[1202] && (!s.b[1316])) && (!s.b[1317])) {
            s.store_div_from_scalar_offset_ad(1268, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_mixed_aai(1269, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1268), (-(p.p51 * 0.1))), (-1.0), 1252, 1.0);
        }

        s.b[1318] = (s.v[1269] > 50.0);
        s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1318]) {
            s.store_mul(1270, 1253, 1269);
        }

        s.b[1319] = (s.v[1269] < (-50.0));
        s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1318])) && s.b[1319]) {
            s.store_mul_exp_rhs(1270, 1253, 1269);
        }

        if ((s.b[1202] && (!s.b[1318])) && (!s.b[1319])) {
            s.store_mul_ln_one_plus_exp_rhs(1270, 1253, 1269);
        }

        if s.b[1202] {
            s.store_div_ad_rhs(1256, 1234, A::mul_offset_rhs(s.ad_value(1267), A::div_scaled_product(s.ad_value(1236), s.ad_value(1270), 1.0, s.ad_value(1223), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1257, 1233, A::div_scaled_offset_numerator(A::mul(s.ad_value(1241), s.ad_value(1219)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1241), s.ad_value(1218)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1242), s.ad_value(1311), 1.0, s.ad_value(1222), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1237), s.ad_value(1270), 1.0, s.ad_value(1223), 1.0), 1.0), 1.0);
            s.store_add_ad(1258, A::div_scaled_product3(s.ad_value(1268), s.ad_value(1220), s.ad_value(1256), 2.0, s.ad_value(1222), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1268), s.ad_value(1257)));
            s.store_div_scaled_product_indices(1274, 1257, 1222, 1.0, 1256, 1.0);
            s.store_add_scaled_product_right_ad(1275, 1274, (-1.0), 1274, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1270), 2.0, s.ad_value(1223), s.ad_value(1274), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1276, A::mul_sub_from_scalar_rhs(s.ad_value(1274), 1.0, s.ad_value(1268)), 1.0, 1252, 1268, 1.0);
            s.store_add_scaled_product_value_ad(1211, A::mul_sub_from_scalar_rhs(s.ad_value(1275), 1.0, s.ad_value(1268)), 1.0, 1252, 1268, 1.0);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1277, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1213), s.ad_value(1211)), 0.5, A::div(s.ad_value(1213), s.ad_value(1211)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1213), s.ad_value(1211))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1213), s.ad_value(1211)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1213), s.ad_value(1211))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul(1278, 1213, 1277);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1279, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul_neg_lhs(1280, 1213, 1279);
            s.store_div_scaled_inputs2_indices(1309, 1212, 1.0, 1310, (-1.0), 1246, 1.0);
        }

        s.b[1320] = (s.v[1309] > 50.0);
        s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1320]) {
            s.store_scalar(1251, 0.0);
        }

        s.b[1321] = (s.v[1309] < (-50.0));
        s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1320])) && s.b[1321]) {
            s.store_scalar(1251, 1.0);
        }

        if ((s.b[1202] && (!s.b[1320])) && (!s.b[1321])) {
            s.store_div_from_scalar_offset_ad(1251, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1254, 1312, 1.0, 1280, (-1.0), A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1251), (-(p.p51 * 0.1))), -1.0, 1252, 1.0);
        }

        s.b[1322] = (s.v[1254] > 50.0);
        s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1322]) {
            s.store_mul(1255, 1253, 1254);
        }

        s.b[1323] = (s.v[1254] < (-50.0));
        s.store_scalar(1323, if s.b[1323] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1322])) && s.b[1323]) {
            s.store_mul_exp_rhs(1255, 1253, 1254);
        }

        if ((s.b[1202] && (!s.b[1322])) && (!s.b[1323])) {
            s.store_mul_ln_one_plus_exp_rhs(1255, 1253, 1254);
        }

    }

    pub(super) fn stamp_transient_block_94(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1309, 1312, 1.0, 1310, (-1.0), 1246, 1.0);
        }

        s.b[1324] = (s.v[1309] > 50.0);
        s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1324]) {
            s.store_scalar(1281, 0.0);
        }

        s.b[1325] = (s.v[1309] < (-50.0));
        s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1324])) && s.b[1325]) {
            s.store_scalar(1281, 1.0);
        }

        if ((s.b[1202] && (!s.b[1324])) && (!s.b[1325])) {
            s.store_div_from_scalar_offset_ad(1281, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1282, 1212, 1.0, 1278, (-1.0), A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1281), (-(p.p51 * 0.1))), -1.0, 1252, 1.0);
        }

        s.b[1326] = (s.v[1282] > 50.0);
        s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1326]) {
            s.store_mul(1283, 1253, 1282);
        }

        s.b[1327] = (s.v[1282] < (-50.0));
        s.store_scalar(1327, if s.b[1327] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1326])) && s.b[1327]) {
            s.store_mul_exp_rhs(1283, 1253, 1282);
        }

        if ((s.b[1202] && (!s.b[1326])) && (!s.b[1327])) {
            s.store_mul_ln_one_plus_exp_rhs(1283, 1253, 1282);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1284, 1255, 1.0, 1283, (-1.0), 1223, 1.0);
            s.store_div(1310, 1284, 1276);
        }

        if s.b[1202] {
            s.store_div_ad_rhs(1285, 1310, A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(1310), A::tanh_scaled_input(s.ad_value(1310), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(s.ad_value(1310), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235))));
        }

        if s.b[1202] {
            s.store_mul(1286, 1258, 1285);
            s.store_mul_product3_mixed_iaai(1204, 1245, A::mul3(s.ad_value(1244), s.ad_value(1221), s.ad_value(1243)), A::add(s.ad_value(1255), s.ad_value(1283)), 1286, 0.5);
            s.store_div_scaled_inputs_indices(1259, 1228, 1.0, 1220, 2.302585092994046);
            s.store_scaled_mul(1261, 1259, 1220, 2.0);
            s.store_mul(1262, 1223, 1261);
            s.store_sub_scaled_inputs(1314, 1249, 1.0, 1246, (p.p51 * 0.5));
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_mixed_aii(1313, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1314, (-1.0), 1246, 1.0);
        }

        s.b[1328] = (s.v[1313] > 50.0);
        s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1328]) {
            s.store_scalar(1271, 0.0);
        }

        s.b[1329] = (s.v[1313] < (-50.0));
        s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1328])) && s.b[1329]) {
            s.store_scalar(1271, 1.0);
        }

        if ((s.b[1202] && (!s.b[1328])) && (!s.b[1329])) {
            s.store_div_from_scalar_offset_ad(1271, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_mixed_aai(1272, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1271), (-(p.p51 * 0.1))), (-1.0), 1261, 1.0);
        }

        s.b[1330] = (s.v[1272] > 50.0);
        s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1330]) {
            s.store_mul(1273, 1262, 1272);
        }

        s.b[1331] = (s.v[1272] < (-50.0));
        s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1330])) && s.b[1331]) {
            s.store_mul_exp_rhs(1273, 1262, 1272);
        }

        if ((s.b[1202] && (!s.b[1330])) && (!s.b[1331])) {
            s.store_mul_ln_one_plus_exp_rhs(1273, 1262, 1272);
        }

        if s.b[1202] {
            s.store_div(1265, 1234, 1267);
            s.store_mul_div_scaled_offset_numerator_rhs(1266, 1233, A::mul(s.ad_value(1241), s.ad_value(1219)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1241), s.ad_value(1218)), 1.0), 1.0);
            s.store_div_scaled_product_indices(1287, 1266, 1222, 1.0, 1265, 1.0);
            s.store_add_scaled_product_right_ad(1288, 1287, (-1.0), 1287, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1273), 2.0, s.ad_value(1223), s.ad_value(1287), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1289, A::mul_sub_from_scalar_rhs(s.ad_value(1288), 1.0, s.ad_value(1271)), 1.0, 1261, 1271, 1.0);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1290, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1213), s.ad_value(1289)), 0.5, A::div(s.ad_value(1213), s.ad_value(1289)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1213), s.ad_value(1289))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1213), s.ad_value(1289)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1213), s.ad_value(1289))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul(1291, 1213, 1290);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1292, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul_neg_lhs(1293, 1213, 1292);
            s.store_div_scaled_inputs2_indices(1313, 1212, 1.0, 1314, (-1.0), 1246, 1.0);
        }

        s.b[1332] = (s.v[1313] > 50.0);
        s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1332]) {
            s.store_scalar(1260, 0.0);
        }

        s.b[1333] = (s.v[1313] < (-50.0));
        s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1332])) && s.b[1333]) {
            s.store_scalar(1260, 1.0);
        }

        if ((s.b[1202] && (!s.b[1332])) && (!s.b[1333])) {
            s.store_div_from_scalar_offset_ad(1260, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1263, 1312, 1.0, 1293, (-1.0), A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1260), (-(p.p51 * 0.1))), -1.0, 1261, 1.0);
        }

        s.b[1334] = (s.v[1263] > 50.0);
        s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1334]) {
            s.store_mul(1264, 1262, 1263);
        }

        s.b[1335] = (s.v[1263] < (-50.0));
        s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1334])) && s.b[1335]) {
            s.store_mul_exp_rhs(1264, 1262, 1263);
        }

        if ((s.b[1202] && (!s.b[1334])) && (!s.b[1335])) {
            s.store_mul_ln_one_plus_exp_rhs(1264, 1262, 1263);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1313, 1312, 1.0, 1314, (-1.0), 1246, 1.0);
        }

        s.b[1336] = (s.v[1313] > 50.0);
        s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1336]) {
            s.store_scalar(1294, 0.0);
        }

        s.b[1337] = (s.v[1313] < (-50.0));
        s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1336])) && s.b[1337]) {
            s.store_scalar(1294, 1.0);
        }

        if ((s.b[1202] && (!s.b[1336])) && (!s.b[1337])) {
            s.store_div_from_scalar_offset_ad(1294, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1295, 1212, 1.0, 1291, (-1.0), A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1294), (-(p.p51 * 0.1))), -1.0, 1261, 1.0);
        }

        s.b[1338] = (s.v[1295] > 50.0);
        s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1338]) {
            s.store_mul(1296, 1262, 1295);
        }

        s.b[1339] = (s.v[1295] < (-50.0));
        s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1338])) && s.b[1339]) {
            s.store_mul_exp_rhs(1296, 1262, 1295);
        }

        if ((s.b[1202] && (!s.b[1338])) && (!s.b[1339])) {
            s.store_mul_ln_one_plus_exp_rhs(1296, 1262, 1295);
        }

        if s.b[1202] {
            s.store_offset_square(1297, 1264, 1e-38);
            s.store_offset_mul(1298, 1297, 1264, 1e-57);
            s.store_offset_square(1299, 1296, 1e-38);
            s.store_offset_mul(1300, 1299, 1296, 1e-57);
            s.store_offset_mul(1301, 1264, 1296, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1302, 1297, (2.0 / 3.0), 1299, (2.0 / 3.0), 1301, (2.0 / 3.0), A::offset(A::add(s.ad_value(1264), s.ad_value(1296)), 2e-19), 1.0);
            s.store_div_ad(1303, A::add_scaled_inputs_products(s.ad_value(1298), (2.0 * 2.0), s.ad_value(1300), (3.0 * 2.0), s.ad_value(1297), s.ad_value(1296), (4.0 * 2.0), s.ad_value(1299), s.ad_value(1264), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1297), 15.0, s.ad_value(1299), 15.0, s.ad_value(1301), (2.0 * 15.0)));
            s.store_sub(1304, 1302, 1303);
            s.copy_ad(1305, 1303);
            s.store_mul_product3_mixed_iaii(1205, 1245, A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1222)), 1244, 1304, 1.0);
            s.store_mul_product3_mixed_iaii(1206, 1245, A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1222)), 1244, 1305, 1.0);
        }

        s.b[1340] = (s.v[1214] == 1.0);
        s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1340]) {
            s.store_div_scaled_inputs3_indices(1306, 1215, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
        }

        s.b[1341] = (s.v[1306] > 50.0);
        s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });

        if ((s.b[1202] && s.b[1340]) && s.b[1341]) {
            s.copy_ad(1309, 1306);
        }

        s.b[1342] = (s.v[1306] < (-50.0));
        s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });

        if (((s.b[1202] && s.b[1340]) && (!s.b[1341])) && s.b[1342]) {
            s.store_exp(1309, 1306);
        }

        if (((s.b[1202] && s.b[1340]) && (!s.b[1341])) && (!s.b[1342])) {
            s.store_ln_one_plus_exp(1309, 1306);
        }

        if (s.b[1202] && s.b[1340]) {
            s.store_mul_ad_product_lhs_mixed_ai(1207, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1225), s.ad_value(1261)), 1309, 1245);
            s.store_div_scaled_inputs3_indices(1307, 1216, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
        }

        s.b[1343] = (s.v[1307] > 50.0);
        s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });

        if ((s.b[1202] && s.b[1340]) && s.b[1343]) {
            s.copy_ad(1309, 1307);
        }

        s.b[1344] = (s.v[1307] < (-50.0));
        s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });

        if (((s.b[1202] && s.b[1340]) && (!s.b[1343])) && s.b[1344]) {
            s.store_exp(1309, 1307);
        }

        if (((s.b[1202] && s.b[1340]) && (!s.b[1343])) && (!s.b[1344])) {
            s.store_ln_one_plus_exp(1309, 1307);
        }

        if (s.b[1202] && s.b[1340]) {
            s.store_mul_ad_product_lhs_mixed_ai(1208, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1226), s.ad_value(1261)), 1309, 1245);
        }

        if (s.b[1202] && (!s.b[1340])) {
            s.store_scalar(1207, 0.0);
            s.store_scalar(1208, 0.0);
        }

        s.b[1345] = (s.v[1217] == 1.0);
        s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1345]) {
            s.store_div_scaled_inputs3_indices(1308, 1212, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
        }

        s.b[1346] = (s.v[1308] > 50.0);
        s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });

        if ((s.b[1202] && s.b[1345]) && s.b[1346]) {
            s.copy_ad(1309, 1308);
        }

        s.b[1347] = (s.v[1308] < (-50.0));
        s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });

        if (((s.b[1202] && s.b[1345]) && (!s.b[1346])) && s.b[1347]) {
            s.store_exp(1309, 1308);
        }

        if (((s.b[1202] && s.b[1345]) && (!s.b[1346])) && (!s.b[1347])) {
            s.store_ln_one_plus_exp(1309, 1308);
        }

        if (s.b[1202] && s.b[1345]) {
            s.store_mul_ad_product_lhs_mixed_ai(1209, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1224), s.ad_value(1261)), 1309, 1245);
        }

        if (s.b[1202] && (!s.b[1345])) {
            s.store_scalar(1209, 0.0);
        }

        if s.b[1202] {
            s.copy_ad(1203, 1204);
            s.copy_ad(178, 1204);
            s.copy_ad(179, 1205);
            s.copy_ad(180, 1206);
            s.copy_ad(181, 1207);
            s.copy_ad(182, 1208);
            s.copy_ad(183, 1209);
            s.copy_ad(178, 1203);
        }

        s.b[1348] = (p.p122 == 1.0);
        s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });

        s.store_scalar(184, 0.0);

        s.store_scalar(185, 0.0);

        s.store_scalar(186, 0.0);

        s.store_scalar(187, 0.0);

        s.store_scalar(188, 0.0);

        s.store_scalar(189, 0.0);

        s.b[1349] = (p.p145 > p.p354);
        s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });

        if s.b[1349] {
            s.store_scalar(1350, 0.0);
            s.store_scalar(1351, 0.0);
            s.store_scalar(1352, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_95(
        s: &mut Scratch,
        p: &Parameters,
        var_tnomk: f64,
    ) {
        if s.b[1349] {
            s.store_scalar(1353, 0.0);
            s.store_scalar(1354, 0.0);
            s.store_scalar(1355, 0.0);
            s.store_scalar(1356, 0.0);
            s.store_scalar(1357, 0.0);
            s.store_scalar(1358, 0.0);
            s.copy_ad(1359, 78);
            s.copy_ad(1360, 79);
        }

        let (assign21750_e20901,) = {
    if s.b[1349] {
        (p.p151,)
    } else {
        (s.v[1361],)
    }
};
        s.store_scalar(1361, assign21750_e20901);

        if s.b[1349] {
            s.copy_ad(1362, 80);
            s.copy_ad(1363, 81);
        }

        let (assign21780_e20913,) = {
    if s.b[1349] {
        (p.p149,)
    } else {
        (s.v[1364],)
    }
};
        s.store_scalar(1364, assign21780_e20913);

        if s.b[1349] {
            s.copy_ad(1365, 111);
            s.store_scalar(1366, var_tnomk);
            s.copy_ad(1367, 113);
            s.store_scalar(1368, p.p0);
            s.store_scalar(1369, p.p145);
            s.copy_ad(1370, 29);
            s.store_scalar(1371, p.p150);
            s.copy_ad(1372, 30);
            s.copy_ad(1373, 31);
            s.store_scalar(1374, p.p146);
            s.store_scalar(1375, p.p160);
            s.store_scalar(1376, p.p159);
            s.store_scalar(1377, 0.0);
            s.store_scalar(1378, p.p161);
            s.store_scalar(1379, p.p165);
            s.store_scalar(1380, p.p156);
            s.store_scalar(1381, p.p157);
            s.store_scalar(1382, p.p158);
            s.store_scalar(1383, p.p164);
            s.store_scalar(1384, p.p163);
            s.store_scalar(1385, p.p162);
            s.store_scalar(1386, p.p39);
            s.store_scalar(1387, p.p47);
            s.store_scalar(1388, p.p45);
            s.store_scalar(1389, p.p42);
            s.store_scalar(1390, p.p2);
            s.store_scalar(1391, p.p6);
            s.store_scalar(1392, 1.0);
            s.store_scalar(1393, 0.0);
            s.store_scalar(1394, 0.0);
            s.store_scalar(1395, 0.0);
            s.store_scalar(1396, 0.0);
            s.store_scalar(1397, 0.0);
            s.store_scalar(1398, 0.0);
            s.store_scalar(1399, 0.0);
            s.store_scalar(1400, 0.0);
            s.store_scalar(1401, 0.0);
            s.store_scalar(1402, 0.0);
            s.store_scalar(1403, 0.0);
            s.store_scalar(1404, 0.0);
            s.store_scalar(1405, 0.0);
            s.store_scalar(1406, 0.0);
            s.store_scalar(1407, 0.0);
            s.store_scalar(1408, 0.0);
            s.store_scalar(1409, 0.0);
            s.store_scalar(1410, 0.0);
            s.store_scalar(1411, 0.0);
            s.store_scalar(1412, 0.0);
            s.store_scalar(1413, 0.0);
            s.store_scalar(1414, 0.0);
            s.store_scalar(1415, 0.0);
            s.store_scalar(1416, 0.0);
            s.store_scalar(1417, 0.0);
            s.store_scalar(1418, 0.0);
            s.store_scalar(1419, 0.0);
            s.store_scalar(1420, 0.0);
            s.store_scalar(1421, 0.0);
            s.store_scalar(1422, 0.0);
            s.store_scalar(1423, 0.0);
            s.store_scalar(1424, 0.0);
            s.store_scalar(1425, 0.0);
            s.store_scalar(1426, 0.0);
            s.store_scalar(1427, 0.0);
            s.store_scalar(1428, 0.0);
            s.store_scalar(1429, 0.0);
            s.store_scalar(1430, 0.0);
            s.store_scalar(1431, 0.0);
            s.store_scalar(1432, 0.0);
            s.store_scalar(1433, 0.0);
            s.store_scalar(1434, 0.0);
            s.store_scalar(1435, 0.0);
            s.store_scalar(1436, 0.0);
            s.store_scalar(1437, 0.0);
            s.store_scalar(1438, 0.0);
            s.store_scalar(1439, 0.0);
            s.store_scalar(1440, 0.0);
            s.store_scalar(1441, 0.0);
            s.store_scalar(1442, 0.0);
            s.store_scalar(1443, 0.0);
            s.store_scalar(1444, 0.0);
            s.store_scalar(1445, 0.0);
            s.store_scalar(1446, 0.0);
            s.store_scalar(1447, 0.0);
            s.store_scalar(1448, 0.0);
            s.store_scalar(1449, 0.0);
            s.store_scalar(1450, 0.0);
            s.store_scalar(1451, 0.0);
            s.store_scalar(1452, 0.0);
            s.store_scalar(1453, 0.0);
            s.store_scalar(1454, 0.0);
            s.store_scalar(1455, 0.0);
            s.store_scalar(1456, 0.0);
            s.store_scalar(1457, 0.0);
            s.store_scalar(1458, 0.0);
            s.store_scalar(1459, 0.0);
            s.store_scalar(1460, 0.0);
            s.store_scalar(1461, 0.0);
        }

        if s.b[1349] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1458, 1360, A::tanh_scaled_input(s.ad_value(1360), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1458, 1360, p.p53);
                } else {
                    s.store_scalar(1458, 0.0);
                }
            }
        }

        if s.b[1349] {
            s.store_sub(1459, 1359, 1360);
            s.store_mul(1393, 1379, 1367);
            s.store_add_scaled_product_value_ad(1395, A::div_scaled_inputs(s.ad_value(1375), 1.0, s.ad_value(1367), 2.302585092994046), 1.0, 1378, 1458, 1.0);
            s.store_add_scaled_product_right_sub(1396, 1374, 1.0, 1385, 1365, 1366, 1.0);
            s.store_pow_ad(1414, A::div(s.ad_value(1365), s.ad_value(1366)), s.ad_value(1387));
        }

        s.b[1462] = (s.v[1386] != 0.0);
        s.store_scalar(1462, if s.b[1462] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1462]) {
            s.store_div_ad_rhs(1397, 1458, A::pow(A::offset(A::pow(A::div(s.ad_value(1458), s.ad_value(1386)), s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382))));
        }

        if (s.b[1349] && (!s.b[1462])) {
            s.store_scalar(1397, 0.0);
        }

        if s.b[1349] {
            s.store_mul_add_scaled_product_rhs(1394, 1458, s.ad_value(1376), 1.0, s.ad_value(1397), s.ad_value(1377), (-1.0));
            s.store_sub(1357, 1396, 1394);
            s.store_scaled_mul(1399, 1395, 1367, 2.0);
            s.store_mul(1400, 1370, 1399);
        }

    }
}
