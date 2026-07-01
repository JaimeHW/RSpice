#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign00_e541: f64 = if p.p3 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign00_e541;

        let (assign10_e545,) = {
    if (locals.var_guard1 != 0.0) {
        (70300000.0,)
    } else {
        (locals.var_an,)
    }
};
        locals.var_an = assign10_e545;

        let (assign20_e549,) = {
    if (locals.var_guard1 != 0.0) {
        (123000000.0,)
    } else {
        (locals.var_bn,)
    }
};
        locals.var_bn = assign20_e549;

        let (assign30_e554,) = {
    if (locals.var_guard1 == 0.0) {
        (158000000.0,)
    } else {
        (locals.var_an,)
    }
};
        locals.var_an = assign30_e554;

        let (assign40_e559,) = {
    if (locals.var_guard1 == 0.0) {
        (204000000.0,)
    } else {
        (locals.var_bn,)
    }
};
        locals.var_bn = assign40_e559;

        let assign50_e562: f64 = (1.0 - p.p32);
        locals.var_xext1 = assign50_e562;

        let assign60_e565: f64 = (p.p4 + 273.15);
        locals.var_trk = assign60_e565;

        let assign70_e566: f64 = ctx_temp;
        let assign70_e568: f64 = (assign70_e566 + p.p0);
        locals.var_tamb = assign70_e568;

        let assign90_e574: f64 = if p.p137 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2 = assign90_e574;

        let (assign100_e578,) = {
    if (locals.var_guard2 != 0.0) {
        (1e-12,)
    } else {
        (locals.var_minr,)
    }
};
        locals.var_minr = assign100_e578;

        let (assign110_e583,) = {
    if (locals.var_guard2 == 0.0) {
        (p.p137,)
    } else {
        (locals.var_minr,)
    }
};
        locals.var_minr = assign110_e583;

        let assign120_e586: f64 = (locals.var_minr * p.p1);
        locals.var_minr_m = assign120_e586;

        locals.var_eps_nf = 0.001;

        locals.var_eps_bavl_t = 0.001;

        let assign160_e595: f64 = (2.0 - p.p66);
        let assign160_e596: f64 = (2.0_f64).powf(assign160_e595);
        locals.var_pow2_2m_pe = assign160_e596;

        let assign180_e603: f64 = (p.p114 * locals.var_trk);
        let assign180_e605: f64 = (assign180_e603 * locals.var_trk);
        let assign180_e608: f64 = (locals.var_trk + p.p115);
        let assign180_e609: f64 = (assign180_e605 / assign180_e608);
        let assign180_e610: f64 = (p.p113 + assign180_e609);
        let assign180_e612: f64 = (assign180_e610 - 0.05);
        let assign180_e614: f64 = (assign180_e612 / 0.1);
        locals.var_dxa = assign180_e614;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;

        let assign190_e618: f64 = (p.p114 * locals.var_trk);
        let assign190_e620: f64 = (assign190_e618 * locals.var_trk);
        let assign190_e623: f64 = (locals.var_trk + p.p115);
        let assign190_e624: f64 = (assign190_e620 / assign190_e623);
        let assign190_e625: f64 = (p.p113 + assign190_e624);
        let assign190_e627: f64 = if assign190_e625 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard3 = assign190_e627;

        let (assign200_e639, assign200_e639_d_n0, assign200_e639_d_n1, assign200_e639_d_n3, assign200_e639_d_n4, assign200_e639_d_n5, assign200_e639_d_n6, assign200_e639_d_n7, assign200_e639_d_n8, assign200_e639_d_n9,) = {
    if (locals.var_guard3 != 0.0) {
        let assign200_e633: f64 = (locals.var_dxa).exp();
        let assign200_e634: f64 = (1.0 + assign200_e633);
        let assign200_e635: f64 = (assign200_e634).ln();
        let assign200_e636: f64 = (0.1 * assign200_e635);
        let assign200_e637: f64 = (0.05 + assign200_e636);
        (assign200_e637, (0.1 * ((assign200_e633 * locals.var_dxa_dn0) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn1) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn3) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn4) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn5) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn6) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn7) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn8) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn9) / assign200_e634)),)
    } else {
        (locals.var_vgzebok, locals.var_vgzebok_dn0, locals.var_vgzebok_dn1, locals.var_vgzebok_dn3, locals.var_vgzebok_dn4, locals.var_vgzebok_dn5, locals.var_vgzebok_dn6, locals.var_vgzebok_dn7, locals.var_vgzebok_dn8, locals.var_vgzebok_dn9,)
    }
};
        locals.var_vgzebok = assign200_e639;
        locals.var_vgzebok_dn0 = assign200_e639_d_n0;
        locals.var_vgzebok_dn1 = assign200_e639_d_n1;
        locals.var_vgzebok_dn3 = assign200_e639_d_n3;
        locals.var_vgzebok_dn4 = assign200_e639_d_n4;
        locals.var_vgzebok_dn5 = assign200_e639_d_n5;
        locals.var_vgzebok_dn6 = assign200_e639_d_n6;
        locals.var_vgzebok_dn7 = assign200_e639_d_n7;
        locals.var_vgzebok_dn8 = assign200_e639_d_n8;
        locals.var_vgzebok_dn9 = assign200_e639_d_n9;

        let (assign210_e663, assign210_e663_d_n0, assign210_e663_d_n1, assign210_e663_d_n3, assign210_e663_d_n4, assign210_e663_d_n5, assign210_e663_d_n6, assign210_e663_d_n7, assign210_e663_d_n8, assign210_e663_d_n9,) = {
    if (locals.var_guard3 == 0.0) {
        let assign210_e645: f64 = (p.p114 * locals.var_trk);
        let assign210_e647: f64 = (assign210_e645 * locals.var_trk);
        let assign210_e650: f64 = (locals.var_trk + p.p115);
        let assign210_e651: f64 = (assign210_e647 / assign210_e650);
        let assign210_e652: f64 = (p.p113 + assign210_e651);
        let assign210_e656: f64 = (-locals.var_dxa);
        let assign210_e657: f64 = (assign210_e656).exp();
        let assign210_e658: f64 = (1.0 + assign210_e657);
        let assign210_e659: f64 = (assign210_e658).ln();
        let assign210_e660: f64 = (0.1 * assign210_e659);
        let assign210_e661: f64 = (assign210_e652 + assign210_e660);
        (assign210_e661, (0.1 * ((assign210_e657 * (-locals.var_dxa_dn0)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn1)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn3)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn4)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn5)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn6)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn7)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn8)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn9)) / assign210_e658)),)
    } else {
        (locals.var_vgzebok, locals.var_vgzebok_dn0, locals.var_vgzebok_dn1, locals.var_vgzebok_dn3, locals.var_vgzebok_dn4, locals.var_vgzebok_dn5, locals.var_vgzebok_dn6, locals.var_vgzebok_dn7, locals.var_vgzebok_dn8, locals.var_vgzebok_dn9,)
    }
};
        locals.var_vgzebok = assign210_e663;
        locals.var_vgzebok_dn0 = assign210_e663_d_n0;
        locals.var_vgzebok_dn1 = assign210_e663_d_n1;
        locals.var_vgzebok_dn3 = assign210_e663_d_n3;
        locals.var_vgzebok_dn4 = assign210_e663_d_n4;
        locals.var_vgzebok_dn5 = assign210_e663_d_n5;
        locals.var_vgzebok_dn6 = assign210_e663_d_n6;
        locals.var_vgzebok_dn7 = assign210_e663_d_n7;
        locals.var_vgzebok_dn8 = assign210_e663_d_n8;
        locals.var_vgzebok_dn9 = assign210_e663_d_n9;

        locals.var_vgzeb_tr = p.p113;

        let assign230_e667: f64 = (1.0 / locals.var_vgzeb_tr);
        locals.var_inv_vgzeb_tr = assign230_e667;

        locals.var_vdc_zener = p.p70;

        locals.var_pc_zener = p.p71;

        let assign270_e676: f64 = (2.0 - locals.var_pc_zener);
        let assign270_e677: f64 = (2.0_f64).powf(assign270_e676);
        locals.var_pow2_2m_pc = assign270_e677;

        let assign290_e684: f64 = (p.p117 * locals.var_trk);
        let assign290_e686: f64 = (assign290_e684 * locals.var_trk);
        let assign290_e689: f64 = (locals.var_trk + p.p118);
        let assign290_e690: f64 = (assign290_e686 / assign290_e689);
        let assign290_e691: f64 = (p.p116 + assign290_e690);
        let assign290_e693: f64 = (assign290_e691 - 0.05);
        let assign290_e695: f64 = (assign290_e693 / 0.1);
        locals.var_dxa = assign290_e695;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;

        let assign300_e699: f64 = (p.p117 * locals.var_trk);
        let assign300_e701: f64 = (assign300_e699 * locals.var_trk);
        let assign300_e704: f64 = (locals.var_trk + p.p118);
        let assign300_e705: f64 = (assign300_e701 / assign300_e704);
        let assign300_e706: f64 = (p.p116 + assign300_e705);
        let assign300_e708: f64 = if assign300_e706 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard4 = assign300_e708;

        let (assign310_e720, assign310_e720_d_n0, assign310_e720_d_n1, assign310_e720_d_n3, assign310_e720_d_n4, assign310_e720_d_n5, assign310_e720_d_n6, assign310_e720_d_n7, assign310_e720_d_n8, assign310_e720_d_n9,) = {
    if (locals.var_guard4 != 0.0) {
        let assign310_e714: f64 = (locals.var_dxa).exp();
        let assign310_e715: f64 = (1.0 + assign310_e714);
        let assign310_e716: f64 = (assign310_e715).ln();
        let assign310_e717: f64 = (0.1 * assign310_e716);
        let assign310_e718: f64 = (0.05 + assign310_e717);
        (assign310_e718, (0.1 * ((assign310_e714 * locals.var_dxa_dn0) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn1) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn3) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn4) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn5) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn6) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn7) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn8) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn9) / assign310_e715)),)
    } else {
        (locals.var_vgzcbok, locals.var_vgzcbok_dn0, locals.var_vgzcbok_dn1, locals.var_vgzcbok_dn3, locals.var_vgzcbok_dn4, locals.var_vgzcbok_dn5, locals.var_vgzcbok_dn6, locals.var_vgzcbok_dn7, locals.var_vgzcbok_dn8, locals.var_vgzcbok_dn9,)
    }
};
        locals.var_vgzcbok = assign310_e720;
        locals.var_vgzcbok_dn0 = assign310_e720_d_n0;
        locals.var_vgzcbok_dn1 = assign310_e720_d_n1;
        locals.var_vgzcbok_dn3 = assign310_e720_d_n3;
        locals.var_vgzcbok_dn4 = assign310_e720_d_n4;
        locals.var_vgzcbok_dn5 = assign310_e720_d_n5;
        locals.var_vgzcbok_dn6 = assign310_e720_d_n6;
        locals.var_vgzcbok_dn7 = assign310_e720_d_n7;
        locals.var_vgzcbok_dn8 = assign310_e720_d_n8;
        locals.var_vgzcbok_dn9 = assign310_e720_d_n9;

        let (assign320_e744, assign320_e744_d_n0, assign320_e744_d_n1, assign320_e744_d_n3, assign320_e744_d_n4, assign320_e744_d_n5, assign320_e744_d_n6, assign320_e744_d_n7, assign320_e744_d_n8, assign320_e744_d_n9,) = {
    if (locals.var_guard4 == 0.0) {
        let assign320_e726: f64 = (p.p117 * locals.var_trk);
        let assign320_e728: f64 = (assign320_e726 * locals.var_trk);
        let assign320_e731: f64 = (locals.var_trk + p.p118);
        let assign320_e732: f64 = (assign320_e728 / assign320_e731);
        let assign320_e733: f64 = (p.p116 + assign320_e732);
        let assign320_e737: f64 = (-locals.var_dxa);
        let assign320_e738: f64 = (assign320_e737).exp();
        let assign320_e739: f64 = (1.0 + assign320_e738);
        let assign320_e740: f64 = (assign320_e739).ln();
        let assign320_e741: f64 = (0.1 * assign320_e740);
        let assign320_e742: f64 = (assign320_e733 + assign320_e741);
        (assign320_e742, (0.1 * ((assign320_e738 * (-locals.var_dxa_dn0)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn1)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn3)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn4)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn5)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn6)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn7)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn8)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn9)) / assign320_e739)),)
    } else {
        (locals.var_vgzcbok, locals.var_vgzcbok_dn0, locals.var_vgzcbok_dn1, locals.var_vgzcbok_dn3, locals.var_vgzcbok_dn4, locals.var_vgzcbok_dn5, locals.var_vgzcbok_dn6, locals.var_vgzcbok_dn7, locals.var_vgzcbok_dn8, locals.var_vgzcbok_dn9,)
    }
};
        locals.var_vgzcbok = assign320_e744;
        locals.var_vgzcbok_dn0 = assign320_e744_d_n0;
        locals.var_vgzcbok_dn1 = assign320_e744_d_n1;
        locals.var_vgzcbok_dn3 = assign320_e744_d_n3;
        locals.var_vgzcbok_dn4 = assign320_e744_d_n4;
        locals.var_vgzcbok_dn5 = assign320_e744_d_n5;
        locals.var_vgzcbok_dn6 = assign320_e744_d_n6;
        locals.var_vgzcbok_dn7 = assign320_e744_d_n7;
        locals.var_vgzcbok_dn8 = assign320_e744_d_n8;
        locals.var_vgzcbok_dn9 = assign320_e744_d_n9;

        locals.var_vgzcb_tr = p.p116;

        let assign340_e748: f64 = (1.0 / locals.var_vgzcb_tr);
        locals.var_inv_vgzcb_tr = assign340_e748;

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

        locals.var_ibi_t = 0.0;

        locals.var_vdt = 0.0;

        let assign510_e773: f64 = (locals.var_tamb + locals.var_vdt);
        locals.var_tk = assign510_e773;

        let assign520_e776: f64 = (locals.var_tk / locals.var_trk);
        locals.var_tn = assign520_e776;

        let assign530_e779: f64 = (8.617086918058125e-5 * locals.var_tk);
        locals.var_vt = assign530_e779;

        let assign540_e782: f64 = (8.617086918058125e-5 * locals.var_trk);
        locals.var_vtr = assign540_e782;

        let assign550_e785: f64 = (1.0 / locals.var_vt);
        locals.var_vtinv = assign550_e785;

        let assign560_e788: f64 = (1.0 / locals.var_vtr);
        locals.var_vtrinv = assign560_e788;

        let assign570_e791: f64 = (locals.var_vtinv - locals.var_vtrinv);
        locals.var_vdtinv = assign570_e791;

        let assign580_e794: f64 = (locals.var_tk - locals.var_trk);
        locals.var_dt = assign580_e794;

        let assign590_e796: f64 = (locals.var_tn).ln();
        locals.var_lntn = assign590_e796;

        let assign600_e800: f64 = (p.p114 * locals.var_tk);
        let assign600_e802: f64 = (assign600_e800 * locals.var_tk);
        let assign600_e805: f64 = (locals.var_tk + p.p115);
        let assign600_e806: f64 = (assign600_e802 / assign600_e805);
        let assign600_e807: f64 = (locals.var_vgzebok - assign600_e806);
        let assign600_e809: f64 = (assign600_e807 - 0.05);
        let assign600_e811: f64 = (assign600_e809 / 0.1);
        locals.var_dxa = assign600_e811;
        locals.var_dxa_dn0 = (locals.var_vgzebok_dn0 / 0.1);
        locals.var_dxa_dn1 = (locals.var_vgzebok_dn1 / 0.1);
        locals.var_dxa_dn3 = (locals.var_vgzebok_dn3 / 0.1);
        locals.var_dxa_dn4 = (locals.var_vgzebok_dn4 / 0.1);
        locals.var_dxa_dn5 = (locals.var_vgzebok_dn5 / 0.1);
        locals.var_dxa_dn6 = (locals.var_vgzebok_dn6 / 0.1);
        locals.var_dxa_dn7 = (locals.var_vgzebok_dn7 / 0.1);
        locals.var_dxa_dn8 = (locals.var_vgzebok_dn8 / 0.1);
        locals.var_dxa_dn9 = (locals.var_vgzebok_dn9 / 0.1);

        let assign610_e815: f64 = (p.p114 * locals.var_tk);
        let assign610_e817: f64 = (assign610_e815 * locals.var_tk);
        let assign610_e820: f64 = (locals.var_tk + p.p115);
        let assign610_e821: f64 = (assign610_e817 / assign610_e820);
        let assign610_e822: f64 = (locals.var_vgzebok - assign610_e821);
        let assign610_e824: f64 = if assign610_e822 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard5 = assign610_e824;

        let (assign620_e836, assign620_e836_d_n0, assign620_e836_d_n1, assign620_e836_d_n3, assign620_e836_d_n4, assign620_e836_d_n5, assign620_e836_d_n6, assign620_e836_d_n7, assign620_e836_d_n8, assign620_e836_d_n9,) = {
    if (locals.var_guard5 != 0.0) {
        let assign620_e830: f64 = (locals.var_dxa).exp();
        let assign620_e831: f64 = (1.0 + assign620_e830);
        let assign620_e832: f64 = (assign620_e831).ln();
        let assign620_e833: f64 = (0.1 * assign620_e832);
        let assign620_e834: f64 = (0.05 + assign620_e833);
        (assign620_e834, (0.1 * ((assign620_e830 * locals.var_dxa_dn0) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn1) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn3) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn4) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn5) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn6) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn7) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn8) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn9) / assign620_e831)),)
    } else {
        (locals.var_vgzeb_t, locals.var_vgzeb_t_dn0, locals.var_vgzeb_t_dn1, locals.var_vgzeb_t_dn3, locals.var_vgzeb_t_dn4, locals.var_vgzeb_t_dn5, locals.var_vgzeb_t_dn6, locals.var_vgzeb_t_dn7, locals.var_vgzeb_t_dn8, locals.var_vgzeb_t_dn9,)
    }
};
        locals.var_vgzeb_t = assign620_e836;
        locals.var_vgzeb_t_dn0 = assign620_e836_d_n0;
        locals.var_vgzeb_t_dn1 = assign620_e836_d_n1;
        locals.var_vgzeb_t_dn3 = assign620_e836_d_n3;
        locals.var_vgzeb_t_dn4 = assign620_e836_d_n4;
        locals.var_vgzeb_t_dn5 = assign620_e836_d_n5;
        locals.var_vgzeb_t_dn6 = assign620_e836_d_n6;
        locals.var_vgzeb_t_dn7 = assign620_e836_d_n7;
        locals.var_vgzeb_t_dn8 = assign620_e836_d_n8;
        locals.var_vgzeb_t_dn9 = assign620_e836_d_n9;

        let (assign630_e860, assign630_e860_d_n0, assign630_e860_d_n1, assign630_e860_d_n3, assign630_e860_d_n4, assign630_e860_d_n5, assign630_e860_d_n6, assign630_e860_d_n7, assign630_e860_d_n8, assign630_e860_d_n9,) = {
    if (locals.var_guard5 == 0.0) {
        let assign630_e842: f64 = (p.p114 * locals.var_tk);
        let assign630_e844: f64 = (assign630_e842 * locals.var_tk);
        let assign630_e847: f64 = (locals.var_tk + p.p115);
        let assign630_e848: f64 = (assign630_e844 / assign630_e847);
        let assign630_e849: f64 = (locals.var_vgzebok - assign630_e848);
        let assign630_e853: f64 = (-locals.var_dxa);
        let assign630_e854: f64 = (assign630_e853).exp();
        let assign630_e855: f64 = (1.0 + assign630_e854);
        let assign630_e856: f64 = (assign630_e855).ln();
        let assign630_e857: f64 = (0.1 * assign630_e856);
        let assign630_e858: f64 = (assign630_e849 + assign630_e857);
        (assign630_e858, (locals.var_vgzebok_dn0 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn0)) / assign630_e855))), (locals.var_vgzebok_dn1 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn1)) / assign630_e855))), (locals.var_vgzebok_dn3 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn3)) / assign630_e855))), (locals.var_vgzebok_dn4 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn4)) / assign630_e855))), (locals.var_vgzebok_dn5 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn5)) / assign630_e855))), (locals.var_vgzebok_dn6 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn6)) / assign630_e855))), (locals.var_vgzebok_dn7 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn7)) / assign630_e855))), (locals.var_vgzebok_dn8 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn8)) / assign630_e855))), (locals.var_vgzebok_dn9 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn9)) / assign630_e855))),)
    } else {
        (locals.var_vgzeb_t, locals.var_vgzeb_t_dn0, locals.var_vgzeb_t_dn1, locals.var_vgzeb_t_dn3, locals.var_vgzeb_t_dn4, locals.var_vgzeb_t_dn5, locals.var_vgzeb_t_dn6, locals.var_vgzeb_t_dn7, locals.var_vgzeb_t_dn8, locals.var_vgzeb_t_dn9,)
    }
};
        locals.var_vgzeb_t = assign630_e860;
        locals.var_vgzeb_t_dn0 = assign630_e860_d_n0;
        locals.var_vgzeb_t_dn1 = assign630_e860_d_n1;
        locals.var_vgzeb_t_dn3 = assign630_e860_d_n3;
        locals.var_vgzeb_t_dn4 = assign630_e860_d_n4;
        locals.var_vgzeb_t_dn5 = assign630_e860_d_n5;
        locals.var_vgzeb_t_dn6 = assign630_e860_d_n6;
        locals.var_vgzeb_t_dn7 = assign630_e860_d_n7;
        locals.var_vgzeb_t_dn8 = assign630_e860_d_n8;
        locals.var_vgzeb_t_dn9 = assign630_e860_d_n9;

        let assign640_e864: f64 = (p.p117 * locals.var_tk);
        let assign640_e866: f64 = (assign640_e864 * locals.var_tk);
        let assign640_e869: f64 = (locals.var_tk + p.p118);
        let assign640_e870: f64 = (assign640_e866 / assign640_e869);
        let assign640_e871: f64 = (locals.var_vgzcbok - assign640_e870);
        let assign640_e873: f64 = (assign640_e871 - 0.05);
        let assign640_e875: f64 = (assign640_e873 / 0.1);
        locals.var_dxa = assign640_e875;
        locals.var_dxa_dn0 = (locals.var_vgzcbok_dn0 / 0.1);
        locals.var_dxa_dn1 = (locals.var_vgzcbok_dn1 / 0.1);
        locals.var_dxa_dn3 = (locals.var_vgzcbok_dn3 / 0.1);
        locals.var_dxa_dn4 = (locals.var_vgzcbok_dn4 / 0.1);
        locals.var_dxa_dn5 = (locals.var_vgzcbok_dn5 / 0.1);
        locals.var_dxa_dn6 = (locals.var_vgzcbok_dn6 / 0.1);
        locals.var_dxa_dn7 = (locals.var_vgzcbok_dn7 / 0.1);
        locals.var_dxa_dn8 = (locals.var_vgzcbok_dn8 / 0.1);
        locals.var_dxa_dn9 = (locals.var_vgzcbok_dn9 / 0.1);

        let assign650_e879: f64 = (p.p117 * locals.var_tk);
        let assign650_e881: f64 = (assign650_e879 * locals.var_tk);
        let assign650_e884: f64 = (locals.var_tk + p.p118);
        let assign650_e885: f64 = (assign650_e881 / assign650_e884);
        let assign650_e886: f64 = (locals.var_vgzcbok - assign650_e885);
        let assign650_e888: f64 = if assign650_e886 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign650_e888;

        let (assign660_e900,) = {
    if (locals.var_guard6 != 0.0) {
        let assign660_e894: f64 = (locals.var_dxa).exp();
        let assign660_e895: f64 = (1.0 + assign660_e894);
        let assign660_e896: f64 = (assign660_e895).ln();
        let assign660_e897: f64 = (0.1 * assign660_e896);
        let assign660_e898: f64 = (0.05 + assign660_e897);
        (assign660_e898,)
    } else {
        (locals.var_vgzcb_t,)
    }
};
        locals.var_vgzcb_t = assign660_e900;

        let (assign670_e924,) = {
    if (locals.var_guard6 == 0.0) {
        let assign670_e906: f64 = (p.p117 * locals.var_tk);
        let assign670_e908: f64 = (assign670_e906 * locals.var_tk);
        let assign670_e911: f64 = (locals.var_tk + p.p118);
        let assign670_e912: f64 = (assign670_e908 / assign670_e911);
        let assign670_e913: f64 = (locals.var_vgzcbok - assign670_e912);
        let assign670_e917: f64 = (-locals.var_dxa);
        let assign670_e918: f64 = (assign670_e917).exp();
        let assign670_e919: f64 = (1.0 + assign670_e918);
        let assign670_e920: f64 = (assign670_e919).ln();
        let assign670_e921: f64 = (0.1 * assign670_e920);
        let assign670_e922: f64 = (assign670_e913 + assign670_e921);
        (assign670_e922,)
    } else {
        (locals.var_vgzcb_t,)
    }
};
        locals.var_vgzcb_t = assign670_e924;

        let assign680_e926: f64 = (-3.0);
        let assign680_e928: f64 = (assign680_e926 * locals.var_vt);
        let assign680_e930: f64 = (assign680_e928 * locals.var_lntn);
        let assign680_e933: f64 = (p.p65 * locals.var_tn);
        let assign680_e934: f64 = (assign680_e930 + assign680_e933);
        let assign680_e937: f64 = (1.0 - locals.var_tn);
        let assign680_e939: f64 = (assign680_e937 * p.p104);
        let assign680_e940: f64 = (assign680_e934 + assign680_e939);
        locals.var_udet = assign680_e940;

        let assign690_e943: f64 = (0.05 - locals.var_udet);
        let assign690_e945: f64 = (assign690_e943 / locals.var_vt);
        locals.var_dxa = assign690_e945;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;

        let assign700_e948: f64 = if 0.05 < locals.var_udet { 1.0 } else { 0.0 };
        locals.var_guard7 = assign700_e948;

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign710_e960, assign710_e960_d_n0, assign710_e960_d_n1, assign710_e960_d_n3, assign710_e960_d_n4, assign710_e960_d_n5, assign710_e960_d_n6, assign710_e960_d_n7, assign710_e960_d_n8, assign710_e960_d_n9,) = {
    if (locals.var_guard7 != 0.0) {
        let assign710_e954: f64 = (locals.var_dxa).exp();
        let assign710_e955: f64 = (1.0 + assign710_e954);
        let assign710_e956: f64 = (assign710_e955).ln();
        let assign710_e957: f64 = (locals.var_vt * assign710_e956);
        let assign710_e958: f64 = (locals.var_udet + assign710_e957);
        (assign710_e958, (locals.var_vt * ((assign710_e954 * locals.var_dxa_dn0) / assign710_e955)), (locals.var_vt * ((assign710_e954 * locals.var_dxa_dn1) / assign710_e955)), (locals.var_vt * ((assign710_e954 * locals.var_dxa_dn3) / assign710_e955)), (locals.var_vt * ((assign710_e954 * locals.var_dxa_dn4) / assign710_e955)), (locals.var_vt * ((assign710_e954 * locals.var_dxa_dn5) / assign710_e955)), (locals.var_vt * ((assign710_e954 * locals.var_dxa_dn6) / assign710_e955)), (locals.var_vt * ((assign710_e954 * locals.var_dxa_dn7) / assign710_e955)), (locals.var_vt * ((assign710_e954 * locals.var_dxa_dn8) / assign710_e955)), (locals.var_vt * ((assign710_e954 * locals.var_dxa_dn9) / assign710_e955)),)
    } else {
        (locals.var_vde_t, locals.var_vde_t_dn0, locals.var_vde_t_dn1, locals.var_vde_t_dn3, locals.var_vde_t_dn4, locals.var_vde_t_dn5, locals.var_vde_t_dn6, locals.var_vde_t_dn7, locals.var_vde_t_dn8, locals.var_vde_t_dn9,)
    }
};
        locals.var_vde_t = assign710_e960;
        locals.var_vde_t_dn0 = assign710_e960_d_n0;
        locals.var_vde_t_dn1 = assign710_e960_d_n1;
        locals.var_vde_t_dn3 = assign710_e960_d_n3;
        locals.var_vde_t_dn4 = assign710_e960_d_n4;
        locals.var_vde_t_dn5 = assign710_e960_d_n5;
        locals.var_vde_t_dn6 = assign710_e960_d_n6;
        locals.var_vde_t_dn7 = assign710_e960_d_n7;
        locals.var_vde_t_dn8 = assign710_e960_d_n8;
        locals.var_vde_t_dn9 = assign710_e960_d_n9;

        let (assign720_e974, assign720_e974_d_n0, assign720_e974_d_n1, assign720_e974_d_n3, assign720_e974_d_n4, assign720_e974_d_n5, assign720_e974_d_n6, assign720_e974_d_n7, assign720_e974_d_n8, assign720_e974_d_n9,) = {
    if (locals.var_guard7 == 0.0) {
        let assign720_e967: f64 = (-locals.var_dxa);
        let assign720_e968: f64 = (assign720_e967).exp();
        let assign720_e969: f64 = (1.0 + assign720_e968);
        let assign720_e970: f64 = (assign720_e969).ln();
        let assign720_e971: f64 = (locals.var_vt * assign720_e970);
        let assign720_e972: f64 = (0.05 + assign720_e971);
        (assign720_e972, (locals.var_vt * ((assign720_e968 * (-locals.var_dxa_dn0)) / assign720_e969)), (locals.var_vt * ((assign720_e968 * (-locals.var_dxa_dn1)) / assign720_e969)), (locals.var_vt * ((assign720_e968 * (-locals.var_dxa_dn3)) / assign720_e969)), (locals.var_vt * ((assign720_e968 * (-locals.var_dxa_dn4)) / assign720_e969)), (locals.var_vt * ((assign720_e968 * (-locals.var_dxa_dn5)) / assign720_e969)), (locals.var_vt * ((assign720_e968 * (-locals.var_dxa_dn6)) / assign720_e969)), (locals.var_vt * ((assign720_e968 * (-locals.var_dxa_dn7)) / assign720_e969)), (locals.var_vt * ((assign720_e968 * (-locals.var_dxa_dn8)) / assign720_e969)), (locals.var_vt * ((assign720_e968 * (-locals.var_dxa_dn9)) / assign720_e969)),)
    } else {
        (locals.var_vde_t, locals.var_vde_t_dn0, locals.var_vde_t_dn1, locals.var_vde_t_dn3, locals.var_vde_t_dn4, locals.var_vde_t_dn5, locals.var_vde_t_dn6, locals.var_vde_t_dn7, locals.var_vde_t_dn8, locals.var_vde_t_dn9,)
    }
};
        locals.var_vde_t = assign720_e974;
        locals.var_vde_t_dn0 = assign720_e974_d_n0;
        locals.var_vde_t_dn1 = assign720_e974_d_n1;
        locals.var_vde_t_dn3 = assign720_e974_d_n3;
        locals.var_vde_t_dn4 = assign720_e974_d_n4;
        locals.var_vde_t_dn5 = assign720_e974_d_n5;
        locals.var_vde_t_dn6 = assign720_e974_d_n6;
        locals.var_vde_t_dn7 = assign720_e974_d_n7;
        locals.var_vde_t_dn8 = assign720_e974_d_n8;
        locals.var_vde_t_dn9 = assign720_e974_d_n9;

        let assign730_e976: f64 = (-3.0);
        let assign730_e978: f64 = (assign730_e976 * locals.var_vt);
        let assign730_e980: f64 = (assign730_e978 * locals.var_lntn);
        let assign730_e983: f64 = (p.p63 * locals.var_tn);
        let assign730_e984: f64 = (assign730_e980 + assign730_e983);
        let assign730_e987: f64 = (1.0 - locals.var_tn);
        let assign730_e989: f64 = (assign730_e987 * p.p109);
        let assign730_e990: f64 = (assign730_e984 + assign730_e989);
        locals.var_udct = assign730_e990;

        let assign740_e993: f64 = (0.05 - locals.var_udct);
        let assign740_e995: f64 = (assign740_e993 / locals.var_vt);
        locals.var_dxa = assign740_e995;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;

        let assign750_e998: f64 = if 0.05 < locals.var_udct { 1.0 } else { 0.0 };
        locals.var_guard8 = assign750_e998;

        let (assign760_e1010, assign760_e1010_d_n0, assign760_e1010_d_n1, assign760_e1010_d_n3, assign760_e1010_d_n4, assign760_e1010_d_n5, assign760_e1010_d_n6, assign760_e1010_d_n7, assign760_e1010_d_n8, assign760_e1010_d_n9,) = {
    if (locals.var_guard8 != 0.0) {
        let assign760_e1004: f64 = (locals.var_dxa).exp();
        let assign760_e1005: f64 = (1.0 + assign760_e1004);
        let assign760_e1006: f64 = (assign760_e1005).ln();
        let assign760_e1007: f64 = (locals.var_vt * assign760_e1006);
        let assign760_e1008: f64 = (locals.var_udct + assign760_e1007);
        (assign760_e1008, (locals.var_vt * ((assign760_e1004 * locals.var_dxa_dn0) / assign760_e1005)), (locals.var_vt * ((assign760_e1004 * locals.var_dxa_dn1) / assign760_e1005)), (locals.var_vt * ((assign760_e1004 * locals.var_dxa_dn3) / assign760_e1005)), (locals.var_vt * ((assign760_e1004 * locals.var_dxa_dn4) / assign760_e1005)), (locals.var_vt * ((assign760_e1004 * locals.var_dxa_dn5) / assign760_e1005)), (locals.var_vt * ((assign760_e1004 * locals.var_dxa_dn6) / assign760_e1005)), (locals.var_vt * ((assign760_e1004 * locals.var_dxa_dn7) / assign760_e1005)), (locals.var_vt * ((assign760_e1004 * locals.var_dxa_dn8) / assign760_e1005)), (locals.var_vt * ((assign760_e1004 * locals.var_dxa_dn9) / assign760_e1005)),)
    } else {
        (locals.var_vdc_t, locals.var_vdc_t_dn0, locals.var_vdc_t_dn1, locals.var_vdc_t_dn3, locals.var_vdc_t_dn4, locals.var_vdc_t_dn5, locals.var_vdc_t_dn6, locals.var_vdc_t_dn7, locals.var_vdc_t_dn8, locals.var_vdc_t_dn9,)
    }
};
        locals.var_vdc_t = assign760_e1010;
        locals.var_vdc_t_dn0 = assign760_e1010_d_n0;
        locals.var_vdc_t_dn1 = assign760_e1010_d_n1;
        locals.var_vdc_t_dn3 = assign760_e1010_d_n3;
        locals.var_vdc_t_dn4 = assign760_e1010_d_n4;
        locals.var_vdc_t_dn5 = assign760_e1010_d_n5;
        locals.var_vdc_t_dn6 = assign760_e1010_d_n6;
        locals.var_vdc_t_dn7 = assign760_e1010_d_n7;
        locals.var_vdc_t_dn8 = assign760_e1010_d_n8;
        locals.var_vdc_t_dn9 = assign760_e1010_d_n9;

        let (assign770_e1024, assign770_e1024_d_n0, assign770_e1024_d_n1, assign770_e1024_d_n3, assign770_e1024_d_n4, assign770_e1024_d_n5, assign770_e1024_d_n6, assign770_e1024_d_n7, assign770_e1024_d_n8, assign770_e1024_d_n9,) = {
    if (locals.var_guard8 == 0.0) {
        let assign770_e1017: f64 = (-locals.var_dxa);
        let assign770_e1018: f64 = (assign770_e1017).exp();
        let assign770_e1019: f64 = (1.0 + assign770_e1018);
        let assign770_e1020: f64 = (assign770_e1019).ln();
        let assign770_e1021: f64 = (locals.var_vt * assign770_e1020);
        let assign770_e1022: f64 = (0.05 + assign770_e1021);
        (assign770_e1022, (locals.var_vt * ((assign770_e1018 * (-locals.var_dxa_dn0)) / assign770_e1019)), (locals.var_vt * ((assign770_e1018 * (-locals.var_dxa_dn1)) / assign770_e1019)), (locals.var_vt * ((assign770_e1018 * (-locals.var_dxa_dn3)) / assign770_e1019)), (locals.var_vt * ((assign770_e1018 * (-locals.var_dxa_dn4)) / assign770_e1019)), (locals.var_vt * ((assign770_e1018 * (-locals.var_dxa_dn5)) / assign770_e1019)), (locals.var_vt * ((assign770_e1018 * (-locals.var_dxa_dn6)) / assign770_e1019)), (locals.var_vt * ((assign770_e1018 * (-locals.var_dxa_dn7)) / assign770_e1019)), (locals.var_vt * ((assign770_e1018 * (-locals.var_dxa_dn8)) / assign770_e1019)), (locals.var_vt * ((assign770_e1018 * (-locals.var_dxa_dn9)) / assign770_e1019)),)
    } else {
        (locals.var_vdc_t, locals.var_vdc_t_dn0, locals.var_vdc_t_dn1, locals.var_vdc_t_dn3, locals.var_vdc_t_dn4, locals.var_vdc_t_dn5, locals.var_vdc_t_dn6, locals.var_vdc_t_dn7, locals.var_vdc_t_dn8, locals.var_vdc_t_dn9,)
    }
};
        locals.var_vdc_t = assign770_e1024;
        locals.var_vdc_t_dn0 = assign770_e1024_d_n0;
        locals.var_vdc_t_dn1 = assign770_e1024_d_n1;
        locals.var_vdc_t_dn3 = assign770_e1024_d_n3;
        locals.var_vdc_t_dn4 = assign770_e1024_d_n4;
        locals.var_vdc_t_dn5 = assign770_e1024_d_n5;
        locals.var_vdc_t_dn6 = assign770_e1024_d_n6;
        locals.var_vdc_t_dn7 = assign770_e1024_d_n7;
        locals.var_vdc_t_dn8 = assign770_e1024_d_n8;
        locals.var_vdc_t_dn9 = assign770_e1024_d_n9;

        let assign780_e1026: f64 = (-3.0);
        let assign780_e1028: f64 = (assign780_e1026 * locals.var_vt);
        let assign780_e1030: f64 = (assign780_e1028 * locals.var_lntn);
        let assign780_e1033: f64 = (p.p79 * locals.var_tn);
        let assign780_e1034: f64 = (assign780_e1030 + assign780_e1033);
        let assign780_e1037: f64 = (1.0 - locals.var_tn);
        let assign780_e1039: f64 = (assign780_e1037 * p.p109);
        let assign780_e1040: f64 = (assign780_e1034 + assign780_e1039);
        locals.var_udcext = assign780_e1040;

        let assign790_e1043: f64 = (0.05 - locals.var_udcext);
        let assign790_e1045: f64 = (assign790_e1043 / locals.var_vt);
        locals.var_dxa = assign790_e1045;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;

        let assign800_e1048: f64 = if 0.05 < locals.var_udcext { 1.0 } else { 0.0 };
        locals.var_guard9 = assign800_e1048;

        let (assign810_e1060, assign810_e1060_d_n0, assign810_e1060_d_n1, assign810_e1060_d_n3, assign810_e1060_d_n4, assign810_e1060_d_n5, assign810_e1060_d_n6, assign810_e1060_d_n7, assign810_e1060_d_n8, assign810_e1060_d_n9,) = {
    if (locals.var_guard9 != 0.0) {
        let assign810_e1054: f64 = (locals.var_dxa).exp();
        let assign810_e1055: f64 = (1.0 + assign810_e1054);
        let assign810_e1056: f64 = (assign810_e1055).ln();
        let assign810_e1057: f64 = (locals.var_vt * assign810_e1056);
        let assign810_e1058: f64 = (locals.var_udcext + assign810_e1057);
        (assign810_e1058, (locals.var_vt * ((assign810_e1054 * locals.var_dxa_dn0) / assign810_e1055)), (locals.var_vt * ((assign810_e1054 * locals.var_dxa_dn1) / assign810_e1055)), (locals.var_vt * ((assign810_e1054 * locals.var_dxa_dn3) / assign810_e1055)), (locals.var_vt * ((assign810_e1054 * locals.var_dxa_dn4) / assign810_e1055)), (locals.var_vt * ((assign810_e1054 * locals.var_dxa_dn5) / assign810_e1055)), (locals.var_vt * ((assign810_e1054 * locals.var_dxa_dn6) / assign810_e1055)), (locals.var_vt * ((assign810_e1054 * locals.var_dxa_dn7) / assign810_e1055)), (locals.var_vt * ((assign810_e1054 * locals.var_dxa_dn8) / assign810_e1055)), (locals.var_vt * ((assign810_e1054 * locals.var_dxa_dn9) / assign810_e1055)),)
    } else {
        (locals.var_vdcex_t, locals.var_vdcex_t_dn0, locals.var_vdcex_t_dn1, locals.var_vdcex_t_dn3, locals.var_vdcex_t_dn4, locals.var_vdcex_t_dn5, locals.var_vdcex_t_dn6, locals.var_vdcex_t_dn7, locals.var_vdcex_t_dn8, locals.var_vdcex_t_dn9,)
    }
};
        locals.var_vdcex_t = assign810_e1060;
        locals.var_vdcex_t_dn0 = assign810_e1060_d_n0;
        locals.var_vdcex_t_dn1 = assign810_e1060_d_n1;
        locals.var_vdcex_t_dn3 = assign810_e1060_d_n3;
        locals.var_vdcex_t_dn4 = assign810_e1060_d_n4;
        locals.var_vdcex_t_dn5 = assign810_e1060_d_n5;
        locals.var_vdcex_t_dn6 = assign810_e1060_d_n6;
        locals.var_vdcex_t_dn7 = assign810_e1060_d_n7;
        locals.var_vdcex_t_dn8 = assign810_e1060_d_n8;
        locals.var_vdcex_t_dn9 = assign810_e1060_d_n9;

        let (assign820_e1074, assign820_e1074_d_n0, assign820_e1074_d_n1, assign820_e1074_d_n3, assign820_e1074_d_n4, assign820_e1074_d_n5, assign820_e1074_d_n6, assign820_e1074_d_n7, assign820_e1074_d_n8, assign820_e1074_d_n9,) = {
    if (locals.var_guard9 == 0.0) {
        let assign820_e1067: f64 = (-locals.var_dxa);
        let assign820_e1068: f64 = (assign820_e1067).exp();
        let assign820_e1069: f64 = (1.0 + assign820_e1068);
        let assign820_e1070: f64 = (assign820_e1069).ln();
        let assign820_e1071: f64 = (locals.var_vt * assign820_e1070);
        let assign820_e1072: f64 = (0.05 + assign820_e1071);
        (assign820_e1072, (locals.var_vt * ((assign820_e1068 * (-locals.var_dxa_dn0)) / assign820_e1069)), (locals.var_vt * ((assign820_e1068 * (-locals.var_dxa_dn1)) / assign820_e1069)), (locals.var_vt * ((assign820_e1068 * (-locals.var_dxa_dn3)) / assign820_e1069)), (locals.var_vt * ((assign820_e1068 * (-locals.var_dxa_dn4)) / assign820_e1069)), (locals.var_vt * ((assign820_e1068 * (-locals.var_dxa_dn5)) / assign820_e1069)), (locals.var_vt * ((assign820_e1068 * (-locals.var_dxa_dn6)) / assign820_e1069)), (locals.var_vt * ((assign820_e1068 * (-locals.var_dxa_dn7)) / assign820_e1069)), (locals.var_vt * ((assign820_e1068 * (-locals.var_dxa_dn8)) / assign820_e1069)), (locals.var_vt * ((assign820_e1068 * (-locals.var_dxa_dn9)) / assign820_e1069)),)
    } else {
        (locals.var_vdcex_t, locals.var_vdcex_t_dn0, locals.var_vdcex_t_dn1, locals.var_vdcex_t_dn3, locals.var_vdcex_t_dn4, locals.var_vdcex_t_dn5, locals.var_vdcex_t_dn6, locals.var_vdcex_t_dn7, locals.var_vdcex_t_dn8, locals.var_vdcex_t_dn9,)
    }
};
        locals.var_vdcex_t = assign820_e1074;
        locals.var_vdcex_t_dn0 = assign820_e1074_d_n0;
        locals.var_vdcex_t_dn1 = assign820_e1074_d_n1;
        locals.var_vdcex_t_dn3 = assign820_e1074_d_n3;
        locals.var_vdcex_t_dn4 = assign820_e1074_d_n4;
        locals.var_vdcex_t_dn5 = assign820_e1074_d_n5;
        locals.var_vdcex_t_dn6 = assign820_e1074_d_n6;
        locals.var_vdcex_t_dn7 = assign820_e1074_d_n7;
        locals.var_vdcex_t_dn8 = assign820_e1074_d_n8;
        locals.var_vdcex_t_dn9 = assign820_e1074_d_n9;

        let assign830_e1076: f64 = (-3.0);
        let assign830_e1078: f64 = (assign830_e1076 * locals.var_vt);
        let assign830_e1080: f64 = (assign830_e1078 * locals.var_lntn);
        let assign830_e1083: f64 = (p.p70 * locals.var_tn);
        let assign830_e1084: f64 = (assign830_e1080 + assign830_e1083);
        let assign830_e1087: f64 = (1.0 - locals.var_tn);
        let assign830_e1089: f64 = (assign830_e1087 * p.p109);
        let assign830_e1090: f64 = (assign830_e1084 + assign830_e1089);
        locals.var_udct_ctc = assign830_e1090;

        let assign840_e1093: f64 = (0.05 - locals.var_udct_ctc);
        let assign840_e1095: f64 = (assign840_e1093 / locals.var_vt);
        locals.var_dxa = assign840_e1095;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;

        let assign850_e1098: f64 = if 0.05 < locals.var_udct_ctc { 1.0 } else { 0.0 };
        locals.var_guard10 = assign850_e1098;

        let (assign860_e1110, assign860_e1110_d_n0, assign860_e1110_d_n1, assign860_e1110_d_n3, assign860_e1110_d_n4, assign860_e1110_d_n5, assign860_e1110_d_n6, assign860_e1110_d_n7, assign860_e1110_d_n8, assign860_e1110_d_n9,) = {
    if (locals.var_guard10 != 0.0) {
        let assign860_e1104: f64 = (locals.var_dxa).exp();
        let assign860_e1105: f64 = (1.0 + assign860_e1104);
        let assign860_e1106: f64 = (assign860_e1105).ln();
        let assign860_e1107: f64 = (locals.var_vt * assign860_e1106);
        let assign860_e1108: f64 = (locals.var_udct_ctc + assign860_e1107);
        (assign860_e1108, (locals.var_vt * ((assign860_e1104 * locals.var_dxa_dn0) / assign860_e1105)), (locals.var_vt * ((assign860_e1104 * locals.var_dxa_dn1) / assign860_e1105)), (locals.var_vt * ((assign860_e1104 * locals.var_dxa_dn3) / assign860_e1105)), (locals.var_vt * ((assign860_e1104 * locals.var_dxa_dn4) / assign860_e1105)), (locals.var_vt * ((assign860_e1104 * locals.var_dxa_dn5) / assign860_e1105)), (locals.var_vt * ((assign860_e1104 * locals.var_dxa_dn6) / assign860_e1105)), (locals.var_vt * ((assign860_e1104 * locals.var_dxa_dn7) / assign860_e1105)), (locals.var_vt * ((assign860_e1104 * locals.var_dxa_dn8) / assign860_e1105)), (locals.var_vt * ((assign860_e1104 * locals.var_dxa_dn9) / assign860_e1105)),)
    } else {
        (locals.var_vdc_ctc_t, locals.var_vdc_ctc_t_dn0, locals.var_vdc_ctc_t_dn1, locals.var_vdc_ctc_t_dn3, locals.var_vdc_ctc_t_dn4, locals.var_vdc_ctc_t_dn5, locals.var_vdc_ctc_t_dn6, locals.var_vdc_ctc_t_dn7, locals.var_vdc_ctc_t_dn8, locals.var_vdc_ctc_t_dn9,)
    }
};
        locals.var_vdc_ctc_t = assign860_e1110;
        locals.var_vdc_ctc_t_dn0 = assign860_e1110_d_n0;
        locals.var_vdc_ctc_t_dn1 = assign860_e1110_d_n1;
        locals.var_vdc_ctc_t_dn3 = assign860_e1110_d_n3;
        locals.var_vdc_ctc_t_dn4 = assign860_e1110_d_n4;
        locals.var_vdc_ctc_t_dn5 = assign860_e1110_d_n5;
        locals.var_vdc_ctc_t_dn6 = assign860_e1110_d_n6;
        locals.var_vdc_ctc_t_dn7 = assign860_e1110_d_n7;
        locals.var_vdc_ctc_t_dn8 = assign860_e1110_d_n8;
        locals.var_vdc_ctc_t_dn9 = assign860_e1110_d_n9;

        let (assign870_e1124, assign870_e1124_d_n0, assign870_e1124_d_n1, assign870_e1124_d_n3, assign870_e1124_d_n4, assign870_e1124_d_n5, assign870_e1124_d_n6, assign870_e1124_d_n7, assign870_e1124_d_n8, assign870_e1124_d_n9,) = {
    if (locals.var_guard10 == 0.0) {
        let assign870_e1117: f64 = (-locals.var_dxa);
        let assign870_e1118: f64 = (assign870_e1117).exp();
        let assign870_e1119: f64 = (1.0 + assign870_e1118);
        let assign870_e1120: f64 = (assign870_e1119).ln();
        let assign870_e1121: f64 = (locals.var_vt * assign870_e1120);
        let assign870_e1122: f64 = (0.05 + assign870_e1121);
        (assign870_e1122, (locals.var_vt * ((assign870_e1118 * (-locals.var_dxa_dn0)) / assign870_e1119)), (locals.var_vt * ((assign870_e1118 * (-locals.var_dxa_dn1)) / assign870_e1119)), (locals.var_vt * ((assign870_e1118 * (-locals.var_dxa_dn3)) / assign870_e1119)), (locals.var_vt * ((assign870_e1118 * (-locals.var_dxa_dn4)) / assign870_e1119)), (locals.var_vt * ((assign870_e1118 * (-locals.var_dxa_dn5)) / assign870_e1119)), (locals.var_vt * ((assign870_e1118 * (-locals.var_dxa_dn6)) / assign870_e1119)), (locals.var_vt * ((assign870_e1118 * (-locals.var_dxa_dn7)) / assign870_e1119)), (locals.var_vt * ((assign870_e1118 * (-locals.var_dxa_dn8)) / assign870_e1119)), (locals.var_vt * ((assign870_e1118 * (-locals.var_dxa_dn9)) / assign870_e1119)),)
    } else {
        (locals.var_vdc_ctc_t, locals.var_vdc_ctc_t_dn0, locals.var_vdc_ctc_t_dn1, locals.var_vdc_ctc_t_dn3, locals.var_vdc_ctc_t_dn4, locals.var_vdc_ctc_t_dn5, locals.var_vdc_ctc_t_dn6, locals.var_vdc_ctc_t_dn7, locals.var_vdc_ctc_t_dn8, locals.var_vdc_ctc_t_dn9,)
    }
};
        locals.var_vdc_ctc_t = assign870_e1124;
        locals.var_vdc_ctc_t_dn0 = assign870_e1124_d_n0;
        locals.var_vdc_ctc_t_dn1 = assign870_e1124_d_n1;
        locals.var_vdc_ctc_t_dn3 = assign870_e1124_d_n3;
        locals.var_vdc_ctc_t_dn4 = assign870_e1124_d_n4;
        locals.var_vdc_ctc_t_dn5 = assign870_e1124_d_n5;
        locals.var_vdc_ctc_t_dn6 = assign870_e1124_d_n6;
        locals.var_vdc_ctc_t_dn7 = assign870_e1124_d_n7;
        locals.var_vdc_ctc_t_dn8 = assign870_e1124_d_n8;
        locals.var_vdc_ctc_t_dn9 = assign870_e1124_d_n9;

        let assign880_e1126: f64 = (-3.0);
        let assign880_e1128: f64 = (assign880_e1126 * locals.var_vt);
        let assign880_e1130: f64 = (assign880_e1128 * locals.var_lntn);
        let assign880_e1133: f64 = (locals.var_vdc_zener * locals.var_tn);
        let assign880_e1134: f64 = (assign880_e1130 + assign880_e1133);
        let assign880_e1137: f64 = (1.0 - locals.var_tn);
        let assign880_e1139: f64 = (assign880_e1137 * p.p109);
        let assign880_e1140: f64 = (assign880_e1134 + assign880_e1139);
        locals.var_udct_zener = assign880_e1140;

        let assign890_e1143: f64 = (0.05 - locals.var_udct_zener);
        let assign890_e1145: f64 = (assign890_e1143 / locals.var_vt);
        locals.var_dxa = assign890_e1145;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;

        let assign900_e1148: f64 = if 0.05 < locals.var_udct_zener { 1.0 } else { 0.0 };
        locals.var_guard11 = assign900_e1148;

        let (assign910_e1160,) = {
    if (locals.var_guard11 != 0.0) {
        let assign910_e1154: f64 = (locals.var_dxa).exp();
        let assign910_e1155: f64 = (1.0 + assign910_e1154);
        let assign910_e1156: f64 = (assign910_e1155).ln();
        let assign910_e1157: f64 = (locals.var_vt * assign910_e1156);
        let assign910_e1158: f64 = (locals.var_udct_zener + assign910_e1157);
        (assign910_e1158,)
    } else {
        (locals.var_vdc_zener_t,)
    }
};
        locals.var_vdc_zener_t = assign910_e1160;

        let (assign920_e1174,) = {
    if (locals.var_guard11 == 0.0) {
        let assign920_e1167: f64 = (-locals.var_dxa);
        let assign920_e1168: f64 = (assign920_e1167).exp();
        let assign920_e1169: f64 = (1.0 + assign920_e1168);
        let assign920_e1170: f64 = (assign920_e1169).ln();
        let assign920_e1171: f64 = (locals.var_vt * assign920_e1170);
        let assign920_e1172: f64 = (0.05 + assign920_e1171);
        (assign920_e1172,)
    } else {
        (locals.var_vdc_zener_t,)
    }
};
        locals.var_vdc_zener_t = assign920_e1174;

        let assign930_e1176: f64 = (-3.0);
        let assign930_e1178: f64 = (assign930_e1176 * locals.var_vt);
        let assign930_e1180: f64 = (assign930_e1178 * locals.var_lntn);
        let assign930_e1183: f64 = (p.p26 * locals.var_tn);
        let assign930_e1184: f64 = (assign930_e1180 + assign930_e1183);
        let assign930_e1187: f64 = (1.0 - locals.var_tn);
        let assign930_e1189: f64 = (assign930_e1187 * p.p108);
        let assign930_e1190: f64 = (assign930_e1184 + assign930_e1189);
        locals.var_uknbrt = assign930_e1190;

        let assign940_e1193: f64 = (0.05 - locals.var_uknbrt);
        let assign940_e1195: f64 = (assign940_e1193 / locals.var_vt);
        locals.var_dxa = assign940_e1195;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;

        let assign950_e1198: f64 = if 0.05 < locals.var_uknbrt { 1.0 } else { 0.0 };
        locals.var_guard12 = assign950_e1198;

        let (assign960_e1210,) = {
    if (locals.var_guard12 != 0.0) {
        let assign960_e1204: f64 = (locals.var_dxa).exp();
        let assign960_e1205: f64 = (1.0 + assign960_e1204);
        let assign960_e1206: f64 = (assign960_e1205).ln();
        let assign960_e1207: f64 = (locals.var_vt * assign960_e1206);
        let assign960_e1208: f64 = (locals.var_uknbrt + assign960_e1207);
        (assign960_e1208,)
    } else {
        (locals.var_vknbr_t,)
    }
};
        locals.var_vknbr_t = assign960_e1210;

        let (assign970_e1224,) = {
    if (locals.var_guard12 == 0.0) {
        let assign970_e1217: f64 = (-locals.var_dxa);
        let assign970_e1218: f64 = (assign970_e1217).exp();
        let assign970_e1219: f64 = (1.0 + assign970_e1218);
        let assign970_e1220: f64 = (assign970_e1219).ln();
        let assign970_e1221: f64 = (locals.var_vt * assign970_e1220);
        let assign970_e1222: f64 = (0.05 + assign970_e1221);
        (assign970_e1222,)
    } else {
        (locals.var_vknbr_t,)
    }
};
        locals.var_vknbr_t = assign970_e1224;

        let assign980_e1227: f64 = (1.0 / locals.var_vde_t);
        locals.var_inv_vde_t = assign980_e1227;
        locals.var_inv_vde_t_dn0 = (-(locals.var_vde_t_dn0 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn1 = (-(locals.var_vde_t_dn1 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn3 = (-(locals.var_vde_t_dn3 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn4 = (-(locals.var_vde_t_dn4 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn5 = (-(locals.var_vde_t_dn5 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn6 = (-(locals.var_vde_t_dn6 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn7 = (-(locals.var_vde_t_dn7 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn8 = (-(locals.var_vde_t_dn8 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn9 = (-(locals.var_vde_t_dn9 / (locals.var_vde_t * locals.var_vde_t)));

        let assign990_e1230: f64 = (1.0 / locals.var_vdc_zener_t);
        locals.var_inv_vdc_zener_t = assign990_e1230;

        let assign1000_e1233: f64 = (p.p65 * locals.var_inv_vde_t);
        let assign1000_e1235: f64 = (assign1000_e1233).powf(p.p66);
        locals.var_cje_t_div_cje = assign1000_e1235;
        locals.var_cje_t_div_cje_dn0 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1000_e1233).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn0))) } } else { (assign1000_e1235 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn0) / assign1000_e1233))) };
        locals.var_cje_t_div_cje_dn1 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1000_e1233).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn1))) } } else { (assign1000_e1235 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn1) / assign1000_e1233))) };
        locals.var_cje_t_div_cje_dn3 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1000_e1233).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn3))) } } else { (assign1000_e1235 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn3) / assign1000_e1233))) };
        locals.var_cje_t_div_cje_dn4 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1000_e1233).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn4))) } } else { (assign1000_e1235 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn4) / assign1000_e1233))) };
        locals.var_cje_t_div_cje_dn5 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1000_e1233).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn5))) } } else { (assign1000_e1235 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn5) / assign1000_e1233))) };
        locals.var_cje_t_div_cje_dn6 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1000_e1233).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn6))) } } else { (assign1000_e1235 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn6) / assign1000_e1233))) };
        locals.var_cje_t_div_cje_dn7 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1000_e1233).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn7))) } } else { (assign1000_e1235 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn7) / assign1000_e1233))) };
        locals.var_cje_t_div_cje_dn8 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1000_e1233).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn8))) } } else { (assign1000_e1235 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn8) / assign1000_e1233))) };
        locals.var_cje_t_div_cje_dn9 = if 0.0 == 0.0 && ((p.p66) as f64).is_finite() && ((p.p66) as f64).fract() == 0.0 { if p.p66 == 0.0 { 0.0 } else { (p.p66 * ((assign1000_e1233).powf(p.p66 - 1.0) * (p.p65 * locals.var_inv_vde_t_dn9))) } } else { (assign1000_e1235 * (p.p66 * ((p.p65 * locals.var_inv_vde_t_dn9) / assign1000_e1233))) };

        let assign1010_e1238: f64 = (locals.var_vdc_zener * locals.var_inv_vdc_zener_t);
        let assign1010_e1240: f64 = (assign1010_e1238).powf(locals.var_pc_zener);
        locals.var_cjc_t_div_cjc_zener = assign1010_e1240;

        let assign1020_e1243: f64 = (p.p64 * locals.var_cje_t_div_cje);
        locals.var_cje_t = assign1020_e1243;
        locals.var_cje_t_dn0 = (p.p64 * locals.var_cje_t_div_cje_dn0);
        locals.var_cje_t_dn1 = (p.p64 * locals.var_cje_t_div_cje_dn1);
        locals.var_cje_t_dn3 = (p.p64 * locals.var_cje_t_div_cje_dn3);
        locals.var_cje_t_dn4 = (p.p64 * locals.var_cje_t_div_cje_dn4);
        locals.var_cje_t_dn5 = (p.p64 * locals.var_cje_t_div_cje_dn5);
        locals.var_cje_t_dn6 = (p.p64 * locals.var_cje_t_div_cje_dn6);
        locals.var_cje_t_dn7 = (p.p64 * locals.var_cje_t_div_cje_dn7);
        locals.var_cje_t_dn8 = (p.p64 * locals.var_cje_t_div_cje_dn8);
        locals.var_cje_t_dn9 = (p.p64 * locals.var_cje_t_div_cje_dn9);

        let assign1030_e1246: f64 = (1.0 - p.p74);
        let assign1030_e1249: f64 = (p.p70 / locals.var_vdc_ctc_t);
        let assign1030_e1251: f64 = (assign1030_e1249).powf(p.p71);
        let assign1030_e1252: f64 = (assign1030_e1246 * assign1030_e1251);
        let assign1030_e1254: f64 = (assign1030_e1252 + p.p74);
        locals.var_cjc_scale = assign1030_e1254;
        locals.var_cjc_scale_dn0 = (assign1030_e1246 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1030_e1249).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn0) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1030_e1251 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn0) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1030_e1249))) });
        locals.var_cjc_scale_dn1 = (assign1030_e1246 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1030_e1249).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn1) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1030_e1251 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn1) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1030_e1249))) });
        locals.var_cjc_scale_dn3 = (assign1030_e1246 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1030_e1249).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn3) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1030_e1251 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn3) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1030_e1249))) });
        locals.var_cjc_scale_dn4 = (assign1030_e1246 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1030_e1249).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn4) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1030_e1251 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn4) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1030_e1249))) });
        locals.var_cjc_scale_dn5 = (assign1030_e1246 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1030_e1249).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn5) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1030_e1251 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn5) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1030_e1249))) });
        locals.var_cjc_scale_dn6 = (assign1030_e1246 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1030_e1249).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn6) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1030_e1251 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn6) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1030_e1249))) });
        locals.var_cjc_scale_dn7 = (assign1030_e1246 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1030_e1249).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn7) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1030_e1251 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn7) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1030_e1249))) });
        locals.var_cjc_scale_dn8 = (assign1030_e1246 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1030_e1249).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn8) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1030_e1251 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn8) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1030_e1249))) });
        locals.var_cjc_scale_dn9 = (assign1030_e1246 * if 0.0 == 0.0 && ((p.p71) as f64).is_finite() && ((p.p71) as f64).fract() == 0.0 { if p.p71 == 0.0 { 0.0 } else { (p.p71 * ((assign1030_e1249).powf(p.p71 - 1.0) * (-((p.p70 * locals.var_vdc_ctc_t_dn9) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1030_e1251 * (p.p71 * ((-((p.p70 * locals.var_vdc_ctc_t_dn9) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1030_e1249))) });

        let assign1040_e1257: f64 = (1.0 / locals.var_cjc_scale);
        locals.var_cjc_scale_inv = assign1040_e1257;
        locals.var_cjc_scale_inv_dn0 = (-(locals.var_cjc_scale_dn0 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn1 = (-(locals.var_cjc_scale_dn1 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn3 = (-(locals.var_cjc_scale_dn3 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn4 = (-(locals.var_cjc_scale_dn4 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn5 = (-(locals.var_cjc_scale_dn5 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn6 = (-(locals.var_cjc_scale_dn6 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn7 = (-(locals.var_cjc_scale_dn7 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn8 = (-(locals.var_cjc_scale_dn8 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn9 = (-(locals.var_cjc_scale_dn9 / (locals.var_cjc_scale * locals.var_cjc_scale)));

        let assign1050_e1260: f64 = (p.p69 * locals.var_cjc_scale);
        locals.var_cjc_t = assign1050_e1260;
        locals.var_cjc_t_dn0 = (p.p69 * locals.var_cjc_scale_dn0);
        locals.var_cjc_t_dn1 = (p.p69 * locals.var_cjc_scale_dn1);
        locals.var_cjc_t_dn3 = (p.p69 * locals.var_cjc_scale_dn3);
        locals.var_cjc_t_dn4 = (p.p69 * locals.var_cjc_scale_dn4);
        locals.var_cjc_t_dn5 = (p.p69 * locals.var_cjc_scale_dn5);
        locals.var_cjc_t_dn6 = (p.p69 * locals.var_cjc_scale_dn6);
        locals.var_cjc_t_dn7 = (p.p69 * locals.var_cjc_scale_dn7);
        locals.var_cjc_t_dn8 = (p.p69 * locals.var_cjc_scale_dn8);
        locals.var_cjc_t_dn9 = (p.p69 * locals.var_cjc_scale_dn9);

        let assign1060_e1263: f64 = (p.p74 * locals.var_cjc_scale_inv);
        locals.var_xp_t = assign1060_e1263;
        locals.var_xp_t_dn0 = (p.p74 * locals.var_cjc_scale_inv_dn0);
        locals.var_xp_t_dn1 = (p.p74 * locals.var_cjc_scale_inv_dn1);
        locals.var_xp_t_dn3 = (p.p74 * locals.var_cjc_scale_inv_dn3);
        locals.var_xp_t_dn4 = (p.p74 * locals.var_cjc_scale_inv_dn4);
        locals.var_xp_t_dn5 = (p.p74 * locals.var_cjc_scale_inv_dn5);
        locals.var_xp_t_dn6 = (p.p74 * locals.var_cjc_scale_inv_dn6);
        locals.var_xp_t_dn7 = (p.p74 * locals.var_cjc_scale_inv_dn7);
        locals.var_xp_t_dn8 = (p.p74 * locals.var_cjc_scale_inv_dn8);
        locals.var_xp_t_dn9 = (p.p74 * locals.var_cjc_scale_inv_dn9);

        let assign1070_e1267: f64 = (locals.var_lntn * p.p96);
        let assign1070_e1268: f64 = (assign1070_e1267).exp();
        let assign1070_e1269: f64 = (p.p53 * assign1070_e1268);
        locals.var_re_t = assign1070_e1269;

        let assign1080_e1272: f64 = if locals.var_re_t < locals.var_minr_m { 1.0 } else { 0.0 };
        locals.var_guard13 = assign1080_e1272;

        let (assign1090_e1276,) = {
    if (locals.var_guard13 != 0.0) {
        (locals.var_minr_m,)
    } else {
        (locals.var_re_t,)
    }
};
        locals.var_re_t = assign1090_e1276;

        let assign1100_e1281: f64 = (p.p97 - p.p95);
        let assign1100_e1282: f64 = (locals.var_lntn * assign1100_e1281);
        let assign1100_e1283: f64 = (assign1100_e1282).exp();
        let assign1100_e1284: f64 = (p.p55 * assign1100_e1283);
        locals.var_rbv_t = assign1100_e1284;

        let assign1110_e1288: f64 = (locals.var_lntn * p.p100);
        let assign1110_e1289: f64 = (assign1110_e1288).exp();
        let assign1110_e1290: f64 = (p.p54 * assign1110_e1289);
        locals.var_rbc_t = assign1110_e1290;

        let assign1120_e1293: f64 = if locals.var_rbc_t < locals.var_minr_m { 1.0 } else { 0.0 };
        locals.var_guard14 = assign1120_e1293;

        let (assign1130_e1297,) = {
    if (locals.var_guard14 != 0.0) {
        (locals.var_minr_m,)
    } else {
        (locals.var_rbc_t,)
    }
};
        locals.var_rbc_t = assign1130_e1297;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign1140_e1301: f64 = (locals.var_lntn * p.p101);
        let assign1140_e1302: f64 = (assign1140_e1301).exp();
        let assign1140_e1303: f64 = (p.p56 * assign1140_e1302);
        locals.var_rcc_xx_t = assign1140_e1303;

        let assign1170_e1319: f64 = (locals.var_lntn * p.p98);
        let assign1170_e1320: f64 = (assign1170_e1319).exp();
        let assign1170_e1321: f64 = (p.p59 * assign1170_e1320);
        locals.var_rcv_t = assign1170_e1321;

        let assign1180_e1324: f64 = if p.p121 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1180_e1324;

        let (assign1190_e1334, assign1190_e1334_d_n0, assign1190_e1334_d_n1, assign1190_e1334_d_n3, assign1190_e1334_d_n4, assign1190_e1334_d_n5, assign1190_e1334_d_n6, assign1190_e1334_d_n7, assign1190_e1334_d_n8, assign1190_e1334_d_n9,) = {
    if (locals.var_guard15 != 0.0) {
        let assign1190_e1330: f64 = (locals.var_dt * p.p121);
        let assign1190_e1331: f64 = (1.0 + assign1190_e1330);
        let assign1190_e1332: f64 = (p.p9 * assign1190_e1331);
        (assign1190_e1332, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nff_t_tmp, locals.var_nff_t_tmp_dn0, locals.var_nff_t_tmp_dn1, locals.var_nff_t_tmp_dn3, locals.var_nff_t_tmp_dn4, locals.var_nff_t_tmp_dn5, locals.var_nff_t_tmp_dn6, locals.var_nff_t_tmp_dn7, locals.var_nff_t_tmp_dn8, locals.var_nff_t_tmp_dn9,)
    }
};
        locals.var_nff_t_tmp = assign1190_e1334;
        locals.var_nff_t_tmp_dn0 = assign1190_e1334_d_n0;
        locals.var_nff_t_tmp_dn1 = assign1190_e1334_d_n1;
        locals.var_nff_t_tmp_dn3 = assign1190_e1334_d_n3;
        locals.var_nff_t_tmp_dn4 = assign1190_e1334_d_n4;
        locals.var_nff_t_tmp_dn5 = assign1190_e1334_d_n5;
        locals.var_nff_t_tmp_dn6 = assign1190_e1334_d_n6;
        locals.var_nff_t_tmp_dn7 = assign1190_e1334_d_n7;
        locals.var_nff_t_tmp_dn8 = assign1190_e1334_d_n8;
        locals.var_nff_t_tmp_dn9 = assign1190_e1334_d_n9;

        let (assign1200_e1342, assign1200_e1342_d_n0, assign1200_e1342_d_n1, assign1200_e1342_d_n3, assign1200_e1342_d_n4, assign1200_e1342_d_n5, assign1200_e1342_d_n6, assign1200_e1342_d_n7, assign1200_e1342_d_n8, assign1200_e1342_d_n9,) = {
    if (locals.var_guard15 != 0.0) {
        let assign1200_e1338: f64 = (locals.var_nff_t_tmp - 1.0);
        let assign1200_e1340: f64 = (assign1200_e1338 / locals.var_eps_nf);
        (assign1200_e1340, (locals.var_nff_t_tmp_dn0 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn1 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn3 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn4 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn5 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn6 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn7 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn8 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn9 / locals.var_eps_nf),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9,)
    }
};
        locals.var_dxa = assign1200_e1342;
        locals.var_dxa_dn0 = assign1200_e1342_d_n0;
        locals.var_dxa_dn1 = assign1200_e1342_d_n1;
        locals.var_dxa_dn3 = assign1200_e1342_d_n3;
        locals.var_dxa_dn4 = assign1200_e1342_d_n4;
        locals.var_dxa_dn5 = assign1200_e1342_d_n5;
        locals.var_dxa_dn6 = assign1200_e1342_d_n6;
        locals.var_dxa_dn7 = assign1200_e1342_d_n7;
        locals.var_dxa_dn8 = assign1200_e1342_d_n8;
        locals.var_dxa_dn9 = assign1200_e1342_d_n9;

        let assign1210_e1345: f64 = if locals.var_nff_t_tmp < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1210_e1345;

        let (assign1220_e1359, assign1220_e1359_d_n0, assign1220_e1359_d_n1, assign1220_e1359_d_n3, assign1220_e1359_d_n4, assign1220_e1359_d_n5, assign1220_e1359_d_n6, assign1220_e1359_d_n7, assign1220_e1359_d_n8, assign1220_e1359_d_n9,) = {
    if ((locals.var_guard15 != 0.0) && (locals.var_guard16 != 0.0)) {
        let assign1220_e1353: f64 = (locals.var_dxa).exp();
        let assign1220_e1354: f64 = (1.0 + assign1220_e1353);
        let assign1220_e1355: f64 = (assign1220_e1354).ln();
        let assign1220_e1356: f64 = (locals.var_eps_nf * assign1220_e1355);
        let assign1220_e1357: f64 = (1.0 + assign1220_e1356);
        (assign1220_e1357, (locals.var_eps_nf * ((assign1220_e1353 * locals.var_dxa_dn0) / assign1220_e1354)), (locals.var_eps_nf * ((assign1220_e1353 * locals.var_dxa_dn1) / assign1220_e1354)), (locals.var_eps_nf * ((assign1220_e1353 * locals.var_dxa_dn3) / assign1220_e1354)), (locals.var_eps_nf * ((assign1220_e1353 * locals.var_dxa_dn4) / assign1220_e1354)), (locals.var_eps_nf * ((assign1220_e1353 * locals.var_dxa_dn5) / assign1220_e1354)), (locals.var_eps_nf * ((assign1220_e1353 * locals.var_dxa_dn6) / assign1220_e1354)), (locals.var_eps_nf * ((assign1220_e1353 * locals.var_dxa_dn7) / assign1220_e1354)), (locals.var_eps_nf * ((assign1220_e1353 * locals.var_dxa_dn8) / assign1220_e1354)), (locals.var_eps_nf * ((assign1220_e1353 * locals.var_dxa_dn9) / assign1220_e1354)),)
    } else {
        (locals.var_nff_t_tmp, locals.var_nff_t_tmp_dn0, locals.var_nff_t_tmp_dn1, locals.var_nff_t_tmp_dn3, locals.var_nff_t_tmp_dn4, locals.var_nff_t_tmp_dn5, locals.var_nff_t_tmp_dn6, locals.var_nff_t_tmp_dn7, locals.var_nff_t_tmp_dn8, locals.var_nff_t_tmp_dn9,)
    }
};
        locals.var_nff_t_tmp = assign1220_e1359;
        locals.var_nff_t_tmp_dn0 = assign1220_e1359_d_n0;
        locals.var_nff_t_tmp_dn1 = assign1220_e1359_d_n1;
        locals.var_nff_t_tmp_dn3 = assign1220_e1359_d_n3;
        locals.var_nff_t_tmp_dn4 = assign1220_e1359_d_n4;
        locals.var_nff_t_tmp_dn5 = assign1220_e1359_d_n5;
        locals.var_nff_t_tmp_dn6 = assign1220_e1359_d_n6;
        locals.var_nff_t_tmp_dn7 = assign1220_e1359_d_n7;
        locals.var_nff_t_tmp_dn8 = assign1220_e1359_d_n8;
        locals.var_nff_t_tmp_dn9 = assign1220_e1359_d_n9;

        let (assign1230_e1375, assign1230_e1375_d_n0, assign1230_e1375_d_n1, assign1230_e1375_d_n3, assign1230_e1375_d_n4, assign1230_e1375_d_n5, assign1230_e1375_d_n6, assign1230_e1375_d_n7, assign1230_e1375_d_n8, assign1230_e1375_d_n9,) = {
    if ((locals.var_guard15 != 0.0) && (locals.var_guard16 == 0.0)) {
        let assign1230_e1368: f64 = (-locals.var_dxa);
        let assign1230_e1369: f64 = (assign1230_e1368).exp();
        let assign1230_e1370: f64 = (1.0 + assign1230_e1369);
        let assign1230_e1371: f64 = (assign1230_e1370).ln();
        let assign1230_e1372: f64 = (locals.var_eps_nf * assign1230_e1371);
        let assign1230_e1373: f64 = (locals.var_nff_t_tmp + assign1230_e1372);
        (assign1230_e1373, (locals.var_nff_t_tmp_dn0 + (locals.var_eps_nf * ((assign1230_e1369 * (-locals.var_dxa_dn0)) / assign1230_e1370))), (locals.var_nff_t_tmp_dn1 + (locals.var_eps_nf * ((assign1230_e1369 * (-locals.var_dxa_dn1)) / assign1230_e1370))), (locals.var_nff_t_tmp_dn3 + (locals.var_eps_nf * ((assign1230_e1369 * (-locals.var_dxa_dn3)) / assign1230_e1370))), (locals.var_nff_t_tmp_dn4 + (locals.var_eps_nf * ((assign1230_e1369 * (-locals.var_dxa_dn4)) / assign1230_e1370))), (locals.var_nff_t_tmp_dn5 + (locals.var_eps_nf * ((assign1230_e1369 * (-locals.var_dxa_dn5)) / assign1230_e1370))), (locals.var_nff_t_tmp_dn6 + (locals.var_eps_nf * ((assign1230_e1369 * (-locals.var_dxa_dn6)) / assign1230_e1370))), (locals.var_nff_t_tmp_dn7 + (locals.var_eps_nf * ((assign1230_e1369 * (-locals.var_dxa_dn7)) / assign1230_e1370))), (locals.var_nff_t_tmp_dn8 + (locals.var_eps_nf * ((assign1230_e1369 * (-locals.var_dxa_dn8)) / assign1230_e1370))), (locals.var_nff_t_tmp_dn9 + (locals.var_eps_nf * ((assign1230_e1369 * (-locals.var_dxa_dn9)) / assign1230_e1370))),)
    } else {
        (locals.var_nff_t_tmp, locals.var_nff_t_tmp_dn0, locals.var_nff_t_tmp_dn1, locals.var_nff_t_tmp_dn3, locals.var_nff_t_tmp_dn4, locals.var_nff_t_tmp_dn5, locals.var_nff_t_tmp_dn6, locals.var_nff_t_tmp_dn7, locals.var_nff_t_tmp_dn8, locals.var_nff_t_tmp_dn9,)
    }
};
        locals.var_nff_t_tmp = assign1230_e1375;
        locals.var_nff_t_tmp_dn0 = assign1230_e1375_d_n0;
        locals.var_nff_t_tmp_dn1 = assign1230_e1375_d_n1;
        locals.var_nff_t_tmp_dn3 = assign1230_e1375_d_n3;
        locals.var_nff_t_tmp_dn4 = assign1230_e1375_d_n4;
        locals.var_nff_t_tmp_dn5 = assign1230_e1375_d_n5;
        locals.var_nff_t_tmp_dn6 = assign1230_e1375_d_n6;
        locals.var_nff_t_tmp_dn7 = assign1230_e1375_d_n7;
        locals.var_nff_t_tmp_dn8 = assign1230_e1375_d_n8;
        locals.var_nff_t_tmp_dn9 = assign1230_e1375_d_n9;

        let (assign1240_e1383, assign1240_e1383_d_n0, assign1240_e1383_d_n1, assign1240_e1383_d_n3, assign1240_e1383_d_n4, assign1240_e1383_d_n5, assign1240_e1383_d_n6, assign1240_e1383_d_n7, assign1240_e1383_d_n8, assign1240_e1383_d_n9,) = {
    if (locals.var_guard15 != 0.0) {
        let assign1240_e1380: f64 = (locals.var_eps_nf * 0.6931471805599453);
        let assign1240_e1381: f64 = (locals.var_nff_t_tmp - assign1240_e1380);
        (assign1240_e1381, locals.var_nff_t_tmp_dn0, locals.var_nff_t_tmp_dn1, locals.var_nff_t_tmp_dn3, locals.var_nff_t_tmp_dn4, locals.var_nff_t_tmp_dn5, locals.var_nff_t_tmp_dn6, locals.var_nff_t_tmp_dn7, locals.var_nff_t_tmp_dn8, locals.var_nff_t_tmp_dn9,)
    } else {
        (locals.var_nff_t, locals.var_nff_t_dn0, locals.var_nff_t_dn1, locals.var_nff_t_dn3, locals.var_nff_t_dn4, locals.var_nff_t_dn5, locals.var_nff_t_dn6, locals.var_nff_t_dn7, locals.var_nff_t_dn8, locals.var_nff_t_dn9,)
    }
};
        locals.var_nff_t = assign1240_e1383;
        locals.var_nff_t_dn0 = assign1240_e1383_d_n0;
        locals.var_nff_t_dn1 = assign1240_e1383_d_n1;
        locals.var_nff_t_dn3 = assign1240_e1383_d_n3;
        locals.var_nff_t_dn4 = assign1240_e1383_d_n4;
        locals.var_nff_t_dn5 = assign1240_e1383_d_n5;
        locals.var_nff_t_dn6 = assign1240_e1383_d_n6;
        locals.var_nff_t_dn7 = assign1240_e1383_d_n7;
        locals.var_nff_t_dn8 = assign1240_e1383_d_n8;
        locals.var_nff_t_dn9 = assign1240_e1383_d_n9;

        let (assign1250_e1388, assign1250_e1388_d_n0, assign1250_e1388_d_n1, assign1250_e1388_d_n3, assign1250_e1388_d_n4, assign1250_e1388_d_n5, assign1250_e1388_d_n6, assign1250_e1388_d_n7, assign1250_e1388_d_n8, assign1250_e1388_d_n9,) = {
    if (locals.var_guard15 == 0.0) {
        (p.p9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nff_t, locals.var_nff_t_dn0, locals.var_nff_t_dn1, locals.var_nff_t_dn3, locals.var_nff_t_dn4, locals.var_nff_t_dn5, locals.var_nff_t_dn6, locals.var_nff_t_dn7, locals.var_nff_t_dn8, locals.var_nff_t_dn9,)
    }
};
        locals.var_nff_t = assign1250_e1388;
        locals.var_nff_t_dn0 = assign1250_e1388_d_n0;
        locals.var_nff_t_dn1 = assign1250_e1388_d_n1;
        locals.var_nff_t_dn3 = assign1250_e1388_d_n3;
        locals.var_nff_t_dn4 = assign1250_e1388_d_n4;
        locals.var_nff_t_dn5 = assign1250_e1388_d_n5;
        locals.var_nff_t_dn6 = assign1250_e1388_d_n6;
        locals.var_nff_t_dn7 = assign1250_e1388_d_n7;
        locals.var_nff_t_dn8 = assign1250_e1388_d_n8;
        locals.var_nff_t_dn9 = assign1250_e1388_d_n9;

        let assign1260_e1391: f64 = if p.p122 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1260_e1391;

        let (assign1270_e1401, assign1270_e1401_d_n0, assign1270_e1401_d_n1, assign1270_e1401_d_n3, assign1270_e1401_d_n4, assign1270_e1401_d_n5, assign1270_e1401_d_n6, assign1270_e1401_d_n7, assign1270_e1401_d_n8, assign1270_e1401_d_n9,) = {
    if (locals.var_guard17 != 0.0) {
        let assign1270_e1397: f64 = (locals.var_dt * p.p122);
        let assign1270_e1398: f64 = (1.0 + assign1270_e1397);
        let assign1270_e1399: f64 = (p.p10 * assign1270_e1398);
        (assign1270_e1399, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nfr_t_tmp, locals.var_nfr_t_tmp_dn0, locals.var_nfr_t_tmp_dn1, locals.var_nfr_t_tmp_dn3, locals.var_nfr_t_tmp_dn4, locals.var_nfr_t_tmp_dn5, locals.var_nfr_t_tmp_dn6, locals.var_nfr_t_tmp_dn7, locals.var_nfr_t_tmp_dn8, locals.var_nfr_t_tmp_dn9,)
    }
};
        locals.var_nfr_t_tmp = assign1270_e1401;
        locals.var_nfr_t_tmp_dn0 = assign1270_e1401_d_n0;
        locals.var_nfr_t_tmp_dn1 = assign1270_e1401_d_n1;
        locals.var_nfr_t_tmp_dn3 = assign1270_e1401_d_n3;
        locals.var_nfr_t_tmp_dn4 = assign1270_e1401_d_n4;
        locals.var_nfr_t_tmp_dn5 = assign1270_e1401_d_n5;
        locals.var_nfr_t_tmp_dn6 = assign1270_e1401_d_n6;
        locals.var_nfr_t_tmp_dn7 = assign1270_e1401_d_n7;
        locals.var_nfr_t_tmp_dn8 = assign1270_e1401_d_n8;
        locals.var_nfr_t_tmp_dn9 = assign1270_e1401_d_n9;

        let (assign1280_e1409, assign1280_e1409_d_n0, assign1280_e1409_d_n1, assign1280_e1409_d_n3, assign1280_e1409_d_n4, assign1280_e1409_d_n5, assign1280_e1409_d_n6, assign1280_e1409_d_n7, assign1280_e1409_d_n8, assign1280_e1409_d_n9,) = {
    if (locals.var_guard17 != 0.0) {
        let assign1280_e1405: f64 = (locals.var_nfr_t_tmp - 1.0);
        let assign1280_e1407: f64 = (assign1280_e1405 / locals.var_eps_nf);
        (assign1280_e1407, (locals.var_nfr_t_tmp_dn0 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn1 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn3 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn4 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn5 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn6 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn7 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn8 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn9 / locals.var_eps_nf),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9,)
    }
};
        locals.var_dxa = assign1280_e1409;
        locals.var_dxa_dn0 = assign1280_e1409_d_n0;
        locals.var_dxa_dn1 = assign1280_e1409_d_n1;
        locals.var_dxa_dn3 = assign1280_e1409_d_n3;
        locals.var_dxa_dn4 = assign1280_e1409_d_n4;
        locals.var_dxa_dn5 = assign1280_e1409_d_n5;
        locals.var_dxa_dn6 = assign1280_e1409_d_n6;
        locals.var_dxa_dn7 = assign1280_e1409_d_n7;
        locals.var_dxa_dn8 = assign1280_e1409_d_n8;
        locals.var_dxa_dn9 = assign1280_e1409_d_n9;

        let assign1290_e1412: f64 = if locals.var_nfr_t_tmp < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard18 = assign1290_e1412;

        let (assign1300_e1426, assign1300_e1426_d_n0, assign1300_e1426_d_n1, assign1300_e1426_d_n3, assign1300_e1426_d_n4, assign1300_e1426_d_n5, assign1300_e1426_d_n6, assign1300_e1426_d_n7, assign1300_e1426_d_n8, assign1300_e1426_d_n9,) = {
    if ((locals.var_guard17 != 0.0) && (locals.var_guard18 != 0.0)) {
        let assign1300_e1420: f64 = (locals.var_dxa).exp();
        let assign1300_e1421: f64 = (1.0 + assign1300_e1420);
        let assign1300_e1422: f64 = (assign1300_e1421).ln();
        let assign1300_e1423: f64 = (locals.var_eps_nf * assign1300_e1422);
        let assign1300_e1424: f64 = (1.0 + assign1300_e1423);
        (assign1300_e1424, (locals.var_eps_nf * ((assign1300_e1420 * locals.var_dxa_dn0) / assign1300_e1421)), (locals.var_eps_nf * ((assign1300_e1420 * locals.var_dxa_dn1) / assign1300_e1421)), (locals.var_eps_nf * ((assign1300_e1420 * locals.var_dxa_dn3) / assign1300_e1421)), (locals.var_eps_nf * ((assign1300_e1420 * locals.var_dxa_dn4) / assign1300_e1421)), (locals.var_eps_nf * ((assign1300_e1420 * locals.var_dxa_dn5) / assign1300_e1421)), (locals.var_eps_nf * ((assign1300_e1420 * locals.var_dxa_dn6) / assign1300_e1421)), (locals.var_eps_nf * ((assign1300_e1420 * locals.var_dxa_dn7) / assign1300_e1421)), (locals.var_eps_nf * ((assign1300_e1420 * locals.var_dxa_dn8) / assign1300_e1421)), (locals.var_eps_nf * ((assign1300_e1420 * locals.var_dxa_dn9) / assign1300_e1421)),)
    } else {
        (locals.var_nfr_t_tmp, locals.var_nfr_t_tmp_dn0, locals.var_nfr_t_tmp_dn1, locals.var_nfr_t_tmp_dn3, locals.var_nfr_t_tmp_dn4, locals.var_nfr_t_tmp_dn5, locals.var_nfr_t_tmp_dn6, locals.var_nfr_t_tmp_dn7, locals.var_nfr_t_tmp_dn8, locals.var_nfr_t_tmp_dn9,)
    }
};
        locals.var_nfr_t_tmp = assign1300_e1426;
        locals.var_nfr_t_tmp_dn0 = assign1300_e1426_d_n0;
        locals.var_nfr_t_tmp_dn1 = assign1300_e1426_d_n1;
        locals.var_nfr_t_tmp_dn3 = assign1300_e1426_d_n3;
        locals.var_nfr_t_tmp_dn4 = assign1300_e1426_d_n4;
        locals.var_nfr_t_tmp_dn5 = assign1300_e1426_d_n5;
        locals.var_nfr_t_tmp_dn6 = assign1300_e1426_d_n6;
        locals.var_nfr_t_tmp_dn7 = assign1300_e1426_d_n7;
        locals.var_nfr_t_tmp_dn8 = assign1300_e1426_d_n8;
        locals.var_nfr_t_tmp_dn9 = assign1300_e1426_d_n9;

        let (assign1310_e1442, assign1310_e1442_d_n0, assign1310_e1442_d_n1, assign1310_e1442_d_n3, assign1310_e1442_d_n4, assign1310_e1442_d_n5, assign1310_e1442_d_n6, assign1310_e1442_d_n7, assign1310_e1442_d_n8, assign1310_e1442_d_n9,) = {
    if ((locals.var_guard17 != 0.0) && (locals.var_guard18 == 0.0)) {
        let assign1310_e1435: f64 = (-locals.var_dxa);
        let assign1310_e1436: f64 = (assign1310_e1435).exp();
        let assign1310_e1437: f64 = (1.0 + assign1310_e1436);
        let assign1310_e1438: f64 = (assign1310_e1437).ln();
        let assign1310_e1439: f64 = (locals.var_eps_nf * assign1310_e1438);
        let assign1310_e1440: f64 = (locals.var_nfr_t_tmp + assign1310_e1439);
        (assign1310_e1440, (locals.var_nfr_t_tmp_dn0 + (locals.var_eps_nf * ((assign1310_e1436 * (-locals.var_dxa_dn0)) / assign1310_e1437))), (locals.var_nfr_t_tmp_dn1 + (locals.var_eps_nf * ((assign1310_e1436 * (-locals.var_dxa_dn1)) / assign1310_e1437))), (locals.var_nfr_t_tmp_dn3 + (locals.var_eps_nf * ((assign1310_e1436 * (-locals.var_dxa_dn3)) / assign1310_e1437))), (locals.var_nfr_t_tmp_dn4 + (locals.var_eps_nf * ((assign1310_e1436 * (-locals.var_dxa_dn4)) / assign1310_e1437))), (locals.var_nfr_t_tmp_dn5 + (locals.var_eps_nf * ((assign1310_e1436 * (-locals.var_dxa_dn5)) / assign1310_e1437))), (locals.var_nfr_t_tmp_dn6 + (locals.var_eps_nf * ((assign1310_e1436 * (-locals.var_dxa_dn6)) / assign1310_e1437))), (locals.var_nfr_t_tmp_dn7 + (locals.var_eps_nf * ((assign1310_e1436 * (-locals.var_dxa_dn7)) / assign1310_e1437))), (locals.var_nfr_t_tmp_dn8 + (locals.var_eps_nf * ((assign1310_e1436 * (-locals.var_dxa_dn8)) / assign1310_e1437))), (locals.var_nfr_t_tmp_dn9 + (locals.var_eps_nf * ((assign1310_e1436 * (-locals.var_dxa_dn9)) / assign1310_e1437))),)
    } else {
        (locals.var_nfr_t_tmp, locals.var_nfr_t_tmp_dn0, locals.var_nfr_t_tmp_dn1, locals.var_nfr_t_tmp_dn3, locals.var_nfr_t_tmp_dn4, locals.var_nfr_t_tmp_dn5, locals.var_nfr_t_tmp_dn6, locals.var_nfr_t_tmp_dn7, locals.var_nfr_t_tmp_dn8, locals.var_nfr_t_tmp_dn9,)
    }
};
        locals.var_nfr_t_tmp = assign1310_e1442;
        locals.var_nfr_t_tmp_dn0 = assign1310_e1442_d_n0;
        locals.var_nfr_t_tmp_dn1 = assign1310_e1442_d_n1;
        locals.var_nfr_t_tmp_dn3 = assign1310_e1442_d_n3;
        locals.var_nfr_t_tmp_dn4 = assign1310_e1442_d_n4;
        locals.var_nfr_t_tmp_dn5 = assign1310_e1442_d_n5;
        locals.var_nfr_t_tmp_dn6 = assign1310_e1442_d_n6;
        locals.var_nfr_t_tmp_dn7 = assign1310_e1442_d_n7;
        locals.var_nfr_t_tmp_dn8 = assign1310_e1442_d_n8;
        locals.var_nfr_t_tmp_dn9 = assign1310_e1442_d_n9;

        let (assign1320_e1450, assign1320_e1450_d_n0, assign1320_e1450_d_n1, assign1320_e1450_d_n3, assign1320_e1450_d_n4, assign1320_e1450_d_n5, assign1320_e1450_d_n6, assign1320_e1450_d_n7, assign1320_e1450_d_n8, assign1320_e1450_d_n9,) = {
    if (locals.var_guard17 != 0.0) {
        let assign1320_e1447: f64 = (locals.var_eps_nf * 0.6931471805599453);
        let assign1320_e1448: f64 = (locals.var_nfr_t_tmp - assign1320_e1447);
        (assign1320_e1448, locals.var_nfr_t_tmp_dn0, locals.var_nfr_t_tmp_dn1, locals.var_nfr_t_tmp_dn3, locals.var_nfr_t_tmp_dn4, locals.var_nfr_t_tmp_dn5, locals.var_nfr_t_tmp_dn6, locals.var_nfr_t_tmp_dn7, locals.var_nfr_t_tmp_dn8, locals.var_nfr_t_tmp_dn9,)
    } else {
        (locals.var_nfr_t, locals.var_nfr_t_dn0, locals.var_nfr_t_dn1, locals.var_nfr_t_dn3, locals.var_nfr_t_dn4, locals.var_nfr_t_dn5, locals.var_nfr_t_dn6, locals.var_nfr_t_dn7, locals.var_nfr_t_dn8, locals.var_nfr_t_dn9,)
    }
};
        locals.var_nfr_t = assign1320_e1450;
        locals.var_nfr_t_dn0 = assign1320_e1450_d_n0;
        locals.var_nfr_t_dn1 = assign1320_e1450_d_n1;
        locals.var_nfr_t_dn3 = assign1320_e1450_d_n3;
        locals.var_nfr_t_dn4 = assign1320_e1450_d_n4;
        locals.var_nfr_t_dn5 = assign1320_e1450_d_n5;
        locals.var_nfr_t_dn6 = assign1320_e1450_d_n6;
        locals.var_nfr_t_dn7 = assign1320_e1450_d_n7;
        locals.var_nfr_t_dn8 = assign1320_e1450_d_n8;
        locals.var_nfr_t_dn9 = assign1320_e1450_d_n9;

        let (assign1330_e1455, assign1330_e1455_d_n0, assign1330_e1455_d_n1, assign1330_e1455_d_n3, assign1330_e1455_d_n4, assign1330_e1455_d_n5, assign1330_e1455_d_n6, assign1330_e1455_d_n7, assign1330_e1455_d_n8, assign1330_e1455_d_n9,) = {
    if (locals.var_guard17 == 0.0) {
        (p.p10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nfr_t, locals.var_nfr_t_dn0, locals.var_nfr_t_dn1, locals.var_nfr_t_dn3, locals.var_nfr_t_dn4, locals.var_nfr_t_dn5, locals.var_nfr_t_dn6, locals.var_nfr_t_dn7, locals.var_nfr_t_dn8, locals.var_nfr_t_dn9,)
    }
};
        locals.var_nfr_t = assign1330_e1455;
        locals.var_nfr_t_dn0 = assign1330_e1455_d_n0;
        locals.var_nfr_t_dn1 = assign1330_e1455_d_n1;
        locals.var_nfr_t_dn3 = assign1330_e1455_d_n3;
        locals.var_nfr_t_dn4 = assign1330_e1455_d_n4;
        locals.var_nfr_t_dn5 = assign1330_e1455_d_n5;
        locals.var_nfr_t_dn6 = assign1330_e1455_d_n6;
        locals.var_nfr_t_dn7 = assign1330_e1455_d_n7;
        locals.var_nfr_t_dn8 = assign1330_e1455_d_n8;
        locals.var_nfr_t_dn9 = assign1330_e1455_d_n9;

        let assign1340_e1460: f64 = (p.p123 * locals.var_dt);
        let assign1340_e1461: f64 = (1.0 + assign1340_e1460);
        let assign1340_e1462: f64 = (p.p42 * assign1340_e1461);
        locals.var_bavl_t_tmp = assign1340_e1462;

        let assign1350_e1465: f64 = (locals.var_eps_bavl_t * locals.var_eps_bavl_t);
        locals.var_eps2 = assign1350_e1465;
        locals.var_eps2_dn0 = 0.0;
        locals.var_eps2_dn1 = 0.0;
        locals.var_eps2_dn3 = 0.0;
        locals.var_eps2_dn4 = 0.0;
        locals.var_eps2_dn5 = 0.0;
        locals.var_eps2_dn6 = 0.0;
        locals.var_eps2_dn7 = 0.0;
        locals.var_eps2_dn8 = 0.0;
        locals.var_eps2_dn9 = 0.0;

        let assign1360_e1468: f64 = (locals.var_bavl_t_tmp * locals.var_bavl_t_tmp);
        locals.var_x2 = assign1360_e1468;
        locals.var_x2_dn0 = 0.0;
        locals.var_x2_dn1 = 0.0;
        locals.var_x2_dn3 = 0.0;
        locals.var_x2_dn4 = 0.0;
        locals.var_x2_dn5 = 0.0;
        locals.var_x2_dn6 = 0.0;
        locals.var_x2_dn7 = 0.0;
        locals.var_x2_dn8 = 0.0;
        locals.var_x2_dn9 = 0.0;

        let assign1370_e1471: f64 = if locals.var_bavl_t_tmp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard19 = assign1370_e1471;

        let (assign1380_e1484, assign1380_e1484_d_n0, assign1380_e1484_d_n1, assign1380_e1484_d_n3, assign1380_e1484_d_n4, assign1380_e1484_d_n5, assign1380_e1484_d_n6, assign1380_e1484_d_n7, assign1380_e1484_d_n8, assign1380_e1484_d_n9,) = {
    if (locals.var_guard19 != 0.0) {
        let assign1380_e1475: f64 = (0.5 * locals.var_eps2);
        let assign1380_e1478: f64 = (locals.var_x2 + locals.var_eps2);
        let assign1380_e1479: f64 = (assign1380_e1478).sqrt();
        let assign1380_e1481: f64 = (assign1380_e1479 - locals.var_bavl_t_tmp);
        let assign1380_e1482: f64 = (assign1380_e1475 / assign1380_e1481);
        (assign1380_e1482, ((((0.5 * locals.var_eps2_dn0) * assign1380_e1481) - (assign1380_e1475 * ((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign1380_e1479)))) / (assign1380_e1481 * assign1380_e1481)), ((((0.5 * locals.var_eps2_dn1) * assign1380_e1481) - (assign1380_e1475 * ((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign1380_e1479)))) / (assign1380_e1481 * assign1380_e1481)), ((((0.5 * locals.var_eps2_dn3) * assign1380_e1481) - (assign1380_e1475 * ((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign1380_e1479)))) / (assign1380_e1481 * assign1380_e1481)), ((((0.5 * locals.var_eps2_dn4) * assign1380_e1481) - (assign1380_e1475 * ((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign1380_e1479)))) / (assign1380_e1481 * assign1380_e1481)), ((((0.5 * locals.var_eps2_dn5) * assign1380_e1481) - (assign1380_e1475 * ((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign1380_e1479)))) / (assign1380_e1481 * assign1380_e1481)), ((((0.5 * locals.var_eps2_dn6) * assign1380_e1481) - (assign1380_e1475 * ((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign1380_e1479)))) / (assign1380_e1481 * assign1380_e1481)), ((((0.5 * locals.var_eps2_dn7) * assign1380_e1481) - (assign1380_e1475 * ((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign1380_e1479)))) / (assign1380_e1481 * assign1380_e1481)), ((((0.5 * locals.var_eps2_dn8) * assign1380_e1481) - (assign1380_e1475 * ((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign1380_e1479)))) / (assign1380_e1481 * assign1380_e1481)), ((((0.5 * locals.var_eps2_dn9) * assign1380_e1481) - (assign1380_e1475 * ((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign1380_e1479)))) / (assign1380_e1481 * assign1380_e1481)),)
    } else {
        (locals.var_bavl_t, locals.var_bavl_t_dn0, locals.var_bavl_t_dn1, locals.var_bavl_t_dn3, locals.var_bavl_t_dn4, locals.var_bavl_t_dn5, locals.var_bavl_t_dn6, locals.var_bavl_t_dn7, locals.var_bavl_t_dn8, locals.var_bavl_t_dn9,)
    }
};
        locals.var_bavl_t = assign1380_e1484;
        locals.var_bavl_t_dn0 = assign1380_e1484_d_n0;
        locals.var_bavl_t_dn1 = assign1380_e1484_d_n1;
        locals.var_bavl_t_dn3 = assign1380_e1484_d_n3;
        locals.var_bavl_t_dn4 = assign1380_e1484_d_n4;
        locals.var_bavl_t_dn5 = assign1380_e1484_d_n5;
        locals.var_bavl_t_dn6 = assign1380_e1484_d_n6;
        locals.var_bavl_t_dn7 = assign1380_e1484_d_n7;
        locals.var_bavl_t_dn8 = assign1380_e1484_d_n8;
        locals.var_bavl_t_dn9 = assign1380_e1484_d_n9;

        let (assign1390_e1496, assign1390_e1496_d_n0, assign1390_e1496_d_n1, assign1390_e1496_d_n3, assign1390_e1496_d_n4, assign1390_e1496_d_n5, assign1390_e1496_d_n6, assign1390_e1496_d_n7, assign1390_e1496_d_n8, assign1390_e1496_d_n9,) = {
    if (locals.var_guard19 == 0.0) {
        let assign1390_e1490: f64 = (locals.var_x2 + locals.var_eps2);
        let assign1390_e1491: f64 = (assign1390_e1490).sqrt();
        let assign1390_e1493: f64 = (assign1390_e1491 + locals.var_bavl_t_tmp);
        let assign1390_e1494: f64 = (0.5 * assign1390_e1493);
        (assign1390_e1494, (0.5 * ((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign1390_e1491))), (0.5 * ((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign1390_e1491))), (0.5 * ((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign1390_e1491))), (0.5 * ((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign1390_e1491))), (0.5 * ((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign1390_e1491))), (0.5 * ((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign1390_e1491))), (0.5 * ((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign1390_e1491))), (0.5 * ((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign1390_e1491))), (0.5 * ((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign1390_e1491))),)
    } else {
        (locals.var_bavl_t, locals.var_bavl_t_dn0, locals.var_bavl_t_dn1, locals.var_bavl_t_dn3, locals.var_bavl_t_dn4, locals.var_bavl_t_dn5, locals.var_bavl_t_dn6, locals.var_bavl_t_dn7, locals.var_bavl_t_dn8, locals.var_bavl_t_dn9,)
    }
};
        locals.var_bavl_t = assign1390_e1496;
        locals.var_bavl_t_dn0 = assign1390_e1496_d_n0;
        locals.var_bavl_t_dn1 = assign1390_e1496_d_n1;
        locals.var_bavl_t_dn3 = assign1390_e1496_d_n3;
        locals.var_bavl_t_dn4 = assign1390_e1496_d_n4;
        locals.var_bavl_t_dn5 = assign1390_e1496_d_n5;
        locals.var_bavl_t_dn6 = assign1390_e1496_d_n6;
        locals.var_bavl_t_dn7 = assign1390_e1496_d_n7;
        locals.var_bavl_t_dn8 = assign1390_e1496_d_n8;
        locals.var_bavl_t_dn9 = assign1390_e1496_d_n9;

        let assign1400_e1501: f64 = (4.0 - p.p97);
        let assign1400_e1503: f64 = (assign1400_e1501 - p.p95);
        let assign1400_e1505: f64 = (assign1400_e1503 + p.p120);
        let assign1400_e1506: f64 = (locals.var_lntn * assign1400_e1505);
        let assign1400_e1508: f64 = (assign1400_e1506 / locals.var_nff_t);
        let assign1400_e1509: f64 = (assign1400_e1508).exp();
        let assign1400_e1510: f64 = (p.p8 * assign1400_e1509);
        let assign1400_e1512: f64 = (-p.p104);
        let assign1400_e1514: f64 = (assign1400_e1512 * locals.var_vdtinv);
        let assign1400_e1516: f64 = (assign1400_e1514 / locals.var_nff_t);
        let assign1400_e1517: f64 = (assign1400_e1516).exp();
        let assign1400_e1518: f64 = (assign1400_e1510 * assign1400_e1517);
        locals.var_is_t = assign1400_e1518;
        locals.var_is_t_dn0 = (((p.p8 * (assign1400_e1509 * (-((assign1400_e1506 * locals.var_nff_t_dn0) / (locals.var_nff_t * locals.var_nff_t))))) * assign1400_e1517) + (assign1400_e1510 * (assign1400_e1517 * (-((assign1400_e1514 * locals.var_nff_t_dn0) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn1 = (((p.p8 * (assign1400_e1509 * (-((assign1400_e1506 * locals.var_nff_t_dn1) / (locals.var_nff_t * locals.var_nff_t))))) * assign1400_e1517) + (assign1400_e1510 * (assign1400_e1517 * (-((assign1400_e1514 * locals.var_nff_t_dn1) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn3 = (((p.p8 * (assign1400_e1509 * (-((assign1400_e1506 * locals.var_nff_t_dn3) / (locals.var_nff_t * locals.var_nff_t))))) * assign1400_e1517) + (assign1400_e1510 * (assign1400_e1517 * (-((assign1400_e1514 * locals.var_nff_t_dn3) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn4 = (((p.p8 * (assign1400_e1509 * (-((assign1400_e1506 * locals.var_nff_t_dn4) / (locals.var_nff_t * locals.var_nff_t))))) * assign1400_e1517) + (assign1400_e1510 * (assign1400_e1517 * (-((assign1400_e1514 * locals.var_nff_t_dn4) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn5 = (((p.p8 * (assign1400_e1509 * (-((assign1400_e1506 * locals.var_nff_t_dn5) / (locals.var_nff_t * locals.var_nff_t))))) * assign1400_e1517) + (assign1400_e1510 * (assign1400_e1517 * (-((assign1400_e1514 * locals.var_nff_t_dn5) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn6 = (((p.p8 * (assign1400_e1509 * (-((assign1400_e1506 * locals.var_nff_t_dn6) / (locals.var_nff_t * locals.var_nff_t))))) * assign1400_e1517) + (assign1400_e1510 * (assign1400_e1517 * (-((assign1400_e1514 * locals.var_nff_t_dn6) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn7 = (((p.p8 * (assign1400_e1509 * (-((assign1400_e1506 * locals.var_nff_t_dn7) / (locals.var_nff_t * locals.var_nff_t))))) * assign1400_e1517) + (assign1400_e1510 * (assign1400_e1517 * (-((assign1400_e1514 * locals.var_nff_t_dn7) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn8 = (((p.p8 * (assign1400_e1509 * (-((assign1400_e1506 * locals.var_nff_t_dn8) / (locals.var_nff_t * locals.var_nff_t))))) * assign1400_e1517) + (assign1400_e1510 * (assign1400_e1517 * (-((assign1400_e1514 * locals.var_nff_t_dn8) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn9 = (((p.p8 * (assign1400_e1509 * (-((assign1400_e1506 * locals.var_nff_t_dn9) / (locals.var_nff_t * locals.var_nff_t))))) * assign1400_e1517) + (assign1400_e1510 * (assign1400_e1517 * (-((assign1400_e1514 * locals.var_nff_t_dn9) / (locals.var_nff_t * locals.var_nff_t))))));

        let assign1410_e1523: f64 = (1.0 - p.p97);
        let assign1410_e1524: f64 = (locals.var_lntn * assign1410_e1523);
        let assign1410_e1525: f64 = (assign1410_e1524).exp();
        let assign1410_e1526: f64 = (p.p11 * assign1410_e1525);
        locals.var_ik_t = assign1410_e1526;

        let assign1420_e1531: f64 = (1.0 - p.p102);
        let assign1420_e1532: f64 = (locals.var_lntn * assign1420_e1531);
        let assign1420_e1533: f64 = (assign1420_e1532).exp();
        let assign1420_e1534: f64 = (p.p29 * assign1420_e1533);
        locals.var_ikbx_t = assign1420_e1534;

        let assign1450_e1575: f64 = (4.0 - p.p96);
        let assign1450_e1577: f64 = (assign1450_e1575 + p.p120);
        let assign1450_e1578: f64 = (locals.var_lntn * assign1450_e1577);
        let assign1450_e1580: f64 = (assign1450_e1578 / p.p16);
        let assign1450_e1581: f64 = (assign1450_e1580).exp();
        let assign1450_e1582: f64 = (p.p15 * assign1450_e1581);
        let assign1450_e1584: f64 = (-p.p110);
        let assign1450_e1586: f64 = (assign1450_e1584 * locals.var_vdtinv);
        let assign1450_e1588: f64 = (assign1450_e1586 / p.p16);
        let assign1450_e1589: f64 = (assign1450_e1588).exp();
        let assign1450_e1590: f64 = (assign1450_e1582 * assign1450_e1589);
        locals.var_ibi_t = assign1450_e1590;

        let assign1510_e1652: f64 = (4.0 - p.p102);
        let assign1510_e1654: f64 = (assign1510_e1652 + p.p120);
        let assign1510_e1655: f64 = (locals.var_lntn * assign1510_e1654);
        let assign1510_e1656: f64 = (assign1510_e1655).exp();
        let assign1510_e1657: f64 = (p.p28 * assign1510_e1656);
        let assign1510_e1659: f64 = (-p.p111);
        let assign1510_e1661: f64 = (assign1510_e1659 * locals.var_vdtinv);
        let assign1510_e1662: f64 = (assign1510_e1661).exp();
        let assign1510_e1663: f64 = (assign1510_e1657 * assign1510_e1662);
        locals.var_ibx_t = assign1510_e1663;

        let assign1550_e1709: f64 = (locals.var_vgzeb_t * locals.var_inv_vgzeb_tr);
        let assign1550_e1711: f64 = (-0.5);
        let assign1550_e1712: f64 = (assign1550_e1709).powf(assign1550_e1711);
        locals.var_x = assign1550_e1712;
        locals.var_x_dn0 = if 0.0 == 0.0 && ((assign1550_e1711) as f64).is_finite() && ((assign1550_e1711) as f64).fract() == 0.0 { if assign1550_e1711 == 0.0 { 0.0 } else { (assign1550_e1711 * ((assign1550_e1709).powf(assign1550_e1711 - 1.0) * (locals.var_vgzeb_t_dn0 * locals.var_inv_vgzeb_tr))) } } else { (assign1550_e1712 * (assign1550_e1711 * ((locals.var_vgzeb_t_dn0 * locals.var_inv_vgzeb_tr) / assign1550_e1709))) };
        locals.var_x_dn1 = if 0.0 == 0.0 && ((assign1550_e1711) as f64).is_finite() && ((assign1550_e1711) as f64).fract() == 0.0 { if assign1550_e1711 == 0.0 { 0.0 } else { (assign1550_e1711 * ((assign1550_e1709).powf(assign1550_e1711 - 1.0) * (locals.var_vgzeb_t_dn1 * locals.var_inv_vgzeb_tr))) } } else { (assign1550_e1712 * (assign1550_e1711 * ((locals.var_vgzeb_t_dn1 * locals.var_inv_vgzeb_tr) / assign1550_e1709))) };
        locals.var_x_dn3 = if 0.0 == 0.0 && ((assign1550_e1711) as f64).is_finite() && ((assign1550_e1711) as f64).fract() == 0.0 { if assign1550_e1711 == 0.0 { 0.0 } else { (assign1550_e1711 * ((assign1550_e1709).powf(assign1550_e1711 - 1.0) * (locals.var_vgzeb_t_dn3 * locals.var_inv_vgzeb_tr))) } } else { (assign1550_e1712 * (assign1550_e1711 * ((locals.var_vgzeb_t_dn3 * locals.var_inv_vgzeb_tr) / assign1550_e1709))) };
        locals.var_x_dn4 = if 0.0 == 0.0 && ((assign1550_e1711) as f64).is_finite() && ((assign1550_e1711) as f64).fract() == 0.0 { if assign1550_e1711 == 0.0 { 0.0 } else { (assign1550_e1711 * ((assign1550_e1709).powf(assign1550_e1711 - 1.0) * (locals.var_vgzeb_t_dn4 * locals.var_inv_vgzeb_tr))) } } else { (assign1550_e1712 * (assign1550_e1711 * ((locals.var_vgzeb_t_dn4 * locals.var_inv_vgzeb_tr) / assign1550_e1709))) };
        locals.var_x_dn5 = if 0.0 == 0.0 && ((assign1550_e1711) as f64).is_finite() && ((assign1550_e1711) as f64).fract() == 0.0 { if assign1550_e1711 == 0.0 { 0.0 } else { (assign1550_e1711 * ((assign1550_e1709).powf(assign1550_e1711 - 1.0) * (locals.var_vgzeb_t_dn5 * locals.var_inv_vgzeb_tr))) } } else { (assign1550_e1712 * (assign1550_e1711 * ((locals.var_vgzeb_t_dn5 * locals.var_inv_vgzeb_tr) / assign1550_e1709))) };
        locals.var_x_dn6 = if 0.0 == 0.0 && ((assign1550_e1711) as f64).is_finite() && ((assign1550_e1711) as f64).fract() == 0.0 { if assign1550_e1711 == 0.0 { 0.0 } else { (assign1550_e1711 * ((assign1550_e1709).powf(assign1550_e1711 - 1.0) * (locals.var_vgzeb_t_dn6 * locals.var_inv_vgzeb_tr))) } } else { (assign1550_e1712 * (assign1550_e1711 * ((locals.var_vgzeb_t_dn6 * locals.var_inv_vgzeb_tr) / assign1550_e1709))) };
        locals.var_x_dn7 = if 0.0 == 0.0 && ((assign1550_e1711) as f64).is_finite() && ((assign1550_e1711) as f64).fract() == 0.0 { if assign1550_e1711 == 0.0 { 0.0 } else { (assign1550_e1711 * ((assign1550_e1709).powf(assign1550_e1711 - 1.0) * (locals.var_vgzeb_t_dn7 * locals.var_inv_vgzeb_tr))) } } else { (assign1550_e1712 * (assign1550_e1711 * ((locals.var_vgzeb_t_dn7 * locals.var_inv_vgzeb_tr) / assign1550_e1709))) };
        locals.var_x_dn8 = if 0.0 == 0.0 && ((assign1550_e1711) as f64).is_finite() && ((assign1550_e1711) as f64).fract() == 0.0 { if assign1550_e1711 == 0.0 { 0.0 } else { (assign1550_e1711 * ((assign1550_e1709).powf(assign1550_e1711 - 1.0) * (locals.var_vgzeb_t_dn8 * locals.var_inv_vgzeb_tr))) } } else { (assign1550_e1712 * (assign1550_e1711 * ((locals.var_vgzeb_t_dn8 * locals.var_inv_vgzeb_tr) / assign1550_e1709))) };
        locals.var_x_dn9 = if 0.0 == 0.0 && ((assign1550_e1711) as f64).is_finite() && ((assign1550_e1711) as f64).fract() == 0.0 { if assign1550_e1711 == 0.0 { 0.0 } else { (assign1550_e1711 * ((assign1550_e1709).powf(assign1550_e1711 - 1.0) * (locals.var_vgzeb_t_dn9 * locals.var_inv_vgzeb_tr))) } } else { (assign1550_e1712 * (assign1550_e1711 * ((locals.var_vgzeb_t_dn9 * locals.var_inv_vgzeb_tr) / assign1550_e1709))) };

        let assign1560_e1715: f64 = (1.0 / locals.var_cje_t_div_cje);
        locals.var_y = assign1560_e1715;
        locals.var_y_dn0 = (-(locals.var_cje_t_div_cje_dn0 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn1 = (-(locals.var_cje_t_div_cje_dn1 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn3 = (-(locals.var_cje_t_div_cje_dn3 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn4 = (-(locals.var_cje_t_div_cje_dn4 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn5 = (-(locals.var_cje_t_div_cje_dn5 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn6 = (-(locals.var_cje_t_div_cje_dn6 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn7 = (-(locals.var_cje_t_div_cje_dn7 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn8 = (-(locals.var_cje_t_div_cje_dn8 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn9 = (-(locals.var_cje_t_div_cje_dn9 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));

        let assign1570_e1718: f64 = (p.p34 * locals.var_vgzeb_t);
        let assign1570_e1720: f64 = (assign1570_e1718 * locals.var_vgzeb_t);
        let assign1570_e1722: f64 = (assign1570_e1720 * locals.var_x);
        let assign1570_e1724: f64 = (assign1570_e1722 * locals.var_y);
        let assign1570_e1726: f64 = (assign1570_e1724 * p.p65);
        let assign1570_e1728: f64 = (assign1570_e1726 * locals.var_inv_vde_t);
        let assign1570_e1730: f64 = (assign1570_e1728 * locals.var_inv_vgzeb_tr);
        let assign1570_e1732: f64 = (assign1570_e1730 * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t = assign1570_e1732;
        locals.var_nzeb_t_dn0 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn0) * locals.var_vgzeb_t) + (assign1570_e1718 * locals.var_vgzeb_t_dn0)) * locals.var_x) + (assign1570_e1720 * locals.var_x_dn0)) * locals.var_y) + (assign1570_e1722 * locals.var_y_dn0)) * p.p65) * locals.var_inv_vde_t) + (assign1570_e1726 * locals.var_inv_vde_t_dn0)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn1 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn1) * locals.var_vgzeb_t) + (assign1570_e1718 * locals.var_vgzeb_t_dn1)) * locals.var_x) + (assign1570_e1720 * locals.var_x_dn1)) * locals.var_y) + (assign1570_e1722 * locals.var_y_dn1)) * p.p65) * locals.var_inv_vde_t) + (assign1570_e1726 * locals.var_inv_vde_t_dn1)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn3 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn3) * locals.var_vgzeb_t) + (assign1570_e1718 * locals.var_vgzeb_t_dn3)) * locals.var_x) + (assign1570_e1720 * locals.var_x_dn3)) * locals.var_y) + (assign1570_e1722 * locals.var_y_dn3)) * p.p65) * locals.var_inv_vde_t) + (assign1570_e1726 * locals.var_inv_vde_t_dn3)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn4 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn4) * locals.var_vgzeb_t) + (assign1570_e1718 * locals.var_vgzeb_t_dn4)) * locals.var_x) + (assign1570_e1720 * locals.var_x_dn4)) * locals.var_y) + (assign1570_e1722 * locals.var_y_dn4)) * p.p65) * locals.var_inv_vde_t) + (assign1570_e1726 * locals.var_inv_vde_t_dn4)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn5 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn5) * locals.var_vgzeb_t) + (assign1570_e1718 * locals.var_vgzeb_t_dn5)) * locals.var_x) + (assign1570_e1720 * locals.var_x_dn5)) * locals.var_y) + (assign1570_e1722 * locals.var_y_dn5)) * p.p65) * locals.var_inv_vde_t) + (assign1570_e1726 * locals.var_inv_vde_t_dn5)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn6 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn6) * locals.var_vgzeb_t) + (assign1570_e1718 * locals.var_vgzeb_t_dn6)) * locals.var_x) + (assign1570_e1720 * locals.var_x_dn6)) * locals.var_y) + (assign1570_e1722 * locals.var_y_dn6)) * p.p65) * locals.var_inv_vde_t) + (assign1570_e1726 * locals.var_inv_vde_t_dn6)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn7 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn7) * locals.var_vgzeb_t) + (assign1570_e1718 * locals.var_vgzeb_t_dn7)) * locals.var_x) + (assign1570_e1720 * locals.var_x_dn7)) * locals.var_y) + (assign1570_e1722 * locals.var_y_dn7)) * p.p65) * locals.var_inv_vde_t) + (assign1570_e1726 * locals.var_inv_vde_t_dn7)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn8 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn8) * locals.var_vgzeb_t) + (assign1570_e1718 * locals.var_vgzeb_t_dn8)) * locals.var_x) + (assign1570_e1720 * locals.var_x_dn8)) * locals.var_y) + (assign1570_e1722 * locals.var_y_dn8)) * p.p65) * locals.var_inv_vde_t) + (assign1570_e1726 * locals.var_inv_vde_t_dn8)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn9 = ((((((((((((p.p34 * locals.var_vgzeb_t_dn9) * locals.var_vgzeb_t) + (assign1570_e1718 * locals.var_vgzeb_t_dn9)) * locals.var_x) + (assign1570_e1720 * locals.var_x_dn9)) * locals.var_y) + (assign1570_e1722 * locals.var_y_dn9)) * p.p65) * locals.var_inv_vde_t) + (assign1570_e1726 * locals.var_inv_vde_t_dn9)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);

        let assign1590_e1753: f64 = (1.0 / locals.var_vdc_zener_t);
        locals.var_inv_vdc_zener_t = assign1590_e1753;

        let assign1600_e1756: f64 = (locals.var_vgzcb_t * locals.var_inv_vgzcb_tr);
        let assign1600_e1758: f64 = (-0.5);
        let assign1600_e1759: f64 = (assign1600_e1756).powf(assign1600_e1758);
        locals.var_xx = assign1600_e1759;

        let assign1610_e1762: f64 = (1.0 / locals.var_cjc_t_div_cjc_zener);
        locals.var_yy = assign1610_e1762;

        let assign1620_e1765: f64 = (p.p36 * locals.var_vgzcb_t);
        let assign1620_e1767: f64 = (assign1620_e1765 * locals.var_vgzcb_t);
        let assign1620_e1769: f64 = (assign1620_e1767 * locals.var_xx);
        let assign1620_e1771: f64 = (assign1620_e1769 * locals.var_yy);
        let assign1620_e1773: f64 = (assign1620_e1771 * locals.var_vdc_zener);
        let assign1620_e1775: f64 = (assign1620_e1773 * locals.var_inv_vdc_zener_t);
        let assign1620_e1777: f64 = (assign1620_e1775 * locals.var_inv_vgzcb_tr);
        let assign1620_e1779: f64 = (assign1620_e1777 * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t = assign1620_e1779;

        let assign1640_e1800: f64 = (locals.var_lntn * p.p95);
        let assign1640_e1801: f64 = (assign1640_e1800).exp();
        locals.var_x = assign1640_e1801;
        locals.var_x_dn0 = 0.0;
        locals.var_x_dn1 = 0.0;
        locals.var_x_dn3 = 0.0;
        locals.var_x_dn4 = 0.0;
        locals.var_x_dn5 = 0.0;
        locals.var_x_dn6 = 0.0;
        locals.var_x_dn7 = 0.0;
        locals.var_x_dn8 = 0.0;
        locals.var_x_dn9 = 0.0;

        let assign1650_e1804: f64 = (p.p13 * locals.var_x);
        let assign1650_e1806: f64 = (assign1650_e1804 * locals.var_cjc_scale_inv);
        locals.var_vef_t = assign1650_e1806;
        locals.var_vef_t_dn0 = (((p.p13 * locals.var_x_dn0) * locals.var_cjc_scale_inv) + (assign1650_e1804 * locals.var_cjc_scale_inv_dn0));
        locals.var_vef_t_dn1 = (((p.p13 * locals.var_x_dn1) * locals.var_cjc_scale_inv) + (assign1650_e1804 * locals.var_cjc_scale_inv_dn1));
        locals.var_vef_t_dn3 = (((p.p13 * locals.var_x_dn3) * locals.var_cjc_scale_inv) + (assign1650_e1804 * locals.var_cjc_scale_inv_dn3));
        locals.var_vef_t_dn4 = (((p.p13 * locals.var_x_dn4) * locals.var_cjc_scale_inv) + (assign1650_e1804 * locals.var_cjc_scale_inv_dn4));
        locals.var_vef_t_dn5 = (((p.p13 * locals.var_x_dn5) * locals.var_cjc_scale_inv) + (assign1650_e1804 * locals.var_cjc_scale_inv_dn5));
        locals.var_vef_t_dn6 = (((p.p13 * locals.var_x_dn6) * locals.var_cjc_scale_inv) + (assign1650_e1804 * locals.var_cjc_scale_inv_dn6));
        locals.var_vef_t_dn7 = (((p.p13 * locals.var_x_dn7) * locals.var_cjc_scale_inv) + (assign1650_e1804 * locals.var_cjc_scale_inv_dn7));
        locals.var_vef_t_dn8 = (((p.p13 * locals.var_x_dn8) * locals.var_cjc_scale_inv) + (assign1650_e1804 * locals.var_cjc_scale_inv_dn8));
        locals.var_vef_t_dn9 = (((p.p13 * locals.var_x_dn9) * locals.var_cjc_scale_inv) + (assign1650_e1804 * locals.var_cjc_scale_inv_dn9));

    }

    pub(super) fn stamp_transient_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let assign1660_e1809: f64 = (p.p12 * locals.var_x);
        let assign1660_e1811: f64 = (assign1660_e1809 * locals.var_y);
        locals.var_ver_t = assign1660_e1811;
        locals.var_ver_t_dn0 = (((p.p12 * locals.var_x_dn0) * locals.var_y) + (assign1660_e1809 * locals.var_y_dn0));
        locals.var_ver_t_dn1 = (((p.p12 * locals.var_x_dn1) * locals.var_y) + (assign1660_e1809 * locals.var_y_dn1));
        locals.var_ver_t_dn3 = (((p.p12 * locals.var_x_dn3) * locals.var_y) + (assign1660_e1809 * locals.var_y_dn3));
        locals.var_ver_t_dn4 = (((p.p12 * locals.var_x_dn4) * locals.var_y) + (assign1660_e1809 * locals.var_y_dn4));
        locals.var_ver_t_dn5 = (((p.p12 * locals.var_x_dn5) * locals.var_y) + (assign1660_e1809 * locals.var_y_dn5));
        locals.var_ver_t_dn6 = (((p.p12 * locals.var_x_dn6) * locals.var_y) + (assign1660_e1809 * locals.var_y_dn6));
        locals.var_ver_t_dn7 = (((p.p12 * locals.var_x_dn7) * locals.var_y) + (assign1660_e1809 * locals.var_y_dn7));
        locals.var_ver_t_dn8 = (((p.p12 * locals.var_x_dn8) * locals.var_y) + (assign1660_e1809 * locals.var_y_dn8));
        locals.var_ver_t_dn9 = (((p.p12 * locals.var_x_dn9) * locals.var_y) + (assign1660_e1809 * locals.var_y_dn9));

        let assign1670_e1816: f64 = (p.p97 - 2.0);
        let assign1670_e1817: f64 = (locals.var_lntn * assign1670_e1816);
        let assign1670_e1818: f64 = (assign1670_e1817).exp();
        let assign1670_e1819: f64 = (p.p85 * assign1670_e1818);
        let assign1670_e1821: f64 = (-p.p119);
        let assign1670_e1823: f64 = (assign1670_e1821 * locals.var_vdtinv);
        let assign1670_e1824: f64 = (assign1670_e1823).exp();
        let assign1670_e1825: f64 = (assign1670_e1819 * assign1670_e1824);
        locals.var_taue_t = assign1670_e1825;

        let assign1680_e1830: f64 = (p.p95 + p.p97);
        let assign1680_e1832: f64 = (assign1680_e1830 - 1.0);
        let assign1680_e1833: f64 = (locals.var_lntn * assign1680_e1832);
        let assign1680_e1834: f64 = (assign1680_e1833).exp();
        let assign1680_e1835: f64 = (p.p86 * assign1680_e1834);
        locals.var_taub_t = assign1680_e1835;

        let assign1690_e1840: f64 = (p.p98 - 1.0);
        let assign1690_e1841: f64 = (locals.var_lntn * assign1690_e1840);
        let assign1690_e1842: f64 = (assign1690_e1841).exp();
        let assign1690_e1843: f64 = (p.p87 * assign1690_e1842);
        locals.var_tepi_t = assign1690_e1843;

        let assign1700_e1847: f64 = (locals.var_taub_t + locals.var_tepi_t);
        let assign1700_e1848: f64 = (p.p88 * assign1700_e1847);
        let assign1700_e1851: f64 = (p.p86 + p.p87);
        let assign1700_e1852: f64 = (assign1700_e1848 / assign1700_e1851);
        locals.var_taur_t = assign1700_e1852;

        let assign1710_e1857: f64 = (p.p99 - 1.0);
        let assign1710_e1858: f64 = (locals.var_lntn * assign1710_e1857);
        let assign1710_e1859: f64 = (assign1710_e1858).exp();
        let assign1710_e1860: f64 = (p.p89 * assign1710_e1859);
        locals.var_tauex_t = assign1710_e1860;

        let assign1720_e1863: f64 = (locals.var_tk - 300.0);
        locals.var_tk300 = assign1720_e1863;

        let assign1730_e1866: f64 = if locals.var_tk < 525.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign1730_e1866;

        let (assign1740_e1882,) = {
    if (locals.var_guard21 != 0.0) {
        let assign1740_e1872: f64 = (0.00072 * locals.var_tk300);
        let assign1740_e1873: f64 = (1.0 + assign1740_e1872);
        let assign1740_e1876: f64 = (1.6e-6 * locals.var_tk300);
        let assign1740_e1878: f64 = (assign1740_e1876 * locals.var_tk300);
        let assign1740_e1879: f64 = (assign1740_e1873 - assign1740_e1878);
        let assign1740_e1880: f64 = (locals.var_bn * assign1740_e1879);
        (assign1740_e1880,)
    } else {
        (locals.var_bnt,)
    }
};
        locals.var_bnt = assign1740_e1882;

        let (assign1750_e1889,) = {
    if (locals.var_guard21 == 0.0) {
        let assign1750_e1887: f64 = (locals.var_bn * 1.081);
        (assign1750_e1887,)
    } else {
        (locals.var_bnt,)
    }
};
        locals.var_bnt = assign1750_e1889;

        let assign1760_e1893: f64 = (locals.var_lntn * p.p95);
        let assign1760_e1894: f64 = (assign1760_e1893).exp();
        let assign1760_e1895: f64 = (p.p91 * assign1760_e1894);
        locals.var_deg_t = assign1760_e1895;

        let assign1920_e1967: f64 = (p.p3 * (nv5 - nv6));
        locals.var_vb2c1 = assign1920_e1967;
        locals.var_vb2c1_dn5 = p.p3;
        locals.var_vb2c1_dn6 = (-p.p3);

        let assign1930_e1970: f64 = (p.p3 * (nv5 - nv7));
        locals.var_vb2c2 = assign1930_e1970;
        locals.var_vb2c2_dn5 = p.p3;
        locals.var_vb2c2_dn7 = (-p.p3);

        let assign1940_e1973: f64 = (p.p3 * (nv5 - nv3));
        locals.var_vb2e1 = assign1940_e1973;
        locals.var_vb2e1_dn3 = (-p.p3);
        locals.var_vb2e1_dn5 = p.p3;

        let assign1950_e1976: f64 = (p.p3 * (nv4 - nv3));
        locals.var_vb1e1 = assign1950_e1976;
        locals.var_vb1e1_dn3 = (-p.p3);
        locals.var_vb1e1_dn4 = p.p3;

        let assign1960_e1979: f64 = (p.p3 * (nv4 - nv5));
        locals.var_vb1b2 = assign1960_e1979;
        locals.var_vb1b2_dn4 = p.p3;
        locals.var_vb1b2_dn5 = (-p.p3);

        let assign1970_e1982: f64 = (p.p3 * (nv6 - nv7));
        locals.var_vc1c2 = assign1970_e1982;
        locals.var_vc1c2_dn6 = p.p3;
        locals.var_vc1c2_dn7 = (-p.p3);

        let assign1990_e1988: f64 = (p.p3 * (nv1 - nv4));
        locals.var_vbb1 = assign1990_e1988;
        locals.var_vbb1_dn1 = p.p3;
        locals.var_vbb1_dn4 = (-p.p3);

        let assign2000_e1991: f64 = (p.p3 * (nv1 - nv2));
        locals.var_vbe = assign2000_e1991;
        locals.var_vbe_dn1 = p.p3;
        locals.var_vbe_dn2 = (-p.p3);

        let assign2010_e1994: f64 = (p.p3 * (nv1 - nv0));
        locals.var_vbc = assign2010_e1994;
        locals.var_vbc_dn0 = (-p.p3);
        locals.var_vbc_dn1 = p.p3;

        let assign2020_e1997: f64 = (p.p3 * (nv9 - nv6));
        locals.var_vc4c1 = assign2020_e1997;
        locals.var_vc4c1_dn6 = (-p.p3);
        locals.var_vc4c1_dn9 = p.p3;

        let assign2030_e2000: f64 = (p.p3 * (nv8 - nv9));
        locals.var_vc3c4 = assign2030_e2000;
        locals.var_vc3c4_dn8 = p.p3;
        locals.var_vc3c4_dn9 = (-p.p3);

        let assign2040_e2003: f64 = (locals.var_vb1b2 + locals.var_vb2c2);
        let assign2040_e2005: f64 = (assign2040_e2003 - locals.var_vc1c2);
        let assign2040_e2007: f64 = (assign2040_e2005 - locals.var_vc4c1);
        locals.var_vb1c4 = assign2040_e2007;
        locals.var_vb1c4_dn4 = locals.var_vb1b2_dn4;
        locals.var_vb1c4_dn5 = (locals.var_vb1b2_dn5 + locals.var_vb2c2_dn5);
        locals.var_vb1c4_dn6 = ((-locals.var_vc1c2_dn6) - locals.var_vc4c1_dn6);
        locals.var_vb1c4_dn7 = (locals.var_vb2c2_dn7 - locals.var_vc1c2_dn7);
        locals.var_vb1c4_dn9 = (-locals.var_vc4c1_dn9);

        let assign2050_e2009: f64 = (-locals.var_vbc);
        let assign2050_e2011: f64 = (assign2050_e2009 + locals.var_vbb1);
        let assign2050_e2013: f64 = (assign2050_e2011 + locals.var_vb1c4);
        let assign2050_e2015: f64 = (assign2050_e2013 - locals.var_vc3c4);
        locals.var_vcc3 = assign2050_e2015;
        locals.var_vcc3_dn0 = (-locals.var_vbc_dn0);
        locals.var_vcc3_dn1 = ((-locals.var_vbc_dn1) + locals.var_vbb1_dn1);
        locals.var_vcc3_dn4 = (locals.var_vbb1_dn4 + locals.var_vb1c4_dn4);
        locals.var_vcc3_dn5 = locals.var_vb1c4_dn5;
        locals.var_vcc3_dn6 = locals.var_vb1c4_dn6;
        locals.var_vcc3_dn7 = locals.var_vb1c4_dn7;
        locals.var_vcc3_dn8 = (-locals.var_vc3c4_dn8);
        locals.var_vcc3_dn9 = (locals.var_vb1c4_dn9 - locals.var_vc3c4_dn9);

        let assign2060_e2018: f64 = (locals.var_vbc + locals.var_vcc3);
        locals.var_vbc3 = assign2060_e2018;
        locals.var_vbc3_dn0 = (locals.var_vbc_dn0 + locals.var_vcc3_dn0);
        locals.var_vbc3_dn1 = (locals.var_vbc_dn1 + locals.var_vcc3_dn1);
        locals.var_vbc3_dn4 = locals.var_vcc3_dn4;
        locals.var_vbc3_dn5 = locals.var_vcc3_dn5;
        locals.var_vbc3_dn6 = locals.var_vcc3_dn6;
        locals.var_vbc3_dn7 = locals.var_vcc3_dn7;
        locals.var_vbc3_dn8 = locals.var_vcc3_dn8;
        locals.var_vbc3_dn9 = locals.var_vcc3_dn9;

        let assign2070_e2021: f64 = (locals.var_vb2c2 * locals.var_vtinv);
        let assign2070_e2023: f64 = if assign2070_e2021 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard28 = assign2070_e2023;

        let (assign2080_e2030, assign2080_e2030_d_n5, assign2080_e2030_d_n7,) = {
    if (locals.var_guard28 != 0.0) {
        let assign2080_e2027: f64 = (locals.var_vb2c2 * locals.var_vtinv);
        let assign2080_e2028: f64 = (assign2080_e2027).exp();
        (assign2080_e2028, (assign2080_e2028 * (locals.var_vb2c2_dn5 * locals.var_vtinv)), (assign2080_e2028 * (locals.var_vb2c2_dn7 * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c2, locals.var_evb2c2_dn5, locals.var_evb2c2_dn7,)
    }
};
        locals.var_evb2c2 = assign2080_e2030;
        locals.var_evb2c2_dn5 = assign2080_e2030_d_n5;
        locals.var_evb2c2_dn7 = assign2080_e2030_d_n7;

        let (assign2090_e2036,) = {
    if (locals.var_guard28 == 0.0) {
        let assign2090_e2034: f64 = (p.p134).exp();
        (assign2090_e2034,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2090_e2036;

        let (assign2100_e2049, assign2100_e2049_d_n5, assign2100_e2049_d_n7,) = {
    if (locals.var_guard28 == 0.0) {
        let assign2100_e2043: f64 = (locals.var_vb2c2 * locals.var_vtinv);
        let assign2100_e2045: f64 = (assign2100_e2043 - p.p134);
        let assign2100_e2046: f64 = (1.0 + assign2100_e2045);
        let assign2100_e2047: f64 = (locals.var_expl * assign2100_e2046);
        (assign2100_e2047, (locals.var_expl * (locals.var_vb2c2_dn5 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb2c2_dn7 * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c2, locals.var_evb2c2_dn5, locals.var_evb2c2_dn7,)
    }
};
        locals.var_evb2c2 = assign2100_e2049;
        locals.var_evb2c2_dn5 = assign2100_e2049_d_n5;
        locals.var_evb2c2_dn7 = assign2100_e2049_d_n7;

        let assign2110_e2052: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign2110_e2054: f64 = (assign2110_e2052 / locals.var_nff_t);
        let assign2110_e2056: f64 = if assign2110_e2054 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign2110_e2056;

        let (assign2120_e2065, assign2120_e2065_d_n0, assign2120_e2065_d_n1, assign2120_e2065_d_n3, assign2120_e2065_d_n4, assign2120_e2065_d_n5, assign2120_e2065_d_n6, assign2120_e2065_d_n7, assign2120_e2065_d_n8, assign2120_e2065_d_n9,) = {
    if (locals.var_guard29 != 0.0) {
        let assign2120_e2060: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign2120_e2062: f64 = (assign2120_e2060 / locals.var_nff_t);
        let assign2120_e2063: f64 = (assign2120_e2062).exp();
        (assign2120_e2063, (assign2120_e2063 * (-((assign2120_e2060 * locals.var_nff_t_dn0) / (locals.var_nff_t * locals.var_nff_t)))), (assign2120_e2063 * (-((assign2120_e2060 * locals.var_nff_t_dn1) / (locals.var_nff_t * locals.var_nff_t)))), (assign2120_e2063 * ((((locals.var_vb2e1_dn3 * locals.var_vtinv) * locals.var_nff_t) - (assign2120_e2060 * locals.var_nff_t_dn3)) / (locals.var_nff_t * locals.var_nff_t))), (assign2120_e2063 * (-((assign2120_e2060 * locals.var_nff_t_dn4) / (locals.var_nff_t * locals.var_nff_t)))), (assign2120_e2063 * ((((locals.var_vb2e1_dn5 * locals.var_vtinv) * locals.var_nff_t) - (assign2120_e2060 * locals.var_nff_t_dn5)) / (locals.var_nff_t * locals.var_nff_t))), (assign2120_e2063 * (-((assign2120_e2060 * locals.var_nff_t_dn6) / (locals.var_nff_t * locals.var_nff_t)))), (assign2120_e2063 * (-((assign2120_e2060 * locals.var_nff_t_dn7) / (locals.var_nff_t * locals.var_nff_t)))), (assign2120_e2063 * (-((assign2120_e2060 * locals.var_nff_t_dn8) / (locals.var_nff_t * locals.var_nff_t)))), (assign2120_e2063 * (-((assign2120_e2060 * locals.var_nff_t_dn9) / (locals.var_nff_t * locals.var_nff_t)))),)
    } else {
        (locals.var_evb2e1, locals.var_evb2e1_dn0, locals.var_evb2e1_dn1, locals.var_evb2e1_dn3, locals.var_evb2e1_dn4, locals.var_evb2e1_dn5, locals.var_evb2e1_dn6, locals.var_evb2e1_dn7, locals.var_evb2e1_dn8, locals.var_evb2e1_dn9,)
    }
};
        locals.var_evb2e1 = assign2120_e2065;
        locals.var_evb2e1_dn0 = assign2120_e2065_d_n0;
        locals.var_evb2e1_dn1 = assign2120_e2065_d_n1;
        locals.var_evb2e1_dn3 = assign2120_e2065_d_n3;
        locals.var_evb2e1_dn4 = assign2120_e2065_d_n4;
        locals.var_evb2e1_dn5 = assign2120_e2065_d_n5;
        locals.var_evb2e1_dn6 = assign2120_e2065_d_n6;
        locals.var_evb2e1_dn7 = assign2120_e2065_d_n7;
        locals.var_evb2e1_dn8 = assign2120_e2065_d_n8;
        locals.var_evb2e1_dn9 = assign2120_e2065_d_n9;

        let (assign2130_e2071,) = {
    if (locals.var_guard29 == 0.0) {
        let assign2130_e2069: f64 = (p.p134).exp();
        (assign2130_e2069,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2130_e2071;

        let (assign2140_e2086, assign2140_e2086_d_n0, assign2140_e2086_d_n1, assign2140_e2086_d_n3, assign2140_e2086_d_n4, assign2140_e2086_d_n5, assign2140_e2086_d_n6, assign2140_e2086_d_n7, assign2140_e2086_d_n8, assign2140_e2086_d_n9,) = {
    if (locals.var_guard29 == 0.0) {
        let assign2140_e2078: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign2140_e2080: f64 = (assign2140_e2078 / locals.var_nff_t);
        let assign2140_e2082: f64 = (assign2140_e2080 - p.p134);
        let assign2140_e2083: f64 = (1.0 + assign2140_e2082);
        let assign2140_e2084: f64 = (locals.var_expl * assign2140_e2083);
        (assign2140_e2084, (locals.var_expl * (-((assign2140_e2078 * locals.var_nff_t_dn0) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * (-((assign2140_e2078 * locals.var_nff_t_dn1) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * ((((locals.var_vb2e1_dn3 * locals.var_vtinv) * locals.var_nff_t) - (assign2140_e2078 * locals.var_nff_t_dn3)) / (locals.var_nff_t * locals.var_nff_t))), (locals.var_expl * (-((assign2140_e2078 * locals.var_nff_t_dn4) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * ((((locals.var_vb2e1_dn5 * locals.var_vtinv) * locals.var_nff_t) - (assign2140_e2078 * locals.var_nff_t_dn5)) / (locals.var_nff_t * locals.var_nff_t))), (locals.var_expl * (-((assign2140_e2078 * locals.var_nff_t_dn6) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * (-((assign2140_e2078 * locals.var_nff_t_dn7) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * (-((assign2140_e2078 * locals.var_nff_t_dn8) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * (-((assign2140_e2078 * locals.var_nff_t_dn9) / (locals.var_nff_t * locals.var_nff_t)))),)
    } else {
        (locals.var_evb2e1, locals.var_evb2e1_dn0, locals.var_evb2e1_dn1, locals.var_evb2e1_dn3, locals.var_evb2e1_dn4, locals.var_evb2e1_dn5, locals.var_evb2e1_dn6, locals.var_evb2e1_dn7, locals.var_evb2e1_dn8, locals.var_evb2e1_dn9,)
    }
};
        locals.var_evb2e1 = assign2140_e2086;
        locals.var_evb2e1_dn0 = assign2140_e2086_d_n0;
        locals.var_evb2e1_dn1 = assign2140_e2086_d_n1;
        locals.var_evb2e1_dn3 = assign2140_e2086_d_n3;
        locals.var_evb2e1_dn4 = assign2140_e2086_d_n4;
        locals.var_evb2e1_dn5 = assign2140_e2086_d_n5;
        locals.var_evb2e1_dn6 = assign2140_e2086_d_n6;
        locals.var_evb2e1_dn7 = assign2140_e2086_d_n7;
        locals.var_evb2e1_dn8 = assign2140_e2086_d_n8;
        locals.var_evb2e1_dn9 = assign2140_e2086_d_n9;

        let assign2150_e2089: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign2150_e2091: f64 = if assign2150_e2089 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign2150_e2091;

        let (assign2160_e2098, assign2160_e2098_d_n4, assign2160_e2098_d_n5, assign2160_e2098_d_n6, assign2160_e2098_d_n7, assign2160_e2098_d_n9,) = {
    if (locals.var_guard30 != 0.0) {
        let assign2160_e2095: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign2160_e2096: f64 = (assign2160_e2095).exp();
        (assign2160_e2096, (assign2160_e2096 * (locals.var_vb1c4_dn4 * locals.var_vtinv)), (assign2160_e2096 * (locals.var_vb1c4_dn5 * locals.var_vtinv)), (assign2160_e2096 * (locals.var_vb1c4_dn6 * locals.var_vtinv)), (assign2160_e2096 * (locals.var_vb1c4_dn7 * locals.var_vtinv)), (assign2160_e2096 * (locals.var_vb1c4_dn9 * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4, locals.var_evb1c4_dn4, locals.var_evb1c4_dn5, locals.var_evb1c4_dn6, locals.var_evb1c4_dn7, locals.var_evb1c4_dn9,)
    }
};
        locals.var_evb1c4 = assign2160_e2098;
        locals.var_evb1c4_dn4 = assign2160_e2098_d_n4;
        locals.var_evb1c4_dn5 = assign2160_e2098_d_n5;
        locals.var_evb1c4_dn6 = assign2160_e2098_d_n6;
        locals.var_evb1c4_dn7 = assign2160_e2098_d_n7;
        locals.var_evb1c4_dn9 = assign2160_e2098_d_n9;

        let (assign2170_e2104,) = {
    if (locals.var_guard30 == 0.0) {
        let assign2170_e2102: f64 = (p.p134).exp();
        (assign2170_e2102,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2170_e2104;

        let (assign2180_e2117, assign2180_e2117_d_n4, assign2180_e2117_d_n5, assign2180_e2117_d_n6, assign2180_e2117_d_n7, assign2180_e2117_d_n9,) = {
    if (locals.var_guard30 == 0.0) {
        let assign2180_e2111: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign2180_e2113: f64 = (assign2180_e2111 - p.p134);
        let assign2180_e2114: f64 = (1.0 + assign2180_e2113);
        let assign2180_e2115: f64 = (locals.var_expl * assign2180_e2114);
        (assign2180_e2115, (locals.var_expl * (locals.var_vb1c4_dn4 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb1c4_dn5 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb1c4_dn6 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb1c4_dn7 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb1c4_dn9 * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4, locals.var_evb1c4_dn4, locals.var_evb1c4_dn5, locals.var_evb1c4_dn6, locals.var_evb1c4_dn7, locals.var_evb1c4_dn9,)
    }
};
        locals.var_evb1c4 = assign2180_e2117;
        locals.var_evb1c4_dn4 = assign2180_e2117_d_n4;
        locals.var_evb1c4_dn5 = assign2180_e2117_d_n5;
        locals.var_evb1c4_dn6 = assign2180_e2117_d_n6;
        locals.var_evb1c4_dn7 = assign2180_e2117_d_n7;
        locals.var_evb1c4_dn9 = assign2180_e2117_d_n9;

        let assign2190_e2120: f64 = (locals.var_vb1b2 * locals.var_vtinv);
        let assign2190_e2122: f64 = if assign2190_e2120 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign2190_e2122;

        let (assign2210_e2135,) = {
    if (locals.var_guard31 == 0.0) {
        let assign2210_e2133: f64 = (p.p134).exp();
        (assign2210_e2133,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2210_e2135;

        let assign2230_e2151: f64 = (locals.var_vbc3 * locals.var_vtinv);
        let assign2230_e2153: f64 = if assign2230_e2151 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign2230_e2153;

        let (assign2240_e2160, assign2240_e2160_d_n0, assign2240_e2160_d_n1, assign2240_e2160_d_n4, assign2240_e2160_d_n5, assign2240_e2160_d_n6, assign2240_e2160_d_n7, assign2240_e2160_d_n8, assign2240_e2160_d_n9,) = {
    if (locals.var_guard32 != 0.0) {
        let assign2240_e2157: f64 = (locals.var_vbc3 * locals.var_vtinv);
        let assign2240_e2158: f64 = (assign2240_e2157).exp();
        (assign2240_e2158, (assign2240_e2158 * (locals.var_vbc3_dn0 * locals.var_vtinv)), (assign2240_e2158 * (locals.var_vbc3_dn1 * locals.var_vtinv)), (assign2240_e2158 * (locals.var_vbc3_dn4 * locals.var_vtinv)), (assign2240_e2158 * (locals.var_vbc3_dn5 * locals.var_vtinv)), (assign2240_e2158 * (locals.var_vbc3_dn6 * locals.var_vtinv)), (assign2240_e2158 * (locals.var_vbc3_dn7 * locals.var_vtinv)), (assign2240_e2158 * (locals.var_vbc3_dn8 * locals.var_vtinv)), (assign2240_e2158 * (locals.var_vbc3_dn9 * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3, locals.var_evbc3_dn0, locals.var_evbc3_dn1, locals.var_evbc3_dn4, locals.var_evbc3_dn5, locals.var_evbc3_dn6, locals.var_evbc3_dn7, locals.var_evbc3_dn8, locals.var_evbc3_dn9,)
    }
};
        locals.var_evbc3 = assign2240_e2160;
        locals.var_evbc3_dn0 = assign2240_e2160_d_n0;
        locals.var_evbc3_dn1 = assign2240_e2160_d_n1;
        locals.var_evbc3_dn4 = assign2240_e2160_d_n4;
        locals.var_evbc3_dn5 = assign2240_e2160_d_n5;
        locals.var_evbc3_dn6 = assign2240_e2160_d_n6;
        locals.var_evbc3_dn7 = assign2240_e2160_d_n7;
        locals.var_evbc3_dn8 = assign2240_e2160_d_n8;
        locals.var_evbc3_dn9 = assign2240_e2160_d_n9;

        let (assign2250_e2166,) = {
    if (locals.var_guard32 == 0.0) {
        let assign2250_e2164: f64 = (p.p134).exp();
        (assign2250_e2164,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2250_e2166;

        let (assign2260_e2179, assign2260_e2179_d_n0, assign2260_e2179_d_n1, assign2260_e2179_d_n4, assign2260_e2179_d_n5, assign2260_e2179_d_n6, assign2260_e2179_d_n7, assign2260_e2179_d_n8, assign2260_e2179_d_n9,) = {
    if (locals.var_guard32 == 0.0) {
        let assign2260_e2173: f64 = (locals.var_vbc3 * locals.var_vtinv);
        let assign2260_e2175: f64 = (assign2260_e2173 - p.p134);
        let assign2260_e2176: f64 = (1.0 + assign2260_e2175);
        let assign2260_e2177: f64 = (locals.var_expl * assign2260_e2176);
        (assign2260_e2177, (locals.var_expl * (locals.var_vbc3_dn0 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn1 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn4 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn5 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn6 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn7 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn8 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn9 * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3, locals.var_evbc3_dn0, locals.var_evbc3_dn1, locals.var_evbc3_dn4, locals.var_evbc3_dn5, locals.var_evbc3_dn6, locals.var_evbc3_dn7, locals.var_evbc3_dn8, locals.var_evbc3_dn9,)
    }
};
        locals.var_evbc3 = assign2260_e2179;
        locals.var_evbc3_dn0 = assign2260_e2179_d_n0;
        locals.var_evbc3_dn1 = assign2260_e2179_d_n1;
        locals.var_evbc3_dn4 = assign2260_e2179_d_n4;
        locals.var_evbc3_dn5 = assign2260_e2179_d_n5;
        locals.var_evbc3_dn6 = assign2260_e2179_d_n6;
        locals.var_evbc3_dn7 = assign2260_e2179_d_n7;
        locals.var_evbc3_dn8 = assign2260_e2179_d_n8;
        locals.var_evbc3_dn9 = assign2260_e2179_d_n9;

        let assign2270_e2182: f64 = (locals.var_vbc3 - locals.var_vdc_t);
        let assign2270_e2184: f64 = (assign2270_e2182 * locals.var_vtinv);
        let assign2270_e2186: f64 = if assign2270_e2184 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign2270_e2186;

        let (assign2280_e2195, assign2280_e2195_d_n0, assign2280_e2195_d_n1, assign2280_e2195_d_n3, assign2280_e2195_d_n4, assign2280_e2195_d_n5, assign2280_e2195_d_n6, assign2280_e2195_d_n7, assign2280_e2195_d_n8, assign2280_e2195_d_n9,) = {
    if (locals.var_guard33 != 0.0) {
        let assign2280_e2190: f64 = (locals.var_vbc3 - locals.var_vdc_t);
        let assign2280_e2192: f64 = (assign2280_e2190 * locals.var_vtinv);
        let assign2280_e2193: f64 = (assign2280_e2192).exp();
        (assign2280_e2193, (assign2280_e2193 * ((locals.var_vbc3_dn0 - locals.var_vdc_t_dn0) * locals.var_vtinv)), (assign2280_e2193 * ((locals.var_vbc3_dn1 - locals.var_vdc_t_dn1) * locals.var_vtinv)), (assign2280_e2193 * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (assign2280_e2193 * ((locals.var_vbc3_dn4 - locals.var_vdc_t_dn4) * locals.var_vtinv)), (assign2280_e2193 * ((locals.var_vbc3_dn5 - locals.var_vdc_t_dn5) * locals.var_vtinv)), (assign2280_e2193 * ((locals.var_vbc3_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (assign2280_e2193 * ((locals.var_vbc3_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (assign2280_e2193 * ((locals.var_vbc3_dn8 - locals.var_vdc_t_dn8) * locals.var_vtinv)), (assign2280_e2193 * ((locals.var_vbc3_dn9 - locals.var_vdc_t_dn9) * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3vdc, locals.var_evbc3vdc_dn0, locals.var_evbc3vdc_dn1, locals.var_evbc3vdc_dn3, locals.var_evbc3vdc_dn4, locals.var_evbc3vdc_dn5, locals.var_evbc3vdc_dn6, locals.var_evbc3vdc_dn7, locals.var_evbc3vdc_dn8, locals.var_evbc3vdc_dn9,)
    }
};
        locals.var_evbc3vdc = assign2280_e2195;
        locals.var_evbc3vdc_dn0 = assign2280_e2195_d_n0;
        locals.var_evbc3vdc_dn1 = assign2280_e2195_d_n1;
        locals.var_evbc3vdc_dn3 = assign2280_e2195_d_n3;
        locals.var_evbc3vdc_dn4 = assign2280_e2195_d_n4;
        locals.var_evbc3vdc_dn5 = assign2280_e2195_d_n5;
        locals.var_evbc3vdc_dn6 = assign2280_e2195_d_n6;
        locals.var_evbc3vdc_dn7 = assign2280_e2195_d_n7;
        locals.var_evbc3vdc_dn8 = assign2280_e2195_d_n8;
        locals.var_evbc3vdc_dn9 = assign2280_e2195_d_n9;

        let (assign2290_e2201,) = {
    if (locals.var_guard33 == 0.0) {
        let assign2290_e2199: f64 = (p.p134).exp();
        (assign2290_e2199,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2290_e2201;

        let (assign2300_e2216, assign2300_e2216_d_n0, assign2300_e2216_d_n1, assign2300_e2216_d_n3, assign2300_e2216_d_n4, assign2300_e2216_d_n5, assign2300_e2216_d_n6, assign2300_e2216_d_n7, assign2300_e2216_d_n8, assign2300_e2216_d_n9,) = {
    if (locals.var_guard33 == 0.0) {
        let assign2300_e2208: f64 = (locals.var_vbc3 - locals.var_vdc_t);
        let assign2300_e2210: f64 = (assign2300_e2208 * locals.var_vtinv);
        let assign2300_e2212: f64 = (assign2300_e2210 - p.p134);
        let assign2300_e2213: f64 = (1.0 + assign2300_e2212);
        let assign2300_e2214: f64 = (locals.var_expl * assign2300_e2213);
        (assign2300_e2214, (locals.var_expl * ((locals.var_vbc3_dn0 - locals.var_vdc_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn1 - locals.var_vdc_t_dn1) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn4 - locals.var_vdc_t_dn4) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn5 - locals.var_vdc_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn8 - locals.var_vdc_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn9 - locals.var_vdc_t_dn9) * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3vdc, locals.var_evbc3vdc_dn0, locals.var_evbc3vdc_dn1, locals.var_evbc3vdc_dn3, locals.var_evbc3vdc_dn4, locals.var_evbc3vdc_dn5, locals.var_evbc3vdc_dn6, locals.var_evbc3vdc_dn7, locals.var_evbc3vdc_dn8, locals.var_evbc3vdc_dn9,)
    }
};
        locals.var_evbc3vdc = assign2300_e2216;
        locals.var_evbc3vdc_dn0 = assign2300_e2216_d_n0;
        locals.var_evbc3vdc_dn1 = assign2300_e2216_d_n1;
        locals.var_evbc3vdc_dn3 = assign2300_e2216_d_n3;
        locals.var_evbc3vdc_dn4 = assign2300_e2216_d_n4;
        locals.var_evbc3vdc_dn5 = assign2300_e2216_d_n5;
        locals.var_evbc3vdc_dn6 = assign2300_e2216_d_n6;
        locals.var_evbc3vdc_dn7 = assign2300_e2216_d_n7;
        locals.var_evbc3vdc_dn8 = assign2300_e2216_d_n8;
        locals.var_evbc3vdc_dn9 = assign2300_e2216_d_n9;

        let assign2310_e2219: f64 = (locals.var_vb1c4 - locals.var_vdc_t);
        let assign2310_e2221: f64 = (assign2310_e2219 * locals.var_vtinv);
        let assign2310_e2223: f64 = if assign2310_e2221 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign2310_e2223;

        let (assign2320_e2232, assign2320_e2232_d_n0, assign2320_e2232_d_n1, assign2320_e2232_d_n3, assign2320_e2232_d_n4, assign2320_e2232_d_n5, assign2320_e2232_d_n6, assign2320_e2232_d_n7, assign2320_e2232_d_n8, assign2320_e2232_d_n9,) = {
    if (locals.var_guard34 != 0.0) {
        let assign2320_e2227: f64 = (locals.var_vb1c4 - locals.var_vdc_t);
        let assign2320_e2229: f64 = (assign2320_e2227 * locals.var_vtinv);
        let assign2320_e2230: f64 = (assign2320_e2229).exp();
        (assign2320_e2230, (assign2320_e2230 * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (assign2320_e2230 * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (assign2320_e2230 * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (assign2320_e2230 * ((locals.var_vb1c4_dn4 - locals.var_vdc_t_dn4) * locals.var_vtinv)), (assign2320_e2230 * ((locals.var_vb1c4_dn5 - locals.var_vdc_t_dn5) * locals.var_vtinv)), (assign2320_e2230 * ((locals.var_vb1c4_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (assign2320_e2230 * ((locals.var_vb1c4_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (assign2320_e2230 * ((-locals.var_vdc_t_dn8) * locals.var_vtinv)), (assign2320_e2230 * ((locals.var_vb1c4_dn9 - locals.var_vdc_t_dn9) * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4vdc, locals.var_evb1c4vdc_dn0, locals.var_evb1c4vdc_dn1, locals.var_evb1c4vdc_dn3, locals.var_evb1c4vdc_dn4, locals.var_evb1c4vdc_dn5, locals.var_evb1c4vdc_dn6, locals.var_evb1c4vdc_dn7, locals.var_evb1c4vdc_dn8, locals.var_evb1c4vdc_dn9,)
    }
};
        locals.var_evb1c4vdc = assign2320_e2232;
        locals.var_evb1c4vdc_dn0 = assign2320_e2232_d_n0;
        locals.var_evb1c4vdc_dn1 = assign2320_e2232_d_n1;
        locals.var_evb1c4vdc_dn3 = assign2320_e2232_d_n3;
        locals.var_evb1c4vdc_dn4 = assign2320_e2232_d_n4;
        locals.var_evb1c4vdc_dn5 = assign2320_e2232_d_n5;
        locals.var_evb1c4vdc_dn6 = assign2320_e2232_d_n6;
        locals.var_evb1c4vdc_dn7 = assign2320_e2232_d_n7;
        locals.var_evb1c4vdc_dn8 = assign2320_e2232_d_n8;
        locals.var_evb1c4vdc_dn9 = assign2320_e2232_d_n9;

        let (assign2330_e2238,) = {
    if (locals.var_guard34 == 0.0) {
        let assign2330_e2236: f64 = (p.p134).exp();
        (assign2330_e2236,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2330_e2238;

        let (assign2340_e2253, assign2340_e2253_d_n0, assign2340_e2253_d_n1, assign2340_e2253_d_n3, assign2340_e2253_d_n4, assign2340_e2253_d_n5, assign2340_e2253_d_n6, assign2340_e2253_d_n7, assign2340_e2253_d_n8, assign2340_e2253_d_n9,) = {
    if (locals.var_guard34 == 0.0) {
        let assign2340_e2245: f64 = (locals.var_vb1c4 - locals.var_vdc_t);
        let assign2340_e2247: f64 = (assign2340_e2245 * locals.var_vtinv);
        let assign2340_e2249: f64 = (assign2340_e2247 - p.p134);
        let assign2340_e2250: f64 = (1.0 + assign2340_e2249);
        let assign2340_e2251: f64 = (locals.var_expl * assign2340_e2250);
        (assign2340_e2251, (locals.var_expl * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn4 - locals.var_vdc_t_dn4) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn5 - locals.var_vdc_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn9 - locals.var_vdc_t_dn9) * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4vdc, locals.var_evb1c4vdc_dn0, locals.var_evb1c4vdc_dn1, locals.var_evb1c4vdc_dn3, locals.var_evb1c4vdc_dn4, locals.var_evb1c4vdc_dn5, locals.var_evb1c4vdc_dn6, locals.var_evb1c4vdc_dn7, locals.var_evb1c4vdc_dn8, locals.var_evb1c4vdc_dn9,)
    }
};
        locals.var_evb1c4vdc = assign2340_e2253;
        locals.var_evb1c4vdc_dn0 = assign2340_e2253_d_n0;
        locals.var_evb1c4vdc_dn1 = assign2340_e2253_d_n1;
        locals.var_evb1c4vdc_dn3 = assign2340_e2253_d_n3;
        locals.var_evb1c4vdc_dn4 = assign2340_e2253_d_n4;
        locals.var_evb1c4vdc_dn5 = assign2340_e2253_d_n5;
        locals.var_evb1c4vdc_dn6 = assign2340_e2253_d_n6;
        locals.var_evb1c4vdc_dn7 = assign2340_e2253_d_n7;
        locals.var_evb1c4vdc_dn8 = assign2340_e2253_d_n8;
        locals.var_evb1c4vdc_dn9 = assign2340_e2253_d_n9;

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2350_e2256: f64 = (locals.var_vb2c2 - locals.var_vdc_t);
        let assign2350_e2258: f64 = (assign2350_e2256 * locals.var_vtinv);
        let assign2350_e2260: f64 = if assign2350_e2258 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign2350_e2260;

        let (assign2360_e2269, assign2360_e2269_d_n0, assign2360_e2269_d_n1, assign2360_e2269_d_n3, assign2360_e2269_d_n4, assign2360_e2269_d_n5, assign2360_e2269_d_n6, assign2360_e2269_d_n7, assign2360_e2269_d_n8, assign2360_e2269_d_n9,) = {
    if (locals.var_guard35 != 0.0) {
        let assign2360_e2264: f64 = (locals.var_vb2c2 - locals.var_vdc_t);
        let assign2360_e2266: f64 = (assign2360_e2264 * locals.var_vtinv);
        let assign2360_e2267: f64 = (assign2360_e2266).exp();
        (assign2360_e2267, (assign2360_e2267 * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (assign2360_e2267 * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (assign2360_e2267 * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (assign2360_e2267 * ((-locals.var_vdc_t_dn4) * locals.var_vtinv)), (assign2360_e2267 * ((locals.var_vb2c2_dn5 - locals.var_vdc_t_dn5) * locals.var_vtinv)), (assign2360_e2267 * ((-locals.var_vdc_t_dn6) * locals.var_vtinv)), (assign2360_e2267 * ((locals.var_vb2c2_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (assign2360_e2267 * ((-locals.var_vdc_t_dn8) * locals.var_vtinv)), (assign2360_e2267 * ((-locals.var_vdc_t_dn9) * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c2vdc, locals.var_evb2c2vdc_dn0, locals.var_evb2c2vdc_dn1, locals.var_evb2c2vdc_dn3, locals.var_evb2c2vdc_dn4, locals.var_evb2c2vdc_dn5, locals.var_evb2c2vdc_dn6, locals.var_evb2c2vdc_dn7, locals.var_evb2c2vdc_dn8, locals.var_evb2c2vdc_dn9,)
    }
};
        locals.var_evb2c2vdc = assign2360_e2269;
        locals.var_evb2c2vdc_dn0 = assign2360_e2269_d_n0;
        locals.var_evb2c2vdc_dn1 = assign2360_e2269_d_n1;
        locals.var_evb2c2vdc_dn3 = assign2360_e2269_d_n3;
        locals.var_evb2c2vdc_dn4 = assign2360_e2269_d_n4;
        locals.var_evb2c2vdc_dn5 = assign2360_e2269_d_n5;
        locals.var_evb2c2vdc_dn6 = assign2360_e2269_d_n6;
        locals.var_evb2c2vdc_dn7 = assign2360_e2269_d_n7;
        locals.var_evb2c2vdc_dn8 = assign2360_e2269_d_n8;
        locals.var_evb2c2vdc_dn9 = assign2360_e2269_d_n9;

        let (assign2370_e2275,) = {
    if (locals.var_guard35 == 0.0) {
        let assign2370_e2273: f64 = (p.p134).exp();
        (assign2370_e2273,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2370_e2275;

        let (assign2380_e2290, assign2380_e2290_d_n0, assign2380_e2290_d_n1, assign2380_e2290_d_n3, assign2380_e2290_d_n4, assign2380_e2290_d_n5, assign2380_e2290_d_n6, assign2380_e2290_d_n7, assign2380_e2290_d_n8, assign2380_e2290_d_n9,) = {
    if (locals.var_guard35 == 0.0) {
        let assign2380_e2282: f64 = (locals.var_vb2c2 - locals.var_vdc_t);
        let assign2380_e2284: f64 = (assign2380_e2282 * locals.var_vtinv);
        let assign2380_e2286: f64 = (assign2380_e2284 - p.p134);
        let assign2380_e2287: f64 = (1.0 + assign2380_e2286);
        let assign2380_e2288: f64 = (locals.var_expl * assign2380_e2287);
        (assign2380_e2288, (locals.var_expl * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn4) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2c2_dn5 - locals.var_vdc_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2c2_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn9) * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c2vdc, locals.var_evb2c2vdc_dn0, locals.var_evb2c2vdc_dn1, locals.var_evb2c2vdc_dn3, locals.var_evb2c2vdc_dn4, locals.var_evb2c2vdc_dn5, locals.var_evb2c2vdc_dn6, locals.var_evb2c2vdc_dn7, locals.var_evb2c2vdc_dn8, locals.var_evb2c2vdc_dn9,)
    }
};
        locals.var_evb2c2vdc = assign2380_e2290;
        locals.var_evb2c2vdc_dn0 = assign2380_e2290_d_n0;
        locals.var_evb2c2vdc_dn1 = assign2380_e2290_d_n1;
        locals.var_evb2c2vdc_dn3 = assign2380_e2290_d_n3;
        locals.var_evb2c2vdc_dn4 = assign2380_e2290_d_n4;
        locals.var_evb2c2vdc_dn5 = assign2380_e2290_d_n5;
        locals.var_evb2c2vdc_dn6 = assign2380_e2290_d_n6;
        locals.var_evb2c2vdc_dn7 = assign2380_e2290_d_n7;
        locals.var_evb2c2vdc_dn8 = assign2380_e2290_d_n8;
        locals.var_evb2c2vdc_dn9 = assign2380_e2290_d_n9;

        let assign2390_e2293: f64 = (locals.var_vb2c1 - locals.var_vdc_t);
        let assign2390_e2295: f64 = (assign2390_e2293 * locals.var_vtinv);
        let assign2390_e2297: f64 = if assign2390_e2295 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign2390_e2297;

        let (assign2400_e2306, assign2400_e2306_d_n0, assign2400_e2306_d_n1, assign2400_e2306_d_n3, assign2400_e2306_d_n4, assign2400_e2306_d_n5, assign2400_e2306_d_n6, assign2400_e2306_d_n7, assign2400_e2306_d_n8, assign2400_e2306_d_n9,) = {
    if (locals.var_guard36 != 0.0) {
        let assign2400_e2301: f64 = (locals.var_vb2c1 - locals.var_vdc_t);
        let assign2400_e2303: f64 = (assign2400_e2301 * locals.var_vtinv);
        let assign2400_e2304: f64 = (assign2400_e2303).exp();
        (assign2400_e2304, (assign2400_e2304 * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (assign2400_e2304 * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (assign2400_e2304 * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (assign2400_e2304 * ((-locals.var_vdc_t_dn4) * locals.var_vtinv)), (assign2400_e2304 * ((locals.var_vb2c1_dn5 - locals.var_vdc_t_dn5) * locals.var_vtinv)), (assign2400_e2304 * ((locals.var_vb2c1_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (assign2400_e2304 * ((-locals.var_vdc_t_dn7) * locals.var_vtinv)), (assign2400_e2304 * ((-locals.var_vdc_t_dn8) * locals.var_vtinv)), (assign2400_e2304 * ((-locals.var_vdc_t_dn9) * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c1vdc, locals.var_evb2c1vdc_dn0, locals.var_evb2c1vdc_dn1, locals.var_evb2c1vdc_dn3, locals.var_evb2c1vdc_dn4, locals.var_evb2c1vdc_dn5, locals.var_evb2c1vdc_dn6, locals.var_evb2c1vdc_dn7, locals.var_evb2c1vdc_dn8, locals.var_evb2c1vdc_dn9,)
    }
};
        locals.var_evb2c1vdc = assign2400_e2306;
        locals.var_evb2c1vdc_dn0 = assign2400_e2306_d_n0;
        locals.var_evb2c1vdc_dn1 = assign2400_e2306_d_n1;
        locals.var_evb2c1vdc_dn3 = assign2400_e2306_d_n3;
        locals.var_evb2c1vdc_dn4 = assign2400_e2306_d_n4;
        locals.var_evb2c1vdc_dn5 = assign2400_e2306_d_n5;
        locals.var_evb2c1vdc_dn6 = assign2400_e2306_d_n6;
        locals.var_evb2c1vdc_dn7 = assign2400_e2306_d_n7;
        locals.var_evb2c1vdc_dn8 = assign2400_e2306_d_n8;
        locals.var_evb2c1vdc_dn9 = assign2400_e2306_d_n9;

        let (assign2410_e2312,) = {
    if (locals.var_guard36 == 0.0) {
        let assign2410_e2310: f64 = (p.p134).exp();
        (assign2410_e2310,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2410_e2312;

        let (assign2420_e2327, assign2420_e2327_d_n0, assign2420_e2327_d_n1, assign2420_e2327_d_n3, assign2420_e2327_d_n4, assign2420_e2327_d_n5, assign2420_e2327_d_n6, assign2420_e2327_d_n7, assign2420_e2327_d_n8, assign2420_e2327_d_n9,) = {
    if (locals.var_guard36 == 0.0) {
        let assign2420_e2319: f64 = (locals.var_vb2c1 - locals.var_vdc_t);
        let assign2420_e2321: f64 = (assign2420_e2319 * locals.var_vtinv);
        let assign2420_e2323: f64 = (assign2420_e2321 - p.p134);
        let assign2420_e2324: f64 = (1.0 + assign2420_e2323);
        let assign2420_e2325: f64 = (locals.var_expl * assign2420_e2324);
        (assign2420_e2325, (locals.var_expl * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn4) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2c1_dn5 - locals.var_vdc_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2c1_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn9) * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c1vdc, locals.var_evb2c1vdc_dn0, locals.var_evb2c1vdc_dn1, locals.var_evb2c1vdc_dn3, locals.var_evb2c1vdc_dn4, locals.var_evb2c1vdc_dn5, locals.var_evb2c1vdc_dn6, locals.var_evb2c1vdc_dn7, locals.var_evb2c1vdc_dn8, locals.var_evb2c1vdc_dn9,)
    }
};
        locals.var_evb2c1vdc = assign2420_e2327;
        locals.var_evb2c1vdc_dn0 = assign2420_e2327_d_n0;
        locals.var_evb2c1vdc_dn1 = assign2420_e2327_d_n1;
        locals.var_evb2c1vdc_dn3 = assign2420_e2327_d_n3;
        locals.var_evb2c1vdc_dn4 = assign2420_e2327_d_n4;
        locals.var_evb2c1vdc_dn5 = assign2420_e2327_d_n5;
        locals.var_evb2c1vdc_dn6 = assign2420_e2327_d_n6;
        locals.var_evb2c1vdc_dn7 = assign2420_e2327_d_n7;
        locals.var_evb2c1vdc_dn8 = assign2420_e2327_d_n8;
        locals.var_evb2c1vdc_dn9 = assign2420_e2327_d_n9;

        let assign2430_e2331: f64 = (4.0 * locals.var_evb2c2vdc);
        let assign2430_e2332: f64 = (1.0 + assign2430_e2331);
        let assign2430_e2333: f64 = (assign2430_e2332).sqrt();
        locals.var_k0 = assign2430_e2333;
        locals.var_k0_dn0 = ((4.0 * locals.var_evb2c2vdc_dn0) / (2.0 * assign2430_e2333));
        locals.var_k0_dn1 = ((4.0 * locals.var_evb2c2vdc_dn1) / (2.0 * assign2430_e2333));
        locals.var_k0_dn3 = ((4.0 * locals.var_evb2c2vdc_dn3) / (2.0 * assign2430_e2333));
        locals.var_k0_dn4 = ((4.0 * locals.var_evb2c2vdc_dn4) / (2.0 * assign2430_e2333));
        locals.var_k0_dn5 = ((4.0 * locals.var_evb2c2vdc_dn5) / (2.0 * assign2430_e2333));
        locals.var_k0_dn6 = ((4.0 * locals.var_evb2c2vdc_dn6) / (2.0 * assign2430_e2333));
        locals.var_k0_dn7 = ((4.0 * locals.var_evb2c2vdc_dn7) / (2.0 * assign2430_e2333));
        locals.var_k0_dn8 = ((4.0 * locals.var_evb2c2vdc_dn8) / (2.0 * assign2430_e2333));
        locals.var_k0_dn9 = ((4.0 * locals.var_evb2c2vdc_dn9) / (2.0 * assign2430_e2333));

        let assign2440_e2337: f64 = (4.0 * locals.var_evb2c1vdc);
        let assign2440_e2338: f64 = (1.0 + assign2440_e2337);
        let assign2440_e2339: f64 = (assign2440_e2338).sqrt();
        locals.var_kw = assign2440_e2339;
        locals.var_kw_dn0 = ((4.0 * locals.var_evb2c1vdc_dn0) / (2.0 * assign2440_e2339));
        locals.var_kw_dn1 = ((4.0 * locals.var_evb2c1vdc_dn1) / (2.0 * assign2440_e2339));
        locals.var_kw_dn3 = ((4.0 * locals.var_evb2c1vdc_dn3) / (2.0 * assign2440_e2339));
        locals.var_kw_dn4 = ((4.0 * locals.var_evb2c1vdc_dn4) / (2.0 * assign2440_e2339));
        locals.var_kw_dn5 = ((4.0 * locals.var_evb2c1vdc_dn5) / (2.0 * assign2440_e2339));
        locals.var_kw_dn6 = ((4.0 * locals.var_evb2c1vdc_dn6) / (2.0 * assign2440_e2339));
        locals.var_kw_dn7 = ((4.0 * locals.var_evb2c1vdc_dn7) / (2.0 * assign2440_e2339));
        locals.var_kw_dn8 = ((4.0 * locals.var_evb2c1vdc_dn8) / (2.0 * assign2440_e2339));
        locals.var_kw_dn9 = ((4.0 * locals.var_evb2c1vdc_dn9) / (2.0 * assign2440_e2339));

        let assign2450_e2342: f64 = (2.0 * locals.var_evb2c1vdc);
        let assign2450_e2345: f64 = (1.0 + locals.var_kw);
        let assign2450_e2346: f64 = (assign2450_e2342 / assign2450_e2345);
        locals.var_pw = assign2450_e2346;
        locals.var_pw_dn0 = ((((2.0 * locals.var_evb2c1vdc_dn0) * assign2450_e2345) - (assign2450_e2342 * locals.var_kw_dn0)) / (assign2450_e2345 * assign2450_e2345));
        locals.var_pw_dn1 = ((((2.0 * locals.var_evb2c1vdc_dn1) * assign2450_e2345) - (assign2450_e2342 * locals.var_kw_dn1)) / (assign2450_e2345 * assign2450_e2345));
        locals.var_pw_dn3 = ((((2.0 * locals.var_evb2c1vdc_dn3) * assign2450_e2345) - (assign2450_e2342 * locals.var_kw_dn3)) / (assign2450_e2345 * assign2450_e2345));
        locals.var_pw_dn4 = ((((2.0 * locals.var_evb2c1vdc_dn4) * assign2450_e2345) - (assign2450_e2342 * locals.var_kw_dn4)) / (assign2450_e2345 * assign2450_e2345));
        locals.var_pw_dn5 = ((((2.0 * locals.var_evb2c1vdc_dn5) * assign2450_e2345) - (assign2450_e2342 * locals.var_kw_dn5)) / (assign2450_e2345 * assign2450_e2345));
        locals.var_pw_dn6 = ((((2.0 * locals.var_evb2c1vdc_dn6) * assign2450_e2345) - (assign2450_e2342 * locals.var_kw_dn6)) / (assign2450_e2345 * assign2450_e2345));
        locals.var_pw_dn7 = ((((2.0 * locals.var_evb2c1vdc_dn7) * assign2450_e2345) - (assign2450_e2342 * locals.var_kw_dn7)) / (assign2450_e2345 * assign2450_e2345));
        locals.var_pw_dn8 = ((((2.0 * locals.var_evb2c1vdc_dn8) * assign2450_e2345) - (assign2450_e2342 * locals.var_kw_dn8)) / (assign2450_e2345 * assign2450_e2345));
        locals.var_pw_dn9 = ((((2.0 * locals.var_evb2c1vdc_dn9) * assign2450_e2345) - (assign2450_e2342 * locals.var_kw_dn9)) / (assign2450_e2345 * assign2450_e2345));

        let assign2460_e2349: f64 = if locals.var_pw < p.p136 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign2460_e2349;

        let (assign2470_e2353, assign2470_e2353_d_n0, assign2470_e2353_d_n1, assign2470_e2353_d_n3, assign2470_e2353_d_n4, assign2470_e2353_d_n5, assign2470_e2353_d_n6, assign2470_e2353_d_n7, assign2470_e2353_d_n8, assign2470_e2353_d_n9,) = {
    if (locals.var_guard37 != 0.0) {
        (p.p136, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pw, locals.var_pw_dn0, locals.var_pw_dn1, locals.var_pw_dn3, locals.var_pw_dn4, locals.var_pw_dn5, locals.var_pw_dn6, locals.var_pw_dn7, locals.var_pw_dn8, locals.var_pw_dn9,)
    }
};
        locals.var_pw = assign2470_e2353;
        locals.var_pw_dn0 = assign2470_e2353_d_n0;
        locals.var_pw_dn1 = assign2470_e2353_d_n1;
        locals.var_pw_dn3 = assign2470_e2353_d_n3;
        locals.var_pw_dn4 = assign2470_e2353_d_n4;
        locals.var_pw_dn5 = assign2470_e2353_d_n5;
        locals.var_pw_dn6 = assign2470_e2353_d_n6;
        locals.var_pw_dn7 = assign2470_e2353_d_n7;
        locals.var_pw_dn8 = assign2470_e2353_d_n8;
        locals.var_pw_dn9 = assign2470_e2353_d_n9;

        let assign2480_e2357: f64 = (locals.var_k0 - locals.var_kw);
        let assign2480_e2360: f64 = (locals.var_k0 + 1.0);
        let assign2480_e2363: f64 = (locals.var_kw + 1.0);
        let assign2480_e2364: f64 = (assign2480_e2360 / assign2480_e2363);
        let assign2480_e2365: f64 = (assign2480_e2364).ln();
        let assign2480_e2366: f64 = (assign2480_e2357 - assign2480_e2365);
        let assign2480_e2367: f64 = (locals.var_vt * assign2480_e2366);
        locals.var_ec = assign2480_e2367;
        locals.var_ec_dn0 = (locals.var_vt * ((locals.var_k0_dn0 - locals.var_kw_dn0) - ((((locals.var_k0_dn0 * assign2480_e2363) - (assign2480_e2360 * locals.var_kw_dn0)) / (assign2480_e2363 * assign2480_e2363)) / assign2480_e2364)));
        locals.var_ec_dn1 = (locals.var_vt * ((locals.var_k0_dn1 - locals.var_kw_dn1) - ((((locals.var_k0_dn1 * assign2480_e2363) - (assign2480_e2360 * locals.var_kw_dn1)) / (assign2480_e2363 * assign2480_e2363)) / assign2480_e2364)));
        locals.var_ec_dn3 = (locals.var_vt * ((locals.var_k0_dn3 - locals.var_kw_dn3) - ((((locals.var_k0_dn3 * assign2480_e2363) - (assign2480_e2360 * locals.var_kw_dn3)) / (assign2480_e2363 * assign2480_e2363)) / assign2480_e2364)));
        locals.var_ec_dn4 = (locals.var_vt * ((locals.var_k0_dn4 - locals.var_kw_dn4) - ((((locals.var_k0_dn4 * assign2480_e2363) - (assign2480_e2360 * locals.var_kw_dn4)) / (assign2480_e2363 * assign2480_e2363)) / assign2480_e2364)));
        locals.var_ec_dn5 = (locals.var_vt * ((locals.var_k0_dn5 - locals.var_kw_dn5) - ((((locals.var_k0_dn5 * assign2480_e2363) - (assign2480_e2360 * locals.var_kw_dn5)) / (assign2480_e2363 * assign2480_e2363)) / assign2480_e2364)));
        locals.var_ec_dn6 = (locals.var_vt * ((locals.var_k0_dn6 - locals.var_kw_dn6) - ((((locals.var_k0_dn6 * assign2480_e2363) - (assign2480_e2360 * locals.var_kw_dn6)) / (assign2480_e2363 * assign2480_e2363)) / assign2480_e2364)));
        locals.var_ec_dn7 = (locals.var_vt * ((locals.var_k0_dn7 - locals.var_kw_dn7) - ((((locals.var_k0_dn7 * assign2480_e2363) - (assign2480_e2360 * locals.var_kw_dn7)) / (assign2480_e2363 * assign2480_e2363)) / assign2480_e2364)));
        locals.var_ec_dn8 = (locals.var_vt * ((locals.var_k0_dn8 - locals.var_kw_dn8) - ((((locals.var_k0_dn8 * assign2480_e2363) - (assign2480_e2360 * locals.var_kw_dn8)) / (assign2480_e2363 * assign2480_e2363)) / assign2480_e2364)));
        locals.var_ec_dn9 = (locals.var_vt * ((locals.var_k0_dn9 - locals.var_kw_dn9) - ((((locals.var_k0_dn9 * assign2480_e2363) - (assign2480_e2360 * locals.var_kw_dn9)) / (assign2480_e2363 * assign2480_e2363)) / assign2480_e2364)));

        let assign2490_e2370: f64 = (locals.var_ec + locals.var_vc1c2);
        let assign2490_e2372: f64 = (assign2490_e2370 / locals.var_rcv_t);
        locals.var_ic1c2 = assign2490_e2372;
        locals.var_ic1c2_dn0 = (locals.var_ec_dn0 / locals.var_rcv_t);
        locals.var_ic1c2_dn1 = (locals.var_ec_dn1 / locals.var_rcv_t);
        locals.var_ic1c2_dn3 = (locals.var_ec_dn3 / locals.var_rcv_t);
        locals.var_ic1c2_dn4 = (locals.var_ec_dn4 / locals.var_rcv_t);
        locals.var_ic1c2_dn5 = (locals.var_ec_dn5 / locals.var_rcv_t);
        locals.var_ic1c2_dn6 = ((locals.var_ec_dn6 + locals.var_vc1c2_dn6) / locals.var_rcv_t);
        locals.var_ic1c2_dn7 = ((locals.var_ec_dn7 + locals.var_vc1c2_dn7) / locals.var_rcv_t);
        locals.var_ic1c2_dn8 = (locals.var_ec_dn8 / locals.var_rcv_t);
        locals.var_ic1c2_dn9 = (locals.var_ec_dn9 / locals.var_rcv_t);

        let assign2500_e2375: f64 = if locals.var_ic1c2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard38 = assign2500_e2375;

        let assign2510_e2378: f64 = if locals.var_vb2c1 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign2510_e2378;

        let (assign2520_e2384, assign2520_e2384_d_n5, assign2520_e2384_d_n6,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard39 != 0.0)) {
        (locals.var_vb2c1, locals.var_vb2c1_dn5, locals.var_vb2c1_dn6,)
    } else {
        (locals.var_tmpv, locals.var_tmpv_dn5, locals.var_tmpv_dn6,)
    }
};
        locals.var_tmpv = assign2520_e2384;
        locals.var_tmpv_dn5 = assign2520_e2384_d_n5;
        locals.var_tmpv_dn6 = assign2520_e2384_d_n6;

        let (assign2530_e2398, assign2530_e2398_d_n5, assign2530_e2398_d_n6,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard39 == 0.0)) {
        let assign2530_e2393: f64 = (locals.var_vb2c1 - 100.0);
        let assign2530_e2394: f64 = (1.0 + assign2530_e2393);
        let assign2530_e2395: f64 = (assign2530_e2394).ln();
        let assign2530_e2396: f64 = (100.0 + assign2530_e2395);
        (assign2530_e2396, (locals.var_vb2c1_dn5 / assign2530_e2394), (locals.var_vb2c1_dn6 / assign2530_e2394),)
    } else {
        (locals.var_tmpv, locals.var_tmpv_dn5, locals.var_tmpv_dn6,)
    }
};
        locals.var_tmpv = assign2530_e2398;
        locals.var_tmpv_dn5 = assign2530_e2398_d_n5;
        locals.var_tmpv_dn6 = assign2530_e2398_d_n6;

        let (assign2540_e2419, assign2540_e2419_d_n0, assign2540_e2419_d_n1, assign2540_e2419_d_n3, assign2540_e2419_d_n4, assign2540_e2419_d_n5, assign2540_e2419_d_n6, assign2540_e2419_d_n7, assign2540_e2419_d_n8, assign2540_e2419_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2540_e2403: f64 = (2.0 * locals.var_vt);
        let assign2540_e2406: f64 = (0.5 * locals.var_ic1c2);
        let assign2540_e2408: f64 = (assign2540_e2406 * locals.var_rcv_t);
        let assign2540_e2410: f64 = (assign2540_e2408 * locals.var_vtinv);
        let assign2540_e2412: f64 = (assign2540_e2410 + 1.0);
        let assign2540_e2413: f64 = (assign2540_e2412).ln();
        let assign2540_e2414: f64 = (assign2540_e2403 * assign2540_e2413);
        let assign2540_e2415: f64 = (locals.var_vdc_t + assign2540_e2414);
        let assign2540_e2417: f64 = (assign2540_e2415 - locals.var_tmpv);
        (assign2540_e2417, (locals.var_vdc_t_dn0 + (assign2540_e2403 * ((((0.5 * locals.var_ic1c2_dn0) * locals.var_rcv_t) * locals.var_vtinv) / assign2540_e2412))), (locals.var_vdc_t_dn1 + (assign2540_e2403 * ((((0.5 * locals.var_ic1c2_dn1) * locals.var_rcv_t) * locals.var_vtinv) / assign2540_e2412))), (locals.var_vdc_t_dn3 + (assign2540_e2403 * ((((0.5 * locals.var_ic1c2_dn3) * locals.var_rcv_t) * locals.var_vtinv) / assign2540_e2412))), (locals.var_vdc_t_dn4 + (assign2540_e2403 * ((((0.5 * locals.var_ic1c2_dn4) * locals.var_rcv_t) * locals.var_vtinv) / assign2540_e2412))), ((locals.var_vdc_t_dn5 + (assign2540_e2403 * ((((0.5 * locals.var_ic1c2_dn5) * locals.var_rcv_t) * locals.var_vtinv) / assign2540_e2412))) - locals.var_tmpv_dn5), ((locals.var_vdc_t_dn6 + (assign2540_e2403 * ((((0.5 * locals.var_ic1c2_dn6) * locals.var_rcv_t) * locals.var_vtinv) / assign2540_e2412))) - locals.var_tmpv_dn6), (locals.var_vdc_t_dn7 + (assign2540_e2403 * ((((0.5 * locals.var_ic1c2_dn7) * locals.var_rcv_t) * locals.var_vtinv) / assign2540_e2412))), (locals.var_vdc_t_dn8 + (assign2540_e2403 * ((((0.5 * locals.var_ic1c2_dn8) * locals.var_rcv_t) * locals.var_vtinv) / assign2540_e2412))), (locals.var_vdc_t_dn9 + (assign2540_e2403 * ((((0.5 * locals.var_ic1c2_dn9) * locals.var_rcv_t) * locals.var_vtinv) / assign2540_e2412))),)
    } else {
        (locals.var_vqs_th, locals.var_vqs_th_dn0, locals.var_vqs_th_dn1, locals.var_vqs_th_dn3, locals.var_vqs_th_dn4, locals.var_vqs_th_dn5, locals.var_vqs_th_dn6, locals.var_vqs_th_dn7, locals.var_vqs_th_dn8, locals.var_vqs_th_dn9,)
    }
};
        locals.var_vqs_th = assign2540_e2419;
        locals.var_vqs_th_dn0 = assign2540_e2419_d_n0;
        locals.var_vqs_th_dn1 = assign2540_e2419_d_n1;
        locals.var_vqs_th_dn3 = assign2540_e2419_d_n3;
        locals.var_vqs_th_dn4 = assign2540_e2419_d_n4;
        locals.var_vqs_th_dn5 = assign2540_e2419_d_n5;
        locals.var_vqs_th_dn6 = assign2540_e2419_d_n6;
        locals.var_vqs_th_dn7 = assign2540_e2419_d_n7;
        locals.var_vqs_th_dn8 = assign2540_e2419_d_n8;
        locals.var_vqs_th_dn9 = assign2540_e2419_d_n9;

        let (assign2550_e2425, assign2550_e2425_d_n0, assign2550_e2425_d_n1, assign2550_e2425_d_n3, assign2550_e2425_d_n4, assign2550_e2425_d_n5, assign2550_e2425_d_n6, assign2550_e2425_d_n7, assign2550_e2425_d_n8, assign2550_e2425_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2550_e2423: f64 = (0.2 * locals.var_vdc_t);
        (assign2550_e2423, (0.2 * locals.var_vdc_t_dn0), (0.2 * locals.var_vdc_t_dn1), (0.2 * locals.var_vdc_t_dn3), (0.2 * locals.var_vdc_t_dn4), (0.2 * locals.var_vdc_t_dn5), (0.2 * locals.var_vdc_t_dn6), (0.2 * locals.var_vdc_t_dn7), (0.2 * locals.var_vdc_t_dn8), (0.2 * locals.var_vdc_t_dn9),)
    } else {
        (locals.var_eps_vdc, locals.var_eps_vdc_dn0, locals.var_eps_vdc_dn1, locals.var_eps_vdc_dn3, locals.var_eps_vdc_dn4, locals.var_eps_vdc_dn5, locals.var_eps_vdc_dn6, locals.var_eps_vdc_dn7, locals.var_eps_vdc_dn8, locals.var_eps_vdc_dn9,)
    }
};
        locals.var_eps_vdc = assign2550_e2425;
        locals.var_eps_vdc_dn0 = assign2550_e2425_d_n0;
        locals.var_eps_vdc_dn1 = assign2550_e2425_d_n1;
        locals.var_eps_vdc_dn3 = assign2550_e2425_d_n3;
        locals.var_eps_vdc_dn4 = assign2550_e2425_d_n4;
        locals.var_eps_vdc_dn5 = assign2550_e2425_d_n5;
        locals.var_eps_vdc_dn6 = assign2550_e2425_d_n6;
        locals.var_eps_vdc_dn7 = assign2550_e2425_d_n7;
        locals.var_eps_vdc_dn8 = assign2550_e2425_d_n8;
        locals.var_eps_vdc_dn9 = assign2550_e2425_d_n9;

        let (assign2560_e2431, assign2560_e2431_d_n0, assign2560_e2431_d_n1, assign2560_e2431_d_n3, assign2560_e2431_d_n4, assign2560_e2431_d_n5, assign2560_e2431_d_n6, assign2560_e2431_d_n7, assign2560_e2431_d_n8, assign2560_e2431_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2560_e2429: f64 = (locals.var_eps_vdc * locals.var_eps_vdc);
        (assign2560_e2429, ((locals.var_eps_vdc_dn0 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn0)), ((locals.var_eps_vdc_dn1 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn1)), ((locals.var_eps_vdc_dn3 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn3)), ((locals.var_eps_vdc_dn4 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn4)), ((locals.var_eps_vdc_dn5 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn5)), ((locals.var_eps_vdc_dn6 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn6)), ((locals.var_eps_vdc_dn7 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn7)), ((locals.var_eps_vdc_dn8 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn8)), ((locals.var_eps_vdc_dn9 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn9)),)
    } else {
        (locals.var_eps2, locals.var_eps2_dn0, locals.var_eps2_dn1, locals.var_eps2_dn3, locals.var_eps2_dn4, locals.var_eps2_dn5, locals.var_eps2_dn6, locals.var_eps2_dn7, locals.var_eps2_dn8, locals.var_eps2_dn9,)
    }
};
        locals.var_eps2 = assign2560_e2431;
        locals.var_eps2_dn0 = assign2560_e2431_d_n0;
        locals.var_eps2_dn1 = assign2560_e2431_d_n1;
        locals.var_eps2_dn3 = assign2560_e2431_d_n3;
        locals.var_eps2_dn4 = assign2560_e2431_d_n4;
        locals.var_eps2_dn5 = assign2560_e2431_d_n5;
        locals.var_eps2_dn6 = assign2560_e2431_d_n6;
        locals.var_eps2_dn7 = assign2560_e2431_d_n7;
        locals.var_eps2_dn8 = assign2560_e2431_d_n8;
        locals.var_eps2_dn9 = assign2560_e2431_d_n9;

        let (assign2570_e2437, assign2570_e2437_d_n0, assign2570_e2437_d_n1, assign2570_e2437_d_n3, assign2570_e2437_d_n4, assign2570_e2437_d_n5, assign2570_e2437_d_n6, assign2570_e2437_d_n7, assign2570_e2437_d_n8, assign2570_e2437_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2570_e2435: f64 = (locals.var_vqs_th * locals.var_vqs_th);
        (assign2570_e2435, ((locals.var_vqs_th_dn0 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn0)), ((locals.var_vqs_th_dn1 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn1)), ((locals.var_vqs_th_dn3 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn3)), ((locals.var_vqs_th_dn4 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn4)), ((locals.var_vqs_th_dn5 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn5)), ((locals.var_vqs_th_dn6 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn6)), ((locals.var_vqs_th_dn7 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn7)), ((locals.var_vqs_th_dn8 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn8)), ((locals.var_vqs_th_dn9 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn9)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn1, locals.var_x2_dn3, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9,)
    }
};
        locals.var_x2 = assign2570_e2437;
        locals.var_x2_dn0 = assign2570_e2437_d_n0;
        locals.var_x2_dn1 = assign2570_e2437_d_n1;
        locals.var_x2_dn3 = assign2570_e2437_d_n3;
        locals.var_x2_dn4 = assign2570_e2437_d_n4;
        locals.var_x2_dn5 = assign2570_e2437_d_n5;
        locals.var_x2_dn6 = assign2570_e2437_d_n6;
        locals.var_x2_dn7 = assign2570_e2437_d_n7;
        locals.var_x2_dn8 = assign2570_e2437_d_n8;
        locals.var_x2_dn9 = assign2570_e2437_d_n9;

        let assign2580_e2440: f64 = if locals.var_vqs_th < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign2580_e2440;

        let (assign2590_e2455, assign2590_e2455_d_n0, assign2590_e2455_d_n1, assign2590_e2455_d_n3, assign2590_e2455_d_n4, assign2590_e2455_d_n5, assign2590_e2455_d_n6, assign2590_e2455_d_n7, assign2590_e2455_d_n8, assign2590_e2455_d_n9,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard40 != 0.0)) {
        let assign2590_e2446: f64 = (0.5 * locals.var_eps2);
        let assign2590_e2449: f64 = (locals.var_x2 + locals.var_eps2);
        let assign2590_e2450: f64 = (assign2590_e2449).sqrt();
        let assign2590_e2452: f64 = (assign2590_e2450 - locals.var_vqs_th);
        let assign2590_e2453: f64 = (assign2590_e2446 / assign2590_e2452);
        (assign2590_e2453, ((((0.5 * locals.var_eps2_dn0) * assign2590_e2452) - (assign2590_e2446 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign2590_e2450)) - locals.var_vqs_th_dn0))) / (assign2590_e2452 * assign2590_e2452)), ((((0.5 * locals.var_eps2_dn1) * assign2590_e2452) - (assign2590_e2446 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign2590_e2450)) - locals.var_vqs_th_dn1))) / (assign2590_e2452 * assign2590_e2452)), ((((0.5 * locals.var_eps2_dn3) * assign2590_e2452) - (assign2590_e2446 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign2590_e2450)) - locals.var_vqs_th_dn3))) / (assign2590_e2452 * assign2590_e2452)), ((((0.5 * locals.var_eps2_dn4) * assign2590_e2452) - (assign2590_e2446 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign2590_e2450)) - locals.var_vqs_th_dn4))) / (assign2590_e2452 * assign2590_e2452)), ((((0.5 * locals.var_eps2_dn5) * assign2590_e2452) - (assign2590_e2446 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign2590_e2450)) - locals.var_vqs_th_dn5))) / (assign2590_e2452 * assign2590_e2452)), ((((0.5 * locals.var_eps2_dn6) * assign2590_e2452) - (assign2590_e2446 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign2590_e2450)) - locals.var_vqs_th_dn6))) / (assign2590_e2452 * assign2590_e2452)), ((((0.5 * locals.var_eps2_dn7) * assign2590_e2452) - (assign2590_e2446 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign2590_e2450)) - locals.var_vqs_th_dn7))) / (assign2590_e2452 * assign2590_e2452)), ((((0.5 * locals.var_eps2_dn8) * assign2590_e2452) - (assign2590_e2446 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign2590_e2450)) - locals.var_vqs_th_dn8))) / (assign2590_e2452 * assign2590_e2452)), ((((0.5 * locals.var_eps2_dn9) * assign2590_e2452) - (assign2590_e2446 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign2590_e2450)) - locals.var_vqs_th_dn9))) / (assign2590_e2452 * assign2590_e2452)),)
    } else {
        (locals.var_vqs, locals.var_vqs_dn0, locals.var_vqs_dn1, locals.var_vqs_dn3, locals.var_vqs_dn4, locals.var_vqs_dn5, locals.var_vqs_dn6, locals.var_vqs_dn7, locals.var_vqs_dn8, locals.var_vqs_dn9,)
    }
};
        locals.var_vqs = assign2590_e2455;
        locals.var_vqs_dn0 = assign2590_e2455_d_n0;
        locals.var_vqs_dn1 = assign2590_e2455_d_n1;
        locals.var_vqs_dn3 = assign2590_e2455_d_n3;
        locals.var_vqs_dn4 = assign2590_e2455_d_n4;
        locals.var_vqs_dn5 = assign2590_e2455_d_n5;
        locals.var_vqs_dn6 = assign2590_e2455_d_n6;
        locals.var_vqs_dn7 = assign2590_e2455_d_n7;
        locals.var_vqs_dn8 = assign2590_e2455_d_n8;
        locals.var_vqs_dn9 = assign2590_e2455_d_n9;

        let (assign2600_e2469, assign2600_e2469_d_n0, assign2600_e2469_d_n1, assign2600_e2469_d_n3, assign2600_e2469_d_n4, assign2600_e2469_d_n5, assign2600_e2469_d_n6, assign2600_e2469_d_n7, assign2600_e2469_d_n8, assign2600_e2469_d_n9,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard40 == 0.0)) {
        let assign2600_e2463: f64 = (locals.var_x2 + locals.var_eps2);
        let assign2600_e2464: f64 = (assign2600_e2463).sqrt();
        let assign2600_e2466: f64 = (assign2600_e2464 + locals.var_vqs_th);
        let assign2600_e2467: f64 = (0.5 * assign2600_e2466);
        (assign2600_e2467, (0.5 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign2600_e2464)) + locals.var_vqs_th_dn0)), (0.5 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign2600_e2464)) + locals.var_vqs_th_dn1)), (0.5 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign2600_e2464)) + locals.var_vqs_th_dn3)), (0.5 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign2600_e2464)) + locals.var_vqs_th_dn4)), (0.5 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign2600_e2464)) + locals.var_vqs_th_dn5)), (0.5 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign2600_e2464)) + locals.var_vqs_th_dn6)), (0.5 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign2600_e2464)) + locals.var_vqs_th_dn7)), (0.5 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign2600_e2464)) + locals.var_vqs_th_dn8)), (0.5 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign2600_e2464)) + locals.var_vqs_th_dn9)),)
    } else {
        (locals.var_vqs, locals.var_vqs_dn0, locals.var_vqs_dn1, locals.var_vqs_dn3, locals.var_vqs_dn4, locals.var_vqs_dn5, locals.var_vqs_dn6, locals.var_vqs_dn7, locals.var_vqs_dn8, locals.var_vqs_dn9,)
    }
};
        locals.var_vqs = assign2600_e2469;
        locals.var_vqs_dn0 = assign2600_e2469_d_n0;
        locals.var_vqs_dn1 = assign2600_e2469_d_n1;
        locals.var_vqs_dn3 = assign2600_e2469_d_n3;
        locals.var_vqs_dn4 = assign2600_e2469_d_n4;
        locals.var_vqs_dn5 = assign2600_e2469_d_n5;
        locals.var_vqs_dn6 = assign2600_e2469_d_n6;
        locals.var_vqs_dn7 = assign2600_e2469_d_n7;
        locals.var_vqs_dn8 = assign2600_e2469_d_n8;
        locals.var_vqs_dn9 = assign2600_e2469_d_n9;

        let (assign2610_e2487, assign2610_e2487_d_n0, assign2610_e2487_d_n1, assign2610_e2487_d_n3, assign2610_e2487_d_n4, assign2610_e2487_d_n5, assign2610_e2487_d_n6, assign2610_e2487_d_n7, assign2610_e2487_d_n8, assign2610_e2487_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2610_e2475: f64 = (p.p61 * p.p60);
        let assign2610_e2476: f64 = (locals.var_vqs + assign2610_e2475);
        let assign2610_e2477: f64 = (locals.var_vqs * assign2610_e2476);
        let assign2610_e2482: f64 = (p.p61 * locals.var_rcv_t);
        let assign2610_e2483: f64 = (locals.var_vqs + assign2610_e2482);
        let assign2610_e2484: f64 = (p.p60 * assign2610_e2483);
        let assign2610_e2485: f64 = (assign2610_e2477 / assign2610_e2484);
        (assign2610_e2485, (((((locals.var_vqs_dn0 * assign2610_e2476) + (locals.var_vqs * locals.var_vqs_dn0)) * assign2610_e2484) - (assign2610_e2477 * (p.p60 * locals.var_vqs_dn0))) / (assign2610_e2484 * assign2610_e2484)), (((((locals.var_vqs_dn1 * assign2610_e2476) + (locals.var_vqs * locals.var_vqs_dn1)) * assign2610_e2484) - (assign2610_e2477 * (p.p60 * locals.var_vqs_dn1))) / (assign2610_e2484 * assign2610_e2484)), (((((locals.var_vqs_dn3 * assign2610_e2476) + (locals.var_vqs * locals.var_vqs_dn3)) * assign2610_e2484) - (assign2610_e2477 * (p.p60 * locals.var_vqs_dn3))) / (assign2610_e2484 * assign2610_e2484)), (((((locals.var_vqs_dn4 * assign2610_e2476) + (locals.var_vqs * locals.var_vqs_dn4)) * assign2610_e2484) - (assign2610_e2477 * (p.p60 * locals.var_vqs_dn4))) / (assign2610_e2484 * assign2610_e2484)), (((((locals.var_vqs_dn5 * assign2610_e2476) + (locals.var_vqs * locals.var_vqs_dn5)) * assign2610_e2484) - (assign2610_e2477 * (p.p60 * locals.var_vqs_dn5))) / (assign2610_e2484 * assign2610_e2484)), (((((locals.var_vqs_dn6 * assign2610_e2476) + (locals.var_vqs * locals.var_vqs_dn6)) * assign2610_e2484) - (assign2610_e2477 * (p.p60 * locals.var_vqs_dn6))) / (assign2610_e2484 * assign2610_e2484)), (((((locals.var_vqs_dn7 * assign2610_e2476) + (locals.var_vqs * locals.var_vqs_dn7)) * assign2610_e2484) - (assign2610_e2477 * (p.p60 * locals.var_vqs_dn7))) / (assign2610_e2484 * assign2610_e2484)), (((((locals.var_vqs_dn8 * assign2610_e2476) + (locals.var_vqs * locals.var_vqs_dn8)) * assign2610_e2484) - (assign2610_e2477 * (p.p60 * locals.var_vqs_dn8))) / (assign2610_e2484 * assign2610_e2484)), (((((locals.var_vqs_dn9 * assign2610_e2476) + (locals.var_vqs * locals.var_vqs_dn9)) * assign2610_e2484) - (assign2610_e2477 * (p.p60 * locals.var_vqs_dn9))) / (assign2610_e2484 * assign2610_e2484)),)
    } else {
        (locals.var_iqs, locals.var_iqs_dn0, locals.var_iqs_dn1, locals.var_iqs_dn3, locals.var_iqs_dn4, locals.var_iqs_dn5, locals.var_iqs_dn6, locals.var_iqs_dn7, locals.var_iqs_dn8, locals.var_iqs_dn9,)
    }
};
        locals.var_iqs = assign2610_e2487;
        locals.var_iqs_dn0 = assign2610_e2487_d_n0;
        locals.var_iqs_dn1 = assign2610_e2487_d_n1;
        locals.var_iqs_dn3 = assign2610_e2487_d_n3;
        locals.var_iqs_dn4 = assign2610_e2487_d_n4;
        locals.var_iqs_dn5 = assign2610_e2487_d_n5;
        locals.var_iqs_dn6 = assign2610_e2487_d_n6;
        locals.var_iqs_dn7 = assign2610_e2487_d_n7;
        locals.var_iqs_dn8 = assign2610_e2487_d_n8;
        locals.var_iqs_dn9 = assign2610_e2487_d_n9;

        let (assign2620_e2493, assign2620_e2493_d_n0, assign2620_e2493_d_n1, assign2620_e2493_d_n3, assign2620_e2493_d_n4, assign2620_e2493_d_n5, assign2620_e2493_d_n6, assign2620_e2493_d_n7, assign2620_e2493_d_n8, assign2620_e2493_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2620_e2491: f64 = (locals.var_ic1c2 / locals.var_iqs);
        (assign2620_e2491, (((locals.var_ic1c2_dn0 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn0)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn1 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn1)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn3 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn3)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn4 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn4)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn5 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn5)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn6 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn6)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn7 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn7)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn8 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn8)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn9 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn9)) / (locals.var_iqs * locals.var_iqs)),)
    } else {
        (locals.var_ic1c2_iqs, locals.var_ic1c2_iqs_dn0, locals.var_ic1c2_iqs_dn1, locals.var_ic1c2_iqs_dn3, locals.var_ic1c2_iqs_dn4, locals.var_ic1c2_iqs_dn5, locals.var_ic1c2_iqs_dn6, locals.var_ic1c2_iqs_dn7, locals.var_ic1c2_iqs_dn8, locals.var_ic1c2_iqs_dn9,)
    }
};
        locals.var_ic1c2_iqs = assign2620_e2493;
        locals.var_ic1c2_iqs_dn0 = assign2620_e2493_d_n0;
        locals.var_ic1c2_iqs_dn1 = assign2620_e2493_d_n1;
        locals.var_ic1c2_iqs_dn3 = assign2620_e2493_d_n3;
        locals.var_ic1c2_iqs_dn4 = assign2620_e2493_d_n4;
        locals.var_ic1c2_iqs_dn5 = assign2620_e2493_d_n5;
        locals.var_ic1c2_iqs_dn6 = assign2620_e2493_d_n6;
        locals.var_ic1c2_iqs_dn7 = assign2620_e2493_d_n7;
        locals.var_ic1c2_iqs_dn8 = assign2620_e2493_d_n8;
        locals.var_ic1c2_iqs_dn9 = assign2620_e2493_d_n9;

        let (assign2630_e2501, assign2630_e2501_d_n0, assign2630_e2501_d_n1, assign2630_e2501_d_n3, assign2630_e2501_d_n4, assign2630_e2501_d_n5, assign2630_e2501_d_n6, assign2630_e2501_d_n7, assign2630_e2501_d_n8, assign2630_e2501_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2630_e2497: f64 = (locals.var_ic1c2_iqs - 1.0);
        let assign2630_e2499: f64 = (assign2630_e2497 / p.p62);
        (assign2630_e2499, (locals.var_ic1c2_iqs_dn0 / p.p62), (locals.var_ic1c2_iqs_dn1 / p.p62), (locals.var_ic1c2_iqs_dn3 / p.p62), (locals.var_ic1c2_iqs_dn4 / p.p62), (locals.var_ic1c2_iqs_dn5 / p.p62), (locals.var_ic1c2_iqs_dn6 / p.p62), (locals.var_ic1c2_iqs_dn7 / p.p62), (locals.var_ic1c2_iqs_dn8 / p.p62), (locals.var_ic1c2_iqs_dn9 / p.p62),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9,)
    }
};
        locals.var_dxa = assign2630_e2501;
        locals.var_dxa_dn0 = assign2630_e2501_d_n0;
        locals.var_dxa_dn1 = assign2630_e2501_d_n1;
        locals.var_dxa_dn3 = assign2630_e2501_d_n3;
        locals.var_dxa_dn4 = assign2630_e2501_d_n4;
        locals.var_dxa_dn5 = assign2630_e2501_d_n5;
        locals.var_dxa_dn6 = assign2630_e2501_d_n6;
        locals.var_dxa_dn7 = assign2630_e2501_d_n7;
        locals.var_dxa_dn8 = assign2630_e2501_d_n8;
        locals.var_dxa_dn9 = assign2630_e2501_d_n9;

        let assign2640_e2504: f64 = if locals.var_ic1c2_iqs < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign2640_e2504;

        let (assign2650_e2518, assign2650_e2518_d_n0, assign2650_e2518_d_n1, assign2650_e2518_d_n3, assign2650_e2518_d_n4, assign2650_e2518_d_n5, assign2650_e2518_d_n6, assign2650_e2518_d_n7, assign2650_e2518_d_n8, assign2650_e2518_d_n9,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard41 != 0.0)) {
        let assign2650_e2512: f64 = (locals.var_dxa).exp();
        let assign2650_e2513: f64 = (1.0 + assign2650_e2512);
        let assign2650_e2514: f64 = (assign2650_e2513).ln();
        let assign2650_e2515: f64 = (p.p62 * assign2650_e2514);
        let assign2650_e2516: f64 = (1.0 + assign2650_e2515);
        (assign2650_e2516, (p.p62 * ((assign2650_e2512 * locals.var_dxa_dn0) / assign2650_e2513)), (p.p62 * ((assign2650_e2512 * locals.var_dxa_dn1) / assign2650_e2513)), (p.p62 * ((assign2650_e2512 * locals.var_dxa_dn3) / assign2650_e2513)), (p.p62 * ((assign2650_e2512 * locals.var_dxa_dn4) / assign2650_e2513)), (p.p62 * ((assign2650_e2512 * locals.var_dxa_dn5) / assign2650_e2513)), (p.p62 * ((assign2650_e2512 * locals.var_dxa_dn6) / assign2650_e2513)), (p.p62 * ((assign2650_e2512 * locals.var_dxa_dn7) / assign2650_e2513)), (p.p62 * ((assign2650_e2512 * locals.var_dxa_dn8) / assign2650_e2513)), (p.p62 * ((assign2650_e2512 * locals.var_dxa_dn9) / assign2650_e2513)),)
    } else {
        (locals.var_alpha1, locals.var_alpha1_dn0, locals.var_alpha1_dn1, locals.var_alpha1_dn3, locals.var_alpha1_dn4, locals.var_alpha1_dn5, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8, locals.var_alpha1_dn9,)
    }
};
        locals.var_alpha1 = assign2650_e2518;
        locals.var_alpha1_dn0 = assign2650_e2518_d_n0;
        locals.var_alpha1_dn1 = assign2650_e2518_d_n1;
        locals.var_alpha1_dn3 = assign2650_e2518_d_n3;
        locals.var_alpha1_dn4 = assign2650_e2518_d_n4;
        locals.var_alpha1_dn5 = assign2650_e2518_d_n5;
        locals.var_alpha1_dn6 = assign2650_e2518_d_n6;
        locals.var_alpha1_dn7 = assign2650_e2518_d_n7;
        locals.var_alpha1_dn8 = assign2650_e2518_d_n8;
        locals.var_alpha1_dn9 = assign2650_e2518_d_n9;

        let (assign2660_e2534, assign2660_e2534_d_n0, assign2660_e2534_d_n1, assign2660_e2534_d_n3, assign2660_e2534_d_n4, assign2660_e2534_d_n5, assign2660_e2534_d_n6, assign2660_e2534_d_n7, assign2660_e2534_d_n8, assign2660_e2534_d_n9,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard41 == 0.0)) {
        let assign2660_e2527: f64 = (-locals.var_dxa);
        let assign2660_e2528: f64 = (assign2660_e2527).exp();
        let assign2660_e2529: f64 = (1.0 + assign2660_e2528);
        let assign2660_e2530: f64 = (assign2660_e2529).ln();
        let assign2660_e2531: f64 = (p.p62 * assign2660_e2530);
        let assign2660_e2532: f64 = (locals.var_ic1c2_iqs + assign2660_e2531);
        (assign2660_e2532, (locals.var_ic1c2_iqs_dn0 + (p.p62 * ((assign2660_e2528 * (-locals.var_dxa_dn0)) / assign2660_e2529))), (locals.var_ic1c2_iqs_dn1 + (p.p62 * ((assign2660_e2528 * (-locals.var_dxa_dn1)) / assign2660_e2529))), (locals.var_ic1c2_iqs_dn3 + (p.p62 * ((assign2660_e2528 * (-locals.var_dxa_dn3)) / assign2660_e2529))), (locals.var_ic1c2_iqs_dn4 + (p.p62 * ((assign2660_e2528 * (-locals.var_dxa_dn4)) / assign2660_e2529))), (locals.var_ic1c2_iqs_dn5 + (p.p62 * ((assign2660_e2528 * (-locals.var_dxa_dn5)) / assign2660_e2529))), (locals.var_ic1c2_iqs_dn6 + (p.p62 * ((assign2660_e2528 * (-locals.var_dxa_dn6)) / assign2660_e2529))), (locals.var_ic1c2_iqs_dn7 + (p.p62 * ((assign2660_e2528 * (-locals.var_dxa_dn7)) / assign2660_e2529))), (locals.var_ic1c2_iqs_dn8 + (p.p62 * ((assign2660_e2528 * (-locals.var_dxa_dn8)) / assign2660_e2529))), (locals.var_ic1c2_iqs_dn9 + (p.p62 * ((assign2660_e2528 * (-locals.var_dxa_dn9)) / assign2660_e2529))),)
    } else {
        (locals.var_alpha1, locals.var_alpha1_dn0, locals.var_alpha1_dn1, locals.var_alpha1_dn3, locals.var_alpha1_dn4, locals.var_alpha1_dn5, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8, locals.var_alpha1_dn9,)
    }
};
        locals.var_alpha1 = assign2660_e2534;
        locals.var_alpha1_dn0 = assign2660_e2534_d_n0;
        locals.var_alpha1_dn1 = assign2660_e2534_d_n1;
        locals.var_alpha1_dn3 = assign2660_e2534_d_n3;
        locals.var_alpha1_dn4 = assign2660_e2534_d_n4;
        locals.var_alpha1_dn5 = assign2660_e2534_d_n5;
        locals.var_alpha1_dn6 = assign2660_e2534_d_n6;
        locals.var_alpha1_dn7 = assign2660_e2534_d_n7;
        locals.var_alpha1_dn8 = assign2660_e2534_d_n8;
        locals.var_alpha1_dn9 = assign2660_e2534_d_n9;

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2670_e2551, assign2670_e2551_d_n0, assign2670_e2551_d_n1, assign2670_e2551_d_n3, assign2670_e2551_d_n4, assign2670_e2551_d_n5, assign2670_e2551_d_n6, assign2670_e2551_d_n7, assign2670_e2551_d_n8, assign2670_e2551_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2670_e2541: f64 = (-1.0);
        let assign2670_e2543: f64 = (assign2670_e2541 / p.p62);
        let assign2670_e2544: f64 = (assign2670_e2543).exp();
        let assign2670_e2545: f64 = (1.0 + assign2670_e2544);
        let assign2670_e2546: f64 = (assign2670_e2545).ln();
        let assign2670_e2547: f64 = (p.p62 * assign2670_e2546);
        let assign2670_e2548: f64 = (1.0 + assign2670_e2547);
        let assign2670_e2549: f64 = (locals.var_alpha1 / assign2670_e2548);
        (assign2670_e2549, (locals.var_alpha1_dn0 / assign2670_e2548), (locals.var_alpha1_dn1 / assign2670_e2548), (locals.var_alpha1_dn3 / assign2670_e2548), (locals.var_alpha1_dn4 / assign2670_e2548), (locals.var_alpha1_dn5 / assign2670_e2548), (locals.var_alpha1_dn6 / assign2670_e2548), (locals.var_alpha1_dn7 / assign2670_e2548), (locals.var_alpha1_dn8 / assign2670_e2548), (locals.var_alpha1_dn9 / assign2670_e2548),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn1, locals.var_alpha_dn3, locals.var_alpha_dn4, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, locals.var_alpha_dn9,)
    }
};
        locals.var_alpha = assign2670_e2551;
        locals.var_alpha_dn0 = assign2670_e2551_d_n0;
        locals.var_alpha_dn1 = assign2670_e2551_d_n1;
        locals.var_alpha_dn3 = assign2670_e2551_d_n3;
        locals.var_alpha_dn4 = assign2670_e2551_d_n4;
        locals.var_alpha_dn5 = assign2670_e2551_d_n5;
        locals.var_alpha_dn6 = assign2670_e2551_d_n6;
        locals.var_alpha_dn7 = assign2670_e2551_d_n7;
        locals.var_alpha_dn8 = assign2670_e2551_d_n8;
        locals.var_alpha_dn9 = assign2670_e2551_d_n9;

        let (assign2680_e2559, assign2680_e2559_d_n0, assign2680_e2559_d_n1, assign2680_e2559_d_n3, assign2680_e2559_d_n4, assign2680_e2559_d_n5, assign2680_e2559_d_n6, assign2680_e2559_d_n7, assign2680_e2559_d_n8, assign2680_e2559_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2680_e2556: f64 = (p.p61 * p.p60);
        let assign2680_e2557: f64 = (locals.var_vqs / assign2680_e2556);
        (assign2680_e2557, (locals.var_vqs_dn0 / assign2680_e2556), (locals.var_vqs_dn1 / assign2680_e2556), (locals.var_vqs_dn3 / assign2680_e2556), (locals.var_vqs_dn4 / assign2680_e2556), (locals.var_vqs_dn5 / assign2680_e2556), (locals.var_vqs_dn6 / assign2680_e2556), (locals.var_vqs_dn7 / assign2680_e2556), (locals.var_vqs_dn8 / assign2680_e2556), (locals.var_vqs_dn9 / assign2680_e2556),)
    } else {
        (locals.var_vyi, locals.var_vyi_dn0, locals.var_vyi_dn1, locals.var_vyi_dn3, locals.var_vyi_dn4, locals.var_vyi_dn5, locals.var_vyi_dn6, locals.var_vyi_dn7, locals.var_vyi_dn8, locals.var_vyi_dn9,)
    }
};
        locals.var_vyi = assign2680_e2559;
        locals.var_vyi_dn0 = assign2680_e2559_d_n0;
        locals.var_vyi_dn1 = assign2680_e2559_d_n1;
        locals.var_vyi_dn3 = assign2680_e2559_d_n3;
        locals.var_vyi_dn4 = assign2680_e2559_d_n4;
        locals.var_vyi_dn5 = assign2680_e2559_d_n5;
        locals.var_vyi_dn6 = assign2680_e2559_d_n6;
        locals.var_vyi_dn7 = assign2680_e2559_d_n7;
        locals.var_vyi_dn8 = assign2680_e2559_d_n8;
        locals.var_vyi_dn9 = assign2680_e2559_d_n9;

        let (assign2690_e2584, assign2690_e2584_d_n0, assign2690_e2584_d_n1, assign2690_e2584_d_n3, assign2690_e2584_d_n4, assign2690_e2584_d_n5, assign2690_e2584_d_n6, assign2690_e2584_d_n7, assign2690_e2584_d_n8, assign2690_e2584_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2690_e2565: f64 = (4.0 * locals.var_alpha);
        let assign2690_e2567: f64 = (assign2690_e2565 * locals.var_vyi);
        let assign2690_e2570: f64 = (1.0 + locals.var_vyi);
        let assign2690_e2571: f64 = (assign2690_e2567 * assign2690_e2570);
        let assign2690_e2572: f64 = (1.0 + assign2690_e2571);
        let assign2690_e2573: f64 = (assign2690_e2572).sqrt();
        let assign2690_e2574: f64 = (1.0 + assign2690_e2573);
        let assign2690_e2577: f64 = (2.0 * locals.var_alpha);
        let assign2690_e2580: f64 = (1.0 + locals.var_vyi);
        let assign2690_e2581: f64 = (assign2690_e2577 * assign2690_e2580);
        let assign2690_e2582: f64 = (assign2690_e2574 / assign2690_e2581);
        (assign2690_e2582, (((((((((4.0 * locals.var_alpha_dn0) * locals.var_vyi) + (assign2690_e2565 * locals.var_vyi_dn0)) * assign2690_e2570) + (assign2690_e2567 * locals.var_vyi_dn0)) / (2.0 * assign2690_e2573)) * assign2690_e2581) - (assign2690_e2574 * (((2.0 * locals.var_alpha_dn0) * assign2690_e2580) + (assign2690_e2577 * locals.var_vyi_dn0)))) / (assign2690_e2581 * assign2690_e2581)), (((((((((4.0 * locals.var_alpha_dn1) * locals.var_vyi) + (assign2690_e2565 * locals.var_vyi_dn1)) * assign2690_e2570) + (assign2690_e2567 * locals.var_vyi_dn1)) / (2.0 * assign2690_e2573)) * assign2690_e2581) - (assign2690_e2574 * (((2.0 * locals.var_alpha_dn1) * assign2690_e2580) + (assign2690_e2577 * locals.var_vyi_dn1)))) / (assign2690_e2581 * assign2690_e2581)), (((((((((4.0 * locals.var_alpha_dn3) * locals.var_vyi) + (assign2690_e2565 * locals.var_vyi_dn3)) * assign2690_e2570) + (assign2690_e2567 * locals.var_vyi_dn3)) / (2.0 * assign2690_e2573)) * assign2690_e2581) - (assign2690_e2574 * (((2.0 * locals.var_alpha_dn3) * assign2690_e2580) + (assign2690_e2577 * locals.var_vyi_dn3)))) / (assign2690_e2581 * assign2690_e2581)), (((((((((4.0 * locals.var_alpha_dn4) * locals.var_vyi) + (assign2690_e2565 * locals.var_vyi_dn4)) * assign2690_e2570) + (assign2690_e2567 * locals.var_vyi_dn4)) / (2.0 * assign2690_e2573)) * assign2690_e2581) - (assign2690_e2574 * (((2.0 * locals.var_alpha_dn4) * assign2690_e2580) + (assign2690_e2577 * locals.var_vyi_dn4)))) / (assign2690_e2581 * assign2690_e2581)), (((((((((4.0 * locals.var_alpha_dn5) * locals.var_vyi) + (assign2690_e2565 * locals.var_vyi_dn5)) * assign2690_e2570) + (assign2690_e2567 * locals.var_vyi_dn5)) / (2.0 * assign2690_e2573)) * assign2690_e2581) - (assign2690_e2574 * (((2.0 * locals.var_alpha_dn5) * assign2690_e2580) + (assign2690_e2577 * locals.var_vyi_dn5)))) / (assign2690_e2581 * assign2690_e2581)), (((((((((4.0 * locals.var_alpha_dn6) * locals.var_vyi) + (assign2690_e2565 * locals.var_vyi_dn6)) * assign2690_e2570) + (assign2690_e2567 * locals.var_vyi_dn6)) / (2.0 * assign2690_e2573)) * assign2690_e2581) - (assign2690_e2574 * (((2.0 * locals.var_alpha_dn6) * assign2690_e2580) + (assign2690_e2577 * locals.var_vyi_dn6)))) / (assign2690_e2581 * assign2690_e2581)), (((((((((4.0 * locals.var_alpha_dn7) * locals.var_vyi) + (assign2690_e2565 * locals.var_vyi_dn7)) * assign2690_e2570) + (assign2690_e2567 * locals.var_vyi_dn7)) / (2.0 * assign2690_e2573)) * assign2690_e2581) - (assign2690_e2574 * (((2.0 * locals.var_alpha_dn7) * assign2690_e2580) + (assign2690_e2577 * locals.var_vyi_dn7)))) / (assign2690_e2581 * assign2690_e2581)), (((((((((4.0 * locals.var_alpha_dn8) * locals.var_vyi) + (assign2690_e2565 * locals.var_vyi_dn8)) * assign2690_e2570) + (assign2690_e2567 * locals.var_vyi_dn8)) / (2.0 * assign2690_e2573)) * assign2690_e2581) - (assign2690_e2574 * (((2.0 * locals.var_alpha_dn8) * assign2690_e2580) + (assign2690_e2577 * locals.var_vyi_dn8)))) / (assign2690_e2581 * assign2690_e2581)), (((((((((4.0 * locals.var_alpha_dn9) * locals.var_vyi) + (assign2690_e2565 * locals.var_vyi_dn9)) * assign2690_e2570) + (assign2690_e2567 * locals.var_vyi_dn9)) / (2.0 * assign2690_e2573)) * assign2690_e2581) - (assign2690_e2574 * (((2.0 * locals.var_alpha_dn9) * assign2690_e2580) + (assign2690_e2577 * locals.var_vyi_dn9)))) / (assign2690_e2581 * assign2690_e2581)),)
    } else {
        (locals.var_yi, locals.var_yi_dn0, locals.var_yi_dn1, locals.var_yi_dn3, locals.var_yi_dn4, locals.var_yi_dn5, locals.var_yi_dn6, locals.var_yi_dn7, locals.var_yi_dn8, locals.var_yi_dn9,)
    }
};
        locals.var_yi = assign2690_e2584;
        locals.var_yi_dn0 = assign2690_e2584_d_n0;
        locals.var_yi_dn1 = assign2690_e2584_d_n1;
        locals.var_yi_dn3 = assign2690_e2584_d_n3;
        locals.var_yi_dn4 = assign2690_e2584_d_n4;
        locals.var_yi_dn5 = assign2690_e2584_d_n5;
        locals.var_yi_dn6 = assign2690_e2584_d_n6;
        locals.var_yi_dn7 = assign2690_e2584_d_n7;
        locals.var_yi_dn8 = assign2690_e2584_d_n8;
        locals.var_yi_dn9 = assign2690_e2584_d_n9;

        let (assign2700_e2600, assign2700_e2600_d_n0, assign2700_e2600_d_n1, assign2700_e2600_d_n3, assign2700_e2600_d_n4, assign2700_e2600_d_n5, assign2700_e2600_d_n6, assign2700_e2600_d_n7, assign2700_e2600_d_n8, assign2700_e2600_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2700_e2588: f64 = (1.0 - locals.var_yi);
        let assign2700_e2591: f64 = (locals.var_pw * locals.var_yi);
        let assign2700_e2592: f64 = (assign2700_e2588 + assign2700_e2591);
        let assign2700_e2596: f64 = (locals.var_pw * locals.var_yi);
        let assign2700_e2597: f64 = (1.0 + assign2700_e2596);
        let assign2700_e2598: f64 = (assign2700_e2592 / assign2700_e2597);
        (assign2700_e2598, (((((-locals.var_yi_dn0) + ((locals.var_pw_dn0 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn0))) * assign2700_e2597) - (assign2700_e2592 * ((locals.var_pw_dn0 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn0)))) / (assign2700_e2597 * assign2700_e2597)), (((((-locals.var_yi_dn1) + ((locals.var_pw_dn1 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn1))) * assign2700_e2597) - (assign2700_e2592 * ((locals.var_pw_dn1 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn1)))) / (assign2700_e2597 * assign2700_e2597)), (((((-locals.var_yi_dn3) + ((locals.var_pw_dn3 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn3))) * assign2700_e2597) - (assign2700_e2592 * ((locals.var_pw_dn3 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn3)))) / (assign2700_e2597 * assign2700_e2597)), (((((-locals.var_yi_dn4) + ((locals.var_pw_dn4 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn4))) * assign2700_e2597) - (assign2700_e2592 * ((locals.var_pw_dn4 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn4)))) / (assign2700_e2597 * assign2700_e2597)), (((((-locals.var_yi_dn5) + ((locals.var_pw_dn5 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn5))) * assign2700_e2597) - (assign2700_e2592 * ((locals.var_pw_dn5 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn5)))) / (assign2700_e2597 * assign2700_e2597)), (((((-locals.var_yi_dn6) + ((locals.var_pw_dn6 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn6))) * assign2700_e2597) - (assign2700_e2592 * ((locals.var_pw_dn6 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn6)))) / (assign2700_e2597 * assign2700_e2597)), (((((-locals.var_yi_dn7) + ((locals.var_pw_dn7 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn7))) * assign2700_e2597) - (assign2700_e2592 * ((locals.var_pw_dn7 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn7)))) / (assign2700_e2597 * assign2700_e2597)), (((((-locals.var_yi_dn8) + ((locals.var_pw_dn8 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn8))) * assign2700_e2597) - (assign2700_e2592 * ((locals.var_pw_dn8 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn8)))) / (assign2700_e2597 * assign2700_e2597)), (((((-locals.var_yi_dn9) + ((locals.var_pw_dn9 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn9))) * assign2700_e2597) - (assign2700_e2592 * ((locals.var_pw_dn9 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn9)))) / (assign2700_e2597 * assign2700_e2597)),)
    } else {
        (locals.var_xi_w, locals.var_xi_w_dn0, locals.var_xi_w_dn1, locals.var_xi_w_dn3, locals.var_xi_w_dn4, locals.var_xi_w_dn5, locals.var_xi_w_dn6, locals.var_xi_w_dn7, locals.var_xi_w_dn8, locals.var_xi_w_dn9,)
    }
};
        locals.var_xi_w = assign2700_e2600;
        locals.var_xi_w_dn0 = assign2700_e2600_d_n0;
        locals.var_xi_w_dn1 = assign2700_e2600_d_n1;
        locals.var_xi_w_dn3 = assign2700_e2600_d_n3;
        locals.var_xi_w_dn4 = assign2700_e2600_d_n4;
        locals.var_xi_w_dn5 = assign2700_e2600_d_n5;
        locals.var_xi_w_dn6 = assign2700_e2600_d_n6;
        locals.var_xi_w_dn7 = assign2700_e2600_d_n7;
        locals.var_xi_w_dn8 = assign2700_e2600_d_n8;
        locals.var_xi_w_dn9 = assign2700_e2600_d_n9;

        let (assign2710_e2612, assign2710_e2612_d_n0, assign2710_e2612_d_n1, assign2710_e2612_d_n3, assign2710_e2612_d_n4, assign2710_e2612_d_n5, assign2710_e2612_d_n6, assign2710_e2612_d_n7, assign2710_e2612_d_n8, assign2710_e2612_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2710_e2604: f64 = (0.5 * locals.var_ic1c2);
        let assign2710_e2606: f64 = (assign2710_e2604 * locals.var_rcv_t);
        let assign2710_e2608: f64 = (assign2710_e2606 * locals.var_xi_w);
        let assign2710_e2610: f64 = (assign2710_e2608 * locals.var_vtinv);
        (assign2710_e2610, (((((0.5 * locals.var_ic1c2_dn0) * locals.var_rcv_t) * locals.var_xi_w) + (assign2710_e2606 * locals.var_xi_w_dn0)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn1) * locals.var_rcv_t) * locals.var_xi_w) + (assign2710_e2606 * locals.var_xi_w_dn1)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn3) * locals.var_rcv_t) * locals.var_xi_w) + (assign2710_e2606 * locals.var_xi_w_dn3)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn4) * locals.var_rcv_t) * locals.var_xi_w) + (assign2710_e2606 * locals.var_xi_w_dn4)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn5) * locals.var_rcv_t) * locals.var_xi_w) + (assign2710_e2606 * locals.var_xi_w_dn5)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn6) * locals.var_rcv_t) * locals.var_xi_w) + (assign2710_e2606 * locals.var_xi_w_dn6)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn7) * locals.var_rcv_t) * locals.var_xi_w) + (assign2710_e2606 * locals.var_xi_w_dn7)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn8) * locals.var_rcv_t) * locals.var_xi_w) + (assign2710_e2606 * locals.var_xi_w_dn8)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn9) * locals.var_rcv_t) * locals.var_xi_w) + (assign2710_e2606 * locals.var_xi_w_dn9)) * locals.var_vtinv),)
    } else {
        (locals.var_gp0, locals.var_gp0_dn0, locals.var_gp0_dn1, locals.var_gp0_dn3, locals.var_gp0_dn4, locals.var_gp0_dn5, locals.var_gp0_dn6, locals.var_gp0_dn7, locals.var_gp0_dn8, locals.var_gp0_dn9,)
    }
};
        locals.var_gp0 = assign2710_e2612;
        locals.var_gp0_dn0 = assign2710_e2612_d_n0;
        locals.var_gp0_dn1 = assign2710_e2612_d_n1;
        locals.var_gp0_dn3 = assign2710_e2612_d_n3;
        locals.var_gp0_dn4 = assign2710_e2612_d_n4;
        locals.var_gp0_dn5 = assign2710_e2612_d_n5;
        locals.var_gp0_dn6 = assign2710_e2612_d_n6;
        locals.var_gp0_dn7 = assign2710_e2612_d_n7;
        locals.var_gp0_dn8 = assign2710_e2612_d_n8;
        locals.var_gp0_dn9 = assign2710_e2612_d_n9;

        let (assign2720_e2626, assign2720_e2626_d_n0, assign2720_e2626_d_n1, assign2720_e2626_d_n3, assign2720_e2626_d_n4, assign2720_e2626_d_n5, assign2720_e2626_d_n6, assign2720_e2626_d_n7, assign2720_e2626_d_n8, assign2720_e2626_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2720_e2616: f64 = (2.0 * locals.var_gp0);
        let assign2720_e2620: f64 = (locals.var_pw + locals.var_gp0);
        let assign2720_e2622: f64 = (assign2720_e2620 + 1.0);
        let assign2720_e2623: f64 = (locals.var_pw * assign2720_e2622);
        let assign2720_e2624: f64 = (assign2720_e2616 + assign2720_e2623);
        (assign2720_e2624, ((2.0 * locals.var_gp0_dn0) + ((locals.var_pw_dn0 * assign2720_e2622) + (locals.var_pw * (locals.var_pw_dn0 + locals.var_gp0_dn0)))), ((2.0 * locals.var_gp0_dn1) + ((locals.var_pw_dn1 * assign2720_e2622) + (locals.var_pw * (locals.var_pw_dn1 + locals.var_gp0_dn1)))), ((2.0 * locals.var_gp0_dn3) + ((locals.var_pw_dn3 * assign2720_e2622) + (locals.var_pw * (locals.var_pw_dn3 + locals.var_gp0_dn3)))), ((2.0 * locals.var_gp0_dn4) + ((locals.var_pw_dn4 * assign2720_e2622) + (locals.var_pw * (locals.var_pw_dn4 + locals.var_gp0_dn4)))), ((2.0 * locals.var_gp0_dn5) + ((locals.var_pw_dn5 * assign2720_e2622) + (locals.var_pw * (locals.var_pw_dn5 + locals.var_gp0_dn5)))), ((2.0 * locals.var_gp0_dn6) + ((locals.var_pw_dn6 * assign2720_e2622) + (locals.var_pw * (locals.var_pw_dn6 + locals.var_gp0_dn6)))), ((2.0 * locals.var_gp0_dn7) + ((locals.var_pw_dn7 * assign2720_e2622) + (locals.var_pw * (locals.var_pw_dn7 + locals.var_gp0_dn7)))), ((2.0 * locals.var_gp0_dn8) + ((locals.var_pw_dn8 * assign2720_e2622) + (locals.var_pw * (locals.var_pw_dn8 + locals.var_gp0_dn8)))), ((2.0 * locals.var_gp0_dn9) + ((locals.var_pw_dn9 * assign2720_e2622) + (locals.var_pw * (locals.var_pw_dn9 + locals.var_gp0_dn9)))),)
    } else {
        (locals.var_gp0_help, locals.var_gp0_help_dn0, locals.var_gp0_help_dn1, locals.var_gp0_help_dn3, locals.var_gp0_help_dn4, locals.var_gp0_help_dn5, locals.var_gp0_help_dn6, locals.var_gp0_help_dn7, locals.var_gp0_help_dn8, locals.var_gp0_help_dn9,)
    }
};
        locals.var_gp0_help = assign2720_e2626;
        locals.var_gp0_help_dn0 = assign2720_e2626_d_n0;
        locals.var_gp0_help_dn1 = assign2720_e2626_d_n1;
        locals.var_gp0_help_dn3 = assign2720_e2626_d_n3;
        locals.var_gp0_help_dn4 = assign2720_e2626_d_n4;
        locals.var_gp0_help_dn5 = assign2720_e2626_d_n5;
        locals.var_gp0_help_dn6 = assign2720_e2626_d_n6;
        locals.var_gp0_help_dn7 = assign2720_e2626_d_n7;
        locals.var_gp0_help_dn8 = assign2720_e2626_d_n8;
        locals.var_gp0_help_dn9 = assign2720_e2626_d_n9;

        let (assign2730_e2634, assign2730_e2634_d_n0, assign2730_e2634_d_n1, assign2730_e2634_d_n3, assign2730_e2634_d_n4, assign2730_e2634_d_n5, assign2730_e2634_d_n6, assign2730_e2634_d_n7, assign2730_e2634_d_n8, assign2730_e2634_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2730_e2631: f64 = (locals.var_gp0 - 1.0);
        let assign2730_e2632: f64 = (0.5 * assign2730_e2631);
        (assign2730_e2632, (0.5 * locals.var_gp0_dn0), (0.5 * locals.var_gp0_dn1), (0.5 * locals.var_gp0_dn3), (0.5 * locals.var_gp0_dn4), (0.5 * locals.var_gp0_dn5), (0.5 * locals.var_gp0_dn6), (0.5 * locals.var_gp0_dn7), (0.5 * locals.var_gp0_dn8), (0.5 * locals.var_gp0_dn9),)
    } else {
        (locals.var_gp02, locals.var_gp02_dn0, locals.var_gp02_dn1, locals.var_gp02_dn3, locals.var_gp02_dn4, locals.var_gp02_dn5, locals.var_gp02_dn6, locals.var_gp02_dn7, locals.var_gp02_dn8, locals.var_gp02_dn9,)
    }
};
        locals.var_gp02 = assign2730_e2634;
        locals.var_gp02_dn0 = assign2730_e2634_d_n0;
        locals.var_gp02_dn1 = assign2730_e2634_d_n1;
        locals.var_gp02_dn3 = assign2730_e2634_d_n3;
        locals.var_gp02_dn4 = assign2730_e2634_d_n4;
        locals.var_gp02_dn5 = assign2730_e2634_d_n5;
        locals.var_gp02_dn6 = assign2730_e2634_d_n6;
        locals.var_gp02_dn7 = assign2730_e2634_d_n7;
        locals.var_gp02_dn8 = assign2730_e2634_d_n8;
        locals.var_gp02_dn9 = assign2730_e2634_d_n9;

        let (assign2740_e2642, assign2740_e2642_d_n0, assign2740_e2642_d_n1, assign2740_e2642_d_n3, assign2740_e2642_d_n4, assign2740_e2642_d_n5, assign2740_e2642_d_n6, assign2740_e2642_d_n7, assign2740_e2642_d_n8, assign2740_e2642_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2740_e2638: f64 = (locals.var_gp02 * locals.var_gp02);
        let assign2740_e2640: f64 = (assign2740_e2638 + locals.var_gp0_help);
        (assign2740_e2640, (((locals.var_gp02_dn0 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn0)) + locals.var_gp0_help_dn0), (((locals.var_gp02_dn1 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn1)) + locals.var_gp0_help_dn1), (((locals.var_gp02_dn3 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn3)) + locals.var_gp0_help_dn3), (((locals.var_gp02_dn4 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn4)) + locals.var_gp0_help_dn4), (((locals.var_gp02_dn5 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn5)) + locals.var_gp0_help_dn5), (((locals.var_gp02_dn6 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn6)) + locals.var_gp0_help_dn6), (((locals.var_gp02_dn7 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn7)) + locals.var_gp0_help_dn7), (((locals.var_gp02_dn8 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn8)) + locals.var_gp0_help_dn8), (((locals.var_gp02_dn9 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn9)) + locals.var_gp0_help_dn9),)
    } else {
        (locals.var_sqr_arg, locals.var_sqr_arg_dn0, locals.var_sqr_arg_dn1, locals.var_sqr_arg_dn3, locals.var_sqr_arg_dn4, locals.var_sqr_arg_dn5, locals.var_sqr_arg_dn6, locals.var_sqr_arg_dn7, locals.var_sqr_arg_dn8, locals.var_sqr_arg_dn9,)
    }
};
        locals.var_sqr_arg = assign2740_e2642;
        locals.var_sqr_arg_dn0 = assign2740_e2642_d_n0;
        locals.var_sqr_arg_dn1 = assign2740_e2642_d_n1;
        locals.var_sqr_arg_dn3 = assign2740_e2642_d_n3;
        locals.var_sqr_arg_dn4 = assign2740_e2642_d_n4;
        locals.var_sqr_arg_dn5 = assign2740_e2642_d_n5;
        locals.var_sqr_arg_dn6 = assign2740_e2642_d_n6;
        locals.var_sqr_arg_dn7 = assign2740_e2642_d_n7;
        locals.var_sqr_arg_dn8 = assign2740_e2642_d_n8;
        locals.var_sqr_arg_dn9 = assign2740_e2642_d_n9;

        let assign2750_e2645: f64 = if locals.var_gp0 >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign2750_e2645;

        let (assign2760_e2654, assign2760_e2654_d_n0, assign2760_e2654_d_n1, assign2760_e2654_d_n3, assign2760_e2654_d_n4, assign2760_e2654_d_n5, assign2760_e2654_d_n6, assign2760_e2654_d_n7, assign2760_e2654_d_n8, assign2760_e2654_d_n9,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard42 != 0.0)) {
        let assign2760_e2651: f64 = (locals.var_sqr_arg).sqrt();
        let assign2760_e2652: f64 = (locals.var_gp02 + assign2760_e2651);
        (assign2760_e2652, (locals.var_gp02_dn0 + (locals.var_sqr_arg_dn0 / (2.0 * assign2760_e2651))), (locals.var_gp02_dn1 + (locals.var_sqr_arg_dn1 / (2.0 * assign2760_e2651))), (locals.var_gp02_dn3 + (locals.var_sqr_arg_dn3 / (2.0 * assign2760_e2651))), (locals.var_gp02_dn4 + (locals.var_sqr_arg_dn4 / (2.0 * assign2760_e2651))), (locals.var_gp02_dn5 + (locals.var_sqr_arg_dn5 / (2.0 * assign2760_e2651))), (locals.var_gp02_dn6 + (locals.var_sqr_arg_dn6 / (2.0 * assign2760_e2651))), (locals.var_gp02_dn7 + (locals.var_sqr_arg_dn7 / (2.0 * assign2760_e2651))), (locals.var_gp02_dn8 + (locals.var_sqr_arg_dn8 / (2.0 * assign2760_e2651))), (locals.var_gp02_dn9 + (locals.var_sqr_arg_dn9 / (2.0 * assign2760_e2651))),)
    } else {
        (locals.var_p0star, locals.var_p0star_dn0, locals.var_p0star_dn1, locals.var_p0star_dn3, locals.var_p0star_dn4, locals.var_p0star_dn5, locals.var_p0star_dn6, locals.var_p0star_dn7, locals.var_p0star_dn8, locals.var_p0star_dn9,)
    }
};
        locals.var_p0star = assign2760_e2654;
        locals.var_p0star_dn0 = assign2760_e2654_d_n0;
        locals.var_p0star_dn1 = assign2760_e2654_d_n1;
        locals.var_p0star_dn3 = assign2760_e2654_d_n3;
        locals.var_p0star_dn4 = assign2760_e2654_d_n4;
        locals.var_p0star_dn5 = assign2760_e2654_d_n5;
        locals.var_p0star_dn6 = assign2760_e2654_d_n6;
        locals.var_p0star_dn7 = assign2760_e2654_d_n7;
        locals.var_p0star_dn8 = assign2760_e2654_d_n8;
        locals.var_p0star_dn9 = assign2760_e2654_d_n9;

        let (assign2770_e2666, assign2770_e2666_d_n0, assign2770_e2666_d_n1, assign2770_e2666_d_n3, assign2770_e2666_d_n4, assign2770_e2666_d_n5, assign2770_e2666_d_n6, assign2770_e2666_d_n7, assign2770_e2666_d_n8, assign2770_e2666_d_n9,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard42 == 0.0)) {
        let assign2770_e2661: f64 = (locals.var_sqr_arg).sqrt();
        let assign2770_e2663: f64 = (assign2770_e2661 - locals.var_gp02);
        let assign2770_e2664: f64 = (locals.var_gp0_help / assign2770_e2663);
        (assign2770_e2664, (((locals.var_gp0_help_dn0 * assign2770_e2663) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn0 / (2.0 * assign2770_e2661)) - locals.var_gp02_dn0))) / (assign2770_e2663 * assign2770_e2663)), (((locals.var_gp0_help_dn1 * assign2770_e2663) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn1 / (2.0 * assign2770_e2661)) - locals.var_gp02_dn1))) / (assign2770_e2663 * assign2770_e2663)), (((locals.var_gp0_help_dn3 * assign2770_e2663) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn3 / (2.0 * assign2770_e2661)) - locals.var_gp02_dn3))) / (assign2770_e2663 * assign2770_e2663)), (((locals.var_gp0_help_dn4 * assign2770_e2663) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn4 / (2.0 * assign2770_e2661)) - locals.var_gp02_dn4))) / (assign2770_e2663 * assign2770_e2663)), (((locals.var_gp0_help_dn5 * assign2770_e2663) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn5 / (2.0 * assign2770_e2661)) - locals.var_gp02_dn5))) / (assign2770_e2663 * assign2770_e2663)), (((locals.var_gp0_help_dn6 * assign2770_e2663) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn6 / (2.0 * assign2770_e2661)) - locals.var_gp02_dn6))) / (assign2770_e2663 * assign2770_e2663)), (((locals.var_gp0_help_dn7 * assign2770_e2663) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn7 / (2.0 * assign2770_e2661)) - locals.var_gp02_dn7))) / (assign2770_e2663 * assign2770_e2663)), (((locals.var_gp0_help_dn8 * assign2770_e2663) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn8 / (2.0 * assign2770_e2661)) - locals.var_gp02_dn8))) / (assign2770_e2663 * assign2770_e2663)), (((locals.var_gp0_help_dn9 * assign2770_e2663) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn9 / (2.0 * assign2770_e2661)) - locals.var_gp02_dn9))) / (assign2770_e2663 * assign2770_e2663)),)
    } else {
        (locals.var_p0star, locals.var_p0star_dn0, locals.var_p0star_dn1, locals.var_p0star_dn3, locals.var_p0star_dn4, locals.var_p0star_dn5, locals.var_p0star_dn6, locals.var_p0star_dn7, locals.var_p0star_dn8, locals.var_p0star_dn9,)
    }
};
        locals.var_p0star = assign2770_e2666;
        locals.var_p0star_dn0 = assign2770_e2666_d_n0;
        locals.var_p0star_dn1 = assign2770_e2666_d_n1;
        locals.var_p0star_dn3 = assign2770_e2666_d_n3;
        locals.var_p0star_dn4 = assign2770_e2666_d_n4;
        locals.var_p0star_dn5 = assign2770_e2666_d_n5;
        locals.var_p0star_dn6 = assign2770_e2666_d_n6;
        locals.var_p0star_dn7 = assign2770_e2666_d_n7;
        locals.var_p0star_dn8 = assign2770_e2666_d_n8;
        locals.var_p0star_dn9 = assign2770_e2666_d_n9;

        let assign2780_e2669: f64 = if locals.var_p0star < p.p135 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign2780_e2669;

        let (assign2790_e2675, assign2790_e2675_d_n0, assign2790_e2675_d_n1, assign2790_e2675_d_n3, assign2790_e2675_d_n4, assign2790_e2675_d_n5, assign2790_e2675_d_n6, assign2790_e2675_d_n7, assign2790_e2675_d_n8, assign2790_e2675_d_n9,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard43 != 0.0)) {
        (p.p135, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_p0star, locals.var_p0star_dn0, locals.var_p0star_dn1, locals.var_p0star_dn3, locals.var_p0star_dn4, locals.var_p0star_dn5, locals.var_p0star_dn6, locals.var_p0star_dn7, locals.var_p0star_dn8, locals.var_p0star_dn9,)
    }
};
        locals.var_p0star = assign2790_e2675;
        locals.var_p0star_dn0 = assign2790_e2675_d_n0;
        locals.var_p0star_dn1 = assign2790_e2675_d_n1;
        locals.var_p0star_dn3 = assign2790_e2675_d_n3;
        locals.var_p0star_dn4 = assign2790_e2675_d_n4;
        locals.var_p0star_dn5 = assign2790_e2675_d_n5;
        locals.var_p0star_dn6 = assign2790_e2675_d_n6;
        locals.var_p0star_dn7 = assign2790_e2675_d_n7;
        locals.var_p0star_dn8 = assign2790_e2675_d_n8;
        locals.var_p0star_dn9 = assign2790_e2675_d_n9;

        let (assign2800_e2688, assign2800_e2688_d_n0, assign2800_e2688_d_n1, assign2800_e2688_d_n3, assign2800_e2688_d_n4, assign2800_e2688_d_n5, assign2800_e2688_d_n6, assign2800_e2688_d_n7, assign2800_e2688_d_n8, assign2800_e2688_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2800_e2680: f64 = (locals.var_p0star + 1.0);
        let assign2800_e2681: f64 = (locals.var_p0star * assign2800_e2680);
        let assign2800_e2684: f64 = (locals.var_vdc_t * locals.var_vtinv);
        let assign2800_e2685: f64 = (assign2800_e2684).exp();
        let assign2800_e2686: f64 = (assign2800_e2681 * assign2800_e2685);
        (assign2800_e2686, ((((locals.var_p0star_dn0 * assign2800_e2680) + (locals.var_p0star * locals.var_p0star_dn0)) * assign2800_e2685) + (assign2800_e2681 * (assign2800_e2685 * (locals.var_vdc_t_dn0 * locals.var_vtinv)))), ((((locals.var_p0star_dn1 * assign2800_e2680) + (locals.var_p0star * locals.var_p0star_dn1)) * assign2800_e2685) + (assign2800_e2681 * (assign2800_e2685 * (locals.var_vdc_t_dn1 * locals.var_vtinv)))), ((((locals.var_p0star_dn3 * assign2800_e2680) + (locals.var_p0star * locals.var_p0star_dn3)) * assign2800_e2685) + (assign2800_e2681 * (assign2800_e2685 * (locals.var_vdc_t_dn3 * locals.var_vtinv)))), ((((locals.var_p0star_dn4 * assign2800_e2680) + (locals.var_p0star * locals.var_p0star_dn4)) * assign2800_e2685) + (assign2800_e2681 * (assign2800_e2685 * (locals.var_vdc_t_dn4 * locals.var_vtinv)))), ((((locals.var_p0star_dn5 * assign2800_e2680) + (locals.var_p0star * locals.var_p0star_dn5)) * assign2800_e2685) + (assign2800_e2681 * (assign2800_e2685 * (locals.var_vdc_t_dn5 * locals.var_vtinv)))), ((((locals.var_p0star_dn6 * assign2800_e2680) + (locals.var_p0star * locals.var_p0star_dn6)) * assign2800_e2685) + (assign2800_e2681 * (assign2800_e2685 * (locals.var_vdc_t_dn6 * locals.var_vtinv)))), ((((locals.var_p0star_dn7 * assign2800_e2680) + (locals.var_p0star * locals.var_p0star_dn7)) * assign2800_e2685) + (assign2800_e2681 * (assign2800_e2685 * (locals.var_vdc_t_dn7 * locals.var_vtinv)))), ((((locals.var_p0star_dn8 * assign2800_e2680) + (locals.var_p0star * locals.var_p0star_dn8)) * assign2800_e2685) + (assign2800_e2681 * (assign2800_e2685 * (locals.var_vdc_t_dn8 * locals.var_vtinv)))), ((((locals.var_p0star_dn9 * assign2800_e2680) + (locals.var_p0star * locals.var_p0star_dn9)) * assign2800_e2685) + (assign2800_e2681 * (assign2800_e2685 * (locals.var_vdc_t_dn9 * locals.var_vtinv)))),)
    } else {
        (locals.var_evb2c2star, locals.var_evb2c2star_dn0, locals.var_evb2c2star_dn1, locals.var_evb2c2star_dn3, locals.var_evb2c2star_dn4, locals.var_evb2c2star_dn5, locals.var_evb2c2star_dn6, locals.var_evb2c2star_dn7, locals.var_evb2c2star_dn8, locals.var_evb2c2star_dn9,)
    }
};
        locals.var_evb2c2star = assign2800_e2688;
        locals.var_evb2c2star_dn0 = assign2800_e2688_d_n0;
        locals.var_evb2c2star_dn1 = assign2800_e2688_d_n1;
        locals.var_evb2c2star_dn3 = assign2800_e2688_d_n3;
        locals.var_evb2c2star_dn4 = assign2800_e2688_d_n4;
        locals.var_evb2c2star_dn5 = assign2800_e2688_d_n5;
        locals.var_evb2c2star_dn6 = assign2800_e2688_d_n6;
        locals.var_evb2c2star_dn7 = assign2800_e2688_d_n7;
        locals.var_evb2c2star_dn8 = assign2800_e2688_d_n8;
        locals.var_evb2c2star_dn9 = assign2800_e2688_d_n9;

        let (assign2810_e2698, assign2810_e2698_d_n0, assign2810_e2698_d_n1, assign2810_e2698_d_n3, assign2810_e2698_d_n4, assign2810_e2698_d_n5, assign2810_e2698_d_n6, assign2810_e2698_d_n7, assign2810_e2698_d_n8, assign2810_e2698_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2810_e2692: f64 = (0.5 * p.p60);
        let assign2810_e2695: f64 = (locals.var_ic1c2 - p.p61);
        let assign2810_e2696: f64 = (assign2810_e2692 * assign2810_e2695);
        (assign2810_e2696, (assign2810_e2692 * locals.var_ic1c2_dn0), (assign2810_e2692 * locals.var_ic1c2_dn1), (assign2810_e2692 * locals.var_ic1c2_dn3), (assign2810_e2692 * locals.var_ic1c2_dn4), (assign2810_e2692 * locals.var_ic1c2_dn5), (assign2810_e2692 * locals.var_ic1c2_dn6), (assign2810_e2692 * locals.var_ic1c2_dn7), (assign2810_e2692 * locals.var_ic1c2_dn8), (assign2810_e2692 * locals.var_ic1c2_dn9),)
    } else {
        (locals.var_b1, locals.var_b1_dn0, locals.var_b1_dn1, locals.var_b1_dn3, locals.var_b1_dn4, locals.var_b1_dn5, locals.var_b1_dn6, locals.var_b1_dn7, locals.var_b1_dn8, locals.var_b1_dn9,)
    }
};
        locals.var_b1 = assign2810_e2698;
        locals.var_b1_dn0 = assign2810_e2698_d_n0;
        locals.var_b1_dn1 = assign2810_e2698_d_n1;
        locals.var_b1_dn3 = assign2810_e2698_d_n3;
        locals.var_b1_dn4 = assign2810_e2698_d_n4;
        locals.var_b1_dn5 = assign2810_e2698_d_n5;
        locals.var_b1_dn6 = assign2810_e2698_d_n6;
        locals.var_b1_dn7 = assign2810_e2698_d_n7;
        locals.var_b1_dn8 = assign2810_e2698_d_n8;
        locals.var_b1_dn9 = assign2810_e2698_d_n9;

        let (assign2820_e2708, assign2820_e2708_d_n0, assign2820_e2708_d_n1, assign2820_e2708_d_n3, assign2820_e2708_d_n4, assign2820_e2708_d_n5, assign2820_e2708_d_n6, assign2820_e2708_d_n7, assign2820_e2708_d_n8, assign2820_e2708_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2820_e2702: f64 = (p.p60 * locals.var_rcv_t);
        let assign2820_e2704: f64 = (assign2820_e2702 * p.p61);
        let assign2820_e2706: f64 = (assign2820_e2704 * locals.var_ic1c2);
        (assign2820_e2706, (assign2820_e2704 * locals.var_ic1c2_dn0), (assign2820_e2704 * locals.var_ic1c2_dn1), (assign2820_e2704 * locals.var_ic1c2_dn3), (assign2820_e2704 * locals.var_ic1c2_dn4), (assign2820_e2704 * locals.var_ic1c2_dn5), (assign2820_e2704 * locals.var_ic1c2_dn6), (assign2820_e2704 * locals.var_ic1c2_dn7), (assign2820_e2704 * locals.var_ic1c2_dn8), (assign2820_e2704 * locals.var_ic1c2_dn9),)
    } else {
        (locals.var_b2, locals.var_b2_dn0, locals.var_b2_dn1, locals.var_b2_dn3, locals.var_b2_dn4, locals.var_b2_dn5, locals.var_b2_dn6, locals.var_b2_dn7, locals.var_b2_dn8, locals.var_b2_dn9,)
    }
};
        locals.var_b2 = assign2820_e2708;
        locals.var_b2_dn0 = assign2820_e2708_d_n0;
        locals.var_b2_dn1 = assign2820_e2708_d_n1;
        locals.var_b2_dn3 = assign2820_e2708_d_n3;
        locals.var_b2_dn4 = assign2820_e2708_d_n4;
        locals.var_b2_dn5 = assign2820_e2708_d_n5;
        locals.var_b2_dn6 = assign2820_e2708_d_n6;
        locals.var_b2_dn7 = assign2820_e2708_d_n7;
        locals.var_b2_dn8 = assign2820_e2708_d_n8;
        locals.var_b2_dn9 = assign2820_e2708_d_n9;

        let (assign2830_e2719, assign2830_e2719_d_n0, assign2830_e2719_d_n1, assign2830_e2719_d_n3, assign2830_e2719_d_n4, assign2830_e2719_d_n5, assign2830_e2719_d_n6, assign2830_e2719_d_n7, assign2830_e2719_d_n8, assign2830_e2719_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2830_e2713: f64 = (locals.var_b1 * locals.var_b1);
        let assign2830_e2715: f64 = (assign2830_e2713 + locals.var_b2);
        let assign2830_e2716: f64 = (assign2830_e2715).sqrt();
        let assign2830_e2717: f64 = (locals.var_b1 + assign2830_e2716);
        (assign2830_e2717, (locals.var_b1_dn0 + ((((locals.var_b1_dn0 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn0)) + locals.var_b2_dn0) / (2.0 * assign2830_e2716))), (locals.var_b1_dn1 + ((((locals.var_b1_dn1 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn1)) + locals.var_b2_dn1) / (2.0 * assign2830_e2716))), (locals.var_b1_dn3 + ((((locals.var_b1_dn3 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn3)) + locals.var_b2_dn3) / (2.0 * assign2830_e2716))), (locals.var_b1_dn4 + ((((locals.var_b1_dn4 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn4)) + locals.var_b2_dn4) / (2.0 * assign2830_e2716))), (locals.var_b1_dn5 + ((((locals.var_b1_dn5 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn5)) + locals.var_b2_dn5) / (2.0 * assign2830_e2716))), (locals.var_b1_dn6 + ((((locals.var_b1_dn6 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn6)) + locals.var_b2_dn6) / (2.0 * assign2830_e2716))), (locals.var_b1_dn7 + ((((locals.var_b1_dn7 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn7)) + locals.var_b2_dn7) / (2.0 * assign2830_e2716))), (locals.var_b1_dn8 + ((((locals.var_b1_dn8 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn8)) + locals.var_b2_dn8) / (2.0 * assign2830_e2716))), (locals.var_b1_dn9 + ((((locals.var_b1_dn9 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn9)) + locals.var_b2_dn9) / (2.0 * assign2830_e2716))),)
    } else {
        (locals.var_vxi0, locals.var_vxi0_dn0, locals.var_vxi0_dn1, locals.var_vxi0_dn3, locals.var_vxi0_dn4, locals.var_vxi0_dn5, locals.var_vxi0_dn6, locals.var_vxi0_dn7, locals.var_vxi0_dn8, locals.var_vxi0_dn9,)
    }
};
        locals.var_vxi0 = assign2830_e2719;
        locals.var_vxi0_dn0 = assign2830_e2719_d_n0;
        locals.var_vxi0_dn1 = assign2830_e2719_d_n1;
        locals.var_vxi0_dn3 = assign2830_e2719_d_n3;
        locals.var_vxi0_dn4 = assign2830_e2719_d_n4;
        locals.var_vxi0_dn5 = assign2830_e2719_d_n5;
        locals.var_vxi0_dn6 = assign2830_e2719_d_n6;
        locals.var_vxi0_dn7 = assign2830_e2719_d_n7;
        locals.var_vxi0_dn8 = assign2830_e2719_d_n8;
        locals.var_vxi0_dn9 = assign2830_e2719_d_n9;

        let assign2840_e2722: f64 = if p.p72 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign2840_e2722;

        let (assign2850_e2730, assign2850_e2730_d_n0, assign2850_e2730_d_n1, assign2850_e2730_d_n3, assign2850_e2730_d_n4, assign2850_e2730_d_n5, assign2850_e2730_d_n6, assign2850_e2730_d_n7, assign2850_e2730_d_n8, assign2850_e2730_d_n9,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard44 != 0.0)) {
        let assign2850_e2728: f64 = (locals.var_vdc_ctc_t * 0.1);
        (assign2850_e2728, (locals.var_vdc_ctc_t_dn0 * 0.1), (locals.var_vdc_ctc_t_dn1 * 0.1), (locals.var_vdc_ctc_t_dn3 * 0.1), (locals.var_vdc_ctc_t_dn4 * 0.1), (locals.var_vdc_ctc_t_dn5 * 0.1), (locals.var_vdc_ctc_t_dn6 * 0.1), (locals.var_vdc_ctc_t_dn7 * 0.1), (locals.var_vdc_ctc_t_dn8 * 0.1), (locals.var_vdc_ctc_t_dn9 * 0.1),)
    } else {
        (locals.var_vch, locals.var_vch_dn0, locals.var_vch_dn1, locals.var_vch_dn3, locals.var_vch_dn4, locals.var_vch_dn5, locals.var_vch_dn6, locals.var_vch_dn7, locals.var_vch_dn8, locals.var_vch_dn9,)
    }
};
        locals.var_vch = assign2850_e2730;
        locals.var_vch_dn0 = assign2850_e2730_d_n0;
        locals.var_vch_dn1 = assign2850_e2730_d_n1;
        locals.var_vch_dn3 = assign2850_e2730_d_n3;
        locals.var_vch_dn4 = assign2850_e2730_d_n4;
        locals.var_vch_dn5 = assign2850_e2730_d_n5;
        locals.var_vch_dn6 = assign2850_e2730_d_n6;
        locals.var_vch_dn7 = assign2850_e2730_d_n7;
        locals.var_vch_dn8 = assign2850_e2730_d_n8;
        locals.var_vch_dn9 = assign2850_e2730_d_n9;

        let (assign2860_e2747, assign2860_e2747_d_n0, assign2860_e2747_d_n1, assign2860_e2747_d_n3, assign2860_e2747_d_n4, assign2860_e2747_d_n5, assign2860_e2747_d_n6, assign2860_e2747_d_n7, assign2860_e2747_d_n8, assign2860_e2747_d_n9,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard44 == 0.0)) {
        let assign2860_e2739: f64 = (2.0 * locals.var_ic1c2);
        let assign2860_e2742: f64 = (locals.var_ic1c2 + locals.var_iqs);
        let assign2860_e2743: f64 = (assign2860_e2739 / assign2860_e2742);
        let assign2860_e2744: f64 = (0.1 + assign2860_e2743);
        let assign2860_e2745: f64 = (locals.var_vdc_ctc_t * assign2860_e2744);
        (assign2860_e2745, ((locals.var_vdc_ctc_t_dn0 * assign2860_e2744) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn0) * assign2860_e2742) - (assign2860_e2739 * (locals.var_ic1c2_dn0 + locals.var_iqs_dn0))) / (assign2860_e2742 * assign2860_e2742)))), ((locals.var_vdc_ctc_t_dn1 * assign2860_e2744) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn1) * assign2860_e2742) - (assign2860_e2739 * (locals.var_ic1c2_dn1 + locals.var_iqs_dn1))) / (assign2860_e2742 * assign2860_e2742)))), ((locals.var_vdc_ctc_t_dn3 * assign2860_e2744) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn3) * assign2860_e2742) - (assign2860_e2739 * (locals.var_ic1c2_dn3 + locals.var_iqs_dn3))) / (assign2860_e2742 * assign2860_e2742)))), ((locals.var_vdc_ctc_t_dn4 * assign2860_e2744) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn4) * assign2860_e2742) - (assign2860_e2739 * (locals.var_ic1c2_dn4 + locals.var_iqs_dn4))) / (assign2860_e2742 * assign2860_e2742)))), ((locals.var_vdc_ctc_t_dn5 * assign2860_e2744) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn5) * assign2860_e2742) - (assign2860_e2739 * (locals.var_ic1c2_dn5 + locals.var_iqs_dn5))) / (assign2860_e2742 * assign2860_e2742)))), ((locals.var_vdc_ctc_t_dn6 * assign2860_e2744) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn6) * assign2860_e2742) - (assign2860_e2739 * (locals.var_ic1c2_dn6 + locals.var_iqs_dn6))) / (assign2860_e2742 * assign2860_e2742)))), ((locals.var_vdc_ctc_t_dn7 * assign2860_e2744) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn7) * assign2860_e2742) - (assign2860_e2739 * (locals.var_ic1c2_dn7 + locals.var_iqs_dn7))) / (assign2860_e2742 * assign2860_e2742)))), ((locals.var_vdc_ctc_t_dn8 * assign2860_e2744) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn8) * assign2860_e2742) - (assign2860_e2739 * (locals.var_ic1c2_dn8 + locals.var_iqs_dn8))) / (assign2860_e2742 * assign2860_e2742)))), ((locals.var_vdc_ctc_t_dn9 * assign2860_e2744) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn9) * assign2860_e2742) - (assign2860_e2739 * (locals.var_ic1c2_dn9 + locals.var_iqs_dn9))) / (assign2860_e2742 * assign2860_e2742)))),)
    } else {
        (locals.var_vch, locals.var_vch_dn0, locals.var_vch_dn1, locals.var_vch_dn3, locals.var_vch_dn4, locals.var_vch_dn5, locals.var_vch_dn6, locals.var_vch_dn7, locals.var_vch_dn8, locals.var_vch_dn9,)
    }
};
        locals.var_vch = assign2860_e2747;
        locals.var_vch_dn0 = assign2860_e2747_d_n0;
        locals.var_vch_dn1 = assign2860_e2747_d_n1;
        locals.var_vch_dn3 = assign2860_e2747_d_n3;
        locals.var_vch_dn4 = assign2860_e2747_d_n4;
        locals.var_vch_dn5 = assign2860_e2747_d_n5;
        locals.var_vch_dn6 = assign2860_e2747_d_n6;
        locals.var_vch_dn7 = assign2860_e2747_d_n7;
        locals.var_vch_dn8 = assign2860_e2747_d_n8;
        locals.var_vch_dn9 = assign2860_e2747_d_n9;

        let (assign2870_e2757, assign2870_e2757_d_n0, assign2870_e2757_d_n1, assign2870_e2757_d_n3, assign2870_e2757_d_n4, assign2870_e2757_d_n5, assign2870_e2757_d_n6, assign2870_e2757_d_n7, assign2870_e2757_d_n8, assign2870_e2757_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2870_e2751: f64 = (p.p61 * locals.var_ic1c2);
        let assign2870_e2754: f64 = (p.p61 + locals.var_ic1c2);
        let assign2870_e2755: f64 = (assign2870_e2751 / assign2870_e2754);
        (assign2870_e2755, ((((p.p61 * locals.var_ic1c2_dn0) * assign2870_e2754) - (assign2870_e2751 * locals.var_ic1c2_dn0)) / (assign2870_e2754 * assign2870_e2754)), ((((p.p61 * locals.var_ic1c2_dn1) * assign2870_e2754) - (assign2870_e2751 * locals.var_ic1c2_dn1)) / (assign2870_e2754 * assign2870_e2754)), ((((p.p61 * locals.var_ic1c2_dn3) * assign2870_e2754) - (assign2870_e2751 * locals.var_ic1c2_dn3)) / (assign2870_e2754 * assign2870_e2754)), ((((p.p61 * locals.var_ic1c2_dn4) * assign2870_e2754) - (assign2870_e2751 * locals.var_ic1c2_dn4)) / (assign2870_e2754 * assign2870_e2754)), ((((p.p61 * locals.var_ic1c2_dn5) * assign2870_e2754) - (assign2870_e2751 * locals.var_ic1c2_dn5)) / (assign2870_e2754 * assign2870_e2754)), ((((p.p61 * locals.var_ic1c2_dn6) * assign2870_e2754) - (assign2870_e2751 * locals.var_ic1c2_dn6)) / (assign2870_e2754 * assign2870_e2754)), ((((p.p61 * locals.var_ic1c2_dn7) * assign2870_e2754) - (assign2870_e2751 * locals.var_ic1c2_dn7)) / (assign2870_e2754 * assign2870_e2754)), ((((p.p61 * locals.var_ic1c2_dn8) * assign2870_e2754) - (assign2870_e2751 * locals.var_ic1c2_dn8)) / (assign2870_e2754 * assign2870_e2754)), ((((p.p61 * locals.var_ic1c2_dn9) * assign2870_e2754) - (assign2870_e2751 * locals.var_ic1c2_dn9)) / (assign2870_e2754 * assign2870_e2754)),)
    } else {
        (locals.var_icap, locals.var_icap_dn0, locals.var_icap_dn1, locals.var_icap_dn3, locals.var_icap_dn4, locals.var_icap_dn5, locals.var_icap_dn6, locals.var_icap_dn7, locals.var_icap_dn8, locals.var_icap_dn9,)
    }
};
        locals.var_icap = assign2870_e2757;
        locals.var_icap_dn0 = assign2870_e2757_d_n0;
        locals.var_icap_dn1 = assign2870_e2757_d_n1;
        locals.var_icap_dn3 = assign2870_e2757_d_n3;
        locals.var_icap_dn4 = assign2870_e2757_d_n4;
        locals.var_icap_dn5 = assign2870_e2757_d_n5;
        locals.var_icap_dn6 = assign2870_e2757_d_n6;
        locals.var_icap_dn7 = assign2870_e2757_d_n7;
        locals.var_icap_dn8 = assign2870_e2757_d_n8;
        locals.var_icap_dn9 = assign2870_e2757_d_n9;

        let (assign2880_e2765, assign2880_e2765_d_n0, assign2880_e2765_d_n1, assign2880_e2765_d_n3, assign2880_e2765_d_n4, assign2880_e2765_d_n5, assign2880_e2765_d_n6, assign2880_e2765_d_n7, assign2880_e2765_d_n8, assign2880_e2765_d_n9,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2880_e2762: f64 = (p.p61 + locals.var_ic1c2);
        let assign2880_e2763: f64 = (p.p61 / assign2880_e2762);
        (assign2880_e2763, (-((p.p61 * locals.var_ic1c2_dn0) / (assign2880_e2762 * assign2880_e2762))), (-((p.p61 * locals.var_ic1c2_dn1) / (assign2880_e2762 * assign2880_e2762))), (-((p.p61 * locals.var_ic1c2_dn3) / (assign2880_e2762 * assign2880_e2762))), (-((p.p61 * locals.var_ic1c2_dn4) / (assign2880_e2762 * assign2880_e2762))), (-((p.p61 * locals.var_ic1c2_dn5) / (assign2880_e2762 * assign2880_e2762))), (-((p.p61 * locals.var_ic1c2_dn6) / (assign2880_e2762 * assign2880_e2762))), (-((p.p61 * locals.var_ic1c2_dn7) / (assign2880_e2762 * assign2880_e2762))), (-((p.p61 * locals.var_ic1c2_dn8) / (assign2880_e2762 * assign2880_e2762))), (-((p.p61 * locals.var_ic1c2_dn9) / (assign2880_e2762 * assign2880_e2762))),)
    } else {
        (locals.var_icap_ihc, locals.var_icap_ihc_dn0, locals.var_icap_ihc_dn1, locals.var_icap_ihc_dn3, locals.var_icap_ihc_dn4, locals.var_icap_ihc_dn5, locals.var_icap_ihc_dn6, locals.var_icap_ihc_dn7, locals.var_icap_ihc_dn8, locals.var_icap_ihc_dn9,)
    }
};
        locals.var_icap_ihc = assign2880_e2765;
        locals.var_icap_ihc_dn0 = assign2880_e2765_d_n0;
        locals.var_icap_ihc_dn1 = assign2880_e2765_d_n1;
        locals.var_icap_ihc_dn3 = assign2880_e2765_d_n3;
        locals.var_icap_ihc_dn4 = assign2880_e2765_d_n4;
        locals.var_icap_ihc_dn5 = assign2880_e2765_d_n5;
        locals.var_icap_ihc_dn6 = assign2880_e2765_d_n6;
        locals.var_icap_ihc_dn7 = assign2880_e2765_d_n7;
        locals.var_icap_ihc_dn8 = assign2880_e2765_d_n8;
        locals.var_icap_ihc_dn9 = assign2880_e2765_d_n9;

        let (assign2890_e2770, assign2890_e2770_d_n0, assign2890_e2770_d_n1, assign2890_e2770_d_n3, assign2890_e2770_d_n4, assign2890_e2770_d_n5, assign2890_e2770_d_n6, assign2890_e2770_d_n7, assign2890_e2770_d_n8, assign2890_e2770_d_n9,) = {
    if (locals.var_guard38 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iqs, locals.var_iqs_dn0, locals.var_iqs_dn1, locals.var_iqs_dn3, locals.var_iqs_dn4, locals.var_iqs_dn5, locals.var_iqs_dn6, locals.var_iqs_dn7, locals.var_iqs_dn8, locals.var_iqs_dn9,)
    }
};
        locals.var_iqs = assign2890_e2770;
        locals.var_iqs_dn0 = assign2890_e2770_d_n0;
        locals.var_iqs_dn1 = assign2890_e2770_d_n1;
        locals.var_iqs_dn3 = assign2890_e2770_d_n3;
        locals.var_iqs_dn4 = assign2890_e2770_d_n4;
        locals.var_iqs_dn5 = assign2890_e2770_d_n5;
        locals.var_iqs_dn6 = assign2890_e2770_d_n6;
        locals.var_iqs_dn7 = assign2890_e2770_d_n7;
        locals.var_iqs_dn8 = assign2890_e2770_d_n8;
        locals.var_iqs_dn9 = assign2890_e2770_d_n9;

        let (assign2900_e2781, assign2900_e2781_d_n0, assign2900_e2781_d_n1, assign2900_e2781_d_n3, assign2900_e2781_d_n4, assign2900_e2781_d_n5, assign2900_e2781_d_n6, assign2900_e2781_d_n7, assign2900_e2781_d_n8, assign2900_e2781_d_n9,) = {
    if (locals.var_guard38 == 0.0) {
        let assign2900_e2775: f64 = (2.0 * locals.var_evb2c2vdc);
        let assign2900_e2778: f64 = (1.0 + locals.var_k0);
        let assign2900_e2779: f64 = (assign2900_e2775 / assign2900_e2778);
        (assign2900_e2779, ((((2.0 * locals.var_evb2c2vdc_dn0) * assign2900_e2778) - (assign2900_e2775 * locals.var_k0_dn0)) / (assign2900_e2778 * assign2900_e2778)), ((((2.0 * locals.var_evb2c2vdc_dn1) * assign2900_e2778) - (assign2900_e2775 * locals.var_k0_dn1)) / (assign2900_e2778 * assign2900_e2778)), ((((2.0 * locals.var_evb2c2vdc_dn3) * assign2900_e2778) - (assign2900_e2775 * locals.var_k0_dn3)) / (assign2900_e2778 * assign2900_e2778)), ((((2.0 * locals.var_evb2c2vdc_dn4) * assign2900_e2778) - (assign2900_e2775 * locals.var_k0_dn4)) / (assign2900_e2778 * assign2900_e2778)), ((((2.0 * locals.var_evb2c2vdc_dn5) * assign2900_e2778) - (assign2900_e2775 * locals.var_k0_dn5)) / (assign2900_e2778 * assign2900_e2778)), ((((2.0 * locals.var_evb2c2vdc_dn6) * assign2900_e2778) - (assign2900_e2775 * locals.var_k0_dn6)) / (assign2900_e2778 * assign2900_e2778)), ((((2.0 * locals.var_evb2c2vdc_dn7) * assign2900_e2778) - (assign2900_e2775 * locals.var_k0_dn7)) / (assign2900_e2778 * assign2900_e2778)), ((((2.0 * locals.var_evb2c2vdc_dn8) * assign2900_e2778) - (assign2900_e2775 * locals.var_k0_dn8)) / (assign2900_e2778 * assign2900_e2778)), ((((2.0 * locals.var_evb2c2vdc_dn9) * assign2900_e2778) - (assign2900_e2775 * locals.var_k0_dn9)) / (assign2900_e2778 * assign2900_e2778)),)
    } else {
        (locals.var_p0star, locals.var_p0star_dn0, locals.var_p0star_dn1, locals.var_p0star_dn3, locals.var_p0star_dn4, locals.var_p0star_dn5, locals.var_p0star_dn6, locals.var_p0star_dn7, locals.var_p0star_dn8, locals.var_p0star_dn9,)
    }
};
        locals.var_p0star = assign2900_e2781;
        locals.var_p0star_dn0 = assign2900_e2781_d_n0;
        locals.var_p0star_dn1 = assign2900_e2781_d_n1;
        locals.var_p0star_dn3 = assign2900_e2781_d_n3;
        locals.var_p0star_dn4 = assign2900_e2781_d_n4;
        locals.var_p0star_dn5 = assign2900_e2781_d_n5;
        locals.var_p0star_dn6 = assign2900_e2781_d_n6;
        locals.var_p0star_dn7 = assign2900_e2781_d_n7;
        locals.var_p0star_dn8 = assign2900_e2781_d_n8;
        locals.var_p0star_dn9 = assign2900_e2781_d_n9;

        let (assign2910_e2786, assign2910_e2786_d_n0, assign2910_e2786_d_n1, assign2910_e2786_d_n3, assign2910_e2786_d_n4, assign2910_e2786_d_n5, assign2910_e2786_d_n6, assign2910_e2786_d_n7, assign2910_e2786_d_n8, assign2910_e2786_d_n9,) = {
    if (locals.var_guard38 == 0.0) {
        (locals.var_evb2c2, 0.0, 0.0, 0.0, 0.0, locals.var_evb2c2_dn5, 0.0, locals.var_evb2c2_dn7, 0.0, 0.0,)
    } else {
        (locals.var_evb2c2star, locals.var_evb2c2star_dn0, locals.var_evb2c2star_dn1, locals.var_evb2c2star_dn3, locals.var_evb2c2star_dn4, locals.var_evb2c2star_dn5, locals.var_evb2c2star_dn6, locals.var_evb2c2star_dn7, locals.var_evb2c2star_dn8, locals.var_evb2c2star_dn9,)
    }
};
        locals.var_evb2c2star = assign2910_e2786;
        locals.var_evb2c2star_dn0 = assign2910_e2786_d_n0;
        locals.var_evb2c2star_dn1 = assign2910_e2786_d_n1;
        locals.var_evb2c2star_dn3 = assign2910_e2786_d_n3;
        locals.var_evb2c2star_dn4 = assign2910_e2786_d_n4;
        locals.var_evb2c2star_dn5 = assign2910_e2786_d_n5;
        locals.var_evb2c2star_dn6 = assign2910_e2786_d_n6;
        locals.var_evb2c2star_dn7 = assign2910_e2786_d_n7;
        locals.var_evb2c2star_dn8 = assign2910_e2786_d_n8;
        locals.var_evb2c2star_dn9 = assign2910_e2786_d_n9;

        let assign2920_e2788: f64 = (locals.var_vc1c2).abs();
        let assign2920_e2791: f64 = (1e-5 * locals.var_vt);
        let assign2920_e2794: f64 = (locals.var_ec).abs();
        let assign2920_e2797: f64 = (1e-40 * locals.var_vt);
        let assign2920_e2800: f64 = (locals.var_k0 + locals.var_kw);
        let assign2920_e2801: f64 = (assign2920_e2797 * assign2920_e2800);
        let assign2920_e2803: f64 = if ((assign2920_e2788 < assign2920_e2791) || (assign2920_e2794 < assign2920_e2801)) { 1.0 } else { 0.0 };
        locals.var_guard45 = assign2920_e2803;

        let (assign2930_e2814, assign2930_e2814_d_n0, assign2930_e2814_d_n1, assign2930_e2814_d_n3, assign2930_e2814_d_n4, assign2930_e2814_d_n5, assign2930_e2814_d_n6, assign2930_e2814_d_n7, assign2930_e2814_d_n8, assign2930_e2814_d_n9,) = {
    if ((locals.var_guard38 == 0.0) && (locals.var_guard45 != 0.0)) {
        let assign2930_e2811: f64 = (locals.var_p0star + locals.var_pw);
        let assign2930_e2812: f64 = (0.5 * assign2930_e2811);
        (assign2930_e2812, (0.5 * (locals.var_p0star_dn0 + locals.var_pw_dn0)), (0.5 * (locals.var_p0star_dn1 + locals.var_pw_dn1)), (0.5 * (locals.var_p0star_dn3 + locals.var_pw_dn3)), (0.5 * (locals.var_p0star_dn4 + locals.var_pw_dn4)), (0.5 * (locals.var_p0star_dn5 + locals.var_pw_dn5)), (0.5 * (locals.var_p0star_dn6 + locals.var_pw_dn6)), (0.5 * (locals.var_p0star_dn7 + locals.var_pw_dn7)), (0.5 * (locals.var_p0star_dn8 + locals.var_pw_dn8)), (0.5 * (locals.var_p0star_dn9 + locals.var_pw_dn9)),)
    } else {
        (locals.var_pav, locals.var_pav_dn0, locals.var_pav_dn1, locals.var_pav_dn3, locals.var_pav_dn4, locals.var_pav_dn5, locals.var_pav_dn6, locals.var_pav_dn7, locals.var_pav_dn8, locals.var_pav_dn9,)
    }
};
        locals.var_pav = assign2930_e2814;
        locals.var_pav_dn0 = assign2930_e2814_d_n0;
        locals.var_pav_dn1 = assign2930_e2814_d_n1;
        locals.var_pav_dn3 = assign2930_e2814_d_n3;
        locals.var_pav_dn4 = assign2930_e2814_d_n4;
        locals.var_pav_dn5 = assign2930_e2814_d_n5;
        locals.var_pav_dn6 = assign2930_e2814_d_n6;
        locals.var_pav_dn7 = assign2930_e2814_d_n7;
        locals.var_pav_dn8 = assign2930_e2814_d_n8;
        locals.var_pav_dn9 = assign2930_e2814_d_n9;

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2940_e2825, assign2940_e2825_d_n0, assign2940_e2825_d_n1, assign2940_e2825_d_n3, assign2940_e2825_d_n4, assign2940_e2825_d_n5, assign2940_e2825_d_n6, assign2940_e2825_d_n7, assign2940_e2825_d_n8, assign2940_e2825_d_n9,) = {
    if ((locals.var_guard38 == 0.0) && (locals.var_guard45 != 0.0)) {
        let assign2940_e2822: f64 = (locals.var_pav + 1.0);
        let assign2940_e2823: f64 = (locals.var_pav / assign2940_e2822);
        (assign2940_e2823, (((locals.var_pav_dn0 * assign2940_e2822) - (locals.var_pav * locals.var_pav_dn0)) / (assign2940_e2822 * assign2940_e2822)), (((locals.var_pav_dn1 * assign2940_e2822) - (locals.var_pav * locals.var_pav_dn1)) / (assign2940_e2822 * assign2940_e2822)), (((locals.var_pav_dn3 * assign2940_e2822) - (locals.var_pav * locals.var_pav_dn3)) / (assign2940_e2822 * assign2940_e2822)), (((locals.var_pav_dn4 * assign2940_e2822) - (locals.var_pav * locals.var_pav_dn4)) / (assign2940_e2822 * assign2940_e2822)), (((locals.var_pav_dn5 * assign2940_e2822) - (locals.var_pav * locals.var_pav_dn5)) / (assign2940_e2822 * assign2940_e2822)), (((locals.var_pav_dn6 * assign2940_e2822) - (locals.var_pav * locals.var_pav_dn6)) / (assign2940_e2822 * assign2940_e2822)), (((locals.var_pav_dn7 * assign2940_e2822) - (locals.var_pav * locals.var_pav_dn7)) / (assign2940_e2822 * assign2940_e2822)), (((locals.var_pav_dn8 * assign2940_e2822) - (locals.var_pav * locals.var_pav_dn8)) / (assign2940_e2822 * assign2940_e2822)), (((locals.var_pav_dn9 * assign2940_e2822) - (locals.var_pav * locals.var_pav_dn9)) / (assign2940_e2822 * assign2940_e2822)),)
    } else {
        (locals.var_xi_w, locals.var_xi_w_dn0, locals.var_xi_w_dn1, locals.var_xi_w_dn3, locals.var_xi_w_dn4, locals.var_xi_w_dn5, locals.var_xi_w_dn6, locals.var_xi_w_dn7, locals.var_xi_w_dn8, locals.var_xi_w_dn9,)
    }
};
        locals.var_xi_w = assign2940_e2825;
        locals.var_xi_w_dn0 = assign2940_e2825_d_n0;
        locals.var_xi_w_dn1 = assign2940_e2825_d_n1;
        locals.var_xi_w_dn3 = assign2940_e2825_d_n3;
        locals.var_xi_w_dn4 = assign2940_e2825_d_n4;
        locals.var_xi_w_dn5 = assign2940_e2825_d_n5;
        locals.var_xi_w_dn6 = assign2940_e2825_d_n6;
        locals.var_xi_w_dn7 = assign2940_e2825_d_n7;
        locals.var_xi_w_dn8 = assign2940_e2825_d_n8;
        locals.var_xi_w_dn9 = assign2940_e2825_d_n9;

        let (assign2950_e2839, assign2950_e2839_d_n0, assign2950_e2839_d_n1, assign2950_e2839_d_n3, assign2950_e2839_d_n4, assign2950_e2839_d_n5, assign2950_e2839_d_n6, assign2950_e2839_d_n7, assign2950_e2839_d_n8, assign2950_e2839_d_n9,) = {
    if ((locals.var_guard38 == 0.0) && (locals.var_guard45 == 0.0)) {
        let assign2950_e2834: f64 = (locals.var_ec + locals.var_vb2c2);
        let assign2950_e2836: f64 = (assign2950_e2834 - locals.var_vb2c1);
        let assign2950_e2837: f64 = (locals.var_ec / assign2950_e2836);
        (assign2950_e2837, (((locals.var_ec_dn0 * assign2950_e2836) - (locals.var_ec * locals.var_ec_dn0)) / (assign2950_e2836 * assign2950_e2836)), (((locals.var_ec_dn1 * assign2950_e2836) - (locals.var_ec * locals.var_ec_dn1)) / (assign2950_e2836 * assign2950_e2836)), (((locals.var_ec_dn3 * assign2950_e2836) - (locals.var_ec * locals.var_ec_dn3)) / (assign2950_e2836 * assign2950_e2836)), (((locals.var_ec_dn4 * assign2950_e2836) - (locals.var_ec * locals.var_ec_dn4)) / (assign2950_e2836 * assign2950_e2836)), (((locals.var_ec_dn5 * assign2950_e2836) - (locals.var_ec * ((locals.var_ec_dn5 + locals.var_vb2c2_dn5) - locals.var_vb2c1_dn5))) / (assign2950_e2836 * assign2950_e2836)), (((locals.var_ec_dn6 * assign2950_e2836) - (locals.var_ec * (locals.var_ec_dn6 - locals.var_vb2c1_dn6))) / (assign2950_e2836 * assign2950_e2836)), (((locals.var_ec_dn7 * assign2950_e2836) - (locals.var_ec * (locals.var_ec_dn7 + locals.var_vb2c2_dn7))) / (assign2950_e2836 * assign2950_e2836)), (((locals.var_ec_dn8 * assign2950_e2836) - (locals.var_ec * locals.var_ec_dn8)) / (assign2950_e2836 * assign2950_e2836)), (((locals.var_ec_dn9 * assign2950_e2836) - (locals.var_ec * locals.var_ec_dn9)) / (assign2950_e2836 * assign2950_e2836)),)
    } else {
        (locals.var_xi_w, locals.var_xi_w_dn0, locals.var_xi_w_dn1, locals.var_xi_w_dn3, locals.var_xi_w_dn4, locals.var_xi_w_dn5, locals.var_xi_w_dn6, locals.var_xi_w_dn7, locals.var_xi_w_dn8, locals.var_xi_w_dn9,)
    }
};
        locals.var_xi_w = assign2950_e2839;
        locals.var_xi_w_dn0 = assign2950_e2839_d_n0;
        locals.var_xi_w_dn1 = assign2950_e2839_d_n1;
        locals.var_xi_w_dn3 = assign2950_e2839_d_n3;
        locals.var_xi_w_dn4 = assign2950_e2839_d_n4;
        locals.var_xi_w_dn5 = assign2950_e2839_d_n5;
        locals.var_xi_w_dn6 = assign2950_e2839_d_n6;
        locals.var_xi_w_dn7 = assign2950_e2839_d_n7;
        locals.var_xi_w_dn8 = assign2950_e2839_d_n8;
        locals.var_xi_w_dn9 = assign2950_e2839_d_n9;

        let (assign2960_e2844, assign2960_e2844_d_n0, assign2960_e2844_d_n1, assign2960_e2844_d_n3, assign2960_e2844_d_n4, assign2960_e2844_d_n5, assign2960_e2844_d_n6, assign2960_e2844_d_n7, assign2960_e2844_d_n8, assign2960_e2844_d_n9,) = {
    if (locals.var_guard38 == 0.0) {
        (locals.var_vc1c2, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_vc1c2_dn6, locals.var_vc1c2_dn7, 0.0, 0.0,)
    } else {
        (locals.var_vxi0, locals.var_vxi0_dn0, locals.var_vxi0_dn1, locals.var_vxi0_dn3, locals.var_vxi0_dn4, locals.var_vxi0_dn5, locals.var_vxi0_dn6, locals.var_vxi0_dn7, locals.var_vxi0_dn8, locals.var_vxi0_dn9,)
    }
};
        locals.var_vxi0 = assign2960_e2844;
        locals.var_vxi0_dn0 = assign2960_e2844_d_n0;
        locals.var_vxi0_dn1 = assign2960_e2844_d_n1;
        locals.var_vxi0_dn3 = assign2960_e2844_d_n3;
        locals.var_vxi0_dn4 = assign2960_e2844_d_n4;
        locals.var_vxi0_dn5 = assign2960_e2844_d_n5;
        locals.var_vxi0_dn6 = assign2960_e2844_d_n6;
        locals.var_vxi0_dn7 = assign2960_e2844_d_n7;
        locals.var_vxi0_dn8 = assign2960_e2844_d_n8;
        locals.var_vxi0_dn9 = assign2960_e2844_d_n9;

        let (assign2970_e2851, assign2970_e2851_d_n0, assign2970_e2851_d_n1, assign2970_e2851_d_n3, assign2970_e2851_d_n4, assign2970_e2851_d_n5, assign2970_e2851_d_n6, assign2970_e2851_d_n7, assign2970_e2851_d_n8, assign2970_e2851_d_n9,) = {
    if (locals.var_guard38 == 0.0) {
        let assign2970_e2849: f64 = (0.1 * locals.var_vdc_ctc_t);
        (assign2970_e2849, (0.1 * locals.var_vdc_ctc_t_dn0), (0.1 * locals.var_vdc_ctc_t_dn1), (0.1 * locals.var_vdc_ctc_t_dn3), (0.1 * locals.var_vdc_ctc_t_dn4), (0.1 * locals.var_vdc_ctc_t_dn5), (0.1 * locals.var_vdc_ctc_t_dn6), (0.1 * locals.var_vdc_ctc_t_dn7), (0.1 * locals.var_vdc_ctc_t_dn8), (0.1 * locals.var_vdc_ctc_t_dn9),)
    } else {
        (locals.var_vch, locals.var_vch_dn0, locals.var_vch_dn1, locals.var_vch_dn3, locals.var_vch_dn4, locals.var_vch_dn5, locals.var_vch_dn6, locals.var_vch_dn7, locals.var_vch_dn8, locals.var_vch_dn9,)
    }
};
        locals.var_vch = assign2970_e2851;
        locals.var_vch_dn0 = assign2970_e2851_d_n0;
        locals.var_vch_dn1 = assign2970_e2851_d_n1;
        locals.var_vch_dn3 = assign2970_e2851_d_n3;
        locals.var_vch_dn4 = assign2970_e2851_d_n4;
        locals.var_vch_dn5 = assign2970_e2851_d_n5;
        locals.var_vch_dn6 = assign2970_e2851_d_n6;
        locals.var_vch_dn7 = assign2970_e2851_d_n7;
        locals.var_vch_dn8 = assign2970_e2851_d_n8;
        locals.var_vch_dn9 = assign2970_e2851_d_n9;

        let (assign2980_e2856, assign2980_e2856_d_n0, assign2980_e2856_d_n1, assign2980_e2856_d_n3, assign2980_e2856_d_n4, assign2980_e2856_d_n5, assign2980_e2856_d_n6, assign2980_e2856_d_n7, assign2980_e2856_d_n8, assign2980_e2856_d_n9,) = {
    if (locals.var_guard38 == 0.0) {
        (locals.var_ic1c2, locals.var_ic1c2_dn0, locals.var_ic1c2_dn1, locals.var_ic1c2_dn3, locals.var_ic1c2_dn4, locals.var_ic1c2_dn5, locals.var_ic1c2_dn6, locals.var_ic1c2_dn7, locals.var_ic1c2_dn8, locals.var_ic1c2_dn9,)
    } else {
        (locals.var_icap, locals.var_icap_dn0, locals.var_icap_dn1, locals.var_icap_dn3, locals.var_icap_dn4, locals.var_icap_dn5, locals.var_icap_dn6, locals.var_icap_dn7, locals.var_icap_dn8, locals.var_icap_dn9,)
    }
};
        locals.var_icap = assign2980_e2856;
        locals.var_icap_dn0 = assign2980_e2856_d_n0;
        locals.var_icap_dn1 = assign2980_e2856_d_n1;
        locals.var_icap_dn3 = assign2980_e2856_d_n3;
        locals.var_icap_dn4 = assign2980_e2856_d_n4;
        locals.var_icap_dn5 = assign2980_e2856_d_n5;
        locals.var_icap_dn6 = assign2980_e2856_d_n6;
        locals.var_icap_dn7 = assign2980_e2856_d_n7;
        locals.var_icap_dn8 = assign2980_e2856_d_n8;
        locals.var_icap_dn9 = assign2980_e2856_d_n9;

        let (assign2990_e2865, assign2990_e2865_d_n0, assign2990_e2865_d_n1, assign2990_e2865_d_n3, assign2990_e2865_d_n4, assign2990_e2865_d_n5, assign2990_e2865_d_n6, assign2990_e2865_d_n7, assign2990_e2865_d_n8, assign2990_e2865_d_n9,) = {
    if (locals.var_guard38 == 0.0) {
        let assign2990_e2862: f64 = (locals.var_icap / p.p61);
        let assign2990_e2863: f64 = (1.0 - assign2990_e2862);
        (assign2990_e2863, (-(locals.var_icap_dn0 / p.p61)), (-(locals.var_icap_dn1 / p.p61)), (-(locals.var_icap_dn3 / p.p61)), (-(locals.var_icap_dn4 / p.p61)), (-(locals.var_icap_dn5 / p.p61)), (-(locals.var_icap_dn6 / p.p61)), (-(locals.var_icap_dn7 / p.p61)), (-(locals.var_icap_dn8 / p.p61)), (-(locals.var_icap_dn9 / p.p61)),)
    } else {
        (locals.var_icap_ihc, locals.var_icap_ihc_dn0, locals.var_icap_ihc_dn1, locals.var_icap_ihc_dn3, locals.var_icap_ihc_dn4, locals.var_icap_ihc_dn5, locals.var_icap_ihc_dn6, locals.var_icap_ihc_dn7, locals.var_icap_ihc_dn8, locals.var_icap_ihc_dn9,)
    }
};
        locals.var_icap_ihc = assign2990_e2865;
        locals.var_icap_ihc_dn0 = assign2990_e2865_d_n0;
        locals.var_icap_ihc_dn1 = assign2990_e2865_d_n1;
        locals.var_icap_ihc_dn3 = assign2990_e2865_d_n3;
        locals.var_icap_ihc_dn4 = assign2990_e2865_d_n4;
        locals.var_icap_ihc_dn5 = assign2990_e2865_d_n5;
        locals.var_icap_ihc_dn6 = assign2990_e2865_d_n6;
        locals.var_icap_ihc_dn7 = assign2990_e2865_d_n7;
        locals.var_icap_ihc_dn8 = assign2990_e2865_d_n8;
        locals.var_icap_ihc_dn9 = assign2990_e2865_d_n9;

        let assign3000_e2870: f64 = (-1.0);
        let assign3000_e2872: f64 = (assign3000_e2870 / p.p66);
        let assign3000_e2873: f64 = (3.0_f64).powf(assign3000_e2872);
        let assign3000_e2874: f64 = (1.0 - assign3000_e2873);
        let assign3000_e2875: f64 = (locals.var_vde_t * assign3000_e2874);
        locals.var_vfe = assign3000_e2875;
        locals.var_vfe_dn0 = (locals.var_vde_t_dn0 * assign3000_e2874);
        locals.var_vfe_dn1 = (locals.var_vde_t_dn1 * assign3000_e2874);
        locals.var_vfe_dn3 = (locals.var_vde_t_dn3 * assign3000_e2874);
        locals.var_vfe_dn4 = (locals.var_vde_t_dn4 * assign3000_e2874);
        locals.var_vfe_dn5 = (locals.var_vde_t_dn5 * assign3000_e2874);
        locals.var_vfe_dn6 = (locals.var_vde_t_dn6 * assign3000_e2874);
        locals.var_vfe_dn7 = (locals.var_vde_t_dn7 * assign3000_e2874);
        locals.var_vfe_dn8 = (locals.var_vde_t_dn8 * assign3000_e2874);
        locals.var_vfe_dn9 = (locals.var_vde_t_dn9 * assign3000_e2874);

        let assign3010_e2878: f64 = (0.1 * locals.var_vde_t);
        locals.var_a_vde = assign3010_e2878;
        locals.var_a_vde_dn0 = (0.1 * locals.var_vde_t_dn0);
        locals.var_a_vde_dn1 = (0.1 * locals.var_vde_t_dn1);
        locals.var_a_vde_dn3 = (0.1 * locals.var_vde_t_dn3);
        locals.var_a_vde_dn4 = (0.1 * locals.var_vde_t_dn4);
        locals.var_a_vde_dn5 = (0.1 * locals.var_vde_t_dn5);
        locals.var_a_vde_dn6 = (0.1 * locals.var_vde_t_dn6);
        locals.var_a_vde_dn7 = (0.1 * locals.var_vde_t_dn7);
        locals.var_a_vde_dn8 = (0.1 * locals.var_vde_t_dn8);
        locals.var_a_vde_dn9 = (0.1 * locals.var_vde_t_dn9);

        let assign3020_e2881: f64 = (locals.var_vb2e1 - locals.var_vfe);
        let assign3020_e2883: f64 = (assign3020_e2881 / locals.var_a_vde);
        locals.var_dxa = assign3020_e2883;
        locals.var_dxa_dn0 = ((((-locals.var_vfe_dn0) * locals.var_a_vde) - (assign3020_e2881 * locals.var_a_vde_dn0)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn1 = ((((-locals.var_vfe_dn1) * locals.var_a_vde) - (assign3020_e2881 * locals.var_a_vde_dn1)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn3 = ((((locals.var_vb2e1_dn3 - locals.var_vfe_dn3) * locals.var_a_vde) - (assign3020_e2881 * locals.var_a_vde_dn3)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn4 = ((((-locals.var_vfe_dn4) * locals.var_a_vde) - (assign3020_e2881 * locals.var_a_vde_dn4)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn5 = ((((locals.var_vb2e1_dn5 - locals.var_vfe_dn5) * locals.var_a_vde) - (assign3020_e2881 * locals.var_a_vde_dn5)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn6 = ((((-locals.var_vfe_dn6) * locals.var_a_vde) - (assign3020_e2881 * locals.var_a_vde_dn6)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn7 = ((((-locals.var_vfe_dn7) * locals.var_a_vde) - (assign3020_e2881 * locals.var_a_vde_dn7)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn8 = ((((-locals.var_vfe_dn8) * locals.var_a_vde) - (assign3020_e2881 * locals.var_a_vde_dn8)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn9 = ((((-locals.var_vfe_dn9) * locals.var_a_vde) - (assign3020_e2881 * locals.var_a_vde_dn9)) / (locals.var_a_vde * locals.var_a_vde));

        let assign3030_e2886: f64 = if locals.var_vb2e1 < locals.var_vfe { 1.0 } else { 0.0 };
        locals.var_guard46 = assign3030_e2886;

        let (assign3040_e2898, assign3040_e2898_d_n0, assign3040_e2898_d_n1, assign3040_e2898_d_n3, assign3040_e2898_d_n4, assign3040_e2898_d_n5, assign3040_e2898_d_n6, assign3040_e2898_d_n7, assign3040_e2898_d_n8, assign3040_e2898_d_n9,) = {
    if (locals.var_guard46 != 0.0) {
        let assign3040_e2892: f64 = (locals.var_dxa).exp();
        let assign3040_e2893: f64 = (1.0 + assign3040_e2892);
        let assign3040_e2894: f64 = (assign3040_e2893).ln();
        let assign3040_e2895: f64 = (locals.var_a_vde * assign3040_e2894);
        let assign3040_e2896: f64 = (locals.var_vb2e1 - assign3040_e2895);
        (assign3040_e2896, (-((locals.var_a_vde_dn0 * assign3040_e2894) + (locals.var_a_vde * ((assign3040_e2892 * locals.var_dxa_dn0) / assign3040_e2893)))), (-((locals.var_a_vde_dn1 * assign3040_e2894) + (locals.var_a_vde * ((assign3040_e2892 * locals.var_dxa_dn1) / assign3040_e2893)))), (locals.var_vb2e1_dn3 - ((locals.var_a_vde_dn3 * assign3040_e2894) + (locals.var_a_vde * ((assign3040_e2892 * locals.var_dxa_dn3) / assign3040_e2893)))), (-((locals.var_a_vde_dn4 * assign3040_e2894) + (locals.var_a_vde * ((assign3040_e2892 * locals.var_dxa_dn4) / assign3040_e2893)))), (locals.var_vb2e1_dn5 - ((locals.var_a_vde_dn5 * assign3040_e2894) + (locals.var_a_vde * ((assign3040_e2892 * locals.var_dxa_dn5) / assign3040_e2893)))), (-((locals.var_a_vde_dn6 * assign3040_e2894) + (locals.var_a_vde * ((assign3040_e2892 * locals.var_dxa_dn6) / assign3040_e2893)))), (-((locals.var_a_vde_dn7 * assign3040_e2894) + (locals.var_a_vde * ((assign3040_e2892 * locals.var_dxa_dn7) / assign3040_e2893)))), (-((locals.var_a_vde_dn8 * assign3040_e2894) + (locals.var_a_vde * ((assign3040_e2892 * locals.var_dxa_dn8) / assign3040_e2893)))), (-((locals.var_a_vde_dn9 * assign3040_e2894) + (locals.var_a_vde * ((assign3040_e2892 * locals.var_dxa_dn9) / assign3040_e2893)))),)
    } else {
        (locals.var_vje, locals.var_vje_dn0, locals.var_vje_dn1, locals.var_vje_dn3, locals.var_vje_dn4, locals.var_vje_dn5, locals.var_vje_dn6, locals.var_vje_dn7, locals.var_vje_dn8, locals.var_vje_dn9,)
    }
};
        locals.var_vje = assign3040_e2898;
        locals.var_vje_dn0 = assign3040_e2898_d_n0;
        locals.var_vje_dn1 = assign3040_e2898_d_n1;
        locals.var_vje_dn3 = assign3040_e2898_d_n3;
        locals.var_vje_dn4 = assign3040_e2898_d_n4;
        locals.var_vje_dn5 = assign3040_e2898_d_n5;
        locals.var_vje_dn6 = assign3040_e2898_d_n6;
        locals.var_vje_dn7 = assign3040_e2898_d_n7;
        locals.var_vje_dn8 = assign3040_e2898_d_n8;
        locals.var_vje_dn9 = assign3040_e2898_d_n9;

        let (assign3050_e2912, assign3050_e2912_d_n0, assign3050_e2912_d_n1, assign3050_e2912_d_n3, assign3050_e2912_d_n4, assign3050_e2912_d_n5, assign3050_e2912_d_n6, assign3050_e2912_d_n7, assign3050_e2912_d_n8, assign3050_e2912_d_n9,) = {
    if (locals.var_guard46 == 0.0) {
        let assign3050_e2905: f64 = (-locals.var_dxa);
        let assign3050_e2906: f64 = (assign3050_e2905).exp();
        let assign3050_e2907: f64 = (1.0 + assign3050_e2906);
        let assign3050_e2908: f64 = (assign3050_e2907).ln();
        let assign3050_e2909: f64 = (locals.var_a_vde * assign3050_e2908);
        let assign3050_e2910: f64 = (locals.var_vfe - assign3050_e2909);
        (assign3050_e2910, (locals.var_vfe_dn0 - ((locals.var_a_vde_dn0 * assign3050_e2908) + (locals.var_a_vde * ((assign3050_e2906 * (-locals.var_dxa_dn0)) / assign3050_e2907)))), (locals.var_vfe_dn1 - ((locals.var_a_vde_dn1 * assign3050_e2908) + (locals.var_a_vde * ((assign3050_e2906 * (-locals.var_dxa_dn1)) / assign3050_e2907)))), (locals.var_vfe_dn3 - ((locals.var_a_vde_dn3 * assign3050_e2908) + (locals.var_a_vde * ((assign3050_e2906 * (-locals.var_dxa_dn3)) / assign3050_e2907)))), (locals.var_vfe_dn4 - ((locals.var_a_vde_dn4 * assign3050_e2908) + (locals.var_a_vde * ((assign3050_e2906 * (-locals.var_dxa_dn4)) / assign3050_e2907)))), (locals.var_vfe_dn5 - ((locals.var_a_vde_dn5 * assign3050_e2908) + (locals.var_a_vde * ((assign3050_e2906 * (-locals.var_dxa_dn5)) / assign3050_e2907)))), (locals.var_vfe_dn6 - ((locals.var_a_vde_dn6 * assign3050_e2908) + (locals.var_a_vde * ((assign3050_e2906 * (-locals.var_dxa_dn6)) / assign3050_e2907)))), (locals.var_vfe_dn7 - ((locals.var_a_vde_dn7 * assign3050_e2908) + (locals.var_a_vde * ((assign3050_e2906 * (-locals.var_dxa_dn7)) / assign3050_e2907)))), (locals.var_vfe_dn8 - ((locals.var_a_vde_dn8 * assign3050_e2908) + (locals.var_a_vde * ((assign3050_e2906 * (-locals.var_dxa_dn8)) / assign3050_e2907)))), (locals.var_vfe_dn9 - ((locals.var_a_vde_dn9 * assign3050_e2908) + (locals.var_a_vde * ((assign3050_e2906 * (-locals.var_dxa_dn9)) / assign3050_e2907)))),)
    } else {
        (locals.var_vje, locals.var_vje_dn0, locals.var_vje_dn1, locals.var_vje_dn3, locals.var_vje_dn4, locals.var_vje_dn5, locals.var_vje_dn6, locals.var_vje_dn7, locals.var_vje_dn8, locals.var_vje_dn9,)
    }
};
        locals.var_vje = assign3050_e2912;
        locals.var_vje_dn0 = assign3050_e2912_d_n0;
        locals.var_vje_dn1 = assign3050_e2912_d_n1;
        locals.var_vje_dn3 = assign3050_e2912_d_n3;
        locals.var_vje_dn4 = assign3050_e2912_d_n4;
        locals.var_vje_dn5 = assign3050_e2912_d_n5;
        locals.var_vje_dn6 = assign3050_e2912_d_n6;
        locals.var_vje_dn7 = assign3050_e2912_d_n7;
        locals.var_vje_dn8 = assign3050_e2912_d_n8;
        locals.var_vje_dn9 = assign3050_e2912_d_n9;

        let assign3060_e2916: f64 = (locals.var_vje * locals.var_inv_vde_t);
        let assign3060_e2917: f64 = (1.0 - assign3060_e2916);
        let assign3060_e2920: f64 = (1.0 - p.p66);
        let assign3060_e2921: f64 = (assign3060_e2917).powf(assign3060_e2920);
        locals.var_e0eb = assign3060_e2921;
        locals.var_e0eb_dn0 = if 0.0 == 0.0 && ((assign3060_e2920) as f64).is_finite() && ((assign3060_e2920) as f64).fract() == 0.0 { if assign3060_e2920 == 0.0 { 0.0 } else { (assign3060_e2920 * ((assign3060_e2917).powf(assign3060_e2920 - 1.0) * (-((locals.var_vje_dn0 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn0))))) } } else { (assign3060_e2921 * (assign3060_e2920 * ((-((locals.var_vje_dn0 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn0))) / assign3060_e2917))) };
        locals.var_e0eb_dn1 = if 0.0 == 0.0 && ((assign3060_e2920) as f64).is_finite() && ((assign3060_e2920) as f64).fract() == 0.0 { if assign3060_e2920 == 0.0 { 0.0 } else { (assign3060_e2920 * ((assign3060_e2917).powf(assign3060_e2920 - 1.0) * (-((locals.var_vje_dn1 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn1))))) } } else { (assign3060_e2921 * (assign3060_e2920 * ((-((locals.var_vje_dn1 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn1))) / assign3060_e2917))) };
        locals.var_e0eb_dn3 = if 0.0 == 0.0 && ((assign3060_e2920) as f64).is_finite() && ((assign3060_e2920) as f64).fract() == 0.0 { if assign3060_e2920 == 0.0 { 0.0 } else { (assign3060_e2920 * ((assign3060_e2917).powf(assign3060_e2920 - 1.0) * (-((locals.var_vje_dn3 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn3))))) } } else { (assign3060_e2921 * (assign3060_e2920 * ((-((locals.var_vje_dn3 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn3))) / assign3060_e2917))) };
        locals.var_e0eb_dn4 = if 0.0 == 0.0 && ((assign3060_e2920) as f64).is_finite() && ((assign3060_e2920) as f64).fract() == 0.0 { if assign3060_e2920 == 0.0 { 0.0 } else { (assign3060_e2920 * ((assign3060_e2917).powf(assign3060_e2920 - 1.0) * (-((locals.var_vje_dn4 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn4))))) } } else { (assign3060_e2921 * (assign3060_e2920 * ((-((locals.var_vje_dn4 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn4))) / assign3060_e2917))) };
        locals.var_e0eb_dn5 = if 0.0 == 0.0 && ((assign3060_e2920) as f64).is_finite() && ((assign3060_e2920) as f64).fract() == 0.0 { if assign3060_e2920 == 0.0 { 0.0 } else { (assign3060_e2920 * ((assign3060_e2917).powf(assign3060_e2920 - 1.0) * (-((locals.var_vje_dn5 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn5))))) } } else { (assign3060_e2921 * (assign3060_e2920 * ((-((locals.var_vje_dn5 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn5))) / assign3060_e2917))) };
        locals.var_e0eb_dn6 = if 0.0 == 0.0 && ((assign3060_e2920) as f64).is_finite() && ((assign3060_e2920) as f64).fract() == 0.0 { if assign3060_e2920 == 0.0 { 0.0 } else { (assign3060_e2920 * ((assign3060_e2917).powf(assign3060_e2920 - 1.0) * (-((locals.var_vje_dn6 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn6))))) } } else { (assign3060_e2921 * (assign3060_e2920 * ((-((locals.var_vje_dn6 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn6))) / assign3060_e2917))) };
        locals.var_e0eb_dn7 = if 0.0 == 0.0 && ((assign3060_e2920) as f64).is_finite() && ((assign3060_e2920) as f64).fract() == 0.0 { if assign3060_e2920 == 0.0 { 0.0 } else { (assign3060_e2920 * ((assign3060_e2917).powf(assign3060_e2920 - 1.0) * (-((locals.var_vje_dn7 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn7))))) } } else { (assign3060_e2921 * (assign3060_e2920 * ((-((locals.var_vje_dn7 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn7))) / assign3060_e2917))) };
        locals.var_e0eb_dn8 = if 0.0 == 0.0 && ((assign3060_e2920) as f64).is_finite() && ((assign3060_e2920) as f64).fract() == 0.0 { if assign3060_e2920 == 0.0 { 0.0 } else { (assign3060_e2920 * ((assign3060_e2917).powf(assign3060_e2920 - 1.0) * (-((locals.var_vje_dn8 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn8))))) } } else { (assign3060_e2921 * (assign3060_e2920 * ((-((locals.var_vje_dn8 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn8))) / assign3060_e2917))) };
        locals.var_e0eb_dn9 = if 0.0 == 0.0 && ((assign3060_e2920) as f64).is_finite() && ((assign3060_e2920) as f64).fract() == 0.0 { if assign3060_e2920 == 0.0 { 0.0 } else { (assign3060_e2920 * ((assign3060_e2917).powf(assign3060_e2920 - 1.0) * (-((locals.var_vje_dn9 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn9))))) } } else { (assign3060_e2921 * (assign3060_e2920 * ((-((locals.var_vje_dn9 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn9))) / assign3060_e2917))) };

        let assign3070_e2925: f64 = (1.0 - p.p66);
        let assign3070_e2926: f64 = (locals.var_vde_t / assign3070_e2925);
        let assign3070_e2929: f64 = (1.0 - locals.var_e0eb);
        let assign3070_e2930: f64 = (assign3070_e2926 * assign3070_e2929);
        let assign3070_e2934: f64 = (locals.var_vb2e1 - locals.var_vje);
        let assign3070_e2935: f64 = (3.0 * assign3070_e2934);
        let assign3070_e2936: f64 = (assign3070_e2930 + assign3070_e2935);
        locals.var_vte = assign3070_e2936;
        locals.var_vte_dn0 = ((((locals.var_vde_t_dn0 / assign3070_e2925) * assign3070_e2929) + (assign3070_e2926 * (-locals.var_e0eb_dn0))) + (3.0 * (-locals.var_vje_dn0)));
        locals.var_vte_dn1 = ((((locals.var_vde_t_dn1 / assign3070_e2925) * assign3070_e2929) + (assign3070_e2926 * (-locals.var_e0eb_dn1))) + (3.0 * (-locals.var_vje_dn1)));
        locals.var_vte_dn3 = ((((locals.var_vde_t_dn3 / assign3070_e2925) * assign3070_e2929) + (assign3070_e2926 * (-locals.var_e0eb_dn3))) + (3.0 * (locals.var_vb2e1_dn3 - locals.var_vje_dn3)));
        locals.var_vte_dn4 = ((((locals.var_vde_t_dn4 / assign3070_e2925) * assign3070_e2929) + (assign3070_e2926 * (-locals.var_e0eb_dn4))) + (3.0 * (-locals.var_vje_dn4)));
        locals.var_vte_dn5 = ((((locals.var_vde_t_dn5 / assign3070_e2925) * assign3070_e2929) + (assign3070_e2926 * (-locals.var_e0eb_dn5))) + (3.0 * (locals.var_vb2e1_dn5 - locals.var_vje_dn5)));
        locals.var_vte_dn6 = ((((locals.var_vde_t_dn6 / assign3070_e2925) * assign3070_e2929) + (assign3070_e2926 * (-locals.var_e0eb_dn6))) + (3.0 * (-locals.var_vje_dn6)));
        locals.var_vte_dn7 = ((((locals.var_vde_t_dn7 / assign3070_e2925) * assign3070_e2929) + (assign3070_e2926 * (-locals.var_e0eb_dn7))) + (3.0 * (-locals.var_vje_dn7)));
        locals.var_vte_dn8 = ((((locals.var_vde_t_dn8 / assign3070_e2925) * assign3070_e2929) + (assign3070_e2926 * (-locals.var_e0eb_dn8))) + (3.0 * (-locals.var_vje_dn8)));
        locals.var_vte_dn9 = ((((locals.var_vde_t_dn9 / assign3070_e2925) * assign3070_e2929) + (assign3070_e2926 * (-locals.var_e0eb_dn9))) + (3.0 * (-locals.var_vje_dn9)));

        let assign3080_e2939: f64 = if p.p73 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard47 = assign3080_e2939;

        let (assign3090_e2943, assign3090_e2943_d_n0, assign3090_e2943_d_n1, assign3090_e2943_d_n3, assign3090_e2943_d_n4, assign3090_e2943_d_n5, assign3090_e2943_d_n6, assign3090_e2943_d_n7, assign3090_e2943_d_n8, assign3090_e2943_d_n9,) = {
    if (locals.var_guard47 != 0.0) {
        (locals.var_vb2c1, 0.0, 0.0, 0.0, 0.0, locals.var_vb2c1_dn5, locals.var_vb2c1_dn6, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjunc, locals.var_vjunc_dn0, locals.var_vjunc_dn1, locals.var_vjunc_dn3, locals.var_vjunc_dn4, locals.var_vjunc_dn5, locals.var_vjunc_dn6, locals.var_vjunc_dn7, locals.var_vjunc_dn8, locals.var_vjunc_dn9,)
    }
};
        locals.var_vjunc = assign3090_e2943;
        locals.var_vjunc_dn0 = assign3090_e2943_d_n0;
        locals.var_vjunc_dn1 = assign3090_e2943_d_n1;
        locals.var_vjunc_dn3 = assign3090_e2943_d_n3;
        locals.var_vjunc_dn4 = assign3090_e2943_d_n4;
        locals.var_vjunc_dn5 = assign3090_e2943_d_n5;
        locals.var_vjunc_dn6 = assign3090_e2943_d_n6;
        locals.var_vjunc_dn7 = assign3090_e2943_d_n7;
        locals.var_vjunc_dn8 = assign3090_e2943_d_n8;
        locals.var_vjunc_dn9 = assign3090_e2943_d_n9;

        let assign3100_e2946: f64 = if p.p73 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard48 = assign3100_e2946;

        let (assign3110_e2955, assign3110_e2955_d_n0, assign3110_e2955_d_n1, assign3110_e2955_d_n3, assign3110_e2955_d_n4, assign3110_e2955_d_n5, assign3110_e2955_d_n6, assign3110_e2955_d_n7, assign3110_e2955_d_n8, assign3110_e2955_d_n9,) = {
    if ((locals.var_guard47 == 0.0) && (locals.var_guard48 != 0.0)) {
        let assign3110_e2953: f64 = (locals.var_vb2c1 + locals.var_vxi0);
        (assign3110_e2953, locals.var_vxi0_dn0, locals.var_vxi0_dn1, locals.var_vxi0_dn3, locals.var_vxi0_dn4, (locals.var_vb2c1_dn5 + locals.var_vxi0_dn5), (locals.var_vb2c1_dn6 + locals.var_vxi0_dn6), locals.var_vxi0_dn7, locals.var_vxi0_dn8, locals.var_vxi0_dn9,)
    } else {
        (locals.var_vjunc, locals.var_vjunc_dn0, locals.var_vjunc_dn1, locals.var_vjunc_dn3, locals.var_vjunc_dn4, locals.var_vjunc_dn5, locals.var_vjunc_dn6, locals.var_vjunc_dn7, locals.var_vjunc_dn8, locals.var_vjunc_dn9,)
    }
};
        locals.var_vjunc = assign3110_e2955;
        locals.var_vjunc_dn0 = assign3110_e2955_d_n0;
        locals.var_vjunc_dn1 = assign3110_e2955_d_n1;
        locals.var_vjunc_dn3 = assign3110_e2955_d_n3;
        locals.var_vjunc_dn4 = assign3110_e2955_d_n4;
        locals.var_vjunc_dn5 = assign3110_e2955_d_n5;
        locals.var_vjunc_dn6 = assign3110_e2955_d_n6;
        locals.var_vjunc_dn7 = assign3110_e2955_d_n7;
        locals.var_vjunc_dn8 = assign3110_e2955_d_n8;
        locals.var_vjunc_dn9 = assign3110_e2955_d_n9;

        let (assign3120_e2963, assign3120_e2963_d_n0, assign3120_e2963_d_n1, assign3120_e2963_d_n3, assign3120_e2963_d_n4, assign3120_e2963_d_n5, assign3120_e2963_d_n6, assign3120_e2963_d_n7, assign3120_e2963_d_n8, assign3120_e2963_d_n9,) = {
    if ((locals.var_guard47 == 0.0) && (locals.var_guard48 == 0.0)) {
        (locals.var_vb2c2, 0.0, 0.0, 0.0, 0.0, locals.var_vb2c2_dn5, 0.0, locals.var_vb2c2_dn7, 0.0, 0.0,)
    } else {
        (locals.var_vjunc, locals.var_vjunc_dn0, locals.var_vjunc_dn1, locals.var_vjunc_dn3, locals.var_vjunc_dn4, locals.var_vjunc_dn5, locals.var_vjunc_dn6, locals.var_vjunc_dn7, locals.var_vjunc_dn8, locals.var_vjunc_dn9,)
    }
};
        locals.var_vjunc = assign3120_e2963;
        locals.var_vjunc_dn0 = assign3120_e2963_d_n0;
        locals.var_vjunc_dn1 = assign3120_e2963_d_n1;
        locals.var_vjunc_dn3 = assign3120_e2963_d_n3;
        locals.var_vjunc_dn4 = assign3120_e2963_d_n4;
        locals.var_vjunc_dn5 = assign3120_e2963_d_n5;
        locals.var_vjunc_dn6 = assign3120_e2963_d_n6;
        locals.var_vjunc_dn7 = assign3120_e2963_d_n7;
        locals.var_vjunc_dn8 = assign3120_e2963_d_n8;
        locals.var_vjunc_dn9 = assign3120_e2963_d_n9;

        let assign3130_e2966: f64 = (2.0 - locals.var_xp_t);
        let assign3130_e2969: f64 = (1.0 - locals.var_xp_t);
        let assign3130_e2970: f64 = (assign3130_e2966 / assign3130_e2969);
        locals.var_bjc = assign3130_e2970;
        locals.var_bjc_dn0 = ((((-locals.var_xp_t_dn0) * assign3130_e2969) - (assign3130_e2966 * (-locals.var_xp_t_dn0))) / (assign3130_e2969 * assign3130_e2969));
        locals.var_bjc_dn1 = ((((-locals.var_xp_t_dn1) * assign3130_e2969) - (assign3130_e2966 * (-locals.var_xp_t_dn1))) / (assign3130_e2969 * assign3130_e2969));
        locals.var_bjc_dn3 = ((((-locals.var_xp_t_dn3) * assign3130_e2969) - (assign3130_e2966 * (-locals.var_xp_t_dn3))) / (assign3130_e2969 * assign3130_e2969));
        locals.var_bjc_dn4 = ((((-locals.var_xp_t_dn4) * assign3130_e2969) - (assign3130_e2966 * (-locals.var_xp_t_dn4))) / (assign3130_e2969 * assign3130_e2969));
        locals.var_bjc_dn5 = ((((-locals.var_xp_t_dn5) * assign3130_e2969) - (assign3130_e2966 * (-locals.var_xp_t_dn5))) / (assign3130_e2969 * assign3130_e2969));
        locals.var_bjc_dn6 = ((((-locals.var_xp_t_dn6) * assign3130_e2969) - (assign3130_e2966 * (-locals.var_xp_t_dn6))) / (assign3130_e2969 * assign3130_e2969));
        locals.var_bjc_dn7 = ((((-locals.var_xp_t_dn7) * assign3130_e2969) - (assign3130_e2966 * (-locals.var_xp_t_dn7))) / (assign3130_e2969 * assign3130_e2969));
        locals.var_bjc_dn8 = ((((-locals.var_xp_t_dn8) * assign3130_e2969) - (assign3130_e2966 * (-locals.var_xp_t_dn8))) / (assign3130_e2969 * assign3130_e2969));
        locals.var_bjc_dn9 = ((((-locals.var_xp_t_dn9) * assign3130_e2969) - (assign3130_e2966 * (-locals.var_xp_t_dn9))) / (assign3130_e2969 * assign3130_e2969));

        let assign3140_e2975: f64 = (-1.0);
        let assign3140_e2977: f64 = (assign3140_e2975 / p.p71);
        let assign3140_e2978: f64 = (locals.var_bjc).powf(assign3140_e2977);
        let assign3140_e2979: f64 = (1.0 - assign3140_e2978);
        let assign3140_e2980: f64 = (locals.var_vdc_ctc_t * assign3140_e2979);
        locals.var_vfc = assign3140_e2980;
        locals.var_vfc_dn0 = ((locals.var_vdc_ctc_t_dn0 * assign3140_e2979) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3140_e2977) as f64).is_finite() && ((assign3140_e2977) as f64).fract() == 0.0 { if assign3140_e2977 == 0.0 { 0.0 } else { (assign3140_e2977 * ((locals.var_bjc).powf(assign3140_e2977 - 1.0) * locals.var_bjc_dn0)) } } else { (assign3140_e2978 * (assign3140_e2977 * (locals.var_bjc_dn0 / locals.var_bjc))) })));
        locals.var_vfc_dn1 = ((locals.var_vdc_ctc_t_dn1 * assign3140_e2979) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3140_e2977) as f64).is_finite() && ((assign3140_e2977) as f64).fract() == 0.0 { if assign3140_e2977 == 0.0 { 0.0 } else { (assign3140_e2977 * ((locals.var_bjc).powf(assign3140_e2977 - 1.0) * locals.var_bjc_dn1)) } } else { (assign3140_e2978 * (assign3140_e2977 * (locals.var_bjc_dn1 / locals.var_bjc))) })));
        locals.var_vfc_dn3 = ((locals.var_vdc_ctc_t_dn3 * assign3140_e2979) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3140_e2977) as f64).is_finite() && ((assign3140_e2977) as f64).fract() == 0.0 { if assign3140_e2977 == 0.0 { 0.0 } else { (assign3140_e2977 * ((locals.var_bjc).powf(assign3140_e2977 - 1.0) * locals.var_bjc_dn3)) } } else { (assign3140_e2978 * (assign3140_e2977 * (locals.var_bjc_dn3 / locals.var_bjc))) })));
        locals.var_vfc_dn4 = ((locals.var_vdc_ctc_t_dn4 * assign3140_e2979) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3140_e2977) as f64).is_finite() && ((assign3140_e2977) as f64).fract() == 0.0 { if assign3140_e2977 == 0.0 { 0.0 } else { (assign3140_e2977 * ((locals.var_bjc).powf(assign3140_e2977 - 1.0) * locals.var_bjc_dn4)) } } else { (assign3140_e2978 * (assign3140_e2977 * (locals.var_bjc_dn4 / locals.var_bjc))) })));
        locals.var_vfc_dn5 = ((locals.var_vdc_ctc_t_dn5 * assign3140_e2979) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3140_e2977) as f64).is_finite() && ((assign3140_e2977) as f64).fract() == 0.0 { if assign3140_e2977 == 0.0 { 0.0 } else { (assign3140_e2977 * ((locals.var_bjc).powf(assign3140_e2977 - 1.0) * locals.var_bjc_dn5)) } } else { (assign3140_e2978 * (assign3140_e2977 * (locals.var_bjc_dn5 / locals.var_bjc))) })));
        locals.var_vfc_dn6 = ((locals.var_vdc_ctc_t_dn6 * assign3140_e2979) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3140_e2977) as f64).is_finite() && ((assign3140_e2977) as f64).fract() == 0.0 { if assign3140_e2977 == 0.0 { 0.0 } else { (assign3140_e2977 * ((locals.var_bjc).powf(assign3140_e2977 - 1.0) * locals.var_bjc_dn6)) } } else { (assign3140_e2978 * (assign3140_e2977 * (locals.var_bjc_dn6 / locals.var_bjc))) })));
        locals.var_vfc_dn7 = ((locals.var_vdc_ctc_t_dn7 * assign3140_e2979) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3140_e2977) as f64).is_finite() && ((assign3140_e2977) as f64).fract() == 0.0 { if assign3140_e2977 == 0.0 { 0.0 } else { (assign3140_e2977 * ((locals.var_bjc).powf(assign3140_e2977 - 1.0) * locals.var_bjc_dn7)) } } else { (assign3140_e2978 * (assign3140_e2977 * (locals.var_bjc_dn7 / locals.var_bjc))) })));
        locals.var_vfc_dn8 = ((locals.var_vdc_ctc_t_dn8 * assign3140_e2979) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3140_e2977) as f64).is_finite() && ((assign3140_e2977) as f64).fract() == 0.0 { if assign3140_e2977 == 0.0 { 0.0 } else { (assign3140_e2977 * ((locals.var_bjc).powf(assign3140_e2977 - 1.0) * locals.var_bjc_dn8)) } } else { (assign3140_e2978 * (assign3140_e2977 * (locals.var_bjc_dn8 / locals.var_bjc))) })));
        locals.var_vfc_dn9 = ((locals.var_vdc_ctc_t_dn9 * assign3140_e2979) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3140_e2977) as f64).is_finite() && ((assign3140_e2977) as f64).fract() == 0.0 { if assign3140_e2977 == 0.0 { 0.0 } else { (assign3140_e2977 * ((locals.var_bjc).powf(assign3140_e2977 - 1.0) * locals.var_bjc_dn9)) } } else { (assign3140_e2978 * (assign3140_e2977 * (locals.var_bjc_dn9 / locals.var_bjc))) })));

        let assign3150_e2983: f64 = (locals.var_vjunc - locals.var_vfc);
        let assign3150_e2985: f64 = (assign3150_e2983 / locals.var_vch);
        locals.var_dxa = assign3150_e2985;
        locals.var_dxa_dn0 = ((((locals.var_vjunc_dn0 - locals.var_vfc_dn0) * locals.var_vch) - (assign3150_e2983 * locals.var_vch_dn0)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn1 = ((((locals.var_vjunc_dn1 - locals.var_vfc_dn1) * locals.var_vch) - (assign3150_e2983 * locals.var_vch_dn1)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn3 = ((((locals.var_vjunc_dn3 - locals.var_vfc_dn3) * locals.var_vch) - (assign3150_e2983 * locals.var_vch_dn3)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn4 = ((((locals.var_vjunc_dn4 - locals.var_vfc_dn4) * locals.var_vch) - (assign3150_e2983 * locals.var_vch_dn4)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn5 = ((((locals.var_vjunc_dn5 - locals.var_vfc_dn5) * locals.var_vch) - (assign3150_e2983 * locals.var_vch_dn5)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn6 = ((((locals.var_vjunc_dn6 - locals.var_vfc_dn6) * locals.var_vch) - (assign3150_e2983 * locals.var_vch_dn6)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn7 = ((((locals.var_vjunc_dn7 - locals.var_vfc_dn7) * locals.var_vch) - (assign3150_e2983 * locals.var_vch_dn7)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn8 = ((((locals.var_vjunc_dn8 - locals.var_vfc_dn8) * locals.var_vch) - (assign3150_e2983 * locals.var_vch_dn8)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn9 = ((((locals.var_vjunc_dn9 - locals.var_vfc_dn9) * locals.var_vch) - (assign3150_e2983 * locals.var_vch_dn9)) / (locals.var_vch * locals.var_vch));

        let assign3160_e2988: f64 = if locals.var_vjunc < locals.var_vfc { 1.0 } else { 0.0 };
        locals.var_guard49 = assign3160_e2988;

        let (assign3170_e3000, assign3170_e3000_d_n0, assign3170_e3000_d_n1, assign3170_e3000_d_n3, assign3170_e3000_d_n4, assign3170_e3000_d_n5, assign3170_e3000_d_n6, assign3170_e3000_d_n7, assign3170_e3000_d_n8, assign3170_e3000_d_n9,) = {
    if (locals.var_guard49 != 0.0) {
        let assign3170_e2994: f64 = (locals.var_dxa).exp();
        let assign3170_e2995: f64 = (1.0 + assign3170_e2994);
        let assign3170_e2996: f64 = (assign3170_e2995).ln();
        let assign3170_e2997: f64 = (locals.var_vch * assign3170_e2996);
        let assign3170_e2998: f64 = (locals.var_vjunc - assign3170_e2997);
        (assign3170_e2998, (locals.var_vjunc_dn0 - ((locals.var_vch_dn0 * assign3170_e2996) + (locals.var_vch * ((assign3170_e2994 * locals.var_dxa_dn0) / assign3170_e2995)))), (locals.var_vjunc_dn1 - ((locals.var_vch_dn1 * assign3170_e2996) + (locals.var_vch * ((assign3170_e2994 * locals.var_dxa_dn1) / assign3170_e2995)))), (locals.var_vjunc_dn3 - ((locals.var_vch_dn3 * assign3170_e2996) + (locals.var_vch * ((assign3170_e2994 * locals.var_dxa_dn3) / assign3170_e2995)))), (locals.var_vjunc_dn4 - ((locals.var_vch_dn4 * assign3170_e2996) + (locals.var_vch * ((assign3170_e2994 * locals.var_dxa_dn4) / assign3170_e2995)))), (locals.var_vjunc_dn5 - ((locals.var_vch_dn5 * assign3170_e2996) + (locals.var_vch * ((assign3170_e2994 * locals.var_dxa_dn5) / assign3170_e2995)))), (locals.var_vjunc_dn6 - ((locals.var_vch_dn6 * assign3170_e2996) + (locals.var_vch * ((assign3170_e2994 * locals.var_dxa_dn6) / assign3170_e2995)))), (locals.var_vjunc_dn7 - ((locals.var_vch_dn7 * assign3170_e2996) + (locals.var_vch * ((assign3170_e2994 * locals.var_dxa_dn7) / assign3170_e2995)))), (locals.var_vjunc_dn8 - ((locals.var_vch_dn8 * assign3170_e2996) + (locals.var_vch * ((assign3170_e2994 * locals.var_dxa_dn8) / assign3170_e2995)))), (locals.var_vjunc_dn9 - ((locals.var_vch_dn9 * assign3170_e2996) + (locals.var_vch * ((assign3170_e2994 * locals.var_dxa_dn9) / assign3170_e2995)))),)
    } else {
        (locals.var_vjc, locals.var_vjc_dn0, locals.var_vjc_dn1, locals.var_vjc_dn3, locals.var_vjc_dn4, locals.var_vjc_dn5, locals.var_vjc_dn6, locals.var_vjc_dn7, locals.var_vjc_dn8, locals.var_vjc_dn9,)
    }
};
        locals.var_vjc = assign3170_e3000;
        locals.var_vjc_dn0 = assign3170_e3000_d_n0;
        locals.var_vjc_dn1 = assign3170_e3000_d_n1;
        locals.var_vjc_dn3 = assign3170_e3000_d_n3;
        locals.var_vjc_dn4 = assign3170_e3000_d_n4;
        locals.var_vjc_dn5 = assign3170_e3000_d_n5;
        locals.var_vjc_dn6 = assign3170_e3000_d_n6;
        locals.var_vjc_dn7 = assign3170_e3000_d_n7;
        locals.var_vjc_dn8 = assign3170_e3000_d_n8;
        locals.var_vjc_dn9 = assign3170_e3000_d_n9;

        let (assign3180_e3014, assign3180_e3014_d_n0, assign3180_e3014_d_n1, assign3180_e3014_d_n3, assign3180_e3014_d_n4, assign3180_e3014_d_n5, assign3180_e3014_d_n6, assign3180_e3014_d_n7, assign3180_e3014_d_n8, assign3180_e3014_d_n9,) = {
    if (locals.var_guard49 == 0.0) {
        let assign3180_e3007: f64 = (-locals.var_dxa);
        let assign3180_e3008: f64 = (assign3180_e3007).exp();
        let assign3180_e3009: f64 = (1.0 + assign3180_e3008);
        let assign3180_e3010: f64 = (assign3180_e3009).ln();
        let assign3180_e3011: f64 = (locals.var_vch * assign3180_e3010);
        let assign3180_e3012: f64 = (locals.var_vfc - assign3180_e3011);
        (assign3180_e3012, (locals.var_vfc_dn0 - ((locals.var_vch_dn0 * assign3180_e3010) + (locals.var_vch * ((assign3180_e3008 * (-locals.var_dxa_dn0)) / assign3180_e3009)))), (locals.var_vfc_dn1 - ((locals.var_vch_dn1 * assign3180_e3010) + (locals.var_vch * ((assign3180_e3008 * (-locals.var_dxa_dn1)) / assign3180_e3009)))), (locals.var_vfc_dn3 - ((locals.var_vch_dn3 * assign3180_e3010) + (locals.var_vch * ((assign3180_e3008 * (-locals.var_dxa_dn3)) / assign3180_e3009)))), (locals.var_vfc_dn4 - ((locals.var_vch_dn4 * assign3180_e3010) + (locals.var_vch * ((assign3180_e3008 * (-locals.var_dxa_dn4)) / assign3180_e3009)))), (locals.var_vfc_dn5 - ((locals.var_vch_dn5 * assign3180_e3010) + (locals.var_vch * ((assign3180_e3008 * (-locals.var_dxa_dn5)) / assign3180_e3009)))), (locals.var_vfc_dn6 - ((locals.var_vch_dn6 * assign3180_e3010) + (locals.var_vch * ((assign3180_e3008 * (-locals.var_dxa_dn6)) / assign3180_e3009)))), (locals.var_vfc_dn7 - ((locals.var_vch_dn7 * assign3180_e3010) + (locals.var_vch * ((assign3180_e3008 * (-locals.var_dxa_dn7)) / assign3180_e3009)))), (locals.var_vfc_dn8 - ((locals.var_vch_dn8 * assign3180_e3010) + (locals.var_vch * ((assign3180_e3008 * (-locals.var_dxa_dn8)) / assign3180_e3009)))), (locals.var_vfc_dn9 - ((locals.var_vch_dn9 * assign3180_e3010) + (locals.var_vch * ((assign3180_e3008 * (-locals.var_dxa_dn9)) / assign3180_e3009)))),)
    } else {
        (locals.var_vjc, locals.var_vjc_dn0, locals.var_vjc_dn1, locals.var_vjc_dn3, locals.var_vjc_dn4, locals.var_vjc_dn5, locals.var_vjc_dn6, locals.var_vjc_dn7, locals.var_vjc_dn8, locals.var_vjc_dn9,)
    }
};
        locals.var_vjc = assign3180_e3014;
        locals.var_vjc_dn0 = assign3180_e3014_d_n0;
        locals.var_vjc_dn1 = assign3180_e3014_d_n1;
        locals.var_vjc_dn3 = assign3180_e3014_d_n3;
        locals.var_vjc_dn4 = assign3180_e3014_d_n4;
        locals.var_vjc_dn5 = assign3180_e3014_d_n5;
        locals.var_vjc_dn6 = assign3180_e3014_d_n6;
        locals.var_vjc_dn7 = assign3180_e3014_d_n7;
        locals.var_vjc_dn8 = assign3180_e3014_d_n8;
        locals.var_vjc_dn9 = assign3180_e3014_d_n9;

        let assign3190_e3017: f64 = (locals.var_icap_ihc).powf(p.p75);
        locals.var_fi = assign3190_e3017;
        locals.var_fi_dn0 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn0)) } } else { (assign3190_e3017 * (p.p75 * (locals.var_icap_ihc_dn0 / locals.var_icap_ihc))) };
        locals.var_fi_dn1 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn1)) } } else { (assign3190_e3017 * (p.p75 * (locals.var_icap_ihc_dn1 / locals.var_icap_ihc))) };
        locals.var_fi_dn3 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn3)) } } else { (assign3190_e3017 * (p.p75 * (locals.var_icap_ihc_dn3 / locals.var_icap_ihc))) };
        locals.var_fi_dn4 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn4)) } } else { (assign3190_e3017 * (p.p75 * (locals.var_icap_ihc_dn4 / locals.var_icap_ihc))) };
        locals.var_fi_dn5 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn5)) } } else { (assign3190_e3017 * (p.p75 * (locals.var_icap_ihc_dn5 / locals.var_icap_ihc))) };
        locals.var_fi_dn6 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn6)) } } else { (assign3190_e3017 * (p.p75 * (locals.var_icap_ihc_dn6 / locals.var_icap_ihc))) };
        locals.var_fi_dn7 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn7)) } } else { (assign3190_e3017 * (p.p75 * (locals.var_icap_ihc_dn7 / locals.var_icap_ihc))) };
        locals.var_fi_dn8 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn8)) } } else { (assign3190_e3017 * (p.p75 * (locals.var_icap_ihc_dn8 / locals.var_icap_ihc))) };
        locals.var_fi_dn9 = if 0.0 == 0.0 && ((p.p75) as f64).is_finite() && ((p.p75) as f64).fract() == 0.0 { if p.p75 == 0.0 { 0.0 } else { (p.p75 * ((locals.var_icap_ihc).powf(p.p75 - 1.0) * locals.var_icap_ihc_dn9)) } } else { (assign3190_e3017 * (p.p75 * (locals.var_icap_ihc_dn9 / locals.var_icap_ihc))) };

        let assign3200_e3021: f64 = (1.0 - p.p71);
        let assign3200_e3022: f64 = (locals.var_vdc_ctc_t / assign3200_e3021);
        let assign3200_e3028: f64 = (locals.var_vjc / locals.var_vdc_ctc_t);
        let assign3200_e3029: f64 = (1.0 - assign3200_e3028);
        let assign3200_e3032: f64 = (1.0 - p.p71);
        let assign3200_e3033: f64 = (assign3200_e3029).powf(assign3200_e3032);
        let assign3200_e3034: f64 = (locals.var_fi * assign3200_e3033);
        let assign3200_e3035: f64 = (1.0 - assign3200_e3034);
        let assign3200_e3036: f64 = (assign3200_e3022 * assign3200_e3035);
        let assign3200_e3039: f64 = (locals.var_fi * locals.var_bjc);
        let assign3200_e3042: f64 = (locals.var_vjunc - locals.var_vjc);
        let assign3200_e3043: f64 = (assign3200_e3039 * assign3200_e3042);
        let assign3200_e3044: f64 = (assign3200_e3036 + assign3200_e3043);
        locals.var_vcv = assign3200_e3044;
        locals.var_vcv_dn0 = ((((locals.var_vdc_ctc_t_dn0 / assign3200_e3021) * assign3200_e3035) + (assign3200_e3022 * (-((locals.var_fi_dn0 * assign3200_e3033) + (locals.var_fi * if 0.0 == 0.0 && ((assign3200_e3032) as f64).is_finite() && ((assign3200_e3032) as f64).fract() == 0.0 { if assign3200_e3032 == 0.0 { 0.0 } else { (assign3200_e3032 * ((assign3200_e3029).powf(assign3200_e3032 - 1.0) * (-(((locals.var_vjc_dn0 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3200_e3033 * (assign3200_e3032 * ((-(((locals.var_vjc_dn0 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3200_e3029))) }))))) + ((((locals.var_fi_dn0 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn0)) * assign3200_e3042) + (assign3200_e3039 * (locals.var_vjunc_dn0 - locals.var_vjc_dn0))));
        locals.var_vcv_dn1 = ((((locals.var_vdc_ctc_t_dn1 / assign3200_e3021) * assign3200_e3035) + (assign3200_e3022 * (-((locals.var_fi_dn1 * assign3200_e3033) + (locals.var_fi * if 0.0 == 0.0 && ((assign3200_e3032) as f64).is_finite() && ((assign3200_e3032) as f64).fract() == 0.0 { if assign3200_e3032 == 0.0 { 0.0 } else { (assign3200_e3032 * ((assign3200_e3029).powf(assign3200_e3032 - 1.0) * (-(((locals.var_vjc_dn1 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3200_e3033 * (assign3200_e3032 * ((-(((locals.var_vjc_dn1 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3200_e3029))) }))))) + ((((locals.var_fi_dn1 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn1)) * assign3200_e3042) + (assign3200_e3039 * (locals.var_vjunc_dn1 - locals.var_vjc_dn1))));
        locals.var_vcv_dn3 = ((((locals.var_vdc_ctc_t_dn3 / assign3200_e3021) * assign3200_e3035) + (assign3200_e3022 * (-((locals.var_fi_dn3 * assign3200_e3033) + (locals.var_fi * if 0.0 == 0.0 && ((assign3200_e3032) as f64).is_finite() && ((assign3200_e3032) as f64).fract() == 0.0 { if assign3200_e3032 == 0.0 { 0.0 } else { (assign3200_e3032 * ((assign3200_e3029).powf(assign3200_e3032 - 1.0) * (-(((locals.var_vjc_dn3 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3200_e3033 * (assign3200_e3032 * ((-(((locals.var_vjc_dn3 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3200_e3029))) }))))) + ((((locals.var_fi_dn3 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn3)) * assign3200_e3042) + (assign3200_e3039 * (locals.var_vjunc_dn3 - locals.var_vjc_dn3))));
        locals.var_vcv_dn4 = ((((locals.var_vdc_ctc_t_dn4 / assign3200_e3021) * assign3200_e3035) + (assign3200_e3022 * (-((locals.var_fi_dn4 * assign3200_e3033) + (locals.var_fi * if 0.0 == 0.0 && ((assign3200_e3032) as f64).is_finite() && ((assign3200_e3032) as f64).fract() == 0.0 { if assign3200_e3032 == 0.0 { 0.0 } else { (assign3200_e3032 * ((assign3200_e3029).powf(assign3200_e3032 - 1.0) * (-(((locals.var_vjc_dn4 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3200_e3033 * (assign3200_e3032 * ((-(((locals.var_vjc_dn4 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3200_e3029))) }))))) + ((((locals.var_fi_dn4 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn4)) * assign3200_e3042) + (assign3200_e3039 * (locals.var_vjunc_dn4 - locals.var_vjc_dn4))));
        locals.var_vcv_dn5 = ((((locals.var_vdc_ctc_t_dn5 / assign3200_e3021) * assign3200_e3035) + (assign3200_e3022 * (-((locals.var_fi_dn5 * assign3200_e3033) + (locals.var_fi * if 0.0 == 0.0 && ((assign3200_e3032) as f64).is_finite() && ((assign3200_e3032) as f64).fract() == 0.0 { if assign3200_e3032 == 0.0 { 0.0 } else { (assign3200_e3032 * ((assign3200_e3029).powf(assign3200_e3032 - 1.0) * (-(((locals.var_vjc_dn5 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3200_e3033 * (assign3200_e3032 * ((-(((locals.var_vjc_dn5 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3200_e3029))) }))))) + ((((locals.var_fi_dn5 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn5)) * assign3200_e3042) + (assign3200_e3039 * (locals.var_vjunc_dn5 - locals.var_vjc_dn5))));
        locals.var_vcv_dn6 = ((((locals.var_vdc_ctc_t_dn6 / assign3200_e3021) * assign3200_e3035) + (assign3200_e3022 * (-((locals.var_fi_dn6 * assign3200_e3033) + (locals.var_fi * if 0.0 == 0.0 && ((assign3200_e3032) as f64).is_finite() && ((assign3200_e3032) as f64).fract() == 0.0 { if assign3200_e3032 == 0.0 { 0.0 } else { (assign3200_e3032 * ((assign3200_e3029).powf(assign3200_e3032 - 1.0) * (-(((locals.var_vjc_dn6 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3200_e3033 * (assign3200_e3032 * ((-(((locals.var_vjc_dn6 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3200_e3029))) }))))) + ((((locals.var_fi_dn6 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn6)) * assign3200_e3042) + (assign3200_e3039 * (locals.var_vjunc_dn6 - locals.var_vjc_dn6))));
        locals.var_vcv_dn7 = ((((locals.var_vdc_ctc_t_dn7 / assign3200_e3021) * assign3200_e3035) + (assign3200_e3022 * (-((locals.var_fi_dn7 * assign3200_e3033) + (locals.var_fi * if 0.0 == 0.0 && ((assign3200_e3032) as f64).is_finite() && ((assign3200_e3032) as f64).fract() == 0.0 { if assign3200_e3032 == 0.0 { 0.0 } else { (assign3200_e3032 * ((assign3200_e3029).powf(assign3200_e3032 - 1.0) * (-(((locals.var_vjc_dn7 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3200_e3033 * (assign3200_e3032 * ((-(((locals.var_vjc_dn7 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3200_e3029))) }))))) + ((((locals.var_fi_dn7 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn7)) * assign3200_e3042) + (assign3200_e3039 * (locals.var_vjunc_dn7 - locals.var_vjc_dn7))));
        locals.var_vcv_dn8 = ((((locals.var_vdc_ctc_t_dn8 / assign3200_e3021) * assign3200_e3035) + (assign3200_e3022 * (-((locals.var_fi_dn8 * assign3200_e3033) + (locals.var_fi * if 0.0 == 0.0 && ((assign3200_e3032) as f64).is_finite() && ((assign3200_e3032) as f64).fract() == 0.0 { if assign3200_e3032 == 0.0 { 0.0 } else { (assign3200_e3032 * ((assign3200_e3029).powf(assign3200_e3032 - 1.0) * (-(((locals.var_vjc_dn8 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3200_e3033 * (assign3200_e3032 * ((-(((locals.var_vjc_dn8 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3200_e3029))) }))))) + ((((locals.var_fi_dn8 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn8)) * assign3200_e3042) + (assign3200_e3039 * (locals.var_vjunc_dn8 - locals.var_vjc_dn8))));
        locals.var_vcv_dn9 = ((((locals.var_vdc_ctc_t_dn9 / assign3200_e3021) * assign3200_e3035) + (assign3200_e3022 * (-((locals.var_fi_dn9 * assign3200_e3033) + (locals.var_fi * if 0.0 == 0.0 && ((assign3200_e3032) as f64).is_finite() && ((assign3200_e3032) as f64).fract() == 0.0 { if assign3200_e3032 == 0.0 { 0.0 } else { (assign3200_e3032 * ((assign3200_e3029).powf(assign3200_e3032 - 1.0) * (-(((locals.var_vjc_dn9 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3200_e3033 * (assign3200_e3032 * ((-(((locals.var_vjc_dn9 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3200_e3029))) }))))) + ((((locals.var_fi_dn9 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn9)) * assign3200_e3042) + (assign3200_e3039 * (locals.var_vjunc_dn9 - locals.var_vjc_dn9))));

        let assign3210_e3047: f64 = (1.0 - locals.var_xp_t);
        let assign3210_e3049: f64 = (assign3210_e3047 * locals.var_vcv);
        let assign3210_e3052: f64 = (locals.var_xp_t * locals.var_vb2c1);
        let assign3210_e3053: f64 = (assign3210_e3049 + assign3210_e3052);
        locals.var_vtc = assign3210_e3053;
        locals.var_vtc_dn0 = ((((-locals.var_xp_t_dn0) * locals.var_vcv) + (assign3210_e3047 * locals.var_vcv_dn0)) + (locals.var_xp_t_dn0 * locals.var_vb2c1));
        locals.var_vtc_dn1 = ((((-locals.var_xp_t_dn1) * locals.var_vcv) + (assign3210_e3047 * locals.var_vcv_dn1)) + (locals.var_xp_t_dn1 * locals.var_vb2c1));
        locals.var_vtc_dn3 = ((((-locals.var_xp_t_dn3) * locals.var_vcv) + (assign3210_e3047 * locals.var_vcv_dn3)) + (locals.var_xp_t_dn3 * locals.var_vb2c1));
        locals.var_vtc_dn4 = ((((-locals.var_xp_t_dn4) * locals.var_vcv) + (assign3210_e3047 * locals.var_vcv_dn4)) + (locals.var_xp_t_dn4 * locals.var_vb2c1));
        locals.var_vtc_dn5 = ((((-locals.var_xp_t_dn5) * locals.var_vcv) + (assign3210_e3047 * locals.var_vcv_dn5)) + ((locals.var_xp_t_dn5 * locals.var_vb2c1) + (locals.var_xp_t * locals.var_vb2c1_dn5)));
        locals.var_vtc_dn6 = ((((-locals.var_xp_t_dn6) * locals.var_vcv) + (assign3210_e3047 * locals.var_vcv_dn6)) + ((locals.var_xp_t_dn6 * locals.var_vb2c1) + (locals.var_xp_t * locals.var_vb2c1_dn6)));
        locals.var_vtc_dn7 = ((((-locals.var_xp_t_dn7) * locals.var_vcv) + (assign3210_e3047 * locals.var_vcv_dn7)) + (locals.var_xp_t_dn7 * locals.var_vb2c1));
        locals.var_vtc_dn8 = ((((-locals.var_xp_t_dn8) * locals.var_vcv) + (assign3210_e3047 * locals.var_vcv_dn8)) + (locals.var_xp_t_dn8 * locals.var_vb2c1));
        locals.var_vtc_dn9 = ((((-locals.var_xp_t_dn9) * locals.var_vcv) + (assign3210_e3047 * locals.var_vcv_dn9)) + (locals.var_xp_t_dn9 * locals.var_vb2c1));

        let assign3220_e3056: f64 = (4.0 * locals.var_is_t);
        let assign3220_e3058: f64 = (assign3220_e3056 / locals.var_ik_t);
        locals.var_if0 = assign3220_e3058;
        locals.var_if0_dn0 = ((4.0 * locals.var_is_t_dn0) / locals.var_ik_t);
        locals.var_if0_dn1 = ((4.0 * locals.var_is_t_dn1) / locals.var_ik_t);
        locals.var_if0_dn3 = ((4.0 * locals.var_is_t_dn3) / locals.var_ik_t);
        locals.var_if0_dn4 = ((4.0 * locals.var_is_t_dn4) / locals.var_ik_t);
        locals.var_if0_dn5 = ((4.0 * locals.var_is_t_dn5) / locals.var_ik_t);
        locals.var_if0_dn6 = ((4.0 * locals.var_is_t_dn6) / locals.var_ik_t);
        locals.var_if0_dn7 = ((4.0 * locals.var_is_t_dn7) / locals.var_ik_t);
        locals.var_if0_dn8 = ((4.0 * locals.var_is_t_dn8) / locals.var_ik_t);
        locals.var_if0_dn9 = ((4.0 * locals.var_is_t_dn9) / locals.var_ik_t);

        let assign3230_e3061: f64 = (locals.var_if0 * locals.var_evb2e1);
        locals.var_f1 = assign3230_e3061;
        locals.var_f1_dn0 = ((locals.var_if0_dn0 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn0));
        locals.var_f1_dn1 = ((locals.var_if0_dn1 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn1));
        locals.var_f1_dn3 = ((locals.var_if0_dn3 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn3));
        locals.var_f1_dn4 = ((locals.var_if0_dn4 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn4));
        locals.var_f1_dn5 = ((locals.var_if0_dn5 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn5));
        locals.var_f1_dn6 = ((locals.var_if0_dn6 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn6));
        locals.var_f1_dn7 = ((locals.var_if0_dn7 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn7));
        locals.var_f1_dn8 = ((locals.var_if0_dn8 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn8));
        locals.var_f1_dn9 = ((locals.var_if0_dn9 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn9));

        let assign3240_e3066: f64 = (1.0 + locals.var_f1);
        let assign3240_e3067: f64 = (assign3240_e3066).sqrt();
        let assign3240_e3068: f64 = (1.0 + assign3240_e3067);
        let assign3240_e3069: f64 = (locals.var_f1 / assign3240_e3068);
        locals.var_n0 = assign3240_e3069;
        locals.var_n0_dn0 = (((locals.var_f1_dn0 * assign3240_e3068) - (locals.var_f1 * (locals.var_f1_dn0 / (2.0 * assign3240_e3067)))) / (assign3240_e3068 * assign3240_e3068));
        locals.var_n0_dn1 = (((locals.var_f1_dn1 * assign3240_e3068) - (locals.var_f1 * (locals.var_f1_dn1 / (2.0 * assign3240_e3067)))) / (assign3240_e3068 * assign3240_e3068));
        locals.var_n0_dn3 = (((locals.var_f1_dn3 * assign3240_e3068) - (locals.var_f1 * (locals.var_f1_dn3 / (2.0 * assign3240_e3067)))) / (assign3240_e3068 * assign3240_e3068));
        locals.var_n0_dn4 = (((locals.var_f1_dn4 * assign3240_e3068) - (locals.var_f1 * (locals.var_f1_dn4 / (2.0 * assign3240_e3067)))) / (assign3240_e3068 * assign3240_e3068));
        locals.var_n0_dn5 = (((locals.var_f1_dn5 * assign3240_e3068) - (locals.var_f1 * (locals.var_f1_dn5 / (2.0 * assign3240_e3067)))) / (assign3240_e3068 * assign3240_e3068));
        locals.var_n0_dn6 = (((locals.var_f1_dn6 * assign3240_e3068) - (locals.var_f1 * (locals.var_f1_dn6 / (2.0 * assign3240_e3067)))) / (assign3240_e3068 * assign3240_e3068));
        locals.var_n0_dn7 = (((locals.var_f1_dn7 * assign3240_e3068) - (locals.var_f1 * (locals.var_f1_dn7 / (2.0 * assign3240_e3067)))) / (assign3240_e3068 * assign3240_e3068));
        locals.var_n0_dn8 = (((locals.var_f1_dn8 * assign3240_e3068) - (locals.var_f1 * (locals.var_f1_dn8 / (2.0 * assign3240_e3067)))) / (assign3240_e3068 * assign3240_e3068));
        locals.var_n0_dn9 = (((locals.var_f1_dn9 * assign3240_e3068) - (locals.var_f1 * (locals.var_f1_dn9 / (2.0 * assign3240_e3067)))) / (assign3240_e3068 * assign3240_e3068));

        let assign3250_e3073: f64 = (1.0 / locals.var_nfr_t);
        let assign3250_e3074: f64 = (locals.var_evb2c2star).powf(assign3250_e3073);
        locals.var_evb2c2star_nfr = assign3250_e3074;
        locals.var_evb2c2star_nfr_dn0 = if (-(locals.var_nfr_t_dn0 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3250_e3073) as f64).is_finite() && ((assign3250_e3073) as f64).fract() == 0.0 { if assign3250_e3073 == 0.0 { 0.0 } else { (assign3250_e3073 * ((locals.var_evb2c2star).powf(assign3250_e3073 - 1.0) * locals.var_evb2c2star_dn0)) } } else { (assign3250_e3074 * (((-(locals.var_nfr_t_dn0 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3250_e3073 * (locals.var_evb2c2star_dn0 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn1 = if (-(locals.var_nfr_t_dn1 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3250_e3073) as f64).is_finite() && ((assign3250_e3073) as f64).fract() == 0.0 { if assign3250_e3073 == 0.0 { 0.0 } else { (assign3250_e3073 * ((locals.var_evb2c2star).powf(assign3250_e3073 - 1.0) * locals.var_evb2c2star_dn1)) } } else { (assign3250_e3074 * (((-(locals.var_nfr_t_dn1 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3250_e3073 * (locals.var_evb2c2star_dn1 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn3 = if (-(locals.var_nfr_t_dn3 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3250_e3073) as f64).is_finite() && ((assign3250_e3073) as f64).fract() == 0.0 { if assign3250_e3073 == 0.0 { 0.0 } else { (assign3250_e3073 * ((locals.var_evb2c2star).powf(assign3250_e3073 - 1.0) * locals.var_evb2c2star_dn3)) } } else { (assign3250_e3074 * (((-(locals.var_nfr_t_dn3 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3250_e3073 * (locals.var_evb2c2star_dn3 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn4 = if (-(locals.var_nfr_t_dn4 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3250_e3073) as f64).is_finite() && ((assign3250_e3073) as f64).fract() == 0.0 { if assign3250_e3073 == 0.0 { 0.0 } else { (assign3250_e3073 * ((locals.var_evb2c2star).powf(assign3250_e3073 - 1.0) * locals.var_evb2c2star_dn4)) } } else { (assign3250_e3074 * (((-(locals.var_nfr_t_dn4 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3250_e3073 * (locals.var_evb2c2star_dn4 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn5 = if (-(locals.var_nfr_t_dn5 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3250_e3073) as f64).is_finite() && ((assign3250_e3073) as f64).fract() == 0.0 { if assign3250_e3073 == 0.0 { 0.0 } else { (assign3250_e3073 * ((locals.var_evb2c2star).powf(assign3250_e3073 - 1.0) * locals.var_evb2c2star_dn5)) } } else { (assign3250_e3074 * (((-(locals.var_nfr_t_dn5 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3250_e3073 * (locals.var_evb2c2star_dn5 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn6 = if (-(locals.var_nfr_t_dn6 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3250_e3073) as f64).is_finite() && ((assign3250_e3073) as f64).fract() == 0.0 { if assign3250_e3073 == 0.0 { 0.0 } else { (assign3250_e3073 * ((locals.var_evb2c2star).powf(assign3250_e3073 - 1.0) * locals.var_evb2c2star_dn6)) } } else { (assign3250_e3074 * (((-(locals.var_nfr_t_dn6 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3250_e3073 * (locals.var_evb2c2star_dn6 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn7 = if (-(locals.var_nfr_t_dn7 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3250_e3073) as f64).is_finite() && ((assign3250_e3073) as f64).fract() == 0.0 { if assign3250_e3073 == 0.0 { 0.0 } else { (assign3250_e3073 * ((locals.var_evb2c2star).powf(assign3250_e3073 - 1.0) * locals.var_evb2c2star_dn7)) } } else { (assign3250_e3074 * (((-(locals.var_nfr_t_dn7 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3250_e3073 * (locals.var_evb2c2star_dn7 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn8 = if (-(locals.var_nfr_t_dn8 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3250_e3073) as f64).is_finite() && ((assign3250_e3073) as f64).fract() == 0.0 { if assign3250_e3073 == 0.0 { 0.0 } else { (assign3250_e3073 * ((locals.var_evb2c2star).powf(assign3250_e3073 - 1.0) * locals.var_evb2c2star_dn8)) } } else { (assign3250_e3074 * (((-(locals.var_nfr_t_dn8 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3250_e3073 * (locals.var_evb2c2star_dn8 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn9 = if (-(locals.var_nfr_t_dn9 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3250_e3073) as f64).is_finite() && ((assign3250_e3073) as f64).fract() == 0.0 { if assign3250_e3073 == 0.0 { 0.0 } else { (assign3250_e3073 * ((locals.var_evb2c2star).powf(assign3250_e3073 - 1.0) * locals.var_evb2c2star_dn9)) } } else { (assign3250_e3074 * (((-(locals.var_nfr_t_dn9 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3250_e3073 * (locals.var_evb2c2star_dn9 / locals.var_evb2c2star)))) };

        let assign3260_e3077: f64 = (locals.var_if0 * locals.var_evb2c2star_nfr);
        locals.var_f2 = assign3260_e3077;
        locals.var_f2_dn0 = ((locals.var_if0_dn0 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn0));
        locals.var_f2_dn1 = ((locals.var_if0_dn1 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn1));
        locals.var_f2_dn3 = ((locals.var_if0_dn3 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn3));
        locals.var_f2_dn4 = ((locals.var_if0_dn4 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn4));
        locals.var_f2_dn5 = ((locals.var_if0_dn5 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn5));
        locals.var_f2_dn6 = ((locals.var_if0_dn6 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn6));
        locals.var_f2_dn7 = ((locals.var_if0_dn7 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn7));
        locals.var_f2_dn8 = ((locals.var_if0_dn8 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn8));
        locals.var_f2_dn9 = ((locals.var_if0_dn9 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn9));

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3270_e3082: f64 = (1.0 + locals.var_f2);
        let assign3270_e3083: f64 = (assign3270_e3082).sqrt();
        let assign3270_e3084: f64 = (1.0 + assign3270_e3083);
        let assign3270_e3085: f64 = (locals.var_f2 / assign3270_e3084);
        locals.var_nb = assign3270_e3085;
        locals.var_nb_dn0 = (((locals.var_f2_dn0 * assign3270_e3084) - (locals.var_f2 * (locals.var_f2_dn0 / (2.0 * assign3270_e3083)))) / (assign3270_e3084 * assign3270_e3084));
        locals.var_nb_dn1 = (((locals.var_f2_dn1 * assign3270_e3084) - (locals.var_f2 * (locals.var_f2_dn1 / (2.0 * assign3270_e3083)))) / (assign3270_e3084 * assign3270_e3084));
        locals.var_nb_dn3 = (((locals.var_f2_dn3 * assign3270_e3084) - (locals.var_f2 * (locals.var_f2_dn3 / (2.0 * assign3270_e3083)))) / (assign3270_e3084 * assign3270_e3084));
        locals.var_nb_dn4 = (((locals.var_f2_dn4 * assign3270_e3084) - (locals.var_f2 * (locals.var_f2_dn4 / (2.0 * assign3270_e3083)))) / (assign3270_e3084 * assign3270_e3084));
        locals.var_nb_dn5 = (((locals.var_f2_dn5 * assign3270_e3084) - (locals.var_f2 * (locals.var_f2_dn5 / (2.0 * assign3270_e3083)))) / (assign3270_e3084 * assign3270_e3084));
        locals.var_nb_dn6 = (((locals.var_f2_dn6 * assign3270_e3084) - (locals.var_f2 * (locals.var_f2_dn6 / (2.0 * assign3270_e3083)))) / (assign3270_e3084 * assign3270_e3084));
        locals.var_nb_dn7 = (((locals.var_f2_dn7 * assign3270_e3084) - (locals.var_f2 * (locals.var_f2_dn7 / (2.0 * assign3270_e3083)))) / (assign3270_e3084 * assign3270_e3084));
        locals.var_nb_dn8 = (((locals.var_f2_dn8 * assign3270_e3084) - (locals.var_f2 * (locals.var_f2_dn8 / (2.0 * assign3270_e3083)))) / (assign3270_e3084 * assign3270_e3084));
        locals.var_nb_dn9 = (((locals.var_f2_dn9 * assign3270_e3084) - (locals.var_f2 * (locals.var_f2_dn9 / (2.0 * assign3270_e3083)))) / (assign3270_e3084 * assign3270_e3084));

        let assign3280_e3088: f64 = if p.p91 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard50 = assign3280_e3088;

        let (assign3290_e3100, assign3290_e3100_d_n0, assign3290_e3100_d_n1, assign3290_e3100_d_n3, assign3290_e3100_d_n4, assign3290_e3100_d_n5, assign3290_e3100_d_n6, assign3290_e3100_d_n7, assign3290_e3100_d_n8, assign3290_e3100_d_n9,) = {
    if (locals.var_guard50 != 0.0) {
        let assign3290_e3093: f64 = (locals.var_vte / locals.var_ver_t);
        let assign3290_e3094: f64 = (1.0 + assign3290_e3093);
        let assign3290_e3097: f64 = (locals.var_vtc / locals.var_vef_t);
        let assign3290_e3098: f64 = (assign3290_e3094 + assign3290_e3097);
        (assign3290_e3098, ((((locals.var_vte_dn0 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn0)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn0 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn0)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn1 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn1)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn1 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn1)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn3 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn3)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn3 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn3)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn4 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn4)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn4 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn4)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn5 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn5)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn5 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn5)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn6 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn6)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn6 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn6)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn7 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn7)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn7 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn7)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn8 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn8)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn8 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn8)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn9 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn9)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn9 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn9)) / (locals.var_vef_t * locals.var_vef_t))),)
    } else {
        (locals.var_q0i, locals.var_q0i_dn0, locals.var_q0i_dn1, locals.var_q0i_dn3, locals.var_q0i_dn4, locals.var_q0i_dn5, locals.var_q0i_dn6, locals.var_q0i_dn7, locals.var_q0i_dn8, locals.var_q0i_dn9,)
    }
};
        locals.var_q0i = assign3290_e3100;
        locals.var_q0i_dn0 = assign3290_e3100_d_n0;
        locals.var_q0i_dn1 = assign3290_e3100_d_n1;
        locals.var_q0i_dn3 = assign3290_e3100_d_n3;
        locals.var_q0i_dn4 = assign3290_e3100_d_n4;
        locals.var_q0i_dn5 = assign3290_e3100_d_n5;
        locals.var_q0i_dn6 = assign3290_e3100_d_n6;
        locals.var_q0i_dn7 = assign3290_e3100_d_n7;
        locals.var_q0i_dn8 = assign3290_e3100_d_n8;
        locals.var_q0i_dn9 = assign3290_e3100_d_n9;

        let (assign3300_e3113, assign3300_e3113_d_n0, assign3300_e3113_d_n1, assign3300_e3113_d_n3, assign3300_e3113_d_n4, assign3300_e3113_d_n5, assign3300_e3113_d_n6, assign3300_e3113_d_n7, assign3300_e3113_d_n8, assign3300_e3113_d_n9,) = {
    if (locals.var_guard50 == 0.0) {
        let assign3300_e3105: f64 = (locals.var_vte / locals.var_ver_t);
        let assign3300_e3107: f64 = (assign3300_e3105 + 1.0);
        let assign3300_e3109: f64 = (assign3300_e3107 * locals.var_deg_t);
        let assign3300_e3111: f64 = (assign3300_e3109 * locals.var_vtinv);
        (assign3300_e3111, (((((locals.var_vte_dn0 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn0)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn1 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn1)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn3 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn3)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn4 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn4)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn5 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn5)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn6 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn6)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn7 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn7)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn8 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn8)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn9 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn9)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv),)
    } else {
        (locals.var_terme, locals.var_terme_dn0, locals.var_terme_dn1, locals.var_terme_dn3, locals.var_terme_dn4, locals.var_terme_dn5, locals.var_terme_dn6, locals.var_terme_dn7, locals.var_terme_dn8, locals.var_terme_dn9,)
    }
};
        locals.var_terme = assign3300_e3113;
        locals.var_terme_dn0 = assign3300_e3113_d_n0;
        locals.var_terme_dn1 = assign3300_e3113_d_n1;
        locals.var_terme_dn3 = assign3300_e3113_d_n3;
        locals.var_terme_dn4 = assign3300_e3113_d_n4;
        locals.var_terme_dn5 = assign3300_e3113_d_n5;
        locals.var_terme_dn6 = assign3300_e3113_d_n6;
        locals.var_terme_dn7 = assign3300_e3113_d_n7;
        locals.var_terme_dn8 = assign3300_e3113_d_n8;
        locals.var_terme_dn9 = assign3300_e3113_d_n9;

        let (assign3310_e3125, assign3310_e3125_d_n0, assign3310_e3125_d_n1, assign3310_e3125_d_n3, assign3310_e3125_d_n4, assign3310_e3125_d_n5, assign3310_e3125_d_n6, assign3310_e3125_d_n7, assign3310_e3125_d_n8, assign3310_e3125_d_n9,) = {
    if (locals.var_guard50 == 0.0) {
        let assign3310_e3117: f64 = (-locals.var_vtc);
        let assign3310_e3119: f64 = (assign3310_e3117 / locals.var_vef_t);
        let assign3310_e3121: f64 = (assign3310_e3119 * locals.var_deg_t);
        let assign3310_e3123: f64 = (assign3310_e3121 * locals.var_vtinv);
        (assign3310_e3123, ((((((-locals.var_vtc_dn0) * locals.var_vef_t) - (assign3310_e3117 * locals.var_vef_t_dn0)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn1) * locals.var_vef_t) - (assign3310_e3117 * locals.var_vef_t_dn1)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn3) * locals.var_vef_t) - (assign3310_e3117 * locals.var_vef_t_dn3)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn4) * locals.var_vef_t) - (assign3310_e3117 * locals.var_vef_t_dn4)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn5) * locals.var_vef_t) - (assign3310_e3117 * locals.var_vef_t_dn5)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn6) * locals.var_vef_t) - (assign3310_e3117 * locals.var_vef_t_dn6)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn7) * locals.var_vef_t) - (assign3310_e3117 * locals.var_vef_t_dn7)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn8) * locals.var_vef_t) - (assign3310_e3117 * locals.var_vef_t_dn8)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn9) * locals.var_vef_t) - (assign3310_e3117 * locals.var_vef_t_dn9)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv),)
    } else {
        (locals.var_termc, locals.var_termc_dn0, locals.var_termc_dn1, locals.var_termc_dn3, locals.var_termc_dn4, locals.var_termc_dn5, locals.var_termc_dn6, locals.var_termc_dn7, locals.var_termc_dn8, locals.var_termc_dn9,)
    }
};
        locals.var_termc = assign3310_e3125;
        locals.var_termc_dn0 = assign3310_e3125_d_n0;
        locals.var_termc_dn1 = assign3310_e3125_d_n1;
        locals.var_termc_dn3 = assign3310_e3125_d_n3;
        locals.var_termc_dn4 = assign3310_e3125_d_n4;
        locals.var_termc_dn5 = assign3310_e3125_d_n5;
        locals.var_termc_dn6 = assign3310_e3125_d_n6;
        locals.var_termc_dn7 = assign3310_e3125_d_n7;
        locals.var_termc_dn8 = assign3310_e3125_d_n8;
        locals.var_termc_dn9 = assign3310_e3125_d_n9;

        let (assign3320_e3141, assign3320_e3141_d_n0, assign3320_e3141_d_n1, assign3320_e3141_d_n3, assign3320_e3141_d_n4, assign3320_e3141_d_n5, assign3320_e3141_d_n6, assign3320_e3141_d_n7, assign3320_e3141_d_n8, assign3320_e3141_d_n9,) = {
    if (locals.var_guard50 == 0.0) {
        let assign3320_e3129: f64 = (locals.var_terme).exp();
        let assign3320_e3131: f64 = (locals.var_termc).exp();
        let assign3320_e3132: f64 = (assign3320_e3129 - assign3320_e3131);
        let assign3320_e3135: f64 = (locals.var_deg_t * locals.var_vtinv);
        let assign3320_e3136: f64 = (assign3320_e3135).exp();
        let assign3320_e3138: f64 = (assign3320_e3136 - 1.0);
        let assign3320_e3139: f64 = (assign3320_e3132 / assign3320_e3138);
        (assign3320_e3139, (((assign3320_e3129 * locals.var_terme_dn0) - (assign3320_e3131 * locals.var_termc_dn0)) / assign3320_e3138), (((assign3320_e3129 * locals.var_terme_dn1) - (assign3320_e3131 * locals.var_termc_dn1)) / assign3320_e3138), (((assign3320_e3129 * locals.var_terme_dn3) - (assign3320_e3131 * locals.var_termc_dn3)) / assign3320_e3138), (((assign3320_e3129 * locals.var_terme_dn4) - (assign3320_e3131 * locals.var_termc_dn4)) / assign3320_e3138), (((assign3320_e3129 * locals.var_terme_dn5) - (assign3320_e3131 * locals.var_termc_dn5)) / assign3320_e3138), (((assign3320_e3129 * locals.var_terme_dn6) - (assign3320_e3131 * locals.var_termc_dn6)) / assign3320_e3138), (((assign3320_e3129 * locals.var_terme_dn7) - (assign3320_e3131 * locals.var_termc_dn7)) / assign3320_e3138), (((assign3320_e3129 * locals.var_terme_dn8) - (assign3320_e3131 * locals.var_termc_dn8)) / assign3320_e3138), (((assign3320_e3129 * locals.var_terme_dn9) - (assign3320_e3131 * locals.var_termc_dn9)) / assign3320_e3138),)
    } else {
        (locals.var_q0i, locals.var_q0i_dn0, locals.var_q0i_dn1, locals.var_q0i_dn3, locals.var_q0i_dn4, locals.var_q0i_dn5, locals.var_q0i_dn6, locals.var_q0i_dn7, locals.var_q0i_dn8, locals.var_q0i_dn9,)
    }
};
        locals.var_q0i = assign3320_e3141;
        locals.var_q0i_dn0 = assign3320_e3141_d_n0;
        locals.var_q0i_dn1 = assign3320_e3141_d_n1;
        locals.var_q0i_dn3 = assign3320_e3141_d_n3;
        locals.var_q0i_dn4 = assign3320_e3141_d_n4;
        locals.var_q0i_dn5 = assign3320_e3141_d_n5;
        locals.var_q0i_dn6 = assign3320_e3141_d_n6;
        locals.var_q0i_dn7 = assign3320_e3141_d_n7;
        locals.var_q0i_dn8 = assign3320_e3141_d_n8;
        locals.var_q0i_dn9 = assign3320_e3141_d_n9;

        let assign3330_e3144: f64 = (0.1 * 0.1);
        locals.var_eps2 = assign3330_e3144;
        locals.var_eps2_dn0 = 0.0;
        locals.var_eps2_dn1 = 0.0;
        locals.var_eps2_dn3 = 0.0;
        locals.var_eps2_dn4 = 0.0;
        locals.var_eps2_dn5 = 0.0;
        locals.var_eps2_dn6 = 0.0;
        locals.var_eps2_dn7 = 0.0;
        locals.var_eps2_dn8 = 0.0;
        locals.var_eps2_dn9 = 0.0;

        let assign3340_e3147: f64 = (locals.var_q0i * locals.var_q0i);
        locals.var_x2 = assign3340_e3147;
        locals.var_x2_dn0 = ((locals.var_q0i_dn0 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn0));
        locals.var_x2_dn1 = ((locals.var_q0i_dn1 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn1));
        locals.var_x2_dn3 = ((locals.var_q0i_dn3 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn3));
        locals.var_x2_dn4 = ((locals.var_q0i_dn4 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn4));
        locals.var_x2_dn5 = ((locals.var_q0i_dn5 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn5));
        locals.var_x2_dn6 = ((locals.var_q0i_dn6 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn6));
        locals.var_x2_dn7 = ((locals.var_q0i_dn7 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn7));
        locals.var_x2_dn8 = ((locals.var_q0i_dn8 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn8));
        locals.var_x2_dn9 = ((locals.var_q0i_dn9 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn9));

        let assign3350_e3150: f64 = if locals.var_q0i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard51 = assign3350_e3150;

        let (assign3360_e3163, assign3360_e3163_d_n0, assign3360_e3163_d_n1, assign3360_e3163_d_n3, assign3360_e3163_d_n4, assign3360_e3163_d_n5, assign3360_e3163_d_n6, assign3360_e3163_d_n7, assign3360_e3163_d_n8, assign3360_e3163_d_n9,) = {
    if (locals.var_guard51 != 0.0) {
        let assign3360_e3154: f64 = (0.5 * locals.var_eps2);
        let assign3360_e3157: f64 = (locals.var_x2 + locals.var_eps2);
        let assign3360_e3158: f64 = (assign3360_e3157).sqrt();
        let assign3360_e3160: f64 = (assign3360_e3158 - locals.var_q0i);
        let assign3360_e3161: f64 = (assign3360_e3154 / assign3360_e3160);
        (assign3360_e3161, ((((0.5 * locals.var_eps2_dn0) * assign3360_e3160) - (assign3360_e3154 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign3360_e3158)) - locals.var_q0i_dn0))) / (assign3360_e3160 * assign3360_e3160)), ((((0.5 * locals.var_eps2_dn1) * assign3360_e3160) - (assign3360_e3154 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign3360_e3158)) - locals.var_q0i_dn1))) / (assign3360_e3160 * assign3360_e3160)), ((((0.5 * locals.var_eps2_dn3) * assign3360_e3160) - (assign3360_e3154 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign3360_e3158)) - locals.var_q0i_dn3))) / (assign3360_e3160 * assign3360_e3160)), ((((0.5 * locals.var_eps2_dn4) * assign3360_e3160) - (assign3360_e3154 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign3360_e3158)) - locals.var_q0i_dn4))) / (assign3360_e3160 * assign3360_e3160)), ((((0.5 * locals.var_eps2_dn5) * assign3360_e3160) - (assign3360_e3154 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign3360_e3158)) - locals.var_q0i_dn5))) / (assign3360_e3160 * assign3360_e3160)), ((((0.5 * locals.var_eps2_dn6) * assign3360_e3160) - (assign3360_e3154 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign3360_e3158)) - locals.var_q0i_dn6))) / (assign3360_e3160 * assign3360_e3160)), ((((0.5 * locals.var_eps2_dn7) * assign3360_e3160) - (assign3360_e3154 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign3360_e3158)) - locals.var_q0i_dn7))) / (assign3360_e3160 * assign3360_e3160)), ((((0.5 * locals.var_eps2_dn8) * assign3360_e3160) - (assign3360_e3154 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign3360_e3158)) - locals.var_q0i_dn8))) / (assign3360_e3160 * assign3360_e3160)), ((((0.5 * locals.var_eps2_dn9) * assign3360_e3160) - (assign3360_e3154 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign3360_e3158)) - locals.var_q0i_dn9))) / (assign3360_e3160 * assign3360_e3160)),)
    } else {
        (locals.var_q1i, locals.var_q1i_dn0, locals.var_q1i_dn1, locals.var_q1i_dn3, locals.var_q1i_dn4, locals.var_q1i_dn5, locals.var_q1i_dn6, locals.var_q1i_dn7, locals.var_q1i_dn8, locals.var_q1i_dn9,)
    }
};
        locals.var_q1i = assign3360_e3163;
        locals.var_q1i_dn0 = assign3360_e3163_d_n0;
        locals.var_q1i_dn1 = assign3360_e3163_d_n1;
        locals.var_q1i_dn3 = assign3360_e3163_d_n3;
        locals.var_q1i_dn4 = assign3360_e3163_d_n4;
        locals.var_q1i_dn5 = assign3360_e3163_d_n5;
        locals.var_q1i_dn6 = assign3360_e3163_d_n6;
        locals.var_q1i_dn7 = assign3360_e3163_d_n7;
        locals.var_q1i_dn8 = assign3360_e3163_d_n8;
        locals.var_q1i_dn9 = assign3360_e3163_d_n9;

        let (assign3370_e3175, assign3370_e3175_d_n0, assign3370_e3175_d_n1, assign3370_e3175_d_n3, assign3370_e3175_d_n4, assign3370_e3175_d_n5, assign3370_e3175_d_n6, assign3370_e3175_d_n7, assign3370_e3175_d_n8, assign3370_e3175_d_n9,) = {
    if (locals.var_guard51 == 0.0) {
        let assign3370_e3169: f64 = (locals.var_x2 + locals.var_eps2);
        let assign3370_e3170: f64 = (assign3370_e3169).sqrt();
        let assign3370_e3172: f64 = (assign3370_e3170 + locals.var_q0i);
        let assign3370_e3173: f64 = (0.5 * assign3370_e3172);
        (assign3370_e3173, (0.5 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign3370_e3170)) + locals.var_q0i_dn0)), (0.5 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign3370_e3170)) + locals.var_q0i_dn1)), (0.5 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign3370_e3170)) + locals.var_q0i_dn3)), (0.5 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign3370_e3170)) + locals.var_q0i_dn4)), (0.5 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign3370_e3170)) + locals.var_q0i_dn5)), (0.5 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign3370_e3170)) + locals.var_q0i_dn6)), (0.5 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign3370_e3170)) + locals.var_q0i_dn7)), (0.5 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign3370_e3170)) + locals.var_q0i_dn8)), (0.5 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign3370_e3170)) + locals.var_q0i_dn9)),)
    } else {
        (locals.var_q1i, locals.var_q1i_dn0, locals.var_q1i_dn1, locals.var_q1i_dn3, locals.var_q1i_dn4, locals.var_q1i_dn5, locals.var_q1i_dn6, locals.var_q1i_dn7, locals.var_q1i_dn8, locals.var_q1i_dn9,)
    }
};
        locals.var_q1i = assign3370_e3175;
        locals.var_q1i_dn0 = assign3370_e3175_d_n0;
        locals.var_q1i_dn1 = assign3370_e3175_d_n1;
        locals.var_q1i_dn3 = assign3370_e3175_d_n3;
        locals.var_q1i_dn4 = assign3370_e3175_d_n4;
        locals.var_q1i_dn5 = assign3370_e3175_d_n5;
        locals.var_q1i_dn6 = assign3370_e3175_d_n6;
        locals.var_q1i_dn7 = assign3370_e3175_d_n7;
        locals.var_q1i_dn8 = assign3370_e3175_d_n8;
        locals.var_q1i_dn9 = assign3370_e3175_d_n9;

        let assign3380_e3181: f64 = (locals.var_n0 + locals.var_nb);
        let assign3380_e3182: f64 = (0.5 * assign3380_e3181);
        let assign3380_e3183: f64 = (1.0 + assign3380_e3182);
        let assign3380_e3184: f64 = (locals.var_q1i * assign3380_e3183);
        locals.var_qbi = assign3380_e3184;
        locals.var_qbi_dn0 = ((locals.var_q1i_dn0 * assign3380_e3183) + (locals.var_q1i * (0.5 * (locals.var_n0_dn0 + locals.var_nb_dn0))));
        locals.var_qbi_dn1 = ((locals.var_q1i_dn1 * assign3380_e3183) + (locals.var_q1i * (0.5 * (locals.var_n0_dn1 + locals.var_nb_dn1))));
        locals.var_qbi_dn3 = ((locals.var_q1i_dn3 * assign3380_e3183) + (locals.var_q1i * (0.5 * (locals.var_n0_dn3 + locals.var_nb_dn3))));
        locals.var_qbi_dn4 = ((locals.var_q1i_dn4 * assign3380_e3183) + (locals.var_q1i * (0.5 * (locals.var_n0_dn4 + locals.var_nb_dn4))));
        locals.var_qbi_dn5 = ((locals.var_q1i_dn5 * assign3380_e3183) + (locals.var_q1i * (0.5 * (locals.var_n0_dn5 + locals.var_nb_dn5))));
        locals.var_qbi_dn6 = ((locals.var_q1i_dn6 * assign3380_e3183) + (locals.var_q1i * (0.5 * (locals.var_n0_dn6 + locals.var_nb_dn6))));
        locals.var_qbi_dn7 = ((locals.var_q1i_dn7 * assign3380_e3183) + (locals.var_q1i * (0.5 * (locals.var_n0_dn7 + locals.var_nb_dn7))));
        locals.var_qbi_dn8 = ((locals.var_q1i_dn8 * assign3380_e3183) + (locals.var_q1i * (0.5 * (locals.var_n0_dn8 + locals.var_nb_dn8))));
        locals.var_qbi_dn9 = ((locals.var_q1i_dn9 * assign3380_e3183) + (locals.var_q1i * (0.5 * (locals.var_n0_dn9 + locals.var_nb_dn9))));

        let assign3390_e3187: f64 = (p.p14 * locals.var_is_t);
        let assign3390_e3189: f64 = (assign3390_e3187 * locals.var_evb2c2star_nfr);
        locals.var_ir = assign3390_e3189;
        locals.var_ir_dn0 = (((p.p14 * locals.var_is_t_dn0) * locals.var_evb2c2star_nfr) + (assign3390_e3187 * locals.var_evb2c2star_nfr_dn0));
        locals.var_ir_dn1 = (((p.p14 * locals.var_is_t_dn1) * locals.var_evb2c2star_nfr) + (assign3390_e3187 * locals.var_evb2c2star_nfr_dn1));
        locals.var_ir_dn3 = (((p.p14 * locals.var_is_t_dn3) * locals.var_evb2c2star_nfr) + (assign3390_e3187 * locals.var_evb2c2star_nfr_dn3));
        locals.var_ir_dn4 = (((p.p14 * locals.var_is_t_dn4) * locals.var_evb2c2star_nfr) + (assign3390_e3187 * locals.var_evb2c2star_nfr_dn4));
        locals.var_ir_dn5 = (((p.p14 * locals.var_is_t_dn5) * locals.var_evb2c2star_nfr) + (assign3390_e3187 * locals.var_evb2c2star_nfr_dn5));
        locals.var_ir_dn6 = (((p.p14 * locals.var_is_t_dn6) * locals.var_evb2c2star_nfr) + (assign3390_e3187 * locals.var_evb2c2star_nfr_dn6));
        locals.var_ir_dn7 = (((p.p14 * locals.var_is_t_dn7) * locals.var_evb2c2star_nfr) + (assign3390_e3187 * locals.var_evb2c2star_nfr_dn7));
        locals.var_ir_dn8 = (((p.p14 * locals.var_is_t_dn8) * locals.var_evb2c2star_nfr) + (assign3390_e3187 * locals.var_evb2c2star_nfr_dn8));
        locals.var_ir_dn9 = (((p.p14 * locals.var_is_t_dn9) * locals.var_evb2c2star_nfr) + (assign3390_e3187 * locals.var_evb2c2star_nfr_dn9));

        let assign3400_e3192: f64 = (locals.var_is_t * locals.var_evb2e1);
        locals.var_if_ = assign3400_e3192;
        locals.var_if__dn0 = ((locals.var_is_t_dn0 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn0));
        locals.var_if__dn1 = ((locals.var_is_t_dn1 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn1));
        locals.var_if__dn3 = ((locals.var_is_t_dn3 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn3));
        locals.var_if__dn4 = ((locals.var_is_t_dn4 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn4));
        locals.var_if__dn5 = ((locals.var_is_t_dn5 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn5));
        locals.var_if__dn6 = ((locals.var_is_t_dn6 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn6));
        locals.var_if__dn7 = ((locals.var_is_t_dn7 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn7));
        locals.var_if__dn8 = ((locals.var_is_t_dn8 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn8));
        locals.var_if__dn9 = ((locals.var_is_t_dn9 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn9));

        let assign3410_e3195: f64 = (locals.var_if_ - locals.var_ir);
        let assign3410_e3197: f64 = (assign3410_e3195 / locals.var_qbi);
        locals.var_in_ = assign3410_e3197;
        locals.var_in__dn0 = ((((locals.var_if__dn0 - locals.var_ir_dn0) * locals.var_qbi) - (assign3410_e3195 * locals.var_qbi_dn0)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn1 = ((((locals.var_if__dn1 - locals.var_ir_dn1) * locals.var_qbi) - (assign3410_e3195 * locals.var_qbi_dn1)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn3 = ((((locals.var_if__dn3 - locals.var_ir_dn3) * locals.var_qbi) - (assign3410_e3195 * locals.var_qbi_dn3)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn4 = ((((locals.var_if__dn4 - locals.var_ir_dn4) * locals.var_qbi) - (assign3410_e3195 * locals.var_qbi_dn4)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn5 = ((((locals.var_if__dn5 - locals.var_ir_dn5) * locals.var_qbi) - (assign3410_e3195 * locals.var_qbi_dn5)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn6 = ((((locals.var_if__dn6 - locals.var_ir_dn6) * locals.var_qbi) - (assign3410_e3195 * locals.var_qbi_dn6)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn7 = ((((locals.var_if__dn7 - locals.var_ir_dn7) * locals.var_qbi) - (assign3410_e3195 * locals.var_qbi_dn7)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn8 = ((((locals.var_if__dn8 - locals.var_ir_dn8) * locals.var_qbi) - (assign3410_e3195 * locals.var_qbi_dn8)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn9 = ((((locals.var_if__dn9 - locals.var_ir_dn9) * locals.var_qbi) - (assign3410_e3195 * locals.var_qbi_dn9)) / (locals.var_qbi * locals.var_qbi));

        let assign3420_e3200: f64 = locals.var_vb2e1;
        let assign3420_e3202: f64 = (assign3420_e3200 / 0.0001);
        locals.var_dxa = assign3420_e3202;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = (locals.var_vb2e1_dn3 / 0.0001);
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = (locals.var_vb2e1_dn5 / 0.0001);
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;

        let assign3430_e3205: f64 = if locals.var_vb2e1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard52 = assign3430_e3205;

        let (assign3440_e3217, assign3440_e3217_d_n0, assign3440_e3217_d_n1, assign3440_e3217_d_n3, assign3440_e3217_d_n4, assign3440_e3217_d_n5, assign3440_e3217_d_n6, assign3440_e3217_d_n7, assign3440_e3217_d_n8, assign3440_e3217_d_n9,) = {
    if (locals.var_guard52 != 0.0) {
        let assign3440_e3211: f64 = (locals.var_dxa).exp();
        let assign3440_e3212: f64 = (1.0 + assign3440_e3211);
        let assign3440_e3213: f64 = (assign3440_e3212).ln();
        let assign3440_e3214: f64 = (0.0001 * assign3440_e3213);
        let assign3440_e3215: f64 = assign3440_e3214;
        (assign3440_e3215, (0.0001 * ((assign3440_e3211 * locals.var_dxa_dn0) / assign3440_e3212)), (0.0001 * ((assign3440_e3211 * locals.var_dxa_dn1) / assign3440_e3212)), (0.0001 * ((assign3440_e3211 * locals.var_dxa_dn3) / assign3440_e3212)), (0.0001 * ((assign3440_e3211 * locals.var_dxa_dn4) / assign3440_e3212)), (0.0001 * ((assign3440_e3211 * locals.var_dxa_dn5) / assign3440_e3212)), (0.0001 * ((assign3440_e3211 * locals.var_dxa_dn6) / assign3440_e3212)), (0.0001 * ((assign3440_e3211 * locals.var_dxa_dn7) / assign3440_e3212)), (0.0001 * ((assign3440_e3211 * locals.var_dxa_dn8) / assign3440_e3212)), (0.0001 * ((assign3440_e3211 * locals.var_dxa_dn9) / assign3440_e3212)),)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign3440_e3217;
        locals.var_tmpexp_dn0 = assign3440_e3217_d_n0;
        locals.var_tmpexp_dn1 = assign3440_e3217_d_n1;
        locals.var_tmpexp_dn3 = assign3440_e3217_d_n3;
        locals.var_tmpexp_dn4 = assign3440_e3217_d_n4;
        locals.var_tmpexp_dn5 = assign3440_e3217_d_n5;
        locals.var_tmpexp_dn6 = assign3440_e3217_d_n6;
        locals.var_tmpexp_dn7 = assign3440_e3217_d_n7;
        locals.var_tmpexp_dn8 = assign3440_e3217_d_n8;
        locals.var_tmpexp_dn9 = assign3440_e3217_d_n9;

        let (assign3450_e3231, assign3450_e3231_d_n0, assign3450_e3231_d_n1, assign3450_e3231_d_n3, assign3450_e3231_d_n4, assign3450_e3231_d_n5, assign3450_e3231_d_n6, assign3450_e3231_d_n7, assign3450_e3231_d_n8, assign3450_e3231_d_n9,) = {
    if (locals.var_guard52 == 0.0) {
        let assign3450_e3224: f64 = (-locals.var_dxa);
        let assign3450_e3225: f64 = (assign3450_e3224).exp();
        let assign3450_e3226: f64 = (1.0 + assign3450_e3225);
        let assign3450_e3227: f64 = (assign3450_e3226).ln();
        let assign3450_e3228: f64 = (0.0001 * assign3450_e3227);
        let assign3450_e3229: f64 = (locals.var_vb2e1 + assign3450_e3228);
        (assign3450_e3229, (0.0001 * ((assign3450_e3225 * (-locals.var_dxa_dn0)) / assign3450_e3226)), (0.0001 * ((assign3450_e3225 * (-locals.var_dxa_dn1)) / assign3450_e3226)), (locals.var_vb2e1_dn3 + (0.0001 * ((assign3450_e3225 * (-locals.var_dxa_dn3)) / assign3450_e3226))), (0.0001 * ((assign3450_e3225 * (-locals.var_dxa_dn4)) / assign3450_e3226)), (locals.var_vb2e1_dn5 + (0.0001 * ((assign3450_e3225 * (-locals.var_dxa_dn5)) / assign3450_e3226))), (0.0001 * ((assign3450_e3225 * (-locals.var_dxa_dn6)) / assign3450_e3226)), (0.0001 * ((assign3450_e3225 * (-locals.var_dxa_dn7)) / assign3450_e3226)), (0.0001 * ((assign3450_e3225 * (-locals.var_dxa_dn8)) / assign3450_e3226)), (0.0001 * ((assign3450_e3225 * (-locals.var_dxa_dn9)) / assign3450_e3226)),)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign3450_e3231;
        locals.var_tmpexp_dn0 = assign3450_e3231_d_n0;
        locals.var_tmpexp_dn1 = assign3450_e3231_d_n1;
        locals.var_tmpexp_dn3 = assign3450_e3231_d_n3;
        locals.var_tmpexp_dn4 = assign3450_e3231_d_n4;
        locals.var_tmpexp_dn5 = assign3450_e3231_d_n5;
        locals.var_tmpexp_dn6 = assign3450_e3231_d_n6;
        locals.var_tmpexp_dn7 = assign3450_e3231_d_n7;
        locals.var_tmpexp_dn8 = assign3450_e3231_d_n8;
        locals.var_tmpexp_dn9 = assign3450_e3231_d_n9;

        let assign3460_e3234: f64 = (locals.var_tmpexp / p.p139);
        locals.var_tmpexp1 = assign3460_e3234;

        let assign3470_e3237: f64 = if locals.var_tmpexp1 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard53 = assign3470_e3237;

        let (assign3490_e3248,) = {
    if (locals.var_guard53 == 0.0) {
        let assign3490_e3246: f64 = (p.p134).exp();
        (assign3490_e3246,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3490_e3248;

        let assign3520_e3267: f64 = (locals.var_vb2e1 - p.p141);
        let assign3520_e3269: f64 = (assign3520_e3267 / 0.001);
        locals.var_dxa = assign3520_e3269;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = (locals.var_vb2e1_dn3 / 0.001);
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = (locals.var_vb2e1_dn5 / 0.001);
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;

        let assign3570_e3310: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3570_e3312: f64 = (assign3570_e3310 / p.p16);
        let assign3570_e3314: f64 = if assign3570_e3312 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard55 = assign3570_e3314;

        let (assign3580_e3323, assign3580_e3323_d_n0, assign3580_e3323_d_n1, assign3580_e3323_d_n3, assign3580_e3323_d_n4, assign3580_e3323_d_n5, assign3580_e3323_d_n6, assign3580_e3323_d_n7, assign3580_e3323_d_n8, assign3580_e3323_d_n9,) = {
    if (locals.var_guard55 != 0.0) {
        let assign3580_e3318: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3580_e3320: f64 = (assign3580_e3318 / p.p16);
        let assign3580_e3321: f64 = (assign3580_e3320).exp();
        (assign3580_e3321, 0.0, 0.0, (assign3580_e3321 * ((locals.var_vb2e1_dn3 * locals.var_vtinv) / p.p16)), 0.0, (assign3580_e3321 * ((locals.var_vb2e1_dn5 * locals.var_vtinv) / p.p16)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign3580_e3323;
        locals.var_tmpexp_dn0 = assign3580_e3323_d_n0;
        locals.var_tmpexp_dn1 = assign3580_e3323_d_n1;
        locals.var_tmpexp_dn3 = assign3580_e3323_d_n3;
        locals.var_tmpexp_dn4 = assign3580_e3323_d_n4;
        locals.var_tmpexp_dn5 = assign3580_e3323_d_n5;
        locals.var_tmpexp_dn6 = assign3580_e3323_d_n6;
        locals.var_tmpexp_dn7 = assign3580_e3323_d_n7;
        locals.var_tmpexp_dn8 = assign3580_e3323_d_n8;
        locals.var_tmpexp_dn9 = assign3580_e3323_d_n9;

        let (assign3590_e3329,) = {
    if (locals.var_guard55 == 0.0) {
        let assign3590_e3327: f64 = (p.p134).exp();
        (assign3590_e3327,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3590_e3329;

        let (assign3600_e3344, assign3600_e3344_d_n0, assign3600_e3344_d_n1, assign3600_e3344_d_n3, assign3600_e3344_d_n4, assign3600_e3344_d_n5, assign3600_e3344_d_n6, assign3600_e3344_d_n7, assign3600_e3344_d_n8, assign3600_e3344_d_n9,) = {
    if (locals.var_guard55 == 0.0) {
        let assign3600_e3336: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3600_e3338: f64 = (assign3600_e3336 / p.p16);
        let assign3600_e3340: f64 = (assign3600_e3338 - p.p134);
        let assign3600_e3341: f64 = (1.0 + assign3600_e3340);
        let assign3600_e3342: f64 = (locals.var_expl * assign3600_e3341);
        (assign3600_e3342, 0.0, 0.0, (locals.var_expl * ((locals.var_vb2e1_dn3 * locals.var_vtinv) / p.p16)), 0.0, (locals.var_expl * ((locals.var_vb2e1_dn5 * locals.var_vtinv) / p.p16)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign3600_e3344;
        locals.var_tmpexp_dn0 = assign3600_e3344_d_n0;
        locals.var_tmpexp_dn1 = assign3600_e3344_d_n1;
        locals.var_tmpexp_dn3 = assign3600_e3344_d_n3;
        locals.var_tmpexp_dn4 = assign3600_e3344_d_n4;
        locals.var_tmpexp_dn5 = assign3600_e3344_d_n5;
        locals.var_tmpexp_dn6 = assign3600_e3344_d_n6;
        locals.var_tmpexp_dn7 = assign3600_e3344_d_n7;
        locals.var_tmpexp_dn8 = assign3600_e3344_d_n8;
        locals.var_tmpexp_dn9 = assign3600_e3344_d_n9;

        let assign3610_e3347: f64 = if p.p23 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard56 = assign3610_e3347;

        let assign3620_e3350: f64 = (locals.var_vb2e1 - locals.var_vknbr_t);
        let assign3620_e3352: f64 = (assign3620_e3350 * locals.var_vtinv);
        let assign3620_e3354: f64 = if assign3620_e3352 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard57 = assign3620_e3354;

        let (assign3630_e3365,) = {
    if ((locals.var_guard56 != 0.0) && (locals.var_guard57 != 0.0)) {
        let assign3630_e3360: f64 = (locals.var_vb2e1 - locals.var_vknbr_t);
        let assign3630_e3362: f64 = (assign3630_e3360 * locals.var_vtinv);
        let assign3630_e3363: f64 = (assign3630_e3362).exp();
        (assign3630_e3363,)
    } else {
        (locals.var_tmpexp1,)
    }
};
        locals.var_tmpexp1 = assign3630_e3365;

        let (assign3640_e3373,) = {
    if ((locals.var_guard56 != 0.0) && (locals.var_guard57 == 0.0)) {
        let assign3640_e3371: f64 = (p.p134).exp();
        (assign3640_e3371,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3640_e3373;

        let (assign3650_e3390,) = {
    if ((locals.var_guard56 != 0.0) && (locals.var_guard57 == 0.0)) {
        let assign3650_e3382: f64 = (locals.var_vb2e1 - locals.var_vknbr_t);
        let assign3650_e3384: f64 = (assign3650_e3382 * locals.var_vtinv);
        let assign3650_e3386: f64 = (assign3650_e3384 - p.p134);
        let assign3650_e3387: f64 = (1.0 + assign3650_e3386);
        let assign3650_e3388: f64 = (locals.var_expl * assign3650_e3387);
        (assign3650_e3388,)
    } else {
        (locals.var_tmpexp1,)
    }
};
        locals.var_tmpexp1 = assign3650_e3390;

        let assign3660_e3393: f64 = (locals.var_in_ / locals.var_is_t);
        let assign3660_e3395: f64 = (assign3660_e3393 - 1000.0);
        let assign3660_e3397: f64 = if assign3660_e3395 < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard58 = assign3660_e3397;

        let (assign3680_e3416,) = {
    if ((locals.var_guard56 != 0.0) && (locals.var_guard58 == 0.0)) {
        let assign3680_e3414: f64 = (40.0_f64).exp();
        (assign3680_e3414,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3680_e3416;

        let assign3740_e3523: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign3740_e3525: f64 = (assign3740_e3523 / p.p18);
        let assign3740_e3527: f64 = if assign3740_e3525 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard60 = assign3740_e3527;

        let (assign3750_e3536, assign3750_e3536_d_n0, assign3750_e3536_d_n1, assign3750_e3536_d_n3, assign3750_e3536_d_n4, assign3750_e3536_d_n5, assign3750_e3536_d_n6, assign3750_e3536_d_n7, assign3750_e3536_d_n8, assign3750_e3536_d_n9,) = {
    if (locals.var_guard60 != 0.0) {
        let assign3750_e3531: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign3750_e3533: f64 = (assign3750_e3531 / p.p18);
        let assign3750_e3534: f64 = (assign3750_e3533).exp();
        (assign3750_e3534, 0.0, 0.0, (assign3750_e3534 * ((locals.var_vb1e1_dn3 * locals.var_vtinv) / p.p18)), (assign3750_e3534 * ((locals.var_vb1e1_dn4 * locals.var_vtinv) / p.p18)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign3750_e3536;
        locals.var_tmpexp_dn0 = assign3750_e3536_d_n0;
        locals.var_tmpexp_dn1 = assign3750_e3536_d_n1;
        locals.var_tmpexp_dn3 = assign3750_e3536_d_n3;
        locals.var_tmpexp_dn4 = assign3750_e3536_d_n4;
        locals.var_tmpexp_dn5 = assign3750_e3536_d_n5;
        locals.var_tmpexp_dn6 = assign3750_e3536_d_n6;
        locals.var_tmpexp_dn7 = assign3750_e3536_d_n7;
        locals.var_tmpexp_dn8 = assign3750_e3536_d_n8;
        locals.var_tmpexp_dn9 = assign3750_e3536_d_n9;

        let (assign3760_e3542,) = {
    if (locals.var_guard60 == 0.0) {
        let assign3760_e3540: f64 = (p.p134).exp();
        (assign3760_e3540,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3760_e3542;

        let (assign3770_e3557, assign3770_e3557_d_n0, assign3770_e3557_d_n1, assign3770_e3557_d_n3, assign3770_e3557_d_n4, assign3770_e3557_d_n5, assign3770_e3557_d_n6, assign3770_e3557_d_n7, assign3770_e3557_d_n8, assign3770_e3557_d_n9,) = {
    if (locals.var_guard60 == 0.0) {
        let assign3770_e3549: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign3770_e3551: f64 = (assign3770_e3549 / p.p18);
        let assign3770_e3553: f64 = (assign3770_e3551 - p.p134);
        let assign3770_e3554: f64 = (1.0 + assign3770_e3553);
        let assign3770_e3555: f64 = (locals.var_expl * assign3770_e3554);
        (assign3770_e3555, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1e1_dn3 * locals.var_vtinv) / p.p18)), (locals.var_expl * ((locals.var_vb1e1_dn4 * locals.var_vtinv) / p.p18)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign3770_e3557;
        locals.var_tmpexp_dn0 = assign3770_e3557_d_n0;
        locals.var_tmpexp_dn1 = assign3770_e3557_d_n1;
        locals.var_tmpexp_dn3 = assign3770_e3557_d_n3;
        locals.var_tmpexp_dn4 = assign3770_e3557_d_n4;
        locals.var_tmpexp_dn5 = assign3770_e3557_d_n5;
        locals.var_tmpexp_dn6 = assign3770_e3557_d_n6;
        locals.var_tmpexp_dn7 = assign3770_e3557_d_n7;
        locals.var_tmpexp_dn8 = assign3770_e3557_d_n8;
        locals.var_tmpexp_dn9 = assign3770_e3557_d_n9;

        let assign3780_e3560: f64 = if p.p23 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard61 = assign3780_e3560;

        let assign3790_e3563: f64 = (locals.var_vb1e1 - locals.var_vknbr_t);
        let assign3790_e3565: f64 = (assign3790_e3563 * locals.var_vtinv);
        let assign3790_e3567: f64 = if assign3790_e3565 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard62 = assign3790_e3567;

    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3800_e3578,) = {
    if ((locals.var_guard61 != 0.0) && (locals.var_guard62 != 0.0)) {
        let assign3800_e3573: f64 = (locals.var_vb1e1 - locals.var_vknbr_t);
        let assign3800_e3575: f64 = (assign3800_e3573 * locals.var_vtinv);
        let assign3800_e3576: f64 = (assign3800_e3575).exp();
        (assign3800_e3576,)
    } else {
        (locals.var_tmpexp1,)
    }
};
        locals.var_tmpexp1 = assign3800_e3578;

        let (assign3810_e3586,) = {
    if ((locals.var_guard61 != 0.0) && (locals.var_guard62 == 0.0)) {
        let assign3810_e3584: f64 = (p.p134).exp();
        (assign3810_e3584,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3810_e3586;

        let (assign3820_e3603,) = {
    if ((locals.var_guard61 != 0.0) && (locals.var_guard62 == 0.0)) {
        let assign3820_e3595: f64 = (locals.var_vb1e1 - locals.var_vknbr_t);
        let assign3820_e3597: f64 = (assign3820_e3595 * locals.var_vtinv);
        let assign3820_e3599: f64 = (assign3820_e3597 - p.p134);
        let assign3820_e3600: f64 = (1.0 + assign3820_e3599);
        let assign3820_e3601: f64 = (locals.var_expl * assign3820_e3600);
        (assign3820_e3601,)
    } else {
        (locals.var_tmpexp1,)
    }
};
        locals.var_tmpexp1 = assign3820_e3603;

        let assign3850_e3640: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3850_e3642: f64 = (assign3850_e3640 / p.p20);
        let assign3850_e3644: f64 = if assign3850_e3642 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard63 = assign3850_e3644;

        let (assign3860_e3653, assign3860_e3653_d_n0, assign3860_e3653_d_n1, assign3860_e3653_d_n3, assign3860_e3653_d_n4, assign3860_e3653_d_n5, assign3860_e3653_d_n6, assign3860_e3653_d_n7, assign3860_e3653_d_n8, assign3860_e3653_d_n9,) = {
    if (locals.var_guard63 != 0.0) {
        let assign3860_e3648: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3860_e3650: f64 = (assign3860_e3648 / p.p20);
        let assign3860_e3651: f64 = (assign3860_e3650).exp();
        (assign3860_e3651, 0.0, 0.0, (assign3860_e3651 * ((locals.var_vb2e1_dn3 * locals.var_vtinv) / p.p20)), 0.0, (assign3860_e3651 * ((locals.var_vb2e1_dn5 * locals.var_vtinv) / p.p20)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign3860_e3653;
        locals.var_tmpexp_dn0 = assign3860_e3653_d_n0;
        locals.var_tmpexp_dn1 = assign3860_e3653_d_n1;
        locals.var_tmpexp_dn3 = assign3860_e3653_d_n3;
        locals.var_tmpexp_dn4 = assign3860_e3653_d_n4;
        locals.var_tmpexp_dn5 = assign3860_e3653_d_n5;
        locals.var_tmpexp_dn6 = assign3860_e3653_d_n6;
        locals.var_tmpexp_dn7 = assign3860_e3653_d_n7;
        locals.var_tmpexp_dn8 = assign3860_e3653_d_n8;
        locals.var_tmpexp_dn9 = assign3860_e3653_d_n9;

        let (assign3870_e3659,) = {
    if (locals.var_guard63 == 0.0) {
        let assign3870_e3657: f64 = (p.p134).exp();
        (assign3870_e3657,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3870_e3659;

        let (assign3880_e3674, assign3880_e3674_d_n0, assign3880_e3674_d_n1, assign3880_e3674_d_n3, assign3880_e3674_d_n4, assign3880_e3674_d_n5, assign3880_e3674_d_n6, assign3880_e3674_d_n7, assign3880_e3674_d_n8, assign3880_e3674_d_n9,) = {
    if (locals.var_guard63 == 0.0) {
        let assign3880_e3666: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3880_e3668: f64 = (assign3880_e3666 / p.p20);
        let assign3880_e3670: f64 = (assign3880_e3668 - p.p134);
        let assign3880_e3671: f64 = (1.0 + assign3880_e3670);
        let assign3880_e3672: f64 = (locals.var_expl * assign3880_e3671);
        (assign3880_e3672, 0.0, 0.0, (locals.var_expl * ((locals.var_vb2e1_dn3 * locals.var_vtinv) / p.p20)), 0.0, (locals.var_expl * ((locals.var_vb2e1_dn5 * locals.var_vtinv) / p.p20)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign3880_e3674;
        locals.var_tmpexp_dn0 = assign3880_e3674_d_n0;
        locals.var_tmpexp_dn1 = assign3880_e3674_d_n1;
        locals.var_tmpexp_dn3 = assign3880_e3674_d_n3;
        locals.var_tmpexp_dn4 = assign3880_e3674_d_n4;
        locals.var_tmpexp_dn5 = assign3880_e3674_d_n5;
        locals.var_tmpexp_dn6 = assign3880_e3674_d_n6;
        locals.var_tmpexp_dn7 = assign3880_e3674_d_n7;
        locals.var_tmpexp_dn8 = assign3880_e3674_d_n8;
        locals.var_tmpexp_dn9 = assign3880_e3674_d_n9;

        let assign3900_e3682: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign3900_e3684: f64 = (assign3900_e3682 / p.p22);
        let assign3900_e3686: f64 = if assign3900_e3684 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard64 = assign3900_e3686;

        let (assign3910_e3695, assign3910_e3695_d_n0, assign3910_e3695_d_n1, assign3910_e3695_d_n3, assign3910_e3695_d_n4, assign3910_e3695_d_n5, assign3910_e3695_d_n6, assign3910_e3695_d_n7, assign3910_e3695_d_n8, assign3910_e3695_d_n9,) = {
    if (locals.var_guard64 != 0.0) {
        let assign3910_e3690: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign3910_e3692: f64 = (assign3910_e3690 / p.p22);
        let assign3910_e3693: f64 = (assign3910_e3692).exp();
        (assign3910_e3693, 0.0, 0.0, (assign3910_e3693 * ((locals.var_vb1e1_dn3 * locals.var_vtinv) / p.p22)), (assign3910_e3693 * ((locals.var_vb1e1_dn4 * locals.var_vtinv) / p.p22)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign3910_e3695;
        locals.var_tmpexp_dn0 = assign3910_e3695_d_n0;
        locals.var_tmpexp_dn1 = assign3910_e3695_d_n1;
        locals.var_tmpexp_dn3 = assign3910_e3695_d_n3;
        locals.var_tmpexp_dn4 = assign3910_e3695_d_n4;
        locals.var_tmpexp_dn5 = assign3910_e3695_d_n5;
        locals.var_tmpexp_dn6 = assign3910_e3695_d_n6;
        locals.var_tmpexp_dn7 = assign3910_e3695_d_n7;
        locals.var_tmpexp_dn8 = assign3910_e3695_d_n8;
        locals.var_tmpexp_dn9 = assign3910_e3695_d_n9;

        let (assign3920_e3701,) = {
    if (locals.var_guard64 == 0.0) {
        let assign3920_e3699: f64 = (p.p134).exp();
        (assign3920_e3699,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3920_e3701;

        let (assign3930_e3716, assign3930_e3716_d_n0, assign3930_e3716_d_n1, assign3930_e3716_d_n3, assign3930_e3716_d_n4, assign3930_e3716_d_n5, assign3930_e3716_d_n6, assign3930_e3716_d_n7, assign3930_e3716_d_n8, assign3930_e3716_d_n9,) = {
    if (locals.var_guard64 == 0.0) {
        let assign3930_e3708: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign3930_e3710: f64 = (assign3930_e3708 / p.p22);
        let assign3930_e3712: f64 = (assign3930_e3710 - p.p134);
        let assign3930_e3713: f64 = (1.0 + assign3930_e3712);
        let assign3930_e3714: f64 = (locals.var_expl * assign3930_e3713);
        (assign3930_e3714, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1e1_dn3 * locals.var_vtinv) / p.p22)), (locals.var_expl * ((locals.var_vb1e1_dn4 * locals.var_vtinv) / p.p22)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign3930_e3716;
        locals.var_tmpexp_dn0 = assign3930_e3716_d_n0;
        locals.var_tmpexp_dn1 = assign3930_e3716_d_n1;
        locals.var_tmpexp_dn3 = assign3930_e3716_d_n3;
        locals.var_tmpexp_dn4 = assign3930_e3716_d_n4;
        locals.var_tmpexp_dn5 = assign3930_e3716_d_n5;
        locals.var_tmpexp_dn6 = assign3930_e3716_d_n6;
        locals.var_tmpexp_dn7 = assign3930_e3716_d_n7;
        locals.var_tmpexp_dn8 = assign3930_e3716_d_n8;
        locals.var_tmpexp_dn9 = assign3930_e3716_d_n9;

        let assign3950_e3724: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign3950_e3726: f64 = (assign3950_e3724 / p.p31);
        let assign3950_e3728: f64 = if assign3950_e3726 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard65 = assign3950_e3728;

        let (assign3960_e3737, assign3960_e3737_d_n0, assign3960_e3737_d_n1, assign3960_e3737_d_n3, assign3960_e3737_d_n4, assign3960_e3737_d_n5, assign3960_e3737_d_n6, assign3960_e3737_d_n7, assign3960_e3737_d_n8, assign3960_e3737_d_n9,) = {
    if (locals.var_guard65 != 0.0) {
        let assign3960_e3732: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign3960_e3734: f64 = (assign3960_e3732 / p.p31);
        let assign3960_e3735: f64 = (assign3960_e3734).exp();
        (assign3960_e3735, 0.0, 0.0, 0.0, (assign3960_e3735 * ((locals.var_vb1c4_dn4 * locals.var_vtinv) / p.p31)), (assign3960_e3735 * ((locals.var_vb1c4_dn5 * locals.var_vtinv) / p.p31)), (assign3960_e3735 * ((locals.var_vb1c4_dn6 * locals.var_vtinv) / p.p31)), (assign3960_e3735 * ((locals.var_vb1c4_dn7 * locals.var_vtinv) / p.p31)), 0.0, (assign3960_e3735 * ((locals.var_vb1c4_dn9 * locals.var_vtinv) / p.p31)),)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign3960_e3737;
        locals.var_tmpexp_dn0 = assign3960_e3737_d_n0;
        locals.var_tmpexp_dn1 = assign3960_e3737_d_n1;
        locals.var_tmpexp_dn3 = assign3960_e3737_d_n3;
        locals.var_tmpexp_dn4 = assign3960_e3737_d_n4;
        locals.var_tmpexp_dn5 = assign3960_e3737_d_n5;
        locals.var_tmpexp_dn6 = assign3960_e3737_d_n6;
        locals.var_tmpexp_dn7 = assign3960_e3737_d_n7;
        locals.var_tmpexp_dn8 = assign3960_e3737_d_n8;
        locals.var_tmpexp_dn9 = assign3960_e3737_d_n9;

        let (assign3970_e3743,) = {
    if (locals.var_guard65 == 0.0) {
        let assign3970_e3741: f64 = (p.p134).exp();
        (assign3970_e3741,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3970_e3743;

        let (assign3980_e3758, assign3980_e3758_d_n0, assign3980_e3758_d_n1, assign3980_e3758_d_n3, assign3980_e3758_d_n4, assign3980_e3758_d_n5, assign3980_e3758_d_n6, assign3980_e3758_d_n7, assign3980_e3758_d_n8, assign3980_e3758_d_n9,) = {
    if (locals.var_guard65 == 0.0) {
        let assign3980_e3750: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign3980_e3752: f64 = (assign3980_e3750 / p.p31);
        let assign3980_e3754: f64 = (assign3980_e3752 - p.p134);
        let assign3980_e3755: f64 = (1.0 + assign3980_e3754);
        let assign3980_e3756: f64 = (locals.var_expl * assign3980_e3755);
        (assign3980_e3756, 0.0, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1c4_dn4 * locals.var_vtinv) / p.p31)), (locals.var_expl * ((locals.var_vb1c4_dn5 * locals.var_vtinv) / p.p31)), (locals.var_expl * ((locals.var_vb1c4_dn6 * locals.var_vtinv) / p.p31)), (locals.var_expl * ((locals.var_vb1c4_dn7 * locals.var_vtinv) / p.p31)), 0.0, (locals.var_expl * ((locals.var_vb1c4_dn9 * locals.var_vtinv) / p.p31)),)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign3980_e3758;
        locals.var_tmpexp_dn0 = assign3980_e3758_d_n0;
        locals.var_tmpexp_dn1 = assign3980_e3758_d_n1;
        locals.var_tmpexp_dn3 = assign3980_e3758_d_n3;
        locals.var_tmpexp_dn4 = assign3980_e3758_d_n4;
        locals.var_tmpexp_dn5 = assign3980_e3758_d_n5;
        locals.var_tmpexp_dn6 = assign3980_e3758_d_n6;
        locals.var_tmpexp_dn7 = assign3980_e3758_d_n7;
        locals.var_tmpexp_dn8 = assign3980_e3758_d_n8;
        locals.var_tmpexp_dn9 = assign3980_e3758_d_n9;

        let assign4000_e3766: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4000_e3768: f64 = (assign4000_e3766 / p.p133);
        let assign4000_e3770: f64 = if assign4000_e3768 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard66 = assign4000_e3770;

        let (assign4010_e3779, assign4010_e3779_d_n0, assign4010_e3779_d_n1, assign4010_e3779_d_n3, assign4010_e3779_d_n4, assign4010_e3779_d_n5, assign4010_e3779_d_n6, assign4010_e3779_d_n7, assign4010_e3779_d_n8, assign4010_e3779_d_n9,) = {
    if (locals.var_guard66 != 0.0) {
        let assign4010_e3774: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4010_e3776: f64 = (assign4010_e3774 / p.p133);
        let assign4010_e3777: f64 = (assign4010_e3776).exp();
        (assign4010_e3777, 0.0, 0.0, (assign4010_e3777 * ((locals.var_vb1e1_dn3 * locals.var_vtinv) / p.p133)), (assign4010_e3777 * ((locals.var_vb1e1_dn4 * locals.var_vtinv) / p.p133)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign4010_e3779;
        locals.var_tmpexp_dn0 = assign4010_e3779_d_n0;
        locals.var_tmpexp_dn1 = assign4010_e3779_d_n1;
        locals.var_tmpexp_dn3 = assign4010_e3779_d_n3;
        locals.var_tmpexp_dn4 = assign4010_e3779_d_n4;
        locals.var_tmpexp_dn5 = assign4010_e3779_d_n5;
        locals.var_tmpexp_dn6 = assign4010_e3779_d_n6;
        locals.var_tmpexp_dn7 = assign4010_e3779_d_n7;
        locals.var_tmpexp_dn8 = assign4010_e3779_d_n8;
        locals.var_tmpexp_dn9 = assign4010_e3779_d_n9;

        let (assign4020_e3785,) = {
    if (locals.var_guard66 == 0.0) {
        let assign4020_e3783: f64 = (p.p134).exp();
        (assign4020_e3783,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4020_e3785;

        let (assign4030_e3800, assign4030_e3800_d_n0, assign4030_e3800_d_n1, assign4030_e3800_d_n3, assign4030_e3800_d_n4, assign4030_e3800_d_n5, assign4030_e3800_d_n6, assign4030_e3800_d_n7, assign4030_e3800_d_n8, assign4030_e3800_d_n9,) = {
    if (locals.var_guard66 == 0.0) {
        let assign4030_e3792: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4030_e3794: f64 = (assign4030_e3792 / p.p133);
        let assign4030_e3796: f64 = (assign4030_e3794 - p.p134);
        let assign4030_e3797: f64 = (1.0 + assign4030_e3796);
        let assign4030_e3798: f64 = (locals.var_expl * assign4030_e3797);
        (assign4030_e3798, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1e1_dn3 * locals.var_vtinv) / p.p133)), (locals.var_expl * ((locals.var_vb1e1_dn4 * locals.var_vtinv) / p.p133)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign4030_e3800;
        locals.var_tmpexp_dn0 = assign4030_e3800_d_n0;
        locals.var_tmpexp_dn1 = assign4030_e3800_d_n1;
        locals.var_tmpexp_dn3 = assign4030_e3800_d_n3;
        locals.var_tmpexp_dn4 = assign4030_e3800_d_n4;
        locals.var_tmpexp_dn5 = assign4030_e3800_d_n5;
        locals.var_tmpexp_dn6 = assign4030_e3800_d_n6;
        locals.var_tmpexp_dn7 = assign4030_e3800_d_n7;
        locals.var_tmpexp_dn8 = assign4030_e3800_d_n8;
        locals.var_tmpexp_dn9 = assign4030_e3800_d_n9;

        let assign4050_e3816: f64 = if (((p.p33 > 0.0) && (p.p34 > 0.0)) && (locals.var_vb2e1 < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard67 = assign4050_e3816;

        let assign4060_e3822: f64 = (2.0 * locals.var_e0eb);
        let assign4060_e3823: f64 = (locals.var_pow2_2m_pe / assign4060_e3822);
        let assign4060_e3824: f64 = (1.0 - assign4060_e3823);
        let assign4060_e3825: f64 = (locals.var_nzeb_t * assign4060_e3824);
        let assign4060_e3827: f64 = if assign4060_e3825 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard68 = assign4060_e3827;

        let (assign4080_e3850,) = {
    if ((locals.var_guard67 != 0.0) && (locals.var_guard68 == 0.0)) {
        let assign4080_e3848: f64 = (p.p134).exp();
        (assign4080_e3848,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4080_e3850;

        let (assign4100_e3877, assign4100_e3877_d_n0, assign4100_e3877_d_n1, assign4100_e3877_d_n3, assign4100_e3877_d_n4, assign4100_e3877_d_n5, assign4100_e3877_d_n6, assign4100_e3877_d_n7, assign4100_e3877_d_n8, assign4100_e3877_d_n9,) = {
    if (locals.var_guard67 != 0.0) {
        let assign4100_e3875: f64 = (locals.var_vb2e1 * locals.var_inv_vde_t);
        (assign4100_e3875, (locals.var_vb2e1 * locals.var_inv_vde_t_dn0), (locals.var_vb2e1 * locals.var_inv_vde_t_dn1), ((locals.var_vb2e1_dn3 * locals.var_inv_vde_t) + (locals.var_vb2e1 * locals.var_inv_vde_t_dn3)), (locals.var_vb2e1 * locals.var_inv_vde_t_dn4), ((locals.var_vb2e1_dn5 * locals.var_inv_vde_t) + (locals.var_vb2e1 * locals.var_inv_vde_t_dn5)), (locals.var_vb2e1 * locals.var_inv_vde_t_dn6), (locals.var_vb2e1 * locals.var_inv_vde_t_dn7), (locals.var_vb2e1 * locals.var_inv_vde_t_dn8), (locals.var_vb2e1 * locals.var_inv_vde_t_dn9),)
    } else {
        (locals.var_x, locals.var_x_dn0, locals.var_x_dn1, locals.var_x_dn3, locals.var_x_dn4, locals.var_x_dn5, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8, locals.var_x_dn9,)
    }
};
        locals.var_x = assign4100_e3877;
        locals.var_x_dn0 = assign4100_e3877_d_n0;
        locals.var_x_dn1 = assign4100_e3877_d_n1;
        locals.var_x_dn3 = assign4100_e3877_d_n3;
        locals.var_x_dn4 = assign4100_e3877_d_n4;
        locals.var_x_dn5 = assign4100_e3877_d_n5;
        locals.var_x_dn6 = assign4100_e3877_d_n6;
        locals.var_x_dn7 = assign4100_e3877_d_n7;
        locals.var_x_dn8 = assign4100_e3877_d_n8;
        locals.var_x_dn9 = assign4100_e3877_d_n9;

        let (assign4110_e3921, assign4110_e3921_d_n0, assign4110_e3921_d_n1, assign4110_e3921_d_n3, assign4110_e3921_d_n4, assign4110_e3921_d_n5, assign4110_e3921_d_n6, assign4110_e3921_d_n7, assign4110_e3921_d_n8, assign4110_e3921_d_n9,) = {
    if (locals.var_guard67 != 0.0) {
        let assign4110_e3881: f64 = (locals.var_x * locals.var_x);
        let assign4110_e3883: f64 = (assign4110_e3881 + 1e-30);
        let assign4110_e3884: f64 = (assign4110_e3883).sqrt();
        let assign4110_e3886: f64 = (-2.0);
        let assign4110_e3888: f64 = (assign4110_e3886 - p.p66);
        let assign4110_e3889: f64 = (assign4110_e3884).powf(assign4110_e3888);
        let assign4110_e3894: f64 = (p.p66 * p.p66);
        let assign4110_e3895: f64 = (1.0 - assign4110_e3894);
        let assign4110_e3898: f64 = (3.0 * locals.var_x);
        let assign4110_e3901: f64 = (p.p66 - 1.0);
        let assign4110_e3902: f64 = (assign4110_e3898 * assign4110_e3901);
        let assign4110_e3903: f64 = (assign4110_e3895 - assign4110_e3902);
        let assign4110_e3904: f64 = (p.p66 * assign4110_e3903);
        let assign4110_e3907: f64 = (6.0 * locals.var_x);
        let assign4110_e3909: f64 = (assign4110_e3907 * locals.var_x);
        let assign4110_e3912: f64 = (p.p66 - 1.0);
        let assign4110_e3914: f64 = (assign4110_e3912 + locals.var_x);
        let assign4110_e3915: f64 = (assign4110_e3909 * assign4110_e3914);
        let assign4110_e3916: f64 = (assign4110_e3904 - assign4110_e3915);
        let assign4110_e3917: f64 = (assign4110_e3889 * assign4110_e3916);
        let assign4110_e3919: f64 = (assign4110_e3917 * 0.16666666666666666);
        (assign4110_e3919, (((if 0.0 == 0.0 && ((assign4110_e3888) as f64).is_finite() && ((assign4110_e3888) as f64).fract() == 0.0 { if assign4110_e3888 == 0.0 { 0.0 } else { (assign4110_e3888 * ((assign4110_e3884).powf(assign4110_e3888 - 1.0) * (((locals.var_x_dn0 * locals.var_x) + (locals.var_x * locals.var_x_dn0)) / (2.0 * assign4110_e3884)))) } } else { (assign4110_e3889 * (assign4110_e3888 * ((((locals.var_x_dn0 * locals.var_x) + (locals.var_x * locals.var_x_dn0)) / (2.0 * assign4110_e3884)) / assign4110_e3884))) } * assign4110_e3916) + (assign4110_e3889 * ((p.p66 * (-((3.0 * locals.var_x_dn0) * assign4110_e3901))) - (((((6.0 * locals.var_x_dn0) * locals.var_x) + (assign4110_e3907 * locals.var_x_dn0)) * assign4110_e3914) + (assign4110_e3909 * locals.var_x_dn0))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4110_e3888) as f64).is_finite() && ((assign4110_e3888) as f64).fract() == 0.0 { if assign4110_e3888 == 0.0 { 0.0 } else { (assign4110_e3888 * ((assign4110_e3884).powf(assign4110_e3888 - 1.0) * (((locals.var_x_dn1 * locals.var_x) + (locals.var_x * locals.var_x_dn1)) / (2.0 * assign4110_e3884)))) } } else { (assign4110_e3889 * (assign4110_e3888 * ((((locals.var_x_dn1 * locals.var_x) + (locals.var_x * locals.var_x_dn1)) / (2.0 * assign4110_e3884)) / assign4110_e3884))) } * assign4110_e3916) + (assign4110_e3889 * ((p.p66 * (-((3.0 * locals.var_x_dn1) * assign4110_e3901))) - (((((6.0 * locals.var_x_dn1) * locals.var_x) + (assign4110_e3907 * locals.var_x_dn1)) * assign4110_e3914) + (assign4110_e3909 * locals.var_x_dn1))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4110_e3888) as f64).is_finite() && ((assign4110_e3888) as f64).fract() == 0.0 { if assign4110_e3888 == 0.0 { 0.0 } else { (assign4110_e3888 * ((assign4110_e3884).powf(assign4110_e3888 - 1.0) * (((locals.var_x_dn3 * locals.var_x) + (locals.var_x * locals.var_x_dn3)) / (2.0 * assign4110_e3884)))) } } else { (assign4110_e3889 * (assign4110_e3888 * ((((locals.var_x_dn3 * locals.var_x) + (locals.var_x * locals.var_x_dn3)) / (2.0 * assign4110_e3884)) / assign4110_e3884))) } * assign4110_e3916) + (assign4110_e3889 * ((p.p66 * (-((3.0 * locals.var_x_dn3) * assign4110_e3901))) - (((((6.0 * locals.var_x_dn3) * locals.var_x) + (assign4110_e3907 * locals.var_x_dn3)) * assign4110_e3914) + (assign4110_e3909 * locals.var_x_dn3))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4110_e3888) as f64).is_finite() && ((assign4110_e3888) as f64).fract() == 0.0 { if assign4110_e3888 == 0.0 { 0.0 } else { (assign4110_e3888 * ((assign4110_e3884).powf(assign4110_e3888 - 1.0) * (((locals.var_x_dn4 * locals.var_x) + (locals.var_x * locals.var_x_dn4)) / (2.0 * assign4110_e3884)))) } } else { (assign4110_e3889 * (assign4110_e3888 * ((((locals.var_x_dn4 * locals.var_x) + (locals.var_x * locals.var_x_dn4)) / (2.0 * assign4110_e3884)) / assign4110_e3884))) } * assign4110_e3916) + (assign4110_e3889 * ((p.p66 * (-((3.0 * locals.var_x_dn4) * assign4110_e3901))) - (((((6.0 * locals.var_x_dn4) * locals.var_x) + (assign4110_e3907 * locals.var_x_dn4)) * assign4110_e3914) + (assign4110_e3909 * locals.var_x_dn4))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4110_e3888) as f64).is_finite() && ((assign4110_e3888) as f64).fract() == 0.0 { if assign4110_e3888 == 0.0 { 0.0 } else { (assign4110_e3888 * ((assign4110_e3884).powf(assign4110_e3888 - 1.0) * (((locals.var_x_dn5 * locals.var_x) + (locals.var_x * locals.var_x_dn5)) / (2.0 * assign4110_e3884)))) } } else { (assign4110_e3889 * (assign4110_e3888 * ((((locals.var_x_dn5 * locals.var_x) + (locals.var_x * locals.var_x_dn5)) / (2.0 * assign4110_e3884)) / assign4110_e3884))) } * assign4110_e3916) + (assign4110_e3889 * ((p.p66 * (-((3.0 * locals.var_x_dn5) * assign4110_e3901))) - (((((6.0 * locals.var_x_dn5) * locals.var_x) + (assign4110_e3907 * locals.var_x_dn5)) * assign4110_e3914) + (assign4110_e3909 * locals.var_x_dn5))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4110_e3888) as f64).is_finite() && ((assign4110_e3888) as f64).fract() == 0.0 { if assign4110_e3888 == 0.0 { 0.0 } else { (assign4110_e3888 * ((assign4110_e3884).powf(assign4110_e3888 - 1.0) * (((locals.var_x_dn6 * locals.var_x) + (locals.var_x * locals.var_x_dn6)) / (2.0 * assign4110_e3884)))) } } else { (assign4110_e3889 * (assign4110_e3888 * ((((locals.var_x_dn6 * locals.var_x) + (locals.var_x * locals.var_x_dn6)) / (2.0 * assign4110_e3884)) / assign4110_e3884))) } * assign4110_e3916) + (assign4110_e3889 * ((p.p66 * (-((3.0 * locals.var_x_dn6) * assign4110_e3901))) - (((((6.0 * locals.var_x_dn6) * locals.var_x) + (assign4110_e3907 * locals.var_x_dn6)) * assign4110_e3914) + (assign4110_e3909 * locals.var_x_dn6))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4110_e3888) as f64).is_finite() && ((assign4110_e3888) as f64).fract() == 0.0 { if assign4110_e3888 == 0.0 { 0.0 } else { (assign4110_e3888 * ((assign4110_e3884).powf(assign4110_e3888 - 1.0) * (((locals.var_x_dn7 * locals.var_x) + (locals.var_x * locals.var_x_dn7)) / (2.0 * assign4110_e3884)))) } } else { (assign4110_e3889 * (assign4110_e3888 * ((((locals.var_x_dn7 * locals.var_x) + (locals.var_x * locals.var_x_dn7)) / (2.0 * assign4110_e3884)) / assign4110_e3884))) } * assign4110_e3916) + (assign4110_e3889 * ((p.p66 * (-((3.0 * locals.var_x_dn7) * assign4110_e3901))) - (((((6.0 * locals.var_x_dn7) * locals.var_x) + (assign4110_e3907 * locals.var_x_dn7)) * assign4110_e3914) + (assign4110_e3909 * locals.var_x_dn7))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4110_e3888) as f64).is_finite() && ((assign4110_e3888) as f64).fract() == 0.0 { if assign4110_e3888 == 0.0 { 0.0 } else { (assign4110_e3888 * ((assign4110_e3884).powf(assign4110_e3888 - 1.0) * (((locals.var_x_dn8 * locals.var_x) + (locals.var_x * locals.var_x_dn8)) / (2.0 * assign4110_e3884)))) } } else { (assign4110_e3889 * (assign4110_e3888 * ((((locals.var_x_dn8 * locals.var_x) + (locals.var_x * locals.var_x_dn8)) / (2.0 * assign4110_e3884)) / assign4110_e3884))) } * assign4110_e3916) + (assign4110_e3889 * ((p.p66 * (-((3.0 * locals.var_x_dn8) * assign4110_e3901))) - (((((6.0 * locals.var_x_dn8) * locals.var_x) + (assign4110_e3907 * locals.var_x_dn8)) * assign4110_e3914) + (assign4110_e3909 * locals.var_x_dn8))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4110_e3888) as f64).is_finite() && ((assign4110_e3888) as f64).fract() == 0.0 { if assign4110_e3888 == 0.0 { 0.0 } else { (assign4110_e3888 * ((assign4110_e3884).powf(assign4110_e3888 - 1.0) * (((locals.var_x_dn9 * locals.var_x) + (locals.var_x * locals.var_x_dn9)) / (2.0 * assign4110_e3884)))) } } else { (assign4110_e3889 * (assign4110_e3888 * ((((locals.var_x_dn9 * locals.var_x) + (locals.var_x * locals.var_x_dn9)) / (2.0 * assign4110_e3884)) / assign4110_e3884))) } * assign4110_e3916) + (assign4110_e3889 * ((p.p66 * (-((3.0 * locals.var_x_dn9) * assign4110_e3901))) - (((((6.0 * locals.var_x_dn9) * locals.var_x) + (assign4110_e3907 * locals.var_x_dn9)) * assign4110_e3914) + (assign4110_e3909 * locals.var_x_dn9))))) * 0.16666666666666666),)
    } else {
        (locals.var_de0eb, locals.var_de0eb_dn0, locals.var_de0eb_dn1, locals.var_de0eb_dn3, locals.var_de0eb_dn4, locals.var_de0eb_dn5, locals.var_de0eb_dn6, locals.var_de0eb_dn7, locals.var_de0eb_dn8, locals.var_de0eb_dn9,)
    }
};
        locals.var_de0eb = assign4110_e3921;
        locals.var_de0eb_dn0 = assign4110_e3921_d_n0;
        locals.var_de0eb_dn1 = assign4110_e3921_d_n1;
        locals.var_de0eb_dn3 = assign4110_e3921_d_n3;
        locals.var_de0eb_dn4 = assign4110_e3921_d_n4;
        locals.var_de0eb_dn5 = assign4110_e3921_d_n5;
        locals.var_de0eb_dn6 = assign4110_e3921_d_n6;
        locals.var_de0eb_dn7 = assign4110_e3921_d_n7;
        locals.var_de0eb_dn8 = assign4110_e3921_d_n8;
        locals.var_de0eb_dn9 = assign4110_e3921_d_n9;

        let (assign4120_e3933, assign4120_e3933_d_n0, assign4120_e3933_d_n1, assign4120_e3933_d_n3, assign4120_e3933_d_n4, assign4120_e3933_d_n5, assign4120_e3933_d_n6, assign4120_e3933_d_n7, assign4120_e3933_d_n8, assign4120_e3933_d_n9,) = {
    if (locals.var_guard67 != 0.0) {
        let assign4120_e3925: f64 = (locals.var_vb2e1 * locals.var_pow2_2m_pe);
        let assign4120_e3927: f64 = (assign4120_e3925 * locals.var_nzeb_t);
        let assign4120_e3930: f64 = (locals.var_vgzeb_t * locals.var_de0eb);
        let assign4120_e3931: f64 = (assign4120_e3927 / assign4120_e3930);
        (assign4120_e3931, ((((assign4120_e3925 * locals.var_nzeb_t_dn0) * assign4120_e3930) - (assign4120_e3927 * ((locals.var_vgzeb_t_dn0 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn0)))) / (assign4120_e3930 * assign4120_e3930)), ((((assign4120_e3925 * locals.var_nzeb_t_dn1) * assign4120_e3930) - (assign4120_e3927 * ((locals.var_vgzeb_t_dn1 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn1)))) / (assign4120_e3930 * assign4120_e3930)), ((((((locals.var_vb2e1_dn3 * locals.var_pow2_2m_pe) * locals.var_nzeb_t) + (assign4120_e3925 * locals.var_nzeb_t_dn3)) * assign4120_e3930) - (assign4120_e3927 * ((locals.var_vgzeb_t_dn3 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn3)))) / (assign4120_e3930 * assign4120_e3930)), ((((assign4120_e3925 * locals.var_nzeb_t_dn4) * assign4120_e3930) - (assign4120_e3927 * ((locals.var_vgzeb_t_dn4 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn4)))) / (assign4120_e3930 * assign4120_e3930)), ((((((locals.var_vb2e1_dn5 * locals.var_pow2_2m_pe) * locals.var_nzeb_t) + (assign4120_e3925 * locals.var_nzeb_t_dn5)) * assign4120_e3930) - (assign4120_e3927 * ((locals.var_vgzeb_t_dn5 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn5)))) / (assign4120_e3930 * assign4120_e3930)), ((((assign4120_e3925 * locals.var_nzeb_t_dn6) * assign4120_e3930) - (assign4120_e3927 * ((locals.var_vgzeb_t_dn6 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn6)))) / (assign4120_e3930 * assign4120_e3930)), ((((assign4120_e3925 * locals.var_nzeb_t_dn7) * assign4120_e3930) - (assign4120_e3927 * ((locals.var_vgzeb_t_dn7 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn7)))) / (assign4120_e3930 * assign4120_e3930)), ((((assign4120_e3925 * locals.var_nzeb_t_dn8) * assign4120_e3930) - (assign4120_e3927 * ((locals.var_vgzeb_t_dn8 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn8)))) / (assign4120_e3930 * assign4120_e3930)), ((((assign4120_e3925 * locals.var_nzeb_t_dn9) * assign4120_e3930) - (assign4120_e3927 * ((locals.var_vgzeb_t_dn9 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn9)))) / (assign4120_e3930 * assign4120_e3930)),)
    } else {
        (locals.var_x, locals.var_x_dn0, locals.var_x_dn1, locals.var_x_dn3, locals.var_x_dn4, locals.var_x_dn5, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8, locals.var_x_dn9,)
    }
};
        locals.var_x = assign4120_e3933;
        locals.var_x_dn0 = assign4120_e3933_d_n0;
        locals.var_x_dn1 = assign4120_e3933_d_n1;
        locals.var_x_dn3 = assign4120_e3933_d_n3;
        locals.var_x_dn4 = assign4120_e3933_d_n4;
        locals.var_x_dn5 = assign4120_e3933_d_n5;
        locals.var_x_dn6 = assign4120_e3933_d_n6;
        locals.var_x_dn7 = assign4120_e3933_d_n7;
        locals.var_x_dn8 = assign4120_e3933_d_n8;
        locals.var_x_dn9 = assign4120_e3933_d_n9;

        let assign4130_e3936: f64 = (-0.001);
        let assign4130_e3937: f64 = if locals.var_x < assign4130_e3936 { 1.0 } else { 0.0 };
        locals.var_guard69 = assign4130_e3937;

        let assign4140_e3940: f64 = if locals.var_x < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign4140_e3940;

        let (assign4160_e3959,) = {
    if (((locals.var_guard67 != 0.0) && (locals.var_guard69 != 0.0)) && (locals.var_guard70 == 0.0)) {
        let assign4160_e3957: f64 = (p.p134).exp();
        (assign4160_e3957,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4160_e3959;

        let assign4230_e4049: f64 = if (((p.p35 > 0.0) && (p.p36 > 0.0)) && (locals.var_vb2c1 < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard71 = assign4230_e4049;

        let (assign4240_e4061,) = {
    if (locals.var_guard71 != 0.0) {
        let assign4240_e4054: f64 = (locals.var_vb2c1 * locals.var_inv_vdc_zener_t);
        let assign4240_e4055: f64 = (1.0 - assign4240_e4054);
        let assign4240_e4058: f64 = (1.0 - locals.var_pc_zener);
        let assign4240_e4059: f64 = (assign4240_e4055).powf(assign4240_e4058);
        (assign4240_e4059,)
    } else {
        (locals.var_e0cb,)
    }
};
        locals.var_e0cb = assign4240_e4061;

        let assign4250_e4067: f64 = (2.0 * locals.var_e0cb);
        let assign4250_e4068: f64 = (locals.var_pow2_2m_pc / assign4250_e4067);
        let assign4250_e4069: f64 = (1.0 - assign4250_e4068);
        let assign4250_e4070: f64 = (locals.var_nzcb_t * assign4250_e4069);
        let assign4250_e4072: f64 = if assign4250_e4070 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard72 = assign4250_e4072;

        let (assign4270_e4095,) = {
    if ((locals.var_guard71 != 0.0) && (locals.var_guard72 == 0.0)) {
        let assign4270_e4093: f64 = (p.p134).exp();
        (assign4270_e4093,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4270_e4095;

        let (assign4290_e4122,) = {
    if (locals.var_guard71 != 0.0) {
        let assign4290_e4120: f64 = (locals.var_vb2c1 * locals.var_inv_vdc_zener_t);
        (assign4290_e4120,)
    } else {
        (locals.var_xx,)
    }
};
        locals.var_xx = assign4290_e4122;

        let (assign4300_e4166,) = {
    if (locals.var_guard71 != 0.0) {
        let assign4300_e4126: f64 = (locals.var_xx * locals.var_xx);
        let assign4300_e4128: f64 = (assign4300_e4126 + 1e-30);
        let assign4300_e4129: f64 = (assign4300_e4128).sqrt();
        let assign4300_e4131: f64 = (-2.0);
        let assign4300_e4133: f64 = (assign4300_e4131 - locals.var_pc_zener);
        let assign4300_e4134: f64 = (assign4300_e4129).powf(assign4300_e4133);
        let assign4300_e4139: f64 = (locals.var_pc_zener * locals.var_pc_zener);
        let assign4300_e4140: f64 = (1.0 - assign4300_e4139);
        let assign4300_e4143: f64 = (3.0 * locals.var_xx);
        let assign4300_e4146: f64 = (locals.var_pc_zener - 1.0);
        let assign4300_e4147: f64 = (assign4300_e4143 * assign4300_e4146);
        let assign4300_e4148: f64 = (assign4300_e4140 - assign4300_e4147);
        let assign4300_e4149: f64 = (locals.var_pc_zener * assign4300_e4148);
        let assign4300_e4152: f64 = (6.0 * locals.var_xx);
        let assign4300_e4154: f64 = (assign4300_e4152 * locals.var_xx);
        let assign4300_e4157: f64 = (locals.var_pc_zener - 1.0);
        let assign4300_e4159: f64 = (assign4300_e4157 + locals.var_xx);
        let assign4300_e4160: f64 = (assign4300_e4154 * assign4300_e4159);
        let assign4300_e4161: f64 = (assign4300_e4149 - assign4300_e4160);
        let assign4300_e4162: f64 = (assign4300_e4134 * assign4300_e4161);
        let assign4300_e4164: f64 = (assign4300_e4162 * 0.16666666666666666);
        (assign4300_e4164,)
    } else {
        (locals.var_de0cb,)
    }
};
        locals.var_de0cb = assign4300_e4166;

        let (assign4310_e4178,) = {
    if (locals.var_guard71 != 0.0) {
        let assign4310_e4170: f64 = (locals.var_vb2c1 * locals.var_pow2_2m_pc);
        let assign4310_e4172: f64 = (assign4310_e4170 * locals.var_nzcb_t);
        let assign4310_e4175: f64 = (locals.var_vgzcb_t * locals.var_de0cb);
        let assign4310_e4176: f64 = (assign4310_e4172 / assign4310_e4175);
        (assign4310_e4176,)
    } else {
        (locals.var_xx,)
    }
};
        locals.var_xx = assign4310_e4178;

        let assign4320_e4181: f64 = (-0.001);
        let assign4320_e4182: f64 = if locals.var_xx < assign4320_e4181 { 1.0 } else { 0.0 };
        locals.var_guard73 = assign4320_e4182;

        let assign4330_e4185: f64 = if locals.var_xx < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard74 = assign4330_e4185;

        let (assign4350_e4204,) = {
    if (((locals.var_guard71 != 0.0) && (locals.var_guard73 != 0.0)) && (locals.var_guard74 == 0.0)) {
        let assign4350_e4202: f64 = (p.p134).exp();
        (assign4350_e4202,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4350_e4204;

        let assign4420_e4286: f64 = (locals.var_if0 * locals.var_evb1c4);
        locals.var_g1 = assign4420_e4286;
        locals.var_g1_dn0 = (locals.var_if0_dn0 * locals.var_evb1c4);
        locals.var_g1_dn1 = (locals.var_if0_dn1 * locals.var_evb1c4);
        locals.var_g1_dn3 = (locals.var_if0_dn3 * locals.var_evb1c4);
        locals.var_g1_dn4 = ((locals.var_if0_dn4 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn4));
        locals.var_g1_dn5 = ((locals.var_if0_dn5 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn5));
        locals.var_g1_dn6 = ((locals.var_if0_dn6 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn6));
        locals.var_g1_dn7 = ((locals.var_if0_dn7 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn7));
        locals.var_g1_dn8 = (locals.var_if0_dn8 * locals.var_evb1c4);
        locals.var_g1_dn9 = ((locals.var_if0_dn9 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn9));

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4430_e4289: f64 = (4.0 * locals.var_evb1c4vdc);
        locals.var_g2 = assign4430_e4289;
        locals.var_g2_dn0 = (4.0 * locals.var_evb1c4vdc_dn0);
        locals.var_g2_dn1 = (4.0 * locals.var_evb1c4vdc_dn1);
        locals.var_g2_dn3 = (4.0 * locals.var_evb1c4vdc_dn3);
        locals.var_g2_dn4 = (4.0 * locals.var_evb1c4vdc_dn4);
        locals.var_g2_dn5 = (4.0 * locals.var_evb1c4vdc_dn5);
        locals.var_g2_dn6 = (4.0 * locals.var_evb1c4vdc_dn6);
        locals.var_g2_dn7 = (4.0 * locals.var_evb1c4vdc_dn7);
        locals.var_g2_dn8 = (4.0 * locals.var_evb1c4vdc_dn8);
        locals.var_g2_dn9 = (4.0 * locals.var_evb1c4vdc_dn9);

        let assign4440_e4292: f64 = (locals.var_g1 - locals.var_if0);
        let assign4440_e4296: f64 = (1.0 + locals.var_g1);
        let assign4440_e4297: f64 = (assign4440_e4296).sqrt();
        let assign4440_e4298: f64 = (1.0 + assign4440_e4297);
        let assign4440_e4299: f64 = (assign4440_e4292 / assign4440_e4298);
        locals.var_nbex = assign4440_e4299;
        locals.var_nbex_dn0 = ((((locals.var_g1_dn0 - locals.var_if0_dn0) * assign4440_e4298) - (assign4440_e4292 * (locals.var_g1_dn0 / (2.0 * assign4440_e4297)))) / (assign4440_e4298 * assign4440_e4298));
        locals.var_nbex_dn1 = ((((locals.var_g1_dn1 - locals.var_if0_dn1) * assign4440_e4298) - (assign4440_e4292 * (locals.var_g1_dn1 / (2.0 * assign4440_e4297)))) / (assign4440_e4298 * assign4440_e4298));
        locals.var_nbex_dn3 = ((((locals.var_g1_dn3 - locals.var_if0_dn3) * assign4440_e4298) - (assign4440_e4292 * (locals.var_g1_dn3 / (2.0 * assign4440_e4297)))) / (assign4440_e4298 * assign4440_e4298));
        locals.var_nbex_dn4 = ((((locals.var_g1_dn4 - locals.var_if0_dn4) * assign4440_e4298) - (assign4440_e4292 * (locals.var_g1_dn4 / (2.0 * assign4440_e4297)))) / (assign4440_e4298 * assign4440_e4298));
        locals.var_nbex_dn5 = ((((locals.var_g1_dn5 - locals.var_if0_dn5) * assign4440_e4298) - (assign4440_e4292 * (locals.var_g1_dn5 / (2.0 * assign4440_e4297)))) / (assign4440_e4298 * assign4440_e4298));
        locals.var_nbex_dn6 = ((((locals.var_g1_dn6 - locals.var_if0_dn6) * assign4440_e4298) - (assign4440_e4292 * (locals.var_g1_dn6 / (2.0 * assign4440_e4297)))) / (assign4440_e4298 * assign4440_e4298));
        locals.var_nbex_dn7 = ((((locals.var_g1_dn7 - locals.var_if0_dn7) * assign4440_e4298) - (assign4440_e4292 * (locals.var_g1_dn7 / (2.0 * assign4440_e4297)))) / (assign4440_e4298 * assign4440_e4298));
        locals.var_nbex_dn8 = ((((locals.var_g1_dn8 - locals.var_if0_dn8) * assign4440_e4298) - (assign4440_e4292 * (locals.var_g1_dn8 / (2.0 * assign4440_e4297)))) / (assign4440_e4298 * assign4440_e4298));
        locals.var_nbex_dn9 = ((((locals.var_g1_dn9 - locals.var_if0_dn9) * assign4440_e4298) - (assign4440_e4292 * (locals.var_g1_dn9 / (2.0 * assign4440_e4297)))) / (assign4440_e4298 * assign4440_e4298));

        let assign4450_e4304: f64 = (1.0 + locals.var_g2);
        let assign4450_e4305: f64 = (assign4450_e4304).sqrt();
        let assign4450_e4306: f64 = (1.0 + assign4450_e4305);
        let assign4450_e4307: f64 = (locals.var_g2 / assign4450_e4306);
        locals.var_pwex = assign4450_e4307;
        locals.var_pwex_dn0 = (((locals.var_g2_dn0 * assign4450_e4306) - (locals.var_g2 * (locals.var_g2_dn0 / (2.0 * assign4450_e4305)))) / (assign4450_e4306 * assign4450_e4306));
        locals.var_pwex_dn1 = (((locals.var_g2_dn1 * assign4450_e4306) - (locals.var_g2 * (locals.var_g2_dn1 / (2.0 * assign4450_e4305)))) / (assign4450_e4306 * assign4450_e4306));
        locals.var_pwex_dn3 = (((locals.var_g2_dn3 * assign4450_e4306) - (locals.var_g2 * (locals.var_g2_dn3 / (2.0 * assign4450_e4305)))) / (assign4450_e4306 * assign4450_e4306));
        locals.var_pwex_dn4 = (((locals.var_g2_dn4 * assign4450_e4306) - (locals.var_g2 * (locals.var_g2_dn4 / (2.0 * assign4450_e4305)))) / (assign4450_e4306 * assign4450_e4306));
        locals.var_pwex_dn5 = (((locals.var_g2_dn5 * assign4450_e4306) - (locals.var_g2 * (locals.var_g2_dn5 / (2.0 * assign4450_e4305)))) / (assign4450_e4306 * assign4450_e4306));
        locals.var_pwex_dn6 = (((locals.var_g2_dn6 * assign4450_e4306) - (locals.var_g2 * (locals.var_g2_dn6 / (2.0 * assign4450_e4305)))) / (assign4450_e4306 * assign4450_e4306));
        locals.var_pwex_dn7 = (((locals.var_g2_dn7 * assign4450_e4306) - (locals.var_g2 * (locals.var_g2_dn7 / (2.0 * assign4450_e4305)))) / (assign4450_e4306 * assign4450_e4306));
        locals.var_pwex_dn8 = (((locals.var_g2_dn8 * assign4450_e4306) - (locals.var_g2 * (locals.var_g2_dn8 / (2.0 * assign4450_e4305)))) / (assign4450_e4306 * assign4450_e4306));
        locals.var_pwex_dn9 = (((locals.var_g2_dn9 * assign4450_e4306) - (locals.var_g2 * (locals.var_g2_dn9 / (2.0 * assign4450_e4305)))) / (assign4450_e4306 * assign4450_e4306));

        let assign4470_e4334: f64 = if ((p.p5 > 0.0) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard75 = assign4470_e4334;

        let (assign4490_e4365, assign4490_e4365_d_n0, assign4490_e4365_d_n1, assign4490_e4365_d_n4, assign4490_e4365_d_n5, assign4490_e4365_d_n6, assign4490_e4365_d_n7, assign4490_e4365_d_n8, assign4490_e4365_d_n9,) = {
    if (locals.var_guard75 != 0.0) {
        let assign4490_e4344: f64 = (p.p32 * 2.0);
        let assign4490_e4346: f64 = (assign4490_e4344 * locals.var_ibx_t);
        let assign4490_e4349: f64 = (locals.var_evbc3 - 1.0);
        let assign4490_e4350: f64 = (assign4490_e4346 * assign4490_e4349);
        let assign4490_e4355: f64 = (4.0 * locals.var_ibx_t);
        let assign4490_e4357: f64 = (assign4490_e4355 / locals.var_ikbx_t);
        let assign4490_e4359: f64 = (assign4490_e4357 * locals.var_evbc3);
        let assign4490_e4360: f64 = (1.0 + assign4490_e4359);
        let assign4490_e4361: f64 = (assign4490_e4360).sqrt();
        let assign4490_e4362: f64 = (1.0 + assign4490_e4361);
        let assign4490_e4363: f64 = (assign4490_e4350 / assign4490_e4362);
        (assign4490_e4363, ((((assign4490_e4346 * locals.var_evbc3_dn0) * assign4490_e4362) - (assign4490_e4350 * ((assign4490_e4357 * locals.var_evbc3_dn0) / (2.0 * assign4490_e4361)))) / (assign4490_e4362 * assign4490_e4362)), ((((assign4490_e4346 * locals.var_evbc3_dn1) * assign4490_e4362) - (assign4490_e4350 * ((assign4490_e4357 * locals.var_evbc3_dn1) / (2.0 * assign4490_e4361)))) / (assign4490_e4362 * assign4490_e4362)), ((((assign4490_e4346 * locals.var_evbc3_dn4) * assign4490_e4362) - (assign4490_e4350 * ((assign4490_e4357 * locals.var_evbc3_dn4) / (2.0 * assign4490_e4361)))) / (assign4490_e4362 * assign4490_e4362)), ((((assign4490_e4346 * locals.var_evbc3_dn5) * assign4490_e4362) - (assign4490_e4350 * ((assign4490_e4357 * locals.var_evbc3_dn5) / (2.0 * assign4490_e4361)))) / (assign4490_e4362 * assign4490_e4362)), ((((assign4490_e4346 * locals.var_evbc3_dn6) * assign4490_e4362) - (assign4490_e4350 * ((assign4490_e4357 * locals.var_evbc3_dn6) / (2.0 * assign4490_e4361)))) / (assign4490_e4362 * assign4490_e4362)), ((((assign4490_e4346 * locals.var_evbc3_dn7) * assign4490_e4362) - (assign4490_e4350 * ((assign4490_e4357 * locals.var_evbc3_dn7) / (2.0 * assign4490_e4361)))) / (assign4490_e4362 * assign4490_e4362)), ((((assign4490_e4346 * locals.var_evbc3_dn8) * assign4490_e4362) - (assign4490_e4350 * ((assign4490_e4357 * locals.var_evbc3_dn8) / (2.0 * assign4490_e4361)))) / (assign4490_e4362 * assign4490_e4362)), ((((assign4490_e4346 * locals.var_evbc3_dn9) * assign4490_e4362) - (assign4490_e4350 * ((assign4490_e4357 * locals.var_evbc3_dn9) / (2.0 * assign4490_e4361)))) / (assign4490_e4362 * assign4490_e4362)),)
    } else {
        (locals.var_ximex, locals.var_ximex_dn0, locals.var_ximex_dn1, locals.var_ximex_dn4, locals.var_ximex_dn5, locals.var_ximex_dn6, locals.var_ximex_dn7, locals.var_ximex_dn8, locals.var_ximex_dn9,)
    }
};
        locals.var_ximex = assign4490_e4365;
        locals.var_ximex_dn0 = assign4490_e4365_d_n0;
        locals.var_ximex_dn1 = assign4490_e4365_d_n1;
        locals.var_ximex_dn4 = assign4490_e4365_d_n4;
        locals.var_ximex_dn5 = assign4490_e4365_d_n5;
        locals.var_ximex_dn6 = assign4490_e4365_d_n6;
        locals.var_ximex_dn7 = assign4490_e4365_d_n7;
        locals.var_ximex_dn8 = assign4490_e4365_d_n8;
        locals.var_ximex_dn9 = assign4490_e4365_d_n9;

        let (assign4500_e4369,) = {
    if (locals.var_guard75 != 0.0) {
        (0.0,)
    } else {
        (locals.var_ximsub,)
    }
};
        locals.var_ximsub = assign4500_e4369;

        let assign4510_e4372: f64 = if p.p5 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign4510_e4372;

        let (assign4520_e4382,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign4520_e4378: f64 = (p.p32 * locals.var_ibx_t);
        let assign4520_e4380: f64 = (assign4520_e4378 * locals.var_rcc_xx_t);
        (assign4520_e4380,)
    } else {
        (locals.var_vex_bias,)
    }
};
        locals.var_vex_bias = assign4520_e4382;

        let (assign4530_e4395,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign4530_e4390: f64 = (locals.var_vex_bias * locals.var_vtinv);
        let assign4530_e4391: f64 = (assign4530_e4390).ln();
        let assign4530_e4392: f64 = (2.0 - assign4530_e4391);
        let assign4530_e4393: f64 = (locals.var_vt * assign4530_e4392);
        (assign4530_e4393,)
    } else {
        (locals.var_vex,)
    }
};
        locals.var_vex = assign4530_e4395;

        let (assign4540_e4403, assign4540_e4403_d_n0, assign4540_e4403_d_n1, assign4540_e4403_d_n4, assign4540_e4403_d_n5, assign4540_e4403_d_n6, assign4540_e4403_d_n7, assign4540_e4403_d_n8, assign4540_e4403_d_n9,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign4540_e4401: f64 = (locals.var_vbc3 - locals.var_vex);
        (assign4540_e4401, locals.var_vbc3_dn0, locals.var_vbc3_dn1, locals.var_vbc3_dn4, locals.var_vbc3_dn5, locals.var_vbc3_dn6, locals.var_vbc3_dn7, locals.var_vbc3_dn8, locals.var_vbc3_dn9,)
    } else {
        (locals.var_vdif, locals.var_vdif_dn0, locals.var_vdif_dn1, locals.var_vdif_dn4, locals.var_vdif_dn5, locals.var_vdif_dn6, locals.var_vdif_dn7, locals.var_vdif_dn8, locals.var_vdif_dn9,)
    }
};
        locals.var_vdif = assign4540_e4403;
        locals.var_vdif_dn0 = assign4540_e4403_d_n0;
        locals.var_vdif_dn1 = assign4540_e4403_d_n1;
        locals.var_vdif_dn4 = assign4540_e4403_d_n4;
        locals.var_vdif_dn5 = assign4540_e4403_d_n5;
        locals.var_vdif_dn6 = assign4540_e4403_d_n6;
        locals.var_vdif_dn7 = assign4540_e4403_d_n7;
        locals.var_vdif_dn8 = assign4540_e4403_d_n8;
        locals.var_vdif_dn9 = assign4540_e4403_d_n9;

        let (assign4550_e4411, assign4550_e4411_d_n0, assign4550_e4411_d_n1, assign4550_e4411_d_n3, assign4550_e4411_d_n4, assign4550_e4411_d_n5, assign4550_e4411_d_n6, assign4550_e4411_d_n7, assign4550_e4411_d_n8, assign4550_e4411_d_n9,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign4550_e4409: f64 = (0.11 * 0.11);
        (assign4550_e4409, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eps2, locals.var_eps2_dn0, locals.var_eps2_dn1, locals.var_eps2_dn3, locals.var_eps2_dn4, locals.var_eps2_dn5, locals.var_eps2_dn6, locals.var_eps2_dn7, locals.var_eps2_dn8, locals.var_eps2_dn9,)
    }
};
        locals.var_eps2 = assign4550_e4411;
        locals.var_eps2_dn0 = assign4550_e4411_d_n0;
        locals.var_eps2_dn1 = assign4550_e4411_d_n1;
        locals.var_eps2_dn3 = assign4550_e4411_d_n3;
        locals.var_eps2_dn4 = assign4550_e4411_d_n4;
        locals.var_eps2_dn5 = assign4550_e4411_d_n5;
        locals.var_eps2_dn6 = assign4550_e4411_d_n6;
        locals.var_eps2_dn7 = assign4550_e4411_d_n7;
        locals.var_eps2_dn8 = assign4550_e4411_d_n8;
        locals.var_eps2_dn9 = assign4550_e4411_d_n9;

        let (assign4560_e4419, assign4560_e4419_d_n0, assign4560_e4419_d_n1, assign4560_e4419_d_n3, assign4560_e4419_d_n4, assign4560_e4419_d_n5, assign4560_e4419_d_n6, assign4560_e4419_d_n7, assign4560_e4419_d_n8, assign4560_e4419_d_n9,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign4560_e4417: f64 = (locals.var_vdif * locals.var_vdif);
        (assign4560_e4417, ((locals.var_vdif_dn0 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn0)), ((locals.var_vdif_dn1 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn1)), 0.0, ((locals.var_vdif_dn4 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn4)), ((locals.var_vdif_dn5 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn5)), ((locals.var_vdif_dn6 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn6)), ((locals.var_vdif_dn7 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn7)), ((locals.var_vdif_dn8 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn8)), ((locals.var_vdif_dn9 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn9)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn1, locals.var_x2_dn3, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9,)
    }
};
        locals.var_x2 = assign4560_e4419;
        locals.var_x2_dn0 = assign4560_e4419_d_n0;
        locals.var_x2_dn1 = assign4560_e4419_d_n1;
        locals.var_x2_dn3 = assign4560_e4419_d_n3;
        locals.var_x2_dn4 = assign4560_e4419_d_n4;
        locals.var_x2_dn5 = assign4560_e4419_d_n5;
        locals.var_x2_dn6 = assign4560_e4419_d_n6;
        locals.var_x2_dn7 = assign4560_e4419_d_n7;
        locals.var_x2_dn8 = assign4560_e4419_d_n8;
        locals.var_x2_dn9 = assign4560_e4419_d_n9;

        let assign4570_e4422: f64 = if locals.var_vdif < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard77 = assign4570_e4422;

        let (assign4580_e4439, assign4580_e4439_d_n0, assign4580_e4439_d_n1, assign4580_e4439_d_n3, assign4580_e4439_d_n4, assign4580_e4439_d_n5, assign4580_e4439_d_n6, assign4580_e4439_d_n7, assign4580_e4439_d_n8, assign4580_e4439_d_n9,) = {
    if (((locals.var_guard75 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 != 0.0)) {
        let assign4580_e4430: f64 = (0.5 * locals.var_eps2);
        let assign4580_e4433: f64 = (locals.var_x2 + locals.var_eps2);
        let assign4580_e4434: f64 = (assign4580_e4433).sqrt();
        let assign4580_e4436: f64 = (assign4580_e4434 - locals.var_vdif);
        let assign4580_e4437: f64 = (assign4580_e4430 / assign4580_e4436);
        (assign4580_e4437, ((((0.5 * locals.var_eps2_dn0) * assign4580_e4436) - (assign4580_e4430 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign4580_e4434)) - locals.var_vdif_dn0))) / (assign4580_e4436 * assign4580_e4436)), ((((0.5 * locals.var_eps2_dn1) * assign4580_e4436) - (assign4580_e4430 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign4580_e4434)) - locals.var_vdif_dn1))) / (assign4580_e4436 * assign4580_e4436)), ((((0.5 * locals.var_eps2_dn3) * assign4580_e4436) - (assign4580_e4430 * ((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign4580_e4434)))) / (assign4580_e4436 * assign4580_e4436)), ((((0.5 * locals.var_eps2_dn4) * assign4580_e4436) - (assign4580_e4430 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign4580_e4434)) - locals.var_vdif_dn4))) / (assign4580_e4436 * assign4580_e4436)), ((((0.5 * locals.var_eps2_dn5) * assign4580_e4436) - (assign4580_e4430 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign4580_e4434)) - locals.var_vdif_dn5))) / (assign4580_e4436 * assign4580_e4436)), ((((0.5 * locals.var_eps2_dn6) * assign4580_e4436) - (assign4580_e4430 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign4580_e4434)) - locals.var_vdif_dn6))) / (assign4580_e4436 * assign4580_e4436)), ((((0.5 * locals.var_eps2_dn7) * assign4580_e4436) - (assign4580_e4430 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign4580_e4434)) - locals.var_vdif_dn7))) / (assign4580_e4436 * assign4580_e4436)), ((((0.5 * locals.var_eps2_dn8) * assign4580_e4436) - (assign4580_e4430 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign4580_e4434)) - locals.var_vdif_dn8))) / (assign4580_e4436 * assign4580_e4436)), ((((0.5 * locals.var_eps2_dn9) * assign4580_e4436) - (assign4580_e4430 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign4580_e4434)) - locals.var_vdif_dn9))) / (assign4580_e4436 * assign4580_e4436)),)
    } else {
        (locals.var_vbex, locals.var_vbex_dn0, locals.var_vbex_dn1, locals.var_vbex_dn3, locals.var_vbex_dn4, locals.var_vbex_dn5, locals.var_vbex_dn6, locals.var_vbex_dn7, locals.var_vbex_dn8, locals.var_vbex_dn9,)
    }
};
        locals.var_vbex = assign4580_e4439;
        locals.var_vbex_dn0 = assign4580_e4439_d_n0;
        locals.var_vbex_dn1 = assign4580_e4439_d_n1;
        locals.var_vbex_dn3 = assign4580_e4439_d_n3;
        locals.var_vbex_dn4 = assign4580_e4439_d_n4;
        locals.var_vbex_dn5 = assign4580_e4439_d_n5;
        locals.var_vbex_dn6 = assign4580_e4439_d_n6;
        locals.var_vbex_dn7 = assign4580_e4439_d_n7;
        locals.var_vbex_dn8 = assign4580_e4439_d_n8;
        locals.var_vbex_dn9 = assign4580_e4439_d_n9;

        let (assign4590_e4455, assign4590_e4455_d_n0, assign4590_e4455_d_n1, assign4590_e4455_d_n3, assign4590_e4455_d_n4, assign4590_e4455_d_n5, assign4590_e4455_d_n6, assign4590_e4455_d_n7, assign4590_e4455_d_n8, assign4590_e4455_d_n9,) = {
    if (((locals.var_guard75 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign4590_e4449: f64 = (locals.var_x2 + locals.var_eps2);
        let assign4590_e4450: f64 = (assign4590_e4449).sqrt();
        let assign4590_e4452: f64 = (assign4590_e4450 + locals.var_vdif);
        let assign4590_e4453: f64 = (0.5 * assign4590_e4452);
        (assign4590_e4453, (0.5 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign4590_e4450)) + locals.var_vdif_dn0)), (0.5 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign4590_e4450)) + locals.var_vdif_dn1)), (0.5 * ((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign4590_e4450))), (0.5 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign4590_e4450)) + locals.var_vdif_dn4)), (0.5 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign4590_e4450)) + locals.var_vdif_dn5)), (0.5 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign4590_e4450)) + locals.var_vdif_dn6)), (0.5 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign4590_e4450)) + locals.var_vdif_dn7)), (0.5 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign4590_e4450)) + locals.var_vdif_dn8)), (0.5 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign4590_e4450)) + locals.var_vdif_dn9)),)
    } else {
        (locals.var_vbex, locals.var_vbex_dn0, locals.var_vbex_dn1, locals.var_vbex_dn3, locals.var_vbex_dn4, locals.var_vbex_dn5, locals.var_vbex_dn6, locals.var_vbex_dn7, locals.var_vbex_dn8, locals.var_vbex_dn9,)
    }
};
        locals.var_vbex = assign4590_e4455;
        locals.var_vbex_dn0 = assign4590_e4455_d_n0;
        locals.var_vbex_dn1 = assign4590_e4455_d_n1;
        locals.var_vbex_dn3 = assign4590_e4455_d_n3;
        locals.var_vbex_dn4 = assign4590_e4455_d_n4;
        locals.var_vbex_dn5 = assign4590_e4455_d_n5;
        locals.var_vbex_dn6 = assign4590_e4455_d_n6;
        locals.var_vbex_dn7 = assign4590_e4455_d_n7;
        locals.var_vbex_dn8 = assign4590_e4455_d_n8;
        locals.var_vbex_dn9 = assign4590_e4455_d_n9;

        let (assign4600_e4471, assign4600_e4471_d_n0, assign4600_e4471_d_n1, assign4600_e4471_d_n3, assign4600_e4471_d_n4, assign4600_e4471_d_n5, assign4600_e4471_d_n6, assign4600_e4471_d_n7, assign4600_e4471_d_n8, assign4600_e4471_d_n9,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign4600_e4463: f64 = (locals.var_ximex + locals.var_ximsub);
        let assign4600_e4465: f64 = (assign4600_e4463 * locals.var_rcc_xx_t);
        let assign4600_e4466: f64 = (locals.var_vex_bias + assign4600_e4465);
        let assign4600_e4468: f64 = (assign4600_e4466 + locals.var_vbex);
        let assign4600_e4469: f64 = (locals.var_vbex / assign4600_e4468);
        (assign4600_e4469, (((locals.var_vbex_dn0 * assign4600_e4468) - (locals.var_vbex * ((locals.var_ximex_dn0 * locals.var_rcc_xx_t) + locals.var_vbex_dn0))) / (assign4600_e4468 * assign4600_e4468)), (((locals.var_vbex_dn1 * assign4600_e4468) - (locals.var_vbex * ((locals.var_ximex_dn1 * locals.var_rcc_xx_t) + locals.var_vbex_dn1))) / (assign4600_e4468 * assign4600_e4468)), (((locals.var_vbex_dn3 * assign4600_e4468) - (locals.var_vbex * locals.var_vbex_dn3)) / (assign4600_e4468 * assign4600_e4468)), (((locals.var_vbex_dn4 * assign4600_e4468) - (locals.var_vbex * ((locals.var_ximex_dn4 * locals.var_rcc_xx_t) + locals.var_vbex_dn4))) / (assign4600_e4468 * assign4600_e4468)), (((locals.var_vbex_dn5 * assign4600_e4468) - (locals.var_vbex * ((locals.var_ximex_dn5 * locals.var_rcc_xx_t) + locals.var_vbex_dn5))) / (assign4600_e4468 * assign4600_e4468)), (((locals.var_vbex_dn6 * assign4600_e4468) - (locals.var_vbex * ((locals.var_ximex_dn6 * locals.var_rcc_xx_t) + locals.var_vbex_dn6))) / (assign4600_e4468 * assign4600_e4468)), (((locals.var_vbex_dn7 * assign4600_e4468) - (locals.var_vbex * ((locals.var_ximex_dn7 * locals.var_rcc_xx_t) + locals.var_vbex_dn7))) / (assign4600_e4468 * assign4600_e4468)), (((locals.var_vbex_dn8 * assign4600_e4468) - (locals.var_vbex * ((locals.var_ximex_dn8 * locals.var_rcc_xx_t) + locals.var_vbex_dn8))) / (assign4600_e4468 * assign4600_e4468)), (((locals.var_vbex_dn9 * assign4600_e4468) - (locals.var_vbex * ((locals.var_ximex_dn9 * locals.var_rcc_xx_t) + locals.var_vbex_dn9))) / (assign4600_e4468 * assign4600_e4468)),)
    } else {
        (locals.var_fex, locals.var_fex_dn0, locals.var_fex_dn1, locals.var_fex_dn3, locals.var_fex_dn4, locals.var_fex_dn5, locals.var_fex_dn6, locals.var_fex_dn7, locals.var_fex_dn8, locals.var_fex_dn9,)
    }
};
        locals.var_fex = assign4600_e4471;
        locals.var_fex_dn0 = assign4600_e4471_d_n0;
        locals.var_fex_dn1 = assign4600_e4471_d_n1;
        locals.var_fex_dn3 = assign4600_e4471_d_n3;
        locals.var_fex_dn4 = assign4600_e4471_d_n4;
        locals.var_fex_dn5 = assign4600_e4471_d_n5;
        locals.var_fex_dn6 = assign4600_e4471_d_n6;
        locals.var_fex_dn7 = assign4600_e4471_d_n7;
        locals.var_fex_dn8 = assign4600_e4471_d_n8;
        locals.var_fex_dn9 = assign4600_e4471_d_n9;

        let (assign4610_e4478,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_vex,)
    }
};
        locals.var_vex = assign4610_e4478;

        let (assign4620_e4485, assign4620_e4485_d_n0, assign4620_e4485_d_n1, assign4620_e4485_d_n4, assign4620_e4485_d_n5, assign4620_e4485_d_n6, assign4620_e4485_d_n7, assign4620_e4485_d_n8, assign4620_e4485_d_n9,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vdif, locals.var_vdif_dn0, locals.var_vdif_dn1, locals.var_vdif_dn4, locals.var_vdif_dn5, locals.var_vdif_dn6, locals.var_vdif_dn7, locals.var_vdif_dn8, locals.var_vdif_dn9,)
    }
};
        locals.var_vdif = assign4620_e4485;
        locals.var_vdif_dn0 = assign4620_e4485_d_n0;
        locals.var_vdif_dn1 = assign4620_e4485_d_n1;
        locals.var_vdif_dn4 = assign4620_e4485_d_n4;
        locals.var_vdif_dn5 = assign4620_e4485_d_n5;
        locals.var_vdif_dn6 = assign4620_e4485_d_n6;
        locals.var_vdif_dn7 = assign4620_e4485_d_n7;
        locals.var_vdif_dn8 = assign4620_e4485_d_n8;
        locals.var_vdif_dn9 = assign4620_e4485_d_n9;

        let (assign4630_e4492, assign4630_e4492_d_n0, assign4630_e4492_d_n1, assign4630_e4492_d_n3, assign4630_e4492_d_n4, assign4630_e4492_d_n5, assign4630_e4492_d_n6, assign4630_e4492_d_n7, assign4630_e4492_d_n8, assign4630_e4492_d_n9,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbex, locals.var_vbex_dn0, locals.var_vbex_dn1, locals.var_vbex_dn3, locals.var_vbex_dn4, locals.var_vbex_dn5, locals.var_vbex_dn6, locals.var_vbex_dn7, locals.var_vbex_dn8, locals.var_vbex_dn9,)
    }
};
        locals.var_vbex = assign4630_e4492;
        locals.var_vbex_dn0 = assign4630_e4492_d_n0;
        locals.var_vbex_dn1 = assign4630_e4492_d_n1;
        locals.var_vbex_dn3 = assign4630_e4492_d_n3;
        locals.var_vbex_dn4 = assign4630_e4492_d_n4;
        locals.var_vbex_dn5 = assign4630_e4492_d_n5;
        locals.var_vbex_dn6 = assign4630_e4492_d_n6;
        locals.var_vbex_dn7 = assign4630_e4492_d_n7;
        locals.var_vbex_dn8 = assign4630_e4492_d_n8;
        locals.var_vbex_dn9 = assign4630_e4492_d_n9;

        let (assign4640_e4499, assign4640_e4499_d_n0, assign4640_e4499_d_n1, assign4640_e4499_d_n3, assign4640_e4499_d_n4, assign4640_e4499_d_n5, assign4640_e4499_d_n6, assign4640_e4499_d_n7, assign4640_e4499_d_n8, assign4640_e4499_d_n9,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fex, locals.var_fex_dn0, locals.var_fex_dn1, locals.var_fex_dn3, locals.var_fex_dn4, locals.var_fex_dn5, locals.var_fex_dn6, locals.var_fex_dn7, locals.var_fex_dn8, locals.var_fex_dn9,)
    }
};
        locals.var_fex = assign4640_e4499;
        locals.var_fex_dn0 = assign4640_e4499_d_n0;
        locals.var_fex_dn1 = assign4640_e4499_d_n1;
        locals.var_fex_dn3 = assign4640_e4499_d_n3;
        locals.var_fex_dn4 = assign4640_e4499_d_n4;
        locals.var_fex_dn5 = assign4640_e4499_d_n5;
        locals.var_fex_dn6 = assign4640_e4499_d_n6;
        locals.var_fex_dn7 = assign4640_e4499_d_n7;
        locals.var_fex_dn8 = assign4640_e4499_d_n8;
        locals.var_fex_dn9 = assign4640_e4499_d_n9;

        let assign4660_e4508: f64 = if p.p83 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard78 = assign4660_e4508;

        let (assign4670_e4514, assign4670_e4514_d_n4, assign4670_e4514_d_n5, assign4670_e4514_d_n6,) = {
    if (locals.var_guard78 != 0.0) {
        let assign4670_e4512: f64 = (locals.var_vb1b2 + locals.var_vb2c1);
        (assign4670_e4512, locals.var_vb1b2_dn4, (locals.var_vb1b2_dn5 + locals.var_vb2c1_dn5), locals.var_vb2c1_dn6,)
    } else {
        (locals.var_vb1c1, locals.var_vb1c1_dn4, locals.var_vb1c1_dn5, locals.var_vb1c1_dn6,)
    }
};
        locals.var_vb1c1 = assign4670_e4514;
        locals.var_vb1c1_dn4 = assign4670_e4514_d_n4;
        locals.var_vb1c1_dn5 = assign4670_e4514_d_n5;
        locals.var_vb1c1_dn6 = assign4670_e4514_d_n6;

        let (assign4680_e4520, assign4680_e4520_d_n0, assign4680_e4520_d_n1, assign4680_e4520_d_n3, assign4680_e4520_d_n4, assign4680_e4520_d_n5, assign4680_e4520_d_n6, assign4680_e4520_d_n7, assign4680_e4520_d_n8, assign4680_e4520_d_n9,) = {
    if (locals.var_guard78 != 0.0) {
        let assign4680_e4518: f64 = (1e-6 * 1e-6);
        (assign4680_e4518, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eps2, locals.var_eps2_dn0, locals.var_eps2_dn1, locals.var_eps2_dn3, locals.var_eps2_dn4, locals.var_eps2_dn5, locals.var_eps2_dn6, locals.var_eps2_dn7, locals.var_eps2_dn8, locals.var_eps2_dn9,)
    }
};
        locals.var_eps2 = assign4680_e4520;
        locals.var_eps2_dn0 = assign4680_e4520_d_n0;
        locals.var_eps2_dn1 = assign4680_e4520_d_n1;
        locals.var_eps2_dn3 = assign4680_e4520_d_n3;
        locals.var_eps2_dn4 = assign4680_e4520_d_n4;
        locals.var_eps2_dn5 = assign4680_e4520_d_n5;
        locals.var_eps2_dn6 = assign4680_e4520_d_n6;
        locals.var_eps2_dn7 = assign4680_e4520_d_n7;
        locals.var_eps2_dn8 = assign4680_e4520_d_n8;
        locals.var_eps2_dn9 = assign4680_e4520_d_n9;

        let (assign4690_e4532, assign4690_e4532_d_n0, assign4690_e4532_d_n1, assign4690_e4532_d_n3, assign4690_e4532_d_n4, assign4690_e4532_d_n5, assign4690_e4532_d_n6, assign4690_e4532_d_n7, assign4690_e4532_d_n8, assign4690_e4532_d_n9,) = {
    if (locals.var_guard78 != 0.0) {
        let assign4690_e4523: f64 = (-1.0);
        let assign4690_e4525: f64 = (assign4690_e4523 * locals.var_vb1c1);
        let assign4690_e4527: f64 = (-1.0);
        let assign4690_e4528: f64 = (assign4690_e4525 * assign4690_e4527);
        let assign4690_e4530: f64 = (assign4690_e4528 * locals.var_vb1c1);
        (assign4690_e4530, 0.0, 0.0, 0.0, ((((assign4690_e4523 * locals.var_vb1c1_dn4) * assign4690_e4527) * locals.var_vb1c1) + (assign4690_e4528 * locals.var_vb1c1_dn4)), ((((assign4690_e4523 * locals.var_vb1c1_dn5) * assign4690_e4527) * locals.var_vb1c1) + (assign4690_e4528 * locals.var_vb1c1_dn5)), ((((assign4690_e4523 * locals.var_vb1c1_dn6) * assign4690_e4527) * locals.var_vb1c1) + (assign4690_e4528 * locals.var_vb1c1_dn6)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn1, locals.var_x2_dn3, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9,)
    }
};
        locals.var_x2 = assign4690_e4532;
        locals.var_x2_dn0 = assign4690_e4532_d_n0;
        locals.var_x2_dn1 = assign4690_e4532_d_n1;
        locals.var_x2_dn3 = assign4690_e4532_d_n3;
        locals.var_x2_dn4 = assign4690_e4532_d_n4;
        locals.var_x2_dn5 = assign4690_e4532_d_n5;
        locals.var_x2_dn6 = assign4690_e4532_d_n6;
        locals.var_x2_dn7 = assign4690_e4532_d_n7;
        locals.var_x2_dn8 = assign4690_e4532_d_n8;
        locals.var_x2_dn9 = assign4690_e4532_d_n9;

        let assign4840_e4656: f64 = (locals.var_vte / locals.var_ver_t);
        let assign4840_e4657: f64 = (1.0 + assign4840_e4656);
        let assign4840_e4660: f64 = (locals.var_vtc / locals.var_vef_t);
        let assign4840_e4661: f64 = (assign4840_e4657 + assign4840_e4660);
        locals.var_q0q = assign4840_e4661;
        locals.var_q0q_dn0 = ((((locals.var_vte_dn0 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn0)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn0 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn0)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn1 = ((((locals.var_vte_dn1 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn1)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn1 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn1)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn3 = ((((locals.var_vte_dn3 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn3)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn3 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn3)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn4 = ((((locals.var_vte_dn4 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn4)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn4 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn4)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn5 = ((((locals.var_vte_dn5 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn5)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn5 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn5)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn6 = ((((locals.var_vte_dn6 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn6)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn6 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn6)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn7 = ((((locals.var_vte_dn7 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn7)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn7 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn7)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn8 = ((((locals.var_vte_dn8 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn8)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn8 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn8)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn9 = ((((locals.var_vte_dn9 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn9)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn9 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn9)) / (locals.var_vef_t * locals.var_vef_t)));

        let assign4850_e4664: f64 = (0.1 * 0.1);
        locals.var_eps2 = assign4850_e4664;
        locals.var_eps2_dn0 = 0.0;
        locals.var_eps2_dn1 = 0.0;
        locals.var_eps2_dn3 = 0.0;
        locals.var_eps2_dn4 = 0.0;
        locals.var_eps2_dn5 = 0.0;
        locals.var_eps2_dn6 = 0.0;
        locals.var_eps2_dn7 = 0.0;
        locals.var_eps2_dn8 = 0.0;
        locals.var_eps2_dn9 = 0.0;

        let assign4860_e4667: f64 = (locals.var_q0q * locals.var_q0q);
        locals.var_x2 = assign4860_e4667;
        locals.var_x2_dn0 = ((locals.var_q0q_dn0 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn0));
        locals.var_x2_dn1 = ((locals.var_q0q_dn1 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn1));
        locals.var_x2_dn3 = ((locals.var_q0q_dn3 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn3));
        locals.var_x2_dn4 = ((locals.var_q0q_dn4 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn4));
        locals.var_x2_dn5 = ((locals.var_q0q_dn5 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn5));
        locals.var_x2_dn6 = ((locals.var_q0q_dn6 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn6));
        locals.var_x2_dn7 = ((locals.var_q0q_dn7 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn7));
        locals.var_x2_dn8 = ((locals.var_q0q_dn8 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn8));
        locals.var_x2_dn9 = ((locals.var_q0q_dn9 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn9));

        let assign4870_e4670: f64 = if locals.var_q0q < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard81 = assign4870_e4670;

        let (assign4880_e4683, assign4880_e4683_d_n0, assign4880_e4683_d_n1, assign4880_e4683_d_n3, assign4880_e4683_d_n4, assign4880_e4683_d_n5, assign4880_e4683_d_n6, assign4880_e4683_d_n7, assign4880_e4683_d_n8, assign4880_e4683_d_n9,) = {
    if (locals.var_guard81 != 0.0) {
        let assign4880_e4674: f64 = (0.5 * locals.var_eps2);
        let assign4880_e4677: f64 = (locals.var_x2 + locals.var_eps2);
        let assign4880_e4678: f64 = (assign4880_e4677).sqrt();
        let assign4880_e4680: f64 = (assign4880_e4678 - locals.var_q0q);
        let assign4880_e4681: f64 = (assign4880_e4674 / assign4880_e4680);
        (assign4880_e4681, ((((0.5 * locals.var_eps2_dn0) * assign4880_e4680) - (assign4880_e4674 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign4880_e4678)) - locals.var_q0q_dn0))) / (assign4880_e4680 * assign4880_e4680)), ((((0.5 * locals.var_eps2_dn1) * assign4880_e4680) - (assign4880_e4674 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign4880_e4678)) - locals.var_q0q_dn1))) / (assign4880_e4680 * assign4880_e4680)), ((((0.5 * locals.var_eps2_dn3) * assign4880_e4680) - (assign4880_e4674 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign4880_e4678)) - locals.var_q0q_dn3))) / (assign4880_e4680 * assign4880_e4680)), ((((0.5 * locals.var_eps2_dn4) * assign4880_e4680) - (assign4880_e4674 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign4880_e4678)) - locals.var_q0q_dn4))) / (assign4880_e4680 * assign4880_e4680)), ((((0.5 * locals.var_eps2_dn5) * assign4880_e4680) - (assign4880_e4674 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign4880_e4678)) - locals.var_q0q_dn5))) / (assign4880_e4680 * assign4880_e4680)), ((((0.5 * locals.var_eps2_dn6) * assign4880_e4680) - (assign4880_e4674 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign4880_e4678)) - locals.var_q0q_dn6))) / (assign4880_e4680 * assign4880_e4680)), ((((0.5 * locals.var_eps2_dn7) * assign4880_e4680) - (assign4880_e4674 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign4880_e4678)) - locals.var_q0q_dn7))) / (assign4880_e4680 * assign4880_e4680)), ((((0.5 * locals.var_eps2_dn8) * assign4880_e4680) - (assign4880_e4674 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign4880_e4678)) - locals.var_q0q_dn8))) / (assign4880_e4680 * assign4880_e4680)), ((((0.5 * locals.var_eps2_dn9) * assign4880_e4680) - (assign4880_e4674 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign4880_e4678)) - locals.var_q0q_dn9))) / (assign4880_e4680 * assign4880_e4680)),)
    } else {
        (locals.var_q1q, locals.var_q1q_dn0, locals.var_q1q_dn1, locals.var_q1q_dn3, locals.var_q1q_dn4, locals.var_q1q_dn5, locals.var_q1q_dn6, locals.var_q1q_dn7, locals.var_q1q_dn8, locals.var_q1q_dn9,)
    }
};
        locals.var_q1q = assign4880_e4683;
        locals.var_q1q_dn0 = assign4880_e4683_d_n0;
        locals.var_q1q_dn1 = assign4880_e4683_d_n1;
        locals.var_q1q_dn3 = assign4880_e4683_d_n3;
        locals.var_q1q_dn4 = assign4880_e4683_d_n4;
        locals.var_q1q_dn5 = assign4880_e4683_d_n5;
        locals.var_q1q_dn6 = assign4880_e4683_d_n6;
        locals.var_q1q_dn7 = assign4880_e4683_d_n7;
        locals.var_q1q_dn8 = assign4880_e4683_d_n8;
        locals.var_q1q_dn9 = assign4880_e4683_d_n9;

        let (assign4890_e4695, assign4890_e4695_d_n0, assign4890_e4695_d_n1, assign4890_e4695_d_n3, assign4890_e4695_d_n4, assign4890_e4695_d_n5, assign4890_e4695_d_n6, assign4890_e4695_d_n7, assign4890_e4695_d_n8, assign4890_e4695_d_n9,) = {
    if (locals.var_guard81 == 0.0) {
        let assign4890_e4689: f64 = (locals.var_x2 + locals.var_eps2);
        let assign4890_e4690: f64 = (assign4890_e4689).sqrt();
        let assign4890_e4692: f64 = (assign4890_e4690 + locals.var_q0q);
        let assign4890_e4693: f64 = (0.5 * assign4890_e4692);
        (assign4890_e4693, (0.5 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign4890_e4690)) + locals.var_q0q_dn0)), (0.5 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign4890_e4690)) + locals.var_q0q_dn1)), (0.5 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign4890_e4690)) + locals.var_q0q_dn3)), (0.5 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign4890_e4690)) + locals.var_q0q_dn4)), (0.5 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign4890_e4690)) + locals.var_q0q_dn5)), (0.5 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign4890_e4690)) + locals.var_q0q_dn6)), (0.5 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign4890_e4690)) + locals.var_q0q_dn7)), (0.5 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign4890_e4690)) + locals.var_q0q_dn8)), (0.5 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign4890_e4690)) + locals.var_q0q_dn9)),)
    } else {
        (locals.var_q1q, locals.var_q1q_dn0, locals.var_q1q_dn1, locals.var_q1q_dn3, locals.var_q1q_dn4, locals.var_q1q_dn5, locals.var_q1q_dn6, locals.var_q1q_dn7, locals.var_q1q_dn8, locals.var_q1q_dn9,)
    }
};
        locals.var_q1q = assign4890_e4695;
        locals.var_q1q_dn0 = assign4890_e4695_d_n0;
        locals.var_q1q_dn1 = assign4890_e4695_d_n1;
        locals.var_q1q_dn3 = assign4890_e4695_d_n3;
        locals.var_q1q_dn4 = assign4890_e4695_d_n4;
        locals.var_q1q_dn5 = assign4890_e4695_d_n5;
        locals.var_q1q_dn6 = assign4890_e4695_d_n6;
        locals.var_q1q_dn7 = assign4890_e4695_d_n7;
        locals.var_q1q_dn8 = assign4890_e4695_d_n8;
        locals.var_q1q_dn9 = assign4890_e4695_d_n9;

        let assign4900_e4701: f64 = (locals.var_n0 + locals.var_nb);
        let assign4900_e4702: f64 = (0.5 * assign4900_e4701);
        let assign4900_e4703: f64 = (1.0 + assign4900_e4702);
        let assign4900_e4704: f64 = (locals.var_q1q * assign4900_e4703);
        locals.var_qbq = assign4900_e4704;
        locals.var_qbq_dn0 = ((locals.var_q1q_dn0 * assign4900_e4703) + (locals.var_q1q * (0.5 * (locals.var_n0_dn0 + locals.var_nb_dn0))));
        locals.var_qbq_dn1 = ((locals.var_q1q_dn1 * assign4900_e4703) + (locals.var_q1q * (0.5 * (locals.var_n0_dn1 + locals.var_nb_dn1))));
        locals.var_qbq_dn3 = ((locals.var_q1q_dn3 * assign4900_e4703) + (locals.var_q1q * (0.5 * (locals.var_n0_dn3 + locals.var_nb_dn3))));
        locals.var_qbq_dn4 = ((locals.var_q1q_dn4 * assign4900_e4703) + (locals.var_q1q * (0.5 * (locals.var_n0_dn4 + locals.var_nb_dn4))));
        locals.var_qbq_dn5 = ((locals.var_q1q_dn5 * assign4900_e4703) + (locals.var_q1q * (0.5 * (locals.var_n0_dn5 + locals.var_nb_dn5))));
        locals.var_qbq_dn6 = ((locals.var_q1q_dn6 * assign4900_e4703) + (locals.var_q1q * (0.5 * (locals.var_n0_dn6 + locals.var_nb_dn6))));
        locals.var_qbq_dn7 = ((locals.var_q1q_dn7 * assign4900_e4703) + (locals.var_q1q * (0.5 * (locals.var_n0_dn7 + locals.var_nb_dn7))));
        locals.var_qbq_dn8 = ((locals.var_q1q_dn8 * assign4900_e4703) + (locals.var_q1q * (0.5 * (locals.var_n0_dn8 + locals.var_nb_dn8))));
        locals.var_qbq_dn9 = ((locals.var_q1q_dn9 * assign4900_e4703) + (locals.var_q1q * (0.5 * (locals.var_n0_dn9 + locals.var_nb_dn9))));

        let assign4910_e4707: f64 = (locals.var_rbv_t / locals.var_qbq);
        locals.var_rbvtemp = assign4910_e4707;
        locals.var_rbvtemp_dn0 = (-((locals.var_rbv_t * locals.var_qbq_dn0) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn1 = (-((locals.var_rbv_t * locals.var_qbq_dn1) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn3 = (-((locals.var_rbv_t * locals.var_qbq_dn3) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn4 = (-((locals.var_rbv_t * locals.var_qbq_dn4) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn5 = (-((locals.var_rbv_t * locals.var_qbq_dn5) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn6 = (-((locals.var_rbv_t * locals.var_qbq_dn6) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn7 = (-((locals.var_rbv_t * locals.var_qbq_dn7) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn8 = (-((locals.var_rbv_t * locals.var_qbq_dn8) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn9 = (-((locals.var_rbv_t * locals.var_qbq_dn9) / (locals.var_qbq * locals.var_qbq)));

        let assign4920_e4710: f64 = if locals.var_rbvtemp < locals.var_minr_m { 1.0 } else { 0.0 };
        locals.var_guard82 = assign4920_e4710;

        let (assign4930_e4714, assign4930_e4714_d_n0, assign4930_e4714_d_n1, assign4930_e4714_d_n3, assign4930_e4714_d_n4, assign4930_e4714_d_n5, assign4930_e4714_d_n6, assign4930_e4714_d_n7, assign4930_e4714_d_n8, assign4930_e4714_d_n9,) = {
    if (locals.var_guard82 != 0.0) {
        (locals.var_minr_m, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rbvtemp, locals.var_rbvtemp_dn0, locals.var_rbvtemp_dn1, locals.var_rbvtemp_dn3, locals.var_rbvtemp_dn4, locals.var_rbvtemp_dn5, locals.var_rbvtemp_dn6, locals.var_rbvtemp_dn7, locals.var_rbvtemp_dn8, locals.var_rbvtemp_dn9,)
    }
};
        locals.var_rbvtemp = assign4930_e4714;
        locals.var_rbvtemp_dn0 = assign4930_e4714_d_n0;
        locals.var_rbvtemp_dn1 = assign4930_e4714_d_n1;
        locals.var_rbvtemp_dn3 = assign4930_e4714_d_n3;
        locals.var_rbvtemp_dn4 = assign4930_e4714_d_n4;
        locals.var_rbvtemp_dn5 = assign4930_e4714_d_n5;
        locals.var_rbvtemp_dn6 = assign4930_e4714_d_n6;
        locals.var_rbvtemp_dn7 = assign4930_e4714_d_n7;
        locals.var_rbvtemp_dn8 = assign4930_e4714_d_n8;
        locals.var_rbvtemp_dn9 = assign4930_e4714_d_n9;

        let assign4940_e4717: f64 = (3.0 * locals.var_rbvtemp);
        locals.var_rb2 = assign4940_e4717;
        locals.var_rb2_dn0 = (3.0 * locals.var_rbvtemp_dn0);
        locals.var_rb2_dn1 = (3.0 * locals.var_rbvtemp_dn1);
        locals.var_rb2_dn3 = (3.0 * locals.var_rbvtemp_dn3);
        locals.var_rb2_dn4 = (3.0 * locals.var_rbvtemp_dn4);
        locals.var_rb2_dn5 = (3.0 * locals.var_rbvtemp_dn5);
        locals.var_rb2_dn6 = (3.0 * locals.var_rbvtemp_dn6);
        locals.var_rb2_dn7 = (3.0 * locals.var_rbvtemp_dn7);
        locals.var_rb2_dn8 = (3.0 * locals.var_rbvtemp_dn8);
        locals.var_rb2_dn9 = (3.0 * locals.var_rbvtemp_dn9);

        let assign4960_e4731: f64 = if locals.var_in_ > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign4960_e4731;

        let assign4970_e4734: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign4970_e4734;

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4980_e4737: f64 = if locals.var_vb2c1 < p.p43 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign4980_e4737;

        let assign4990_e4739: f64 = (-locals.var_in_);
        let assign4990_e4741: f64 = (assign4990_e4739 / p.p41);
        let assign4990_e4743: f64 = if assign4990_e4741 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign4990_e4743;

        let (assign5000_e4757, assign5000_e4757_d_n0, assign5000_e4757_d_n1, assign5000_e4757_d_n3, assign5000_e4757_d_n4, assign5000_e4757_d_n5, assign5000_e4757_d_n6, assign5000_e4757_d_n7, assign5000_e4757_d_n8, assign5000_e4757_d_n9,) = {
    if ((((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign5000_e4752: f64 = (-locals.var_in_);
        let assign5000_e4754: f64 = (assign5000_e4752 / p.p41);
        let assign5000_e4755: f64 = (assign5000_e4754).exp();
        (assign5000_e4755, (assign5000_e4755 * ((-locals.var_in__dn0) / p.p41)), (assign5000_e4755 * ((-locals.var_in__dn1) / p.p41)), (assign5000_e4755 * ((-locals.var_in__dn3) / p.p41)), (assign5000_e4755 * ((-locals.var_in__dn4) / p.p41)), (assign5000_e4755 * ((-locals.var_in__dn5) / p.p41)), (assign5000_e4755 * ((-locals.var_in__dn6) / p.p41)), (assign5000_e4755 * ((-locals.var_in__dn7) / p.p41)), (assign5000_e4755 * ((-locals.var_in__dn8) / p.p41)), (assign5000_e4755 * ((-locals.var_in__dn9) / p.p41)),)
    } else {
        (locals.var_expin, locals.var_expin_dn0, locals.var_expin_dn1, locals.var_expin_dn3, locals.var_expin_dn4, locals.var_expin_dn5, locals.var_expin_dn6, locals.var_expin_dn7, locals.var_expin_dn8, locals.var_expin_dn9,)
    }
};
        locals.var_expin = assign5000_e4757;
        locals.var_expin_dn0 = assign5000_e4757_d_n0;
        locals.var_expin_dn1 = assign5000_e4757_d_n1;
        locals.var_expin_dn3 = assign5000_e4757_d_n3;
        locals.var_expin_dn4 = assign5000_e4757_d_n4;
        locals.var_expin_dn5 = assign5000_e4757_d_n5;
        locals.var_expin_dn6 = assign5000_e4757_d_n6;
        locals.var_expin_dn7 = assign5000_e4757_d_n7;
        locals.var_expin_dn8 = assign5000_e4757_d_n8;
        locals.var_expin_dn9 = assign5000_e4757_d_n9;

        let (assign5010_e4769,) = {
    if ((((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard86 == 0.0)) {
        let assign5010_e4767: f64 = (p.p134).exp();
        (assign5010_e4767,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5010_e4769;

        let (assign5020_e4789, assign5020_e4789_d_n0, assign5020_e4789_d_n1, assign5020_e4789_d_n3, assign5020_e4789_d_n4, assign5020_e4789_d_n5, assign5020_e4789_d_n6, assign5020_e4789_d_n7, assign5020_e4789_d_n8, assign5020_e4789_d_n9,) = {
    if ((((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard86 == 0.0)) {
        let assign5020_e4781: f64 = (-locals.var_in_);
        let assign5020_e4783: f64 = (assign5020_e4781 / p.p41);
        let assign5020_e4785: f64 = (assign5020_e4783 - p.p134);
        let assign5020_e4786: f64 = (1.0 + assign5020_e4785);
        let assign5020_e4787: f64 = (locals.var_expl * assign5020_e4786);
        (assign5020_e4787, (locals.var_expl * ((-locals.var_in__dn0) / p.p41)), (locals.var_expl * ((-locals.var_in__dn1) / p.p41)), (locals.var_expl * ((-locals.var_in__dn3) / p.p41)), (locals.var_expl * ((-locals.var_in__dn4) / p.p41)), (locals.var_expl * ((-locals.var_in__dn5) / p.p41)), (locals.var_expl * ((-locals.var_in__dn6) / p.p41)), (locals.var_expl * ((-locals.var_in__dn7) / p.p41)), (locals.var_expl * ((-locals.var_in__dn8) / p.p41)), (locals.var_expl * ((-locals.var_in__dn9) / p.p41)),)
    } else {
        (locals.var_expin, locals.var_expin_dn0, locals.var_expin_dn1, locals.var_expin_dn3, locals.var_expin_dn4, locals.var_expin_dn5, locals.var_expin_dn6, locals.var_expin_dn7, locals.var_expin_dn8, locals.var_expin_dn9,)
    }
};
        locals.var_expin = assign5020_e4789;
        locals.var_expin_dn0 = assign5020_e4789_d_n0;
        locals.var_expin_dn1 = assign5020_e4789_d_n1;
        locals.var_expin_dn3 = assign5020_e4789_d_n3;
        locals.var_expin_dn4 = assign5020_e4789_d_n4;
        locals.var_expin_dn5 = assign5020_e4789_d_n5;
        locals.var_expin_dn6 = assign5020_e4789_d_n6;
        locals.var_expin_dn7 = assign5020_e4789_d_n7;
        locals.var_expin_dn8 = assign5020_e4789_d_n8;
        locals.var_expin_dn9 = assign5020_e4789_d_n9;

        let (assign5030_e4801, assign5030_e4801_d_n0, assign5030_e4801_d_n1, assign5030_e4801_d_n3, assign5030_e4801_d_n4, assign5030_e4801_d_n5, assign5030_e4801_d_n6, assign5030_e4801_d_n7, assign5030_e4801_d_n8, assign5030_e4801_d_n9,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) && (locals.var_guard85 != 0.0)) {
        let assign5030_e4797: f64 = (p.p43 - locals.var_vb2c1);
        let assign5030_e4799: f64 = (assign5030_e4797 * locals.var_expin);
        (assign5030_e4799, (assign5030_e4797 * locals.var_expin_dn0), (assign5030_e4797 * locals.var_expin_dn1), (assign5030_e4797 * locals.var_expin_dn3), (assign5030_e4797 * locals.var_expin_dn4), (((-locals.var_vb2c1_dn5) * locals.var_expin) + (assign5030_e4797 * locals.var_expin_dn5)), (((-locals.var_vb2c1_dn6) * locals.var_expin) + (assign5030_e4797 * locals.var_expin_dn6)), (assign5030_e4797 * locals.var_expin_dn7), (assign5030_e4797 * locals.var_expin_dn8), (assign5030_e4797 * locals.var_expin_dn9),)
    } else {
        (locals.var_vl, locals.var_vl_dn0, locals.var_vl_dn1, locals.var_vl_dn3, locals.var_vl_dn4, locals.var_vl_dn5, locals.var_vl_dn6, locals.var_vl_dn7, locals.var_vl_dn8, locals.var_vl_dn9,)
    }
};
        locals.var_vl = assign5030_e4801;
        locals.var_vl_dn0 = assign5030_e4801_d_n0;
        locals.var_vl_dn1 = assign5030_e4801_d_n1;
        locals.var_vl_dn3 = assign5030_e4801_d_n3;
        locals.var_vl_dn4 = assign5030_e4801_d_n4;
        locals.var_vl_dn5 = assign5030_e4801_d_n5;
        locals.var_vl_dn6 = assign5030_e4801_d_n6;
        locals.var_vl_dn7 = assign5030_e4801_d_n7;
        locals.var_vl_dn8 = assign5030_e4801_d_n8;
        locals.var_vl_dn9 = assign5030_e4801_d_n9;

        let assign5040_e4803: f64 = (-locals.var_bavl_t);
        let assign5040_e4806: f64 = (locals.var_vl).powf(p.p40);
        let assign5040_e4807: f64 = (assign5040_e4803 * assign5040_e4806);
        let assign5040_e4809: f64 = if assign5040_e4807 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign5040_e4809;

        let (assign5050_e4825, assign5050_e4825_d_n0, assign5050_e4825_d_n1, assign5050_e4825_d_n3, assign5050_e4825_d_n4, assign5050_e4825_d_n5, assign5050_e4825_d_n6, assign5050_e4825_d_n7, assign5050_e4825_d_n8, assign5050_e4825_d_n9,) = {
    if ((((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard87 != 0.0)) {
        let assign5050_e4818: f64 = (-locals.var_bavl_t);
        let assign5050_e4821: f64 = (locals.var_vl).powf(p.p40);
        let assign5050_e4822: f64 = (assign5050_e4818 * assign5050_e4821);
        let assign5050_e4823: f64 = (assign5050_e4822).exp();
        (assign5050_e4823, (assign5050_e4823 * (((-locals.var_bavl_t_dn0) * assign5050_e4821) + (assign5050_e4818 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn0)) } } else { (assign5050_e4821 * (p.p40 * (locals.var_vl_dn0 / locals.var_vl))) }))), (assign5050_e4823 * (((-locals.var_bavl_t_dn1) * assign5050_e4821) + (assign5050_e4818 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn1)) } } else { (assign5050_e4821 * (p.p40 * (locals.var_vl_dn1 / locals.var_vl))) }))), (assign5050_e4823 * (((-locals.var_bavl_t_dn3) * assign5050_e4821) + (assign5050_e4818 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn3)) } } else { (assign5050_e4821 * (p.p40 * (locals.var_vl_dn3 / locals.var_vl))) }))), (assign5050_e4823 * (((-locals.var_bavl_t_dn4) * assign5050_e4821) + (assign5050_e4818 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn4)) } } else { (assign5050_e4821 * (p.p40 * (locals.var_vl_dn4 / locals.var_vl))) }))), (assign5050_e4823 * (((-locals.var_bavl_t_dn5) * assign5050_e4821) + (assign5050_e4818 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn5)) } } else { (assign5050_e4821 * (p.p40 * (locals.var_vl_dn5 / locals.var_vl))) }))), (assign5050_e4823 * (((-locals.var_bavl_t_dn6) * assign5050_e4821) + (assign5050_e4818 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn6)) } } else { (assign5050_e4821 * (p.p40 * (locals.var_vl_dn6 / locals.var_vl))) }))), (assign5050_e4823 * (((-locals.var_bavl_t_dn7) * assign5050_e4821) + (assign5050_e4818 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn7)) } } else { (assign5050_e4821 * (p.p40 * (locals.var_vl_dn7 / locals.var_vl))) }))), (assign5050_e4823 * (((-locals.var_bavl_t_dn8) * assign5050_e4821) + (assign5050_e4818 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn8)) } } else { (assign5050_e4821 * (p.p40 * (locals.var_vl_dn8 / locals.var_vl))) }))), (assign5050_e4823 * (((-locals.var_bavl_t_dn9) * assign5050_e4821) + (assign5050_e4818 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn9)) } } else { (assign5050_e4821 * (p.p40 * (locals.var_vl_dn9 / locals.var_vl))) }))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9,)
    }
};
        locals.var_expmm1 = assign5050_e4825;
        locals.var_expmm1_dn0 = assign5050_e4825_d_n0;
        locals.var_expmm1_dn1 = assign5050_e4825_d_n1;
        locals.var_expmm1_dn3 = assign5050_e4825_d_n3;
        locals.var_expmm1_dn4 = assign5050_e4825_d_n4;
        locals.var_expmm1_dn5 = assign5050_e4825_d_n5;
        locals.var_expmm1_dn6 = assign5050_e4825_d_n6;
        locals.var_expmm1_dn7 = assign5050_e4825_d_n7;
        locals.var_expmm1_dn8 = assign5050_e4825_d_n8;
        locals.var_expmm1_dn9 = assign5050_e4825_d_n9;

        let (assign5060_e4837,) = {
    if ((((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard87 == 0.0)) {
        let assign5060_e4835: f64 = (p.p134).exp();
        (assign5060_e4835,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5060_e4837;

        let (assign5070_e4859, assign5070_e4859_d_n0, assign5070_e4859_d_n1, assign5070_e4859_d_n3, assign5070_e4859_d_n4, assign5070_e4859_d_n5, assign5070_e4859_d_n6, assign5070_e4859_d_n7, assign5070_e4859_d_n8, assign5070_e4859_d_n9,) = {
    if ((((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard87 == 0.0)) {
        let assign5070_e4849: f64 = (-locals.var_bavl_t);
        let assign5070_e4852: f64 = (locals.var_vl).powf(p.p40);
        let assign5070_e4853: f64 = (assign5070_e4849 * assign5070_e4852);
        let assign5070_e4855: f64 = (assign5070_e4853 - p.p134);
        let assign5070_e4856: f64 = (1.0 + assign5070_e4855);
        let assign5070_e4857: f64 = (locals.var_expl * assign5070_e4856);
        (assign5070_e4857, (locals.var_expl * (((-locals.var_bavl_t_dn0) * assign5070_e4852) + (assign5070_e4849 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn0)) } } else { (assign5070_e4852 * (p.p40 * (locals.var_vl_dn0 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn1) * assign5070_e4852) + (assign5070_e4849 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn1)) } } else { (assign5070_e4852 * (p.p40 * (locals.var_vl_dn1 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn3) * assign5070_e4852) + (assign5070_e4849 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn3)) } } else { (assign5070_e4852 * (p.p40 * (locals.var_vl_dn3 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn4) * assign5070_e4852) + (assign5070_e4849 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn4)) } } else { (assign5070_e4852 * (p.p40 * (locals.var_vl_dn4 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn5) * assign5070_e4852) + (assign5070_e4849 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn5)) } } else { (assign5070_e4852 * (p.p40 * (locals.var_vl_dn5 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn6) * assign5070_e4852) + (assign5070_e4849 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn6)) } } else { (assign5070_e4852 * (p.p40 * (locals.var_vl_dn6 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn7) * assign5070_e4852) + (assign5070_e4849 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn7)) } } else { (assign5070_e4852 * (p.p40 * (locals.var_vl_dn7 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn8) * assign5070_e4852) + (assign5070_e4849 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn8)) } } else { (assign5070_e4852 * (p.p40 * (locals.var_vl_dn8 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn9) * assign5070_e4852) + (assign5070_e4849 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn9)) } } else { (assign5070_e4852 * (p.p40 * (locals.var_vl_dn9 / locals.var_vl))) }))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9,)
    }
};
        locals.var_expmm1 = assign5070_e4859;
        locals.var_expmm1_dn0 = assign5070_e4859_d_n0;
        locals.var_expmm1_dn1 = assign5070_e4859_d_n1;
        locals.var_expmm1_dn3 = assign5070_e4859_d_n3;
        locals.var_expmm1_dn4 = assign5070_e4859_d_n4;
        locals.var_expmm1_dn5 = assign5070_e4859_d_n5;
        locals.var_expmm1_dn6 = assign5070_e4859_d_n6;
        locals.var_expmm1_dn7 = assign5070_e4859_d_n7;
        locals.var_expmm1_dn8 = assign5070_e4859_d_n8;
        locals.var_expmm1_dn9 = assign5070_e4859_d_n9;

        let (assign5080_e4873, assign5080_e4873_d_n0, assign5080_e4873_d_n1, assign5080_e4873_d_n3, assign5080_e4873_d_n4, assign5080_e4873_d_n5, assign5080_e4873_d_n6, assign5080_e4873_d_n7, assign5080_e4873_d_n8, assign5080_e4873_d_n9,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) && (locals.var_guard85 != 0.0)) {
        let assign5080_e4867: f64 = (p.p39 / locals.var_bavl_t);
        let assign5080_e4869: f64 = (assign5080_e4867 * locals.var_vl);
        let assign5080_e4871: f64 = (assign5080_e4869 * locals.var_expmm1);
        (assign5080_e4871, (((((-((p.p39 * locals.var_bavl_t_dn0) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5080_e4867 * locals.var_vl_dn0)) * locals.var_expmm1) + (assign5080_e4869 * locals.var_expmm1_dn0)), (((((-((p.p39 * locals.var_bavl_t_dn1) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5080_e4867 * locals.var_vl_dn1)) * locals.var_expmm1) + (assign5080_e4869 * locals.var_expmm1_dn1)), (((((-((p.p39 * locals.var_bavl_t_dn3) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5080_e4867 * locals.var_vl_dn3)) * locals.var_expmm1) + (assign5080_e4869 * locals.var_expmm1_dn3)), (((((-((p.p39 * locals.var_bavl_t_dn4) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5080_e4867 * locals.var_vl_dn4)) * locals.var_expmm1) + (assign5080_e4869 * locals.var_expmm1_dn4)), (((((-((p.p39 * locals.var_bavl_t_dn5) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5080_e4867 * locals.var_vl_dn5)) * locals.var_expmm1) + (assign5080_e4869 * locals.var_expmm1_dn5)), (((((-((p.p39 * locals.var_bavl_t_dn6) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5080_e4867 * locals.var_vl_dn6)) * locals.var_expmm1) + (assign5080_e4869 * locals.var_expmm1_dn6)), (((((-((p.p39 * locals.var_bavl_t_dn7) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5080_e4867 * locals.var_vl_dn7)) * locals.var_expmm1) + (assign5080_e4869 * locals.var_expmm1_dn7)), (((((-((p.p39 * locals.var_bavl_t_dn8) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5080_e4867 * locals.var_vl_dn8)) * locals.var_expmm1) + (assign5080_e4869 * locals.var_expmm1_dn8)), (((((-((p.p39 * locals.var_bavl_t_dn9) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5080_e4867 * locals.var_vl_dn9)) * locals.var_expmm1) + (assign5080_e4869 * locals.var_expmm1_dn9)),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9,)
    }
};
        locals.var_gem = assign5080_e4873;
        locals.var_gem_dn0 = assign5080_e4873_d_n0;
        locals.var_gem_dn1 = assign5080_e4873_d_n1;
        locals.var_gem_dn3 = assign5080_e4873_d_n3;
        locals.var_gem_dn4 = assign5080_e4873_d_n4;
        locals.var_gem_dn5 = assign5080_e4873_d_n5;
        locals.var_gem_dn6 = assign5080_e4873_d_n6;
        locals.var_gem_dn7 = assign5080_e4873_d_n7;
        locals.var_gem_dn8 = assign5080_e4873_d_n8;
        locals.var_gem_dn9 = assign5080_e4873_d_n9;

        let assign5090_e4876: f64 = if p.p38 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard88 = assign5090_e4876;

        let assign5100_e4879: f64 = if locals.var_vb2c1 < locals.var_vdc_t { 1.0 } else { 0.0 };
        locals.var_guard89 = assign5100_e4879;

        let (assign5110_e4896,) = {
    if ((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5110_e4890: f64 = (2.0 * p.p45);
        let assign5110_e4893: f64 = (p.p44 * p.p44);
        let assign5110_e4894: f64 = (assign5110_e4890 / assign5110_e4893);
        (assign5110_e4894,)
    } else {
        (locals.var_dedx0,)
    }
};
        locals.var_dedx0 = assign5110_e4896;

        let (assign5120_e4911, assign5120_e4911_d_n0, assign5120_e4911_d_n1, assign5120_e4911_d_n3, assign5120_e4911_d_n4, assign5120_e4911_d_n5, assign5120_e4911_d_n6, assign5120_e4911_d_n7, assign5120_e4911_d_n8, assign5120_e4911_d_n9,) = {
    if ((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5120_e4907: f64 = (locals.var_vdc_t - locals.var_vb2c1);
        let assign5120_e4909: f64 = (assign5120_e4907 / locals.var_icap_ihc);
        (assign5120_e4909, (((locals.var_vdc_t_dn0 * locals.var_icap_ihc) - (assign5120_e4907 * locals.var_icap_ihc_dn0)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn1 * locals.var_icap_ihc) - (assign5120_e4907 * locals.var_icap_ihc_dn1)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn3 * locals.var_icap_ihc) - (assign5120_e4907 * locals.var_icap_ihc_dn3)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn4 * locals.var_icap_ihc) - (assign5120_e4907 * locals.var_icap_ihc_dn4)) / (locals.var_icap_ihc * locals.var_icap_ihc)), ((((locals.var_vdc_t_dn5 - locals.var_vb2c1_dn5) * locals.var_icap_ihc) - (assign5120_e4907 * locals.var_icap_ihc_dn5)) / (locals.var_icap_ihc * locals.var_icap_ihc)), ((((locals.var_vdc_t_dn6 - locals.var_vb2c1_dn6) * locals.var_icap_ihc) - (assign5120_e4907 * locals.var_icap_ihc_dn6)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn7 * locals.var_icap_ihc) - (assign5120_e4907 * locals.var_icap_ihc_dn7)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn8 * locals.var_icap_ihc) - (assign5120_e4907 * locals.var_icap_ihc_dn8)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn9 * locals.var_icap_ihc) - (assign5120_e4907 * locals.var_icap_ihc_dn9)) / (locals.var_icap_ihc * locals.var_icap_ihc)),)
    } else {
        (locals.var_sqr_arg, locals.var_sqr_arg_dn0, locals.var_sqr_arg_dn1, locals.var_sqr_arg_dn3, locals.var_sqr_arg_dn4, locals.var_sqr_arg_dn5, locals.var_sqr_arg_dn6, locals.var_sqr_arg_dn7, locals.var_sqr_arg_dn8, locals.var_sqr_arg_dn9,)
    }
};
        locals.var_sqr_arg = assign5120_e4911;
        locals.var_sqr_arg_dn0 = assign5120_e4911_d_n0;
        locals.var_sqr_arg_dn1 = assign5120_e4911_d_n1;
        locals.var_sqr_arg_dn3 = assign5120_e4911_d_n3;
        locals.var_sqr_arg_dn4 = assign5120_e4911_d_n4;
        locals.var_sqr_arg_dn5 = assign5120_e4911_d_n5;
        locals.var_sqr_arg_dn6 = assign5120_e4911_d_n6;
        locals.var_sqr_arg_dn7 = assign5120_e4911_d_n7;
        locals.var_sqr_arg_dn8 = assign5120_e4911_d_n8;
        locals.var_sqr_arg_dn9 = assign5120_e4911_d_n9;

        let (assign5130_e4927, assign5130_e4927_d_n0, assign5130_e4927_d_n1, assign5130_e4927_d_n3, assign5130_e4927_d_n4, assign5130_e4927_d_n5, assign5130_e4927_d_n6, assign5130_e4927_d_n7, assign5130_e4927_d_n8, assign5130_e4927_d_n9,) = {
    if ((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5130_e4922: f64 = (2.0 * locals.var_sqr_arg);
        let assign5130_e4924: f64 = (assign5130_e4922 / locals.var_dedx0);
        let assign5130_e4925: f64 = (assign5130_e4924).sqrt();
        (assign5130_e4925, (((2.0 * locals.var_sqr_arg_dn0) / locals.var_dedx0) / (2.0 * assign5130_e4925)), (((2.0 * locals.var_sqr_arg_dn1) / locals.var_dedx0) / (2.0 * assign5130_e4925)), (((2.0 * locals.var_sqr_arg_dn3) / locals.var_dedx0) / (2.0 * assign5130_e4925)), (((2.0 * locals.var_sqr_arg_dn4) / locals.var_dedx0) / (2.0 * assign5130_e4925)), (((2.0 * locals.var_sqr_arg_dn5) / locals.var_dedx0) / (2.0 * assign5130_e4925)), (((2.0 * locals.var_sqr_arg_dn6) / locals.var_dedx0) / (2.0 * assign5130_e4925)), (((2.0 * locals.var_sqr_arg_dn7) / locals.var_dedx0) / (2.0 * assign5130_e4925)), (((2.0 * locals.var_sqr_arg_dn8) / locals.var_dedx0) / (2.0 * assign5130_e4925)), (((2.0 * locals.var_sqr_arg_dn9) / locals.var_dedx0) / (2.0 * assign5130_e4925)),)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn1, locals.var_xd_dn3, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9,)
    }
};
        locals.var_xd = assign5130_e4927;
        locals.var_xd_dn0 = assign5130_e4927_d_n0;
        locals.var_xd_dn1 = assign5130_e4927_d_n1;
        locals.var_xd_dn3 = assign5130_e4927_d_n3;
        locals.var_xd_dn4 = assign5130_e4927_d_n4;
        locals.var_xd_dn5 = assign5130_e4927_d_n5;
        locals.var_xd_dn6 = assign5130_e4927_d_n6;
        locals.var_xd_dn7 = assign5130_e4927_d_n7;
        locals.var_xd_dn8 = assign5130_e4927_d_n8;
        locals.var_xd_dn9 = assign5130_e4927_d_n9;

        let assign5140_e4930: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign5140_e4930;

        let (assign5150_e4943, assign5150_e4943_d_n0, assign5150_e4943_d_n1, assign5150_e4943_d_n3, assign5150_e4943_d_n4, assign5150_e4943_d_n5, assign5150_e4943_d_n6, assign5150_e4943_d_n7, assign5150_e4943_d_n8, assign5150_e4943_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) && (locals.var_guard90 != 0.0)) {
        (p.p44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_weff, locals.var_weff_dn0, locals.var_weff_dn1, locals.var_weff_dn3, locals.var_weff_dn4, locals.var_weff_dn5, locals.var_weff_dn6, locals.var_weff_dn7, locals.var_weff_dn8, locals.var_weff_dn9,)
    }
};
        locals.var_weff = assign5150_e4943;
        locals.var_weff_dn0 = assign5150_e4943_d_n0;
        locals.var_weff_dn1 = assign5150_e4943_d_n1;
        locals.var_weff_dn3 = assign5150_e4943_d_n3;
        locals.var_weff_dn4 = assign5150_e4943_d_n4;
        locals.var_weff_dn5 = assign5150_e4943_d_n5;
        locals.var_weff_dn6 = assign5150_e4943_d_n6;
        locals.var_weff_dn7 = assign5150_e4943_d_n7;
        locals.var_weff_dn8 = assign5150_e4943_d_n8;
        locals.var_weff_dn9 = assign5150_e4943_d_n9;

        let (assign5160_e4961, assign5160_e4961_d_n0, assign5160_e4961_d_n1, assign5160_e4961_d_n3, assign5160_e4961_d_n4, assign5160_e4961_d_n5, assign5160_e4961_d_n6, assign5160_e4961_d_n7, assign5160_e4961_d_n8, assign5160_e4961_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) && (locals.var_guard90 == 0.0)) {
        let assign5160_e4958: f64 = (0.5 * locals.var_xi_w);
        let assign5160_e4959: f64 = (1.0 - assign5160_e4958);
        (assign5160_e4959, (-(0.5 * locals.var_xi_w_dn0)), (-(0.5 * locals.var_xi_w_dn1)), (-(0.5 * locals.var_xi_w_dn3)), (-(0.5 * locals.var_xi_w_dn4)), (-(0.5 * locals.var_xi_w_dn5)), (-(0.5 * locals.var_xi_w_dn6)), (-(0.5 * locals.var_xi_w_dn7)), (-(0.5 * locals.var_xi_w_dn8)), (-(0.5 * locals.var_xi_w_dn9)),)
    } else {
        (locals.var_xi_w1, locals.var_xi_w1_dn0, locals.var_xi_w1_dn1, locals.var_xi_w1_dn3, locals.var_xi_w1_dn4, locals.var_xi_w1_dn5, locals.var_xi_w1_dn6, locals.var_xi_w1_dn7, locals.var_xi_w1_dn8, locals.var_xi_w1_dn9,)
    }
};
        locals.var_xi_w1 = assign5160_e4961;
        locals.var_xi_w1_dn0 = assign5160_e4961_d_n0;
        locals.var_xi_w1_dn1 = assign5160_e4961_d_n1;
        locals.var_xi_w1_dn3 = assign5160_e4961_d_n3;
        locals.var_xi_w1_dn4 = assign5160_e4961_d_n4;
        locals.var_xi_w1_dn5 = assign5160_e4961_d_n5;
        locals.var_xi_w1_dn6 = assign5160_e4961_d_n6;
        locals.var_xi_w1_dn7 = assign5160_e4961_d_n7;
        locals.var_xi_w1_dn8 = assign5160_e4961_d_n8;
        locals.var_xi_w1_dn9 = assign5160_e4961_d_n9;

        let (assign5170_e4979, assign5170_e4979_d_n0, assign5170_e4979_d_n1, assign5170_e4979_d_n3, assign5170_e4979_d_n4, assign5170_e4979_d_n5, assign5170_e4979_d_n6, assign5170_e4979_d_n7, assign5170_e4979_d_n8, assign5170_e4979_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) && (locals.var_guard90 == 0.0)) {
        let assign5170_e4975: f64 = (p.p44 * locals.var_xi_w1);
        let assign5170_e4977: f64 = (assign5170_e4975 * locals.var_xi_w1);
        (assign5170_e4977, (((p.p44 * locals.var_xi_w1_dn0) * locals.var_xi_w1) + (assign5170_e4975 * locals.var_xi_w1_dn0)), (((p.p44 * locals.var_xi_w1_dn1) * locals.var_xi_w1) + (assign5170_e4975 * locals.var_xi_w1_dn1)), (((p.p44 * locals.var_xi_w1_dn3) * locals.var_xi_w1) + (assign5170_e4975 * locals.var_xi_w1_dn3)), (((p.p44 * locals.var_xi_w1_dn4) * locals.var_xi_w1) + (assign5170_e4975 * locals.var_xi_w1_dn4)), (((p.p44 * locals.var_xi_w1_dn5) * locals.var_xi_w1) + (assign5170_e4975 * locals.var_xi_w1_dn5)), (((p.p44 * locals.var_xi_w1_dn6) * locals.var_xi_w1) + (assign5170_e4975 * locals.var_xi_w1_dn6)), (((p.p44 * locals.var_xi_w1_dn7) * locals.var_xi_w1) + (assign5170_e4975 * locals.var_xi_w1_dn7)), (((p.p44 * locals.var_xi_w1_dn8) * locals.var_xi_w1) + (assign5170_e4975 * locals.var_xi_w1_dn8)), (((p.p44 * locals.var_xi_w1_dn9) * locals.var_xi_w1) + (assign5170_e4975 * locals.var_xi_w1_dn9)),)
    } else {
        (locals.var_weff, locals.var_weff_dn0, locals.var_weff_dn1, locals.var_weff_dn3, locals.var_weff_dn4, locals.var_weff_dn5, locals.var_weff_dn6, locals.var_weff_dn7, locals.var_weff_dn8, locals.var_weff_dn9,)
    }
};
        locals.var_weff = assign5170_e4979;
        locals.var_weff_dn0 = assign5170_e4979_d_n0;
        locals.var_weff_dn1 = assign5170_e4979_d_n1;
        locals.var_weff_dn3 = assign5170_e4979_d_n3;
        locals.var_weff_dn4 = assign5170_e4979_d_n4;
        locals.var_weff_dn5 = assign5170_e4979_d_n5;
        locals.var_weff_dn6 = assign5170_e4979_d_n6;
        locals.var_weff_dn7 = assign5170_e4979_d_n7;
        locals.var_weff_dn8 = assign5170_e4979_d_n8;
        locals.var_weff_dn9 = assign5170_e4979_d_n9;

        let (assign5180_e5001, assign5180_e5001_d_n0, assign5180_e5001_d_n1, assign5180_e5001_d_n3, assign5180_e5001_d_n4, assign5180_e5001_d_n5, assign5180_e5001_d_n6, assign5180_e5001_d_n7, assign5180_e5001_d_n8, assign5180_e5001_d_n9,) = {
    if ((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5180_e4990: f64 = (locals.var_xd * locals.var_weff);
        let assign5180_e4993: f64 = (locals.var_xd * locals.var_xd);
        let assign5180_e4996: f64 = (locals.var_weff * locals.var_weff);
        let assign5180_e4997: f64 = (assign5180_e4993 + assign5180_e4996);
        let assign5180_e4998: f64 = (assign5180_e4997).sqrt();
        let assign5180_e4999: f64 = (assign5180_e4990 / assign5180_e4998);
        (assign5180_e4999, (((((locals.var_xd_dn0 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn0)) * assign5180_e4998) - (assign5180_e4990 * ((((locals.var_xd_dn0 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn0)) + ((locals.var_weff_dn0 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn0))) / (2.0 * assign5180_e4998)))) / (assign5180_e4998 * assign5180_e4998)), (((((locals.var_xd_dn1 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn1)) * assign5180_e4998) - (assign5180_e4990 * ((((locals.var_xd_dn1 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn1)) + ((locals.var_weff_dn1 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn1))) / (2.0 * assign5180_e4998)))) / (assign5180_e4998 * assign5180_e4998)), (((((locals.var_xd_dn3 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn3)) * assign5180_e4998) - (assign5180_e4990 * ((((locals.var_xd_dn3 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn3)) + ((locals.var_weff_dn3 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn3))) / (2.0 * assign5180_e4998)))) / (assign5180_e4998 * assign5180_e4998)), (((((locals.var_xd_dn4 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn4)) * assign5180_e4998) - (assign5180_e4990 * ((((locals.var_xd_dn4 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn4)) + ((locals.var_weff_dn4 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn4))) / (2.0 * assign5180_e4998)))) / (assign5180_e4998 * assign5180_e4998)), (((((locals.var_xd_dn5 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn5)) * assign5180_e4998) - (assign5180_e4990 * ((((locals.var_xd_dn5 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn5)) + ((locals.var_weff_dn5 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn5))) / (2.0 * assign5180_e4998)))) / (assign5180_e4998 * assign5180_e4998)), (((((locals.var_xd_dn6 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn6)) * assign5180_e4998) - (assign5180_e4990 * ((((locals.var_xd_dn6 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn6)) + ((locals.var_weff_dn6 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn6))) / (2.0 * assign5180_e4998)))) / (assign5180_e4998 * assign5180_e4998)), (((((locals.var_xd_dn7 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn7)) * assign5180_e4998) - (assign5180_e4990 * ((((locals.var_xd_dn7 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn7)) + ((locals.var_weff_dn7 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn7))) / (2.0 * assign5180_e4998)))) / (assign5180_e4998 * assign5180_e4998)), (((((locals.var_xd_dn8 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn8)) * assign5180_e4998) - (assign5180_e4990 * ((((locals.var_xd_dn8 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn8)) + ((locals.var_weff_dn8 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn8))) / (2.0 * assign5180_e4998)))) / (assign5180_e4998 * assign5180_e4998)), (((((locals.var_xd_dn9 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn9)) * assign5180_e4998) - (assign5180_e4990 * ((((locals.var_xd_dn9 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn9)) + ((locals.var_weff_dn9 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn9))) / (2.0 * assign5180_e4998)))) / (assign5180_e4998 * assign5180_e4998)),)
    } else {
        (locals.var_wd, locals.var_wd_dn0, locals.var_wd_dn1, locals.var_wd_dn3, locals.var_wd_dn4, locals.var_wd_dn5, locals.var_wd_dn6, locals.var_wd_dn7, locals.var_wd_dn8, locals.var_wd_dn9,)
    }
};
        locals.var_wd = assign5180_e5001;
        locals.var_wd_dn0 = assign5180_e5001_d_n0;
        locals.var_wd_dn1 = assign5180_e5001_d_n1;
        locals.var_wd_dn3 = assign5180_e5001_d_n3;
        locals.var_wd_dn4 = assign5180_e5001_d_n4;
        locals.var_wd_dn5 = assign5180_e5001_d_n5;
        locals.var_wd_dn6 = assign5180_e5001_d_n6;
        locals.var_wd_dn7 = assign5180_e5001_d_n7;
        locals.var_wd_dn8 = assign5180_e5001_d_n8;
        locals.var_wd_dn9 = assign5180_e5001_d_n9;

        let (assign5190_e5016, assign5190_e5016_d_n0, assign5190_e5016_d_n1, assign5190_e5016_d_n3, assign5190_e5016_d_n4, assign5190_e5016_d_n5, assign5190_e5016_d_n6, assign5190_e5016_d_n7, assign5190_e5016_d_n8, assign5190_e5016_d_n9,) = {
    if ((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5190_e5012: f64 = (locals.var_vdc_t - locals.var_vb2c1);
        let assign5190_e5014: f64 = (assign5190_e5012 / locals.var_wd);
        (assign5190_e5014, (((locals.var_vdc_t_dn0 * locals.var_wd) - (assign5190_e5012 * locals.var_wd_dn0)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn1 * locals.var_wd) - (assign5190_e5012 * locals.var_wd_dn1)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn3 * locals.var_wd) - (assign5190_e5012 * locals.var_wd_dn3)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn4 * locals.var_wd) - (assign5190_e5012 * locals.var_wd_dn4)) / (locals.var_wd * locals.var_wd)), ((((locals.var_vdc_t_dn5 - locals.var_vb2c1_dn5) * locals.var_wd) - (assign5190_e5012 * locals.var_wd_dn5)) / (locals.var_wd * locals.var_wd)), ((((locals.var_vdc_t_dn6 - locals.var_vb2c1_dn6) * locals.var_wd) - (assign5190_e5012 * locals.var_wd_dn6)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn7 * locals.var_wd) - (assign5190_e5012 * locals.var_wd_dn7)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn8 * locals.var_wd) - (assign5190_e5012 * locals.var_wd_dn8)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn9 * locals.var_wd) - (assign5190_e5012 * locals.var_wd_dn9)) / (locals.var_wd * locals.var_wd)),)
    } else {
        (locals.var_eav, locals.var_eav_dn0, locals.var_eav_dn1, locals.var_eav_dn3, locals.var_eav_dn4, locals.var_eav_dn5, locals.var_eav_dn6, locals.var_eav_dn7, locals.var_eav_dn8, locals.var_eav_dn9,)
    }
};
        locals.var_eav = assign5190_e5016;
        locals.var_eav_dn0 = assign5190_e5016_d_n0;
        locals.var_eav_dn1 = assign5190_e5016_d_n1;
        locals.var_eav_dn3 = assign5190_e5016_d_n3;
        locals.var_eav_dn4 = assign5190_e5016_d_n4;
        locals.var_eav_dn5 = assign5190_e5016_d_n5;
        locals.var_eav_dn6 = assign5190_e5016_d_n6;
        locals.var_eav_dn7 = assign5190_e5016_d_n7;
        locals.var_eav_dn8 = assign5190_e5016_d_n8;
        locals.var_eav_dn9 = assign5190_e5016_d_n9;

        let (assign5200_e5035, assign5200_e5035_d_n0, assign5200_e5035_d_n1, assign5200_e5035_d_n3, assign5200_e5035_d_n4, assign5200_e5035_d_n5, assign5200_e5035_d_n6, assign5200_e5035_d_n7, assign5200_e5035_d_n8, assign5200_e5035_d_n9,) = {
    if ((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5200_e5028: f64 = (0.5 * locals.var_wd);
        let assign5200_e5030: f64 = (assign5200_e5028 * locals.var_dedx0);
        let assign5200_e5032: f64 = (assign5200_e5030 * locals.var_icap_ihc);
        let assign5200_e5033: f64 = (locals.var_eav + assign5200_e5032);
        (assign5200_e5033, (locals.var_eav_dn0 + ((((0.5 * locals.var_wd_dn0) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5200_e5030 * locals.var_icap_ihc_dn0))), (locals.var_eav_dn1 + ((((0.5 * locals.var_wd_dn1) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5200_e5030 * locals.var_icap_ihc_dn1))), (locals.var_eav_dn3 + ((((0.5 * locals.var_wd_dn3) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5200_e5030 * locals.var_icap_ihc_dn3))), (locals.var_eav_dn4 + ((((0.5 * locals.var_wd_dn4) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5200_e5030 * locals.var_icap_ihc_dn4))), (locals.var_eav_dn5 + ((((0.5 * locals.var_wd_dn5) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5200_e5030 * locals.var_icap_ihc_dn5))), (locals.var_eav_dn6 + ((((0.5 * locals.var_wd_dn6) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5200_e5030 * locals.var_icap_ihc_dn6))), (locals.var_eav_dn7 + ((((0.5 * locals.var_wd_dn7) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5200_e5030 * locals.var_icap_ihc_dn7))), (locals.var_eav_dn8 + ((((0.5 * locals.var_wd_dn8) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5200_e5030 * locals.var_icap_ihc_dn8))), (locals.var_eav_dn9 + ((((0.5 * locals.var_wd_dn9) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5200_e5030 * locals.var_icap_ihc_dn9))),)
    } else {
        (locals.var_e0, locals.var_e0_dn0, locals.var_e0_dn1, locals.var_e0_dn3, locals.var_e0_dn4, locals.var_e0_dn5, locals.var_e0_dn6, locals.var_e0_dn7, locals.var_e0_dn8, locals.var_e0_dn9,)
    }
};
        locals.var_e0 = assign5200_e5035;
        locals.var_e0_dn0 = assign5200_e5035_d_n0;
        locals.var_e0_dn1 = assign5200_e5035_d_n1;
        locals.var_e0_dn3 = assign5200_e5035_d_n3;
        locals.var_e0_dn4 = assign5200_e5035_d_n4;
        locals.var_e0_dn5 = assign5200_e5035_d_n5;
        locals.var_e0_dn6 = assign5200_e5035_d_n6;
        locals.var_e0_dn7 = assign5200_e5035_d_n7;
        locals.var_e0_dn8 = assign5200_e5035_d_n8;
        locals.var_e0_dn9 = assign5200_e5035_d_n9;

        let assign5210_e5038: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard91 = assign5210_e5038;

        let (assign5220_e5051, assign5220_e5051_d_n0, assign5220_e5051_d_n1, assign5220_e5051_d_n3, assign5220_e5051_d_n4, assign5220_e5051_d_n5, assign5220_e5051_d_n6, assign5220_e5051_d_n7, assign5220_e5051_d_n8, assign5220_e5051_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) && (locals.var_guard91 != 0.0)) {
        (locals.var_e0, locals.var_e0_dn0, locals.var_e0_dn1, locals.var_e0_dn3, locals.var_e0_dn4, locals.var_e0_dn5, locals.var_e0_dn6, locals.var_e0_dn7, locals.var_e0_dn8, locals.var_e0_dn9,)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn1, locals.var_em_dn3, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9,)
    }
};
        locals.var_em = assign5220_e5051;
        locals.var_em_dn0 = assign5220_e5051_d_n0;
        locals.var_em_dn1 = assign5220_e5051_d_n1;
        locals.var_em_dn3 = assign5220_e5051_d_n3;
        locals.var_em_dn4 = assign5220_e5051_d_n4;
        locals.var_em_dn5 = assign5220_e5051_d_n5;
        locals.var_em_dn6 = assign5220_e5051_d_n6;
        locals.var_em_dn7 = assign5220_e5051_d_n7;
        locals.var_em_dn8 = assign5220_e5051_d_n8;
        locals.var_em_dn9 = assign5220_e5051_d_n9;

        let (assign5230_e5075, assign5230_e5075_d_n0, assign5230_e5075_d_n1, assign5230_e5075_d_n3, assign5230_e5075_d_n4, assign5230_e5075_d_n5, assign5230_e5075_d_n6, assign5230_e5075_d_n7, assign5230_e5075_d_n8, assign5230_e5075_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) && (locals.var_guard91 == 0.0)) {
        let assign5230_e5066: f64 = (2.0 * p.p46);
        let assign5230_e5070: f64 = (2.0 * locals.var_xi_w);
        let assign5230_e5071: f64 = (1.0 + assign5230_e5070);
        let assign5230_e5072: f64 = (assign5230_e5066 * assign5230_e5071);
        let assign5230_e5073: f64 = (1.0 + assign5230_e5072);
        (assign5230_e5073, (assign5230_e5066 * (2.0 * locals.var_xi_w_dn0)), (assign5230_e5066 * (2.0 * locals.var_xi_w_dn1)), (assign5230_e5066 * (2.0 * locals.var_xi_w_dn3)), (assign5230_e5066 * (2.0 * locals.var_xi_w_dn4)), (assign5230_e5066 * (2.0 * locals.var_xi_w_dn5)), (assign5230_e5066 * (2.0 * locals.var_xi_w_dn6)), (assign5230_e5066 * (2.0 * locals.var_xi_w_dn7)), (assign5230_e5066 * (2.0 * locals.var_xi_w_dn8)), (assign5230_e5066 * (2.0 * locals.var_xi_w_dn9)),)
    } else {
        (locals.var_shw, locals.var_shw_dn0, locals.var_shw_dn1, locals.var_shw_dn3, locals.var_shw_dn4, locals.var_shw_dn5, locals.var_shw_dn6, locals.var_shw_dn7, locals.var_shw_dn8, locals.var_shw_dn9,)
    }
};
        locals.var_shw = assign5230_e5075;
        locals.var_shw_dn0 = assign5230_e5075_d_n0;
        locals.var_shw_dn1 = assign5230_e5075_d_n1;
        locals.var_shw_dn3 = assign5230_e5075_d_n3;
        locals.var_shw_dn4 = assign5230_e5075_d_n4;
        locals.var_shw_dn5 = assign5230_e5075_d_n5;
        locals.var_shw_dn6 = assign5230_e5075_d_n6;
        locals.var_shw_dn7 = assign5230_e5075_d_n7;
        locals.var_shw_dn8 = assign5230_e5075_d_n8;
        locals.var_shw_dn9 = assign5230_e5075_d_n9;

        let (assign5240_e5097,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) && (locals.var_guard91 == 0.0)) {
        let assign5240_e5089: f64 = (1.0 + p.p46);
        let assign5240_e5093: f64 = (2.0 * p.p46);
        let assign5240_e5094: f64 = (1.0 + assign5240_e5093);
        let assign5240_e5095: f64 = (assign5240_e5089 / assign5240_e5094);
        (assign5240_e5095,)
    } else {
        (locals.var_efi,)
    }
};
        locals.var_efi = assign5240_e5097;

        let (assign5250_e5125, assign5250_e5125_d_n0, assign5250_e5125_d_n1, assign5250_e5125_d_n3, assign5250_e5125_d_n4, assign5250_e5125_d_n5, assign5250_e5125_d_n6, assign5250_e5125_d_n7, assign5250_e5125_d_n8, assign5250_e5125_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) && (locals.var_guard91 == 0.0)) {
        let assign5250_e5112: f64 = (0.5 * locals.var_wd);
        let assign5250_e5114: f64 = (assign5250_e5112 * locals.var_dedx0);
        let assign5250_e5119: f64 = (p.p61 * locals.var_shw);
        let assign5250_e5120: f64 = (locals.var_in_ / assign5250_e5119);
        let assign5250_e5121: f64 = (locals.var_efi - assign5250_e5120);
        let assign5250_e5122: f64 = (assign5250_e5114 * assign5250_e5121);
        let assign5250_e5123: f64 = (locals.var_eav - assign5250_e5122);
        (assign5250_e5123, (locals.var_eav_dn0 - ((((0.5 * locals.var_wd_dn0) * locals.var_dedx0) * assign5250_e5121) + (assign5250_e5114 * (-(((locals.var_in__dn0 * assign5250_e5119) - (locals.var_in_ * (p.p61 * locals.var_shw_dn0))) / (assign5250_e5119 * assign5250_e5119)))))), (locals.var_eav_dn1 - ((((0.5 * locals.var_wd_dn1) * locals.var_dedx0) * assign5250_e5121) + (assign5250_e5114 * (-(((locals.var_in__dn1 * assign5250_e5119) - (locals.var_in_ * (p.p61 * locals.var_shw_dn1))) / (assign5250_e5119 * assign5250_e5119)))))), (locals.var_eav_dn3 - ((((0.5 * locals.var_wd_dn3) * locals.var_dedx0) * assign5250_e5121) + (assign5250_e5114 * (-(((locals.var_in__dn3 * assign5250_e5119) - (locals.var_in_ * (p.p61 * locals.var_shw_dn3))) / (assign5250_e5119 * assign5250_e5119)))))), (locals.var_eav_dn4 - ((((0.5 * locals.var_wd_dn4) * locals.var_dedx0) * assign5250_e5121) + (assign5250_e5114 * (-(((locals.var_in__dn4 * assign5250_e5119) - (locals.var_in_ * (p.p61 * locals.var_shw_dn4))) / (assign5250_e5119 * assign5250_e5119)))))), (locals.var_eav_dn5 - ((((0.5 * locals.var_wd_dn5) * locals.var_dedx0) * assign5250_e5121) + (assign5250_e5114 * (-(((locals.var_in__dn5 * assign5250_e5119) - (locals.var_in_ * (p.p61 * locals.var_shw_dn5))) / (assign5250_e5119 * assign5250_e5119)))))), (locals.var_eav_dn6 - ((((0.5 * locals.var_wd_dn6) * locals.var_dedx0) * assign5250_e5121) + (assign5250_e5114 * (-(((locals.var_in__dn6 * assign5250_e5119) - (locals.var_in_ * (p.p61 * locals.var_shw_dn6))) / (assign5250_e5119 * assign5250_e5119)))))), (locals.var_eav_dn7 - ((((0.5 * locals.var_wd_dn7) * locals.var_dedx0) * assign5250_e5121) + (assign5250_e5114 * (-(((locals.var_in__dn7 * assign5250_e5119) - (locals.var_in_ * (p.p61 * locals.var_shw_dn7))) / (assign5250_e5119 * assign5250_e5119)))))), (locals.var_eav_dn8 - ((((0.5 * locals.var_wd_dn8) * locals.var_dedx0) * assign5250_e5121) + (assign5250_e5114 * (-(((locals.var_in__dn8 * assign5250_e5119) - (locals.var_in_ * (p.p61 * locals.var_shw_dn8))) / (assign5250_e5119 * assign5250_e5119)))))), (locals.var_eav_dn9 - ((((0.5 * locals.var_wd_dn9) * locals.var_dedx0) * assign5250_e5121) + (assign5250_e5114 * (-(((locals.var_in__dn9 * assign5250_e5119) - (locals.var_in_ * (p.p61 * locals.var_shw_dn9))) / (assign5250_e5119 * assign5250_e5119)))))),)
    } else {
        (locals.var_ew, locals.var_ew_dn0, locals.var_ew_dn1, locals.var_ew_dn3, locals.var_ew_dn4, locals.var_ew_dn5, locals.var_ew_dn6, locals.var_ew_dn7, locals.var_ew_dn8, locals.var_ew_dn9,)
    }
};
        locals.var_ew = assign5250_e5125;
        locals.var_ew_dn0 = assign5250_e5125_d_n0;
        locals.var_ew_dn1 = assign5250_e5125_d_n1;
        locals.var_ew_dn3 = assign5250_e5125_d_n3;
        locals.var_ew_dn4 = assign5250_e5125_d_n4;
        locals.var_ew_dn5 = assign5250_e5125_d_n5;
        locals.var_ew_dn6 = assign5250_e5125_d_n6;
        locals.var_ew_dn7 = assign5250_e5125_d_n7;
        locals.var_ew_dn8 = assign5250_e5125_d_n8;
        locals.var_ew_dn9 = assign5250_e5125_d_n9;

        let (assign5260_e5155, assign5260_e5155_d_n0, assign5260_e5155_d_n1, assign5260_e5155_d_n3, assign5260_e5155_d_n4, assign5260_e5155_d_n5, assign5260_e5155_d_n6, assign5260_e5155_d_n7, assign5260_e5155_d_n8, assign5260_e5155_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) && (locals.var_guard91 == 0.0)) {
        let assign5260_e5139: f64 = (locals.var_ew - locals.var_e0);
        let assign5260_e5142: f64 = (locals.var_ew - locals.var_e0);
        let assign5260_e5143: f64 = (assign5260_e5139 * assign5260_e5142);
        let assign5260_e5146: f64 = (0.1 * locals.var_eav);
        let assign5260_e5148: f64 = (assign5260_e5146 * locals.var_eav);
        let assign5260_e5150: f64 = (assign5260_e5148 * locals.var_icap);
        let assign5260_e5152: f64 = (assign5260_e5150 / p.p61);
        let assign5260_e5153: f64 = (assign5260_e5143 + assign5260_e5152);
        (assign5260_e5153, ((((locals.var_ew_dn0 - locals.var_e0_dn0) * assign5260_e5142) + (assign5260_e5139 * (locals.var_ew_dn0 - locals.var_e0_dn0))) + ((((((0.1 * locals.var_eav_dn0) * locals.var_eav) + (assign5260_e5146 * locals.var_eav_dn0)) * locals.var_icap) + (assign5260_e5148 * locals.var_icap_dn0)) / p.p61)), ((((locals.var_ew_dn1 - locals.var_e0_dn1) * assign5260_e5142) + (assign5260_e5139 * (locals.var_ew_dn1 - locals.var_e0_dn1))) + ((((((0.1 * locals.var_eav_dn1) * locals.var_eav) + (assign5260_e5146 * locals.var_eav_dn1)) * locals.var_icap) + (assign5260_e5148 * locals.var_icap_dn1)) / p.p61)), ((((locals.var_ew_dn3 - locals.var_e0_dn3) * assign5260_e5142) + (assign5260_e5139 * (locals.var_ew_dn3 - locals.var_e0_dn3))) + ((((((0.1 * locals.var_eav_dn3) * locals.var_eav) + (assign5260_e5146 * locals.var_eav_dn3)) * locals.var_icap) + (assign5260_e5148 * locals.var_icap_dn3)) / p.p61)), ((((locals.var_ew_dn4 - locals.var_e0_dn4) * assign5260_e5142) + (assign5260_e5139 * (locals.var_ew_dn4 - locals.var_e0_dn4))) + ((((((0.1 * locals.var_eav_dn4) * locals.var_eav) + (assign5260_e5146 * locals.var_eav_dn4)) * locals.var_icap) + (assign5260_e5148 * locals.var_icap_dn4)) / p.p61)), ((((locals.var_ew_dn5 - locals.var_e0_dn5) * assign5260_e5142) + (assign5260_e5139 * (locals.var_ew_dn5 - locals.var_e0_dn5))) + ((((((0.1 * locals.var_eav_dn5) * locals.var_eav) + (assign5260_e5146 * locals.var_eav_dn5)) * locals.var_icap) + (assign5260_e5148 * locals.var_icap_dn5)) / p.p61)), ((((locals.var_ew_dn6 - locals.var_e0_dn6) * assign5260_e5142) + (assign5260_e5139 * (locals.var_ew_dn6 - locals.var_e0_dn6))) + ((((((0.1 * locals.var_eav_dn6) * locals.var_eav) + (assign5260_e5146 * locals.var_eav_dn6)) * locals.var_icap) + (assign5260_e5148 * locals.var_icap_dn6)) / p.p61)), ((((locals.var_ew_dn7 - locals.var_e0_dn7) * assign5260_e5142) + (assign5260_e5139 * (locals.var_ew_dn7 - locals.var_e0_dn7))) + ((((((0.1 * locals.var_eav_dn7) * locals.var_eav) + (assign5260_e5146 * locals.var_eav_dn7)) * locals.var_icap) + (assign5260_e5148 * locals.var_icap_dn7)) / p.p61)), ((((locals.var_ew_dn8 - locals.var_e0_dn8) * assign5260_e5142) + (assign5260_e5139 * (locals.var_ew_dn8 - locals.var_e0_dn8))) + ((((((0.1 * locals.var_eav_dn8) * locals.var_eav) + (assign5260_e5146 * locals.var_eav_dn8)) * locals.var_icap) + (assign5260_e5148 * locals.var_icap_dn8)) / p.p61)), ((((locals.var_ew_dn9 - locals.var_e0_dn9) * assign5260_e5142) + (assign5260_e5139 * (locals.var_ew_dn9 - locals.var_e0_dn9))) + ((((((0.1 * locals.var_eav_dn9) * locals.var_eav) + (assign5260_e5146 * locals.var_eav_dn9)) * locals.var_icap) + (assign5260_e5148 * locals.var_icap_dn9)) / p.p61)),)
    } else {
        (locals.var_sqr_arg, locals.var_sqr_arg_dn0, locals.var_sqr_arg_dn1, locals.var_sqr_arg_dn3, locals.var_sqr_arg_dn4, locals.var_sqr_arg_dn5, locals.var_sqr_arg_dn6, locals.var_sqr_arg_dn7, locals.var_sqr_arg_dn8, locals.var_sqr_arg_dn9,)
    }
};
        locals.var_sqr_arg = assign5260_e5155;
        locals.var_sqr_arg_dn0 = assign5260_e5155_d_n0;
        locals.var_sqr_arg_dn1 = assign5260_e5155_d_n1;
        locals.var_sqr_arg_dn3 = assign5260_e5155_d_n3;
        locals.var_sqr_arg_dn4 = assign5260_e5155_d_n4;
        locals.var_sqr_arg_dn5 = assign5260_e5155_d_n5;
        locals.var_sqr_arg_dn6 = assign5260_e5155_d_n6;
        locals.var_sqr_arg_dn7 = assign5260_e5155_d_n7;
        locals.var_sqr_arg_dn8 = assign5260_e5155_d_n8;
        locals.var_sqr_arg_dn9 = assign5260_e5155_d_n9;

        let (assign5270_e5176, assign5270_e5176_d_n0, assign5270_e5176_d_n1, assign5270_e5176_d_n3, assign5270_e5176_d_n4, assign5270_e5176_d_n5, assign5270_e5176_d_n6, assign5270_e5176_d_n7, assign5270_e5176_d_n8, assign5270_e5176_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) && (locals.var_guard91 == 0.0)) {
        let assign5270_e5170: f64 = (locals.var_ew + locals.var_e0);
        let assign5270_e5172: f64 = (locals.var_sqr_arg).sqrt();
        let assign5270_e5173: f64 = (assign5270_e5170 + assign5270_e5172);
        let assign5270_e5174: f64 = (0.5 * assign5270_e5173);
        (assign5270_e5174, (0.5 * ((locals.var_ew_dn0 + locals.var_e0_dn0) + (locals.var_sqr_arg_dn0 / (2.0 * assign5270_e5172)))), (0.5 * ((locals.var_ew_dn1 + locals.var_e0_dn1) + (locals.var_sqr_arg_dn1 / (2.0 * assign5270_e5172)))), (0.5 * ((locals.var_ew_dn3 + locals.var_e0_dn3) + (locals.var_sqr_arg_dn3 / (2.0 * assign5270_e5172)))), (0.5 * ((locals.var_ew_dn4 + locals.var_e0_dn4) + (locals.var_sqr_arg_dn4 / (2.0 * assign5270_e5172)))), (0.5 * ((locals.var_ew_dn5 + locals.var_e0_dn5) + (locals.var_sqr_arg_dn5 / (2.0 * assign5270_e5172)))), (0.5 * ((locals.var_ew_dn6 + locals.var_e0_dn6) + (locals.var_sqr_arg_dn6 / (2.0 * assign5270_e5172)))), (0.5 * ((locals.var_ew_dn7 + locals.var_e0_dn7) + (locals.var_sqr_arg_dn7 / (2.0 * assign5270_e5172)))), (0.5 * ((locals.var_ew_dn8 + locals.var_e0_dn8) + (locals.var_sqr_arg_dn8 / (2.0 * assign5270_e5172)))), (0.5 * ((locals.var_ew_dn9 + locals.var_e0_dn9) + (locals.var_sqr_arg_dn9 / (2.0 * assign5270_e5172)))),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn1, locals.var_em_dn3, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9,)
    }
};
        locals.var_em = assign5270_e5176;
        locals.var_em_dn0 = assign5270_e5176_d_n0;
        locals.var_em_dn1 = assign5270_e5176_d_n1;
        locals.var_em_dn3 = assign5270_e5176_d_n3;
        locals.var_em_dn4 = assign5270_e5176_d_n4;
        locals.var_em_dn5 = assign5270_e5176_d_n5;
        locals.var_em_dn6 = assign5270_e5176_d_n6;
        locals.var_em_dn7 = assign5270_e5176_d_n7;
        locals.var_em_dn8 = assign5270_e5176_d_n8;
        locals.var_em_dn9 = assign5270_e5176_d_n9;

        let (assign5280_e5191, assign5280_e5191_d_n0, assign5280_e5191_d_n1, assign5280_e5191_d_n3, assign5280_e5191_d_n4, assign5280_e5191_d_n5, assign5280_e5191_d_n6, assign5280_e5191_d_n7, assign5280_e5191_d_n8, assign5280_e5191_d_n9,) = {
    if ((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5280_e5187: f64 = (locals.var_em - locals.var_eav);
        let assign5280_e5189: f64 = (assign5280_e5187 / locals.var_em);
        (assign5280_e5189, ((((locals.var_em_dn0 - locals.var_eav_dn0) * locals.var_em) - (assign5280_e5187 * locals.var_em_dn0)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn1 - locals.var_eav_dn1) * locals.var_em) - (assign5280_e5187 * locals.var_em_dn1)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn3 - locals.var_eav_dn3) * locals.var_em) - (assign5280_e5187 * locals.var_em_dn3)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn4 - locals.var_eav_dn4) * locals.var_em) - (assign5280_e5187 * locals.var_em_dn4)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn5 - locals.var_eav_dn5) * locals.var_em) - (assign5280_e5187 * locals.var_em_dn5)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn6 - locals.var_eav_dn6) * locals.var_em) - (assign5280_e5187 * locals.var_em_dn6)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn7 - locals.var_eav_dn7) * locals.var_em) - (assign5280_e5187 * locals.var_em_dn7)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn8 - locals.var_eav_dn8) * locals.var_em) - (assign5280_e5187 * locals.var_em_dn8)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn9 - locals.var_eav_dn9) * locals.var_em) - (assign5280_e5187 * locals.var_em_dn9)) / (locals.var_em * locals.var_em)),)
    } else {
        (locals.var_emeav_em, locals.var_emeav_em_dn0, locals.var_emeav_em_dn1, locals.var_emeav_em_dn3, locals.var_emeav_em_dn4, locals.var_emeav_em_dn5, locals.var_emeav_em_dn6, locals.var_emeav_em_dn7, locals.var_emeav_em_dn8, locals.var_emeav_em_dn9,)
    }
};
        locals.var_emeav_em = assign5280_e5191;
        locals.var_emeav_em_dn0 = assign5280_e5191_d_n0;
        locals.var_emeav_em_dn1 = assign5280_e5191_d_n1;
        locals.var_emeav_em_dn3 = assign5280_e5191_d_n3;
        locals.var_emeav_em_dn4 = assign5280_e5191_d_n4;
        locals.var_emeav_em_dn5 = assign5280_e5191_d_n5;
        locals.var_emeav_em_dn6 = assign5280_e5191_d_n6;
        locals.var_emeav_em_dn7 = assign5280_e5191_d_n7;
        locals.var_emeav_em_dn8 = assign5280_e5191_d_n8;
        locals.var_emeav_em_dn9 = assign5280_e5191_d_n9;

        let assign5290_e5193: f64 = (locals.var_emeav_em).abs();
        let assign5290_e5195: f64 = if assign5290_e5193 > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign5290_e5195;

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5300_e5212, assign5300_e5212_d_n0, assign5300_e5212_d_n1, assign5300_e5212_d_n3, assign5300_e5212_d_n4, assign5300_e5212_d_n5, assign5300_e5212_d_n6, assign5300_e5212_d_n7, assign5300_e5212_d_n8, assign5300_e5212_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5300_e5208: f64 = (0.5 * locals.var_wd);
        let assign5300_e5210: f64 = (assign5300_e5208 / locals.var_emeav_em);
        (assign5300_e5210, ((((0.5 * locals.var_wd_dn0) * locals.var_emeav_em) - (assign5300_e5208 * locals.var_emeav_em_dn0)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn1) * locals.var_emeav_em) - (assign5300_e5208 * locals.var_emeav_em_dn1)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn3) * locals.var_emeav_em) - (assign5300_e5208 * locals.var_emeav_em_dn3)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn4) * locals.var_emeav_em) - (assign5300_e5208 * locals.var_emeav_em_dn4)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn5) * locals.var_emeav_em) - (assign5300_e5208 * locals.var_emeav_em_dn5)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn6) * locals.var_emeav_em) - (assign5300_e5208 * locals.var_emeav_em_dn6)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn7) * locals.var_emeav_em) - (assign5300_e5208 * locals.var_emeav_em_dn7)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn8) * locals.var_emeav_em) - (assign5300_e5208 * locals.var_emeav_em_dn8)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn9) * locals.var_emeav_em) - (assign5300_e5208 * locals.var_emeav_em_dn9)) / (locals.var_emeav_em * locals.var_emeav_em)),)
    } else {
        (locals.var_lambda, locals.var_lambda_dn0, locals.var_lambda_dn1, locals.var_lambda_dn3, locals.var_lambda_dn4, locals.var_lambda_dn5, locals.var_lambda_dn6, locals.var_lambda_dn7, locals.var_lambda_dn8, locals.var_lambda_dn9,)
    }
};
        locals.var_lambda = assign5300_e5212;
        locals.var_lambda_dn0 = assign5300_e5212_d_n0;
        locals.var_lambda_dn1 = assign5300_e5212_d_n1;
        locals.var_lambda_dn3 = assign5300_e5212_d_n3;
        locals.var_lambda_dn4 = assign5300_e5212_d_n4;
        locals.var_lambda_dn5 = assign5300_e5212_d_n5;
        locals.var_lambda_dn6 = assign5300_e5212_d_n6;
        locals.var_lambda_dn7 = assign5300_e5212_d_n7;
        locals.var_lambda_dn8 = assign5300_e5212_d_n8;
        locals.var_lambda_dn9 = assign5300_e5212_d_n9;

        let (assign5310_e5249, assign5310_e5249_d_n0, assign5310_e5249_d_n1, assign5310_e5249_d_n3, assign5310_e5249_d_n4, assign5310_e5249_d_n5, assign5310_e5249_d_n6, assign5310_e5249_d_n7, assign5310_e5249_d_n8, assign5310_e5249_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) && (locals.var_guard92 != 0.0)) {
        let assign5310_e5225: f64 = (locals.var_an / locals.var_bnt);
        let assign5310_e5227: f64 = (assign5310_e5225 * locals.var_em);
        let assign5310_e5229: f64 = (assign5310_e5227 * locals.var_lambda);
        let assign5310_e5231: f64 = (-locals.var_bnt);
        let assign5310_e5233: f64 = (assign5310_e5231 / locals.var_em);
        let assign5310_e5234: f64 = (assign5310_e5233).exp();
        let assign5310_e5236: f64 = (-locals.var_bnt);
        let assign5310_e5238: f64 = (assign5310_e5236 / locals.var_em);
        let assign5310_e5242: f64 = (locals.var_weff / locals.var_lambda);
        let assign5310_e5243: f64 = (1.0 + assign5310_e5242);
        let assign5310_e5244: f64 = (assign5310_e5238 * assign5310_e5243);
        let assign5310_e5245: f64 = (assign5310_e5244).exp();
        let assign5310_e5246: f64 = (assign5310_e5234 - assign5310_e5245);
        let assign5310_e5247: f64 = (assign5310_e5229 * assign5310_e5246);
        (assign5310_e5247, (((((assign5310_e5225 * locals.var_em_dn0) * locals.var_lambda) + (assign5310_e5227 * locals.var_lambda_dn0)) * assign5310_e5246) + (assign5310_e5229 * ((assign5310_e5234 * (-((assign5310_e5231 * locals.var_em_dn0) / (locals.var_em * locals.var_em)))) - (assign5310_e5245 * (((-((assign5310_e5236 * locals.var_em_dn0) / (locals.var_em * locals.var_em))) * assign5310_e5243) + (assign5310_e5238 * (((locals.var_weff_dn0 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn0)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5310_e5225 * locals.var_em_dn1) * locals.var_lambda) + (assign5310_e5227 * locals.var_lambda_dn1)) * assign5310_e5246) + (assign5310_e5229 * ((assign5310_e5234 * (-((assign5310_e5231 * locals.var_em_dn1) / (locals.var_em * locals.var_em)))) - (assign5310_e5245 * (((-((assign5310_e5236 * locals.var_em_dn1) / (locals.var_em * locals.var_em))) * assign5310_e5243) + (assign5310_e5238 * (((locals.var_weff_dn1 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn1)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5310_e5225 * locals.var_em_dn3) * locals.var_lambda) + (assign5310_e5227 * locals.var_lambda_dn3)) * assign5310_e5246) + (assign5310_e5229 * ((assign5310_e5234 * (-((assign5310_e5231 * locals.var_em_dn3) / (locals.var_em * locals.var_em)))) - (assign5310_e5245 * (((-((assign5310_e5236 * locals.var_em_dn3) / (locals.var_em * locals.var_em))) * assign5310_e5243) + (assign5310_e5238 * (((locals.var_weff_dn3 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn3)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5310_e5225 * locals.var_em_dn4) * locals.var_lambda) + (assign5310_e5227 * locals.var_lambda_dn4)) * assign5310_e5246) + (assign5310_e5229 * ((assign5310_e5234 * (-((assign5310_e5231 * locals.var_em_dn4) / (locals.var_em * locals.var_em)))) - (assign5310_e5245 * (((-((assign5310_e5236 * locals.var_em_dn4) / (locals.var_em * locals.var_em))) * assign5310_e5243) + (assign5310_e5238 * (((locals.var_weff_dn4 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn4)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5310_e5225 * locals.var_em_dn5) * locals.var_lambda) + (assign5310_e5227 * locals.var_lambda_dn5)) * assign5310_e5246) + (assign5310_e5229 * ((assign5310_e5234 * (-((assign5310_e5231 * locals.var_em_dn5) / (locals.var_em * locals.var_em)))) - (assign5310_e5245 * (((-((assign5310_e5236 * locals.var_em_dn5) / (locals.var_em * locals.var_em))) * assign5310_e5243) + (assign5310_e5238 * (((locals.var_weff_dn5 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn5)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5310_e5225 * locals.var_em_dn6) * locals.var_lambda) + (assign5310_e5227 * locals.var_lambda_dn6)) * assign5310_e5246) + (assign5310_e5229 * ((assign5310_e5234 * (-((assign5310_e5231 * locals.var_em_dn6) / (locals.var_em * locals.var_em)))) - (assign5310_e5245 * (((-((assign5310_e5236 * locals.var_em_dn6) / (locals.var_em * locals.var_em))) * assign5310_e5243) + (assign5310_e5238 * (((locals.var_weff_dn6 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn6)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5310_e5225 * locals.var_em_dn7) * locals.var_lambda) + (assign5310_e5227 * locals.var_lambda_dn7)) * assign5310_e5246) + (assign5310_e5229 * ((assign5310_e5234 * (-((assign5310_e5231 * locals.var_em_dn7) / (locals.var_em * locals.var_em)))) - (assign5310_e5245 * (((-((assign5310_e5236 * locals.var_em_dn7) / (locals.var_em * locals.var_em))) * assign5310_e5243) + (assign5310_e5238 * (((locals.var_weff_dn7 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn7)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5310_e5225 * locals.var_em_dn8) * locals.var_lambda) + (assign5310_e5227 * locals.var_lambda_dn8)) * assign5310_e5246) + (assign5310_e5229 * ((assign5310_e5234 * (-((assign5310_e5231 * locals.var_em_dn8) / (locals.var_em * locals.var_em)))) - (assign5310_e5245 * (((-((assign5310_e5236 * locals.var_em_dn8) / (locals.var_em * locals.var_em))) * assign5310_e5243) + (assign5310_e5238 * (((locals.var_weff_dn8 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn8)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5310_e5225 * locals.var_em_dn9) * locals.var_lambda) + (assign5310_e5227 * locals.var_lambda_dn9)) * assign5310_e5246) + (assign5310_e5229 * ((assign5310_e5234 * (-((assign5310_e5231 * locals.var_em_dn9) / (locals.var_em * locals.var_em)))) - (assign5310_e5245 * (((-((assign5310_e5236 * locals.var_em_dn9) / (locals.var_em * locals.var_em))) * assign5310_e5243) + (assign5310_e5238 * (((locals.var_weff_dn9 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn9)) / (locals.var_lambda * locals.var_lambda)))))))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9,)
    }
};
        locals.var_gem = assign5310_e5249;
        locals.var_gem_dn0 = assign5310_e5249_d_n0;
        locals.var_gem_dn1 = assign5310_e5249_d_n1;
        locals.var_gem_dn3 = assign5310_e5249_d_n3;
        locals.var_gem_dn4 = assign5310_e5249_d_n4;
        locals.var_gem_dn5 = assign5310_e5249_d_n5;
        locals.var_gem_dn6 = assign5310_e5249_d_n6;
        locals.var_gem_dn7 = assign5310_e5249_d_n7;
        locals.var_gem_dn8 = assign5310_e5249_d_n8;
        locals.var_gem_dn9 = assign5310_e5249_d_n9;

        let (assign5320_e5271, assign5320_e5271_d_n0, assign5320_e5271_d_n1, assign5320_e5271_d_n3, assign5320_e5271_d_n4, assign5320_e5271_d_n5, assign5320_e5271_d_n6, assign5320_e5271_d_n7, assign5320_e5271_d_n8, assign5320_e5271_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 != 0.0)) && (locals.var_guard89 != 0.0)) && (locals.var_guard92 == 0.0)) {
        let assign5320_e5263: f64 = (locals.var_an * locals.var_weff);
        let assign5320_e5265: f64 = (-locals.var_bnt);
        let assign5320_e5267: f64 = (assign5320_e5265 / locals.var_em);
        let assign5320_e5268: f64 = (assign5320_e5267).exp();
        let assign5320_e5269: f64 = (assign5320_e5263 * assign5320_e5268);
        (assign5320_e5269, (((locals.var_an * locals.var_weff_dn0) * assign5320_e5268) + (assign5320_e5263 * (assign5320_e5268 * (-((assign5320_e5265 * locals.var_em_dn0) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn1) * assign5320_e5268) + (assign5320_e5263 * (assign5320_e5268 * (-((assign5320_e5265 * locals.var_em_dn1) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn3) * assign5320_e5268) + (assign5320_e5263 * (assign5320_e5268 * (-((assign5320_e5265 * locals.var_em_dn3) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn4) * assign5320_e5268) + (assign5320_e5263 * (assign5320_e5268 * (-((assign5320_e5265 * locals.var_em_dn4) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn5) * assign5320_e5268) + (assign5320_e5263 * (assign5320_e5268 * (-((assign5320_e5265 * locals.var_em_dn5) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn6) * assign5320_e5268) + (assign5320_e5263 * (assign5320_e5268 * (-((assign5320_e5265 * locals.var_em_dn6) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn7) * assign5320_e5268) + (assign5320_e5263 * (assign5320_e5268 * (-((assign5320_e5265 * locals.var_em_dn7) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn8) * assign5320_e5268) + (assign5320_e5263 * (assign5320_e5268 * (-((assign5320_e5265 * locals.var_em_dn8) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn9) * assign5320_e5268) + (assign5320_e5263 * (assign5320_e5268 * (-((assign5320_e5265 * locals.var_em_dn9) / (locals.var_em * locals.var_em)))))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9,)
    }
};
        locals.var_gem = assign5320_e5271;
        locals.var_gem_dn0 = assign5320_e5271_d_n0;
        locals.var_gem_dn1 = assign5320_e5271_d_n1;
        locals.var_gem_dn3 = assign5320_e5271_d_n3;
        locals.var_gem_dn4 = assign5320_e5271_d_n4;
        locals.var_gem_dn5 = assign5320_e5271_d_n5;
        locals.var_gem_dn6 = assign5320_e5271_d_n6;
        locals.var_gem_dn7 = assign5320_e5271_d_n7;
        locals.var_gem_dn8 = assign5320_e5271_d_n8;
        locals.var_gem_dn9 = assign5320_e5271_d_n9;

        let assign5330_e5274: f64 = if p.p38 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign5330_e5274;

        let assign5340_e5277: f64 = if locals.var_vb2c1 < p.p43 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign5340_e5277;

        let (assign5350_e5305, assign5350_e5305_d_n0, assign5350_e5305_d_n1, assign5350_e5305_d_n3, assign5350_e5305_d_n4, assign5350_e5305_d_n5, assign5350_e5305_d_n6, assign5350_e5305_d_n7, assign5350_e5305_d_n8, assign5350_e5305_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) {
        let assign5350_e5291: f64 = (p.p43 - locals.var_vb2c1);
        let assign5350_e5293: f64 = (assign5350_e5291).powf(p.p40);
        let assign5350_e5298: f64 = (p.p47 + locals.var_in_);
        let assign5350_e5299: f64 = (locals.var_in_ / assign5350_e5298);
        let assign5350_e5300: f64 = (1.0 - assign5350_e5299);
        let assign5350_e5302: f64 = (assign5350_e5300).powf(p.p48);
        let assign5350_e5303: f64 = (assign5350_e5293 * assign5350_e5302);
        (assign5350_e5303, (assign5350_e5293 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5350_e5300).powf(p.p48 - 1.0) * (-(((locals.var_in__dn0 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn0)) / (assign5350_e5298 * assign5350_e5298))))) } } else { (assign5350_e5302 * (p.p48 * ((-(((locals.var_in__dn0 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn0)) / (assign5350_e5298 * assign5350_e5298))) / assign5350_e5300))) }), (assign5350_e5293 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5350_e5300).powf(p.p48 - 1.0) * (-(((locals.var_in__dn1 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn1)) / (assign5350_e5298 * assign5350_e5298))))) } } else { (assign5350_e5302 * (p.p48 * ((-(((locals.var_in__dn1 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn1)) / (assign5350_e5298 * assign5350_e5298))) / assign5350_e5300))) }), (assign5350_e5293 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5350_e5300).powf(p.p48 - 1.0) * (-(((locals.var_in__dn3 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn3)) / (assign5350_e5298 * assign5350_e5298))))) } } else { (assign5350_e5302 * (p.p48 * ((-(((locals.var_in__dn3 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn3)) / (assign5350_e5298 * assign5350_e5298))) / assign5350_e5300))) }), (assign5350_e5293 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5350_e5300).powf(p.p48 - 1.0) * (-(((locals.var_in__dn4 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn4)) / (assign5350_e5298 * assign5350_e5298))))) } } else { (assign5350_e5302 * (p.p48 * ((-(((locals.var_in__dn4 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn4)) / (assign5350_e5298 * assign5350_e5298))) / assign5350_e5300))) }), ((if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((assign5350_e5291).powf(p.p40 - 1.0) * (-locals.var_vb2c1_dn5))) } } else { (assign5350_e5293 * (p.p40 * ((-locals.var_vb2c1_dn5) / assign5350_e5291))) } * assign5350_e5302) + (assign5350_e5293 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5350_e5300).powf(p.p48 - 1.0) * (-(((locals.var_in__dn5 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn5)) / (assign5350_e5298 * assign5350_e5298))))) } } else { (assign5350_e5302 * (p.p48 * ((-(((locals.var_in__dn5 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn5)) / (assign5350_e5298 * assign5350_e5298))) / assign5350_e5300))) })), ((if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((assign5350_e5291).powf(p.p40 - 1.0) * (-locals.var_vb2c1_dn6))) } } else { (assign5350_e5293 * (p.p40 * ((-locals.var_vb2c1_dn6) / assign5350_e5291))) } * assign5350_e5302) + (assign5350_e5293 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5350_e5300).powf(p.p48 - 1.0) * (-(((locals.var_in__dn6 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn6)) / (assign5350_e5298 * assign5350_e5298))))) } } else { (assign5350_e5302 * (p.p48 * ((-(((locals.var_in__dn6 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn6)) / (assign5350_e5298 * assign5350_e5298))) / assign5350_e5300))) })), (assign5350_e5293 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5350_e5300).powf(p.p48 - 1.0) * (-(((locals.var_in__dn7 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn7)) / (assign5350_e5298 * assign5350_e5298))))) } } else { (assign5350_e5302 * (p.p48 * ((-(((locals.var_in__dn7 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn7)) / (assign5350_e5298 * assign5350_e5298))) / assign5350_e5300))) }), (assign5350_e5293 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5350_e5300).powf(p.p48 - 1.0) * (-(((locals.var_in__dn8 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn8)) / (assign5350_e5298 * assign5350_e5298))))) } } else { (assign5350_e5302 * (p.p48 * ((-(((locals.var_in__dn8 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn8)) / (assign5350_e5298 * assign5350_e5298))) / assign5350_e5300))) }), (assign5350_e5293 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5350_e5300).powf(p.p48 - 1.0) * (-(((locals.var_in__dn9 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn9)) / (assign5350_e5298 * assign5350_e5298))))) } } else { (assign5350_e5302 * (p.p48 * ((-(((locals.var_in__dn9 * assign5350_e5298) - (locals.var_in_ * locals.var_in__dn9)) / (assign5350_e5298 * assign5350_e5298))) / assign5350_e5300))) }),)
    } else {
        (locals.var_vdeptmp, locals.var_vdeptmp_dn0, locals.var_vdeptmp_dn1, locals.var_vdeptmp_dn3, locals.var_vdeptmp_dn4, locals.var_vdeptmp_dn5, locals.var_vdeptmp_dn6, locals.var_vdeptmp_dn7, locals.var_vdeptmp_dn8, locals.var_vdeptmp_dn9,)
    }
};
        locals.var_vdeptmp = assign5350_e5305;
        locals.var_vdeptmp_dn0 = assign5350_e5305_d_n0;
        locals.var_vdeptmp_dn1 = assign5350_e5305_d_n1;
        locals.var_vdeptmp_dn3 = assign5350_e5305_d_n3;
        locals.var_vdeptmp_dn4 = assign5350_e5305_d_n4;
        locals.var_vdeptmp_dn5 = assign5350_e5305_d_n5;
        locals.var_vdeptmp_dn6 = assign5350_e5305_d_n6;
        locals.var_vdeptmp_dn7 = assign5350_e5305_d_n7;
        locals.var_vdeptmp_dn8 = assign5350_e5305_d_n8;
        locals.var_vdeptmp_dn9 = assign5350_e5305_d_n9;

        let assign5360_e5308: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign5360_e5308;

        let (assign5370_e5324, assign5370_e5324_d_n0, assign5370_e5324_d_n1, assign5370_e5324_d_n3, assign5370_e5324_d_n4, assign5370_e5324_d_n5, assign5370_e5324_d_n6, assign5370_e5324_d_n7, assign5370_e5324_d_n8, assign5370_e5324_d_n9,) = {
    if ((((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard95 != 0.0)) {
        (locals.var_vdeptmp, locals.var_vdeptmp_dn0, locals.var_vdeptmp_dn1, locals.var_vdeptmp_dn3, locals.var_vdeptmp_dn4, locals.var_vdeptmp_dn5, locals.var_vdeptmp_dn6, locals.var_vdeptmp_dn7, locals.var_vdeptmp_dn8, locals.var_vdeptmp_dn9,)
    } else {
        (locals.var_vdep, locals.var_vdep_dn0, locals.var_vdep_dn1, locals.var_vdep_dn3, locals.var_vdep_dn4, locals.var_vdep_dn5, locals.var_vdep_dn6, locals.var_vdep_dn7, locals.var_vdep_dn8, locals.var_vdep_dn9,)
    }
};
        locals.var_vdep = assign5370_e5324;
        locals.var_vdep_dn0 = assign5370_e5324_d_n0;
        locals.var_vdep_dn1 = assign5370_e5324_d_n1;
        locals.var_vdep_dn3 = assign5370_e5324_d_n3;
        locals.var_vdep_dn4 = assign5370_e5324_d_n4;
        locals.var_vdep_dn5 = assign5370_e5324_d_n5;
        locals.var_vdep_dn6 = assign5370_e5324_d_n6;
        locals.var_vdep_dn7 = assign5370_e5324_d_n7;
        locals.var_vdep_dn8 = assign5370_e5324_d_n8;
        locals.var_vdep_dn9 = assign5370_e5324_d_n9;

        let (assign5380_e5345, assign5380_e5345_d_n0, assign5380_e5345_d_n1, assign5380_e5345_d_n3, assign5380_e5345_d_n4, assign5380_e5345_d_n5, assign5380_e5345_d_n6, assign5380_e5345_d_n7, assign5380_e5345_d_n8, assign5380_e5345_d_n9,) = {
    if ((((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard95 == 0.0)) {
        let assign5380_e5341: f64 = (locals.var_in_ - p.p51);
        let assign5380_e5343: f64 = (assign5380_e5341 / p.p47);
        (assign5380_e5343, (locals.var_in__dn0 / p.p47), (locals.var_in__dn1 / p.p47), (locals.var_in__dn3 / p.p47), (locals.var_in__dn4 / p.p47), (locals.var_in__dn5 / p.p47), (locals.var_in__dn6 / p.p47), (locals.var_in__dn7 / p.p47), (locals.var_in__dn8 / p.p47), (locals.var_in__dn9 / p.p47),)
    } else {
        (locals.var_in_shift_ihcavl, locals.var_in_shift_ihcavl_dn0, locals.var_in_shift_ihcavl_dn1, locals.var_in_shift_ihcavl_dn3, locals.var_in_shift_ihcavl_dn4, locals.var_in_shift_ihcavl_dn5, locals.var_in_shift_ihcavl_dn6, locals.var_in_shift_ihcavl_dn7, locals.var_in_shift_ihcavl_dn8, locals.var_in_shift_ihcavl_dn9,)
    }
};
        locals.var_in_shift_ihcavl = assign5380_e5345;
        locals.var_in_shift_ihcavl_dn0 = assign5380_e5345_d_n0;
        locals.var_in_shift_ihcavl_dn1 = assign5380_e5345_d_n1;
        locals.var_in_shift_ihcavl_dn3 = assign5380_e5345_d_n3;
        locals.var_in_shift_ihcavl_dn4 = assign5380_e5345_d_n4;
        locals.var_in_shift_ihcavl_dn5 = assign5380_e5345_d_n5;
        locals.var_in_shift_ihcavl_dn6 = assign5380_e5345_d_n6;
        locals.var_in_shift_ihcavl_dn7 = assign5380_e5345_d_n7;
        locals.var_in_shift_ihcavl_dn8 = assign5380_e5345_d_n8;
        locals.var_in_shift_ihcavl_dn9 = assign5380_e5345_d_n9;

        let (assign5390_e5366, assign5390_e5366_d_n0, assign5390_e5366_d_n1, assign5390_e5366_d_n3, assign5390_e5366_d_n4, assign5390_e5366_d_n5, assign5390_e5366_d_n6, assign5390_e5366_d_n7, assign5390_e5366_d_n8, assign5390_e5366_d_n9,) = {
    if ((((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard95 == 0.0)) {
        let assign5390_e5362: f64 = (locals.var_in_shift_ihcavl - 1.0);
        let assign5390_e5364: f64 = (assign5390_e5362 / p.p50);
        (assign5390_e5364, (locals.var_in_shift_ihcavl_dn0 / p.p50), (locals.var_in_shift_ihcavl_dn1 / p.p50), (locals.var_in_shift_ihcavl_dn3 / p.p50), (locals.var_in_shift_ihcavl_dn4 / p.p50), (locals.var_in_shift_ihcavl_dn5 / p.p50), (locals.var_in_shift_ihcavl_dn6 / p.p50), (locals.var_in_shift_ihcavl_dn7 / p.p50), (locals.var_in_shift_ihcavl_dn8 / p.p50), (locals.var_in_shift_ihcavl_dn9 / p.p50),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9,)
    }
};
        locals.var_dxa = assign5390_e5366;
        locals.var_dxa_dn0 = assign5390_e5366_d_n0;
        locals.var_dxa_dn1 = assign5390_e5366_d_n1;
        locals.var_dxa_dn3 = assign5390_e5366_d_n3;
        locals.var_dxa_dn4 = assign5390_e5366_d_n4;
        locals.var_dxa_dn5 = assign5390_e5366_d_n5;
        locals.var_dxa_dn6 = assign5390_e5366_d_n6;
        locals.var_dxa_dn7 = assign5390_e5366_d_n7;
        locals.var_dxa_dn8 = assign5390_e5366_d_n8;
        locals.var_dxa_dn9 = assign5390_e5366_d_n9;

        let assign5400_e5369: f64 = if locals.var_in_shift_ihcavl < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign5400_e5369;

        let (assign5410_e5396, assign5410_e5396_d_n0, assign5410_e5396_d_n1, assign5410_e5396_d_n3, assign5410_e5396_d_n4, assign5410_e5396_d_n5, assign5410_e5396_d_n6, assign5410_e5396_d_n7, assign5410_e5396_d_n8, assign5410_e5396_d_n9,) = {
    if (((((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard95 == 0.0)) && (locals.var_guard96 != 0.0)) {
        let assign5410_e5390: f64 = (locals.var_dxa).exp();
        let assign5410_e5391: f64 = (1.0 + assign5410_e5390);
        let assign5410_e5392: f64 = (assign5410_e5391).ln();
        let assign5410_e5393: f64 = (p.p50 * assign5410_e5392);
        let assign5410_e5394: f64 = (1.0 + assign5410_e5393);
        (assign5410_e5394, (p.p50 * ((assign5410_e5390 * locals.var_dxa_dn0) / assign5410_e5391)), (p.p50 * ((assign5410_e5390 * locals.var_dxa_dn1) / assign5410_e5391)), (p.p50 * ((assign5410_e5390 * locals.var_dxa_dn3) / assign5410_e5391)), (p.p50 * ((assign5410_e5390 * locals.var_dxa_dn4) / assign5410_e5391)), (p.p50 * ((assign5410_e5390 * locals.var_dxa_dn5) / assign5410_e5391)), (p.p50 * ((assign5410_e5390 * locals.var_dxa_dn6) / assign5410_e5391)), (p.p50 * ((assign5410_e5390 * locals.var_dxa_dn7) / assign5410_e5391)), (p.p50 * ((assign5410_e5390 * locals.var_dxa_dn8) / assign5410_e5391)), (p.p50 * ((assign5410_e5390 * locals.var_dxa_dn9) / assign5410_e5391)),)
    } else {
        (locals.var_in_shift_n, locals.var_in_shift_n_dn0, locals.var_in_shift_n_dn1, locals.var_in_shift_n_dn3, locals.var_in_shift_n_dn4, locals.var_in_shift_n_dn5, locals.var_in_shift_n_dn6, locals.var_in_shift_n_dn7, locals.var_in_shift_n_dn8, locals.var_in_shift_n_dn9,)
    }
};
        locals.var_in_shift_n = assign5410_e5396;
        locals.var_in_shift_n_dn0 = assign5410_e5396_d_n0;
        locals.var_in_shift_n_dn1 = assign5410_e5396_d_n1;
        locals.var_in_shift_n_dn3 = assign5410_e5396_d_n3;
        locals.var_in_shift_n_dn4 = assign5410_e5396_d_n4;
        locals.var_in_shift_n_dn5 = assign5410_e5396_d_n5;
        locals.var_in_shift_n_dn6 = assign5410_e5396_d_n6;
        locals.var_in_shift_n_dn7 = assign5410_e5396_d_n7;
        locals.var_in_shift_n_dn8 = assign5410_e5396_d_n8;
        locals.var_in_shift_n_dn9 = assign5410_e5396_d_n9;

        let (assign5420_e5425, assign5420_e5425_d_n0, assign5420_e5425_d_n1, assign5420_e5425_d_n3, assign5420_e5425_d_n4, assign5420_e5425_d_n5, assign5420_e5425_d_n6, assign5420_e5425_d_n7, assign5420_e5425_d_n8, assign5420_e5425_d_n9,) = {
    if (((((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard95 == 0.0)) && (locals.var_guard96 == 0.0)) {
        let assign5420_e5418: f64 = (-locals.var_dxa);
        let assign5420_e5419: f64 = (assign5420_e5418).exp();
        let assign5420_e5420: f64 = (1.0 + assign5420_e5419);
        let assign5420_e5421: f64 = (assign5420_e5420).ln();
        let assign5420_e5422: f64 = (p.p50 * assign5420_e5421);
        let assign5420_e5423: f64 = (locals.var_in_shift_ihcavl + assign5420_e5422);
        (assign5420_e5423, (locals.var_in_shift_ihcavl_dn0 + (p.p50 * ((assign5420_e5419 * (-locals.var_dxa_dn0)) / assign5420_e5420))), (locals.var_in_shift_ihcavl_dn1 + (p.p50 * ((assign5420_e5419 * (-locals.var_dxa_dn1)) / assign5420_e5420))), (locals.var_in_shift_ihcavl_dn3 + (p.p50 * ((assign5420_e5419 * (-locals.var_dxa_dn3)) / assign5420_e5420))), (locals.var_in_shift_ihcavl_dn4 + (p.p50 * ((assign5420_e5419 * (-locals.var_dxa_dn4)) / assign5420_e5420))), (locals.var_in_shift_ihcavl_dn5 + (p.p50 * ((assign5420_e5419 * (-locals.var_dxa_dn5)) / assign5420_e5420))), (locals.var_in_shift_ihcavl_dn6 + (p.p50 * ((assign5420_e5419 * (-locals.var_dxa_dn6)) / assign5420_e5420))), (locals.var_in_shift_ihcavl_dn7 + (p.p50 * ((assign5420_e5419 * (-locals.var_dxa_dn7)) / assign5420_e5420))), (locals.var_in_shift_ihcavl_dn8 + (p.p50 * ((assign5420_e5419 * (-locals.var_dxa_dn8)) / assign5420_e5420))), (locals.var_in_shift_ihcavl_dn9 + (p.p50 * ((assign5420_e5419 * (-locals.var_dxa_dn9)) / assign5420_e5420))),)
    } else {
        (locals.var_in_shift_n, locals.var_in_shift_n_dn0, locals.var_in_shift_n_dn1, locals.var_in_shift_n_dn3, locals.var_in_shift_n_dn4, locals.var_in_shift_n_dn5, locals.var_in_shift_n_dn6, locals.var_in_shift_n_dn7, locals.var_in_shift_n_dn8, locals.var_in_shift_n_dn9,)
    }
};
        locals.var_in_shift_n = assign5420_e5425;
        locals.var_in_shift_n_dn0 = assign5420_e5425_d_n0;
        locals.var_in_shift_n_dn1 = assign5420_e5425_d_n1;
        locals.var_in_shift_n_dn3 = assign5420_e5425_d_n3;
        locals.var_in_shift_n_dn4 = assign5420_e5425_d_n4;
        locals.var_in_shift_n_dn5 = assign5420_e5425_d_n5;
        locals.var_in_shift_n_dn6 = assign5420_e5425_d_n6;
        locals.var_in_shift_n_dn7 = assign5420_e5425_d_n7;
        locals.var_in_shift_n_dn8 = assign5420_e5425_d_n8;
        locals.var_in_shift_n_dn9 = assign5420_e5425_d_n9;

        let (assign5430_e5446, assign5430_e5446_d_n0, assign5430_e5446_d_n1, assign5430_e5446_d_n3, assign5430_e5446_d_n4, assign5430_e5446_d_n5, assign5430_e5446_d_n6, assign5430_e5446_d_n7, assign5430_e5446_d_n8, assign5430_e5446_d_n9,) = {
    if ((((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard95 == 0.0)) {
        let assign5430_e5443: f64 = (locals.var_in_shift_n).powf(p.p49);
        let assign5430_e5444: f64 = (locals.var_vdeptmp * assign5430_e5443);
        (assign5430_e5444, ((locals.var_vdeptmp_dn0 * assign5430_e5443) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn0)) } } else { (assign5430_e5443 * (p.p49 * (locals.var_in_shift_n_dn0 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn1 * assign5430_e5443) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn1)) } } else { (assign5430_e5443 * (p.p49 * (locals.var_in_shift_n_dn1 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn3 * assign5430_e5443) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn3)) } } else { (assign5430_e5443 * (p.p49 * (locals.var_in_shift_n_dn3 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn4 * assign5430_e5443) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn4)) } } else { (assign5430_e5443 * (p.p49 * (locals.var_in_shift_n_dn4 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn5 * assign5430_e5443) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn5)) } } else { (assign5430_e5443 * (p.p49 * (locals.var_in_shift_n_dn5 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn6 * assign5430_e5443) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn6)) } } else { (assign5430_e5443 * (p.p49 * (locals.var_in_shift_n_dn6 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn7 * assign5430_e5443) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn7)) } } else { (assign5430_e5443 * (p.p49 * (locals.var_in_shift_n_dn7 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn8 * assign5430_e5443) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn8)) } } else { (assign5430_e5443 * (p.p49 * (locals.var_in_shift_n_dn8 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn9 * assign5430_e5443) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn9)) } } else { (assign5430_e5443 * (p.p49 * (locals.var_in_shift_n_dn9 / locals.var_in_shift_n))) })),)
    } else {
        (locals.var_vdep, locals.var_vdep_dn0, locals.var_vdep_dn1, locals.var_vdep_dn3, locals.var_vdep_dn4, locals.var_vdep_dn5, locals.var_vdep_dn6, locals.var_vdep_dn7, locals.var_vdep_dn8, locals.var_vdep_dn9,)
    }
};
        locals.var_vdep = assign5430_e5446;
        locals.var_vdep_dn0 = assign5430_e5446_d_n0;
        locals.var_vdep_dn1 = assign5430_e5446_d_n1;
        locals.var_vdep_dn3 = assign5430_e5446_d_n3;
        locals.var_vdep_dn4 = assign5430_e5446_d_n4;
        locals.var_vdep_dn5 = assign5430_e5446_d_n5;
        locals.var_vdep_dn6 = assign5430_e5446_d_n6;
        locals.var_vdep_dn7 = assign5430_e5446_d_n7;
        locals.var_vdep_dn8 = assign5430_e5446_d_n8;
        locals.var_vdep_dn9 = assign5430_e5446_d_n9;

        let assign5440_e5448: f64 = (-locals.var_bavl_t);
        let assign5440_e5450: f64 = (assign5440_e5448 * locals.var_vdep);
        let assign5440_e5452: f64 = if assign5440_e5450 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign5440_e5452;

        let (assign5450_e5472, assign5450_e5472_d_n0, assign5450_e5472_d_n1, assign5450_e5472_d_n3, assign5450_e5472_d_n4, assign5450_e5472_d_n5, assign5450_e5472_d_n6, assign5450_e5472_d_n7, assign5450_e5472_d_n8, assign5450_e5472_d_n9,) = {
    if ((((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard97 != 0.0)) {
        let assign5450_e5467: f64 = (-locals.var_bavl_t);
        let assign5450_e5469: f64 = (assign5450_e5467 * locals.var_vdep);
        let assign5450_e5470: f64 = (assign5450_e5469).exp();
        (assign5450_e5470, (assign5450_e5470 * (((-locals.var_bavl_t_dn0) * locals.var_vdep) + (assign5450_e5467 * locals.var_vdep_dn0))), (assign5450_e5470 * (((-locals.var_bavl_t_dn1) * locals.var_vdep) + (assign5450_e5467 * locals.var_vdep_dn1))), (assign5450_e5470 * (((-locals.var_bavl_t_dn3) * locals.var_vdep) + (assign5450_e5467 * locals.var_vdep_dn3))), (assign5450_e5470 * (((-locals.var_bavl_t_dn4) * locals.var_vdep) + (assign5450_e5467 * locals.var_vdep_dn4))), (assign5450_e5470 * (((-locals.var_bavl_t_dn5) * locals.var_vdep) + (assign5450_e5467 * locals.var_vdep_dn5))), (assign5450_e5470 * (((-locals.var_bavl_t_dn6) * locals.var_vdep) + (assign5450_e5467 * locals.var_vdep_dn6))), (assign5450_e5470 * (((-locals.var_bavl_t_dn7) * locals.var_vdep) + (assign5450_e5467 * locals.var_vdep_dn7))), (assign5450_e5470 * (((-locals.var_bavl_t_dn8) * locals.var_vdep) + (assign5450_e5467 * locals.var_vdep_dn8))), (assign5450_e5470 * (((-locals.var_bavl_t_dn9) * locals.var_vdep) + (assign5450_e5467 * locals.var_vdep_dn9))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9,)
    }
};
        locals.var_expmm1 = assign5450_e5472;
        locals.var_expmm1_dn0 = assign5450_e5472_d_n0;
        locals.var_expmm1_dn1 = assign5450_e5472_d_n1;
        locals.var_expmm1_dn3 = assign5450_e5472_d_n3;
        locals.var_expmm1_dn4 = assign5450_e5472_d_n4;
        locals.var_expmm1_dn5 = assign5450_e5472_d_n5;
        locals.var_expmm1_dn6 = assign5450_e5472_d_n6;
        locals.var_expmm1_dn7 = assign5450_e5472_d_n7;
        locals.var_expmm1_dn8 = assign5450_e5472_d_n8;
        locals.var_expmm1_dn9 = assign5450_e5472_d_n9;

        let (assign5460_e5490,) = {
    if ((((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard97 == 0.0)) {
        let assign5460_e5488: f64 = (p.p134).exp();
        (assign5460_e5488,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5460_e5490;

        let (assign5470_e5516, assign5470_e5516_d_n0, assign5470_e5516_d_n1, assign5470_e5516_d_n3, assign5470_e5516_d_n4, assign5470_e5516_d_n5, assign5470_e5516_d_n6, assign5470_e5516_d_n7, assign5470_e5516_d_n8, assign5470_e5516_d_n9,) = {
    if ((((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard97 == 0.0)) {
        let assign5470_e5508: f64 = (-locals.var_bavl_t);
        let assign5470_e5510: f64 = (assign5470_e5508 * locals.var_vdep);
        let assign5470_e5512: f64 = (assign5470_e5510 - p.p134);
        let assign5470_e5513: f64 = (1.0 + assign5470_e5512);
        let assign5470_e5514: f64 = (locals.var_expl * assign5470_e5513);
        (assign5470_e5514, (locals.var_expl * (((-locals.var_bavl_t_dn0) * locals.var_vdep) + (assign5470_e5508 * locals.var_vdep_dn0))), (locals.var_expl * (((-locals.var_bavl_t_dn1) * locals.var_vdep) + (assign5470_e5508 * locals.var_vdep_dn1))), (locals.var_expl * (((-locals.var_bavl_t_dn3) * locals.var_vdep) + (assign5470_e5508 * locals.var_vdep_dn3))), (locals.var_expl * (((-locals.var_bavl_t_dn4) * locals.var_vdep) + (assign5470_e5508 * locals.var_vdep_dn4))), (locals.var_expl * (((-locals.var_bavl_t_dn5) * locals.var_vdep) + (assign5470_e5508 * locals.var_vdep_dn5))), (locals.var_expl * (((-locals.var_bavl_t_dn6) * locals.var_vdep) + (assign5470_e5508 * locals.var_vdep_dn6))), (locals.var_expl * (((-locals.var_bavl_t_dn7) * locals.var_vdep) + (assign5470_e5508 * locals.var_vdep_dn7))), (locals.var_expl * (((-locals.var_bavl_t_dn8) * locals.var_vdep) + (assign5470_e5508 * locals.var_vdep_dn8))), (locals.var_expl * (((-locals.var_bavl_t_dn9) * locals.var_vdep) + (assign5470_e5508 * locals.var_vdep_dn9))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9,)
    }
};
        locals.var_expmm1 = assign5470_e5516;
        locals.var_expmm1_dn0 = assign5470_e5516_d_n0;
        locals.var_expmm1_dn1 = assign5470_e5516_d_n1;
        locals.var_expmm1_dn3 = assign5470_e5516_d_n3;
        locals.var_expmm1_dn4 = assign5470_e5516_d_n4;
        locals.var_expmm1_dn5 = assign5470_e5516_d_n5;
        locals.var_expmm1_dn6 = assign5470_e5516_d_n6;
        locals.var_expmm1_dn7 = assign5470_e5516_d_n7;
        locals.var_expmm1_dn8 = assign5470_e5516_d_n8;
        locals.var_expmm1_dn9 = assign5470_e5516_d_n9;

        let (assign5480_e5538, assign5480_e5538_d_n0, assign5480_e5538_d_n1, assign5480_e5538_d_n3, assign5480_e5538_d_n4, assign5480_e5538_d_n5, assign5480_e5538_d_n6, assign5480_e5538_d_n7, assign5480_e5538_d_n8, assign5480_e5538_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) {
        let assign5480_e5530: f64 = (p.p39 / locals.var_bavl_t);
        let assign5480_e5533: f64 = (p.p43 - locals.var_vb2c1);
        let assign5480_e5534: f64 = (assign5480_e5530 * assign5480_e5533);
        let assign5480_e5536: f64 = (assign5480_e5534 * locals.var_expmm1);
        (assign5480_e5536, ((((-((p.p39 * locals.var_bavl_t_dn0) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5480_e5533) * locals.var_expmm1) + (assign5480_e5534 * locals.var_expmm1_dn0)), ((((-((p.p39 * locals.var_bavl_t_dn1) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5480_e5533) * locals.var_expmm1) + (assign5480_e5534 * locals.var_expmm1_dn1)), ((((-((p.p39 * locals.var_bavl_t_dn3) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5480_e5533) * locals.var_expmm1) + (assign5480_e5534 * locals.var_expmm1_dn3)), ((((-((p.p39 * locals.var_bavl_t_dn4) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5480_e5533) * locals.var_expmm1) + (assign5480_e5534 * locals.var_expmm1_dn4)), (((((-((p.p39 * locals.var_bavl_t_dn5) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5480_e5533) + (assign5480_e5530 * (-locals.var_vb2c1_dn5))) * locals.var_expmm1) + (assign5480_e5534 * locals.var_expmm1_dn5)), (((((-((p.p39 * locals.var_bavl_t_dn6) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5480_e5533) + (assign5480_e5530 * (-locals.var_vb2c1_dn6))) * locals.var_expmm1) + (assign5480_e5534 * locals.var_expmm1_dn6)), ((((-((p.p39 * locals.var_bavl_t_dn7) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5480_e5533) * locals.var_expmm1) + (assign5480_e5534 * locals.var_expmm1_dn7)), ((((-((p.p39 * locals.var_bavl_t_dn8) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5480_e5533) * locals.var_expmm1) + (assign5480_e5534 * locals.var_expmm1_dn8)), ((((-((p.p39 * locals.var_bavl_t_dn9) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5480_e5533) * locals.var_expmm1) + (assign5480_e5534 * locals.var_expmm1_dn9)),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9,)
    }
};
        locals.var_gem = assign5480_e5538;
        locals.var_gem_dn0 = assign5480_e5538_d_n0;
        locals.var_gem_dn1 = assign5480_e5538_d_n1;
        locals.var_gem_dn3 = assign5480_e5538_d_n3;
        locals.var_gem_dn4 = assign5480_e5538_d_n4;
        locals.var_gem_dn5 = assign5480_e5538_d_n5;
        locals.var_gem_dn6 = assign5480_e5538_d_n6;
        locals.var_gem_dn7 = assign5480_e5538_d_n7;
        locals.var_gem_dn8 = assign5480_e5538_d_n8;
        locals.var_gem_dn9 = assign5480_e5538_d_n9;

        let assign5490_e5541: f64 = if locals.var_gem > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign5490_e5541;

        let assign5500_e5544: f64 = if p.p52 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign5500_e5544;

        let (assign5510_e5570, assign5510_e5570_d_n0, assign5510_e5570_d_n1, assign5510_e5570_d_n3, assign5510_e5570_d_n4, assign5510_e5570_d_n5, assign5510_e5570_d_n6, assign5510_e5570_d_n7, assign5510_e5570_d_n8, assign5510_e5570_d_n9,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) {
        let assign5510_e5554: f64 = (locals.var_rbc_t + locals.var_rb2);
        let assign5510_e5555: f64 = (locals.var_in_ * assign5510_e5554);
        let assign5510_e5556: f64 = (locals.var_vt / assign5510_e5555);
        let assign5510_e5559: f64 = (locals.var_qbi / locals.var_is_t);
        let assign5510_e5561: f64 = (assign5510_e5559 * locals.var_ibi_t);
        let assign5510_e5562: f64 = (assign5510_e5556 + assign5510_e5561);
        let assign5510_e5566: f64 = (locals.var_rbc_t + locals.var_rb2);
        let assign5510_e5567: f64 = (locals.var_re_t / assign5510_e5566);
        let assign5510_e5568: f64 = (assign5510_e5562 + assign5510_e5567);
        (assign5510_e5568, (((-((locals.var_vt * ((locals.var_in__dn0 * assign5510_e5554) + (locals.var_in_ * locals.var_rb2_dn0))) / (assign5510_e5555 * assign5510_e5555))) + ((((locals.var_qbi_dn0 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn0)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn0) / (assign5510_e5566 * assign5510_e5566)))), (((-((locals.var_vt * ((locals.var_in__dn1 * assign5510_e5554) + (locals.var_in_ * locals.var_rb2_dn1))) / (assign5510_e5555 * assign5510_e5555))) + ((((locals.var_qbi_dn1 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn1)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn1) / (assign5510_e5566 * assign5510_e5566)))), (((-((locals.var_vt * ((locals.var_in__dn3 * assign5510_e5554) + (locals.var_in_ * locals.var_rb2_dn3))) / (assign5510_e5555 * assign5510_e5555))) + ((((locals.var_qbi_dn3 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn3)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn3) / (assign5510_e5566 * assign5510_e5566)))), (((-((locals.var_vt * ((locals.var_in__dn4 * assign5510_e5554) + (locals.var_in_ * locals.var_rb2_dn4))) / (assign5510_e5555 * assign5510_e5555))) + ((((locals.var_qbi_dn4 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn4)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn4) / (assign5510_e5566 * assign5510_e5566)))), (((-((locals.var_vt * ((locals.var_in__dn5 * assign5510_e5554) + (locals.var_in_ * locals.var_rb2_dn5))) / (assign5510_e5555 * assign5510_e5555))) + ((((locals.var_qbi_dn5 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn5)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn5) / (assign5510_e5566 * assign5510_e5566)))), (((-((locals.var_vt * ((locals.var_in__dn6 * assign5510_e5554) + (locals.var_in_ * locals.var_rb2_dn6))) / (assign5510_e5555 * assign5510_e5555))) + ((((locals.var_qbi_dn6 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn6)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn6) / (assign5510_e5566 * assign5510_e5566)))), (((-((locals.var_vt * ((locals.var_in__dn7 * assign5510_e5554) + (locals.var_in_ * locals.var_rb2_dn7))) / (assign5510_e5555 * assign5510_e5555))) + ((((locals.var_qbi_dn7 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn7)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn7) / (assign5510_e5566 * assign5510_e5566)))), (((-((locals.var_vt * ((locals.var_in__dn8 * assign5510_e5554) + (locals.var_in_ * locals.var_rb2_dn8))) / (assign5510_e5555 * assign5510_e5555))) + ((((locals.var_qbi_dn8 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn8)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn8) / (assign5510_e5566 * assign5510_e5566)))), (((-((locals.var_vt * ((locals.var_in__dn9 * assign5510_e5554) + (locals.var_in_ * locals.var_rb2_dn9))) / (assign5510_e5555 * assign5510_e5555))) + ((((locals.var_qbi_dn9 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn9)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn9) / (assign5510_e5566 * assign5510_e5566)))),)
    } else {
        (locals.var_gmax, locals.var_gmax_dn0, locals.var_gmax_dn1, locals.var_gmax_dn3, locals.var_gmax_dn4, locals.var_gmax_dn5, locals.var_gmax_dn6, locals.var_gmax_dn7, locals.var_gmax_dn8, locals.var_gmax_dn9,)
    }
};
        locals.var_gmax = assign5510_e5570;
        locals.var_gmax_dn0 = assign5510_e5570_d_n0;
        locals.var_gmax_dn1 = assign5510_e5570_d_n1;
        locals.var_gmax_dn3 = assign5510_e5570_d_n3;
        locals.var_gmax_dn4 = assign5510_e5570_d_n4;
        locals.var_gmax_dn5 = assign5510_e5570_d_n5;
        locals.var_gmax_dn6 = assign5510_e5570_d_n6;
        locals.var_gmax_dn7 = assign5510_e5570_d_n7;
        locals.var_gmax_dn8 = assign5510_e5570_d_n8;
        locals.var_gmax_dn9 = assign5510_e5570_d_n9;

        let assign5520_e5573: f64 = if p.p38 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard100 = assign5520_e5573;

        let (assign5530_e5587, assign5530_e5587_d_n0, assign5530_e5587_d_n1, assign5530_e5587_d_n3, assign5530_e5587_d_n4, assign5530_e5587_d_n5, assign5530_e5587_d_n6, assign5530_e5587_d_n7, assign5530_e5587_d_n8, assign5530_e5587_d_n9,) = {
    if ((((locals.var_guard83 != 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) && (locals.var_guard100 != 0.0)) {
        let assign5530_e5583: f64 = (locals.var_gem - locals.var_gmax);
        let assign5530_e5585: f64 = (assign5530_e5583 / 1e-6);
        (assign5530_e5585, ((locals.var_gem_dn0 - locals.var_gmax_dn0) / 1e-6), ((locals.var_gem_dn1 - locals.var_gmax_dn1) / 1e-6), ((locals.var_gem_dn3 - locals.var_gmax_dn3) / 1e-6), ((locals.var_gem_dn4 - locals.var_gmax_dn4) / 1e-6), ((locals.var_gem_dn5 - locals.var_gmax_dn5) / 1e-6), ((locals.var_gem_dn6 - locals.var_gmax_dn6) / 1e-6), ((locals.var_gem_dn7 - locals.var_gmax_dn7) / 1e-6), ((locals.var_gem_dn8 - locals.var_gmax_dn8) / 1e-6), ((locals.var_gem_dn9 - locals.var_gmax_dn9) / 1e-6),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9,)
    }
};
        locals.var_dxa = assign5530_e5587;
        locals.var_dxa_dn0 = assign5530_e5587_d_n0;
        locals.var_dxa_dn1 = assign5530_e5587_d_n1;
        locals.var_dxa_dn3 = assign5530_e5587_d_n3;
        locals.var_dxa_dn4 = assign5530_e5587_d_n4;
        locals.var_dxa_dn5 = assign5530_e5587_d_n5;
        locals.var_dxa_dn6 = assign5530_e5587_d_n6;
        locals.var_dxa_dn7 = assign5530_e5587_d_n7;
        locals.var_dxa_dn8 = assign5530_e5587_d_n8;
        locals.var_dxa_dn9 = assign5530_e5587_d_n9;

        let assign5540_e5590: f64 = if locals.var_gem < locals.var_gmax { 1.0 } else { 0.0 };
        locals.var_guard101 = assign5540_e5590;

        let (assign5550_e5610, assign5550_e5610_d_n0, assign5550_e5610_d_n1, assign5550_e5610_d_n3, assign5550_e5610_d_n4, assign5550_e5610_d_n5, assign5550_e5610_d_n6, assign5550_e5610_d_n7, assign5550_e5610_d_n8, assign5550_e5610_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) && (locals.var_guard100 != 0.0)) && (locals.var_guard101 != 0.0)) {
        let assign5550_e5604: f64 = (locals.var_dxa).exp();
        let assign5550_e5605: f64 = (1.0 + assign5550_e5604);
        let assign5550_e5606: f64 = (assign5550_e5605).ln();
        let assign5550_e5607: f64 = (1e-6 * assign5550_e5606);
        let assign5550_e5608: f64 = (locals.var_gem - assign5550_e5607);
        (assign5550_e5608, (locals.var_gem_dn0 - (1e-6 * ((assign5550_e5604 * locals.var_dxa_dn0) / assign5550_e5605))), (locals.var_gem_dn1 - (1e-6 * ((assign5550_e5604 * locals.var_dxa_dn1) / assign5550_e5605))), (locals.var_gem_dn3 - (1e-6 * ((assign5550_e5604 * locals.var_dxa_dn3) / assign5550_e5605))), (locals.var_gem_dn4 - (1e-6 * ((assign5550_e5604 * locals.var_dxa_dn4) / assign5550_e5605))), (locals.var_gem_dn5 - (1e-6 * ((assign5550_e5604 * locals.var_dxa_dn5) / assign5550_e5605))), (locals.var_gem_dn6 - (1e-6 * ((assign5550_e5604 * locals.var_dxa_dn6) / assign5550_e5605))), (locals.var_gem_dn7 - (1e-6 * ((assign5550_e5604 * locals.var_dxa_dn7) / assign5550_e5605))), (locals.var_gem_dn8 - (1e-6 * ((assign5550_e5604 * locals.var_dxa_dn8) / assign5550_e5605))), (locals.var_gem_dn9 - (1e-6 * ((assign5550_e5604 * locals.var_dxa_dn9) / assign5550_e5605))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9,)
    }
};
        locals.var_gem = assign5550_e5610;
        locals.var_gem_dn0 = assign5550_e5610_d_n0;
        locals.var_gem_dn1 = assign5550_e5610_d_n1;
        locals.var_gem_dn3 = assign5550_e5610_d_n3;
        locals.var_gem_dn4 = assign5550_e5610_d_n4;
        locals.var_gem_dn5 = assign5550_e5610_d_n5;
        locals.var_gem_dn6 = assign5550_e5610_d_n6;
        locals.var_gem_dn7 = assign5550_e5610_d_n7;
        locals.var_gem_dn8 = assign5550_e5610_d_n8;
        locals.var_gem_dn9 = assign5550_e5610_d_n9;

        let (assign5560_e5632, assign5560_e5632_d_n0, assign5560_e5632_d_n1, assign5560_e5632_d_n3, assign5560_e5632_d_n4, assign5560_e5632_d_n5, assign5560_e5632_d_n6, assign5560_e5632_d_n7, assign5560_e5632_d_n8, assign5560_e5632_d_n9,) = {
    if (((((locals.var_guard83 != 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) && (locals.var_guard100 != 0.0)) && (locals.var_guard101 == 0.0)) {
        let assign5560_e5625: f64 = (-locals.var_dxa);
        let assign5560_e5626: f64 = (assign5560_e5625).exp();
        let assign5560_e5627: f64 = (1.0 + assign5560_e5626);
        let assign5560_e5628: f64 = (assign5560_e5627).ln();
        let assign5560_e5629: f64 = (1e-6 * assign5560_e5628);
        let assign5560_e5630: f64 = (locals.var_gmax - assign5560_e5629);
        (assign5560_e5630, (locals.var_gmax_dn0 - (1e-6 * ((assign5560_e5626 * (-locals.var_dxa_dn0)) / assign5560_e5627))), (locals.var_gmax_dn1 - (1e-6 * ((assign5560_e5626 * (-locals.var_dxa_dn1)) / assign5560_e5627))), (locals.var_gmax_dn3 - (1e-6 * ((assign5560_e5626 * (-locals.var_dxa_dn3)) / assign5560_e5627))), (locals.var_gmax_dn4 - (1e-6 * ((assign5560_e5626 * (-locals.var_dxa_dn4)) / assign5560_e5627))), (locals.var_gmax_dn5 - (1e-6 * ((assign5560_e5626 * (-locals.var_dxa_dn5)) / assign5560_e5627))), (locals.var_gmax_dn6 - (1e-6 * ((assign5560_e5626 * (-locals.var_dxa_dn6)) / assign5560_e5627))), (locals.var_gmax_dn7 - (1e-6 * ((assign5560_e5626 * (-locals.var_dxa_dn7)) / assign5560_e5627))), (locals.var_gmax_dn8 - (1e-6 * ((assign5560_e5626 * (-locals.var_dxa_dn8)) / assign5560_e5627))), (locals.var_gmax_dn9 - (1e-6 * ((assign5560_e5626 * (-locals.var_dxa_dn9)) / assign5560_e5627))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9,)
    }
};
        locals.var_gem = assign5560_e5632;
        locals.var_gem_dn0 = assign5560_e5632_d_n0;
        locals.var_gem_dn1 = assign5560_e5632_d_n1;
        locals.var_gem_dn3 = assign5560_e5632_d_n3;
        locals.var_gem_dn4 = assign5560_e5632_d_n4;
        locals.var_gem_dn5 = assign5560_e5632_d_n5;
        locals.var_gem_dn6 = assign5560_e5632_d_n6;
        locals.var_gem_dn7 = assign5560_e5632_d_n7;
        locals.var_gem_dn8 = assign5560_e5632_d_n8;
        locals.var_gem_dn9 = assign5560_e5632_d_n9;

        let assign5630_e5692: f64 = (1.0 - p.p67);
        let assign5630_e5694: f64 = (assign5630_e5692 * locals.var_cje_t);
        let assign5630_e5696: f64 = (assign5630_e5694 * locals.var_vte);
        locals.var_qte = assign5630_e5696;
        locals.var_qte_dn0 = (((assign5630_e5692 * locals.var_cje_t_dn0) * locals.var_vte) + (assign5630_e5694 * locals.var_vte_dn0));
        locals.var_qte_dn1 = (((assign5630_e5692 * locals.var_cje_t_dn1) * locals.var_vte) + (assign5630_e5694 * locals.var_vte_dn1));
        locals.var_qte_dn3 = (((assign5630_e5692 * locals.var_cje_t_dn3) * locals.var_vte) + (assign5630_e5694 * locals.var_vte_dn3));
        locals.var_qte_dn4 = (((assign5630_e5692 * locals.var_cje_t_dn4) * locals.var_vte) + (assign5630_e5694 * locals.var_vte_dn4));
        locals.var_qte_dn5 = (((assign5630_e5692 * locals.var_cje_t_dn5) * locals.var_vte) + (assign5630_e5694 * locals.var_vte_dn5));
        locals.var_qte_dn6 = (((assign5630_e5692 * locals.var_cje_t_dn6) * locals.var_vte) + (assign5630_e5694 * locals.var_vte_dn6));
        locals.var_qte_dn7 = (((assign5630_e5692 * locals.var_cje_t_dn7) * locals.var_vte) + (assign5630_e5694 * locals.var_vte_dn7));
        locals.var_qte_dn8 = (((assign5630_e5692 * locals.var_cje_t_dn8) * locals.var_vte) + (assign5630_e5694 * locals.var_vte_dn8));
        locals.var_qte_dn9 = (((assign5630_e5692 * locals.var_cje_t_dn9) * locals.var_vte) + (assign5630_e5694 * locals.var_vte_dn9));

        let assign5640_e5699: f64 = (locals.var_vb1e1 - locals.var_vfe);
        let assign5640_e5701: f64 = (assign5640_e5699 / locals.var_a_vde);
        locals.var_dxa = assign5640_e5701;
        locals.var_dxa_dn0 = ((((-locals.var_vfe_dn0) * locals.var_a_vde) - (assign5640_e5699 * locals.var_a_vde_dn0)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn1 = ((((-locals.var_vfe_dn1) * locals.var_a_vde) - (assign5640_e5699 * locals.var_a_vde_dn1)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn3 = ((((locals.var_vb1e1_dn3 - locals.var_vfe_dn3) * locals.var_a_vde) - (assign5640_e5699 * locals.var_a_vde_dn3)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn4 = ((((locals.var_vb1e1_dn4 - locals.var_vfe_dn4) * locals.var_a_vde) - (assign5640_e5699 * locals.var_a_vde_dn4)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn5 = ((((-locals.var_vfe_dn5) * locals.var_a_vde) - (assign5640_e5699 * locals.var_a_vde_dn5)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn6 = ((((-locals.var_vfe_dn6) * locals.var_a_vde) - (assign5640_e5699 * locals.var_a_vde_dn6)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn7 = ((((-locals.var_vfe_dn7) * locals.var_a_vde) - (assign5640_e5699 * locals.var_a_vde_dn7)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn8 = ((((-locals.var_vfe_dn8) * locals.var_a_vde) - (assign5640_e5699 * locals.var_a_vde_dn8)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn9 = ((((-locals.var_vfe_dn9) * locals.var_a_vde) - (assign5640_e5699 * locals.var_a_vde_dn9)) / (locals.var_a_vde * locals.var_a_vde));

        let assign5650_e5704: f64 = if locals.var_vb1e1 < locals.var_vfe { 1.0 } else { 0.0 };
        locals.var_guard103 = assign5650_e5704;

        let (assign5660_e5716, assign5660_e5716_d_n0, assign5660_e5716_d_n1, assign5660_e5716_d_n3, assign5660_e5716_d_n4, assign5660_e5716_d_n5, assign5660_e5716_d_n6, assign5660_e5716_d_n7, assign5660_e5716_d_n8, assign5660_e5716_d_n9,) = {
    if (locals.var_guard103 != 0.0) {
        let assign5660_e5710: f64 = (locals.var_dxa).exp();
        let assign5660_e5711: f64 = (1.0 + assign5660_e5710);
        let assign5660_e5712: f64 = (assign5660_e5711).ln();
        let assign5660_e5713: f64 = (locals.var_a_vde * assign5660_e5712);
        let assign5660_e5714: f64 = (locals.var_vb1e1 - assign5660_e5713);
        (assign5660_e5714, (-((locals.var_a_vde_dn0 * assign5660_e5712) + (locals.var_a_vde * ((assign5660_e5710 * locals.var_dxa_dn0) / assign5660_e5711)))), (-((locals.var_a_vde_dn1 * assign5660_e5712) + (locals.var_a_vde * ((assign5660_e5710 * locals.var_dxa_dn1) / assign5660_e5711)))), (locals.var_vb1e1_dn3 - ((locals.var_a_vde_dn3 * assign5660_e5712) + (locals.var_a_vde * ((assign5660_e5710 * locals.var_dxa_dn3) / assign5660_e5711)))), (locals.var_vb1e1_dn4 - ((locals.var_a_vde_dn4 * assign5660_e5712) + (locals.var_a_vde * ((assign5660_e5710 * locals.var_dxa_dn4) / assign5660_e5711)))), (-((locals.var_a_vde_dn5 * assign5660_e5712) + (locals.var_a_vde * ((assign5660_e5710 * locals.var_dxa_dn5) / assign5660_e5711)))), (-((locals.var_a_vde_dn6 * assign5660_e5712) + (locals.var_a_vde * ((assign5660_e5710 * locals.var_dxa_dn6) / assign5660_e5711)))), (-((locals.var_a_vde_dn7 * assign5660_e5712) + (locals.var_a_vde * ((assign5660_e5710 * locals.var_dxa_dn7) / assign5660_e5711)))), (-((locals.var_a_vde_dn8 * assign5660_e5712) + (locals.var_a_vde * ((assign5660_e5710 * locals.var_dxa_dn8) / assign5660_e5711)))), (-((locals.var_a_vde_dn9 * assign5660_e5712) + (locals.var_a_vde * ((assign5660_e5710 * locals.var_dxa_dn9) / assign5660_e5711)))),)
    } else {
        (locals.var_vje_s, locals.var_vje_s_dn0, locals.var_vje_s_dn1, locals.var_vje_s_dn3, locals.var_vje_s_dn4, locals.var_vje_s_dn5, locals.var_vje_s_dn6, locals.var_vje_s_dn7, locals.var_vje_s_dn8, locals.var_vje_s_dn9,)
    }
};
        locals.var_vje_s = assign5660_e5716;
        locals.var_vje_s_dn0 = assign5660_e5716_d_n0;
        locals.var_vje_s_dn1 = assign5660_e5716_d_n1;
        locals.var_vje_s_dn3 = assign5660_e5716_d_n3;
        locals.var_vje_s_dn4 = assign5660_e5716_d_n4;
        locals.var_vje_s_dn5 = assign5660_e5716_d_n5;
        locals.var_vje_s_dn6 = assign5660_e5716_d_n6;
        locals.var_vje_s_dn7 = assign5660_e5716_d_n7;
        locals.var_vje_s_dn8 = assign5660_e5716_d_n8;
        locals.var_vje_s_dn9 = assign5660_e5716_d_n9;

        let (assign5670_e5730, assign5670_e5730_d_n0, assign5670_e5730_d_n1, assign5670_e5730_d_n3, assign5670_e5730_d_n4, assign5670_e5730_d_n5, assign5670_e5730_d_n6, assign5670_e5730_d_n7, assign5670_e5730_d_n8, assign5670_e5730_d_n9,) = {
    if (locals.var_guard103 == 0.0) {
        let assign5670_e5723: f64 = (-locals.var_dxa);
        let assign5670_e5724: f64 = (assign5670_e5723).exp();
        let assign5670_e5725: f64 = (1.0 + assign5670_e5724);
        let assign5670_e5726: f64 = (assign5670_e5725).ln();
        let assign5670_e5727: f64 = (locals.var_a_vde * assign5670_e5726);
        let assign5670_e5728: f64 = (locals.var_vfe - assign5670_e5727);
        (assign5670_e5728, (locals.var_vfe_dn0 - ((locals.var_a_vde_dn0 * assign5670_e5726) + (locals.var_a_vde * ((assign5670_e5724 * (-locals.var_dxa_dn0)) / assign5670_e5725)))), (locals.var_vfe_dn1 - ((locals.var_a_vde_dn1 * assign5670_e5726) + (locals.var_a_vde * ((assign5670_e5724 * (-locals.var_dxa_dn1)) / assign5670_e5725)))), (locals.var_vfe_dn3 - ((locals.var_a_vde_dn3 * assign5670_e5726) + (locals.var_a_vde * ((assign5670_e5724 * (-locals.var_dxa_dn3)) / assign5670_e5725)))), (locals.var_vfe_dn4 - ((locals.var_a_vde_dn4 * assign5670_e5726) + (locals.var_a_vde * ((assign5670_e5724 * (-locals.var_dxa_dn4)) / assign5670_e5725)))), (locals.var_vfe_dn5 - ((locals.var_a_vde_dn5 * assign5670_e5726) + (locals.var_a_vde * ((assign5670_e5724 * (-locals.var_dxa_dn5)) / assign5670_e5725)))), (locals.var_vfe_dn6 - ((locals.var_a_vde_dn6 * assign5670_e5726) + (locals.var_a_vde * ((assign5670_e5724 * (-locals.var_dxa_dn6)) / assign5670_e5725)))), (locals.var_vfe_dn7 - ((locals.var_a_vde_dn7 * assign5670_e5726) + (locals.var_a_vde * ((assign5670_e5724 * (-locals.var_dxa_dn7)) / assign5670_e5725)))), (locals.var_vfe_dn8 - ((locals.var_a_vde_dn8 * assign5670_e5726) + (locals.var_a_vde * ((assign5670_e5724 * (-locals.var_dxa_dn8)) / assign5670_e5725)))), (locals.var_vfe_dn9 - ((locals.var_a_vde_dn9 * assign5670_e5726) + (locals.var_a_vde * ((assign5670_e5724 * (-locals.var_dxa_dn9)) / assign5670_e5725)))),)
    } else {
        (locals.var_vje_s, locals.var_vje_s_dn0, locals.var_vje_s_dn1, locals.var_vje_s_dn3, locals.var_vje_s_dn4, locals.var_vje_s_dn5, locals.var_vje_s_dn6, locals.var_vje_s_dn7, locals.var_vje_s_dn8, locals.var_vje_s_dn9,)
    }
};
        locals.var_vje_s = assign5670_e5730;
        locals.var_vje_s_dn0 = assign5670_e5730_d_n0;
        locals.var_vje_s_dn1 = assign5670_e5730_d_n1;
        locals.var_vje_s_dn3 = assign5670_e5730_d_n3;
        locals.var_vje_s_dn4 = assign5670_e5730_d_n4;
        locals.var_vje_s_dn5 = assign5670_e5730_d_n5;
        locals.var_vje_s_dn6 = assign5670_e5730_d_n6;
        locals.var_vje_s_dn7 = assign5670_e5730_d_n7;
        locals.var_vje_s_dn8 = assign5670_e5730_d_n8;
        locals.var_vje_s_dn9 = assign5670_e5730_d_n9;

    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign5680_e5733: f64 = (p.p67 * locals.var_cje_t);
        let assign5680_e5737: f64 = (1.0 - p.p66);
        let assign5680_e5738: f64 = (locals.var_vde_t / assign5680_e5737);
        let assign5680_e5743: f64 = (locals.var_vje_s * locals.var_inv_vde_t);
        let assign5680_e5744: f64 = (1.0 - assign5680_e5743);
        let assign5680_e5747: f64 = (1.0 - p.p66);
        let assign5680_e5748: f64 = (assign5680_e5744).powf(assign5680_e5747);
        let assign5680_e5749: f64 = (1.0 - assign5680_e5748);
        let assign5680_e5750: f64 = (assign5680_e5738 * assign5680_e5749);
        let assign5680_e5754: f64 = (locals.var_vb1e1 - locals.var_vje_s);
        let assign5680_e5755: f64 = (3.0 * assign5680_e5754);
        let assign5680_e5756: f64 = (assign5680_e5750 + assign5680_e5755);
        let assign5680_e5757: f64 = (assign5680_e5733 * assign5680_e5756);
        locals.var_qte_s = assign5680_e5757;
        locals.var_qte_s_dn0 = (((p.p67 * locals.var_cje_t_dn0) * assign5680_e5756) + (assign5680_e5733 * ((((locals.var_vde_t_dn0 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((locals.var_vje_s_dn0 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn0))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((locals.var_vje_s_dn0 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn0))) / assign5680_e5744))) }))) + (3.0 * (-locals.var_vje_s_dn0)))));
        locals.var_qte_s_dn1 = (((p.p67 * locals.var_cje_t_dn1) * assign5680_e5756) + (assign5680_e5733 * ((((locals.var_vde_t_dn1 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((locals.var_vje_s_dn1 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn1))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((locals.var_vje_s_dn1 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn1))) / assign5680_e5744))) }))) + (3.0 * (-locals.var_vje_s_dn1)))));
        locals.var_qte_s_dn3 = (((p.p67 * locals.var_cje_t_dn3) * assign5680_e5756) + (assign5680_e5733 * ((((locals.var_vde_t_dn3 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((locals.var_vje_s_dn3 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn3))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((locals.var_vje_s_dn3 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn3))) / assign5680_e5744))) }))) + (3.0 * (locals.var_vb1e1_dn3 - locals.var_vje_s_dn3)))));
        locals.var_qte_s_dn4 = (((p.p67 * locals.var_cje_t_dn4) * assign5680_e5756) + (assign5680_e5733 * ((((locals.var_vde_t_dn4 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((locals.var_vje_s_dn4 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn4))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((locals.var_vje_s_dn4 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn4))) / assign5680_e5744))) }))) + (3.0 * (locals.var_vb1e1_dn4 - locals.var_vje_s_dn4)))));
        locals.var_qte_s_dn5 = (((p.p67 * locals.var_cje_t_dn5) * assign5680_e5756) + (assign5680_e5733 * ((((locals.var_vde_t_dn5 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((locals.var_vje_s_dn5 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn5))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((locals.var_vje_s_dn5 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn5))) / assign5680_e5744))) }))) + (3.0 * (-locals.var_vje_s_dn5)))));
        locals.var_qte_s_dn6 = (((p.p67 * locals.var_cje_t_dn6) * assign5680_e5756) + (assign5680_e5733 * ((((locals.var_vde_t_dn6 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((locals.var_vje_s_dn6 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn6))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((locals.var_vje_s_dn6 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn6))) / assign5680_e5744))) }))) + (3.0 * (-locals.var_vje_s_dn6)))));
        locals.var_qte_s_dn7 = (((p.p67 * locals.var_cje_t_dn7) * assign5680_e5756) + (assign5680_e5733 * ((((locals.var_vde_t_dn7 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((locals.var_vje_s_dn7 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn7))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((locals.var_vje_s_dn7 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn7))) / assign5680_e5744))) }))) + (3.0 * (-locals.var_vje_s_dn7)))));
        locals.var_qte_s_dn8 = (((p.p67 * locals.var_cje_t_dn8) * assign5680_e5756) + (assign5680_e5733 * ((((locals.var_vde_t_dn8 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((locals.var_vje_s_dn8 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn8))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((locals.var_vje_s_dn8 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn8))) / assign5680_e5744))) }))) + (3.0 * (-locals.var_vje_s_dn8)))));
        locals.var_qte_s_dn9 = (((p.p67 * locals.var_cje_t_dn9) * assign5680_e5756) + (assign5680_e5733 * ((((locals.var_vde_t_dn9 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((locals.var_vje_s_dn9 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn9))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((locals.var_vje_s_dn9 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn9))) / assign5680_e5744))) }))) + (3.0 * (-locals.var_vje_s_dn9)))));

        let assign5690_e5760: f64 = (p.p76 * locals.var_cjc_t);
        let assign5690_e5762: f64 = (assign5690_e5760 * locals.var_vtc);
        locals.var_qtc = assign5690_e5762;
        locals.var_qtc_dn0 = (((p.p76 * locals.var_cjc_t_dn0) * locals.var_vtc) + (assign5690_e5760 * locals.var_vtc_dn0));
        locals.var_qtc_dn1 = (((p.p76 * locals.var_cjc_t_dn1) * locals.var_vtc) + (assign5690_e5760 * locals.var_vtc_dn1));
        locals.var_qtc_dn3 = (((p.p76 * locals.var_cjc_t_dn3) * locals.var_vtc) + (assign5690_e5760 * locals.var_vtc_dn3));
        locals.var_qtc_dn4 = (((p.p76 * locals.var_cjc_t_dn4) * locals.var_vtc) + (assign5690_e5760 * locals.var_vtc_dn4));
        locals.var_qtc_dn5 = (((p.p76 * locals.var_cjc_t_dn5) * locals.var_vtc) + (assign5690_e5760 * locals.var_vtc_dn5));
        locals.var_qtc_dn6 = (((p.p76 * locals.var_cjc_t_dn6) * locals.var_vtc) + (assign5690_e5760 * locals.var_vtc_dn6));
        locals.var_qtc_dn7 = (((p.p76 * locals.var_cjc_t_dn7) * locals.var_vtc) + (assign5690_e5760 * locals.var_vtc_dn7));
        locals.var_qtc_dn8 = (((p.p76 * locals.var_cjc_t_dn8) * locals.var_vtc) + (assign5690_e5760 * locals.var_vtc_dn8));
        locals.var_qtc_dn9 = (((p.p76 * locals.var_cjc_t_dn9) * locals.var_vtc) + (assign5690_e5760 * locals.var_vtc_dn9));

        let assign5700_e5765: f64 = (locals.var_taub_t * locals.var_ik_t);
        locals.var_qb0 = assign5700_e5765;

        let assign5710_e5768: f64 = (0.5 * locals.var_qb0);
        let assign5710_e5770: f64 = (assign5710_e5768 * locals.var_n0);
        let assign5710_e5772: f64 = (assign5710_e5770 * locals.var_q1q);
        locals.var_qbe_qs = assign5710_e5772;
        locals.var_qbe_qs_dn0 = (((assign5710_e5768 * locals.var_n0_dn0) * locals.var_q1q) + (assign5710_e5770 * locals.var_q1q_dn0));
        locals.var_qbe_qs_dn1 = (((assign5710_e5768 * locals.var_n0_dn1) * locals.var_q1q) + (assign5710_e5770 * locals.var_q1q_dn1));
        locals.var_qbe_qs_dn3 = (((assign5710_e5768 * locals.var_n0_dn3) * locals.var_q1q) + (assign5710_e5770 * locals.var_q1q_dn3));
        locals.var_qbe_qs_dn4 = (((assign5710_e5768 * locals.var_n0_dn4) * locals.var_q1q) + (assign5710_e5770 * locals.var_q1q_dn4));
        locals.var_qbe_qs_dn5 = (((assign5710_e5768 * locals.var_n0_dn5) * locals.var_q1q) + (assign5710_e5770 * locals.var_q1q_dn5));
        locals.var_qbe_qs_dn6 = (((assign5710_e5768 * locals.var_n0_dn6) * locals.var_q1q) + (assign5710_e5770 * locals.var_q1q_dn6));
        locals.var_qbe_qs_dn7 = (((assign5710_e5768 * locals.var_n0_dn7) * locals.var_q1q) + (assign5710_e5770 * locals.var_q1q_dn7));
        locals.var_qbe_qs_dn8 = (((assign5710_e5768 * locals.var_n0_dn8) * locals.var_q1q) + (assign5710_e5770 * locals.var_q1q_dn8));
        locals.var_qbe_qs_dn9 = (((assign5710_e5768 * locals.var_n0_dn9) * locals.var_q1q) + (assign5710_e5770 * locals.var_q1q_dn9));

        let assign5720_e5775: f64 = (0.5 * locals.var_qb0);
        let assign5720_e5777: f64 = (assign5720_e5775 * locals.var_nb);
        let assign5720_e5779: f64 = (assign5720_e5777 * locals.var_q1q);
        locals.var_qbc_qs = assign5720_e5779;
        locals.var_qbc_qs_dn0 = (((assign5720_e5775 * locals.var_nb_dn0) * locals.var_q1q) + (assign5720_e5777 * locals.var_q1q_dn0));
        locals.var_qbc_qs_dn1 = (((assign5720_e5775 * locals.var_nb_dn1) * locals.var_q1q) + (assign5720_e5777 * locals.var_q1q_dn1));
        locals.var_qbc_qs_dn3 = (((assign5720_e5775 * locals.var_nb_dn3) * locals.var_q1q) + (assign5720_e5777 * locals.var_q1q_dn3));
        locals.var_qbc_qs_dn4 = (((assign5720_e5775 * locals.var_nb_dn4) * locals.var_q1q) + (assign5720_e5777 * locals.var_q1q_dn4));
        locals.var_qbc_qs_dn5 = (((assign5720_e5775 * locals.var_nb_dn5) * locals.var_q1q) + (assign5720_e5777 * locals.var_q1q_dn5));
        locals.var_qbc_qs_dn6 = (((assign5720_e5775 * locals.var_nb_dn6) * locals.var_q1q) + (assign5720_e5777 * locals.var_q1q_dn6));
        locals.var_qbc_qs_dn7 = (((assign5720_e5775 * locals.var_nb_dn7) * locals.var_q1q) + (assign5720_e5777 * locals.var_q1q_dn7));
        locals.var_qbc_qs_dn8 = (((assign5720_e5775 * locals.var_nb_dn8) * locals.var_q1q) + (assign5720_e5777 * locals.var_q1q_dn8));
        locals.var_qbc_qs_dn9 = (((assign5720_e5775 * locals.var_nb_dn9) * locals.var_q1q) + (assign5720_e5777 * locals.var_q1q_dn9));

        let assign5730_e5782: f64 = (0.1 * locals.var_vdc_ctc_t);
        locals.var_a_vdcctc = assign5730_e5782;
        locals.var_a_vdcctc_dn0 = (0.1 * locals.var_vdc_ctc_t_dn0);
        locals.var_a_vdcctc_dn1 = (0.1 * locals.var_vdc_ctc_t_dn1);
        locals.var_a_vdcctc_dn3 = (0.1 * locals.var_vdc_ctc_t_dn3);
        locals.var_a_vdcctc_dn4 = (0.1 * locals.var_vdc_ctc_t_dn4);
        locals.var_a_vdcctc_dn5 = (0.1 * locals.var_vdc_ctc_t_dn5);
        locals.var_a_vdcctc_dn6 = (0.1 * locals.var_vdc_ctc_t_dn6);
        locals.var_a_vdcctc_dn7 = (0.1 * locals.var_vdc_ctc_t_dn7);
        locals.var_a_vdcctc_dn8 = (0.1 * locals.var_vdc_ctc_t_dn8);
        locals.var_a_vdcctc_dn9 = (0.1 * locals.var_vdc_ctc_t_dn9);

        let assign5740_e5785: f64 = (locals.var_vb1c4 - locals.var_vfc);
        let assign5740_e5787: f64 = (assign5740_e5785 / locals.var_a_vdcctc);
        locals.var_dxa = assign5740_e5787;
        locals.var_dxa_dn0 = ((((-locals.var_vfc_dn0) * locals.var_a_vdcctc) - (assign5740_e5785 * locals.var_a_vdcctc_dn0)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn1 = ((((-locals.var_vfc_dn1) * locals.var_a_vdcctc) - (assign5740_e5785 * locals.var_a_vdcctc_dn1)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn3 = ((((-locals.var_vfc_dn3) * locals.var_a_vdcctc) - (assign5740_e5785 * locals.var_a_vdcctc_dn3)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn4 = ((((locals.var_vb1c4_dn4 - locals.var_vfc_dn4) * locals.var_a_vdcctc) - (assign5740_e5785 * locals.var_a_vdcctc_dn4)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn5 = ((((locals.var_vb1c4_dn5 - locals.var_vfc_dn5) * locals.var_a_vdcctc) - (assign5740_e5785 * locals.var_a_vdcctc_dn5)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn6 = ((((locals.var_vb1c4_dn6 - locals.var_vfc_dn6) * locals.var_a_vdcctc) - (assign5740_e5785 * locals.var_a_vdcctc_dn6)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn7 = ((((locals.var_vb1c4_dn7 - locals.var_vfc_dn7) * locals.var_a_vdcctc) - (assign5740_e5785 * locals.var_a_vdcctc_dn7)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn8 = ((((-locals.var_vfc_dn8) * locals.var_a_vdcctc) - (assign5740_e5785 * locals.var_a_vdcctc_dn8)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn9 = ((((locals.var_vb1c4_dn9 - locals.var_vfc_dn9) * locals.var_a_vdcctc) - (assign5740_e5785 * locals.var_a_vdcctc_dn9)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));

        let assign5750_e5790: f64 = if locals.var_vb1c4 < locals.var_vfc { 1.0 } else { 0.0 };
        locals.var_guard104 = assign5750_e5790;

        let (assign5760_e5802, assign5760_e5802_d_n0, assign5760_e5802_d_n1, assign5760_e5802_d_n3, assign5760_e5802_d_n4, assign5760_e5802_d_n5, assign5760_e5802_d_n6, assign5760_e5802_d_n7, assign5760_e5802_d_n8, assign5760_e5802_d_n9,) = {
    if (locals.var_guard104 != 0.0) {
        let assign5760_e5796: f64 = (locals.var_dxa).exp();
        let assign5760_e5797: f64 = (1.0 + assign5760_e5796);
        let assign5760_e5798: f64 = (assign5760_e5797).ln();
        let assign5760_e5799: f64 = (locals.var_a_vdcctc * assign5760_e5798);
        let assign5760_e5800: f64 = (locals.var_vb1c4 - assign5760_e5799);
        (assign5760_e5800, (-((locals.var_a_vdcctc_dn0 * assign5760_e5798) + (locals.var_a_vdcctc * ((assign5760_e5796 * locals.var_dxa_dn0) / assign5760_e5797)))), (-((locals.var_a_vdcctc_dn1 * assign5760_e5798) + (locals.var_a_vdcctc * ((assign5760_e5796 * locals.var_dxa_dn1) / assign5760_e5797)))), (-((locals.var_a_vdcctc_dn3 * assign5760_e5798) + (locals.var_a_vdcctc * ((assign5760_e5796 * locals.var_dxa_dn3) / assign5760_e5797)))), (locals.var_vb1c4_dn4 - ((locals.var_a_vdcctc_dn4 * assign5760_e5798) + (locals.var_a_vdcctc * ((assign5760_e5796 * locals.var_dxa_dn4) / assign5760_e5797)))), (locals.var_vb1c4_dn5 - ((locals.var_a_vdcctc_dn5 * assign5760_e5798) + (locals.var_a_vdcctc * ((assign5760_e5796 * locals.var_dxa_dn5) / assign5760_e5797)))), (locals.var_vb1c4_dn6 - ((locals.var_a_vdcctc_dn6 * assign5760_e5798) + (locals.var_a_vdcctc * ((assign5760_e5796 * locals.var_dxa_dn6) / assign5760_e5797)))), (locals.var_vb1c4_dn7 - ((locals.var_a_vdcctc_dn7 * assign5760_e5798) + (locals.var_a_vdcctc * ((assign5760_e5796 * locals.var_dxa_dn7) / assign5760_e5797)))), (-((locals.var_a_vdcctc_dn8 * assign5760_e5798) + (locals.var_a_vdcctc * ((assign5760_e5796 * locals.var_dxa_dn8) / assign5760_e5797)))), (locals.var_vb1c4_dn9 - ((locals.var_a_vdcctc_dn9 * assign5760_e5798) + (locals.var_a_vdcctc * ((assign5760_e5796 * locals.var_dxa_dn9) / assign5760_e5797)))),)
    } else {
        (locals.var_vjcex, locals.var_vjcex_dn0, locals.var_vjcex_dn1, locals.var_vjcex_dn3, locals.var_vjcex_dn4, locals.var_vjcex_dn5, locals.var_vjcex_dn6, locals.var_vjcex_dn7, locals.var_vjcex_dn8, locals.var_vjcex_dn9,)
    }
};
        locals.var_vjcex = assign5760_e5802;
        locals.var_vjcex_dn0 = assign5760_e5802_d_n0;
        locals.var_vjcex_dn1 = assign5760_e5802_d_n1;
        locals.var_vjcex_dn3 = assign5760_e5802_d_n3;
        locals.var_vjcex_dn4 = assign5760_e5802_d_n4;
        locals.var_vjcex_dn5 = assign5760_e5802_d_n5;
        locals.var_vjcex_dn6 = assign5760_e5802_d_n6;
        locals.var_vjcex_dn7 = assign5760_e5802_d_n7;
        locals.var_vjcex_dn8 = assign5760_e5802_d_n8;
        locals.var_vjcex_dn9 = assign5760_e5802_d_n9;

        let (assign5770_e5816, assign5770_e5816_d_n0, assign5770_e5816_d_n1, assign5770_e5816_d_n3, assign5770_e5816_d_n4, assign5770_e5816_d_n5, assign5770_e5816_d_n6, assign5770_e5816_d_n7, assign5770_e5816_d_n8, assign5770_e5816_d_n9,) = {
    if (locals.var_guard104 == 0.0) {
        let assign5770_e5809: f64 = (-locals.var_dxa);
        let assign5770_e5810: f64 = (assign5770_e5809).exp();
        let assign5770_e5811: f64 = (1.0 + assign5770_e5810);
        let assign5770_e5812: f64 = (assign5770_e5811).ln();
        let assign5770_e5813: f64 = (locals.var_a_vdcctc * assign5770_e5812);
        let assign5770_e5814: f64 = (locals.var_vfc - assign5770_e5813);
        (assign5770_e5814, (locals.var_vfc_dn0 - ((locals.var_a_vdcctc_dn0 * assign5770_e5812) + (locals.var_a_vdcctc * ((assign5770_e5810 * (-locals.var_dxa_dn0)) / assign5770_e5811)))), (locals.var_vfc_dn1 - ((locals.var_a_vdcctc_dn1 * assign5770_e5812) + (locals.var_a_vdcctc * ((assign5770_e5810 * (-locals.var_dxa_dn1)) / assign5770_e5811)))), (locals.var_vfc_dn3 - ((locals.var_a_vdcctc_dn3 * assign5770_e5812) + (locals.var_a_vdcctc * ((assign5770_e5810 * (-locals.var_dxa_dn3)) / assign5770_e5811)))), (locals.var_vfc_dn4 - ((locals.var_a_vdcctc_dn4 * assign5770_e5812) + (locals.var_a_vdcctc * ((assign5770_e5810 * (-locals.var_dxa_dn4)) / assign5770_e5811)))), (locals.var_vfc_dn5 - ((locals.var_a_vdcctc_dn5 * assign5770_e5812) + (locals.var_a_vdcctc * ((assign5770_e5810 * (-locals.var_dxa_dn5)) / assign5770_e5811)))), (locals.var_vfc_dn6 - ((locals.var_a_vdcctc_dn6 * assign5770_e5812) + (locals.var_a_vdcctc * ((assign5770_e5810 * (-locals.var_dxa_dn6)) / assign5770_e5811)))), (locals.var_vfc_dn7 - ((locals.var_a_vdcctc_dn7 * assign5770_e5812) + (locals.var_a_vdcctc * ((assign5770_e5810 * (-locals.var_dxa_dn7)) / assign5770_e5811)))), (locals.var_vfc_dn8 - ((locals.var_a_vdcctc_dn8 * assign5770_e5812) + (locals.var_a_vdcctc * ((assign5770_e5810 * (-locals.var_dxa_dn8)) / assign5770_e5811)))), (locals.var_vfc_dn9 - ((locals.var_a_vdcctc_dn9 * assign5770_e5812) + (locals.var_a_vdcctc * ((assign5770_e5810 * (-locals.var_dxa_dn9)) / assign5770_e5811)))),)
    } else {
        (locals.var_vjcex, locals.var_vjcex_dn0, locals.var_vjcex_dn1, locals.var_vjcex_dn3, locals.var_vjcex_dn4, locals.var_vjcex_dn5, locals.var_vjcex_dn6, locals.var_vjcex_dn7, locals.var_vjcex_dn8, locals.var_vjcex_dn9,)
    }
};
        locals.var_vjcex = assign5770_e5816;
        locals.var_vjcex_dn0 = assign5770_e5816_d_n0;
        locals.var_vjcex_dn1 = assign5770_e5816_d_n1;
        locals.var_vjcex_dn3 = assign5770_e5816_d_n3;
        locals.var_vjcex_dn4 = assign5770_e5816_d_n4;
        locals.var_vjcex_dn5 = assign5770_e5816_d_n5;
        locals.var_vjcex_dn6 = assign5770_e5816_d_n6;
        locals.var_vjcex_dn7 = assign5770_e5816_d_n7;
        locals.var_vjcex_dn8 = assign5770_e5816_d_n8;
        locals.var_vjcex_dn9 = assign5770_e5816_d_n9;

        let assign5780_e5820: f64 = (1.0 - p.p71);
        let assign5780_e5821: f64 = (locals.var_vdc_ctc_t / assign5780_e5820);
        let assign5780_e5826: f64 = (locals.var_vjcex / locals.var_vdc_ctc_t);
        let assign5780_e5827: f64 = (1.0 - assign5780_e5826);
        let assign5780_e5830: f64 = (1.0 - p.p71);
        let assign5780_e5831: f64 = (assign5780_e5827).powf(assign5780_e5830);
        let assign5780_e5832: f64 = (1.0 - assign5780_e5831);
        let assign5780_e5833: f64 = (assign5780_e5821 * assign5780_e5832);
        let assign5780_e5837: f64 = (locals.var_vb1c4 - locals.var_vjcex);
        let assign5780_e5838: f64 = (locals.var_bjc * assign5780_e5837);
        let assign5780_e5839: f64 = (assign5780_e5833 + assign5780_e5838);
        locals.var_vtexv = assign5780_e5839;
        locals.var_vtexv_dn0 = ((((locals.var_vdc_ctc_t_dn0 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((locals.var_vjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((locals.var_vjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((locals.var_bjc_dn0 * assign5780_e5837) + (locals.var_bjc * (-locals.var_vjcex_dn0))));
        locals.var_vtexv_dn1 = ((((locals.var_vdc_ctc_t_dn1 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((locals.var_vjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((locals.var_vjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((locals.var_bjc_dn1 * assign5780_e5837) + (locals.var_bjc * (-locals.var_vjcex_dn1))));
        locals.var_vtexv_dn3 = ((((locals.var_vdc_ctc_t_dn3 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((locals.var_vjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((locals.var_vjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((locals.var_bjc_dn3 * assign5780_e5837) + (locals.var_bjc * (-locals.var_vjcex_dn3))));
        locals.var_vtexv_dn4 = ((((locals.var_vdc_ctc_t_dn4 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((locals.var_vjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((locals.var_vjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((locals.var_bjc_dn4 * assign5780_e5837) + (locals.var_bjc * (locals.var_vb1c4_dn4 - locals.var_vjcex_dn4))));
        locals.var_vtexv_dn5 = ((((locals.var_vdc_ctc_t_dn5 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((locals.var_vjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((locals.var_vjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((locals.var_bjc_dn5 * assign5780_e5837) + (locals.var_bjc * (locals.var_vb1c4_dn5 - locals.var_vjcex_dn5))));
        locals.var_vtexv_dn6 = ((((locals.var_vdc_ctc_t_dn6 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((locals.var_vjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((locals.var_vjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((locals.var_bjc_dn6 * assign5780_e5837) + (locals.var_bjc * (locals.var_vb1c4_dn6 - locals.var_vjcex_dn6))));
        locals.var_vtexv_dn7 = ((((locals.var_vdc_ctc_t_dn7 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((locals.var_vjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((locals.var_vjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((locals.var_bjc_dn7 * assign5780_e5837) + (locals.var_bjc * (locals.var_vb1c4_dn7 - locals.var_vjcex_dn7))));
        locals.var_vtexv_dn8 = ((((locals.var_vdc_ctc_t_dn8 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((locals.var_vjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((locals.var_vjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((locals.var_bjc_dn8 * assign5780_e5837) + (locals.var_bjc * (-locals.var_vjcex_dn8))));
        locals.var_vtexv_dn9 = ((((locals.var_vdc_ctc_t_dn9 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((locals.var_vjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((locals.var_vjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((locals.var_bjc_dn9 * assign5780_e5837) + (locals.var_bjc * (locals.var_vb1c4_dn9 - locals.var_vjcex_dn9))));

        let assign5790_e5843: f64 = (1.0 - locals.var_xp_t);
        let assign5790_e5845: f64 = (assign5790_e5843 * locals.var_vtexv);
        let assign5790_e5848: f64 = (locals.var_xp_t * locals.var_vb1c4);
        let assign5790_e5849: f64 = (assign5790_e5845 + assign5790_e5848);
        let assign5790_e5850: f64 = (locals.var_cjc_t * assign5790_e5849);
        let assign5790_e5853: f64 = (1.0 - p.p76);
        let assign5790_e5854: f64 = (assign5790_e5850 * assign5790_e5853);
        let assign5790_e5857: f64 = (1.0 - p.p32);
        let assign5790_e5858: f64 = (assign5790_e5854 * assign5790_e5857);
        locals.var_qtex = assign5790_e5858;
        locals.var_qtex_dn0 = ((((locals.var_cjc_t_dn0 * assign5790_e5849) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn0) * locals.var_vtexv) + (assign5790_e5843 * locals.var_vtexv_dn0)) + (locals.var_xp_t_dn0 * locals.var_vb1c4)))) * assign5790_e5853) * assign5790_e5857);
        locals.var_qtex_dn1 = ((((locals.var_cjc_t_dn1 * assign5790_e5849) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn1) * locals.var_vtexv) + (assign5790_e5843 * locals.var_vtexv_dn1)) + (locals.var_xp_t_dn1 * locals.var_vb1c4)))) * assign5790_e5853) * assign5790_e5857);
        locals.var_qtex_dn3 = ((((locals.var_cjc_t_dn3 * assign5790_e5849) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn3) * locals.var_vtexv) + (assign5790_e5843 * locals.var_vtexv_dn3)) + (locals.var_xp_t_dn3 * locals.var_vb1c4)))) * assign5790_e5853) * assign5790_e5857);
        locals.var_qtex_dn4 = ((((locals.var_cjc_t_dn4 * assign5790_e5849) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn4) * locals.var_vtexv) + (assign5790_e5843 * locals.var_vtexv_dn4)) + ((locals.var_xp_t_dn4 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn4))))) * assign5790_e5853) * assign5790_e5857);
        locals.var_qtex_dn5 = ((((locals.var_cjc_t_dn5 * assign5790_e5849) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn5) * locals.var_vtexv) + (assign5790_e5843 * locals.var_vtexv_dn5)) + ((locals.var_xp_t_dn5 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn5))))) * assign5790_e5853) * assign5790_e5857);
        locals.var_qtex_dn6 = ((((locals.var_cjc_t_dn6 * assign5790_e5849) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn6) * locals.var_vtexv) + (assign5790_e5843 * locals.var_vtexv_dn6)) + ((locals.var_xp_t_dn6 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn6))))) * assign5790_e5853) * assign5790_e5857);
        locals.var_qtex_dn7 = ((((locals.var_cjc_t_dn7 * assign5790_e5849) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn7) * locals.var_vtexv) + (assign5790_e5843 * locals.var_vtexv_dn7)) + ((locals.var_xp_t_dn7 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn7))))) * assign5790_e5853) * assign5790_e5857);
        locals.var_qtex_dn8 = ((((locals.var_cjc_t_dn8 * assign5790_e5849) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn8) * locals.var_vtexv) + (assign5790_e5843 * locals.var_vtexv_dn8)) + (locals.var_xp_t_dn8 * locals.var_vb1c4)))) * assign5790_e5853) * assign5790_e5857);
        locals.var_qtex_dn9 = ((((locals.var_cjc_t_dn9 * assign5790_e5849) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn9) * locals.var_vtexv) + (assign5790_e5843 * locals.var_vtexv_dn9)) + ((locals.var_xp_t_dn9 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn9))))) * assign5790_e5853) * assign5790_e5857);

        let assign5800_e5861: f64 = (locals.var_vbc3 - locals.var_vfc);
        let assign5800_e5863: f64 = (assign5800_e5861 / locals.var_a_vdcctc);
        locals.var_dxa = assign5800_e5863;
        locals.var_dxa_dn0 = ((((locals.var_vbc3_dn0 - locals.var_vfc_dn0) * locals.var_a_vdcctc) - (assign5800_e5861 * locals.var_a_vdcctc_dn0)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn1 = ((((locals.var_vbc3_dn1 - locals.var_vfc_dn1) * locals.var_a_vdcctc) - (assign5800_e5861 * locals.var_a_vdcctc_dn1)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn3 = ((((-locals.var_vfc_dn3) * locals.var_a_vdcctc) - (assign5800_e5861 * locals.var_a_vdcctc_dn3)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn4 = ((((locals.var_vbc3_dn4 - locals.var_vfc_dn4) * locals.var_a_vdcctc) - (assign5800_e5861 * locals.var_a_vdcctc_dn4)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn5 = ((((locals.var_vbc3_dn5 - locals.var_vfc_dn5) * locals.var_a_vdcctc) - (assign5800_e5861 * locals.var_a_vdcctc_dn5)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn6 = ((((locals.var_vbc3_dn6 - locals.var_vfc_dn6) * locals.var_a_vdcctc) - (assign5800_e5861 * locals.var_a_vdcctc_dn6)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn7 = ((((locals.var_vbc3_dn7 - locals.var_vfc_dn7) * locals.var_a_vdcctc) - (assign5800_e5861 * locals.var_a_vdcctc_dn7)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn8 = ((((locals.var_vbc3_dn8 - locals.var_vfc_dn8) * locals.var_a_vdcctc) - (assign5800_e5861 * locals.var_a_vdcctc_dn8)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn9 = ((((locals.var_vbc3_dn9 - locals.var_vfc_dn9) * locals.var_a_vdcctc) - (assign5800_e5861 * locals.var_a_vdcctc_dn9)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));

        let assign5810_e5866: f64 = if locals.var_vbc3 < locals.var_vfc { 1.0 } else { 0.0 };
        locals.var_guard105 = assign5810_e5866;

        let (assign5820_e5878, assign5820_e5878_d_n0, assign5820_e5878_d_n1, assign5820_e5878_d_n3, assign5820_e5878_d_n4, assign5820_e5878_d_n5, assign5820_e5878_d_n6, assign5820_e5878_d_n7, assign5820_e5878_d_n8, assign5820_e5878_d_n9,) = {
    if (locals.var_guard105 != 0.0) {
        let assign5820_e5872: f64 = (locals.var_dxa).exp();
        let assign5820_e5873: f64 = (1.0 + assign5820_e5872);
        let assign5820_e5874: f64 = (assign5820_e5873).ln();
        let assign5820_e5875: f64 = (locals.var_a_vdcctc * assign5820_e5874);
        let assign5820_e5876: f64 = (locals.var_vbc3 - assign5820_e5875);
        (assign5820_e5876, (locals.var_vbc3_dn0 - ((locals.var_a_vdcctc_dn0 * assign5820_e5874) + (locals.var_a_vdcctc * ((assign5820_e5872 * locals.var_dxa_dn0) / assign5820_e5873)))), (locals.var_vbc3_dn1 - ((locals.var_a_vdcctc_dn1 * assign5820_e5874) + (locals.var_a_vdcctc * ((assign5820_e5872 * locals.var_dxa_dn1) / assign5820_e5873)))), (-((locals.var_a_vdcctc_dn3 * assign5820_e5874) + (locals.var_a_vdcctc * ((assign5820_e5872 * locals.var_dxa_dn3) / assign5820_e5873)))), (locals.var_vbc3_dn4 - ((locals.var_a_vdcctc_dn4 * assign5820_e5874) + (locals.var_a_vdcctc * ((assign5820_e5872 * locals.var_dxa_dn4) / assign5820_e5873)))), (locals.var_vbc3_dn5 - ((locals.var_a_vdcctc_dn5 * assign5820_e5874) + (locals.var_a_vdcctc * ((assign5820_e5872 * locals.var_dxa_dn5) / assign5820_e5873)))), (locals.var_vbc3_dn6 - ((locals.var_a_vdcctc_dn6 * assign5820_e5874) + (locals.var_a_vdcctc * ((assign5820_e5872 * locals.var_dxa_dn6) / assign5820_e5873)))), (locals.var_vbc3_dn7 - ((locals.var_a_vdcctc_dn7 * assign5820_e5874) + (locals.var_a_vdcctc * ((assign5820_e5872 * locals.var_dxa_dn7) / assign5820_e5873)))), (locals.var_vbc3_dn8 - ((locals.var_a_vdcctc_dn8 * assign5820_e5874) + (locals.var_a_vdcctc * ((assign5820_e5872 * locals.var_dxa_dn8) / assign5820_e5873)))), (locals.var_vbc3_dn9 - ((locals.var_a_vdcctc_dn9 * assign5820_e5874) + (locals.var_a_vdcctc * ((assign5820_e5872 * locals.var_dxa_dn9) / assign5820_e5873)))),)
    } else {
        (locals.var_xvjcex, locals.var_xvjcex_dn0, locals.var_xvjcex_dn1, locals.var_xvjcex_dn3, locals.var_xvjcex_dn4, locals.var_xvjcex_dn5, locals.var_xvjcex_dn6, locals.var_xvjcex_dn7, locals.var_xvjcex_dn8, locals.var_xvjcex_dn9,)
    }
};
        locals.var_xvjcex = assign5820_e5878;
        locals.var_xvjcex_dn0 = assign5820_e5878_d_n0;
        locals.var_xvjcex_dn1 = assign5820_e5878_d_n1;
        locals.var_xvjcex_dn3 = assign5820_e5878_d_n3;
        locals.var_xvjcex_dn4 = assign5820_e5878_d_n4;
        locals.var_xvjcex_dn5 = assign5820_e5878_d_n5;
        locals.var_xvjcex_dn6 = assign5820_e5878_d_n6;
        locals.var_xvjcex_dn7 = assign5820_e5878_d_n7;
        locals.var_xvjcex_dn8 = assign5820_e5878_d_n8;
        locals.var_xvjcex_dn9 = assign5820_e5878_d_n9;

        let (assign5830_e5892, assign5830_e5892_d_n0, assign5830_e5892_d_n1, assign5830_e5892_d_n3, assign5830_e5892_d_n4, assign5830_e5892_d_n5, assign5830_e5892_d_n6, assign5830_e5892_d_n7, assign5830_e5892_d_n8, assign5830_e5892_d_n9,) = {
    if (locals.var_guard105 == 0.0) {
        let assign5830_e5885: f64 = (-locals.var_dxa);
        let assign5830_e5886: f64 = (assign5830_e5885).exp();
        let assign5830_e5887: f64 = (1.0 + assign5830_e5886);
        let assign5830_e5888: f64 = (assign5830_e5887).ln();
        let assign5830_e5889: f64 = (locals.var_a_vdcctc * assign5830_e5888);
        let assign5830_e5890: f64 = (locals.var_vfc - assign5830_e5889);
        (assign5830_e5890, (locals.var_vfc_dn0 - ((locals.var_a_vdcctc_dn0 * assign5830_e5888) + (locals.var_a_vdcctc * ((assign5830_e5886 * (-locals.var_dxa_dn0)) / assign5830_e5887)))), (locals.var_vfc_dn1 - ((locals.var_a_vdcctc_dn1 * assign5830_e5888) + (locals.var_a_vdcctc * ((assign5830_e5886 * (-locals.var_dxa_dn1)) / assign5830_e5887)))), (locals.var_vfc_dn3 - ((locals.var_a_vdcctc_dn3 * assign5830_e5888) + (locals.var_a_vdcctc * ((assign5830_e5886 * (-locals.var_dxa_dn3)) / assign5830_e5887)))), (locals.var_vfc_dn4 - ((locals.var_a_vdcctc_dn4 * assign5830_e5888) + (locals.var_a_vdcctc * ((assign5830_e5886 * (-locals.var_dxa_dn4)) / assign5830_e5887)))), (locals.var_vfc_dn5 - ((locals.var_a_vdcctc_dn5 * assign5830_e5888) + (locals.var_a_vdcctc * ((assign5830_e5886 * (-locals.var_dxa_dn5)) / assign5830_e5887)))), (locals.var_vfc_dn6 - ((locals.var_a_vdcctc_dn6 * assign5830_e5888) + (locals.var_a_vdcctc * ((assign5830_e5886 * (-locals.var_dxa_dn6)) / assign5830_e5887)))), (locals.var_vfc_dn7 - ((locals.var_a_vdcctc_dn7 * assign5830_e5888) + (locals.var_a_vdcctc * ((assign5830_e5886 * (-locals.var_dxa_dn7)) / assign5830_e5887)))), (locals.var_vfc_dn8 - ((locals.var_a_vdcctc_dn8 * assign5830_e5888) + (locals.var_a_vdcctc * ((assign5830_e5886 * (-locals.var_dxa_dn8)) / assign5830_e5887)))), (locals.var_vfc_dn9 - ((locals.var_a_vdcctc_dn9 * assign5830_e5888) + (locals.var_a_vdcctc * ((assign5830_e5886 * (-locals.var_dxa_dn9)) / assign5830_e5887)))),)
    } else {
        (locals.var_xvjcex, locals.var_xvjcex_dn0, locals.var_xvjcex_dn1, locals.var_xvjcex_dn3, locals.var_xvjcex_dn4, locals.var_xvjcex_dn5, locals.var_xvjcex_dn6, locals.var_xvjcex_dn7, locals.var_xvjcex_dn8, locals.var_xvjcex_dn9,)
    }
};
        locals.var_xvjcex = assign5830_e5892;
        locals.var_xvjcex_dn0 = assign5830_e5892_d_n0;
        locals.var_xvjcex_dn1 = assign5830_e5892_d_n1;
        locals.var_xvjcex_dn3 = assign5830_e5892_d_n3;
        locals.var_xvjcex_dn4 = assign5830_e5892_d_n4;
        locals.var_xvjcex_dn5 = assign5830_e5892_d_n5;
        locals.var_xvjcex_dn6 = assign5830_e5892_d_n6;
        locals.var_xvjcex_dn7 = assign5830_e5892_d_n7;
        locals.var_xvjcex_dn8 = assign5830_e5892_d_n8;
        locals.var_xvjcex_dn9 = assign5830_e5892_d_n9;

        let assign5840_e5896: f64 = (1.0 - p.p71);
        let assign5840_e5897: f64 = (locals.var_vdc_ctc_t / assign5840_e5896);
        let assign5840_e5902: f64 = (locals.var_xvjcex / locals.var_vdc_ctc_t);
        let assign5840_e5903: f64 = (1.0 - assign5840_e5902);
        let assign5840_e5906: f64 = (1.0 - p.p71);
        let assign5840_e5907: f64 = (assign5840_e5903).powf(assign5840_e5906);
        let assign5840_e5908: f64 = (1.0 - assign5840_e5907);
        let assign5840_e5909: f64 = (assign5840_e5897 * assign5840_e5908);
        let assign5840_e5913: f64 = (locals.var_vbc3 - locals.var_xvjcex);
        let assign5840_e5914: f64 = (locals.var_bjc * assign5840_e5913);
        let assign5840_e5915: f64 = (assign5840_e5909 + assign5840_e5914);
        locals.var_xvtexv = assign5840_e5915;
        locals.var_xvtexv_dn0 = ((((locals.var_vdc_ctc_t_dn0 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((locals.var_xvjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((locals.var_xvjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((locals.var_bjc_dn0 * assign5840_e5913) + (locals.var_bjc * (locals.var_vbc3_dn0 - locals.var_xvjcex_dn0))));
        locals.var_xvtexv_dn1 = ((((locals.var_vdc_ctc_t_dn1 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((locals.var_xvjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((locals.var_xvjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((locals.var_bjc_dn1 * assign5840_e5913) + (locals.var_bjc * (locals.var_vbc3_dn1 - locals.var_xvjcex_dn1))));
        locals.var_xvtexv_dn3 = ((((locals.var_vdc_ctc_t_dn3 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((locals.var_xvjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((locals.var_xvjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((locals.var_bjc_dn3 * assign5840_e5913) + (locals.var_bjc * (-locals.var_xvjcex_dn3))));
        locals.var_xvtexv_dn4 = ((((locals.var_vdc_ctc_t_dn4 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((locals.var_xvjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((locals.var_xvjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((locals.var_bjc_dn4 * assign5840_e5913) + (locals.var_bjc * (locals.var_vbc3_dn4 - locals.var_xvjcex_dn4))));
        locals.var_xvtexv_dn5 = ((((locals.var_vdc_ctc_t_dn5 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((locals.var_xvjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((locals.var_xvjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((locals.var_bjc_dn5 * assign5840_e5913) + (locals.var_bjc * (locals.var_vbc3_dn5 - locals.var_xvjcex_dn5))));
        locals.var_xvtexv_dn6 = ((((locals.var_vdc_ctc_t_dn6 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((locals.var_xvjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((locals.var_xvjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((locals.var_bjc_dn6 * assign5840_e5913) + (locals.var_bjc * (locals.var_vbc3_dn6 - locals.var_xvjcex_dn6))));
        locals.var_xvtexv_dn7 = ((((locals.var_vdc_ctc_t_dn7 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((locals.var_xvjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((locals.var_xvjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((locals.var_bjc_dn7 * assign5840_e5913) + (locals.var_bjc * (locals.var_vbc3_dn7 - locals.var_xvjcex_dn7))));
        locals.var_xvtexv_dn8 = ((((locals.var_vdc_ctc_t_dn8 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((locals.var_xvjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((locals.var_xvjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((locals.var_bjc_dn8 * assign5840_e5913) + (locals.var_bjc * (locals.var_vbc3_dn8 - locals.var_xvjcex_dn8))));
        locals.var_xvtexv_dn9 = ((((locals.var_vdc_ctc_t_dn9 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((locals.var_xvjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((locals.var_xvjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((locals.var_bjc_dn9 * assign5840_e5913) + (locals.var_bjc * (locals.var_vbc3_dn9 - locals.var_xvjcex_dn9))));

        let assign5850_e5919: f64 = (1.0 - locals.var_xp_t);
        let assign5850_e5921: f64 = (assign5850_e5919 * locals.var_xvtexv);
        let assign5850_e5924: f64 = (locals.var_xp_t * locals.var_vbc3);
        let assign5850_e5925: f64 = (assign5850_e5921 + assign5850_e5924);
        let assign5850_e5926: f64 = (locals.var_cjc_t * assign5850_e5925);
        let assign5850_e5929: f64 = (1.0 - p.p76);
        let assign5850_e5930: f64 = (assign5850_e5926 * assign5850_e5929);
        let assign5850_e5932: f64 = (assign5850_e5930 * p.p32);
        locals.var_xqtex = assign5850_e5932;
        locals.var_xqtex_dn0 = ((((locals.var_cjc_t_dn0 * assign5850_e5925) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn0) * locals.var_xvtexv) + (assign5850_e5919 * locals.var_xvtexv_dn0)) + ((locals.var_xp_t_dn0 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn0))))) * assign5850_e5929) * p.p32);
        locals.var_xqtex_dn1 = ((((locals.var_cjc_t_dn1 * assign5850_e5925) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn1) * locals.var_xvtexv) + (assign5850_e5919 * locals.var_xvtexv_dn1)) + ((locals.var_xp_t_dn1 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn1))))) * assign5850_e5929) * p.p32);
        locals.var_xqtex_dn3 = ((((locals.var_cjc_t_dn3 * assign5850_e5925) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn3) * locals.var_xvtexv) + (assign5850_e5919 * locals.var_xvtexv_dn3)) + (locals.var_xp_t_dn3 * locals.var_vbc3)))) * assign5850_e5929) * p.p32);
        locals.var_xqtex_dn4 = ((((locals.var_cjc_t_dn4 * assign5850_e5925) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn4) * locals.var_xvtexv) + (assign5850_e5919 * locals.var_xvtexv_dn4)) + ((locals.var_xp_t_dn4 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn4))))) * assign5850_e5929) * p.p32);
        locals.var_xqtex_dn5 = ((((locals.var_cjc_t_dn5 * assign5850_e5925) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn5) * locals.var_xvtexv) + (assign5850_e5919 * locals.var_xvtexv_dn5)) + ((locals.var_xp_t_dn5 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn5))))) * assign5850_e5929) * p.p32);
        locals.var_xqtex_dn6 = ((((locals.var_cjc_t_dn6 * assign5850_e5925) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn6) * locals.var_xvtexv) + (assign5850_e5919 * locals.var_xvtexv_dn6)) + ((locals.var_xp_t_dn6 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn6))))) * assign5850_e5929) * p.p32);
        locals.var_xqtex_dn7 = ((((locals.var_cjc_t_dn7 * assign5850_e5925) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn7) * locals.var_xvtexv) + (assign5850_e5919 * locals.var_xvtexv_dn7)) + ((locals.var_xp_t_dn7 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn7))))) * assign5850_e5929) * p.p32);
        locals.var_xqtex_dn8 = ((((locals.var_cjc_t_dn8 * assign5850_e5925) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn8) * locals.var_xvtexv) + (assign5850_e5919 * locals.var_xvtexv_dn8)) + ((locals.var_xp_t_dn8 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn8))))) * assign5850_e5929) * p.p32);
        locals.var_xqtex_dn9 = ((((locals.var_cjc_t_dn9 * assign5850_e5925) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn9) * locals.var_xvtexv) + (assign5850_e5919 * locals.var_xvtexv_dn9)) + ((locals.var_xp_t_dn9 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn9))))) * assign5850_e5929) * p.p32);

        let assign5860_e5935: f64 = (locals.var_taue_t * locals.var_ik_t);
        let assign5860_e5938: f64 = (locals.var_is_t / locals.var_ik_t);
        let assign5860_e5941: f64 = (1.0 / p.p84);
        let assign5860_e5942: f64 = (assign5860_e5938).powf(assign5860_e5941);
        let assign5860_e5943: f64 = (assign5860_e5935 * assign5860_e5942);
        locals.var_qe0 = assign5860_e5943;
        locals.var_qe0_dn0 = (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (locals.var_is_t_dn0 / locals.var_ik_t))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((locals.var_is_t_dn0 / locals.var_ik_t) / assign5860_e5938))) });
        locals.var_qe0_dn1 = (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (locals.var_is_t_dn1 / locals.var_ik_t))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((locals.var_is_t_dn1 / locals.var_ik_t) / assign5860_e5938))) });
        locals.var_qe0_dn3 = (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (locals.var_is_t_dn3 / locals.var_ik_t))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((locals.var_is_t_dn3 / locals.var_ik_t) / assign5860_e5938))) });
        locals.var_qe0_dn4 = (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (locals.var_is_t_dn4 / locals.var_ik_t))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((locals.var_is_t_dn4 / locals.var_ik_t) / assign5860_e5938))) });
        locals.var_qe0_dn5 = (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (locals.var_is_t_dn5 / locals.var_ik_t))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((locals.var_is_t_dn5 / locals.var_ik_t) / assign5860_e5938))) });
        locals.var_qe0_dn6 = (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (locals.var_is_t_dn6 / locals.var_ik_t))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((locals.var_is_t_dn6 / locals.var_ik_t) / assign5860_e5938))) });
        locals.var_qe0_dn7 = (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (locals.var_is_t_dn7 / locals.var_ik_t))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((locals.var_is_t_dn7 / locals.var_ik_t) / assign5860_e5938))) });
        locals.var_qe0_dn8 = (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (locals.var_is_t_dn8 / locals.var_ik_t))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((locals.var_is_t_dn8 / locals.var_ik_t) / assign5860_e5938))) });
        locals.var_qe0_dn9 = (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (locals.var_is_t_dn9 / locals.var_ik_t))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((locals.var_is_t_dn9 / locals.var_ik_t) / assign5860_e5938))) });

        let assign5870_e5947: f64 = (p.p84 * locals.var_vt);
        let assign5870_e5948: f64 = (locals.var_vb2e1 / assign5870_e5947);
        let assign5870_e5950: f64 = if assign5870_e5948 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign5870_e5950;

        let (assign5880_e5959, assign5880_e5959_d_n0, assign5880_e5959_d_n1, assign5880_e5959_d_n3, assign5880_e5959_d_n4, assign5880_e5959_d_n5, assign5880_e5959_d_n6, assign5880_e5959_d_n7, assign5880_e5959_d_n8, assign5880_e5959_d_n9,) = {
    if (locals.var_guard106 != 0.0) {
        let assign5880_e5955: f64 = (p.p84 * locals.var_vt);
        let assign5880_e5956: f64 = (locals.var_vb2e1 / assign5880_e5955);
        let assign5880_e5957: f64 = (assign5880_e5956).exp();
        (assign5880_e5957, 0.0, 0.0, (assign5880_e5957 * (locals.var_vb2e1_dn3 / assign5880_e5955)), 0.0, (assign5880_e5957 * (locals.var_vb2e1_dn5 / assign5880_e5955)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign5880_e5959;
        locals.var_tmpexp_dn0 = assign5880_e5959_d_n0;
        locals.var_tmpexp_dn1 = assign5880_e5959_d_n1;
        locals.var_tmpexp_dn3 = assign5880_e5959_d_n3;
        locals.var_tmpexp_dn4 = assign5880_e5959_d_n4;
        locals.var_tmpexp_dn5 = assign5880_e5959_d_n5;
        locals.var_tmpexp_dn6 = assign5880_e5959_d_n6;
        locals.var_tmpexp_dn7 = assign5880_e5959_d_n7;
        locals.var_tmpexp_dn8 = assign5880_e5959_d_n8;
        locals.var_tmpexp_dn9 = assign5880_e5959_d_n9;

        let (assign5890_e5965,) = {
    if (locals.var_guard106 == 0.0) {
        let assign5890_e5963: f64 = (p.p134).exp();
        (assign5890_e5963,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5890_e5965;

        let (assign5900_e5980, assign5900_e5980_d_n0, assign5900_e5980_d_n1, assign5900_e5980_d_n3, assign5900_e5980_d_n4, assign5900_e5980_d_n5, assign5900_e5980_d_n6, assign5900_e5980_d_n7, assign5900_e5980_d_n8, assign5900_e5980_d_n9,) = {
    if (locals.var_guard106 == 0.0) {
        let assign5900_e5973: f64 = (p.p84 * locals.var_vt);
        let assign5900_e5974: f64 = (locals.var_vb2e1 / assign5900_e5973);
        let assign5900_e5976: f64 = (assign5900_e5974 - p.p134);
        let assign5900_e5977: f64 = (1.0 + assign5900_e5976);
        let assign5900_e5978: f64 = (locals.var_expl * assign5900_e5977);
        (assign5900_e5978, 0.0, 0.0, (locals.var_expl * (locals.var_vb2e1_dn3 / assign5900_e5973)), 0.0, (locals.var_expl * (locals.var_vb2e1_dn5 / assign5900_e5973)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9,)
    }
};
        locals.var_tmpexp = assign5900_e5980;
        locals.var_tmpexp_dn0 = assign5900_e5980_d_n0;
        locals.var_tmpexp_dn1 = assign5900_e5980_d_n1;
        locals.var_tmpexp_dn3 = assign5900_e5980_d_n3;
        locals.var_tmpexp_dn4 = assign5900_e5980_d_n4;
        locals.var_tmpexp_dn5 = assign5900_e5980_d_n5;
        locals.var_tmpexp_dn6 = assign5900_e5980_d_n6;
        locals.var_tmpexp_dn7 = assign5900_e5980_d_n7;
        locals.var_tmpexp_dn8 = assign5900_e5980_d_n8;
        locals.var_tmpexp_dn9 = assign5900_e5980_d_n9;

        let assign5910_e5983: f64 = (locals.var_qe0 * locals.var_tmpexp);
        locals.var_qe_qs = assign5910_e5983;
        locals.var_qe_qs_dn0 = ((locals.var_qe0_dn0 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn0));
        locals.var_qe_qs_dn1 = ((locals.var_qe0_dn1 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn1));
        locals.var_qe_qs_dn3 = ((locals.var_qe0_dn3 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn3));
        locals.var_qe_qs_dn4 = ((locals.var_qe0_dn4 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn4));
        locals.var_qe_qs_dn5 = ((locals.var_qe0_dn5 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn5));
        locals.var_qe_qs_dn6 = ((locals.var_qe0_dn6 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn6));
        locals.var_qe_qs_dn7 = ((locals.var_qe0_dn7 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn7));
        locals.var_qe_qs_dn8 = ((locals.var_qe0_dn8 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn8));
        locals.var_qe_qs_dn9 = ((locals.var_qe0_dn9 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn9));

        let assign5920_e5986: f64 = (4.0 * locals.var_tepi_t);
        let assign5920_e5988: f64 = (assign5920_e5986 * locals.var_vt);
        let assign5920_e5990: f64 = (assign5920_e5988 / locals.var_rcv_t);
        locals.var_qepi0 = assign5920_e5990;

        let assign5930_e5993: f64 = (0.5 * locals.var_qepi0);
        let assign5930_e5995: f64 = (assign5930_e5993 * locals.var_xi_w);
        let assign5930_e5998: f64 = (locals.var_p0star + locals.var_pw);
        let assign5930_e6000: f64 = (assign5930_e5998 + 2.0);
        let assign5930_e6001: f64 = (assign5930_e5995 * assign5930_e6000);
        locals.var_qepi = assign5930_e6001;
        locals.var_qepi_dn0 = (((assign5930_e5993 * locals.var_xi_w_dn0) * assign5930_e6000) + (assign5930_e5995 * (locals.var_p0star_dn0 + locals.var_pw_dn0)));
        locals.var_qepi_dn1 = (((assign5930_e5993 * locals.var_xi_w_dn1) * assign5930_e6000) + (assign5930_e5995 * (locals.var_p0star_dn1 + locals.var_pw_dn1)));
        locals.var_qepi_dn3 = (((assign5930_e5993 * locals.var_xi_w_dn3) * assign5930_e6000) + (assign5930_e5995 * (locals.var_p0star_dn3 + locals.var_pw_dn3)));
        locals.var_qepi_dn4 = (((assign5930_e5993 * locals.var_xi_w_dn4) * assign5930_e6000) + (assign5930_e5995 * (locals.var_p0star_dn4 + locals.var_pw_dn4)));
        locals.var_qepi_dn5 = (((assign5930_e5993 * locals.var_xi_w_dn5) * assign5930_e6000) + (assign5930_e5995 * (locals.var_p0star_dn5 + locals.var_pw_dn5)));
        locals.var_qepi_dn6 = (((assign5930_e5993 * locals.var_xi_w_dn6) * assign5930_e6000) + (assign5930_e5995 * (locals.var_p0star_dn6 + locals.var_pw_dn6)));
        locals.var_qepi_dn7 = (((assign5930_e5993 * locals.var_xi_w_dn7) * assign5930_e6000) + (assign5930_e5995 * (locals.var_p0star_dn7 + locals.var_pw_dn7)));
        locals.var_qepi_dn8 = (((assign5930_e5993 * locals.var_xi_w_dn8) * assign5930_e6000) + (assign5930_e5995 * (locals.var_p0star_dn8 + locals.var_pw_dn8)));
        locals.var_qepi_dn9 = (((assign5930_e5993 * locals.var_xi_w_dn9) * assign5930_e6000) + (assign5930_e5995 * (locals.var_p0star_dn9 + locals.var_pw_dn9)));

        let assign5940_e6004: f64 = if p.p78 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign5940_e6004;

        let (assign5950_e6022, assign5950_e6022_d_n0, assign5950_e6022_d_n1, assign5950_e6022_d_n3, assign5950_e6022_d_n4, assign5950_e6022_d_n5, assign5950_e6022_d_n6, assign5950_e6022_d_n7, assign5950_e6022_d_n8, assign5950_e6022_d_n9,) = {
    if (locals.var_guard107 != 0.0) {
        let assign5950_e6008: f64 = (locals.var_taur_t * 0.5);
        let assign5950_e6011: f64 = (locals.var_qb0 * locals.var_nbex);
        let assign5950_e6014: f64 = (locals.var_qepi0 * locals.var_pwex);
        let assign5950_e6015: f64 = (assign5950_e6011 + assign5950_e6014);
        let assign5950_e6016: f64 = (assign5950_e6008 * assign5950_e6015);
        let assign5950_e6019: f64 = (locals.var_taub_t + locals.var_tepi_t);
        let assign5950_e6020: f64 = (assign5950_e6016 / assign5950_e6019);
        (assign5950_e6020, ((assign5950_e6008 * ((locals.var_qb0 * locals.var_nbex_dn0) + (locals.var_qepi0 * locals.var_pwex_dn0))) / assign5950_e6019), ((assign5950_e6008 * ((locals.var_qb0 * locals.var_nbex_dn1) + (locals.var_qepi0 * locals.var_pwex_dn1))) / assign5950_e6019), ((assign5950_e6008 * ((locals.var_qb0 * locals.var_nbex_dn3) + (locals.var_qepi0 * locals.var_pwex_dn3))) / assign5950_e6019), ((assign5950_e6008 * ((locals.var_qb0 * locals.var_nbex_dn4) + (locals.var_qepi0 * locals.var_pwex_dn4))) / assign5950_e6019), ((assign5950_e6008 * ((locals.var_qb0 * locals.var_nbex_dn5) + (locals.var_qepi0 * locals.var_pwex_dn5))) / assign5950_e6019), ((assign5950_e6008 * ((locals.var_qb0 * locals.var_nbex_dn6) + (locals.var_qepi0 * locals.var_pwex_dn6))) / assign5950_e6019), ((assign5950_e6008 * ((locals.var_qb0 * locals.var_nbex_dn7) + (locals.var_qepi0 * locals.var_pwex_dn7))) / assign5950_e6019), ((assign5950_e6008 * ((locals.var_qb0 * locals.var_nbex_dn8) + (locals.var_qepi0 * locals.var_pwex_dn8))) / assign5950_e6019), ((assign5950_e6008 * ((locals.var_qb0 * locals.var_nbex_dn9) + (locals.var_qepi0 * locals.var_pwex_dn9))) / assign5950_e6019),)
    } else {
        (locals.var_qex, locals.var_qex_dn0, locals.var_qex_dn1, locals.var_qex_dn3, locals.var_qex_dn4, locals.var_qex_dn5, locals.var_qex_dn6, locals.var_qex_dn7, locals.var_qex_dn8, locals.var_qex_dn9,)
    }
};
        locals.var_qex = assign5950_e6022;
        locals.var_qex_dn0 = assign5950_e6022_d_n0;
        locals.var_qex_dn1 = assign5950_e6022_d_n1;
        locals.var_qex_dn3 = assign5950_e6022_d_n3;
        locals.var_qex_dn4 = assign5950_e6022_d_n4;
        locals.var_qex_dn5 = assign5950_e6022_d_n5;
        locals.var_qex_dn6 = assign5950_e6022_d_n6;
        locals.var_qex_dn7 = assign5950_e6022_d_n7;
        locals.var_qex_dn8 = assign5950_e6022_d_n8;
        locals.var_qex_dn9 = assign5950_e6022_d_n9;

        let assign5960_e6025: f64 = (locals.var_vb1c4 - locals.var_vdcex_t);
        let assign5960_e6027: f64 = (assign5960_e6025 / p.p90);
        let assign5960_e6029: f64 = (assign5960_e6027 * locals.var_vtinv);
        let assign5960_e6031: f64 = if assign5960_e6029 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign5960_e6031;

        let (assign5970_e6045, assign5970_e6045_d_n0, assign5970_e6045_d_n1, assign5970_e6045_d_n3, assign5970_e6045_d_n4, assign5970_e6045_d_n5, assign5970_e6045_d_n6, assign5970_e6045_d_n7, assign5970_e6045_d_n8, assign5970_e6045_d_n9,) = {
    if ((locals.var_guard107 == 0.0) && (locals.var_guard108 != 0.0)) {
        let assign5970_e6038: f64 = (locals.var_vb1c4 - locals.var_vdcex_t);
        let assign5970_e6040: f64 = (assign5970_e6038 / p.p90);
        let assign5970_e6042: f64 = (assign5970_e6040 * locals.var_vtinv);
        let assign5970_e6043: f64 = (assign5970_e6042).exp();
        (assign5970_e6043, (assign5970_e6043 * (((-locals.var_vdcex_t_dn0) / p.p90) * locals.var_vtinv)), (assign5970_e6043 * (((-locals.var_vdcex_t_dn1) / p.p90) * locals.var_vtinv)), (assign5970_e6043 * (((-locals.var_vdcex_t_dn3) / p.p90) * locals.var_vtinv)), (assign5970_e6043 * (((locals.var_vb1c4_dn4 - locals.var_vdcex_t_dn4) / p.p90) * locals.var_vtinv)), (assign5970_e6043 * (((locals.var_vb1c4_dn5 - locals.var_vdcex_t_dn5) / p.p90) * locals.var_vtinv)), (assign5970_e6043 * (((locals.var_vb1c4_dn6 - locals.var_vdcex_t_dn6) / p.p90) * locals.var_vtinv)), (assign5970_e6043 * (((locals.var_vb1c4_dn7 - locals.var_vdcex_t_dn7) / p.p90) * locals.var_vtinv)), (assign5970_e6043 * (((-locals.var_vdcex_t_dn8) / p.p90) * locals.var_vtinv)), (assign5970_e6043 * (((locals.var_vb1c4_dn9 - locals.var_vdcex_t_dn9) / p.p90) * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4vdcex, locals.var_evb1c4vdcex_dn0, locals.var_evb1c4vdcex_dn1, locals.var_evb1c4vdcex_dn3, locals.var_evb1c4vdcex_dn4, locals.var_evb1c4vdcex_dn5, locals.var_evb1c4vdcex_dn6, locals.var_evb1c4vdcex_dn7, locals.var_evb1c4vdcex_dn8, locals.var_evb1c4vdcex_dn9,)
    }
};
        locals.var_evb1c4vdcex = assign5970_e6045;
        locals.var_evb1c4vdcex_dn0 = assign5970_e6045_d_n0;
        locals.var_evb1c4vdcex_dn1 = assign5970_e6045_d_n1;
        locals.var_evb1c4vdcex_dn3 = assign5970_e6045_d_n3;
        locals.var_evb1c4vdcex_dn4 = assign5970_e6045_d_n4;
        locals.var_evb1c4vdcex_dn5 = assign5970_e6045_d_n5;
        locals.var_evb1c4vdcex_dn6 = assign5970_e6045_d_n6;
        locals.var_evb1c4vdcex_dn7 = assign5970_e6045_d_n7;
        locals.var_evb1c4vdcex_dn8 = assign5970_e6045_d_n8;
        locals.var_evb1c4vdcex_dn9 = assign5970_e6045_d_n9;

        let (assign5980_e6054,) = {
    if ((locals.var_guard107 == 0.0) && (locals.var_guard108 == 0.0)) {
        let assign5980_e6052: f64 = (p.p134).exp();
        (assign5980_e6052,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5980_e6054;

        let (assign5990_e6074, assign5990_e6074_d_n0, assign5990_e6074_d_n1, assign5990_e6074_d_n3, assign5990_e6074_d_n4, assign5990_e6074_d_n5, assign5990_e6074_d_n6, assign5990_e6074_d_n7, assign5990_e6074_d_n8, assign5990_e6074_d_n9,) = {
    if ((locals.var_guard107 == 0.0) && (locals.var_guard108 == 0.0)) {
        let assign5990_e6064: f64 = (locals.var_vb1c4 - locals.var_vdcex_t);
        let assign5990_e6066: f64 = (assign5990_e6064 / p.p90);
        let assign5990_e6068: f64 = (assign5990_e6066 * locals.var_vtinv);
        let assign5990_e6070: f64 = (assign5990_e6068 - p.p134);
        let assign5990_e6071: f64 = (1.0 + assign5990_e6070);
        let assign5990_e6072: f64 = (locals.var_expl * assign5990_e6071);
        (assign5990_e6072, (locals.var_expl * (((-locals.var_vdcex_t_dn0) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdcex_t_dn1) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdcex_t_dn3) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn4 - locals.var_vdcex_t_dn4) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn5 - locals.var_vdcex_t_dn5) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn6 - locals.var_vdcex_t_dn6) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn7 - locals.var_vdcex_t_dn7) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdcex_t_dn8) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn9 - locals.var_vdcex_t_dn9) / p.p90) * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4vdcex, locals.var_evb1c4vdcex_dn0, locals.var_evb1c4vdcex_dn1, locals.var_evb1c4vdcex_dn3, locals.var_evb1c4vdcex_dn4, locals.var_evb1c4vdcex_dn5, locals.var_evb1c4vdcex_dn6, locals.var_evb1c4vdcex_dn7, locals.var_evb1c4vdcex_dn8, locals.var_evb1c4vdcex_dn9,)
    }
};
        locals.var_evb1c4vdcex = assign5990_e6074;
        locals.var_evb1c4vdcex_dn0 = assign5990_e6074_d_n0;
        locals.var_evb1c4vdcex_dn1 = assign5990_e6074_d_n1;
        locals.var_evb1c4vdcex_dn3 = assign5990_e6074_d_n3;
        locals.var_evb1c4vdcex_dn4 = assign5990_e6074_d_n4;
        locals.var_evb1c4vdcex_dn5 = assign5990_e6074_d_n5;
        locals.var_evb1c4vdcex_dn6 = assign5990_e6074_d_n6;
        locals.var_evb1c4vdcex_dn7 = assign5990_e6074_d_n7;
        locals.var_evb1c4vdcex_dn8 = assign5990_e6074_d_n8;
        locals.var_evb1c4vdcex_dn9 = assign5990_e6074_d_n9;

    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6000_e6094, assign6000_e6094_d_n0, assign6000_e6094_d_n1, assign6000_e6094_d_n3, assign6000_e6094_d_n4, assign6000_e6094_d_n5, assign6000_e6094_d_n6, assign6000_e6094_d_n7, assign6000_e6094_d_n8, assign6000_e6094_d_n9,) = {
    if (locals.var_guard107 == 0.0) {
        let assign6000_e6079: f64 = (2.0 * locals.var_ibx_t);
        let assign6000_e6081: f64 = (assign6000_e6079 * locals.var_tauex_t);
        let assign6000_e6083: f64 = (assign6000_e6081 * locals.var_evb1c4);
        let assign6000_e6088: f64 = (4.0 * locals.var_evb1c4vdcex);
        let assign6000_e6089: f64 = (1.0 + assign6000_e6088);
        let assign6000_e6090: f64 = (assign6000_e6089).sqrt();
        let assign6000_e6091: f64 = (1.0 + assign6000_e6090);
        let assign6000_e6092: f64 = (assign6000_e6083 / assign6000_e6091);
        (assign6000_e6092, (-((assign6000_e6083 * ((4.0 * locals.var_evb1c4vdcex_dn0) / (2.0 * assign6000_e6090))) / (assign6000_e6091 * assign6000_e6091))), (-((assign6000_e6083 * ((4.0 * locals.var_evb1c4vdcex_dn1) / (2.0 * assign6000_e6090))) / (assign6000_e6091 * assign6000_e6091))), (-((assign6000_e6083 * ((4.0 * locals.var_evb1c4vdcex_dn3) / (2.0 * assign6000_e6090))) / (assign6000_e6091 * assign6000_e6091))), ((((assign6000_e6081 * locals.var_evb1c4_dn4) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * locals.var_evb1c4vdcex_dn4) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((assign6000_e6081 * locals.var_evb1c4_dn5) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * locals.var_evb1c4vdcex_dn5) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((assign6000_e6081 * locals.var_evb1c4_dn6) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * locals.var_evb1c4vdcex_dn6) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((assign6000_e6081 * locals.var_evb1c4_dn7) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * locals.var_evb1c4vdcex_dn7) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), (-((assign6000_e6083 * ((4.0 * locals.var_evb1c4vdcex_dn8) / (2.0 * assign6000_e6090))) / (assign6000_e6091 * assign6000_e6091))), ((((assign6000_e6081 * locals.var_evb1c4_dn9) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * locals.var_evb1c4vdcex_dn9) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)),)
    } else {
        (locals.var_qex, locals.var_qex_dn0, locals.var_qex_dn1, locals.var_qex_dn3, locals.var_qex_dn4, locals.var_qex_dn5, locals.var_qex_dn6, locals.var_qex_dn7, locals.var_qex_dn8, locals.var_qex_dn9,)
    }
};
        locals.var_qex = assign6000_e6094;
        locals.var_qex_dn0 = assign6000_e6094_d_n0;
        locals.var_qex_dn1 = assign6000_e6094_d_n1;
        locals.var_qex_dn3 = assign6000_e6094_d_n3;
        locals.var_qex_dn4 = assign6000_e6094_d_n4;
        locals.var_qex_dn5 = assign6000_e6094_d_n5;
        locals.var_qex_dn6 = assign6000_e6094_d_n6;
        locals.var_qex_dn7 = assign6000_e6094_d_n7;
        locals.var_qex_dn8 = assign6000_e6094_d_n8;
        locals.var_qex_dn9 = assign6000_e6094_d_n9;

        let assign6010_e6105: f64 = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard109 = assign6010_e6105;

        let (assign6020_e6111, assign6020_e6111_d_n0, assign6020_e6111_d_n1, assign6020_e6111_d_n3, assign6020_e6111_d_n4, assign6020_e6111_d_n5, assign6020_e6111_d_n6, assign6020_e6111_d_n7, assign6020_e6111_d_n8, assign6020_e6111_d_n9,) = {
    if (locals.var_guard109 != 0.0) {
        let assign6020_e6109: f64 = (locals.var_qex * locals.var_xext1);
        (assign6020_e6109, (locals.var_qex_dn0 * locals.var_xext1), (locals.var_qex_dn1 * locals.var_xext1), (locals.var_qex_dn3 * locals.var_xext1), (locals.var_qex_dn4 * locals.var_xext1), (locals.var_qex_dn5 * locals.var_xext1), (locals.var_qex_dn6 * locals.var_xext1), (locals.var_qex_dn7 * locals.var_xext1), (locals.var_qex_dn8 * locals.var_xext1), (locals.var_qex_dn9 * locals.var_xext1),)
    } else {
        (locals.var_qex, locals.var_qex_dn0, locals.var_qex_dn1, locals.var_qex_dn3, locals.var_qex_dn4, locals.var_qex_dn5, locals.var_qex_dn6, locals.var_qex_dn7, locals.var_qex_dn8, locals.var_qex_dn9,)
    }
};
        locals.var_qex = assign6020_e6111;
        locals.var_qex_dn0 = assign6020_e6111_d_n0;
        locals.var_qex_dn1 = assign6020_e6111_d_n1;
        locals.var_qex_dn3 = assign6020_e6111_d_n3;
        locals.var_qex_dn4 = assign6020_e6111_d_n4;
        locals.var_qex_dn5 = assign6020_e6111_d_n5;
        locals.var_qex_dn6 = assign6020_e6111_d_n6;
        locals.var_qex_dn7 = assign6020_e6111_d_n7;
        locals.var_qex_dn8 = assign6020_e6111_d_n8;
        locals.var_qex_dn9 = assign6020_e6111_d_n9;

        let assign6030_e6114: f64 = if p.p78 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign6030_e6114;

        let (assign6040_e6122, assign6040_e6122_d_n0, assign6040_e6122_d_n1, assign6040_e6122_d_n3, assign6040_e6122_d_n4, assign6040_e6122_d_n5, assign6040_e6122_d_n6, assign6040_e6122_d_n7, assign6040_e6122_d_n8, assign6040_e6122_d_n9,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard110 != 0.0)) {
        let assign6040_e6120: f64 = (locals.var_if0 * locals.var_evbc3);
        (assign6040_e6120, ((locals.var_if0_dn0 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn0)), ((locals.var_if0_dn1 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn1)), (locals.var_if0_dn3 * locals.var_evbc3), ((locals.var_if0_dn4 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn4)), ((locals.var_if0_dn5 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn5)), ((locals.var_if0_dn6 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn6)), ((locals.var_if0_dn7 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn7)), ((locals.var_if0_dn8 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn8)), ((locals.var_if0_dn9 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn9)),)
    } else {
        (locals.var_xg1, locals.var_xg1_dn0, locals.var_xg1_dn1, locals.var_xg1_dn3, locals.var_xg1_dn4, locals.var_xg1_dn5, locals.var_xg1_dn6, locals.var_xg1_dn7, locals.var_xg1_dn8, locals.var_xg1_dn9,)
    }
};
        locals.var_xg1 = assign6040_e6122;
        locals.var_xg1_dn0 = assign6040_e6122_d_n0;
        locals.var_xg1_dn1 = assign6040_e6122_d_n1;
        locals.var_xg1_dn3 = assign6040_e6122_d_n3;
        locals.var_xg1_dn4 = assign6040_e6122_d_n4;
        locals.var_xg1_dn5 = assign6040_e6122_d_n5;
        locals.var_xg1_dn6 = assign6040_e6122_d_n6;
        locals.var_xg1_dn7 = assign6040_e6122_d_n7;
        locals.var_xg1_dn8 = assign6040_e6122_d_n8;
        locals.var_xg1_dn9 = assign6040_e6122_d_n9;

        let (assign6050_e6137, assign6050_e6137_d_n0, assign6050_e6137_d_n1, assign6050_e6137_d_n3, assign6050_e6137_d_n4, assign6050_e6137_d_n5, assign6050_e6137_d_n6, assign6050_e6137_d_n7, assign6050_e6137_d_n8, assign6050_e6137_d_n9,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard110 != 0.0)) {
        let assign6050_e6128: f64 = (locals.var_xg1 - locals.var_if0);
        let assign6050_e6132: f64 = (1.0 + locals.var_xg1);
        let assign6050_e6133: f64 = (assign6050_e6132).sqrt();
        let assign6050_e6134: f64 = (1.0 + assign6050_e6133);
        let assign6050_e6135: f64 = (assign6050_e6128 / assign6050_e6134);
        (assign6050_e6135, ((((locals.var_xg1_dn0 - locals.var_if0_dn0) * assign6050_e6134) - (assign6050_e6128 * (locals.var_xg1_dn0 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((locals.var_xg1_dn1 - locals.var_if0_dn1) * assign6050_e6134) - (assign6050_e6128 * (locals.var_xg1_dn1 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((locals.var_xg1_dn3 - locals.var_if0_dn3) * assign6050_e6134) - (assign6050_e6128 * (locals.var_xg1_dn3 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((locals.var_xg1_dn4 - locals.var_if0_dn4) * assign6050_e6134) - (assign6050_e6128 * (locals.var_xg1_dn4 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((locals.var_xg1_dn5 - locals.var_if0_dn5) * assign6050_e6134) - (assign6050_e6128 * (locals.var_xg1_dn5 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((locals.var_xg1_dn6 - locals.var_if0_dn6) * assign6050_e6134) - (assign6050_e6128 * (locals.var_xg1_dn6 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((locals.var_xg1_dn7 - locals.var_if0_dn7) * assign6050_e6134) - (assign6050_e6128 * (locals.var_xg1_dn7 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((locals.var_xg1_dn8 - locals.var_if0_dn8) * assign6050_e6134) - (assign6050_e6128 * (locals.var_xg1_dn8 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((locals.var_xg1_dn9 - locals.var_if0_dn9) * assign6050_e6134) - (assign6050_e6128 * (locals.var_xg1_dn9 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)),)
    } else {
        (locals.var_xnbex, locals.var_xnbex_dn0, locals.var_xnbex_dn1, locals.var_xnbex_dn3, locals.var_xnbex_dn4, locals.var_xnbex_dn5, locals.var_xnbex_dn6, locals.var_xnbex_dn7, locals.var_xnbex_dn8, locals.var_xnbex_dn9,)
    }
};
        locals.var_xnbex = assign6050_e6137;
        locals.var_xnbex_dn0 = assign6050_e6137_d_n0;
        locals.var_xnbex_dn1 = assign6050_e6137_d_n1;
        locals.var_xnbex_dn3 = assign6050_e6137_d_n3;
        locals.var_xnbex_dn4 = assign6050_e6137_d_n4;
        locals.var_xnbex_dn5 = assign6050_e6137_d_n5;
        locals.var_xnbex_dn6 = assign6050_e6137_d_n6;
        locals.var_xnbex_dn7 = assign6050_e6137_d_n7;
        locals.var_xnbex_dn8 = assign6050_e6137_d_n8;
        locals.var_xnbex_dn9 = assign6050_e6137_d_n9;

        let (assign6060_e6145, assign6060_e6145_d_n0, assign6060_e6145_d_n1, assign6060_e6145_d_n3, assign6060_e6145_d_n4, assign6060_e6145_d_n5, assign6060_e6145_d_n6, assign6060_e6145_d_n7, assign6060_e6145_d_n8, assign6060_e6145_d_n9,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard110 != 0.0)) {
        let assign6060_e6143: f64 = (4.0 * locals.var_evbc3vdc);
        (assign6060_e6143, (4.0 * locals.var_evbc3vdc_dn0), (4.0 * locals.var_evbc3vdc_dn1), (4.0 * locals.var_evbc3vdc_dn3), (4.0 * locals.var_evbc3vdc_dn4), (4.0 * locals.var_evbc3vdc_dn5), (4.0 * locals.var_evbc3vdc_dn6), (4.0 * locals.var_evbc3vdc_dn7), (4.0 * locals.var_evbc3vdc_dn8), (4.0 * locals.var_evbc3vdc_dn9),)
    } else {
        (locals.var_xg2, locals.var_xg2_dn0, locals.var_xg2_dn1, locals.var_xg2_dn3, locals.var_xg2_dn4, locals.var_xg2_dn5, locals.var_xg2_dn6, locals.var_xg2_dn7, locals.var_xg2_dn8, locals.var_xg2_dn9,)
    }
};
        locals.var_xg2 = assign6060_e6145;
        locals.var_xg2_dn0 = assign6060_e6145_d_n0;
        locals.var_xg2_dn1 = assign6060_e6145_d_n1;
        locals.var_xg2_dn3 = assign6060_e6145_d_n3;
        locals.var_xg2_dn4 = assign6060_e6145_d_n4;
        locals.var_xg2_dn5 = assign6060_e6145_d_n5;
        locals.var_xg2_dn6 = assign6060_e6145_d_n6;
        locals.var_xg2_dn7 = assign6060_e6145_d_n7;
        locals.var_xg2_dn8 = assign6060_e6145_d_n8;
        locals.var_xg2_dn9 = assign6060_e6145_d_n9;

        let (assign6070_e6158, assign6070_e6158_d_n0, assign6070_e6158_d_n1, assign6070_e6158_d_n3, assign6070_e6158_d_n4, assign6070_e6158_d_n5, assign6070_e6158_d_n6, assign6070_e6158_d_n7, assign6070_e6158_d_n8, assign6070_e6158_d_n9,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard110 != 0.0)) {
        let assign6070_e6153: f64 = (1.0 + locals.var_xg2);
        let assign6070_e6154: f64 = (assign6070_e6153).sqrt();
        let assign6070_e6155: f64 = (1.0 + assign6070_e6154);
        let assign6070_e6156: f64 = (locals.var_xg2 / assign6070_e6155);
        (assign6070_e6156, (((locals.var_xg2_dn0 * assign6070_e6155) - (locals.var_xg2 * (locals.var_xg2_dn0 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((locals.var_xg2_dn1 * assign6070_e6155) - (locals.var_xg2 * (locals.var_xg2_dn1 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((locals.var_xg2_dn3 * assign6070_e6155) - (locals.var_xg2 * (locals.var_xg2_dn3 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((locals.var_xg2_dn4 * assign6070_e6155) - (locals.var_xg2 * (locals.var_xg2_dn4 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((locals.var_xg2_dn5 * assign6070_e6155) - (locals.var_xg2 * (locals.var_xg2_dn5 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((locals.var_xg2_dn6 * assign6070_e6155) - (locals.var_xg2 * (locals.var_xg2_dn6 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((locals.var_xg2_dn7 * assign6070_e6155) - (locals.var_xg2 * (locals.var_xg2_dn7 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((locals.var_xg2_dn8 * assign6070_e6155) - (locals.var_xg2 * (locals.var_xg2_dn8 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((locals.var_xg2_dn9 * assign6070_e6155) - (locals.var_xg2 * (locals.var_xg2_dn9 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)),)
    } else {
        (locals.var_xpwex, locals.var_xpwex_dn0, locals.var_xpwex_dn1, locals.var_xpwex_dn3, locals.var_xpwex_dn4, locals.var_xpwex_dn5, locals.var_xpwex_dn6, locals.var_xpwex_dn7, locals.var_xpwex_dn8, locals.var_xpwex_dn9,)
    }
};
        locals.var_xpwex = assign6070_e6158;
        locals.var_xpwex_dn0 = assign6070_e6158_d_n0;
        locals.var_xpwex_dn1 = assign6070_e6158_d_n1;
        locals.var_xpwex_dn3 = assign6070_e6158_d_n3;
        locals.var_xpwex_dn4 = assign6070_e6158_d_n4;
        locals.var_xpwex_dn5 = assign6070_e6158_d_n5;
        locals.var_xpwex_dn6 = assign6070_e6158_d_n6;
        locals.var_xpwex_dn7 = assign6070_e6158_d_n7;
        locals.var_xpwex_dn8 = assign6070_e6158_d_n8;
        locals.var_xpwex_dn9 = assign6070_e6158_d_n9;

        let (assign6080_e6180, assign6080_e6180_d_n0, assign6080_e6180_d_n1, assign6080_e6180_d_n3, assign6080_e6180_d_n4, assign6080_e6180_d_n5, assign6080_e6180_d_n6, assign6080_e6180_d_n7, assign6080_e6180_d_n8, assign6080_e6180_d_n9,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard110 != 0.0)) {
        let assign6080_e6164: f64 = (0.5 * p.p32);
        let assign6080_e6166: f64 = (assign6080_e6164 * locals.var_taur_t);
        let assign6080_e6169: f64 = (locals.var_qb0 * locals.var_xnbex);
        let assign6080_e6172: f64 = (locals.var_qepi0 * locals.var_xpwex);
        let assign6080_e6173: f64 = (assign6080_e6169 + assign6080_e6172);
        let assign6080_e6174: f64 = (assign6080_e6166 * assign6080_e6173);
        let assign6080_e6177: f64 = (locals.var_taub_t + locals.var_tepi_t);
        let assign6080_e6178: f64 = (assign6080_e6174 / assign6080_e6177);
        (assign6080_e6178, ((assign6080_e6166 * ((locals.var_qb0 * locals.var_xnbex_dn0) + (locals.var_qepi0 * locals.var_xpwex_dn0))) / assign6080_e6177), ((assign6080_e6166 * ((locals.var_qb0 * locals.var_xnbex_dn1) + (locals.var_qepi0 * locals.var_xpwex_dn1))) / assign6080_e6177), ((assign6080_e6166 * ((locals.var_qb0 * locals.var_xnbex_dn3) + (locals.var_qepi0 * locals.var_xpwex_dn3))) / assign6080_e6177), ((assign6080_e6166 * ((locals.var_qb0 * locals.var_xnbex_dn4) + (locals.var_qepi0 * locals.var_xpwex_dn4))) / assign6080_e6177), ((assign6080_e6166 * ((locals.var_qb0 * locals.var_xnbex_dn5) + (locals.var_qepi0 * locals.var_xpwex_dn5))) / assign6080_e6177), ((assign6080_e6166 * ((locals.var_qb0 * locals.var_xnbex_dn6) + (locals.var_qepi0 * locals.var_xpwex_dn6))) / assign6080_e6177), ((assign6080_e6166 * ((locals.var_qb0 * locals.var_xnbex_dn7) + (locals.var_qepi0 * locals.var_xpwex_dn7))) / assign6080_e6177), ((assign6080_e6166 * ((locals.var_qb0 * locals.var_xnbex_dn8) + (locals.var_qepi0 * locals.var_xpwex_dn8))) / assign6080_e6177), ((assign6080_e6166 * ((locals.var_qb0 * locals.var_xnbex_dn9) + (locals.var_qepi0 * locals.var_xpwex_dn9))) / assign6080_e6177),)
    } else {
        (locals.var_xqmex, locals.var_xqmex_dn0, locals.var_xqmex_dn1, locals.var_xqmex_dn3, locals.var_xqmex_dn4, locals.var_xqmex_dn5, locals.var_xqmex_dn6, locals.var_xqmex_dn7, locals.var_xqmex_dn8, locals.var_xqmex_dn9,)
    }
};
        locals.var_xqmex = assign6080_e6180;
        locals.var_xqmex_dn0 = assign6080_e6180_d_n0;
        locals.var_xqmex_dn1 = assign6080_e6180_d_n1;
        locals.var_xqmex_dn3 = assign6080_e6180_d_n3;
        locals.var_xqmex_dn4 = assign6080_e6180_d_n4;
        locals.var_xqmex_dn5 = assign6080_e6180_d_n5;
        locals.var_xqmex_dn6 = assign6080_e6180_d_n6;
        locals.var_xqmex_dn7 = assign6080_e6180_d_n7;
        locals.var_xqmex_dn8 = assign6080_e6180_d_n8;
        locals.var_xqmex_dn9 = assign6080_e6180_d_n9;

        let assign6090_e6183: f64 = (locals.var_vbc3 - locals.var_vdcex_t);
        let assign6090_e6185: f64 = (assign6090_e6183 * locals.var_vtinv);
        let assign6090_e6187: f64 = if assign6090_e6185 < p.p134 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign6090_e6187;

        let (assign6100_e6201, assign6100_e6201_d_n0, assign6100_e6201_d_n1, assign6100_e6201_d_n3, assign6100_e6201_d_n4, assign6100_e6201_d_n5, assign6100_e6201_d_n6, assign6100_e6201_d_n7, assign6100_e6201_d_n8, assign6100_e6201_d_n9,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard110 == 0.0)) && (locals.var_guard111 != 0.0)) {
        let assign6100_e6196: f64 = (locals.var_vbc3 - locals.var_vdcex_t);
        let assign6100_e6198: f64 = (assign6100_e6196 * locals.var_vtinv);
        let assign6100_e6199: f64 = (assign6100_e6198).exp();
        (assign6100_e6199, (assign6100_e6199 * ((locals.var_vbc3_dn0 - locals.var_vdcex_t_dn0) * locals.var_vtinv)), (assign6100_e6199 * ((locals.var_vbc3_dn1 - locals.var_vdcex_t_dn1) * locals.var_vtinv)), (assign6100_e6199 * ((-locals.var_vdcex_t_dn3) * locals.var_vtinv)), (assign6100_e6199 * ((locals.var_vbc3_dn4 - locals.var_vdcex_t_dn4) * locals.var_vtinv)), (assign6100_e6199 * ((locals.var_vbc3_dn5 - locals.var_vdcex_t_dn5) * locals.var_vtinv)), (assign6100_e6199 * ((locals.var_vbc3_dn6 - locals.var_vdcex_t_dn6) * locals.var_vtinv)), (assign6100_e6199 * ((locals.var_vbc3_dn7 - locals.var_vdcex_t_dn7) * locals.var_vtinv)), (assign6100_e6199 * ((locals.var_vbc3_dn8 - locals.var_vdcex_t_dn8) * locals.var_vtinv)), (assign6100_e6199 * ((locals.var_vbc3_dn9 - locals.var_vdcex_t_dn9) * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3vdcex, locals.var_evbc3vdcex_dn0, locals.var_evbc3vdcex_dn1, locals.var_evbc3vdcex_dn3, locals.var_evbc3vdcex_dn4, locals.var_evbc3vdcex_dn5, locals.var_evbc3vdcex_dn6, locals.var_evbc3vdcex_dn7, locals.var_evbc3vdcex_dn8, locals.var_evbc3vdcex_dn9,)
    }
};
        locals.var_evbc3vdcex = assign6100_e6201;
        locals.var_evbc3vdcex_dn0 = assign6100_e6201_d_n0;
        locals.var_evbc3vdcex_dn1 = assign6100_e6201_d_n1;
        locals.var_evbc3vdcex_dn3 = assign6100_e6201_d_n3;
        locals.var_evbc3vdcex_dn4 = assign6100_e6201_d_n4;
        locals.var_evbc3vdcex_dn5 = assign6100_e6201_d_n5;
        locals.var_evbc3vdcex_dn6 = assign6100_e6201_d_n6;
        locals.var_evbc3vdcex_dn7 = assign6100_e6201_d_n7;
        locals.var_evbc3vdcex_dn8 = assign6100_e6201_d_n8;
        locals.var_evbc3vdcex_dn9 = assign6100_e6201_d_n9;

        let (assign6110_e6212,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard110 == 0.0)) && (locals.var_guard111 == 0.0)) {
        let assign6110_e6210: f64 = (p.p134).exp();
        (assign6110_e6210,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign6110_e6212;

        let (assign6120_e6232, assign6120_e6232_d_n0, assign6120_e6232_d_n1, assign6120_e6232_d_n3, assign6120_e6232_d_n4, assign6120_e6232_d_n5, assign6120_e6232_d_n6, assign6120_e6232_d_n7, assign6120_e6232_d_n8, assign6120_e6232_d_n9,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard110 == 0.0)) && (locals.var_guard111 == 0.0)) {
        let assign6120_e6224: f64 = (locals.var_vbc3 - locals.var_vdcex_t);
        let assign6120_e6226: f64 = (assign6120_e6224 * locals.var_vtinv);
        let assign6120_e6228: f64 = (assign6120_e6226 - p.p134);
        let assign6120_e6229: f64 = (1.0 + assign6120_e6228);
        let assign6120_e6230: f64 = (locals.var_expl * assign6120_e6229);
        (assign6120_e6230, (locals.var_expl * ((locals.var_vbc3_dn0 - locals.var_vdcex_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn1 - locals.var_vdcex_t_dn1) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdcex_t_dn3) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn4 - locals.var_vdcex_t_dn4) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn5 - locals.var_vdcex_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn6 - locals.var_vdcex_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn7 - locals.var_vdcex_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn8 - locals.var_vdcex_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn9 - locals.var_vdcex_t_dn9) * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3vdcex, locals.var_evbc3vdcex_dn0, locals.var_evbc3vdcex_dn1, locals.var_evbc3vdcex_dn3, locals.var_evbc3vdcex_dn4, locals.var_evbc3vdcex_dn5, locals.var_evbc3vdcex_dn6, locals.var_evbc3vdcex_dn7, locals.var_evbc3vdcex_dn8, locals.var_evbc3vdcex_dn9,)
    }
};
        locals.var_evbc3vdcex = assign6120_e6232;
        locals.var_evbc3vdcex_dn0 = assign6120_e6232_d_n0;
        locals.var_evbc3vdcex_dn1 = assign6120_e6232_d_n1;
        locals.var_evbc3vdcex_dn3 = assign6120_e6232_d_n3;
        locals.var_evbc3vdcex_dn4 = assign6120_e6232_d_n4;
        locals.var_evbc3vdcex_dn5 = assign6120_e6232_d_n5;
        locals.var_evbc3vdcex_dn6 = assign6120_e6232_d_n6;
        locals.var_evbc3vdcex_dn7 = assign6120_e6232_d_n7;
        locals.var_evbc3vdcex_dn8 = assign6120_e6232_d_n8;
        locals.var_evbc3vdcex_dn9 = assign6120_e6232_d_n9;

        let (assign6130_e6256, assign6130_e6256_d_n0, assign6130_e6256_d_n1, assign6130_e6256_d_n3, assign6130_e6256_d_n4, assign6130_e6256_d_n5, assign6130_e6256_d_n6, assign6130_e6256_d_n7, assign6130_e6256_d_n8, assign6130_e6256_d_n9,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard110 == 0.0)) {
        let assign6130_e6239: f64 = (2.0 * p.p32);
        let assign6130_e6241: f64 = (assign6130_e6239 * locals.var_ibx_t);
        let assign6130_e6243: f64 = (assign6130_e6241 * locals.var_tauex_t);
        let assign6130_e6245: f64 = (assign6130_e6243 * locals.var_evbc3);
        let assign6130_e6250: f64 = (4.0 * locals.var_evbc3vdcex);
        let assign6130_e6251: f64 = (1.0 + assign6130_e6250);
        let assign6130_e6252: f64 = (assign6130_e6251).sqrt();
        let assign6130_e6253: f64 = (1.0 + assign6130_e6252);
        let assign6130_e6254: f64 = (assign6130_e6245 / assign6130_e6253);
        (assign6130_e6254, ((((assign6130_e6243 * locals.var_evbc3_dn0) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * locals.var_evbc3vdcex_dn0) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((assign6130_e6243 * locals.var_evbc3_dn1) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * locals.var_evbc3vdcex_dn1) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), (-((assign6130_e6245 * ((4.0 * locals.var_evbc3vdcex_dn3) / (2.0 * assign6130_e6252))) / (assign6130_e6253 * assign6130_e6253))), ((((assign6130_e6243 * locals.var_evbc3_dn4) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * locals.var_evbc3vdcex_dn4) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((assign6130_e6243 * locals.var_evbc3_dn5) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * locals.var_evbc3vdcex_dn5) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((assign6130_e6243 * locals.var_evbc3_dn6) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * locals.var_evbc3vdcex_dn6) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((assign6130_e6243 * locals.var_evbc3_dn7) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * locals.var_evbc3vdcex_dn7) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((assign6130_e6243 * locals.var_evbc3_dn8) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * locals.var_evbc3vdcex_dn8) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((assign6130_e6243 * locals.var_evbc3_dn9) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * locals.var_evbc3vdcex_dn9) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)),)
    } else {
        (locals.var_xqmex, locals.var_xqmex_dn0, locals.var_xqmex_dn1, locals.var_xqmex_dn3, locals.var_xqmex_dn4, locals.var_xqmex_dn5, locals.var_xqmex_dn6, locals.var_xqmex_dn7, locals.var_xqmex_dn8, locals.var_xqmex_dn9,)
    }
};
        locals.var_xqmex = assign6130_e6256;
        locals.var_xqmex_dn0 = assign6130_e6256_d_n0;
        locals.var_xqmex_dn1 = assign6130_e6256_d_n1;
        locals.var_xqmex_dn3 = assign6130_e6256_d_n3;
        locals.var_xqmex_dn4 = assign6130_e6256_d_n4;
        locals.var_xqmex_dn5 = assign6130_e6256_d_n5;
        locals.var_xqmex_dn6 = assign6130_e6256_d_n6;
        locals.var_xqmex_dn7 = assign6130_e6256_d_n7;
        locals.var_xqmex_dn8 = assign6130_e6256_d_n8;
        locals.var_xqmex_dn9 = assign6130_e6256_d_n9;

        let (assign6140_e6262, assign6140_e6262_d_n0, assign6140_e6262_d_n1, assign6140_e6262_d_n3, assign6140_e6262_d_n4, assign6140_e6262_d_n5, assign6140_e6262_d_n6, assign6140_e6262_d_n7, assign6140_e6262_d_n8, assign6140_e6262_d_n9,) = {
    if (locals.var_guard109 != 0.0) {
        let assign6140_e6260: f64 = (locals.var_fex * locals.var_xqmex);
        (assign6140_e6260, ((locals.var_fex_dn0 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn0)), ((locals.var_fex_dn1 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn1)), ((locals.var_fex_dn3 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn3)), ((locals.var_fex_dn4 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn4)), ((locals.var_fex_dn5 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn5)), ((locals.var_fex_dn6 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn6)), ((locals.var_fex_dn7 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn7)), ((locals.var_fex_dn8 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn8)), ((locals.var_fex_dn9 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn9)),)
    } else {
        (locals.var_xqex, locals.var_xqex_dn0, locals.var_xqex_dn1, locals.var_xqex_dn3, locals.var_xqex_dn4, locals.var_xqex_dn5, locals.var_xqex_dn6, locals.var_xqex_dn7, locals.var_xqex_dn8, locals.var_xqex_dn9,)
    }
};
        locals.var_xqex = assign6140_e6262;
        locals.var_xqex_dn0 = assign6140_e6262_d_n0;
        locals.var_xqex_dn1 = assign6140_e6262_d_n1;
        locals.var_xqex_dn3 = assign6140_e6262_d_n3;
        locals.var_xqex_dn4 = assign6140_e6262_d_n4;
        locals.var_xqex_dn5 = assign6140_e6262_d_n5;
        locals.var_xqex_dn6 = assign6140_e6262_d_n6;
        locals.var_xqex_dn7 = assign6140_e6262_d_n7;
        locals.var_xqex_dn8 = assign6140_e6262_d_n8;
        locals.var_xqex_dn9 = assign6140_e6262_d_n9;

        let assign6150_e6265: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign6150_e6265;

        let (assign6160_e6278, assign6160_e6278_d_n0, assign6160_e6278_d_n1, assign6160_e6278_d_n3, assign6160_e6278_d_n4, assign6160_e6278_d_n5, assign6160_e6278_d_n6, assign6160_e6278_d_n7, assign6160_e6278_d_n8, assign6160_e6278_d_n9,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6160_e6270: f64 = (locals.var_vje * locals.var_inv_vde_t);
        let assign6160_e6271: f64 = (1.0 - assign6160_e6270);
        let assign6160_e6273: f64 = (-p.p66);
        let assign6160_e6274: f64 = (assign6160_e6271).powf(assign6160_e6273);
        let assign6160_e6276: f64 = (assign6160_e6274 - 3.0);
        (assign6160_e6276, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((locals.var_vje_dn0 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn0))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((locals.var_vje_dn0 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn0))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((locals.var_vje_dn1 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn1))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((locals.var_vje_dn1 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn1))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((locals.var_vje_dn3 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn3))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((locals.var_vje_dn3 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn3))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((locals.var_vje_dn4 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn4))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((locals.var_vje_dn4 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn4))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((locals.var_vje_dn5 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn5))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((locals.var_vje_dn5 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn5))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((locals.var_vje_dn6 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn6))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((locals.var_vje_dn6 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn6))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((locals.var_vje_dn7 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn7))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((locals.var_vje_dn7 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn7))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((locals.var_vje_dn8 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn8))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((locals.var_vje_dn8 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn8))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((locals.var_vje_dn9 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn9))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((locals.var_vje_dn9 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn9))) / assign6160_e6271))) },)
    } else {
        (locals.var_dvtevje, locals.var_dvtevje_dn0, locals.var_dvtevje_dn1, locals.var_dvtevje_dn3, locals.var_dvtevje_dn4, locals.var_dvtevje_dn5, locals.var_dvtevje_dn6, locals.var_dvtevje_dn7, locals.var_dvtevje_dn8, locals.var_dvtevje_dn9,)
    }
};
        locals.var_dvtevje = assign6160_e6278;
        locals.var_dvtevje_dn0 = assign6160_e6278_d_n0;
        locals.var_dvtevje_dn1 = assign6160_e6278_d_n1;
        locals.var_dvtevje_dn3 = assign6160_e6278_d_n3;
        locals.var_dvtevje_dn4 = assign6160_e6278_d_n4;
        locals.var_dvtevje_dn5 = assign6160_e6278_d_n5;
        locals.var_dvtevje_dn6 = assign6160_e6278_d_n6;
        locals.var_dvtevje_dn7 = assign6160_e6278_d_n7;
        locals.var_dvtevje_dn8 = assign6160_e6278_d_n8;
        locals.var_dvtevje_dn9 = assign6160_e6278_d_n9;

        let (assign6170_e6286, assign6170_e6286_d_n0, assign6170_e6286_d_n1, assign6170_e6286_d_n3, assign6170_e6286_d_n4, assign6170_e6286_d_n5, assign6170_e6286_d_n6, assign6170_e6286_d_n7, assign6170_e6286_d_n8, assign6170_e6286_d_n9,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6170_e6282: f64 = (locals.var_vb2e1 - locals.var_vfe);
        let assign6170_e6284: f64 = (assign6170_e6282 / locals.var_a_vde);
        (assign6170_e6284, ((((-locals.var_vfe_dn0) * locals.var_a_vde) - (assign6170_e6282 * locals.var_a_vde_dn0)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn1) * locals.var_a_vde) - (assign6170_e6282 * locals.var_a_vde_dn1)) / (locals.var_a_vde * locals.var_a_vde)), ((((locals.var_vb2e1_dn3 - locals.var_vfe_dn3) * locals.var_a_vde) - (assign6170_e6282 * locals.var_a_vde_dn3)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn4) * locals.var_a_vde) - (assign6170_e6282 * locals.var_a_vde_dn4)) / (locals.var_a_vde * locals.var_a_vde)), ((((locals.var_vb2e1_dn5 - locals.var_vfe_dn5) * locals.var_a_vde) - (assign6170_e6282 * locals.var_a_vde_dn5)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn6) * locals.var_a_vde) - (assign6170_e6282 * locals.var_a_vde_dn6)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn7) * locals.var_a_vde) - (assign6170_e6282 * locals.var_a_vde_dn7)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn8) * locals.var_a_vde) - (assign6170_e6282 * locals.var_a_vde_dn8)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn9) * locals.var_a_vde) - (assign6170_e6282 * locals.var_a_vde_dn9)) / (locals.var_a_vde * locals.var_a_vde)),)
    } else {
        (locals.var_vb2e1vfe, locals.var_vb2e1vfe_dn0, locals.var_vb2e1vfe_dn1, locals.var_vb2e1vfe_dn3, locals.var_vb2e1vfe_dn4, locals.var_vb2e1vfe_dn5, locals.var_vb2e1vfe_dn6, locals.var_vb2e1vfe_dn7, locals.var_vb2e1vfe_dn8, locals.var_vb2e1vfe_dn9,)
    }
};
        locals.var_vb2e1vfe = assign6170_e6286;
        locals.var_vb2e1vfe_dn0 = assign6170_e6286_d_n0;
        locals.var_vb2e1vfe_dn1 = assign6170_e6286_d_n1;
        locals.var_vb2e1vfe_dn3 = assign6170_e6286_d_n3;
        locals.var_vb2e1vfe_dn4 = assign6170_e6286_d_n4;
        locals.var_vb2e1vfe_dn5 = assign6170_e6286_d_n5;
        locals.var_vb2e1vfe_dn6 = assign6170_e6286_d_n6;
        locals.var_vb2e1vfe_dn7 = assign6170_e6286_d_n7;
        locals.var_vb2e1vfe_dn8 = assign6170_e6286_d_n8;
        locals.var_vb2e1vfe_dn9 = assign6170_e6286_d_n9;

        let assign6180_e6289: f64 = if locals.var_vb2e1vfe < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign6180_e6289;

        let (assign6190_e6300, assign6190_e6300_d_n0, assign6190_e6300_d_n1, assign6190_e6300_d_n3, assign6190_e6300_d_n4, assign6190_e6300_d_n5, assign6190_e6300_d_n6, assign6190_e6300_d_n7, assign6190_e6300_d_n8, assign6190_e6300_d_n9,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard113 != 0.0)) {
        let assign6190_e6296: f64 = (locals.var_vb2e1vfe).exp();
        let assign6190_e6297: f64 = (1.0 + assign6190_e6296);
        let assign6190_e6298: f64 = (1.0 / assign6190_e6297);
        (assign6190_e6298, (-((assign6190_e6296 * locals.var_vb2e1vfe_dn0) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * locals.var_vb2e1vfe_dn1) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * locals.var_vb2e1vfe_dn3) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * locals.var_vb2e1vfe_dn4) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * locals.var_vb2e1vfe_dn5) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * locals.var_vb2e1vfe_dn6) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * locals.var_vb2e1vfe_dn7) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * locals.var_vb2e1vfe_dn8) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * locals.var_vb2e1vfe_dn9) / (assign6190_e6297 * assign6190_e6297))),)
    } else {
        (locals.var_dvjevb2e1, locals.var_dvjevb2e1_dn0, locals.var_dvjevb2e1_dn1, locals.var_dvjevb2e1_dn3, locals.var_dvjevb2e1_dn4, locals.var_dvjevb2e1_dn5, locals.var_dvjevb2e1_dn6, locals.var_dvjevb2e1_dn7, locals.var_dvjevb2e1_dn8, locals.var_dvjevb2e1_dn9,)
    }
};
        locals.var_dvjevb2e1 = assign6190_e6300;
        locals.var_dvjevb2e1_dn0 = assign6190_e6300_d_n0;
        locals.var_dvjevb2e1_dn1 = assign6190_e6300_d_n1;
        locals.var_dvjevb2e1_dn3 = assign6190_e6300_d_n3;
        locals.var_dvjevb2e1_dn4 = assign6190_e6300_d_n4;
        locals.var_dvjevb2e1_dn5 = assign6190_e6300_d_n5;
        locals.var_dvjevb2e1_dn6 = assign6190_e6300_d_n6;
        locals.var_dvjevb2e1_dn7 = assign6190_e6300_d_n7;
        locals.var_dvjevb2e1_dn8 = assign6190_e6300_d_n8;
        locals.var_dvjevb2e1_dn9 = assign6190_e6300_d_n9;

        let (assign6200_e6315, assign6200_e6315_d_n0, assign6200_e6315_d_n1, assign6200_e6315_d_n3, assign6200_e6315_d_n4, assign6200_e6315_d_n5, assign6200_e6315_d_n6, assign6200_e6315_d_n7, assign6200_e6315_d_n8, assign6200_e6315_d_n9,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard113 == 0.0)) {
        let assign6200_e6306: f64 = (-locals.var_vb2e1vfe);
        let assign6200_e6307: f64 = (assign6200_e6306).exp();
        let assign6200_e6310: f64 = (-locals.var_vb2e1vfe);
        let assign6200_e6311: f64 = (assign6200_e6310).exp();
        let assign6200_e6312: f64 = (1.0 + assign6200_e6311);
        let assign6200_e6313: f64 = (assign6200_e6307 / assign6200_e6312);
        (assign6200_e6313, ((((assign6200_e6307 * (-locals.var_vb2e1vfe_dn0)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-locals.var_vb2e1vfe_dn0)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-locals.var_vb2e1vfe_dn1)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-locals.var_vb2e1vfe_dn1)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-locals.var_vb2e1vfe_dn3)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-locals.var_vb2e1vfe_dn3)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-locals.var_vb2e1vfe_dn4)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-locals.var_vb2e1vfe_dn4)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-locals.var_vb2e1vfe_dn5)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-locals.var_vb2e1vfe_dn5)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-locals.var_vb2e1vfe_dn6)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-locals.var_vb2e1vfe_dn6)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-locals.var_vb2e1vfe_dn7)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-locals.var_vb2e1vfe_dn7)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-locals.var_vb2e1vfe_dn8)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-locals.var_vb2e1vfe_dn8)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-locals.var_vb2e1vfe_dn9)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-locals.var_vb2e1vfe_dn9)))) / (assign6200_e6312 * assign6200_e6312)),)
    } else {
        (locals.var_dvjevb2e1, locals.var_dvjevb2e1_dn0, locals.var_dvjevb2e1_dn1, locals.var_dvjevb2e1_dn3, locals.var_dvjevb2e1_dn4, locals.var_dvjevb2e1_dn5, locals.var_dvjevb2e1_dn6, locals.var_dvjevb2e1_dn7, locals.var_dvjevb2e1_dn8, locals.var_dvjevb2e1_dn9,)
    }
};
        locals.var_dvjevb2e1 = assign6200_e6315;
        locals.var_dvjevb2e1_dn0 = assign6200_e6315_d_n0;
        locals.var_dvjevb2e1_dn1 = assign6200_e6315_d_n1;
        locals.var_dvjevb2e1_dn3 = assign6200_e6315_d_n3;
        locals.var_dvjevb2e1_dn4 = assign6200_e6315_d_n4;
        locals.var_dvjevb2e1_dn5 = assign6200_e6315_d_n5;
        locals.var_dvjevb2e1_dn6 = assign6200_e6315_d_n6;
        locals.var_dvjevb2e1_dn7 = assign6200_e6315_d_n7;
        locals.var_dvjevb2e1_dn8 = assign6200_e6315_d_n8;
        locals.var_dvjevb2e1_dn9 = assign6200_e6315_d_n9;

        let (assign6210_e6323, assign6210_e6323_d_n0, assign6210_e6323_d_n1, assign6210_e6323_d_n3, assign6210_e6323_d_n4, assign6210_e6323_d_n5, assign6210_e6323_d_n6, assign6210_e6323_d_n7, assign6210_e6323_d_n8, assign6210_e6323_d_n9,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6210_e6319: f64 = (locals.var_dvtevje * locals.var_dvjevb2e1);
        let assign6210_e6321: f64 = (assign6210_e6319 + 3.0);
        (assign6210_e6321, ((locals.var_dvtevje_dn0 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn0)), ((locals.var_dvtevje_dn1 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn1)), ((locals.var_dvtevje_dn3 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn3)), ((locals.var_dvtevje_dn4 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn4)), ((locals.var_dvtevje_dn5 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn5)), ((locals.var_dvtevje_dn6 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn6)), ((locals.var_dvtevje_dn7 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn7)), ((locals.var_dvtevje_dn8 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn8)), ((locals.var_dvtevje_dn9 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn9)),)
    } else {
        (locals.var_dvtevb2e1, locals.var_dvtevb2e1_dn0, locals.var_dvtevb2e1_dn1, locals.var_dvtevb2e1_dn3, locals.var_dvtevb2e1_dn4, locals.var_dvtevb2e1_dn5, locals.var_dvtevb2e1_dn6, locals.var_dvtevb2e1_dn7, locals.var_dvtevb2e1_dn8, locals.var_dvtevb2e1_dn9,)
    }
};
        locals.var_dvtevb2e1 = assign6210_e6323;
        locals.var_dvtevb2e1_dn0 = assign6210_e6323_d_n0;
        locals.var_dvtevb2e1_dn1 = assign6210_e6323_d_n1;
        locals.var_dvtevb2e1_dn3 = assign6210_e6323_d_n3;
        locals.var_dvtevb2e1_dn4 = assign6210_e6323_d_n4;
        locals.var_dvtevb2e1_dn5 = assign6210_e6323_d_n5;
        locals.var_dvtevb2e1_dn6 = assign6210_e6323_d_n6;
        locals.var_dvtevb2e1_dn7 = assign6210_e6323_d_n7;
        locals.var_dvtevb2e1_dn8 = assign6210_e6323_d_n8;
        locals.var_dvtevb2e1_dn9 = assign6210_e6323_d_n9;

        let (assign6220_e6333, assign6220_e6333_d_n0, assign6220_e6333_d_n1, assign6220_e6333_d_n3, assign6220_e6333_d_n4, assign6220_e6333_d_n5, assign6220_e6333_d_n6, assign6220_e6333_d_n7, assign6220_e6333_d_n8, assign6220_e6333_d_n9,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6220_e6327: f64 = (1.0 - p.p67);
        let assign6220_e6329: f64 = (assign6220_e6327 * locals.var_cje_t);
        let assign6220_e6331: f64 = (assign6220_e6329 * locals.var_dvtevb2e1);
        (assign6220_e6331, (((assign6220_e6327 * locals.var_cje_t_dn0) * locals.var_dvtevb2e1) + (assign6220_e6329 * locals.var_dvtevb2e1_dn0)), (((assign6220_e6327 * locals.var_cje_t_dn1) * locals.var_dvtevb2e1) + (assign6220_e6329 * locals.var_dvtevb2e1_dn1)), (((assign6220_e6327 * locals.var_cje_t_dn3) * locals.var_dvtevb2e1) + (assign6220_e6329 * locals.var_dvtevb2e1_dn3)), (((assign6220_e6327 * locals.var_cje_t_dn4) * locals.var_dvtevb2e1) + (assign6220_e6329 * locals.var_dvtevb2e1_dn4)), (((assign6220_e6327 * locals.var_cje_t_dn5) * locals.var_dvtevb2e1) + (assign6220_e6329 * locals.var_dvtevb2e1_dn5)), (((assign6220_e6327 * locals.var_cje_t_dn6) * locals.var_dvtevb2e1) + (assign6220_e6329 * locals.var_dvtevb2e1_dn6)), (((assign6220_e6327 * locals.var_cje_t_dn7) * locals.var_dvtevb2e1) + (assign6220_e6329 * locals.var_dvtevb2e1_dn7)), (((assign6220_e6327 * locals.var_cje_t_dn8) * locals.var_dvtevb2e1) + (assign6220_e6329 * locals.var_dvtevb2e1_dn8)), (((assign6220_e6327 * locals.var_cje_t_dn9) * locals.var_dvtevb2e1) + (assign6220_e6329 * locals.var_dvtevb2e1_dn9)),)
    } else {
        (locals.var_dqtevb2e1, locals.var_dqtevb2e1_dn0, locals.var_dqtevb2e1_dn1, locals.var_dqtevb2e1_dn3, locals.var_dqtevb2e1_dn4, locals.var_dqtevb2e1_dn5, locals.var_dqtevb2e1_dn6, locals.var_dqtevb2e1_dn7, locals.var_dqtevb2e1_dn8, locals.var_dqtevb2e1_dn9,)
    }
};
        locals.var_dqtevb2e1 = assign6220_e6333;
        locals.var_dqtevb2e1_dn0 = assign6220_e6333_d_n0;
        locals.var_dqtevb2e1_dn1 = assign6220_e6333_d_n1;
        locals.var_dqtevb2e1_dn3 = assign6220_e6333_d_n3;
        locals.var_dqtevb2e1_dn4 = assign6220_e6333_d_n4;
        locals.var_dqtevb2e1_dn5 = assign6220_e6333_d_n5;
        locals.var_dqtevb2e1_dn6 = assign6220_e6333_d_n6;
        locals.var_dqtevb2e1_dn7 = assign6220_e6333_d_n7;
        locals.var_dqtevb2e1_dn8 = assign6220_e6333_d_n8;
        locals.var_dqtevb2e1_dn9 = assign6220_e6333_d_n9;

        let (assign6230_e6350, assign6230_e6350_d_n0, assign6230_e6350_d_n1, assign6230_e6350_d_n3, assign6230_e6350_d_n4, assign6230_e6350_d_n5, assign6230_e6350_d_n6, assign6230_e6350_d_n7, assign6230_e6350_d_n8, assign6230_e6350_d_n9,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6230_e6337: f64 = (locals.var_if0 * locals.var_evb2e1);
        let assign6230_e6339: f64 = (assign6230_e6337 * locals.var_vtinv);
        let assign6230_e6341: f64 = (assign6230_e6339 / locals.var_nff_t);
        let assign6230_e6345: f64 = (1.0 + locals.var_f1);
        let assign6230_e6346: f64 = (assign6230_e6345).sqrt();
        let assign6230_e6347: f64 = (0.5 / assign6230_e6346);
        let assign6230_e6348: f64 = (assign6230_e6341 * assign6230_e6347);
        (assign6230_e6348, ((((((((locals.var_if0_dn0 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn0)) * locals.var_vtinv) * locals.var_nff_t) - (assign6230_e6339 * locals.var_nff_t_dn0)) / (locals.var_nff_t * locals.var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (locals.var_f1_dn0 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), ((((((((locals.var_if0_dn1 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn1)) * locals.var_vtinv) * locals.var_nff_t) - (assign6230_e6339 * locals.var_nff_t_dn1)) / (locals.var_nff_t * locals.var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (locals.var_f1_dn1 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), ((((((((locals.var_if0_dn3 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn3)) * locals.var_vtinv) * locals.var_nff_t) - (assign6230_e6339 * locals.var_nff_t_dn3)) / (locals.var_nff_t * locals.var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (locals.var_f1_dn3 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), ((((((((locals.var_if0_dn4 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn4)) * locals.var_vtinv) * locals.var_nff_t) - (assign6230_e6339 * locals.var_nff_t_dn4)) / (locals.var_nff_t * locals.var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (locals.var_f1_dn4 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), ((((((((locals.var_if0_dn5 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn5)) * locals.var_vtinv) * locals.var_nff_t) - (assign6230_e6339 * locals.var_nff_t_dn5)) / (locals.var_nff_t * locals.var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (locals.var_f1_dn5 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), ((((((((locals.var_if0_dn6 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn6)) * locals.var_vtinv) * locals.var_nff_t) - (assign6230_e6339 * locals.var_nff_t_dn6)) / (locals.var_nff_t * locals.var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (locals.var_f1_dn6 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), ((((((((locals.var_if0_dn7 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn7)) * locals.var_vtinv) * locals.var_nff_t) - (assign6230_e6339 * locals.var_nff_t_dn7)) / (locals.var_nff_t * locals.var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (locals.var_f1_dn7 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), ((((((((locals.var_if0_dn8 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn8)) * locals.var_vtinv) * locals.var_nff_t) - (assign6230_e6339 * locals.var_nff_t_dn8)) / (locals.var_nff_t * locals.var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (locals.var_f1_dn8 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), ((((((((locals.var_if0_dn9 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn9)) * locals.var_vtinv) * locals.var_nff_t) - (assign6230_e6339 * locals.var_nff_t_dn9)) / (locals.var_nff_t * locals.var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (locals.var_f1_dn9 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))),)
    } else {
        (locals.var_dn0vb2e1, locals.var_dn0vb2e1_dn0, locals.var_dn0vb2e1_dn1, locals.var_dn0vb2e1_dn3, locals.var_dn0vb2e1_dn4, locals.var_dn0vb2e1_dn5, locals.var_dn0vb2e1_dn6, locals.var_dn0vb2e1_dn7, locals.var_dn0vb2e1_dn8, locals.var_dn0vb2e1_dn9,)
    }
};
        locals.var_dn0vb2e1 = assign6230_e6350;
        locals.var_dn0vb2e1_dn0 = assign6230_e6350_d_n0;
        locals.var_dn0vb2e1_dn1 = assign6230_e6350_d_n1;
        locals.var_dn0vb2e1_dn3 = assign6230_e6350_d_n3;
        locals.var_dn0vb2e1_dn4 = assign6230_e6350_d_n4;
        locals.var_dn0vb2e1_dn5 = assign6230_e6350_d_n5;
        locals.var_dn0vb2e1_dn6 = assign6230_e6350_d_n6;
        locals.var_dn0vb2e1_dn7 = assign6230_e6350_d_n7;
        locals.var_dn0vb2e1_dn8 = assign6230_e6350_d_n8;
        locals.var_dn0vb2e1_dn9 = assign6230_e6350_d_n9;

        let (assign6240_e6360, assign6240_e6360_d_n0, assign6240_e6360_d_n1, assign6240_e6360_d_n3, assign6240_e6360_d_n4, assign6240_e6360_d_n5, assign6240_e6360_d_n6, assign6240_e6360_d_n7, assign6240_e6360_d_n8, assign6240_e6360_d_n9,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6240_e6354: f64 = (0.5 * locals.var_qb0);
        let assign6240_e6356: f64 = (assign6240_e6354 * locals.var_q1q);
        let assign6240_e6358: f64 = (assign6240_e6356 * locals.var_dn0vb2e1);
        (assign6240_e6358, (((assign6240_e6354 * locals.var_q1q_dn0) * locals.var_dn0vb2e1) + (assign6240_e6356 * locals.var_dn0vb2e1_dn0)), (((assign6240_e6354 * locals.var_q1q_dn1) * locals.var_dn0vb2e1) + (assign6240_e6356 * locals.var_dn0vb2e1_dn1)), (((assign6240_e6354 * locals.var_q1q_dn3) * locals.var_dn0vb2e1) + (assign6240_e6356 * locals.var_dn0vb2e1_dn3)), (((assign6240_e6354 * locals.var_q1q_dn4) * locals.var_dn0vb2e1) + (assign6240_e6356 * locals.var_dn0vb2e1_dn4)), (((assign6240_e6354 * locals.var_q1q_dn5) * locals.var_dn0vb2e1) + (assign6240_e6356 * locals.var_dn0vb2e1_dn5)), (((assign6240_e6354 * locals.var_q1q_dn6) * locals.var_dn0vb2e1) + (assign6240_e6356 * locals.var_dn0vb2e1_dn6)), (((assign6240_e6354 * locals.var_q1q_dn7) * locals.var_dn0vb2e1) + (assign6240_e6356 * locals.var_dn0vb2e1_dn7)), (((assign6240_e6354 * locals.var_q1q_dn8) * locals.var_dn0vb2e1) + (assign6240_e6356 * locals.var_dn0vb2e1_dn8)), (((assign6240_e6354 * locals.var_q1q_dn9) * locals.var_dn0vb2e1) + (assign6240_e6356 * locals.var_dn0vb2e1_dn9)),)
    } else {
        (locals.var_dqbevb2e1, locals.var_dqbevb2e1_dn0, locals.var_dqbevb2e1_dn1, locals.var_dqbevb2e1_dn3, locals.var_dqbevb2e1_dn4, locals.var_dqbevb2e1_dn5, locals.var_dqbevb2e1_dn6, locals.var_dqbevb2e1_dn7, locals.var_dqbevb2e1_dn8, locals.var_dqbevb2e1_dn9,)
    }
};
        locals.var_dqbevb2e1 = assign6240_e6360;
        locals.var_dqbevb2e1_dn0 = assign6240_e6360_d_n0;
        locals.var_dqbevb2e1_dn1 = assign6240_e6360_d_n1;
        locals.var_dqbevb2e1_dn3 = assign6240_e6360_d_n3;
        locals.var_dqbevb2e1_dn4 = assign6240_e6360_d_n4;
        locals.var_dqbevb2e1_dn5 = assign6240_e6360_d_n5;
        locals.var_dqbevb2e1_dn6 = assign6240_e6360_d_n6;
        locals.var_dqbevb2e1_dn7 = assign6240_e6360_d_n7;
        locals.var_dqbevb2e1_dn8 = assign6240_e6360_d_n8;
        locals.var_dqbevb2e1_dn9 = assign6240_e6360_d_n9;

        let (assign6250_e6368, assign6250_e6368_d_n0, assign6250_e6368_d_n1, assign6250_e6368_d_n3, assign6250_e6368_d_n4, assign6250_e6368_d_n5, assign6250_e6368_d_n6, assign6250_e6368_d_n7, assign6250_e6368_d_n8, assign6250_e6368_d_n9,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6250_e6365: f64 = (p.p84 * locals.var_vt);
        let assign6250_e6366: f64 = (locals.var_qe_qs / assign6250_e6365);
        (assign6250_e6366, (locals.var_qe_qs_dn0 / assign6250_e6365), (locals.var_qe_qs_dn1 / assign6250_e6365), (locals.var_qe_qs_dn3 / assign6250_e6365), (locals.var_qe_qs_dn4 / assign6250_e6365), (locals.var_qe_qs_dn5 / assign6250_e6365), (locals.var_qe_qs_dn6 / assign6250_e6365), (locals.var_qe_qs_dn7 / assign6250_e6365), (locals.var_qe_qs_dn8 / assign6250_e6365), (locals.var_qe_qs_dn9 / assign6250_e6365),)
    } else {
        (locals.var_dqevb2e1, locals.var_dqevb2e1_dn0, locals.var_dqevb2e1_dn1, locals.var_dqevb2e1_dn3, locals.var_dqevb2e1_dn4, locals.var_dqevb2e1_dn5, locals.var_dqevb2e1_dn6, locals.var_dqevb2e1_dn7, locals.var_dqevb2e1_dn8, locals.var_dqevb2e1_dn9,)
    }
};
        locals.var_dqevb2e1 = assign6250_e6368;
        locals.var_dqevb2e1_dn0 = assign6250_e6368_d_n0;
        locals.var_dqevb2e1_dn1 = assign6250_e6368_d_n1;
        locals.var_dqevb2e1_dn3 = assign6250_e6368_d_n3;
        locals.var_dqevb2e1_dn4 = assign6250_e6368_d_n4;
        locals.var_dqevb2e1_dn5 = assign6250_e6368_d_n5;
        locals.var_dqevb2e1_dn6 = assign6250_e6368_d_n6;
        locals.var_dqevb2e1_dn7 = assign6250_e6368_d_n7;
        locals.var_dqevb2e1_dn8 = assign6250_e6368_d_n8;
        locals.var_dqevb2e1_dn9 = assign6250_e6368_d_n9;

        let (assign6260_e6380, assign6260_e6380_d_n0, assign6260_e6380_d_n1, assign6260_e6380_d_n3, assign6260_e6380_d_n4, assign6260_e6380_d_n5, assign6260_e6380_d_n6, assign6260_e6380_d_n7, assign6260_e6380_d_n8, assign6260_e6380_d_n9,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6260_e6372: f64 = (0.2 * locals.var_vb1b2);
        let assign6260_e6375: f64 = (locals.var_dqtevb2e1 + locals.var_dqbevb2e1);
        let assign6260_e6377: f64 = (assign6260_e6375 + locals.var_dqevb2e1);
        let assign6260_e6378: f64 = (assign6260_e6372 * assign6260_e6377);
        (assign6260_e6378, (assign6260_e6372 * ((locals.var_dqtevb2e1_dn0 + locals.var_dqbevb2e1_dn0) + locals.var_dqevb2e1_dn0)), (assign6260_e6372 * ((locals.var_dqtevb2e1_dn1 + locals.var_dqbevb2e1_dn1) + locals.var_dqevb2e1_dn1)), (assign6260_e6372 * ((locals.var_dqtevb2e1_dn3 + locals.var_dqbevb2e1_dn3) + locals.var_dqevb2e1_dn3)), (((0.2 * locals.var_vb1b2_dn4) * assign6260_e6377) + (assign6260_e6372 * ((locals.var_dqtevb2e1_dn4 + locals.var_dqbevb2e1_dn4) + locals.var_dqevb2e1_dn4))), (((0.2 * locals.var_vb1b2_dn5) * assign6260_e6377) + (assign6260_e6372 * ((locals.var_dqtevb2e1_dn5 + locals.var_dqbevb2e1_dn5) + locals.var_dqevb2e1_dn5))), (assign6260_e6372 * ((locals.var_dqtevb2e1_dn6 + locals.var_dqbevb2e1_dn6) + locals.var_dqevb2e1_dn6)), (assign6260_e6372 * ((locals.var_dqtevb2e1_dn7 + locals.var_dqbevb2e1_dn7) + locals.var_dqevb2e1_dn7)), (assign6260_e6372 * ((locals.var_dqtevb2e1_dn8 + locals.var_dqbevb2e1_dn8) + locals.var_dqevb2e1_dn8)), (assign6260_e6372 * ((locals.var_dqtevb2e1_dn9 + locals.var_dqbevb2e1_dn9) + locals.var_dqevb2e1_dn9)),)
    } else {
        (locals.var_qb1b2, locals.var_qb1b2_dn0, locals.var_qb1b2_dn1, locals.var_qb1b2_dn3, locals.var_qb1b2_dn4, locals.var_qb1b2_dn5, locals.var_qb1b2_dn6, locals.var_qb1b2_dn7, locals.var_qb1b2_dn8, locals.var_qb1b2_dn9,)
    }
};
        locals.var_qb1b2 = assign6260_e6380;
        locals.var_qb1b2_dn0 = assign6260_e6380_d_n0;
        locals.var_qb1b2_dn1 = assign6260_e6380_d_n1;
        locals.var_qb1b2_dn3 = assign6260_e6380_d_n3;
        locals.var_qb1b2_dn4 = assign6260_e6380_d_n4;
        locals.var_qb1b2_dn5 = assign6260_e6380_d_n5;
        locals.var_qb1b2_dn6 = assign6260_e6380_d_n6;
        locals.var_qb1b2_dn7 = assign6260_e6380_d_n7;
        locals.var_qb1b2_dn8 = assign6260_e6380_d_n8;
        locals.var_qb1b2_dn9 = assign6260_e6380_d_n9;

        let (assign6270_e6388, assign6270_e6388_d_n0, assign6270_e6388_d_n1, assign6270_e6388_d_n3, assign6270_e6388_d_n4, assign6270_e6388_d_n5, assign6270_e6388_d_n6, assign6270_e6388_d_n7, assign6270_e6388_d_n8, assign6270_e6388_d_n9,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6270_e6384: f64 = (1.0 - p.p94);
        let assign6270_e6386: f64 = (assign6270_e6384 * locals.var_qe_qs);
        (assign6270_e6386, (assign6270_e6384 * locals.var_qe_qs_dn0), (assign6270_e6384 * locals.var_qe_qs_dn1), (assign6270_e6384 * locals.var_qe_qs_dn3), (assign6270_e6384 * locals.var_qe_qs_dn4), (assign6270_e6384 * locals.var_qe_qs_dn5), (assign6270_e6384 * locals.var_qe_qs_dn6), (assign6270_e6384 * locals.var_qe_qs_dn7), (assign6270_e6384 * locals.var_qe_qs_dn8), (assign6270_e6384 * locals.var_qe_qs_dn9),)
    } else {
        (locals.var_qe, locals.var_qe_dn0, locals.var_qe_dn1, locals.var_qe_dn3, locals.var_qe_dn4, locals.var_qe_dn5, locals.var_qe_dn6, locals.var_qe_dn7, locals.var_qe_dn8, locals.var_qe_dn9,)
    }
};
        locals.var_qe = assign6270_e6388;
        locals.var_qe_dn0 = assign6270_e6388_d_n0;
        locals.var_qe_dn1 = assign6270_e6388_d_n1;
        locals.var_qe_dn3 = assign6270_e6388_d_n3;
        locals.var_qe_dn4 = assign6270_e6388_d_n4;
        locals.var_qe_dn5 = assign6270_e6388_d_n5;
        locals.var_qe_dn6 = assign6270_e6388_d_n6;
        locals.var_qe_dn7 = assign6270_e6388_d_n7;
        locals.var_qe_dn8 = assign6270_e6388_d_n8;
        locals.var_qe_dn9 = assign6270_e6388_d_n9;

    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6280_e6396, assign6280_e6396_d_n0, assign6280_e6396_d_n1, assign6280_e6396_d_n3, assign6280_e6396_d_n4, assign6280_e6396_d_n5, assign6280_e6396_d_n6, assign6280_e6396_d_n7, assign6280_e6396_d_n8, assign6280_e6396_d_n9,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6280_e6393: f64 = (p.p94 * locals.var_qe_qs);
        let assign6280_e6394: f64 = (locals.var_qbe_qs + assign6280_e6393);
        (assign6280_e6394, (locals.var_qbe_qs_dn0 + (p.p94 * locals.var_qe_qs_dn0)), (locals.var_qbe_qs_dn1 + (p.p94 * locals.var_qe_qs_dn1)), (locals.var_qbe_qs_dn3 + (p.p94 * locals.var_qe_qs_dn3)), (locals.var_qbe_qs_dn4 + (p.p94 * locals.var_qe_qs_dn4)), (locals.var_qbe_qs_dn5 + (p.p94 * locals.var_qe_qs_dn5)), (locals.var_qbe_qs_dn6 + (p.p94 * locals.var_qe_qs_dn6)), (locals.var_qbe_qs_dn7 + (p.p94 * locals.var_qe_qs_dn7)), (locals.var_qbe_qs_dn8 + (p.p94 * locals.var_qe_qs_dn8)), (locals.var_qbe_qs_dn9 + (p.p94 * locals.var_qe_qs_dn9)),)
    } else {
        (locals.var_qbe_qs_eff, locals.var_qbe_qs_eff_dn0, locals.var_qbe_qs_eff_dn1, locals.var_qbe_qs_eff_dn3, locals.var_qbe_qs_eff_dn4, locals.var_qbe_qs_eff_dn5, locals.var_qbe_qs_eff_dn6, locals.var_qbe_qs_eff_dn7, locals.var_qbe_qs_eff_dn8, locals.var_qbe_qs_eff_dn9,)
    }
};
        locals.var_qbe_qs_eff = assign6280_e6396;
        locals.var_qbe_qs_eff_dn0 = assign6280_e6396_d_n0;
        locals.var_qbe_qs_eff_dn1 = assign6280_e6396_d_n1;
        locals.var_qbe_qs_eff_dn3 = assign6280_e6396_d_n3;
        locals.var_qbe_qs_eff_dn4 = assign6280_e6396_d_n4;
        locals.var_qbe_qs_eff_dn5 = assign6280_e6396_d_n5;
        locals.var_qbe_qs_eff_dn6 = assign6280_e6396_d_n6;
        locals.var_qbe_qs_eff_dn7 = assign6280_e6396_d_n7;
        locals.var_qbe_qs_eff_dn8 = assign6280_e6396_d_n8;
        locals.var_qbe_qs_eff_dn9 = assign6280_e6396_d_n9;

        let (assign6290_e6404, assign6290_e6404_d_n0, assign6290_e6404_d_n1, assign6290_e6404_d_n3, assign6290_e6404_d_n4, assign6290_e6404_d_n5, assign6290_e6404_d_n6, assign6290_e6404_d_n7, assign6290_e6404_d_n8, assign6290_e6404_d_n9,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6290_e6400: f64 = (p.p93 * locals.var_qbe_qs_eff);
        let assign6290_e6402: f64 = (assign6290_e6400 + locals.var_qbc_qs);
        (assign6290_e6402, ((p.p93 * locals.var_qbe_qs_eff_dn0) + locals.var_qbc_qs_dn0), ((p.p93 * locals.var_qbe_qs_eff_dn1) + locals.var_qbc_qs_dn1), ((p.p93 * locals.var_qbe_qs_eff_dn3) + locals.var_qbc_qs_dn3), ((p.p93 * locals.var_qbe_qs_eff_dn4) + locals.var_qbc_qs_dn4), ((p.p93 * locals.var_qbe_qs_eff_dn5) + locals.var_qbc_qs_dn5), ((p.p93 * locals.var_qbe_qs_eff_dn6) + locals.var_qbc_qs_dn6), ((p.p93 * locals.var_qbe_qs_eff_dn7) + locals.var_qbc_qs_dn7), ((p.p93 * locals.var_qbe_qs_eff_dn8) + locals.var_qbc_qs_dn8), ((p.p93 * locals.var_qbe_qs_eff_dn9) + locals.var_qbc_qs_dn9),)
    } else {
        (locals.var_qbc, locals.var_qbc_dn0, locals.var_qbc_dn1, locals.var_qbc_dn3, locals.var_qbc_dn4, locals.var_qbc_dn5, locals.var_qbc_dn6, locals.var_qbc_dn7, locals.var_qbc_dn8, locals.var_qbc_dn9,)
    }
};
        locals.var_qbc = assign6290_e6404;
        locals.var_qbc_dn0 = assign6290_e6404_d_n0;
        locals.var_qbc_dn1 = assign6290_e6404_d_n1;
        locals.var_qbc_dn3 = assign6290_e6404_d_n3;
        locals.var_qbc_dn4 = assign6290_e6404_d_n4;
        locals.var_qbc_dn5 = assign6290_e6404_d_n5;
        locals.var_qbc_dn6 = assign6290_e6404_d_n6;
        locals.var_qbc_dn7 = assign6290_e6404_d_n7;
        locals.var_qbc_dn8 = assign6290_e6404_d_n8;
        locals.var_qbc_dn9 = assign6290_e6404_d_n9;

        let (assign6300_e6412, assign6300_e6412_d_n0, assign6300_e6412_d_n1, assign6300_e6412_d_n3, assign6300_e6412_d_n4, assign6300_e6412_d_n5, assign6300_e6412_d_n6, assign6300_e6412_d_n7, assign6300_e6412_d_n8, assign6300_e6412_d_n9,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6300_e6408: f64 = (1.0 - p.p93);
        let assign6300_e6410: f64 = (assign6300_e6408 * locals.var_qbe_qs_eff);
        (assign6300_e6410, (assign6300_e6408 * locals.var_qbe_qs_eff_dn0), (assign6300_e6408 * locals.var_qbe_qs_eff_dn1), (assign6300_e6408 * locals.var_qbe_qs_eff_dn3), (assign6300_e6408 * locals.var_qbe_qs_eff_dn4), (assign6300_e6408 * locals.var_qbe_qs_eff_dn5), (assign6300_e6408 * locals.var_qbe_qs_eff_dn6), (assign6300_e6408 * locals.var_qbe_qs_eff_dn7), (assign6300_e6408 * locals.var_qbe_qs_eff_dn8), (assign6300_e6408 * locals.var_qbe_qs_eff_dn9),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn1, locals.var_qbe_dn3, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn8, locals.var_qbe_dn9,)
    }
};
        locals.var_qbe = assign6300_e6412;
        locals.var_qbe_dn0 = assign6300_e6412_d_n0;
        locals.var_qbe_dn1 = assign6300_e6412_d_n1;
        locals.var_qbe_dn3 = assign6300_e6412_d_n3;
        locals.var_qbe_dn4 = assign6300_e6412_d_n4;
        locals.var_qbe_dn5 = assign6300_e6412_d_n5;
        locals.var_qbe_dn6 = assign6300_e6412_d_n6;
        locals.var_qbe_dn7 = assign6300_e6412_d_n7;
        locals.var_qbe_dn8 = assign6300_e6412_d_n8;
        locals.var_qbe_dn9 = assign6300_e6412_d_n9;

        let (assign6310_e6417, assign6310_e6417_d_n0, assign6310_e6417_d_n1, assign6310_e6417_d_n3, assign6310_e6417_d_n4, assign6310_e6417_d_n5, assign6310_e6417_d_n6, assign6310_e6417_d_n7, assign6310_e6417_d_n8, assign6310_e6417_d_n9,) = {
    if (locals.var_guard112 == 0.0) {
        (locals.var_qbe_qs, locals.var_qbe_qs_dn0, locals.var_qbe_qs_dn1, locals.var_qbe_qs_dn3, locals.var_qbe_qs_dn4, locals.var_qbe_qs_dn5, locals.var_qbe_qs_dn6, locals.var_qbe_qs_dn7, locals.var_qbe_qs_dn8, locals.var_qbe_qs_dn9,)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn1, locals.var_qbe_dn3, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn8, locals.var_qbe_dn9,)
    }
};
        locals.var_qbe = assign6310_e6417;
        locals.var_qbe_dn0 = assign6310_e6417_d_n0;
        locals.var_qbe_dn1 = assign6310_e6417_d_n1;
        locals.var_qbe_dn3 = assign6310_e6417_d_n3;
        locals.var_qbe_dn4 = assign6310_e6417_d_n4;
        locals.var_qbe_dn5 = assign6310_e6417_d_n5;
        locals.var_qbe_dn6 = assign6310_e6417_d_n6;
        locals.var_qbe_dn7 = assign6310_e6417_d_n7;
        locals.var_qbe_dn8 = assign6310_e6417_d_n8;
        locals.var_qbe_dn9 = assign6310_e6417_d_n9;

        let (assign6320_e6422, assign6320_e6422_d_n0, assign6320_e6422_d_n1, assign6320_e6422_d_n3, assign6320_e6422_d_n4, assign6320_e6422_d_n5, assign6320_e6422_d_n6, assign6320_e6422_d_n7, assign6320_e6422_d_n8, assign6320_e6422_d_n9,) = {
    if (locals.var_guard112 == 0.0) {
        (locals.var_qbc_qs, locals.var_qbc_qs_dn0, locals.var_qbc_qs_dn1, locals.var_qbc_qs_dn3, locals.var_qbc_qs_dn4, locals.var_qbc_qs_dn5, locals.var_qbc_qs_dn6, locals.var_qbc_qs_dn7, locals.var_qbc_qs_dn8, locals.var_qbc_qs_dn9,)
    } else {
        (locals.var_qbc, locals.var_qbc_dn0, locals.var_qbc_dn1, locals.var_qbc_dn3, locals.var_qbc_dn4, locals.var_qbc_dn5, locals.var_qbc_dn6, locals.var_qbc_dn7, locals.var_qbc_dn8, locals.var_qbc_dn9,)
    }
};
        locals.var_qbc = assign6320_e6422;
        locals.var_qbc_dn0 = assign6320_e6422_d_n0;
        locals.var_qbc_dn1 = assign6320_e6422_d_n1;
        locals.var_qbc_dn3 = assign6320_e6422_d_n3;
        locals.var_qbc_dn4 = assign6320_e6422_d_n4;
        locals.var_qbc_dn5 = assign6320_e6422_d_n5;
        locals.var_qbc_dn6 = assign6320_e6422_d_n6;
        locals.var_qbc_dn7 = assign6320_e6422_d_n7;
        locals.var_qbc_dn8 = assign6320_e6422_d_n8;
        locals.var_qbc_dn9 = assign6320_e6422_d_n9;

        let (assign6330_e6427, assign6330_e6427_d_n0, assign6330_e6427_d_n1, assign6330_e6427_d_n3, assign6330_e6427_d_n4, assign6330_e6427_d_n5, assign6330_e6427_d_n6, assign6330_e6427_d_n7, assign6330_e6427_d_n8, assign6330_e6427_d_n9,) = {
    if (locals.var_guard112 == 0.0) {
        (locals.var_qe_qs, locals.var_qe_qs_dn0, locals.var_qe_qs_dn1, locals.var_qe_qs_dn3, locals.var_qe_qs_dn4, locals.var_qe_qs_dn5, locals.var_qe_qs_dn6, locals.var_qe_qs_dn7, locals.var_qe_qs_dn8, locals.var_qe_qs_dn9,)
    } else {
        (locals.var_qe, locals.var_qe_dn0, locals.var_qe_dn1, locals.var_qe_dn3, locals.var_qe_dn4, locals.var_qe_dn5, locals.var_qe_dn6, locals.var_qe_dn7, locals.var_qe_dn8, locals.var_qe_dn9,)
    }
};
        locals.var_qe = assign6330_e6427;
        locals.var_qe_dn0 = assign6330_e6427_d_n0;
        locals.var_qe_dn1 = assign6330_e6427_d_n1;
        locals.var_qe_dn3 = assign6330_e6427_d_n3;
        locals.var_qe_dn4 = assign6330_e6427_d_n4;
        locals.var_qe_dn5 = assign6330_e6427_d_n5;
        locals.var_qe_dn6 = assign6330_e6427_d_n6;
        locals.var_qe_dn7 = assign6330_e6427_d_n7;
        locals.var_qe_dn8 = assign6330_e6427_d_n8;
        locals.var_qe_dn9 = assign6330_e6427_d_n9;

        let assign6440_e6470: f64 = (locals.var_if_ + locals.var_ir);
        let assign6440_e6472: f64 = (assign6440_e6470 / locals.var_qbi);
        locals.var_in_n = assign6440_e6472;
        locals.var_in_n_dn0 = ((((locals.var_if__dn0 + locals.var_ir_dn0) * locals.var_qbi) - (assign6440_e6470 * locals.var_qbi_dn0)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn1 = ((((locals.var_if__dn1 + locals.var_ir_dn1) * locals.var_qbi) - (assign6440_e6470 * locals.var_qbi_dn1)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn3 = ((((locals.var_if__dn3 + locals.var_ir_dn3) * locals.var_qbi) - (assign6440_e6470 * locals.var_qbi_dn3)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn4 = ((((locals.var_if__dn4 + locals.var_ir_dn4) * locals.var_qbi) - (assign6440_e6470 * locals.var_qbi_dn4)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn5 = ((((locals.var_if__dn5 + locals.var_ir_dn5) * locals.var_qbi) - (assign6440_e6470 * locals.var_qbi_dn5)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn6 = ((((locals.var_if__dn6 + locals.var_ir_dn6) * locals.var_qbi) - (assign6440_e6470 * locals.var_qbi_dn6)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn7 = ((((locals.var_if__dn7 + locals.var_ir_dn7) * locals.var_qbi) - (assign6440_e6470 * locals.var_qbi_dn7)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn8 = ((((locals.var_if__dn8 + locals.var_ir_dn8) * locals.var_qbi) - (assign6440_e6470 * locals.var_qbi_dn8)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn9 = ((((locals.var_if__dn9 + locals.var_ir_dn9) * locals.var_qbi) - (assign6440_e6470 * locals.var_qbi_dn9)) / (locals.var_qbi * locals.var_qbi));

        let assign6500_e6505: f64 = if locals.var_in_n > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign6500_e6505;

        let (assign6510_e6513, assign6510_e6513_d_n0, assign6510_e6513_d_n1, assign6510_e6513_d_n3, assign6510_e6513_d_n4, assign6510_e6513_d_n5, assign6510_e6513_d_n6, assign6510_e6513_d_n7, assign6510_e6513_d_n8, assign6510_e6513_d_n9,) = {
    if (locals.var_guard118 != 0.0) {
        let assign6510_e6509: f64 = (locals.var_qbe + locals.var_qbc);
        let assign6510_e6511: f64 = (assign6510_e6509 / locals.var_in_n);
        (assign6510_e6511, ((((locals.var_qbe_dn0 + locals.var_qbc_dn0) * locals.var_in_n) - (assign6510_e6509 * locals.var_in_n_dn0)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn1 + locals.var_qbc_dn1) * locals.var_in_n) - (assign6510_e6509 * locals.var_in_n_dn1)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn3 + locals.var_qbc_dn3) * locals.var_in_n) - (assign6510_e6509 * locals.var_in_n_dn3)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn4 + locals.var_qbc_dn4) * locals.var_in_n) - (assign6510_e6509 * locals.var_in_n_dn4)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn5 + locals.var_qbc_dn5) * locals.var_in_n) - (assign6510_e6509 * locals.var_in_n_dn5)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn6 + locals.var_qbc_dn6) * locals.var_in_n) - (assign6510_e6509 * locals.var_in_n_dn6)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn7 + locals.var_qbc_dn7) * locals.var_in_n) - (assign6510_e6509 * locals.var_in_n_dn7)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn8 + locals.var_qbc_dn8) * locals.var_in_n) - (assign6510_e6509 * locals.var_in_n_dn8)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn9 + locals.var_qbc_dn9) * locals.var_in_n) - (assign6510_e6509 * locals.var_in_n_dn9)) / (locals.var_in_n * locals.var_in_n)),)
    } else {
        (locals.var_taub_n, locals.var_taub_n_dn0, locals.var_taub_n_dn1, locals.var_taub_n_dn3, locals.var_taub_n_dn4, locals.var_taub_n_dn5, locals.var_taub_n_dn6, locals.var_taub_n_dn7, locals.var_taub_n_dn8, locals.var_taub_n_dn9,)
    }
};
        locals.var_taub_n = assign6510_e6513;
        locals.var_taub_n_dn0 = assign6510_e6513_d_n0;
        locals.var_taub_n_dn1 = assign6510_e6513_d_n1;
        locals.var_taub_n_dn3 = assign6510_e6513_d_n3;
        locals.var_taub_n_dn4 = assign6510_e6513_d_n4;
        locals.var_taub_n_dn5 = assign6510_e6513_d_n5;
        locals.var_taub_n_dn6 = assign6510_e6513_d_n6;
        locals.var_taub_n_dn7 = assign6510_e6513_d_n7;
        locals.var_taub_n_dn8 = assign6510_e6513_d_n8;
        locals.var_taub_n_dn9 = assign6510_e6513_d_n9;

        let (assign6520_e6522, assign6520_e6522_d_n0, assign6520_e6522_d_n1, assign6520_e6522_d_n3, assign6520_e6522_d_n4, assign6520_e6522_d_n5, assign6520_e6522_d_n6, assign6520_e6522_d_n7, assign6520_e6522_d_n8, assign6520_e6522_d_n9,) = {
    if (locals.var_guard118 == 0.0) {
        let assign6520_e6518: f64 = (locals.var_taub_t * locals.var_q1q);
        let assign6520_e6520: f64 = (assign6520_e6518 * locals.var_qbi);
        (assign6520_e6520, (((locals.var_taub_t * locals.var_q1q_dn0) * locals.var_qbi) + (assign6520_e6518 * locals.var_qbi_dn0)), (((locals.var_taub_t * locals.var_q1q_dn1) * locals.var_qbi) + (assign6520_e6518 * locals.var_qbi_dn1)), (((locals.var_taub_t * locals.var_q1q_dn3) * locals.var_qbi) + (assign6520_e6518 * locals.var_qbi_dn3)), (((locals.var_taub_t * locals.var_q1q_dn4) * locals.var_qbi) + (assign6520_e6518 * locals.var_qbi_dn4)), (((locals.var_taub_t * locals.var_q1q_dn5) * locals.var_qbi) + (assign6520_e6518 * locals.var_qbi_dn5)), (((locals.var_taub_t * locals.var_q1q_dn6) * locals.var_qbi) + (assign6520_e6518 * locals.var_qbi_dn6)), (((locals.var_taub_t * locals.var_q1q_dn7) * locals.var_qbi) + (assign6520_e6518 * locals.var_qbi_dn7)), (((locals.var_taub_t * locals.var_q1q_dn8) * locals.var_qbi) + (assign6520_e6518 * locals.var_qbi_dn8)), (((locals.var_taub_t * locals.var_q1q_dn9) * locals.var_qbi) + (assign6520_e6518 * locals.var_qbi_dn9)),)
    } else {
        (locals.var_taub_n, locals.var_taub_n_dn0, locals.var_taub_n_dn1, locals.var_taub_n_dn3, locals.var_taub_n_dn4, locals.var_taub_n_dn5, locals.var_taub_n_dn6, locals.var_taub_n_dn7, locals.var_taub_n_dn8, locals.var_taub_n_dn9,)
    }
};
        locals.var_taub_n = assign6520_e6522;
        locals.var_taub_n_dn0 = assign6520_e6522_d_n0;
        locals.var_taub_n_dn1 = assign6520_e6522_d_n1;
        locals.var_taub_n_dn3 = assign6520_e6522_d_n3;
        locals.var_taub_n_dn4 = assign6520_e6522_d_n4;
        locals.var_taub_n_dn5 = assign6520_e6522_d_n5;
        locals.var_taub_n_dn6 = assign6520_e6522_d_n6;
        locals.var_taub_n_dn7 = assign6520_e6522_d_n7;
        locals.var_taub_n_dn8 = assign6520_e6522_d_n8;
        locals.var_taub_n_dn9 = assign6520_e6522_d_n9;

        let assign6530_e6525: f64 = if p.p130 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign6530_e6525;

        let (assign6540_e6531, assign6540_e6531_d_n0, assign6540_e6531_d_n1, assign6540_e6531_d_n3, assign6540_e6531_d_n4, assign6540_e6531_d_n5, assign6540_e6531_d_n6, assign6540_e6531_d_n7, assign6540_e6531_d_n8, assign6540_e6531_d_n9,) = {
    if (locals.var_guard119 != 0.0) {
        let assign6540_e6529: f64 = (p.p93 * locals.var_taub_n);
        (assign6540_e6529, (p.p93 * locals.var_taub_n_dn0), (p.p93 * locals.var_taub_n_dn1), (p.p93 * locals.var_taub_n_dn3), (p.p93 * locals.var_taub_n_dn4), (p.p93 * locals.var_taub_n_dn5), (p.p93 * locals.var_taub_n_dn6), (p.p93 * locals.var_taub_n_dn7), (p.p93 * locals.var_taub_n_dn8), (p.p93 * locals.var_taub_n_dn9),)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9,)
    }
};
        locals.var_taun = assign6540_e6531;
        locals.var_taun_dn0 = assign6540_e6531_d_n0;
        locals.var_taun_dn1 = assign6540_e6531_d_n1;
        locals.var_taun_dn3 = assign6540_e6531_d_n3;
        locals.var_taun_dn4 = assign6540_e6531_d_n4;
        locals.var_taun_dn5 = assign6540_e6531_d_n5;
        locals.var_taun_dn6 = assign6540_e6531_d_n6;
        locals.var_taun_dn7 = assign6540_e6531_d_n7;
        locals.var_taun_dn8 = assign6540_e6531_d_n8;
        locals.var_taun_dn9 = assign6540_e6531_d_n9;

        let assign6550_e6534: f64 = if p.p130 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign6550_e6534;

        let (assign6560_e6543, assign6560_e6543_d_n0, assign6560_e6543_d_n1, assign6560_e6543_d_n3, assign6560_e6543_d_n4, assign6560_e6543_d_n5, assign6560_e6543_d_n6, assign6560_e6543_d_n7, assign6560_e6543_d_n8, assign6560_e6543_d_n9,) = {
    if ((locals.var_guard119 == 0.0) && (locals.var_guard120 != 0.0)) {
        let assign6560_e6541: f64 = (p.p131 * locals.var_taub_n);
        (assign6560_e6541, (p.p131 * locals.var_taub_n_dn0), (p.p131 * locals.var_taub_n_dn1), (p.p131 * locals.var_taub_n_dn3), (p.p131 * locals.var_taub_n_dn4), (p.p131 * locals.var_taub_n_dn5), (p.p131 * locals.var_taub_n_dn6), (p.p131 * locals.var_taub_n_dn7), (p.p131 * locals.var_taub_n_dn8), (p.p131 * locals.var_taub_n_dn9),)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9,)
    }
};
        locals.var_taun = assign6560_e6543;
        locals.var_taun_dn0 = assign6560_e6543_d_n0;
        locals.var_taun_dn1 = assign6560_e6543_d_n1;
        locals.var_taun_dn3 = assign6560_e6543_d_n3;
        locals.var_taun_dn4 = assign6560_e6543_d_n4;
        locals.var_taun_dn5 = assign6560_e6543_d_n5;
        locals.var_taun_dn6 = assign6560_e6543_d_n6;
        locals.var_taun_dn7 = assign6560_e6543_d_n7;
        locals.var_taun_dn8 = assign6560_e6543_d_n8;
        locals.var_taun_dn9 = assign6560_e6543_d_n9;

        let (assign6570_e6551, assign6570_e6551_d_n0, assign6570_e6551_d_n1, assign6570_e6551_d_n3, assign6570_e6551_d_n4, assign6570_e6551_d_n5, assign6570_e6551_d_n6, assign6570_e6551_d_n7, assign6570_e6551_d_n8, assign6570_e6551_d_n9,) = {
    if ((locals.var_guard119 == 0.0) && (locals.var_guard120 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9,)
    }
};
        locals.var_taun = assign6570_e6551;
        locals.var_taun_dn0 = assign6570_e6551_d_n0;
        locals.var_taun_dn1 = assign6570_e6551_d_n1;
        locals.var_taun_dn3 = assign6570_e6551_d_n3;
        locals.var_taun_dn4 = assign6570_e6551_d_n4;
        locals.var_taun_dn5 = assign6570_e6551_d_n5;
        locals.var_taun_dn6 = assign6570_e6551_d_n6;
        locals.var_taun_dn7 = assign6570_e6551_d_n7;
        locals.var_taun_dn8 = assign6570_e6551_d_n8;
        locals.var_taun_dn9 = assign6570_e6551_d_n9;

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign00_e541: f64 = if p.p3 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign00_e541;
        locals.var_guard1_rv = 0.0;

        let (assign10_e545,) = {
    if (locals.var_guard1 != 0.0) {
        (70300000.0,)
    } else {
        (locals.var_an,)
    }
};
        locals.var_an = assign10_e545;
        locals.var_an_rv = 0.0;

        let (assign20_e549,) = {
    if (locals.var_guard1 != 0.0) {
        (123000000.0,)
    } else {
        (locals.var_bn,)
    }
};
        locals.var_bn = assign20_e549;
        locals.var_bn_rv = 0.0;

        let (assign30_e554,) = {
    if (locals.var_guard1 == 0.0) {
        (158000000.0,)
    } else {
        (locals.var_an,)
    }
};
        locals.var_an = assign30_e554;
        locals.var_an_rv = 0.0;

        let (assign40_e559,) = {
    if (locals.var_guard1 == 0.0) {
        (204000000.0,)
    } else {
        (locals.var_bn,)
    }
};
        locals.var_bn = assign40_e559;
        locals.var_bn_rv = 0.0;

        let assign50_e562: f64 = (1.0 - p.p32);
        locals.var_xext1 = assign50_e562;
        locals.var_xext1_rv = 0.0;

        let assign60_e565: f64 = (p.p4 + 273.15);
        locals.var_trk = assign60_e565;
        locals.var_trk_rv = 0.0;

        let assign70_e566: f64 = ctx_temp;
        let assign70_e568: f64 = (assign70_e566 + p.p0);
        locals.var_tamb = assign70_e568;
        locals.var_tamb_rv = 0.0;

        let assign90_e574: f64 = if p.p137 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2 = assign90_e574;
        locals.var_guard2_rv = 0.0;

        let (assign100_e578,) = {
    if (locals.var_guard2 != 0.0) {
        (1e-12,)
    } else {
        (locals.var_minr,)
    }
};
        locals.var_minr = assign100_e578;
        locals.var_minr_rv = 0.0;

        let (assign110_e583,) = {
    if (locals.var_guard2 == 0.0) {
        (p.p137,)
    } else {
        (locals.var_minr,)
    }
};
        locals.var_minr = assign110_e583;
        locals.var_minr_rv = 0.0;

        let assign120_e586: f64 = (locals.var_minr * p.p1);
        locals.var_minr_m = assign120_e586;
        locals.var_minr_m_rv = 0.0;

        locals.var_eps_nf = 0.001;
        locals.var_eps_nf_rv = 0.0;

        locals.var_eps_bavl_t = 0.001;
        locals.var_eps_bavl_t_rv = 0.0;

        let assign160_e595: f64 = (2.0 - p.p66);
        let assign160_e596: f64 = (2.0_f64).powf(assign160_e595);
        locals.var_pow2_2m_pe = assign160_e596;
        locals.var_pow2_2m_pe_rv = 0.0;

        let assign180_e603: f64 = (p.p114 * locals.var_trk);
        let assign180_e605: f64 = (assign180_e603 * locals.var_trk);
        let assign180_e608: f64 = (locals.var_trk + p.p115);
        let assign180_e609: f64 = (assign180_e605 / assign180_e608);
        let assign180_e610: f64 = (p.p113 + assign180_e609);
        let assign180_e612: f64 = (assign180_e610 - 0.05);
        let assign180_e614: f64 = (assign180_e612 / 0.1);
        locals.var_dxa = assign180_e614;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign190_e618: f64 = (p.p114 * locals.var_trk);
        let assign190_e620: f64 = (assign190_e618 * locals.var_trk);
        let assign190_e623: f64 = (locals.var_trk + p.p115);
        let assign190_e624: f64 = (assign190_e620 / assign190_e623);
        let assign190_e625: f64 = (p.p113 + assign190_e624);
        let assign190_e627: f64 = if assign190_e625 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard3 = assign190_e627;
        locals.var_guard3_rv = 0.0;

        let (assign200_e639, assign200_e639_d_n0, assign200_e639_d_n1, assign200_e639_d_n3, assign200_e639_d_n4, assign200_e639_d_n5, assign200_e639_d_n6, assign200_e639_d_n7, assign200_e639_d_n8, assign200_e639_d_n9,) = {
    if (locals.var_guard3 != 0.0) {
        let assign200_e633: f64 = (locals.var_dxa).exp();
        let assign200_e634: f64 = (1.0 + assign200_e633);
        let assign200_e635: f64 = (assign200_e634).ln();
        let assign200_e636: f64 = (0.1 * assign200_e635);
        let assign200_e637: f64 = (0.05 + assign200_e636);
        (assign200_e637, (0.1 * ((assign200_e633 * locals.var_dxa_dn0) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn1) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn3) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn4) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn5) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn6) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn7) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn8) / assign200_e634)), (0.1 * ((assign200_e633 * locals.var_dxa_dn9) / assign200_e634)),)
    } else {
        (locals.var_vgzebok, locals.var_vgzebok_dn0, locals.var_vgzebok_dn1, locals.var_vgzebok_dn3, locals.var_vgzebok_dn4, locals.var_vgzebok_dn5, locals.var_vgzebok_dn6, locals.var_vgzebok_dn7, locals.var_vgzebok_dn8, locals.var_vgzebok_dn9,)
    }
};
        locals.var_vgzebok = assign200_e639;
        locals.var_vgzebok_dn0 = assign200_e639_d_n0;
        locals.var_vgzebok_dn1 = assign200_e639_d_n1;
        locals.var_vgzebok_dn3 = assign200_e639_d_n3;
        locals.var_vgzebok_dn4 = assign200_e639_d_n4;
        locals.var_vgzebok_dn5 = assign200_e639_d_n5;
        locals.var_vgzebok_dn6 = assign200_e639_d_n6;
        locals.var_vgzebok_dn7 = assign200_e639_d_n7;
        locals.var_vgzebok_dn8 = assign200_e639_d_n8;
        locals.var_vgzebok_dn9 = assign200_e639_d_n9;
        locals.var_vgzebok_rv = 0.0;

        let (assign210_e663, assign210_e663_d_n0, assign210_e663_d_n1, assign210_e663_d_n3, assign210_e663_d_n4, assign210_e663_d_n5, assign210_e663_d_n6, assign210_e663_d_n7, assign210_e663_d_n8, assign210_e663_d_n9,) = {
    if (locals.var_guard3 == 0.0) {
        let assign210_e645: f64 = (p.p114 * locals.var_trk);
        let assign210_e647: f64 = (assign210_e645 * locals.var_trk);
        let assign210_e650: f64 = (locals.var_trk + p.p115);
        let assign210_e651: f64 = (assign210_e647 / assign210_e650);
        let assign210_e652: f64 = (p.p113 + assign210_e651);
        let assign210_e656: f64 = (-locals.var_dxa);
        let assign210_e657: f64 = (assign210_e656).exp();
        let assign210_e658: f64 = (1.0 + assign210_e657);
        let assign210_e659: f64 = (assign210_e658).ln();
        let assign210_e660: f64 = (0.1 * assign210_e659);
        let assign210_e661: f64 = (assign210_e652 + assign210_e660);
        (assign210_e661, (0.1 * ((assign210_e657 * (-locals.var_dxa_dn0)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn1)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn3)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn4)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn5)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn6)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn7)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn8)) / assign210_e658)), (0.1 * ((assign210_e657 * (-locals.var_dxa_dn9)) / assign210_e658)),)
    } else {
        (locals.var_vgzebok, locals.var_vgzebok_dn0, locals.var_vgzebok_dn1, locals.var_vgzebok_dn3, locals.var_vgzebok_dn4, locals.var_vgzebok_dn5, locals.var_vgzebok_dn6, locals.var_vgzebok_dn7, locals.var_vgzebok_dn8, locals.var_vgzebok_dn9,)
    }
};
        locals.var_vgzebok = assign210_e663;
        locals.var_vgzebok_dn0 = assign210_e663_d_n0;
        locals.var_vgzebok_dn1 = assign210_e663_d_n1;
        locals.var_vgzebok_dn3 = assign210_e663_d_n3;
        locals.var_vgzebok_dn4 = assign210_e663_d_n4;
        locals.var_vgzebok_dn5 = assign210_e663_d_n5;
        locals.var_vgzebok_dn6 = assign210_e663_d_n6;
        locals.var_vgzebok_dn7 = assign210_e663_d_n7;
        locals.var_vgzebok_dn8 = assign210_e663_d_n8;
        locals.var_vgzebok_dn9 = assign210_e663_d_n9;
        locals.var_vgzebok_rv = 0.0;

        locals.var_vgzeb_tr = p.p113;
        locals.var_vgzeb_tr_rv = 0.0;

        let assign230_e667: f64 = (1.0 / locals.var_vgzeb_tr);
        locals.var_inv_vgzeb_tr = assign230_e667;
        locals.var_inv_vgzeb_tr_rv = 0.0;

        locals.var_vdc_zener = p.p70;
        locals.var_vdc_zener_rv = 0.0;

        locals.var_pc_zener = p.p71;
        locals.var_pc_zener_rv = 0.0;

        let assign270_e676: f64 = (2.0 - locals.var_pc_zener);
        let assign270_e677: f64 = (2.0_f64).powf(assign270_e676);
        locals.var_pow2_2m_pc = assign270_e677;
        locals.var_pow2_2m_pc_rv = 0.0;

        let assign290_e684: f64 = (p.p117 * locals.var_trk);
        let assign290_e686: f64 = (assign290_e684 * locals.var_trk);
        let assign290_e689: f64 = (locals.var_trk + p.p118);
        let assign290_e690: f64 = (assign290_e686 / assign290_e689);
        let assign290_e691: f64 = (p.p116 + assign290_e690);
        let assign290_e693: f64 = (assign290_e691 - 0.05);
        let assign290_e695: f64 = (assign290_e693 / 0.1);
        locals.var_dxa = assign290_e695;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign300_e699: f64 = (p.p117 * locals.var_trk);
        let assign300_e701: f64 = (assign300_e699 * locals.var_trk);
        let assign300_e704: f64 = (locals.var_trk + p.p118);
        let assign300_e705: f64 = (assign300_e701 / assign300_e704);
        let assign300_e706: f64 = (p.p116 + assign300_e705);
        let assign300_e708: f64 = if assign300_e706 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard4 = assign300_e708;
        locals.var_guard4_rv = 0.0;

        let (assign310_e720, assign310_e720_d_n0, assign310_e720_d_n1, assign310_e720_d_n3, assign310_e720_d_n4, assign310_e720_d_n5, assign310_e720_d_n6, assign310_e720_d_n7, assign310_e720_d_n8, assign310_e720_d_n9,) = {
    if (locals.var_guard4 != 0.0) {
        let assign310_e714: f64 = (locals.var_dxa).exp();
        let assign310_e715: f64 = (1.0 + assign310_e714);
        let assign310_e716: f64 = (assign310_e715).ln();
        let assign310_e717: f64 = (0.1 * assign310_e716);
        let assign310_e718: f64 = (0.05 + assign310_e717);
        (assign310_e718, (0.1 * ((assign310_e714 * locals.var_dxa_dn0) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn1) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn3) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn4) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn5) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn6) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn7) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn8) / assign310_e715)), (0.1 * ((assign310_e714 * locals.var_dxa_dn9) / assign310_e715)),)
    } else {
        (locals.var_vgzcbok, locals.var_vgzcbok_dn0, locals.var_vgzcbok_dn1, locals.var_vgzcbok_dn3, locals.var_vgzcbok_dn4, locals.var_vgzcbok_dn5, locals.var_vgzcbok_dn6, locals.var_vgzcbok_dn7, locals.var_vgzcbok_dn8, locals.var_vgzcbok_dn9,)
    }
};
        locals.var_vgzcbok = assign310_e720;
        locals.var_vgzcbok_dn0 = assign310_e720_d_n0;
        locals.var_vgzcbok_dn1 = assign310_e720_d_n1;
        locals.var_vgzcbok_dn3 = assign310_e720_d_n3;
        locals.var_vgzcbok_dn4 = assign310_e720_d_n4;
        locals.var_vgzcbok_dn5 = assign310_e720_d_n5;
        locals.var_vgzcbok_dn6 = assign310_e720_d_n6;
        locals.var_vgzcbok_dn7 = assign310_e720_d_n7;
        locals.var_vgzcbok_dn8 = assign310_e720_d_n8;
        locals.var_vgzcbok_dn9 = assign310_e720_d_n9;
        locals.var_vgzcbok_rv = 0.0;

        let (assign320_e744, assign320_e744_d_n0, assign320_e744_d_n1, assign320_e744_d_n3, assign320_e744_d_n4, assign320_e744_d_n5, assign320_e744_d_n6, assign320_e744_d_n7, assign320_e744_d_n8, assign320_e744_d_n9,) = {
    if (locals.var_guard4 == 0.0) {
        let assign320_e726: f64 = (p.p117 * locals.var_trk);
        let assign320_e728: f64 = (assign320_e726 * locals.var_trk);
        let assign320_e731: f64 = (locals.var_trk + p.p118);
        let assign320_e732: f64 = (assign320_e728 / assign320_e731);
        let assign320_e733: f64 = (p.p116 + assign320_e732);
        let assign320_e737: f64 = (-locals.var_dxa);
        let assign320_e738: f64 = (assign320_e737).exp();
        let assign320_e739: f64 = (1.0 + assign320_e738);
        let assign320_e740: f64 = (assign320_e739).ln();
        let assign320_e741: f64 = (0.1 * assign320_e740);
        let assign320_e742: f64 = (assign320_e733 + assign320_e741);
        (assign320_e742, (0.1 * ((assign320_e738 * (-locals.var_dxa_dn0)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn1)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn3)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn4)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn5)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn6)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn7)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn8)) / assign320_e739)), (0.1 * ((assign320_e738 * (-locals.var_dxa_dn9)) / assign320_e739)),)
    } else {
        (locals.var_vgzcbok, locals.var_vgzcbok_dn0, locals.var_vgzcbok_dn1, locals.var_vgzcbok_dn3, locals.var_vgzcbok_dn4, locals.var_vgzcbok_dn5, locals.var_vgzcbok_dn6, locals.var_vgzcbok_dn7, locals.var_vgzcbok_dn8, locals.var_vgzcbok_dn9,)
    }
};
        locals.var_vgzcbok = assign320_e744;
        locals.var_vgzcbok_dn0 = assign320_e744_d_n0;
        locals.var_vgzcbok_dn1 = assign320_e744_d_n1;
        locals.var_vgzcbok_dn3 = assign320_e744_d_n3;
        locals.var_vgzcbok_dn4 = assign320_e744_d_n4;
        locals.var_vgzcbok_dn5 = assign320_e744_d_n5;
        locals.var_vgzcbok_dn6 = assign320_e744_d_n6;
        locals.var_vgzcbok_dn7 = assign320_e744_d_n7;
        locals.var_vgzcbok_dn8 = assign320_e744_d_n8;
        locals.var_vgzcbok_dn9 = assign320_e744_d_n9;
        locals.var_vgzcbok_rv = 0.0;

        locals.var_vgzcb_tr = p.p116;
        locals.var_vgzcb_tr_rv = 0.0;

        let assign340_e748: f64 = (1.0 / locals.var_vgzcb_tr);
        locals.var_inv_vgzcb_tr = assign340_e748;
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
        locals.var_qb1b2_rv = 0.0;

        locals.var_ibi_t = 0.0;
        locals.var_ibi_t_rv = 0.0;

        locals.var_vdt = 0.0;
        locals.var_vdt_rv = 0.0;

        let assign510_e773: f64 = (locals.var_tamb + locals.var_vdt);
        locals.var_tk = assign510_e773;
        locals.var_tk_rv = 0.0;

        let assign520_e776: f64 = (locals.var_tk / locals.var_trk);
        locals.var_tn = assign520_e776;
        locals.var_tn_rv = 0.0;

        let assign530_e779: f64 = (8.617086918058125e-5 * locals.var_tk);
        locals.var_vt = assign530_e779;
        locals.var_vt_rv = 0.0;

        let assign540_e782: f64 = (8.617086918058125e-5 * locals.var_trk);
        locals.var_vtr = assign540_e782;
        locals.var_vtr_rv = 0.0;

        let assign550_e785: f64 = (1.0 / locals.var_vt);
        locals.var_vtinv = assign550_e785;
        locals.var_vtinv_rv = 0.0;

        let assign560_e788: f64 = (1.0 / locals.var_vtr);
        locals.var_vtrinv = assign560_e788;
        locals.var_vtrinv_rv = 0.0;

        let assign570_e791: f64 = (locals.var_vtinv - locals.var_vtrinv);
        locals.var_vdtinv = assign570_e791;
        locals.var_vdtinv_rv = 0.0;

        let assign580_e794: f64 = (locals.var_tk - locals.var_trk);
        locals.var_dt = assign580_e794;
        locals.var_dt_rv = 0.0;

        let assign590_e796: f64 = (locals.var_tn).ln();
        locals.var_lntn = assign590_e796;
        locals.var_lntn_rv = 0.0;

        let assign600_e800: f64 = (p.p114 * locals.var_tk);
        let assign600_e802: f64 = (assign600_e800 * locals.var_tk);
        let assign600_e805: f64 = (locals.var_tk + p.p115);
        let assign600_e806: f64 = (assign600_e802 / assign600_e805);
        let assign600_e807: f64 = (locals.var_vgzebok - assign600_e806);
        let assign600_e809: f64 = (assign600_e807 - 0.05);
        let assign600_e811: f64 = (assign600_e809 / 0.1);
        locals.var_dxa = assign600_e811;
        locals.var_dxa_dn0 = (locals.var_vgzebok_dn0 / 0.1);
        locals.var_dxa_dn1 = (locals.var_vgzebok_dn1 / 0.1);
        locals.var_dxa_dn3 = (locals.var_vgzebok_dn3 / 0.1);
        locals.var_dxa_dn4 = (locals.var_vgzebok_dn4 / 0.1);
        locals.var_dxa_dn5 = (locals.var_vgzebok_dn5 / 0.1);
        locals.var_dxa_dn6 = (locals.var_vgzebok_dn6 / 0.1);
        locals.var_dxa_dn7 = (locals.var_vgzebok_dn7 / 0.1);
        locals.var_dxa_dn8 = (locals.var_vgzebok_dn8 / 0.1);
        locals.var_dxa_dn9 = (locals.var_vgzebok_dn9 / 0.1);
        locals.var_dxa_rv = 0.0;

        let assign610_e815: f64 = (p.p114 * locals.var_tk);
        let assign610_e817: f64 = (assign610_e815 * locals.var_tk);
        let assign610_e820: f64 = (locals.var_tk + p.p115);
        let assign610_e821: f64 = (assign610_e817 / assign610_e820);
        let assign610_e822: f64 = (locals.var_vgzebok - assign610_e821);
        let assign610_e824: f64 = if assign610_e822 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard5 = assign610_e824;
        locals.var_guard5_rv = 0.0;

        let (assign620_e836, assign620_e836_d_n0, assign620_e836_d_n1, assign620_e836_d_n3, assign620_e836_d_n4, assign620_e836_d_n5, assign620_e836_d_n6, assign620_e836_d_n7, assign620_e836_d_n8, assign620_e836_d_n9,) = {
    if (locals.var_guard5 != 0.0) {
        let assign620_e830: f64 = (locals.var_dxa).exp();
        let assign620_e831: f64 = (1.0 + assign620_e830);
        let assign620_e832: f64 = (assign620_e831).ln();
        let assign620_e833: f64 = (0.1 * assign620_e832);
        let assign620_e834: f64 = (0.05 + assign620_e833);
        (assign620_e834, (0.1 * ((assign620_e830 * locals.var_dxa_dn0) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn1) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn3) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn4) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn5) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn6) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn7) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn8) / assign620_e831)), (0.1 * ((assign620_e830 * locals.var_dxa_dn9) / assign620_e831)),)
    } else {
        (locals.var_vgzeb_t, locals.var_vgzeb_t_dn0, locals.var_vgzeb_t_dn1, locals.var_vgzeb_t_dn3, locals.var_vgzeb_t_dn4, locals.var_vgzeb_t_dn5, locals.var_vgzeb_t_dn6, locals.var_vgzeb_t_dn7, locals.var_vgzeb_t_dn8, locals.var_vgzeb_t_dn9,)
    }
};
        locals.var_vgzeb_t = assign620_e836;
        locals.var_vgzeb_t_dn0 = assign620_e836_d_n0;
        locals.var_vgzeb_t_dn1 = assign620_e836_d_n1;
        locals.var_vgzeb_t_dn3 = assign620_e836_d_n3;
        locals.var_vgzeb_t_dn4 = assign620_e836_d_n4;
        locals.var_vgzeb_t_dn5 = assign620_e836_d_n5;
        locals.var_vgzeb_t_dn6 = assign620_e836_d_n6;
        locals.var_vgzeb_t_dn7 = assign620_e836_d_n7;
        locals.var_vgzeb_t_dn8 = assign620_e836_d_n8;
        locals.var_vgzeb_t_dn9 = assign620_e836_d_n9;
        locals.var_vgzeb_t_rv = 0.0;

        let (assign630_e860, assign630_e860_d_n0, assign630_e860_d_n1, assign630_e860_d_n3, assign630_e860_d_n4, assign630_e860_d_n5, assign630_e860_d_n6, assign630_e860_d_n7, assign630_e860_d_n8, assign630_e860_d_n9,) = {
    if (locals.var_guard5 == 0.0) {
        let assign630_e842: f64 = (p.p114 * locals.var_tk);
        let assign630_e844: f64 = (assign630_e842 * locals.var_tk);
        let assign630_e847: f64 = (locals.var_tk + p.p115);
        let assign630_e848: f64 = (assign630_e844 / assign630_e847);
        let assign630_e849: f64 = (locals.var_vgzebok - assign630_e848);
        let assign630_e853: f64 = (-locals.var_dxa);
        let assign630_e854: f64 = (assign630_e853).exp();
        let assign630_e855: f64 = (1.0 + assign630_e854);
        let assign630_e856: f64 = (assign630_e855).ln();
        let assign630_e857: f64 = (0.1 * assign630_e856);
        let assign630_e858: f64 = (assign630_e849 + assign630_e857);
        (assign630_e858, (locals.var_vgzebok_dn0 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn0)) / assign630_e855))), (locals.var_vgzebok_dn1 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn1)) / assign630_e855))), (locals.var_vgzebok_dn3 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn3)) / assign630_e855))), (locals.var_vgzebok_dn4 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn4)) / assign630_e855))), (locals.var_vgzebok_dn5 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn5)) / assign630_e855))), (locals.var_vgzebok_dn6 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn6)) / assign630_e855))), (locals.var_vgzebok_dn7 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn7)) / assign630_e855))), (locals.var_vgzebok_dn8 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn8)) / assign630_e855))), (locals.var_vgzebok_dn9 + (0.1 * ((assign630_e854 * (-locals.var_dxa_dn9)) / assign630_e855))),)
    } else {
        (locals.var_vgzeb_t, locals.var_vgzeb_t_dn0, locals.var_vgzeb_t_dn1, locals.var_vgzeb_t_dn3, locals.var_vgzeb_t_dn4, locals.var_vgzeb_t_dn5, locals.var_vgzeb_t_dn6, locals.var_vgzeb_t_dn7, locals.var_vgzeb_t_dn8, locals.var_vgzeb_t_dn9,)
    }
};
        locals.var_vgzeb_t = assign630_e860;
        locals.var_vgzeb_t_dn0 = assign630_e860_d_n0;
        locals.var_vgzeb_t_dn1 = assign630_e860_d_n1;
        locals.var_vgzeb_t_dn3 = assign630_e860_d_n3;
        locals.var_vgzeb_t_dn4 = assign630_e860_d_n4;
        locals.var_vgzeb_t_dn5 = assign630_e860_d_n5;
        locals.var_vgzeb_t_dn6 = assign630_e860_d_n6;
        locals.var_vgzeb_t_dn7 = assign630_e860_d_n7;
        locals.var_vgzeb_t_dn8 = assign630_e860_d_n8;
        locals.var_vgzeb_t_dn9 = assign630_e860_d_n9;
        locals.var_vgzeb_t_rv = 0.0;

        let assign640_e864: f64 = (p.p117 * locals.var_tk);
        let assign640_e866: f64 = (assign640_e864 * locals.var_tk);
        let assign640_e869: f64 = (locals.var_tk + p.p118);
        let assign640_e870: f64 = (assign640_e866 / assign640_e869);
        let assign640_e871: f64 = (locals.var_vgzcbok - assign640_e870);
        let assign640_e873: f64 = (assign640_e871 - 0.05);
        let assign640_e875: f64 = (assign640_e873 / 0.1);
        locals.var_dxa = assign640_e875;
        locals.var_dxa_dn0 = (locals.var_vgzcbok_dn0 / 0.1);
        locals.var_dxa_dn1 = (locals.var_vgzcbok_dn1 / 0.1);
        locals.var_dxa_dn3 = (locals.var_vgzcbok_dn3 / 0.1);
        locals.var_dxa_dn4 = (locals.var_vgzcbok_dn4 / 0.1);
        locals.var_dxa_dn5 = (locals.var_vgzcbok_dn5 / 0.1);
        locals.var_dxa_dn6 = (locals.var_vgzcbok_dn6 / 0.1);
        locals.var_dxa_dn7 = (locals.var_vgzcbok_dn7 / 0.1);
        locals.var_dxa_dn8 = (locals.var_vgzcbok_dn8 / 0.1);
        locals.var_dxa_dn9 = (locals.var_vgzcbok_dn9 / 0.1);
        locals.var_dxa_rv = 0.0;

        let assign650_e879: f64 = (p.p117 * locals.var_tk);
        let assign650_e881: f64 = (assign650_e879 * locals.var_tk);
        let assign650_e884: f64 = (locals.var_tk + p.p118);
        let assign650_e885: f64 = (assign650_e881 / assign650_e884);
        let assign650_e886: f64 = (locals.var_vgzcbok - assign650_e885);
        let assign650_e888: f64 = if assign650_e886 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign650_e888;
        locals.var_guard6_rv = 0.0;

        let (assign660_e900, assign660_e900_d_n0, assign660_e900_d_n1, assign660_e900_d_n3, assign660_e900_d_n4, assign660_e900_d_n5, assign660_e900_d_n6, assign660_e900_d_n7, assign660_e900_d_n8, assign660_e900_d_n9,) = {
    if (locals.var_guard6 != 0.0) {
        let assign660_e894: f64 = (locals.var_dxa).exp();
        let assign660_e895: f64 = (1.0 + assign660_e894);
        let assign660_e896: f64 = (assign660_e895).ln();
        let assign660_e897: f64 = (0.1 * assign660_e896);
        let assign660_e898: f64 = (0.05 + assign660_e897);
        (assign660_e898, (0.1 * ((assign660_e894 * locals.var_dxa_dn0) / assign660_e895)), (0.1 * ((assign660_e894 * locals.var_dxa_dn1) / assign660_e895)), (0.1 * ((assign660_e894 * locals.var_dxa_dn3) / assign660_e895)), (0.1 * ((assign660_e894 * locals.var_dxa_dn4) / assign660_e895)), (0.1 * ((assign660_e894 * locals.var_dxa_dn5) / assign660_e895)), (0.1 * ((assign660_e894 * locals.var_dxa_dn6) / assign660_e895)), (0.1 * ((assign660_e894 * locals.var_dxa_dn7) / assign660_e895)), (0.1 * ((assign660_e894 * locals.var_dxa_dn8) / assign660_e895)), (0.1 * ((assign660_e894 * locals.var_dxa_dn9) / assign660_e895)),)
    } else {
        (locals.var_vgzcb_t, locals.var_vgzcb_t_dn0, locals.var_vgzcb_t_dn1, locals.var_vgzcb_t_dn3, locals.var_vgzcb_t_dn4, locals.var_vgzcb_t_dn5, locals.var_vgzcb_t_dn6, locals.var_vgzcb_t_dn7, locals.var_vgzcb_t_dn8, locals.var_vgzcb_t_dn9,)
    }
};
        locals.var_vgzcb_t = assign660_e900;
        locals.var_vgzcb_t_dn0 = assign660_e900_d_n0;
        locals.var_vgzcb_t_dn1 = assign660_e900_d_n1;
        locals.var_vgzcb_t_dn3 = assign660_e900_d_n3;
        locals.var_vgzcb_t_dn4 = assign660_e900_d_n4;
        locals.var_vgzcb_t_dn5 = assign660_e900_d_n5;
        locals.var_vgzcb_t_dn6 = assign660_e900_d_n6;
        locals.var_vgzcb_t_dn7 = assign660_e900_d_n7;
        locals.var_vgzcb_t_dn8 = assign660_e900_d_n8;
        locals.var_vgzcb_t_dn9 = assign660_e900_d_n9;
        locals.var_vgzcb_t_rv = 0.0;

    }
}
