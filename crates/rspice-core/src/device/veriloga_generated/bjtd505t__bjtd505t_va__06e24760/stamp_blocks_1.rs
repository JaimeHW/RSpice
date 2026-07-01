#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv3 = ctx.node_voltage(nodes[3]);
        let assign00_e553: f64 = if p.p3 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign00_e553;
        locals.var_guard1_rv = 0.0;

        let (assign10_e557,) = {
    if (locals.var_guard1 != 0.0) {
        (70300000.0,)
    } else {
        (locals.var_an,)
    }
};
        locals.var_an = assign10_e557;
        locals.var_an_rv = 0.0;

        let (assign20_e561,) = {
    if (locals.var_guard1 != 0.0) {
        (123000000.0,)
    } else {
        (locals.var_bn,)
    }
};
        locals.var_bn = assign20_e561;
        locals.var_bn_rv = 0.0;

        let (assign30_e566,) = {
    if (locals.var_guard1 == 0.0) {
        (158000000.0,)
    } else {
        (locals.var_an,)
    }
};
        locals.var_an = assign30_e566;
        locals.var_an_rv = 0.0;

        let (assign40_e571,) = {
    if (locals.var_guard1 == 0.0) {
        (204000000.0,)
    } else {
        (locals.var_bn,)
    }
};
        locals.var_bn = assign40_e571;
        locals.var_bn_rv = 0.0;

        let assign50_e574: f64 = (1.0 - p.p32);
        locals.var_xext1 = assign50_e574;
        locals.var_xext1_rv = 0.0;

        let assign60_e577: f64 = (p.p4 + 273.15);
        locals.var_trk = assign60_e577;
        locals.var_trk_rv = 0.0;

        let assign70_e578: f64 = ctx_temp;
        let assign70_e580: f64 = (assign70_e578 + p.p0);
        locals.var_tamb = assign70_e580;
        locals.var_tamb_rv = 0.0;

        let assign90_e586: f64 = if p.p141 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2 = assign90_e586;
        locals.var_guard2_rv = 0.0;

        let (assign100_e590,) = {
    if (locals.var_guard2 != 0.0) {
        (1e-12,)
    } else {
        (locals.var_minr,)
    }
};
        locals.var_minr = assign100_e590;
        locals.var_minr_rv = 0.0;

        let (assign110_e595,) = {
    if (locals.var_guard2 == 0.0) {
        (p.p141,)
    } else {
        (locals.var_minr,)
    }
};
        locals.var_minr = assign110_e595;
        locals.var_minr_rv = 0.0;

        let assign120_e598: f64 = (locals.var_minr * p.p1);
        locals.var_minr_m = assign120_e598;
        locals.var_minr_m_rv = 0.0;

        locals.var_eps_nf = 0.001;
        locals.var_eps_nf_rv = 0.0;

        locals.var_eps_bavl_t = 0.001;
        locals.var_eps_bavl_t_rv = 0.0;

        let assign160_e607: f64 = (2.0 - p.p66);
        let assign160_e608: f64 = (2.0_f64).powf(assign160_e607);
        locals.var_pow2_2m_pe = assign160_e608;
        locals.var_pow2_2m_pe_rv = 0.0;

        let assign180_e615: f64 = (p.p114 * locals.var_trk);
        let assign180_e617: f64 = (assign180_e615 * locals.var_trk);
        let assign180_e620: f64 = (locals.var_trk + p.p115);
        let assign180_e621: f64 = (assign180_e617 / assign180_e620);
        let assign180_e622: f64 = (p.p113 + assign180_e621);
        let assign180_e624: f64 = (assign180_e622 - 0.05);
        let assign180_e626: f64 = (assign180_e624 / 0.1);
        locals.var_dxa = assign180_e626;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign190_e630: f64 = (p.p114 * locals.var_trk);
        let assign190_e632: f64 = (assign190_e630 * locals.var_trk);
        let assign190_e635: f64 = (locals.var_trk + p.p115);
        let assign190_e636: f64 = (assign190_e632 / assign190_e635);
        let assign190_e637: f64 = (p.p113 + assign190_e636);
        let assign190_e639: f64 = if assign190_e637 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard3 = assign190_e639;
        locals.var_guard3_rv = 0.0;

        let (assign200_e651, assign200_e651_d_n0, assign200_e651_d_n1, assign200_e651_d_n3, assign200_e651_d_n4, assign200_e651_d_n5, assign200_e651_d_n6, assign200_e651_d_n7, assign200_e651_d_n8, assign200_e651_d_n9, assign200_e651_d_n10,) = {
    if (locals.var_guard3 != 0.0) {
        let assign200_e645: f64 = (locals.var_dxa).exp();
        let assign200_e646: f64 = (1.0 + assign200_e645);
        let assign200_e647: f64 = (assign200_e646).ln();
        let assign200_e648: f64 = (0.1 * assign200_e647);
        let assign200_e649: f64 = (0.05 + assign200_e648);
        (assign200_e649, (0.1 * ((assign200_e645 * locals.var_dxa_dn0) / assign200_e646)), (0.1 * ((assign200_e645 * locals.var_dxa_dn1) / assign200_e646)), (0.1 * ((assign200_e645 * locals.var_dxa_dn3) / assign200_e646)), (0.1 * ((assign200_e645 * locals.var_dxa_dn4) / assign200_e646)), (0.1 * ((assign200_e645 * locals.var_dxa_dn5) / assign200_e646)), (0.1 * ((assign200_e645 * locals.var_dxa_dn6) / assign200_e646)), (0.1 * ((assign200_e645 * locals.var_dxa_dn7) / assign200_e646)), (0.1 * ((assign200_e645 * locals.var_dxa_dn8) / assign200_e646)), (0.1 * ((assign200_e645 * locals.var_dxa_dn9) / assign200_e646)), (0.1 * ((assign200_e645 * locals.var_dxa_dn10) / assign200_e646)),)
    } else {
        (locals.var_vgzebok, locals.var_vgzebok_dn0, locals.var_vgzebok_dn1, locals.var_vgzebok_dn3, locals.var_vgzebok_dn4, locals.var_vgzebok_dn5, locals.var_vgzebok_dn6, locals.var_vgzebok_dn7, locals.var_vgzebok_dn8, locals.var_vgzebok_dn9, locals.var_vgzebok_dn10,)
    }
};
        locals.var_vgzebok = assign200_e651;
        locals.var_vgzebok_dn0 = assign200_e651_d_n0;
        locals.var_vgzebok_dn1 = assign200_e651_d_n1;
        locals.var_vgzebok_dn3 = assign200_e651_d_n3;
        locals.var_vgzebok_dn4 = assign200_e651_d_n4;
        locals.var_vgzebok_dn5 = assign200_e651_d_n5;
        locals.var_vgzebok_dn6 = assign200_e651_d_n6;
        locals.var_vgzebok_dn7 = assign200_e651_d_n7;
        locals.var_vgzebok_dn8 = assign200_e651_d_n8;
        locals.var_vgzebok_dn9 = assign200_e651_d_n9;
        locals.var_vgzebok_dn10 = assign200_e651_d_n10;
        locals.var_vgzebok_rv = 0.0;

        let (assign210_e675, assign210_e675_d_n0, assign210_e675_d_n1, assign210_e675_d_n3, assign210_e675_d_n4, assign210_e675_d_n5, assign210_e675_d_n6, assign210_e675_d_n7, assign210_e675_d_n8, assign210_e675_d_n9, assign210_e675_d_n10,) = {
    if (locals.var_guard3 == 0.0) {
        let assign210_e657: f64 = (p.p114 * locals.var_trk);
        let assign210_e659: f64 = (assign210_e657 * locals.var_trk);
        let assign210_e662: f64 = (locals.var_trk + p.p115);
        let assign210_e663: f64 = (assign210_e659 / assign210_e662);
        let assign210_e664: f64 = (p.p113 + assign210_e663);
        let assign210_e668: f64 = (-locals.var_dxa);
        let assign210_e669: f64 = (assign210_e668).exp();
        let assign210_e670: f64 = (1.0 + assign210_e669);
        let assign210_e671: f64 = (assign210_e670).ln();
        let assign210_e672: f64 = (0.1 * assign210_e671);
        let assign210_e673: f64 = (assign210_e664 + assign210_e672);
        (assign210_e673, (0.1 * ((assign210_e669 * (-locals.var_dxa_dn0)) / assign210_e670)), (0.1 * ((assign210_e669 * (-locals.var_dxa_dn1)) / assign210_e670)), (0.1 * ((assign210_e669 * (-locals.var_dxa_dn3)) / assign210_e670)), (0.1 * ((assign210_e669 * (-locals.var_dxa_dn4)) / assign210_e670)), (0.1 * ((assign210_e669 * (-locals.var_dxa_dn5)) / assign210_e670)), (0.1 * ((assign210_e669 * (-locals.var_dxa_dn6)) / assign210_e670)), (0.1 * ((assign210_e669 * (-locals.var_dxa_dn7)) / assign210_e670)), (0.1 * ((assign210_e669 * (-locals.var_dxa_dn8)) / assign210_e670)), (0.1 * ((assign210_e669 * (-locals.var_dxa_dn9)) / assign210_e670)), (0.1 * ((assign210_e669 * (-locals.var_dxa_dn10)) / assign210_e670)),)
    } else {
        (locals.var_vgzebok, locals.var_vgzebok_dn0, locals.var_vgzebok_dn1, locals.var_vgzebok_dn3, locals.var_vgzebok_dn4, locals.var_vgzebok_dn5, locals.var_vgzebok_dn6, locals.var_vgzebok_dn7, locals.var_vgzebok_dn8, locals.var_vgzebok_dn9, locals.var_vgzebok_dn10,)
    }
};
        locals.var_vgzebok = assign210_e675;
        locals.var_vgzebok_dn0 = assign210_e675_d_n0;
        locals.var_vgzebok_dn1 = assign210_e675_d_n1;
        locals.var_vgzebok_dn3 = assign210_e675_d_n3;
        locals.var_vgzebok_dn4 = assign210_e675_d_n4;
        locals.var_vgzebok_dn5 = assign210_e675_d_n5;
        locals.var_vgzebok_dn6 = assign210_e675_d_n6;
        locals.var_vgzebok_dn7 = assign210_e675_d_n7;
        locals.var_vgzebok_dn8 = assign210_e675_d_n8;
        locals.var_vgzebok_dn9 = assign210_e675_d_n9;
        locals.var_vgzebok_dn10 = assign210_e675_d_n10;
        locals.var_vgzebok_rv = 0.0;

        locals.var_vgzeb_tr = p.p113;
        locals.var_vgzeb_tr_rv = 0.0;

        let assign230_e679: f64 = (1.0 / locals.var_vgzeb_tr);
        locals.var_inv_vgzeb_tr = assign230_e679;
        locals.var_inv_vgzeb_tr_rv = 0.0;

        locals.var_vdc_zener = p.p70;
        locals.var_vdc_zener_rv = 0.0;

        locals.var_pc_zener = p.p71;
        locals.var_pc_zener_rv = 0.0;

        let assign270_e688: f64 = (2.0 - locals.var_pc_zener);
        let assign270_e689: f64 = (2.0_f64).powf(assign270_e688);
        locals.var_pow2_2m_pc = assign270_e689;
        locals.var_pow2_2m_pc_rv = 0.0;

        let assign290_e696: f64 = (p.p117 * locals.var_trk);
        let assign290_e698: f64 = (assign290_e696 * locals.var_trk);
        let assign290_e701: f64 = (locals.var_trk + p.p118);
        let assign290_e702: f64 = (assign290_e698 / assign290_e701);
        let assign290_e703: f64 = (p.p116 + assign290_e702);
        let assign290_e705: f64 = (assign290_e703 - 0.05);
        let assign290_e707: f64 = (assign290_e705 / 0.1);
        locals.var_dxa = assign290_e707;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign300_e711: f64 = (p.p117 * locals.var_trk);
        let assign300_e713: f64 = (assign300_e711 * locals.var_trk);
        let assign300_e716: f64 = (locals.var_trk + p.p118);
        let assign300_e717: f64 = (assign300_e713 / assign300_e716);
        let assign300_e718: f64 = (p.p116 + assign300_e717);
        let assign300_e720: f64 = if assign300_e718 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard4 = assign300_e720;
        locals.var_guard4_rv = 0.0;

        let (assign310_e732, assign310_e732_d_n0, assign310_e732_d_n1, assign310_e732_d_n3, assign310_e732_d_n4, assign310_e732_d_n5, assign310_e732_d_n6, assign310_e732_d_n7, assign310_e732_d_n8, assign310_e732_d_n9, assign310_e732_d_n10,) = {
    if (locals.var_guard4 != 0.0) {
        let assign310_e726: f64 = (locals.var_dxa).exp();
        let assign310_e727: f64 = (1.0 + assign310_e726);
        let assign310_e728: f64 = (assign310_e727).ln();
        let assign310_e729: f64 = (0.1 * assign310_e728);
        let assign310_e730: f64 = (0.05 + assign310_e729);
        (assign310_e730, (0.1 * ((assign310_e726 * locals.var_dxa_dn0) / assign310_e727)), (0.1 * ((assign310_e726 * locals.var_dxa_dn1) / assign310_e727)), (0.1 * ((assign310_e726 * locals.var_dxa_dn3) / assign310_e727)), (0.1 * ((assign310_e726 * locals.var_dxa_dn4) / assign310_e727)), (0.1 * ((assign310_e726 * locals.var_dxa_dn5) / assign310_e727)), (0.1 * ((assign310_e726 * locals.var_dxa_dn6) / assign310_e727)), (0.1 * ((assign310_e726 * locals.var_dxa_dn7) / assign310_e727)), (0.1 * ((assign310_e726 * locals.var_dxa_dn8) / assign310_e727)), (0.1 * ((assign310_e726 * locals.var_dxa_dn9) / assign310_e727)), (0.1 * ((assign310_e726 * locals.var_dxa_dn10) / assign310_e727)),)
    } else {
        (locals.var_vgzcbok, locals.var_vgzcbok_dn0, locals.var_vgzcbok_dn1, locals.var_vgzcbok_dn3, locals.var_vgzcbok_dn4, locals.var_vgzcbok_dn5, locals.var_vgzcbok_dn6, locals.var_vgzcbok_dn7, locals.var_vgzcbok_dn8, locals.var_vgzcbok_dn9, locals.var_vgzcbok_dn10,)
    }
};
        locals.var_vgzcbok = assign310_e732;
        locals.var_vgzcbok_dn0 = assign310_e732_d_n0;
        locals.var_vgzcbok_dn1 = assign310_e732_d_n1;
        locals.var_vgzcbok_dn3 = assign310_e732_d_n3;
        locals.var_vgzcbok_dn4 = assign310_e732_d_n4;
        locals.var_vgzcbok_dn5 = assign310_e732_d_n5;
        locals.var_vgzcbok_dn6 = assign310_e732_d_n6;
        locals.var_vgzcbok_dn7 = assign310_e732_d_n7;
        locals.var_vgzcbok_dn8 = assign310_e732_d_n8;
        locals.var_vgzcbok_dn9 = assign310_e732_d_n9;
        locals.var_vgzcbok_dn10 = assign310_e732_d_n10;
        locals.var_vgzcbok_rv = 0.0;

        let (assign320_e756, assign320_e756_d_n0, assign320_e756_d_n1, assign320_e756_d_n3, assign320_e756_d_n4, assign320_e756_d_n5, assign320_e756_d_n6, assign320_e756_d_n7, assign320_e756_d_n8, assign320_e756_d_n9, assign320_e756_d_n10,) = {
    if (locals.var_guard4 == 0.0) {
        let assign320_e738: f64 = (p.p117 * locals.var_trk);
        let assign320_e740: f64 = (assign320_e738 * locals.var_trk);
        let assign320_e743: f64 = (locals.var_trk + p.p118);
        let assign320_e744: f64 = (assign320_e740 / assign320_e743);
        let assign320_e745: f64 = (p.p116 + assign320_e744);
        let assign320_e749: f64 = (-locals.var_dxa);
        let assign320_e750: f64 = (assign320_e749).exp();
        let assign320_e751: f64 = (1.0 + assign320_e750);
        let assign320_e752: f64 = (assign320_e751).ln();
        let assign320_e753: f64 = (0.1 * assign320_e752);
        let assign320_e754: f64 = (assign320_e745 + assign320_e753);
        (assign320_e754, (0.1 * ((assign320_e750 * (-locals.var_dxa_dn0)) / assign320_e751)), (0.1 * ((assign320_e750 * (-locals.var_dxa_dn1)) / assign320_e751)), (0.1 * ((assign320_e750 * (-locals.var_dxa_dn3)) / assign320_e751)), (0.1 * ((assign320_e750 * (-locals.var_dxa_dn4)) / assign320_e751)), (0.1 * ((assign320_e750 * (-locals.var_dxa_dn5)) / assign320_e751)), (0.1 * ((assign320_e750 * (-locals.var_dxa_dn6)) / assign320_e751)), (0.1 * ((assign320_e750 * (-locals.var_dxa_dn7)) / assign320_e751)), (0.1 * ((assign320_e750 * (-locals.var_dxa_dn8)) / assign320_e751)), (0.1 * ((assign320_e750 * (-locals.var_dxa_dn9)) / assign320_e751)), (0.1 * ((assign320_e750 * (-locals.var_dxa_dn10)) / assign320_e751)),)
    } else {
        (locals.var_vgzcbok, locals.var_vgzcbok_dn0, locals.var_vgzcbok_dn1, locals.var_vgzcbok_dn3, locals.var_vgzcbok_dn4, locals.var_vgzcbok_dn5, locals.var_vgzcbok_dn6, locals.var_vgzcbok_dn7, locals.var_vgzcbok_dn8, locals.var_vgzcbok_dn9, locals.var_vgzcbok_dn10,)
    }
};
        locals.var_vgzcbok = assign320_e756;
        locals.var_vgzcbok_dn0 = assign320_e756_d_n0;
        locals.var_vgzcbok_dn1 = assign320_e756_d_n1;
        locals.var_vgzcbok_dn3 = assign320_e756_d_n3;
        locals.var_vgzcbok_dn4 = assign320_e756_d_n4;
        locals.var_vgzcbok_dn5 = assign320_e756_d_n5;
        locals.var_vgzcbok_dn6 = assign320_e756_d_n6;
        locals.var_vgzcbok_dn7 = assign320_e756_d_n7;
        locals.var_vgzcbok_dn8 = assign320_e756_d_n8;
        locals.var_vgzcbok_dn9 = assign320_e756_d_n9;
        locals.var_vgzcbok_dn10 = assign320_e756_d_n10;
        locals.var_vgzcbok_rv = 0.0;

        locals.var_vgzcb_tr = p.p116;
        locals.var_vgzcb_tr_rv = 0.0;

        let assign340_e760: f64 = (1.0 / locals.var_vgzcb_tr);
        locals.var_inv_vgzcb_tr = assign340_e760;
        locals.var_inv_vgzcb_tr_rv = 0.0;

        locals.var_fex = 1.0;
        locals.var_fex_dn0 = 0.0;
        locals.var_fex_dn1 = 0.0;
        locals.var_fex_dn3 = 0.0;
        locals.var_fex_dn4 = 0.0;
        locals.var_fex_dn5 = 0.0;
        locals.var_fex_dn6 = 0.0;
        locals.var_fex_dn7 = 0.0;
        locals.var_fex_dn8 = 0.0;
        locals.var_fex_dn9 = 0.0;
        locals.var_fex_dn10 = 0.0;
        locals.var_fex_rv = 0.0;

        locals.var_gem = 0.0;
        locals.var_gem_dn0 = 0.0;
        locals.var_gem_dn1 = 0.0;
        locals.var_gem_dn3 = 0.0;
        locals.var_gem_dn4 = 0.0;
        locals.var_gem_dn5 = 0.0;
        locals.var_gem_dn6 = 0.0;
        locals.var_gem_dn7 = 0.0;
        locals.var_gem_dn8 = 0.0;
        locals.var_gem_dn9 = 0.0;
        locals.var_gem_dn10 = 0.0;
        locals.var_gem_rv = 0.0;

        locals.var_xqex = 0.0;
        locals.var_xqex_dn0 = 0.0;
        locals.var_xqex_dn1 = 0.0;
        locals.var_xqex_dn3 = 0.0;
        locals.var_xqex_dn4 = 0.0;
        locals.var_xqex_dn5 = 0.0;
        locals.var_xqex_dn6 = 0.0;
        locals.var_xqex_dn7 = 0.0;
        locals.var_xqex_dn8 = 0.0;
        locals.var_xqex_dn9 = 0.0;
        locals.var_xqex_dn10 = 0.0;
        locals.var_xqex_rv = 0.0;

        locals.var_qb1b2 = 0.0;
        locals.var_qb1b2_dn0 = 0.0;
        locals.var_qb1b2_dn1 = 0.0;
        locals.var_qb1b2_dn3 = 0.0;
        locals.var_qb1b2_dn4 = 0.0;
        locals.var_qb1b2_dn5 = 0.0;
        locals.var_qb1b2_dn6 = 0.0;
        locals.var_qb1b2_dn7 = 0.0;
        locals.var_qb1b2_dn8 = 0.0;
        locals.var_qb1b2_dn9 = 0.0;
        locals.var_qb1b2_dn10 = 0.0;
        locals.var_qb1b2_rv = 0.0;

        locals.var_ibi_t = 0.0;
        locals.var_ibi_t_dn3 = 0.0;
        locals.var_ibi_t_rv = 0.0;

        locals.var_tki = (nv3 - 0.0);
        locals.var_tki_dn3 = 1.0;
        locals.var_tki_rv = 0.0;

        let assign510_e785: f64 = if locals.var_tki < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard5 = assign510_e785;
        locals.var_guard5_rv = 0.0;

        let (assign520_e793, assign520_e793_d_n3,) = {
    if (locals.var_guard5 != 0.0) {
        let assign520_e789: f64 = (1.0 - locals.var_tki);
        let assign520_e790: f64 = (assign520_e789).ln();
        let assign520_e791: f64 = (-assign520_e790);
        (assign520_e791, (-((-locals.var_tki_dn3) / assign520_e789)),)
    } else {
        (locals.var_tki, locals.var_tki_dn3,)
    }
};
        locals.var_tki = assign520_e793;
        locals.var_tki_dn3 = assign520_e793_d_n3;
        locals.var_tki_rv = 0.0;

        let assign530_e796: f64 = if locals.var_tki < p.p124 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign530_e796;
        locals.var_guard6_rv = 0.0;

        let (assign540_e800, assign540_e800_d_n3,) = {
    if (locals.var_guard6 != 0.0) {
        (locals.var_tki, locals.var_tki_dn3,)
    } else {
        (locals.var_vdt, locals.var_vdt_dn3,)
    }
};
        locals.var_vdt = assign540_e800;
        locals.var_vdt_dn3 = assign540_e800_d_n3;
        locals.var_vdt_rv = 0.0;

        let (assign550_e812, assign550_e812_d_n3,) = {
    if (locals.var_guard6 == 0.0) {
        let assign550_e807: f64 = (locals.var_tki - p.p124);
        let assign550_e808: f64 = (1.0 + assign550_e807);
        let assign550_e809: f64 = (assign550_e808).ln();
        let assign550_e810: f64 = (p.p124 + assign550_e809);
        (assign550_e810, (locals.var_tki_dn3 / assign550_e808),)
    } else {
        (locals.var_vdt, locals.var_vdt_dn3,)
    }
};
        locals.var_vdt = assign550_e812;
        locals.var_vdt_dn3 = assign550_e812_d_n3;
        locals.var_vdt_rv = 0.0;

        let assign560_e815: f64 = (locals.var_tamb + locals.var_vdt);
        locals.var_tk = assign560_e815;
        locals.var_tk_dn3 = locals.var_vdt_dn3;
        locals.var_tk_rv = 0.0;

        let assign570_e818: f64 = (locals.var_tk / locals.var_trk);
        locals.var_tn = assign570_e818;
        locals.var_tn_dn3 = (locals.var_tk_dn3 / locals.var_trk);
        locals.var_tn_rv = 0.0;

        let assign580_e821: f64 = (8.617086918058125e-5 * locals.var_tk);
        locals.var_vt = assign580_e821;
        locals.var_vt_dn3 = (8.617086918058125e-5 * locals.var_tk_dn3);
        locals.var_vt_rv = 0.0;

        let assign590_e824: f64 = (8.617086918058125e-5 * locals.var_trk);
        locals.var_vtr = assign590_e824;
        locals.var_vtr_rv = 0.0;

        let assign600_e827: f64 = (1.0 / locals.var_vt);
        locals.var_vtinv = assign600_e827;
        locals.var_vtinv_dn3 = (-(locals.var_vt_dn3 / (locals.var_vt * locals.var_vt)));
        locals.var_vtinv_rv = 0.0;

        let assign610_e830: f64 = (1.0 / locals.var_vtr);
        locals.var_vtrinv = assign610_e830;
        locals.var_vtrinv_rv = 0.0;

        let assign620_e833: f64 = (locals.var_vtinv - locals.var_vtrinv);
        locals.var_vdtinv = assign620_e833;
        locals.var_vdtinv_dn3 = locals.var_vtinv_dn3;
        locals.var_vdtinv_rv = 0.0;

        let assign630_e836: f64 = (locals.var_tk - locals.var_trk);
        locals.var_dt = assign630_e836;
        locals.var_dt_dn3 = locals.var_tk_dn3;
        locals.var_dt_rv = 0.0;

        let assign640_e838: f64 = (locals.var_tn).ln();
        locals.var_lntn = assign640_e838;
        locals.var_lntn_dn3 = (locals.var_tn_dn3 / locals.var_tn);
        locals.var_lntn_rv = 0.0;

        let assign650_e842: f64 = (p.p114 * locals.var_tk);
        let assign650_e844: f64 = (assign650_e842 * locals.var_tk);
        let assign650_e847: f64 = (locals.var_tk + p.p115);
        let assign650_e848: f64 = (assign650_e844 / assign650_e847);
        let assign650_e849: f64 = (locals.var_vgzebok - assign650_e848);
        let assign650_e851: f64 = (assign650_e849 - 0.05);
        let assign650_e853: f64 = (assign650_e851 / 0.1);
        locals.var_dxa = assign650_e853;
        locals.var_dxa_dn0 = (locals.var_vgzebok_dn0 / 0.1);
        locals.var_dxa_dn1 = (locals.var_vgzebok_dn1 / 0.1);
        locals.var_dxa_dn3 = ((locals.var_vgzebok_dn3 - ((((((p.p114 * locals.var_tk_dn3) * locals.var_tk) + (assign650_e842 * locals.var_tk_dn3)) * assign650_e847) - (assign650_e844 * locals.var_tk_dn3)) / (assign650_e847 * assign650_e847))) / 0.1);
        locals.var_dxa_dn4 = (locals.var_vgzebok_dn4 / 0.1);
        locals.var_dxa_dn5 = (locals.var_vgzebok_dn5 / 0.1);
        locals.var_dxa_dn6 = (locals.var_vgzebok_dn6 / 0.1);
        locals.var_dxa_dn7 = (locals.var_vgzebok_dn7 / 0.1);
        locals.var_dxa_dn8 = (locals.var_vgzebok_dn8 / 0.1);
        locals.var_dxa_dn9 = (locals.var_vgzebok_dn9 / 0.1);
        locals.var_dxa_dn10 = (locals.var_vgzebok_dn10 / 0.1);
        locals.var_dxa_rv = 0.0;

        let assign660_e857: f64 = (p.p114 * locals.var_tk);
        let assign660_e859: f64 = (assign660_e857 * locals.var_tk);
        let assign660_e862: f64 = (locals.var_tk + p.p115);
        let assign660_e863: f64 = (assign660_e859 / assign660_e862);
        let assign660_e864: f64 = (locals.var_vgzebok - assign660_e863);
        let assign660_e866: f64 = if assign660_e864 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard7 = assign660_e866;
        locals.var_guard7_rv = 0.0;

        let (assign670_e878, assign670_e878_d_n0, assign670_e878_d_n1, assign670_e878_d_n3, assign670_e878_d_n4, assign670_e878_d_n5, assign670_e878_d_n6, assign670_e878_d_n7, assign670_e878_d_n8, assign670_e878_d_n9, assign670_e878_d_n10,) = {
    if (locals.var_guard7 != 0.0) {
        let assign670_e872: f64 = (locals.var_dxa).exp();
        let assign670_e873: f64 = (1.0 + assign670_e872);
        let assign670_e874: f64 = (assign670_e873).ln();
        let assign670_e875: f64 = (0.1 * assign670_e874);
        let assign670_e876: f64 = (0.05 + assign670_e875);
        (assign670_e876, (0.1 * ((assign670_e872 * locals.var_dxa_dn0) / assign670_e873)), (0.1 * ((assign670_e872 * locals.var_dxa_dn1) / assign670_e873)), (0.1 * ((assign670_e872 * locals.var_dxa_dn3) / assign670_e873)), (0.1 * ((assign670_e872 * locals.var_dxa_dn4) / assign670_e873)), (0.1 * ((assign670_e872 * locals.var_dxa_dn5) / assign670_e873)), (0.1 * ((assign670_e872 * locals.var_dxa_dn6) / assign670_e873)), (0.1 * ((assign670_e872 * locals.var_dxa_dn7) / assign670_e873)), (0.1 * ((assign670_e872 * locals.var_dxa_dn8) / assign670_e873)), (0.1 * ((assign670_e872 * locals.var_dxa_dn9) / assign670_e873)), (0.1 * ((assign670_e872 * locals.var_dxa_dn10) / assign670_e873)),)
    } else {
        (locals.var_vgzeb_t, locals.var_vgzeb_t_dn0, locals.var_vgzeb_t_dn1, locals.var_vgzeb_t_dn3, locals.var_vgzeb_t_dn4, locals.var_vgzeb_t_dn5, locals.var_vgzeb_t_dn6, locals.var_vgzeb_t_dn7, locals.var_vgzeb_t_dn8, locals.var_vgzeb_t_dn9, locals.var_vgzeb_t_dn10,)
    }
};
        locals.var_vgzeb_t = assign670_e878;
        locals.var_vgzeb_t_dn0 = assign670_e878_d_n0;
        locals.var_vgzeb_t_dn1 = assign670_e878_d_n1;
        locals.var_vgzeb_t_dn3 = assign670_e878_d_n3;
        locals.var_vgzeb_t_dn4 = assign670_e878_d_n4;
        locals.var_vgzeb_t_dn5 = assign670_e878_d_n5;
        locals.var_vgzeb_t_dn6 = assign670_e878_d_n6;
        locals.var_vgzeb_t_dn7 = assign670_e878_d_n7;
        locals.var_vgzeb_t_dn8 = assign670_e878_d_n8;
        locals.var_vgzeb_t_dn9 = assign670_e878_d_n9;
        locals.var_vgzeb_t_dn10 = assign670_e878_d_n10;
        locals.var_vgzeb_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign680_e902, assign680_e902_d_n0, assign680_e902_d_n1, assign680_e902_d_n3, assign680_e902_d_n4, assign680_e902_d_n5, assign680_e902_d_n6, assign680_e902_d_n7, assign680_e902_d_n8, assign680_e902_d_n9, assign680_e902_d_n10,) = {
    if (locals.var_guard7 == 0.0) {
        let assign680_e884: f64 = (p.p114 * locals.var_tk);
        let assign680_e886: f64 = (assign680_e884 * locals.var_tk);
        let assign680_e889: f64 = (locals.var_tk + p.p115);
        let assign680_e890: f64 = (assign680_e886 / assign680_e889);
        let assign680_e891: f64 = (locals.var_vgzebok - assign680_e890);
        let assign680_e895: f64 = (-locals.var_dxa);
        let assign680_e896: f64 = (assign680_e895).exp();
        let assign680_e897: f64 = (1.0 + assign680_e896);
        let assign680_e898: f64 = (assign680_e897).ln();
        let assign680_e899: f64 = (0.1 * assign680_e898);
        let assign680_e900: f64 = (assign680_e891 + assign680_e899);
        (assign680_e900, (locals.var_vgzebok_dn0 + (0.1 * ((assign680_e896 * (-locals.var_dxa_dn0)) / assign680_e897))), (locals.var_vgzebok_dn1 + (0.1 * ((assign680_e896 * (-locals.var_dxa_dn1)) / assign680_e897))), ((locals.var_vgzebok_dn3 - ((((((p.p114 * locals.var_tk_dn3) * locals.var_tk) + (assign680_e884 * locals.var_tk_dn3)) * assign680_e889) - (assign680_e886 * locals.var_tk_dn3)) / (assign680_e889 * assign680_e889))) + (0.1 * ((assign680_e896 * (-locals.var_dxa_dn3)) / assign680_e897))), (locals.var_vgzebok_dn4 + (0.1 * ((assign680_e896 * (-locals.var_dxa_dn4)) / assign680_e897))), (locals.var_vgzebok_dn5 + (0.1 * ((assign680_e896 * (-locals.var_dxa_dn5)) / assign680_e897))), (locals.var_vgzebok_dn6 + (0.1 * ((assign680_e896 * (-locals.var_dxa_dn6)) / assign680_e897))), (locals.var_vgzebok_dn7 + (0.1 * ((assign680_e896 * (-locals.var_dxa_dn7)) / assign680_e897))), (locals.var_vgzebok_dn8 + (0.1 * ((assign680_e896 * (-locals.var_dxa_dn8)) / assign680_e897))), (locals.var_vgzebok_dn9 + (0.1 * ((assign680_e896 * (-locals.var_dxa_dn9)) / assign680_e897))), (locals.var_vgzebok_dn10 + (0.1 * ((assign680_e896 * (-locals.var_dxa_dn10)) / assign680_e897))),)
    } else {
        (locals.var_vgzeb_t, locals.var_vgzeb_t_dn0, locals.var_vgzeb_t_dn1, locals.var_vgzeb_t_dn3, locals.var_vgzeb_t_dn4, locals.var_vgzeb_t_dn5, locals.var_vgzeb_t_dn6, locals.var_vgzeb_t_dn7, locals.var_vgzeb_t_dn8, locals.var_vgzeb_t_dn9, locals.var_vgzeb_t_dn10,)
    }
};
        locals.var_vgzeb_t = assign680_e902;
        locals.var_vgzeb_t_dn0 = assign680_e902_d_n0;
        locals.var_vgzeb_t_dn1 = assign680_e902_d_n1;
        locals.var_vgzeb_t_dn3 = assign680_e902_d_n3;
        locals.var_vgzeb_t_dn4 = assign680_e902_d_n4;
        locals.var_vgzeb_t_dn5 = assign680_e902_d_n5;
        locals.var_vgzeb_t_dn6 = assign680_e902_d_n6;
        locals.var_vgzeb_t_dn7 = assign680_e902_d_n7;
        locals.var_vgzeb_t_dn8 = assign680_e902_d_n8;
        locals.var_vgzeb_t_dn9 = assign680_e902_d_n9;
        locals.var_vgzeb_t_dn10 = assign680_e902_d_n10;
        locals.var_vgzeb_t_rv = 0.0;

        let assign690_e906: f64 = (p.p117 * locals.var_tk);
        let assign690_e908: f64 = (assign690_e906 * locals.var_tk);
        let assign690_e911: f64 = (locals.var_tk + p.p118);
        let assign690_e912: f64 = (assign690_e908 / assign690_e911);
        let assign690_e913: f64 = (locals.var_vgzcbok - assign690_e912);
        let assign690_e915: f64 = (assign690_e913 - 0.05);
        let assign690_e917: f64 = (assign690_e915 / 0.1);
        locals.var_dxa = assign690_e917;
        locals.var_dxa_dn0 = (locals.var_vgzcbok_dn0 / 0.1);
        locals.var_dxa_dn1 = (locals.var_vgzcbok_dn1 / 0.1);
        locals.var_dxa_dn3 = ((locals.var_vgzcbok_dn3 - ((((((p.p117 * locals.var_tk_dn3) * locals.var_tk) + (assign690_e906 * locals.var_tk_dn3)) * assign690_e911) - (assign690_e908 * locals.var_tk_dn3)) / (assign690_e911 * assign690_e911))) / 0.1);
        locals.var_dxa_dn4 = (locals.var_vgzcbok_dn4 / 0.1);
        locals.var_dxa_dn5 = (locals.var_vgzcbok_dn5 / 0.1);
        locals.var_dxa_dn6 = (locals.var_vgzcbok_dn6 / 0.1);
        locals.var_dxa_dn7 = (locals.var_vgzcbok_dn7 / 0.1);
        locals.var_dxa_dn8 = (locals.var_vgzcbok_dn8 / 0.1);
        locals.var_dxa_dn9 = (locals.var_vgzcbok_dn9 / 0.1);
        locals.var_dxa_dn10 = (locals.var_vgzcbok_dn10 / 0.1);
        locals.var_dxa_rv = 0.0;

        let assign700_e921: f64 = (p.p117 * locals.var_tk);
        let assign700_e923: f64 = (assign700_e921 * locals.var_tk);
        let assign700_e926: f64 = (locals.var_tk + p.p118);
        let assign700_e927: f64 = (assign700_e923 / assign700_e926);
        let assign700_e928: f64 = (locals.var_vgzcbok - assign700_e927);
        let assign700_e930: f64 = if assign700_e928 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign700_e930;
        locals.var_guard8_rv = 0.0;

        let (assign710_e942, assign710_e942_d_n0, assign710_e942_d_n1, assign710_e942_d_n3, assign710_e942_d_n4, assign710_e942_d_n5, assign710_e942_d_n6, assign710_e942_d_n7, assign710_e942_d_n8, assign710_e942_d_n9, assign710_e942_d_n10,) = {
    if (locals.var_guard8 != 0.0) {
        let assign710_e936: f64 = (locals.var_dxa).exp();
        let assign710_e937: f64 = (1.0 + assign710_e936);
        let assign710_e938: f64 = (assign710_e937).ln();
        let assign710_e939: f64 = (0.1 * assign710_e938);
        let assign710_e940: f64 = (0.05 + assign710_e939);
        (assign710_e940, (0.1 * ((assign710_e936 * locals.var_dxa_dn0) / assign710_e937)), (0.1 * ((assign710_e936 * locals.var_dxa_dn1) / assign710_e937)), (0.1 * ((assign710_e936 * locals.var_dxa_dn3) / assign710_e937)), (0.1 * ((assign710_e936 * locals.var_dxa_dn4) / assign710_e937)), (0.1 * ((assign710_e936 * locals.var_dxa_dn5) / assign710_e937)), (0.1 * ((assign710_e936 * locals.var_dxa_dn6) / assign710_e937)), (0.1 * ((assign710_e936 * locals.var_dxa_dn7) / assign710_e937)), (0.1 * ((assign710_e936 * locals.var_dxa_dn8) / assign710_e937)), (0.1 * ((assign710_e936 * locals.var_dxa_dn9) / assign710_e937)), (0.1 * ((assign710_e936 * locals.var_dxa_dn10) / assign710_e937)),)
    } else {
        (locals.var_vgzcb_t, locals.var_vgzcb_t_dn0, locals.var_vgzcb_t_dn1, locals.var_vgzcb_t_dn3, locals.var_vgzcb_t_dn4, locals.var_vgzcb_t_dn5, locals.var_vgzcb_t_dn6, locals.var_vgzcb_t_dn7, locals.var_vgzcb_t_dn8, locals.var_vgzcb_t_dn9, locals.var_vgzcb_t_dn10,)
    }
};
        locals.var_vgzcb_t = assign710_e942;
        locals.var_vgzcb_t_dn0 = assign710_e942_d_n0;
        locals.var_vgzcb_t_dn1 = assign710_e942_d_n1;
        locals.var_vgzcb_t_dn3 = assign710_e942_d_n3;
        locals.var_vgzcb_t_dn4 = assign710_e942_d_n4;
        locals.var_vgzcb_t_dn5 = assign710_e942_d_n5;
        locals.var_vgzcb_t_dn6 = assign710_e942_d_n6;
        locals.var_vgzcb_t_dn7 = assign710_e942_d_n7;
        locals.var_vgzcb_t_dn8 = assign710_e942_d_n8;
        locals.var_vgzcb_t_dn9 = assign710_e942_d_n9;
        locals.var_vgzcb_t_dn10 = assign710_e942_d_n10;
        locals.var_vgzcb_t_rv = 0.0;

        let (assign720_e966, assign720_e966_d_n0, assign720_e966_d_n1, assign720_e966_d_n3, assign720_e966_d_n4, assign720_e966_d_n5, assign720_e966_d_n6, assign720_e966_d_n7, assign720_e966_d_n8, assign720_e966_d_n9, assign720_e966_d_n10,) = {
    if (locals.var_guard8 == 0.0) {
        let assign720_e948: f64 = (p.p117 * locals.var_tk);
        let assign720_e950: f64 = (assign720_e948 * locals.var_tk);
        let assign720_e953: f64 = (locals.var_tk + p.p118);
        let assign720_e954: f64 = (assign720_e950 / assign720_e953);
        let assign720_e955: f64 = (locals.var_vgzcbok - assign720_e954);
        let assign720_e959: f64 = (-locals.var_dxa);
        let assign720_e960: f64 = (assign720_e959).exp();
        let assign720_e961: f64 = (1.0 + assign720_e960);
        let assign720_e962: f64 = (assign720_e961).ln();
        let assign720_e963: f64 = (0.1 * assign720_e962);
        let assign720_e964: f64 = (assign720_e955 + assign720_e963);
        (assign720_e964, (locals.var_vgzcbok_dn0 + (0.1 * ((assign720_e960 * (-locals.var_dxa_dn0)) / assign720_e961))), (locals.var_vgzcbok_dn1 + (0.1 * ((assign720_e960 * (-locals.var_dxa_dn1)) / assign720_e961))), ((locals.var_vgzcbok_dn3 - ((((((p.p117 * locals.var_tk_dn3) * locals.var_tk) + (assign720_e948 * locals.var_tk_dn3)) * assign720_e953) - (assign720_e950 * locals.var_tk_dn3)) / (assign720_e953 * assign720_e953))) + (0.1 * ((assign720_e960 * (-locals.var_dxa_dn3)) / assign720_e961))), (locals.var_vgzcbok_dn4 + (0.1 * ((assign720_e960 * (-locals.var_dxa_dn4)) / assign720_e961))), (locals.var_vgzcbok_dn5 + (0.1 * ((assign720_e960 * (-locals.var_dxa_dn5)) / assign720_e961))), (locals.var_vgzcbok_dn6 + (0.1 * ((assign720_e960 * (-locals.var_dxa_dn6)) / assign720_e961))), (locals.var_vgzcbok_dn7 + (0.1 * ((assign720_e960 * (-locals.var_dxa_dn7)) / assign720_e961))), (locals.var_vgzcbok_dn8 + (0.1 * ((assign720_e960 * (-locals.var_dxa_dn8)) / assign720_e961))), (locals.var_vgzcbok_dn9 + (0.1 * ((assign720_e960 * (-locals.var_dxa_dn9)) / assign720_e961))), (locals.var_vgzcbok_dn10 + (0.1 * ((assign720_e960 * (-locals.var_dxa_dn10)) / assign720_e961))),)
    } else {
        (locals.var_vgzcb_t, locals.var_vgzcb_t_dn0, locals.var_vgzcb_t_dn1, locals.var_vgzcb_t_dn3, locals.var_vgzcb_t_dn4, locals.var_vgzcb_t_dn5, locals.var_vgzcb_t_dn6, locals.var_vgzcb_t_dn7, locals.var_vgzcb_t_dn8, locals.var_vgzcb_t_dn9, locals.var_vgzcb_t_dn10,)
    }
};
        locals.var_vgzcb_t = assign720_e966;
        locals.var_vgzcb_t_dn0 = assign720_e966_d_n0;
        locals.var_vgzcb_t_dn1 = assign720_e966_d_n1;
        locals.var_vgzcb_t_dn3 = assign720_e966_d_n3;
        locals.var_vgzcb_t_dn4 = assign720_e966_d_n4;
        locals.var_vgzcb_t_dn5 = assign720_e966_d_n5;
        locals.var_vgzcb_t_dn6 = assign720_e966_d_n6;
        locals.var_vgzcb_t_dn7 = assign720_e966_d_n7;
        locals.var_vgzcb_t_dn8 = assign720_e966_d_n8;
        locals.var_vgzcb_t_dn9 = assign720_e966_d_n9;
        locals.var_vgzcb_t_dn10 = assign720_e966_d_n10;
        locals.var_vgzcb_t_rv = 0.0;

        let assign730_e968: f64 = (-3.0);
        let assign730_e970: f64 = (assign730_e968 * locals.var_vt);
        let assign730_e972: f64 = (assign730_e970 * locals.var_lntn);
        let assign730_e975: f64 = (p.p65 * locals.var_tn);
        let assign730_e976: f64 = (assign730_e972 + assign730_e975);
        let assign730_e979: f64 = (1.0 - locals.var_tn);
        let assign730_e981: f64 = (assign730_e979 * p.p104);
        let assign730_e982: f64 = (assign730_e976 + assign730_e981);
        locals.var_udet = assign730_e982;
        locals.var_udet_dn3 = (((((assign730_e968 * locals.var_vt_dn3) * locals.var_lntn) + (assign730_e970 * locals.var_lntn_dn3)) + (p.p65 * locals.var_tn_dn3)) + ((-locals.var_tn_dn3) * p.p104));
        locals.var_udet_rv = 0.0;

        let assign740_e985: f64 = (0.05 - locals.var_udet);
        let assign740_e987: f64 = (assign740_e985 / locals.var_vt);
        locals.var_dxa = assign740_e987;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = ((((-locals.var_udet_dn3) * locals.var_vt) - (assign740_e985 * locals.var_vt_dn3)) / (locals.var_vt * locals.var_vt));
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign750_e990: f64 = if 0.05 < locals.var_udet { 1.0 } else { 0.0 };
        locals.var_guard9 = assign750_e990;
        locals.var_guard9_rv = 0.0;

        let (assign760_e1002, assign760_e1002_d_n0, assign760_e1002_d_n1, assign760_e1002_d_n3, assign760_e1002_d_n4, assign760_e1002_d_n5, assign760_e1002_d_n6, assign760_e1002_d_n7, assign760_e1002_d_n8, assign760_e1002_d_n9, assign760_e1002_d_n10,) = {
    if (locals.var_guard9 != 0.0) {
        let assign760_e996: f64 = (locals.var_dxa).exp();
        let assign760_e997: f64 = (1.0 + assign760_e996);
        let assign760_e998: f64 = (assign760_e997).ln();
        let assign760_e999: f64 = (locals.var_vt * assign760_e998);
        let assign760_e1000: f64 = (locals.var_udet + assign760_e999);
        (assign760_e1000, (locals.var_vt * ((assign760_e996 * locals.var_dxa_dn0) / assign760_e997)), (locals.var_vt * ((assign760_e996 * locals.var_dxa_dn1) / assign760_e997)), (locals.var_udet_dn3 + ((locals.var_vt_dn3 * assign760_e998) + (locals.var_vt * ((assign760_e996 * locals.var_dxa_dn3) / assign760_e997)))), (locals.var_vt * ((assign760_e996 * locals.var_dxa_dn4) / assign760_e997)), (locals.var_vt * ((assign760_e996 * locals.var_dxa_dn5) / assign760_e997)), (locals.var_vt * ((assign760_e996 * locals.var_dxa_dn6) / assign760_e997)), (locals.var_vt * ((assign760_e996 * locals.var_dxa_dn7) / assign760_e997)), (locals.var_vt * ((assign760_e996 * locals.var_dxa_dn8) / assign760_e997)), (locals.var_vt * ((assign760_e996 * locals.var_dxa_dn9) / assign760_e997)), (locals.var_vt * ((assign760_e996 * locals.var_dxa_dn10) / assign760_e997)),)
    } else {
        (locals.var_vde_t, locals.var_vde_t_dn0, locals.var_vde_t_dn1, locals.var_vde_t_dn3, locals.var_vde_t_dn4, locals.var_vde_t_dn5, locals.var_vde_t_dn6, locals.var_vde_t_dn7, locals.var_vde_t_dn8, locals.var_vde_t_dn9, locals.var_vde_t_dn10,)
    }
};
        locals.var_vde_t = assign760_e1002;
        locals.var_vde_t_dn0 = assign760_e1002_d_n0;
        locals.var_vde_t_dn1 = assign760_e1002_d_n1;
        locals.var_vde_t_dn3 = assign760_e1002_d_n3;
        locals.var_vde_t_dn4 = assign760_e1002_d_n4;
        locals.var_vde_t_dn5 = assign760_e1002_d_n5;
        locals.var_vde_t_dn6 = assign760_e1002_d_n6;
        locals.var_vde_t_dn7 = assign760_e1002_d_n7;
        locals.var_vde_t_dn8 = assign760_e1002_d_n8;
        locals.var_vde_t_dn9 = assign760_e1002_d_n9;
        locals.var_vde_t_dn10 = assign760_e1002_d_n10;
        locals.var_vde_t_rv = 0.0;

        let (assign770_e1016, assign770_e1016_d_n0, assign770_e1016_d_n1, assign770_e1016_d_n3, assign770_e1016_d_n4, assign770_e1016_d_n5, assign770_e1016_d_n6, assign770_e1016_d_n7, assign770_e1016_d_n8, assign770_e1016_d_n9, assign770_e1016_d_n10,) = {
    if (locals.var_guard9 == 0.0) {
        let assign770_e1009: f64 = (-locals.var_dxa);
        let assign770_e1010: f64 = (assign770_e1009).exp();
        let assign770_e1011: f64 = (1.0 + assign770_e1010);
        let assign770_e1012: f64 = (assign770_e1011).ln();
        let assign770_e1013: f64 = (locals.var_vt * assign770_e1012);
        let assign770_e1014: f64 = (0.05 + assign770_e1013);
        (assign770_e1014, (locals.var_vt * ((assign770_e1010 * (-locals.var_dxa_dn0)) / assign770_e1011)), (locals.var_vt * ((assign770_e1010 * (-locals.var_dxa_dn1)) / assign770_e1011)), ((locals.var_vt_dn3 * assign770_e1012) + (locals.var_vt * ((assign770_e1010 * (-locals.var_dxa_dn3)) / assign770_e1011))), (locals.var_vt * ((assign770_e1010 * (-locals.var_dxa_dn4)) / assign770_e1011)), (locals.var_vt * ((assign770_e1010 * (-locals.var_dxa_dn5)) / assign770_e1011)), (locals.var_vt * ((assign770_e1010 * (-locals.var_dxa_dn6)) / assign770_e1011)), (locals.var_vt * ((assign770_e1010 * (-locals.var_dxa_dn7)) / assign770_e1011)), (locals.var_vt * ((assign770_e1010 * (-locals.var_dxa_dn8)) / assign770_e1011)), (locals.var_vt * ((assign770_e1010 * (-locals.var_dxa_dn9)) / assign770_e1011)), (locals.var_vt * ((assign770_e1010 * (-locals.var_dxa_dn10)) / assign770_e1011)),)
    } else {
        (locals.var_vde_t, locals.var_vde_t_dn0, locals.var_vde_t_dn1, locals.var_vde_t_dn3, locals.var_vde_t_dn4, locals.var_vde_t_dn5, locals.var_vde_t_dn6, locals.var_vde_t_dn7, locals.var_vde_t_dn8, locals.var_vde_t_dn9, locals.var_vde_t_dn10,)
    }
};
        locals.var_vde_t = assign770_e1016;
        locals.var_vde_t_dn0 = assign770_e1016_d_n0;
        locals.var_vde_t_dn1 = assign770_e1016_d_n1;
        locals.var_vde_t_dn3 = assign770_e1016_d_n3;
        locals.var_vde_t_dn4 = assign770_e1016_d_n4;
        locals.var_vde_t_dn5 = assign770_e1016_d_n5;
        locals.var_vde_t_dn6 = assign770_e1016_d_n6;
        locals.var_vde_t_dn7 = assign770_e1016_d_n7;
        locals.var_vde_t_dn8 = assign770_e1016_d_n8;
        locals.var_vde_t_dn9 = assign770_e1016_d_n9;
        locals.var_vde_t_dn10 = assign770_e1016_d_n10;
        locals.var_vde_t_rv = 0.0;

        let assign780_e1018: f64 = (-3.0);
        let assign780_e1020: f64 = (assign780_e1018 * locals.var_vt);
        let assign780_e1022: f64 = (assign780_e1020 * locals.var_lntn);
        let assign780_e1025: f64 = (p.p63 * locals.var_tn);
        let assign780_e1026: f64 = (assign780_e1022 + assign780_e1025);
        let assign780_e1029: f64 = (1.0 - locals.var_tn);
        let assign780_e1031: f64 = (assign780_e1029 * p.p109);
        let assign780_e1032: f64 = (assign780_e1026 + assign780_e1031);
        locals.var_udct = assign780_e1032;
        locals.var_udct_dn3 = (((((assign780_e1018 * locals.var_vt_dn3) * locals.var_lntn) + (assign780_e1020 * locals.var_lntn_dn3)) + (p.p63 * locals.var_tn_dn3)) + ((-locals.var_tn_dn3) * p.p109));
        locals.var_udct_rv = 0.0;

        let assign790_e1035: f64 = (0.05 - locals.var_udct);
        let assign790_e1037: f64 = (assign790_e1035 / locals.var_vt);
        locals.var_dxa = assign790_e1037;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = ((((-locals.var_udct_dn3) * locals.var_vt) - (assign790_e1035 * locals.var_vt_dn3)) / (locals.var_vt * locals.var_vt));
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign800_e1040: f64 = if 0.05 < locals.var_udct { 1.0 } else { 0.0 };
        locals.var_guard10 = assign800_e1040;
        locals.var_guard10_rv = 0.0;

        let (assign810_e1052, assign810_e1052_d_n0, assign810_e1052_d_n1, assign810_e1052_d_n3, assign810_e1052_d_n4, assign810_e1052_d_n5, assign810_e1052_d_n6, assign810_e1052_d_n7, assign810_e1052_d_n8, assign810_e1052_d_n9, assign810_e1052_d_n10,) = {
    if (locals.var_guard10 != 0.0) {
        let assign810_e1046: f64 = (locals.var_dxa).exp();
        let assign810_e1047: f64 = (1.0 + assign810_e1046);
        let assign810_e1048: f64 = (assign810_e1047).ln();
        let assign810_e1049: f64 = (locals.var_vt * assign810_e1048);
        let assign810_e1050: f64 = (locals.var_udct + assign810_e1049);
        (assign810_e1050, (locals.var_vt * ((assign810_e1046 * locals.var_dxa_dn0) / assign810_e1047)), (locals.var_vt * ((assign810_e1046 * locals.var_dxa_dn1) / assign810_e1047)), (locals.var_udct_dn3 + ((locals.var_vt_dn3 * assign810_e1048) + (locals.var_vt * ((assign810_e1046 * locals.var_dxa_dn3) / assign810_e1047)))), (locals.var_vt * ((assign810_e1046 * locals.var_dxa_dn4) / assign810_e1047)), (locals.var_vt * ((assign810_e1046 * locals.var_dxa_dn5) / assign810_e1047)), (locals.var_vt * ((assign810_e1046 * locals.var_dxa_dn6) / assign810_e1047)), (locals.var_vt * ((assign810_e1046 * locals.var_dxa_dn7) / assign810_e1047)), (locals.var_vt * ((assign810_e1046 * locals.var_dxa_dn8) / assign810_e1047)), (locals.var_vt * ((assign810_e1046 * locals.var_dxa_dn9) / assign810_e1047)), (locals.var_vt * ((assign810_e1046 * locals.var_dxa_dn10) / assign810_e1047)),)
    } else {
        (locals.var_vdc_t, locals.var_vdc_t_dn0, locals.var_vdc_t_dn1, locals.var_vdc_t_dn3, locals.var_vdc_t_dn4, locals.var_vdc_t_dn5, locals.var_vdc_t_dn6, locals.var_vdc_t_dn7, locals.var_vdc_t_dn8, locals.var_vdc_t_dn9, locals.var_vdc_t_dn10,)
    }
};
        locals.var_vdc_t = assign810_e1052;
        locals.var_vdc_t_dn0 = assign810_e1052_d_n0;
        locals.var_vdc_t_dn1 = assign810_e1052_d_n1;
        locals.var_vdc_t_dn3 = assign810_e1052_d_n3;
        locals.var_vdc_t_dn4 = assign810_e1052_d_n4;
        locals.var_vdc_t_dn5 = assign810_e1052_d_n5;
        locals.var_vdc_t_dn6 = assign810_e1052_d_n6;
        locals.var_vdc_t_dn7 = assign810_e1052_d_n7;
        locals.var_vdc_t_dn8 = assign810_e1052_d_n8;
        locals.var_vdc_t_dn9 = assign810_e1052_d_n9;
        locals.var_vdc_t_dn10 = assign810_e1052_d_n10;
        locals.var_vdc_t_rv = 0.0;

        let (assign820_e1066, assign820_e1066_d_n0, assign820_e1066_d_n1, assign820_e1066_d_n3, assign820_e1066_d_n4, assign820_e1066_d_n5, assign820_e1066_d_n6, assign820_e1066_d_n7, assign820_e1066_d_n8, assign820_e1066_d_n9, assign820_e1066_d_n10,) = {
    if (locals.var_guard10 == 0.0) {
        let assign820_e1059: f64 = (-locals.var_dxa);
        let assign820_e1060: f64 = (assign820_e1059).exp();
        let assign820_e1061: f64 = (1.0 + assign820_e1060);
        let assign820_e1062: f64 = (assign820_e1061).ln();
        let assign820_e1063: f64 = (locals.var_vt * assign820_e1062);
        let assign820_e1064: f64 = (0.05 + assign820_e1063);
        (assign820_e1064, (locals.var_vt * ((assign820_e1060 * (-locals.var_dxa_dn0)) / assign820_e1061)), (locals.var_vt * ((assign820_e1060 * (-locals.var_dxa_dn1)) / assign820_e1061)), ((locals.var_vt_dn3 * assign820_e1062) + (locals.var_vt * ((assign820_e1060 * (-locals.var_dxa_dn3)) / assign820_e1061))), (locals.var_vt * ((assign820_e1060 * (-locals.var_dxa_dn4)) / assign820_e1061)), (locals.var_vt * ((assign820_e1060 * (-locals.var_dxa_dn5)) / assign820_e1061)), (locals.var_vt * ((assign820_e1060 * (-locals.var_dxa_dn6)) / assign820_e1061)), (locals.var_vt * ((assign820_e1060 * (-locals.var_dxa_dn7)) / assign820_e1061)), (locals.var_vt * ((assign820_e1060 * (-locals.var_dxa_dn8)) / assign820_e1061)), (locals.var_vt * ((assign820_e1060 * (-locals.var_dxa_dn9)) / assign820_e1061)), (locals.var_vt * ((assign820_e1060 * (-locals.var_dxa_dn10)) / assign820_e1061)),)
    } else {
        (locals.var_vdc_t, locals.var_vdc_t_dn0, locals.var_vdc_t_dn1, locals.var_vdc_t_dn3, locals.var_vdc_t_dn4, locals.var_vdc_t_dn5, locals.var_vdc_t_dn6, locals.var_vdc_t_dn7, locals.var_vdc_t_dn8, locals.var_vdc_t_dn9, locals.var_vdc_t_dn10,)
    }
};
        locals.var_vdc_t = assign820_e1066;
        locals.var_vdc_t_dn0 = assign820_e1066_d_n0;
        locals.var_vdc_t_dn1 = assign820_e1066_d_n1;
        locals.var_vdc_t_dn3 = assign820_e1066_d_n3;
        locals.var_vdc_t_dn4 = assign820_e1066_d_n4;
        locals.var_vdc_t_dn5 = assign820_e1066_d_n5;
        locals.var_vdc_t_dn6 = assign820_e1066_d_n6;
        locals.var_vdc_t_dn7 = assign820_e1066_d_n7;
        locals.var_vdc_t_dn8 = assign820_e1066_d_n8;
        locals.var_vdc_t_dn9 = assign820_e1066_d_n9;
        locals.var_vdc_t_dn10 = assign820_e1066_d_n10;
        locals.var_vdc_t_rv = 0.0;

        let assign830_e1068: f64 = (-3.0);
        let assign830_e1070: f64 = (assign830_e1068 * locals.var_vt);
        let assign830_e1072: f64 = (assign830_e1070 * locals.var_lntn);
        let assign830_e1075: f64 = (p.p79 * locals.var_tn);
        let assign830_e1076: f64 = (assign830_e1072 + assign830_e1075);
        let assign830_e1079: f64 = (1.0 - locals.var_tn);
        let assign830_e1081: f64 = (assign830_e1079 * p.p109);
        let assign830_e1082: f64 = (assign830_e1076 + assign830_e1081);
        locals.var_udcext = assign830_e1082;
        locals.var_udcext_dn3 = (((((assign830_e1068 * locals.var_vt_dn3) * locals.var_lntn) + (assign830_e1070 * locals.var_lntn_dn3)) + (p.p79 * locals.var_tn_dn3)) + ((-locals.var_tn_dn3) * p.p109));
        locals.var_udcext_rv = 0.0;

        let assign840_e1085: f64 = (0.05 - locals.var_udcext);
        let assign840_e1087: f64 = (assign840_e1085 / locals.var_vt);
        locals.var_dxa = assign840_e1087;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = ((((-locals.var_udcext_dn3) * locals.var_vt) - (assign840_e1085 * locals.var_vt_dn3)) / (locals.var_vt * locals.var_vt));
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign850_e1090: f64 = if 0.05 < locals.var_udcext { 1.0 } else { 0.0 };
        locals.var_guard11 = assign850_e1090;
        locals.var_guard11_rv = 0.0;

        let (assign860_e1102, assign860_e1102_d_n0, assign860_e1102_d_n1, assign860_e1102_d_n3, assign860_e1102_d_n4, assign860_e1102_d_n5, assign860_e1102_d_n6, assign860_e1102_d_n7, assign860_e1102_d_n8, assign860_e1102_d_n9, assign860_e1102_d_n10,) = {
    if (locals.var_guard11 != 0.0) {
        let assign860_e1096: f64 = (locals.var_dxa).exp();
        let assign860_e1097: f64 = (1.0 + assign860_e1096);
        let assign860_e1098: f64 = (assign860_e1097).ln();
        let assign860_e1099: f64 = (locals.var_vt * assign860_e1098);
        let assign860_e1100: f64 = (locals.var_udcext + assign860_e1099);
        (assign860_e1100, (locals.var_vt * ((assign860_e1096 * locals.var_dxa_dn0) / assign860_e1097)), (locals.var_vt * ((assign860_e1096 * locals.var_dxa_dn1) / assign860_e1097)), (locals.var_udcext_dn3 + ((locals.var_vt_dn3 * assign860_e1098) + (locals.var_vt * ((assign860_e1096 * locals.var_dxa_dn3) / assign860_e1097)))), (locals.var_vt * ((assign860_e1096 * locals.var_dxa_dn4) / assign860_e1097)), (locals.var_vt * ((assign860_e1096 * locals.var_dxa_dn5) / assign860_e1097)), (locals.var_vt * ((assign860_e1096 * locals.var_dxa_dn6) / assign860_e1097)), (locals.var_vt * ((assign860_e1096 * locals.var_dxa_dn7) / assign860_e1097)), (locals.var_vt * ((assign860_e1096 * locals.var_dxa_dn8) / assign860_e1097)), (locals.var_vt * ((assign860_e1096 * locals.var_dxa_dn9) / assign860_e1097)), (locals.var_vt * ((assign860_e1096 * locals.var_dxa_dn10) / assign860_e1097)),)
    } else {
        (locals.var_vdcex_t, locals.var_vdcex_t_dn0, locals.var_vdcex_t_dn1, locals.var_vdcex_t_dn3, locals.var_vdcex_t_dn4, locals.var_vdcex_t_dn5, locals.var_vdcex_t_dn6, locals.var_vdcex_t_dn7, locals.var_vdcex_t_dn8, locals.var_vdcex_t_dn9, locals.var_vdcex_t_dn10,)
    }
};
        locals.var_vdcex_t = assign860_e1102;
        locals.var_vdcex_t_dn0 = assign860_e1102_d_n0;
        locals.var_vdcex_t_dn1 = assign860_e1102_d_n1;
        locals.var_vdcex_t_dn3 = assign860_e1102_d_n3;
        locals.var_vdcex_t_dn4 = assign860_e1102_d_n4;
        locals.var_vdcex_t_dn5 = assign860_e1102_d_n5;
        locals.var_vdcex_t_dn6 = assign860_e1102_d_n6;
        locals.var_vdcex_t_dn7 = assign860_e1102_d_n7;
        locals.var_vdcex_t_dn8 = assign860_e1102_d_n8;
        locals.var_vdcex_t_dn9 = assign860_e1102_d_n9;
        locals.var_vdcex_t_dn10 = assign860_e1102_d_n10;
        locals.var_vdcex_t_rv = 0.0;

        let (assign870_e1116, assign870_e1116_d_n0, assign870_e1116_d_n1, assign870_e1116_d_n3, assign870_e1116_d_n4, assign870_e1116_d_n5, assign870_e1116_d_n6, assign870_e1116_d_n7, assign870_e1116_d_n8, assign870_e1116_d_n9, assign870_e1116_d_n10,) = {
    if (locals.var_guard11 == 0.0) {
        let assign870_e1109: f64 = (-locals.var_dxa);
        let assign870_e1110: f64 = (assign870_e1109).exp();
        let assign870_e1111: f64 = (1.0 + assign870_e1110);
        let assign870_e1112: f64 = (assign870_e1111).ln();
        let assign870_e1113: f64 = (locals.var_vt * assign870_e1112);
        let assign870_e1114: f64 = (0.05 + assign870_e1113);
        (assign870_e1114, (locals.var_vt * ((assign870_e1110 * (-locals.var_dxa_dn0)) / assign870_e1111)), (locals.var_vt * ((assign870_e1110 * (-locals.var_dxa_dn1)) / assign870_e1111)), ((locals.var_vt_dn3 * assign870_e1112) + (locals.var_vt * ((assign870_e1110 * (-locals.var_dxa_dn3)) / assign870_e1111))), (locals.var_vt * ((assign870_e1110 * (-locals.var_dxa_dn4)) / assign870_e1111)), (locals.var_vt * ((assign870_e1110 * (-locals.var_dxa_dn5)) / assign870_e1111)), (locals.var_vt * ((assign870_e1110 * (-locals.var_dxa_dn6)) / assign870_e1111)), (locals.var_vt * ((assign870_e1110 * (-locals.var_dxa_dn7)) / assign870_e1111)), (locals.var_vt * ((assign870_e1110 * (-locals.var_dxa_dn8)) / assign870_e1111)), (locals.var_vt * ((assign870_e1110 * (-locals.var_dxa_dn9)) / assign870_e1111)), (locals.var_vt * ((assign870_e1110 * (-locals.var_dxa_dn10)) / assign870_e1111)),)
    } else {
        (locals.var_vdcex_t, locals.var_vdcex_t_dn0, locals.var_vdcex_t_dn1, locals.var_vdcex_t_dn3, locals.var_vdcex_t_dn4, locals.var_vdcex_t_dn5, locals.var_vdcex_t_dn6, locals.var_vdcex_t_dn7, locals.var_vdcex_t_dn8, locals.var_vdcex_t_dn9, locals.var_vdcex_t_dn10,)
    }
};
        locals.var_vdcex_t = assign870_e1116;
        locals.var_vdcex_t_dn0 = assign870_e1116_d_n0;
        locals.var_vdcex_t_dn1 = assign870_e1116_d_n1;
        locals.var_vdcex_t_dn3 = assign870_e1116_d_n3;
        locals.var_vdcex_t_dn4 = assign870_e1116_d_n4;
        locals.var_vdcex_t_dn5 = assign870_e1116_d_n5;
        locals.var_vdcex_t_dn6 = assign870_e1116_d_n6;
        locals.var_vdcex_t_dn7 = assign870_e1116_d_n7;
        locals.var_vdcex_t_dn8 = assign870_e1116_d_n8;
        locals.var_vdcex_t_dn9 = assign870_e1116_d_n9;
        locals.var_vdcex_t_dn10 = assign870_e1116_d_n10;
        locals.var_vdcex_t_rv = 0.0;

        let assign880_e1118: f64 = (-3.0);
        let assign880_e1120: f64 = (assign880_e1118 * locals.var_vt);
        let assign880_e1122: f64 = (assign880_e1120 * locals.var_lntn);
        let assign880_e1125: f64 = (p.p70 * locals.var_tn);
        let assign880_e1126: f64 = (assign880_e1122 + assign880_e1125);
        let assign880_e1129: f64 = (1.0 - locals.var_tn);
        let assign880_e1131: f64 = (assign880_e1129 * p.p109);
        let assign880_e1132: f64 = (assign880_e1126 + assign880_e1131);
        locals.var_udct_ctc = assign880_e1132;
        locals.var_udct_ctc_dn3 = (((((assign880_e1118 * locals.var_vt_dn3) * locals.var_lntn) + (assign880_e1120 * locals.var_lntn_dn3)) + (p.p70 * locals.var_tn_dn3)) + ((-locals.var_tn_dn3) * p.p109));
        locals.var_udct_ctc_rv = 0.0;

        let assign890_e1135: f64 = (0.05 - locals.var_udct_ctc);
        let assign890_e1137: f64 = (assign890_e1135 / locals.var_vt);
        locals.var_dxa = assign890_e1137;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = ((((-locals.var_udct_ctc_dn3) * locals.var_vt) - (assign890_e1135 * locals.var_vt_dn3)) / (locals.var_vt * locals.var_vt));
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign900_e1140: f64 = if 0.05 < locals.var_udct_ctc { 1.0 } else { 0.0 };
        locals.var_guard12 = assign900_e1140;
        locals.var_guard12_rv = 0.0;

        let (assign910_e1152, assign910_e1152_d_n0, assign910_e1152_d_n1, assign910_e1152_d_n3, assign910_e1152_d_n4, assign910_e1152_d_n5, assign910_e1152_d_n6, assign910_e1152_d_n7, assign910_e1152_d_n8, assign910_e1152_d_n9, assign910_e1152_d_n10,) = {
    if (locals.var_guard12 != 0.0) {
        let assign910_e1146: f64 = (locals.var_dxa).exp();
        let assign910_e1147: f64 = (1.0 + assign910_e1146);
        let assign910_e1148: f64 = (assign910_e1147).ln();
        let assign910_e1149: f64 = (locals.var_vt * assign910_e1148);
        let assign910_e1150: f64 = (locals.var_udct_ctc + assign910_e1149);
        (assign910_e1150, (locals.var_vt * ((assign910_e1146 * locals.var_dxa_dn0) / assign910_e1147)), (locals.var_vt * ((assign910_e1146 * locals.var_dxa_dn1) / assign910_e1147)), (locals.var_udct_ctc_dn3 + ((locals.var_vt_dn3 * assign910_e1148) + (locals.var_vt * ((assign910_e1146 * locals.var_dxa_dn3) / assign910_e1147)))), (locals.var_vt * ((assign910_e1146 * locals.var_dxa_dn4) / assign910_e1147)), (locals.var_vt * ((assign910_e1146 * locals.var_dxa_dn5) / assign910_e1147)), (locals.var_vt * ((assign910_e1146 * locals.var_dxa_dn6) / assign910_e1147)), (locals.var_vt * ((assign910_e1146 * locals.var_dxa_dn7) / assign910_e1147)), (locals.var_vt * ((assign910_e1146 * locals.var_dxa_dn8) / assign910_e1147)), (locals.var_vt * ((assign910_e1146 * locals.var_dxa_dn9) / assign910_e1147)), (locals.var_vt * ((assign910_e1146 * locals.var_dxa_dn10) / assign910_e1147)),)
    } else {
        (locals.var_vdc_ctc_t, locals.var_vdc_ctc_t_dn0, locals.var_vdc_ctc_t_dn1, locals.var_vdc_ctc_t_dn3, locals.var_vdc_ctc_t_dn4, locals.var_vdc_ctc_t_dn5, locals.var_vdc_ctc_t_dn6, locals.var_vdc_ctc_t_dn7, locals.var_vdc_ctc_t_dn8, locals.var_vdc_ctc_t_dn9, locals.var_vdc_ctc_t_dn10,)
    }
};
        locals.var_vdc_ctc_t = assign910_e1152;
        locals.var_vdc_ctc_t_dn0 = assign910_e1152_d_n0;
        locals.var_vdc_ctc_t_dn1 = assign910_e1152_d_n1;
        locals.var_vdc_ctc_t_dn3 = assign910_e1152_d_n3;
        locals.var_vdc_ctc_t_dn4 = assign910_e1152_d_n4;
        locals.var_vdc_ctc_t_dn5 = assign910_e1152_d_n5;
        locals.var_vdc_ctc_t_dn6 = assign910_e1152_d_n6;
        locals.var_vdc_ctc_t_dn7 = assign910_e1152_d_n7;
        locals.var_vdc_ctc_t_dn8 = assign910_e1152_d_n8;
        locals.var_vdc_ctc_t_dn9 = assign910_e1152_d_n9;
        locals.var_vdc_ctc_t_dn10 = assign910_e1152_d_n10;
        locals.var_vdc_ctc_t_rv = 0.0;

        let (assign920_e1166, assign920_e1166_d_n0, assign920_e1166_d_n1, assign920_e1166_d_n3, assign920_e1166_d_n4, assign920_e1166_d_n5, assign920_e1166_d_n6, assign920_e1166_d_n7, assign920_e1166_d_n8, assign920_e1166_d_n9, assign920_e1166_d_n10,) = {
    if (locals.var_guard12 == 0.0) {
        let assign920_e1159: f64 = (-locals.var_dxa);
        let assign920_e1160: f64 = (assign920_e1159).exp();
        let assign920_e1161: f64 = (1.0 + assign920_e1160);
        let assign920_e1162: f64 = (assign920_e1161).ln();
        let assign920_e1163: f64 = (locals.var_vt * assign920_e1162);
        let assign920_e1164: f64 = (0.05 + assign920_e1163);
        (assign920_e1164, (locals.var_vt * ((assign920_e1160 * (-locals.var_dxa_dn0)) / assign920_e1161)), (locals.var_vt * ((assign920_e1160 * (-locals.var_dxa_dn1)) / assign920_e1161)), ((locals.var_vt_dn3 * assign920_e1162) + (locals.var_vt * ((assign920_e1160 * (-locals.var_dxa_dn3)) / assign920_e1161))), (locals.var_vt * ((assign920_e1160 * (-locals.var_dxa_dn4)) / assign920_e1161)), (locals.var_vt * ((assign920_e1160 * (-locals.var_dxa_dn5)) / assign920_e1161)), (locals.var_vt * ((assign920_e1160 * (-locals.var_dxa_dn6)) / assign920_e1161)), (locals.var_vt * ((assign920_e1160 * (-locals.var_dxa_dn7)) / assign920_e1161)), (locals.var_vt * ((assign920_e1160 * (-locals.var_dxa_dn8)) / assign920_e1161)), (locals.var_vt * ((assign920_e1160 * (-locals.var_dxa_dn9)) / assign920_e1161)), (locals.var_vt * ((assign920_e1160 * (-locals.var_dxa_dn10)) / assign920_e1161)),)
    } else {
        (locals.var_vdc_ctc_t, locals.var_vdc_ctc_t_dn0, locals.var_vdc_ctc_t_dn1, locals.var_vdc_ctc_t_dn3, locals.var_vdc_ctc_t_dn4, locals.var_vdc_ctc_t_dn5, locals.var_vdc_ctc_t_dn6, locals.var_vdc_ctc_t_dn7, locals.var_vdc_ctc_t_dn8, locals.var_vdc_ctc_t_dn9, locals.var_vdc_ctc_t_dn10,)
    }
};
        locals.var_vdc_ctc_t = assign920_e1166;
        locals.var_vdc_ctc_t_dn0 = assign920_e1166_d_n0;
        locals.var_vdc_ctc_t_dn1 = assign920_e1166_d_n1;
        locals.var_vdc_ctc_t_dn3 = assign920_e1166_d_n3;
        locals.var_vdc_ctc_t_dn4 = assign920_e1166_d_n4;
        locals.var_vdc_ctc_t_dn5 = assign920_e1166_d_n5;
        locals.var_vdc_ctc_t_dn6 = assign920_e1166_d_n6;
        locals.var_vdc_ctc_t_dn7 = assign920_e1166_d_n7;
        locals.var_vdc_ctc_t_dn8 = assign920_e1166_d_n8;
        locals.var_vdc_ctc_t_dn9 = assign920_e1166_d_n9;
        locals.var_vdc_ctc_t_dn10 = assign920_e1166_d_n10;
        locals.var_vdc_ctc_t_rv = 0.0;

        let assign930_e1168: f64 = (-3.0);
        let assign930_e1170: f64 = (assign930_e1168 * locals.var_vt);
        let assign930_e1172: f64 = (assign930_e1170 * locals.var_lntn);
        let assign930_e1175: f64 = (locals.var_vdc_zener * locals.var_tn);
        let assign930_e1176: f64 = (assign930_e1172 + assign930_e1175);
        let assign930_e1179: f64 = (1.0 - locals.var_tn);
        let assign930_e1181: f64 = (assign930_e1179 * p.p109);
        let assign930_e1182: f64 = (assign930_e1176 + assign930_e1181);
        locals.var_udct_zener = assign930_e1182;
        locals.var_udct_zener_dn3 = (((((assign930_e1168 * locals.var_vt_dn3) * locals.var_lntn) + (assign930_e1170 * locals.var_lntn_dn3)) + (locals.var_vdc_zener * locals.var_tn_dn3)) + ((-locals.var_tn_dn3) * p.p109));
        locals.var_udct_zener_rv = 0.0;

        let assign940_e1185: f64 = (0.05 - locals.var_udct_zener);
        let assign940_e1187: f64 = (assign940_e1185 / locals.var_vt);
        locals.var_dxa = assign940_e1187;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = ((((-locals.var_udct_zener_dn3) * locals.var_vt) - (assign940_e1185 * locals.var_vt_dn3)) / (locals.var_vt * locals.var_vt));
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign950_e1190: f64 = if 0.05 < locals.var_udct_zener { 1.0 } else { 0.0 };
        locals.var_guard13 = assign950_e1190;
        locals.var_guard13_rv = 0.0;

        let (assign960_e1202, assign960_e1202_d_n0, assign960_e1202_d_n1, assign960_e1202_d_n3, assign960_e1202_d_n4, assign960_e1202_d_n5, assign960_e1202_d_n6, assign960_e1202_d_n7, assign960_e1202_d_n8, assign960_e1202_d_n9, assign960_e1202_d_n10,) = {
    if (locals.var_guard13 != 0.0) {
        let assign960_e1196: f64 = (locals.var_dxa).exp();
        let assign960_e1197: f64 = (1.0 + assign960_e1196);
        let assign960_e1198: f64 = (assign960_e1197).ln();
        let assign960_e1199: f64 = (locals.var_vt * assign960_e1198);
        let assign960_e1200: f64 = (locals.var_udct_zener + assign960_e1199);
        (assign960_e1200, (locals.var_vt * ((assign960_e1196 * locals.var_dxa_dn0) / assign960_e1197)), (locals.var_vt * ((assign960_e1196 * locals.var_dxa_dn1) / assign960_e1197)), (locals.var_udct_zener_dn3 + ((locals.var_vt_dn3 * assign960_e1198) + (locals.var_vt * ((assign960_e1196 * locals.var_dxa_dn3) / assign960_e1197)))), (locals.var_vt * ((assign960_e1196 * locals.var_dxa_dn4) / assign960_e1197)), (locals.var_vt * ((assign960_e1196 * locals.var_dxa_dn5) / assign960_e1197)), (locals.var_vt * ((assign960_e1196 * locals.var_dxa_dn6) / assign960_e1197)), (locals.var_vt * ((assign960_e1196 * locals.var_dxa_dn7) / assign960_e1197)), (locals.var_vt * ((assign960_e1196 * locals.var_dxa_dn8) / assign960_e1197)), (locals.var_vt * ((assign960_e1196 * locals.var_dxa_dn9) / assign960_e1197)), (locals.var_vt * ((assign960_e1196 * locals.var_dxa_dn10) / assign960_e1197)),)
    } else {
        (locals.var_vdc_zener_t, locals.var_vdc_zener_t_dn0, locals.var_vdc_zener_t_dn1, locals.var_vdc_zener_t_dn3, locals.var_vdc_zener_t_dn4, locals.var_vdc_zener_t_dn5, locals.var_vdc_zener_t_dn6, locals.var_vdc_zener_t_dn7, locals.var_vdc_zener_t_dn8, locals.var_vdc_zener_t_dn9, locals.var_vdc_zener_t_dn10,)
    }
};
        locals.var_vdc_zener_t = assign960_e1202;
        locals.var_vdc_zener_t_dn0 = assign960_e1202_d_n0;
        locals.var_vdc_zener_t_dn1 = assign960_e1202_d_n1;
        locals.var_vdc_zener_t_dn3 = assign960_e1202_d_n3;
        locals.var_vdc_zener_t_dn4 = assign960_e1202_d_n4;
        locals.var_vdc_zener_t_dn5 = assign960_e1202_d_n5;
        locals.var_vdc_zener_t_dn6 = assign960_e1202_d_n6;
        locals.var_vdc_zener_t_dn7 = assign960_e1202_d_n7;
        locals.var_vdc_zener_t_dn8 = assign960_e1202_d_n8;
        locals.var_vdc_zener_t_dn9 = assign960_e1202_d_n9;
        locals.var_vdc_zener_t_dn10 = assign960_e1202_d_n10;
        locals.var_vdc_zener_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign970_e1216, assign970_e1216_d_n0, assign970_e1216_d_n1, assign970_e1216_d_n3, assign970_e1216_d_n4, assign970_e1216_d_n5, assign970_e1216_d_n6, assign970_e1216_d_n7, assign970_e1216_d_n8, assign970_e1216_d_n9, assign970_e1216_d_n10,) = {
    if (locals.var_guard13 == 0.0) {
        let assign970_e1209: f64 = (-locals.var_dxa);
        let assign970_e1210: f64 = (assign970_e1209).exp();
        let assign970_e1211: f64 = (1.0 + assign970_e1210);
        let assign970_e1212: f64 = (assign970_e1211).ln();
        let assign970_e1213: f64 = (locals.var_vt * assign970_e1212);
        let assign970_e1214: f64 = (0.05 + assign970_e1213);
        (assign970_e1214, (locals.var_vt * ((assign970_e1210 * (-locals.var_dxa_dn0)) / assign970_e1211)), (locals.var_vt * ((assign970_e1210 * (-locals.var_dxa_dn1)) / assign970_e1211)), ((locals.var_vt_dn3 * assign970_e1212) + (locals.var_vt * ((assign970_e1210 * (-locals.var_dxa_dn3)) / assign970_e1211))), (locals.var_vt * ((assign970_e1210 * (-locals.var_dxa_dn4)) / assign970_e1211)), (locals.var_vt * ((assign970_e1210 * (-locals.var_dxa_dn5)) / assign970_e1211)), (locals.var_vt * ((assign970_e1210 * (-locals.var_dxa_dn6)) / assign970_e1211)), (locals.var_vt * ((assign970_e1210 * (-locals.var_dxa_dn7)) / assign970_e1211)), (locals.var_vt * ((assign970_e1210 * (-locals.var_dxa_dn8)) / assign970_e1211)), (locals.var_vt * ((assign970_e1210 * (-locals.var_dxa_dn9)) / assign970_e1211)), (locals.var_vt * ((assign970_e1210 * (-locals.var_dxa_dn10)) / assign970_e1211)),)
    } else {
        (locals.var_vdc_zener_t, locals.var_vdc_zener_t_dn0, locals.var_vdc_zener_t_dn1, locals.var_vdc_zener_t_dn3, locals.var_vdc_zener_t_dn4, locals.var_vdc_zener_t_dn5, locals.var_vdc_zener_t_dn6, locals.var_vdc_zener_t_dn7, locals.var_vdc_zener_t_dn8, locals.var_vdc_zener_t_dn9, locals.var_vdc_zener_t_dn10,)
    }
};
        locals.var_vdc_zener_t = assign970_e1216;
        locals.var_vdc_zener_t_dn0 = assign970_e1216_d_n0;
        locals.var_vdc_zener_t_dn1 = assign970_e1216_d_n1;
        locals.var_vdc_zener_t_dn3 = assign970_e1216_d_n3;
        locals.var_vdc_zener_t_dn4 = assign970_e1216_d_n4;
        locals.var_vdc_zener_t_dn5 = assign970_e1216_d_n5;
        locals.var_vdc_zener_t_dn6 = assign970_e1216_d_n6;
        locals.var_vdc_zener_t_dn7 = assign970_e1216_d_n7;
        locals.var_vdc_zener_t_dn8 = assign970_e1216_d_n8;
        locals.var_vdc_zener_t_dn9 = assign970_e1216_d_n9;
        locals.var_vdc_zener_t_dn10 = assign970_e1216_d_n10;
        locals.var_vdc_zener_t_rv = 0.0;

        let assign980_e1218: f64 = (-3.0);
        let assign980_e1220: f64 = (assign980_e1218 * locals.var_vt);
        let assign980_e1222: f64 = (assign980_e1220 * locals.var_lntn);
        let assign980_e1225: f64 = (p.p26 * locals.var_tn);
        let assign980_e1226: f64 = (assign980_e1222 + assign980_e1225);
        let assign980_e1229: f64 = (1.0 - locals.var_tn);
        let assign980_e1231: f64 = (assign980_e1229 * p.p108);
        let assign980_e1232: f64 = (assign980_e1226 + assign980_e1231);
        locals.var_uknbrt = assign980_e1232;
        locals.var_uknbrt_dn3 = (((((assign980_e1218 * locals.var_vt_dn3) * locals.var_lntn) + (assign980_e1220 * locals.var_lntn_dn3)) + (p.p26 * locals.var_tn_dn3)) + ((-locals.var_tn_dn3) * p.p108));
        locals.var_uknbrt_rv = 0.0;

        let assign990_e1235: f64 = (0.05 - locals.var_uknbrt);
        let assign990_e1237: f64 = (assign990_e1235 / locals.var_vt);
        locals.var_dxa = assign990_e1237;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = ((((-locals.var_uknbrt_dn3) * locals.var_vt) - (assign990_e1235 * locals.var_vt_dn3)) / (locals.var_vt * locals.var_vt));
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign1000_e1240: f64 = if 0.05 < locals.var_uknbrt { 1.0 } else { 0.0 };
        locals.var_guard14 = assign1000_e1240;
        locals.var_guard14_rv = 0.0;

        let (assign1010_e1252, assign1010_e1252_d_n0, assign1010_e1252_d_n1, assign1010_e1252_d_n3, assign1010_e1252_d_n4, assign1010_e1252_d_n5, assign1010_e1252_d_n6, assign1010_e1252_d_n7, assign1010_e1252_d_n8, assign1010_e1252_d_n9, assign1010_e1252_d_n10,) = {
    if (locals.var_guard14 != 0.0) {
        let assign1010_e1246: f64 = (locals.var_dxa).exp();
        let assign1010_e1247: f64 = (1.0 + assign1010_e1246);
        let assign1010_e1248: f64 = (assign1010_e1247).ln();
        let assign1010_e1249: f64 = (locals.var_vt * assign1010_e1248);
        let assign1010_e1250: f64 = (locals.var_uknbrt + assign1010_e1249);
        (assign1010_e1250, (locals.var_vt * ((assign1010_e1246 * locals.var_dxa_dn0) / assign1010_e1247)), (locals.var_vt * ((assign1010_e1246 * locals.var_dxa_dn1) / assign1010_e1247)), (locals.var_uknbrt_dn3 + ((locals.var_vt_dn3 * assign1010_e1248) + (locals.var_vt * ((assign1010_e1246 * locals.var_dxa_dn3) / assign1010_e1247)))), (locals.var_vt * ((assign1010_e1246 * locals.var_dxa_dn4) / assign1010_e1247)), (locals.var_vt * ((assign1010_e1246 * locals.var_dxa_dn5) / assign1010_e1247)), (locals.var_vt * ((assign1010_e1246 * locals.var_dxa_dn6) / assign1010_e1247)), (locals.var_vt * ((assign1010_e1246 * locals.var_dxa_dn7) / assign1010_e1247)), (locals.var_vt * ((assign1010_e1246 * locals.var_dxa_dn8) / assign1010_e1247)), (locals.var_vt * ((assign1010_e1246 * locals.var_dxa_dn9) / assign1010_e1247)), (locals.var_vt * ((assign1010_e1246 * locals.var_dxa_dn10) / assign1010_e1247)),)
    } else {
        (locals.var_vknbr_t, locals.var_vknbr_t_dn0, locals.var_vknbr_t_dn1, locals.var_vknbr_t_dn3, locals.var_vknbr_t_dn4, locals.var_vknbr_t_dn5, locals.var_vknbr_t_dn6, locals.var_vknbr_t_dn7, locals.var_vknbr_t_dn8, locals.var_vknbr_t_dn9, locals.var_vknbr_t_dn10,)
    }
};
        locals.var_vknbr_t = assign1010_e1252;
        locals.var_vknbr_t_dn0 = assign1010_e1252_d_n0;
        locals.var_vknbr_t_dn1 = assign1010_e1252_d_n1;
        locals.var_vknbr_t_dn3 = assign1010_e1252_d_n3;
        locals.var_vknbr_t_dn4 = assign1010_e1252_d_n4;
        locals.var_vknbr_t_dn5 = assign1010_e1252_d_n5;
        locals.var_vknbr_t_dn6 = assign1010_e1252_d_n6;
        locals.var_vknbr_t_dn7 = assign1010_e1252_d_n7;
        locals.var_vknbr_t_dn8 = assign1010_e1252_d_n8;
        locals.var_vknbr_t_dn9 = assign1010_e1252_d_n9;
        locals.var_vknbr_t_dn10 = assign1010_e1252_d_n10;
        locals.var_vknbr_t_rv = 0.0;

        let (assign1020_e1266, assign1020_e1266_d_n0, assign1020_e1266_d_n1, assign1020_e1266_d_n3, assign1020_e1266_d_n4, assign1020_e1266_d_n5, assign1020_e1266_d_n6, assign1020_e1266_d_n7, assign1020_e1266_d_n8, assign1020_e1266_d_n9, assign1020_e1266_d_n10,) = {
    if (locals.var_guard14 == 0.0) {
        let assign1020_e1259: f64 = (-locals.var_dxa);
        let assign1020_e1260: f64 = (assign1020_e1259).exp();
        let assign1020_e1261: f64 = (1.0 + assign1020_e1260);
        let assign1020_e1262: f64 = (assign1020_e1261).ln();
        let assign1020_e1263: f64 = (locals.var_vt * assign1020_e1262);
        let assign1020_e1264: f64 = (0.05 + assign1020_e1263);
        (assign1020_e1264, (locals.var_vt * ((assign1020_e1260 * (-locals.var_dxa_dn0)) / assign1020_e1261)), (locals.var_vt * ((assign1020_e1260 * (-locals.var_dxa_dn1)) / assign1020_e1261)), ((locals.var_vt_dn3 * assign1020_e1262) + (locals.var_vt * ((assign1020_e1260 * (-locals.var_dxa_dn3)) / assign1020_e1261))), (locals.var_vt * ((assign1020_e1260 * (-locals.var_dxa_dn4)) / assign1020_e1261)), (locals.var_vt * ((assign1020_e1260 * (-locals.var_dxa_dn5)) / assign1020_e1261)), (locals.var_vt * ((assign1020_e1260 * (-locals.var_dxa_dn6)) / assign1020_e1261)), (locals.var_vt * ((assign1020_e1260 * (-locals.var_dxa_dn7)) / assign1020_e1261)), (locals.var_vt * ((assign1020_e1260 * (-locals.var_dxa_dn8)) / assign1020_e1261)), (locals.var_vt * ((assign1020_e1260 * (-locals.var_dxa_dn9)) / assign1020_e1261)), (locals.var_vt * ((assign1020_e1260 * (-locals.var_dxa_dn10)) / assign1020_e1261)),)
    } else {
        (locals.var_vknbr_t, locals.var_vknbr_t_dn0, locals.var_vknbr_t_dn1, locals.var_vknbr_t_dn3, locals.var_vknbr_t_dn4, locals.var_vknbr_t_dn5, locals.var_vknbr_t_dn6, locals.var_vknbr_t_dn7, locals.var_vknbr_t_dn8, locals.var_vknbr_t_dn9, locals.var_vknbr_t_dn10,)
    }
};
        locals.var_vknbr_t = assign1020_e1266;
        locals.var_vknbr_t_dn0 = assign1020_e1266_d_n0;
        locals.var_vknbr_t_dn1 = assign1020_e1266_d_n1;
        locals.var_vknbr_t_dn3 = assign1020_e1266_d_n3;
        locals.var_vknbr_t_dn4 = assign1020_e1266_d_n4;
        locals.var_vknbr_t_dn5 = assign1020_e1266_d_n5;
        locals.var_vknbr_t_dn6 = assign1020_e1266_d_n6;
        locals.var_vknbr_t_dn7 = assign1020_e1266_d_n7;
        locals.var_vknbr_t_dn8 = assign1020_e1266_d_n8;
        locals.var_vknbr_t_dn9 = assign1020_e1266_d_n9;
        locals.var_vknbr_t_dn10 = assign1020_e1266_d_n10;
        locals.var_vknbr_t_rv = 0.0;

        let assign1030_e1269: f64 = (1.0 / locals.var_vde_t);
        locals.var_inv_vde_t = assign1030_e1269;
        locals.var_inv_vde_t_dn0 = (-(locals.var_vde_t_dn0 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn1 = (-(locals.var_vde_t_dn1 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn3 = (-(locals.var_vde_t_dn3 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn4 = (-(locals.var_vde_t_dn4 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn5 = (-(locals.var_vde_t_dn5 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn6 = (-(locals.var_vde_t_dn6 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn7 = (-(locals.var_vde_t_dn7 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn8 = (-(locals.var_vde_t_dn8 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn9 = (-(locals.var_vde_t_dn9 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn10 = (-(locals.var_vde_t_dn10 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_rv = 0.0;

        let assign1040_e1272: f64 = (1.0 / locals.var_vdc_zener_t);
        locals.var_inv_vdc_zener_t = assign1040_e1272;
        locals.var_inv_vdc_zener_t_dn0 = (-(locals.var_vdc_zener_t_dn0 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn1 = (-(locals.var_vdc_zener_t_dn1 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn3 = (-(locals.var_vdc_zener_t_dn3 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn4 = (-(locals.var_vdc_zener_t_dn4 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn5 = (-(locals.var_vdc_zener_t_dn5 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn6 = (-(locals.var_vdc_zener_t_dn6 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn7 = (-(locals.var_vdc_zener_t_dn7 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn8 = (-(locals.var_vdc_zener_t_dn8 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn9 = (-(locals.var_vdc_zener_t_dn9 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn10 = (-(locals.var_vdc_zener_t_dn10 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_rv = 0.0;

        let assign1050_e1275: f64 = (p.p65 * locals.var_inv_vde_t);
        let assign1050_e1277: f64 = (assign1050_e1275).powf(p.p66);
        locals.var_cje_t_div_cje = assign1050_e1277;
        locals.var_cje_t_div_cje_dn0 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1050_e1275).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn0))) } } else { (assign1050_e1277 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn0) / assign1050_e1275))) };
        locals.var_cje_t_div_cje_dn1 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1050_e1275).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn1))) } } else { (assign1050_e1277 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn1) / assign1050_e1275))) };
        locals.var_cje_t_div_cje_dn3 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1050_e1275).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn3))) } } else { (assign1050_e1277 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn3) / assign1050_e1275))) };
        locals.var_cje_t_div_cje_dn4 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1050_e1275).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn4))) } } else { (assign1050_e1277 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn4) / assign1050_e1275))) };
        locals.var_cje_t_div_cje_dn5 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1050_e1275).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn5))) } } else { (assign1050_e1277 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn5) / assign1050_e1275))) };
        locals.var_cje_t_div_cje_dn6 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1050_e1275).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn6))) } } else { (assign1050_e1277 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn6) / assign1050_e1275))) };
        locals.var_cje_t_div_cje_dn7 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1050_e1275).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn7))) } } else { (assign1050_e1277 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn7) / assign1050_e1275))) };
        locals.var_cje_t_div_cje_dn8 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1050_e1275).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn8))) } } else { (assign1050_e1277 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn8) / assign1050_e1275))) };
        locals.var_cje_t_div_cje_dn9 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1050_e1275).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn9))) } } else { (assign1050_e1277 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn9) / assign1050_e1275))) };
        locals.var_cje_t_div_cje_dn10 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1050_e1275).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn10))) } } else { (assign1050_e1277 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn10) / assign1050_e1275))) };
        locals.var_cje_t_div_cje_rv = 0.0;

        let assign1060_e1280: f64 = (locals.var_vdc_zener * locals.var_inv_vdc_zener_t);
        let assign1060_e1282: f64 = (assign1060_e1280).powf(locals.var_pc_zener);
        locals.var_cjc_t_div_cjc_zener = assign1060_e1282;
        locals.var_cjc_t_div_cjc_zener_dn0 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1060_e1280).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn0))) } } else { (assign1060_e1282 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn0) / assign1060_e1280))) };
        locals.var_cjc_t_div_cjc_zener_dn1 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1060_e1280).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn1))) } } else { (assign1060_e1282 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn1) / assign1060_e1280))) };
        locals.var_cjc_t_div_cjc_zener_dn3 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1060_e1280).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn3))) } } else { (assign1060_e1282 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn3) / assign1060_e1280))) };
        locals.var_cjc_t_div_cjc_zener_dn4 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1060_e1280).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn4))) } } else { (assign1060_e1282 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn4) / assign1060_e1280))) };
        locals.var_cjc_t_div_cjc_zener_dn5 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1060_e1280).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn5))) } } else { (assign1060_e1282 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn5) / assign1060_e1280))) };
        locals.var_cjc_t_div_cjc_zener_dn6 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1060_e1280).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn6))) } } else { (assign1060_e1282 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn6) / assign1060_e1280))) };
        locals.var_cjc_t_div_cjc_zener_dn7 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1060_e1280).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn7))) } } else { (assign1060_e1282 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn7) / assign1060_e1280))) };
        locals.var_cjc_t_div_cjc_zener_dn8 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1060_e1280).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn8))) } } else { (assign1060_e1282 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn8) / assign1060_e1280))) };
        locals.var_cjc_t_div_cjc_zener_dn9 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1060_e1280).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn9))) } } else { (assign1060_e1282 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn9) / assign1060_e1280))) };
        locals.var_cjc_t_div_cjc_zener_dn10 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1060_e1280).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn10))) } } else { (assign1060_e1282 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn10) / assign1060_e1280))) };
        locals.var_cjc_t_div_cjc_zener_rv = 0.0;

        let assign1070_e1285: f64 = (p.p64 * locals.var_cje_t_div_cje);
        locals.var_cje_t = assign1070_e1285;
        locals.var_cje_t_dn0 = (p.p64 * locals.var_cje_t_div_cje_dn0);
        locals.var_cje_t_dn1 = (p.p64 * locals.var_cje_t_div_cje_dn1);
        locals.var_cje_t_dn3 = (p.p64 * locals.var_cje_t_div_cje_dn3);
        locals.var_cje_t_dn4 = (p.p64 * locals.var_cje_t_div_cje_dn4);
        locals.var_cje_t_dn5 = (p.p64 * locals.var_cje_t_div_cje_dn5);
        locals.var_cje_t_dn6 = (p.p64 * locals.var_cje_t_div_cje_dn6);
        locals.var_cje_t_dn7 = (p.p64 * locals.var_cje_t_div_cje_dn7);
        locals.var_cje_t_dn8 = (p.p64 * locals.var_cje_t_div_cje_dn8);
        locals.var_cje_t_dn9 = (p.p64 * locals.var_cje_t_div_cje_dn9);
        locals.var_cje_t_dn10 = (p.p64 * locals.var_cje_t_div_cje_dn10);
        locals.var_cje_t_rv = 0.0;

        let assign1080_e1288: f64 = (1.0 - p.p74);
        let assign1080_e1291: f64 = (p.p70 / locals.var_vdc_ctc_t);
        let assign1080_e1293: f64 = (assign1080_e1291).powf(p.p71);
        let assign1080_e1294: f64 = (assign1080_e1288 * assign1080_e1293);
        let assign1080_e1296: f64 = (assign1080_e1294 + p.p74);
        locals.var_cjc_scale = assign1080_e1296;
        locals.var_cjc_scale_dn0 = (assign1080_e1288 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1080_e1291).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn0) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1080_e1293 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn0) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1080_e1291))) });
        locals.var_cjc_scale_dn1 = (assign1080_e1288 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1080_e1291).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn1) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1080_e1293 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn1) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1080_e1291))) });
        locals.var_cjc_scale_dn3 = (assign1080_e1288 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1080_e1291).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn3) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1080_e1293 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn3) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1080_e1291))) });
        locals.var_cjc_scale_dn4 = (assign1080_e1288 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1080_e1291).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn4) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1080_e1293 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn4) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1080_e1291))) });
        locals.var_cjc_scale_dn5 = (assign1080_e1288 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1080_e1291).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn5) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1080_e1293 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn5) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1080_e1291))) });
        locals.var_cjc_scale_dn6 = (assign1080_e1288 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1080_e1291).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn6) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1080_e1293 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn6) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1080_e1291))) });
        locals.var_cjc_scale_dn7 = (assign1080_e1288 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1080_e1291).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn7) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1080_e1293 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn7) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1080_e1291))) });
        locals.var_cjc_scale_dn8 = (assign1080_e1288 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1080_e1291).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn8) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1080_e1293 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn8) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1080_e1291))) });
        locals.var_cjc_scale_dn9 = (assign1080_e1288 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1080_e1291).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn9) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1080_e1293 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn9) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1080_e1291))) });
        locals.var_cjc_scale_dn10 = (assign1080_e1288 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1080_e1291).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn10) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1080_e1293 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn10) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1080_e1291))) });
        locals.var_cjc_scale_rv = 0.0;

        let assign1090_e1299: f64 = (1.0 / locals.var_cjc_scale);
        locals.var_cjc_scale_inv = assign1090_e1299;
        locals.var_cjc_scale_inv_dn0 = (-(locals.var_cjc_scale_dn0 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn1 = (-(locals.var_cjc_scale_dn1 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn3 = (-(locals.var_cjc_scale_dn3 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn4 = (-(locals.var_cjc_scale_dn4 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn5 = (-(locals.var_cjc_scale_dn5 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn6 = (-(locals.var_cjc_scale_dn6 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn7 = (-(locals.var_cjc_scale_dn7 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn8 = (-(locals.var_cjc_scale_dn8 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn9 = (-(locals.var_cjc_scale_dn9 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn10 = (-(locals.var_cjc_scale_dn10 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_rv = 0.0;

        let assign1100_e1302: f64 = (p.p69 * locals.var_cjc_scale);
        locals.var_cjc_t = assign1100_e1302;
        locals.var_cjc_t_dn0 = (p.p69 * locals.var_cjc_scale_dn0);
        locals.var_cjc_t_dn1 = (p.p69 * locals.var_cjc_scale_dn1);
        locals.var_cjc_t_dn3 = (p.p69 * locals.var_cjc_scale_dn3);
        locals.var_cjc_t_dn4 = (p.p69 * locals.var_cjc_scale_dn4);
        locals.var_cjc_t_dn5 = (p.p69 * locals.var_cjc_scale_dn5);
        locals.var_cjc_t_dn6 = (p.p69 * locals.var_cjc_scale_dn6);
        locals.var_cjc_t_dn7 = (p.p69 * locals.var_cjc_scale_dn7);
        locals.var_cjc_t_dn8 = (p.p69 * locals.var_cjc_scale_dn8);
        locals.var_cjc_t_dn9 = (p.p69 * locals.var_cjc_scale_dn9);
        locals.var_cjc_t_dn10 = (p.p69 * locals.var_cjc_scale_dn10);
        locals.var_cjc_t_rv = 0.0;

        let assign1110_e1305: f64 = (p.p74 * locals.var_cjc_scale_inv);
        locals.var_xp_t = assign1110_e1305;
        locals.var_xp_t_dn0 = (p.p74 * locals.var_cjc_scale_inv_dn0);
        locals.var_xp_t_dn1 = (p.p74 * locals.var_cjc_scale_inv_dn1);
        locals.var_xp_t_dn3 = (p.p74 * locals.var_cjc_scale_inv_dn3);
        locals.var_xp_t_dn4 = (p.p74 * locals.var_cjc_scale_inv_dn4);
        locals.var_xp_t_dn5 = (p.p74 * locals.var_cjc_scale_inv_dn5);
        locals.var_xp_t_dn6 = (p.p74 * locals.var_cjc_scale_inv_dn6);
        locals.var_xp_t_dn7 = (p.p74 * locals.var_cjc_scale_inv_dn7);
        locals.var_xp_t_dn8 = (p.p74 * locals.var_cjc_scale_inv_dn8);
        locals.var_xp_t_dn9 = (p.p74 * locals.var_cjc_scale_inv_dn9);
        locals.var_xp_t_dn10 = (p.p74 * locals.var_cjc_scale_inv_dn10);
        locals.var_xp_t_rv = 0.0;

        let assign1120_e1309: f64 = (locals.var_lntn * p.p96);
        let assign1120_e1310: f64 = (assign1120_e1309).exp();
        let assign1120_e1311: f64 = (p.p53 * assign1120_e1310);
        locals.var_re_t = assign1120_e1311;
        locals.var_re_t_dn3 = (p.p53 * (assign1120_e1310 * (locals.var_lntn_dn3 * p.p96)));
        locals.var_re_t_rv = 0.0;

        let assign1130_e1314: f64 = if locals.var_re_t < locals.var_minr_m { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1130_e1314;
        locals.var_guard15_rv = 0.0;

        let (assign1140_e1318, assign1140_e1318_d_n3,) = {
    if (locals.var_guard15 != 0.0) {
        (locals.var_minr_m, 0.0,)
    } else {
        (locals.var_re_t, locals.var_re_t_dn3,)
    }
};
        locals.var_re_t = assign1140_e1318;
        locals.var_re_t_dn3 = assign1140_e1318_d_n3;
        locals.var_re_t_rv = 0.0;

        let assign1150_e1323: f64 = (p.p97 - p.p95);
        let assign1150_e1324: f64 = (locals.var_lntn * assign1150_e1323);
        let assign1150_e1325: f64 = (assign1150_e1324).exp();
        let assign1150_e1326: f64 = (p.p55 * assign1150_e1325);
        locals.var_rbv_t = assign1150_e1326;
        locals.var_rbv_t_dn3 = (p.p55 * (assign1150_e1325 * (locals.var_lntn_dn3 * assign1150_e1323)));
        locals.var_rbv_t_rv = 0.0;

        let assign1160_e1330: f64 = (locals.var_lntn * p.p100);
        let assign1160_e1331: f64 = (assign1160_e1330).exp();
        let assign1160_e1332: f64 = (p.p54 * assign1160_e1331);
        locals.var_rbc_t = assign1160_e1332;
        locals.var_rbc_t_dn3 = (p.p54 * (assign1160_e1331 * (locals.var_lntn_dn3 * p.p100)));
        locals.var_rbc_t_rv = 0.0;

        let assign1170_e1335: f64 = if locals.var_rbc_t < locals.var_minr_m { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1170_e1335;
        locals.var_guard16_rv = 0.0;

        let (assign1180_e1339, assign1180_e1339_d_n3,) = {
    if (locals.var_guard16 != 0.0) {
        (locals.var_minr_m, 0.0,)
    } else {
        (locals.var_rbc_t, locals.var_rbc_t_dn3,)
    }
};
        locals.var_rbc_t = assign1180_e1339;
        locals.var_rbc_t_dn3 = assign1180_e1339_d_n3;
        locals.var_rbc_t_rv = 0.0;

        let assign1190_e1343: f64 = (locals.var_lntn * p.p101);
        let assign1190_e1344: f64 = (assign1190_e1343).exp();
        let assign1190_e1345: f64 = (p.p56 * assign1190_e1344);
        locals.var_rcc_xx_t = assign1190_e1345;
        locals.var_rcc_xx_t_dn3 = (p.p56 * (assign1190_e1344 * (locals.var_lntn_dn3 * p.p101)));
        locals.var_rcc_xx_t_rv = 0.0;

        let assign1220_e1361: f64 = (locals.var_lntn * p.p98);
        let assign1220_e1362: f64 = (assign1220_e1361).exp();
        let assign1220_e1363: f64 = (p.p59 * assign1220_e1362);
        locals.var_rcv_t = assign1220_e1363;
        locals.var_rcv_t_dn3 = (p.p59 * (assign1220_e1362 * (locals.var_lntn_dn3 * p.p98)));
        locals.var_rcv_t_rv = 0.0;

        let assign1230_e1366: f64 = if p.p121 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1230_e1366;
        locals.var_guard17_rv = 0.0;

        let (assign1240_e1376, assign1240_e1376_d_n0, assign1240_e1376_d_n1, assign1240_e1376_d_n3, assign1240_e1376_d_n4, assign1240_e1376_d_n5, assign1240_e1376_d_n6, assign1240_e1376_d_n7, assign1240_e1376_d_n8, assign1240_e1376_d_n9, assign1240_e1376_d_n10,) = {
    if (locals.var_guard17 != 0.0) {
        let assign1240_e1372: f64 = (locals.var_dt * p.p121);
        let assign1240_e1373: f64 = (1.0 + assign1240_e1372);
        let assign1240_e1374: f64 = (p.p9 * assign1240_e1373);
        (assign1240_e1374, 0.0, 0.0, (p.p9 * (locals.var_dt_dn3 * p.p121)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nff_t_tmp, locals.var_nff_t_tmp_dn0, locals.var_nff_t_tmp_dn1, locals.var_nff_t_tmp_dn3, locals.var_nff_t_tmp_dn4, locals.var_nff_t_tmp_dn5, locals.var_nff_t_tmp_dn6, locals.var_nff_t_tmp_dn7, locals.var_nff_t_tmp_dn8, locals.var_nff_t_tmp_dn9, locals.var_nff_t_tmp_dn10,)
    }
};
        locals.var_nff_t_tmp = assign1240_e1376;
        locals.var_nff_t_tmp_dn0 = assign1240_e1376_d_n0;
        locals.var_nff_t_tmp_dn1 = assign1240_e1376_d_n1;
        locals.var_nff_t_tmp_dn3 = assign1240_e1376_d_n3;
        locals.var_nff_t_tmp_dn4 = assign1240_e1376_d_n4;
        locals.var_nff_t_tmp_dn5 = assign1240_e1376_d_n5;
        locals.var_nff_t_tmp_dn6 = assign1240_e1376_d_n6;
        locals.var_nff_t_tmp_dn7 = assign1240_e1376_d_n7;
        locals.var_nff_t_tmp_dn8 = assign1240_e1376_d_n8;
        locals.var_nff_t_tmp_dn9 = assign1240_e1376_d_n9;
        locals.var_nff_t_tmp_dn10 = assign1240_e1376_d_n10;
        locals.var_nff_t_tmp_rv = 0.0;

        let (assign1250_e1384, assign1250_e1384_d_n0, assign1250_e1384_d_n1, assign1250_e1384_d_n3, assign1250_e1384_d_n4, assign1250_e1384_d_n5, assign1250_e1384_d_n6, assign1250_e1384_d_n7, assign1250_e1384_d_n8, assign1250_e1384_d_n9, assign1250_e1384_d_n10,) = {
    if (locals.var_guard17 != 0.0) {
        let assign1250_e1380: f64 = (locals.var_nff_t_tmp - 1.0);
        let assign1250_e1382: f64 = (assign1250_e1380 / locals.var_eps_nf);
        (assign1250_e1382, (locals.var_nff_t_tmp_dn0 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn1 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn3 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn4 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn5 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn6 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn7 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn8 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn9 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn10 / locals.var_eps_nf),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9, locals.var_dxa_dn10,)
    }
};
        locals.var_dxa = assign1250_e1384;
        locals.var_dxa_dn0 = assign1250_e1384_d_n0;
        locals.var_dxa_dn1 = assign1250_e1384_d_n1;
        locals.var_dxa_dn3 = assign1250_e1384_d_n3;
        locals.var_dxa_dn4 = assign1250_e1384_d_n4;
        locals.var_dxa_dn5 = assign1250_e1384_d_n5;
        locals.var_dxa_dn6 = assign1250_e1384_d_n6;
        locals.var_dxa_dn7 = assign1250_e1384_d_n7;
        locals.var_dxa_dn8 = assign1250_e1384_d_n8;
        locals.var_dxa_dn9 = assign1250_e1384_d_n9;
        locals.var_dxa_dn10 = assign1250_e1384_d_n10;
        locals.var_dxa_rv = 0.0;

        let assign1260_e1387: f64 = if locals.var_nff_t_tmp < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard18 = assign1260_e1387;
        locals.var_guard18_rv = 0.0;

        let (assign1270_e1401, assign1270_e1401_d_n0, assign1270_e1401_d_n1, assign1270_e1401_d_n3, assign1270_e1401_d_n4, assign1270_e1401_d_n5, assign1270_e1401_d_n6, assign1270_e1401_d_n7, assign1270_e1401_d_n8, assign1270_e1401_d_n9, assign1270_e1401_d_n10,) = {
    if ((locals.var_guard17 != 0.0) && (locals.var_guard18 != 0.0)) {
        let assign1270_e1395: f64 = (locals.var_dxa).exp();
        let assign1270_e1396: f64 = (1.0 + assign1270_e1395);
        let assign1270_e1397: f64 = (assign1270_e1396).ln();
        let assign1270_e1398: f64 = (locals.var_eps_nf * assign1270_e1397);
        let assign1270_e1399: f64 = (1.0 + assign1270_e1398);
        (assign1270_e1399, (locals.var_eps_nf * ((assign1270_e1395 * locals.var_dxa_dn0) / assign1270_e1396)), (locals.var_eps_nf * ((assign1270_e1395 * locals.var_dxa_dn1) / assign1270_e1396)), (locals.var_eps_nf * ((assign1270_e1395 * locals.var_dxa_dn3) / assign1270_e1396)), (locals.var_eps_nf * ((assign1270_e1395 * locals.var_dxa_dn4) / assign1270_e1396)), (locals.var_eps_nf * ((assign1270_e1395 * locals.var_dxa_dn5) / assign1270_e1396)), (locals.var_eps_nf * ((assign1270_e1395 * locals.var_dxa_dn6) / assign1270_e1396)), (locals.var_eps_nf * ((assign1270_e1395 * locals.var_dxa_dn7) / assign1270_e1396)), (locals.var_eps_nf * ((assign1270_e1395 * locals.var_dxa_dn8) / assign1270_e1396)), (locals.var_eps_nf * ((assign1270_e1395 * locals.var_dxa_dn9) / assign1270_e1396)), (locals.var_eps_nf * ((assign1270_e1395 * locals.var_dxa_dn10) / assign1270_e1396)),)
    } else {
        (locals.var_nff_t_tmp, locals.var_nff_t_tmp_dn0, locals.var_nff_t_tmp_dn1, locals.var_nff_t_tmp_dn3, locals.var_nff_t_tmp_dn4, locals.var_nff_t_tmp_dn5, locals.var_nff_t_tmp_dn6, locals.var_nff_t_tmp_dn7, locals.var_nff_t_tmp_dn8, locals.var_nff_t_tmp_dn9, locals.var_nff_t_tmp_dn10,)
    }
};
        locals.var_nff_t_tmp = assign1270_e1401;
        locals.var_nff_t_tmp_dn0 = assign1270_e1401_d_n0;
        locals.var_nff_t_tmp_dn1 = assign1270_e1401_d_n1;
        locals.var_nff_t_tmp_dn3 = assign1270_e1401_d_n3;
        locals.var_nff_t_tmp_dn4 = assign1270_e1401_d_n4;
        locals.var_nff_t_tmp_dn5 = assign1270_e1401_d_n5;
        locals.var_nff_t_tmp_dn6 = assign1270_e1401_d_n6;
        locals.var_nff_t_tmp_dn7 = assign1270_e1401_d_n7;
        locals.var_nff_t_tmp_dn8 = assign1270_e1401_d_n8;
        locals.var_nff_t_tmp_dn9 = assign1270_e1401_d_n9;
        locals.var_nff_t_tmp_dn10 = assign1270_e1401_d_n10;
        locals.var_nff_t_tmp_rv = 0.0;

        let (assign1280_e1417, assign1280_e1417_d_n0, assign1280_e1417_d_n1, assign1280_e1417_d_n3, assign1280_e1417_d_n4, assign1280_e1417_d_n5, assign1280_e1417_d_n6, assign1280_e1417_d_n7, assign1280_e1417_d_n8, assign1280_e1417_d_n9, assign1280_e1417_d_n10,) = {
    if ((locals.var_guard17 != 0.0) && (locals.var_guard18 == 0.0)) {
        let assign1280_e1410: f64 = (-locals.var_dxa);
        let assign1280_e1411: f64 = (assign1280_e1410).exp();
        let assign1280_e1412: f64 = (1.0 + assign1280_e1411);
        let assign1280_e1413: f64 = (assign1280_e1412).ln();
        let assign1280_e1414: f64 = (locals.var_eps_nf * assign1280_e1413);
        let assign1280_e1415: f64 = (locals.var_nff_t_tmp + assign1280_e1414);
        (assign1280_e1415, (locals.var_nff_t_tmp_dn0 + (locals.var_eps_nf * ((assign1280_e1411 * (-locals.var_dxa_dn0)) / assign1280_e1412))), (locals.var_nff_t_tmp_dn1 + (locals.var_eps_nf * ((assign1280_e1411 * (-locals.var_dxa_dn1)) / assign1280_e1412))), (locals.var_nff_t_tmp_dn3 + (locals.var_eps_nf * ((assign1280_e1411 * (-locals.var_dxa_dn3)) / assign1280_e1412))), (locals.var_nff_t_tmp_dn4 + (locals.var_eps_nf * ((assign1280_e1411 * (-locals.var_dxa_dn4)) / assign1280_e1412))), (locals.var_nff_t_tmp_dn5 + (locals.var_eps_nf * ((assign1280_e1411 * (-locals.var_dxa_dn5)) / assign1280_e1412))), (locals.var_nff_t_tmp_dn6 + (locals.var_eps_nf * ((assign1280_e1411 * (-locals.var_dxa_dn6)) / assign1280_e1412))), (locals.var_nff_t_tmp_dn7 + (locals.var_eps_nf * ((assign1280_e1411 * (-locals.var_dxa_dn7)) / assign1280_e1412))), (locals.var_nff_t_tmp_dn8 + (locals.var_eps_nf * ((assign1280_e1411 * (-locals.var_dxa_dn8)) / assign1280_e1412))), (locals.var_nff_t_tmp_dn9 + (locals.var_eps_nf * ((assign1280_e1411 * (-locals.var_dxa_dn9)) / assign1280_e1412))), (locals.var_nff_t_tmp_dn10 + (locals.var_eps_nf * ((assign1280_e1411 * (-locals.var_dxa_dn10)) / assign1280_e1412))),)
    } else {
        (locals.var_nff_t_tmp, locals.var_nff_t_tmp_dn0, locals.var_nff_t_tmp_dn1, locals.var_nff_t_tmp_dn3, locals.var_nff_t_tmp_dn4, locals.var_nff_t_tmp_dn5, locals.var_nff_t_tmp_dn6, locals.var_nff_t_tmp_dn7, locals.var_nff_t_tmp_dn8, locals.var_nff_t_tmp_dn9, locals.var_nff_t_tmp_dn10,)
    }
};
        locals.var_nff_t_tmp = assign1280_e1417;
        locals.var_nff_t_tmp_dn0 = assign1280_e1417_d_n0;
        locals.var_nff_t_tmp_dn1 = assign1280_e1417_d_n1;
        locals.var_nff_t_tmp_dn3 = assign1280_e1417_d_n3;
        locals.var_nff_t_tmp_dn4 = assign1280_e1417_d_n4;
        locals.var_nff_t_tmp_dn5 = assign1280_e1417_d_n5;
        locals.var_nff_t_tmp_dn6 = assign1280_e1417_d_n6;
        locals.var_nff_t_tmp_dn7 = assign1280_e1417_d_n7;
        locals.var_nff_t_tmp_dn8 = assign1280_e1417_d_n8;
        locals.var_nff_t_tmp_dn9 = assign1280_e1417_d_n9;
        locals.var_nff_t_tmp_dn10 = assign1280_e1417_d_n10;
        locals.var_nff_t_tmp_rv = 0.0;

        let (assign1290_e1425, assign1290_e1425_d_n0, assign1290_e1425_d_n1, assign1290_e1425_d_n3, assign1290_e1425_d_n4, assign1290_e1425_d_n5, assign1290_e1425_d_n6, assign1290_e1425_d_n7, assign1290_e1425_d_n8, assign1290_e1425_d_n9, assign1290_e1425_d_n10,) = {
    if (locals.var_guard17 != 0.0) {
        let assign1290_e1422: f64 = (locals.var_eps_nf * 0.6931471805599453);
        let assign1290_e1423: f64 = (locals.var_nff_t_tmp - assign1290_e1422);
        (assign1290_e1423, locals.var_nff_t_tmp_dn0, locals.var_nff_t_tmp_dn1, locals.var_nff_t_tmp_dn3, locals.var_nff_t_tmp_dn4, locals.var_nff_t_tmp_dn5, locals.var_nff_t_tmp_dn6, locals.var_nff_t_tmp_dn7, locals.var_nff_t_tmp_dn8, locals.var_nff_t_tmp_dn9, locals.var_nff_t_tmp_dn10,)
    } else {
        (locals.var_nff_t, locals.var_nff_t_dn0, locals.var_nff_t_dn1, locals.var_nff_t_dn3, locals.var_nff_t_dn4, locals.var_nff_t_dn5, locals.var_nff_t_dn6, locals.var_nff_t_dn7, locals.var_nff_t_dn8, locals.var_nff_t_dn9, locals.var_nff_t_dn10,)
    }
};
        locals.var_nff_t = assign1290_e1425;
        locals.var_nff_t_dn0 = assign1290_e1425_d_n0;
        locals.var_nff_t_dn1 = assign1290_e1425_d_n1;
        locals.var_nff_t_dn3 = assign1290_e1425_d_n3;
        locals.var_nff_t_dn4 = assign1290_e1425_d_n4;
        locals.var_nff_t_dn5 = assign1290_e1425_d_n5;
        locals.var_nff_t_dn6 = assign1290_e1425_d_n6;
        locals.var_nff_t_dn7 = assign1290_e1425_d_n7;
        locals.var_nff_t_dn8 = assign1290_e1425_d_n8;
        locals.var_nff_t_dn9 = assign1290_e1425_d_n9;
        locals.var_nff_t_dn10 = assign1290_e1425_d_n10;
        locals.var_nff_t_rv = 0.0;

        let (assign1300_e1430, assign1300_e1430_d_n0, assign1300_e1430_d_n1, assign1300_e1430_d_n3, assign1300_e1430_d_n4, assign1300_e1430_d_n5, assign1300_e1430_d_n6, assign1300_e1430_d_n7, assign1300_e1430_d_n8, assign1300_e1430_d_n9, assign1300_e1430_d_n10,) = {
    if (locals.var_guard17 == 0.0) {
        (p.p9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nff_t, locals.var_nff_t_dn0, locals.var_nff_t_dn1, locals.var_nff_t_dn3, locals.var_nff_t_dn4, locals.var_nff_t_dn5, locals.var_nff_t_dn6, locals.var_nff_t_dn7, locals.var_nff_t_dn8, locals.var_nff_t_dn9, locals.var_nff_t_dn10,)
    }
};
        locals.var_nff_t = assign1300_e1430;
        locals.var_nff_t_dn0 = assign1300_e1430_d_n0;
        locals.var_nff_t_dn1 = assign1300_e1430_d_n1;
        locals.var_nff_t_dn3 = assign1300_e1430_d_n3;
        locals.var_nff_t_dn4 = assign1300_e1430_d_n4;
        locals.var_nff_t_dn5 = assign1300_e1430_d_n5;
        locals.var_nff_t_dn6 = assign1300_e1430_d_n6;
        locals.var_nff_t_dn7 = assign1300_e1430_d_n7;
        locals.var_nff_t_dn8 = assign1300_e1430_d_n8;
        locals.var_nff_t_dn9 = assign1300_e1430_d_n9;
        locals.var_nff_t_dn10 = assign1300_e1430_d_n10;
        locals.var_nff_t_rv = 0.0;

        let assign1310_e1433: f64 = if p.p122 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard19 = assign1310_e1433;
        locals.var_guard19_rv = 0.0;

        let (assign1320_e1443, assign1320_e1443_d_n0, assign1320_e1443_d_n1, assign1320_e1443_d_n3, assign1320_e1443_d_n4, assign1320_e1443_d_n5, assign1320_e1443_d_n6, assign1320_e1443_d_n7, assign1320_e1443_d_n8, assign1320_e1443_d_n9, assign1320_e1443_d_n10,) = {
    if (locals.var_guard19 != 0.0) {
        let assign1320_e1439: f64 = (locals.var_dt * p.p122);
        let assign1320_e1440: f64 = (1.0 + assign1320_e1439);
        let assign1320_e1441: f64 = (p.p10 * assign1320_e1440);
        (assign1320_e1441, 0.0, 0.0, (p.p10 * (locals.var_dt_dn3 * p.p122)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nfr_t_tmp, locals.var_nfr_t_tmp_dn0, locals.var_nfr_t_tmp_dn1, locals.var_nfr_t_tmp_dn3, locals.var_nfr_t_tmp_dn4, locals.var_nfr_t_tmp_dn5, locals.var_nfr_t_tmp_dn6, locals.var_nfr_t_tmp_dn7, locals.var_nfr_t_tmp_dn8, locals.var_nfr_t_tmp_dn9, locals.var_nfr_t_tmp_dn10,)
    }
};
        locals.var_nfr_t_tmp = assign1320_e1443;
        locals.var_nfr_t_tmp_dn0 = assign1320_e1443_d_n0;
        locals.var_nfr_t_tmp_dn1 = assign1320_e1443_d_n1;
        locals.var_nfr_t_tmp_dn3 = assign1320_e1443_d_n3;
        locals.var_nfr_t_tmp_dn4 = assign1320_e1443_d_n4;
        locals.var_nfr_t_tmp_dn5 = assign1320_e1443_d_n5;
        locals.var_nfr_t_tmp_dn6 = assign1320_e1443_d_n6;
        locals.var_nfr_t_tmp_dn7 = assign1320_e1443_d_n7;
        locals.var_nfr_t_tmp_dn8 = assign1320_e1443_d_n8;
        locals.var_nfr_t_tmp_dn9 = assign1320_e1443_d_n9;
        locals.var_nfr_t_tmp_dn10 = assign1320_e1443_d_n10;
        locals.var_nfr_t_tmp_rv = 0.0;

        let (assign1330_e1451, assign1330_e1451_d_n0, assign1330_e1451_d_n1, assign1330_e1451_d_n3, assign1330_e1451_d_n4, assign1330_e1451_d_n5, assign1330_e1451_d_n6, assign1330_e1451_d_n7, assign1330_e1451_d_n8, assign1330_e1451_d_n9, assign1330_e1451_d_n10,) = {
    if (locals.var_guard19 != 0.0) {
        let assign1330_e1447: f64 = (locals.var_nfr_t_tmp - 1.0);
        let assign1330_e1449: f64 = (assign1330_e1447 / locals.var_eps_nf);
        (assign1330_e1449, (locals.var_nfr_t_tmp_dn0 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn1 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn3 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn4 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn5 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn6 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn7 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn8 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn9 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn10 / locals.var_eps_nf),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9, locals.var_dxa_dn10,)
    }
};
        locals.var_dxa = assign1330_e1451;
        locals.var_dxa_dn0 = assign1330_e1451_d_n0;
        locals.var_dxa_dn1 = assign1330_e1451_d_n1;
        locals.var_dxa_dn3 = assign1330_e1451_d_n3;
        locals.var_dxa_dn4 = assign1330_e1451_d_n4;
        locals.var_dxa_dn5 = assign1330_e1451_d_n5;
        locals.var_dxa_dn6 = assign1330_e1451_d_n6;
        locals.var_dxa_dn7 = assign1330_e1451_d_n7;
        locals.var_dxa_dn8 = assign1330_e1451_d_n8;
        locals.var_dxa_dn9 = assign1330_e1451_d_n9;
        locals.var_dxa_dn10 = assign1330_e1451_d_n10;
        locals.var_dxa_rv = 0.0;

        let assign1340_e1454: f64 = if locals.var_nfr_t_tmp < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign1340_e1454;
        locals.var_guard20_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (assign1350_e1468, assign1350_e1468_d_n0, assign1350_e1468_d_n1, assign1350_e1468_d_n3, assign1350_e1468_d_n4, assign1350_e1468_d_n5, assign1350_e1468_d_n6, assign1350_e1468_d_n7, assign1350_e1468_d_n8, assign1350_e1468_d_n9, assign1350_e1468_d_n10,) = {
    if ((locals.var_guard19 != 0.0) && (locals.var_guard20 != 0.0)) {
        let assign1350_e1462: f64 = (locals.var_dxa).exp();
        let assign1350_e1463: f64 = (1.0 + assign1350_e1462);
        let assign1350_e1464: f64 = (assign1350_e1463).ln();
        let assign1350_e1465: f64 = (locals.var_eps_nf * assign1350_e1464);
        let assign1350_e1466: f64 = (1.0 + assign1350_e1465);
        (assign1350_e1466, (locals.var_eps_nf * ((assign1350_e1462 * locals.var_dxa_dn0) / assign1350_e1463)), (locals.var_eps_nf * ((assign1350_e1462 * locals.var_dxa_dn1) / assign1350_e1463)), (locals.var_eps_nf * ((assign1350_e1462 * locals.var_dxa_dn3) / assign1350_e1463)), (locals.var_eps_nf * ((assign1350_e1462 * locals.var_dxa_dn4) / assign1350_e1463)), (locals.var_eps_nf * ((assign1350_e1462 * locals.var_dxa_dn5) / assign1350_e1463)), (locals.var_eps_nf * ((assign1350_e1462 * locals.var_dxa_dn6) / assign1350_e1463)), (locals.var_eps_nf * ((assign1350_e1462 * locals.var_dxa_dn7) / assign1350_e1463)), (locals.var_eps_nf * ((assign1350_e1462 * locals.var_dxa_dn8) / assign1350_e1463)), (locals.var_eps_nf * ((assign1350_e1462 * locals.var_dxa_dn9) / assign1350_e1463)), (locals.var_eps_nf * ((assign1350_e1462 * locals.var_dxa_dn10) / assign1350_e1463)),)
    } else {
        (locals.var_nfr_t_tmp, locals.var_nfr_t_tmp_dn0, locals.var_nfr_t_tmp_dn1, locals.var_nfr_t_tmp_dn3, locals.var_nfr_t_tmp_dn4, locals.var_nfr_t_tmp_dn5, locals.var_nfr_t_tmp_dn6, locals.var_nfr_t_tmp_dn7, locals.var_nfr_t_tmp_dn8, locals.var_nfr_t_tmp_dn9, locals.var_nfr_t_tmp_dn10,)
    }
};
        locals.var_nfr_t_tmp = assign1350_e1468;
        locals.var_nfr_t_tmp_dn0 = assign1350_e1468_d_n0;
        locals.var_nfr_t_tmp_dn1 = assign1350_e1468_d_n1;
        locals.var_nfr_t_tmp_dn3 = assign1350_e1468_d_n3;
        locals.var_nfr_t_tmp_dn4 = assign1350_e1468_d_n4;
        locals.var_nfr_t_tmp_dn5 = assign1350_e1468_d_n5;
        locals.var_nfr_t_tmp_dn6 = assign1350_e1468_d_n6;
        locals.var_nfr_t_tmp_dn7 = assign1350_e1468_d_n7;
        locals.var_nfr_t_tmp_dn8 = assign1350_e1468_d_n8;
        locals.var_nfr_t_tmp_dn9 = assign1350_e1468_d_n9;
        locals.var_nfr_t_tmp_dn10 = assign1350_e1468_d_n10;
        locals.var_nfr_t_tmp_rv = 0.0;

        let (assign1360_e1484, assign1360_e1484_d_n0, assign1360_e1484_d_n1, assign1360_e1484_d_n3, assign1360_e1484_d_n4, assign1360_e1484_d_n5, assign1360_e1484_d_n6, assign1360_e1484_d_n7, assign1360_e1484_d_n8, assign1360_e1484_d_n9, assign1360_e1484_d_n10,) = {
    if ((locals.var_guard19 != 0.0) && (locals.var_guard20 == 0.0)) {
        let assign1360_e1477: f64 = (-locals.var_dxa);
        let assign1360_e1478: f64 = (assign1360_e1477).exp();
        let assign1360_e1479: f64 = (1.0 + assign1360_e1478);
        let assign1360_e1480: f64 = (assign1360_e1479).ln();
        let assign1360_e1481: f64 = (locals.var_eps_nf * assign1360_e1480);
        let assign1360_e1482: f64 = (locals.var_nfr_t_tmp + assign1360_e1481);
        (assign1360_e1482, (locals.var_nfr_t_tmp_dn0 + (locals.var_eps_nf * ((assign1360_e1478 * (-locals.var_dxa_dn0)) / assign1360_e1479))), (locals.var_nfr_t_tmp_dn1 + (locals.var_eps_nf * ((assign1360_e1478 * (-locals.var_dxa_dn1)) / assign1360_e1479))), (locals.var_nfr_t_tmp_dn3 + (locals.var_eps_nf * ((assign1360_e1478 * (-locals.var_dxa_dn3)) / assign1360_e1479))), (locals.var_nfr_t_tmp_dn4 + (locals.var_eps_nf * ((assign1360_e1478 * (-locals.var_dxa_dn4)) / assign1360_e1479))), (locals.var_nfr_t_tmp_dn5 + (locals.var_eps_nf * ((assign1360_e1478 * (-locals.var_dxa_dn5)) / assign1360_e1479))), (locals.var_nfr_t_tmp_dn6 + (locals.var_eps_nf * ((assign1360_e1478 * (-locals.var_dxa_dn6)) / assign1360_e1479))), (locals.var_nfr_t_tmp_dn7 + (locals.var_eps_nf * ((assign1360_e1478 * (-locals.var_dxa_dn7)) / assign1360_e1479))), (locals.var_nfr_t_tmp_dn8 + (locals.var_eps_nf * ((assign1360_e1478 * (-locals.var_dxa_dn8)) / assign1360_e1479))), (locals.var_nfr_t_tmp_dn9 + (locals.var_eps_nf * ((assign1360_e1478 * (-locals.var_dxa_dn9)) / assign1360_e1479))), (locals.var_nfr_t_tmp_dn10 + (locals.var_eps_nf * ((assign1360_e1478 * (-locals.var_dxa_dn10)) / assign1360_e1479))),)
    } else {
        (locals.var_nfr_t_tmp, locals.var_nfr_t_tmp_dn0, locals.var_nfr_t_tmp_dn1, locals.var_nfr_t_tmp_dn3, locals.var_nfr_t_tmp_dn4, locals.var_nfr_t_tmp_dn5, locals.var_nfr_t_tmp_dn6, locals.var_nfr_t_tmp_dn7, locals.var_nfr_t_tmp_dn8, locals.var_nfr_t_tmp_dn9, locals.var_nfr_t_tmp_dn10,)
    }
};
        locals.var_nfr_t_tmp = assign1360_e1484;
        locals.var_nfr_t_tmp_dn0 = assign1360_e1484_d_n0;
        locals.var_nfr_t_tmp_dn1 = assign1360_e1484_d_n1;
        locals.var_nfr_t_tmp_dn3 = assign1360_e1484_d_n3;
        locals.var_nfr_t_tmp_dn4 = assign1360_e1484_d_n4;
        locals.var_nfr_t_tmp_dn5 = assign1360_e1484_d_n5;
        locals.var_nfr_t_tmp_dn6 = assign1360_e1484_d_n6;
        locals.var_nfr_t_tmp_dn7 = assign1360_e1484_d_n7;
        locals.var_nfr_t_tmp_dn8 = assign1360_e1484_d_n8;
        locals.var_nfr_t_tmp_dn9 = assign1360_e1484_d_n9;
        locals.var_nfr_t_tmp_dn10 = assign1360_e1484_d_n10;
        locals.var_nfr_t_tmp_rv = 0.0;

        let (assign1370_e1492, assign1370_e1492_d_n0, assign1370_e1492_d_n1, assign1370_e1492_d_n3, assign1370_e1492_d_n4, assign1370_e1492_d_n5, assign1370_e1492_d_n6, assign1370_e1492_d_n7, assign1370_e1492_d_n8, assign1370_e1492_d_n9, assign1370_e1492_d_n10,) = {
    if (locals.var_guard19 != 0.0) {
        let assign1370_e1489: f64 = (locals.var_eps_nf * 0.6931471805599453);
        let assign1370_e1490: f64 = (locals.var_nfr_t_tmp - assign1370_e1489);
        (assign1370_e1490, locals.var_nfr_t_tmp_dn0, locals.var_nfr_t_tmp_dn1, locals.var_nfr_t_tmp_dn3, locals.var_nfr_t_tmp_dn4, locals.var_nfr_t_tmp_dn5, locals.var_nfr_t_tmp_dn6, locals.var_nfr_t_tmp_dn7, locals.var_nfr_t_tmp_dn8, locals.var_nfr_t_tmp_dn9, locals.var_nfr_t_tmp_dn10,)
    } else {
        (locals.var_nfr_t, locals.var_nfr_t_dn0, locals.var_nfr_t_dn1, locals.var_nfr_t_dn3, locals.var_nfr_t_dn4, locals.var_nfr_t_dn5, locals.var_nfr_t_dn6, locals.var_nfr_t_dn7, locals.var_nfr_t_dn8, locals.var_nfr_t_dn9, locals.var_nfr_t_dn10,)
    }
};
        locals.var_nfr_t = assign1370_e1492;
        locals.var_nfr_t_dn0 = assign1370_e1492_d_n0;
        locals.var_nfr_t_dn1 = assign1370_e1492_d_n1;
        locals.var_nfr_t_dn3 = assign1370_e1492_d_n3;
        locals.var_nfr_t_dn4 = assign1370_e1492_d_n4;
        locals.var_nfr_t_dn5 = assign1370_e1492_d_n5;
        locals.var_nfr_t_dn6 = assign1370_e1492_d_n6;
        locals.var_nfr_t_dn7 = assign1370_e1492_d_n7;
        locals.var_nfr_t_dn8 = assign1370_e1492_d_n8;
        locals.var_nfr_t_dn9 = assign1370_e1492_d_n9;
        locals.var_nfr_t_dn10 = assign1370_e1492_d_n10;
        locals.var_nfr_t_rv = 0.0;

        let (assign1380_e1497, assign1380_e1497_d_n0, assign1380_e1497_d_n1, assign1380_e1497_d_n3, assign1380_e1497_d_n4, assign1380_e1497_d_n5, assign1380_e1497_d_n6, assign1380_e1497_d_n7, assign1380_e1497_d_n8, assign1380_e1497_d_n9, assign1380_e1497_d_n10,) = {
    if (locals.var_guard19 == 0.0) {
        (p.p10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nfr_t, locals.var_nfr_t_dn0, locals.var_nfr_t_dn1, locals.var_nfr_t_dn3, locals.var_nfr_t_dn4, locals.var_nfr_t_dn5, locals.var_nfr_t_dn6, locals.var_nfr_t_dn7, locals.var_nfr_t_dn8, locals.var_nfr_t_dn9, locals.var_nfr_t_dn10,)
    }
};
        locals.var_nfr_t = assign1380_e1497;
        locals.var_nfr_t_dn0 = assign1380_e1497_d_n0;
        locals.var_nfr_t_dn1 = assign1380_e1497_d_n1;
        locals.var_nfr_t_dn3 = assign1380_e1497_d_n3;
        locals.var_nfr_t_dn4 = assign1380_e1497_d_n4;
        locals.var_nfr_t_dn5 = assign1380_e1497_d_n5;
        locals.var_nfr_t_dn6 = assign1380_e1497_d_n6;
        locals.var_nfr_t_dn7 = assign1380_e1497_d_n7;
        locals.var_nfr_t_dn8 = assign1380_e1497_d_n8;
        locals.var_nfr_t_dn9 = assign1380_e1497_d_n9;
        locals.var_nfr_t_dn10 = assign1380_e1497_d_n10;
        locals.var_nfr_t_rv = 0.0;

        let assign1390_e1502: f64 = (p.p123 * locals.var_dt);
        let assign1390_e1503: f64 = (1.0 + assign1390_e1502);
        let assign1390_e1504: f64 = (p.p42 * assign1390_e1503);
        locals.var_bavl_t_tmp = assign1390_e1504;
        locals.var_bavl_t_tmp_dn3 = (p.p42 * (p.p123 * locals.var_dt_dn3));
        locals.var_bavl_t_tmp_rv = 0.0;

        let assign1400_e1507: f64 = (locals.var_eps_bavl_t * locals.var_eps_bavl_t);
        locals.var_eps2 = assign1400_e1507;
        locals.var_eps2_dn0 = 0.0;
        locals.var_eps2_dn1 = 0.0;
        locals.var_eps2_dn3 = 0.0;
        locals.var_eps2_dn4 = 0.0;
        locals.var_eps2_dn5 = 0.0;
        locals.var_eps2_dn6 = 0.0;
        locals.var_eps2_dn7 = 0.0;
        locals.var_eps2_dn8 = 0.0;
        locals.var_eps2_dn9 = 0.0;
        locals.var_eps2_dn10 = 0.0;
        locals.var_eps2_rv = 0.0;

        let assign1410_e1510: f64 = (locals.var_bavl_t_tmp * locals.var_bavl_t_tmp);
        locals.var_x2 = assign1410_e1510;
        locals.var_x2_dn0 = 0.0;
        locals.var_x2_dn1 = 0.0;
        locals.var_x2_dn3 = ((locals.var_bavl_t_tmp_dn3 * locals.var_bavl_t_tmp) + (locals.var_bavl_t_tmp * locals.var_bavl_t_tmp_dn3));
        locals.var_x2_dn4 = 0.0;
        locals.var_x2_dn5 = 0.0;
        locals.var_x2_dn6 = 0.0;
        locals.var_x2_dn7 = 0.0;
        locals.var_x2_dn8 = 0.0;
        locals.var_x2_dn9 = 0.0;
        locals.var_x2_dn10 = 0.0;
        locals.var_x2_rv = 0.0;

        let assign1420_e1513: f64 = if locals.var_bavl_t_tmp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign1420_e1513;
        locals.var_guard21_rv = 0.0;

        let (assign1430_e1526, assign1430_e1526_d_n0, assign1430_e1526_d_n1, assign1430_e1526_d_n3, assign1430_e1526_d_n4, assign1430_e1526_d_n5, assign1430_e1526_d_n6, assign1430_e1526_d_n7, assign1430_e1526_d_n8, assign1430_e1526_d_n9, assign1430_e1526_d_n10,) = {
    if (locals.var_guard21 != 0.0) {
        let assign1430_e1517: f64 = (0.5 * locals.var_eps2);
        let assign1430_e1520: f64 = (locals.var_x2 + locals.var_eps2);
        let assign1430_e1521: f64 = (assign1430_e1520).sqrt();
        let assign1430_e1523: f64 = (assign1430_e1521 - locals.var_bavl_t_tmp);
        let assign1430_e1524: f64 = (assign1430_e1517 / assign1430_e1523);
        (assign1430_e1524, ((((0.5 * locals.var_eps2_dn0) * assign1430_e1523) - (assign1430_e1517 * ((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign1430_e1521)))) / (assign1430_e1523 * assign1430_e1523)), ((((0.5 * locals.var_eps2_dn1) * assign1430_e1523) - (assign1430_e1517 * ((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign1430_e1521)))) / (assign1430_e1523 * assign1430_e1523)), ((((0.5 * locals.var_eps2_dn3) * assign1430_e1523) - (assign1430_e1517 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign1430_e1521)) - locals.var_bavl_t_tmp_dn3))) / (assign1430_e1523 * assign1430_e1523)), ((((0.5 * locals.var_eps2_dn4) * assign1430_e1523) - (assign1430_e1517 * ((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign1430_e1521)))) / (assign1430_e1523 * assign1430_e1523)), ((((0.5 * locals.var_eps2_dn5) * assign1430_e1523) - (assign1430_e1517 * ((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign1430_e1521)))) / (assign1430_e1523 * assign1430_e1523)), ((((0.5 * locals.var_eps2_dn6) * assign1430_e1523) - (assign1430_e1517 * ((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign1430_e1521)))) / (assign1430_e1523 * assign1430_e1523)), ((((0.5 * locals.var_eps2_dn7) * assign1430_e1523) - (assign1430_e1517 * ((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign1430_e1521)))) / (assign1430_e1523 * assign1430_e1523)), ((((0.5 * locals.var_eps2_dn8) * assign1430_e1523) - (assign1430_e1517 * ((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign1430_e1521)))) / (assign1430_e1523 * assign1430_e1523)), ((((0.5 * locals.var_eps2_dn9) * assign1430_e1523) - (assign1430_e1517 * ((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign1430_e1521)))) / (assign1430_e1523 * assign1430_e1523)), ((((0.5 * locals.var_eps2_dn10) * assign1430_e1523) - (assign1430_e1517 * ((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign1430_e1521)))) / (assign1430_e1523 * assign1430_e1523)),)
    } else {
        (locals.var_bavl_t, locals.var_bavl_t_dn0, locals.var_bavl_t_dn1, locals.var_bavl_t_dn3, locals.var_bavl_t_dn4, locals.var_bavl_t_dn5, locals.var_bavl_t_dn6, locals.var_bavl_t_dn7, locals.var_bavl_t_dn8, locals.var_bavl_t_dn9, locals.var_bavl_t_dn10,)
    }
};
        locals.var_bavl_t = assign1430_e1526;
        locals.var_bavl_t_dn0 = assign1430_e1526_d_n0;
        locals.var_bavl_t_dn1 = assign1430_e1526_d_n1;
        locals.var_bavl_t_dn3 = assign1430_e1526_d_n3;
        locals.var_bavl_t_dn4 = assign1430_e1526_d_n4;
        locals.var_bavl_t_dn5 = assign1430_e1526_d_n5;
        locals.var_bavl_t_dn6 = assign1430_e1526_d_n6;
        locals.var_bavl_t_dn7 = assign1430_e1526_d_n7;
        locals.var_bavl_t_dn8 = assign1430_e1526_d_n8;
        locals.var_bavl_t_dn9 = assign1430_e1526_d_n9;
        locals.var_bavl_t_dn10 = assign1430_e1526_d_n10;
        locals.var_bavl_t_rv = 0.0;

        let (assign1440_e1538, assign1440_e1538_d_n0, assign1440_e1538_d_n1, assign1440_e1538_d_n3, assign1440_e1538_d_n4, assign1440_e1538_d_n5, assign1440_e1538_d_n6, assign1440_e1538_d_n7, assign1440_e1538_d_n8, assign1440_e1538_d_n9, assign1440_e1538_d_n10,) = {
    if (locals.var_guard21 == 0.0) {
        let assign1440_e1532: f64 = (locals.var_x2 + locals.var_eps2);
        let assign1440_e1533: f64 = (assign1440_e1532).sqrt();
        let assign1440_e1535: f64 = (assign1440_e1533 + locals.var_bavl_t_tmp);
        let assign1440_e1536: f64 = (0.5 * assign1440_e1535);
        (assign1440_e1536, (0.5 * ((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign1440_e1533))), (0.5 * ((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign1440_e1533))), (0.5 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign1440_e1533)) + locals.var_bavl_t_tmp_dn3)), (0.5 * ((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign1440_e1533))), (0.5 * ((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign1440_e1533))), (0.5 * ((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign1440_e1533))), (0.5 * ((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign1440_e1533))), (0.5 * ((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign1440_e1533))), (0.5 * ((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign1440_e1533))), (0.5 * ((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign1440_e1533))),)
    } else {
        (locals.var_bavl_t, locals.var_bavl_t_dn0, locals.var_bavl_t_dn1, locals.var_bavl_t_dn3, locals.var_bavl_t_dn4, locals.var_bavl_t_dn5, locals.var_bavl_t_dn6, locals.var_bavl_t_dn7, locals.var_bavl_t_dn8, locals.var_bavl_t_dn9, locals.var_bavl_t_dn10,)
    }
};
        locals.var_bavl_t = assign1440_e1538;
        locals.var_bavl_t_dn0 = assign1440_e1538_d_n0;
        locals.var_bavl_t_dn1 = assign1440_e1538_d_n1;
        locals.var_bavl_t_dn3 = assign1440_e1538_d_n3;
        locals.var_bavl_t_dn4 = assign1440_e1538_d_n4;
        locals.var_bavl_t_dn5 = assign1440_e1538_d_n5;
        locals.var_bavl_t_dn6 = assign1440_e1538_d_n6;
        locals.var_bavl_t_dn7 = assign1440_e1538_d_n7;
        locals.var_bavl_t_dn8 = assign1440_e1538_d_n8;
        locals.var_bavl_t_dn9 = assign1440_e1538_d_n9;
        locals.var_bavl_t_dn10 = assign1440_e1538_d_n10;
        locals.var_bavl_t_rv = 0.0;

        let assign1450_e1543: f64 = (4.0 - p.p97);
        let assign1450_e1545: f64 = (assign1450_e1543 - p.p95);
        let assign1450_e1547: f64 = (assign1450_e1545 + p.p120);
        let assign1450_e1548: f64 = (locals.var_lntn * assign1450_e1547);
        let assign1450_e1550: f64 = (assign1450_e1548 / locals.var_nff_t);
        let assign1450_e1551: f64 = (assign1450_e1550).exp();
        let assign1450_e1552: f64 = (p.p8 * assign1450_e1551);
        let assign1450_e1554: f64 = (-p.p104);
        let assign1450_e1556: f64 = (assign1450_e1554 * locals.var_vdtinv);
        let assign1450_e1558: f64 = (assign1450_e1556 / locals.var_nff_t);
        let assign1450_e1559: f64 = (assign1450_e1558).exp();
        let assign1450_e1560: f64 = (assign1450_e1552 * assign1450_e1559);
        locals.var_is_t = assign1450_e1560;
        locals.var_is_t_dn0 = (((p.p8 * (assign1450_e1551 * (-((assign1450_e1548 * locals.var_nff_t_dn0) / (locals.var_nff_t * locals.var_nff_t))))) * assign1450_e1559) + (assign1450_e1552 * (assign1450_e1559 * (-((assign1450_e1556 * locals.var_nff_t_dn0) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn1 = (((p.p8 * (assign1450_e1551 * (-((assign1450_e1548 * locals.var_nff_t_dn1) / (locals.var_nff_t * locals.var_nff_t))))) * assign1450_e1559) + (assign1450_e1552 * (assign1450_e1559 * (-((assign1450_e1556 * locals.var_nff_t_dn1) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn3 = (((p.p8 * (assign1450_e1551 * ((((locals.var_lntn_dn3 * assign1450_e1547) * locals.var_nff_t) - (assign1450_e1548 * locals.var_nff_t_dn3)) / (locals.var_nff_t * locals.var_nff_t)))) * assign1450_e1559) + (assign1450_e1552 * (assign1450_e1559 * ((((assign1450_e1554 * locals.var_vdtinv_dn3) * locals.var_nff_t) - (assign1450_e1556 * locals.var_nff_t_dn3)) / (locals.var_nff_t * locals.var_nff_t)))));
        locals.var_is_t_dn4 = (((p.p8 * (assign1450_e1551 * (-((assign1450_e1548 * locals.var_nff_t_dn4) / (locals.var_nff_t * locals.var_nff_t))))) * assign1450_e1559) + (assign1450_e1552 * (assign1450_e1559 * (-((assign1450_e1556 * locals.var_nff_t_dn4) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn5 = (((p.p8 * (assign1450_e1551 * (-((assign1450_e1548 * locals.var_nff_t_dn5) / (locals.var_nff_t * locals.var_nff_t))))) * assign1450_e1559) + (assign1450_e1552 * (assign1450_e1559 * (-((assign1450_e1556 * locals.var_nff_t_dn5) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn6 = (((p.p8 * (assign1450_e1551 * (-((assign1450_e1548 * locals.var_nff_t_dn6) / (locals.var_nff_t * locals.var_nff_t))))) * assign1450_e1559) + (assign1450_e1552 * (assign1450_e1559 * (-((assign1450_e1556 * locals.var_nff_t_dn6) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn7 = (((p.p8 * (assign1450_e1551 * (-((assign1450_e1548 * locals.var_nff_t_dn7) / (locals.var_nff_t * locals.var_nff_t))))) * assign1450_e1559) + (assign1450_e1552 * (assign1450_e1559 * (-((assign1450_e1556 * locals.var_nff_t_dn7) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn8 = (((p.p8 * (assign1450_e1551 * (-((assign1450_e1548 * locals.var_nff_t_dn8) / (locals.var_nff_t * locals.var_nff_t))))) * assign1450_e1559) + (assign1450_e1552 * (assign1450_e1559 * (-((assign1450_e1556 * locals.var_nff_t_dn8) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn9 = (((p.p8 * (assign1450_e1551 * (-((assign1450_e1548 * locals.var_nff_t_dn9) / (locals.var_nff_t * locals.var_nff_t))))) * assign1450_e1559) + (assign1450_e1552 * (assign1450_e1559 * (-((assign1450_e1556 * locals.var_nff_t_dn9) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn10 = (((p.p8 * (assign1450_e1551 * (-((assign1450_e1548 * locals.var_nff_t_dn10) / (locals.var_nff_t * locals.var_nff_t))))) * assign1450_e1559) + (assign1450_e1552 * (assign1450_e1559 * (-((assign1450_e1556 * locals.var_nff_t_dn10) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_rv = 0.0;

        let assign1460_e1565: f64 = (1.0 - p.p97);
        let assign1460_e1566: f64 = (locals.var_lntn * assign1460_e1565);
        let assign1460_e1567: f64 = (assign1460_e1566).exp();
        let assign1460_e1568: f64 = (p.p11 * assign1460_e1567);
        locals.var_ik_t = assign1460_e1568;
        locals.var_ik_t_dn3 = (p.p11 * (assign1460_e1567 * (locals.var_lntn_dn3 * assign1460_e1565)));
        locals.var_ik_t_rv = 0.0;

        let assign1470_e1573: f64 = (1.0 - p.p102);
        let assign1470_e1574: f64 = (locals.var_lntn * assign1470_e1573);
        let assign1470_e1575: f64 = (assign1470_e1574).exp();
        let assign1470_e1576: f64 = (p.p29 * assign1470_e1575);
        locals.var_ikbx_t = assign1470_e1576;
        locals.var_ikbx_t_dn3 = (p.p29 * (assign1470_e1575 * (locals.var_lntn_dn3 * assign1470_e1573)));
        locals.var_ikbx_t_rv = 0.0;

        let assign1500_e1617: f64 = (4.0 - p.p96);
        let assign1500_e1619: f64 = (assign1500_e1617 + p.p120);
        let assign1500_e1620: f64 = (locals.var_lntn * assign1500_e1619);
        let assign1500_e1622: f64 = (assign1500_e1620 / p.p16);
        let assign1500_e1623: f64 = (assign1500_e1622).exp();
        let assign1500_e1624: f64 = (p.p15 * assign1500_e1623);
        let assign1500_e1626: f64 = (-p.p110);
        let assign1500_e1628: f64 = (assign1500_e1626 * locals.var_vdtinv);
        let assign1500_e1630: f64 = (assign1500_e1628 / p.p16);
        let assign1500_e1631: f64 = (assign1500_e1630).exp();
        let assign1500_e1632: f64 = (assign1500_e1624 * assign1500_e1631);
        locals.var_ibi_t = assign1500_e1632;
        locals.var_ibi_t_dn3 = (((p.p15 * (assign1500_e1623 * ((locals.var_lntn_dn3 * assign1500_e1619) / p.p16))) * assign1500_e1631) + (assign1500_e1624 * (assign1500_e1631 * ((assign1500_e1626 * locals.var_vdtinv_dn3) / p.p16))));
        locals.var_ibi_t_rv = 0.0;

        let assign1560_e1694: f64 = (4.0 - p.p102);
        let assign1560_e1696: f64 = (assign1560_e1694 + p.p120);
        let assign1560_e1697: f64 = (locals.var_lntn * assign1560_e1696);
        let assign1560_e1698: f64 = (assign1560_e1697).exp();
        let assign1560_e1699: f64 = (p.p28 * assign1560_e1698);
        let assign1560_e1701: f64 = (-p.p111);
        let assign1560_e1703: f64 = (assign1560_e1701 * locals.var_vdtinv);
        let assign1560_e1704: f64 = (assign1560_e1703).exp();
        let assign1560_e1705: f64 = (assign1560_e1699 * assign1560_e1704);
        locals.var_ibx_t = assign1560_e1705;
        locals.var_ibx_t_dn3 = (((p.p28 * (assign1560_e1698 * (locals.var_lntn_dn3 * assign1560_e1696))) * assign1560_e1704) + (assign1560_e1699 * (assign1560_e1704 * (assign1560_e1701 * locals.var_vdtinv_dn3))));
        locals.var_ibx_t_rv = 0.0;

        let assign1600_e1751: f64 = (locals.var_vgzeb_t * locals.var_inv_vgzeb_tr);
        let assign1600_e1753: f64 = (-0.5);
        let assign1600_e1754: f64 = (assign1600_e1751).powf(assign1600_e1753);
        locals.var_x = assign1600_e1754;
        locals.var_x_dn0 = if 0.0 == 0.0 && ((assign1600_e1753) as f64).is_finite() && ((assign1600_e1753) as f64).fract() == 0.0 { if assign1600_e1753 == 0.0 { 0.0 } else { (assign1600_e1753 * ((assign1600_e1751).powf(assign1600_e1753 - 1.0) * (locals.var_vgzeb_t_dn0 * locals.var_inv_vgzeb_tr))) } } else { (assign1600_e1754 * (assign1600_e1753 * ((locals.var_vgzeb_t_dn0 * locals.var_inv_vgzeb_tr) / assign1600_e1751))) };
        locals.var_x_dn1 = if 0.0 == 0.0 && ((assign1600_e1753) as f64).is_finite() && ((assign1600_e1753) as f64).fract() == 0.0 { if assign1600_e1753 == 0.0 { 0.0 } else { (assign1600_e1753 * ((assign1600_e1751).powf(assign1600_e1753 - 1.0) * (locals.var_vgzeb_t_dn1 * locals.var_inv_vgzeb_tr))) } } else { (assign1600_e1754 * (assign1600_e1753 * ((locals.var_vgzeb_t_dn1 * locals.var_inv_vgzeb_tr) / assign1600_e1751))) };
        locals.var_x_dn3 = if 0.0 == 0.0 && ((assign1600_e1753) as f64).is_finite() && ((assign1600_e1753) as f64).fract() == 0.0 { if assign1600_e1753 == 0.0 { 0.0 } else { (assign1600_e1753 * ((assign1600_e1751).powf(assign1600_e1753 - 1.0) * (locals.var_vgzeb_t_dn3 * locals.var_inv_vgzeb_tr))) } } else { (assign1600_e1754 * (assign1600_e1753 * ((locals.var_vgzeb_t_dn3 * locals.var_inv_vgzeb_tr) / assign1600_e1751))) };
        locals.var_x_dn4 = if 0.0 == 0.0 && ((assign1600_e1753) as f64).is_finite() && ((assign1600_e1753) as f64).fract() == 0.0 { if assign1600_e1753 == 0.0 { 0.0 } else { (assign1600_e1753 * ((assign1600_e1751).powf(assign1600_e1753 - 1.0) * (locals.var_vgzeb_t_dn4 * locals.var_inv_vgzeb_tr))) } } else { (assign1600_e1754 * (assign1600_e1753 * ((locals.var_vgzeb_t_dn4 * locals.var_inv_vgzeb_tr) / assign1600_e1751))) };
        locals.var_x_dn5 = if 0.0 == 0.0 && ((assign1600_e1753) as f64).is_finite() && ((assign1600_e1753) as f64).fract() == 0.0 { if assign1600_e1753 == 0.0 { 0.0 } else { (assign1600_e1753 * ((assign1600_e1751).powf(assign1600_e1753 - 1.0) * (locals.var_vgzeb_t_dn5 * locals.var_inv_vgzeb_tr))) } } else { (assign1600_e1754 * (assign1600_e1753 * ((locals.var_vgzeb_t_dn5 * locals.var_inv_vgzeb_tr) / assign1600_e1751))) };
        locals.var_x_dn6 = if 0.0 == 0.0 && ((assign1600_e1753) as f64).is_finite() && ((assign1600_e1753) as f64).fract() == 0.0 { if assign1600_e1753 == 0.0 { 0.0 } else { (assign1600_e1753 * ((assign1600_e1751).powf(assign1600_e1753 - 1.0) * (locals.var_vgzeb_t_dn6 * locals.var_inv_vgzeb_tr))) } } else { (assign1600_e1754 * (assign1600_e1753 * ((locals.var_vgzeb_t_dn6 * locals.var_inv_vgzeb_tr) / assign1600_e1751))) };
        locals.var_x_dn7 = if 0.0 == 0.0 && ((assign1600_e1753) as f64).is_finite() && ((assign1600_e1753) as f64).fract() == 0.0 { if assign1600_e1753 == 0.0 { 0.0 } else { (assign1600_e1753 * ((assign1600_e1751).powf(assign1600_e1753 - 1.0) * (locals.var_vgzeb_t_dn7 * locals.var_inv_vgzeb_tr))) } } else { (assign1600_e1754 * (assign1600_e1753 * ((locals.var_vgzeb_t_dn7 * locals.var_inv_vgzeb_tr) / assign1600_e1751))) };
        locals.var_x_dn8 = if 0.0 == 0.0 && ((assign1600_e1753) as f64).is_finite() && ((assign1600_e1753) as f64).fract() == 0.0 { if assign1600_e1753 == 0.0 { 0.0 } else { (assign1600_e1753 * ((assign1600_e1751).powf(assign1600_e1753 - 1.0) * (locals.var_vgzeb_t_dn8 * locals.var_inv_vgzeb_tr))) } } else { (assign1600_e1754 * (assign1600_e1753 * ((locals.var_vgzeb_t_dn8 * locals.var_inv_vgzeb_tr) / assign1600_e1751))) };
        locals.var_x_dn9 = if 0.0 == 0.0 && ((assign1600_e1753) as f64).is_finite() && ((assign1600_e1753) as f64).fract() == 0.0 { if assign1600_e1753 == 0.0 { 0.0 } else { (assign1600_e1753 * ((assign1600_e1751).powf(assign1600_e1753 - 1.0) * (locals.var_vgzeb_t_dn9 * locals.var_inv_vgzeb_tr))) } } else { (assign1600_e1754 * (assign1600_e1753 * ((locals.var_vgzeb_t_dn9 * locals.var_inv_vgzeb_tr) / assign1600_e1751))) };
        locals.var_x_dn10 = if 0.0 == 0.0 && ((assign1600_e1753) as f64).is_finite() && ((assign1600_e1753) as f64).fract() == 0.0 { if assign1600_e1753 == 0.0 { 0.0 } else { (assign1600_e1753 * ((assign1600_e1751).powf(assign1600_e1753 - 1.0) * (locals.var_vgzeb_t_dn10 * locals.var_inv_vgzeb_tr))) } } else { (assign1600_e1754 * (assign1600_e1753 * ((locals.var_vgzeb_t_dn10 * locals.var_inv_vgzeb_tr) / assign1600_e1751))) };
        locals.var_x_rv = 0.0;

        let assign1610_e1757: f64 = (1.0 / locals.var_cje_t_div_cje);
        locals.var_y = assign1610_e1757;
        locals.var_y_dn0 = (-(locals.var_cje_t_div_cje_dn0 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn1 = (-(locals.var_cje_t_div_cje_dn1 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn3 = (-(locals.var_cje_t_div_cje_dn3 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn4 = (-(locals.var_cje_t_div_cje_dn4 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn5 = (-(locals.var_cje_t_div_cje_dn5 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn6 = (-(locals.var_cje_t_div_cje_dn6 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn7 = (-(locals.var_cje_t_div_cje_dn7 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn8 = (-(locals.var_cje_t_div_cje_dn8 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn9 = (-(locals.var_cje_t_div_cje_dn9 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn10 = (-(locals.var_cje_t_div_cje_dn10 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_rv = 0.0;

        let assign1620_e1760: f64 = (p.p34 * locals.var_vgzeb_t);
        let assign1620_e1762: f64 = (assign1620_e1760 * locals.var_vgzeb_t);
        let assign1620_e1764: f64 = (assign1620_e1762 * locals.var_x);
        let assign1620_e1766: f64 = (assign1620_e1764 * locals.var_y);
        let assign1620_e1768: f64 = (assign1620_e1766 * p.p65);
        let assign1620_e1770: f64 = (assign1620_e1768 * locals.var_inv_vde_t);
        let assign1620_e1772: f64 = (assign1620_e1770 * locals.var_inv_vgzeb_tr);
        let assign1620_e1774: f64 = (assign1620_e1772 * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t = assign1620_e1774;
        locals.var_nzeb_t_dn0 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn0) * locals.var_vgzeb_t) + (assign1620_e1760 * locals.var_vgzeb_t_dn0)) * locals.var_x) + (assign1620_e1762 * locals.var_x_dn0)) * locals.var_y) + (assign1620_e1764 * locals.var_y_dn0)) * p.p65) * locals.var_inv_vde_t) + (assign1620_e1768 * locals.var_inv_vde_t_dn0)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn1 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn1) * locals.var_vgzeb_t) + (assign1620_e1760 * locals.var_vgzeb_t_dn1)) * locals.var_x) + (assign1620_e1762 * locals.var_x_dn1)) * locals.var_y) + (assign1620_e1764 * locals.var_y_dn1)) * p.p65) * locals.var_inv_vde_t) + (assign1620_e1768 * locals.var_inv_vde_t_dn1)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn3 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn3) * locals.var_vgzeb_t) + (assign1620_e1760 * locals.var_vgzeb_t_dn3)) * locals.var_x) + (assign1620_e1762 * locals.var_x_dn3)) * locals.var_y) + (assign1620_e1764 * locals.var_y_dn3)) * p.p65) * locals.var_inv_vde_t) + (assign1620_e1768 * locals.var_inv_vde_t_dn3)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn4 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn4) * locals.var_vgzeb_t) + (assign1620_e1760 * locals.var_vgzeb_t_dn4)) * locals.var_x) + (assign1620_e1762 * locals.var_x_dn4)) * locals.var_y) + (assign1620_e1764 * locals.var_y_dn4)) * p.p65) * locals.var_inv_vde_t) + (assign1620_e1768 * locals.var_inv_vde_t_dn4)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn5 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn5) * locals.var_vgzeb_t) + (assign1620_e1760 * locals.var_vgzeb_t_dn5)) * locals.var_x) + (assign1620_e1762 * locals.var_x_dn5)) * locals.var_y) + (assign1620_e1764 * locals.var_y_dn5)) * p.p65) * locals.var_inv_vde_t) + (assign1620_e1768 * locals.var_inv_vde_t_dn5)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn6 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn6) * locals.var_vgzeb_t) + (assign1620_e1760 * locals.var_vgzeb_t_dn6)) * locals.var_x) + (assign1620_e1762 * locals.var_x_dn6)) * locals.var_y) + (assign1620_e1764 * locals.var_y_dn6)) * p.p65) * locals.var_inv_vde_t) + (assign1620_e1768 * locals.var_inv_vde_t_dn6)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn7 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn7) * locals.var_vgzeb_t) + (assign1620_e1760 * locals.var_vgzeb_t_dn7)) * locals.var_x) + (assign1620_e1762 * locals.var_x_dn7)) * locals.var_y) + (assign1620_e1764 * locals.var_y_dn7)) * p.p65) * locals.var_inv_vde_t) + (assign1620_e1768 * locals.var_inv_vde_t_dn7)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn8 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn8) * locals.var_vgzeb_t) + (assign1620_e1760 * locals.var_vgzeb_t_dn8)) * locals.var_x) + (assign1620_e1762 * locals.var_x_dn8)) * locals.var_y) + (assign1620_e1764 * locals.var_y_dn8)) * p.p65) * locals.var_inv_vde_t) + (assign1620_e1768 * locals.var_inv_vde_t_dn8)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn9 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn9) * locals.var_vgzeb_t) + (assign1620_e1760 * locals.var_vgzeb_t_dn9)) * locals.var_x) + (assign1620_e1762 * locals.var_x_dn9)) * locals.var_y) + (assign1620_e1764 * locals.var_y_dn9)) * p.p65) * locals.var_inv_vde_t) + (assign1620_e1768 * locals.var_inv_vde_t_dn9)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn10 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn10) * locals.var_vgzeb_t) + (assign1620_e1760 * locals.var_vgzeb_t_dn10)) * locals.var_x) + (assign1620_e1762 * locals.var_x_dn10)) * locals.var_y) + (assign1620_e1764 * locals.var_y_dn10)) * p.p65) * locals.var_inv_vde_t) + (assign1620_e1768 * locals.var_inv_vde_t_dn10)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_rv = 0.0;

        let assign1640_e1795: f64 = (1.0 / locals.var_vdc_zener_t);
        locals.var_inv_vdc_zener_t = assign1640_e1795;
        locals.var_inv_vdc_zener_t_dn0 = (-(locals.var_vdc_zener_t_dn0 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn1 = (-(locals.var_vdc_zener_t_dn1 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn3 = (-(locals.var_vdc_zener_t_dn3 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn4 = (-(locals.var_vdc_zener_t_dn4 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn5 = (-(locals.var_vdc_zener_t_dn5 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn6 = (-(locals.var_vdc_zener_t_dn6 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn7 = (-(locals.var_vdc_zener_t_dn7 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn8 = (-(locals.var_vdc_zener_t_dn8 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn9 = (-(locals.var_vdc_zener_t_dn9 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn10 = (-(locals.var_vdc_zener_t_dn10 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_rv = 0.0;

        let assign1650_e1798: f64 = (locals.var_vgzcb_t * locals.var_inv_vgzcb_tr);
        let assign1650_e1800: f64 = (-0.5);
        let assign1650_e1801: f64 = (assign1650_e1798).powf(assign1650_e1800);
        locals.var_xx = assign1650_e1801;
        locals.var_xx_dn0 = if 0.0 == 0.0 && ((assign1650_e1800) as f64).is_finite() && ((assign1650_e1800) as f64).fract() == 0.0 { if assign1650_e1800 == 0.0 { 0.0 } else { (assign1650_e1800 * ((assign1650_e1798).powf(assign1650_e1800 - 1.0) * (locals.var_vgzcb_t_dn0 * locals.var_inv_vgzcb_tr))) } } else { (assign1650_e1801 * (assign1650_e1800 * ((locals.var_vgzcb_t_dn0 * locals.var_inv_vgzcb_tr) / assign1650_e1798))) };
        locals.var_xx_dn1 = if 0.0 == 0.0 && ((assign1650_e1800) as f64).is_finite() && ((assign1650_e1800) as f64).fract() == 0.0 { if assign1650_e1800 == 0.0 { 0.0 } else { (assign1650_e1800 * ((assign1650_e1798).powf(assign1650_e1800 - 1.0) * (locals.var_vgzcb_t_dn1 * locals.var_inv_vgzcb_tr))) } } else { (assign1650_e1801 * (assign1650_e1800 * ((locals.var_vgzcb_t_dn1 * locals.var_inv_vgzcb_tr) / assign1650_e1798))) };
        locals.var_xx_dn3 = if 0.0 == 0.0 && ((assign1650_e1800) as f64).is_finite() && ((assign1650_e1800) as f64).fract() == 0.0 { if assign1650_e1800 == 0.0 { 0.0 } else { (assign1650_e1800 * ((assign1650_e1798).powf(assign1650_e1800 - 1.0) * (locals.var_vgzcb_t_dn3 * locals.var_inv_vgzcb_tr))) } } else { (assign1650_e1801 * (assign1650_e1800 * ((locals.var_vgzcb_t_dn3 * locals.var_inv_vgzcb_tr) / assign1650_e1798))) };
        locals.var_xx_dn4 = if 0.0 == 0.0 && ((assign1650_e1800) as f64).is_finite() && ((assign1650_e1800) as f64).fract() == 0.0 { if assign1650_e1800 == 0.0 { 0.0 } else { (assign1650_e1800 * ((assign1650_e1798).powf(assign1650_e1800 - 1.0) * (locals.var_vgzcb_t_dn4 * locals.var_inv_vgzcb_tr))) } } else { (assign1650_e1801 * (assign1650_e1800 * ((locals.var_vgzcb_t_dn4 * locals.var_inv_vgzcb_tr) / assign1650_e1798))) };
        locals.var_xx_dn5 = if 0.0 == 0.0 && ((assign1650_e1800) as f64).is_finite() && ((assign1650_e1800) as f64).fract() == 0.0 { if assign1650_e1800 == 0.0 { 0.0 } else { (assign1650_e1800 * ((assign1650_e1798).powf(assign1650_e1800 - 1.0) * (locals.var_vgzcb_t_dn5 * locals.var_inv_vgzcb_tr))) } } else { (assign1650_e1801 * (assign1650_e1800 * ((locals.var_vgzcb_t_dn5 * locals.var_inv_vgzcb_tr) / assign1650_e1798))) };
        locals.var_xx_dn6 = if 0.0 == 0.0 && ((assign1650_e1800) as f64).is_finite() && ((assign1650_e1800) as f64).fract() == 0.0 { if assign1650_e1800 == 0.0 { 0.0 } else { (assign1650_e1800 * ((assign1650_e1798).powf(assign1650_e1800 - 1.0) * (locals.var_vgzcb_t_dn6 * locals.var_inv_vgzcb_tr))) } } else { (assign1650_e1801 * (assign1650_e1800 * ((locals.var_vgzcb_t_dn6 * locals.var_inv_vgzcb_tr) / assign1650_e1798))) };
        locals.var_xx_dn7 = if 0.0 == 0.0 && ((assign1650_e1800) as f64).is_finite() && ((assign1650_e1800) as f64).fract() == 0.0 { if assign1650_e1800 == 0.0 { 0.0 } else { (assign1650_e1800 * ((assign1650_e1798).powf(assign1650_e1800 - 1.0) * (locals.var_vgzcb_t_dn7 * locals.var_inv_vgzcb_tr))) } } else { (assign1650_e1801 * (assign1650_e1800 * ((locals.var_vgzcb_t_dn7 * locals.var_inv_vgzcb_tr) / assign1650_e1798))) };
        locals.var_xx_dn8 = if 0.0 == 0.0 && ((assign1650_e1800) as f64).is_finite() && ((assign1650_e1800) as f64).fract() == 0.0 { if assign1650_e1800 == 0.0 { 0.0 } else { (assign1650_e1800 * ((assign1650_e1798).powf(assign1650_e1800 - 1.0) * (locals.var_vgzcb_t_dn8 * locals.var_inv_vgzcb_tr))) } } else { (assign1650_e1801 * (assign1650_e1800 * ((locals.var_vgzcb_t_dn8 * locals.var_inv_vgzcb_tr) / assign1650_e1798))) };
        locals.var_xx_dn9 = if 0.0 == 0.0 && ((assign1650_e1800) as f64).is_finite() && ((assign1650_e1800) as f64).fract() == 0.0 { if assign1650_e1800 == 0.0 { 0.0 } else { (assign1650_e1800 * ((assign1650_e1798).powf(assign1650_e1800 - 1.0) * (locals.var_vgzcb_t_dn9 * locals.var_inv_vgzcb_tr))) } } else { (assign1650_e1801 * (assign1650_e1800 * ((locals.var_vgzcb_t_dn9 * locals.var_inv_vgzcb_tr) / assign1650_e1798))) };
        locals.var_xx_dn10 = if 0.0 == 0.0 && ((assign1650_e1800) as f64).is_finite() && ((assign1650_e1800) as f64).fract() == 0.0 { if assign1650_e1800 == 0.0 { 0.0 } else { (assign1650_e1800 * ((assign1650_e1798).powf(assign1650_e1800 - 1.0) * (locals.var_vgzcb_t_dn10 * locals.var_inv_vgzcb_tr))) } } else { (assign1650_e1801 * (assign1650_e1800 * ((locals.var_vgzcb_t_dn10 * locals.var_inv_vgzcb_tr) / assign1650_e1798))) };
        locals.var_xx_rv = 0.0;

        let assign1660_e1804: f64 = (1.0 / locals.var_cjc_t_div_cjc_zener);
        locals.var_yy = assign1660_e1804;
        locals.var_yy_dn0 = (-(locals.var_cjc_t_div_cjc_zener_dn0 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn1 = (-(locals.var_cjc_t_div_cjc_zener_dn1 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn3 = (-(locals.var_cjc_t_div_cjc_zener_dn3 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn4 = (-(locals.var_cjc_t_div_cjc_zener_dn4 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn5 = (-(locals.var_cjc_t_div_cjc_zener_dn5 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn6 = (-(locals.var_cjc_t_div_cjc_zener_dn6 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn7 = (-(locals.var_cjc_t_div_cjc_zener_dn7 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn8 = (-(locals.var_cjc_t_div_cjc_zener_dn8 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn9 = (-(locals.var_cjc_t_div_cjc_zener_dn9 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn10 = (-(locals.var_cjc_t_div_cjc_zener_dn10 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_rv = 0.0;

        let assign1670_e1807: f64 = (p.p36 * locals.var_vgzcb_t);
        let assign1670_e1809: f64 = (assign1670_e1807 * locals.var_vgzcb_t);
        let assign1670_e1811: f64 = (assign1670_e1809 * locals.var_xx);
        let assign1670_e1813: f64 = (assign1670_e1811 * locals.var_yy);
        let assign1670_e1815: f64 = (assign1670_e1813 * locals.var_vdc_zener);
        let assign1670_e1817: f64 = (assign1670_e1815 * locals.var_inv_vdc_zener_t);
        let assign1670_e1819: f64 = (assign1670_e1817 * locals.var_inv_vgzcb_tr);
        let assign1670_e1821: f64 = (assign1670_e1819 * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t = assign1670_e1821;
        locals.var_nzcb_t_dn0 = ((((((((((((p.p36 * locals.var_vgzcb_t_dn0) * locals.var_vgzcb_t) + (assign1670_e1807 * locals.var_vgzcb_t_dn0)) * locals.var_xx) + (assign1670_e1809 * locals.var_xx_dn0)) * locals.var_yy) + (assign1670_e1811 * locals.var_yy_dn0)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1670_e1815 * locals.var_inv_vdc_zener_t_dn0)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn1 = ((((((((((((p.p36 * locals.var_vgzcb_t_dn1) * locals.var_vgzcb_t) + (assign1670_e1807 * locals.var_vgzcb_t_dn1)) * locals.var_xx) + (assign1670_e1809 * locals.var_xx_dn1)) * locals.var_yy) + (assign1670_e1811 * locals.var_yy_dn1)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1670_e1815 * locals.var_inv_vdc_zener_t_dn1)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn3 = ((((((((((((p.p36 * locals.var_vgzcb_t_dn3) * locals.var_vgzcb_t) + (assign1670_e1807 * locals.var_vgzcb_t_dn3)) * locals.var_xx) + (assign1670_e1809 * locals.var_xx_dn3)) * locals.var_yy) + (assign1670_e1811 * locals.var_yy_dn3)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1670_e1815 * locals.var_inv_vdc_zener_t_dn3)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn4 = ((((((((((((p.p36 * locals.var_vgzcb_t_dn4) * locals.var_vgzcb_t) + (assign1670_e1807 * locals.var_vgzcb_t_dn4)) * locals.var_xx) + (assign1670_e1809 * locals.var_xx_dn4)) * locals.var_yy) + (assign1670_e1811 * locals.var_yy_dn4)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1670_e1815 * locals.var_inv_vdc_zener_t_dn4)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn5 = ((((((((((((p.p36 * locals.var_vgzcb_t_dn5) * locals.var_vgzcb_t) + (assign1670_e1807 * locals.var_vgzcb_t_dn5)) * locals.var_xx) + (assign1670_e1809 * locals.var_xx_dn5)) * locals.var_yy) + (assign1670_e1811 * locals.var_yy_dn5)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1670_e1815 * locals.var_inv_vdc_zener_t_dn5)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn6 = ((((((((((((p.p36 * locals.var_vgzcb_t_dn6) * locals.var_vgzcb_t) + (assign1670_e1807 * locals.var_vgzcb_t_dn6)) * locals.var_xx) + (assign1670_e1809 * locals.var_xx_dn6)) * locals.var_yy) + (assign1670_e1811 * locals.var_yy_dn6)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1670_e1815 * locals.var_inv_vdc_zener_t_dn6)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn7 = ((((((((((((p.p36 * locals.var_vgzcb_t_dn7) * locals.var_vgzcb_t) + (assign1670_e1807 * locals.var_vgzcb_t_dn7)) * locals.var_xx) + (assign1670_e1809 * locals.var_xx_dn7)) * locals.var_yy) + (assign1670_e1811 * locals.var_yy_dn7)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1670_e1815 * locals.var_inv_vdc_zener_t_dn7)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn8 = ((((((((((((p.p36 * locals.var_vgzcb_t_dn8) * locals.var_vgzcb_t) + (assign1670_e1807 * locals.var_vgzcb_t_dn8)) * locals.var_xx) + (assign1670_e1809 * locals.var_xx_dn8)) * locals.var_yy) + (assign1670_e1811 * locals.var_yy_dn8)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1670_e1815 * locals.var_inv_vdc_zener_t_dn8)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn9 = ((((((((((((p.p36 * locals.var_vgzcb_t_dn9) * locals.var_vgzcb_t) + (assign1670_e1807 * locals.var_vgzcb_t_dn9)) * locals.var_xx) + (assign1670_e1809 * locals.var_xx_dn9)) * locals.var_yy) + (assign1670_e1811 * locals.var_yy_dn9)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1670_e1815 * locals.var_inv_vdc_zener_t_dn9)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn10 = ((((((((((((p.p36 * locals.var_vgzcb_t_dn10) * locals.var_vgzcb_t) + (assign1670_e1807 * locals.var_vgzcb_t_dn10)) * locals.var_xx) + (assign1670_e1809 * locals.var_xx_dn10)) * locals.var_yy) + (assign1670_e1811 * locals.var_yy_dn10)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1670_e1815 * locals.var_inv_vdc_zener_t_dn10)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_rv = 0.0;

        let assign1690_e1842: f64 = (locals.var_lntn * p.p95);
        let assign1690_e1843: f64 = (assign1690_e1842).exp();
        locals.var_x = assign1690_e1843;
        locals.var_x_dn0 = 0.0;
        locals.var_x_dn1 = 0.0;
        locals.var_x_dn3 = (assign1690_e1843 * (locals.var_lntn_dn3 * p.p95));
        locals.var_x_dn4 = 0.0;
        locals.var_x_dn5 = 0.0;
        locals.var_x_dn6 = 0.0;
        locals.var_x_dn7 = 0.0;
        locals.var_x_dn8 = 0.0;
        locals.var_x_dn9 = 0.0;
        locals.var_x_dn10 = 0.0;
        locals.var_x_rv = 0.0;

        let assign1700_e1846: f64 = (p.p13 * locals.var_x);
        let assign1700_e1848: f64 = (assign1700_e1846 * locals.var_cjc_scale_inv);
        locals.var_vef_t = assign1700_e1848;
        locals.var_vef_t_dn0 = (((p.p13 * locals.var_x_dn0) * locals.var_cjc_scale_inv) + (assign1700_e1846 * locals.var_cjc_scale_inv_dn0));
        locals.var_vef_t_dn1 = (((p.p13 * locals.var_x_dn1) * locals.var_cjc_scale_inv) + (assign1700_e1846 * locals.var_cjc_scale_inv_dn1));
        locals.var_vef_t_dn3 = (((p.p13 * locals.var_x_dn3) * locals.var_cjc_scale_inv) + (assign1700_e1846 * locals.var_cjc_scale_inv_dn3));
        locals.var_vef_t_dn4 = (((p.p13 * locals.var_x_dn4) * locals.var_cjc_scale_inv) + (assign1700_e1846 * locals.var_cjc_scale_inv_dn4));
        locals.var_vef_t_dn5 = (((p.p13 * locals.var_x_dn5) * locals.var_cjc_scale_inv) + (assign1700_e1846 * locals.var_cjc_scale_inv_dn5));
        locals.var_vef_t_dn6 = (((p.p13 * locals.var_x_dn6) * locals.var_cjc_scale_inv) + (assign1700_e1846 * locals.var_cjc_scale_inv_dn6));
        locals.var_vef_t_dn7 = (((p.p13 * locals.var_x_dn7) * locals.var_cjc_scale_inv) + (assign1700_e1846 * locals.var_cjc_scale_inv_dn7));
        locals.var_vef_t_dn8 = (((p.p13 * locals.var_x_dn8) * locals.var_cjc_scale_inv) + (assign1700_e1846 * locals.var_cjc_scale_inv_dn8));
        locals.var_vef_t_dn9 = (((p.p13 * locals.var_x_dn9) * locals.var_cjc_scale_inv) + (assign1700_e1846 * locals.var_cjc_scale_inv_dn9));
        locals.var_vef_t_dn10 = (((p.p13 * locals.var_x_dn10) * locals.var_cjc_scale_inv) + (assign1700_e1846 * locals.var_cjc_scale_inv_dn10));
        locals.var_vef_t_rv = 0.0;

        let assign1710_e1851: f64 = (p.p12 * locals.var_x);
        let assign1710_e1853: f64 = (assign1710_e1851 * locals.var_y);
        locals.var_ver_t = assign1710_e1853;
        locals.var_ver_t_dn0 = (((p.p12 * locals.var_x_dn0) * locals.var_y) + (assign1710_e1851 * locals.var_y_dn0));
        locals.var_ver_t_dn1 = (((p.p12 * locals.var_x_dn1) * locals.var_y) + (assign1710_e1851 * locals.var_y_dn1));
        locals.var_ver_t_dn3 = (((p.p12 * locals.var_x_dn3) * locals.var_y) + (assign1710_e1851 * locals.var_y_dn3));
        locals.var_ver_t_dn4 = (((p.p12 * locals.var_x_dn4) * locals.var_y) + (assign1710_e1851 * locals.var_y_dn4));
        locals.var_ver_t_dn5 = (((p.p12 * locals.var_x_dn5) * locals.var_y) + (assign1710_e1851 * locals.var_y_dn5));
        locals.var_ver_t_dn6 = (((p.p12 * locals.var_x_dn6) * locals.var_y) + (assign1710_e1851 * locals.var_y_dn6));
        locals.var_ver_t_dn7 = (((p.p12 * locals.var_x_dn7) * locals.var_y) + (assign1710_e1851 * locals.var_y_dn7));
        locals.var_ver_t_dn8 = (((p.p12 * locals.var_x_dn8) * locals.var_y) + (assign1710_e1851 * locals.var_y_dn8));
        locals.var_ver_t_dn9 = (((p.p12 * locals.var_x_dn9) * locals.var_y) + (assign1710_e1851 * locals.var_y_dn9));
        locals.var_ver_t_dn10 = (((p.p12 * locals.var_x_dn10) * locals.var_y) + (assign1710_e1851 * locals.var_y_dn10));
        locals.var_ver_t_rv = 0.0;

        let assign1720_e1858: f64 = (p.p97 - 2.0);
        let assign1720_e1859: f64 = (locals.var_lntn * assign1720_e1858);
        let assign1720_e1860: f64 = (assign1720_e1859).exp();
        let assign1720_e1861: f64 = (p.p85 * assign1720_e1860);
        let assign1720_e1863: f64 = (-p.p119);
        let assign1720_e1865: f64 = (assign1720_e1863 * locals.var_vdtinv);
        let assign1720_e1866: f64 = (assign1720_e1865).exp();
        let assign1720_e1867: f64 = (assign1720_e1861 * assign1720_e1866);
        locals.var_taue_t = assign1720_e1867;
        locals.var_taue_t_dn3 = (((p.p85 * (assign1720_e1860 * (locals.var_lntn_dn3 * assign1720_e1858))) * assign1720_e1866) + (assign1720_e1861 * (assign1720_e1866 * (assign1720_e1863 * locals.var_vdtinv_dn3))));
        locals.var_taue_t_rv = 0.0;

        let assign1730_e1872: f64 = (p.p95 + p.p97);
        let assign1730_e1874: f64 = (assign1730_e1872 - 1.0);
        let assign1730_e1875: f64 = (locals.var_lntn * assign1730_e1874);
        let assign1730_e1876: f64 = (assign1730_e1875).exp();
        let assign1730_e1877: f64 = (p.p86 * assign1730_e1876);
        locals.var_taub_t = assign1730_e1877;
        locals.var_taub_t_dn3 = (p.p86 * (assign1730_e1876 * (locals.var_lntn_dn3 * assign1730_e1874)));
        locals.var_taub_t_rv = 0.0;

        let assign1740_e1882: f64 = (p.p98 - 1.0);
        let assign1740_e1883: f64 = (locals.var_lntn * assign1740_e1882);
        let assign1740_e1884: f64 = (assign1740_e1883).exp();
        let assign1740_e1885: f64 = (p.p87 * assign1740_e1884);
        locals.var_tepi_t = assign1740_e1885;
        locals.var_tepi_t_dn3 = (p.p87 * (assign1740_e1884 * (locals.var_lntn_dn3 * assign1740_e1882)));
        locals.var_tepi_t_rv = 0.0;

        let assign1750_e1889: f64 = (locals.var_taub_t + locals.var_tepi_t);
        let assign1750_e1890: f64 = (p.p88 * assign1750_e1889);
        let assign1750_e1893: f64 = (p.p86 + p.p87);
        let assign1750_e1894: f64 = (assign1750_e1890 / assign1750_e1893);
        locals.var_taur_t = assign1750_e1894;
        locals.var_taur_t_dn3 = ((p.p88 * (locals.var_taub_t_dn3 + locals.var_tepi_t_dn3)) / assign1750_e1893);
        locals.var_taur_t_rv = 0.0;

        let assign1760_e1899: f64 = (p.p99 - 1.0);
        let assign1760_e1900: f64 = (locals.var_lntn * assign1760_e1899);
        let assign1760_e1901: f64 = (assign1760_e1900).exp();
        let assign1760_e1902: f64 = (p.p89 * assign1760_e1901);
        locals.var_tauex_t = assign1760_e1902;
        locals.var_tauex_t_dn3 = (p.p89 * (assign1760_e1901 * (locals.var_lntn_dn3 * assign1760_e1899)));
        locals.var_tauex_t_rv = 0.0;

        let assign1770_e1905: f64 = (locals.var_tk - 300.0);
        locals.var_tk300 = assign1770_e1905;
        locals.var_tk300_dn3 = locals.var_tk_dn3;
        locals.var_tk300_rv = 0.0;

        let assign1780_e1908: f64 = if locals.var_tk < 525.0 { 1.0 } else { 0.0 };
        locals.var_guard23 = assign1780_e1908;
        locals.var_guard23_rv = 0.0;

        let (assign1790_e1924, assign1790_e1924_d_n3,) = {
    if (locals.var_guard23 != 0.0) {
        let assign1790_e1914: f64 = (0.00072 * locals.var_tk300);
        let assign1790_e1915: f64 = (1.0 + assign1790_e1914);
        let assign1790_e1918: f64 = (1.6e-6 * locals.var_tk300);
        let assign1790_e1920: f64 = (assign1790_e1918 * locals.var_tk300);
        let assign1790_e1921: f64 = (assign1790_e1915 - assign1790_e1920);
        let assign1790_e1922: f64 = (locals.var_bn * assign1790_e1921);
        (assign1790_e1922, (locals.var_bn * ((0.00072 * locals.var_tk300_dn3) - (((1.6e-6 * locals.var_tk300_dn3) * locals.var_tk300) + (assign1790_e1918 * locals.var_tk300_dn3)))),)
    } else {
        (locals.var_bnt, locals.var_bnt_dn3,)
    }
};
        locals.var_bnt = assign1790_e1924;
        locals.var_bnt_dn3 = assign1790_e1924_d_n3;
        locals.var_bnt_rv = 0.0;

        let (assign1800_e1931, assign1800_e1931_d_n3,) = {
    if (locals.var_guard23 == 0.0) {
        let assign1800_e1929: f64 = (locals.var_bn * 1.081);
        (assign1800_e1929, 0.0,)
    } else {
        (locals.var_bnt, locals.var_bnt_dn3,)
    }
};
        locals.var_bnt = assign1800_e1931;
        locals.var_bnt_dn3 = assign1800_e1931_d_n3;
        locals.var_bnt_rv = 0.0;

        let assign1810_e1935: f64 = (locals.var_lntn * p.p95);
        let assign1810_e1936: f64 = (assign1810_e1935).exp();
        let assign1810_e1937: f64 = (p.p91 * assign1810_e1936);
        locals.var_deg_t = assign1810_e1937;
        locals.var_deg_t_dn3 = (p.p91 * (assign1810_e1936 * (locals.var_lntn_dn3 * p.p95)));
        locals.var_deg_t_rv = 0.0;

        let assign1980_e2016: f64 = (p.p3 * (nv6 - nv7));
        locals.var_vb2c1 = assign1980_e2016;
        locals.var_vb2c1_dn6 = p.p3;
        locals.var_vb2c1_dn7 = (-p.p3);
        locals.var_vb2c1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let assign1990_e2019: f64 = (p.p3 * (nv6 - nv8));
        locals.var_vb2c2 = assign1990_e2019;
        locals.var_vb2c2_dn6 = p.p3;
        locals.var_vb2c2_dn8 = (-p.p3);
        locals.var_vb2c2_rv = 0.0;

        let assign2000_e2022: f64 = (p.p3 * (nv6 - nv4));
        locals.var_vb2e1 = assign2000_e2022;
        locals.var_vb2e1_dn4 = (-p.p3);
        locals.var_vb2e1_dn6 = p.p3;
        locals.var_vb2e1_rv = 0.0;

        let assign2010_e2025: f64 = (p.p3 * (nv5 - nv4));
        locals.var_vb1e1 = assign2010_e2025;
        locals.var_vb1e1_dn4 = (-p.p3);
        locals.var_vb1e1_dn5 = p.p3;
        locals.var_vb1e1_rv = 0.0;

        let assign2020_e2028: f64 = (p.p3 * (nv5 - nv6));
        locals.var_vb1b2 = assign2020_e2028;
        locals.var_vb1b2_dn5 = p.p3;
        locals.var_vb1b2_dn6 = (-p.p3);
        locals.var_vb1b2_rv = 0.0;

        let assign2030_e2031: f64 = (p.p3 * (nv7 - nv8));
        locals.var_vc1c2 = assign2030_e2031;
        locals.var_vc1c2_dn7 = p.p3;
        locals.var_vc1c2_dn8 = (-p.p3);
        locals.var_vc1c2_rv = 0.0;

        let assign2050_e2037: f64 = (p.p3 * (nv1 - nv5));
        locals.var_vbb1 = assign2050_e2037;
        locals.var_vbb1_dn1 = p.p3;
        locals.var_vbb1_dn5 = (-p.p3);
        locals.var_vbb1_rv = 0.0;

        let assign2060_e2040: f64 = (p.p3 * (nv1 - nv2));
        locals.var_vbe = assign2060_e2040;
        locals.var_vbe_dn1 = p.p3;
        locals.var_vbe_dn2 = (-p.p3);
        locals.var_vbe_rv = 0.0;

        let assign2070_e2043: f64 = (p.p3 * (nv1 - nv0));
        locals.var_vbc = assign2070_e2043;
        locals.var_vbc_dn0 = (-p.p3);
        locals.var_vbc_dn1 = p.p3;
        locals.var_vbc_rv = 0.0;

        let assign2080_e2046: f64 = (p.p3 * (nv10 - nv7));
        locals.var_vc4c1 = assign2080_e2046;
        locals.var_vc4c1_dn7 = (-p.p3);
        locals.var_vc4c1_dn10 = p.p3;
        locals.var_vc4c1_rv = 0.0;

        let assign2090_e2049: f64 = (p.p3 * (nv9 - nv10));
        locals.var_vc3c4 = assign2090_e2049;
        locals.var_vc3c4_dn9 = p.p3;
        locals.var_vc3c4_dn10 = (-p.p3);
        locals.var_vc3c4_rv = 0.0;

        let assign2100_e2052: f64 = (locals.var_vb1b2 + locals.var_vb2c2);
        let assign2100_e2054: f64 = (assign2100_e2052 - locals.var_vc1c2);
        let assign2100_e2056: f64 = (assign2100_e2054 - locals.var_vc4c1);
        locals.var_vb1c4 = assign2100_e2056;
        locals.var_vb1c4_dn5 = locals.var_vb1b2_dn5;
        locals.var_vb1c4_dn6 = (locals.var_vb1b2_dn6 + locals.var_vb2c2_dn6);
        locals.var_vb1c4_dn7 = ((-locals.var_vc1c2_dn7) - locals.var_vc4c1_dn7);
        locals.var_vb1c4_dn8 = (locals.var_vb2c2_dn8 - locals.var_vc1c2_dn8);
        locals.var_vb1c4_dn10 = (-locals.var_vc4c1_dn10);
        locals.var_vb1c4_rv = 0.0;

        let assign2110_e2058: f64 = (-locals.var_vbc);
        let assign2110_e2060: f64 = (assign2110_e2058 + locals.var_vbb1);
        let assign2110_e2062: f64 = (assign2110_e2060 + locals.var_vb1c4);
        let assign2110_e2064: f64 = (assign2110_e2062 - locals.var_vc3c4);
        locals.var_vcc3 = assign2110_e2064;
        locals.var_vcc3_dn0 = (-locals.var_vbc_dn0);
        locals.var_vcc3_dn1 = ((-locals.var_vbc_dn1) + locals.var_vbb1_dn1);
        locals.var_vcc3_dn5 = (locals.var_vbb1_dn5 + locals.var_vb1c4_dn5);
        locals.var_vcc3_dn6 = locals.var_vb1c4_dn6;
        locals.var_vcc3_dn7 = locals.var_vb1c4_dn7;
        locals.var_vcc3_dn8 = locals.var_vb1c4_dn8;
        locals.var_vcc3_dn9 = (-locals.var_vc3c4_dn9);
        locals.var_vcc3_dn10 = (locals.var_vb1c4_dn10 - locals.var_vc3c4_dn10);
        locals.var_vcc3_rv = 0.0;

        let assign2120_e2067: f64 = (locals.var_vbc + locals.var_vcc3);
        locals.var_vbc3 = assign2120_e2067;
        locals.var_vbc3_dn0 = (locals.var_vbc_dn0 + locals.var_vcc3_dn0);
        locals.var_vbc3_dn1 = (locals.var_vbc_dn1 + locals.var_vcc3_dn1);
        locals.var_vbc3_dn5 = locals.var_vcc3_dn5;
        locals.var_vbc3_dn6 = locals.var_vcc3_dn6;
        locals.var_vbc3_dn7 = locals.var_vcc3_dn7;
        locals.var_vbc3_dn8 = locals.var_vcc3_dn8;
        locals.var_vbc3_dn9 = locals.var_vcc3_dn9;
        locals.var_vbc3_dn10 = locals.var_vcc3_dn10;
        locals.var_vbc3_rv = 0.0;

        let assign2130_e2070: f64 = (locals.var_vb2c2 * locals.var_vtinv);
        let assign2130_e2072: f64 = if assign2130_e2070 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign2130_e2072;
        locals.var_guard30_rv = 0.0;

        let (assign2140_e2079, assign2140_e2079_d_n3, assign2140_e2079_d_n6, assign2140_e2079_d_n8,) = {
    if (locals.var_guard30 != 0.0) {
        let assign2140_e2076: f64 = (locals.var_vb2c2 * locals.var_vtinv);
        let assign2140_e2077: f64 = (assign2140_e2076).exp();
        (assign2140_e2077, (assign2140_e2077 * (locals.var_vb2c2 * locals.var_vtinv_dn3)), (assign2140_e2077 * (locals.var_vb2c2_dn6 * locals.var_vtinv)), (assign2140_e2077 * (locals.var_vb2c2_dn8 * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c2, locals.var_evb2c2_dn3, locals.var_evb2c2_dn6, locals.var_evb2c2_dn8,)
    }
};
        locals.var_evb2c2 = assign2140_e2079;
        locals.var_evb2c2_dn3 = assign2140_e2079_d_n3;
        locals.var_evb2c2_dn6 = assign2140_e2079_d_n6;
        locals.var_evb2c2_dn8 = assign2140_e2079_d_n8;
        locals.var_evb2c2_rv = 0.0;

        let (assign2150_e2085,) = {
    if (locals.var_guard30 == 0.0) {
        let assign2150_e2083: f64 = (p.p138).exp();
        (assign2150_e2083,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2150_e2085;
        locals.var_expl_rv = 0.0;

        let (assign2160_e2098, assign2160_e2098_d_n3, assign2160_e2098_d_n6, assign2160_e2098_d_n8,) = {
    if (locals.var_guard30 == 0.0) {
        let assign2160_e2092: f64 = (locals.var_vb2c2 * locals.var_vtinv);
        let assign2160_e2094: f64 = (assign2160_e2092 - p.p138);
        let assign2160_e2095: f64 = (1.0 + assign2160_e2094);
        let assign2160_e2096: f64 = (locals.var_expl * assign2160_e2095);
        (assign2160_e2096, (locals.var_expl * (locals.var_vb2c2 * locals.var_vtinv_dn3)), (locals.var_expl * (locals.var_vb2c2_dn6 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb2c2_dn8 * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c2, locals.var_evb2c2_dn3, locals.var_evb2c2_dn6, locals.var_evb2c2_dn8,)
    }
};
        locals.var_evb2c2 = assign2160_e2098;
        locals.var_evb2c2_dn3 = assign2160_e2098_d_n3;
        locals.var_evb2c2_dn6 = assign2160_e2098_d_n6;
        locals.var_evb2c2_dn8 = assign2160_e2098_d_n8;
        locals.var_evb2c2_rv = 0.0;

        let assign2170_e2101: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign2170_e2103: f64 = (assign2170_e2101 / locals.var_nff_t);
        let assign2170_e2105: f64 = if assign2170_e2103 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign2170_e2105;
        locals.var_guard31_rv = 0.0;

        let (assign2180_e2114, assign2180_e2114_d_n0, assign2180_e2114_d_n1, assign2180_e2114_d_n3, assign2180_e2114_d_n4, assign2180_e2114_d_n5, assign2180_e2114_d_n6, assign2180_e2114_d_n7, assign2180_e2114_d_n8, assign2180_e2114_d_n9, assign2180_e2114_d_n10,) = {
    if (locals.var_guard31 != 0.0) {
        let assign2180_e2109: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign2180_e2111: f64 = (assign2180_e2109 / locals.var_nff_t);
        let assign2180_e2112: f64 = (assign2180_e2111).exp();
        (assign2180_e2112, (assign2180_e2112 * (-((assign2180_e2109 * locals.var_nff_t_dn0) / (locals.var_nff_t * locals.var_nff_t)))), (assign2180_e2112 * (-((assign2180_e2109 * locals.var_nff_t_dn1) / (locals.var_nff_t * locals.var_nff_t)))), (assign2180_e2112 * ((((locals.var_vb2e1 * locals.var_vtinv_dn3) * locals.var_nff_t) - (assign2180_e2109 * locals.var_nff_t_dn3)) / (locals.var_nff_t * locals.var_nff_t))), (assign2180_e2112 * ((((locals.var_vb2e1_dn4 * locals.var_vtinv) * locals.var_nff_t) - (assign2180_e2109 * locals.var_nff_t_dn4)) / (locals.var_nff_t * locals.var_nff_t))), (assign2180_e2112 * (-((assign2180_e2109 * locals.var_nff_t_dn5) / (locals.var_nff_t * locals.var_nff_t)))), (assign2180_e2112 * ((((locals.var_vb2e1_dn6 * locals.var_vtinv) * locals.var_nff_t) - (assign2180_e2109 * locals.var_nff_t_dn6)) / (locals.var_nff_t * locals.var_nff_t))), (assign2180_e2112 * (-((assign2180_e2109 * locals.var_nff_t_dn7) / (locals.var_nff_t * locals.var_nff_t)))), (assign2180_e2112 * (-((assign2180_e2109 * locals.var_nff_t_dn8) / (locals.var_nff_t * locals.var_nff_t)))), (assign2180_e2112 * (-((assign2180_e2109 * locals.var_nff_t_dn9) / (locals.var_nff_t * locals.var_nff_t)))), (assign2180_e2112 * (-((assign2180_e2109 * locals.var_nff_t_dn10) / (locals.var_nff_t * locals.var_nff_t)))),)
    } else {
        (locals.var_evb2e1, locals.var_evb2e1_dn0, locals.var_evb2e1_dn1, locals.var_evb2e1_dn3, locals.var_evb2e1_dn4, locals.var_evb2e1_dn5, locals.var_evb2e1_dn6, locals.var_evb2e1_dn7, locals.var_evb2e1_dn8, locals.var_evb2e1_dn9, locals.var_evb2e1_dn10,)
    }
};
        locals.var_evb2e1 = assign2180_e2114;
        locals.var_evb2e1_dn0 = assign2180_e2114_d_n0;
        locals.var_evb2e1_dn1 = assign2180_e2114_d_n1;
        locals.var_evb2e1_dn3 = assign2180_e2114_d_n3;
        locals.var_evb2e1_dn4 = assign2180_e2114_d_n4;
        locals.var_evb2e1_dn5 = assign2180_e2114_d_n5;
        locals.var_evb2e1_dn6 = assign2180_e2114_d_n6;
        locals.var_evb2e1_dn7 = assign2180_e2114_d_n7;
        locals.var_evb2e1_dn8 = assign2180_e2114_d_n8;
        locals.var_evb2e1_dn9 = assign2180_e2114_d_n9;
        locals.var_evb2e1_dn10 = assign2180_e2114_d_n10;
        locals.var_evb2e1_rv = 0.0;

        let (assign2190_e2120,) = {
    if (locals.var_guard31 == 0.0) {
        let assign2190_e2118: f64 = (p.p138).exp();
        (assign2190_e2118,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2190_e2120;
        locals.var_expl_rv = 0.0;

        let (assign2200_e2135, assign2200_e2135_d_n0, assign2200_e2135_d_n1, assign2200_e2135_d_n3, assign2200_e2135_d_n4, assign2200_e2135_d_n5, assign2200_e2135_d_n6, assign2200_e2135_d_n7, assign2200_e2135_d_n8, assign2200_e2135_d_n9, assign2200_e2135_d_n10,) = {
    if (locals.var_guard31 == 0.0) {
        let assign2200_e2127: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign2200_e2129: f64 = (assign2200_e2127 / locals.var_nff_t);
        let assign2200_e2131: f64 = (assign2200_e2129 - p.p138);
        let assign2200_e2132: f64 = (1.0 + assign2200_e2131);
        let assign2200_e2133: f64 = (locals.var_expl * assign2200_e2132);
        (assign2200_e2133, (locals.var_expl * (-((assign2200_e2127 * locals.var_nff_t_dn0) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * (-((assign2200_e2127 * locals.var_nff_t_dn1) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * ((((locals.var_vb2e1 * locals.var_vtinv_dn3) * locals.var_nff_t) - (assign2200_e2127 * locals.var_nff_t_dn3)) / (locals.var_nff_t * locals.var_nff_t))), (locals.var_expl * ((((locals.var_vb2e1_dn4 * locals.var_vtinv) * locals.var_nff_t) - (assign2200_e2127 * locals.var_nff_t_dn4)) / (locals.var_nff_t * locals.var_nff_t))), (locals.var_expl * (-((assign2200_e2127 * locals.var_nff_t_dn5) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * ((((locals.var_vb2e1_dn6 * locals.var_vtinv) * locals.var_nff_t) - (assign2200_e2127 * locals.var_nff_t_dn6)) / (locals.var_nff_t * locals.var_nff_t))), (locals.var_expl * (-((assign2200_e2127 * locals.var_nff_t_dn7) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * (-((assign2200_e2127 * locals.var_nff_t_dn8) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * (-((assign2200_e2127 * locals.var_nff_t_dn9) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * (-((assign2200_e2127 * locals.var_nff_t_dn10) / (locals.var_nff_t * locals.var_nff_t)))),)
    } else {
        (locals.var_evb2e1, locals.var_evb2e1_dn0, locals.var_evb2e1_dn1, locals.var_evb2e1_dn3, locals.var_evb2e1_dn4, locals.var_evb2e1_dn5, locals.var_evb2e1_dn6, locals.var_evb2e1_dn7, locals.var_evb2e1_dn8, locals.var_evb2e1_dn9, locals.var_evb2e1_dn10,)
    }
};
        locals.var_evb2e1 = assign2200_e2135;
        locals.var_evb2e1_dn0 = assign2200_e2135_d_n0;
        locals.var_evb2e1_dn1 = assign2200_e2135_d_n1;
        locals.var_evb2e1_dn3 = assign2200_e2135_d_n3;
        locals.var_evb2e1_dn4 = assign2200_e2135_d_n4;
        locals.var_evb2e1_dn5 = assign2200_e2135_d_n5;
        locals.var_evb2e1_dn6 = assign2200_e2135_d_n6;
        locals.var_evb2e1_dn7 = assign2200_e2135_d_n7;
        locals.var_evb2e1_dn8 = assign2200_e2135_d_n8;
        locals.var_evb2e1_dn9 = assign2200_e2135_d_n9;
        locals.var_evb2e1_dn10 = assign2200_e2135_d_n10;
        locals.var_evb2e1_rv = 0.0;

        let assign2210_e2138: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign2210_e2140: f64 = if assign2210_e2138 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign2210_e2140;
        locals.var_guard32_rv = 0.0;

        let (assign2220_e2147, assign2220_e2147_d_n3, assign2220_e2147_d_n5, assign2220_e2147_d_n6, assign2220_e2147_d_n7, assign2220_e2147_d_n8, assign2220_e2147_d_n10,) = {
    if (locals.var_guard32 != 0.0) {
        let assign2220_e2144: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign2220_e2145: f64 = (assign2220_e2144).exp();
        (assign2220_e2145, (assign2220_e2145 * (locals.var_vb1c4 * locals.var_vtinv_dn3)), (assign2220_e2145 * (locals.var_vb1c4_dn5 * locals.var_vtinv)), (assign2220_e2145 * (locals.var_vb1c4_dn6 * locals.var_vtinv)), (assign2220_e2145 * (locals.var_vb1c4_dn7 * locals.var_vtinv)), (assign2220_e2145 * (locals.var_vb1c4_dn8 * locals.var_vtinv)), (assign2220_e2145 * (locals.var_vb1c4_dn10 * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4, locals.var_evb1c4_dn3, locals.var_evb1c4_dn5, locals.var_evb1c4_dn6, locals.var_evb1c4_dn7, locals.var_evb1c4_dn8, locals.var_evb1c4_dn10,)
    }
};
        locals.var_evb1c4 = assign2220_e2147;
        locals.var_evb1c4_dn3 = assign2220_e2147_d_n3;
        locals.var_evb1c4_dn5 = assign2220_e2147_d_n5;
        locals.var_evb1c4_dn6 = assign2220_e2147_d_n6;
        locals.var_evb1c4_dn7 = assign2220_e2147_d_n7;
        locals.var_evb1c4_dn8 = assign2220_e2147_d_n8;
        locals.var_evb1c4_dn10 = assign2220_e2147_d_n10;
        locals.var_evb1c4_rv = 0.0;

        let (assign2230_e2153,) = {
    if (locals.var_guard32 == 0.0) {
        let assign2230_e2151: f64 = (p.p138).exp();
        (assign2230_e2151,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2230_e2153;
        locals.var_expl_rv = 0.0;

        let (assign2240_e2166, assign2240_e2166_d_n3, assign2240_e2166_d_n5, assign2240_e2166_d_n6, assign2240_e2166_d_n7, assign2240_e2166_d_n8, assign2240_e2166_d_n10,) = {
    if (locals.var_guard32 == 0.0) {
        let assign2240_e2160: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign2240_e2162: f64 = (assign2240_e2160 - p.p138);
        let assign2240_e2163: f64 = (1.0 + assign2240_e2162);
        let assign2240_e2164: f64 = (locals.var_expl * assign2240_e2163);
        (assign2240_e2164, (locals.var_expl * (locals.var_vb1c4 * locals.var_vtinv_dn3)), (locals.var_expl * (locals.var_vb1c4_dn5 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb1c4_dn6 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb1c4_dn7 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb1c4_dn8 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb1c4_dn10 * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4, locals.var_evb1c4_dn3, locals.var_evb1c4_dn5, locals.var_evb1c4_dn6, locals.var_evb1c4_dn7, locals.var_evb1c4_dn8, locals.var_evb1c4_dn10,)
    }
};
        locals.var_evb1c4 = assign2240_e2166;
        locals.var_evb1c4_dn3 = assign2240_e2166_d_n3;
        locals.var_evb1c4_dn5 = assign2240_e2166_d_n5;
        locals.var_evb1c4_dn6 = assign2240_e2166_d_n6;
        locals.var_evb1c4_dn7 = assign2240_e2166_d_n7;
        locals.var_evb1c4_dn8 = assign2240_e2166_d_n8;
        locals.var_evb1c4_dn10 = assign2240_e2166_d_n10;
        locals.var_evb1c4_rv = 0.0;

        let assign2250_e2169: f64 = (locals.var_vb1b2 * locals.var_vtinv);
        let assign2250_e2171: f64 = if assign2250_e2169 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign2250_e2171;
        locals.var_guard33_rv = 0.0;

        let (assign2270_e2184,) = {
    if (locals.var_guard33 == 0.0) {
        let assign2270_e2182: f64 = (p.p138).exp();
        (assign2270_e2182,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2270_e2184;
        locals.var_expl_rv = 0.0;

        let assign2290_e2200: f64 = (locals.var_vbc3 * locals.var_vtinv);
        let assign2290_e2202: f64 = if assign2290_e2200 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign2290_e2202;
        locals.var_guard34_rv = 0.0;

        let (assign2300_e2209, assign2300_e2209_d_n0, assign2300_e2209_d_n1, assign2300_e2209_d_n3, assign2300_e2209_d_n5, assign2300_e2209_d_n6, assign2300_e2209_d_n7, assign2300_e2209_d_n8, assign2300_e2209_d_n9, assign2300_e2209_d_n10,) = {
    if (locals.var_guard34 != 0.0) {
        let assign2300_e2206: f64 = (locals.var_vbc3 * locals.var_vtinv);
        let assign2300_e2207: f64 = (assign2300_e2206).exp();
        (assign2300_e2207, (assign2300_e2207 * (locals.var_vbc3_dn0 * locals.var_vtinv)), (assign2300_e2207 * (locals.var_vbc3_dn1 * locals.var_vtinv)), (assign2300_e2207 * (locals.var_vbc3 * locals.var_vtinv_dn3)), (assign2300_e2207 * (locals.var_vbc3_dn5 * locals.var_vtinv)), (assign2300_e2207 * (locals.var_vbc3_dn6 * locals.var_vtinv)), (assign2300_e2207 * (locals.var_vbc3_dn7 * locals.var_vtinv)), (assign2300_e2207 * (locals.var_vbc3_dn8 * locals.var_vtinv)), (assign2300_e2207 * (locals.var_vbc3_dn9 * locals.var_vtinv)), (assign2300_e2207 * (locals.var_vbc3_dn10 * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3, locals.var_evbc3_dn0, locals.var_evbc3_dn1, locals.var_evbc3_dn3, locals.var_evbc3_dn5, locals.var_evbc3_dn6, locals.var_evbc3_dn7, locals.var_evbc3_dn8, locals.var_evbc3_dn9, locals.var_evbc3_dn10,)
    }
};
        locals.var_evbc3 = assign2300_e2209;
        locals.var_evbc3_dn0 = assign2300_e2209_d_n0;
        locals.var_evbc3_dn1 = assign2300_e2209_d_n1;
        locals.var_evbc3_dn3 = assign2300_e2209_d_n3;
        locals.var_evbc3_dn5 = assign2300_e2209_d_n5;
        locals.var_evbc3_dn6 = assign2300_e2209_d_n6;
        locals.var_evbc3_dn7 = assign2300_e2209_d_n7;
        locals.var_evbc3_dn8 = assign2300_e2209_d_n8;
        locals.var_evbc3_dn9 = assign2300_e2209_d_n9;
        locals.var_evbc3_dn10 = assign2300_e2209_d_n10;
        locals.var_evbc3_rv = 0.0;

        let (assign2310_e2215,) = {
    if (locals.var_guard34 == 0.0) {
        let assign2310_e2213: f64 = (p.p138).exp();
        (assign2310_e2213,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2310_e2215;
        locals.var_expl_rv = 0.0;

        let (assign2320_e2228, assign2320_e2228_d_n0, assign2320_e2228_d_n1, assign2320_e2228_d_n3, assign2320_e2228_d_n5, assign2320_e2228_d_n6, assign2320_e2228_d_n7, assign2320_e2228_d_n8, assign2320_e2228_d_n9, assign2320_e2228_d_n10,) = {
    if (locals.var_guard34 == 0.0) {
        let assign2320_e2222: f64 = (locals.var_vbc3 * locals.var_vtinv);
        let assign2320_e2224: f64 = (assign2320_e2222 - p.p138);
        let assign2320_e2225: f64 = (1.0 + assign2320_e2224);
        let assign2320_e2226: f64 = (locals.var_expl * assign2320_e2225);
        (assign2320_e2226, (locals.var_expl * (locals.var_vbc3_dn0 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn1 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3 * locals.var_vtinv_dn3)), (locals.var_expl * (locals.var_vbc3_dn5 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn6 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn7 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn8 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn9 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn10 * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3, locals.var_evbc3_dn0, locals.var_evbc3_dn1, locals.var_evbc3_dn3, locals.var_evbc3_dn5, locals.var_evbc3_dn6, locals.var_evbc3_dn7, locals.var_evbc3_dn8, locals.var_evbc3_dn9, locals.var_evbc3_dn10,)
    }
};
        locals.var_evbc3 = assign2320_e2228;
        locals.var_evbc3_dn0 = assign2320_e2228_d_n0;
        locals.var_evbc3_dn1 = assign2320_e2228_d_n1;
        locals.var_evbc3_dn3 = assign2320_e2228_d_n3;
        locals.var_evbc3_dn5 = assign2320_e2228_d_n5;
        locals.var_evbc3_dn6 = assign2320_e2228_d_n6;
        locals.var_evbc3_dn7 = assign2320_e2228_d_n7;
        locals.var_evbc3_dn8 = assign2320_e2228_d_n8;
        locals.var_evbc3_dn9 = assign2320_e2228_d_n9;
        locals.var_evbc3_dn10 = assign2320_e2228_d_n10;
        locals.var_evbc3_rv = 0.0;

        let assign2330_e2231: f64 = (locals.var_vbc3 - locals.var_vdc_t);
        let assign2330_e2233: f64 = (assign2330_e2231 * locals.var_vtinv);
        let assign2330_e2235: f64 = if assign2330_e2233 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign2330_e2235;
        locals.var_guard35_rv = 0.0;

        let (assign2340_e2244, assign2340_e2244_d_n0, assign2340_e2244_d_n1, assign2340_e2244_d_n3, assign2340_e2244_d_n4, assign2340_e2244_d_n5, assign2340_e2244_d_n6, assign2340_e2244_d_n7, assign2340_e2244_d_n8, assign2340_e2244_d_n9, assign2340_e2244_d_n10,) = {
    if (locals.var_guard35 != 0.0) {
        let assign2340_e2239: f64 = (locals.var_vbc3 - locals.var_vdc_t);
        let assign2340_e2241: f64 = (assign2340_e2239 * locals.var_vtinv);
        let assign2340_e2242: f64 = (assign2340_e2241).exp();
        (assign2340_e2242, (assign2340_e2242 * ((locals.var_vbc3_dn0 - locals.var_vdc_t_dn0) * locals.var_vtinv)), (assign2340_e2242 * ((locals.var_vbc3_dn1 - locals.var_vdc_t_dn1) * locals.var_vtinv)), (assign2340_e2242 * (((-locals.var_vdc_t_dn3) * locals.var_vtinv) + (assign2340_e2239 * locals.var_vtinv_dn3))), (assign2340_e2242 * ((-locals.var_vdc_t_dn4) * locals.var_vtinv)), (assign2340_e2242 * ((locals.var_vbc3_dn5 - locals.var_vdc_t_dn5) * locals.var_vtinv)), (assign2340_e2242 * ((locals.var_vbc3_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (assign2340_e2242 * ((locals.var_vbc3_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (assign2340_e2242 * ((locals.var_vbc3_dn8 - locals.var_vdc_t_dn8) * locals.var_vtinv)), (assign2340_e2242 * ((locals.var_vbc3_dn9 - locals.var_vdc_t_dn9) * locals.var_vtinv)), (assign2340_e2242 * ((locals.var_vbc3_dn10 - locals.var_vdc_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3vdc, locals.var_evbc3vdc_dn0, locals.var_evbc3vdc_dn1, locals.var_evbc3vdc_dn3, locals.var_evbc3vdc_dn4, locals.var_evbc3vdc_dn5, locals.var_evbc3vdc_dn6, locals.var_evbc3vdc_dn7, locals.var_evbc3vdc_dn8, locals.var_evbc3vdc_dn9, locals.var_evbc3vdc_dn10,)
    }
};
        locals.var_evbc3vdc = assign2340_e2244;
        locals.var_evbc3vdc_dn0 = assign2340_e2244_d_n0;
        locals.var_evbc3vdc_dn1 = assign2340_e2244_d_n1;
        locals.var_evbc3vdc_dn3 = assign2340_e2244_d_n3;
        locals.var_evbc3vdc_dn4 = assign2340_e2244_d_n4;
        locals.var_evbc3vdc_dn5 = assign2340_e2244_d_n5;
        locals.var_evbc3vdc_dn6 = assign2340_e2244_d_n6;
        locals.var_evbc3vdc_dn7 = assign2340_e2244_d_n7;
        locals.var_evbc3vdc_dn8 = assign2340_e2244_d_n8;
        locals.var_evbc3vdc_dn9 = assign2340_e2244_d_n9;
        locals.var_evbc3vdc_dn10 = assign2340_e2244_d_n10;
        locals.var_evbc3vdc_rv = 0.0;

        let (assign2350_e2250,) = {
    if (locals.var_guard35 == 0.0) {
        let assign2350_e2248: f64 = (p.p138).exp();
        (assign2350_e2248,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2350_e2250;
        locals.var_expl_rv = 0.0;

        let (assign2360_e2265, assign2360_e2265_d_n0, assign2360_e2265_d_n1, assign2360_e2265_d_n3, assign2360_e2265_d_n4, assign2360_e2265_d_n5, assign2360_e2265_d_n6, assign2360_e2265_d_n7, assign2360_e2265_d_n8, assign2360_e2265_d_n9, assign2360_e2265_d_n10,) = {
    if (locals.var_guard35 == 0.0) {
        let assign2360_e2257: f64 = (locals.var_vbc3 - locals.var_vdc_t);
        let assign2360_e2259: f64 = (assign2360_e2257 * locals.var_vtinv);
        let assign2360_e2261: f64 = (assign2360_e2259 - p.p138);
        let assign2360_e2262: f64 = (1.0 + assign2360_e2261);
        let assign2360_e2263: f64 = (locals.var_expl * assign2360_e2262);
        (assign2360_e2263, (locals.var_expl * ((locals.var_vbc3_dn0 - locals.var_vdc_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn1 - locals.var_vdc_t_dn1) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdc_t_dn3) * locals.var_vtinv) + (assign2360_e2257 * locals.var_vtinv_dn3))), (locals.var_expl * ((-locals.var_vdc_t_dn4) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn5 - locals.var_vdc_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn8 - locals.var_vdc_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn9 - locals.var_vdc_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn10 - locals.var_vdc_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3vdc, locals.var_evbc3vdc_dn0, locals.var_evbc3vdc_dn1, locals.var_evbc3vdc_dn3, locals.var_evbc3vdc_dn4, locals.var_evbc3vdc_dn5, locals.var_evbc3vdc_dn6, locals.var_evbc3vdc_dn7, locals.var_evbc3vdc_dn8, locals.var_evbc3vdc_dn9, locals.var_evbc3vdc_dn10,)
    }
};
        locals.var_evbc3vdc = assign2360_e2265;
        locals.var_evbc3vdc_dn0 = assign2360_e2265_d_n0;
        locals.var_evbc3vdc_dn1 = assign2360_e2265_d_n1;
        locals.var_evbc3vdc_dn3 = assign2360_e2265_d_n3;
        locals.var_evbc3vdc_dn4 = assign2360_e2265_d_n4;
        locals.var_evbc3vdc_dn5 = assign2360_e2265_d_n5;
        locals.var_evbc3vdc_dn6 = assign2360_e2265_d_n6;
        locals.var_evbc3vdc_dn7 = assign2360_e2265_d_n7;
        locals.var_evbc3vdc_dn8 = assign2360_e2265_d_n8;
        locals.var_evbc3vdc_dn9 = assign2360_e2265_d_n9;
        locals.var_evbc3vdc_dn10 = assign2360_e2265_d_n10;
        locals.var_evbc3vdc_rv = 0.0;

        let assign2370_e2268: f64 = (locals.var_vb1c4 - locals.var_vdc_t);
        let assign2370_e2270: f64 = (assign2370_e2268 * locals.var_vtinv);
        let assign2370_e2272: f64 = if assign2370_e2270 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign2370_e2272;
        locals.var_guard36_rv = 0.0;

        let (assign2380_e2281, assign2380_e2281_d_n0, assign2380_e2281_d_n1, assign2380_e2281_d_n3, assign2380_e2281_d_n4, assign2380_e2281_d_n5, assign2380_e2281_d_n6, assign2380_e2281_d_n7, assign2380_e2281_d_n8, assign2380_e2281_d_n9, assign2380_e2281_d_n10,) = {
    if (locals.var_guard36 != 0.0) {
        let assign2380_e2276: f64 = (locals.var_vb1c4 - locals.var_vdc_t);
        let assign2380_e2278: f64 = (assign2380_e2276 * locals.var_vtinv);
        let assign2380_e2279: f64 = (assign2380_e2278).exp();
        (assign2380_e2279, (assign2380_e2279 * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (assign2380_e2279 * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (assign2380_e2279 * (((-locals.var_vdc_t_dn3) * locals.var_vtinv) + (assign2380_e2276 * locals.var_vtinv_dn3))), (assign2380_e2279 * ((-locals.var_vdc_t_dn4) * locals.var_vtinv)), (assign2380_e2279 * ((locals.var_vb1c4_dn5 - locals.var_vdc_t_dn5) * locals.var_vtinv)), (assign2380_e2279 * ((locals.var_vb1c4_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (assign2380_e2279 * ((locals.var_vb1c4_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (assign2380_e2279 * ((locals.var_vb1c4_dn8 - locals.var_vdc_t_dn8) * locals.var_vtinv)), (assign2380_e2279 * ((-locals.var_vdc_t_dn9) * locals.var_vtinv)), (assign2380_e2279 * ((locals.var_vb1c4_dn10 - locals.var_vdc_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4vdc, locals.var_evb1c4vdc_dn0, locals.var_evb1c4vdc_dn1, locals.var_evb1c4vdc_dn3, locals.var_evb1c4vdc_dn4, locals.var_evb1c4vdc_dn5, locals.var_evb1c4vdc_dn6, locals.var_evb1c4vdc_dn7, locals.var_evb1c4vdc_dn8, locals.var_evb1c4vdc_dn9, locals.var_evb1c4vdc_dn10,)
    }
};
        locals.var_evb1c4vdc = assign2380_e2281;
        locals.var_evb1c4vdc_dn0 = assign2380_e2281_d_n0;
        locals.var_evb1c4vdc_dn1 = assign2380_e2281_d_n1;
        locals.var_evb1c4vdc_dn3 = assign2380_e2281_d_n3;
        locals.var_evb1c4vdc_dn4 = assign2380_e2281_d_n4;
        locals.var_evb1c4vdc_dn5 = assign2380_e2281_d_n5;
        locals.var_evb1c4vdc_dn6 = assign2380_e2281_d_n6;
        locals.var_evb1c4vdc_dn7 = assign2380_e2281_d_n7;
        locals.var_evb1c4vdc_dn8 = assign2380_e2281_d_n8;
        locals.var_evb1c4vdc_dn9 = assign2380_e2281_d_n9;
        locals.var_evb1c4vdc_dn10 = assign2380_e2281_d_n10;
        locals.var_evb1c4vdc_rv = 0.0;

        let (assign2390_e2287,) = {
    if (locals.var_guard36 == 0.0) {
        let assign2390_e2285: f64 = (p.p138).exp();
        (assign2390_e2285,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2390_e2287;
        locals.var_expl_rv = 0.0;

        let (assign2400_e2302, assign2400_e2302_d_n0, assign2400_e2302_d_n1, assign2400_e2302_d_n3, assign2400_e2302_d_n4, assign2400_e2302_d_n5, assign2400_e2302_d_n6, assign2400_e2302_d_n7, assign2400_e2302_d_n8, assign2400_e2302_d_n9, assign2400_e2302_d_n10,) = {
    if (locals.var_guard36 == 0.0) {
        let assign2400_e2294: f64 = (locals.var_vb1c4 - locals.var_vdc_t);
        let assign2400_e2296: f64 = (assign2400_e2294 * locals.var_vtinv);
        let assign2400_e2298: f64 = (assign2400_e2296 - p.p138);
        let assign2400_e2299: f64 = (1.0 + assign2400_e2298);
        let assign2400_e2300: f64 = (locals.var_expl * assign2400_e2299);
        (assign2400_e2300, (locals.var_expl * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdc_t_dn3) * locals.var_vtinv) + (assign2400_e2294 * locals.var_vtinv_dn3))), (locals.var_expl * ((-locals.var_vdc_t_dn4) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn5 - locals.var_vdc_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn8 - locals.var_vdc_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn10 - locals.var_vdc_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4vdc, locals.var_evb1c4vdc_dn0, locals.var_evb1c4vdc_dn1, locals.var_evb1c4vdc_dn3, locals.var_evb1c4vdc_dn4, locals.var_evb1c4vdc_dn5, locals.var_evb1c4vdc_dn6, locals.var_evb1c4vdc_dn7, locals.var_evb1c4vdc_dn8, locals.var_evb1c4vdc_dn9, locals.var_evb1c4vdc_dn10,)
    }
};
        locals.var_evb1c4vdc = assign2400_e2302;
        locals.var_evb1c4vdc_dn0 = assign2400_e2302_d_n0;
        locals.var_evb1c4vdc_dn1 = assign2400_e2302_d_n1;
        locals.var_evb1c4vdc_dn3 = assign2400_e2302_d_n3;
        locals.var_evb1c4vdc_dn4 = assign2400_e2302_d_n4;
        locals.var_evb1c4vdc_dn5 = assign2400_e2302_d_n5;
        locals.var_evb1c4vdc_dn6 = assign2400_e2302_d_n6;
        locals.var_evb1c4vdc_dn7 = assign2400_e2302_d_n7;
        locals.var_evb1c4vdc_dn8 = assign2400_e2302_d_n8;
        locals.var_evb1c4vdc_dn9 = assign2400_e2302_d_n9;
        locals.var_evb1c4vdc_dn10 = assign2400_e2302_d_n10;
        locals.var_evb1c4vdc_rv = 0.0;

        let assign2410_e2305: f64 = (locals.var_vb2c2 - locals.var_vdc_t);
        let assign2410_e2307: f64 = (assign2410_e2305 * locals.var_vtinv);
        let assign2410_e2309: f64 = if assign2410_e2307 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign2410_e2309;
        locals.var_guard37_rv = 0.0;

        let (assign2420_e2318, assign2420_e2318_d_n0, assign2420_e2318_d_n1, assign2420_e2318_d_n3, assign2420_e2318_d_n4, assign2420_e2318_d_n5, assign2420_e2318_d_n6, assign2420_e2318_d_n7, assign2420_e2318_d_n8, assign2420_e2318_d_n9, assign2420_e2318_d_n10,) = {
    if (locals.var_guard37 != 0.0) {
        let assign2420_e2313: f64 = (locals.var_vb2c2 - locals.var_vdc_t);
        let assign2420_e2315: f64 = (assign2420_e2313 * locals.var_vtinv);
        let assign2420_e2316: f64 = (assign2420_e2315).exp();
        (assign2420_e2316, (assign2420_e2316 * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (assign2420_e2316 * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (assign2420_e2316 * (((-locals.var_vdc_t_dn3) * locals.var_vtinv) + (assign2420_e2313 * locals.var_vtinv_dn3))), (assign2420_e2316 * ((-locals.var_vdc_t_dn4) * locals.var_vtinv)), (assign2420_e2316 * ((-locals.var_vdc_t_dn5) * locals.var_vtinv)), (assign2420_e2316 * ((locals.var_vb2c2_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (assign2420_e2316 * ((-locals.var_vdc_t_dn7) * locals.var_vtinv)), (assign2420_e2316 * ((locals.var_vb2c2_dn8 - locals.var_vdc_t_dn8) * locals.var_vtinv)), (assign2420_e2316 * ((-locals.var_vdc_t_dn9) * locals.var_vtinv)), (assign2420_e2316 * ((-locals.var_vdc_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c2vdc, locals.var_evb2c2vdc_dn0, locals.var_evb2c2vdc_dn1, locals.var_evb2c2vdc_dn3, locals.var_evb2c2vdc_dn4, locals.var_evb2c2vdc_dn5, locals.var_evb2c2vdc_dn6, locals.var_evb2c2vdc_dn7, locals.var_evb2c2vdc_dn8, locals.var_evb2c2vdc_dn9, locals.var_evb2c2vdc_dn10,)
    }
};
        locals.var_evb2c2vdc = assign2420_e2318;
        locals.var_evb2c2vdc_dn0 = assign2420_e2318_d_n0;
        locals.var_evb2c2vdc_dn1 = assign2420_e2318_d_n1;
        locals.var_evb2c2vdc_dn3 = assign2420_e2318_d_n3;
        locals.var_evb2c2vdc_dn4 = assign2420_e2318_d_n4;
        locals.var_evb2c2vdc_dn5 = assign2420_e2318_d_n5;
        locals.var_evb2c2vdc_dn6 = assign2420_e2318_d_n6;
        locals.var_evb2c2vdc_dn7 = assign2420_e2318_d_n7;
        locals.var_evb2c2vdc_dn8 = assign2420_e2318_d_n8;
        locals.var_evb2c2vdc_dn9 = assign2420_e2318_d_n9;
        locals.var_evb2c2vdc_dn10 = assign2420_e2318_d_n10;
        locals.var_evb2c2vdc_rv = 0.0;

        let (assign2430_e2324,) = {
    if (locals.var_guard37 == 0.0) {
        let assign2430_e2322: f64 = (p.p138).exp();
        (assign2430_e2322,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2430_e2324;
        locals.var_expl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2440_e2339, assign2440_e2339_d_n0, assign2440_e2339_d_n1, assign2440_e2339_d_n3, assign2440_e2339_d_n4, assign2440_e2339_d_n5, assign2440_e2339_d_n6, assign2440_e2339_d_n7, assign2440_e2339_d_n8, assign2440_e2339_d_n9, assign2440_e2339_d_n10,) = {
    if (locals.var_guard37 == 0.0) {
        let assign2440_e2331: f64 = (locals.var_vb2c2 - locals.var_vdc_t);
        let assign2440_e2333: f64 = (assign2440_e2331 * locals.var_vtinv);
        let assign2440_e2335: f64 = (assign2440_e2333 - p.p138);
        let assign2440_e2336: f64 = (1.0 + assign2440_e2335);
        let assign2440_e2337: f64 = (locals.var_expl * assign2440_e2336);
        (assign2440_e2337, (locals.var_expl * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdc_t_dn3) * locals.var_vtinv) + (assign2440_e2331 * locals.var_vtinv_dn3))), (locals.var_expl * ((-locals.var_vdc_t_dn4) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2c2_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2c2_dn8 - locals.var_vdc_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c2vdc, locals.var_evb2c2vdc_dn0, locals.var_evb2c2vdc_dn1, locals.var_evb2c2vdc_dn3, locals.var_evb2c2vdc_dn4, locals.var_evb2c2vdc_dn5, locals.var_evb2c2vdc_dn6, locals.var_evb2c2vdc_dn7, locals.var_evb2c2vdc_dn8, locals.var_evb2c2vdc_dn9, locals.var_evb2c2vdc_dn10,)
    }
};
        locals.var_evb2c2vdc = assign2440_e2339;
        locals.var_evb2c2vdc_dn0 = assign2440_e2339_d_n0;
        locals.var_evb2c2vdc_dn1 = assign2440_e2339_d_n1;
        locals.var_evb2c2vdc_dn3 = assign2440_e2339_d_n3;
        locals.var_evb2c2vdc_dn4 = assign2440_e2339_d_n4;
        locals.var_evb2c2vdc_dn5 = assign2440_e2339_d_n5;
        locals.var_evb2c2vdc_dn6 = assign2440_e2339_d_n6;
        locals.var_evb2c2vdc_dn7 = assign2440_e2339_d_n7;
        locals.var_evb2c2vdc_dn8 = assign2440_e2339_d_n8;
        locals.var_evb2c2vdc_dn9 = assign2440_e2339_d_n9;
        locals.var_evb2c2vdc_dn10 = assign2440_e2339_d_n10;
        locals.var_evb2c2vdc_rv = 0.0;

        let assign2450_e2342: f64 = (locals.var_vb2c1 - locals.var_vdc_t);
        let assign2450_e2344: f64 = (assign2450_e2342 * locals.var_vtinv);
        let assign2450_e2346: f64 = if assign2450_e2344 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard38 = assign2450_e2346;
        locals.var_guard38_rv = 0.0;

        let (assign2460_e2355, assign2460_e2355_d_n0, assign2460_e2355_d_n1, assign2460_e2355_d_n3, assign2460_e2355_d_n4, assign2460_e2355_d_n5, assign2460_e2355_d_n6, assign2460_e2355_d_n7, assign2460_e2355_d_n8, assign2460_e2355_d_n9, assign2460_e2355_d_n10,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2460_e2350: f64 = (locals.var_vb2c1 - locals.var_vdc_t);
        let assign2460_e2352: f64 = (assign2460_e2350 * locals.var_vtinv);
        let assign2460_e2353: f64 = (assign2460_e2352).exp();
        (assign2460_e2353, (assign2460_e2353 * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (assign2460_e2353 * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (assign2460_e2353 * (((-locals.var_vdc_t_dn3) * locals.var_vtinv) + (assign2460_e2350 * locals.var_vtinv_dn3))), (assign2460_e2353 * ((-locals.var_vdc_t_dn4) * locals.var_vtinv)), (assign2460_e2353 * ((-locals.var_vdc_t_dn5) * locals.var_vtinv)), (assign2460_e2353 * ((locals.var_vb2c1_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (assign2460_e2353 * ((locals.var_vb2c1_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (assign2460_e2353 * ((-locals.var_vdc_t_dn8) * locals.var_vtinv)), (assign2460_e2353 * ((-locals.var_vdc_t_dn9) * locals.var_vtinv)), (assign2460_e2353 * ((-locals.var_vdc_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c1vdc, locals.var_evb2c1vdc_dn0, locals.var_evb2c1vdc_dn1, locals.var_evb2c1vdc_dn3, locals.var_evb2c1vdc_dn4, locals.var_evb2c1vdc_dn5, locals.var_evb2c1vdc_dn6, locals.var_evb2c1vdc_dn7, locals.var_evb2c1vdc_dn8, locals.var_evb2c1vdc_dn9, locals.var_evb2c1vdc_dn10,)
    }
};
        locals.var_evb2c1vdc = assign2460_e2355;
        locals.var_evb2c1vdc_dn0 = assign2460_e2355_d_n0;
        locals.var_evb2c1vdc_dn1 = assign2460_e2355_d_n1;
        locals.var_evb2c1vdc_dn3 = assign2460_e2355_d_n3;
        locals.var_evb2c1vdc_dn4 = assign2460_e2355_d_n4;
        locals.var_evb2c1vdc_dn5 = assign2460_e2355_d_n5;
        locals.var_evb2c1vdc_dn6 = assign2460_e2355_d_n6;
        locals.var_evb2c1vdc_dn7 = assign2460_e2355_d_n7;
        locals.var_evb2c1vdc_dn8 = assign2460_e2355_d_n8;
        locals.var_evb2c1vdc_dn9 = assign2460_e2355_d_n9;
        locals.var_evb2c1vdc_dn10 = assign2460_e2355_d_n10;
        locals.var_evb2c1vdc_rv = 0.0;

        let (assign2470_e2361,) = {
    if (locals.var_guard38 == 0.0) {
        let assign2470_e2359: f64 = (p.p138).exp();
        (assign2470_e2359,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2470_e2361;
        locals.var_expl_rv = 0.0;

        let (assign2480_e2376, assign2480_e2376_d_n0, assign2480_e2376_d_n1, assign2480_e2376_d_n3, assign2480_e2376_d_n4, assign2480_e2376_d_n5, assign2480_e2376_d_n6, assign2480_e2376_d_n7, assign2480_e2376_d_n8, assign2480_e2376_d_n9, assign2480_e2376_d_n10,) = {
    if (locals.var_guard38 == 0.0) {
        let assign2480_e2368: f64 = (locals.var_vb2c1 - locals.var_vdc_t);
        let assign2480_e2370: f64 = (assign2480_e2368 * locals.var_vtinv);
        let assign2480_e2372: f64 = (assign2480_e2370 - p.p138);
        let assign2480_e2373: f64 = (1.0 + assign2480_e2372);
        let assign2480_e2374: f64 = (locals.var_expl * assign2480_e2373);
        (assign2480_e2374, (locals.var_expl * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdc_t_dn3) * locals.var_vtinv) + (assign2480_e2368 * locals.var_vtinv_dn3))), (locals.var_expl * ((-locals.var_vdc_t_dn4) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2c1_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2c1_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c1vdc, locals.var_evb2c1vdc_dn0, locals.var_evb2c1vdc_dn1, locals.var_evb2c1vdc_dn3, locals.var_evb2c1vdc_dn4, locals.var_evb2c1vdc_dn5, locals.var_evb2c1vdc_dn6, locals.var_evb2c1vdc_dn7, locals.var_evb2c1vdc_dn8, locals.var_evb2c1vdc_dn9, locals.var_evb2c1vdc_dn10,)
    }
};
        locals.var_evb2c1vdc = assign2480_e2376;
        locals.var_evb2c1vdc_dn0 = assign2480_e2376_d_n0;
        locals.var_evb2c1vdc_dn1 = assign2480_e2376_d_n1;
        locals.var_evb2c1vdc_dn3 = assign2480_e2376_d_n3;
        locals.var_evb2c1vdc_dn4 = assign2480_e2376_d_n4;
        locals.var_evb2c1vdc_dn5 = assign2480_e2376_d_n5;
        locals.var_evb2c1vdc_dn6 = assign2480_e2376_d_n6;
        locals.var_evb2c1vdc_dn7 = assign2480_e2376_d_n7;
        locals.var_evb2c1vdc_dn8 = assign2480_e2376_d_n8;
        locals.var_evb2c1vdc_dn9 = assign2480_e2376_d_n9;
        locals.var_evb2c1vdc_dn10 = assign2480_e2376_d_n10;
        locals.var_evb2c1vdc_rv = 0.0;

        let assign2490_e2380: f64 = (4.0 * locals.var_evb2c2vdc);
        let assign2490_e2381: f64 = (1.0 + assign2490_e2380);
        let assign2490_e2382: f64 = (assign2490_e2381).sqrt();
        locals.var_k0 = assign2490_e2382;
        locals.var_k0_dn0 = ((4.0 * locals.var_evb2c2vdc_dn0) / (2.0 * assign2490_e2382));
        locals.var_k0_dn1 = ((4.0 * locals.var_evb2c2vdc_dn1) / (2.0 * assign2490_e2382));
        locals.var_k0_dn3 = ((4.0 * locals.var_evb2c2vdc_dn3) / (2.0 * assign2490_e2382));
        locals.var_k0_dn4 = ((4.0 * locals.var_evb2c2vdc_dn4) / (2.0 * assign2490_e2382));
        locals.var_k0_dn5 = ((4.0 * locals.var_evb2c2vdc_dn5) / (2.0 * assign2490_e2382));
        locals.var_k0_dn6 = ((4.0 * locals.var_evb2c2vdc_dn6) / (2.0 * assign2490_e2382));
        locals.var_k0_dn7 = ((4.0 * locals.var_evb2c2vdc_dn7) / (2.0 * assign2490_e2382));
        locals.var_k0_dn8 = ((4.0 * locals.var_evb2c2vdc_dn8) / (2.0 * assign2490_e2382));
        locals.var_k0_dn9 = ((4.0 * locals.var_evb2c2vdc_dn9) / (2.0 * assign2490_e2382));
        locals.var_k0_dn10 = ((4.0 * locals.var_evb2c2vdc_dn10) / (2.0 * assign2490_e2382));
        locals.var_k0_rv = 0.0;

        let assign2500_e2386: f64 = (4.0 * locals.var_evb2c1vdc);
        let assign2500_e2387: f64 = (1.0 + assign2500_e2386);
        let assign2500_e2388: f64 = (assign2500_e2387).sqrt();
        locals.var_kw = assign2500_e2388;
        locals.var_kw_dn0 = ((4.0 * locals.var_evb2c1vdc_dn0) / (2.0 * assign2500_e2388));
        locals.var_kw_dn1 = ((4.0 * locals.var_evb2c1vdc_dn1) / (2.0 * assign2500_e2388));
        locals.var_kw_dn3 = ((4.0 * locals.var_evb2c1vdc_dn3) / (2.0 * assign2500_e2388));
        locals.var_kw_dn4 = ((4.0 * locals.var_evb2c1vdc_dn4) / (2.0 * assign2500_e2388));
        locals.var_kw_dn5 = ((4.0 * locals.var_evb2c1vdc_dn5) / (2.0 * assign2500_e2388));
        locals.var_kw_dn6 = ((4.0 * locals.var_evb2c1vdc_dn6) / (2.0 * assign2500_e2388));
        locals.var_kw_dn7 = ((4.0 * locals.var_evb2c1vdc_dn7) / (2.0 * assign2500_e2388));
        locals.var_kw_dn8 = ((4.0 * locals.var_evb2c1vdc_dn8) / (2.0 * assign2500_e2388));
        locals.var_kw_dn9 = ((4.0 * locals.var_evb2c1vdc_dn9) / (2.0 * assign2500_e2388));
        locals.var_kw_dn10 = ((4.0 * locals.var_evb2c1vdc_dn10) / (2.0 * assign2500_e2388));
        locals.var_kw_rv = 0.0;

        let assign2510_e2391: f64 = (2.0 * locals.var_evb2c1vdc);
        let assign2510_e2394: f64 = (1.0 + locals.var_kw);
        let assign2510_e2395: f64 = (assign2510_e2391 / assign2510_e2394);
        locals.var_pw = assign2510_e2395;
        locals.var_pw_dn0 = ((((2.0 * locals.var_evb2c1vdc_dn0) * assign2510_e2394) - (assign2510_e2391 * locals.var_kw_dn0)) / (assign2510_e2394 * assign2510_e2394));
        locals.var_pw_dn1 = ((((2.0 * locals.var_evb2c1vdc_dn1) * assign2510_e2394) - (assign2510_e2391 * locals.var_kw_dn1)) / (assign2510_e2394 * assign2510_e2394));
        locals.var_pw_dn3 = ((((2.0 * locals.var_evb2c1vdc_dn3) * assign2510_e2394) - (assign2510_e2391 * locals.var_kw_dn3)) / (assign2510_e2394 * assign2510_e2394));
        locals.var_pw_dn4 = ((((2.0 * locals.var_evb2c1vdc_dn4) * assign2510_e2394) - (assign2510_e2391 * locals.var_kw_dn4)) / (assign2510_e2394 * assign2510_e2394));
        locals.var_pw_dn5 = ((((2.0 * locals.var_evb2c1vdc_dn5) * assign2510_e2394) - (assign2510_e2391 * locals.var_kw_dn5)) / (assign2510_e2394 * assign2510_e2394));
        locals.var_pw_dn6 = ((((2.0 * locals.var_evb2c1vdc_dn6) * assign2510_e2394) - (assign2510_e2391 * locals.var_kw_dn6)) / (assign2510_e2394 * assign2510_e2394));
        locals.var_pw_dn7 = ((((2.0 * locals.var_evb2c1vdc_dn7) * assign2510_e2394) - (assign2510_e2391 * locals.var_kw_dn7)) / (assign2510_e2394 * assign2510_e2394));
        locals.var_pw_dn8 = ((((2.0 * locals.var_evb2c1vdc_dn8) * assign2510_e2394) - (assign2510_e2391 * locals.var_kw_dn8)) / (assign2510_e2394 * assign2510_e2394));
        locals.var_pw_dn9 = ((((2.0 * locals.var_evb2c1vdc_dn9) * assign2510_e2394) - (assign2510_e2391 * locals.var_kw_dn9)) / (assign2510_e2394 * assign2510_e2394));
        locals.var_pw_dn10 = ((((2.0 * locals.var_evb2c1vdc_dn10) * assign2510_e2394) - (assign2510_e2391 * locals.var_kw_dn10)) / (assign2510_e2394 * assign2510_e2394));
        locals.var_pw_rv = 0.0;

        let assign2520_e2398: f64 = if locals.var_pw < p.p140 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign2520_e2398;
        locals.var_guard39_rv = 0.0;

        let (assign2530_e2402, assign2530_e2402_d_n0, assign2530_e2402_d_n1, assign2530_e2402_d_n3, assign2530_e2402_d_n4, assign2530_e2402_d_n5, assign2530_e2402_d_n6, assign2530_e2402_d_n7, assign2530_e2402_d_n8, assign2530_e2402_d_n9, assign2530_e2402_d_n10,) = {
    if (locals.var_guard39 != 0.0) {
        (p.p140, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pw, locals.var_pw_dn0, locals.var_pw_dn1, locals.var_pw_dn3, locals.var_pw_dn4, locals.var_pw_dn5, locals.var_pw_dn6, locals.var_pw_dn7, locals.var_pw_dn8, locals.var_pw_dn9, locals.var_pw_dn10,)
    }
};
        locals.var_pw = assign2530_e2402;
        locals.var_pw_dn0 = assign2530_e2402_d_n0;
        locals.var_pw_dn1 = assign2530_e2402_d_n1;
        locals.var_pw_dn3 = assign2530_e2402_d_n3;
        locals.var_pw_dn4 = assign2530_e2402_d_n4;
        locals.var_pw_dn5 = assign2530_e2402_d_n5;
        locals.var_pw_dn6 = assign2530_e2402_d_n6;
        locals.var_pw_dn7 = assign2530_e2402_d_n7;
        locals.var_pw_dn8 = assign2530_e2402_d_n8;
        locals.var_pw_dn9 = assign2530_e2402_d_n9;
        locals.var_pw_dn10 = assign2530_e2402_d_n10;
        locals.var_pw_rv = 0.0;

        let assign2540_e2406: f64 = (locals.var_k0 - locals.var_kw);
        let assign2540_e2409: f64 = (locals.var_k0 + 1.0);
        let assign2540_e2412: f64 = (locals.var_kw + 1.0);
        let assign2540_e2413: f64 = (assign2540_e2409 / assign2540_e2412);
        let assign2540_e2414: f64 = (assign2540_e2413).ln();
        let assign2540_e2415: f64 = (assign2540_e2406 - assign2540_e2414);
        let assign2540_e2416: f64 = (locals.var_vt * assign2540_e2415);
        locals.var_ec = assign2540_e2416;
        locals.var_ec_dn0 = (locals.var_vt * ((locals.var_k0_dn0 - locals.var_kw_dn0) - ((((locals.var_k0_dn0 * assign2540_e2412) - (assign2540_e2409 * locals.var_kw_dn0)) / (assign2540_e2412 * assign2540_e2412)) / assign2540_e2413)));
        locals.var_ec_dn1 = (locals.var_vt * ((locals.var_k0_dn1 - locals.var_kw_dn1) - ((((locals.var_k0_dn1 * assign2540_e2412) - (assign2540_e2409 * locals.var_kw_dn1)) / (assign2540_e2412 * assign2540_e2412)) / assign2540_e2413)));
        locals.var_ec_dn3 = ((locals.var_vt_dn3 * assign2540_e2415) + (locals.var_vt * ((locals.var_k0_dn3 - locals.var_kw_dn3) - ((((locals.var_k0_dn3 * assign2540_e2412) - (assign2540_e2409 * locals.var_kw_dn3)) / (assign2540_e2412 * assign2540_e2412)) / assign2540_e2413))));
        locals.var_ec_dn4 = (locals.var_vt * ((locals.var_k0_dn4 - locals.var_kw_dn4) - ((((locals.var_k0_dn4 * assign2540_e2412) - (assign2540_e2409 * locals.var_kw_dn4)) / (assign2540_e2412 * assign2540_e2412)) / assign2540_e2413)));
        locals.var_ec_dn5 = (locals.var_vt * ((locals.var_k0_dn5 - locals.var_kw_dn5) - ((((locals.var_k0_dn5 * assign2540_e2412) - (assign2540_e2409 * locals.var_kw_dn5)) / (assign2540_e2412 * assign2540_e2412)) / assign2540_e2413)));
        locals.var_ec_dn6 = (locals.var_vt * ((locals.var_k0_dn6 - locals.var_kw_dn6) - ((((locals.var_k0_dn6 * assign2540_e2412) - (assign2540_e2409 * locals.var_kw_dn6)) / (assign2540_e2412 * assign2540_e2412)) / assign2540_e2413)));
        locals.var_ec_dn7 = (locals.var_vt * ((locals.var_k0_dn7 - locals.var_kw_dn7) - ((((locals.var_k0_dn7 * assign2540_e2412) - (assign2540_e2409 * locals.var_kw_dn7)) / (assign2540_e2412 * assign2540_e2412)) / assign2540_e2413)));
        locals.var_ec_dn8 = (locals.var_vt * ((locals.var_k0_dn8 - locals.var_kw_dn8) - ((((locals.var_k0_dn8 * assign2540_e2412) - (assign2540_e2409 * locals.var_kw_dn8)) / (assign2540_e2412 * assign2540_e2412)) / assign2540_e2413)));
        locals.var_ec_dn9 = (locals.var_vt * ((locals.var_k0_dn9 - locals.var_kw_dn9) - ((((locals.var_k0_dn9 * assign2540_e2412) - (assign2540_e2409 * locals.var_kw_dn9)) / (assign2540_e2412 * assign2540_e2412)) / assign2540_e2413)));
        locals.var_ec_dn10 = (locals.var_vt * ((locals.var_k0_dn10 - locals.var_kw_dn10) - ((((locals.var_k0_dn10 * assign2540_e2412) - (assign2540_e2409 * locals.var_kw_dn10)) / (assign2540_e2412 * assign2540_e2412)) / assign2540_e2413)));
        locals.var_ec_rv = 0.0;

        let assign2550_e2419: f64 = (locals.var_ec + locals.var_vc1c2);
        let assign2550_e2421: f64 = (assign2550_e2419 / locals.var_rcv_t);
        locals.var_ic1c2 = assign2550_e2421;
        locals.var_ic1c2_dn0 = (locals.var_ec_dn0 / locals.var_rcv_t);
        locals.var_ic1c2_dn1 = (locals.var_ec_dn1 / locals.var_rcv_t);
        locals.var_ic1c2_dn3 = (((locals.var_ec_dn3 * locals.var_rcv_t) - (assign2550_e2419 * locals.var_rcv_t_dn3)) / (locals.var_rcv_t * locals.var_rcv_t));
        locals.var_ic1c2_dn4 = (locals.var_ec_dn4 / locals.var_rcv_t);
        locals.var_ic1c2_dn5 = (locals.var_ec_dn5 / locals.var_rcv_t);
        locals.var_ic1c2_dn6 = (locals.var_ec_dn6 / locals.var_rcv_t);
        locals.var_ic1c2_dn7 = ((locals.var_ec_dn7 + locals.var_vc1c2_dn7) / locals.var_rcv_t);
        locals.var_ic1c2_dn8 = ((locals.var_ec_dn8 + locals.var_vc1c2_dn8) / locals.var_rcv_t);
        locals.var_ic1c2_dn9 = (locals.var_ec_dn9 / locals.var_rcv_t);
        locals.var_ic1c2_dn10 = (locals.var_ec_dn10 / locals.var_rcv_t);
        locals.var_ic1c2_rv = 0.0;

        let assign2560_e2424: f64 = if locals.var_ic1c2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign2560_e2424;
        locals.var_guard40_rv = 0.0;

        let assign2570_e2427: f64 = if locals.var_vb2c1 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign2570_e2427;
        locals.var_guard41_rv = 0.0;

        let (assign2580_e2433, assign2580_e2433_d_n6, assign2580_e2433_d_n7,) = {
    if ((locals.var_guard40 != 0.0) && (locals.var_guard41 != 0.0)) {
        (locals.var_vb2c1, locals.var_vb2c1_dn6, locals.var_vb2c1_dn7,)
    } else {
        (locals.var_tmpv, locals.var_tmpv_dn6, locals.var_tmpv_dn7,)
    }
};
        locals.var_tmpv = assign2580_e2433;
        locals.var_tmpv_dn6 = assign2580_e2433_d_n6;
        locals.var_tmpv_dn7 = assign2580_e2433_d_n7;
        locals.var_tmpv_rv = 0.0;

        let (assign2590_e2447, assign2590_e2447_d_n6, assign2590_e2447_d_n7,) = {
    if ((locals.var_guard40 != 0.0) && (locals.var_guard41 == 0.0)) {
        let assign2590_e2442: f64 = (locals.var_vb2c1 - 100.0);
        let assign2590_e2443: f64 = (1.0 + assign2590_e2442);
        let assign2590_e2444: f64 = (assign2590_e2443).ln();
        let assign2590_e2445: f64 = (100.0 + assign2590_e2444);
        (assign2590_e2445, (locals.var_vb2c1_dn6 / assign2590_e2443), (locals.var_vb2c1_dn7 / assign2590_e2443),)
    } else {
        (locals.var_tmpv, locals.var_tmpv_dn6, locals.var_tmpv_dn7,)
    }
};
        locals.var_tmpv = assign2590_e2447;
        locals.var_tmpv_dn6 = assign2590_e2447_d_n6;
        locals.var_tmpv_dn7 = assign2590_e2447_d_n7;
        locals.var_tmpv_rv = 0.0;

        let (assign2600_e2468, assign2600_e2468_d_n0, assign2600_e2468_d_n1, assign2600_e2468_d_n3, assign2600_e2468_d_n4, assign2600_e2468_d_n5, assign2600_e2468_d_n6, assign2600_e2468_d_n7, assign2600_e2468_d_n8, assign2600_e2468_d_n9, assign2600_e2468_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2600_e2452: f64 = (2.0 * locals.var_vt);
        let assign2600_e2455: f64 = (0.5 * locals.var_ic1c2);
        let assign2600_e2457: f64 = (assign2600_e2455 * locals.var_rcv_t);
        let assign2600_e2459: f64 = (assign2600_e2457 * locals.var_vtinv);
        let assign2600_e2461: f64 = (assign2600_e2459 + 1.0);
        let assign2600_e2462: f64 = (assign2600_e2461).ln();
        let assign2600_e2463: f64 = (assign2600_e2452 * assign2600_e2462);
        let assign2600_e2464: f64 = (locals.var_vdc_t + assign2600_e2463);
        let assign2600_e2466: f64 = (assign2600_e2464 - locals.var_tmpv);
        (assign2600_e2466, (locals.var_vdc_t_dn0 + (assign2600_e2452 * ((((0.5 * locals.var_ic1c2_dn0) * locals.var_rcv_t) * locals.var_vtinv) / assign2600_e2461))), (locals.var_vdc_t_dn1 + (assign2600_e2452 * ((((0.5 * locals.var_ic1c2_dn1) * locals.var_rcv_t) * locals.var_vtinv) / assign2600_e2461))), (locals.var_vdc_t_dn3 + (((2.0 * locals.var_vt_dn3) * assign2600_e2462) + (assign2600_e2452 * ((((((0.5 * locals.var_ic1c2_dn3) * locals.var_rcv_t) + (assign2600_e2455 * locals.var_rcv_t_dn3)) * locals.var_vtinv) + (assign2600_e2457 * locals.var_vtinv_dn3)) / assign2600_e2461)))), (locals.var_vdc_t_dn4 + (assign2600_e2452 * ((((0.5 * locals.var_ic1c2_dn4) * locals.var_rcv_t) * locals.var_vtinv) / assign2600_e2461))), (locals.var_vdc_t_dn5 + (assign2600_e2452 * ((((0.5 * locals.var_ic1c2_dn5) * locals.var_rcv_t) * locals.var_vtinv) / assign2600_e2461))), ((locals.var_vdc_t_dn6 + (assign2600_e2452 * ((((0.5 * locals.var_ic1c2_dn6) * locals.var_rcv_t) * locals.var_vtinv) / assign2600_e2461))) - locals.var_tmpv_dn6), ((locals.var_vdc_t_dn7 + (assign2600_e2452 * ((((0.5 * locals.var_ic1c2_dn7) * locals.var_rcv_t) * locals.var_vtinv) / assign2600_e2461))) - locals.var_tmpv_dn7), (locals.var_vdc_t_dn8 + (assign2600_e2452 * ((((0.5 * locals.var_ic1c2_dn8) * locals.var_rcv_t) * locals.var_vtinv) / assign2600_e2461))), (locals.var_vdc_t_dn9 + (assign2600_e2452 * ((((0.5 * locals.var_ic1c2_dn9) * locals.var_rcv_t) * locals.var_vtinv) / assign2600_e2461))), (locals.var_vdc_t_dn10 + (assign2600_e2452 * ((((0.5 * locals.var_ic1c2_dn10) * locals.var_rcv_t) * locals.var_vtinv) / assign2600_e2461))),)
    } else {
        (locals.var_vqs_th, locals.var_vqs_th_dn0, locals.var_vqs_th_dn1, locals.var_vqs_th_dn3, locals.var_vqs_th_dn4, locals.var_vqs_th_dn5, locals.var_vqs_th_dn6, locals.var_vqs_th_dn7, locals.var_vqs_th_dn8, locals.var_vqs_th_dn9, locals.var_vqs_th_dn10,)
    }
};
        locals.var_vqs_th = assign2600_e2468;
        locals.var_vqs_th_dn0 = assign2600_e2468_d_n0;
        locals.var_vqs_th_dn1 = assign2600_e2468_d_n1;
        locals.var_vqs_th_dn3 = assign2600_e2468_d_n3;
        locals.var_vqs_th_dn4 = assign2600_e2468_d_n4;
        locals.var_vqs_th_dn5 = assign2600_e2468_d_n5;
        locals.var_vqs_th_dn6 = assign2600_e2468_d_n6;
        locals.var_vqs_th_dn7 = assign2600_e2468_d_n7;
        locals.var_vqs_th_dn8 = assign2600_e2468_d_n8;
        locals.var_vqs_th_dn9 = assign2600_e2468_d_n9;
        locals.var_vqs_th_dn10 = assign2600_e2468_d_n10;
        locals.var_vqs_th_rv = 0.0;

        let (assign2610_e2474, assign2610_e2474_d_n0, assign2610_e2474_d_n1, assign2610_e2474_d_n3, assign2610_e2474_d_n4, assign2610_e2474_d_n5, assign2610_e2474_d_n6, assign2610_e2474_d_n7, assign2610_e2474_d_n8, assign2610_e2474_d_n9, assign2610_e2474_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2610_e2472: f64 = (0.2 * locals.var_vdc_t);
        (assign2610_e2472, (0.2 * locals.var_vdc_t_dn0), (0.2 * locals.var_vdc_t_dn1), (0.2 * locals.var_vdc_t_dn3), (0.2 * locals.var_vdc_t_dn4), (0.2 * locals.var_vdc_t_dn5), (0.2 * locals.var_vdc_t_dn6), (0.2 * locals.var_vdc_t_dn7), (0.2 * locals.var_vdc_t_dn8), (0.2 * locals.var_vdc_t_dn9), (0.2 * locals.var_vdc_t_dn10),)
    } else {
        (locals.var_eps_vdc, locals.var_eps_vdc_dn0, locals.var_eps_vdc_dn1, locals.var_eps_vdc_dn3, locals.var_eps_vdc_dn4, locals.var_eps_vdc_dn5, locals.var_eps_vdc_dn6, locals.var_eps_vdc_dn7, locals.var_eps_vdc_dn8, locals.var_eps_vdc_dn9, locals.var_eps_vdc_dn10,)
    }
};
        locals.var_eps_vdc = assign2610_e2474;
        locals.var_eps_vdc_dn0 = assign2610_e2474_d_n0;
        locals.var_eps_vdc_dn1 = assign2610_e2474_d_n1;
        locals.var_eps_vdc_dn3 = assign2610_e2474_d_n3;
        locals.var_eps_vdc_dn4 = assign2610_e2474_d_n4;
        locals.var_eps_vdc_dn5 = assign2610_e2474_d_n5;
        locals.var_eps_vdc_dn6 = assign2610_e2474_d_n6;
        locals.var_eps_vdc_dn7 = assign2610_e2474_d_n7;
        locals.var_eps_vdc_dn8 = assign2610_e2474_d_n8;
        locals.var_eps_vdc_dn9 = assign2610_e2474_d_n9;
        locals.var_eps_vdc_dn10 = assign2610_e2474_d_n10;
        locals.var_eps_vdc_rv = 0.0;

        let (assign2620_e2480, assign2620_e2480_d_n0, assign2620_e2480_d_n1, assign2620_e2480_d_n3, assign2620_e2480_d_n4, assign2620_e2480_d_n5, assign2620_e2480_d_n6, assign2620_e2480_d_n7, assign2620_e2480_d_n8, assign2620_e2480_d_n9, assign2620_e2480_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2620_e2478: f64 = (locals.var_eps_vdc * locals.var_eps_vdc);
        (assign2620_e2478, ((locals.var_eps_vdc_dn0 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn0)), ((locals.var_eps_vdc_dn1 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn1)), ((locals.var_eps_vdc_dn3 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn3)), ((locals.var_eps_vdc_dn4 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn4)), ((locals.var_eps_vdc_dn5 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn5)), ((locals.var_eps_vdc_dn6 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn6)), ((locals.var_eps_vdc_dn7 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn7)), ((locals.var_eps_vdc_dn8 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn8)), ((locals.var_eps_vdc_dn9 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn9)), ((locals.var_eps_vdc_dn10 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn10)),)
    } else {
        (locals.var_eps2, locals.var_eps2_dn0, locals.var_eps2_dn1, locals.var_eps2_dn3, locals.var_eps2_dn4, locals.var_eps2_dn5, locals.var_eps2_dn6, locals.var_eps2_dn7, locals.var_eps2_dn8, locals.var_eps2_dn9, locals.var_eps2_dn10,)
    }
};
        locals.var_eps2 = assign2620_e2480;
        locals.var_eps2_dn0 = assign2620_e2480_d_n0;
        locals.var_eps2_dn1 = assign2620_e2480_d_n1;
        locals.var_eps2_dn3 = assign2620_e2480_d_n3;
        locals.var_eps2_dn4 = assign2620_e2480_d_n4;
        locals.var_eps2_dn5 = assign2620_e2480_d_n5;
        locals.var_eps2_dn6 = assign2620_e2480_d_n6;
        locals.var_eps2_dn7 = assign2620_e2480_d_n7;
        locals.var_eps2_dn8 = assign2620_e2480_d_n8;
        locals.var_eps2_dn9 = assign2620_e2480_d_n9;
        locals.var_eps2_dn10 = assign2620_e2480_d_n10;
        locals.var_eps2_rv = 0.0;

        let (assign2630_e2486, assign2630_e2486_d_n0, assign2630_e2486_d_n1, assign2630_e2486_d_n3, assign2630_e2486_d_n4, assign2630_e2486_d_n5, assign2630_e2486_d_n6, assign2630_e2486_d_n7, assign2630_e2486_d_n8, assign2630_e2486_d_n9, assign2630_e2486_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2630_e2484: f64 = (locals.var_vqs_th * locals.var_vqs_th);
        (assign2630_e2484, ((locals.var_vqs_th_dn0 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn0)), ((locals.var_vqs_th_dn1 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn1)), ((locals.var_vqs_th_dn3 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn3)), ((locals.var_vqs_th_dn4 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn4)), ((locals.var_vqs_th_dn5 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn5)), ((locals.var_vqs_th_dn6 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn6)), ((locals.var_vqs_th_dn7 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn7)), ((locals.var_vqs_th_dn8 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn8)), ((locals.var_vqs_th_dn9 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn9)), ((locals.var_vqs_th_dn10 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn10)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn1, locals.var_x2_dn3, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10,)
    }
};
        locals.var_x2 = assign2630_e2486;
        locals.var_x2_dn0 = assign2630_e2486_d_n0;
        locals.var_x2_dn1 = assign2630_e2486_d_n1;
        locals.var_x2_dn3 = assign2630_e2486_d_n3;
        locals.var_x2_dn4 = assign2630_e2486_d_n4;
        locals.var_x2_dn5 = assign2630_e2486_d_n5;
        locals.var_x2_dn6 = assign2630_e2486_d_n6;
        locals.var_x2_dn7 = assign2630_e2486_d_n7;
        locals.var_x2_dn8 = assign2630_e2486_d_n8;
        locals.var_x2_dn9 = assign2630_e2486_d_n9;
        locals.var_x2_dn10 = assign2630_e2486_d_n10;
        locals.var_x2_rv = 0.0;

        let assign2640_e2489: f64 = if locals.var_vqs_th < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign2640_e2489;
        locals.var_guard42_rv = 0.0;

        let (assign2650_e2504, assign2650_e2504_d_n0, assign2650_e2504_d_n1, assign2650_e2504_d_n3, assign2650_e2504_d_n4, assign2650_e2504_d_n5, assign2650_e2504_d_n6, assign2650_e2504_d_n7, assign2650_e2504_d_n8, assign2650_e2504_d_n9, assign2650_e2504_d_n10,) = {
    if ((locals.var_guard40 != 0.0) && (locals.var_guard42 != 0.0)) {
        let assign2650_e2495: f64 = (0.5 * locals.var_eps2);
        let assign2650_e2498: f64 = (locals.var_x2 + locals.var_eps2);
        let assign2650_e2499: f64 = (assign2650_e2498).sqrt();
        let assign2650_e2501: f64 = (assign2650_e2499 - locals.var_vqs_th);
        let assign2650_e2502: f64 = (assign2650_e2495 / assign2650_e2501);
        (assign2650_e2502, ((((0.5 * locals.var_eps2_dn0) * assign2650_e2501) - (assign2650_e2495 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign2650_e2499)) - locals.var_vqs_th_dn0))) / (assign2650_e2501 * assign2650_e2501)), ((((0.5 * locals.var_eps2_dn1) * assign2650_e2501) - (assign2650_e2495 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign2650_e2499)) - locals.var_vqs_th_dn1))) / (assign2650_e2501 * assign2650_e2501)), ((((0.5 * locals.var_eps2_dn3) * assign2650_e2501) - (assign2650_e2495 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign2650_e2499)) - locals.var_vqs_th_dn3))) / (assign2650_e2501 * assign2650_e2501)), ((((0.5 * locals.var_eps2_dn4) * assign2650_e2501) - (assign2650_e2495 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign2650_e2499)) - locals.var_vqs_th_dn4))) / (assign2650_e2501 * assign2650_e2501)), ((((0.5 * locals.var_eps2_dn5) * assign2650_e2501) - (assign2650_e2495 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign2650_e2499)) - locals.var_vqs_th_dn5))) / (assign2650_e2501 * assign2650_e2501)), ((((0.5 * locals.var_eps2_dn6) * assign2650_e2501) - (assign2650_e2495 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign2650_e2499)) - locals.var_vqs_th_dn6))) / (assign2650_e2501 * assign2650_e2501)), ((((0.5 * locals.var_eps2_dn7) * assign2650_e2501) - (assign2650_e2495 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign2650_e2499)) - locals.var_vqs_th_dn7))) / (assign2650_e2501 * assign2650_e2501)), ((((0.5 * locals.var_eps2_dn8) * assign2650_e2501) - (assign2650_e2495 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign2650_e2499)) - locals.var_vqs_th_dn8))) / (assign2650_e2501 * assign2650_e2501)), ((((0.5 * locals.var_eps2_dn9) * assign2650_e2501) - (assign2650_e2495 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign2650_e2499)) - locals.var_vqs_th_dn9))) / (assign2650_e2501 * assign2650_e2501)), ((((0.5 * locals.var_eps2_dn10) * assign2650_e2501) - (assign2650_e2495 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign2650_e2499)) - locals.var_vqs_th_dn10))) / (assign2650_e2501 * assign2650_e2501)),)
    } else {
        (locals.var_vqs, locals.var_vqs_dn0, locals.var_vqs_dn1, locals.var_vqs_dn3, locals.var_vqs_dn4, locals.var_vqs_dn5, locals.var_vqs_dn6, locals.var_vqs_dn7, locals.var_vqs_dn8, locals.var_vqs_dn9, locals.var_vqs_dn10,)
    }
};
        locals.var_vqs = assign2650_e2504;
        locals.var_vqs_dn0 = assign2650_e2504_d_n0;
        locals.var_vqs_dn1 = assign2650_e2504_d_n1;
        locals.var_vqs_dn3 = assign2650_e2504_d_n3;
        locals.var_vqs_dn4 = assign2650_e2504_d_n4;
        locals.var_vqs_dn5 = assign2650_e2504_d_n5;
        locals.var_vqs_dn6 = assign2650_e2504_d_n6;
        locals.var_vqs_dn7 = assign2650_e2504_d_n7;
        locals.var_vqs_dn8 = assign2650_e2504_d_n8;
        locals.var_vqs_dn9 = assign2650_e2504_d_n9;
        locals.var_vqs_dn10 = assign2650_e2504_d_n10;
        locals.var_vqs_rv = 0.0;

        let (assign2660_e2518, assign2660_e2518_d_n0, assign2660_e2518_d_n1, assign2660_e2518_d_n3, assign2660_e2518_d_n4, assign2660_e2518_d_n5, assign2660_e2518_d_n6, assign2660_e2518_d_n7, assign2660_e2518_d_n8, assign2660_e2518_d_n9, assign2660_e2518_d_n10,) = {
    if ((locals.var_guard40 != 0.0) && (locals.var_guard42 == 0.0)) {
        let assign2660_e2512: f64 = (locals.var_x2 + locals.var_eps2);
        let assign2660_e2513: f64 = (assign2660_e2512).sqrt();
        let assign2660_e2515: f64 = (assign2660_e2513 + locals.var_vqs_th);
        let assign2660_e2516: f64 = (0.5 * assign2660_e2515);
        (assign2660_e2516, (0.5 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign2660_e2513)) + locals.var_vqs_th_dn0)), (0.5 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign2660_e2513)) + locals.var_vqs_th_dn1)), (0.5 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign2660_e2513)) + locals.var_vqs_th_dn3)), (0.5 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign2660_e2513)) + locals.var_vqs_th_dn4)), (0.5 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign2660_e2513)) + locals.var_vqs_th_dn5)), (0.5 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign2660_e2513)) + locals.var_vqs_th_dn6)), (0.5 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign2660_e2513)) + locals.var_vqs_th_dn7)), (0.5 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign2660_e2513)) + locals.var_vqs_th_dn8)), (0.5 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign2660_e2513)) + locals.var_vqs_th_dn9)), (0.5 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign2660_e2513)) + locals.var_vqs_th_dn10)),)
    } else {
        (locals.var_vqs, locals.var_vqs_dn0, locals.var_vqs_dn1, locals.var_vqs_dn3, locals.var_vqs_dn4, locals.var_vqs_dn5, locals.var_vqs_dn6, locals.var_vqs_dn7, locals.var_vqs_dn8, locals.var_vqs_dn9, locals.var_vqs_dn10,)
    }
};
        locals.var_vqs = assign2660_e2518;
        locals.var_vqs_dn0 = assign2660_e2518_d_n0;
        locals.var_vqs_dn1 = assign2660_e2518_d_n1;
        locals.var_vqs_dn3 = assign2660_e2518_d_n3;
        locals.var_vqs_dn4 = assign2660_e2518_d_n4;
        locals.var_vqs_dn5 = assign2660_e2518_d_n5;
        locals.var_vqs_dn6 = assign2660_e2518_d_n6;
        locals.var_vqs_dn7 = assign2660_e2518_d_n7;
        locals.var_vqs_dn8 = assign2660_e2518_d_n8;
        locals.var_vqs_dn9 = assign2660_e2518_d_n9;
        locals.var_vqs_dn10 = assign2660_e2518_d_n10;
        locals.var_vqs_rv = 0.0;

        let (assign2670_e2536, assign2670_e2536_d_n0, assign2670_e2536_d_n1, assign2670_e2536_d_n3, assign2670_e2536_d_n4, assign2670_e2536_d_n5, assign2670_e2536_d_n6, assign2670_e2536_d_n7, assign2670_e2536_d_n8, assign2670_e2536_d_n9, assign2670_e2536_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2670_e2524: f64 = (p.p61 * p.p60);
        let assign2670_e2525: f64 = (locals.var_vqs + assign2670_e2524);
        let assign2670_e2526: f64 = (locals.var_vqs * assign2670_e2525);
        let assign2670_e2531: f64 = (p.p61 * locals.var_rcv_t);
        let assign2670_e2532: f64 = (locals.var_vqs + assign2670_e2531);
        let assign2670_e2533: f64 = (p.p60 * assign2670_e2532);
        let assign2670_e2534: f64 = (assign2670_e2526 / assign2670_e2533);
        (assign2670_e2534, (((((locals.var_vqs_dn0 * assign2670_e2525) + (locals.var_vqs * locals.var_vqs_dn0)) * assign2670_e2533) - (assign2670_e2526 * (p.p60 * locals.var_vqs_dn0))) / (assign2670_e2533 * assign2670_e2533)), (((((locals.var_vqs_dn1 * assign2670_e2525) + (locals.var_vqs * locals.var_vqs_dn1)) * assign2670_e2533) - (assign2670_e2526 * (p.p60 * locals.var_vqs_dn1))) / (assign2670_e2533 * assign2670_e2533)), (((((locals.var_vqs_dn3 * assign2670_e2525) + (locals.var_vqs * locals.var_vqs_dn3)) * assign2670_e2533) - (assign2670_e2526 * (p.p60 * (locals.var_vqs_dn3 + (p.p61 * locals.var_rcv_t_dn3))))) / (assign2670_e2533 * assign2670_e2533)), (((((locals.var_vqs_dn4 * assign2670_e2525) + (locals.var_vqs * locals.var_vqs_dn4)) * assign2670_e2533) - (assign2670_e2526 * (p.p60 * locals.var_vqs_dn4))) / (assign2670_e2533 * assign2670_e2533)), (((((locals.var_vqs_dn5 * assign2670_e2525) + (locals.var_vqs * locals.var_vqs_dn5)) * assign2670_e2533) - (assign2670_e2526 * (p.p60 * locals.var_vqs_dn5))) / (assign2670_e2533 * assign2670_e2533)), (((((locals.var_vqs_dn6 * assign2670_e2525) + (locals.var_vqs * locals.var_vqs_dn6)) * assign2670_e2533) - (assign2670_e2526 * (p.p60 * locals.var_vqs_dn6))) / (assign2670_e2533 * assign2670_e2533)), (((((locals.var_vqs_dn7 * assign2670_e2525) + (locals.var_vqs * locals.var_vqs_dn7)) * assign2670_e2533) - (assign2670_e2526 * (p.p60 * locals.var_vqs_dn7))) / (assign2670_e2533 * assign2670_e2533)), (((((locals.var_vqs_dn8 * assign2670_e2525) + (locals.var_vqs * locals.var_vqs_dn8)) * assign2670_e2533) - (assign2670_e2526 * (p.p60 * locals.var_vqs_dn8))) / (assign2670_e2533 * assign2670_e2533)), (((((locals.var_vqs_dn9 * assign2670_e2525) + (locals.var_vqs * locals.var_vqs_dn9)) * assign2670_e2533) - (assign2670_e2526 * (p.p60 * locals.var_vqs_dn9))) / (assign2670_e2533 * assign2670_e2533)), (((((locals.var_vqs_dn10 * assign2670_e2525) + (locals.var_vqs * locals.var_vqs_dn10)) * assign2670_e2533) - (assign2670_e2526 * (p.p60 * locals.var_vqs_dn10))) / (assign2670_e2533 * assign2670_e2533)),)
    } else {
        (locals.var_iqs, locals.var_iqs_dn0, locals.var_iqs_dn1, locals.var_iqs_dn3, locals.var_iqs_dn4, locals.var_iqs_dn5, locals.var_iqs_dn6, locals.var_iqs_dn7, locals.var_iqs_dn8, locals.var_iqs_dn9, locals.var_iqs_dn10,)
    }
};
        locals.var_iqs = assign2670_e2536;
        locals.var_iqs_dn0 = assign2670_e2536_d_n0;
        locals.var_iqs_dn1 = assign2670_e2536_d_n1;
        locals.var_iqs_dn3 = assign2670_e2536_d_n3;
        locals.var_iqs_dn4 = assign2670_e2536_d_n4;
        locals.var_iqs_dn5 = assign2670_e2536_d_n5;
        locals.var_iqs_dn6 = assign2670_e2536_d_n6;
        locals.var_iqs_dn7 = assign2670_e2536_d_n7;
        locals.var_iqs_dn8 = assign2670_e2536_d_n8;
        locals.var_iqs_dn9 = assign2670_e2536_d_n9;
        locals.var_iqs_dn10 = assign2670_e2536_d_n10;
        locals.var_iqs_rv = 0.0;

        let (assign2680_e2542, assign2680_e2542_d_n0, assign2680_e2542_d_n1, assign2680_e2542_d_n3, assign2680_e2542_d_n4, assign2680_e2542_d_n5, assign2680_e2542_d_n6, assign2680_e2542_d_n7, assign2680_e2542_d_n8, assign2680_e2542_d_n9, assign2680_e2542_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2680_e2540: f64 = (locals.var_ic1c2 / locals.var_iqs);
        (assign2680_e2540, (((locals.var_ic1c2_dn0 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn0)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn1 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn1)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn3 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn3)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn4 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn4)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn5 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn5)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn6 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn6)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn7 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn7)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn8 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn8)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn9 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn9)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn10 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn10)) / (locals.var_iqs * locals.var_iqs)),)
    } else {
        (locals.var_ic1c2_iqs, locals.var_ic1c2_iqs_dn0, locals.var_ic1c2_iqs_dn1, locals.var_ic1c2_iqs_dn3, locals.var_ic1c2_iqs_dn4, locals.var_ic1c2_iqs_dn5, locals.var_ic1c2_iqs_dn6, locals.var_ic1c2_iqs_dn7, locals.var_ic1c2_iqs_dn8, locals.var_ic1c2_iqs_dn9, locals.var_ic1c2_iqs_dn10,)
    }
};
        locals.var_ic1c2_iqs = assign2680_e2542;
        locals.var_ic1c2_iqs_dn0 = assign2680_e2542_d_n0;
        locals.var_ic1c2_iqs_dn1 = assign2680_e2542_d_n1;
        locals.var_ic1c2_iqs_dn3 = assign2680_e2542_d_n3;
        locals.var_ic1c2_iqs_dn4 = assign2680_e2542_d_n4;
        locals.var_ic1c2_iqs_dn5 = assign2680_e2542_d_n5;
        locals.var_ic1c2_iqs_dn6 = assign2680_e2542_d_n6;
        locals.var_ic1c2_iqs_dn7 = assign2680_e2542_d_n7;
        locals.var_ic1c2_iqs_dn8 = assign2680_e2542_d_n8;
        locals.var_ic1c2_iqs_dn9 = assign2680_e2542_d_n9;
        locals.var_ic1c2_iqs_dn10 = assign2680_e2542_d_n10;
        locals.var_ic1c2_iqs_rv = 0.0;

        let (assign2690_e2550, assign2690_e2550_d_n0, assign2690_e2550_d_n1, assign2690_e2550_d_n3, assign2690_e2550_d_n4, assign2690_e2550_d_n5, assign2690_e2550_d_n6, assign2690_e2550_d_n7, assign2690_e2550_d_n8, assign2690_e2550_d_n9, assign2690_e2550_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2690_e2546: f64 = (locals.var_ic1c2_iqs - 1.0);
        let assign2690_e2548: f64 = (assign2690_e2546 / p.p62);
        (assign2690_e2548, (locals.var_ic1c2_iqs_dn0 / p.p62), (locals.var_ic1c2_iqs_dn1 / p.p62), (locals.var_ic1c2_iqs_dn3 / p.p62), (locals.var_ic1c2_iqs_dn4 / p.p62), (locals.var_ic1c2_iqs_dn5 / p.p62), (locals.var_ic1c2_iqs_dn6 / p.p62), (locals.var_ic1c2_iqs_dn7 / p.p62), (locals.var_ic1c2_iqs_dn8 / p.p62), (locals.var_ic1c2_iqs_dn9 / p.p62), (locals.var_ic1c2_iqs_dn10 / p.p62),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9, locals.var_dxa_dn10,)
    }
};
        locals.var_dxa = assign2690_e2550;
        locals.var_dxa_dn0 = assign2690_e2550_d_n0;
        locals.var_dxa_dn1 = assign2690_e2550_d_n1;
        locals.var_dxa_dn3 = assign2690_e2550_d_n3;
        locals.var_dxa_dn4 = assign2690_e2550_d_n4;
        locals.var_dxa_dn5 = assign2690_e2550_d_n5;
        locals.var_dxa_dn6 = assign2690_e2550_d_n6;
        locals.var_dxa_dn7 = assign2690_e2550_d_n7;
        locals.var_dxa_dn8 = assign2690_e2550_d_n8;
        locals.var_dxa_dn9 = assign2690_e2550_d_n9;
        locals.var_dxa_dn10 = assign2690_e2550_d_n10;
        locals.var_dxa_rv = 0.0;

        let assign2700_e2553: f64 = if locals.var_ic1c2_iqs < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign2700_e2553;
        locals.var_guard43_rv = 0.0;

        let (assign2710_e2567, assign2710_e2567_d_n0, assign2710_e2567_d_n1, assign2710_e2567_d_n3, assign2710_e2567_d_n4, assign2710_e2567_d_n5, assign2710_e2567_d_n6, assign2710_e2567_d_n7, assign2710_e2567_d_n8, assign2710_e2567_d_n9, assign2710_e2567_d_n10,) = {
    if ((locals.var_guard40 != 0.0) && (locals.var_guard43 != 0.0)) {
        let assign2710_e2561: f64 = (locals.var_dxa).exp();
        let assign2710_e2562: f64 = (1.0 + assign2710_e2561);
        let assign2710_e2563: f64 = (assign2710_e2562).ln();
        let assign2710_e2564: f64 = (p.p62 * assign2710_e2563);
        let assign2710_e2565: f64 = (1.0 + assign2710_e2564);
        (assign2710_e2565, (p.p62 * ((assign2710_e2561 * locals.var_dxa_dn0) / assign2710_e2562)), (p.p62 * ((assign2710_e2561 * locals.var_dxa_dn1) / assign2710_e2562)), (p.p62 * ((assign2710_e2561 * locals.var_dxa_dn3) / assign2710_e2562)), (p.p62 * ((assign2710_e2561 * locals.var_dxa_dn4) / assign2710_e2562)), (p.p62 * ((assign2710_e2561 * locals.var_dxa_dn5) / assign2710_e2562)), (p.p62 * ((assign2710_e2561 * locals.var_dxa_dn6) / assign2710_e2562)), (p.p62 * ((assign2710_e2561 * locals.var_dxa_dn7) / assign2710_e2562)), (p.p62 * ((assign2710_e2561 * locals.var_dxa_dn8) / assign2710_e2562)), (p.p62 * ((assign2710_e2561 * locals.var_dxa_dn9) / assign2710_e2562)), (p.p62 * ((assign2710_e2561 * locals.var_dxa_dn10) / assign2710_e2562)),)
    } else {
        (locals.var_alpha1, locals.var_alpha1_dn0, locals.var_alpha1_dn1, locals.var_alpha1_dn3, locals.var_alpha1_dn4, locals.var_alpha1_dn5, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8, locals.var_alpha1_dn9, locals.var_alpha1_dn10,)
    }
};
        locals.var_alpha1 = assign2710_e2567;
        locals.var_alpha1_dn0 = assign2710_e2567_d_n0;
        locals.var_alpha1_dn1 = assign2710_e2567_d_n1;
        locals.var_alpha1_dn3 = assign2710_e2567_d_n3;
        locals.var_alpha1_dn4 = assign2710_e2567_d_n4;
        locals.var_alpha1_dn5 = assign2710_e2567_d_n5;
        locals.var_alpha1_dn6 = assign2710_e2567_d_n6;
        locals.var_alpha1_dn7 = assign2710_e2567_d_n7;
        locals.var_alpha1_dn8 = assign2710_e2567_d_n8;
        locals.var_alpha1_dn9 = assign2710_e2567_d_n9;
        locals.var_alpha1_dn10 = assign2710_e2567_d_n10;
        locals.var_alpha1_rv = 0.0;

        let (assign2720_e2583, assign2720_e2583_d_n0, assign2720_e2583_d_n1, assign2720_e2583_d_n3, assign2720_e2583_d_n4, assign2720_e2583_d_n5, assign2720_e2583_d_n6, assign2720_e2583_d_n7, assign2720_e2583_d_n8, assign2720_e2583_d_n9, assign2720_e2583_d_n10,) = {
    if ((locals.var_guard40 != 0.0) && (locals.var_guard43 == 0.0)) {
        let assign2720_e2576: f64 = (-locals.var_dxa);
        let assign2720_e2577: f64 = (assign2720_e2576).exp();
        let assign2720_e2578: f64 = (1.0 + assign2720_e2577);
        let assign2720_e2579: f64 = (assign2720_e2578).ln();
        let assign2720_e2580: f64 = (p.p62 * assign2720_e2579);
        let assign2720_e2581: f64 = (locals.var_ic1c2_iqs + assign2720_e2580);
        (assign2720_e2581, (locals.var_ic1c2_iqs_dn0 + (p.p62 * ((assign2720_e2577 * (-locals.var_dxa_dn0)) / assign2720_e2578))), (locals.var_ic1c2_iqs_dn1 + (p.p62 * ((assign2720_e2577 * (-locals.var_dxa_dn1)) / assign2720_e2578))), (locals.var_ic1c2_iqs_dn3 + (p.p62 * ((assign2720_e2577 * (-locals.var_dxa_dn3)) / assign2720_e2578))), (locals.var_ic1c2_iqs_dn4 + (p.p62 * ((assign2720_e2577 * (-locals.var_dxa_dn4)) / assign2720_e2578))), (locals.var_ic1c2_iqs_dn5 + (p.p62 * ((assign2720_e2577 * (-locals.var_dxa_dn5)) / assign2720_e2578))), (locals.var_ic1c2_iqs_dn6 + (p.p62 * ((assign2720_e2577 * (-locals.var_dxa_dn6)) / assign2720_e2578))), (locals.var_ic1c2_iqs_dn7 + (p.p62 * ((assign2720_e2577 * (-locals.var_dxa_dn7)) / assign2720_e2578))), (locals.var_ic1c2_iqs_dn8 + (p.p62 * ((assign2720_e2577 * (-locals.var_dxa_dn8)) / assign2720_e2578))), (locals.var_ic1c2_iqs_dn9 + (p.p62 * ((assign2720_e2577 * (-locals.var_dxa_dn9)) / assign2720_e2578))), (locals.var_ic1c2_iqs_dn10 + (p.p62 * ((assign2720_e2577 * (-locals.var_dxa_dn10)) / assign2720_e2578))),)
    } else {
        (locals.var_alpha1, locals.var_alpha1_dn0, locals.var_alpha1_dn1, locals.var_alpha1_dn3, locals.var_alpha1_dn4, locals.var_alpha1_dn5, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8, locals.var_alpha1_dn9, locals.var_alpha1_dn10,)
    }
};
        locals.var_alpha1 = assign2720_e2583;
        locals.var_alpha1_dn0 = assign2720_e2583_d_n0;
        locals.var_alpha1_dn1 = assign2720_e2583_d_n1;
        locals.var_alpha1_dn3 = assign2720_e2583_d_n3;
        locals.var_alpha1_dn4 = assign2720_e2583_d_n4;
        locals.var_alpha1_dn5 = assign2720_e2583_d_n5;
        locals.var_alpha1_dn6 = assign2720_e2583_d_n6;
        locals.var_alpha1_dn7 = assign2720_e2583_d_n7;
        locals.var_alpha1_dn8 = assign2720_e2583_d_n8;
        locals.var_alpha1_dn9 = assign2720_e2583_d_n9;
        locals.var_alpha1_dn10 = assign2720_e2583_d_n10;
        locals.var_alpha1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2730_e2600, assign2730_e2600_d_n0, assign2730_e2600_d_n1, assign2730_e2600_d_n3, assign2730_e2600_d_n4, assign2730_e2600_d_n5, assign2730_e2600_d_n6, assign2730_e2600_d_n7, assign2730_e2600_d_n8, assign2730_e2600_d_n9, assign2730_e2600_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2730_e2590: f64 = (-1.0);
        let assign2730_e2592: f64 = (assign2730_e2590 / p.p62);
        let assign2730_e2593: f64 = (assign2730_e2592).exp();
        let assign2730_e2594: f64 = (1.0 + assign2730_e2593);
        let assign2730_e2595: f64 = (assign2730_e2594).ln();
        let assign2730_e2596: f64 = (p.p62 * assign2730_e2595);
        let assign2730_e2597: f64 = (1.0 + assign2730_e2596);
        let assign2730_e2598: f64 = (locals.var_alpha1 / assign2730_e2597);
        (assign2730_e2598, (locals.var_alpha1_dn0 / assign2730_e2597), (locals.var_alpha1_dn1 / assign2730_e2597), (locals.var_alpha1_dn3 / assign2730_e2597), (locals.var_alpha1_dn4 / assign2730_e2597), (locals.var_alpha1_dn5 / assign2730_e2597), (locals.var_alpha1_dn6 / assign2730_e2597), (locals.var_alpha1_dn7 / assign2730_e2597), (locals.var_alpha1_dn8 / assign2730_e2597), (locals.var_alpha1_dn9 / assign2730_e2597), (locals.var_alpha1_dn10 / assign2730_e2597),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn1, locals.var_alpha_dn3, locals.var_alpha_dn4, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, locals.var_alpha_dn9, locals.var_alpha_dn10,)
    }
};
        locals.var_alpha = assign2730_e2600;
        locals.var_alpha_dn0 = assign2730_e2600_d_n0;
        locals.var_alpha_dn1 = assign2730_e2600_d_n1;
        locals.var_alpha_dn3 = assign2730_e2600_d_n3;
        locals.var_alpha_dn4 = assign2730_e2600_d_n4;
        locals.var_alpha_dn5 = assign2730_e2600_d_n5;
        locals.var_alpha_dn6 = assign2730_e2600_d_n6;
        locals.var_alpha_dn7 = assign2730_e2600_d_n7;
        locals.var_alpha_dn8 = assign2730_e2600_d_n8;
        locals.var_alpha_dn9 = assign2730_e2600_d_n9;
        locals.var_alpha_dn10 = assign2730_e2600_d_n10;
        locals.var_alpha_rv = 0.0;

        let (assign2740_e2608, assign2740_e2608_d_n0, assign2740_e2608_d_n1, assign2740_e2608_d_n3, assign2740_e2608_d_n4, assign2740_e2608_d_n5, assign2740_e2608_d_n6, assign2740_e2608_d_n7, assign2740_e2608_d_n8, assign2740_e2608_d_n9, assign2740_e2608_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2740_e2605: f64 = (p.p61 * p.p60);
        let assign2740_e2606: f64 = (locals.var_vqs / assign2740_e2605);
        (assign2740_e2606, (locals.var_vqs_dn0 / assign2740_e2605), (locals.var_vqs_dn1 / assign2740_e2605), (locals.var_vqs_dn3 / assign2740_e2605), (locals.var_vqs_dn4 / assign2740_e2605), (locals.var_vqs_dn5 / assign2740_e2605), (locals.var_vqs_dn6 / assign2740_e2605), (locals.var_vqs_dn7 / assign2740_e2605), (locals.var_vqs_dn8 / assign2740_e2605), (locals.var_vqs_dn9 / assign2740_e2605), (locals.var_vqs_dn10 / assign2740_e2605),)
    } else {
        (locals.var_vyi, locals.var_vyi_dn0, locals.var_vyi_dn1, locals.var_vyi_dn3, locals.var_vyi_dn4, locals.var_vyi_dn5, locals.var_vyi_dn6, locals.var_vyi_dn7, locals.var_vyi_dn8, locals.var_vyi_dn9, locals.var_vyi_dn10,)
    }
};
        locals.var_vyi = assign2740_e2608;
        locals.var_vyi_dn0 = assign2740_e2608_d_n0;
        locals.var_vyi_dn1 = assign2740_e2608_d_n1;
        locals.var_vyi_dn3 = assign2740_e2608_d_n3;
        locals.var_vyi_dn4 = assign2740_e2608_d_n4;
        locals.var_vyi_dn5 = assign2740_e2608_d_n5;
        locals.var_vyi_dn6 = assign2740_e2608_d_n6;
        locals.var_vyi_dn7 = assign2740_e2608_d_n7;
        locals.var_vyi_dn8 = assign2740_e2608_d_n8;
        locals.var_vyi_dn9 = assign2740_e2608_d_n9;
        locals.var_vyi_dn10 = assign2740_e2608_d_n10;
        locals.var_vyi_rv = 0.0;

        let (assign2750_e2633, assign2750_e2633_d_n0, assign2750_e2633_d_n1, assign2750_e2633_d_n3, assign2750_e2633_d_n4, assign2750_e2633_d_n5, assign2750_e2633_d_n6, assign2750_e2633_d_n7, assign2750_e2633_d_n8, assign2750_e2633_d_n9, assign2750_e2633_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2750_e2614: f64 = (4.0 * locals.var_alpha);
        let assign2750_e2616: f64 = (assign2750_e2614 * locals.var_vyi);
        let assign2750_e2619: f64 = (1.0 + locals.var_vyi);
        let assign2750_e2620: f64 = (assign2750_e2616 * assign2750_e2619);
        let assign2750_e2621: f64 = (1.0 + assign2750_e2620);
        let assign2750_e2622: f64 = (assign2750_e2621).sqrt();
        let assign2750_e2623: f64 = (1.0 + assign2750_e2622);
        let assign2750_e2626: f64 = (2.0 * locals.var_alpha);
        let assign2750_e2629: f64 = (1.0 + locals.var_vyi);
        let assign2750_e2630: f64 = (assign2750_e2626 * assign2750_e2629);
        let assign2750_e2631: f64 = (assign2750_e2623 / assign2750_e2630);
        (assign2750_e2631, (((((((((4.0 * locals.var_alpha_dn0) * locals.var_vyi) + (assign2750_e2614 * locals.var_vyi_dn0)) * assign2750_e2619) + (assign2750_e2616 * locals.var_vyi_dn0)) / (2.0 * assign2750_e2622)) * assign2750_e2630) - (assign2750_e2623 * (((2.0 * locals.var_alpha_dn0) * assign2750_e2629) + (assign2750_e2626 * locals.var_vyi_dn0)))) / (assign2750_e2630 * assign2750_e2630)), (((((((((4.0 * locals.var_alpha_dn1) * locals.var_vyi) + (assign2750_e2614 * locals.var_vyi_dn1)) * assign2750_e2619) + (assign2750_e2616 * locals.var_vyi_dn1)) / (2.0 * assign2750_e2622)) * assign2750_e2630) - (assign2750_e2623 * (((2.0 * locals.var_alpha_dn1) * assign2750_e2629) + (assign2750_e2626 * locals.var_vyi_dn1)))) / (assign2750_e2630 * assign2750_e2630)), (((((((((4.0 * locals.var_alpha_dn3) * locals.var_vyi) + (assign2750_e2614 * locals.var_vyi_dn3)) * assign2750_e2619) + (assign2750_e2616 * locals.var_vyi_dn3)) / (2.0 * assign2750_e2622)) * assign2750_e2630) - (assign2750_e2623 * (((2.0 * locals.var_alpha_dn3) * assign2750_e2629) + (assign2750_e2626 * locals.var_vyi_dn3)))) / (assign2750_e2630 * assign2750_e2630)), (((((((((4.0 * locals.var_alpha_dn4) * locals.var_vyi) + (assign2750_e2614 * locals.var_vyi_dn4)) * assign2750_e2619) + (assign2750_e2616 * locals.var_vyi_dn4)) / (2.0 * assign2750_e2622)) * assign2750_e2630) - (assign2750_e2623 * (((2.0 * locals.var_alpha_dn4) * assign2750_e2629) + (assign2750_e2626 * locals.var_vyi_dn4)))) / (assign2750_e2630 * assign2750_e2630)), (((((((((4.0 * locals.var_alpha_dn5) * locals.var_vyi) + (assign2750_e2614 * locals.var_vyi_dn5)) * assign2750_e2619) + (assign2750_e2616 * locals.var_vyi_dn5)) / (2.0 * assign2750_e2622)) * assign2750_e2630) - (assign2750_e2623 * (((2.0 * locals.var_alpha_dn5) * assign2750_e2629) + (assign2750_e2626 * locals.var_vyi_dn5)))) / (assign2750_e2630 * assign2750_e2630)), (((((((((4.0 * locals.var_alpha_dn6) * locals.var_vyi) + (assign2750_e2614 * locals.var_vyi_dn6)) * assign2750_e2619) + (assign2750_e2616 * locals.var_vyi_dn6)) / (2.0 * assign2750_e2622)) * assign2750_e2630) - (assign2750_e2623 * (((2.0 * locals.var_alpha_dn6) * assign2750_e2629) + (assign2750_e2626 * locals.var_vyi_dn6)))) / (assign2750_e2630 * assign2750_e2630)), (((((((((4.0 * locals.var_alpha_dn7) * locals.var_vyi) + (assign2750_e2614 * locals.var_vyi_dn7)) * assign2750_e2619) + (assign2750_e2616 * locals.var_vyi_dn7)) / (2.0 * assign2750_e2622)) * assign2750_e2630) - (assign2750_e2623 * (((2.0 * locals.var_alpha_dn7) * assign2750_e2629) + (assign2750_e2626 * locals.var_vyi_dn7)))) / (assign2750_e2630 * assign2750_e2630)), (((((((((4.0 * locals.var_alpha_dn8) * locals.var_vyi) + (assign2750_e2614 * locals.var_vyi_dn8)) * assign2750_e2619) + (assign2750_e2616 * locals.var_vyi_dn8)) / (2.0 * assign2750_e2622)) * assign2750_e2630) - (assign2750_e2623 * (((2.0 * locals.var_alpha_dn8) * assign2750_e2629) + (assign2750_e2626 * locals.var_vyi_dn8)))) / (assign2750_e2630 * assign2750_e2630)), (((((((((4.0 * locals.var_alpha_dn9) * locals.var_vyi) + (assign2750_e2614 * locals.var_vyi_dn9)) * assign2750_e2619) + (assign2750_e2616 * locals.var_vyi_dn9)) / (2.0 * assign2750_e2622)) * assign2750_e2630) - (assign2750_e2623 * (((2.0 * locals.var_alpha_dn9) * assign2750_e2629) + (assign2750_e2626 * locals.var_vyi_dn9)))) / (assign2750_e2630 * assign2750_e2630)), (((((((((4.0 * locals.var_alpha_dn10) * locals.var_vyi) + (assign2750_e2614 * locals.var_vyi_dn10)) * assign2750_e2619) + (assign2750_e2616 * locals.var_vyi_dn10)) / (2.0 * assign2750_e2622)) * assign2750_e2630) - (assign2750_e2623 * (((2.0 * locals.var_alpha_dn10) * assign2750_e2629) + (assign2750_e2626 * locals.var_vyi_dn10)))) / (assign2750_e2630 * assign2750_e2630)),)
    } else {
        (locals.var_yi, locals.var_yi_dn0, locals.var_yi_dn1, locals.var_yi_dn3, locals.var_yi_dn4, locals.var_yi_dn5, locals.var_yi_dn6, locals.var_yi_dn7, locals.var_yi_dn8, locals.var_yi_dn9, locals.var_yi_dn10,)
    }
};
        locals.var_yi = assign2750_e2633;
        locals.var_yi_dn0 = assign2750_e2633_d_n0;
        locals.var_yi_dn1 = assign2750_e2633_d_n1;
        locals.var_yi_dn3 = assign2750_e2633_d_n3;
        locals.var_yi_dn4 = assign2750_e2633_d_n4;
        locals.var_yi_dn5 = assign2750_e2633_d_n5;
        locals.var_yi_dn6 = assign2750_e2633_d_n6;
        locals.var_yi_dn7 = assign2750_e2633_d_n7;
        locals.var_yi_dn8 = assign2750_e2633_d_n8;
        locals.var_yi_dn9 = assign2750_e2633_d_n9;
        locals.var_yi_dn10 = assign2750_e2633_d_n10;
        locals.var_yi_rv = 0.0;

        let (assign2760_e2649, assign2760_e2649_d_n0, assign2760_e2649_d_n1, assign2760_e2649_d_n3, assign2760_e2649_d_n4, assign2760_e2649_d_n5, assign2760_e2649_d_n6, assign2760_e2649_d_n7, assign2760_e2649_d_n8, assign2760_e2649_d_n9, assign2760_e2649_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2760_e2637: f64 = (1.0 - locals.var_yi);
        let assign2760_e2640: f64 = (locals.var_pw * locals.var_yi);
        let assign2760_e2641: f64 = (assign2760_e2637 + assign2760_e2640);
        let assign2760_e2645: f64 = (locals.var_pw * locals.var_yi);
        let assign2760_e2646: f64 = (1.0 + assign2760_e2645);
        let assign2760_e2647: f64 = (assign2760_e2641 / assign2760_e2646);
        (assign2760_e2647, (((((-locals.var_yi_dn0) + ((locals.var_pw_dn0 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn0))) * assign2760_e2646) - (assign2760_e2641 * ((locals.var_pw_dn0 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn0)))) / (assign2760_e2646 * assign2760_e2646)), (((((-locals.var_yi_dn1) + ((locals.var_pw_dn1 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn1))) * assign2760_e2646) - (assign2760_e2641 * ((locals.var_pw_dn1 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn1)))) / (assign2760_e2646 * assign2760_e2646)), (((((-locals.var_yi_dn3) + ((locals.var_pw_dn3 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn3))) * assign2760_e2646) - (assign2760_e2641 * ((locals.var_pw_dn3 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn3)))) / (assign2760_e2646 * assign2760_e2646)), (((((-locals.var_yi_dn4) + ((locals.var_pw_dn4 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn4))) * assign2760_e2646) - (assign2760_e2641 * ((locals.var_pw_dn4 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn4)))) / (assign2760_e2646 * assign2760_e2646)), (((((-locals.var_yi_dn5) + ((locals.var_pw_dn5 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn5))) * assign2760_e2646) - (assign2760_e2641 * ((locals.var_pw_dn5 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn5)))) / (assign2760_e2646 * assign2760_e2646)), (((((-locals.var_yi_dn6) + ((locals.var_pw_dn6 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn6))) * assign2760_e2646) - (assign2760_e2641 * ((locals.var_pw_dn6 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn6)))) / (assign2760_e2646 * assign2760_e2646)), (((((-locals.var_yi_dn7) + ((locals.var_pw_dn7 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn7))) * assign2760_e2646) - (assign2760_e2641 * ((locals.var_pw_dn7 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn7)))) / (assign2760_e2646 * assign2760_e2646)), (((((-locals.var_yi_dn8) + ((locals.var_pw_dn8 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn8))) * assign2760_e2646) - (assign2760_e2641 * ((locals.var_pw_dn8 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn8)))) / (assign2760_e2646 * assign2760_e2646)), (((((-locals.var_yi_dn9) + ((locals.var_pw_dn9 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn9))) * assign2760_e2646) - (assign2760_e2641 * ((locals.var_pw_dn9 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn9)))) / (assign2760_e2646 * assign2760_e2646)), (((((-locals.var_yi_dn10) + ((locals.var_pw_dn10 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn10))) * assign2760_e2646) - (assign2760_e2641 * ((locals.var_pw_dn10 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn10)))) / (assign2760_e2646 * assign2760_e2646)),)
    } else {
        (locals.var_xi_w, locals.var_xi_w_dn0, locals.var_xi_w_dn1, locals.var_xi_w_dn3, locals.var_xi_w_dn4, locals.var_xi_w_dn5, locals.var_xi_w_dn6, locals.var_xi_w_dn7, locals.var_xi_w_dn8, locals.var_xi_w_dn9, locals.var_xi_w_dn10,)
    }
};
        locals.var_xi_w = assign2760_e2649;
        locals.var_xi_w_dn0 = assign2760_e2649_d_n0;
        locals.var_xi_w_dn1 = assign2760_e2649_d_n1;
        locals.var_xi_w_dn3 = assign2760_e2649_d_n3;
        locals.var_xi_w_dn4 = assign2760_e2649_d_n4;
        locals.var_xi_w_dn5 = assign2760_e2649_d_n5;
        locals.var_xi_w_dn6 = assign2760_e2649_d_n6;
        locals.var_xi_w_dn7 = assign2760_e2649_d_n7;
        locals.var_xi_w_dn8 = assign2760_e2649_d_n8;
        locals.var_xi_w_dn9 = assign2760_e2649_d_n9;
        locals.var_xi_w_dn10 = assign2760_e2649_d_n10;
        locals.var_xi_w_rv = 0.0;

        let (assign2770_e2661, assign2770_e2661_d_n0, assign2770_e2661_d_n1, assign2770_e2661_d_n3, assign2770_e2661_d_n4, assign2770_e2661_d_n5, assign2770_e2661_d_n6, assign2770_e2661_d_n7, assign2770_e2661_d_n8, assign2770_e2661_d_n9, assign2770_e2661_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2770_e2653: f64 = (0.5 * locals.var_ic1c2);
        let assign2770_e2655: f64 = (assign2770_e2653 * locals.var_rcv_t);
        let assign2770_e2657: f64 = (assign2770_e2655 * locals.var_xi_w);
        let assign2770_e2659: f64 = (assign2770_e2657 * locals.var_vtinv);
        (assign2770_e2659, (((((0.5 * locals.var_ic1c2_dn0) * locals.var_rcv_t) * locals.var_xi_w) + (assign2770_e2655 * locals.var_xi_w_dn0)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn1) * locals.var_rcv_t) * locals.var_xi_w) + (assign2770_e2655 * locals.var_xi_w_dn1)) * locals.var_vtinv), (((((((0.5 * locals.var_ic1c2_dn3) * locals.var_rcv_t) + (assign2770_e2653 * locals.var_rcv_t_dn3)) * locals.var_xi_w) + (assign2770_e2655 * locals.var_xi_w_dn3)) * locals.var_vtinv) + (assign2770_e2657 * locals.var_vtinv_dn3)), (((((0.5 * locals.var_ic1c2_dn4) * locals.var_rcv_t) * locals.var_xi_w) + (assign2770_e2655 * locals.var_xi_w_dn4)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn5) * locals.var_rcv_t) * locals.var_xi_w) + (assign2770_e2655 * locals.var_xi_w_dn5)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn6) * locals.var_rcv_t) * locals.var_xi_w) + (assign2770_e2655 * locals.var_xi_w_dn6)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn7) * locals.var_rcv_t) * locals.var_xi_w) + (assign2770_e2655 * locals.var_xi_w_dn7)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn8) * locals.var_rcv_t) * locals.var_xi_w) + (assign2770_e2655 * locals.var_xi_w_dn8)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn9) * locals.var_rcv_t) * locals.var_xi_w) + (assign2770_e2655 * locals.var_xi_w_dn9)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn10) * locals.var_rcv_t) * locals.var_xi_w) + (assign2770_e2655 * locals.var_xi_w_dn10)) * locals.var_vtinv),)
    } else {
        (locals.var_gp0, locals.var_gp0_dn0, locals.var_gp0_dn1, locals.var_gp0_dn3, locals.var_gp0_dn4, locals.var_gp0_dn5, locals.var_gp0_dn6, locals.var_gp0_dn7, locals.var_gp0_dn8, locals.var_gp0_dn9, locals.var_gp0_dn10,)
    }
};
        locals.var_gp0 = assign2770_e2661;
        locals.var_gp0_dn0 = assign2770_e2661_d_n0;
        locals.var_gp0_dn1 = assign2770_e2661_d_n1;
        locals.var_gp0_dn3 = assign2770_e2661_d_n3;
        locals.var_gp0_dn4 = assign2770_e2661_d_n4;
        locals.var_gp0_dn5 = assign2770_e2661_d_n5;
        locals.var_gp0_dn6 = assign2770_e2661_d_n6;
        locals.var_gp0_dn7 = assign2770_e2661_d_n7;
        locals.var_gp0_dn8 = assign2770_e2661_d_n8;
        locals.var_gp0_dn9 = assign2770_e2661_d_n9;
        locals.var_gp0_dn10 = assign2770_e2661_d_n10;
        locals.var_gp0_rv = 0.0;

        let (assign2780_e2675, assign2780_e2675_d_n0, assign2780_e2675_d_n1, assign2780_e2675_d_n3, assign2780_e2675_d_n4, assign2780_e2675_d_n5, assign2780_e2675_d_n6, assign2780_e2675_d_n7, assign2780_e2675_d_n8, assign2780_e2675_d_n9, assign2780_e2675_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2780_e2665: f64 = (2.0 * locals.var_gp0);
        let assign2780_e2669: f64 = (locals.var_pw + locals.var_gp0);
        let assign2780_e2671: f64 = (assign2780_e2669 + 1.0);
        let assign2780_e2672: f64 = (locals.var_pw * assign2780_e2671);
        let assign2780_e2673: f64 = (assign2780_e2665 + assign2780_e2672);
        (assign2780_e2673, ((2.0 * locals.var_gp0_dn0) + ((locals.var_pw_dn0 * assign2780_e2671) + (locals.var_pw * (locals.var_pw_dn0 + locals.var_gp0_dn0)))), ((2.0 * locals.var_gp0_dn1) + ((locals.var_pw_dn1 * assign2780_e2671) + (locals.var_pw * (locals.var_pw_dn1 + locals.var_gp0_dn1)))), ((2.0 * locals.var_gp0_dn3) + ((locals.var_pw_dn3 * assign2780_e2671) + (locals.var_pw * (locals.var_pw_dn3 + locals.var_gp0_dn3)))), ((2.0 * locals.var_gp0_dn4) + ((locals.var_pw_dn4 * assign2780_e2671) + (locals.var_pw * (locals.var_pw_dn4 + locals.var_gp0_dn4)))), ((2.0 * locals.var_gp0_dn5) + ((locals.var_pw_dn5 * assign2780_e2671) + (locals.var_pw * (locals.var_pw_dn5 + locals.var_gp0_dn5)))), ((2.0 * locals.var_gp0_dn6) + ((locals.var_pw_dn6 * assign2780_e2671) + (locals.var_pw * (locals.var_pw_dn6 + locals.var_gp0_dn6)))), ((2.0 * locals.var_gp0_dn7) + ((locals.var_pw_dn7 * assign2780_e2671) + (locals.var_pw * (locals.var_pw_dn7 + locals.var_gp0_dn7)))), ((2.0 * locals.var_gp0_dn8) + ((locals.var_pw_dn8 * assign2780_e2671) + (locals.var_pw * (locals.var_pw_dn8 + locals.var_gp0_dn8)))), ((2.0 * locals.var_gp0_dn9) + ((locals.var_pw_dn9 * assign2780_e2671) + (locals.var_pw * (locals.var_pw_dn9 + locals.var_gp0_dn9)))), ((2.0 * locals.var_gp0_dn10) + ((locals.var_pw_dn10 * assign2780_e2671) + (locals.var_pw * (locals.var_pw_dn10 + locals.var_gp0_dn10)))),)
    } else {
        (locals.var_gp0_help, locals.var_gp0_help_dn0, locals.var_gp0_help_dn1, locals.var_gp0_help_dn3, locals.var_gp0_help_dn4, locals.var_gp0_help_dn5, locals.var_gp0_help_dn6, locals.var_gp0_help_dn7, locals.var_gp0_help_dn8, locals.var_gp0_help_dn9, locals.var_gp0_help_dn10,)
    }
};
        locals.var_gp0_help = assign2780_e2675;
        locals.var_gp0_help_dn0 = assign2780_e2675_d_n0;
        locals.var_gp0_help_dn1 = assign2780_e2675_d_n1;
        locals.var_gp0_help_dn3 = assign2780_e2675_d_n3;
        locals.var_gp0_help_dn4 = assign2780_e2675_d_n4;
        locals.var_gp0_help_dn5 = assign2780_e2675_d_n5;
        locals.var_gp0_help_dn6 = assign2780_e2675_d_n6;
        locals.var_gp0_help_dn7 = assign2780_e2675_d_n7;
        locals.var_gp0_help_dn8 = assign2780_e2675_d_n8;
        locals.var_gp0_help_dn9 = assign2780_e2675_d_n9;
        locals.var_gp0_help_dn10 = assign2780_e2675_d_n10;
        locals.var_gp0_help_rv = 0.0;

        let (assign2790_e2683, assign2790_e2683_d_n0, assign2790_e2683_d_n1, assign2790_e2683_d_n3, assign2790_e2683_d_n4, assign2790_e2683_d_n5, assign2790_e2683_d_n6, assign2790_e2683_d_n7, assign2790_e2683_d_n8, assign2790_e2683_d_n9, assign2790_e2683_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2790_e2680: f64 = (locals.var_gp0 - 1.0);
        let assign2790_e2681: f64 = (0.5 * assign2790_e2680);
        (assign2790_e2681, (0.5 * locals.var_gp0_dn0), (0.5 * locals.var_gp0_dn1), (0.5 * locals.var_gp0_dn3), (0.5 * locals.var_gp0_dn4), (0.5 * locals.var_gp0_dn5), (0.5 * locals.var_gp0_dn6), (0.5 * locals.var_gp0_dn7), (0.5 * locals.var_gp0_dn8), (0.5 * locals.var_gp0_dn9), (0.5 * locals.var_gp0_dn10),)
    } else {
        (locals.var_gp02, locals.var_gp02_dn0, locals.var_gp02_dn1, locals.var_gp02_dn3, locals.var_gp02_dn4, locals.var_gp02_dn5, locals.var_gp02_dn6, locals.var_gp02_dn7, locals.var_gp02_dn8, locals.var_gp02_dn9, locals.var_gp02_dn10,)
    }
};
        locals.var_gp02 = assign2790_e2683;
        locals.var_gp02_dn0 = assign2790_e2683_d_n0;
        locals.var_gp02_dn1 = assign2790_e2683_d_n1;
        locals.var_gp02_dn3 = assign2790_e2683_d_n3;
        locals.var_gp02_dn4 = assign2790_e2683_d_n4;
        locals.var_gp02_dn5 = assign2790_e2683_d_n5;
        locals.var_gp02_dn6 = assign2790_e2683_d_n6;
        locals.var_gp02_dn7 = assign2790_e2683_d_n7;
        locals.var_gp02_dn8 = assign2790_e2683_d_n8;
        locals.var_gp02_dn9 = assign2790_e2683_d_n9;
        locals.var_gp02_dn10 = assign2790_e2683_d_n10;
        locals.var_gp02_rv = 0.0;

        let (assign2800_e2691, assign2800_e2691_d_n0, assign2800_e2691_d_n1, assign2800_e2691_d_n3, assign2800_e2691_d_n4, assign2800_e2691_d_n5, assign2800_e2691_d_n6, assign2800_e2691_d_n7, assign2800_e2691_d_n8, assign2800_e2691_d_n9, assign2800_e2691_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2800_e2687: f64 = (locals.var_gp02 * locals.var_gp02);
        let assign2800_e2689: f64 = (assign2800_e2687 + locals.var_gp0_help);
        (assign2800_e2689, (((locals.var_gp02_dn0 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn0)) + locals.var_gp0_help_dn0), (((locals.var_gp02_dn1 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn1)) + locals.var_gp0_help_dn1), (((locals.var_gp02_dn3 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn3)) + locals.var_gp0_help_dn3), (((locals.var_gp02_dn4 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn4)) + locals.var_gp0_help_dn4), (((locals.var_gp02_dn5 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn5)) + locals.var_gp0_help_dn5), (((locals.var_gp02_dn6 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn6)) + locals.var_gp0_help_dn6), (((locals.var_gp02_dn7 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn7)) + locals.var_gp0_help_dn7), (((locals.var_gp02_dn8 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn8)) + locals.var_gp0_help_dn8), (((locals.var_gp02_dn9 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn9)) + locals.var_gp0_help_dn9), (((locals.var_gp02_dn10 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn10)) + locals.var_gp0_help_dn10),)
    } else {
        (locals.var_sqr_arg, locals.var_sqr_arg_dn0, locals.var_sqr_arg_dn1, locals.var_sqr_arg_dn3, locals.var_sqr_arg_dn4, locals.var_sqr_arg_dn5, locals.var_sqr_arg_dn6, locals.var_sqr_arg_dn7, locals.var_sqr_arg_dn8, locals.var_sqr_arg_dn9, locals.var_sqr_arg_dn10,)
    }
};
        locals.var_sqr_arg = assign2800_e2691;
        locals.var_sqr_arg_dn0 = assign2800_e2691_d_n0;
        locals.var_sqr_arg_dn1 = assign2800_e2691_d_n1;
        locals.var_sqr_arg_dn3 = assign2800_e2691_d_n3;
        locals.var_sqr_arg_dn4 = assign2800_e2691_d_n4;
        locals.var_sqr_arg_dn5 = assign2800_e2691_d_n5;
        locals.var_sqr_arg_dn6 = assign2800_e2691_d_n6;
        locals.var_sqr_arg_dn7 = assign2800_e2691_d_n7;
        locals.var_sqr_arg_dn8 = assign2800_e2691_d_n8;
        locals.var_sqr_arg_dn9 = assign2800_e2691_d_n9;
        locals.var_sqr_arg_dn10 = assign2800_e2691_d_n10;
        locals.var_sqr_arg_rv = 0.0;

        let assign2810_e2694: f64 = if locals.var_gp0 >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign2810_e2694;
        locals.var_guard44_rv = 0.0;

        let (assign2820_e2703, assign2820_e2703_d_n0, assign2820_e2703_d_n1, assign2820_e2703_d_n3, assign2820_e2703_d_n4, assign2820_e2703_d_n5, assign2820_e2703_d_n6, assign2820_e2703_d_n7, assign2820_e2703_d_n8, assign2820_e2703_d_n9, assign2820_e2703_d_n10,) = {
    if ((locals.var_guard40 != 0.0) && (locals.var_guard44 != 0.0)) {
        let assign2820_e2700: f64 = (locals.var_sqr_arg).sqrt();
        let assign2820_e2701: f64 = (locals.var_gp02 + assign2820_e2700);
        (assign2820_e2701, (locals.var_gp02_dn0 + (locals.var_sqr_arg_dn0 / (2.0 * assign2820_e2700))), (locals.var_gp02_dn1 + (locals.var_sqr_arg_dn1 / (2.0 * assign2820_e2700))), (locals.var_gp02_dn3 + (locals.var_sqr_arg_dn3 / (2.0 * assign2820_e2700))), (locals.var_gp02_dn4 + (locals.var_sqr_arg_dn4 / (2.0 * assign2820_e2700))), (locals.var_gp02_dn5 + (locals.var_sqr_arg_dn5 / (2.0 * assign2820_e2700))), (locals.var_gp02_dn6 + (locals.var_sqr_arg_dn6 / (2.0 * assign2820_e2700))), (locals.var_gp02_dn7 + (locals.var_sqr_arg_dn7 / (2.0 * assign2820_e2700))), (locals.var_gp02_dn8 + (locals.var_sqr_arg_dn8 / (2.0 * assign2820_e2700))), (locals.var_gp02_dn9 + (locals.var_sqr_arg_dn9 / (2.0 * assign2820_e2700))), (locals.var_gp02_dn10 + (locals.var_sqr_arg_dn10 / (2.0 * assign2820_e2700))),)
    } else {
        (locals.var_p0star, locals.var_p0star_dn0, locals.var_p0star_dn1, locals.var_p0star_dn3, locals.var_p0star_dn4, locals.var_p0star_dn5, locals.var_p0star_dn6, locals.var_p0star_dn7, locals.var_p0star_dn8, locals.var_p0star_dn9, locals.var_p0star_dn10,)
    }
};
        locals.var_p0star = assign2820_e2703;
        locals.var_p0star_dn0 = assign2820_e2703_d_n0;
        locals.var_p0star_dn1 = assign2820_e2703_d_n1;
        locals.var_p0star_dn3 = assign2820_e2703_d_n3;
        locals.var_p0star_dn4 = assign2820_e2703_d_n4;
        locals.var_p0star_dn5 = assign2820_e2703_d_n5;
        locals.var_p0star_dn6 = assign2820_e2703_d_n6;
        locals.var_p0star_dn7 = assign2820_e2703_d_n7;
        locals.var_p0star_dn8 = assign2820_e2703_d_n8;
        locals.var_p0star_dn9 = assign2820_e2703_d_n9;
        locals.var_p0star_dn10 = assign2820_e2703_d_n10;
        locals.var_p0star_rv = 0.0;

        let (assign2830_e2715, assign2830_e2715_d_n0, assign2830_e2715_d_n1, assign2830_e2715_d_n3, assign2830_e2715_d_n4, assign2830_e2715_d_n5, assign2830_e2715_d_n6, assign2830_e2715_d_n7, assign2830_e2715_d_n8, assign2830_e2715_d_n9, assign2830_e2715_d_n10,) = {
    if ((locals.var_guard40 != 0.0) && (locals.var_guard44 == 0.0)) {
        let assign2830_e2710: f64 = (locals.var_sqr_arg).sqrt();
        let assign2830_e2712: f64 = (assign2830_e2710 - locals.var_gp02);
        let assign2830_e2713: f64 = (locals.var_gp0_help / assign2830_e2712);
        (assign2830_e2713, (((locals.var_gp0_help_dn0 * assign2830_e2712) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn0 / (2.0 * assign2830_e2710)) - locals.var_gp02_dn0))) / (assign2830_e2712 * assign2830_e2712)), (((locals.var_gp0_help_dn1 * assign2830_e2712) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn1 / (2.0 * assign2830_e2710)) - locals.var_gp02_dn1))) / (assign2830_e2712 * assign2830_e2712)), (((locals.var_gp0_help_dn3 * assign2830_e2712) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn3 / (2.0 * assign2830_e2710)) - locals.var_gp02_dn3))) / (assign2830_e2712 * assign2830_e2712)), (((locals.var_gp0_help_dn4 * assign2830_e2712) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn4 / (2.0 * assign2830_e2710)) - locals.var_gp02_dn4))) / (assign2830_e2712 * assign2830_e2712)), (((locals.var_gp0_help_dn5 * assign2830_e2712) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn5 / (2.0 * assign2830_e2710)) - locals.var_gp02_dn5))) / (assign2830_e2712 * assign2830_e2712)), (((locals.var_gp0_help_dn6 * assign2830_e2712) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn6 / (2.0 * assign2830_e2710)) - locals.var_gp02_dn6))) / (assign2830_e2712 * assign2830_e2712)), (((locals.var_gp0_help_dn7 * assign2830_e2712) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn7 / (2.0 * assign2830_e2710)) - locals.var_gp02_dn7))) / (assign2830_e2712 * assign2830_e2712)), (((locals.var_gp0_help_dn8 * assign2830_e2712) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn8 / (2.0 * assign2830_e2710)) - locals.var_gp02_dn8))) / (assign2830_e2712 * assign2830_e2712)), (((locals.var_gp0_help_dn9 * assign2830_e2712) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn9 / (2.0 * assign2830_e2710)) - locals.var_gp02_dn9))) / (assign2830_e2712 * assign2830_e2712)), (((locals.var_gp0_help_dn10 * assign2830_e2712) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn10 / (2.0 * assign2830_e2710)) - locals.var_gp02_dn10))) / (assign2830_e2712 * assign2830_e2712)),)
    } else {
        (locals.var_p0star, locals.var_p0star_dn0, locals.var_p0star_dn1, locals.var_p0star_dn3, locals.var_p0star_dn4, locals.var_p0star_dn5, locals.var_p0star_dn6, locals.var_p0star_dn7, locals.var_p0star_dn8, locals.var_p0star_dn9, locals.var_p0star_dn10,)
    }
};
        locals.var_p0star = assign2830_e2715;
        locals.var_p0star_dn0 = assign2830_e2715_d_n0;
        locals.var_p0star_dn1 = assign2830_e2715_d_n1;
        locals.var_p0star_dn3 = assign2830_e2715_d_n3;
        locals.var_p0star_dn4 = assign2830_e2715_d_n4;
        locals.var_p0star_dn5 = assign2830_e2715_d_n5;
        locals.var_p0star_dn6 = assign2830_e2715_d_n6;
        locals.var_p0star_dn7 = assign2830_e2715_d_n7;
        locals.var_p0star_dn8 = assign2830_e2715_d_n8;
        locals.var_p0star_dn9 = assign2830_e2715_d_n9;
        locals.var_p0star_dn10 = assign2830_e2715_d_n10;
        locals.var_p0star_rv = 0.0;

        let assign2840_e2718: f64 = if locals.var_p0star < p.p139 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign2840_e2718;
        locals.var_guard45_rv = 0.0;

        let (assign2850_e2724, assign2850_e2724_d_n0, assign2850_e2724_d_n1, assign2850_e2724_d_n3, assign2850_e2724_d_n4, assign2850_e2724_d_n5, assign2850_e2724_d_n6, assign2850_e2724_d_n7, assign2850_e2724_d_n8, assign2850_e2724_d_n9, assign2850_e2724_d_n10,) = {
    if ((locals.var_guard40 != 0.0) && (locals.var_guard45 != 0.0)) {
        (p.p139, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_p0star, locals.var_p0star_dn0, locals.var_p0star_dn1, locals.var_p0star_dn3, locals.var_p0star_dn4, locals.var_p0star_dn5, locals.var_p0star_dn6, locals.var_p0star_dn7, locals.var_p0star_dn8, locals.var_p0star_dn9, locals.var_p0star_dn10,)
    }
};
        locals.var_p0star = assign2850_e2724;
        locals.var_p0star_dn0 = assign2850_e2724_d_n0;
        locals.var_p0star_dn1 = assign2850_e2724_d_n1;
        locals.var_p0star_dn3 = assign2850_e2724_d_n3;
        locals.var_p0star_dn4 = assign2850_e2724_d_n4;
        locals.var_p0star_dn5 = assign2850_e2724_d_n5;
        locals.var_p0star_dn6 = assign2850_e2724_d_n6;
        locals.var_p0star_dn7 = assign2850_e2724_d_n7;
        locals.var_p0star_dn8 = assign2850_e2724_d_n8;
        locals.var_p0star_dn9 = assign2850_e2724_d_n9;
        locals.var_p0star_dn10 = assign2850_e2724_d_n10;
        locals.var_p0star_rv = 0.0;

        let (assign2860_e2737, assign2860_e2737_d_n0, assign2860_e2737_d_n1, assign2860_e2737_d_n3, assign2860_e2737_d_n4, assign2860_e2737_d_n5, assign2860_e2737_d_n6, assign2860_e2737_d_n7, assign2860_e2737_d_n8, assign2860_e2737_d_n9, assign2860_e2737_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2860_e2729: f64 = (locals.var_p0star + 1.0);
        let assign2860_e2730: f64 = (locals.var_p0star * assign2860_e2729);
        let assign2860_e2733: f64 = (locals.var_vdc_t * locals.var_vtinv);
        let assign2860_e2734: f64 = (assign2860_e2733).exp();
        let assign2860_e2735: f64 = (assign2860_e2730 * assign2860_e2734);
        (assign2860_e2735, ((((locals.var_p0star_dn0 * assign2860_e2729) + (locals.var_p0star * locals.var_p0star_dn0)) * assign2860_e2734) + (assign2860_e2730 * (assign2860_e2734 * (locals.var_vdc_t_dn0 * locals.var_vtinv)))), ((((locals.var_p0star_dn1 * assign2860_e2729) + (locals.var_p0star * locals.var_p0star_dn1)) * assign2860_e2734) + (assign2860_e2730 * (assign2860_e2734 * (locals.var_vdc_t_dn1 * locals.var_vtinv)))), ((((locals.var_p0star_dn3 * assign2860_e2729) + (locals.var_p0star * locals.var_p0star_dn3)) * assign2860_e2734) + (assign2860_e2730 * (assign2860_e2734 * ((locals.var_vdc_t_dn3 * locals.var_vtinv) + (locals.var_vdc_t * locals.var_vtinv_dn3))))), ((((locals.var_p0star_dn4 * assign2860_e2729) + (locals.var_p0star * locals.var_p0star_dn4)) * assign2860_e2734) + (assign2860_e2730 * (assign2860_e2734 * (locals.var_vdc_t_dn4 * locals.var_vtinv)))), ((((locals.var_p0star_dn5 * assign2860_e2729) + (locals.var_p0star * locals.var_p0star_dn5)) * assign2860_e2734) + (assign2860_e2730 * (assign2860_e2734 * (locals.var_vdc_t_dn5 * locals.var_vtinv)))), ((((locals.var_p0star_dn6 * assign2860_e2729) + (locals.var_p0star * locals.var_p0star_dn6)) * assign2860_e2734) + (assign2860_e2730 * (assign2860_e2734 * (locals.var_vdc_t_dn6 * locals.var_vtinv)))), ((((locals.var_p0star_dn7 * assign2860_e2729) + (locals.var_p0star * locals.var_p0star_dn7)) * assign2860_e2734) + (assign2860_e2730 * (assign2860_e2734 * (locals.var_vdc_t_dn7 * locals.var_vtinv)))), ((((locals.var_p0star_dn8 * assign2860_e2729) + (locals.var_p0star * locals.var_p0star_dn8)) * assign2860_e2734) + (assign2860_e2730 * (assign2860_e2734 * (locals.var_vdc_t_dn8 * locals.var_vtinv)))), ((((locals.var_p0star_dn9 * assign2860_e2729) + (locals.var_p0star * locals.var_p0star_dn9)) * assign2860_e2734) + (assign2860_e2730 * (assign2860_e2734 * (locals.var_vdc_t_dn9 * locals.var_vtinv)))), ((((locals.var_p0star_dn10 * assign2860_e2729) + (locals.var_p0star * locals.var_p0star_dn10)) * assign2860_e2734) + (assign2860_e2730 * (assign2860_e2734 * (locals.var_vdc_t_dn10 * locals.var_vtinv)))),)
    } else {
        (locals.var_evb2c2star, locals.var_evb2c2star_dn0, locals.var_evb2c2star_dn1, locals.var_evb2c2star_dn3, locals.var_evb2c2star_dn4, locals.var_evb2c2star_dn5, locals.var_evb2c2star_dn6, locals.var_evb2c2star_dn7, locals.var_evb2c2star_dn8, locals.var_evb2c2star_dn9, locals.var_evb2c2star_dn10,)
    }
};
        locals.var_evb2c2star = assign2860_e2737;
        locals.var_evb2c2star_dn0 = assign2860_e2737_d_n0;
        locals.var_evb2c2star_dn1 = assign2860_e2737_d_n1;
        locals.var_evb2c2star_dn3 = assign2860_e2737_d_n3;
        locals.var_evb2c2star_dn4 = assign2860_e2737_d_n4;
        locals.var_evb2c2star_dn5 = assign2860_e2737_d_n5;
        locals.var_evb2c2star_dn6 = assign2860_e2737_d_n6;
        locals.var_evb2c2star_dn7 = assign2860_e2737_d_n7;
        locals.var_evb2c2star_dn8 = assign2860_e2737_d_n8;
        locals.var_evb2c2star_dn9 = assign2860_e2737_d_n9;
        locals.var_evb2c2star_dn10 = assign2860_e2737_d_n10;
        locals.var_evb2c2star_rv = 0.0;

        let (assign2870_e2747, assign2870_e2747_d_n0, assign2870_e2747_d_n1, assign2870_e2747_d_n3, assign2870_e2747_d_n4, assign2870_e2747_d_n5, assign2870_e2747_d_n6, assign2870_e2747_d_n7, assign2870_e2747_d_n8, assign2870_e2747_d_n9, assign2870_e2747_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2870_e2741: f64 = (0.5 * p.p60);
        let assign2870_e2744: f64 = (locals.var_ic1c2 - p.p61);
        let assign2870_e2745: f64 = (assign2870_e2741 * assign2870_e2744);
        (assign2870_e2745, (assign2870_e2741 * locals.var_ic1c2_dn0), (assign2870_e2741 * locals.var_ic1c2_dn1), (assign2870_e2741 * locals.var_ic1c2_dn3), (assign2870_e2741 * locals.var_ic1c2_dn4), (assign2870_e2741 * locals.var_ic1c2_dn5), (assign2870_e2741 * locals.var_ic1c2_dn6), (assign2870_e2741 * locals.var_ic1c2_dn7), (assign2870_e2741 * locals.var_ic1c2_dn8), (assign2870_e2741 * locals.var_ic1c2_dn9), (assign2870_e2741 * locals.var_ic1c2_dn10),)
    } else {
        (locals.var_b1, locals.var_b1_dn0, locals.var_b1_dn1, locals.var_b1_dn3, locals.var_b1_dn4, locals.var_b1_dn5, locals.var_b1_dn6, locals.var_b1_dn7, locals.var_b1_dn8, locals.var_b1_dn9, locals.var_b1_dn10,)
    }
};
        locals.var_b1 = assign2870_e2747;
        locals.var_b1_dn0 = assign2870_e2747_d_n0;
        locals.var_b1_dn1 = assign2870_e2747_d_n1;
        locals.var_b1_dn3 = assign2870_e2747_d_n3;
        locals.var_b1_dn4 = assign2870_e2747_d_n4;
        locals.var_b1_dn5 = assign2870_e2747_d_n5;
        locals.var_b1_dn6 = assign2870_e2747_d_n6;
        locals.var_b1_dn7 = assign2870_e2747_d_n7;
        locals.var_b1_dn8 = assign2870_e2747_d_n8;
        locals.var_b1_dn9 = assign2870_e2747_d_n9;
        locals.var_b1_dn10 = assign2870_e2747_d_n10;
        locals.var_b1_rv = 0.0;

        let (assign2880_e2757, assign2880_e2757_d_n0, assign2880_e2757_d_n1, assign2880_e2757_d_n3, assign2880_e2757_d_n4, assign2880_e2757_d_n5, assign2880_e2757_d_n6, assign2880_e2757_d_n7, assign2880_e2757_d_n8, assign2880_e2757_d_n9, assign2880_e2757_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2880_e2751: f64 = (p.p60 * locals.var_rcv_t);
        let assign2880_e2753: f64 = (assign2880_e2751 * p.p61);
        let assign2880_e2755: f64 = (assign2880_e2753 * locals.var_ic1c2);
        (assign2880_e2755, (assign2880_e2753 * locals.var_ic1c2_dn0), (assign2880_e2753 * locals.var_ic1c2_dn1), ((((p.p60 * locals.var_rcv_t_dn3) * p.p61) * locals.var_ic1c2) + (assign2880_e2753 * locals.var_ic1c2_dn3)), (assign2880_e2753 * locals.var_ic1c2_dn4), (assign2880_e2753 * locals.var_ic1c2_dn5), (assign2880_e2753 * locals.var_ic1c2_dn6), (assign2880_e2753 * locals.var_ic1c2_dn7), (assign2880_e2753 * locals.var_ic1c2_dn8), (assign2880_e2753 * locals.var_ic1c2_dn9), (assign2880_e2753 * locals.var_ic1c2_dn10),)
    } else {
        (locals.var_b2, locals.var_b2_dn0, locals.var_b2_dn1, locals.var_b2_dn3, locals.var_b2_dn4, locals.var_b2_dn5, locals.var_b2_dn6, locals.var_b2_dn7, locals.var_b2_dn8, locals.var_b2_dn9, locals.var_b2_dn10,)
    }
};
        locals.var_b2 = assign2880_e2757;
        locals.var_b2_dn0 = assign2880_e2757_d_n0;
        locals.var_b2_dn1 = assign2880_e2757_d_n1;
        locals.var_b2_dn3 = assign2880_e2757_d_n3;
        locals.var_b2_dn4 = assign2880_e2757_d_n4;
        locals.var_b2_dn5 = assign2880_e2757_d_n5;
        locals.var_b2_dn6 = assign2880_e2757_d_n6;
        locals.var_b2_dn7 = assign2880_e2757_d_n7;
        locals.var_b2_dn8 = assign2880_e2757_d_n8;
        locals.var_b2_dn9 = assign2880_e2757_d_n9;
        locals.var_b2_dn10 = assign2880_e2757_d_n10;
        locals.var_b2_rv = 0.0;

        let (assign2890_e2768, assign2890_e2768_d_n0, assign2890_e2768_d_n1, assign2890_e2768_d_n3, assign2890_e2768_d_n4, assign2890_e2768_d_n5, assign2890_e2768_d_n6, assign2890_e2768_d_n7, assign2890_e2768_d_n8, assign2890_e2768_d_n9, assign2890_e2768_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2890_e2762: f64 = (locals.var_b1 * locals.var_b1);
        let assign2890_e2764: f64 = (assign2890_e2762 + locals.var_b2);
        let assign2890_e2765: f64 = (assign2890_e2764).sqrt();
        let assign2890_e2766: f64 = (locals.var_b1 + assign2890_e2765);
        (assign2890_e2766, (locals.var_b1_dn0 + ((((locals.var_b1_dn0 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn0)) + locals.var_b2_dn0) / (2.0 * assign2890_e2765))), (locals.var_b1_dn1 + ((((locals.var_b1_dn1 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn1)) + locals.var_b2_dn1) / (2.0 * assign2890_e2765))), (locals.var_b1_dn3 + ((((locals.var_b1_dn3 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn3)) + locals.var_b2_dn3) / (2.0 * assign2890_e2765))), (locals.var_b1_dn4 + ((((locals.var_b1_dn4 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn4)) + locals.var_b2_dn4) / (2.0 * assign2890_e2765))), (locals.var_b1_dn5 + ((((locals.var_b1_dn5 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn5)) + locals.var_b2_dn5) / (2.0 * assign2890_e2765))), (locals.var_b1_dn6 + ((((locals.var_b1_dn6 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn6)) + locals.var_b2_dn6) / (2.0 * assign2890_e2765))), (locals.var_b1_dn7 + ((((locals.var_b1_dn7 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn7)) + locals.var_b2_dn7) / (2.0 * assign2890_e2765))), (locals.var_b1_dn8 + ((((locals.var_b1_dn8 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn8)) + locals.var_b2_dn8) / (2.0 * assign2890_e2765))), (locals.var_b1_dn9 + ((((locals.var_b1_dn9 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn9)) + locals.var_b2_dn9) / (2.0 * assign2890_e2765))), (locals.var_b1_dn10 + ((((locals.var_b1_dn10 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn10)) + locals.var_b2_dn10) / (2.0 * assign2890_e2765))),)
    } else {
        (locals.var_vxi0, locals.var_vxi0_dn0, locals.var_vxi0_dn1, locals.var_vxi0_dn3, locals.var_vxi0_dn4, locals.var_vxi0_dn5, locals.var_vxi0_dn6, locals.var_vxi0_dn7, locals.var_vxi0_dn8, locals.var_vxi0_dn9, locals.var_vxi0_dn10,)
    }
};
        locals.var_vxi0 = assign2890_e2768;
        locals.var_vxi0_dn0 = assign2890_e2768_d_n0;
        locals.var_vxi0_dn1 = assign2890_e2768_d_n1;
        locals.var_vxi0_dn3 = assign2890_e2768_d_n3;
        locals.var_vxi0_dn4 = assign2890_e2768_d_n4;
        locals.var_vxi0_dn5 = assign2890_e2768_d_n5;
        locals.var_vxi0_dn6 = assign2890_e2768_d_n6;
        locals.var_vxi0_dn7 = assign2890_e2768_d_n7;
        locals.var_vxi0_dn8 = assign2890_e2768_d_n8;
        locals.var_vxi0_dn9 = assign2890_e2768_d_n9;
        locals.var_vxi0_dn10 = assign2890_e2768_d_n10;
        locals.var_vxi0_rv = 0.0;

        let assign2900_e2771: f64 = if p.p72 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard46 = assign2900_e2771;
        locals.var_guard46_rv = 0.0;

        let (assign2910_e2779, assign2910_e2779_d_n0, assign2910_e2779_d_n1, assign2910_e2779_d_n3, assign2910_e2779_d_n4, assign2910_e2779_d_n5, assign2910_e2779_d_n6, assign2910_e2779_d_n7, assign2910_e2779_d_n8, assign2910_e2779_d_n9, assign2910_e2779_d_n10,) = {
    if ((locals.var_guard40 != 0.0) && (locals.var_guard46 != 0.0)) {
        let assign2910_e2777: f64 = (locals.var_vdc_ctc_t * 0.1);
        (assign2910_e2777, (locals.var_vdc_ctc_t_dn0 * 0.1), (locals.var_vdc_ctc_t_dn1 * 0.1), (locals.var_vdc_ctc_t_dn3 * 0.1), (locals.var_vdc_ctc_t_dn4 * 0.1), (locals.var_vdc_ctc_t_dn5 * 0.1), (locals.var_vdc_ctc_t_dn6 * 0.1), (locals.var_vdc_ctc_t_dn7 * 0.1), (locals.var_vdc_ctc_t_dn8 * 0.1), (locals.var_vdc_ctc_t_dn9 * 0.1), (locals.var_vdc_ctc_t_dn10 * 0.1),)
    } else {
        (locals.var_vch, locals.var_vch_dn0, locals.var_vch_dn1, locals.var_vch_dn3, locals.var_vch_dn4, locals.var_vch_dn5, locals.var_vch_dn6, locals.var_vch_dn7, locals.var_vch_dn8, locals.var_vch_dn9, locals.var_vch_dn10,)
    }
};
        locals.var_vch = assign2910_e2779;
        locals.var_vch_dn0 = assign2910_e2779_d_n0;
        locals.var_vch_dn1 = assign2910_e2779_d_n1;
        locals.var_vch_dn3 = assign2910_e2779_d_n3;
        locals.var_vch_dn4 = assign2910_e2779_d_n4;
        locals.var_vch_dn5 = assign2910_e2779_d_n5;
        locals.var_vch_dn6 = assign2910_e2779_d_n6;
        locals.var_vch_dn7 = assign2910_e2779_d_n7;
        locals.var_vch_dn8 = assign2910_e2779_d_n8;
        locals.var_vch_dn9 = assign2910_e2779_d_n9;
        locals.var_vch_dn10 = assign2910_e2779_d_n10;
        locals.var_vch_rv = 0.0;

        let (assign2920_e2796, assign2920_e2796_d_n0, assign2920_e2796_d_n1, assign2920_e2796_d_n3, assign2920_e2796_d_n4, assign2920_e2796_d_n5, assign2920_e2796_d_n6, assign2920_e2796_d_n7, assign2920_e2796_d_n8, assign2920_e2796_d_n9, assign2920_e2796_d_n10,) = {
    if ((locals.var_guard40 != 0.0) && (locals.var_guard46 == 0.0)) {
        let assign2920_e2788: f64 = (2.0 * locals.var_ic1c2);
        let assign2920_e2791: f64 = (locals.var_ic1c2 + locals.var_iqs);
        let assign2920_e2792: f64 = (assign2920_e2788 / assign2920_e2791);
        let assign2920_e2793: f64 = (0.1 + assign2920_e2792);
        let assign2920_e2794: f64 = (locals.var_vdc_ctc_t * assign2920_e2793);
        (assign2920_e2794, ((locals.var_vdc_ctc_t_dn0 * assign2920_e2793) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn0) * assign2920_e2791) - (assign2920_e2788 * (locals.var_ic1c2_dn0 + locals.var_iqs_dn0))) / (assign2920_e2791 * assign2920_e2791)))), ((locals.var_vdc_ctc_t_dn1 * assign2920_e2793) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn1) * assign2920_e2791) - (assign2920_e2788 * (locals.var_ic1c2_dn1 + locals.var_iqs_dn1))) / (assign2920_e2791 * assign2920_e2791)))), ((locals.var_vdc_ctc_t_dn3 * assign2920_e2793) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn3) * assign2920_e2791) - (assign2920_e2788 * (locals.var_ic1c2_dn3 + locals.var_iqs_dn3))) / (assign2920_e2791 * assign2920_e2791)))), ((locals.var_vdc_ctc_t_dn4 * assign2920_e2793) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn4) * assign2920_e2791) - (assign2920_e2788 * (locals.var_ic1c2_dn4 + locals.var_iqs_dn4))) / (assign2920_e2791 * assign2920_e2791)))), ((locals.var_vdc_ctc_t_dn5 * assign2920_e2793) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn5) * assign2920_e2791) - (assign2920_e2788 * (locals.var_ic1c2_dn5 + locals.var_iqs_dn5))) / (assign2920_e2791 * assign2920_e2791)))), ((locals.var_vdc_ctc_t_dn6 * assign2920_e2793) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn6) * assign2920_e2791) - (assign2920_e2788 * (locals.var_ic1c2_dn6 + locals.var_iqs_dn6))) / (assign2920_e2791 * assign2920_e2791)))), ((locals.var_vdc_ctc_t_dn7 * assign2920_e2793) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn7) * assign2920_e2791) - (assign2920_e2788 * (locals.var_ic1c2_dn7 + locals.var_iqs_dn7))) / (assign2920_e2791 * assign2920_e2791)))), ((locals.var_vdc_ctc_t_dn8 * assign2920_e2793) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn8) * assign2920_e2791) - (assign2920_e2788 * (locals.var_ic1c2_dn8 + locals.var_iqs_dn8))) / (assign2920_e2791 * assign2920_e2791)))), ((locals.var_vdc_ctc_t_dn9 * assign2920_e2793) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn9) * assign2920_e2791) - (assign2920_e2788 * (locals.var_ic1c2_dn9 + locals.var_iqs_dn9))) / (assign2920_e2791 * assign2920_e2791)))), ((locals.var_vdc_ctc_t_dn10 * assign2920_e2793) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn10) * assign2920_e2791) - (assign2920_e2788 * (locals.var_ic1c2_dn10 + locals.var_iqs_dn10))) / (assign2920_e2791 * assign2920_e2791)))),)
    } else {
        (locals.var_vch, locals.var_vch_dn0, locals.var_vch_dn1, locals.var_vch_dn3, locals.var_vch_dn4, locals.var_vch_dn5, locals.var_vch_dn6, locals.var_vch_dn7, locals.var_vch_dn8, locals.var_vch_dn9, locals.var_vch_dn10,)
    }
};
        locals.var_vch = assign2920_e2796;
        locals.var_vch_dn0 = assign2920_e2796_d_n0;
        locals.var_vch_dn1 = assign2920_e2796_d_n1;
        locals.var_vch_dn3 = assign2920_e2796_d_n3;
        locals.var_vch_dn4 = assign2920_e2796_d_n4;
        locals.var_vch_dn5 = assign2920_e2796_d_n5;
        locals.var_vch_dn6 = assign2920_e2796_d_n6;
        locals.var_vch_dn7 = assign2920_e2796_d_n7;
        locals.var_vch_dn8 = assign2920_e2796_d_n8;
        locals.var_vch_dn9 = assign2920_e2796_d_n9;
        locals.var_vch_dn10 = assign2920_e2796_d_n10;
        locals.var_vch_rv = 0.0;

        let (assign2930_e2806, assign2930_e2806_d_n0, assign2930_e2806_d_n1, assign2930_e2806_d_n3, assign2930_e2806_d_n4, assign2930_e2806_d_n5, assign2930_e2806_d_n6, assign2930_e2806_d_n7, assign2930_e2806_d_n8, assign2930_e2806_d_n9, assign2930_e2806_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2930_e2800: f64 = (p.p61 * locals.var_ic1c2);
        let assign2930_e2803: f64 = (p.p61 + locals.var_ic1c2);
        let assign2930_e2804: f64 = (assign2930_e2800 / assign2930_e2803);
        (assign2930_e2804, ((((p.p61 * locals.var_ic1c2_dn0) * assign2930_e2803) - (assign2930_e2800 * locals.var_ic1c2_dn0)) / (assign2930_e2803 * assign2930_e2803)), ((((p.p61 * locals.var_ic1c2_dn1) * assign2930_e2803) - (assign2930_e2800 * locals.var_ic1c2_dn1)) / (assign2930_e2803 * assign2930_e2803)), ((((p.p61 * locals.var_ic1c2_dn3) * assign2930_e2803) - (assign2930_e2800 * locals.var_ic1c2_dn3)) / (assign2930_e2803 * assign2930_e2803)), ((((p.p61 * locals.var_ic1c2_dn4) * assign2930_e2803) - (assign2930_e2800 * locals.var_ic1c2_dn4)) / (assign2930_e2803 * assign2930_e2803)), ((((p.p61 * locals.var_ic1c2_dn5) * assign2930_e2803) - (assign2930_e2800 * locals.var_ic1c2_dn5)) / (assign2930_e2803 * assign2930_e2803)), ((((p.p61 * locals.var_ic1c2_dn6) * assign2930_e2803) - (assign2930_e2800 * locals.var_ic1c2_dn6)) / (assign2930_e2803 * assign2930_e2803)), ((((p.p61 * locals.var_ic1c2_dn7) * assign2930_e2803) - (assign2930_e2800 * locals.var_ic1c2_dn7)) / (assign2930_e2803 * assign2930_e2803)), ((((p.p61 * locals.var_ic1c2_dn8) * assign2930_e2803) - (assign2930_e2800 * locals.var_ic1c2_dn8)) / (assign2930_e2803 * assign2930_e2803)), ((((p.p61 * locals.var_ic1c2_dn9) * assign2930_e2803) - (assign2930_e2800 * locals.var_ic1c2_dn9)) / (assign2930_e2803 * assign2930_e2803)), ((((p.p61 * locals.var_ic1c2_dn10) * assign2930_e2803) - (assign2930_e2800 * locals.var_ic1c2_dn10)) / (assign2930_e2803 * assign2930_e2803)),)
    } else {
        (locals.var_icap, locals.var_icap_dn0, locals.var_icap_dn1, locals.var_icap_dn3, locals.var_icap_dn4, locals.var_icap_dn5, locals.var_icap_dn6, locals.var_icap_dn7, locals.var_icap_dn8, locals.var_icap_dn9, locals.var_icap_dn10,)
    }
};
        locals.var_icap = assign2930_e2806;
        locals.var_icap_dn0 = assign2930_e2806_d_n0;
        locals.var_icap_dn1 = assign2930_e2806_d_n1;
        locals.var_icap_dn3 = assign2930_e2806_d_n3;
        locals.var_icap_dn4 = assign2930_e2806_d_n4;
        locals.var_icap_dn5 = assign2930_e2806_d_n5;
        locals.var_icap_dn6 = assign2930_e2806_d_n6;
        locals.var_icap_dn7 = assign2930_e2806_d_n7;
        locals.var_icap_dn8 = assign2930_e2806_d_n8;
        locals.var_icap_dn9 = assign2930_e2806_d_n9;
        locals.var_icap_dn10 = assign2930_e2806_d_n10;
        locals.var_icap_rv = 0.0;

        let (assign2940_e2814, assign2940_e2814_d_n0, assign2940_e2814_d_n1, assign2940_e2814_d_n3, assign2940_e2814_d_n4, assign2940_e2814_d_n5, assign2940_e2814_d_n6, assign2940_e2814_d_n7, assign2940_e2814_d_n8, assign2940_e2814_d_n9, assign2940_e2814_d_n10,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2940_e2811: f64 = (p.p61 + locals.var_ic1c2);
        let assign2940_e2812: f64 = (p.p61 / assign2940_e2811);
        (assign2940_e2812, (-((p.p61 * locals.var_ic1c2_dn0) / (assign2940_e2811 * assign2940_e2811))), (-((p.p61 * locals.var_ic1c2_dn1) / (assign2940_e2811 * assign2940_e2811))), (-((p.p61 * locals.var_ic1c2_dn3) / (assign2940_e2811 * assign2940_e2811))), (-((p.p61 * locals.var_ic1c2_dn4) / (assign2940_e2811 * assign2940_e2811))), (-((p.p61 * locals.var_ic1c2_dn5) / (assign2940_e2811 * assign2940_e2811))), (-((p.p61 * locals.var_ic1c2_dn6) / (assign2940_e2811 * assign2940_e2811))), (-((p.p61 * locals.var_ic1c2_dn7) / (assign2940_e2811 * assign2940_e2811))), (-((p.p61 * locals.var_ic1c2_dn8) / (assign2940_e2811 * assign2940_e2811))), (-((p.p61 * locals.var_ic1c2_dn9) / (assign2940_e2811 * assign2940_e2811))), (-((p.p61 * locals.var_ic1c2_dn10) / (assign2940_e2811 * assign2940_e2811))),)
    } else {
        (locals.var_icap_ihc, locals.var_icap_ihc_dn0, locals.var_icap_ihc_dn1, locals.var_icap_ihc_dn3, locals.var_icap_ihc_dn4, locals.var_icap_ihc_dn5, locals.var_icap_ihc_dn6, locals.var_icap_ihc_dn7, locals.var_icap_ihc_dn8, locals.var_icap_ihc_dn9, locals.var_icap_ihc_dn10,)
    }
};
        locals.var_icap_ihc = assign2940_e2814;
        locals.var_icap_ihc_dn0 = assign2940_e2814_d_n0;
        locals.var_icap_ihc_dn1 = assign2940_e2814_d_n1;
        locals.var_icap_ihc_dn3 = assign2940_e2814_d_n3;
        locals.var_icap_ihc_dn4 = assign2940_e2814_d_n4;
        locals.var_icap_ihc_dn5 = assign2940_e2814_d_n5;
        locals.var_icap_ihc_dn6 = assign2940_e2814_d_n6;
        locals.var_icap_ihc_dn7 = assign2940_e2814_d_n7;
        locals.var_icap_ihc_dn8 = assign2940_e2814_d_n8;
        locals.var_icap_ihc_dn9 = assign2940_e2814_d_n9;
        locals.var_icap_ihc_dn10 = assign2940_e2814_d_n10;
        locals.var_icap_ihc_rv = 0.0;

        let (assign2950_e2819, assign2950_e2819_d_n0, assign2950_e2819_d_n1, assign2950_e2819_d_n3, assign2950_e2819_d_n4, assign2950_e2819_d_n5, assign2950_e2819_d_n6, assign2950_e2819_d_n7, assign2950_e2819_d_n8, assign2950_e2819_d_n9, assign2950_e2819_d_n10,) = {
    if (locals.var_guard40 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iqs, locals.var_iqs_dn0, locals.var_iqs_dn1, locals.var_iqs_dn3, locals.var_iqs_dn4, locals.var_iqs_dn5, locals.var_iqs_dn6, locals.var_iqs_dn7, locals.var_iqs_dn8, locals.var_iqs_dn9, locals.var_iqs_dn10,)
    }
};
        locals.var_iqs = assign2950_e2819;
        locals.var_iqs_dn0 = assign2950_e2819_d_n0;
        locals.var_iqs_dn1 = assign2950_e2819_d_n1;
        locals.var_iqs_dn3 = assign2950_e2819_d_n3;
        locals.var_iqs_dn4 = assign2950_e2819_d_n4;
        locals.var_iqs_dn5 = assign2950_e2819_d_n5;
        locals.var_iqs_dn6 = assign2950_e2819_d_n6;
        locals.var_iqs_dn7 = assign2950_e2819_d_n7;
        locals.var_iqs_dn8 = assign2950_e2819_d_n8;
        locals.var_iqs_dn9 = assign2950_e2819_d_n9;
        locals.var_iqs_dn10 = assign2950_e2819_d_n10;
        locals.var_iqs_rv = 0.0;

        let (assign2960_e2830, assign2960_e2830_d_n0, assign2960_e2830_d_n1, assign2960_e2830_d_n3, assign2960_e2830_d_n4, assign2960_e2830_d_n5, assign2960_e2830_d_n6, assign2960_e2830_d_n7, assign2960_e2830_d_n8, assign2960_e2830_d_n9, assign2960_e2830_d_n10,) = {
    if (locals.var_guard40 == 0.0) {
        let assign2960_e2824: f64 = (2.0 * locals.var_evb2c2vdc);
        let assign2960_e2827: f64 = (1.0 + locals.var_k0);
        let assign2960_e2828: f64 = (assign2960_e2824 / assign2960_e2827);
        (assign2960_e2828, ((((2.0 * locals.var_evb2c2vdc_dn0) * assign2960_e2827) - (assign2960_e2824 * locals.var_k0_dn0)) / (assign2960_e2827 * assign2960_e2827)), ((((2.0 * locals.var_evb2c2vdc_dn1) * assign2960_e2827) - (assign2960_e2824 * locals.var_k0_dn1)) / (assign2960_e2827 * assign2960_e2827)), ((((2.0 * locals.var_evb2c2vdc_dn3) * assign2960_e2827) - (assign2960_e2824 * locals.var_k0_dn3)) / (assign2960_e2827 * assign2960_e2827)), ((((2.0 * locals.var_evb2c2vdc_dn4) * assign2960_e2827) - (assign2960_e2824 * locals.var_k0_dn4)) / (assign2960_e2827 * assign2960_e2827)), ((((2.0 * locals.var_evb2c2vdc_dn5) * assign2960_e2827) - (assign2960_e2824 * locals.var_k0_dn5)) / (assign2960_e2827 * assign2960_e2827)), ((((2.0 * locals.var_evb2c2vdc_dn6) * assign2960_e2827) - (assign2960_e2824 * locals.var_k0_dn6)) / (assign2960_e2827 * assign2960_e2827)), ((((2.0 * locals.var_evb2c2vdc_dn7) * assign2960_e2827) - (assign2960_e2824 * locals.var_k0_dn7)) / (assign2960_e2827 * assign2960_e2827)), ((((2.0 * locals.var_evb2c2vdc_dn8) * assign2960_e2827) - (assign2960_e2824 * locals.var_k0_dn8)) / (assign2960_e2827 * assign2960_e2827)), ((((2.0 * locals.var_evb2c2vdc_dn9) * assign2960_e2827) - (assign2960_e2824 * locals.var_k0_dn9)) / (assign2960_e2827 * assign2960_e2827)), ((((2.0 * locals.var_evb2c2vdc_dn10) * assign2960_e2827) - (assign2960_e2824 * locals.var_k0_dn10)) / (assign2960_e2827 * assign2960_e2827)),)
    } else {
        (locals.var_p0star, locals.var_p0star_dn0, locals.var_p0star_dn1, locals.var_p0star_dn3, locals.var_p0star_dn4, locals.var_p0star_dn5, locals.var_p0star_dn6, locals.var_p0star_dn7, locals.var_p0star_dn8, locals.var_p0star_dn9, locals.var_p0star_dn10,)
    }
};
        locals.var_p0star = assign2960_e2830;
        locals.var_p0star_dn0 = assign2960_e2830_d_n0;
        locals.var_p0star_dn1 = assign2960_e2830_d_n1;
        locals.var_p0star_dn3 = assign2960_e2830_d_n3;
        locals.var_p0star_dn4 = assign2960_e2830_d_n4;
        locals.var_p0star_dn5 = assign2960_e2830_d_n5;
        locals.var_p0star_dn6 = assign2960_e2830_d_n6;
        locals.var_p0star_dn7 = assign2960_e2830_d_n7;
        locals.var_p0star_dn8 = assign2960_e2830_d_n8;
        locals.var_p0star_dn9 = assign2960_e2830_d_n9;
        locals.var_p0star_dn10 = assign2960_e2830_d_n10;
        locals.var_p0star_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2970_e2835, assign2970_e2835_d_n0, assign2970_e2835_d_n1, assign2970_e2835_d_n3, assign2970_e2835_d_n4, assign2970_e2835_d_n5, assign2970_e2835_d_n6, assign2970_e2835_d_n7, assign2970_e2835_d_n8, assign2970_e2835_d_n9, assign2970_e2835_d_n10,) = {
    if (locals.var_guard40 == 0.0) {
        (locals.var_evb2c2, 0.0, 0.0, locals.var_evb2c2_dn3, 0.0, 0.0, locals.var_evb2c2_dn6, 0.0, locals.var_evb2c2_dn8, 0.0, 0.0,)
    } else {
        (locals.var_evb2c2star, locals.var_evb2c2star_dn0, locals.var_evb2c2star_dn1, locals.var_evb2c2star_dn3, locals.var_evb2c2star_dn4, locals.var_evb2c2star_dn5, locals.var_evb2c2star_dn6, locals.var_evb2c2star_dn7, locals.var_evb2c2star_dn8, locals.var_evb2c2star_dn9, locals.var_evb2c2star_dn10,)
    }
};
        locals.var_evb2c2star = assign2970_e2835;
        locals.var_evb2c2star_dn0 = assign2970_e2835_d_n0;
        locals.var_evb2c2star_dn1 = assign2970_e2835_d_n1;
        locals.var_evb2c2star_dn3 = assign2970_e2835_d_n3;
        locals.var_evb2c2star_dn4 = assign2970_e2835_d_n4;
        locals.var_evb2c2star_dn5 = assign2970_e2835_d_n5;
        locals.var_evb2c2star_dn6 = assign2970_e2835_d_n6;
        locals.var_evb2c2star_dn7 = assign2970_e2835_d_n7;
        locals.var_evb2c2star_dn8 = assign2970_e2835_d_n8;
        locals.var_evb2c2star_dn9 = assign2970_e2835_d_n9;
        locals.var_evb2c2star_dn10 = assign2970_e2835_d_n10;
        locals.var_evb2c2star_rv = 0.0;

        let assign2980_e2837: f64 = (locals.var_vc1c2).abs();
        let assign2980_e2840: f64 = (1e-5 * locals.var_vt);
        let assign2980_e2843: f64 = (locals.var_ec).abs();
        let assign2980_e2846: f64 = (1e-40 * locals.var_vt);
        let assign2980_e2849: f64 = (locals.var_k0 + locals.var_kw);
        let assign2980_e2850: f64 = (assign2980_e2846 * assign2980_e2849);
        let assign2980_e2852: f64 = if ((assign2980_e2837 < assign2980_e2840) || (assign2980_e2843 < assign2980_e2850)) { 1.0 } else { 0.0 };
        locals.var_guard47 = assign2980_e2852;
        locals.var_guard47_rv = 0.0;

        let (assign2990_e2863, assign2990_e2863_d_n0, assign2990_e2863_d_n1, assign2990_e2863_d_n3, assign2990_e2863_d_n4, assign2990_e2863_d_n5, assign2990_e2863_d_n6, assign2990_e2863_d_n7, assign2990_e2863_d_n8, assign2990_e2863_d_n9, assign2990_e2863_d_n10,) = {
    if ((locals.var_guard40 == 0.0) && (locals.var_guard47 != 0.0)) {
        let assign2990_e2860: f64 = (locals.var_p0star + locals.var_pw);
        let assign2990_e2861: f64 = (0.5 * assign2990_e2860);
        (assign2990_e2861, (0.5 * (locals.var_p0star_dn0 + locals.var_pw_dn0)), (0.5 * (locals.var_p0star_dn1 + locals.var_pw_dn1)), (0.5 * (locals.var_p0star_dn3 + locals.var_pw_dn3)), (0.5 * (locals.var_p0star_dn4 + locals.var_pw_dn4)), (0.5 * (locals.var_p0star_dn5 + locals.var_pw_dn5)), (0.5 * (locals.var_p0star_dn6 + locals.var_pw_dn6)), (0.5 * (locals.var_p0star_dn7 + locals.var_pw_dn7)), (0.5 * (locals.var_p0star_dn8 + locals.var_pw_dn8)), (0.5 * (locals.var_p0star_dn9 + locals.var_pw_dn9)), (0.5 * (locals.var_p0star_dn10 + locals.var_pw_dn10)),)
    } else {
        (locals.var_pav, locals.var_pav_dn0, locals.var_pav_dn1, locals.var_pav_dn3, locals.var_pav_dn4, locals.var_pav_dn5, locals.var_pav_dn6, locals.var_pav_dn7, locals.var_pav_dn8, locals.var_pav_dn9, locals.var_pav_dn10,)
    }
};
        locals.var_pav = assign2990_e2863;
        locals.var_pav_dn0 = assign2990_e2863_d_n0;
        locals.var_pav_dn1 = assign2990_e2863_d_n1;
        locals.var_pav_dn3 = assign2990_e2863_d_n3;
        locals.var_pav_dn4 = assign2990_e2863_d_n4;
        locals.var_pav_dn5 = assign2990_e2863_d_n5;
        locals.var_pav_dn6 = assign2990_e2863_d_n6;
        locals.var_pav_dn7 = assign2990_e2863_d_n7;
        locals.var_pav_dn8 = assign2990_e2863_d_n8;
        locals.var_pav_dn9 = assign2990_e2863_d_n9;
        locals.var_pav_dn10 = assign2990_e2863_d_n10;
        locals.var_pav_rv = 0.0;

        let (assign3000_e2874, assign3000_e2874_d_n0, assign3000_e2874_d_n1, assign3000_e2874_d_n3, assign3000_e2874_d_n4, assign3000_e2874_d_n5, assign3000_e2874_d_n6, assign3000_e2874_d_n7, assign3000_e2874_d_n8, assign3000_e2874_d_n9, assign3000_e2874_d_n10,) = {
    if ((locals.var_guard40 == 0.0) && (locals.var_guard47 != 0.0)) {
        let assign3000_e2871: f64 = (locals.var_pav + 1.0);
        let assign3000_e2872: f64 = (locals.var_pav / assign3000_e2871);
        (assign3000_e2872, (((locals.var_pav_dn0 * assign3000_e2871) - (locals.var_pav * locals.var_pav_dn0)) / (assign3000_e2871 * assign3000_e2871)), (((locals.var_pav_dn1 * assign3000_e2871) - (locals.var_pav * locals.var_pav_dn1)) / (assign3000_e2871 * assign3000_e2871)), (((locals.var_pav_dn3 * assign3000_e2871) - (locals.var_pav * locals.var_pav_dn3)) / (assign3000_e2871 * assign3000_e2871)), (((locals.var_pav_dn4 * assign3000_e2871) - (locals.var_pav * locals.var_pav_dn4)) / (assign3000_e2871 * assign3000_e2871)), (((locals.var_pav_dn5 * assign3000_e2871) - (locals.var_pav * locals.var_pav_dn5)) / (assign3000_e2871 * assign3000_e2871)), (((locals.var_pav_dn6 * assign3000_e2871) - (locals.var_pav * locals.var_pav_dn6)) / (assign3000_e2871 * assign3000_e2871)), (((locals.var_pav_dn7 * assign3000_e2871) - (locals.var_pav * locals.var_pav_dn7)) / (assign3000_e2871 * assign3000_e2871)), (((locals.var_pav_dn8 * assign3000_e2871) - (locals.var_pav * locals.var_pav_dn8)) / (assign3000_e2871 * assign3000_e2871)), (((locals.var_pav_dn9 * assign3000_e2871) - (locals.var_pav * locals.var_pav_dn9)) / (assign3000_e2871 * assign3000_e2871)), (((locals.var_pav_dn10 * assign3000_e2871) - (locals.var_pav * locals.var_pav_dn10)) / (assign3000_e2871 * assign3000_e2871)),)
    } else {
        (locals.var_xi_w, locals.var_xi_w_dn0, locals.var_xi_w_dn1, locals.var_xi_w_dn3, locals.var_xi_w_dn4, locals.var_xi_w_dn5, locals.var_xi_w_dn6, locals.var_xi_w_dn7, locals.var_xi_w_dn8, locals.var_xi_w_dn9, locals.var_xi_w_dn10,)
    }
};
        locals.var_xi_w = assign3000_e2874;
        locals.var_xi_w_dn0 = assign3000_e2874_d_n0;
        locals.var_xi_w_dn1 = assign3000_e2874_d_n1;
        locals.var_xi_w_dn3 = assign3000_e2874_d_n3;
        locals.var_xi_w_dn4 = assign3000_e2874_d_n4;
        locals.var_xi_w_dn5 = assign3000_e2874_d_n5;
        locals.var_xi_w_dn6 = assign3000_e2874_d_n6;
        locals.var_xi_w_dn7 = assign3000_e2874_d_n7;
        locals.var_xi_w_dn8 = assign3000_e2874_d_n8;
        locals.var_xi_w_dn9 = assign3000_e2874_d_n9;
        locals.var_xi_w_dn10 = assign3000_e2874_d_n10;
        locals.var_xi_w_rv = 0.0;

        let (assign3010_e2888, assign3010_e2888_d_n0, assign3010_e2888_d_n1, assign3010_e2888_d_n3, assign3010_e2888_d_n4, assign3010_e2888_d_n5, assign3010_e2888_d_n6, assign3010_e2888_d_n7, assign3010_e2888_d_n8, assign3010_e2888_d_n9, assign3010_e2888_d_n10,) = {
    if ((locals.var_guard40 == 0.0) && (locals.var_guard47 == 0.0)) {
        let assign3010_e2883: f64 = (locals.var_ec + locals.var_vb2c2);
        let assign3010_e2885: f64 = (assign3010_e2883 - locals.var_vb2c1);
        let assign3010_e2886: f64 = (locals.var_ec / assign3010_e2885);
        (assign3010_e2886, (((locals.var_ec_dn0 * assign3010_e2885) - (locals.var_ec * locals.var_ec_dn0)) / (assign3010_e2885 * assign3010_e2885)), (((locals.var_ec_dn1 * assign3010_e2885) - (locals.var_ec * locals.var_ec_dn1)) / (assign3010_e2885 * assign3010_e2885)), (((locals.var_ec_dn3 * assign3010_e2885) - (locals.var_ec * locals.var_ec_dn3)) / (assign3010_e2885 * assign3010_e2885)), (((locals.var_ec_dn4 * assign3010_e2885) - (locals.var_ec * locals.var_ec_dn4)) / (assign3010_e2885 * assign3010_e2885)), (((locals.var_ec_dn5 * assign3010_e2885) - (locals.var_ec * locals.var_ec_dn5)) / (assign3010_e2885 * assign3010_e2885)), (((locals.var_ec_dn6 * assign3010_e2885) - (locals.var_ec * ((locals.var_ec_dn6 + locals.var_vb2c2_dn6) - locals.var_vb2c1_dn6))) / (assign3010_e2885 * assign3010_e2885)), (((locals.var_ec_dn7 * assign3010_e2885) - (locals.var_ec * (locals.var_ec_dn7 - locals.var_vb2c1_dn7))) / (assign3010_e2885 * assign3010_e2885)), (((locals.var_ec_dn8 * assign3010_e2885) - (locals.var_ec * (locals.var_ec_dn8 + locals.var_vb2c2_dn8))) / (assign3010_e2885 * assign3010_e2885)), (((locals.var_ec_dn9 * assign3010_e2885) - (locals.var_ec * locals.var_ec_dn9)) / (assign3010_e2885 * assign3010_e2885)), (((locals.var_ec_dn10 * assign3010_e2885) - (locals.var_ec * locals.var_ec_dn10)) / (assign3010_e2885 * assign3010_e2885)),)
    } else {
        (locals.var_xi_w, locals.var_xi_w_dn0, locals.var_xi_w_dn1, locals.var_xi_w_dn3, locals.var_xi_w_dn4, locals.var_xi_w_dn5, locals.var_xi_w_dn6, locals.var_xi_w_dn7, locals.var_xi_w_dn8, locals.var_xi_w_dn9, locals.var_xi_w_dn10,)
    }
};
        locals.var_xi_w = assign3010_e2888;
        locals.var_xi_w_dn0 = assign3010_e2888_d_n0;
        locals.var_xi_w_dn1 = assign3010_e2888_d_n1;
        locals.var_xi_w_dn3 = assign3010_e2888_d_n3;
        locals.var_xi_w_dn4 = assign3010_e2888_d_n4;
        locals.var_xi_w_dn5 = assign3010_e2888_d_n5;
        locals.var_xi_w_dn6 = assign3010_e2888_d_n6;
        locals.var_xi_w_dn7 = assign3010_e2888_d_n7;
        locals.var_xi_w_dn8 = assign3010_e2888_d_n8;
        locals.var_xi_w_dn9 = assign3010_e2888_d_n9;
        locals.var_xi_w_dn10 = assign3010_e2888_d_n10;
        locals.var_xi_w_rv = 0.0;

        let (assign3020_e2893, assign3020_e2893_d_n0, assign3020_e2893_d_n1, assign3020_e2893_d_n3, assign3020_e2893_d_n4, assign3020_e2893_d_n5, assign3020_e2893_d_n6, assign3020_e2893_d_n7, assign3020_e2893_d_n8, assign3020_e2893_d_n9, assign3020_e2893_d_n10,) = {
    if (locals.var_guard40 == 0.0) {
        (locals.var_vc1c2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_vc1c2_dn7, locals.var_vc1c2_dn8, 0.0, 0.0,)
    } else {
        (locals.var_vxi0, locals.var_vxi0_dn0, locals.var_vxi0_dn1, locals.var_vxi0_dn3, locals.var_vxi0_dn4, locals.var_vxi0_dn5, locals.var_vxi0_dn6, locals.var_vxi0_dn7, locals.var_vxi0_dn8, locals.var_vxi0_dn9, locals.var_vxi0_dn10,)
    }
};
        locals.var_vxi0 = assign3020_e2893;
        locals.var_vxi0_dn0 = assign3020_e2893_d_n0;
        locals.var_vxi0_dn1 = assign3020_e2893_d_n1;
        locals.var_vxi0_dn3 = assign3020_e2893_d_n3;
        locals.var_vxi0_dn4 = assign3020_e2893_d_n4;
        locals.var_vxi0_dn5 = assign3020_e2893_d_n5;
        locals.var_vxi0_dn6 = assign3020_e2893_d_n6;
        locals.var_vxi0_dn7 = assign3020_e2893_d_n7;
        locals.var_vxi0_dn8 = assign3020_e2893_d_n8;
        locals.var_vxi0_dn9 = assign3020_e2893_d_n9;
        locals.var_vxi0_dn10 = assign3020_e2893_d_n10;
        locals.var_vxi0_rv = 0.0;

        let (assign3030_e2900, assign3030_e2900_d_n0, assign3030_e2900_d_n1, assign3030_e2900_d_n3, assign3030_e2900_d_n4, assign3030_e2900_d_n5, assign3030_e2900_d_n6, assign3030_e2900_d_n7, assign3030_e2900_d_n8, assign3030_e2900_d_n9, assign3030_e2900_d_n10,) = {
    if (locals.var_guard40 == 0.0) {
        let assign3030_e2898: f64 = (0.1 * locals.var_vdc_ctc_t);
        (assign3030_e2898, (0.1 * locals.var_vdc_ctc_t_dn0), (0.1 * locals.var_vdc_ctc_t_dn1), (0.1 * locals.var_vdc_ctc_t_dn3), (0.1 * locals.var_vdc_ctc_t_dn4), (0.1 * locals.var_vdc_ctc_t_dn5), (0.1 * locals.var_vdc_ctc_t_dn6), (0.1 * locals.var_vdc_ctc_t_dn7), (0.1 * locals.var_vdc_ctc_t_dn8), (0.1 * locals.var_vdc_ctc_t_dn9), (0.1 * locals.var_vdc_ctc_t_dn10),)
    } else {
        (locals.var_vch, locals.var_vch_dn0, locals.var_vch_dn1, locals.var_vch_dn3, locals.var_vch_dn4, locals.var_vch_dn5, locals.var_vch_dn6, locals.var_vch_dn7, locals.var_vch_dn8, locals.var_vch_dn9, locals.var_vch_dn10,)
    }
};
        locals.var_vch = assign3030_e2900;
        locals.var_vch_dn0 = assign3030_e2900_d_n0;
        locals.var_vch_dn1 = assign3030_e2900_d_n1;
        locals.var_vch_dn3 = assign3030_e2900_d_n3;
        locals.var_vch_dn4 = assign3030_e2900_d_n4;
        locals.var_vch_dn5 = assign3030_e2900_d_n5;
        locals.var_vch_dn6 = assign3030_e2900_d_n6;
        locals.var_vch_dn7 = assign3030_e2900_d_n7;
        locals.var_vch_dn8 = assign3030_e2900_d_n8;
        locals.var_vch_dn9 = assign3030_e2900_d_n9;
        locals.var_vch_dn10 = assign3030_e2900_d_n10;
        locals.var_vch_rv = 0.0;

        let (assign3040_e2905, assign3040_e2905_d_n0, assign3040_e2905_d_n1, assign3040_e2905_d_n3, assign3040_e2905_d_n4, assign3040_e2905_d_n5, assign3040_e2905_d_n6, assign3040_e2905_d_n7, assign3040_e2905_d_n8, assign3040_e2905_d_n9, assign3040_e2905_d_n10,) = {
    if (locals.var_guard40 == 0.0) {
        (locals.var_ic1c2, locals.var_ic1c2_dn0, locals.var_ic1c2_dn1, locals.var_ic1c2_dn3, locals.var_ic1c2_dn4, locals.var_ic1c2_dn5, locals.var_ic1c2_dn6, locals.var_ic1c2_dn7, locals.var_ic1c2_dn8, locals.var_ic1c2_dn9, locals.var_ic1c2_dn10,)
    } else {
        (locals.var_icap, locals.var_icap_dn0, locals.var_icap_dn1, locals.var_icap_dn3, locals.var_icap_dn4, locals.var_icap_dn5, locals.var_icap_dn6, locals.var_icap_dn7, locals.var_icap_dn8, locals.var_icap_dn9, locals.var_icap_dn10,)
    }
};
        locals.var_icap = assign3040_e2905;
        locals.var_icap_dn0 = assign3040_e2905_d_n0;
        locals.var_icap_dn1 = assign3040_e2905_d_n1;
        locals.var_icap_dn3 = assign3040_e2905_d_n3;
        locals.var_icap_dn4 = assign3040_e2905_d_n4;
        locals.var_icap_dn5 = assign3040_e2905_d_n5;
        locals.var_icap_dn6 = assign3040_e2905_d_n6;
        locals.var_icap_dn7 = assign3040_e2905_d_n7;
        locals.var_icap_dn8 = assign3040_e2905_d_n8;
        locals.var_icap_dn9 = assign3040_e2905_d_n9;
        locals.var_icap_dn10 = assign3040_e2905_d_n10;
        locals.var_icap_rv = 0.0;

        let (assign3050_e2914, assign3050_e2914_d_n0, assign3050_e2914_d_n1, assign3050_e2914_d_n3, assign3050_e2914_d_n4, assign3050_e2914_d_n5, assign3050_e2914_d_n6, assign3050_e2914_d_n7, assign3050_e2914_d_n8, assign3050_e2914_d_n9, assign3050_e2914_d_n10,) = {
    if (locals.var_guard40 == 0.0) {
        let assign3050_e2911: f64 = (locals.var_icap / p.p61);
        let assign3050_e2912: f64 = (1.0 - assign3050_e2911);
        (assign3050_e2912, (-(locals.var_icap_dn0 / p.p61)), (-(locals.var_icap_dn1 / p.p61)), (-(locals.var_icap_dn3 / p.p61)), (-(locals.var_icap_dn4 / p.p61)), (-(locals.var_icap_dn5 / p.p61)), (-(locals.var_icap_dn6 / p.p61)), (-(locals.var_icap_dn7 / p.p61)), (-(locals.var_icap_dn8 / p.p61)), (-(locals.var_icap_dn9 / p.p61)), (-(locals.var_icap_dn10 / p.p61)),)
    } else {
        (locals.var_icap_ihc, locals.var_icap_ihc_dn0, locals.var_icap_ihc_dn1, locals.var_icap_ihc_dn3, locals.var_icap_ihc_dn4, locals.var_icap_ihc_dn5, locals.var_icap_ihc_dn6, locals.var_icap_ihc_dn7, locals.var_icap_ihc_dn8, locals.var_icap_ihc_dn9, locals.var_icap_ihc_dn10,)
    }
};
        locals.var_icap_ihc = assign3050_e2914;
        locals.var_icap_ihc_dn0 = assign3050_e2914_d_n0;
        locals.var_icap_ihc_dn1 = assign3050_e2914_d_n1;
        locals.var_icap_ihc_dn3 = assign3050_e2914_d_n3;
        locals.var_icap_ihc_dn4 = assign3050_e2914_d_n4;
        locals.var_icap_ihc_dn5 = assign3050_e2914_d_n5;
        locals.var_icap_ihc_dn6 = assign3050_e2914_d_n6;
        locals.var_icap_ihc_dn7 = assign3050_e2914_d_n7;
        locals.var_icap_ihc_dn8 = assign3050_e2914_d_n8;
        locals.var_icap_ihc_dn9 = assign3050_e2914_d_n9;
        locals.var_icap_ihc_dn10 = assign3050_e2914_d_n10;
        locals.var_icap_ihc_rv = 0.0;

        let assign3060_e2919: f64 = (-1.0);
        let assign3060_e2921: f64 = (assign3060_e2919 / p.p66);
        let assign3060_e2922: f64 = (3.0_f64).powf(assign3060_e2921);
        let assign3060_e2923: f64 = (1.0 - assign3060_e2922);
        let assign3060_e2924: f64 = (locals.var_vde_t * assign3060_e2923);
        locals.var_vfe = assign3060_e2924;
        locals.var_vfe_dn0 = (locals.var_vde_t_dn0 * assign3060_e2923);
        locals.var_vfe_dn1 = (locals.var_vde_t_dn1 * assign3060_e2923);
        locals.var_vfe_dn3 = (locals.var_vde_t_dn3 * assign3060_e2923);
        locals.var_vfe_dn4 = (locals.var_vde_t_dn4 * assign3060_e2923);
        locals.var_vfe_dn5 = (locals.var_vde_t_dn5 * assign3060_e2923);
        locals.var_vfe_dn6 = (locals.var_vde_t_dn6 * assign3060_e2923);
        locals.var_vfe_dn7 = (locals.var_vde_t_dn7 * assign3060_e2923);
        locals.var_vfe_dn8 = (locals.var_vde_t_dn8 * assign3060_e2923);
        locals.var_vfe_dn9 = (locals.var_vde_t_dn9 * assign3060_e2923);
        locals.var_vfe_dn10 = (locals.var_vde_t_dn10 * assign3060_e2923);
        locals.var_vfe_rv = 0.0;

        let assign3070_e2927: f64 = (0.1 * locals.var_vde_t);
        locals.var_a_vde = assign3070_e2927;
        locals.var_a_vde_dn0 = (0.1 * locals.var_vde_t_dn0);
        locals.var_a_vde_dn1 = (0.1 * locals.var_vde_t_dn1);
        locals.var_a_vde_dn3 = (0.1 * locals.var_vde_t_dn3);
        locals.var_a_vde_dn4 = (0.1 * locals.var_vde_t_dn4);
        locals.var_a_vde_dn5 = (0.1 * locals.var_vde_t_dn5);
        locals.var_a_vde_dn6 = (0.1 * locals.var_vde_t_dn6);
        locals.var_a_vde_dn7 = (0.1 * locals.var_vde_t_dn7);
        locals.var_a_vde_dn8 = (0.1 * locals.var_vde_t_dn8);
        locals.var_a_vde_dn9 = (0.1 * locals.var_vde_t_dn9);
        locals.var_a_vde_dn10 = (0.1 * locals.var_vde_t_dn10);
        locals.var_a_vde_rv = 0.0;

        let assign3080_e2930: f64 = (locals.var_vb2e1 - locals.var_vfe);
        let assign3080_e2932: f64 = (assign3080_e2930 / locals.var_a_vde);
        locals.var_dxa = assign3080_e2932;
        locals.var_dxa_dn0 = ((((-locals.var_vfe_dn0) * locals.var_a_vde) - (assign3080_e2930 * locals.var_a_vde_dn0)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn1 = ((((-locals.var_vfe_dn1) * locals.var_a_vde) - (assign3080_e2930 * locals.var_a_vde_dn1)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn3 = ((((-locals.var_vfe_dn3) * locals.var_a_vde) - (assign3080_e2930 * locals.var_a_vde_dn3)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn4 = ((((locals.var_vb2e1_dn4 - locals.var_vfe_dn4) * locals.var_a_vde) - (assign3080_e2930 * locals.var_a_vde_dn4)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn5 = ((((-locals.var_vfe_dn5) * locals.var_a_vde) - (assign3080_e2930 * locals.var_a_vde_dn5)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn6 = ((((locals.var_vb2e1_dn6 - locals.var_vfe_dn6) * locals.var_a_vde) - (assign3080_e2930 * locals.var_a_vde_dn6)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn7 = ((((-locals.var_vfe_dn7) * locals.var_a_vde) - (assign3080_e2930 * locals.var_a_vde_dn7)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn8 = ((((-locals.var_vfe_dn8) * locals.var_a_vde) - (assign3080_e2930 * locals.var_a_vde_dn8)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn9 = ((((-locals.var_vfe_dn9) * locals.var_a_vde) - (assign3080_e2930 * locals.var_a_vde_dn9)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn10 = ((((-locals.var_vfe_dn10) * locals.var_a_vde) - (assign3080_e2930 * locals.var_a_vde_dn10)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_rv = 0.0;

        let assign3090_e2935: f64 = if locals.var_vb2e1 < locals.var_vfe { 1.0 } else { 0.0 };
        locals.var_guard48 = assign3090_e2935;
        locals.var_guard48_rv = 0.0;

        let (assign3100_e2947, assign3100_e2947_d_n0, assign3100_e2947_d_n1, assign3100_e2947_d_n3, assign3100_e2947_d_n4, assign3100_e2947_d_n5, assign3100_e2947_d_n6, assign3100_e2947_d_n7, assign3100_e2947_d_n8, assign3100_e2947_d_n9, assign3100_e2947_d_n10,) = {
    if (locals.var_guard48 != 0.0) {
        let assign3100_e2941: f64 = (locals.var_dxa).exp();
        let assign3100_e2942: f64 = (1.0 + assign3100_e2941);
        let assign3100_e2943: f64 = (assign3100_e2942).ln();
        let assign3100_e2944: f64 = (locals.var_a_vde * assign3100_e2943);
        let assign3100_e2945: f64 = (locals.var_vb2e1 - assign3100_e2944);
        (assign3100_e2945, (-((locals.var_a_vde_dn0 * assign3100_e2943) + (locals.var_a_vde * ((assign3100_e2941 * locals.var_dxa_dn0) / assign3100_e2942)))), (-((locals.var_a_vde_dn1 * assign3100_e2943) + (locals.var_a_vde * ((assign3100_e2941 * locals.var_dxa_dn1) / assign3100_e2942)))), (-((locals.var_a_vde_dn3 * assign3100_e2943) + (locals.var_a_vde * ((assign3100_e2941 * locals.var_dxa_dn3) / assign3100_e2942)))), (locals.var_vb2e1_dn4 - ((locals.var_a_vde_dn4 * assign3100_e2943) + (locals.var_a_vde * ((assign3100_e2941 * locals.var_dxa_dn4) / assign3100_e2942)))), (-((locals.var_a_vde_dn5 * assign3100_e2943) + (locals.var_a_vde * ((assign3100_e2941 * locals.var_dxa_dn5) / assign3100_e2942)))), (locals.var_vb2e1_dn6 - ((locals.var_a_vde_dn6 * assign3100_e2943) + (locals.var_a_vde * ((assign3100_e2941 * locals.var_dxa_dn6) / assign3100_e2942)))), (-((locals.var_a_vde_dn7 * assign3100_e2943) + (locals.var_a_vde * ((assign3100_e2941 * locals.var_dxa_dn7) / assign3100_e2942)))), (-((locals.var_a_vde_dn8 * assign3100_e2943) + (locals.var_a_vde * ((assign3100_e2941 * locals.var_dxa_dn8) / assign3100_e2942)))), (-((locals.var_a_vde_dn9 * assign3100_e2943) + (locals.var_a_vde * ((assign3100_e2941 * locals.var_dxa_dn9) / assign3100_e2942)))), (-((locals.var_a_vde_dn10 * assign3100_e2943) + (locals.var_a_vde * ((assign3100_e2941 * locals.var_dxa_dn10) / assign3100_e2942)))),)
    } else {
        (locals.var_vje, locals.var_vje_dn0, locals.var_vje_dn1, locals.var_vje_dn3, locals.var_vje_dn4, locals.var_vje_dn5, locals.var_vje_dn6, locals.var_vje_dn7, locals.var_vje_dn8, locals.var_vje_dn9, locals.var_vje_dn10,)
    }
};
        locals.var_vje = assign3100_e2947;
        locals.var_vje_dn0 = assign3100_e2947_d_n0;
        locals.var_vje_dn1 = assign3100_e2947_d_n1;
        locals.var_vje_dn3 = assign3100_e2947_d_n3;
        locals.var_vje_dn4 = assign3100_e2947_d_n4;
        locals.var_vje_dn5 = assign3100_e2947_d_n5;
        locals.var_vje_dn6 = assign3100_e2947_d_n6;
        locals.var_vje_dn7 = assign3100_e2947_d_n7;
        locals.var_vje_dn8 = assign3100_e2947_d_n8;
        locals.var_vje_dn9 = assign3100_e2947_d_n9;
        locals.var_vje_dn10 = assign3100_e2947_d_n10;
        locals.var_vje_rv = 0.0;

        let (assign3110_e2961, assign3110_e2961_d_n0, assign3110_e2961_d_n1, assign3110_e2961_d_n3, assign3110_e2961_d_n4, assign3110_e2961_d_n5, assign3110_e2961_d_n6, assign3110_e2961_d_n7, assign3110_e2961_d_n8, assign3110_e2961_d_n9, assign3110_e2961_d_n10,) = {
    if (locals.var_guard48 == 0.0) {
        let assign3110_e2954: f64 = (-locals.var_dxa);
        let assign3110_e2955: f64 = (assign3110_e2954).exp();
        let assign3110_e2956: f64 = (1.0 + assign3110_e2955);
        let assign3110_e2957: f64 = (assign3110_e2956).ln();
        let assign3110_e2958: f64 = (locals.var_a_vde * assign3110_e2957);
        let assign3110_e2959: f64 = (locals.var_vfe - assign3110_e2958);
        (assign3110_e2959, (locals.var_vfe_dn0 - ((locals.var_a_vde_dn0 * assign3110_e2957) + (locals.var_a_vde * ((assign3110_e2955 * (-locals.var_dxa_dn0)) / assign3110_e2956)))), (locals.var_vfe_dn1 - ((locals.var_a_vde_dn1 * assign3110_e2957) + (locals.var_a_vde * ((assign3110_e2955 * (-locals.var_dxa_dn1)) / assign3110_e2956)))), (locals.var_vfe_dn3 - ((locals.var_a_vde_dn3 * assign3110_e2957) + (locals.var_a_vde * ((assign3110_e2955 * (-locals.var_dxa_dn3)) / assign3110_e2956)))), (locals.var_vfe_dn4 - ((locals.var_a_vde_dn4 * assign3110_e2957) + (locals.var_a_vde * ((assign3110_e2955 * (-locals.var_dxa_dn4)) / assign3110_e2956)))), (locals.var_vfe_dn5 - ((locals.var_a_vde_dn5 * assign3110_e2957) + (locals.var_a_vde * ((assign3110_e2955 * (-locals.var_dxa_dn5)) / assign3110_e2956)))), (locals.var_vfe_dn6 - ((locals.var_a_vde_dn6 * assign3110_e2957) + (locals.var_a_vde * ((assign3110_e2955 * (-locals.var_dxa_dn6)) / assign3110_e2956)))), (locals.var_vfe_dn7 - ((locals.var_a_vde_dn7 * assign3110_e2957) + (locals.var_a_vde * ((assign3110_e2955 * (-locals.var_dxa_dn7)) / assign3110_e2956)))), (locals.var_vfe_dn8 - ((locals.var_a_vde_dn8 * assign3110_e2957) + (locals.var_a_vde * ((assign3110_e2955 * (-locals.var_dxa_dn8)) / assign3110_e2956)))), (locals.var_vfe_dn9 - ((locals.var_a_vde_dn9 * assign3110_e2957) + (locals.var_a_vde * ((assign3110_e2955 * (-locals.var_dxa_dn9)) / assign3110_e2956)))), (locals.var_vfe_dn10 - ((locals.var_a_vde_dn10 * assign3110_e2957) + (locals.var_a_vde * ((assign3110_e2955 * (-locals.var_dxa_dn10)) / assign3110_e2956)))),)
    } else {
        (locals.var_vje, locals.var_vje_dn0, locals.var_vje_dn1, locals.var_vje_dn3, locals.var_vje_dn4, locals.var_vje_dn5, locals.var_vje_dn6, locals.var_vje_dn7, locals.var_vje_dn8, locals.var_vje_dn9, locals.var_vje_dn10,)
    }
};
        locals.var_vje = assign3110_e2961;
        locals.var_vje_dn0 = assign3110_e2961_d_n0;
        locals.var_vje_dn1 = assign3110_e2961_d_n1;
        locals.var_vje_dn3 = assign3110_e2961_d_n3;
        locals.var_vje_dn4 = assign3110_e2961_d_n4;
        locals.var_vje_dn5 = assign3110_e2961_d_n5;
        locals.var_vje_dn6 = assign3110_e2961_d_n6;
        locals.var_vje_dn7 = assign3110_e2961_d_n7;
        locals.var_vje_dn8 = assign3110_e2961_d_n8;
        locals.var_vje_dn9 = assign3110_e2961_d_n9;
        locals.var_vje_dn10 = assign3110_e2961_d_n10;
        locals.var_vje_rv = 0.0;

        let assign3120_e2965: f64 = (locals.var_vje * locals.var_inv_vde_t);
        let assign3120_e2966: f64 = (1.0 - assign3120_e2965);
        let assign3120_e2969: f64 = (1.0 - p.p66);
        let assign3120_e2970: f64 = (assign3120_e2966).powf(assign3120_e2969);
        locals.var_e0eb = assign3120_e2970;
        locals.var_e0eb_dn0 = if 0.0 == 0.0 && ((assign3120_e2969) as f64).is_finite() && ((assign3120_e2969) as f64).fract() == 0.0 { if assign3120_e2969 == 0.0 { 0.0 } else { (assign3120_e2969 * ((assign3120_e2966).powf(assign3120_e2969 - 1.0) * (-((locals.var_vje_dn0 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn0))))) } } else { (assign3120_e2970 * (assign3120_e2969 * ((-((locals.var_vje_dn0 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn0))) / assign3120_e2966))) };
        locals.var_e0eb_dn1 = if 0.0 == 0.0 && ((assign3120_e2969) as f64).is_finite() && ((assign3120_e2969) as f64).fract() == 0.0 { if assign3120_e2969 == 0.0 { 0.0 } else { (assign3120_e2969 * ((assign3120_e2966).powf(assign3120_e2969 - 1.0) * (-((locals.var_vje_dn1 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn1))))) } } else { (assign3120_e2970 * (assign3120_e2969 * ((-((locals.var_vje_dn1 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn1))) / assign3120_e2966))) };
        locals.var_e0eb_dn3 = if 0.0 == 0.0 && ((assign3120_e2969) as f64).is_finite() && ((assign3120_e2969) as f64).fract() == 0.0 { if assign3120_e2969 == 0.0 { 0.0 } else { (assign3120_e2969 * ((assign3120_e2966).powf(assign3120_e2969 - 1.0) * (-((locals.var_vje_dn3 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn3))))) } } else { (assign3120_e2970 * (assign3120_e2969 * ((-((locals.var_vje_dn3 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn3))) / assign3120_e2966))) };
        locals.var_e0eb_dn4 = if 0.0 == 0.0 && ((assign3120_e2969) as f64).is_finite() && ((assign3120_e2969) as f64).fract() == 0.0 { if assign3120_e2969 == 0.0 { 0.0 } else { (assign3120_e2969 * ((assign3120_e2966).powf(assign3120_e2969 - 1.0) * (-((locals.var_vje_dn4 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn4))))) } } else { (assign3120_e2970 * (assign3120_e2969 * ((-((locals.var_vje_dn4 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn4))) / assign3120_e2966))) };
        locals.var_e0eb_dn5 = if 0.0 == 0.0 && ((assign3120_e2969) as f64).is_finite() && ((assign3120_e2969) as f64).fract() == 0.0 { if assign3120_e2969 == 0.0 { 0.0 } else { (assign3120_e2969 * ((assign3120_e2966).powf(assign3120_e2969 - 1.0) * (-((locals.var_vje_dn5 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn5))))) } } else { (assign3120_e2970 * (assign3120_e2969 * ((-((locals.var_vje_dn5 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn5))) / assign3120_e2966))) };
        locals.var_e0eb_dn6 = if 0.0 == 0.0 && ((assign3120_e2969) as f64).is_finite() && ((assign3120_e2969) as f64).fract() == 0.0 { if assign3120_e2969 == 0.0 { 0.0 } else { (assign3120_e2969 * ((assign3120_e2966).powf(assign3120_e2969 - 1.0) * (-((locals.var_vje_dn6 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn6))))) } } else { (assign3120_e2970 * (assign3120_e2969 * ((-((locals.var_vje_dn6 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn6))) / assign3120_e2966))) };
        locals.var_e0eb_dn7 = if 0.0 == 0.0 && ((assign3120_e2969) as f64).is_finite() && ((assign3120_e2969) as f64).fract() == 0.0 { if assign3120_e2969 == 0.0 { 0.0 } else { (assign3120_e2969 * ((assign3120_e2966).powf(assign3120_e2969 - 1.0) * (-((locals.var_vje_dn7 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn7))))) } } else { (assign3120_e2970 * (assign3120_e2969 * ((-((locals.var_vje_dn7 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn7))) / assign3120_e2966))) };
        locals.var_e0eb_dn8 = if 0.0 == 0.0 && ((assign3120_e2969) as f64).is_finite() && ((assign3120_e2969) as f64).fract() == 0.0 { if assign3120_e2969 == 0.0 { 0.0 } else { (assign3120_e2969 * ((assign3120_e2966).powf(assign3120_e2969 - 1.0) * (-((locals.var_vje_dn8 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn8))))) } } else { (assign3120_e2970 * (assign3120_e2969 * ((-((locals.var_vje_dn8 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn8))) / assign3120_e2966))) };
        locals.var_e0eb_dn9 = if 0.0 == 0.0 && ((assign3120_e2969) as f64).is_finite() && ((assign3120_e2969) as f64).fract() == 0.0 { if assign3120_e2969 == 0.0 { 0.0 } else { (assign3120_e2969 * ((assign3120_e2966).powf(assign3120_e2969 - 1.0) * (-((locals.var_vje_dn9 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn9))))) } } else { (assign3120_e2970 * (assign3120_e2969 * ((-((locals.var_vje_dn9 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn9))) / assign3120_e2966))) };
        locals.var_e0eb_dn10 = if 0.0 == 0.0 && ((assign3120_e2969) as f64).is_finite() && ((assign3120_e2969) as f64).fract() == 0.0 { if assign3120_e2969 == 0.0 { 0.0 } else { (assign3120_e2969 * ((assign3120_e2966).powf(assign3120_e2969 - 1.0) * (-((locals.var_vje_dn10 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn10))))) } } else { (assign3120_e2970 * (assign3120_e2969 * ((-((locals.var_vje_dn10 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn10))) / assign3120_e2966))) };
        locals.var_e0eb_rv = 0.0;

        let assign3130_e2974: f64 = (1.0 - p.p66);
        let assign3130_e2975: f64 = (locals.var_vde_t / assign3130_e2974);
        let assign3130_e2978: f64 = (1.0 - locals.var_e0eb);
        let assign3130_e2979: f64 = (assign3130_e2975 * assign3130_e2978);
        let assign3130_e2983: f64 = (locals.var_vb2e1 - locals.var_vje);
        let assign3130_e2984: f64 = (3.0 * assign3130_e2983);
        let assign3130_e2985: f64 = (assign3130_e2979 + assign3130_e2984);
        locals.var_vte = assign3130_e2985;
        locals.var_vte_dn0 = ((((locals.var_vde_t_dn0 / assign3130_e2974) * assign3130_e2978) + (assign3130_e2975 * (-locals.var_e0eb_dn0))) + (3.0 * (-locals.var_vje_dn0)));
        locals.var_vte_dn1 = ((((locals.var_vde_t_dn1 / assign3130_e2974) * assign3130_e2978) + (assign3130_e2975 * (-locals.var_e0eb_dn1))) + (3.0 * (-locals.var_vje_dn1)));
        locals.var_vte_dn3 = ((((locals.var_vde_t_dn3 / assign3130_e2974) * assign3130_e2978) + (assign3130_e2975 * (-locals.var_e0eb_dn3))) + (3.0 * (-locals.var_vje_dn3)));
        locals.var_vte_dn4 = ((((locals.var_vde_t_dn4 / assign3130_e2974) * assign3130_e2978) + (assign3130_e2975 * (-locals.var_e0eb_dn4))) + (3.0 * (locals.var_vb2e1_dn4 - locals.var_vje_dn4)));
        locals.var_vte_dn5 = ((((locals.var_vde_t_dn5 / assign3130_e2974) * assign3130_e2978) + (assign3130_e2975 * (-locals.var_e0eb_dn5))) + (3.0 * (-locals.var_vje_dn5)));
        locals.var_vte_dn6 = ((((locals.var_vde_t_dn6 / assign3130_e2974) * assign3130_e2978) + (assign3130_e2975 * (-locals.var_e0eb_dn6))) + (3.0 * (locals.var_vb2e1_dn6 - locals.var_vje_dn6)));
        locals.var_vte_dn7 = ((((locals.var_vde_t_dn7 / assign3130_e2974) * assign3130_e2978) + (assign3130_e2975 * (-locals.var_e0eb_dn7))) + (3.0 * (-locals.var_vje_dn7)));
        locals.var_vte_dn8 = ((((locals.var_vde_t_dn8 / assign3130_e2974) * assign3130_e2978) + (assign3130_e2975 * (-locals.var_e0eb_dn8))) + (3.0 * (-locals.var_vje_dn8)));
        locals.var_vte_dn9 = ((((locals.var_vde_t_dn9 / assign3130_e2974) * assign3130_e2978) + (assign3130_e2975 * (-locals.var_e0eb_dn9))) + (3.0 * (-locals.var_vje_dn9)));
        locals.var_vte_dn10 = ((((locals.var_vde_t_dn10 / assign3130_e2974) * assign3130_e2978) + (assign3130_e2975 * (-locals.var_e0eb_dn10))) + (3.0 * (-locals.var_vje_dn10)));
        locals.var_vte_rv = 0.0;

        let assign3140_e2988: f64 = if p.p73 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard49 = assign3140_e2988;
        locals.var_guard49_rv = 0.0;

        let (assign3150_e2992, assign3150_e2992_d_n0, assign3150_e2992_d_n1, assign3150_e2992_d_n3, assign3150_e2992_d_n4, assign3150_e2992_d_n5, assign3150_e2992_d_n6, assign3150_e2992_d_n7, assign3150_e2992_d_n8, assign3150_e2992_d_n9, assign3150_e2992_d_n10,) = {
    if (locals.var_guard49 != 0.0) {
        (locals.var_vb2c1, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_vb2c1_dn6, locals.var_vb2c1_dn7, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjunc, locals.var_vjunc_dn0, locals.var_vjunc_dn1, locals.var_vjunc_dn3, locals.var_vjunc_dn4, locals.var_vjunc_dn5, locals.var_vjunc_dn6, locals.var_vjunc_dn7, locals.var_vjunc_dn8, locals.var_vjunc_dn9, locals.var_vjunc_dn10,)
    }
};
        locals.var_vjunc = assign3150_e2992;
        locals.var_vjunc_dn0 = assign3150_e2992_d_n0;
        locals.var_vjunc_dn1 = assign3150_e2992_d_n1;
        locals.var_vjunc_dn3 = assign3150_e2992_d_n3;
        locals.var_vjunc_dn4 = assign3150_e2992_d_n4;
        locals.var_vjunc_dn5 = assign3150_e2992_d_n5;
        locals.var_vjunc_dn6 = assign3150_e2992_d_n6;
        locals.var_vjunc_dn7 = assign3150_e2992_d_n7;
        locals.var_vjunc_dn8 = assign3150_e2992_d_n8;
        locals.var_vjunc_dn9 = assign3150_e2992_d_n9;
        locals.var_vjunc_dn10 = assign3150_e2992_d_n10;
        locals.var_vjunc_rv = 0.0;

        let assign3160_e2995: f64 = if p.p73 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard50 = assign3160_e2995;
        locals.var_guard50_rv = 0.0;

        let (assign3170_e3004, assign3170_e3004_d_n0, assign3170_e3004_d_n1, assign3170_e3004_d_n3, assign3170_e3004_d_n4, assign3170_e3004_d_n5, assign3170_e3004_d_n6, assign3170_e3004_d_n7, assign3170_e3004_d_n8, assign3170_e3004_d_n9, assign3170_e3004_d_n10,) = {
    if ((locals.var_guard49 == 0.0) && (locals.var_guard50 != 0.0)) {
        let assign3170_e3002: f64 = (locals.var_vb2c1 + locals.var_vxi0);
        (assign3170_e3002, locals.var_vxi0_dn0, locals.var_vxi0_dn1, locals.var_vxi0_dn3, locals.var_vxi0_dn4, locals.var_vxi0_dn5, (locals.var_vb2c1_dn6 + locals.var_vxi0_dn6), (locals.var_vb2c1_dn7 + locals.var_vxi0_dn7), locals.var_vxi0_dn8, locals.var_vxi0_dn9, locals.var_vxi0_dn10,)
    } else {
        (locals.var_vjunc, locals.var_vjunc_dn0, locals.var_vjunc_dn1, locals.var_vjunc_dn3, locals.var_vjunc_dn4, locals.var_vjunc_dn5, locals.var_vjunc_dn6, locals.var_vjunc_dn7, locals.var_vjunc_dn8, locals.var_vjunc_dn9, locals.var_vjunc_dn10,)
    }
};
        locals.var_vjunc = assign3170_e3004;
        locals.var_vjunc_dn0 = assign3170_e3004_d_n0;
        locals.var_vjunc_dn1 = assign3170_e3004_d_n1;
        locals.var_vjunc_dn3 = assign3170_e3004_d_n3;
        locals.var_vjunc_dn4 = assign3170_e3004_d_n4;
        locals.var_vjunc_dn5 = assign3170_e3004_d_n5;
        locals.var_vjunc_dn6 = assign3170_e3004_d_n6;
        locals.var_vjunc_dn7 = assign3170_e3004_d_n7;
        locals.var_vjunc_dn8 = assign3170_e3004_d_n8;
        locals.var_vjunc_dn9 = assign3170_e3004_d_n9;
        locals.var_vjunc_dn10 = assign3170_e3004_d_n10;
        locals.var_vjunc_rv = 0.0;

        let (assign3180_e3012, assign3180_e3012_d_n0, assign3180_e3012_d_n1, assign3180_e3012_d_n3, assign3180_e3012_d_n4, assign3180_e3012_d_n5, assign3180_e3012_d_n6, assign3180_e3012_d_n7, assign3180_e3012_d_n8, assign3180_e3012_d_n9, assign3180_e3012_d_n10,) = {
    if ((locals.var_guard49 == 0.0) && (locals.var_guard50 == 0.0)) {
        (locals.var_vb2c2, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_vb2c2_dn6, 0.0, locals.var_vb2c2_dn8, 0.0, 0.0,)
    } else {
        (locals.var_vjunc, locals.var_vjunc_dn0, locals.var_vjunc_dn1, locals.var_vjunc_dn3, locals.var_vjunc_dn4, locals.var_vjunc_dn5, locals.var_vjunc_dn6, locals.var_vjunc_dn7, locals.var_vjunc_dn8, locals.var_vjunc_dn9, locals.var_vjunc_dn10,)
    }
};
        locals.var_vjunc = assign3180_e3012;
        locals.var_vjunc_dn0 = assign3180_e3012_d_n0;
        locals.var_vjunc_dn1 = assign3180_e3012_d_n1;
        locals.var_vjunc_dn3 = assign3180_e3012_d_n3;
        locals.var_vjunc_dn4 = assign3180_e3012_d_n4;
        locals.var_vjunc_dn5 = assign3180_e3012_d_n5;
        locals.var_vjunc_dn6 = assign3180_e3012_d_n6;
        locals.var_vjunc_dn7 = assign3180_e3012_d_n7;
        locals.var_vjunc_dn8 = assign3180_e3012_d_n8;
        locals.var_vjunc_dn9 = assign3180_e3012_d_n9;
        locals.var_vjunc_dn10 = assign3180_e3012_d_n10;
        locals.var_vjunc_rv = 0.0;

        let assign3190_e3015: f64 = (2.0 - locals.var_xp_t);
        let assign3190_e3018: f64 = (1.0 - locals.var_xp_t);
        let assign3190_e3019: f64 = (assign3190_e3015 / assign3190_e3018);
        locals.var_bjc = assign3190_e3019;
        locals.var_bjc_dn0 = ((((-locals.var_xp_t_dn0) * assign3190_e3018) - (assign3190_e3015 * (-locals.var_xp_t_dn0))) / (assign3190_e3018 * assign3190_e3018));
        locals.var_bjc_dn1 = ((((-locals.var_xp_t_dn1) * assign3190_e3018) - (assign3190_e3015 * (-locals.var_xp_t_dn1))) / (assign3190_e3018 * assign3190_e3018));
        locals.var_bjc_dn3 = ((((-locals.var_xp_t_dn3) * assign3190_e3018) - (assign3190_e3015 * (-locals.var_xp_t_dn3))) / (assign3190_e3018 * assign3190_e3018));
        locals.var_bjc_dn4 = ((((-locals.var_xp_t_dn4) * assign3190_e3018) - (assign3190_e3015 * (-locals.var_xp_t_dn4))) / (assign3190_e3018 * assign3190_e3018));
        locals.var_bjc_dn5 = ((((-locals.var_xp_t_dn5) * assign3190_e3018) - (assign3190_e3015 * (-locals.var_xp_t_dn5))) / (assign3190_e3018 * assign3190_e3018));
        locals.var_bjc_dn6 = ((((-locals.var_xp_t_dn6) * assign3190_e3018) - (assign3190_e3015 * (-locals.var_xp_t_dn6))) / (assign3190_e3018 * assign3190_e3018));
        locals.var_bjc_dn7 = ((((-locals.var_xp_t_dn7) * assign3190_e3018) - (assign3190_e3015 * (-locals.var_xp_t_dn7))) / (assign3190_e3018 * assign3190_e3018));
        locals.var_bjc_dn8 = ((((-locals.var_xp_t_dn8) * assign3190_e3018) - (assign3190_e3015 * (-locals.var_xp_t_dn8))) / (assign3190_e3018 * assign3190_e3018));
        locals.var_bjc_dn9 = ((((-locals.var_xp_t_dn9) * assign3190_e3018) - (assign3190_e3015 * (-locals.var_xp_t_dn9))) / (assign3190_e3018 * assign3190_e3018));
        locals.var_bjc_dn10 = ((((-locals.var_xp_t_dn10) * assign3190_e3018) - (assign3190_e3015 * (-locals.var_xp_t_dn10))) / (assign3190_e3018 * assign3190_e3018));
        locals.var_bjc_rv = 0.0;

        let assign3200_e3024: f64 = (-1.0);
        let assign3200_e3026: f64 = (assign3200_e3024 / p.p71);
        let assign3200_e3027: f64 = (locals.var_bjc).powf(assign3200_e3026);
        let assign3200_e3028: f64 = (1.0 - assign3200_e3027);
        let assign3200_e3029: f64 = (locals.var_vdc_ctc_t * assign3200_e3028);
        locals.var_vfc = assign3200_e3029;
        locals.var_vfc_dn0 = ((locals.var_vdc_ctc_t_dn0 * assign3200_e3028) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3200_e3026) as f64).is_finite() && ((assign3200_e3026) as f64).fract() == 0.0 { if assign3200_e3026 == 0.0 { 0.0 } else { (assign3200_e3026 * ((locals.var_bjc).powf(assign3200_e3026 - 1.0) * locals.var_bjc_dn0)) } } else { (assign3200_e3027 * (assign3200_e3026 * (locals.var_bjc_dn0 / locals.var_bjc))) })));
        locals.var_vfc_dn1 = ((locals.var_vdc_ctc_t_dn1 * assign3200_e3028) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3200_e3026) as f64).is_finite() && ((assign3200_e3026) as f64).fract() == 0.0 { if assign3200_e3026 == 0.0 { 0.0 } else { (assign3200_e3026 * ((locals.var_bjc).powf(assign3200_e3026 - 1.0) * locals.var_bjc_dn1)) } } else { (assign3200_e3027 * (assign3200_e3026 * (locals.var_bjc_dn1 / locals.var_bjc))) })));
        locals.var_vfc_dn3 = ((locals.var_vdc_ctc_t_dn3 * assign3200_e3028) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3200_e3026) as f64).is_finite() && ((assign3200_e3026) as f64).fract() == 0.0 { if assign3200_e3026 == 0.0 { 0.0 } else { (assign3200_e3026 * ((locals.var_bjc).powf(assign3200_e3026 - 1.0) * locals.var_bjc_dn3)) } } else { (assign3200_e3027 * (assign3200_e3026 * (locals.var_bjc_dn3 / locals.var_bjc))) })));
        locals.var_vfc_dn4 = ((locals.var_vdc_ctc_t_dn4 * assign3200_e3028) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3200_e3026) as f64).is_finite() && ((assign3200_e3026) as f64).fract() == 0.0 { if assign3200_e3026 == 0.0 { 0.0 } else { (assign3200_e3026 * ((locals.var_bjc).powf(assign3200_e3026 - 1.0) * locals.var_bjc_dn4)) } } else { (assign3200_e3027 * (assign3200_e3026 * (locals.var_bjc_dn4 / locals.var_bjc))) })));
        locals.var_vfc_dn5 = ((locals.var_vdc_ctc_t_dn5 * assign3200_e3028) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3200_e3026) as f64).is_finite() && ((assign3200_e3026) as f64).fract() == 0.0 { if assign3200_e3026 == 0.0 { 0.0 } else { (assign3200_e3026 * ((locals.var_bjc).powf(assign3200_e3026 - 1.0) * locals.var_bjc_dn5)) } } else { (assign3200_e3027 * (assign3200_e3026 * (locals.var_bjc_dn5 / locals.var_bjc))) })));
        locals.var_vfc_dn6 = ((locals.var_vdc_ctc_t_dn6 * assign3200_e3028) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3200_e3026) as f64).is_finite() && ((assign3200_e3026) as f64).fract() == 0.0 { if assign3200_e3026 == 0.0 { 0.0 } else { (assign3200_e3026 * ((locals.var_bjc).powf(assign3200_e3026 - 1.0) * locals.var_bjc_dn6)) } } else { (assign3200_e3027 * (assign3200_e3026 * (locals.var_bjc_dn6 / locals.var_bjc))) })));
        locals.var_vfc_dn7 = ((locals.var_vdc_ctc_t_dn7 * assign3200_e3028) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3200_e3026) as f64).is_finite() && ((assign3200_e3026) as f64).fract() == 0.0 { if assign3200_e3026 == 0.0 { 0.0 } else { (assign3200_e3026 * ((locals.var_bjc).powf(assign3200_e3026 - 1.0) * locals.var_bjc_dn7)) } } else { (assign3200_e3027 * (assign3200_e3026 * (locals.var_bjc_dn7 / locals.var_bjc))) })));
        locals.var_vfc_dn8 = ((locals.var_vdc_ctc_t_dn8 * assign3200_e3028) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3200_e3026) as f64).is_finite() && ((assign3200_e3026) as f64).fract() == 0.0 { if assign3200_e3026 == 0.0 { 0.0 } else { (assign3200_e3026 * ((locals.var_bjc).powf(assign3200_e3026 - 1.0) * locals.var_bjc_dn8)) } } else { (assign3200_e3027 * (assign3200_e3026 * (locals.var_bjc_dn8 / locals.var_bjc))) })));
        locals.var_vfc_dn9 = ((locals.var_vdc_ctc_t_dn9 * assign3200_e3028) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3200_e3026) as f64).is_finite() && ((assign3200_e3026) as f64).fract() == 0.0 { if assign3200_e3026 == 0.0 { 0.0 } else { (assign3200_e3026 * ((locals.var_bjc).powf(assign3200_e3026 - 1.0) * locals.var_bjc_dn9)) } } else { (assign3200_e3027 * (assign3200_e3026 * (locals.var_bjc_dn9 / locals.var_bjc))) })));
        locals.var_vfc_dn10 = ((locals.var_vdc_ctc_t_dn10 * assign3200_e3028) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3200_e3026) as f64).is_finite() && ((assign3200_e3026) as f64).fract() == 0.0 { if assign3200_e3026 == 0.0 { 0.0 } else { (assign3200_e3026 * ((locals.var_bjc).powf(assign3200_e3026 - 1.0) * locals.var_bjc_dn10)) } } else { (assign3200_e3027 * (assign3200_e3026 * (locals.var_bjc_dn10 / locals.var_bjc))) })));
        locals.var_vfc_rv = 0.0;

        let assign3210_e3032: f64 = (locals.var_vjunc - locals.var_vfc);
        let assign3210_e3034: f64 = (assign3210_e3032 / locals.var_vch);
        locals.var_dxa = assign3210_e3034;
        locals.var_dxa_dn0 = ((((locals.var_vjunc_dn0 - locals.var_vfc_dn0) * locals.var_vch) - (assign3210_e3032 * locals.var_vch_dn0)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn1 = ((((locals.var_vjunc_dn1 - locals.var_vfc_dn1) * locals.var_vch) - (assign3210_e3032 * locals.var_vch_dn1)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn3 = ((((locals.var_vjunc_dn3 - locals.var_vfc_dn3) * locals.var_vch) - (assign3210_e3032 * locals.var_vch_dn3)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn4 = ((((locals.var_vjunc_dn4 - locals.var_vfc_dn4) * locals.var_vch) - (assign3210_e3032 * locals.var_vch_dn4)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn5 = ((((locals.var_vjunc_dn5 - locals.var_vfc_dn5) * locals.var_vch) - (assign3210_e3032 * locals.var_vch_dn5)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn6 = ((((locals.var_vjunc_dn6 - locals.var_vfc_dn6) * locals.var_vch) - (assign3210_e3032 * locals.var_vch_dn6)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn7 = ((((locals.var_vjunc_dn7 - locals.var_vfc_dn7) * locals.var_vch) - (assign3210_e3032 * locals.var_vch_dn7)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn8 = ((((locals.var_vjunc_dn8 - locals.var_vfc_dn8) * locals.var_vch) - (assign3210_e3032 * locals.var_vch_dn8)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn9 = ((((locals.var_vjunc_dn9 - locals.var_vfc_dn9) * locals.var_vch) - (assign3210_e3032 * locals.var_vch_dn9)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn10 = ((((locals.var_vjunc_dn10 - locals.var_vfc_dn10) * locals.var_vch) - (assign3210_e3032 * locals.var_vch_dn10)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_rv = 0.0;

        let assign3220_e3037: f64 = if locals.var_vjunc < locals.var_vfc { 1.0 } else { 0.0 };
        locals.var_guard51 = assign3220_e3037;
        locals.var_guard51_rv = 0.0;

        let (assign3230_e3049, assign3230_e3049_d_n0, assign3230_e3049_d_n1, assign3230_e3049_d_n3, assign3230_e3049_d_n4, assign3230_e3049_d_n5, assign3230_e3049_d_n6, assign3230_e3049_d_n7, assign3230_e3049_d_n8, assign3230_e3049_d_n9, assign3230_e3049_d_n10,) = {
    if (locals.var_guard51 != 0.0) {
        let assign3230_e3043: f64 = (locals.var_dxa).exp();
        let assign3230_e3044: f64 = (1.0 + assign3230_e3043);
        let assign3230_e3045: f64 = (assign3230_e3044).ln();
        let assign3230_e3046: f64 = (locals.var_vch * assign3230_e3045);
        let assign3230_e3047: f64 = (locals.var_vjunc - assign3230_e3046);
        (assign3230_e3047, (locals.var_vjunc_dn0 - ((locals.var_vch_dn0 * assign3230_e3045) + (locals.var_vch * ((assign3230_e3043 * locals.var_dxa_dn0) / assign3230_e3044)))), (locals.var_vjunc_dn1 - ((locals.var_vch_dn1 * assign3230_e3045) + (locals.var_vch * ((assign3230_e3043 * locals.var_dxa_dn1) / assign3230_e3044)))), (locals.var_vjunc_dn3 - ((locals.var_vch_dn3 * assign3230_e3045) + (locals.var_vch * ((assign3230_e3043 * locals.var_dxa_dn3) / assign3230_e3044)))), (locals.var_vjunc_dn4 - ((locals.var_vch_dn4 * assign3230_e3045) + (locals.var_vch * ((assign3230_e3043 * locals.var_dxa_dn4) / assign3230_e3044)))), (locals.var_vjunc_dn5 - ((locals.var_vch_dn5 * assign3230_e3045) + (locals.var_vch * ((assign3230_e3043 * locals.var_dxa_dn5) / assign3230_e3044)))), (locals.var_vjunc_dn6 - ((locals.var_vch_dn6 * assign3230_e3045) + (locals.var_vch * ((assign3230_e3043 * locals.var_dxa_dn6) / assign3230_e3044)))), (locals.var_vjunc_dn7 - ((locals.var_vch_dn7 * assign3230_e3045) + (locals.var_vch * ((assign3230_e3043 * locals.var_dxa_dn7) / assign3230_e3044)))), (locals.var_vjunc_dn8 - ((locals.var_vch_dn8 * assign3230_e3045) + (locals.var_vch * ((assign3230_e3043 * locals.var_dxa_dn8) / assign3230_e3044)))), (locals.var_vjunc_dn9 - ((locals.var_vch_dn9 * assign3230_e3045) + (locals.var_vch * ((assign3230_e3043 * locals.var_dxa_dn9) / assign3230_e3044)))), (locals.var_vjunc_dn10 - ((locals.var_vch_dn10 * assign3230_e3045) + (locals.var_vch * ((assign3230_e3043 * locals.var_dxa_dn10) / assign3230_e3044)))),)
    } else {
        (locals.var_vjc, locals.var_vjc_dn0, locals.var_vjc_dn1, locals.var_vjc_dn3, locals.var_vjc_dn4, locals.var_vjc_dn5, locals.var_vjc_dn6, locals.var_vjc_dn7, locals.var_vjc_dn8, locals.var_vjc_dn9, locals.var_vjc_dn10,)
    }
};
        locals.var_vjc = assign3230_e3049;
        locals.var_vjc_dn0 = assign3230_e3049_d_n0;
        locals.var_vjc_dn1 = assign3230_e3049_d_n1;
        locals.var_vjc_dn3 = assign3230_e3049_d_n3;
        locals.var_vjc_dn4 = assign3230_e3049_d_n4;
        locals.var_vjc_dn5 = assign3230_e3049_d_n5;
        locals.var_vjc_dn6 = assign3230_e3049_d_n6;
        locals.var_vjc_dn7 = assign3230_e3049_d_n7;
        locals.var_vjc_dn8 = assign3230_e3049_d_n8;
        locals.var_vjc_dn9 = assign3230_e3049_d_n9;
        locals.var_vjc_dn10 = assign3230_e3049_d_n10;
        locals.var_vjc_rv = 0.0;

        let (assign3240_e3063, assign3240_e3063_d_n0, assign3240_e3063_d_n1, assign3240_e3063_d_n3, assign3240_e3063_d_n4, assign3240_e3063_d_n5, assign3240_e3063_d_n6, assign3240_e3063_d_n7, assign3240_e3063_d_n8, assign3240_e3063_d_n9, assign3240_e3063_d_n10,) = {
    if (locals.var_guard51 == 0.0) {
        let assign3240_e3056: f64 = (-locals.var_dxa);
        let assign3240_e3057: f64 = (assign3240_e3056).exp();
        let assign3240_e3058: f64 = (1.0 + assign3240_e3057);
        let assign3240_e3059: f64 = (assign3240_e3058).ln();
        let assign3240_e3060: f64 = (locals.var_vch * assign3240_e3059);
        let assign3240_e3061: f64 = (locals.var_vfc - assign3240_e3060);
        (assign3240_e3061, (locals.var_vfc_dn0 - ((locals.var_vch_dn0 * assign3240_e3059) + (locals.var_vch * ((assign3240_e3057 * (-locals.var_dxa_dn0)) / assign3240_e3058)))), (locals.var_vfc_dn1 - ((locals.var_vch_dn1 * assign3240_e3059) + (locals.var_vch * ((assign3240_e3057 * (-locals.var_dxa_dn1)) / assign3240_e3058)))), (locals.var_vfc_dn3 - ((locals.var_vch_dn3 * assign3240_e3059) + (locals.var_vch * ((assign3240_e3057 * (-locals.var_dxa_dn3)) / assign3240_e3058)))), (locals.var_vfc_dn4 - ((locals.var_vch_dn4 * assign3240_e3059) + (locals.var_vch * ((assign3240_e3057 * (-locals.var_dxa_dn4)) / assign3240_e3058)))), (locals.var_vfc_dn5 - ((locals.var_vch_dn5 * assign3240_e3059) + (locals.var_vch * ((assign3240_e3057 * (-locals.var_dxa_dn5)) / assign3240_e3058)))), (locals.var_vfc_dn6 - ((locals.var_vch_dn6 * assign3240_e3059) + (locals.var_vch * ((assign3240_e3057 * (-locals.var_dxa_dn6)) / assign3240_e3058)))), (locals.var_vfc_dn7 - ((locals.var_vch_dn7 * assign3240_e3059) + (locals.var_vch * ((assign3240_e3057 * (-locals.var_dxa_dn7)) / assign3240_e3058)))), (locals.var_vfc_dn8 - ((locals.var_vch_dn8 * assign3240_e3059) + (locals.var_vch * ((assign3240_e3057 * (-locals.var_dxa_dn8)) / assign3240_e3058)))), (locals.var_vfc_dn9 - ((locals.var_vch_dn9 * assign3240_e3059) + (locals.var_vch * ((assign3240_e3057 * (-locals.var_dxa_dn9)) / assign3240_e3058)))), (locals.var_vfc_dn10 - ((locals.var_vch_dn10 * assign3240_e3059) + (locals.var_vch * ((assign3240_e3057 * (-locals.var_dxa_dn10)) / assign3240_e3058)))),)
    } else {
        (locals.var_vjc, locals.var_vjc_dn0, locals.var_vjc_dn1, locals.var_vjc_dn3, locals.var_vjc_dn4, locals.var_vjc_dn5, locals.var_vjc_dn6, locals.var_vjc_dn7, locals.var_vjc_dn8, locals.var_vjc_dn9, locals.var_vjc_dn10,)
    }
};
        locals.var_vjc = assign3240_e3063;
        locals.var_vjc_dn0 = assign3240_e3063_d_n0;
        locals.var_vjc_dn1 = assign3240_e3063_d_n1;
        locals.var_vjc_dn3 = assign3240_e3063_d_n3;
        locals.var_vjc_dn4 = assign3240_e3063_d_n4;
        locals.var_vjc_dn5 = assign3240_e3063_d_n5;
        locals.var_vjc_dn6 = assign3240_e3063_d_n6;
        locals.var_vjc_dn7 = assign3240_e3063_d_n7;
        locals.var_vjc_dn8 = assign3240_e3063_d_n8;
        locals.var_vjc_dn9 = assign3240_e3063_d_n9;
        locals.var_vjc_dn10 = assign3240_e3063_d_n10;
        locals.var_vjc_rv = 0.0;

        let assign3250_e3066: f64 = (locals.var_icap_ihc).powf(p.p75);
        locals.var_fi = assign3250_e3066;
        locals.var_fi_dn0 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn0)) } } else { (assign3250_e3066 * (p.p75 * (locals.var_icap_ihc_dn0 / locals.var_icap_ihc))) };
        locals.var_fi_dn1 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn1)) } } else { (assign3250_e3066 * (p.p75 * (locals.var_icap_ihc_dn1 / locals.var_icap_ihc))) };
        locals.var_fi_dn3 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn3)) } } else { (assign3250_e3066 * (p.p75 * (locals.var_icap_ihc_dn3 / locals.var_icap_ihc))) };
        locals.var_fi_dn4 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn4)) } } else { (assign3250_e3066 * (p.p75 * (locals.var_icap_ihc_dn4 / locals.var_icap_ihc))) };
        locals.var_fi_dn5 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn5)) } } else { (assign3250_e3066 * (p.p75 * (locals.var_icap_ihc_dn5 / locals.var_icap_ihc))) };
        locals.var_fi_dn6 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn6)) } } else { (assign3250_e3066 * (p.p75 * (locals.var_icap_ihc_dn6 / locals.var_icap_ihc))) };
        locals.var_fi_dn7 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn7)) } } else { (assign3250_e3066 * (p.p75 * (locals.var_icap_ihc_dn7 / locals.var_icap_ihc))) };
        locals.var_fi_dn8 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn8)) } } else { (assign3250_e3066 * (p.p75 * (locals.var_icap_ihc_dn8 / locals.var_icap_ihc))) };
        locals.var_fi_dn9 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn9)) } } else { (assign3250_e3066 * (p.p75 * (locals.var_icap_ihc_dn9 / locals.var_icap_ihc))) };
        locals.var_fi_dn10 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn10)) } } else { (assign3250_e3066 * (p.p75 * (locals.var_icap_ihc_dn10 / locals.var_icap_ihc))) };
        locals.var_fi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3260_e3070: f64 = (1.0 - p.p71);
        let assign3260_e3071: f64 = (locals.var_vdc_ctc_t / assign3260_e3070);
        let assign3260_e3077: f64 = (locals.var_vjc / locals.var_vdc_ctc_t);
        let assign3260_e3078: f64 = (1.0 - assign3260_e3077);
        let assign3260_e3081: f64 = (1.0 - p.p71);
        let assign3260_e3082: f64 = (assign3260_e3078).powf(assign3260_e3081);
        let assign3260_e3083: f64 = (locals.var_fi * assign3260_e3082);
        let assign3260_e3084: f64 = (1.0 - assign3260_e3083);
        let assign3260_e3085: f64 = (assign3260_e3071 * assign3260_e3084);
        let assign3260_e3088: f64 = (locals.var_fi * locals.var_bjc);
        let assign3260_e3091: f64 = (locals.var_vjunc - locals.var_vjc);
        let assign3260_e3092: f64 = (assign3260_e3088 * assign3260_e3091);
        let assign3260_e3093: f64 = (assign3260_e3085 + assign3260_e3092);
        locals.var_vcv = assign3260_e3093;
        locals.var_vcv_dn0 = ((((locals.var_vdc_ctc_t_dn0 / assign3260_e3070) * assign3260_e3084) + (assign3260_e3071 * (-((locals.var_fi_dn0 * assign3260_e3082) + (locals.var_fi * if 0.0 == 0.0 && ((assign3260_e3081) as f64).is_finite() && ((assign3260_e3081) as f64).fract() == 0.0 { if assign3260_e3081 == 0.0 { 0.0 } else { (assign3260_e3081 * ((assign3260_e3078).powf(assign3260_e3081 - 1.0) * (-(((locals.var_vjc_dn0 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3260_e3082 * (assign3260_e3081 * ((-(((locals.var_vjc_dn0 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3260_e3078))) }))))) + ((((locals.var_fi_dn0 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn0)) * assign3260_e3091) + (assign3260_e3088 * (locals.var_vjunc_dn0 - locals.var_vjc_dn0))));
        locals.var_vcv_dn1 = ((((locals.var_vdc_ctc_t_dn1 / assign3260_e3070) * assign3260_e3084) + (assign3260_e3071 * (-((locals.var_fi_dn1 * assign3260_e3082) + (locals.var_fi * if 0.0 == 0.0 && ((assign3260_e3081) as f64).is_finite() && ((assign3260_e3081) as f64).fract() == 0.0 { if assign3260_e3081 == 0.0 { 0.0 } else { (assign3260_e3081 * ((assign3260_e3078).powf(assign3260_e3081 - 1.0) * (-(((locals.var_vjc_dn1 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3260_e3082 * (assign3260_e3081 * ((-(((locals.var_vjc_dn1 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3260_e3078))) }))))) + ((((locals.var_fi_dn1 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn1)) * assign3260_e3091) + (assign3260_e3088 * (locals.var_vjunc_dn1 - locals.var_vjc_dn1))));
        locals.var_vcv_dn3 = ((((locals.var_vdc_ctc_t_dn3 / assign3260_e3070) * assign3260_e3084) + (assign3260_e3071 * (-((locals.var_fi_dn3 * assign3260_e3082) + (locals.var_fi * if 0.0 == 0.0 && ((assign3260_e3081) as f64).is_finite() && ((assign3260_e3081) as f64).fract() == 0.0 { if assign3260_e3081 == 0.0 { 0.0 } else { (assign3260_e3081 * ((assign3260_e3078).powf(assign3260_e3081 - 1.0) * (-(((locals.var_vjc_dn3 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3260_e3082 * (assign3260_e3081 * ((-(((locals.var_vjc_dn3 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3260_e3078))) }))))) + ((((locals.var_fi_dn3 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn3)) * assign3260_e3091) + (assign3260_e3088 * (locals.var_vjunc_dn3 - locals.var_vjc_dn3))));
        locals.var_vcv_dn4 = ((((locals.var_vdc_ctc_t_dn4 / assign3260_e3070) * assign3260_e3084) + (assign3260_e3071 * (-((locals.var_fi_dn4 * assign3260_e3082) + (locals.var_fi * if 0.0 == 0.0 && ((assign3260_e3081) as f64).is_finite() && ((assign3260_e3081) as f64).fract() == 0.0 { if assign3260_e3081 == 0.0 { 0.0 } else { (assign3260_e3081 * ((assign3260_e3078).powf(assign3260_e3081 - 1.0) * (-(((locals.var_vjc_dn4 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3260_e3082 * (assign3260_e3081 * ((-(((locals.var_vjc_dn4 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3260_e3078))) }))))) + ((((locals.var_fi_dn4 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn4)) * assign3260_e3091) + (assign3260_e3088 * (locals.var_vjunc_dn4 - locals.var_vjc_dn4))));
        locals.var_vcv_dn5 = ((((locals.var_vdc_ctc_t_dn5 / assign3260_e3070) * assign3260_e3084) + (assign3260_e3071 * (-((locals.var_fi_dn5 * assign3260_e3082) + (locals.var_fi * if 0.0 == 0.0 && ((assign3260_e3081) as f64).is_finite() && ((assign3260_e3081) as f64).fract() == 0.0 { if assign3260_e3081 == 0.0 { 0.0 } else { (assign3260_e3081 * ((assign3260_e3078).powf(assign3260_e3081 - 1.0) * (-(((locals.var_vjc_dn5 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3260_e3082 * (assign3260_e3081 * ((-(((locals.var_vjc_dn5 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3260_e3078))) }))))) + ((((locals.var_fi_dn5 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn5)) * assign3260_e3091) + (assign3260_e3088 * (locals.var_vjunc_dn5 - locals.var_vjc_dn5))));
        locals.var_vcv_dn6 = ((((locals.var_vdc_ctc_t_dn6 / assign3260_e3070) * assign3260_e3084) + (assign3260_e3071 * (-((locals.var_fi_dn6 * assign3260_e3082) + (locals.var_fi * if 0.0 == 0.0 && ((assign3260_e3081) as f64).is_finite() && ((assign3260_e3081) as f64).fract() == 0.0 { if assign3260_e3081 == 0.0 { 0.0 } else { (assign3260_e3081 * ((assign3260_e3078).powf(assign3260_e3081 - 1.0) * (-(((locals.var_vjc_dn6 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3260_e3082 * (assign3260_e3081 * ((-(((locals.var_vjc_dn6 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3260_e3078))) }))))) + ((((locals.var_fi_dn6 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn6)) * assign3260_e3091) + (assign3260_e3088 * (locals.var_vjunc_dn6 - locals.var_vjc_dn6))));
        locals.var_vcv_dn7 = ((((locals.var_vdc_ctc_t_dn7 / assign3260_e3070) * assign3260_e3084) + (assign3260_e3071 * (-((locals.var_fi_dn7 * assign3260_e3082) + (locals.var_fi * if 0.0 == 0.0 && ((assign3260_e3081) as f64).is_finite() && ((assign3260_e3081) as f64).fract() == 0.0 { if assign3260_e3081 == 0.0 { 0.0 } else { (assign3260_e3081 * ((assign3260_e3078).powf(assign3260_e3081 - 1.0) * (-(((locals.var_vjc_dn7 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3260_e3082 * (assign3260_e3081 * ((-(((locals.var_vjc_dn7 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3260_e3078))) }))))) + ((((locals.var_fi_dn7 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn7)) * assign3260_e3091) + (assign3260_e3088 * (locals.var_vjunc_dn7 - locals.var_vjc_dn7))));
        locals.var_vcv_dn8 = ((((locals.var_vdc_ctc_t_dn8 / assign3260_e3070) * assign3260_e3084) + (assign3260_e3071 * (-((locals.var_fi_dn8 * assign3260_e3082) + (locals.var_fi * if 0.0 == 0.0 && ((assign3260_e3081) as f64).is_finite() && ((assign3260_e3081) as f64).fract() == 0.0 { if assign3260_e3081 == 0.0 { 0.0 } else { (assign3260_e3081 * ((assign3260_e3078).powf(assign3260_e3081 - 1.0) * (-(((locals.var_vjc_dn8 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3260_e3082 * (assign3260_e3081 * ((-(((locals.var_vjc_dn8 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3260_e3078))) }))))) + ((((locals.var_fi_dn8 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn8)) * assign3260_e3091) + (assign3260_e3088 * (locals.var_vjunc_dn8 - locals.var_vjc_dn8))));
        locals.var_vcv_dn9 = ((((locals.var_vdc_ctc_t_dn9 / assign3260_e3070) * assign3260_e3084) + (assign3260_e3071 * (-((locals.var_fi_dn9 * assign3260_e3082) + (locals.var_fi * if 0.0 == 0.0 && ((assign3260_e3081) as f64).is_finite() && ((assign3260_e3081) as f64).fract() == 0.0 { if assign3260_e3081 == 0.0 { 0.0 } else { (assign3260_e3081 * ((assign3260_e3078).powf(assign3260_e3081 - 1.0) * (-(((locals.var_vjc_dn9 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3260_e3082 * (assign3260_e3081 * ((-(((locals.var_vjc_dn9 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3260_e3078))) }))))) + ((((locals.var_fi_dn9 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn9)) * assign3260_e3091) + (assign3260_e3088 * (locals.var_vjunc_dn9 - locals.var_vjc_dn9))));
        locals.var_vcv_dn10 = ((((locals.var_vdc_ctc_t_dn10 / assign3260_e3070) * assign3260_e3084) + (assign3260_e3071 * (-((locals.var_fi_dn10 * assign3260_e3082) + (locals.var_fi * if 0.0 == 0.0 && ((assign3260_e3081) as f64).is_finite() && ((assign3260_e3081) as f64).fract() == 0.0 { if assign3260_e3081 == 0.0 { 0.0 } else { (assign3260_e3081 * ((assign3260_e3078).powf(assign3260_e3081 - 1.0) * (-(((locals.var_vjc_dn10 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3260_e3082 * (assign3260_e3081 * ((-(((locals.var_vjc_dn10 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3260_e3078))) }))))) + ((((locals.var_fi_dn10 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn10)) * assign3260_e3091) + (assign3260_e3088 * (locals.var_vjunc_dn10 - locals.var_vjc_dn10))));
        locals.var_vcv_rv = 0.0;

        let assign3270_e3096: f64 = (1.0 - locals.var_xp_t);
        let assign3270_e3098: f64 = (assign3270_e3096 * locals.var_vcv);
        let assign3270_e3101: f64 = (locals.var_xp_t * locals.var_vb2c1);
        let assign3270_e3102: f64 = (assign3270_e3098 + assign3270_e3101);
        locals.var_vtc = assign3270_e3102;
        locals.var_vtc_dn0 = ((((-locals.var_xp_t_dn0) * locals.var_vcv) + (assign3270_e3096 * locals.var_vcv_dn0)) + (locals.var_xp_t_dn0 * locals.var_vb2c1));
        locals.var_vtc_dn1 = ((((-locals.var_xp_t_dn1) * locals.var_vcv) + (assign3270_e3096 * locals.var_vcv_dn1)) + (locals.var_xp_t_dn1 * locals.var_vb2c1));
        locals.var_vtc_dn3 = ((((-locals.var_xp_t_dn3) * locals.var_vcv) + (assign3270_e3096 * locals.var_vcv_dn3)) + (locals.var_xp_t_dn3 * locals.var_vb2c1));
        locals.var_vtc_dn4 = ((((-locals.var_xp_t_dn4) * locals.var_vcv) + (assign3270_e3096 * locals.var_vcv_dn4)) + (locals.var_xp_t_dn4 * locals.var_vb2c1));
        locals.var_vtc_dn5 = ((((-locals.var_xp_t_dn5) * locals.var_vcv) + (assign3270_e3096 * locals.var_vcv_dn5)) + (locals.var_xp_t_dn5 * locals.var_vb2c1));
        locals.var_vtc_dn6 = ((((-locals.var_xp_t_dn6) * locals.var_vcv) + (assign3270_e3096 * locals.var_vcv_dn6)) + ((locals.var_xp_t_dn6 * locals.var_vb2c1) + (locals.var_xp_t * locals.var_vb2c1_dn6)));
        locals.var_vtc_dn7 = ((((-locals.var_xp_t_dn7) * locals.var_vcv) + (assign3270_e3096 * locals.var_vcv_dn7)) + ((locals.var_xp_t_dn7 * locals.var_vb2c1) + (locals.var_xp_t * locals.var_vb2c1_dn7)));
        locals.var_vtc_dn8 = ((((-locals.var_xp_t_dn8) * locals.var_vcv) + (assign3270_e3096 * locals.var_vcv_dn8)) + (locals.var_xp_t_dn8 * locals.var_vb2c1));
        locals.var_vtc_dn9 = ((((-locals.var_xp_t_dn9) * locals.var_vcv) + (assign3270_e3096 * locals.var_vcv_dn9)) + (locals.var_xp_t_dn9 * locals.var_vb2c1));
        locals.var_vtc_dn10 = ((((-locals.var_xp_t_dn10) * locals.var_vcv) + (assign3270_e3096 * locals.var_vcv_dn10)) + (locals.var_xp_t_dn10 * locals.var_vb2c1));
        locals.var_vtc_rv = 0.0;

        let assign3280_e3105: f64 = (4.0 * locals.var_is_t);
        let assign3280_e3107: f64 = (assign3280_e3105 / locals.var_ik_t);
        locals.var_if0 = assign3280_e3107;
        locals.var_if0_dn0 = ((4.0 * locals.var_is_t_dn0) / locals.var_ik_t);
        locals.var_if0_dn1 = ((4.0 * locals.var_is_t_dn1) / locals.var_ik_t);
        locals.var_if0_dn3 = ((((4.0 * locals.var_is_t_dn3) * locals.var_ik_t) - (assign3280_e3105 * locals.var_ik_t_dn3)) / (locals.var_ik_t * locals.var_ik_t));
        locals.var_if0_dn4 = ((4.0 * locals.var_is_t_dn4) / locals.var_ik_t);
        locals.var_if0_dn5 = ((4.0 * locals.var_is_t_dn5) / locals.var_ik_t);
        locals.var_if0_dn6 = ((4.0 * locals.var_is_t_dn6) / locals.var_ik_t);
        locals.var_if0_dn7 = ((4.0 * locals.var_is_t_dn7) / locals.var_ik_t);
        locals.var_if0_dn8 = ((4.0 * locals.var_is_t_dn8) / locals.var_ik_t);
        locals.var_if0_dn9 = ((4.0 * locals.var_is_t_dn9) / locals.var_ik_t);
        locals.var_if0_dn10 = ((4.0 * locals.var_is_t_dn10) / locals.var_ik_t);
        locals.var_if0_rv = 0.0;

        let assign3290_e3110: f64 = (locals.var_if0 * locals.var_evb2e1);
        locals.var_f1 = assign3290_e3110;
        locals.var_f1_dn0 = ((locals.var_if0_dn0 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn0));
        locals.var_f1_dn1 = ((locals.var_if0_dn1 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn1));
        locals.var_f1_dn3 = ((locals.var_if0_dn3 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn3));
        locals.var_f1_dn4 = ((locals.var_if0_dn4 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn4));
        locals.var_f1_dn5 = ((locals.var_if0_dn5 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn5));
        locals.var_f1_dn6 = ((locals.var_if0_dn6 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn6));
        locals.var_f1_dn7 = ((locals.var_if0_dn7 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn7));
        locals.var_f1_dn8 = ((locals.var_if0_dn8 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn8));
        locals.var_f1_dn9 = ((locals.var_if0_dn9 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn9));
        locals.var_f1_dn10 = ((locals.var_if0_dn10 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn10));
        locals.var_f1_rv = 0.0;

        let assign3300_e3115: f64 = (1.0 + locals.var_f1);
        let assign3300_e3116: f64 = (assign3300_e3115).sqrt();
        let assign3300_e3117: f64 = (1.0 + assign3300_e3116);
        let assign3300_e3118: f64 = (locals.var_f1 / assign3300_e3117);
        locals.var_n0 = assign3300_e3118;
        locals.var_n0_dn0 = (((locals.var_f1_dn0 * assign3300_e3117) - (locals.var_f1 * (locals.var_f1_dn0 / (2.0 * assign3300_e3116)))) / (assign3300_e3117 * assign3300_e3117));
        locals.var_n0_dn1 = (((locals.var_f1_dn1 * assign3300_e3117) - (locals.var_f1 * (locals.var_f1_dn1 / (2.0 * assign3300_e3116)))) / (assign3300_e3117 * assign3300_e3117));
        locals.var_n0_dn3 = (((locals.var_f1_dn3 * assign3300_e3117) - (locals.var_f1 * (locals.var_f1_dn3 / (2.0 * assign3300_e3116)))) / (assign3300_e3117 * assign3300_e3117));
        locals.var_n0_dn4 = (((locals.var_f1_dn4 * assign3300_e3117) - (locals.var_f1 * (locals.var_f1_dn4 / (2.0 * assign3300_e3116)))) / (assign3300_e3117 * assign3300_e3117));
        locals.var_n0_dn5 = (((locals.var_f1_dn5 * assign3300_e3117) - (locals.var_f1 * (locals.var_f1_dn5 / (2.0 * assign3300_e3116)))) / (assign3300_e3117 * assign3300_e3117));
        locals.var_n0_dn6 = (((locals.var_f1_dn6 * assign3300_e3117) - (locals.var_f1 * (locals.var_f1_dn6 / (2.0 * assign3300_e3116)))) / (assign3300_e3117 * assign3300_e3117));
        locals.var_n0_dn7 = (((locals.var_f1_dn7 * assign3300_e3117) - (locals.var_f1 * (locals.var_f1_dn7 / (2.0 * assign3300_e3116)))) / (assign3300_e3117 * assign3300_e3117));
        locals.var_n0_dn8 = (((locals.var_f1_dn8 * assign3300_e3117) - (locals.var_f1 * (locals.var_f1_dn8 / (2.0 * assign3300_e3116)))) / (assign3300_e3117 * assign3300_e3117));
        locals.var_n0_dn9 = (((locals.var_f1_dn9 * assign3300_e3117) - (locals.var_f1 * (locals.var_f1_dn9 / (2.0 * assign3300_e3116)))) / (assign3300_e3117 * assign3300_e3117));
        locals.var_n0_dn10 = (((locals.var_f1_dn10 * assign3300_e3117) - (locals.var_f1 * (locals.var_f1_dn10 / (2.0 * assign3300_e3116)))) / (assign3300_e3117 * assign3300_e3117));
        locals.var_n0_rv = 0.0;

        let assign3310_e3122: f64 = (1.0 / locals.var_nfr_t);
        let assign3310_e3123: f64 = (locals.var_evb2c2star).powf(assign3310_e3122);
        locals.var_evb2c2star_nfr = assign3310_e3123;
        locals.var_evb2c2star_nfr_dn0 = if (-(locals.var_nfr_t_dn0 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3310_e3122) as f64).is_finite() && ((assign3310_e3122) as f64).fract() == 0.0 { if assign3310_e3122 == 0.0 { 0.0 } else { (assign3310_e3122 * ((locals.var_evb2c2star).powf(assign3310_e3122 - 1.0) * locals.var_evb2c2star_dn0)) } } else { (assign3310_e3123 * (((-(locals.var_nfr_t_dn0 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3310_e3122 * (locals.var_evb2c2star_dn0 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn1 = if (-(locals.var_nfr_t_dn1 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3310_e3122) as f64).is_finite() && ((assign3310_e3122) as f64).fract() == 0.0 { if assign3310_e3122 == 0.0 { 0.0 } else { (assign3310_e3122 * ((locals.var_evb2c2star).powf(assign3310_e3122 - 1.0) * locals.var_evb2c2star_dn1)) } } else { (assign3310_e3123 * (((-(locals.var_nfr_t_dn1 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3310_e3122 * (locals.var_evb2c2star_dn1 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn3 = if (-(locals.var_nfr_t_dn3 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3310_e3122) as f64).is_finite() && ((assign3310_e3122) as f64).fract() == 0.0 { if assign3310_e3122 == 0.0 { 0.0 } else { (assign3310_e3122 * ((locals.var_evb2c2star).powf(assign3310_e3122 - 1.0) * locals.var_evb2c2star_dn3)) } } else { (assign3310_e3123 * (((-(locals.var_nfr_t_dn3 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3310_e3122 * (locals.var_evb2c2star_dn3 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn4 = if (-(locals.var_nfr_t_dn4 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3310_e3122) as f64).is_finite() && ((assign3310_e3122) as f64).fract() == 0.0 { if assign3310_e3122 == 0.0 { 0.0 } else { (assign3310_e3122 * ((locals.var_evb2c2star).powf(assign3310_e3122 - 1.0) * locals.var_evb2c2star_dn4)) } } else { (assign3310_e3123 * (((-(locals.var_nfr_t_dn4 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3310_e3122 * (locals.var_evb2c2star_dn4 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn5 = if (-(locals.var_nfr_t_dn5 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3310_e3122) as f64).is_finite() && ((assign3310_e3122) as f64).fract() == 0.0 { if assign3310_e3122 == 0.0 { 0.0 } else { (assign3310_e3122 * ((locals.var_evb2c2star).powf(assign3310_e3122 - 1.0) * locals.var_evb2c2star_dn5)) } } else { (assign3310_e3123 * (((-(locals.var_nfr_t_dn5 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3310_e3122 * (locals.var_evb2c2star_dn5 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn6 = if (-(locals.var_nfr_t_dn6 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3310_e3122) as f64).is_finite() && ((assign3310_e3122) as f64).fract() == 0.0 { if assign3310_e3122 == 0.0 { 0.0 } else { (assign3310_e3122 * ((locals.var_evb2c2star).powf(assign3310_e3122 - 1.0) * locals.var_evb2c2star_dn6)) } } else { (assign3310_e3123 * (((-(locals.var_nfr_t_dn6 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3310_e3122 * (locals.var_evb2c2star_dn6 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn7 = if (-(locals.var_nfr_t_dn7 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3310_e3122) as f64).is_finite() && ((assign3310_e3122) as f64).fract() == 0.0 { if assign3310_e3122 == 0.0 { 0.0 } else { (assign3310_e3122 * ((locals.var_evb2c2star).powf(assign3310_e3122 - 1.0) * locals.var_evb2c2star_dn7)) } } else { (assign3310_e3123 * (((-(locals.var_nfr_t_dn7 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3310_e3122 * (locals.var_evb2c2star_dn7 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn8 = if (-(locals.var_nfr_t_dn8 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3310_e3122) as f64).is_finite() && ((assign3310_e3122) as f64).fract() == 0.0 { if assign3310_e3122 == 0.0 { 0.0 } else { (assign3310_e3122 * ((locals.var_evb2c2star).powf(assign3310_e3122 - 1.0) * locals.var_evb2c2star_dn8)) } } else { (assign3310_e3123 * (((-(locals.var_nfr_t_dn8 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3310_e3122 * (locals.var_evb2c2star_dn8 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn9 = if (-(locals.var_nfr_t_dn9 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3310_e3122) as f64).is_finite() && ((assign3310_e3122) as f64).fract() == 0.0 { if assign3310_e3122 == 0.0 { 0.0 } else { (assign3310_e3122 * ((locals.var_evb2c2star).powf(assign3310_e3122 - 1.0) * locals.var_evb2c2star_dn9)) } } else { (assign3310_e3123 * (((-(locals.var_nfr_t_dn9 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3310_e3122 * (locals.var_evb2c2star_dn9 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn10 = if (-(locals.var_nfr_t_dn10 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3310_e3122) as f64).is_finite() && ((assign3310_e3122) as f64).fract() == 0.0 { if assign3310_e3122 == 0.0 { 0.0 } else { (assign3310_e3122 * ((locals.var_evb2c2star).powf(assign3310_e3122 - 1.0) * locals.var_evb2c2star_dn10)) } } else { (assign3310_e3123 * (((-(locals.var_nfr_t_dn10 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3310_e3122 * (locals.var_evb2c2star_dn10 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_rv = 0.0;

        let assign3320_e3126: f64 = (locals.var_if0 * locals.var_evb2c2star_nfr);
        locals.var_f2 = assign3320_e3126;
        locals.var_f2_dn0 = ((locals.var_if0_dn0 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn0));
        locals.var_f2_dn1 = ((locals.var_if0_dn1 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn1));
        locals.var_f2_dn3 = ((locals.var_if0_dn3 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn3));
        locals.var_f2_dn4 = ((locals.var_if0_dn4 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn4));
        locals.var_f2_dn5 = ((locals.var_if0_dn5 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn5));
        locals.var_f2_dn6 = ((locals.var_if0_dn6 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn6));
        locals.var_f2_dn7 = ((locals.var_if0_dn7 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn7));
        locals.var_f2_dn8 = ((locals.var_if0_dn8 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn8));
        locals.var_f2_dn9 = ((locals.var_if0_dn9 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn9));
        locals.var_f2_dn10 = ((locals.var_if0_dn10 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn10));
        locals.var_f2_rv = 0.0;

        let assign3330_e3131: f64 = (1.0 + locals.var_f2);
        let assign3330_e3132: f64 = (assign3330_e3131).sqrt();
        let assign3330_e3133: f64 = (1.0 + assign3330_e3132);
        let assign3330_e3134: f64 = (locals.var_f2 / assign3330_e3133);
        locals.var_nb = assign3330_e3134;
        locals.var_nb_dn0 = (((locals.var_f2_dn0 * assign3330_e3133) - (locals.var_f2 * (locals.var_f2_dn0 / (2.0 * assign3330_e3132)))) / (assign3330_e3133 * assign3330_e3133));
        locals.var_nb_dn1 = (((locals.var_f2_dn1 * assign3330_e3133) - (locals.var_f2 * (locals.var_f2_dn1 / (2.0 * assign3330_e3132)))) / (assign3330_e3133 * assign3330_e3133));
        locals.var_nb_dn3 = (((locals.var_f2_dn3 * assign3330_e3133) - (locals.var_f2 * (locals.var_f2_dn3 / (2.0 * assign3330_e3132)))) / (assign3330_e3133 * assign3330_e3133));
        locals.var_nb_dn4 = (((locals.var_f2_dn4 * assign3330_e3133) - (locals.var_f2 * (locals.var_f2_dn4 / (2.0 * assign3330_e3132)))) / (assign3330_e3133 * assign3330_e3133));
        locals.var_nb_dn5 = (((locals.var_f2_dn5 * assign3330_e3133) - (locals.var_f2 * (locals.var_f2_dn5 / (2.0 * assign3330_e3132)))) / (assign3330_e3133 * assign3330_e3133));
        locals.var_nb_dn6 = (((locals.var_f2_dn6 * assign3330_e3133) - (locals.var_f2 * (locals.var_f2_dn6 / (2.0 * assign3330_e3132)))) / (assign3330_e3133 * assign3330_e3133));
        locals.var_nb_dn7 = (((locals.var_f2_dn7 * assign3330_e3133) - (locals.var_f2 * (locals.var_f2_dn7 / (2.0 * assign3330_e3132)))) / (assign3330_e3133 * assign3330_e3133));
        locals.var_nb_dn8 = (((locals.var_f2_dn8 * assign3330_e3133) - (locals.var_f2 * (locals.var_f2_dn8 / (2.0 * assign3330_e3132)))) / (assign3330_e3133 * assign3330_e3133));
        locals.var_nb_dn9 = (((locals.var_f2_dn9 * assign3330_e3133) - (locals.var_f2 * (locals.var_f2_dn9 / (2.0 * assign3330_e3132)))) / (assign3330_e3133 * assign3330_e3133));
        locals.var_nb_dn10 = (((locals.var_f2_dn10 * assign3330_e3133) - (locals.var_f2 * (locals.var_f2_dn10 / (2.0 * assign3330_e3132)))) / (assign3330_e3133 * assign3330_e3133));
        locals.var_nb_rv = 0.0;

        let assign3340_e3137: f64 = if p.p91 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard52 = assign3340_e3137;
        locals.var_guard52_rv = 0.0;

        let (assign3350_e3149, assign3350_e3149_d_n0, assign3350_e3149_d_n1, assign3350_e3149_d_n3, assign3350_e3149_d_n4, assign3350_e3149_d_n5, assign3350_e3149_d_n6, assign3350_e3149_d_n7, assign3350_e3149_d_n8, assign3350_e3149_d_n9, assign3350_e3149_d_n10,) = {
    if (locals.var_guard52 != 0.0) {
        let assign3350_e3142: f64 = (locals.var_vte / locals.var_ver_t);
        let assign3350_e3143: f64 = (1.0 + assign3350_e3142);
        let assign3350_e3146: f64 = (locals.var_vtc / locals.var_vef_t);
        let assign3350_e3147: f64 = (assign3350_e3143 + assign3350_e3146);
        (assign3350_e3147, ((((locals.var_vte_dn0 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn0)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn0 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn0)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn1 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn1)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn1 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn1)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn3 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn3)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn3 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn3)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn4 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn4)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn4 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn4)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn5 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn5)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn5 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn5)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn6 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn6)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn6 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn6)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn7 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn7)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn7 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn7)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn8 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn8)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn8 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn8)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn9 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn9)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn9 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn9)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn10 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn10)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn10 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn10)) / (locals.var_vef_t * locals.var_vef_t))),)
    } else {
        (locals.var_q0i, locals.var_q0i_dn0, locals.var_q0i_dn1, locals.var_q0i_dn3, locals.var_q0i_dn4, locals.var_q0i_dn5, locals.var_q0i_dn6, locals.var_q0i_dn7, locals.var_q0i_dn8, locals.var_q0i_dn9, locals.var_q0i_dn10,)
    }
};
        locals.var_q0i = assign3350_e3149;
        locals.var_q0i_dn0 = assign3350_e3149_d_n0;
        locals.var_q0i_dn1 = assign3350_e3149_d_n1;
        locals.var_q0i_dn3 = assign3350_e3149_d_n3;
        locals.var_q0i_dn4 = assign3350_e3149_d_n4;
        locals.var_q0i_dn5 = assign3350_e3149_d_n5;
        locals.var_q0i_dn6 = assign3350_e3149_d_n6;
        locals.var_q0i_dn7 = assign3350_e3149_d_n7;
        locals.var_q0i_dn8 = assign3350_e3149_d_n8;
        locals.var_q0i_dn9 = assign3350_e3149_d_n9;
        locals.var_q0i_dn10 = assign3350_e3149_d_n10;
        locals.var_q0i_rv = 0.0;

        let (assign3360_e3162, assign3360_e3162_d_n0, assign3360_e3162_d_n1, assign3360_e3162_d_n3, assign3360_e3162_d_n4, assign3360_e3162_d_n5, assign3360_e3162_d_n6, assign3360_e3162_d_n7, assign3360_e3162_d_n8, assign3360_e3162_d_n9, assign3360_e3162_d_n10,) = {
    if (locals.var_guard52 == 0.0) {
        let assign3360_e3154: f64 = (locals.var_vte / locals.var_ver_t);
        let assign3360_e3156: f64 = (assign3360_e3154 + 1.0);
        let assign3360_e3158: f64 = (assign3360_e3156 * locals.var_deg_t);
        let assign3360_e3160: f64 = (assign3360_e3158 * locals.var_vtinv);
        (assign3360_e3160, (((((locals.var_vte_dn0 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn0)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn1 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn1)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((((locals.var_vte_dn3 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn3)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) + (assign3360_e3156 * locals.var_deg_t_dn3)) * locals.var_vtinv) + (assign3360_e3158 * locals.var_vtinv_dn3)), (((((locals.var_vte_dn4 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn4)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn5 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn5)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn6 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn6)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn7 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn7)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn8 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn8)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn9 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn9)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn10 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn10)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv),)
    } else {
        (locals.var_terme, locals.var_terme_dn0, locals.var_terme_dn1, locals.var_terme_dn3, locals.var_terme_dn4, locals.var_terme_dn5, locals.var_terme_dn6, locals.var_terme_dn7, locals.var_terme_dn8, locals.var_terme_dn9, locals.var_terme_dn10,)
    }
};
        locals.var_terme = assign3360_e3162;
        locals.var_terme_dn0 = assign3360_e3162_d_n0;
        locals.var_terme_dn1 = assign3360_e3162_d_n1;
        locals.var_terme_dn3 = assign3360_e3162_d_n3;
        locals.var_terme_dn4 = assign3360_e3162_d_n4;
        locals.var_terme_dn5 = assign3360_e3162_d_n5;
        locals.var_terme_dn6 = assign3360_e3162_d_n6;
        locals.var_terme_dn7 = assign3360_e3162_d_n7;
        locals.var_terme_dn8 = assign3360_e3162_d_n8;
        locals.var_terme_dn9 = assign3360_e3162_d_n9;
        locals.var_terme_dn10 = assign3360_e3162_d_n10;
        locals.var_terme_rv = 0.0;

        let (assign3370_e3174, assign3370_e3174_d_n0, assign3370_e3174_d_n1, assign3370_e3174_d_n3, assign3370_e3174_d_n4, assign3370_e3174_d_n5, assign3370_e3174_d_n6, assign3370_e3174_d_n7, assign3370_e3174_d_n8, assign3370_e3174_d_n9, assign3370_e3174_d_n10,) = {
    if (locals.var_guard52 == 0.0) {
        let assign3370_e3166: f64 = (-locals.var_vtc);
        let assign3370_e3168: f64 = (assign3370_e3166 / locals.var_vef_t);
        let assign3370_e3170: f64 = (assign3370_e3168 * locals.var_deg_t);
        let assign3370_e3172: f64 = (assign3370_e3170 * locals.var_vtinv);
        (assign3370_e3172, ((((((-locals.var_vtc_dn0) * locals.var_vef_t) - (assign3370_e3166 * locals.var_vef_t_dn0)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn1) * locals.var_vef_t) - (assign3370_e3166 * locals.var_vef_t_dn1)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((((-locals.var_vtc_dn3) * locals.var_vef_t) - (assign3370_e3166 * locals.var_vef_t_dn3)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) + (assign3370_e3168 * locals.var_deg_t_dn3)) * locals.var_vtinv) + (assign3370_e3170 * locals.var_vtinv_dn3)), ((((((-locals.var_vtc_dn4) * locals.var_vef_t) - (assign3370_e3166 * locals.var_vef_t_dn4)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn5) * locals.var_vef_t) - (assign3370_e3166 * locals.var_vef_t_dn5)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn6) * locals.var_vef_t) - (assign3370_e3166 * locals.var_vef_t_dn6)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn7) * locals.var_vef_t) - (assign3370_e3166 * locals.var_vef_t_dn7)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn8) * locals.var_vef_t) - (assign3370_e3166 * locals.var_vef_t_dn8)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn9) * locals.var_vef_t) - (assign3370_e3166 * locals.var_vef_t_dn9)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn10) * locals.var_vef_t) - (assign3370_e3166 * locals.var_vef_t_dn10)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv),)
    } else {
        (locals.var_termc, locals.var_termc_dn0, locals.var_termc_dn1, locals.var_termc_dn3, locals.var_termc_dn4, locals.var_termc_dn5, locals.var_termc_dn6, locals.var_termc_dn7, locals.var_termc_dn8, locals.var_termc_dn9, locals.var_termc_dn10,)
    }
};
        locals.var_termc = assign3370_e3174;
        locals.var_termc_dn0 = assign3370_e3174_d_n0;
        locals.var_termc_dn1 = assign3370_e3174_d_n1;
        locals.var_termc_dn3 = assign3370_e3174_d_n3;
        locals.var_termc_dn4 = assign3370_e3174_d_n4;
        locals.var_termc_dn5 = assign3370_e3174_d_n5;
        locals.var_termc_dn6 = assign3370_e3174_d_n6;
        locals.var_termc_dn7 = assign3370_e3174_d_n7;
        locals.var_termc_dn8 = assign3370_e3174_d_n8;
        locals.var_termc_dn9 = assign3370_e3174_d_n9;
        locals.var_termc_dn10 = assign3370_e3174_d_n10;
        locals.var_termc_rv = 0.0;

        let (assign3380_e3190, assign3380_e3190_d_n0, assign3380_e3190_d_n1, assign3380_e3190_d_n3, assign3380_e3190_d_n4, assign3380_e3190_d_n5, assign3380_e3190_d_n6, assign3380_e3190_d_n7, assign3380_e3190_d_n8, assign3380_e3190_d_n9, assign3380_e3190_d_n10,) = {
    if (locals.var_guard52 == 0.0) {
        let assign3380_e3178: f64 = (locals.var_terme).exp();
        let assign3380_e3180: f64 = (locals.var_termc).exp();
        let assign3380_e3181: f64 = (assign3380_e3178 - assign3380_e3180);
        let assign3380_e3184: f64 = (locals.var_deg_t * locals.var_vtinv);
        let assign3380_e3185: f64 = (assign3380_e3184).exp();
        let assign3380_e3187: f64 = (assign3380_e3185 - 1.0);
        let assign3380_e3188: f64 = (assign3380_e3181 / assign3380_e3187);
        (assign3380_e3188, (((assign3380_e3178 * locals.var_terme_dn0) - (assign3380_e3180 * locals.var_termc_dn0)) / assign3380_e3187), (((assign3380_e3178 * locals.var_terme_dn1) - (assign3380_e3180 * locals.var_termc_dn1)) / assign3380_e3187), (((((assign3380_e3178 * locals.var_terme_dn3) - (assign3380_e3180 * locals.var_termc_dn3)) * assign3380_e3187) - (assign3380_e3181 * (assign3380_e3185 * ((locals.var_deg_t_dn3 * locals.var_vtinv) + (locals.var_deg_t * locals.var_vtinv_dn3))))) / (assign3380_e3187 * assign3380_e3187)), (((assign3380_e3178 * locals.var_terme_dn4) - (assign3380_e3180 * locals.var_termc_dn4)) / assign3380_e3187), (((assign3380_e3178 * locals.var_terme_dn5) - (assign3380_e3180 * locals.var_termc_dn5)) / assign3380_e3187), (((assign3380_e3178 * locals.var_terme_dn6) - (assign3380_e3180 * locals.var_termc_dn6)) / assign3380_e3187), (((assign3380_e3178 * locals.var_terme_dn7) - (assign3380_e3180 * locals.var_termc_dn7)) / assign3380_e3187), (((assign3380_e3178 * locals.var_terme_dn8) - (assign3380_e3180 * locals.var_termc_dn8)) / assign3380_e3187), (((assign3380_e3178 * locals.var_terme_dn9) - (assign3380_e3180 * locals.var_termc_dn9)) / assign3380_e3187), (((assign3380_e3178 * locals.var_terme_dn10) - (assign3380_e3180 * locals.var_termc_dn10)) / assign3380_e3187),)
    } else {
        (locals.var_q0i, locals.var_q0i_dn0, locals.var_q0i_dn1, locals.var_q0i_dn3, locals.var_q0i_dn4, locals.var_q0i_dn5, locals.var_q0i_dn6, locals.var_q0i_dn7, locals.var_q0i_dn8, locals.var_q0i_dn9, locals.var_q0i_dn10,)
    }
};
        locals.var_q0i = assign3380_e3190;
        locals.var_q0i_dn0 = assign3380_e3190_d_n0;
        locals.var_q0i_dn1 = assign3380_e3190_d_n1;
        locals.var_q0i_dn3 = assign3380_e3190_d_n3;
        locals.var_q0i_dn4 = assign3380_e3190_d_n4;
        locals.var_q0i_dn5 = assign3380_e3190_d_n5;
        locals.var_q0i_dn6 = assign3380_e3190_d_n6;
        locals.var_q0i_dn7 = assign3380_e3190_d_n7;
        locals.var_q0i_dn8 = assign3380_e3190_d_n8;
        locals.var_q0i_dn9 = assign3380_e3190_d_n9;
        locals.var_q0i_dn10 = assign3380_e3190_d_n10;
        locals.var_q0i_rv = 0.0;

        let assign3390_e3193: f64 = (0.1 * 0.1);
        locals.var_eps2 = assign3390_e3193;
        locals.var_eps2_dn0 = 0.0;
        locals.var_eps2_dn1 = 0.0;
        locals.var_eps2_dn3 = 0.0;
        locals.var_eps2_dn4 = 0.0;
        locals.var_eps2_dn5 = 0.0;
        locals.var_eps2_dn6 = 0.0;
        locals.var_eps2_dn7 = 0.0;
        locals.var_eps2_dn8 = 0.0;
        locals.var_eps2_dn9 = 0.0;
        locals.var_eps2_dn10 = 0.0;
        locals.var_eps2_rv = 0.0;

        let assign3400_e3196: f64 = (locals.var_q0i * locals.var_q0i);
        locals.var_x2 = assign3400_e3196;
        locals.var_x2_dn0 = ((locals.var_q0i_dn0 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn0));
        locals.var_x2_dn1 = ((locals.var_q0i_dn1 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn1));
        locals.var_x2_dn3 = ((locals.var_q0i_dn3 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn3));
        locals.var_x2_dn4 = ((locals.var_q0i_dn4 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn4));
        locals.var_x2_dn5 = ((locals.var_q0i_dn5 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn5));
        locals.var_x2_dn6 = ((locals.var_q0i_dn6 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn6));
        locals.var_x2_dn7 = ((locals.var_q0i_dn7 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn7));
        locals.var_x2_dn8 = ((locals.var_q0i_dn8 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn8));
        locals.var_x2_dn9 = ((locals.var_q0i_dn9 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn9));
        locals.var_x2_dn10 = ((locals.var_q0i_dn10 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn10));
        locals.var_x2_rv = 0.0;

        let assign3410_e3199: f64 = if locals.var_q0i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard53 = assign3410_e3199;
        locals.var_guard53_rv = 0.0;

        let (assign3420_e3212, assign3420_e3212_d_n0, assign3420_e3212_d_n1, assign3420_e3212_d_n3, assign3420_e3212_d_n4, assign3420_e3212_d_n5, assign3420_e3212_d_n6, assign3420_e3212_d_n7, assign3420_e3212_d_n8, assign3420_e3212_d_n9, assign3420_e3212_d_n10,) = {
    if (locals.var_guard53 != 0.0) {
        let assign3420_e3203: f64 = (0.5 * locals.var_eps2);
        let assign3420_e3206: f64 = (locals.var_x2 + locals.var_eps2);
        let assign3420_e3207: f64 = (assign3420_e3206).sqrt();
        let assign3420_e3209: f64 = (assign3420_e3207 - locals.var_q0i);
        let assign3420_e3210: f64 = (assign3420_e3203 / assign3420_e3209);
        (assign3420_e3210, ((((0.5 * locals.var_eps2_dn0) * assign3420_e3209) - (assign3420_e3203 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign3420_e3207)) - locals.var_q0i_dn0))) / (assign3420_e3209 * assign3420_e3209)), ((((0.5 * locals.var_eps2_dn1) * assign3420_e3209) - (assign3420_e3203 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign3420_e3207)) - locals.var_q0i_dn1))) / (assign3420_e3209 * assign3420_e3209)), ((((0.5 * locals.var_eps2_dn3) * assign3420_e3209) - (assign3420_e3203 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign3420_e3207)) - locals.var_q0i_dn3))) / (assign3420_e3209 * assign3420_e3209)), ((((0.5 * locals.var_eps2_dn4) * assign3420_e3209) - (assign3420_e3203 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign3420_e3207)) - locals.var_q0i_dn4))) / (assign3420_e3209 * assign3420_e3209)), ((((0.5 * locals.var_eps2_dn5) * assign3420_e3209) - (assign3420_e3203 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign3420_e3207)) - locals.var_q0i_dn5))) / (assign3420_e3209 * assign3420_e3209)), ((((0.5 * locals.var_eps2_dn6) * assign3420_e3209) - (assign3420_e3203 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign3420_e3207)) - locals.var_q0i_dn6))) / (assign3420_e3209 * assign3420_e3209)), ((((0.5 * locals.var_eps2_dn7) * assign3420_e3209) - (assign3420_e3203 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign3420_e3207)) - locals.var_q0i_dn7))) / (assign3420_e3209 * assign3420_e3209)), ((((0.5 * locals.var_eps2_dn8) * assign3420_e3209) - (assign3420_e3203 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign3420_e3207)) - locals.var_q0i_dn8))) / (assign3420_e3209 * assign3420_e3209)), ((((0.5 * locals.var_eps2_dn9) * assign3420_e3209) - (assign3420_e3203 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign3420_e3207)) - locals.var_q0i_dn9))) / (assign3420_e3209 * assign3420_e3209)), ((((0.5 * locals.var_eps2_dn10) * assign3420_e3209) - (assign3420_e3203 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign3420_e3207)) - locals.var_q0i_dn10))) / (assign3420_e3209 * assign3420_e3209)),)
    } else {
        (locals.var_q1i, locals.var_q1i_dn0, locals.var_q1i_dn1, locals.var_q1i_dn3, locals.var_q1i_dn4, locals.var_q1i_dn5, locals.var_q1i_dn6, locals.var_q1i_dn7, locals.var_q1i_dn8, locals.var_q1i_dn9, locals.var_q1i_dn10,)
    }
};
        locals.var_q1i = assign3420_e3212;
        locals.var_q1i_dn0 = assign3420_e3212_d_n0;
        locals.var_q1i_dn1 = assign3420_e3212_d_n1;
        locals.var_q1i_dn3 = assign3420_e3212_d_n3;
        locals.var_q1i_dn4 = assign3420_e3212_d_n4;
        locals.var_q1i_dn5 = assign3420_e3212_d_n5;
        locals.var_q1i_dn6 = assign3420_e3212_d_n6;
        locals.var_q1i_dn7 = assign3420_e3212_d_n7;
        locals.var_q1i_dn8 = assign3420_e3212_d_n8;
        locals.var_q1i_dn9 = assign3420_e3212_d_n9;
        locals.var_q1i_dn10 = assign3420_e3212_d_n10;
        locals.var_q1i_rv = 0.0;

        let (assign3430_e3224, assign3430_e3224_d_n0, assign3430_e3224_d_n1, assign3430_e3224_d_n3, assign3430_e3224_d_n4, assign3430_e3224_d_n5, assign3430_e3224_d_n6, assign3430_e3224_d_n7, assign3430_e3224_d_n8, assign3430_e3224_d_n9, assign3430_e3224_d_n10,) = {
    if (locals.var_guard53 == 0.0) {
        let assign3430_e3218: f64 = (locals.var_x2 + locals.var_eps2);
        let assign3430_e3219: f64 = (assign3430_e3218).sqrt();
        let assign3430_e3221: f64 = (assign3430_e3219 + locals.var_q0i);
        let assign3430_e3222: f64 = (0.5 * assign3430_e3221);
        (assign3430_e3222, (0.5 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign3430_e3219)) + locals.var_q0i_dn0)), (0.5 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign3430_e3219)) + locals.var_q0i_dn1)), (0.5 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign3430_e3219)) + locals.var_q0i_dn3)), (0.5 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign3430_e3219)) + locals.var_q0i_dn4)), (0.5 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign3430_e3219)) + locals.var_q0i_dn5)), (0.5 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign3430_e3219)) + locals.var_q0i_dn6)), (0.5 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign3430_e3219)) + locals.var_q0i_dn7)), (0.5 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign3430_e3219)) + locals.var_q0i_dn8)), (0.5 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign3430_e3219)) + locals.var_q0i_dn9)), (0.5 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign3430_e3219)) + locals.var_q0i_dn10)),)
    } else {
        (locals.var_q1i, locals.var_q1i_dn0, locals.var_q1i_dn1, locals.var_q1i_dn3, locals.var_q1i_dn4, locals.var_q1i_dn5, locals.var_q1i_dn6, locals.var_q1i_dn7, locals.var_q1i_dn8, locals.var_q1i_dn9, locals.var_q1i_dn10,)
    }
};
        locals.var_q1i = assign3430_e3224;
        locals.var_q1i_dn0 = assign3430_e3224_d_n0;
        locals.var_q1i_dn1 = assign3430_e3224_d_n1;
        locals.var_q1i_dn3 = assign3430_e3224_d_n3;
        locals.var_q1i_dn4 = assign3430_e3224_d_n4;
        locals.var_q1i_dn5 = assign3430_e3224_d_n5;
        locals.var_q1i_dn6 = assign3430_e3224_d_n6;
        locals.var_q1i_dn7 = assign3430_e3224_d_n7;
        locals.var_q1i_dn8 = assign3430_e3224_d_n8;
        locals.var_q1i_dn9 = assign3430_e3224_d_n9;
        locals.var_q1i_dn10 = assign3430_e3224_d_n10;
        locals.var_q1i_rv = 0.0;

        let assign3440_e3230: f64 = (locals.var_n0 + locals.var_nb);
        let assign3440_e3231: f64 = (0.5 * assign3440_e3230);
        let assign3440_e3232: f64 = (1.0 + assign3440_e3231);
        let assign3440_e3233: f64 = (locals.var_q1i * assign3440_e3232);
        locals.var_qbi = assign3440_e3233;
        locals.var_qbi_dn0 = ((locals.var_q1i_dn0 * assign3440_e3232) + (locals.var_q1i * (0.5 * (locals.var_n0_dn0 + locals.var_nb_dn0))));
        locals.var_qbi_dn1 = ((locals.var_q1i_dn1 * assign3440_e3232) + (locals.var_q1i * (0.5 * (locals.var_n0_dn1 + locals.var_nb_dn1))));
        locals.var_qbi_dn3 = ((locals.var_q1i_dn3 * assign3440_e3232) + (locals.var_q1i * (0.5 * (locals.var_n0_dn3 + locals.var_nb_dn3))));
        locals.var_qbi_dn4 = ((locals.var_q1i_dn4 * assign3440_e3232) + (locals.var_q1i * (0.5 * (locals.var_n0_dn4 + locals.var_nb_dn4))));
        locals.var_qbi_dn5 = ((locals.var_q1i_dn5 * assign3440_e3232) + (locals.var_q1i * (0.5 * (locals.var_n0_dn5 + locals.var_nb_dn5))));
        locals.var_qbi_dn6 = ((locals.var_q1i_dn6 * assign3440_e3232) + (locals.var_q1i * (0.5 * (locals.var_n0_dn6 + locals.var_nb_dn6))));
        locals.var_qbi_dn7 = ((locals.var_q1i_dn7 * assign3440_e3232) + (locals.var_q1i * (0.5 * (locals.var_n0_dn7 + locals.var_nb_dn7))));
        locals.var_qbi_dn8 = ((locals.var_q1i_dn8 * assign3440_e3232) + (locals.var_q1i * (0.5 * (locals.var_n0_dn8 + locals.var_nb_dn8))));
        locals.var_qbi_dn9 = ((locals.var_q1i_dn9 * assign3440_e3232) + (locals.var_q1i * (0.5 * (locals.var_n0_dn9 + locals.var_nb_dn9))));
        locals.var_qbi_dn10 = ((locals.var_q1i_dn10 * assign3440_e3232) + (locals.var_q1i * (0.5 * (locals.var_n0_dn10 + locals.var_nb_dn10))));
        locals.var_qbi_rv = 0.0;

        let assign3450_e3236: f64 = (p.p14 * locals.var_is_t);
        let assign3450_e3238: f64 = (assign3450_e3236 * locals.var_evb2c2star_nfr);
        locals.var_ir = assign3450_e3238;
        locals.var_ir_dn0 = (((p.p14 * locals.var_is_t_dn0) * locals.var_evb2c2star_nfr) + (assign3450_e3236 * locals.var_evb2c2star_nfr_dn0));
        locals.var_ir_dn1 = (((p.p14 * locals.var_is_t_dn1) * locals.var_evb2c2star_nfr) + (assign3450_e3236 * locals.var_evb2c2star_nfr_dn1));
        locals.var_ir_dn3 = (((p.p14 * locals.var_is_t_dn3) * locals.var_evb2c2star_nfr) + (assign3450_e3236 * locals.var_evb2c2star_nfr_dn3));
        locals.var_ir_dn4 = (((p.p14 * locals.var_is_t_dn4) * locals.var_evb2c2star_nfr) + (assign3450_e3236 * locals.var_evb2c2star_nfr_dn4));
        locals.var_ir_dn5 = (((p.p14 * locals.var_is_t_dn5) * locals.var_evb2c2star_nfr) + (assign3450_e3236 * locals.var_evb2c2star_nfr_dn5));
        locals.var_ir_dn6 = (((p.p14 * locals.var_is_t_dn6) * locals.var_evb2c2star_nfr) + (assign3450_e3236 * locals.var_evb2c2star_nfr_dn6));
        locals.var_ir_dn7 = (((p.p14 * locals.var_is_t_dn7) * locals.var_evb2c2star_nfr) + (assign3450_e3236 * locals.var_evb2c2star_nfr_dn7));
        locals.var_ir_dn8 = (((p.p14 * locals.var_is_t_dn8) * locals.var_evb2c2star_nfr) + (assign3450_e3236 * locals.var_evb2c2star_nfr_dn8));
        locals.var_ir_dn9 = (((p.p14 * locals.var_is_t_dn9) * locals.var_evb2c2star_nfr) + (assign3450_e3236 * locals.var_evb2c2star_nfr_dn9));
        locals.var_ir_dn10 = (((p.p14 * locals.var_is_t_dn10) * locals.var_evb2c2star_nfr) + (assign3450_e3236 * locals.var_evb2c2star_nfr_dn10));
        locals.var_ir_rv = 0.0;

        let assign3460_e3241: f64 = (locals.var_is_t * locals.var_evb2e1);
        locals.var_if_ = assign3460_e3241;
        locals.var_if__dn0 = ((locals.var_is_t_dn0 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn0));
        locals.var_if__dn1 = ((locals.var_is_t_dn1 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn1));
        locals.var_if__dn3 = ((locals.var_is_t_dn3 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn3));
        locals.var_if__dn4 = ((locals.var_is_t_dn4 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn4));
        locals.var_if__dn5 = ((locals.var_is_t_dn5 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn5));
        locals.var_if__dn6 = ((locals.var_is_t_dn6 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn6));
        locals.var_if__dn7 = ((locals.var_is_t_dn7 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn7));
        locals.var_if__dn8 = ((locals.var_is_t_dn8 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn8));
        locals.var_if__dn9 = ((locals.var_is_t_dn9 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn9));
        locals.var_if__dn10 = ((locals.var_is_t_dn10 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn10));
        locals.var_if__rv = 0.0;

        let assign3470_e3244: f64 = (locals.var_if_ - locals.var_ir);
        let assign3470_e3246: f64 = (assign3470_e3244 / locals.var_qbi);
        locals.var_in_ = assign3470_e3246;
        locals.var_in__dn0 = ((((locals.var_if__dn0 - locals.var_ir_dn0) * locals.var_qbi) - (assign3470_e3244 * locals.var_qbi_dn0)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn1 = ((((locals.var_if__dn1 - locals.var_ir_dn1) * locals.var_qbi) - (assign3470_e3244 * locals.var_qbi_dn1)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn3 = ((((locals.var_if__dn3 - locals.var_ir_dn3) * locals.var_qbi) - (assign3470_e3244 * locals.var_qbi_dn3)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn4 = ((((locals.var_if__dn4 - locals.var_ir_dn4) * locals.var_qbi) - (assign3470_e3244 * locals.var_qbi_dn4)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn5 = ((((locals.var_if__dn5 - locals.var_ir_dn5) * locals.var_qbi) - (assign3470_e3244 * locals.var_qbi_dn5)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn6 = ((((locals.var_if__dn6 - locals.var_ir_dn6) * locals.var_qbi) - (assign3470_e3244 * locals.var_qbi_dn6)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn7 = ((((locals.var_if__dn7 - locals.var_ir_dn7) * locals.var_qbi) - (assign3470_e3244 * locals.var_qbi_dn7)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn8 = ((((locals.var_if__dn8 - locals.var_ir_dn8) * locals.var_qbi) - (assign3470_e3244 * locals.var_qbi_dn8)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn9 = ((((locals.var_if__dn9 - locals.var_ir_dn9) * locals.var_qbi) - (assign3470_e3244 * locals.var_qbi_dn9)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn10 = ((((locals.var_if__dn10 - locals.var_ir_dn10) * locals.var_qbi) - (assign3470_e3244 * locals.var_qbi_dn10)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__rv = 0.0;

        let assign3480_e3249: f64 = locals.var_vb2e1;
        let assign3480_e3251: f64 = (assign3480_e3249 / 0.0001);
        locals.var_dxa = assign3480_e3251;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = (locals.var_vb2e1_dn4 / 0.0001);
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = (locals.var_vb2e1_dn6 / 0.0001);
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign3490_e3254: f64 = if locals.var_vb2e1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard54 = assign3490_e3254;
        locals.var_guard54_rv = 0.0;

        let (assign3500_e3266, assign3500_e3266_d_n0, assign3500_e3266_d_n1, assign3500_e3266_d_n3, assign3500_e3266_d_n4, assign3500_e3266_d_n5, assign3500_e3266_d_n6, assign3500_e3266_d_n7, assign3500_e3266_d_n8, assign3500_e3266_d_n9, assign3500_e3266_d_n10,) = {
    if (locals.var_guard54 != 0.0) {
        let assign3500_e3260: f64 = (locals.var_dxa).exp();
        let assign3500_e3261: f64 = (1.0 + assign3500_e3260);
        let assign3500_e3262: f64 = (assign3500_e3261).ln();
        let assign3500_e3263: f64 = (0.0001 * assign3500_e3262);
        let assign3500_e3264: f64 = assign3500_e3263;
        (assign3500_e3264, (0.0001 * ((assign3500_e3260 * locals.var_dxa_dn0) / assign3500_e3261)), (0.0001 * ((assign3500_e3260 * locals.var_dxa_dn1) / assign3500_e3261)), (0.0001 * ((assign3500_e3260 * locals.var_dxa_dn3) / assign3500_e3261)), (0.0001 * ((assign3500_e3260 * locals.var_dxa_dn4) / assign3500_e3261)), (0.0001 * ((assign3500_e3260 * locals.var_dxa_dn5) / assign3500_e3261)), (0.0001 * ((assign3500_e3260 * locals.var_dxa_dn6) / assign3500_e3261)), (0.0001 * ((assign3500_e3260 * locals.var_dxa_dn7) / assign3500_e3261)), (0.0001 * ((assign3500_e3260 * locals.var_dxa_dn8) / assign3500_e3261)), (0.0001 * ((assign3500_e3260 * locals.var_dxa_dn9) / assign3500_e3261)), (0.0001 * ((assign3500_e3260 * locals.var_dxa_dn10) / assign3500_e3261)),)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign3500_e3266;
        locals.var_tmpexp_dn0 = assign3500_e3266_d_n0;
        locals.var_tmpexp_dn1 = assign3500_e3266_d_n1;
        locals.var_tmpexp_dn3 = assign3500_e3266_d_n3;
        locals.var_tmpexp_dn4 = assign3500_e3266_d_n4;
        locals.var_tmpexp_dn5 = assign3500_e3266_d_n5;
        locals.var_tmpexp_dn6 = assign3500_e3266_d_n6;
        locals.var_tmpexp_dn7 = assign3500_e3266_d_n7;
        locals.var_tmpexp_dn8 = assign3500_e3266_d_n8;
        locals.var_tmpexp_dn9 = assign3500_e3266_d_n9;
        locals.var_tmpexp_dn10 = assign3500_e3266_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let (assign3510_e3280, assign3510_e3280_d_n0, assign3510_e3280_d_n1, assign3510_e3280_d_n3, assign3510_e3280_d_n4, assign3510_e3280_d_n5, assign3510_e3280_d_n6, assign3510_e3280_d_n7, assign3510_e3280_d_n8, assign3510_e3280_d_n9, assign3510_e3280_d_n10,) = {
    if (locals.var_guard54 == 0.0) {
        let assign3510_e3273: f64 = (-locals.var_dxa);
        let assign3510_e3274: f64 = (assign3510_e3273).exp();
        let assign3510_e3275: f64 = (1.0 + assign3510_e3274);
        let assign3510_e3276: f64 = (assign3510_e3275).ln();
        let assign3510_e3277: f64 = (0.0001 * assign3510_e3276);
        let assign3510_e3278: f64 = (locals.var_vb2e1 + assign3510_e3277);
        (assign3510_e3278, (0.0001 * ((assign3510_e3274 * (-locals.var_dxa_dn0)) / assign3510_e3275)), (0.0001 * ((assign3510_e3274 * (-locals.var_dxa_dn1)) / assign3510_e3275)), (0.0001 * ((assign3510_e3274 * (-locals.var_dxa_dn3)) / assign3510_e3275)), (locals.var_vb2e1_dn4 + (0.0001 * ((assign3510_e3274 * (-locals.var_dxa_dn4)) / assign3510_e3275))), (0.0001 * ((assign3510_e3274 * (-locals.var_dxa_dn5)) / assign3510_e3275)), (locals.var_vb2e1_dn6 + (0.0001 * ((assign3510_e3274 * (-locals.var_dxa_dn6)) / assign3510_e3275))), (0.0001 * ((assign3510_e3274 * (-locals.var_dxa_dn7)) / assign3510_e3275)), (0.0001 * ((assign3510_e3274 * (-locals.var_dxa_dn8)) / assign3510_e3275)), (0.0001 * ((assign3510_e3274 * (-locals.var_dxa_dn9)) / assign3510_e3275)), (0.0001 * ((assign3510_e3274 * (-locals.var_dxa_dn10)) / assign3510_e3275)),)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign3510_e3280;
        locals.var_tmpexp_dn0 = assign3510_e3280_d_n0;
        locals.var_tmpexp_dn1 = assign3510_e3280_d_n1;
        locals.var_tmpexp_dn3 = assign3510_e3280_d_n3;
        locals.var_tmpexp_dn4 = assign3510_e3280_d_n4;
        locals.var_tmpexp_dn5 = assign3510_e3280_d_n5;
        locals.var_tmpexp_dn6 = assign3510_e3280_d_n6;
        locals.var_tmpexp_dn7 = assign3510_e3280_d_n7;
        locals.var_tmpexp_dn8 = assign3510_e3280_d_n8;
        locals.var_tmpexp_dn9 = assign3510_e3280_d_n9;
        locals.var_tmpexp_dn10 = assign3510_e3280_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let assign3520_e3283: f64 = (locals.var_tmpexp / p.p143);
        locals.var_tmpexp1 = assign3520_e3283;
        locals.var_tmpexp1_dn0 = (locals.var_tmpexp_dn0 / p.p143);
        locals.var_tmpexp1_dn1 = (locals.var_tmpexp_dn1 / p.p143);
        locals.var_tmpexp1_dn3 = (locals.var_tmpexp_dn3 / p.p143);
        locals.var_tmpexp1_dn4 = (locals.var_tmpexp_dn4 / p.p143);
        locals.var_tmpexp1_dn5 = (locals.var_tmpexp_dn5 / p.p143);
        locals.var_tmpexp1_dn6 = (locals.var_tmpexp_dn6 / p.p143);
        locals.var_tmpexp1_dn7 = (locals.var_tmpexp_dn7 / p.p143);
        locals.var_tmpexp1_dn8 = (locals.var_tmpexp_dn8 / p.p143);
        locals.var_tmpexp1_dn9 = (locals.var_tmpexp_dn9 / p.p143);
        locals.var_tmpexp1_dn10 = (locals.var_tmpexp_dn10 / p.p143);
        locals.var_tmpexp1_rv = 0.0;

        let assign3530_e3286: f64 = if locals.var_tmpexp1 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard55 = assign3530_e3286;
        locals.var_guard55_rv = 0.0;

        let (assign3550_e3297,) = {
    if (locals.var_guard55 == 0.0) {
        let assign3550_e3295: f64 = (p.p138).exp();
        (assign3550_e3295,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3550_e3297;
        locals.var_expl_rv = 0.0;

        let assign3580_e3316: f64 = (locals.var_vb2e1 - p.p145);
        let assign3580_e3318: f64 = (assign3580_e3316 / 0.001);
        locals.var_dxa = assign3580_e3318;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = (locals.var_vb2e1_dn4 / 0.001);
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = (locals.var_vb2e1_dn6 / 0.001);
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign3630_e3359: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3630_e3361: f64 = (assign3630_e3359 / p.p16);
        let assign3630_e3363: f64 = if assign3630_e3361 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard57 = assign3630_e3363;
        locals.var_guard57_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3640_e3372, assign3640_e3372_d_n0, assign3640_e3372_d_n1, assign3640_e3372_d_n3, assign3640_e3372_d_n4, assign3640_e3372_d_n5, assign3640_e3372_d_n6, assign3640_e3372_d_n7, assign3640_e3372_d_n8, assign3640_e3372_d_n9, assign3640_e3372_d_n10,) = {
    if (locals.var_guard57 != 0.0) {
        let assign3640_e3367: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3640_e3369: f64 = (assign3640_e3367 / p.p16);
        let assign3640_e3370: f64 = (assign3640_e3369).exp();
        (assign3640_e3370, 0.0, 0.0, (assign3640_e3370 * ((locals.var_vb2e1 * locals.var_vtinv_dn3) / p.p16)), (assign3640_e3370 * ((locals.var_vb2e1_dn4 * locals.var_vtinv) / p.p16)), 0.0, (assign3640_e3370 * ((locals.var_vb2e1_dn6 * locals.var_vtinv) / p.p16)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign3640_e3372;
        locals.var_tmpexp_dn0 = assign3640_e3372_d_n0;
        locals.var_tmpexp_dn1 = assign3640_e3372_d_n1;
        locals.var_tmpexp_dn3 = assign3640_e3372_d_n3;
        locals.var_tmpexp_dn4 = assign3640_e3372_d_n4;
        locals.var_tmpexp_dn5 = assign3640_e3372_d_n5;
        locals.var_tmpexp_dn6 = assign3640_e3372_d_n6;
        locals.var_tmpexp_dn7 = assign3640_e3372_d_n7;
        locals.var_tmpexp_dn8 = assign3640_e3372_d_n8;
        locals.var_tmpexp_dn9 = assign3640_e3372_d_n9;
        locals.var_tmpexp_dn10 = assign3640_e3372_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let (assign3650_e3378,) = {
    if (locals.var_guard57 == 0.0) {
        let assign3650_e3376: f64 = (p.p138).exp();
        (assign3650_e3376,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3650_e3378;
        locals.var_expl_rv = 0.0;

        let (assign3660_e3393, assign3660_e3393_d_n0, assign3660_e3393_d_n1, assign3660_e3393_d_n3, assign3660_e3393_d_n4, assign3660_e3393_d_n5, assign3660_e3393_d_n6, assign3660_e3393_d_n7, assign3660_e3393_d_n8, assign3660_e3393_d_n9, assign3660_e3393_d_n10,) = {
    if (locals.var_guard57 == 0.0) {
        let assign3660_e3385: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3660_e3387: f64 = (assign3660_e3385 / p.p16);
        let assign3660_e3389: f64 = (assign3660_e3387 - p.p138);
        let assign3660_e3390: f64 = (1.0 + assign3660_e3389);
        let assign3660_e3391: f64 = (locals.var_expl * assign3660_e3390);
        (assign3660_e3391, 0.0, 0.0, (locals.var_expl * ((locals.var_vb2e1 * locals.var_vtinv_dn3) / p.p16)), (locals.var_expl * ((locals.var_vb2e1_dn4 * locals.var_vtinv) / p.p16)), 0.0, (locals.var_expl * ((locals.var_vb2e1_dn6 * locals.var_vtinv) / p.p16)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign3660_e3393;
        locals.var_tmpexp_dn0 = assign3660_e3393_d_n0;
        locals.var_tmpexp_dn1 = assign3660_e3393_d_n1;
        locals.var_tmpexp_dn3 = assign3660_e3393_d_n3;
        locals.var_tmpexp_dn4 = assign3660_e3393_d_n4;
        locals.var_tmpexp_dn5 = assign3660_e3393_d_n5;
        locals.var_tmpexp_dn6 = assign3660_e3393_d_n6;
        locals.var_tmpexp_dn7 = assign3660_e3393_d_n7;
        locals.var_tmpexp_dn8 = assign3660_e3393_d_n8;
        locals.var_tmpexp_dn9 = assign3660_e3393_d_n9;
        locals.var_tmpexp_dn10 = assign3660_e3393_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let assign3670_e3396: f64 = if p.p23 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard58 = assign3670_e3396;
        locals.var_guard58_rv = 0.0;

        let assign3680_e3399: f64 = (locals.var_vb2e1 - locals.var_vknbr_t);
        let assign3680_e3401: f64 = (assign3680_e3399 * locals.var_vtinv);
        let assign3680_e3403: f64 = if assign3680_e3401 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard59 = assign3680_e3403;
        locals.var_guard59_rv = 0.0;

        let (assign3690_e3414, assign3690_e3414_d_n0, assign3690_e3414_d_n1, assign3690_e3414_d_n3, assign3690_e3414_d_n4, assign3690_e3414_d_n5, assign3690_e3414_d_n6, assign3690_e3414_d_n7, assign3690_e3414_d_n8, assign3690_e3414_d_n9, assign3690_e3414_d_n10,) = {
    if ((locals.var_guard58 != 0.0) && (locals.var_guard59 != 0.0)) {
        let assign3690_e3409: f64 = (locals.var_vb2e1 - locals.var_vknbr_t);
        let assign3690_e3411: f64 = (assign3690_e3409 * locals.var_vtinv);
        let assign3690_e3412: f64 = (assign3690_e3411).exp();
        (assign3690_e3412, (assign3690_e3412 * ((-locals.var_vknbr_t_dn0) * locals.var_vtinv)), (assign3690_e3412 * ((-locals.var_vknbr_t_dn1) * locals.var_vtinv)), (assign3690_e3412 * (((-locals.var_vknbr_t_dn3) * locals.var_vtinv) + (assign3690_e3409 * locals.var_vtinv_dn3))), (assign3690_e3412 * ((locals.var_vb2e1_dn4 - locals.var_vknbr_t_dn4) * locals.var_vtinv)), (assign3690_e3412 * ((-locals.var_vknbr_t_dn5) * locals.var_vtinv)), (assign3690_e3412 * ((locals.var_vb2e1_dn6 - locals.var_vknbr_t_dn6) * locals.var_vtinv)), (assign3690_e3412 * ((-locals.var_vknbr_t_dn7) * locals.var_vtinv)), (assign3690_e3412 * ((-locals.var_vknbr_t_dn8) * locals.var_vtinv)), (assign3690_e3412 * ((-locals.var_vknbr_t_dn9) * locals.var_vtinv)), (assign3690_e3412 * ((-locals.var_vknbr_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_tmpexp1, locals.var_tmpexp1_dn0, locals.var_tmpexp1_dn1, locals.var_tmpexp1_dn3, locals.var_tmpexp1_dn4, locals.var_tmpexp1_dn5, locals.var_tmpexp1_dn6, locals.var_tmpexp1_dn7, locals.var_tmpexp1_dn8, locals.var_tmpexp1_dn9, locals.var_tmpexp1_dn10,)
    }
};
        locals.var_tmpexp1 = assign3690_e3414;
        locals.var_tmpexp1_dn0 = assign3690_e3414_d_n0;
        locals.var_tmpexp1_dn1 = assign3690_e3414_d_n1;
        locals.var_tmpexp1_dn3 = assign3690_e3414_d_n3;
        locals.var_tmpexp1_dn4 = assign3690_e3414_d_n4;
        locals.var_tmpexp1_dn5 = assign3690_e3414_d_n5;
        locals.var_tmpexp1_dn6 = assign3690_e3414_d_n6;
        locals.var_tmpexp1_dn7 = assign3690_e3414_d_n7;
        locals.var_tmpexp1_dn8 = assign3690_e3414_d_n8;
        locals.var_tmpexp1_dn9 = assign3690_e3414_d_n9;
        locals.var_tmpexp1_dn10 = assign3690_e3414_d_n10;
        locals.var_tmpexp1_rv = 0.0;

        let (assign3700_e3422,) = {
    if ((locals.var_guard58 != 0.0) && (locals.var_guard59 == 0.0)) {
        let assign3700_e3420: f64 = (p.p138).exp();
        (assign3700_e3420,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3700_e3422;
        locals.var_expl_rv = 0.0;

        let (assign3710_e3439, assign3710_e3439_d_n0, assign3710_e3439_d_n1, assign3710_e3439_d_n3, assign3710_e3439_d_n4, assign3710_e3439_d_n5, assign3710_e3439_d_n6, assign3710_e3439_d_n7, assign3710_e3439_d_n8, assign3710_e3439_d_n9, assign3710_e3439_d_n10,) = {
    if ((locals.var_guard58 != 0.0) && (locals.var_guard59 == 0.0)) {
        let assign3710_e3431: f64 = (locals.var_vb2e1 - locals.var_vknbr_t);
        let assign3710_e3433: f64 = (assign3710_e3431 * locals.var_vtinv);
        let assign3710_e3435: f64 = (assign3710_e3433 - p.p138);
        let assign3710_e3436: f64 = (1.0 + assign3710_e3435);
        let assign3710_e3437: f64 = (locals.var_expl * assign3710_e3436);
        (assign3710_e3437, (locals.var_expl * ((-locals.var_vknbr_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn1) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vknbr_t_dn3) * locals.var_vtinv) + (assign3710_e3431 * locals.var_vtinv_dn3))), (locals.var_expl * ((locals.var_vb2e1_dn4 - locals.var_vknbr_t_dn4) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2e1_dn6 - locals.var_vknbr_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_tmpexp1, locals.var_tmpexp1_dn0, locals.var_tmpexp1_dn1, locals.var_tmpexp1_dn3, locals.var_tmpexp1_dn4, locals.var_tmpexp1_dn5, locals.var_tmpexp1_dn6, locals.var_tmpexp1_dn7, locals.var_tmpexp1_dn8, locals.var_tmpexp1_dn9, locals.var_tmpexp1_dn10,)
    }
};
        locals.var_tmpexp1 = assign3710_e3439;
        locals.var_tmpexp1_dn0 = assign3710_e3439_d_n0;
        locals.var_tmpexp1_dn1 = assign3710_e3439_d_n1;
        locals.var_tmpexp1_dn3 = assign3710_e3439_d_n3;
        locals.var_tmpexp1_dn4 = assign3710_e3439_d_n4;
        locals.var_tmpexp1_dn5 = assign3710_e3439_d_n5;
        locals.var_tmpexp1_dn6 = assign3710_e3439_d_n6;
        locals.var_tmpexp1_dn7 = assign3710_e3439_d_n7;
        locals.var_tmpexp1_dn8 = assign3710_e3439_d_n8;
        locals.var_tmpexp1_dn9 = assign3710_e3439_d_n9;
        locals.var_tmpexp1_dn10 = assign3710_e3439_d_n10;
        locals.var_tmpexp1_rv = 0.0;

        let assign3720_e3442: f64 = (locals.var_in_ / locals.var_is_t);
        let assign3720_e3444: f64 = (assign3720_e3442 - 1000.0);
        let assign3720_e3446: f64 = if assign3720_e3444 < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard60 = assign3720_e3446;
        locals.var_guard60_rv = 0.0;

        let (assign3740_e3465,) = {
    if ((locals.var_guard58 != 0.0) && (locals.var_guard60 == 0.0)) {
        let assign3740_e3463: f64 = (40.0_f64).exp();
        (assign3740_e3463,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3740_e3465;
        locals.var_expl_rv = 0.0;

        let assign3800_e3572: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign3800_e3574: f64 = (assign3800_e3572 / p.p18);
        let assign3800_e3576: f64 = if assign3800_e3574 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard62 = assign3800_e3576;
        locals.var_guard62_rv = 0.0;

        let (assign3810_e3585, assign3810_e3585_d_n0, assign3810_e3585_d_n1, assign3810_e3585_d_n3, assign3810_e3585_d_n4, assign3810_e3585_d_n5, assign3810_e3585_d_n6, assign3810_e3585_d_n7, assign3810_e3585_d_n8, assign3810_e3585_d_n9, assign3810_e3585_d_n10,) = {
    if (locals.var_guard62 != 0.0) {
        let assign3810_e3580: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign3810_e3582: f64 = (assign3810_e3580 / p.p18);
        let assign3810_e3583: f64 = (assign3810_e3582).exp();
        (assign3810_e3583, 0.0, 0.0, (assign3810_e3583 * ((locals.var_vb1e1 * locals.var_vtinv_dn3) / p.p18)), (assign3810_e3583 * ((locals.var_vb1e1_dn4 * locals.var_vtinv) / p.p18)), (assign3810_e3583 * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p18)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign3810_e3585;
        locals.var_tmpexp_dn0 = assign3810_e3585_d_n0;
        locals.var_tmpexp_dn1 = assign3810_e3585_d_n1;
        locals.var_tmpexp_dn3 = assign3810_e3585_d_n3;
        locals.var_tmpexp_dn4 = assign3810_e3585_d_n4;
        locals.var_tmpexp_dn5 = assign3810_e3585_d_n5;
        locals.var_tmpexp_dn6 = assign3810_e3585_d_n6;
        locals.var_tmpexp_dn7 = assign3810_e3585_d_n7;
        locals.var_tmpexp_dn8 = assign3810_e3585_d_n8;
        locals.var_tmpexp_dn9 = assign3810_e3585_d_n9;
        locals.var_tmpexp_dn10 = assign3810_e3585_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let (assign3820_e3591,) = {
    if (locals.var_guard62 == 0.0) {
        let assign3820_e3589: f64 = (p.p138).exp();
        (assign3820_e3589,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3820_e3591;
        locals.var_expl_rv = 0.0;

        let (assign3830_e3606, assign3830_e3606_d_n0, assign3830_e3606_d_n1, assign3830_e3606_d_n3, assign3830_e3606_d_n4, assign3830_e3606_d_n5, assign3830_e3606_d_n6, assign3830_e3606_d_n7, assign3830_e3606_d_n8, assign3830_e3606_d_n9, assign3830_e3606_d_n10,) = {
    if (locals.var_guard62 == 0.0) {
        let assign3830_e3598: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign3830_e3600: f64 = (assign3830_e3598 / p.p18);
        let assign3830_e3602: f64 = (assign3830_e3600 - p.p138);
        let assign3830_e3603: f64 = (1.0 + assign3830_e3602);
        let assign3830_e3604: f64 = (locals.var_expl * assign3830_e3603);
        (assign3830_e3604, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1e1 * locals.var_vtinv_dn3) / p.p18)), (locals.var_expl * ((locals.var_vb1e1_dn4 * locals.var_vtinv) / p.p18)), (locals.var_expl * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p18)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign3830_e3606;
        locals.var_tmpexp_dn0 = assign3830_e3606_d_n0;
        locals.var_tmpexp_dn1 = assign3830_e3606_d_n1;
        locals.var_tmpexp_dn3 = assign3830_e3606_d_n3;
        locals.var_tmpexp_dn4 = assign3830_e3606_d_n4;
        locals.var_tmpexp_dn5 = assign3830_e3606_d_n5;
        locals.var_tmpexp_dn6 = assign3830_e3606_d_n6;
        locals.var_tmpexp_dn7 = assign3830_e3606_d_n7;
        locals.var_tmpexp_dn8 = assign3830_e3606_d_n8;
        locals.var_tmpexp_dn9 = assign3830_e3606_d_n9;
        locals.var_tmpexp_dn10 = assign3830_e3606_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let assign3840_e3609: f64 = if p.p23 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard63 = assign3840_e3609;
        locals.var_guard63_rv = 0.0;

        let assign3850_e3612: f64 = (locals.var_vb1e1 - locals.var_vknbr_t);
        let assign3850_e3614: f64 = (assign3850_e3612 * locals.var_vtinv);
        let assign3850_e3616: f64 = if assign3850_e3614 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard64 = assign3850_e3616;
        locals.var_guard64_rv = 0.0;

        let (assign3860_e3627, assign3860_e3627_d_n0, assign3860_e3627_d_n1, assign3860_e3627_d_n3, assign3860_e3627_d_n4, assign3860_e3627_d_n5, assign3860_e3627_d_n6, assign3860_e3627_d_n7, assign3860_e3627_d_n8, assign3860_e3627_d_n9, assign3860_e3627_d_n10,) = {
    if ((locals.var_guard63 != 0.0) && (locals.var_guard64 != 0.0)) {
        let assign3860_e3622: f64 = (locals.var_vb1e1 - locals.var_vknbr_t);
        let assign3860_e3624: f64 = (assign3860_e3622 * locals.var_vtinv);
        let assign3860_e3625: f64 = (assign3860_e3624).exp();
        (assign3860_e3625, (assign3860_e3625 * ((-locals.var_vknbr_t_dn0) * locals.var_vtinv)), (assign3860_e3625 * ((-locals.var_vknbr_t_dn1) * locals.var_vtinv)), (assign3860_e3625 * (((-locals.var_vknbr_t_dn3) * locals.var_vtinv) + (assign3860_e3622 * locals.var_vtinv_dn3))), (assign3860_e3625 * ((locals.var_vb1e1_dn4 - locals.var_vknbr_t_dn4) * locals.var_vtinv)), (assign3860_e3625 * ((locals.var_vb1e1_dn5 - locals.var_vknbr_t_dn5) * locals.var_vtinv)), (assign3860_e3625 * ((-locals.var_vknbr_t_dn6) * locals.var_vtinv)), (assign3860_e3625 * ((-locals.var_vknbr_t_dn7) * locals.var_vtinv)), (assign3860_e3625 * ((-locals.var_vknbr_t_dn8) * locals.var_vtinv)), (assign3860_e3625 * ((-locals.var_vknbr_t_dn9) * locals.var_vtinv)), (assign3860_e3625 * ((-locals.var_vknbr_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_tmpexp1, locals.var_tmpexp1_dn0, locals.var_tmpexp1_dn1, locals.var_tmpexp1_dn3, locals.var_tmpexp1_dn4, locals.var_tmpexp1_dn5, locals.var_tmpexp1_dn6, locals.var_tmpexp1_dn7, locals.var_tmpexp1_dn8, locals.var_tmpexp1_dn9, locals.var_tmpexp1_dn10,)
    }
};
        locals.var_tmpexp1 = assign3860_e3627;
        locals.var_tmpexp1_dn0 = assign3860_e3627_d_n0;
        locals.var_tmpexp1_dn1 = assign3860_e3627_d_n1;
        locals.var_tmpexp1_dn3 = assign3860_e3627_d_n3;
        locals.var_tmpexp1_dn4 = assign3860_e3627_d_n4;
        locals.var_tmpexp1_dn5 = assign3860_e3627_d_n5;
        locals.var_tmpexp1_dn6 = assign3860_e3627_d_n6;
        locals.var_tmpexp1_dn7 = assign3860_e3627_d_n7;
        locals.var_tmpexp1_dn8 = assign3860_e3627_d_n8;
        locals.var_tmpexp1_dn9 = assign3860_e3627_d_n9;
        locals.var_tmpexp1_dn10 = assign3860_e3627_d_n10;
        locals.var_tmpexp1_rv = 0.0;

        let (assign3870_e3635,) = {
    if ((locals.var_guard63 != 0.0) && (locals.var_guard64 == 0.0)) {
        let assign3870_e3633: f64 = (p.p138).exp();
        (assign3870_e3633,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3870_e3635;
        locals.var_expl_rv = 0.0;

        let (assign3880_e3652, assign3880_e3652_d_n0, assign3880_e3652_d_n1, assign3880_e3652_d_n3, assign3880_e3652_d_n4, assign3880_e3652_d_n5, assign3880_e3652_d_n6, assign3880_e3652_d_n7, assign3880_e3652_d_n8, assign3880_e3652_d_n9, assign3880_e3652_d_n10,) = {
    if ((locals.var_guard63 != 0.0) && (locals.var_guard64 == 0.0)) {
        let assign3880_e3644: f64 = (locals.var_vb1e1 - locals.var_vknbr_t);
        let assign3880_e3646: f64 = (assign3880_e3644 * locals.var_vtinv);
        let assign3880_e3648: f64 = (assign3880_e3646 - p.p138);
        let assign3880_e3649: f64 = (1.0 + assign3880_e3648);
        let assign3880_e3650: f64 = (locals.var_expl * assign3880_e3649);
        (assign3880_e3650, (locals.var_expl * ((-locals.var_vknbr_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn1) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vknbr_t_dn3) * locals.var_vtinv) + (assign3880_e3644 * locals.var_vtinv_dn3))), (locals.var_expl * ((locals.var_vb1e1_dn4 - locals.var_vknbr_t_dn4) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1e1_dn5 - locals.var_vknbr_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_tmpexp1, locals.var_tmpexp1_dn0, locals.var_tmpexp1_dn1, locals.var_tmpexp1_dn3, locals.var_tmpexp1_dn4, locals.var_tmpexp1_dn5, locals.var_tmpexp1_dn6, locals.var_tmpexp1_dn7, locals.var_tmpexp1_dn8, locals.var_tmpexp1_dn9, locals.var_tmpexp1_dn10,)
    }
};
        locals.var_tmpexp1 = assign3880_e3652;
        locals.var_tmpexp1_dn0 = assign3880_e3652_d_n0;
        locals.var_tmpexp1_dn1 = assign3880_e3652_d_n1;
        locals.var_tmpexp1_dn3 = assign3880_e3652_d_n3;
        locals.var_tmpexp1_dn4 = assign3880_e3652_d_n4;
        locals.var_tmpexp1_dn5 = assign3880_e3652_d_n5;
        locals.var_tmpexp1_dn6 = assign3880_e3652_d_n6;
        locals.var_tmpexp1_dn7 = assign3880_e3652_d_n7;
        locals.var_tmpexp1_dn8 = assign3880_e3652_d_n8;
        locals.var_tmpexp1_dn9 = assign3880_e3652_d_n9;
        locals.var_tmpexp1_dn10 = assign3880_e3652_d_n10;
        locals.var_tmpexp1_rv = 0.0;

        let assign3910_e3689: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3910_e3691: f64 = (assign3910_e3689 / p.p20);
        let assign3910_e3693: f64 = if assign3910_e3691 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard65 = assign3910_e3693;
        locals.var_guard65_rv = 0.0;

        let (assign3920_e3702, assign3920_e3702_d_n0, assign3920_e3702_d_n1, assign3920_e3702_d_n3, assign3920_e3702_d_n4, assign3920_e3702_d_n5, assign3920_e3702_d_n6, assign3920_e3702_d_n7, assign3920_e3702_d_n8, assign3920_e3702_d_n9, assign3920_e3702_d_n10,) = {
    if (locals.var_guard65 != 0.0) {
        let assign3920_e3697: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3920_e3699: f64 = (assign3920_e3697 / p.p20);
        let assign3920_e3700: f64 = (assign3920_e3699).exp();
        (assign3920_e3700, 0.0, 0.0, (assign3920_e3700 * ((locals.var_vb2e1 * locals.var_vtinv_dn3) / p.p20)), (assign3920_e3700 * ((locals.var_vb2e1_dn4 * locals.var_vtinv) / p.p20)), 0.0, (assign3920_e3700 * ((locals.var_vb2e1_dn6 * locals.var_vtinv) / p.p20)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign3920_e3702;
        locals.var_tmpexp_dn0 = assign3920_e3702_d_n0;
        locals.var_tmpexp_dn1 = assign3920_e3702_d_n1;
        locals.var_tmpexp_dn3 = assign3920_e3702_d_n3;
        locals.var_tmpexp_dn4 = assign3920_e3702_d_n4;
        locals.var_tmpexp_dn5 = assign3920_e3702_d_n5;
        locals.var_tmpexp_dn6 = assign3920_e3702_d_n6;
        locals.var_tmpexp_dn7 = assign3920_e3702_d_n7;
        locals.var_tmpexp_dn8 = assign3920_e3702_d_n8;
        locals.var_tmpexp_dn9 = assign3920_e3702_d_n9;
        locals.var_tmpexp_dn10 = assign3920_e3702_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let (assign3930_e3708,) = {
    if (locals.var_guard65 == 0.0) {
        let assign3930_e3706: f64 = (p.p138).exp();
        (assign3930_e3706,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3930_e3708;
        locals.var_expl_rv = 0.0;

        let (assign3940_e3723, assign3940_e3723_d_n0, assign3940_e3723_d_n1, assign3940_e3723_d_n3, assign3940_e3723_d_n4, assign3940_e3723_d_n5, assign3940_e3723_d_n6, assign3940_e3723_d_n7, assign3940_e3723_d_n8, assign3940_e3723_d_n9, assign3940_e3723_d_n10,) = {
    if (locals.var_guard65 == 0.0) {
        let assign3940_e3715: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3940_e3717: f64 = (assign3940_e3715 / p.p20);
        let assign3940_e3719: f64 = (assign3940_e3717 - p.p138);
        let assign3940_e3720: f64 = (1.0 + assign3940_e3719);
        let assign3940_e3721: f64 = (locals.var_expl * assign3940_e3720);
        (assign3940_e3721, 0.0, 0.0, (locals.var_expl * ((locals.var_vb2e1 * locals.var_vtinv_dn3) / p.p20)), (locals.var_expl * ((locals.var_vb2e1_dn4 * locals.var_vtinv) / p.p20)), 0.0, (locals.var_expl * ((locals.var_vb2e1_dn6 * locals.var_vtinv) / p.p20)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign3940_e3723;
        locals.var_tmpexp_dn0 = assign3940_e3723_d_n0;
        locals.var_tmpexp_dn1 = assign3940_e3723_d_n1;
        locals.var_tmpexp_dn3 = assign3940_e3723_d_n3;
        locals.var_tmpexp_dn4 = assign3940_e3723_d_n4;
        locals.var_tmpexp_dn5 = assign3940_e3723_d_n5;
        locals.var_tmpexp_dn6 = assign3940_e3723_d_n6;
        locals.var_tmpexp_dn7 = assign3940_e3723_d_n7;
        locals.var_tmpexp_dn8 = assign3940_e3723_d_n8;
        locals.var_tmpexp_dn9 = assign3940_e3723_d_n9;
        locals.var_tmpexp_dn10 = assign3940_e3723_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let assign3960_e3731: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign3960_e3733: f64 = (assign3960_e3731 / p.p22);
        let assign3960_e3735: f64 = if assign3960_e3733 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard66 = assign3960_e3735;
        locals.var_guard66_rv = 0.0;

        let (assign3970_e3744, assign3970_e3744_d_n0, assign3970_e3744_d_n1, assign3970_e3744_d_n3, assign3970_e3744_d_n4, assign3970_e3744_d_n5, assign3970_e3744_d_n6, assign3970_e3744_d_n7, assign3970_e3744_d_n8, assign3970_e3744_d_n9, assign3970_e3744_d_n10,) = {
    if (locals.var_guard66 != 0.0) {
        let assign3970_e3739: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign3970_e3741: f64 = (assign3970_e3739 / p.p22);
        let assign3970_e3742: f64 = (assign3970_e3741).exp();
        (assign3970_e3742, 0.0, 0.0, (assign3970_e3742 * ((locals.var_vb1e1 * locals.var_vtinv_dn3) / p.p22)), (assign3970_e3742 * ((locals.var_vb1e1_dn4 * locals.var_vtinv) / p.p22)), (assign3970_e3742 * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p22)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign3970_e3744;
        locals.var_tmpexp_dn0 = assign3970_e3744_d_n0;
        locals.var_tmpexp_dn1 = assign3970_e3744_d_n1;
        locals.var_tmpexp_dn3 = assign3970_e3744_d_n3;
        locals.var_tmpexp_dn4 = assign3970_e3744_d_n4;
        locals.var_tmpexp_dn5 = assign3970_e3744_d_n5;
        locals.var_tmpexp_dn6 = assign3970_e3744_d_n6;
        locals.var_tmpexp_dn7 = assign3970_e3744_d_n7;
        locals.var_tmpexp_dn8 = assign3970_e3744_d_n8;
        locals.var_tmpexp_dn9 = assign3970_e3744_d_n9;
        locals.var_tmpexp_dn10 = assign3970_e3744_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let (assign3980_e3750,) = {
    if (locals.var_guard66 == 0.0) {
        let assign3980_e3748: f64 = (p.p138).exp();
        (assign3980_e3748,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3980_e3750;
        locals.var_expl_rv = 0.0;

        let (assign3990_e3765, assign3990_e3765_d_n0, assign3990_e3765_d_n1, assign3990_e3765_d_n3, assign3990_e3765_d_n4, assign3990_e3765_d_n5, assign3990_e3765_d_n6, assign3990_e3765_d_n7, assign3990_e3765_d_n8, assign3990_e3765_d_n9, assign3990_e3765_d_n10,) = {
    if (locals.var_guard66 == 0.0) {
        let assign3990_e3757: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign3990_e3759: f64 = (assign3990_e3757 / p.p22);
        let assign3990_e3761: f64 = (assign3990_e3759 - p.p138);
        let assign3990_e3762: f64 = (1.0 + assign3990_e3761);
        let assign3990_e3763: f64 = (locals.var_expl * assign3990_e3762);
        (assign3990_e3763, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1e1 * locals.var_vtinv_dn3) / p.p22)), (locals.var_expl * ((locals.var_vb1e1_dn4 * locals.var_vtinv) / p.p22)), (locals.var_expl * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p22)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign3990_e3765;
        locals.var_tmpexp_dn0 = assign3990_e3765_d_n0;
        locals.var_tmpexp_dn1 = assign3990_e3765_d_n1;
        locals.var_tmpexp_dn3 = assign3990_e3765_d_n3;
        locals.var_tmpexp_dn4 = assign3990_e3765_d_n4;
        locals.var_tmpexp_dn5 = assign3990_e3765_d_n5;
        locals.var_tmpexp_dn6 = assign3990_e3765_d_n6;
        locals.var_tmpexp_dn7 = assign3990_e3765_d_n7;
        locals.var_tmpexp_dn8 = assign3990_e3765_d_n8;
        locals.var_tmpexp_dn9 = assign3990_e3765_d_n9;
        locals.var_tmpexp_dn10 = assign3990_e3765_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let assign4010_e3773: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign4010_e3775: f64 = (assign4010_e3773 / p.p31);
        let assign4010_e3777: f64 = if assign4010_e3775 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard67 = assign4010_e3777;
        locals.var_guard67_rv = 0.0;

        let (assign4020_e3786, assign4020_e3786_d_n0, assign4020_e3786_d_n1, assign4020_e3786_d_n3, assign4020_e3786_d_n4, assign4020_e3786_d_n5, assign4020_e3786_d_n6, assign4020_e3786_d_n7, assign4020_e3786_d_n8, assign4020_e3786_d_n9, assign4020_e3786_d_n10,) = {
    if (locals.var_guard67 != 0.0) {
        let assign4020_e3781: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign4020_e3783: f64 = (assign4020_e3781 / p.p31);
        let assign4020_e3784: f64 = (assign4020_e3783).exp();
        (assign4020_e3784, 0.0, 0.0, (assign4020_e3784 * ((locals.var_vb1c4 * locals.var_vtinv_dn3) / p.p31)), 0.0, (assign4020_e3784 * ((locals.var_vb1c4_dn5 * locals.var_vtinv) / p.p31)), (assign4020_e3784 * ((locals.var_vb1c4_dn6 * locals.var_vtinv) / p.p31)), (assign4020_e3784 * ((locals.var_vb1c4_dn7 * locals.var_vtinv) / p.p31)), (assign4020_e3784 * ((locals.var_vb1c4_dn8 * locals.var_vtinv) / p.p31)), 0.0, (assign4020_e3784 * ((locals.var_vb1c4_dn10 * locals.var_vtinv) / p.p31)),)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign4020_e3786;
        locals.var_tmpexp_dn0 = assign4020_e3786_d_n0;
        locals.var_tmpexp_dn1 = assign4020_e3786_d_n1;
        locals.var_tmpexp_dn3 = assign4020_e3786_d_n3;
        locals.var_tmpexp_dn4 = assign4020_e3786_d_n4;
        locals.var_tmpexp_dn5 = assign4020_e3786_d_n5;
        locals.var_tmpexp_dn6 = assign4020_e3786_d_n6;
        locals.var_tmpexp_dn7 = assign4020_e3786_d_n7;
        locals.var_tmpexp_dn8 = assign4020_e3786_d_n8;
        locals.var_tmpexp_dn9 = assign4020_e3786_d_n9;
        locals.var_tmpexp_dn10 = assign4020_e3786_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let (assign4030_e3792,) = {
    if (locals.var_guard67 == 0.0) {
        let assign4030_e3790: f64 = (p.p138).exp();
        (assign4030_e3790,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4030_e3792;
        locals.var_expl_rv = 0.0;

        let (assign4040_e3807, assign4040_e3807_d_n0, assign4040_e3807_d_n1, assign4040_e3807_d_n3, assign4040_e3807_d_n4, assign4040_e3807_d_n5, assign4040_e3807_d_n6, assign4040_e3807_d_n7, assign4040_e3807_d_n8, assign4040_e3807_d_n9, assign4040_e3807_d_n10,) = {
    if (locals.var_guard67 == 0.0) {
        let assign4040_e3799: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign4040_e3801: f64 = (assign4040_e3799 / p.p31);
        let assign4040_e3803: f64 = (assign4040_e3801 - p.p138);
        let assign4040_e3804: f64 = (1.0 + assign4040_e3803);
        let assign4040_e3805: f64 = (locals.var_expl * assign4040_e3804);
        (assign4040_e3805, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1c4 * locals.var_vtinv_dn3) / p.p31)), 0.0, (locals.var_expl * ((locals.var_vb1c4_dn5 * locals.var_vtinv) / p.p31)), (locals.var_expl * ((locals.var_vb1c4_dn6 * locals.var_vtinv) / p.p31)), (locals.var_expl * ((locals.var_vb1c4_dn7 * locals.var_vtinv) / p.p31)), (locals.var_expl * ((locals.var_vb1c4_dn8 * locals.var_vtinv) / p.p31)), 0.0, (locals.var_expl * ((locals.var_vb1c4_dn10 * locals.var_vtinv) / p.p31)),)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign4040_e3807;
        locals.var_tmpexp_dn0 = assign4040_e3807_d_n0;
        locals.var_tmpexp_dn1 = assign4040_e3807_d_n1;
        locals.var_tmpexp_dn3 = assign4040_e3807_d_n3;
        locals.var_tmpexp_dn4 = assign4040_e3807_d_n4;
        locals.var_tmpexp_dn5 = assign4040_e3807_d_n5;
        locals.var_tmpexp_dn6 = assign4040_e3807_d_n6;
        locals.var_tmpexp_dn7 = assign4040_e3807_d_n7;
        locals.var_tmpexp_dn8 = assign4040_e3807_d_n8;
        locals.var_tmpexp_dn9 = assign4040_e3807_d_n9;
        locals.var_tmpexp_dn10 = assign4040_e3807_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let assign4060_e3815: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4060_e3817: f64 = (assign4060_e3815 / p.p137);
        let assign4060_e3819: f64 = if assign4060_e3817 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard68 = assign4060_e3819;
        locals.var_guard68_rv = 0.0;

        let (assign4070_e3828, assign4070_e3828_d_n0, assign4070_e3828_d_n1, assign4070_e3828_d_n3, assign4070_e3828_d_n4, assign4070_e3828_d_n5, assign4070_e3828_d_n6, assign4070_e3828_d_n7, assign4070_e3828_d_n8, assign4070_e3828_d_n9, assign4070_e3828_d_n10,) = {
    if (locals.var_guard68 != 0.0) {
        let assign4070_e3823: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4070_e3825: f64 = (assign4070_e3823 / p.p137);
        let assign4070_e3826: f64 = (assign4070_e3825).exp();
        (assign4070_e3826, 0.0, 0.0, (assign4070_e3826 * ((locals.var_vb1e1 * locals.var_vtinv_dn3) / p.p137)), (assign4070_e3826 * ((locals.var_vb1e1_dn4 * locals.var_vtinv) / p.p137)), (assign4070_e3826 * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p137)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign4070_e3828;
        locals.var_tmpexp_dn0 = assign4070_e3828_d_n0;
        locals.var_tmpexp_dn1 = assign4070_e3828_d_n1;
        locals.var_tmpexp_dn3 = assign4070_e3828_d_n3;
        locals.var_tmpexp_dn4 = assign4070_e3828_d_n4;
        locals.var_tmpexp_dn5 = assign4070_e3828_d_n5;
        locals.var_tmpexp_dn6 = assign4070_e3828_d_n6;
        locals.var_tmpexp_dn7 = assign4070_e3828_d_n7;
        locals.var_tmpexp_dn8 = assign4070_e3828_d_n8;
        locals.var_tmpexp_dn9 = assign4070_e3828_d_n9;
        locals.var_tmpexp_dn10 = assign4070_e3828_d_n10;
        locals.var_tmpexp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4080_e3834,) = {
    if (locals.var_guard68 == 0.0) {
        let assign4080_e3832: f64 = (p.p138).exp();
        (assign4080_e3832,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4080_e3834;
        locals.var_expl_rv = 0.0;

        let (assign4090_e3849, assign4090_e3849_d_n0, assign4090_e3849_d_n1, assign4090_e3849_d_n3, assign4090_e3849_d_n4, assign4090_e3849_d_n5, assign4090_e3849_d_n6, assign4090_e3849_d_n7, assign4090_e3849_d_n8, assign4090_e3849_d_n9, assign4090_e3849_d_n10,) = {
    if (locals.var_guard68 == 0.0) {
        let assign4090_e3841: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4090_e3843: f64 = (assign4090_e3841 / p.p137);
        let assign4090_e3845: f64 = (assign4090_e3843 - p.p138);
        let assign4090_e3846: f64 = (1.0 + assign4090_e3845);
        let assign4090_e3847: f64 = (locals.var_expl * assign4090_e3846);
        (assign4090_e3847, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1e1 * locals.var_vtinv_dn3) / p.p137)), (locals.var_expl * ((locals.var_vb1e1_dn4 * locals.var_vtinv) / p.p137)), (locals.var_expl * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p137)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign4090_e3849;
        locals.var_tmpexp_dn0 = assign4090_e3849_d_n0;
        locals.var_tmpexp_dn1 = assign4090_e3849_d_n1;
        locals.var_tmpexp_dn3 = assign4090_e3849_d_n3;
        locals.var_tmpexp_dn4 = assign4090_e3849_d_n4;
        locals.var_tmpexp_dn5 = assign4090_e3849_d_n5;
        locals.var_tmpexp_dn6 = assign4090_e3849_d_n6;
        locals.var_tmpexp_dn7 = assign4090_e3849_d_n7;
        locals.var_tmpexp_dn8 = assign4090_e3849_d_n8;
        locals.var_tmpexp_dn9 = assign4090_e3849_d_n9;
        locals.var_tmpexp_dn10 = assign4090_e3849_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let assign4110_e3865: f64 = if (((p.p33 > 0.0) && (p.p34 > 0.0)) && (locals.var_vb2e1 < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard69 = assign4110_e3865;
        locals.var_guard69_rv = 0.0;

        let assign4120_e3871: f64 = (2.0 * locals.var_e0eb);
        let assign4120_e3872: f64 = (locals.var_pow2_2m_pe / assign4120_e3871);
        let assign4120_e3873: f64 = (1.0 - assign4120_e3872);
        let assign4120_e3874: f64 = (locals.var_nzeb_t * assign4120_e3873);
        let assign4120_e3876: f64 = if assign4120_e3874 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign4120_e3876;
        locals.var_guard70_rv = 0.0;

        let (assign4140_e3899,) = {
    if ((locals.var_guard69 != 0.0) && (locals.var_guard70 == 0.0)) {
        let assign4140_e3897: f64 = (p.p138).exp();
        (assign4140_e3897,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4140_e3899;
        locals.var_expl_rv = 0.0;

        let (assign4160_e3926, assign4160_e3926_d_n0, assign4160_e3926_d_n1, assign4160_e3926_d_n3, assign4160_e3926_d_n4, assign4160_e3926_d_n5, assign4160_e3926_d_n6, assign4160_e3926_d_n7, assign4160_e3926_d_n8, assign4160_e3926_d_n9, assign4160_e3926_d_n10,) = {
    if (locals.var_guard69 != 0.0) {
        let assign4160_e3924: f64 = (locals.var_vb2e1 * locals.var_inv_vde_t);
        (assign4160_e3924, (locals.var_vb2e1 * locals.var_inv_vde_t_dn0), (locals.var_vb2e1 * locals.var_inv_vde_t_dn1), (locals.var_vb2e1 * locals.var_inv_vde_t_dn3), ((locals.var_vb2e1_dn4 * locals.var_inv_vde_t) + (locals.var_vb2e1 * locals.var_inv_vde_t_dn4)), (locals.var_vb2e1 * locals.var_inv_vde_t_dn5), ((locals.var_vb2e1_dn6 * locals.var_inv_vde_t) + (locals.var_vb2e1 * locals.var_inv_vde_t_dn6)), (locals.var_vb2e1 * locals.var_inv_vde_t_dn7), (locals.var_vb2e1 * locals.var_inv_vde_t_dn8), (locals.var_vb2e1 * locals.var_inv_vde_t_dn9), (locals.var_vb2e1 * locals.var_inv_vde_t_dn10),)
    } else {
        (locals.var_x, locals.var_x_dn0, locals.var_x_dn1, locals.var_x_dn3, locals.var_x_dn4, locals.var_x_dn5, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8, locals.var_x_dn9, locals.var_x_dn10,)
    }
};
        locals.var_x = assign4160_e3926;
        locals.var_x_dn0 = assign4160_e3926_d_n0;
        locals.var_x_dn1 = assign4160_e3926_d_n1;
        locals.var_x_dn3 = assign4160_e3926_d_n3;
        locals.var_x_dn4 = assign4160_e3926_d_n4;
        locals.var_x_dn5 = assign4160_e3926_d_n5;
        locals.var_x_dn6 = assign4160_e3926_d_n6;
        locals.var_x_dn7 = assign4160_e3926_d_n7;
        locals.var_x_dn8 = assign4160_e3926_d_n8;
        locals.var_x_dn9 = assign4160_e3926_d_n9;
        locals.var_x_dn10 = assign4160_e3926_d_n10;
        locals.var_x_rv = 0.0;

        let (assign4170_e3970, assign4170_e3970_d_n0, assign4170_e3970_d_n1, assign4170_e3970_d_n3, assign4170_e3970_d_n4, assign4170_e3970_d_n5, assign4170_e3970_d_n6, assign4170_e3970_d_n7, assign4170_e3970_d_n8, assign4170_e3970_d_n9, assign4170_e3970_d_n10,) = {
    if (locals.var_guard69 != 0.0) {
        let assign4170_e3930: f64 = (locals.var_x * locals.var_x);
        let assign4170_e3932: f64 = (assign4170_e3930 + 1e-30);
        let assign4170_e3933: f64 = (assign4170_e3932).sqrt();
        let assign4170_e3935: f64 = (-2.0);
        let assign4170_e3937: f64 = (assign4170_e3935 - p.p66);
        let assign4170_e3938: f64 = (assign4170_e3933).powf(assign4170_e3937);
        let assign4170_e3943: f64 = (p.p66 * p.p66);
        let assign4170_e3944: f64 = (1.0 - assign4170_e3943);
        let assign4170_e3947: f64 = (3.0 * locals.var_x);
        let assign4170_e3950: f64 = (p.p66 - 1.0);
        let assign4170_e3951: f64 = (assign4170_e3947 * assign4170_e3950);
        let assign4170_e3952: f64 = (assign4170_e3944 - assign4170_e3951);
        let assign4170_e3953: f64 = (p.p66 * assign4170_e3952);
        let assign4170_e3956: f64 = (6.0 * locals.var_x);
        let assign4170_e3958: f64 = (assign4170_e3956 * locals.var_x);
        let assign4170_e3961: f64 = (p.p66 - 1.0);
        let assign4170_e3963: f64 = (assign4170_e3961 + locals.var_x);
        let assign4170_e3964: f64 = (assign4170_e3958 * assign4170_e3963);
        let assign4170_e3965: f64 = (assign4170_e3953 - assign4170_e3964);
        let assign4170_e3966: f64 = (assign4170_e3938 * assign4170_e3965);
        let assign4170_e3968: f64 = (assign4170_e3966 * 0.16666666666666666);
        (assign4170_e3968, (((if 0.0 == 0.0 && ((assign4170_e3937) as f64).is_finite() && ((assign4170_e3937) as f64).fract() == 0.0 { if assign4170_e3937 == 0.0 { 0.0 } else { (assign4170_e3937 * ((assign4170_e3933).powf(assign4170_e3937 - 1.0) * (((locals.var_x_dn0 * locals.var_x) + (locals.var_x * locals.var_x_dn0)) / (2.0 * assign4170_e3933)))) } } else { (assign4170_e3938 * (assign4170_e3937 * ((((locals.var_x_dn0 * locals.var_x) + (locals.var_x * locals.var_x_dn0)) / (2.0 * assign4170_e3933)) / assign4170_e3933))) } * assign4170_e3965) + (assign4170_e3938 * ((p.p66 * (-((3.0 * locals.var_x_dn0) * assign4170_e3950))) - (((((6.0 * locals.var_x_dn0) * locals.var_x) + (assign4170_e3956 * locals.var_x_dn0)) * assign4170_e3963) + (assign4170_e3958 * locals.var_x_dn0))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4170_e3937) as f64).is_finite() && ((assign4170_e3937) as f64).fract() == 0.0 { if assign4170_e3937 == 0.0 { 0.0 } else { (assign4170_e3937 * ((assign4170_e3933).powf(assign4170_e3937 - 1.0) * (((locals.var_x_dn1 * locals.var_x) + (locals.var_x * locals.var_x_dn1)) / (2.0 * assign4170_e3933)))) } } else { (assign4170_e3938 * (assign4170_e3937 * ((((locals.var_x_dn1 * locals.var_x) + (locals.var_x * locals.var_x_dn1)) / (2.0 * assign4170_e3933)) / assign4170_e3933))) } * assign4170_e3965) + (assign4170_e3938 * ((p.p66 * (-((3.0 * locals.var_x_dn1) * assign4170_e3950))) - (((((6.0 * locals.var_x_dn1) * locals.var_x) + (assign4170_e3956 * locals.var_x_dn1)) * assign4170_e3963) + (assign4170_e3958 * locals.var_x_dn1))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4170_e3937) as f64).is_finite() && ((assign4170_e3937) as f64).fract() == 0.0 { if assign4170_e3937 == 0.0 { 0.0 } else { (assign4170_e3937 * ((assign4170_e3933).powf(assign4170_e3937 - 1.0) * (((locals.var_x_dn3 * locals.var_x) + (locals.var_x * locals.var_x_dn3)) / (2.0 * assign4170_e3933)))) } } else { (assign4170_e3938 * (assign4170_e3937 * ((((locals.var_x_dn3 * locals.var_x) + (locals.var_x * locals.var_x_dn3)) / (2.0 * assign4170_e3933)) / assign4170_e3933))) } * assign4170_e3965) + (assign4170_e3938 * ((p.p66 * (-((3.0 * locals.var_x_dn3) * assign4170_e3950))) - (((((6.0 * locals.var_x_dn3) * locals.var_x) + (assign4170_e3956 * locals.var_x_dn3)) * assign4170_e3963) + (assign4170_e3958 * locals.var_x_dn3))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4170_e3937) as f64).is_finite() && ((assign4170_e3937) as f64).fract() == 0.0 { if assign4170_e3937 == 0.0 { 0.0 } else { (assign4170_e3937 * ((assign4170_e3933).powf(assign4170_e3937 - 1.0) * (((locals.var_x_dn4 * locals.var_x) + (locals.var_x * locals.var_x_dn4)) / (2.0 * assign4170_e3933)))) } } else { (assign4170_e3938 * (assign4170_e3937 * ((((locals.var_x_dn4 * locals.var_x) + (locals.var_x * locals.var_x_dn4)) / (2.0 * assign4170_e3933)) / assign4170_e3933))) } * assign4170_e3965) + (assign4170_e3938 * ((p.p66 * (-((3.0 * locals.var_x_dn4) * assign4170_e3950))) - (((((6.0 * locals.var_x_dn4) * locals.var_x) + (assign4170_e3956 * locals.var_x_dn4)) * assign4170_e3963) + (assign4170_e3958 * locals.var_x_dn4))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4170_e3937) as f64).is_finite() && ((assign4170_e3937) as f64).fract() == 0.0 { if assign4170_e3937 == 0.0 { 0.0 } else { (assign4170_e3937 * ((assign4170_e3933).powf(assign4170_e3937 - 1.0) * (((locals.var_x_dn5 * locals.var_x) + (locals.var_x * locals.var_x_dn5)) / (2.0 * assign4170_e3933)))) } } else { (assign4170_e3938 * (assign4170_e3937 * ((((locals.var_x_dn5 * locals.var_x) + (locals.var_x * locals.var_x_dn5)) / (2.0 * assign4170_e3933)) / assign4170_e3933))) } * assign4170_e3965) + (assign4170_e3938 * ((p.p66 * (-((3.0 * locals.var_x_dn5) * assign4170_e3950))) - (((((6.0 * locals.var_x_dn5) * locals.var_x) + (assign4170_e3956 * locals.var_x_dn5)) * assign4170_e3963) + (assign4170_e3958 * locals.var_x_dn5))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4170_e3937) as f64).is_finite() && ((assign4170_e3937) as f64).fract() == 0.0 { if assign4170_e3937 == 0.0 { 0.0 } else { (assign4170_e3937 * ((assign4170_e3933).powf(assign4170_e3937 - 1.0) * (((locals.var_x_dn6 * locals.var_x) + (locals.var_x * locals.var_x_dn6)) / (2.0 * assign4170_e3933)))) } } else { (assign4170_e3938 * (assign4170_e3937 * ((((locals.var_x_dn6 * locals.var_x) + (locals.var_x * locals.var_x_dn6)) / (2.0 * assign4170_e3933)) / assign4170_e3933))) } * assign4170_e3965) + (assign4170_e3938 * ((p.p66 * (-((3.0 * locals.var_x_dn6) * assign4170_e3950))) - (((((6.0 * locals.var_x_dn6) * locals.var_x) + (assign4170_e3956 * locals.var_x_dn6)) * assign4170_e3963) + (assign4170_e3958 * locals.var_x_dn6))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4170_e3937) as f64).is_finite() && ((assign4170_e3937) as f64).fract() == 0.0 { if assign4170_e3937 == 0.0 { 0.0 } else { (assign4170_e3937 * ((assign4170_e3933).powf(assign4170_e3937 - 1.0) * (((locals.var_x_dn7 * locals.var_x) + (locals.var_x * locals.var_x_dn7)) / (2.0 * assign4170_e3933)))) } } else { (assign4170_e3938 * (assign4170_e3937 * ((((locals.var_x_dn7 * locals.var_x) + (locals.var_x * locals.var_x_dn7)) / (2.0 * assign4170_e3933)) / assign4170_e3933))) } * assign4170_e3965) + (assign4170_e3938 * ((p.p66 * (-((3.0 * locals.var_x_dn7) * assign4170_e3950))) - (((((6.0 * locals.var_x_dn7) * locals.var_x) + (assign4170_e3956 * locals.var_x_dn7)) * assign4170_e3963) + (assign4170_e3958 * locals.var_x_dn7))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4170_e3937) as f64).is_finite() && ((assign4170_e3937) as f64).fract() == 0.0 { if assign4170_e3937 == 0.0 { 0.0 } else { (assign4170_e3937 * ((assign4170_e3933).powf(assign4170_e3937 - 1.0) * (((locals.var_x_dn8 * locals.var_x) + (locals.var_x * locals.var_x_dn8)) / (2.0 * assign4170_e3933)))) } } else { (assign4170_e3938 * (assign4170_e3937 * ((((locals.var_x_dn8 * locals.var_x) + (locals.var_x * locals.var_x_dn8)) / (2.0 * assign4170_e3933)) / assign4170_e3933))) } * assign4170_e3965) + (assign4170_e3938 * ((p.p66 * (-((3.0 * locals.var_x_dn8) * assign4170_e3950))) - (((((6.0 * locals.var_x_dn8) * locals.var_x) + (assign4170_e3956 * locals.var_x_dn8)) * assign4170_e3963) + (assign4170_e3958 * locals.var_x_dn8))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4170_e3937) as f64).is_finite() && ((assign4170_e3937) as f64).fract() == 0.0 { if assign4170_e3937 == 0.0 { 0.0 } else { (assign4170_e3937 * ((assign4170_e3933).powf(assign4170_e3937 - 1.0) * (((locals.var_x_dn9 * locals.var_x) + (locals.var_x * locals.var_x_dn9)) / (2.0 * assign4170_e3933)))) } } else { (assign4170_e3938 * (assign4170_e3937 * ((((locals.var_x_dn9 * locals.var_x) + (locals.var_x * locals.var_x_dn9)) / (2.0 * assign4170_e3933)) / assign4170_e3933))) } * assign4170_e3965) + (assign4170_e3938 * ((p.p66 * (-((3.0 * locals.var_x_dn9) * assign4170_e3950))) - (((((6.0 * locals.var_x_dn9) * locals.var_x) + (assign4170_e3956 * locals.var_x_dn9)) * assign4170_e3963) + (assign4170_e3958 * locals.var_x_dn9))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4170_e3937) as f64).is_finite() && ((assign4170_e3937) as f64).fract() == 0.0 { if assign4170_e3937 == 0.0 { 0.0 } else { (assign4170_e3937 * ((assign4170_e3933).powf(assign4170_e3937 - 1.0) * (((locals.var_x_dn10 * locals.var_x) + (locals.var_x * locals.var_x_dn10)) / (2.0 * assign4170_e3933)))) } } else { (assign4170_e3938 * (assign4170_e3937 * ((((locals.var_x_dn10 * locals.var_x) + (locals.var_x * locals.var_x_dn10)) / (2.0 * assign4170_e3933)) / assign4170_e3933))) } * assign4170_e3965) + (assign4170_e3938 * ((p.p66 * (-((3.0 * locals.var_x_dn10) * assign4170_e3950))) - (((((6.0 * locals.var_x_dn10) * locals.var_x) + (assign4170_e3956 * locals.var_x_dn10)) * assign4170_e3963) + (assign4170_e3958 * locals.var_x_dn10))))) * 0.16666666666666666),)
    } else {
        (locals.var_de0eb, locals.var_de0eb_dn0, locals.var_de0eb_dn1, locals.var_de0eb_dn3, locals.var_de0eb_dn4, locals.var_de0eb_dn5, locals.var_de0eb_dn6, locals.var_de0eb_dn7, locals.var_de0eb_dn8, locals.var_de0eb_dn9, locals.var_de0eb_dn10,)
    }
};
        locals.var_de0eb = assign4170_e3970;
        locals.var_de0eb_dn0 = assign4170_e3970_d_n0;
        locals.var_de0eb_dn1 = assign4170_e3970_d_n1;
        locals.var_de0eb_dn3 = assign4170_e3970_d_n3;
        locals.var_de0eb_dn4 = assign4170_e3970_d_n4;
        locals.var_de0eb_dn5 = assign4170_e3970_d_n5;
        locals.var_de0eb_dn6 = assign4170_e3970_d_n6;
        locals.var_de0eb_dn7 = assign4170_e3970_d_n7;
        locals.var_de0eb_dn8 = assign4170_e3970_d_n8;
        locals.var_de0eb_dn9 = assign4170_e3970_d_n9;
        locals.var_de0eb_dn10 = assign4170_e3970_d_n10;
        locals.var_de0eb_rv = 0.0;

        let (assign4180_e3982, assign4180_e3982_d_n0, assign4180_e3982_d_n1, assign4180_e3982_d_n3, assign4180_e3982_d_n4, assign4180_e3982_d_n5, assign4180_e3982_d_n6, assign4180_e3982_d_n7, assign4180_e3982_d_n8, assign4180_e3982_d_n9, assign4180_e3982_d_n10,) = {
    if (locals.var_guard69 != 0.0) {
        let assign4180_e3974: f64 = (locals.var_vb2e1 * locals.var_pow2_2m_pe);
        let assign4180_e3976: f64 = (assign4180_e3974 * locals.var_nzeb_t);
        let assign4180_e3979: f64 = (locals.var_vgzeb_t * locals.var_de0eb);
        let assign4180_e3980: f64 = (assign4180_e3976 / assign4180_e3979);
        (assign4180_e3980, ((((assign4180_e3974 * locals.var_nzeb_t_dn0) * assign4180_e3979) - (assign4180_e3976 * ((locals.var_vgzeb_t_dn0 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn0)))) / (assign4180_e3979 * assign4180_e3979)), ((((assign4180_e3974 * locals.var_nzeb_t_dn1) * assign4180_e3979) - (assign4180_e3976 * ((locals.var_vgzeb_t_dn1 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn1)))) / (assign4180_e3979 * assign4180_e3979)), ((((assign4180_e3974 * locals.var_nzeb_t_dn3) * assign4180_e3979) - (assign4180_e3976 * ((locals.var_vgzeb_t_dn3 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn3)))) / (assign4180_e3979 * assign4180_e3979)), ((((((locals.var_vb2e1_dn4 * locals.var_pow2_2m_pe) * locals.var_nzeb_t) + (assign4180_e3974 * locals.var_nzeb_t_dn4)) * assign4180_e3979) - (assign4180_e3976 * ((locals.var_vgzeb_t_dn4 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn4)))) / (assign4180_e3979 * assign4180_e3979)), ((((assign4180_e3974 * locals.var_nzeb_t_dn5) * assign4180_e3979) - (assign4180_e3976 * ((locals.var_vgzeb_t_dn5 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn5)))) / (assign4180_e3979 * assign4180_e3979)), ((((((locals.var_vb2e1_dn6 * locals.var_pow2_2m_pe) * locals.var_nzeb_t) + (assign4180_e3974 * locals.var_nzeb_t_dn6)) * assign4180_e3979) - (assign4180_e3976 * ((locals.var_vgzeb_t_dn6 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn6)))) / (assign4180_e3979 * assign4180_e3979)), ((((assign4180_e3974 * locals.var_nzeb_t_dn7) * assign4180_e3979) - (assign4180_e3976 * ((locals.var_vgzeb_t_dn7 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn7)))) / (assign4180_e3979 * assign4180_e3979)), ((((assign4180_e3974 * locals.var_nzeb_t_dn8) * assign4180_e3979) - (assign4180_e3976 * ((locals.var_vgzeb_t_dn8 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn8)))) / (assign4180_e3979 * assign4180_e3979)), ((((assign4180_e3974 * locals.var_nzeb_t_dn9) * assign4180_e3979) - (assign4180_e3976 * ((locals.var_vgzeb_t_dn9 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn9)))) / (assign4180_e3979 * assign4180_e3979)), ((((assign4180_e3974 * locals.var_nzeb_t_dn10) * assign4180_e3979) - (assign4180_e3976 * ((locals.var_vgzeb_t_dn10 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn10)))) / (assign4180_e3979 * assign4180_e3979)),)
    } else {
        (locals.var_x, locals.var_x_dn0, locals.var_x_dn1, locals.var_x_dn3, locals.var_x_dn4, locals.var_x_dn5, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8, locals.var_x_dn9, locals.var_x_dn10,)
    }
};
        locals.var_x = assign4180_e3982;
        locals.var_x_dn0 = assign4180_e3982_d_n0;
        locals.var_x_dn1 = assign4180_e3982_d_n1;
        locals.var_x_dn3 = assign4180_e3982_d_n3;
        locals.var_x_dn4 = assign4180_e3982_d_n4;
        locals.var_x_dn5 = assign4180_e3982_d_n5;
        locals.var_x_dn6 = assign4180_e3982_d_n6;
        locals.var_x_dn7 = assign4180_e3982_d_n7;
        locals.var_x_dn8 = assign4180_e3982_d_n8;
        locals.var_x_dn9 = assign4180_e3982_d_n9;
        locals.var_x_dn10 = assign4180_e3982_d_n10;
        locals.var_x_rv = 0.0;

        let assign4190_e3985: f64 = (-0.001);
        let assign4190_e3986: f64 = if locals.var_x < assign4190_e3985 { 1.0 } else { 0.0 };
        locals.var_guard71 = assign4190_e3986;
        locals.var_guard71_rv = 0.0;

        let assign4200_e3989: f64 = if locals.var_x < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard72 = assign4200_e3989;
        locals.var_guard72_rv = 0.0;

        let (assign4220_e4008,) = {
    if (((locals.var_guard69 != 0.0) && (locals.var_guard71 != 0.0)) && (locals.var_guard72 == 0.0)) {
        let assign4220_e4006: f64 = (p.p138).exp();
        (assign4220_e4006,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4220_e4008;
        locals.var_expl_rv = 0.0;

        let assign4290_e4098: f64 = if (((p.p35 > 0.0) && (p.p36 > 0.0)) && (locals.var_vb2c1 < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard73 = assign4290_e4098;
        locals.var_guard73_rv = 0.0;

        let (assign4300_e4110, assign4300_e4110_d_n0, assign4300_e4110_d_n1, assign4300_e4110_d_n3, assign4300_e4110_d_n4, assign4300_e4110_d_n5, assign4300_e4110_d_n6, assign4300_e4110_d_n7, assign4300_e4110_d_n8, assign4300_e4110_d_n9, assign4300_e4110_d_n10,) = {
    if (locals.var_guard73 != 0.0) {
        let assign4300_e4103: f64 = (locals.var_vb2c1 * locals.var_inv_vdc_zener_t);
        let assign4300_e4104: f64 = (1.0 - assign4300_e4103);
        let assign4300_e4107: f64 = (1.0 - locals.var_pc_zener);
        let assign4300_e4108: f64 = (assign4300_e4104).powf(assign4300_e4107);
        (assign4300_e4108, if 0.0 == 0.0 && ((assign4300_e4107) as f64).is_finite() && ((assign4300_e4107) as f64).fract() == 0.0 { if assign4300_e4107 == 0.0 { 0.0 } else { (assign4300_e4107 * ((assign4300_e4104).powf(assign4300_e4107 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn0)))) } } else { (assign4300_e4108 * (assign4300_e4107 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn0)) / assign4300_e4104))) }, if 0.0 == 0.0 && ((assign4300_e4107) as f64).is_finite() && ((assign4300_e4107) as f64).fract() == 0.0 { if assign4300_e4107 == 0.0 { 0.0 } else { (assign4300_e4107 * ((assign4300_e4104).powf(assign4300_e4107 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn1)))) } } else { (assign4300_e4108 * (assign4300_e4107 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn1)) / assign4300_e4104))) }, if 0.0 == 0.0 && ((assign4300_e4107) as f64).is_finite() && ((assign4300_e4107) as f64).fract() == 0.0 { if assign4300_e4107 == 0.0 { 0.0 } else { (assign4300_e4107 * ((assign4300_e4104).powf(assign4300_e4107 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn3)))) } } else { (assign4300_e4108 * (assign4300_e4107 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn3)) / assign4300_e4104))) }, if 0.0 == 0.0 && ((assign4300_e4107) as f64).is_finite() && ((assign4300_e4107) as f64).fract() == 0.0 { if assign4300_e4107 == 0.0 { 0.0 } else { (assign4300_e4107 * ((assign4300_e4104).powf(assign4300_e4107 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn4)))) } } else { (assign4300_e4108 * (assign4300_e4107 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn4)) / assign4300_e4104))) }, if 0.0 == 0.0 && ((assign4300_e4107) as f64).is_finite() && ((assign4300_e4107) as f64).fract() == 0.0 { if assign4300_e4107 == 0.0 { 0.0 } else { (assign4300_e4107 * ((assign4300_e4104).powf(assign4300_e4107 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn5)))) } } else { (assign4300_e4108 * (assign4300_e4107 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn5)) / assign4300_e4104))) }, if 0.0 == 0.0 && ((assign4300_e4107) as f64).is_finite() && ((assign4300_e4107) as f64).fract() == 0.0 { if assign4300_e4107 == 0.0 { 0.0 } else { (assign4300_e4107 * ((assign4300_e4104).powf(assign4300_e4107 - 1.0) * (-((locals.var_vb2c1_dn6 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn6))))) } } else { (assign4300_e4108 * (assign4300_e4107 * ((-((locals.var_vb2c1_dn6 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn6))) / assign4300_e4104))) }, if 0.0 == 0.0 && ((assign4300_e4107) as f64).is_finite() && ((assign4300_e4107) as f64).fract() == 0.0 { if assign4300_e4107 == 0.0 { 0.0 } else { (assign4300_e4107 * ((assign4300_e4104).powf(assign4300_e4107 - 1.0) * (-((locals.var_vb2c1_dn7 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn7))))) } } else { (assign4300_e4108 * (assign4300_e4107 * ((-((locals.var_vb2c1_dn7 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn7))) / assign4300_e4104))) }, if 0.0 == 0.0 && ((assign4300_e4107) as f64).is_finite() && ((assign4300_e4107) as f64).fract() == 0.0 { if assign4300_e4107 == 0.0 { 0.0 } else { (assign4300_e4107 * ((assign4300_e4104).powf(assign4300_e4107 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn8)))) } } else { (assign4300_e4108 * (assign4300_e4107 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn8)) / assign4300_e4104))) }, if 0.0 == 0.0 && ((assign4300_e4107) as f64).is_finite() && ((assign4300_e4107) as f64).fract() == 0.0 { if assign4300_e4107 == 0.0 { 0.0 } else { (assign4300_e4107 * ((assign4300_e4104).powf(assign4300_e4107 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn9)))) } } else { (assign4300_e4108 * (assign4300_e4107 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn9)) / assign4300_e4104))) }, if 0.0 == 0.0 && ((assign4300_e4107) as f64).is_finite() && ((assign4300_e4107) as f64).fract() == 0.0 { if assign4300_e4107 == 0.0 { 0.0 } else { (assign4300_e4107 * ((assign4300_e4104).powf(assign4300_e4107 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn10)))) } } else { (assign4300_e4108 * (assign4300_e4107 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn10)) / assign4300_e4104))) },)
    } else {
        (locals.var_e0cb, locals.var_e0cb_dn0, locals.var_e0cb_dn1, locals.var_e0cb_dn3, locals.var_e0cb_dn4, locals.var_e0cb_dn5, locals.var_e0cb_dn6, locals.var_e0cb_dn7, locals.var_e0cb_dn8, locals.var_e0cb_dn9, locals.var_e0cb_dn10,)
    }
};
        locals.var_e0cb = assign4300_e4110;
        locals.var_e0cb_dn0 = assign4300_e4110_d_n0;
        locals.var_e0cb_dn1 = assign4300_e4110_d_n1;
        locals.var_e0cb_dn3 = assign4300_e4110_d_n3;
        locals.var_e0cb_dn4 = assign4300_e4110_d_n4;
        locals.var_e0cb_dn5 = assign4300_e4110_d_n5;
        locals.var_e0cb_dn6 = assign4300_e4110_d_n6;
        locals.var_e0cb_dn7 = assign4300_e4110_d_n7;
        locals.var_e0cb_dn8 = assign4300_e4110_d_n8;
        locals.var_e0cb_dn9 = assign4300_e4110_d_n9;
        locals.var_e0cb_dn10 = assign4300_e4110_d_n10;
        locals.var_e0cb_rv = 0.0;

        let assign4310_e4116: f64 = (2.0 * locals.var_e0cb);
        let assign4310_e4117: f64 = (locals.var_pow2_2m_pc / assign4310_e4116);
        let assign4310_e4118: f64 = (1.0 - assign4310_e4117);
        let assign4310_e4119: f64 = (locals.var_nzcb_t * assign4310_e4118);
        let assign4310_e4121: f64 = if assign4310_e4119 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard74 = assign4310_e4121;
        locals.var_guard74_rv = 0.0;

        let (assign4330_e4144,) = {
    if ((locals.var_guard73 != 0.0) && (locals.var_guard74 == 0.0)) {
        let assign4330_e4142: f64 = (p.p138).exp();
        (assign4330_e4142,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4330_e4144;
        locals.var_expl_rv = 0.0;

        let (assign4350_e4171, assign4350_e4171_d_n0, assign4350_e4171_d_n1, assign4350_e4171_d_n3, assign4350_e4171_d_n4, assign4350_e4171_d_n5, assign4350_e4171_d_n6, assign4350_e4171_d_n7, assign4350_e4171_d_n8, assign4350_e4171_d_n9, assign4350_e4171_d_n10,) = {
    if (locals.var_guard73 != 0.0) {
        let assign4350_e4169: f64 = (locals.var_vb2c1 * locals.var_inv_vdc_zener_t);
        (assign4350_e4169, (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn0), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn1), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn3), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn4), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn5), ((locals.var_vb2c1_dn6 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn6)), ((locals.var_vb2c1_dn7 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn7)), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn8), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn9), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn10),)
    } else {
        (locals.var_xx, locals.var_xx_dn0, locals.var_xx_dn1, locals.var_xx_dn3, locals.var_xx_dn4, locals.var_xx_dn5, locals.var_xx_dn6, locals.var_xx_dn7, locals.var_xx_dn8, locals.var_xx_dn9, locals.var_xx_dn10,)
    }
};
        locals.var_xx = assign4350_e4171;
        locals.var_xx_dn0 = assign4350_e4171_d_n0;
        locals.var_xx_dn1 = assign4350_e4171_d_n1;
        locals.var_xx_dn3 = assign4350_e4171_d_n3;
        locals.var_xx_dn4 = assign4350_e4171_d_n4;
        locals.var_xx_dn5 = assign4350_e4171_d_n5;
        locals.var_xx_dn6 = assign4350_e4171_d_n6;
        locals.var_xx_dn7 = assign4350_e4171_d_n7;
        locals.var_xx_dn8 = assign4350_e4171_d_n8;
        locals.var_xx_dn9 = assign4350_e4171_d_n9;
        locals.var_xx_dn10 = assign4350_e4171_d_n10;
        locals.var_xx_rv = 0.0;

        let (assign4360_e4215, assign4360_e4215_d_n0, assign4360_e4215_d_n1, assign4360_e4215_d_n3, assign4360_e4215_d_n4, assign4360_e4215_d_n5, assign4360_e4215_d_n6, assign4360_e4215_d_n7, assign4360_e4215_d_n8, assign4360_e4215_d_n9, assign4360_e4215_d_n10,) = {
    if (locals.var_guard73 != 0.0) {
        let assign4360_e4175: f64 = (locals.var_xx * locals.var_xx);
        let assign4360_e4177: f64 = (assign4360_e4175 + 1e-30);
        let assign4360_e4178: f64 = (assign4360_e4177).sqrt();
        let assign4360_e4180: f64 = (-2.0);
        let assign4360_e4182: f64 = (assign4360_e4180 - locals.var_pc_zener);
        let assign4360_e4183: f64 = (assign4360_e4178).powf(assign4360_e4182);
        let assign4360_e4188: f64 = (locals.var_pc_zener * locals.var_pc_zener);
        let assign4360_e4189: f64 = (1.0 - assign4360_e4188);
        let assign4360_e4192: f64 = (3.0 * locals.var_xx);
        let assign4360_e4195: f64 = (locals.var_pc_zener - 1.0);
        let assign4360_e4196: f64 = (assign4360_e4192 * assign4360_e4195);
        let assign4360_e4197: f64 = (assign4360_e4189 - assign4360_e4196);
        let assign4360_e4198: f64 = (locals.var_pc_zener * assign4360_e4197);
        let assign4360_e4201: f64 = (6.0 * locals.var_xx);
        let assign4360_e4203: f64 = (assign4360_e4201 * locals.var_xx);
        let assign4360_e4206: f64 = (locals.var_pc_zener - 1.0);
        let assign4360_e4208: f64 = (assign4360_e4206 + locals.var_xx);
        let assign4360_e4209: f64 = (assign4360_e4203 * assign4360_e4208);
        let assign4360_e4210: f64 = (assign4360_e4198 - assign4360_e4209);
        let assign4360_e4211: f64 = (assign4360_e4183 * assign4360_e4210);
        let assign4360_e4213: f64 = (assign4360_e4211 * 0.16666666666666666);
        (assign4360_e4213, (((if 0.0 == 0.0 && ((assign4360_e4182) as f64).is_finite() && ((assign4360_e4182) as f64).fract() == 0.0 { if assign4360_e4182 == 0.0 { 0.0 } else { (assign4360_e4182 * ((assign4360_e4178).powf(assign4360_e4182 - 1.0) * (((locals.var_xx_dn0 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn0)) / (2.0 * assign4360_e4178)))) } } else { (assign4360_e4183 * (assign4360_e4182 * ((((locals.var_xx_dn0 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn0)) / (2.0 * assign4360_e4178)) / assign4360_e4178))) } * assign4360_e4210) + (assign4360_e4183 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn0) * assign4360_e4195))) - (((((6.0 * locals.var_xx_dn0) * locals.var_xx) + (assign4360_e4201 * locals.var_xx_dn0)) * assign4360_e4208) + (assign4360_e4203 * locals.var_xx_dn0))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4360_e4182) as f64).is_finite() && ((assign4360_e4182) as f64).fract() == 0.0 { if assign4360_e4182 == 0.0 { 0.0 } else { (assign4360_e4182 * ((assign4360_e4178).powf(assign4360_e4182 - 1.0) * (((locals.var_xx_dn1 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn1)) / (2.0 * assign4360_e4178)))) } } else { (assign4360_e4183 * (assign4360_e4182 * ((((locals.var_xx_dn1 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn1)) / (2.0 * assign4360_e4178)) / assign4360_e4178))) } * assign4360_e4210) + (assign4360_e4183 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn1) * assign4360_e4195))) - (((((6.0 * locals.var_xx_dn1) * locals.var_xx) + (assign4360_e4201 * locals.var_xx_dn1)) * assign4360_e4208) + (assign4360_e4203 * locals.var_xx_dn1))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4360_e4182) as f64).is_finite() && ((assign4360_e4182) as f64).fract() == 0.0 { if assign4360_e4182 == 0.0 { 0.0 } else { (assign4360_e4182 * ((assign4360_e4178).powf(assign4360_e4182 - 1.0) * (((locals.var_xx_dn3 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn3)) / (2.0 * assign4360_e4178)))) } } else { (assign4360_e4183 * (assign4360_e4182 * ((((locals.var_xx_dn3 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn3)) / (2.0 * assign4360_e4178)) / assign4360_e4178))) } * assign4360_e4210) + (assign4360_e4183 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn3) * assign4360_e4195))) - (((((6.0 * locals.var_xx_dn3) * locals.var_xx) + (assign4360_e4201 * locals.var_xx_dn3)) * assign4360_e4208) + (assign4360_e4203 * locals.var_xx_dn3))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4360_e4182) as f64).is_finite() && ((assign4360_e4182) as f64).fract() == 0.0 { if assign4360_e4182 == 0.0 { 0.0 } else { (assign4360_e4182 * ((assign4360_e4178).powf(assign4360_e4182 - 1.0) * (((locals.var_xx_dn4 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn4)) / (2.0 * assign4360_e4178)))) } } else { (assign4360_e4183 * (assign4360_e4182 * ((((locals.var_xx_dn4 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn4)) / (2.0 * assign4360_e4178)) / assign4360_e4178))) } * assign4360_e4210) + (assign4360_e4183 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn4) * assign4360_e4195))) - (((((6.0 * locals.var_xx_dn4) * locals.var_xx) + (assign4360_e4201 * locals.var_xx_dn4)) * assign4360_e4208) + (assign4360_e4203 * locals.var_xx_dn4))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4360_e4182) as f64).is_finite() && ((assign4360_e4182) as f64).fract() == 0.0 { if assign4360_e4182 == 0.0 { 0.0 } else { (assign4360_e4182 * ((assign4360_e4178).powf(assign4360_e4182 - 1.0) * (((locals.var_xx_dn5 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn5)) / (2.0 * assign4360_e4178)))) } } else { (assign4360_e4183 * (assign4360_e4182 * ((((locals.var_xx_dn5 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn5)) / (2.0 * assign4360_e4178)) / assign4360_e4178))) } * assign4360_e4210) + (assign4360_e4183 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn5) * assign4360_e4195))) - (((((6.0 * locals.var_xx_dn5) * locals.var_xx) + (assign4360_e4201 * locals.var_xx_dn5)) * assign4360_e4208) + (assign4360_e4203 * locals.var_xx_dn5))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4360_e4182) as f64).is_finite() && ((assign4360_e4182) as f64).fract() == 0.0 { if assign4360_e4182 == 0.0 { 0.0 } else { (assign4360_e4182 * ((assign4360_e4178).powf(assign4360_e4182 - 1.0) * (((locals.var_xx_dn6 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn6)) / (2.0 * assign4360_e4178)))) } } else { (assign4360_e4183 * (assign4360_e4182 * ((((locals.var_xx_dn6 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn6)) / (2.0 * assign4360_e4178)) / assign4360_e4178))) } * assign4360_e4210) + (assign4360_e4183 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn6) * assign4360_e4195))) - (((((6.0 * locals.var_xx_dn6) * locals.var_xx) + (assign4360_e4201 * locals.var_xx_dn6)) * assign4360_e4208) + (assign4360_e4203 * locals.var_xx_dn6))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4360_e4182) as f64).is_finite() && ((assign4360_e4182) as f64).fract() == 0.0 { if assign4360_e4182 == 0.0 { 0.0 } else { (assign4360_e4182 * ((assign4360_e4178).powf(assign4360_e4182 - 1.0) * (((locals.var_xx_dn7 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn7)) / (2.0 * assign4360_e4178)))) } } else { (assign4360_e4183 * (assign4360_e4182 * ((((locals.var_xx_dn7 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn7)) / (2.0 * assign4360_e4178)) / assign4360_e4178))) } * assign4360_e4210) + (assign4360_e4183 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn7) * assign4360_e4195))) - (((((6.0 * locals.var_xx_dn7) * locals.var_xx) + (assign4360_e4201 * locals.var_xx_dn7)) * assign4360_e4208) + (assign4360_e4203 * locals.var_xx_dn7))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4360_e4182) as f64).is_finite() && ((assign4360_e4182) as f64).fract() == 0.0 { if assign4360_e4182 == 0.0 { 0.0 } else { (assign4360_e4182 * ((assign4360_e4178).powf(assign4360_e4182 - 1.0) * (((locals.var_xx_dn8 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn8)) / (2.0 * assign4360_e4178)))) } } else { (assign4360_e4183 * (assign4360_e4182 * ((((locals.var_xx_dn8 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn8)) / (2.0 * assign4360_e4178)) / assign4360_e4178))) } * assign4360_e4210) + (assign4360_e4183 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn8) * assign4360_e4195))) - (((((6.0 * locals.var_xx_dn8) * locals.var_xx) + (assign4360_e4201 * locals.var_xx_dn8)) * assign4360_e4208) + (assign4360_e4203 * locals.var_xx_dn8))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4360_e4182) as f64).is_finite() && ((assign4360_e4182) as f64).fract() == 0.0 { if assign4360_e4182 == 0.0 { 0.0 } else { (assign4360_e4182 * ((assign4360_e4178).powf(assign4360_e4182 - 1.0) * (((locals.var_xx_dn9 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn9)) / (2.0 * assign4360_e4178)))) } } else { (assign4360_e4183 * (assign4360_e4182 * ((((locals.var_xx_dn9 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn9)) / (2.0 * assign4360_e4178)) / assign4360_e4178))) } * assign4360_e4210) + (assign4360_e4183 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn9) * assign4360_e4195))) - (((((6.0 * locals.var_xx_dn9) * locals.var_xx) + (assign4360_e4201 * locals.var_xx_dn9)) * assign4360_e4208) + (assign4360_e4203 * locals.var_xx_dn9))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4360_e4182) as f64).is_finite() && ((assign4360_e4182) as f64).fract() == 0.0 { if assign4360_e4182 == 0.0 { 0.0 } else { (assign4360_e4182 * ((assign4360_e4178).powf(assign4360_e4182 - 1.0) * (((locals.var_xx_dn10 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn10)) / (2.0 * assign4360_e4178)))) } } else { (assign4360_e4183 * (assign4360_e4182 * ((((locals.var_xx_dn10 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn10)) / (2.0 * assign4360_e4178)) / assign4360_e4178))) } * assign4360_e4210) + (assign4360_e4183 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn10) * assign4360_e4195))) - (((((6.0 * locals.var_xx_dn10) * locals.var_xx) + (assign4360_e4201 * locals.var_xx_dn10)) * assign4360_e4208) + (assign4360_e4203 * locals.var_xx_dn10))))) * 0.16666666666666666),)
    } else {
        (locals.var_de0cb, locals.var_de0cb_dn0, locals.var_de0cb_dn1, locals.var_de0cb_dn3, locals.var_de0cb_dn4, locals.var_de0cb_dn5, locals.var_de0cb_dn6, locals.var_de0cb_dn7, locals.var_de0cb_dn8, locals.var_de0cb_dn9, locals.var_de0cb_dn10,)
    }
};
        locals.var_de0cb = assign4360_e4215;
        locals.var_de0cb_dn0 = assign4360_e4215_d_n0;
        locals.var_de0cb_dn1 = assign4360_e4215_d_n1;
        locals.var_de0cb_dn3 = assign4360_e4215_d_n3;
        locals.var_de0cb_dn4 = assign4360_e4215_d_n4;
        locals.var_de0cb_dn5 = assign4360_e4215_d_n5;
        locals.var_de0cb_dn6 = assign4360_e4215_d_n6;
        locals.var_de0cb_dn7 = assign4360_e4215_d_n7;
        locals.var_de0cb_dn8 = assign4360_e4215_d_n8;
        locals.var_de0cb_dn9 = assign4360_e4215_d_n9;
        locals.var_de0cb_dn10 = assign4360_e4215_d_n10;
        locals.var_de0cb_rv = 0.0;

        let (assign4370_e4227, assign4370_e4227_d_n0, assign4370_e4227_d_n1, assign4370_e4227_d_n3, assign4370_e4227_d_n4, assign4370_e4227_d_n5, assign4370_e4227_d_n6, assign4370_e4227_d_n7, assign4370_e4227_d_n8, assign4370_e4227_d_n9, assign4370_e4227_d_n10,) = {
    if (locals.var_guard73 != 0.0) {
        let assign4370_e4219: f64 = (locals.var_vb2c1 * locals.var_pow2_2m_pc);
        let assign4370_e4221: f64 = (assign4370_e4219 * locals.var_nzcb_t);
        let assign4370_e4224: f64 = (locals.var_vgzcb_t * locals.var_de0cb);
        let assign4370_e4225: f64 = (assign4370_e4221 / assign4370_e4224);
        (assign4370_e4225, ((((assign4370_e4219 * locals.var_nzcb_t_dn0) * assign4370_e4224) - (assign4370_e4221 * ((locals.var_vgzcb_t_dn0 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn0)))) / (assign4370_e4224 * assign4370_e4224)), ((((assign4370_e4219 * locals.var_nzcb_t_dn1) * assign4370_e4224) - (assign4370_e4221 * ((locals.var_vgzcb_t_dn1 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn1)))) / (assign4370_e4224 * assign4370_e4224)), ((((assign4370_e4219 * locals.var_nzcb_t_dn3) * assign4370_e4224) - (assign4370_e4221 * ((locals.var_vgzcb_t_dn3 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn3)))) / (assign4370_e4224 * assign4370_e4224)), ((((assign4370_e4219 * locals.var_nzcb_t_dn4) * assign4370_e4224) - (assign4370_e4221 * ((locals.var_vgzcb_t_dn4 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn4)))) / (assign4370_e4224 * assign4370_e4224)), ((((assign4370_e4219 * locals.var_nzcb_t_dn5) * assign4370_e4224) - (assign4370_e4221 * ((locals.var_vgzcb_t_dn5 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn5)))) / (assign4370_e4224 * assign4370_e4224)), ((((((locals.var_vb2c1_dn6 * locals.var_pow2_2m_pc) * locals.var_nzcb_t) + (assign4370_e4219 * locals.var_nzcb_t_dn6)) * assign4370_e4224) - (assign4370_e4221 * ((locals.var_vgzcb_t_dn6 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn6)))) / (assign4370_e4224 * assign4370_e4224)), ((((((locals.var_vb2c1_dn7 * locals.var_pow2_2m_pc) * locals.var_nzcb_t) + (assign4370_e4219 * locals.var_nzcb_t_dn7)) * assign4370_e4224) - (assign4370_e4221 * ((locals.var_vgzcb_t_dn7 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn7)))) / (assign4370_e4224 * assign4370_e4224)), ((((assign4370_e4219 * locals.var_nzcb_t_dn8) * assign4370_e4224) - (assign4370_e4221 * ((locals.var_vgzcb_t_dn8 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn8)))) / (assign4370_e4224 * assign4370_e4224)), ((((assign4370_e4219 * locals.var_nzcb_t_dn9) * assign4370_e4224) - (assign4370_e4221 * ((locals.var_vgzcb_t_dn9 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn9)))) / (assign4370_e4224 * assign4370_e4224)), ((((assign4370_e4219 * locals.var_nzcb_t_dn10) * assign4370_e4224) - (assign4370_e4221 * ((locals.var_vgzcb_t_dn10 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn10)))) / (assign4370_e4224 * assign4370_e4224)),)
    } else {
        (locals.var_xx, locals.var_xx_dn0, locals.var_xx_dn1, locals.var_xx_dn3, locals.var_xx_dn4, locals.var_xx_dn5, locals.var_xx_dn6, locals.var_xx_dn7, locals.var_xx_dn8, locals.var_xx_dn9, locals.var_xx_dn10,)
    }
};
        locals.var_xx = assign4370_e4227;
        locals.var_xx_dn0 = assign4370_e4227_d_n0;
        locals.var_xx_dn1 = assign4370_e4227_d_n1;
        locals.var_xx_dn3 = assign4370_e4227_d_n3;
        locals.var_xx_dn4 = assign4370_e4227_d_n4;
        locals.var_xx_dn5 = assign4370_e4227_d_n5;
        locals.var_xx_dn6 = assign4370_e4227_d_n6;
        locals.var_xx_dn7 = assign4370_e4227_d_n7;
        locals.var_xx_dn8 = assign4370_e4227_d_n8;
        locals.var_xx_dn9 = assign4370_e4227_d_n9;
        locals.var_xx_dn10 = assign4370_e4227_d_n10;
        locals.var_xx_rv = 0.0;

        let assign4380_e4230: f64 = (-0.001);
        let assign4380_e4231: f64 = if locals.var_xx < assign4380_e4230 { 1.0 } else { 0.0 };
        locals.var_guard75 = assign4380_e4231;
        locals.var_guard75_rv = 0.0;

        let assign4390_e4234: f64 = if locals.var_xx < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign4390_e4234;
        locals.var_guard76_rv = 0.0;

        let (assign4410_e4253,) = {
    if (((locals.var_guard73 != 0.0) && (locals.var_guard75 != 0.0)) && (locals.var_guard76 == 0.0)) {
        let assign4410_e4251: f64 = (p.p138).exp();
        (assign4410_e4251,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4410_e4253;
        locals.var_expl_rv = 0.0;

        let assign4480_e4335: f64 = (locals.var_if0 * locals.var_evb1c4);
        locals.var_g1 = assign4480_e4335;
        locals.var_g1_dn0 = (locals.var_if0_dn0 * locals.var_evb1c4);
        locals.var_g1_dn1 = (locals.var_if0_dn1 * locals.var_evb1c4);
        locals.var_g1_dn3 = ((locals.var_if0_dn3 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn3));
        locals.var_g1_dn4 = (locals.var_if0_dn4 * locals.var_evb1c4);
        locals.var_g1_dn5 = ((locals.var_if0_dn5 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn5));
        locals.var_g1_dn6 = ((locals.var_if0_dn6 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn6));
        locals.var_g1_dn7 = ((locals.var_if0_dn7 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn7));
        locals.var_g1_dn8 = ((locals.var_if0_dn8 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn8));
        locals.var_g1_dn9 = (locals.var_if0_dn9 * locals.var_evb1c4);
        locals.var_g1_dn10 = ((locals.var_if0_dn10 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn10));
        locals.var_g1_rv = 0.0;

        let assign4490_e4338: f64 = (4.0 * locals.var_evb1c4vdc);
        locals.var_g2 = assign4490_e4338;
        locals.var_g2_dn0 = (4.0 * locals.var_evb1c4vdc_dn0);
        locals.var_g2_dn1 = (4.0 * locals.var_evb1c4vdc_dn1);
        locals.var_g2_dn3 = (4.0 * locals.var_evb1c4vdc_dn3);
        locals.var_g2_dn4 = (4.0 * locals.var_evb1c4vdc_dn4);
        locals.var_g2_dn5 = (4.0 * locals.var_evb1c4vdc_dn5);
        locals.var_g2_dn6 = (4.0 * locals.var_evb1c4vdc_dn6);
        locals.var_g2_dn7 = (4.0 * locals.var_evb1c4vdc_dn7);
        locals.var_g2_dn8 = (4.0 * locals.var_evb1c4vdc_dn8);
        locals.var_g2_dn9 = (4.0 * locals.var_evb1c4vdc_dn9);
        locals.var_g2_dn10 = (4.0 * locals.var_evb1c4vdc_dn10);
        locals.var_g2_rv = 0.0;

        let assign4500_e4341: f64 = (locals.var_g1 - locals.var_if0);
        let assign4500_e4345: f64 = (1.0 + locals.var_g1);
        let assign4500_e4346: f64 = (assign4500_e4345).sqrt();
        let assign4500_e4347: f64 = (1.0 + assign4500_e4346);
        let assign4500_e4348: f64 = (assign4500_e4341 / assign4500_e4347);
        locals.var_nbex = assign4500_e4348;
        locals.var_nbex_dn0 = ((((locals.var_g1_dn0 - locals.var_if0_dn0) * assign4500_e4347) - (assign4500_e4341 * (locals.var_g1_dn0 / (2.0 * assign4500_e4346)))) / (assign4500_e4347 * assign4500_e4347));
        locals.var_nbex_dn1 = ((((locals.var_g1_dn1 - locals.var_if0_dn1) * assign4500_e4347) - (assign4500_e4341 * (locals.var_g1_dn1 / (2.0 * assign4500_e4346)))) / (assign4500_e4347 * assign4500_e4347));
        locals.var_nbex_dn3 = ((((locals.var_g1_dn3 - locals.var_if0_dn3) * assign4500_e4347) - (assign4500_e4341 * (locals.var_g1_dn3 / (2.0 * assign4500_e4346)))) / (assign4500_e4347 * assign4500_e4347));
        locals.var_nbex_dn4 = ((((locals.var_g1_dn4 - locals.var_if0_dn4) * assign4500_e4347) - (assign4500_e4341 * (locals.var_g1_dn4 / (2.0 * assign4500_e4346)))) / (assign4500_e4347 * assign4500_e4347));
        locals.var_nbex_dn5 = ((((locals.var_g1_dn5 - locals.var_if0_dn5) * assign4500_e4347) - (assign4500_e4341 * (locals.var_g1_dn5 / (2.0 * assign4500_e4346)))) / (assign4500_e4347 * assign4500_e4347));
        locals.var_nbex_dn6 = ((((locals.var_g1_dn6 - locals.var_if0_dn6) * assign4500_e4347) - (assign4500_e4341 * (locals.var_g1_dn6 / (2.0 * assign4500_e4346)))) / (assign4500_e4347 * assign4500_e4347));
        locals.var_nbex_dn7 = ((((locals.var_g1_dn7 - locals.var_if0_dn7) * assign4500_e4347) - (assign4500_e4341 * (locals.var_g1_dn7 / (2.0 * assign4500_e4346)))) / (assign4500_e4347 * assign4500_e4347));
        locals.var_nbex_dn8 = ((((locals.var_g1_dn8 - locals.var_if0_dn8) * assign4500_e4347) - (assign4500_e4341 * (locals.var_g1_dn8 / (2.0 * assign4500_e4346)))) / (assign4500_e4347 * assign4500_e4347));
        locals.var_nbex_dn9 = ((((locals.var_g1_dn9 - locals.var_if0_dn9) * assign4500_e4347) - (assign4500_e4341 * (locals.var_g1_dn9 / (2.0 * assign4500_e4346)))) / (assign4500_e4347 * assign4500_e4347));
        locals.var_nbex_dn10 = ((((locals.var_g1_dn10 - locals.var_if0_dn10) * assign4500_e4347) - (assign4500_e4341 * (locals.var_g1_dn10 / (2.0 * assign4500_e4346)))) / (assign4500_e4347 * assign4500_e4347));
        locals.var_nbex_rv = 0.0;

        let assign4510_e4353: f64 = (1.0 + locals.var_g2);
        let assign4510_e4354: f64 = (assign4510_e4353).sqrt();
        let assign4510_e4355: f64 = (1.0 + assign4510_e4354);
        let assign4510_e4356: f64 = (locals.var_g2 / assign4510_e4355);
        locals.var_pwex = assign4510_e4356;
        locals.var_pwex_dn0 = (((locals.var_g2_dn0 * assign4510_e4355) - (locals.var_g2 * (locals.var_g2_dn0 / (2.0 * assign4510_e4354)))) / (assign4510_e4355 * assign4510_e4355));
        locals.var_pwex_dn1 = (((locals.var_g2_dn1 * assign4510_e4355) - (locals.var_g2 * (locals.var_g2_dn1 / (2.0 * assign4510_e4354)))) / (assign4510_e4355 * assign4510_e4355));
        locals.var_pwex_dn3 = (((locals.var_g2_dn3 * assign4510_e4355) - (locals.var_g2 * (locals.var_g2_dn3 / (2.0 * assign4510_e4354)))) / (assign4510_e4355 * assign4510_e4355));
        locals.var_pwex_dn4 = (((locals.var_g2_dn4 * assign4510_e4355) - (locals.var_g2 * (locals.var_g2_dn4 / (2.0 * assign4510_e4354)))) / (assign4510_e4355 * assign4510_e4355));
        locals.var_pwex_dn5 = (((locals.var_g2_dn5 * assign4510_e4355) - (locals.var_g2 * (locals.var_g2_dn5 / (2.0 * assign4510_e4354)))) / (assign4510_e4355 * assign4510_e4355));
        locals.var_pwex_dn6 = (((locals.var_g2_dn6 * assign4510_e4355) - (locals.var_g2 * (locals.var_g2_dn6 / (2.0 * assign4510_e4354)))) / (assign4510_e4355 * assign4510_e4355));
        locals.var_pwex_dn7 = (((locals.var_g2_dn7 * assign4510_e4355) - (locals.var_g2 * (locals.var_g2_dn7 / (2.0 * assign4510_e4354)))) / (assign4510_e4355 * assign4510_e4355));
        locals.var_pwex_dn8 = (((locals.var_g2_dn8 * assign4510_e4355) - (locals.var_g2 * (locals.var_g2_dn8 / (2.0 * assign4510_e4354)))) / (assign4510_e4355 * assign4510_e4355));
        locals.var_pwex_dn9 = (((locals.var_g2_dn9 * assign4510_e4355) - (locals.var_g2 * (locals.var_g2_dn9 / (2.0 * assign4510_e4354)))) / (assign4510_e4355 * assign4510_e4355));
        locals.var_pwex_dn10 = (((locals.var_g2_dn10 * assign4510_e4355) - (locals.var_g2 * (locals.var_g2_dn10 / (2.0 * assign4510_e4354)))) / (assign4510_e4355 * assign4510_e4355));
        locals.var_pwex_rv = 0.0;

        let assign4530_e4383: f64 = if ((p.p5 > 0.0) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard77 = assign4530_e4383;
        locals.var_guard77_rv = 0.0;

        let (assign4550_e4414, assign4550_e4414_d_n0, assign4550_e4414_d_n1, assign4550_e4414_d_n3, assign4550_e4414_d_n5, assign4550_e4414_d_n6, assign4550_e4414_d_n7, assign4550_e4414_d_n8, assign4550_e4414_d_n9, assign4550_e4414_d_n10,) = {
    if (locals.var_guard77 != 0.0) {
        let assign4550_e4393: f64 = (p.p32 * 2.0);
        let assign4550_e4395: f64 = (assign4550_e4393 * locals.var_ibx_t);
        let assign4550_e4398: f64 = (locals.var_evbc3 - 1.0);
        let assign4550_e4399: f64 = (assign4550_e4395 * assign4550_e4398);
        let assign4550_e4404: f64 = (4.0 * locals.var_ibx_t);
        let assign4550_e4406: f64 = (assign4550_e4404 / locals.var_ikbx_t);
        let assign4550_e4408: f64 = (assign4550_e4406 * locals.var_evbc3);
        let assign4550_e4409: f64 = (1.0 + assign4550_e4408);
        let assign4550_e4410: f64 = (assign4550_e4409).sqrt();
        let assign4550_e4411: f64 = (1.0 + assign4550_e4410);
        let assign4550_e4412: f64 = (assign4550_e4399 / assign4550_e4411);
        (assign4550_e4412, ((((assign4550_e4395 * locals.var_evbc3_dn0) * assign4550_e4411) - (assign4550_e4399 * ((assign4550_e4406 * locals.var_evbc3_dn0) / (2.0 * assign4550_e4410)))) / (assign4550_e4411 * assign4550_e4411)), ((((assign4550_e4395 * locals.var_evbc3_dn1) * assign4550_e4411) - (assign4550_e4399 * ((assign4550_e4406 * locals.var_evbc3_dn1) / (2.0 * assign4550_e4410)))) / (assign4550_e4411 * assign4550_e4411)), ((((((assign4550_e4393 * locals.var_ibx_t_dn3) * assign4550_e4398) + (assign4550_e4395 * locals.var_evbc3_dn3)) * assign4550_e4411) - (assign4550_e4399 * (((((((4.0 * locals.var_ibx_t_dn3) * locals.var_ikbx_t) - (assign4550_e4404 * locals.var_ikbx_t_dn3)) / (locals.var_ikbx_t * locals.var_ikbx_t)) * locals.var_evbc3) + (assign4550_e4406 * locals.var_evbc3_dn3)) / (2.0 * assign4550_e4410)))) / (assign4550_e4411 * assign4550_e4411)), ((((assign4550_e4395 * locals.var_evbc3_dn5) * assign4550_e4411) - (assign4550_e4399 * ((assign4550_e4406 * locals.var_evbc3_dn5) / (2.0 * assign4550_e4410)))) / (assign4550_e4411 * assign4550_e4411)), ((((assign4550_e4395 * locals.var_evbc3_dn6) * assign4550_e4411) - (assign4550_e4399 * ((assign4550_e4406 * locals.var_evbc3_dn6) / (2.0 * assign4550_e4410)))) / (assign4550_e4411 * assign4550_e4411)), ((((assign4550_e4395 * locals.var_evbc3_dn7) * assign4550_e4411) - (assign4550_e4399 * ((assign4550_e4406 * locals.var_evbc3_dn7) / (2.0 * assign4550_e4410)))) / (assign4550_e4411 * assign4550_e4411)), ((((assign4550_e4395 * locals.var_evbc3_dn8) * assign4550_e4411) - (assign4550_e4399 * ((assign4550_e4406 * locals.var_evbc3_dn8) / (2.0 * assign4550_e4410)))) / (assign4550_e4411 * assign4550_e4411)), ((((assign4550_e4395 * locals.var_evbc3_dn9) * assign4550_e4411) - (assign4550_e4399 * ((assign4550_e4406 * locals.var_evbc3_dn9) / (2.0 * assign4550_e4410)))) / (assign4550_e4411 * assign4550_e4411)), ((((assign4550_e4395 * locals.var_evbc3_dn10) * assign4550_e4411) - (assign4550_e4399 * ((assign4550_e4406 * locals.var_evbc3_dn10) / (2.0 * assign4550_e4410)))) / (assign4550_e4411 * assign4550_e4411)),)
    } else {
        (locals.var_ximex, locals.var_ximex_dn0, locals.var_ximex_dn1, locals.var_ximex_dn3, locals.var_ximex_dn5, locals.var_ximex_dn6, locals.var_ximex_dn7, locals.var_ximex_dn8, locals.var_ximex_dn9, locals.var_ximex_dn10,)
    }
};
        locals.var_ximex = assign4550_e4414;
        locals.var_ximex_dn0 = assign4550_e4414_d_n0;
        locals.var_ximex_dn1 = assign4550_e4414_d_n1;
        locals.var_ximex_dn3 = assign4550_e4414_d_n3;
        locals.var_ximex_dn5 = assign4550_e4414_d_n5;
        locals.var_ximex_dn6 = assign4550_e4414_d_n6;
        locals.var_ximex_dn7 = assign4550_e4414_d_n7;
        locals.var_ximex_dn8 = assign4550_e4414_d_n8;
        locals.var_ximex_dn9 = assign4550_e4414_d_n9;
        locals.var_ximex_dn10 = assign4550_e4414_d_n10;
        locals.var_ximex_rv = 0.0;

        let (assign4560_e4418,) = {
    if (locals.var_guard77 != 0.0) {
        (0.0,)
    } else {
        (locals.var_ximsub,)
    }
};
        locals.var_ximsub = assign4560_e4418;
        locals.var_ximsub_rv = 0.0;

        let assign4570_e4421: f64 = if p.p5 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard78 = assign4570_e4421;
        locals.var_guard78_rv = 0.0;

        let (assign4580_e4431, assign4580_e4431_d_n3,) = {
    if ((locals.var_guard77 != 0.0) && (locals.var_guard78 != 0.0)) {
        let assign4580_e4427: f64 = (p.p32 * locals.var_ibx_t);
        let assign4580_e4429: f64 = (assign4580_e4427 * locals.var_rcc_xx_t);
        (assign4580_e4429, (((p.p32 * locals.var_ibx_t_dn3) * locals.var_rcc_xx_t) + (assign4580_e4427 * locals.var_rcc_xx_t_dn3)),)
    } else {
        (locals.var_vex_bias, locals.var_vex_bias_dn3,)
    }
};
        locals.var_vex_bias = assign4580_e4431;
        locals.var_vex_bias_dn3 = assign4580_e4431_d_n3;
        locals.var_vex_bias_rv = 0.0;

        let (assign4590_e4444, assign4590_e4444_d_n3,) = {
    if ((locals.var_guard77 != 0.0) && (locals.var_guard78 != 0.0)) {
        let assign4590_e4439: f64 = (locals.var_vex_bias * locals.var_vtinv);
        let assign4590_e4440: f64 = (assign4590_e4439).ln();
        let assign4590_e4441: f64 = (2.0 - assign4590_e4440);
        let assign4590_e4442: f64 = (locals.var_vt * assign4590_e4441);
        (assign4590_e4442, ((locals.var_vt_dn3 * assign4590_e4441) + (locals.var_vt * (-(((locals.var_vex_bias_dn3 * locals.var_vtinv) + (locals.var_vex_bias * locals.var_vtinv_dn3)) / assign4590_e4439)))),)
    } else {
        (locals.var_vex, locals.var_vex_dn3,)
    }
};
        locals.var_vex = assign4590_e4444;
        locals.var_vex_dn3 = assign4590_e4444_d_n3;
        locals.var_vex_rv = 0.0;

        let (assign4600_e4452, assign4600_e4452_d_n0, assign4600_e4452_d_n1, assign4600_e4452_d_n3, assign4600_e4452_d_n5, assign4600_e4452_d_n6, assign4600_e4452_d_n7, assign4600_e4452_d_n8, assign4600_e4452_d_n9, assign4600_e4452_d_n10,) = {
    if ((locals.var_guard77 != 0.0) && (locals.var_guard78 != 0.0)) {
        let assign4600_e4450: f64 = (locals.var_vbc3 - locals.var_vex);
        (assign4600_e4450, locals.var_vbc3_dn0, locals.var_vbc3_dn1, (-locals.var_vex_dn3), locals.var_vbc3_dn5, locals.var_vbc3_dn6, locals.var_vbc3_dn7, locals.var_vbc3_dn8, locals.var_vbc3_dn9, locals.var_vbc3_dn10,)
    } else {
        (locals.var_vdif, locals.var_vdif_dn0, locals.var_vdif_dn1, locals.var_vdif_dn3, locals.var_vdif_dn5, locals.var_vdif_dn6, locals.var_vdif_dn7, locals.var_vdif_dn8, locals.var_vdif_dn9, locals.var_vdif_dn10,)
    }
};
        locals.var_vdif = assign4600_e4452;
        locals.var_vdif_dn0 = assign4600_e4452_d_n0;
        locals.var_vdif_dn1 = assign4600_e4452_d_n1;
        locals.var_vdif_dn3 = assign4600_e4452_d_n3;
        locals.var_vdif_dn5 = assign4600_e4452_d_n5;
        locals.var_vdif_dn6 = assign4600_e4452_d_n6;
        locals.var_vdif_dn7 = assign4600_e4452_d_n7;
        locals.var_vdif_dn8 = assign4600_e4452_d_n8;
        locals.var_vdif_dn9 = assign4600_e4452_d_n9;
        locals.var_vdif_dn10 = assign4600_e4452_d_n10;
        locals.var_vdif_rv = 0.0;

        let (assign4610_e4460, assign4610_e4460_d_n0, assign4610_e4460_d_n1, assign4610_e4460_d_n3, assign4610_e4460_d_n4, assign4610_e4460_d_n5, assign4610_e4460_d_n6, assign4610_e4460_d_n7, assign4610_e4460_d_n8, assign4610_e4460_d_n9, assign4610_e4460_d_n10,) = {
    if ((locals.var_guard77 != 0.0) && (locals.var_guard78 != 0.0)) {
        let assign4610_e4458: f64 = (0.11 * 0.11);
        (assign4610_e4458, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eps2, locals.var_eps2_dn0, locals.var_eps2_dn1, locals.var_eps2_dn3, locals.var_eps2_dn4, locals.var_eps2_dn5, locals.var_eps2_dn6, locals.var_eps2_dn7, locals.var_eps2_dn8, locals.var_eps2_dn9, locals.var_eps2_dn10,)
    }
};
        locals.var_eps2 = assign4610_e4460;
        locals.var_eps2_dn0 = assign4610_e4460_d_n0;
        locals.var_eps2_dn1 = assign4610_e4460_d_n1;
        locals.var_eps2_dn3 = assign4610_e4460_d_n3;
        locals.var_eps2_dn4 = assign4610_e4460_d_n4;
        locals.var_eps2_dn5 = assign4610_e4460_d_n5;
        locals.var_eps2_dn6 = assign4610_e4460_d_n6;
        locals.var_eps2_dn7 = assign4610_e4460_d_n7;
        locals.var_eps2_dn8 = assign4610_e4460_d_n8;
        locals.var_eps2_dn9 = assign4610_e4460_d_n9;
        locals.var_eps2_dn10 = assign4610_e4460_d_n10;
        locals.var_eps2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4620_e4468, assign4620_e4468_d_n0, assign4620_e4468_d_n1, assign4620_e4468_d_n3, assign4620_e4468_d_n4, assign4620_e4468_d_n5, assign4620_e4468_d_n6, assign4620_e4468_d_n7, assign4620_e4468_d_n8, assign4620_e4468_d_n9, assign4620_e4468_d_n10,) = {
    if ((locals.var_guard77 != 0.0) && (locals.var_guard78 != 0.0)) {
        let assign4620_e4466: f64 = (locals.var_vdif * locals.var_vdif);
        (assign4620_e4466, ((locals.var_vdif_dn0 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn0)), ((locals.var_vdif_dn1 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn1)), ((locals.var_vdif_dn3 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn3)), 0.0, ((locals.var_vdif_dn5 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn5)), ((locals.var_vdif_dn6 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn6)), ((locals.var_vdif_dn7 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn7)), ((locals.var_vdif_dn8 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn8)), ((locals.var_vdif_dn9 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn9)), ((locals.var_vdif_dn10 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn10)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn1, locals.var_x2_dn3, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10,)
    }
};
        locals.var_x2 = assign4620_e4468;
        locals.var_x2_dn0 = assign4620_e4468_d_n0;
        locals.var_x2_dn1 = assign4620_e4468_d_n1;
        locals.var_x2_dn3 = assign4620_e4468_d_n3;
        locals.var_x2_dn4 = assign4620_e4468_d_n4;
        locals.var_x2_dn5 = assign4620_e4468_d_n5;
        locals.var_x2_dn6 = assign4620_e4468_d_n6;
        locals.var_x2_dn7 = assign4620_e4468_d_n7;
        locals.var_x2_dn8 = assign4620_e4468_d_n8;
        locals.var_x2_dn9 = assign4620_e4468_d_n9;
        locals.var_x2_dn10 = assign4620_e4468_d_n10;
        locals.var_x2_rv = 0.0;

        let assign4630_e4471: f64 = if locals.var_vdif < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard79 = assign4630_e4471;
        locals.var_guard79_rv = 0.0;

        let (assign4640_e4488, assign4640_e4488_d_n0, assign4640_e4488_d_n1, assign4640_e4488_d_n3, assign4640_e4488_d_n4, assign4640_e4488_d_n5, assign4640_e4488_d_n6, assign4640_e4488_d_n7, assign4640_e4488_d_n8, assign4640_e4488_d_n9, assign4640_e4488_d_n10,) = {
    if (((locals.var_guard77 != 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) {
        let assign4640_e4479: f64 = (0.5 * locals.var_eps2);
        let assign4640_e4482: f64 = (locals.var_x2 + locals.var_eps2);
        let assign4640_e4483: f64 = (assign4640_e4482).sqrt();
        let assign4640_e4485: f64 = (assign4640_e4483 - locals.var_vdif);
        let assign4640_e4486: f64 = (assign4640_e4479 / assign4640_e4485);
        (assign4640_e4486, ((((0.5 * locals.var_eps2_dn0) * assign4640_e4485) - (assign4640_e4479 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign4640_e4483)) - locals.var_vdif_dn0))) / (assign4640_e4485 * assign4640_e4485)), ((((0.5 * locals.var_eps2_dn1) * assign4640_e4485) - (assign4640_e4479 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign4640_e4483)) - locals.var_vdif_dn1))) / (assign4640_e4485 * assign4640_e4485)), ((((0.5 * locals.var_eps2_dn3) * assign4640_e4485) - (assign4640_e4479 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign4640_e4483)) - locals.var_vdif_dn3))) / (assign4640_e4485 * assign4640_e4485)), ((((0.5 * locals.var_eps2_dn4) * assign4640_e4485) - (assign4640_e4479 * ((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign4640_e4483)))) / (assign4640_e4485 * assign4640_e4485)), ((((0.5 * locals.var_eps2_dn5) * assign4640_e4485) - (assign4640_e4479 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign4640_e4483)) - locals.var_vdif_dn5))) / (assign4640_e4485 * assign4640_e4485)), ((((0.5 * locals.var_eps2_dn6) * assign4640_e4485) - (assign4640_e4479 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign4640_e4483)) - locals.var_vdif_dn6))) / (assign4640_e4485 * assign4640_e4485)), ((((0.5 * locals.var_eps2_dn7) * assign4640_e4485) - (assign4640_e4479 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign4640_e4483)) - locals.var_vdif_dn7))) / (assign4640_e4485 * assign4640_e4485)), ((((0.5 * locals.var_eps2_dn8) * assign4640_e4485) - (assign4640_e4479 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign4640_e4483)) - locals.var_vdif_dn8))) / (assign4640_e4485 * assign4640_e4485)), ((((0.5 * locals.var_eps2_dn9) * assign4640_e4485) - (assign4640_e4479 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign4640_e4483)) - locals.var_vdif_dn9))) / (assign4640_e4485 * assign4640_e4485)), ((((0.5 * locals.var_eps2_dn10) * assign4640_e4485) - (assign4640_e4479 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign4640_e4483)) - locals.var_vdif_dn10))) / (assign4640_e4485 * assign4640_e4485)),)
    } else {
        (locals.var_vbex, locals.var_vbex_dn0, locals.var_vbex_dn1, locals.var_vbex_dn3, locals.var_vbex_dn4, locals.var_vbex_dn5, locals.var_vbex_dn6, locals.var_vbex_dn7, locals.var_vbex_dn8, locals.var_vbex_dn9, locals.var_vbex_dn10,)
    }
};
        locals.var_vbex = assign4640_e4488;
        locals.var_vbex_dn0 = assign4640_e4488_d_n0;
        locals.var_vbex_dn1 = assign4640_e4488_d_n1;
        locals.var_vbex_dn3 = assign4640_e4488_d_n3;
        locals.var_vbex_dn4 = assign4640_e4488_d_n4;
        locals.var_vbex_dn5 = assign4640_e4488_d_n5;
        locals.var_vbex_dn6 = assign4640_e4488_d_n6;
        locals.var_vbex_dn7 = assign4640_e4488_d_n7;
        locals.var_vbex_dn8 = assign4640_e4488_d_n8;
        locals.var_vbex_dn9 = assign4640_e4488_d_n9;
        locals.var_vbex_dn10 = assign4640_e4488_d_n10;
        locals.var_vbex_rv = 0.0;

        let (assign4650_e4504, assign4650_e4504_d_n0, assign4650_e4504_d_n1, assign4650_e4504_d_n3, assign4650_e4504_d_n4, assign4650_e4504_d_n5, assign4650_e4504_d_n6, assign4650_e4504_d_n7, assign4650_e4504_d_n8, assign4650_e4504_d_n9, assign4650_e4504_d_n10,) = {
    if (((locals.var_guard77 != 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 == 0.0)) {
        let assign4650_e4498: f64 = (locals.var_x2 + locals.var_eps2);
        let assign4650_e4499: f64 = (assign4650_e4498).sqrt();
        let assign4650_e4501: f64 = (assign4650_e4499 + locals.var_vdif);
        let assign4650_e4502: f64 = (0.5 * assign4650_e4501);
        (assign4650_e4502, (0.5 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign4650_e4499)) + locals.var_vdif_dn0)), (0.5 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign4650_e4499)) + locals.var_vdif_dn1)), (0.5 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign4650_e4499)) + locals.var_vdif_dn3)), (0.5 * ((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign4650_e4499))), (0.5 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign4650_e4499)) + locals.var_vdif_dn5)), (0.5 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign4650_e4499)) + locals.var_vdif_dn6)), (0.5 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign4650_e4499)) + locals.var_vdif_dn7)), (0.5 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign4650_e4499)) + locals.var_vdif_dn8)), (0.5 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign4650_e4499)) + locals.var_vdif_dn9)), (0.5 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign4650_e4499)) + locals.var_vdif_dn10)),)
    } else {
        (locals.var_vbex, locals.var_vbex_dn0, locals.var_vbex_dn1, locals.var_vbex_dn3, locals.var_vbex_dn4, locals.var_vbex_dn5, locals.var_vbex_dn6, locals.var_vbex_dn7, locals.var_vbex_dn8, locals.var_vbex_dn9, locals.var_vbex_dn10,)
    }
};
        locals.var_vbex = assign4650_e4504;
        locals.var_vbex_dn0 = assign4650_e4504_d_n0;
        locals.var_vbex_dn1 = assign4650_e4504_d_n1;
        locals.var_vbex_dn3 = assign4650_e4504_d_n3;
        locals.var_vbex_dn4 = assign4650_e4504_d_n4;
        locals.var_vbex_dn5 = assign4650_e4504_d_n5;
        locals.var_vbex_dn6 = assign4650_e4504_d_n6;
        locals.var_vbex_dn7 = assign4650_e4504_d_n7;
        locals.var_vbex_dn8 = assign4650_e4504_d_n8;
        locals.var_vbex_dn9 = assign4650_e4504_d_n9;
        locals.var_vbex_dn10 = assign4650_e4504_d_n10;
        locals.var_vbex_rv = 0.0;

        let (assign4660_e4520, assign4660_e4520_d_n0, assign4660_e4520_d_n1, assign4660_e4520_d_n3, assign4660_e4520_d_n4, assign4660_e4520_d_n5, assign4660_e4520_d_n6, assign4660_e4520_d_n7, assign4660_e4520_d_n8, assign4660_e4520_d_n9, assign4660_e4520_d_n10,) = {
    if ((locals.var_guard77 != 0.0) && (locals.var_guard78 != 0.0)) {
        let assign4660_e4512: f64 = (locals.var_ximex + locals.var_ximsub);
        let assign4660_e4514: f64 = (assign4660_e4512 * locals.var_rcc_xx_t);
        let assign4660_e4515: f64 = (locals.var_vex_bias + assign4660_e4514);
        let assign4660_e4517: f64 = (assign4660_e4515 + locals.var_vbex);
        let assign4660_e4518: f64 = (locals.var_vbex / assign4660_e4517);
        (assign4660_e4518, (((locals.var_vbex_dn0 * assign4660_e4517) - (locals.var_vbex * ((locals.var_ximex_dn0 * locals.var_rcc_xx_t) + locals.var_vbex_dn0))) / (assign4660_e4517 * assign4660_e4517)), (((locals.var_vbex_dn1 * assign4660_e4517) - (locals.var_vbex * ((locals.var_ximex_dn1 * locals.var_rcc_xx_t) + locals.var_vbex_dn1))) / (assign4660_e4517 * assign4660_e4517)), (((locals.var_vbex_dn3 * assign4660_e4517) - (locals.var_vbex * ((locals.var_vex_bias_dn3 + ((locals.var_ximex_dn3 * locals.var_rcc_xx_t) + (assign4660_e4512 * locals.var_rcc_xx_t_dn3))) + locals.var_vbex_dn3))) / (assign4660_e4517 * assign4660_e4517)), (((locals.var_vbex_dn4 * assign4660_e4517) - (locals.var_vbex * locals.var_vbex_dn4)) / (assign4660_e4517 * assign4660_e4517)), (((locals.var_vbex_dn5 * assign4660_e4517) - (locals.var_vbex * ((locals.var_ximex_dn5 * locals.var_rcc_xx_t) + locals.var_vbex_dn5))) / (assign4660_e4517 * assign4660_e4517)), (((locals.var_vbex_dn6 * assign4660_e4517) - (locals.var_vbex * ((locals.var_ximex_dn6 * locals.var_rcc_xx_t) + locals.var_vbex_dn6))) / (assign4660_e4517 * assign4660_e4517)), (((locals.var_vbex_dn7 * assign4660_e4517) - (locals.var_vbex * ((locals.var_ximex_dn7 * locals.var_rcc_xx_t) + locals.var_vbex_dn7))) / (assign4660_e4517 * assign4660_e4517)), (((locals.var_vbex_dn8 * assign4660_e4517) - (locals.var_vbex * ((locals.var_ximex_dn8 * locals.var_rcc_xx_t) + locals.var_vbex_dn8))) / (assign4660_e4517 * assign4660_e4517)), (((locals.var_vbex_dn9 * assign4660_e4517) - (locals.var_vbex * ((locals.var_ximex_dn9 * locals.var_rcc_xx_t) + locals.var_vbex_dn9))) / (assign4660_e4517 * assign4660_e4517)), (((locals.var_vbex_dn10 * assign4660_e4517) - (locals.var_vbex * ((locals.var_ximex_dn10 * locals.var_rcc_xx_t) + locals.var_vbex_dn10))) / (assign4660_e4517 * assign4660_e4517)),)
    } else {
        (locals.var_fex, locals.var_fex_dn0, locals.var_fex_dn1, locals.var_fex_dn3, locals.var_fex_dn4, locals.var_fex_dn5, locals.var_fex_dn6, locals.var_fex_dn7, locals.var_fex_dn8, locals.var_fex_dn9, locals.var_fex_dn10,)
    }
};
        locals.var_fex = assign4660_e4520;
        locals.var_fex_dn0 = assign4660_e4520_d_n0;
        locals.var_fex_dn1 = assign4660_e4520_d_n1;
        locals.var_fex_dn3 = assign4660_e4520_d_n3;
        locals.var_fex_dn4 = assign4660_e4520_d_n4;
        locals.var_fex_dn5 = assign4660_e4520_d_n5;
        locals.var_fex_dn6 = assign4660_e4520_d_n6;
        locals.var_fex_dn7 = assign4660_e4520_d_n7;
        locals.var_fex_dn8 = assign4660_e4520_d_n8;
        locals.var_fex_dn9 = assign4660_e4520_d_n9;
        locals.var_fex_dn10 = assign4660_e4520_d_n10;
        locals.var_fex_rv = 0.0;

        let (assign4670_e4527, assign4670_e4527_d_n3,) = {
    if ((locals.var_guard77 != 0.0) && (locals.var_guard78 == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_vex, locals.var_vex_dn3,)
    }
};
        locals.var_vex = assign4670_e4527;
        locals.var_vex_dn3 = assign4670_e4527_d_n3;
        locals.var_vex_rv = 0.0;

        let (assign4680_e4534, assign4680_e4534_d_n0, assign4680_e4534_d_n1, assign4680_e4534_d_n3, assign4680_e4534_d_n5, assign4680_e4534_d_n6, assign4680_e4534_d_n7, assign4680_e4534_d_n8, assign4680_e4534_d_n9, assign4680_e4534_d_n10,) = {
    if ((locals.var_guard77 != 0.0) && (locals.var_guard78 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vdif, locals.var_vdif_dn0, locals.var_vdif_dn1, locals.var_vdif_dn3, locals.var_vdif_dn5, locals.var_vdif_dn6, locals.var_vdif_dn7, locals.var_vdif_dn8, locals.var_vdif_dn9, locals.var_vdif_dn10,)
    }
};
        locals.var_vdif = assign4680_e4534;
        locals.var_vdif_dn0 = assign4680_e4534_d_n0;
        locals.var_vdif_dn1 = assign4680_e4534_d_n1;
        locals.var_vdif_dn3 = assign4680_e4534_d_n3;
        locals.var_vdif_dn5 = assign4680_e4534_d_n5;
        locals.var_vdif_dn6 = assign4680_e4534_d_n6;
        locals.var_vdif_dn7 = assign4680_e4534_d_n7;
        locals.var_vdif_dn8 = assign4680_e4534_d_n8;
        locals.var_vdif_dn9 = assign4680_e4534_d_n9;
        locals.var_vdif_dn10 = assign4680_e4534_d_n10;
        locals.var_vdif_rv = 0.0;

        let (assign4690_e4541, assign4690_e4541_d_n0, assign4690_e4541_d_n1, assign4690_e4541_d_n3, assign4690_e4541_d_n4, assign4690_e4541_d_n5, assign4690_e4541_d_n6, assign4690_e4541_d_n7, assign4690_e4541_d_n8, assign4690_e4541_d_n9, assign4690_e4541_d_n10,) = {
    if ((locals.var_guard77 != 0.0) && (locals.var_guard78 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbex, locals.var_vbex_dn0, locals.var_vbex_dn1, locals.var_vbex_dn3, locals.var_vbex_dn4, locals.var_vbex_dn5, locals.var_vbex_dn6, locals.var_vbex_dn7, locals.var_vbex_dn8, locals.var_vbex_dn9, locals.var_vbex_dn10,)
    }
};
        locals.var_vbex = assign4690_e4541;
        locals.var_vbex_dn0 = assign4690_e4541_d_n0;
        locals.var_vbex_dn1 = assign4690_e4541_d_n1;
        locals.var_vbex_dn3 = assign4690_e4541_d_n3;
        locals.var_vbex_dn4 = assign4690_e4541_d_n4;
        locals.var_vbex_dn5 = assign4690_e4541_d_n5;
        locals.var_vbex_dn6 = assign4690_e4541_d_n6;
        locals.var_vbex_dn7 = assign4690_e4541_d_n7;
        locals.var_vbex_dn8 = assign4690_e4541_d_n8;
        locals.var_vbex_dn9 = assign4690_e4541_d_n9;
        locals.var_vbex_dn10 = assign4690_e4541_d_n10;
        locals.var_vbex_rv = 0.0;

        let (assign4700_e4548, assign4700_e4548_d_n0, assign4700_e4548_d_n1, assign4700_e4548_d_n3, assign4700_e4548_d_n4, assign4700_e4548_d_n5, assign4700_e4548_d_n6, assign4700_e4548_d_n7, assign4700_e4548_d_n8, assign4700_e4548_d_n9, assign4700_e4548_d_n10,) = {
    if ((locals.var_guard77 != 0.0) && (locals.var_guard78 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fex, locals.var_fex_dn0, locals.var_fex_dn1, locals.var_fex_dn3, locals.var_fex_dn4, locals.var_fex_dn5, locals.var_fex_dn6, locals.var_fex_dn7, locals.var_fex_dn8, locals.var_fex_dn9, locals.var_fex_dn10,)
    }
};
        locals.var_fex = assign4700_e4548;
        locals.var_fex_dn0 = assign4700_e4548_d_n0;
        locals.var_fex_dn1 = assign4700_e4548_d_n1;
        locals.var_fex_dn3 = assign4700_e4548_d_n3;
        locals.var_fex_dn4 = assign4700_e4548_d_n4;
        locals.var_fex_dn5 = assign4700_e4548_d_n5;
        locals.var_fex_dn6 = assign4700_e4548_d_n6;
        locals.var_fex_dn7 = assign4700_e4548_d_n7;
        locals.var_fex_dn8 = assign4700_e4548_d_n8;
        locals.var_fex_dn9 = assign4700_e4548_d_n9;
        locals.var_fex_dn10 = assign4700_e4548_d_n10;
        locals.var_fex_rv = 0.0;

        let assign4720_e4557: f64 = if p.p83 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard80 = assign4720_e4557;
        locals.var_guard80_rv = 0.0;

        let (assign4730_e4563, assign4730_e4563_d_n5, assign4730_e4563_d_n6, assign4730_e4563_d_n7,) = {
    if (locals.var_guard80 != 0.0) {
        let assign4730_e4561: f64 = (locals.var_vb1b2 + locals.var_vb2c1);
        (assign4730_e4561, locals.var_vb1b2_dn5, (locals.var_vb1b2_dn6 + locals.var_vb2c1_dn6), locals.var_vb2c1_dn7,)
    } else {
        (locals.var_vb1c1, locals.var_vb1c1_dn5, locals.var_vb1c1_dn6, locals.var_vb1c1_dn7,)
    }
};
        locals.var_vb1c1 = assign4730_e4563;
        locals.var_vb1c1_dn5 = assign4730_e4563_d_n5;
        locals.var_vb1c1_dn6 = assign4730_e4563_d_n6;
        locals.var_vb1c1_dn7 = assign4730_e4563_d_n7;
        locals.var_vb1c1_rv = 0.0;

        let (assign4740_e4569, assign4740_e4569_d_n0, assign4740_e4569_d_n1, assign4740_e4569_d_n3, assign4740_e4569_d_n4, assign4740_e4569_d_n5, assign4740_e4569_d_n6, assign4740_e4569_d_n7, assign4740_e4569_d_n8, assign4740_e4569_d_n9, assign4740_e4569_d_n10,) = {
    if (locals.var_guard80 != 0.0) {
        let assign4740_e4567: f64 = (1e-6 * 1e-6);
        (assign4740_e4567, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eps2, locals.var_eps2_dn0, locals.var_eps2_dn1, locals.var_eps2_dn3, locals.var_eps2_dn4, locals.var_eps2_dn5, locals.var_eps2_dn6, locals.var_eps2_dn7, locals.var_eps2_dn8, locals.var_eps2_dn9, locals.var_eps2_dn10,)
    }
};
        locals.var_eps2 = assign4740_e4569;
        locals.var_eps2_dn0 = assign4740_e4569_d_n0;
        locals.var_eps2_dn1 = assign4740_e4569_d_n1;
        locals.var_eps2_dn3 = assign4740_e4569_d_n3;
        locals.var_eps2_dn4 = assign4740_e4569_d_n4;
        locals.var_eps2_dn5 = assign4740_e4569_d_n5;
        locals.var_eps2_dn6 = assign4740_e4569_d_n6;
        locals.var_eps2_dn7 = assign4740_e4569_d_n7;
        locals.var_eps2_dn8 = assign4740_e4569_d_n8;
        locals.var_eps2_dn9 = assign4740_e4569_d_n9;
        locals.var_eps2_dn10 = assign4740_e4569_d_n10;
        locals.var_eps2_rv = 0.0;

        let (assign4750_e4581, assign4750_e4581_d_n0, assign4750_e4581_d_n1, assign4750_e4581_d_n3, assign4750_e4581_d_n4, assign4750_e4581_d_n5, assign4750_e4581_d_n6, assign4750_e4581_d_n7, assign4750_e4581_d_n8, assign4750_e4581_d_n9, assign4750_e4581_d_n10,) = {
    if (locals.var_guard80 != 0.0) {
        let assign4750_e4572: f64 = (-1.0);
        let assign4750_e4574: f64 = (assign4750_e4572 * locals.var_vb1c1);
        let assign4750_e4576: f64 = (-1.0);
        let assign4750_e4577: f64 = (assign4750_e4574 * assign4750_e4576);
        let assign4750_e4579: f64 = (assign4750_e4577 * locals.var_vb1c1);
        (assign4750_e4579, 0.0, 0.0, 0.0, 0.0, ((((assign4750_e4572 * locals.var_vb1c1_dn5) * assign4750_e4576) * locals.var_vb1c1) + (assign4750_e4577 * locals.var_vb1c1_dn5)), ((((assign4750_e4572 * locals.var_vb1c1_dn6) * assign4750_e4576) * locals.var_vb1c1) + (assign4750_e4577 * locals.var_vb1c1_dn6)), ((((assign4750_e4572 * locals.var_vb1c1_dn7) * assign4750_e4576) * locals.var_vb1c1) + (assign4750_e4577 * locals.var_vb1c1_dn7)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn1, locals.var_x2_dn3, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10,)
    }
};
        locals.var_x2 = assign4750_e4581;
        locals.var_x2_dn0 = assign4750_e4581_d_n0;
        locals.var_x2_dn1 = assign4750_e4581_d_n1;
        locals.var_x2_dn3 = assign4750_e4581_d_n3;
        locals.var_x2_dn4 = assign4750_e4581_d_n4;
        locals.var_x2_dn5 = assign4750_e4581_d_n5;
        locals.var_x2_dn6 = assign4750_e4581_d_n6;
        locals.var_x2_dn7 = assign4750_e4581_d_n7;
        locals.var_x2_dn8 = assign4750_e4581_d_n8;
        locals.var_x2_dn9 = assign4750_e4581_d_n9;
        locals.var_x2_dn10 = assign4750_e4581_d_n10;
        locals.var_x2_rv = 0.0;

        let assign4900_e4705: f64 = (locals.var_vte / locals.var_ver_t);
        let assign4900_e4706: f64 = (1.0 + assign4900_e4705);
        let assign4900_e4709: f64 = (locals.var_vtc / locals.var_vef_t);
        let assign4900_e4710: f64 = (assign4900_e4706 + assign4900_e4709);
        locals.var_q0q = assign4900_e4710;
        locals.var_q0q_dn0 = ((((locals.var_vte_dn0 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn0)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn0 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn0)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn1 = ((((locals.var_vte_dn1 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn1)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn1 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn1)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn3 = ((((locals.var_vte_dn3 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn3)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn3 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn3)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn4 = ((((locals.var_vte_dn4 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn4)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn4 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn4)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn5 = ((((locals.var_vte_dn5 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn5)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn5 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn5)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn6 = ((((locals.var_vte_dn6 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn6)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn6 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn6)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn7 = ((((locals.var_vte_dn7 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn7)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn7 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn7)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn8 = ((((locals.var_vte_dn8 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn8)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn8 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn8)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn9 = ((((locals.var_vte_dn9 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn9)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn9 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn9)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn10 = ((((locals.var_vte_dn10 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn10)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn10 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn10)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_rv = 0.0;

        let assign4910_e4713: f64 = (0.1 * 0.1);
        locals.var_eps2 = assign4910_e4713;
        locals.var_eps2_dn0 = 0.0;
        locals.var_eps2_dn1 = 0.0;
        locals.var_eps2_dn3 = 0.0;
        locals.var_eps2_dn4 = 0.0;
        locals.var_eps2_dn5 = 0.0;
        locals.var_eps2_dn6 = 0.0;
        locals.var_eps2_dn7 = 0.0;
        locals.var_eps2_dn8 = 0.0;
        locals.var_eps2_dn9 = 0.0;
        locals.var_eps2_dn10 = 0.0;
        locals.var_eps2_rv = 0.0;

        let assign4920_e4716: f64 = (locals.var_q0q * locals.var_q0q);
        locals.var_x2 = assign4920_e4716;
        locals.var_x2_dn0 = ((locals.var_q0q_dn0 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn0));
        locals.var_x2_dn1 = ((locals.var_q0q_dn1 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn1));
        locals.var_x2_dn3 = ((locals.var_q0q_dn3 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn3));
        locals.var_x2_dn4 = ((locals.var_q0q_dn4 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn4));
        locals.var_x2_dn5 = ((locals.var_q0q_dn5 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn5));
        locals.var_x2_dn6 = ((locals.var_q0q_dn6 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn6));
        locals.var_x2_dn7 = ((locals.var_q0q_dn7 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn7));
        locals.var_x2_dn8 = ((locals.var_q0q_dn8 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn8));
        locals.var_x2_dn9 = ((locals.var_q0q_dn9 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn9));
        locals.var_x2_dn10 = ((locals.var_q0q_dn10 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn10));
        locals.var_x2_rv = 0.0;

        let assign4930_e4719: f64 = if locals.var_q0q < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign4930_e4719;
        locals.var_guard83_rv = 0.0;

        let (assign4940_e4732, assign4940_e4732_d_n0, assign4940_e4732_d_n1, assign4940_e4732_d_n3, assign4940_e4732_d_n4, assign4940_e4732_d_n5, assign4940_e4732_d_n6, assign4940_e4732_d_n7, assign4940_e4732_d_n8, assign4940_e4732_d_n9, assign4940_e4732_d_n10,) = {
    if (locals.var_guard83 != 0.0) {
        let assign4940_e4723: f64 = (0.5 * locals.var_eps2);
        let assign4940_e4726: f64 = (locals.var_x2 + locals.var_eps2);
        let assign4940_e4727: f64 = (assign4940_e4726).sqrt();
        let assign4940_e4729: f64 = (assign4940_e4727 - locals.var_q0q);
        let assign4940_e4730: f64 = (assign4940_e4723 / assign4940_e4729);
        (assign4940_e4730, ((((0.5 * locals.var_eps2_dn0) * assign4940_e4729) - (assign4940_e4723 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign4940_e4727)) - locals.var_q0q_dn0))) / (assign4940_e4729 * assign4940_e4729)), ((((0.5 * locals.var_eps2_dn1) * assign4940_e4729) - (assign4940_e4723 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign4940_e4727)) - locals.var_q0q_dn1))) / (assign4940_e4729 * assign4940_e4729)), ((((0.5 * locals.var_eps2_dn3) * assign4940_e4729) - (assign4940_e4723 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign4940_e4727)) - locals.var_q0q_dn3))) / (assign4940_e4729 * assign4940_e4729)), ((((0.5 * locals.var_eps2_dn4) * assign4940_e4729) - (assign4940_e4723 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign4940_e4727)) - locals.var_q0q_dn4))) / (assign4940_e4729 * assign4940_e4729)), ((((0.5 * locals.var_eps2_dn5) * assign4940_e4729) - (assign4940_e4723 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign4940_e4727)) - locals.var_q0q_dn5))) / (assign4940_e4729 * assign4940_e4729)), ((((0.5 * locals.var_eps2_dn6) * assign4940_e4729) - (assign4940_e4723 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign4940_e4727)) - locals.var_q0q_dn6))) / (assign4940_e4729 * assign4940_e4729)), ((((0.5 * locals.var_eps2_dn7) * assign4940_e4729) - (assign4940_e4723 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign4940_e4727)) - locals.var_q0q_dn7))) / (assign4940_e4729 * assign4940_e4729)), ((((0.5 * locals.var_eps2_dn8) * assign4940_e4729) - (assign4940_e4723 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign4940_e4727)) - locals.var_q0q_dn8))) / (assign4940_e4729 * assign4940_e4729)), ((((0.5 * locals.var_eps2_dn9) * assign4940_e4729) - (assign4940_e4723 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign4940_e4727)) - locals.var_q0q_dn9))) / (assign4940_e4729 * assign4940_e4729)), ((((0.5 * locals.var_eps2_dn10) * assign4940_e4729) - (assign4940_e4723 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign4940_e4727)) - locals.var_q0q_dn10))) / (assign4940_e4729 * assign4940_e4729)),)
    } else {
        (locals.var_q1q, locals.var_q1q_dn0, locals.var_q1q_dn1, locals.var_q1q_dn3, locals.var_q1q_dn4, locals.var_q1q_dn5, locals.var_q1q_dn6, locals.var_q1q_dn7, locals.var_q1q_dn8, locals.var_q1q_dn9, locals.var_q1q_dn10,)
    }
};
        locals.var_q1q = assign4940_e4732;
        locals.var_q1q_dn0 = assign4940_e4732_d_n0;
        locals.var_q1q_dn1 = assign4940_e4732_d_n1;
        locals.var_q1q_dn3 = assign4940_e4732_d_n3;
        locals.var_q1q_dn4 = assign4940_e4732_d_n4;
        locals.var_q1q_dn5 = assign4940_e4732_d_n5;
        locals.var_q1q_dn6 = assign4940_e4732_d_n6;
        locals.var_q1q_dn7 = assign4940_e4732_d_n7;
        locals.var_q1q_dn8 = assign4940_e4732_d_n8;
        locals.var_q1q_dn9 = assign4940_e4732_d_n9;
        locals.var_q1q_dn10 = assign4940_e4732_d_n10;
        locals.var_q1q_rv = 0.0;

        let (assign4950_e4744, assign4950_e4744_d_n0, assign4950_e4744_d_n1, assign4950_e4744_d_n3, assign4950_e4744_d_n4, assign4950_e4744_d_n5, assign4950_e4744_d_n6, assign4950_e4744_d_n7, assign4950_e4744_d_n8, assign4950_e4744_d_n9, assign4950_e4744_d_n10,) = {
    if (locals.var_guard83 == 0.0) {
        let assign4950_e4738: f64 = (locals.var_x2 + locals.var_eps2);
        let assign4950_e4739: f64 = (assign4950_e4738).sqrt();
        let assign4950_e4741: f64 = (assign4950_e4739 + locals.var_q0q);
        let assign4950_e4742: f64 = (0.5 * assign4950_e4741);
        (assign4950_e4742, (0.5 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign4950_e4739)) + locals.var_q0q_dn0)), (0.5 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign4950_e4739)) + locals.var_q0q_dn1)), (0.5 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign4950_e4739)) + locals.var_q0q_dn3)), (0.5 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign4950_e4739)) + locals.var_q0q_dn4)), (0.5 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign4950_e4739)) + locals.var_q0q_dn5)), (0.5 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign4950_e4739)) + locals.var_q0q_dn6)), (0.5 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign4950_e4739)) + locals.var_q0q_dn7)), (0.5 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign4950_e4739)) + locals.var_q0q_dn8)), (0.5 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign4950_e4739)) + locals.var_q0q_dn9)), (0.5 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign4950_e4739)) + locals.var_q0q_dn10)),)
    } else {
        (locals.var_q1q, locals.var_q1q_dn0, locals.var_q1q_dn1, locals.var_q1q_dn3, locals.var_q1q_dn4, locals.var_q1q_dn5, locals.var_q1q_dn6, locals.var_q1q_dn7, locals.var_q1q_dn8, locals.var_q1q_dn9, locals.var_q1q_dn10,)
    }
};
        locals.var_q1q = assign4950_e4744;
        locals.var_q1q_dn0 = assign4950_e4744_d_n0;
        locals.var_q1q_dn1 = assign4950_e4744_d_n1;
        locals.var_q1q_dn3 = assign4950_e4744_d_n3;
        locals.var_q1q_dn4 = assign4950_e4744_d_n4;
        locals.var_q1q_dn5 = assign4950_e4744_d_n5;
        locals.var_q1q_dn6 = assign4950_e4744_d_n6;
        locals.var_q1q_dn7 = assign4950_e4744_d_n7;
        locals.var_q1q_dn8 = assign4950_e4744_d_n8;
        locals.var_q1q_dn9 = assign4950_e4744_d_n9;
        locals.var_q1q_dn10 = assign4950_e4744_d_n10;
        locals.var_q1q_rv = 0.0;

        let assign4960_e4750: f64 = (locals.var_n0 + locals.var_nb);
        let assign4960_e4751: f64 = (0.5 * assign4960_e4750);
        let assign4960_e4752: f64 = (1.0 + assign4960_e4751);
        let assign4960_e4753: f64 = (locals.var_q1q * assign4960_e4752);
        locals.var_qbq = assign4960_e4753;
        locals.var_qbq_dn0 = ((locals.var_q1q_dn0 * assign4960_e4752) + (locals.var_q1q * (0.5 * (locals.var_n0_dn0 + locals.var_nb_dn0))));
        locals.var_qbq_dn1 = ((locals.var_q1q_dn1 * assign4960_e4752) + (locals.var_q1q * (0.5 * (locals.var_n0_dn1 + locals.var_nb_dn1))));
        locals.var_qbq_dn3 = ((locals.var_q1q_dn3 * assign4960_e4752) + (locals.var_q1q * (0.5 * (locals.var_n0_dn3 + locals.var_nb_dn3))));
        locals.var_qbq_dn4 = ((locals.var_q1q_dn4 * assign4960_e4752) + (locals.var_q1q * (0.5 * (locals.var_n0_dn4 + locals.var_nb_dn4))));
        locals.var_qbq_dn5 = ((locals.var_q1q_dn5 * assign4960_e4752) + (locals.var_q1q * (0.5 * (locals.var_n0_dn5 + locals.var_nb_dn5))));
        locals.var_qbq_dn6 = ((locals.var_q1q_dn6 * assign4960_e4752) + (locals.var_q1q * (0.5 * (locals.var_n0_dn6 + locals.var_nb_dn6))));
        locals.var_qbq_dn7 = ((locals.var_q1q_dn7 * assign4960_e4752) + (locals.var_q1q * (0.5 * (locals.var_n0_dn7 + locals.var_nb_dn7))));
        locals.var_qbq_dn8 = ((locals.var_q1q_dn8 * assign4960_e4752) + (locals.var_q1q * (0.5 * (locals.var_n0_dn8 + locals.var_nb_dn8))));
        locals.var_qbq_dn9 = ((locals.var_q1q_dn9 * assign4960_e4752) + (locals.var_q1q * (0.5 * (locals.var_n0_dn9 + locals.var_nb_dn9))));
        locals.var_qbq_dn10 = ((locals.var_q1q_dn10 * assign4960_e4752) + (locals.var_q1q * (0.5 * (locals.var_n0_dn10 + locals.var_nb_dn10))));
        locals.var_qbq_rv = 0.0;

        let assign4970_e4756: f64 = (locals.var_rbv_t / locals.var_qbq);
        locals.var_rbvtemp = assign4970_e4756;
        locals.var_rbvtemp_dn0 = (-((locals.var_rbv_t * locals.var_qbq_dn0) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn1 = (-((locals.var_rbv_t * locals.var_qbq_dn1) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn3 = (((locals.var_rbv_t_dn3 * locals.var_qbq) - (locals.var_rbv_t * locals.var_qbq_dn3)) / (locals.var_qbq * locals.var_qbq));
        locals.var_rbvtemp_dn4 = (-((locals.var_rbv_t * locals.var_qbq_dn4) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn5 = (-((locals.var_rbv_t * locals.var_qbq_dn5) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn6 = (-((locals.var_rbv_t * locals.var_qbq_dn6) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn7 = (-((locals.var_rbv_t * locals.var_qbq_dn7) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn8 = (-((locals.var_rbv_t * locals.var_qbq_dn8) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn9 = (-((locals.var_rbv_t * locals.var_qbq_dn9) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn10 = (-((locals.var_rbv_t * locals.var_qbq_dn10) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_rv = 0.0;

        let assign4980_e4759: f64 = if locals.var_rbvtemp < locals.var_minr_m { 1.0 } else { 0.0 };
        locals.var_guard84 = assign4980_e4759;
        locals.var_guard84_rv = 0.0;

        let (assign4990_e4763, assign4990_e4763_d_n0, assign4990_e4763_d_n1, assign4990_e4763_d_n3, assign4990_e4763_d_n4, assign4990_e4763_d_n5, assign4990_e4763_d_n6, assign4990_e4763_d_n7, assign4990_e4763_d_n8, assign4990_e4763_d_n9, assign4990_e4763_d_n10,) = {
    if (locals.var_guard84 != 0.0) {
        (locals.var_minr_m, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rbvtemp, locals.var_rbvtemp_dn0, locals.var_rbvtemp_dn1, locals.var_rbvtemp_dn3, locals.var_rbvtemp_dn4, locals.var_rbvtemp_dn5, locals.var_rbvtemp_dn6, locals.var_rbvtemp_dn7, locals.var_rbvtemp_dn8, locals.var_rbvtemp_dn9, locals.var_rbvtemp_dn10,)
    }
};
        locals.var_rbvtemp = assign4990_e4763;
        locals.var_rbvtemp_dn0 = assign4990_e4763_d_n0;
        locals.var_rbvtemp_dn1 = assign4990_e4763_d_n1;
        locals.var_rbvtemp_dn3 = assign4990_e4763_d_n3;
        locals.var_rbvtemp_dn4 = assign4990_e4763_d_n4;
        locals.var_rbvtemp_dn5 = assign4990_e4763_d_n5;
        locals.var_rbvtemp_dn6 = assign4990_e4763_d_n6;
        locals.var_rbvtemp_dn7 = assign4990_e4763_d_n7;
        locals.var_rbvtemp_dn8 = assign4990_e4763_d_n8;
        locals.var_rbvtemp_dn9 = assign4990_e4763_d_n9;
        locals.var_rbvtemp_dn10 = assign4990_e4763_d_n10;
        locals.var_rbvtemp_rv = 0.0;

        let assign5000_e4766: f64 = (3.0 * locals.var_rbvtemp);
        locals.var_rb2 = assign5000_e4766;
        locals.var_rb2_dn0 = (3.0 * locals.var_rbvtemp_dn0);
        locals.var_rb2_dn1 = (3.0 * locals.var_rbvtemp_dn1);
        locals.var_rb2_dn3 = (3.0 * locals.var_rbvtemp_dn3);
        locals.var_rb2_dn4 = (3.0 * locals.var_rbvtemp_dn4);
        locals.var_rb2_dn5 = (3.0 * locals.var_rbvtemp_dn5);
        locals.var_rb2_dn6 = (3.0 * locals.var_rbvtemp_dn6);
        locals.var_rb2_dn7 = (3.0 * locals.var_rbvtemp_dn7);
        locals.var_rb2_dn8 = (3.0 * locals.var_rbvtemp_dn8);
        locals.var_rb2_dn9 = (3.0 * locals.var_rbvtemp_dn9);
        locals.var_rb2_dn10 = (3.0 * locals.var_rbvtemp_dn10);
        locals.var_rb2_rv = 0.0;

        let assign5020_e4780: f64 = if locals.var_in_ > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign5020_e4780;
        locals.var_guard85_rv = 0.0;

        let assign5030_e4783: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign5030_e4783;
        locals.var_guard86_rv = 0.0;

        let assign5040_e4786: f64 = if locals.var_vb2c1 < p.p43 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign5040_e4786;
        locals.var_guard87_rv = 0.0;

        let assign5050_e4788: f64 = (-locals.var_in_);
        let assign5050_e4790: f64 = (assign5050_e4788 / p.p41);
        let assign5050_e4792: f64 = if assign5050_e4790 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard88 = assign5050_e4792;
        locals.var_guard88_rv = 0.0;

        let (assign5060_e4806, assign5060_e4806_d_n0, assign5060_e4806_d_n1, assign5060_e4806_d_n3, assign5060_e4806_d_n4, assign5060_e4806_d_n5, assign5060_e4806_d_n6, assign5060_e4806_d_n7, assign5060_e4806_d_n8, assign5060_e4806_d_n9, assign5060_e4806_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) && (locals.var_guard88 != 0.0)) {
        let assign5060_e4801: f64 = (-locals.var_in_);
        let assign5060_e4803: f64 = (assign5060_e4801 / p.p41);
        let assign5060_e4804: f64 = (assign5060_e4803).exp();
        (assign5060_e4804, (assign5060_e4804 * ((-locals.var_in__dn0) / p.p41)), (assign5060_e4804 * ((-locals.var_in__dn1) / p.p41)), (assign5060_e4804 * ((-locals.var_in__dn3) / p.p41)), (assign5060_e4804 * ((-locals.var_in__dn4) / p.p41)), (assign5060_e4804 * ((-locals.var_in__dn5) / p.p41)), (assign5060_e4804 * ((-locals.var_in__dn6) / p.p41)), (assign5060_e4804 * ((-locals.var_in__dn7) / p.p41)), (assign5060_e4804 * ((-locals.var_in__dn8) / p.p41)), (assign5060_e4804 * ((-locals.var_in__dn9) / p.p41)), (assign5060_e4804 * ((-locals.var_in__dn10) / p.p41)),)
    } else {
        (locals.var_expin, locals.var_expin_dn0, locals.var_expin_dn1, locals.var_expin_dn3, locals.var_expin_dn4, locals.var_expin_dn5, locals.var_expin_dn6, locals.var_expin_dn7, locals.var_expin_dn8, locals.var_expin_dn9, locals.var_expin_dn10,)
    }
};
        locals.var_expin = assign5060_e4806;
        locals.var_expin_dn0 = assign5060_e4806_d_n0;
        locals.var_expin_dn1 = assign5060_e4806_d_n1;
        locals.var_expin_dn3 = assign5060_e4806_d_n3;
        locals.var_expin_dn4 = assign5060_e4806_d_n4;
        locals.var_expin_dn5 = assign5060_e4806_d_n5;
        locals.var_expin_dn6 = assign5060_e4806_d_n6;
        locals.var_expin_dn7 = assign5060_e4806_d_n7;
        locals.var_expin_dn8 = assign5060_e4806_d_n8;
        locals.var_expin_dn9 = assign5060_e4806_d_n9;
        locals.var_expin_dn10 = assign5060_e4806_d_n10;
        locals.var_expin_rv = 0.0;

        let (assign5070_e4818,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) && (locals.var_guard88 == 0.0)) {
        let assign5070_e4816: f64 = (p.p138).exp();
        (assign5070_e4816,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5070_e4818;
        locals.var_expl_rv = 0.0;

        let (assign5080_e4838, assign5080_e4838_d_n0, assign5080_e4838_d_n1, assign5080_e4838_d_n3, assign5080_e4838_d_n4, assign5080_e4838_d_n5, assign5080_e4838_d_n6, assign5080_e4838_d_n7, assign5080_e4838_d_n8, assign5080_e4838_d_n9, assign5080_e4838_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) && (locals.var_guard88 == 0.0)) {
        let assign5080_e4830: f64 = (-locals.var_in_);
        let assign5080_e4832: f64 = (assign5080_e4830 / p.p41);
        let assign5080_e4834: f64 = (assign5080_e4832 - p.p138);
        let assign5080_e4835: f64 = (1.0 + assign5080_e4834);
        let assign5080_e4836: f64 = (locals.var_expl * assign5080_e4835);
        (assign5080_e4836, (locals.var_expl * ((-locals.var_in__dn0) / p.p41)), (locals.var_expl * ((-locals.var_in__dn1) / p.p41)), (locals.var_expl * ((-locals.var_in__dn3) / p.p41)), (locals.var_expl * ((-locals.var_in__dn4) / p.p41)), (locals.var_expl * ((-locals.var_in__dn5) / p.p41)), (locals.var_expl * ((-locals.var_in__dn6) / p.p41)), (locals.var_expl * ((-locals.var_in__dn7) / p.p41)), (locals.var_expl * ((-locals.var_in__dn8) / p.p41)), (locals.var_expl * ((-locals.var_in__dn9) / p.p41)), (locals.var_expl * ((-locals.var_in__dn10) / p.p41)),)
    } else {
        (locals.var_expin, locals.var_expin_dn0, locals.var_expin_dn1, locals.var_expin_dn3, locals.var_expin_dn4, locals.var_expin_dn5, locals.var_expin_dn6, locals.var_expin_dn7, locals.var_expin_dn8, locals.var_expin_dn9, locals.var_expin_dn10,)
    }
};
        locals.var_expin = assign5080_e4838;
        locals.var_expin_dn0 = assign5080_e4838_d_n0;
        locals.var_expin_dn1 = assign5080_e4838_d_n1;
        locals.var_expin_dn3 = assign5080_e4838_d_n3;
        locals.var_expin_dn4 = assign5080_e4838_d_n4;
        locals.var_expin_dn5 = assign5080_e4838_d_n5;
        locals.var_expin_dn6 = assign5080_e4838_d_n6;
        locals.var_expin_dn7 = assign5080_e4838_d_n7;
        locals.var_expin_dn8 = assign5080_e4838_d_n8;
        locals.var_expin_dn9 = assign5080_e4838_d_n9;
        locals.var_expin_dn10 = assign5080_e4838_d_n10;
        locals.var_expin_rv = 0.0;

        let (assign5090_e4850, assign5090_e4850_d_n0, assign5090_e4850_d_n1, assign5090_e4850_d_n3, assign5090_e4850_d_n4, assign5090_e4850_d_n5, assign5090_e4850_d_n6, assign5090_e4850_d_n7, assign5090_e4850_d_n8, assign5090_e4850_d_n9, assign5090_e4850_d_n10,) = {
    if (((locals.var_guard85 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) {
        let assign5090_e4846: f64 = (p.p43 - locals.var_vb2c1);
        let assign5090_e4848: f64 = (assign5090_e4846 * locals.var_expin);
        (assign5090_e4848, (assign5090_e4846 * locals.var_expin_dn0), (assign5090_e4846 * locals.var_expin_dn1), (assign5090_e4846 * locals.var_expin_dn3), (assign5090_e4846 * locals.var_expin_dn4), (assign5090_e4846 * locals.var_expin_dn5), (((-locals.var_vb2c1_dn6) * locals.var_expin) + (assign5090_e4846 * locals.var_expin_dn6)), (((-locals.var_vb2c1_dn7) * locals.var_expin) + (assign5090_e4846 * locals.var_expin_dn7)), (assign5090_e4846 * locals.var_expin_dn8), (assign5090_e4846 * locals.var_expin_dn9), (assign5090_e4846 * locals.var_expin_dn10),)
    } else {
        (locals.var_vl, locals.var_vl_dn0, locals.var_vl_dn1, locals.var_vl_dn3, locals.var_vl_dn4, locals.var_vl_dn5, locals.var_vl_dn6, locals.var_vl_dn7, locals.var_vl_dn8, locals.var_vl_dn9, locals.var_vl_dn10,)
    }
};
        locals.var_vl = assign5090_e4850;
        locals.var_vl_dn0 = assign5090_e4850_d_n0;
        locals.var_vl_dn1 = assign5090_e4850_d_n1;
        locals.var_vl_dn3 = assign5090_e4850_d_n3;
        locals.var_vl_dn4 = assign5090_e4850_d_n4;
        locals.var_vl_dn5 = assign5090_e4850_d_n5;
        locals.var_vl_dn6 = assign5090_e4850_d_n6;
        locals.var_vl_dn7 = assign5090_e4850_d_n7;
        locals.var_vl_dn8 = assign5090_e4850_d_n8;
        locals.var_vl_dn9 = assign5090_e4850_d_n9;
        locals.var_vl_dn10 = assign5090_e4850_d_n10;
        locals.var_vl_rv = 0.0;

        let assign5100_e4852: f64 = (-locals.var_bavl_t);
        let assign5100_e4855: f64 = (locals.var_vl).powf(p.p40);
        let assign5100_e4856: f64 = (assign5100_e4852 * assign5100_e4855);
        let assign5100_e4858: f64 = if assign5100_e4856 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign5100_e4858;
        locals.var_guard89_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5110_e4874, assign5110_e4874_d_n0, assign5110_e4874_d_n1, assign5110_e4874_d_n3, assign5110_e4874_d_n4, assign5110_e4874_d_n5, assign5110_e4874_d_n6, assign5110_e4874_d_n7, assign5110_e4874_d_n8, assign5110_e4874_d_n9, assign5110_e4874_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5110_e4867: f64 = (-locals.var_bavl_t);
        let assign5110_e4870: f64 = (locals.var_vl).powf(p.p40);
        let assign5110_e4871: f64 = (assign5110_e4867 * assign5110_e4870);
        let assign5110_e4872: f64 = (assign5110_e4871).exp();
        (assign5110_e4872, (assign5110_e4872 * (((-locals.var_bavl_t_dn0) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn0)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn0 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn1) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn1)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn1 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn3) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn3)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn3 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn4) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn4)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn4 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn5) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn5)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn5 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn6) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn6)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn6 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn7) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn7)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn7 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn8) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn8)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn8 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn9) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn9)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn9 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn10) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn10)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn10 / locals.var_vl))) }))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10,)
    }
};
        locals.var_expmm1 = assign5110_e4874;
        locals.var_expmm1_dn0 = assign5110_e4874_d_n0;
        locals.var_expmm1_dn1 = assign5110_e4874_d_n1;
        locals.var_expmm1_dn3 = assign5110_e4874_d_n3;
        locals.var_expmm1_dn4 = assign5110_e4874_d_n4;
        locals.var_expmm1_dn5 = assign5110_e4874_d_n5;
        locals.var_expmm1_dn6 = assign5110_e4874_d_n6;
        locals.var_expmm1_dn7 = assign5110_e4874_d_n7;
        locals.var_expmm1_dn8 = assign5110_e4874_d_n8;
        locals.var_expmm1_dn9 = assign5110_e4874_d_n9;
        locals.var_expmm1_dn10 = assign5110_e4874_d_n10;
        locals.var_expmm1_rv = 0.0;

        let (assign5120_e4886,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) && (locals.var_guard89 == 0.0)) {
        let assign5120_e4884: f64 = (p.p138).exp();
        (assign5120_e4884,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5120_e4886;
        locals.var_expl_rv = 0.0;

        let (assign5130_e4908, assign5130_e4908_d_n0, assign5130_e4908_d_n1, assign5130_e4908_d_n3, assign5130_e4908_d_n4, assign5130_e4908_d_n5, assign5130_e4908_d_n6, assign5130_e4908_d_n7, assign5130_e4908_d_n8, assign5130_e4908_d_n9, assign5130_e4908_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) && (locals.var_guard89 == 0.0)) {
        let assign5130_e4898: f64 = (-locals.var_bavl_t);
        let assign5130_e4901: f64 = (locals.var_vl).powf(p.p40);
        let assign5130_e4902: f64 = (assign5130_e4898 * assign5130_e4901);
        let assign5130_e4904: f64 = (assign5130_e4902 - p.p138);
        let assign5130_e4905: f64 = (1.0 + assign5130_e4904);
        let assign5130_e4906: f64 = (locals.var_expl * assign5130_e4905);
        (assign5130_e4906, (locals.var_expl * (((-locals.var_bavl_t_dn0) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn0)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn0 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn1) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn1)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn1 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn3) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn3)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn3 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn4) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn4)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn4 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn5) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn5)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn5 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn6) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn6)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn6 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn7) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn7)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn7 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn8) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn8)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn8 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn9) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn9)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn9 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn10) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn10)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn10 / locals.var_vl))) }))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10,)
    }
};
        locals.var_expmm1 = assign5130_e4908;
        locals.var_expmm1_dn0 = assign5130_e4908_d_n0;
        locals.var_expmm1_dn1 = assign5130_e4908_d_n1;
        locals.var_expmm1_dn3 = assign5130_e4908_d_n3;
        locals.var_expmm1_dn4 = assign5130_e4908_d_n4;
        locals.var_expmm1_dn5 = assign5130_e4908_d_n5;
        locals.var_expmm1_dn6 = assign5130_e4908_d_n6;
        locals.var_expmm1_dn7 = assign5130_e4908_d_n7;
        locals.var_expmm1_dn8 = assign5130_e4908_d_n8;
        locals.var_expmm1_dn9 = assign5130_e4908_d_n9;
        locals.var_expmm1_dn10 = assign5130_e4908_d_n10;
        locals.var_expmm1_rv = 0.0;

        let (assign5140_e4922, assign5140_e4922_d_n0, assign5140_e4922_d_n1, assign5140_e4922_d_n3, assign5140_e4922_d_n4, assign5140_e4922_d_n5, assign5140_e4922_d_n6, assign5140_e4922_d_n7, assign5140_e4922_d_n8, assign5140_e4922_d_n9, assign5140_e4922_d_n10,) = {
    if (((locals.var_guard85 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) {
        let assign5140_e4916: f64 = (p.p39 / locals.var_bavl_t);
        let assign5140_e4918: f64 = (assign5140_e4916 * locals.var_vl);
        let assign5140_e4920: f64 = (assign5140_e4918 * locals.var_expmm1);
        (assign5140_e4920, (((((-((p.p39 * locals.var_bavl_t_dn0) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn0)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn0)), (((((-((p.p39 * locals.var_bavl_t_dn1) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn1)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn1)), (((((-((p.p39 * locals.var_bavl_t_dn3) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn3)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn3)), (((((-((p.p39 * locals.var_bavl_t_dn4) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn4)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn4)), (((((-((p.p39 * locals.var_bavl_t_dn5) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn5)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn5)), (((((-((p.p39 * locals.var_bavl_t_dn6) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn6)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn6)), (((((-((p.p39 * locals.var_bavl_t_dn7) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn7)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn7)), (((((-((p.p39 * locals.var_bavl_t_dn8) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn8)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn8)), (((((-((p.p39 * locals.var_bavl_t_dn9) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn9)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn9)), (((((-((p.p39 * locals.var_bavl_t_dn10) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn10)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn10)),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10,)
    }
};
        locals.var_gem = assign5140_e4922;
        locals.var_gem_dn0 = assign5140_e4922_d_n0;
        locals.var_gem_dn1 = assign5140_e4922_d_n1;
        locals.var_gem_dn3 = assign5140_e4922_d_n3;
        locals.var_gem_dn4 = assign5140_e4922_d_n4;
        locals.var_gem_dn5 = assign5140_e4922_d_n5;
        locals.var_gem_dn6 = assign5140_e4922_d_n6;
        locals.var_gem_dn7 = assign5140_e4922_d_n7;
        locals.var_gem_dn8 = assign5140_e4922_d_n8;
        locals.var_gem_dn9 = assign5140_e4922_d_n9;
        locals.var_gem_dn10 = assign5140_e4922_d_n10;
        locals.var_gem_rv = 0.0;

        let assign5150_e4925: f64 = if p.p38 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign5150_e4925;
        locals.var_guard90_rv = 0.0;

        let assign5160_e4928: f64 = if locals.var_vb2c1 < locals.var_vdc_t { 1.0 } else { 0.0 };
        locals.var_guard91 = assign5160_e4928;
        locals.var_guard91_rv = 0.0;

        let (assign5170_e4945,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5170_e4939: f64 = (2.0 * p.p45);
        let assign5170_e4942: f64 = (p.p44 * p.p44);
        let assign5170_e4943: f64 = (assign5170_e4939 / assign5170_e4942);
        (assign5170_e4943,)
    } else {
        (locals.var_dedx0,)
    }
};
        locals.var_dedx0 = assign5170_e4945;
        locals.var_dedx0_rv = 0.0;

        let (assign5180_e4960, assign5180_e4960_d_n0, assign5180_e4960_d_n1, assign5180_e4960_d_n3, assign5180_e4960_d_n4, assign5180_e4960_d_n5, assign5180_e4960_d_n6, assign5180_e4960_d_n7, assign5180_e4960_d_n8, assign5180_e4960_d_n9, assign5180_e4960_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5180_e4956: f64 = (locals.var_vdc_t - locals.var_vb2c1);
        let assign5180_e4958: f64 = (assign5180_e4956 / locals.var_icap_ihc);
        (assign5180_e4958, (((locals.var_vdc_t_dn0 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn0)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn1 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn1)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn3 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn3)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn4 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn4)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn5 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn5)) / (locals.var_icap_ihc * locals.var_icap_ihc)), ((((locals.var_vdc_t_dn6 - locals.var_vb2c1_dn6) * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn6)) / (locals.var_icap_ihc * locals.var_icap_ihc)), ((((locals.var_vdc_t_dn7 - locals.var_vb2c1_dn7) * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn7)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn8 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn8)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn9 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn9)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn10 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn10)) / (locals.var_icap_ihc * locals.var_icap_ihc)),)
    } else {
        (locals.var_sqr_arg, locals.var_sqr_arg_dn0, locals.var_sqr_arg_dn1, locals.var_sqr_arg_dn3, locals.var_sqr_arg_dn4, locals.var_sqr_arg_dn5, locals.var_sqr_arg_dn6, locals.var_sqr_arg_dn7, locals.var_sqr_arg_dn8, locals.var_sqr_arg_dn9, locals.var_sqr_arg_dn10,)
    }
};
        locals.var_sqr_arg = assign5180_e4960;
        locals.var_sqr_arg_dn0 = assign5180_e4960_d_n0;
        locals.var_sqr_arg_dn1 = assign5180_e4960_d_n1;
        locals.var_sqr_arg_dn3 = assign5180_e4960_d_n3;
        locals.var_sqr_arg_dn4 = assign5180_e4960_d_n4;
        locals.var_sqr_arg_dn5 = assign5180_e4960_d_n5;
        locals.var_sqr_arg_dn6 = assign5180_e4960_d_n6;
        locals.var_sqr_arg_dn7 = assign5180_e4960_d_n7;
        locals.var_sqr_arg_dn8 = assign5180_e4960_d_n8;
        locals.var_sqr_arg_dn9 = assign5180_e4960_d_n9;
        locals.var_sqr_arg_dn10 = assign5180_e4960_d_n10;
        locals.var_sqr_arg_rv = 0.0;

        let (assign5190_e4976, assign5190_e4976_d_n0, assign5190_e4976_d_n1, assign5190_e4976_d_n3, assign5190_e4976_d_n4, assign5190_e4976_d_n5, assign5190_e4976_d_n6, assign5190_e4976_d_n7, assign5190_e4976_d_n8, assign5190_e4976_d_n9, assign5190_e4976_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5190_e4971: f64 = (2.0 * locals.var_sqr_arg);
        let assign5190_e4973: f64 = (assign5190_e4971 / locals.var_dedx0);
        let assign5190_e4974: f64 = (assign5190_e4973).sqrt();
        (assign5190_e4974, (((2.0 * locals.var_sqr_arg_dn0) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn1) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn3) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn4) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn5) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn6) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn7) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn8) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn9) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn10) / locals.var_dedx0) / (2.0 * assign5190_e4974)),)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn1, locals.var_xd_dn3, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10,)
    }
};
        locals.var_xd = assign5190_e4976;
        locals.var_xd_dn0 = assign5190_e4976_d_n0;
        locals.var_xd_dn1 = assign5190_e4976_d_n1;
        locals.var_xd_dn3 = assign5190_e4976_d_n3;
        locals.var_xd_dn4 = assign5190_e4976_d_n4;
        locals.var_xd_dn5 = assign5190_e4976_d_n5;
        locals.var_xd_dn6 = assign5190_e4976_d_n6;
        locals.var_xd_dn7 = assign5190_e4976_d_n7;
        locals.var_xd_dn8 = assign5190_e4976_d_n8;
        locals.var_xd_dn9 = assign5190_e4976_d_n9;
        locals.var_xd_dn10 = assign5190_e4976_d_n10;
        locals.var_xd_rv = 0.0;

        let assign5200_e4979: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign5200_e4979;
        locals.var_guard92_rv = 0.0;

        let (assign5210_e4992, assign5210_e4992_d_n0, assign5210_e4992_d_n1, assign5210_e4992_d_n3, assign5210_e4992_d_n4, assign5210_e4992_d_n5, assign5210_e4992_d_n6, assign5210_e4992_d_n7, assign5210_e4992_d_n8, assign5210_e4992_d_n9, assign5210_e4992_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard92 != 0.0)) {
        (p.p44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_weff, locals.var_weff_dn0, locals.var_weff_dn1, locals.var_weff_dn3, locals.var_weff_dn4, locals.var_weff_dn5, locals.var_weff_dn6, locals.var_weff_dn7, locals.var_weff_dn8, locals.var_weff_dn9, locals.var_weff_dn10,)
    }
};
        locals.var_weff = assign5210_e4992;
        locals.var_weff_dn0 = assign5210_e4992_d_n0;
        locals.var_weff_dn1 = assign5210_e4992_d_n1;
        locals.var_weff_dn3 = assign5210_e4992_d_n3;
        locals.var_weff_dn4 = assign5210_e4992_d_n4;
        locals.var_weff_dn5 = assign5210_e4992_d_n5;
        locals.var_weff_dn6 = assign5210_e4992_d_n6;
        locals.var_weff_dn7 = assign5210_e4992_d_n7;
        locals.var_weff_dn8 = assign5210_e4992_d_n8;
        locals.var_weff_dn9 = assign5210_e4992_d_n9;
        locals.var_weff_dn10 = assign5210_e4992_d_n10;
        locals.var_weff_rv = 0.0;

        let (assign5220_e5010, assign5220_e5010_d_n0, assign5220_e5010_d_n1, assign5220_e5010_d_n3, assign5220_e5010_d_n4, assign5220_e5010_d_n5, assign5220_e5010_d_n6, assign5220_e5010_d_n7, assign5220_e5010_d_n8, assign5220_e5010_d_n9, assign5220_e5010_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard92 == 0.0)) {
        let assign5220_e5007: f64 = (0.5 * locals.var_xi_w);
        let assign5220_e5008: f64 = (1.0 - assign5220_e5007);
        (assign5220_e5008, (-(0.5 * locals.var_xi_w_dn0)), (-(0.5 * locals.var_xi_w_dn1)), (-(0.5 * locals.var_xi_w_dn3)), (-(0.5 * locals.var_xi_w_dn4)), (-(0.5 * locals.var_xi_w_dn5)), (-(0.5 * locals.var_xi_w_dn6)), (-(0.5 * locals.var_xi_w_dn7)), (-(0.5 * locals.var_xi_w_dn8)), (-(0.5 * locals.var_xi_w_dn9)), (-(0.5 * locals.var_xi_w_dn10)),)
    } else {
        (locals.var_xi_w1, locals.var_xi_w1_dn0, locals.var_xi_w1_dn1, locals.var_xi_w1_dn3, locals.var_xi_w1_dn4, locals.var_xi_w1_dn5, locals.var_xi_w1_dn6, locals.var_xi_w1_dn7, locals.var_xi_w1_dn8, locals.var_xi_w1_dn9, locals.var_xi_w1_dn10,)
    }
};
        locals.var_xi_w1 = assign5220_e5010;
        locals.var_xi_w1_dn0 = assign5220_e5010_d_n0;
        locals.var_xi_w1_dn1 = assign5220_e5010_d_n1;
        locals.var_xi_w1_dn3 = assign5220_e5010_d_n3;
        locals.var_xi_w1_dn4 = assign5220_e5010_d_n4;
        locals.var_xi_w1_dn5 = assign5220_e5010_d_n5;
        locals.var_xi_w1_dn6 = assign5220_e5010_d_n6;
        locals.var_xi_w1_dn7 = assign5220_e5010_d_n7;
        locals.var_xi_w1_dn8 = assign5220_e5010_d_n8;
        locals.var_xi_w1_dn9 = assign5220_e5010_d_n9;
        locals.var_xi_w1_dn10 = assign5220_e5010_d_n10;
        locals.var_xi_w1_rv = 0.0;

        let (assign5230_e5028, assign5230_e5028_d_n0, assign5230_e5028_d_n1, assign5230_e5028_d_n3, assign5230_e5028_d_n4, assign5230_e5028_d_n5, assign5230_e5028_d_n6, assign5230_e5028_d_n7, assign5230_e5028_d_n8, assign5230_e5028_d_n9, assign5230_e5028_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard92 == 0.0)) {
        let assign5230_e5024: f64 = (p.p44 * locals.var_xi_w1);
        let assign5230_e5026: f64 = (assign5230_e5024 * locals.var_xi_w1);
        (assign5230_e5026, (((p.p44 * locals.var_xi_w1_dn0) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn0)), (((p.p44 * locals.var_xi_w1_dn1) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn1)), (((p.p44 * locals.var_xi_w1_dn3) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn3)), (((p.p44 * locals.var_xi_w1_dn4) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn4)), (((p.p44 * locals.var_xi_w1_dn5) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn5)), (((p.p44 * locals.var_xi_w1_dn6) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn6)), (((p.p44 * locals.var_xi_w1_dn7) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn7)), (((p.p44 * locals.var_xi_w1_dn8) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn8)), (((p.p44 * locals.var_xi_w1_dn9) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn9)), (((p.p44 * locals.var_xi_w1_dn10) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn10)),)
    } else {
        (locals.var_weff, locals.var_weff_dn0, locals.var_weff_dn1, locals.var_weff_dn3, locals.var_weff_dn4, locals.var_weff_dn5, locals.var_weff_dn6, locals.var_weff_dn7, locals.var_weff_dn8, locals.var_weff_dn9, locals.var_weff_dn10,)
    }
};
        locals.var_weff = assign5230_e5028;
        locals.var_weff_dn0 = assign5230_e5028_d_n0;
        locals.var_weff_dn1 = assign5230_e5028_d_n1;
        locals.var_weff_dn3 = assign5230_e5028_d_n3;
        locals.var_weff_dn4 = assign5230_e5028_d_n4;
        locals.var_weff_dn5 = assign5230_e5028_d_n5;
        locals.var_weff_dn6 = assign5230_e5028_d_n6;
        locals.var_weff_dn7 = assign5230_e5028_d_n7;
        locals.var_weff_dn8 = assign5230_e5028_d_n8;
        locals.var_weff_dn9 = assign5230_e5028_d_n9;
        locals.var_weff_dn10 = assign5230_e5028_d_n10;
        locals.var_weff_rv = 0.0;

        let (assign5240_e5050, assign5240_e5050_d_n0, assign5240_e5050_d_n1, assign5240_e5050_d_n3, assign5240_e5050_d_n4, assign5240_e5050_d_n5, assign5240_e5050_d_n6, assign5240_e5050_d_n7, assign5240_e5050_d_n8, assign5240_e5050_d_n9, assign5240_e5050_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5240_e5039: f64 = (locals.var_xd * locals.var_weff);
        let assign5240_e5042: f64 = (locals.var_xd * locals.var_xd);
        let assign5240_e5045: f64 = (locals.var_weff * locals.var_weff);
        let assign5240_e5046: f64 = (assign5240_e5042 + assign5240_e5045);
        let assign5240_e5047: f64 = (assign5240_e5046).sqrt();
        let assign5240_e5048: f64 = (assign5240_e5039 / assign5240_e5047);
        (assign5240_e5048, (((((locals.var_xd_dn0 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn0)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn0 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn0)) + ((locals.var_weff_dn0 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn0))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn1 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn1)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn1 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn1)) + ((locals.var_weff_dn1 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn1))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn3 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn3)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn3 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn3)) + ((locals.var_weff_dn3 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn3))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn4 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn4)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn4 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn4)) + ((locals.var_weff_dn4 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn4))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn5 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn5)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn5 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn5)) + ((locals.var_weff_dn5 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn5))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn6 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn6)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn6 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn6)) + ((locals.var_weff_dn6 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn6))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn7 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn7)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn7 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn7)) + ((locals.var_weff_dn7 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn7))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn8 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn8)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn8 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn8)) + ((locals.var_weff_dn8 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn8))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn9 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn9)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn9 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn9)) + ((locals.var_weff_dn9 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn9))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn10 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn10)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn10 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn10)) + ((locals.var_weff_dn10 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn10))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)),)
    } else {
        (locals.var_wd, locals.var_wd_dn0, locals.var_wd_dn1, locals.var_wd_dn3, locals.var_wd_dn4, locals.var_wd_dn5, locals.var_wd_dn6, locals.var_wd_dn7, locals.var_wd_dn8, locals.var_wd_dn9, locals.var_wd_dn10,)
    }
};
        locals.var_wd = assign5240_e5050;
        locals.var_wd_dn0 = assign5240_e5050_d_n0;
        locals.var_wd_dn1 = assign5240_e5050_d_n1;
        locals.var_wd_dn3 = assign5240_e5050_d_n3;
        locals.var_wd_dn4 = assign5240_e5050_d_n4;
        locals.var_wd_dn5 = assign5240_e5050_d_n5;
        locals.var_wd_dn6 = assign5240_e5050_d_n6;
        locals.var_wd_dn7 = assign5240_e5050_d_n7;
        locals.var_wd_dn8 = assign5240_e5050_d_n8;
        locals.var_wd_dn9 = assign5240_e5050_d_n9;
        locals.var_wd_dn10 = assign5240_e5050_d_n10;
        locals.var_wd_rv = 0.0;

        let (assign5250_e5065, assign5250_e5065_d_n0, assign5250_e5065_d_n1, assign5250_e5065_d_n3, assign5250_e5065_d_n4, assign5250_e5065_d_n5, assign5250_e5065_d_n6, assign5250_e5065_d_n7, assign5250_e5065_d_n8, assign5250_e5065_d_n9, assign5250_e5065_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5250_e5061: f64 = (locals.var_vdc_t - locals.var_vb2c1);
        let assign5250_e5063: f64 = (assign5250_e5061 / locals.var_wd);
        (assign5250_e5063, (((locals.var_vdc_t_dn0 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn0)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn1 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn1)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn3 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn3)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn4 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn4)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn5 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn5)) / (locals.var_wd * locals.var_wd)), ((((locals.var_vdc_t_dn6 - locals.var_vb2c1_dn6) * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn6)) / (locals.var_wd * locals.var_wd)), ((((locals.var_vdc_t_dn7 - locals.var_vb2c1_dn7) * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn7)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn8 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn8)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn9 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn9)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn10 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn10)) / (locals.var_wd * locals.var_wd)),)
    } else {
        (locals.var_eav, locals.var_eav_dn0, locals.var_eav_dn1, locals.var_eav_dn3, locals.var_eav_dn4, locals.var_eav_dn5, locals.var_eav_dn6, locals.var_eav_dn7, locals.var_eav_dn8, locals.var_eav_dn9, locals.var_eav_dn10,)
    }
};
        locals.var_eav = assign5250_e5065;
        locals.var_eav_dn0 = assign5250_e5065_d_n0;
        locals.var_eav_dn1 = assign5250_e5065_d_n1;
        locals.var_eav_dn3 = assign5250_e5065_d_n3;
        locals.var_eav_dn4 = assign5250_e5065_d_n4;
        locals.var_eav_dn5 = assign5250_e5065_d_n5;
        locals.var_eav_dn6 = assign5250_e5065_d_n6;
        locals.var_eav_dn7 = assign5250_e5065_d_n7;
        locals.var_eav_dn8 = assign5250_e5065_d_n8;
        locals.var_eav_dn9 = assign5250_e5065_d_n9;
        locals.var_eav_dn10 = assign5250_e5065_d_n10;
        locals.var_eav_rv = 0.0;

        let (assign5260_e5084, assign5260_e5084_d_n0, assign5260_e5084_d_n1, assign5260_e5084_d_n3, assign5260_e5084_d_n4, assign5260_e5084_d_n5, assign5260_e5084_d_n6, assign5260_e5084_d_n7, assign5260_e5084_d_n8, assign5260_e5084_d_n9, assign5260_e5084_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5260_e5077: f64 = (0.5 * locals.var_wd);
        let assign5260_e5079: f64 = (assign5260_e5077 * locals.var_dedx0);
        let assign5260_e5081: f64 = (assign5260_e5079 * locals.var_icap_ihc);
        let assign5260_e5082: f64 = (locals.var_eav + assign5260_e5081);
        (assign5260_e5082, (locals.var_eav_dn0 + ((((0.5 * locals.var_wd_dn0) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn0))), (locals.var_eav_dn1 + ((((0.5 * locals.var_wd_dn1) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn1))), (locals.var_eav_dn3 + ((((0.5 * locals.var_wd_dn3) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn3))), (locals.var_eav_dn4 + ((((0.5 * locals.var_wd_dn4) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn4))), (locals.var_eav_dn5 + ((((0.5 * locals.var_wd_dn5) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn5))), (locals.var_eav_dn6 + ((((0.5 * locals.var_wd_dn6) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn6))), (locals.var_eav_dn7 + ((((0.5 * locals.var_wd_dn7) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn7))), (locals.var_eav_dn8 + ((((0.5 * locals.var_wd_dn8) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn8))), (locals.var_eav_dn9 + ((((0.5 * locals.var_wd_dn9) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn9))), (locals.var_eav_dn10 + ((((0.5 * locals.var_wd_dn10) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn10))),)
    } else {
        (locals.var_e0, locals.var_e0_dn0, locals.var_e0_dn1, locals.var_e0_dn3, locals.var_e0_dn4, locals.var_e0_dn5, locals.var_e0_dn6, locals.var_e0_dn7, locals.var_e0_dn8, locals.var_e0_dn9, locals.var_e0_dn10,)
    }
};
        locals.var_e0 = assign5260_e5084;
        locals.var_e0_dn0 = assign5260_e5084_d_n0;
        locals.var_e0_dn1 = assign5260_e5084_d_n1;
        locals.var_e0_dn3 = assign5260_e5084_d_n3;
        locals.var_e0_dn4 = assign5260_e5084_d_n4;
        locals.var_e0_dn5 = assign5260_e5084_d_n5;
        locals.var_e0_dn6 = assign5260_e5084_d_n6;
        locals.var_e0_dn7 = assign5260_e5084_d_n7;
        locals.var_e0_dn8 = assign5260_e5084_d_n8;
        locals.var_e0_dn9 = assign5260_e5084_d_n9;
        locals.var_e0_dn10 = assign5260_e5084_d_n10;
        locals.var_e0_rv = 0.0;

        let assign5270_e5087: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign5270_e5087;
        locals.var_guard93_rv = 0.0;

        let (assign5280_e5100, assign5280_e5100_d_n0, assign5280_e5100_d_n1, assign5280_e5100_d_n3, assign5280_e5100_d_n4, assign5280_e5100_d_n5, assign5280_e5100_d_n6, assign5280_e5100_d_n7, assign5280_e5100_d_n8, assign5280_e5100_d_n9, assign5280_e5100_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard93 != 0.0)) {
        (locals.var_e0, locals.var_e0_dn0, locals.var_e0_dn1, locals.var_e0_dn3, locals.var_e0_dn4, locals.var_e0_dn5, locals.var_e0_dn6, locals.var_e0_dn7, locals.var_e0_dn8, locals.var_e0_dn9, locals.var_e0_dn10,)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn1, locals.var_em_dn3, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10,)
    }
};
        locals.var_em = assign5280_e5100;
        locals.var_em_dn0 = assign5280_e5100_d_n0;
        locals.var_em_dn1 = assign5280_e5100_d_n1;
        locals.var_em_dn3 = assign5280_e5100_d_n3;
        locals.var_em_dn4 = assign5280_e5100_d_n4;
        locals.var_em_dn5 = assign5280_e5100_d_n5;
        locals.var_em_dn6 = assign5280_e5100_d_n6;
        locals.var_em_dn7 = assign5280_e5100_d_n7;
        locals.var_em_dn8 = assign5280_e5100_d_n8;
        locals.var_em_dn9 = assign5280_e5100_d_n9;
        locals.var_em_dn10 = assign5280_e5100_d_n10;
        locals.var_em_rv = 0.0;

        let (assign5290_e5124, assign5290_e5124_d_n0, assign5290_e5124_d_n1, assign5290_e5124_d_n3, assign5290_e5124_d_n4, assign5290_e5124_d_n5, assign5290_e5124_d_n6, assign5290_e5124_d_n7, assign5290_e5124_d_n8, assign5290_e5124_d_n9, assign5290_e5124_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard93 == 0.0)) {
        let assign5290_e5115: f64 = (2.0 * p.p46);
        let assign5290_e5119: f64 = (2.0 * locals.var_xi_w);
        let assign5290_e5120: f64 = (1.0 + assign5290_e5119);
        let assign5290_e5121: f64 = (assign5290_e5115 * assign5290_e5120);
        let assign5290_e5122: f64 = (1.0 + assign5290_e5121);
        (assign5290_e5122, (assign5290_e5115 * (2.0 * locals.var_xi_w_dn0)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn1)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn3)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn4)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn5)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn6)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn7)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn8)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn9)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn10)),)
    } else {
        (locals.var_shw, locals.var_shw_dn0, locals.var_shw_dn1, locals.var_shw_dn3, locals.var_shw_dn4, locals.var_shw_dn5, locals.var_shw_dn6, locals.var_shw_dn7, locals.var_shw_dn8, locals.var_shw_dn9, locals.var_shw_dn10,)
    }
};
        locals.var_shw = assign5290_e5124;
        locals.var_shw_dn0 = assign5290_e5124_d_n0;
        locals.var_shw_dn1 = assign5290_e5124_d_n1;
        locals.var_shw_dn3 = assign5290_e5124_d_n3;
        locals.var_shw_dn4 = assign5290_e5124_d_n4;
        locals.var_shw_dn5 = assign5290_e5124_d_n5;
        locals.var_shw_dn6 = assign5290_e5124_d_n6;
        locals.var_shw_dn7 = assign5290_e5124_d_n7;
        locals.var_shw_dn8 = assign5290_e5124_d_n8;
        locals.var_shw_dn9 = assign5290_e5124_d_n9;
        locals.var_shw_dn10 = assign5290_e5124_d_n10;
        locals.var_shw_rv = 0.0;

        let (assign5300_e5146,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard93 == 0.0)) {
        let assign5300_e5138: f64 = (1.0 + p.p46);
        let assign5300_e5142: f64 = (2.0 * p.p46);
        let assign5300_e5143: f64 = (1.0 + assign5300_e5142);
        let assign5300_e5144: f64 = (assign5300_e5138 / assign5300_e5143);
        (assign5300_e5144,)
    } else {
        (locals.var_efi,)
    }
};
        locals.var_efi = assign5300_e5146;
        locals.var_efi_rv = 0.0;

        let (assign5310_e5174, assign5310_e5174_d_n0, assign5310_e5174_d_n1, assign5310_e5174_d_n3, assign5310_e5174_d_n4, assign5310_e5174_d_n5, assign5310_e5174_d_n6, assign5310_e5174_d_n7, assign5310_e5174_d_n8, assign5310_e5174_d_n9, assign5310_e5174_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard93 == 0.0)) {
        let assign5310_e5161: f64 = (0.5 * locals.var_wd);
        let assign5310_e5163: f64 = (assign5310_e5161 * locals.var_dedx0);
        let assign5310_e5168: f64 = (p.p61 * locals.var_shw);
        let assign5310_e5169: f64 = (locals.var_in_ / assign5310_e5168);
        let assign5310_e5170: f64 = (locals.var_efi - assign5310_e5169);
        let assign5310_e5171: f64 = (assign5310_e5163 * assign5310_e5170);
        let assign5310_e5172: f64 = (locals.var_eav - assign5310_e5171);
        (assign5310_e5172, (locals.var_eav_dn0 - ((((0.5 * locals.var_wd_dn0) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn0 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn0))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn1 - ((((0.5 * locals.var_wd_dn1) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn1 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn1))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn3 - ((((0.5 * locals.var_wd_dn3) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn3 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn3))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn4 - ((((0.5 * locals.var_wd_dn4) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn4 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn4))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn5 - ((((0.5 * locals.var_wd_dn5) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn5 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn5))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn6 - ((((0.5 * locals.var_wd_dn6) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn6 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn6))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn7 - ((((0.5 * locals.var_wd_dn7) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn7 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn7))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn8 - ((((0.5 * locals.var_wd_dn8) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn8 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn8))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn9 - ((((0.5 * locals.var_wd_dn9) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn9 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn9))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn10 - ((((0.5 * locals.var_wd_dn10) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn10 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn10))) / (assign5310_e5168 * assign5310_e5168)))))),)
    } else {
        (locals.var_ew, locals.var_ew_dn0, locals.var_ew_dn1, locals.var_ew_dn3, locals.var_ew_dn4, locals.var_ew_dn5, locals.var_ew_dn6, locals.var_ew_dn7, locals.var_ew_dn8, locals.var_ew_dn9, locals.var_ew_dn10,)
    }
};
        locals.var_ew = assign5310_e5174;
        locals.var_ew_dn0 = assign5310_e5174_d_n0;
        locals.var_ew_dn1 = assign5310_e5174_d_n1;
        locals.var_ew_dn3 = assign5310_e5174_d_n3;
        locals.var_ew_dn4 = assign5310_e5174_d_n4;
        locals.var_ew_dn5 = assign5310_e5174_d_n5;
        locals.var_ew_dn6 = assign5310_e5174_d_n6;
        locals.var_ew_dn7 = assign5310_e5174_d_n7;
        locals.var_ew_dn8 = assign5310_e5174_d_n8;
        locals.var_ew_dn9 = assign5310_e5174_d_n9;
        locals.var_ew_dn10 = assign5310_e5174_d_n10;
        locals.var_ew_rv = 0.0;

        let (assign5320_e5204, assign5320_e5204_d_n0, assign5320_e5204_d_n1, assign5320_e5204_d_n3, assign5320_e5204_d_n4, assign5320_e5204_d_n5, assign5320_e5204_d_n6, assign5320_e5204_d_n7, assign5320_e5204_d_n8, assign5320_e5204_d_n9, assign5320_e5204_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard93 == 0.0)) {
        let assign5320_e5188: f64 = (locals.var_ew - locals.var_e0);
        let assign5320_e5191: f64 = (locals.var_ew - locals.var_e0);
        let assign5320_e5192: f64 = (assign5320_e5188 * assign5320_e5191);
        let assign5320_e5195: f64 = (0.1 * locals.var_eav);
        let assign5320_e5197: f64 = (assign5320_e5195 * locals.var_eav);
        let assign5320_e5199: f64 = (assign5320_e5197 * locals.var_icap);
        let assign5320_e5201: f64 = (assign5320_e5199 / p.p61);
        let assign5320_e5202: f64 = (assign5320_e5192 + assign5320_e5201);
        (assign5320_e5202, ((((locals.var_ew_dn0 - locals.var_e0_dn0) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn0 - locals.var_e0_dn0))) + ((((((0.1 * locals.var_eav_dn0) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn0)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn0)) / p.p61)), ((((locals.var_ew_dn1 - locals.var_e0_dn1) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn1 - locals.var_e0_dn1))) + ((((((0.1 * locals.var_eav_dn1) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn1)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn1)) / p.p61)), ((((locals.var_ew_dn3 - locals.var_e0_dn3) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn3 - locals.var_e0_dn3))) + ((((((0.1 * locals.var_eav_dn3) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn3)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn3)) / p.p61)), ((((locals.var_ew_dn4 - locals.var_e0_dn4) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn4 - locals.var_e0_dn4))) + ((((((0.1 * locals.var_eav_dn4) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn4)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn4)) / p.p61)), ((((locals.var_ew_dn5 - locals.var_e0_dn5) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn5 - locals.var_e0_dn5))) + ((((((0.1 * locals.var_eav_dn5) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn5)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn5)) / p.p61)), ((((locals.var_ew_dn6 - locals.var_e0_dn6) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn6 - locals.var_e0_dn6))) + ((((((0.1 * locals.var_eav_dn6) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn6)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn6)) / p.p61)), ((((locals.var_ew_dn7 - locals.var_e0_dn7) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn7 - locals.var_e0_dn7))) + ((((((0.1 * locals.var_eav_dn7) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn7)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn7)) / p.p61)), ((((locals.var_ew_dn8 - locals.var_e0_dn8) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn8 - locals.var_e0_dn8))) + ((((((0.1 * locals.var_eav_dn8) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn8)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn8)) / p.p61)), ((((locals.var_ew_dn9 - locals.var_e0_dn9) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn9 - locals.var_e0_dn9))) + ((((((0.1 * locals.var_eav_dn9) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn9)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn9)) / p.p61)), ((((locals.var_ew_dn10 - locals.var_e0_dn10) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn10 - locals.var_e0_dn10))) + ((((((0.1 * locals.var_eav_dn10) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn10)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn10)) / p.p61)),)
    } else {
        (locals.var_sqr_arg, locals.var_sqr_arg_dn0, locals.var_sqr_arg_dn1, locals.var_sqr_arg_dn3, locals.var_sqr_arg_dn4, locals.var_sqr_arg_dn5, locals.var_sqr_arg_dn6, locals.var_sqr_arg_dn7, locals.var_sqr_arg_dn8, locals.var_sqr_arg_dn9, locals.var_sqr_arg_dn10,)
    }
};
        locals.var_sqr_arg = assign5320_e5204;
        locals.var_sqr_arg_dn0 = assign5320_e5204_d_n0;
        locals.var_sqr_arg_dn1 = assign5320_e5204_d_n1;
        locals.var_sqr_arg_dn3 = assign5320_e5204_d_n3;
        locals.var_sqr_arg_dn4 = assign5320_e5204_d_n4;
        locals.var_sqr_arg_dn5 = assign5320_e5204_d_n5;
        locals.var_sqr_arg_dn6 = assign5320_e5204_d_n6;
        locals.var_sqr_arg_dn7 = assign5320_e5204_d_n7;
        locals.var_sqr_arg_dn8 = assign5320_e5204_d_n8;
        locals.var_sqr_arg_dn9 = assign5320_e5204_d_n9;
        locals.var_sqr_arg_dn10 = assign5320_e5204_d_n10;
        locals.var_sqr_arg_rv = 0.0;

        let (assign5330_e5225, assign5330_e5225_d_n0, assign5330_e5225_d_n1, assign5330_e5225_d_n3, assign5330_e5225_d_n4, assign5330_e5225_d_n5, assign5330_e5225_d_n6, assign5330_e5225_d_n7, assign5330_e5225_d_n8, assign5330_e5225_d_n9, assign5330_e5225_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard93 == 0.0)) {
        let assign5330_e5219: f64 = (locals.var_ew + locals.var_e0);
        let assign5330_e5221: f64 = (locals.var_sqr_arg).sqrt();
        let assign5330_e5222: f64 = (assign5330_e5219 + assign5330_e5221);
        let assign5330_e5223: f64 = (0.5 * assign5330_e5222);
        (assign5330_e5223, (0.5 * ((locals.var_ew_dn0 + locals.var_e0_dn0) + (locals.var_sqr_arg_dn0 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn1 + locals.var_e0_dn1) + (locals.var_sqr_arg_dn1 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn3 + locals.var_e0_dn3) + (locals.var_sqr_arg_dn3 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn4 + locals.var_e0_dn4) + (locals.var_sqr_arg_dn4 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn5 + locals.var_e0_dn5) + (locals.var_sqr_arg_dn5 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn6 + locals.var_e0_dn6) + (locals.var_sqr_arg_dn6 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn7 + locals.var_e0_dn7) + (locals.var_sqr_arg_dn7 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn8 + locals.var_e0_dn8) + (locals.var_sqr_arg_dn8 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn9 + locals.var_e0_dn9) + (locals.var_sqr_arg_dn9 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn10 + locals.var_e0_dn10) + (locals.var_sqr_arg_dn10 / (2.0 * assign5330_e5221)))),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn1, locals.var_em_dn3, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10,)
    }
};
        locals.var_em = assign5330_e5225;
        locals.var_em_dn0 = assign5330_e5225_d_n0;
        locals.var_em_dn1 = assign5330_e5225_d_n1;
        locals.var_em_dn3 = assign5330_e5225_d_n3;
        locals.var_em_dn4 = assign5330_e5225_d_n4;
        locals.var_em_dn5 = assign5330_e5225_d_n5;
        locals.var_em_dn6 = assign5330_e5225_d_n6;
        locals.var_em_dn7 = assign5330_e5225_d_n7;
        locals.var_em_dn8 = assign5330_e5225_d_n8;
        locals.var_em_dn9 = assign5330_e5225_d_n9;
        locals.var_em_dn10 = assign5330_e5225_d_n10;
        locals.var_em_rv = 0.0;

        let (assign5340_e5240, assign5340_e5240_d_n0, assign5340_e5240_d_n1, assign5340_e5240_d_n3, assign5340_e5240_d_n4, assign5340_e5240_d_n5, assign5340_e5240_d_n6, assign5340_e5240_d_n7, assign5340_e5240_d_n8, assign5340_e5240_d_n9, assign5340_e5240_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5340_e5236: f64 = (locals.var_em - locals.var_eav);
        let assign5340_e5238: f64 = (assign5340_e5236 / locals.var_em);
        (assign5340_e5238, ((((locals.var_em_dn0 - locals.var_eav_dn0) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn0)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn1 - locals.var_eav_dn1) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn1)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn3 - locals.var_eav_dn3) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn3)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn4 - locals.var_eav_dn4) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn4)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn5 - locals.var_eav_dn5) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn5)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn6 - locals.var_eav_dn6) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn6)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn7 - locals.var_eav_dn7) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn7)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn8 - locals.var_eav_dn8) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn8)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn9 - locals.var_eav_dn9) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn9)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn10 - locals.var_eav_dn10) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn10)) / (locals.var_em * locals.var_em)),)
    } else {
        (locals.var_emeav_em, locals.var_emeav_em_dn0, locals.var_emeav_em_dn1, locals.var_emeav_em_dn3, locals.var_emeav_em_dn4, locals.var_emeav_em_dn5, locals.var_emeav_em_dn6, locals.var_emeav_em_dn7, locals.var_emeav_em_dn8, locals.var_emeav_em_dn9, locals.var_emeav_em_dn10,)
    }
};
        locals.var_emeav_em = assign5340_e5240;
        locals.var_emeav_em_dn0 = assign5340_e5240_d_n0;
        locals.var_emeav_em_dn1 = assign5340_e5240_d_n1;
        locals.var_emeav_em_dn3 = assign5340_e5240_d_n3;
        locals.var_emeav_em_dn4 = assign5340_e5240_d_n4;
        locals.var_emeav_em_dn5 = assign5340_e5240_d_n5;
        locals.var_emeav_em_dn6 = assign5340_e5240_d_n6;
        locals.var_emeav_em_dn7 = assign5340_e5240_d_n7;
        locals.var_emeav_em_dn8 = assign5340_e5240_d_n8;
        locals.var_emeav_em_dn9 = assign5340_e5240_d_n9;
        locals.var_emeav_em_dn10 = assign5340_e5240_d_n10;
        locals.var_emeav_em_rv = 0.0;

        let assign5350_e5242: f64 = (locals.var_emeav_em).abs();
        let assign5350_e5244: f64 = if assign5350_e5242 > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign5350_e5244;
        locals.var_guard94_rv = 0.0;

        let (assign5360_e5261, assign5360_e5261_d_n0, assign5360_e5261_d_n1, assign5360_e5261_d_n3, assign5360_e5261_d_n4, assign5360_e5261_d_n5, assign5360_e5261_d_n6, assign5360_e5261_d_n7, assign5360_e5261_d_n8, assign5360_e5261_d_n9, assign5360_e5261_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard94 != 0.0)) {
        let assign5360_e5257: f64 = (0.5 * locals.var_wd);
        let assign5360_e5259: f64 = (assign5360_e5257 / locals.var_emeav_em);
        (assign5360_e5259, ((((0.5 * locals.var_wd_dn0) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn0)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn1) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn1)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn3) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn3)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn4) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn4)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn5) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn5)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn6) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn6)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn7) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn7)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn8) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn8)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn9) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn9)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn10) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn10)) / (locals.var_emeav_em * locals.var_emeav_em)),)
    } else {
        (locals.var_lambda, locals.var_lambda_dn0, locals.var_lambda_dn1, locals.var_lambda_dn3, locals.var_lambda_dn4, locals.var_lambda_dn5, locals.var_lambda_dn6, locals.var_lambda_dn7, locals.var_lambda_dn8, locals.var_lambda_dn9, locals.var_lambda_dn10,)
    }
};
        locals.var_lambda = assign5360_e5261;
        locals.var_lambda_dn0 = assign5360_e5261_d_n0;
        locals.var_lambda_dn1 = assign5360_e5261_d_n1;
        locals.var_lambda_dn3 = assign5360_e5261_d_n3;
        locals.var_lambda_dn4 = assign5360_e5261_d_n4;
        locals.var_lambda_dn5 = assign5360_e5261_d_n5;
        locals.var_lambda_dn6 = assign5360_e5261_d_n6;
        locals.var_lambda_dn7 = assign5360_e5261_d_n7;
        locals.var_lambda_dn8 = assign5360_e5261_d_n8;
        locals.var_lambda_dn9 = assign5360_e5261_d_n9;
        locals.var_lambda_dn10 = assign5360_e5261_d_n10;
        locals.var_lambda_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5370_e5298, assign5370_e5298_d_n0, assign5370_e5298_d_n1, assign5370_e5298_d_n3, assign5370_e5298_d_n4, assign5370_e5298_d_n5, assign5370_e5298_d_n6, assign5370_e5298_d_n7, assign5370_e5298_d_n8, assign5370_e5298_d_n9, assign5370_e5298_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard94 != 0.0)) {
        let assign5370_e5274: f64 = (locals.var_an / locals.var_bnt);
        let assign5370_e5276: f64 = (assign5370_e5274 * locals.var_em);
        let assign5370_e5278: f64 = (assign5370_e5276 * locals.var_lambda);
        let assign5370_e5280: f64 = (-locals.var_bnt);
        let assign5370_e5282: f64 = (assign5370_e5280 / locals.var_em);
        let assign5370_e5283: f64 = (assign5370_e5282).exp();
        let assign5370_e5285: f64 = (-locals.var_bnt);
        let assign5370_e5287: f64 = (assign5370_e5285 / locals.var_em);
        let assign5370_e5291: f64 = (locals.var_weff / locals.var_lambda);
        let assign5370_e5292: f64 = (1.0 + assign5370_e5291);
        let assign5370_e5293: f64 = (assign5370_e5287 * assign5370_e5292);
        let assign5370_e5294: f64 = (assign5370_e5293).exp();
        let assign5370_e5295: f64 = (assign5370_e5283 - assign5370_e5294);
        let assign5370_e5296: f64 = (assign5370_e5278 * assign5370_e5295);
        (assign5370_e5296, (((((assign5370_e5274 * locals.var_em_dn0) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn0)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn0) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn0) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn0 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn0)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn1) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn1)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn1) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn1) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn1 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn1)) / (locals.var_lambda * locals.var_lambda)))))))), (((((((-((locals.var_an * locals.var_bnt_dn3) / (locals.var_bnt * locals.var_bnt))) * locals.var_em) + (assign5370_e5274 * locals.var_em_dn3)) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn3)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * ((((-locals.var_bnt_dn3) * locals.var_em) - (assign5370_e5280 * locals.var_em_dn3)) / (locals.var_em * locals.var_em))) - (assign5370_e5294 * ((((((-locals.var_bnt_dn3) * locals.var_em) - (assign5370_e5285 * locals.var_em_dn3)) / (locals.var_em * locals.var_em)) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn3 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn3)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn4) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn4)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn4) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn4) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn4 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn4)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn5) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn5)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn5) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn5) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn5 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn5)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn6) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn6)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn6) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn6) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn6 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn6)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn7) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn7)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn7) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn7) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn7 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn7)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn8) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn8)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn8) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn8) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn8 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn8)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn9) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn9)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn9) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn9) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn9 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn9)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn10) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn10)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn10) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn10) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn10 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn10)) / (locals.var_lambda * locals.var_lambda)))))))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10,)
    }
};
        locals.var_gem = assign5370_e5298;
        locals.var_gem_dn0 = assign5370_e5298_d_n0;
        locals.var_gem_dn1 = assign5370_e5298_d_n1;
        locals.var_gem_dn3 = assign5370_e5298_d_n3;
        locals.var_gem_dn4 = assign5370_e5298_d_n4;
        locals.var_gem_dn5 = assign5370_e5298_d_n5;
        locals.var_gem_dn6 = assign5370_e5298_d_n6;
        locals.var_gem_dn7 = assign5370_e5298_d_n7;
        locals.var_gem_dn8 = assign5370_e5298_d_n8;
        locals.var_gem_dn9 = assign5370_e5298_d_n9;
        locals.var_gem_dn10 = assign5370_e5298_d_n10;
        locals.var_gem_rv = 0.0;

        let (assign5380_e5320, assign5380_e5320_d_n0, assign5380_e5320_d_n1, assign5380_e5320_d_n3, assign5380_e5320_d_n4, assign5380_e5320_d_n5, assign5380_e5320_d_n6, assign5380_e5320_d_n7, assign5380_e5320_d_n8, assign5380_e5320_d_n9, assign5380_e5320_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard94 == 0.0)) {
        let assign5380_e5312: f64 = (locals.var_an * locals.var_weff);
        let assign5380_e5314: f64 = (-locals.var_bnt);
        let assign5380_e5316: f64 = (assign5380_e5314 / locals.var_em);
        let assign5380_e5317: f64 = (assign5380_e5316).exp();
        let assign5380_e5318: f64 = (assign5380_e5312 * assign5380_e5317);
        (assign5380_e5318, (((locals.var_an * locals.var_weff_dn0) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn0) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn1) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn1) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn3) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * ((((-locals.var_bnt_dn3) * locals.var_em) - (assign5380_e5314 * locals.var_em_dn3)) / (locals.var_em * locals.var_em))))), (((locals.var_an * locals.var_weff_dn4) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn4) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn5) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn5) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn6) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn6) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn7) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn7) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn8) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn8) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn9) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn9) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn10) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn10) / (locals.var_em * locals.var_em)))))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10,)
    }
};
        locals.var_gem = assign5380_e5320;
        locals.var_gem_dn0 = assign5380_e5320_d_n0;
        locals.var_gem_dn1 = assign5380_e5320_d_n1;
        locals.var_gem_dn3 = assign5380_e5320_d_n3;
        locals.var_gem_dn4 = assign5380_e5320_d_n4;
        locals.var_gem_dn5 = assign5380_e5320_d_n5;
        locals.var_gem_dn6 = assign5380_e5320_d_n6;
        locals.var_gem_dn7 = assign5380_e5320_d_n7;
        locals.var_gem_dn8 = assign5380_e5320_d_n8;
        locals.var_gem_dn9 = assign5380_e5320_d_n9;
        locals.var_gem_dn10 = assign5380_e5320_d_n10;
        locals.var_gem_rv = 0.0;

        let assign5390_e5323: f64 = if p.p38 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign5390_e5323;
        locals.var_guard95_rv = 0.0;

        let assign5400_e5326: f64 = if locals.var_vb2c1 < p.p43 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign5400_e5326;
        locals.var_guard96_rv = 0.0;

        let (assign5410_e5354, assign5410_e5354_d_n0, assign5410_e5354_d_n1, assign5410_e5354_d_n3, assign5410_e5354_d_n4, assign5410_e5354_d_n5, assign5410_e5354_d_n6, assign5410_e5354_d_n7, assign5410_e5354_d_n8, assign5410_e5354_d_n9, assign5410_e5354_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) {
        let assign5410_e5340: f64 = (p.p43 - locals.var_vb2c1);
        let assign5410_e5342: f64 = (assign5410_e5340).powf(p.p40);
        let assign5410_e5347: f64 = (p.p47 + locals.var_in_);
        let assign5410_e5348: f64 = (locals.var_in_ / assign5410_e5347);
        let assign5410_e5349: f64 = (1.0 - assign5410_e5348);
        let assign5410_e5351: f64 = (assign5410_e5349).powf(p.p48);
        let assign5410_e5352: f64 = (assign5410_e5342 * assign5410_e5351);
        (assign5410_e5352, (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn0 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn0)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn0 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn0)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn1 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn1)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn1 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn1)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn3 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn3)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn3 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn3)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn4 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn4)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn4 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn4)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn5 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn5)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn5 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn5)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), ((if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((assign5410_e5340).powf(p.p40 - 1.0) * (-locals.var_vb2c1_dn6))) } } else { (assign5410_e5342 * (p.p40 * ((-locals.var_vb2c1_dn6) / assign5410_e5340))) } * assign5410_e5351) + (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn6 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn6)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn6 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn6)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) })), ((if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((assign5410_e5340).powf(p.p40 - 1.0) * (-locals.var_vb2c1_dn7))) } } else { (assign5410_e5342 * (p.p40 * ((-locals.var_vb2c1_dn7) / assign5410_e5340))) } * assign5410_e5351) + (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn7 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn7)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn7 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn7)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) })), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn8 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn8)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn8 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn8)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn9 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn9)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn9 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn9)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn10 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn10)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn10 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn10)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }),)
    } else {
        (locals.var_vdeptmp, locals.var_vdeptmp_dn0, locals.var_vdeptmp_dn1, locals.var_vdeptmp_dn3, locals.var_vdeptmp_dn4, locals.var_vdeptmp_dn5, locals.var_vdeptmp_dn6, locals.var_vdeptmp_dn7, locals.var_vdeptmp_dn8, locals.var_vdeptmp_dn9, locals.var_vdeptmp_dn10,)
    }
};
        locals.var_vdeptmp = assign5410_e5354;
        locals.var_vdeptmp_dn0 = assign5410_e5354_d_n0;
        locals.var_vdeptmp_dn1 = assign5410_e5354_d_n1;
        locals.var_vdeptmp_dn3 = assign5410_e5354_d_n3;
        locals.var_vdeptmp_dn4 = assign5410_e5354_d_n4;
        locals.var_vdeptmp_dn5 = assign5410_e5354_d_n5;
        locals.var_vdeptmp_dn6 = assign5410_e5354_d_n6;
        locals.var_vdeptmp_dn7 = assign5410_e5354_d_n7;
        locals.var_vdeptmp_dn8 = assign5410_e5354_d_n8;
        locals.var_vdeptmp_dn9 = assign5410_e5354_d_n9;
        locals.var_vdeptmp_dn10 = assign5410_e5354_d_n10;
        locals.var_vdeptmp_rv = 0.0;

        let assign5420_e5357: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign5420_e5357;
        locals.var_guard97_rv = 0.0;

        let (assign5430_e5373, assign5430_e5373_d_n0, assign5430_e5373_d_n1, assign5430_e5373_d_n3, assign5430_e5373_d_n4, assign5430_e5373_d_n5, assign5430_e5373_d_n6, assign5430_e5373_d_n7, assign5430_e5373_d_n8, assign5430_e5373_d_n9, assign5430_e5373_d_n10,) = {
    if ((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) {
        (locals.var_vdeptmp, locals.var_vdeptmp_dn0, locals.var_vdeptmp_dn1, locals.var_vdeptmp_dn3, locals.var_vdeptmp_dn4, locals.var_vdeptmp_dn5, locals.var_vdeptmp_dn6, locals.var_vdeptmp_dn7, locals.var_vdeptmp_dn8, locals.var_vdeptmp_dn9, locals.var_vdeptmp_dn10,)
    } else {
        (locals.var_vdep, locals.var_vdep_dn0, locals.var_vdep_dn1, locals.var_vdep_dn3, locals.var_vdep_dn4, locals.var_vdep_dn5, locals.var_vdep_dn6, locals.var_vdep_dn7, locals.var_vdep_dn8, locals.var_vdep_dn9, locals.var_vdep_dn10,)
    }
};
        locals.var_vdep = assign5430_e5373;
        locals.var_vdep_dn0 = assign5430_e5373_d_n0;
        locals.var_vdep_dn1 = assign5430_e5373_d_n1;
        locals.var_vdep_dn3 = assign5430_e5373_d_n3;
        locals.var_vdep_dn4 = assign5430_e5373_d_n4;
        locals.var_vdep_dn5 = assign5430_e5373_d_n5;
        locals.var_vdep_dn6 = assign5430_e5373_d_n6;
        locals.var_vdep_dn7 = assign5430_e5373_d_n7;
        locals.var_vdep_dn8 = assign5430_e5373_d_n8;
        locals.var_vdep_dn9 = assign5430_e5373_d_n9;
        locals.var_vdep_dn10 = assign5430_e5373_d_n10;
        locals.var_vdep_rv = 0.0;

        let (assign5440_e5394, assign5440_e5394_d_n0, assign5440_e5394_d_n1, assign5440_e5394_d_n3, assign5440_e5394_d_n4, assign5440_e5394_d_n5, assign5440_e5394_d_n6, assign5440_e5394_d_n7, assign5440_e5394_d_n8, assign5440_e5394_d_n9, assign5440_e5394_d_n10,) = {
    if ((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) {
        let assign5440_e5390: f64 = (locals.var_in_ - p.p51);
        let assign5440_e5392: f64 = (assign5440_e5390 / p.p47);
        (assign5440_e5392, (locals.var_in__dn0 / p.p47), (locals.var_in__dn1 / p.p47), (locals.var_in__dn3 / p.p47), (locals.var_in__dn4 / p.p47), (locals.var_in__dn5 / p.p47), (locals.var_in__dn6 / p.p47), (locals.var_in__dn7 / p.p47), (locals.var_in__dn8 / p.p47), (locals.var_in__dn9 / p.p47), (locals.var_in__dn10 / p.p47),)
    } else {
        (locals.var_in_shift_ihcavl, locals.var_in_shift_ihcavl_dn0, locals.var_in_shift_ihcavl_dn1, locals.var_in_shift_ihcavl_dn3, locals.var_in_shift_ihcavl_dn4, locals.var_in_shift_ihcavl_dn5, locals.var_in_shift_ihcavl_dn6, locals.var_in_shift_ihcavl_dn7, locals.var_in_shift_ihcavl_dn8, locals.var_in_shift_ihcavl_dn9, locals.var_in_shift_ihcavl_dn10,)
    }
};
        locals.var_in_shift_ihcavl = assign5440_e5394;
        locals.var_in_shift_ihcavl_dn0 = assign5440_e5394_d_n0;
        locals.var_in_shift_ihcavl_dn1 = assign5440_e5394_d_n1;
        locals.var_in_shift_ihcavl_dn3 = assign5440_e5394_d_n3;
        locals.var_in_shift_ihcavl_dn4 = assign5440_e5394_d_n4;
        locals.var_in_shift_ihcavl_dn5 = assign5440_e5394_d_n5;
        locals.var_in_shift_ihcavl_dn6 = assign5440_e5394_d_n6;
        locals.var_in_shift_ihcavl_dn7 = assign5440_e5394_d_n7;
        locals.var_in_shift_ihcavl_dn8 = assign5440_e5394_d_n8;
        locals.var_in_shift_ihcavl_dn9 = assign5440_e5394_d_n9;
        locals.var_in_shift_ihcavl_dn10 = assign5440_e5394_d_n10;
        locals.var_in_shift_ihcavl_rv = 0.0;

        let (assign5450_e5415, assign5450_e5415_d_n0, assign5450_e5415_d_n1, assign5450_e5415_d_n3, assign5450_e5415_d_n4, assign5450_e5415_d_n5, assign5450_e5415_d_n6, assign5450_e5415_d_n7, assign5450_e5415_d_n8, assign5450_e5415_d_n9, assign5450_e5415_d_n10,) = {
    if ((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) {
        let assign5450_e5411: f64 = (locals.var_in_shift_ihcavl - 1.0);
        let assign5450_e5413: f64 = (assign5450_e5411 / p.p50);
        (assign5450_e5413, (locals.var_in_shift_ihcavl_dn0 / p.p50), (locals.var_in_shift_ihcavl_dn1 / p.p50), (locals.var_in_shift_ihcavl_dn3 / p.p50), (locals.var_in_shift_ihcavl_dn4 / p.p50), (locals.var_in_shift_ihcavl_dn5 / p.p50), (locals.var_in_shift_ihcavl_dn6 / p.p50), (locals.var_in_shift_ihcavl_dn7 / p.p50), (locals.var_in_shift_ihcavl_dn8 / p.p50), (locals.var_in_shift_ihcavl_dn9 / p.p50), (locals.var_in_shift_ihcavl_dn10 / p.p50),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9, locals.var_dxa_dn10,)
    }
};
        locals.var_dxa = assign5450_e5415;
        locals.var_dxa_dn0 = assign5450_e5415_d_n0;
        locals.var_dxa_dn1 = assign5450_e5415_d_n1;
        locals.var_dxa_dn3 = assign5450_e5415_d_n3;
        locals.var_dxa_dn4 = assign5450_e5415_d_n4;
        locals.var_dxa_dn5 = assign5450_e5415_d_n5;
        locals.var_dxa_dn6 = assign5450_e5415_d_n6;
        locals.var_dxa_dn7 = assign5450_e5415_d_n7;
        locals.var_dxa_dn8 = assign5450_e5415_d_n8;
        locals.var_dxa_dn9 = assign5450_e5415_d_n9;
        locals.var_dxa_dn10 = assign5450_e5415_d_n10;
        locals.var_dxa_rv = 0.0;

        let assign5460_e5418: f64 = if locals.var_in_shift_ihcavl < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign5460_e5418;
        locals.var_guard98_rv = 0.0;

        let (assign5470_e5445, assign5470_e5445_d_n0, assign5470_e5445_d_n1, assign5470_e5445_d_n3, assign5470_e5445_d_n4, assign5470_e5445_d_n5, assign5470_e5445_d_n6, assign5470_e5445_d_n7, assign5470_e5445_d_n8, assign5470_e5445_d_n9, assign5470_e5445_d_n10,) = {
    if (((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5470_e5439: f64 = (locals.var_dxa).exp();
        let assign5470_e5440: f64 = (1.0 + assign5470_e5439);
        let assign5470_e5441: f64 = (assign5470_e5440).ln();
        let assign5470_e5442: f64 = (p.p50 * assign5470_e5441);
        let assign5470_e5443: f64 = (1.0 + assign5470_e5442);
        (assign5470_e5443, (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn0) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn1) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn3) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn4) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn5) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn6) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn7) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn8) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn9) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn10) / assign5470_e5440)),)
    } else {
        (locals.var_in_shift_n, locals.var_in_shift_n_dn0, locals.var_in_shift_n_dn1, locals.var_in_shift_n_dn3, locals.var_in_shift_n_dn4, locals.var_in_shift_n_dn5, locals.var_in_shift_n_dn6, locals.var_in_shift_n_dn7, locals.var_in_shift_n_dn8, locals.var_in_shift_n_dn9, locals.var_in_shift_n_dn10,)
    }
};
        locals.var_in_shift_n = assign5470_e5445;
        locals.var_in_shift_n_dn0 = assign5470_e5445_d_n0;
        locals.var_in_shift_n_dn1 = assign5470_e5445_d_n1;
        locals.var_in_shift_n_dn3 = assign5470_e5445_d_n3;
        locals.var_in_shift_n_dn4 = assign5470_e5445_d_n4;
        locals.var_in_shift_n_dn5 = assign5470_e5445_d_n5;
        locals.var_in_shift_n_dn6 = assign5470_e5445_d_n6;
        locals.var_in_shift_n_dn7 = assign5470_e5445_d_n7;
        locals.var_in_shift_n_dn8 = assign5470_e5445_d_n8;
        locals.var_in_shift_n_dn9 = assign5470_e5445_d_n9;
        locals.var_in_shift_n_dn10 = assign5470_e5445_d_n10;
        locals.var_in_shift_n_rv = 0.0;

        let (assign5480_e5474, assign5480_e5474_d_n0, assign5480_e5474_d_n1, assign5480_e5474_d_n3, assign5480_e5474_d_n4, assign5480_e5474_d_n5, assign5480_e5474_d_n6, assign5480_e5474_d_n7, assign5480_e5474_d_n8, assign5480_e5474_d_n9, assign5480_e5474_d_n10,) = {
    if (((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard98 == 0.0)) {
        let assign5480_e5467: f64 = (-locals.var_dxa);
        let assign5480_e5468: f64 = (assign5480_e5467).exp();
        let assign5480_e5469: f64 = (1.0 + assign5480_e5468);
        let assign5480_e5470: f64 = (assign5480_e5469).ln();
        let assign5480_e5471: f64 = (p.p50 * assign5480_e5470);
        let assign5480_e5472: f64 = (locals.var_in_shift_ihcavl + assign5480_e5471);
        (assign5480_e5472, (locals.var_in_shift_ihcavl_dn0 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn0)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn1 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn1)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn3 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn3)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn4 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn4)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn5 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn5)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn6 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn6)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn7 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn7)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn8 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn8)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn9 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn9)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn10 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn10)) / assign5480_e5469))),)
    } else {
        (locals.var_in_shift_n, locals.var_in_shift_n_dn0, locals.var_in_shift_n_dn1, locals.var_in_shift_n_dn3, locals.var_in_shift_n_dn4, locals.var_in_shift_n_dn5, locals.var_in_shift_n_dn6, locals.var_in_shift_n_dn7, locals.var_in_shift_n_dn8, locals.var_in_shift_n_dn9, locals.var_in_shift_n_dn10,)
    }
};
        locals.var_in_shift_n = assign5480_e5474;
        locals.var_in_shift_n_dn0 = assign5480_e5474_d_n0;
        locals.var_in_shift_n_dn1 = assign5480_e5474_d_n1;
        locals.var_in_shift_n_dn3 = assign5480_e5474_d_n3;
        locals.var_in_shift_n_dn4 = assign5480_e5474_d_n4;
        locals.var_in_shift_n_dn5 = assign5480_e5474_d_n5;
        locals.var_in_shift_n_dn6 = assign5480_e5474_d_n6;
        locals.var_in_shift_n_dn7 = assign5480_e5474_d_n7;
        locals.var_in_shift_n_dn8 = assign5480_e5474_d_n8;
        locals.var_in_shift_n_dn9 = assign5480_e5474_d_n9;
        locals.var_in_shift_n_dn10 = assign5480_e5474_d_n10;
        locals.var_in_shift_n_rv = 0.0;

        let (assign5490_e5495, assign5490_e5495_d_n0, assign5490_e5495_d_n1, assign5490_e5495_d_n3, assign5490_e5495_d_n4, assign5490_e5495_d_n5, assign5490_e5495_d_n6, assign5490_e5495_d_n7, assign5490_e5495_d_n8, assign5490_e5495_d_n9, assign5490_e5495_d_n10,) = {
    if ((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) {
        let assign5490_e5492: f64 = (locals.var_in_shift_n).powf(p.p49);
        let assign5490_e5493: f64 = (locals.var_vdeptmp * assign5490_e5492);
        (assign5490_e5493, ((locals.var_vdeptmp_dn0 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn0)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn0 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn1 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn1)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn1 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn3 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn3)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn3 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn4 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn4)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn4 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn5 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn5)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn5 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn6 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn6)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn6 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn7 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn7)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn7 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn8 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn8)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn8 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn9 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn9)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn9 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn10 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn10)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn10 / locals.var_in_shift_n))) })),)
    } else {
        (locals.var_vdep, locals.var_vdep_dn0, locals.var_vdep_dn1, locals.var_vdep_dn3, locals.var_vdep_dn4, locals.var_vdep_dn5, locals.var_vdep_dn6, locals.var_vdep_dn7, locals.var_vdep_dn8, locals.var_vdep_dn9, locals.var_vdep_dn10,)
    }
};
        locals.var_vdep = assign5490_e5495;
        locals.var_vdep_dn0 = assign5490_e5495_d_n0;
        locals.var_vdep_dn1 = assign5490_e5495_d_n1;
        locals.var_vdep_dn3 = assign5490_e5495_d_n3;
        locals.var_vdep_dn4 = assign5490_e5495_d_n4;
        locals.var_vdep_dn5 = assign5490_e5495_d_n5;
        locals.var_vdep_dn6 = assign5490_e5495_d_n6;
        locals.var_vdep_dn7 = assign5490_e5495_d_n7;
        locals.var_vdep_dn8 = assign5490_e5495_d_n8;
        locals.var_vdep_dn9 = assign5490_e5495_d_n9;
        locals.var_vdep_dn10 = assign5490_e5495_d_n10;
        locals.var_vdep_rv = 0.0;

        let assign5500_e5497: f64 = (-locals.var_bavl_t);
        let assign5500_e5499: f64 = (assign5500_e5497 * locals.var_vdep);
        let assign5500_e5501: f64 = if assign5500_e5499 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign5500_e5501;
        locals.var_guard99_rv = 0.0;

        let (assign5510_e5521, assign5510_e5521_d_n0, assign5510_e5521_d_n1, assign5510_e5521_d_n3, assign5510_e5521_d_n4, assign5510_e5521_d_n5, assign5510_e5521_d_n6, assign5510_e5521_d_n7, assign5510_e5521_d_n8, assign5510_e5521_d_n9, assign5510_e5521_d_n10,) = {
    if ((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard99 != 0.0)) {
        let assign5510_e5516: f64 = (-locals.var_bavl_t);
        let assign5510_e5518: f64 = (assign5510_e5516 * locals.var_vdep);
        let assign5510_e5519: f64 = (assign5510_e5518).exp();
        (assign5510_e5519, (assign5510_e5519 * (((-locals.var_bavl_t_dn0) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn0))), (assign5510_e5519 * (((-locals.var_bavl_t_dn1) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn1))), (assign5510_e5519 * (((-locals.var_bavl_t_dn3) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn3))), (assign5510_e5519 * (((-locals.var_bavl_t_dn4) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn4))), (assign5510_e5519 * (((-locals.var_bavl_t_dn5) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn5))), (assign5510_e5519 * (((-locals.var_bavl_t_dn6) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn6))), (assign5510_e5519 * (((-locals.var_bavl_t_dn7) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn7))), (assign5510_e5519 * (((-locals.var_bavl_t_dn8) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn8))), (assign5510_e5519 * (((-locals.var_bavl_t_dn9) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn9))), (assign5510_e5519 * (((-locals.var_bavl_t_dn10) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn10))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10,)
    }
};
        locals.var_expmm1 = assign5510_e5521;
        locals.var_expmm1_dn0 = assign5510_e5521_d_n0;
        locals.var_expmm1_dn1 = assign5510_e5521_d_n1;
        locals.var_expmm1_dn3 = assign5510_e5521_d_n3;
        locals.var_expmm1_dn4 = assign5510_e5521_d_n4;
        locals.var_expmm1_dn5 = assign5510_e5521_d_n5;
        locals.var_expmm1_dn6 = assign5510_e5521_d_n6;
        locals.var_expmm1_dn7 = assign5510_e5521_d_n7;
        locals.var_expmm1_dn8 = assign5510_e5521_d_n8;
        locals.var_expmm1_dn9 = assign5510_e5521_d_n9;
        locals.var_expmm1_dn10 = assign5510_e5521_d_n10;
        locals.var_expmm1_rv = 0.0;

        let (assign5520_e5539,) = {
    if ((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard99 == 0.0)) {
        let assign5520_e5537: f64 = (p.p138).exp();
        (assign5520_e5537,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5520_e5539;
        locals.var_expl_rv = 0.0;

        let (assign5530_e5565, assign5530_e5565_d_n0, assign5530_e5565_d_n1, assign5530_e5565_d_n3, assign5530_e5565_d_n4, assign5530_e5565_d_n5, assign5530_e5565_d_n6, assign5530_e5565_d_n7, assign5530_e5565_d_n8, assign5530_e5565_d_n9, assign5530_e5565_d_n10,) = {
    if ((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard99 == 0.0)) {
        let assign5530_e5557: f64 = (-locals.var_bavl_t);
        let assign5530_e5559: f64 = (assign5530_e5557 * locals.var_vdep);
        let assign5530_e5561: f64 = (assign5530_e5559 - p.p138);
        let assign5530_e5562: f64 = (1.0 + assign5530_e5561);
        let assign5530_e5563: f64 = (locals.var_expl * assign5530_e5562);
        (assign5530_e5563, (locals.var_expl * (((-locals.var_bavl_t_dn0) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn0))), (locals.var_expl * (((-locals.var_bavl_t_dn1) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn1))), (locals.var_expl * (((-locals.var_bavl_t_dn3) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn3))), (locals.var_expl * (((-locals.var_bavl_t_dn4) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn4))), (locals.var_expl * (((-locals.var_bavl_t_dn5) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn5))), (locals.var_expl * (((-locals.var_bavl_t_dn6) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn6))), (locals.var_expl * (((-locals.var_bavl_t_dn7) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn7))), (locals.var_expl * (((-locals.var_bavl_t_dn8) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn8))), (locals.var_expl * (((-locals.var_bavl_t_dn9) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn9))), (locals.var_expl * (((-locals.var_bavl_t_dn10) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn10))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10,)
    }
};
        locals.var_expmm1 = assign5530_e5565;
        locals.var_expmm1_dn0 = assign5530_e5565_d_n0;
        locals.var_expmm1_dn1 = assign5530_e5565_d_n1;
        locals.var_expmm1_dn3 = assign5530_e5565_d_n3;
        locals.var_expmm1_dn4 = assign5530_e5565_d_n4;
        locals.var_expmm1_dn5 = assign5530_e5565_d_n5;
        locals.var_expmm1_dn6 = assign5530_e5565_d_n6;
        locals.var_expmm1_dn7 = assign5530_e5565_d_n7;
        locals.var_expmm1_dn8 = assign5530_e5565_d_n8;
        locals.var_expmm1_dn9 = assign5530_e5565_d_n9;
        locals.var_expmm1_dn10 = assign5530_e5565_d_n10;
        locals.var_expmm1_rv = 0.0;

        let (assign5540_e5587, assign5540_e5587_d_n0, assign5540_e5587_d_n1, assign5540_e5587_d_n3, assign5540_e5587_d_n4, assign5540_e5587_d_n5, assign5540_e5587_d_n6, assign5540_e5587_d_n7, assign5540_e5587_d_n8, assign5540_e5587_d_n9, assign5540_e5587_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) {
        let assign5540_e5579: f64 = (p.p39 / locals.var_bavl_t);
        let assign5540_e5582: f64 = (p.p43 - locals.var_vb2c1);
        let assign5540_e5583: f64 = (assign5540_e5579 * assign5540_e5582);
        let assign5540_e5585: f64 = (assign5540_e5583 * locals.var_expmm1);
        (assign5540_e5585, ((((-((p.p39 * locals.var_bavl_t_dn0) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn0)), ((((-((p.p39 * locals.var_bavl_t_dn1) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn1)), ((((-((p.p39 * locals.var_bavl_t_dn3) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn3)), ((((-((p.p39 * locals.var_bavl_t_dn4) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn4)), ((((-((p.p39 * locals.var_bavl_t_dn5) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn5)), (((((-((p.p39 * locals.var_bavl_t_dn6) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) + (assign5540_e5579 * (-locals.var_vb2c1_dn6))) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn6)), (((((-((p.p39 * locals.var_bavl_t_dn7) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) + (assign5540_e5579 * (-locals.var_vb2c1_dn7))) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn7)), ((((-((p.p39 * locals.var_bavl_t_dn8) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn8)), ((((-((p.p39 * locals.var_bavl_t_dn9) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn9)), ((((-((p.p39 * locals.var_bavl_t_dn10) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn10)),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10,)
    }
};
        locals.var_gem = assign5540_e5587;
        locals.var_gem_dn0 = assign5540_e5587_d_n0;
        locals.var_gem_dn1 = assign5540_e5587_d_n1;
        locals.var_gem_dn3 = assign5540_e5587_d_n3;
        locals.var_gem_dn4 = assign5540_e5587_d_n4;
        locals.var_gem_dn5 = assign5540_e5587_d_n5;
        locals.var_gem_dn6 = assign5540_e5587_d_n6;
        locals.var_gem_dn7 = assign5540_e5587_d_n7;
        locals.var_gem_dn8 = assign5540_e5587_d_n8;
        locals.var_gem_dn9 = assign5540_e5587_d_n9;
        locals.var_gem_dn10 = assign5540_e5587_d_n10;
        locals.var_gem_rv = 0.0;

        let assign5550_e5590: f64 = if locals.var_gem > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard100 = assign5550_e5590;
        locals.var_guard100_rv = 0.0;

        let assign5560_e5593: f64 = if p.p52 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard101 = assign5560_e5593;
        locals.var_guard101_rv = 0.0;

        let (assign5570_e5619, assign5570_e5619_d_n0, assign5570_e5619_d_n1, assign5570_e5619_d_n3, assign5570_e5619_d_n4, assign5570_e5619_d_n5, assign5570_e5619_d_n6, assign5570_e5619_d_n7, assign5570_e5619_d_n8, assign5570_e5619_d_n9, assign5570_e5619_d_n10,) = {
    if (((locals.var_guard85 != 0.0) && (locals.var_guard100 != 0.0)) && (locals.var_guard101 != 0.0)) {
        let assign5570_e5603: f64 = (locals.var_rbc_t + locals.var_rb2);
        let assign5570_e5604: f64 = (locals.var_in_ * assign5570_e5603);
        let assign5570_e5605: f64 = (locals.var_vt / assign5570_e5604);
        let assign5570_e5608: f64 = (locals.var_qbi / locals.var_is_t);
        let assign5570_e5610: f64 = (assign5570_e5608 * locals.var_ibi_t);
        let assign5570_e5611: f64 = (assign5570_e5605 + assign5570_e5610);
        let assign5570_e5615: f64 = (locals.var_rbc_t + locals.var_rb2);
        let assign5570_e5616: f64 = (locals.var_re_t / assign5570_e5615);
        let assign5570_e5617: f64 = (assign5570_e5611 + assign5570_e5616);
        (assign5570_e5617, (((-((locals.var_vt * ((locals.var_in__dn0 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn0))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn0 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn0)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn0) / (assign5570_e5615 * assign5570_e5615)))), (((-((locals.var_vt * ((locals.var_in__dn1 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn1))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn1 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn1)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn1) / (assign5570_e5615 * assign5570_e5615)))), (((((locals.var_vt_dn3 * assign5570_e5604) - (locals.var_vt * ((locals.var_in__dn3 * assign5570_e5603) + (locals.var_in_ * (locals.var_rbc_t_dn3 + locals.var_rb2_dn3))))) / (assign5570_e5604 * assign5570_e5604)) + (((((locals.var_qbi_dn3 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn3)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t) + (assign5570_e5608 * locals.var_ibi_t_dn3))) + (((locals.var_re_t_dn3 * assign5570_e5615) - (locals.var_re_t * (locals.var_rbc_t_dn3 + locals.var_rb2_dn3))) / (assign5570_e5615 * assign5570_e5615))), (((-((locals.var_vt * ((locals.var_in__dn4 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn4))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn4 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn4)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn4) / (assign5570_e5615 * assign5570_e5615)))), (((-((locals.var_vt * ((locals.var_in__dn5 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn5))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn5 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn5)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn5) / (assign5570_e5615 * assign5570_e5615)))), (((-((locals.var_vt * ((locals.var_in__dn6 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn6))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn6 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn6)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn6) / (assign5570_e5615 * assign5570_e5615)))), (((-((locals.var_vt * ((locals.var_in__dn7 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn7))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn7 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn7)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn7) / (assign5570_e5615 * assign5570_e5615)))), (((-((locals.var_vt * ((locals.var_in__dn8 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn8))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn8 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn8)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn8) / (assign5570_e5615 * assign5570_e5615)))), (((-((locals.var_vt * ((locals.var_in__dn9 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn9))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn9 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn9)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn9) / (assign5570_e5615 * assign5570_e5615)))), (((-((locals.var_vt * ((locals.var_in__dn10 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn10))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn10 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn10)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn10) / (assign5570_e5615 * assign5570_e5615)))),)
    } else {
        (locals.var_gmax, locals.var_gmax_dn0, locals.var_gmax_dn1, locals.var_gmax_dn3, locals.var_gmax_dn4, locals.var_gmax_dn5, locals.var_gmax_dn6, locals.var_gmax_dn7, locals.var_gmax_dn8, locals.var_gmax_dn9, locals.var_gmax_dn10,)
    }
};
        locals.var_gmax = assign5570_e5619;
        locals.var_gmax_dn0 = assign5570_e5619_d_n0;
        locals.var_gmax_dn1 = assign5570_e5619_d_n1;
        locals.var_gmax_dn3 = assign5570_e5619_d_n3;
        locals.var_gmax_dn4 = assign5570_e5619_d_n4;
        locals.var_gmax_dn5 = assign5570_e5619_d_n5;
        locals.var_gmax_dn6 = assign5570_e5619_d_n6;
        locals.var_gmax_dn7 = assign5570_e5619_d_n7;
        locals.var_gmax_dn8 = assign5570_e5619_d_n8;
        locals.var_gmax_dn9 = assign5570_e5619_d_n9;
        locals.var_gmax_dn10 = assign5570_e5619_d_n10;
        locals.var_gmax_rv = 0.0;

        let assign5580_e5622: f64 = if p.p38 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard102 = assign5580_e5622;
        locals.var_guard102_rv = 0.0;

        let (assign5590_e5636, assign5590_e5636_d_n0, assign5590_e5636_d_n1, assign5590_e5636_d_n3, assign5590_e5636_d_n4, assign5590_e5636_d_n5, assign5590_e5636_d_n6, assign5590_e5636_d_n7, assign5590_e5636_d_n8, assign5590_e5636_d_n9, assign5590_e5636_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard100 != 0.0)) && (locals.var_guard101 != 0.0)) && (locals.var_guard102 != 0.0)) {
        let assign5590_e5632: f64 = (locals.var_gem - locals.var_gmax);
        let assign5590_e5634: f64 = (assign5590_e5632 / 1e-6);
        (assign5590_e5634, ((locals.var_gem_dn0 - locals.var_gmax_dn0) / 1e-6), ((locals.var_gem_dn1 - locals.var_gmax_dn1) / 1e-6), ((locals.var_gem_dn3 - locals.var_gmax_dn3) / 1e-6), ((locals.var_gem_dn4 - locals.var_gmax_dn4) / 1e-6), ((locals.var_gem_dn5 - locals.var_gmax_dn5) / 1e-6), ((locals.var_gem_dn6 - locals.var_gmax_dn6) / 1e-6), ((locals.var_gem_dn7 - locals.var_gmax_dn7) / 1e-6), ((locals.var_gem_dn8 - locals.var_gmax_dn8) / 1e-6), ((locals.var_gem_dn9 - locals.var_gmax_dn9) / 1e-6), ((locals.var_gem_dn10 - locals.var_gmax_dn10) / 1e-6),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9, locals.var_dxa_dn10,)
    }
};
        locals.var_dxa = assign5590_e5636;
        locals.var_dxa_dn0 = assign5590_e5636_d_n0;
        locals.var_dxa_dn1 = assign5590_e5636_d_n1;
        locals.var_dxa_dn3 = assign5590_e5636_d_n3;
        locals.var_dxa_dn4 = assign5590_e5636_d_n4;
        locals.var_dxa_dn5 = assign5590_e5636_d_n5;
        locals.var_dxa_dn6 = assign5590_e5636_d_n6;
        locals.var_dxa_dn7 = assign5590_e5636_d_n7;
        locals.var_dxa_dn8 = assign5590_e5636_d_n8;
        locals.var_dxa_dn9 = assign5590_e5636_d_n9;
        locals.var_dxa_dn10 = assign5590_e5636_d_n10;
        locals.var_dxa_rv = 0.0;

        let assign5600_e5639: f64 = if locals.var_gem < locals.var_gmax { 1.0 } else { 0.0 };
        locals.var_guard103 = assign5600_e5639;
        locals.var_guard103_rv = 0.0;

        let (assign5610_e5659, assign5610_e5659_d_n0, assign5610_e5659_d_n1, assign5610_e5659_d_n3, assign5610_e5659_d_n4, assign5610_e5659_d_n5, assign5610_e5659_d_n6, assign5610_e5659_d_n7, assign5610_e5659_d_n8, assign5610_e5659_d_n9, assign5610_e5659_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard100 != 0.0)) && (locals.var_guard101 != 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) {
        let assign5610_e5653: f64 = (locals.var_dxa).exp();
        let assign5610_e5654: f64 = (1.0 + assign5610_e5653);
        let assign5610_e5655: f64 = (assign5610_e5654).ln();
        let assign5610_e5656: f64 = (1e-6 * assign5610_e5655);
        let assign5610_e5657: f64 = (locals.var_gem - assign5610_e5656);
        (assign5610_e5657, (locals.var_gem_dn0 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn0) / assign5610_e5654))), (locals.var_gem_dn1 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn1) / assign5610_e5654))), (locals.var_gem_dn3 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn3) / assign5610_e5654))), (locals.var_gem_dn4 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn4) / assign5610_e5654))), (locals.var_gem_dn5 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn5) / assign5610_e5654))), (locals.var_gem_dn6 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn6) / assign5610_e5654))), (locals.var_gem_dn7 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn7) / assign5610_e5654))), (locals.var_gem_dn8 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn8) / assign5610_e5654))), (locals.var_gem_dn9 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn9) / assign5610_e5654))), (locals.var_gem_dn10 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn10) / assign5610_e5654))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10,)
    }
};
        locals.var_gem = assign5610_e5659;
        locals.var_gem_dn0 = assign5610_e5659_d_n0;
        locals.var_gem_dn1 = assign5610_e5659_d_n1;
        locals.var_gem_dn3 = assign5610_e5659_d_n3;
        locals.var_gem_dn4 = assign5610_e5659_d_n4;
        locals.var_gem_dn5 = assign5610_e5659_d_n5;
        locals.var_gem_dn6 = assign5610_e5659_d_n6;
        locals.var_gem_dn7 = assign5610_e5659_d_n7;
        locals.var_gem_dn8 = assign5610_e5659_d_n8;
        locals.var_gem_dn9 = assign5610_e5659_d_n9;
        locals.var_gem_dn10 = assign5610_e5659_d_n10;
        locals.var_gem_rv = 0.0;

        let (assign5620_e5681, assign5620_e5681_d_n0, assign5620_e5681_d_n1, assign5620_e5681_d_n3, assign5620_e5681_d_n4, assign5620_e5681_d_n5, assign5620_e5681_d_n6, assign5620_e5681_d_n7, assign5620_e5681_d_n8, assign5620_e5681_d_n9, assign5620_e5681_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard100 != 0.0)) && (locals.var_guard101 != 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 == 0.0)) {
        let assign5620_e5674: f64 = (-locals.var_dxa);
        let assign5620_e5675: f64 = (assign5620_e5674).exp();
        let assign5620_e5676: f64 = (1.0 + assign5620_e5675);
        let assign5620_e5677: f64 = (assign5620_e5676).ln();
        let assign5620_e5678: f64 = (1e-6 * assign5620_e5677);
        let assign5620_e5679: f64 = (locals.var_gmax - assign5620_e5678);
        (assign5620_e5679, (locals.var_gmax_dn0 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn0)) / assign5620_e5676))), (locals.var_gmax_dn1 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn1)) / assign5620_e5676))), (locals.var_gmax_dn3 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn3)) / assign5620_e5676))), (locals.var_gmax_dn4 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn4)) / assign5620_e5676))), (locals.var_gmax_dn5 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn5)) / assign5620_e5676))), (locals.var_gmax_dn6 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn6)) / assign5620_e5676))), (locals.var_gmax_dn7 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn7)) / assign5620_e5676))), (locals.var_gmax_dn8 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn8)) / assign5620_e5676))), (locals.var_gmax_dn9 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn9)) / assign5620_e5676))), (locals.var_gmax_dn10 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn10)) / assign5620_e5676))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10,)
    }
};
        locals.var_gem = assign5620_e5681;
        locals.var_gem_dn0 = assign5620_e5681_d_n0;
        locals.var_gem_dn1 = assign5620_e5681_d_n1;
        locals.var_gem_dn3 = assign5620_e5681_d_n3;
        locals.var_gem_dn4 = assign5620_e5681_d_n4;
        locals.var_gem_dn5 = assign5620_e5681_d_n5;
        locals.var_gem_dn6 = assign5620_e5681_d_n6;
        locals.var_gem_dn7 = assign5620_e5681_d_n7;
        locals.var_gem_dn8 = assign5620_e5681_d_n8;
        locals.var_gem_dn9 = assign5620_e5681_d_n9;
        locals.var_gem_dn10 = assign5620_e5681_d_n10;
        locals.var_gem_rv = 0.0;

        let assign5730_e5844: f64 = (1.0 - p.p67);
        let assign5730_e5846: f64 = (assign5730_e5844 * locals.var_cje_t);
        let assign5730_e5848: f64 = (assign5730_e5846 * locals.var_vte);
        locals.var_qte = assign5730_e5848;
        locals.var_qte_dn0 = (((assign5730_e5844 * locals.var_cje_t_dn0) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn0));
        locals.var_qte_dn1 = (((assign5730_e5844 * locals.var_cje_t_dn1) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn1));
        locals.var_qte_dn3 = (((assign5730_e5844 * locals.var_cje_t_dn3) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn3));
        locals.var_qte_dn4 = (((assign5730_e5844 * locals.var_cje_t_dn4) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn4));
        locals.var_qte_dn5 = (((assign5730_e5844 * locals.var_cje_t_dn5) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn5));
        locals.var_qte_dn6 = (((assign5730_e5844 * locals.var_cje_t_dn6) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn6));
        locals.var_qte_dn7 = (((assign5730_e5844 * locals.var_cje_t_dn7) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn7));
        locals.var_qte_dn8 = (((assign5730_e5844 * locals.var_cje_t_dn8) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn8));
        locals.var_qte_dn9 = (((assign5730_e5844 * locals.var_cje_t_dn9) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn9));
        locals.var_qte_dn10 = (((assign5730_e5844 * locals.var_cje_t_dn10) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn10));
        locals.var_qte_rv = 0.0;

        let assign5740_e5851: f64 = (locals.var_vb1e1 - locals.var_vfe);
        let assign5740_e5853: f64 = (assign5740_e5851 / locals.var_a_vde);
        locals.var_dxa = assign5740_e5853;
        locals.var_dxa_dn0 = ((((-locals.var_vfe_dn0) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn0)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn1 = ((((-locals.var_vfe_dn1) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn1)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn3 = ((((-locals.var_vfe_dn3) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn3)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn4 = ((((locals.var_vb1e1_dn4 - locals.var_vfe_dn4) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn4)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn5 = ((((locals.var_vb1e1_dn5 - locals.var_vfe_dn5) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn5)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn6 = ((((-locals.var_vfe_dn6) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn6)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn7 = ((((-locals.var_vfe_dn7) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn7)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn8 = ((((-locals.var_vfe_dn8) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn8)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn9 = ((((-locals.var_vfe_dn9) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn9)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn10 = ((((-locals.var_vfe_dn10) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn10)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_rv = 0.0;

        let assign5750_e5856: f64 = if locals.var_vb1e1 < locals.var_vfe { 1.0 } else { 0.0 };
        locals.var_guard106 = assign5750_e5856;
        locals.var_guard106_rv = 0.0;

        let (assign5760_e5868, assign5760_e5868_d_n0, assign5760_e5868_d_n1, assign5760_e5868_d_n3, assign5760_e5868_d_n4, assign5760_e5868_d_n5, assign5760_e5868_d_n6, assign5760_e5868_d_n7, assign5760_e5868_d_n8, assign5760_e5868_d_n9, assign5760_e5868_d_n10,) = {
    if (locals.var_guard106 != 0.0) {
        let assign5760_e5862: f64 = (locals.var_dxa).exp();
        let assign5760_e5863: f64 = (1.0 + assign5760_e5862);
        let assign5760_e5864: f64 = (assign5760_e5863).ln();
        let assign5760_e5865: f64 = (locals.var_a_vde * assign5760_e5864);
        let assign5760_e5866: f64 = (locals.var_vb1e1 - assign5760_e5865);
        (assign5760_e5866, (-((locals.var_a_vde_dn0 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn0) / assign5760_e5863)))), (-((locals.var_a_vde_dn1 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn1) / assign5760_e5863)))), (-((locals.var_a_vde_dn3 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn3) / assign5760_e5863)))), (locals.var_vb1e1_dn4 - ((locals.var_a_vde_dn4 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn4) / assign5760_e5863)))), (locals.var_vb1e1_dn5 - ((locals.var_a_vde_dn5 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn5) / assign5760_e5863)))), (-((locals.var_a_vde_dn6 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn6) / assign5760_e5863)))), (-((locals.var_a_vde_dn7 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn7) / assign5760_e5863)))), (-((locals.var_a_vde_dn8 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn8) / assign5760_e5863)))), (-((locals.var_a_vde_dn9 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn9) / assign5760_e5863)))), (-((locals.var_a_vde_dn10 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn10) / assign5760_e5863)))),)
    } else {
        (locals.var_vje_s, locals.var_vje_s_dn0, locals.var_vje_s_dn1, locals.var_vje_s_dn3, locals.var_vje_s_dn4, locals.var_vje_s_dn5, locals.var_vje_s_dn6, locals.var_vje_s_dn7, locals.var_vje_s_dn8, locals.var_vje_s_dn9, locals.var_vje_s_dn10,)
    }
};
        locals.var_vje_s = assign5760_e5868;
        locals.var_vje_s_dn0 = assign5760_e5868_d_n0;
        locals.var_vje_s_dn1 = assign5760_e5868_d_n1;
        locals.var_vje_s_dn3 = assign5760_e5868_d_n3;
        locals.var_vje_s_dn4 = assign5760_e5868_d_n4;
        locals.var_vje_s_dn5 = assign5760_e5868_d_n5;
        locals.var_vje_s_dn6 = assign5760_e5868_d_n6;
        locals.var_vje_s_dn7 = assign5760_e5868_d_n7;
        locals.var_vje_s_dn8 = assign5760_e5868_d_n8;
        locals.var_vje_s_dn9 = assign5760_e5868_d_n9;
        locals.var_vje_s_dn10 = assign5760_e5868_d_n10;
        locals.var_vje_s_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5770_e5882, assign5770_e5882_d_n0, assign5770_e5882_d_n1, assign5770_e5882_d_n3, assign5770_e5882_d_n4, assign5770_e5882_d_n5, assign5770_e5882_d_n6, assign5770_e5882_d_n7, assign5770_e5882_d_n8, assign5770_e5882_d_n9, assign5770_e5882_d_n10,) = {
    if (locals.var_guard106 == 0.0) {
        let assign5770_e5875: f64 = (-locals.var_dxa);
        let assign5770_e5876: f64 = (assign5770_e5875).exp();
        let assign5770_e5877: f64 = (1.0 + assign5770_e5876);
        let assign5770_e5878: f64 = (assign5770_e5877).ln();
        let assign5770_e5879: f64 = (locals.var_a_vde * assign5770_e5878);
        let assign5770_e5880: f64 = (locals.var_vfe - assign5770_e5879);
        (assign5770_e5880, (locals.var_vfe_dn0 - ((locals.var_a_vde_dn0 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn0)) / assign5770_e5877)))), (locals.var_vfe_dn1 - ((locals.var_a_vde_dn1 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn1)) / assign5770_e5877)))), (locals.var_vfe_dn3 - ((locals.var_a_vde_dn3 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn3)) / assign5770_e5877)))), (locals.var_vfe_dn4 - ((locals.var_a_vde_dn4 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn4)) / assign5770_e5877)))), (locals.var_vfe_dn5 - ((locals.var_a_vde_dn5 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn5)) / assign5770_e5877)))), (locals.var_vfe_dn6 - ((locals.var_a_vde_dn6 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn6)) / assign5770_e5877)))), (locals.var_vfe_dn7 - ((locals.var_a_vde_dn7 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn7)) / assign5770_e5877)))), (locals.var_vfe_dn8 - ((locals.var_a_vde_dn8 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn8)) / assign5770_e5877)))), (locals.var_vfe_dn9 - ((locals.var_a_vde_dn9 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn9)) / assign5770_e5877)))), (locals.var_vfe_dn10 - ((locals.var_a_vde_dn10 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn10)) / assign5770_e5877)))),)
    } else {
        (locals.var_vje_s, locals.var_vje_s_dn0, locals.var_vje_s_dn1, locals.var_vje_s_dn3, locals.var_vje_s_dn4, locals.var_vje_s_dn5, locals.var_vje_s_dn6, locals.var_vje_s_dn7, locals.var_vje_s_dn8, locals.var_vje_s_dn9, locals.var_vje_s_dn10,)
    }
};
        locals.var_vje_s = assign5770_e5882;
        locals.var_vje_s_dn0 = assign5770_e5882_d_n0;
        locals.var_vje_s_dn1 = assign5770_e5882_d_n1;
        locals.var_vje_s_dn3 = assign5770_e5882_d_n3;
        locals.var_vje_s_dn4 = assign5770_e5882_d_n4;
        locals.var_vje_s_dn5 = assign5770_e5882_d_n5;
        locals.var_vje_s_dn6 = assign5770_e5882_d_n6;
        locals.var_vje_s_dn7 = assign5770_e5882_d_n7;
        locals.var_vje_s_dn8 = assign5770_e5882_d_n8;
        locals.var_vje_s_dn9 = assign5770_e5882_d_n9;
        locals.var_vje_s_dn10 = assign5770_e5882_d_n10;
        locals.var_vje_s_rv = 0.0;

        let assign5780_e5885: f64 = (p.p67 * locals.var_cje_t);
        let assign5780_e5889: f64 = (1.0 - p.p66);
        let assign5780_e5890: f64 = (locals.var_vde_t / assign5780_e5889);
        let assign5780_e5895: f64 = (locals.var_vje_s * locals.var_inv_vde_t);
        let assign5780_e5896: f64 = (1.0 - assign5780_e5895);
        let assign5780_e5899: f64 = (1.0 - p.p66);
        let assign5780_e5900: f64 = (assign5780_e5896).powf(assign5780_e5899);
        let assign5780_e5901: f64 = (1.0 - assign5780_e5900);
        let assign5780_e5902: f64 = (assign5780_e5890 * assign5780_e5901);
        let assign5780_e5906: f64 = (locals.var_vb1e1 - locals.var_vje_s);
        let assign5780_e5907: f64 = (3.0 * assign5780_e5906);
        let assign5780_e5908: f64 = (assign5780_e5902 + assign5780_e5907);
        let assign5780_e5909: f64 = (assign5780_e5885 * assign5780_e5908);
        locals.var_qte_s = assign5780_e5909;
        locals.var_qte_s_dn0 = (((p.p67 * locals.var_cje_t_dn0) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn0 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn0 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn0))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn0 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn0))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn0)))));
        locals.var_qte_s_dn1 = (((p.p67 * locals.var_cje_t_dn1) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn1 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn1 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn1))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn1 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn1))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn1)))));
        locals.var_qte_s_dn3 = (((p.p67 * locals.var_cje_t_dn3) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn3 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn3 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn3))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn3 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn3))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn3)))));
        locals.var_qte_s_dn4 = (((p.p67 * locals.var_cje_t_dn4) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn4 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn4 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn4))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn4 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn4))) / assign5780_e5896))) }))) + (3.0 * (locals.var_vb1e1_dn4 - locals.var_vje_s_dn4)))));
        locals.var_qte_s_dn5 = (((p.p67 * locals.var_cje_t_dn5) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn5 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn5 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn5))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn5 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn5))) / assign5780_e5896))) }))) + (3.0 * (locals.var_vb1e1_dn5 - locals.var_vje_s_dn5)))));
        locals.var_qte_s_dn6 = (((p.p67 * locals.var_cje_t_dn6) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn6 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn6 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn6))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn6 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn6))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn6)))));
        locals.var_qte_s_dn7 = (((p.p67 * locals.var_cje_t_dn7) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn7 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn7 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn7))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn7 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn7))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn7)))));
        locals.var_qte_s_dn8 = (((p.p67 * locals.var_cje_t_dn8) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn8 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn8 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn8))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn8 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn8))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn8)))));
        locals.var_qte_s_dn9 = (((p.p67 * locals.var_cje_t_dn9) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn9 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn9 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn9))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn9 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn9))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn9)))));
        locals.var_qte_s_dn10 = (((p.p67 * locals.var_cje_t_dn10) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn10 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn10 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn10))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn10 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn10))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn10)))));
        locals.var_qte_s_rv = 0.0;

        let assign5790_e5912: f64 = (p.p76 * locals.var_cjc_t);
        let assign5790_e5914: f64 = (assign5790_e5912 * locals.var_vtc);
        locals.var_qtc = assign5790_e5914;
        locals.var_qtc_dn0 = (((p.p76 * locals.var_cjc_t_dn0) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn0));
        locals.var_qtc_dn1 = (((p.p76 * locals.var_cjc_t_dn1) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn1));
        locals.var_qtc_dn3 = (((p.p76 * locals.var_cjc_t_dn3) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn3));
        locals.var_qtc_dn4 = (((p.p76 * locals.var_cjc_t_dn4) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn4));
        locals.var_qtc_dn5 = (((p.p76 * locals.var_cjc_t_dn5) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn5));
        locals.var_qtc_dn6 = (((p.p76 * locals.var_cjc_t_dn6) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn6));
        locals.var_qtc_dn7 = (((p.p76 * locals.var_cjc_t_dn7) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn7));
        locals.var_qtc_dn8 = (((p.p76 * locals.var_cjc_t_dn8) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn8));
        locals.var_qtc_dn9 = (((p.p76 * locals.var_cjc_t_dn9) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn9));
        locals.var_qtc_dn10 = (((p.p76 * locals.var_cjc_t_dn10) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn10));
        locals.var_qtc_rv = 0.0;

        let assign5800_e5917: f64 = (locals.var_taub_t * locals.var_ik_t);
        locals.var_qb0 = assign5800_e5917;
        locals.var_qb0_dn3 = ((locals.var_taub_t_dn3 * locals.var_ik_t) + (locals.var_taub_t * locals.var_ik_t_dn3));
        locals.var_qb0_rv = 0.0;

        let assign5810_e5920: f64 = (0.5 * locals.var_qb0);
        let assign5810_e5922: f64 = (assign5810_e5920 * locals.var_n0);
        let assign5810_e5924: f64 = (assign5810_e5922 * locals.var_q1q);
        locals.var_qbe_qs = assign5810_e5924;
        locals.var_qbe_qs_dn0 = (((assign5810_e5920 * locals.var_n0_dn0) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn0));
        locals.var_qbe_qs_dn1 = (((assign5810_e5920 * locals.var_n0_dn1) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn1));
        locals.var_qbe_qs_dn3 = (((((0.5 * locals.var_qb0_dn3) * locals.var_n0) + (assign5810_e5920 * locals.var_n0_dn3)) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn3));
        locals.var_qbe_qs_dn4 = (((assign5810_e5920 * locals.var_n0_dn4) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn4));
        locals.var_qbe_qs_dn5 = (((assign5810_e5920 * locals.var_n0_dn5) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn5));
        locals.var_qbe_qs_dn6 = (((assign5810_e5920 * locals.var_n0_dn6) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn6));
        locals.var_qbe_qs_dn7 = (((assign5810_e5920 * locals.var_n0_dn7) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn7));
        locals.var_qbe_qs_dn8 = (((assign5810_e5920 * locals.var_n0_dn8) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn8));
        locals.var_qbe_qs_dn9 = (((assign5810_e5920 * locals.var_n0_dn9) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn9));
        locals.var_qbe_qs_dn10 = (((assign5810_e5920 * locals.var_n0_dn10) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn10));
        locals.var_qbe_qs_rv = 0.0;

        let assign5820_e5927: f64 = (0.5 * locals.var_qb0);
        let assign5820_e5929: f64 = (assign5820_e5927 * locals.var_nb);
        let assign5820_e5931: f64 = (assign5820_e5929 * locals.var_q1q);
        locals.var_qbc_qs = assign5820_e5931;
        locals.var_qbc_qs_dn0 = (((assign5820_e5927 * locals.var_nb_dn0) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn0));
        locals.var_qbc_qs_dn1 = (((assign5820_e5927 * locals.var_nb_dn1) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn1));
        locals.var_qbc_qs_dn3 = (((((0.5 * locals.var_qb0_dn3) * locals.var_nb) + (assign5820_e5927 * locals.var_nb_dn3)) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn3));
        locals.var_qbc_qs_dn4 = (((assign5820_e5927 * locals.var_nb_dn4) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn4));
        locals.var_qbc_qs_dn5 = (((assign5820_e5927 * locals.var_nb_dn5) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn5));
        locals.var_qbc_qs_dn6 = (((assign5820_e5927 * locals.var_nb_dn6) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn6));
        locals.var_qbc_qs_dn7 = (((assign5820_e5927 * locals.var_nb_dn7) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn7));
        locals.var_qbc_qs_dn8 = (((assign5820_e5927 * locals.var_nb_dn8) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn8));
        locals.var_qbc_qs_dn9 = (((assign5820_e5927 * locals.var_nb_dn9) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn9));
        locals.var_qbc_qs_dn10 = (((assign5820_e5927 * locals.var_nb_dn10) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn10));
        locals.var_qbc_qs_rv = 0.0;

        let assign5830_e5934: f64 = (0.1 * locals.var_vdc_ctc_t);
        locals.var_a_vdcctc = assign5830_e5934;
        locals.var_a_vdcctc_dn0 = (0.1 * locals.var_vdc_ctc_t_dn0);
        locals.var_a_vdcctc_dn1 = (0.1 * locals.var_vdc_ctc_t_dn1);
        locals.var_a_vdcctc_dn3 = (0.1 * locals.var_vdc_ctc_t_dn3);
        locals.var_a_vdcctc_dn4 = (0.1 * locals.var_vdc_ctc_t_dn4);
        locals.var_a_vdcctc_dn5 = (0.1 * locals.var_vdc_ctc_t_dn5);
        locals.var_a_vdcctc_dn6 = (0.1 * locals.var_vdc_ctc_t_dn6);
        locals.var_a_vdcctc_dn7 = (0.1 * locals.var_vdc_ctc_t_dn7);
        locals.var_a_vdcctc_dn8 = (0.1 * locals.var_vdc_ctc_t_dn8);
        locals.var_a_vdcctc_dn9 = (0.1 * locals.var_vdc_ctc_t_dn9);
        locals.var_a_vdcctc_dn10 = (0.1 * locals.var_vdc_ctc_t_dn10);
        locals.var_a_vdcctc_rv = 0.0;

        let assign5840_e5937: f64 = (locals.var_vb1c4 - locals.var_vfc);
        let assign5840_e5939: f64 = (assign5840_e5937 / locals.var_a_vdcctc);
        locals.var_dxa = assign5840_e5939;
        locals.var_dxa_dn0 = ((((-locals.var_vfc_dn0) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn0)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn1 = ((((-locals.var_vfc_dn1) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn1)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn3 = ((((-locals.var_vfc_dn3) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn3)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn4 = ((((-locals.var_vfc_dn4) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn4)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn5 = ((((locals.var_vb1c4_dn5 - locals.var_vfc_dn5) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn5)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn6 = ((((locals.var_vb1c4_dn6 - locals.var_vfc_dn6) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn6)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn7 = ((((locals.var_vb1c4_dn7 - locals.var_vfc_dn7) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn7)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn8 = ((((locals.var_vb1c4_dn8 - locals.var_vfc_dn8) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn8)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn9 = ((((-locals.var_vfc_dn9) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn9)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn10 = ((((locals.var_vb1c4_dn10 - locals.var_vfc_dn10) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn10)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_rv = 0.0;

        let assign5850_e5942: f64 = if locals.var_vb1c4 < locals.var_vfc { 1.0 } else { 0.0 };
        locals.var_guard107 = assign5850_e5942;
        locals.var_guard107_rv = 0.0;

        let (assign5860_e5954, assign5860_e5954_d_n0, assign5860_e5954_d_n1, assign5860_e5954_d_n3, assign5860_e5954_d_n4, assign5860_e5954_d_n5, assign5860_e5954_d_n6, assign5860_e5954_d_n7, assign5860_e5954_d_n8, assign5860_e5954_d_n9, assign5860_e5954_d_n10,) = {
    if (locals.var_guard107 != 0.0) {
        let assign5860_e5948: f64 = (locals.var_dxa).exp();
        let assign5860_e5949: f64 = (1.0 + assign5860_e5948);
        let assign5860_e5950: f64 = (assign5860_e5949).ln();
        let assign5860_e5951: f64 = (locals.var_a_vdcctc * assign5860_e5950);
        let assign5860_e5952: f64 = (locals.var_vb1c4 - assign5860_e5951);
        (assign5860_e5952, (-((locals.var_a_vdcctc_dn0 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn0) / assign5860_e5949)))), (-((locals.var_a_vdcctc_dn1 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn1) / assign5860_e5949)))), (-((locals.var_a_vdcctc_dn3 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn3) / assign5860_e5949)))), (-((locals.var_a_vdcctc_dn4 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn4) / assign5860_e5949)))), (locals.var_vb1c4_dn5 - ((locals.var_a_vdcctc_dn5 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn5) / assign5860_e5949)))), (locals.var_vb1c4_dn6 - ((locals.var_a_vdcctc_dn6 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn6) / assign5860_e5949)))), (locals.var_vb1c4_dn7 - ((locals.var_a_vdcctc_dn7 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn7) / assign5860_e5949)))), (locals.var_vb1c4_dn8 - ((locals.var_a_vdcctc_dn8 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn8) / assign5860_e5949)))), (-((locals.var_a_vdcctc_dn9 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn9) / assign5860_e5949)))), (locals.var_vb1c4_dn10 - ((locals.var_a_vdcctc_dn10 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn10) / assign5860_e5949)))),)
    } else {
        (locals.var_vjcex, locals.var_vjcex_dn0, locals.var_vjcex_dn1, locals.var_vjcex_dn3, locals.var_vjcex_dn4, locals.var_vjcex_dn5, locals.var_vjcex_dn6, locals.var_vjcex_dn7, locals.var_vjcex_dn8, locals.var_vjcex_dn9, locals.var_vjcex_dn10,)
    }
};
        locals.var_vjcex = assign5860_e5954;
        locals.var_vjcex_dn0 = assign5860_e5954_d_n0;
        locals.var_vjcex_dn1 = assign5860_e5954_d_n1;
        locals.var_vjcex_dn3 = assign5860_e5954_d_n3;
        locals.var_vjcex_dn4 = assign5860_e5954_d_n4;
        locals.var_vjcex_dn5 = assign5860_e5954_d_n5;
        locals.var_vjcex_dn6 = assign5860_e5954_d_n6;
        locals.var_vjcex_dn7 = assign5860_e5954_d_n7;
        locals.var_vjcex_dn8 = assign5860_e5954_d_n8;
        locals.var_vjcex_dn9 = assign5860_e5954_d_n9;
        locals.var_vjcex_dn10 = assign5860_e5954_d_n10;
        locals.var_vjcex_rv = 0.0;

        let (assign5870_e5968, assign5870_e5968_d_n0, assign5870_e5968_d_n1, assign5870_e5968_d_n3, assign5870_e5968_d_n4, assign5870_e5968_d_n5, assign5870_e5968_d_n6, assign5870_e5968_d_n7, assign5870_e5968_d_n8, assign5870_e5968_d_n9, assign5870_e5968_d_n10,) = {
    if (locals.var_guard107 == 0.0) {
        let assign5870_e5961: f64 = (-locals.var_dxa);
        let assign5870_e5962: f64 = (assign5870_e5961).exp();
        let assign5870_e5963: f64 = (1.0 + assign5870_e5962);
        let assign5870_e5964: f64 = (assign5870_e5963).ln();
        let assign5870_e5965: f64 = (locals.var_a_vdcctc * assign5870_e5964);
        let assign5870_e5966: f64 = (locals.var_vfc - assign5870_e5965);
        (assign5870_e5966, (locals.var_vfc_dn0 - ((locals.var_a_vdcctc_dn0 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn0)) / assign5870_e5963)))), (locals.var_vfc_dn1 - ((locals.var_a_vdcctc_dn1 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn1)) / assign5870_e5963)))), (locals.var_vfc_dn3 - ((locals.var_a_vdcctc_dn3 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn3)) / assign5870_e5963)))), (locals.var_vfc_dn4 - ((locals.var_a_vdcctc_dn4 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn4)) / assign5870_e5963)))), (locals.var_vfc_dn5 - ((locals.var_a_vdcctc_dn5 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn5)) / assign5870_e5963)))), (locals.var_vfc_dn6 - ((locals.var_a_vdcctc_dn6 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn6)) / assign5870_e5963)))), (locals.var_vfc_dn7 - ((locals.var_a_vdcctc_dn7 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn7)) / assign5870_e5963)))), (locals.var_vfc_dn8 - ((locals.var_a_vdcctc_dn8 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn8)) / assign5870_e5963)))), (locals.var_vfc_dn9 - ((locals.var_a_vdcctc_dn9 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn9)) / assign5870_e5963)))), (locals.var_vfc_dn10 - ((locals.var_a_vdcctc_dn10 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn10)) / assign5870_e5963)))),)
    } else {
        (locals.var_vjcex, locals.var_vjcex_dn0, locals.var_vjcex_dn1, locals.var_vjcex_dn3, locals.var_vjcex_dn4, locals.var_vjcex_dn5, locals.var_vjcex_dn6, locals.var_vjcex_dn7, locals.var_vjcex_dn8, locals.var_vjcex_dn9, locals.var_vjcex_dn10,)
    }
};
        locals.var_vjcex = assign5870_e5968;
        locals.var_vjcex_dn0 = assign5870_e5968_d_n0;
        locals.var_vjcex_dn1 = assign5870_e5968_d_n1;
        locals.var_vjcex_dn3 = assign5870_e5968_d_n3;
        locals.var_vjcex_dn4 = assign5870_e5968_d_n4;
        locals.var_vjcex_dn5 = assign5870_e5968_d_n5;
        locals.var_vjcex_dn6 = assign5870_e5968_d_n6;
        locals.var_vjcex_dn7 = assign5870_e5968_d_n7;
        locals.var_vjcex_dn8 = assign5870_e5968_d_n8;
        locals.var_vjcex_dn9 = assign5870_e5968_d_n9;
        locals.var_vjcex_dn10 = assign5870_e5968_d_n10;
        locals.var_vjcex_rv = 0.0;

        let assign5880_e5972: f64 = (1.0 - p.p71);
        let assign5880_e5973: f64 = (locals.var_vdc_ctc_t / assign5880_e5972);
        let assign5880_e5978: f64 = (locals.var_vjcex / locals.var_vdc_ctc_t);
        let assign5880_e5979: f64 = (1.0 - assign5880_e5978);
        let assign5880_e5982: f64 = (1.0 - p.p71);
        let assign5880_e5983: f64 = (assign5880_e5979).powf(assign5880_e5982);
        let assign5880_e5984: f64 = (1.0 - assign5880_e5983);
        let assign5880_e5985: f64 = (assign5880_e5973 * assign5880_e5984);
        let assign5880_e5989: f64 = (locals.var_vb1c4 - locals.var_vjcex);
        let assign5880_e5990: f64 = (locals.var_bjc * assign5880_e5989);
        let assign5880_e5991: f64 = (assign5880_e5985 + assign5880_e5990);
        locals.var_vtexv = assign5880_e5991;
        locals.var_vtexv_dn0 = ((((locals.var_vdc_ctc_t_dn0 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn0 * assign5880_e5989) + (locals.var_bjc * (-locals.var_vjcex_dn0))));
        locals.var_vtexv_dn1 = ((((locals.var_vdc_ctc_t_dn1 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn1 * assign5880_e5989) + (locals.var_bjc * (-locals.var_vjcex_dn1))));
        locals.var_vtexv_dn3 = ((((locals.var_vdc_ctc_t_dn3 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn3 * assign5880_e5989) + (locals.var_bjc * (-locals.var_vjcex_dn3))));
        locals.var_vtexv_dn4 = ((((locals.var_vdc_ctc_t_dn4 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn4 * assign5880_e5989) + (locals.var_bjc * (-locals.var_vjcex_dn4))));
        locals.var_vtexv_dn5 = ((((locals.var_vdc_ctc_t_dn5 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn5 * assign5880_e5989) + (locals.var_bjc * (locals.var_vb1c4_dn5 - locals.var_vjcex_dn5))));
        locals.var_vtexv_dn6 = ((((locals.var_vdc_ctc_t_dn6 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn6 * assign5880_e5989) + (locals.var_bjc * (locals.var_vb1c4_dn6 - locals.var_vjcex_dn6))));
        locals.var_vtexv_dn7 = ((((locals.var_vdc_ctc_t_dn7 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn7 * assign5880_e5989) + (locals.var_bjc * (locals.var_vb1c4_dn7 - locals.var_vjcex_dn7))));
        locals.var_vtexv_dn8 = ((((locals.var_vdc_ctc_t_dn8 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn8 * assign5880_e5989) + (locals.var_bjc * (locals.var_vb1c4_dn8 - locals.var_vjcex_dn8))));
        locals.var_vtexv_dn9 = ((((locals.var_vdc_ctc_t_dn9 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn9 * assign5880_e5989) + (locals.var_bjc * (-locals.var_vjcex_dn9))));
        locals.var_vtexv_dn10 = ((((locals.var_vdc_ctc_t_dn10 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn10 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn10 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn10 * assign5880_e5989) + (locals.var_bjc * (locals.var_vb1c4_dn10 - locals.var_vjcex_dn10))));
        locals.var_vtexv_rv = 0.0;

        let assign5890_e5995: f64 = (1.0 - locals.var_xp_t);
        let assign5890_e5997: f64 = (assign5890_e5995 * locals.var_vtexv);
        let assign5890_e6000: f64 = (locals.var_xp_t * locals.var_vb1c4);
        let assign5890_e6001: f64 = (assign5890_e5997 + assign5890_e6000);
        let assign5890_e6002: f64 = (locals.var_cjc_t * assign5890_e6001);
        let assign5890_e6005: f64 = (1.0 - p.p76);
        let assign5890_e6006: f64 = (assign5890_e6002 * assign5890_e6005);
        let assign5890_e6009: f64 = (1.0 - p.p32);
        let assign5890_e6010: f64 = (assign5890_e6006 * assign5890_e6009);
        locals.var_qtex = assign5890_e6010;
        locals.var_qtex_dn0 = ((((locals.var_cjc_t_dn0 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn0) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn0)) + (locals.var_xp_t_dn0 * locals.var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn1 = ((((locals.var_cjc_t_dn1 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn1) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn1)) + (locals.var_xp_t_dn1 * locals.var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn3 = ((((locals.var_cjc_t_dn3 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn3) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn3)) + (locals.var_xp_t_dn3 * locals.var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn4 = ((((locals.var_cjc_t_dn4 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn4) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn4)) + (locals.var_xp_t_dn4 * locals.var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn5 = ((((locals.var_cjc_t_dn5 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn5) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn5)) + ((locals.var_xp_t_dn5 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn5))))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn6 = ((((locals.var_cjc_t_dn6 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn6) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn6)) + ((locals.var_xp_t_dn6 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn6))))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn7 = ((((locals.var_cjc_t_dn7 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn7) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn7)) + ((locals.var_xp_t_dn7 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn7))))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn8 = ((((locals.var_cjc_t_dn8 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn8) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn8)) + ((locals.var_xp_t_dn8 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn8))))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn9 = ((((locals.var_cjc_t_dn9 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn9) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn9)) + (locals.var_xp_t_dn9 * locals.var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn10 = ((((locals.var_cjc_t_dn10 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn10) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn10)) + ((locals.var_xp_t_dn10 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn10))))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_rv = 0.0;

        let assign5900_e6013: f64 = (locals.var_vbc3 - locals.var_vfc);
        let assign5900_e6015: f64 = (assign5900_e6013 / locals.var_a_vdcctc);
        locals.var_dxa = assign5900_e6015;
        locals.var_dxa_dn0 = ((((locals.var_vbc3_dn0 - locals.var_vfc_dn0) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn0)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn1 = ((((locals.var_vbc3_dn1 - locals.var_vfc_dn1) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn1)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn3 = ((((-locals.var_vfc_dn3) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn3)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn4 = ((((-locals.var_vfc_dn4) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn4)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn5 = ((((locals.var_vbc3_dn5 - locals.var_vfc_dn5) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn5)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn6 = ((((locals.var_vbc3_dn6 - locals.var_vfc_dn6) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn6)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn7 = ((((locals.var_vbc3_dn7 - locals.var_vfc_dn7) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn7)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn8 = ((((locals.var_vbc3_dn8 - locals.var_vfc_dn8) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn8)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn9 = ((((locals.var_vbc3_dn9 - locals.var_vfc_dn9) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn9)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn10 = ((((locals.var_vbc3_dn10 - locals.var_vfc_dn10) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn10)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_rv = 0.0;

        let assign5910_e6018: f64 = if locals.var_vbc3 < locals.var_vfc { 1.0 } else { 0.0 };
        locals.var_guard108 = assign5910_e6018;
        locals.var_guard108_rv = 0.0;

        let (assign5920_e6030, assign5920_e6030_d_n0, assign5920_e6030_d_n1, assign5920_e6030_d_n3, assign5920_e6030_d_n4, assign5920_e6030_d_n5, assign5920_e6030_d_n6, assign5920_e6030_d_n7, assign5920_e6030_d_n8, assign5920_e6030_d_n9, assign5920_e6030_d_n10,) = {
    if (locals.var_guard108 != 0.0) {
        let assign5920_e6024: f64 = (locals.var_dxa).exp();
        let assign5920_e6025: f64 = (1.0 + assign5920_e6024);
        let assign5920_e6026: f64 = (assign5920_e6025).ln();
        let assign5920_e6027: f64 = (locals.var_a_vdcctc * assign5920_e6026);
        let assign5920_e6028: f64 = (locals.var_vbc3 - assign5920_e6027);
        (assign5920_e6028, (locals.var_vbc3_dn0 - ((locals.var_a_vdcctc_dn0 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn0) / assign5920_e6025)))), (locals.var_vbc3_dn1 - ((locals.var_a_vdcctc_dn1 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn1) / assign5920_e6025)))), (-((locals.var_a_vdcctc_dn3 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn3) / assign5920_e6025)))), (-((locals.var_a_vdcctc_dn4 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn4) / assign5920_e6025)))), (locals.var_vbc3_dn5 - ((locals.var_a_vdcctc_dn5 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn5) / assign5920_e6025)))), (locals.var_vbc3_dn6 - ((locals.var_a_vdcctc_dn6 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn6) / assign5920_e6025)))), (locals.var_vbc3_dn7 - ((locals.var_a_vdcctc_dn7 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn7) / assign5920_e6025)))), (locals.var_vbc3_dn8 - ((locals.var_a_vdcctc_dn8 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn8) / assign5920_e6025)))), (locals.var_vbc3_dn9 - ((locals.var_a_vdcctc_dn9 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn9) / assign5920_e6025)))), (locals.var_vbc3_dn10 - ((locals.var_a_vdcctc_dn10 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn10) / assign5920_e6025)))),)
    } else {
        (locals.var_xvjcex, locals.var_xvjcex_dn0, locals.var_xvjcex_dn1, locals.var_xvjcex_dn3, locals.var_xvjcex_dn4, locals.var_xvjcex_dn5, locals.var_xvjcex_dn6, locals.var_xvjcex_dn7, locals.var_xvjcex_dn8, locals.var_xvjcex_dn9, locals.var_xvjcex_dn10,)
    }
};
        locals.var_xvjcex = assign5920_e6030;
        locals.var_xvjcex_dn0 = assign5920_e6030_d_n0;
        locals.var_xvjcex_dn1 = assign5920_e6030_d_n1;
        locals.var_xvjcex_dn3 = assign5920_e6030_d_n3;
        locals.var_xvjcex_dn4 = assign5920_e6030_d_n4;
        locals.var_xvjcex_dn5 = assign5920_e6030_d_n5;
        locals.var_xvjcex_dn6 = assign5920_e6030_d_n6;
        locals.var_xvjcex_dn7 = assign5920_e6030_d_n7;
        locals.var_xvjcex_dn8 = assign5920_e6030_d_n8;
        locals.var_xvjcex_dn9 = assign5920_e6030_d_n9;
        locals.var_xvjcex_dn10 = assign5920_e6030_d_n10;
        locals.var_xvjcex_rv = 0.0;

        let (assign5930_e6044, assign5930_e6044_d_n0, assign5930_e6044_d_n1, assign5930_e6044_d_n3, assign5930_e6044_d_n4, assign5930_e6044_d_n5, assign5930_e6044_d_n6, assign5930_e6044_d_n7, assign5930_e6044_d_n8, assign5930_e6044_d_n9, assign5930_e6044_d_n10,) = {
    if (locals.var_guard108 == 0.0) {
        let assign5930_e6037: f64 = (-locals.var_dxa);
        let assign5930_e6038: f64 = (assign5930_e6037).exp();
        let assign5930_e6039: f64 = (1.0 + assign5930_e6038);
        let assign5930_e6040: f64 = (assign5930_e6039).ln();
        let assign5930_e6041: f64 = (locals.var_a_vdcctc * assign5930_e6040);
        let assign5930_e6042: f64 = (locals.var_vfc - assign5930_e6041);
        (assign5930_e6042, (locals.var_vfc_dn0 - ((locals.var_a_vdcctc_dn0 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn0)) / assign5930_e6039)))), (locals.var_vfc_dn1 - ((locals.var_a_vdcctc_dn1 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn1)) / assign5930_e6039)))), (locals.var_vfc_dn3 - ((locals.var_a_vdcctc_dn3 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn3)) / assign5930_e6039)))), (locals.var_vfc_dn4 - ((locals.var_a_vdcctc_dn4 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn4)) / assign5930_e6039)))), (locals.var_vfc_dn5 - ((locals.var_a_vdcctc_dn5 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn5)) / assign5930_e6039)))), (locals.var_vfc_dn6 - ((locals.var_a_vdcctc_dn6 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn6)) / assign5930_e6039)))), (locals.var_vfc_dn7 - ((locals.var_a_vdcctc_dn7 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn7)) / assign5930_e6039)))), (locals.var_vfc_dn8 - ((locals.var_a_vdcctc_dn8 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn8)) / assign5930_e6039)))), (locals.var_vfc_dn9 - ((locals.var_a_vdcctc_dn9 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn9)) / assign5930_e6039)))), (locals.var_vfc_dn10 - ((locals.var_a_vdcctc_dn10 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn10)) / assign5930_e6039)))),)
    } else {
        (locals.var_xvjcex, locals.var_xvjcex_dn0, locals.var_xvjcex_dn1, locals.var_xvjcex_dn3, locals.var_xvjcex_dn4, locals.var_xvjcex_dn5, locals.var_xvjcex_dn6, locals.var_xvjcex_dn7, locals.var_xvjcex_dn8, locals.var_xvjcex_dn9, locals.var_xvjcex_dn10,)
    }
};
        locals.var_xvjcex = assign5930_e6044;
        locals.var_xvjcex_dn0 = assign5930_e6044_d_n0;
        locals.var_xvjcex_dn1 = assign5930_e6044_d_n1;
        locals.var_xvjcex_dn3 = assign5930_e6044_d_n3;
        locals.var_xvjcex_dn4 = assign5930_e6044_d_n4;
        locals.var_xvjcex_dn5 = assign5930_e6044_d_n5;
        locals.var_xvjcex_dn6 = assign5930_e6044_d_n6;
        locals.var_xvjcex_dn7 = assign5930_e6044_d_n7;
        locals.var_xvjcex_dn8 = assign5930_e6044_d_n8;
        locals.var_xvjcex_dn9 = assign5930_e6044_d_n9;
        locals.var_xvjcex_dn10 = assign5930_e6044_d_n10;
        locals.var_xvjcex_rv = 0.0;

        let assign5940_e6048: f64 = (1.0 - p.p71);
        let assign5940_e6049: f64 = (locals.var_vdc_ctc_t / assign5940_e6048);
        let assign5940_e6054: f64 = (locals.var_xvjcex / locals.var_vdc_ctc_t);
        let assign5940_e6055: f64 = (1.0 - assign5940_e6054);
        let assign5940_e6058: f64 = (1.0 - p.p71);
        let assign5940_e6059: f64 = (assign5940_e6055).powf(assign5940_e6058);
        let assign5940_e6060: f64 = (1.0 - assign5940_e6059);
        let assign5940_e6061: f64 = (assign5940_e6049 * assign5940_e6060);
        let assign5940_e6065: f64 = (locals.var_vbc3 - locals.var_xvjcex);
        let assign5940_e6066: f64 = (locals.var_bjc * assign5940_e6065);
        let assign5940_e6067: f64 = (assign5940_e6061 + assign5940_e6066);
        locals.var_xvtexv = assign5940_e6067;
        locals.var_xvtexv_dn0 = ((((locals.var_vdc_ctc_t_dn0 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn0 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn0 - locals.var_xvjcex_dn0))));
        locals.var_xvtexv_dn1 = ((((locals.var_vdc_ctc_t_dn1 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn1 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn1 - locals.var_xvjcex_dn1))));
        locals.var_xvtexv_dn3 = ((((locals.var_vdc_ctc_t_dn3 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn3 * assign5940_e6065) + (locals.var_bjc * (-locals.var_xvjcex_dn3))));
        locals.var_xvtexv_dn4 = ((((locals.var_vdc_ctc_t_dn4 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn4 * assign5940_e6065) + (locals.var_bjc * (-locals.var_xvjcex_dn4))));
        locals.var_xvtexv_dn5 = ((((locals.var_vdc_ctc_t_dn5 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn5 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn5 - locals.var_xvjcex_dn5))));
        locals.var_xvtexv_dn6 = ((((locals.var_vdc_ctc_t_dn6 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn6 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn6 - locals.var_xvjcex_dn6))));
        locals.var_xvtexv_dn7 = ((((locals.var_vdc_ctc_t_dn7 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn7 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn7 - locals.var_xvjcex_dn7))));
        locals.var_xvtexv_dn8 = ((((locals.var_vdc_ctc_t_dn8 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn8 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn8 - locals.var_xvjcex_dn8))));
        locals.var_xvtexv_dn9 = ((((locals.var_vdc_ctc_t_dn9 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn9 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn9 - locals.var_xvjcex_dn9))));
        locals.var_xvtexv_dn10 = ((((locals.var_vdc_ctc_t_dn10 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn10 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn10 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn10 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn10 - locals.var_xvjcex_dn10))));
        locals.var_xvtexv_rv = 0.0;

        let assign5950_e6071: f64 = (1.0 - locals.var_xp_t);
        let assign5950_e6073: f64 = (assign5950_e6071 * locals.var_xvtexv);
        let assign5950_e6076: f64 = (locals.var_xp_t * locals.var_vbc3);
        let assign5950_e6077: f64 = (assign5950_e6073 + assign5950_e6076);
        let assign5950_e6078: f64 = (locals.var_cjc_t * assign5950_e6077);
        let assign5950_e6081: f64 = (1.0 - p.p76);
        let assign5950_e6082: f64 = (assign5950_e6078 * assign5950_e6081);
        let assign5950_e6084: f64 = (assign5950_e6082 * p.p32);
        locals.var_xqtex = assign5950_e6084;
        locals.var_xqtex_dn0 = ((((locals.var_cjc_t_dn0 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn0) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn0)) + ((locals.var_xp_t_dn0 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn0))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn1 = ((((locals.var_cjc_t_dn1 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn1) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn1)) + ((locals.var_xp_t_dn1 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn1))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn3 = ((((locals.var_cjc_t_dn3 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn3) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn3)) + (locals.var_xp_t_dn3 * locals.var_vbc3)))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn4 = ((((locals.var_cjc_t_dn4 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn4) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn4)) + (locals.var_xp_t_dn4 * locals.var_vbc3)))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn5 = ((((locals.var_cjc_t_dn5 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn5) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn5)) + ((locals.var_xp_t_dn5 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn5))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn6 = ((((locals.var_cjc_t_dn6 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn6) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn6)) + ((locals.var_xp_t_dn6 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn6))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn7 = ((((locals.var_cjc_t_dn7 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn7) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn7)) + ((locals.var_xp_t_dn7 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn7))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn8 = ((((locals.var_cjc_t_dn8 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn8) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn8)) + ((locals.var_xp_t_dn8 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn8))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn9 = ((((locals.var_cjc_t_dn9 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn9) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn9)) + ((locals.var_xp_t_dn9 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn9))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn10 = ((((locals.var_cjc_t_dn10 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn10) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn10)) + ((locals.var_xp_t_dn10 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn10))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_rv = 0.0;

        let assign5960_e6087: f64 = (locals.var_taue_t * locals.var_ik_t);
        let assign5960_e6090: f64 = (locals.var_is_t / locals.var_ik_t);
        let assign5960_e6093: f64 = (1.0 / p.p84);
        let assign5960_e6094: f64 = (assign5960_e6090).powf(assign5960_e6093);
        let assign5960_e6095: f64 = (assign5960_e6087 * assign5960_e6094);
        locals.var_qe0 = assign5960_e6095;
        locals.var_qe0_dn0 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn0 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn0 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn1 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn1 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn1 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn3 = ((((locals.var_taue_t_dn3 * locals.var_ik_t) + (locals.var_taue_t * locals.var_ik_t_dn3)) * assign5960_e6094) + (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (((locals.var_is_t_dn3 * locals.var_ik_t) - (locals.var_is_t * locals.var_ik_t_dn3)) / (locals.var_ik_t * locals.var_ik_t)))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((((locals.var_is_t_dn3 * locals.var_ik_t) - (locals.var_is_t * locals.var_ik_t_dn3)) / (locals.var_ik_t * locals.var_ik_t)) / assign5960_e6090))) }));
        locals.var_qe0_dn4 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn4 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn4 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn5 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn5 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn5 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn6 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn6 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn6 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn7 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn7 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn7 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn8 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn8 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn8 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn9 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn9 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn9 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn10 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn10 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn10 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_rv = 0.0;

        let assign5970_e6099: f64 = (p.p84 * locals.var_vt);
        let assign5970_e6100: f64 = (locals.var_vb2e1 / assign5970_e6099);
        let assign5970_e6102: f64 = if assign5970_e6100 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign5970_e6102;
        locals.var_guard109_rv = 0.0;

        let (assign5980_e6111, assign5980_e6111_d_n0, assign5980_e6111_d_n1, assign5980_e6111_d_n3, assign5980_e6111_d_n4, assign5980_e6111_d_n5, assign5980_e6111_d_n6, assign5980_e6111_d_n7, assign5980_e6111_d_n8, assign5980_e6111_d_n9, assign5980_e6111_d_n10,) = {
    if (locals.var_guard109 != 0.0) {
        let assign5980_e6107: f64 = (p.p84 * locals.var_vt);
        let assign5980_e6108: f64 = (locals.var_vb2e1 / assign5980_e6107);
        let assign5980_e6109: f64 = (assign5980_e6108).exp();
        (assign5980_e6109, 0.0, 0.0, (assign5980_e6109 * (-((locals.var_vb2e1 * (p.p84 * locals.var_vt_dn3)) / (assign5980_e6107 * assign5980_e6107)))), (assign5980_e6109 * (locals.var_vb2e1_dn4 / assign5980_e6107)), 0.0, (assign5980_e6109 * (locals.var_vb2e1_dn6 / assign5980_e6107)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign5980_e6111;
        locals.var_tmpexp_dn0 = assign5980_e6111_d_n0;
        locals.var_tmpexp_dn1 = assign5980_e6111_d_n1;
        locals.var_tmpexp_dn3 = assign5980_e6111_d_n3;
        locals.var_tmpexp_dn4 = assign5980_e6111_d_n4;
        locals.var_tmpexp_dn5 = assign5980_e6111_d_n5;
        locals.var_tmpexp_dn6 = assign5980_e6111_d_n6;
        locals.var_tmpexp_dn7 = assign5980_e6111_d_n7;
        locals.var_tmpexp_dn8 = assign5980_e6111_d_n8;
        locals.var_tmpexp_dn9 = assign5980_e6111_d_n9;
        locals.var_tmpexp_dn10 = assign5980_e6111_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let (assign5990_e6117,) = {
    if (locals.var_guard109 == 0.0) {
        let assign5990_e6115: f64 = (p.p138).exp();
        (assign5990_e6115,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5990_e6117;
        locals.var_expl_rv = 0.0;

        let (assign6000_e6132, assign6000_e6132_d_n0, assign6000_e6132_d_n1, assign6000_e6132_d_n3, assign6000_e6132_d_n4, assign6000_e6132_d_n5, assign6000_e6132_d_n6, assign6000_e6132_d_n7, assign6000_e6132_d_n8, assign6000_e6132_d_n9, assign6000_e6132_d_n10,) = {
    if (locals.var_guard109 == 0.0) {
        let assign6000_e6125: f64 = (p.p84 * locals.var_vt);
        let assign6000_e6126: f64 = (locals.var_vb2e1 / assign6000_e6125);
        let assign6000_e6128: f64 = (assign6000_e6126 - p.p138);
        let assign6000_e6129: f64 = (1.0 + assign6000_e6128);
        let assign6000_e6130: f64 = (locals.var_expl * assign6000_e6129);
        (assign6000_e6130, 0.0, 0.0, (locals.var_expl * (-((locals.var_vb2e1 * (p.p84 * locals.var_vt_dn3)) / (assign6000_e6125 * assign6000_e6125)))), (locals.var_expl * (locals.var_vb2e1_dn4 / assign6000_e6125)), 0.0, (locals.var_expl * (locals.var_vb2e1_dn6 / assign6000_e6125)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign6000_e6132;
        locals.var_tmpexp_dn0 = assign6000_e6132_d_n0;
        locals.var_tmpexp_dn1 = assign6000_e6132_d_n1;
        locals.var_tmpexp_dn3 = assign6000_e6132_d_n3;
        locals.var_tmpexp_dn4 = assign6000_e6132_d_n4;
        locals.var_tmpexp_dn5 = assign6000_e6132_d_n5;
        locals.var_tmpexp_dn6 = assign6000_e6132_d_n6;
        locals.var_tmpexp_dn7 = assign6000_e6132_d_n7;
        locals.var_tmpexp_dn8 = assign6000_e6132_d_n8;
        locals.var_tmpexp_dn9 = assign6000_e6132_d_n9;
        locals.var_tmpexp_dn10 = assign6000_e6132_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let assign6010_e6135: f64 = (locals.var_qe0 * locals.var_tmpexp);
        locals.var_qe_qs = assign6010_e6135;
        locals.var_qe_qs_dn0 = ((locals.var_qe0_dn0 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn0));
        locals.var_qe_qs_dn1 = ((locals.var_qe0_dn1 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn1));
        locals.var_qe_qs_dn3 = ((locals.var_qe0_dn3 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn3));
        locals.var_qe_qs_dn4 = ((locals.var_qe0_dn4 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn4));
        locals.var_qe_qs_dn5 = ((locals.var_qe0_dn5 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn5));
        locals.var_qe_qs_dn6 = ((locals.var_qe0_dn6 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn6));
        locals.var_qe_qs_dn7 = ((locals.var_qe0_dn7 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn7));
        locals.var_qe_qs_dn8 = ((locals.var_qe0_dn8 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn8));
        locals.var_qe_qs_dn9 = ((locals.var_qe0_dn9 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn9));
        locals.var_qe_qs_dn10 = ((locals.var_qe0_dn10 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn10));
        locals.var_qe_qs_rv = 0.0;

        let assign6020_e6138: f64 = (4.0 * locals.var_tepi_t);
        let assign6020_e6140: f64 = (assign6020_e6138 * locals.var_vt);
        let assign6020_e6142: f64 = (assign6020_e6140 / locals.var_rcv_t);
        locals.var_qepi0 = assign6020_e6142;
        locals.var_qepi0_dn3 = ((((((4.0 * locals.var_tepi_t_dn3) * locals.var_vt) + (assign6020_e6138 * locals.var_vt_dn3)) * locals.var_rcv_t) - (assign6020_e6140 * locals.var_rcv_t_dn3)) / (locals.var_rcv_t * locals.var_rcv_t));
        locals.var_qepi0_rv = 0.0;

        let assign6030_e6145: f64 = (0.5 * locals.var_qepi0);
        let assign6030_e6147: f64 = (assign6030_e6145 * locals.var_xi_w);
        let assign6030_e6150: f64 = (locals.var_p0star + locals.var_pw);
        let assign6030_e6152: f64 = (assign6030_e6150 + 2.0);
        let assign6030_e6153: f64 = (assign6030_e6147 * assign6030_e6152);
        locals.var_qepi = assign6030_e6153;
        locals.var_qepi_dn0 = (((assign6030_e6145 * locals.var_xi_w_dn0) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn0 + locals.var_pw_dn0)));
        locals.var_qepi_dn1 = (((assign6030_e6145 * locals.var_xi_w_dn1) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn1 + locals.var_pw_dn1)));
        locals.var_qepi_dn3 = (((((0.5 * locals.var_qepi0_dn3) * locals.var_xi_w) + (assign6030_e6145 * locals.var_xi_w_dn3)) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn3 + locals.var_pw_dn3)));
        locals.var_qepi_dn4 = (((assign6030_e6145 * locals.var_xi_w_dn4) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn4 + locals.var_pw_dn4)));
        locals.var_qepi_dn5 = (((assign6030_e6145 * locals.var_xi_w_dn5) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn5 + locals.var_pw_dn5)));
        locals.var_qepi_dn6 = (((assign6030_e6145 * locals.var_xi_w_dn6) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn6 + locals.var_pw_dn6)));
        locals.var_qepi_dn7 = (((assign6030_e6145 * locals.var_xi_w_dn7) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn7 + locals.var_pw_dn7)));
        locals.var_qepi_dn8 = (((assign6030_e6145 * locals.var_xi_w_dn8) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn8 + locals.var_pw_dn8)));
        locals.var_qepi_dn9 = (((assign6030_e6145 * locals.var_xi_w_dn9) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn9 + locals.var_pw_dn9)));
        locals.var_qepi_dn10 = (((assign6030_e6145 * locals.var_xi_w_dn10) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn10 + locals.var_pw_dn10)));
        locals.var_qepi_rv = 0.0;

        let assign6040_e6156: f64 = if p.p78 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign6040_e6156;
        locals.var_guard110_rv = 0.0;

        let (assign6050_e6174, assign6050_e6174_d_n0, assign6050_e6174_d_n1, assign6050_e6174_d_n3, assign6050_e6174_d_n4, assign6050_e6174_d_n5, assign6050_e6174_d_n6, assign6050_e6174_d_n7, assign6050_e6174_d_n8, assign6050_e6174_d_n9, assign6050_e6174_d_n10,) = {
    if (locals.var_guard110 != 0.0) {
        let assign6050_e6160: f64 = (locals.var_taur_t * 0.5);
        let assign6050_e6163: f64 = (locals.var_qb0 * locals.var_nbex);
        let assign6050_e6166: f64 = (locals.var_qepi0 * locals.var_pwex);
        let assign6050_e6167: f64 = (assign6050_e6163 + assign6050_e6166);
        let assign6050_e6168: f64 = (assign6050_e6160 * assign6050_e6167);
        let assign6050_e6171: f64 = (locals.var_taub_t + locals.var_tepi_t);
        let assign6050_e6172: f64 = (assign6050_e6168 / assign6050_e6171);
        (assign6050_e6172, ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn0) + (locals.var_qepi0 * locals.var_pwex_dn0))) / assign6050_e6171), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn1) + (locals.var_qepi0 * locals.var_pwex_dn1))) / assign6050_e6171), ((((((locals.var_taur_t_dn3 * 0.5) * assign6050_e6167) + (assign6050_e6160 * (((locals.var_qb0_dn3 * locals.var_nbex) + (locals.var_qb0 * locals.var_nbex_dn3)) + ((locals.var_qepi0_dn3 * locals.var_pwex) + (locals.var_qepi0 * locals.var_pwex_dn3))))) * assign6050_e6171) - (assign6050_e6168 * (locals.var_taub_t_dn3 + locals.var_tepi_t_dn3))) / (assign6050_e6171 * assign6050_e6171)), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn4) + (locals.var_qepi0 * locals.var_pwex_dn4))) / assign6050_e6171), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn5) + (locals.var_qepi0 * locals.var_pwex_dn5))) / assign6050_e6171), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn6) + (locals.var_qepi0 * locals.var_pwex_dn6))) / assign6050_e6171), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn7) + (locals.var_qepi0 * locals.var_pwex_dn7))) / assign6050_e6171), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn8) + (locals.var_qepi0 * locals.var_pwex_dn8))) / assign6050_e6171), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn9) + (locals.var_qepi0 * locals.var_pwex_dn9))) / assign6050_e6171), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn10) + (locals.var_qepi0 * locals.var_pwex_dn10))) / assign6050_e6171),)
    } else {
        (locals.var_qex, locals.var_qex_dn0, locals.var_qex_dn1, locals.var_qex_dn3, locals.var_qex_dn4, locals.var_qex_dn5, locals.var_qex_dn6, locals.var_qex_dn7, locals.var_qex_dn8, locals.var_qex_dn9, locals.var_qex_dn10,)
    }
};
        locals.var_qex = assign6050_e6174;
        locals.var_qex_dn0 = assign6050_e6174_d_n0;
        locals.var_qex_dn1 = assign6050_e6174_d_n1;
        locals.var_qex_dn3 = assign6050_e6174_d_n3;
        locals.var_qex_dn4 = assign6050_e6174_d_n4;
        locals.var_qex_dn5 = assign6050_e6174_d_n5;
        locals.var_qex_dn6 = assign6050_e6174_d_n6;
        locals.var_qex_dn7 = assign6050_e6174_d_n7;
        locals.var_qex_dn8 = assign6050_e6174_d_n8;
        locals.var_qex_dn9 = assign6050_e6174_d_n9;
        locals.var_qex_dn10 = assign6050_e6174_d_n10;
        locals.var_qex_rv = 0.0;

        let assign6060_e6177: f64 = (locals.var_vb1c4 - locals.var_vdcex_t);
        let assign6060_e6179: f64 = (assign6060_e6177 / p.p90);
        let assign6060_e6181: f64 = (assign6060_e6179 * locals.var_vtinv);
        let assign6060_e6183: f64 = if assign6060_e6181 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign6060_e6183;
        locals.var_guard111_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6070_e6197, assign6070_e6197_d_n0, assign6070_e6197_d_n1, assign6070_e6197_d_n3, assign6070_e6197_d_n4, assign6070_e6197_d_n5, assign6070_e6197_d_n6, assign6070_e6197_d_n7, assign6070_e6197_d_n8, assign6070_e6197_d_n9, assign6070_e6197_d_n10,) = {
    if ((locals.var_guard110 == 0.0) && (locals.var_guard111 != 0.0)) {
        let assign6070_e6190: f64 = (locals.var_vb1c4 - locals.var_vdcex_t);
        let assign6070_e6192: f64 = (assign6070_e6190 / p.p90);
        let assign6070_e6194: f64 = (assign6070_e6192 * locals.var_vtinv);
        let assign6070_e6195: f64 = (assign6070_e6194).exp();
        (assign6070_e6195, (assign6070_e6195 * (((-locals.var_vdcex_t_dn0) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * (((-locals.var_vdcex_t_dn1) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * ((((-locals.var_vdcex_t_dn3) / p.p90) * locals.var_vtinv) + (assign6070_e6192 * locals.var_vtinv_dn3))), (assign6070_e6195 * (((-locals.var_vdcex_t_dn4) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * (((locals.var_vb1c4_dn5 - locals.var_vdcex_t_dn5) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * (((locals.var_vb1c4_dn6 - locals.var_vdcex_t_dn6) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * (((locals.var_vb1c4_dn7 - locals.var_vdcex_t_dn7) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * (((locals.var_vb1c4_dn8 - locals.var_vdcex_t_dn8) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * (((-locals.var_vdcex_t_dn9) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * (((locals.var_vb1c4_dn10 - locals.var_vdcex_t_dn10) / p.p90) * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4vdcex, locals.var_evb1c4vdcex_dn0, locals.var_evb1c4vdcex_dn1, locals.var_evb1c4vdcex_dn3, locals.var_evb1c4vdcex_dn4, locals.var_evb1c4vdcex_dn5, locals.var_evb1c4vdcex_dn6, locals.var_evb1c4vdcex_dn7, locals.var_evb1c4vdcex_dn8, locals.var_evb1c4vdcex_dn9, locals.var_evb1c4vdcex_dn10,)
    }
};
        locals.var_evb1c4vdcex = assign6070_e6197;
        locals.var_evb1c4vdcex_dn0 = assign6070_e6197_d_n0;
        locals.var_evb1c4vdcex_dn1 = assign6070_e6197_d_n1;
        locals.var_evb1c4vdcex_dn3 = assign6070_e6197_d_n3;
        locals.var_evb1c4vdcex_dn4 = assign6070_e6197_d_n4;
        locals.var_evb1c4vdcex_dn5 = assign6070_e6197_d_n5;
        locals.var_evb1c4vdcex_dn6 = assign6070_e6197_d_n6;
        locals.var_evb1c4vdcex_dn7 = assign6070_e6197_d_n7;
        locals.var_evb1c4vdcex_dn8 = assign6070_e6197_d_n8;
        locals.var_evb1c4vdcex_dn9 = assign6070_e6197_d_n9;
        locals.var_evb1c4vdcex_dn10 = assign6070_e6197_d_n10;
        locals.var_evb1c4vdcex_rv = 0.0;

        let (assign6080_e6206,) = {
    if ((locals.var_guard110 == 0.0) && (locals.var_guard111 == 0.0)) {
        let assign6080_e6204: f64 = (p.p138).exp();
        (assign6080_e6204,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign6080_e6206;
        locals.var_expl_rv = 0.0;

        let (assign6090_e6226, assign6090_e6226_d_n0, assign6090_e6226_d_n1, assign6090_e6226_d_n3, assign6090_e6226_d_n4, assign6090_e6226_d_n5, assign6090_e6226_d_n6, assign6090_e6226_d_n7, assign6090_e6226_d_n8, assign6090_e6226_d_n9, assign6090_e6226_d_n10,) = {
    if ((locals.var_guard110 == 0.0) && (locals.var_guard111 == 0.0)) {
        let assign6090_e6216: f64 = (locals.var_vb1c4 - locals.var_vdcex_t);
        let assign6090_e6218: f64 = (assign6090_e6216 / p.p90);
        let assign6090_e6220: f64 = (assign6090_e6218 * locals.var_vtinv);
        let assign6090_e6222: f64 = (assign6090_e6220 - p.p138);
        let assign6090_e6223: f64 = (1.0 + assign6090_e6222);
        let assign6090_e6224: f64 = (locals.var_expl * assign6090_e6223);
        (assign6090_e6224, (locals.var_expl * (((-locals.var_vdcex_t_dn0) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdcex_t_dn1) / p.p90) * locals.var_vtinv)), (locals.var_expl * ((((-locals.var_vdcex_t_dn3) / p.p90) * locals.var_vtinv) + (assign6090_e6218 * locals.var_vtinv_dn3))), (locals.var_expl * (((-locals.var_vdcex_t_dn4) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn5 - locals.var_vdcex_t_dn5) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn6 - locals.var_vdcex_t_dn6) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn7 - locals.var_vdcex_t_dn7) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn8 - locals.var_vdcex_t_dn8) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdcex_t_dn9) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn10 - locals.var_vdcex_t_dn10) / p.p90) * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4vdcex, locals.var_evb1c4vdcex_dn0, locals.var_evb1c4vdcex_dn1, locals.var_evb1c4vdcex_dn3, locals.var_evb1c4vdcex_dn4, locals.var_evb1c4vdcex_dn5, locals.var_evb1c4vdcex_dn6, locals.var_evb1c4vdcex_dn7, locals.var_evb1c4vdcex_dn8, locals.var_evb1c4vdcex_dn9, locals.var_evb1c4vdcex_dn10,)
    }
};
        locals.var_evb1c4vdcex = assign6090_e6226;
        locals.var_evb1c4vdcex_dn0 = assign6090_e6226_d_n0;
        locals.var_evb1c4vdcex_dn1 = assign6090_e6226_d_n1;
        locals.var_evb1c4vdcex_dn3 = assign6090_e6226_d_n3;
        locals.var_evb1c4vdcex_dn4 = assign6090_e6226_d_n4;
        locals.var_evb1c4vdcex_dn5 = assign6090_e6226_d_n5;
        locals.var_evb1c4vdcex_dn6 = assign6090_e6226_d_n6;
        locals.var_evb1c4vdcex_dn7 = assign6090_e6226_d_n7;
        locals.var_evb1c4vdcex_dn8 = assign6090_e6226_d_n8;
        locals.var_evb1c4vdcex_dn9 = assign6090_e6226_d_n9;
        locals.var_evb1c4vdcex_dn10 = assign6090_e6226_d_n10;
        locals.var_evb1c4vdcex_rv = 0.0;

        let (assign6100_e6246, assign6100_e6246_d_n0, assign6100_e6246_d_n1, assign6100_e6246_d_n3, assign6100_e6246_d_n4, assign6100_e6246_d_n5, assign6100_e6246_d_n6, assign6100_e6246_d_n7, assign6100_e6246_d_n8, assign6100_e6246_d_n9, assign6100_e6246_d_n10,) = {
    if (locals.var_guard110 == 0.0) {
        let assign6100_e6231: f64 = (2.0 * locals.var_ibx_t);
        let assign6100_e6233: f64 = (assign6100_e6231 * locals.var_tauex_t);
        let assign6100_e6235: f64 = (assign6100_e6233 * locals.var_evb1c4);
        let assign6100_e6240: f64 = (4.0 * locals.var_evb1c4vdcex);
        let assign6100_e6241: f64 = (1.0 + assign6100_e6240);
        let assign6100_e6242: f64 = (assign6100_e6241).sqrt();
        let assign6100_e6243: f64 = (1.0 + assign6100_e6242);
        let assign6100_e6244: f64 = (assign6100_e6235 / assign6100_e6243);
        (assign6100_e6244, (-((assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn0) / (2.0 * assign6100_e6242))) / (assign6100_e6243 * assign6100_e6243))), (-((assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn1) / (2.0 * assign6100_e6242))) / (assign6100_e6243 * assign6100_e6243))), ((((((((2.0 * locals.var_ibx_t_dn3) * locals.var_tauex_t) + (assign6100_e6231 * locals.var_tauex_t_dn3)) * locals.var_evb1c4) + (assign6100_e6233 * locals.var_evb1c4_dn3)) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn3) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), (-((assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn4) / (2.0 * assign6100_e6242))) / (assign6100_e6243 * assign6100_e6243))), ((((assign6100_e6233 * locals.var_evb1c4_dn5) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn5) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), ((((assign6100_e6233 * locals.var_evb1c4_dn6) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn6) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), ((((assign6100_e6233 * locals.var_evb1c4_dn7) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn7) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), ((((assign6100_e6233 * locals.var_evb1c4_dn8) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn8) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), (-((assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn9) / (2.0 * assign6100_e6242))) / (assign6100_e6243 * assign6100_e6243))), ((((assign6100_e6233 * locals.var_evb1c4_dn10) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn10) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)),)
    } else {
        (locals.var_qex, locals.var_qex_dn0, locals.var_qex_dn1, locals.var_qex_dn3, locals.var_qex_dn4, locals.var_qex_dn5, locals.var_qex_dn6, locals.var_qex_dn7, locals.var_qex_dn8, locals.var_qex_dn9, locals.var_qex_dn10,)
    }
};
        locals.var_qex = assign6100_e6246;
        locals.var_qex_dn0 = assign6100_e6246_d_n0;
        locals.var_qex_dn1 = assign6100_e6246_d_n1;
        locals.var_qex_dn3 = assign6100_e6246_d_n3;
        locals.var_qex_dn4 = assign6100_e6246_d_n4;
        locals.var_qex_dn5 = assign6100_e6246_d_n5;
        locals.var_qex_dn6 = assign6100_e6246_d_n6;
        locals.var_qex_dn7 = assign6100_e6246_d_n7;
        locals.var_qex_dn8 = assign6100_e6246_d_n8;
        locals.var_qex_dn9 = assign6100_e6246_d_n9;
        locals.var_qex_dn10 = assign6100_e6246_d_n10;
        locals.var_qex_rv = 0.0;

        let assign6110_e6257: f64 = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard112 = assign6110_e6257;
        locals.var_guard112_rv = 0.0;

        let (assign6120_e6263, assign6120_e6263_d_n0, assign6120_e6263_d_n1, assign6120_e6263_d_n3, assign6120_e6263_d_n4, assign6120_e6263_d_n5, assign6120_e6263_d_n6, assign6120_e6263_d_n7, assign6120_e6263_d_n8, assign6120_e6263_d_n9, assign6120_e6263_d_n10,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6120_e6261: f64 = (locals.var_qex * locals.var_xext1);
        (assign6120_e6261, (locals.var_qex_dn0 * locals.var_xext1), (locals.var_qex_dn1 * locals.var_xext1), (locals.var_qex_dn3 * locals.var_xext1), (locals.var_qex_dn4 * locals.var_xext1), (locals.var_qex_dn5 * locals.var_xext1), (locals.var_qex_dn6 * locals.var_xext1), (locals.var_qex_dn7 * locals.var_xext1), (locals.var_qex_dn8 * locals.var_xext1), (locals.var_qex_dn9 * locals.var_xext1), (locals.var_qex_dn10 * locals.var_xext1),)
    } else {
        (locals.var_qex, locals.var_qex_dn0, locals.var_qex_dn1, locals.var_qex_dn3, locals.var_qex_dn4, locals.var_qex_dn5, locals.var_qex_dn6, locals.var_qex_dn7, locals.var_qex_dn8, locals.var_qex_dn9, locals.var_qex_dn10,)
    }
};
        locals.var_qex = assign6120_e6263;
        locals.var_qex_dn0 = assign6120_e6263_d_n0;
        locals.var_qex_dn1 = assign6120_e6263_d_n1;
        locals.var_qex_dn3 = assign6120_e6263_d_n3;
        locals.var_qex_dn4 = assign6120_e6263_d_n4;
        locals.var_qex_dn5 = assign6120_e6263_d_n5;
        locals.var_qex_dn6 = assign6120_e6263_d_n6;
        locals.var_qex_dn7 = assign6120_e6263_d_n7;
        locals.var_qex_dn8 = assign6120_e6263_d_n8;
        locals.var_qex_dn9 = assign6120_e6263_d_n9;
        locals.var_qex_dn10 = assign6120_e6263_d_n10;
        locals.var_qex_rv = 0.0;

        let assign6130_e6266: f64 = if p.p78 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign6130_e6266;
        locals.var_guard113_rv = 0.0;

        let (assign6140_e6274, assign6140_e6274_d_n0, assign6140_e6274_d_n1, assign6140_e6274_d_n3, assign6140_e6274_d_n4, assign6140_e6274_d_n5, assign6140_e6274_d_n6, assign6140_e6274_d_n7, assign6140_e6274_d_n8, assign6140_e6274_d_n9, assign6140_e6274_d_n10,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard113 != 0.0)) {
        let assign6140_e6272: f64 = (locals.var_if0 * locals.var_evbc3);
        (assign6140_e6272, ((locals.var_if0_dn0 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn0)), ((locals.var_if0_dn1 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn1)), ((locals.var_if0_dn3 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn3)), (locals.var_if0_dn4 * locals.var_evbc3), ((locals.var_if0_dn5 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn5)), ((locals.var_if0_dn6 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn6)), ((locals.var_if0_dn7 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn7)), ((locals.var_if0_dn8 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn8)), ((locals.var_if0_dn9 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn9)), ((locals.var_if0_dn10 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn10)),)
    } else {
        (locals.var_xg1, locals.var_xg1_dn0, locals.var_xg1_dn1, locals.var_xg1_dn3, locals.var_xg1_dn4, locals.var_xg1_dn5, locals.var_xg1_dn6, locals.var_xg1_dn7, locals.var_xg1_dn8, locals.var_xg1_dn9, locals.var_xg1_dn10,)
    }
};
        locals.var_xg1 = assign6140_e6274;
        locals.var_xg1_dn0 = assign6140_e6274_d_n0;
        locals.var_xg1_dn1 = assign6140_e6274_d_n1;
        locals.var_xg1_dn3 = assign6140_e6274_d_n3;
        locals.var_xg1_dn4 = assign6140_e6274_d_n4;
        locals.var_xg1_dn5 = assign6140_e6274_d_n5;
        locals.var_xg1_dn6 = assign6140_e6274_d_n6;
        locals.var_xg1_dn7 = assign6140_e6274_d_n7;
        locals.var_xg1_dn8 = assign6140_e6274_d_n8;
        locals.var_xg1_dn9 = assign6140_e6274_d_n9;
        locals.var_xg1_dn10 = assign6140_e6274_d_n10;
        locals.var_xg1_rv = 0.0;

        let (assign6150_e6289, assign6150_e6289_d_n0, assign6150_e6289_d_n1, assign6150_e6289_d_n3, assign6150_e6289_d_n4, assign6150_e6289_d_n5, assign6150_e6289_d_n6, assign6150_e6289_d_n7, assign6150_e6289_d_n8, assign6150_e6289_d_n9, assign6150_e6289_d_n10,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard113 != 0.0)) {
        let assign6150_e6280: f64 = (locals.var_xg1 - locals.var_if0);
        let assign6150_e6284: f64 = (1.0 + locals.var_xg1);
        let assign6150_e6285: f64 = (assign6150_e6284).sqrt();
        let assign6150_e6286: f64 = (1.0 + assign6150_e6285);
        let assign6150_e6287: f64 = (assign6150_e6280 / assign6150_e6286);
        (assign6150_e6287, ((((locals.var_xg1_dn0 - locals.var_if0_dn0) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn0 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn1 - locals.var_if0_dn1) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn1 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn3 - locals.var_if0_dn3) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn3 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn4 - locals.var_if0_dn4) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn4 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn5 - locals.var_if0_dn5) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn5 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn6 - locals.var_if0_dn6) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn6 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn7 - locals.var_if0_dn7) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn7 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn8 - locals.var_if0_dn8) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn8 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn9 - locals.var_if0_dn9) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn9 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn10 - locals.var_if0_dn10) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn10 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)),)
    } else {
        (locals.var_xnbex, locals.var_xnbex_dn0, locals.var_xnbex_dn1, locals.var_xnbex_dn3, locals.var_xnbex_dn4, locals.var_xnbex_dn5, locals.var_xnbex_dn6, locals.var_xnbex_dn7, locals.var_xnbex_dn8, locals.var_xnbex_dn9, locals.var_xnbex_dn10,)
    }
};
        locals.var_xnbex = assign6150_e6289;
        locals.var_xnbex_dn0 = assign6150_e6289_d_n0;
        locals.var_xnbex_dn1 = assign6150_e6289_d_n1;
        locals.var_xnbex_dn3 = assign6150_e6289_d_n3;
        locals.var_xnbex_dn4 = assign6150_e6289_d_n4;
        locals.var_xnbex_dn5 = assign6150_e6289_d_n5;
        locals.var_xnbex_dn6 = assign6150_e6289_d_n6;
        locals.var_xnbex_dn7 = assign6150_e6289_d_n7;
        locals.var_xnbex_dn8 = assign6150_e6289_d_n8;
        locals.var_xnbex_dn9 = assign6150_e6289_d_n9;
        locals.var_xnbex_dn10 = assign6150_e6289_d_n10;
        locals.var_xnbex_rv = 0.0;

        let (assign6160_e6297, assign6160_e6297_d_n0, assign6160_e6297_d_n1, assign6160_e6297_d_n3, assign6160_e6297_d_n4, assign6160_e6297_d_n5, assign6160_e6297_d_n6, assign6160_e6297_d_n7, assign6160_e6297_d_n8, assign6160_e6297_d_n9, assign6160_e6297_d_n10,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard113 != 0.0)) {
        let assign6160_e6295: f64 = (4.0 * locals.var_evbc3vdc);
        (assign6160_e6295, (4.0 * locals.var_evbc3vdc_dn0), (4.0 * locals.var_evbc3vdc_dn1), (4.0 * locals.var_evbc3vdc_dn3), (4.0 * locals.var_evbc3vdc_dn4), (4.0 * locals.var_evbc3vdc_dn5), (4.0 * locals.var_evbc3vdc_dn6), (4.0 * locals.var_evbc3vdc_dn7), (4.0 * locals.var_evbc3vdc_dn8), (4.0 * locals.var_evbc3vdc_dn9), (4.0 * locals.var_evbc3vdc_dn10),)
    } else {
        (locals.var_xg2, locals.var_xg2_dn0, locals.var_xg2_dn1, locals.var_xg2_dn3, locals.var_xg2_dn4, locals.var_xg2_dn5, locals.var_xg2_dn6, locals.var_xg2_dn7, locals.var_xg2_dn8, locals.var_xg2_dn9, locals.var_xg2_dn10,)
    }
};
        locals.var_xg2 = assign6160_e6297;
        locals.var_xg2_dn0 = assign6160_e6297_d_n0;
        locals.var_xg2_dn1 = assign6160_e6297_d_n1;
        locals.var_xg2_dn3 = assign6160_e6297_d_n3;
        locals.var_xg2_dn4 = assign6160_e6297_d_n4;
        locals.var_xg2_dn5 = assign6160_e6297_d_n5;
        locals.var_xg2_dn6 = assign6160_e6297_d_n6;
        locals.var_xg2_dn7 = assign6160_e6297_d_n7;
        locals.var_xg2_dn8 = assign6160_e6297_d_n8;
        locals.var_xg2_dn9 = assign6160_e6297_d_n9;
        locals.var_xg2_dn10 = assign6160_e6297_d_n10;
        locals.var_xg2_rv = 0.0;

        let (assign6170_e6310, assign6170_e6310_d_n0, assign6170_e6310_d_n1, assign6170_e6310_d_n3, assign6170_e6310_d_n4, assign6170_e6310_d_n5, assign6170_e6310_d_n6, assign6170_e6310_d_n7, assign6170_e6310_d_n8, assign6170_e6310_d_n9, assign6170_e6310_d_n10,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard113 != 0.0)) {
        let assign6170_e6305: f64 = (1.0 + locals.var_xg2);
        let assign6170_e6306: f64 = (assign6170_e6305).sqrt();
        let assign6170_e6307: f64 = (1.0 + assign6170_e6306);
        let assign6170_e6308: f64 = (locals.var_xg2 / assign6170_e6307);
        (assign6170_e6308, (((locals.var_xg2_dn0 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn0 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn1 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn1 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn3 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn3 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn4 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn4 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn5 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn5 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn6 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn6 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn7 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn7 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn8 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn8 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn9 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn9 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn10 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn10 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)),)
    } else {
        (locals.var_xpwex, locals.var_xpwex_dn0, locals.var_xpwex_dn1, locals.var_xpwex_dn3, locals.var_xpwex_dn4, locals.var_xpwex_dn5, locals.var_xpwex_dn6, locals.var_xpwex_dn7, locals.var_xpwex_dn8, locals.var_xpwex_dn9, locals.var_xpwex_dn10,)
    }
};
        locals.var_xpwex = assign6170_e6310;
        locals.var_xpwex_dn0 = assign6170_e6310_d_n0;
        locals.var_xpwex_dn1 = assign6170_e6310_d_n1;
        locals.var_xpwex_dn3 = assign6170_e6310_d_n3;
        locals.var_xpwex_dn4 = assign6170_e6310_d_n4;
        locals.var_xpwex_dn5 = assign6170_e6310_d_n5;
        locals.var_xpwex_dn6 = assign6170_e6310_d_n6;
        locals.var_xpwex_dn7 = assign6170_e6310_d_n7;
        locals.var_xpwex_dn8 = assign6170_e6310_d_n8;
        locals.var_xpwex_dn9 = assign6170_e6310_d_n9;
        locals.var_xpwex_dn10 = assign6170_e6310_d_n10;
        locals.var_xpwex_rv = 0.0;

        let (assign6180_e6332, assign6180_e6332_d_n0, assign6180_e6332_d_n1, assign6180_e6332_d_n3, assign6180_e6332_d_n4, assign6180_e6332_d_n5, assign6180_e6332_d_n6, assign6180_e6332_d_n7, assign6180_e6332_d_n8, assign6180_e6332_d_n9, assign6180_e6332_d_n10,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard113 != 0.0)) {
        let assign6180_e6316: f64 = (0.5 * p.p32);
        let assign6180_e6318: f64 = (assign6180_e6316 * locals.var_taur_t);
        let assign6180_e6321: f64 = (locals.var_qb0 * locals.var_xnbex);
        let assign6180_e6324: f64 = (locals.var_qepi0 * locals.var_xpwex);
        let assign6180_e6325: f64 = (assign6180_e6321 + assign6180_e6324);
        let assign6180_e6326: f64 = (assign6180_e6318 * assign6180_e6325);
        let assign6180_e6329: f64 = (locals.var_taub_t + locals.var_tepi_t);
        let assign6180_e6330: f64 = (assign6180_e6326 / assign6180_e6329);
        (assign6180_e6330, ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn0) + (locals.var_qepi0 * locals.var_xpwex_dn0))) / assign6180_e6329), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn1) + (locals.var_qepi0 * locals.var_xpwex_dn1))) / assign6180_e6329), ((((((assign6180_e6316 * locals.var_taur_t_dn3) * assign6180_e6325) + (assign6180_e6318 * (((locals.var_qb0_dn3 * locals.var_xnbex) + (locals.var_qb0 * locals.var_xnbex_dn3)) + ((locals.var_qepi0_dn3 * locals.var_xpwex) + (locals.var_qepi0 * locals.var_xpwex_dn3))))) * assign6180_e6329) - (assign6180_e6326 * (locals.var_taub_t_dn3 + locals.var_tepi_t_dn3))) / (assign6180_e6329 * assign6180_e6329)), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn4) + (locals.var_qepi0 * locals.var_xpwex_dn4))) / assign6180_e6329), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn5) + (locals.var_qepi0 * locals.var_xpwex_dn5))) / assign6180_e6329), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn6) + (locals.var_qepi0 * locals.var_xpwex_dn6))) / assign6180_e6329), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn7) + (locals.var_qepi0 * locals.var_xpwex_dn7))) / assign6180_e6329), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn8) + (locals.var_qepi0 * locals.var_xpwex_dn8))) / assign6180_e6329), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn9) + (locals.var_qepi0 * locals.var_xpwex_dn9))) / assign6180_e6329), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn10) + (locals.var_qepi0 * locals.var_xpwex_dn10))) / assign6180_e6329),)
    } else {
        (locals.var_xqmex, locals.var_xqmex_dn0, locals.var_xqmex_dn1, locals.var_xqmex_dn3, locals.var_xqmex_dn4, locals.var_xqmex_dn5, locals.var_xqmex_dn6, locals.var_xqmex_dn7, locals.var_xqmex_dn8, locals.var_xqmex_dn9, locals.var_xqmex_dn10,)
    }
};
        locals.var_xqmex = assign6180_e6332;
        locals.var_xqmex_dn0 = assign6180_e6332_d_n0;
        locals.var_xqmex_dn1 = assign6180_e6332_d_n1;
        locals.var_xqmex_dn3 = assign6180_e6332_d_n3;
        locals.var_xqmex_dn4 = assign6180_e6332_d_n4;
        locals.var_xqmex_dn5 = assign6180_e6332_d_n5;
        locals.var_xqmex_dn6 = assign6180_e6332_d_n6;
        locals.var_xqmex_dn7 = assign6180_e6332_d_n7;
        locals.var_xqmex_dn8 = assign6180_e6332_d_n8;
        locals.var_xqmex_dn9 = assign6180_e6332_d_n9;
        locals.var_xqmex_dn10 = assign6180_e6332_d_n10;
        locals.var_xqmex_rv = 0.0;

        let assign6190_e6335: f64 = (locals.var_vbc3 - locals.var_vdcex_t);
        let assign6190_e6337: f64 = (assign6190_e6335 * locals.var_vtinv);
        let assign6190_e6339: f64 = if assign6190_e6337 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard114 = assign6190_e6339;
        locals.var_guard114_rv = 0.0;

        let (assign6200_e6353, assign6200_e6353_d_n0, assign6200_e6353_d_n1, assign6200_e6353_d_n3, assign6200_e6353_d_n4, assign6200_e6353_d_n5, assign6200_e6353_d_n6, assign6200_e6353_d_n7, assign6200_e6353_d_n8, assign6200_e6353_d_n9, assign6200_e6353_d_n10,) = {
    if (((locals.var_guard112 != 0.0) && (locals.var_guard113 == 0.0)) && (locals.var_guard114 != 0.0)) {
        let assign6200_e6348: f64 = (locals.var_vbc3 - locals.var_vdcex_t);
        let assign6200_e6350: f64 = (assign6200_e6348 * locals.var_vtinv);
        let assign6200_e6351: f64 = (assign6200_e6350).exp();
        (assign6200_e6351, (assign6200_e6351 * ((locals.var_vbc3_dn0 - locals.var_vdcex_t_dn0) * locals.var_vtinv)), (assign6200_e6351 * ((locals.var_vbc3_dn1 - locals.var_vdcex_t_dn1) * locals.var_vtinv)), (assign6200_e6351 * (((-locals.var_vdcex_t_dn3) * locals.var_vtinv) + (assign6200_e6348 * locals.var_vtinv_dn3))), (assign6200_e6351 * ((-locals.var_vdcex_t_dn4) * locals.var_vtinv)), (assign6200_e6351 * ((locals.var_vbc3_dn5 - locals.var_vdcex_t_dn5) * locals.var_vtinv)), (assign6200_e6351 * ((locals.var_vbc3_dn6 - locals.var_vdcex_t_dn6) * locals.var_vtinv)), (assign6200_e6351 * ((locals.var_vbc3_dn7 - locals.var_vdcex_t_dn7) * locals.var_vtinv)), (assign6200_e6351 * ((locals.var_vbc3_dn8 - locals.var_vdcex_t_dn8) * locals.var_vtinv)), (assign6200_e6351 * ((locals.var_vbc3_dn9 - locals.var_vdcex_t_dn9) * locals.var_vtinv)), (assign6200_e6351 * ((locals.var_vbc3_dn10 - locals.var_vdcex_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3vdcex, locals.var_evbc3vdcex_dn0, locals.var_evbc3vdcex_dn1, locals.var_evbc3vdcex_dn3, locals.var_evbc3vdcex_dn4, locals.var_evbc3vdcex_dn5, locals.var_evbc3vdcex_dn6, locals.var_evbc3vdcex_dn7, locals.var_evbc3vdcex_dn8, locals.var_evbc3vdcex_dn9, locals.var_evbc3vdcex_dn10,)
    }
};
        locals.var_evbc3vdcex = assign6200_e6353;
        locals.var_evbc3vdcex_dn0 = assign6200_e6353_d_n0;
        locals.var_evbc3vdcex_dn1 = assign6200_e6353_d_n1;
        locals.var_evbc3vdcex_dn3 = assign6200_e6353_d_n3;
        locals.var_evbc3vdcex_dn4 = assign6200_e6353_d_n4;
        locals.var_evbc3vdcex_dn5 = assign6200_e6353_d_n5;
        locals.var_evbc3vdcex_dn6 = assign6200_e6353_d_n6;
        locals.var_evbc3vdcex_dn7 = assign6200_e6353_d_n7;
        locals.var_evbc3vdcex_dn8 = assign6200_e6353_d_n8;
        locals.var_evbc3vdcex_dn9 = assign6200_e6353_d_n9;
        locals.var_evbc3vdcex_dn10 = assign6200_e6353_d_n10;
        locals.var_evbc3vdcex_rv = 0.0;

        let (assign6210_e6364,) = {
    if (((locals.var_guard112 != 0.0) && (locals.var_guard113 == 0.0)) && (locals.var_guard114 == 0.0)) {
        let assign6210_e6362: f64 = (p.p138).exp();
        (assign6210_e6362,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign6210_e6364;
        locals.var_expl_rv = 0.0;

        let (assign6220_e6384, assign6220_e6384_d_n0, assign6220_e6384_d_n1, assign6220_e6384_d_n3, assign6220_e6384_d_n4, assign6220_e6384_d_n5, assign6220_e6384_d_n6, assign6220_e6384_d_n7, assign6220_e6384_d_n8, assign6220_e6384_d_n9, assign6220_e6384_d_n10,) = {
    if (((locals.var_guard112 != 0.0) && (locals.var_guard113 == 0.0)) && (locals.var_guard114 == 0.0)) {
        let assign6220_e6376: f64 = (locals.var_vbc3 - locals.var_vdcex_t);
        let assign6220_e6378: f64 = (assign6220_e6376 * locals.var_vtinv);
        let assign6220_e6380: f64 = (assign6220_e6378 - p.p138);
        let assign6220_e6381: f64 = (1.0 + assign6220_e6380);
        let assign6220_e6382: f64 = (locals.var_expl * assign6220_e6381);
        (assign6220_e6382, (locals.var_expl * ((locals.var_vbc3_dn0 - locals.var_vdcex_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn1 - locals.var_vdcex_t_dn1) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdcex_t_dn3) * locals.var_vtinv) + (assign6220_e6376 * locals.var_vtinv_dn3))), (locals.var_expl * ((-locals.var_vdcex_t_dn4) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn5 - locals.var_vdcex_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn6 - locals.var_vdcex_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn7 - locals.var_vdcex_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn8 - locals.var_vdcex_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn9 - locals.var_vdcex_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn10 - locals.var_vdcex_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3vdcex, locals.var_evbc3vdcex_dn0, locals.var_evbc3vdcex_dn1, locals.var_evbc3vdcex_dn3, locals.var_evbc3vdcex_dn4, locals.var_evbc3vdcex_dn5, locals.var_evbc3vdcex_dn6, locals.var_evbc3vdcex_dn7, locals.var_evbc3vdcex_dn8, locals.var_evbc3vdcex_dn9, locals.var_evbc3vdcex_dn10,)
    }
};
        locals.var_evbc3vdcex = assign6220_e6384;
        locals.var_evbc3vdcex_dn0 = assign6220_e6384_d_n0;
        locals.var_evbc3vdcex_dn1 = assign6220_e6384_d_n1;
        locals.var_evbc3vdcex_dn3 = assign6220_e6384_d_n3;
        locals.var_evbc3vdcex_dn4 = assign6220_e6384_d_n4;
        locals.var_evbc3vdcex_dn5 = assign6220_e6384_d_n5;
        locals.var_evbc3vdcex_dn6 = assign6220_e6384_d_n6;
        locals.var_evbc3vdcex_dn7 = assign6220_e6384_d_n7;
        locals.var_evbc3vdcex_dn8 = assign6220_e6384_d_n8;
        locals.var_evbc3vdcex_dn9 = assign6220_e6384_d_n9;
        locals.var_evbc3vdcex_dn10 = assign6220_e6384_d_n10;
        locals.var_evbc3vdcex_rv = 0.0;

        let (assign6230_e6408, assign6230_e6408_d_n0, assign6230_e6408_d_n1, assign6230_e6408_d_n3, assign6230_e6408_d_n4, assign6230_e6408_d_n5, assign6230_e6408_d_n6, assign6230_e6408_d_n7, assign6230_e6408_d_n8, assign6230_e6408_d_n9, assign6230_e6408_d_n10,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard113 == 0.0)) {
        let assign6230_e6391: f64 = (2.0 * p.p32);
        let assign6230_e6393: f64 = (assign6230_e6391 * locals.var_ibx_t);
        let assign6230_e6395: f64 = (assign6230_e6393 * locals.var_tauex_t);
        let assign6230_e6397: f64 = (assign6230_e6395 * locals.var_evbc3);
        let assign6230_e6402: f64 = (4.0 * locals.var_evbc3vdcex);
        let assign6230_e6403: f64 = (1.0 + assign6230_e6402);
        let assign6230_e6404: f64 = (assign6230_e6403).sqrt();
        let assign6230_e6405: f64 = (1.0 + assign6230_e6404);
        let assign6230_e6406: f64 = (assign6230_e6397 / assign6230_e6405);
        (assign6230_e6406, ((((assign6230_e6395 * locals.var_evbc3_dn0) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn0) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * locals.var_evbc3_dn1) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn1) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((((((assign6230_e6391 * locals.var_ibx_t_dn3) * locals.var_tauex_t) + (assign6230_e6393 * locals.var_tauex_t_dn3)) * locals.var_evbc3) + (assign6230_e6395 * locals.var_evbc3_dn3)) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn3) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), (-((assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn4) / (2.0 * assign6230_e6404))) / (assign6230_e6405 * assign6230_e6405))), ((((assign6230_e6395 * locals.var_evbc3_dn5) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn5) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * locals.var_evbc3_dn6) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn6) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * locals.var_evbc3_dn7) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn7) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * locals.var_evbc3_dn8) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn8) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * locals.var_evbc3_dn9) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn9) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * locals.var_evbc3_dn10) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn10) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)),)
    } else {
        (locals.var_xqmex, locals.var_xqmex_dn0, locals.var_xqmex_dn1, locals.var_xqmex_dn3, locals.var_xqmex_dn4, locals.var_xqmex_dn5, locals.var_xqmex_dn6, locals.var_xqmex_dn7, locals.var_xqmex_dn8, locals.var_xqmex_dn9, locals.var_xqmex_dn10,)
    }
};
        locals.var_xqmex = assign6230_e6408;
        locals.var_xqmex_dn0 = assign6230_e6408_d_n0;
        locals.var_xqmex_dn1 = assign6230_e6408_d_n1;
        locals.var_xqmex_dn3 = assign6230_e6408_d_n3;
        locals.var_xqmex_dn4 = assign6230_e6408_d_n4;
        locals.var_xqmex_dn5 = assign6230_e6408_d_n5;
        locals.var_xqmex_dn6 = assign6230_e6408_d_n6;
        locals.var_xqmex_dn7 = assign6230_e6408_d_n7;
        locals.var_xqmex_dn8 = assign6230_e6408_d_n8;
        locals.var_xqmex_dn9 = assign6230_e6408_d_n9;
        locals.var_xqmex_dn10 = assign6230_e6408_d_n10;
        locals.var_xqmex_rv = 0.0;

        let (assign6240_e6414, assign6240_e6414_d_n0, assign6240_e6414_d_n1, assign6240_e6414_d_n3, assign6240_e6414_d_n4, assign6240_e6414_d_n5, assign6240_e6414_d_n6, assign6240_e6414_d_n7, assign6240_e6414_d_n8, assign6240_e6414_d_n9, assign6240_e6414_d_n10,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6240_e6412: f64 = (locals.var_fex * locals.var_xqmex);
        (assign6240_e6412, ((locals.var_fex_dn0 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn0)), ((locals.var_fex_dn1 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn1)), ((locals.var_fex_dn3 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn3)), ((locals.var_fex_dn4 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn4)), ((locals.var_fex_dn5 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn5)), ((locals.var_fex_dn6 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn6)), ((locals.var_fex_dn7 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn7)), ((locals.var_fex_dn8 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn8)), ((locals.var_fex_dn9 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn9)), ((locals.var_fex_dn10 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn10)),)
    } else {
        (locals.var_xqex, locals.var_xqex_dn0, locals.var_xqex_dn1, locals.var_xqex_dn3, locals.var_xqex_dn4, locals.var_xqex_dn5, locals.var_xqex_dn6, locals.var_xqex_dn7, locals.var_xqex_dn8, locals.var_xqex_dn9, locals.var_xqex_dn10,)
    }
};
        locals.var_xqex = assign6240_e6414;
        locals.var_xqex_dn0 = assign6240_e6414_d_n0;
        locals.var_xqex_dn1 = assign6240_e6414_d_n1;
        locals.var_xqex_dn3 = assign6240_e6414_d_n3;
        locals.var_xqex_dn4 = assign6240_e6414_d_n4;
        locals.var_xqex_dn5 = assign6240_e6414_d_n5;
        locals.var_xqex_dn6 = assign6240_e6414_d_n6;
        locals.var_xqex_dn7 = assign6240_e6414_d_n7;
        locals.var_xqex_dn8 = assign6240_e6414_d_n8;
        locals.var_xqex_dn9 = assign6240_e6414_d_n9;
        locals.var_xqex_dn10 = assign6240_e6414_d_n10;
        locals.var_xqex_rv = 0.0;

        let assign6250_e6417: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard115 = assign6250_e6417;
        locals.var_guard115_rv = 0.0;

        let (assign6260_e6430, assign6260_e6430_d_n0, assign6260_e6430_d_n1, assign6260_e6430_d_n3, assign6260_e6430_d_n4, assign6260_e6430_d_n5, assign6260_e6430_d_n6, assign6260_e6430_d_n7, assign6260_e6430_d_n8, assign6260_e6430_d_n9, assign6260_e6430_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6260_e6422: f64 = (locals.var_vje * locals.var_inv_vde_t);
        let assign6260_e6423: f64 = (1.0 - assign6260_e6422);
        let assign6260_e6425: f64 = (-p.p66);
        let assign6260_e6426: f64 = (assign6260_e6423).powf(assign6260_e6425);
        let assign6260_e6428: f64 = (assign6260_e6426 - 3.0);
        (assign6260_e6428, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn0 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn0))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn0 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn0))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn1 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn1))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn1 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn1))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn3 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn3))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn3 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn3))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn4 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn4))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn4 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn4))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn5 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn5))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn5 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn5))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn6 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn6))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn6 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn6))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn7 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn7))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn7 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn7))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn8 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn8))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn8 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn8))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn9 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn9))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn9 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn9))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn10 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn10))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn10 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn10))) / assign6260_e6423))) },)
    } else {
        (locals.var_dvtevje, locals.var_dvtevje_dn0, locals.var_dvtevje_dn1, locals.var_dvtevje_dn3, locals.var_dvtevje_dn4, locals.var_dvtevje_dn5, locals.var_dvtevje_dn6, locals.var_dvtevje_dn7, locals.var_dvtevje_dn8, locals.var_dvtevje_dn9, locals.var_dvtevje_dn10,)
    }
};
        locals.var_dvtevje = assign6260_e6430;
        locals.var_dvtevje_dn0 = assign6260_e6430_d_n0;
        locals.var_dvtevje_dn1 = assign6260_e6430_d_n1;
        locals.var_dvtevje_dn3 = assign6260_e6430_d_n3;
        locals.var_dvtevje_dn4 = assign6260_e6430_d_n4;
        locals.var_dvtevje_dn5 = assign6260_e6430_d_n5;
        locals.var_dvtevje_dn6 = assign6260_e6430_d_n6;
        locals.var_dvtevje_dn7 = assign6260_e6430_d_n7;
        locals.var_dvtevje_dn8 = assign6260_e6430_d_n8;
        locals.var_dvtevje_dn9 = assign6260_e6430_d_n9;
        locals.var_dvtevje_dn10 = assign6260_e6430_d_n10;
        locals.var_dvtevje_rv = 0.0;

        let (assign6270_e6438, assign6270_e6438_d_n0, assign6270_e6438_d_n1, assign6270_e6438_d_n3, assign6270_e6438_d_n4, assign6270_e6438_d_n5, assign6270_e6438_d_n6, assign6270_e6438_d_n7, assign6270_e6438_d_n8, assign6270_e6438_d_n9, assign6270_e6438_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6270_e6434: f64 = (locals.var_vb2e1 - locals.var_vfe);
        let assign6270_e6436: f64 = (assign6270_e6434 / locals.var_a_vde);
        (assign6270_e6436, ((((-locals.var_vfe_dn0) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn0)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn1) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn1)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn3) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn3)) / (locals.var_a_vde * locals.var_a_vde)), ((((locals.var_vb2e1_dn4 - locals.var_vfe_dn4) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn4)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn5) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn5)) / (locals.var_a_vde * locals.var_a_vde)), ((((locals.var_vb2e1_dn6 - locals.var_vfe_dn6) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn6)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn7) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn7)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn8) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn8)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn9) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn9)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn10) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn10)) / (locals.var_a_vde * locals.var_a_vde)),)
    } else {
        (locals.var_vb2e1vfe, locals.var_vb2e1vfe_dn0, locals.var_vb2e1vfe_dn1, locals.var_vb2e1vfe_dn3, locals.var_vb2e1vfe_dn4, locals.var_vb2e1vfe_dn5, locals.var_vb2e1vfe_dn6, locals.var_vb2e1vfe_dn7, locals.var_vb2e1vfe_dn8, locals.var_vb2e1vfe_dn9, locals.var_vb2e1vfe_dn10,)
    }
};
        locals.var_vb2e1vfe = assign6270_e6438;
        locals.var_vb2e1vfe_dn0 = assign6270_e6438_d_n0;
        locals.var_vb2e1vfe_dn1 = assign6270_e6438_d_n1;
        locals.var_vb2e1vfe_dn3 = assign6270_e6438_d_n3;
        locals.var_vb2e1vfe_dn4 = assign6270_e6438_d_n4;
        locals.var_vb2e1vfe_dn5 = assign6270_e6438_d_n5;
        locals.var_vb2e1vfe_dn6 = assign6270_e6438_d_n6;
        locals.var_vb2e1vfe_dn7 = assign6270_e6438_d_n7;
        locals.var_vb2e1vfe_dn8 = assign6270_e6438_d_n8;
        locals.var_vb2e1vfe_dn9 = assign6270_e6438_d_n9;
        locals.var_vb2e1vfe_dn10 = assign6270_e6438_d_n10;
        locals.var_vb2e1vfe_rv = 0.0;

        let assign6280_e6441: f64 = if locals.var_vb2e1vfe < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign6280_e6441;
        locals.var_guard116_rv = 0.0;

        let (assign6290_e6452, assign6290_e6452_d_n0, assign6290_e6452_d_n1, assign6290_e6452_d_n3, assign6290_e6452_d_n4, assign6290_e6452_d_n5, assign6290_e6452_d_n6, assign6290_e6452_d_n7, assign6290_e6452_d_n8, assign6290_e6452_d_n9, assign6290_e6452_d_n10,) = {
    if ((locals.var_guard115 != 0.0) && (locals.var_guard116 != 0.0)) {
        let assign6290_e6448: f64 = (locals.var_vb2e1vfe).exp();
        let assign6290_e6449: f64 = (1.0 + assign6290_e6448);
        let assign6290_e6450: f64 = (1.0 / assign6290_e6449);
        (assign6290_e6450, (-((assign6290_e6448 * locals.var_vb2e1vfe_dn0) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn1) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn3) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn4) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn5) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn6) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn7) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn8) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn9) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn10) / (assign6290_e6449 * assign6290_e6449))),)
    } else {
        (locals.var_dvjevb2e1, locals.var_dvjevb2e1_dn0, locals.var_dvjevb2e1_dn1, locals.var_dvjevb2e1_dn3, locals.var_dvjevb2e1_dn4, locals.var_dvjevb2e1_dn5, locals.var_dvjevb2e1_dn6, locals.var_dvjevb2e1_dn7, locals.var_dvjevb2e1_dn8, locals.var_dvjevb2e1_dn9, locals.var_dvjevb2e1_dn10,)
    }
};
        locals.var_dvjevb2e1 = assign6290_e6452;
        locals.var_dvjevb2e1_dn0 = assign6290_e6452_d_n0;
        locals.var_dvjevb2e1_dn1 = assign6290_e6452_d_n1;
        locals.var_dvjevb2e1_dn3 = assign6290_e6452_d_n3;
        locals.var_dvjevb2e1_dn4 = assign6290_e6452_d_n4;
        locals.var_dvjevb2e1_dn5 = assign6290_e6452_d_n5;
        locals.var_dvjevb2e1_dn6 = assign6290_e6452_d_n6;
        locals.var_dvjevb2e1_dn7 = assign6290_e6452_d_n7;
        locals.var_dvjevb2e1_dn8 = assign6290_e6452_d_n8;
        locals.var_dvjevb2e1_dn9 = assign6290_e6452_d_n9;
        locals.var_dvjevb2e1_dn10 = assign6290_e6452_d_n10;
        locals.var_dvjevb2e1_rv = 0.0;

        let (assign6300_e6467, assign6300_e6467_d_n0, assign6300_e6467_d_n1, assign6300_e6467_d_n3, assign6300_e6467_d_n4, assign6300_e6467_d_n5, assign6300_e6467_d_n6, assign6300_e6467_d_n7, assign6300_e6467_d_n8, assign6300_e6467_d_n9, assign6300_e6467_d_n10,) = {
    if ((locals.var_guard115 != 0.0) && (locals.var_guard116 == 0.0)) {
        let assign6300_e6458: f64 = (-locals.var_vb2e1vfe);
        let assign6300_e6459: f64 = (assign6300_e6458).exp();
        let assign6300_e6462: f64 = (-locals.var_vb2e1vfe);
        let assign6300_e6463: f64 = (assign6300_e6462).exp();
        let assign6300_e6464: f64 = (1.0 + assign6300_e6463);
        let assign6300_e6465: f64 = (assign6300_e6459 / assign6300_e6464);
        (assign6300_e6465, ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn0)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn0)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn1)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn1)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn3)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn3)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn4)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn4)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn5)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn5)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn6)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn6)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn7)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn7)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn8)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn8)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn9)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn9)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn10)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn10)))) / (assign6300_e6464 * assign6300_e6464)),)
    } else {
        (locals.var_dvjevb2e1, locals.var_dvjevb2e1_dn0, locals.var_dvjevb2e1_dn1, locals.var_dvjevb2e1_dn3, locals.var_dvjevb2e1_dn4, locals.var_dvjevb2e1_dn5, locals.var_dvjevb2e1_dn6, locals.var_dvjevb2e1_dn7, locals.var_dvjevb2e1_dn8, locals.var_dvjevb2e1_dn9, locals.var_dvjevb2e1_dn10,)
    }
};
        locals.var_dvjevb2e1 = assign6300_e6467;
        locals.var_dvjevb2e1_dn0 = assign6300_e6467_d_n0;
        locals.var_dvjevb2e1_dn1 = assign6300_e6467_d_n1;
        locals.var_dvjevb2e1_dn3 = assign6300_e6467_d_n3;
        locals.var_dvjevb2e1_dn4 = assign6300_e6467_d_n4;
        locals.var_dvjevb2e1_dn5 = assign6300_e6467_d_n5;
        locals.var_dvjevb2e1_dn6 = assign6300_e6467_d_n6;
        locals.var_dvjevb2e1_dn7 = assign6300_e6467_d_n7;
        locals.var_dvjevb2e1_dn8 = assign6300_e6467_d_n8;
        locals.var_dvjevb2e1_dn9 = assign6300_e6467_d_n9;
        locals.var_dvjevb2e1_dn10 = assign6300_e6467_d_n10;
        locals.var_dvjevb2e1_rv = 0.0;

        let (assign6310_e6475, assign6310_e6475_d_n0, assign6310_e6475_d_n1, assign6310_e6475_d_n3, assign6310_e6475_d_n4, assign6310_e6475_d_n5, assign6310_e6475_d_n6, assign6310_e6475_d_n7, assign6310_e6475_d_n8, assign6310_e6475_d_n9, assign6310_e6475_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6310_e6471: f64 = (locals.var_dvtevje * locals.var_dvjevb2e1);
        let assign6310_e6473: f64 = (assign6310_e6471 + 3.0);
        (assign6310_e6473, ((locals.var_dvtevje_dn0 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn0)), ((locals.var_dvtevje_dn1 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn1)), ((locals.var_dvtevje_dn3 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn3)), ((locals.var_dvtevje_dn4 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn4)), ((locals.var_dvtevje_dn5 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn5)), ((locals.var_dvtevje_dn6 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn6)), ((locals.var_dvtevje_dn7 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn7)), ((locals.var_dvtevje_dn8 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn8)), ((locals.var_dvtevje_dn9 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn9)), ((locals.var_dvtevje_dn10 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn10)),)
    } else {
        (locals.var_dvtevb2e1, locals.var_dvtevb2e1_dn0, locals.var_dvtevb2e1_dn1, locals.var_dvtevb2e1_dn3, locals.var_dvtevb2e1_dn4, locals.var_dvtevb2e1_dn5, locals.var_dvtevb2e1_dn6, locals.var_dvtevb2e1_dn7, locals.var_dvtevb2e1_dn8, locals.var_dvtevb2e1_dn9, locals.var_dvtevb2e1_dn10,)
    }
};
        locals.var_dvtevb2e1 = assign6310_e6475;
        locals.var_dvtevb2e1_dn0 = assign6310_e6475_d_n0;
        locals.var_dvtevb2e1_dn1 = assign6310_e6475_d_n1;
        locals.var_dvtevb2e1_dn3 = assign6310_e6475_d_n3;
        locals.var_dvtevb2e1_dn4 = assign6310_e6475_d_n4;
        locals.var_dvtevb2e1_dn5 = assign6310_e6475_d_n5;
        locals.var_dvtevb2e1_dn6 = assign6310_e6475_d_n6;
        locals.var_dvtevb2e1_dn7 = assign6310_e6475_d_n7;
        locals.var_dvtevb2e1_dn8 = assign6310_e6475_d_n8;
        locals.var_dvtevb2e1_dn9 = assign6310_e6475_d_n9;
        locals.var_dvtevb2e1_dn10 = assign6310_e6475_d_n10;
        locals.var_dvtevb2e1_rv = 0.0;

        let (assign6320_e6485, assign6320_e6485_d_n0, assign6320_e6485_d_n1, assign6320_e6485_d_n3, assign6320_e6485_d_n4, assign6320_e6485_d_n5, assign6320_e6485_d_n6, assign6320_e6485_d_n7, assign6320_e6485_d_n8, assign6320_e6485_d_n9, assign6320_e6485_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6320_e6479: f64 = (1.0 - p.p67);
        let assign6320_e6481: f64 = (assign6320_e6479 * locals.var_cje_t);
        let assign6320_e6483: f64 = (assign6320_e6481 * locals.var_dvtevb2e1);
        (assign6320_e6483, (((assign6320_e6479 * locals.var_cje_t_dn0) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn0)), (((assign6320_e6479 * locals.var_cje_t_dn1) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn1)), (((assign6320_e6479 * locals.var_cje_t_dn3) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn3)), (((assign6320_e6479 * locals.var_cje_t_dn4) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn4)), (((assign6320_e6479 * locals.var_cje_t_dn5) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn5)), (((assign6320_e6479 * locals.var_cje_t_dn6) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn6)), (((assign6320_e6479 * locals.var_cje_t_dn7) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn7)), (((assign6320_e6479 * locals.var_cje_t_dn8) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn8)), (((assign6320_e6479 * locals.var_cje_t_dn9) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn9)), (((assign6320_e6479 * locals.var_cje_t_dn10) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn10)),)
    } else {
        (locals.var_dqtevb2e1, locals.var_dqtevb2e1_dn0, locals.var_dqtevb2e1_dn1, locals.var_dqtevb2e1_dn3, locals.var_dqtevb2e1_dn4, locals.var_dqtevb2e1_dn5, locals.var_dqtevb2e1_dn6, locals.var_dqtevb2e1_dn7, locals.var_dqtevb2e1_dn8, locals.var_dqtevb2e1_dn9, locals.var_dqtevb2e1_dn10,)
    }
};
        locals.var_dqtevb2e1 = assign6320_e6485;
        locals.var_dqtevb2e1_dn0 = assign6320_e6485_d_n0;
        locals.var_dqtevb2e1_dn1 = assign6320_e6485_d_n1;
        locals.var_dqtevb2e1_dn3 = assign6320_e6485_d_n3;
        locals.var_dqtevb2e1_dn4 = assign6320_e6485_d_n4;
        locals.var_dqtevb2e1_dn5 = assign6320_e6485_d_n5;
        locals.var_dqtevb2e1_dn6 = assign6320_e6485_d_n6;
        locals.var_dqtevb2e1_dn7 = assign6320_e6485_d_n7;
        locals.var_dqtevb2e1_dn8 = assign6320_e6485_d_n8;
        locals.var_dqtevb2e1_dn9 = assign6320_e6485_d_n9;
        locals.var_dqtevb2e1_dn10 = assign6320_e6485_d_n10;
        locals.var_dqtevb2e1_rv = 0.0;

    }
}
