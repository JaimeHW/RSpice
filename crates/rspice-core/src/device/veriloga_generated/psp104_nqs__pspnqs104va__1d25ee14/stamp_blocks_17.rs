#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard41: f64,
        var_iae: f64,
        var_iiwcv: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfbedge_p_rv_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfdedge_p_rv_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfedge_p_rv_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cfrd_p_rv_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ct_p_rv_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_ctedge_p_rv_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_dphib_p_rv_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_dphibedge_p_rv_slot: &mut f64,
        var_fnt_p_slot: &mut f64,
        var_fnt_p_rv_slot: &mut f64,
        var_gfacnud_p_slot: &mut f64,
        var_gfacnud_p_rv_slot: &mut f64,
        var_gpe_edge_slot: &mut f64,
        var_gpe_edge_rv_slot: &mut f64,
        var_guard56_slot: &mut f64,
        var_guard56_rv_slot: &mut f64,
        var_guard57_slot: &mut f64,
        var_guard57_rv_slot: &mut f64,
        var_guard58_slot: &mut f64,
        var_guard58_rv_slot: &mut f64,
        var_guard59_slot: &mut f64,
        var_guard59_rv_slot: &mut f64,
        var_guard60_slot: &mut f64,
        var_guard60_rv_slot: &mut f64,
        var_guard61_slot: &mut f64,
        var_guard61_rv_slot: &mut f64,
        var_guard62_slot: &mut f64,
        var_guard62_rv_slot: &mut f64,
        var_guard63_slot: &mut f64,
        var_guard63_rv_slot: &mut f64,
        var_guard64_slot: &mut f64,
        var_guard64_rv_slot: &mut f64,
        var_guard65_slot: &mut f64,
        var_guard65_rv_slot: &mut f64,
        var_guard66_slot: &mut f64,
        var_guard66_rv_slot: &mut f64,
        var_kuowe_slot: &mut f64,
        var_kuowe_rv_slot: &mut f64,
        var_kvthowe_slot: &mut f64,
        var_kvthowe_rv_slot: &mut f64,
        var_munqs_p_slot: &mut f64,
        var_munqs_p_rv_slot: &mut f64,
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
        var_pscebedge_p_slot: &mut f64,
        var_pscebedge_p_rv_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_pscededge_p_rv_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_psceedge_p_rv_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_stbetedge_p_rv_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
        var_stvfb_p_rv_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_stvfbedge_p_rv_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_temp0_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
        var_vsbnud_p_rv_slot: &mut f64,
        var_we_edge_slot: &mut f64,
        var_we_edge_rv_slot: &mut f64,
    ) {
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfbedge_p_rv: f64 = *var_cfbedge_p_rv_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfdedge_p_rv: f64 = *var_cfdedge_p_rv_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfedge_p_rv: f64 = *var_cfedge_p_rv_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cfrd_p_rv: f64 = *var_cfrd_p_rv_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ct_p_rv: f64 = *var_ct_p_rv_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_ctedge_p_rv: f64 = *var_ctedge_p_rv_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_dphib_p_rv: f64 = *var_dphib_p_rv_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_dphibedge_p_rv: f64 = *var_dphibedge_p_rv_slot;
        let mut var_fnt_p: f64 = *var_fnt_p_slot;
        let mut var_fnt_p_rv: f64 = *var_fnt_p_rv_slot;
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_gfacnud_p_rv: f64 = *var_gfacnud_p_rv_slot;
        let mut var_gpe_edge: f64 = *var_gpe_edge_slot;
        let mut var_gpe_edge_rv: f64 = *var_gpe_edge_rv_slot;
        let mut var_guard56: f64 = *var_guard56_slot;
        let mut var_guard56_rv: f64 = *var_guard56_rv_slot;
        let mut var_guard57: f64 = *var_guard57_slot;
        let mut var_guard57_rv: f64 = *var_guard57_rv_slot;
        let mut var_guard58: f64 = *var_guard58_slot;
        let mut var_guard58_rv: f64 = *var_guard58_rv_slot;
        let mut var_guard59: f64 = *var_guard59_slot;
        let mut var_guard59_rv: f64 = *var_guard59_rv_slot;
        let mut var_guard60: f64 = *var_guard60_slot;
        let mut var_guard60_rv: f64 = *var_guard60_rv_slot;
        let mut var_guard61: f64 = *var_guard61_slot;
        let mut var_guard61_rv: f64 = *var_guard61_rv_slot;
        let mut var_guard62: f64 = *var_guard62_slot;
        let mut var_guard62_rv: f64 = *var_guard62_rv_slot;
        let mut var_guard63: f64 = *var_guard63_slot;
        let mut var_guard63_rv: f64 = *var_guard63_rv_slot;
        let mut var_guard64: f64 = *var_guard64_slot;
        let mut var_guard64_rv: f64 = *var_guard64_rv_slot;
        let mut var_guard65: f64 = *var_guard65_slot;
        let mut var_guard65_rv: f64 = *var_guard65_rv_slot;
        let mut var_guard66: f64 = *var_guard66_slot;
        let mut var_guard66_rv: f64 = *var_guard66_rv_slot;
        let mut var_kuowe: f64 = *var_kuowe_slot;
        let mut var_kuowe_rv: f64 = *var_kuowe_rv_slot;
        let mut var_kvthowe: f64 = *var_kvthowe_slot;
        let mut var_kvthowe_rv: f64 = *var_kvthowe_rv_slot;
        let mut var_munqs_p: f64 = *var_munqs_p_slot;
        let mut var_munqs_p_rv: f64 = *var_munqs_p_rv_slot;
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
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_pscebedge_p_rv: f64 = *var_pscebedge_p_rv_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_pscededge_p_rv: f64 = *var_pscededge_p_rv_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_psceedge_p_rv: f64 = *var_psceedge_p_rv_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stbetedge_p_rv: f64 = *var_stbetedge_p_rv_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_stvfb_p_rv: f64 = *var_stvfb_p_rv_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_stvfbedge_p_rv: f64 = *var_stvfbedge_p_rv_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_temp0_rv: f64 = *var_temp0_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;
        let mut var_vsbnud_p_rv: f64 = *var_vsbnud_p_rv_slot;
        let mut var_we_edge: f64 = *var_we_edge_slot;
        let mut var_we_edge_rv: f64 = *var_we_edge_rv_slot;

        let (assign6860_e5342,) = {
    if (var_guard41 != 0.0) {
        let assign6860_e5340: f64 = (p.p388 * var_iiwcv);
        (assign6860_e5340,)
    } else {
        (var_cfrd_p,)
    }
};
        var_cfrd_p = assign6860_e5342;
        var_cfrd_p_rv = 0.0;

        let (assign6870_e5352,) = {
    if (var_guard41 != 0.0) {
        let assign6870_e5347: f64 = (2.0 * p.p395);
        let assign6870_e5349: f64 = (assign6870_e5347 / var_le);
        let assign6870_e5350: f64 = (1.0 - assign6870_e5349);
        (assign6870_e5350,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign6870_e5352;
        var_temp0_rv = 0.0;

        let (assign6900_e5373,) = {
    if (var_guard41 != 0.0) {
        (p.p389,)
    } else {
        (var_fnt_p,)
    }
};
        var_fnt_p = assign6900_e5373;
        var_fnt_p_rv = 0.0;

        let (assign6960_e5423,) = {
    if (var_guard41 != 0.0) {
        let assign6960_e5417: f64 = (2.0 * p.p397);
        let assign6960_e5420: f64 = (p.p398 * var_we);
        let assign6960_e5421: f64 = (assign6960_e5417 + assign6960_e5420);
        (assign6960_e5421,)
    } else {
        (var_we_edge,)
    }
};
        var_we_edge = assign6960_e5423;
        var_we_edge_rv = 0.0;

        let (assign6990_e5439,) = {
    if (var_guard41 != 0.0) {
        (p.p399,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign6990_e5439;
        var_vfbedge_p_rv = 0.0;

        let (assign7000_e5455,) = {
    if (var_guard41 != 0.0) {
        let assign7000_e5444: f64 = (p.p401 * var_ile);
        let assign7000_e5445: f64 = (p.p400 + assign7000_e5444);
        let assign7000_e5448: f64 = (p.p402 * var_iwe);
        let assign7000_e5449: f64 = (assign7000_e5445 + assign7000_e5448);
        let assign7000_e5452: f64 = (p.p403 * var_iae);
        let assign7000_e5453: f64 = (assign7000_e5449 + assign7000_e5452);
        (assign7000_e5453,)
    } else {
        (var_stvfbedge_p,)
    }
};
        var_stvfbedge_p = assign7000_e5455;
        var_stvfbedge_p_rv = 0.0;

        let (assign7010_e5473,) = {
    if (var_guard41 != 0.0) {
        let assign7010_e5461: f64 = (var_ile).powf(p.p406);
        let assign7010_e5462: f64 = (p.p405 * assign7010_e5461);
        let assign7010_e5463: f64 = (p.p404 + assign7010_e5462);
        let assign7010_e5466: f64 = (p.p407 * var_iwe);
        let assign7010_e5467: f64 = (assign7010_e5463 + assign7010_e5466);
        let assign7010_e5470: f64 = (p.p408 * var_iae);
        let assign7010_e5471: f64 = (assign7010_e5467 + assign7010_e5470);
        (assign7010_e5471,)
    } else {
        (var_dphibedge_p,)
    }
};
        var_dphibedge_p = assign7010_e5473;
        var_dphibedge_p_rv = 0.0;

        let (assign7020_e5497,) = {
    if (var_guard41 != 0.0) {
        let assign7020_e5480: f64 = (var_ile).powf(p.p411);
        let assign7020_e5481: f64 = (p.p410 * assign7020_e5480);
        let assign7020_e5482: f64 = (1.0 + assign7020_e5481);
        let assign7020_e5483: f64 = (p.p409 * assign7020_e5482);
        let assign7020_e5487: f64 = (p.p412 * var_iwe);
        let assign7020_e5488: f64 = (1.0 + assign7020_e5487);
        let assign7020_e5489: f64 = (assign7020_e5483 * assign7020_e5488);
        let assign7020_e5493: f64 = (p.p413 * var_iae);
        let assign7020_e5494: f64 = (1.0 + assign7020_e5493);
        let assign7020_e5495: f64 = (assign7020_e5489 * assign7020_e5494);
        (assign7020_e5495,)
    } else {
        (var_neffedge_p,)
    }
};
        var_neffedge_p = assign7020_e5497;
        var_neffedge_p_rv = 0.0;

        let (assign7030_e5507,) = {
    if (var_guard41 != 0.0) {
        let assign7030_e5503: f64 = (var_ile).powf(p.p416);
        let assign7030_e5504: f64 = (p.p415 * assign7030_e5503);
        let assign7030_e5505: f64 = (p.p414 + assign7030_e5504);
        (assign7030_e5505,)
    } else {
        (var_ctedge_p,)
    }
};
        var_ctedge_p = assign7030_e5507;
        var_ctedge_p_rv = 0.0;

        let (assign7040_e5525,) = {
    if (var_guard41 != 0.0) {
        let assign7040_e5512: f64 = (p.p417 * p.p418);
        let assign7040_e5514: f64 = (assign7040_e5512 / var_le);
        let assign7040_e5517: f64 = (-var_le);
        let assign7040_e5519: f64 = (assign7040_e5517 / p.p418);
        let assign7040_e5520: f64 = (assign7040_e5519).exp();
        let assign7040_e5521: f64 = (1.0 - assign7040_e5520);
        let assign7040_e5522: f64 = (assign7040_e5514 * assign7040_e5521);
        let assign7040_e5523: f64 = (1.0 + assign7040_e5522);
        (assign7040_e5523,)
    } else {
        (var_gpe_edge,)
    }
};
        var_gpe_edge = assign7040_e5525;
        var_gpe_edge_rv = 0.0;

        let (assign7050_e5534,) = {
    if (var_guard41 != 0.0) {
        let (assign7050_e5532,) = {
            if (var_gpe_edge > 1e-15) {
                (var_gpe_edge,)
            } else {
                (1e-15,)
            }
        };
        (assign7050_e5532,)
    } else {
        (var_gpe_edge,)
    }
};
        var_gpe_edge = assign7050_e5534;
        var_gpe_edge_rv = 0.0;

        let (assign7060_e5550,) = {
    if (var_guard41 != 0.0) {
        let assign7060_e5538: f64 = (p.p258 * var_we_edge);
        let assign7060_e5541: f64 = (var_gpe_edge * var_le);
        let assign7060_e5542: f64 = (assign7060_e5538 / assign7060_e5541);
        let assign7060_e5546: f64 = (p.p419 * var_iwe);
        let assign7060_e5547: f64 = (1.0 + assign7060_e5546);
        let assign7060_e5548: f64 = (assign7060_e5542 * assign7060_e5547);
        (assign7060_e5548,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign7060_e5550;
        var_betnedge_p_rv = 0.0;

        let (assign7070_e5566,) = {
    if (var_guard41 != 0.0) {
        let assign7070_e5555: f64 = (p.p421 * var_ile);
        let assign7070_e5556: f64 = (p.p420 + assign7070_e5555);
        let assign7070_e5559: f64 = (p.p422 * var_iwe);
        let assign7070_e5560: f64 = (assign7070_e5556 + assign7070_e5559);
        let assign7070_e5563: f64 = (p.p423 * var_iae);
        let assign7070_e5564: f64 = (assign7070_e5560 + assign7070_e5563);
        (assign7070_e5564,)
    } else {
        (var_stbetedge_p,)
    }
};
        var_stbetedge_p = assign7070_e5566;
        var_stbetedge_p_rv = 0.0;

        let (assign7080_e5580,) = {
    if (var_guard41 != 0.0) {
        let assign7080_e5571: f64 = (var_ile).powf(p.p425);
        let assign7080_e5572: f64 = (p.p424 * assign7080_e5571);
        let assign7080_e5576: f64 = (p.p426 * var_iwe);
        let assign7080_e5577: f64 = (1.0 + assign7080_e5576);
        let assign7080_e5578: f64 = (assign7080_e5572 * assign7080_e5577);
        (assign7080_e5578,)
    } else {
        (var_psceedge_p,)
    }
};
        var_psceedge_p = assign7080_e5580;
        var_psceedge_p_rv = 0.0;

        let (assign7090_e5584,) = {
    if (var_guard41 != 0.0) {
        (p.p427,)
    } else {
        (var_pscebedge_p,)
    }
};
        var_pscebedge_p = assign7090_e5584;
        var_pscebedge_p_rv = 0.0;

        let (assign7100_e5588,) = {
    if (var_guard41 != 0.0) {
        (p.p428,)
    } else {
        (var_pscededge_p,)
    }
};
        var_pscededge_p = assign7100_e5588;
        var_pscededge_p_rv = 0.0;

        let (assign7110_e5602,) = {
    if (var_guard41 != 0.0) {
        let assign7110_e5593: f64 = (var_ile).powf(p.p430);
        let assign7110_e5594: f64 = (p.p429 * assign7110_e5593);
        let assign7110_e5598: f64 = (p.p431 * var_iwe);
        let assign7110_e5599: f64 = (1.0 + assign7110_e5598);
        let assign7110_e5600: f64 = (assign7110_e5594 * assign7110_e5599);
        (assign7110_e5600,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign7110_e5602;
        var_cfedge_p_rv = 0.0;

        let (assign7120_e5606,) = {
    if (var_guard41 != 0.0) {
        (p.p433,)
    } else {
        (var_cfdedge_p,)
    }
};
        var_cfdedge_p = assign7120_e5606;
        var_cfdedge_p_rv = 0.0;

        let (assign7130_e5610,) = {
    if (var_guard41 != 0.0) {
        (p.p432,)
    } else {
        (var_cfbedge_p,)
    }
};
        var_cfbedge_p = assign7130_e5610;
        var_cfbedge_p_rv = 0.0;

        let (assign7190_e5652,) = {
    if (var_guard41 != 0.0) {
        let assign7190_e5641: f64 = (p.p815 * var_ile);
        let assign7190_e5642: f64 = (p.p814 + assign7190_e5641);
        let assign7190_e5645: f64 = (p.p816 * var_iwe);
        let assign7190_e5646: f64 = (assign7190_e5642 + assign7190_e5645);
        let assign7190_e5649: f64 = (p.p817 * var_iae);
        let assign7190_e5650: f64 = (assign7190_e5646 + assign7190_e5649);
        (assign7190_e5650,)
    } else {
        (var_kvthowe,)
    }
};
        var_kvthowe = assign7190_e5652;
        var_kvthowe_rv = 0.0;

        let (assign7200_e5668,) = {
    if (var_guard41 != 0.0) {
        let assign7200_e5657: f64 = (p.p819 * var_ile);
        let assign7200_e5658: f64 = (p.p818 + assign7200_e5657);
        let assign7200_e5661: f64 = (p.p820 * var_iwe);
        let assign7200_e5662: f64 = (assign7200_e5658 + assign7200_e5661);
        let assign7200_e5665: f64 = (p.p821 * var_iae);
        let assign7200_e5666: f64 = (assign7200_e5662 + assign7200_e5665);
        (assign7200_e5666,)
    } else {
        (var_kuowe,)
    }
};
        var_kuowe = assign7200_e5668;
        var_kuowe_rv = 0.0;

        let (assign7320_e5767,) = {
    if (var_guard41 != 0.0) {
        (p.p450,)
    } else {
        (var_munqs_p,)
    }
};
        var_munqs_p = assign7320_e5767;
        var_munqs_p_rv = 0.0;

        let assign7330_e5786: f64 = if (((param_given[451] || param_given[452]) || param_given[453]) || param_given[454]) { 1.0 } else { 0.0 };
        var_guard56 = assign7330_e5786;
        var_guard56_rv = 0.0;

        let (assign7340_e5804,) = {
    if ((var_guard41 != 0.0) && (var_guard56 != 0.0)) {
        let assign7340_e5793: f64 = (p.p452 * var_ile);
        let assign7340_e5794: f64 = (p.p451 + assign7340_e5793);
        let assign7340_e5797: f64 = (p.p453 * var_iwe);
        let assign7340_e5798: f64 = (assign7340_e5794 + assign7340_e5797);
        let assign7340_e5801: f64 = (p.p454 * var_iae);
        let assign7340_e5802: f64 = (assign7340_e5798 + assign7340_e5801);
        (assign7340_e5802,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign7340_e5804;
        var_vfb_p_rv = 0.0;

        let assign7350_e5823: f64 = if (((param_given[455] || param_given[456]) || param_given[457]) || param_given[458]) { 1.0 } else { 0.0 };
        var_guard57 = assign7350_e5823;
        var_guard57_rv = 0.0;

        let (assign7360_e5841,) = {
    if ((var_guard41 != 0.0) && (var_guard57 != 0.0)) {
        let assign7360_e5830: f64 = (p.p456 * var_ile);
        let assign7360_e5831: f64 = (p.p455 + assign7360_e5830);
        let assign7360_e5834: f64 = (p.p457 * var_iwe);
        let assign7360_e5835: f64 = (assign7360_e5831 + assign7360_e5834);
        let assign7360_e5838: f64 = (p.p458 * var_iae);
        let assign7360_e5839: f64 = (assign7360_e5835 + assign7360_e5838);
        (assign7360_e5839,)
    } else {
        (var_stvfb_p,)
    }
};
        var_stvfb_p = assign7360_e5841;
        var_stvfb_p_rv = 0.0;

        let assign7370_e5860: f64 = if (((param_given[459] || param_given[460]) || param_given[461]) || param_given[462]) { 1.0 } else { 0.0 };
        var_guard58 = assign7370_e5860;
        var_guard58_rv = 0.0;

        let (assign7380_e5878,) = {
    if ((var_guard41 != 0.0) && (var_guard58 != 0.0)) {
        let assign7380_e5867: f64 = (p.p460 * var_ile);
        let assign7380_e5868: f64 = (p.p459 + assign7380_e5867);
        let assign7380_e5871: f64 = (p.p461 * var_iwe);
        let assign7380_e5872: f64 = (assign7380_e5868 + assign7380_e5871);
        let assign7380_e5875: f64 = (p.p462 * var_iae);
        let assign7380_e5876: f64 = (assign7380_e5872 + assign7380_e5875);
        (assign7380_e5876,)
    } else {
        (var_neff_p,)
    }
};
        var_neff_p = assign7380_e5878;
        var_neff_p_rv = 0.0;

        let assign7390_e5897: f64 = if (((param_given[463] || param_given[464]) || param_given[465]) || param_given[466]) { 1.0 } else { 0.0 };
        var_guard59 = assign7390_e5897;
        var_guard59_rv = 0.0;

        let (assign7400_e5915,) = {
    if ((var_guard41 != 0.0) && (var_guard59 != 0.0)) {
        let assign7400_e5904: f64 = (p.p464 * var_ile);
        let assign7400_e5905: f64 = (p.p463 + assign7400_e5904);
        let assign7400_e5908: f64 = (p.p465 * var_iwe);
        let assign7400_e5909: f64 = (assign7400_e5905 + assign7400_e5908);
        let assign7400_e5912: f64 = (p.p466 * var_iae);
        let assign7400_e5913: f64 = (assign7400_e5909 + assign7400_e5912);
        (assign7400_e5913,)
    } else {
        (var_gfacnud_p,)
    }
};
        var_gfacnud_p = assign7400_e5915;
        var_gfacnud_p_rv = 0.0;

        let assign7410_e5934: f64 = if (((param_given[467] || param_given[468]) || param_given[469]) || param_given[470]) { 1.0 } else { 0.0 };
        var_guard60 = assign7410_e5934;
        var_guard60_rv = 0.0;

        let (assign7420_e5952,) = {
    if ((var_guard41 != 0.0) && (var_guard60 != 0.0)) {
        let assign7420_e5941: f64 = (p.p468 * var_ile);
        let assign7420_e5942: f64 = (p.p467 + assign7420_e5941);
        let assign7420_e5945: f64 = (p.p469 * var_iwe);
        let assign7420_e5946: f64 = (assign7420_e5942 + assign7420_e5945);
        let assign7420_e5949: f64 = (p.p470 * var_iae);
        let assign7420_e5950: f64 = (assign7420_e5946 + assign7420_e5949);
        (assign7420_e5950,)
    } else {
        (var_vsbnud_p,)
    }
};
        var_vsbnud_p = assign7420_e5952;
        var_vsbnud_p_rv = 0.0;

        let assign7430_e5971: f64 = if (((param_given[471] || param_given[472]) || param_given[473]) || param_given[474]) { 1.0 } else { 0.0 };
        var_guard61 = assign7430_e5971;
        var_guard61_rv = 0.0;

        let (assign7440_e5989,) = {
    if ((var_guard41 != 0.0) && (var_guard61 != 0.0)) {
        let assign7440_e5978: f64 = (p.p472 * var_ile);
        let assign7440_e5979: f64 = (p.p471 + assign7440_e5978);
        let assign7440_e5982: f64 = (p.p473 * var_iwe);
        let assign7440_e5983: f64 = (assign7440_e5979 + assign7440_e5982);
        let assign7440_e5986: f64 = (p.p474 * var_iae);
        let assign7440_e5987: f64 = (assign7440_e5983 + assign7440_e5986);
        (assign7440_e5987,)
    } else {
        (var_dphib_p,)
    }
};
        var_dphib_p = assign7440_e5989;
        var_dphib_p_rv = 0.0;

        let assign7450_e6008: f64 = if (((param_given[475] || param_given[476]) || param_given[477]) || param_given[478]) { 1.0 } else { 0.0 };
        var_guard62 = assign7450_e6008;
        var_guard62_rv = 0.0;

        let (assign7460_e6026,) = {
    if ((var_guard41 != 0.0) && (var_guard62 != 0.0)) {
        let assign7460_e6015: f64 = (p.p476 * var_ile);
        let assign7460_e6016: f64 = (p.p475 + assign7460_e6015);
        let assign7460_e6019: f64 = (p.p477 * var_iwe);
        let assign7460_e6020: f64 = (assign7460_e6016 + assign7460_e6019);
        let assign7460_e6023: f64 = (p.p478 * var_iae);
        let assign7460_e6024: f64 = (assign7460_e6020 + assign7460_e6023);
        (assign7460_e6024,)
    } else {
        (var_np_p,)
    }
};
        var_np_p = assign7460_e6026;
        var_np_p_rv = 0.0;

        let assign7470_e6045: f64 = if (((param_given[479] || param_given[480]) || param_given[481]) || param_given[482]) { 1.0 } else { 0.0 };
        var_guard63 = assign7470_e6045;
        var_guard63_rv = 0.0;

        let (assign7480_e6063,) = {
    if ((var_guard41 != 0.0) && (var_guard63 != 0.0)) {
        let assign7480_e6052: f64 = (p.p480 * var_ile);
        let assign7480_e6053: f64 = (p.p479 + assign7480_e6052);
        let assign7480_e6056: f64 = (p.p481 * var_iwe);
        let assign7480_e6057: f64 = (assign7480_e6053 + assign7480_e6056);
        let assign7480_e6060: f64 = (p.p482 * var_iae);
        let assign7480_e6061: f64 = (assign7480_e6057 + assign7480_e6060);
        (assign7480_e6061,)
    } else {
        (var_nov_p,)
    }
};
        var_nov_p = assign7480_e6063;
        var_nov_p_rv = 0.0;

        let assign7490_e6082: f64 = if (((param_given[483] || param_given[484]) || param_given[485]) || param_given[486]) { 1.0 } else { 0.0 };
        var_guard64 = assign7490_e6082;
        var_guard64_rv = 0.0;

        let (assign7500_e6100,) = {
    if ((var_guard41 != 0.0) && (var_guard64 != 0.0)) {
        let assign7500_e6089: f64 = (p.p484 * var_ile);
        let assign7500_e6090: f64 = (p.p483 + assign7500_e6089);
        let assign7500_e6093: f64 = (p.p485 * var_iwe);
        let assign7500_e6094: f64 = (assign7500_e6090 + assign7500_e6093);
        let assign7500_e6097: f64 = (p.p486 * var_iae);
        let assign7500_e6098: f64 = (assign7500_e6094 + assign7500_e6097);
        (assign7500_e6098,)
    } else {
        (var_novd_p,)
    }
};
        var_novd_p = assign7500_e6100;
        var_novd_p_rv = 0.0;

        let assign7510_e6119: f64 = if (((param_given[487] || param_given[488]) || param_given[489]) || param_given[490]) { 1.0 } else { 0.0 };
        var_guard65 = assign7510_e6119;
        var_guard65_rv = 0.0;

        let (assign7520_e6137,) = {
    if ((var_guard41 != 0.0) && (var_guard65 != 0.0)) {
        let assign7520_e6126: f64 = (p.p488 * var_ile);
        let assign7520_e6127: f64 = (p.p487 + assign7520_e6126);
        let assign7520_e6130: f64 = (p.p489 * var_iwe);
        let assign7520_e6131: f64 = (assign7520_e6127 + assign7520_e6130);
        let assign7520_e6134: f64 = (p.p490 * var_iae);
        let assign7520_e6135: f64 = (assign7520_e6131 + assign7520_e6134);
        (assign7520_e6135,)
    } else {
        (var_ct_p,)
    }
};
        var_ct_p = assign7520_e6137;
        var_ct_p_rv = 0.0;

        let assign7530_e6156: f64 = if (((param_given[495] || param_given[496]) || param_given[497]) || param_given[498]) { 1.0 } else { 0.0 };
        var_guard66 = assign7530_e6156;
        var_guard66_rv = 0.0;

        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfbedge_p_rv_slot = var_cfbedge_p_rv;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfdedge_p_rv_slot = var_cfdedge_p_rv;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfedge_p_rv_slot = var_cfedge_p_rv;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cfrd_p_rv_slot = var_cfrd_p_rv;
        *var_ct_p_slot = var_ct_p;
        *var_ct_p_rv_slot = var_ct_p_rv;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_ctedge_p_rv_slot = var_ctedge_p_rv;
        *var_dphib_p_slot = var_dphib_p;
        *var_dphib_p_rv_slot = var_dphib_p_rv;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_dphibedge_p_rv_slot = var_dphibedge_p_rv;
        *var_fnt_p_slot = var_fnt_p;
        *var_fnt_p_rv_slot = var_fnt_p_rv;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_gfacnud_p_rv_slot = var_gfacnud_p_rv;
        *var_gpe_edge_slot = var_gpe_edge;
        *var_gpe_edge_rv_slot = var_gpe_edge_rv;
        *var_guard56_slot = var_guard56;
        *var_guard56_rv_slot = var_guard56_rv;
        *var_guard57_slot = var_guard57;
        *var_guard57_rv_slot = var_guard57_rv;
        *var_guard58_slot = var_guard58;
        *var_guard58_rv_slot = var_guard58_rv;
        *var_guard59_slot = var_guard59;
        *var_guard59_rv_slot = var_guard59_rv;
        *var_guard60_slot = var_guard60;
        *var_guard60_rv_slot = var_guard60_rv;
        *var_guard61_slot = var_guard61;
        *var_guard61_rv_slot = var_guard61_rv;
        *var_guard62_slot = var_guard62;
        *var_guard62_rv_slot = var_guard62_rv;
        *var_guard63_slot = var_guard63;
        *var_guard63_rv_slot = var_guard63_rv;
        *var_guard64_slot = var_guard64;
        *var_guard64_rv_slot = var_guard64_rv;
        *var_guard65_slot = var_guard65;
        *var_guard65_rv_slot = var_guard65_rv;
        *var_guard66_slot = var_guard66;
        *var_guard66_rv_slot = var_guard66_rv;
        *var_kuowe_slot = var_kuowe;
        *var_kuowe_rv_slot = var_kuowe_rv;
        *var_kvthowe_slot = var_kvthowe;
        *var_kvthowe_rv_slot = var_kvthowe_rv;
        *var_munqs_p_slot = var_munqs_p;
        *var_munqs_p_rv_slot = var_munqs_p_rv;
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
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_pscebedge_p_rv_slot = var_pscebedge_p_rv;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_pscededge_p_rv_slot = var_pscededge_p_rv;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_psceedge_p_rv_slot = var_psceedge_p_rv;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stbetedge_p_rv_slot = var_stbetedge_p_rv;
        *var_stvfb_p_slot = var_stvfb_p;
        *var_stvfb_p_rv_slot = var_stvfb_p_rv;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_stvfbedge_p_rv_slot = var_stvfbedge_p_rv;
        *var_temp0_slot = var_temp0;
        *var_temp0_rv_slot = var_temp0_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
        *var_vsbnud_p_slot = var_vsbnud_p;
        *var_vsbnud_p_rv_slot = var_vsbnud_p_rv;
        *var_we_edge_slot = var_we_edge;
        *var_we_edge_rv_slot = var_we_edge_rv;
    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard41: f64,
        var_guard66: f64,
        var_iae: f64,
        var_ile: f64,
        var_ile2: f64,
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
        var_ctb_p_slot: &mut f64,
        var_ctb_p_rv_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_ctg_p_rv_slot: &mut f64,
        var_guard67_slot: &mut f64,
        var_guard67_rv_slot: &mut f64,
        var_guard68_slot: &mut f64,
        var_guard68_rv_slot: &mut f64,
        var_guard69_slot: &mut f64,
        var_guard69_rv_slot: &mut f64,
        var_guard70_slot: &mut f64,
        var_guard70_rv_slot: &mut f64,
        var_guard71_slot: &mut f64,
        var_guard71_rv_slot: &mut f64,
        var_guard72_slot: &mut f64,
        var_guard72_rv_slot: &mut f64,
        var_guard73_slot: &mut f64,
        var_guard73_rv_slot: &mut f64,
        var_guard74_slot: &mut f64,
        var_guard74_rv_slot: &mut f64,
        var_guard75_slot: &mut f64,
        var_guard75_rv_slot: &mut f64,
        var_guard76_slot: &mut f64,
        var_guard76_rv_slot: &mut f64,
        var_guard77_slot: &mut f64,
        var_guard77_rv_slot: &mut f64,
        var_guard78_slot: &mut f64,
        var_guard78_rv_slot: &mut f64,
        var_guard79_slot: &mut f64,
        var_guard79_rv_slot: &mut f64,
        var_guard80_slot: &mut f64,
        var_guard80_rv_slot: &mut f64,
        var_guard81_slot: &mut f64,
        var_guard81_rv_slot: &mut f64,
        var_guard82_slot: &mut f64,
        var_guard82_rv_slot: &mut f64,
        var_guard83_slot: &mut f64,
        var_guard83_rv_slot: &mut f64,
        var_guard84_slot: &mut f64,
        var_guard84_rv_slot: &mut f64,
        var_guard85_slot: &mut f64,
        var_guard85_rv_slot: &mut f64,
        var_guard86_slot: &mut f64,
        var_guard86_rv_slot: &mut f64,
        var_guard87_slot: &mut f64,
        var_guard87_rv_slot: &mut f64,
        var_guard88_slot: &mut f64,
        var_guard88_rv_slot: &mut f64,
        var_guard89_slot: &mut f64,
        var_guard89_rv_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard90_rv_slot: &mut f64,
        var_guard91_slot: &mut f64,
        var_guard91_rv_slot: &mut f64,
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
        var_stct_p_slot: &mut f64,
        var_stct_p_rv_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_strs_p_rv_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_stthesat_p_rv_slot: &mut f64,
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
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctb_p_rv: f64 = *var_ctb_p_rv_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_ctg_p_rv: f64 = *var_ctg_p_rv_slot;
        let mut var_guard67: f64 = *var_guard67_slot;
        let mut var_guard67_rv: f64 = *var_guard67_rv_slot;
        let mut var_guard68: f64 = *var_guard68_slot;
        let mut var_guard68_rv: f64 = *var_guard68_rv_slot;
        let mut var_guard69: f64 = *var_guard69_slot;
        let mut var_guard69_rv: f64 = *var_guard69_rv_slot;
        let mut var_guard70: f64 = *var_guard70_slot;
        let mut var_guard70_rv: f64 = *var_guard70_rv_slot;
        let mut var_guard71: f64 = *var_guard71_slot;
        let mut var_guard71_rv: f64 = *var_guard71_rv_slot;
        let mut var_guard72: f64 = *var_guard72_slot;
        let mut var_guard72_rv: f64 = *var_guard72_rv_slot;
        let mut var_guard73: f64 = *var_guard73_slot;
        let mut var_guard73_rv: f64 = *var_guard73_rv_slot;
        let mut var_guard74: f64 = *var_guard74_slot;
        let mut var_guard74_rv: f64 = *var_guard74_rv_slot;
        let mut var_guard75: f64 = *var_guard75_slot;
        let mut var_guard75_rv: f64 = *var_guard75_rv_slot;
        let mut var_guard76: f64 = *var_guard76_slot;
        let mut var_guard76_rv: f64 = *var_guard76_rv_slot;
        let mut var_guard77: f64 = *var_guard77_slot;
        let mut var_guard77_rv: f64 = *var_guard77_rv_slot;
        let mut var_guard78: f64 = *var_guard78_slot;
        let mut var_guard78_rv: f64 = *var_guard78_rv_slot;
        let mut var_guard79: f64 = *var_guard79_slot;
        let mut var_guard79_rv: f64 = *var_guard79_rv_slot;
        let mut var_guard80: f64 = *var_guard80_slot;
        let mut var_guard80_rv: f64 = *var_guard80_rv_slot;
        let mut var_guard81: f64 = *var_guard81_slot;
        let mut var_guard81_rv: f64 = *var_guard81_rv_slot;
        let mut var_guard82: f64 = *var_guard82_slot;
        let mut var_guard82_rv: f64 = *var_guard82_rv_slot;
        let mut var_guard83: f64 = *var_guard83_slot;
        let mut var_guard83_rv: f64 = *var_guard83_rv_slot;
        let mut var_guard84: f64 = *var_guard84_slot;
        let mut var_guard84_rv: f64 = *var_guard84_rv_slot;
        let mut var_guard85: f64 = *var_guard85_slot;
        let mut var_guard85_rv: f64 = *var_guard85_rv_slot;
        let mut var_guard86: f64 = *var_guard86_slot;
        let mut var_guard86_rv: f64 = *var_guard86_rv_slot;
        let mut var_guard87: f64 = *var_guard87_slot;
        let mut var_guard87_rv: f64 = *var_guard87_rv_slot;
        let mut var_guard88: f64 = *var_guard88_slot;
        let mut var_guard88_rv: f64 = *var_guard88_rv_slot;
        let mut var_guard89: f64 = *var_guard89_slot;
        let mut var_guard89_rv: f64 = *var_guard89_rv_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard90_rv: f64 = *var_guard90_rv_slot;
        let mut var_guard91: f64 = *var_guard91_slot;
        let mut var_guard91_rv: f64 = *var_guard91_rv_slot;
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
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stct_p_rv: f64 = *var_stct_p_rv_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_strs_p_rv: f64 = *var_strs_p_rv_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_stthesat_p_rv: f64 = *var_stthesat_p_rv_slot;
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
        let mut var_xcor_p: f64 = *var_xcor_p_slot;
        let mut var_xcor_p_rv: f64 = *var_xcor_p_rv_slot;

        let (assign7540_e6174,) = {
    if ((var_guard41 != 0.0) && (var_guard66 != 0.0)) {
        let assign7540_e6163: f64 = (p.p496 * var_ile);
        let assign7540_e6164: f64 = (p.p495 + assign7540_e6163);
        let assign7540_e6167: f64 = (p.p497 * var_iwe);
        let assign7540_e6168: f64 = (assign7540_e6164 + assign7540_e6167);
        let assign7540_e6171: f64 = (p.p498 * var_iae);
        let assign7540_e6172: f64 = (assign7540_e6168 + assign7540_e6171);
        (assign7540_e6172,)
    } else {
        (var_ctg_p,)
    }
};
        var_ctg_p = assign7540_e6174;
        var_ctg_p_rv = 0.0;

        let assign7550_e6193: f64 = if (((param_given[491] || param_given[492]) || param_given[493]) || param_given[494]) { 1.0 } else { 0.0 };
        var_guard67 = assign7550_e6193;
        var_guard67_rv = 0.0;

        let (assign7560_e6211,) = {
    if ((var_guard41 != 0.0) && (var_guard67 != 0.0)) {
        let assign7560_e6200: f64 = (p.p492 * var_ile);
        let assign7560_e6201: f64 = (p.p491 + assign7560_e6200);
        let assign7560_e6204: f64 = (p.p493 * var_iwe);
        let assign7560_e6205: f64 = (assign7560_e6201 + assign7560_e6204);
        let assign7560_e6208: f64 = (p.p494 * var_iae);
        let assign7560_e6209: f64 = (assign7560_e6205 + assign7560_e6208);
        (assign7560_e6209,)
    } else {
        (var_ctb_p,)
    }
};
        var_ctb_p = assign7560_e6211;
        var_ctb_p_rv = 0.0;

        let assign7570_e6230: f64 = if (((param_given[499] || param_given[500]) || param_given[501]) || param_given[502]) { 1.0 } else { 0.0 };
        var_guard68 = assign7570_e6230;
        var_guard68_rv = 0.0;

        let (assign7580_e6248,) = {
    if ((var_guard41 != 0.0) && (var_guard68 != 0.0)) {
        let assign7580_e6237: f64 = (p.p500 * var_ile);
        let assign7580_e6238: f64 = (p.p499 + assign7580_e6237);
        let assign7580_e6241: f64 = (p.p501 * var_iwe);
        let assign7580_e6242: f64 = (assign7580_e6238 + assign7580_e6241);
        let assign7580_e6245: f64 = (p.p502 * var_iae);
        let assign7580_e6246: f64 = (assign7580_e6242 + assign7580_e6245);
        (assign7580_e6246,)
    } else {
        (var_stct_p,)
    }
};
        var_stct_p = assign7580_e6248;
        var_stct_p_rv = 0.0;

        let assign7590_e6267: f64 = if (((param_given[503] || param_given[504]) || param_given[505]) || param_given[506]) { 1.0 } else { 0.0 };
        var_guard69 = assign7590_e6267;
        var_guard69_rv = 0.0;

        let (assign7600_e6287,) = {
    if ((var_guard41 != 0.0) && (var_guard69 != 0.0)) {
        let assign7600_e6275: f64 = (p.p504 * var_ile);
        let assign7600_e6276: f64 = (p.p503 + assign7600_e6275);
        let assign7600_e6279: f64 = (p.p505 * var_iwe);
        let assign7600_e6280: f64 = (assign7600_e6276 + assign7600_e6279);
        let assign7600_e6283: f64 = (p.p506 * var_iae);
        let assign7600_e6284: f64 = (assign7600_e6280 + assign7600_e6283);
        let assign7600_e6285: f64 = (var_ile2 * assign7600_e6284);
        (assign7600_e6285,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign7600_e6287;
        var_cf_p_rv = 0.0;

        let assign7610_e6306: f64 = if (((param_given[511] || param_given[512]) || param_given[513]) || param_given[514]) { 1.0 } else { 0.0 };
        var_guard70 = assign7610_e6306;
        var_guard70_rv = 0.0;

        let (assign7620_e6324,) = {
    if ((var_guard41 != 0.0) && (var_guard70 != 0.0)) {
        let assign7620_e6313: f64 = (p.p512 * var_ile);
        let assign7620_e6314: f64 = (p.p511 + assign7620_e6313);
        let assign7620_e6317: f64 = (p.p513 * var_iwe);
        let assign7620_e6318: f64 = (assign7620_e6314 + assign7620_e6317);
        let assign7620_e6321: f64 = (p.p514 * var_iae);
        let assign7620_e6322: f64 = (assign7620_e6318 + assign7620_e6321);
        (assign7620_e6322,)
    } else {
        (var_cfd_p,)
    }
};
        var_cfd_p = assign7620_e6324;
        var_cfd_p_rv = 0.0;

        let assign7630_e6343: f64 = if (((param_given[507] || param_given[508]) || param_given[509]) || param_given[510]) { 1.0 } else { 0.0 };
        var_guard71 = assign7630_e6343;
        var_guard71_rv = 0.0;

        let (assign7640_e6361,) = {
    if ((var_guard41 != 0.0) && (var_guard71 != 0.0)) {
        let assign7640_e6350: f64 = (p.p508 * var_ile);
        let assign7640_e6351: f64 = (p.p507 + assign7640_e6350);
        let assign7640_e6354: f64 = (p.p509 * var_iwe);
        let assign7640_e6355: f64 = (assign7640_e6351 + assign7640_e6354);
        let assign7640_e6358: f64 = (p.p510 * var_iae);
        let assign7640_e6359: f64 = (assign7640_e6355 + assign7640_e6358);
        (assign7640_e6359,)
    } else {
        (var_cfb_p,)
    }
};
        var_cfb_p = assign7640_e6361;
        var_cfb_p_rv = 0.0;

        let assign7650_e6380: f64 = if (((param_given[515] || param_given[516]) || param_given[517]) || param_given[518]) { 1.0 } else { 0.0 };
        var_guard72 = assign7650_e6380;
        var_guard72_rv = 0.0;

        let (assign7660_e6400,) = {
    if ((var_guard41 != 0.0) && (var_guard72 != 0.0)) {
        let assign7660_e6388: f64 = (p.p516 * var_ile);
        let assign7660_e6389: f64 = (p.p515 + assign7660_e6388);
        let assign7660_e6392: f64 = (p.p517 * var_iwe);
        let assign7660_e6393: f64 = (assign7660_e6389 + assign7660_e6392);
        let assign7660_e6396: f64 = (p.p518 * var_iae);
        let assign7660_e6397: f64 = (assign7660_e6393 + assign7660_e6396);
        let assign7660_e6398: f64 = (var_ile2 * assign7660_e6397);
        (assign7660_e6398,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign7660_e6400;
        var_psce_p_rv = 0.0;

        let assign7670_e6419: f64 = if (((param_given[523] || param_given[524]) || param_given[525]) || param_given[526]) { 1.0 } else { 0.0 };
        var_guard73 = assign7670_e6419;
        var_guard73_rv = 0.0;

        let (assign7680_e6437,) = {
    if ((var_guard41 != 0.0) && (var_guard73 != 0.0)) {
        let assign7680_e6426: f64 = (p.p524 * var_ile);
        let assign7680_e6427: f64 = (p.p523 + assign7680_e6426);
        let assign7680_e6430: f64 = (p.p525 * var_iwe);
        let assign7680_e6431: f64 = (assign7680_e6427 + assign7680_e6430);
        let assign7680_e6434: f64 = (p.p526 * var_iae);
        let assign7680_e6435: f64 = (assign7680_e6431 + assign7680_e6434);
        (assign7680_e6435,)
    } else {
        (var_psced_p,)
    }
};
        var_psced_p = assign7680_e6437;
        var_psced_p_rv = 0.0;

        let assign7690_e6456: f64 = if (((param_given[519] || param_given[520]) || param_given[521]) || param_given[522]) { 1.0 } else { 0.0 };
        var_guard74 = assign7690_e6456;
        var_guard74_rv = 0.0;

        let (assign7700_e6474,) = {
    if ((var_guard41 != 0.0) && (var_guard74 != 0.0)) {
        let assign7700_e6463: f64 = (p.p520 * var_ile);
        let assign7700_e6464: f64 = (p.p519 + assign7700_e6463);
        let assign7700_e6467: f64 = (p.p521 * var_iwe);
        let assign7700_e6468: f64 = (assign7700_e6464 + assign7700_e6467);
        let assign7700_e6471: f64 = (p.p522 * var_iae);
        let assign7700_e6472: f64 = (assign7700_e6468 + assign7700_e6471);
        (assign7700_e6472,)
    } else {
        (var_psceb_p,)
    }
};
        var_psceb_p = assign7700_e6474;
        var_psceb_p_rv = 0.0;

        let assign7710_e6493: f64 = if (((param_given[527] || param_given[528]) || param_given[529]) || param_given[530]) { 1.0 } else { 0.0 };
        var_guard75 = assign7710_e6493;
        var_guard75_rv = 0.0;

        let (assign7720_e6515,) = {
    if ((var_guard41 != 0.0) && (var_guard75 != 0.0)) {
        let assign7720_e6499: f64 = (var_we / var_le);
        let assign7720_e6503: f64 = (p.p528 * var_ile);
        let assign7720_e6504: f64 = (p.p527 + assign7720_e6503);
        let assign7720_e6507: f64 = (p.p529 * var_iwe);
        let assign7720_e6508: f64 = (assign7720_e6504 + assign7720_e6507);
        let assign7720_e6511: f64 = (p.p530 * var_iae);
        let assign7720_e6512: f64 = (assign7720_e6508 + assign7720_e6511);
        let assign7720_e6513: f64 = (assign7720_e6499 * assign7720_e6512);
        (assign7720_e6513,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign7720_e6515;
        var_betn_p_rv = 0.0;

        let assign7730_e6534: f64 = if (((param_given[531] || param_given[532]) || param_given[533]) || param_given[534]) { 1.0 } else { 0.0 };
        var_guard76 = assign7730_e6534;
        var_guard76_rv = 0.0;

        let (assign7740_e6552,) = {
    if ((var_guard41 != 0.0) && (var_guard76 != 0.0)) {
        let assign7740_e6541: f64 = (p.p532 * var_ile);
        let assign7740_e6542: f64 = (p.p531 + assign7740_e6541);
        let assign7740_e6545: f64 = (p.p533 * var_iwe);
        let assign7740_e6546: f64 = (assign7740_e6542 + assign7740_e6545);
        let assign7740_e6549: f64 = (p.p534 * var_iae);
        let assign7740_e6550: f64 = (assign7740_e6546 + assign7740_e6549);
        (assign7740_e6550,)
    } else {
        (var_stbet_p,)
    }
};
        var_stbet_p = assign7740_e6552;
        var_stbet_p_rv = 0.0;

        let assign7750_e6571: f64 = if (((param_given[535] || param_given[536]) || param_given[537]) || param_given[538]) { 1.0 } else { 0.0 };
        var_guard77 = assign7750_e6571;
        var_guard77_rv = 0.0;

        let (assign7760_e6589,) = {
    if ((var_guard41 != 0.0) && (var_guard77 != 0.0)) {
        let assign7760_e6578: f64 = (p.p536 * var_ile);
        let assign7760_e6579: f64 = (p.p535 + assign7760_e6578);
        let assign7760_e6582: f64 = (p.p537 * var_iwe);
        let assign7760_e6583: f64 = (assign7760_e6579 + assign7760_e6582);
        let assign7760_e6586: f64 = (p.p538 * var_iae);
        let assign7760_e6587: f64 = (assign7760_e6583 + assign7760_e6586);
        (assign7760_e6587,)
    } else {
        (var_mue_p,)
    }
};
        var_mue_p = assign7760_e6589;
        var_mue_p_rv = 0.0;

        let assign7770_e6608: f64 = if (((param_given[539] || param_given[540]) || param_given[541]) || param_given[542]) { 1.0 } else { 0.0 };
        var_guard78 = assign7770_e6608;
        var_guard78_rv = 0.0;

        let (assign7780_e6626,) = {
    if ((var_guard41 != 0.0) && (var_guard78 != 0.0)) {
        let assign7780_e6615: f64 = (p.p540 * var_ile);
        let assign7780_e6616: f64 = (p.p539 + assign7780_e6615);
        let assign7780_e6619: f64 = (p.p541 * var_iwe);
        let assign7780_e6620: f64 = (assign7780_e6616 + assign7780_e6619);
        let assign7780_e6623: f64 = (p.p542 * var_iae);
        let assign7780_e6624: f64 = (assign7780_e6620 + assign7780_e6623);
        (assign7780_e6624,)
    } else {
        (var_themu_p,)
    }
};
        var_themu_p = assign7780_e6626;
        var_themu_p_rv = 0.0;

        let assign7790_e6645: f64 = if (((param_given[543] || param_given[544]) || param_given[545]) || param_given[546]) { 1.0 } else { 0.0 };
        var_guard79 = assign7790_e6645;
        var_guard79_rv = 0.0;

        let (assign7800_e6663,) = {
    if ((var_guard41 != 0.0) && (var_guard79 != 0.0)) {
        let assign7800_e6652: f64 = (p.p544 * var_ile);
        let assign7800_e6653: f64 = (p.p543 + assign7800_e6652);
        let assign7800_e6656: f64 = (p.p545 * var_iwe);
        let assign7800_e6657: f64 = (assign7800_e6653 + assign7800_e6656);
        let assign7800_e6660: f64 = (p.p546 * var_iae);
        let assign7800_e6661: f64 = (assign7800_e6657 + assign7800_e6660);
        (assign7800_e6661,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign7800_e6663;
        var_cs_p_rv = 0.0;

        let assign7810_e6682: f64 = if (((param_given[547] || param_given[548]) || param_given[549]) || param_given[550]) { 1.0 } else { 0.0 };
        var_guard80 = assign7810_e6682;
        var_guard80_rv = 0.0;

        let (assign7820_e6700,) = {
    if ((var_guard41 != 0.0) && (var_guard80 != 0.0)) {
        let assign7820_e6689: f64 = (p.p548 * var_ile);
        let assign7820_e6690: f64 = (p.p547 + assign7820_e6689);
        let assign7820_e6693: f64 = (p.p549 * var_iwe);
        let assign7820_e6694: f64 = (assign7820_e6690 + assign7820_e6693);
        let assign7820_e6697: f64 = (p.p550 * var_iae);
        let assign7820_e6698: f64 = (assign7820_e6694 + assign7820_e6697);
        (assign7820_e6698,)
    } else {
        (var_thecs_p,)
    }
};
        var_thecs_p = assign7820_e6700;
        var_thecs_p_rv = 0.0;

        let assign7830_e6719: f64 = if (((param_given[551] || param_given[552]) || param_given[553]) || param_given[554]) { 1.0 } else { 0.0 };
        var_guard81 = assign7830_e6719;
        var_guard81_rv = 0.0;

        let (assign7840_e6737,) = {
    if ((var_guard41 != 0.0) && (var_guard81 != 0.0)) {
        let assign7840_e6726: f64 = (p.p552 * var_ile);
        let assign7840_e6727: f64 = (p.p551 + assign7840_e6726);
        let assign7840_e6730: f64 = (p.p553 * var_iwe);
        let assign7840_e6731: f64 = (assign7840_e6727 + assign7840_e6730);
        let assign7840_e6734: f64 = (p.p554 * var_iae);
        let assign7840_e6735: f64 = (assign7840_e6731 + assign7840_e6734);
        (assign7840_e6735,)
    } else {
        (var_xcor_p,)
    }
};
        var_xcor_p = assign7840_e6737;
        var_xcor_p_rv = 0.0;

        let assign7850_e6756: f64 = if (((param_given[555] || param_given[556]) || param_given[557]) || param_given[558]) { 1.0 } else { 0.0 };
        var_guard82 = assign7850_e6756;
        var_guard82_rv = 0.0;

        let (assign7860_e6776,) = {
    if ((var_guard41 != 0.0) && (var_guard82 != 0.0)) {
        let assign7860_e6764: f64 = (p.p556 * var_ile);
        let assign7860_e6765: f64 = (p.p555 + assign7860_e6764);
        let assign7860_e6768: f64 = (p.p557 * var_iwe);
        let assign7860_e6769: f64 = (assign7860_e6765 + assign7860_e6768);
        let assign7860_e6772: f64 = (p.p558 * var_iae);
        let assign7860_e6773: f64 = (assign7860_e6769 + assign7860_e6772);
        let assign7860_e6774: f64 = (var_iwe * assign7860_e6773);
        (assign7860_e6774,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign7860_e6776;
        var_rs_p_rv = 0.0;

        let assign7870_e6795: f64 = if (((param_given[559] || param_given[560]) || param_given[561]) || param_given[562]) { 1.0 } else { 0.0 };
        var_guard83 = assign7870_e6795;
        var_guard83_rv = 0.0;

        let (assign7880_e6813,) = {
    if ((var_guard41 != 0.0) && (var_guard83 != 0.0)) {
        let assign7880_e6802: f64 = (p.p560 * var_ile);
        let assign7880_e6803: f64 = (p.p559 + assign7880_e6802);
        let assign7880_e6806: f64 = (p.p561 * var_iwe);
        let assign7880_e6807: f64 = (assign7880_e6803 + assign7880_e6806);
        let assign7880_e6810: f64 = (p.p562 * var_iae);
        let assign7880_e6811: f64 = (assign7880_e6807 + assign7880_e6810);
        (assign7880_e6811,)
    } else {
        (var_strs_p,)
    }
};
        var_strs_p = assign7880_e6813;
        var_strs_p_rv = 0.0;

        let assign7890_e6832: f64 = if (((param_given[563] || param_given[564]) || param_given[565]) || param_given[566]) { 1.0 } else { 0.0 };
        var_guard84 = assign7890_e6832;
        var_guard84_rv = 0.0;

        let (assign7900_e6850,) = {
    if ((var_guard41 != 0.0) && (var_guard84 != 0.0)) {
        let assign7900_e6839: f64 = (p.p564 * var_ile);
        let assign7900_e6840: f64 = (p.p563 + assign7900_e6839);
        let assign7900_e6843: f64 = (p.p565 * var_iwe);
        let assign7900_e6844: f64 = (assign7900_e6840 + assign7900_e6843);
        let assign7900_e6847: f64 = (p.p566 * var_iae);
        let assign7900_e6848: f64 = (assign7900_e6844 + assign7900_e6847);
        (assign7900_e6848,)
    } else {
        (var_rsb_p,)
    }
};
        var_rsb_p = assign7900_e6850;
        var_rsb_p_rv = 0.0;

        let assign7910_e6869: f64 = if (((param_given[567] || param_given[568]) || param_given[569]) || param_given[570]) { 1.0 } else { 0.0 };
        var_guard85 = assign7910_e6869;
        var_guard85_rv = 0.0;

        let (assign7920_e6887,) = {
    if ((var_guard41 != 0.0) && (var_guard85 != 0.0)) {
        let assign7920_e6876: f64 = (p.p568 * var_ile);
        let assign7920_e6877: f64 = (p.p567 + assign7920_e6876);
        let assign7920_e6880: f64 = (p.p569 * var_iwe);
        let assign7920_e6881: f64 = (assign7920_e6877 + assign7920_e6880);
        let assign7920_e6884: f64 = (p.p570 * var_iae);
        let assign7920_e6885: f64 = (assign7920_e6881 + assign7920_e6884);
        (assign7920_e6885,)
    } else {
        (var_rsg_p,)
    }
};
        var_rsg_p = assign7920_e6887;
        var_rsg_p_rv = 0.0;

        let assign7930_e6906: f64 = if (((param_given[571] || param_given[572]) || param_given[573]) || param_given[574]) { 1.0 } else { 0.0 };
        var_guard86 = assign7930_e6906;
        var_guard86_rv = 0.0;

        let (assign7940_e6926,) = {
    if ((var_guard41 != 0.0) && (var_guard86 != 0.0)) {
        let assign7940_e6914: f64 = (p.p572 * var_ile);
        let assign7940_e6915: f64 = (p.p571 + assign7940_e6914);
        let assign7940_e6918: f64 = (p.p573 * var_iwe);
        let assign7940_e6919: f64 = (assign7940_e6915 + assign7940_e6918);
        let assign7940_e6922: f64 = (p.p574 * var_iae);
        let assign7940_e6923: f64 = (assign7940_e6919 + assign7940_e6922);
        let assign7940_e6924: f64 = (var_ile * assign7940_e6923);
        (assign7940_e6924,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign7940_e6926;
        var_thesat_p_rv = 0.0;

        let assign7950_e6945: f64 = if (((param_given[575] || param_given[576]) || param_given[577]) || param_given[578]) { 1.0 } else { 0.0 };
        var_guard87 = assign7950_e6945;
        var_guard87_rv = 0.0;

        let (assign7960_e6963,) = {
    if ((var_guard41 != 0.0) && (var_guard87 != 0.0)) {
        let assign7960_e6952: f64 = (p.p576 * var_ile);
        let assign7960_e6953: f64 = (p.p575 + assign7960_e6952);
        let assign7960_e6956: f64 = (p.p577 * var_iwe);
        let assign7960_e6957: f64 = (assign7960_e6953 + assign7960_e6956);
        let assign7960_e6960: f64 = (p.p578 * var_iae);
        let assign7960_e6961: f64 = (assign7960_e6957 + assign7960_e6960);
        (assign7960_e6961,)
    } else {
        (var_stthesat_p,)
    }
};
        var_stthesat_p = assign7960_e6963;
        var_stthesat_p_rv = 0.0;

        let assign7970_e6982: f64 = if (((param_given[579] || param_given[580]) || param_given[581]) || param_given[582]) { 1.0 } else { 0.0 };
        var_guard88 = assign7970_e6982;
        var_guard88_rv = 0.0;

        let (assign7980_e7000,) = {
    if ((var_guard41 != 0.0) && (var_guard88 != 0.0)) {
        let assign7980_e6989: f64 = (p.p580 * var_ile);
        let assign7980_e6990: f64 = (p.p579 + assign7980_e6989);
        let assign7980_e6993: f64 = (p.p581 * var_iwe);
        let assign7980_e6994: f64 = (assign7980_e6990 + assign7980_e6993);
        let assign7980_e6997: f64 = (p.p582 * var_iae);
        let assign7980_e6998: f64 = (assign7980_e6994 + assign7980_e6997);
        (assign7980_e6998,)
    } else {
        (var_thesatb_p,)
    }
};
        var_thesatb_p = assign7980_e7000;
        var_thesatb_p_rv = 0.0;

        let assign7990_e7019: f64 = if (((param_given[583] || param_given[584]) || param_given[585]) || param_given[586]) { 1.0 } else { 0.0 };
        var_guard89 = assign7990_e7019;
        var_guard89_rv = 0.0;

        let (assign8000_e7037,) = {
    if ((var_guard41 != 0.0) && (var_guard89 != 0.0)) {
        let assign8000_e7026: f64 = (p.p584 * var_ile);
        let assign8000_e7027: f64 = (p.p583 + assign8000_e7026);
        let assign8000_e7030: f64 = (p.p585 * var_iwe);
        let assign8000_e7031: f64 = (assign8000_e7027 + assign8000_e7030);
        let assign8000_e7034: f64 = (p.p586 * var_iae);
        let assign8000_e7035: f64 = (assign8000_e7031 + assign8000_e7034);
        (assign8000_e7035,)
    } else {
        (var_thesatg_p,)
    }
};
        var_thesatg_p = assign8000_e7037;
        var_thesatg_p_rv = 0.0;

        let assign8010_e7056: f64 = if (((param_given[587] || param_given[588]) || param_given[589]) || param_given[590]) { 1.0 } else { 0.0 };
        var_guard90 = assign8010_e7056;
        var_guard90_rv = 0.0;

        let (assign8020_e7074,) = {
    if ((var_guard41 != 0.0) && (var_guard90 != 0.0)) {
        let assign8020_e7063: f64 = (p.p588 * var_ile);
        let assign8020_e7064: f64 = (p.p587 + assign8020_e7063);
        let assign8020_e7067: f64 = (p.p589 * var_iwe);
        let assign8020_e7068: f64 = (assign8020_e7064 + assign8020_e7067);
        let assign8020_e7071: f64 = (p.p590 * var_iae);
        let assign8020_e7072: f64 = (assign8020_e7068 + assign8020_e7071);
        (assign8020_e7072,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign8020_e7074;
        var_ax_p_rv = 0.0;

        let assign8030_e7093: f64 = if (((param_given[591] || param_given[592]) || param_given[593]) || param_given[594]) { 1.0 } else { 0.0 };
        var_guard91 = assign8030_e7093;
        var_guard91_rv = 0.0;

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
        *var_ctb_p_slot = var_ctb_p;
        *var_ctb_p_rv_slot = var_ctb_p_rv;
        *var_ctg_p_slot = var_ctg_p;
        *var_ctg_p_rv_slot = var_ctg_p_rv;
        *var_guard67_slot = var_guard67;
        *var_guard67_rv_slot = var_guard67_rv;
        *var_guard68_slot = var_guard68;
        *var_guard68_rv_slot = var_guard68_rv;
        *var_guard69_slot = var_guard69;
        *var_guard69_rv_slot = var_guard69_rv;
        *var_guard70_slot = var_guard70;
        *var_guard70_rv_slot = var_guard70_rv;
        *var_guard71_slot = var_guard71;
        *var_guard71_rv_slot = var_guard71_rv;
        *var_guard72_slot = var_guard72;
        *var_guard72_rv_slot = var_guard72_rv;
        *var_guard73_slot = var_guard73;
        *var_guard73_rv_slot = var_guard73_rv;
        *var_guard74_slot = var_guard74;
        *var_guard74_rv_slot = var_guard74_rv;
        *var_guard75_slot = var_guard75;
        *var_guard75_rv_slot = var_guard75_rv;
        *var_guard76_slot = var_guard76;
        *var_guard76_rv_slot = var_guard76_rv;
        *var_guard77_slot = var_guard77;
        *var_guard77_rv_slot = var_guard77_rv;
        *var_guard78_slot = var_guard78;
        *var_guard78_rv_slot = var_guard78_rv;
        *var_guard79_slot = var_guard79;
        *var_guard79_rv_slot = var_guard79_rv;
        *var_guard80_slot = var_guard80;
        *var_guard80_rv_slot = var_guard80_rv;
        *var_guard81_slot = var_guard81;
        *var_guard81_rv_slot = var_guard81_rv;
        *var_guard82_slot = var_guard82;
        *var_guard82_rv_slot = var_guard82_rv;
        *var_guard83_slot = var_guard83;
        *var_guard83_rv_slot = var_guard83_rv;
        *var_guard84_slot = var_guard84;
        *var_guard84_rv_slot = var_guard84_rv;
        *var_guard85_slot = var_guard85;
        *var_guard85_rv_slot = var_guard85_rv;
        *var_guard86_slot = var_guard86;
        *var_guard86_rv_slot = var_guard86_rv;
        *var_guard87_slot = var_guard87;
        *var_guard87_rv_slot = var_guard87_rv;
        *var_guard88_slot = var_guard88;
        *var_guard88_rv_slot = var_guard88_rv;
        *var_guard89_slot = var_guard89;
        *var_guard89_rv_slot = var_guard89_rv;
        *var_guard90_slot = var_guard90;
        *var_guard90_rv_slot = var_guard90_rv;
        *var_guard91_slot = var_guard91;
        *var_guard91_rv_slot = var_guard91_rv;
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
        *var_stct_p_slot = var_stct_p;
        *var_stct_p_rv_slot = var_stct_p_rv;
        *var_strs_p_slot = var_strs_p;
        *var_strs_p_rv_slot = var_strs_p_rv;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_stthesat_p_rv_slot = var_stthesat_p_rv;
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
        *var_xcor_p_slot = var_xcor_p;
        *var_xcor_p_rv_slot = var_xcor_p_rv;
    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard41: f64,
        var_guard91: f64,
        var_iae: f64,
        var_iiae: f64,
        var_iiwe: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lecv: f64,
        var_a1_p_slot: &mut f64,
        var_a1_p_rv_slot: &mut f64,
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
        var_cox_p_slot: &mut f64,
        var_cox_p_rv_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_delvtac_p_rv_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_facneffac_p_rv_slot: &mut f64,
        var_guard100_slot: &mut f64,
        var_guard100_rv_slot: &mut f64,
        var_guard101_slot: &mut f64,
        var_guard101_rv_slot: &mut f64,
        var_guard102_slot: &mut f64,
        var_guard102_rv_slot: &mut f64,
        var_guard103_slot: &mut f64,
        var_guard103_rv_slot: &mut f64,
        var_guard104_slot: &mut f64,
        var_guard104_rv_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_guard105_rv_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard106_rv_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard107_rv_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard108_rv_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard109_rv_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard110_rv_slot: &mut f64,
        var_guard111_slot: &mut f64,
        var_guard111_rv_slot: &mut f64,
        var_guard112_slot: &mut f64,
        var_guard112_rv_slot: &mut f64,
        var_guard113_slot: &mut f64,
        var_guard113_rv_slot: &mut f64,
        var_guard114_slot: &mut f64,
        var_guard114_rv_slot: &mut f64,
        var_guard115_slot: &mut f64,
        var_guard115_rv_slot: &mut f64,
        var_guard92_slot: &mut f64,
        var_guard92_rv_slot: &mut f64,
        var_guard93_slot: &mut f64,
        var_guard93_rv_slot: &mut f64,
        var_guard94_slot: &mut f64,
        var_guard94_rv_slot: &mut f64,
        var_guard95_slot: &mut f64,
        var_guard95_rv_slot: &mut f64,
        var_guard96_slot: &mut f64,
        var_guard96_rv_slot: &mut f64,
        var_guard97_slot: &mut f64,
        var_guard97_rv_slot: &mut f64,
        var_guard98_slot: &mut f64,
        var_guard98_rv_slot: &mut f64,
        var_guard99_slot: &mut f64,
        var_guard99_rv_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_iginv_p_rv_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igov_p_rv_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_igovd_p_rv_slot: &mut f64,
        var_plparam_i_slot: &mut f64,
        var_plparam_i_rv_slot: &mut f64,
        var_plwparam_i_slot: &mut f64,
        var_plwparam_i_rv_slot: &mut f64,
        var_poparam_i_slot: &mut f64,
        var_poparam_i_rv_slot: &mut f64,
        var_pwparam_i_slot: &mut f64,
        var_pwparam_i_rv_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_sta2_p_rv_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidl_p_rv_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stbgidld_p_rv_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_stig_p_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
    ) {
        let mut var_a1_p: f64 = *var_a1_p_slot;
        let mut var_a1_p_rv: f64 = *var_a1_p_rv_slot;
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
        let mut var_cox_p: f64 = *var_cox_p_slot;
        let mut var_cox_p_rv: f64 = *var_cox_p_rv_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_delvtac_p_rv: f64 = *var_delvtac_p_rv_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_facneffac_p_rv: f64 = *var_facneffac_p_rv_slot;
        let mut var_guard100: f64 = *var_guard100_slot;
        let mut var_guard100_rv: f64 = *var_guard100_rv_slot;
        let mut var_guard101: f64 = *var_guard101_slot;
        let mut var_guard101_rv: f64 = *var_guard101_rv_slot;
        let mut var_guard102: f64 = *var_guard102_slot;
        let mut var_guard102_rv: f64 = *var_guard102_rv_slot;
        let mut var_guard103: f64 = *var_guard103_slot;
        let mut var_guard103_rv: f64 = *var_guard103_rv_slot;
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard104_rv: f64 = *var_guard104_rv_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_guard105_rv: f64 = *var_guard105_rv_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard106_rv: f64 = *var_guard106_rv_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard107_rv: f64 = *var_guard107_rv_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard108_rv: f64 = *var_guard108_rv_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard109_rv: f64 = *var_guard109_rv_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard110_rv: f64 = *var_guard110_rv_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
        let mut var_guard111_rv: f64 = *var_guard111_rv_slot;
        let mut var_guard112: f64 = *var_guard112_slot;
        let mut var_guard112_rv: f64 = *var_guard112_rv_slot;
        let mut var_guard113: f64 = *var_guard113_slot;
        let mut var_guard113_rv: f64 = *var_guard113_rv_slot;
        let mut var_guard114: f64 = *var_guard114_slot;
        let mut var_guard114_rv: f64 = *var_guard114_rv_slot;
        let mut var_guard115: f64 = *var_guard115_slot;
        let mut var_guard115_rv: f64 = *var_guard115_rv_slot;
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_guard92_rv: f64 = *var_guard92_rv_slot;
        let mut var_guard93: f64 = *var_guard93_slot;
        let mut var_guard93_rv: f64 = *var_guard93_rv_slot;
        let mut var_guard94: f64 = *var_guard94_slot;
        let mut var_guard94_rv: f64 = *var_guard94_rv_slot;
        let mut var_guard95: f64 = *var_guard95_slot;
        let mut var_guard95_rv: f64 = *var_guard95_rv_slot;
        let mut var_guard96: f64 = *var_guard96_slot;
        let mut var_guard96_rv: f64 = *var_guard96_rv_slot;
        let mut var_guard97: f64 = *var_guard97_slot;
        let mut var_guard97_rv: f64 = *var_guard97_rv_slot;
        let mut var_guard98: f64 = *var_guard98_slot;
        let mut var_guard98_rv: f64 = *var_guard98_rv_slot;
        let mut var_guard99: f64 = *var_guard99_slot;
        let mut var_guard99_rv: f64 = *var_guard99_rv_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_iginv_p_rv: f64 = *var_iginv_p_rv_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igov_p_rv: f64 = *var_igov_p_rv_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_igovd_p_rv: f64 = *var_igovd_p_rv_slot;
        let mut var_plparam_i: f64 = *var_plparam_i_slot;
        let mut var_plparam_i_rv: f64 = *var_plparam_i_rv_slot;
        let mut var_plwparam_i: f64 = *var_plwparam_i_slot;
        let mut var_plwparam_i_rv: f64 = *var_plwparam_i_rv_slot;
        let mut var_poparam_i: f64 = *var_poparam_i_slot;
        let mut var_poparam_i_rv: f64 = *var_poparam_i_rv_slot;
        let mut var_pwparam_i: f64 = *var_pwparam_i_slot;
        let mut var_pwparam_i_rv: f64 = *var_pwparam_i_rv_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_sta2_p_rv: f64 = *var_sta2_p_rv_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidl_p_rv: f64 = *var_stbgidl_p_rv_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stbgidld_p_rv: f64 = *var_stbgidld_p_rv_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_stig_p_rv: f64 = *var_stig_p_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;

        let (assign8040_e7113,) = {
    if ((var_guard41 != 0.0) && (var_guard91 != 0.0)) {
        let assign8040_e7101: f64 = (p.p592 * var_ile);
        let assign8040_e7102: f64 = (p.p591 + assign8040_e7101);
        let assign8040_e7105: f64 = (p.p593 * var_iwe);
        let assign8040_e7106: f64 = (assign8040_e7102 + assign8040_e7105);
        let assign8040_e7109: f64 = (p.p594 * var_iae);
        let assign8040_e7110: f64 = (assign8040_e7106 + assign8040_e7109);
        let assign8040_e7111: f64 = (var_ile * assign8040_e7110);
        (assign8040_e7111,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign8040_e7113;
        var_alp_p_rv = 0.0;

        let assign8050_e7132: f64 = if (((param_given[595] || param_given[596]) || param_given[597]) || param_given[598]) { 1.0 } else { 0.0 };
        var_guard92 = assign8050_e7132;
        var_guard92_rv = 0.0;

        let (assign8060_e7150,) = {
    if ((var_guard41 != 0.0) && (var_guard92 != 0.0)) {
        let assign8060_e7139: f64 = (p.p596 * var_ile);
        let assign8060_e7140: f64 = (p.p595 + assign8060_e7139);
        let assign8060_e7143: f64 = (p.p597 * var_iwe);
        let assign8060_e7144: f64 = (assign8060_e7140 + assign8060_e7143);
        let assign8060_e7147: f64 = (p.p598 * var_iae);
        let assign8060_e7148: f64 = (assign8060_e7144 + assign8060_e7147);
        (assign8060_e7148,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign8060_e7150;
        var_alp1_p_rv = 0.0;

        let assign8070_e7169: f64 = if (((param_given[599] || param_given[600]) || param_given[601]) || param_given[602]) { 1.0 } else { 0.0 };
        var_guard93 = assign8070_e7169;
        var_guard93_rv = 0.0;

        let (assign8080_e7187,) = {
    if ((var_guard41 != 0.0) && (var_guard93 != 0.0)) {
        let assign8080_e7176: f64 = (p.p600 * var_ile);
        let assign8080_e7177: f64 = (p.p599 + assign8080_e7176);
        let assign8080_e7180: f64 = (p.p601 * var_iwe);
        let assign8080_e7181: f64 = (assign8080_e7177 + assign8080_e7180);
        let assign8080_e7184: f64 = (p.p602 * var_iae);
        let assign8080_e7185: f64 = (assign8080_e7181 + assign8080_e7184);
        (assign8080_e7185,)
    } else {
        (var_alp2_p,)
    }
};
        var_alp2_p = assign8080_e7187;
        var_alp2_p_rv = 0.0;

        let assign8090_e7206: f64 = if (((param_given[603] || param_given[604]) || param_given[605]) || param_given[606]) { 1.0 } else { 0.0 };
        var_guard94 = assign8090_e7206;
        var_guard94_rv = 0.0;

        let (assign8100_e7224,) = {
    if ((var_guard41 != 0.0) && (var_guard94 != 0.0)) {
        let assign8100_e7213: f64 = (p.p604 * var_ile);
        let assign8100_e7214: f64 = (p.p603 + assign8100_e7213);
        let assign8100_e7217: f64 = (p.p605 * var_iwe);
        let assign8100_e7218: f64 = (assign8100_e7214 + assign8100_e7217);
        let assign8100_e7221: f64 = (p.p606 * var_iae);
        let assign8100_e7222: f64 = (assign8100_e7218 + assign8100_e7221);
        (assign8100_e7222,)
    } else {
        (var_a1_p,)
    }
};
        var_a1_p = assign8100_e7224;
        var_a1_p_rv = 0.0;

        let assign8110_e7243: f64 = if (((param_given[607] || param_given[608]) || param_given[609]) || param_given[610]) { 1.0 } else { 0.0 };
        var_guard95 = assign8110_e7243;
        var_guard95_rv = 0.0;

        let (assign8120_e7261,) = {
    if ((var_guard41 != 0.0) && (var_guard95 != 0.0)) {
        let assign8120_e7250: f64 = (p.p608 * var_ile);
        let assign8120_e7251: f64 = (p.p607 + assign8120_e7250);
        let assign8120_e7254: f64 = (p.p609 * var_iwe);
        let assign8120_e7255: f64 = (assign8120_e7251 + assign8120_e7254);
        let assign8120_e7258: f64 = (p.p610 * var_iae);
        let assign8120_e7259: f64 = (assign8120_e7255 + assign8120_e7258);
        (assign8120_e7259,)
    } else {
        (var_sta2_p,)
    }
};
        var_sta2_p = assign8120_e7261;
        var_sta2_p_rv = 0.0;

        let assign8130_e7280: f64 = if (((param_given[611] || param_given[612]) || param_given[613]) || param_given[614]) { 1.0 } else { 0.0 };
        var_guard96 = assign8130_e7280;
        var_guard96_rv = 0.0;

        let (assign8140_e7298,) = {
    if ((var_guard41 != 0.0) && (var_guard96 != 0.0)) {
        let assign8140_e7287: f64 = (p.p612 * var_ile);
        let assign8140_e7288: f64 = (p.p611 + assign8140_e7287);
        let assign8140_e7291: f64 = (p.p613 * var_iwe);
        let assign8140_e7292: f64 = (assign8140_e7288 + assign8140_e7291);
        let assign8140_e7295: f64 = (p.p614 * var_iae);
        let assign8140_e7296: f64 = (assign8140_e7292 + assign8140_e7295);
        (assign8140_e7296,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign8140_e7298;
        var_a3_p_rv = 0.0;

        let assign8150_e7317: f64 = if (((param_given[615] || param_given[616]) || param_given[617]) || param_given[618]) { 1.0 } else { 0.0 };
        var_guard97 = assign8150_e7317;
        var_guard97_rv = 0.0;

        let (assign8160_e7335,) = {
    if ((var_guard41 != 0.0) && (var_guard97 != 0.0)) {
        let assign8160_e7324: f64 = (p.p616 * var_ile);
        let assign8160_e7325: f64 = (p.p615 + assign8160_e7324);
        let assign8160_e7328: f64 = (p.p617 * var_iwe);
        let assign8160_e7329: f64 = (assign8160_e7325 + assign8160_e7328);
        let assign8160_e7332: f64 = (p.p618 * var_iae);
        let assign8160_e7333: f64 = (assign8160_e7329 + assign8160_e7332);
        (assign8160_e7333,)
    } else {
        (var_a4_p,)
    }
};
        var_a4_p = assign8160_e7335;
        var_a4_p_rv = 0.0;

        let assign8170_e7354: f64 = if (((param_given[619] || param_given[620]) || param_given[621]) || param_given[622]) { 1.0 } else { 0.0 };
        var_guard98 = assign8170_e7354;
        var_guard98_rv = 0.0;

        let (assign8180_e7374,) = {
    if ((var_guard41 != 0.0) && (var_guard98 != 0.0)) {
        let assign8180_e7362: f64 = (p.p620 * var_ile);
        let assign8180_e7363: f64 = (p.p619 + assign8180_e7362);
        let assign8180_e7366: f64 = (p.p621 * var_iwe);
        let assign8180_e7367: f64 = (assign8180_e7363 + assign8180_e7366);
        let assign8180_e7370: f64 = (p.p622 * var_iae);
        let assign8180_e7371: f64 = (assign8180_e7367 + assign8180_e7370);
        let assign8180_e7372: f64 = (var_iiae * assign8180_e7371);
        (assign8180_e7372,)
    } else {
        (var_iginv_p,)
    }
};
        var_iginv_p = assign8180_e7374;
        var_iginv_p_rv = 0.0;

        let assign8190_e7393: f64 = if (((param_given[623] || param_given[624]) || param_given[625]) || param_given[626]) { 1.0 } else { 0.0 };
        var_guard99 = assign8190_e7393;
        var_guard99_rv = 0.0;

        let (assign8200_e7413,) = {
    if ((var_guard41 != 0.0) && (var_guard99 != 0.0)) {
        let assign8200_e7401: f64 = (p.p624 * var_ile);
        let assign8200_e7402: f64 = (p.p623 + assign8200_e7401);
        let assign8200_e7405: f64 = (p.p625 * var_iwe);
        let assign8200_e7406: f64 = (assign8200_e7402 + assign8200_e7405);
        let assign8200_e7409: f64 = (p.p626 * var_iae);
        let assign8200_e7410: f64 = (assign8200_e7406 + assign8200_e7409);
        let assign8200_e7411: f64 = (var_iiwe * assign8200_e7410);
        (assign8200_e7411,)
    } else {
        (var_igov_p,)
    }
};
        var_igov_p = assign8200_e7413;
        var_igov_p_rv = 0.0;

        let assign8210_e7432: f64 = if (((param_given[627] || param_given[628]) || param_given[629]) || param_given[630]) { 1.0 } else { 0.0 };
        var_guard100 = assign8210_e7432;
        var_guard100_rv = 0.0;

        let (assign8220_e7452,) = {
    if ((var_guard41 != 0.0) && (var_guard100 != 0.0)) {
        let assign8220_e7440: f64 = (p.p628 * var_ile);
        let assign8220_e7441: f64 = (p.p627 + assign8220_e7440);
        let assign8220_e7444: f64 = (p.p629 * var_iwe);
        let assign8220_e7445: f64 = (assign8220_e7441 + assign8220_e7444);
        let assign8220_e7448: f64 = (p.p630 * var_iae);
        let assign8220_e7449: f64 = (assign8220_e7445 + assign8220_e7448);
        let assign8220_e7450: f64 = (var_iiwe * assign8220_e7449);
        (assign8220_e7450,)
    } else {
        (var_igovd_p,)
    }
};
        var_igovd_p = assign8220_e7452;
        var_igovd_p_rv = 0.0;

        let assign8230_e7471: f64 = if (((param_given[631] || param_given[632]) || param_given[633]) || param_given[634]) { 1.0 } else { 0.0 };
        var_guard101 = assign8230_e7471;
        var_guard101_rv = 0.0;

        let (assign8240_e7489,) = {
    if ((var_guard41 != 0.0) && (var_guard101 != 0.0)) {
        let assign8240_e7478: f64 = (p.p632 * var_ile);
        let assign8240_e7479: f64 = (p.p631 + assign8240_e7478);
        let assign8240_e7482: f64 = (p.p633 * var_iwe);
        let assign8240_e7483: f64 = (assign8240_e7479 + assign8240_e7482);
        let assign8240_e7486: f64 = (p.p634 * var_iae);
        let assign8240_e7487: f64 = (assign8240_e7483 + assign8240_e7486);
        (assign8240_e7487,)
    } else {
        (var_stig_p,)
    }
};
        var_stig_p = assign8240_e7489;
        var_stig_p_rv = 0.0;

        let assign8250_e7508: f64 = if (((param_given[635] || param_given[636]) || param_given[637]) || param_given[638]) { 1.0 } else { 0.0 };
        var_guard102 = assign8250_e7508;
        var_guard102_rv = 0.0;

        let (assign8260_e7528,) = {
    if ((var_guard41 != 0.0) && (var_guard102 != 0.0)) {
        let assign8260_e7516: f64 = (p.p636 * var_ile);
        let assign8260_e7517: f64 = (p.p635 + assign8260_e7516);
        let assign8260_e7520: f64 = (p.p637 * var_iwe);
        let assign8260_e7521: f64 = (assign8260_e7517 + assign8260_e7520);
        let assign8260_e7524: f64 = (p.p638 * var_iae);
        let assign8260_e7525: f64 = (assign8260_e7521 + assign8260_e7524);
        let assign8260_e7526: f64 = (var_iiwe * assign8260_e7525);
        (assign8260_e7526,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign8260_e7528;
        var_agidl_p_rv = 0.0;

        let assign8270_e7547: f64 = if (((param_given[639] || param_given[640]) || param_given[641]) || param_given[642]) { 1.0 } else { 0.0 };
        var_guard103 = assign8270_e7547;
        var_guard103_rv = 0.0;

        let (assign8280_e7567,) = {
    if ((var_guard41 != 0.0) && (var_guard103 != 0.0)) {
        let assign8280_e7555: f64 = (p.p640 * var_ile);
        let assign8280_e7556: f64 = (p.p639 + assign8280_e7555);
        let assign8280_e7559: f64 = (p.p641 * var_iwe);
        let assign8280_e7560: f64 = (assign8280_e7556 + assign8280_e7559);
        let assign8280_e7563: f64 = (p.p642 * var_iae);
        let assign8280_e7564: f64 = (assign8280_e7560 + assign8280_e7563);
        let assign8280_e7565: f64 = (var_iiwe * assign8280_e7564);
        (assign8280_e7565,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign8280_e7567;
        var_agidld_p_rv = 0.0;

        let assign8290_e7586: f64 = if (((param_given[643] || param_given[644]) || param_given[645]) || param_given[646]) { 1.0 } else { 0.0 };
        var_guard104 = assign8290_e7586;
        var_guard104_rv = 0.0;

        let (assign8300_e7604,) = {
    if ((var_guard41 != 0.0) && (var_guard104 != 0.0)) {
        let assign8300_e7593: f64 = (p.p644 * var_ile);
        let assign8300_e7594: f64 = (p.p643 + assign8300_e7593);
        let assign8300_e7597: f64 = (p.p645 * var_iwe);
        let assign8300_e7598: f64 = (assign8300_e7594 + assign8300_e7597);
        let assign8300_e7601: f64 = (p.p646 * var_iae);
        let assign8300_e7602: f64 = (assign8300_e7598 + assign8300_e7601);
        (assign8300_e7602,)
    } else {
        (var_stbgidl_p,)
    }
};
        var_stbgidl_p = assign8300_e7604;
        var_stbgidl_p_rv = 0.0;

        let assign8310_e7623: f64 = if (((param_given[647] || param_given[648]) || param_given[649]) || param_given[650]) { 1.0 } else { 0.0 };
        var_guard105 = assign8310_e7623;
        var_guard105_rv = 0.0;

        let (assign8320_e7641,) = {
    if ((var_guard41 != 0.0) && (var_guard105 != 0.0)) {
        let assign8320_e7630: f64 = (p.p648 * var_ile);
        let assign8320_e7631: f64 = (p.p647 + assign8320_e7630);
        let assign8320_e7634: f64 = (p.p649 * var_iwe);
        let assign8320_e7635: f64 = (assign8320_e7631 + assign8320_e7634);
        let assign8320_e7638: f64 = (p.p650 * var_iae);
        let assign8320_e7639: f64 = (assign8320_e7635 + assign8320_e7638);
        (assign8320_e7639,)
    } else {
        (var_stbgidld_p,)
    }
};
        var_stbgidld_p = assign8320_e7641;
        var_stbgidld_p_rv = 0.0;

        let assign8330_e7660: f64 = if (((param_given[651] || param_given[652]) || param_given[653]) || param_given[654]) { 1.0 } else { 0.0 };
        var_guard106 = assign8330_e7660;
        var_guard106_rv = 0.0;

        let (assign8340_e7684,) = {
    if ((var_guard41 != 0.0) && (var_guard106 != 0.0)) {
        let assign8340_e7666: f64 = (var_iiwecv * var_lecv);
        let assign8340_e7668: f64 = (assign8340_e7666 / 1e-6);
        let assign8340_e7672: f64 = (p.p652 * var_ile);
        let assign8340_e7673: f64 = (p.p651 + assign8340_e7672);
        let assign8340_e7676: f64 = (p.p653 * var_iwe);
        let assign8340_e7677: f64 = (assign8340_e7673 + assign8340_e7676);
        let assign8340_e7680: f64 = (p.p654 * var_iae);
        let assign8340_e7681: f64 = (assign8340_e7677 + assign8340_e7680);
        let assign8340_e7682: f64 = (assign8340_e7668 * assign8340_e7681);
        (assign8340_e7682,)
    } else {
        (var_cox_p,)
    }
};
        var_cox_p = assign8340_e7684;
        var_cox_p_rv = 0.0;

        let assign8350_e7703: f64 = if (((param_given[655] || param_given[656]) || param_given[657]) || param_given[658]) { 1.0 } else { 0.0 };
        var_guard107 = assign8350_e7703;
        var_guard107_rv = 0.0;

        let (assign8360_e7721,) = {
    if ((var_guard41 != 0.0) && (var_guard107 != 0.0)) {
        let assign8360_e7710: f64 = (p.p656 * var_ile);
        let assign8360_e7711: f64 = (p.p655 + assign8360_e7710);
        let assign8360_e7714: f64 = (p.p657 * var_iwe);
        let assign8360_e7715: f64 = (assign8360_e7711 + assign8360_e7714);
        let assign8360_e7718: f64 = (p.p658 * var_iae);
        let assign8360_e7719: f64 = (assign8360_e7715 + assign8360_e7718);
        (assign8360_e7719,)
    } else {
        (var_delvtac_p,)
    }
};
        var_delvtac_p = assign8360_e7721;
        var_delvtac_p_rv = 0.0;

        let assign8370_e7740: f64 = if (((param_given[659] || param_given[660]) || param_given[661]) || param_given[662]) { 1.0 } else { 0.0 };
        var_guard108 = assign8370_e7740;
        var_guard108_rv = 0.0;

        let (assign8380_e7758,) = {
    if ((var_guard41 != 0.0) && (var_guard108 != 0.0)) {
        let assign8380_e7747: f64 = (p.p660 * var_ile);
        let assign8380_e7748: f64 = (p.p659 + assign8380_e7747);
        let assign8380_e7751: f64 = (p.p661 * var_iwe);
        let assign8380_e7752: f64 = (assign8380_e7748 + assign8380_e7751);
        let assign8380_e7755: f64 = (p.p662 * var_iae);
        let assign8380_e7756: f64 = (assign8380_e7752 + assign8380_e7755);
        (assign8380_e7756,)
    } else {
        (var_facneffac_p,)
    }
};
        var_facneffac_p = assign8380_e7758;
        var_facneffac_p_rv = 0.0;

        let assign8390_e7797: f64 = if (((((((param_given[663] || param_given[664]) || param_given[665]) || param_given[666]) || param_given[571]) || param_given[572]) || param_given[573]) || param_given[574]) { 1.0 } else { 0.0 };
        var_guard109 = assign8390_e7797;
        var_guard109_rv = 0.0;

        let (assign8400_e7803,) = {
    if ((var_guard41 != 0.0) && (var_guard109 != 0.0)) {
        (p.p571,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8400_e7803;
        var_poparam_i_rv = 0.0;

        let assign8410_e7805: f64 = if param_given[663] { 1.0 } else { 0.0 };
        let assign8410_e7807: f64 = if assign8410_e7805 == 1.0 { 1.0 } else { 0.0 };
        var_guard110 = assign8410_e7807;
        var_guard110_rv = 0.0;

        let (assign8420_e7815,) = {
    if (((var_guard41 != 0.0) && (var_guard109 != 0.0)) && (var_guard110 != 0.0)) {
        (p.p663,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8420_e7815;
        var_poparam_i_rv = 0.0;

        let (assign8430_e7821,) = {
    if ((var_guard41 != 0.0) && (var_guard109 != 0.0)) {
        (p.p572,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8430_e7821;
        var_plparam_i_rv = 0.0;

        let assign8440_e7823: f64 = if param_given[664] { 1.0 } else { 0.0 };
        let assign8440_e7825: f64 = if assign8440_e7823 == 1.0 { 1.0 } else { 0.0 };
        var_guard111 = assign8440_e7825;
        var_guard111_rv = 0.0;

        let (assign8450_e7833,) = {
    if (((var_guard41 != 0.0) && (var_guard109 != 0.0)) && (var_guard111 != 0.0)) {
        (p.p664,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8450_e7833;
        var_plparam_i_rv = 0.0;

        let (assign8460_e7839,) = {
    if ((var_guard41 != 0.0) && (var_guard109 != 0.0)) {
        (p.p573,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8460_e7839;
        var_pwparam_i_rv = 0.0;

        let assign8470_e7841: f64 = if param_given[665] { 1.0 } else { 0.0 };
        let assign8470_e7843: f64 = if assign8470_e7841 == 1.0 { 1.0 } else { 0.0 };
        var_guard112 = assign8470_e7843;
        var_guard112_rv = 0.0;

        let (assign8480_e7851,) = {
    if (((var_guard41 != 0.0) && (var_guard109 != 0.0)) && (var_guard112 != 0.0)) {
        (p.p665,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8480_e7851;
        var_pwparam_i_rv = 0.0;

        let (assign8490_e7857,) = {
    if ((var_guard41 != 0.0) && (var_guard109 != 0.0)) {
        (p.p574,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8490_e7857;
        var_plwparam_i_rv = 0.0;

        let assign8500_e7859: f64 = if param_given[666] { 1.0 } else { 0.0 };
        let assign8500_e7861: f64 = if assign8500_e7859 == 1.0 { 1.0 } else { 0.0 };
        var_guard113 = assign8500_e7861;
        var_guard113_rv = 0.0;

        let (assign8510_e7869,) = {
    if (((var_guard41 != 0.0) && (var_guard109 != 0.0)) && (var_guard113 != 0.0)) {
        (p.p666,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8510_e7869;
        var_plwparam_i_rv = 0.0;

        let (assign8520_e7889,) = {
    if ((var_guard41 != 0.0) && (var_guard109 != 0.0)) {
        let assign8520_e7877: f64 = (var_plparam_i * var_ile);
        let assign8520_e7878: f64 = (var_poparam_i + assign8520_e7877);
        let assign8520_e7881: f64 = (var_pwparam_i * var_iwe);
        let assign8520_e7882: f64 = (assign8520_e7878 + assign8520_e7881);
        let assign8520_e7885: f64 = (var_plwparam_i * var_iae);
        let assign8520_e7886: f64 = (assign8520_e7882 + assign8520_e7885);
        let assign8520_e7887: f64 = (var_ile * assign8520_e7886);
        (assign8520_e7887,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign8520_e7889;
        var_thesatac_p_rv = 0.0;

        let assign8530_e7928: f64 = if (((((((param_given[667] || param_given[668]) || param_given[669]) || param_given[670]) || param_given[587]) || param_given[588]) || param_given[589]) || param_given[590]) { 1.0 } else { 0.0 };
        var_guard114 = assign8530_e7928;
        var_guard114_rv = 0.0;

        let (assign8540_e7934,) = {
    if ((var_guard41 != 0.0) && (var_guard114 != 0.0)) {
        (p.p587,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8540_e7934;
        var_poparam_i_rv = 0.0;

        let assign8550_e7936: f64 = if param_given[667] { 1.0 } else { 0.0 };
        let assign8550_e7938: f64 = if assign8550_e7936 == 1.0 { 1.0 } else { 0.0 };
        var_guard115 = assign8550_e7938;
        var_guard115_rv = 0.0;

        *var_a1_p_slot = var_a1_p;
        *var_a1_p_rv_slot = var_a1_p_rv;
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
        *var_cox_p_slot = var_cox_p;
        *var_cox_p_rv_slot = var_cox_p_rv;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_delvtac_p_rv_slot = var_delvtac_p_rv;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_facneffac_p_rv_slot = var_facneffac_p_rv;
        *var_guard100_slot = var_guard100;
        *var_guard100_rv_slot = var_guard100_rv;
        *var_guard101_slot = var_guard101;
        *var_guard101_rv_slot = var_guard101_rv;
        *var_guard102_slot = var_guard102;
        *var_guard102_rv_slot = var_guard102_rv;
        *var_guard103_slot = var_guard103;
        *var_guard103_rv_slot = var_guard103_rv;
        *var_guard104_slot = var_guard104;
        *var_guard104_rv_slot = var_guard104_rv;
        *var_guard105_slot = var_guard105;
        *var_guard105_rv_slot = var_guard105_rv;
        *var_guard106_slot = var_guard106;
        *var_guard106_rv_slot = var_guard106_rv;
        *var_guard107_slot = var_guard107;
        *var_guard107_rv_slot = var_guard107_rv;
        *var_guard108_slot = var_guard108;
        *var_guard108_rv_slot = var_guard108_rv;
        *var_guard109_slot = var_guard109;
        *var_guard109_rv_slot = var_guard109_rv;
        *var_guard110_slot = var_guard110;
        *var_guard110_rv_slot = var_guard110_rv;
        *var_guard111_slot = var_guard111;
        *var_guard111_rv_slot = var_guard111_rv;
        *var_guard112_slot = var_guard112;
        *var_guard112_rv_slot = var_guard112_rv;
        *var_guard113_slot = var_guard113;
        *var_guard113_rv_slot = var_guard113_rv;
        *var_guard114_slot = var_guard114;
        *var_guard114_rv_slot = var_guard114_rv;
        *var_guard115_slot = var_guard115;
        *var_guard115_rv_slot = var_guard115_rv;
        *var_guard92_slot = var_guard92;
        *var_guard92_rv_slot = var_guard92_rv;
        *var_guard93_slot = var_guard93;
        *var_guard93_rv_slot = var_guard93_rv;
        *var_guard94_slot = var_guard94;
        *var_guard94_rv_slot = var_guard94_rv;
        *var_guard95_slot = var_guard95;
        *var_guard95_rv_slot = var_guard95_rv;
        *var_guard96_slot = var_guard96;
        *var_guard96_rv_slot = var_guard96_rv;
        *var_guard97_slot = var_guard97;
        *var_guard97_rv_slot = var_guard97_rv;
        *var_guard98_slot = var_guard98;
        *var_guard98_rv_slot = var_guard98_rv;
        *var_guard99_slot = var_guard99;
        *var_guard99_rv_slot = var_guard99_rv;
        *var_iginv_p_slot = var_iginv_p;
        *var_iginv_p_rv_slot = var_iginv_p_rv;
        *var_igov_p_slot = var_igov_p;
        *var_igov_p_rv_slot = var_igov_p_rv;
        *var_igovd_p_slot = var_igovd_p;
        *var_igovd_p_rv_slot = var_igovd_p_rv;
        *var_plparam_i_slot = var_plparam_i;
        *var_plparam_i_rv_slot = var_plparam_i_rv;
        *var_plwparam_i_slot = var_plwparam_i;
        *var_plwparam_i_rv_slot = var_plwparam_i_rv;
        *var_poparam_i_slot = var_poparam_i;
        *var_poparam_i_rv_slot = var_poparam_i_rv;
        *var_pwparam_i_slot = var_pwparam_i;
        *var_pwparam_i_rv_slot = var_pwparam_i_rv;
        *var_sta2_p_slot = var_sta2_p;
        *var_sta2_p_rv_slot = var_sta2_p_rv;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidl_p_rv_slot = var_stbgidl_p_rv;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stbgidld_p_rv_slot = var_stbgidld_p_rv;
        *var_stig_p_slot = var_stig_p;
        *var_stig_p_rv_slot = var_stig_p_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard114: f64,
        var_guard115: f64,
        var_guard41: f64,
        var_iae: f64,
        var_iilcv: f64,
        var_iiwcv: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_we_edge: f64,
        var_alp1ac_p_slot: &mut f64,
        var_alp1ac_p_rv_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_alpac_p_rv_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_axac_p_rv_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfr_p_rv_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cfrd_p_rv_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgbov_p_rv_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgov_p_rv_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_cgovd_p_rv_slot: &mut f64,
        var_cinr_p_slot: &mut f64,
        var_cinr_p_rv_slot: &mut f64,
        var_cinrd_p_slot: &mut f64,
        var_cinrd_p_rv_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_ctedge_p_rv_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_dphibedge_p_rv_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_guard116_rv_slot: &mut f64,
        var_guard117_slot: &mut f64,
        var_guard117_rv_slot: &mut f64,
        var_guard118_slot: &mut f64,
        var_guard118_rv_slot: &mut f64,
        var_guard119_slot: &mut f64,
        var_guard119_rv_slot: &mut f64,
        var_guard120_slot: &mut f64,
        var_guard120_rv_slot: &mut f64,
        var_guard121_slot: &mut f64,
        var_guard121_rv_slot: &mut f64,
        var_guard122_slot: &mut f64,
        var_guard122_rv_slot: &mut f64,
        var_guard123_slot: &mut f64,
        var_guard123_rv_slot: &mut f64,
        var_guard124_slot: &mut f64,
        var_guard124_rv_slot: &mut f64,
        var_guard125_slot: &mut f64,
        var_guard125_rv_slot: &mut f64,
        var_guard126_slot: &mut f64,
        var_guard126_rv_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard127_rv_slot: &mut f64,
        var_guard132_slot: &mut f64,
        var_guard132_rv_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard133_rv_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_guard134_rv_slot: &mut f64,
        var_guard135_slot: &mut f64,
        var_guard135_rv_slot: &mut f64,
        var_guard136_slot: &mut f64,
        var_guard136_rv_slot: &mut f64,
        var_guard137_slot: &mut f64,
        var_guard137_rv_slot: &mut f64,
        var_guard138_slot: &mut f64,
        var_guard138_rv_slot: &mut f64,
        var_guard139_slot: &mut f64,
        var_guard139_rv_slot: &mut f64,
        var_guard140_slot: &mut f64,
        var_guard140_rv_slot: &mut f64,
        var_guard141_slot: &mut f64,
        var_guard141_rv_slot: &mut f64,
        var_guard142_slot: &mut f64,
        var_guard142_rv_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_neffedge_p_rv_slot: &mut f64,
        var_plparam_i_slot: &mut f64,
        var_plparam_i_rv_slot: &mut f64,
        var_plwparam_i_slot: &mut f64,
        var_plwparam_i_rv_slot: &mut f64,
        var_poparam_i_slot: &mut f64,
        var_poparam_i_rv_slot: &mut f64,
        var_pscebedge_p_slot: &mut f64,
        var_pscebedge_p_rv_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_pscededge_p_rv_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_psceedge_p_rv_slot: &mut f64,
        var_pwparam_i_slot: &mut f64,
        var_pwparam_i_rv_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_stbetedge_p_rv_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_stvfbedge_p_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
    ) {
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
        let mut var_alp1ac_p_rv: f64 = *var_alp1ac_p_rv_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_alpac_p_rv: f64 = *var_alpac_p_rv_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_axac_p_rv: f64 = *var_axac_p_rv_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfr_p_rv: f64 = *var_cfr_p_rv_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cfrd_p_rv: f64 = *var_cfrd_p_rv_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgbov_p_rv: f64 = *var_cgbov_p_rv_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgov_p_rv: f64 = *var_cgov_p_rv_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_cgovd_p_rv: f64 = *var_cgovd_p_rv_slot;
        let mut var_cinr_p: f64 = *var_cinr_p_slot;
        let mut var_cinr_p_rv: f64 = *var_cinr_p_rv_slot;
        let mut var_cinrd_p: f64 = *var_cinrd_p_slot;
        let mut var_cinrd_p_rv: f64 = *var_cinrd_p_rv_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_ctedge_p_rv: f64 = *var_ctedge_p_rv_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_dphibedge_p_rv: f64 = *var_dphibedge_p_rv_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_guard116_rv: f64 = *var_guard116_rv_slot;
        let mut var_guard117: f64 = *var_guard117_slot;
        let mut var_guard117_rv: f64 = *var_guard117_rv_slot;
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_guard118_rv: f64 = *var_guard118_rv_slot;
        let mut var_guard119: f64 = *var_guard119_slot;
        let mut var_guard119_rv: f64 = *var_guard119_rv_slot;
        let mut var_guard120: f64 = *var_guard120_slot;
        let mut var_guard120_rv: f64 = *var_guard120_rv_slot;
        let mut var_guard121: f64 = *var_guard121_slot;
        let mut var_guard121_rv: f64 = *var_guard121_rv_slot;
        let mut var_guard122: f64 = *var_guard122_slot;
        let mut var_guard122_rv: f64 = *var_guard122_rv_slot;
        let mut var_guard123: f64 = *var_guard123_slot;
        let mut var_guard123_rv: f64 = *var_guard123_rv_slot;
        let mut var_guard124: f64 = *var_guard124_slot;
        let mut var_guard124_rv: f64 = *var_guard124_rv_slot;
        let mut var_guard125: f64 = *var_guard125_slot;
        let mut var_guard125_rv: f64 = *var_guard125_rv_slot;
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard126_rv: f64 = *var_guard126_rv_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard127_rv: f64 = *var_guard127_rv_slot;
        let mut var_guard132: f64 = *var_guard132_slot;
        let mut var_guard132_rv: f64 = *var_guard132_rv_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard133_rv: f64 = *var_guard133_rv_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_guard134_rv: f64 = *var_guard134_rv_slot;
        let mut var_guard135: f64 = *var_guard135_slot;
        let mut var_guard135_rv: f64 = *var_guard135_rv_slot;
        let mut var_guard136: f64 = *var_guard136_slot;
        let mut var_guard136_rv: f64 = *var_guard136_rv_slot;
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard137_rv: f64 = *var_guard137_rv_slot;
        let mut var_guard138: f64 = *var_guard138_slot;
        let mut var_guard138_rv: f64 = *var_guard138_rv_slot;
        let mut var_guard139: f64 = *var_guard139_slot;
        let mut var_guard139_rv: f64 = *var_guard139_rv_slot;
        let mut var_guard140: f64 = *var_guard140_slot;
        let mut var_guard140_rv: f64 = *var_guard140_rv_slot;
        let mut var_guard141: f64 = *var_guard141_slot;
        let mut var_guard141_rv: f64 = *var_guard141_rv_slot;
        let mut var_guard142: f64 = *var_guard142_slot;
        let mut var_guard142_rv: f64 = *var_guard142_rv_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_neffedge_p_rv: f64 = *var_neffedge_p_rv_slot;
        let mut var_plparam_i: f64 = *var_plparam_i_slot;
        let mut var_plparam_i_rv: f64 = *var_plparam_i_rv_slot;
        let mut var_plwparam_i: f64 = *var_plwparam_i_slot;
        let mut var_plwparam_i_rv: f64 = *var_plwparam_i_rv_slot;
        let mut var_poparam_i: f64 = *var_poparam_i_slot;
        let mut var_poparam_i_rv: f64 = *var_poparam_i_rv_slot;
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_pscebedge_p_rv: f64 = *var_pscebedge_p_rv_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_pscededge_p_rv: f64 = *var_pscededge_p_rv_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_psceedge_p_rv: f64 = *var_psceedge_p_rv_slot;
        let mut var_pwparam_i: f64 = *var_pwparam_i_slot;
        let mut var_pwparam_i_rv: f64 = *var_pwparam_i_rv_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stbetedge_p_rv: f64 = *var_stbetedge_p_rv_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_stvfbedge_p_rv: f64 = *var_stvfbedge_p_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;

        let (assign8560_e7946,) = {
    if (((var_guard41 != 0.0) && (var_guard114 != 0.0)) && (var_guard115 != 0.0)) {
        (p.p667,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8560_e7946;
        var_poparam_i_rv = 0.0;

        let (assign8570_e7952,) = {
    if ((var_guard41 != 0.0) && (var_guard114 != 0.0)) {
        (p.p588,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8570_e7952;
        var_plparam_i_rv = 0.0;

        let assign8580_e7954: f64 = if param_given[668] { 1.0 } else { 0.0 };
        let assign8580_e7956: f64 = if assign8580_e7954 == 1.0 { 1.0 } else { 0.0 };
        var_guard116 = assign8580_e7956;
        var_guard116_rv = 0.0;

        let (assign8590_e7964,) = {
    if (((var_guard41 != 0.0) && (var_guard114 != 0.0)) && (var_guard116 != 0.0)) {
        (p.p668,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8590_e7964;
        var_plparam_i_rv = 0.0;

        let (assign8600_e7970,) = {
    if ((var_guard41 != 0.0) && (var_guard114 != 0.0)) {
        (p.p589,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8600_e7970;
        var_pwparam_i_rv = 0.0;

        let assign8610_e7972: f64 = if param_given[669] { 1.0 } else { 0.0 };
        let assign8610_e7974: f64 = if assign8610_e7972 == 1.0 { 1.0 } else { 0.0 };
        var_guard117 = assign8610_e7974;
        var_guard117_rv = 0.0;

        let (assign8620_e7982,) = {
    if (((var_guard41 != 0.0) && (var_guard114 != 0.0)) && (var_guard117 != 0.0)) {
        (p.p669,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8620_e7982;
        var_pwparam_i_rv = 0.0;

        let (assign8630_e7988,) = {
    if ((var_guard41 != 0.0) && (var_guard114 != 0.0)) {
        (p.p590,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8630_e7988;
        var_plwparam_i_rv = 0.0;

        let assign8640_e7990: f64 = if param_given[670] { 1.0 } else { 0.0 };
        let assign8640_e7992: f64 = if assign8640_e7990 == 1.0 { 1.0 } else { 0.0 };
        var_guard118 = assign8640_e7992;
        var_guard118_rv = 0.0;

        let (assign8650_e8000,) = {
    if (((var_guard41 != 0.0) && (var_guard114 != 0.0)) && (var_guard118 != 0.0)) {
        (p.p670,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8650_e8000;
        var_plwparam_i_rv = 0.0;

        let (assign8660_e8020,) = {
    if ((var_guard41 != 0.0) && (var_guard114 != 0.0)) {
        let assign8660_e8008: f64 = (var_plparam_i * var_ile);
        let assign8660_e8009: f64 = (var_poparam_i + assign8660_e8008);
        let assign8660_e8012: f64 = (var_pwparam_i * var_iwe);
        let assign8660_e8013: f64 = (assign8660_e8009 + assign8660_e8012);
        let assign8660_e8016: f64 = (var_plwparam_i * var_iae);
        let assign8660_e8017: f64 = (assign8660_e8013 + assign8660_e8016);
        let assign8660_e8018: f64 = assign8660_e8017;
        (assign8660_e8018,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign8660_e8020;
        var_axac_p_rv = 0.0;

        let assign8670_e8039: f64 = if (((param_given[671] || param_given[672]) || param_given[673]) || param_given[674]) { 1.0 } else { 0.0 };
        var_guard119 = assign8670_e8039;
        var_guard119_rv = 0.0;

        let (assign8680_e8059,) = {
    if ((var_guard41 != 0.0) && (var_guard119 != 0.0)) {
        let assign8680_e8047: f64 = (p.p672 * var_ile);
        let assign8680_e8048: f64 = (p.p671 + assign8680_e8047);
        let assign8680_e8051: f64 = (p.p673 * var_iwe);
        let assign8680_e8052: f64 = (assign8680_e8048 + assign8680_e8051);
        let assign8680_e8055: f64 = (p.p674 * var_iae);
        let assign8680_e8056: f64 = (assign8680_e8052 + assign8680_e8055);
        let assign8680_e8057: f64 = (var_ile * assign8680_e8056);
        (assign8680_e8057,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign8680_e8059;
        var_alpac_p_rv = 0.0;

        let assign8690_e8078: f64 = if (((param_given[675] || param_given[676]) || param_given[677]) || param_given[678]) { 1.0 } else { 0.0 };
        var_guard120 = assign8690_e8078;
        var_guard120_rv = 0.0;

        let (assign8700_e8098,) = {
    if ((var_guard41 != 0.0) && (var_guard120 != 0.0)) {
        let assign8700_e8086: f64 = (p.p676 * var_ile);
        let assign8700_e8087: f64 = (p.p675 + assign8700_e8086);
        let assign8700_e8090: f64 = (p.p677 * var_iwe);
        let assign8700_e8091: f64 = (assign8700_e8087 + assign8700_e8090);
        let assign8700_e8094: f64 = (p.p678 * var_iae);
        let assign8700_e8095: f64 = (assign8700_e8091 + assign8700_e8094);
        let assign8700_e8096: f64 = (var_ile * assign8700_e8095);
        (assign8700_e8096,)
    } else {
        (var_alp1ac_p,)
    }
};
        var_alp1ac_p = assign8700_e8098;
        var_alp1ac_p_rv = 0.0;

        let assign8710_e8117: f64 = if (((param_given[679] || param_given[680]) || param_given[681]) || param_given[682]) { 1.0 } else { 0.0 };
        var_guard121 = assign8710_e8117;
        var_guard121_rv = 0.0;

        let (assign8720_e8137,) = {
    if ((var_guard41 != 0.0) && (var_guard121 != 0.0)) {
        let assign8720_e8125: f64 = (p.p680 * var_ile);
        let assign8720_e8126: f64 = (p.p679 + assign8720_e8125);
        let assign8720_e8129: f64 = (p.p681 * var_iwe);
        let assign8720_e8130: f64 = (assign8720_e8126 + assign8720_e8129);
        let assign8720_e8133: f64 = (p.p682 * var_iae);
        let assign8720_e8134: f64 = (assign8720_e8130 + assign8720_e8133);
        let assign8720_e8135: f64 = (var_iiwecv * assign8720_e8134);
        (assign8720_e8135,)
    } else {
        (var_cgov_p,)
    }
};
        var_cgov_p = assign8720_e8137;
        var_cgov_p_rv = 0.0;

        let assign8730_e8156: f64 = if (((param_given[683] || param_given[684]) || param_given[685]) || param_given[686]) { 1.0 } else { 0.0 };
        var_guard122 = assign8730_e8156;
        var_guard122_rv = 0.0;

        let (assign8740_e8176,) = {
    if ((var_guard41 != 0.0) && (var_guard122 != 0.0)) {
        let assign8740_e8164: f64 = (p.p684 * var_ile);
        let assign8740_e8165: f64 = (p.p683 + assign8740_e8164);
        let assign8740_e8168: f64 = (p.p685 * var_iwe);
        let assign8740_e8169: f64 = (assign8740_e8165 + assign8740_e8168);
        let assign8740_e8172: f64 = (p.p686 * var_iae);
        let assign8740_e8173: f64 = (assign8740_e8169 + assign8740_e8172);
        let assign8740_e8174: f64 = (var_iiwecv * assign8740_e8173);
        (assign8740_e8174,)
    } else {
        (var_cgovd_p,)
    }
};
        var_cgovd_p = assign8740_e8176;
        var_cgovd_p_rv = 0.0;

        let assign8750_e8195: f64 = if (((param_given[687] || param_given[688]) || param_given[689]) || param_given[690]) { 1.0 } else { 0.0 };
        var_guard123 = assign8750_e8195;
        var_guard123_rv = 0.0;

        let (assign8760_e8215,) = {
    if ((var_guard41 != 0.0) && (var_guard123 != 0.0)) {
        let assign8760_e8203: f64 = (p.p688 * var_ile);
        let assign8760_e8204: f64 = (p.p687 + assign8760_e8203);
        let assign8760_e8207: f64 = (p.p689 * var_iwe);
        let assign8760_e8208: f64 = (assign8760_e8204 + assign8760_e8207);
        let assign8760_e8211: f64 = (p.p690 * var_iae);
        let assign8760_e8212: f64 = (assign8760_e8208 + assign8760_e8211);
        let assign8760_e8213: f64 = (var_iilcv * assign8760_e8212);
        (assign8760_e8213,)
    } else {
        (var_cgbov_p,)
    }
};
        var_cgbov_p = assign8760_e8215;
        var_cgbov_p_rv = 0.0;

        let assign8770_e8234: f64 = if (((param_given[691] || param_given[692]) || param_given[693]) || param_given[694]) { 1.0 } else { 0.0 };
        var_guard124 = assign8770_e8234;
        var_guard124_rv = 0.0;

        let (assign8780_e8254,) = {
    if ((var_guard41 != 0.0) && (var_guard124 != 0.0)) {
        let assign8780_e8242: f64 = (p.p692 * var_ile);
        let assign8780_e8243: f64 = (p.p691 + assign8780_e8242);
        let assign8780_e8246: f64 = (p.p693 * var_iwe);
        let assign8780_e8247: f64 = (assign8780_e8243 + assign8780_e8246);
        let assign8780_e8250: f64 = (p.p694 * var_iae);
        let assign8780_e8251: f64 = (assign8780_e8247 + assign8780_e8250);
        let assign8780_e8252: f64 = (var_iiwecv * assign8780_e8251);
        (assign8780_e8252,)
    } else {
        (var_cinr_p,)
    }
};
        var_cinr_p = assign8780_e8254;
        var_cinr_p_rv = 0.0;

        let assign8790_e8273: f64 = if (((param_given[695] || param_given[696]) || param_given[697]) || param_given[698]) { 1.0 } else { 0.0 };
        var_guard125 = assign8790_e8273;
        var_guard125_rv = 0.0;

        let (assign8800_e8293,) = {
    if ((var_guard41 != 0.0) && (var_guard125 != 0.0)) {
        let assign8800_e8281: f64 = (p.p696 * var_ile);
        let assign8800_e8282: f64 = (p.p695 + assign8800_e8281);
        let assign8800_e8285: f64 = (p.p697 * var_iwe);
        let assign8800_e8286: f64 = (assign8800_e8282 + assign8800_e8285);
        let assign8800_e8289: f64 = (p.p698 * var_iae);
        let assign8800_e8290: f64 = (assign8800_e8286 + assign8800_e8289);
        let assign8800_e8291: f64 = (var_iiwecv * assign8800_e8290);
        (assign8800_e8291,)
    } else {
        (var_cinrd_p,)
    }
};
        var_cinrd_p = assign8800_e8293;
        var_cinrd_p_rv = 0.0;

        let assign8810_e8312: f64 = if (((param_given[699] || param_given[700]) || param_given[701]) || param_given[702]) { 1.0 } else { 0.0 };
        var_guard126 = assign8810_e8312;
        var_guard126_rv = 0.0;

        let (assign8820_e8332,) = {
    if ((var_guard41 != 0.0) && (var_guard126 != 0.0)) {
        let assign8820_e8320: f64 = (p.p700 * var_ile);
        let assign8820_e8321: f64 = (p.p699 + assign8820_e8320);
        let assign8820_e8324: f64 = (p.p701 * var_iwe);
        let assign8820_e8325: f64 = (assign8820_e8321 + assign8820_e8324);
        let assign8820_e8328: f64 = (p.p702 * var_iae);
        let assign8820_e8329: f64 = (assign8820_e8325 + assign8820_e8328);
        let assign8820_e8330: f64 = (var_iiwcv * assign8820_e8329);
        (assign8820_e8330,)
    } else {
        (var_cfr_p,)
    }
};
        var_cfr_p = assign8820_e8332;
        var_cfr_p_rv = 0.0;

        let assign8830_e8351: f64 = if (((param_given[703] || param_given[704]) || param_given[705]) || param_given[706]) { 1.0 } else { 0.0 };
        var_guard127 = assign8830_e8351;
        var_guard127_rv = 0.0;

        let (assign8840_e8371,) = {
    if ((var_guard41 != 0.0) && (var_guard127 != 0.0)) {
        let assign8840_e8359: f64 = (p.p704 * var_ile);
        let assign8840_e8360: f64 = (p.p703 + assign8840_e8359);
        let assign8840_e8363: f64 = (p.p705 * var_iwe);
        let assign8840_e8364: f64 = (assign8840_e8360 + assign8840_e8363);
        let assign8840_e8367: f64 = (p.p706 * var_iae);
        let assign8840_e8368: f64 = (assign8840_e8364 + assign8840_e8367);
        let assign8840_e8369: f64 = (var_iiwcv * assign8840_e8368);
        (assign8840_e8369,)
    } else {
        (var_cfrd_p,)
    }
};
        var_cfrd_p = assign8840_e8371;
        var_cfrd_p_rv = 0.0;

        let assign8930_e8546: f64 = if (((param_given[723] || param_given[724]) || param_given[725]) || param_given[726]) { 1.0 } else { 0.0 };
        var_guard132 = assign8930_e8546;
        var_guard132_rv = 0.0;

        let (assign8940_e8564,) = {
    if ((var_guard41 != 0.0) && (var_guard132 != 0.0)) {
        let assign8940_e8553: f64 = (p.p724 * var_ile);
        let assign8940_e8554: f64 = (p.p723 + assign8940_e8553);
        let assign8940_e8557: f64 = (p.p725 * var_iwe);
        let assign8940_e8558: f64 = (assign8940_e8554 + assign8940_e8557);
        let assign8940_e8561: f64 = (p.p726 * var_iae);
        let assign8940_e8562: f64 = (assign8940_e8558 + assign8940_e8561);
        (assign8940_e8562,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign8940_e8564;
        var_vfbedge_p_rv = 0.0;

        let assign8950_e8583: f64 = if (((param_given[727] || param_given[728]) || param_given[729]) || param_given[730]) { 1.0 } else { 0.0 };
        var_guard133 = assign8950_e8583;
        var_guard133_rv = 0.0;

        let (assign8960_e8601,) = {
    if ((var_guard41 != 0.0) && (var_guard133 != 0.0)) {
        let assign8960_e8590: f64 = (p.p728 * var_ile);
        let assign8960_e8591: f64 = (p.p727 + assign8960_e8590);
        let assign8960_e8594: f64 = (p.p729 * var_iwe);
        let assign8960_e8595: f64 = (assign8960_e8591 + assign8960_e8594);
        let assign8960_e8598: f64 = (p.p730 * var_iae);
        let assign8960_e8599: f64 = (assign8960_e8595 + assign8960_e8598);
        (assign8960_e8599,)
    } else {
        (var_stvfbedge_p,)
    }
};
        var_stvfbedge_p = assign8960_e8601;
        var_stvfbedge_p_rv = 0.0;

        let assign8970_e8620: f64 = if (((param_given[731] || param_given[732]) || param_given[733]) || param_given[734]) { 1.0 } else { 0.0 };
        var_guard134 = assign8970_e8620;
        var_guard134_rv = 0.0;

        let (assign8980_e8638,) = {
    if ((var_guard41 != 0.0) && (var_guard134 != 0.0)) {
        let assign8980_e8627: f64 = (p.p732 * var_ile);
        let assign8980_e8628: f64 = (p.p731 + assign8980_e8627);
        let assign8980_e8631: f64 = (p.p733 * var_iwe);
        let assign8980_e8632: f64 = (assign8980_e8628 + assign8980_e8631);
        let assign8980_e8635: f64 = (p.p734 * var_iae);
        let assign8980_e8636: f64 = (assign8980_e8632 + assign8980_e8635);
        (assign8980_e8636,)
    } else {
        (var_dphibedge_p,)
    }
};
        var_dphibedge_p = assign8980_e8638;
        var_dphibedge_p_rv = 0.0;

        let assign8990_e8657: f64 = if (((param_given[735] || param_given[736]) || param_given[737]) || param_given[738]) { 1.0 } else { 0.0 };
        var_guard135 = assign8990_e8657;
        var_guard135_rv = 0.0;

        let (assign9000_e8675,) = {
    if ((var_guard41 != 0.0) && (var_guard135 != 0.0)) {
        let assign9000_e8664: f64 = (p.p736 * var_ile);
        let assign9000_e8665: f64 = (p.p735 + assign9000_e8664);
        let assign9000_e8668: f64 = (p.p737 * var_iwe);
        let assign9000_e8669: f64 = (assign9000_e8665 + assign9000_e8668);
        let assign9000_e8672: f64 = (p.p738 * var_iae);
        let assign9000_e8673: f64 = (assign9000_e8669 + assign9000_e8672);
        (assign9000_e8673,)
    } else {
        (var_neffedge_p,)
    }
};
        var_neffedge_p = assign9000_e8675;
        var_neffedge_p_rv = 0.0;

        let assign9010_e8694: f64 = if (((param_given[739] || param_given[740]) || param_given[741]) || param_given[742]) { 1.0 } else { 0.0 };
        var_guard136 = assign9010_e8694;
        var_guard136_rv = 0.0;

        let (assign9020_e8712,) = {
    if ((var_guard41 != 0.0) && (var_guard136 != 0.0)) {
        let assign9020_e8701: f64 = (p.p740 * var_ile);
        let assign9020_e8702: f64 = (p.p739 + assign9020_e8701);
        let assign9020_e8705: f64 = (p.p741 * var_iwe);
        let assign9020_e8706: f64 = (assign9020_e8702 + assign9020_e8705);
        let assign9020_e8709: f64 = (p.p742 * var_iae);
        let assign9020_e8710: f64 = (assign9020_e8706 + assign9020_e8709);
        (assign9020_e8710,)
    } else {
        (var_ctedge_p,)
    }
};
        var_ctedge_p = assign9020_e8712;
        var_ctedge_p_rv = 0.0;

        let assign9030_e8731: f64 = if (((param_given[743] || param_given[744]) || param_given[745]) || param_given[746]) { 1.0 } else { 0.0 };
        var_guard137 = assign9030_e8731;
        var_guard137_rv = 0.0;

        let (assign9040_e8753,) = {
    if ((var_guard41 != 0.0) && (var_guard137 != 0.0)) {
        let assign9040_e8737: f64 = (var_we_edge / var_le);
        let assign9040_e8741: f64 = (p.p744 * var_ile);
        let assign9040_e8742: f64 = (p.p743 + assign9040_e8741);
        let assign9040_e8745: f64 = (p.p745 * var_iwe);
        let assign9040_e8746: f64 = (assign9040_e8742 + assign9040_e8745);
        let assign9040_e8749: f64 = (p.p746 * var_iae);
        let assign9040_e8750: f64 = (assign9040_e8746 + assign9040_e8749);
        let assign9040_e8751: f64 = (assign9040_e8737 * assign9040_e8750);
        (assign9040_e8751,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign9040_e8753;
        var_betnedge_p_rv = 0.0;

        let assign9050_e8772: f64 = if (((param_given[747] || param_given[748]) || param_given[749]) || param_given[750]) { 1.0 } else { 0.0 };
        var_guard138 = assign9050_e8772;
        var_guard138_rv = 0.0;

        let (assign9060_e8790,) = {
    if ((var_guard41 != 0.0) && (var_guard138 != 0.0)) {
        let assign9060_e8779: f64 = (p.p748 * var_ile);
        let assign9060_e8780: f64 = (p.p747 + assign9060_e8779);
        let assign9060_e8783: f64 = (p.p749 * var_iwe);
        let assign9060_e8784: f64 = (assign9060_e8780 + assign9060_e8783);
        let assign9060_e8787: f64 = (p.p750 * var_iae);
        let assign9060_e8788: f64 = (assign9060_e8784 + assign9060_e8787);
        (assign9060_e8788,)
    } else {
        (var_stbetedge_p,)
    }
};
        var_stbetedge_p = assign9060_e8790;
        var_stbetedge_p_rv = 0.0;

        let assign9070_e8809: f64 = if (((param_given[751] || param_given[752]) || param_given[753]) || param_given[754]) { 1.0 } else { 0.0 };
        var_guard139 = assign9070_e8809;
        var_guard139_rv = 0.0;

        let (assign9080_e8829,) = {
    if ((var_guard41 != 0.0) && (var_guard139 != 0.0)) {
        let assign9080_e8817: f64 = (p.p752 * var_ile);
        let assign9080_e8818: f64 = (p.p751 + assign9080_e8817);
        let assign9080_e8821: f64 = (p.p753 * var_iwe);
        let assign9080_e8822: f64 = (assign9080_e8818 + assign9080_e8821);
        let assign9080_e8825: f64 = (p.p754 * var_iae);
        let assign9080_e8826: f64 = (assign9080_e8822 + assign9080_e8825);
        let assign9080_e8827: f64 = (var_ile2 * assign9080_e8826);
        (assign9080_e8827,)
    } else {
        (var_psceedge_p,)
    }
};
        var_psceedge_p = assign9080_e8829;
        var_psceedge_p_rv = 0.0;

        let assign9090_e8848: f64 = if (((param_given[755] || param_given[756]) || param_given[757]) || param_given[758]) { 1.0 } else { 0.0 };
        var_guard140 = assign9090_e8848;
        var_guard140_rv = 0.0;

        let (assign9100_e8866,) = {
    if ((var_guard41 != 0.0) && (var_guard140 != 0.0)) {
        let assign9100_e8855: f64 = (p.p756 * var_ile);
        let assign9100_e8856: f64 = (p.p755 + assign9100_e8855);
        let assign9100_e8859: f64 = (p.p757 * var_iwe);
        let assign9100_e8860: f64 = (assign9100_e8856 + assign9100_e8859);
        let assign9100_e8863: f64 = (p.p758 * var_iae);
        let assign9100_e8864: f64 = (assign9100_e8860 + assign9100_e8863);
        (assign9100_e8864,)
    } else {
        (var_pscebedge_p,)
    }
};
        var_pscebedge_p = assign9100_e8866;
        var_pscebedge_p_rv = 0.0;

        let assign9110_e8885: f64 = if (((param_given[759] || param_given[760]) || param_given[761]) || param_given[762]) { 1.0 } else { 0.0 };
        var_guard141 = assign9110_e8885;
        var_guard141_rv = 0.0;

        let (assign9120_e8903,) = {
    if ((var_guard41 != 0.0) && (var_guard141 != 0.0)) {
        let assign9120_e8892: f64 = (p.p760 * var_ile);
        let assign9120_e8893: f64 = (p.p759 + assign9120_e8892);
        let assign9120_e8896: f64 = (p.p761 * var_iwe);
        let assign9120_e8897: f64 = (assign9120_e8893 + assign9120_e8896);
        let assign9120_e8900: f64 = (p.p762 * var_iae);
        let assign9120_e8901: f64 = (assign9120_e8897 + assign9120_e8900);
        (assign9120_e8901,)
    } else {
        (var_pscededge_p,)
    }
};
        var_pscededge_p = assign9120_e8903;
        var_pscededge_p_rv = 0.0;

        let assign9130_e8922: f64 = if (((param_given[763] || param_given[764]) || param_given[765]) || param_given[766]) { 1.0 } else { 0.0 };
        var_guard142 = assign9130_e8922;
        var_guard142_rv = 0.0;

        *var_alp1ac_p_slot = var_alp1ac_p;
        *var_alp1ac_p_rv_slot = var_alp1ac_p_rv;
        *var_alpac_p_slot = var_alpac_p;
        *var_alpac_p_rv_slot = var_alpac_p_rv;
        *var_axac_p_slot = var_axac_p;
        *var_axac_p_rv_slot = var_axac_p_rv;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfr_p_rv_slot = var_cfr_p_rv;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cfrd_p_rv_slot = var_cfrd_p_rv;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgbov_p_rv_slot = var_cgbov_p_rv;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgov_p_rv_slot = var_cgov_p_rv;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_cgovd_p_rv_slot = var_cgovd_p_rv;
        *var_cinr_p_slot = var_cinr_p;
        *var_cinr_p_rv_slot = var_cinr_p_rv;
        *var_cinrd_p_slot = var_cinrd_p;
        *var_cinrd_p_rv_slot = var_cinrd_p_rv;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_ctedge_p_rv_slot = var_ctedge_p_rv;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_dphibedge_p_rv_slot = var_dphibedge_p_rv;
        *var_guard116_slot = var_guard116;
        *var_guard116_rv_slot = var_guard116_rv;
        *var_guard117_slot = var_guard117;
        *var_guard117_rv_slot = var_guard117_rv;
        *var_guard118_slot = var_guard118;
        *var_guard118_rv_slot = var_guard118_rv;
        *var_guard119_slot = var_guard119;
        *var_guard119_rv_slot = var_guard119_rv;
        *var_guard120_slot = var_guard120;
        *var_guard120_rv_slot = var_guard120_rv;
        *var_guard121_slot = var_guard121;
        *var_guard121_rv_slot = var_guard121_rv;
        *var_guard122_slot = var_guard122;
        *var_guard122_rv_slot = var_guard122_rv;
        *var_guard123_slot = var_guard123;
        *var_guard123_rv_slot = var_guard123_rv;
        *var_guard124_slot = var_guard124;
        *var_guard124_rv_slot = var_guard124_rv;
        *var_guard125_slot = var_guard125;
        *var_guard125_rv_slot = var_guard125_rv;
        *var_guard126_slot = var_guard126;
        *var_guard126_rv_slot = var_guard126_rv;
        *var_guard127_slot = var_guard127;
        *var_guard127_rv_slot = var_guard127_rv;
        *var_guard132_slot = var_guard132;
        *var_guard132_rv_slot = var_guard132_rv;
        *var_guard133_slot = var_guard133;
        *var_guard133_rv_slot = var_guard133_rv;
        *var_guard134_slot = var_guard134;
        *var_guard134_rv_slot = var_guard134_rv;
        *var_guard135_slot = var_guard135;
        *var_guard135_rv_slot = var_guard135_rv;
        *var_guard136_slot = var_guard136;
        *var_guard136_rv_slot = var_guard136_rv;
        *var_guard137_slot = var_guard137;
        *var_guard137_rv_slot = var_guard137_rv;
        *var_guard138_slot = var_guard138;
        *var_guard138_rv_slot = var_guard138_rv;
        *var_guard139_slot = var_guard139;
        *var_guard139_rv_slot = var_guard139_rv;
        *var_guard140_slot = var_guard140;
        *var_guard140_rv_slot = var_guard140_rv;
        *var_guard141_slot = var_guard141;
        *var_guard141_rv_slot = var_guard141_rv;
        *var_guard142_slot = var_guard142;
        *var_guard142_rv_slot = var_guard142_rv;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_neffedge_p_rv_slot = var_neffedge_p_rv;
        *var_plparam_i_slot = var_plparam_i;
        *var_plparam_i_rv_slot = var_plparam_i_rv;
        *var_plwparam_i_slot = var_plwparam_i;
        *var_plwparam_i_rv_slot = var_plwparam_i_rv;
        *var_poparam_i_slot = var_poparam_i;
        *var_poparam_i_rv_slot = var_poparam_i_rv;
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_pscebedge_p_rv_slot = var_pscebedge_p_rv;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_pscededge_p_rv_slot = var_pscededge_p_rv;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_psceedge_p_rv_slot = var_psceedge_p_rv;
        *var_pwparam_i_slot = var_pwparam_i;
        *var_pwparam_i_rv_slot = var_pwparam_i_rv;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stbetedge_p_rv_slot = var_stbetedge_p_rv;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_stvfbedge_p_rv_slot = var_stvfbedge_p_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_dellps: f64,
        var_delwod: f64,
        var_guard142: f64,
        var_guard41: f64,
        var_iae: f64,
        var_ile: f64,
        var_ile2: f64,
        var_invnf: f64,
        var_iwe: f64,
        var_l_i: f64,
        var_nf_i: f64,
        var_rta: f64,
        var_sa_i: f64,
        var_sb_i: f64,
        var_sd_i: f64,
        var_w_i: f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfbedge_p_rv_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfdedge_p_rv_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfedge_p_rv_slot: &mut f64,
        var_guard143_slot: &mut f64,
        var_guard143_rv_slot: &mut f64,
        var_guard144_slot: &mut f64,
        var_guard144_rv_slot: &mut f64,
        var_guard148_slot: &mut f64,
        var_guard148_rv_slot: &mut f64,
        var_guard149_slot: &mut f64,
        var_guard149_rv_slot: &mut f64,
        var_guard150_slot: &mut f64,
        var_guard150_rv_slot: &mut f64,
        var_invsa_slot: &mut f64,
        var_invsa_rv_slot: &mut f64,
        var_invsaref_slot: &mut f64,
        var_invsaref_rv_slot: &mut f64,
        var_invsb_slot: &mut f64,
        var_invsb_rv_slot: &mut f64,
        var_invsbref_slot: &mut f64,
        var_invsbref_rv_slot: &mut f64,
        var_kstressu0_slot: &mut f64,
        var_kstressu0_rv_slot: &mut f64,
        var_kstressvth0_slot: &mut f64,
        var_kstressvth0_rv_slot: &mut f64,
        var_kvsatac_i_slot: &mut f64,
        var_kvsatac_i_rv_slot: &mut f64,
        var_loop__slot: &mut f64,
        var_loop__rv_slot: &mut f64,
        var_lx_slot: &mut f64,
        var_lx_rv_slot: &mut f64,
        var_munqs_p_slot: &mut f64,
        var_munqs_p_rv_slot: &mut f64,
        var_rhobeta_slot: &mut f64,
        var_rhobeta_rv_slot: &mut f64,
        var_rhobetaref_slot: &mut f64,
        var_rhobetaref_rv_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_temp00_slot: &mut f64,
        var_temp00_rv_slot: &mut f64,
        var_temp0_rv_slot: &mut f64,
        var_templ_slot: &mut f64,
        var_templ_rv_slot: &mut f64,
        var_tempw_slot: &mut f64,
        var_tempw_rv_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpa_rv_slot: &mut f64,
        var_tmpb_slot: &mut f64,
        var_tmpb_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
        var_wx_slot: &mut f64,
        var_wx_rv_slot: &mut f64,
    ) {
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfbedge_p_rv: f64 = *var_cfbedge_p_rv_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfdedge_p_rv: f64 = *var_cfdedge_p_rv_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfedge_p_rv: f64 = *var_cfedge_p_rv_slot;
        let mut var_guard143: f64 = *var_guard143_slot;
        let mut var_guard143_rv: f64 = *var_guard143_rv_slot;
        let mut var_guard144: f64 = *var_guard144_slot;
        let mut var_guard144_rv: f64 = *var_guard144_rv_slot;
        let mut var_guard148: f64 = *var_guard148_slot;
        let mut var_guard148_rv: f64 = *var_guard148_rv_slot;
        let mut var_guard149: f64 = *var_guard149_slot;
        let mut var_guard149_rv: f64 = *var_guard149_rv_slot;
        let mut var_guard150: f64 = *var_guard150_slot;
        let mut var_guard150_rv: f64 = *var_guard150_rv_slot;
        let mut var_invsa: f64 = *var_invsa_slot;
        let mut var_invsa_rv: f64 = *var_invsa_rv_slot;
        let mut var_invsaref: f64 = *var_invsaref_slot;
        let mut var_invsaref_rv: f64 = *var_invsaref_rv_slot;
        let mut var_invsb: f64 = *var_invsb_slot;
        let mut var_invsb_rv: f64 = *var_invsb_rv_slot;
        let mut var_invsbref: f64 = *var_invsbref_slot;
        let mut var_invsbref_rv: f64 = *var_invsbref_rv_slot;
        let mut var_kstressu0: f64 = *var_kstressu0_slot;
        let mut var_kstressu0_rv: f64 = *var_kstressu0_rv_slot;
        let mut var_kstressvth0: f64 = *var_kstressvth0_slot;
        let mut var_kstressvth0_rv: f64 = *var_kstressvth0_rv_slot;
        let mut var_kvsatac_i: f64 = *var_kvsatac_i_slot;
        let mut var_kvsatac_i_rv: f64 = *var_kvsatac_i_rv_slot;
        let mut var_loop_: f64 = *var_loop__slot;
        let mut var_loop__rv: f64 = *var_loop__rv_slot;
        let mut var_lx: f64 = *var_lx_slot;
        let mut var_lx_rv: f64 = *var_lx_rv_slot;
        let mut var_munqs_p: f64 = *var_munqs_p_slot;
        let mut var_munqs_p_rv: f64 = *var_munqs_p_rv_slot;
        let mut var_rhobeta: f64 = *var_rhobeta_slot;
        let mut var_rhobeta_rv: f64 = *var_rhobeta_rv_slot;
        let mut var_rhobetaref: f64 = *var_rhobetaref_slot;
        let mut var_rhobetaref_rv: f64 = *var_rhobetaref_rv_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_temp00: f64 = *var_temp00_slot;
        let mut var_temp00_rv: f64 = *var_temp00_rv_slot;
        let mut var_temp0_rv: f64 = *var_temp0_rv_slot;
        let mut var_templ: f64 = *var_templ_slot;
        let mut var_templ_rv: f64 = *var_templ_rv_slot;
        let mut var_tempw: f64 = *var_tempw_slot;
        let mut var_tempw_rv: f64 = *var_tempw_rv_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_rv: f64 = *var_thesat_p_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpa_rv: f64 = *var_tmpa_rv_slot;
        let mut var_tmpb: f64 = *var_tmpb_slot;
        let mut var_tmpb_rv: f64 = *var_tmpb_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;
        let mut var_wx: f64 = *var_wx_slot;
        let mut var_wx_rv: f64 = *var_wx_rv_slot;

        let (assign9140_e8942,) = {
    if ((var_guard41 != 0.0) && (var_guard142 != 0.0)) {
        let assign9140_e8930: f64 = (p.p764 * var_ile);
        let assign9140_e8931: f64 = (p.p763 + assign9140_e8930);
        let assign9140_e8934: f64 = (p.p765 * var_iwe);
        let assign9140_e8935: f64 = (assign9140_e8931 + assign9140_e8934);
        let assign9140_e8938: f64 = (p.p766 * var_iae);
        let assign9140_e8939: f64 = (assign9140_e8935 + assign9140_e8938);
        let assign9140_e8940: f64 = (var_ile2 * assign9140_e8939);
        (assign9140_e8940,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign9140_e8942;
        var_cfedge_p_rv = 0.0;

        let assign9150_e8961: f64 = if (((param_given[771] || param_given[772]) || param_given[773]) || param_given[774]) { 1.0 } else { 0.0 };
        var_guard143 = assign9150_e8961;
        var_guard143_rv = 0.0;

        let (assign9160_e8979,) = {
    if ((var_guard41 != 0.0) && (var_guard143 != 0.0)) {
        let assign9160_e8968: f64 = (p.p772 * var_ile);
        let assign9160_e8969: f64 = (p.p771 + assign9160_e8968);
        let assign9160_e8972: f64 = (p.p773 * var_iwe);
        let assign9160_e8973: f64 = (assign9160_e8969 + assign9160_e8972);
        let assign9160_e8976: f64 = (p.p774 * var_iae);
        let assign9160_e8977: f64 = (assign9160_e8973 + assign9160_e8976);
        (assign9160_e8977,)
    } else {
        (var_cfdedge_p,)
    }
};
        var_cfdedge_p = assign9160_e8979;
        var_cfdedge_p_rv = 0.0;

        let assign9170_e8998: f64 = if (((param_given[767] || param_given[768]) || param_given[769]) || param_given[770]) { 1.0 } else { 0.0 };
        var_guard144 = assign9170_e8998;
        var_guard144_rv = 0.0;

        let (assign9180_e9016,) = {
    if ((var_guard41 != 0.0) && (var_guard144 != 0.0)) {
        let assign9180_e9005: f64 = (p.p768 * var_ile);
        let assign9180_e9006: f64 = (p.p767 + assign9180_e9005);
        let assign9180_e9009: f64 = (p.p769 * var_iwe);
        let assign9180_e9010: f64 = (assign9180_e9006 + assign9180_e9009);
        let assign9180_e9013: f64 = (p.p770 * var_iae);
        let assign9180_e9014: f64 = (assign9180_e9010 + assign9180_e9013);
        (assign9180_e9014,)
    } else {
        (var_cfbedge_p,)
    }
};
        var_cfbedge_p = assign9180_e9016;
        var_cfbedge_p_rv = 0.0;

        let assign9250_e9152: f64 = if (((param_given[787] || param_given[788]) || param_given[789]) || param_given[790]) { 1.0 } else { 0.0 };
        var_guard148 = assign9250_e9152;
        var_guard148_rv = 0.0;

        let (assign9260_e9170,) = {
    if ((var_guard41 != 0.0) && (var_guard148 != 0.0)) {
        let assign9260_e9159: f64 = (p.p788 * var_ile);
        let assign9260_e9160: f64 = (p.p787 + assign9260_e9159);
        let assign9260_e9163: f64 = (p.p789 * var_iwe);
        let assign9260_e9164: f64 = (assign9260_e9160 + assign9260_e9163);
        let assign9260_e9167: f64 = (p.p790 * var_iae);
        let assign9260_e9168: f64 = (assign9260_e9164 + assign9260_e9167);
        (assign9260_e9168,)
    } else {
        (var_munqs_p,)
    }
};
        var_munqs_p = assign9260_e9170;
        var_munqs_p_rv = 0.0;

        let (assign9270_e9174,) = {
    if (var_guard41 != 0.0) {
        (0.0,)
    } else {
        (var_tmpa,)
    }
};
        var_tmpa = assign9270_e9174;
        var_tmpa_rv = 0.0;

        let (assign9280_e9178,) = {
    if (var_guard41 != 0.0) {
        (0.0,)
    } else {
        (var_tmpb,)
    }
};
        var_tmpb = assign9280_e9178;
        var_tmpb_rv = 0.0;

        let (assign9290_e9182,) = {
    if (var_guard41 != 0.0) {
        (0.0,)
    } else {
        (var_loop_,)
    }
};
        var_loop_ = assign9290_e9182;
        var_loop__rv = 0.0;

        let (assign9300_e9186,) = {
    if (var_guard41 != 0.0) {
        (p.p795,)
    } else {
        (var_kvsatac_i,)
    }
};
        var_kvsatac_i = assign9300_e9186;
        var_kvsatac_i_rv = 0.0;

        let assign9310_e9188: f64 = if param_given[796] { 1.0 } else { 0.0 };
        let assign9310_e9190: f64 = if assign9310_e9188 == 1.0 { 1.0 } else { 0.0 };
        var_guard149 = assign9310_e9190;
        var_guard149_rv = 0.0;

        let (assign9320_e9196,) = {
    if ((var_guard41 != 0.0) && (var_guard149 != 0.0)) {
        (p.p796,)
    } else {
        (var_kvsatac_i,)
    }
};
        var_kvsatac_i = assign9320_e9196;
        var_kvsatac_i_rv = 0.0;

        let assign9330_e9215: f64 = if (((var_sa_i > 0.0) && (var_sb_i > 0.0)) && ((var_nf_i == 1.0) || ((var_nf_i > 1.0) && (var_sd_i > 0.0)))) { 1.0 } else { 0.0 };
        var_guard150 = assign9330_e9215;
        var_guard150_rv = 0.0;

        let mut assign9340_loop_guard: usize = 0;
        while {
            let assign9340_cond_e9222: f64 = (var_nf_i - 0.5);
            let assign9340_cond_e9224: f64 = if (((var_guard41 != 0.0) && (var_guard150 != 0.0)) && (var_loop_ < assign9340_cond_e9222)) { 1.0 } else { 0.0 };
            assign9340_cond_e9224 != 0.0
        } {
            assign9340_loop_guard += 1;
            assert!(assign9340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9340_body0_e9244,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9340_body0_e9233: f64 = (0.5 * var_l_i);
        let assign9340_body0_e9234: f64 = (var_sa_i + assign9340_body0_e9233);
        let assign9340_body0_e9238: f64 = (var_sd_i + var_l_i);
        let assign9340_body0_e9239: f64 = (var_loop_ * assign9340_body0_e9238);
        let assign9340_body0_e9240: f64 = (assign9340_body0_e9234 + assign9340_body0_e9239);
        let assign9340_body0_e9241: f64 = (1.0 / assign9340_body0_e9240);
        let assign9340_body0_e9242: f64 = (var_tmpa + assign9340_body0_e9241);
        (assign9340_body0_e9242,)
    } else {
        (var_tmpa,)
    }
};
            var_tmpa = assign9340_body0_e9244;
            var_tmpa_rv = 0.0;
            let (assign9340_body1_e9264,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9340_body1_e9253: f64 = (0.5 * var_l_i);
        let assign9340_body1_e9254: f64 = (var_sb_i + assign9340_body1_e9253);
        let assign9340_body1_e9258: f64 = (var_sd_i + var_l_i);
        let assign9340_body1_e9259: f64 = (var_loop_ * assign9340_body1_e9258);
        let assign9340_body1_e9260: f64 = (assign9340_body1_e9254 + assign9340_body1_e9259);
        let assign9340_body1_e9261: f64 = (1.0 / assign9340_body1_e9260);
        let assign9340_body1_e9262: f64 = (var_tmpb + assign9340_body1_e9261);
        (assign9340_body1_e9262,)
    } else {
        (var_tmpb,)
    }
};
            var_tmpb = assign9340_body1_e9264;
            var_tmpb_rv = 0.0;
            let (assign9340_body2_e9272,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9340_body2_e9270: f64 = (var_loop_ + 1.0);
        (assign9340_body2_e9270,)
    } else {
        (var_loop_,)
    }
};
            var_loop_ = assign9340_body2_e9272;
            var_loop__rv = 0.0;
        }

        let (assign9350_e9280,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9350_e9278: f64 = (var_tmpa * var_invnf);
        (assign9350_e9278,)
    } else {
        (var_invsa,)
    }
};
        var_invsa = assign9350_e9280;
        var_invsa_rv = 0.0;

        let (assign9360_e9288,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9360_e9286: f64 = (var_tmpb * var_invnf);
        (assign9360_e9286,)
    } else {
        (var_invsb,)
    }
};
        var_invsb = assign9360_e9288;
        var_invsb_rv = 0.0;

        let (assign9370_e9300,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9370_e9296: f64 = (0.5 * var_l_i);
        let assign9370_e9297: f64 = (p.p791 + assign9370_e9296);
        let assign9370_e9298: f64 = (1.0 / assign9370_e9297);
        (assign9370_e9298,)
    } else {
        (var_invsaref,)
    }
};
        var_invsaref = assign9370_e9300;
        var_invsaref_rv = 0.0;

        let (assign9380_e9312,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9380_e9308: f64 = (0.5 * var_l_i);
        let assign9380_e9309: f64 = (p.p792 + assign9380_e9308);
        let assign9380_e9310: f64 = (1.0 / assign9380_e9309);
        (assign9380_e9310,)
    } else {
        (var_invsbref,)
    }
};
        var_invsbref = assign9380_e9312;
        var_invsbref_rv = 0.0;

        let (assign9390_e9327,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9390_e9318: f64 = (var_l_i + var_dellps);
        let (assign9390_e9325,) = {
            if (assign9390_e9318 > 1e-9) {
                let assign9390_e9323: f64 = (var_l_i + var_dellps);
                (assign9390_e9323,)
            } else {
                (1e-9,)
            }
        };
        (assign9390_e9325,)
    } else {
        (var_lx,)
    }
};
        var_lx = assign9390_e9327;
        var_lx_rv = 0.0;

        let (assign9400_e9346,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9400_e9333: f64 = (var_w_i + var_delwod);
        let assign9400_e9335: f64 = (assign9400_e9333 + p.p793);
        let (assign9400_e9344,) = {
            if (assign9400_e9335 > 1e-9) {
                let assign9400_e9340: f64 = (var_w_i + var_delwod);
                let assign9400_e9342: f64 = (assign9400_e9340 + p.p793);
                (assign9400_e9342,)
            } else {
                (1e-9,)
            }
        };
        (assign9400_e9344,)
    } else {
        (var_wx,)
    }
};
        var_wx = assign9400_e9346;
        var_wx_rv = 0.0;

        let (assign9410_e9356,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9410_e9353: f64 = (var_lx).powf(p.p801);
        let assign9410_e9354: f64 = (1.0 / assign9410_e9353);
        (assign9410_e9354,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign9410_e9356;
        var_templ_rv = 0.0;

        let (assign9420_e9366,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9420_e9363: f64 = (var_wx).powf(p.p802);
        let assign9420_e9364: f64 = (1.0 / assign9420_e9363);
        (assign9420_e9364,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign9420_e9366;
        var_tempw_rv = 0.0;

        let (assign9430_e9394,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9430_e9373: f64 = (p.p798 * var_templ);
        let assign9430_e9374: f64 = (1.0 + assign9430_e9373);
        let assign9430_e9377: f64 = (p.p799 * var_tempw);
        let assign9430_e9378: f64 = (assign9430_e9374 + assign9430_e9377);
        let assign9430_e9381: f64 = (p.p800 * var_templ);
        let assign9430_e9383: f64 = (assign9430_e9381 * var_tempw);
        let assign9430_e9384: f64 = (assign9430_e9378 + assign9430_e9383);
        let assign9430_e9389: f64 = (var_rta - 1.0);
        let assign9430_e9390: f64 = (p.p797 * assign9430_e9389);
        let assign9430_e9391: f64 = (1.0 + assign9430_e9390);
        let assign9430_e9392: f64 = (assign9430_e9384 * assign9430_e9391);
        (assign9430_e9392,)
    } else {
        (var_kstressu0,)
    }
};
        var_kstressu0 = assign9430_e9394;
        var_kstressu0_rv = 0.0;

        let (assign9440_e9406,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9440_e9401: f64 = (var_invsa + var_invsb);
        let assign9440_e9402: f64 = (p.p794 * assign9440_e9401);
        let assign9440_e9404: f64 = (assign9440_e9402 / var_kstressu0);
        (assign9440_e9404,)
    } else {
        (var_rhobeta,)
    }
};
        var_rhobeta = assign9440_e9406;
        var_rhobeta_rv = 0.0;

        let (assign9450_e9418,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9450_e9413: f64 = (var_invsaref + var_invsbref);
        let assign9450_e9414: f64 = (p.p794 * assign9450_e9413);
        let assign9450_e9416: f64 = (assign9450_e9414 / var_kstressu0);
        (assign9450_e9416,)
    } else {
        (var_rhobetaref,)
    }
};
        var_rhobetaref = assign9450_e9418;
        var_rhobetaref_rv = 0.0;

        let (assign9460_e9428,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9460_e9425: f64 = (var_lx).powf(p.p807);
        let assign9460_e9426: f64 = (1.0 / assign9460_e9425);
        (assign9460_e9426,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign9460_e9428;
        var_templ_rv = 0.0;

        let (assign9470_e9438,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9470_e9435: f64 = (var_wx).powf(p.p808);
        let assign9470_e9436: f64 = (1.0 / assign9470_e9435);
        (assign9470_e9436,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign9470_e9438;
        var_tempw_rv = 0.0;

        let (assign9480_e9458,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9480_e9445: f64 = (p.p804 * var_templ);
        let assign9480_e9446: f64 = (1.0 + assign9480_e9445);
        let assign9480_e9449: f64 = (p.p805 * var_tempw);
        let assign9480_e9450: f64 = (assign9480_e9446 + assign9480_e9449);
        let assign9480_e9453: f64 = (p.p806 * var_templ);
        let assign9480_e9455: f64 = (assign9480_e9453 * var_tempw);
        let assign9480_e9456: f64 = (assign9480_e9450 + assign9480_e9455);
        (assign9480_e9456,)
    } else {
        (var_kstressvth0,)
    }
};
        var_kstressvth0 = assign9480_e9458;
        var_kstressvth0_rv = 0.0;

        let (assign9490_e9470,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9490_e9464: f64 = (var_invsa + var_invsb);
        let assign9490_e9466: f64 = (assign9490_e9464 - var_invsaref);
        let assign9490_e9468: f64 = (assign9490_e9466 - var_invsbref);
        (assign9490_e9468,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9490_e9470;
        var_temp0_rv = 0.0;

        let (assign9500_e9482,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9500_e9476: f64 = (1.0 + var_rhobeta);
        let assign9500_e9479: f64 = (1.0 + var_rhobetaref);
        let assign9500_e9480: f64 = (assign9500_e9476 / assign9500_e9479);
        (assign9500_e9480,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9500_e9482;
        var_temp00_rv = 0.0;

        let (assign9510_e9490,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9510_e9488: f64 = (var_betn_p * var_temp00);
        (assign9510_e9488,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign9510_e9490;
        var_betn_p_rv = 0.0;

        let (assign9520_e9510,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9520_e9496: f64 = (var_thesat_p * var_temp00);
        let assign9520_e9500: f64 = (p.p795 * var_rhobetaref);
        let assign9520_e9501: f64 = (1.0 + assign9520_e9500);
        let assign9520_e9502: f64 = (assign9520_e9496 * assign9520_e9501);
        let assign9520_e9506: f64 = (p.p795 * var_rhobeta);
        let assign9520_e9507: f64 = (1.0 + assign9520_e9506);
        let assign9520_e9508: f64 = (assign9520_e9502 / assign9520_e9507);
        (assign9520_e9508,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign9520_e9510;
        var_thesat_p_rv = 0.0;

        let (assign9530_e9530,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9530_e9516: f64 = (var_thesatac_p * var_temp00);
        let assign9530_e9520: f64 = (var_kvsatac_i * var_rhobetaref);
        let assign9530_e9521: f64 = (1.0 + assign9530_e9520);
        let assign9530_e9522: f64 = (assign9530_e9516 * assign9530_e9521);
        let assign9530_e9526: f64 = (var_kvsatac_i * var_rhobeta);
        let assign9530_e9527: f64 = (1.0 + assign9530_e9526);
        let assign9530_e9528: f64 = (assign9530_e9522 / assign9530_e9527);
        (assign9530_e9528,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign9530_e9530;
        var_thesatac_p_rv = 0.0;

        let (assign9540_e9538,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9540_e9536: f64 = (var_betnedge_p * var_temp00);
        (assign9540_e9536,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign9540_e9538;
        var_betnedge_p_rv = 0.0;

        let (assign9550_e9548,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9550_e9544: f64 = (p.p803 * var_temp0);
        let assign9550_e9546: f64 = (assign9550_e9544 / var_kstressvth0);
        (assign9550_e9546,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9550_e9548;
        var_temp00_rv = 0.0;

        let (assign9560_e9556,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9560_e9554: f64 = (var_vfb_p + var_temp00);
        (assign9560_e9554,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign9560_e9556;
        var_vfb_p_rv = 0.0;

        let (assign9570_e9564,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9570_e9562: f64 = (var_vfbedge_p + var_temp00);
        (assign9570_e9562,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign9570_e9564;
        var_vfbedge_p_rv = 0.0;

        *var_betn_p_slot = var_betn_p;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfbedge_p_rv_slot = var_cfbedge_p_rv;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfdedge_p_rv_slot = var_cfdedge_p_rv;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfedge_p_rv_slot = var_cfedge_p_rv;
        *var_guard143_slot = var_guard143;
        *var_guard143_rv_slot = var_guard143_rv;
        *var_guard144_slot = var_guard144;
        *var_guard144_rv_slot = var_guard144_rv;
        *var_guard148_slot = var_guard148;
        *var_guard148_rv_slot = var_guard148_rv;
        *var_guard149_slot = var_guard149;
        *var_guard149_rv_slot = var_guard149_rv;
        *var_guard150_slot = var_guard150;
        *var_guard150_rv_slot = var_guard150_rv;
        *var_invsa_slot = var_invsa;
        *var_invsa_rv_slot = var_invsa_rv;
        *var_invsaref_slot = var_invsaref;
        *var_invsaref_rv_slot = var_invsaref_rv;
        *var_invsb_slot = var_invsb;
        *var_invsb_rv_slot = var_invsb_rv;
        *var_invsbref_slot = var_invsbref;
        *var_invsbref_rv_slot = var_invsbref_rv;
        *var_kstressu0_slot = var_kstressu0;
        *var_kstressu0_rv_slot = var_kstressu0_rv;
        *var_kstressvth0_slot = var_kstressvth0;
        *var_kstressvth0_rv_slot = var_kstressvth0_rv;
        *var_kvsatac_i_slot = var_kvsatac_i;
        *var_kvsatac_i_rv_slot = var_kvsatac_i_rv;
        *var_loop__slot = var_loop_;
        *var_loop__rv_slot = var_loop__rv;
        *var_lx_slot = var_lx;
        *var_lx_rv_slot = var_lx_rv;
        *var_munqs_p_slot = var_munqs_p;
        *var_munqs_p_rv_slot = var_munqs_p_rv;
        *var_rhobeta_slot = var_rhobeta;
        *var_rhobeta_rv_slot = var_rhobeta_rv;
        *var_rhobetaref_slot = var_rhobetaref;
        *var_rhobetaref_rv_slot = var_rhobetaref_rv;
        *var_temp0_slot = var_temp0;
        *var_temp00_slot = var_temp00;
        *var_temp00_rv_slot = var_temp00_rv;
        *var_temp0_rv_slot = var_temp0_rv;
        *var_templ_slot = var_templ;
        *var_templ_rv_slot = var_templ_rv;
        *var_tempw_slot = var_tempw;
        *var_tempw_rv_slot = var_tempw_rv;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_rv_slot = var_thesat_p_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
        *var_tmpa_slot = var_tmpa;
        *var_tmpa_rv_slot = var_tmpa_rv;
        *var_tmpb_slot = var_tmpb;
        *var_tmpb_rv_slot = var_tmpb_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
        *var_wx_slot = var_wx;
        *var_wx_rv_slot = var_wx_rv;
    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        var_cfb_p: f64,
        var_cfd_p: f64,
        var_cs_p: f64,
        var_ct_p: f64,
        var_ctb_p: f64,
        var_ctg_p: f64,
        var_dphib_p: f64,
        var_dvsbnud_p: f64,
        var_epsrox_p: f64,
        var_feta_p: f64,
        var_gfacnud_p: f64,
        var_guard150: f64,
        var_guard41: f64,
        var_kstressvth0: f64,
        var_kuowe: f64,
        var_kvthowe: f64,
        var_mue_p: f64,
        var_neff_p: f64,
        var_nov_p: f64,
        var_novd_p: f64,
        var_np_p: f64,
        var_psce_p: f64,
        var_psceb_p: f64,
        var_psced_p: f64,
        var_sc_i: f64,
        var_st2vfb_p: f64,
        var_stbet_p: f64,
        var_stcs_p: f64,
        var_stct_p: f64,
        var_stmue_p: f64,
        var_stthecs_p: f64,
        var_stthemu_p: f64,
        var_stvfb_p: f64,
        var_stxcor_p: f64,
        var_thecs_p: f64,
        var_themu_p: f64,
        var_tox_p: f64,
        var_toxov_p: f64,
        var_toxovd_p: f64,
        var_vsbnud_p: f64,
        var_w_i: f64,
        var_xcor_p: f64,
        var_betn_i_slot: &mut f64,
        var_betn_i_rv_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_cf_i_slot: &mut f64,
        var_cf_i_rv_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_rv_slot: &mut f64,
        var_cfb_i_slot: &mut f64,
        var_cfb_i_rv_slot: &mut f64,
        var_cfd_i_slot: &mut f64,
        var_cfd_i_rv_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfedge_p_rv_slot: &mut f64,
        var_cs_i_slot: &mut f64,
        var_cs_i_rv_slot: &mut f64,
        var_ct_i_slot: &mut f64,
        var_ct_i_rv_slot: &mut f64,
        var_ctb_i_slot: &mut f64,
        var_ctb_i_rv_slot: &mut f64,
        var_ctg_i_slot: &mut f64,
        var_ctg_i_rv_slot: &mut f64,
        var_dphib_i_slot: &mut f64,
        var_dphib_i_rv_slot: &mut f64,
        var_dvsbnud_i_slot: &mut f64,
        var_dvsbnud_i_rv_slot: &mut f64,
        var_epsrox_i_slot: &mut f64,
        var_epsrox_i_rv_slot: &mut f64,
        var_feta_i_slot: &mut f64,
        var_feta_i_rv_slot: &mut f64,
        var_gfacnud_i_slot: &mut f64,
        var_gfacnud_i_rv_slot: &mut f64,
        var_guard151_slot: &mut f64,
        var_guard151_rv_slot: &mut f64,
        var_guard152_slot: &mut f64,
        var_guard152_rv_slot: &mut f64,
        var_mue_i_slot: &mut f64,
        var_mue_i_rv_slot: &mut f64,
        var_neff_i_slot: &mut f64,
        var_neff_i_rv_slot: &mut f64,
        var_nov_i_slot: &mut f64,
        var_nov_i_rv_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_novd_i_rv_slot: &mut f64,
        var_np_i_slot: &mut f64,
        var_np_i_rv_slot: &mut f64,
        var_psce_i_slot: &mut f64,
        var_psce_i_rv_slot: &mut f64,
        var_psceb_i_slot: &mut f64,
        var_psceb_i_rv_slot: &mut f64,
        var_psced_i_slot: &mut f64,
        var_psced_i_rv_slot: &mut f64,
        var_sca_i_slot: &mut f64,
        var_sca_i_rv_slot: &mut f64,
        var_scb_i_slot: &mut f64,
        var_scb_i_rv_slot: &mut f64,
        var_scc_i_slot: &mut f64,
        var_scc_i_rv_slot: &mut f64,
        var_st2vfb_i_slot: &mut f64,
        var_st2vfb_i_rv_slot: &mut f64,
        var_stbet_i_slot: &mut f64,
        var_stbet_i_rv_slot: &mut f64,
        var_stcs_i_slot: &mut f64,
        var_stcs_i_rv_slot: &mut f64,
        var_stct_i_slot: &mut f64,
        var_stct_i_rv_slot: &mut f64,
        var_stmue_i_slot: &mut f64,
        var_stmue_i_rv_slot: &mut f64,
        var_stthecs_i_slot: &mut f64,
        var_stthecs_i_rv_slot: &mut f64,
        var_stthemu_i_slot: &mut f64,
        var_stthemu_i_rv_slot: &mut f64,
        var_stvfb_i_slot: &mut f64,
        var_stvfb_i_rv_slot: &mut f64,
        var_stxcor_i_slot: &mut f64,
        var_stxcor_i_rv_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_temp00_slot: &mut f64,
        var_temp00_rv_slot: &mut f64,
        var_temp0_rv_slot: &mut f64,
        var_thecs_i_slot: &mut f64,
        var_thecs_i_rv_slot: &mut f64,
        var_themu_i_slot: &mut f64,
        var_themu_i_rv_slot: &mut f64,
        var_tox_i_slot: &mut f64,
        var_tox_i_rv_slot: &mut f64,
        var_toxov_i_slot: &mut f64,
        var_toxov_i_rv_slot: &mut f64,
        var_toxovd_i_slot: &mut f64,
        var_toxovd_i_rv_slot: &mut f64,
        var_vfb_i_slot: &mut f64,
        var_vfb_i_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
        var_vsbnud_i_slot: &mut f64,
        var_vsbnud_i_rv_slot: &mut f64,
        var_xcor_i_slot: &mut f64,
        var_xcor_i_rv_slot: &mut f64,
    ) {
        let mut var_betn_i: f64 = *var_betn_i_slot;
        let mut var_betn_i_rv: f64 = *var_betn_i_rv_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_cf_i: f64 = *var_cf_i_slot;
        let mut var_cf_i_rv: f64 = *var_cf_i_rv_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_rv: f64 = *var_cf_p_rv_slot;
        let mut var_cfb_i: f64 = *var_cfb_i_slot;
        let mut var_cfb_i_rv: f64 = *var_cfb_i_rv_slot;
        let mut var_cfd_i: f64 = *var_cfd_i_slot;
        let mut var_cfd_i_rv: f64 = *var_cfd_i_rv_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfedge_p_rv: f64 = *var_cfedge_p_rv_slot;
        let mut var_cs_i: f64 = *var_cs_i_slot;
        let mut var_cs_i_rv: f64 = *var_cs_i_rv_slot;
        let mut var_ct_i: f64 = *var_ct_i_slot;
        let mut var_ct_i_rv: f64 = *var_ct_i_rv_slot;
        let mut var_ctb_i: f64 = *var_ctb_i_slot;
        let mut var_ctb_i_rv: f64 = *var_ctb_i_rv_slot;
        let mut var_ctg_i: f64 = *var_ctg_i_slot;
        let mut var_ctg_i_rv: f64 = *var_ctg_i_rv_slot;
        let mut var_dphib_i: f64 = *var_dphib_i_slot;
        let mut var_dphib_i_rv: f64 = *var_dphib_i_rv_slot;
        let mut var_dvsbnud_i: f64 = *var_dvsbnud_i_slot;
        let mut var_dvsbnud_i_rv: f64 = *var_dvsbnud_i_rv_slot;
        let mut var_epsrox_i: f64 = *var_epsrox_i_slot;
        let mut var_epsrox_i_rv: f64 = *var_epsrox_i_rv_slot;
        let mut var_feta_i: f64 = *var_feta_i_slot;
        let mut var_feta_i_rv: f64 = *var_feta_i_rv_slot;
        let mut var_gfacnud_i: f64 = *var_gfacnud_i_slot;
        let mut var_gfacnud_i_rv: f64 = *var_gfacnud_i_rv_slot;
        let mut var_guard151: f64 = *var_guard151_slot;
        let mut var_guard151_rv: f64 = *var_guard151_rv_slot;
        let mut var_guard152: f64 = *var_guard152_slot;
        let mut var_guard152_rv: f64 = *var_guard152_rv_slot;
        let mut var_mue_i: f64 = *var_mue_i_slot;
        let mut var_mue_i_rv: f64 = *var_mue_i_rv_slot;
        let mut var_neff_i: f64 = *var_neff_i_slot;
        let mut var_neff_i_rv: f64 = *var_neff_i_rv_slot;
        let mut var_nov_i: f64 = *var_nov_i_slot;
        let mut var_nov_i_rv: f64 = *var_nov_i_rv_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_novd_i_rv: f64 = *var_novd_i_rv_slot;
        let mut var_np_i: f64 = *var_np_i_slot;
        let mut var_np_i_rv: f64 = *var_np_i_rv_slot;
        let mut var_psce_i: f64 = *var_psce_i_slot;
        let mut var_psce_i_rv: f64 = *var_psce_i_rv_slot;
        let mut var_psceb_i: f64 = *var_psceb_i_slot;
        let mut var_psceb_i_rv: f64 = *var_psceb_i_rv_slot;
        let mut var_psced_i: f64 = *var_psced_i_slot;
        let mut var_psced_i_rv: f64 = *var_psced_i_rv_slot;
        let mut var_sca_i: f64 = *var_sca_i_slot;
        let mut var_sca_i_rv: f64 = *var_sca_i_rv_slot;
        let mut var_scb_i: f64 = *var_scb_i_slot;
        let mut var_scb_i_rv: f64 = *var_scb_i_rv_slot;
        let mut var_scc_i: f64 = *var_scc_i_slot;
        let mut var_scc_i_rv: f64 = *var_scc_i_rv_slot;
        let mut var_st2vfb_i: f64 = *var_st2vfb_i_slot;
        let mut var_st2vfb_i_rv: f64 = *var_st2vfb_i_rv_slot;
        let mut var_stbet_i: f64 = *var_stbet_i_slot;
        let mut var_stbet_i_rv: f64 = *var_stbet_i_rv_slot;
        let mut var_stcs_i: f64 = *var_stcs_i_slot;
        let mut var_stcs_i_rv: f64 = *var_stcs_i_rv_slot;
        let mut var_stct_i: f64 = *var_stct_i_slot;
        let mut var_stct_i_rv: f64 = *var_stct_i_rv_slot;
        let mut var_stmue_i: f64 = *var_stmue_i_slot;
        let mut var_stmue_i_rv: f64 = *var_stmue_i_rv_slot;
        let mut var_stthecs_i: f64 = *var_stthecs_i_slot;
        let mut var_stthecs_i_rv: f64 = *var_stthecs_i_rv_slot;
        let mut var_stthemu_i: f64 = *var_stthemu_i_slot;
        let mut var_stthemu_i_rv: f64 = *var_stthemu_i_rv_slot;
        let mut var_stvfb_i: f64 = *var_stvfb_i_slot;
        let mut var_stvfb_i_rv: f64 = *var_stvfb_i_rv_slot;
        let mut var_stxcor_i: f64 = *var_stxcor_i_slot;
        let mut var_stxcor_i_rv: f64 = *var_stxcor_i_rv_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_temp00: f64 = *var_temp00_slot;
        let mut var_temp00_rv: f64 = *var_temp00_rv_slot;
        let mut var_temp0_rv: f64 = *var_temp0_rv_slot;
        let mut var_thecs_i: f64 = *var_thecs_i_slot;
        let mut var_thecs_i_rv: f64 = *var_thecs_i_rv_slot;
        let mut var_themu_i: f64 = *var_themu_i_slot;
        let mut var_themu_i_rv: f64 = *var_themu_i_rv_slot;
        let mut var_tox_i: f64 = *var_tox_i_slot;
        let mut var_tox_i_rv: f64 = *var_tox_i_rv_slot;
        let mut var_toxov_i: f64 = *var_toxov_i_slot;
        let mut var_toxov_i_rv: f64 = *var_toxov_i_rv_slot;
        let mut var_toxovd_i: f64 = *var_toxovd_i_slot;
        let mut var_toxovd_i_rv: f64 = *var_toxovd_i_rv_slot;
        let mut var_vfb_i: f64 = *var_vfb_i_slot;
        let mut var_vfb_i_rv: f64 = *var_vfb_i_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;
        let mut var_vsbnud_i: f64 = *var_vsbnud_i_slot;
        let mut var_vsbnud_i_rv: f64 = *var_vsbnud_i_rv_slot;
        let mut var_xcor_i: f64 = *var_xcor_i_slot;
        let mut var_xcor_i_rv: f64 = *var_xcor_i_rv_slot;

        let (assign9580_e9576,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9580_e9570: f64 = (p.p809 * var_temp0);
        let assign9580_e9573: f64 = (var_kstressvth0).powf(p.p810);
        let assign9580_e9574: f64 = (assign9580_e9570 / assign9580_e9573);
        (assign9580_e9574,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9580_e9576;
        var_temp00_rv = 0.0;

        let (assign9590_e9584,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9590_e9582: f64 = (var_cf_p + var_temp00);
        (assign9590_e9582,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign9590_e9584;
        var_cf_p_rv = 0.0;

        let (assign9600_e9592,) = {
    if ((var_guard41 != 0.0) && (var_guard150 != 0.0)) {
        let assign9600_e9590: f64 = (var_cfedge_p + var_temp00);
        (assign9600_e9590,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign9600_e9592;
        var_cfedge_p_rv = 0.0;

        let assign9610_e9607: f64 = if ((((var_sca_i > 0.0) || (var_scb_i > 0.0)) || (var_scc_i > 0.0)) || (var_sc_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard151 = assign9610_e9607;
        var_guard151_rv = 0.0;

        let assign9620_e9618: f64 = if (((var_sca_i == 0.0) && (var_scb_i == 0.0)) && (var_scc_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard152 = assign9620_e9618;
        var_guard152_rv = 0.0;

        let (assign9630_e9628,) = {
    if (((var_guard41 != 0.0) && (var_guard151 != 0.0)) && (var_guard152 != 0.0)) {
        let assign9630_e9626: f64 = (var_sc_i + var_w_i);
        (assign9630_e9626,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9630_e9628;
        var_temp0_rv = 0.0;

        let (assign9640_e9638,) = {
    if (((var_guard41 != 0.0) && (var_guard151 != 0.0)) && (var_guard152 != 0.0)) {
        let assign9640_e9636: f64 = (1.0 / p.p811);
        (assign9640_e9636,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9640_e9638;
        var_temp00_rv = 0.0;

        let (assign9650_e9652,) = {
    if (((var_guard41 != 0.0) && (var_guard151 != 0.0)) && (var_guard152 != 0.0)) {
        let assign9650_e9646: f64 = (p.p811 * p.p811);
        let assign9650_e9649: f64 = (var_sc_i * var_temp0);
        let assign9650_e9650: f64 = (assign9650_e9646 / assign9650_e9649);
        (assign9650_e9650,)
    } else {
        (var_sca_i,)
    }
};
        var_sca_i = assign9650_e9652;
        var_sca_i_rv = 0.0;

        let (assign9660_e9692,) = {
    if (((var_guard41 != 0.0) && (var_guard151 != 0.0)) && (var_guard152 != 0.0)) {
        let assign9660_e9660: f64 = (0.1 * var_sc_i);
        let assign9660_e9663: f64 = (0.01 * p.p811);
        let assign9660_e9664: f64 = (assign9660_e9660 + assign9660_e9663);
        let assign9660_e9666: f64 = (-10.0);
        let assign9660_e9668: f64 = (assign9660_e9666 * var_sc_i);
        let assign9660_e9670: f64 = (assign9660_e9668 * var_temp00);
        let assign9660_e9671: f64 = (assign9660_e9670).exp();
        let assign9660_e9672: f64 = (assign9660_e9664 * assign9660_e9671);
        let assign9660_e9675: f64 = (0.1 * var_temp0);
        let assign9660_e9678: f64 = (0.01 * p.p811);
        let assign9660_e9679: f64 = (assign9660_e9675 + assign9660_e9678);
        let assign9660_e9681: f64 = (-10.0);
        let assign9660_e9683: f64 = (assign9660_e9681 * var_temp0);
        let assign9660_e9685: f64 = (assign9660_e9683 * var_temp00);
        let assign9660_e9686: f64 = (assign9660_e9685).exp();
        let assign9660_e9687: f64 = (assign9660_e9679 * assign9660_e9686);
        let assign9660_e9688: f64 = (assign9660_e9672 - assign9660_e9687);
        let assign9660_e9690: f64 = (assign9660_e9688 / var_w_i);
        (assign9660_e9690,)
    } else {
        (var_scb_i,)
    }
};
        var_scb_i = assign9660_e9692;
        var_scb_i_rv = 0.0;

        let (assign9670_e9732,) = {
    if (((var_guard41 != 0.0) && (var_guard151 != 0.0)) && (var_guard152 != 0.0)) {
        let assign9670_e9700: f64 = (0.05 * var_sc_i);
        let assign9670_e9703: f64 = (0.0025 * p.p811);
        let assign9670_e9704: f64 = (assign9670_e9700 + assign9670_e9703);
        let assign9670_e9706: f64 = (-20.0);
        let assign9670_e9708: f64 = (assign9670_e9706 * var_sc_i);
        let assign9670_e9710: f64 = (assign9670_e9708 * var_temp00);
        let assign9670_e9711: f64 = (assign9670_e9710).exp();
        let assign9670_e9712: f64 = (assign9670_e9704 * assign9670_e9711);
        let assign9670_e9715: f64 = (0.05 * var_temp0);
        let assign9670_e9718: f64 = (0.0025 * p.p811);
        let assign9670_e9719: f64 = (assign9670_e9715 + assign9670_e9718);
        let assign9670_e9721: f64 = (-20.0);
        let assign9670_e9723: f64 = (assign9670_e9721 * var_temp0);
        let assign9670_e9725: f64 = (assign9670_e9723 * var_temp00);
        let assign9670_e9726: f64 = (assign9670_e9725).exp();
        let assign9670_e9727: f64 = (assign9670_e9719 * assign9670_e9726);
        let assign9670_e9728: f64 = (assign9670_e9712 - assign9670_e9727);
        let assign9670_e9730: f64 = (assign9670_e9728 / var_w_i);
        (assign9670_e9730,)
    } else {
        (var_scc_i,)
    }
};
        var_scc_i = assign9670_e9732;
        var_scc_i_rv = 0.0;

        let (assign9680_e9746,) = {
    if ((var_guard41 != 0.0) && (var_guard151 != 0.0)) {
        let assign9680_e9739: f64 = (p.p812 * var_scb_i);
        let assign9680_e9740: f64 = (var_sca_i + assign9680_e9739);
        let assign9680_e9743: f64 = (p.p813 * var_scc_i);
        let assign9680_e9744: f64 = (assign9680_e9740 + assign9680_e9743);
        (assign9680_e9744,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9680_e9746;
        var_temp0_rv = 0.0;

        let (assign9690_e9756,) = {
    if ((var_guard41 != 0.0) && (var_guard151 != 0.0)) {
        let assign9690_e9753: f64 = (var_kvthowe * var_temp0);
        let assign9690_e9754: f64 = (var_vfb_p + assign9690_e9753);
        (assign9690_e9754,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign9690_e9756;
        var_vfb_p_rv = 0.0;

        let (assign9700_e9768,) = {
    if ((var_guard41 != 0.0) && (var_guard151 != 0.0)) {
        let assign9700_e9764: f64 = (var_kuowe * var_temp0);
        let assign9700_e9765: f64 = (1.0 + assign9700_e9764);
        let assign9700_e9766: f64 = (var_betn_p * assign9700_e9765);
        (assign9700_e9766,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign9700_e9768;
        var_betn_p_rv = 0.0;

        let (assign9710_e9778,) = {
    if ((var_guard41 != 0.0) && (var_guard151 != 0.0)) {
        let assign9710_e9775: f64 = (var_kvthowe * var_temp0);
        let assign9710_e9776: f64 = (var_vfbedge_p + assign9710_e9775);
        (assign9710_e9776,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign9710_e9778;
        var_vfbedge_p_rv = 0.0;

        let (assign9720_e9790,) = {
    if ((var_guard41 != 0.0) && (var_guard151 != 0.0)) {
        let assign9720_e9786: f64 = (var_kuowe * var_temp0);
        let assign9720_e9787: f64 = (1.0 + assign9720_e9786);
        let assign9720_e9788: f64 = (var_betnedge_p * assign9720_e9787);
        (assign9720_e9788,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign9720_e9790;
        var_betnedge_p_rv = 0.0;

        var_vfb_i = var_vfb_p;
        var_vfb_i_rv = 0.0;

        var_stvfb_i = var_stvfb_p;
        var_stvfb_i_rv = 0.0;

        var_st2vfb_i = var_st2vfb_p;
        var_st2vfb_i_rv = 0.0;

        var_tox_i = var_tox_p;
        var_tox_i_rv = 0.0;

        var_epsrox_i = var_epsrox_p;
        var_epsrox_i_rv = 0.0;

        let (assign9780_e9806,) = {
    if (var_neff_p > 1e20) {
        let (assign9780_e9804,) = {
            if (var_neff_p < 1e26) {
                (var_neff_p,)
            } else {
                (1e26,)
            }
        };
        (assign9780_e9804,)
    } else {
        (1e20,)
    }
};
        var_neff_i = assign9780_e9806;
        var_neff_i_rv = 0.0;

        let (assign9790_e9812,) = {
    if (var_gfacnud_p > 0.01) {
        (var_gfacnud_p,)
    } else {
        (0.01,)
    }
};
        var_gfacnud_i = assign9790_e9812;
        var_gfacnud_i_rv = 0.0;

        let (assign9800_e9818,) = {
    if (var_vsbnud_p > 0.0) {
        (var_vsbnud_p,)
    } else {
        (0.0,)
    }
};
        var_vsbnud_i = assign9800_e9818;
        var_vsbnud_i_rv = 0.0;

        var_dvsbnud_i = var_dvsbnud_p;
        var_dvsbnud_i_rv = 0.0;

        var_dphib_i = var_dphib_p;
        var_dphib_i_rv = 0.0;

        let (assign9830_e9826,) = {
    if (var_np_p > 0.0) {
        (var_np_p,)
    } else {
        (0.0,)
    }
};
        var_np_i = assign9830_e9826;
        var_np_i_rv = 0.0;

        var_toxov_i = var_toxov_p;
        var_toxov_i_rv = 0.0;

        var_toxovd_i = var_toxovd_p;
        var_toxovd_i_rv = 0.0;

        let (assign9860_e9839,) = {
    if (var_nov_p > 1e23) {
        let (assign9860_e9837,) = {
            if (var_nov_p < 1e27) {
                (var_nov_p,)
            } else {
                (1e27,)
            }
        };
        (assign9860_e9837,)
    } else {
        (1e23,)
    }
};
        var_nov_i = assign9860_e9839;
        var_nov_i_rv = 0.0;

        let (assign9870_e9850,) = {
    if (var_novd_p > 1e23) {
        let (assign9870_e9848,) = {
            if (var_novd_p < 1e27) {
                (var_novd_p,)
            } else {
                (1e27,)
            }
        };
        (assign9870_e9848,)
    } else {
        (1e23,)
    }
};
        var_novd_i = assign9870_e9850;
        var_novd_i_rv = 0.0;

        let (assign9880_e9856,) = {
    if (var_ct_p > 0.0) {
        (var_ct_p,)
    } else {
        (0.0,)
    }
};
        var_ct_i = assign9880_e9856;
        var_ct_i_rv = 0.0;

        let (assign9890_e9867,) = {
    if (var_ctb_p > 0.0) {
        let (assign9890_e9865,) = {
            if (var_ctb_p < 0.5) {
                (var_ctb_p,)
            } else {
                (0.5,)
            }
        };
        (assign9890_e9865,)
    } else {
        (0.0,)
    }
};
        var_ctb_i = assign9890_e9867;
        var_ctb_i_rv = 0.0;

        let (assign9900_e9878,) = {
    if (var_ctg_p > 0.0) {
        let (assign9900_e9876,) = {
            if (var_ctg_p < 1.0) {
                (var_ctg_p,)
            } else {
                (1.0,)
            }
        };
        (assign9900_e9876,)
    } else {
        (0.0,)
    }
};
        var_ctg_i = assign9900_e9878;
        var_ctg_i_rv = 0.0;

        var_stct_i = var_stct_p;
        var_stct_i_rv = 0.0;

        let (assign9920_e9885,) = {
    if (var_cf_p > 0.0) {
        (var_cf_p,)
    } else {
        (0.0,)
    }
};
        var_cf_i = assign9920_e9885;
        var_cf_i_rv = 0.0;

        let (assign9930_e9896,) = {
    if (var_cfb_p > 0.0) {
        let (assign9930_e9894,) = {
            if (var_cfb_p < 1.0) {
                (var_cfb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9930_e9894,)
    } else {
        (0.0,)
    }
};
        var_cfb_i = assign9930_e9896;
        var_cfb_i_rv = 0.0;

        let (assign9940_e9902,) = {
    if (var_cfd_p > 0.0) {
        (var_cfd_p,)
    } else {
        (0.0,)
    }
};
        var_cfd_i = assign9940_e9902;
        var_cfd_i_rv = 0.0;

        let (assign9950_e9908,) = {
    if (var_psce_p > 0.0) {
        (var_psce_p,)
    } else {
        (0.0,)
    }
};
        var_psce_i = assign9950_e9908;
        var_psce_i_rv = 0.0;

        let (assign9960_e9919,) = {
    if (var_psceb_p > 0.0) {
        let (assign9960_e9917,) = {
            if (var_psceb_p < 1.0) {
                (var_psceb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9960_e9917,)
    } else {
        (0.0,)
    }
};
        var_psceb_i = assign9960_e9919;
        var_psceb_i_rv = 0.0;

        let (assign9970_e9925,) = {
    if (var_psced_p > 0.0) {
        (var_psced_p,)
    } else {
        (0.0,)
    }
};
        var_psced_i = assign9970_e9925;
        var_psced_i_rv = 0.0;

        let (assign9980_e9931,) = {
    if (var_betn_p > 0.0) {
        (var_betn_p,)
    } else {
        (0.0,)
    }
};
        var_betn_i = assign9980_e9931;
        var_betn_i_rv = 0.0;

        var_stbet_i = var_stbet_p;
        var_stbet_i_rv = 0.0;

        let (assign10000_e9938,) = {
    if (var_mue_p > 0.0) {
        (var_mue_p,)
    } else {
        (0.0,)
    }
};
        var_mue_i = assign10000_e9938;
        var_mue_i_rv = 0.0;

        var_stmue_i = var_stmue_p;
        var_stmue_i_rv = 0.0;

        let (assign10020_e9945,) = {
    if (var_themu_p > 0.0) {
        (var_themu_p,)
    } else {
        (0.0,)
    }
};
        var_themu_i = assign10020_e9945;
        var_themu_i_rv = 0.0;

        var_stthemu_i = var_stthemu_p;
        var_stthemu_i_rv = 0.0;

        let (assign10040_e9952,) = {
    if (var_cs_p > 0.0) {
        (var_cs_p,)
    } else {
        (0.0,)
    }
};
        var_cs_i = assign10040_e9952;
        var_cs_i_rv = 0.0;

        var_stcs_i = var_stcs_p;
        var_stcs_i_rv = 0.0;

        let (assign10060_e9959,) = {
    if (var_thecs_p > 0.0) {
        (var_thecs_p,)
    } else {
        (0.0,)
    }
};
        var_thecs_i = assign10060_e9959;
        var_thecs_i_rv = 0.0;

        var_stthecs_i = var_stthecs_p;
        var_stthecs_i_rv = 0.0;

        let (assign10080_e9966,) = {
    if (var_xcor_p > 0.0) {
        (var_xcor_p,)
    } else {
        (0.0,)
    }
};
        var_xcor_i = assign10080_e9966;
        var_xcor_i_rv = 0.0;

        var_stxcor_i = var_stxcor_p;
        var_stxcor_i_rv = 0.0;

        var_feta_i = var_feta_p;
        var_feta_i_rv = 0.0;

        *var_betn_i_slot = var_betn_i;
        *var_betn_i_rv_slot = var_betn_i_rv;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_cf_i_slot = var_cf_i;
        *var_cf_i_rv_slot = var_cf_i_rv;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_rv_slot = var_cf_p_rv;
        *var_cfb_i_slot = var_cfb_i;
        *var_cfb_i_rv_slot = var_cfb_i_rv;
        *var_cfd_i_slot = var_cfd_i;
        *var_cfd_i_rv_slot = var_cfd_i_rv;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfedge_p_rv_slot = var_cfedge_p_rv;
        *var_cs_i_slot = var_cs_i;
        *var_cs_i_rv_slot = var_cs_i_rv;
        *var_ct_i_slot = var_ct_i;
        *var_ct_i_rv_slot = var_ct_i_rv;
        *var_ctb_i_slot = var_ctb_i;
        *var_ctb_i_rv_slot = var_ctb_i_rv;
        *var_ctg_i_slot = var_ctg_i;
        *var_ctg_i_rv_slot = var_ctg_i_rv;
        *var_dphib_i_slot = var_dphib_i;
        *var_dphib_i_rv_slot = var_dphib_i_rv;
        *var_dvsbnud_i_slot = var_dvsbnud_i;
        *var_dvsbnud_i_rv_slot = var_dvsbnud_i_rv;
        *var_epsrox_i_slot = var_epsrox_i;
        *var_epsrox_i_rv_slot = var_epsrox_i_rv;
        *var_feta_i_slot = var_feta_i;
        *var_feta_i_rv_slot = var_feta_i_rv;
        *var_gfacnud_i_slot = var_gfacnud_i;
        *var_gfacnud_i_rv_slot = var_gfacnud_i_rv;
        *var_guard151_slot = var_guard151;
        *var_guard151_rv_slot = var_guard151_rv;
        *var_guard152_slot = var_guard152;
        *var_guard152_rv_slot = var_guard152_rv;
        *var_mue_i_slot = var_mue_i;
        *var_mue_i_rv_slot = var_mue_i_rv;
        *var_neff_i_slot = var_neff_i;
        *var_neff_i_rv_slot = var_neff_i_rv;
        *var_nov_i_slot = var_nov_i;
        *var_nov_i_rv_slot = var_nov_i_rv;
        *var_novd_i_slot = var_novd_i;
        *var_novd_i_rv_slot = var_novd_i_rv;
        *var_np_i_slot = var_np_i;
        *var_np_i_rv_slot = var_np_i_rv;
        *var_psce_i_slot = var_psce_i;
        *var_psce_i_rv_slot = var_psce_i_rv;
        *var_psceb_i_slot = var_psceb_i;
        *var_psceb_i_rv_slot = var_psceb_i_rv;
        *var_psced_i_slot = var_psced_i;
        *var_psced_i_rv_slot = var_psced_i_rv;
        *var_sca_i_slot = var_sca_i;
        *var_sca_i_rv_slot = var_sca_i_rv;
        *var_scb_i_slot = var_scb_i;
        *var_scb_i_rv_slot = var_scb_i_rv;
        *var_scc_i_slot = var_scc_i;
        *var_scc_i_rv_slot = var_scc_i_rv;
        *var_st2vfb_i_slot = var_st2vfb_i;
        *var_st2vfb_i_rv_slot = var_st2vfb_i_rv;
        *var_stbet_i_slot = var_stbet_i;
        *var_stbet_i_rv_slot = var_stbet_i_rv;
        *var_stcs_i_slot = var_stcs_i;
        *var_stcs_i_rv_slot = var_stcs_i_rv;
        *var_stct_i_slot = var_stct_i;
        *var_stct_i_rv_slot = var_stct_i_rv;
        *var_stmue_i_slot = var_stmue_i;
        *var_stmue_i_rv_slot = var_stmue_i_rv;
        *var_stthecs_i_slot = var_stthecs_i;
        *var_stthecs_i_rv_slot = var_stthecs_i_rv;
        *var_stthemu_i_slot = var_stthemu_i;
        *var_stthemu_i_rv_slot = var_stthemu_i_rv;
        *var_stvfb_i_slot = var_stvfb_i;
        *var_stvfb_i_rv_slot = var_stvfb_i_rv;
        *var_stxcor_i_slot = var_stxcor_i;
        *var_stxcor_i_rv_slot = var_stxcor_i_rv;
        *var_temp0_slot = var_temp0;
        *var_temp00_slot = var_temp00;
        *var_temp00_rv_slot = var_temp00_rv;
        *var_temp0_rv_slot = var_temp0_rv;
        *var_thecs_i_slot = var_thecs_i;
        *var_thecs_i_rv_slot = var_thecs_i_rv;
        *var_themu_i_slot = var_themu_i;
        *var_themu_i_rv_slot = var_themu_i_rv;
        *var_tox_i_slot = var_tox_i;
        *var_tox_i_rv_slot = var_tox_i_rv;
        *var_toxov_i_slot = var_toxov_i;
        *var_toxov_i_rv_slot = var_toxov_i_rv;
        *var_toxovd_i_slot = var_toxovd_i;
        *var_toxovd_i_rv_slot = var_toxovd_i_rv;
        *var_vfb_i_slot = var_vfb_i;
        *var_vfb_i_rv_slot = var_vfb_i_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
        *var_vsbnud_i_slot = var_vsbnud_i;
        *var_vsbnud_i_rv_slot = var_vsbnud_i_rv;
        *var_xcor_i_slot = var_xcor_i;
        *var_xcor_i_rv_slot = var_xcor_i_rv;
    }

    pub(super) fn stamp_reactive_block_16(
        var_a1_p: f64,
        var_a2_p: f64,
        var_a3_p: f64,
        var_a4_p: f64,
        var_agidl_p: f64,
        var_agidld_p: f64,
        var_alp1_p: f64,
        var_alp1ac_p: f64,
        var_alp2_p: f64,
        var_alp_p: f64,
        var_alpac_p: f64,
        var_ax_p: f64,
        var_axac_p: f64,
        var_axinr_p: f64,
        var_betnedge_p: f64,
        var_bgidl_p: f64,
        var_bgidld_p: f64,
        var_cfr_p: f64,
        var_cfrd_p: f64,
        var_cgbov_p: f64,
        var_cgidl_p: f64,
        var_cgidld_p: f64,
        var_cgov_p: f64,
        var_cgovaccg_p: f64,
        var_cgovd_p: f64,
        var_chib_p: f64,
        var_cinr_p: f64,
        var_cinrd_p: f64,
        var_cox_p: f64,
        var_ctedge_p: f64,
        var_delvtac_p: f64,
        var_dphibedge_p: f64,
        var_dvfbinr_p: f64,
        var_facneffac_p: f64,
        var_fcgovacc_p: f64,
        var_fcgovaccd_p: f64,
        var_fcinracc_p: f64,
        var_fcinrdep_p: f64,
        var_fnt_p: f64,
        var_gc2_p: f64,
        var_gc2ov_p: f64,
        var_gc2ovd_p: f64,
        var_gc3_p: f64,
        var_gc3ov_p: f64,
        var_gc3ovd_p: f64,
        var_gco_p: f64,
        var_iginv_p: f64,
        var_igov_p: f64,
        var_igovd_p: f64,
        var_imaxii_p: f64,
        var_neffedge_p: f64,
        var_pscebedge_p: f64,
        var_psceedge_p: f64,
        var_rs_p: f64,
        var_rsb_p: f64,
        var_rsg_p: f64,
        var_sta2_p: f64,
        var_stbetedge_p: f64,
        var_stbgidl_p: f64,
        var_stbgidld_p: f64,
        var_stig_p: f64,
        var_strs_p: f64,
        var_stthesat_p: f64,
        var_stvfbedge_p: f64,
        var_thesat_p: f64,
        var_thesatac_p: f64,
        var_thesatb_p: f64,
        var_thesatg_p: f64,
        var_thesatt_p: f64,
        var_vfbedge_p: f64,
        var_vp_p: f64,
        var_a1_i_slot: &mut f64,
        var_a1_i_rv_slot: &mut f64,
        var_a2_i_slot: &mut f64,
        var_a2_i_rv_slot: &mut f64,
        var_a3_i_slot: &mut f64,
        var_a3_i_rv_slot: &mut f64,
        var_a4_i_slot: &mut f64,
        var_a4_i_rv_slot: &mut f64,
        var_agidl_i_slot: &mut f64,
        var_agidl_i_rv_slot: &mut f64,
        var_agidld_i_slot: &mut f64,
        var_agidld_i_rv_slot: &mut f64,
        var_alp1_i_slot: &mut f64,
        var_alp1_i_rv_slot: &mut f64,
        var_alp1ac_i_slot: &mut f64,
        var_alp1ac_i_rv_slot: &mut f64,
        var_alp2_i_slot: &mut f64,
        var_alp2_i_rv_slot: &mut f64,
        var_alp_i_slot: &mut f64,
        var_alp_i_rv_slot: &mut f64,
        var_alpac_i_slot: &mut f64,
        var_alpac_i_rv_slot: &mut f64,
        var_ax_i_slot: &mut f64,
        var_ax_i_rv_slot: &mut f64,
        var_axac_i_slot: &mut f64,
        var_axac_i_rv_slot: &mut f64,
        var_axinr_i_slot: &mut f64,
        var_axinr_i_rv_slot: &mut f64,
        var_betnedge_i_slot: &mut f64,
        var_betnedge_i_rv_slot: &mut f64,
        var_bgidl_i_slot: &mut f64,
        var_bgidl_i_rv_slot: &mut f64,
        var_bgidld_i_slot: &mut f64,
        var_bgidld_i_rv_slot: &mut f64,
        var_cfr_i_slot: &mut f64,
        var_cfr_i_rv_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cfrd_i_rv_slot: &mut f64,
        var_cgbov_i_slot: &mut f64,
        var_cgbov_i_rv_slot: &mut f64,
        var_cgidl_i_slot: &mut f64,
        var_cgidl_i_rv_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_cgidld_i_rv_slot: &mut f64,
        var_cgov_i_slot: &mut f64,
        var_cgov_i_rv_slot: &mut f64,
        var_cgovaccg_i_slot: &mut f64,
        var_cgovaccg_i_rv_slot: &mut f64,
        var_cgovd_i_slot: &mut f64,
        var_cgovd_i_rv_slot: &mut f64,
        var_chib_i_slot: &mut f64,
        var_chib_i_rv_slot: &mut f64,
        var_cinr_i_slot: &mut f64,
        var_cinr_i_rv_slot: &mut f64,
        var_cinrd_i_slot: &mut f64,
        var_cinrd_i_rv_slot: &mut f64,
        var_cox_i_slot: &mut f64,
        var_cox_i_rv_slot: &mut f64,
        var_ctedge_i_slot: &mut f64,
        var_ctedge_i_rv_slot: &mut f64,
        var_delvtac_i_slot: &mut f64,
        var_delvtac_i_rv_slot: &mut f64,
        var_dphibedge_i_slot: &mut f64,
        var_dphibedge_i_rv_slot: &mut f64,
        var_dvfbinr_i_slot: &mut f64,
        var_dvfbinr_i_rv_slot: &mut f64,
        var_facneffac_i_slot: &mut f64,
        var_facneffac_i_rv_slot: &mut f64,
        var_fcgovacc_i_slot: &mut f64,
        var_fcgovacc_i_rv_slot: &mut f64,
        var_fcgovaccd_i_slot: &mut f64,
        var_fcgovaccd_i_rv_slot: &mut f64,
        var_fcinracc_i_slot: &mut f64,
        var_fcinracc_i_rv_slot: &mut f64,
        var_fcinrdep_i_slot: &mut f64,
        var_fcinrdep_i_rv_slot: &mut f64,
        var_fnt_i_slot: &mut f64,
        var_fnt_i_rv_slot: &mut f64,
        var_gc2_i_slot: &mut f64,
        var_gc2_i_rv_slot: &mut f64,
        var_gc2ov_i_slot: &mut f64,
        var_gc2ov_i_rv_slot: &mut f64,
        var_gc2ovd_i_slot: &mut f64,
        var_gc2ovd_i_rv_slot: &mut f64,
        var_gc3_i_slot: &mut f64,
        var_gc3_i_rv_slot: &mut f64,
        var_gc3ov_i_slot: &mut f64,
        var_gc3ov_i_rv_slot: &mut f64,
        var_gc3ovd_i_slot: &mut f64,
        var_gc3ovd_i_rv_slot: &mut f64,
        var_gco_i_slot: &mut f64,
        var_gco_i_rv_slot: &mut f64,
        var_iginv_i_slot: &mut f64,
        var_iginv_i_rv_slot: &mut f64,
        var_igov_i_slot: &mut f64,
        var_igov_i_rv_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_igovd_i_rv_slot: &mut f64,
        var_imaxii_i_slot: &mut f64,
        var_imaxii_i_rv_slot: &mut f64,
        var_neffedge_i_slot: &mut f64,
        var_neffedge_i_rv_slot: &mut f64,
        var_pscebedge_i_slot: &mut f64,
        var_pscebedge_i_rv_slot: &mut f64,
        var_psceedge_i_slot: &mut f64,
        var_psceedge_i_rv_slot: &mut f64,
        var_rs_i_slot: &mut f64,
        var_rs_i_rv_slot: &mut f64,
        var_rsb_i_slot: &mut f64,
        var_rsb_i_rv_slot: &mut f64,
        var_rsg_i_slot: &mut f64,
        var_rsg_i_rv_slot: &mut f64,
        var_sta2_i_slot: &mut f64,
        var_sta2_i_rv_slot: &mut f64,
        var_stbetedge_i_slot: &mut f64,
        var_stbetedge_i_rv_slot: &mut f64,
        var_stbgidl_i_slot: &mut f64,
        var_stbgidl_i_rv_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_stbgidld_i_rv_slot: &mut f64,
        var_stig_i_slot: &mut f64,
        var_stig_i_rv_slot: &mut f64,
        var_strs_i_slot: &mut f64,
        var_strs_i_rv_slot: &mut f64,
        var_stthesat_i_slot: &mut f64,
        var_stthesat_i_rv_slot: &mut f64,
        var_stvfbedge_i_slot: &mut f64,
        var_stvfbedge_i_rv_slot: &mut f64,
        var_thesat_i_slot: &mut f64,
        var_thesat_i_rv_slot: &mut f64,
        var_thesatac_i_slot: &mut f64,
        var_thesatac_i_rv_slot: &mut f64,
        var_thesatb_i_slot: &mut f64,
        var_thesatb_i_rv_slot: &mut f64,
        var_thesatg_i_slot: &mut f64,
        var_thesatg_i_rv_slot: &mut f64,
        var_thesatt_i_slot: &mut f64,
        var_thesatt_i_rv_slot: &mut f64,
        var_vfbedge_i_slot: &mut f64,
        var_vfbedge_i_rv_slot: &mut f64,
        var_vp_i_slot: &mut f64,
        var_vp_i_rv_slot: &mut f64,
    ) {
        let mut var_a1_i: f64 = *var_a1_i_slot;
        let mut var_a1_i_rv: f64 = *var_a1_i_rv_slot;
        let mut var_a2_i: f64 = *var_a2_i_slot;
        let mut var_a2_i_rv: f64 = *var_a2_i_rv_slot;
        let mut var_a3_i: f64 = *var_a3_i_slot;
        let mut var_a3_i_rv: f64 = *var_a3_i_rv_slot;
        let mut var_a4_i: f64 = *var_a4_i_slot;
        let mut var_a4_i_rv: f64 = *var_a4_i_rv_slot;
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agidl_i_rv: f64 = *var_agidl_i_rv_slot;
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_agidld_i_rv: f64 = *var_agidld_i_rv_slot;
        let mut var_alp1_i: f64 = *var_alp1_i_slot;
        let mut var_alp1_i_rv: f64 = *var_alp1_i_rv_slot;
        let mut var_alp1ac_i: f64 = *var_alp1ac_i_slot;
        let mut var_alp1ac_i_rv: f64 = *var_alp1ac_i_rv_slot;
        let mut var_alp2_i: f64 = *var_alp2_i_slot;
        let mut var_alp2_i_rv: f64 = *var_alp2_i_rv_slot;
        let mut var_alp_i: f64 = *var_alp_i_slot;
        let mut var_alp_i_rv: f64 = *var_alp_i_rv_slot;
        let mut var_alpac_i: f64 = *var_alpac_i_slot;
        let mut var_alpac_i_rv: f64 = *var_alpac_i_rv_slot;
        let mut var_ax_i: f64 = *var_ax_i_slot;
        let mut var_ax_i_rv: f64 = *var_ax_i_rv_slot;
        let mut var_axac_i: f64 = *var_axac_i_slot;
        let mut var_axac_i_rv: f64 = *var_axac_i_rv_slot;
        let mut var_axinr_i: f64 = *var_axinr_i_slot;
        let mut var_axinr_i_rv: f64 = *var_axinr_i_rv_slot;
        let mut var_betnedge_i: f64 = *var_betnedge_i_slot;
        let mut var_betnedge_i_rv: f64 = *var_betnedge_i_rv_slot;
        let mut var_bgidl_i: f64 = *var_bgidl_i_slot;
        let mut var_bgidl_i_rv: f64 = *var_bgidl_i_rv_slot;
        let mut var_bgidld_i: f64 = *var_bgidld_i_slot;
        let mut var_bgidld_i_rv: f64 = *var_bgidld_i_rv_slot;
        let mut var_cfr_i: f64 = *var_cfr_i_slot;
        let mut var_cfr_i_rv: f64 = *var_cfr_i_rv_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cfrd_i_rv: f64 = *var_cfrd_i_rv_slot;
        let mut var_cgbov_i: f64 = *var_cgbov_i_slot;
        let mut var_cgbov_i_rv: f64 = *var_cgbov_i_rv_slot;
        let mut var_cgidl_i: f64 = *var_cgidl_i_slot;
        let mut var_cgidl_i_rv: f64 = *var_cgidl_i_rv_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_cgidld_i_rv: f64 = *var_cgidld_i_rv_slot;
        let mut var_cgov_i: f64 = *var_cgov_i_slot;
        let mut var_cgov_i_rv: f64 = *var_cgov_i_rv_slot;
        let mut var_cgovaccg_i: f64 = *var_cgovaccg_i_slot;
        let mut var_cgovaccg_i_rv: f64 = *var_cgovaccg_i_rv_slot;
        let mut var_cgovd_i: f64 = *var_cgovd_i_slot;
        let mut var_cgovd_i_rv: f64 = *var_cgovd_i_rv_slot;
        let mut var_chib_i: f64 = *var_chib_i_slot;
        let mut var_chib_i_rv: f64 = *var_chib_i_rv_slot;
        let mut var_cinr_i: f64 = *var_cinr_i_slot;
        let mut var_cinr_i_rv: f64 = *var_cinr_i_rv_slot;
        let mut var_cinrd_i: f64 = *var_cinrd_i_slot;
        let mut var_cinrd_i_rv: f64 = *var_cinrd_i_rv_slot;
        let mut var_cox_i: f64 = *var_cox_i_slot;
        let mut var_cox_i_rv: f64 = *var_cox_i_rv_slot;
        let mut var_ctedge_i: f64 = *var_ctedge_i_slot;
        let mut var_ctedge_i_rv: f64 = *var_ctedge_i_rv_slot;
        let mut var_delvtac_i: f64 = *var_delvtac_i_slot;
        let mut var_delvtac_i_rv: f64 = *var_delvtac_i_rv_slot;
        let mut var_dphibedge_i: f64 = *var_dphibedge_i_slot;
        let mut var_dphibedge_i_rv: f64 = *var_dphibedge_i_rv_slot;
        let mut var_dvfbinr_i: f64 = *var_dvfbinr_i_slot;
        let mut var_dvfbinr_i_rv: f64 = *var_dvfbinr_i_rv_slot;
        let mut var_facneffac_i: f64 = *var_facneffac_i_slot;
        let mut var_facneffac_i_rv: f64 = *var_facneffac_i_rv_slot;
        let mut var_fcgovacc_i: f64 = *var_fcgovacc_i_slot;
        let mut var_fcgovacc_i_rv: f64 = *var_fcgovacc_i_rv_slot;
        let mut var_fcgovaccd_i: f64 = *var_fcgovaccd_i_slot;
        let mut var_fcgovaccd_i_rv: f64 = *var_fcgovaccd_i_rv_slot;
        let mut var_fcinracc_i: f64 = *var_fcinracc_i_slot;
        let mut var_fcinracc_i_rv: f64 = *var_fcinracc_i_rv_slot;
        let mut var_fcinrdep_i: f64 = *var_fcinrdep_i_slot;
        let mut var_fcinrdep_i_rv: f64 = *var_fcinrdep_i_rv_slot;
        let mut var_fnt_i: f64 = *var_fnt_i_slot;
        let mut var_fnt_i_rv: f64 = *var_fnt_i_rv_slot;
        let mut var_gc2_i: f64 = *var_gc2_i_slot;
        let mut var_gc2_i_rv: f64 = *var_gc2_i_rv_slot;
        let mut var_gc2ov_i: f64 = *var_gc2ov_i_slot;
        let mut var_gc2ov_i_rv: f64 = *var_gc2ov_i_rv_slot;
        let mut var_gc2ovd_i: f64 = *var_gc2ovd_i_slot;
        let mut var_gc2ovd_i_rv: f64 = *var_gc2ovd_i_rv_slot;
        let mut var_gc3_i: f64 = *var_gc3_i_slot;
        let mut var_gc3_i_rv: f64 = *var_gc3_i_rv_slot;
        let mut var_gc3ov_i: f64 = *var_gc3ov_i_slot;
        let mut var_gc3ov_i_rv: f64 = *var_gc3ov_i_rv_slot;
        let mut var_gc3ovd_i: f64 = *var_gc3ovd_i_slot;
        let mut var_gc3ovd_i_rv: f64 = *var_gc3ovd_i_rv_slot;
        let mut var_gco_i: f64 = *var_gco_i_slot;
        let mut var_gco_i_rv: f64 = *var_gco_i_rv_slot;
        let mut var_iginv_i: f64 = *var_iginv_i_slot;
        let mut var_iginv_i_rv: f64 = *var_iginv_i_rv_slot;
        let mut var_igov_i: f64 = *var_igov_i_slot;
        let mut var_igov_i_rv: f64 = *var_igov_i_rv_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_igovd_i_rv: f64 = *var_igovd_i_rv_slot;
        let mut var_imaxii_i: f64 = *var_imaxii_i_slot;
        let mut var_imaxii_i_rv: f64 = *var_imaxii_i_rv_slot;
        let mut var_neffedge_i: f64 = *var_neffedge_i_slot;
        let mut var_neffedge_i_rv: f64 = *var_neffedge_i_rv_slot;
        let mut var_pscebedge_i: f64 = *var_pscebedge_i_slot;
        let mut var_pscebedge_i_rv: f64 = *var_pscebedge_i_rv_slot;
        let mut var_psceedge_i: f64 = *var_psceedge_i_slot;
        let mut var_psceedge_i_rv: f64 = *var_psceedge_i_rv_slot;
        let mut var_rs_i: f64 = *var_rs_i_slot;
        let mut var_rs_i_rv: f64 = *var_rs_i_rv_slot;
        let mut var_rsb_i: f64 = *var_rsb_i_slot;
        let mut var_rsb_i_rv: f64 = *var_rsb_i_rv_slot;
        let mut var_rsg_i: f64 = *var_rsg_i_slot;
        let mut var_rsg_i_rv: f64 = *var_rsg_i_rv_slot;
        let mut var_sta2_i: f64 = *var_sta2_i_slot;
        let mut var_sta2_i_rv: f64 = *var_sta2_i_rv_slot;
        let mut var_stbetedge_i: f64 = *var_stbetedge_i_slot;
        let mut var_stbetedge_i_rv: f64 = *var_stbetedge_i_rv_slot;
        let mut var_stbgidl_i: f64 = *var_stbgidl_i_slot;
        let mut var_stbgidl_i_rv: f64 = *var_stbgidl_i_rv_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_stbgidld_i_rv: f64 = *var_stbgidld_i_rv_slot;
        let mut var_stig_i: f64 = *var_stig_i_slot;
        let mut var_stig_i_rv: f64 = *var_stig_i_rv_slot;
        let mut var_strs_i: f64 = *var_strs_i_slot;
        let mut var_strs_i_rv: f64 = *var_strs_i_rv_slot;
        let mut var_stthesat_i: f64 = *var_stthesat_i_slot;
        let mut var_stthesat_i_rv: f64 = *var_stthesat_i_rv_slot;
        let mut var_stvfbedge_i: f64 = *var_stvfbedge_i_slot;
        let mut var_stvfbedge_i_rv: f64 = *var_stvfbedge_i_rv_slot;
        let mut var_thesat_i: f64 = *var_thesat_i_slot;
        let mut var_thesat_i_rv: f64 = *var_thesat_i_rv_slot;
        let mut var_thesatac_i: f64 = *var_thesatac_i_slot;
        let mut var_thesatac_i_rv: f64 = *var_thesatac_i_rv_slot;
        let mut var_thesatb_i: f64 = *var_thesatb_i_slot;
        let mut var_thesatb_i_rv: f64 = *var_thesatb_i_rv_slot;
        let mut var_thesatg_i: f64 = *var_thesatg_i_slot;
        let mut var_thesatg_i_rv: f64 = *var_thesatg_i_rv_slot;
        let mut var_thesatt_i: f64 = *var_thesatt_i_slot;
        let mut var_thesatt_i_rv: f64 = *var_thesatt_i_rv_slot;
        let mut var_vfbedge_i: f64 = *var_vfbedge_i_slot;
        let mut var_vfbedge_i_rv: f64 = *var_vfbedge_i_rv_slot;
        let mut var_vp_i: f64 = *var_vp_i_slot;
        let mut var_vp_i_rv: f64 = *var_vp_i_rv_slot;

        let (assign10110_e9974,) = {
    if (var_rs_p > 0.0) {
        (var_rs_p,)
    } else {
        (0.0,)
    }
};
        var_rs_i = assign10110_e9974;
        var_rs_i_rv = 0.0;

        var_strs_i = var_strs_p;
        var_strs_i_rv = 0.0;

        let assign10130_e9978: f64 = (-0.5);
        let (assign10130_e9988,) = {
    if (var_rsb_p > assign10130_e9978) {
        let (assign10130_e9985,) = {
            if (var_rsb_p < 1.0) {
                (var_rsb_p,)
            } else {
                (1.0,)
            }
        };
        (assign10130_e9985,)
    } else {
        let assign10130_e9987: f64 = (-0.5);
        (assign10130_e9987,)
    }
};
        var_rsb_i = assign10130_e9988;
        var_rsb_i_rv = 0.0;

        let assign10140_e9991: f64 = (-0.5);
        let (assign10140_e9996,) = {
    if (var_rsg_p > assign10140_e9991) {
        (var_rsg_p,)
    } else {
        let assign10140_e9995: f64 = (-0.5);
        (assign10140_e9995,)
    }
};
        var_rsg_i = assign10140_e9996;
        var_rsg_i_rv = 0.0;

        let (assign10150_e10002,) = {
    if (var_thesat_p > 0.0) {
        (var_thesat_p,)
    } else {
        (0.0,)
    }
};
        var_thesat_i = assign10150_e10002;
        var_thesat_i_rv = 0.0;

        var_stthesat_i = var_stthesat_p;
        var_stthesat_i_rv = 0.0;

        let assign10170_e10006: f64 = (-0.5);
        let (assign10170_e10016,) = {
    if (var_thesatb_p > assign10170_e10006) {
        let (assign10170_e10013,) = {
            if (var_thesatb_p < 1.0) {
                (var_thesatb_p,)
            } else {
                (1.0,)
            }
        };
        (assign10170_e10013,)
    } else {
        let assign10170_e10015: f64 = (-0.5);
        (assign10170_e10015,)
    }
};
        var_thesatb_i = assign10170_e10016;
        var_thesatb_i_rv = 0.0;

        let assign10180_e10019: f64 = (-0.5);
        let (assign10180_e10024,) = {
    if (var_thesatg_p > assign10180_e10019) {
        (var_thesatg_p,)
    } else {
        let assign10180_e10023: f64 = (-0.5);
        (assign10180_e10023,)
    }
};
        var_thesatg_i = assign10180_e10024;
        var_thesatg_i_rv = 0.0;

        let (assign10190_e10030,) = {
    if (var_thesatt_p > 0.01) {
        (var_thesatt_p,)
    } else {
        (0.01,)
    }
};
        var_thesatt_i = assign10190_e10030;
        var_thesatt_i_rv = 0.0;

        let (assign10200_e10036,) = {
    if (var_ax_p > 2.0) {
        (var_ax_p,)
    } else {
        (2.0,)
    }
};
        var_ax_i = assign10200_e10036;
        var_ax_i_rv = 0.0;

        let (assign10210_e10042,) = {
    if (var_alp_p > 0.0) {
        (var_alp_p,)
    } else {
        (0.0,)
    }
};
        var_alp_i = assign10210_e10042;
        var_alp_i_rv = 0.0;

        let (assign10220_e10048,) = {
    if (var_alp1_p > 0.0) {
        (var_alp1_p,)
    } else {
        (0.0,)
    }
};
        var_alp1_i = assign10220_e10048;
        var_alp1_i_rv = 0.0;

        let (assign10230_e10054,) = {
    if (var_alp2_p > 0.0) {
        (var_alp2_p,)
    } else {
        (0.0,)
    }
};
        var_alp2_i = assign10230_e10054;
        var_alp2_i_rv = 0.0;

        var_vp_i = var_vp_p;
        var_vp_i_rv = 0.0;

        let (assign10250_e10061,) = {
    if (var_a1_p > 0.0) {
        (var_a1_p,)
    } else {
        (0.0,)
    }
};
        var_a1_i = assign10250_e10061;
        var_a1_i_rv = 0.0;

        var_a2_i = var_a2_p;
        var_a2_i_rv = 0.0;

        var_sta2_i = var_sta2_p;
        var_sta2_i_rv = 0.0;

        let (assign10280_e10069,) = {
    if (var_a3_p > 0.0) {
        (var_a3_p,)
    } else {
        (0.0,)
    }
};
        var_a3_i = assign10280_e10069;
        var_a3_i_rv = 0.0;

        let (assign10290_e10075,) = {
    if (var_a4_p > 0.0) {
        (var_a4_p,)
    } else {
        (0.0,)
    }
};
        var_a4_i = assign10290_e10075;
        var_a4_i_rv = 0.0;

        let (assign10300_e10081,) = {
    if (var_imaxii_p > 1e-12) {
        (var_imaxii_p,)
    } else {
        (1e-12,)
    }
};
        var_imaxii_i = assign10300_e10081;
        var_imaxii_i_rv = 0.0;

        var_gco_i = var_gco_p;
        var_gco_i_rv = 0.0;

        let (assign10320_e10088,) = {
    if (var_iginv_p > 0.0) {
        (var_iginv_p,)
    } else {
        (0.0,)
    }
};
        var_iginv_i = assign10320_e10088;
        var_iginv_i_rv = 0.0;

        let (assign10330_e10094,) = {
    if (var_igov_p > 0.0) {
        (var_igov_p,)
    } else {
        (0.0,)
    }
};
        var_igov_i = assign10330_e10094;
        var_igov_i_rv = 0.0;

        let (assign10340_e10100,) = {
    if (var_igovd_p > 0.0) {
        (var_igovd_p,)
    } else {
        (0.0,)
    }
};
        var_igovd_i = assign10340_e10100;
        var_igovd_i_rv = 0.0;

        var_stig_i = var_stig_p;
        var_stig_i_rv = 0.0;

        var_gc2_i = var_gc2_p;
        var_gc2_i_rv = 0.0;

        var_gc3_i = var_gc3_p;
        var_gc3_i_rv = 0.0;

        var_gc2ov_i = var_gc2ov_p;
        var_gc2ov_i_rv = 0.0;

        var_gc3ov_i = var_gc3ov_p;
        var_gc3ov_i_rv = 0.0;

        var_gc2ovd_i = var_gc2ovd_p;
        var_gc2ovd_i_rv = 0.0;

        var_gc3ovd_i = var_gc3ovd_p;
        var_gc3ovd_i_rv = 0.0;

        var_chib_i = var_chib_p;
        var_chib_i_rv = 0.0;

        let (assign10430_e10114,) = {
    if (var_agidl_p > 0.0) {
        (var_agidl_p,)
    } else {
        (0.0,)
    }
};
        var_agidl_i = assign10430_e10114;
        var_agidl_i_rv = 0.0;

        let (assign10440_e10120,) = {
    if (var_agidld_p > 0.0) {
        (var_agidld_p,)
    } else {
        (0.0,)
    }
};
        var_agidld_i = assign10440_e10120;
        var_agidld_i_rv = 0.0;

        var_bgidl_i = var_bgidl_p;
        var_bgidl_i_rv = 0.0;

        var_bgidld_i = var_bgidld_p;
        var_bgidld_i_rv = 0.0;

        var_stbgidl_i = var_stbgidl_p;
        var_stbgidl_i_rv = 0.0;

        var_stbgidld_i = var_stbgidld_p;
        var_stbgidld_i_rv = 0.0;

        var_cgidl_i = var_cgidl_p;
        var_cgidl_i_rv = 0.0;

        var_cgidld_i = var_cgidld_p;
        var_cgidld_i_rv = 0.0;

        let (assign10510_e10132,) = {
    if (var_cox_p > 0.0) {
        (var_cox_p,)
    } else {
        (0.0,)
    }
};
        var_cox_i = assign10510_e10132;
        var_cox_i_rv = 0.0;

        var_delvtac_i = var_delvtac_p;
        var_delvtac_i_rv = 0.0;

        let (assign10530_e10139,) = {
    if (var_facneffac_p > 0.0) {
        (var_facneffac_p,)
    } else {
        (0.0,)
    }
};
        var_facneffac_i = assign10530_e10139;
        var_facneffac_i_rv = 0.0;

        let (assign10540_e10145,) = {
    if (var_thesatac_p > 0.0) {
        (var_thesatac_p,)
    } else {
        (0.0,)
    }
};
        var_thesatac_i = assign10540_e10145;
        var_thesatac_i_rv = 0.0;

        let (assign10550_e10151,) = {
    if (var_axac_p > 2.0) {
        (var_axac_p,)
    } else {
        (2.0,)
    }
};
        var_axac_i = assign10550_e10151;
        var_axac_i_rv = 0.0;

        var_alpac_i = var_alpac_p;
        var_alpac_i_rv = 0.0;

        let (assign10570_e10158,) = {
    if (var_alp1ac_p > 0.0) {
        (var_alp1ac_p,)
    } else {
        (0.0,)
    }
};
        var_alp1ac_i = assign10570_e10158;
        var_alp1ac_i_rv = 0.0;

        let (assign10580_e10164,) = {
    if (var_cgov_p > 0.0) {
        (var_cgov_p,)
    } else {
        (0.0,)
    }
};
        var_cgov_i = assign10580_e10164;
        var_cgov_i_rv = 0.0;

        let (assign10590_e10170,) = {
    if (var_cgovd_p > 0.0) {
        (var_cgovd_p,)
    } else {
        (0.0,)
    }
};
        var_cgovd_i = assign10590_e10170;
        var_cgovd_i_rv = 0.0;

        var_fcgovacc_i = var_fcgovacc_p;
        var_fcgovacc_i_rv = 0.0;

        var_fcgovaccd_i = var_fcgovaccd_p;
        var_fcgovaccd_i_rv = 0.0;

        var_cgovaccg_i = var_cgovaccg_p;
        var_cgovaccg_i_rv = 0.0;

        let (assign10630_e10179,) = {
    if (var_cgbov_p > 0.0) {
        (var_cgbov_p,)
    } else {
        (0.0,)
    }
};
        var_cgbov_i = assign10630_e10179;
        var_cgbov_i_rv = 0.0;

        let (assign10640_e10185,) = {
    if (var_cinr_p > 0.0) {
        (var_cinr_p,)
    } else {
        (0.0,)
    }
};
        var_cinr_i = assign10640_e10185;
        var_cinr_i_rv = 0.0;

        let (assign10650_e10191,) = {
    if (var_cinrd_p > 0.0) {
        (var_cinrd_p,)
    } else {
        (0.0,)
    }
};
        var_cinrd_i = assign10650_e10191;
        var_cinrd_i_rv = 0.0;

        var_dvfbinr_i = var_dvfbinr_p;
        var_dvfbinr_i_rv = 0.0;

        var_fcinrdep_i = var_fcinrdep_p;
        var_fcinrdep_i_rv = 0.0;

        var_fcinracc_i = var_fcinracc_p;
        var_fcinracc_i_rv = 0.0;

        var_axinr_i = var_axinr_p;
        var_axinr_i_rv = 0.0;

        let (assign10700_e10201,) = {
    if (var_cfr_p > 0.0) {
        (var_cfr_p,)
    } else {
        (0.0,)
    }
};
        var_cfr_i = assign10700_e10201;
        var_cfr_i_rv = 0.0;

        let (assign10710_e10207,) = {
    if (var_cfrd_p > 0.0) {
        (var_cfrd_p,)
    } else {
        (0.0,)
    }
};
        var_cfrd_i = assign10710_e10207;
        var_cfrd_i_rv = 0.0;

        var_fnt_i = var_fnt_p;
        var_fnt_i_rv = 0.0;

        var_vfbedge_i = var_vfbedge_p;
        var_vfbedge_i_rv = 0.0;

        var_stvfbedge_i = var_stvfbedge_p;
        var_stvfbedge_i_rv = 0.0;

        var_dphibedge_i = var_dphibedge_p;
        var_dphibedge_i_rv = 0.0;

        let (assign10810_e10247,) = {
    if (var_neffedge_p > 1e20) {
        let (assign10810_e10245,) = {
            if (var_neffedge_p < 1e26) {
                (var_neffedge_p,)
            } else {
                (1e26,)
            }
        };
        (assign10810_e10245,)
    } else {
        (1e20,)
    }
};
        var_neffedge_i = assign10810_e10247;
        var_neffedge_i_rv = 0.0;

        let (assign10820_e10253,) = {
    if (var_ctedge_p > 0.0) {
        (var_ctedge_p,)
    } else {
        (0.0,)
    }
};
        var_ctedge_i = assign10820_e10253;
        var_ctedge_i_rv = 0.0;

        let (assign10830_e10259,) = {
    if (var_betnedge_p > 0.0) {
        (var_betnedge_p,)
    } else {
        (0.0,)
    }
};
        var_betnedge_i = assign10830_e10259;
        var_betnedge_i_rv = 0.0;

        var_stbetedge_i = var_stbetedge_p;
        var_stbetedge_i_rv = 0.0;

        let (assign10850_e10266,) = {
    if (var_psceedge_p > 0.0) {
        (var_psceedge_p,)
    } else {
        (0.0,)
    }
};
        var_psceedge_i = assign10850_e10266;
        var_psceedge_i_rv = 0.0;

        let (assign10860_e10277,) = {
    if (var_pscebedge_p > 0.0) {
        let (assign10860_e10275,) = {
            if (var_pscebedge_p < 1.0) {
                (var_pscebedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10860_e10275,)
    } else {
        (0.0,)
    }
};
        var_pscebedge_i = assign10860_e10277;
        var_pscebedge_i_rv = 0.0;

        *var_a1_i_slot = var_a1_i;
        *var_a1_i_rv_slot = var_a1_i_rv;
        *var_a2_i_slot = var_a2_i;
        *var_a2_i_rv_slot = var_a2_i_rv;
        *var_a3_i_slot = var_a3_i;
        *var_a3_i_rv_slot = var_a3_i_rv;
        *var_a4_i_slot = var_a4_i;
        *var_a4_i_rv_slot = var_a4_i_rv;
        *var_agidl_i_slot = var_agidl_i;
        *var_agidl_i_rv_slot = var_agidl_i_rv;
        *var_agidld_i_slot = var_agidld_i;
        *var_agidld_i_rv_slot = var_agidld_i_rv;
        *var_alp1_i_slot = var_alp1_i;
        *var_alp1_i_rv_slot = var_alp1_i_rv;
        *var_alp1ac_i_slot = var_alp1ac_i;
        *var_alp1ac_i_rv_slot = var_alp1ac_i_rv;
        *var_alp2_i_slot = var_alp2_i;
        *var_alp2_i_rv_slot = var_alp2_i_rv;
        *var_alp_i_slot = var_alp_i;
        *var_alp_i_rv_slot = var_alp_i_rv;
        *var_alpac_i_slot = var_alpac_i;
        *var_alpac_i_rv_slot = var_alpac_i_rv;
        *var_ax_i_slot = var_ax_i;
        *var_ax_i_rv_slot = var_ax_i_rv;
        *var_axac_i_slot = var_axac_i;
        *var_axac_i_rv_slot = var_axac_i_rv;
        *var_axinr_i_slot = var_axinr_i;
        *var_axinr_i_rv_slot = var_axinr_i_rv;
        *var_betnedge_i_slot = var_betnedge_i;
        *var_betnedge_i_rv_slot = var_betnedge_i_rv;
        *var_bgidl_i_slot = var_bgidl_i;
        *var_bgidl_i_rv_slot = var_bgidl_i_rv;
        *var_bgidld_i_slot = var_bgidld_i;
        *var_bgidld_i_rv_slot = var_bgidld_i_rv;
        *var_cfr_i_slot = var_cfr_i;
        *var_cfr_i_rv_slot = var_cfr_i_rv;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cfrd_i_rv_slot = var_cfrd_i_rv;
        *var_cgbov_i_slot = var_cgbov_i;
        *var_cgbov_i_rv_slot = var_cgbov_i_rv;
        *var_cgidl_i_slot = var_cgidl_i;
        *var_cgidl_i_rv_slot = var_cgidl_i_rv;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_cgidld_i_rv_slot = var_cgidld_i_rv;
        *var_cgov_i_slot = var_cgov_i;
        *var_cgov_i_rv_slot = var_cgov_i_rv;
        *var_cgovaccg_i_slot = var_cgovaccg_i;
        *var_cgovaccg_i_rv_slot = var_cgovaccg_i_rv;
        *var_cgovd_i_slot = var_cgovd_i;
        *var_cgovd_i_rv_slot = var_cgovd_i_rv;
        *var_chib_i_slot = var_chib_i;
        *var_chib_i_rv_slot = var_chib_i_rv;
        *var_cinr_i_slot = var_cinr_i;
        *var_cinr_i_rv_slot = var_cinr_i_rv;
        *var_cinrd_i_slot = var_cinrd_i;
        *var_cinrd_i_rv_slot = var_cinrd_i_rv;
        *var_cox_i_slot = var_cox_i;
        *var_cox_i_rv_slot = var_cox_i_rv;
        *var_ctedge_i_slot = var_ctedge_i;
        *var_ctedge_i_rv_slot = var_ctedge_i_rv;
        *var_delvtac_i_slot = var_delvtac_i;
        *var_delvtac_i_rv_slot = var_delvtac_i_rv;
        *var_dphibedge_i_slot = var_dphibedge_i;
        *var_dphibedge_i_rv_slot = var_dphibedge_i_rv;
        *var_dvfbinr_i_slot = var_dvfbinr_i;
        *var_dvfbinr_i_rv_slot = var_dvfbinr_i_rv;
        *var_facneffac_i_slot = var_facneffac_i;
        *var_facneffac_i_rv_slot = var_facneffac_i_rv;
        *var_fcgovacc_i_slot = var_fcgovacc_i;
        *var_fcgovacc_i_rv_slot = var_fcgovacc_i_rv;
        *var_fcgovaccd_i_slot = var_fcgovaccd_i;
        *var_fcgovaccd_i_rv_slot = var_fcgovaccd_i_rv;
        *var_fcinracc_i_slot = var_fcinracc_i;
        *var_fcinracc_i_rv_slot = var_fcinracc_i_rv;
        *var_fcinrdep_i_slot = var_fcinrdep_i;
        *var_fcinrdep_i_rv_slot = var_fcinrdep_i_rv;
        *var_fnt_i_slot = var_fnt_i;
        *var_fnt_i_rv_slot = var_fnt_i_rv;
        *var_gc2_i_slot = var_gc2_i;
        *var_gc2_i_rv_slot = var_gc2_i_rv;
        *var_gc2ov_i_slot = var_gc2ov_i;
        *var_gc2ov_i_rv_slot = var_gc2ov_i_rv;
        *var_gc2ovd_i_slot = var_gc2ovd_i;
        *var_gc2ovd_i_rv_slot = var_gc2ovd_i_rv;
        *var_gc3_i_slot = var_gc3_i;
        *var_gc3_i_rv_slot = var_gc3_i_rv;
        *var_gc3ov_i_slot = var_gc3ov_i;
        *var_gc3ov_i_rv_slot = var_gc3ov_i_rv;
        *var_gc3ovd_i_slot = var_gc3ovd_i;
        *var_gc3ovd_i_rv_slot = var_gc3ovd_i_rv;
        *var_gco_i_slot = var_gco_i;
        *var_gco_i_rv_slot = var_gco_i_rv;
        *var_iginv_i_slot = var_iginv_i;
        *var_iginv_i_rv_slot = var_iginv_i_rv;
        *var_igov_i_slot = var_igov_i;
        *var_igov_i_rv_slot = var_igov_i_rv;
        *var_igovd_i_slot = var_igovd_i;
        *var_igovd_i_rv_slot = var_igovd_i_rv;
        *var_imaxii_i_slot = var_imaxii_i;
        *var_imaxii_i_rv_slot = var_imaxii_i_rv;
        *var_neffedge_i_slot = var_neffedge_i;
        *var_neffedge_i_rv_slot = var_neffedge_i_rv;
        *var_pscebedge_i_slot = var_pscebedge_i;
        *var_pscebedge_i_rv_slot = var_pscebedge_i_rv;
        *var_psceedge_i_slot = var_psceedge_i;
        *var_psceedge_i_rv_slot = var_psceedge_i_rv;
        *var_rs_i_slot = var_rs_i;
        *var_rs_i_rv_slot = var_rs_i_rv;
        *var_rsb_i_slot = var_rsb_i;
        *var_rsb_i_rv_slot = var_rsb_i_rv;
        *var_rsg_i_slot = var_rsg_i;
        *var_rsg_i_rv_slot = var_rsg_i_rv;
        *var_sta2_i_slot = var_sta2_i;
        *var_sta2_i_rv_slot = var_sta2_i_rv;
        *var_stbetedge_i_slot = var_stbetedge_i;
        *var_stbetedge_i_rv_slot = var_stbetedge_i_rv;
        *var_stbgidl_i_slot = var_stbgidl_i;
        *var_stbgidl_i_rv_slot = var_stbgidl_i_rv;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_stbgidld_i_rv_slot = var_stbgidld_i_rv;
        *var_stig_i_slot = var_stig_i;
        *var_stig_i_rv_slot = var_stig_i_rv;
        *var_strs_i_slot = var_strs_i;
        *var_strs_i_rv_slot = var_strs_i_rv;
        *var_stthesat_i_slot = var_stthesat_i;
        *var_stthesat_i_rv_slot = var_stthesat_i_rv;
        *var_stvfbedge_i_slot = var_stvfbedge_i;
        *var_stvfbedge_i_rv_slot = var_stvfbedge_i_rv;
        *var_thesat_i_slot = var_thesat_i;
        *var_thesat_i_rv_slot = var_thesat_i_rv;
        *var_thesatac_i_slot = var_thesatac_i;
        *var_thesatac_i_rv_slot = var_thesatac_i_rv;
        *var_thesatb_i_slot = var_thesatb_i;
        *var_thesatb_i_rv_slot = var_thesatb_i_rv;
        *var_thesatg_i_slot = var_thesatg_i;
        *var_thesatg_i_rv_slot = var_thesatg_i_rv;
        *var_thesatt_i_slot = var_thesatt_i;
        *var_thesatt_i_rv_slot = var_thesatt_i_rv;
        *var_vfbedge_i_slot = var_vfbedge_i;
        *var_vfbedge_i_rv_slot = var_vfbedge_i_rv;
        *var_vp_i_slot = var_vp_i;
        *var_vp_i_rv_slot = var_vp_i_rv;
    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        var_agidl_i: f64,
        var_ax_i: f64,
        var_axac_i: f64,
        var_bgidl_i: f64,
        var_cfbedge_p: f64,
        var_cfdedge_p: f64,
        var_cfedge_p: f64,
        var_cfr_i: f64,
        var_cgidl_i: f64,
        var_cgov_i: f64,
        var_cgovaccg_i: f64,
        var_chnl_type: f64,
        var_cinr_i: f64,
        var_epsrox_i: f64,
        var_epssi: f64,
        var_facneffac_i: f64,
        var_fcgovacc_i: f64,
        var_feta_i: f64,
        var_gc2ov_i: f64,
        var_gc3ov_i: f64,
        var_igov_i: f64,
        var_inv_phita: f64,
        var_munqs_p: f64,
        var_neff_i: f64,
        var_nf_i: f64,
        var_nov_i: f64,
        var_pscededge_p: f64,
        var_stbgidl_i: f64,
        var_tox_i: f64,
        var_toxov_i: f64,
        var_vp_i: f64,
        var_agidld_i_slot: &mut f64,
        var_agidld_i_rv_slot: &mut f64,
        var_ar_slot: &mut f64,
        var_ar_rv_slot: &mut f64,
        var_arac_slot: &mut f64,
        var_arac_rv_slot: &mut f64,
        var_bgidld_i_slot: &mut f64,
        var_bgidld_i_rv_slot: &mut f64,
        var_cfbedge_i_slot: &mut f64,
        var_cfbedge_i_rv_slot: &mut f64,
        var_cfdedge_i_slot: &mut f64,
        var_cfdedge_i_rv_slot: &mut f64,
        var_cfedge_i_slot: &mut f64,
        var_cfedge_i_rv_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cfrd_i_rv_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_cgidld_i_rv_slot: &mut f64,
        var_cgovd_i_slot: &mut f64,
        var_cgovd_i_rv_slot: &mut f64,
        var_cinrd_i_slot: &mut f64,
        var_cinrd_i_rv_slot: &mut f64,
        var_cox_over_q_slot: &mut f64,
        var_cox_over_q_rv_slot: &mut f64,
        var_coxovprime_slot: &mut f64,
        var_coxovprime_d_slot: &mut f64,
        var_coxovprime_d_rv_slot: &mut f64,
        var_coxovprime_rv_slot: &mut f64,
        var_coxprime_slot: &mut f64,
        var_coxprime_rv_slot: &mut f64,
        var_delvto_i_slot: &mut f64,
        var_delvto_i_rv_slot: &mut f64,
        var_delvtoedge_i_slot: &mut f64,
        var_delvtoedge_i_rv_slot: &mut f64,
        var_dxgb_ov_d_slot: &mut f64,
        var_dxgb_ov_d_rv_slot: &mut f64,
        var_dxgb_ov_s_slot: &mut f64,
        var_dxgb_ov_s_rv_slot: &mut f64,
        var_dxgb_ov_th_slot: &mut f64,
        var_dxgb_ov_th_rv_slot: &mut f64,
        var_e_eff0_slot: &mut f64,
        var_e_eff0_rv_slot: &mut f64,
        var_epsox_slot: &mut f64,
        var_epsox_rv_slot: &mut f64,
        var_eta_mu_slot: &mut f64,
        var_eta_mu1_slot: &mut f64,
        var_eta_mu1_rv_slot: &mut f64,
        var_eta_mu_rv_slot: &mut f64,
        var_factuo_i_slot: &mut f64,
        var_factuo_i_rv_slot: &mut f64,
        var_factuoedge_i_slot: &mut f64,
        var_factuoedge_i_rv_slot: &mut f64,
        var_fcgovaccd_i_slot: &mut f64,
        var_fcgovaccd_i_rv_slot: &mut f64,
        var_gc2ovd_i_slot: &mut f64,
        var_gc2ovd_i_rv_slot: &mut f64,
        var_gc3ovd_i_slot: &mut f64,
        var_gc3ovd_i_rv_slot: &mut f64,
        var_gov2_d_slot: &mut f64,
        var_gov2_d_rv_slot: &mut f64,
        var_gov2_s_slot: &mut f64,
        var_gov2_s_rv_slot: &mut f64,
        var_gov_d_slot: &mut f64,
        var_gov_d_rv_slot: &mut f64,
        var_gov_s_slot: &mut f64,
        var_gov_s_rv_slot: &mut f64,
        var_guard153_slot: &mut f64,
        var_guard153_rv_slot: &mut f64,
        var_guard154_slot: &mut f64,
        var_guard154_rv_slot: &mut f64,
        var_guard155_slot: &mut f64,
        var_guard155_rv_slot: &mut f64,
        var_guard156_slot: &mut f64,
        var_guard156_rv_slot: &mut f64,
        var_guard157_slot: &mut f64,
        var_guard157_rv_slot: &mut f64,
        var_guard158_slot: &mut f64,
        var_guard158_rv_slot: &mut f64,
        var_guard159_slot: &mut f64,
        var_guard159_rv_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_igovd_i_rv_slot: &mut f64,
        var_inv_gov_slot: &mut f64,
        var_inv_gov_rv_slot: &mut f64,
        var_inv_vp_slot: &mut f64,
        var_inv_vp_rv_slot: &mut f64,
        var_mult_inst_slot: &mut f64,
        var_mult_inst_rv_slot: &mut f64,
        var_munqs_i_slot: &mut f64,
        var_munqs_i_rv_slot: &mut f64,
        var_neffac_i_slot: &mut f64,
        var_neffac_i_rv_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_novd_i_rv_slot: &mut f64,
        var_pscededge_i_slot: &mut f64,
        var_pscededge_i_rv_slot: &mut f64,
        var_qq_slot: &mut f64,
        var_qq_rv_slot: &mut f64,
        var_sp_ov_a_s_slot: &mut f64,
        var_sp_ov_a_s_rv_slot: &mut f64,
        var_sp_ov_delta_slot: &mut f64,
        var_sp_ov_delta_rv_slot: &mut f64,
        var_sp_ov_eps_slot: &mut f64,
        var_sp_ov_eps2_s_slot: &mut f64,
        var_sp_ov_eps2_s_rv_slot: &mut f64,
        var_sp_ov_eps_rv_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_stbgidld_i_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
        var_tox_sq_slot: &mut f64,
        var_tox_sq_rv_slot: &mut f64,
        var_toxovd_i_slot: &mut f64,
        var_toxovd_i_rv_slot: &mut f64,
    ) {
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_agidld_i_rv: f64 = *var_agidld_i_rv_slot;
        let mut var_ar: f64 = *var_ar_slot;
        let mut var_ar_rv: f64 = *var_ar_rv_slot;
        let mut var_arac: f64 = *var_arac_slot;
        let mut var_arac_rv: f64 = *var_arac_rv_slot;
        let mut var_bgidld_i: f64 = *var_bgidld_i_slot;
        let mut var_bgidld_i_rv: f64 = *var_bgidld_i_rv_slot;
        let mut var_cfbedge_i: f64 = *var_cfbedge_i_slot;
        let mut var_cfbedge_i_rv: f64 = *var_cfbedge_i_rv_slot;
        let mut var_cfdedge_i: f64 = *var_cfdedge_i_slot;
        let mut var_cfdedge_i_rv: f64 = *var_cfdedge_i_rv_slot;
        let mut var_cfedge_i: f64 = *var_cfedge_i_slot;
        let mut var_cfedge_i_rv: f64 = *var_cfedge_i_rv_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cfrd_i_rv: f64 = *var_cfrd_i_rv_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_cgidld_i_rv: f64 = *var_cgidld_i_rv_slot;
        let mut var_cgovd_i: f64 = *var_cgovd_i_slot;
        let mut var_cgovd_i_rv: f64 = *var_cgovd_i_rv_slot;
        let mut var_cinrd_i: f64 = *var_cinrd_i_slot;
        let mut var_cinrd_i_rv: f64 = *var_cinrd_i_rv_slot;
        let mut var_cox_over_q: f64 = *var_cox_over_q_slot;
        let mut var_cox_over_q_rv: f64 = *var_cox_over_q_rv_slot;
        let mut var_coxovprime: f64 = *var_coxovprime_slot;
        let mut var_coxovprime_d: f64 = *var_coxovprime_d_slot;
        let mut var_coxovprime_d_rv: f64 = *var_coxovprime_d_rv_slot;
        let mut var_coxovprime_rv: f64 = *var_coxovprime_rv_slot;
        let mut var_coxprime: f64 = *var_coxprime_slot;
        let mut var_coxprime_rv: f64 = *var_coxprime_rv_slot;
        let mut var_delvto_i: f64 = *var_delvto_i_slot;
        let mut var_delvto_i_rv: f64 = *var_delvto_i_rv_slot;
        let mut var_delvtoedge_i: f64 = *var_delvtoedge_i_slot;
        let mut var_delvtoedge_i_rv: f64 = *var_delvtoedge_i_rv_slot;
        let mut var_dxgb_ov_d: f64 = *var_dxgb_ov_d_slot;
        let mut var_dxgb_ov_d_rv: f64 = *var_dxgb_ov_d_rv_slot;
        let mut var_dxgb_ov_s: f64 = *var_dxgb_ov_s_slot;
        let mut var_dxgb_ov_s_rv: f64 = *var_dxgb_ov_s_rv_slot;
        let mut var_dxgb_ov_th: f64 = *var_dxgb_ov_th_slot;
        let mut var_dxgb_ov_th_rv: f64 = *var_dxgb_ov_th_rv_slot;
        let mut var_e_eff0: f64 = *var_e_eff0_slot;
        let mut var_e_eff0_rv: f64 = *var_e_eff0_rv_slot;
        let mut var_epsox: f64 = *var_epsox_slot;
        let mut var_epsox_rv: f64 = *var_epsox_rv_slot;
        let mut var_eta_mu: f64 = *var_eta_mu_slot;
        let mut var_eta_mu1: f64 = *var_eta_mu1_slot;
        let mut var_eta_mu1_rv: f64 = *var_eta_mu1_rv_slot;
        let mut var_eta_mu_rv: f64 = *var_eta_mu_rv_slot;
        let mut var_factuo_i: f64 = *var_factuo_i_slot;
        let mut var_factuo_i_rv: f64 = *var_factuo_i_rv_slot;
        let mut var_factuoedge_i: f64 = *var_factuoedge_i_slot;
        let mut var_factuoedge_i_rv: f64 = *var_factuoedge_i_rv_slot;
        let mut var_fcgovaccd_i: f64 = *var_fcgovaccd_i_slot;
        let mut var_fcgovaccd_i_rv: f64 = *var_fcgovaccd_i_rv_slot;
        let mut var_gc2ovd_i: f64 = *var_gc2ovd_i_slot;
        let mut var_gc2ovd_i_rv: f64 = *var_gc2ovd_i_rv_slot;
        let mut var_gc3ovd_i: f64 = *var_gc3ovd_i_slot;
        let mut var_gc3ovd_i_rv: f64 = *var_gc3ovd_i_rv_slot;
        let mut var_gov2_d: f64 = *var_gov2_d_slot;
        let mut var_gov2_d_rv: f64 = *var_gov2_d_rv_slot;
        let mut var_gov2_s: f64 = *var_gov2_s_slot;
        let mut var_gov2_s_rv: f64 = *var_gov2_s_rv_slot;
        let mut var_gov_d: f64 = *var_gov_d_slot;
        let mut var_gov_d_rv: f64 = *var_gov_d_rv_slot;
        let mut var_gov_s: f64 = *var_gov_s_slot;
        let mut var_gov_s_rv: f64 = *var_gov_s_rv_slot;
        let mut var_guard153: f64 = *var_guard153_slot;
        let mut var_guard153_rv: f64 = *var_guard153_rv_slot;
        let mut var_guard154: f64 = *var_guard154_slot;
        let mut var_guard154_rv: f64 = *var_guard154_rv_slot;
        let mut var_guard155: f64 = *var_guard155_slot;
        let mut var_guard155_rv: f64 = *var_guard155_rv_slot;
        let mut var_guard156: f64 = *var_guard156_slot;
        let mut var_guard156_rv: f64 = *var_guard156_rv_slot;
        let mut var_guard157: f64 = *var_guard157_slot;
        let mut var_guard157_rv: f64 = *var_guard157_rv_slot;
        let mut var_guard158: f64 = *var_guard158_slot;
        let mut var_guard158_rv: f64 = *var_guard158_rv_slot;
        let mut var_guard159: f64 = *var_guard159_slot;
        let mut var_guard159_rv: f64 = *var_guard159_rv_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_igovd_i_rv: f64 = *var_igovd_i_rv_slot;
        let mut var_inv_gov: f64 = *var_inv_gov_slot;
        let mut var_inv_gov_rv: f64 = *var_inv_gov_rv_slot;
        let mut var_inv_vp: f64 = *var_inv_vp_slot;
        let mut var_inv_vp_rv: f64 = *var_inv_vp_rv_slot;
        let mut var_mult_inst: f64 = *var_mult_inst_slot;
        let mut var_mult_inst_rv: f64 = *var_mult_inst_rv_slot;
        let mut var_munqs_i: f64 = *var_munqs_i_slot;
        let mut var_munqs_i_rv: f64 = *var_munqs_i_rv_slot;
        let mut var_neffac_i: f64 = *var_neffac_i_slot;
        let mut var_neffac_i_rv: f64 = *var_neffac_i_rv_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_novd_i_rv: f64 = *var_novd_i_rv_slot;
        let mut var_pscededge_i: f64 = *var_pscededge_i_slot;
        let mut var_pscededge_i_rv: f64 = *var_pscededge_i_rv_slot;
        let mut var_qq: f64 = *var_qq_slot;
        let mut var_qq_rv: f64 = *var_qq_rv_slot;
        let mut var_sp_ov_a_s: f64 = *var_sp_ov_a_s_slot;
        let mut var_sp_ov_a_s_rv: f64 = *var_sp_ov_a_s_rv_slot;
        let mut var_sp_ov_delta: f64 = *var_sp_ov_delta_slot;
        let mut var_sp_ov_delta_rv: f64 = *var_sp_ov_delta_rv_slot;
        let mut var_sp_ov_eps: f64 = *var_sp_ov_eps_slot;
        let mut var_sp_ov_eps2_s: f64 = *var_sp_ov_eps2_s_slot;
        let mut var_sp_ov_eps2_s_rv: f64 = *var_sp_ov_eps2_s_rv_slot;
        let mut var_sp_ov_eps_rv: f64 = *var_sp_ov_eps_rv_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_stbgidld_i_rv: f64 = *var_stbgidld_i_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;
        let mut var_tox_sq: f64 = *var_tox_sq_slot;
        let mut var_tox_sq_rv: f64 = *var_tox_sq_rv_slot;
        let mut var_toxovd_i: f64 = *var_toxovd_i_slot;
        let mut var_toxovd_i_rv: f64 = *var_toxovd_i_rv_slot;

        let (assign10870_e10283,) = {
    if (var_pscededge_p > 0.0) {
        (var_pscededge_p,)
    } else {
        (0.0,)
    }
};
        var_pscededge_i = assign10870_e10283;
        var_pscededge_i_rv = 0.0;

        let (assign10880_e10289,) = {
    if (var_cfedge_p > 0.0) {
        (var_cfedge_p,)
    } else {
        (0.0,)
    }
};
        var_cfedge_i = assign10880_e10289;
        var_cfedge_i_rv = 0.0;

        let (assign10890_e10300,) = {
    if (var_cfbedge_p > 0.0) {
        let (assign10890_e10298,) = {
            if (var_cfbedge_p < 1.0) {
                (var_cfbedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10890_e10298,)
    } else {
        (0.0,)
    }
};
        var_cfbedge_i = assign10890_e10300;
        var_cfbedge_i_rv = 0.0;

        let (assign10900_e10306,) = {
    if (var_cfdedge_p > 0.0) {
        (var_cfdedge_p,)
    } else {
        (0.0,)
    }
};
        var_cfdedge_i = assign10900_e10306;
        var_cfdedge_i_rv = 0.0;

        let assign11030_e10341: f64 = (p.p31 * var_nf_i);
        let (assign11030_e10348,) = {
    if (assign11030_e10341 > 0.0) {
        let assign11030_e10346: f64 = (p.p31 * var_nf_i);
        (assign11030_e10346,)
    } else {
        (0.0,)
    }
};
        var_mult_inst = assign11030_e10348;
        var_mult_inst_rv = 0.0;

        var_factuo_i = p.p16;
        var_factuo_i_rv = 0.0;

        var_delvto_i = p.p15;
        var_delvto_i_rv = 0.0;

        var_factuoedge_i = p.p18;
        var_factuoedge_i_rv = 0.0;

        var_delvtoedge_i = p.p17;
        var_delvtoedge_i_rv = 0.0;

        let (assign11080_e10358,) = {
    if (var_munqs_p > 0.0) {
        (var_munqs_p,)
    } else {
        (0.0,)
    }
};
        var_munqs_i = assign11080_e10358;
        var_munqs_i_rv = 0.0;

        let assign11090_e10361: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard153 = assign11090_e10361;
        var_guard153_rv = 0.0;

        let (assign11100_e10365,) = {
    if (var_guard153 != 0.0) {
        (var_toxov_i,)
    } else {
        (var_toxovd_i,)
    }
};
        var_toxovd_i = assign11100_e10365;
        var_toxovd_i_rv = 0.0;

        let (assign11110_e10369,) = {
    if (var_guard153 != 0.0) {
        (var_nov_i,)
    } else {
        (var_novd_i,)
    }
};
        var_novd_i = assign11110_e10369;
        var_novd_i_rv = 0.0;

        let (assign11120_e10373,) = {
    if (var_guard153 != 0.0) {
        (var_agidl_i,)
    } else {
        (var_agidld_i,)
    }
};
        var_agidld_i = assign11120_e10373;
        var_agidld_i_rv = 0.0;

        let (assign11130_e10377,) = {
    if (var_guard153 != 0.0) {
        (var_bgidl_i,)
    } else {
        (var_bgidld_i,)
    }
};
        var_bgidld_i = assign11130_e10377;
        var_bgidld_i_rv = 0.0;

        let (assign11140_e10381,) = {
    if (var_guard153 != 0.0) {
        (var_stbgidl_i,)
    } else {
        (var_stbgidld_i,)
    }
};
        var_stbgidld_i = assign11140_e10381;
        var_stbgidld_i_rv = 0.0;

        let (assign11150_e10385,) = {
    if (var_guard153 != 0.0) {
        (var_cgidl_i,)
    } else {
        (var_cgidld_i,)
    }
};
        var_cgidld_i = assign11150_e10385;
        var_cgidld_i_rv = 0.0;

        let (assign11160_e10389,) = {
    if (var_guard153 != 0.0) {
        (var_igov_i,)
    } else {
        (var_igovd_i,)
    }
};
        var_igovd_i = assign11160_e10389;
        var_igovd_i_rv = 0.0;

        let (assign11170_e10393,) = {
    if (var_guard153 != 0.0) {
        (var_gc2ov_i,)
    } else {
        (var_gc2ovd_i,)
    }
};
        var_gc2ovd_i = assign11170_e10393;
        var_gc2ovd_i_rv = 0.0;

        let (assign11180_e10397,) = {
    if (var_guard153 != 0.0) {
        (var_gc3ov_i,)
    } else {
        (var_gc3ovd_i,)
    }
};
        var_gc3ovd_i = assign11180_e10397;
        var_gc3ovd_i_rv = 0.0;

        let (assign11190_e10401,) = {
    if (var_guard153 != 0.0) {
        (var_cgov_i,)
    } else {
        (var_cgovd_i,)
    }
};
        var_cgovd_i = assign11190_e10401;
        var_cgovd_i_rv = 0.0;

        let (assign11200_e10405,) = {
    if (var_guard153 != 0.0) {
        (var_fcgovacc_i,)
    } else {
        (var_fcgovaccd_i,)
    }
};
        var_fcgovaccd_i = assign11200_e10405;
        var_fcgovaccd_i_rv = 0.0;

        let (assign11210_e10409,) = {
    if (var_guard153 != 0.0) {
        (var_cinr_i,)
    } else {
        (var_cinrd_i,)
    }
};
        var_cinrd_i = assign11210_e10409;
        var_cinrd_i_rv = 0.0;

        let (assign11220_e10413,) = {
    if (var_guard153 != 0.0) {
        (var_cfr_i,)
    } else {
        (var_cfrd_i,)
    }
};
        var_cfrd_i = assign11220_e10413;
        var_cfrd_i_rv = 0.0;

        let assign11230_e10416: f64 = (8.8541878176e-12 * var_epsrox_i);
        var_epsox = assign11230_e10416;
        var_epsox_rv = 0.0;

        let assign11240_e10419: f64 = (var_epsox / var_tox_i);
        var_coxprime = assign11240_e10419;
        var_coxprime_rv = 0.0;

        let assign11250_e10422: f64 = (var_tox_i * var_tox_i);
        var_tox_sq = assign11250_e10422;
        var_tox_sq_rv = 0.0;

        let assign11260_e10425: f64 = (var_coxprime / 1.6021918e-19);
        var_cox_over_q = assign11260_e10425;
        var_cox_over_q_rv = 0.0;

        let assign11270_e10428: f64 = (var_facneffac_i * var_neff_i);
        var_neffac_i = assign11270_e10428;
        var_neffac_i_rv = 0.0;

        let (assign11280_e10439,) = {
    if (var_neffac_i > 1e20) {
        let (assign11280_e10437,) = {
            if (var_neffac_i < 1e26) {
                (var_neffac_i,)
            } else {
                (1e26,)
            }
        };
        (assign11280_e10437,)
    } else {
        (1e20,)
    }
};
        var_neffac_i = assign11280_e10439;
        var_neffac_i_rv = 0.0;

        var_qq = 0.0;
        var_qq_rv = 0.0;

        let assign11300_e10443: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        var_guard154 = assign11300_e10443;
        var_guard154_rv = 0.0;

        let (assign11310_e10455,) = {
    if (var_guard154 != 0.0) {
        let assign11310_e10447: f64 = (0.4 * 5.951993);
        let assign11310_e10449: f64 = (assign11310_e10447 * p.p52);
        let assign11310_e10452: f64 = (var_coxprime).powf(0.6666666666666666);
        let assign11310_e10453: f64 = (assign11310_e10449 * assign11310_e10452);
        (assign11310_e10453,)
    } else {
        (var_qq,)
    }
};
        var_qq = assign11310_e10455;
        var_qq_rv = 0.0;

        let assign11320_e10458: f64 = (-1.0);
        let assign11320_e10459: f64 = if var_chnl_type == assign11320_e10458 { 1.0 } else { 0.0 };
        var_guard155 = assign11320_e10459;
        var_guard155_rv = 0.0;

        let (assign11330_e10469,) = {
    if ((var_guard154 != 0.0) && (var_guard155 != 0.0)) {
        let assign11330_e10465: f64 = (7.448711 / 5.951993);
        let assign11330_e10467: f64 = (assign11330_e10465 * var_qq);
        (assign11330_e10467,)
    } else {
        (var_qq,)
    }
};
        var_qq = assign11330_e10469;
        var_qq_rv = 0.0;

        let assign11340_e10472: f64 = (1e-8 * var_coxprime);
        let assign11340_e10474: f64 = (assign11340_e10472 / var_epssi);
        var_e_eff0 = assign11340_e10474;
        var_e_eff0_rv = 0.0;

        let assign11350_e10477: f64 = (0.5 * var_feta_i);
        var_eta_mu = assign11350_e10477;
        var_eta_mu_rv = 0.0;

        var_eta_mu1 = 0.5;
        var_eta_mu1_rv = 0.0;

        let assign11370_e10481: f64 = (-1.0);
        let assign11370_e10482: f64 = if var_chnl_type == assign11370_e10481 { 1.0 } else { 0.0 };
        var_guard156 = assign11370_e10482;
        var_guard156_rv = 0.0;

        let (assign11380_e10488,) = {
    if (var_guard156 != 0.0) {
        let assign11380_e10486: f64 = (0.3333333333333333 * var_feta_i);
        (assign11380_e10486,)
    } else {
        (var_eta_mu,)
    }
};
        var_eta_mu = assign11380_e10488;
        var_eta_mu_rv = 0.0;

        let (assign11390_e10492,) = {
    if (var_guard156 != 0.0) {
        (0.3333333333333333,)
    } else {
        (var_eta_mu1,)
    }
};
        var_eta_mu1 = assign11390_e10492;
        var_eta_mu1_rv = 0.0;

        let assign11400_e10495: f64 = (-2.0);
        let assign11400_e10497: f64 = (assign11400_e10495 / var_ax_i);
        let assign11400_e10499: f64 = (assign11400_e10497 + 1.0);
        let assign11400_e10500: f64 = (2.0_f64).powf(assign11400_e10499);
        let assign11400_e10502: f64 = (assign11400_e10500 - 1.0);
        var_temp = assign11400_e10502;
        var_temp_rv = 0.0;

        let assign11410_e10505: f64 = (var_temp - 1.0);
        let assign11410_e10508: f64 = (var_temp - 1.0);
        let assign11410_e10509: f64 = (assign11410_e10505 * assign11410_e10508);
        let assign11410_e10512: f64 = (4.0 * var_temp);
        let (assign11410_e10519,) = {
    if (assign11410_e10512 > 0.0001) {
        let assign11410_e10517: f64 = (4.0 * var_temp);
        (assign11410_e10517,)
    } else {
        (0.0001,)
    }
};
        let assign11410_e10520: f64 = (assign11410_e10509 / assign11410_e10519);
        var_ar = assign11410_e10520;
        var_ar_rv = 0.0;

        let assign11420_e10523: f64 = (-2.0);
        let assign11420_e10525: f64 = (assign11420_e10523 / var_axac_i);
        let assign11420_e10527: f64 = (assign11420_e10525 + 1.0);
        let assign11420_e10528: f64 = (2.0_f64).powf(assign11420_e10527);
        let assign11420_e10530: f64 = (assign11420_e10528 - 1.0);
        var_temp = assign11420_e10530;
        var_temp_rv = 0.0;

        let assign11430_e10533: f64 = (var_temp - 1.0);
        let assign11430_e10536: f64 = (var_temp - 1.0);
        let assign11430_e10537: f64 = (assign11430_e10533 * assign11430_e10536);
        let assign11430_e10540: f64 = (4.0 * var_temp);
        let (assign11430_e10547,) = {
    if (assign11430_e10540 > 0.0001) {
        let assign11430_e10545: f64 = (4.0 * var_temp);
        (assign11430_e10545,)
    } else {
        (0.0001,)
    }
};
        let assign11430_e10548: f64 = (assign11430_e10537 / assign11430_e10547);
        var_arac = assign11430_e10548;
        var_arac_rv = 0.0;

        let assign11440_e10551: f64 = (1.0 / var_vp_i);
        var_inv_vp = assign11440_e10551;
        var_inv_vp_rv = 0.0;

        let assign11450_e10554: f64 = (var_epsox / var_toxov_i);
        var_coxovprime = assign11450_e10554;
        var_coxovprime_rv = 0.0;

        let assign11460_e10557: f64 = (var_epsox / var_toxovd_i);
        var_coxovprime_d = assign11460_e10557;
        var_coxovprime_d_rv = 0.0;

        let assign11470_e10560: f64 = (2.0 * 1.6021918e-19);
        let assign11470_e10562: f64 = (assign11470_e10560 * var_nov_i);
        let assign11470_e10564: f64 = (assign11470_e10562 * var_epssi);
        let assign11470_e10566: f64 = (assign11470_e10564 * var_inv_phita);
        let assign11470_e10567: f64 = (assign11470_e10566).sqrt();
        let assign11470_e10569: f64 = (assign11470_e10567 / var_coxovprime);
        var_gov_s = assign11470_e10569;
        var_gov_s_rv = 0.0;

        let assign11480_e10572: f64 = (2.0 * 1.6021918e-19);
        let assign11480_e10574: f64 = (assign11480_e10572 * var_novd_i);
        let assign11480_e10576: f64 = (assign11480_e10574 * var_epssi);
        let assign11480_e10578: f64 = (assign11480_e10576 * var_inv_phita);
        let assign11480_e10579: f64 = (assign11480_e10578).sqrt();
        let assign11480_e10581: f64 = (assign11480_e10579 / var_coxovprime_d);
        var_gov_d = assign11480_e10581;
        var_gov_d_rv = 0.0;

        let assign11490_e10584: f64 = (var_gov_s * var_gov_s);
        var_gov2_s = assign11490_e10584;
        var_gov2_s_rv = 0.0;

        let assign11500_e10587: f64 = (var_gov_d * var_gov_d);
        var_gov2_d = assign11500_e10587;
        var_gov2_d_rv = 0.0;

        let assign11510_e10590: f64 = (var_cgovaccg_i * 0.005);
        let assign11510_e10592: f64 = (assign11510_e10590 * var_inv_phita);
        let assign11510_e10593: f64 = (assign11510_e10592).exp();
        let assign11510_e10595: f64 = (assign11510_e10593 - 1.0);
        let assign11510_e10596: f64 = (assign11510_e10595).ln();
        let assign11510_e10598: f64 = (assign11510_e10596 / var_cgovaccg_i);
        let assign11510_e10601: f64 = (0.005 * var_inv_phita);
        let assign11510_e10602: f64 = (assign11510_e10601).exp();
        let assign11510_e10604: f64 = (assign11510_e10602 - 1.0);
        let assign11510_e10605: f64 = (assign11510_e10604).ln();
        let assign11510_e10606: f64 = (assign11510_e10598 - assign11510_e10605);
        var_dxgb_ov_th = assign11510_e10606;
        var_dxgb_ov_th_rv = 0.0;

        let assign11520_e10609: f64 = (0.5 * var_gov_s);
        let assign11520_e10610: f64 = (assign11520_e10609).ln();
        let assign11520_e10612: f64 = (assign11520_e10610 + var_dxgb_ov_th);
        var_dxgb_ov_s = assign11520_e10612;
        var_dxgb_ov_s_rv = 0.0;

        let assign11530_e10615: f64 = (0.5 * var_gov_d);
        let assign11530_e10616: f64 = (assign11530_e10615).ln();
        let assign11530_e10618: f64 = (assign11530_e10616 + var_dxgb_ov_th);
        var_dxgb_ov_d = assign11530_e10618;
        var_dxgb_ov_d_rv = 0.0;

        let assign11540_e10621: f64 = (1.0 / var_gov_s);
        var_inv_gov = assign11540_e10621;
        var_inv_gov_rv = 0.0;

        let assign11550_e10624: f64 = (3.1 * var_gov_s);
        let assign11550_e10626: f64 = (assign11550_e10624 + 8.5);
        var_sp_ov_eps = assign11550_e10626;
        var_sp_ov_eps_rv = 0.0;

        let assign11560_e10629: f64 = (var_sp_ov_eps * var_sp_ov_eps);
        var_sp_ov_eps2_s = assign11560_e10629;
        var_sp_ov_eps2_s_rv = 0.0;

        let assign11570_e10632: f64 = (0.5 * var_sp_ov_eps);
        var_sp_ov_delta = assign11570_e10632;
        var_sp_ov_delta_rv = 0.0;

        let assign11580_e10635: f64 = if var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        var_guard157 = assign11580_e10635;
        var_guard157_rv = 0.0;

        let (assign11590_e10641,) = {
    if (var_guard157 != 0.0) {
        let assign11590_e10639: f64 = (64.0 * var_inv_gov);
        (assign11590_e10639,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11590_e10641;
        var_sp_ov_a_s_rv = 0.0;

        let assign11600_e10644: f64 = if var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        var_guard158 = assign11600_e10644;
        var_guard158_rv = 0.0;

        let (assign11610_e10655,) = {
    if ((var_guard157 == 0.0) && (var_guard158 != 0.0)) {
        let assign11610_e10651: f64 = (22.0 * var_inv_gov);
        let assign11610_e10653: f64 = (assign11610_e10651 + 3.0);
        (assign11610_e10653,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11610_e10655;
        var_sp_ov_a_s_rv = 0.0;

        let assign11620_e10658: f64 = if var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        var_guard159 = assign11620_e10658;
        var_guard159_rv = 0.0;

        let (assign11630_e10673,) = {
    if (((var_guard157 == 0.0) && (var_guard158 == 0.0)) && (var_guard159 != 0.0)) {
        let assign11630_e10667: f64 = (-7.2);
        let assign11630_e10669: f64 = (assign11630_e10667 * var_inv_gov);
        let assign11630_e10671: f64 = (assign11630_e10669 + 15.5);
        (assign11630_e10671,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11630_e10673;
        var_sp_ov_a_s_rv = 0.0;

        *var_agidld_i_slot = var_agidld_i;
        *var_agidld_i_rv_slot = var_agidld_i_rv;
        *var_ar_slot = var_ar;
        *var_ar_rv_slot = var_ar_rv;
        *var_arac_slot = var_arac;
        *var_arac_rv_slot = var_arac_rv;
        *var_bgidld_i_slot = var_bgidld_i;
        *var_bgidld_i_rv_slot = var_bgidld_i_rv;
        *var_cfbedge_i_slot = var_cfbedge_i;
        *var_cfbedge_i_rv_slot = var_cfbedge_i_rv;
        *var_cfdedge_i_slot = var_cfdedge_i;
        *var_cfdedge_i_rv_slot = var_cfdedge_i_rv;
        *var_cfedge_i_slot = var_cfedge_i;
        *var_cfedge_i_rv_slot = var_cfedge_i_rv;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cfrd_i_rv_slot = var_cfrd_i_rv;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_cgidld_i_rv_slot = var_cgidld_i_rv;
        *var_cgovd_i_slot = var_cgovd_i;
        *var_cgovd_i_rv_slot = var_cgovd_i_rv;
        *var_cinrd_i_slot = var_cinrd_i;
        *var_cinrd_i_rv_slot = var_cinrd_i_rv;
        *var_cox_over_q_slot = var_cox_over_q;
        *var_cox_over_q_rv_slot = var_cox_over_q_rv;
        *var_coxovprime_slot = var_coxovprime;
        *var_coxovprime_d_slot = var_coxovprime_d;
        *var_coxovprime_d_rv_slot = var_coxovprime_d_rv;
        *var_coxovprime_rv_slot = var_coxovprime_rv;
        *var_coxprime_slot = var_coxprime;
        *var_coxprime_rv_slot = var_coxprime_rv;
        *var_delvto_i_slot = var_delvto_i;
        *var_delvto_i_rv_slot = var_delvto_i_rv;
        *var_delvtoedge_i_slot = var_delvtoedge_i;
        *var_delvtoedge_i_rv_slot = var_delvtoedge_i_rv;
        *var_dxgb_ov_d_slot = var_dxgb_ov_d;
        *var_dxgb_ov_d_rv_slot = var_dxgb_ov_d_rv;
        *var_dxgb_ov_s_slot = var_dxgb_ov_s;
        *var_dxgb_ov_s_rv_slot = var_dxgb_ov_s_rv;
        *var_dxgb_ov_th_slot = var_dxgb_ov_th;
        *var_dxgb_ov_th_rv_slot = var_dxgb_ov_th_rv;
        *var_e_eff0_slot = var_e_eff0;
        *var_e_eff0_rv_slot = var_e_eff0_rv;
        *var_epsox_slot = var_epsox;
        *var_epsox_rv_slot = var_epsox_rv;
        *var_eta_mu_slot = var_eta_mu;
        *var_eta_mu1_slot = var_eta_mu1;
        *var_eta_mu1_rv_slot = var_eta_mu1_rv;
        *var_eta_mu_rv_slot = var_eta_mu_rv;
        *var_factuo_i_slot = var_factuo_i;
        *var_factuo_i_rv_slot = var_factuo_i_rv;
        *var_factuoedge_i_slot = var_factuoedge_i;
        *var_factuoedge_i_rv_slot = var_factuoedge_i_rv;
        *var_fcgovaccd_i_slot = var_fcgovaccd_i;
        *var_fcgovaccd_i_rv_slot = var_fcgovaccd_i_rv;
        *var_gc2ovd_i_slot = var_gc2ovd_i;
        *var_gc2ovd_i_rv_slot = var_gc2ovd_i_rv;
        *var_gc3ovd_i_slot = var_gc3ovd_i;
        *var_gc3ovd_i_rv_slot = var_gc3ovd_i_rv;
        *var_gov2_d_slot = var_gov2_d;
        *var_gov2_d_rv_slot = var_gov2_d_rv;
        *var_gov2_s_slot = var_gov2_s;
        *var_gov2_s_rv_slot = var_gov2_s_rv;
        *var_gov_d_slot = var_gov_d;
        *var_gov_d_rv_slot = var_gov_d_rv;
        *var_gov_s_slot = var_gov_s;
        *var_gov_s_rv_slot = var_gov_s_rv;
        *var_guard153_slot = var_guard153;
        *var_guard153_rv_slot = var_guard153_rv;
        *var_guard154_slot = var_guard154;
        *var_guard154_rv_slot = var_guard154_rv;
        *var_guard155_slot = var_guard155;
        *var_guard155_rv_slot = var_guard155_rv;
        *var_guard156_slot = var_guard156;
        *var_guard156_rv_slot = var_guard156_rv;
        *var_guard157_slot = var_guard157;
        *var_guard157_rv_slot = var_guard157_rv;
        *var_guard158_slot = var_guard158;
        *var_guard158_rv_slot = var_guard158_rv;
        *var_guard159_slot = var_guard159;
        *var_guard159_rv_slot = var_guard159_rv;
        *var_igovd_i_slot = var_igovd_i;
        *var_igovd_i_rv_slot = var_igovd_i_rv;
        *var_inv_gov_slot = var_inv_gov;
        *var_inv_gov_rv_slot = var_inv_gov_rv;
        *var_inv_vp_slot = var_inv_vp;
        *var_inv_vp_rv_slot = var_inv_vp_rv;
        *var_mult_inst_slot = var_mult_inst;
        *var_mult_inst_rv_slot = var_mult_inst_rv;
        *var_munqs_i_slot = var_munqs_i;
        *var_munqs_i_rv_slot = var_munqs_i_rv;
        *var_neffac_i_slot = var_neffac_i;
        *var_neffac_i_rv_slot = var_neffac_i_rv;
        *var_novd_i_slot = var_novd_i;
        *var_novd_i_rv_slot = var_novd_i_rv;
        *var_pscededge_i_slot = var_pscededge_i;
        *var_pscededge_i_rv_slot = var_pscededge_i_rv;
        *var_qq_slot = var_qq;
        *var_qq_rv_slot = var_qq_rv;
        *var_sp_ov_a_s_slot = var_sp_ov_a_s;
        *var_sp_ov_a_s_rv_slot = var_sp_ov_a_s_rv;
        *var_sp_ov_delta_slot = var_sp_ov_delta;
        *var_sp_ov_delta_rv_slot = var_sp_ov_delta_rv;
        *var_sp_ov_eps_slot = var_sp_ov_eps;
        *var_sp_ov_eps2_s_slot = var_sp_ov_eps2_s;
        *var_sp_ov_eps2_s_rv_slot = var_sp_ov_eps2_s_rv;
        *var_sp_ov_eps_rv_slot = var_sp_ov_eps_rv;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_stbgidld_i_rv_slot = var_stbgidld_i_rv;
        *var_temp_slot = var_temp;
        *var_temp_rv_slot = var_temp_rv;
        *var_tox_sq_slot = var_tox_sq;
        *var_tox_sq_rv_slot = var_tox_sq_rv;
        *var_toxovd_i_slot = var_toxovd_i;
        *var_toxovd_i_rv_slot = var_toxovd_i_rv;
    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        var_betn_i: f64,
        var_coxprime: f64,
        var_cs_i: f64,
        var_ct_i: f64,
        var_ctg_i: f64,
        var_delt: f64,
        var_delvtac_i: f64,
        var_delvto_i: f64,
        var_dphib_i: f64,
        var_dvsbnud_i: f64,
        var_eg: f64,
        var_epssi: f64,
        var_factuo_i: f64,
        var_gov2_d: f64,
        var_gov2_s: f64,
        var_gov_d: f64,
        var_gov_s: f64,
        var_guard157: f64,
        var_guard158: f64,
        var_guard159: f64,
        var_inv_phit: f64,
        var_ln_rtn: f64,
        var_mue_i: f64,
        var_neff_i: f64,
        var_neffac_i: f64,
        var_np_i: f64,
        var_phibfac: f64,
        var_phit: f64,
        var_qq: f64,
        var_rtn: f64,
        var_st2vfb_i: f64,
        var_stbet_i: f64,
        var_stcs_i: f64,
        var_stct_i: f64,
        var_stmue_i: f64,
        var_stthecs_i: f64,
        var_stthemu_i: f64,
        var_stvfb_i: f64,
        var_thecs_i: f64,
        var_themu_i: f64,
        var_tox_sq: f64,
        var_vfb_i: f64,
        var_vsbnud_i: f64,
        var_alpha_b_slot: &mut f64,
        var_alpha_b_rv_slot: &mut f64,
        var_aphi_ac_slot: &mut f64,
        var_aphi_ac_rv_slot: &mut f64,
        var_aphi_dc_slot: &mut f64,
        var_aphi_dc_rv_slot: &mut f64,
        var_arg2max_slot: &mut f64,
        var_arg2max_rv_slot: &mut f64,
        var_bet_i_slot: &mut f64,
        var_bet_i_rv_slot: &mut f64,
        var_betn_t_slot: &mut f64,
        var_betn_t_rv_slot: &mut f64,
        var_bphi_ac_slot: &mut f64,
        var_bphi_ac_rv_slot: &mut f64,
        var_bphi_dc_slot: &mut f64,
        var_bphi_dc_rv_slot: &mut f64,
        var_cs_t_slot: &mut f64,
        var_cs_t_rv_slot: &mut f64,
        var_ct_t_slot: &mut f64,
        var_ct_t_rv_slot: &mut f64,
        var_ctg_t_slot: &mut f64,
        var_ctg_t_rv_slot: &mut f64,
        var_dphibq_slot: &mut f64,
        var_dphibq_rv_slot: &mut f64,
        var_g_0_ac_slot: &mut f64,
        var_g_0_ac_rv_slot: &mut f64,
        var_g_0_dc_slot: &mut f64,
        var_g_0_dc_rv_slot: &mut f64,
        var_guard160_slot: &mut f64,
        var_guard160_rv_slot: &mut f64,
        var_guard161_slot: &mut f64,
        var_guard161_rv_slot: &mut f64,
        var_guard162_slot: &mut f64,
        var_guard162_rv_slot: &mut f64,
        var_guard163_slot: &mut f64,
        var_guard163_rv_slot: &mut f64,
        var_guard164_slot: &mut f64,
        var_guard164_rv_slot: &mut f64,
        var_guard165_slot: &mut f64,
        var_guard165_rv_slot: &mut f64,
        var_inv_gov_slot: &mut f64,
        var_inv_gov_rv_slot: &mut f64,
        var_kp_slot: &mut f64,
        var_kp_rv_slot: &mut f64,
        var_mue_t_slot: &mut f64,
        var_mue_t_rv_slot: &mut f64,
        var_np_slot: &mut f64,
        var_np_rv_slot: &mut f64,
        var_phib_ac_slot: &mut f64,
        var_phib_ac_rv_slot: &mut f64,
        var_phib_dc_slot: &mut f64,
        var_phib_dc_rv_slot: &mut f64,
        var_phix1_ac_slot: &mut f64,
        var_phix1_ac_rv_slot: &mut f64,
        var_phix1_dc_slot: &mut f64,
        var_phix1_dc_rv_slot: &mut f64,
        var_phix2_slot: &mut f64,
        var_phix2_rv_slot: &mut f64,
        var_phix_ac_slot: &mut f64,
        var_phix_ac_rv_slot: &mut f64,
        var_phix_dc_slot: &mut f64,
        var_phix_dc_rv_slot: &mut f64,
        var_qb0_slot: &mut f64,
        var_qb0_rv_slot: &mut f64,
        var_qlim2_slot: &mut f64,
        var_qlim2_rv_slot: &mut f64,
        var_sp_ov_a_d_slot: &mut f64,
        var_sp_ov_a_d_rv_slot: &mut f64,
        var_sp_ov_a_s_slot: &mut f64,
        var_sp_ov_a_s_rv_slot: &mut f64,
        var_sp_ov_delta_slot: &mut f64,
        var_sp_ov_delta1_d_slot: &mut f64,
        var_sp_ov_delta1_d_rv_slot: &mut f64,
        var_sp_ov_delta1_s_slot: &mut f64,
        var_sp_ov_delta1_s_rv_slot: &mut f64,
        var_sp_ov_delta_rv_slot: &mut f64,
        var_sp_ov_eps_slot: &mut f64,
        var_sp_ov_eps2_d_slot: &mut f64,
        var_sp_ov_eps2_d_rv_slot: &mut f64,
        var_sp_ov_eps_rv_slot: &mut f64,
        var_sqrt_phib_dc_slot: &mut f64,
        var_sqrt_phib_dc_rv_slot: &mut f64,
        var_tf_bet_slot: &mut f64,
        var_tf_bet_rv_slot: &mut f64,
        var_tf_cs_slot: &mut f64,
        var_tf_cs_rv_slot: &mut f64,
        var_tf_ct_slot: &mut f64,
        var_tf_ct_rv_slot: &mut f64,
        var_tf_mue_slot: &mut f64,
        var_tf_mue_rv_slot: &mut f64,
        var_thecs_t_slot: &mut f64,
        var_thecs_t_rv_slot: &mut f64,
        var_themu_t_slot: &mut f64,
        var_themu_t_rv_slot: &mut f64,
        var_us1_slot: &mut f64,
        var_us1_rv_slot: &mut f64,
        var_us21_slot: &mut f64,
        var_us21_rv_slot: &mut f64,
        var_vfb_t_slot: &mut f64,
        var_vfb_t_rv_slot: &mut f64,
    ) {
        let mut var_alpha_b: f64 = *var_alpha_b_slot;
        let mut var_alpha_b_rv: f64 = *var_alpha_b_rv_slot;
        let mut var_aphi_ac: f64 = *var_aphi_ac_slot;
        let mut var_aphi_ac_rv: f64 = *var_aphi_ac_rv_slot;
        let mut var_aphi_dc: f64 = *var_aphi_dc_slot;
        let mut var_aphi_dc_rv: f64 = *var_aphi_dc_rv_slot;
        let mut var_arg2max: f64 = *var_arg2max_slot;
        let mut var_arg2max_rv: f64 = *var_arg2max_rv_slot;
        let mut var_bet_i: f64 = *var_bet_i_slot;
        let mut var_bet_i_rv: f64 = *var_bet_i_rv_slot;
        let mut var_betn_t: f64 = *var_betn_t_slot;
        let mut var_betn_t_rv: f64 = *var_betn_t_rv_slot;
        let mut var_bphi_ac: f64 = *var_bphi_ac_slot;
        let mut var_bphi_ac_rv: f64 = *var_bphi_ac_rv_slot;
        let mut var_bphi_dc: f64 = *var_bphi_dc_slot;
        let mut var_bphi_dc_rv: f64 = *var_bphi_dc_rv_slot;
        let mut var_cs_t: f64 = *var_cs_t_slot;
        let mut var_cs_t_rv: f64 = *var_cs_t_rv_slot;
        let mut var_ct_t: f64 = *var_ct_t_slot;
        let mut var_ct_t_rv: f64 = *var_ct_t_rv_slot;
        let mut var_ctg_t: f64 = *var_ctg_t_slot;
        let mut var_ctg_t_rv: f64 = *var_ctg_t_rv_slot;
        let mut var_dphibq: f64 = *var_dphibq_slot;
        let mut var_dphibq_rv: f64 = *var_dphibq_rv_slot;
        let mut var_g_0_ac: f64 = *var_g_0_ac_slot;
        let mut var_g_0_ac_rv: f64 = *var_g_0_ac_rv_slot;
        let mut var_g_0_dc: f64 = *var_g_0_dc_slot;
        let mut var_g_0_dc_rv: f64 = *var_g_0_dc_rv_slot;
        let mut var_guard160: f64 = *var_guard160_slot;
        let mut var_guard160_rv: f64 = *var_guard160_rv_slot;
        let mut var_guard161: f64 = *var_guard161_slot;
        let mut var_guard161_rv: f64 = *var_guard161_rv_slot;
        let mut var_guard162: f64 = *var_guard162_slot;
        let mut var_guard162_rv: f64 = *var_guard162_rv_slot;
        let mut var_guard163: f64 = *var_guard163_slot;
        let mut var_guard163_rv: f64 = *var_guard163_rv_slot;
        let mut var_guard164: f64 = *var_guard164_slot;
        let mut var_guard164_rv: f64 = *var_guard164_rv_slot;
        let mut var_guard165: f64 = *var_guard165_slot;
        let mut var_guard165_rv: f64 = *var_guard165_rv_slot;
        let mut var_inv_gov: f64 = *var_inv_gov_slot;
        let mut var_inv_gov_rv: f64 = *var_inv_gov_rv_slot;
        let mut var_kp: f64 = *var_kp_slot;
        let mut var_kp_rv: f64 = *var_kp_rv_slot;
        let mut var_mue_t: f64 = *var_mue_t_slot;
        let mut var_mue_t_rv: f64 = *var_mue_t_rv_slot;
        let mut var_np: f64 = *var_np_slot;
        let mut var_np_rv: f64 = *var_np_rv_slot;
        let mut var_phib_ac: f64 = *var_phib_ac_slot;
        let mut var_phib_ac_rv: f64 = *var_phib_ac_rv_slot;
        let mut var_phib_dc: f64 = *var_phib_dc_slot;
        let mut var_phib_dc_rv: f64 = *var_phib_dc_rv_slot;
        let mut var_phix1_ac: f64 = *var_phix1_ac_slot;
        let mut var_phix1_ac_rv: f64 = *var_phix1_ac_rv_slot;
        let mut var_phix1_dc: f64 = *var_phix1_dc_slot;
        let mut var_phix1_dc_rv: f64 = *var_phix1_dc_rv_slot;
        let mut var_phix2: f64 = *var_phix2_slot;
        let mut var_phix2_rv: f64 = *var_phix2_rv_slot;
        let mut var_phix_ac: f64 = *var_phix_ac_slot;
        let mut var_phix_ac_rv: f64 = *var_phix_ac_rv_slot;
        let mut var_phix_dc: f64 = *var_phix_dc_slot;
        let mut var_phix_dc_rv: f64 = *var_phix_dc_rv_slot;
        let mut var_qb0: f64 = *var_qb0_slot;
        let mut var_qb0_rv: f64 = *var_qb0_rv_slot;
        let mut var_qlim2: f64 = *var_qlim2_slot;
        let mut var_qlim2_rv: f64 = *var_qlim2_rv_slot;
        let mut var_sp_ov_a_d: f64 = *var_sp_ov_a_d_slot;
        let mut var_sp_ov_a_d_rv: f64 = *var_sp_ov_a_d_rv_slot;
        let mut var_sp_ov_a_s: f64 = *var_sp_ov_a_s_slot;
        let mut var_sp_ov_a_s_rv: f64 = *var_sp_ov_a_s_rv_slot;
        let mut var_sp_ov_delta: f64 = *var_sp_ov_delta_slot;
        let mut var_sp_ov_delta1_d: f64 = *var_sp_ov_delta1_d_slot;
        let mut var_sp_ov_delta1_d_rv: f64 = *var_sp_ov_delta1_d_rv_slot;
        let mut var_sp_ov_delta1_s: f64 = *var_sp_ov_delta1_s_slot;
        let mut var_sp_ov_delta1_s_rv: f64 = *var_sp_ov_delta1_s_rv_slot;
        let mut var_sp_ov_delta_rv: f64 = *var_sp_ov_delta_rv_slot;
        let mut var_sp_ov_eps: f64 = *var_sp_ov_eps_slot;
        let mut var_sp_ov_eps2_d: f64 = *var_sp_ov_eps2_d_slot;
        let mut var_sp_ov_eps2_d_rv: f64 = *var_sp_ov_eps2_d_rv_slot;
        let mut var_sp_ov_eps_rv: f64 = *var_sp_ov_eps_rv_slot;
        let mut var_sqrt_phib_dc: f64 = *var_sqrt_phib_dc_slot;
        let mut var_sqrt_phib_dc_rv: f64 = *var_sqrt_phib_dc_rv_slot;
        let mut var_tf_bet: f64 = *var_tf_bet_slot;
        let mut var_tf_bet_rv: f64 = *var_tf_bet_rv_slot;
        let mut var_tf_cs: f64 = *var_tf_cs_slot;
        let mut var_tf_cs_rv: f64 = *var_tf_cs_rv_slot;
        let mut var_tf_ct: f64 = *var_tf_ct_slot;
        let mut var_tf_ct_rv: f64 = *var_tf_ct_rv_slot;
        let mut var_tf_mue: f64 = *var_tf_mue_slot;
        let mut var_tf_mue_rv: f64 = *var_tf_mue_rv_slot;
        let mut var_thecs_t: f64 = *var_thecs_t_slot;
        let mut var_thecs_t_rv: f64 = *var_thecs_t_rv_slot;
        let mut var_themu_t: f64 = *var_themu_t_slot;
        let mut var_themu_t_rv: f64 = *var_themu_t_rv_slot;
        let mut var_us1: f64 = *var_us1_slot;
        let mut var_us1_rv: f64 = *var_us1_rv_slot;
        let mut var_us21: f64 = *var_us21_slot;
        let mut var_us21_rv: f64 = *var_us21_rv_slot;
        let mut var_vfb_t: f64 = *var_vfb_t_slot;
        let mut var_vfb_t_rv: f64 = *var_vfb_t_rv_slot;

        let (assign11640_e10684,) = {
    if (((var_guard157 == 0.0) && (var_guard158 == 0.0)) && (var_guard159 == 0.0)) {
        (var_gov_s,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11640_e10684;
        var_sp_ov_a_s_rv = 0.0;

        let assign11650_e10688: f64 = (var_gov2_s * 0.5);
        let assign11650_e10689: f64 = (var_sp_ov_delta + assign11650_e10688);
        let assign11650_e10694: f64 = (var_gov2_s * 0.25);
        let assign11650_e10695: f64 = (var_sp_ov_delta + assign11650_e10694);
        let assign11650_e10697: f64 = (assign11650_e10695 + var_sp_ov_a_s);
        let assign11650_e10698: f64 = (assign11650_e10697).sqrt();
        let assign11650_e10699: f64 = (var_gov_s * assign11650_e10698);
        let assign11650_e10700: f64 = (assign11650_e10689 - assign11650_e10699);
        var_sp_ov_delta1_s = assign11650_e10700;
        var_sp_ov_delta1_s_rv = 0.0;

        let assign11660_e10703: f64 = (1.0 / var_gov_d);
        var_inv_gov = assign11660_e10703;
        var_inv_gov_rv = 0.0;

        let assign11670_e10706: f64 = (3.1 * var_gov_d);
        let assign11670_e10708: f64 = (assign11670_e10706 + 8.5);
        var_sp_ov_eps = assign11670_e10708;
        var_sp_ov_eps_rv = 0.0;

        let assign11680_e10711: f64 = (var_sp_ov_eps * var_sp_ov_eps);
        var_sp_ov_eps2_d = assign11680_e10711;
        var_sp_ov_eps2_d_rv = 0.0;

        let assign11690_e10714: f64 = (0.5 * var_sp_ov_eps);
        var_sp_ov_delta = assign11690_e10714;
        var_sp_ov_delta_rv = 0.0;

        let assign11700_e10717: f64 = if var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        var_guard160 = assign11700_e10717;
        var_guard160_rv = 0.0;

        let (assign11710_e10723,) = {
    if (var_guard160 != 0.0) {
        let assign11710_e10721: f64 = (64.0 * var_inv_gov);
        (assign11710_e10721,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11710_e10723;
        var_sp_ov_a_d_rv = 0.0;

        let assign11720_e10726: f64 = if var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        var_guard161 = assign11720_e10726;
        var_guard161_rv = 0.0;

        let (assign11730_e10737,) = {
    if ((var_guard160 == 0.0) && (var_guard161 != 0.0)) {
        let assign11730_e10733: f64 = (22.0 * var_inv_gov);
        let assign11730_e10735: f64 = (assign11730_e10733 + 3.0);
        (assign11730_e10735,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11730_e10737;
        var_sp_ov_a_d_rv = 0.0;

        let assign11740_e10740: f64 = if var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        var_guard162 = assign11740_e10740;
        var_guard162_rv = 0.0;

        let (assign11750_e10755,) = {
    if (((var_guard160 == 0.0) && (var_guard161 == 0.0)) && (var_guard162 != 0.0)) {
        let assign11750_e10749: f64 = (-7.2);
        let assign11750_e10751: f64 = (assign11750_e10749 * var_inv_gov);
        let assign11750_e10753: f64 = (assign11750_e10751 + 15.5);
        (assign11750_e10753,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11750_e10755;
        var_sp_ov_a_d_rv = 0.0;

        let (assign11760_e10766,) = {
    if (((var_guard160 == 0.0) && (var_guard161 == 0.0)) && (var_guard162 == 0.0)) {
        (var_gov_d,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11760_e10766;
        var_sp_ov_a_d_rv = 0.0;

        let assign11770_e10770: f64 = (var_gov2_d * 0.5);
        let assign11770_e10771: f64 = (var_sp_ov_delta + assign11770_e10770);
        let assign11770_e10776: f64 = (var_gov2_d * 0.25);
        let assign11770_e10777: f64 = (var_sp_ov_delta + assign11770_e10776);
        let assign11770_e10779: f64 = (assign11770_e10777 + var_sp_ov_a_d);
        let assign11770_e10780: f64 = (assign11770_e10779).sqrt();
        let assign11770_e10781: f64 = (var_gov_d * assign11770_e10780);
        let assign11770_e10782: f64 = (assign11770_e10771 - assign11770_e10781);
        var_sp_ov_delta1_d = assign11770_e10782;
        var_sp_ov_delta1_d_rv = 0.0;

        let assign11780_e10785: f64 = (var_eg + var_dphib_i);
        let assign11780_e10788: f64 = (2.0 * var_phit);
        let assign11780_e10792: f64 = (-0.75);
        let assign11780_e10793: f64 = (var_phibfac).powf(assign11780_e10792);
        let assign11780_e10794: f64 = (var_neff_i * assign11780_e10793);
        let assign11780_e10796: f64 = (assign11780_e10794 * 4e-26);
        let assign11780_e10797: f64 = (assign11780_e10796).ln();
        let assign11780_e10798: f64 = (assign11780_e10788 * assign11780_e10797);
        let assign11780_e10799: f64 = (assign11780_e10785 + assign11780_e10798);
        var_phib_dc = assign11780_e10799;
        var_phib_dc_rv = 0.0;

        let (assign11790_e10805,) = {
    if (var_phib_dc > 0.05) {
        (var_phib_dc,)
    } else {
        (0.05,)
    }
};
        var_phib_dc = assign11790_e10805;
        var_phib_dc_rv = 0.0;

        let assign11800_e10808: f64 = (2.0 * 1.6021918e-19);
        let assign11800_e10810: f64 = (assign11800_e10808 * var_neff_i);
        let assign11800_e10812: f64 = (assign11800_e10810 * var_epssi);
        let assign11800_e10814: f64 = (assign11800_e10812 * var_inv_phit);
        let assign11800_e10815: f64 = (assign11800_e10814).sqrt();
        let assign11800_e10817: f64 = (assign11800_e10815 / var_coxprime);
        var_g_0_dc = assign11800_e10817;
        var_g_0_dc_rv = 0.0;

        var_kp = 0.0;
        var_kp_rv = 0.0;

        var_np = 0.0;
        var_np_rv = 0.0;

        let assign11830_e10822: f64 = if var_np_i > 0.0 { 1.0 } else { 0.0 };
        var_guard163 = assign11830_e10822;
        var_guard163_rv = 0.0;

        let (assign11840_e10828,) = {
    if (var_guard163 != 0.0) {
        let assign11840_e10826: f64 = (80000000.0 / var_tox_sq);
        (assign11840_e10826,)
    } else {
        (var_arg2max,)
    }
};
        var_arg2max = assign11840_e10828;
        var_arg2max_rv = 0.0;

        let (assign11850_e10837,) = {
    if (var_guard163 != 0.0) {
        let (assign11850_e10835,) = {
            if (var_np_i > var_arg2max) {
                (var_np_i,)
            } else {
                (var_arg2max,)
            }
        };
        (assign11850_e10835,)
    } else {
        (var_np,)
    }
};
        var_np = assign11850_e10837;
        var_np_rv = 0.0;

        let (assign11860_e10846,) = {
    if (var_guard163 != 0.0) {
        let (assign11860_e10844,) = {
            if (5e24 > var_np) {
                (5e24,)
            } else {
                (var_np,)
            }
        };
        (assign11860_e10844,)
    } else {
        (var_np,)
    }
};
        var_np = assign11860_e10846;
        var_np_rv = 0.0;

        let (assign11870_e10862,) = {
    if (var_guard163 != 0.0) {
        let assign11870_e10850: f64 = (2.0 * var_coxprime);
        let assign11870_e10852: f64 = (assign11870_e10850 * var_coxprime);
        let assign11870_e10854: f64 = (assign11870_e10852 * var_phit);
        let assign11870_e10857: f64 = (1.6021918e-19 * var_np);
        let assign11870_e10859: f64 = (assign11870_e10857 * var_epssi);
        let assign11870_e10860: f64 = (assign11870_e10854 / assign11870_e10859);
        (assign11870_e10860,)
    } else {
        (var_kp,)
    }
};
        var_kp = assign11870_e10862;
        var_kp_rv = 0.0;

        let assign11880_e10865: f64 = (100.0 * var_phit);
        let assign11880_e10867: f64 = (assign11880_e10865 * var_phit);
        var_qlim2 = assign11880_e10867;
        var_qlim2_rv = 0.0;

        let assign11890_e10870: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        var_guard164 = assign11890_e10870;
        var_guard164_rv = 0.0;

        let (assign11900_e10881,) = {
    if (var_guard164 != 0.0) {
        let assign11900_e10874: f64 = (var_phit * var_g_0_dc);
        let assign11900_e10876: f64 = (assign11900_e10874 * var_g_0_dc);
        let assign11900_e10878: f64 = (assign11900_e10876 * var_phib_dc);
        let assign11900_e10879: f64 = (assign11900_e10878).sqrt();
        (assign11900_e10879,)
    } else {
        (var_qb0,)
    }
};
        var_qb0 = assign11900_e10881;
        var_qb0_rv = 0.0;

        let (assign11910_e10891,) = {
    if (var_guard164 != 0.0) {
        let assign11910_e10885: f64 = (0.75 * var_qq);
        let assign11910_e10888: f64 = (var_qb0).powf(0.6666666666666666);
        let assign11910_e10889: f64 = (assign11910_e10885 * assign11910_e10888);
        (assign11910_e10889,)
    } else {
        (var_dphibq,)
    }
};
        var_dphibq = assign11910_e10891;
        var_dphibq_rv = 0.0;

        let (assign11920_e10897,) = {
    if (var_guard164 != 0.0) {
        let assign11920_e10895: f64 = (var_phib_dc + var_dphibq);
        (assign11920_e10895,)
    } else {
        (var_phib_dc,)
    }
};
        var_phib_dc = assign11920_e10897;
        var_phib_dc_rv = 0.0;

        let (assign11930_e10911,) = {
    if (var_guard164 != 0.0) {
        let assign11930_e10903: f64 = (2.0 * 0.6666666666666666);
        let assign11930_e10905: f64 = (assign11930_e10903 * var_dphibq);
        let assign11930_e10907: f64 = (assign11930_e10905 / var_qb0);
        let assign11930_e10908: f64 = (1.0 + assign11930_e10907);
        let assign11930_e10909: f64 = (var_g_0_dc * assign11930_e10908);
        (assign11930_e10909,)
    } else {
        (var_g_0_dc,)
    }
};
        var_g_0_dc = assign11930_e10911;
        var_g_0_dc_rv = 0.0;

        let assign11940_e10913: f64 = (var_phib_dc).sqrt();
        var_sqrt_phib_dc = assign11940_e10913;
        var_sqrt_phib_dc_rv = 0.0;

        let assign11950_e10916: f64 = (0.95 * var_phib_dc);
        var_phix_dc = assign11950_e10916;
        var_phix_dc_rv = 0.0;

        let assign11960_e10919: f64 = (0.0025 * var_phib_dc);
        let assign11960_e10921: f64 = (assign11960_e10919 * var_phib_dc);
        var_aphi_dc = assign11960_e10921;
        var_aphi_dc_rv = 0.0;

        var_bphi_dc = var_aphi_dc;
        var_bphi_dc_rv = 0.0;

        let assign11980_e10925: f64 = (var_bphi_dc).sqrt();
        let assign11980_e10926: f64 = (0.5 * assign11980_e10925);
        var_phix2 = assign11980_e10926;
        var_phix2_rv = 0.0;

        let assign11990_e10930: f64 = (var_phix_dc - var_phix2);
        let assign11990_e10932: f64 = assign11990_e10930;
        let assign11990_e10935: f64 = (var_phix_dc - var_phix2);
        let assign11990_e10937: f64 = assign11990_e10935;
        let assign11990_e10940: f64 = (var_phix_dc - var_phix2);
        let assign11990_e10942: f64 = assign11990_e10940;
        let assign11990_e10943: f64 = (assign11990_e10937 * assign11990_e10942);
        let assign11990_e10945: f64 = (assign11990_e10943 + var_aphi_dc);
        let assign11990_e10946: f64 = (assign11990_e10945).sqrt();
        let assign11990_e10947: f64 = (assign11990_e10932 - assign11990_e10946);
        let assign11990_e10948: f64 = (0.5 * assign11990_e10947);
        var_phix1_dc = assign11990_e10948;
        var_phix1_dc_rv = 0.0;

        let assign12000_e10952: f64 = (var_phib_dc + var_eg);
        let assign12000_e10953: f64 = (0.5 * assign12000_e10952);
        var_alpha_b = assign12000_e10953;
        var_alpha_b_rv = 0.0;

        let assign12010_e10956: f64 = (var_vsbnud_i + var_phib_dc);
        let assign12010_e10957: f64 = (assign12010_e10956).sqrt();
        let assign12010_e10959: f64 = (assign12010_e10957 - var_sqrt_phib_dc);
        var_us1 = assign12010_e10959;
        var_us1_rv = 0.0;

        let assign12020_e10962: f64 = (var_vsbnud_i + var_dvsbnud_i);
        let assign12020_e10964: f64 = (assign12020_e10962 + var_phib_dc);
        let assign12020_e10965: f64 = (assign12020_e10964).sqrt();
        let assign12020_e10967: f64 = (assign12020_e10965 - var_sqrt_phib_dc);
        let assign12020_e10969: f64 = (assign12020_e10967 - var_us1);
        var_us21 = assign12020_e10969;
        var_us21_rv = 0.0;

        let assign12030_e10972: f64 = (var_eg + var_dphib_i);
        let assign12030_e10974: f64 = (assign12030_e10972 + var_delvtac_i);
        let assign12030_e10977: f64 = (2.0 * var_phit);
        let assign12030_e10981: f64 = (-0.75);
        let assign12030_e10982: f64 = (var_phibfac).powf(assign12030_e10981);
        let assign12030_e10983: f64 = (var_neffac_i * assign12030_e10982);
        let assign12030_e10985: f64 = (assign12030_e10983 * 4e-26);
        let assign12030_e10986: f64 = (assign12030_e10985).ln();
        let assign12030_e10987: f64 = (assign12030_e10977 * assign12030_e10986);
        let assign12030_e10988: f64 = (assign12030_e10974 + assign12030_e10987);
        var_phib_ac = assign12030_e10988;
        var_phib_ac_rv = 0.0;

        let (assign12040_e10994,) = {
    if (var_phib_ac > 0.05) {
        (var_phib_ac,)
    } else {
        (0.05,)
    }
};
        var_phib_ac = assign12040_e10994;
        var_phib_ac_rv = 0.0;

        let assign12050_e10997: f64 = (2.0 * 1.6021918e-19);
        let assign12050_e10999: f64 = (assign12050_e10997 * var_neffac_i);
        let assign12050_e11001: f64 = (assign12050_e10999 * var_epssi);
        let assign12050_e11003: f64 = (assign12050_e11001 * var_inv_phit);
        let assign12050_e11004: f64 = (assign12050_e11003).sqrt();
        let assign12050_e11006: f64 = (assign12050_e11004 / var_coxprime);
        var_g_0_ac = assign12050_e11006;
        var_g_0_ac_rv = 0.0;

        let assign12060_e11009: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        var_guard165 = assign12060_e11009;
        var_guard165_rv = 0.0;

        let (assign12070_e11020,) = {
    if (var_guard165 != 0.0) {
        let assign12070_e11013: f64 = (var_phit * var_g_0_ac);
        let assign12070_e11015: f64 = (assign12070_e11013 * var_g_0_ac);
        let assign12070_e11017: f64 = (assign12070_e11015 * var_phib_ac);
        let assign12070_e11018: f64 = (assign12070_e11017).sqrt();
        (assign12070_e11018,)
    } else {
        (var_qb0,)
    }
};
        var_qb0 = assign12070_e11020;
        var_qb0_rv = 0.0;

        let (assign12080_e11030,) = {
    if (var_guard165 != 0.0) {
        let assign12080_e11024: f64 = (0.75 * var_qq);
        let assign12080_e11027: f64 = (var_qb0).powf(0.6666666666666666);
        let assign12080_e11028: f64 = (assign12080_e11024 * assign12080_e11027);
        (assign12080_e11028,)
    } else {
        (var_dphibq,)
    }
};
        var_dphibq = assign12080_e11030;
        var_dphibq_rv = 0.0;

        let (assign12090_e11036,) = {
    if (var_guard165 != 0.0) {
        let assign12090_e11034: f64 = (var_phib_ac + var_dphibq);
        (assign12090_e11034,)
    } else {
        (var_phib_ac,)
    }
};
        var_phib_ac = assign12090_e11036;
        var_phib_ac_rv = 0.0;

        let (assign12100_e11050,) = {
    if (var_guard165 != 0.0) {
        let assign12100_e11042: f64 = (2.0 * 0.6666666666666666);
        let assign12100_e11044: f64 = (assign12100_e11042 * var_dphibq);
        let assign12100_e11046: f64 = (assign12100_e11044 / var_qb0);
        let assign12100_e11047: f64 = (1.0 + assign12100_e11046);
        let assign12100_e11048: f64 = (var_g_0_ac * assign12100_e11047);
        (assign12100_e11048,)
    } else {
        (var_g_0_ac,)
    }
};
        var_g_0_ac = assign12100_e11050;
        var_g_0_ac_rv = 0.0;

        let assign12110_e11053: f64 = (0.95 * var_phib_ac);
        var_phix_ac = assign12110_e11053;
        var_phix_ac_rv = 0.0;

        let assign12120_e11056: f64 = (0.0025 * var_phib_ac);
        let assign12120_e11058: f64 = (assign12120_e11056 * var_phib_ac);
        var_aphi_ac = assign12120_e11058;
        var_aphi_ac_rv = 0.0;

        var_bphi_ac = var_aphi_ac;
        var_bphi_ac_rv = 0.0;

        let assign12140_e11062: f64 = (var_bphi_ac).sqrt();
        let assign12140_e11063: f64 = (0.5 * assign12140_e11062);
        var_phix2 = assign12140_e11063;
        var_phix2_rv = 0.0;

        let assign12150_e11067: f64 = (var_phix_ac - var_phix2);
        let assign12150_e11069: f64 = assign12150_e11067;
        let assign12150_e11072: f64 = (var_phix_ac - var_phix2);
        let assign12150_e11074: f64 = assign12150_e11072;
        let assign12150_e11077: f64 = (var_phix_ac - var_phix2);
        let assign12150_e11079: f64 = assign12150_e11077;
        let assign12150_e11080: f64 = (assign12150_e11074 * assign12150_e11079);
        let assign12150_e11082: f64 = (assign12150_e11080 + var_aphi_ac);
        let assign12150_e11083: f64 = (assign12150_e11082).sqrt();
        let assign12150_e11084: f64 = (assign12150_e11069 - assign12150_e11083);
        let assign12150_e11085: f64 = (0.5 * assign12150_e11084);
        var_phix1_ac = assign12150_e11085;
        var_phix1_ac_rv = 0.0;

        let assign12160_e11089: f64 = (var_stvfb_i * var_delt);
        let assign12160_e11093: f64 = (var_st2vfb_i * var_delt);
        let assign12160_e11094: f64 = (1.0 + assign12160_e11093);
        let assign12160_e11095: f64 = (assign12160_e11089 * assign12160_e11094);
        let assign12160_e11096: f64 = (var_vfb_i + assign12160_e11095);
        let assign12160_e11098: f64 = (assign12160_e11096 + var_delvto_i);
        var_vfb_t = assign12160_e11098;
        var_vfb_t_rv = 0.0;

        let assign12170_e11101: f64 = (var_stct_i * var_ln_rtn);
        let assign12170_e11102: f64 = (assign12170_e11101).exp();
        var_tf_ct = assign12170_e11102;
        var_tf_ct_rv = 0.0;

        let assign12180_e11105: f64 = (var_ct_i * var_tf_ct);
        var_ct_t = assign12180_e11105;
        var_ct_t_rv = 0.0;

        let assign12190_e11108: f64 = (var_ctg_i / var_rtn);
        var_ctg_t = assign12190_e11108;
        var_ctg_t_rv = 0.0;

        let assign12200_e11111: f64 = (var_stbet_i * var_ln_rtn);
        let assign12200_e11112: f64 = (assign12200_e11111).exp();
        var_tf_bet = assign12200_e11112;
        var_tf_bet_rv = 0.0;

        let assign12210_e11115: f64 = (var_betn_i * var_tf_bet);
        var_betn_t = assign12210_e11115;
        var_betn_t_rv = 0.0;

        let assign12220_e11118: f64 = (var_factuo_i * var_betn_t);
        let assign12220_e11120: f64 = (assign12220_e11118 * var_coxprime);
        var_bet_i = assign12220_e11120;
        var_bet_i_rv = 0.0;

        let assign12230_e11124: f64 = (var_stthemu_i * var_ln_rtn);
        let assign12230_e11125: f64 = (assign12230_e11124).exp();
        let assign12230_e11126: f64 = (var_themu_i * assign12230_e11125);
        var_themu_t = assign12230_e11126;
        var_themu_t_rv = 0.0;

        let assign12240_e11129: f64 = (var_stmue_i * var_ln_rtn);
        let assign12240_e11130: f64 = (assign12240_e11129).exp();
        var_tf_mue = assign12240_e11130;
        var_tf_mue_rv = 0.0;

        let assign12250_e11133: f64 = (var_mue_i * var_tf_mue);
        var_mue_t = assign12250_e11133;
        var_mue_t_rv = 0.0;

        let assign12260_e11137: f64 = (var_stthecs_i * var_ln_rtn);
        let assign12260_e11138: f64 = (assign12260_e11137).exp();
        let assign12260_e11139: f64 = (var_thecs_i * assign12260_e11138);
        var_thecs_t = assign12260_e11139;
        var_thecs_t_rv = 0.0;

        let assign12270_e11142: f64 = (var_stcs_i * var_ln_rtn);
        let assign12270_e11143: f64 = (assign12270_e11142).exp();
        var_tf_cs = assign12270_e11143;
        var_tf_cs_rv = 0.0;

        let assign12280_e11146: f64 = (var_cs_i * var_tf_cs);
        var_cs_t = assign12280_e11146;
        var_cs_t_rv = 0.0;

        *var_alpha_b_slot = var_alpha_b;
        *var_alpha_b_rv_slot = var_alpha_b_rv;
        *var_aphi_ac_slot = var_aphi_ac;
        *var_aphi_ac_rv_slot = var_aphi_ac_rv;
        *var_aphi_dc_slot = var_aphi_dc;
        *var_aphi_dc_rv_slot = var_aphi_dc_rv;
        *var_arg2max_slot = var_arg2max;
        *var_arg2max_rv_slot = var_arg2max_rv;
        *var_bet_i_slot = var_bet_i;
        *var_bet_i_rv_slot = var_bet_i_rv;
        *var_betn_t_slot = var_betn_t;
        *var_betn_t_rv_slot = var_betn_t_rv;
        *var_bphi_ac_slot = var_bphi_ac;
        *var_bphi_ac_rv_slot = var_bphi_ac_rv;
        *var_bphi_dc_slot = var_bphi_dc;
        *var_bphi_dc_rv_slot = var_bphi_dc_rv;
        *var_cs_t_slot = var_cs_t;
        *var_cs_t_rv_slot = var_cs_t_rv;
        *var_ct_t_slot = var_ct_t;
        *var_ct_t_rv_slot = var_ct_t_rv;
        *var_ctg_t_slot = var_ctg_t;
        *var_ctg_t_rv_slot = var_ctg_t_rv;
        *var_dphibq_slot = var_dphibq;
        *var_dphibq_rv_slot = var_dphibq_rv;
        *var_g_0_ac_slot = var_g_0_ac;
        *var_g_0_ac_rv_slot = var_g_0_ac_rv;
        *var_g_0_dc_slot = var_g_0_dc;
        *var_g_0_dc_rv_slot = var_g_0_dc_rv;
        *var_guard160_slot = var_guard160;
        *var_guard160_rv_slot = var_guard160_rv;
        *var_guard161_slot = var_guard161;
        *var_guard161_rv_slot = var_guard161_rv;
        *var_guard162_slot = var_guard162;
        *var_guard162_rv_slot = var_guard162_rv;
        *var_guard163_slot = var_guard163;
        *var_guard163_rv_slot = var_guard163_rv;
        *var_guard164_slot = var_guard164;
        *var_guard164_rv_slot = var_guard164_rv;
        *var_guard165_slot = var_guard165;
        *var_guard165_rv_slot = var_guard165_rv;
        *var_inv_gov_slot = var_inv_gov;
        *var_inv_gov_rv_slot = var_inv_gov_rv;
        *var_kp_slot = var_kp;
        *var_kp_rv_slot = var_kp_rv;
        *var_mue_t_slot = var_mue_t;
        *var_mue_t_rv_slot = var_mue_t_rv;
        *var_np_slot = var_np;
        *var_np_rv_slot = var_np_rv;
        *var_phib_ac_slot = var_phib_ac;
        *var_phib_ac_rv_slot = var_phib_ac_rv;
        *var_phib_dc_slot = var_phib_dc;
        *var_phib_dc_rv_slot = var_phib_dc_rv;
        *var_phix1_ac_slot = var_phix1_ac;
        *var_phix1_ac_rv_slot = var_phix1_ac_rv;
        *var_phix1_dc_slot = var_phix1_dc;
        *var_phix1_dc_rv_slot = var_phix1_dc_rv;
        *var_phix2_slot = var_phix2;
        *var_phix2_rv_slot = var_phix2_rv;
        *var_phix_ac_slot = var_phix_ac;
        *var_phix_ac_rv_slot = var_phix_ac_rv;
        *var_phix_dc_slot = var_phix_dc;
        *var_phix_dc_rv_slot = var_phix_dc_rv;
        *var_qb0_slot = var_qb0;
        *var_qb0_rv_slot = var_qb0_rv;
        *var_qlim2_slot = var_qlim2;
        *var_qlim2_rv_slot = var_qlim2_rv;
        *var_sp_ov_a_d_slot = var_sp_ov_a_d;
        *var_sp_ov_a_d_rv_slot = var_sp_ov_a_d_rv;
        *var_sp_ov_a_s_slot = var_sp_ov_a_s;
        *var_sp_ov_a_s_rv_slot = var_sp_ov_a_s_rv;
        *var_sp_ov_delta_slot = var_sp_ov_delta;
        *var_sp_ov_delta1_d_slot = var_sp_ov_delta1_d;
        *var_sp_ov_delta1_d_rv_slot = var_sp_ov_delta1_d_rv;
        *var_sp_ov_delta1_s_slot = var_sp_ov_delta1_s;
        *var_sp_ov_delta1_s_rv_slot = var_sp_ov_delta1_s_rv;
        *var_sp_ov_delta_rv_slot = var_sp_ov_delta_rv;
        *var_sp_ov_eps_slot = var_sp_ov_eps;
        *var_sp_ov_eps2_d_slot = var_sp_ov_eps2_d;
        *var_sp_ov_eps2_d_rv_slot = var_sp_ov_eps2_d_rv;
        *var_sp_ov_eps_rv_slot = var_sp_ov_eps_rv;
        *var_sqrt_phib_dc_slot = var_sqrt_phib_dc;
        *var_sqrt_phib_dc_rv_slot = var_sqrt_phib_dc_rv;
        *var_tf_bet_slot = var_tf_bet;
        *var_tf_bet_rv_slot = var_tf_bet_rv;
        *var_tf_cs_slot = var_tf_cs;
        *var_tf_cs_rv_slot = var_tf_cs_rv;
        *var_tf_ct_slot = var_tf_ct;
        *var_tf_ct_rv_slot = var_tf_ct_rv;
        *var_tf_mue_slot = var_tf_mue;
        *var_tf_mue_rv_slot = var_tf_mue_rv;
        *var_thecs_t_slot = var_thecs_t;
        *var_thecs_t_rv_slot = var_thecs_t_rv;
        *var_themu_t_slot = var_themu_t;
        *var_themu_t_rv_slot = var_themu_t_rv;
        *var_us1_slot = var_us1;
        *var_us1_rv_slot = var_us1_rv;
        *var_us21_slot = var_us21;
        *var_us21_rv_slot = var_us21_rv;
        *var_vfb_t_slot = var_vfb_t;
        *var_vfb_t_rv_slot = var_vfb_t_rv;
    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        var_a2_i: f64,
        var_bet_i: f64,
        var_betnedge_i: f64,
        var_bgidl_i: f64,
        var_chib_i: f64,
        var_coxprime: f64,
        var_ctedge_i: f64,
        var_delt: f64,
        var_delta: f64,
        var_delvtoedge_i: f64,
        var_dphibedge_i: f64,
        var_eg: f64,
        var_epssi: f64,
        var_factuoedge_i: f64,
        var_fnt_i: f64,
        var_gc2_i: f64,
        var_gc2ov_i: f64,
        var_gc2ovd_i: f64,
        var_gc3_i: f64,
        var_gc3ov_i: f64,
        var_gc3ovd_i: f64,
        var_inv_phit: f64,
        var_ln_rtn: f64,
        var_neffedge_i: f64,
        var_phibfac: f64,
        var_phit: f64,
        var_rs_i: f64,
        var_rta: f64,
        var_rtn: f64,
        var_sta2_i: f64,
        var_stbetedge_i: f64,
        var_stbgidl_i: f64,
        var_stig_i: f64,
        var_strs_i: f64,
        var_stthesat_i: f64,
        var_stvfbedge_i: f64,
        var_stxcor_i: f64,
        var_thesat_i: f64,
        var_thesatac_i: f64,
        var_tkd: f64,
        var_tox_i: f64,
        var_toxov_i: f64,
        var_toxovd_i: f64,
        var_vfbedge_i: f64,
        var_xcor_i: f64,
        var_a2_t_slot: &mut f64,
        var_a2_t_rv_slot: &mut f64,
        var_aphiedge_slot: &mut f64,
        var_aphiedge_rv_slot: &mut f64,
        var_b_fact_slot: &mut f64,
        var_b_fact_rv_slot: &mut f64,
        var_bch_slot: &mut f64,
        var_bch_rv_slot: &mut f64,
        var_betedge_i_slot: &mut f64,
        var_betedge_i_rv_slot: &mut f64,
        var_betnedge_t_slot: &mut f64,
        var_betnedge_t_rv_slot: &mut f64,
        var_bgidl_t_slot: &mut f64,
        var_bgidl_t_rv_slot: &mut f64,
        var_bov_slot: &mut f64,
        var_bov_d_slot: &mut f64,
        var_bov_d_rv_slot: &mut f64,
        var_bov_rv_slot: &mut f64,
        var_bphiedge_slot: &mut f64,
        var_bphiedge_rv_slot: &mut f64,
        var_gcq_slot: &mut f64,
        var_gcq_rv_slot: &mut f64,
        var_gcqov_slot: &mut f64,
        var_gcqov_rv_slot: &mut f64,
        var_gcqovd_slot: &mut f64,
        var_gcqovd_rv_slot: &mut f64,
        var_gfedge_slot: &mut f64,
        var_gfedge2_slot: &mut f64,
        var_gfedge2_rv_slot: &mut f64,
        var_gfedge_rv_slot: &mut f64,
        var_guard166_slot: &mut f64,
        var_guard166_rv_slot: &mut f64,
        var_guard167_slot: &mut f64,
        var_guard167_rv_slot: &mut f64,
        var_guard168_slot: &mut f64,
        var_guard168_rv_slot: &mut f64,
        var_guard169_slot: &mut f64,
        var_guard169_rv_slot: &mut f64,
        var_iginv_i_slot: &mut f64,
        var_iginv_i_rv_slot: &mut f64,
        var_igov_i_slot: &mut f64,
        var_igov_i_rv_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_igovd_i_rv_slot: &mut f64,
        var_inv_chib_slot: &mut f64,
        var_inv_chib_rv_slot: &mut f64,
        var_lngfedge2_slot: &mut f64,
        var_lngfedge2_rv_slot: &mut f64,
        var_nt_slot: &mut f64,
        var_nt_rv_slot: &mut f64,
        var_phibedge_slot: &mut f64,
        var_phibedge_rv_slot: &mut f64,
        var_phit0edge_slot: &mut f64,
        var_phit0edge_rv_slot: &mut f64,
        var_phix1edge_slot: &mut f64,
        var_phix1edge_rv_slot: &mut f64,
        var_phix2edge_slot: &mut f64,
        var_phix2edge_rv_slot: &mut f64,
        var_phixedge_slot: &mut f64,
        var_phixedge_rv_slot: &mut f64,
        var_rs_t_slot: &mut f64,
        var_rs_t_rv_slot: &mut f64,
        var_tf_betedge_slot: &mut f64,
        var_tf_betedge_rv_slot: &mut f64,
        var_tf_ig_slot: &mut f64,
        var_tf_ig_rv_slot: &mut f64,
        var_tf_ther_slot: &mut f64,
        var_tf_ther_rv_slot: &mut f64,
        var_tf_thesat_slot: &mut f64,
        var_tf_thesat_rv_slot: &mut f64,
        var_tf_xcor_slot: &mut f64,
        var_tf_xcor_rv_slot: &mut f64,
        var_ther_i_slot: &mut f64,
        var_ther_i_rv_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesat_t_rv_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_rv_slot: &mut f64,
        var_vfbedge_t_slot: &mut f64,
        var_vfbedge_t_rv_slot: &mut f64,
        var_xcor_t_slot: &mut f64,
        var_xcor_t_rv_slot: &mut f64,
    ) {
        let mut var_a2_t: f64 = *var_a2_t_slot;
        let mut var_a2_t_rv: f64 = *var_a2_t_rv_slot;
        let mut var_aphiedge: f64 = *var_aphiedge_slot;
        let mut var_aphiedge_rv: f64 = *var_aphiedge_rv_slot;
        let mut var_b_fact: f64 = *var_b_fact_slot;
        let mut var_b_fact_rv: f64 = *var_b_fact_rv_slot;
        let mut var_bch: f64 = *var_bch_slot;
        let mut var_bch_rv: f64 = *var_bch_rv_slot;
        let mut var_betedge_i: f64 = *var_betedge_i_slot;
        let mut var_betedge_i_rv: f64 = *var_betedge_i_rv_slot;
        let mut var_betnedge_t: f64 = *var_betnedge_t_slot;
        let mut var_betnedge_t_rv: f64 = *var_betnedge_t_rv_slot;
        let mut var_bgidl_t: f64 = *var_bgidl_t_slot;
        let mut var_bgidl_t_rv: f64 = *var_bgidl_t_rv_slot;
        let mut var_bov: f64 = *var_bov_slot;
        let mut var_bov_d: f64 = *var_bov_d_slot;
        let mut var_bov_d_rv: f64 = *var_bov_d_rv_slot;
        let mut var_bov_rv: f64 = *var_bov_rv_slot;
        let mut var_bphiedge: f64 = *var_bphiedge_slot;
        let mut var_bphiedge_rv: f64 = *var_bphiedge_rv_slot;
        let mut var_gcq: f64 = *var_gcq_slot;
        let mut var_gcq_rv: f64 = *var_gcq_rv_slot;
        let mut var_gcqov: f64 = *var_gcqov_slot;
        let mut var_gcqov_rv: f64 = *var_gcqov_rv_slot;
        let mut var_gcqovd: f64 = *var_gcqovd_slot;
        let mut var_gcqovd_rv: f64 = *var_gcqovd_rv_slot;
        let mut var_gfedge: f64 = *var_gfedge_slot;
        let mut var_gfedge2: f64 = *var_gfedge2_slot;
        let mut var_gfedge2_rv: f64 = *var_gfedge2_rv_slot;
        let mut var_gfedge_rv: f64 = *var_gfedge_rv_slot;
        let mut var_guard166: f64 = *var_guard166_slot;
        let mut var_guard166_rv: f64 = *var_guard166_rv_slot;
        let mut var_guard167: f64 = *var_guard167_slot;
        let mut var_guard167_rv: f64 = *var_guard167_rv_slot;
        let mut var_guard168: f64 = *var_guard168_slot;
        let mut var_guard168_rv: f64 = *var_guard168_rv_slot;
        let mut var_guard169: f64 = *var_guard169_slot;
        let mut var_guard169_rv: f64 = *var_guard169_rv_slot;
        let mut var_iginv_i: f64 = *var_iginv_i_slot;
        let mut var_iginv_i_rv: f64 = *var_iginv_i_rv_slot;
        let mut var_igov_i: f64 = *var_igov_i_slot;
        let mut var_igov_i_rv: f64 = *var_igov_i_rv_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_igovd_i_rv: f64 = *var_igovd_i_rv_slot;
        let mut var_inv_chib: f64 = *var_inv_chib_slot;
        let mut var_inv_chib_rv: f64 = *var_inv_chib_rv_slot;
        let mut var_lngfedge2: f64 = *var_lngfedge2_slot;
        let mut var_lngfedge2_rv: f64 = *var_lngfedge2_rv_slot;
        let mut var_nt: f64 = *var_nt_slot;
        let mut var_nt_rv: f64 = *var_nt_rv_slot;
        let mut var_phibedge: f64 = *var_phibedge_slot;
        let mut var_phibedge_rv: f64 = *var_phibedge_rv_slot;
        let mut var_phit0edge: f64 = *var_phit0edge_slot;
        let mut var_phit0edge_rv: f64 = *var_phit0edge_rv_slot;
        let mut var_phix1edge: f64 = *var_phix1edge_slot;
        let mut var_phix1edge_rv: f64 = *var_phix1edge_rv_slot;
        let mut var_phix2edge: f64 = *var_phix2edge_slot;
        let mut var_phix2edge_rv: f64 = *var_phix2edge_rv_slot;
        let mut var_phixedge: f64 = *var_phixedge_slot;
        let mut var_phixedge_rv: f64 = *var_phixedge_rv_slot;
        let mut var_rs_t: f64 = *var_rs_t_slot;
        let mut var_rs_t_rv: f64 = *var_rs_t_rv_slot;
        let mut var_tf_betedge: f64 = *var_tf_betedge_slot;
        let mut var_tf_betedge_rv: f64 = *var_tf_betedge_rv_slot;
        let mut var_tf_ig: f64 = *var_tf_ig_slot;
        let mut var_tf_ig_rv: f64 = *var_tf_ig_rv_slot;
        let mut var_tf_ther: f64 = *var_tf_ther_slot;
        let mut var_tf_ther_rv: f64 = *var_tf_ther_rv_slot;
        let mut var_tf_thesat: f64 = *var_tf_thesat_slot;
        let mut var_tf_thesat_rv: f64 = *var_tf_thesat_rv_slot;
        let mut var_tf_xcor: f64 = *var_tf_xcor_slot;
        let mut var_tf_xcor_rv: f64 = *var_tf_xcor_rv_slot;
        let mut var_ther_i: f64 = *var_ther_i_slot;
        let mut var_ther_i_rv: f64 = *var_ther_i_rv_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesat_t_rv: f64 = *var_thesat_t_rv_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_rv: f64 = *var_thesatac_t_rv_slot;
        let mut var_vfbedge_t: f64 = *var_vfbedge_t_slot;
        let mut var_vfbedge_t_rv: f64 = *var_vfbedge_t_rv_slot;
        let mut var_xcor_t: f64 = *var_xcor_t_slot;
        let mut var_xcor_t_rv: f64 = *var_xcor_t_rv_slot;

        let assign12290_e11149: f64 = (var_stxcor_i * var_ln_rtn);
        let assign12290_e11150: f64 = (assign12290_e11149).exp();
        var_tf_xcor = assign12290_e11150;
        var_tf_xcor_rv = 0.0;

        let assign12300_e11153: f64 = (var_xcor_i * var_tf_xcor);
        var_xcor_t = assign12300_e11153;
        var_xcor_t_rv = 0.0;

        let assign12310_e11156: f64 = (var_strs_i * var_ln_rtn);
        let assign12310_e11157: f64 = (assign12310_e11156).exp();
        var_tf_ther = assign12310_e11157;
        var_tf_ther_rv = 0.0;

        let assign12320_e11160: f64 = (var_rs_i * var_tf_ther);
        var_rs_t = assign12320_e11160;
        var_rs_t_rv = 0.0;

        let assign12330_e11163: f64 = (2.0 * var_bet_i);
        let assign12330_e11165: f64 = (assign12330_e11163 * var_rs_t);
        var_ther_i = assign12330_e11165;
        var_ther_i_rv = 0.0;

        let assign12340_e11168: f64 = (var_stthesat_i * var_ln_rtn);
        let assign12340_e11169: f64 = (assign12340_e11168).exp();
        var_tf_thesat = assign12340_e11169;
        var_tf_thesat_rv = 0.0;

        let assign12350_e11172: f64 = (var_thesat_i * var_tf_thesat);
        var_thesat_t = assign12350_e11172;
        var_thesat_t_rv = 0.0;

        let assign12360_e11175: f64 = (var_thesatac_i * var_tf_thesat);
        var_thesatac_t = assign12360_e11175;
        var_thesatac_t_rv = 0.0;

        let assign12370_e11178: f64 = (-var_sta2_i);
        let assign12370_e11180: f64 = (assign12370_e11178 * var_ln_rtn);
        let assign12370_e11181: f64 = (assign12370_e11180).exp();
        let assign12370_e11182: f64 = (var_a2_i * assign12370_e11181);
        var_a2_t = assign12370_e11182;
        var_a2_t_rv = 0.0;

        let assign12380_e11185: f64 = (var_fnt_i * 4.0);
        let assign12380_e11187: f64 = (assign12380_e11185 * 1.3806505e-23);
        let assign12380_e11189: f64 = (assign12380_e11187 * var_tkd);
        var_nt = assign12380_e11189;
        var_nt_rv = 0.0;

        let assign12400_e11203: f64 = if ((p.p46 != 0.0) && (var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard166 = assign12400_e11203;
        var_guard166_rv = 0.0;

        let (assign12410_e11213,) = {
    if (var_guard166 != 0.0) {
        let assign12410_e11208: f64 = (var_stvfbedge_i * var_delt);
        let assign12410_e11209: f64 = (var_vfbedge_i + assign12410_e11208);
        let assign12410_e11211: f64 = (assign12410_e11209 + var_delvtoedge_i);
        (assign12410_e11211,)
    } else {
        (var_vfbedge_t,)
    }
};
        var_vfbedge_t = assign12410_e11213;
        var_vfbedge_t_rv = 0.0;

        let (assign12420_e11220,) = {
    if (var_guard166 != 0.0) {
        let assign12420_e11217: f64 = (var_stbetedge_i * var_ln_rtn);
        let assign12420_e11218: f64 = (assign12420_e11217).exp();
        (assign12420_e11218,)
    } else {
        (var_tf_betedge,)
    }
};
        var_tf_betedge = assign12420_e11220;
        var_tf_betedge_rv = 0.0;

        let (assign12430_e11226,) = {
    if (var_guard166 != 0.0) {
        let assign12430_e11224: f64 = (var_betnedge_i * var_tf_betedge);
        (assign12430_e11224,)
    } else {
        (var_betnedge_t,)
    }
};
        var_betnedge_t = assign12430_e11226;
        var_betnedge_t_rv = 0.0;

        let (assign12440_e11234,) = {
    if (var_guard166 != 0.0) {
        let assign12440_e11230: f64 = (var_factuoedge_i * var_betnedge_t);
        let assign12440_e11232: f64 = (assign12440_e11230 * var_coxprime);
        (assign12440_e11232,)
    } else {
        (var_betedge_i,)
    }
};
        var_betedge_i = assign12440_e11234;
        var_betedge_i_rv = 0.0;

        let (assign12450_e11244,) = {
    if (var_guard166 != 0.0) {
        let assign12450_e11240: f64 = (var_ctedge_i * var_rtn);
        let assign12450_e11241: f64 = (1.0 + assign12450_e11240);
        let assign12450_e11242: f64 = (var_phit * assign12450_e11241);
        (assign12450_e11242,)
    } else {
        (var_phit0edge,)
    }
};
        var_phit0edge = assign12450_e11244;
        var_phit0edge_rv = 0.0;

        let (assign12460_e11264,) = {
    if (var_guard166 != 0.0) {
        let assign12460_e11248: f64 = (var_eg + var_dphibedge_i);
        let assign12460_e11251: f64 = (2.0 * var_phit0edge);
        let assign12460_e11255: f64 = (-0.75);
        let assign12460_e11256: f64 = (var_phibfac).powf(assign12460_e11255);
        let assign12460_e11257: f64 = (var_neffedge_i * assign12460_e11256);
        let assign12460_e11259: f64 = (assign12460_e11257 * 4e-26);
        let assign12460_e11260: f64 = (assign12460_e11259).ln();
        let assign12460_e11261: f64 = (assign12460_e11251 * assign12460_e11260);
        let assign12460_e11262: f64 = (assign12460_e11248 + assign12460_e11261);
        (assign12460_e11262,)
    } else {
        (var_phibedge,)
    }
};
        var_phibedge = assign12460_e11264;
        var_phibedge_rv = 0.0;

        let (assign12470_e11273,) = {
    if (var_guard166 != 0.0) {
        let (assign12470_e11271,) = {
            if (var_phibedge > 0.05) {
                (var_phibedge,)
            } else {
                (0.05,)
            }
        };
        (assign12470_e11271,)
    } else {
        (var_phibedge,)
    }
};
        var_phibedge = assign12470_e11273;
        var_phibedge_rv = 0.0;

        let (assign12480_e11288,) = {
    if (var_guard166 != 0.0) {
        let assign12480_e11277: f64 = (2.0 * 1.6021918e-19);
        let assign12480_e11279: f64 = (assign12480_e11277 * var_neffedge_i);
        let assign12480_e11281: f64 = (assign12480_e11279 * var_epssi);
        let assign12480_e11283: f64 = (assign12480_e11281 * var_inv_phit);
        let assign12480_e11284: f64 = (assign12480_e11283).sqrt();
        let assign12480_e11286: f64 = (assign12480_e11284 / var_coxprime);
        (assign12480_e11286,)
    } else {
        (var_gfedge,)
    }
};
        var_gfedge = assign12480_e11288;
        var_gfedge_rv = 0.0;

        let (assign12490_e11294,) = {
    if (var_guard166 != 0.0) {
        let assign12490_e11292: f64 = (var_gfedge * var_gfedge);
        (assign12490_e11292,)
    } else {
        (var_gfedge2,)
    }
};
        var_gfedge2 = assign12490_e11294;
        var_gfedge2_rv = 0.0;

        let (assign12500_e11299,) = {
    if (var_guard166 != 0.0) {
        let assign12500_e11297: f64 = (var_gfedge2).ln();
        (assign12500_e11297,)
    } else {
        (var_lngfedge2,)
    }
};
        var_lngfedge2 = assign12500_e11299;
        var_lngfedge2_rv = 0.0;

        let (assign12510_e11305,) = {
    if (var_guard166 != 0.0) {
        let assign12510_e11303: f64 = (0.95 * var_phibedge);
        (assign12510_e11303,)
    } else {
        (var_phixedge,)
    }
};
        var_phixedge = assign12510_e11305;
        var_phixedge_rv = 0.0;

        let (assign12520_e11313,) = {
    if (var_guard166 != 0.0) {
        let assign12520_e11309: f64 = (0.0025 * var_phibedge);
        let assign12520_e11311: f64 = (assign12520_e11309 * var_phibedge);
        (assign12520_e11311,)
    } else {
        (var_aphiedge,)
    }
};
        var_aphiedge = assign12520_e11313;
        var_aphiedge_rv = 0.0;

        let (assign12530_e11317,) = {
    if (var_guard166 != 0.0) {
        (var_aphiedge,)
    } else {
        (var_bphiedge,)
    }
};
        var_bphiedge = assign12530_e11317;
        var_bphiedge_rv = 0.0;

        let (assign12540_e11324,) = {
    if (var_guard166 != 0.0) {
        let assign12540_e11321: f64 = (var_bphiedge).sqrt();
        let assign12540_e11322: f64 = (0.5 * assign12540_e11321);
        (assign12540_e11322,)
    } else {
        (var_phix2edge,)
    }
};
        var_phix2edge = assign12540_e11324;
        var_phix2edge_rv = 0.0;

        let (assign12550_e11349,) = {
    if (var_guard166 != 0.0) {
        let assign12550_e11329: f64 = (var_phixedge - var_phix2edge);
        let assign12550_e11331: f64 = assign12550_e11329;
        let assign12550_e11334: f64 = (var_phixedge - var_phix2edge);
        let assign12550_e11336: f64 = assign12550_e11334;
        let assign12550_e11339: f64 = (var_phixedge - var_phix2edge);
        let assign12550_e11341: f64 = assign12550_e11339;
        let assign12550_e11342: f64 = (assign12550_e11336 * assign12550_e11341);
        let assign12550_e11344: f64 = (assign12550_e11342 + var_aphiedge);
        let assign12550_e11345: f64 = (assign12550_e11344).sqrt();
        let assign12550_e11346: f64 = (assign12550_e11331 - assign12550_e11345);
        let assign12550_e11347: f64 = (0.5 * assign12550_e11346);
        (assign12550_e11347,)
    } else {
        (var_phix1edge,)
    }
};
        var_phix1edge = assign12550_e11349;
        var_phix1edge_rv = 0.0;

        let (assign12580_e11374,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_vfbedge_t,)
    }
};
        var_vfbedge_t = assign12580_e11374;
        var_vfbedge_t_rv = 0.0;

        let (assign12590_e11379,) = {
    if (var_guard166 == 0.0) {
        (1.0,)
    } else {
        (var_tf_betedge,)
    }
};
        var_tf_betedge = assign12590_e11379;
        var_tf_betedge_rv = 0.0;

        let (assign12600_e11384,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_betnedge_t,)
    }
};
        var_betnedge_t = assign12600_e11384;
        var_betnedge_t_rv = 0.0;

        let (assign12610_e11389,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_betedge_i,)
    }
};
        var_betedge_i = assign12610_e11389;
        var_betedge_i_rv = 0.0;

        let (assign12620_e11394,) = {
    if (var_guard166 == 0.0) {
        (var_phit,)
    } else {
        (var_phit0edge,)
    }
};
        var_phit0edge = assign12620_e11394;
        var_phit0edge_rv = 0.0;

        let (assign12630_e11399,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_phibedge,)
    }
};
        var_phibedge = assign12630_e11399;
        var_phibedge_rv = 0.0;

        let (assign12640_e11404,) = {
    if (var_guard166 == 0.0) {
        (1.0,)
    } else {
        (var_gfedge,)
    }
};
        var_gfedge = assign12640_e11404;
        var_gfedge_rv = 0.0;

        let (assign12650_e11409,) = {
    if (var_guard166 == 0.0) {
        (1.0,)
    } else {
        (var_gfedge2,)
    }
};
        var_gfedge2 = assign12650_e11409;
        var_gfedge2_rv = 0.0;

        let (assign12660_e11414,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_lngfedge2,)
    }
};
        var_lngfedge2 = assign12660_e11414;
        var_lngfedge2_rv = 0.0;

        let (assign12670_e11419,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_phixedge,)
    }
};
        var_phixedge = assign12670_e11419;
        var_phixedge_rv = 0.0;

        let (assign12680_e11424,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_aphiedge,)
    }
};
        var_aphiedge = assign12680_e11424;
        var_aphiedge_rv = 0.0;

        let (assign12690_e11429,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_bphiedge,)
    }
};
        var_bphiedge = assign12690_e11429;
        var_bphiedge_rv = 0.0;

        let (assign12700_e11434,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_phix2edge,)
    }
};
        var_phix2edge = assign12700_e11434;
        var_phix2edge_rv = 0.0;

        let (assign12710_e11439,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_phix1edge,)
    }
};
        var_phix1edge = assign12710_e11439;
        var_phix1edge_rv = 0.0;

        let assign12740_e11452: f64 = (1.0 / var_chib_i);
        var_inv_chib = assign12740_e11452;
        var_inv_chib_rv = 0.0;

        let assign12750_e11455: f64 = (4.0 * 0.3333333333333333);
        let assign12750_e11458: f64 = (2.0 * 1.6021918e-19);
        let assign12750_e11460: f64 = (assign12750_e11458 * 9.1093826e-31);
        let assign12750_e11462: f64 = (assign12750_e11460 * var_chib_i);
        let assign12750_e11463: f64 = (assign12750_e11462).sqrt();
        let assign12750_e11464: f64 = (assign12750_e11455 * assign12750_e11463);
        let assign12750_e11466: f64 = (assign12750_e11464 / 1.05457168e-34);
        var_b_fact = assign12750_e11466;
        var_b_fact_rv = 0.0;

        let assign12760_e11469: f64 = (var_b_fact * var_tox_i);
        var_bch = assign12760_e11469;
        var_bch_rv = 0.0;

        let assign12770_e11472: f64 = (var_b_fact * var_toxov_i);
        var_bov = assign12770_e11472;
        var_bov_rv = 0.0;

        let assign12780_e11475: f64 = (var_b_fact * var_toxovd_i);
        var_bov_d = assign12780_e11475;
        var_bov_d_rv = 0.0;

        var_gcq = 0.0;
        var_gcq_rv = 0.0;

        let assign12800_e11479: f64 = if var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        var_guard167 = assign12800_e11479;
        var_guard167_rv = 0.0;

        let (assign12810_e11488,) = {
    if (var_guard167 != 0.0) {
        let assign12810_e11482: f64 = (-0.495);
        let assign12810_e11484: f64 = (assign12810_e11482 * var_gc2_i);
        let assign12810_e11486: f64 = (assign12810_e11484 / var_gc3_i);
        (assign12810_e11486,)
    } else {
        (var_gcq,)
    }
};
        var_gcq = assign12810_e11488;
        var_gcq_rv = 0.0;

        var_gcqov = 0.0;
        var_gcqov_rv = 0.0;

        let assign12830_e11492: f64 = if var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        var_guard168 = assign12830_e11492;
        var_guard168_rv = 0.0;

        let (assign12840_e11501,) = {
    if (var_guard168 != 0.0) {
        let assign12840_e11495: f64 = (-0.495);
        let assign12840_e11497: f64 = (assign12840_e11495 * var_gc2ov_i);
        let assign12840_e11499: f64 = (assign12840_e11497 / var_gc3ov_i);
        (assign12840_e11499,)
    } else {
        (var_gcqov,)
    }
};
        var_gcqov = assign12840_e11501;
        var_gcqov_rv = 0.0;

        let assign12850_e11504: f64 = if var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        var_guard169 = assign12850_e11504;
        var_guard169_rv = 0.0;

        let (assign12860_e11513,) = {
    if (var_guard169 != 0.0) {
        let assign12860_e11507: f64 = (-0.495);
        let assign12860_e11509: f64 = (assign12860_e11507 * var_gc2ovd_i);
        let assign12860_e11511: f64 = (assign12860_e11509 / var_gc3ovd_i);
        (assign12860_e11511,)
    } else {
        (var_gcqovd,)
    }
};
        var_gcqovd = assign12860_e11513;
        var_gcqovd_rv = 0.0;

        let assign12870_e11516: f64 = (var_rta).powf(var_stig_i);
        var_tf_ig = assign12870_e11516;
        var_tf_ig_rv = 0.0;

        let assign12880_e11519: f64 = (var_iginv_i * var_tf_ig);
        var_iginv_i = assign12880_e11519;
        var_iginv_i_rv = 0.0;

        let assign12890_e11522: f64 = (var_igov_i * var_tf_ig);
        var_igov_i = assign12890_e11522;
        var_igov_i_rv = 0.0;

        let assign12900_e11525: f64 = (var_igovd_i * var_tf_ig);
        var_igovd_i = assign12900_e11525;
        var_igovd_i_rv = 0.0;

        let assign12930_e11543: f64 = (var_stbgidl_i * var_delta);
        let assign12930_e11544: f64 = (1.0 + assign12930_e11543);
        let (assign12930_e11553,) = {
    if (assign12930_e11544 > 0.0) {
        let assign12930_e11550: f64 = (var_stbgidl_i * var_delta);
        let assign12930_e11551: f64 = (1.0 + assign12930_e11550);
        (assign12930_e11551,)
    } else {
        (0.0,)
    }
};
        var_b_fact = assign12930_e11553;
        var_b_fact_rv = 0.0;

        let assign12940_e11556: f64 = (var_bgidl_i * var_b_fact);
        var_bgidl_t = assign12940_e11556;
        var_bgidl_t_rv = 0.0;

        *var_a2_t_slot = var_a2_t;
        *var_a2_t_rv_slot = var_a2_t_rv;
        *var_aphiedge_slot = var_aphiedge;
        *var_aphiedge_rv_slot = var_aphiedge_rv;
        *var_b_fact_slot = var_b_fact;
        *var_b_fact_rv_slot = var_b_fact_rv;
        *var_bch_slot = var_bch;
        *var_bch_rv_slot = var_bch_rv;
        *var_betedge_i_slot = var_betedge_i;
        *var_betedge_i_rv_slot = var_betedge_i_rv;
        *var_betnedge_t_slot = var_betnedge_t;
        *var_betnedge_t_rv_slot = var_betnedge_t_rv;
        *var_bgidl_t_slot = var_bgidl_t;
        *var_bgidl_t_rv_slot = var_bgidl_t_rv;
        *var_bov_slot = var_bov;
        *var_bov_d_slot = var_bov_d;
        *var_bov_d_rv_slot = var_bov_d_rv;
        *var_bov_rv_slot = var_bov_rv;
        *var_bphiedge_slot = var_bphiedge;
        *var_bphiedge_rv_slot = var_bphiedge_rv;
        *var_gcq_slot = var_gcq;
        *var_gcq_rv_slot = var_gcq_rv;
        *var_gcqov_slot = var_gcqov;
        *var_gcqov_rv_slot = var_gcqov_rv;
        *var_gcqovd_slot = var_gcqovd;
        *var_gcqovd_rv_slot = var_gcqovd_rv;
        *var_gfedge_slot = var_gfedge;
        *var_gfedge2_slot = var_gfedge2;
        *var_gfedge2_rv_slot = var_gfedge2_rv;
        *var_gfedge_rv_slot = var_gfedge_rv;
        *var_guard166_slot = var_guard166;
        *var_guard166_rv_slot = var_guard166_rv;
        *var_guard167_slot = var_guard167;
        *var_guard167_rv_slot = var_guard167_rv;
        *var_guard168_slot = var_guard168;
        *var_guard168_rv_slot = var_guard168_rv;
        *var_guard169_slot = var_guard169;
        *var_guard169_rv_slot = var_guard169_rv;
        *var_iginv_i_slot = var_iginv_i;
        *var_iginv_i_rv_slot = var_iginv_i_rv;
        *var_igov_i_slot = var_igov_i;
        *var_igov_i_rv_slot = var_igov_i_rv;
        *var_igovd_i_slot = var_igovd_i;
        *var_igovd_i_rv_slot = var_igovd_i_rv;
        *var_inv_chib_slot = var_inv_chib;
        *var_inv_chib_rv_slot = var_inv_chib_rv;
        *var_lngfedge2_slot = var_lngfedge2;
        *var_lngfedge2_rv_slot = var_lngfedge2_rv;
        *var_nt_slot = var_nt;
        *var_nt_rv_slot = var_nt_rv;
        *var_phibedge_slot = var_phibedge;
        *var_phibedge_rv_slot = var_phibedge_rv;
        *var_phit0edge_slot = var_phit0edge;
        *var_phit0edge_rv_slot = var_phit0edge_rv;
        *var_phix1edge_slot = var_phix1edge;
        *var_phix1edge_rv_slot = var_phix1edge_rv;
        *var_phix2edge_slot = var_phix2edge;
        *var_phix2edge_rv_slot = var_phix2edge_rv;
        *var_phixedge_slot = var_phixedge;
        *var_phixedge_rv_slot = var_phixedge_rv;
        *var_rs_t_slot = var_rs_t;
        *var_rs_t_rv_slot = var_rs_t_rv;
        *var_tf_betedge_slot = var_tf_betedge;
        *var_tf_betedge_rv_slot = var_tf_betedge_rv;
        *var_tf_ig_slot = var_tf_ig;
        *var_tf_ig_rv_slot = var_tf_ig_rv;
        *var_tf_ther_slot = var_tf_ther;
        *var_tf_ther_rv_slot = var_tf_ther_rv;
        *var_tf_thesat_slot = var_tf_thesat;
        *var_tf_thesat_rv_slot = var_tf_thesat_rv;
        *var_tf_xcor_slot = var_tf_xcor;
        *var_tf_xcor_rv_slot = var_tf_xcor_rv;
        *var_ther_i_slot = var_ther_i;
        *var_ther_i_rv_slot = var_ther_i_rv;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesat_t_rv_slot = var_thesat_t_rv;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_rv_slot = var_thesatac_t_rv;
        *var_vfbedge_t_slot = var_vfbedge_t;
        *var_vfbedge_t_rv_slot = var_vfbedge_t_rv;
        *var_xcor_t_slot = var_xcor_t;
        *var_xcor_t_rv_slot = var_xcor_t_rv;
    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        var_ad_i: f64,
        var_as_i: f64,
        var_axinr_i: f64,
        var_bgidl_t: f64,
        var_bgidld_i: f64,
        var_delta: f64,
        var_fcinracc_i: f64,
        var_idsatbot: f64,
        var_idsatgat: f64,
        var_idsatsti: f64,
        var_invnf: f64,
        var_jw_i: f64,
        var_pd_i: f64,
        var_phitd: f64,
        var_ps_i: f64,
        var_stbgidld_i: f64,
        var_toxov_i: f64,
        var_toxovd_i: f64,
        var_we: f64,
        var_abd_i_slot: &mut f64,
        var_abd_i_rv_slot: &mut f64,
        var_abdrain_i_slot: &mut f64,
        var_abdrain_i_rv_slot: &mut f64,
        var_abs_i_slot: &mut f64,
        var_abs_i_rv_slot: &mut f64,
        var_absource_i_slot: &mut f64,
        var_absource_i_rv_slot: &mut f64,
        var_ainr_slot: &mut f64,
        var_ainr_rv_slot: &mut f64,
        var_b_fact_slot: &mut f64,
        var_b_fact_rv_slot: &mut f64,
        var_bgidld_t_slot: &mut f64,
        var_bgidld_t_rv_slot: &mut f64,
        var_bgidlds_slot: &mut f64,
        var_bgidlds_rv_slot: &mut f64,
        var_bgidls_slot: &mut f64,
        var_bgidls_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_d_slot: &mut f64,
        var_exp_vmax_over_phitd_d_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_s_slot: &mut f64,
        var_exp_vmax_over_phitd_s_rv_slot: &mut f64,
        var_guard170_slot: &mut f64,
        var_guard170_rv_slot: &mut f64,
        var_guard178_slot: &mut f64,
        var_guard178_rv_slot: &mut f64,
        var_guard179_slot: &mut f64,
        var_guard179_rv_slot: &mut f64,
        var_guard180_slot: &mut f64,
        var_guard180_rv_slot: &mut f64,
        var_guard181_slot: &mut f64,
        var_guard181_rv_slot: &mut f64,
        var_guard182_slot: &mut f64,
        var_guard182_rv_slot: &mut f64,
        var_guard183_slot: &mut f64,
        var_guard183_rv_slot: &mut f64,
        var_guard184_slot: &mut f64,
        var_guard184_rv_slot: &mut f64,
        var_guard185_slot: &mut f64,
        var_guard185_rv_slot: &mut f64,
        var_jwcorr_slot: &mut f64,
        var_jwcorr_rv_slot: &mut f64,
        var_jww_slot: &mut f64,
        var_jww_rv_slot: &mut f64,
        var_lgd_i_slot: &mut f64,
        var_lgd_i_rv_slot: &mut f64,
        var_lgdrain_i_slot: &mut f64,
        var_lgdrain_i_rv_slot: &mut f64,
        var_lgs_i_slot: &mut f64,
        var_lgs_i_rv_slot: &mut f64,
        var_lgsource_i_slot: &mut f64,
        var_lgsource_i_rv_slot: &mut f64,
        var_lsd_i_slot: &mut f64,
        var_lsd_i_rv_slot: &mut f64,
        var_lsdrain_i_slot: &mut f64,
        var_lsdrain_i_rv_slot: &mut f64,
        var_lss_i_slot: &mut f64,
        var_lss_i_rv_slot: &mut f64,
        var_lssource_i_slot: &mut f64,
        var_lssource_i_rv_slot: &mut f64,
        var_vbbtlim_d_slot: &mut f64,
        var_vbbtlim_d_rv_slot: &mut f64,
        var_vbbtlim_s_slot: &mut f64,
        var_vbbtlim_s_rv_slot: &mut f64,
        var_vbimin_d_slot: &mut f64,
        var_vbimin_d_rv_slot: &mut f64,
        var_vbimin_s_slot: &mut f64,
        var_vbimin_s_rv_slot: &mut f64,
        var_vch_d_slot: &mut f64,
        var_vch_d_rv_slot: &mut f64,
        var_vch_s_slot: &mut f64,
        var_vch_s_rv_slot: &mut f64,
        var_vfmin_d_slot: &mut f64,
        var_vfmin_d_rv_slot: &mut f64,
        var_vfmin_s_slot: &mut f64,
        var_vfmin_s_rv_slot: &mut f64,
        var_vinr_max_slot: &mut f64,
        var_vinr_max_rv_slot: &mut f64,
        var_vmax_d_slot: &mut f64,
        var_vmax_d_rv_slot: &mut f64,
        var_vmax_s_slot: &mut f64,
        var_vmax_s_rv_slot: &mut f64,
        var_vmaxbot_slot: &mut f64,
        var_vmaxbot_rv_slot: &mut f64,
        var_vmaxgat_slot: &mut f64,
        var_vmaxgat_rv_slot: &mut f64,
        var_vmaxsti_slot: &mut f64,
        var_vmaxsti_rv_slot: &mut f64,
        var_zflagbot_d_slot: &mut f64,
        var_zflagbot_d_rv_slot: &mut f64,
        var_zflagbot_s_slot: &mut f64,
        var_zflagbot_s_rv_slot: &mut f64,
        var_zflaggat_d_slot: &mut f64,
        var_zflaggat_d_rv_slot: &mut f64,
        var_zflaggat_s_slot: &mut f64,
        var_zflaggat_s_rv_slot: &mut f64,
        var_zflagsti_d_slot: &mut f64,
        var_zflagsti_d_rv_slot: &mut f64,
        var_zflagsti_s_slot: &mut f64,
        var_zflagsti_s_rv_slot: &mut f64,
        var_zfrac_slot: &mut f64,
        var_zfrac_rv_slot: &mut f64,
    ) {
        let mut var_abd_i: f64 = *var_abd_i_slot;
        let mut var_abd_i_rv: f64 = *var_abd_i_rv_slot;
        let mut var_abdrain_i: f64 = *var_abdrain_i_slot;
        let mut var_abdrain_i_rv: f64 = *var_abdrain_i_rv_slot;
        let mut var_abs_i: f64 = *var_abs_i_slot;
        let mut var_abs_i_rv: f64 = *var_abs_i_rv_slot;
        let mut var_absource_i: f64 = *var_absource_i_slot;
        let mut var_absource_i_rv: f64 = *var_absource_i_rv_slot;
        let mut var_ainr: f64 = *var_ainr_slot;
        let mut var_ainr_rv: f64 = *var_ainr_rv_slot;
        let mut var_b_fact: f64 = *var_b_fact_slot;
        let mut var_b_fact_rv: f64 = *var_b_fact_rv_slot;
        let mut var_bgidld_t: f64 = *var_bgidld_t_slot;
        let mut var_bgidld_t_rv: f64 = *var_bgidld_t_rv_slot;
        let mut var_bgidlds: f64 = *var_bgidlds_slot;
        let mut var_bgidlds_rv: f64 = *var_bgidlds_rv_slot;
        let mut var_bgidls: f64 = *var_bgidls_slot;
        let mut var_bgidls_rv: f64 = *var_bgidls_rv_slot;
        let mut var_exp_vmax_over_phitd_d: f64 = *var_exp_vmax_over_phitd_d_slot;
        let mut var_exp_vmax_over_phitd_d_rv: f64 = *var_exp_vmax_over_phitd_d_rv_slot;
        let mut var_exp_vmax_over_phitd_s: f64 = *var_exp_vmax_over_phitd_s_slot;
        let mut var_exp_vmax_over_phitd_s_rv: f64 = *var_exp_vmax_over_phitd_s_rv_slot;
        let mut var_guard170: f64 = *var_guard170_slot;
        let mut var_guard170_rv: f64 = *var_guard170_rv_slot;
        let mut var_guard178: f64 = *var_guard178_slot;
        let mut var_guard178_rv: f64 = *var_guard178_rv_slot;
        let mut var_guard179: f64 = *var_guard179_slot;
        let mut var_guard179_rv: f64 = *var_guard179_rv_slot;
        let mut var_guard180: f64 = *var_guard180_slot;
        let mut var_guard180_rv: f64 = *var_guard180_rv_slot;
        let mut var_guard181: f64 = *var_guard181_slot;
        let mut var_guard181_rv: f64 = *var_guard181_rv_slot;
        let mut var_guard182: f64 = *var_guard182_slot;
        let mut var_guard182_rv: f64 = *var_guard182_rv_slot;
        let mut var_guard183: f64 = *var_guard183_slot;
        let mut var_guard183_rv: f64 = *var_guard183_rv_slot;
        let mut var_guard184: f64 = *var_guard184_slot;
        let mut var_guard184_rv: f64 = *var_guard184_rv_slot;
        let mut var_guard185: f64 = *var_guard185_slot;
        let mut var_guard185_rv: f64 = *var_guard185_rv_slot;
        let mut var_jwcorr: f64 = *var_jwcorr_slot;
        let mut var_jwcorr_rv: f64 = *var_jwcorr_rv_slot;
        let mut var_jww: f64 = *var_jww_slot;
        let mut var_jww_rv: f64 = *var_jww_rv_slot;
        let mut var_lgd_i: f64 = *var_lgd_i_slot;
        let mut var_lgd_i_rv: f64 = *var_lgd_i_rv_slot;
        let mut var_lgdrain_i: f64 = *var_lgdrain_i_slot;
        let mut var_lgdrain_i_rv: f64 = *var_lgdrain_i_rv_slot;
        let mut var_lgs_i: f64 = *var_lgs_i_slot;
        let mut var_lgs_i_rv: f64 = *var_lgs_i_rv_slot;
        let mut var_lgsource_i: f64 = *var_lgsource_i_slot;
        let mut var_lgsource_i_rv: f64 = *var_lgsource_i_rv_slot;
        let mut var_lsd_i: f64 = *var_lsd_i_slot;
        let mut var_lsd_i_rv: f64 = *var_lsd_i_rv_slot;
        let mut var_lsdrain_i: f64 = *var_lsdrain_i_slot;
        let mut var_lsdrain_i_rv: f64 = *var_lsdrain_i_rv_slot;
        let mut var_lss_i: f64 = *var_lss_i_slot;
        let mut var_lss_i_rv: f64 = *var_lss_i_rv_slot;
        let mut var_lssource_i: f64 = *var_lssource_i_slot;
        let mut var_lssource_i_rv: f64 = *var_lssource_i_rv_slot;
        let mut var_vbbtlim_d: f64 = *var_vbbtlim_d_slot;
        let mut var_vbbtlim_d_rv: f64 = *var_vbbtlim_d_rv_slot;
        let mut var_vbbtlim_s: f64 = *var_vbbtlim_s_slot;
        let mut var_vbbtlim_s_rv: f64 = *var_vbbtlim_s_rv_slot;
        let mut var_vbimin_d: f64 = *var_vbimin_d_slot;
        let mut var_vbimin_d_rv: f64 = *var_vbimin_d_rv_slot;
        let mut var_vbimin_s: f64 = *var_vbimin_s_slot;
        let mut var_vbimin_s_rv: f64 = *var_vbimin_s_rv_slot;
        let mut var_vch_d: f64 = *var_vch_d_slot;
        let mut var_vch_d_rv: f64 = *var_vch_d_rv_slot;
        let mut var_vch_s: f64 = *var_vch_s_slot;
        let mut var_vch_s_rv: f64 = *var_vch_s_rv_slot;
        let mut var_vfmin_d: f64 = *var_vfmin_d_slot;
        let mut var_vfmin_d_rv: f64 = *var_vfmin_d_rv_slot;
        let mut var_vfmin_s: f64 = *var_vfmin_s_slot;
        let mut var_vfmin_s_rv: f64 = *var_vfmin_s_rv_slot;
        let mut var_vinr_max: f64 = *var_vinr_max_slot;
        let mut var_vinr_max_rv: f64 = *var_vinr_max_rv_slot;
        let mut var_vmax_d: f64 = *var_vmax_d_slot;
        let mut var_vmax_d_rv: f64 = *var_vmax_d_rv_slot;
        let mut var_vmax_s: f64 = *var_vmax_s_slot;
        let mut var_vmax_s_rv: f64 = *var_vmax_s_rv_slot;
        let mut var_vmaxbot: f64 = *var_vmaxbot_slot;
        let mut var_vmaxbot_rv: f64 = *var_vmaxbot_rv_slot;
        let mut var_vmaxgat: f64 = *var_vmaxgat_slot;
        let mut var_vmaxgat_rv: f64 = *var_vmaxgat_rv_slot;
        let mut var_vmaxsti: f64 = *var_vmaxsti_slot;
        let mut var_vmaxsti_rv: f64 = *var_vmaxsti_rv_slot;
        let mut var_zflagbot_d: f64 = *var_zflagbot_d_slot;
        let mut var_zflagbot_d_rv: f64 = *var_zflagbot_d_rv_slot;
        let mut var_zflagbot_s: f64 = *var_zflagbot_s_slot;
        let mut var_zflagbot_s_rv: f64 = *var_zflagbot_s_rv_slot;
        let mut var_zflaggat_d: f64 = *var_zflaggat_d_slot;
        let mut var_zflaggat_d_rv: f64 = *var_zflaggat_d_rv_slot;
        let mut var_zflaggat_s: f64 = *var_zflaggat_s_slot;
        let mut var_zflaggat_s_rv: f64 = *var_zflaggat_s_rv_slot;
        let mut var_zflagsti_d: f64 = *var_zflagsti_d_slot;
        let mut var_zflagsti_d_rv: f64 = *var_zflagsti_d_rv_slot;
        let mut var_zflagsti_s: f64 = *var_zflagsti_s_slot;
        let mut var_zflagsti_s_rv: f64 = *var_zflagsti_s_rv_slot;
        let mut var_zfrac: f64 = *var_zfrac_slot;
        let mut var_zfrac_rv: f64 = *var_zfrac_rv_slot;

        let assign12950_e11559: f64 = (var_bgidl_t * var_toxov_i);
        let assign12950_e11561: f64 = (assign12950_e11559 * 500000000.0);
        var_bgidls = assign12950_e11561;
        var_bgidls_rv = 0.0;

        let assign12960_e11565: f64 = (var_stbgidld_i * var_delta);
        let assign12960_e11566: f64 = (1.0 + assign12960_e11565);
        let (assign12960_e11575,) = {
    if (assign12960_e11566 > 0.0) {
        let assign12960_e11572: f64 = (var_stbgidld_i * var_delta);
        let assign12960_e11573: f64 = (1.0 + assign12960_e11572);
        (assign12960_e11573,)
    } else {
        (0.0,)
    }
};
        var_b_fact = assign12960_e11575;
        var_b_fact_rv = 0.0;

        let assign12970_e11578: f64 = (var_bgidld_i * var_b_fact);
        var_bgidld_t = assign12970_e11578;
        var_bgidld_t_rv = 0.0;

        let assign12980_e11581: f64 = (var_bgidld_t * var_toxovd_i);
        let assign12980_e11583: f64 = (assign12980_e11581 * 500000000.0);
        var_bgidlds = assign12980_e11583;
        var_bgidlds_rv = 0.0;

        var_vinr_max = 0.0;
        var_vinr_max_rv = 0.0;

        let assign13000_e11587: f64 = if var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        var_guard170 = assign13000_e11587;
        var_guard170_rv = 0.0;

        let (assign13010_e11593,) = {
    if (var_guard170 != 0.0) {
        let assign13010_e11591: f64 = (0.75 / var_fcinracc_i);
        (assign13010_e11591,)
    } else {
        (var_vinr_max,)
    }
};
        var_vinr_max = assign13010_e11593;
        var_vinr_max_rv = 0.0;

        let assign13020_e11596: f64 = (var_axinr_i * var_axinr_i);
        var_ainr = assign13020_e11596;
        var_ainr_rv = 0.0;

        let assign13250_e11702: f64 = (var_absource_i * var_invnf);
        var_abs_i = assign13250_e11702;
        var_abs_i_rv = 0.0;

        let assign13260_e11705: f64 = (var_lssource_i * var_invnf);
        var_lss_i = assign13260_e11705;
        var_lss_i_rv = 0.0;

        let assign13270_e11708: f64 = (var_lgsource_i * var_invnf);
        var_lgs_i = assign13270_e11708;
        var_lgs_i_rv = 0.0;

        let assign13280_e11711: f64 = (var_abdrain_i * var_invnf);
        var_abd_i = assign13280_e11711;
        var_abd_i_rv = 0.0;

        let assign13290_e11714: f64 = (var_lsdrain_i * var_invnf);
        var_lsd_i = assign13290_e11714;
        var_lsd_i_rv = 0.0;

        let assign13300_e11717: f64 = (var_lgdrain_i * var_invnf);
        var_lgd_i = assign13300_e11717;
        var_lgd_i_rv = 0.0;

        var_jwcorr = 0.0;
        var_jwcorr_rv = 0.0;

        let assign13320_e11721: f64 = if p.p43 == 3.0 { 1.0 } else { 0.0 };
        var_guard178 = assign13320_e11721;
        var_guard178_rv = 0.0;

        let (assign13330_e11725,) = {
    if (var_guard178 != 0.0) {
        (1.0,)
    } else {
        (var_jwcorr,)
    }
};
        var_jwcorr = assign13330_e11725;
        var_jwcorr_rv = 0.0;

        var_jww = var_we;
        var_jww_rv = 0.0;

        let assign13350_e11729: f64 = if p.p39 == 0.0 { 1.0 } else { 0.0 };
        var_guard179 = assign13350_e11729;
        var_guard179_rv = 0.0;

        let (assign13360_e11738,) = {
    if (var_guard179 != 0.0) {
        let (assign13360_e11736,) = {
            if (var_jw_i > 0.0) {
                (var_jw_i,)
            } else {
                (0.0,)
            }
        };
        (assign13360_e11736,)
    } else {
        (var_jww,)
    }
};
        var_jww = assign13360_e11738;
        var_jww_rv = 0.0;

        let assign13370_e11745: f64 = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };
        var_guard180 = assign13370_e11745;
        var_guard180_rv = 0.0;

        let (assign13380_e11751,) = {
    if (var_guard180 != 0.0) {
        let assign13380_e11749: f64 = (var_as_i * var_invnf);
        (assign13380_e11749,)
    } else {
        (var_abs_i,)
    }
};
        var_abs_i = assign13380_e11751;
        var_abs_i_rv = 0.0;

        let (assign13390_e11761,) = {
    if (var_guard180 != 0.0) {
        let assign13390_e11755: f64 = (var_ps_i * var_invnf);
        let assign13390_e11758: f64 = (var_jwcorr * var_jww);
        let assign13390_e11759: f64 = (assign13390_e11755 - assign13390_e11758);
        (assign13390_e11759,)
    } else {
        (var_lss_i,)
    }
};
        var_lss_i = assign13390_e11761;
        var_lss_i_rv = 0.0;

        let (assign13400_e11765,) = {
    if (var_guard180 != 0.0) {
        (var_jww,)
    } else {
        (var_lgs_i,)
    }
};
        var_lgs_i = assign13400_e11765;
        var_lgs_i_rv = 0.0;

        let (assign13410_e11771,) = {
    if (var_guard180 != 0.0) {
        let assign13410_e11769: f64 = (var_ad_i * var_invnf);
        (assign13410_e11769,)
    } else {
        (var_abd_i,)
    }
};
        var_abd_i = assign13410_e11771;
        var_abd_i_rv = 0.0;

        let (assign13420_e11781,) = {
    if (var_guard180 != 0.0) {
        let assign13420_e11775: f64 = (var_pd_i * var_invnf);
        let assign13420_e11778: f64 = (var_jwcorr * var_jww);
        let assign13420_e11779: f64 = (assign13420_e11775 - assign13420_e11778);
        (assign13420_e11779,)
    } else {
        (var_lsd_i,)
    }
};
        var_lsd_i = assign13420_e11781;
        var_lsd_i_rv = 0.0;

        let (assign13430_e11785,) = {
    if (var_guard180 != 0.0) {
        (var_jww,)
    } else {
        (var_lgd_i,)
    }
};
        var_lgd_i = assign13430_e11785;
        var_lgd_i_rv = 0.0;

        let assign13440_e11796: f64 = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };
        var_guard181 = assign13440_e11796;
        var_guard181_rv = 0.0;

        let (assign13450_e11805,) = {
    if (var_guard181 != 0.0) {
        let (assign13450_e11803,) = {
            if (var_abs_i > 0.0) {
                (var_abs_i,)
            } else {
                (0.0,)
            }
        };
        (assign13450_e11803,)
    } else {
        (var_absource_i,)
    }
};
        var_absource_i = assign13450_e11805;
        var_absource_i_rv = 0.0;

        let (assign13460_e11814,) = {
    if (var_guard181 != 0.0) {
        let (assign13460_e11812,) = {
            if (var_lss_i > 0.0) {
                (var_lss_i,)
            } else {
                (0.0,)
            }
        };
        (assign13460_e11812,)
    } else {
        (var_lssource_i,)
    }
};
        var_lssource_i = assign13460_e11814;
        var_lssource_i_rv = 0.0;

        let (assign13470_e11823,) = {
    if (var_guard181 != 0.0) {
        let (assign13470_e11821,) = {
            if (var_lgs_i > 0.0) {
                (var_lgs_i,)
            } else {
                (0.0,)
            }
        };
        (assign13470_e11821,)
    } else {
        (var_lgsource_i,)
    }
};
        var_lgsource_i = assign13470_e11823;
        var_lgsource_i_rv = 0.0;

        let (assign13480_e11832,) = {
    if (var_guard181 != 0.0) {
        let (assign13480_e11830,) = {
            if (var_abd_i > 0.0) {
                (var_abd_i,)
            } else {
                (0.0,)
            }
        };
        (assign13480_e11830,)
    } else {
        (var_abdrain_i,)
    }
};
        var_abdrain_i = assign13480_e11832;
        var_abdrain_i_rv = 0.0;

        let (assign13490_e11841,) = {
    if (var_guard181 != 0.0) {
        let (assign13490_e11839,) = {
            if (var_lsd_i > 0.0) {
                (var_lsd_i,)
            } else {
                (0.0,)
            }
        };
        (assign13490_e11839,)
    } else {
        (var_lsdrain_i,)
    }
};
        var_lsdrain_i = assign13490_e11841;
        var_lsdrain_i_rv = 0.0;

        let (assign13500_e11850,) = {
    if (var_guard181 != 0.0) {
        let (assign13500_e11848,) = {
            if (var_lgd_i > 0.0) {
                (var_lgd_i,)
            } else {
                (0.0,)
            }
        };
        (assign13500_e11848,)
    } else {
        (var_lgdrain_i,)
    }
};
        var_lgdrain_i = assign13500_e11850;
        var_lgdrain_i_rv = 0.0;

        let (assign13510_e11855,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_absource_i,)
    }
};
        var_absource_i = assign13510_e11855;
        var_absource_i_rv = 0.0;

        let (assign13520_e11860,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_lssource_i,)
    }
};
        var_lssource_i = assign13520_e11860;
        var_lssource_i_rv = 0.0;

        let (assign13530_e11865,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_lgsource_i,)
    }
};
        var_lgsource_i = assign13530_e11865;
        var_lgsource_i_rv = 0.0;

        let (assign13540_e11870,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_abdrain_i,)
    }
};
        var_abdrain_i = assign13540_e11870;
        var_abdrain_i_rv = 0.0;

        let (assign13550_e11875,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_lsdrain_i,)
    }
};
        var_lsdrain_i = assign13550_e11875;
        var_lsdrain_i_rv = 0.0;

        let (assign13560_e11880,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_lgdrain_i,)
    }
};
        var_lgdrain_i = assign13560_e11880;
        var_lgdrain_i_rv = 0.0;

        var_vbimin_s = 0.0;
        var_vbimin_s_rv = 0.0;

        var_vbimin_d = 0.0;
        var_vbimin_d_rv = 0.0;

        var_vfmin_s = 0.0;
        var_vfmin_s_rv = 0.0;

        var_vfmin_d = 0.0;
        var_vfmin_d_rv = 0.0;

        var_vch_s = 0.0;
        var_vch_s_rv = 0.0;

        var_vch_d = 0.0;
        var_vch_d_rv = 0.0;

        var_vbbtlim_s = 0.0;
        var_vbbtlim_s_rv = 0.0;

        var_vbbtlim_d = 0.0;
        var_vbbtlim_d_rv = 0.0;

        var_vmax_s = 0.0;
        var_vmax_s_rv = 0.0;

        var_vmax_d = 0.0;
        var_vmax_d_rv = 0.0;

        var_exp_vmax_over_phitd_s = 0.0;
        var_exp_vmax_over_phitd_s_rv = 0.0;

        var_exp_vmax_over_phitd_d = 0.0;
        var_exp_vmax_over_phitd_d_rv = 0.0;

        var_zflagbot_s = 1.0;
        var_zflagbot_s_rv = 0.0;

        var_zflagbot_d = 1.0;
        var_zflagbot_d_rv = 0.0;

        var_zflagsti_s = 1.0;
        var_zflagsti_s_rv = 0.0;

        var_zflagsti_d = 1.0;
        var_zflagsti_d_rv = 0.0;

        var_zflaggat_s = 1.0;
        var_zflaggat_s_rv = 0.0;

        var_zflaggat_d = 1.0;
        var_zflaggat_d_rv = 0.0;

        var_zfrac = 0.0;
        var_zfrac_rv = 0.0;

        let assign14130_e11939: f64 = if p.p43 > 0.0 { 1.0 } else { 0.0 };
        var_guard182 = assign14130_e11939;
        var_guard182_rv = 0.0;

        let assign14140_e11942: f64 = (var_idsatbot * var_absource_i);
        let assign14140_e11944: f64 = if assign14140_e11942 > 0.0 { 1.0 } else { 0.0 };
        var_guard183 = assign14140_e11944;
        var_guard183_rv = 0.0;

        let (assign14150_e11959,) = {
    if ((var_guard182 != 0.0) && (var_guard183 != 0.0)) {
        let assign14150_e11952: f64 = (var_idsatbot * var_absource_i);
        let assign14150_e11953: f64 = (p.p822 / assign14150_e11952);
        let assign14150_e11955: f64 = (assign14150_e11953 + 1.0);
        let assign14150_e11956: f64 = (assign14150_e11955).ln();
        let assign14150_e11957: f64 = (var_phitd * assign14150_e11956);
        (assign14150_e11957,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign14150_e11959;
        var_vmaxbot_rv = 0.0;

        let (assign14160_e11966,) = {
    if ((var_guard182 != 0.0) && (var_guard183 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign14160_e11966;
        var_vmaxbot_rv = 0.0;

        let assign14170_e11969: f64 = (var_idsatsti * var_lssource_i);
        let assign14170_e11971: f64 = if assign14170_e11969 > 0.0 { 1.0 } else { 0.0 };
        var_guard184 = assign14170_e11971;
        var_guard184_rv = 0.0;

        let (assign14180_e11986,) = {
    if ((var_guard182 != 0.0) && (var_guard184 != 0.0)) {
        let assign14180_e11979: f64 = (var_idsatsti * var_lssource_i);
        let assign14180_e11980: f64 = (p.p822 / assign14180_e11979);
        let assign14180_e11982: f64 = (assign14180_e11980 + 1.0);
        let assign14180_e11983: f64 = (assign14180_e11982).ln();
        let assign14180_e11984: f64 = (var_phitd * assign14180_e11983);
        (assign14180_e11984,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign14180_e11986;
        var_vmaxsti_rv = 0.0;

        let (assign14190_e11993,) = {
    if ((var_guard182 != 0.0) && (var_guard184 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign14190_e11993;
        var_vmaxsti_rv = 0.0;

        let assign14200_e11996: f64 = (var_idsatgat * var_lgsource_i);
        let assign14200_e11998: f64 = if assign14200_e11996 > 0.0 { 1.0 } else { 0.0 };
        var_guard185 = assign14200_e11998;
        var_guard185_rv = 0.0;

        let (assign14210_e12013,) = {
    if ((var_guard182 != 0.0) && (var_guard185 != 0.0)) {
        let assign14210_e12006: f64 = (var_idsatgat * var_lgsource_i);
        let assign14210_e12007: f64 = (p.p822 / assign14210_e12006);
        let assign14210_e12009: f64 = (assign14210_e12007 + 1.0);
        let assign14210_e12010: f64 = (assign14210_e12009).ln();
        let assign14210_e12011: f64 = (var_phitd * assign14210_e12010);
        (assign14210_e12011,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign14210_e12013;
        var_vmaxgat_rv = 0.0;

        let (assign14220_e12020,) = {
    if ((var_guard182 != 0.0) && (var_guard185 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign14220_e12020;
        var_vmaxgat_rv = 0.0;

        *var_abd_i_slot = var_abd_i;
        *var_abd_i_rv_slot = var_abd_i_rv;
        *var_abdrain_i_slot = var_abdrain_i;
        *var_abdrain_i_rv_slot = var_abdrain_i_rv;
        *var_abs_i_slot = var_abs_i;
        *var_abs_i_rv_slot = var_abs_i_rv;
        *var_absource_i_slot = var_absource_i;
        *var_absource_i_rv_slot = var_absource_i_rv;
        *var_ainr_slot = var_ainr;
        *var_ainr_rv_slot = var_ainr_rv;
        *var_b_fact_slot = var_b_fact;
        *var_b_fact_rv_slot = var_b_fact_rv;
        *var_bgidld_t_slot = var_bgidld_t;
        *var_bgidld_t_rv_slot = var_bgidld_t_rv;
        *var_bgidlds_slot = var_bgidlds;
        *var_bgidlds_rv_slot = var_bgidlds_rv;
        *var_bgidls_slot = var_bgidls;
        *var_bgidls_rv_slot = var_bgidls_rv;
        *var_exp_vmax_over_phitd_d_slot = var_exp_vmax_over_phitd_d;
        *var_exp_vmax_over_phitd_d_rv_slot = var_exp_vmax_over_phitd_d_rv;
        *var_exp_vmax_over_phitd_s_slot = var_exp_vmax_over_phitd_s;
        *var_exp_vmax_over_phitd_s_rv_slot = var_exp_vmax_over_phitd_s_rv;
        *var_guard170_slot = var_guard170;
        *var_guard170_rv_slot = var_guard170_rv;
        *var_guard178_slot = var_guard178;
        *var_guard178_rv_slot = var_guard178_rv;
        *var_guard179_slot = var_guard179;
        *var_guard179_rv_slot = var_guard179_rv;
        *var_guard180_slot = var_guard180;
        *var_guard180_rv_slot = var_guard180_rv;
        *var_guard181_slot = var_guard181;
        *var_guard181_rv_slot = var_guard181_rv;
        *var_guard182_slot = var_guard182;
        *var_guard182_rv_slot = var_guard182_rv;
        *var_guard183_slot = var_guard183;
        *var_guard183_rv_slot = var_guard183_rv;
        *var_guard184_slot = var_guard184;
        *var_guard184_rv_slot = var_guard184_rv;
        *var_guard185_slot = var_guard185;
        *var_guard185_rv_slot = var_guard185_rv;
        *var_jwcorr_slot = var_jwcorr;
        *var_jwcorr_rv_slot = var_jwcorr_rv;
        *var_jww_slot = var_jww;
        *var_jww_rv_slot = var_jww_rv;
        *var_lgd_i_slot = var_lgd_i;
        *var_lgd_i_rv_slot = var_lgd_i_rv;
        *var_lgdrain_i_slot = var_lgdrain_i;
        *var_lgdrain_i_rv_slot = var_lgdrain_i_rv;
        *var_lgs_i_slot = var_lgs_i;
        *var_lgs_i_rv_slot = var_lgs_i_rv;
        *var_lgsource_i_slot = var_lgsource_i;
        *var_lgsource_i_rv_slot = var_lgsource_i_rv;
        *var_lsd_i_slot = var_lsd_i;
        *var_lsd_i_rv_slot = var_lsd_i_rv;
        *var_lsdrain_i_slot = var_lsdrain_i;
        *var_lsdrain_i_rv_slot = var_lsdrain_i_rv;
        *var_lss_i_slot = var_lss_i;
        *var_lss_i_rv_slot = var_lss_i_rv;
        *var_lssource_i_slot = var_lssource_i;
        *var_lssource_i_rv_slot = var_lssource_i_rv;
        *var_vbbtlim_d_slot = var_vbbtlim_d;
        *var_vbbtlim_d_rv_slot = var_vbbtlim_d_rv;
        *var_vbbtlim_s_slot = var_vbbtlim_s;
        *var_vbbtlim_s_rv_slot = var_vbbtlim_s_rv;
        *var_vbimin_d_slot = var_vbimin_d;
        *var_vbimin_d_rv_slot = var_vbimin_d_rv;
        *var_vbimin_s_slot = var_vbimin_s;
        *var_vbimin_s_rv_slot = var_vbimin_s_rv;
        *var_vch_d_slot = var_vch_d;
        *var_vch_d_rv_slot = var_vch_d_rv;
        *var_vch_s_slot = var_vch_s;
        *var_vch_s_rv_slot = var_vch_s_rv;
        *var_vfmin_d_slot = var_vfmin_d;
        *var_vfmin_d_rv_slot = var_vfmin_d_rv;
        *var_vfmin_s_slot = var_vfmin_s;
        *var_vfmin_s_rv_slot = var_vfmin_s_rv;
        *var_vinr_max_slot = var_vinr_max;
        *var_vinr_max_rv_slot = var_vinr_max_rv;
        *var_vmax_d_slot = var_vmax_d;
        *var_vmax_d_rv_slot = var_vmax_d_rv;
        *var_vmax_s_slot = var_vmax_s;
        *var_vmax_s_rv_slot = var_vmax_s_rv;
        *var_vmaxbot_slot = var_vmaxbot;
        *var_vmaxbot_rv_slot = var_vmaxbot_rv;
        *var_vmaxgat_slot = var_vmaxgat;
        *var_vmaxgat_rv_slot = var_vmaxgat_rv;
        *var_vmaxsti_slot = var_vmaxsti;
        *var_vmaxsti_rv_slot = var_vmaxsti_rv;
        *var_zflagbot_d_slot = var_zflagbot_d;
        *var_zflagbot_d_rv_slot = var_zflagbot_d_rv;
        *var_zflagbot_s_slot = var_zflagbot_s;
        *var_zflagbot_s_rv_slot = var_zflagbot_s_rv;
        *var_zflaggat_d_slot = var_zflaggat_d;
        *var_zflaggat_d_rv_slot = var_zflaggat_d_rv;
        *var_zflaggat_s_slot = var_zflaggat_s;
        *var_zflaggat_s_rv_slot = var_zflaggat_s_rv;
        *var_zflagsti_d_slot = var_zflagsti_d;
        *var_zflagsti_d_rv_slot = var_zflagsti_d_rv;
        *var_zflagsti_s_slot = var_zflagsti_s;
        *var_zflagsti_s_rv_slot = var_zflagsti_s_rv;
        *var_zfrac_slot = var_zfrac;
        *var_zfrac_rv_slot = var_zfrac_rv;
    }

    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_guard182: f64,
        var_idsatbot_d: f64,
        var_idsatgat_d: f64,
        var_idsatsti_d: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_vbibot: f64,
        var_vbigat: f64,
        var_vbisti: f64,
        var_exp_vmax_over_phitd_d_slot: &mut f64,
        var_exp_vmax_over_phitd_d_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_s_slot: &mut f64,
        var_exp_vmax_over_phitd_s_rv_slot: &mut f64,
        var_guard186_slot: &mut f64,
        var_guard186_rv_slot: &mut f64,
        var_guard187_slot: &mut f64,
        var_guard187_rv_slot: &mut f64,
        var_guard188_slot: &mut f64,
        var_guard188_rv_slot: &mut f64,
        var_guard189_slot: &mut f64,
        var_guard189_rv_slot: &mut f64,
        var_guard190_slot: &mut f64,
        var_guard190_rv_slot: &mut f64,
        var_guard191_slot: &mut f64,
        var_guard191_rv_slot: &mut f64,
        var_guard192_slot: &mut f64,
        var_guard192_rv_slot: &mut f64,
        var_guard193_slot: &mut f64,
        var_guard193_rv_slot: &mut f64,
        var_guard194_slot: &mut f64,
        var_guard194_rv_slot: &mut f64,
        var_guard195_slot: &mut f64,
        var_guard195_rv_slot: &mut f64,
        var_pbot2_slot: &mut f64,
        var_pbot2_rv_slot: &mut f64,
        var_pgat2_slot: &mut f64,
        var_pgat2_rv_slot: &mut f64,
        var_pmax_slot: &mut f64,
        var_pmax_rv_slot: &mut f64,
        var_psti2_slot: &mut f64,
        var_psti2_rv_slot: &mut f64,
        var_vbbtlim_s_slot: &mut f64,
        var_vbbtlim_s_rv_slot: &mut f64,
        var_vbibot2_slot: &mut f64,
        var_vbibot2_rv_slot: &mut f64,
        var_vbibot2r_slot: &mut f64,
        var_vbibot2r_rv_slot: &mut f64,
        var_vbigat2_slot: &mut f64,
        var_vbigat2_rv_slot: &mut f64,
        var_vbigat2r_slot: &mut f64,
        var_vbigat2r_rv_slot: &mut f64,
        var_vbimin_s_slot: &mut f64,
        var_vbimin_s_rv_slot: &mut f64,
        var_vbisti2_slot: &mut f64,
        var_vbisti2_rv_slot: &mut f64,
        var_vbisti2r_slot: &mut f64,
        var_vbisti2r_rv_slot: &mut f64,
        var_vch_s_slot: &mut f64,
        var_vch_s_rv_slot: &mut f64,
        var_vfmin_s_slot: &mut f64,
        var_vfmin_s_rv_slot: &mut f64,
        var_vmax_d_slot: &mut f64,
        var_vmax_d_rv_slot: &mut f64,
        var_vmax_s_slot: &mut f64,
        var_vmax_s_rv_slot: &mut f64,
        var_vmaxbot_slot: &mut f64,
        var_vmaxbot_rv_slot: &mut f64,
        var_vmaxgat_slot: &mut f64,
        var_vmaxgat_rv_slot: &mut f64,
        var_vmaxsti_slot: &mut f64,
        var_vmaxsti_rv_slot: &mut f64,
    ) {
        let mut var_exp_vmax_over_phitd_d: f64 = *var_exp_vmax_over_phitd_d_slot;
        let mut var_exp_vmax_over_phitd_d_rv: f64 = *var_exp_vmax_over_phitd_d_rv_slot;
        let mut var_exp_vmax_over_phitd_s: f64 = *var_exp_vmax_over_phitd_s_slot;
        let mut var_exp_vmax_over_phitd_s_rv: f64 = *var_exp_vmax_over_phitd_s_rv_slot;
        let mut var_guard186: f64 = *var_guard186_slot;
        let mut var_guard186_rv: f64 = *var_guard186_rv_slot;
        let mut var_guard187: f64 = *var_guard187_slot;
        let mut var_guard187_rv: f64 = *var_guard187_rv_slot;
        let mut var_guard188: f64 = *var_guard188_slot;
        let mut var_guard188_rv: f64 = *var_guard188_rv_slot;
        let mut var_guard189: f64 = *var_guard189_slot;
        let mut var_guard189_rv: f64 = *var_guard189_rv_slot;
        let mut var_guard190: f64 = *var_guard190_slot;
        let mut var_guard190_rv: f64 = *var_guard190_rv_slot;
        let mut var_guard191: f64 = *var_guard191_slot;
        let mut var_guard191_rv: f64 = *var_guard191_rv_slot;
        let mut var_guard192: f64 = *var_guard192_slot;
        let mut var_guard192_rv: f64 = *var_guard192_rv_slot;
        let mut var_guard193: f64 = *var_guard193_slot;
        let mut var_guard193_rv: f64 = *var_guard193_rv_slot;
        let mut var_guard194: f64 = *var_guard194_slot;
        let mut var_guard194_rv: f64 = *var_guard194_rv_slot;
        let mut var_guard195: f64 = *var_guard195_slot;
        let mut var_guard195_rv: f64 = *var_guard195_rv_slot;
        let mut var_pbot2: f64 = *var_pbot2_slot;
        let mut var_pbot2_rv: f64 = *var_pbot2_rv_slot;
        let mut var_pgat2: f64 = *var_pgat2_slot;
        let mut var_pgat2_rv: f64 = *var_pgat2_rv_slot;
        let mut var_pmax: f64 = *var_pmax_slot;
        let mut var_pmax_rv: f64 = *var_pmax_rv_slot;
        let mut var_psti2: f64 = *var_psti2_slot;
        let mut var_psti2_rv: f64 = *var_psti2_rv_slot;
        let mut var_vbbtlim_s: f64 = *var_vbbtlim_s_slot;
        let mut var_vbbtlim_s_rv: f64 = *var_vbbtlim_s_rv_slot;
        let mut var_vbibot2: f64 = *var_vbibot2_slot;
        let mut var_vbibot2_rv: f64 = *var_vbibot2_rv_slot;
        let mut var_vbibot2r: f64 = *var_vbibot2r_slot;
        let mut var_vbibot2r_rv: f64 = *var_vbibot2r_rv_slot;
        let mut var_vbigat2: f64 = *var_vbigat2_slot;
        let mut var_vbigat2_rv: f64 = *var_vbigat2_rv_slot;
        let mut var_vbigat2r: f64 = *var_vbigat2r_slot;
        let mut var_vbigat2r_rv: f64 = *var_vbigat2r_rv_slot;
        let mut var_vbimin_s: f64 = *var_vbimin_s_slot;
        let mut var_vbimin_s_rv: f64 = *var_vbimin_s_rv_slot;
        let mut var_vbisti2: f64 = *var_vbisti2_slot;
        let mut var_vbisti2_rv: f64 = *var_vbisti2_rv_slot;
        let mut var_vbisti2r: f64 = *var_vbisti2r_slot;
        let mut var_vbisti2r_rv: f64 = *var_vbisti2r_rv_slot;
        let mut var_vch_s: f64 = *var_vch_s_slot;
        let mut var_vch_s_rv: f64 = *var_vch_s_rv_slot;
        let mut var_vfmin_s: f64 = *var_vfmin_s_slot;
        let mut var_vfmin_s_rv: f64 = *var_vfmin_s_rv_slot;
        let mut var_vmax_d: f64 = *var_vmax_d_slot;
        let mut var_vmax_d_rv: f64 = *var_vmax_d_rv_slot;
        let mut var_vmax_s: f64 = *var_vmax_s_slot;
        let mut var_vmax_s_rv: f64 = *var_vmax_s_rv_slot;
        let mut var_vmaxbot: f64 = *var_vmaxbot_slot;
        let mut var_vmaxbot_rv: f64 = *var_vmaxbot_rv_slot;
        let mut var_vmaxgat: f64 = *var_vmaxgat_slot;
        let mut var_vmaxgat_rv: f64 = *var_vmaxgat_rv_slot;
        let mut var_vmaxsti: f64 = *var_vmaxsti_slot;
        let mut var_vmaxsti_rv: f64 = *var_vmaxsti_rv_slot;

        let (assign14230_e12028,) = {
    if (var_guard182 != 0.0) {
        let assign14230_e12024: f64 = (var_vmaxbot).min(var_vmaxsti);
        let assign14230_e12026: f64 = (assign14230_e12024).min(var_vmaxgat);
        (assign14230_e12026,)
    } else {
        (var_vmax_s,)
    }
};
        var_vmax_s = assign14230_e12028;
        var_vmax_s_rv = 0.0;

        let assign14240_e12031: f64 = (var_vmax_s * var_phitdinv);
        let assign14240_e12032: f64 = (assign14240_e12031).abs();
        let assign14240_e12034: f64 = if assign14240_e12032 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard186 = assign14240_e12034;
        var_guard186_rv = 0.0;

        let (assign14250_e12043,) = {
    if ((var_guard182 != 0.0) && (var_guard186 != 0.0)) {
        let assign14250_e12040: f64 = (var_vmax_s * var_phitdinv);
        let assign14250_e12041: f64 = (assign14250_e12040).exp();
        (assign14250_e12041,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign14250_e12043;
        var_exp_vmax_over_phitd_s_rv = 0.0;

        let assign14260_e12046: f64 = (var_vmax_s * var_phitdinv);
        let assign14260_e12048: f64 = if assign14260_e12046 < 0.0 { 1.0 } else { 0.0 };
        var_guard187 = assign14260_e12048;
        var_guard187_rv = 0.0;

        let (assign14270_e12088,) = {
    if (((var_guard182 != 0.0) && (var_guard186 == 0.0)) && (var_guard187 != 0.0)) {
        let assign14270_e12058: f64 = (-230.25850929940458);
        let assign14270_e12061: f64 = (var_vmax_s * var_phitdinv);
        let assign14270_e12062: f64 = (assign14270_e12058 - assign14270_e12061);
        let assign14270_e12066: f64 = (-230.25850929940458);
        let assign14270_e12069: f64 = (var_vmax_s * var_phitdinv);
        let assign14270_e12070: f64 = (assign14270_e12066 - assign14270_e12069);
        let assign14270_e12073: f64 = (-230.25850929940458);
        let assign14270_e12076: f64 = (var_vmax_s * var_phitdinv);
        let assign14270_e12077: f64 = (assign14270_e12073 - assign14270_e12076);
        let assign14270_e12079: f64 = (assign14270_e12077 * 0.3333333333333333);
        let assign14270_e12080: f64 = (1.0 + assign14270_e12079);
        let assign14270_e12081: f64 = (assign14270_e12070 * assign14270_e12080);
        let assign14270_e12082: f64 = (0.5 * assign14270_e12081);
        let assign14270_e12083: f64 = (1.0 + assign14270_e12082);
        let assign14270_e12084: f64 = (assign14270_e12062 * assign14270_e12083);
        let assign14270_e12085: f64 = (1.0 + assign14270_e12084);
        let assign14270_e12086: f64 = (1e-100 / assign14270_e12085);
        (assign14270_e12086,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign14270_e12088;
        var_exp_vmax_over_phitd_s_rv = 0.0;

        let (assign14280_e12126,) = {
    if (((var_guard182 != 0.0) && (var_guard186 == 0.0)) && (var_guard187 == 0.0)) {
        let assign14280_e12100: f64 = (var_vmax_s * var_phitdinv);
        let assign14280_e12102: f64 = (assign14280_e12100 - 230.25850929940458);
        let assign14280_e12107: f64 = (var_vmax_s * var_phitdinv);
        let assign14280_e12109: f64 = (assign14280_e12107 - 230.25850929940458);
        let assign14280_e12113: f64 = (var_vmax_s * var_phitdinv);
        let assign14280_e12115: f64 = (assign14280_e12113 - 230.25850929940458);
        let assign14280_e12117: f64 = (assign14280_e12115 * 0.3333333333333333);
        let assign14280_e12118: f64 = (1.0 + assign14280_e12117);
        let assign14280_e12119: f64 = (assign14280_e12109 * assign14280_e12118);
        let assign14280_e12120: f64 = (0.5 * assign14280_e12119);
        let assign14280_e12121: f64 = (1.0 + assign14280_e12120);
        let assign14280_e12122: f64 = (assign14280_e12102 * assign14280_e12121);
        let assign14280_e12123: f64 = (1.0 + assign14280_e12122);
        let assign14280_e12124: f64 = (1e100 * assign14280_e12123);
        (assign14280_e12124,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign14280_e12126;
        var_exp_vmax_over_phitd_s_rv = 0.0;

        let (assign14290_e12130,) = {
    if (var_guard182 != 0.0) {
        (var_vbibot,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign14290_e12130;
        var_vbibot2_rv = 0.0;

        let (assign14300_e12134,) = {
    if (var_guard182 != 0.0) {
        (var_vbisti,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign14300_e12134;
        var_vbisti2_rv = 0.0;

        let (assign14310_e12138,) = {
    if (var_guard182 != 0.0) {
        (var_vbigat,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign14310_e12138;
        var_vbigat2_rv = 0.0;

        let (assign14320_e12142,) = {
    if (var_guard182 != 0.0) {
        (p.p831,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign14320_e12142;
        var_pbot2_rv = 0.0;

        let (assign14330_e12146,) = {
    if (var_guard182 != 0.0) {
        (p.p832,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign14330_e12146;
        var_psti2_rv = 0.0;

        let (assign14340_e12150,) = {
    if (var_guard182 != 0.0) {
        (p.p833,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign14340_e12150;
        var_pgat2_rv = 0.0;

        let (assign14350_e12154,) = {
    if (var_guard182 != 0.0) {
        (p.p828,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign14350_e12154;
        var_vbibot2r_rv = 0.0;

        let (assign14360_e12158,) = {
    if (var_guard182 != 0.0) {
        (p.p829,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign14360_e12158;
        var_vbisti2r_rv = 0.0;

        let (assign14370_e12162,) = {
    if (var_guard182 != 0.0) {
        (p.p830,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign14370_e12162;
        var_vbigat2r_rv = 0.0;

        let assign14380_e12165: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard188 = assign14380_e12165;
        var_guard188_rv = 0.0;

        let (assign14390_e12173,) = {
    if ((var_guard182 != 0.0) && (var_guard188 != 0.0)) {
        let assign14390_e12171: f64 = (var_vbisti + var_vbigat);
        (assign14390_e12171,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign14390_e12173;
        var_vbibot2_rv = 0.0;

        let (assign14400_e12183,) = {
    if ((var_guard182 != 0.0) && (var_guard188 != 0.0)) {
        let assign14400_e12180: f64 = (p.p832).min(p.p833);
        let assign14400_e12181: f64 = (0.9 * assign14400_e12180);
        (assign14400_e12181,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign14400_e12183;
        var_pbot2_rv = 0.0;

        let (assign14410_e12191,) = {
    if ((var_guard182 != 0.0) && (var_guard188 != 0.0)) {
        let assign14410_e12189: f64 = (p.p829 + p.p830);
        (assign14410_e12189,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign14410_e12191;
        var_vbibot2r_rv = 0.0;

        let assign14420_e12194: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard189 = assign14420_e12194;
        var_guard189_rv = 0.0;

        let (assign14430_e12202,) = {
    if ((var_guard182 != 0.0) && (var_guard189 != 0.0)) {
        let assign14430_e12200: f64 = (var_vbibot + var_vbigat);
        (assign14430_e12200,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign14430_e12202;
        var_vbisti2_rv = 0.0;

        let (assign14440_e12212,) = {
    if ((var_guard182 != 0.0) && (var_guard189 != 0.0)) {
        let assign14440_e12209: f64 = (p.p831).min(p.p833);
        let assign14440_e12210: f64 = (0.9 * assign14440_e12209);
        (assign14440_e12210,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign14440_e12212;
        var_psti2_rv = 0.0;

        let (assign14450_e12220,) = {
    if ((var_guard182 != 0.0) && (var_guard189 != 0.0)) {
        let assign14450_e12218: f64 = (p.p828 + p.p830);
        (assign14450_e12218,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign14450_e12220;
        var_vbisti2r_rv = 0.0;

        let assign14460_e12223: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard190 = assign14460_e12223;
        var_guard190_rv = 0.0;

        let (assign14470_e12231,) = {
    if ((var_guard182 != 0.0) && (var_guard190 != 0.0)) {
        let assign14470_e12229: f64 = (var_vbibot + var_vbisti);
        (assign14470_e12229,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign14470_e12231;
        var_vbigat2_rv = 0.0;

        let (assign14480_e12241,) = {
    if ((var_guard182 != 0.0) && (var_guard190 != 0.0)) {
        let assign14480_e12238: f64 = (p.p831).min(p.p832);
        let assign14480_e12239: f64 = (0.9 * assign14480_e12238);
        (assign14480_e12239,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign14480_e12241;
        var_pgat2_rv = 0.0;

        let (assign14490_e12249,) = {
    if ((var_guard182 != 0.0) && (var_guard190 != 0.0)) {
        let assign14490_e12247: f64 = (p.p828 + p.p829);
        (assign14490_e12247,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign14490_e12249;
        var_vbigat2r_rv = 0.0;

        let (assign14500_e12257,) = {
    if (var_guard182 != 0.0) {
        let assign14500_e12253: f64 = (var_vbibot2).min(var_vbisti2);
        let assign14500_e12255: f64 = (assign14500_e12253).min(var_vbigat2);
        (assign14500_e12255,)
    } else {
        (var_vbimin_s,)
    }
};
        var_vbimin_s = assign14500_e12257;
        var_vbimin_s_rv = 0.0;

        let (assign14510_e12263,) = {
    if (var_guard182 != 0.0) {
        let assign14510_e12261: f64 = (var_vbimin_s * 0.1);
        (assign14510_e12261,)
    } else {
        (var_vch_s,)
    }
};
        var_vch_s = assign14510_e12263;
        var_vch_s_rv = 0.0;

        let (assign14520_e12271,) = {
    if (var_guard182 != 0.0) {
        let assign14520_e12267: f64 = (var_pbot2).max(var_psti2);
        let assign14520_e12269: f64 = (assign14520_e12267).max(var_pgat2);
        (assign14520_e12269,)
    } else {
        (var_pmax,)
    }
};
        var_pmax = assign14520_e12271;
        var_pmax_rv = 0.0;

        let (assign14530_e12284,) = {
    if (var_guard182 != 0.0) {
        let assign14530_e12277: f64 = (-1.0);
        let assign14530_e12279: f64 = (assign14530_e12277 / var_pmax);
        let assign14530_e12280: f64 = (2.0_f64).powf(assign14530_e12279);
        let assign14530_e12281: f64 = (1.0 - assign14530_e12280);
        let assign14530_e12282: f64 = (var_vbimin_s * assign14530_e12281);
        (assign14530_e12282,)
    } else {
        (var_vfmin_s,)
    }
};
        var_vfmin_s = assign14530_e12284;
        var_vfmin_s_rv = 0.0;

        let (assign14540_e12294,) = {
    if (var_guard182 != 0.0) {
        let assign14540_e12288: f64 = (var_vbibot2r).min(var_vbisti2r);
        let assign14540_e12290: f64 = (assign14540_e12288).min(var_vbigat2r);
        let assign14540_e12292: f64 = (assign14540_e12290 - 0.05);
        (assign14540_e12292,)
    } else {
        (var_vbbtlim_s,)
    }
};
        var_vbbtlim_s = assign14540_e12294;
        var_vbbtlim_s_rv = 0.0;

        let assign14550_e12297: f64 = (var_idsatbot_d * var_abdrain_i);
        let assign14550_e12299: f64 = if assign14550_e12297 > 0.0 { 1.0 } else { 0.0 };
        var_guard191 = assign14550_e12299;
        var_guard191_rv = 0.0;

        let (assign14560_e12314,) = {
    if ((var_guard182 != 0.0) && (var_guard191 != 0.0)) {
        let assign14560_e12307: f64 = (var_idsatbot_d * var_abdrain_i);
        let assign14560_e12308: f64 = (p.p822 / assign14560_e12307);
        let assign14560_e12310: f64 = (assign14560_e12308 + 1.0);
        let assign14560_e12311: f64 = (assign14560_e12310).ln();
        let assign14560_e12312: f64 = (var_phitd * assign14560_e12311);
        (assign14560_e12312,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign14560_e12314;
        var_vmaxbot_rv = 0.0;

        let (assign14570_e12321,) = {
    if ((var_guard182 != 0.0) && (var_guard191 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign14570_e12321;
        var_vmaxbot_rv = 0.0;

        let assign14580_e12324: f64 = (var_idsatsti_d * var_lsdrain_i);
        let assign14580_e12326: f64 = if assign14580_e12324 > 0.0 { 1.0 } else { 0.0 };
        var_guard192 = assign14580_e12326;
        var_guard192_rv = 0.0;

        let (assign14590_e12341,) = {
    if ((var_guard182 != 0.0) && (var_guard192 != 0.0)) {
        let assign14590_e12334: f64 = (var_idsatsti_d * var_lsdrain_i);
        let assign14590_e12335: f64 = (p.p822 / assign14590_e12334);
        let assign14590_e12337: f64 = (assign14590_e12335 + 1.0);
        let assign14590_e12338: f64 = (assign14590_e12337).ln();
        let assign14590_e12339: f64 = (var_phitd * assign14590_e12338);
        (assign14590_e12339,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign14590_e12341;
        var_vmaxsti_rv = 0.0;

        let (assign14600_e12348,) = {
    if ((var_guard182 != 0.0) && (var_guard192 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign14600_e12348;
        var_vmaxsti_rv = 0.0;

        let assign14610_e12351: f64 = (var_idsatgat_d * var_lgdrain_i);
        let assign14610_e12353: f64 = if assign14610_e12351 > 0.0 { 1.0 } else { 0.0 };
        var_guard193 = assign14610_e12353;
        var_guard193_rv = 0.0;

        let (assign14620_e12368,) = {
    if ((var_guard182 != 0.0) && (var_guard193 != 0.0)) {
        let assign14620_e12361: f64 = (var_idsatgat_d * var_lgdrain_i);
        let assign14620_e12362: f64 = (p.p822 / assign14620_e12361);
        let assign14620_e12364: f64 = (assign14620_e12362 + 1.0);
        let assign14620_e12365: f64 = (assign14620_e12364).ln();
        let assign14620_e12366: f64 = (var_phitd * assign14620_e12365);
        (assign14620_e12366,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign14620_e12368;
        var_vmaxgat_rv = 0.0;

        let (assign14630_e12375,) = {
    if ((var_guard182 != 0.0) && (var_guard193 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign14630_e12375;
        var_vmaxgat_rv = 0.0;

        let (assign14640_e12383,) = {
    if (var_guard182 != 0.0) {
        let assign14640_e12379: f64 = (var_vmaxbot).min(var_vmaxsti);
        let assign14640_e12381: f64 = (assign14640_e12379).min(var_vmaxgat);
        (assign14640_e12381,)
    } else {
        (var_vmax_d,)
    }
};
        var_vmax_d = assign14640_e12383;
        var_vmax_d_rv = 0.0;

        let assign14650_e12386: f64 = (var_vmax_d * var_phitdinv);
        let assign14650_e12387: f64 = (assign14650_e12386).abs();
        let assign14650_e12389: f64 = if assign14650_e12387 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard194 = assign14650_e12389;
        var_guard194_rv = 0.0;

        let (assign14660_e12398,) = {
    if ((var_guard182 != 0.0) && (var_guard194 != 0.0)) {
        let assign14660_e12395: f64 = (var_vmax_d * var_phitdinv);
        let assign14660_e12396: f64 = (assign14660_e12395).exp();
        (assign14660_e12396,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign14660_e12398;
        var_exp_vmax_over_phitd_d_rv = 0.0;

        let assign14670_e12401: f64 = (var_vmax_d * var_phitdinv);
        let assign14670_e12403: f64 = if assign14670_e12401 < 0.0 { 1.0 } else { 0.0 };
        var_guard195 = assign14670_e12403;
        var_guard195_rv = 0.0;

        let (assign14680_e12443,) = {
    if (((var_guard182 != 0.0) && (var_guard194 == 0.0)) && (var_guard195 != 0.0)) {
        let assign14680_e12413: f64 = (-230.25850929940458);
        let assign14680_e12416: f64 = (var_vmax_d * var_phitdinv);
        let assign14680_e12417: f64 = (assign14680_e12413 - assign14680_e12416);
        let assign14680_e12421: f64 = (-230.25850929940458);
        let assign14680_e12424: f64 = (var_vmax_d * var_phitdinv);
        let assign14680_e12425: f64 = (assign14680_e12421 - assign14680_e12424);
        let assign14680_e12428: f64 = (-230.25850929940458);
        let assign14680_e12431: f64 = (var_vmax_d * var_phitdinv);
        let assign14680_e12432: f64 = (assign14680_e12428 - assign14680_e12431);
        let assign14680_e12434: f64 = (assign14680_e12432 * 0.3333333333333333);
        let assign14680_e12435: f64 = (1.0 + assign14680_e12434);
        let assign14680_e12436: f64 = (assign14680_e12425 * assign14680_e12435);
        let assign14680_e12437: f64 = (0.5 * assign14680_e12436);
        let assign14680_e12438: f64 = (1.0 + assign14680_e12437);
        let assign14680_e12439: f64 = (assign14680_e12417 * assign14680_e12438);
        let assign14680_e12440: f64 = (1.0 + assign14680_e12439);
        let assign14680_e12441: f64 = (1e-100 / assign14680_e12440);
        (assign14680_e12441,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign14680_e12443;
        var_exp_vmax_over_phitd_d_rv = 0.0;

        *var_exp_vmax_over_phitd_d_slot = var_exp_vmax_over_phitd_d;
        *var_exp_vmax_over_phitd_d_rv_slot = var_exp_vmax_over_phitd_d_rv;
        *var_exp_vmax_over_phitd_s_slot = var_exp_vmax_over_phitd_s;
        *var_exp_vmax_over_phitd_s_rv_slot = var_exp_vmax_over_phitd_s_rv;
        *var_guard186_slot = var_guard186;
        *var_guard186_rv_slot = var_guard186_rv;
        *var_guard187_slot = var_guard187;
        *var_guard187_rv_slot = var_guard187_rv;
        *var_guard188_slot = var_guard188;
        *var_guard188_rv_slot = var_guard188_rv;
        *var_guard189_slot = var_guard189;
        *var_guard189_rv_slot = var_guard189_rv;
        *var_guard190_slot = var_guard190;
        *var_guard190_rv_slot = var_guard190_rv;
        *var_guard191_slot = var_guard191;
        *var_guard191_rv_slot = var_guard191_rv;
        *var_guard192_slot = var_guard192;
        *var_guard192_rv_slot = var_guard192_rv;
        *var_guard193_slot = var_guard193;
        *var_guard193_rv_slot = var_guard193_rv;
        *var_guard194_slot = var_guard194;
        *var_guard194_rv_slot = var_guard194_rv;
        *var_guard195_slot = var_guard195;
        *var_guard195_rv_slot = var_guard195_rv;
        *var_pbot2_slot = var_pbot2;
        *var_pbot2_rv_slot = var_pbot2_rv;
        *var_pgat2_slot = var_pgat2;
        *var_pgat2_rv_slot = var_pgat2_rv;
        *var_pmax_slot = var_pmax;
        *var_pmax_rv_slot = var_pmax_rv;
        *var_psti2_slot = var_psti2;
        *var_psti2_rv_slot = var_psti2_rv;
        *var_vbbtlim_s_slot = var_vbbtlim_s;
        *var_vbbtlim_s_rv_slot = var_vbbtlim_s_rv;
        *var_vbibot2_slot = var_vbibot2;
        *var_vbibot2_rv_slot = var_vbibot2_rv;
        *var_vbibot2r_slot = var_vbibot2r;
        *var_vbibot2r_rv_slot = var_vbibot2r_rv;
        *var_vbigat2_slot = var_vbigat2;
        *var_vbigat2_rv_slot = var_vbigat2_rv;
        *var_vbigat2r_slot = var_vbigat2r;
        *var_vbigat2r_rv_slot = var_vbigat2r_rv;
        *var_vbimin_s_slot = var_vbimin_s;
        *var_vbimin_s_rv_slot = var_vbimin_s_rv;
        *var_vbisti2_slot = var_vbisti2;
        *var_vbisti2_rv_slot = var_vbisti2_rv;
        *var_vbisti2r_slot = var_vbisti2r;
        *var_vbisti2r_rv_slot = var_vbisti2r_rv;
        *var_vch_s_slot = var_vch_s;
        *var_vch_s_rv_slot = var_vch_s_rv;
        *var_vfmin_s_slot = var_vfmin_s;
        *var_vfmin_s_rv_slot = var_vfmin_s_rv;
        *var_vmax_d_slot = var_vmax_d;
        *var_vmax_d_rv_slot = var_vmax_d_rv;
        *var_vmax_s_slot = var_vmax_s;
        *var_vmax_s_rv_slot = var_vmax_s_rv;
        *var_vmaxbot_slot = var_vmaxbot;
        *var_vmaxbot_rv_slot = var_vmaxbot_rv;
        *var_vmaxgat_slot = var_vmaxgat;
        *var_vmaxgat_rv_slot = var_vmaxgat_rv;
        *var_vmaxsti_slot = var_vmaxsti;
        *var_vmaxsti_rv_slot = var_vmaxsti_rv;
    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_chnl_type: f64,
        var_cjobot: f64,
        var_cjobot_d: f64,
        var_cjogat: f64,
        var_cjogat_d: f64,
        var_cjosti: f64,
        var_cjosti_d: f64,
        var_fjunqd_i: f64,
        var_guard182: f64,
        var_guard194: f64,
        var_guard195: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_pbotd_i: f64,
        var_pgatd_i: f64,
        var_phitdinv: f64,
        var_pstid_i: f64,
        var_swjunexp_i: f64,
        var_vbibot_d: f64,
        var_vbigat_d: f64,
        var_vbirbotd_i: f64,
        var_vbirgatd_i: f64,
        var_vbirstid_i: f64,
        var_vbisti_d: f64,
        var_vmax_d: f64,
        var_exp_vmax_over_phitd_d_slot: &mut f64,
        var_exp_vmax_over_phitd_d_rv_slot: &mut f64,
        var_guard1113_slot: &mut f64,
        var_guard1113_rv_slot: &mut f64,
        var_guard196_slot: &mut f64,
        var_guard196_rv_slot: &mut f64,
        var_guard197_slot: &mut f64,
        var_guard197_rv_slot: &mut f64,
        var_guard198_slot: &mut f64,
        var_guard198_rv_slot: &mut f64,
        var_guard199_slot: &mut f64,
        var_guard199_rv_slot: &mut f64,
        var_guard534_slot: &mut f64,
        var_guard534_rv_slot: &mut f64,
        var_guard535_slot: &mut f64,
        var_guard535_rv_slot: &mut f64,
        var_guard536_slot: &mut f64,
        var_guard536_rv_slot: &mut f64,
        var_guard824_slot: &mut f64,
        var_guard824_rv_slot: &mut f64,
        var_guard825_slot: &mut f64,
        var_guard825_rv_slot: &mut f64,
        var_guard826_slot: &mut f64,
        var_guard826_rv_slot: &mut f64,
        var_pbot2_slot: &mut f64,
        var_pbot2_rv_slot: &mut f64,
        var_pd_slot: &mut f64,
        var_pd_dn12_slot: &mut f64,
        var_pd_dn13_slot: &mut f64,
        var_pd_dn14_slot: &mut f64,
        var_pd_dn15_slot: &mut f64,
        var_pd_dn16_slot: &mut f64,
        var_pd_dn17_slot: &mut f64,
        var_pd_dn18_slot: &mut f64,
        var_pd_dn19_slot: &mut f64,
        var_pd_dn20_slot: &mut f64,
        var_pd_dn5_slot: &mut f64,
        var_pd_dn6_slot: &mut f64,
        var_pd_dn7_slot: &mut f64,
        var_pd_dn8_slot: &mut f64,
        var_pd_rv_slot: &mut f64,
        var_pgat2_slot: &mut f64,
        var_pgat2_rv_slot: &mut f64,
        var_pmax_slot: &mut f64,
        var_pmax_rv_slot: &mut f64,
        var_psti2_slot: &mut f64,
        var_psti2_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn12_slot: &mut f64,
        var_temp1_dn13_slot: &mut f64,
        var_temp1_dn14_slot: &mut f64,
        var_temp1_dn15_slot: &mut f64,
        var_temp1_dn16_slot: &mut f64,
        var_temp1_dn17_slot: &mut f64,
        var_temp1_dn18_slot: &mut f64,
        var_temp1_dn19_slot: &mut f64,
        var_temp1_dn20_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn12_slot: &mut f64,
        var_temp2_dn13_slot: &mut f64,
        var_temp2_dn14_slot: &mut f64,
        var_temp2_dn15_slot: &mut f64,
        var_temp2_dn16_slot: &mut f64,
        var_temp2_dn17_slot: &mut f64,
        var_temp2_dn18_slot: &mut f64,
        var_temp2_dn19_slot: &mut f64,
        var_temp2_dn20_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp__blk1038_slot: &mut f64,
        var_temp__blk1038_dn12_slot: &mut f64,
        var_temp__blk1038_dn13_slot: &mut f64,
        var_temp__blk1038_dn14_slot: &mut f64,
        var_temp__blk1038_dn15_slot: &mut f64,
        var_temp__blk1038_dn16_slot: &mut f64,
        var_temp__blk1038_dn17_slot: &mut f64,
        var_temp__blk1038_dn18_slot: &mut f64,
        var_temp__blk1038_dn19_slot: &mut f64,
        var_temp__blk1038_dn20_slot: &mut f64,
        var_temp__blk1038_dn5_slot: &mut f64,
        var_temp__blk1038_dn6_slot: &mut f64,
        var_temp__blk1038_dn7_slot: &mut f64,
        var_temp__blk1038_dn8_slot: &mut f64,
        var_temp__blk1038_rv_slot: &mut f64,
        var_vbbtlim_d_slot: &mut f64,
        var_vbbtlim_d_rv_slot: &mut f64,
        var_vbibot2_slot: &mut f64,
        var_vbibot2_rv_slot: &mut f64,
        var_vbibot2r_slot: &mut f64,
        var_vbibot2r_rv_slot: &mut f64,
        var_vbigat2_slot: &mut f64,
        var_vbigat2_rv_slot: &mut f64,
        var_vbigat2r_slot: &mut f64,
        var_vbigat2r_rv_slot: &mut f64,
        var_vbimin_d_slot: &mut f64,
        var_vbimin_d_rv_slot: &mut f64,
        var_vbisti2_slot: &mut f64,
        var_vbisti2_rv_slot: &mut f64,
        var_vbisti2r_slot: &mut f64,
        var_vbisti2r_rv_slot: &mut f64,
        var_vch_d_slot: &mut f64,
        var_vch_d_rv_slot: &mut f64,
        var_vfmin_d_slot: &mut f64,
        var_vfmin_d_rv_slot: &mut f64,
        var_ym_slot: &mut f64,
        var_ym_dn12_slot: &mut f64,
        var_ym_dn13_slot: &mut f64,
        var_ym_dn14_slot: &mut f64,
        var_ym_dn15_slot: &mut f64,
        var_ym_dn16_slot: &mut f64,
        var_ym_dn17_slot: &mut f64,
        var_ym_dn18_slot: &mut f64,
        var_ym_dn19_slot: &mut f64,
        var_ym_dn20_slot: &mut f64,
        var_ym_dn5_slot: &mut f64,
        var_ym_dn6_slot: &mut f64,
        var_ym_dn7_slot: &mut f64,
        var_ym_dn8_slot: &mut f64,
        var_ym_rv_slot: &mut f64,
        var_zflagbot_d_slot: &mut f64,
        var_zflagbot_d_rv_slot: &mut f64,
        var_zflagbot_s_slot: &mut f64,
        var_zflagbot_s_rv_slot: &mut f64,
        var_zflaggat_d_slot: &mut f64,
        var_zflaggat_d_rv_slot: &mut f64,
        var_zflaggat_s_slot: &mut f64,
        var_zflaggat_s_rv_slot: &mut f64,
        var_zflagsti_d_slot: &mut f64,
        var_zflagsti_d_rv_slot: &mut f64,
        var_zflagsti_s_slot: &mut f64,
        var_zflagsti_s_rv_slot: &mut f64,
        var_zfrac_slot: &mut f64,
        var_zfrac_rv_slot: &mut f64,
    ) {
        let mut var_exp_vmax_over_phitd_d: f64 = *var_exp_vmax_over_phitd_d_slot;
        let mut var_exp_vmax_over_phitd_d_rv: f64 = *var_exp_vmax_over_phitd_d_rv_slot;
        let mut var_guard1113: f64 = *var_guard1113_slot;
        let mut var_guard1113_rv: f64 = *var_guard1113_rv_slot;
        let mut var_guard196: f64 = *var_guard196_slot;
        let mut var_guard196_rv: f64 = *var_guard196_rv_slot;
        let mut var_guard197: f64 = *var_guard197_slot;
        let mut var_guard197_rv: f64 = *var_guard197_rv_slot;
        let mut var_guard198: f64 = *var_guard198_slot;
        let mut var_guard198_rv: f64 = *var_guard198_rv_slot;
        let mut var_guard199: f64 = *var_guard199_slot;
        let mut var_guard199_rv: f64 = *var_guard199_rv_slot;
        let mut var_guard534: f64 = *var_guard534_slot;
        let mut var_guard534_rv: f64 = *var_guard534_rv_slot;
        let mut var_guard535: f64 = *var_guard535_slot;
        let mut var_guard535_rv: f64 = *var_guard535_rv_slot;
        let mut var_guard536: f64 = *var_guard536_slot;
        let mut var_guard536_rv: f64 = *var_guard536_rv_slot;
        let mut var_guard824: f64 = *var_guard824_slot;
        let mut var_guard824_rv: f64 = *var_guard824_rv_slot;
        let mut var_guard825: f64 = *var_guard825_slot;
        let mut var_guard825_rv: f64 = *var_guard825_rv_slot;
        let mut var_guard826: f64 = *var_guard826_slot;
        let mut var_guard826_rv: f64 = *var_guard826_rv_slot;
        let mut var_pbot2: f64 = *var_pbot2_slot;
        let mut var_pbot2_rv: f64 = *var_pbot2_rv_slot;
        let mut var_pd: f64 = *var_pd_slot;
        let mut var_pd_dn12: f64 = *var_pd_dn12_slot;
        let mut var_pd_dn13: f64 = *var_pd_dn13_slot;
        let mut var_pd_dn14: f64 = *var_pd_dn14_slot;
        let mut var_pd_dn15: f64 = *var_pd_dn15_slot;
        let mut var_pd_dn16: f64 = *var_pd_dn16_slot;
        let mut var_pd_dn17: f64 = *var_pd_dn17_slot;
        let mut var_pd_dn18: f64 = *var_pd_dn18_slot;
        let mut var_pd_dn19: f64 = *var_pd_dn19_slot;
        let mut var_pd_dn20: f64 = *var_pd_dn20_slot;
        let mut var_pd_dn5: f64 = *var_pd_dn5_slot;
        let mut var_pd_dn6: f64 = *var_pd_dn6_slot;
        let mut var_pd_dn7: f64 = *var_pd_dn7_slot;
        let mut var_pd_dn8: f64 = *var_pd_dn8_slot;
        let mut var_pd_rv: f64 = *var_pd_rv_slot;
        let mut var_pgat2: f64 = *var_pgat2_slot;
        let mut var_pgat2_rv: f64 = *var_pgat2_rv_slot;
        let mut var_pmax: f64 = *var_pmax_slot;
        let mut var_pmax_rv: f64 = *var_pmax_rv_slot;
        let mut var_psti2: f64 = *var_psti2_slot;
        let mut var_psti2_rv: f64 = *var_psti2_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn12: f64 = *var_temp1_dn12_slot;
        let mut var_temp1_dn13: f64 = *var_temp1_dn13_slot;
        let mut var_temp1_dn14: f64 = *var_temp1_dn14_slot;
        let mut var_temp1_dn15: f64 = *var_temp1_dn15_slot;
        let mut var_temp1_dn16: f64 = *var_temp1_dn16_slot;
        let mut var_temp1_dn17: f64 = *var_temp1_dn17_slot;
        let mut var_temp1_dn18: f64 = *var_temp1_dn18_slot;
        let mut var_temp1_dn19: f64 = *var_temp1_dn19_slot;
        let mut var_temp1_dn20: f64 = *var_temp1_dn20_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn12: f64 = *var_temp2_dn12_slot;
        let mut var_temp2_dn13: f64 = *var_temp2_dn13_slot;
        let mut var_temp2_dn14: f64 = *var_temp2_dn14_slot;
        let mut var_temp2_dn15: f64 = *var_temp2_dn15_slot;
        let mut var_temp2_dn16: f64 = *var_temp2_dn16_slot;
        let mut var_temp2_dn17: f64 = *var_temp2_dn17_slot;
        let mut var_temp2_dn18: f64 = *var_temp2_dn18_slot;
        let mut var_temp2_dn19: f64 = *var_temp2_dn19_slot;
        let mut var_temp2_dn20: f64 = *var_temp2_dn20_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp__blk1038: f64 = *var_temp__blk1038_slot;
        let mut var_temp__blk1038_dn12: f64 = *var_temp__blk1038_dn12_slot;
        let mut var_temp__blk1038_dn13: f64 = *var_temp__blk1038_dn13_slot;
        let mut var_temp__blk1038_dn14: f64 = *var_temp__blk1038_dn14_slot;
        let mut var_temp__blk1038_dn15: f64 = *var_temp__blk1038_dn15_slot;
        let mut var_temp__blk1038_dn16: f64 = *var_temp__blk1038_dn16_slot;
        let mut var_temp__blk1038_dn17: f64 = *var_temp__blk1038_dn17_slot;
        let mut var_temp__blk1038_dn18: f64 = *var_temp__blk1038_dn18_slot;
        let mut var_temp__blk1038_dn19: f64 = *var_temp__blk1038_dn19_slot;
        let mut var_temp__blk1038_dn20: f64 = *var_temp__blk1038_dn20_slot;
        let mut var_temp__blk1038_dn5: f64 = *var_temp__blk1038_dn5_slot;
        let mut var_temp__blk1038_dn6: f64 = *var_temp__blk1038_dn6_slot;
        let mut var_temp__blk1038_dn7: f64 = *var_temp__blk1038_dn7_slot;
        let mut var_temp__blk1038_dn8: f64 = *var_temp__blk1038_dn8_slot;
        let mut var_temp__blk1038_rv: f64 = *var_temp__blk1038_rv_slot;
        let mut var_vbbtlim_d: f64 = *var_vbbtlim_d_slot;
        let mut var_vbbtlim_d_rv: f64 = *var_vbbtlim_d_rv_slot;
        let mut var_vbibot2: f64 = *var_vbibot2_slot;
        let mut var_vbibot2_rv: f64 = *var_vbibot2_rv_slot;
        let mut var_vbibot2r: f64 = *var_vbibot2r_slot;
        let mut var_vbibot2r_rv: f64 = *var_vbibot2r_rv_slot;
        let mut var_vbigat2: f64 = *var_vbigat2_slot;
        let mut var_vbigat2_rv: f64 = *var_vbigat2_rv_slot;
        let mut var_vbigat2r: f64 = *var_vbigat2r_slot;
        let mut var_vbigat2r_rv: f64 = *var_vbigat2r_rv_slot;
        let mut var_vbimin_d: f64 = *var_vbimin_d_slot;
        let mut var_vbimin_d_rv: f64 = *var_vbimin_d_rv_slot;
        let mut var_vbisti2: f64 = *var_vbisti2_slot;
        let mut var_vbisti2_rv: f64 = *var_vbisti2_rv_slot;
        let mut var_vbisti2r: f64 = *var_vbisti2r_slot;
        let mut var_vbisti2r_rv: f64 = *var_vbisti2r_rv_slot;
        let mut var_vch_d: f64 = *var_vch_d_slot;
        let mut var_vch_d_rv: f64 = *var_vch_d_rv_slot;
        let mut var_vfmin_d: f64 = *var_vfmin_d_slot;
        let mut var_vfmin_d_rv: f64 = *var_vfmin_d_rv_slot;
        let mut var_ym: f64 = *var_ym_slot;
        let mut var_ym_dn12: f64 = *var_ym_dn12_slot;
        let mut var_ym_dn13: f64 = *var_ym_dn13_slot;
        let mut var_ym_dn14: f64 = *var_ym_dn14_slot;
        let mut var_ym_dn15: f64 = *var_ym_dn15_slot;
        let mut var_ym_dn16: f64 = *var_ym_dn16_slot;
        let mut var_ym_dn17: f64 = *var_ym_dn17_slot;
        let mut var_ym_dn18: f64 = *var_ym_dn18_slot;
        let mut var_ym_dn19: f64 = *var_ym_dn19_slot;
        let mut var_ym_dn20: f64 = *var_ym_dn20_slot;
        let mut var_ym_dn5: f64 = *var_ym_dn5_slot;
        let mut var_ym_dn6: f64 = *var_ym_dn6_slot;
        let mut var_ym_dn7: f64 = *var_ym_dn7_slot;
        let mut var_ym_dn8: f64 = *var_ym_dn8_slot;
        let mut var_ym_rv: f64 = *var_ym_rv_slot;
        let mut var_zflagbot_d: f64 = *var_zflagbot_d_slot;
        let mut var_zflagbot_d_rv: f64 = *var_zflagbot_d_rv_slot;
        let mut var_zflagbot_s: f64 = *var_zflagbot_s_slot;
        let mut var_zflagbot_s_rv: f64 = *var_zflagbot_s_rv_slot;
        let mut var_zflaggat_d: f64 = *var_zflaggat_d_slot;
        let mut var_zflaggat_d_rv: f64 = *var_zflaggat_d_rv_slot;
        let mut var_zflaggat_s: f64 = *var_zflaggat_s_slot;
        let mut var_zflaggat_s_rv: f64 = *var_zflaggat_s_rv_slot;
        let mut var_zflagsti_d: f64 = *var_zflagsti_d_slot;
        let mut var_zflagsti_d_rv: f64 = *var_zflagsti_d_rv_slot;
        let mut var_zflagsti_s: f64 = *var_zflagsti_s_slot;
        let mut var_zflagsti_s_rv: f64 = *var_zflagsti_s_rv_slot;
        let mut var_zfrac: f64 = *var_zfrac_slot;
        let mut var_zfrac_rv: f64 = *var_zfrac_rv_slot;

        let (assign14690_e12481,) = {
    if (((var_guard182 != 0.0) && (var_guard194 == 0.0)) && (var_guard195 == 0.0)) {
        let assign14690_e12455: f64 = (var_vmax_d * var_phitdinv);
        let assign14690_e12457: f64 = (assign14690_e12455 - 230.25850929940458);
        let assign14690_e12462: f64 = (var_vmax_d * var_phitdinv);
        let assign14690_e12464: f64 = (assign14690_e12462 - 230.25850929940458);
        let assign14690_e12468: f64 = (var_vmax_d * var_phitdinv);
        let assign14690_e12470: f64 = (assign14690_e12468 - 230.25850929940458);
        let assign14690_e12472: f64 = (assign14690_e12470 * 0.3333333333333333);
        let assign14690_e12473: f64 = (1.0 + assign14690_e12472);
        let assign14690_e12474: f64 = (assign14690_e12464 * assign14690_e12473);
        let assign14690_e12475: f64 = (0.5 * assign14690_e12474);
        let assign14690_e12476: f64 = (1.0 + assign14690_e12475);
        let assign14690_e12477: f64 = (assign14690_e12457 * assign14690_e12476);
        let assign14690_e12478: f64 = (1.0 + assign14690_e12477);
        let assign14690_e12479: f64 = (1e100 * assign14690_e12478);
        (assign14690_e12479,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign14690_e12481;
        var_exp_vmax_over_phitd_d_rv = 0.0;

        let (assign14700_e12485,) = {
    if (var_guard182 != 0.0) {
        (var_vbibot_d,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign14700_e12485;
        var_vbibot2_rv = 0.0;

        let (assign14710_e12489,) = {
    if (var_guard182 != 0.0) {
        (var_vbisti_d,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign14710_e12489;
        var_vbisti2_rv = 0.0;

        let (assign14720_e12493,) = {
    if (var_guard182 != 0.0) {
        (var_vbigat_d,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign14720_e12493;
        var_vbigat2_rv = 0.0;

        let (assign14730_e12497,) = {
    if (var_guard182 != 0.0) {
        (var_pbotd_i,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign14730_e12497;
        var_pbot2_rv = 0.0;

        let (assign14740_e12501,) = {
    if (var_guard182 != 0.0) {
        (var_pstid_i,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign14740_e12501;
        var_psti2_rv = 0.0;

        let (assign14750_e12505,) = {
    if (var_guard182 != 0.0) {
        (var_pgatd_i,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign14750_e12505;
        var_pgat2_rv = 0.0;

        let (assign14760_e12509,) = {
    if (var_guard182 != 0.0) {
        (var_vbirbotd_i,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign14760_e12509;
        var_vbibot2r_rv = 0.0;

        let (assign14770_e12513,) = {
    if (var_guard182 != 0.0) {
        (var_vbirstid_i,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign14770_e12513;
        var_vbisti2r_rv = 0.0;

        let (assign14780_e12517,) = {
    if (var_guard182 != 0.0) {
        (var_vbirgatd_i,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign14780_e12517;
        var_vbigat2r_rv = 0.0;

        let assign14790_e12520: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard196 = assign14790_e12520;
        var_guard196_rv = 0.0;

        let (assign14800_e12528,) = {
    if ((var_guard182 != 0.0) && (var_guard196 != 0.0)) {
        let assign14800_e12526: f64 = (var_vbisti_d + var_vbigat_d);
        (assign14800_e12526,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign14800_e12528;
        var_vbibot2_rv = 0.0;

        let (assign14810_e12538,) = {
    if ((var_guard182 != 0.0) && (var_guard196 != 0.0)) {
        let assign14810_e12535: f64 = (var_pstid_i).min(var_pgatd_i);
        let assign14810_e12536: f64 = (0.9 * assign14810_e12535);
        (assign14810_e12536,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign14810_e12538;
        var_pbot2_rv = 0.0;

        let (assign14820_e12546,) = {
    if ((var_guard182 != 0.0) && (var_guard196 != 0.0)) {
        let assign14820_e12544: f64 = (var_vbirstid_i + var_vbirgatd_i);
        (assign14820_e12544,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign14820_e12546;
        var_vbibot2r_rv = 0.0;

        let assign14830_e12549: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard197 = assign14830_e12549;
        var_guard197_rv = 0.0;

        let (assign14840_e12557,) = {
    if ((var_guard182 != 0.0) && (var_guard197 != 0.0)) {
        let assign14840_e12555: f64 = (var_vbibot_d + var_vbigat_d);
        (assign14840_e12555,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign14840_e12557;
        var_vbisti2_rv = 0.0;

        let (assign14850_e12567,) = {
    if ((var_guard182 != 0.0) && (var_guard197 != 0.0)) {
        let assign14850_e12564: f64 = (var_pbotd_i).min(var_pgatd_i);
        let assign14850_e12565: f64 = (0.9 * assign14850_e12564);
        (assign14850_e12565,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign14850_e12567;
        var_psti2_rv = 0.0;

        let (assign14860_e12575,) = {
    if ((var_guard182 != 0.0) && (var_guard197 != 0.0)) {
        let assign14860_e12573: f64 = (var_vbirbotd_i + var_vbirgatd_i);
        (assign14860_e12573,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign14860_e12575;
        var_vbisti2r_rv = 0.0;

        let assign14870_e12578: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard198 = assign14870_e12578;
        var_guard198_rv = 0.0;

        let (assign14880_e12586,) = {
    if ((var_guard182 != 0.0) && (var_guard198 != 0.0)) {
        let assign14880_e12584: f64 = (var_vbibot_d + var_vbisti_d);
        (assign14880_e12584,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign14880_e12586;
        var_vbigat2_rv = 0.0;

        let (assign14890_e12596,) = {
    if ((var_guard182 != 0.0) && (var_guard198 != 0.0)) {
        let assign14890_e12593: f64 = (var_pbotd_i).min(var_pstid_i);
        let assign14890_e12594: f64 = (0.9 * assign14890_e12593);
        (assign14890_e12594,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign14890_e12596;
        var_pgat2_rv = 0.0;

        let (assign14900_e12604,) = {
    if ((var_guard182 != 0.0) && (var_guard198 != 0.0)) {
        let assign14900_e12602: f64 = (var_vbirbotd_i + var_vbirstid_i);
        (assign14900_e12602,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign14900_e12604;
        var_vbigat2r_rv = 0.0;

        let (assign14910_e12612,) = {
    if (var_guard182 != 0.0) {
        let assign14910_e12608: f64 = (var_vbibot2).min(var_vbisti2);
        let assign14910_e12610: f64 = (assign14910_e12608).min(var_vbigat2);
        (assign14910_e12610,)
    } else {
        (var_vbimin_d,)
    }
};
        var_vbimin_d = assign14910_e12612;
        var_vbimin_d_rv = 0.0;

        let (assign14920_e12618,) = {
    if (var_guard182 != 0.0) {
        let assign14920_e12616: f64 = (var_vbimin_d * 0.1);
        (assign14920_e12616,)
    } else {
        (var_vch_d,)
    }
};
        var_vch_d = assign14920_e12618;
        var_vch_d_rv = 0.0;

        let (assign14930_e12626,) = {
    if (var_guard182 != 0.0) {
        let assign14930_e12622: f64 = (var_pbot2).max(var_psti2);
        let assign14930_e12624: f64 = (assign14930_e12622).max(var_pgat2);
        (assign14930_e12624,)
    } else {
        (var_pmax,)
    }
};
        var_pmax = assign14930_e12626;
        var_pmax_rv = 0.0;

        let (assign14940_e12639,) = {
    if (var_guard182 != 0.0) {
        let assign14940_e12632: f64 = (-1.0);
        let assign14940_e12634: f64 = (assign14940_e12632 / var_pmax);
        let assign14940_e12635: f64 = (2.0_f64).powf(assign14940_e12634);
        let assign14940_e12636: f64 = (1.0 - assign14940_e12635);
        let assign14940_e12637: f64 = (var_vbimin_d * assign14940_e12636);
        (assign14940_e12637,)
    } else {
        (var_vfmin_d,)
    }
};
        var_vfmin_d = assign14940_e12639;
        var_vfmin_d_rv = 0.0;

        let (assign14950_e12649,) = {
    if (var_guard182 != 0.0) {
        let assign14950_e12643: f64 = (var_vbibot2r).min(var_vbisti2r);
        let assign14950_e12645: f64 = (assign14950_e12643).min(var_vbigat2r);
        let assign14950_e12647: f64 = (assign14950_e12645 - 0.05);
        (assign14950_e12647,)
    } else {
        (var_vbbtlim_d,)
    }
};
        var_vbbtlim_d = assign14950_e12649;
        var_vbbtlim_d_rv = 0.0;

        let assign14960_e12652: f64 = if var_swjunexp_i == 1.0 { 1.0 } else { 0.0 };
        var_guard199 = assign14960_e12652;
        var_guard199_rv = 0.0;

        let (assign27780_e33173,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign27780_e33162: f64 = (var_absource_i * var_cjobot);
        let assign27780_e33165: f64 = (var_lssource_i * var_cjosti);
        let assign27780_e33166: f64 = (assign27780_e33162 + assign27780_e33165);
        let assign27780_e33169: f64 = (var_lgsource_i * var_cjogat);
        let assign27780_e33170: f64 = (assign27780_e33166 + assign27780_e33169);
        let assign27780_e33171: f64 = (p.p929 * assign27780_e33170);
        (assign27780_e33171,)
    } else {
        (var_zfrac,)
    }
};
        var_zfrac = assign27780_e33173;
        var_zfrac_rv = 0.0;

        let assign27790_e33176: f64 = (var_absource_i * var_cjobot);
        let assign27790_e33178: f64 = if assign27790_e33176 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard534 = assign27790_e33178;
        var_guard534_rv = 0.0;

        let (assign27800_e33186,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard534 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagbot_s,)
    }
};
        var_zflagbot_s = assign27800_e33186;
        var_zflagbot_s_rv = 0.0;

        let assign27810_e33189: f64 = (var_lssource_i * var_cjosti);
        let assign27810_e33191: f64 = if assign27810_e33189 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard535 = assign27810_e33191;
        var_guard535_rv = 0.0;

        let (assign27820_e33199,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard535 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagsti_s,)
    }
};
        var_zflagsti_s = assign27820_e33199;
        var_zflagsti_s_rv = 0.0;

        let assign27830_e33202: f64 = (var_lgsource_i * var_cjogat);
        let assign27830_e33204: f64 = if assign27830_e33202 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard536 = assign27830_e33204;
        var_guard536_rv = 0.0;

        let (assign27840_e33212,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard536 != 0.0)) {
        (0.0,)
    } else {
        (var_zflaggat_s,)
    }
};
        var_zflaggat_s = assign27840_e33212;
        var_zflaggat_s_rv = 0.0;

        let (assign40310_e53566,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign40310_e53555: f64 = (var_abdrain_i * var_cjobot_d);
        let assign40310_e53558: f64 = (var_lsdrain_i * var_cjosti_d);
        let assign40310_e53559: f64 = (assign40310_e53555 + assign40310_e53558);
        let assign40310_e53562: f64 = (var_lgdrain_i * var_cjogat_d);
        let assign40310_e53563: f64 = (assign40310_e53559 + assign40310_e53562);
        let assign40310_e53564: f64 = (var_fjunqd_i * assign40310_e53563);
        (assign40310_e53564,)
    } else {
        (var_zfrac,)
    }
};
        var_zfrac = assign40310_e53566;
        var_zfrac_rv = 0.0;

        let assign40320_e53569: f64 = (var_abdrain_i * var_cjobot_d);
        let assign40320_e53571: f64 = if assign40320_e53569 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard824 = assign40320_e53571;
        var_guard824_rv = 0.0;

        let (assign40330_e53579,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard824 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagbot_d,)
    }
};
        var_zflagbot_d = assign40330_e53579;
        var_zflagbot_d_rv = 0.0;

        let assign40340_e53582: f64 = (var_lsdrain_i * var_cjosti_d);
        let assign40340_e53584: f64 = if assign40340_e53582 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard825 = assign40340_e53584;
        var_guard825_rv = 0.0;

        let (assign40350_e53592,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard825 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagsti_d,)
    }
};
        var_zflagsti_d = assign40350_e53592;
        var_zflagsti_d_rv = 0.0;

        let assign40360_e53595: f64 = (var_lgdrain_i * var_cjogat_d);
        let assign40360_e53597: f64 = if assign40360_e53595 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard826 = assign40360_e53597;
        var_guard826_rv = 0.0;

        let (assign40370_e53605,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard826 != 0.0)) {
        (0.0,)
    } else {
        (var_zflaggat_d,)
    }
};
        var_zflaggat_d = assign40370_e53605;
        var_zflaggat_d_rv = 0.0;

        var_temp__blk1038 = 0.0;
        var_temp__blk1038_dn5 = 0.0;
        var_temp__blk1038_dn6 = 0.0;
        var_temp__blk1038_dn7 = 0.0;
        var_temp__blk1038_dn8 = 0.0;
        var_temp__blk1038_dn12 = 0.0;
        var_temp__blk1038_dn13 = 0.0;
        var_temp__blk1038_dn14 = 0.0;
        var_temp__blk1038_dn15 = 0.0;
        var_temp__blk1038_dn16 = 0.0;
        var_temp__blk1038_dn17 = 0.0;
        var_temp__blk1038_dn18 = 0.0;
        var_temp__blk1038_dn19 = 0.0;
        var_temp__blk1038_dn20 = 0.0;
        var_temp__blk1038_rv = 0.0;

        var_temp1 = 0.0;
        var_temp1_dn5 = 0.0;
        var_temp1_dn6 = 0.0;
        var_temp1_dn7 = 0.0;
        var_temp1_dn8 = 0.0;
        var_temp1_dn12 = 0.0;
        var_temp1_dn13 = 0.0;
        var_temp1_dn14 = 0.0;
        var_temp1_dn15 = 0.0;
        var_temp1_dn16 = 0.0;
        var_temp1_dn17 = 0.0;
        var_temp1_dn18 = 0.0;
        var_temp1_dn19 = 0.0;
        var_temp1_dn20 = 0.0;
        var_temp1_rv = 0.0;

        var_temp2 = 0.0;
        var_temp2_dn5 = 0.0;
        var_temp2_dn6 = 0.0;
        var_temp2_dn7 = 0.0;
        var_temp2_dn8 = 0.0;
        var_temp2_dn12 = 0.0;
        var_temp2_dn13 = 0.0;
        var_temp2_dn14 = 0.0;
        var_temp2_dn15 = 0.0;
        var_temp2_dn16 = 0.0;
        var_temp2_dn17 = 0.0;
        var_temp2_dn18 = 0.0;
        var_temp2_dn19 = 0.0;
        var_temp2_dn20 = 0.0;
        var_temp2_rv = 0.0;

        var_pd = 1.0;
        var_pd_dn5 = 0.0;
        var_pd_dn6 = 0.0;
        var_pd_dn7 = 0.0;
        var_pd_dn8 = 0.0;
        var_pd_dn12 = 0.0;
        var_pd_dn13 = 0.0;
        var_pd_dn14 = 0.0;
        var_pd_dn15 = 0.0;
        var_pd_dn16 = 0.0;
        var_pd_dn17 = 0.0;
        var_pd_dn18 = 0.0;
        var_pd_dn19 = 0.0;
        var_pd_dn20 = 0.0;
        var_pd_rv = 0.0;

        var_ym = 0.0;
        var_ym_dn5 = 0.0;
        var_ym_dn6 = 0.0;
        var_ym_dn7 = 0.0;
        var_ym_dn8 = 0.0;
        var_ym_dn12 = 0.0;
        var_ym_dn13 = 0.0;
        var_ym_dn14 = 0.0;
        var_ym_dn15 = 0.0;
        var_ym_dn16 = 0.0;
        var_ym_dn17 = 0.0;
        var_ym_dn18 = 0.0;
        var_ym_dn19 = 0.0;
        var_ym_dn20 = 0.0;
        var_ym_rv = 0.0;

        let assign40530_e53716: f64 = 1.0;
        let assign40530_e53717: f64 = if var_chnl_type == assign40530_e53716 { 1.0 } else { 0.0 };
        var_guard1113 = assign40530_e53717;
        var_guard1113_rv = 0.0;

        *var_exp_vmax_over_phitd_d_slot = var_exp_vmax_over_phitd_d;
        *var_exp_vmax_over_phitd_d_rv_slot = var_exp_vmax_over_phitd_d_rv;
        *var_guard1113_slot = var_guard1113;
        *var_guard1113_rv_slot = var_guard1113_rv;
        *var_guard196_slot = var_guard196;
        *var_guard196_rv_slot = var_guard196_rv;
        *var_guard197_slot = var_guard197;
        *var_guard197_rv_slot = var_guard197_rv;
        *var_guard198_slot = var_guard198;
        *var_guard198_rv_slot = var_guard198_rv;
        *var_guard199_slot = var_guard199;
        *var_guard199_rv_slot = var_guard199_rv;
        *var_guard534_slot = var_guard534;
        *var_guard534_rv_slot = var_guard534_rv;
        *var_guard535_slot = var_guard535;
        *var_guard535_rv_slot = var_guard535_rv;
        *var_guard536_slot = var_guard536;
        *var_guard536_rv_slot = var_guard536_rv;
        *var_guard824_slot = var_guard824;
        *var_guard824_rv_slot = var_guard824_rv;
        *var_guard825_slot = var_guard825;
        *var_guard825_rv_slot = var_guard825_rv;
        *var_guard826_slot = var_guard826;
        *var_guard826_rv_slot = var_guard826_rv;
        *var_pbot2_slot = var_pbot2;
        *var_pbot2_rv_slot = var_pbot2_rv;
        *var_pd_slot = var_pd;
        *var_pd_dn12_slot = var_pd_dn12;
        *var_pd_dn13_slot = var_pd_dn13;
        *var_pd_dn14_slot = var_pd_dn14;
        *var_pd_dn15_slot = var_pd_dn15;
        *var_pd_dn16_slot = var_pd_dn16;
        *var_pd_dn17_slot = var_pd_dn17;
        *var_pd_dn18_slot = var_pd_dn18;
        *var_pd_dn19_slot = var_pd_dn19;
        *var_pd_dn20_slot = var_pd_dn20;
        *var_pd_dn5_slot = var_pd_dn5;
        *var_pd_dn6_slot = var_pd_dn6;
        *var_pd_dn7_slot = var_pd_dn7;
        *var_pd_dn8_slot = var_pd_dn8;
        *var_pd_rv_slot = var_pd_rv;
        *var_pgat2_slot = var_pgat2;
        *var_pgat2_rv_slot = var_pgat2_rv;
        *var_pmax_slot = var_pmax;
        *var_pmax_rv_slot = var_pmax_rv;
        *var_psti2_slot = var_psti2;
        *var_psti2_rv_slot = var_psti2_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn12_slot = var_temp1_dn12;
        *var_temp1_dn13_slot = var_temp1_dn13;
        *var_temp1_dn14_slot = var_temp1_dn14;
        *var_temp1_dn15_slot = var_temp1_dn15;
        *var_temp1_dn16_slot = var_temp1_dn16;
        *var_temp1_dn17_slot = var_temp1_dn17;
        *var_temp1_dn18_slot = var_temp1_dn18;
        *var_temp1_dn19_slot = var_temp1_dn19;
        *var_temp1_dn20_slot = var_temp1_dn20;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn12_slot = var_temp2_dn12;
        *var_temp2_dn13_slot = var_temp2_dn13;
        *var_temp2_dn14_slot = var_temp2_dn14;
        *var_temp2_dn15_slot = var_temp2_dn15;
        *var_temp2_dn16_slot = var_temp2_dn16;
        *var_temp2_dn17_slot = var_temp2_dn17;
        *var_temp2_dn18_slot = var_temp2_dn18;
        *var_temp2_dn19_slot = var_temp2_dn19;
        *var_temp2_dn20_slot = var_temp2_dn20;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp__blk1038_slot = var_temp__blk1038;
        *var_temp__blk1038_dn12_slot = var_temp__blk1038_dn12;
        *var_temp__blk1038_dn13_slot = var_temp__blk1038_dn13;
        *var_temp__blk1038_dn14_slot = var_temp__blk1038_dn14;
        *var_temp__blk1038_dn15_slot = var_temp__blk1038_dn15;
        *var_temp__blk1038_dn16_slot = var_temp__blk1038_dn16;
        *var_temp__blk1038_dn17_slot = var_temp__blk1038_dn17;
        *var_temp__blk1038_dn18_slot = var_temp__blk1038_dn18;
        *var_temp__blk1038_dn19_slot = var_temp__blk1038_dn19;
        *var_temp__blk1038_dn20_slot = var_temp__blk1038_dn20;
        *var_temp__blk1038_dn5_slot = var_temp__blk1038_dn5;
        *var_temp__blk1038_dn6_slot = var_temp__blk1038_dn6;
        *var_temp__blk1038_dn7_slot = var_temp__blk1038_dn7;
        *var_temp__blk1038_dn8_slot = var_temp__blk1038_dn8;
        *var_temp__blk1038_rv_slot = var_temp__blk1038_rv;
        *var_vbbtlim_d_slot = var_vbbtlim_d;
        *var_vbbtlim_d_rv_slot = var_vbbtlim_d_rv;
        *var_vbibot2_slot = var_vbibot2;
        *var_vbibot2_rv_slot = var_vbibot2_rv;
        *var_vbibot2r_slot = var_vbibot2r;
        *var_vbibot2r_rv_slot = var_vbibot2r_rv;
        *var_vbigat2_slot = var_vbigat2;
        *var_vbigat2_rv_slot = var_vbigat2_rv;
        *var_vbigat2r_slot = var_vbigat2r;
        *var_vbigat2r_rv_slot = var_vbigat2r_rv;
        *var_vbimin_d_slot = var_vbimin_d;
        *var_vbimin_d_rv_slot = var_vbimin_d_rv;
        *var_vbisti2_slot = var_vbisti2;
        *var_vbisti2_rv_slot = var_vbisti2_rv;
        *var_vbisti2r_slot = var_vbisti2r;
        *var_vbisti2r_rv_slot = var_vbisti2r_rv;
        *var_vch_d_slot = var_vch_d;
        *var_vch_d_rv_slot = var_vch_d_rv;
        *var_vfmin_d_slot = var_vfmin_d;
        *var_vfmin_d_rv_slot = var_vfmin_d_rv;
        *var_ym_slot = var_ym;
        *var_ym_dn12_slot = var_ym_dn12;
        *var_ym_dn13_slot = var_ym_dn13;
        *var_ym_dn14_slot = var_ym_dn14;
        *var_ym_dn15_slot = var_ym_dn15;
        *var_ym_dn16_slot = var_ym_dn16;
        *var_ym_dn17_slot = var_ym_dn17;
        *var_ym_dn18_slot = var_ym_dn18;
        *var_ym_dn19_slot = var_ym_dn19;
        *var_ym_dn20_slot = var_ym_dn20;
        *var_ym_dn5_slot = var_ym_dn5;
        *var_ym_dn6_slot = var_ym_dn6;
        *var_ym_dn7_slot = var_ym_dn7;
        *var_ym_dn8_slot = var_ym_dn8;
        *var_ym_rv_slot = var_ym_rv;
        *var_zflagbot_d_slot = var_zflagbot_d;
        *var_zflagbot_d_rv_slot = var_zflagbot_d_rv;
        *var_zflagbot_s_slot = var_zflagbot_s;
        *var_zflagbot_s_rv_slot = var_zflagbot_s_rv;
        *var_zflaggat_d_slot = var_zflaggat_d;
        *var_zflaggat_d_rv_slot = var_zflaggat_d_rv;
        *var_zflaggat_s_slot = var_zflaggat_s;
        *var_zflaggat_s_rv_slot = var_zflaggat_s_rv;
        *var_zflagsti_d_slot = var_zflagsti_d;
        *var_zflagsti_d_rv_slot = var_zflagsti_d_rv;
        *var_zflagsti_s_slot = var_zflagsti_s;
        *var_zflagsti_s_rv_slot = var_zflagsti_s_rv;
        *var_zfrac_slot = var_zfrac;
        *var_zfrac_rv_slot = var_zfrac_rv;
    }

    pub(super) fn stamp_reactive_block_23(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_aphi_dc: f64,
        var_bphi_dc: f64,
        var_gfacnud_i: f64,
        var_guard1113: f64,
        var_inv_phita: f64,
        var_phib_dc: f64,
        var_phix1_dc: f64,
        var_phix_dc: f64,
        var_sqrt_phib_dc: f64,
        var_us1: f64,
        var_us21: f64,
        var_vfb_t: f64,
        var_dvbstar_dc_slot: &mut f64,
        var_dvbstar_dc_dn12_slot: &mut f64,
        var_dvbstar_dc_dn13_slot: &mut f64,
        var_dvbstar_dc_dn14_slot: &mut f64,
        var_dvbstar_dc_dn15_slot: &mut f64,
        var_dvbstar_dc_dn16_slot: &mut f64,
        var_dvbstar_dc_dn17_slot: &mut f64,
        var_dvbstar_dc_dn18_slot: &mut f64,
        var_dvbstar_dc_dn19_slot: &mut f64,
        var_dvbstar_dc_dn20_slot: &mut f64,
        var_dvbstar_dc_dn5_slot: &mut f64,
        var_dvbstar_dc_dn6_slot: &mut f64,
        var_dvbstar_dc_dn7_slot: &mut f64,
        var_dvbstar_dc_dn8_slot: &mut f64,
        var_dvbstar_dc_rv_slot: &mut f64,
        var_guard1114_slot: &mut f64,
        var_guard1114_rv_slot: &mut f64,
        var_guard1274_slot: &mut f64,
        var_guard1274_rv_slot: &mut f64,
        var_sigvds_slot: &mut f64,
        var_sigvds_rv_slot: &mut f64,
        var_temp__blk1038_slot: &mut f64,
        var_temp__blk1038_dn12_slot: &mut f64,
        var_temp__blk1038_dn13_slot: &mut f64,
        var_temp__blk1038_dn14_slot: &mut f64,
        var_temp__blk1038_dn15_slot: &mut f64,
        var_temp__blk1038_dn16_slot: &mut f64,
        var_temp__blk1038_dn17_slot: &mut f64,
        var_temp__blk1038_dn18_slot: &mut f64,
        var_temp__blk1038_dn19_slot: &mut f64,
        var_temp__blk1038_dn20_slot: &mut f64,
        var_temp__blk1038_dn5_slot: &mut f64,
        var_temp__blk1038_dn6_slot: &mut f64,
        var_temp__blk1038_dn7_slot: &mut f64,
        var_temp__blk1038_dn8_slot: &mut f64,
        var_temp__blk1038_rv_slot: &mut f64,
        var_us_slot: &mut f64,
        var_us_dn12_slot: &mut f64,
        var_us_dn13_slot: &mut f64,
        var_us_dn14_slot: &mut f64,
        var_us_dn15_slot: &mut f64,
        var_us_dn16_slot: &mut f64,
        var_us_dn17_slot: &mut f64,
        var_us_dn18_slot: &mut f64,
        var_us_dn19_slot: &mut f64,
        var_us_dn20_slot: &mut f64,
        var_us_dn5_slot: &mut f64,
        var_us_dn6_slot: &mut f64,
        var_us_dn7_slot: &mut f64,
        var_us_dn8_slot: &mut f64,
        var_us_rv_slot: &mut f64,
        var_usnew_slot: &mut f64,
        var_usnew_dn12_slot: &mut f64,
        var_usnew_dn13_slot: &mut f64,
        var_usnew_dn14_slot: &mut f64,
        var_usnew_dn15_slot: &mut f64,
        var_usnew_dn16_slot: &mut f64,
        var_usnew_dn17_slot: &mut f64,
        var_usnew_dn18_slot: &mut f64,
        var_usnew_dn19_slot: &mut f64,
        var_usnew_dn20_slot: &mut f64,
        var_usnew_dn5_slot: &mut f64,
        var_usnew_dn6_slot: &mut f64,
        var_usnew_dn7_slot: &mut f64,
        var_usnew_dn8_slot: &mut f64,
        var_usnew_rv_slot: &mut f64,
        var_v_db_slot: &mut f64,
        var_v_db_dn6_slot: &mut f64,
        var_v_db_dn7_slot: &mut f64,
        var_v_db_dn8_slot: &mut f64,
        var_v_db_rv_slot: &mut f64,
        var_v_ds_slot: &mut f64,
        var_v_ds_dn6_slot: &mut f64,
        var_v_ds_dn7_slot: &mut f64,
        var_v_ds_rv_slot: &mut f64,
        var_v_gs_slot: &mut f64,
        var_v_gs_dn5_slot: &mut f64,
        var_v_gs_dn6_slot: &mut f64,
        var_v_gs_dn7_slot: &mut f64,
        var_v_gs_rv_slot: &mut f64,
        var_v_sb_slot: &mut f64,
        var_v_sb_dn6_slot: &mut f64,
        var_v_sb_dn7_slot: &mut f64,
        var_v_sb_dn8_slot: &mut f64,
        var_v_sb_rv_slot: &mut f64,
        var_v_xb_slot: &mut f64,
        var_v_xb_dc_tmp_slot: &mut f64,
        var_v_xb_dc_tmp_dn6_slot: &mut f64,
        var_v_xb_dc_tmp_dn7_slot: &mut f64,
        var_v_xb_dc_tmp_dn8_slot: &mut f64,
        var_v_xb_dc_tmp_rv_slot: &mut f64,
        var_v_xb_dn6_slot: &mut f64,
        var_v_xb_dn7_slot: &mut f64,
        var_v_xb_dn8_slot: &mut f64,
        var_v_xb_rv_slot: &mut f64,
        var_vdbprime_slot: &mut f64,
        var_vdbprime_dn6_slot: &mut f64,
        var_vdbprime_dn7_slot: &mut f64,
        var_vdbprime_dn8_slot: &mut f64,
        var_vdbprime_rv_slot: &mut f64,
        var_vdsx_slot: &mut f64,
        var_vdsx_dn6_slot: &mut f64,
        var_vdsx_dn7_slot: &mut f64,
        var_vdsx_rv_slot: &mut f64,
        var_vgb_slot: &mut f64,
        var_vgb_dn5_slot: &mut f64,
        var_vgb_dn6_slot: &mut f64,
        var_vgb_dn7_slot: &mut f64,
        var_vgb_dn8_slot: &mut f64,
        var_vgb_rv_slot: &mut f64,
        var_vgdprime_slot: &mut f64,
        var_vgdprime_dn5_slot: &mut f64,
        var_vgdprime_dn6_slot: &mut f64,
        var_vgdprime_dn7_slot: &mut f64,
        var_vgdprime_rv_slot: &mut f64,
        var_vgsprime_slot: &mut f64,
        var_vgsprime_dn5_slot: &mut f64,
        var_vgsprime_dn6_slot: &mut f64,
        var_vgsprime_dn7_slot: &mut f64,
        var_vgsprime_rv_slot: &mut f64,
        var_vjun_d_slot: &mut f64,
        var_vjun_d_dn11_slot: &mut f64,
        var_vjun_d_dn7_slot: &mut f64,
        var_vjun_d_rv_slot: &mut f64,
        var_vjun_s_slot: &mut f64,
        var_vjun_s_dn10_slot: &mut f64,
        var_vjun_s_dn6_slot: &mut f64,
        var_vjun_s_rv_slot: &mut f64,
        var_vmb_slot: &mut f64,
        var_vmb_dn12_slot: &mut f64,
        var_vmb_dn13_slot: &mut f64,
        var_vmb_dn14_slot: &mut f64,
        var_vmb_dn15_slot: &mut f64,
        var_vmb_dn16_slot: &mut f64,
        var_vmb_dn17_slot: &mut f64,
        var_vmb_dn18_slot: &mut f64,
        var_vmb_dn19_slot: &mut f64,
        var_vmb_dn20_slot: &mut f64,
        var_vmb_dn5_slot: &mut f64,
        var_vmb_dn6_slot: &mut f64,
        var_vmb_dn7_slot: &mut f64,
        var_vmb_dn8_slot: &mut f64,
        var_vmb_rv_slot: &mut f64,
        var_vmbnew_slot: &mut f64,
        var_vmbnew_dn12_slot: &mut f64,
        var_vmbnew_dn13_slot: &mut f64,
        var_vmbnew_dn14_slot: &mut f64,
        var_vmbnew_dn15_slot: &mut f64,
        var_vmbnew_dn16_slot: &mut f64,
        var_vmbnew_dn17_slot: &mut f64,
        var_vmbnew_dn18_slot: &mut f64,
        var_vmbnew_dn19_slot: &mut f64,
        var_vmbnew_dn20_slot: &mut f64,
        var_vmbnew_dn5_slot: &mut f64,
        var_vmbnew_dn6_slot: &mut f64,
        var_vmbnew_dn7_slot: &mut f64,
        var_vmbnew_dn8_slot: &mut f64,
        var_vmbnew_rv_slot: &mut f64,
        var_vsbprime_slot: &mut f64,
        var_vsbprime_dn6_slot: &mut f64,
        var_vsbprime_dn7_slot: &mut f64,
        var_vsbprime_dn8_slot: &mut f64,
        var_vsbprime_rv_slot: &mut f64,
        var_vsbstar_dc_slot: &mut f64,
        var_vsbstar_dc_dn12_slot: &mut f64,
        var_vsbstar_dc_dn13_slot: &mut f64,
        var_vsbstar_dc_dn14_slot: &mut f64,
        var_vsbstar_dc_dn15_slot: &mut f64,
        var_vsbstar_dc_dn16_slot: &mut f64,
        var_vsbstar_dc_dn17_slot: &mut f64,
        var_vsbstar_dc_dn18_slot: &mut f64,
        var_vsbstar_dc_dn19_slot: &mut f64,
        var_vsbstar_dc_dn20_slot: &mut f64,
        var_vsbstar_dc_dn5_slot: &mut f64,
        var_vsbstar_dc_dn6_slot: &mut f64,
        var_vsbstar_dc_dn7_slot: &mut f64,
        var_vsbstar_dc_dn8_slot: &mut f64,
        var_vsbstar_dc_rv_slot: &mut f64,
        var_vsbstar_dc_tmp_slot: &mut f64,
        var_vsbstar_dc_tmp_dn12_slot: &mut f64,
        var_vsbstar_dc_tmp_dn13_slot: &mut f64,
        var_vsbstar_dc_tmp_dn14_slot: &mut f64,
        var_vsbstar_dc_tmp_dn15_slot: &mut f64,
        var_vsbstar_dc_tmp_dn16_slot: &mut f64,
        var_vsbstar_dc_tmp_dn17_slot: &mut f64,
        var_vsbstar_dc_tmp_dn18_slot: &mut f64,
        var_vsbstar_dc_tmp_dn19_slot: &mut f64,
        var_vsbstar_dc_tmp_dn20_slot: &mut f64,
        var_vsbstar_dc_tmp_dn5_slot: &mut f64,
        var_vsbstar_dc_tmp_dn6_slot: &mut f64,
        var_vsbstar_dc_tmp_dn7_slot: &mut f64,
        var_vsbstar_dc_tmp_dn8_slot: &mut f64,
        var_vsbstar_dc_tmp_rv_slot: &mut f64,
        var_xgb_ov_slot: &mut f64,
        var_xgb_ov_dn5_slot: &mut f64,
        var_xgb_ov_dn6_slot: &mut f64,
        var_xgb_ov_dn7_slot: &mut f64,
        var_xgb_ov_dn8_slot: &mut f64,
        var_xgb_ov_rv_slot: &mut f64,
        var_xgd_ov_slot: &mut f64,
        var_xgd_ov_dn5_slot: &mut f64,
        var_xgd_ov_dn6_slot: &mut f64,
        var_xgd_ov_dn7_slot: &mut f64,
        var_xgd_ov_rv_slot: &mut f64,
        var_xgs_ov_slot: &mut f64,
        var_xgs_ov_dn5_slot: &mut f64,
        var_xgs_ov_dn6_slot: &mut f64,
        var_xgs_ov_dn7_slot: &mut f64,
        var_xgs_ov_rv_slot: &mut f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let mut var_dvbstar_dc: f64 = *var_dvbstar_dc_slot;
        let mut var_dvbstar_dc_dn12: f64 = *var_dvbstar_dc_dn12_slot;
        let mut var_dvbstar_dc_dn13: f64 = *var_dvbstar_dc_dn13_slot;
        let mut var_dvbstar_dc_dn14: f64 = *var_dvbstar_dc_dn14_slot;
        let mut var_dvbstar_dc_dn15: f64 = *var_dvbstar_dc_dn15_slot;
        let mut var_dvbstar_dc_dn16: f64 = *var_dvbstar_dc_dn16_slot;
        let mut var_dvbstar_dc_dn17: f64 = *var_dvbstar_dc_dn17_slot;
        let mut var_dvbstar_dc_dn18: f64 = *var_dvbstar_dc_dn18_slot;
        let mut var_dvbstar_dc_dn19: f64 = *var_dvbstar_dc_dn19_slot;
        let mut var_dvbstar_dc_dn20: f64 = *var_dvbstar_dc_dn20_slot;
        let mut var_dvbstar_dc_dn5: f64 = *var_dvbstar_dc_dn5_slot;
        let mut var_dvbstar_dc_dn6: f64 = *var_dvbstar_dc_dn6_slot;
        let mut var_dvbstar_dc_dn7: f64 = *var_dvbstar_dc_dn7_slot;
        let mut var_dvbstar_dc_dn8: f64 = *var_dvbstar_dc_dn8_slot;
        let mut var_dvbstar_dc_rv: f64 = *var_dvbstar_dc_rv_slot;
        let mut var_guard1114: f64 = *var_guard1114_slot;
        let mut var_guard1114_rv: f64 = *var_guard1114_rv_slot;
        let mut var_guard1274: f64 = *var_guard1274_slot;
        let mut var_guard1274_rv: f64 = *var_guard1274_rv_slot;
        let mut var_sigvds: f64 = *var_sigvds_slot;
        let mut var_sigvds_rv: f64 = *var_sigvds_rv_slot;
        let mut var_temp__blk1038: f64 = *var_temp__blk1038_slot;
        let mut var_temp__blk1038_dn12: f64 = *var_temp__blk1038_dn12_slot;
        let mut var_temp__blk1038_dn13: f64 = *var_temp__blk1038_dn13_slot;
        let mut var_temp__blk1038_dn14: f64 = *var_temp__blk1038_dn14_slot;
        let mut var_temp__blk1038_dn15: f64 = *var_temp__blk1038_dn15_slot;
        let mut var_temp__blk1038_dn16: f64 = *var_temp__blk1038_dn16_slot;
        let mut var_temp__blk1038_dn17: f64 = *var_temp__blk1038_dn17_slot;
        let mut var_temp__blk1038_dn18: f64 = *var_temp__blk1038_dn18_slot;
        let mut var_temp__blk1038_dn19: f64 = *var_temp__blk1038_dn19_slot;
        let mut var_temp__blk1038_dn20: f64 = *var_temp__blk1038_dn20_slot;
        let mut var_temp__blk1038_dn5: f64 = *var_temp__blk1038_dn5_slot;
        let mut var_temp__blk1038_dn6: f64 = *var_temp__blk1038_dn6_slot;
        let mut var_temp__blk1038_dn7: f64 = *var_temp__blk1038_dn7_slot;
        let mut var_temp__blk1038_dn8: f64 = *var_temp__blk1038_dn8_slot;
        let mut var_temp__blk1038_rv: f64 = *var_temp__blk1038_rv_slot;
        let mut var_us: f64 = *var_us_slot;
        let mut var_us_dn12: f64 = *var_us_dn12_slot;
        let mut var_us_dn13: f64 = *var_us_dn13_slot;
        let mut var_us_dn14: f64 = *var_us_dn14_slot;
        let mut var_us_dn15: f64 = *var_us_dn15_slot;
        let mut var_us_dn16: f64 = *var_us_dn16_slot;
        let mut var_us_dn17: f64 = *var_us_dn17_slot;
        let mut var_us_dn18: f64 = *var_us_dn18_slot;
        let mut var_us_dn19: f64 = *var_us_dn19_slot;
        let mut var_us_dn20: f64 = *var_us_dn20_slot;
        let mut var_us_dn5: f64 = *var_us_dn5_slot;
        let mut var_us_dn6: f64 = *var_us_dn6_slot;
        let mut var_us_dn7: f64 = *var_us_dn7_slot;
        let mut var_us_dn8: f64 = *var_us_dn8_slot;
        let mut var_us_rv: f64 = *var_us_rv_slot;
        let mut var_usnew: f64 = *var_usnew_slot;
        let mut var_usnew_dn12: f64 = *var_usnew_dn12_slot;
        let mut var_usnew_dn13: f64 = *var_usnew_dn13_slot;
        let mut var_usnew_dn14: f64 = *var_usnew_dn14_slot;
        let mut var_usnew_dn15: f64 = *var_usnew_dn15_slot;
        let mut var_usnew_dn16: f64 = *var_usnew_dn16_slot;
        let mut var_usnew_dn17: f64 = *var_usnew_dn17_slot;
        let mut var_usnew_dn18: f64 = *var_usnew_dn18_slot;
        let mut var_usnew_dn19: f64 = *var_usnew_dn19_slot;
        let mut var_usnew_dn20: f64 = *var_usnew_dn20_slot;
        let mut var_usnew_dn5: f64 = *var_usnew_dn5_slot;
        let mut var_usnew_dn6: f64 = *var_usnew_dn6_slot;
        let mut var_usnew_dn7: f64 = *var_usnew_dn7_slot;
        let mut var_usnew_dn8: f64 = *var_usnew_dn8_slot;
        let mut var_usnew_rv: f64 = *var_usnew_rv_slot;
        let mut var_v_db: f64 = *var_v_db_slot;
        let mut var_v_db_dn6: f64 = *var_v_db_dn6_slot;
        let mut var_v_db_dn7: f64 = *var_v_db_dn7_slot;
        let mut var_v_db_dn8: f64 = *var_v_db_dn8_slot;
        let mut var_v_db_rv: f64 = *var_v_db_rv_slot;
        let mut var_v_ds: f64 = *var_v_ds_slot;
        let mut var_v_ds_dn6: f64 = *var_v_ds_dn6_slot;
        let mut var_v_ds_dn7: f64 = *var_v_ds_dn7_slot;
        let mut var_v_ds_rv: f64 = *var_v_ds_rv_slot;
        let mut var_v_gs: f64 = *var_v_gs_slot;
        let mut var_v_gs_dn5: f64 = *var_v_gs_dn5_slot;
        let mut var_v_gs_dn6: f64 = *var_v_gs_dn6_slot;
        let mut var_v_gs_dn7: f64 = *var_v_gs_dn7_slot;
        let mut var_v_gs_rv: f64 = *var_v_gs_rv_slot;
        let mut var_v_sb: f64 = *var_v_sb_slot;
        let mut var_v_sb_dn6: f64 = *var_v_sb_dn6_slot;
        let mut var_v_sb_dn7: f64 = *var_v_sb_dn7_slot;
        let mut var_v_sb_dn8: f64 = *var_v_sb_dn8_slot;
        let mut var_v_sb_rv: f64 = *var_v_sb_rv_slot;
        let mut var_v_xb: f64 = *var_v_xb_slot;
        let mut var_v_xb_dc_tmp: f64 = *var_v_xb_dc_tmp_slot;
        let mut var_v_xb_dc_tmp_dn6: f64 = *var_v_xb_dc_tmp_dn6_slot;
        let mut var_v_xb_dc_tmp_dn7: f64 = *var_v_xb_dc_tmp_dn7_slot;
        let mut var_v_xb_dc_tmp_dn8: f64 = *var_v_xb_dc_tmp_dn8_slot;
        let mut var_v_xb_dc_tmp_rv: f64 = *var_v_xb_dc_tmp_rv_slot;
        let mut var_v_xb_dn6: f64 = *var_v_xb_dn6_slot;
        let mut var_v_xb_dn7: f64 = *var_v_xb_dn7_slot;
        let mut var_v_xb_dn8: f64 = *var_v_xb_dn8_slot;
        let mut var_v_xb_rv: f64 = *var_v_xb_rv_slot;
        let mut var_vdbprime: f64 = *var_vdbprime_slot;
        let mut var_vdbprime_dn6: f64 = *var_vdbprime_dn6_slot;
        let mut var_vdbprime_dn7: f64 = *var_vdbprime_dn7_slot;
        let mut var_vdbprime_dn8: f64 = *var_vdbprime_dn8_slot;
        let mut var_vdbprime_rv: f64 = *var_vdbprime_rv_slot;
        let mut var_vdsx: f64 = *var_vdsx_slot;
        let mut var_vdsx_dn6: f64 = *var_vdsx_dn6_slot;
        let mut var_vdsx_dn7: f64 = *var_vdsx_dn7_slot;
        let mut var_vdsx_rv: f64 = *var_vdsx_rv_slot;
        let mut var_vgb: f64 = *var_vgb_slot;
        let mut var_vgb_dn5: f64 = *var_vgb_dn5_slot;
        let mut var_vgb_dn6: f64 = *var_vgb_dn6_slot;
        let mut var_vgb_dn7: f64 = *var_vgb_dn7_slot;
        let mut var_vgb_dn8: f64 = *var_vgb_dn8_slot;
        let mut var_vgb_rv: f64 = *var_vgb_rv_slot;
        let mut var_vgdprime: f64 = *var_vgdprime_slot;
        let mut var_vgdprime_dn5: f64 = *var_vgdprime_dn5_slot;
        let mut var_vgdprime_dn6: f64 = *var_vgdprime_dn6_slot;
        let mut var_vgdprime_dn7: f64 = *var_vgdprime_dn7_slot;
        let mut var_vgdprime_rv: f64 = *var_vgdprime_rv_slot;
        let mut var_vgsprime: f64 = *var_vgsprime_slot;
        let mut var_vgsprime_dn5: f64 = *var_vgsprime_dn5_slot;
        let mut var_vgsprime_dn6: f64 = *var_vgsprime_dn6_slot;
        let mut var_vgsprime_dn7: f64 = *var_vgsprime_dn7_slot;
        let mut var_vgsprime_rv: f64 = *var_vgsprime_rv_slot;
        let mut var_vjun_d: f64 = *var_vjun_d_slot;
        let mut var_vjun_d_dn11: f64 = *var_vjun_d_dn11_slot;
        let mut var_vjun_d_dn7: f64 = *var_vjun_d_dn7_slot;
        let mut var_vjun_d_rv: f64 = *var_vjun_d_rv_slot;
        let mut var_vjun_s: f64 = *var_vjun_s_slot;
        let mut var_vjun_s_dn10: f64 = *var_vjun_s_dn10_slot;
        let mut var_vjun_s_dn6: f64 = *var_vjun_s_dn6_slot;
        let mut var_vjun_s_rv: f64 = *var_vjun_s_rv_slot;
        let mut var_vmb: f64 = *var_vmb_slot;
        let mut var_vmb_dn12: f64 = *var_vmb_dn12_slot;
        let mut var_vmb_dn13: f64 = *var_vmb_dn13_slot;
        let mut var_vmb_dn14: f64 = *var_vmb_dn14_slot;
        let mut var_vmb_dn15: f64 = *var_vmb_dn15_slot;
        let mut var_vmb_dn16: f64 = *var_vmb_dn16_slot;
        let mut var_vmb_dn17: f64 = *var_vmb_dn17_slot;
        let mut var_vmb_dn18: f64 = *var_vmb_dn18_slot;
        let mut var_vmb_dn19: f64 = *var_vmb_dn19_slot;
        let mut var_vmb_dn20: f64 = *var_vmb_dn20_slot;
        let mut var_vmb_dn5: f64 = *var_vmb_dn5_slot;
        let mut var_vmb_dn6: f64 = *var_vmb_dn6_slot;
        let mut var_vmb_dn7: f64 = *var_vmb_dn7_slot;
        let mut var_vmb_dn8: f64 = *var_vmb_dn8_slot;
        let mut var_vmb_rv: f64 = *var_vmb_rv_slot;
        let mut var_vmbnew: f64 = *var_vmbnew_slot;
        let mut var_vmbnew_dn12: f64 = *var_vmbnew_dn12_slot;
        let mut var_vmbnew_dn13: f64 = *var_vmbnew_dn13_slot;
        let mut var_vmbnew_dn14: f64 = *var_vmbnew_dn14_slot;
        let mut var_vmbnew_dn15: f64 = *var_vmbnew_dn15_slot;
        let mut var_vmbnew_dn16: f64 = *var_vmbnew_dn16_slot;
        let mut var_vmbnew_dn17: f64 = *var_vmbnew_dn17_slot;
        let mut var_vmbnew_dn18: f64 = *var_vmbnew_dn18_slot;
        let mut var_vmbnew_dn19: f64 = *var_vmbnew_dn19_slot;
        let mut var_vmbnew_dn20: f64 = *var_vmbnew_dn20_slot;
        let mut var_vmbnew_dn5: f64 = *var_vmbnew_dn5_slot;
        let mut var_vmbnew_dn6: f64 = *var_vmbnew_dn6_slot;
        let mut var_vmbnew_dn7: f64 = *var_vmbnew_dn7_slot;
        let mut var_vmbnew_dn8: f64 = *var_vmbnew_dn8_slot;
        let mut var_vmbnew_rv: f64 = *var_vmbnew_rv_slot;
        let mut var_vsbprime: f64 = *var_vsbprime_slot;
        let mut var_vsbprime_dn6: f64 = *var_vsbprime_dn6_slot;
        let mut var_vsbprime_dn7: f64 = *var_vsbprime_dn7_slot;
        let mut var_vsbprime_dn8: f64 = *var_vsbprime_dn8_slot;
        let mut var_vsbprime_rv: f64 = *var_vsbprime_rv_slot;
        let mut var_vsbstar_dc: f64 = *var_vsbstar_dc_slot;
        let mut var_vsbstar_dc_dn12: f64 = *var_vsbstar_dc_dn12_slot;
        let mut var_vsbstar_dc_dn13: f64 = *var_vsbstar_dc_dn13_slot;
        let mut var_vsbstar_dc_dn14: f64 = *var_vsbstar_dc_dn14_slot;
        let mut var_vsbstar_dc_dn15: f64 = *var_vsbstar_dc_dn15_slot;
        let mut var_vsbstar_dc_dn16: f64 = *var_vsbstar_dc_dn16_slot;
        let mut var_vsbstar_dc_dn17: f64 = *var_vsbstar_dc_dn17_slot;
        let mut var_vsbstar_dc_dn18: f64 = *var_vsbstar_dc_dn18_slot;
        let mut var_vsbstar_dc_dn19: f64 = *var_vsbstar_dc_dn19_slot;
        let mut var_vsbstar_dc_dn20: f64 = *var_vsbstar_dc_dn20_slot;
        let mut var_vsbstar_dc_dn5: f64 = *var_vsbstar_dc_dn5_slot;
        let mut var_vsbstar_dc_dn6: f64 = *var_vsbstar_dc_dn6_slot;
        let mut var_vsbstar_dc_dn7: f64 = *var_vsbstar_dc_dn7_slot;
        let mut var_vsbstar_dc_dn8: f64 = *var_vsbstar_dc_dn8_slot;
        let mut var_vsbstar_dc_rv: f64 = *var_vsbstar_dc_rv_slot;
        let mut var_vsbstar_dc_tmp: f64 = *var_vsbstar_dc_tmp_slot;
        let mut var_vsbstar_dc_tmp_dn12: f64 = *var_vsbstar_dc_tmp_dn12_slot;
        let mut var_vsbstar_dc_tmp_dn13: f64 = *var_vsbstar_dc_tmp_dn13_slot;
        let mut var_vsbstar_dc_tmp_dn14: f64 = *var_vsbstar_dc_tmp_dn14_slot;
        let mut var_vsbstar_dc_tmp_dn15: f64 = *var_vsbstar_dc_tmp_dn15_slot;
        let mut var_vsbstar_dc_tmp_dn16: f64 = *var_vsbstar_dc_tmp_dn16_slot;
        let mut var_vsbstar_dc_tmp_dn17: f64 = *var_vsbstar_dc_tmp_dn17_slot;
        let mut var_vsbstar_dc_tmp_dn18: f64 = *var_vsbstar_dc_tmp_dn18_slot;
        let mut var_vsbstar_dc_tmp_dn19: f64 = *var_vsbstar_dc_tmp_dn19_slot;
        let mut var_vsbstar_dc_tmp_dn20: f64 = *var_vsbstar_dc_tmp_dn20_slot;
        let mut var_vsbstar_dc_tmp_dn5: f64 = *var_vsbstar_dc_tmp_dn5_slot;
        let mut var_vsbstar_dc_tmp_dn6: f64 = *var_vsbstar_dc_tmp_dn6_slot;
        let mut var_vsbstar_dc_tmp_dn7: f64 = *var_vsbstar_dc_tmp_dn7_slot;
        let mut var_vsbstar_dc_tmp_dn8: f64 = *var_vsbstar_dc_tmp_dn8_slot;
        let mut var_vsbstar_dc_tmp_rv: f64 = *var_vsbstar_dc_tmp_rv_slot;
        let mut var_xgb_ov: f64 = *var_xgb_ov_slot;
        let mut var_xgb_ov_dn5: f64 = *var_xgb_ov_dn5_slot;
        let mut var_xgb_ov_dn6: f64 = *var_xgb_ov_dn6_slot;
        let mut var_xgb_ov_dn7: f64 = *var_xgb_ov_dn7_slot;
        let mut var_xgb_ov_dn8: f64 = *var_xgb_ov_dn8_slot;
        let mut var_xgb_ov_rv: f64 = *var_xgb_ov_rv_slot;
        let mut var_xgd_ov: f64 = *var_xgd_ov_slot;
        let mut var_xgd_ov_dn5: f64 = *var_xgd_ov_dn5_slot;
        let mut var_xgd_ov_dn6: f64 = *var_xgd_ov_dn6_slot;
        let mut var_xgd_ov_dn7: f64 = *var_xgd_ov_dn7_slot;
        let mut var_xgd_ov_rv: f64 = *var_xgd_ov_rv_slot;
        let mut var_xgs_ov: f64 = *var_xgs_ov_slot;
        let mut var_xgs_ov_dn5: f64 = *var_xgs_ov_dn5_slot;
        let mut var_xgs_ov_dn6: f64 = *var_xgs_ov_dn6_slot;
        let mut var_xgs_ov_dn7: f64 = *var_xgs_ov_dn7_slot;
        let mut var_xgs_ov_rv: f64 = *var_xgs_ov_rv_slot;

        let (assign40540_e53721, assign40540_e53721_d_n5, assign40540_e53721_d_n6, assign40540_e53721_d_n7,) = {
    if (var_guard1113 != 0.0) {
        ((nv5 - nv6), 1.0, -1.0, 0.0,)
    } else {
        (var_v_gs, var_v_gs_dn5, var_v_gs_dn6, var_v_gs_dn7,)
    }
};
        var_v_gs = assign40540_e53721;
        var_v_gs_dn5 = assign40540_e53721_d_n5;
        var_v_gs_dn6 = assign40540_e53721_d_n6;
        var_v_gs_dn7 = assign40540_e53721_d_n7;
        var_v_gs_rv = 0.0;

        let (assign40550_e53725, assign40550_e53725_d_n6, assign40550_e53725_d_n7,) = {
    if (var_guard1113 != 0.0) {
        ((nv7 - nv6), -1.0, 1.0,)
    } else {
        (var_v_ds, var_v_ds_dn6, var_v_ds_dn7,)
    }
};
        var_v_ds = assign40550_e53725;
        var_v_ds_dn6 = assign40550_e53725_d_n6;
        var_v_ds_dn7 = assign40550_e53725_d_n7;
        var_v_ds_rv = 0.0;

        let (assign40560_e53729, assign40560_e53729_d_n6, assign40560_e53729_d_n7, assign40560_e53729_d_n8,) = {
    if (var_guard1113 != 0.0) {
        ((nv6 - nv8), 1.0, 0.0, -1.0,)
    } else {
        (var_v_sb, var_v_sb_dn6, var_v_sb_dn7, var_v_sb_dn8,)
    }
};
        var_v_sb = assign40560_e53729;
        var_v_sb_dn6 = assign40560_e53729_d_n6;
        var_v_sb_dn7 = assign40560_e53729_d_n7;
        var_v_sb_dn8 = assign40560_e53729_d_n8;
        var_v_sb_rv = 0.0;

        let (assign40570_e53734, assign40570_e53734_d_n6, assign40570_e53734_d_n10,) = {
    if (var_guard1113 != 0.0) {
        let assign40570_e53732: f64 = (-(nv6 - nv10));
        (assign40570_e53732, (-1.0), 1.0,)
    } else {
        (var_vjun_s, var_vjun_s_dn6, var_vjun_s_dn10,)
    }
};
        var_vjun_s = assign40570_e53734;
        var_vjun_s_dn6 = assign40570_e53734_d_n6;
        var_vjun_s_dn10 = assign40570_e53734_d_n10;
        var_vjun_s_rv = 0.0;

        let (assign40580_e53739, assign40580_e53739_d_n7, assign40580_e53739_d_n11,) = {
    if (var_guard1113 != 0.0) {
        let assign40580_e53737: f64 = (-(nv7 - nv11));
        (assign40580_e53737, (-1.0), 1.0,)
    } else {
        (var_vjun_d, var_vjun_d_dn7, var_vjun_d_dn11,)
    }
};
        var_vjun_d = assign40580_e53739;
        var_vjun_d_dn7 = assign40580_e53739_d_n7;
        var_vjun_d_dn11 = assign40580_e53739_d_n11;
        var_vjun_d_rv = 0.0;

        let (assign40590_e53745, assign40590_e53745_d_n5, assign40590_e53745_d_n6, assign40590_e53745_d_n7,) = {
    if (var_guard1113 == 0.0) {
        let assign40590_e53743: f64 = (-(nv5 - nv6));
        (assign40590_e53743, (-1.0), 1.0, 0.0,)
    } else {
        (var_v_gs, var_v_gs_dn5, var_v_gs_dn6, var_v_gs_dn7,)
    }
};
        var_v_gs = assign40590_e53745;
        var_v_gs_dn5 = assign40590_e53745_d_n5;
        var_v_gs_dn6 = assign40590_e53745_d_n6;
        var_v_gs_dn7 = assign40590_e53745_d_n7;
        var_v_gs_rv = 0.0;

        let (assign40600_e53751, assign40600_e53751_d_n6, assign40600_e53751_d_n7,) = {
    if (var_guard1113 == 0.0) {
        let assign40600_e53749: f64 = (-(nv7 - nv6));
        (assign40600_e53749, 1.0, (-1.0),)
    } else {
        (var_v_ds, var_v_ds_dn6, var_v_ds_dn7,)
    }
};
        var_v_ds = assign40600_e53751;
        var_v_ds_dn6 = assign40600_e53751_d_n6;
        var_v_ds_dn7 = assign40600_e53751_d_n7;
        var_v_ds_rv = 0.0;

        let (assign40610_e53757, assign40610_e53757_d_n6, assign40610_e53757_d_n7, assign40610_e53757_d_n8,) = {
    if (var_guard1113 == 0.0) {
        let assign40610_e53755: f64 = (-(nv6 - nv8));
        (assign40610_e53755, (-1.0), 0.0, 1.0,)
    } else {
        (var_v_sb, var_v_sb_dn6, var_v_sb_dn7, var_v_sb_dn8,)
    }
};
        var_v_sb = assign40610_e53757;
        var_v_sb_dn6 = assign40610_e53757_d_n6;
        var_v_sb_dn7 = assign40610_e53757_d_n7;
        var_v_sb_dn8 = assign40610_e53757_d_n8;
        var_v_sb_rv = 0.0;

        let (assign40620_e53762, assign40620_e53762_d_n6, assign40620_e53762_d_n10,) = {
    if (var_guard1113 == 0.0) {
        ((nv6 - nv10), 1.0, -1.0,)
    } else {
        (var_vjun_s, var_vjun_s_dn6, var_vjun_s_dn10,)
    }
};
        var_vjun_s = assign40620_e53762;
        var_vjun_s_dn6 = assign40620_e53762_d_n6;
        var_vjun_s_dn10 = assign40620_e53762_d_n10;
        var_vjun_s_rv = 0.0;

        let (assign40630_e53767, assign40630_e53767_d_n7, assign40630_e53767_d_n11,) = {
    if (var_guard1113 == 0.0) {
        ((nv7 - nv11), 1.0, -1.0,)
    } else {
        (var_vjun_d, var_vjun_d_dn7, var_vjun_d_dn11,)
    }
};
        var_vjun_d = assign40630_e53767;
        var_vjun_d_dn7 = assign40630_e53767_d_n7;
        var_vjun_d_dn11 = assign40630_e53767_d_n11;
        var_vjun_d_rv = 0.0;

        let assign40640_e53770: f64 = (var_v_gs + var_v_sb);
        var_vgb = assign40640_e53770;
        var_vgb_dn5 = var_v_gs_dn5;
        var_vgb_dn6 = (var_v_gs_dn6 + var_v_sb_dn6);
        var_vgb_dn7 = (var_v_gs_dn7 + var_v_sb_dn7);
        var_vgb_dn8 = var_v_sb_dn8;
        var_vgb_rv = 0.0;

        var_vgsprime = var_v_gs;
        var_vgsprime_dn5 = var_v_gs_dn5;
        var_vgsprime_dn6 = var_v_gs_dn6;
        var_vgsprime_dn7 = var_v_gs_dn7;
        var_vgsprime_rv = 0.0;

        var_vsbprime = var_v_sb;
        var_vsbprime_dn6 = var_v_sb_dn6;
        var_vsbprime_dn7 = var_v_sb_dn7;
        var_vsbprime_dn8 = var_v_sb_dn8;
        var_vsbprime_rv = 0.0;

        let assign40670_e53775: f64 = (var_v_ds + var_v_sb);
        var_vdbprime = assign40670_e53775;
        var_vdbprime_dn6 = (var_v_ds_dn6 + var_v_sb_dn6);
        var_vdbprime_dn7 = (var_v_ds_dn7 + var_v_sb_dn7);
        var_vdbprime_dn8 = var_v_sb_dn8;
        var_vdbprime_rv = 0.0;

        let assign40680_e53778: f64 = (var_v_gs - var_v_ds);
        var_vgdprime = assign40680_e53778;
        var_vgdprime_dn5 = var_v_gs_dn5;
        var_vgdprime_dn6 = (var_v_gs_dn6 - var_v_ds_dn6);
        var_vgdprime_dn7 = (var_v_gs_dn7 - var_v_ds_dn7);
        var_vgdprime_rv = 0.0;

        let assign40690_e53780: f64 = (-var_vgsprime);
        let assign40690_e53782: f64 = (assign40690_e53780 * var_inv_phita);
        var_xgs_ov = assign40690_e53782;
        var_xgs_ov_dn5 = ((-var_vgsprime_dn5) * var_inv_phita);
        var_xgs_ov_dn6 = ((-var_vgsprime_dn6) * var_inv_phita);
        var_xgs_ov_dn7 = ((-var_vgsprime_dn7) * var_inv_phita);
        var_xgs_ov_rv = 0.0;

        let assign40700_e53784: f64 = (-var_vgdprime);
        let assign40700_e53786: f64 = (assign40700_e53784 * var_inv_phita);
        var_xgd_ov = assign40700_e53786;
        var_xgd_ov_dn5 = ((-var_vgdprime_dn5) * var_inv_phita);
        var_xgd_ov_dn6 = ((-var_vgdprime_dn6) * var_inv_phita);
        var_xgd_ov_dn7 = ((-var_vgdprime_dn7) * var_inv_phita);
        var_xgd_ov_rv = 0.0;

        let assign40710_e53789: f64 = (var_vgb - var_vfb_t);
        let assign40710_e53790: f64 = (-assign40710_e53789);
        let assign40710_e53792: f64 = (assign40710_e53790 * var_inv_phita);
        var_xgb_ov = assign40710_e53792;
        var_xgb_ov_dn5 = ((-var_vgb_dn5) * var_inv_phita);
        var_xgb_ov_dn6 = ((-var_vgb_dn6) * var_inv_phita);
        var_xgb_ov_dn7 = ((-var_vgb_dn7) * var_inv_phita);
        var_xgb_ov_dn8 = ((-var_vgb_dn8) * var_inv_phita);
        var_xgb_ov_rv = 0.0;

        var_sigvds = 1.0;
        var_sigvds_rv = 0.0;

        let assign40730_e53796: f64 = if var_v_ds < 0.0 { 1.0 } else { 0.0 };
        var_guard1114 = assign40730_e53796;
        var_guard1114_rv = 0.0;

        let (assign40740_e53801,) = {
    if (var_guard1114 != 0.0) {
        let assign40740_e53799: f64 = (-1.0);
        (assign40740_e53799,)
    } else {
        (var_sigvds,)
    }
};
        var_sigvds = assign40740_e53801;
        var_sigvds_rv = 0.0;

        let (assign40750_e53807, assign40750_e53807_d_n5, assign40750_e53807_d_n6, assign40750_e53807_d_n7,) = {
    if (var_guard1114 != 0.0) {
        let assign40750_e53805: f64 = (var_v_gs - var_v_ds);
        (assign40750_e53805, var_v_gs_dn5, (var_v_gs_dn6 - var_v_ds_dn6), (var_v_gs_dn7 - var_v_ds_dn7),)
    } else {
        (var_v_gs, var_v_gs_dn5, var_v_gs_dn6, var_v_gs_dn7,)
    }
};
        var_v_gs = assign40750_e53807;
        var_v_gs_dn5 = assign40750_e53807_d_n5;
        var_v_gs_dn6 = assign40750_e53807_d_n6;
        var_v_gs_dn7 = assign40750_e53807_d_n7;
        var_v_gs_rv = 0.0;

        let (assign40760_e53813, assign40760_e53813_d_n6, assign40760_e53813_d_n7, assign40760_e53813_d_n8,) = {
    if (var_guard1114 != 0.0) {
        let assign40760_e53811: f64 = (var_v_sb + var_v_ds);
        (assign40760_e53811, (var_v_sb_dn6 + var_v_ds_dn6), (var_v_sb_dn7 + var_v_ds_dn7), var_v_sb_dn8,)
    } else {
        (var_v_sb, var_v_sb_dn6, var_v_sb_dn7, var_v_sb_dn8,)
    }
};
        var_v_sb = assign40760_e53813;
        var_v_sb_dn6 = assign40760_e53813_d_n6;
        var_v_sb_dn7 = assign40760_e53813_d_n7;
        var_v_sb_dn8 = assign40760_e53813_d_n8;
        var_v_sb_rv = 0.0;

        let (assign40770_e53818, assign40770_e53818_d_n6, assign40770_e53818_d_n7,) = {
    if (var_guard1114 != 0.0) {
        let assign40770_e53816: f64 = (-var_v_ds);
        (assign40770_e53816, (-var_v_ds_dn6), (-var_v_ds_dn7),)
    } else {
        (var_v_ds, var_v_ds_dn6, var_v_ds_dn7,)
    }
};
        var_v_ds = assign40770_e53818;
        var_v_ds_dn6 = assign40770_e53818_d_n6;
        var_v_ds_dn7 = assign40770_e53818_d_n7;
        var_v_ds_rv = 0.0;

        let assign40780_e53821: f64 = (var_v_ds + var_v_sb);
        var_v_db = assign40780_e53821;
        var_v_db_dn6 = (var_v_ds_dn6 + var_v_sb_dn6);
        var_v_db_dn7 = (var_v_ds_dn7 + var_v_sb_dn7);
        var_v_db_dn8 = var_v_sb_dn8;
        var_v_db_rv = 0.0;

        let assign40790_e53824: f64 = (var_v_ds * var_v_ds);
        let assign40790_e53827: f64 = (var_v_ds * var_v_ds);
        let assign40790_e53829: f64 = (assign40790_e53827 + 0.01);
        let assign40790_e53830: f64 = (assign40790_e53829).sqrt();
        let assign40790_e53832: f64 = (assign40790_e53830 + 0.1);
        let assign40790_e53833: f64 = (assign40790_e53824 / assign40790_e53832);
        var_vdsx = assign40790_e53833;
        var_vdsx_dn6 = (((((var_v_ds_dn6 * var_v_ds) + (var_v_ds * var_v_ds_dn6)) * assign40790_e53832) - (assign40790_e53824 * (((var_v_ds_dn6 * var_v_ds) + (var_v_ds * var_v_ds_dn6)) / (2.0 * assign40790_e53830)))) / (assign40790_e53832 * assign40790_e53832));
        var_vdsx_dn7 = (((((var_v_ds_dn7 * var_v_ds) + (var_v_ds * var_v_ds_dn7)) * assign40790_e53832) - (assign40790_e53824 * (((var_v_ds_dn7 * var_v_ds) + (var_v_ds * var_v_ds_dn7)) / (2.0 * assign40790_e53830)))) / (assign40790_e53832 * assign40790_e53832));
        var_vdsx_rv = 0.0;

        let assign40800_e53837: f64 = (var_v_db + var_v_sb);
        let assign40800_e53840: f64 = (var_v_db - var_v_sb);
        let assign40800_e53843: f64 = (var_v_db - var_v_sb);
        let assign40800_e53844: f64 = (assign40800_e53840 * assign40800_e53843);
        let assign40800_e53846: f64 = (assign40800_e53844 + var_bphi_dc);
        let assign40800_e53847: f64 = (assign40800_e53846).sqrt();
        let assign40800_e53848: f64 = (assign40800_e53837 - assign40800_e53847);
        let assign40800_e53849: f64 = (0.5 * assign40800_e53848);
        let assign40800_e53851: f64 = (assign40800_e53849 + var_phix_dc);
        var_v_xb = assign40800_e53851;
        var_v_xb_dn6 = (0.5 * ((var_v_db_dn6 + var_v_sb_dn6) - ((((var_v_db_dn6 - var_v_sb_dn6) * assign40800_e53843) + (assign40800_e53840 * (var_v_db_dn6 - var_v_sb_dn6))) / (2.0 * assign40800_e53847))));
        var_v_xb_dn7 = (0.5 * ((var_v_db_dn7 + var_v_sb_dn7) - ((((var_v_db_dn7 - var_v_sb_dn7) * assign40800_e53843) + (assign40800_e53840 * (var_v_db_dn7 - var_v_sb_dn7))) / (2.0 * assign40800_e53847))));
        var_v_xb_dn8 = (0.5 * ((var_v_db_dn8 + var_v_sb_dn8) - ((((var_v_db_dn8 - var_v_sb_dn8) * assign40800_e53843) + (assign40800_e53840 * (var_v_db_dn8 - var_v_sb_dn8))) / (2.0 * assign40800_e53847))));
        var_v_xb_rv = 0.0;

        var_v_xb_dc_tmp = var_v_xb;
        var_v_xb_dc_tmp_dn6 = var_v_xb_dn6;
        var_v_xb_dc_tmp_dn7 = var_v_xb_dn7;
        var_v_xb_dc_tmp_dn8 = var_v_xb_dn8;
        var_v_xb_dc_tmp_rv = 0.0;

        let assign40820_e53857: f64 = var_v_xb;
        let assign40820_e53860: f64 = var_v_xb;
        let assign40820_e53863: f64 = var_v_xb;
        let assign40820_e53864: f64 = (assign40820_e53860 * assign40820_e53863);
        let assign40820_e53866: f64 = (assign40820_e53864 + var_aphi_dc);
        let assign40820_e53867: f64 = (assign40820_e53866).sqrt();
        let assign40820_e53868: f64 = (assign40820_e53857 - assign40820_e53867);
        let assign40820_e53869: f64 = (0.5 * assign40820_e53868);
        let assign40820_e53870: f64 = (var_v_sb - assign40820_e53869);
        let assign40820_e53872: f64 = (assign40820_e53870 + var_phix1_dc);
        var_vsbstar_dc = assign40820_e53872;
        var_vsbstar_dc_dn5 = 0.0;
        var_vsbstar_dc_dn6 = (var_v_sb_dn6 - (0.5 * (var_v_xb_dn6 - (((var_v_xb_dn6 * assign40820_e53863) + (assign40820_e53860 * var_v_xb_dn6)) / (2.0 * assign40820_e53867)))));
        var_vsbstar_dc_dn7 = (var_v_sb_dn7 - (0.5 * (var_v_xb_dn7 - (((var_v_xb_dn7 * assign40820_e53863) + (assign40820_e53860 * var_v_xb_dn7)) / (2.0 * assign40820_e53867)))));
        var_vsbstar_dc_dn8 = (var_v_sb_dn8 - (0.5 * (var_v_xb_dn8 - (((var_v_xb_dn8 * assign40820_e53863) + (assign40820_e53860 * var_v_xb_dn8)) / (2.0 * assign40820_e53867)))));
        var_vsbstar_dc_dn12 = 0.0;
        var_vsbstar_dc_dn13 = 0.0;
        var_vsbstar_dc_dn14 = 0.0;
        var_vsbstar_dc_dn15 = 0.0;
        var_vsbstar_dc_dn16 = 0.0;
        var_vsbstar_dc_dn17 = 0.0;
        var_vsbstar_dc_dn18 = 0.0;
        var_vsbstar_dc_dn19 = 0.0;
        var_vsbstar_dc_dn20 = 0.0;
        var_vsbstar_dc_rv = 0.0;

        var_vsbstar_dc_tmp = var_vsbstar_dc;
        var_vsbstar_dc_tmp_dn5 = var_vsbstar_dc_dn5;
        var_vsbstar_dc_tmp_dn6 = var_vsbstar_dc_dn6;
        var_vsbstar_dc_tmp_dn7 = var_vsbstar_dc_dn7;
        var_vsbstar_dc_tmp_dn8 = var_vsbstar_dc_dn8;
        var_vsbstar_dc_tmp_dn12 = var_vsbstar_dc_dn12;
        var_vsbstar_dc_tmp_dn13 = var_vsbstar_dc_dn13;
        var_vsbstar_dc_tmp_dn14 = var_vsbstar_dc_dn14;
        var_vsbstar_dc_tmp_dn15 = var_vsbstar_dc_dn15;
        var_vsbstar_dc_tmp_dn16 = var_vsbstar_dc_dn16;
        var_vsbstar_dc_tmp_dn17 = var_vsbstar_dc_dn17;
        var_vsbstar_dc_tmp_dn18 = var_vsbstar_dc_dn18;
        var_vsbstar_dc_tmp_dn19 = var_vsbstar_dc_dn19;
        var_vsbstar_dc_tmp_dn20 = var_vsbstar_dc_dn20;
        var_vsbstar_dc_tmp_rv = 0.0;

        var_dvbstar_dc = 0.0;
        var_dvbstar_dc_dn5 = 0.0;
        var_dvbstar_dc_dn6 = 0.0;
        var_dvbstar_dc_dn7 = 0.0;
        var_dvbstar_dc_dn8 = 0.0;
        var_dvbstar_dc_dn12 = 0.0;
        var_dvbstar_dc_dn13 = 0.0;
        var_dvbstar_dc_dn14 = 0.0;
        var_dvbstar_dc_dn15 = 0.0;
        var_dvbstar_dc_dn16 = 0.0;
        var_dvbstar_dc_dn17 = 0.0;
        var_dvbstar_dc_dn18 = 0.0;
        var_dvbstar_dc_dn19 = 0.0;
        var_dvbstar_dc_dn20 = 0.0;
        var_dvbstar_dc_rv = 0.0;

        let assign40850_e53881: f64 = if ((p.p45 != 0.0) && (var_gfacnud_i != 1.0)) { 1.0 } else { 0.0 };
        var_guard1274 = assign40850_e53881;
        var_guard1274_rv = 0.0;

        let (assign40860_e53891, assign40860_e53891_d_n5, assign40860_e53891_d_n6, assign40860_e53891_d_n7, assign40860_e53891_d_n8, assign40860_e53891_d_n12, assign40860_e53891_d_n13, assign40860_e53891_d_n14, assign40860_e53891_d_n15, assign40860_e53891_d_n16, assign40860_e53891_d_n17, assign40860_e53891_d_n18, assign40860_e53891_d_n19, assign40860_e53891_d_n20,) = {
    if (var_guard1274 != 0.0) {
        let assign40860_e53887: f64 = (var_v_ds - var_vdsx);
        let assign40860_e53888: f64 = (0.5 * assign40860_e53887);
        let assign40860_e53889: f64 = (var_vsbstar_dc + assign40860_e53888);
        (assign40860_e53889, var_vsbstar_dc_dn5, (var_vsbstar_dc_dn6 + (0.5 * (var_v_ds_dn6 - var_vdsx_dn6))), (var_vsbstar_dc_dn7 + (0.5 * (var_v_ds_dn7 - var_vdsx_dn7))), var_vsbstar_dc_dn8, var_vsbstar_dc_dn12, var_vsbstar_dc_dn13, var_vsbstar_dc_dn14, var_vsbstar_dc_dn15, var_vsbstar_dc_dn16, var_vsbstar_dc_dn17, var_vsbstar_dc_dn18, var_vsbstar_dc_dn19, var_vsbstar_dc_dn20,)
    } else {
        (var_vmb, var_vmb_dn5, var_vmb_dn6, var_vmb_dn7, var_vmb_dn8, var_vmb_dn12, var_vmb_dn13, var_vmb_dn14, var_vmb_dn15, var_vmb_dn16, var_vmb_dn17, var_vmb_dn18, var_vmb_dn19, var_vmb_dn20,)
    }
};
        var_vmb = assign40860_e53891;
        var_vmb_dn5 = assign40860_e53891_d_n5;
        var_vmb_dn6 = assign40860_e53891_d_n6;
        var_vmb_dn7 = assign40860_e53891_d_n7;
        var_vmb_dn8 = assign40860_e53891_d_n8;
        var_vmb_dn12 = assign40860_e53891_d_n12;
        var_vmb_dn13 = assign40860_e53891_d_n13;
        var_vmb_dn14 = assign40860_e53891_d_n14;
        var_vmb_dn15 = assign40860_e53891_d_n15;
        var_vmb_dn16 = assign40860_e53891_d_n16;
        var_vmb_dn17 = assign40860_e53891_d_n17;
        var_vmb_dn18 = assign40860_e53891_d_n18;
        var_vmb_dn19 = assign40860_e53891_d_n19;
        var_vmb_dn20 = assign40860_e53891_d_n20;
        var_vmb_rv = 0.0;

        let (assign40870_e53900, assign40870_e53900_d_n5, assign40870_e53900_d_n6, assign40870_e53900_d_n7, assign40870_e53900_d_n8, assign40870_e53900_d_n12, assign40870_e53900_d_n13, assign40870_e53900_d_n14, assign40870_e53900_d_n15, assign40870_e53900_d_n16, assign40870_e53900_d_n17, assign40870_e53900_d_n18, assign40870_e53900_d_n19, assign40870_e53900_d_n20,) = {
    if (var_guard1274 != 0.0) {
        let assign40870_e53895: f64 = (var_vmb + var_phib_dc);
        let assign40870_e53896: f64 = (assign40870_e53895).sqrt();
        let assign40870_e53898: f64 = (assign40870_e53896 - var_sqrt_phib_dc);
        (assign40870_e53898, (var_vmb_dn5 / (2.0 * assign40870_e53896)), (var_vmb_dn6 / (2.0 * assign40870_e53896)), (var_vmb_dn7 / (2.0 * assign40870_e53896)), (var_vmb_dn8 / (2.0 * assign40870_e53896)), (var_vmb_dn12 / (2.0 * assign40870_e53896)), (var_vmb_dn13 / (2.0 * assign40870_e53896)), (var_vmb_dn14 / (2.0 * assign40870_e53896)), (var_vmb_dn15 / (2.0 * assign40870_e53896)), (var_vmb_dn16 / (2.0 * assign40870_e53896)), (var_vmb_dn17 / (2.0 * assign40870_e53896)), (var_vmb_dn18 / (2.0 * assign40870_e53896)), (var_vmb_dn19 / (2.0 * assign40870_e53896)), (var_vmb_dn20 / (2.0 * assign40870_e53896)),)
    } else {
        (var_us, var_us_dn5, var_us_dn6, var_us_dn7, var_us_dn8, var_us_dn12, var_us_dn13, var_us_dn14, var_us_dn15, var_us_dn16, var_us_dn17, var_us_dn18, var_us_dn19, var_us_dn20,)
    }
};
        var_us = assign40870_e53900;
        var_us_dn5 = assign40870_e53900_d_n5;
        var_us_dn6 = assign40870_e53900_d_n6;
        var_us_dn7 = assign40870_e53900_d_n7;
        var_us_dn8 = assign40870_e53900_d_n8;
        var_us_dn12 = assign40870_e53900_d_n12;
        var_us_dn13 = assign40870_e53900_d_n13;
        var_us_dn14 = assign40870_e53900_d_n14;
        var_us_dn15 = assign40870_e53900_d_n15;
        var_us_dn16 = assign40870_e53900_d_n16;
        var_us_dn17 = assign40870_e53900_d_n17;
        var_us_dn18 = assign40870_e53900_d_n18;
        var_us_dn19 = assign40870_e53900_d_n19;
        var_us_dn20 = assign40870_e53900_d_n20;
        var_us_rv = 0.0;

        let (assign40880_e53912, assign40880_e53912_d_n5, assign40880_e53912_d_n6, assign40880_e53912_d_n7, assign40880_e53912_d_n8, assign40880_e53912_d_n12, assign40880_e53912_d_n13, assign40880_e53912_d_n14, assign40880_e53912_d_n15, assign40880_e53912_d_n16, assign40880_e53912_d_n17, assign40880_e53912_d_n18, assign40880_e53912_d_n19, assign40880_e53912_d_n20,) = {
    if (var_guard1274 != 0.0) {
        let assign40880_e53905: f64 = (var_us - var_us1);
        let assign40880_e53906: f64 = (2.0 * assign40880_e53905);
        let assign40880_e53908: f64 = (assign40880_e53906 / var_us21);
        let assign40880_e53910: f64 = (assign40880_e53908 - 1.0);
        (assign40880_e53910, ((2.0 * var_us_dn5) / var_us21), ((2.0 * var_us_dn6) / var_us21), ((2.0 * var_us_dn7) / var_us21), ((2.0 * var_us_dn8) / var_us21), ((2.0 * var_us_dn12) / var_us21), ((2.0 * var_us_dn13) / var_us21), ((2.0 * var_us_dn14) / var_us21), ((2.0 * var_us_dn15) / var_us21), ((2.0 * var_us_dn16) / var_us21), ((2.0 * var_us_dn17) / var_us21), ((2.0 * var_us_dn18) / var_us21), ((2.0 * var_us_dn19) / var_us21), ((2.0 * var_us_dn20) / var_us21),)
    } else {
        (var_temp__blk1038, var_temp__blk1038_dn5, var_temp__blk1038_dn6, var_temp__blk1038_dn7, var_temp__blk1038_dn8, var_temp__blk1038_dn12, var_temp__blk1038_dn13, var_temp__blk1038_dn14, var_temp__blk1038_dn15, var_temp__blk1038_dn16, var_temp__blk1038_dn17, var_temp__blk1038_dn18, var_temp__blk1038_dn19, var_temp__blk1038_dn20,)
    }
};
        var_temp__blk1038 = assign40880_e53912;
        var_temp__blk1038_dn5 = assign40880_e53912_d_n5;
        var_temp__blk1038_dn6 = assign40880_e53912_d_n6;
        var_temp__blk1038_dn7 = assign40880_e53912_d_n7;
        var_temp__blk1038_dn8 = assign40880_e53912_d_n8;
        var_temp__blk1038_dn12 = assign40880_e53912_d_n12;
        var_temp__blk1038_dn13 = assign40880_e53912_d_n13;
        var_temp__blk1038_dn14 = assign40880_e53912_d_n14;
        var_temp__blk1038_dn15 = assign40880_e53912_d_n15;
        var_temp__blk1038_dn16 = assign40880_e53912_d_n16;
        var_temp__blk1038_dn17 = assign40880_e53912_d_n17;
        var_temp__blk1038_dn18 = assign40880_e53912_d_n18;
        var_temp__blk1038_dn19 = assign40880_e53912_d_n19;
        var_temp__blk1038_dn20 = assign40880_e53912_d_n20;
        var_temp__blk1038_rv = 0.0;

        let (assign40890_e53933, assign40890_e53933_d_n5, assign40890_e53933_d_n6, assign40890_e53933_d_n7, assign40890_e53933_d_n8, assign40890_e53933_d_n12, assign40890_e53933_d_n13, assign40890_e53933_d_n14, assign40890_e53933_d_n15, assign40890_e53933_d_n16, assign40890_e53933_d_n17, assign40890_e53933_d_n18, assign40890_e53933_d_n19, assign40890_e53933_d_n20,) = {
    if (var_guard1274 != 0.0) {
        let assign40890_e53918: f64 = (1.0 - var_gfacnud_i);
        let assign40890_e53919: f64 = (0.25 * assign40890_e53918);
        let assign40890_e53921: f64 = (assign40890_e53919 * var_us21);
        let assign40890_e53925: f64 = (var_temp__blk1038 * var_temp__blk1038);
        let assign40890_e53927: f64 = (assign40890_e53925 + 0.4804530139182);
        let assign40890_e53928: f64 = (assign40890_e53927).sqrt();
        let assign40890_e53929: f64 = (var_temp__blk1038 + assign40890_e53928);
        let assign40890_e53930: f64 = (assign40890_e53921 * assign40890_e53929);
        let assign40890_e53931: f64 = (var_us - assign40890_e53930);
        (assign40890_e53931, (var_us_dn5 - (assign40890_e53921 * (var_temp__blk1038_dn5 + (((var_temp__blk1038_dn5 * var_temp__blk1038) + (var_temp__blk1038 * var_temp__blk1038_dn5)) / (2.0 * assign40890_e53928))))), (var_us_dn6 - (assign40890_e53921 * (var_temp__blk1038_dn6 + (((var_temp__blk1038_dn6 * var_temp__blk1038) + (var_temp__blk1038 * var_temp__blk1038_dn6)) / (2.0 * assign40890_e53928))))), (var_us_dn7 - (assign40890_e53921 * (var_temp__blk1038_dn7 + (((var_temp__blk1038_dn7 * var_temp__blk1038) + (var_temp__blk1038 * var_temp__blk1038_dn7)) / (2.0 * assign40890_e53928))))), (var_us_dn8 - (assign40890_e53921 * (var_temp__blk1038_dn8 + (((var_temp__blk1038_dn8 * var_temp__blk1038) + (var_temp__blk1038 * var_temp__blk1038_dn8)) / (2.0 * assign40890_e53928))))), (var_us_dn12 - (assign40890_e53921 * (var_temp__blk1038_dn12 + (((var_temp__blk1038_dn12 * var_temp__blk1038) + (var_temp__blk1038 * var_temp__blk1038_dn12)) / (2.0 * assign40890_e53928))))), (var_us_dn13 - (assign40890_e53921 * (var_temp__blk1038_dn13 + (((var_temp__blk1038_dn13 * var_temp__blk1038) + (var_temp__blk1038 * var_temp__blk1038_dn13)) / (2.0 * assign40890_e53928))))), (var_us_dn14 - (assign40890_e53921 * (var_temp__blk1038_dn14 + (((var_temp__blk1038_dn14 * var_temp__blk1038) + (var_temp__blk1038 * var_temp__blk1038_dn14)) / (2.0 * assign40890_e53928))))), (var_us_dn15 - (assign40890_e53921 * (var_temp__blk1038_dn15 + (((var_temp__blk1038_dn15 * var_temp__blk1038) + (var_temp__blk1038 * var_temp__blk1038_dn15)) / (2.0 * assign40890_e53928))))), (var_us_dn16 - (assign40890_e53921 * (var_temp__blk1038_dn16 + (((var_temp__blk1038_dn16 * var_temp__blk1038) + (var_temp__blk1038 * var_temp__blk1038_dn16)) / (2.0 * assign40890_e53928))))), (var_us_dn17 - (assign40890_e53921 * (var_temp__blk1038_dn17 + (((var_temp__blk1038_dn17 * var_temp__blk1038) + (var_temp__blk1038 * var_temp__blk1038_dn17)) / (2.0 * assign40890_e53928))))), (var_us_dn18 - (assign40890_e53921 * (var_temp__blk1038_dn18 + (((var_temp__blk1038_dn18 * var_temp__blk1038) + (var_temp__blk1038 * var_temp__blk1038_dn18)) / (2.0 * assign40890_e53928))))), (var_us_dn19 - (assign40890_e53921 * (var_temp__blk1038_dn19 + (((var_temp__blk1038_dn19 * var_temp__blk1038) + (var_temp__blk1038 * var_temp__blk1038_dn19)) / (2.0 * assign40890_e53928))))), (var_us_dn20 - (assign40890_e53921 * (var_temp__blk1038_dn20 + (((var_temp__blk1038_dn20 * var_temp__blk1038) + (var_temp__blk1038 * var_temp__blk1038_dn20)) / (2.0 * assign40890_e53928))))),)
    } else {
        (var_usnew, var_usnew_dn5, var_usnew_dn6, var_usnew_dn7, var_usnew_dn8, var_usnew_dn12, var_usnew_dn13, var_usnew_dn14, var_usnew_dn15, var_usnew_dn16, var_usnew_dn17, var_usnew_dn18, var_usnew_dn19, var_usnew_dn20,)
    }
};
        var_usnew = assign40890_e53933;
        var_usnew_dn5 = assign40890_e53933_d_n5;
        var_usnew_dn6 = assign40890_e53933_d_n6;
        var_usnew_dn7 = assign40890_e53933_d_n7;
        var_usnew_dn8 = assign40890_e53933_d_n8;
        var_usnew_dn12 = assign40890_e53933_d_n12;
        var_usnew_dn13 = assign40890_e53933_d_n13;
        var_usnew_dn14 = assign40890_e53933_d_n14;
        var_usnew_dn15 = assign40890_e53933_d_n15;
        var_usnew_dn16 = assign40890_e53933_d_n16;
        var_usnew_dn17 = assign40890_e53933_d_n17;
        var_usnew_dn18 = assign40890_e53933_d_n18;
        var_usnew_dn19 = assign40890_e53933_d_n19;
        var_usnew_dn20 = assign40890_e53933_d_n20;
        var_usnew_rv = 0.0;

        let (assign40900_e53945, assign40900_e53945_d_n5, assign40900_e53945_d_n6, assign40900_e53945_d_n7, assign40900_e53945_d_n8, assign40900_e53945_d_n12, assign40900_e53945_d_n13, assign40900_e53945_d_n14, assign40900_e53945_d_n15, assign40900_e53945_d_n16, assign40900_e53945_d_n17, assign40900_e53945_d_n18, assign40900_e53945_d_n19, assign40900_e53945_d_n20,) = {
    if (var_guard1274 != 0.0) {
        let assign40900_e53937: f64 = (var_usnew * var_usnew);
        let assign40900_e53940: f64 = (2.0 * var_sqrt_phib_dc);
        let assign40900_e53942: f64 = (assign40900_e53940 * var_usnew);
        let assign40900_e53943: f64 = (assign40900_e53937 + assign40900_e53942);
        (assign40900_e53943, (((var_usnew_dn5 * var_usnew) + (var_usnew * var_usnew_dn5)) + (assign40900_e53940 * var_usnew_dn5)), (((var_usnew_dn6 * var_usnew) + (var_usnew * var_usnew_dn6)) + (assign40900_e53940 * var_usnew_dn6)), (((var_usnew_dn7 * var_usnew) + (var_usnew * var_usnew_dn7)) + (assign40900_e53940 * var_usnew_dn7)), (((var_usnew_dn8 * var_usnew) + (var_usnew * var_usnew_dn8)) + (assign40900_e53940 * var_usnew_dn8)), (((var_usnew_dn12 * var_usnew) + (var_usnew * var_usnew_dn12)) + (assign40900_e53940 * var_usnew_dn12)), (((var_usnew_dn13 * var_usnew) + (var_usnew * var_usnew_dn13)) + (assign40900_e53940 * var_usnew_dn13)), (((var_usnew_dn14 * var_usnew) + (var_usnew * var_usnew_dn14)) + (assign40900_e53940 * var_usnew_dn14)), (((var_usnew_dn15 * var_usnew) + (var_usnew * var_usnew_dn15)) + (assign40900_e53940 * var_usnew_dn15)), (((var_usnew_dn16 * var_usnew) + (var_usnew * var_usnew_dn16)) + (assign40900_e53940 * var_usnew_dn16)), (((var_usnew_dn17 * var_usnew) + (var_usnew * var_usnew_dn17)) + (assign40900_e53940 * var_usnew_dn17)), (((var_usnew_dn18 * var_usnew) + (var_usnew * var_usnew_dn18)) + (assign40900_e53940 * var_usnew_dn18)), (((var_usnew_dn19 * var_usnew) + (var_usnew * var_usnew_dn19)) + (assign40900_e53940 * var_usnew_dn19)), (((var_usnew_dn20 * var_usnew) + (var_usnew * var_usnew_dn20)) + (assign40900_e53940 * var_usnew_dn20)),)
    } else {
        (var_vmbnew, var_vmbnew_dn5, var_vmbnew_dn6, var_vmbnew_dn7, var_vmbnew_dn8, var_vmbnew_dn12, var_vmbnew_dn13, var_vmbnew_dn14, var_vmbnew_dn15, var_vmbnew_dn16, var_vmbnew_dn17, var_vmbnew_dn18, var_vmbnew_dn19, var_vmbnew_dn20,)
    }
};
        var_vmbnew = assign40900_e53945;
        var_vmbnew_dn5 = assign40900_e53945_d_n5;
        var_vmbnew_dn6 = assign40900_e53945_d_n6;
        var_vmbnew_dn7 = assign40900_e53945_d_n7;
        var_vmbnew_dn8 = assign40900_e53945_d_n8;
        var_vmbnew_dn12 = assign40900_e53945_d_n12;
        var_vmbnew_dn13 = assign40900_e53945_d_n13;
        var_vmbnew_dn14 = assign40900_e53945_d_n14;
        var_vmbnew_dn15 = assign40900_e53945_d_n15;
        var_vmbnew_dn16 = assign40900_e53945_d_n16;
        var_vmbnew_dn17 = assign40900_e53945_d_n17;
        var_vmbnew_dn18 = assign40900_e53945_d_n18;
        var_vmbnew_dn19 = assign40900_e53945_d_n19;
        var_vmbnew_dn20 = assign40900_e53945_d_n20;
        var_vmbnew_rv = 0.0;

        *var_dvbstar_dc_slot = var_dvbstar_dc;
        *var_dvbstar_dc_dn12_slot = var_dvbstar_dc_dn12;
        *var_dvbstar_dc_dn13_slot = var_dvbstar_dc_dn13;
        *var_dvbstar_dc_dn14_slot = var_dvbstar_dc_dn14;
        *var_dvbstar_dc_dn15_slot = var_dvbstar_dc_dn15;
        *var_dvbstar_dc_dn16_slot = var_dvbstar_dc_dn16;
        *var_dvbstar_dc_dn17_slot = var_dvbstar_dc_dn17;
        *var_dvbstar_dc_dn18_slot = var_dvbstar_dc_dn18;
        *var_dvbstar_dc_dn19_slot = var_dvbstar_dc_dn19;
        *var_dvbstar_dc_dn20_slot = var_dvbstar_dc_dn20;
        *var_dvbstar_dc_dn5_slot = var_dvbstar_dc_dn5;
        *var_dvbstar_dc_dn6_slot = var_dvbstar_dc_dn6;
        *var_dvbstar_dc_dn7_slot = var_dvbstar_dc_dn7;
        *var_dvbstar_dc_dn8_slot = var_dvbstar_dc_dn8;
        *var_dvbstar_dc_rv_slot = var_dvbstar_dc_rv;
        *var_guard1114_slot = var_guard1114;
        *var_guard1114_rv_slot = var_guard1114_rv;
        *var_guard1274_slot = var_guard1274;
        *var_guard1274_rv_slot = var_guard1274_rv;
        *var_sigvds_slot = var_sigvds;
        *var_sigvds_rv_slot = var_sigvds_rv;
        *var_temp__blk1038_slot = var_temp__blk1038;
        *var_temp__blk1038_dn12_slot = var_temp__blk1038_dn12;
        *var_temp__blk1038_dn13_slot = var_temp__blk1038_dn13;
        *var_temp__blk1038_dn14_slot = var_temp__blk1038_dn14;
        *var_temp__blk1038_dn15_slot = var_temp__blk1038_dn15;
        *var_temp__blk1038_dn16_slot = var_temp__blk1038_dn16;
        *var_temp__blk1038_dn17_slot = var_temp__blk1038_dn17;
        *var_temp__blk1038_dn18_slot = var_temp__blk1038_dn18;
        *var_temp__blk1038_dn19_slot = var_temp__blk1038_dn19;
        *var_temp__blk1038_dn20_slot = var_temp__blk1038_dn20;
        *var_temp__blk1038_dn5_slot = var_temp__blk1038_dn5;
        *var_temp__blk1038_dn6_slot = var_temp__blk1038_dn6;
        *var_temp__blk1038_dn7_slot = var_temp__blk1038_dn7;
        *var_temp__blk1038_dn8_slot = var_temp__blk1038_dn8;
        *var_temp__blk1038_rv_slot = var_temp__blk1038_rv;
        *var_us_slot = var_us;
        *var_us_dn12_slot = var_us_dn12;
        *var_us_dn13_slot = var_us_dn13;
        *var_us_dn14_slot = var_us_dn14;
        *var_us_dn15_slot = var_us_dn15;
        *var_us_dn16_slot = var_us_dn16;
        *var_us_dn17_slot = var_us_dn17;
        *var_us_dn18_slot = var_us_dn18;
        *var_us_dn19_slot = var_us_dn19;
        *var_us_dn20_slot = var_us_dn20;
        *var_us_dn5_slot = var_us_dn5;
        *var_us_dn6_slot = var_us_dn6;
        *var_us_dn7_slot = var_us_dn7;
        *var_us_dn8_slot = var_us_dn8;
        *var_us_rv_slot = var_us_rv;
        *var_usnew_slot = var_usnew;
        *var_usnew_dn12_slot = var_usnew_dn12;
        *var_usnew_dn13_slot = var_usnew_dn13;
        *var_usnew_dn14_slot = var_usnew_dn14;
        *var_usnew_dn15_slot = var_usnew_dn15;
        *var_usnew_dn16_slot = var_usnew_dn16;
        *var_usnew_dn17_slot = var_usnew_dn17;
        *var_usnew_dn18_slot = var_usnew_dn18;
        *var_usnew_dn19_slot = var_usnew_dn19;
        *var_usnew_dn20_slot = var_usnew_dn20;
        *var_usnew_dn5_slot = var_usnew_dn5;
        *var_usnew_dn6_slot = var_usnew_dn6;
        *var_usnew_dn7_slot = var_usnew_dn7;
        *var_usnew_dn8_slot = var_usnew_dn8;
        *var_usnew_rv_slot = var_usnew_rv;
        *var_v_db_slot = var_v_db;
        *var_v_db_dn6_slot = var_v_db_dn6;
        *var_v_db_dn7_slot = var_v_db_dn7;
        *var_v_db_dn8_slot = var_v_db_dn8;
        *var_v_db_rv_slot = var_v_db_rv;
        *var_v_ds_slot = var_v_ds;
        *var_v_ds_dn6_slot = var_v_ds_dn6;
        *var_v_ds_dn7_slot = var_v_ds_dn7;
        *var_v_ds_rv_slot = var_v_ds_rv;
        *var_v_gs_slot = var_v_gs;
        *var_v_gs_dn5_slot = var_v_gs_dn5;
        *var_v_gs_dn6_slot = var_v_gs_dn6;
        *var_v_gs_dn7_slot = var_v_gs_dn7;
        *var_v_gs_rv_slot = var_v_gs_rv;
        *var_v_sb_slot = var_v_sb;
        *var_v_sb_dn6_slot = var_v_sb_dn6;
        *var_v_sb_dn7_slot = var_v_sb_dn7;
        *var_v_sb_dn8_slot = var_v_sb_dn8;
        *var_v_sb_rv_slot = var_v_sb_rv;
        *var_v_xb_slot = var_v_xb;
        *var_v_xb_dc_tmp_slot = var_v_xb_dc_tmp;
        *var_v_xb_dc_tmp_dn6_slot = var_v_xb_dc_tmp_dn6;
        *var_v_xb_dc_tmp_dn7_slot = var_v_xb_dc_tmp_dn7;
        *var_v_xb_dc_tmp_dn8_slot = var_v_xb_dc_tmp_dn8;
        *var_v_xb_dc_tmp_rv_slot = var_v_xb_dc_tmp_rv;
        *var_v_xb_dn6_slot = var_v_xb_dn6;
        *var_v_xb_dn7_slot = var_v_xb_dn7;
        *var_v_xb_dn8_slot = var_v_xb_dn8;
        *var_v_xb_rv_slot = var_v_xb_rv;
        *var_vdbprime_slot = var_vdbprime;
        *var_vdbprime_dn6_slot = var_vdbprime_dn6;
        *var_vdbprime_dn7_slot = var_vdbprime_dn7;
        *var_vdbprime_dn8_slot = var_vdbprime_dn8;
        *var_vdbprime_rv_slot = var_vdbprime_rv;
        *var_vdsx_slot = var_vdsx;
        *var_vdsx_dn6_slot = var_vdsx_dn6;
        *var_vdsx_dn7_slot = var_vdsx_dn7;
        *var_vdsx_rv_slot = var_vdsx_rv;
        *var_vgb_slot = var_vgb;
        *var_vgb_dn5_slot = var_vgb_dn5;
        *var_vgb_dn6_slot = var_vgb_dn6;
        *var_vgb_dn7_slot = var_vgb_dn7;
        *var_vgb_dn8_slot = var_vgb_dn8;
        *var_vgb_rv_slot = var_vgb_rv;
        *var_vgdprime_slot = var_vgdprime;
        *var_vgdprime_dn5_slot = var_vgdprime_dn5;
        *var_vgdprime_dn6_slot = var_vgdprime_dn6;
        *var_vgdprime_dn7_slot = var_vgdprime_dn7;
        *var_vgdprime_rv_slot = var_vgdprime_rv;
        *var_vgsprime_slot = var_vgsprime;
        *var_vgsprime_dn5_slot = var_vgsprime_dn5;
        *var_vgsprime_dn6_slot = var_vgsprime_dn6;
        *var_vgsprime_dn7_slot = var_vgsprime_dn7;
        *var_vgsprime_rv_slot = var_vgsprime_rv;
        *var_vjun_d_slot = var_vjun_d;
        *var_vjun_d_dn11_slot = var_vjun_d_dn11;
        *var_vjun_d_dn7_slot = var_vjun_d_dn7;
        *var_vjun_d_rv_slot = var_vjun_d_rv;
        *var_vjun_s_slot = var_vjun_s;
        *var_vjun_s_dn10_slot = var_vjun_s_dn10;
        *var_vjun_s_dn6_slot = var_vjun_s_dn6;
        *var_vjun_s_rv_slot = var_vjun_s_rv;
        *var_vmb_slot = var_vmb;
        *var_vmb_dn12_slot = var_vmb_dn12;
        *var_vmb_dn13_slot = var_vmb_dn13;
        *var_vmb_dn14_slot = var_vmb_dn14;
        *var_vmb_dn15_slot = var_vmb_dn15;
        *var_vmb_dn16_slot = var_vmb_dn16;
        *var_vmb_dn17_slot = var_vmb_dn17;
        *var_vmb_dn18_slot = var_vmb_dn18;
        *var_vmb_dn19_slot = var_vmb_dn19;
        *var_vmb_dn20_slot = var_vmb_dn20;
        *var_vmb_dn5_slot = var_vmb_dn5;
        *var_vmb_dn6_slot = var_vmb_dn6;
        *var_vmb_dn7_slot = var_vmb_dn7;
        *var_vmb_dn8_slot = var_vmb_dn8;
        *var_vmb_rv_slot = var_vmb_rv;
        *var_vmbnew_slot = var_vmbnew;
        *var_vmbnew_dn12_slot = var_vmbnew_dn12;
        *var_vmbnew_dn13_slot = var_vmbnew_dn13;
        *var_vmbnew_dn14_slot = var_vmbnew_dn14;
        *var_vmbnew_dn15_slot = var_vmbnew_dn15;
        *var_vmbnew_dn16_slot = var_vmbnew_dn16;
        *var_vmbnew_dn17_slot = var_vmbnew_dn17;
        *var_vmbnew_dn18_slot = var_vmbnew_dn18;
        *var_vmbnew_dn19_slot = var_vmbnew_dn19;
        *var_vmbnew_dn20_slot = var_vmbnew_dn20;
        *var_vmbnew_dn5_slot = var_vmbnew_dn5;
        *var_vmbnew_dn6_slot = var_vmbnew_dn6;
        *var_vmbnew_dn7_slot = var_vmbnew_dn7;
        *var_vmbnew_dn8_slot = var_vmbnew_dn8;
        *var_vmbnew_rv_slot = var_vmbnew_rv;
        *var_vsbprime_slot = var_vsbprime;
        *var_vsbprime_dn6_slot = var_vsbprime_dn6;
        *var_vsbprime_dn7_slot = var_vsbprime_dn7;
        *var_vsbprime_dn8_slot = var_vsbprime_dn8;
        *var_vsbprime_rv_slot = var_vsbprime_rv;
        *var_vsbstar_dc_slot = var_vsbstar_dc;
        *var_vsbstar_dc_dn12_slot = var_vsbstar_dc_dn12;
        *var_vsbstar_dc_dn13_slot = var_vsbstar_dc_dn13;
        *var_vsbstar_dc_dn14_slot = var_vsbstar_dc_dn14;
        *var_vsbstar_dc_dn15_slot = var_vsbstar_dc_dn15;
        *var_vsbstar_dc_dn16_slot = var_vsbstar_dc_dn16;
        *var_vsbstar_dc_dn17_slot = var_vsbstar_dc_dn17;
        *var_vsbstar_dc_dn18_slot = var_vsbstar_dc_dn18;
        *var_vsbstar_dc_dn19_slot = var_vsbstar_dc_dn19;
        *var_vsbstar_dc_dn20_slot = var_vsbstar_dc_dn20;
        *var_vsbstar_dc_dn5_slot = var_vsbstar_dc_dn5;
        *var_vsbstar_dc_dn6_slot = var_vsbstar_dc_dn6;
        *var_vsbstar_dc_dn7_slot = var_vsbstar_dc_dn7;
        *var_vsbstar_dc_dn8_slot = var_vsbstar_dc_dn8;
        *var_vsbstar_dc_rv_slot = var_vsbstar_dc_rv;
        *var_vsbstar_dc_tmp_slot = var_vsbstar_dc_tmp;
        *var_vsbstar_dc_tmp_dn12_slot = var_vsbstar_dc_tmp_dn12;
        *var_vsbstar_dc_tmp_dn13_slot = var_vsbstar_dc_tmp_dn13;
        *var_vsbstar_dc_tmp_dn14_slot = var_vsbstar_dc_tmp_dn14;
        *var_vsbstar_dc_tmp_dn15_slot = var_vsbstar_dc_tmp_dn15;
        *var_vsbstar_dc_tmp_dn16_slot = var_vsbstar_dc_tmp_dn16;
        *var_vsbstar_dc_tmp_dn17_slot = var_vsbstar_dc_tmp_dn17;
        *var_vsbstar_dc_tmp_dn18_slot = var_vsbstar_dc_tmp_dn18;
        *var_vsbstar_dc_tmp_dn19_slot = var_vsbstar_dc_tmp_dn19;
        *var_vsbstar_dc_tmp_dn20_slot = var_vsbstar_dc_tmp_dn20;
        *var_vsbstar_dc_tmp_dn5_slot = var_vsbstar_dc_tmp_dn5;
        *var_vsbstar_dc_tmp_dn6_slot = var_vsbstar_dc_tmp_dn6;
        *var_vsbstar_dc_tmp_dn7_slot = var_vsbstar_dc_tmp_dn7;
        *var_vsbstar_dc_tmp_dn8_slot = var_vsbstar_dc_tmp_dn8;
        *var_vsbstar_dc_tmp_rv_slot = var_vsbstar_dc_tmp_rv;
        *var_xgb_ov_slot = var_xgb_ov;
        *var_xgb_ov_dn5_slot = var_xgb_ov_dn5;
        *var_xgb_ov_dn6_slot = var_xgb_ov_dn6;
        *var_xgb_ov_dn7_slot = var_xgb_ov_dn7;
        *var_xgb_ov_dn8_slot = var_xgb_ov_dn8;
        *var_xgb_ov_rv_slot = var_xgb_ov_rv;
        *var_xgd_ov_slot = var_xgd_ov;
        *var_xgd_ov_dn5_slot = var_xgd_ov_dn5;
        *var_xgd_ov_dn6_slot = var_xgd_ov_dn6;
        *var_xgd_ov_dn7_slot = var_xgd_ov_dn7;
        *var_xgd_ov_rv_slot = var_xgd_ov_rv;
        *var_xgs_ov_slot = var_xgs_ov;
        *var_xgs_ov_dn5_slot = var_xgs_ov_dn5;
        *var_xgs_ov_dn6_slot = var_xgs_ov_dn6;
        *var_xgs_ov_dn7_slot = var_xgs_ov_dn7;
        *var_xgs_ov_rv_slot = var_xgs_ov_rv;
    }

    pub(super) fn stamp_reactive_block_24(
        var_aphi_dc: f64,
        var_ar: f64,
        var_ctb_i: f64,
        var_ctg_i: f64,
        var_g_0_dc: f64,
        var_guard1274: f64,
        var_inv_phit: f64,
        var_phib_dc: f64,
        var_thesat_t: f64,
        var_v_ds: f64,
        var_v_ds_dn6: f64,
        var_v_ds_dn7: f64,
        var_vdsx: f64,
        var_vdsx_dn6: f64,
        var_vdsx_dn7: f64,
        var_vfb_t: f64,
        var_vgb: f64,
        var_vgb_dn5: f64,
        var_vgb_dn6: f64,
        var_vgb_dn7: f64,
        var_vgb_dn8: f64,
        var_vmbnew: f64,
        var_vmbnew_dn12: f64,
        var_vmbnew_dn13: f64,
        var_vmbnew_dn14: f64,
        var_vmbnew_dn15: f64,
        var_vmbnew_dn16: f64,
        var_vmbnew_dn17: f64,
        var_vmbnew_dn18: f64,
        var_vmbnew_dn19: f64,
        var_vmbnew_dn20: f64,
        var_vmbnew_dn5: f64,
        var_vmbnew_dn6: f64,
        var_vmbnew_dn7: f64,
        var_vmbnew_dn8: f64,
        var_vsbstar_dc_tmp: f64,
        var_vsbstar_dc_tmp_dn12: f64,
        var_vsbstar_dc_tmp_dn13: f64,
        var_vsbstar_dc_tmp_dn14: f64,
        var_vsbstar_dc_tmp_dn15: f64,
        var_vsbstar_dc_tmp_dn16: f64,
        var_vsbstar_dc_tmp_dn17: f64,
        var_vsbstar_dc_tmp_dn18: f64,
        var_vsbstar_dc_tmp_dn19: f64,
        var_vsbstar_dc_tmp_dn20: f64,
        var_vsbstar_dc_tmp_dn5: f64,
        var_vsbstar_dc_tmp_dn6: f64,
        var_vsbstar_dc_tmp_dn7: f64,
        var_vsbstar_dc_tmp_dn8: f64,
        var_aphi_slot: &mut f64,
        var_aphi_rv_slot: &mut f64,
        var_arloc_slot: &mut f64,
        var_arloc_rv_slot: &mut f64,
        var_dctg_slot: &mut f64,
        var_dctg_dn12_slot: &mut f64,
        var_dctg_dn13_slot: &mut f64,
        var_dctg_dn14_slot: &mut f64,
        var_dctg_dn15_slot: &mut f64,
        var_dctg_dn16_slot: &mut f64,
        var_dctg_dn17_slot: &mut f64,
        var_dctg_dn18_slot: &mut f64,
        var_dctg_dn19_slot: &mut f64,
        var_dctg_dn20_slot: &mut f64,
        var_dctg_dn5_slot: &mut f64,
        var_dctg_dn6_slot: &mut f64,
        var_dctg_dn7_slot: &mut f64,
        var_dctg_dn8_slot: &mut f64,
        var_dctg_rv_slot: &mut f64,
        var_dvbstar_slot: &mut f64,
        var_dvbstar_dc_slot: &mut f64,
        var_dvbstar_dc_dn12_slot: &mut f64,
        var_dvbstar_dc_dn13_slot: &mut f64,
        var_dvbstar_dc_dn14_slot: &mut f64,
        var_dvbstar_dc_dn15_slot: &mut f64,
        var_dvbstar_dc_dn16_slot: &mut f64,
        var_dvbstar_dc_dn17_slot: &mut f64,
        var_dvbstar_dc_dn18_slot: &mut f64,
        var_dvbstar_dc_dn19_slot: &mut f64,
        var_dvbstar_dc_dn20_slot: &mut f64,
        var_dvbstar_dc_dn5_slot: &mut f64,
        var_dvbstar_dc_dn6_slot: &mut f64,
        var_dvbstar_dc_dn7_slot: &mut f64,
        var_dvbstar_dc_dn8_slot: &mut f64,
        var_dvbstar_dc_rv_slot: &mut f64,
        var_dvbstar_dn12_slot: &mut f64,
        var_dvbstar_dn13_slot: &mut f64,
        var_dvbstar_dn14_slot: &mut f64,
        var_dvbstar_dn15_slot: &mut f64,
        var_dvbstar_dn16_slot: &mut f64,
        var_dvbstar_dn17_slot: &mut f64,
        var_dvbstar_dn18_slot: &mut f64,
        var_dvbstar_dn19_slot: &mut f64,
        var_dvbstar_dn20_slot: &mut f64,
        var_dvbstar_dn5_slot: &mut f64,
        var_dvbstar_dn6_slot: &mut f64,
        var_dvbstar_dn7_slot: &mut f64,
        var_dvbstar_dn8_slot: &mut f64,
        var_dvbstar_rv_slot: &mut f64,
        var_g_0_slot: &mut f64,
        var_g_0_rv_slot: &mut f64,
        var_guard1275_slot: &mut f64,
        var_guard1275_rv_slot: &mut f64,
        var_phib_slot: &mut f64,
        var_phib_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn12_slot: &mut f64,
        var_temp1_dn13_slot: &mut f64,
        var_temp1_dn14_slot: &mut f64,
        var_temp1_dn15_slot: &mut f64,
        var_temp1_dn16_slot: &mut f64,
        var_temp1_dn17_slot: &mut f64,
        var_temp1_dn18_slot: &mut f64,
        var_temp1_dn19_slot: &mut f64,
        var_temp1_dn20_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn12_slot: &mut f64,
        var_temp2_dn13_slot: &mut f64,
        var_temp2_dn14_slot: &mut f64,
        var_temp2_dn15_slot: &mut f64,
        var_temp2_dn16_slot: &mut f64,
        var_temp2_dn17_slot: &mut f64,
        var_temp2_dn18_slot: &mut f64,
        var_temp2_dn19_slot: &mut f64,
        var_temp2_dn20_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_thesatloc_slot: &mut f64,
        var_thesatloc_rv_slot: &mut f64,
        var_vgb1_slot: &mut f64,
        var_vgb1_dn12_slot: &mut f64,
        var_vgb1_dn13_slot: &mut f64,
        var_vgb1_dn14_slot: &mut f64,
        var_vgb1_dn15_slot: &mut f64,
        var_vgb1_dn16_slot: &mut f64,
        var_vgb1_dn17_slot: &mut f64,
        var_vgb1_dn18_slot: &mut f64,
        var_vgb1_dn19_slot: &mut f64,
        var_vgb1_dn20_slot: &mut f64,
        var_vgb1_dn5_slot: &mut f64,
        var_vgb1_dn6_slot: &mut f64,
        var_vgb1_dn7_slot: &mut f64,
        var_vgb1_dn8_slot: &mut f64,
        var_vgb1_rv_slot: &mut f64,
        var_vsbstar_slot: &mut f64,
        var_vsbstar_dc_slot: &mut f64,
        var_vsbstar_dc_dn12_slot: &mut f64,
        var_vsbstar_dc_dn13_slot: &mut f64,
        var_vsbstar_dc_dn14_slot: &mut f64,
        var_vsbstar_dc_dn15_slot: &mut f64,
        var_vsbstar_dc_dn16_slot: &mut f64,
        var_vsbstar_dc_dn17_slot: &mut f64,
        var_vsbstar_dc_dn18_slot: &mut f64,
        var_vsbstar_dc_dn19_slot: &mut f64,
        var_vsbstar_dc_dn20_slot: &mut f64,
        var_vsbstar_dc_dn5_slot: &mut f64,
        var_vsbstar_dc_dn6_slot: &mut f64,
        var_vsbstar_dc_dn7_slot: &mut f64,
        var_vsbstar_dc_dn8_slot: &mut f64,
        var_vsbstar_dc_rv_slot: &mut f64,
        var_vsbstar_dn12_slot: &mut f64,
        var_vsbstar_dn13_slot: &mut f64,
        var_vsbstar_dn14_slot: &mut f64,
        var_vsbstar_dn15_slot: &mut f64,
        var_vsbstar_dn16_slot: &mut f64,
        var_vsbstar_dn17_slot: &mut f64,
        var_vsbstar_dn18_slot: &mut f64,
        var_vsbstar_dn19_slot: &mut f64,
        var_vsbstar_dn20_slot: &mut f64,
        var_vsbstar_dn5_slot: &mut f64,
        var_vsbstar_dn6_slot: &mut f64,
        var_vsbstar_dn7_slot: &mut f64,
        var_vsbstar_dn8_slot: &mut f64,
        var_vsbstar_rv_slot: &mut f64,
        var_vsbx_slot: &mut f64,
        var_vsbx_dn12_slot: &mut f64,
        var_vsbx_dn13_slot: &mut f64,
        var_vsbx_dn14_slot: &mut f64,
        var_vsbx_dn15_slot: &mut f64,
        var_vsbx_dn16_slot: &mut f64,
        var_vsbx_dn17_slot: &mut f64,
        var_vsbx_dn18_slot: &mut f64,
        var_vsbx_dn19_slot: &mut f64,
        var_vsbx_dn20_slot: &mut f64,
        var_vsbx_dn5_slot: &mut f64,
        var_vsbx_dn6_slot: &mut f64,
        var_vsbx_dn7_slot: &mut f64,
        var_vsbx_dn8_slot: &mut f64,
        var_vsbx_rv_slot: &mut f64,
        var_xbct_slot: &mut f64,
        var_xbct_rv_slot: &mut f64,
        var_xctmax_slot: &mut f64,
        var_xctmax_rv_slot: &mut f64,
        var_xgct_slot: &mut f64,
        var_xgct_dn12_slot: &mut f64,
        var_xgct_dn13_slot: &mut f64,
        var_xgct_dn14_slot: &mut f64,
        var_xgct_dn15_slot: &mut f64,
        var_xgct_dn16_slot: &mut f64,
        var_xgct_dn17_slot: &mut f64,
        var_xgct_dn18_slot: &mut f64,
        var_xgct_dn19_slot: &mut f64,
        var_xgct_dn20_slot: &mut f64,
        var_xgct_dn5_slot: &mut f64,
        var_xgct_dn6_slot: &mut f64,
        var_xgct_dn7_slot: &mut f64,
        var_xgct_dn8_slot: &mut f64,
        var_xgct_rv_slot: &mut f64,
        var_xmict_slot: &mut f64,
        var_xmict_dn12_slot: &mut f64,
        var_xmict_dn13_slot: &mut f64,
        var_xmict_dn14_slot: &mut f64,
        var_xmict_dn15_slot: &mut f64,
        var_xmict_dn16_slot: &mut f64,
        var_xmict_dn17_slot: &mut f64,
        var_xmict_dn18_slot: &mut f64,
        var_xmict_dn19_slot: &mut f64,
        var_xmict_dn20_slot: &mut f64,
        var_xmict_dn5_slot: &mut f64,
        var_xmict_dn6_slot: &mut f64,
        var_xmict_dn7_slot: &mut f64,
        var_xmict_dn8_slot: &mut f64,
        var_xmict_rv_slot: &mut f64,
        var_xnct_slot: &mut f64,
        var_xnct_dn12_slot: &mut f64,
        var_xnct_dn13_slot: &mut f64,
        var_xnct_dn14_slot: &mut f64,
        var_xnct_dn15_slot: &mut f64,
        var_xnct_dn16_slot: &mut f64,
        var_xnct_dn17_slot: &mut f64,
        var_xnct_dn18_slot: &mut f64,
        var_xnct_dn19_slot: &mut f64,
        var_xnct_dn20_slot: &mut f64,
        var_xnct_dn5_slot: &mut f64,
        var_xnct_dn6_slot: &mut f64,
        var_xnct_dn7_slot: &mut f64,
        var_xnct_dn8_slot: &mut f64,
        var_xnct_rv_slot: &mut f64,
        var_xsbstar_slot: &mut f64,
        var_xsbstar_dn12_slot: &mut f64,
        var_xsbstar_dn13_slot: &mut f64,
        var_xsbstar_dn14_slot: &mut f64,
        var_xsbstar_dn15_slot: &mut f64,
        var_xsbstar_dn16_slot: &mut f64,
        var_xsbstar_dn17_slot: &mut f64,
        var_xsbstar_dn18_slot: &mut f64,
        var_xsbstar_dn19_slot: &mut f64,
        var_xsbstar_dn20_slot: &mut f64,
        var_xsbstar_dn5_slot: &mut f64,
        var_xsbstar_dn6_slot: &mut f64,
        var_xsbstar_dn7_slot: &mut f64,
        var_xsbstar_dn8_slot: &mut f64,
        var_xsbstar_rv_slot: &mut f64,
        var_xsubct_slot: &mut f64,
        var_xsubct_dn12_slot: &mut f64,
        var_xsubct_dn13_slot: &mut f64,
        var_xsubct_dn14_slot: &mut f64,
        var_xsubct_dn15_slot: &mut f64,
        var_xsubct_dn16_slot: &mut f64,
        var_xsubct_dn17_slot: &mut f64,
        var_xsubct_dn18_slot: &mut f64,
        var_xsubct_dn19_slot: &mut f64,
        var_xsubct_dn20_slot: &mut f64,
        var_xsubct_dn5_slot: &mut f64,
        var_xsubct_dn6_slot: &mut f64,
        var_xsubct_dn7_slot: &mut f64,
        var_xsubct_dn8_slot: &mut f64,
        var_xsubct_rv_slot: &mut f64,
        var_xwict_slot: &mut f64,
        var_xwict_dn12_slot: &mut f64,
        var_xwict_dn13_slot: &mut f64,
        var_xwict_dn14_slot: &mut f64,
        var_xwict_dn15_slot: &mut f64,
        var_xwict_dn16_slot: &mut f64,
        var_xwict_dn17_slot: &mut f64,
        var_xwict_dn18_slot: &mut f64,
        var_xwict_dn19_slot: &mut f64,
        var_xwict_dn20_slot: &mut f64,
        var_xwict_dn5_slot: &mut f64,
        var_xwict_dn6_slot: &mut f64,
        var_xwict_dn7_slot: &mut f64,
        var_xwict_dn8_slot: &mut f64,
        var_xwict_rv_slot: &mut f64,
    ) {
        let mut var_aphi: f64 = *var_aphi_slot;
        let mut var_aphi_rv: f64 = *var_aphi_rv_slot;
        let mut var_arloc: f64 = *var_arloc_slot;
        let mut var_arloc_rv: f64 = *var_arloc_rv_slot;
        let mut var_dctg: f64 = *var_dctg_slot;
        let mut var_dctg_dn12: f64 = *var_dctg_dn12_slot;
        let mut var_dctg_dn13: f64 = *var_dctg_dn13_slot;
        let mut var_dctg_dn14: f64 = *var_dctg_dn14_slot;
        let mut var_dctg_dn15: f64 = *var_dctg_dn15_slot;
        let mut var_dctg_dn16: f64 = *var_dctg_dn16_slot;
        let mut var_dctg_dn17: f64 = *var_dctg_dn17_slot;
        let mut var_dctg_dn18: f64 = *var_dctg_dn18_slot;
        let mut var_dctg_dn19: f64 = *var_dctg_dn19_slot;
        let mut var_dctg_dn20: f64 = *var_dctg_dn20_slot;
        let mut var_dctg_dn5: f64 = *var_dctg_dn5_slot;
        let mut var_dctg_dn6: f64 = *var_dctg_dn6_slot;
        let mut var_dctg_dn7: f64 = *var_dctg_dn7_slot;
        let mut var_dctg_dn8: f64 = *var_dctg_dn8_slot;
        let mut var_dctg_rv: f64 = *var_dctg_rv_slot;
        let mut var_dvbstar: f64 = *var_dvbstar_slot;
        let mut var_dvbstar_dc: f64 = *var_dvbstar_dc_slot;
        let mut var_dvbstar_dc_dn12: f64 = *var_dvbstar_dc_dn12_slot;
        let mut var_dvbstar_dc_dn13: f64 = *var_dvbstar_dc_dn13_slot;
        let mut var_dvbstar_dc_dn14: f64 = *var_dvbstar_dc_dn14_slot;
        let mut var_dvbstar_dc_dn15: f64 = *var_dvbstar_dc_dn15_slot;
        let mut var_dvbstar_dc_dn16: f64 = *var_dvbstar_dc_dn16_slot;
        let mut var_dvbstar_dc_dn17: f64 = *var_dvbstar_dc_dn17_slot;
        let mut var_dvbstar_dc_dn18: f64 = *var_dvbstar_dc_dn18_slot;
        let mut var_dvbstar_dc_dn19: f64 = *var_dvbstar_dc_dn19_slot;
        let mut var_dvbstar_dc_dn20: f64 = *var_dvbstar_dc_dn20_slot;
        let mut var_dvbstar_dc_dn5: f64 = *var_dvbstar_dc_dn5_slot;
        let mut var_dvbstar_dc_dn6: f64 = *var_dvbstar_dc_dn6_slot;
        let mut var_dvbstar_dc_dn7: f64 = *var_dvbstar_dc_dn7_slot;
        let mut var_dvbstar_dc_dn8: f64 = *var_dvbstar_dc_dn8_slot;
        let mut var_dvbstar_dc_rv: f64 = *var_dvbstar_dc_rv_slot;
        let mut var_dvbstar_dn12: f64 = *var_dvbstar_dn12_slot;
        let mut var_dvbstar_dn13: f64 = *var_dvbstar_dn13_slot;
        let mut var_dvbstar_dn14: f64 = *var_dvbstar_dn14_slot;
        let mut var_dvbstar_dn15: f64 = *var_dvbstar_dn15_slot;
        let mut var_dvbstar_dn16: f64 = *var_dvbstar_dn16_slot;
        let mut var_dvbstar_dn17: f64 = *var_dvbstar_dn17_slot;
        let mut var_dvbstar_dn18: f64 = *var_dvbstar_dn18_slot;
        let mut var_dvbstar_dn19: f64 = *var_dvbstar_dn19_slot;
        let mut var_dvbstar_dn20: f64 = *var_dvbstar_dn20_slot;
        let mut var_dvbstar_dn5: f64 = *var_dvbstar_dn5_slot;
        let mut var_dvbstar_dn6: f64 = *var_dvbstar_dn6_slot;
        let mut var_dvbstar_dn7: f64 = *var_dvbstar_dn7_slot;
        let mut var_dvbstar_dn8: f64 = *var_dvbstar_dn8_slot;
        let mut var_dvbstar_rv: f64 = *var_dvbstar_rv_slot;
        let mut var_g_0: f64 = *var_g_0_slot;
        let mut var_g_0_rv: f64 = *var_g_0_rv_slot;
        let mut var_guard1275: f64 = *var_guard1275_slot;
        let mut var_guard1275_rv: f64 = *var_guard1275_rv_slot;
        let mut var_phib: f64 = *var_phib_slot;
        let mut var_phib_rv: f64 = *var_phib_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn12: f64 = *var_temp1_dn12_slot;
        let mut var_temp1_dn13: f64 = *var_temp1_dn13_slot;
        let mut var_temp1_dn14: f64 = *var_temp1_dn14_slot;
        let mut var_temp1_dn15: f64 = *var_temp1_dn15_slot;
        let mut var_temp1_dn16: f64 = *var_temp1_dn16_slot;
        let mut var_temp1_dn17: f64 = *var_temp1_dn17_slot;
        let mut var_temp1_dn18: f64 = *var_temp1_dn18_slot;
        let mut var_temp1_dn19: f64 = *var_temp1_dn19_slot;
        let mut var_temp1_dn20: f64 = *var_temp1_dn20_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn12: f64 = *var_temp2_dn12_slot;
        let mut var_temp2_dn13: f64 = *var_temp2_dn13_slot;
        let mut var_temp2_dn14: f64 = *var_temp2_dn14_slot;
        let mut var_temp2_dn15: f64 = *var_temp2_dn15_slot;
        let mut var_temp2_dn16: f64 = *var_temp2_dn16_slot;
        let mut var_temp2_dn17: f64 = *var_temp2_dn17_slot;
        let mut var_temp2_dn18: f64 = *var_temp2_dn18_slot;
        let mut var_temp2_dn19: f64 = *var_temp2_dn19_slot;
        let mut var_temp2_dn20: f64 = *var_temp2_dn20_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_thesatloc: f64 = *var_thesatloc_slot;
        let mut var_thesatloc_rv: f64 = *var_thesatloc_rv_slot;
        let mut var_vgb1: f64 = *var_vgb1_slot;
        let mut var_vgb1_dn12: f64 = *var_vgb1_dn12_slot;
        let mut var_vgb1_dn13: f64 = *var_vgb1_dn13_slot;
        let mut var_vgb1_dn14: f64 = *var_vgb1_dn14_slot;
        let mut var_vgb1_dn15: f64 = *var_vgb1_dn15_slot;
        let mut var_vgb1_dn16: f64 = *var_vgb1_dn16_slot;
        let mut var_vgb1_dn17: f64 = *var_vgb1_dn17_slot;
        let mut var_vgb1_dn18: f64 = *var_vgb1_dn18_slot;
        let mut var_vgb1_dn19: f64 = *var_vgb1_dn19_slot;
        let mut var_vgb1_dn20: f64 = *var_vgb1_dn20_slot;
        let mut var_vgb1_dn5: f64 = *var_vgb1_dn5_slot;
        let mut var_vgb1_dn6: f64 = *var_vgb1_dn6_slot;
        let mut var_vgb1_dn7: f64 = *var_vgb1_dn7_slot;
        let mut var_vgb1_dn8: f64 = *var_vgb1_dn8_slot;
        let mut var_vgb1_rv: f64 = *var_vgb1_rv_slot;
        let mut var_vsbstar: f64 = *var_vsbstar_slot;
        let mut var_vsbstar_dc: f64 = *var_vsbstar_dc_slot;
        let mut var_vsbstar_dc_dn12: f64 = *var_vsbstar_dc_dn12_slot;
        let mut var_vsbstar_dc_dn13: f64 = *var_vsbstar_dc_dn13_slot;
        let mut var_vsbstar_dc_dn14: f64 = *var_vsbstar_dc_dn14_slot;
        let mut var_vsbstar_dc_dn15: f64 = *var_vsbstar_dc_dn15_slot;
        let mut var_vsbstar_dc_dn16: f64 = *var_vsbstar_dc_dn16_slot;
        let mut var_vsbstar_dc_dn17: f64 = *var_vsbstar_dc_dn17_slot;
        let mut var_vsbstar_dc_dn18: f64 = *var_vsbstar_dc_dn18_slot;
        let mut var_vsbstar_dc_dn19: f64 = *var_vsbstar_dc_dn19_slot;
        let mut var_vsbstar_dc_dn20: f64 = *var_vsbstar_dc_dn20_slot;
        let mut var_vsbstar_dc_dn5: f64 = *var_vsbstar_dc_dn5_slot;
        let mut var_vsbstar_dc_dn6: f64 = *var_vsbstar_dc_dn6_slot;
        let mut var_vsbstar_dc_dn7: f64 = *var_vsbstar_dc_dn7_slot;
        let mut var_vsbstar_dc_dn8: f64 = *var_vsbstar_dc_dn8_slot;
        let mut var_vsbstar_dc_rv: f64 = *var_vsbstar_dc_rv_slot;
        let mut var_vsbstar_dn12: f64 = *var_vsbstar_dn12_slot;
        let mut var_vsbstar_dn13: f64 = *var_vsbstar_dn13_slot;
        let mut var_vsbstar_dn14: f64 = *var_vsbstar_dn14_slot;
        let mut var_vsbstar_dn15: f64 = *var_vsbstar_dn15_slot;
        let mut var_vsbstar_dn16: f64 = *var_vsbstar_dn16_slot;
        let mut var_vsbstar_dn17: f64 = *var_vsbstar_dn17_slot;
        let mut var_vsbstar_dn18: f64 = *var_vsbstar_dn18_slot;
        let mut var_vsbstar_dn19: f64 = *var_vsbstar_dn19_slot;
        let mut var_vsbstar_dn20: f64 = *var_vsbstar_dn20_slot;
        let mut var_vsbstar_dn5: f64 = *var_vsbstar_dn5_slot;
        let mut var_vsbstar_dn6: f64 = *var_vsbstar_dn6_slot;
        let mut var_vsbstar_dn7: f64 = *var_vsbstar_dn7_slot;
        let mut var_vsbstar_dn8: f64 = *var_vsbstar_dn8_slot;
        let mut var_vsbstar_rv: f64 = *var_vsbstar_rv_slot;
        let mut var_vsbx: f64 = *var_vsbx_slot;
        let mut var_vsbx_dn12: f64 = *var_vsbx_dn12_slot;
        let mut var_vsbx_dn13: f64 = *var_vsbx_dn13_slot;
        let mut var_vsbx_dn14: f64 = *var_vsbx_dn14_slot;
        let mut var_vsbx_dn15: f64 = *var_vsbx_dn15_slot;
        let mut var_vsbx_dn16: f64 = *var_vsbx_dn16_slot;
        let mut var_vsbx_dn17: f64 = *var_vsbx_dn17_slot;
        let mut var_vsbx_dn18: f64 = *var_vsbx_dn18_slot;
        let mut var_vsbx_dn19: f64 = *var_vsbx_dn19_slot;
        let mut var_vsbx_dn20: f64 = *var_vsbx_dn20_slot;
        let mut var_vsbx_dn5: f64 = *var_vsbx_dn5_slot;
        let mut var_vsbx_dn6: f64 = *var_vsbx_dn6_slot;
        let mut var_vsbx_dn7: f64 = *var_vsbx_dn7_slot;
        let mut var_vsbx_dn8: f64 = *var_vsbx_dn8_slot;
        let mut var_vsbx_rv: f64 = *var_vsbx_rv_slot;
        let mut var_xbct: f64 = *var_xbct_slot;
        let mut var_xbct_rv: f64 = *var_xbct_rv_slot;
        let mut var_xctmax: f64 = *var_xctmax_slot;
        let mut var_xctmax_rv: f64 = *var_xctmax_rv_slot;
        let mut var_xgct: f64 = *var_xgct_slot;
        let mut var_xgct_dn12: f64 = *var_xgct_dn12_slot;
        let mut var_xgct_dn13: f64 = *var_xgct_dn13_slot;
        let mut var_xgct_dn14: f64 = *var_xgct_dn14_slot;
        let mut var_xgct_dn15: f64 = *var_xgct_dn15_slot;
        let mut var_xgct_dn16: f64 = *var_xgct_dn16_slot;
        let mut var_xgct_dn17: f64 = *var_xgct_dn17_slot;
        let mut var_xgct_dn18: f64 = *var_xgct_dn18_slot;
        let mut var_xgct_dn19: f64 = *var_xgct_dn19_slot;
        let mut var_xgct_dn20: f64 = *var_xgct_dn20_slot;
        let mut var_xgct_dn5: f64 = *var_xgct_dn5_slot;
        let mut var_xgct_dn6: f64 = *var_xgct_dn6_slot;
        let mut var_xgct_dn7: f64 = *var_xgct_dn7_slot;
        let mut var_xgct_dn8: f64 = *var_xgct_dn8_slot;
        let mut var_xgct_rv: f64 = *var_xgct_rv_slot;
        let mut var_xmict: f64 = *var_xmict_slot;
        let mut var_xmict_dn12: f64 = *var_xmict_dn12_slot;
        let mut var_xmict_dn13: f64 = *var_xmict_dn13_slot;
        let mut var_xmict_dn14: f64 = *var_xmict_dn14_slot;
        let mut var_xmict_dn15: f64 = *var_xmict_dn15_slot;
        let mut var_xmict_dn16: f64 = *var_xmict_dn16_slot;
        let mut var_xmict_dn17: f64 = *var_xmict_dn17_slot;
        let mut var_xmict_dn18: f64 = *var_xmict_dn18_slot;
        let mut var_xmict_dn19: f64 = *var_xmict_dn19_slot;
        let mut var_xmict_dn20: f64 = *var_xmict_dn20_slot;
        let mut var_xmict_dn5: f64 = *var_xmict_dn5_slot;
        let mut var_xmict_dn6: f64 = *var_xmict_dn6_slot;
        let mut var_xmict_dn7: f64 = *var_xmict_dn7_slot;
        let mut var_xmict_dn8: f64 = *var_xmict_dn8_slot;
        let mut var_xmict_rv: f64 = *var_xmict_rv_slot;
        let mut var_xnct: f64 = *var_xnct_slot;
        let mut var_xnct_dn12: f64 = *var_xnct_dn12_slot;
        let mut var_xnct_dn13: f64 = *var_xnct_dn13_slot;
        let mut var_xnct_dn14: f64 = *var_xnct_dn14_slot;
        let mut var_xnct_dn15: f64 = *var_xnct_dn15_slot;
        let mut var_xnct_dn16: f64 = *var_xnct_dn16_slot;
        let mut var_xnct_dn17: f64 = *var_xnct_dn17_slot;
        let mut var_xnct_dn18: f64 = *var_xnct_dn18_slot;
        let mut var_xnct_dn19: f64 = *var_xnct_dn19_slot;
        let mut var_xnct_dn20: f64 = *var_xnct_dn20_slot;
        let mut var_xnct_dn5: f64 = *var_xnct_dn5_slot;
        let mut var_xnct_dn6: f64 = *var_xnct_dn6_slot;
        let mut var_xnct_dn7: f64 = *var_xnct_dn7_slot;
        let mut var_xnct_dn8: f64 = *var_xnct_dn8_slot;
        let mut var_xnct_rv: f64 = *var_xnct_rv_slot;
        let mut var_xsbstar: f64 = *var_xsbstar_slot;
        let mut var_xsbstar_dn12: f64 = *var_xsbstar_dn12_slot;
        let mut var_xsbstar_dn13: f64 = *var_xsbstar_dn13_slot;
        let mut var_xsbstar_dn14: f64 = *var_xsbstar_dn14_slot;
        let mut var_xsbstar_dn15: f64 = *var_xsbstar_dn15_slot;
        let mut var_xsbstar_dn16: f64 = *var_xsbstar_dn16_slot;
        let mut var_xsbstar_dn17: f64 = *var_xsbstar_dn17_slot;
        let mut var_xsbstar_dn18: f64 = *var_xsbstar_dn18_slot;
        let mut var_xsbstar_dn19: f64 = *var_xsbstar_dn19_slot;
        let mut var_xsbstar_dn20: f64 = *var_xsbstar_dn20_slot;
        let mut var_xsbstar_dn5: f64 = *var_xsbstar_dn5_slot;
        let mut var_xsbstar_dn6: f64 = *var_xsbstar_dn6_slot;
        let mut var_xsbstar_dn7: f64 = *var_xsbstar_dn7_slot;
        let mut var_xsbstar_dn8: f64 = *var_xsbstar_dn8_slot;
        let mut var_xsbstar_rv: f64 = *var_xsbstar_rv_slot;
        let mut var_xsubct: f64 = *var_xsubct_slot;
        let mut var_xsubct_dn12: f64 = *var_xsubct_dn12_slot;
        let mut var_xsubct_dn13: f64 = *var_xsubct_dn13_slot;
        let mut var_xsubct_dn14: f64 = *var_xsubct_dn14_slot;
        let mut var_xsubct_dn15: f64 = *var_xsubct_dn15_slot;
        let mut var_xsubct_dn16: f64 = *var_xsubct_dn16_slot;
        let mut var_xsubct_dn17: f64 = *var_xsubct_dn17_slot;
        let mut var_xsubct_dn18: f64 = *var_xsubct_dn18_slot;
        let mut var_xsubct_dn19: f64 = *var_xsubct_dn19_slot;
        let mut var_xsubct_dn20: f64 = *var_xsubct_dn20_slot;
        let mut var_xsubct_dn5: f64 = *var_xsubct_dn5_slot;
        let mut var_xsubct_dn6: f64 = *var_xsubct_dn6_slot;
        let mut var_xsubct_dn7: f64 = *var_xsubct_dn7_slot;
        let mut var_xsubct_dn8: f64 = *var_xsubct_dn8_slot;
        let mut var_xsubct_rv: f64 = *var_xsubct_rv_slot;
        let mut var_xwict: f64 = *var_xwict_slot;
        let mut var_xwict_dn12: f64 = *var_xwict_dn12_slot;
        let mut var_xwict_dn13: f64 = *var_xwict_dn13_slot;
        let mut var_xwict_dn14: f64 = *var_xwict_dn14_slot;
        let mut var_xwict_dn15: f64 = *var_xwict_dn15_slot;
        let mut var_xwict_dn16: f64 = *var_xwict_dn16_slot;
        let mut var_xwict_dn17: f64 = *var_xwict_dn17_slot;
        let mut var_xwict_dn18: f64 = *var_xwict_dn18_slot;
        let mut var_xwict_dn19: f64 = *var_xwict_dn19_slot;
        let mut var_xwict_dn20: f64 = *var_xwict_dn20_slot;
        let mut var_xwict_dn5: f64 = *var_xwict_dn5_slot;
        let mut var_xwict_dn6: f64 = *var_xwict_dn6_slot;
        let mut var_xwict_dn7: f64 = *var_xwict_dn7_slot;
        let mut var_xwict_dn8: f64 = *var_xwict_dn8_slot;
        let mut var_xwict_rv: f64 = *var_xwict_rv_slot;

        let (assign40910_e53955, assign40910_e53955_d_n5, assign40910_e53955_d_n6, assign40910_e53955_d_n7, assign40910_e53955_d_n8, assign40910_e53955_d_n12, assign40910_e53955_d_n13, assign40910_e53955_d_n14, assign40910_e53955_d_n15, assign40910_e53955_d_n16, assign40910_e53955_d_n17, assign40910_e53955_d_n18, assign40910_e53955_d_n19, assign40910_e53955_d_n20,) = {
    if (var_guard1274 != 0.0) {
        let assign40910_e53951: f64 = (var_v_ds - var_vdsx);
        let assign40910_e53952: f64 = (0.5 * assign40910_e53951);
        let assign40910_e53953: f64 = (var_vmbnew - assign40910_e53952);
        (assign40910_e53953, var_vmbnew_dn5, (var_vmbnew_dn6 - (0.5 * (var_v_ds_dn6 - var_vdsx_dn6))), (var_vmbnew_dn7 - (0.5 * (var_v_ds_dn7 - var_vdsx_dn7))), var_vmbnew_dn8, var_vmbnew_dn12, var_vmbnew_dn13, var_vmbnew_dn14, var_vmbnew_dn15, var_vmbnew_dn16, var_vmbnew_dn17, var_vmbnew_dn18, var_vmbnew_dn19, var_vmbnew_dn20,)
    } else {
        (var_vsbstar_dc, var_vsbstar_dc_dn5, var_vsbstar_dc_dn6, var_vsbstar_dc_dn7, var_vsbstar_dc_dn8, var_vsbstar_dc_dn12, var_vsbstar_dc_dn13, var_vsbstar_dc_dn14, var_vsbstar_dc_dn15, var_vsbstar_dc_dn16, var_vsbstar_dc_dn17, var_vsbstar_dc_dn18, var_vsbstar_dc_dn19, var_vsbstar_dc_dn20,)
    }
};
        var_vsbstar_dc = assign40910_e53955;
        var_vsbstar_dc_dn5 = assign40910_e53955_d_n5;
        var_vsbstar_dc_dn6 = assign40910_e53955_d_n6;
        var_vsbstar_dc_dn7 = assign40910_e53955_d_n7;
        var_vsbstar_dc_dn8 = assign40910_e53955_d_n8;
        var_vsbstar_dc_dn12 = assign40910_e53955_d_n12;
        var_vsbstar_dc_dn13 = assign40910_e53955_d_n13;
        var_vsbstar_dc_dn14 = assign40910_e53955_d_n14;
        var_vsbstar_dc_dn15 = assign40910_e53955_d_n15;
        var_vsbstar_dc_dn16 = assign40910_e53955_d_n16;
        var_vsbstar_dc_dn17 = assign40910_e53955_d_n17;
        var_vsbstar_dc_dn18 = assign40910_e53955_d_n18;
        var_vsbstar_dc_dn19 = assign40910_e53955_d_n19;
        var_vsbstar_dc_dn20 = assign40910_e53955_d_n20;
        var_vsbstar_dc_rv = 0.0;

        let (assign40920_e53961, assign40920_e53961_d_n5, assign40920_e53961_d_n6, assign40920_e53961_d_n7, assign40920_e53961_d_n8, assign40920_e53961_d_n12, assign40920_e53961_d_n13, assign40920_e53961_d_n14, assign40920_e53961_d_n15, assign40920_e53961_d_n16, assign40920_e53961_d_n17, assign40920_e53961_d_n18, assign40920_e53961_d_n19, assign40920_e53961_d_n20,) = {
    if (var_guard1274 != 0.0) {
        let assign40920_e53959: f64 = (var_vsbstar_dc_tmp - var_vsbstar_dc);
        (assign40920_e53959, (var_vsbstar_dc_tmp_dn5 - var_vsbstar_dc_dn5), (var_vsbstar_dc_tmp_dn6 - var_vsbstar_dc_dn6), (var_vsbstar_dc_tmp_dn7 - var_vsbstar_dc_dn7), (var_vsbstar_dc_tmp_dn8 - var_vsbstar_dc_dn8), (var_vsbstar_dc_tmp_dn12 - var_vsbstar_dc_dn12), (var_vsbstar_dc_tmp_dn13 - var_vsbstar_dc_dn13), (var_vsbstar_dc_tmp_dn14 - var_vsbstar_dc_dn14), (var_vsbstar_dc_tmp_dn15 - var_vsbstar_dc_dn15), (var_vsbstar_dc_tmp_dn16 - var_vsbstar_dc_dn16), (var_vsbstar_dc_tmp_dn17 - var_vsbstar_dc_dn17), (var_vsbstar_dc_tmp_dn18 - var_vsbstar_dc_dn18), (var_vsbstar_dc_tmp_dn19 - var_vsbstar_dc_dn19), (var_vsbstar_dc_tmp_dn20 - var_vsbstar_dc_dn20),)
    } else {
        (var_dvbstar_dc, var_dvbstar_dc_dn5, var_dvbstar_dc_dn6, var_dvbstar_dc_dn7, var_dvbstar_dc_dn8, var_dvbstar_dc_dn12, var_dvbstar_dc_dn13, var_dvbstar_dc_dn14, var_dvbstar_dc_dn15, var_dvbstar_dc_dn16, var_dvbstar_dc_dn17, var_dvbstar_dc_dn18, var_dvbstar_dc_dn19, var_dvbstar_dc_dn20,)
    }
};
        var_dvbstar_dc = assign40920_e53961;
        var_dvbstar_dc_dn5 = assign40920_e53961_d_n5;
        var_dvbstar_dc_dn6 = assign40920_e53961_d_n6;
        var_dvbstar_dc_dn7 = assign40920_e53961_d_n7;
        var_dvbstar_dc_dn8 = assign40920_e53961_d_n8;
        var_dvbstar_dc_dn12 = assign40920_e53961_d_n12;
        var_dvbstar_dc_dn13 = assign40920_e53961_d_n13;
        var_dvbstar_dc_dn14 = assign40920_e53961_d_n14;
        var_dvbstar_dc_dn15 = assign40920_e53961_d_n15;
        var_dvbstar_dc_dn16 = assign40920_e53961_d_n16;
        var_dvbstar_dc_dn17 = assign40920_e53961_d_n17;
        var_dvbstar_dc_dn18 = assign40920_e53961_d_n18;
        var_dvbstar_dc_dn19 = assign40920_e53961_d_n19;
        var_dvbstar_dc_dn20 = assign40920_e53961_d_n20;
        var_dvbstar_dc_rv = 0.0;

        var_phib = var_phib_dc;
        var_phib_rv = 0.0;

        var_aphi = var_aphi_dc;
        var_aphi_rv = 0.0;

        var_g_0 = var_g_0_dc;
        var_g_0_rv = 0.0;

        var_vsbstar = var_vsbstar_dc;
        var_vsbstar_dn5 = var_vsbstar_dc_dn5;
        var_vsbstar_dn6 = var_vsbstar_dc_dn6;
        var_vsbstar_dn7 = var_vsbstar_dc_dn7;
        var_vsbstar_dn8 = var_vsbstar_dc_dn8;
        var_vsbstar_dn12 = var_vsbstar_dc_dn12;
        var_vsbstar_dn13 = var_vsbstar_dc_dn13;
        var_vsbstar_dn14 = var_vsbstar_dc_dn14;
        var_vsbstar_dn15 = var_vsbstar_dc_dn15;
        var_vsbstar_dn16 = var_vsbstar_dc_dn16;
        var_vsbstar_dn17 = var_vsbstar_dc_dn17;
        var_vsbstar_dn18 = var_vsbstar_dc_dn18;
        var_vsbstar_dn19 = var_vsbstar_dc_dn19;
        var_vsbstar_dn20 = var_vsbstar_dc_dn20;
        var_vsbstar_rv = 0.0;

        var_dvbstar = var_dvbstar_dc;
        var_dvbstar_dn5 = var_dvbstar_dc_dn5;
        var_dvbstar_dn6 = var_dvbstar_dc_dn6;
        var_dvbstar_dn7 = var_dvbstar_dc_dn7;
        var_dvbstar_dn8 = var_dvbstar_dc_dn8;
        var_dvbstar_dn12 = var_dvbstar_dc_dn12;
        var_dvbstar_dn13 = var_dvbstar_dc_dn13;
        var_dvbstar_dn14 = var_dvbstar_dc_dn14;
        var_dvbstar_dn15 = var_dvbstar_dc_dn15;
        var_dvbstar_dn16 = var_dvbstar_dc_dn16;
        var_dvbstar_dn17 = var_dvbstar_dc_dn17;
        var_dvbstar_dn18 = var_dvbstar_dc_dn18;
        var_dvbstar_dn19 = var_dvbstar_dc_dn19;
        var_dvbstar_dn20 = var_dvbstar_dc_dn20;
        var_dvbstar_rv = 0.0;

        var_thesatloc = var_thesat_t;
        var_thesatloc_rv = 0.0;

        var_arloc = var_ar;
        var_arloc_rv = 0.0;

        let assign41000_e53971: f64 = (var_vgb - var_dvbstar);
        let assign41000_e53973: f64 = (assign41000_e53971 - var_vfb_t);
        var_vgb1 = assign41000_e53973;
        var_vgb1_dn5 = (var_vgb_dn5 - var_dvbstar_dn5);
        var_vgb1_dn6 = (var_vgb_dn6 - var_dvbstar_dn6);
        var_vgb1_dn7 = (var_vgb_dn7 - var_dvbstar_dn7);
        var_vgb1_dn8 = (var_vgb_dn8 - var_dvbstar_dn8);
        var_vgb1_dn12 = (-var_dvbstar_dn12);
        var_vgb1_dn13 = (-var_dvbstar_dn13);
        var_vgb1_dn14 = (-var_dvbstar_dn14);
        var_vgb1_dn15 = (-var_dvbstar_dn15);
        var_vgb1_dn16 = (-var_dvbstar_dn16);
        var_vgb1_dn17 = (-var_dvbstar_dn17);
        var_vgb1_dn18 = (-var_dvbstar_dn18);
        var_vgb1_dn19 = (-var_dvbstar_dn19);
        var_vgb1_dn20 = (-var_dvbstar_dn20);
        var_vgb1_rv = 0.0;

        let assign41010_e53978: f64 = (var_v_ds - var_vdsx);
        let assign41010_e53979: f64 = (0.5 * assign41010_e53978);
        let assign41010_e53980: f64 = (var_vsbstar + assign41010_e53979);
        var_vsbx = assign41010_e53980;
        var_vsbx_dn5 = var_vsbstar_dn5;
        var_vsbx_dn6 = (var_vsbstar_dn6 + (0.5 * (var_v_ds_dn6 - var_vdsx_dn6)));
        var_vsbx_dn7 = (var_vsbstar_dn7 + (0.5 * (var_v_ds_dn7 - var_vdsx_dn7)));
        var_vsbx_dn8 = var_vsbstar_dn8;
        var_vsbx_dn12 = var_vsbstar_dn12;
        var_vsbx_dn13 = var_vsbstar_dn13;
        var_vsbx_dn14 = var_vsbstar_dn14;
        var_vsbx_dn15 = var_vsbstar_dn15;
        var_vsbx_dn16 = var_vsbstar_dn16;
        var_vsbx_dn17 = var_vsbstar_dn17;
        var_vsbx_dn18 = var_vsbstar_dn18;
        var_vsbx_dn19 = var_vsbstar_dn19;
        var_vsbx_dn20 = var_vsbstar_dn20;
        var_vsbx_rv = 0.0;

        var_dctg = 1.0;
        var_dctg_dn5 = 0.0;
        var_dctg_dn6 = 0.0;
        var_dctg_dn7 = 0.0;
        var_dctg_dn8 = 0.0;
        var_dctg_dn12 = 0.0;
        var_dctg_dn13 = 0.0;
        var_dctg_dn14 = 0.0;
        var_dctg_dn15 = 0.0;
        var_dctg_dn16 = 0.0;
        var_dctg_dn17 = 0.0;
        var_dctg_dn18 = 0.0;
        var_dctg_dn19 = 0.0;
        var_dctg_dn20 = 0.0;
        var_dctg_rv = 0.0;

        let assign41030_e53984: f64 = if var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1275 = assign41030_e53984;
        var_guard1275_rv = 0.0;

        let (assign41040_e53990,) = {
    if (var_guard1275 != 0.0) {
        let assign41040_e53988: f64 = (var_phib * var_inv_phit);
        (assign41040_e53988,)
    } else {
        (var_xbct,)
    }
};
        var_xbct = assign41040_e53990;
        var_xbct_rv = 0.0;

        let (assign41050_e53996, assign41050_e53996_d_n5, assign41050_e53996_d_n6, assign41050_e53996_d_n7, assign41050_e53996_d_n8, assign41050_e53996_d_n12, assign41050_e53996_d_n13, assign41050_e53996_d_n14, assign41050_e53996_d_n15, assign41050_e53996_d_n16, assign41050_e53996_d_n17, assign41050_e53996_d_n18, assign41050_e53996_d_n19, assign41050_e53996_d_n20,) = {
    if (var_guard1275 != 0.0) {
        let assign41050_e53994: f64 = (var_vsbx * var_inv_phit);
        (assign41050_e53994, (var_vsbx_dn5 * var_inv_phit), (var_vsbx_dn6 * var_inv_phit), (var_vsbx_dn7 * var_inv_phit), (var_vsbx_dn8 * var_inv_phit), (var_vsbx_dn12 * var_inv_phit), (var_vsbx_dn13 * var_inv_phit), (var_vsbx_dn14 * var_inv_phit), (var_vsbx_dn15 * var_inv_phit), (var_vsbx_dn16 * var_inv_phit), (var_vsbx_dn17 * var_inv_phit), (var_vsbx_dn18 * var_inv_phit), (var_vsbx_dn19 * var_inv_phit), (var_vsbx_dn20 * var_inv_phit),)
    } else {
        (var_xsbstar, var_xsbstar_dn5, var_xsbstar_dn6, var_xsbstar_dn7, var_xsbstar_dn8, var_xsbstar_dn12, var_xsbstar_dn13, var_xsbstar_dn14, var_xsbstar_dn15, var_xsbstar_dn16, var_xsbstar_dn17, var_xsbstar_dn18, var_xsbstar_dn19, var_xsbstar_dn20,)
    }
};
        var_xsbstar = assign41050_e53996;
        var_xsbstar_dn5 = assign41050_e53996_d_n5;
        var_xsbstar_dn6 = assign41050_e53996_d_n6;
        var_xsbstar_dn7 = assign41050_e53996_d_n7;
        var_xsbstar_dn8 = assign41050_e53996_d_n8;
        var_xsbstar_dn12 = assign41050_e53996_d_n12;
        var_xsbstar_dn13 = assign41050_e53996_d_n13;
        var_xsbstar_dn14 = assign41050_e53996_d_n14;
        var_xsbstar_dn15 = assign41050_e53996_d_n15;
        var_xsbstar_dn16 = assign41050_e53996_d_n16;
        var_xsbstar_dn17 = assign41050_e53996_d_n17;
        var_xsbstar_dn18 = assign41050_e53996_d_n18;
        var_xsbstar_dn19 = assign41050_e53996_d_n19;
        var_xsbstar_dn20 = assign41050_e53996_d_n20;
        var_xsbstar_rv = 0.0;

        let (assign41060_e54002, assign41060_e54002_d_n5, assign41060_e54002_d_n6, assign41060_e54002_d_n7, assign41060_e54002_d_n8, assign41060_e54002_d_n12, assign41060_e54002_d_n13, assign41060_e54002_d_n14, assign41060_e54002_d_n15, assign41060_e54002_d_n16, assign41060_e54002_d_n17, assign41060_e54002_d_n18, assign41060_e54002_d_n19, assign41060_e54002_d_n20,) = {
    if (var_guard1275 != 0.0) {
        let assign41060_e54000: f64 = (var_vgb1 * var_inv_phit);
        (assign41060_e54000, (var_vgb1_dn5 * var_inv_phit), (var_vgb1_dn6 * var_inv_phit), (var_vgb1_dn7 * var_inv_phit), (var_vgb1_dn8 * var_inv_phit), (var_vgb1_dn12 * var_inv_phit), (var_vgb1_dn13 * var_inv_phit), (var_vgb1_dn14 * var_inv_phit), (var_vgb1_dn15 * var_inv_phit), (var_vgb1_dn16 * var_inv_phit), (var_vgb1_dn17 * var_inv_phit), (var_vgb1_dn18 * var_inv_phit), (var_vgb1_dn19 * var_inv_phit), (var_vgb1_dn20 * var_inv_phit),)
    } else {
        (var_xgct, var_xgct_dn5, var_xgct_dn6, var_xgct_dn7, var_xgct_dn8, var_xgct_dn12, var_xgct_dn13, var_xgct_dn14, var_xgct_dn15, var_xgct_dn16, var_xgct_dn17, var_xgct_dn18, var_xgct_dn19, var_xgct_dn20,)
    }
};
        var_xgct = assign41060_e54002;
        var_xgct_dn5 = assign41060_e54002_d_n5;
        var_xgct_dn6 = assign41060_e54002_d_n6;
        var_xgct_dn7 = assign41060_e54002_d_n7;
        var_xgct_dn8 = assign41060_e54002_d_n8;
        var_xgct_dn12 = assign41060_e54002_d_n12;
        var_xgct_dn13 = assign41060_e54002_d_n13;
        var_xgct_dn14 = assign41060_e54002_d_n14;
        var_xgct_dn15 = assign41060_e54002_d_n15;
        var_xgct_dn16 = assign41060_e54002_d_n16;
        var_xgct_dn17 = assign41060_e54002_d_n17;
        var_xgct_dn18 = assign41060_e54002_d_n18;
        var_xgct_dn19 = assign41060_e54002_d_n19;
        var_xgct_dn20 = assign41060_e54002_d_n20;
        var_xgct_rv = 0.0;

        let (assign41070_e54013, assign41070_e54013_d_n5, assign41070_e54013_d_n6, assign41070_e54013_d_n7, assign41070_e54013_d_n8, assign41070_e54013_d_n12, assign41070_e54013_d_n13, assign41070_e54013_d_n14, assign41070_e54013_d_n15, assign41070_e54013_d_n16, assign41070_e54013_d_n17, assign41070_e54013_d_n18, assign41070_e54013_d_n19, assign41070_e54013_d_n20,) = {
    if (var_guard1275 != 0.0) {
        let assign41070_e54007: f64 = (0.5 * var_g_0);
        let assign41070_e54009: f64 = (var_xbct).sqrt();
        let assign41070_e54010: f64 = (assign41070_e54007 / assign41070_e54009);
        let assign41070_e54011: f64 = (1.0 + assign41070_e54010);
        (assign41070_e54011, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn12, var_temp1_dn13, var_temp1_dn14, var_temp1_dn15, var_temp1_dn16, var_temp1_dn17, var_temp1_dn18, var_temp1_dn19, var_temp1_dn20,)
    }
};
        var_temp1 = assign41070_e54013;
        var_temp1_dn5 = assign41070_e54013_d_n5;
        var_temp1_dn6 = assign41070_e54013_d_n6;
        var_temp1_dn7 = assign41070_e54013_d_n7;
        var_temp1_dn8 = assign41070_e54013_d_n8;
        var_temp1_dn12 = assign41070_e54013_d_n12;
        var_temp1_dn13 = assign41070_e54013_d_n13;
        var_temp1_dn14 = assign41070_e54013_d_n14;
        var_temp1_dn15 = assign41070_e54013_d_n15;
        var_temp1_dn16 = assign41070_e54013_d_n16;
        var_temp1_dn17 = assign41070_e54013_d_n17;
        var_temp1_dn18 = assign41070_e54013_d_n18;
        var_temp1_dn19 = assign41070_e54013_d_n19;
        var_temp1_dn20 = assign41070_e54013_d_n20;
        var_temp1_rv = 0.0;

        let (assign41080_e54022, assign41080_e54022_d_n5, assign41080_e54022_d_n6, assign41080_e54022_d_n7, assign41080_e54022_d_n8, assign41080_e54022_d_n12, assign41080_e54022_d_n13, assign41080_e54022_d_n14, assign41080_e54022_d_n15, assign41080_e54022_d_n16, assign41080_e54022_d_n17, assign41080_e54022_d_n18, assign41080_e54022_d_n19, assign41080_e54022_d_n20,) = {
    if (var_guard1275 != 0.0) {
        let assign41080_e54018: f64 = (var_xbct).sqrt();
        let assign41080_e54019: f64 = (var_g_0 * assign41080_e54018);
        let assign41080_e54020: f64 = (var_xbct + assign41080_e54019);
        (assign41080_e54020, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn12, var_temp2_dn13, var_temp2_dn14, var_temp2_dn15, var_temp2_dn16, var_temp2_dn17, var_temp2_dn18, var_temp2_dn19, var_temp2_dn20,)
    }
};
        var_temp2 = assign41080_e54022;
        var_temp2_dn5 = assign41080_e54022_d_n5;
        var_temp2_dn6 = assign41080_e54022_d_n6;
        var_temp2_dn7 = assign41080_e54022_d_n7;
        var_temp2_dn8 = assign41080_e54022_d_n8;
        var_temp2_dn12 = assign41080_e54022_d_n12;
        var_temp2_dn13 = assign41080_e54022_d_n13;
        var_temp2_dn14 = assign41080_e54022_d_n14;
        var_temp2_dn15 = assign41080_e54022_d_n15;
        var_temp2_dn16 = assign41080_e54022_d_n16;
        var_temp2_dn17 = assign41080_e54022_d_n17;
        var_temp2_dn18 = assign41080_e54022_d_n18;
        var_temp2_dn19 = assign41080_e54022_d_n19;
        var_temp2_dn20 = assign41080_e54022_d_n20;
        var_temp2_rv = 0.0;

        let (assign41090_e54040, assign41090_e54040_d_n5, assign41090_e54040_d_n6, assign41090_e54040_d_n7, assign41090_e54040_d_n8, assign41090_e54040_d_n12, assign41090_e54040_d_n13, assign41090_e54040_d_n14, assign41090_e54040_d_n15, assign41090_e54040_d_n16, assign41090_e54040_d_n17, assign41090_e54040_d_n18, assign41090_e54040_d_n19, assign41090_e54040_d_n20,) = {
    if (var_guard1275 != 0.0) {
        let assign41090_e54026: f64 = (var_xgct - var_temp2);
        let assign41090_e54028: f64 = (assign41090_e54026 / var_temp1);
        let assign41090_e54031: f64 = (0.5 * var_xbct);
        let assign41090_e54032: f64 = (assign41090_e54028 + assign41090_e54031);
        let assign41090_e54035: f64 = (1.0 + var_ctb_i);
        let assign41090_e54037: f64 = (assign41090_e54035 * var_xsbstar);
        let assign41090_e54038: f64 = (assign41090_e54032 - assign41090_e54037);
        (assign41090_e54038, (((((var_xgct_dn5 - var_temp2_dn5) * var_temp1) - (assign41090_e54026 * var_temp1_dn5)) / (var_temp1 * var_temp1)) - (assign41090_e54035 * var_xsbstar_dn5)), (((((var_xgct_dn6 - var_temp2_dn6) * var_temp1) - (assign41090_e54026 * var_temp1_dn6)) / (var_temp1 * var_temp1)) - (assign41090_e54035 * var_xsbstar_dn6)), (((((var_xgct_dn7 - var_temp2_dn7) * var_temp1) - (assign41090_e54026 * var_temp1_dn7)) / (var_temp1 * var_temp1)) - (assign41090_e54035 * var_xsbstar_dn7)), (((((var_xgct_dn8 - var_temp2_dn8) * var_temp1) - (assign41090_e54026 * var_temp1_dn8)) / (var_temp1 * var_temp1)) - (assign41090_e54035 * var_xsbstar_dn8)), (((((var_xgct_dn12 - var_temp2_dn12) * var_temp1) - (assign41090_e54026 * var_temp1_dn12)) / (var_temp1 * var_temp1)) - (assign41090_e54035 * var_xsbstar_dn12)), (((((var_xgct_dn13 - var_temp2_dn13) * var_temp1) - (assign41090_e54026 * var_temp1_dn13)) / (var_temp1 * var_temp1)) - (assign41090_e54035 * var_xsbstar_dn13)), (((((var_xgct_dn14 - var_temp2_dn14) * var_temp1) - (assign41090_e54026 * var_temp1_dn14)) / (var_temp1 * var_temp1)) - (assign41090_e54035 * var_xsbstar_dn14)), (((((var_xgct_dn15 - var_temp2_dn15) * var_temp1) - (assign41090_e54026 * var_temp1_dn15)) / (var_temp1 * var_temp1)) - (assign41090_e54035 * var_xsbstar_dn15)), (((((var_xgct_dn16 - var_temp2_dn16) * var_temp1) - (assign41090_e54026 * var_temp1_dn16)) / (var_temp1 * var_temp1)) - (assign41090_e54035 * var_xsbstar_dn16)), (((((var_xgct_dn17 - var_temp2_dn17) * var_temp1) - (assign41090_e54026 * var_temp1_dn17)) / (var_temp1 * var_temp1)) - (assign41090_e54035 * var_xsbstar_dn17)), (((((var_xgct_dn18 - var_temp2_dn18) * var_temp1) - (assign41090_e54026 * var_temp1_dn18)) / (var_temp1 * var_temp1)) - (assign41090_e54035 * var_xsbstar_dn18)), (((((var_xgct_dn19 - var_temp2_dn19) * var_temp1) - (assign41090_e54026 * var_temp1_dn19)) / (var_temp1 * var_temp1)) - (assign41090_e54035 * var_xsbstar_dn19)), (((((var_xgct_dn20 - var_temp2_dn20) * var_temp1) - (assign41090_e54026 * var_temp1_dn20)) / (var_temp1 * var_temp1)) - (assign41090_e54035 * var_xsbstar_dn20)),)
    } else {
        (var_xwict, var_xwict_dn5, var_xwict_dn6, var_xwict_dn7, var_xwict_dn8, var_xwict_dn12, var_xwict_dn13, var_xwict_dn14, var_xwict_dn15, var_xwict_dn16, var_xwict_dn17, var_xwict_dn18, var_xwict_dn19, var_xwict_dn20,)
    }
};
        var_xwict = assign41090_e54040;
        var_xwict_dn5 = assign41090_e54040_d_n5;
        var_xwict_dn6 = assign41090_e54040_d_n6;
        var_xwict_dn7 = assign41090_e54040_d_n7;
        var_xwict_dn8 = assign41090_e54040_d_n8;
        var_xwict_dn12 = assign41090_e54040_d_n12;
        var_xwict_dn13 = assign41090_e54040_d_n13;
        var_xwict_dn14 = assign41090_e54040_d_n14;
        var_xwict_dn15 = assign41090_e54040_d_n15;
        var_xwict_dn16 = assign41090_e54040_d_n16;
        var_xwict_dn17 = assign41090_e54040_d_n17;
        var_xwict_dn18 = assign41090_e54040_d_n18;
        var_xwict_dn19 = assign41090_e54040_d_n19;
        var_xwict_dn20 = assign41090_e54040_d_n20;
        var_xwict_rv = 0.0;

        let (assign41100_e54048,) = {
    if (var_guard1275 != 0.0) {
        let assign41100_e54044: f64 = (0.5 * var_xbct);
        let assign41100_e54046: f64 = (assign41100_e54044 + 2.0);
        (assign41100_e54046,)
    } else {
        (var_xctmax,)
    }
};
        var_xctmax = assign41100_e54048;
        var_xctmax_rv = 0.0;

        let (assign41110_e54054, assign41110_e54054_d_n5, assign41110_e54054_d_n6, assign41110_e54054_d_n7, assign41110_e54054_d_n8, assign41110_e54054_d_n12, assign41110_e54054_d_n13, assign41110_e54054_d_n14, assign41110_e54054_d_n15, assign41110_e54054_d_n16, assign41110_e54054_d_n17, assign41110_e54054_d_n18, assign41110_e54054_d_n19, assign41110_e54054_d_n20,) = {
    if (var_guard1275 != 0.0) {
        let assign41110_e54052: f64 = (var_xbct + var_xsbstar);
        (assign41110_e54052, var_xsbstar_dn5, var_xsbstar_dn6, var_xsbstar_dn7, var_xsbstar_dn8, var_xsbstar_dn12, var_xsbstar_dn13, var_xsbstar_dn14, var_xsbstar_dn15, var_xsbstar_dn16, var_xsbstar_dn17, var_xsbstar_dn18, var_xsbstar_dn19, var_xsbstar_dn20,)
    } else {
        (var_xnct, var_xnct_dn5, var_xnct_dn6, var_xnct_dn7, var_xnct_dn8, var_xnct_dn12, var_xnct_dn13, var_xnct_dn14, var_xnct_dn15, var_xnct_dn16, var_xnct_dn17, var_xnct_dn18, var_xnct_dn19, var_xnct_dn20,)
    }
};
        var_xnct = assign41110_e54054;
        var_xnct_dn5 = assign41110_e54054_d_n5;
        var_xnct_dn6 = assign41110_e54054_d_n6;
        var_xnct_dn7 = assign41110_e54054_d_n7;
        var_xnct_dn8 = assign41110_e54054_d_n8;
        var_xnct_dn12 = assign41110_e54054_d_n12;
        var_xnct_dn13 = assign41110_e54054_d_n13;
        var_xnct_dn14 = assign41110_e54054_d_n14;
        var_xnct_dn15 = assign41110_e54054_d_n15;
        var_xnct_dn16 = assign41110_e54054_d_n16;
        var_xnct_dn17 = assign41110_e54054_d_n17;
        var_xnct_dn18 = assign41110_e54054_d_n18;
        var_xnct_dn19 = assign41110_e54054_d_n19;
        var_xnct_dn20 = assign41110_e54054_d_n20;
        var_xnct_rv = 0.0;

        let (assign41120_e54075, assign41120_e54075_d_n5, assign41120_e54075_d_n6, assign41120_e54075_d_n7, assign41120_e54075_d_n8, assign41120_e54075_d_n12, assign41120_e54075_d_n13, assign41120_e54075_d_n14, assign41120_e54075_d_n15, assign41120_e54075_d_n16, assign41120_e54075_d_n17, assign41120_e54075_d_n18, assign41120_e54075_d_n19, assign41120_e54075_d_n20,) = {
    if (var_guard1275 != 0.0) {
        let assign41120_e54058: f64 = (var_xgct - var_xnct);
        let assign41120_e54061: f64 = (var_xnct).sqrt();
        let assign41120_e54062: f64 = (var_g_0 * assign41120_e54061);
        let assign41120_e54063: f64 = (assign41120_e54058 - assign41120_e54062);
        let assign41120_e54067: f64 = (var_xbct / var_g_0);
        let assign41120_e54069: f64 = (var_xbct).sqrt();
        let assign41120_e54070: f64 = (assign41120_e54067 + assign41120_e54069);
        let assign41120_e54071: f64 = (assign41120_e54070).ln();
        let assign41120_e54072: f64 = (2.0 * assign41120_e54071);
        let assign41120_e54073: f64 = (assign41120_e54063 - assign41120_e54072);
        (assign41120_e54073, ((var_xgct_dn5 - var_xnct_dn5) - (var_g_0 * (var_xnct_dn5 / (2.0 * assign41120_e54061)))), ((var_xgct_dn6 - var_xnct_dn6) - (var_g_0 * (var_xnct_dn6 / (2.0 * assign41120_e54061)))), ((var_xgct_dn7 - var_xnct_dn7) - (var_g_0 * (var_xnct_dn7 / (2.0 * assign41120_e54061)))), ((var_xgct_dn8 - var_xnct_dn8) - (var_g_0 * (var_xnct_dn8 / (2.0 * assign41120_e54061)))), ((var_xgct_dn12 - var_xnct_dn12) - (var_g_0 * (var_xnct_dn12 / (2.0 * assign41120_e54061)))), ((var_xgct_dn13 - var_xnct_dn13) - (var_g_0 * (var_xnct_dn13 / (2.0 * assign41120_e54061)))), ((var_xgct_dn14 - var_xnct_dn14) - (var_g_0 * (var_xnct_dn14 / (2.0 * assign41120_e54061)))), ((var_xgct_dn15 - var_xnct_dn15) - (var_g_0 * (var_xnct_dn15 / (2.0 * assign41120_e54061)))), ((var_xgct_dn16 - var_xnct_dn16) - (var_g_0 * (var_xnct_dn16 / (2.0 * assign41120_e54061)))), ((var_xgct_dn17 - var_xnct_dn17) - (var_g_0 * (var_xnct_dn17 / (2.0 * assign41120_e54061)))), ((var_xgct_dn18 - var_xnct_dn18) - (var_g_0 * (var_xnct_dn18 / (2.0 * assign41120_e54061)))), ((var_xgct_dn19 - var_xnct_dn19) - (var_g_0 * (var_xnct_dn19 / (2.0 * assign41120_e54061)))), ((var_xgct_dn20 - var_xnct_dn20) - (var_g_0 * (var_xnct_dn20 / (2.0 * assign41120_e54061)))),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn12, var_temp1_dn13, var_temp1_dn14, var_temp1_dn15, var_temp1_dn16, var_temp1_dn17, var_temp1_dn18, var_temp1_dn19, var_temp1_dn20,)
    }
};
        var_temp1 = assign41120_e54075;
        var_temp1_dn5 = assign41120_e54075_d_n5;
        var_temp1_dn6 = assign41120_e54075_d_n6;
        var_temp1_dn7 = assign41120_e54075_d_n7;
        var_temp1_dn8 = assign41120_e54075_d_n8;
        var_temp1_dn12 = assign41120_e54075_d_n12;
        var_temp1_dn13 = assign41120_e54075_d_n13;
        var_temp1_dn14 = assign41120_e54075_d_n14;
        var_temp1_dn15 = assign41120_e54075_d_n15;
        var_temp1_dn16 = assign41120_e54075_d_n16;
        var_temp1_dn17 = assign41120_e54075_d_n17;
        var_temp1_dn18 = assign41120_e54075_d_n18;
        var_temp1_dn19 = assign41120_e54075_d_n19;
        var_temp1_dn20 = assign41120_e54075_d_n20;
        var_temp1_rv = 0.0;

        let (assign41130_e54083, assign41130_e54083_d_n5, assign41130_e54083_d_n6, assign41130_e54083_d_n7, assign41130_e54083_d_n8, assign41130_e54083_d_n12, assign41130_e54083_d_n13, assign41130_e54083_d_n14, assign41130_e54083_d_n15, assign41130_e54083_d_n16, assign41130_e54083_d_n17, assign41130_e54083_d_n18, assign41130_e54083_d_n19, assign41130_e54083_d_n20,) = {
    if (var_guard1275 != 0.0) {
        let assign41130_e54079: f64 = (2.0 * var_temp1);
        let assign41130_e54081: f64 = (assign41130_e54079 + var_xctmax);
        (assign41130_e54081, (2.0 * var_temp1_dn5), (2.0 * var_temp1_dn6), (2.0 * var_temp1_dn7), (2.0 * var_temp1_dn8), (2.0 * var_temp1_dn12), (2.0 * var_temp1_dn13), (2.0 * var_temp1_dn14), (2.0 * var_temp1_dn15), (2.0 * var_temp1_dn16), (2.0 * var_temp1_dn17), (2.0 * var_temp1_dn18), (2.0 * var_temp1_dn19), (2.0 * var_temp1_dn20),)
    } else {
        (var_xmict, var_xmict_dn5, var_xmict_dn6, var_xmict_dn7, var_xmict_dn8, var_xmict_dn12, var_xmict_dn13, var_xmict_dn14, var_xmict_dn15, var_xmict_dn16, var_xmict_dn17, var_xmict_dn18, var_xmict_dn19, var_xmict_dn20,)
    }
};
        var_xmict = assign41130_e54083;
        var_xmict_dn5 = assign41130_e54083_d_n5;
        var_xmict_dn6 = assign41130_e54083_d_n6;
        var_xmict_dn7 = assign41130_e54083_d_n7;
        var_xmict_dn8 = assign41130_e54083_d_n8;
        var_xmict_dn12 = assign41130_e54083_d_n12;
        var_xmict_dn13 = assign41130_e54083_d_n13;
        var_xmict_dn14 = assign41130_e54083_d_n14;
        var_xmict_dn15 = assign41130_e54083_d_n15;
        var_xmict_dn16 = assign41130_e54083_d_n16;
        var_xmict_dn17 = assign41130_e54083_d_n17;
        var_xmict_dn18 = assign41130_e54083_d_n18;
        var_xmict_dn19 = assign41130_e54083_d_n19;
        var_xmict_dn20 = assign41130_e54083_d_n20;
        var_xmict_rv = 0.0;

        let (assign41140_e54102, assign41140_e54102_d_n5, assign41140_e54102_d_n6, assign41140_e54102_d_n7, assign41140_e54102_d_n8, assign41140_e54102_d_n12, assign41140_e54102_d_n13, assign41140_e54102_d_n14, assign41140_e54102_d_n15, assign41140_e54102_d_n16, assign41140_e54102_d_n17, assign41140_e54102_d_n18, assign41140_e54102_d_n19, assign41140_e54102_d_n20,) = {
    if (var_guard1275 != 0.0) {
        let assign41140_e54088: f64 = (var_xwict + var_xmict);
        let assign41140_e54091: f64 = (var_xwict - var_xmict);
        let assign41140_e54094: f64 = (var_xwict - var_xmict);
        let assign41140_e54095: f64 = (assign41140_e54091 * assign41140_e54094);
        let assign41140_e54097: f64 = (assign41140_e54095 + 20.0);
        let assign41140_e54098: f64 = (assign41140_e54097).sqrt();
        let assign41140_e54099: f64 = (assign41140_e54088 + assign41140_e54098);
        let assign41140_e54100: f64 = (0.5 * assign41140_e54099);
        (assign41140_e54100, (0.5 * ((var_xwict_dn5 + var_xmict_dn5) + ((((var_xwict_dn5 - var_xmict_dn5) * assign41140_e54094) + (assign41140_e54091 * (var_xwict_dn5 - var_xmict_dn5))) / (2.0 * assign41140_e54098)))), (0.5 * ((var_xwict_dn6 + var_xmict_dn6) + ((((var_xwict_dn6 - var_xmict_dn6) * assign41140_e54094) + (assign41140_e54091 * (var_xwict_dn6 - var_xmict_dn6))) / (2.0 * assign41140_e54098)))), (0.5 * ((var_xwict_dn7 + var_xmict_dn7) + ((((var_xwict_dn7 - var_xmict_dn7) * assign41140_e54094) + (assign41140_e54091 * (var_xwict_dn7 - var_xmict_dn7))) / (2.0 * assign41140_e54098)))), (0.5 * ((var_xwict_dn8 + var_xmict_dn8) + ((((var_xwict_dn8 - var_xmict_dn8) * assign41140_e54094) + (assign41140_e54091 * (var_xwict_dn8 - var_xmict_dn8))) / (2.0 * assign41140_e54098)))), (0.5 * ((var_xwict_dn12 + var_xmict_dn12) + ((((var_xwict_dn12 - var_xmict_dn12) * assign41140_e54094) + (assign41140_e54091 * (var_xwict_dn12 - var_xmict_dn12))) / (2.0 * assign41140_e54098)))), (0.5 * ((var_xwict_dn13 + var_xmict_dn13) + ((((var_xwict_dn13 - var_xmict_dn13) * assign41140_e54094) + (assign41140_e54091 * (var_xwict_dn13 - var_xmict_dn13))) / (2.0 * assign41140_e54098)))), (0.5 * ((var_xwict_dn14 + var_xmict_dn14) + ((((var_xwict_dn14 - var_xmict_dn14) * assign41140_e54094) + (assign41140_e54091 * (var_xwict_dn14 - var_xmict_dn14))) / (2.0 * assign41140_e54098)))), (0.5 * ((var_xwict_dn15 + var_xmict_dn15) + ((((var_xwict_dn15 - var_xmict_dn15) * assign41140_e54094) + (assign41140_e54091 * (var_xwict_dn15 - var_xmict_dn15))) / (2.0 * assign41140_e54098)))), (0.5 * ((var_xwict_dn16 + var_xmict_dn16) + ((((var_xwict_dn16 - var_xmict_dn16) * assign41140_e54094) + (assign41140_e54091 * (var_xwict_dn16 - var_xmict_dn16))) / (2.0 * assign41140_e54098)))), (0.5 * ((var_xwict_dn17 + var_xmict_dn17) + ((((var_xwict_dn17 - var_xmict_dn17) * assign41140_e54094) + (assign41140_e54091 * (var_xwict_dn17 - var_xmict_dn17))) / (2.0 * assign41140_e54098)))), (0.5 * ((var_xwict_dn18 + var_xmict_dn18) + ((((var_xwict_dn18 - var_xmict_dn18) * assign41140_e54094) + (assign41140_e54091 * (var_xwict_dn18 - var_xmict_dn18))) / (2.0 * assign41140_e54098)))), (0.5 * ((var_xwict_dn19 + var_xmict_dn19) + ((((var_xwict_dn19 - var_xmict_dn19) * assign41140_e54094) + (assign41140_e54091 * (var_xwict_dn19 - var_xmict_dn19))) / (2.0 * assign41140_e54098)))), (0.5 * ((var_xwict_dn20 + var_xmict_dn20) + ((((var_xwict_dn20 - var_xmict_dn20) * assign41140_e54094) + (assign41140_e54091 * (var_xwict_dn20 - var_xmict_dn20))) / (2.0 * assign41140_e54098)))),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn12, var_temp1_dn13, var_temp1_dn14, var_temp1_dn15, var_temp1_dn16, var_temp1_dn17, var_temp1_dn18, var_temp1_dn19, var_temp1_dn20,)
    }
};
        var_temp1 = assign41140_e54102;
        var_temp1_dn5 = assign41140_e54102_d_n5;
        var_temp1_dn6 = assign41140_e54102_d_n6;
        var_temp1_dn7 = assign41140_e54102_d_n7;
        var_temp1_dn8 = assign41140_e54102_d_n8;
        var_temp1_dn12 = assign41140_e54102_d_n12;
        var_temp1_dn13 = assign41140_e54102_d_n13;
        var_temp1_dn14 = assign41140_e54102_d_n14;
        var_temp1_dn15 = assign41140_e54102_d_n15;
        var_temp1_dn16 = assign41140_e54102_d_n16;
        var_temp1_dn17 = assign41140_e54102_d_n17;
        var_temp1_dn18 = assign41140_e54102_d_n18;
        var_temp1_dn19 = assign41140_e54102_d_n19;
        var_temp1_dn20 = assign41140_e54102_d_n20;
        var_temp1_rv = 0.0;

        let (assign41150_e54112, assign41150_e54112_d_n5, assign41150_e54112_d_n6, assign41150_e54112_d_n7, assign41150_e54112_d_n8, assign41150_e54112_d_n12, assign41150_e54112_d_n13, assign41150_e54112_d_n14, assign41150_e54112_d_n15, assign41150_e54112_d_n16, assign41150_e54112_d_n17, assign41150_e54112_d_n18, assign41150_e54112_d_n19, assign41150_e54112_d_n20,) = {
    if (var_guard1275 != 0.0) {
        let assign41150_e54107: f64 = (var_xgct - var_xsbstar);
        let assign41150_e54108: f64 = (2.0 * assign41150_e54107);
        let assign41150_e54110: f64 = (assign41150_e54108 - var_xctmax);
        (assign41150_e54110, (2.0 * (var_xgct_dn5 - var_xsbstar_dn5)), (2.0 * (var_xgct_dn6 - var_xsbstar_dn6)), (2.0 * (var_xgct_dn7 - var_xsbstar_dn7)), (2.0 * (var_xgct_dn8 - var_xsbstar_dn8)), (2.0 * (var_xgct_dn12 - var_xsbstar_dn12)), (2.0 * (var_xgct_dn13 - var_xsbstar_dn13)), (2.0 * (var_xgct_dn14 - var_xsbstar_dn14)), (2.0 * (var_xgct_dn15 - var_xsbstar_dn15)), (2.0 * (var_xgct_dn16 - var_xsbstar_dn16)), (2.0 * (var_xgct_dn17 - var_xsbstar_dn17)), (2.0 * (var_xgct_dn18 - var_xsbstar_dn18)), (2.0 * (var_xgct_dn19 - var_xsbstar_dn19)), (2.0 * (var_xgct_dn20 - var_xsbstar_dn20)),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn12, var_temp2_dn13, var_temp2_dn14, var_temp2_dn15, var_temp2_dn16, var_temp2_dn17, var_temp2_dn18, var_temp2_dn19, var_temp2_dn20,)
    }
};
        var_temp2 = assign41150_e54112;
        var_temp2_dn5 = assign41150_e54112_d_n5;
        var_temp2_dn6 = assign41150_e54112_d_n6;
        var_temp2_dn7 = assign41150_e54112_d_n7;
        var_temp2_dn8 = assign41150_e54112_d_n8;
        var_temp2_dn12 = assign41150_e54112_d_n12;
        var_temp2_dn13 = assign41150_e54112_d_n13;
        var_temp2_dn14 = assign41150_e54112_d_n14;
        var_temp2_dn15 = assign41150_e54112_d_n15;
        var_temp2_dn16 = assign41150_e54112_d_n16;
        var_temp2_dn17 = assign41150_e54112_d_n17;
        var_temp2_dn18 = assign41150_e54112_d_n18;
        var_temp2_dn19 = assign41150_e54112_d_n19;
        var_temp2_dn20 = assign41150_e54112_d_n20;
        var_temp2_rv = 0.0;

        let (assign41160_e54131, assign41160_e54131_d_n5, assign41160_e54131_d_n6, assign41160_e54131_d_n7, assign41160_e54131_d_n8, assign41160_e54131_d_n12, assign41160_e54131_d_n13, assign41160_e54131_d_n14, assign41160_e54131_d_n15, assign41160_e54131_d_n16, assign41160_e54131_d_n17, assign41160_e54131_d_n18, assign41160_e54131_d_n19, assign41160_e54131_d_n20,) = {
    if (var_guard1275 != 0.0) {
        let assign41160_e54117: f64 = (var_temp1 + var_temp2);
        let assign41160_e54120: f64 = (var_temp1 - var_temp2);
        let assign41160_e54123: f64 = (var_temp1 - var_temp2);
        let assign41160_e54124: f64 = (assign41160_e54120 * assign41160_e54123);
        let assign41160_e54126: f64 = (assign41160_e54124 + 20.0);
        let assign41160_e54127: f64 = (assign41160_e54126).sqrt();
        let assign41160_e54128: f64 = (assign41160_e54117 - assign41160_e54127);
        let assign41160_e54129: f64 = (0.5 * assign41160_e54128);
        (assign41160_e54129, (0.5 * ((var_temp1_dn5 + var_temp2_dn5) - ((((var_temp1_dn5 - var_temp2_dn5) * assign41160_e54123) + (assign41160_e54120 * (var_temp1_dn5 - var_temp2_dn5))) / (2.0 * assign41160_e54127)))), (0.5 * ((var_temp1_dn6 + var_temp2_dn6) - ((((var_temp1_dn6 - var_temp2_dn6) * assign41160_e54123) + (assign41160_e54120 * (var_temp1_dn6 - var_temp2_dn6))) / (2.0 * assign41160_e54127)))), (0.5 * ((var_temp1_dn7 + var_temp2_dn7) - ((((var_temp1_dn7 - var_temp2_dn7) * assign41160_e54123) + (assign41160_e54120 * (var_temp1_dn7 - var_temp2_dn7))) / (2.0 * assign41160_e54127)))), (0.5 * ((var_temp1_dn8 + var_temp2_dn8) - ((((var_temp1_dn8 - var_temp2_dn8) * assign41160_e54123) + (assign41160_e54120 * (var_temp1_dn8 - var_temp2_dn8))) / (2.0 * assign41160_e54127)))), (0.5 * ((var_temp1_dn12 + var_temp2_dn12) - ((((var_temp1_dn12 - var_temp2_dn12) * assign41160_e54123) + (assign41160_e54120 * (var_temp1_dn12 - var_temp2_dn12))) / (2.0 * assign41160_e54127)))), (0.5 * ((var_temp1_dn13 + var_temp2_dn13) - ((((var_temp1_dn13 - var_temp2_dn13) * assign41160_e54123) + (assign41160_e54120 * (var_temp1_dn13 - var_temp2_dn13))) / (2.0 * assign41160_e54127)))), (0.5 * ((var_temp1_dn14 + var_temp2_dn14) - ((((var_temp1_dn14 - var_temp2_dn14) * assign41160_e54123) + (assign41160_e54120 * (var_temp1_dn14 - var_temp2_dn14))) / (2.0 * assign41160_e54127)))), (0.5 * ((var_temp1_dn15 + var_temp2_dn15) - ((((var_temp1_dn15 - var_temp2_dn15) * assign41160_e54123) + (assign41160_e54120 * (var_temp1_dn15 - var_temp2_dn15))) / (2.0 * assign41160_e54127)))), (0.5 * ((var_temp1_dn16 + var_temp2_dn16) - ((((var_temp1_dn16 - var_temp2_dn16) * assign41160_e54123) + (assign41160_e54120 * (var_temp1_dn16 - var_temp2_dn16))) / (2.0 * assign41160_e54127)))), (0.5 * ((var_temp1_dn17 + var_temp2_dn17) - ((((var_temp1_dn17 - var_temp2_dn17) * assign41160_e54123) + (assign41160_e54120 * (var_temp1_dn17 - var_temp2_dn17))) / (2.0 * assign41160_e54127)))), (0.5 * ((var_temp1_dn18 + var_temp2_dn18) - ((((var_temp1_dn18 - var_temp2_dn18) * assign41160_e54123) + (assign41160_e54120 * (var_temp1_dn18 - var_temp2_dn18))) / (2.0 * assign41160_e54127)))), (0.5 * ((var_temp1_dn19 + var_temp2_dn19) - ((((var_temp1_dn19 - var_temp2_dn19) * assign41160_e54123) + (assign41160_e54120 * (var_temp1_dn19 - var_temp2_dn19))) / (2.0 * assign41160_e54127)))), (0.5 * ((var_temp1_dn20 + var_temp2_dn20) - ((((var_temp1_dn20 - var_temp2_dn20) * assign41160_e54123) + (assign41160_e54120 * (var_temp1_dn20 - var_temp2_dn20))) / (2.0 * assign41160_e54127)))),)
    } else {
        (var_xsubct, var_xsubct_dn5, var_xsubct_dn6, var_xsubct_dn7, var_xsubct_dn8, var_xsubct_dn12, var_xsubct_dn13, var_xsubct_dn14, var_xsubct_dn15, var_xsubct_dn16, var_xsubct_dn17, var_xsubct_dn18, var_xsubct_dn19, var_xsubct_dn20,)
    }
};
        var_xsubct = assign41160_e54131;
        var_xsubct_dn5 = assign41160_e54131_d_n5;
        var_xsubct_dn6 = assign41160_e54131_d_n6;
        var_xsubct_dn7 = assign41160_e54131_d_n7;
        var_xsubct_dn8 = assign41160_e54131_d_n8;
        var_xsubct_dn12 = assign41160_e54131_d_n12;
        var_xsubct_dn13 = assign41160_e54131_d_n13;
        var_xsubct_dn14 = assign41160_e54131_d_n14;
        var_xsubct_dn15 = assign41160_e54131_d_n15;
        var_xsubct_dn16 = assign41160_e54131_d_n16;
        var_xsubct_dn17 = assign41160_e54131_d_n17;
        var_xsubct_dn18 = assign41160_e54131_d_n18;
        var_xsubct_dn19 = assign41160_e54131_d_n19;
        var_xsubct_dn20 = assign41160_e54131_d_n20;
        var_xsubct_rv = 0.0;

        let (assign41170_e54150, assign41170_e54150_d_n5, assign41170_e54150_d_n6, assign41170_e54150_d_n7, assign41170_e54150_d_n8, assign41170_e54150_d_n12, assign41170_e54150_d_n13, assign41170_e54150_d_n14, assign41170_e54150_d_n15, assign41170_e54150_d_n16, assign41170_e54150_d_n17, assign41170_e54150_d_n18, assign41170_e54150_d_n19, assign41170_e54150_d_n20,) = {
    if (var_guard1275 != 0.0) {
        let assign41170_e54136: f64 = (var_xsubct + var_xctmax);
        let assign41170_e54139: f64 = (var_xsubct - var_xctmax);
        let assign41170_e54142: f64 = (var_xsubct - var_xctmax);
        let assign41170_e54143: f64 = (assign41170_e54139 * assign41170_e54142);
        let assign41170_e54145: f64 = (assign41170_e54143 + 5.0);
        let assign41170_e54146: f64 = (assign41170_e54145).sqrt();
        let assign41170_e54147: f64 = (assign41170_e54136 - assign41170_e54146);
        let assign41170_e54148: f64 = (0.5 * assign41170_e54147);
        (assign41170_e54148, (0.5 * (var_xsubct_dn5 - (((var_xsubct_dn5 * assign41170_e54142) + (assign41170_e54139 * var_xsubct_dn5)) / (2.0 * assign41170_e54146)))), (0.5 * (var_xsubct_dn6 - (((var_xsubct_dn6 * assign41170_e54142) + (assign41170_e54139 * var_xsubct_dn6)) / (2.0 * assign41170_e54146)))), (0.5 * (var_xsubct_dn7 - (((var_xsubct_dn7 * assign41170_e54142) + (assign41170_e54139 * var_xsubct_dn7)) / (2.0 * assign41170_e54146)))), (0.5 * (var_xsubct_dn8 - (((var_xsubct_dn8 * assign41170_e54142) + (assign41170_e54139 * var_xsubct_dn8)) / (2.0 * assign41170_e54146)))), (0.5 * (var_xsubct_dn12 - (((var_xsubct_dn12 * assign41170_e54142) + (assign41170_e54139 * var_xsubct_dn12)) / (2.0 * assign41170_e54146)))), (0.5 * (var_xsubct_dn13 - (((var_xsubct_dn13 * assign41170_e54142) + (assign41170_e54139 * var_xsubct_dn13)) / (2.0 * assign41170_e54146)))), (0.5 * (var_xsubct_dn14 - (((var_xsubct_dn14 * assign41170_e54142) + (assign41170_e54139 * var_xsubct_dn14)) / (2.0 * assign41170_e54146)))), (0.5 * (var_xsubct_dn15 - (((var_xsubct_dn15 * assign41170_e54142) + (assign41170_e54139 * var_xsubct_dn15)) / (2.0 * assign41170_e54146)))), (0.5 * (var_xsubct_dn16 - (((var_xsubct_dn16 * assign41170_e54142) + (assign41170_e54139 * var_xsubct_dn16)) / (2.0 * assign41170_e54146)))), (0.5 * (var_xsubct_dn17 - (((var_xsubct_dn17 * assign41170_e54142) + (assign41170_e54139 * var_xsubct_dn17)) / (2.0 * assign41170_e54146)))), (0.5 * (var_xsubct_dn18 - (((var_xsubct_dn18 * assign41170_e54142) + (assign41170_e54139 * var_xsubct_dn18)) / (2.0 * assign41170_e54146)))), (0.5 * (var_xsubct_dn19 - (((var_xsubct_dn19 * assign41170_e54142) + (assign41170_e54139 * var_xsubct_dn19)) / (2.0 * assign41170_e54146)))), (0.5 * (var_xsubct_dn20 - (((var_xsubct_dn20 * assign41170_e54142) + (assign41170_e54139 * var_xsubct_dn20)) / (2.0 * assign41170_e54146)))),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn12, var_temp1_dn13, var_temp1_dn14, var_temp1_dn15, var_temp1_dn16, var_temp1_dn17, var_temp1_dn18, var_temp1_dn19, var_temp1_dn20,)
    }
};
        var_temp1 = assign41170_e54150;
        var_temp1_dn5 = assign41170_e54150_d_n5;
        var_temp1_dn6 = assign41170_e54150_d_n6;
        var_temp1_dn7 = assign41170_e54150_d_n7;
        var_temp1_dn8 = assign41170_e54150_d_n8;
        var_temp1_dn12 = assign41170_e54150_d_n12;
        var_temp1_dn13 = assign41170_e54150_d_n13;
        var_temp1_dn14 = assign41170_e54150_d_n14;
        var_temp1_dn15 = assign41170_e54150_d_n15;
        var_temp1_dn16 = assign41170_e54150_d_n16;
        var_temp1_dn17 = assign41170_e54150_d_n17;
        var_temp1_dn18 = assign41170_e54150_d_n18;
        var_temp1_dn19 = assign41170_e54150_d_n19;
        var_temp1_dn20 = assign41170_e54150_d_n20;
        var_temp1_rv = 0.0;

        *var_aphi_slot = var_aphi;
        *var_aphi_rv_slot = var_aphi_rv;
        *var_arloc_slot = var_arloc;
        *var_arloc_rv_slot = var_arloc_rv;
        *var_dctg_slot = var_dctg;
        *var_dctg_dn12_slot = var_dctg_dn12;
        *var_dctg_dn13_slot = var_dctg_dn13;
        *var_dctg_dn14_slot = var_dctg_dn14;
        *var_dctg_dn15_slot = var_dctg_dn15;
        *var_dctg_dn16_slot = var_dctg_dn16;
        *var_dctg_dn17_slot = var_dctg_dn17;
        *var_dctg_dn18_slot = var_dctg_dn18;
        *var_dctg_dn19_slot = var_dctg_dn19;
        *var_dctg_dn20_slot = var_dctg_dn20;
        *var_dctg_dn5_slot = var_dctg_dn5;
        *var_dctg_dn6_slot = var_dctg_dn6;
        *var_dctg_dn7_slot = var_dctg_dn7;
        *var_dctg_dn8_slot = var_dctg_dn8;
        *var_dctg_rv_slot = var_dctg_rv;
        *var_dvbstar_slot = var_dvbstar;
        *var_dvbstar_dc_slot = var_dvbstar_dc;
        *var_dvbstar_dc_dn12_slot = var_dvbstar_dc_dn12;
        *var_dvbstar_dc_dn13_slot = var_dvbstar_dc_dn13;
        *var_dvbstar_dc_dn14_slot = var_dvbstar_dc_dn14;
        *var_dvbstar_dc_dn15_slot = var_dvbstar_dc_dn15;
        *var_dvbstar_dc_dn16_slot = var_dvbstar_dc_dn16;
        *var_dvbstar_dc_dn17_slot = var_dvbstar_dc_dn17;
        *var_dvbstar_dc_dn18_slot = var_dvbstar_dc_dn18;
        *var_dvbstar_dc_dn19_slot = var_dvbstar_dc_dn19;
        *var_dvbstar_dc_dn20_slot = var_dvbstar_dc_dn20;
        *var_dvbstar_dc_dn5_slot = var_dvbstar_dc_dn5;
        *var_dvbstar_dc_dn6_slot = var_dvbstar_dc_dn6;
        *var_dvbstar_dc_dn7_slot = var_dvbstar_dc_dn7;
        *var_dvbstar_dc_dn8_slot = var_dvbstar_dc_dn8;
        *var_dvbstar_dc_rv_slot = var_dvbstar_dc_rv;
        *var_dvbstar_dn12_slot = var_dvbstar_dn12;
        *var_dvbstar_dn13_slot = var_dvbstar_dn13;
        *var_dvbstar_dn14_slot = var_dvbstar_dn14;
        *var_dvbstar_dn15_slot = var_dvbstar_dn15;
        *var_dvbstar_dn16_slot = var_dvbstar_dn16;
        *var_dvbstar_dn17_slot = var_dvbstar_dn17;
        *var_dvbstar_dn18_slot = var_dvbstar_dn18;
        *var_dvbstar_dn19_slot = var_dvbstar_dn19;
        *var_dvbstar_dn20_slot = var_dvbstar_dn20;
        *var_dvbstar_dn5_slot = var_dvbstar_dn5;
        *var_dvbstar_dn6_slot = var_dvbstar_dn6;
        *var_dvbstar_dn7_slot = var_dvbstar_dn7;
        *var_dvbstar_dn8_slot = var_dvbstar_dn8;
        *var_dvbstar_rv_slot = var_dvbstar_rv;
        *var_g_0_slot = var_g_0;
        *var_g_0_rv_slot = var_g_0_rv;
        *var_guard1275_slot = var_guard1275;
        *var_guard1275_rv_slot = var_guard1275_rv;
        *var_phib_slot = var_phib;
        *var_phib_rv_slot = var_phib_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn12_slot = var_temp1_dn12;
        *var_temp1_dn13_slot = var_temp1_dn13;
        *var_temp1_dn14_slot = var_temp1_dn14;
        *var_temp1_dn15_slot = var_temp1_dn15;
        *var_temp1_dn16_slot = var_temp1_dn16;
        *var_temp1_dn17_slot = var_temp1_dn17;
        *var_temp1_dn18_slot = var_temp1_dn18;
        *var_temp1_dn19_slot = var_temp1_dn19;
        *var_temp1_dn20_slot = var_temp1_dn20;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn12_slot = var_temp2_dn12;
        *var_temp2_dn13_slot = var_temp2_dn13;
        *var_temp2_dn14_slot = var_temp2_dn14;
        *var_temp2_dn15_slot = var_temp2_dn15;
        *var_temp2_dn16_slot = var_temp2_dn16;
        *var_temp2_dn17_slot = var_temp2_dn17;
        *var_temp2_dn18_slot = var_temp2_dn18;
        *var_temp2_dn19_slot = var_temp2_dn19;
        *var_temp2_dn20_slot = var_temp2_dn20;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_thesatloc_slot = var_thesatloc;
        *var_thesatloc_rv_slot = var_thesatloc_rv;
        *var_vgb1_slot = var_vgb1;
        *var_vgb1_dn12_slot = var_vgb1_dn12;
        *var_vgb1_dn13_slot = var_vgb1_dn13;
        *var_vgb1_dn14_slot = var_vgb1_dn14;
        *var_vgb1_dn15_slot = var_vgb1_dn15;
        *var_vgb1_dn16_slot = var_vgb1_dn16;
        *var_vgb1_dn17_slot = var_vgb1_dn17;
        *var_vgb1_dn18_slot = var_vgb1_dn18;
        *var_vgb1_dn19_slot = var_vgb1_dn19;
        *var_vgb1_dn20_slot = var_vgb1_dn20;
        *var_vgb1_dn5_slot = var_vgb1_dn5;
        *var_vgb1_dn6_slot = var_vgb1_dn6;
        *var_vgb1_dn7_slot = var_vgb1_dn7;
        *var_vgb1_dn8_slot = var_vgb1_dn8;
        *var_vgb1_rv_slot = var_vgb1_rv;
        *var_vsbstar_slot = var_vsbstar;
        *var_vsbstar_dc_slot = var_vsbstar_dc;
        *var_vsbstar_dc_dn12_slot = var_vsbstar_dc_dn12;
        *var_vsbstar_dc_dn13_slot = var_vsbstar_dc_dn13;
        *var_vsbstar_dc_dn14_slot = var_vsbstar_dc_dn14;
        *var_vsbstar_dc_dn15_slot = var_vsbstar_dc_dn15;
        *var_vsbstar_dc_dn16_slot = var_vsbstar_dc_dn16;
        *var_vsbstar_dc_dn17_slot = var_vsbstar_dc_dn17;
        *var_vsbstar_dc_dn18_slot = var_vsbstar_dc_dn18;
        *var_vsbstar_dc_dn19_slot = var_vsbstar_dc_dn19;
        *var_vsbstar_dc_dn20_slot = var_vsbstar_dc_dn20;
        *var_vsbstar_dc_dn5_slot = var_vsbstar_dc_dn5;
        *var_vsbstar_dc_dn6_slot = var_vsbstar_dc_dn6;
        *var_vsbstar_dc_dn7_slot = var_vsbstar_dc_dn7;
        *var_vsbstar_dc_dn8_slot = var_vsbstar_dc_dn8;
        *var_vsbstar_dc_rv_slot = var_vsbstar_dc_rv;
        *var_vsbstar_dn12_slot = var_vsbstar_dn12;
        *var_vsbstar_dn13_slot = var_vsbstar_dn13;
        *var_vsbstar_dn14_slot = var_vsbstar_dn14;
        *var_vsbstar_dn15_slot = var_vsbstar_dn15;
        *var_vsbstar_dn16_slot = var_vsbstar_dn16;
        *var_vsbstar_dn17_slot = var_vsbstar_dn17;
        *var_vsbstar_dn18_slot = var_vsbstar_dn18;
        *var_vsbstar_dn19_slot = var_vsbstar_dn19;
        *var_vsbstar_dn20_slot = var_vsbstar_dn20;
        *var_vsbstar_dn5_slot = var_vsbstar_dn5;
        *var_vsbstar_dn6_slot = var_vsbstar_dn6;
        *var_vsbstar_dn7_slot = var_vsbstar_dn7;
        *var_vsbstar_dn8_slot = var_vsbstar_dn8;
        *var_vsbstar_rv_slot = var_vsbstar_rv;
        *var_vsbx_slot = var_vsbx;
        *var_vsbx_dn12_slot = var_vsbx_dn12;
        *var_vsbx_dn13_slot = var_vsbx_dn13;
        *var_vsbx_dn14_slot = var_vsbx_dn14;
        *var_vsbx_dn15_slot = var_vsbx_dn15;
        *var_vsbx_dn16_slot = var_vsbx_dn16;
        *var_vsbx_dn17_slot = var_vsbx_dn17;
        *var_vsbx_dn18_slot = var_vsbx_dn18;
        *var_vsbx_dn19_slot = var_vsbx_dn19;
        *var_vsbx_dn20_slot = var_vsbx_dn20;
        *var_vsbx_dn5_slot = var_vsbx_dn5;
        *var_vsbx_dn6_slot = var_vsbx_dn6;
        *var_vsbx_dn7_slot = var_vsbx_dn7;
        *var_vsbx_dn8_slot = var_vsbx_dn8;
        *var_vsbx_rv_slot = var_vsbx_rv;
        *var_xbct_slot = var_xbct;
        *var_xbct_rv_slot = var_xbct_rv;
        *var_xctmax_slot = var_xctmax;
        *var_xctmax_rv_slot = var_xctmax_rv;
        *var_xgct_slot = var_xgct;
        *var_xgct_dn12_slot = var_xgct_dn12;
        *var_xgct_dn13_slot = var_xgct_dn13;
        *var_xgct_dn14_slot = var_xgct_dn14;
        *var_xgct_dn15_slot = var_xgct_dn15;
        *var_xgct_dn16_slot = var_xgct_dn16;
        *var_xgct_dn17_slot = var_xgct_dn17;
        *var_xgct_dn18_slot = var_xgct_dn18;
        *var_xgct_dn19_slot = var_xgct_dn19;
        *var_xgct_dn20_slot = var_xgct_dn20;
        *var_xgct_dn5_slot = var_xgct_dn5;
        *var_xgct_dn6_slot = var_xgct_dn6;
        *var_xgct_dn7_slot = var_xgct_dn7;
        *var_xgct_dn8_slot = var_xgct_dn8;
        *var_xgct_rv_slot = var_xgct_rv;
        *var_xmict_slot = var_xmict;
        *var_xmict_dn12_slot = var_xmict_dn12;
        *var_xmict_dn13_slot = var_xmict_dn13;
        *var_xmict_dn14_slot = var_xmict_dn14;
        *var_xmict_dn15_slot = var_xmict_dn15;
        *var_xmict_dn16_slot = var_xmict_dn16;
        *var_xmict_dn17_slot = var_xmict_dn17;
        *var_xmict_dn18_slot = var_xmict_dn18;
        *var_xmict_dn19_slot = var_xmict_dn19;
        *var_xmict_dn20_slot = var_xmict_dn20;
        *var_xmict_dn5_slot = var_xmict_dn5;
        *var_xmict_dn6_slot = var_xmict_dn6;
        *var_xmict_dn7_slot = var_xmict_dn7;
        *var_xmict_dn8_slot = var_xmict_dn8;
        *var_xmict_rv_slot = var_xmict_rv;
        *var_xnct_slot = var_xnct;
        *var_xnct_dn12_slot = var_xnct_dn12;
        *var_xnct_dn13_slot = var_xnct_dn13;
        *var_xnct_dn14_slot = var_xnct_dn14;
        *var_xnct_dn15_slot = var_xnct_dn15;
        *var_xnct_dn16_slot = var_xnct_dn16;
        *var_xnct_dn17_slot = var_xnct_dn17;
        *var_xnct_dn18_slot = var_xnct_dn18;
        *var_xnct_dn19_slot = var_xnct_dn19;
        *var_xnct_dn20_slot = var_xnct_dn20;
        *var_xnct_dn5_slot = var_xnct_dn5;
        *var_xnct_dn6_slot = var_xnct_dn6;
        *var_xnct_dn7_slot = var_xnct_dn7;
        *var_xnct_dn8_slot = var_xnct_dn8;
        *var_xnct_rv_slot = var_xnct_rv;
        *var_xsbstar_slot = var_xsbstar;
        *var_xsbstar_dn12_slot = var_xsbstar_dn12;
        *var_xsbstar_dn13_slot = var_xsbstar_dn13;
        *var_xsbstar_dn14_slot = var_xsbstar_dn14;
        *var_xsbstar_dn15_slot = var_xsbstar_dn15;
        *var_xsbstar_dn16_slot = var_xsbstar_dn16;
        *var_xsbstar_dn17_slot = var_xsbstar_dn17;
        *var_xsbstar_dn18_slot = var_xsbstar_dn18;
        *var_xsbstar_dn19_slot = var_xsbstar_dn19;
        *var_xsbstar_dn20_slot = var_xsbstar_dn20;
        *var_xsbstar_dn5_slot = var_xsbstar_dn5;
        *var_xsbstar_dn6_slot = var_xsbstar_dn6;
        *var_xsbstar_dn7_slot = var_xsbstar_dn7;
        *var_xsbstar_dn8_slot = var_xsbstar_dn8;
        *var_xsbstar_rv_slot = var_xsbstar_rv;
        *var_xsubct_slot = var_xsubct;
        *var_xsubct_dn12_slot = var_xsubct_dn12;
        *var_xsubct_dn13_slot = var_xsubct_dn13;
        *var_xsubct_dn14_slot = var_xsubct_dn14;
        *var_xsubct_dn15_slot = var_xsubct_dn15;
        *var_xsubct_dn16_slot = var_xsubct_dn16;
        *var_xsubct_dn17_slot = var_xsubct_dn17;
        *var_xsubct_dn18_slot = var_xsubct_dn18;
        *var_xsubct_dn19_slot = var_xsubct_dn19;
        *var_xsubct_dn20_slot = var_xsubct_dn20;
        *var_xsubct_dn5_slot = var_xsubct_dn5;
        *var_xsubct_dn6_slot = var_xsubct_dn6;
        *var_xsubct_dn7_slot = var_xsubct_dn7;
        *var_xsubct_dn8_slot = var_xsubct_dn8;
        *var_xsubct_rv_slot = var_xsubct_rv;
        *var_xwict_slot = var_xwict;
        *var_xwict_dn12_slot = var_xwict_dn12;
        *var_xwict_dn13_slot = var_xwict_dn13;
        *var_xwict_dn14_slot = var_xwict_dn14;
        *var_xwict_dn15_slot = var_xwict_dn15;
        *var_xwict_dn16_slot = var_xwict_dn16;
        *var_xwict_dn17_slot = var_xwict_dn17;
        *var_xwict_dn18_slot = var_xwict_dn18;
        *var_xwict_dn19_slot = var_xwict_dn19;
        *var_xwict_dn20_slot = var_xwict_dn20;
        *var_xwict_dn5_slot = var_xwict_dn5;
        *var_xwict_dn6_slot = var_xwict_dn6;
        *var_xwict_dn7_slot = var_xwict_dn7;
        *var_xwict_dn8_slot = var_xwict_dn8;
        *var_xwict_rv_slot = var_xwict_rv;
    }

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        var_aphi: f64,
        var_cf_i: f64,
        var_cfb_i: f64,
        var_cfd_i: f64,
        var_ct_t: f64,
        var_ctg_t: f64,
        var_g_0: f64,
        var_guard1275: f64,
        var_phib: f64,
        var_phit: f64,
        var_psce_i: f64,
        var_psceb_i: f64,
        var_psced_i: f64,
        var_v_xb: f64,
        var_v_xb_dn6: f64,
        var_v_xb_dn7: f64,
        var_v_xb_dn8: f64,
        var_vdsx: f64,
        var_vdsx_dn6: f64,
        var_vdsx_dn7: f64,
        var_vgb1: f64,
        var_vgb1_dn12: f64,
        var_vgb1_dn13: f64,
        var_vgb1_dn14: f64,
        var_vgb1_dn15: f64,
        var_vgb1_dn16: f64,
        var_vgb1_dn17: f64,
        var_vgb1_dn18: f64,
        var_vgb1_dn19: f64,
        var_vgb1_dn20: f64,
        var_vgb1_dn5: f64,
        var_vgb1_dn6: f64,
        var_vgb1_dn7: f64,
        var_vgb1_dn8: f64,
        var_vsbstar: f64,
        var_vsbstar_dn12: f64,
        var_vsbstar_dn13: f64,
        var_vsbstar_dn14: f64,
        var_vsbstar_dn15: f64,
        var_vsbstar_dn16: f64,
        var_vsbstar_dn17: f64,
        var_vsbstar_dn18: f64,
        var_vsbstar_dn19: f64,
        var_vsbstar_dn20: f64,
        var_vsbstar_dn5: f64,
        var_vsbstar_dn6: f64,
        var_vsbstar_dn7: f64,
        var_vsbstar_dn8: f64,
        var_vsbx: f64,
        var_vsbx_dn12: f64,
        var_vsbx_dn13: f64,
        var_vsbx_dn14: f64,
        var_vsbx_dn15: f64,
        var_vsbx_dn16: f64,
        var_vsbx_dn17: f64,
        var_vsbx_dn18: f64,
        var_vsbx_dn19: f64,
        var_vsbx_dn20: f64,
        var_vsbx_dn5: f64,
        var_vsbx_dn6: f64,
        var_vsbx_dn7: f64,
        var_vsbx_dn8: f64,
        var_xctmax: f64,
        var_ct_fact_slot: &mut f64,
        var_ct_fact_dn12_slot: &mut f64,
        var_ct_fact_dn13_slot: &mut f64,
        var_ct_fact_dn14_slot: &mut f64,
        var_ct_fact_dn15_slot: &mut f64,
        var_ct_fact_dn16_slot: &mut f64,
        var_ct_fact_dn17_slot: &mut f64,
        var_ct_fact_dn18_slot: &mut f64,
        var_ct_fact_dn19_slot: &mut f64,
        var_ct_fact_dn20_slot: &mut f64,
        var_ct_fact_dn5_slot: &mut f64,
        var_ct_fact_dn6_slot: &mut f64,
        var_ct_fact_dn7_slot: &mut f64,
        var_ct_fact_dn8_slot: &mut f64,
        var_ct_fact_rv_slot: &mut f64,
        var_dctg_slot: &mut f64,
        var_dctg_dn12_slot: &mut f64,
        var_dctg_dn13_slot: &mut f64,
        var_dctg_dn14_slot: &mut f64,
        var_dctg_dn15_slot: &mut f64,
        var_dctg_dn16_slot: &mut f64,
        var_dctg_dn17_slot: &mut f64,
        var_dctg_dn18_slot: &mut f64,
        var_dctg_dn19_slot: &mut f64,
        var_dctg_dn20_slot: &mut f64,
        var_dctg_dn5_slot: &mut f64,
        var_dctg_dn6_slot: &mut f64,
        var_dctg_dn7_slot: &mut f64,
        var_dctg_dn8_slot: &mut f64,
        var_dctg_rv_slot: &mut f64,
        var_delphib_slot: &mut f64,
        var_delphib_dn12_slot: &mut f64,
        var_delphib_dn13_slot: &mut f64,
        var_delphib_dn14_slot: &mut f64,
        var_delphib_dn15_slot: &mut f64,
        var_delphib_dn16_slot: &mut f64,
        var_delphib_dn17_slot: &mut f64,
        var_delphib_dn18_slot: &mut f64,
        var_delphib_dn19_slot: &mut f64,
        var_delphib_dn20_slot: &mut f64,
        var_delphib_dn5_slot: &mut f64,
        var_delphib_dn6_slot: &mut f64,
        var_delphib_dn7_slot: &mut f64,
        var_delphib_dn8_slot: &mut f64,
        var_delphib_rv_slot: &mut f64,
        var_delxb_slot: &mut f64,
        var_delxb_dn12_slot: &mut f64,
        var_delxb_dn13_slot: &mut f64,
        var_delxb_dn14_slot: &mut f64,
        var_delxb_dn15_slot: &mut f64,
        var_delxb_dn16_slot: &mut f64,
        var_delxb_dn17_slot: &mut f64,
        var_delxb_dn18_slot: &mut f64,
        var_delxb_dn19_slot: &mut f64,
        var_delxb_dn20_slot: &mut f64,
        var_delxb_dn5_slot: &mut f64,
        var_delxb_dn6_slot: &mut f64,
        var_delxb_dn7_slot: &mut f64,
        var_delxb_dn8_slot: &mut f64,
        var_delxb_rv_slot: &mut f64,
        var_dphit1_slot: &mut f64,
        var_dphit1_dn12_slot: &mut f64,
        var_dphit1_dn13_slot: &mut f64,
        var_dphit1_dn14_slot: &mut f64,
        var_dphit1_dn15_slot: &mut f64,
        var_dphit1_dn16_slot: &mut f64,
        var_dphit1_dn17_slot: &mut f64,
        var_dphit1_dn18_slot: &mut f64,
        var_dphit1_dn19_slot: &mut f64,
        var_dphit1_dn20_slot: &mut f64,
        var_dphit1_dn5_slot: &mut f64,
        var_dphit1_dn6_slot: &mut f64,
        var_dphit1_dn7_slot: &mut f64,
        var_dphit1_dn8_slot: &mut f64,
        var_dphit1_rv_slot: &mut f64,
        var_gf_slot: &mut f64,
        var_gf2_slot: &mut f64,
        var_gf2_dn12_slot: &mut f64,
        var_gf2_dn13_slot: &mut f64,
        var_gf2_dn14_slot: &mut f64,
        var_gf2_dn15_slot: &mut f64,
        var_gf2_dn16_slot: &mut f64,
        var_gf2_dn17_slot: &mut f64,
        var_gf2_dn18_slot: &mut f64,
        var_gf2_dn19_slot: &mut f64,
        var_gf2_dn20_slot: &mut f64,
        var_gf2_dn5_slot: &mut f64,
        var_gf2_dn6_slot: &mut f64,
        var_gf2_dn7_slot: &mut f64,
        var_gf2_dn8_slot: &mut f64,
        var_gf2_rv_slot: &mut f64,
        var_gf_dn12_slot: &mut f64,
        var_gf_dn13_slot: &mut f64,
        var_gf_dn14_slot: &mut f64,
        var_gf_dn15_slot: &mut f64,
        var_gf_dn16_slot: &mut f64,
        var_gf_dn17_slot: &mut f64,
        var_gf_dn18_slot: &mut f64,
        var_gf_dn19_slot: &mut f64,
        var_gf_dn20_slot: &mut f64,
        var_gf_dn5_slot: &mut f64,
        var_gf_dn6_slot: &mut f64,
        var_gf_dn7_slot: &mut f64,
        var_gf_dn8_slot: &mut f64,
        var_gf_rv_slot: &mut f64,
        var_guard1276_slot: &mut f64,
        var_guard1276_rv_slot: &mut f64,
        var_guard1277_slot: &mut f64,
        var_guard1277_rv_slot: &mut f64,
        var_guard1278_slot: &mut f64,
        var_guard1278_rv_slot: &mut f64,
        var_guard1279_slot: &mut f64,
        var_guard1279_rv_slot: &mut f64,
        var_inv_gf2_slot: &mut f64,
        var_inv_gf2_dn12_slot: &mut f64,
        var_inv_gf2_dn13_slot: &mut f64,
        var_inv_gf2_dn14_slot: &mut f64,
        var_inv_gf2_dn15_slot: &mut f64,
        var_inv_gf2_dn16_slot: &mut f64,
        var_inv_gf2_dn17_slot: &mut f64,
        var_inv_gf2_dn18_slot: &mut f64,
        var_inv_gf2_dn19_slot: &mut f64,
        var_inv_gf2_dn20_slot: &mut f64,
        var_inv_gf2_dn5_slot: &mut f64,
        var_inv_gf2_dn6_slot: &mut f64,
        var_inv_gf2_dn7_slot: &mut f64,
        var_inv_gf2_dn8_slot: &mut f64,
        var_inv_gf2_rv_slot: &mut f64,
        var_inv_phit1_slot: &mut f64,
        var_inv_phit1_dn12_slot: &mut f64,
        var_inv_phit1_dn13_slot: &mut f64,
        var_inv_phit1_dn14_slot: &mut f64,
        var_inv_phit1_dn15_slot: &mut f64,
        var_inv_phit1_dn16_slot: &mut f64,
        var_inv_phit1_dn17_slot: &mut f64,
        var_inv_phit1_dn18_slot: &mut f64,
        var_inv_phit1_dn19_slot: &mut f64,
        var_inv_phit1_dn20_slot: &mut f64,
        var_inv_phit1_dn5_slot: &mut f64,
        var_inv_phit1_dn6_slot: &mut f64,
        var_inv_phit1_dn7_slot: &mut f64,
        var_inv_phit1_dn8_slot: &mut f64,
        var_inv_phit1_rv_slot: &mut f64,
        var_nscr_slot: &mut f64,
        var_nscr_dn12_slot: &mut f64,
        var_nscr_dn13_slot: &mut f64,
        var_nscr_dn14_slot: &mut f64,
        var_nscr_dn15_slot: &mut f64,
        var_nscr_dn16_slot: &mut f64,
        var_nscr_dn17_slot: &mut f64,
        var_nscr_dn18_slot: &mut f64,
        var_nscr_dn19_slot: &mut f64,
        var_nscr_dn20_slot: &mut f64,
        var_nscr_dn5_slot: &mut f64,
        var_nscr_dn6_slot: &mut f64,
        var_nscr_dn7_slot: &mut f64,
        var_nscr_dn8_slot: &mut f64,
        var_nscr_rv_slot: &mut f64,
        var_phit1_slot: &mut f64,
        var_phit1_dn12_slot: &mut f64,
        var_phit1_dn13_slot: &mut f64,
        var_phit1_dn14_slot: &mut f64,
        var_phit1_dn15_slot: &mut f64,
        var_phit1_dn16_slot: &mut f64,
        var_phit1_dn17_slot: &mut f64,
        var_phit1_dn18_slot: &mut f64,
        var_phit1_dn19_slot: &mut f64,
        var_phit1_dn20_slot: &mut f64,
        var_phit1_dn5_slot: &mut f64,
        var_phit1_dn6_slot: &mut f64,
        var_phit1_dn7_slot: &mut f64,
        var_phit1_dn8_slot: &mut f64,
        var_phit1_rv_slot: &mut f64,
        var_phitct_slot: &mut f64,
        var_phitct_dn12_slot: &mut f64,
        var_phitct_dn13_slot: &mut f64,
        var_phitct_dn14_slot: &mut f64,
        var_phitct_dn15_slot: &mut f64,
        var_phitct_dn16_slot: &mut f64,
        var_phitct_dn17_slot: &mut f64,
        var_phitct_dn18_slot: &mut f64,
        var_phitct_dn19_slot: &mut f64,
        var_phitct_dn20_slot: &mut f64,
        var_phitct_dn5_slot: &mut f64,
        var_phitct_dn6_slot: &mut f64,
        var_phitct_dn7_slot: &mut f64,
        var_phitct_dn8_slot: &mut f64,
        var_phitct_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn12_slot: &mut f64,
        var_temp1_dn13_slot: &mut f64,
        var_temp1_dn14_slot: &mut f64,
        var_temp1_dn15_slot: &mut f64,
        var_temp1_dn16_slot: &mut f64,
        var_temp1_dn17_slot: &mut f64,
        var_temp1_dn18_slot: &mut f64,
        var_temp1_dn19_slot: &mut f64,
        var_temp1_dn20_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn12_slot: &mut f64,
        var_temp2_dn13_slot: &mut f64,
        var_temp2_dn14_slot: &mut f64,
        var_temp2_dn15_slot: &mut f64,
        var_temp2_dn16_slot: &mut f64,
        var_temp2_dn17_slot: &mut f64,
        var_temp2_dn18_slot: &mut f64,
        var_temp2_dn19_slot: &mut f64,
        var_temp2_dn20_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_ux_slot: &mut f64,
        var_ux_dn12_slot: &mut f64,
        var_ux_dn13_slot: &mut f64,
        var_ux_dn14_slot: &mut f64,
        var_ux_dn15_slot: &mut f64,
        var_ux_dn16_slot: &mut f64,
        var_ux_dn17_slot: &mut f64,
        var_ux_dn18_slot: &mut f64,
        var_ux_dn19_slot: &mut f64,
        var_ux_dn20_slot: &mut f64,
        var_ux_dn5_slot: &mut f64,
        var_ux_dn6_slot: &mut f64,
        var_ux_dn7_slot: &mut f64,
        var_ux_dn8_slot: &mut f64,
        var_ux_rv_slot: &mut f64,
        var_vdsp_slot: &mut f64,
        var_vdsp_dn6_slot: &mut f64,
        var_vdsp_dn7_slot: &mut f64,
        var_vdsp_rv_slot: &mut f64,
        var_xb_slot: &mut f64,
        var_xb_dn12_slot: &mut f64,
        var_xb_dn13_slot: &mut f64,
        var_xb_dn14_slot: &mut f64,
        var_xb_dn15_slot: &mut f64,
        var_xb_dn16_slot: &mut f64,
        var_xb_dn17_slot: &mut f64,
        var_xb_dn18_slot: &mut f64,
        var_xb_dn19_slot: &mut f64,
        var_xb_dn20_slot: &mut f64,
        var_xb_dn5_slot: &mut f64,
        var_xb_dn6_slot: &mut f64,
        var_xb_dn7_slot: &mut f64,
        var_xb_dn8_slot: &mut f64,
        var_xb_rv_slot: &mut f64,
        var_xct_slot: &mut f64,
        var_xct_dn12_slot: &mut f64,
        var_xct_dn13_slot: &mut f64,
        var_xct_dn14_slot: &mut f64,
        var_xct_dn15_slot: &mut f64,
        var_xct_dn16_slot: &mut f64,
        var_xct_dn17_slot: &mut f64,
        var_xct_dn18_slot: &mut f64,
        var_xct_dn19_slot: &mut f64,
        var_xct_dn20_slot: &mut f64,
        var_xct_dn5_slot: &mut f64,
        var_xct_dn6_slot: &mut f64,
        var_xct_dn7_slot: &mut f64,
        var_xct_dn8_slot: &mut f64,
        var_xct_rv_slot: &mut f64,
        var_xg_slot: &mut f64,
        var_xg_dn12_slot: &mut f64,
        var_xg_dn13_slot: &mut f64,
        var_xg_dn14_slot: &mut f64,
        var_xg_dn15_slot: &mut f64,
        var_xg_dn16_slot: &mut f64,
        var_xg_dn17_slot: &mut f64,
        var_xg_dn18_slot: &mut f64,
        var_xg_dn19_slot: &mut f64,
        var_xg_dn20_slot: &mut f64,
        var_xg_dn5_slot: &mut f64,
        var_xg_dn6_slot: &mut f64,
        var_xg_dn7_slot: &mut f64,
        var_xg_dn8_slot: &mut f64,
        var_xg_rv_slot: &mut f64,
        var_xn_s_slot: &mut f64,
        var_xn_s_dn12_slot: &mut f64,
        var_xn_s_dn13_slot: &mut f64,
        var_xn_s_dn14_slot: &mut f64,
        var_xn_s_dn15_slot: &mut f64,
        var_xn_s_dn16_slot: &mut f64,
        var_xn_s_dn17_slot: &mut f64,
        var_xn_s_dn18_slot: &mut f64,
        var_xn_s_dn19_slot: &mut f64,
        var_xn_s_dn20_slot: &mut f64,
        var_xn_s_dn5_slot: &mut f64,
        var_xn_s_dn6_slot: &mut f64,
        var_xn_s_dn7_slot: &mut f64,
        var_xn_s_dn8_slot: &mut f64,
        var_xn_s_rv_slot: &mut f64,
        var_xno_s_slot: &mut f64,
        var_xno_s_dn12_slot: &mut f64,
        var_xno_s_dn13_slot: &mut f64,
        var_xno_s_dn14_slot: &mut f64,
        var_xno_s_dn15_slot: &mut f64,
        var_xno_s_dn16_slot: &mut f64,
        var_xno_s_dn17_slot: &mut f64,
        var_xno_s_dn18_slot: &mut f64,
        var_xno_s_dn19_slot: &mut f64,
        var_xno_s_dn20_slot: &mut f64,
        var_xno_s_dn5_slot: &mut f64,
        var_xno_s_dn6_slot: &mut f64,
        var_xno_s_dn7_slot: &mut f64,
        var_xno_s_dn8_slot: &mut f64,
        var_xno_s_rv_slot: &mut f64,
    ) {
        let mut var_ct_fact: f64 = *var_ct_fact_slot;
        let mut var_ct_fact_dn12: f64 = *var_ct_fact_dn12_slot;
        let mut var_ct_fact_dn13: f64 = *var_ct_fact_dn13_slot;
        let mut var_ct_fact_dn14: f64 = *var_ct_fact_dn14_slot;
        let mut var_ct_fact_dn15: f64 = *var_ct_fact_dn15_slot;
        let mut var_ct_fact_dn16: f64 = *var_ct_fact_dn16_slot;
        let mut var_ct_fact_dn17: f64 = *var_ct_fact_dn17_slot;
        let mut var_ct_fact_dn18: f64 = *var_ct_fact_dn18_slot;
        let mut var_ct_fact_dn19: f64 = *var_ct_fact_dn19_slot;
        let mut var_ct_fact_dn20: f64 = *var_ct_fact_dn20_slot;
        let mut var_ct_fact_dn5: f64 = *var_ct_fact_dn5_slot;
        let mut var_ct_fact_dn6: f64 = *var_ct_fact_dn6_slot;
        let mut var_ct_fact_dn7: f64 = *var_ct_fact_dn7_slot;
        let mut var_ct_fact_dn8: f64 = *var_ct_fact_dn8_slot;
        let mut var_ct_fact_rv: f64 = *var_ct_fact_rv_slot;
        let mut var_dctg: f64 = *var_dctg_slot;
        let mut var_dctg_dn12: f64 = *var_dctg_dn12_slot;
        let mut var_dctg_dn13: f64 = *var_dctg_dn13_slot;
        let mut var_dctg_dn14: f64 = *var_dctg_dn14_slot;
        let mut var_dctg_dn15: f64 = *var_dctg_dn15_slot;
        let mut var_dctg_dn16: f64 = *var_dctg_dn16_slot;
        let mut var_dctg_dn17: f64 = *var_dctg_dn17_slot;
        let mut var_dctg_dn18: f64 = *var_dctg_dn18_slot;
        let mut var_dctg_dn19: f64 = *var_dctg_dn19_slot;
        let mut var_dctg_dn20: f64 = *var_dctg_dn20_slot;
        let mut var_dctg_dn5: f64 = *var_dctg_dn5_slot;
        let mut var_dctg_dn6: f64 = *var_dctg_dn6_slot;
        let mut var_dctg_dn7: f64 = *var_dctg_dn7_slot;
        let mut var_dctg_dn8: f64 = *var_dctg_dn8_slot;
        let mut var_dctg_rv: f64 = *var_dctg_rv_slot;
        let mut var_delphib: f64 = *var_delphib_slot;
        let mut var_delphib_dn12: f64 = *var_delphib_dn12_slot;
        let mut var_delphib_dn13: f64 = *var_delphib_dn13_slot;
        let mut var_delphib_dn14: f64 = *var_delphib_dn14_slot;
        let mut var_delphib_dn15: f64 = *var_delphib_dn15_slot;
        let mut var_delphib_dn16: f64 = *var_delphib_dn16_slot;
        let mut var_delphib_dn17: f64 = *var_delphib_dn17_slot;
        let mut var_delphib_dn18: f64 = *var_delphib_dn18_slot;
        let mut var_delphib_dn19: f64 = *var_delphib_dn19_slot;
        let mut var_delphib_dn20: f64 = *var_delphib_dn20_slot;
        let mut var_delphib_dn5: f64 = *var_delphib_dn5_slot;
        let mut var_delphib_dn6: f64 = *var_delphib_dn6_slot;
        let mut var_delphib_dn7: f64 = *var_delphib_dn7_slot;
        let mut var_delphib_dn8: f64 = *var_delphib_dn8_slot;
        let mut var_delphib_rv: f64 = *var_delphib_rv_slot;
        let mut var_delxb: f64 = *var_delxb_slot;
        let mut var_delxb_dn12: f64 = *var_delxb_dn12_slot;
        let mut var_delxb_dn13: f64 = *var_delxb_dn13_slot;
        let mut var_delxb_dn14: f64 = *var_delxb_dn14_slot;
        let mut var_delxb_dn15: f64 = *var_delxb_dn15_slot;
        let mut var_delxb_dn16: f64 = *var_delxb_dn16_slot;
        let mut var_delxb_dn17: f64 = *var_delxb_dn17_slot;
        let mut var_delxb_dn18: f64 = *var_delxb_dn18_slot;
        let mut var_delxb_dn19: f64 = *var_delxb_dn19_slot;
        let mut var_delxb_dn20: f64 = *var_delxb_dn20_slot;
        let mut var_delxb_dn5: f64 = *var_delxb_dn5_slot;
        let mut var_delxb_dn6: f64 = *var_delxb_dn6_slot;
        let mut var_delxb_dn7: f64 = *var_delxb_dn7_slot;
        let mut var_delxb_dn8: f64 = *var_delxb_dn8_slot;
        let mut var_delxb_rv: f64 = *var_delxb_rv_slot;
        let mut var_dphit1: f64 = *var_dphit1_slot;
        let mut var_dphit1_dn12: f64 = *var_dphit1_dn12_slot;
        let mut var_dphit1_dn13: f64 = *var_dphit1_dn13_slot;
        let mut var_dphit1_dn14: f64 = *var_dphit1_dn14_slot;
        let mut var_dphit1_dn15: f64 = *var_dphit1_dn15_slot;
        let mut var_dphit1_dn16: f64 = *var_dphit1_dn16_slot;
        let mut var_dphit1_dn17: f64 = *var_dphit1_dn17_slot;
        let mut var_dphit1_dn18: f64 = *var_dphit1_dn18_slot;
        let mut var_dphit1_dn19: f64 = *var_dphit1_dn19_slot;
        let mut var_dphit1_dn20: f64 = *var_dphit1_dn20_slot;
        let mut var_dphit1_dn5: f64 = *var_dphit1_dn5_slot;
        let mut var_dphit1_dn6: f64 = *var_dphit1_dn6_slot;
        let mut var_dphit1_dn7: f64 = *var_dphit1_dn7_slot;
        let mut var_dphit1_dn8: f64 = *var_dphit1_dn8_slot;
        let mut var_dphit1_rv: f64 = *var_dphit1_rv_slot;
        let mut var_gf: f64 = *var_gf_slot;
        let mut var_gf2: f64 = *var_gf2_slot;
        let mut var_gf2_dn12: f64 = *var_gf2_dn12_slot;
        let mut var_gf2_dn13: f64 = *var_gf2_dn13_slot;
        let mut var_gf2_dn14: f64 = *var_gf2_dn14_slot;
        let mut var_gf2_dn15: f64 = *var_gf2_dn15_slot;
        let mut var_gf2_dn16: f64 = *var_gf2_dn16_slot;
        let mut var_gf2_dn17: f64 = *var_gf2_dn17_slot;
        let mut var_gf2_dn18: f64 = *var_gf2_dn18_slot;
        let mut var_gf2_dn19: f64 = *var_gf2_dn19_slot;
        let mut var_gf2_dn20: f64 = *var_gf2_dn20_slot;
        let mut var_gf2_dn5: f64 = *var_gf2_dn5_slot;
        let mut var_gf2_dn6: f64 = *var_gf2_dn6_slot;
        let mut var_gf2_dn7: f64 = *var_gf2_dn7_slot;
        let mut var_gf2_dn8: f64 = *var_gf2_dn8_slot;
        let mut var_gf2_rv: f64 = *var_gf2_rv_slot;
        let mut var_gf_dn12: f64 = *var_gf_dn12_slot;
        let mut var_gf_dn13: f64 = *var_gf_dn13_slot;
        let mut var_gf_dn14: f64 = *var_gf_dn14_slot;
        let mut var_gf_dn15: f64 = *var_gf_dn15_slot;
        let mut var_gf_dn16: f64 = *var_gf_dn16_slot;
        let mut var_gf_dn17: f64 = *var_gf_dn17_slot;
        let mut var_gf_dn18: f64 = *var_gf_dn18_slot;
        let mut var_gf_dn19: f64 = *var_gf_dn19_slot;
        let mut var_gf_dn20: f64 = *var_gf_dn20_slot;
        let mut var_gf_dn5: f64 = *var_gf_dn5_slot;
        let mut var_gf_dn6: f64 = *var_gf_dn6_slot;
        let mut var_gf_dn7: f64 = *var_gf_dn7_slot;
        let mut var_gf_dn8: f64 = *var_gf_dn8_slot;
        let mut var_gf_rv: f64 = *var_gf_rv_slot;
        let mut var_guard1276: f64 = *var_guard1276_slot;
        let mut var_guard1276_rv: f64 = *var_guard1276_rv_slot;
        let mut var_guard1277: f64 = *var_guard1277_slot;
        let mut var_guard1277_rv: f64 = *var_guard1277_rv_slot;
        let mut var_guard1278: f64 = *var_guard1278_slot;
        let mut var_guard1278_rv: f64 = *var_guard1278_rv_slot;
        let mut var_guard1279: f64 = *var_guard1279_slot;
        let mut var_guard1279_rv: f64 = *var_guard1279_rv_slot;
        let mut var_inv_gf2: f64 = *var_inv_gf2_slot;
        let mut var_inv_gf2_dn12: f64 = *var_inv_gf2_dn12_slot;
        let mut var_inv_gf2_dn13: f64 = *var_inv_gf2_dn13_slot;
        let mut var_inv_gf2_dn14: f64 = *var_inv_gf2_dn14_slot;
        let mut var_inv_gf2_dn15: f64 = *var_inv_gf2_dn15_slot;
        let mut var_inv_gf2_dn16: f64 = *var_inv_gf2_dn16_slot;
        let mut var_inv_gf2_dn17: f64 = *var_inv_gf2_dn17_slot;
        let mut var_inv_gf2_dn18: f64 = *var_inv_gf2_dn18_slot;
        let mut var_inv_gf2_dn19: f64 = *var_inv_gf2_dn19_slot;
        let mut var_inv_gf2_dn20: f64 = *var_inv_gf2_dn20_slot;
        let mut var_inv_gf2_dn5: f64 = *var_inv_gf2_dn5_slot;
        let mut var_inv_gf2_dn6: f64 = *var_inv_gf2_dn6_slot;
        let mut var_inv_gf2_dn7: f64 = *var_inv_gf2_dn7_slot;
        let mut var_inv_gf2_dn8: f64 = *var_inv_gf2_dn8_slot;
        let mut var_inv_gf2_rv: f64 = *var_inv_gf2_rv_slot;
        let mut var_inv_phit1: f64 = *var_inv_phit1_slot;
        let mut var_inv_phit1_dn12: f64 = *var_inv_phit1_dn12_slot;
        let mut var_inv_phit1_dn13: f64 = *var_inv_phit1_dn13_slot;
        let mut var_inv_phit1_dn14: f64 = *var_inv_phit1_dn14_slot;
        let mut var_inv_phit1_dn15: f64 = *var_inv_phit1_dn15_slot;
        let mut var_inv_phit1_dn16: f64 = *var_inv_phit1_dn16_slot;
        let mut var_inv_phit1_dn17: f64 = *var_inv_phit1_dn17_slot;
        let mut var_inv_phit1_dn18: f64 = *var_inv_phit1_dn18_slot;
        let mut var_inv_phit1_dn19: f64 = *var_inv_phit1_dn19_slot;
        let mut var_inv_phit1_dn20: f64 = *var_inv_phit1_dn20_slot;
        let mut var_inv_phit1_dn5: f64 = *var_inv_phit1_dn5_slot;
        let mut var_inv_phit1_dn6: f64 = *var_inv_phit1_dn6_slot;
        let mut var_inv_phit1_dn7: f64 = *var_inv_phit1_dn7_slot;
        let mut var_inv_phit1_dn8: f64 = *var_inv_phit1_dn8_slot;
        let mut var_inv_phit1_rv: f64 = *var_inv_phit1_rv_slot;
        let mut var_nscr: f64 = *var_nscr_slot;
        let mut var_nscr_dn12: f64 = *var_nscr_dn12_slot;
        let mut var_nscr_dn13: f64 = *var_nscr_dn13_slot;
        let mut var_nscr_dn14: f64 = *var_nscr_dn14_slot;
        let mut var_nscr_dn15: f64 = *var_nscr_dn15_slot;
        let mut var_nscr_dn16: f64 = *var_nscr_dn16_slot;
        let mut var_nscr_dn17: f64 = *var_nscr_dn17_slot;
        let mut var_nscr_dn18: f64 = *var_nscr_dn18_slot;
        let mut var_nscr_dn19: f64 = *var_nscr_dn19_slot;
        let mut var_nscr_dn20: f64 = *var_nscr_dn20_slot;
        let mut var_nscr_dn5: f64 = *var_nscr_dn5_slot;
        let mut var_nscr_dn6: f64 = *var_nscr_dn6_slot;
        let mut var_nscr_dn7: f64 = *var_nscr_dn7_slot;
        let mut var_nscr_dn8: f64 = *var_nscr_dn8_slot;
        let mut var_nscr_rv: f64 = *var_nscr_rv_slot;
        let mut var_phit1: f64 = *var_phit1_slot;
        let mut var_phit1_dn12: f64 = *var_phit1_dn12_slot;
        let mut var_phit1_dn13: f64 = *var_phit1_dn13_slot;
        let mut var_phit1_dn14: f64 = *var_phit1_dn14_slot;
        let mut var_phit1_dn15: f64 = *var_phit1_dn15_slot;
        let mut var_phit1_dn16: f64 = *var_phit1_dn16_slot;
        let mut var_phit1_dn17: f64 = *var_phit1_dn17_slot;
        let mut var_phit1_dn18: f64 = *var_phit1_dn18_slot;
        let mut var_phit1_dn19: f64 = *var_phit1_dn19_slot;
        let mut var_phit1_dn20: f64 = *var_phit1_dn20_slot;
        let mut var_phit1_dn5: f64 = *var_phit1_dn5_slot;
        let mut var_phit1_dn6: f64 = *var_phit1_dn6_slot;
        let mut var_phit1_dn7: f64 = *var_phit1_dn7_slot;
        let mut var_phit1_dn8: f64 = *var_phit1_dn8_slot;
        let mut var_phit1_rv: f64 = *var_phit1_rv_slot;
        let mut var_phitct: f64 = *var_phitct_slot;
        let mut var_phitct_dn12: f64 = *var_phitct_dn12_slot;
        let mut var_phitct_dn13: f64 = *var_phitct_dn13_slot;
        let mut var_phitct_dn14: f64 = *var_phitct_dn14_slot;
        let mut var_phitct_dn15: f64 = *var_phitct_dn15_slot;
        let mut var_phitct_dn16: f64 = *var_phitct_dn16_slot;
        let mut var_phitct_dn17: f64 = *var_phitct_dn17_slot;
        let mut var_phitct_dn18: f64 = *var_phitct_dn18_slot;
        let mut var_phitct_dn19: f64 = *var_phitct_dn19_slot;
        let mut var_phitct_dn20: f64 = *var_phitct_dn20_slot;
        let mut var_phitct_dn5: f64 = *var_phitct_dn5_slot;
        let mut var_phitct_dn6: f64 = *var_phitct_dn6_slot;
        let mut var_phitct_dn7: f64 = *var_phitct_dn7_slot;
        let mut var_phitct_dn8: f64 = *var_phitct_dn8_slot;
        let mut var_phitct_rv: f64 = *var_phitct_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn12: f64 = *var_temp1_dn12_slot;
        let mut var_temp1_dn13: f64 = *var_temp1_dn13_slot;
        let mut var_temp1_dn14: f64 = *var_temp1_dn14_slot;
        let mut var_temp1_dn15: f64 = *var_temp1_dn15_slot;
        let mut var_temp1_dn16: f64 = *var_temp1_dn16_slot;
        let mut var_temp1_dn17: f64 = *var_temp1_dn17_slot;
        let mut var_temp1_dn18: f64 = *var_temp1_dn18_slot;
        let mut var_temp1_dn19: f64 = *var_temp1_dn19_slot;
        let mut var_temp1_dn20: f64 = *var_temp1_dn20_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn12: f64 = *var_temp2_dn12_slot;
        let mut var_temp2_dn13: f64 = *var_temp2_dn13_slot;
        let mut var_temp2_dn14: f64 = *var_temp2_dn14_slot;
        let mut var_temp2_dn15: f64 = *var_temp2_dn15_slot;
        let mut var_temp2_dn16: f64 = *var_temp2_dn16_slot;
        let mut var_temp2_dn17: f64 = *var_temp2_dn17_slot;
        let mut var_temp2_dn18: f64 = *var_temp2_dn18_slot;
        let mut var_temp2_dn19: f64 = *var_temp2_dn19_slot;
        let mut var_temp2_dn20: f64 = *var_temp2_dn20_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_ux: f64 = *var_ux_slot;
        let mut var_ux_dn12: f64 = *var_ux_dn12_slot;
        let mut var_ux_dn13: f64 = *var_ux_dn13_slot;
        let mut var_ux_dn14: f64 = *var_ux_dn14_slot;
        let mut var_ux_dn15: f64 = *var_ux_dn15_slot;
        let mut var_ux_dn16: f64 = *var_ux_dn16_slot;
        let mut var_ux_dn17: f64 = *var_ux_dn17_slot;
        let mut var_ux_dn18: f64 = *var_ux_dn18_slot;
        let mut var_ux_dn19: f64 = *var_ux_dn19_slot;
        let mut var_ux_dn20: f64 = *var_ux_dn20_slot;
        let mut var_ux_dn5: f64 = *var_ux_dn5_slot;
        let mut var_ux_dn6: f64 = *var_ux_dn6_slot;
        let mut var_ux_dn7: f64 = *var_ux_dn7_slot;
        let mut var_ux_dn8: f64 = *var_ux_dn8_slot;
        let mut var_ux_rv: f64 = *var_ux_rv_slot;
        let mut var_vdsp: f64 = *var_vdsp_slot;
        let mut var_vdsp_dn6: f64 = *var_vdsp_dn6_slot;
        let mut var_vdsp_dn7: f64 = *var_vdsp_dn7_slot;
        let mut var_vdsp_rv: f64 = *var_vdsp_rv_slot;
        let mut var_xb: f64 = *var_xb_slot;
        let mut var_xb_dn12: f64 = *var_xb_dn12_slot;
        let mut var_xb_dn13: f64 = *var_xb_dn13_slot;
        let mut var_xb_dn14: f64 = *var_xb_dn14_slot;
        let mut var_xb_dn15: f64 = *var_xb_dn15_slot;
        let mut var_xb_dn16: f64 = *var_xb_dn16_slot;
        let mut var_xb_dn17: f64 = *var_xb_dn17_slot;
        let mut var_xb_dn18: f64 = *var_xb_dn18_slot;
        let mut var_xb_dn19: f64 = *var_xb_dn19_slot;
        let mut var_xb_dn20: f64 = *var_xb_dn20_slot;
        let mut var_xb_dn5: f64 = *var_xb_dn5_slot;
        let mut var_xb_dn6: f64 = *var_xb_dn6_slot;
        let mut var_xb_dn7: f64 = *var_xb_dn7_slot;
        let mut var_xb_dn8: f64 = *var_xb_dn8_slot;
        let mut var_xb_rv: f64 = *var_xb_rv_slot;
        let mut var_xct: f64 = *var_xct_slot;
        let mut var_xct_dn12: f64 = *var_xct_dn12_slot;
        let mut var_xct_dn13: f64 = *var_xct_dn13_slot;
        let mut var_xct_dn14: f64 = *var_xct_dn14_slot;
        let mut var_xct_dn15: f64 = *var_xct_dn15_slot;
        let mut var_xct_dn16: f64 = *var_xct_dn16_slot;
        let mut var_xct_dn17: f64 = *var_xct_dn17_slot;
        let mut var_xct_dn18: f64 = *var_xct_dn18_slot;
        let mut var_xct_dn19: f64 = *var_xct_dn19_slot;
        let mut var_xct_dn20: f64 = *var_xct_dn20_slot;
        let mut var_xct_dn5: f64 = *var_xct_dn5_slot;
        let mut var_xct_dn6: f64 = *var_xct_dn6_slot;
        let mut var_xct_dn7: f64 = *var_xct_dn7_slot;
        let mut var_xct_dn8: f64 = *var_xct_dn8_slot;
        let mut var_xct_rv: f64 = *var_xct_rv_slot;
        let mut var_xg: f64 = *var_xg_slot;
        let mut var_xg_dn12: f64 = *var_xg_dn12_slot;
        let mut var_xg_dn13: f64 = *var_xg_dn13_slot;
        let mut var_xg_dn14: f64 = *var_xg_dn14_slot;
        let mut var_xg_dn15: f64 = *var_xg_dn15_slot;
        let mut var_xg_dn16: f64 = *var_xg_dn16_slot;
        let mut var_xg_dn17: f64 = *var_xg_dn17_slot;
        let mut var_xg_dn18: f64 = *var_xg_dn18_slot;
        let mut var_xg_dn19: f64 = *var_xg_dn19_slot;
        let mut var_xg_dn20: f64 = *var_xg_dn20_slot;
        let mut var_xg_dn5: f64 = *var_xg_dn5_slot;
        let mut var_xg_dn6: f64 = *var_xg_dn6_slot;
        let mut var_xg_dn7: f64 = *var_xg_dn7_slot;
        let mut var_xg_dn8: f64 = *var_xg_dn8_slot;
        let mut var_xg_rv: f64 = *var_xg_rv_slot;
        let mut var_xn_s: f64 = *var_xn_s_slot;
        let mut var_xn_s_dn12: f64 = *var_xn_s_dn12_slot;
        let mut var_xn_s_dn13: f64 = *var_xn_s_dn13_slot;
        let mut var_xn_s_dn14: f64 = *var_xn_s_dn14_slot;
        let mut var_xn_s_dn15: f64 = *var_xn_s_dn15_slot;
        let mut var_xn_s_dn16: f64 = *var_xn_s_dn16_slot;
        let mut var_xn_s_dn17: f64 = *var_xn_s_dn17_slot;
        let mut var_xn_s_dn18: f64 = *var_xn_s_dn18_slot;
        let mut var_xn_s_dn19: f64 = *var_xn_s_dn19_slot;
        let mut var_xn_s_dn20: f64 = *var_xn_s_dn20_slot;
        let mut var_xn_s_dn5: f64 = *var_xn_s_dn5_slot;
        let mut var_xn_s_dn6: f64 = *var_xn_s_dn6_slot;
        let mut var_xn_s_dn7: f64 = *var_xn_s_dn7_slot;
        let mut var_xn_s_dn8: f64 = *var_xn_s_dn8_slot;
        let mut var_xn_s_rv: f64 = *var_xn_s_rv_slot;
        let mut var_xno_s: f64 = *var_xno_s_slot;
        let mut var_xno_s_dn12: f64 = *var_xno_s_dn12_slot;
        let mut var_xno_s_dn13: f64 = *var_xno_s_dn13_slot;
        let mut var_xno_s_dn14: f64 = *var_xno_s_dn14_slot;
        let mut var_xno_s_dn15: f64 = *var_xno_s_dn15_slot;
        let mut var_xno_s_dn16: f64 = *var_xno_s_dn16_slot;
        let mut var_xno_s_dn17: f64 = *var_xno_s_dn17_slot;
        let mut var_xno_s_dn18: f64 = *var_xno_s_dn18_slot;
        let mut var_xno_s_dn19: f64 = *var_xno_s_dn19_slot;
        let mut var_xno_s_dn20: f64 = *var_xno_s_dn20_slot;
        let mut var_xno_s_dn5: f64 = *var_xno_s_dn5_slot;
        let mut var_xno_s_dn6: f64 = *var_xno_s_dn6_slot;
        let mut var_xno_s_dn7: f64 = *var_xno_s_dn7_slot;
        let mut var_xno_s_dn8: f64 = *var_xno_s_dn8_slot;
        let mut var_xno_s_rv: f64 = *var_xno_s_rv_slot;

        let (assign41180_e54172, assign41180_e54172_d_n5, assign41180_e54172_d_n6, assign41180_e54172_d_n7, assign41180_e54172_d_n8, assign41180_e54172_d_n12, assign41180_e54172_d_n13, assign41180_e54172_d_n14, assign41180_e54172_d_n15, assign41180_e54172_d_n16, assign41180_e54172_d_n17, assign41180_e54172_d_n18, assign41180_e54172_d_n19, assign41180_e54172_d_n20,) = {
    if (var_guard1275 != 0.0) {
        let assign41180_e54155: f64 = (-var_xctmax);
        let assign41180_e54156: f64 = (var_temp1 + assign41180_e54155);
        let assign41180_e54159: f64 = (-var_xctmax);
        let assign41180_e54160: f64 = (var_temp1 - assign41180_e54159);
        let assign41180_e54163: f64 = (-var_xctmax);
        let assign41180_e54164: f64 = (var_temp1 - assign41180_e54163);
        let assign41180_e54165: f64 = (assign41180_e54160 * assign41180_e54164);
        let assign41180_e54167: f64 = (assign41180_e54165 + 20.0);
        let assign41180_e54168: f64 = (assign41180_e54167).sqrt();
        let assign41180_e54169: f64 = (assign41180_e54156 + assign41180_e54168);
        let assign41180_e54170: f64 = (0.5 * assign41180_e54169);
        (assign41180_e54170, (0.5 * (var_temp1_dn5 + (((var_temp1_dn5 * assign41180_e54164) + (assign41180_e54160 * var_temp1_dn5)) / (2.0 * assign41180_e54168)))), (0.5 * (var_temp1_dn6 + (((var_temp1_dn6 * assign41180_e54164) + (assign41180_e54160 * var_temp1_dn6)) / (2.0 * assign41180_e54168)))), (0.5 * (var_temp1_dn7 + (((var_temp1_dn7 * assign41180_e54164) + (assign41180_e54160 * var_temp1_dn7)) / (2.0 * assign41180_e54168)))), (0.5 * (var_temp1_dn8 + (((var_temp1_dn8 * assign41180_e54164) + (assign41180_e54160 * var_temp1_dn8)) / (2.0 * assign41180_e54168)))), (0.5 * (var_temp1_dn12 + (((var_temp1_dn12 * assign41180_e54164) + (assign41180_e54160 * var_temp1_dn12)) / (2.0 * assign41180_e54168)))), (0.5 * (var_temp1_dn13 + (((var_temp1_dn13 * assign41180_e54164) + (assign41180_e54160 * var_temp1_dn13)) / (2.0 * assign41180_e54168)))), (0.5 * (var_temp1_dn14 + (((var_temp1_dn14 * assign41180_e54164) + (assign41180_e54160 * var_temp1_dn14)) / (2.0 * assign41180_e54168)))), (0.5 * (var_temp1_dn15 + (((var_temp1_dn15 * assign41180_e54164) + (assign41180_e54160 * var_temp1_dn15)) / (2.0 * assign41180_e54168)))), (0.5 * (var_temp1_dn16 + (((var_temp1_dn16 * assign41180_e54164) + (assign41180_e54160 * var_temp1_dn16)) / (2.0 * assign41180_e54168)))), (0.5 * (var_temp1_dn17 + (((var_temp1_dn17 * assign41180_e54164) + (assign41180_e54160 * var_temp1_dn17)) / (2.0 * assign41180_e54168)))), (0.5 * (var_temp1_dn18 + (((var_temp1_dn18 * assign41180_e54164) + (assign41180_e54160 * var_temp1_dn18)) / (2.0 * assign41180_e54168)))), (0.5 * (var_temp1_dn19 + (((var_temp1_dn19 * assign41180_e54164) + (assign41180_e54160 * var_temp1_dn19)) / (2.0 * assign41180_e54168)))), (0.5 * (var_temp1_dn20 + (((var_temp1_dn20 * assign41180_e54164) + (assign41180_e54160 * var_temp1_dn20)) / (2.0 * assign41180_e54168)))),)
    } else {
        (var_xct, var_xct_dn5, var_xct_dn6, var_xct_dn7, var_xct_dn8, var_xct_dn12, var_xct_dn13, var_xct_dn14, var_xct_dn15, var_xct_dn16, var_xct_dn17, var_xct_dn18, var_xct_dn19, var_xct_dn20,)
    }
};
        var_xct = assign41180_e54172;
        var_xct_dn5 = assign41180_e54172_d_n5;
        var_xct_dn6 = assign41180_e54172_d_n6;
        var_xct_dn7 = assign41180_e54172_d_n7;
        var_xct_dn8 = assign41180_e54172_d_n8;
        var_xct_dn12 = assign41180_e54172_d_n12;
        var_xct_dn13 = assign41180_e54172_d_n13;
        var_xct_dn14 = assign41180_e54172_d_n14;
        var_xct_dn15 = assign41180_e54172_d_n15;
        var_xct_dn16 = assign41180_e54172_d_n16;
        var_xct_dn17 = assign41180_e54172_d_n17;
        var_xct_dn18 = assign41180_e54172_d_n18;
        var_xct_dn19 = assign41180_e54172_d_n19;
        var_xct_dn20 = assign41180_e54172_d_n20;
        var_xct_rv = 0.0;

        let (assign41190_e54182, assign41190_e54182_d_n5, assign41190_e54182_d_n6, assign41190_e54182_d_n7, assign41190_e54182_d_n8, assign41190_e54182_d_n12, assign41190_e54182_d_n13, assign41190_e54182_d_n14, assign41190_e54182_d_n15, assign41190_e54182_d_n16, assign41190_e54182_d_n17, assign41190_e54182_d_n18, assign41190_e54182_d_n19, assign41190_e54182_d_n20,) = {
    if (var_guard1275 != 0.0) {
        let assign41190_e54177: f64 = (var_xct / var_xctmax);
        let assign41190_e54179: f64 = (assign41190_e54177 + 1.0);
        let assign41190_e54180: f64 = (var_ctg_t * assign41190_e54179);
        (assign41190_e54180, (var_ctg_t * (var_xct_dn5 / var_xctmax)), (var_ctg_t * (var_xct_dn6 / var_xctmax)), (var_ctg_t * (var_xct_dn7 / var_xctmax)), (var_ctg_t * (var_xct_dn8 / var_xctmax)), (var_ctg_t * (var_xct_dn12 / var_xctmax)), (var_ctg_t * (var_xct_dn13 / var_xctmax)), (var_ctg_t * (var_xct_dn14 / var_xctmax)), (var_ctg_t * (var_xct_dn15 / var_xctmax)), (var_ctg_t * (var_xct_dn16 / var_xctmax)), (var_ctg_t * (var_xct_dn17 / var_xctmax)), (var_ctg_t * (var_xct_dn18 / var_xctmax)), (var_ctg_t * (var_xct_dn19 / var_xctmax)), (var_ctg_t * (var_xct_dn20 / var_xctmax)),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn12, var_temp2_dn13, var_temp2_dn14, var_temp2_dn15, var_temp2_dn16, var_temp2_dn17, var_temp2_dn18, var_temp2_dn19, var_temp2_dn20,)
    }
};
        var_temp2 = assign41190_e54182;
        var_temp2_dn5 = assign41190_e54182_d_n5;
        var_temp2_dn6 = assign41190_e54182_d_n6;
        var_temp2_dn7 = assign41190_e54182_d_n7;
        var_temp2_dn8 = assign41190_e54182_d_n8;
        var_temp2_dn12 = assign41190_e54182_d_n12;
        var_temp2_dn13 = assign41190_e54182_d_n13;
        var_temp2_dn14 = assign41190_e54182_d_n14;
        var_temp2_dn15 = assign41190_e54182_d_n15;
        var_temp2_dn16 = assign41190_e54182_d_n16;
        var_temp2_dn17 = assign41190_e54182_d_n17;
        var_temp2_dn18 = assign41190_e54182_d_n18;
        var_temp2_dn19 = assign41190_e54182_d_n19;
        var_temp2_dn20 = assign41190_e54182_d_n20;
        var_temp2_rv = 0.0;

        let assign41200_e54185: f64 = (-230.25850929940458);
        let assign41200_e54186: f64 = if var_temp2 > assign41200_e54185 { 1.0 } else { 0.0 };
        var_guard1276 = assign41200_e54186;
        var_guard1276_rv = 0.0;

        let (assign41210_e54193, assign41210_e54193_d_n5, assign41210_e54193_d_n6, assign41210_e54193_d_n7, assign41210_e54193_d_n8, assign41210_e54193_d_n12, assign41210_e54193_d_n13, assign41210_e54193_d_n14, assign41210_e54193_d_n15, assign41210_e54193_d_n16, assign41210_e54193_d_n17, assign41210_e54193_d_n18, assign41210_e54193_d_n19, assign41210_e54193_d_n20,) = {
    if ((var_guard1275 != 0.0) && (var_guard1276 != 0.0)) {
        let assign41210_e54191: f64 = (var_temp2).exp();
        (assign41210_e54191, (assign41210_e54191 * var_temp2_dn5), (assign41210_e54191 * var_temp2_dn6), (assign41210_e54191 * var_temp2_dn7), (assign41210_e54191 * var_temp2_dn8), (assign41210_e54191 * var_temp2_dn12), (assign41210_e54191 * var_temp2_dn13), (assign41210_e54191 * var_temp2_dn14), (assign41210_e54191 * var_temp2_dn15), (assign41210_e54191 * var_temp2_dn16), (assign41210_e54191 * var_temp2_dn17), (assign41210_e54191 * var_temp2_dn18), (assign41210_e54191 * var_temp2_dn19), (assign41210_e54191 * var_temp2_dn20),)
    } else {
        (var_dctg, var_dctg_dn5, var_dctg_dn6, var_dctg_dn7, var_dctg_dn8, var_dctg_dn12, var_dctg_dn13, var_dctg_dn14, var_dctg_dn15, var_dctg_dn16, var_dctg_dn17, var_dctg_dn18, var_dctg_dn19, var_dctg_dn20,)
    }
};
        var_dctg = assign41210_e54193;
        var_dctg_dn5 = assign41210_e54193_d_n5;
        var_dctg_dn6 = assign41210_e54193_d_n6;
        var_dctg_dn7 = assign41210_e54193_d_n7;
        var_dctg_dn8 = assign41210_e54193_d_n8;
        var_dctg_dn12 = assign41210_e54193_d_n12;
        var_dctg_dn13 = assign41210_e54193_d_n13;
        var_dctg_dn14 = assign41210_e54193_d_n14;
        var_dctg_dn15 = assign41210_e54193_d_n15;
        var_dctg_dn16 = assign41210_e54193_d_n16;
        var_dctg_dn17 = assign41210_e54193_d_n17;
        var_dctg_dn18 = assign41210_e54193_d_n18;
        var_dctg_dn19 = assign41210_e54193_d_n19;
        var_dctg_dn20 = assign41210_e54193_d_n20;
        var_dctg_rv = 0.0;

        let (assign41220_e54225, assign41220_e54225_d_n5, assign41220_e54225_d_n6, assign41220_e54225_d_n7, assign41220_e54225_d_n8, assign41220_e54225_d_n12, assign41220_e54225_d_n13, assign41220_e54225_d_n14, assign41220_e54225_d_n15, assign41220_e54225_d_n16, assign41220_e54225_d_n17, assign41220_e54225_d_n18, assign41220_e54225_d_n19, assign41220_e54225_d_n20,) = {
    if ((var_guard1275 != 0.0) && (var_guard1276 == 0.0)) {
        let assign41220_e54201: f64 = (-230.25850929940458);
        let assign41220_e54203: f64 = (assign41220_e54201 - var_temp2);
        let assign41220_e54207: f64 = (-230.25850929940458);
        let assign41220_e54209: f64 = (assign41220_e54207 - var_temp2);
        let assign41220_e54212: f64 = (-230.25850929940458);
        let assign41220_e54214: f64 = (assign41220_e54212 - var_temp2);
        let assign41220_e54216: f64 = (assign41220_e54214 * 0.3333333333333333);
        let assign41220_e54217: f64 = (1.0 + assign41220_e54216);
        let assign41220_e54218: f64 = (assign41220_e54209 * assign41220_e54217);
        let assign41220_e54219: f64 = (0.5 * assign41220_e54218);
        let assign41220_e54220: f64 = (1.0 + assign41220_e54219);
        let assign41220_e54221: f64 = (assign41220_e54203 * assign41220_e54220);
        let assign41220_e54222: f64 = (1.0 + assign41220_e54221);
        let assign41220_e54223: f64 = (1e-100 / assign41220_e54222);
        (assign41220_e54223, (-((1e-100 * (((-var_temp2_dn5) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-var_temp2_dn5) * assign41220_e54217) + (assign41220_e54209 * ((-var_temp2_dn5) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-var_temp2_dn6) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-var_temp2_dn6) * assign41220_e54217) + (assign41220_e54209 * ((-var_temp2_dn6) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-var_temp2_dn7) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-var_temp2_dn7) * assign41220_e54217) + (assign41220_e54209 * ((-var_temp2_dn7) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-var_temp2_dn8) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-var_temp2_dn8) * assign41220_e54217) + (assign41220_e54209 * ((-var_temp2_dn8) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-var_temp2_dn12) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-var_temp2_dn12) * assign41220_e54217) + (assign41220_e54209 * ((-var_temp2_dn12) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-var_temp2_dn13) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-var_temp2_dn13) * assign41220_e54217) + (assign41220_e54209 * ((-var_temp2_dn13) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-var_temp2_dn14) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-var_temp2_dn14) * assign41220_e54217) + (assign41220_e54209 * ((-var_temp2_dn14) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-var_temp2_dn15) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-var_temp2_dn15) * assign41220_e54217) + (assign41220_e54209 * ((-var_temp2_dn15) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-var_temp2_dn16) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-var_temp2_dn16) * assign41220_e54217) + (assign41220_e54209 * ((-var_temp2_dn16) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-var_temp2_dn17) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-var_temp2_dn17) * assign41220_e54217) + (assign41220_e54209 * ((-var_temp2_dn17) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-var_temp2_dn18) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-var_temp2_dn18) * assign41220_e54217) + (assign41220_e54209 * ((-var_temp2_dn18) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-var_temp2_dn19) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-var_temp2_dn19) * assign41220_e54217) + (assign41220_e54209 * ((-var_temp2_dn19) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))), (-((1e-100 * (((-var_temp2_dn20) * assign41220_e54220) + (assign41220_e54203 * (0.5 * (((-var_temp2_dn20) * assign41220_e54217) + (assign41220_e54209 * ((-var_temp2_dn20) * 0.3333333333333333))))))) / (assign41220_e54222 * assign41220_e54222))),)
    } else {
        (var_dctg, var_dctg_dn5, var_dctg_dn6, var_dctg_dn7, var_dctg_dn8, var_dctg_dn12, var_dctg_dn13, var_dctg_dn14, var_dctg_dn15, var_dctg_dn16, var_dctg_dn17, var_dctg_dn18, var_dctg_dn19, var_dctg_dn20,)
    }
};
        var_dctg = assign41220_e54225;
        var_dctg_dn5 = assign41220_e54225_d_n5;
        var_dctg_dn6 = assign41220_e54225_d_n6;
        var_dctg_dn7 = assign41220_e54225_d_n7;
        var_dctg_dn8 = assign41220_e54225_d_n8;
        var_dctg_dn12 = assign41220_e54225_d_n12;
        var_dctg_dn13 = assign41220_e54225_d_n13;
        var_dctg_dn14 = assign41220_e54225_d_n14;
        var_dctg_dn15 = assign41220_e54225_d_n15;
        var_dctg_dn16 = assign41220_e54225_d_n16;
        var_dctg_dn17 = assign41220_e54225_d_n17;
        var_dctg_dn18 = assign41220_e54225_d_n18;
        var_dctg_dn19 = assign41220_e54225_d_n19;
        var_dctg_dn20 = assign41220_e54225_d_n20;
        var_dctg_rv = 0.0;

        let assign41230_e54229: f64 = (var_ct_t * var_dctg);
        let assign41230_e54230: f64 = (1.0 + assign41230_e54229);
        var_ct_fact = assign41230_e54230;
        var_ct_fact_dn5 = (var_ct_t * var_dctg_dn5);
        var_ct_fact_dn6 = (var_ct_t * var_dctg_dn6);
        var_ct_fact_dn7 = (var_ct_t * var_dctg_dn7);
        var_ct_fact_dn8 = (var_ct_t * var_dctg_dn8);
        var_ct_fact_dn12 = (var_ct_t * var_dctg_dn12);
        var_ct_fact_dn13 = (var_ct_t * var_dctg_dn13);
        var_ct_fact_dn14 = (var_ct_t * var_dctg_dn14);
        var_ct_fact_dn15 = (var_ct_t * var_dctg_dn15);
        var_ct_fact_dn16 = (var_ct_t * var_dctg_dn16);
        var_ct_fact_dn17 = (var_ct_t * var_dctg_dn17);
        var_ct_fact_dn18 = (var_ct_t * var_dctg_dn18);
        var_ct_fact_dn19 = (var_ct_t * var_dctg_dn19);
        var_ct_fact_dn20 = (var_ct_t * var_dctg_dn20);
        var_ct_fact_rv = 0.0;

        let assign41240_e54233: f64 = (var_phit * var_ct_fact);
        var_phitct = assign41240_e54233;
        var_phitct_dn5 = (var_phit * var_ct_fact_dn5);
        var_phitct_dn6 = (var_phit * var_ct_fact_dn6);
        var_phitct_dn7 = (var_phit * var_ct_fact_dn7);
        var_phitct_dn8 = (var_phit * var_ct_fact_dn8);
        var_phitct_dn12 = (var_phit * var_ct_fact_dn12);
        var_phitct_dn13 = (var_phit * var_ct_fact_dn13);
        var_phitct_dn14 = (var_phit * var_ct_fact_dn14);
        var_phitct_dn15 = (var_phit * var_ct_fact_dn15);
        var_phitct_dn16 = (var_phit * var_ct_fact_dn16);
        var_phitct_dn17 = (var_phit * var_ct_fact_dn17);
        var_phitct_dn18 = (var_phit * var_ct_fact_dn18);
        var_phitct_dn19 = (var_phit * var_ct_fact_dn19);
        var_phitct_dn20 = (var_phit * var_ct_fact_dn20);
        var_phitct_rv = 0.0;

        let assign41250_e54238: f64 = (var_psced_i * var_vdsx);
        let assign41250_e54239: f64 = (1.0 + assign41250_e54238);
        let assign41250_e54240: f64 = (var_psce_i * assign41250_e54239);
        let assign41250_e54244: f64 = (var_psceb_i * var_vsbx);
        let assign41250_e54245: f64 = (1.0 + assign41250_e54244);
        let assign41250_e54246: f64 = (assign41250_e54240 * assign41250_e54245);
        var_dphit1 = assign41250_e54246;
        var_dphit1_dn5 = (assign41250_e54240 * (var_psceb_i * var_vsbx_dn5));
        var_dphit1_dn6 = (((var_psce_i * (var_psced_i * var_vdsx_dn6)) * assign41250_e54245) + (assign41250_e54240 * (var_psceb_i * var_vsbx_dn6)));
        var_dphit1_dn7 = (((var_psce_i * (var_psced_i * var_vdsx_dn7)) * assign41250_e54245) + (assign41250_e54240 * (var_psceb_i * var_vsbx_dn7)));
        var_dphit1_dn8 = (assign41250_e54240 * (var_psceb_i * var_vsbx_dn8));
        var_dphit1_dn12 = (assign41250_e54240 * (var_psceb_i * var_vsbx_dn12));
        var_dphit1_dn13 = (assign41250_e54240 * (var_psceb_i * var_vsbx_dn13));
        var_dphit1_dn14 = (assign41250_e54240 * (var_psceb_i * var_vsbx_dn14));
        var_dphit1_dn15 = (assign41250_e54240 * (var_psceb_i * var_vsbx_dn15));
        var_dphit1_dn16 = (assign41250_e54240 * (var_psceb_i * var_vsbx_dn16));
        var_dphit1_dn17 = (assign41250_e54240 * (var_psceb_i * var_vsbx_dn17));
        var_dphit1_dn18 = (assign41250_e54240 * (var_psceb_i * var_vsbx_dn18));
        var_dphit1_dn19 = (assign41250_e54240 * (var_psceb_i * var_vsbx_dn19));
        var_dphit1_dn20 = (assign41250_e54240 * (var_psceb_i * var_vsbx_dn20));
        var_dphit1_rv = 0.0;

        let assign41260_e54250: f64 = (1.0 + var_dphit1);
        let assign41260_e54251: f64 = (var_phitct * assign41260_e54250);
        var_phit1 = assign41260_e54251;
        var_phit1_dn5 = ((var_phitct_dn5 * assign41260_e54250) + (var_phitct * var_dphit1_dn5));
        var_phit1_dn6 = ((var_phitct_dn6 * assign41260_e54250) + (var_phitct * var_dphit1_dn6));
        var_phit1_dn7 = ((var_phitct_dn7 * assign41260_e54250) + (var_phitct * var_dphit1_dn7));
        var_phit1_dn8 = ((var_phitct_dn8 * assign41260_e54250) + (var_phitct * var_dphit1_dn8));
        var_phit1_dn12 = ((var_phitct_dn12 * assign41260_e54250) + (var_phitct * var_dphit1_dn12));
        var_phit1_dn13 = ((var_phitct_dn13 * assign41260_e54250) + (var_phitct * var_dphit1_dn13));
        var_phit1_dn14 = ((var_phitct_dn14 * assign41260_e54250) + (var_phitct * var_dphit1_dn14));
        var_phit1_dn15 = ((var_phitct_dn15 * assign41260_e54250) + (var_phitct * var_dphit1_dn15));
        var_phit1_dn16 = ((var_phitct_dn16 * assign41260_e54250) + (var_phitct * var_dphit1_dn16));
        var_phit1_dn17 = ((var_phitct_dn17 * assign41260_e54250) + (var_phitct * var_dphit1_dn17));
        var_phit1_dn18 = ((var_phitct_dn18 * assign41260_e54250) + (var_phitct * var_dphit1_dn18));
        var_phit1_dn19 = ((var_phitct_dn19 * assign41260_e54250) + (var_phitct * var_dphit1_dn19));
        var_phit1_dn20 = ((var_phitct_dn20 * assign41260_e54250) + (var_phitct * var_dphit1_dn20));
        var_phit1_rv = 0.0;

        let assign41270_e54254: f64 = (1.0 / var_phit1);
        var_inv_phit1 = assign41270_e54254;
        var_inv_phit1_dn5 = (-(var_phit1_dn5 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn6 = (-(var_phit1_dn6 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn7 = (-(var_phit1_dn7 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn8 = (-(var_phit1_dn8 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn12 = (-(var_phit1_dn12 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn13 = (-(var_phit1_dn13 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn14 = (-(var_phit1_dn14 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn15 = (-(var_phit1_dn15 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn16 = (-(var_phit1_dn16 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn17 = (-(var_phit1_dn17 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn18 = (-(var_phit1_dn18 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn19 = (-(var_phit1_dn19 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn20 = (-(var_phit1_dn20 / (var_phit1 * var_phit1)));
        var_inv_phit1_rv = 0.0;

        let assign41280_e54258: f64 = (var_phit * var_inv_phit1);
        let assign41280_e54259: f64 = (assign41280_e54258).sqrt();
        let assign41280_e54260: f64 = (var_g_0 * assign41280_e54259);
        var_gf = assign41280_e54260;
        var_gf_dn5 = (var_g_0 * ((var_phit * var_inv_phit1_dn5) / (2.0 * assign41280_e54259)));
        var_gf_dn6 = (var_g_0 * ((var_phit * var_inv_phit1_dn6) / (2.0 * assign41280_e54259)));
        var_gf_dn7 = (var_g_0 * ((var_phit * var_inv_phit1_dn7) / (2.0 * assign41280_e54259)));
        var_gf_dn8 = (var_g_0 * ((var_phit * var_inv_phit1_dn8) / (2.0 * assign41280_e54259)));
        var_gf_dn12 = (var_g_0 * ((var_phit * var_inv_phit1_dn12) / (2.0 * assign41280_e54259)));
        var_gf_dn13 = (var_g_0 * ((var_phit * var_inv_phit1_dn13) / (2.0 * assign41280_e54259)));
        var_gf_dn14 = (var_g_0 * ((var_phit * var_inv_phit1_dn14) / (2.0 * assign41280_e54259)));
        var_gf_dn15 = (var_g_0 * ((var_phit * var_inv_phit1_dn15) / (2.0 * assign41280_e54259)));
        var_gf_dn16 = (var_g_0 * ((var_phit * var_inv_phit1_dn16) / (2.0 * assign41280_e54259)));
        var_gf_dn17 = (var_g_0 * ((var_phit * var_inv_phit1_dn17) / (2.0 * assign41280_e54259)));
        var_gf_dn18 = (var_g_0 * ((var_phit * var_inv_phit1_dn18) / (2.0 * assign41280_e54259)));
        var_gf_dn19 = (var_g_0 * ((var_phit * var_inv_phit1_dn19) / (2.0 * assign41280_e54259)));
        var_gf_dn20 = (var_g_0 * ((var_phit * var_inv_phit1_dn20) / (2.0 * assign41280_e54259)));
        var_gf_rv = 0.0;

        let assign41290_e54263: f64 = (var_gf * var_gf);
        var_gf2 = assign41290_e54263;
        var_gf2_dn5 = ((var_gf_dn5 * var_gf) + (var_gf * var_gf_dn5));
        var_gf2_dn6 = ((var_gf_dn6 * var_gf) + (var_gf * var_gf_dn6));
        var_gf2_dn7 = ((var_gf_dn7 * var_gf) + (var_gf * var_gf_dn7));
        var_gf2_dn8 = ((var_gf_dn8 * var_gf) + (var_gf * var_gf_dn8));
        var_gf2_dn12 = ((var_gf_dn12 * var_gf) + (var_gf * var_gf_dn12));
        var_gf2_dn13 = ((var_gf_dn13 * var_gf) + (var_gf * var_gf_dn13));
        var_gf2_dn14 = ((var_gf_dn14 * var_gf) + (var_gf * var_gf_dn14));
        var_gf2_dn15 = ((var_gf_dn15 * var_gf) + (var_gf * var_gf_dn15));
        var_gf2_dn16 = ((var_gf_dn16 * var_gf) + (var_gf * var_gf_dn16));
        var_gf2_dn17 = ((var_gf_dn17 * var_gf) + (var_gf * var_gf_dn17));
        var_gf2_dn18 = ((var_gf_dn18 * var_gf) + (var_gf * var_gf_dn18));
        var_gf2_dn19 = ((var_gf_dn19 * var_gf) + (var_gf * var_gf_dn19));
        var_gf2_dn20 = ((var_gf_dn20 * var_gf) + (var_gf * var_gf_dn20));
        var_gf2_rv = 0.0;

        let assign41300_e54266: f64 = (1.0 / var_gf2);
        var_inv_gf2 = assign41300_e54266;
        var_inv_gf2_dn5 = (-(var_gf2_dn5 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn6 = (-(var_gf2_dn6 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn7 = (-(var_gf2_dn7 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn8 = (-(var_gf2_dn8 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn12 = (-(var_gf2_dn12 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn13 = (-(var_gf2_dn13 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn14 = (-(var_gf2_dn14 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn15 = (-(var_gf2_dn15 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn16 = (-(var_gf2_dn16 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn17 = (-(var_gf2_dn17 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn18 = (-(var_gf2_dn18 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn19 = (-(var_gf2_dn19 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn20 = (-(var_gf2_dn20 / (var_gf2 * var_gf2)));
        var_inv_gf2_rv = 0.0;

        let assign41310_e54269: f64 = (var_vsbstar * var_inv_phit1);
        var_ux = assign41310_e54269;
        var_ux_dn5 = ((var_vsbstar_dn5 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn5));
        var_ux_dn6 = ((var_vsbstar_dn6 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn6));
        var_ux_dn7 = ((var_vsbstar_dn7 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn7));
        var_ux_dn8 = ((var_vsbstar_dn8 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn8));
        var_ux_dn12 = ((var_vsbstar_dn12 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn12));
        var_ux_dn13 = ((var_vsbstar_dn13 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn13));
        var_ux_dn14 = ((var_vsbstar_dn14 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn14));
        var_ux_dn15 = ((var_vsbstar_dn15 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn15));
        var_ux_dn16 = ((var_vsbstar_dn16 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn16));
        var_ux_dn17 = ((var_vsbstar_dn17 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn17));
        var_ux_dn18 = ((var_vsbstar_dn18 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn18));
        var_ux_dn19 = ((var_vsbstar_dn19 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn19));
        var_ux_dn20 = ((var_vsbstar_dn20 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn20));
        var_ux_rv = 0.0;

        let assign41320_e54272: f64 = (var_vgb1 * var_inv_phit1);
        var_xg = assign41320_e54272;
        var_xg_dn5 = ((var_vgb1_dn5 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn5));
        var_xg_dn6 = ((var_vgb1_dn6 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn6));
        var_xg_dn7 = ((var_vgb1_dn7 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn7));
        var_xg_dn8 = ((var_vgb1_dn8 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn8));
        var_xg_dn12 = ((var_vgb1_dn12 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn12));
        var_xg_dn13 = ((var_vgb1_dn13 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn13));
        var_xg_dn14 = ((var_vgb1_dn14 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn14));
        var_xg_dn15 = ((var_vgb1_dn15 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn15));
        var_xg_dn16 = ((var_vgb1_dn16 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn16));
        var_xg_dn17 = ((var_vgb1_dn17 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn17));
        var_xg_dn18 = ((var_vgb1_dn18 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn18));
        var_xg_dn19 = ((var_vgb1_dn19 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn19));
        var_xg_dn20 = ((var_vgb1_dn20 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn20));
        var_xg_rv = 0.0;

        let assign41330_e54275: f64 = (2.0 * var_vdsx);
        let assign41330_e54280: f64 = (var_cfd_i * var_vdsx);
        let assign41330_e54281: f64 = (1.0 + assign41330_e54280);
        let assign41330_e54282: f64 = (assign41330_e54281).sqrt();
        let assign41330_e54283: f64 = (1.0 + assign41330_e54282);
        let assign41330_e54284: f64 = (assign41330_e54275 / assign41330_e54283);
        var_vdsp = assign41330_e54284;
        var_vdsp_dn6 = ((((2.0 * var_vdsx_dn6) * assign41330_e54283) - (assign41330_e54275 * ((var_cfd_i * var_vdsx_dn6) / (2.0 * assign41330_e54282)))) / (assign41330_e54283 * assign41330_e54283));
        var_vdsp_dn7 = ((((2.0 * var_vdsx_dn7) * assign41330_e54283) - (assign41330_e54275 * ((var_cfd_i * var_vdsx_dn7) / (2.0 * assign41330_e54282)))) / (assign41330_e54283 * assign41330_e54283));
        var_vdsp_rv = 0.0;

        let assign41340_e54287: f64 = (var_cf_i * var_vdsp);
        let assign41340_e54291: f64 = (var_cfb_i * var_vsbx);
        let assign41340_e54292: f64 = (1.0 + assign41340_e54291);
        let assign41340_e54293: f64 = (assign41340_e54287 * assign41340_e54292);
        var_delphib = assign41340_e54293;
        var_delphib_dn5 = (assign41340_e54287 * (var_cfb_i * var_vsbx_dn5));
        var_delphib_dn6 = (((var_cf_i * var_vdsp_dn6) * assign41340_e54292) + (assign41340_e54287 * (var_cfb_i * var_vsbx_dn6)));
        var_delphib_dn7 = (((var_cf_i * var_vdsp_dn7) * assign41340_e54292) + (assign41340_e54287 * (var_cfb_i * var_vsbx_dn7)));
        var_delphib_dn8 = (assign41340_e54287 * (var_cfb_i * var_vsbx_dn8));
        var_delphib_dn12 = (assign41340_e54287 * (var_cfb_i * var_vsbx_dn12));
        var_delphib_dn13 = (assign41340_e54287 * (var_cfb_i * var_vsbx_dn13));
        var_delphib_dn14 = (assign41340_e54287 * (var_cfb_i * var_vsbx_dn14));
        var_delphib_dn15 = (assign41340_e54287 * (var_cfb_i * var_vsbx_dn15));
        var_delphib_dn16 = (assign41340_e54287 * (var_cfb_i * var_vsbx_dn16));
        var_delphib_dn17 = (assign41340_e54287 * (var_cfb_i * var_vsbx_dn17));
        var_delphib_dn18 = (assign41340_e54287 * (var_cfb_i * var_vsbx_dn18));
        var_delphib_dn19 = (assign41340_e54287 * (var_cfb_i * var_vsbx_dn19));
        var_delphib_dn20 = (assign41340_e54287 * (var_cfb_i * var_vsbx_dn20));
        var_delphib_rv = 0.0;

        let assign41350_e54296: f64 = (var_phib * var_inv_phit1);
        var_xb = assign41350_e54296;
        var_xb_dn5 = (var_phib * var_inv_phit1_dn5);
        var_xb_dn6 = (var_phib * var_inv_phit1_dn6);
        var_xb_dn7 = (var_phib * var_inv_phit1_dn7);
        var_xb_dn8 = (var_phib * var_inv_phit1_dn8);
        var_xb_dn12 = (var_phib * var_inv_phit1_dn12);
        var_xb_dn13 = (var_phib * var_inv_phit1_dn13);
        var_xb_dn14 = (var_phib * var_inv_phit1_dn14);
        var_xb_dn15 = (var_phib * var_inv_phit1_dn15);
        var_xb_dn16 = (var_phib * var_inv_phit1_dn16);
        var_xb_dn17 = (var_phib * var_inv_phit1_dn17);
        var_xb_dn18 = (var_phib * var_inv_phit1_dn18);
        var_xb_dn19 = (var_phib * var_inv_phit1_dn19);
        var_xb_dn20 = (var_phib * var_inv_phit1_dn20);
        var_xb_rv = 0.0;

        let assign41360_e54299: f64 = (var_v_xb * var_v_xb);
        let assign41360_e54301: f64 = (assign41360_e54299 + var_aphi);
        let assign41360_e54302: f64 = (assign41360_e54301).sqrt();
        var_temp1 = assign41360_e54302;
        var_temp1_dn5 = 0.0;
        var_temp1_dn6 = (((var_v_xb_dn6 * var_v_xb) + (var_v_xb * var_v_xb_dn6)) / (2.0 * assign41360_e54302));
        var_temp1_dn7 = (((var_v_xb_dn7 * var_v_xb) + (var_v_xb * var_v_xb_dn7)) / (2.0 * assign41360_e54302));
        var_temp1_dn8 = (((var_v_xb_dn8 * var_v_xb) + (var_v_xb * var_v_xb_dn8)) / (2.0 * assign41360_e54302));
        var_temp1_dn12 = 0.0;
        var_temp1_dn13 = 0.0;
        var_temp1_dn14 = 0.0;
        var_temp1_dn15 = 0.0;
        var_temp1_dn16 = 0.0;
        var_temp1_dn17 = 0.0;
        var_temp1_dn18 = 0.0;
        var_temp1_dn19 = 0.0;
        var_temp1_dn20 = 0.0;
        var_temp1_rv = 0.0;

        let assign41370_e54305: f64 = (var_v_xb - var_delphib);
        let assign41370_e54308: f64 = (var_v_xb - var_delphib);
        let assign41370_e54309: f64 = (assign41370_e54305 * assign41370_e54308);
        let assign41370_e54311: f64 = (assign41370_e54309 + var_aphi);
        let assign41370_e54312: f64 = (assign41370_e54311).sqrt();
        var_temp2 = assign41370_e54312;
        var_temp2_dn5 = ((((-var_delphib_dn5) * assign41370_e54308) + (assign41370_e54305 * (-var_delphib_dn5))) / (2.0 * assign41370_e54312));
        var_temp2_dn6 = ((((var_v_xb_dn6 - var_delphib_dn6) * assign41370_e54308) + (assign41370_e54305 * (var_v_xb_dn6 - var_delphib_dn6))) / (2.0 * assign41370_e54312));
        var_temp2_dn7 = ((((var_v_xb_dn7 - var_delphib_dn7) * assign41370_e54308) + (assign41370_e54305 * (var_v_xb_dn7 - var_delphib_dn7))) / (2.0 * assign41370_e54312));
        var_temp2_dn8 = ((((var_v_xb_dn8 - var_delphib_dn8) * assign41370_e54308) + (assign41370_e54305 * (var_v_xb_dn8 - var_delphib_dn8))) / (2.0 * assign41370_e54312));
        var_temp2_dn12 = ((((-var_delphib_dn12) * assign41370_e54308) + (assign41370_e54305 * (-var_delphib_dn12))) / (2.0 * assign41370_e54312));
        var_temp2_dn13 = ((((-var_delphib_dn13) * assign41370_e54308) + (assign41370_e54305 * (-var_delphib_dn13))) / (2.0 * assign41370_e54312));
        var_temp2_dn14 = ((((-var_delphib_dn14) * assign41370_e54308) + (assign41370_e54305 * (-var_delphib_dn14))) / (2.0 * assign41370_e54312));
        var_temp2_dn15 = ((((-var_delphib_dn15) * assign41370_e54308) + (assign41370_e54305 * (-var_delphib_dn15))) / (2.0 * assign41370_e54312));
        var_temp2_dn16 = ((((-var_delphib_dn16) * assign41370_e54308) + (assign41370_e54305 * (-var_delphib_dn16))) / (2.0 * assign41370_e54312));
        var_temp2_dn17 = ((((-var_delphib_dn17) * assign41370_e54308) + (assign41370_e54305 * (-var_delphib_dn17))) / (2.0 * assign41370_e54312));
        var_temp2_dn18 = ((((-var_delphib_dn18) * assign41370_e54308) + (assign41370_e54305 * (-var_delphib_dn18))) / (2.0 * assign41370_e54312));
        var_temp2_dn19 = ((((-var_delphib_dn19) * assign41370_e54308) + (assign41370_e54305 * (-var_delphib_dn19))) / (2.0 * assign41370_e54312));
        var_temp2_dn20 = ((((-var_delphib_dn20) * assign41370_e54308) + (assign41370_e54305 * (-var_delphib_dn20))) / (2.0 * assign41370_e54312));
        var_temp2_rv = 0.0;

        let assign41380_e54315: f64 = (0.5 * var_inv_phit1);
        let assign41380_e54318: f64 = (var_delphib + var_temp1);
        let assign41380_e54320: f64 = (assign41380_e54318 - var_temp2);
        let assign41380_e54321: f64 = (assign41380_e54315 * assign41380_e54320);
        var_delxb = assign41380_e54321;
        var_delxb_dn5 = (((0.5 * var_inv_phit1_dn5) * assign41380_e54320) + (assign41380_e54315 * ((var_delphib_dn5 + var_temp1_dn5) - var_temp2_dn5)));
        var_delxb_dn6 = (((0.5 * var_inv_phit1_dn6) * assign41380_e54320) + (assign41380_e54315 * ((var_delphib_dn6 + var_temp1_dn6) - var_temp2_dn6)));
        var_delxb_dn7 = (((0.5 * var_inv_phit1_dn7) * assign41380_e54320) + (assign41380_e54315 * ((var_delphib_dn7 + var_temp1_dn7) - var_temp2_dn7)));
        var_delxb_dn8 = (((0.5 * var_inv_phit1_dn8) * assign41380_e54320) + (assign41380_e54315 * ((var_delphib_dn8 + var_temp1_dn8) - var_temp2_dn8)));
        var_delxb_dn12 = (((0.5 * var_inv_phit1_dn12) * assign41380_e54320) + (assign41380_e54315 * ((var_delphib_dn12 + var_temp1_dn12) - var_temp2_dn12)));
        var_delxb_dn13 = (((0.5 * var_inv_phit1_dn13) * assign41380_e54320) + (assign41380_e54315 * ((var_delphib_dn13 + var_temp1_dn13) - var_temp2_dn13)));
        var_delxb_dn14 = (((0.5 * var_inv_phit1_dn14) * assign41380_e54320) + (assign41380_e54315 * ((var_delphib_dn14 + var_temp1_dn14) - var_temp2_dn14)));
        var_delxb_dn15 = (((0.5 * var_inv_phit1_dn15) * assign41380_e54320) + (assign41380_e54315 * ((var_delphib_dn15 + var_temp1_dn15) - var_temp2_dn15)));
        var_delxb_dn16 = (((0.5 * var_inv_phit1_dn16) * assign41380_e54320) + (assign41380_e54315 * ((var_delphib_dn16 + var_temp1_dn16) - var_temp2_dn16)));
        var_delxb_dn17 = (((0.5 * var_inv_phit1_dn17) * assign41380_e54320) + (assign41380_e54315 * ((var_delphib_dn17 + var_temp1_dn17) - var_temp2_dn17)));
        var_delxb_dn18 = (((0.5 * var_inv_phit1_dn18) * assign41380_e54320) + (assign41380_e54315 * ((var_delphib_dn18 + var_temp1_dn18) - var_temp2_dn18)));
        var_delxb_dn19 = (((0.5 * var_inv_phit1_dn19) * assign41380_e54320) + (assign41380_e54315 * ((var_delphib_dn19 + var_temp1_dn19) - var_temp2_dn19)));
        var_delxb_dn20 = (((0.5 * var_inv_phit1_dn20) * assign41380_e54320) + (assign41380_e54315 * ((var_delphib_dn20 + var_temp1_dn20) - var_temp2_dn20)));
        var_delxb_rv = 0.0;

        let assign41390_e54324: f64 = (var_xb + var_ux);
        var_xno_s = assign41390_e54324;
        var_xno_s_dn5 = (var_xb_dn5 + var_ux_dn5);
        var_xno_s_dn6 = (var_xb_dn6 + var_ux_dn6);
        var_xno_s_dn7 = (var_xb_dn7 + var_ux_dn7);
        var_xno_s_dn8 = (var_xb_dn8 + var_ux_dn8);
        var_xno_s_dn12 = (var_xb_dn12 + var_ux_dn12);
        var_xno_s_dn13 = (var_xb_dn13 + var_ux_dn13);
        var_xno_s_dn14 = (var_xb_dn14 + var_ux_dn14);
        var_xno_s_dn15 = (var_xb_dn15 + var_ux_dn15);
        var_xno_s_dn16 = (var_xb_dn16 + var_ux_dn16);
        var_xno_s_dn17 = (var_xb_dn17 + var_ux_dn17);
        var_xno_s_dn18 = (var_xb_dn18 + var_ux_dn18);
        var_xno_s_dn19 = (var_xb_dn19 + var_ux_dn19);
        var_xno_s_dn20 = (var_xb_dn20 + var_ux_dn20);
        var_xno_s_rv = 0.0;

        let assign41400_e54327: f64 = (var_xno_s - var_delxb);
        var_xn_s = assign41400_e54327;
        var_xn_s_dn5 = (var_xno_s_dn5 - var_delxb_dn5);
        var_xn_s_dn6 = (var_xno_s_dn6 - var_delxb_dn6);
        var_xn_s_dn7 = (var_xno_s_dn7 - var_delxb_dn7);
        var_xn_s_dn8 = (var_xno_s_dn8 - var_delxb_dn8);
        var_xn_s_dn12 = (var_xno_s_dn12 - var_delxb_dn12);
        var_xn_s_dn13 = (var_xno_s_dn13 - var_delxb_dn13);
        var_xn_s_dn14 = (var_xno_s_dn14 - var_delxb_dn14);
        var_xn_s_dn15 = (var_xno_s_dn15 - var_delxb_dn15);
        var_xn_s_dn16 = (var_xno_s_dn16 - var_delxb_dn16);
        var_xn_s_dn17 = (var_xno_s_dn17 - var_delxb_dn17);
        var_xn_s_dn18 = (var_xno_s_dn18 - var_delxb_dn18);
        var_xn_s_dn19 = (var_xno_s_dn19 - var_delxb_dn19);
        var_xn_s_dn20 = (var_xno_s_dn20 - var_delxb_dn20);
        var_xn_s_rv = 0.0;

        let assign41410_e54330: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        var_guard1277 = assign41410_e54330;
        var_guard1277_rv = 0.0;

        let assign41420_e54332: f64 = (var_xn_s).abs();
        let assign41420_e54334: f64 = if assign41420_e54332 < 1e-5 { 1.0 } else { 0.0 };
        var_guard1278 = assign41420_e54334;
        var_guard1278_rv = 0.0;

        let (assign41430_e54354, assign41430_e54354_d_n5, assign41430_e54354_d_n6, assign41430_e54354_d_n7, assign41430_e54354_d_n8, assign41430_e54354_d_n12, assign41430_e54354_d_n13, assign41430_e54354_d_n14, assign41430_e54354_d_n15, assign41430_e54354_d_n16, assign41430_e54354_d_n17, assign41430_e54354_d_n18, assign41430_e54354_d_n19, assign41430_e54354_d_n20,) = {
    if ((var_guard1277 != 0.0) && (var_guard1278 != 0.0)) {
        let assign41430_e54343: f64 = (0.5 * var_xn_s);
        let assign41430_e54347: f64 = (0.3125 * var_xn_s);
        let assign41430_e54348: f64 = (1.0 - assign41430_e54347);
        let assign41430_e54349: f64 = (assign41430_e54343 * assign41430_e54348);
        let assign41430_e54350: f64 = (1.0 - assign41430_e54349);
        let assign41430_e54351: f64 = (var_gf * assign41430_e54350);
        let assign41430_e54352: f64 = (1.0 + assign41430_e54351);
        (assign41430_e54352, ((var_gf_dn5 * assign41430_e54350) + (var_gf * (-(((0.5 * var_xn_s_dn5) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * var_xn_s_dn5))))))), ((var_gf_dn6 * assign41430_e54350) + (var_gf * (-(((0.5 * var_xn_s_dn6) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * var_xn_s_dn6))))))), ((var_gf_dn7 * assign41430_e54350) + (var_gf * (-(((0.5 * var_xn_s_dn7) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * var_xn_s_dn7))))))), ((var_gf_dn8 * assign41430_e54350) + (var_gf * (-(((0.5 * var_xn_s_dn8) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * var_xn_s_dn8))))))), ((var_gf_dn12 * assign41430_e54350) + (var_gf * (-(((0.5 * var_xn_s_dn12) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * var_xn_s_dn12))))))), ((var_gf_dn13 * assign41430_e54350) + (var_gf * (-(((0.5 * var_xn_s_dn13) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * var_xn_s_dn13))))))), ((var_gf_dn14 * assign41430_e54350) + (var_gf * (-(((0.5 * var_xn_s_dn14) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * var_xn_s_dn14))))))), ((var_gf_dn15 * assign41430_e54350) + (var_gf * (-(((0.5 * var_xn_s_dn15) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * var_xn_s_dn15))))))), ((var_gf_dn16 * assign41430_e54350) + (var_gf * (-(((0.5 * var_xn_s_dn16) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * var_xn_s_dn16))))))), ((var_gf_dn17 * assign41430_e54350) + (var_gf * (-(((0.5 * var_xn_s_dn17) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * var_xn_s_dn17))))))), ((var_gf_dn18 * assign41430_e54350) + (var_gf * (-(((0.5 * var_xn_s_dn18) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * var_xn_s_dn18))))))), ((var_gf_dn19 * assign41430_e54350) + (var_gf * (-(((0.5 * var_xn_s_dn19) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * var_xn_s_dn19))))))), ((var_gf_dn20 * assign41430_e54350) + (var_gf * (-(((0.5 * var_xn_s_dn20) * assign41430_e54348) + (assign41430_e54343 * (-(0.3125 * var_xn_s_dn20))))))),)
    } else {
        (var_nscr, var_nscr_dn5, var_nscr_dn6, var_nscr_dn7, var_nscr_dn8, var_nscr_dn12, var_nscr_dn13, var_nscr_dn14, var_nscr_dn15, var_nscr_dn16, var_nscr_dn17, var_nscr_dn18, var_nscr_dn19, var_nscr_dn20,)
    }
};
        var_nscr = assign41430_e54354;
        var_nscr_dn5 = assign41430_e54354_d_n5;
        var_nscr_dn6 = assign41430_e54354_d_n6;
        var_nscr_dn7 = assign41430_e54354_d_n7;
        var_nscr_dn8 = assign41430_e54354_d_n8;
        var_nscr_dn12 = assign41430_e54354_d_n12;
        var_nscr_dn13 = assign41430_e54354_d_n13;
        var_nscr_dn14 = assign41430_e54354_d_n14;
        var_nscr_dn15 = assign41430_e54354_d_n15;
        var_nscr_dn16 = assign41430_e54354_d_n16;
        var_nscr_dn17 = assign41430_e54354_d_n17;
        var_nscr_dn18 = assign41430_e54354_d_n18;
        var_nscr_dn19 = assign41430_e54354_d_n19;
        var_nscr_dn20 = assign41430_e54354_d_n20;
        var_nscr_rv = 0.0;

        let assign41440_e54357: f64 = if var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        var_guard1279 = assign41440_e54357;
        var_guard1279_rv = 0.0;

        *var_ct_fact_slot = var_ct_fact;
        *var_ct_fact_dn12_slot = var_ct_fact_dn12;
        *var_ct_fact_dn13_slot = var_ct_fact_dn13;
        *var_ct_fact_dn14_slot = var_ct_fact_dn14;
        *var_ct_fact_dn15_slot = var_ct_fact_dn15;
        *var_ct_fact_dn16_slot = var_ct_fact_dn16;
        *var_ct_fact_dn17_slot = var_ct_fact_dn17;
        *var_ct_fact_dn18_slot = var_ct_fact_dn18;
        *var_ct_fact_dn19_slot = var_ct_fact_dn19;
        *var_ct_fact_dn20_slot = var_ct_fact_dn20;
        *var_ct_fact_dn5_slot = var_ct_fact_dn5;
        *var_ct_fact_dn6_slot = var_ct_fact_dn6;
        *var_ct_fact_dn7_slot = var_ct_fact_dn7;
        *var_ct_fact_dn8_slot = var_ct_fact_dn8;
        *var_ct_fact_rv_slot = var_ct_fact_rv;
        *var_dctg_slot = var_dctg;
        *var_dctg_dn12_slot = var_dctg_dn12;
        *var_dctg_dn13_slot = var_dctg_dn13;
        *var_dctg_dn14_slot = var_dctg_dn14;
        *var_dctg_dn15_slot = var_dctg_dn15;
        *var_dctg_dn16_slot = var_dctg_dn16;
        *var_dctg_dn17_slot = var_dctg_dn17;
        *var_dctg_dn18_slot = var_dctg_dn18;
        *var_dctg_dn19_slot = var_dctg_dn19;
        *var_dctg_dn20_slot = var_dctg_dn20;
        *var_dctg_dn5_slot = var_dctg_dn5;
        *var_dctg_dn6_slot = var_dctg_dn6;
        *var_dctg_dn7_slot = var_dctg_dn7;
        *var_dctg_dn8_slot = var_dctg_dn8;
        *var_dctg_rv_slot = var_dctg_rv;
        *var_delphib_slot = var_delphib;
        *var_delphib_dn12_slot = var_delphib_dn12;
        *var_delphib_dn13_slot = var_delphib_dn13;
        *var_delphib_dn14_slot = var_delphib_dn14;
        *var_delphib_dn15_slot = var_delphib_dn15;
        *var_delphib_dn16_slot = var_delphib_dn16;
        *var_delphib_dn17_slot = var_delphib_dn17;
        *var_delphib_dn18_slot = var_delphib_dn18;
        *var_delphib_dn19_slot = var_delphib_dn19;
        *var_delphib_dn20_slot = var_delphib_dn20;
        *var_delphib_dn5_slot = var_delphib_dn5;
        *var_delphib_dn6_slot = var_delphib_dn6;
        *var_delphib_dn7_slot = var_delphib_dn7;
        *var_delphib_dn8_slot = var_delphib_dn8;
        *var_delphib_rv_slot = var_delphib_rv;
        *var_delxb_slot = var_delxb;
        *var_delxb_dn12_slot = var_delxb_dn12;
        *var_delxb_dn13_slot = var_delxb_dn13;
        *var_delxb_dn14_slot = var_delxb_dn14;
        *var_delxb_dn15_slot = var_delxb_dn15;
        *var_delxb_dn16_slot = var_delxb_dn16;
        *var_delxb_dn17_slot = var_delxb_dn17;
        *var_delxb_dn18_slot = var_delxb_dn18;
        *var_delxb_dn19_slot = var_delxb_dn19;
        *var_delxb_dn20_slot = var_delxb_dn20;
        *var_delxb_dn5_slot = var_delxb_dn5;
        *var_delxb_dn6_slot = var_delxb_dn6;
        *var_delxb_dn7_slot = var_delxb_dn7;
        *var_delxb_dn8_slot = var_delxb_dn8;
        *var_delxb_rv_slot = var_delxb_rv;
        *var_dphit1_slot = var_dphit1;
        *var_dphit1_dn12_slot = var_dphit1_dn12;
        *var_dphit1_dn13_slot = var_dphit1_dn13;
        *var_dphit1_dn14_slot = var_dphit1_dn14;
        *var_dphit1_dn15_slot = var_dphit1_dn15;
        *var_dphit1_dn16_slot = var_dphit1_dn16;
        *var_dphit1_dn17_slot = var_dphit1_dn17;
        *var_dphit1_dn18_slot = var_dphit1_dn18;
        *var_dphit1_dn19_slot = var_dphit1_dn19;
        *var_dphit1_dn20_slot = var_dphit1_dn20;
        *var_dphit1_dn5_slot = var_dphit1_dn5;
        *var_dphit1_dn6_slot = var_dphit1_dn6;
        *var_dphit1_dn7_slot = var_dphit1_dn7;
        *var_dphit1_dn8_slot = var_dphit1_dn8;
        *var_dphit1_rv_slot = var_dphit1_rv;
        *var_gf_slot = var_gf;
        *var_gf2_slot = var_gf2;
        *var_gf2_dn12_slot = var_gf2_dn12;
        *var_gf2_dn13_slot = var_gf2_dn13;
        *var_gf2_dn14_slot = var_gf2_dn14;
        *var_gf2_dn15_slot = var_gf2_dn15;
        *var_gf2_dn16_slot = var_gf2_dn16;
        *var_gf2_dn17_slot = var_gf2_dn17;
        *var_gf2_dn18_slot = var_gf2_dn18;
        *var_gf2_dn19_slot = var_gf2_dn19;
        *var_gf2_dn20_slot = var_gf2_dn20;
        *var_gf2_dn5_slot = var_gf2_dn5;
        *var_gf2_dn6_slot = var_gf2_dn6;
        *var_gf2_dn7_slot = var_gf2_dn7;
        *var_gf2_dn8_slot = var_gf2_dn8;
        *var_gf2_rv_slot = var_gf2_rv;
        *var_gf_dn12_slot = var_gf_dn12;
        *var_gf_dn13_slot = var_gf_dn13;
        *var_gf_dn14_slot = var_gf_dn14;
        *var_gf_dn15_slot = var_gf_dn15;
        *var_gf_dn16_slot = var_gf_dn16;
        *var_gf_dn17_slot = var_gf_dn17;
        *var_gf_dn18_slot = var_gf_dn18;
        *var_gf_dn19_slot = var_gf_dn19;
        *var_gf_dn20_slot = var_gf_dn20;
        *var_gf_dn5_slot = var_gf_dn5;
        *var_gf_dn6_slot = var_gf_dn6;
        *var_gf_dn7_slot = var_gf_dn7;
        *var_gf_dn8_slot = var_gf_dn8;
        *var_gf_rv_slot = var_gf_rv;
        *var_guard1276_slot = var_guard1276;
        *var_guard1276_rv_slot = var_guard1276_rv;
        *var_guard1277_slot = var_guard1277;
        *var_guard1277_rv_slot = var_guard1277_rv;
        *var_guard1278_slot = var_guard1278;
        *var_guard1278_rv_slot = var_guard1278_rv;
        *var_guard1279_slot = var_guard1279;
        *var_guard1279_rv_slot = var_guard1279_rv;
        *var_inv_gf2_slot = var_inv_gf2;
        *var_inv_gf2_dn12_slot = var_inv_gf2_dn12;
        *var_inv_gf2_dn13_slot = var_inv_gf2_dn13;
        *var_inv_gf2_dn14_slot = var_inv_gf2_dn14;
        *var_inv_gf2_dn15_slot = var_inv_gf2_dn15;
        *var_inv_gf2_dn16_slot = var_inv_gf2_dn16;
        *var_inv_gf2_dn17_slot = var_inv_gf2_dn17;
        *var_inv_gf2_dn18_slot = var_inv_gf2_dn18;
        *var_inv_gf2_dn19_slot = var_inv_gf2_dn19;
        *var_inv_gf2_dn20_slot = var_inv_gf2_dn20;
        *var_inv_gf2_dn5_slot = var_inv_gf2_dn5;
        *var_inv_gf2_dn6_slot = var_inv_gf2_dn6;
        *var_inv_gf2_dn7_slot = var_inv_gf2_dn7;
        *var_inv_gf2_dn8_slot = var_inv_gf2_dn8;
        *var_inv_gf2_rv_slot = var_inv_gf2_rv;
        *var_inv_phit1_slot = var_inv_phit1;
        *var_inv_phit1_dn12_slot = var_inv_phit1_dn12;
        *var_inv_phit1_dn13_slot = var_inv_phit1_dn13;
        *var_inv_phit1_dn14_slot = var_inv_phit1_dn14;
        *var_inv_phit1_dn15_slot = var_inv_phit1_dn15;
        *var_inv_phit1_dn16_slot = var_inv_phit1_dn16;
        *var_inv_phit1_dn17_slot = var_inv_phit1_dn17;
        *var_inv_phit1_dn18_slot = var_inv_phit1_dn18;
        *var_inv_phit1_dn19_slot = var_inv_phit1_dn19;
        *var_inv_phit1_dn20_slot = var_inv_phit1_dn20;
        *var_inv_phit1_dn5_slot = var_inv_phit1_dn5;
        *var_inv_phit1_dn6_slot = var_inv_phit1_dn6;
        *var_inv_phit1_dn7_slot = var_inv_phit1_dn7;
        *var_inv_phit1_dn8_slot = var_inv_phit1_dn8;
        *var_inv_phit1_rv_slot = var_inv_phit1_rv;
        *var_nscr_slot = var_nscr;
        *var_nscr_dn12_slot = var_nscr_dn12;
        *var_nscr_dn13_slot = var_nscr_dn13;
        *var_nscr_dn14_slot = var_nscr_dn14;
        *var_nscr_dn15_slot = var_nscr_dn15;
        *var_nscr_dn16_slot = var_nscr_dn16;
        *var_nscr_dn17_slot = var_nscr_dn17;
        *var_nscr_dn18_slot = var_nscr_dn18;
        *var_nscr_dn19_slot = var_nscr_dn19;
        *var_nscr_dn20_slot = var_nscr_dn20;
        *var_nscr_dn5_slot = var_nscr_dn5;
        *var_nscr_dn6_slot = var_nscr_dn6;
        *var_nscr_dn7_slot = var_nscr_dn7;
        *var_nscr_dn8_slot = var_nscr_dn8;
        *var_nscr_rv_slot = var_nscr_rv;
        *var_phit1_slot = var_phit1;
        *var_phit1_dn12_slot = var_phit1_dn12;
        *var_phit1_dn13_slot = var_phit1_dn13;
        *var_phit1_dn14_slot = var_phit1_dn14;
        *var_phit1_dn15_slot = var_phit1_dn15;
        *var_phit1_dn16_slot = var_phit1_dn16;
        *var_phit1_dn17_slot = var_phit1_dn17;
        *var_phit1_dn18_slot = var_phit1_dn18;
        *var_phit1_dn19_slot = var_phit1_dn19;
        *var_phit1_dn20_slot = var_phit1_dn20;
        *var_phit1_dn5_slot = var_phit1_dn5;
        *var_phit1_dn6_slot = var_phit1_dn6;
        *var_phit1_dn7_slot = var_phit1_dn7;
        *var_phit1_dn8_slot = var_phit1_dn8;
        *var_phit1_rv_slot = var_phit1_rv;
        *var_phitct_slot = var_phitct;
        *var_phitct_dn12_slot = var_phitct_dn12;
        *var_phitct_dn13_slot = var_phitct_dn13;
        *var_phitct_dn14_slot = var_phitct_dn14;
        *var_phitct_dn15_slot = var_phitct_dn15;
        *var_phitct_dn16_slot = var_phitct_dn16;
        *var_phitct_dn17_slot = var_phitct_dn17;
        *var_phitct_dn18_slot = var_phitct_dn18;
        *var_phitct_dn19_slot = var_phitct_dn19;
        *var_phitct_dn20_slot = var_phitct_dn20;
        *var_phitct_dn5_slot = var_phitct_dn5;
        *var_phitct_dn6_slot = var_phitct_dn6;
        *var_phitct_dn7_slot = var_phitct_dn7;
        *var_phitct_dn8_slot = var_phitct_dn8;
        *var_phitct_rv_slot = var_phitct_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn12_slot = var_temp1_dn12;
        *var_temp1_dn13_slot = var_temp1_dn13;
        *var_temp1_dn14_slot = var_temp1_dn14;
        *var_temp1_dn15_slot = var_temp1_dn15;
        *var_temp1_dn16_slot = var_temp1_dn16;
        *var_temp1_dn17_slot = var_temp1_dn17;
        *var_temp1_dn18_slot = var_temp1_dn18;
        *var_temp1_dn19_slot = var_temp1_dn19;
        *var_temp1_dn20_slot = var_temp1_dn20;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn12_slot = var_temp2_dn12;
        *var_temp2_dn13_slot = var_temp2_dn13;
        *var_temp2_dn14_slot = var_temp2_dn14;
        *var_temp2_dn15_slot = var_temp2_dn15;
        *var_temp2_dn16_slot = var_temp2_dn16;
        *var_temp2_dn17_slot = var_temp2_dn17;
        *var_temp2_dn18_slot = var_temp2_dn18;
        *var_temp2_dn19_slot = var_temp2_dn19;
        *var_temp2_dn20_slot = var_temp2_dn20;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_ux_slot = var_ux;
        *var_ux_dn12_slot = var_ux_dn12;
        *var_ux_dn13_slot = var_ux_dn13;
        *var_ux_dn14_slot = var_ux_dn14;
        *var_ux_dn15_slot = var_ux_dn15;
        *var_ux_dn16_slot = var_ux_dn16;
        *var_ux_dn17_slot = var_ux_dn17;
        *var_ux_dn18_slot = var_ux_dn18;
        *var_ux_dn19_slot = var_ux_dn19;
        *var_ux_dn20_slot = var_ux_dn20;
        *var_ux_dn5_slot = var_ux_dn5;
        *var_ux_dn6_slot = var_ux_dn6;
        *var_ux_dn7_slot = var_ux_dn7;
        *var_ux_dn8_slot = var_ux_dn8;
        *var_ux_rv_slot = var_ux_rv;
        *var_vdsp_slot = var_vdsp;
        *var_vdsp_dn6_slot = var_vdsp_dn6;
        *var_vdsp_dn7_slot = var_vdsp_dn7;
        *var_vdsp_rv_slot = var_vdsp_rv;
        *var_xb_slot = var_xb;
        *var_xb_dn12_slot = var_xb_dn12;
        *var_xb_dn13_slot = var_xb_dn13;
        *var_xb_dn14_slot = var_xb_dn14;
        *var_xb_dn15_slot = var_xb_dn15;
        *var_xb_dn16_slot = var_xb_dn16;
        *var_xb_dn17_slot = var_xb_dn17;
        *var_xb_dn18_slot = var_xb_dn18;
        *var_xb_dn19_slot = var_xb_dn19;
        *var_xb_dn20_slot = var_xb_dn20;
        *var_xb_dn5_slot = var_xb_dn5;
        *var_xb_dn6_slot = var_xb_dn6;
        *var_xb_dn7_slot = var_xb_dn7;
        *var_xb_dn8_slot = var_xb_dn8;
        *var_xb_rv_slot = var_xb_rv;
        *var_xct_slot = var_xct;
        *var_xct_dn12_slot = var_xct_dn12;
        *var_xct_dn13_slot = var_xct_dn13;
        *var_xct_dn14_slot = var_xct_dn14;
        *var_xct_dn15_slot = var_xct_dn15;
        *var_xct_dn16_slot = var_xct_dn16;
        *var_xct_dn17_slot = var_xct_dn17;
        *var_xct_dn18_slot = var_xct_dn18;
        *var_xct_dn19_slot = var_xct_dn19;
        *var_xct_dn20_slot = var_xct_dn20;
        *var_xct_dn5_slot = var_xct_dn5;
        *var_xct_dn6_slot = var_xct_dn6;
        *var_xct_dn7_slot = var_xct_dn7;
        *var_xct_dn8_slot = var_xct_dn8;
        *var_xct_rv_slot = var_xct_rv;
        *var_xg_slot = var_xg;
        *var_xg_dn12_slot = var_xg_dn12;
        *var_xg_dn13_slot = var_xg_dn13;
        *var_xg_dn14_slot = var_xg_dn14;
        *var_xg_dn15_slot = var_xg_dn15;
        *var_xg_dn16_slot = var_xg_dn16;
        *var_xg_dn17_slot = var_xg_dn17;
        *var_xg_dn18_slot = var_xg_dn18;
        *var_xg_dn19_slot = var_xg_dn19;
        *var_xg_dn20_slot = var_xg_dn20;
        *var_xg_dn5_slot = var_xg_dn5;
        *var_xg_dn6_slot = var_xg_dn6;
        *var_xg_dn7_slot = var_xg_dn7;
        *var_xg_dn8_slot = var_xg_dn8;
        *var_xg_rv_slot = var_xg_rv;
        *var_xn_s_slot = var_xn_s;
        *var_xn_s_dn12_slot = var_xn_s_dn12;
        *var_xn_s_dn13_slot = var_xn_s_dn13;
        *var_xn_s_dn14_slot = var_xn_s_dn14;
        *var_xn_s_dn15_slot = var_xn_s_dn15;
        *var_xn_s_dn16_slot = var_xn_s_dn16;
        *var_xn_s_dn17_slot = var_xn_s_dn17;
        *var_xn_s_dn18_slot = var_xn_s_dn18;
        *var_xn_s_dn19_slot = var_xn_s_dn19;
        *var_xn_s_dn20_slot = var_xn_s_dn20;
        *var_xn_s_dn5_slot = var_xn_s_dn5;
        *var_xn_s_dn6_slot = var_xn_s_dn6;
        *var_xn_s_dn7_slot = var_xn_s_dn7;
        *var_xn_s_dn8_slot = var_xn_s_dn8;
        *var_xn_s_rv_slot = var_xn_s_rv;
        *var_xno_s_slot = var_xno_s;
        *var_xno_s_dn12_slot = var_xno_s_dn12;
        *var_xno_s_dn13_slot = var_xno_s_dn13;
        *var_xno_s_dn14_slot = var_xno_s_dn14;
        *var_xno_s_dn15_slot = var_xno_s_dn15;
        *var_xno_s_dn16_slot = var_xno_s_dn16;
        *var_xno_s_dn17_slot = var_xno_s_dn17;
        *var_xno_s_dn18_slot = var_xno_s_dn18;
        *var_xno_s_dn19_slot = var_xno_s_dn19;
        *var_xno_s_dn20_slot = var_xno_s_dn20;
        *var_xno_s_dn5_slot = var_xno_s_dn5;
        *var_xno_s_dn6_slot = var_xno_s_dn6;
        *var_xno_s_dn7_slot = var_xno_s_dn7;
        *var_xno_s_dn8_slot = var_xno_s_dn8;
        *var_xno_s_rv_slot = var_xno_s_rv;
    }
}
