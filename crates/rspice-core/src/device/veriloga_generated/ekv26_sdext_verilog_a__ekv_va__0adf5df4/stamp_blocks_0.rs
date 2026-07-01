#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let assign10_e194: f64 = (11.7 * 8.8541879239442e-12);
        locals.var_epssil = assign10_e194;

        locals.var_theta_vp_1 = 0.0;
        locals.var_theta_vp_1_dn0 = 0.0;
        locals.var_theta_vp_1_dn1 = 0.0;
        locals.var_theta_vp_1_dn2 = 0.0;
        locals.var_theta_vp_1_dn3 = 0.0;

        locals.var_vpprime = 0.0;
        locals.var_vpprime_dn0 = 0.0;
        locals.var_vpprime_dn1 = 0.0;
        locals.var_vpprime_dn2 = 0.0;
        locals.var_vpprime_dn3 = 0.0;

        locals.var_sqrt_vp_vt = 0.0;
        locals.var_sqrt_vp_vt_dn0 = 0.0;
        locals.var_sqrt_vp_vt_dn1 = 0.0;
        locals.var_sqrt_vp_vt_dn2 = 0.0;
        locals.var_sqrt_vp_vt_dn3 = 0.0;

        let assign60_e201: f64 = (locals.var_epssil / p.p13);
        locals.var_eps_cox = assign60_e201;

        let assign70_e204: f64 = (locals.var_eps_cox * p.p14);
        let assign70_e205: f64 = (assign70_e204).sqrt();
        locals.var_lc = assign70_e205;

        let assign80_e208: f64 = (locals.var_lc * p.p25);
        locals.var_lc_lambda = assign80_e208;

        let assign90_e211: f64 = (3.0 * locals.var_eps_cox);
        let assign90_e213: f64 = (assign90_e211 * p.p28);
        locals.var_eps_cox_w = assign90_e213;

        let assign100_e216: f64 = (locals.var_eps_cox * p.p29);
        locals.var_eps_cox_l = assign100_e216;

        let assign120_e223: f64 = (locals.var_epssil * p.p22);
        let assign120_e224: f64 = (p.p13 / assign120_e223);
        locals.var_t0 = assign120_e224;

        let assign130_e227: f64 = (p.p30 + p.p30);
        let assign130_e229: f64 = (assign130_e227 / p.p13);
        locals.var_v0 = assign130_e229;

        let (assign140_e235,) = {
    if (p.p0 > 0.0) {
        (0.5,)
    } else {
        (0.3333333333333,)
    }
};
        locals.var_eta_qi = assign140_e235;

        let assign150_e238: f64 = (-1e21);
        let assign150_e239: f64 = (-assign150_e238);
        let assign150_e240: f64 = if p.p3 == assign150_e239 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign150_e240;

        let (assign160_e246,) = {
    if (locals.var_guard1 != 0.0) {
        let assign160_e242: f64 = ctx_temp;
        let assign160_e244: f64 = (assign160_e242 + p.p2);
        (assign160_e244,)
    } else {
        (locals.var_t,)
    }
};
        locals.var_t = assign160_e246;

        let (assign170_e253,) = {
    if (locals.var_guard1 == 0.0) {
        let assign170_e251: f64 = (p.p3 + 273.15);
        (assign170_e251,)
    } else {
        (locals.var_t,)
    }
};
        locals.var_t = assign170_e253;

        let assign180_e256: f64 = (-1e21);
        let assign180_e257: f64 = (-assign180_e256);
        let assign180_e258: f64 = if p.p4 == assign180_e257 { 1.0 } else { 0.0 };
        locals.var_guard2 = assign180_e258;

        let (assign190_e264,) = {
    if (locals.var_guard2 != 0.0) {
        let assign190_e262: f64 = (25.0 + 273.15);
        (assign190_e262,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign190_e264;

        let (assign200_e271,) = {
    if (locals.var_guard2 == 0.0) {
        let assign200_e269: f64 = (p.p4 + 273.15);
        (assign200_e269,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign200_e271;

        let assign210_e273: f64 = (locals.var_t * THERMAL_VOLTAGE_PER_K);
        locals.var_vt = assign210_e273;

        let assign220_e276: f64 = (0.1 * locals.var_vt);
        locals.var_vt_01 = assign220_e276;

        let assign230_e279: f64 = (1.0 / locals.var_vt);
        locals.var_inv_vt = assign230_e279;

        let assign240_e282: f64 = (locals.var_vt + locals.var_vt);
        locals.var_vt_2 = assign240_e282;

        let assign250_e285: f64 = (locals.var_vt_2 + locals.var_vt_2);
        locals.var_vt_4 = assign250_e285;

        let assign260_e288: f64 = (locals.var_vt * locals.var_vt);
        locals.var_vt_vt = assign260_e288;

        let assign270_e291: f64 = (locals.var_vt_vt + locals.var_vt_vt);
        locals.var_vt_vt_2 = assign270_e291;

        let assign280_e294: f64 = (16.0 * locals.var_vt_vt);
        locals.var_vt_vt_16 = assign280_e294;

        let assign290_e298: f64 = (0.000702 * locals.var_t);
        let assign290_e300: f64 = (assign290_e298 * locals.var_t);
        let assign290_e303: f64 = (locals.var_t + 1108.0);
        let assign290_e304: f64 = (assign290_e300 / assign290_e303);
        let assign290_e305: f64 = (1.16 - assign290_e304);
        locals.var_eg = assign290_e305;

        let assign300_e309: f64 = (0.000702 * locals.var_tnom);
        let assign300_e311: f64 = (assign300_e309 * locals.var_tnom);
        let assign300_e314: f64 = (locals.var_tnom + 1108.0);
        let assign300_e315: f64 = (assign300_e311 / assign300_e314);
        let assign300_e316: f64 = (1.16 - assign300_e315);
        locals.var_refeg = assign300_e316;

        let assign310_e319: f64 = (locals.var_t - locals.var_tnom);
        locals.var_deltat = assign310_e319;

        let assign320_e322: f64 = (locals.var_t / locals.var_tnom);
        locals.var_ratiot = assign320_e322;

        let assign330_e326: f64 = (p.p16 * locals.var_deltat);
        let assign330_e327: f64 = (p.p15 - assign330_e326);
        locals.var_vto_t = assign330_e327;

        let assign340_e331: f64 = (locals.var_ratiot).powf(p.p20);
        let assign340_e332: f64 = (p.p19 * assign340_e331);
        locals.var_kp_t = assign340_e332;

        let assign350_e336: f64 = (locals.var_ratiot).powf(p.p24);
        let assign350_e337: f64 = (p.p23 * assign350_e336);
        locals.var_ucrit_t = assign350_e337;

        let assign370_e347: f64 = (p.p18 * locals.var_ratiot);
        let assign370_e350: f64 = (3.0 * locals.var_vt);
        let assign370_e352: f64 = (locals.var_ratiot).ln();
        let assign370_e353: f64 = (assign370_e350 * assign370_e352);
        let assign370_e354: f64 = (assign370_e347 - assign370_e353);
        let assign370_e357: f64 = (locals.var_refeg * locals.var_ratiot);
        let assign370_e358: f64 = (assign370_e354 - assign370_e357);
        let assign370_e360: f64 = (assign370_e358 + locals.var_eg);
        locals.var_phi_t = assign370_e360;
        locals.var_phi_t_dn0 = 0.0;
        locals.var_phi_t_dn1 = 0.0;
        locals.var_phi_t_dn2 = 0.0;
        locals.var_phi_t_dn3 = 0.0;

        locals.var_tmp1 = 0.2;
        locals.var_tmp1_dn0 = 0.0;
        locals.var_tmp1_dn1 = 0.0;
        locals.var_tmp1_dn2 = 0.0;
        locals.var_tmp1_dn3 = 0.0;

        let assign390_e364: f64 = (locals.var_phi_t - locals.var_tmp1);
        locals.var_tmp2 = assign390_e364;
        locals.var_tmp2_dn0 = (locals.var_phi_t_dn0 - locals.var_tmp1_dn0);
        locals.var_tmp2_dn1 = (locals.var_phi_t_dn1 - locals.var_tmp1_dn1);
        locals.var_tmp2_dn2 = (locals.var_phi_t_dn2 - locals.var_tmp1_dn2);
        locals.var_tmp2_dn3 = (locals.var_phi_t_dn3 - locals.var_tmp1_dn3);

        let assign400_e369: f64 = (locals.var_tmp2 * locals.var_tmp2);
        let assign400_e372: f64 = (locals.var_vt * locals.var_vt);
        let assign400_e373: f64 = (assign400_e369 + assign400_e372);
        let assign400_e374: f64 = (assign400_e373).sqrt();
        let assign400_e375: f64 = (locals.var_tmp2 + assign400_e374);
        let assign400_e376: f64 = (0.5 * assign400_e375);
        let assign400_e378: f64 = (assign400_e376 + locals.var_tmp1);
        locals.var_phi_t = assign400_e378;
        locals.var_phi_t_dn0 = ((0.5 * (locals.var_tmp2_dn0 + (((locals.var_tmp2_dn0 * locals.var_tmp2) + (locals.var_tmp2 * locals.var_tmp2_dn0)) / (2.0 * assign400_e374)))) + locals.var_tmp1_dn0);
        locals.var_phi_t_dn1 = ((0.5 * (locals.var_tmp2_dn1 + (((locals.var_tmp2_dn1 * locals.var_tmp2) + (locals.var_tmp2 * locals.var_tmp2_dn1)) / (2.0 * assign400_e374)))) + locals.var_tmp1_dn1);
        locals.var_phi_t_dn2 = ((0.5 * (locals.var_tmp2_dn2 + (((locals.var_tmp2_dn2 * locals.var_tmp2) + (locals.var_tmp2 * locals.var_tmp2_dn2)) / (2.0 * assign400_e374)))) + locals.var_tmp1_dn2);
        locals.var_phi_t_dn3 = ((0.5 * (locals.var_tmp2_dn3 + (((locals.var_tmp2_dn3 * locals.var_tmp2) + (locals.var_tmp2 * locals.var_tmp2_dn3)) / (2.0 * assign400_e374)))) + locals.var_tmp1_dn3);

        let assign410_e380: f64 = (locals.var_phi_t).sqrt();
        locals.var_sqrt_phi = assign410_e380;
        locals.var_sqrt_phi_dn0 = (locals.var_phi_t_dn0 / (2.0 * assign410_e380));
        locals.var_sqrt_phi_dn1 = (locals.var_phi_t_dn1 / (2.0 * assign410_e380));
        locals.var_sqrt_phi_dn2 = (locals.var_phi_t_dn2 / (2.0 * assign410_e380));
        locals.var_sqrt_phi_dn3 = (locals.var_phi_t_dn3 / (2.0 * assign410_e380));

        let assign420_e383: f64 = (1.0 / locals.var_ucrit_t);
        locals.var_inv_ucrit = assign420_e383;

        let assign430_e386: f64 = (locals.var_lc * locals.var_ucrit_t);
        locals.var_lc_ucrit = assign430_e386;

        let assign460_e395: f64 = (p.p5 + p.p26);
        locals.var_leff = assign460_e395;

        let assign470_e398: f64 = (p.p6 + p.p27);
        locals.var_weff = assign470_e398;

        let assign480_e401: f64 = (locals.var_ucrit_t * locals.var_leff);
        locals.var_vc = assign480_e401;

        let assign490_e405: f64 = (0.5 * locals.var_vc);
        let assign490_e407: f64 = (assign490_e405 * locals.var_inv_vt);
        let assign490_e408: f64 = (assign490_e407).ln();
        let assign490_e410: f64 = (assign490_e408 - 0.6);
        let assign490_e411: f64 = (locals.var_vt * assign490_e410);
        locals.var_log_vc_vt = assign490_e411;

        let assign500_e415: f64 = (locals.var_weff * locals.var_leff);
        let assign500_e416: f64 = (assign500_e415).sqrt();
        let assign500_e417: f64 = (1.0 / assign500_e416);
        locals.var_awl = assign500_e417;

        let assign510_e420: f64 = if p.p0 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard3 = assign510_e420;

        let (assign520_e435,) = {
    if (locals.var_guard3 != 0.0) {
        let (assign520_e433,) = {
            if (p.p38 != 1e-6) {
                let assign520_e428: f64 = (p.p38 - 1e-6);
                let assign520_e429: f64 = (locals.var_awl * assign520_e428);
                let assign520_e431: f64 = (assign520_e429 + locals.var_vto_t);
                (assign520_e431,)
            } else {
                (locals.var_vto_t,)
            }
        };
        (assign520_e433,)
    } else {
        (locals.var_vto_s,)
    }
};
        locals.var_vto_s = assign520_e435;

        let (assign530_e452,) = {
    if (locals.var_guard3 == 0.0) {
        let (assign530_e450,) = {
            if (p.p38 != 1e-6) {
                let assign530_e444: f64 = (1e-6 - p.p38);
                let assign530_e445: f64 = (locals.var_awl * assign530_e444);
                let assign530_e447: f64 = (assign530_e445 - locals.var_vto_t);
                (assign530_e447,)
            } else {
                let assign530_e449: f64 = (-locals.var_vto_t);
                (assign530_e449,)
            }
        };
        (assign530_e450,)
    } else {
        (locals.var_vto_s,)
    }
};
        locals.var_vto_s = assign530_e452;

        let (assign540_e467,) = {
    if (p.p39 != 1e-6) {
        let assign540_e461: f64 = (p.p39 - 1e-6);
        let assign540_e463: f64 = (assign540_e461 * locals.var_awl);
        let assign540_e464: f64 = (1.0 + assign540_e463);
        let assign540_e465: f64 = (locals.var_kp_t * assign540_e464);
        (assign540_e465,)
    } else {
        (locals.var_kp_t,)
    }
};
        let assign540_e468: f64 = (locals.var_weff * assign540_e467);
        locals.var_kp_weff = assign540_e468;

        let (assign550_e480,) = {
    if (p.p40 != 1e-6) {
        let assign550_e475: f64 = (p.p40 - 1e-6);
        let assign550_e477: f64 = (assign550_e475 * locals.var_awl);
        let assign550_e478: f64 = (p.p17 + assign550_e477);
        (assign550_e478,)
    } else {
        (p.p17,)
    }
};
        locals.var_gamma_s = assign550_e480;

        let assign560_e483: f64 = (locals.var_gamma_s * locals.var_sqrt_phi);
        locals.var_gamma_sqrt_phi = assign560_e483;
        locals.var_gamma_sqrt_phi_dn0 = (locals.var_gamma_s * locals.var_sqrt_phi_dn0);
        locals.var_gamma_sqrt_phi_dn1 = (locals.var_gamma_s * locals.var_sqrt_phi_dn1);
        locals.var_gamma_sqrt_phi_dn2 = (locals.var_gamma_s * locals.var_sqrt_phi_dn2);
        locals.var_gamma_sqrt_phi_dn3 = (locals.var_gamma_s * locals.var_sqrt_phi_dn3);

        let assign570_e486: f64 = if locals.var_v0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard4 = assign570_e486;

        let (assign580_e490,) = {
    if (locals.var_guard4 != 0.0) {
        (0.0,)
    } else {
        (locals.var_deltavfb,)
    }
};
        locals.var_deltavfb = assign580_e490;

        let (assign590_e503,) = {
    if (locals.var_guard4 == 0.0) {
        let assign590_e497: f64 = (p.p31 * p.p8);
        let assign590_e498: f64 = (locals.var_leff / assign590_e497);
        let assign590_e500: f64 = (assign590_e498 - 0.1);
        let assign590_e501: f64 = (0.28 * assign590_e500);
        (assign590_e501,)
    } else {
        (locals.var_vl,)
    }
};
        locals.var_vl = assign590_e503;

        let (assign600_e521,) = {
    if (locals.var_guard4 == 0.0) {
        let assign600_e512: f64 = (locals.var_vl * locals.var_vl);
        let assign600_e514: f64 = (assign600_e512 + 0.001936);
        let assign600_e515: f64 = (assign600_e514).sqrt();
        let assign600_e516: f64 = (locals.var_vl + assign600_e515);
        let assign600_e517: f64 = (0.5 * assign600_e516);
        let assign600_e518: f64 = (1.0 + assign600_e517);
        let assign600_e519: f64 = (1.0 / assign600_e518);
        (assign600_e519,)
    } else {
        (locals.var_sqv,)
    }
};
        locals.var_sqv = assign600_e521;

        let (assign610_e530,) = {
    if (locals.var_guard4 == 0.0) {
        let assign610_e526: f64 = (locals.var_v0 * locals.var_sqv);
        let assign610_e528: f64 = (assign610_e526 * locals.var_sqv);
        (assign610_e528,)
    } else {
        (locals.var_deltavfb,)
    }
};
        locals.var_deltavfb = assign610_e530;

        let assign620_e533: f64 = (p.p0 * (nv1 - nv3));
        locals.var_vg = assign620_e533;
        locals.var_vg_dn1 = p.p0;
        locals.var_vg_dn3 = (-p.p0);

        let assign630_e536: f64 = (p.p0 * (nv2 - nv3));
        locals.var_vs = assign630_e536;
        locals.var_vs_dn0 = 0.0;
        locals.var_vs_dn2 = p.p0;
        locals.var_vs_dn3 = (-p.p0);

        let assign640_e539: f64 = (p.p0 * (nv0 - nv3));
        locals.var_vd = assign640_e539;
        locals.var_vd_dn0 = p.p0;
        locals.var_vd_dn2 = 0.0;
        locals.var_vd_dn3 = (-p.p0);

        let assign650_e542: f64 = (locals.var_vd - locals.var_vs);
        let assign650_e544: f64 = if assign650_e542 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign650_e544;

        let (assign660_e549,) = {
    if (locals.var_guard6 != 0.0) {
        let assign660_e547: f64 = (-1.0);
        (assign660_e547,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign660_e549;

        let (assign670_e553, assign670_e553_d_n0, assign670_e553_d_n2, assign670_e553_d_n3,) = {
    if (locals.var_guard6 != 0.0) {
        (locals.var_vs, locals.var_vs_dn0, locals.var_vs_dn2, locals.var_vs_dn3,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3,)
    }
};
        locals.var_t1 = assign670_e553;
        locals.var_t1_dn0 = assign670_e553_d_n0;
        locals.var_t1_dn2 = assign670_e553_d_n2;
        locals.var_t1_dn3 = assign670_e553_d_n3;

        let (assign680_e557, assign680_e557_d_n0, assign680_e557_d_n2, assign680_e557_d_n3,) = {
    if (locals.var_guard6 != 0.0) {
        (locals.var_vd, locals.var_vd_dn0, locals.var_vd_dn2, locals.var_vd_dn3,)
    } else {
        (locals.var_vs, locals.var_vs_dn0, locals.var_vs_dn2, locals.var_vs_dn3,)
    }
};
        locals.var_vs = assign680_e557;
        locals.var_vs_dn0 = assign680_e557_d_n0;
        locals.var_vs_dn2 = assign680_e557_d_n2;
        locals.var_vs_dn3 = assign680_e557_d_n3;

        let (assign690_e561, assign690_e561_d_n0, assign690_e561_d_n2, assign690_e561_d_n3,) = {
    if (locals.var_guard6 != 0.0) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3,)
    } else {
        (locals.var_vd, locals.var_vd_dn0, locals.var_vd_dn2, locals.var_vd_dn3,)
    }
};
        locals.var_vd = assign690_e561;
        locals.var_vd_dn0 = assign690_e561_d_n0;
        locals.var_vd_dn2 = assign690_e561_d_n2;
        locals.var_vd_dn3 = assign690_e561_d_n3;

        let (assign700_e566,) = {
    if (locals.var_guard6 == 0.0) {
        (1.0,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign700_e566;

        let assign710_e569: f64 = (locals.var_vg - locals.var_vto_s);
        let assign710_e571: f64 = (assign710_e569 - locals.var_deltavfb);
        let assign710_e573: f64 = (assign710_e571 + locals.var_phi_t);
        let assign710_e575: f64 = (assign710_e573 + locals.var_gamma_sqrt_phi);
        locals.var_vgstar = assign710_e575;
        locals.var_vgstar_dn0 = (locals.var_phi_t_dn0 + locals.var_gamma_sqrt_phi_dn0);
        locals.var_vgstar_dn1 = ((locals.var_vg_dn1 + locals.var_phi_t_dn1) + locals.var_gamma_sqrt_phi_dn1);
        locals.var_vgstar_dn2 = (locals.var_phi_t_dn2 + locals.var_gamma_sqrt_phi_dn2);
        locals.var_vgstar_dn3 = ((locals.var_vg_dn3 + locals.var_phi_t_dn3) + locals.var_gamma_sqrt_phi_dn3);

        let assign720_e578: f64 = (locals.var_vgstar * locals.var_vgstar);
        let assign720_e581: f64 = (2.0 * locals.var_vt_vt_16);
        let assign720_e582: f64 = (assign720_e578 + assign720_e581);
        let assign720_e583: f64 = (assign720_e582).sqrt();
        locals.var_sqrt_vgstar = assign720_e583;
        locals.var_sqrt_vgstar_dn0 = (((locals.var_vgstar_dn0 * locals.var_vgstar) + (locals.var_vgstar * locals.var_vgstar_dn0)) / (2.0 * assign720_e583));
        locals.var_sqrt_vgstar_dn1 = (((locals.var_vgstar_dn1 * locals.var_vgstar) + (locals.var_vgstar * locals.var_vgstar_dn1)) / (2.0 * assign720_e583));
        locals.var_sqrt_vgstar_dn2 = (((locals.var_vgstar_dn2 * locals.var_vgstar) + (locals.var_vgstar * locals.var_vgstar_dn2)) / (2.0 * assign720_e583));
        locals.var_sqrt_vgstar_dn3 = (((locals.var_vgstar_dn3 * locals.var_vgstar) + (locals.var_vgstar * locals.var_vgstar_dn3)) / (2.0 * assign720_e583));

        let assign730_e587: f64 = (locals.var_vgstar + locals.var_sqrt_vgstar);
        let assign730_e588: f64 = (0.5 * assign730_e587);
        locals.var_vgprime = assign730_e588;
        locals.var_vgprime_dn0 = (0.5 * (locals.var_vgstar_dn0 + locals.var_sqrt_vgstar_dn0));
        locals.var_vgprime_dn1 = (0.5 * (locals.var_vgstar_dn1 + locals.var_sqrt_vgstar_dn1));
        locals.var_vgprime_dn2 = (0.5 * (locals.var_vgstar_dn2 + locals.var_sqrt_vgstar_dn2));
        locals.var_vgprime_dn3 = (0.5 * (locals.var_vgstar_dn3 + locals.var_sqrt_vgstar_dn3));

        let assign740_e591: f64 = (locals.var_phi_t + locals.var_vs);
        locals.var_phi_vs = assign740_e591;
        locals.var_phi_vs_dn0 = (locals.var_phi_t_dn0 + locals.var_vs_dn0);
        locals.var_phi_vs_dn1 = locals.var_phi_t_dn1;
        locals.var_phi_vs_dn2 = (locals.var_phi_t_dn2 + locals.var_vs_dn2);
        locals.var_phi_vs_dn3 = (locals.var_phi_t_dn3 + locals.var_vs_dn3);

        let assign750_e594: f64 = (locals.var_phi_vs * locals.var_phi_vs);
        let assign750_e596: f64 = (assign750_e594 + locals.var_vt_vt_16);
        let assign750_e597: f64 = (assign750_e596).sqrt();
        locals.var_sqrt_phi_vs_vt = assign750_e597;
        locals.var_sqrt_phi_vs_vt_dn0 = (((locals.var_phi_vs_dn0 * locals.var_phi_vs) + (locals.var_phi_vs * locals.var_phi_vs_dn0)) / (2.0 * assign750_e597));
        locals.var_sqrt_phi_vs_vt_dn1 = (((locals.var_phi_vs_dn1 * locals.var_phi_vs) + (locals.var_phi_vs * locals.var_phi_vs_dn1)) / (2.0 * assign750_e597));
        locals.var_sqrt_phi_vs_vt_dn2 = (((locals.var_phi_vs_dn2 * locals.var_phi_vs) + (locals.var_phi_vs * locals.var_phi_vs_dn2)) / (2.0 * assign750_e597));
        locals.var_sqrt_phi_vs_vt_dn3 = (((locals.var_phi_vs_dn3 * locals.var_phi_vs) + (locals.var_phi_vs * locals.var_phi_vs_dn3)) / (2.0 * assign750_e597));

        let assign760_e601: f64 = (locals.var_phi_vs + locals.var_sqrt_phi_vs_vt);
        let assign760_e602: f64 = (0.5 * assign760_e601);
        let assign760_e603: f64 = (assign760_e602).sqrt();
        locals.var_sqrt_phi_vs = assign760_e603;
        locals.var_sqrt_phi_vs_dn0 = ((0.5 * (locals.var_phi_vs_dn0 + locals.var_sqrt_phi_vs_vt_dn0)) / (2.0 * assign760_e603));
        locals.var_sqrt_phi_vs_dn1 = ((0.5 * (locals.var_phi_vs_dn1 + locals.var_sqrt_phi_vs_vt_dn1)) / (2.0 * assign760_e603));
        locals.var_sqrt_phi_vs_dn2 = ((0.5 * (locals.var_phi_vs_dn2 + locals.var_sqrt_phi_vs_vt_dn2)) / (2.0 * assign760_e603));
        locals.var_sqrt_phi_vs_dn3 = ((0.5 * (locals.var_phi_vs_dn3 + locals.var_sqrt_phi_vs_vt_dn3)) / (2.0 * assign760_e603));

        let assign770_e606: f64 = (locals.var_phi_t + locals.var_vd);
        locals.var_phi_vd = assign770_e606;
        locals.var_phi_vd_dn0 = (locals.var_phi_t_dn0 + locals.var_vd_dn0);
        locals.var_phi_vd_dn1 = locals.var_phi_t_dn1;
        locals.var_phi_vd_dn2 = (locals.var_phi_t_dn2 + locals.var_vd_dn2);
        locals.var_phi_vd_dn3 = (locals.var_phi_t_dn3 + locals.var_vd_dn3);

        let assign780_e609: f64 = (locals.var_phi_vd * locals.var_phi_vd);
        let assign780_e611: f64 = (assign780_e609 + locals.var_vt_vt_16);
        let assign780_e612: f64 = (assign780_e611).sqrt();
        locals.var_sqrt_phi_vd_vt = assign780_e612;
        locals.var_sqrt_phi_vd_vt_dn0 = (((locals.var_phi_vd_dn0 * locals.var_phi_vd) + (locals.var_phi_vd * locals.var_phi_vd_dn0)) / (2.0 * assign780_e612));
        locals.var_sqrt_phi_vd_vt_dn1 = (((locals.var_phi_vd_dn1 * locals.var_phi_vd) + (locals.var_phi_vd * locals.var_phi_vd_dn1)) / (2.0 * assign780_e612));
        locals.var_sqrt_phi_vd_vt_dn2 = (((locals.var_phi_vd_dn2 * locals.var_phi_vd) + (locals.var_phi_vd * locals.var_phi_vd_dn2)) / (2.0 * assign780_e612));
        locals.var_sqrt_phi_vd_vt_dn3 = (((locals.var_phi_vd_dn3 * locals.var_phi_vd) + (locals.var_phi_vd * locals.var_phi_vd_dn3)) / (2.0 * assign780_e612));

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign790_e616: f64 = (locals.var_phi_vd + locals.var_sqrt_phi_vd_vt);
        let assign790_e617: f64 = (0.5 * assign790_e616);
        let assign790_e618: f64 = (assign790_e617).sqrt();
        locals.var_sqrt_phi_vd = assign790_e618;
        locals.var_sqrt_phi_vd_dn0 = ((0.5 * (locals.var_phi_vd_dn0 + locals.var_sqrt_phi_vd_vt_dn0)) / (2.0 * assign790_e618));
        locals.var_sqrt_phi_vd_dn1 = ((0.5 * (locals.var_phi_vd_dn1 + locals.var_sqrt_phi_vd_vt_dn1)) / (2.0 * assign790_e618));
        locals.var_sqrt_phi_vd_dn2 = ((0.5 * (locals.var_phi_vd_dn2 + locals.var_sqrt_phi_vd_vt_dn2)) / (2.0 * assign790_e618));
        locals.var_sqrt_phi_vd_dn3 = ((0.5 * (locals.var_phi_vd_dn3 + locals.var_sqrt_phi_vd_vt_dn3)) / (2.0 * assign790_e618));

        let assign800_e621: f64 = (locals.var_eps_cox_w * p.p7);
        let assign800_e623: f64 = (assign800_e621 / locals.var_weff);
        locals.var_weta_w = assign800_e623;

        let assign810_e626: f64 = (locals.var_eps_cox_l * p.p8);
        let assign810_e628: f64 = (assign810_e626 / locals.var_leff);
        locals.var_leta_l = assign810_e628;

        let assign820_e632: f64 = (0.25 * locals.var_gamma_s);
        let assign820_e634: f64 = (assign820_e632 * locals.var_gamma_s);
        let assign820_e635: f64 = (locals.var_vgprime + assign820_e634);
        let assign820_e636: f64 = (assign820_e635).sqrt();
        locals.var_big_sqrt_vp0 = assign820_e636;
        locals.var_big_sqrt_vp0_dn0 = (locals.var_vgprime_dn0 / (2.0 * assign820_e636));
        locals.var_big_sqrt_vp0_dn1 = (locals.var_vgprime_dn1 / (2.0 * assign820_e636));
        locals.var_big_sqrt_vp0_dn2 = (locals.var_vgprime_dn2 / (2.0 * assign820_e636));
        locals.var_big_sqrt_vp0_dn3 = (locals.var_vgprime_dn3 / (2.0 * assign820_e636));

        let assign830_e639: f64 = (locals.var_vgprime - locals.var_phi_t);
        let assign830_e644: f64 = (0.5 * locals.var_gamma_s);
        let assign830_e645: f64 = (locals.var_big_sqrt_vp0 - assign830_e644);
        let assign830_e646: f64 = (locals.var_gamma_s * assign830_e645);
        let assign830_e647: f64 = (assign830_e639 - assign830_e646);
        locals.var_vp0 = assign830_e647;
        locals.var_vp0_dn0 = ((locals.var_vgprime_dn0 - locals.var_phi_t_dn0) - (locals.var_gamma_s * locals.var_big_sqrt_vp0_dn0));
        locals.var_vp0_dn1 = ((locals.var_vgprime_dn1 - locals.var_phi_t_dn1) - (locals.var_gamma_s * locals.var_big_sqrt_vp0_dn1));
        locals.var_vp0_dn2 = ((locals.var_vgprime_dn2 - locals.var_phi_t_dn2) - (locals.var_gamma_s * locals.var_big_sqrt_vp0_dn2));
        locals.var_vp0_dn3 = ((locals.var_vgprime_dn3 - locals.var_phi_t_dn3) - (locals.var_gamma_s * locals.var_big_sqrt_vp0_dn3));

        let assign840_e650: f64 = (locals.var_vp0 + locals.var_phi_t);
        let assign840_e652: f64 = (assign840_e650 + locals.var_vt_01);
        let assign840_e653: f64 = (assign840_e652).sqrt();
        locals.var_sqrt_phi_vp0 = assign840_e653;
        locals.var_sqrt_phi_vp0_dn0 = ((locals.var_vp0_dn0 + locals.var_phi_t_dn0) / (2.0 * assign840_e653));
        locals.var_sqrt_phi_vp0_dn1 = ((locals.var_vp0_dn1 + locals.var_phi_t_dn1) / (2.0 * assign840_e653));
        locals.var_sqrt_phi_vp0_dn2 = ((locals.var_vp0_dn2 + locals.var_phi_t_dn2) / (2.0 * assign840_e653));
        locals.var_sqrt_phi_vp0_dn3 = ((locals.var_vp0_dn3 + locals.var_phi_t_dn3) / (2.0 * assign840_e653));

        let assign850_e658: f64 = (locals.var_sqrt_phi_vs + locals.var_sqrt_phi_vd);
        let assign850_e659: f64 = (locals.var_leta_l * assign850_e658);
        let assign850_e660: f64 = (locals.var_gamma_s - assign850_e659);
        let assign850_e663: f64 = (locals.var_weta_w * locals.var_sqrt_phi_vp0);
        let assign850_e664: f64 = (assign850_e660 + assign850_e663);
        locals.var_gammastar = assign850_e664;
        locals.var_gammastar_dn0 = ((-(locals.var_leta_l * (locals.var_sqrt_phi_vs_dn0 + locals.var_sqrt_phi_vd_dn0))) + (locals.var_weta_w * locals.var_sqrt_phi_vp0_dn0));
        locals.var_gammastar_dn1 = ((-(locals.var_leta_l * (locals.var_sqrt_phi_vs_dn1 + locals.var_sqrt_phi_vd_dn1))) + (locals.var_weta_w * locals.var_sqrt_phi_vp0_dn1));
        locals.var_gammastar_dn2 = ((-(locals.var_leta_l * (locals.var_sqrt_phi_vs_dn2 + locals.var_sqrt_phi_vd_dn2))) + (locals.var_weta_w * locals.var_sqrt_phi_vp0_dn2));
        locals.var_gammastar_dn3 = ((-(locals.var_leta_l * (locals.var_sqrt_phi_vs_dn3 + locals.var_sqrt_phi_vd_dn3))) + (locals.var_weta_w * locals.var_sqrt_phi_vp0_dn3));

        let assign860_e667: f64 = (locals.var_gammastar * locals.var_gammastar);
        let assign860_e669: f64 = (assign860_e667 + locals.var_vt_01);
        let assign860_e670: f64 = (assign860_e669).sqrt();
        locals.var_sqrt_gammastar = assign860_e670;
        locals.var_sqrt_gammastar_dn0 = (((locals.var_gammastar_dn0 * locals.var_gammastar) + (locals.var_gammastar * locals.var_gammastar_dn0)) / (2.0 * assign860_e670));
        locals.var_sqrt_gammastar_dn1 = (((locals.var_gammastar_dn1 * locals.var_gammastar) + (locals.var_gammastar * locals.var_gammastar_dn1)) / (2.0 * assign860_e670));
        locals.var_sqrt_gammastar_dn2 = (((locals.var_gammastar_dn2 * locals.var_gammastar) + (locals.var_gammastar * locals.var_gammastar_dn2)) / (2.0 * assign860_e670));
        locals.var_sqrt_gammastar_dn3 = (((locals.var_gammastar_dn3 * locals.var_gammastar) + (locals.var_gammastar * locals.var_gammastar_dn3)) / (2.0 * assign860_e670));

        let assign870_e674: f64 = (locals.var_gammastar + locals.var_sqrt_gammastar);
        let assign870_e675: f64 = (0.5 * assign870_e674);
        locals.var_gammaprime = assign870_e675;
        locals.var_gammaprime_dn0 = (0.5 * (locals.var_gammastar_dn0 + locals.var_sqrt_gammastar_dn0));
        locals.var_gammaprime_dn1 = (0.5 * (locals.var_gammastar_dn1 + locals.var_sqrt_gammastar_dn1));
        locals.var_gammaprime_dn2 = (0.5 * (locals.var_gammastar_dn2 + locals.var_sqrt_gammastar_dn2));
        locals.var_gammaprime_dn3 = (0.5 * (locals.var_gammastar_dn3 + locals.var_sqrt_gammastar_dn3));

        let assign880_e679: f64 = (0.25 * locals.var_gammaprime);
        let assign880_e681: f64 = (assign880_e679 * locals.var_gammaprime);
        let assign880_e682: f64 = (locals.var_vgprime + assign880_e681);
        let assign880_e683: f64 = (assign880_e682).sqrt();
        locals.var_big_sqrt_vp = assign880_e683;
        locals.var_big_sqrt_vp_dn0 = ((locals.var_vgprime_dn0 + (((0.25 * locals.var_gammaprime_dn0) * locals.var_gammaprime) + (assign880_e679 * locals.var_gammaprime_dn0))) / (2.0 * assign880_e683));
        locals.var_big_sqrt_vp_dn1 = ((locals.var_vgprime_dn1 + (((0.25 * locals.var_gammaprime_dn1) * locals.var_gammaprime) + (assign880_e679 * locals.var_gammaprime_dn1))) / (2.0 * assign880_e683));
        locals.var_big_sqrt_vp_dn2 = ((locals.var_vgprime_dn2 + (((0.25 * locals.var_gammaprime_dn2) * locals.var_gammaprime) + (assign880_e679 * locals.var_gammaprime_dn2))) / (2.0 * assign880_e683));
        locals.var_big_sqrt_vp_dn3 = ((locals.var_vgprime_dn3 + (((0.25 * locals.var_gammaprime_dn3) * locals.var_gammaprime) + (assign880_e679 * locals.var_gammaprime_dn3))) / (2.0 * assign880_e683));

        let assign890_e686: f64 = (locals.var_vgprime - locals.var_phi_t);
        let assign890_e691: f64 = (0.5 * locals.var_gammaprime);
        let assign890_e692: f64 = (locals.var_big_sqrt_vp - assign890_e691);
        let assign890_e693: f64 = (locals.var_gammaprime * assign890_e692);
        let assign890_e694: f64 = (assign890_e686 - assign890_e693);
        locals.var_vp = assign890_e694;
        locals.var_vp_dn0 = ((locals.var_vgprime_dn0 - locals.var_phi_t_dn0) - ((locals.var_gammaprime_dn0 * assign890_e692) + (locals.var_gammaprime * (locals.var_big_sqrt_vp_dn0 - (0.5 * locals.var_gammaprime_dn0)))));
        locals.var_vp_dn1 = ((locals.var_vgprime_dn1 - locals.var_phi_t_dn1) - ((locals.var_gammaprime_dn1 * assign890_e692) + (locals.var_gammaprime * (locals.var_big_sqrt_vp_dn1 - (0.5 * locals.var_gammaprime_dn1)))));
        locals.var_vp_dn2 = ((locals.var_vgprime_dn2 - locals.var_phi_t_dn2) - ((locals.var_gammaprime_dn2 * assign890_e692) + (locals.var_gammaprime * (locals.var_big_sqrt_vp_dn2 - (0.5 * locals.var_gammaprime_dn2)))));
        locals.var_vp_dn3 = ((locals.var_vgprime_dn3 - locals.var_phi_t_dn3) - ((locals.var_gammaprime_dn3 * assign890_e692) + (locals.var_gammaprime * (locals.var_big_sqrt_vp_dn3 - (0.5 * locals.var_gammaprime_dn3)))));

        let assign900_e697: f64 = (locals.var_vp - locals.var_vs);
        let assign900_e699: f64 = (assign900_e697 * locals.var_inv_vt);
        locals.var_tmp1 = assign900_e699;
        locals.var_tmp1_dn0 = ((locals.var_vp_dn0 - locals.var_vs_dn0) * locals.var_inv_vt);
        locals.var_tmp1_dn1 = (locals.var_vp_dn1 * locals.var_inv_vt);
        locals.var_tmp1_dn2 = ((locals.var_vp_dn2 - locals.var_vs_dn2) * locals.var_inv_vt);
        locals.var_tmp1_dn3 = ((locals.var_vp_dn3 - locals.var_vs_dn3) * locals.var_inv_vt);

        let assign910_e702: f64 = (-0.35);
        let assign910_e703: f64 = if locals.var_tmp1 > assign910_e702 { 1.0 } else { 0.0 };
        locals.var_guard7 = assign910_e703;

        let (assign920_e716, assign920_e716_d_n0, assign920_e716_d_n1, assign920_e716_d_n2, assign920_e716_d_n3,) = {
    if (locals.var_guard7 != 0.0) {
        let assign920_e708: f64 = (1.3 + locals.var_tmp1);
        let assign920_e711: f64 = (locals.var_tmp1 + 1.6);
        let assign920_e712: f64 = (assign920_e711).ln();
        let assign920_e713: f64 = (assign920_e708 - assign920_e712);
        let assign920_e714: f64 = (2.0 / assign920_e713);
        (assign920_e714, (-((2.0 * (locals.var_tmp1_dn0 - (locals.var_tmp1_dn0 / assign920_e711))) / (assign920_e713 * assign920_e713))), (-((2.0 * (locals.var_tmp1_dn1 - (locals.var_tmp1_dn1 / assign920_e711))) / (assign920_e713 * assign920_e713))), (-((2.0 * (locals.var_tmp1_dn2 - (locals.var_tmp1_dn2 / assign920_e711))) / (assign920_e713 * assign920_e713))), (-((2.0 * (locals.var_tmp1_dn3 - (locals.var_tmp1_dn3 / assign920_e711))) / (assign920_e713 * assign920_e713))),)
    } else {
        (locals.var_z0, locals.var_z0_dn0, locals.var_z0_dn1, locals.var_z0_dn2, locals.var_z0_dn3,)
    }
};
        locals.var_z0 = assign920_e716;
        locals.var_z0_dn0 = assign920_e716_d_n0;
        locals.var_z0_dn1 = assign920_e716_d_n1;
        locals.var_z0_dn2 = assign920_e716_d_n2;
        locals.var_z0_dn3 = assign920_e716_d_n3;

        let (assign930_e729, assign930_e729_d_n0, assign930_e729_d_n1, assign930_e729_d_n2, assign930_e729_d_n3,) = {
    if (locals.var_guard7 != 0.0) {
        let assign930_e720: f64 = (2.0 + locals.var_z0);
        let assign930_e723: f64 = (1.0 + locals.var_tmp1);
        let assign930_e725: f64 = (locals.var_z0).ln();
        let assign930_e726: f64 = (assign930_e723 + assign930_e725);
        let assign930_e727: f64 = (assign930_e720 / assign930_e726);
        (assign930_e727, (((locals.var_z0_dn0 * assign930_e726) - (assign930_e720 * (locals.var_tmp1_dn0 + (locals.var_z0_dn0 / locals.var_z0)))) / (assign930_e726 * assign930_e726)), (((locals.var_z0_dn1 * assign930_e726) - (assign930_e720 * (locals.var_tmp1_dn1 + (locals.var_z0_dn1 / locals.var_z0)))) / (assign930_e726 * assign930_e726)), (((locals.var_z0_dn2 * assign930_e726) - (assign930_e720 * (locals.var_tmp1_dn2 + (locals.var_z0_dn2 / locals.var_z0)))) / (assign930_e726 * assign930_e726)), (((locals.var_z0_dn3 * assign930_e726) - (assign930_e720 * (locals.var_tmp1_dn3 + (locals.var_z0_dn3 / locals.var_z0)))) / (assign930_e726 * assign930_e726)),)
    } else {
        (locals.var_zk, locals.var_zk_dn0, locals.var_zk_dn1, locals.var_zk_dn2, locals.var_zk_dn3,)
    }
};
        locals.var_zk = assign930_e729;
        locals.var_zk_dn0 = assign930_e729_d_n0;
        locals.var_zk_dn1 = assign930_e729_d_n1;
        locals.var_zk_dn2 = assign930_e729_d_n2;
        locals.var_zk_dn3 = assign930_e729_d_n3;

        let (assign940_e742, assign940_e742_d_n0, assign940_e742_d_n1, assign940_e742_d_n2, assign940_e742_d_n3,) = {
    if (locals.var_guard7 != 0.0) {
        let assign940_e733: f64 = (1.0 + locals.var_tmp1);
        let assign940_e735: f64 = (locals.var_zk).ln();
        let assign940_e736: f64 = (assign940_e733 + assign940_e735);
        let assign940_e739: f64 = (2.0 + locals.var_zk);
        let assign940_e740: f64 = (assign940_e736 / assign940_e739);
        (assign940_e740, ((((locals.var_tmp1_dn0 + (locals.var_zk_dn0 / locals.var_zk)) * assign940_e739) - (assign940_e736 * locals.var_zk_dn0)) / (assign940_e739 * assign940_e739)), ((((locals.var_tmp1_dn1 + (locals.var_zk_dn1 / locals.var_zk)) * assign940_e739) - (assign940_e736 * locals.var_zk_dn1)) / (assign940_e739 * assign940_e739)), ((((locals.var_tmp1_dn2 + (locals.var_zk_dn2 / locals.var_zk)) * assign940_e739) - (assign940_e736 * locals.var_zk_dn2)) / (assign940_e739 * assign940_e739)), ((((locals.var_tmp1_dn3 + (locals.var_zk_dn3 / locals.var_zk)) * assign940_e739) - (assign940_e736 * locals.var_zk_dn3)) / (assign940_e739 * assign940_e739)),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign940_e742;
        locals.var_yk_dn0 = assign940_e742_d_n0;
        locals.var_yk_dn1 = assign940_e742_d_n1;
        locals.var_yk_dn2 = assign940_e742_d_n2;
        locals.var_yk_dn3 = assign940_e742_d_n3;

        let assign950_e745: f64 = (-15.0);
        let assign950_e746: f64 = if locals.var_tmp1 > assign950_e745 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign950_e746;

        let (assign960_e757, assign960_e757_d_n0, assign960_e757_d_n1, assign960_e757_d_n2, assign960_e757_d_n3,) = {
    if ((locals.var_guard7 == 0.0) && (locals.var_guard8 != 0.0)) {
        let assign960_e753: f64 = (-locals.var_tmp1);
        let assign960_e754: f64 = (assign960_e753).exp();
        let assign960_e755: f64 = (1.55 + assign960_e754);
        (assign960_e755, (assign960_e754 * (-locals.var_tmp1_dn0)), (assign960_e754 * (-locals.var_tmp1_dn1)), (assign960_e754 * (-locals.var_tmp1_dn2)), (assign960_e754 * (-locals.var_tmp1_dn3)),)
    } else {
        (locals.var_z0, locals.var_z0_dn0, locals.var_z0_dn1, locals.var_z0_dn2, locals.var_z0_dn3,)
    }
};
        locals.var_z0 = assign960_e757;
        locals.var_z0_dn0 = assign960_e757_d_n0;
        locals.var_z0_dn1 = assign960_e757_d_n1;
        locals.var_z0_dn2 = assign960_e757_d_n2;
        locals.var_z0_dn3 = assign960_e757_d_n3;

        let (assign970_e773, assign970_e773_d_n0, assign970_e773_d_n1, assign970_e773_d_n2, assign970_e773_d_n3,) = {
    if ((locals.var_guard7 == 0.0) && (locals.var_guard8 != 0.0)) {
        let assign970_e764: f64 = (2.0 + locals.var_z0);
        let assign970_e767: f64 = (1.0 + locals.var_tmp1);
        let assign970_e769: f64 = (locals.var_z0).ln();
        let assign970_e770: f64 = (assign970_e767 + assign970_e769);
        let assign970_e771: f64 = (assign970_e764 / assign970_e770);
        (assign970_e771, (((locals.var_z0_dn0 * assign970_e770) - (assign970_e764 * (locals.var_tmp1_dn0 + (locals.var_z0_dn0 / locals.var_z0)))) / (assign970_e770 * assign970_e770)), (((locals.var_z0_dn1 * assign970_e770) - (assign970_e764 * (locals.var_tmp1_dn1 + (locals.var_z0_dn1 / locals.var_z0)))) / (assign970_e770 * assign970_e770)), (((locals.var_z0_dn2 * assign970_e770) - (assign970_e764 * (locals.var_tmp1_dn2 + (locals.var_z0_dn2 / locals.var_z0)))) / (assign970_e770 * assign970_e770)), (((locals.var_z0_dn3 * assign970_e770) - (assign970_e764 * (locals.var_tmp1_dn3 + (locals.var_z0_dn3 / locals.var_z0)))) / (assign970_e770 * assign970_e770)),)
    } else {
        (locals.var_zk, locals.var_zk_dn0, locals.var_zk_dn1, locals.var_zk_dn2, locals.var_zk_dn3,)
    }
};
        locals.var_zk = assign970_e773;
        locals.var_zk_dn0 = assign970_e773_d_n0;
        locals.var_zk_dn1 = assign970_e773_d_n1;
        locals.var_zk_dn2 = assign970_e773_d_n2;
        locals.var_zk_dn3 = assign970_e773_d_n3;

        let (assign980_e789, assign980_e789_d_n0, assign980_e789_d_n1, assign980_e789_d_n2, assign980_e789_d_n3,) = {
    if ((locals.var_guard7 == 0.0) && (locals.var_guard8 != 0.0)) {
        let assign980_e780: f64 = (1.0 + locals.var_tmp1);
        let assign980_e782: f64 = (locals.var_zk).ln();
        let assign980_e783: f64 = (assign980_e780 + assign980_e782);
        let assign980_e786: f64 = (2.0 + locals.var_zk);
        let assign980_e787: f64 = (assign980_e783 / assign980_e786);
        (assign980_e787, ((((locals.var_tmp1_dn0 + (locals.var_zk_dn0 / locals.var_zk)) * assign980_e786) - (assign980_e783 * locals.var_zk_dn0)) / (assign980_e786 * assign980_e786)), ((((locals.var_tmp1_dn1 + (locals.var_zk_dn1 / locals.var_zk)) * assign980_e786) - (assign980_e783 * locals.var_zk_dn1)) / (assign980_e786 * assign980_e786)), ((((locals.var_tmp1_dn2 + (locals.var_zk_dn2 / locals.var_zk)) * assign980_e786) - (assign980_e783 * locals.var_zk_dn2)) / (assign980_e786 * assign980_e786)), ((((locals.var_tmp1_dn3 + (locals.var_zk_dn3 / locals.var_zk)) * assign980_e786) - (assign980_e783 * locals.var_zk_dn3)) / (assign980_e786 * assign980_e786)),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign980_e789;
        locals.var_yk_dn0 = assign980_e789_d_n0;
        locals.var_yk_dn1 = assign980_e789_d_n1;
        locals.var_yk_dn2 = assign980_e789_d_n2;
        locals.var_yk_dn3 = assign980_e789_d_n3;

        let assign990_e792: f64 = (-23.0);
        let assign990_e793: f64 = if locals.var_tmp1 > assign990_e792 { 1.0 } else { 0.0 };
        locals.var_guard9 = assign990_e793;

        let (assign1000_e809, assign1000_e809_d_n0, assign1000_e809_d_n1, assign1000_e809_d_n2, assign1000_e809_d_n3,) = {
    if (((locals.var_guard7 == 0.0) && (locals.var_guard8 == 0.0)) && (locals.var_guard9 != 0.0)) {
        let assign1000_e804: f64 = (-locals.var_tmp1);
        let assign1000_e805: f64 = (assign1000_e804).exp();
        let assign1000_e806: f64 = (2.0 + assign1000_e805);
        let assign1000_e807: f64 = (1.0 / assign1000_e806);
        (assign1000_e807, (-((assign1000_e805 * (-locals.var_tmp1_dn0)) / (assign1000_e806 * assign1000_e806))), (-((assign1000_e805 * (-locals.var_tmp1_dn1)) / (assign1000_e806 * assign1000_e806))), (-((assign1000_e805 * (-locals.var_tmp1_dn2)) / (assign1000_e806 * assign1000_e806))), (-((assign1000_e805 * (-locals.var_tmp1_dn3)) / (assign1000_e806 * assign1000_e806))),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1000_e809;
        locals.var_yk_dn0 = assign1000_e809_d_n0;
        locals.var_yk_dn1 = assign1000_e809_d_n1;
        locals.var_yk_dn2 = assign1000_e809_d_n2;
        locals.var_yk_dn3 = assign1000_e809_d_n3;

        let (assign1010_e823, assign1010_e823_d_n0, assign1010_e823_d_n1, assign1010_e823_d_n2, assign1010_e823_d_n3,) = {
    if (((locals.var_guard7 == 0.0) && (locals.var_guard8 == 0.0)) && (locals.var_guard9 == 0.0)) {
        let assign1010_e819: f64 = (locals.var_tmp1).exp();
        let assign1010_e821: f64 = (assign1010_e819 + 1e-64);
        (assign1010_e821, (assign1010_e819 * locals.var_tmp1_dn0), (assign1010_e819 * locals.var_tmp1_dn1), (assign1010_e819 * locals.var_tmp1_dn2), (assign1010_e819 * locals.var_tmp1_dn3),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1010_e823;
        locals.var_yk_dn0 = assign1010_e823_d_n0;
        locals.var_yk_dn1 = assign1010_e823_d_n1;
        locals.var_yk_dn2 = assign1010_e823_d_n2;
        locals.var_yk_dn3 = assign1010_e823_d_n3;

        let assign1020_e827: f64 = (1.0 + locals.var_yk);
        let assign1020_e828: f64 = (locals.var_yk * assign1020_e827);
        locals.var_if_ = assign1020_e828;
        locals.var_if__dn0 = ((locals.var_yk_dn0 * assign1020_e827) + (locals.var_yk * locals.var_yk_dn0));
        locals.var_if__dn1 = ((locals.var_yk_dn1 * assign1020_e827) + (locals.var_yk * locals.var_yk_dn1));
        locals.var_if__dn2 = ((locals.var_yk_dn2 * assign1020_e827) + (locals.var_yk * locals.var_yk_dn2));
        locals.var_if__dn3 = ((locals.var_yk_dn3 * assign1020_e827) + (locals.var_yk * locals.var_yk_dn3));

        let assign1030_e830: f64 = (locals.var_if_).sqrt();
        locals.var_sqrt_if = assign1030_e830;
        locals.var_sqrt_if_dn0 = (locals.var_if__dn0 / (2.0 * assign1030_e830));
        locals.var_sqrt_if_dn1 = (locals.var_if__dn1 / (2.0 * assign1030_e830));
        locals.var_sqrt_if_dn2 = (locals.var_if__dn2 / (2.0 * assign1030_e830));
        locals.var_sqrt_if_dn3 = (locals.var_if__dn3 / (2.0 * assign1030_e830));

        locals.var_dif_dv = locals.var_yk;
        locals.var_dif_dv_dn0 = locals.var_yk_dn0;
        locals.var_dif_dv_dn1 = locals.var_yk_dn1;
        locals.var_dif_dv_dn2 = locals.var_yk_dn2;
        locals.var_dif_dv_dn3 = locals.var_yk_dn3;

        let assign1050_e834: f64 = (locals.var_vt / locals.var_vc);
        locals.var_vt_vc = assign1050_e834;

        let assign1060_e838: f64 = (locals.var_sqrt_if * locals.var_vt_vc);
        let assign1060_e839: f64 = (0.25 + assign1060_e838);
        let assign1060_e840: f64 = (assign1060_e839).sqrt();
        locals.var_vdss_sqrt = assign1060_e840;
        locals.var_vdss_sqrt_dn0 = ((locals.var_sqrt_if_dn0 * locals.var_vt_vc) / (2.0 * assign1060_e840));
        locals.var_vdss_sqrt_dn1 = ((locals.var_sqrt_if_dn1 * locals.var_vt_vc) / (2.0 * assign1060_e840));
        locals.var_vdss_sqrt_dn2 = ((locals.var_sqrt_if_dn2 * locals.var_vt_vc) / (2.0 * assign1060_e840));
        locals.var_vdss_sqrt_dn3 = ((locals.var_sqrt_if_dn3 * locals.var_vt_vc) / (2.0 * assign1060_e840));

        let assign1070_e844: f64 = (locals.var_vdss_sqrt - 0.5);
        let assign1070_e845: f64 = (locals.var_vc * assign1070_e844);
        locals.var_vdss = assign1070_e845;
        locals.var_vdss_dn0 = (locals.var_vc * locals.var_vdss_sqrt_dn0);
        locals.var_vdss_dn1 = (locals.var_vc * locals.var_vdss_sqrt_dn1);
        locals.var_vdss_dn2 = (locals.var_vc * locals.var_vdss_sqrt_dn2);
        locals.var_vdss_dn3 = (locals.var_vc * locals.var_vdss_sqrt_dn3);

        let assign1080_e849: f64 = (locals.var_vd - locals.var_vs);
        let assign1080_e850: f64 = (0.5 * assign1080_e849);
        locals.var_vds = assign1080_e850;
        locals.var_vds_dn0 = (0.5 * (locals.var_vd_dn0 - locals.var_vs_dn0));
        locals.var_vds_dn2 = (0.5 * (locals.var_vd_dn2 - locals.var_vs_dn2));
        locals.var_vds_dn3 = (0.5 * (locals.var_vd_dn3 - locals.var_vs_dn3));

        let assign1090_e856: f64 = (locals.var_vdss * locals.var_inv_vt);
        let assign1090_e857: f64 = (locals.var_sqrt_if - assign1090_e856);
        let assign1090_e858: f64 = (p.p25 * assign1090_e857);
        let assign1090_e860: f64 = (assign1090_e858 + 0.015625);
        let assign1090_e861: f64 = (locals.var_vt_vt_16 * assign1090_e860);
        locals.var_deltav_2 = assign1090_e861;
        locals.var_deltav_2_dn0 = (locals.var_vt_vt_16 * (p.p25 * (locals.var_sqrt_if_dn0 - (locals.var_vdss_dn0 * locals.var_inv_vt))));
        locals.var_deltav_2_dn1 = (locals.var_vt_vt_16 * (p.p25 * (locals.var_sqrt_if_dn1 - (locals.var_vdss_dn1 * locals.var_inv_vt))));
        locals.var_deltav_2_dn2 = (locals.var_vt_vt_16 * (p.p25 * (locals.var_sqrt_if_dn2 - (locals.var_vdss_dn2 * locals.var_inv_vt))));
        locals.var_deltav_2_dn3 = (locals.var_vt_vt_16 * (p.p25 * (locals.var_sqrt_if_dn3 - (locals.var_vdss_dn3 * locals.var_inv_vt))));

        let assign1100_e864: f64 = (locals.var_vdss * locals.var_vdss);
        let assign1100_e866: f64 = (assign1100_e864 + locals.var_deltav_2);
        let assign1100_e867: f64 = (assign1100_e866).sqrt();
        locals.var_sqrt_vdss_deltav = assign1100_e867;
        locals.var_sqrt_vdss_deltav_dn0 = ((((locals.var_vdss_dn0 * locals.var_vdss) + (locals.var_vdss * locals.var_vdss_dn0)) + locals.var_deltav_2_dn0) / (2.0 * assign1100_e867));
        locals.var_sqrt_vdss_deltav_dn1 = ((((locals.var_vdss_dn1 * locals.var_vdss) + (locals.var_vdss * locals.var_vdss_dn1)) + locals.var_deltav_2_dn1) / (2.0 * assign1100_e867));
        locals.var_sqrt_vdss_deltav_dn2 = ((((locals.var_vdss_dn2 * locals.var_vdss) + (locals.var_vdss * locals.var_vdss_dn2)) + locals.var_deltav_2_dn2) / (2.0 * assign1100_e867));
        locals.var_sqrt_vdss_deltav_dn3 = ((((locals.var_vdss_dn3 * locals.var_vdss) + (locals.var_vdss * locals.var_vdss_dn3)) + locals.var_deltav_2_dn3) / (2.0 * assign1100_e867));

        let assign1110_e870: f64 = (locals.var_vds - locals.var_vdss);
        let assign1110_e873: f64 = (locals.var_vds - locals.var_vdss);
        let assign1110_e874: f64 = (assign1110_e870 * assign1110_e873);
        let assign1110_e876: f64 = (assign1110_e874 + locals.var_deltav_2);
        let assign1110_e877: f64 = (assign1110_e876).sqrt();
        locals.var_sqrt_vds_vdss_deltav = assign1110_e877;
        locals.var_sqrt_vds_vdss_deltav_dn0 = (((((locals.var_vds_dn0 - locals.var_vdss_dn0) * assign1110_e873) + (assign1110_e870 * (locals.var_vds_dn0 - locals.var_vdss_dn0))) + locals.var_deltav_2_dn0) / (2.0 * assign1110_e877));
        locals.var_sqrt_vds_vdss_deltav_dn1 = (((((-locals.var_vdss_dn1) * assign1110_e873) + (assign1110_e870 * (-locals.var_vdss_dn1))) + locals.var_deltav_2_dn1) / (2.0 * assign1110_e877));
        locals.var_sqrt_vds_vdss_deltav_dn2 = (((((locals.var_vds_dn2 - locals.var_vdss_dn2) * assign1110_e873) + (assign1110_e870 * (locals.var_vds_dn2 - locals.var_vdss_dn2))) + locals.var_deltav_2_dn2) / (2.0 * assign1110_e877));
        locals.var_sqrt_vds_vdss_deltav_dn3 = (((((locals.var_vds_dn3 - locals.var_vdss_dn3) * assign1110_e873) + (assign1110_e870 * (locals.var_vds_dn3 - locals.var_vdss_dn3))) + locals.var_deltav_2_dn3) / (2.0 * assign1110_e877));

        let assign1120_e880: f64 = (locals.var_sqrt_vdss_deltav - locals.var_sqrt_vds_vdss_deltav);
        locals.var_vip = assign1120_e880;
        locals.var_vip_dn0 = (locals.var_sqrt_vdss_deltav_dn0 - locals.var_sqrt_vds_vdss_deltav_dn0);
        locals.var_vip_dn1 = (locals.var_sqrt_vdss_deltav_dn1 - locals.var_sqrt_vds_vdss_deltav_dn1);
        locals.var_vip_dn2 = (locals.var_sqrt_vdss_deltav_dn2 - locals.var_sqrt_vds_vdss_deltav_dn2);
        locals.var_vip_dn3 = (locals.var_sqrt_vdss_deltav_dn3 - locals.var_sqrt_vds_vdss_deltav_dn3);

        let assign1130_e885: f64 = (locals.var_if_).ln();
        let assign1130_e886: f64 = (0.75 * assign1130_e885);
        let assign1130_e887: f64 = (locals.var_sqrt_if - assign1130_e886);
        let assign1130_e889: f64 = (assign1130_e887 * locals.var_vt_vc);
        let assign1130_e890: f64 = (0.25 + assign1130_e889);
        let assign1130_e891: f64 = (assign1130_e890).sqrt();
        locals.var_vdssprime_sqrt = assign1130_e891;
        locals.var_vdssprime_sqrt_dn0 = (((locals.var_sqrt_if_dn0 - (0.75 * (locals.var_if__dn0 / locals.var_if_))) * locals.var_vt_vc) / (2.0 * assign1130_e891));
        locals.var_vdssprime_sqrt_dn1 = (((locals.var_sqrt_if_dn1 - (0.75 * (locals.var_if__dn1 / locals.var_if_))) * locals.var_vt_vc) / (2.0 * assign1130_e891));
        locals.var_vdssprime_sqrt_dn2 = (((locals.var_sqrt_if_dn2 - (0.75 * (locals.var_if__dn2 / locals.var_if_))) * locals.var_vt_vc) / (2.0 * assign1130_e891));
        locals.var_vdssprime_sqrt_dn3 = (((locals.var_sqrt_if_dn3 - (0.75 * (locals.var_if__dn3 / locals.var_if_))) * locals.var_vt_vc) / (2.0 * assign1130_e891));

        let assign1140_e895: f64 = (locals.var_vdssprime_sqrt - 0.5);
        let assign1140_e896: f64 = (locals.var_vc * assign1140_e895);
        let assign1140_e898: f64 = (assign1140_e896 + locals.var_log_vc_vt);
        locals.var_vdssprime = assign1140_e898;
        locals.var_vdssprime_dn0 = (locals.var_vc * locals.var_vdssprime_sqrt_dn0);
        locals.var_vdssprime_dn1 = (locals.var_vc * locals.var_vdssprime_sqrt_dn1);
        locals.var_vdssprime_dn2 = (locals.var_vc * locals.var_vdssprime_sqrt_dn2);
        locals.var_vdssprime_dn3 = (locals.var_vc * locals.var_vdssprime_sqrt_dn3);

        let assign1150_e901: f64 = (locals.var_vds - locals.var_vdssprime);
        locals.var_vdsprime = assign1150_e901;
        locals.var_vdsprime_dn0 = (locals.var_vds_dn0 - locals.var_vdssprime_dn0);
        locals.var_vdsprime_dn1 = (-locals.var_vdssprime_dn1);
        locals.var_vdsprime_dn2 = (locals.var_vds_dn2 - locals.var_vdssprime_dn2);
        locals.var_vdsprime_dn3 = (locals.var_vds_dn3 - locals.var_vdssprime_dn3);

        let assign1160_e904: f64 = (locals.var_vdssprime * locals.var_vdssprime);
        let assign1160_e906: f64 = (assign1160_e904 + locals.var_deltav_2);
        let assign1160_e907: f64 = (assign1160_e906).sqrt();
        locals.var_sqrt_vdssprime_deltav = assign1160_e907;
        locals.var_sqrt_vdssprime_deltav_dn0 = ((((locals.var_vdssprime_dn0 * locals.var_vdssprime) + (locals.var_vdssprime * locals.var_vdssprime_dn0)) + locals.var_deltav_2_dn0) / (2.0 * assign1160_e907));
        locals.var_sqrt_vdssprime_deltav_dn1 = ((((locals.var_vdssprime_dn1 * locals.var_vdssprime) + (locals.var_vdssprime * locals.var_vdssprime_dn1)) + locals.var_deltav_2_dn1) / (2.0 * assign1160_e907));
        locals.var_sqrt_vdssprime_deltav_dn2 = ((((locals.var_vdssprime_dn2 * locals.var_vdssprime) + (locals.var_vdssprime * locals.var_vdssprime_dn2)) + locals.var_deltav_2_dn2) / (2.0 * assign1160_e907));
        locals.var_sqrt_vdssprime_deltav_dn3 = ((((locals.var_vdssprime_dn3 * locals.var_vdssprime) + (locals.var_vdssprime * locals.var_vdssprime_dn3)) + locals.var_deltav_2_dn3) / (2.0 * assign1160_e907));

        let assign1170_e910: f64 = (locals.var_vdsprime * locals.var_vdsprime);
        let assign1170_e912: f64 = (assign1170_e910 + locals.var_deltav_2);
        let assign1170_e913: f64 = (assign1170_e912).sqrt();
        locals.var_sqrt_vds_vdssprime_deltav = assign1170_e913;
        locals.var_sqrt_vds_vdssprime_deltav_dn0 = ((((locals.var_vdsprime_dn0 * locals.var_vdsprime) + (locals.var_vdsprime * locals.var_vdsprime_dn0)) + locals.var_deltav_2_dn0) / (2.0 * assign1170_e913));
        locals.var_sqrt_vds_vdssprime_deltav_dn1 = ((((locals.var_vdsprime_dn1 * locals.var_vdsprime) + (locals.var_vdsprime * locals.var_vdsprime_dn1)) + locals.var_deltav_2_dn1) / (2.0 * assign1170_e913));
        locals.var_sqrt_vds_vdssprime_deltav_dn2 = ((((locals.var_vdsprime_dn2 * locals.var_vdsprime) + (locals.var_vdsprime * locals.var_vdsprime_dn2)) + locals.var_deltav_2_dn2) / (2.0 * assign1170_e913));
        locals.var_sqrt_vds_vdssprime_deltav_dn3 = ((((locals.var_vdsprime_dn3 * locals.var_vdsprime) + (locals.var_vdsprime * locals.var_vdsprime_dn3)) + locals.var_deltav_2_dn3) / (2.0 * assign1170_e913));

        let assign1180_e916: f64 = (locals.var_vp - locals.var_vds);
        let assign1180_e918: f64 = (assign1180_e916 - locals.var_vs);
        let assign1180_e920: f64 = (assign1180_e918 - locals.var_sqrt_vdssprime_deltav);
        let assign1180_e922: f64 = (assign1180_e920 + locals.var_sqrt_vds_vdssprime_deltav);
        let assign1180_e924: f64 = (assign1180_e922 * locals.var_inv_vt);
        locals.var_tmp1 = assign1180_e924;
        locals.var_tmp1_dn0 = (((((locals.var_vp_dn0 - locals.var_vds_dn0) - locals.var_vs_dn0) - locals.var_sqrt_vdssprime_deltav_dn0) + locals.var_sqrt_vds_vdssprime_deltav_dn0) * locals.var_inv_vt);
        locals.var_tmp1_dn1 = (((locals.var_vp_dn1 - locals.var_sqrt_vdssprime_deltav_dn1) + locals.var_sqrt_vds_vdssprime_deltav_dn1) * locals.var_inv_vt);
        locals.var_tmp1_dn2 = (((((locals.var_vp_dn2 - locals.var_vds_dn2) - locals.var_vs_dn2) - locals.var_sqrt_vdssprime_deltav_dn2) + locals.var_sqrt_vds_vdssprime_deltav_dn2) * locals.var_inv_vt);
        locals.var_tmp1_dn3 = (((((locals.var_vp_dn3 - locals.var_vds_dn3) - locals.var_vs_dn3) - locals.var_sqrt_vdssprime_deltav_dn3) + locals.var_sqrt_vds_vdssprime_deltav_dn3) * locals.var_inv_vt);

        let assign1190_e927: f64 = (-0.35);
        let assign1190_e928: f64 = if locals.var_tmp1 > assign1190_e927 { 1.0 } else { 0.0 };
        locals.var_guard10 = assign1190_e928;

        let (assign1200_e941, assign1200_e941_d_n0, assign1200_e941_d_n1, assign1200_e941_d_n2, assign1200_e941_d_n3,) = {
    if (locals.var_guard10 != 0.0) {
        let assign1200_e933: f64 = (1.3 + locals.var_tmp1);
        let assign1200_e936: f64 = (locals.var_tmp1 + 1.6);
        let assign1200_e937: f64 = (assign1200_e936).ln();
        let assign1200_e938: f64 = (assign1200_e933 - assign1200_e937);
        let assign1200_e939: f64 = (2.0 / assign1200_e938);
        (assign1200_e939, (-((2.0 * (locals.var_tmp1_dn0 - (locals.var_tmp1_dn0 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))), (-((2.0 * (locals.var_tmp1_dn1 - (locals.var_tmp1_dn1 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))), (-((2.0 * (locals.var_tmp1_dn2 - (locals.var_tmp1_dn2 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))), (-((2.0 * (locals.var_tmp1_dn3 - (locals.var_tmp1_dn3 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))),)
    } else {
        (locals.var_z0, locals.var_z0_dn0, locals.var_z0_dn1, locals.var_z0_dn2, locals.var_z0_dn3,)
    }
};
        locals.var_z0 = assign1200_e941;
        locals.var_z0_dn0 = assign1200_e941_d_n0;
        locals.var_z0_dn1 = assign1200_e941_d_n1;
        locals.var_z0_dn2 = assign1200_e941_d_n2;
        locals.var_z0_dn3 = assign1200_e941_d_n3;

        let (assign1210_e954, assign1210_e954_d_n0, assign1210_e954_d_n1, assign1210_e954_d_n2, assign1210_e954_d_n3,) = {
    if (locals.var_guard10 != 0.0) {
        let assign1210_e945: f64 = (2.0 + locals.var_z0);
        let assign1210_e948: f64 = (1.0 + locals.var_tmp1);
        let assign1210_e950: f64 = (locals.var_z0).ln();
        let assign1210_e951: f64 = (assign1210_e948 + assign1210_e950);
        let assign1210_e952: f64 = (assign1210_e945 / assign1210_e951);
        (assign1210_e952, (((locals.var_z0_dn0 * assign1210_e951) - (assign1210_e945 * (locals.var_tmp1_dn0 + (locals.var_z0_dn0 / locals.var_z0)))) / (assign1210_e951 * assign1210_e951)), (((locals.var_z0_dn1 * assign1210_e951) - (assign1210_e945 * (locals.var_tmp1_dn1 + (locals.var_z0_dn1 / locals.var_z0)))) / (assign1210_e951 * assign1210_e951)), (((locals.var_z0_dn2 * assign1210_e951) - (assign1210_e945 * (locals.var_tmp1_dn2 + (locals.var_z0_dn2 / locals.var_z0)))) / (assign1210_e951 * assign1210_e951)), (((locals.var_z0_dn3 * assign1210_e951) - (assign1210_e945 * (locals.var_tmp1_dn3 + (locals.var_z0_dn3 / locals.var_z0)))) / (assign1210_e951 * assign1210_e951)),)
    } else {
        (locals.var_zk, locals.var_zk_dn0, locals.var_zk_dn1, locals.var_zk_dn2, locals.var_zk_dn3,)
    }
};
        locals.var_zk = assign1210_e954;
        locals.var_zk_dn0 = assign1210_e954_d_n0;
        locals.var_zk_dn1 = assign1210_e954_d_n1;
        locals.var_zk_dn2 = assign1210_e954_d_n2;
        locals.var_zk_dn3 = assign1210_e954_d_n3;

        let (assign1220_e967, assign1220_e967_d_n0, assign1220_e967_d_n1, assign1220_e967_d_n2, assign1220_e967_d_n3,) = {
    if (locals.var_guard10 != 0.0) {
        let assign1220_e958: f64 = (1.0 + locals.var_tmp1);
        let assign1220_e960: f64 = (locals.var_zk).ln();
        let assign1220_e961: f64 = (assign1220_e958 + assign1220_e960);
        let assign1220_e964: f64 = (2.0 + locals.var_zk);
        let assign1220_e965: f64 = (assign1220_e961 / assign1220_e964);
        (assign1220_e965, ((((locals.var_tmp1_dn0 + (locals.var_zk_dn0 / locals.var_zk)) * assign1220_e964) - (assign1220_e961 * locals.var_zk_dn0)) / (assign1220_e964 * assign1220_e964)), ((((locals.var_tmp1_dn1 + (locals.var_zk_dn1 / locals.var_zk)) * assign1220_e964) - (assign1220_e961 * locals.var_zk_dn1)) / (assign1220_e964 * assign1220_e964)), ((((locals.var_tmp1_dn2 + (locals.var_zk_dn2 / locals.var_zk)) * assign1220_e964) - (assign1220_e961 * locals.var_zk_dn2)) / (assign1220_e964 * assign1220_e964)), ((((locals.var_tmp1_dn3 + (locals.var_zk_dn3 / locals.var_zk)) * assign1220_e964) - (assign1220_e961 * locals.var_zk_dn3)) / (assign1220_e964 * assign1220_e964)),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1220_e967;
        locals.var_yk_dn0 = assign1220_e967_d_n0;
        locals.var_yk_dn1 = assign1220_e967_d_n1;
        locals.var_yk_dn2 = assign1220_e967_d_n2;
        locals.var_yk_dn3 = assign1220_e967_d_n3;

        let assign1230_e970: f64 = (-15.0);
        let assign1230_e971: f64 = if locals.var_tmp1 > assign1230_e970 { 1.0 } else { 0.0 };
        locals.var_guard11 = assign1230_e971;

        let (assign1240_e982, assign1240_e982_d_n0, assign1240_e982_d_n1, assign1240_e982_d_n2, assign1240_e982_d_n3,) = {
    if ((locals.var_guard10 == 0.0) && (locals.var_guard11 != 0.0)) {
        let assign1240_e978: f64 = (-locals.var_tmp1);
        let assign1240_e979: f64 = (assign1240_e978).exp();
        let assign1240_e980: f64 = (1.55 + assign1240_e979);
        (assign1240_e980, (assign1240_e979 * (-locals.var_tmp1_dn0)), (assign1240_e979 * (-locals.var_tmp1_dn1)), (assign1240_e979 * (-locals.var_tmp1_dn2)), (assign1240_e979 * (-locals.var_tmp1_dn3)),)
    } else {
        (locals.var_z0, locals.var_z0_dn0, locals.var_z0_dn1, locals.var_z0_dn2, locals.var_z0_dn3,)
    }
};
        locals.var_z0 = assign1240_e982;
        locals.var_z0_dn0 = assign1240_e982_d_n0;
        locals.var_z0_dn1 = assign1240_e982_d_n1;
        locals.var_z0_dn2 = assign1240_e982_d_n2;
        locals.var_z0_dn3 = assign1240_e982_d_n3;

        let (assign1250_e998, assign1250_e998_d_n0, assign1250_e998_d_n1, assign1250_e998_d_n2, assign1250_e998_d_n3,) = {
    if ((locals.var_guard10 == 0.0) && (locals.var_guard11 != 0.0)) {
        let assign1250_e989: f64 = (2.0 + locals.var_z0);
        let assign1250_e992: f64 = (1.0 + locals.var_tmp1);
        let assign1250_e994: f64 = (locals.var_z0).ln();
        let assign1250_e995: f64 = (assign1250_e992 + assign1250_e994);
        let assign1250_e996: f64 = (assign1250_e989 / assign1250_e995);
        (assign1250_e996, (((locals.var_z0_dn0 * assign1250_e995) - (assign1250_e989 * (locals.var_tmp1_dn0 + (locals.var_z0_dn0 / locals.var_z0)))) / (assign1250_e995 * assign1250_e995)), (((locals.var_z0_dn1 * assign1250_e995) - (assign1250_e989 * (locals.var_tmp1_dn1 + (locals.var_z0_dn1 / locals.var_z0)))) / (assign1250_e995 * assign1250_e995)), (((locals.var_z0_dn2 * assign1250_e995) - (assign1250_e989 * (locals.var_tmp1_dn2 + (locals.var_z0_dn2 / locals.var_z0)))) / (assign1250_e995 * assign1250_e995)), (((locals.var_z0_dn3 * assign1250_e995) - (assign1250_e989 * (locals.var_tmp1_dn3 + (locals.var_z0_dn3 / locals.var_z0)))) / (assign1250_e995 * assign1250_e995)),)
    } else {
        (locals.var_zk, locals.var_zk_dn0, locals.var_zk_dn1, locals.var_zk_dn2, locals.var_zk_dn3,)
    }
};
        locals.var_zk = assign1250_e998;
        locals.var_zk_dn0 = assign1250_e998_d_n0;
        locals.var_zk_dn1 = assign1250_e998_d_n1;
        locals.var_zk_dn2 = assign1250_e998_d_n2;
        locals.var_zk_dn3 = assign1250_e998_d_n3;

        let (assign1260_e1014, assign1260_e1014_d_n0, assign1260_e1014_d_n1, assign1260_e1014_d_n2, assign1260_e1014_d_n3,) = {
    if ((locals.var_guard10 == 0.0) && (locals.var_guard11 != 0.0)) {
        let assign1260_e1005: f64 = (1.0 + locals.var_tmp1);
        let assign1260_e1007: f64 = (locals.var_zk).ln();
        let assign1260_e1008: f64 = (assign1260_e1005 + assign1260_e1007);
        let assign1260_e1011: f64 = (2.0 + locals.var_zk);
        let assign1260_e1012: f64 = (assign1260_e1008 / assign1260_e1011);
        (assign1260_e1012, ((((locals.var_tmp1_dn0 + (locals.var_zk_dn0 / locals.var_zk)) * assign1260_e1011) - (assign1260_e1008 * locals.var_zk_dn0)) / (assign1260_e1011 * assign1260_e1011)), ((((locals.var_tmp1_dn1 + (locals.var_zk_dn1 / locals.var_zk)) * assign1260_e1011) - (assign1260_e1008 * locals.var_zk_dn1)) / (assign1260_e1011 * assign1260_e1011)), ((((locals.var_tmp1_dn2 + (locals.var_zk_dn2 / locals.var_zk)) * assign1260_e1011) - (assign1260_e1008 * locals.var_zk_dn2)) / (assign1260_e1011 * assign1260_e1011)), ((((locals.var_tmp1_dn3 + (locals.var_zk_dn3 / locals.var_zk)) * assign1260_e1011) - (assign1260_e1008 * locals.var_zk_dn3)) / (assign1260_e1011 * assign1260_e1011)),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1260_e1014;
        locals.var_yk_dn0 = assign1260_e1014_d_n0;
        locals.var_yk_dn1 = assign1260_e1014_d_n1;
        locals.var_yk_dn2 = assign1260_e1014_d_n2;
        locals.var_yk_dn3 = assign1260_e1014_d_n3;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign1270_e1017: f64 = (-23.0);
        let assign1270_e1018: f64 = if locals.var_tmp1 > assign1270_e1017 { 1.0 } else { 0.0 };
        locals.var_guard12 = assign1270_e1018;

        let (assign1280_e1034, assign1280_e1034_d_n0, assign1280_e1034_d_n1, assign1280_e1034_d_n2, assign1280_e1034_d_n3,) = {
    if (((locals.var_guard10 == 0.0) && (locals.var_guard11 == 0.0)) && (locals.var_guard12 != 0.0)) {
        let assign1280_e1029: f64 = (-locals.var_tmp1);
        let assign1280_e1030: f64 = (assign1280_e1029).exp();
        let assign1280_e1031: f64 = (2.0 + assign1280_e1030);
        let assign1280_e1032: f64 = (1.0 / assign1280_e1031);
        (assign1280_e1032, (-((assign1280_e1030 * (-locals.var_tmp1_dn0)) / (assign1280_e1031 * assign1280_e1031))), (-((assign1280_e1030 * (-locals.var_tmp1_dn1)) / (assign1280_e1031 * assign1280_e1031))), (-((assign1280_e1030 * (-locals.var_tmp1_dn2)) / (assign1280_e1031 * assign1280_e1031))), (-((assign1280_e1030 * (-locals.var_tmp1_dn3)) / (assign1280_e1031 * assign1280_e1031))),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1280_e1034;
        locals.var_yk_dn0 = assign1280_e1034_d_n0;
        locals.var_yk_dn1 = assign1280_e1034_d_n1;
        locals.var_yk_dn2 = assign1280_e1034_d_n2;
        locals.var_yk_dn3 = assign1280_e1034_d_n3;

        let (assign1290_e1048, assign1290_e1048_d_n0, assign1290_e1048_d_n1, assign1290_e1048_d_n2, assign1290_e1048_d_n3,) = {
    if (((locals.var_guard10 == 0.0) && (locals.var_guard11 == 0.0)) && (locals.var_guard12 == 0.0)) {
        let assign1290_e1044: f64 = (locals.var_tmp1).exp();
        let assign1290_e1046: f64 = (assign1290_e1044 + 1e-64);
        (assign1290_e1046, (assign1290_e1044 * locals.var_tmp1_dn0), (assign1290_e1044 * locals.var_tmp1_dn1), (assign1290_e1044 * locals.var_tmp1_dn2), (assign1290_e1044 * locals.var_tmp1_dn3),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1290_e1048;
        locals.var_yk_dn0 = assign1290_e1048_d_n0;
        locals.var_yk_dn1 = assign1290_e1048_d_n1;
        locals.var_yk_dn2 = assign1290_e1048_d_n2;
        locals.var_yk_dn3 = assign1290_e1048_d_n3;

        let assign1300_e1052: f64 = (1.0 + locals.var_yk);
        let assign1300_e1053: f64 = (locals.var_yk * assign1300_e1052);
        locals.var_irprime = assign1300_e1053;
        locals.var_irprime_dn0 = ((locals.var_yk_dn0 * assign1300_e1052) + (locals.var_yk * locals.var_yk_dn0));
        locals.var_irprime_dn1 = ((locals.var_yk_dn1 * assign1300_e1052) + (locals.var_yk * locals.var_yk_dn1));
        locals.var_irprime_dn2 = ((locals.var_yk_dn2 * assign1300_e1052) + (locals.var_yk * locals.var_yk_dn2));
        locals.var_irprime_dn3 = ((locals.var_yk_dn3 * assign1300_e1052) + (locals.var_yk * locals.var_yk_dn3));

        locals.var_dirprime_dv = locals.var_yk;
        locals.var_dirprime_dv_dn0 = locals.var_yk_dn0;
        locals.var_dirprime_dv_dn1 = locals.var_yk_dn1;
        locals.var_dirprime_dv_dn2 = locals.var_yk_dn2;
        locals.var_dirprime_dv_dn3 = locals.var_yk_dn3;

        let assign1330_e1061: f64 = (locals.var_vds - locals.var_vip);
        let assign1330_e1063: f64 = (assign1330_e1061 / locals.var_lc_ucrit);
        let assign1330_e1064: f64 = (1.0 + assign1330_e1063);
        let assign1330_e1065: f64 = (assign1330_e1064).ln();
        let assign1330_e1066: f64 = (locals.var_lc_lambda * assign1330_e1065);
        locals.var_deltal = assign1330_e1066;
        locals.var_deltal_dn0 = (locals.var_lc_lambda * (((locals.var_vds_dn0 - locals.var_vip_dn0) / locals.var_lc_ucrit) / assign1330_e1064));
        locals.var_deltal_dn1 = (locals.var_lc_lambda * (((-locals.var_vip_dn1) / locals.var_lc_ucrit) / assign1330_e1064));
        locals.var_deltal_dn2 = (locals.var_lc_lambda * (((locals.var_vds_dn2 - locals.var_vip_dn2) / locals.var_lc_ucrit) / assign1330_e1064));
        locals.var_deltal_dn3 = (locals.var_lc_lambda * (((locals.var_vds_dn3 - locals.var_vip_dn3) / locals.var_lc_ucrit) / assign1330_e1064));

        let assign1340_e1069: f64 = (locals.var_leff - locals.var_deltal);
        let assign1340_e1072: f64 = (locals.var_vds + locals.var_vip);
        let assign1340_e1074: f64 = (assign1340_e1072 * locals.var_inv_ucrit);
        let assign1340_e1075: f64 = (assign1340_e1069 + assign1340_e1074);
        locals.var_lprime = assign1340_e1075;
        locals.var_lprime_dn0 = ((-locals.var_deltal_dn0) + ((locals.var_vds_dn0 + locals.var_vip_dn0) * locals.var_inv_ucrit));
        locals.var_lprime_dn1 = ((-locals.var_deltal_dn1) + (locals.var_vip_dn1 * locals.var_inv_ucrit));
        locals.var_lprime_dn2 = ((-locals.var_deltal_dn2) + ((locals.var_vds_dn2 + locals.var_vip_dn2) * locals.var_inv_ucrit));
        locals.var_lprime_dn3 = ((-locals.var_deltal_dn3) + ((locals.var_vds_dn3 + locals.var_vip_dn3) * locals.var_inv_ucrit));

        let assign1350_e1078: f64 = (0.1 * locals.var_leff);
        locals.var_lmin = assign1350_e1078;

        let assign1360_e1081: f64 = (locals.var_lprime * locals.var_lprime);
        let assign1360_e1084: f64 = (locals.var_lmin * locals.var_lmin);
        let assign1360_e1085: f64 = (assign1360_e1081 + assign1360_e1084);
        let assign1360_e1086: f64 = (assign1360_e1085).sqrt();
        locals.var_sqrt_lprime_lmin = assign1360_e1086;
        locals.var_sqrt_lprime_lmin_dn0 = (((locals.var_lprime_dn0 * locals.var_lprime) + (locals.var_lprime * locals.var_lprime_dn0)) / (2.0 * assign1360_e1086));
        locals.var_sqrt_lprime_lmin_dn1 = (((locals.var_lprime_dn1 * locals.var_lprime) + (locals.var_lprime * locals.var_lprime_dn1)) / (2.0 * assign1360_e1086));
        locals.var_sqrt_lprime_lmin_dn2 = (((locals.var_lprime_dn2 * locals.var_lprime) + (locals.var_lprime * locals.var_lprime_dn2)) / (2.0 * assign1360_e1086));
        locals.var_sqrt_lprime_lmin_dn3 = (((locals.var_lprime_dn3 * locals.var_lprime) + (locals.var_lprime * locals.var_lprime_dn3)) / (2.0 * assign1360_e1086));

        let assign1370_e1090: f64 = (locals.var_lprime + locals.var_sqrt_lprime_lmin);
        let assign1370_e1091: f64 = (0.5 * assign1370_e1090);
        locals.var_leq = assign1370_e1091;
        locals.var_leq_dn0 = (0.5 * (locals.var_lprime_dn0 + locals.var_sqrt_lprime_lmin_dn0));
        locals.var_leq_dn1 = (0.5 * (locals.var_lprime_dn1 + locals.var_sqrt_lprime_lmin_dn1));
        locals.var_leq_dn2 = (0.5 * (locals.var_lprime_dn2 + locals.var_sqrt_lprime_lmin_dn2));
        locals.var_leq_dn3 = (0.5 * (locals.var_lprime_dn3 + locals.var_sqrt_lprime_lmin_dn3));

        let assign1380_e1094: f64 = (locals.var_vp - locals.var_vd);
        let assign1380_e1096: f64 = (assign1380_e1094 * locals.var_inv_vt);
        locals.var_tmp1 = assign1380_e1096;
        locals.var_tmp1_dn0 = ((locals.var_vp_dn0 - locals.var_vd_dn0) * locals.var_inv_vt);
        locals.var_tmp1_dn1 = (locals.var_vp_dn1 * locals.var_inv_vt);
        locals.var_tmp1_dn2 = ((locals.var_vp_dn2 - locals.var_vd_dn2) * locals.var_inv_vt);
        locals.var_tmp1_dn3 = ((locals.var_vp_dn3 - locals.var_vd_dn3) * locals.var_inv_vt);

        let assign1390_e1099: f64 = (-0.35);
        let assign1390_e1100: f64 = if locals.var_tmp1 > assign1390_e1099 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign1390_e1100;

        let (assign1400_e1113, assign1400_e1113_d_n0, assign1400_e1113_d_n1, assign1400_e1113_d_n2, assign1400_e1113_d_n3,) = {
    if (locals.var_guard13 != 0.0) {
        let assign1400_e1105: f64 = (1.3 + locals.var_tmp1);
        let assign1400_e1108: f64 = (locals.var_tmp1 + 1.6);
        let assign1400_e1109: f64 = (assign1400_e1108).ln();
        let assign1400_e1110: f64 = (assign1400_e1105 - assign1400_e1109);
        let assign1400_e1111: f64 = (2.0 / assign1400_e1110);
        (assign1400_e1111, (-((2.0 * (locals.var_tmp1_dn0 - (locals.var_tmp1_dn0 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))), (-((2.0 * (locals.var_tmp1_dn1 - (locals.var_tmp1_dn1 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))), (-((2.0 * (locals.var_tmp1_dn2 - (locals.var_tmp1_dn2 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))), (-((2.0 * (locals.var_tmp1_dn3 - (locals.var_tmp1_dn3 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))),)
    } else {
        (locals.var_z0, locals.var_z0_dn0, locals.var_z0_dn1, locals.var_z0_dn2, locals.var_z0_dn3,)
    }
};
        locals.var_z0 = assign1400_e1113;
        locals.var_z0_dn0 = assign1400_e1113_d_n0;
        locals.var_z0_dn1 = assign1400_e1113_d_n1;
        locals.var_z0_dn2 = assign1400_e1113_d_n2;
        locals.var_z0_dn3 = assign1400_e1113_d_n3;

        let (assign1410_e1126, assign1410_e1126_d_n0, assign1410_e1126_d_n1, assign1410_e1126_d_n2, assign1410_e1126_d_n3,) = {
    if (locals.var_guard13 != 0.0) {
        let assign1410_e1117: f64 = (2.0 + locals.var_z0);
        let assign1410_e1120: f64 = (1.0 + locals.var_tmp1);
        let assign1410_e1122: f64 = (locals.var_z0).ln();
        let assign1410_e1123: f64 = (assign1410_e1120 + assign1410_e1122);
        let assign1410_e1124: f64 = (assign1410_e1117 / assign1410_e1123);
        (assign1410_e1124, (((locals.var_z0_dn0 * assign1410_e1123) - (assign1410_e1117 * (locals.var_tmp1_dn0 + (locals.var_z0_dn0 / locals.var_z0)))) / (assign1410_e1123 * assign1410_e1123)), (((locals.var_z0_dn1 * assign1410_e1123) - (assign1410_e1117 * (locals.var_tmp1_dn1 + (locals.var_z0_dn1 / locals.var_z0)))) / (assign1410_e1123 * assign1410_e1123)), (((locals.var_z0_dn2 * assign1410_e1123) - (assign1410_e1117 * (locals.var_tmp1_dn2 + (locals.var_z0_dn2 / locals.var_z0)))) / (assign1410_e1123 * assign1410_e1123)), (((locals.var_z0_dn3 * assign1410_e1123) - (assign1410_e1117 * (locals.var_tmp1_dn3 + (locals.var_z0_dn3 / locals.var_z0)))) / (assign1410_e1123 * assign1410_e1123)),)
    } else {
        (locals.var_zk, locals.var_zk_dn0, locals.var_zk_dn1, locals.var_zk_dn2, locals.var_zk_dn3,)
    }
};
        locals.var_zk = assign1410_e1126;
        locals.var_zk_dn0 = assign1410_e1126_d_n0;
        locals.var_zk_dn1 = assign1410_e1126_d_n1;
        locals.var_zk_dn2 = assign1410_e1126_d_n2;
        locals.var_zk_dn3 = assign1410_e1126_d_n3;

        let (assign1420_e1139, assign1420_e1139_d_n0, assign1420_e1139_d_n1, assign1420_e1139_d_n2, assign1420_e1139_d_n3,) = {
    if (locals.var_guard13 != 0.0) {
        let assign1420_e1130: f64 = (1.0 + locals.var_tmp1);
        let assign1420_e1132: f64 = (locals.var_zk).ln();
        let assign1420_e1133: f64 = (assign1420_e1130 + assign1420_e1132);
        let assign1420_e1136: f64 = (2.0 + locals.var_zk);
        let assign1420_e1137: f64 = (assign1420_e1133 / assign1420_e1136);
        (assign1420_e1137, ((((locals.var_tmp1_dn0 + (locals.var_zk_dn0 / locals.var_zk)) * assign1420_e1136) - (assign1420_e1133 * locals.var_zk_dn0)) / (assign1420_e1136 * assign1420_e1136)), ((((locals.var_tmp1_dn1 + (locals.var_zk_dn1 / locals.var_zk)) * assign1420_e1136) - (assign1420_e1133 * locals.var_zk_dn1)) / (assign1420_e1136 * assign1420_e1136)), ((((locals.var_tmp1_dn2 + (locals.var_zk_dn2 / locals.var_zk)) * assign1420_e1136) - (assign1420_e1133 * locals.var_zk_dn2)) / (assign1420_e1136 * assign1420_e1136)), ((((locals.var_tmp1_dn3 + (locals.var_zk_dn3 / locals.var_zk)) * assign1420_e1136) - (assign1420_e1133 * locals.var_zk_dn3)) / (assign1420_e1136 * assign1420_e1136)),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1420_e1139;
        locals.var_yk_dn0 = assign1420_e1139_d_n0;
        locals.var_yk_dn1 = assign1420_e1139_d_n1;
        locals.var_yk_dn2 = assign1420_e1139_d_n2;
        locals.var_yk_dn3 = assign1420_e1139_d_n3;

        let assign1430_e1142: f64 = (-15.0);
        let assign1430_e1143: f64 = if locals.var_tmp1 > assign1430_e1142 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign1430_e1143;

        let (assign1440_e1154, assign1440_e1154_d_n0, assign1440_e1154_d_n1, assign1440_e1154_d_n2, assign1440_e1154_d_n3,) = {
    if ((locals.var_guard13 == 0.0) && (locals.var_guard14 != 0.0)) {
        let assign1440_e1150: f64 = (-locals.var_tmp1);
        let assign1440_e1151: f64 = (assign1440_e1150).exp();
        let assign1440_e1152: f64 = (1.55 + assign1440_e1151);
        (assign1440_e1152, (assign1440_e1151 * (-locals.var_tmp1_dn0)), (assign1440_e1151 * (-locals.var_tmp1_dn1)), (assign1440_e1151 * (-locals.var_tmp1_dn2)), (assign1440_e1151 * (-locals.var_tmp1_dn3)),)
    } else {
        (locals.var_z0, locals.var_z0_dn0, locals.var_z0_dn1, locals.var_z0_dn2, locals.var_z0_dn3,)
    }
};
        locals.var_z0 = assign1440_e1154;
        locals.var_z0_dn0 = assign1440_e1154_d_n0;
        locals.var_z0_dn1 = assign1440_e1154_d_n1;
        locals.var_z0_dn2 = assign1440_e1154_d_n2;
        locals.var_z0_dn3 = assign1440_e1154_d_n3;

        let (assign1450_e1170, assign1450_e1170_d_n0, assign1450_e1170_d_n1, assign1450_e1170_d_n2, assign1450_e1170_d_n3,) = {
    if ((locals.var_guard13 == 0.0) && (locals.var_guard14 != 0.0)) {
        let assign1450_e1161: f64 = (2.0 + locals.var_z0);
        let assign1450_e1164: f64 = (1.0 + locals.var_tmp1);
        let assign1450_e1166: f64 = (locals.var_z0).ln();
        let assign1450_e1167: f64 = (assign1450_e1164 + assign1450_e1166);
        let assign1450_e1168: f64 = (assign1450_e1161 / assign1450_e1167);
        (assign1450_e1168, (((locals.var_z0_dn0 * assign1450_e1167) - (assign1450_e1161 * (locals.var_tmp1_dn0 + (locals.var_z0_dn0 / locals.var_z0)))) / (assign1450_e1167 * assign1450_e1167)), (((locals.var_z0_dn1 * assign1450_e1167) - (assign1450_e1161 * (locals.var_tmp1_dn1 + (locals.var_z0_dn1 / locals.var_z0)))) / (assign1450_e1167 * assign1450_e1167)), (((locals.var_z0_dn2 * assign1450_e1167) - (assign1450_e1161 * (locals.var_tmp1_dn2 + (locals.var_z0_dn2 / locals.var_z0)))) / (assign1450_e1167 * assign1450_e1167)), (((locals.var_z0_dn3 * assign1450_e1167) - (assign1450_e1161 * (locals.var_tmp1_dn3 + (locals.var_z0_dn3 / locals.var_z0)))) / (assign1450_e1167 * assign1450_e1167)),)
    } else {
        (locals.var_zk, locals.var_zk_dn0, locals.var_zk_dn1, locals.var_zk_dn2, locals.var_zk_dn3,)
    }
};
        locals.var_zk = assign1450_e1170;
        locals.var_zk_dn0 = assign1450_e1170_d_n0;
        locals.var_zk_dn1 = assign1450_e1170_d_n1;
        locals.var_zk_dn2 = assign1450_e1170_d_n2;
        locals.var_zk_dn3 = assign1450_e1170_d_n3;

        let (assign1460_e1186, assign1460_e1186_d_n0, assign1460_e1186_d_n1, assign1460_e1186_d_n2, assign1460_e1186_d_n3,) = {
    if ((locals.var_guard13 == 0.0) && (locals.var_guard14 != 0.0)) {
        let assign1460_e1177: f64 = (1.0 + locals.var_tmp1);
        let assign1460_e1179: f64 = (locals.var_zk).ln();
        let assign1460_e1180: f64 = (assign1460_e1177 + assign1460_e1179);
        let assign1460_e1183: f64 = (2.0 + locals.var_zk);
        let assign1460_e1184: f64 = (assign1460_e1180 / assign1460_e1183);
        (assign1460_e1184, ((((locals.var_tmp1_dn0 + (locals.var_zk_dn0 / locals.var_zk)) * assign1460_e1183) - (assign1460_e1180 * locals.var_zk_dn0)) / (assign1460_e1183 * assign1460_e1183)), ((((locals.var_tmp1_dn1 + (locals.var_zk_dn1 / locals.var_zk)) * assign1460_e1183) - (assign1460_e1180 * locals.var_zk_dn1)) / (assign1460_e1183 * assign1460_e1183)), ((((locals.var_tmp1_dn2 + (locals.var_zk_dn2 / locals.var_zk)) * assign1460_e1183) - (assign1460_e1180 * locals.var_zk_dn2)) / (assign1460_e1183 * assign1460_e1183)), ((((locals.var_tmp1_dn3 + (locals.var_zk_dn3 / locals.var_zk)) * assign1460_e1183) - (assign1460_e1180 * locals.var_zk_dn3)) / (assign1460_e1183 * assign1460_e1183)),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1460_e1186;
        locals.var_yk_dn0 = assign1460_e1186_d_n0;
        locals.var_yk_dn1 = assign1460_e1186_d_n1;
        locals.var_yk_dn2 = assign1460_e1186_d_n2;
        locals.var_yk_dn3 = assign1460_e1186_d_n3;

        let assign1470_e1189: f64 = (-23.0);
        let assign1470_e1190: f64 = if locals.var_tmp1 > assign1470_e1189 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1470_e1190;

        let (assign1480_e1206, assign1480_e1206_d_n0, assign1480_e1206_d_n1, assign1480_e1206_d_n2, assign1480_e1206_d_n3,) = {
    if (((locals.var_guard13 == 0.0) && (locals.var_guard14 == 0.0)) && (locals.var_guard15 != 0.0)) {
        let assign1480_e1201: f64 = (-locals.var_tmp1);
        let assign1480_e1202: f64 = (assign1480_e1201).exp();
        let assign1480_e1203: f64 = (2.0 + assign1480_e1202);
        let assign1480_e1204: f64 = (1.0 / assign1480_e1203);
        (assign1480_e1204, (-((assign1480_e1202 * (-locals.var_tmp1_dn0)) / (assign1480_e1203 * assign1480_e1203))), (-((assign1480_e1202 * (-locals.var_tmp1_dn1)) / (assign1480_e1203 * assign1480_e1203))), (-((assign1480_e1202 * (-locals.var_tmp1_dn2)) / (assign1480_e1203 * assign1480_e1203))), (-((assign1480_e1202 * (-locals.var_tmp1_dn3)) / (assign1480_e1203 * assign1480_e1203))),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1480_e1206;
        locals.var_yk_dn0 = assign1480_e1206_d_n0;
        locals.var_yk_dn1 = assign1480_e1206_d_n1;
        locals.var_yk_dn2 = assign1480_e1206_d_n2;
        locals.var_yk_dn3 = assign1480_e1206_d_n3;

        let (assign1490_e1220, assign1490_e1220_d_n0, assign1490_e1220_d_n1, assign1490_e1220_d_n2, assign1490_e1220_d_n3,) = {
    if (((locals.var_guard13 == 0.0) && (locals.var_guard14 == 0.0)) && (locals.var_guard15 == 0.0)) {
        let assign1490_e1216: f64 = (locals.var_tmp1).exp();
        let assign1490_e1218: f64 = (assign1490_e1216 + 1e-64);
        (assign1490_e1218, (assign1490_e1216 * locals.var_tmp1_dn0), (assign1490_e1216 * locals.var_tmp1_dn1), (assign1490_e1216 * locals.var_tmp1_dn2), (assign1490_e1216 * locals.var_tmp1_dn3),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1490_e1220;
        locals.var_yk_dn0 = assign1490_e1220_d_n0;
        locals.var_yk_dn1 = assign1490_e1220_d_n1;
        locals.var_yk_dn2 = assign1490_e1220_d_n2;
        locals.var_yk_dn3 = assign1490_e1220_d_n3;

        let assign1500_e1224: f64 = (1.0 + locals.var_yk);
        let assign1500_e1225: f64 = (locals.var_yk * assign1500_e1224);
        locals.var_ir = assign1500_e1225;
        locals.var_ir_dn0 = ((locals.var_yk_dn0 * assign1500_e1224) + (locals.var_yk * locals.var_yk_dn0));
        locals.var_ir_dn1 = ((locals.var_yk_dn1 * assign1500_e1224) + (locals.var_yk * locals.var_yk_dn1));
        locals.var_ir_dn2 = ((locals.var_yk_dn2 * assign1500_e1224) + (locals.var_yk * locals.var_yk_dn2));
        locals.var_ir_dn3 = ((locals.var_yk_dn3 * assign1500_e1224) + (locals.var_yk * locals.var_yk_dn3));

        locals.var_dir_dv = locals.var_yk;
        locals.var_dir_dv_dn0 = locals.var_yk_dn0;
        locals.var_dir_dv_dn1 = locals.var_yk_dn1;
        locals.var_dir_dv_dn2 = locals.var_yk_dn2;
        locals.var_dir_dv_dn3 = locals.var_yk_dn3;

        let assign1530_e1231: f64 = (0.25 + locals.var_if_);
        locals.var_sif2 = assign1530_e1231;
        locals.var_sif2_dn0 = locals.var_if__dn0;
        locals.var_sif2_dn1 = locals.var_if__dn1;
        locals.var_sif2_dn2 = locals.var_if__dn2;
        locals.var_sif2_dn3 = locals.var_if__dn3;

        let assign1540_e1234: f64 = (0.25 + locals.var_ir);
        locals.var_sir2 = assign1540_e1234;
        locals.var_sir2_dn0 = locals.var_ir_dn0;
        locals.var_sir2_dn1 = locals.var_ir_dn1;
        locals.var_sir2_dn2 = locals.var_ir_dn2;
        locals.var_sir2_dn3 = locals.var_ir_dn3;

        let assign1550_e1236: f64 = (locals.var_sif2).sqrt();
        locals.var_sif = assign1550_e1236;
        locals.var_sif_dn0 = (locals.var_sif2_dn0 / (2.0 * assign1550_e1236));
        locals.var_sif_dn1 = (locals.var_sif2_dn1 / (2.0 * assign1550_e1236));
        locals.var_sif_dn2 = (locals.var_sif2_dn2 / (2.0 * assign1550_e1236));
        locals.var_sif_dn3 = (locals.var_sif2_dn3 / (2.0 * assign1550_e1236));

        let assign1560_e1238: f64 = (locals.var_sir2).sqrt();
        locals.var_sir = assign1560_e1238;
        locals.var_sir_dn0 = (locals.var_sir2_dn0 / (2.0 * assign1560_e1238));
        locals.var_sir_dn1 = (locals.var_sir2_dn1 / (2.0 * assign1560_e1238));
        locals.var_sir_dn2 = (locals.var_sir2_dn2 / (2.0 * assign1560_e1238));
        locals.var_sir_dn3 = (locals.var_sir2_dn3 / (2.0 * assign1560_e1238));

        let assign1570_e1241: f64 = (locals.var_sif + locals.var_sir);
        let assign1570_e1244: f64 = (locals.var_sif + locals.var_sir);
        let assign1570_e1245: f64 = (assign1570_e1241 * assign1570_e1244);
        locals.var_sif_sir_2 = assign1570_e1245;
        locals.var_sif_sir_2_dn0 = (((locals.var_sif_dn0 + locals.var_sir_dn0) * assign1570_e1244) + (assign1570_e1241 * (locals.var_sif_dn0 + locals.var_sir_dn0)));
        locals.var_sif_sir_2_dn1 = (((locals.var_sif_dn1 + locals.var_sir_dn1) * assign1570_e1244) + (assign1570_e1241 * (locals.var_sif_dn1 + locals.var_sir_dn1)));
        locals.var_sif_sir_2_dn2 = (((locals.var_sif_dn2 + locals.var_sir_dn2) * assign1570_e1244) + (assign1570_e1241 * (locals.var_sif_dn2 + locals.var_sir_dn2)));
        locals.var_sif_sir_2_dn3 = (((locals.var_sif_dn3 + locals.var_sir_dn3) * assign1570_e1244) + (assign1570_e1241 * (locals.var_sif_dn3 + locals.var_sir_dn3)));

        let assign1580_e1248: f64 = (locals.var_vp + locals.var_phi_t);
        let assign1580_e1250: f64 = (assign1580_e1248 + 1e-6);
        locals.var_vp_phi_eps = assign1580_e1250;
        locals.var_vp_phi_eps_dn0 = (locals.var_vp_dn0 + locals.var_phi_t_dn0);
        locals.var_vp_phi_eps_dn1 = (locals.var_vp_dn1 + locals.var_phi_t_dn1);
        locals.var_vp_phi_eps_dn2 = (locals.var_vp_dn2 + locals.var_phi_t_dn2);
        locals.var_vp_phi_eps_dn3 = (locals.var_vp_dn3 + locals.var_phi_t_dn3);

        let assign1590_e1253: f64 = (locals.var_vp_phi_eps).sqrt();
        let assign1590_e1254: f64 = (2.0 * assign1590_e1253);
        locals.var_sqrt_phi_vp_2 = assign1590_e1254;
        locals.var_sqrt_phi_vp_2_dn0 = (2.0 * (locals.var_vp_phi_eps_dn0 / (2.0 * assign1590_e1253)));
        locals.var_sqrt_phi_vp_2_dn1 = (2.0 * (locals.var_vp_phi_eps_dn1 / (2.0 * assign1590_e1253)));
        locals.var_sqrt_phi_vp_2_dn2 = (2.0 * (locals.var_vp_phi_eps_dn2 / (2.0 * assign1590_e1253)));
        locals.var_sqrt_phi_vp_2_dn3 = (2.0 * (locals.var_vp_phi_eps_dn3 / (2.0 * assign1590_e1253)));

        let assign1600_e1257: f64 = (locals.var_gamma_s / locals.var_sqrt_phi_vp_2);
        locals.var_n_1 = assign1600_e1257;
        locals.var_n_1_dn0 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn0) / (locals.var_sqrt_phi_vp_2 * locals.var_sqrt_phi_vp_2)));
        locals.var_n_1_dn1 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn1) / (locals.var_sqrt_phi_vp_2 * locals.var_sqrt_phi_vp_2)));
        locals.var_n_1_dn2 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn2) / (locals.var_sqrt_phi_vp_2 * locals.var_sqrt_phi_vp_2)));
        locals.var_n_1_dn3 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn3) / (locals.var_sqrt_phi_vp_2 * locals.var_sqrt_phi_vp_2)));

        let assign1610_e1261: f64 = (locals.var_sqrt_phi_vp_2 + locals.var_gamma_s);
        let assign1610_e1262: f64 = (locals.var_gamma_s / assign1610_e1261);
        locals.var_n_1_n = assign1610_e1262;
        locals.var_n_1_n_dn0 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn0) / (assign1610_e1261 * assign1610_e1261)));
        locals.var_n_1_n_dn1 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn1) / (assign1610_e1261 * assign1610_e1261)));
        locals.var_n_1_n_dn2 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn2) / (assign1610_e1261 * assign1610_e1261)));
        locals.var_n_1_n_dn3 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn3) / (assign1610_e1261 * assign1610_e1261)));

        let assign1620_e1265: f64 = (1.0 + locals.var_n_1);
        let assign1620_e1266: f64 = (-assign1620_e1265);
        let assign1620_e1268: f64 = (assign1620_e1266 * locals.var_vt);
        let assign1620_e1271: f64 = (0.66666666 + 0.66666666);
        let assign1620_e1275: f64 = (locals.var_sir * locals.var_sif);
        let assign1620_e1276: f64 = (locals.var_sir2 + assign1620_e1275);
        let assign1620_e1278: f64 = (assign1620_e1276 + locals.var_sif2);
        let assign1620_e1279: f64 = (assign1620_e1271 * assign1620_e1278);
        let assign1620_e1282: f64 = (locals.var_sif + locals.var_sir);
        let assign1620_e1283: f64 = (assign1620_e1279 / assign1620_e1282);
        let assign1620_e1285: f64 = (assign1620_e1283 - 1.0);
        let assign1620_e1286: f64 = (assign1620_e1268 * assign1620_e1285);
        locals.var_qi = assign1620_e1286;
        locals.var_qi_dn0 = ((((-locals.var_n_1_dn0) * locals.var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((locals.var_sir2_dn0 + ((locals.var_sir_dn0 * locals.var_sif) + (locals.var_sir * locals.var_sif_dn0))) + locals.var_sif2_dn0)) * assign1620_e1282) - (assign1620_e1279 * (locals.var_sif_dn0 + locals.var_sir_dn0))) / (assign1620_e1282 * assign1620_e1282))));
        locals.var_qi_dn1 = ((((-locals.var_n_1_dn1) * locals.var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((locals.var_sir2_dn1 + ((locals.var_sir_dn1 * locals.var_sif) + (locals.var_sir * locals.var_sif_dn1))) + locals.var_sif2_dn1)) * assign1620_e1282) - (assign1620_e1279 * (locals.var_sif_dn1 + locals.var_sir_dn1))) / (assign1620_e1282 * assign1620_e1282))));
        locals.var_qi_dn2 = ((((-locals.var_n_1_dn2) * locals.var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((locals.var_sir2_dn2 + ((locals.var_sir_dn2 * locals.var_sif) + (locals.var_sir * locals.var_sif_dn2))) + locals.var_sif2_dn2)) * assign1620_e1282) - (assign1620_e1279 * (locals.var_sif_dn2 + locals.var_sir_dn2))) / (assign1620_e1282 * assign1620_e1282))));
        locals.var_qi_dn3 = ((((-locals.var_n_1_dn3) * locals.var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((locals.var_sir2_dn3 + ((locals.var_sir_dn3 * locals.var_sif) + (locals.var_sir * locals.var_sif_dn3))) + locals.var_sif2_dn3)) * assign1620_e1282) - (assign1620_e1279 * (locals.var_sif_dn3 + locals.var_sir_dn3))) / (assign1620_e1282 * assign1620_e1282))));

        let assign1630_e1288: f64 = (-0.5);
        let assign1630_e1290: f64 = (assign1630_e1288 * locals.var_gamma_s);
        let assign1630_e1292: f64 = (assign1630_e1290 * locals.var_sqrt_phi_vp_2);
        let assign1630_e1295: f64 = (locals.var_n_1_n * locals.var_qi);
        let assign1630_e1296: f64 = (assign1630_e1292 - assign1630_e1295);
        locals.var_qb = assign1630_e1296;
        locals.var_qb_dn0 = ((assign1630_e1290 * locals.var_sqrt_phi_vp_2_dn0) - ((locals.var_n_1_n_dn0 * locals.var_qi) + (locals.var_n_1_n * locals.var_qi_dn0)));
        locals.var_qb_dn1 = ((assign1630_e1290 * locals.var_sqrt_phi_vp_2_dn1) - ((locals.var_n_1_n_dn1 * locals.var_qi) + (locals.var_n_1_n * locals.var_qi_dn1)));
        locals.var_qb_dn2 = ((assign1630_e1290 * locals.var_sqrt_phi_vp_2_dn2) - ((locals.var_n_1_n_dn2 * locals.var_qi) + (locals.var_n_1_n * locals.var_qi_dn2)));
        locals.var_qb_dn3 = ((assign1630_e1290 * locals.var_sqrt_phi_vp_2_dn3) - ((locals.var_n_1_n_dn3 * locals.var_qi) + (locals.var_n_1_n * locals.var_qi_dn3)));

        let assign1640_e1299: f64 = if p.p22 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1640_e1299;

        let (assign1650_e1308, assign1650_e1308_d_n0, assign1650_e1308_d_n1, assign1650_e1308_d_n2, assign1650_e1308_d_n3,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1650_e1303: f64 = (locals.var_vp * locals.var_vp);
        let assign1650_e1305: f64 = (assign1650_e1303 + locals.var_vt_vt_2);
        let assign1650_e1306: f64 = (assign1650_e1305).sqrt();
        (assign1650_e1306, (((locals.var_vp_dn0 * locals.var_vp) + (locals.var_vp * locals.var_vp_dn0)) / (2.0 * assign1650_e1306)), (((locals.var_vp_dn1 * locals.var_vp) + (locals.var_vp * locals.var_vp_dn1)) / (2.0 * assign1650_e1306)), (((locals.var_vp_dn2 * locals.var_vp) + (locals.var_vp * locals.var_vp_dn2)) / (2.0 * assign1650_e1306)), (((locals.var_vp_dn3 * locals.var_vp) + (locals.var_vp * locals.var_vp_dn3)) / (2.0 * assign1650_e1306)),)
    } else {
        (locals.var_sqrt_vp_vt, locals.var_sqrt_vp_vt_dn0, locals.var_sqrt_vp_vt_dn1, locals.var_sqrt_vp_vt_dn2, locals.var_sqrt_vp_vt_dn3,)
    }
};
        locals.var_sqrt_vp_vt = assign1650_e1308;
        locals.var_sqrt_vp_vt_dn0 = assign1650_e1308_d_n0;
        locals.var_sqrt_vp_vt_dn1 = assign1650_e1308_d_n1;
        locals.var_sqrt_vp_vt_dn2 = assign1650_e1308_d_n2;
        locals.var_sqrt_vp_vt_dn3 = assign1650_e1308_d_n3;

        let (assign1660_e1316, assign1660_e1316_d_n0, assign1660_e1316_d_n1, assign1660_e1316_d_n2, assign1660_e1316_d_n3,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1660_e1313: f64 = (locals.var_vp + locals.var_sqrt_vp_vt);
        let assign1660_e1314: f64 = (0.5 * assign1660_e1313);
        (assign1660_e1314, (0.5 * (locals.var_vp_dn0 + locals.var_sqrt_vp_vt_dn0)), (0.5 * (locals.var_vp_dn1 + locals.var_sqrt_vp_vt_dn1)), (0.5 * (locals.var_vp_dn2 + locals.var_sqrt_vp_vt_dn2)), (0.5 * (locals.var_vp_dn3 + locals.var_sqrt_vp_vt_dn3)),)
    } else {
        (locals.var_vpprime, locals.var_vpprime_dn0, locals.var_vpprime_dn1, locals.var_vpprime_dn2, locals.var_vpprime_dn3,)
    }
};
        locals.var_vpprime = assign1660_e1316;
        locals.var_vpprime_dn0 = assign1660_e1316_d_n0;
        locals.var_vpprime_dn1 = assign1660_e1316_d_n1;
        locals.var_vpprime_dn2 = assign1660_e1316_d_n2;
        locals.var_vpprime_dn3 = assign1660_e1316_d_n3;

        let (assign1670_e1324, assign1670_e1324_d_n0, assign1670_e1324_d_n1, assign1670_e1324_d_n2, assign1670_e1324_d_n3,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1670_e1321: f64 = (p.p21 * locals.var_vpprime);
        let assign1670_e1322: f64 = (1.0 + assign1670_e1321);
        (assign1670_e1322, (p.p21 * locals.var_vpprime_dn0), (p.p21 * locals.var_vpprime_dn1), (p.p21 * locals.var_vpprime_dn2), (p.p21 * locals.var_vpprime_dn3),)
    } else {
        (locals.var_theta_vp_1, locals.var_theta_vp_1_dn0, locals.var_theta_vp_1_dn1, locals.var_theta_vp_1_dn2, locals.var_theta_vp_1_dn3,)
    }
};
        locals.var_theta_vp_1 = assign1670_e1324;
        locals.var_theta_vp_1_dn0 = assign1670_e1324_d_n0;
        locals.var_theta_vp_1_dn1 = assign1670_e1324_d_n1;
        locals.var_theta_vp_1_dn2 = assign1670_e1324_d_n2;
        locals.var_theta_vp_1_dn3 = assign1670_e1324_d_n3;

        let (assign1680_e1332, assign1680_e1332_d_n0, assign1680_e1332_d_n1, assign1680_e1332_d_n2, assign1680_e1332_d_n3,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1680_e1329: f64 = (locals.var_leq * locals.var_theta_vp_1);
        let assign1680_e1330: f64 = (locals.var_kp_weff / assign1680_e1329);
        (assign1680_e1330, (-((locals.var_kp_weff * ((locals.var_leq_dn0 * locals.var_theta_vp_1) + (locals.var_leq * locals.var_theta_vp_1_dn0))) / (assign1680_e1329 * assign1680_e1329))), (-((locals.var_kp_weff * ((locals.var_leq_dn1 * locals.var_theta_vp_1) + (locals.var_leq * locals.var_theta_vp_1_dn1))) / (assign1680_e1329 * assign1680_e1329))), (-((locals.var_kp_weff * ((locals.var_leq_dn2 * locals.var_theta_vp_1) + (locals.var_leq * locals.var_theta_vp_1_dn2))) / (assign1680_e1329 * assign1680_e1329))), (-((locals.var_kp_weff * ((locals.var_leq_dn3 * locals.var_theta_vp_1) + (locals.var_leq * locals.var_theta_vp_1_dn3))) / (assign1680_e1329 * assign1680_e1329))),)
    } else {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn1, locals.var_beta_dn2, locals.var_beta_dn3,)
    }
};
        locals.var_beta = assign1680_e1332;
        locals.var_beta_dn0 = assign1680_e1332_d_n0;
        locals.var_beta_dn1 = assign1680_e1332_d_n1;
        locals.var_beta_dn2 = assign1680_e1332_d_n2;
        locals.var_beta_dn3 = assign1680_e1332_d_n3;

        let assign1690_e1336: f64 = (locals.var_eta_qi * locals.var_qi);
        let assign1690_e1337: f64 = (locals.var_qb + assign1690_e1336);
        let assign1690_e1339: f64 = if assign1690_e1337 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1690_e1339;

        let (assign1700_e1354, assign1700_e1354_d_n0, assign1700_e1354_d_n1, assign1700_e1354_d_n2, assign1700_e1354_d_n3,) = {
    if ((locals.var_guard16 == 0.0) && (locals.var_guard17 != 0.0)) {
        let assign1700_e1349: f64 = (locals.var_eta_qi * locals.var_qi);
        let assign1700_e1350: f64 = (locals.var_qb + assign1700_e1349);
        let assign1700_e1351: f64 = (locals.var_t0 * assign1700_e1350);
        let assign1700_e1352: f64 = (1.0 + assign1700_e1351);
        (assign1700_e1352, (locals.var_t0 * (locals.var_qb_dn0 + (locals.var_eta_qi * locals.var_qi_dn0))), (locals.var_t0 * (locals.var_qb_dn1 + (locals.var_eta_qi * locals.var_qi_dn1))), (locals.var_t0 * (locals.var_qb_dn2 + (locals.var_eta_qi * locals.var_qi_dn2))), (locals.var_t0 * (locals.var_qb_dn3 + (locals.var_eta_qi * locals.var_qi_dn3))),)
    } else {
        (locals.var_e0_q_1, locals.var_e0_q_1_dn0, locals.var_e0_q_1_dn1, locals.var_e0_q_1_dn2, locals.var_e0_q_1_dn3,)
    }
};
        locals.var_e0_q_1 = assign1700_e1354;
        locals.var_e0_q_1_dn0 = assign1700_e1354_d_n0;
        locals.var_e0_q_1_dn1 = assign1700_e1354_d_n1;
        locals.var_e0_q_1_dn2 = assign1700_e1354_d_n2;
        locals.var_e0_q_1_dn3 = assign1700_e1354_d_n3;

        let (assign1710_e1370, assign1710_e1370_d_n0, assign1710_e1370_d_n1, assign1710_e1370_d_n2, assign1710_e1370_d_n3,) = {
    if ((locals.var_guard16 == 0.0) && (locals.var_guard17 == 0.0)) {
        let assign1710_e1365: f64 = (locals.var_eta_qi * locals.var_qi);
        let assign1710_e1366: f64 = (locals.var_qb + assign1710_e1365);
        let assign1710_e1367: f64 = (locals.var_t0 * assign1710_e1366);
        let assign1710_e1368: f64 = (1.0 - assign1710_e1367);
        (assign1710_e1368, (-(locals.var_t0 * (locals.var_qb_dn0 + (locals.var_eta_qi * locals.var_qi_dn0)))), (-(locals.var_t0 * (locals.var_qb_dn1 + (locals.var_eta_qi * locals.var_qi_dn1)))), (-(locals.var_t0 * (locals.var_qb_dn2 + (locals.var_eta_qi * locals.var_qi_dn2)))), (-(locals.var_t0 * (locals.var_qb_dn3 + (locals.var_eta_qi * locals.var_qi_dn3)))),)
    } else {
        (locals.var_e0_q_1, locals.var_e0_q_1_dn0, locals.var_e0_q_1_dn1, locals.var_e0_q_1_dn2, locals.var_e0_q_1_dn3,)
    }
};
        locals.var_e0_q_1 = assign1710_e1370;
        locals.var_e0_q_1_dn0 = assign1710_e1370_d_n0;
        locals.var_e0_q_1_dn1 = assign1710_e1370_d_n1;
        locals.var_e0_q_1_dn2 = assign1710_e1370_d_n2;
        locals.var_e0_q_1_dn3 = assign1710_e1370_d_n3;

        let (assign1720_e1379, assign1720_e1379_d_n0, assign1720_e1379_d_n1, assign1720_e1379_d_n2, assign1720_e1379_d_n3,) = {
    if (locals.var_guard16 == 0.0) {
        let assign1720_e1376: f64 = (locals.var_t0 * locals.var_gamma_sqrt_phi);
        let assign1720_e1377: f64 = (1.0 + assign1720_e1376);
        (assign1720_e1377, (locals.var_t0 * locals.var_gamma_sqrt_phi_dn0), (locals.var_t0 * locals.var_gamma_sqrt_phi_dn1), (locals.var_t0 * locals.var_gamma_sqrt_phi_dn2), (locals.var_t0 * locals.var_gamma_sqrt_phi_dn3),)
    } else {
        (locals.var_t0_gamma_1, locals.var_t0_gamma_1_dn0, locals.var_t0_gamma_1_dn1, locals.var_t0_gamma_1_dn2, locals.var_t0_gamma_1_dn3,)
    }
};
        locals.var_t0_gamma_1 = assign1720_e1379;
        locals.var_t0_gamma_1_dn0 = assign1720_e1379_d_n0;
        locals.var_t0_gamma_1_dn1 = assign1720_e1379_d_n1;
        locals.var_t0_gamma_1_dn2 = assign1720_e1379_d_n2;
        locals.var_t0_gamma_1_dn3 = assign1720_e1379_d_n3;

        let (assign1730_e1390, assign1730_e1390_d_n0, assign1730_e1390_d_n1, assign1730_e1390_d_n2, assign1730_e1390_d_n3,) = {
    if (locals.var_guard16 == 0.0) {
        let assign1730_e1384: f64 = (locals.var_kp_weff * locals.var_t0_gamma_1);
        let assign1730_e1387: f64 = (locals.var_leq * locals.var_e0_q_1);
        let assign1730_e1388: f64 = (assign1730_e1384 / assign1730_e1387);
        (assign1730_e1388, ((((locals.var_kp_weff * locals.var_t0_gamma_1_dn0) * assign1730_e1387) - (assign1730_e1384 * ((locals.var_leq_dn0 * locals.var_e0_q_1) + (locals.var_leq * locals.var_e0_q_1_dn0)))) / (assign1730_e1387 * assign1730_e1387)), ((((locals.var_kp_weff * locals.var_t0_gamma_1_dn1) * assign1730_e1387) - (assign1730_e1384 * ((locals.var_leq_dn1 * locals.var_e0_q_1) + (locals.var_leq * locals.var_e0_q_1_dn1)))) / (assign1730_e1387 * assign1730_e1387)), ((((locals.var_kp_weff * locals.var_t0_gamma_1_dn2) * assign1730_e1387) - (assign1730_e1384 * ((locals.var_leq_dn2 * locals.var_e0_q_1) + (locals.var_leq * locals.var_e0_q_1_dn2)))) / (assign1730_e1387 * assign1730_e1387)), ((((locals.var_kp_weff * locals.var_t0_gamma_1_dn3) * assign1730_e1387) - (assign1730_e1384 * ((locals.var_leq_dn3 * locals.var_e0_q_1) + (locals.var_leq * locals.var_e0_q_1_dn3)))) / (assign1730_e1387 * assign1730_e1387)),)
    } else {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn1, locals.var_beta_dn2, locals.var_beta_dn3,)
    }
};
        locals.var_beta = assign1730_e1390;
        locals.var_beta_dn0 = assign1730_e1390_d_n0;
        locals.var_beta_dn1 = assign1730_e1390_d_n1;
        locals.var_beta_dn2 = assign1730_e1390_d_n2;
        locals.var_beta_dn3 = assign1730_e1390_d_n3;

        let assign1740_e1393: f64 = (locals.var_phi_t + locals.var_vp);
        let assign1740_e1395: f64 = (assign1740_e1393 + locals.var_vt_4);
        let assign1740_e1396: f64 = (assign1740_e1395).sqrt();
        locals.var_sqrt_phi_vp = assign1740_e1396;
        locals.var_sqrt_phi_vp_dn0 = ((locals.var_phi_t_dn0 + locals.var_vp_dn0) / (2.0 * assign1740_e1396));
        locals.var_sqrt_phi_vp_dn1 = ((locals.var_phi_t_dn1 + locals.var_vp_dn1) / (2.0 * assign1740_e1396));
        locals.var_sqrt_phi_vp_dn2 = ((locals.var_phi_t_dn2 + locals.var_vp_dn2) / (2.0 * assign1740_e1396));
        locals.var_sqrt_phi_vp_dn3 = ((locals.var_phi_t_dn3 + locals.var_vp_dn3) / (2.0 * assign1740_e1396));

    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign1750_e1401: f64 = (2.0 * locals.var_sqrt_phi_vp);
        let assign1750_e1402: f64 = (locals.var_gamma_s / assign1750_e1401);
        let assign1750_e1403: f64 = (1.0 + assign1750_e1402);
        locals.var_n = assign1750_e1403;
        locals.var_n_dn0 = (-((locals.var_gamma_s * (2.0 * locals.var_sqrt_phi_vp_dn0)) / (assign1750_e1401 * assign1750_e1401)));
        locals.var_n_dn1 = (-((locals.var_gamma_s * (2.0 * locals.var_sqrt_phi_vp_dn1)) / (assign1750_e1401 * assign1750_e1401)));
        locals.var_n_dn2 = (-((locals.var_gamma_s * (2.0 * locals.var_sqrt_phi_vp_dn2)) / (assign1750_e1401 * assign1750_e1401)));
        locals.var_n_dn3 = (-((locals.var_gamma_s * (2.0 * locals.var_sqrt_phi_vp_dn3)) / (assign1750_e1401 * assign1750_e1401)));

        let assign1760_e1406: f64 = (locals.var_if_ - locals.var_irprime);
        locals.var_if_ir = assign1760_e1406;
        locals.var_if_ir_dn0 = (locals.var_if__dn0 - locals.var_irprime_dn0);
        locals.var_if_ir_dn1 = (locals.var_if__dn1 - locals.var_irprime_dn1);
        locals.var_if_ir_dn2 = (locals.var_if__dn2 - locals.var_irprime_dn2);
        locals.var_if_ir_dn3 = (locals.var_if__dn3 - locals.var_irprime_dn3);

        let assign1770_e1409: f64 = (locals.var_vt_vt_2 * locals.var_n);
        let assign1770_e1411: f64 = (assign1770_e1409 * locals.var_beta);
        locals.var_ispec = assign1770_e1411;
        locals.var_ispec_dn0 = (((locals.var_vt_vt_2 * locals.var_n_dn0) * locals.var_beta) + (assign1770_e1409 * locals.var_beta_dn0));
        locals.var_ispec_dn1 = (((locals.var_vt_vt_2 * locals.var_n_dn1) * locals.var_beta) + (assign1770_e1409 * locals.var_beta_dn1));
        locals.var_ispec_dn2 = (((locals.var_vt_vt_2 * locals.var_n_dn2) * locals.var_beta) + (assign1770_e1409 * locals.var_beta_dn2));
        locals.var_ispec_dn3 = (((locals.var_vt_vt_2 * locals.var_n_dn3) * locals.var_beta) + (assign1770_e1409 * locals.var_beta_dn3));

        let assign1820_e1436: f64 = (locals.var_sqrt_gammastar + locals.var_sqrt_gammastar);
        let assign1820_e1437: f64 = (locals.var_gammaprime / assign1820_e1436);
        locals.var_tmp1 = assign1820_e1437;
        locals.var_tmp1_dn0 = (((locals.var_gammaprime_dn0 * assign1820_e1436) - (locals.var_gammaprime * (locals.var_sqrt_gammastar_dn0 + locals.var_sqrt_gammastar_dn0))) / (assign1820_e1436 * assign1820_e1436));
        locals.var_tmp1_dn1 = (((locals.var_gammaprime_dn1 * assign1820_e1436) - (locals.var_gammaprime * (locals.var_sqrt_gammastar_dn1 + locals.var_sqrt_gammastar_dn1))) / (assign1820_e1436 * assign1820_e1436));
        locals.var_tmp1_dn2 = (((locals.var_gammaprime_dn2 * assign1820_e1436) - (locals.var_gammaprime * (locals.var_sqrt_gammastar_dn2 + locals.var_sqrt_gammastar_dn2))) / (assign1820_e1436 * assign1820_e1436));
        locals.var_tmp1_dn3 = (((locals.var_gammaprime_dn3 * assign1820_e1436) - (locals.var_gammaprime * (locals.var_sqrt_gammastar_dn3 + locals.var_sqrt_gammastar_dn3))) / (assign1820_e1436 * assign1820_e1436));

        let assign1830_e1440: f64 = (locals.var_vgprime / locals.var_sqrt_vgstar);
        locals.var_tmp2 = assign1830_e1440;
        locals.var_tmp2_dn0 = (((locals.var_vgprime_dn0 * locals.var_sqrt_vgstar) - (locals.var_vgprime * locals.var_sqrt_vgstar_dn0)) / (locals.var_sqrt_vgstar * locals.var_sqrt_vgstar));
        locals.var_tmp2_dn1 = (((locals.var_vgprime_dn1 * locals.var_sqrt_vgstar) - (locals.var_vgprime * locals.var_sqrt_vgstar_dn1)) / (locals.var_sqrt_vgstar * locals.var_sqrt_vgstar));
        locals.var_tmp2_dn2 = (((locals.var_vgprime_dn2 * locals.var_sqrt_vgstar) - (locals.var_vgprime * locals.var_sqrt_vgstar_dn2)) / (locals.var_sqrt_vgstar * locals.var_sqrt_vgstar));
        locals.var_tmp2_dn3 = (((locals.var_vgprime_dn3 * locals.var_sqrt_vgstar) - (locals.var_vgprime * locals.var_sqrt_vgstar_dn3)) / (locals.var_sqrt_vgstar * locals.var_sqrt_vgstar));

        let assign1840_e1442: f64 = (-locals.var_leta_l);
        let assign1840_e1444: f64 = (assign1840_e1442 * locals.var_tmp1);
        let assign1840_e1446: f64 = (assign1840_e1444 * locals.var_sqrt_phi_vd);
        let assign1840_e1448: f64 = (assign1840_e1446 / locals.var_sqrt_phi_vd_vt);
        locals.var_dgammaprime_dvd = assign1840_e1448;
        locals.var_dgammaprime_dvd_dn0 = ((((((assign1840_e1442 * locals.var_tmp1_dn0) * locals.var_sqrt_phi_vd) + (assign1840_e1444 * locals.var_sqrt_phi_vd_dn0)) * locals.var_sqrt_phi_vd_vt) - (assign1840_e1446 * locals.var_sqrt_phi_vd_vt_dn0)) / (locals.var_sqrt_phi_vd_vt * locals.var_sqrt_phi_vd_vt));
        locals.var_dgammaprime_dvd_dn1 = ((((((assign1840_e1442 * locals.var_tmp1_dn1) * locals.var_sqrt_phi_vd) + (assign1840_e1444 * locals.var_sqrt_phi_vd_dn1)) * locals.var_sqrt_phi_vd_vt) - (assign1840_e1446 * locals.var_sqrt_phi_vd_vt_dn1)) / (locals.var_sqrt_phi_vd_vt * locals.var_sqrt_phi_vd_vt));
        locals.var_dgammaprime_dvd_dn2 = ((((((assign1840_e1442 * locals.var_tmp1_dn2) * locals.var_sqrt_phi_vd) + (assign1840_e1444 * locals.var_sqrt_phi_vd_dn2)) * locals.var_sqrt_phi_vd_vt) - (assign1840_e1446 * locals.var_sqrt_phi_vd_vt_dn2)) / (locals.var_sqrt_phi_vd_vt * locals.var_sqrt_phi_vd_vt));
        locals.var_dgammaprime_dvd_dn3 = ((((((assign1840_e1442 * locals.var_tmp1_dn3) * locals.var_sqrt_phi_vd) + (assign1840_e1444 * locals.var_sqrt_phi_vd_dn3)) * locals.var_sqrt_phi_vd_vt) - (assign1840_e1446 * locals.var_sqrt_phi_vd_vt_dn3)) / (locals.var_sqrt_phi_vd_vt * locals.var_sqrt_phi_vd_vt));

        let assign1850_e1450: f64 = (-locals.var_leta_l);
        let assign1850_e1452: f64 = (assign1850_e1450 * locals.var_tmp1);
        let assign1850_e1454: f64 = (assign1850_e1452 * locals.var_sqrt_phi_vs);
        let assign1850_e1456: f64 = (assign1850_e1454 / locals.var_sqrt_phi_vs_vt);
        locals.var_dgammaprime_dvs = assign1850_e1456;
        locals.var_dgammaprime_dvs_dn0 = ((((((assign1850_e1450 * locals.var_tmp1_dn0) * locals.var_sqrt_phi_vs) + (assign1850_e1452 * locals.var_sqrt_phi_vs_dn0)) * locals.var_sqrt_phi_vs_vt) - (assign1850_e1454 * locals.var_sqrt_phi_vs_vt_dn0)) / (locals.var_sqrt_phi_vs_vt * locals.var_sqrt_phi_vs_vt));
        locals.var_dgammaprime_dvs_dn1 = ((((((assign1850_e1450 * locals.var_tmp1_dn1) * locals.var_sqrt_phi_vs) + (assign1850_e1452 * locals.var_sqrt_phi_vs_dn1)) * locals.var_sqrt_phi_vs_vt) - (assign1850_e1454 * locals.var_sqrt_phi_vs_vt_dn1)) / (locals.var_sqrt_phi_vs_vt * locals.var_sqrt_phi_vs_vt));
        locals.var_dgammaprime_dvs_dn2 = ((((((assign1850_e1450 * locals.var_tmp1_dn2) * locals.var_sqrt_phi_vs) + (assign1850_e1452 * locals.var_sqrt_phi_vs_dn2)) * locals.var_sqrt_phi_vs_vt) - (assign1850_e1454 * locals.var_sqrt_phi_vs_vt_dn2)) / (locals.var_sqrt_phi_vs_vt * locals.var_sqrt_phi_vs_vt));
        locals.var_dgammaprime_dvs_dn3 = ((((((assign1850_e1450 * locals.var_tmp1_dn3) * locals.var_sqrt_phi_vs) + (assign1850_e1452 * locals.var_sqrt_phi_vs_dn3)) * locals.var_sqrt_phi_vs_vt) - (assign1850_e1454 * locals.var_sqrt_phi_vs_vt_dn3)) / (locals.var_sqrt_phi_vs_vt * locals.var_sqrt_phi_vs_vt));

        let assign1870_e1474: f64 = (locals.var_vp + locals.var_phi_t);
        let assign1870_e1476: f64 = (assign1870_e1474 / locals.var_big_sqrt_vp);
        locals.var_tmp3 = assign1870_e1476;
        locals.var_tmp3_dn0 = ((((locals.var_vp_dn0 + locals.var_phi_t_dn0) * locals.var_big_sqrt_vp) - (assign1870_e1474 * locals.var_big_sqrt_vp_dn0)) / (locals.var_big_sqrt_vp * locals.var_big_sqrt_vp));
        locals.var_tmp3_dn1 = ((((locals.var_vp_dn1 + locals.var_phi_t_dn1) * locals.var_big_sqrt_vp) - (assign1870_e1474 * locals.var_big_sqrt_vp_dn1)) / (locals.var_big_sqrt_vp * locals.var_big_sqrt_vp));
        locals.var_tmp3_dn2 = ((((locals.var_vp_dn2 + locals.var_phi_t_dn2) * locals.var_big_sqrt_vp) - (assign1870_e1474 * locals.var_big_sqrt_vp_dn2)) / (locals.var_big_sqrt_vp * locals.var_big_sqrt_vp));
        locals.var_tmp3_dn3 = ((((locals.var_vp_dn3 + locals.var_phi_t_dn3) * locals.var_big_sqrt_vp) - (assign1870_e1474 * locals.var_big_sqrt_vp_dn3)) / (locals.var_big_sqrt_vp * locals.var_big_sqrt_vp));

        let assign1880_e1478: f64 = (-locals.var_tmp3);
        let assign1880_e1480: f64 = (assign1880_e1478 * locals.var_dgammaprime_dvd);
        locals.var_dvp_dvd = assign1880_e1480;
        locals.var_dvp_dvd_dn0 = (((-locals.var_tmp3_dn0) * locals.var_dgammaprime_dvd) + (assign1880_e1478 * locals.var_dgammaprime_dvd_dn0));
        locals.var_dvp_dvd_dn1 = (((-locals.var_tmp3_dn1) * locals.var_dgammaprime_dvd) + (assign1880_e1478 * locals.var_dgammaprime_dvd_dn1));
        locals.var_dvp_dvd_dn2 = (((-locals.var_tmp3_dn2) * locals.var_dgammaprime_dvd) + (assign1880_e1478 * locals.var_dgammaprime_dvd_dn2));
        locals.var_dvp_dvd_dn3 = (((-locals.var_tmp3_dn3) * locals.var_dgammaprime_dvd) + (assign1880_e1478 * locals.var_dgammaprime_dvd_dn3));

        let assign1890_e1482: f64 = (-locals.var_tmp3);
        let assign1890_e1484: f64 = (assign1890_e1482 * locals.var_dgammaprime_dvs);
        locals.var_dvp_dvs = assign1890_e1484;
        locals.var_dvp_dvs_dn0 = (((-locals.var_tmp3_dn0) * locals.var_dgammaprime_dvs) + (assign1890_e1482 * locals.var_dgammaprime_dvs_dn0));
        locals.var_dvp_dvs_dn1 = (((-locals.var_tmp3_dn1) * locals.var_dgammaprime_dvs) + (assign1890_e1482 * locals.var_dgammaprime_dvs_dn1));
        locals.var_dvp_dvs_dn2 = (((-locals.var_tmp3_dn2) * locals.var_dgammaprime_dvs) + (assign1890_e1482 * locals.var_dgammaprime_dvs_dn2));
        locals.var_dvp_dvs_dn3 = (((-locals.var_tmp3_dn3) * locals.var_dgammaprime_dvs) + (assign1890_e1482 * locals.var_dgammaprime_dvs_dn3));

        let assign1910_e1501: f64 = (locals.var_dif_dv * locals.var_inv_vt);
        locals.var_tmp1 = assign1910_e1501;
        locals.var_tmp1_dn0 = (locals.var_dif_dv_dn0 * locals.var_inv_vt);
        locals.var_tmp1_dn1 = (locals.var_dif_dv_dn1 * locals.var_inv_vt);
        locals.var_tmp1_dn2 = (locals.var_dif_dv_dn2 * locals.var_inv_vt);
        locals.var_tmp1_dn3 = (locals.var_dif_dv_dn3 * locals.var_inv_vt);

        let assign1920_e1504: f64 = (locals.var_tmp1 * locals.var_dvp_dvd);
        locals.var_dif_dvd = assign1920_e1504;
        locals.var_dif_dvd_dn0 = ((locals.var_tmp1_dn0 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn0));
        locals.var_dif_dvd_dn1 = ((locals.var_tmp1_dn1 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn1));
        locals.var_dif_dvd_dn2 = ((locals.var_tmp1_dn2 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn2));
        locals.var_dif_dvd_dn3 = ((locals.var_tmp1_dn3 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn3));

        let assign1930_e1508: f64 = (locals.var_dvp_dvs - 1.0);
        let assign1930_e1509: f64 = (locals.var_tmp1 * assign1930_e1508);
        locals.var_dif_dvs = assign1930_e1509;
        locals.var_dif_dvs_dn0 = ((locals.var_tmp1_dn0 * assign1930_e1508) + (locals.var_tmp1 * locals.var_dvp_dvs_dn0));
        locals.var_dif_dvs_dn1 = ((locals.var_tmp1_dn1 * assign1930_e1508) + (locals.var_tmp1 * locals.var_dvp_dvs_dn1));
        locals.var_dif_dvs_dn2 = ((locals.var_tmp1_dn2 * assign1930_e1508) + (locals.var_tmp1 * locals.var_dvp_dvs_dn2));
        locals.var_dif_dvs_dn3 = ((locals.var_tmp1_dn3 * assign1930_e1508) + (locals.var_tmp1 * locals.var_dvp_dvs_dn3));

        let assign1950_e1516: f64 = (4.0 * locals.var_vdss_sqrt);
        let assign1950_e1518: f64 = (assign1950_e1516 * locals.var_sqrt_if);
        let assign1950_e1519: f64 = (locals.var_vt / assign1950_e1518);
        locals.var_tmp1 = assign1950_e1519;
        locals.var_tmp1_dn0 = (-((locals.var_vt * (((4.0 * locals.var_vdss_sqrt_dn0) * locals.var_sqrt_if) + (assign1950_e1516 * locals.var_sqrt_if_dn0))) / (assign1950_e1518 * assign1950_e1518)));
        locals.var_tmp1_dn1 = (-((locals.var_vt * (((4.0 * locals.var_vdss_sqrt_dn1) * locals.var_sqrt_if) + (assign1950_e1516 * locals.var_sqrt_if_dn1))) / (assign1950_e1518 * assign1950_e1518)));
        locals.var_tmp1_dn2 = (-((locals.var_vt * (((4.0 * locals.var_vdss_sqrt_dn2) * locals.var_sqrt_if) + (assign1950_e1516 * locals.var_sqrt_if_dn2))) / (assign1950_e1518 * assign1950_e1518)));
        locals.var_tmp1_dn3 = (-((locals.var_vt * (((4.0 * locals.var_vdss_sqrt_dn3) * locals.var_sqrt_if) + (assign1950_e1516 * locals.var_sqrt_if_dn3))) / (assign1950_e1518 * assign1950_e1518)));

        let assign1960_e1522: f64 = (locals.var_tmp1 * locals.var_dif_dvd);
        locals.var_dvdss_dvd = assign1960_e1522;
        locals.var_dvdss_dvd_dn0 = ((locals.var_tmp1_dn0 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn0));
        locals.var_dvdss_dvd_dn1 = ((locals.var_tmp1_dn1 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn1));
        locals.var_dvdss_dvd_dn2 = ((locals.var_tmp1_dn2 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn2));
        locals.var_dvdss_dvd_dn3 = ((locals.var_tmp1_dn3 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn3));

        let assign1970_e1525: f64 = (locals.var_tmp1 * locals.var_dif_dvs);
        locals.var_dvdss_dvs = assign1970_e1525;
        locals.var_dvdss_dvs_dn0 = ((locals.var_tmp1_dn0 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn0));
        locals.var_dvdss_dvs_dn1 = ((locals.var_tmp1_dn1 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn1));
        locals.var_dvdss_dvs_dn2 = ((locals.var_tmp1_dn2 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn2));
        locals.var_dvdss_dvs_dn3 = ((locals.var_tmp1_dn3 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn3));

        let assign1990_e1531: f64 = (locals.var_vt_4 + locals.var_vt_4);
        let assign1990_e1533: f64 = (assign1990_e1531 * p.p25);
        locals.var_tmp1 = assign1990_e1533;
        locals.var_tmp1_dn0 = 0.0;
        locals.var_tmp1_dn1 = 0.0;
        locals.var_tmp1_dn2 = 0.0;
        locals.var_tmp1_dn3 = 0.0;

        let assign2000_e1537: f64 = (locals.var_sqrt_if + locals.var_sqrt_if);
        let assign2000_e1538: f64 = (locals.var_vt / assign2000_e1537);
        locals.var_tmp2 = assign2000_e1538;
        locals.var_tmp2_dn0 = (-((locals.var_vt * (locals.var_sqrt_if_dn0 + locals.var_sqrt_if_dn0)) / (assign2000_e1537 * assign2000_e1537)));
        locals.var_tmp2_dn1 = (-((locals.var_vt * (locals.var_sqrt_if_dn1 + locals.var_sqrt_if_dn1)) / (assign2000_e1537 * assign2000_e1537)));
        locals.var_tmp2_dn2 = (-((locals.var_vt * (locals.var_sqrt_if_dn2 + locals.var_sqrt_if_dn2)) / (assign2000_e1537 * assign2000_e1537)));
        locals.var_tmp2_dn3 = (-((locals.var_vt * (locals.var_sqrt_if_dn3 + locals.var_sqrt_if_dn3)) / (assign2000_e1537 * assign2000_e1537)));

        let assign2010_e1542: f64 = (locals.var_dif_dvd * locals.var_tmp2);
        let assign2010_e1544: f64 = (assign2010_e1542 - locals.var_dvdss_dvd);
        let assign2010_e1545: f64 = (locals.var_tmp1 * assign2010_e1544);
        locals.var_ddeltav_dvd = assign2010_e1545;
        locals.var_ddeltav_dvd_dn0 = ((locals.var_tmp1_dn0 * assign2010_e1544) + (locals.var_tmp1 * (((locals.var_dif_dvd_dn0 * locals.var_tmp2) + (locals.var_dif_dvd * locals.var_tmp2_dn0)) - locals.var_dvdss_dvd_dn0)));
        locals.var_ddeltav_dvd_dn1 = ((locals.var_tmp1_dn1 * assign2010_e1544) + (locals.var_tmp1 * (((locals.var_dif_dvd_dn1 * locals.var_tmp2) + (locals.var_dif_dvd * locals.var_tmp2_dn1)) - locals.var_dvdss_dvd_dn1)));
        locals.var_ddeltav_dvd_dn2 = ((locals.var_tmp1_dn2 * assign2010_e1544) + (locals.var_tmp1 * (((locals.var_dif_dvd_dn2 * locals.var_tmp2) + (locals.var_dif_dvd * locals.var_tmp2_dn2)) - locals.var_dvdss_dvd_dn2)));
        locals.var_ddeltav_dvd_dn3 = ((locals.var_tmp1_dn3 * assign2010_e1544) + (locals.var_tmp1 * (((locals.var_dif_dvd_dn3 * locals.var_tmp2) + (locals.var_dif_dvd * locals.var_tmp2_dn3)) - locals.var_dvdss_dvd_dn3)));

        let assign2020_e1549: f64 = (locals.var_dif_dvs * locals.var_tmp2);
        let assign2020_e1551: f64 = (assign2020_e1549 - locals.var_dvdss_dvs);
        let assign2020_e1552: f64 = (locals.var_tmp1 * assign2020_e1551);
        locals.var_ddeltav_dvs = assign2020_e1552;
        locals.var_ddeltav_dvs_dn0 = ((locals.var_tmp1_dn0 * assign2020_e1551) + (locals.var_tmp1 * (((locals.var_dif_dvs_dn0 * locals.var_tmp2) + (locals.var_dif_dvs * locals.var_tmp2_dn0)) - locals.var_dvdss_dvs_dn0)));
        locals.var_ddeltav_dvs_dn1 = ((locals.var_tmp1_dn1 * assign2020_e1551) + (locals.var_tmp1 * (((locals.var_dif_dvs_dn1 * locals.var_tmp2) + (locals.var_dif_dvs * locals.var_tmp2_dn1)) - locals.var_dvdss_dvs_dn1)));
        locals.var_ddeltav_dvs_dn2 = ((locals.var_tmp1_dn2 * assign2020_e1551) + (locals.var_tmp1 * (((locals.var_dif_dvs_dn2 * locals.var_tmp2) + (locals.var_dif_dvs * locals.var_tmp2_dn2)) - locals.var_dvdss_dvs_dn2)));
        locals.var_ddeltav_dvs_dn3 = ((locals.var_tmp1_dn3 * assign2020_e1551) + (locals.var_tmp1 * (((locals.var_dif_dvs_dn3 * locals.var_tmp2) + (locals.var_dif_dvs * locals.var_tmp2_dn3)) - locals.var_dvdss_dvs_dn3)));

        let assign2040_e1562: f64 = (1.0 / locals.var_sqrt_vdss_deltav);
        locals.var_tmp1 = assign2040_e1562;
        locals.var_tmp1_dn0 = (-(locals.var_sqrt_vdss_deltav_dn0 / (locals.var_sqrt_vdss_deltav * locals.var_sqrt_vdss_deltav)));
        locals.var_tmp1_dn1 = (-(locals.var_sqrt_vdss_deltav_dn1 / (locals.var_sqrt_vdss_deltav * locals.var_sqrt_vdss_deltav)));
        locals.var_tmp1_dn2 = (-(locals.var_sqrt_vdss_deltav_dn2 / (locals.var_sqrt_vdss_deltav * locals.var_sqrt_vdss_deltav)));
        locals.var_tmp1_dn3 = (-(locals.var_sqrt_vdss_deltav_dn3 / (locals.var_sqrt_vdss_deltav * locals.var_sqrt_vdss_deltav)));

        let assign2050_e1565: f64 = (1.0 / locals.var_sqrt_vds_vdss_deltav);
        locals.var_tmp2 = assign2050_e1565;
        locals.var_tmp2_dn0 = (-(locals.var_sqrt_vds_vdss_deltav_dn0 / (locals.var_sqrt_vds_vdss_deltav * locals.var_sqrt_vds_vdss_deltav)));
        locals.var_tmp2_dn1 = (-(locals.var_sqrt_vds_vdss_deltav_dn1 / (locals.var_sqrt_vds_vdss_deltav * locals.var_sqrt_vds_vdss_deltav)));
        locals.var_tmp2_dn2 = (-(locals.var_sqrt_vds_vdss_deltav_dn2 / (locals.var_sqrt_vds_vdss_deltav * locals.var_sqrt_vds_vdss_deltav)));
        locals.var_tmp2_dn3 = (-(locals.var_sqrt_vds_vdss_deltav_dn3 / (locals.var_sqrt_vds_vdss_deltav * locals.var_sqrt_vds_vdss_deltav)));

        let assign2060_e1568: f64 = (locals.var_vds - locals.var_vdss);
        locals.var_tmp3 = assign2060_e1568;
        locals.var_tmp3_dn0 = (locals.var_vds_dn0 - locals.var_vdss_dn0);
        locals.var_tmp3_dn1 = (-locals.var_vdss_dn1);
        locals.var_tmp3_dn2 = (locals.var_vds_dn2 - locals.var_vdss_dn2);
        locals.var_tmp3_dn3 = (locals.var_vds_dn3 - locals.var_vdss_dn3);

        let assign2070_e1571: f64 = (locals.var_vdss * locals.var_dvdss_dvd);
        let assign2070_e1573: f64 = (assign2070_e1571 + locals.var_ddeltav_dvd);
        let assign2070_e1575: f64 = (assign2070_e1573 * locals.var_tmp1);
        let assign2070_e1579: f64 = (0.5 - locals.var_dvdss_dvd);
        let assign2070_e1580: f64 = (locals.var_tmp3 * assign2070_e1579);
        let assign2070_e1582: f64 = (assign2070_e1580 + locals.var_ddeltav_dvd);
        let assign2070_e1584: f64 = (assign2070_e1582 * locals.var_tmp2);
        let assign2070_e1585: f64 = (assign2070_e1575 - assign2070_e1584);
        locals.var_dvip_dvd = assign2070_e1585;
        locals.var_dvip_dvd_dn0 = ((((((locals.var_vdss_dn0 * locals.var_dvdss_dvd) + (locals.var_vdss * locals.var_dvdss_dvd_dn0)) + locals.var_ddeltav_dvd_dn0) * locals.var_tmp1) + (assign2070_e1573 * locals.var_tmp1_dn0)) - (((((locals.var_tmp3_dn0 * assign2070_e1579) + (locals.var_tmp3 * (-locals.var_dvdss_dvd_dn0))) + locals.var_ddeltav_dvd_dn0) * locals.var_tmp2) + (assign2070_e1582 * locals.var_tmp2_dn0)));
        locals.var_dvip_dvd_dn1 = ((((((locals.var_vdss_dn1 * locals.var_dvdss_dvd) + (locals.var_vdss * locals.var_dvdss_dvd_dn1)) + locals.var_ddeltav_dvd_dn1) * locals.var_tmp1) + (assign2070_e1573 * locals.var_tmp1_dn1)) - (((((locals.var_tmp3_dn1 * assign2070_e1579) + (locals.var_tmp3 * (-locals.var_dvdss_dvd_dn1))) + locals.var_ddeltav_dvd_dn1) * locals.var_tmp2) + (assign2070_e1582 * locals.var_tmp2_dn1)));
        locals.var_dvip_dvd_dn2 = ((((((locals.var_vdss_dn2 * locals.var_dvdss_dvd) + (locals.var_vdss * locals.var_dvdss_dvd_dn2)) + locals.var_ddeltav_dvd_dn2) * locals.var_tmp1) + (assign2070_e1573 * locals.var_tmp1_dn2)) - (((((locals.var_tmp3_dn2 * assign2070_e1579) + (locals.var_tmp3 * (-locals.var_dvdss_dvd_dn2))) + locals.var_ddeltav_dvd_dn2) * locals.var_tmp2) + (assign2070_e1582 * locals.var_tmp2_dn2)));
        locals.var_dvip_dvd_dn3 = ((((((locals.var_vdss_dn3 * locals.var_dvdss_dvd) + (locals.var_vdss * locals.var_dvdss_dvd_dn3)) + locals.var_ddeltav_dvd_dn3) * locals.var_tmp1) + (assign2070_e1573 * locals.var_tmp1_dn3)) - (((((locals.var_tmp3_dn3 * assign2070_e1579) + (locals.var_tmp3 * (-locals.var_dvdss_dvd_dn3))) + locals.var_ddeltav_dvd_dn3) * locals.var_tmp2) + (assign2070_e1582 * locals.var_tmp2_dn3)));

        let assign2080_e1588: f64 = (locals.var_vdss * locals.var_dvdss_dvs);
        let assign2080_e1590: f64 = (assign2080_e1588 + locals.var_ddeltav_dvs);
        let assign2080_e1592: f64 = (assign2080_e1590 * locals.var_tmp1);
        let assign2080_e1595: f64 = (-0.5);
        let assign2080_e1597: f64 = (assign2080_e1595 - locals.var_dvdss_dvs);
        let assign2080_e1598: f64 = (locals.var_tmp3 * assign2080_e1597);
        let assign2080_e1600: f64 = (assign2080_e1598 + locals.var_ddeltav_dvs);
        let assign2080_e1602: f64 = (assign2080_e1600 * locals.var_tmp2);
        let assign2080_e1603: f64 = (assign2080_e1592 - assign2080_e1602);
        locals.var_dvip_dvs = assign2080_e1603;
        locals.var_dvip_dvs_dn0 = ((((((locals.var_vdss_dn0 * locals.var_dvdss_dvs) + (locals.var_vdss * locals.var_dvdss_dvs_dn0)) + locals.var_ddeltav_dvs_dn0) * locals.var_tmp1) + (assign2080_e1590 * locals.var_tmp1_dn0)) - (((((locals.var_tmp3_dn0 * assign2080_e1597) + (locals.var_tmp3 * (-locals.var_dvdss_dvs_dn0))) + locals.var_ddeltav_dvs_dn0) * locals.var_tmp2) + (assign2080_e1600 * locals.var_tmp2_dn0)));
        locals.var_dvip_dvs_dn1 = ((((((locals.var_vdss_dn1 * locals.var_dvdss_dvs) + (locals.var_vdss * locals.var_dvdss_dvs_dn1)) + locals.var_ddeltav_dvs_dn1) * locals.var_tmp1) + (assign2080_e1590 * locals.var_tmp1_dn1)) - (((((locals.var_tmp3_dn1 * assign2080_e1597) + (locals.var_tmp3 * (-locals.var_dvdss_dvs_dn1))) + locals.var_ddeltav_dvs_dn1) * locals.var_tmp2) + (assign2080_e1600 * locals.var_tmp2_dn1)));
        locals.var_dvip_dvs_dn2 = ((((((locals.var_vdss_dn2 * locals.var_dvdss_dvs) + (locals.var_vdss * locals.var_dvdss_dvs_dn2)) + locals.var_ddeltav_dvs_dn2) * locals.var_tmp1) + (assign2080_e1590 * locals.var_tmp1_dn2)) - (((((locals.var_tmp3_dn2 * assign2080_e1597) + (locals.var_tmp3 * (-locals.var_dvdss_dvs_dn2))) + locals.var_ddeltav_dvs_dn2) * locals.var_tmp2) + (assign2080_e1600 * locals.var_tmp2_dn2)));
        locals.var_dvip_dvs_dn3 = ((((((locals.var_vdss_dn3 * locals.var_dvdss_dvs) + (locals.var_vdss * locals.var_dvdss_dvs_dn3)) + locals.var_ddeltav_dvs_dn3) * locals.var_tmp1) + (assign2080_e1590 * locals.var_tmp1_dn3)) - (((((locals.var_tmp3_dn3 * assign2080_e1597) + (locals.var_tmp3 * (-locals.var_dvdss_dvs_dn3))) + locals.var_ddeltav_dvs_dn3) * locals.var_tmp2) + (assign2080_e1600 * locals.var_tmp2_dn3)));

        let assign2100_e1623: f64 = (locals.var_sqrt_if - 1.5);
        let assign2100_e1624: f64 = (locals.var_vt * assign2100_e1623);
        let assign2100_e1627: f64 = (4.0 * locals.var_vdssprime_sqrt);
        let assign2100_e1629: f64 = (assign2100_e1627 * locals.var_if_);
        let assign2100_e1630: f64 = (assign2100_e1624 / assign2100_e1629);
        locals.var_tmp1 = assign2100_e1630;
        locals.var_tmp1_dn0 = ((((locals.var_vt * locals.var_sqrt_if_dn0) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * locals.var_vdssprime_sqrt_dn0) * locals.var_if_) + (assign2100_e1627 * locals.var_if__dn0)))) / (assign2100_e1629 * assign2100_e1629));
        locals.var_tmp1_dn1 = ((((locals.var_vt * locals.var_sqrt_if_dn1) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * locals.var_vdssprime_sqrt_dn1) * locals.var_if_) + (assign2100_e1627 * locals.var_if__dn1)))) / (assign2100_e1629 * assign2100_e1629));
        locals.var_tmp1_dn2 = ((((locals.var_vt * locals.var_sqrt_if_dn2) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * locals.var_vdssprime_sqrt_dn2) * locals.var_if_) + (assign2100_e1627 * locals.var_if__dn2)))) / (assign2100_e1629 * assign2100_e1629));
        locals.var_tmp1_dn3 = ((((locals.var_vt * locals.var_sqrt_if_dn3) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * locals.var_vdssprime_sqrt_dn3) * locals.var_if_) + (assign2100_e1627 * locals.var_if__dn3)))) / (assign2100_e1629 * assign2100_e1629));

        let assign2110_e1633: f64 = (locals.var_tmp1 * locals.var_dif_dvd);
        locals.var_dvdssprime_dvd = assign2110_e1633;
        locals.var_dvdssprime_dvd_dn0 = ((locals.var_tmp1_dn0 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn0));
        locals.var_dvdssprime_dvd_dn1 = ((locals.var_tmp1_dn1 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn1));
        locals.var_dvdssprime_dvd_dn2 = ((locals.var_tmp1_dn2 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn2));
        locals.var_dvdssprime_dvd_dn3 = ((locals.var_tmp1_dn3 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn3));

        let assign2120_e1636: f64 = (locals.var_tmp1 * locals.var_dif_dvs);
        locals.var_dvdssprime_dvs = assign2120_e1636;
        locals.var_dvdssprime_dvs_dn0 = ((locals.var_tmp1_dn0 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn0));
        locals.var_dvdssprime_dvs_dn1 = ((locals.var_tmp1_dn1 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn1));
        locals.var_dvdssprime_dvs_dn2 = ((locals.var_tmp1_dn2 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn2));
        locals.var_dvdssprime_dvs_dn3 = ((locals.var_tmp1_dn3 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn3));

        let assign2140_e1642: f64 = (locals.var_dirprime_dv * locals.var_inv_vt);
        locals.var_tmp1 = assign2140_e1642;
        locals.var_tmp1_dn0 = (locals.var_dirprime_dv_dn0 * locals.var_inv_vt);
        locals.var_tmp1_dn1 = (locals.var_dirprime_dv_dn1 * locals.var_inv_vt);
        locals.var_tmp1_dn2 = (locals.var_dirprime_dv_dn2 * locals.var_inv_vt);
        locals.var_tmp1_dn3 = (locals.var_dirprime_dv_dn3 * locals.var_inv_vt);

        let assign2150_e1645: f64 = (1.0 / locals.var_sqrt_vdssprime_deltav);
        locals.var_tmp2 = assign2150_e1645;
        locals.var_tmp2_dn0 = (-(locals.var_sqrt_vdssprime_deltav_dn0 / (locals.var_sqrt_vdssprime_deltav * locals.var_sqrt_vdssprime_deltav)));
        locals.var_tmp2_dn1 = (-(locals.var_sqrt_vdssprime_deltav_dn1 / (locals.var_sqrt_vdssprime_deltav * locals.var_sqrt_vdssprime_deltav)));
        locals.var_tmp2_dn2 = (-(locals.var_sqrt_vdssprime_deltav_dn2 / (locals.var_sqrt_vdssprime_deltav * locals.var_sqrt_vdssprime_deltav)));
        locals.var_tmp2_dn3 = (-(locals.var_sqrt_vdssprime_deltav_dn3 / (locals.var_sqrt_vdssprime_deltav * locals.var_sqrt_vdssprime_deltav)));

        let assign2160_e1648: f64 = (1.0 / locals.var_sqrt_vds_vdssprime_deltav);
        locals.var_tmp3 = assign2160_e1648;
        locals.var_tmp3_dn0 = (-(locals.var_sqrt_vds_vdssprime_deltav_dn0 / (locals.var_sqrt_vds_vdssprime_deltav * locals.var_sqrt_vds_vdssprime_deltav)));
        locals.var_tmp3_dn1 = (-(locals.var_sqrt_vds_vdssprime_deltav_dn1 / (locals.var_sqrt_vds_vdssprime_deltav * locals.var_sqrt_vds_vdssprime_deltav)));
        locals.var_tmp3_dn2 = (-(locals.var_sqrt_vds_vdssprime_deltav_dn2 / (locals.var_sqrt_vds_vdssprime_deltav * locals.var_sqrt_vds_vdssprime_deltav)));
        locals.var_tmp3_dn3 = (-(locals.var_sqrt_vds_vdssprime_deltav_dn3 / (locals.var_sqrt_vds_vdssprime_deltav * locals.var_sqrt_vds_vdssprime_deltav)));

        let assign2170_e1652: f64 = (locals.var_dvp_dvd - 0.5);
        let assign2170_e1655: f64 = (locals.var_vdssprime * locals.var_dvdssprime_dvd);
        let assign2170_e1657: f64 = (assign2170_e1655 + locals.var_ddeltav_dvd);
        let assign2170_e1659: f64 = (assign2170_e1657 * locals.var_tmp2);
        let assign2170_e1660: f64 = (assign2170_e1652 - assign2170_e1659);
        let assign2170_e1664: f64 = (0.5 - locals.var_dvdssprime_dvd);
        let assign2170_e1665: f64 = (locals.var_vdsprime * assign2170_e1664);
        let assign2170_e1667: f64 = (assign2170_e1665 + locals.var_ddeltav_dvd);
        let assign2170_e1669: f64 = (assign2170_e1667 * locals.var_tmp3);
        let assign2170_e1670: f64 = (assign2170_e1660 + assign2170_e1669);
        let assign2170_e1671: f64 = (locals.var_tmp1 * assign2170_e1670);
        locals.var_dirprime_dvd = assign2170_e1671;
        locals.var_dirprime_dvd_dn0 = ((locals.var_tmp1_dn0 * assign2170_e1670) + (locals.var_tmp1 * ((locals.var_dvp_dvd_dn0 - (((((locals.var_vdssprime_dn0 * locals.var_dvdssprime_dvd) + (locals.var_vdssprime * locals.var_dvdssprime_dvd_dn0)) + locals.var_ddeltav_dvd_dn0) * locals.var_tmp2) + (assign2170_e1657 * locals.var_tmp2_dn0))) + (((((locals.var_vdsprime_dn0 * assign2170_e1664) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvd_dn0))) + locals.var_ddeltav_dvd_dn0) * locals.var_tmp3) + (assign2170_e1667 * locals.var_tmp3_dn0)))));
        locals.var_dirprime_dvd_dn1 = ((locals.var_tmp1_dn1 * assign2170_e1670) + (locals.var_tmp1 * ((locals.var_dvp_dvd_dn1 - (((((locals.var_vdssprime_dn1 * locals.var_dvdssprime_dvd) + (locals.var_vdssprime * locals.var_dvdssprime_dvd_dn1)) + locals.var_ddeltav_dvd_dn1) * locals.var_tmp2) + (assign2170_e1657 * locals.var_tmp2_dn1))) + (((((locals.var_vdsprime_dn1 * assign2170_e1664) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvd_dn1))) + locals.var_ddeltav_dvd_dn1) * locals.var_tmp3) + (assign2170_e1667 * locals.var_tmp3_dn1)))));
        locals.var_dirprime_dvd_dn2 = ((locals.var_tmp1_dn2 * assign2170_e1670) + (locals.var_tmp1 * ((locals.var_dvp_dvd_dn2 - (((((locals.var_vdssprime_dn2 * locals.var_dvdssprime_dvd) + (locals.var_vdssprime * locals.var_dvdssprime_dvd_dn2)) + locals.var_ddeltav_dvd_dn2) * locals.var_tmp2) + (assign2170_e1657 * locals.var_tmp2_dn2))) + (((((locals.var_vdsprime_dn2 * assign2170_e1664) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvd_dn2))) + locals.var_ddeltav_dvd_dn2) * locals.var_tmp3) + (assign2170_e1667 * locals.var_tmp3_dn2)))));
        locals.var_dirprime_dvd_dn3 = ((locals.var_tmp1_dn3 * assign2170_e1670) + (locals.var_tmp1 * ((locals.var_dvp_dvd_dn3 - (((((locals.var_vdssprime_dn3 * locals.var_dvdssprime_dvd) + (locals.var_vdssprime * locals.var_dvdssprime_dvd_dn3)) + locals.var_ddeltav_dvd_dn3) * locals.var_tmp2) + (assign2170_e1657 * locals.var_tmp2_dn3))) + (((((locals.var_vdsprime_dn3 * assign2170_e1664) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvd_dn3))) + locals.var_ddeltav_dvd_dn3) * locals.var_tmp3) + (assign2170_e1667 * locals.var_tmp3_dn3)))));

        let assign2180_e1675: f64 = (locals.var_dvp_dvs - 0.5);
        let assign2180_e1678: f64 = (locals.var_vdssprime * locals.var_dvdssprime_dvs);
        let assign2180_e1680: f64 = (assign2180_e1678 + locals.var_ddeltav_dvs);
        let assign2180_e1682: f64 = (assign2180_e1680 * locals.var_tmp2);
        let assign2180_e1683: f64 = (assign2180_e1675 - assign2180_e1682);
        let assign2180_e1686: f64 = (-0.5);
        let assign2180_e1688: f64 = (assign2180_e1686 - locals.var_dvdssprime_dvs);
        let assign2180_e1689: f64 = (locals.var_vdsprime * assign2180_e1688);
        let assign2180_e1691: f64 = (assign2180_e1689 + locals.var_ddeltav_dvs);
        let assign2180_e1693: f64 = (assign2180_e1691 * locals.var_tmp3);
        let assign2180_e1694: f64 = (assign2180_e1683 + assign2180_e1693);
        let assign2180_e1695: f64 = (locals.var_tmp1 * assign2180_e1694);
        locals.var_dirprime_dvs = assign2180_e1695;
        locals.var_dirprime_dvs_dn0 = ((locals.var_tmp1_dn0 * assign2180_e1694) + (locals.var_tmp1 * ((locals.var_dvp_dvs_dn0 - (((((locals.var_vdssprime_dn0 * locals.var_dvdssprime_dvs) + (locals.var_vdssprime * locals.var_dvdssprime_dvs_dn0)) + locals.var_ddeltav_dvs_dn0) * locals.var_tmp2) + (assign2180_e1680 * locals.var_tmp2_dn0))) + (((((locals.var_vdsprime_dn0 * assign2180_e1688) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvs_dn0))) + locals.var_ddeltav_dvs_dn0) * locals.var_tmp3) + (assign2180_e1691 * locals.var_tmp3_dn0)))));
        locals.var_dirprime_dvs_dn1 = ((locals.var_tmp1_dn1 * assign2180_e1694) + (locals.var_tmp1 * ((locals.var_dvp_dvs_dn1 - (((((locals.var_vdssprime_dn1 * locals.var_dvdssprime_dvs) + (locals.var_vdssprime * locals.var_dvdssprime_dvs_dn1)) + locals.var_ddeltav_dvs_dn1) * locals.var_tmp2) + (assign2180_e1680 * locals.var_tmp2_dn1))) + (((((locals.var_vdsprime_dn1 * assign2180_e1688) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvs_dn1))) + locals.var_ddeltav_dvs_dn1) * locals.var_tmp3) + (assign2180_e1691 * locals.var_tmp3_dn1)))));
        locals.var_dirprime_dvs_dn2 = ((locals.var_tmp1_dn2 * assign2180_e1694) + (locals.var_tmp1 * ((locals.var_dvp_dvs_dn2 - (((((locals.var_vdssprime_dn2 * locals.var_dvdssprime_dvs) + (locals.var_vdssprime * locals.var_dvdssprime_dvs_dn2)) + locals.var_ddeltav_dvs_dn2) * locals.var_tmp2) + (assign2180_e1680 * locals.var_tmp2_dn2))) + (((((locals.var_vdsprime_dn2 * assign2180_e1688) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvs_dn2))) + locals.var_ddeltav_dvs_dn2) * locals.var_tmp3) + (assign2180_e1691 * locals.var_tmp3_dn2)))));
        locals.var_dirprime_dvs_dn3 = ((locals.var_tmp1_dn3 * assign2180_e1694) + (locals.var_tmp1 * ((locals.var_dvp_dvs_dn3 - (((((locals.var_vdssprime_dn3 * locals.var_dvdssprime_dvs) + (locals.var_vdssprime * locals.var_dvdssprime_dvs_dn3)) + locals.var_ddeltav_dvs_dn3) * locals.var_tmp2) + (assign2180_e1680 * locals.var_tmp2_dn3))) + (((((locals.var_vdsprime_dn3 * assign2180_e1688) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvs_dn3))) + locals.var_ddeltav_dvs_dn3) * locals.var_tmp3) + (assign2180_e1691 * locals.var_tmp3_dn3)))));

        let assign2200_e1719: f64 = (locals.var_lc_ucrit + locals.var_vds);
        let assign2200_e1721: f64 = (assign2200_e1719 - locals.var_vip);
        let assign2200_e1722: f64 = (locals.var_lc_lambda / assign2200_e1721);
        locals.var_tmp1 = assign2200_e1722;
        locals.var_tmp1_dn0 = (-((locals.var_lc_lambda * (locals.var_vds_dn0 - locals.var_vip_dn0)) / (assign2200_e1721 * assign2200_e1721)));
        locals.var_tmp1_dn1 = (-((locals.var_lc_lambda * (-locals.var_vip_dn1)) / (assign2200_e1721 * assign2200_e1721)));
        locals.var_tmp1_dn2 = (-((locals.var_lc_lambda * (locals.var_vds_dn2 - locals.var_vip_dn2)) / (assign2200_e1721 * assign2200_e1721)));
        locals.var_tmp1_dn3 = (-((locals.var_lc_lambda * (locals.var_vds_dn3 - locals.var_vip_dn3)) / (assign2200_e1721 * assign2200_e1721)));

        let assign2210_e1726: f64 = (0.5 - locals.var_dvip_dvd);
        let assign2210_e1727: f64 = (locals.var_tmp1 * assign2210_e1726);
        locals.var_ddeltal_dvd = assign2210_e1727;
        locals.var_ddeltal_dvd_dn0 = ((locals.var_tmp1_dn0 * assign2210_e1726) + (locals.var_tmp1 * (-locals.var_dvip_dvd_dn0)));
        locals.var_ddeltal_dvd_dn1 = ((locals.var_tmp1_dn1 * assign2210_e1726) + (locals.var_tmp1 * (-locals.var_dvip_dvd_dn1)));
        locals.var_ddeltal_dvd_dn2 = ((locals.var_tmp1_dn2 * assign2210_e1726) + (locals.var_tmp1 * (-locals.var_dvip_dvd_dn2)));
        locals.var_ddeltal_dvd_dn3 = ((locals.var_tmp1_dn3 * assign2210_e1726) + (locals.var_tmp1 * (-locals.var_dvip_dvd_dn3)));

        let assign2220_e1730: f64 = (-0.5);
        let assign2220_e1732: f64 = (assign2220_e1730 - locals.var_dvip_dvs);
        let assign2220_e1733: f64 = (locals.var_tmp1 * assign2220_e1732);
        locals.var_ddeltal_dvs = assign2220_e1733;
        locals.var_ddeltal_dvs_dn0 = ((locals.var_tmp1_dn0 * assign2220_e1732) + (locals.var_tmp1 * (-locals.var_dvip_dvs_dn0)));
        locals.var_ddeltal_dvs_dn1 = ((locals.var_tmp1_dn1 * assign2220_e1732) + (locals.var_tmp1 * (-locals.var_dvip_dvs_dn1)));
        locals.var_ddeltal_dvs_dn2 = ((locals.var_tmp1_dn2 * assign2220_e1732) + (locals.var_tmp1 * (-locals.var_dvip_dvs_dn2)));
        locals.var_ddeltal_dvs_dn3 = ((locals.var_tmp1_dn3 * assign2220_e1732) + (locals.var_tmp1 * (-locals.var_dvip_dvs_dn3)));

        let assign2240_e1740: f64 = (1.0 / locals.var_sqrt_lprime_lmin);
        locals.var_tmp1 = assign2240_e1740;
        locals.var_tmp1_dn0 = (-(locals.var_sqrt_lprime_lmin_dn0 / (locals.var_sqrt_lprime_lmin * locals.var_sqrt_lprime_lmin)));
        locals.var_tmp1_dn1 = (-(locals.var_sqrt_lprime_lmin_dn1 / (locals.var_sqrt_lprime_lmin * locals.var_sqrt_lprime_lmin)));
        locals.var_tmp1_dn2 = (-(locals.var_sqrt_lprime_lmin_dn2 / (locals.var_sqrt_lprime_lmin * locals.var_sqrt_lprime_lmin)));
        locals.var_tmp1_dn3 = (-(locals.var_sqrt_lprime_lmin_dn3 / (locals.var_sqrt_lprime_lmin * locals.var_sqrt_lprime_lmin)));

        let assign2250_e1743: f64 = (-locals.var_ddeltal_dvd);
        let assign2250_e1746: f64 = (0.5 + locals.var_dvip_dvd);
        let assign2250_e1748: f64 = (assign2250_e1746 * locals.var_inv_ucrit);
        let assign2250_e1749: f64 = (assign2250_e1743 + assign2250_e1748);
        let assign2250_e1750: f64 = (locals.var_tmp1 * assign2250_e1749);
        locals.var_dleq_dvd = assign2250_e1750;
        locals.var_dleq_dvd_dn0 = ((locals.var_tmp1_dn0 * assign2250_e1749) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvd_dn0) + (locals.var_dvip_dvd_dn0 * locals.var_inv_ucrit))));
        locals.var_dleq_dvd_dn1 = ((locals.var_tmp1_dn1 * assign2250_e1749) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvd_dn1) + (locals.var_dvip_dvd_dn1 * locals.var_inv_ucrit))));
        locals.var_dleq_dvd_dn2 = ((locals.var_tmp1_dn2 * assign2250_e1749) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvd_dn2) + (locals.var_dvip_dvd_dn2 * locals.var_inv_ucrit))));
        locals.var_dleq_dvd_dn3 = ((locals.var_tmp1_dn3 * assign2250_e1749) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvd_dn3) + (locals.var_dvip_dvd_dn3 * locals.var_inv_ucrit))));

        let assign2260_e1753: f64 = (-locals.var_ddeltal_dvs);
        let assign2260_e1755: f64 = (-0.5);
        let assign2260_e1757: f64 = (assign2260_e1755 + locals.var_dvip_dvs);
        let assign2260_e1759: f64 = (assign2260_e1757 * locals.var_inv_ucrit);
        let assign2260_e1760: f64 = (assign2260_e1753 + assign2260_e1759);
        let assign2260_e1761: f64 = (locals.var_tmp1 * assign2260_e1760);
        locals.var_dleq_dvs = assign2260_e1761;
        locals.var_dleq_dvs_dn0 = ((locals.var_tmp1_dn0 * assign2260_e1760) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvs_dn0) + (locals.var_dvip_dvs_dn0 * locals.var_inv_ucrit))));
        locals.var_dleq_dvs_dn1 = ((locals.var_tmp1_dn1 * assign2260_e1760) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvs_dn1) + (locals.var_dvip_dvs_dn1 * locals.var_inv_ucrit))));
        locals.var_dleq_dvs_dn2 = ((locals.var_tmp1_dn2 * assign2260_e1760) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvs_dn2) + (locals.var_dvip_dvs_dn2 * locals.var_inv_ucrit))));
        locals.var_dleq_dvs_dn3 = ((locals.var_tmp1_dn3 * assign2260_e1760) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvs_dn3) + (locals.var_dvip_dvs_dn3 * locals.var_inv_ucrit))));

        let assign2280_e1772: f64 = (locals.var_dir_dv * locals.var_inv_vt);
        locals.var_tmp1 = assign2280_e1772;
        locals.var_tmp1_dn0 = (locals.var_dir_dv_dn0 * locals.var_inv_vt);
        locals.var_tmp1_dn1 = (locals.var_dir_dv_dn1 * locals.var_inv_vt);
        locals.var_tmp1_dn2 = (locals.var_dir_dv_dn2 * locals.var_inv_vt);
        locals.var_tmp1_dn3 = (locals.var_dir_dv_dn3 * locals.var_inv_vt);

        let assign2290_e1776: f64 = (locals.var_dvp_dvd - 1.0);
        let assign2290_e1777: f64 = (locals.var_tmp1 * assign2290_e1776);
        locals.var_dir_dvd = assign2290_e1777;
        locals.var_dir_dvd_dn0 = ((locals.var_tmp1_dn0 * assign2290_e1776) + (locals.var_tmp1 * locals.var_dvp_dvd_dn0));
        locals.var_dir_dvd_dn1 = ((locals.var_tmp1_dn1 * assign2290_e1776) + (locals.var_tmp1 * locals.var_dvp_dvd_dn1));
        locals.var_dir_dvd_dn2 = ((locals.var_tmp1_dn2 * assign2290_e1776) + (locals.var_tmp1 * locals.var_dvp_dvd_dn2));
        locals.var_dir_dvd_dn3 = ((locals.var_tmp1_dn3 * assign2290_e1776) + (locals.var_tmp1 * locals.var_dvp_dvd_dn3));

        let assign2300_e1780: f64 = (locals.var_tmp1 * locals.var_dvp_dvs);
        locals.var_dir_dvs = assign2300_e1780;
        locals.var_dir_dvs_dn0 = ((locals.var_tmp1_dn0 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn0));
        locals.var_dir_dvs_dn1 = ((locals.var_tmp1_dn1 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn1));
        locals.var_dir_dvs_dn2 = ((locals.var_tmp1_dn2 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn2));
        locals.var_dir_dvs_dn3 = ((locals.var_tmp1_dn3 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn3));

        let assign2320_e1786: f64 = (1.0 + locals.var_n_1);
        let assign2320_e1787: f64 = (-assign2320_e1786);
        let assign2320_e1789: f64 = (assign2320_e1787 * locals.var_vt);
        let assign2320_e1791: f64 = (assign2320_e1789 * 0.66666666);
        let assign2320_e1793: f64 = (assign2320_e1791 / locals.var_sif_sir_2);
        locals.var_tmp1 = assign2320_e1793;
        locals.var_tmp1_dn0 = ((((((-locals.var_n_1_dn0) * locals.var_vt) * 0.66666666) * locals.var_sif_sir_2) - (assign2320_e1791 * locals.var_sif_sir_2_dn0)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2));
        locals.var_tmp1_dn1 = ((((((-locals.var_n_1_dn1) * locals.var_vt) * 0.66666666) * locals.var_sif_sir_2) - (assign2320_e1791 * locals.var_sif_sir_2_dn1)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2));
        locals.var_tmp1_dn2 = ((((((-locals.var_n_1_dn2) * locals.var_vt) * 0.66666666) * locals.var_sif_sir_2) - (assign2320_e1791 * locals.var_sif_sir_2_dn2)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2));
        locals.var_tmp1_dn3 = ((((((-locals.var_n_1_dn3) * locals.var_vt) * 0.66666666) * locals.var_sif_sir_2) - (assign2320_e1791 * locals.var_sif_sir_2_dn3)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2));

        let assign2330_e1798: f64 = (2.0 * locals.var_sir);
        let assign2330_e1799: f64 = (locals.var_sif + assign2330_e1798);
        let assign2330_e1800: f64 = (locals.var_tmp1 * assign2330_e1799);
        locals.var_tmp2 = assign2330_e1800;
        locals.var_tmp2_dn0 = ((locals.var_tmp1_dn0 * assign2330_e1799) + (locals.var_tmp1 * (locals.var_sif_dn0 + (2.0 * locals.var_sir_dn0))));
        locals.var_tmp2_dn1 = ((locals.var_tmp1_dn1 * assign2330_e1799) + (locals.var_tmp1 * (locals.var_sif_dn1 + (2.0 * locals.var_sir_dn1))));
        locals.var_tmp2_dn2 = ((locals.var_tmp1_dn2 * assign2330_e1799) + (locals.var_tmp1 * (locals.var_sif_dn2 + (2.0 * locals.var_sir_dn2))));
        locals.var_tmp2_dn3 = ((locals.var_tmp1_dn3 * assign2330_e1799) + (locals.var_tmp1 * (locals.var_sif_dn3 + (2.0 * locals.var_sir_dn3))));

        let assign2340_e1805: f64 = (2.0 * locals.var_sif);
        let assign2340_e1806: f64 = (locals.var_sir + assign2340_e1805);
        let assign2340_e1807: f64 = (locals.var_tmp1 * assign2340_e1806);
        locals.var_tmp3 = assign2340_e1807;
        locals.var_tmp3_dn0 = ((locals.var_tmp1_dn0 * assign2340_e1806) + (locals.var_tmp1 * (locals.var_sir_dn0 + (2.0 * locals.var_sif_dn0))));
        locals.var_tmp3_dn1 = ((locals.var_tmp1_dn1 * assign2340_e1806) + (locals.var_tmp1 * (locals.var_sir_dn1 + (2.0 * locals.var_sif_dn1))));
        locals.var_tmp3_dn2 = ((locals.var_tmp1_dn2 * assign2340_e1806) + (locals.var_tmp1 * (locals.var_sir_dn2 + (2.0 * locals.var_sif_dn2))));
        locals.var_tmp3_dn3 = ((locals.var_tmp1_dn3 * assign2340_e1806) + (locals.var_tmp1 * (locals.var_sir_dn3 + (2.0 * locals.var_sif_dn3))));

        let assign2350_e1809: f64 = (-locals.var_n_1);
        let assign2350_e1811: f64 = (assign2350_e1809 * locals.var_qi);
        let assign2350_e1814: f64 = (2.0 + locals.var_n_1);
        let assign2350_e1816: f64 = (assign2350_e1814 + locals.var_n_1);
        let assign2350_e1818: f64 = (assign2350_e1816 * locals.var_vp_phi_eps);
        let assign2350_e1819: f64 = (assign2350_e1811 / assign2350_e1818);
        locals.var_tmp1 = assign2350_e1819;
        locals.var_tmp1_dn0 = ((((((-locals.var_n_1_dn0) * locals.var_qi) + (assign2350_e1809 * locals.var_qi_dn0)) * assign2350_e1818) - (assign2350_e1811 * (((locals.var_n_1_dn0 + locals.var_n_1_dn0) * locals.var_vp_phi_eps) + (assign2350_e1816 * locals.var_vp_phi_eps_dn0)))) / (assign2350_e1818 * assign2350_e1818));
        locals.var_tmp1_dn1 = ((((((-locals.var_n_1_dn1) * locals.var_qi) + (assign2350_e1809 * locals.var_qi_dn1)) * assign2350_e1818) - (assign2350_e1811 * (((locals.var_n_1_dn1 + locals.var_n_1_dn1) * locals.var_vp_phi_eps) + (assign2350_e1816 * locals.var_vp_phi_eps_dn1)))) / (assign2350_e1818 * assign2350_e1818));
        locals.var_tmp1_dn2 = ((((((-locals.var_n_1_dn2) * locals.var_qi) + (assign2350_e1809 * locals.var_qi_dn2)) * assign2350_e1818) - (assign2350_e1811 * (((locals.var_n_1_dn2 + locals.var_n_1_dn2) * locals.var_vp_phi_eps) + (assign2350_e1816 * locals.var_vp_phi_eps_dn2)))) / (assign2350_e1818 * assign2350_e1818));
        locals.var_tmp1_dn3 = ((((((-locals.var_n_1_dn3) * locals.var_qi) + (assign2350_e1809 * locals.var_qi_dn3)) * assign2350_e1818) - (assign2350_e1811 * (((locals.var_n_1_dn3 + locals.var_n_1_dn3) * locals.var_vp_phi_eps) + (assign2350_e1816 * locals.var_vp_phi_eps_dn3)))) / (assign2350_e1818 * assign2350_e1818));

        let assign2360_e1822: f64 = (locals.var_tmp1 * locals.var_dvp_dvd);
        let assign2360_e1825: f64 = (locals.var_tmp2 * locals.var_dif_dvd);
        let assign2360_e1826: f64 = (assign2360_e1822 + assign2360_e1825);
        let assign2360_e1829: f64 = (locals.var_tmp3 * locals.var_dir_dvd);
        let assign2360_e1830: f64 = (assign2360_e1826 + assign2360_e1829);
        locals.var_dqi_dvd = assign2360_e1830;
        locals.var_dqi_dvd_dn0 = ((((locals.var_tmp1_dn0 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn0)) + ((locals.var_tmp2_dn0 * locals.var_dif_dvd) + (locals.var_tmp2 * locals.var_dif_dvd_dn0))) + ((locals.var_tmp3_dn0 * locals.var_dir_dvd) + (locals.var_tmp3 * locals.var_dir_dvd_dn0)));
        locals.var_dqi_dvd_dn1 = ((((locals.var_tmp1_dn1 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn1)) + ((locals.var_tmp2_dn1 * locals.var_dif_dvd) + (locals.var_tmp2 * locals.var_dif_dvd_dn1))) + ((locals.var_tmp3_dn1 * locals.var_dir_dvd) + (locals.var_tmp3 * locals.var_dir_dvd_dn1)));
        locals.var_dqi_dvd_dn2 = ((((locals.var_tmp1_dn2 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn2)) + ((locals.var_tmp2_dn2 * locals.var_dif_dvd) + (locals.var_tmp2 * locals.var_dif_dvd_dn2))) + ((locals.var_tmp3_dn2 * locals.var_dir_dvd) + (locals.var_tmp3 * locals.var_dir_dvd_dn2)));
        locals.var_dqi_dvd_dn3 = ((((locals.var_tmp1_dn3 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn3)) + ((locals.var_tmp2_dn3 * locals.var_dif_dvd) + (locals.var_tmp2 * locals.var_dif_dvd_dn3))) + ((locals.var_tmp3_dn3 * locals.var_dir_dvd) + (locals.var_tmp3 * locals.var_dir_dvd_dn3)));

        let assign2370_e1833: f64 = (locals.var_tmp1 * locals.var_dvp_dvs);
        let assign2370_e1836: f64 = (locals.var_tmp2 * locals.var_dif_dvs);
        let assign2370_e1837: f64 = (assign2370_e1833 + assign2370_e1836);
        let assign2370_e1840: f64 = (locals.var_tmp3 * locals.var_dir_dvs);
        let assign2370_e1841: f64 = (assign2370_e1837 + assign2370_e1840);
        locals.var_dqi_dvs = assign2370_e1841;
        locals.var_dqi_dvs_dn0 = ((((locals.var_tmp1_dn0 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn0)) + ((locals.var_tmp2_dn0 * locals.var_dif_dvs) + (locals.var_tmp2 * locals.var_dif_dvs_dn0))) + ((locals.var_tmp3_dn0 * locals.var_dir_dvs) + (locals.var_tmp3 * locals.var_dir_dvs_dn0)));
        locals.var_dqi_dvs_dn1 = ((((locals.var_tmp1_dn1 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn1)) + ((locals.var_tmp2_dn1 * locals.var_dif_dvs) + (locals.var_tmp2 * locals.var_dif_dvs_dn1))) + ((locals.var_tmp3_dn1 * locals.var_dir_dvs) + (locals.var_tmp3 * locals.var_dir_dvs_dn1)));
        locals.var_dqi_dvs_dn2 = ((((locals.var_tmp1_dn2 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn2)) + ((locals.var_tmp2_dn2 * locals.var_dif_dvs) + (locals.var_tmp2 * locals.var_dif_dvs_dn2))) + ((locals.var_tmp3_dn2 * locals.var_dir_dvs) + (locals.var_tmp3 * locals.var_dir_dvs_dn2)));
        locals.var_dqi_dvs_dn3 = ((((locals.var_tmp1_dn3 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn3)) + ((locals.var_tmp2_dn3 * locals.var_dif_dvs) + (locals.var_tmp2 * locals.var_dif_dvs_dn3))) + ((locals.var_tmp3_dn3 * locals.var_dir_dvs) + (locals.var_tmp3 * locals.var_dir_dvs_dn3)));

        let assign2390_e1855: f64 = (1.0 + locals.var_n_1);
        let assign2390_e1860: f64 = (1.0 + locals.var_n_1);
        let assign2390_e1861: f64 = (2.0 * assign2390_e1860);
        let assign2390_e1863: f64 = (assign2390_e1861 * locals.var_vp_phi_eps);
        let assign2390_e1864: f64 = (locals.var_qi / assign2390_e1863);
        let assign2390_e1865: f64 = (assign2390_e1855 - assign2390_e1864);
        locals.var_tmp1 = assign2390_e1865;
        locals.var_tmp1_dn0 = (locals.var_n_1_dn0 - (((locals.var_qi_dn0 * assign2390_e1863) - (locals.var_qi * (((2.0 * locals.var_n_1_dn0) * locals.var_vp_phi_eps) + (assign2390_e1861 * locals.var_vp_phi_eps_dn0)))) / (assign2390_e1863 * assign2390_e1863)));
        locals.var_tmp1_dn1 = (locals.var_n_1_dn1 - (((locals.var_qi_dn1 * assign2390_e1863) - (locals.var_qi * (((2.0 * locals.var_n_1_dn1) * locals.var_vp_phi_eps) + (assign2390_e1861 * locals.var_vp_phi_eps_dn1)))) / (assign2390_e1863 * assign2390_e1863)));
        locals.var_tmp1_dn2 = (locals.var_n_1_dn2 - (((locals.var_qi_dn2 * assign2390_e1863) - (locals.var_qi * (((2.0 * locals.var_n_1_dn2) * locals.var_vp_phi_eps) + (assign2390_e1861 * locals.var_vp_phi_eps_dn2)))) / (assign2390_e1863 * assign2390_e1863)));
        locals.var_tmp1_dn3 = (locals.var_n_1_dn3 - (((locals.var_qi_dn3 * assign2390_e1863) - (locals.var_qi * (((2.0 * locals.var_n_1_dn3) * locals.var_vp_phi_eps) + (assign2390_e1861 * locals.var_vp_phi_eps_dn3)))) / (assign2390_e1863 * assign2390_e1863)));

        let assign2400_e1867: f64 = (-locals.var_n_1_n);
        let assign2400_e1870: f64 = (locals.var_tmp1 * locals.var_dvp_dvd);
        let assign2400_e1872: f64 = (assign2400_e1870 + locals.var_dqi_dvd);
        let assign2400_e1873: f64 = (assign2400_e1867 * assign2400_e1872);
        locals.var_dqb_dvd = assign2400_e1873;
        locals.var_dqb_dvd_dn0 = (((-locals.var_n_1_n_dn0) * assign2400_e1872) + (assign2400_e1867 * (((locals.var_tmp1_dn0 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn0)) + locals.var_dqi_dvd_dn0)));
        locals.var_dqb_dvd_dn1 = (((-locals.var_n_1_n_dn1) * assign2400_e1872) + (assign2400_e1867 * (((locals.var_tmp1_dn1 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn1)) + locals.var_dqi_dvd_dn1)));
        locals.var_dqb_dvd_dn2 = (((-locals.var_n_1_n_dn2) * assign2400_e1872) + (assign2400_e1867 * (((locals.var_tmp1_dn2 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn2)) + locals.var_dqi_dvd_dn2)));
        locals.var_dqb_dvd_dn3 = (((-locals.var_n_1_n_dn3) * assign2400_e1872) + (assign2400_e1867 * (((locals.var_tmp1_dn3 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn3)) + locals.var_dqi_dvd_dn3)));

        let assign2410_e1875: f64 = (-locals.var_n_1_n);
        let assign2410_e1878: f64 = (locals.var_tmp1 * locals.var_dvp_dvs);
        let assign2410_e1880: f64 = (assign2410_e1878 + locals.var_dqi_dvs);
        let assign2410_e1881: f64 = (assign2410_e1875 * assign2410_e1880);
        locals.var_dqb_dvs = assign2410_e1881;
        locals.var_dqb_dvs_dn0 = (((-locals.var_n_1_n_dn0) * assign2410_e1880) + (assign2410_e1875 * (((locals.var_tmp1_dn0 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn0)) + locals.var_dqi_dvs_dn0)));
        locals.var_dqb_dvs_dn1 = (((-locals.var_n_1_n_dn1) * assign2410_e1880) + (assign2410_e1875 * (((locals.var_tmp1_dn1 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn1)) + locals.var_dqi_dvs_dn1)));
        locals.var_dqb_dvs_dn2 = (((-locals.var_n_1_n_dn2) * assign2410_e1880) + (assign2410_e1875 * (((locals.var_tmp1_dn2 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn2)) + locals.var_dqi_dvs_dn2)));
        locals.var_dqb_dvs_dn3 = (((-locals.var_n_1_n_dn3) * assign2410_e1880) + (assign2410_e1875 * (((locals.var_tmp1_dn3 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn3)) + locals.var_dqi_dvs_dn3)));

        let assign2430_e1892: f64 = if p.p22 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard18 = assign2430_e1892;

        let (assign2440_e1902, assign2440_e1902_d_n0, assign2440_e1902_d_n1, assign2440_e1902_d_n2, assign2440_e1902_d_n3,) = {
    if (locals.var_guard18 != 0.0) {
        let assign2440_e1896: f64 = (p.p21 * locals.var_vpprime);
        let assign2440_e1899: f64 = (locals.var_theta_vp_1 * locals.var_sqrt_vp_vt);
        let assign2440_e1900: f64 = (assign2440_e1896 / assign2440_e1899);
        (assign2440_e1900, ((((p.p21 * locals.var_vpprime_dn0) * assign2440_e1899) - (assign2440_e1896 * ((locals.var_theta_vp_1_dn0 * locals.var_sqrt_vp_vt) + (locals.var_theta_vp_1 * locals.var_sqrt_vp_vt_dn0)))) / (assign2440_e1899 * assign2440_e1899)), ((((p.p21 * locals.var_vpprime_dn1) * assign2440_e1899) - (assign2440_e1896 * ((locals.var_theta_vp_1_dn1 * locals.var_sqrt_vp_vt) + (locals.var_theta_vp_1 * locals.var_sqrt_vp_vt_dn1)))) / (assign2440_e1899 * assign2440_e1899)), ((((p.p21 * locals.var_vpprime_dn2) * assign2440_e1899) - (assign2440_e1896 * ((locals.var_theta_vp_1_dn2 * locals.var_sqrt_vp_vt) + (locals.var_theta_vp_1 * locals.var_sqrt_vp_vt_dn2)))) / (assign2440_e1899 * assign2440_e1899)), ((((p.p21 * locals.var_vpprime_dn3) * assign2440_e1899) - (assign2440_e1896 * ((locals.var_theta_vp_1_dn3 * locals.var_sqrt_vp_vt) + (locals.var_theta_vp_1 * locals.var_sqrt_vp_vt_dn3)))) / (assign2440_e1899 * assign2440_e1899)),)
    } else {
        (locals.var_tmp1, locals.var_tmp1_dn0, locals.var_tmp1_dn1, locals.var_tmp1_dn2, locals.var_tmp1_dn3,)
    }
};
        locals.var_tmp1 = assign2440_e1902;
        locals.var_tmp1_dn0 = assign2440_e1902_d_n0;
        locals.var_tmp1_dn1 = assign2440_e1902_d_n1;
        locals.var_tmp1_dn2 = assign2440_e1902_d_n2;
        locals.var_tmp1_dn3 = assign2440_e1902_d_n3;

        let (assign2450_e1908, assign2450_e1908_d_n0, assign2450_e1908_d_n1, assign2450_e1908_d_n2, assign2450_e1908_d_n3,) = {
    if (locals.var_guard18 != 0.0) {
        let assign2450_e1906: f64 = (locals.var_tmp1 * locals.var_dvp_dvd);
        (assign2450_e1906, ((locals.var_tmp1_dn0 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn0)), ((locals.var_tmp1_dn1 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn1)), ((locals.var_tmp1_dn2 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn2)), ((locals.var_tmp1_dn3 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn3)),)
    } else {
        (locals.var_dvpprime_dvd, locals.var_dvpprime_dvd_dn0, locals.var_dvpprime_dvd_dn1, locals.var_dvpprime_dvd_dn2, locals.var_dvpprime_dvd_dn3,)
    }
};
        locals.var_dvpprime_dvd = assign2450_e1908;
        locals.var_dvpprime_dvd_dn0 = assign2450_e1908_d_n0;
        locals.var_dvpprime_dvd_dn1 = assign2450_e1908_d_n1;
        locals.var_dvpprime_dvd_dn2 = assign2450_e1908_d_n2;
        locals.var_dvpprime_dvd_dn3 = assign2450_e1908_d_n3;

    }

    pub(super) fn stamp_transient_block_4(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let (assign2460_e1914, assign2460_e1914_d_n0, assign2460_e1914_d_n1, assign2460_e1914_d_n2, assign2460_e1914_d_n3,) = {
    if (locals.var_guard18 != 0.0) {
        let assign2460_e1912: f64 = (locals.var_tmp1 * locals.var_dvp_dvs);
        (assign2460_e1912, ((locals.var_tmp1_dn0 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn0)), ((locals.var_tmp1_dn1 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn1)), ((locals.var_tmp1_dn2 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn2)), ((locals.var_tmp1_dn3 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn3)),)
    } else {
        (locals.var_dvpprime_dvs, locals.var_dvpprime_dvs_dn0, locals.var_dvpprime_dvs_dn1, locals.var_dvpprime_dvs_dn2, locals.var_dvpprime_dvs_dn3,)
    }
};
        locals.var_dvpprime_dvs = assign2460_e1914;
        locals.var_dvpprime_dvs_dn0 = assign2460_e1914_d_n0;
        locals.var_dvpprime_dvs_dn1 = assign2460_e1914_d_n1;
        locals.var_dvpprime_dvs_dn2 = assign2460_e1914_d_n2;
        locals.var_dvpprime_dvs_dn3 = assign2460_e1914_d_n3;

        let (assign2480_e1927, assign2480_e1927_d_n0, assign2480_e1927_d_n1, assign2480_e1927_d_n2, assign2480_e1927_d_n3,) = {
    if (locals.var_guard18 != 0.0) {
        let assign2480_e1923: f64 = (-locals.var_dleq_dvd);
        let assign2480_e1925: f64 = (assign2480_e1923 - locals.var_dvpprime_dvd);
        (assign2480_e1925, ((-locals.var_dleq_dvd_dn0) - locals.var_dvpprime_dvd_dn0), ((-locals.var_dleq_dvd_dn1) - locals.var_dvpprime_dvd_dn1), ((-locals.var_dleq_dvd_dn2) - locals.var_dvpprime_dvd_dn2), ((-locals.var_dleq_dvd_dn3) - locals.var_dvpprime_dvd_dn3),)
    } else {
        (locals.var_dbeta_dvd, locals.var_dbeta_dvd_dn0, locals.var_dbeta_dvd_dn1, locals.var_dbeta_dvd_dn2, locals.var_dbeta_dvd_dn3,)
    }
};
        locals.var_dbeta_dvd = assign2480_e1927;
        locals.var_dbeta_dvd_dn0 = assign2480_e1927_d_n0;
        locals.var_dbeta_dvd_dn1 = assign2480_e1927_d_n1;
        locals.var_dbeta_dvd_dn2 = assign2480_e1927_d_n2;
        locals.var_dbeta_dvd_dn3 = assign2480_e1927_d_n3;

        let (assign2490_e1934, assign2490_e1934_d_n0, assign2490_e1934_d_n1, assign2490_e1934_d_n2, assign2490_e1934_d_n3,) = {
    if (locals.var_guard18 != 0.0) {
        let assign2490_e1930: f64 = (-locals.var_dleq_dvs);
        let assign2490_e1932: f64 = (assign2490_e1930 - locals.var_dvpprime_dvs);
        (assign2490_e1932, ((-locals.var_dleq_dvs_dn0) - locals.var_dvpprime_dvs_dn0), ((-locals.var_dleq_dvs_dn1) - locals.var_dvpprime_dvs_dn1), ((-locals.var_dleq_dvs_dn2) - locals.var_dvpprime_dvs_dn2), ((-locals.var_dleq_dvs_dn3) - locals.var_dvpprime_dvs_dn3),)
    } else {
        (locals.var_dbeta_dvs, locals.var_dbeta_dvs_dn0, locals.var_dbeta_dvs_dn1, locals.var_dbeta_dvs_dn2, locals.var_dbeta_dvs_dn3,)
    }
};
        locals.var_dbeta_dvs = assign2490_e1934;
        locals.var_dbeta_dvs_dn0 = assign2490_e1934_d_n0;
        locals.var_dbeta_dvs_dn1 = assign2490_e1934_d_n1;
        locals.var_dbeta_dvs_dn2 = assign2490_e1934_d_n2;
        locals.var_dbeta_dvs_dn3 = assign2490_e1934_d_n3;

        let (assign2510_e1948, assign2510_e1948_d_n0, assign2510_e1948_d_n1, assign2510_e1948_d_n2, assign2510_e1948_d_n3,) = {
    if (locals.var_guard18 == 0.0) {
        let assign2510_e1946: f64 = (locals.var_t0 / locals.var_e0_q_1);
        (assign2510_e1946, (-((locals.var_t0 * locals.var_e0_q_1_dn0) / (locals.var_e0_q_1 * locals.var_e0_q_1))), (-((locals.var_t0 * locals.var_e0_q_1_dn1) / (locals.var_e0_q_1 * locals.var_e0_q_1))), (-((locals.var_t0 * locals.var_e0_q_1_dn2) / (locals.var_e0_q_1 * locals.var_e0_q_1))), (-((locals.var_t0 * locals.var_e0_q_1_dn3) / (locals.var_e0_q_1 * locals.var_e0_q_1))),)
    } else {
        (locals.var_tmp1, locals.var_tmp1_dn0, locals.var_tmp1_dn1, locals.var_tmp1_dn2, locals.var_tmp1_dn3,)
    }
};
        locals.var_tmp1 = assign2510_e1948;
        locals.var_tmp1_dn0 = assign2510_e1948_d_n0;
        locals.var_tmp1_dn1 = assign2510_e1948_d_n1;
        locals.var_tmp1_dn2 = assign2510_e1948_d_n2;
        locals.var_tmp1_dn3 = assign2510_e1948_d_n3;

        let (assign2520_e1962, assign2520_e1962_d_n0, assign2520_e1962_d_n1, assign2520_e1962_d_n2, assign2520_e1962_d_n3,) = {
    if (locals.var_guard18 == 0.0) {
        let assign2520_e1952: f64 = (-locals.var_dleq_dvd);
        let assign2520_e1957: f64 = (locals.var_eta_qi * locals.var_dqi_dvd);
        let assign2520_e1958: f64 = (locals.var_dqb_dvd + assign2520_e1957);
        let assign2520_e1959: f64 = (locals.var_tmp1 * assign2520_e1958);
        let assign2520_e1960: f64 = (assign2520_e1952 + assign2520_e1959);
        (assign2520_e1960, ((-locals.var_dleq_dvd_dn0) + ((locals.var_tmp1_dn0 * assign2520_e1958) + (locals.var_tmp1 * (locals.var_dqb_dvd_dn0 + (locals.var_eta_qi * locals.var_dqi_dvd_dn0))))), ((-locals.var_dleq_dvd_dn1) + ((locals.var_tmp1_dn1 * assign2520_e1958) + (locals.var_tmp1 * (locals.var_dqb_dvd_dn1 + (locals.var_eta_qi * locals.var_dqi_dvd_dn1))))), ((-locals.var_dleq_dvd_dn2) + ((locals.var_tmp1_dn2 * assign2520_e1958) + (locals.var_tmp1 * (locals.var_dqb_dvd_dn2 + (locals.var_eta_qi * locals.var_dqi_dvd_dn2))))), ((-locals.var_dleq_dvd_dn3) + ((locals.var_tmp1_dn3 * assign2520_e1958) + (locals.var_tmp1 * (locals.var_dqb_dvd_dn3 + (locals.var_eta_qi * locals.var_dqi_dvd_dn3))))),)
    } else {
        (locals.var_dbeta_dvd, locals.var_dbeta_dvd_dn0, locals.var_dbeta_dvd_dn1, locals.var_dbeta_dvd_dn2, locals.var_dbeta_dvd_dn3,)
    }
};
        locals.var_dbeta_dvd = assign2520_e1962;
        locals.var_dbeta_dvd_dn0 = assign2520_e1962_d_n0;
        locals.var_dbeta_dvd_dn1 = assign2520_e1962_d_n1;
        locals.var_dbeta_dvd_dn2 = assign2520_e1962_d_n2;
        locals.var_dbeta_dvd_dn3 = assign2520_e1962_d_n3;

        let (assign2530_e1976, assign2530_e1976_d_n0, assign2530_e1976_d_n1, assign2530_e1976_d_n2, assign2530_e1976_d_n3,) = {
    if (locals.var_guard18 == 0.0) {
        let assign2530_e1966: f64 = (-locals.var_dleq_dvs);
        let assign2530_e1971: f64 = (locals.var_eta_qi * locals.var_dqi_dvs);
        let assign2530_e1972: f64 = (locals.var_dqb_dvs + assign2530_e1971);
        let assign2530_e1973: f64 = (locals.var_tmp1 * assign2530_e1972);
        let assign2530_e1974: f64 = (assign2530_e1966 + assign2530_e1973);
        (assign2530_e1974, ((-locals.var_dleq_dvs_dn0) + ((locals.var_tmp1_dn0 * assign2530_e1972) + (locals.var_tmp1 * (locals.var_dqb_dvs_dn0 + (locals.var_eta_qi * locals.var_dqi_dvs_dn0))))), ((-locals.var_dleq_dvs_dn1) + ((locals.var_tmp1_dn1 * assign2530_e1972) + (locals.var_tmp1 * (locals.var_dqb_dvs_dn1 + (locals.var_eta_qi * locals.var_dqi_dvs_dn1))))), ((-locals.var_dleq_dvs_dn2) + ((locals.var_tmp1_dn2 * assign2530_e1972) + (locals.var_tmp1 * (locals.var_dqb_dvs_dn2 + (locals.var_eta_qi * locals.var_dqi_dvs_dn2))))), ((-locals.var_dleq_dvs_dn3) + ((locals.var_tmp1_dn3 * assign2530_e1972) + (locals.var_tmp1 * (locals.var_dqb_dvs_dn3 + (locals.var_eta_qi * locals.var_dqi_dvs_dn3))))),)
    } else {
        (locals.var_dbeta_dvs, locals.var_dbeta_dvs_dn0, locals.var_dbeta_dvs_dn1, locals.var_dbeta_dvs_dn2, locals.var_dbeta_dvs_dn3,)
    }
};
        locals.var_dbeta_dvs = assign2530_e1976;
        locals.var_dbeta_dvs_dn0 = assign2530_e1976_d_n0;
        locals.var_dbeta_dvs_dn1 = assign2530_e1976_d_n1;
        locals.var_dbeta_dvs_dn2 = assign2530_e1976_d_n2;
        locals.var_dbeta_dvs_dn3 = assign2530_e1976_d_n3;

        let assign2550_e1992: f64 = (-locals.var_gamma_s);
        let assign2550_e1995: f64 = (4.0 * locals.var_n);
        let assign2550_e1997: f64 = (assign2550_e1995 * locals.var_sqrt_phi_vp);
        let assign2550_e2000: f64 = (locals.var_phi_t + locals.var_vp);
        let assign2550_e2002: f64 = (assign2550_e2000 + locals.var_vt_4);
        let assign2550_e2003: f64 = (assign2550_e1997 * assign2550_e2002);
        let assign2550_e2004: f64 = (assign2550_e1992 / assign2550_e2003);
        locals.var_tmp1 = assign2550_e2004;
        locals.var_tmp1_dn0 = (-((assign2550_e1992 * (((((4.0 * locals.var_n_dn0) * locals.var_sqrt_phi_vp) + (assign2550_e1995 * locals.var_sqrt_phi_vp_dn0)) * assign2550_e2002) + (assign2550_e1997 * (locals.var_phi_t_dn0 + locals.var_vp_dn0)))) / (assign2550_e2003 * assign2550_e2003)));
        locals.var_tmp1_dn1 = (-((assign2550_e1992 * (((((4.0 * locals.var_n_dn1) * locals.var_sqrt_phi_vp) + (assign2550_e1995 * locals.var_sqrt_phi_vp_dn1)) * assign2550_e2002) + (assign2550_e1997 * (locals.var_phi_t_dn1 + locals.var_vp_dn1)))) / (assign2550_e2003 * assign2550_e2003)));
        locals.var_tmp1_dn2 = (-((assign2550_e1992 * (((((4.0 * locals.var_n_dn2) * locals.var_sqrt_phi_vp) + (assign2550_e1995 * locals.var_sqrt_phi_vp_dn2)) * assign2550_e2002) + (assign2550_e1997 * (locals.var_phi_t_dn2 + locals.var_vp_dn2)))) / (assign2550_e2003 * assign2550_e2003)));
        locals.var_tmp1_dn3 = (-((assign2550_e1992 * (((((4.0 * locals.var_n_dn3) * locals.var_sqrt_phi_vp) + (assign2550_e1995 * locals.var_sqrt_phi_vp_dn3)) * assign2550_e2002) + (assign2550_e1997 * (locals.var_phi_t_dn3 + locals.var_vp_dn3)))) / (assign2550_e2003 * assign2550_e2003)));

        let assign2560_e2007: f64 = (locals.var_tmp1 * locals.var_dvp_dvd);
        locals.var_dn_dvd = assign2560_e2007;
        locals.var_dn_dvd_dn0 = ((locals.var_tmp1_dn0 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn0));
        locals.var_dn_dvd_dn1 = ((locals.var_tmp1_dn1 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn1));
        locals.var_dn_dvd_dn2 = ((locals.var_tmp1_dn2 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn2));
        locals.var_dn_dvd_dn3 = ((locals.var_tmp1_dn3 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn3));

        let assign2570_e2010: f64 = (locals.var_tmp1 * locals.var_dvp_dvs);
        locals.var_dn_dvs = assign2570_e2010;
        locals.var_dn_dvs_dn0 = ((locals.var_tmp1_dn0 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn0));
        locals.var_dn_dvs_dn1 = ((locals.var_tmp1_dn1 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn1));
        locals.var_dn_dvs_dn2 = ((locals.var_tmp1_dn2 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn2));
        locals.var_dn_dvs_dn3 = ((locals.var_tmp1_dn3 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn3));

        let assign2590_e2017: f64 = (locals.var_dn_dvd + locals.var_dbeta_dvd);
        let assign2590_e2019: f64 = (assign2590_e2017 * locals.var_if_ir);
        let assign2590_e2021: f64 = (assign2590_e2019 + locals.var_dif_dvd);
        let assign2590_e2023: f64 = (assign2590_e2021 - locals.var_dirprime_dvd);
        let assign2590_e2024: f64 = (locals.var_ispec * assign2590_e2023);
        locals.var_gds = assign2590_e2024;
        locals.var_gds_dn0 = ((locals.var_ispec_dn0 * assign2590_e2023) + (locals.var_ispec * (((((locals.var_dn_dvd_dn0 + locals.var_dbeta_dvd_dn0) * locals.var_if_ir) + (assign2590_e2017 * locals.var_if_ir_dn0)) + locals.var_dif_dvd_dn0) - locals.var_dirprime_dvd_dn0)));
        locals.var_gds_dn1 = ((locals.var_ispec_dn1 * assign2590_e2023) + (locals.var_ispec * (((((locals.var_dn_dvd_dn1 + locals.var_dbeta_dvd_dn1) * locals.var_if_ir) + (assign2590_e2017 * locals.var_if_ir_dn1)) + locals.var_dif_dvd_dn1) - locals.var_dirprime_dvd_dn1)));
        locals.var_gds_dn2 = ((locals.var_ispec_dn2 * assign2590_e2023) + (locals.var_ispec * (((((locals.var_dn_dvd_dn2 + locals.var_dbeta_dvd_dn2) * locals.var_if_ir) + (assign2590_e2017 * locals.var_if_ir_dn2)) + locals.var_dif_dvd_dn2) - locals.var_dirprime_dvd_dn2)));
        locals.var_gds_dn3 = ((locals.var_ispec_dn3 * assign2590_e2023) + (locals.var_ispec * (((((locals.var_dn_dvd_dn3 + locals.var_dbeta_dvd_dn3) * locals.var_if_ir) + (assign2590_e2017 * locals.var_if_ir_dn3)) + locals.var_dif_dvd_dn3) - locals.var_dirprime_dvd_dn3)));

        let assign2600_e2026: f64 = (-locals.var_ispec);
        let assign2600_e2029: f64 = (locals.var_dn_dvs + locals.var_dbeta_dvs);
        let assign2600_e2031: f64 = (assign2600_e2029 * locals.var_if_ir);
        let assign2600_e2033: f64 = (assign2600_e2031 + locals.var_dif_dvs);
        let assign2600_e2035: f64 = (assign2600_e2033 - locals.var_dirprime_dvs);
        let assign2600_e2036: f64 = (assign2600_e2026 * assign2600_e2035);
        locals.var_gms = assign2600_e2036;
        locals.var_gms_dn0 = (((-locals.var_ispec_dn0) * assign2600_e2035) + (assign2600_e2026 * (((((locals.var_dn_dvs_dn0 + locals.var_dbeta_dvs_dn0) * locals.var_if_ir) + (assign2600_e2029 * locals.var_if_ir_dn0)) + locals.var_dif_dvs_dn0) - locals.var_dirprime_dvs_dn0)));
        locals.var_gms_dn1 = (((-locals.var_ispec_dn1) * assign2600_e2035) + (assign2600_e2026 * (((((locals.var_dn_dvs_dn1 + locals.var_dbeta_dvs_dn1) * locals.var_if_ir) + (assign2600_e2029 * locals.var_if_ir_dn1)) + locals.var_dif_dvs_dn1) - locals.var_dirprime_dvs_dn1)));
        locals.var_gms_dn2 = (((-locals.var_ispec_dn2) * assign2600_e2035) + (assign2600_e2026 * (((((locals.var_dn_dvs_dn2 + locals.var_dbeta_dvs_dn2) * locals.var_if_ir) + (assign2600_e2029 * locals.var_if_ir_dn2)) + locals.var_dif_dvs_dn2) - locals.var_dirprime_dvs_dn2)));
        locals.var_gms_dn3 = (((-locals.var_ispec_dn3) * assign2600_e2035) + (assign2600_e2026 * (((((locals.var_dn_dvs_dn3 + locals.var_dbeta_dvs_dn3) * locals.var_if_ir) + (assign2600_e2029 * locals.var_if_ir_dn3)) + locals.var_dif_dvs_dn3) - locals.var_dirprime_dvs_dn3)));

        let assign2630_e2055: f64 = (p.p36 * p.p37);
        let assign2630_e2058: f64 = (locals.var_weff - p.p27);
        let assign2630_e2059: f64 = (assign2630_e2055 / assign2630_e2058);
        locals.var_rseff = assign2630_e2059;

        let assign2640_e2062: f64 = (p.p36 * p.p37);
        let assign2640_e2065: f64 = (locals.var_weff - p.p27);
        let assign2640_e2066: f64 = (assign2640_e2062 / assign2640_e2065);
        locals.var_rdeff = assign2640_e2066;

        let assign2650_e2071: f64 = (locals.var_gms * locals.var_rseff);
        let assign2650_e2072: f64 = (1.0 + assign2650_e2071);
        let assign2650_e2075: f64 = (locals.var_gds * locals.var_rdeff);
        let assign2650_e2076: f64 = (assign2650_e2072 + assign2650_e2075);
        let assign2650_e2077: f64 = (1.0 / assign2650_e2076);
        locals.var_tmp1 = assign2650_e2077;
        locals.var_tmp1_dn0 = (-(((locals.var_gms_dn0 * locals.var_rseff) + (locals.var_gds_dn0 * locals.var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));
        locals.var_tmp1_dn1 = (-(((locals.var_gms_dn1 * locals.var_rseff) + (locals.var_gds_dn1 * locals.var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));
        locals.var_tmp1_dn2 = (-(((locals.var_gms_dn2 * locals.var_rseff) + (locals.var_gds_dn2 * locals.var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));
        locals.var_tmp1_dn3 = (-(((locals.var_gms_dn3 * locals.var_rseff) + (locals.var_gds_dn3 * locals.var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));

        let assign2800_e2163: f64 = (locals.var_weff * locals.var_leff);
        let assign2800_e2165: f64 = (assign2800_e2163 * p.p13);
        locals.var_wlcox = assign2800_e2165;

        let assign2810_e2168: f64 = (locals.var_sif * locals.var_sif2);
        locals.var_sif3 = assign2810_e2168;
        locals.var_sif3_dn0 = ((locals.var_sif_dn0 * locals.var_sif2) + (locals.var_sif * locals.var_sif2_dn0));
        locals.var_sif3_dn1 = ((locals.var_sif_dn1 * locals.var_sif2) + (locals.var_sif * locals.var_sif2_dn1));
        locals.var_sif3_dn2 = ((locals.var_sif_dn2 * locals.var_sif2) + (locals.var_sif * locals.var_sif2_dn2));
        locals.var_sif3_dn3 = ((locals.var_sif_dn3 * locals.var_sif2) + (locals.var_sif * locals.var_sif2_dn3));

        let assign2820_e2171: f64 = (locals.var_sir * locals.var_sir2);
        locals.var_sir3 = assign2820_e2171;
        locals.var_sir3_dn0 = ((locals.var_sir_dn0 * locals.var_sir2) + (locals.var_sir * locals.var_sir2_dn0));
        locals.var_sir3_dn1 = ((locals.var_sir_dn1 * locals.var_sir2) + (locals.var_sir * locals.var_sir2_dn1));
        locals.var_sir3_dn2 = ((locals.var_sir_dn2 * locals.var_sir2) + (locals.var_sir * locals.var_sir2_dn2));
        locals.var_sir3_dn3 = ((locals.var_sir_dn3 * locals.var_sir2) + (locals.var_sir * locals.var_sir2_dn3));

        let assign2830_e2175: f64 = (0.5 * locals.var_vp);
        let assign2830_e2176: f64 = (locals.var_phi_t + assign2830_e2175);
        let assign2830_e2177: f64 = (assign2830_e2176).sqrt();
        locals.var_tmp1 = assign2830_e2177;
        locals.var_tmp1_dn0 = ((locals.var_phi_t_dn0 + (0.5 * locals.var_vp_dn0)) / (2.0 * assign2830_e2177));
        locals.var_tmp1_dn1 = ((locals.var_phi_t_dn1 + (0.5 * locals.var_vp_dn1)) / (2.0 * assign2830_e2177));
        locals.var_tmp1_dn2 = ((locals.var_phi_t_dn2 + (0.5 * locals.var_vp_dn2)) / (2.0 * assign2830_e2177));
        locals.var_tmp1_dn3 = ((locals.var_phi_t_dn3 + (0.5 * locals.var_vp_dn3)) / (2.0 * assign2830_e2177));

        let assign2840_e2180: f64 = (locals.var_tmp1 + locals.var_tmp1);
        locals.var_sqrt_phi_vp2_2 = assign2840_e2180;
        locals.var_sqrt_phi_vp2_2_dn0 = (locals.var_tmp1_dn0 + locals.var_tmp1_dn0);
        locals.var_sqrt_phi_vp2_2_dn1 = (locals.var_tmp1_dn1 + locals.var_tmp1_dn1);
        locals.var_sqrt_phi_vp2_2_dn2 = (locals.var_tmp1_dn2 + locals.var_tmp1_dn2);
        locals.var_sqrt_phi_vp2_2_dn3 = (locals.var_tmp1_dn3 + locals.var_tmp1_dn3);

        let assign2850_e2184: f64 = (locals.var_gammaprime / locals.var_sqrt_phi_vp2_2);
        let assign2850_e2185: f64 = (1.0 + assign2850_e2184);
        let assign2850_e2187: f64 = (assign2850_e2185 * locals.var_vt);
        let assign2850_e2189: f64 = (assign2850_e2187 * locals.var_wlcox);
        locals.var_n_vt_cox = assign2850_e2189;
        locals.var_n_vt_cox_dn0 = (((((locals.var_gammaprime_dn0 * locals.var_sqrt_phi_vp2_2) - (locals.var_gammaprime * locals.var_sqrt_phi_vp2_2_dn0)) / (locals.var_sqrt_phi_vp2_2 * locals.var_sqrt_phi_vp2_2)) * locals.var_vt) * locals.var_wlcox);
        locals.var_n_vt_cox_dn1 = (((((locals.var_gammaprime_dn1 * locals.var_sqrt_phi_vp2_2) - (locals.var_gammaprime * locals.var_sqrt_phi_vp2_2_dn1)) / (locals.var_sqrt_phi_vp2_2 * locals.var_sqrt_phi_vp2_2)) * locals.var_vt) * locals.var_wlcox);
        locals.var_n_vt_cox_dn2 = (((((locals.var_gammaprime_dn2 * locals.var_sqrt_phi_vp2_2) - (locals.var_gammaprime * locals.var_sqrt_phi_vp2_2_dn2)) / (locals.var_sqrt_phi_vp2_2 * locals.var_sqrt_phi_vp2_2)) * locals.var_vt) * locals.var_wlcox);
        locals.var_n_vt_cox_dn3 = (((((locals.var_gammaprime_dn3 * locals.var_sqrt_phi_vp2_2) - (locals.var_gammaprime * locals.var_sqrt_phi_vp2_2_dn3)) / (locals.var_sqrt_phi_vp2_2 * locals.var_sqrt_phi_vp2_2)) * locals.var_vt) * locals.var_wlcox);

        let assign2860_e2191: f64 = (-locals.var_n_vt_cox);
        let assign2860_e2195: f64 = (3.0 * locals.var_sir3);
        let assign2860_e2198: f64 = (6.0 * locals.var_sir2);
        let assign2860_e2200: f64 = (assign2860_e2198 * locals.var_sif);
        let assign2860_e2201: f64 = (assign2860_e2195 + assign2860_e2200);
        let assign2860_e2204: f64 = (4.0 * locals.var_sir);
        let assign2860_e2206: f64 = (assign2860_e2204 * locals.var_sif2);
        let assign2860_e2207: f64 = (assign2860_e2201 + assign2860_e2206);
        let assign2860_e2210: f64 = (2.0 * locals.var_sif3);
        let assign2860_e2211: f64 = (assign2860_e2207 + assign2860_e2210);
        let assign2860_e2212: f64 = (0.266666666 * assign2860_e2211);
        let assign2860_e2214: f64 = (assign2860_e2212 / locals.var_sif_sir_2);
        let assign2860_e2216: f64 = (assign2860_e2214 - 0.5);
        let assign2860_e2217: f64 = (assign2860_e2191 * assign2860_e2216);
        locals.var_qd = assign2860_e2217;
        locals.var_qd_dn0 = (((-locals.var_n_vt_cox_dn0) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * locals.var_sir3_dn0) + (((6.0 * locals.var_sir2_dn0) * locals.var_sif) + (assign2860_e2198 * locals.var_sif_dn0))) + (((4.0 * locals.var_sir_dn0) * locals.var_sif2) + (assign2860_e2204 * locals.var_sif2_dn0))) + (2.0 * locals.var_sif3_dn0))) * locals.var_sif_sir_2) - (assign2860_e2212 * locals.var_sif_sir_2_dn0)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));
        locals.var_qd_dn1 = (((-locals.var_n_vt_cox_dn1) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * locals.var_sir3_dn1) + (((6.0 * locals.var_sir2_dn1) * locals.var_sif) + (assign2860_e2198 * locals.var_sif_dn1))) + (((4.0 * locals.var_sir_dn1) * locals.var_sif2) + (assign2860_e2204 * locals.var_sif2_dn1))) + (2.0 * locals.var_sif3_dn1))) * locals.var_sif_sir_2) - (assign2860_e2212 * locals.var_sif_sir_2_dn1)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));
        locals.var_qd_dn2 = (((-locals.var_n_vt_cox_dn2) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * locals.var_sir3_dn2) + (((6.0 * locals.var_sir2_dn2) * locals.var_sif) + (assign2860_e2198 * locals.var_sif_dn2))) + (((4.0 * locals.var_sir_dn2) * locals.var_sif2) + (assign2860_e2204 * locals.var_sif2_dn2))) + (2.0 * locals.var_sif3_dn2))) * locals.var_sif_sir_2) - (assign2860_e2212 * locals.var_sif_sir_2_dn2)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));
        locals.var_qd_dn3 = (((-locals.var_n_vt_cox_dn3) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * locals.var_sir3_dn3) + (((6.0 * locals.var_sir2_dn3) * locals.var_sif) + (assign2860_e2198 * locals.var_sif_dn3))) + (((4.0 * locals.var_sir_dn3) * locals.var_sif2) + (assign2860_e2204 * locals.var_sif2_dn3))) + (2.0 * locals.var_sif3_dn3))) * locals.var_sif_sir_2) - (assign2860_e2212 * locals.var_sif_sir_2_dn3)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));

        let assign2870_e2219: f64 = (-locals.var_n_vt_cox);
        let assign2870_e2223: f64 = (3.0 * locals.var_sif3);
        let assign2870_e2226: f64 = (6.0 * locals.var_sif2);
        let assign2870_e2228: f64 = (assign2870_e2226 * locals.var_sir);
        let assign2870_e2229: f64 = (assign2870_e2223 + assign2870_e2228);
        let assign2870_e2232: f64 = (4.0 * locals.var_sif);
        let assign2870_e2234: f64 = (assign2870_e2232 * locals.var_sir2);
        let assign2870_e2235: f64 = (assign2870_e2229 + assign2870_e2234);
        let assign2870_e2238: f64 = (2.0 * locals.var_sir3);
        let assign2870_e2239: f64 = (assign2870_e2235 + assign2870_e2238);
        let assign2870_e2240: f64 = (0.266666666 * assign2870_e2239);
        let assign2870_e2242: f64 = (assign2870_e2240 / locals.var_sif_sir_2);
        let assign2870_e2244: f64 = (assign2870_e2242 - 0.5);
        let assign2870_e2245: f64 = (assign2870_e2219 * assign2870_e2244);
        locals.var_qs = assign2870_e2245;
        locals.var_qs_dn0 = (((-locals.var_n_vt_cox_dn0) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * locals.var_sif3_dn0) + (((6.0 * locals.var_sif2_dn0) * locals.var_sir) + (assign2870_e2226 * locals.var_sir_dn0))) + (((4.0 * locals.var_sif_dn0) * locals.var_sir2) + (assign2870_e2232 * locals.var_sir2_dn0))) + (2.0 * locals.var_sir3_dn0))) * locals.var_sif_sir_2) - (assign2870_e2240 * locals.var_sif_sir_2_dn0)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));
        locals.var_qs_dn1 = (((-locals.var_n_vt_cox_dn1) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * locals.var_sif3_dn1) + (((6.0 * locals.var_sif2_dn1) * locals.var_sir) + (assign2870_e2226 * locals.var_sir_dn1))) + (((4.0 * locals.var_sif_dn1) * locals.var_sir2) + (assign2870_e2232 * locals.var_sir2_dn1))) + (2.0 * locals.var_sir3_dn1))) * locals.var_sif_sir_2) - (assign2870_e2240 * locals.var_sif_sir_2_dn1)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));
        locals.var_qs_dn2 = (((-locals.var_n_vt_cox_dn2) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * locals.var_sif3_dn2) + (((6.0 * locals.var_sif2_dn2) * locals.var_sir) + (assign2870_e2226 * locals.var_sir_dn2))) + (((4.0 * locals.var_sif_dn2) * locals.var_sir2) + (assign2870_e2232 * locals.var_sir2_dn2))) + (2.0 * locals.var_sir3_dn2))) * locals.var_sif_sir_2) - (assign2870_e2240 * locals.var_sif_sir_2_dn2)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));
        locals.var_qs_dn3 = (((-locals.var_n_vt_cox_dn3) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * locals.var_sif3_dn3) + (((6.0 * locals.var_sif2_dn3) * locals.var_sir) + (assign2870_e2226 * locals.var_sir_dn3))) + (((4.0 * locals.var_sif_dn3) * locals.var_sir2) + (assign2870_e2232 * locals.var_sir2_dn3))) + (2.0 * locals.var_sir3_dn3))) * locals.var_sif_sir_2) - (assign2870_e2240 * locals.var_sif_sir_2_dn3)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));

        let assign2880_e2248: f64 = (locals.var_qs + locals.var_qd);
        locals.var_qi_1 = assign2880_e2248;
        locals.var_qi_1_dn0 = (locals.var_qs_dn0 + locals.var_qd_dn0);
        locals.var_qi_1_dn1 = (locals.var_qs_dn1 + locals.var_qd_dn1);
        locals.var_qi_1_dn2 = (locals.var_qs_dn2 + locals.var_qd_dn2);
        locals.var_qi_1_dn3 = (locals.var_qs_dn3 + locals.var_qd_dn3);

        let assign2890_e2251: f64 = (-0.5);
        let assign2890_e2253: f64 = (assign2890_e2251 * locals.var_gammaprime);
        let assign2890_e2255: f64 = (assign2890_e2253 * locals.var_sqrt_phi_vp_2);
        let assign2890_e2257: f64 = (assign2890_e2255 + locals.var_vgprime);
        let assign2890_e2259: f64 = (assign2890_e2257 - locals.var_vgstar);
        let assign2890_e2260: f64 = (locals.var_wlcox * assign2890_e2259);
        let assign2890_e2263: f64 = (locals.var_qi_1 * locals.var_gammaprime);
        let assign2890_e2266: f64 = (locals.var_gammaprime + locals.var_sqrt_phi_vp2_2);
        let assign2890_e2267: f64 = (assign2890_e2263 / assign2890_e2266);
        let assign2890_e2268: f64 = (assign2890_e2260 - assign2890_e2267);
        locals.var_qb_1 = assign2890_e2268;
        locals.var_qb_1_dn0 = ((locals.var_wlcox * (((((assign2890_e2251 * locals.var_gammaprime_dn0) * locals.var_sqrt_phi_vp_2) + (assign2890_e2253 * locals.var_sqrt_phi_vp_2_dn0)) + locals.var_vgprime_dn0) - locals.var_vgstar_dn0)) - (((((locals.var_qi_1_dn0 * locals.var_gammaprime) + (locals.var_qi_1 * locals.var_gammaprime_dn0)) * assign2890_e2266) - (assign2890_e2263 * (locals.var_gammaprime_dn0 + locals.var_sqrt_phi_vp2_2_dn0))) / (assign2890_e2266 * assign2890_e2266)));
        locals.var_qb_1_dn1 = ((locals.var_wlcox * (((((assign2890_e2251 * locals.var_gammaprime_dn1) * locals.var_sqrt_phi_vp_2) + (assign2890_e2253 * locals.var_sqrt_phi_vp_2_dn1)) + locals.var_vgprime_dn1) - locals.var_vgstar_dn1)) - (((((locals.var_qi_1_dn1 * locals.var_gammaprime) + (locals.var_qi_1 * locals.var_gammaprime_dn1)) * assign2890_e2266) - (assign2890_e2263 * (locals.var_gammaprime_dn1 + locals.var_sqrt_phi_vp2_2_dn1))) / (assign2890_e2266 * assign2890_e2266)));
        locals.var_qb_1_dn2 = ((locals.var_wlcox * (((((assign2890_e2251 * locals.var_gammaprime_dn2) * locals.var_sqrt_phi_vp_2) + (assign2890_e2253 * locals.var_sqrt_phi_vp_2_dn2)) + locals.var_vgprime_dn2) - locals.var_vgstar_dn2)) - (((((locals.var_qi_1_dn2 * locals.var_gammaprime) + (locals.var_qi_1 * locals.var_gammaprime_dn2)) * assign2890_e2266) - (assign2890_e2263 * (locals.var_gammaprime_dn2 + locals.var_sqrt_phi_vp2_2_dn2))) / (assign2890_e2266 * assign2890_e2266)));
        locals.var_qb_1_dn3 = ((locals.var_wlcox * (((((assign2890_e2251 * locals.var_gammaprime_dn3) * locals.var_sqrt_phi_vp_2) + (assign2890_e2253 * locals.var_sqrt_phi_vp_2_dn3)) + locals.var_vgprime_dn3) - locals.var_vgstar_dn3)) - (((((locals.var_qi_1_dn3 * locals.var_gammaprime) + (locals.var_qi_1 * locals.var_gammaprime_dn3)) * assign2890_e2266) - (assign2890_e2263 * (locals.var_gammaprime_dn3 + locals.var_sqrt_phi_vp2_2_dn3))) / (assign2890_e2266 * assign2890_e2266)));

        let assign2900_e2270: f64 = (-locals.var_qi_1);
        let assign2900_e2272: f64 = (assign2900_e2270 - locals.var_qb_1);
        locals.var_qg = assign2900_e2272;
        locals.var_qg_dn0 = ((-locals.var_qi_1_dn0) - locals.var_qb_1_dn0);
        locals.var_qg_dn1 = ((-locals.var_qi_1_dn1) - locals.var_qb_1_dn1);
        locals.var_qg_dn2 = ((-locals.var_qi_1_dn2) - locals.var_qb_1_dn2);
        locals.var_qg_dn3 = ((-locals.var_qi_1_dn3) - locals.var_qb_1_dn3);

        let assign2910_e2274: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qd);
        locals.var_ddt_qd = assign2910_e2274;
        locals.var_ddt_qd_dn0 = (locals.var_qd_dn0 * ddt_scale);
        locals.var_ddt_qd_dn1 = (locals.var_qd_dn1 * ddt_scale);
        locals.var_ddt_qd_dn2 = (locals.var_qd_dn2 * ddt_scale);
        locals.var_ddt_qd_dn3 = (locals.var_qd_dn3 * ddt_scale);

        let assign2920_e2276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_qs);
        locals.var_ddt_qs = assign2920_e2276;
        locals.var_ddt_qs_dn0 = (locals.var_qs_dn0 * ddt_scale);
        locals.var_ddt_qs_dn1 = (locals.var_qs_dn1 * ddt_scale);
        locals.var_ddt_qs_dn2 = (locals.var_qs_dn2 * ddt_scale);
        locals.var_ddt_qs_dn3 = (locals.var_qs_dn3 * ddt_scale);

        let assign2930_e2279: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign2930_e2279;

        let assign2960_e2312: f64 = if ((p.p9 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard24 = assign2960_e2312;

        let (assign2970_e2320,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2970_e2316: f64 = (2.0 * p.p37);
        let assign2970_e2318: f64 = (assign2970_e2316 * locals.var_weff);
        (assign2970_e2318,)
    } else {
        (locals.var_as_i,)
    }
};
        locals.var_as_i = assign2970_e2320;

        let (assign2980_e2325,) = {
    if (locals.var_guard24 == 0.0) {
        (p.p9,)
    } else {
        (locals.var_as_i,)
    }
};
        locals.var_as_i = assign2980_e2325;

        let assign2990_e2332: f64 = if ((p.p11 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard25 = assign2990_e2332;

        let (assign3000_e2342,) = {
    if (locals.var_guard25 != 0.0) {
        let assign3000_e2336: f64 = (4.0 * p.p37);
        let assign3000_e2339: f64 = locals.var_weff;
        let assign3000_e2340: f64 = (assign3000_e2336 + assign3000_e2339);
        (assign3000_e2340,)
    } else {
        (locals.var_ps_i,)
    }
};
        locals.var_ps_i = assign3000_e2342;

        let (assign3010_e2347,) = {
    if (locals.var_guard25 == 0.0) {
        (p.p11,)
    } else {
        (locals.var_ps_i,)
    }
};
        locals.var_ps_i = assign3010_e2347;

        let assign3020_e2354: f64 = if ((p.p10 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard26 = assign3020_e2354;

        let (assign3030_e2362,) = {
    if (locals.var_guard26 != 0.0) {
        let assign3030_e2358: f64 = (2.0 * p.p37);
        let assign3030_e2360: f64 = (assign3030_e2358 * locals.var_weff);
        (assign3030_e2360,)
    } else {
        (locals.var_ad_i,)
    }
};
        locals.var_ad_i = assign3030_e2362;

        let (assign3040_e2367,) = {
    if (locals.var_guard26 == 0.0) {
        (p.p10,)
    } else {
        (locals.var_ad_i,)
    }
};
        locals.var_ad_i = assign3040_e2367;

        let assign3050_e2374: f64 = if ((p.p12 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard27 = assign3050_e2374;

        let (assign3060_e2384,) = {
    if (locals.var_guard27 != 0.0) {
        let assign3060_e2378: f64 = (4.0 * p.p37);
        let assign3060_e2381: f64 = locals.var_weff;
        let assign3060_e2382: f64 = (assign3060_e2378 + assign3060_e2381);
        (assign3060_e2382,)
    } else {
        (locals.var_pd_i,)
    }
};
        locals.var_pd_i = assign3060_e2384;

        let (assign3070_e2389,) = {
    if (locals.var_guard27 == 0.0) {
        (p.p12,)
    } else {
        (locals.var_pd_i,)
    }
};
        locals.var_pd_i = assign3070_e2389;

        let assign3120_e2418: f64 = (p.p69 * locals.var_deltat);
        let assign3120_e2419: f64 = (p.p50 - assign3120_e2418);
        locals.var_pb_t = assign3120_e2419;

        let assign3130_e2423: f64 = (p.p70 * locals.var_deltat);
        let assign3130_e2424: f64 = (p.p51 - assign3130_e2423);
        locals.var_pbsw_t = assign3130_e2424;

        let assign3140_e2428: f64 = (p.p71 * locals.var_deltat);
        let assign3140_e2429: f64 = (p.p52 - assign3140_e2428);
        locals.var_pbswg_t = assign3140_e2429;

        let assign3150_e2434: f64 = (p.p66 * locals.var_deltat);
        let assign3150_e2435: f64 = (1.0 + assign3150_e2434);
        let assign3150_e2436: f64 = (p.p53 * assign3150_e2435);
        locals.var_cj_t = assign3150_e2436;

        let assign3160_e2441: f64 = (p.p67 * locals.var_deltat);
        let assign3160_e2442: f64 = (1.0 + assign3160_e2441);
        let assign3160_e2443: f64 = (p.p54 * assign3160_e2442);
        locals.var_cjsw_t = assign3160_e2443;

        let assign3170_e2448: f64 = (p.p68 * locals.var_deltat);
        let assign3170_e2449: f64 = (1.0 + assign3170_e2448);
        let assign3170_e2450: f64 = (p.p55 * assign3170_e2449);
        locals.var_cjswg_t = assign3170_e2450;

        let assign3210_e2480: f64 = (p.p0 * (nv0 - nv3));
        locals.var_v_di_b = assign3210_e2480;
        locals.var_v_di_b_dn0 = p.p0;
        locals.var_v_di_b_dn3 = (-p.p0);

        let assign3220_e2483: f64 = (p.p0 * (nv2 - nv3));
        locals.var_v_si_b = assign3220_e2483;
        locals.var_v_si_b_dn2 = p.p0;
        locals.var_v_si_b_dn3 = (-p.p0);

        let assign3450_e2740: f64 = if locals.var_v_di_b > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign3450_e2740;

        let (assign3460_e2757, assign3460_e2757_d_n0, assign3460_e2757_d_n3,) = {
    if (locals.var_guard32 != 0.0) {
        let assign3460_e2744: f64 = (locals.var_cj_t * locals.var_ad_i);
        let assign3460_e2746: f64 = (-p.p47);
        let assign3460_e2750: f64 = (locals.var_v_di_b / locals.var_pb_t);
        let assign3460_e2751: f64 = (1.0 + assign3460_e2750);
        let assign3460_e2752: f64 = (assign3460_e2751).ln();
        let assign3460_e2753: f64 = (assign3460_e2746 * assign3460_e2752);
        let assign3460_e2754: f64 = (assign3460_e2753).exp();
        let assign3460_e2755: f64 = (assign3460_e2744 * assign3460_e2754);
        (assign3460_e2755, (assign3460_e2744 * (assign3460_e2754 * (assign3460_e2746 * ((locals.var_v_di_b_dn0 / locals.var_pb_t) / assign3460_e2751)))), (assign3460_e2744 * (assign3460_e2754 * (assign3460_e2746 * ((locals.var_v_di_b_dn3 / locals.var_pb_t) / assign3460_e2751)))),)
    } else {
        (locals.var_csb_d, locals.var_csb_d_dn0, locals.var_csb_d_dn3,)
    }
};
        locals.var_csb_d = assign3460_e2757;
        locals.var_csb_d_dn0 = assign3460_e2757_d_n0;
        locals.var_csb_d_dn3 = assign3460_e2757_d_n3;

        let (assign3470_e2774, assign3470_e2774_d_n0, assign3470_e2774_d_n3,) = {
    if (locals.var_guard32 != 0.0) {
        let assign3470_e2761: f64 = (locals.var_cjsw_t * locals.var_pd_i);
        let assign3470_e2763: f64 = (-p.p48);
        let assign3470_e2767: f64 = (locals.var_v_di_b / locals.var_pbsw_t);
        let assign3470_e2768: f64 = (1.0 + assign3470_e2767);
        let assign3470_e2769: f64 = (assign3470_e2768).ln();
        let assign3470_e2770: f64 = (assign3470_e2763 * assign3470_e2769);
        let assign3470_e2771: f64 = (assign3470_e2770).exp();
        let assign3470_e2772: f64 = (assign3470_e2761 * assign3470_e2771);
        (assign3470_e2772, (assign3470_e2761 * (assign3470_e2771 * (assign3470_e2763 * ((locals.var_v_di_b_dn0 / locals.var_pbsw_t) / assign3470_e2768)))), (assign3470_e2761 * (assign3470_e2771 * (assign3470_e2763 * ((locals.var_v_di_b_dn3 / locals.var_pbsw_t) / assign3470_e2768)))),)
    } else {
        (locals.var_cssw_d, locals.var_cssw_d_dn0, locals.var_cssw_d_dn3,)
    }
};
        locals.var_cssw_d = assign3470_e2774;
        locals.var_cssw_d_dn0 = assign3470_e2774_d_n0;
        locals.var_cssw_d_dn3 = assign3470_e2774_d_n3;

        let (assign3480_e2791, assign3480_e2791_d_n0, assign3480_e2791_d_n3,) = {
    if (locals.var_guard32 != 0.0) {
        let assign3480_e2778: f64 = (locals.var_cjswg_t * locals.var_weff);
        let assign3480_e2780: f64 = (-p.p49);
        let assign3480_e2784: f64 = (locals.var_v_di_b / locals.var_pbswg_t);
        let assign3480_e2785: f64 = (1.0 + assign3480_e2784);
        let assign3480_e2786: f64 = (assign3480_e2785).ln();
        let assign3480_e2787: f64 = (assign3480_e2780 * assign3480_e2786);
        let assign3480_e2788: f64 = (assign3480_e2787).exp();
        let assign3480_e2789: f64 = (assign3480_e2778 * assign3480_e2788);
        (assign3480_e2789, (assign3480_e2778 * (assign3480_e2788 * (assign3480_e2780 * ((locals.var_v_di_b_dn0 / locals.var_pbswg_t) / assign3480_e2785)))), (assign3480_e2778 * (assign3480_e2788 * (assign3480_e2780 * ((locals.var_v_di_b_dn3 / locals.var_pbswg_t) / assign3480_e2785)))),)
    } else {
        (locals.var_csswg_d, locals.var_csswg_d_dn0, locals.var_csswg_d_dn3,)
    }
};
        locals.var_csswg_d = assign3480_e2791;
        locals.var_csswg_d_dn0 = assign3480_e2791_d_n0;
        locals.var_csswg_d_dn3 = assign3480_e2791_d_n3;

        let (assign3490_e2806, assign3490_e2806_d_n0, assign3490_e2806_d_n3,) = {
    if (locals.var_guard32 == 0.0) {
        let assign3490_e2796: f64 = (locals.var_cj_t * locals.var_ad_i);
        let assign3490_e2800: f64 = (p.p47 * locals.var_v_di_b);
        let assign3490_e2802: f64 = (assign3490_e2800 / locals.var_pb_t);
        let assign3490_e2803: f64 = (1.0 - assign3490_e2802);
        let assign3490_e2804: f64 = (assign3490_e2796 * assign3490_e2803);
        (assign3490_e2804, (assign3490_e2796 * (-((p.p47 * locals.var_v_di_b_dn0) / locals.var_pb_t))), (assign3490_e2796 * (-((p.p47 * locals.var_v_di_b_dn3) / locals.var_pb_t))),)
    } else {
        (locals.var_csb_d, locals.var_csb_d_dn0, locals.var_csb_d_dn3,)
    }
};
        locals.var_csb_d = assign3490_e2806;
        locals.var_csb_d_dn0 = assign3490_e2806_d_n0;
        locals.var_csb_d_dn3 = assign3490_e2806_d_n3;

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3500_e2821, assign3500_e2821_d_n0, assign3500_e2821_d_n3,) = {
    if (locals.var_guard32 == 0.0) {
        let assign3500_e2811: f64 = (locals.var_cjsw_t * locals.var_pd_i);
        let assign3500_e2815: f64 = (p.p48 * locals.var_v_di_b);
        let assign3500_e2817: f64 = (assign3500_e2815 / locals.var_pbsw_t);
        let assign3500_e2818: f64 = (1.0 - assign3500_e2817);
        let assign3500_e2819: f64 = (assign3500_e2811 * assign3500_e2818);
        (assign3500_e2819, (assign3500_e2811 * (-((p.p48 * locals.var_v_di_b_dn0) / locals.var_pbsw_t))), (assign3500_e2811 * (-((p.p48 * locals.var_v_di_b_dn3) / locals.var_pbsw_t))),)
    } else {
        (locals.var_cssw_d, locals.var_cssw_d_dn0, locals.var_cssw_d_dn3,)
    }
};
        locals.var_cssw_d = assign3500_e2821;
        locals.var_cssw_d_dn0 = assign3500_e2821_d_n0;
        locals.var_cssw_d_dn3 = assign3500_e2821_d_n3;

        let (assign3510_e2836, assign3510_e2836_d_n0, assign3510_e2836_d_n3,) = {
    if (locals.var_guard32 == 0.0) {
        let assign3510_e2826: f64 = (locals.var_cjswg_t * locals.var_weff);
        let assign3510_e2830: f64 = (p.p49 * locals.var_v_di_b);
        let assign3510_e2832: f64 = (assign3510_e2830 / locals.var_pbswg_t);
        let assign3510_e2833: f64 = (1.0 - assign3510_e2832);
        let assign3510_e2834: f64 = (assign3510_e2826 * assign3510_e2833);
        (assign3510_e2834, (assign3510_e2826 * (-((p.p49 * locals.var_v_di_b_dn0) / locals.var_pbswg_t))), (assign3510_e2826 * (-((p.p49 * locals.var_v_di_b_dn3) / locals.var_pbswg_t))),)
    } else {
        (locals.var_csswg_d, locals.var_csswg_d_dn0, locals.var_csswg_d_dn3,)
    }
};
        locals.var_csswg_d = assign3510_e2836;
        locals.var_csswg_d_dn0 = assign3510_e2836_d_n0;
        locals.var_csswg_d_dn3 = assign3510_e2836_d_n3;

        let assign3520_e2839: f64 = (locals.var_csb_d + locals.var_cssw_d);
        let assign3520_e2841: f64 = (assign3520_e2839 + locals.var_csswg_d);
        let assign3520_e2843: f64 = (assign3520_e2841 * locals.var_v_di_b);
        locals.var_qjd = assign3520_e2843;
        locals.var_qjd_dn0 = ((((locals.var_csb_d_dn0 + locals.var_cssw_d_dn0) + locals.var_csswg_d_dn0) * locals.var_v_di_b) + (assign3520_e2841 * locals.var_v_di_b_dn0));
        locals.var_qjd_dn3 = ((((locals.var_csb_d_dn3 + locals.var_cssw_d_dn3) + locals.var_csswg_d_dn3) * locals.var_v_di_b) + (assign3520_e2841 * locals.var_v_di_b_dn3));

        let assign3530_e2846: f64 = if locals.var_v_si_b > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign3530_e2846;

        let (assign3540_e2863, assign3540_e2863_d_n2, assign3540_e2863_d_n3,) = {
    if (locals.var_guard33 != 0.0) {
        let assign3540_e2850: f64 = (locals.var_cj_t * locals.var_as_i);
        let assign3540_e2852: f64 = (-p.p47);
        let assign3540_e2856: f64 = (locals.var_v_si_b / locals.var_pb_t);
        let assign3540_e2857: f64 = (1.0 + assign3540_e2856);
        let assign3540_e2858: f64 = (assign3540_e2857).ln();
        let assign3540_e2859: f64 = (assign3540_e2852 * assign3540_e2858);
        let assign3540_e2860: f64 = (assign3540_e2859).exp();
        let assign3540_e2861: f64 = (assign3540_e2850 * assign3540_e2860);
        (assign3540_e2861, (assign3540_e2850 * (assign3540_e2860 * (assign3540_e2852 * ((locals.var_v_si_b_dn2 / locals.var_pb_t) / assign3540_e2857)))), (assign3540_e2850 * (assign3540_e2860 * (assign3540_e2852 * ((locals.var_v_si_b_dn3 / locals.var_pb_t) / assign3540_e2857)))),)
    } else {
        (locals.var_csb_s, locals.var_csb_s_dn2, locals.var_csb_s_dn3,)
    }
};
        locals.var_csb_s = assign3540_e2863;
        locals.var_csb_s_dn2 = assign3540_e2863_d_n2;
        locals.var_csb_s_dn3 = assign3540_e2863_d_n3;

        let (assign3550_e2880, assign3550_e2880_d_n2, assign3550_e2880_d_n3,) = {
    if (locals.var_guard33 != 0.0) {
        let assign3550_e2867: f64 = (locals.var_cjsw_t * locals.var_ps_i);
        let assign3550_e2869: f64 = (-p.p48);
        let assign3550_e2873: f64 = (locals.var_v_si_b / locals.var_pbsw_t);
        let assign3550_e2874: f64 = (1.0 + assign3550_e2873);
        let assign3550_e2875: f64 = (assign3550_e2874).ln();
        let assign3550_e2876: f64 = (assign3550_e2869 * assign3550_e2875);
        let assign3550_e2877: f64 = (assign3550_e2876).exp();
        let assign3550_e2878: f64 = (assign3550_e2867 * assign3550_e2877);
        (assign3550_e2878, (assign3550_e2867 * (assign3550_e2877 * (assign3550_e2869 * ((locals.var_v_si_b_dn2 / locals.var_pbsw_t) / assign3550_e2874)))), (assign3550_e2867 * (assign3550_e2877 * (assign3550_e2869 * ((locals.var_v_si_b_dn3 / locals.var_pbsw_t) / assign3550_e2874)))),)
    } else {
        (locals.var_cssw_s, locals.var_cssw_s_dn2, locals.var_cssw_s_dn3,)
    }
};
        locals.var_cssw_s = assign3550_e2880;
        locals.var_cssw_s_dn2 = assign3550_e2880_d_n2;
        locals.var_cssw_s_dn3 = assign3550_e2880_d_n3;

        let (assign3560_e2897, assign3560_e2897_d_n2, assign3560_e2897_d_n3,) = {
    if (locals.var_guard33 != 0.0) {
        let assign3560_e2884: f64 = (locals.var_cjswg_t * locals.var_weff);
        let assign3560_e2886: f64 = (-p.p49);
        let assign3560_e2890: f64 = (locals.var_v_si_b / locals.var_pbswg_t);
        let assign3560_e2891: f64 = (1.0 + assign3560_e2890);
        let assign3560_e2892: f64 = (assign3560_e2891).ln();
        let assign3560_e2893: f64 = (assign3560_e2886 * assign3560_e2892);
        let assign3560_e2894: f64 = (assign3560_e2893).exp();
        let assign3560_e2895: f64 = (assign3560_e2884 * assign3560_e2894);
        (assign3560_e2895, (assign3560_e2884 * (assign3560_e2894 * (assign3560_e2886 * ((locals.var_v_si_b_dn2 / locals.var_pbswg_t) / assign3560_e2891)))), (assign3560_e2884 * (assign3560_e2894 * (assign3560_e2886 * ((locals.var_v_si_b_dn3 / locals.var_pbswg_t) / assign3560_e2891)))),)
    } else {
        (locals.var_csswg_s, locals.var_csswg_s_dn2, locals.var_csswg_s_dn3,)
    }
};
        locals.var_csswg_s = assign3560_e2897;
        locals.var_csswg_s_dn2 = assign3560_e2897_d_n2;
        locals.var_csswg_s_dn3 = assign3560_e2897_d_n3;

        let (assign3570_e2912, assign3570_e2912_d_n2, assign3570_e2912_d_n3,) = {
    if (locals.var_guard33 == 0.0) {
        let assign3570_e2902: f64 = (locals.var_cj_t * locals.var_as_i);
        let assign3570_e2906: f64 = (p.p47 * locals.var_v_si_b);
        let assign3570_e2908: f64 = (assign3570_e2906 / locals.var_pb_t);
        let assign3570_e2909: f64 = (1.0 - assign3570_e2908);
        let assign3570_e2910: f64 = (assign3570_e2902 * assign3570_e2909);
        (assign3570_e2910, (assign3570_e2902 * (-((p.p47 * locals.var_v_si_b_dn2) / locals.var_pb_t))), (assign3570_e2902 * (-((p.p47 * locals.var_v_si_b_dn3) / locals.var_pb_t))),)
    } else {
        (locals.var_csb_s, locals.var_csb_s_dn2, locals.var_csb_s_dn3,)
    }
};
        locals.var_csb_s = assign3570_e2912;
        locals.var_csb_s_dn2 = assign3570_e2912_d_n2;
        locals.var_csb_s_dn3 = assign3570_e2912_d_n3;

        let (assign3580_e2927, assign3580_e2927_d_n2, assign3580_e2927_d_n3,) = {
    if (locals.var_guard33 == 0.0) {
        let assign3580_e2917: f64 = (locals.var_cjsw_t * locals.var_ps_i);
        let assign3580_e2921: f64 = (p.p48 * locals.var_v_si_b);
        let assign3580_e2923: f64 = (assign3580_e2921 / locals.var_pbsw_t);
        let assign3580_e2924: f64 = (1.0 - assign3580_e2923);
        let assign3580_e2925: f64 = (assign3580_e2917 * assign3580_e2924);
        (assign3580_e2925, (assign3580_e2917 * (-((p.p48 * locals.var_v_si_b_dn2) / locals.var_pbsw_t))), (assign3580_e2917 * (-((p.p48 * locals.var_v_si_b_dn3) / locals.var_pbsw_t))),)
    } else {
        (locals.var_cssw_s, locals.var_cssw_s_dn2, locals.var_cssw_s_dn3,)
    }
};
        locals.var_cssw_s = assign3580_e2927;
        locals.var_cssw_s_dn2 = assign3580_e2927_d_n2;
        locals.var_cssw_s_dn3 = assign3580_e2927_d_n3;

        let (assign3590_e2942, assign3590_e2942_d_n2, assign3590_e2942_d_n3,) = {
    if (locals.var_guard33 == 0.0) {
        let assign3590_e2932: f64 = (locals.var_cjswg_t * locals.var_weff);
        let assign3590_e2936: f64 = (p.p49 * locals.var_v_si_b);
        let assign3590_e2938: f64 = (assign3590_e2936 / locals.var_pbswg_t);
        let assign3590_e2939: f64 = (1.0 - assign3590_e2938);
        let assign3590_e2940: f64 = (assign3590_e2932 * assign3590_e2939);
        (assign3590_e2940, (assign3590_e2932 * (-((p.p49 * locals.var_v_si_b_dn2) / locals.var_pbswg_t))), (assign3590_e2932 * (-((p.p49 * locals.var_v_si_b_dn3) / locals.var_pbswg_t))),)
    } else {
        (locals.var_csswg_s, locals.var_csswg_s_dn2, locals.var_csswg_s_dn3,)
    }
};
        locals.var_csswg_s = assign3590_e2942;
        locals.var_csswg_s_dn2 = assign3590_e2942_d_n2;
        locals.var_csswg_s_dn3 = assign3590_e2942_d_n3;

        let assign3600_e2945: f64 = (locals.var_csb_s + locals.var_cssw_s);
        let assign3600_e2947: f64 = (assign3600_e2945 + locals.var_csswg_s);
        let assign3600_e2949: f64 = (assign3600_e2947 * locals.var_v_si_b);
        locals.var_qjs = assign3600_e2949;
        locals.var_qjs_dn2 = ((((locals.var_csb_s_dn2 + locals.var_cssw_s_dn2) + locals.var_csswg_s_dn2) * locals.var_v_si_b) + (assign3600_e2947 * locals.var_v_si_b_dn2));
        locals.var_qjs_dn3 = ((((locals.var_csb_s_dn3 + locals.var_cssw_s_dn3) + locals.var_csswg_s_dn3) * locals.var_v_si_b) + (assign3600_e2947 * locals.var_v_si_b_dn3));

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let assign10_e194: f64 = (11.7 * 8.8541879239442e-12);
        locals.var_epssil = assign10_e194;
        locals.var_epssil_rv = 0.0;

        locals.var_theta_vp_1 = 0.0;
        locals.var_theta_vp_1_dn0 = 0.0;
        locals.var_theta_vp_1_dn1 = 0.0;
        locals.var_theta_vp_1_dn2 = 0.0;
        locals.var_theta_vp_1_dn3 = 0.0;
        locals.var_theta_vp_1_rv = 0.0;

        locals.var_vpprime = 0.0;
        locals.var_vpprime_dn0 = 0.0;
        locals.var_vpprime_dn1 = 0.0;
        locals.var_vpprime_dn2 = 0.0;
        locals.var_vpprime_dn3 = 0.0;
        locals.var_vpprime_rv = 0.0;

        locals.var_sqrt_vp_vt = 0.0;
        locals.var_sqrt_vp_vt_dn0 = 0.0;
        locals.var_sqrt_vp_vt_dn1 = 0.0;
        locals.var_sqrt_vp_vt_dn2 = 0.0;
        locals.var_sqrt_vp_vt_dn3 = 0.0;
        locals.var_sqrt_vp_vt_rv = 0.0;

        let assign60_e201: f64 = (locals.var_epssil / p.p13);
        locals.var_eps_cox = assign60_e201;
        locals.var_eps_cox_rv = 0.0;

        let assign70_e204: f64 = (locals.var_eps_cox * p.p14);
        let assign70_e205: f64 = (assign70_e204).sqrt();
        locals.var_lc = assign70_e205;
        locals.var_lc_rv = 0.0;

        let assign80_e208: f64 = (locals.var_lc * p.p25);
        locals.var_lc_lambda = assign80_e208;
        locals.var_lc_lambda_rv = 0.0;

        let assign90_e211: f64 = (3.0 * locals.var_eps_cox);
        let assign90_e213: f64 = (assign90_e211 * p.p28);
        locals.var_eps_cox_w = assign90_e213;
        locals.var_eps_cox_w_rv = 0.0;

        let assign100_e216: f64 = (locals.var_eps_cox * p.p29);
        locals.var_eps_cox_l = assign100_e216;
        locals.var_eps_cox_l_rv = 0.0;

        let assign120_e223: f64 = (locals.var_epssil * p.p22);
        let assign120_e224: f64 = (p.p13 / assign120_e223);
        locals.var_t0 = assign120_e224;
        locals.var_t0_rv = 0.0;

        let assign130_e227: f64 = (p.p30 + p.p30);
        let assign130_e229: f64 = (assign130_e227 / p.p13);
        locals.var_v0 = assign130_e229;
        locals.var_v0_rv = 0.0;

        let (assign140_e235,) = {
    if (p.p0 > 0.0) {
        (0.5,)
    } else {
        (0.3333333333333,)
    }
};
        locals.var_eta_qi = assign140_e235;
        locals.var_eta_qi_rv = 0.0;

        let assign150_e238: f64 = (-1e21);
        let assign150_e239: f64 = (-assign150_e238);
        let assign150_e240: f64 = if p.p3 == assign150_e239 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign150_e240;
        locals.var_guard1_rv = 0.0;

        let (assign160_e246,) = {
    if (locals.var_guard1 != 0.0) {
        let assign160_e242: f64 = ctx_temp;
        let assign160_e244: f64 = (assign160_e242 + p.p2);
        (assign160_e244,)
    } else {
        (locals.var_t,)
    }
};
        locals.var_t = assign160_e246;
        locals.var_t_rv = 0.0;

        let (assign170_e253,) = {
    if (locals.var_guard1 == 0.0) {
        let assign170_e251: f64 = (p.p3 + 273.15);
        (assign170_e251,)
    } else {
        (locals.var_t,)
    }
};
        locals.var_t = assign170_e253;
        locals.var_t_rv = 0.0;

        let assign180_e256: f64 = (-1e21);
        let assign180_e257: f64 = (-assign180_e256);
        let assign180_e258: f64 = if p.p4 == assign180_e257 { 1.0 } else { 0.0 };
        locals.var_guard2 = assign180_e258;
        locals.var_guard2_rv = 0.0;

        let (assign190_e264,) = {
    if (locals.var_guard2 != 0.0) {
        let assign190_e262: f64 = (25.0 + 273.15);
        (assign190_e262,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign190_e264;
        locals.var_tnom_rv = 0.0;

        let (assign200_e271,) = {
    if (locals.var_guard2 == 0.0) {
        let assign200_e269: f64 = (p.p4 + 273.15);
        (assign200_e269,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign200_e271;
        locals.var_tnom_rv = 0.0;

        let assign210_e273: f64 = (locals.var_t * THERMAL_VOLTAGE_PER_K);
        locals.var_vt = assign210_e273;
        locals.var_vt_rv = 0.0;

        let assign220_e276: f64 = (0.1 * locals.var_vt);
        locals.var_vt_01 = assign220_e276;
        locals.var_vt_01_rv = 0.0;

        let assign230_e279: f64 = (1.0 / locals.var_vt);
        locals.var_inv_vt = assign230_e279;
        locals.var_inv_vt_rv = 0.0;

        let assign240_e282: f64 = (locals.var_vt + locals.var_vt);
        locals.var_vt_2 = assign240_e282;
        locals.var_vt_2_rv = 0.0;

        let assign250_e285: f64 = (locals.var_vt_2 + locals.var_vt_2);
        locals.var_vt_4 = assign250_e285;
        locals.var_vt_4_rv = 0.0;

        let assign260_e288: f64 = (locals.var_vt * locals.var_vt);
        locals.var_vt_vt = assign260_e288;
        locals.var_vt_vt_rv = 0.0;

        let assign270_e291: f64 = (locals.var_vt_vt + locals.var_vt_vt);
        locals.var_vt_vt_2 = assign270_e291;
        locals.var_vt_vt_2_rv = 0.0;

        let assign280_e294: f64 = (16.0 * locals.var_vt_vt);
        locals.var_vt_vt_16 = assign280_e294;
        locals.var_vt_vt_16_rv = 0.0;

        let assign290_e298: f64 = (0.000702 * locals.var_t);
        let assign290_e300: f64 = (assign290_e298 * locals.var_t);
        let assign290_e303: f64 = (locals.var_t + 1108.0);
        let assign290_e304: f64 = (assign290_e300 / assign290_e303);
        let assign290_e305: f64 = (1.16 - assign290_e304);
        locals.var_eg = assign290_e305;
        locals.var_eg_rv = 0.0;

        let assign300_e309: f64 = (0.000702 * locals.var_tnom);
        let assign300_e311: f64 = (assign300_e309 * locals.var_tnom);
        let assign300_e314: f64 = (locals.var_tnom + 1108.0);
        let assign300_e315: f64 = (assign300_e311 / assign300_e314);
        let assign300_e316: f64 = (1.16 - assign300_e315);
        locals.var_refeg = assign300_e316;
        locals.var_refeg_rv = 0.0;

        let assign310_e319: f64 = (locals.var_t - locals.var_tnom);
        locals.var_deltat = assign310_e319;
        locals.var_deltat_rv = 0.0;

        let assign320_e322: f64 = (locals.var_t / locals.var_tnom);
        locals.var_ratiot = assign320_e322;
        locals.var_ratiot_rv = 0.0;

        let assign330_e326: f64 = (p.p16 * locals.var_deltat);
        let assign330_e327: f64 = (p.p15 - assign330_e326);
        locals.var_vto_t = assign330_e327;
        locals.var_vto_t_rv = 0.0;

        let assign340_e331: f64 = (locals.var_ratiot).powf(p.p20);
        let assign340_e332: f64 = (p.p19 * assign340_e331);
        locals.var_kp_t = assign340_e332;
        locals.var_kp_t_rv = 0.0;

        let assign350_e336: f64 = (locals.var_ratiot).powf(p.p24);
        let assign350_e337: f64 = (p.p23 * assign350_e336);
        locals.var_ucrit_t = assign350_e337;
        locals.var_ucrit_t_rv = 0.0;

        let assign370_e347: f64 = (p.p18 * locals.var_ratiot);
        let assign370_e350: f64 = (3.0 * locals.var_vt);
        let assign370_e352: f64 = (locals.var_ratiot).ln();
        let assign370_e353: f64 = (assign370_e350 * assign370_e352);
        let assign370_e354: f64 = (assign370_e347 - assign370_e353);
        let assign370_e357: f64 = (locals.var_refeg * locals.var_ratiot);
        let assign370_e358: f64 = (assign370_e354 - assign370_e357);
        let assign370_e360: f64 = (assign370_e358 + locals.var_eg);
        locals.var_phi_t = assign370_e360;
        locals.var_phi_t_dn0 = 0.0;
        locals.var_phi_t_dn1 = 0.0;
        locals.var_phi_t_dn2 = 0.0;
        locals.var_phi_t_dn3 = 0.0;
        locals.var_phi_t_rv = 0.0;

        locals.var_tmp1 = 0.2;
        locals.var_tmp1_dn0 = 0.0;
        locals.var_tmp1_dn1 = 0.0;
        locals.var_tmp1_dn2 = 0.0;
        locals.var_tmp1_dn3 = 0.0;
        locals.var_tmp1_rv = 0.0;

        let assign390_e364: f64 = (locals.var_phi_t - locals.var_tmp1);
        locals.var_tmp2 = assign390_e364;
        locals.var_tmp2_dn0 = (locals.var_phi_t_dn0 - locals.var_tmp1_dn0);
        locals.var_tmp2_dn1 = (locals.var_phi_t_dn1 - locals.var_tmp1_dn1);
        locals.var_tmp2_dn2 = (locals.var_phi_t_dn2 - locals.var_tmp1_dn2);
        locals.var_tmp2_dn3 = (locals.var_phi_t_dn3 - locals.var_tmp1_dn3);
        locals.var_tmp2_rv = 0.0;

        let assign400_e369: f64 = (locals.var_tmp2 * locals.var_tmp2);
        let assign400_e372: f64 = (locals.var_vt * locals.var_vt);
        let assign400_e373: f64 = (assign400_e369 + assign400_e372);
        let assign400_e374: f64 = (assign400_e373).sqrt();
        let assign400_e375: f64 = (locals.var_tmp2 + assign400_e374);
        let assign400_e376: f64 = (0.5 * assign400_e375);
        let assign400_e378: f64 = (assign400_e376 + locals.var_tmp1);
        locals.var_phi_t = assign400_e378;
        locals.var_phi_t_dn0 = ((0.5 * (locals.var_tmp2_dn0 + (((locals.var_tmp2_dn0 * locals.var_tmp2) + (locals.var_tmp2 * locals.var_tmp2_dn0)) / (2.0 * assign400_e374)))) + locals.var_tmp1_dn0);
        locals.var_phi_t_dn1 = ((0.5 * (locals.var_tmp2_dn1 + (((locals.var_tmp2_dn1 * locals.var_tmp2) + (locals.var_tmp2 * locals.var_tmp2_dn1)) / (2.0 * assign400_e374)))) + locals.var_tmp1_dn1);
        locals.var_phi_t_dn2 = ((0.5 * (locals.var_tmp2_dn2 + (((locals.var_tmp2_dn2 * locals.var_tmp2) + (locals.var_tmp2 * locals.var_tmp2_dn2)) / (2.0 * assign400_e374)))) + locals.var_tmp1_dn2);
        locals.var_phi_t_dn3 = ((0.5 * (locals.var_tmp2_dn3 + (((locals.var_tmp2_dn3 * locals.var_tmp2) + (locals.var_tmp2 * locals.var_tmp2_dn3)) / (2.0 * assign400_e374)))) + locals.var_tmp1_dn3);
        locals.var_phi_t_rv = 0.0;

        let assign410_e380: f64 = (locals.var_phi_t).sqrt();
        locals.var_sqrt_phi = assign410_e380;
        locals.var_sqrt_phi_dn0 = (locals.var_phi_t_dn0 / (2.0 * assign410_e380));
        locals.var_sqrt_phi_dn1 = (locals.var_phi_t_dn1 / (2.0 * assign410_e380));
        locals.var_sqrt_phi_dn2 = (locals.var_phi_t_dn2 / (2.0 * assign410_e380));
        locals.var_sqrt_phi_dn3 = (locals.var_phi_t_dn3 / (2.0 * assign410_e380));
        locals.var_sqrt_phi_rv = 0.0;

        let assign420_e383: f64 = (1.0 / locals.var_ucrit_t);
        locals.var_inv_ucrit = assign420_e383;
        locals.var_inv_ucrit_rv = 0.0;

        let assign430_e386: f64 = (locals.var_lc * locals.var_ucrit_t);
        locals.var_lc_ucrit = assign430_e386;
        locals.var_lc_ucrit_rv = 0.0;

        let assign460_e395: f64 = (p.p5 + p.p26);
        locals.var_leff = assign460_e395;
        locals.var_leff_rv = 0.0;

        let assign470_e398: f64 = (p.p6 + p.p27);
        locals.var_weff = assign470_e398;
        locals.var_weff_rv = 0.0;

        let assign480_e401: f64 = (locals.var_ucrit_t * locals.var_leff);
        locals.var_vc = assign480_e401;
        locals.var_vc_rv = 0.0;

        let assign490_e405: f64 = (0.5 * locals.var_vc);
        let assign490_e407: f64 = (assign490_e405 * locals.var_inv_vt);
        let assign490_e408: f64 = (assign490_e407).ln();
        let assign490_e410: f64 = (assign490_e408 - 0.6);
        let assign490_e411: f64 = (locals.var_vt * assign490_e410);
        locals.var_log_vc_vt = assign490_e411;
        locals.var_log_vc_vt_rv = 0.0;

        let assign500_e415: f64 = (locals.var_weff * locals.var_leff);
        let assign500_e416: f64 = (assign500_e415).sqrt();
        let assign500_e417: f64 = (1.0 / assign500_e416);
        locals.var_awl = assign500_e417;
        locals.var_awl_rv = 0.0;

        let assign510_e420: f64 = if p.p0 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard3 = assign510_e420;
        locals.var_guard3_rv = 0.0;

        let (assign520_e435,) = {
    if (locals.var_guard3 != 0.0) {
        let (assign520_e433,) = {
            if (p.p38 != 1e-6) {
                let assign520_e428: f64 = (p.p38 - 1e-6);
                let assign520_e429: f64 = (locals.var_awl * assign520_e428);
                let assign520_e431: f64 = (assign520_e429 + locals.var_vto_t);
                (assign520_e431,)
            } else {
                (locals.var_vto_t,)
            }
        };
        (assign520_e433,)
    } else {
        (locals.var_vto_s,)
    }
};
        locals.var_vto_s = assign520_e435;
        locals.var_vto_s_rv = 0.0;

        let (assign530_e452,) = {
    if (locals.var_guard3 == 0.0) {
        let (assign530_e450,) = {
            if (p.p38 != 1e-6) {
                let assign530_e444: f64 = (1e-6 - p.p38);
                let assign530_e445: f64 = (locals.var_awl * assign530_e444);
                let assign530_e447: f64 = (assign530_e445 - locals.var_vto_t);
                (assign530_e447,)
            } else {
                let assign530_e449: f64 = (-locals.var_vto_t);
                (assign530_e449,)
            }
        };
        (assign530_e450,)
    } else {
        (locals.var_vto_s,)
    }
};
        locals.var_vto_s = assign530_e452;
        locals.var_vto_s_rv = 0.0;

        let (assign540_e467,) = {
    if (p.p39 != 1e-6) {
        let assign540_e461: f64 = (p.p39 - 1e-6);
        let assign540_e463: f64 = (assign540_e461 * locals.var_awl);
        let assign540_e464: f64 = (1.0 + assign540_e463);
        let assign540_e465: f64 = (locals.var_kp_t * assign540_e464);
        (assign540_e465,)
    } else {
        (locals.var_kp_t,)
    }
};
        let assign540_e468: f64 = (locals.var_weff * assign540_e467);
        locals.var_kp_weff = assign540_e468;
        locals.var_kp_weff_rv = 0.0;

        let (assign550_e480,) = {
    if (p.p40 != 1e-6) {
        let assign550_e475: f64 = (p.p40 - 1e-6);
        let assign550_e477: f64 = (assign550_e475 * locals.var_awl);
        let assign550_e478: f64 = (p.p17 + assign550_e477);
        (assign550_e478,)
    } else {
        (p.p17,)
    }
};
        locals.var_gamma_s = assign550_e480;
        locals.var_gamma_s_rv = 0.0;

        let assign560_e483: f64 = (locals.var_gamma_s * locals.var_sqrt_phi);
        locals.var_gamma_sqrt_phi = assign560_e483;
        locals.var_gamma_sqrt_phi_dn0 = (locals.var_gamma_s * locals.var_sqrt_phi_dn0);
        locals.var_gamma_sqrt_phi_dn1 = (locals.var_gamma_s * locals.var_sqrt_phi_dn1);
        locals.var_gamma_sqrt_phi_dn2 = (locals.var_gamma_s * locals.var_sqrt_phi_dn2);
        locals.var_gamma_sqrt_phi_dn3 = (locals.var_gamma_s * locals.var_sqrt_phi_dn3);
        locals.var_gamma_sqrt_phi_rv = 0.0;

        let assign570_e486: f64 = if locals.var_v0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard4 = assign570_e486;
        locals.var_guard4_rv = 0.0;

        let (assign580_e490,) = {
    if (locals.var_guard4 != 0.0) {
        (0.0,)
    } else {
        (locals.var_deltavfb,)
    }
};
        locals.var_deltavfb = assign580_e490;
        locals.var_deltavfb_rv = 0.0;

        let (assign590_e503,) = {
    if (locals.var_guard4 == 0.0) {
        let assign590_e497: f64 = (p.p31 * p.p8);
        let assign590_e498: f64 = (locals.var_leff / assign590_e497);
        let assign590_e500: f64 = (assign590_e498 - 0.1);
        let assign590_e501: f64 = (0.28 * assign590_e500);
        (assign590_e501,)
    } else {
        (locals.var_vl,)
    }
};
        locals.var_vl = assign590_e503;
        locals.var_vl_rv = 0.0;

        let (assign600_e521,) = {
    if (locals.var_guard4 == 0.0) {
        let assign600_e512: f64 = (locals.var_vl * locals.var_vl);
        let assign600_e514: f64 = (assign600_e512 + 0.001936);
        let assign600_e515: f64 = (assign600_e514).sqrt();
        let assign600_e516: f64 = (locals.var_vl + assign600_e515);
        let assign600_e517: f64 = (0.5 * assign600_e516);
        let assign600_e518: f64 = (1.0 + assign600_e517);
        let assign600_e519: f64 = (1.0 / assign600_e518);
        (assign600_e519,)
    } else {
        (locals.var_sqv,)
    }
};
        locals.var_sqv = assign600_e521;
        locals.var_sqv_rv = 0.0;

        let (assign610_e530,) = {
    if (locals.var_guard4 == 0.0) {
        let assign610_e526: f64 = (locals.var_v0 * locals.var_sqv);
        let assign610_e528: f64 = (assign610_e526 * locals.var_sqv);
        (assign610_e528,)
    } else {
        (locals.var_deltavfb,)
    }
};
        locals.var_deltavfb = assign610_e530;
        locals.var_deltavfb_rv = 0.0;

        let assign620_e533: f64 = (p.p0 * (nv1 - nv3));
        locals.var_vg = assign620_e533;
        locals.var_vg_dn1 = p.p0;
        locals.var_vg_dn3 = (-p.p0);
        locals.var_vg_rv = 0.0;

        let assign630_e536: f64 = (p.p0 * (nv2 - nv3));
        locals.var_vs = assign630_e536;
        locals.var_vs_dn0 = 0.0;
        locals.var_vs_dn2 = p.p0;
        locals.var_vs_dn3 = (-p.p0);
        locals.var_vs_rv = 0.0;

        let assign640_e539: f64 = (p.p0 * (nv0 - nv3));
        locals.var_vd = assign640_e539;
        locals.var_vd_dn0 = p.p0;
        locals.var_vd_dn2 = 0.0;
        locals.var_vd_dn3 = (-p.p0);
        locals.var_vd_rv = 0.0;

        let assign650_e542: f64 = (locals.var_vd - locals.var_vs);
        let assign650_e544: f64 = if assign650_e542 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign650_e544;
        locals.var_guard6_rv = 0.0;

        let (assign660_e549,) = {
    if (locals.var_guard6 != 0.0) {
        let assign660_e547: f64 = (-1.0);
        (assign660_e547,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign660_e549;
        locals.var_mode_rv = 0.0;

        let (assign670_e553, assign670_e553_d_n0, assign670_e553_d_n2, assign670_e553_d_n3,) = {
    if (locals.var_guard6 != 0.0) {
        (locals.var_vs, locals.var_vs_dn0, locals.var_vs_dn2, locals.var_vs_dn3,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3,)
    }
};
        locals.var_t1 = assign670_e553;
        locals.var_t1_dn0 = assign670_e553_d_n0;
        locals.var_t1_dn2 = assign670_e553_d_n2;
        locals.var_t1_dn3 = assign670_e553_d_n3;
        locals.var_t1_rv = 0.0;

        let (assign680_e557, assign680_e557_d_n0, assign680_e557_d_n2, assign680_e557_d_n3,) = {
    if (locals.var_guard6 != 0.0) {
        (locals.var_vd, locals.var_vd_dn0, locals.var_vd_dn2, locals.var_vd_dn3,)
    } else {
        (locals.var_vs, locals.var_vs_dn0, locals.var_vs_dn2, locals.var_vs_dn3,)
    }
};
        locals.var_vs = assign680_e557;
        locals.var_vs_dn0 = assign680_e557_d_n0;
        locals.var_vs_dn2 = assign680_e557_d_n2;
        locals.var_vs_dn3 = assign680_e557_d_n3;
        locals.var_vs_rv = 0.0;

        let (assign690_e561, assign690_e561_d_n0, assign690_e561_d_n2, assign690_e561_d_n3,) = {
    if (locals.var_guard6 != 0.0) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3,)
    } else {
        (locals.var_vd, locals.var_vd_dn0, locals.var_vd_dn2, locals.var_vd_dn3,)
    }
};
        locals.var_vd = assign690_e561;
        locals.var_vd_dn0 = assign690_e561_d_n0;
        locals.var_vd_dn2 = assign690_e561_d_n2;
        locals.var_vd_dn3 = assign690_e561_d_n3;
        locals.var_vd_rv = 0.0;

        let (assign700_e566,) = {
    if (locals.var_guard6 == 0.0) {
        (1.0,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign700_e566;
        locals.var_mode_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign710_e569: f64 = (locals.var_vg - locals.var_vto_s);
        let assign710_e571: f64 = (assign710_e569 - locals.var_deltavfb);
        let assign710_e573: f64 = (assign710_e571 + locals.var_phi_t);
        let assign710_e575: f64 = (assign710_e573 + locals.var_gamma_sqrt_phi);
        locals.var_vgstar = assign710_e575;
        locals.var_vgstar_dn0 = (locals.var_phi_t_dn0 + locals.var_gamma_sqrt_phi_dn0);
        locals.var_vgstar_dn1 = ((locals.var_vg_dn1 + locals.var_phi_t_dn1) + locals.var_gamma_sqrt_phi_dn1);
        locals.var_vgstar_dn2 = (locals.var_phi_t_dn2 + locals.var_gamma_sqrt_phi_dn2);
        locals.var_vgstar_dn3 = ((locals.var_vg_dn3 + locals.var_phi_t_dn3) + locals.var_gamma_sqrt_phi_dn3);
        locals.var_vgstar_rv = 0.0;

        let assign720_e578: f64 = (locals.var_vgstar * locals.var_vgstar);
        let assign720_e581: f64 = (2.0 * locals.var_vt_vt_16);
        let assign720_e582: f64 = (assign720_e578 + assign720_e581);
        let assign720_e583: f64 = (assign720_e582).sqrt();
        locals.var_sqrt_vgstar = assign720_e583;
        locals.var_sqrt_vgstar_dn0 = (((locals.var_vgstar_dn0 * locals.var_vgstar) + (locals.var_vgstar * locals.var_vgstar_dn0)) / (2.0 * assign720_e583));
        locals.var_sqrt_vgstar_dn1 = (((locals.var_vgstar_dn1 * locals.var_vgstar) + (locals.var_vgstar * locals.var_vgstar_dn1)) / (2.0 * assign720_e583));
        locals.var_sqrt_vgstar_dn2 = (((locals.var_vgstar_dn2 * locals.var_vgstar) + (locals.var_vgstar * locals.var_vgstar_dn2)) / (2.0 * assign720_e583));
        locals.var_sqrt_vgstar_dn3 = (((locals.var_vgstar_dn3 * locals.var_vgstar) + (locals.var_vgstar * locals.var_vgstar_dn3)) / (2.0 * assign720_e583));
        locals.var_sqrt_vgstar_rv = 0.0;

        let assign730_e587: f64 = (locals.var_vgstar + locals.var_sqrt_vgstar);
        let assign730_e588: f64 = (0.5 * assign730_e587);
        locals.var_vgprime = assign730_e588;
        locals.var_vgprime_dn0 = (0.5 * (locals.var_vgstar_dn0 + locals.var_sqrt_vgstar_dn0));
        locals.var_vgprime_dn1 = (0.5 * (locals.var_vgstar_dn1 + locals.var_sqrt_vgstar_dn1));
        locals.var_vgprime_dn2 = (0.5 * (locals.var_vgstar_dn2 + locals.var_sqrt_vgstar_dn2));
        locals.var_vgprime_dn3 = (0.5 * (locals.var_vgstar_dn3 + locals.var_sqrt_vgstar_dn3));
        locals.var_vgprime_rv = 0.0;

        let assign740_e591: f64 = (locals.var_phi_t + locals.var_vs);
        locals.var_phi_vs = assign740_e591;
        locals.var_phi_vs_dn0 = (locals.var_phi_t_dn0 + locals.var_vs_dn0);
        locals.var_phi_vs_dn1 = locals.var_phi_t_dn1;
        locals.var_phi_vs_dn2 = (locals.var_phi_t_dn2 + locals.var_vs_dn2);
        locals.var_phi_vs_dn3 = (locals.var_phi_t_dn3 + locals.var_vs_dn3);
        locals.var_phi_vs_rv = 0.0;

        let assign750_e594: f64 = (locals.var_phi_vs * locals.var_phi_vs);
        let assign750_e596: f64 = (assign750_e594 + locals.var_vt_vt_16);
        let assign750_e597: f64 = (assign750_e596).sqrt();
        locals.var_sqrt_phi_vs_vt = assign750_e597;
        locals.var_sqrt_phi_vs_vt_dn0 = (((locals.var_phi_vs_dn0 * locals.var_phi_vs) + (locals.var_phi_vs * locals.var_phi_vs_dn0)) / (2.0 * assign750_e597));
        locals.var_sqrt_phi_vs_vt_dn1 = (((locals.var_phi_vs_dn1 * locals.var_phi_vs) + (locals.var_phi_vs * locals.var_phi_vs_dn1)) / (2.0 * assign750_e597));
        locals.var_sqrt_phi_vs_vt_dn2 = (((locals.var_phi_vs_dn2 * locals.var_phi_vs) + (locals.var_phi_vs * locals.var_phi_vs_dn2)) / (2.0 * assign750_e597));
        locals.var_sqrt_phi_vs_vt_dn3 = (((locals.var_phi_vs_dn3 * locals.var_phi_vs) + (locals.var_phi_vs * locals.var_phi_vs_dn3)) / (2.0 * assign750_e597));
        locals.var_sqrt_phi_vs_vt_rv = 0.0;

        let assign760_e601: f64 = (locals.var_phi_vs + locals.var_sqrt_phi_vs_vt);
        let assign760_e602: f64 = (0.5 * assign760_e601);
        let assign760_e603: f64 = (assign760_e602).sqrt();
        locals.var_sqrt_phi_vs = assign760_e603;
        locals.var_sqrt_phi_vs_dn0 = ((0.5 * (locals.var_phi_vs_dn0 + locals.var_sqrt_phi_vs_vt_dn0)) / (2.0 * assign760_e603));
        locals.var_sqrt_phi_vs_dn1 = ((0.5 * (locals.var_phi_vs_dn1 + locals.var_sqrt_phi_vs_vt_dn1)) / (2.0 * assign760_e603));
        locals.var_sqrt_phi_vs_dn2 = ((0.5 * (locals.var_phi_vs_dn2 + locals.var_sqrt_phi_vs_vt_dn2)) / (2.0 * assign760_e603));
        locals.var_sqrt_phi_vs_dn3 = ((0.5 * (locals.var_phi_vs_dn3 + locals.var_sqrt_phi_vs_vt_dn3)) / (2.0 * assign760_e603));
        locals.var_sqrt_phi_vs_rv = 0.0;

        let assign770_e606: f64 = (locals.var_phi_t + locals.var_vd);
        locals.var_phi_vd = assign770_e606;
        locals.var_phi_vd_dn0 = (locals.var_phi_t_dn0 + locals.var_vd_dn0);
        locals.var_phi_vd_dn1 = locals.var_phi_t_dn1;
        locals.var_phi_vd_dn2 = (locals.var_phi_t_dn2 + locals.var_vd_dn2);
        locals.var_phi_vd_dn3 = (locals.var_phi_t_dn3 + locals.var_vd_dn3);
        locals.var_phi_vd_rv = 0.0;

        let assign780_e609: f64 = (locals.var_phi_vd * locals.var_phi_vd);
        let assign780_e611: f64 = (assign780_e609 + locals.var_vt_vt_16);
        let assign780_e612: f64 = (assign780_e611).sqrt();
        locals.var_sqrt_phi_vd_vt = assign780_e612;
        locals.var_sqrt_phi_vd_vt_dn0 = (((locals.var_phi_vd_dn0 * locals.var_phi_vd) + (locals.var_phi_vd * locals.var_phi_vd_dn0)) / (2.0 * assign780_e612));
        locals.var_sqrt_phi_vd_vt_dn1 = (((locals.var_phi_vd_dn1 * locals.var_phi_vd) + (locals.var_phi_vd * locals.var_phi_vd_dn1)) / (2.0 * assign780_e612));
        locals.var_sqrt_phi_vd_vt_dn2 = (((locals.var_phi_vd_dn2 * locals.var_phi_vd) + (locals.var_phi_vd * locals.var_phi_vd_dn2)) / (2.0 * assign780_e612));
        locals.var_sqrt_phi_vd_vt_dn3 = (((locals.var_phi_vd_dn3 * locals.var_phi_vd) + (locals.var_phi_vd * locals.var_phi_vd_dn3)) / (2.0 * assign780_e612));
        locals.var_sqrt_phi_vd_vt_rv = 0.0;

        let assign790_e616: f64 = (locals.var_phi_vd + locals.var_sqrt_phi_vd_vt);
        let assign790_e617: f64 = (0.5 * assign790_e616);
        let assign790_e618: f64 = (assign790_e617).sqrt();
        locals.var_sqrt_phi_vd = assign790_e618;
        locals.var_sqrt_phi_vd_dn0 = ((0.5 * (locals.var_phi_vd_dn0 + locals.var_sqrt_phi_vd_vt_dn0)) / (2.0 * assign790_e618));
        locals.var_sqrt_phi_vd_dn1 = ((0.5 * (locals.var_phi_vd_dn1 + locals.var_sqrt_phi_vd_vt_dn1)) / (2.0 * assign790_e618));
        locals.var_sqrt_phi_vd_dn2 = ((0.5 * (locals.var_phi_vd_dn2 + locals.var_sqrt_phi_vd_vt_dn2)) / (2.0 * assign790_e618));
        locals.var_sqrt_phi_vd_dn3 = ((0.5 * (locals.var_phi_vd_dn3 + locals.var_sqrt_phi_vd_vt_dn3)) / (2.0 * assign790_e618));
        locals.var_sqrt_phi_vd_rv = 0.0;

        let assign800_e621: f64 = (locals.var_eps_cox_w * p.p7);
        let assign800_e623: f64 = (assign800_e621 / locals.var_weff);
        locals.var_weta_w = assign800_e623;
        locals.var_weta_w_rv = 0.0;

        let assign810_e626: f64 = (locals.var_eps_cox_l * p.p8);
        let assign810_e628: f64 = (assign810_e626 / locals.var_leff);
        locals.var_leta_l = assign810_e628;
        locals.var_leta_l_rv = 0.0;

        let assign820_e632: f64 = (0.25 * locals.var_gamma_s);
        let assign820_e634: f64 = (assign820_e632 * locals.var_gamma_s);
        let assign820_e635: f64 = (locals.var_vgprime + assign820_e634);
        let assign820_e636: f64 = (assign820_e635).sqrt();
        locals.var_big_sqrt_vp0 = assign820_e636;
        locals.var_big_sqrt_vp0_dn0 = (locals.var_vgprime_dn0 / (2.0 * assign820_e636));
        locals.var_big_sqrt_vp0_dn1 = (locals.var_vgprime_dn1 / (2.0 * assign820_e636));
        locals.var_big_sqrt_vp0_dn2 = (locals.var_vgprime_dn2 / (2.0 * assign820_e636));
        locals.var_big_sqrt_vp0_dn3 = (locals.var_vgprime_dn3 / (2.0 * assign820_e636));
        locals.var_big_sqrt_vp0_rv = 0.0;

        let assign830_e639: f64 = (locals.var_vgprime - locals.var_phi_t);
        let assign830_e644: f64 = (0.5 * locals.var_gamma_s);
        let assign830_e645: f64 = (locals.var_big_sqrt_vp0 - assign830_e644);
        let assign830_e646: f64 = (locals.var_gamma_s * assign830_e645);
        let assign830_e647: f64 = (assign830_e639 - assign830_e646);
        locals.var_vp0 = assign830_e647;
        locals.var_vp0_dn0 = ((locals.var_vgprime_dn0 - locals.var_phi_t_dn0) - (locals.var_gamma_s * locals.var_big_sqrt_vp0_dn0));
        locals.var_vp0_dn1 = ((locals.var_vgprime_dn1 - locals.var_phi_t_dn1) - (locals.var_gamma_s * locals.var_big_sqrt_vp0_dn1));
        locals.var_vp0_dn2 = ((locals.var_vgprime_dn2 - locals.var_phi_t_dn2) - (locals.var_gamma_s * locals.var_big_sqrt_vp0_dn2));
        locals.var_vp0_dn3 = ((locals.var_vgprime_dn3 - locals.var_phi_t_dn3) - (locals.var_gamma_s * locals.var_big_sqrt_vp0_dn3));
        locals.var_vp0_rv = 0.0;

        let assign840_e650: f64 = (locals.var_vp0 + locals.var_phi_t);
        let assign840_e652: f64 = (assign840_e650 + locals.var_vt_01);
        let assign840_e653: f64 = (assign840_e652).sqrt();
        locals.var_sqrt_phi_vp0 = assign840_e653;
        locals.var_sqrt_phi_vp0_dn0 = ((locals.var_vp0_dn0 + locals.var_phi_t_dn0) / (2.0 * assign840_e653));
        locals.var_sqrt_phi_vp0_dn1 = ((locals.var_vp0_dn1 + locals.var_phi_t_dn1) / (2.0 * assign840_e653));
        locals.var_sqrt_phi_vp0_dn2 = ((locals.var_vp0_dn2 + locals.var_phi_t_dn2) / (2.0 * assign840_e653));
        locals.var_sqrt_phi_vp0_dn3 = ((locals.var_vp0_dn3 + locals.var_phi_t_dn3) / (2.0 * assign840_e653));
        locals.var_sqrt_phi_vp0_rv = 0.0;

        let assign850_e658: f64 = (locals.var_sqrt_phi_vs + locals.var_sqrt_phi_vd);
        let assign850_e659: f64 = (locals.var_leta_l * assign850_e658);
        let assign850_e660: f64 = (locals.var_gamma_s - assign850_e659);
        let assign850_e663: f64 = (locals.var_weta_w * locals.var_sqrt_phi_vp0);
        let assign850_e664: f64 = (assign850_e660 + assign850_e663);
        locals.var_gammastar = assign850_e664;
        locals.var_gammastar_dn0 = ((-(locals.var_leta_l * (locals.var_sqrt_phi_vs_dn0 + locals.var_sqrt_phi_vd_dn0))) + (locals.var_weta_w * locals.var_sqrt_phi_vp0_dn0));
        locals.var_gammastar_dn1 = ((-(locals.var_leta_l * (locals.var_sqrt_phi_vs_dn1 + locals.var_sqrt_phi_vd_dn1))) + (locals.var_weta_w * locals.var_sqrt_phi_vp0_dn1));
        locals.var_gammastar_dn2 = ((-(locals.var_leta_l * (locals.var_sqrt_phi_vs_dn2 + locals.var_sqrt_phi_vd_dn2))) + (locals.var_weta_w * locals.var_sqrt_phi_vp0_dn2));
        locals.var_gammastar_dn3 = ((-(locals.var_leta_l * (locals.var_sqrt_phi_vs_dn3 + locals.var_sqrt_phi_vd_dn3))) + (locals.var_weta_w * locals.var_sqrt_phi_vp0_dn3));
        locals.var_gammastar_rv = 0.0;

        let assign860_e667: f64 = (locals.var_gammastar * locals.var_gammastar);
        let assign860_e669: f64 = (assign860_e667 + locals.var_vt_01);
        let assign860_e670: f64 = (assign860_e669).sqrt();
        locals.var_sqrt_gammastar = assign860_e670;
        locals.var_sqrt_gammastar_dn0 = (((locals.var_gammastar_dn0 * locals.var_gammastar) + (locals.var_gammastar * locals.var_gammastar_dn0)) / (2.0 * assign860_e670));
        locals.var_sqrt_gammastar_dn1 = (((locals.var_gammastar_dn1 * locals.var_gammastar) + (locals.var_gammastar * locals.var_gammastar_dn1)) / (2.0 * assign860_e670));
        locals.var_sqrt_gammastar_dn2 = (((locals.var_gammastar_dn2 * locals.var_gammastar) + (locals.var_gammastar * locals.var_gammastar_dn2)) / (2.0 * assign860_e670));
        locals.var_sqrt_gammastar_dn3 = (((locals.var_gammastar_dn3 * locals.var_gammastar) + (locals.var_gammastar * locals.var_gammastar_dn3)) / (2.0 * assign860_e670));
        locals.var_sqrt_gammastar_rv = 0.0;

        let assign870_e674: f64 = (locals.var_gammastar + locals.var_sqrt_gammastar);
        let assign870_e675: f64 = (0.5 * assign870_e674);
        locals.var_gammaprime = assign870_e675;
        locals.var_gammaprime_dn0 = (0.5 * (locals.var_gammastar_dn0 + locals.var_sqrt_gammastar_dn0));
        locals.var_gammaprime_dn1 = (0.5 * (locals.var_gammastar_dn1 + locals.var_sqrt_gammastar_dn1));
        locals.var_gammaprime_dn2 = (0.5 * (locals.var_gammastar_dn2 + locals.var_sqrt_gammastar_dn2));
        locals.var_gammaprime_dn3 = (0.5 * (locals.var_gammastar_dn3 + locals.var_sqrt_gammastar_dn3));
        locals.var_gammaprime_rv = 0.0;

        let assign880_e679: f64 = (0.25 * locals.var_gammaprime);
        let assign880_e681: f64 = (assign880_e679 * locals.var_gammaprime);
        let assign880_e682: f64 = (locals.var_vgprime + assign880_e681);
        let assign880_e683: f64 = (assign880_e682).sqrt();
        locals.var_big_sqrt_vp = assign880_e683;
        locals.var_big_sqrt_vp_dn0 = ((locals.var_vgprime_dn0 + (((0.25 * locals.var_gammaprime_dn0) * locals.var_gammaprime) + (assign880_e679 * locals.var_gammaprime_dn0))) / (2.0 * assign880_e683));
        locals.var_big_sqrt_vp_dn1 = ((locals.var_vgprime_dn1 + (((0.25 * locals.var_gammaprime_dn1) * locals.var_gammaprime) + (assign880_e679 * locals.var_gammaprime_dn1))) / (2.0 * assign880_e683));
        locals.var_big_sqrt_vp_dn2 = ((locals.var_vgprime_dn2 + (((0.25 * locals.var_gammaprime_dn2) * locals.var_gammaprime) + (assign880_e679 * locals.var_gammaprime_dn2))) / (2.0 * assign880_e683));
        locals.var_big_sqrt_vp_dn3 = ((locals.var_vgprime_dn3 + (((0.25 * locals.var_gammaprime_dn3) * locals.var_gammaprime) + (assign880_e679 * locals.var_gammaprime_dn3))) / (2.0 * assign880_e683));
        locals.var_big_sqrt_vp_rv = 0.0;

        let assign890_e686: f64 = (locals.var_vgprime - locals.var_phi_t);
        let assign890_e691: f64 = (0.5 * locals.var_gammaprime);
        let assign890_e692: f64 = (locals.var_big_sqrt_vp - assign890_e691);
        let assign890_e693: f64 = (locals.var_gammaprime * assign890_e692);
        let assign890_e694: f64 = (assign890_e686 - assign890_e693);
        locals.var_vp = assign890_e694;
        locals.var_vp_dn0 = ((locals.var_vgprime_dn0 - locals.var_phi_t_dn0) - ((locals.var_gammaprime_dn0 * assign890_e692) + (locals.var_gammaprime * (locals.var_big_sqrt_vp_dn0 - (0.5 * locals.var_gammaprime_dn0)))));
        locals.var_vp_dn1 = ((locals.var_vgprime_dn1 - locals.var_phi_t_dn1) - ((locals.var_gammaprime_dn1 * assign890_e692) + (locals.var_gammaprime * (locals.var_big_sqrt_vp_dn1 - (0.5 * locals.var_gammaprime_dn1)))));
        locals.var_vp_dn2 = ((locals.var_vgprime_dn2 - locals.var_phi_t_dn2) - ((locals.var_gammaprime_dn2 * assign890_e692) + (locals.var_gammaprime * (locals.var_big_sqrt_vp_dn2 - (0.5 * locals.var_gammaprime_dn2)))));
        locals.var_vp_dn3 = ((locals.var_vgprime_dn3 - locals.var_phi_t_dn3) - ((locals.var_gammaprime_dn3 * assign890_e692) + (locals.var_gammaprime * (locals.var_big_sqrt_vp_dn3 - (0.5 * locals.var_gammaprime_dn3)))));
        locals.var_vp_rv = 0.0;

        let assign900_e697: f64 = (locals.var_vp - locals.var_vs);
        let assign900_e699: f64 = (assign900_e697 * locals.var_inv_vt);
        locals.var_tmp1 = assign900_e699;
        locals.var_tmp1_dn0 = ((locals.var_vp_dn0 - locals.var_vs_dn0) * locals.var_inv_vt);
        locals.var_tmp1_dn1 = (locals.var_vp_dn1 * locals.var_inv_vt);
        locals.var_tmp1_dn2 = ((locals.var_vp_dn2 - locals.var_vs_dn2) * locals.var_inv_vt);
        locals.var_tmp1_dn3 = ((locals.var_vp_dn3 - locals.var_vs_dn3) * locals.var_inv_vt);
        locals.var_tmp1_rv = 0.0;

        let assign910_e702: f64 = (-0.35);
        let assign910_e703: f64 = if locals.var_tmp1 > assign910_e702 { 1.0 } else { 0.0 };
        locals.var_guard7 = assign910_e703;
        locals.var_guard7_rv = 0.0;

        let (assign920_e716, assign920_e716_d_n0, assign920_e716_d_n1, assign920_e716_d_n2, assign920_e716_d_n3,) = {
    if (locals.var_guard7 != 0.0) {
        let assign920_e708: f64 = (1.3 + locals.var_tmp1);
        let assign920_e711: f64 = (locals.var_tmp1 + 1.6);
        let assign920_e712: f64 = (assign920_e711).ln();
        let assign920_e713: f64 = (assign920_e708 - assign920_e712);
        let assign920_e714: f64 = (2.0 / assign920_e713);
        (assign920_e714, (-((2.0 * (locals.var_tmp1_dn0 - (locals.var_tmp1_dn0 / assign920_e711))) / (assign920_e713 * assign920_e713))), (-((2.0 * (locals.var_tmp1_dn1 - (locals.var_tmp1_dn1 / assign920_e711))) / (assign920_e713 * assign920_e713))), (-((2.0 * (locals.var_tmp1_dn2 - (locals.var_tmp1_dn2 / assign920_e711))) / (assign920_e713 * assign920_e713))), (-((2.0 * (locals.var_tmp1_dn3 - (locals.var_tmp1_dn3 / assign920_e711))) / (assign920_e713 * assign920_e713))),)
    } else {
        (locals.var_z0, locals.var_z0_dn0, locals.var_z0_dn1, locals.var_z0_dn2, locals.var_z0_dn3,)
    }
};
        locals.var_z0 = assign920_e716;
        locals.var_z0_dn0 = assign920_e716_d_n0;
        locals.var_z0_dn1 = assign920_e716_d_n1;
        locals.var_z0_dn2 = assign920_e716_d_n2;
        locals.var_z0_dn3 = assign920_e716_d_n3;
        locals.var_z0_rv = 0.0;

        let (assign930_e729, assign930_e729_d_n0, assign930_e729_d_n1, assign930_e729_d_n2, assign930_e729_d_n3,) = {
    if (locals.var_guard7 != 0.0) {
        let assign930_e720: f64 = (2.0 + locals.var_z0);
        let assign930_e723: f64 = (1.0 + locals.var_tmp1);
        let assign930_e725: f64 = (locals.var_z0).ln();
        let assign930_e726: f64 = (assign930_e723 + assign930_e725);
        let assign930_e727: f64 = (assign930_e720 / assign930_e726);
        (assign930_e727, (((locals.var_z0_dn0 * assign930_e726) - (assign930_e720 * (locals.var_tmp1_dn0 + (locals.var_z0_dn0 / locals.var_z0)))) / (assign930_e726 * assign930_e726)), (((locals.var_z0_dn1 * assign930_e726) - (assign930_e720 * (locals.var_tmp1_dn1 + (locals.var_z0_dn1 / locals.var_z0)))) / (assign930_e726 * assign930_e726)), (((locals.var_z0_dn2 * assign930_e726) - (assign930_e720 * (locals.var_tmp1_dn2 + (locals.var_z0_dn2 / locals.var_z0)))) / (assign930_e726 * assign930_e726)), (((locals.var_z0_dn3 * assign930_e726) - (assign930_e720 * (locals.var_tmp1_dn3 + (locals.var_z0_dn3 / locals.var_z0)))) / (assign930_e726 * assign930_e726)),)
    } else {
        (locals.var_zk, locals.var_zk_dn0, locals.var_zk_dn1, locals.var_zk_dn2, locals.var_zk_dn3,)
    }
};
        locals.var_zk = assign930_e729;
        locals.var_zk_dn0 = assign930_e729_d_n0;
        locals.var_zk_dn1 = assign930_e729_d_n1;
        locals.var_zk_dn2 = assign930_e729_d_n2;
        locals.var_zk_dn3 = assign930_e729_d_n3;
        locals.var_zk_rv = 0.0;

        let (assign940_e742, assign940_e742_d_n0, assign940_e742_d_n1, assign940_e742_d_n2, assign940_e742_d_n3,) = {
    if (locals.var_guard7 != 0.0) {
        let assign940_e733: f64 = (1.0 + locals.var_tmp1);
        let assign940_e735: f64 = (locals.var_zk).ln();
        let assign940_e736: f64 = (assign940_e733 + assign940_e735);
        let assign940_e739: f64 = (2.0 + locals.var_zk);
        let assign940_e740: f64 = (assign940_e736 / assign940_e739);
        (assign940_e740, ((((locals.var_tmp1_dn0 + (locals.var_zk_dn0 / locals.var_zk)) * assign940_e739) - (assign940_e736 * locals.var_zk_dn0)) / (assign940_e739 * assign940_e739)), ((((locals.var_tmp1_dn1 + (locals.var_zk_dn1 / locals.var_zk)) * assign940_e739) - (assign940_e736 * locals.var_zk_dn1)) / (assign940_e739 * assign940_e739)), ((((locals.var_tmp1_dn2 + (locals.var_zk_dn2 / locals.var_zk)) * assign940_e739) - (assign940_e736 * locals.var_zk_dn2)) / (assign940_e739 * assign940_e739)), ((((locals.var_tmp1_dn3 + (locals.var_zk_dn3 / locals.var_zk)) * assign940_e739) - (assign940_e736 * locals.var_zk_dn3)) / (assign940_e739 * assign940_e739)),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign940_e742;
        locals.var_yk_dn0 = assign940_e742_d_n0;
        locals.var_yk_dn1 = assign940_e742_d_n1;
        locals.var_yk_dn2 = assign940_e742_d_n2;
        locals.var_yk_dn3 = assign940_e742_d_n3;
        locals.var_yk_rv = 0.0;

        let assign950_e745: f64 = (-15.0);
        let assign950_e746: f64 = if locals.var_tmp1 > assign950_e745 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign950_e746;
        locals.var_guard8_rv = 0.0;

        let (assign960_e757, assign960_e757_d_n0, assign960_e757_d_n1, assign960_e757_d_n2, assign960_e757_d_n3,) = {
    if ((locals.var_guard7 == 0.0) && (locals.var_guard8 != 0.0)) {
        let assign960_e753: f64 = (-locals.var_tmp1);
        let assign960_e754: f64 = (assign960_e753).exp();
        let assign960_e755: f64 = (1.55 + assign960_e754);
        (assign960_e755, (assign960_e754 * (-locals.var_tmp1_dn0)), (assign960_e754 * (-locals.var_tmp1_dn1)), (assign960_e754 * (-locals.var_tmp1_dn2)), (assign960_e754 * (-locals.var_tmp1_dn3)),)
    } else {
        (locals.var_z0, locals.var_z0_dn0, locals.var_z0_dn1, locals.var_z0_dn2, locals.var_z0_dn3,)
    }
};
        locals.var_z0 = assign960_e757;
        locals.var_z0_dn0 = assign960_e757_d_n0;
        locals.var_z0_dn1 = assign960_e757_d_n1;
        locals.var_z0_dn2 = assign960_e757_d_n2;
        locals.var_z0_dn3 = assign960_e757_d_n3;
        locals.var_z0_rv = 0.0;

        let (assign970_e773, assign970_e773_d_n0, assign970_e773_d_n1, assign970_e773_d_n2, assign970_e773_d_n3,) = {
    if ((locals.var_guard7 == 0.0) && (locals.var_guard8 != 0.0)) {
        let assign970_e764: f64 = (2.0 + locals.var_z0);
        let assign970_e767: f64 = (1.0 + locals.var_tmp1);
        let assign970_e769: f64 = (locals.var_z0).ln();
        let assign970_e770: f64 = (assign970_e767 + assign970_e769);
        let assign970_e771: f64 = (assign970_e764 / assign970_e770);
        (assign970_e771, (((locals.var_z0_dn0 * assign970_e770) - (assign970_e764 * (locals.var_tmp1_dn0 + (locals.var_z0_dn0 / locals.var_z0)))) / (assign970_e770 * assign970_e770)), (((locals.var_z0_dn1 * assign970_e770) - (assign970_e764 * (locals.var_tmp1_dn1 + (locals.var_z0_dn1 / locals.var_z0)))) / (assign970_e770 * assign970_e770)), (((locals.var_z0_dn2 * assign970_e770) - (assign970_e764 * (locals.var_tmp1_dn2 + (locals.var_z0_dn2 / locals.var_z0)))) / (assign970_e770 * assign970_e770)), (((locals.var_z0_dn3 * assign970_e770) - (assign970_e764 * (locals.var_tmp1_dn3 + (locals.var_z0_dn3 / locals.var_z0)))) / (assign970_e770 * assign970_e770)),)
    } else {
        (locals.var_zk, locals.var_zk_dn0, locals.var_zk_dn1, locals.var_zk_dn2, locals.var_zk_dn3,)
    }
};
        locals.var_zk = assign970_e773;
        locals.var_zk_dn0 = assign970_e773_d_n0;
        locals.var_zk_dn1 = assign970_e773_d_n1;
        locals.var_zk_dn2 = assign970_e773_d_n2;
        locals.var_zk_dn3 = assign970_e773_d_n3;
        locals.var_zk_rv = 0.0;

        let (assign980_e789, assign980_e789_d_n0, assign980_e789_d_n1, assign980_e789_d_n2, assign980_e789_d_n3,) = {
    if ((locals.var_guard7 == 0.0) && (locals.var_guard8 != 0.0)) {
        let assign980_e780: f64 = (1.0 + locals.var_tmp1);
        let assign980_e782: f64 = (locals.var_zk).ln();
        let assign980_e783: f64 = (assign980_e780 + assign980_e782);
        let assign980_e786: f64 = (2.0 + locals.var_zk);
        let assign980_e787: f64 = (assign980_e783 / assign980_e786);
        (assign980_e787, ((((locals.var_tmp1_dn0 + (locals.var_zk_dn0 / locals.var_zk)) * assign980_e786) - (assign980_e783 * locals.var_zk_dn0)) / (assign980_e786 * assign980_e786)), ((((locals.var_tmp1_dn1 + (locals.var_zk_dn1 / locals.var_zk)) * assign980_e786) - (assign980_e783 * locals.var_zk_dn1)) / (assign980_e786 * assign980_e786)), ((((locals.var_tmp1_dn2 + (locals.var_zk_dn2 / locals.var_zk)) * assign980_e786) - (assign980_e783 * locals.var_zk_dn2)) / (assign980_e786 * assign980_e786)), ((((locals.var_tmp1_dn3 + (locals.var_zk_dn3 / locals.var_zk)) * assign980_e786) - (assign980_e783 * locals.var_zk_dn3)) / (assign980_e786 * assign980_e786)),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign980_e789;
        locals.var_yk_dn0 = assign980_e789_d_n0;
        locals.var_yk_dn1 = assign980_e789_d_n1;
        locals.var_yk_dn2 = assign980_e789_d_n2;
        locals.var_yk_dn3 = assign980_e789_d_n3;
        locals.var_yk_rv = 0.0;

        let assign990_e792: f64 = (-23.0);
        let assign990_e793: f64 = if locals.var_tmp1 > assign990_e792 { 1.0 } else { 0.0 };
        locals.var_guard9 = assign990_e793;
        locals.var_guard9_rv = 0.0;

        let (assign1000_e809, assign1000_e809_d_n0, assign1000_e809_d_n1, assign1000_e809_d_n2, assign1000_e809_d_n3,) = {
    if (((locals.var_guard7 == 0.0) && (locals.var_guard8 == 0.0)) && (locals.var_guard9 != 0.0)) {
        let assign1000_e804: f64 = (-locals.var_tmp1);
        let assign1000_e805: f64 = (assign1000_e804).exp();
        let assign1000_e806: f64 = (2.0 + assign1000_e805);
        let assign1000_e807: f64 = (1.0 / assign1000_e806);
        (assign1000_e807, (-((assign1000_e805 * (-locals.var_tmp1_dn0)) / (assign1000_e806 * assign1000_e806))), (-((assign1000_e805 * (-locals.var_tmp1_dn1)) / (assign1000_e806 * assign1000_e806))), (-((assign1000_e805 * (-locals.var_tmp1_dn2)) / (assign1000_e806 * assign1000_e806))), (-((assign1000_e805 * (-locals.var_tmp1_dn3)) / (assign1000_e806 * assign1000_e806))),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1000_e809;
        locals.var_yk_dn0 = assign1000_e809_d_n0;
        locals.var_yk_dn1 = assign1000_e809_d_n1;
        locals.var_yk_dn2 = assign1000_e809_d_n2;
        locals.var_yk_dn3 = assign1000_e809_d_n3;
        locals.var_yk_rv = 0.0;

        let (assign1010_e823, assign1010_e823_d_n0, assign1010_e823_d_n1, assign1010_e823_d_n2, assign1010_e823_d_n3,) = {
    if (((locals.var_guard7 == 0.0) && (locals.var_guard8 == 0.0)) && (locals.var_guard9 == 0.0)) {
        let assign1010_e819: f64 = (locals.var_tmp1).exp();
        let assign1010_e821: f64 = (assign1010_e819 + 1e-64);
        (assign1010_e821, (assign1010_e819 * locals.var_tmp1_dn0), (assign1010_e819 * locals.var_tmp1_dn1), (assign1010_e819 * locals.var_tmp1_dn2), (assign1010_e819 * locals.var_tmp1_dn3),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1010_e823;
        locals.var_yk_dn0 = assign1010_e823_d_n0;
        locals.var_yk_dn1 = assign1010_e823_d_n1;
        locals.var_yk_dn2 = assign1010_e823_d_n2;
        locals.var_yk_dn3 = assign1010_e823_d_n3;
        locals.var_yk_rv = 0.0;

        let assign1020_e827: f64 = (1.0 + locals.var_yk);
        let assign1020_e828: f64 = (locals.var_yk * assign1020_e827);
        locals.var_if_ = assign1020_e828;
        locals.var_if__dn0 = ((locals.var_yk_dn0 * assign1020_e827) + (locals.var_yk * locals.var_yk_dn0));
        locals.var_if__dn1 = ((locals.var_yk_dn1 * assign1020_e827) + (locals.var_yk * locals.var_yk_dn1));
        locals.var_if__dn2 = ((locals.var_yk_dn2 * assign1020_e827) + (locals.var_yk * locals.var_yk_dn2));
        locals.var_if__dn3 = ((locals.var_yk_dn3 * assign1020_e827) + (locals.var_yk * locals.var_yk_dn3));
        locals.var_if__rv = 0.0;

        let assign1030_e830: f64 = (locals.var_if_).sqrt();
        locals.var_sqrt_if = assign1030_e830;
        locals.var_sqrt_if_dn0 = (locals.var_if__dn0 / (2.0 * assign1030_e830));
        locals.var_sqrt_if_dn1 = (locals.var_if__dn1 / (2.0 * assign1030_e830));
        locals.var_sqrt_if_dn2 = (locals.var_if__dn2 / (2.0 * assign1030_e830));
        locals.var_sqrt_if_dn3 = (locals.var_if__dn3 / (2.0 * assign1030_e830));
        locals.var_sqrt_if_rv = 0.0;

        locals.var_dif_dv = locals.var_yk;
        locals.var_dif_dv_dn0 = locals.var_yk_dn0;
        locals.var_dif_dv_dn1 = locals.var_yk_dn1;
        locals.var_dif_dv_dn2 = locals.var_yk_dn2;
        locals.var_dif_dv_dn3 = locals.var_yk_dn3;
        locals.var_dif_dv_rv = 0.0;

        let assign1050_e834: f64 = (locals.var_vt / locals.var_vc);
        locals.var_vt_vc = assign1050_e834;
        locals.var_vt_vc_rv = 0.0;

        let assign1060_e838: f64 = (locals.var_sqrt_if * locals.var_vt_vc);
        let assign1060_e839: f64 = (0.25 + assign1060_e838);
        let assign1060_e840: f64 = (assign1060_e839).sqrt();
        locals.var_vdss_sqrt = assign1060_e840;
        locals.var_vdss_sqrt_dn0 = ((locals.var_sqrt_if_dn0 * locals.var_vt_vc) / (2.0 * assign1060_e840));
        locals.var_vdss_sqrt_dn1 = ((locals.var_sqrt_if_dn1 * locals.var_vt_vc) / (2.0 * assign1060_e840));
        locals.var_vdss_sqrt_dn2 = ((locals.var_sqrt_if_dn2 * locals.var_vt_vc) / (2.0 * assign1060_e840));
        locals.var_vdss_sqrt_dn3 = ((locals.var_sqrt_if_dn3 * locals.var_vt_vc) / (2.0 * assign1060_e840));
        locals.var_vdss_sqrt_rv = 0.0;

        let assign1070_e844: f64 = (locals.var_vdss_sqrt - 0.5);
        let assign1070_e845: f64 = (locals.var_vc * assign1070_e844);
        locals.var_vdss = assign1070_e845;
        locals.var_vdss_dn0 = (locals.var_vc * locals.var_vdss_sqrt_dn0);
        locals.var_vdss_dn1 = (locals.var_vc * locals.var_vdss_sqrt_dn1);
        locals.var_vdss_dn2 = (locals.var_vc * locals.var_vdss_sqrt_dn2);
        locals.var_vdss_dn3 = (locals.var_vc * locals.var_vdss_sqrt_dn3);
        locals.var_vdss_rv = 0.0;

        let assign1080_e849: f64 = (locals.var_vd - locals.var_vs);
        let assign1080_e850: f64 = (0.5 * assign1080_e849);
        locals.var_vds = assign1080_e850;
        locals.var_vds_dn0 = (0.5 * (locals.var_vd_dn0 - locals.var_vs_dn0));
        locals.var_vds_dn2 = (0.5 * (locals.var_vd_dn2 - locals.var_vs_dn2));
        locals.var_vds_dn3 = (0.5 * (locals.var_vd_dn3 - locals.var_vs_dn3));
        locals.var_vds_rv = 0.0;

        let assign1090_e856: f64 = (locals.var_vdss * locals.var_inv_vt);
        let assign1090_e857: f64 = (locals.var_sqrt_if - assign1090_e856);
        let assign1090_e858: f64 = (p.p25 * assign1090_e857);
        let assign1090_e860: f64 = (assign1090_e858 + 0.015625);
        let assign1090_e861: f64 = (locals.var_vt_vt_16 * assign1090_e860);
        locals.var_deltav_2 = assign1090_e861;
        locals.var_deltav_2_dn0 = (locals.var_vt_vt_16 * (p.p25 * (locals.var_sqrt_if_dn0 - (locals.var_vdss_dn0 * locals.var_inv_vt))));
        locals.var_deltav_2_dn1 = (locals.var_vt_vt_16 * (p.p25 * (locals.var_sqrt_if_dn1 - (locals.var_vdss_dn1 * locals.var_inv_vt))));
        locals.var_deltav_2_dn2 = (locals.var_vt_vt_16 * (p.p25 * (locals.var_sqrt_if_dn2 - (locals.var_vdss_dn2 * locals.var_inv_vt))));
        locals.var_deltav_2_dn3 = (locals.var_vt_vt_16 * (p.p25 * (locals.var_sqrt_if_dn3 - (locals.var_vdss_dn3 * locals.var_inv_vt))));
        locals.var_deltav_2_rv = 0.0;

        let assign1100_e864: f64 = (locals.var_vdss * locals.var_vdss);
        let assign1100_e866: f64 = (assign1100_e864 + locals.var_deltav_2);
        let assign1100_e867: f64 = (assign1100_e866).sqrt();
        locals.var_sqrt_vdss_deltav = assign1100_e867;
        locals.var_sqrt_vdss_deltav_dn0 = ((((locals.var_vdss_dn0 * locals.var_vdss) + (locals.var_vdss * locals.var_vdss_dn0)) + locals.var_deltav_2_dn0) / (2.0 * assign1100_e867));
        locals.var_sqrt_vdss_deltav_dn1 = ((((locals.var_vdss_dn1 * locals.var_vdss) + (locals.var_vdss * locals.var_vdss_dn1)) + locals.var_deltav_2_dn1) / (2.0 * assign1100_e867));
        locals.var_sqrt_vdss_deltav_dn2 = ((((locals.var_vdss_dn2 * locals.var_vdss) + (locals.var_vdss * locals.var_vdss_dn2)) + locals.var_deltav_2_dn2) / (2.0 * assign1100_e867));
        locals.var_sqrt_vdss_deltav_dn3 = ((((locals.var_vdss_dn3 * locals.var_vdss) + (locals.var_vdss * locals.var_vdss_dn3)) + locals.var_deltav_2_dn3) / (2.0 * assign1100_e867));
        locals.var_sqrt_vdss_deltav_rv = 0.0;

        let assign1110_e870: f64 = (locals.var_vds - locals.var_vdss);
        let assign1110_e873: f64 = (locals.var_vds - locals.var_vdss);
        let assign1110_e874: f64 = (assign1110_e870 * assign1110_e873);
        let assign1110_e876: f64 = (assign1110_e874 + locals.var_deltav_2);
        let assign1110_e877: f64 = (assign1110_e876).sqrt();
        locals.var_sqrt_vds_vdss_deltav = assign1110_e877;
        locals.var_sqrt_vds_vdss_deltav_dn0 = (((((locals.var_vds_dn0 - locals.var_vdss_dn0) * assign1110_e873) + (assign1110_e870 * (locals.var_vds_dn0 - locals.var_vdss_dn0))) + locals.var_deltav_2_dn0) / (2.0 * assign1110_e877));
        locals.var_sqrt_vds_vdss_deltav_dn1 = (((((-locals.var_vdss_dn1) * assign1110_e873) + (assign1110_e870 * (-locals.var_vdss_dn1))) + locals.var_deltav_2_dn1) / (2.0 * assign1110_e877));
        locals.var_sqrt_vds_vdss_deltav_dn2 = (((((locals.var_vds_dn2 - locals.var_vdss_dn2) * assign1110_e873) + (assign1110_e870 * (locals.var_vds_dn2 - locals.var_vdss_dn2))) + locals.var_deltav_2_dn2) / (2.0 * assign1110_e877));
        locals.var_sqrt_vds_vdss_deltav_dn3 = (((((locals.var_vds_dn3 - locals.var_vdss_dn3) * assign1110_e873) + (assign1110_e870 * (locals.var_vds_dn3 - locals.var_vdss_dn3))) + locals.var_deltav_2_dn3) / (2.0 * assign1110_e877));
        locals.var_sqrt_vds_vdss_deltav_rv = 0.0;

        let assign1120_e880: f64 = (locals.var_sqrt_vdss_deltav - locals.var_sqrt_vds_vdss_deltav);
        locals.var_vip = assign1120_e880;
        locals.var_vip_dn0 = (locals.var_sqrt_vdss_deltav_dn0 - locals.var_sqrt_vds_vdss_deltav_dn0);
        locals.var_vip_dn1 = (locals.var_sqrt_vdss_deltav_dn1 - locals.var_sqrt_vds_vdss_deltav_dn1);
        locals.var_vip_dn2 = (locals.var_sqrt_vdss_deltav_dn2 - locals.var_sqrt_vds_vdss_deltav_dn2);
        locals.var_vip_dn3 = (locals.var_sqrt_vdss_deltav_dn3 - locals.var_sqrt_vds_vdss_deltav_dn3);
        locals.var_vip_rv = 0.0;

        let assign1130_e885: f64 = (locals.var_if_).ln();
        let assign1130_e886: f64 = (0.75 * assign1130_e885);
        let assign1130_e887: f64 = (locals.var_sqrt_if - assign1130_e886);
        let assign1130_e889: f64 = (assign1130_e887 * locals.var_vt_vc);
        let assign1130_e890: f64 = (0.25 + assign1130_e889);
        let assign1130_e891: f64 = (assign1130_e890).sqrt();
        locals.var_vdssprime_sqrt = assign1130_e891;
        locals.var_vdssprime_sqrt_dn0 = (((locals.var_sqrt_if_dn0 - (0.75 * (locals.var_if__dn0 / locals.var_if_))) * locals.var_vt_vc) / (2.0 * assign1130_e891));
        locals.var_vdssprime_sqrt_dn1 = (((locals.var_sqrt_if_dn1 - (0.75 * (locals.var_if__dn1 / locals.var_if_))) * locals.var_vt_vc) / (2.0 * assign1130_e891));
        locals.var_vdssprime_sqrt_dn2 = (((locals.var_sqrt_if_dn2 - (0.75 * (locals.var_if__dn2 / locals.var_if_))) * locals.var_vt_vc) / (2.0 * assign1130_e891));
        locals.var_vdssprime_sqrt_dn3 = (((locals.var_sqrt_if_dn3 - (0.75 * (locals.var_if__dn3 / locals.var_if_))) * locals.var_vt_vc) / (2.0 * assign1130_e891));
        locals.var_vdssprime_sqrt_rv = 0.0;

        let assign1140_e895: f64 = (locals.var_vdssprime_sqrt - 0.5);
        let assign1140_e896: f64 = (locals.var_vc * assign1140_e895);
        let assign1140_e898: f64 = (assign1140_e896 + locals.var_log_vc_vt);
        locals.var_vdssprime = assign1140_e898;
        locals.var_vdssprime_dn0 = (locals.var_vc * locals.var_vdssprime_sqrt_dn0);
        locals.var_vdssprime_dn1 = (locals.var_vc * locals.var_vdssprime_sqrt_dn1);
        locals.var_vdssprime_dn2 = (locals.var_vc * locals.var_vdssprime_sqrt_dn2);
        locals.var_vdssprime_dn3 = (locals.var_vc * locals.var_vdssprime_sqrt_dn3);
        locals.var_vdssprime_rv = 0.0;

        let assign1150_e901: f64 = (locals.var_vds - locals.var_vdssprime);
        locals.var_vdsprime = assign1150_e901;
        locals.var_vdsprime_dn0 = (locals.var_vds_dn0 - locals.var_vdssprime_dn0);
        locals.var_vdsprime_dn1 = (-locals.var_vdssprime_dn1);
        locals.var_vdsprime_dn2 = (locals.var_vds_dn2 - locals.var_vdssprime_dn2);
        locals.var_vdsprime_dn3 = (locals.var_vds_dn3 - locals.var_vdssprime_dn3);
        locals.var_vdsprime_rv = 0.0;

        let assign1160_e904: f64 = (locals.var_vdssprime * locals.var_vdssprime);
        let assign1160_e906: f64 = (assign1160_e904 + locals.var_deltav_2);
        let assign1160_e907: f64 = (assign1160_e906).sqrt();
        locals.var_sqrt_vdssprime_deltav = assign1160_e907;
        locals.var_sqrt_vdssprime_deltav_dn0 = ((((locals.var_vdssprime_dn0 * locals.var_vdssprime) + (locals.var_vdssprime * locals.var_vdssprime_dn0)) + locals.var_deltav_2_dn0) / (2.0 * assign1160_e907));
        locals.var_sqrt_vdssprime_deltav_dn1 = ((((locals.var_vdssprime_dn1 * locals.var_vdssprime) + (locals.var_vdssprime * locals.var_vdssprime_dn1)) + locals.var_deltav_2_dn1) / (2.0 * assign1160_e907));
        locals.var_sqrt_vdssprime_deltav_dn2 = ((((locals.var_vdssprime_dn2 * locals.var_vdssprime) + (locals.var_vdssprime * locals.var_vdssprime_dn2)) + locals.var_deltav_2_dn2) / (2.0 * assign1160_e907));
        locals.var_sqrt_vdssprime_deltav_dn3 = ((((locals.var_vdssprime_dn3 * locals.var_vdssprime) + (locals.var_vdssprime * locals.var_vdssprime_dn3)) + locals.var_deltav_2_dn3) / (2.0 * assign1160_e907));
        locals.var_sqrt_vdssprime_deltav_rv = 0.0;

        let assign1170_e910: f64 = (locals.var_vdsprime * locals.var_vdsprime);
        let assign1170_e912: f64 = (assign1170_e910 + locals.var_deltav_2);
        let assign1170_e913: f64 = (assign1170_e912).sqrt();
        locals.var_sqrt_vds_vdssprime_deltav = assign1170_e913;
        locals.var_sqrt_vds_vdssprime_deltav_dn0 = ((((locals.var_vdsprime_dn0 * locals.var_vdsprime) + (locals.var_vdsprime * locals.var_vdsprime_dn0)) + locals.var_deltav_2_dn0) / (2.0 * assign1170_e913));
        locals.var_sqrt_vds_vdssprime_deltav_dn1 = ((((locals.var_vdsprime_dn1 * locals.var_vdsprime) + (locals.var_vdsprime * locals.var_vdsprime_dn1)) + locals.var_deltav_2_dn1) / (2.0 * assign1170_e913));
        locals.var_sqrt_vds_vdssprime_deltav_dn2 = ((((locals.var_vdsprime_dn2 * locals.var_vdsprime) + (locals.var_vdsprime * locals.var_vdsprime_dn2)) + locals.var_deltav_2_dn2) / (2.0 * assign1170_e913));
        locals.var_sqrt_vds_vdssprime_deltav_dn3 = ((((locals.var_vdsprime_dn3 * locals.var_vdsprime) + (locals.var_vdsprime * locals.var_vdsprime_dn3)) + locals.var_deltav_2_dn3) / (2.0 * assign1170_e913));
        locals.var_sqrt_vds_vdssprime_deltav_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        locals: &mut StampLocals,
    ) {
        let assign1180_e916: f64 = (locals.var_vp - locals.var_vds);
        let assign1180_e918: f64 = (assign1180_e916 - locals.var_vs);
        let assign1180_e920: f64 = (assign1180_e918 - locals.var_sqrt_vdssprime_deltav);
        let assign1180_e922: f64 = (assign1180_e920 + locals.var_sqrt_vds_vdssprime_deltav);
        let assign1180_e924: f64 = (assign1180_e922 * locals.var_inv_vt);
        locals.var_tmp1 = assign1180_e924;
        locals.var_tmp1_dn0 = (((((locals.var_vp_dn0 - locals.var_vds_dn0) - locals.var_vs_dn0) - locals.var_sqrt_vdssprime_deltav_dn0) + locals.var_sqrt_vds_vdssprime_deltav_dn0) * locals.var_inv_vt);
        locals.var_tmp1_dn1 = (((locals.var_vp_dn1 - locals.var_sqrt_vdssprime_deltav_dn1) + locals.var_sqrt_vds_vdssprime_deltav_dn1) * locals.var_inv_vt);
        locals.var_tmp1_dn2 = (((((locals.var_vp_dn2 - locals.var_vds_dn2) - locals.var_vs_dn2) - locals.var_sqrt_vdssprime_deltav_dn2) + locals.var_sqrt_vds_vdssprime_deltav_dn2) * locals.var_inv_vt);
        locals.var_tmp1_dn3 = (((((locals.var_vp_dn3 - locals.var_vds_dn3) - locals.var_vs_dn3) - locals.var_sqrt_vdssprime_deltav_dn3) + locals.var_sqrt_vds_vdssprime_deltav_dn3) * locals.var_inv_vt);
        locals.var_tmp1_rv = 0.0;

        let assign1190_e927: f64 = (-0.35);
        let assign1190_e928: f64 = if locals.var_tmp1 > assign1190_e927 { 1.0 } else { 0.0 };
        locals.var_guard10 = assign1190_e928;
        locals.var_guard10_rv = 0.0;

        let (assign1200_e941, assign1200_e941_d_n0, assign1200_e941_d_n1, assign1200_e941_d_n2, assign1200_e941_d_n3,) = {
    if (locals.var_guard10 != 0.0) {
        let assign1200_e933: f64 = (1.3 + locals.var_tmp1);
        let assign1200_e936: f64 = (locals.var_tmp1 + 1.6);
        let assign1200_e937: f64 = (assign1200_e936).ln();
        let assign1200_e938: f64 = (assign1200_e933 - assign1200_e937);
        let assign1200_e939: f64 = (2.0 / assign1200_e938);
        (assign1200_e939, (-((2.0 * (locals.var_tmp1_dn0 - (locals.var_tmp1_dn0 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))), (-((2.0 * (locals.var_tmp1_dn1 - (locals.var_tmp1_dn1 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))), (-((2.0 * (locals.var_tmp1_dn2 - (locals.var_tmp1_dn2 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))), (-((2.0 * (locals.var_tmp1_dn3 - (locals.var_tmp1_dn3 / assign1200_e936))) / (assign1200_e938 * assign1200_e938))),)
    } else {
        (locals.var_z0, locals.var_z0_dn0, locals.var_z0_dn1, locals.var_z0_dn2, locals.var_z0_dn3,)
    }
};
        locals.var_z0 = assign1200_e941;
        locals.var_z0_dn0 = assign1200_e941_d_n0;
        locals.var_z0_dn1 = assign1200_e941_d_n1;
        locals.var_z0_dn2 = assign1200_e941_d_n2;
        locals.var_z0_dn3 = assign1200_e941_d_n3;
        locals.var_z0_rv = 0.0;

        let (assign1210_e954, assign1210_e954_d_n0, assign1210_e954_d_n1, assign1210_e954_d_n2, assign1210_e954_d_n3,) = {
    if (locals.var_guard10 != 0.0) {
        let assign1210_e945: f64 = (2.0 + locals.var_z0);
        let assign1210_e948: f64 = (1.0 + locals.var_tmp1);
        let assign1210_e950: f64 = (locals.var_z0).ln();
        let assign1210_e951: f64 = (assign1210_e948 + assign1210_e950);
        let assign1210_e952: f64 = (assign1210_e945 / assign1210_e951);
        (assign1210_e952, (((locals.var_z0_dn0 * assign1210_e951) - (assign1210_e945 * (locals.var_tmp1_dn0 + (locals.var_z0_dn0 / locals.var_z0)))) / (assign1210_e951 * assign1210_e951)), (((locals.var_z0_dn1 * assign1210_e951) - (assign1210_e945 * (locals.var_tmp1_dn1 + (locals.var_z0_dn1 / locals.var_z0)))) / (assign1210_e951 * assign1210_e951)), (((locals.var_z0_dn2 * assign1210_e951) - (assign1210_e945 * (locals.var_tmp1_dn2 + (locals.var_z0_dn2 / locals.var_z0)))) / (assign1210_e951 * assign1210_e951)), (((locals.var_z0_dn3 * assign1210_e951) - (assign1210_e945 * (locals.var_tmp1_dn3 + (locals.var_z0_dn3 / locals.var_z0)))) / (assign1210_e951 * assign1210_e951)),)
    } else {
        (locals.var_zk, locals.var_zk_dn0, locals.var_zk_dn1, locals.var_zk_dn2, locals.var_zk_dn3,)
    }
};
        locals.var_zk = assign1210_e954;
        locals.var_zk_dn0 = assign1210_e954_d_n0;
        locals.var_zk_dn1 = assign1210_e954_d_n1;
        locals.var_zk_dn2 = assign1210_e954_d_n2;
        locals.var_zk_dn3 = assign1210_e954_d_n3;
        locals.var_zk_rv = 0.0;

        let (assign1220_e967, assign1220_e967_d_n0, assign1220_e967_d_n1, assign1220_e967_d_n2, assign1220_e967_d_n3,) = {
    if (locals.var_guard10 != 0.0) {
        let assign1220_e958: f64 = (1.0 + locals.var_tmp1);
        let assign1220_e960: f64 = (locals.var_zk).ln();
        let assign1220_e961: f64 = (assign1220_e958 + assign1220_e960);
        let assign1220_e964: f64 = (2.0 + locals.var_zk);
        let assign1220_e965: f64 = (assign1220_e961 / assign1220_e964);
        (assign1220_e965, ((((locals.var_tmp1_dn0 + (locals.var_zk_dn0 / locals.var_zk)) * assign1220_e964) - (assign1220_e961 * locals.var_zk_dn0)) / (assign1220_e964 * assign1220_e964)), ((((locals.var_tmp1_dn1 + (locals.var_zk_dn1 / locals.var_zk)) * assign1220_e964) - (assign1220_e961 * locals.var_zk_dn1)) / (assign1220_e964 * assign1220_e964)), ((((locals.var_tmp1_dn2 + (locals.var_zk_dn2 / locals.var_zk)) * assign1220_e964) - (assign1220_e961 * locals.var_zk_dn2)) / (assign1220_e964 * assign1220_e964)), ((((locals.var_tmp1_dn3 + (locals.var_zk_dn3 / locals.var_zk)) * assign1220_e964) - (assign1220_e961 * locals.var_zk_dn3)) / (assign1220_e964 * assign1220_e964)),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1220_e967;
        locals.var_yk_dn0 = assign1220_e967_d_n0;
        locals.var_yk_dn1 = assign1220_e967_d_n1;
        locals.var_yk_dn2 = assign1220_e967_d_n2;
        locals.var_yk_dn3 = assign1220_e967_d_n3;
        locals.var_yk_rv = 0.0;

        let assign1230_e970: f64 = (-15.0);
        let assign1230_e971: f64 = if locals.var_tmp1 > assign1230_e970 { 1.0 } else { 0.0 };
        locals.var_guard11 = assign1230_e971;
        locals.var_guard11_rv = 0.0;

        let (assign1240_e982, assign1240_e982_d_n0, assign1240_e982_d_n1, assign1240_e982_d_n2, assign1240_e982_d_n3,) = {
    if ((locals.var_guard10 == 0.0) && (locals.var_guard11 != 0.0)) {
        let assign1240_e978: f64 = (-locals.var_tmp1);
        let assign1240_e979: f64 = (assign1240_e978).exp();
        let assign1240_e980: f64 = (1.55 + assign1240_e979);
        (assign1240_e980, (assign1240_e979 * (-locals.var_tmp1_dn0)), (assign1240_e979 * (-locals.var_tmp1_dn1)), (assign1240_e979 * (-locals.var_tmp1_dn2)), (assign1240_e979 * (-locals.var_tmp1_dn3)),)
    } else {
        (locals.var_z0, locals.var_z0_dn0, locals.var_z0_dn1, locals.var_z0_dn2, locals.var_z0_dn3,)
    }
};
        locals.var_z0 = assign1240_e982;
        locals.var_z0_dn0 = assign1240_e982_d_n0;
        locals.var_z0_dn1 = assign1240_e982_d_n1;
        locals.var_z0_dn2 = assign1240_e982_d_n2;
        locals.var_z0_dn3 = assign1240_e982_d_n3;
        locals.var_z0_rv = 0.0;

        let (assign1250_e998, assign1250_e998_d_n0, assign1250_e998_d_n1, assign1250_e998_d_n2, assign1250_e998_d_n3,) = {
    if ((locals.var_guard10 == 0.0) && (locals.var_guard11 != 0.0)) {
        let assign1250_e989: f64 = (2.0 + locals.var_z0);
        let assign1250_e992: f64 = (1.0 + locals.var_tmp1);
        let assign1250_e994: f64 = (locals.var_z0).ln();
        let assign1250_e995: f64 = (assign1250_e992 + assign1250_e994);
        let assign1250_e996: f64 = (assign1250_e989 / assign1250_e995);
        (assign1250_e996, (((locals.var_z0_dn0 * assign1250_e995) - (assign1250_e989 * (locals.var_tmp1_dn0 + (locals.var_z0_dn0 / locals.var_z0)))) / (assign1250_e995 * assign1250_e995)), (((locals.var_z0_dn1 * assign1250_e995) - (assign1250_e989 * (locals.var_tmp1_dn1 + (locals.var_z0_dn1 / locals.var_z0)))) / (assign1250_e995 * assign1250_e995)), (((locals.var_z0_dn2 * assign1250_e995) - (assign1250_e989 * (locals.var_tmp1_dn2 + (locals.var_z0_dn2 / locals.var_z0)))) / (assign1250_e995 * assign1250_e995)), (((locals.var_z0_dn3 * assign1250_e995) - (assign1250_e989 * (locals.var_tmp1_dn3 + (locals.var_z0_dn3 / locals.var_z0)))) / (assign1250_e995 * assign1250_e995)),)
    } else {
        (locals.var_zk, locals.var_zk_dn0, locals.var_zk_dn1, locals.var_zk_dn2, locals.var_zk_dn3,)
    }
};
        locals.var_zk = assign1250_e998;
        locals.var_zk_dn0 = assign1250_e998_d_n0;
        locals.var_zk_dn1 = assign1250_e998_d_n1;
        locals.var_zk_dn2 = assign1250_e998_d_n2;
        locals.var_zk_dn3 = assign1250_e998_d_n3;
        locals.var_zk_rv = 0.0;

        let (assign1260_e1014, assign1260_e1014_d_n0, assign1260_e1014_d_n1, assign1260_e1014_d_n2, assign1260_e1014_d_n3,) = {
    if ((locals.var_guard10 == 0.0) && (locals.var_guard11 != 0.0)) {
        let assign1260_e1005: f64 = (1.0 + locals.var_tmp1);
        let assign1260_e1007: f64 = (locals.var_zk).ln();
        let assign1260_e1008: f64 = (assign1260_e1005 + assign1260_e1007);
        let assign1260_e1011: f64 = (2.0 + locals.var_zk);
        let assign1260_e1012: f64 = (assign1260_e1008 / assign1260_e1011);
        (assign1260_e1012, ((((locals.var_tmp1_dn0 + (locals.var_zk_dn0 / locals.var_zk)) * assign1260_e1011) - (assign1260_e1008 * locals.var_zk_dn0)) / (assign1260_e1011 * assign1260_e1011)), ((((locals.var_tmp1_dn1 + (locals.var_zk_dn1 / locals.var_zk)) * assign1260_e1011) - (assign1260_e1008 * locals.var_zk_dn1)) / (assign1260_e1011 * assign1260_e1011)), ((((locals.var_tmp1_dn2 + (locals.var_zk_dn2 / locals.var_zk)) * assign1260_e1011) - (assign1260_e1008 * locals.var_zk_dn2)) / (assign1260_e1011 * assign1260_e1011)), ((((locals.var_tmp1_dn3 + (locals.var_zk_dn3 / locals.var_zk)) * assign1260_e1011) - (assign1260_e1008 * locals.var_zk_dn3)) / (assign1260_e1011 * assign1260_e1011)),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1260_e1014;
        locals.var_yk_dn0 = assign1260_e1014_d_n0;
        locals.var_yk_dn1 = assign1260_e1014_d_n1;
        locals.var_yk_dn2 = assign1260_e1014_d_n2;
        locals.var_yk_dn3 = assign1260_e1014_d_n3;
        locals.var_yk_rv = 0.0;

        let assign1270_e1017: f64 = (-23.0);
        let assign1270_e1018: f64 = if locals.var_tmp1 > assign1270_e1017 { 1.0 } else { 0.0 };
        locals.var_guard12 = assign1270_e1018;
        locals.var_guard12_rv = 0.0;

        let (assign1280_e1034, assign1280_e1034_d_n0, assign1280_e1034_d_n1, assign1280_e1034_d_n2, assign1280_e1034_d_n3,) = {
    if (((locals.var_guard10 == 0.0) && (locals.var_guard11 == 0.0)) && (locals.var_guard12 != 0.0)) {
        let assign1280_e1029: f64 = (-locals.var_tmp1);
        let assign1280_e1030: f64 = (assign1280_e1029).exp();
        let assign1280_e1031: f64 = (2.0 + assign1280_e1030);
        let assign1280_e1032: f64 = (1.0 / assign1280_e1031);
        (assign1280_e1032, (-((assign1280_e1030 * (-locals.var_tmp1_dn0)) / (assign1280_e1031 * assign1280_e1031))), (-((assign1280_e1030 * (-locals.var_tmp1_dn1)) / (assign1280_e1031 * assign1280_e1031))), (-((assign1280_e1030 * (-locals.var_tmp1_dn2)) / (assign1280_e1031 * assign1280_e1031))), (-((assign1280_e1030 * (-locals.var_tmp1_dn3)) / (assign1280_e1031 * assign1280_e1031))),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1280_e1034;
        locals.var_yk_dn0 = assign1280_e1034_d_n0;
        locals.var_yk_dn1 = assign1280_e1034_d_n1;
        locals.var_yk_dn2 = assign1280_e1034_d_n2;
        locals.var_yk_dn3 = assign1280_e1034_d_n3;
        locals.var_yk_rv = 0.0;

        let (assign1290_e1048, assign1290_e1048_d_n0, assign1290_e1048_d_n1, assign1290_e1048_d_n2, assign1290_e1048_d_n3,) = {
    if (((locals.var_guard10 == 0.0) && (locals.var_guard11 == 0.0)) && (locals.var_guard12 == 0.0)) {
        let assign1290_e1044: f64 = (locals.var_tmp1).exp();
        let assign1290_e1046: f64 = (assign1290_e1044 + 1e-64);
        (assign1290_e1046, (assign1290_e1044 * locals.var_tmp1_dn0), (assign1290_e1044 * locals.var_tmp1_dn1), (assign1290_e1044 * locals.var_tmp1_dn2), (assign1290_e1044 * locals.var_tmp1_dn3),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1290_e1048;
        locals.var_yk_dn0 = assign1290_e1048_d_n0;
        locals.var_yk_dn1 = assign1290_e1048_d_n1;
        locals.var_yk_dn2 = assign1290_e1048_d_n2;
        locals.var_yk_dn3 = assign1290_e1048_d_n3;
        locals.var_yk_rv = 0.0;

        let assign1300_e1052: f64 = (1.0 + locals.var_yk);
        let assign1300_e1053: f64 = (locals.var_yk * assign1300_e1052);
        locals.var_irprime = assign1300_e1053;
        locals.var_irprime_dn0 = ((locals.var_yk_dn0 * assign1300_e1052) + (locals.var_yk * locals.var_yk_dn0));
        locals.var_irprime_dn1 = ((locals.var_yk_dn1 * assign1300_e1052) + (locals.var_yk * locals.var_yk_dn1));
        locals.var_irprime_dn2 = ((locals.var_yk_dn2 * assign1300_e1052) + (locals.var_yk * locals.var_yk_dn2));
        locals.var_irprime_dn3 = ((locals.var_yk_dn3 * assign1300_e1052) + (locals.var_yk * locals.var_yk_dn3));
        locals.var_irprime_rv = 0.0;

        locals.var_dirprime_dv = locals.var_yk;
        locals.var_dirprime_dv_dn0 = locals.var_yk_dn0;
        locals.var_dirprime_dv_dn1 = locals.var_yk_dn1;
        locals.var_dirprime_dv_dn2 = locals.var_yk_dn2;
        locals.var_dirprime_dv_dn3 = locals.var_yk_dn3;
        locals.var_dirprime_dv_rv = 0.0;

        let assign1330_e1061: f64 = (locals.var_vds - locals.var_vip);
        let assign1330_e1063: f64 = (assign1330_e1061 / locals.var_lc_ucrit);
        let assign1330_e1064: f64 = (1.0 + assign1330_e1063);
        let assign1330_e1065: f64 = (assign1330_e1064).ln();
        let assign1330_e1066: f64 = (locals.var_lc_lambda * assign1330_e1065);
        locals.var_deltal = assign1330_e1066;
        locals.var_deltal_dn0 = (locals.var_lc_lambda * (((locals.var_vds_dn0 - locals.var_vip_dn0) / locals.var_lc_ucrit) / assign1330_e1064));
        locals.var_deltal_dn1 = (locals.var_lc_lambda * (((-locals.var_vip_dn1) / locals.var_lc_ucrit) / assign1330_e1064));
        locals.var_deltal_dn2 = (locals.var_lc_lambda * (((locals.var_vds_dn2 - locals.var_vip_dn2) / locals.var_lc_ucrit) / assign1330_e1064));
        locals.var_deltal_dn3 = (locals.var_lc_lambda * (((locals.var_vds_dn3 - locals.var_vip_dn3) / locals.var_lc_ucrit) / assign1330_e1064));
        locals.var_deltal_rv = 0.0;

        let assign1340_e1069: f64 = (locals.var_leff - locals.var_deltal);
        let assign1340_e1072: f64 = (locals.var_vds + locals.var_vip);
        let assign1340_e1074: f64 = (assign1340_e1072 * locals.var_inv_ucrit);
        let assign1340_e1075: f64 = (assign1340_e1069 + assign1340_e1074);
        locals.var_lprime = assign1340_e1075;
        locals.var_lprime_dn0 = ((-locals.var_deltal_dn0) + ((locals.var_vds_dn0 + locals.var_vip_dn0) * locals.var_inv_ucrit));
        locals.var_lprime_dn1 = ((-locals.var_deltal_dn1) + (locals.var_vip_dn1 * locals.var_inv_ucrit));
        locals.var_lprime_dn2 = ((-locals.var_deltal_dn2) + ((locals.var_vds_dn2 + locals.var_vip_dn2) * locals.var_inv_ucrit));
        locals.var_lprime_dn3 = ((-locals.var_deltal_dn3) + ((locals.var_vds_dn3 + locals.var_vip_dn3) * locals.var_inv_ucrit));
        locals.var_lprime_rv = 0.0;

        let assign1350_e1078: f64 = (0.1 * locals.var_leff);
        locals.var_lmin = assign1350_e1078;
        locals.var_lmin_rv = 0.0;

        let assign1360_e1081: f64 = (locals.var_lprime * locals.var_lprime);
        let assign1360_e1084: f64 = (locals.var_lmin * locals.var_lmin);
        let assign1360_e1085: f64 = (assign1360_e1081 + assign1360_e1084);
        let assign1360_e1086: f64 = (assign1360_e1085).sqrt();
        locals.var_sqrt_lprime_lmin = assign1360_e1086;
        locals.var_sqrt_lprime_lmin_dn0 = (((locals.var_lprime_dn0 * locals.var_lprime) + (locals.var_lprime * locals.var_lprime_dn0)) / (2.0 * assign1360_e1086));
        locals.var_sqrt_lprime_lmin_dn1 = (((locals.var_lprime_dn1 * locals.var_lprime) + (locals.var_lprime * locals.var_lprime_dn1)) / (2.0 * assign1360_e1086));
        locals.var_sqrt_lprime_lmin_dn2 = (((locals.var_lprime_dn2 * locals.var_lprime) + (locals.var_lprime * locals.var_lprime_dn2)) / (2.0 * assign1360_e1086));
        locals.var_sqrt_lprime_lmin_dn3 = (((locals.var_lprime_dn3 * locals.var_lprime) + (locals.var_lprime * locals.var_lprime_dn3)) / (2.0 * assign1360_e1086));
        locals.var_sqrt_lprime_lmin_rv = 0.0;

        let assign1370_e1090: f64 = (locals.var_lprime + locals.var_sqrt_lprime_lmin);
        let assign1370_e1091: f64 = (0.5 * assign1370_e1090);
        locals.var_leq = assign1370_e1091;
        locals.var_leq_dn0 = (0.5 * (locals.var_lprime_dn0 + locals.var_sqrt_lprime_lmin_dn0));
        locals.var_leq_dn1 = (0.5 * (locals.var_lprime_dn1 + locals.var_sqrt_lprime_lmin_dn1));
        locals.var_leq_dn2 = (0.5 * (locals.var_lprime_dn2 + locals.var_sqrt_lprime_lmin_dn2));
        locals.var_leq_dn3 = (0.5 * (locals.var_lprime_dn3 + locals.var_sqrt_lprime_lmin_dn3));
        locals.var_leq_rv = 0.0;

        let assign1380_e1094: f64 = (locals.var_vp - locals.var_vd);
        let assign1380_e1096: f64 = (assign1380_e1094 * locals.var_inv_vt);
        locals.var_tmp1 = assign1380_e1096;
        locals.var_tmp1_dn0 = ((locals.var_vp_dn0 - locals.var_vd_dn0) * locals.var_inv_vt);
        locals.var_tmp1_dn1 = (locals.var_vp_dn1 * locals.var_inv_vt);
        locals.var_tmp1_dn2 = ((locals.var_vp_dn2 - locals.var_vd_dn2) * locals.var_inv_vt);
        locals.var_tmp1_dn3 = ((locals.var_vp_dn3 - locals.var_vd_dn3) * locals.var_inv_vt);
        locals.var_tmp1_rv = 0.0;

        let assign1390_e1099: f64 = (-0.35);
        let assign1390_e1100: f64 = if locals.var_tmp1 > assign1390_e1099 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign1390_e1100;
        locals.var_guard13_rv = 0.0;

        let (assign1400_e1113, assign1400_e1113_d_n0, assign1400_e1113_d_n1, assign1400_e1113_d_n2, assign1400_e1113_d_n3,) = {
    if (locals.var_guard13 != 0.0) {
        let assign1400_e1105: f64 = (1.3 + locals.var_tmp1);
        let assign1400_e1108: f64 = (locals.var_tmp1 + 1.6);
        let assign1400_e1109: f64 = (assign1400_e1108).ln();
        let assign1400_e1110: f64 = (assign1400_e1105 - assign1400_e1109);
        let assign1400_e1111: f64 = (2.0 / assign1400_e1110);
        (assign1400_e1111, (-((2.0 * (locals.var_tmp1_dn0 - (locals.var_tmp1_dn0 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))), (-((2.0 * (locals.var_tmp1_dn1 - (locals.var_tmp1_dn1 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))), (-((2.0 * (locals.var_tmp1_dn2 - (locals.var_tmp1_dn2 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))), (-((2.0 * (locals.var_tmp1_dn3 - (locals.var_tmp1_dn3 / assign1400_e1108))) / (assign1400_e1110 * assign1400_e1110))),)
    } else {
        (locals.var_z0, locals.var_z0_dn0, locals.var_z0_dn1, locals.var_z0_dn2, locals.var_z0_dn3,)
    }
};
        locals.var_z0 = assign1400_e1113;
        locals.var_z0_dn0 = assign1400_e1113_d_n0;
        locals.var_z0_dn1 = assign1400_e1113_d_n1;
        locals.var_z0_dn2 = assign1400_e1113_d_n2;
        locals.var_z0_dn3 = assign1400_e1113_d_n3;
        locals.var_z0_rv = 0.0;

        let (assign1410_e1126, assign1410_e1126_d_n0, assign1410_e1126_d_n1, assign1410_e1126_d_n2, assign1410_e1126_d_n3,) = {
    if (locals.var_guard13 != 0.0) {
        let assign1410_e1117: f64 = (2.0 + locals.var_z0);
        let assign1410_e1120: f64 = (1.0 + locals.var_tmp1);
        let assign1410_e1122: f64 = (locals.var_z0).ln();
        let assign1410_e1123: f64 = (assign1410_e1120 + assign1410_e1122);
        let assign1410_e1124: f64 = (assign1410_e1117 / assign1410_e1123);
        (assign1410_e1124, (((locals.var_z0_dn0 * assign1410_e1123) - (assign1410_e1117 * (locals.var_tmp1_dn0 + (locals.var_z0_dn0 / locals.var_z0)))) / (assign1410_e1123 * assign1410_e1123)), (((locals.var_z0_dn1 * assign1410_e1123) - (assign1410_e1117 * (locals.var_tmp1_dn1 + (locals.var_z0_dn1 / locals.var_z0)))) / (assign1410_e1123 * assign1410_e1123)), (((locals.var_z0_dn2 * assign1410_e1123) - (assign1410_e1117 * (locals.var_tmp1_dn2 + (locals.var_z0_dn2 / locals.var_z0)))) / (assign1410_e1123 * assign1410_e1123)), (((locals.var_z0_dn3 * assign1410_e1123) - (assign1410_e1117 * (locals.var_tmp1_dn3 + (locals.var_z0_dn3 / locals.var_z0)))) / (assign1410_e1123 * assign1410_e1123)),)
    } else {
        (locals.var_zk, locals.var_zk_dn0, locals.var_zk_dn1, locals.var_zk_dn2, locals.var_zk_dn3,)
    }
};
        locals.var_zk = assign1410_e1126;
        locals.var_zk_dn0 = assign1410_e1126_d_n0;
        locals.var_zk_dn1 = assign1410_e1126_d_n1;
        locals.var_zk_dn2 = assign1410_e1126_d_n2;
        locals.var_zk_dn3 = assign1410_e1126_d_n3;
        locals.var_zk_rv = 0.0;

        let (assign1420_e1139, assign1420_e1139_d_n0, assign1420_e1139_d_n1, assign1420_e1139_d_n2, assign1420_e1139_d_n3,) = {
    if (locals.var_guard13 != 0.0) {
        let assign1420_e1130: f64 = (1.0 + locals.var_tmp1);
        let assign1420_e1132: f64 = (locals.var_zk).ln();
        let assign1420_e1133: f64 = (assign1420_e1130 + assign1420_e1132);
        let assign1420_e1136: f64 = (2.0 + locals.var_zk);
        let assign1420_e1137: f64 = (assign1420_e1133 / assign1420_e1136);
        (assign1420_e1137, ((((locals.var_tmp1_dn0 + (locals.var_zk_dn0 / locals.var_zk)) * assign1420_e1136) - (assign1420_e1133 * locals.var_zk_dn0)) / (assign1420_e1136 * assign1420_e1136)), ((((locals.var_tmp1_dn1 + (locals.var_zk_dn1 / locals.var_zk)) * assign1420_e1136) - (assign1420_e1133 * locals.var_zk_dn1)) / (assign1420_e1136 * assign1420_e1136)), ((((locals.var_tmp1_dn2 + (locals.var_zk_dn2 / locals.var_zk)) * assign1420_e1136) - (assign1420_e1133 * locals.var_zk_dn2)) / (assign1420_e1136 * assign1420_e1136)), ((((locals.var_tmp1_dn3 + (locals.var_zk_dn3 / locals.var_zk)) * assign1420_e1136) - (assign1420_e1133 * locals.var_zk_dn3)) / (assign1420_e1136 * assign1420_e1136)),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1420_e1139;
        locals.var_yk_dn0 = assign1420_e1139_d_n0;
        locals.var_yk_dn1 = assign1420_e1139_d_n1;
        locals.var_yk_dn2 = assign1420_e1139_d_n2;
        locals.var_yk_dn3 = assign1420_e1139_d_n3;
        locals.var_yk_rv = 0.0;

        let assign1430_e1142: f64 = (-15.0);
        let assign1430_e1143: f64 = if locals.var_tmp1 > assign1430_e1142 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign1430_e1143;
        locals.var_guard14_rv = 0.0;

        let (assign1440_e1154, assign1440_e1154_d_n0, assign1440_e1154_d_n1, assign1440_e1154_d_n2, assign1440_e1154_d_n3,) = {
    if ((locals.var_guard13 == 0.0) && (locals.var_guard14 != 0.0)) {
        let assign1440_e1150: f64 = (-locals.var_tmp1);
        let assign1440_e1151: f64 = (assign1440_e1150).exp();
        let assign1440_e1152: f64 = (1.55 + assign1440_e1151);
        (assign1440_e1152, (assign1440_e1151 * (-locals.var_tmp1_dn0)), (assign1440_e1151 * (-locals.var_tmp1_dn1)), (assign1440_e1151 * (-locals.var_tmp1_dn2)), (assign1440_e1151 * (-locals.var_tmp1_dn3)),)
    } else {
        (locals.var_z0, locals.var_z0_dn0, locals.var_z0_dn1, locals.var_z0_dn2, locals.var_z0_dn3,)
    }
};
        locals.var_z0 = assign1440_e1154;
        locals.var_z0_dn0 = assign1440_e1154_d_n0;
        locals.var_z0_dn1 = assign1440_e1154_d_n1;
        locals.var_z0_dn2 = assign1440_e1154_d_n2;
        locals.var_z0_dn3 = assign1440_e1154_d_n3;
        locals.var_z0_rv = 0.0;

        let (assign1450_e1170, assign1450_e1170_d_n0, assign1450_e1170_d_n1, assign1450_e1170_d_n2, assign1450_e1170_d_n3,) = {
    if ((locals.var_guard13 == 0.0) && (locals.var_guard14 != 0.0)) {
        let assign1450_e1161: f64 = (2.0 + locals.var_z0);
        let assign1450_e1164: f64 = (1.0 + locals.var_tmp1);
        let assign1450_e1166: f64 = (locals.var_z0).ln();
        let assign1450_e1167: f64 = (assign1450_e1164 + assign1450_e1166);
        let assign1450_e1168: f64 = (assign1450_e1161 / assign1450_e1167);
        (assign1450_e1168, (((locals.var_z0_dn0 * assign1450_e1167) - (assign1450_e1161 * (locals.var_tmp1_dn0 + (locals.var_z0_dn0 / locals.var_z0)))) / (assign1450_e1167 * assign1450_e1167)), (((locals.var_z0_dn1 * assign1450_e1167) - (assign1450_e1161 * (locals.var_tmp1_dn1 + (locals.var_z0_dn1 / locals.var_z0)))) / (assign1450_e1167 * assign1450_e1167)), (((locals.var_z0_dn2 * assign1450_e1167) - (assign1450_e1161 * (locals.var_tmp1_dn2 + (locals.var_z0_dn2 / locals.var_z0)))) / (assign1450_e1167 * assign1450_e1167)), (((locals.var_z0_dn3 * assign1450_e1167) - (assign1450_e1161 * (locals.var_tmp1_dn3 + (locals.var_z0_dn3 / locals.var_z0)))) / (assign1450_e1167 * assign1450_e1167)),)
    } else {
        (locals.var_zk, locals.var_zk_dn0, locals.var_zk_dn1, locals.var_zk_dn2, locals.var_zk_dn3,)
    }
};
        locals.var_zk = assign1450_e1170;
        locals.var_zk_dn0 = assign1450_e1170_d_n0;
        locals.var_zk_dn1 = assign1450_e1170_d_n1;
        locals.var_zk_dn2 = assign1450_e1170_d_n2;
        locals.var_zk_dn3 = assign1450_e1170_d_n3;
        locals.var_zk_rv = 0.0;

        let (assign1460_e1186, assign1460_e1186_d_n0, assign1460_e1186_d_n1, assign1460_e1186_d_n2, assign1460_e1186_d_n3,) = {
    if ((locals.var_guard13 == 0.0) && (locals.var_guard14 != 0.0)) {
        let assign1460_e1177: f64 = (1.0 + locals.var_tmp1);
        let assign1460_e1179: f64 = (locals.var_zk).ln();
        let assign1460_e1180: f64 = (assign1460_e1177 + assign1460_e1179);
        let assign1460_e1183: f64 = (2.0 + locals.var_zk);
        let assign1460_e1184: f64 = (assign1460_e1180 / assign1460_e1183);
        (assign1460_e1184, ((((locals.var_tmp1_dn0 + (locals.var_zk_dn0 / locals.var_zk)) * assign1460_e1183) - (assign1460_e1180 * locals.var_zk_dn0)) / (assign1460_e1183 * assign1460_e1183)), ((((locals.var_tmp1_dn1 + (locals.var_zk_dn1 / locals.var_zk)) * assign1460_e1183) - (assign1460_e1180 * locals.var_zk_dn1)) / (assign1460_e1183 * assign1460_e1183)), ((((locals.var_tmp1_dn2 + (locals.var_zk_dn2 / locals.var_zk)) * assign1460_e1183) - (assign1460_e1180 * locals.var_zk_dn2)) / (assign1460_e1183 * assign1460_e1183)), ((((locals.var_tmp1_dn3 + (locals.var_zk_dn3 / locals.var_zk)) * assign1460_e1183) - (assign1460_e1180 * locals.var_zk_dn3)) / (assign1460_e1183 * assign1460_e1183)),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1460_e1186;
        locals.var_yk_dn0 = assign1460_e1186_d_n0;
        locals.var_yk_dn1 = assign1460_e1186_d_n1;
        locals.var_yk_dn2 = assign1460_e1186_d_n2;
        locals.var_yk_dn3 = assign1460_e1186_d_n3;
        locals.var_yk_rv = 0.0;

        let assign1470_e1189: f64 = (-23.0);
        let assign1470_e1190: f64 = if locals.var_tmp1 > assign1470_e1189 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1470_e1190;
        locals.var_guard15_rv = 0.0;

        let (assign1480_e1206, assign1480_e1206_d_n0, assign1480_e1206_d_n1, assign1480_e1206_d_n2, assign1480_e1206_d_n3,) = {
    if (((locals.var_guard13 == 0.0) && (locals.var_guard14 == 0.0)) && (locals.var_guard15 != 0.0)) {
        let assign1480_e1201: f64 = (-locals.var_tmp1);
        let assign1480_e1202: f64 = (assign1480_e1201).exp();
        let assign1480_e1203: f64 = (2.0 + assign1480_e1202);
        let assign1480_e1204: f64 = (1.0 / assign1480_e1203);
        (assign1480_e1204, (-((assign1480_e1202 * (-locals.var_tmp1_dn0)) / (assign1480_e1203 * assign1480_e1203))), (-((assign1480_e1202 * (-locals.var_tmp1_dn1)) / (assign1480_e1203 * assign1480_e1203))), (-((assign1480_e1202 * (-locals.var_tmp1_dn2)) / (assign1480_e1203 * assign1480_e1203))), (-((assign1480_e1202 * (-locals.var_tmp1_dn3)) / (assign1480_e1203 * assign1480_e1203))),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1480_e1206;
        locals.var_yk_dn0 = assign1480_e1206_d_n0;
        locals.var_yk_dn1 = assign1480_e1206_d_n1;
        locals.var_yk_dn2 = assign1480_e1206_d_n2;
        locals.var_yk_dn3 = assign1480_e1206_d_n3;
        locals.var_yk_rv = 0.0;

        let (assign1490_e1220, assign1490_e1220_d_n0, assign1490_e1220_d_n1, assign1490_e1220_d_n2, assign1490_e1220_d_n3,) = {
    if (((locals.var_guard13 == 0.0) && (locals.var_guard14 == 0.0)) && (locals.var_guard15 == 0.0)) {
        let assign1490_e1216: f64 = (locals.var_tmp1).exp();
        let assign1490_e1218: f64 = (assign1490_e1216 + 1e-64);
        (assign1490_e1218, (assign1490_e1216 * locals.var_tmp1_dn0), (assign1490_e1216 * locals.var_tmp1_dn1), (assign1490_e1216 * locals.var_tmp1_dn2), (assign1490_e1216 * locals.var_tmp1_dn3),)
    } else {
        (locals.var_yk, locals.var_yk_dn0, locals.var_yk_dn1, locals.var_yk_dn2, locals.var_yk_dn3,)
    }
};
        locals.var_yk = assign1490_e1220;
        locals.var_yk_dn0 = assign1490_e1220_d_n0;
        locals.var_yk_dn1 = assign1490_e1220_d_n1;
        locals.var_yk_dn2 = assign1490_e1220_d_n2;
        locals.var_yk_dn3 = assign1490_e1220_d_n3;
        locals.var_yk_rv = 0.0;

        let assign1500_e1224: f64 = (1.0 + locals.var_yk);
        let assign1500_e1225: f64 = (locals.var_yk * assign1500_e1224);
        locals.var_ir = assign1500_e1225;
        locals.var_ir_dn0 = ((locals.var_yk_dn0 * assign1500_e1224) + (locals.var_yk * locals.var_yk_dn0));
        locals.var_ir_dn1 = ((locals.var_yk_dn1 * assign1500_e1224) + (locals.var_yk * locals.var_yk_dn1));
        locals.var_ir_dn2 = ((locals.var_yk_dn2 * assign1500_e1224) + (locals.var_yk * locals.var_yk_dn2));
        locals.var_ir_dn3 = ((locals.var_yk_dn3 * assign1500_e1224) + (locals.var_yk * locals.var_yk_dn3));
        locals.var_ir_rv = 0.0;

        locals.var_dir_dv = locals.var_yk;
        locals.var_dir_dv_dn0 = locals.var_yk_dn0;
        locals.var_dir_dv_dn1 = locals.var_yk_dn1;
        locals.var_dir_dv_dn2 = locals.var_yk_dn2;
        locals.var_dir_dv_dn3 = locals.var_yk_dn3;
        locals.var_dir_dv_rv = 0.0;

        let assign1530_e1231: f64 = (0.25 + locals.var_if_);
        locals.var_sif2 = assign1530_e1231;
        locals.var_sif2_dn0 = locals.var_if__dn0;
        locals.var_sif2_dn1 = locals.var_if__dn1;
        locals.var_sif2_dn2 = locals.var_if__dn2;
        locals.var_sif2_dn3 = locals.var_if__dn3;
        locals.var_sif2_rv = 0.0;

        let assign1540_e1234: f64 = (0.25 + locals.var_ir);
        locals.var_sir2 = assign1540_e1234;
        locals.var_sir2_dn0 = locals.var_ir_dn0;
        locals.var_sir2_dn1 = locals.var_ir_dn1;
        locals.var_sir2_dn2 = locals.var_ir_dn2;
        locals.var_sir2_dn3 = locals.var_ir_dn3;
        locals.var_sir2_rv = 0.0;

        let assign1550_e1236: f64 = (locals.var_sif2).sqrt();
        locals.var_sif = assign1550_e1236;
        locals.var_sif_dn0 = (locals.var_sif2_dn0 / (2.0 * assign1550_e1236));
        locals.var_sif_dn1 = (locals.var_sif2_dn1 / (2.0 * assign1550_e1236));
        locals.var_sif_dn2 = (locals.var_sif2_dn2 / (2.0 * assign1550_e1236));
        locals.var_sif_dn3 = (locals.var_sif2_dn3 / (2.0 * assign1550_e1236));
        locals.var_sif_rv = 0.0;

        let assign1560_e1238: f64 = (locals.var_sir2).sqrt();
        locals.var_sir = assign1560_e1238;
        locals.var_sir_dn0 = (locals.var_sir2_dn0 / (2.0 * assign1560_e1238));
        locals.var_sir_dn1 = (locals.var_sir2_dn1 / (2.0 * assign1560_e1238));
        locals.var_sir_dn2 = (locals.var_sir2_dn2 / (2.0 * assign1560_e1238));
        locals.var_sir_dn3 = (locals.var_sir2_dn3 / (2.0 * assign1560_e1238));
        locals.var_sir_rv = 0.0;

        let assign1570_e1241: f64 = (locals.var_sif + locals.var_sir);
        let assign1570_e1244: f64 = (locals.var_sif + locals.var_sir);
        let assign1570_e1245: f64 = (assign1570_e1241 * assign1570_e1244);
        locals.var_sif_sir_2 = assign1570_e1245;
        locals.var_sif_sir_2_dn0 = (((locals.var_sif_dn0 + locals.var_sir_dn0) * assign1570_e1244) + (assign1570_e1241 * (locals.var_sif_dn0 + locals.var_sir_dn0)));
        locals.var_sif_sir_2_dn1 = (((locals.var_sif_dn1 + locals.var_sir_dn1) * assign1570_e1244) + (assign1570_e1241 * (locals.var_sif_dn1 + locals.var_sir_dn1)));
        locals.var_sif_sir_2_dn2 = (((locals.var_sif_dn2 + locals.var_sir_dn2) * assign1570_e1244) + (assign1570_e1241 * (locals.var_sif_dn2 + locals.var_sir_dn2)));
        locals.var_sif_sir_2_dn3 = (((locals.var_sif_dn3 + locals.var_sir_dn3) * assign1570_e1244) + (assign1570_e1241 * (locals.var_sif_dn3 + locals.var_sir_dn3)));
        locals.var_sif_sir_2_rv = 0.0;

        let assign1580_e1248: f64 = (locals.var_vp + locals.var_phi_t);
        let assign1580_e1250: f64 = (assign1580_e1248 + 1e-6);
        locals.var_vp_phi_eps = assign1580_e1250;
        locals.var_vp_phi_eps_dn0 = (locals.var_vp_dn0 + locals.var_phi_t_dn0);
        locals.var_vp_phi_eps_dn1 = (locals.var_vp_dn1 + locals.var_phi_t_dn1);
        locals.var_vp_phi_eps_dn2 = (locals.var_vp_dn2 + locals.var_phi_t_dn2);
        locals.var_vp_phi_eps_dn3 = (locals.var_vp_dn3 + locals.var_phi_t_dn3);
        locals.var_vp_phi_eps_rv = 0.0;

        let assign1590_e1253: f64 = (locals.var_vp_phi_eps).sqrt();
        let assign1590_e1254: f64 = (2.0 * assign1590_e1253);
        locals.var_sqrt_phi_vp_2 = assign1590_e1254;
        locals.var_sqrt_phi_vp_2_dn0 = (2.0 * (locals.var_vp_phi_eps_dn0 / (2.0 * assign1590_e1253)));
        locals.var_sqrt_phi_vp_2_dn1 = (2.0 * (locals.var_vp_phi_eps_dn1 / (2.0 * assign1590_e1253)));
        locals.var_sqrt_phi_vp_2_dn2 = (2.0 * (locals.var_vp_phi_eps_dn2 / (2.0 * assign1590_e1253)));
        locals.var_sqrt_phi_vp_2_dn3 = (2.0 * (locals.var_vp_phi_eps_dn3 / (2.0 * assign1590_e1253)));
        locals.var_sqrt_phi_vp_2_rv = 0.0;

        let assign1600_e1257: f64 = (locals.var_gamma_s / locals.var_sqrt_phi_vp_2);
        locals.var_n_1 = assign1600_e1257;
        locals.var_n_1_dn0 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn0) / (locals.var_sqrt_phi_vp_2 * locals.var_sqrt_phi_vp_2)));
        locals.var_n_1_dn1 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn1) / (locals.var_sqrt_phi_vp_2 * locals.var_sqrt_phi_vp_2)));
        locals.var_n_1_dn2 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn2) / (locals.var_sqrt_phi_vp_2 * locals.var_sqrt_phi_vp_2)));
        locals.var_n_1_dn3 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn3) / (locals.var_sqrt_phi_vp_2 * locals.var_sqrt_phi_vp_2)));
        locals.var_n_1_rv = 0.0;

        let assign1610_e1261: f64 = (locals.var_sqrt_phi_vp_2 + locals.var_gamma_s);
        let assign1610_e1262: f64 = (locals.var_gamma_s / assign1610_e1261);
        locals.var_n_1_n = assign1610_e1262;
        locals.var_n_1_n_dn0 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn0) / (assign1610_e1261 * assign1610_e1261)));
        locals.var_n_1_n_dn1 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn1) / (assign1610_e1261 * assign1610_e1261)));
        locals.var_n_1_n_dn2 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn2) / (assign1610_e1261 * assign1610_e1261)));
        locals.var_n_1_n_dn3 = (-((locals.var_gamma_s * locals.var_sqrt_phi_vp_2_dn3) / (assign1610_e1261 * assign1610_e1261)));
        locals.var_n_1_n_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign1620_e1265: f64 = (1.0 + locals.var_n_1);
        let assign1620_e1266: f64 = (-assign1620_e1265);
        let assign1620_e1268: f64 = (assign1620_e1266 * locals.var_vt);
        let assign1620_e1271: f64 = (0.66666666 + 0.66666666);
        let assign1620_e1275: f64 = (locals.var_sir * locals.var_sif);
        let assign1620_e1276: f64 = (locals.var_sir2 + assign1620_e1275);
        let assign1620_e1278: f64 = (assign1620_e1276 + locals.var_sif2);
        let assign1620_e1279: f64 = (assign1620_e1271 * assign1620_e1278);
        let assign1620_e1282: f64 = (locals.var_sif + locals.var_sir);
        let assign1620_e1283: f64 = (assign1620_e1279 / assign1620_e1282);
        let assign1620_e1285: f64 = (assign1620_e1283 - 1.0);
        let assign1620_e1286: f64 = (assign1620_e1268 * assign1620_e1285);
        locals.var_qi = assign1620_e1286;
        locals.var_qi_dn0 = ((((-locals.var_n_1_dn0) * locals.var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((locals.var_sir2_dn0 + ((locals.var_sir_dn0 * locals.var_sif) + (locals.var_sir * locals.var_sif_dn0))) + locals.var_sif2_dn0)) * assign1620_e1282) - (assign1620_e1279 * (locals.var_sif_dn0 + locals.var_sir_dn0))) / (assign1620_e1282 * assign1620_e1282))));
        locals.var_qi_dn1 = ((((-locals.var_n_1_dn1) * locals.var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((locals.var_sir2_dn1 + ((locals.var_sir_dn1 * locals.var_sif) + (locals.var_sir * locals.var_sif_dn1))) + locals.var_sif2_dn1)) * assign1620_e1282) - (assign1620_e1279 * (locals.var_sif_dn1 + locals.var_sir_dn1))) / (assign1620_e1282 * assign1620_e1282))));
        locals.var_qi_dn2 = ((((-locals.var_n_1_dn2) * locals.var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((locals.var_sir2_dn2 + ((locals.var_sir_dn2 * locals.var_sif) + (locals.var_sir * locals.var_sif_dn2))) + locals.var_sif2_dn2)) * assign1620_e1282) - (assign1620_e1279 * (locals.var_sif_dn2 + locals.var_sir_dn2))) / (assign1620_e1282 * assign1620_e1282))));
        locals.var_qi_dn3 = ((((-locals.var_n_1_dn3) * locals.var_vt) * assign1620_e1285) + (assign1620_e1268 * ((((assign1620_e1271 * ((locals.var_sir2_dn3 + ((locals.var_sir_dn3 * locals.var_sif) + (locals.var_sir * locals.var_sif_dn3))) + locals.var_sif2_dn3)) * assign1620_e1282) - (assign1620_e1279 * (locals.var_sif_dn3 + locals.var_sir_dn3))) / (assign1620_e1282 * assign1620_e1282))));
        locals.var_qi_rv = 0.0;

        let assign1630_e1288: f64 = (-0.5);
        let assign1630_e1290: f64 = (assign1630_e1288 * locals.var_gamma_s);
        let assign1630_e1292: f64 = (assign1630_e1290 * locals.var_sqrt_phi_vp_2);
        let assign1630_e1295: f64 = (locals.var_n_1_n * locals.var_qi);
        let assign1630_e1296: f64 = (assign1630_e1292 - assign1630_e1295);
        locals.var_qb = assign1630_e1296;
        locals.var_qb_dn0 = ((assign1630_e1290 * locals.var_sqrt_phi_vp_2_dn0) - ((locals.var_n_1_n_dn0 * locals.var_qi) + (locals.var_n_1_n * locals.var_qi_dn0)));
        locals.var_qb_dn1 = ((assign1630_e1290 * locals.var_sqrt_phi_vp_2_dn1) - ((locals.var_n_1_n_dn1 * locals.var_qi) + (locals.var_n_1_n * locals.var_qi_dn1)));
        locals.var_qb_dn2 = ((assign1630_e1290 * locals.var_sqrt_phi_vp_2_dn2) - ((locals.var_n_1_n_dn2 * locals.var_qi) + (locals.var_n_1_n * locals.var_qi_dn2)));
        locals.var_qb_dn3 = ((assign1630_e1290 * locals.var_sqrt_phi_vp_2_dn3) - ((locals.var_n_1_n_dn3 * locals.var_qi) + (locals.var_n_1_n * locals.var_qi_dn3)));
        locals.var_qb_rv = 0.0;

        let assign1640_e1299: f64 = if p.p22 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1640_e1299;
        locals.var_guard16_rv = 0.0;

        let (assign1650_e1308, assign1650_e1308_d_n0, assign1650_e1308_d_n1, assign1650_e1308_d_n2, assign1650_e1308_d_n3,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1650_e1303: f64 = (locals.var_vp * locals.var_vp);
        let assign1650_e1305: f64 = (assign1650_e1303 + locals.var_vt_vt_2);
        let assign1650_e1306: f64 = (assign1650_e1305).sqrt();
        (assign1650_e1306, (((locals.var_vp_dn0 * locals.var_vp) + (locals.var_vp * locals.var_vp_dn0)) / (2.0 * assign1650_e1306)), (((locals.var_vp_dn1 * locals.var_vp) + (locals.var_vp * locals.var_vp_dn1)) / (2.0 * assign1650_e1306)), (((locals.var_vp_dn2 * locals.var_vp) + (locals.var_vp * locals.var_vp_dn2)) / (2.0 * assign1650_e1306)), (((locals.var_vp_dn3 * locals.var_vp) + (locals.var_vp * locals.var_vp_dn3)) / (2.0 * assign1650_e1306)),)
    } else {
        (locals.var_sqrt_vp_vt, locals.var_sqrt_vp_vt_dn0, locals.var_sqrt_vp_vt_dn1, locals.var_sqrt_vp_vt_dn2, locals.var_sqrt_vp_vt_dn3,)
    }
};
        locals.var_sqrt_vp_vt = assign1650_e1308;
        locals.var_sqrt_vp_vt_dn0 = assign1650_e1308_d_n0;
        locals.var_sqrt_vp_vt_dn1 = assign1650_e1308_d_n1;
        locals.var_sqrt_vp_vt_dn2 = assign1650_e1308_d_n2;
        locals.var_sqrt_vp_vt_dn3 = assign1650_e1308_d_n3;
        locals.var_sqrt_vp_vt_rv = 0.0;

        let (assign1660_e1316, assign1660_e1316_d_n0, assign1660_e1316_d_n1, assign1660_e1316_d_n2, assign1660_e1316_d_n3,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1660_e1313: f64 = (locals.var_vp + locals.var_sqrt_vp_vt);
        let assign1660_e1314: f64 = (0.5 * assign1660_e1313);
        (assign1660_e1314, (0.5 * (locals.var_vp_dn0 + locals.var_sqrt_vp_vt_dn0)), (0.5 * (locals.var_vp_dn1 + locals.var_sqrt_vp_vt_dn1)), (0.5 * (locals.var_vp_dn2 + locals.var_sqrt_vp_vt_dn2)), (0.5 * (locals.var_vp_dn3 + locals.var_sqrt_vp_vt_dn3)),)
    } else {
        (locals.var_vpprime, locals.var_vpprime_dn0, locals.var_vpprime_dn1, locals.var_vpprime_dn2, locals.var_vpprime_dn3,)
    }
};
        locals.var_vpprime = assign1660_e1316;
        locals.var_vpprime_dn0 = assign1660_e1316_d_n0;
        locals.var_vpprime_dn1 = assign1660_e1316_d_n1;
        locals.var_vpprime_dn2 = assign1660_e1316_d_n2;
        locals.var_vpprime_dn3 = assign1660_e1316_d_n3;
        locals.var_vpprime_rv = 0.0;

        let (assign1670_e1324, assign1670_e1324_d_n0, assign1670_e1324_d_n1, assign1670_e1324_d_n2, assign1670_e1324_d_n3,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1670_e1321: f64 = (p.p21 * locals.var_vpprime);
        let assign1670_e1322: f64 = (1.0 + assign1670_e1321);
        (assign1670_e1322, (p.p21 * locals.var_vpprime_dn0), (p.p21 * locals.var_vpprime_dn1), (p.p21 * locals.var_vpprime_dn2), (p.p21 * locals.var_vpprime_dn3),)
    } else {
        (locals.var_theta_vp_1, locals.var_theta_vp_1_dn0, locals.var_theta_vp_1_dn1, locals.var_theta_vp_1_dn2, locals.var_theta_vp_1_dn3,)
    }
};
        locals.var_theta_vp_1 = assign1670_e1324;
        locals.var_theta_vp_1_dn0 = assign1670_e1324_d_n0;
        locals.var_theta_vp_1_dn1 = assign1670_e1324_d_n1;
        locals.var_theta_vp_1_dn2 = assign1670_e1324_d_n2;
        locals.var_theta_vp_1_dn3 = assign1670_e1324_d_n3;
        locals.var_theta_vp_1_rv = 0.0;

        let (assign1680_e1332, assign1680_e1332_d_n0, assign1680_e1332_d_n1, assign1680_e1332_d_n2, assign1680_e1332_d_n3,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1680_e1329: f64 = (locals.var_leq * locals.var_theta_vp_1);
        let assign1680_e1330: f64 = (locals.var_kp_weff / assign1680_e1329);
        (assign1680_e1330, (-((locals.var_kp_weff * ((locals.var_leq_dn0 * locals.var_theta_vp_1) + (locals.var_leq * locals.var_theta_vp_1_dn0))) / (assign1680_e1329 * assign1680_e1329))), (-((locals.var_kp_weff * ((locals.var_leq_dn1 * locals.var_theta_vp_1) + (locals.var_leq * locals.var_theta_vp_1_dn1))) / (assign1680_e1329 * assign1680_e1329))), (-((locals.var_kp_weff * ((locals.var_leq_dn2 * locals.var_theta_vp_1) + (locals.var_leq * locals.var_theta_vp_1_dn2))) / (assign1680_e1329 * assign1680_e1329))), (-((locals.var_kp_weff * ((locals.var_leq_dn3 * locals.var_theta_vp_1) + (locals.var_leq * locals.var_theta_vp_1_dn3))) / (assign1680_e1329 * assign1680_e1329))),)
    } else {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn1, locals.var_beta_dn2, locals.var_beta_dn3,)
    }
};
        locals.var_beta = assign1680_e1332;
        locals.var_beta_dn0 = assign1680_e1332_d_n0;
        locals.var_beta_dn1 = assign1680_e1332_d_n1;
        locals.var_beta_dn2 = assign1680_e1332_d_n2;
        locals.var_beta_dn3 = assign1680_e1332_d_n3;
        locals.var_beta_rv = 0.0;

        let assign1690_e1336: f64 = (locals.var_eta_qi * locals.var_qi);
        let assign1690_e1337: f64 = (locals.var_qb + assign1690_e1336);
        let assign1690_e1339: f64 = if assign1690_e1337 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1690_e1339;
        locals.var_guard17_rv = 0.0;

        let (assign1700_e1354, assign1700_e1354_d_n0, assign1700_e1354_d_n1, assign1700_e1354_d_n2, assign1700_e1354_d_n3,) = {
    if ((locals.var_guard16 == 0.0) && (locals.var_guard17 != 0.0)) {
        let assign1700_e1349: f64 = (locals.var_eta_qi * locals.var_qi);
        let assign1700_e1350: f64 = (locals.var_qb + assign1700_e1349);
        let assign1700_e1351: f64 = (locals.var_t0 * assign1700_e1350);
        let assign1700_e1352: f64 = (1.0 + assign1700_e1351);
        (assign1700_e1352, (locals.var_t0 * (locals.var_qb_dn0 + (locals.var_eta_qi * locals.var_qi_dn0))), (locals.var_t0 * (locals.var_qb_dn1 + (locals.var_eta_qi * locals.var_qi_dn1))), (locals.var_t0 * (locals.var_qb_dn2 + (locals.var_eta_qi * locals.var_qi_dn2))), (locals.var_t0 * (locals.var_qb_dn3 + (locals.var_eta_qi * locals.var_qi_dn3))),)
    } else {
        (locals.var_e0_q_1, locals.var_e0_q_1_dn0, locals.var_e0_q_1_dn1, locals.var_e0_q_1_dn2, locals.var_e0_q_1_dn3,)
    }
};
        locals.var_e0_q_1 = assign1700_e1354;
        locals.var_e0_q_1_dn0 = assign1700_e1354_d_n0;
        locals.var_e0_q_1_dn1 = assign1700_e1354_d_n1;
        locals.var_e0_q_1_dn2 = assign1700_e1354_d_n2;
        locals.var_e0_q_1_dn3 = assign1700_e1354_d_n3;
        locals.var_e0_q_1_rv = 0.0;

        let (assign1710_e1370, assign1710_e1370_d_n0, assign1710_e1370_d_n1, assign1710_e1370_d_n2, assign1710_e1370_d_n3,) = {
    if ((locals.var_guard16 == 0.0) && (locals.var_guard17 == 0.0)) {
        let assign1710_e1365: f64 = (locals.var_eta_qi * locals.var_qi);
        let assign1710_e1366: f64 = (locals.var_qb + assign1710_e1365);
        let assign1710_e1367: f64 = (locals.var_t0 * assign1710_e1366);
        let assign1710_e1368: f64 = (1.0 - assign1710_e1367);
        (assign1710_e1368, (-(locals.var_t0 * (locals.var_qb_dn0 + (locals.var_eta_qi * locals.var_qi_dn0)))), (-(locals.var_t0 * (locals.var_qb_dn1 + (locals.var_eta_qi * locals.var_qi_dn1)))), (-(locals.var_t0 * (locals.var_qb_dn2 + (locals.var_eta_qi * locals.var_qi_dn2)))), (-(locals.var_t0 * (locals.var_qb_dn3 + (locals.var_eta_qi * locals.var_qi_dn3)))),)
    } else {
        (locals.var_e0_q_1, locals.var_e0_q_1_dn0, locals.var_e0_q_1_dn1, locals.var_e0_q_1_dn2, locals.var_e0_q_1_dn3,)
    }
};
        locals.var_e0_q_1 = assign1710_e1370;
        locals.var_e0_q_1_dn0 = assign1710_e1370_d_n0;
        locals.var_e0_q_1_dn1 = assign1710_e1370_d_n1;
        locals.var_e0_q_1_dn2 = assign1710_e1370_d_n2;
        locals.var_e0_q_1_dn3 = assign1710_e1370_d_n3;
        locals.var_e0_q_1_rv = 0.0;

        let (assign1720_e1379, assign1720_e1379_d_n0, assign1720_e1379_d_n1, assign1720_e1379_d_n2, assign1720_e1379_d_n3,) = {
    if (locals.var_guard16 == 0.0) {
        let assign1720_e1376: f64 = (locals.var_t0 * locals.var_gamma_sqrt_phi);
        let assign1720_e1377: f64 = (1.0 + assign1720_e1376);
        (assign1720_e1377, (locals.var_t0 * locals.var_gamma_sqrt_phi_dn0), (locals.var_t0 * locals.var_gamma_sqrt_phi_dn1), (locals.var_t0 * locals.var_gamma_sqrt_phi_dn2), (locals.var_t0 * locals.var_gamma_sqrt_phi_dn3),)
    } else {
        (locals.var_t0_gamma_1, locals.var_t0_gamma_1_dn0, locals.var_t0_gamma_1_dn1, locals.var_t0_gamma_1_dn2, locals.var_t0_gamma_1_dn3,)
    }
};
        locals.var_t0_gamma_1 = assign1720_e1379;
        locals.var_t0_gamma_1_dn0 = assign1720_e1379_d_n0;
        locals.var_t0_gamma_1_dn1 = assign1720_e1379_d_n1;
        locals.var_t0_gamma_1_dn2 = assign1720_e1379_d_n2;
        locals.var_t0_gamma_1_dn3 = assign1720_e1379_d_n3;
        locals.var_t0_gamma_1_rv = 0.0;

        let (assign1730_e1390, assign1730_e1390_d_n0, assign1730_e1390_d_n1, assign1730_e1390_d_n2, assign1730_e1390_d_n3,) = {
    if (locals.var_guard16 == 0.0) {
        let assign1730_e1384: f64 = (locals.var_kp_weff * locals.var_t0_gamma_1);
        let assign1730_e1387: f64 = (locals.var_leq * locals.var_e0_q_1);
        let assign1730_e1388: f64 = (assign1730_e1384 / assign1730_e1387);
        (assign1730_e1388, ((((locals.var_kp_weff * locals.var_t0_gamma_1_dn0) * assign1730_e1387) - (assign1730_e1384 * ((locals.var_leq_dn0 * locals.var_e0_q_1) + (locals.var_leq * locals.var_e0_q_1_dn0)))) / (assign1730_e1387 * assign1730_e1387)), ((((locals.var_kp_weff * locals.var_t0_gamma_1_dn1) * assign1730_e1387) - (assign1730_e1384 * ((locals.var_leq_dn1 * locals.var_e0_q_1) + (locals.var_leq * locals.var_e0_q_1_dn1)))) / (assign1730_e1387 * assign1730_e1387)), ((((locals.var_kp_weff * locals.var_t0_gamma_1_dn2) * assign1730_e1387) - (assign1730_e1384 * ((locals.var_leq_dn2 * locals.var_e0_q_1) + (locals.var_leq * locals.var_e0_q_1_dn2)))) / (assign1730_e1387 * assign1730_e1387)), ((((locals.var_kp_weff * locals.var_t0_gamma_1_dn3) * assign1730_e1387) - (assign1730_e1384 * ((locals.var_leq_dn3 * locals.var_e0_q_1) + (locals.var_leq * locals.var_e0_q_1_dn3)))) / (assign1730_e1387 * assign1730_e1387)),)
    } else {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn1, locals.var_beta_dn2, locals.var_beta_dn3,)
    }
};
        locals.var_beta = assign1730_e1390;
        locals.var_beta_dn0 = assign1730_e1390_d_n0;
        locals.var_beta_dn1 = assign1730_e1390_d_n1;
        locals.var_beta_dn2 = assign1730_e1390_d_n2;
        locals.var_beta_dn3 = assign1730_e1390_d_n3;
        locals.var_beta_rv = 0.0;

        let assign1740_e1393: f64 = (locals.var_phi_t + locals.var_vp);
        let assign1740_e1395: f64 = (assign1740_e1393 + locals.var_vt_4);
        let assign1740_e1396: f64 = (assign1740_e1395).sqrt();
        locals.var_sqrt_phi_vp = assign1740_e1396;
        locals.var_sqrt_phi_vp_dn0 = ((locals.var_phi_t_dn0 + locals.var_vp_dn0) / (2.0 * assign1740_e1396));
        locals.var_sqrt_phi_vp_dn1 = ((locals.var_phi_t_dn1 + locals.var_vp_dn1) / (2.0 * assign1740_e1396));
        locals.var_sqrt_phi_vp_dn2 = ((locals.var_phi_t_dn2 + locals.var_vp_dn2) / (2.0 * assign1740_e1396));
        locals.var_sqrt_phi_vp_dn3 = ((locals.var_phi_t_dn3 + locals.var_vp_dn3) / (2.0 * assign1740_e1396));
        locals.var_sqrt_phi_vp_rv = 0.0;

        let assign1750_e1401: f64 = (2.0 * locals.var_sqrt_phi_vp);
        let assign1750_e1402: f64 = (locals.var_gamma_s / assign1750_e1401);
        let assign1750_e1403: f64 = (1.0 + assign1750_e1402);
        locals.var_n = assign1750_e1403;
        locals.var_n_dn0 = (-((locals.var_gamma_s * (2.0 * locals.var_sqrt_phi_vp_dn0)) / (assign1750_e1401 * assign1750_e1401)));
        locals.var_n_dn1 = (-((locals.var_gamma_s * (2.0 * locals.var_sqrt_phi_vp_dn1)) / (assign1750_e1401 * assign1750_e1401)));
        locals.var_n_dn2 = (-((locals.var_gamma_s * (2.0 * locals.var_sqrt_phi_vp_dn2)) / (assign1750_e1401 * assign1750_e1401)));
        locals.var_n_dn3 = (-((locals.var_gamma_s * (2.0 * locals.var_sqrt_phi_vp_dn3)) / (assign1750_e1401 * assign1750_e1401)));
        locals.var_n_rv = 0.0;

        let assign1760_e1406: f64 = (locals.var_if_ - locals.var_irprime);
        locals.var_if_ir = assign1760_e1406;
        locals.var_if_ir_dn0 = (locals.var_if__dn0 - locals.var_irprime_dn0);
        locals.var_if_ir_dn1 = (locals.var_if__dn1 - locals.var_irprime_dn1);
        locals.var_if_ir_dn2 = (locals.var_if__dn2 - locals.var_irprime_dn2);
        locals.var_if_ir_dn3 = (locals.var_if__dn3 - locals.var_irprime_dn3);
        locals.var_if_ir_rv = 0.0;

        let assign1770_e1409: f64 = (locals.var_vt_vt_2 * locals.var_n);
        let assign1770_e1411: f64 = (assign1770_e1409 * locals.var_beta);
        locals.var_ispec = assign1770_e1411;
        locals.var_ispec_dn0 = (((locals.var_vt_vt_2 * locals.var_n_dn0) * locals.var_beta) + (assign1770_e1409 * locals.var_beta_dn0));
        locals.var_ispec_dn1 = (((locals.var_vt_vt_2 * locals.var_n_dn1) * locals.var_beta) + (assign1770_e1409 * locals.var_beta_dn1));
        locals.var_ispec_dn2 = (((locals.var_vt_vt_2 * locals.var_n_dn2) * locals.var_beta) + (assign1770_e1409 * locals.var_beta_dn2));
        locals.var_ispec_dn3 = (((locals.var_vt_vt_2 * locals.var_n_dn3) * locals.var_beta) + (assign1770_e1409 * locals.var_beta_dn3));
        locals.var_ispec_rv = 0.0;

        let assign1820_e1436: f64 = (locals.var_sqrt_gammastar + locals.var_sqrt_gammastar);
        let assign1820_e1437: f64 = (locals.var_gammaprime / assign1820_e1436);
        locals.var_tmp1 = assign1820_e1437;
        locals.var_tmp1_dn0 = (((locals.var_gammaprime_dn0 * assign1820_e1436) - (locals.var_gammaprime * (locals.var_sqrt_gammastar_dn0 + locals.var_sqrt_gammastar_dn0))) / (assign1820_e1436 * assign1820_e1436));
        locals.var_tmp1_dn1 = (((locals.var_gammaprime_dn1 * assign1820_e1436) - (locals.var_gammaprime * (locals.var_sqrt_gammastar_dn1 + locals.var_sqrt_gammastar_dn1))) / (assign1820_e1436 * assign1820_e1436));
        locals.var_tmp1_dn2 = (((locals.var_gammaprime_dn2 * assign1820_e1436) - (locals.var_gammaprime * (locals.var_sqrt_gammastar_dn2 + locals.var_sqrt_gammastar_dn2))) / (assign1820_e1436 * assign1820_e1436));
        locals.var_tmp1_dn3 = (((locals.var_gammaprime_dn3 * assign1820_e1436) - (locals.var_gammaprime * (locals.var_sqrt_gammastar_dn3 + locals.var_sqrt_gammastar_dn3))) / (assign1820_e1436 * assign1820_e1436));
        locals.var_tmp1_rv = 0.0;

        let assign1830_e1440: f64 = (locals.var_vgprime / locals.var_sqrt_vgstar);
        locals.var_tmp2 = assign1830_e1440;
        locals.var_tmp2_dn0 = (((locals.var_vgprime_dn0 * locals.var_sqrt_vgstar) - (locals.var_vgprime * locals.var_sqrt_vgstar_dn0)) / (locals.var_sqrt_vgstar * locals.var_sqrt_vgstar));
        locals.var_tmp2_dn1 = (((locals.var_vgprime_dn1 * locals.var_sqrt_vgstar) - (locals.var_vgprime * locals.var_sqrt_vgstar_dn1)) / (locals.var_sqrt_vgstar * locals.var_sqrt_vgstar));
        locals.var_tmp2_dn2 = (((locals.var_vgprime_dn2 * locals.var_sqrt_vgstar) - (locals.var_vgprime * locals.var_sqrt_vgstar_dn2)) / (locals.var_sqrt_vgstar * locals.var_sqrt_vgstar));
        locals.var_tmp2_dn3 = (((locals.var_vgprime_dn3 * locals.var_sqrt_vgstar) - (locals.var_vgprime * locals.var_sqrt_vgstar_dn3)) / (locals.var_sqrt_vgstar * locals.var_sqrt_vgstar));
        locals.var_tmp2_rv = 0.0;

        let assign1840_e1442: f64 = (-locals.var_leta_l);
        let assign1840_e1444: f64 = (assign1840_e1442 * locals.var_tmp1);
        let assign1840_e1446: f64 = (assign1840_e1444 * locals.var_sqrt_phi_vd);
        let assign1840_e1448: f64 = (assign1840_e1446 / locals.var_sqrt_phi_vd_vt);
        locals.var_dgammaprime_dvd = assign1840_e1448;
        locals.var_dgammaprime_dvd_dn0 = ((((((assign1840_e1442 * locals.var_tmp1_dn0) * locals.var_sqrt_phi_vd) + (assign1840_e1444 * locals.var_sqrt_phi_vd_dn0)) * locals.var_sqrt_phi_vd_vt) - (assign1840_e1446 * locals.var_sqrt_phi_vd_vt_dn0)) / (locals.var_sqrt_phi_vd_vt * locals.var_sqrt_phi_vd_vt));
        locals.var_dgammaprime_dvd_dn1 = ((((((assign1840_e1442 * locals.var_tmp1_dn1) * locals.var_sqrt_phi_vd) + (assign1840_e1444 * locals.var_sqrt_phi_vd_dn1)) * locals.var_sqrt_phi_vd_vt) - (assign1840_e1446 * locals.var_sqrt_phi_vd_vt_dn1)) / (locals.var_sqrt_phi_vd_vt * locals.var_sqrt_phi_vd_vt));
        locals.var_dgammaprime_dvd_dn2 = ((((((assign1840_e1442 * locals.var_tmp1_dn2) * locals.var_sqrt_phi_vd) + (assign1840_e1444 * locals.var_sqrt_phi_vd_dn2)) * locals.var_sqrt_phi_vd_vt) - (assign1840_e1446 * locals.var_sqrt_phi_vd_vt_dn2)) / (locals.var_sqrt_phi_vd_vt * locals.var_sqrt_phi_vd_vt));
        locals.var_dgammaprime_dvd_dn3 = ((((((assign1840_e1442 * locals.var_tmp1_dn3) * locals.var_sqrt_phi_vd) + (assign1840_e1444 * locals.var_sqrt_phi_vd_dn3)) * locals.var_sqrt_phi_vd_vt) - (assign1840_e1446 * locals.var_sqrt_phi_vd_vt_dn3)) / (locals.var_sqrt_phi_vd_vt * locals.var_sqrt_phi_vd_vt));
        locals.var_dgammaprime_dvd_rv = 0.0;

        let assign1850_e1450: f64 = (-locals.var_leta_l);
        let assign1850_e1452: f64 = (assign1850_e1450 * locals.var_tmp1);
        let assign1850_e1454: f64 = (assign1850_e1452 * locals.var_sqrt_phi_vs);
        let assign1850_e1456: f64 = (assign1850_e1454 / locals.var_sqrt_phi_vs_vt);
        locals.var_dgammaprime_dvs = assign1850_e1456;
        locals.var_dgammaprime_dvs_dn0 = ((((((assign1850_e1450 * locals.var_tmp1_dn0) * locals.var_sqrt_phi_vs) + (assign1850_e1452 * locals.var_sqrt_phi_vs_dn0)) * locals.var_sqrt_phi_vs_vt) - (assign1850_e1454 * locals.var_sqrt_phi_vs_vt_dn0)) / (locals.var_sqrt_phi_vs_vt * locals.var_sqrt_phi_vs_vt));
        locals.var_dgammaprime_dvs_dn1 = ((((((assign1850_e1450 * locals.var_tmp1_dn1) * locals.var_sqrt_phi_vs) + (assign1850_e1452 * locals.var_sqrt_phi_vs_dn1)) * locals.var_sqrt_phi_vs_vt) - (assign1850_e1454 * locals.var_sqrt_phi_vs_vt_dn1)) / (locals.var_sqrt_phi_vs_vt * locals.var_sqrt_phi_vs_vt));
        locals.var_dgammaprime_dvs_dn2 = ((((((assign1850_e1450 * locals.var_tmp1_dn2) * locals.var_sqrt_phi_vs) + (assign1850_e1452 * locals.var_sqrt_phi_vs_dn2)) * locals.var_sqrt_phi_vs_vt) - (assign1850_e1454 * locals.var_sqrt_phi_vs_vt_dn2)) / (locals.var_sqrt_phi_vs_vt * locals.var_sqrt_phi_vs_vt));
        locals.var_dgammaprime_dvs_dn3 = ((((((assign1850_e1450 * locals.var_tmp1_dn3) * locals.var_sqrt_phi_vs) + (assign1850_e1452 * locals.var_sqrt_phi_vs_dn3)) * locals.var_sqrt_phi_vs_vt) - (assign1850_e1454 * locals.var_sqrt_phi_vs_vt_dn3)) / (locals.var_sqrt_phi_vs_vt * locals.var_sqrt_phi_vs_vt));
        locals.var_dgammaprime_dvs_rv = 0.0;

        let assign1870_e1474: f64 = (locals.var_vp + locals.var_phi_t);
        let assign1870_e1476: f64 = (assign1870_e1474 / locals.var_big_sqrt_vp);
        locals.var_tmp3 = assign1870_e1476;
        locals.var_tmp3_dn0 = ((((locals.var_vp_dn0 + locals.var_phi_t_dn0) * locals.var_big_sqrt_vp) - (assign1870_e1474 * locals.var_big_sqrt_vp_dn0)) / (locals.var_big_sqrt_vp * locals.var_big_sqrt_vp));
        locals.var_tmp3_dn1 = ((((locals.var_vp_dn1 + locals.var_phi_t_dn1) * locals.var_big_sqrt_vp) - (assign1870_e1474 * locals.var_big_sqrt_vp_dn1)) / (locals.var_big_sqrt_vp * locals.var_big_sqrt_vp));
        locals.var_tmp3_dn2 = ((((locals.var_vp_dn2 + locals.var_phi_t_dn2) * locals.var_big_sqrt_vp) - (assign1870_e1474 * locals.var_big_sqrt_vp_dn2)) / (locals.var_big_sqrt_vp * locals.var_big_sqrt_vp));
        locals.var_tmp3_dn3 = ((((locals.var_vp_dn3 + locals.var_phi_t_dn3) * locals.var_big_sqrt_vp) - (assign1870_e1474 * locals.var_big_sqrt_vp_dn3)) / (locals.var_big_sqrt_vp * locals.var_big_sqrt_vp));
        locals.var_tmp3_rv = 0.0;

        let assign1880_e1478: f64 = (-locals.var_tmp3);
        let assign1880_e1480: f64 = (assign1880_e1478 * locals.var_dgammaprime_dvd);
        locals.var_dvp_dvd = assign1880_e1480;
        locals.var_dvp_dvd_dn0 = (((-locals.var_tmp3_dn0) * locals.var_dgammaprime_dvd) + (assign1880_e1478 * locals.var_dgammaprime_dvd_dn0));
        locals.var_dvp_dvd_dn1 = (((-locals.var_tmp3_dn1) * locals.var_dgammaprime_dvd) + (assign1880_e1478 * locals.var_dgammaprime_dvd_dn1));
        locals.var_dvp_dvd_dn2 = (((-locals.var_tmp3_dn2) * locals.var_dgammaprime_dvd) + (assign1880_e1478 * locals.var_dgammaprime_dvd_dn2));
        locals.var_dvp_dvd_dn3 = (((-locals.var_tmp3_dn3) * locals.var_dgammaprime_dvd) + (assign1880_e1478 * locals.var_dgammaprime_dvd_dn3));
        locals.var_dvp_dvd_rv = 0.0;

        let assign1890_e1482: f64 = (-locals.var_tmp3);
        let assign1890_e1484: f64 = (assign1890_e1482 * locals.var_dgammaprime_dvs);
        locals.var_dvp_dvs = assign1890_e1484;
        locals.var_dvp_dvs_dn0 = (((-locals.var_tmp3_dn0) * locals.var_dgammaprime_dvs) + (assign1890_e1482 * locals.var_dgammaprime_dvs_dn0));
        locals.var_dvp_dvs_dn1 = (((-locals.var_tmp3_dn1) * locals.var_dgammaprime_dvs) + (assign1890_e1482 * locals.var_dgammaprime_dvs_dn1));
        locals.var_dvp_dvs_dn2 = (((-locals.var_tmp3_dn2) * locals.var_dgammaprime_dvs) + (assign1890_e1482 * locals.var_dgammaprime_dvs_dn2));
        locals.var_dvp_dvs_dn3 = (((-locals.var_tmp3_dn3) * locals.var_dgammaprime_dvs) + (assign1890_e1482 * locals.var_dgammaprime_dvs_dn3));
        locals.var_dvp_dvs_rv = 0.0;

        let assign1910_e1501: f64 = (locals.var_dif_dv * locals.var_inv_vt);
        locals.var_tmp1 = assign1910_e1501;
        locals.var_tmp1_dn0 = (locals.var_dif_dv_dn0 * locals.var_inv_vt);
        locals.var_tmp1_dn1 = (locals.var_dif_dv_dn1 * locals.var_inv_vt);
        locals.var_tmp1_dn2 = (locals.var_dif_dv_dn2 * locals.var_inv_vt);
        locals.var_tmp1_dn3 = (locals.var_dif_dv_dn3 * locals.var_inv_vt);
        locals.var_tmp1_rv = 0.0;

        let assign1920_e1504: f64 = (locals.var_tmp1 * locals.var_dvp_dvd);
        locals.var_dif_dvd = assign1920_e1504;
        locals.var_dif_dvd_dn0 = ((locals.var_tmp1_dn0 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn0));
        locals.var_dif_dvd_dn1 = ((locals.var_tmp1_dn1 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn1));
        locals.var_dif_dvd_dn2 = ((locals.var_tmp1_dn2 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn2));
        locals.var_dif_dvd_dn3 = ((locals.var_tmp1_dn3 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn3));
        locals.var_dif_dvd_rv = 0.0;

        let assign1930_e1508: f64 = (locals.var_dvp_dvs - 1.0);
        let assign1930_e1509: f64 = (locals.var_tmp1 * assign1930_e1508);
        locals.var_dif_dvs = assign1930_e1509;
        locals.var_dif_dvs_dn0 = ((locals.var_tmp1_dn0 * assign1930_e1508) + (locals.var_tmp1 * locals.var_dvp_dvs_dn0));
        locals.var_dif_dvs_dn1 = ((locals.var_tmp1_dn1 * assign1930_e1508) + (locals.var_tmp1 * locals.var_dvp_dvs_dn1));
        locals.var_dif_dvs_dn2 = ((locals.var_tmp1_dn2 * assign1930_e1508) + (locals.var_tmp1 * locals.var_dvp_dvs_dn2));
        locals.var_dif_dvs_dn3 = ((locals.var_tmp1_dn3 * assign1930_e1508) + (locals.var_tmp1 * locals.var_dvp_dvs_dn3));
        locals.var_dif_dvs_rv = 0.0;

        let assign1950_e1516: f64 = (4.0 * locals.var_vdss_sqrt);
        let assign1950_e1518: f64 = (assign1950_e1516 * locals.var_sqrt_if);
        let assign1950_e1519: f64 = (locals.var_vt / assign1950_e1518);
        locals.var_tmp1 = assign1950_e1519;
        locals.var_tmp1_dn0 = (-((locals.var_vt * (((4.0 * locals.var_vdss_sqrt_dn0) * locals.var_sqrt_if) + (assign1950_e1516 * locals.var_sqrt_if_dn0))) / (assign1950_e1518 * assign1950_e1518)));
        locals.var_tmp1_dn1 = (-((locals.var_vt * (((4.0 * locals.var_vdss_sqrt_dn1) * locals.var_sqrt_if) + (assign1950_e1516 * locals.var_sqrt_if_dn1))) / (assign1950_e1518 * assign1950_e1518)));
        locals.var_tmp1_dn2 = (-((locals.var_vt * (((4.0 * locals.var_vdss_sqrt_dn2) * locals.var_sqrt_if) + (assign1950_e1516 * locals.var_sqrt_if_dn2))) / (assign1950_e1518 * assign1950_e1518)));
        locals.var_tmp1_dn3 = (-((locals.var_vt * (((4.0 * locals.var_vdss_sqrt_dn3) * locals.var_sqrt_if) + (assign1950_e1516 * locals.var_sqrt_if_dn3))) / (assign1950_e1518 * assign1950_e1518)));
        locals.var_tmp1_rv = 0.0;

        let assign1960_e1522: f64 = (locals.var_tmp1 * locals.var_dif_dvd);
        locals.var_dvdss_dvd = assign1960_e1522;
        locals.var_dvdss_dvd_dn0 = ((locals.var_tmp1_dn0 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn0));
        locals.var_dvdss_dvd_dn1 = ((locals.var_tmp1_dn1 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn1));
        locals.var_dvdss_dvd_dn2 = ((locals.var_tmp1_dn2 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn2));
        locals.var_dvdss_dvd_dn3 = ((locals.var_tmp1_dn3 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn3));
        locals.var_dvdss_dvd_rv = 0.0;

        let assign1970_e1525: f64 = (locals.var_tmp1 * locals.var_dif_dvs);
        locals.var_dvdss_dvs = assign1970_e1525;
        locals.var_dvdss_dvs_dn0 = ((locals.var_tmp1_dn0 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn0));
        locals.var_dvdss_dvs_dn1 = ((locals.var_tmp1_dn1 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn1));
        locals.var_dvdss_dvs_dn2 = ((locals.var_tmp1_dn2 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn2));
        locals.var_dvdss_dvs_dn3 = ((locals.var_tmp1_dn3 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn3));
        locals.var_dvdss_dvs_rv = 0.0;

        let assign1990_e1531: f64 = (locals.var_vt_4 + locals.var_vt_4);
        let assign1990_e1533: f64 = (assign1990_e1531 * p.p25);
        locals.var_tmp1 = assign1990_e1533;
        locals.var_tmp1_dn0 = 0.0;
        locals.var_tmp1_dn1 = 0.0;
        locals.var_tmp1_dn2 = 0.0;
        locals.var_tmp1_dn3 = 0.0;
        locals.var_tmp1_rv = 0.0;

        let assign2000_e1537: f64 = (locals.var_sqrt_if + locals.var_sqrt_if);
        let assign2000_e1538: f64 = (locals.var_vt / assign2000_e1537);
        locals.var_tmp2 = assign2000_e1538;
        locals.var_tmp2_dn0 = (-((locals.var_vt * (locals.var_sqrt_if_dn0 + locals.var_sqrt_if_dn0)) / (assign2000_e1537 * assign2000_e1537)));
        locals.var_tmp2_dn1 = (-((locals.var_vt * (locals.var_sqrt_if_dn1 + locals.var_sqrt_if_dn1)) / (assign2000_e1537 * assign2000_e1537)));
        locals.var_tmp2_dn2 = (-((locals.var_vt * (locals.var_sqrt_if_dn2 + locals.var_sqrt_if_dn2)) / (assign2000_e1537 * assign2000_e1537)));
        locals.var_tmp2_dn3 = (-((locals.var_vt * (locals.var_sqrt_if_dn3 + locals.var_sqrt_if_dn3)) / (assign2000_e1537 * assign2000_e1537)));
        locals.var_tmp2_rv = 0.0;

        let assign2010_e1542: f64 = (locals.var_dif_dvd * locals.var_tmp2);
        let assign2010_e1544: f64 = (assign2010_e1542 - locals.var_dvdss_dvd);
        let assign2010_e1545: f64 = (locals.var_tmp1 * assign2010_e1544);
        locals.var_ddeltav_dvd = assign2010_e1545;
        locals.var_ddeltav_dvd_dn0 = ((locals.var_tmp1_dn0 * assign2010_e1544) + (locals.var_tmp1 * (((locals.var_dif_dvd_dn0 * locals.var_tmp2) + (locals.var_dif_dvd * locals.var_tmp2_dn0)) - locals.var_dvdss_dvd_dn0)));
        locals.var_ddeltav_dvd_dn1 = ((locals.var_tmp1_dn1 * assign2010_e1544) + (locals.var_tmp1 * (((locals.var_dif_dvd_dn1 * locals.var_tmp2) + (locals.var_dif_dvd * locals.var_tmp2_dn1)) - locals.var_dvdss_dvd_dn1)));
        locals.var_ddeltav_dvd_dn2 = ((locals.var_tmp1_dn2 * assign2010_e1544) + (locals.var_tmp1 * (((locals.var_dif_dvd_dn2 * locals.var_tmp2) + (locals.var_dif_dvd * locals.var_tmp2_dn2)) - locals.var_dvdss_dvd_dn2)));
        locals.var_ddeltav_dvd_dn3 = ((locals.var_tmp1_dn3 * assign2010_e1544) + (locals.var_tmp1 * (((locals.var_dif_dvd_dn3 * locals.var_tmp2) + (locals.var_dif_dvd * locals.var_tmp2_dn3)) - locals.var_dvdss_dvd_dn3)));
        locals.var_ddeltav_dvd_rv = 0.0;

        let assign2020_e1549: f64 = (locals.var_dif_dvs * locals.var_tmp2);
        let assign2020_e1551: f64 = (assign2020_e1549 - locals.var_dvdss_dvs);
        let assign2020_e1552: f64 = (locals.var_tmp1 * assign2020_e1551);
        locals.var_ddeltav_dvs = assign2020_e1552;
        locals.var_ddeltav_dvs_dn0 = ((locals.var_tmp1_dn0 * assign2020_e1551) + (locals.var_tmp1 * (((locals.var_dif_dvs_dn0 * locals.var_tmp2) + (locals.var_dif_dvs * locals.var_tmp2_dn0)) - locals.var_dvdss_dvs_dn0)));
        locals.var_ddeltav_dvs_dn1 = ((locals.var_tmp1_dn1 * assign2020_e1551) + (locals.var_tmp1 * (((locals.var_dif_dvs_dn1 * locals.var_tmp2) + (locals.var_dif_dvs * locals.var_tmp2_dn1)) - locals.var_dvdss_dvs_dn1)));
        locals.var_ddeltav_dvs_dn2 = ((locals.var_tmp1_dn2 * assign2020_e1551) + (locals.var_tmp1 * (((locals.var_dif_dvs_dn2 * locals.var_tmp2) + (locals.var_dif_dvs * locals.var_tmp2_dn2)) - locals.var_dvdss_dvs_dn2)));
        locals.var_ddeltav_dvs_dn3 = ((locals.var_tmp1_dn3 * assign2020_e1551) + (locals.var_tmp1 * (((locals.var_dif_dvs_dn3 * locals.var_tmp2) + (locals.var_dif_dvs * locals.var_tmp2_dn3)) - locals.var_dvdss_dvs_dn3)));
        locals.var_ddeltav_dvs_rv = 0.0;

        let assign2040_e1562: f64 = (1.0 / locals.var_sqrt_vdss_deltav);
        locals.var_tmp1 = assign2040_e1562;
        locals.var_tmp1_dn0 = (-(locals.var_sqrt_vdss_deltav_dn0 / (locals.var_sqrt_vdss_deltav * locals.var_sqrt_vdss_deltav)));
        locals.var_tmp1_dn1 = (-(locals.var_sqrt_vdss_deltav_dn1 / (locals.var_sqrt_vdss_deltav * locals.var_sqrt_vdss_deltav)));
        locals.var_tmp1_dn2 = (-(locals.var_sqrt_vdss_deltav_dn2 / (locals.var_sqrt_vdss_deltav * locals.var_sqrt_vdss_deltav)));
        locals.var_tmp1_dn3 = (-(locals.var_sqrt_vdss_deltav_dn3 / (locals.var_sqrt_vdss_deltav * locals.var_sqrt_vdss_deltav)));
        locals.var_tmp1_rv = 0.0;

        let assign2050_e1565: f64 = (1.0 / locals.var_sqrt_vds_vdss_deltav);
        locals.var_tmp2 = assign2050_e1565;
        locals.var_tmp2_dn0 = (-(locals.var_sqrt_vds_vdss_deltav_dn0 / (locals.var_sqrt_vds_vdss_deltav * locals.var_sqrt_vds_vdss_deltav)));
        locals.var_tmp2_dn1 = (-(locals.var_sqrt_vds_vdss_deltav_dn1 / (locals.var_sqrt_vds_vdss_deltav * locals.var_sqrt_vds_vdss_deltav)));
        locals.var_tmp2_dn2 = (-(locals.var_sqrt_vds_vdss_deltav_dn2 / (locals.var_sqrt_vds_vdss_deltav * locals.var_sqrt_vds_vdss_deltav)));
        locals.var_tmp2_dn3 = (-(locals.var_sqrt_vds_vdss_deltav_dn3 / (locals.var_sqrt_vds_vdss_deltav * locals.var_sqrt_vds_vdss_deltav)));
        locals.var_tmp2_rv = 0.0;

        let assign2060_e1568: f64 = (locals.var_vds - locals.var_vdss);
        locals.var_tmp3 = assign2060_e1568;
        locals.var_tmp3_dn0 = (locals.var_vds_dn0 - locals.var_vdss_dn0);
        locals.var_tmp3_dn1 = (-locals.var_vdss_dn1);
        locals.var_tmp3_dn2 = (locals.var_vds_dn2 - locals.var_vdss_dn2);
        locals.var_tmp3_dn3 = (locals.var_vds_dn3 - locals.var_vdss_dn3);
        locals.var_tmp3_rv = 0.0;

        let assign2070_e1571: f64 = (locals.var_vdss * locals.var_dvdss_dvd);
        let assign2070_e1573: f64 = (assign2070_e1571 + locals.var_ddeltav_dvd);
        let assign2070_e1575: f64 = (assign2070_e1573 * locals.var_tmp1);
        let assign2070_e1579: f64 = (0.5 - locals.var_dvdss_dvd);
        let assign2070_e1580: f64 = (locals.var_tmp3 * assign2070_e1579);
        let assign2070_e1582: f64 = (assign2070_e1580 + locals.var_ddeltav_dvd);
        let assign2070_e1584: f64 = (assign2070_e1582 * locals.var_tmp2);
        let assign2070_e1585: f64 = (assign2070_e1575 - assign2070_e1584);
        locals.var_dvip_dvd = assign2070_e1585;
        locals.var_dvip_dvd_dn0 = ((((((locals.var_vdss_dn0 * locals.var_dvdss_dvd) + (locals.var_vdss * locals.var_dvdss_dvd_dn0)) + locals.var_ddeltav_dvd_dn0) * locals.var_tmp1) + (assign2070_e1573 * locals.var_tmp1_dn0)) - (((((locals.var_tmp3_dn0 * assign2070_e1579) + (locals.var_tmp3 * (-locals.var_dvdss_dvd_dn0))) + locals.var_ddeltav_dvd_dn0) * locals.var_tmp2) + (assign2070_e1582 * locals.var_tmp2_dn0)));
        locals.var_dvip_dvd_dn1 = ((((((locals.var_vdss_dn1 * locals.var_dvdss_dvd) + (locals.var_vdss * locals.var_dvdss_dvd_dn1)) + locals.var_ddeltav_dvd_dn1) * locals.var_tmp1) + (assign2070_e1573 * locals.var_tmp1_dn1)) - (((((locals.var_tmp3_dn1 * assign2070_e1579) + (locals.var_tmp3 * (-locals.var_dvdss_dvd_dn1))) + locals.var_ddeltav_dvd_dn1) * locals.var_tmp2) + (assign2070_e1582 * locals.var_tmp2_dn1)));
        locals.var_dvip_dvd_dn2 = ((((((locals.var_vdss_dn2 * locals.var_dvdss_dvd) + (locals.var_vdss * locals.var_dvdss_dvd_dn2)) + locals.var_ddeltav_dvd_dn2) * locals.var_tmp1) + (assign2070_e1573 * locals.var_tmp1_dn2)) - (((((locals.var_tmp3_dn2 * assign2070_e1579) + (locals.var_tmp3 * (-locals.var_dvdss_dvd_dn2))) + locals.var_ddeltav_dvd_dn2) * locals.var_tmp2) + (assign2070_e1582 * locals.var_tmp2_dn2)));
        locals.var_dvip_dvd_dn3 = ((((((locals.var_vdss_dn3 * locals.var_dvdss_dvd) + (locals.var_vdss * locals.var_dvdss_dvd_dn3)) + locals.var_ddeltav_dvd_dn3) * locals.var_tmp1) + (assign2070_e1573 * locals.var_tmp1_dn3)) - (((((locals.var_tmp3_dn3 * assign2070_e1579) + (locals.var_tmp3 * (-locals.var_dvdss_dvd_dn3))) + locals.var_ddeltav_dvd_dn3) * locals.var_tmp2) + (assign2070_e1582 * locals.var_tmp2_dn3)));
        locals.var_dvip_dvd_rv = 0.0;

        let assign2080_e1588: f64 = (locals.var_vdss * locals.var_dvdss_dvs);
        let assign2080_e1590: f64 = (assign2080_e1588 + locals.var_ddeltav_dvs);
        let assign2080_e1592: f64 = (assign2080_e1590 * locals.var_tmp1);
        let assign2080_e1595: f64 = (-0.5);
        let assign2080_e1597: f64 = (assign2080_e1595 - locals.var_dvdss_dvs);
        let assign2080_e1598: f64 = (locals.var_tmp3 * assign2080_e1597);
        let assign2080_e1600: f64 = (assign2080_e1598 + locals.var_ddeltav_dvs);
        let assign2080_e1602: f64 = (assign2080_e1600 * locals.var_tmp2);
        let assign2080_e1603: f64 = (assign2080_e1592 - assign2080_e1602);
        locals.var_dvip_dvs = assign2080_e1603;
        locals.var_dvip_dvs_dn0 = ((((((locals.var_vdss_dn0 * locals.var_dvdss_dvs) + (locals.var_vdss * locals.var_dvdss_dvs_dn0)) + locals.var_ddeltav_dvs_dn0) * locals.var_tmp1) + (assign2080_e1590 * locals.var_tmp1_dn0)) - (((((locals.var_tmp3_dn0 * assign2080_e1597) + (locals.var_tmp3 * (-locals.var_dvdss_dvs_dn0))) + locals.var_ddeltav_dvs_dn0) * locals.var_tmp2) + (assign2080_e1600 * locals.var_tmp2_dn0)));
        locals.var_dvip_dvs_dn1 = ((((((locals.var_vdss_dn1 * locals.var_dvdss_dvs) + (locals.var_vdss * locals.var_dvdss_dvs_dn1)) + locals.var_ddeltav_dvs_dn1) * locals.var_tmp1) + (assign2080_e1590 * locals.var_tmp1_dn1)) - (((((locals.var_tmp3_dn1 * assign2080_e1597) + (locals.var_tmp3 * (-locals.var_dvdss_dvs_dn1))) + locals.var_ddeltav_dvs_dn1) * locals.var_tmp2) + (assign2080_e1600 * locals.var_tmp2_dn1)));
        locals.var_dvip_dvs_dn2 = ((((((locals.var_vdss_dn2 * locals.var_dvdss_dvs) + (locals.var_vdss * locals.var_dvdss_dvs_dn2)) + locals.var_ddeltav_dvs_dn2) * locals.var_tmp1) + (assign2080_e1590 * locals.var_tmp1_dn2)) - (((((locals.var_tmp3_dn2 * assign2080_e1597) + (locals.var_tmp3 * (-locals.var_dvdss_dvs_dn2))) + locals.var_ddeltav_dvs_dn2) * locals.var_tmp2) + (assign2080_e1600 * locals.var_tmp2_dn2)));
        locals.var_dvip_dvs_dn3 = ((((((locals.var_vdss_dn3 * locals.var_dvdss_dvs) + (locals.var_vdss * locals.var_dvdss_dvs_dn3)) + locals.var_ddeltav_dvs_dn3) * locals.var_tmp1) + (assign2080_e1590 * locals.var_tmp1_dn3)) - (((((locals.var_tmp3_dn3 * assign2080_e1597) + (locals.var_tmp3 * (-locals.var_dvdss_dvs_dn3))) + locals.var_ddeltav_dvs_dn3) * locals.var_tmp2) + (assign2080_e1600 * locals.var_tmp2_dn3)));
        locals.var_dvip_dvs_rv = 0.0;

        let assign2100_e1623: f64 = (locals.var_sqrt_if - 1.5);
        let assign2100_e1624: f64 = (locals.var_vt * assign2100_e1623);
        let assign2100_e1627: f64 = (4.0 * locals.var_vdssprime_sqrt);
        let assign2100_e1629: f64 = (assign2100_e1627 * locals.var_if_);
        let assign2100_e1630: f64 = (assign2100_e1624 / assign2100_e1629);
        locals.var_tmp1 = assign2100_e1630;
        locals.var_tmp1_dn0 = ((((locals.var_vt * locals.var_sqrt_if_dn0) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * locals.var_vdssprime_sqrt_dn0) * locals.var_if_) + (assign2100_e1627 * locals.var_if__dn0)))) / (assign2100_e1629 * assign2100_e1629));
        locals.var_tmp1_dn1 = ((((locals.var_vt * locals.var_sqrt_if_dn1) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * locals.var_vdssprime_sqrt_dn1) * locals.var_if_) + (assign2100_e1627 * locals.var_if__dn1)))) / (assign2100_e1629 * assign2100_e1629));
        locals.var_tmp1_dn2 = ((((locals.var_vt * locals.var_sqrt_if_dn2) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * locals.var_vdssprime_sqrt_dn2) * locals.var_if_) + (assign2100_e1627 * locals.var_if__dn2)))) / (assign2100_e1629 * assign2100_e1629));
        locals.var_tmp1_dn3 = ((((locals.var_vt * locals.var_sqrt_if_dn3) * assign2100_e1629) - (assign2100_e1624 * (((4.0 * locals.var_vdssprime_sqrt_dn3) * locals.var_if_) + (assign2100_e1627 * locals.var_if__dn3)))) / (assign2100_e1629 * assign2100_e1629));
        locals.var_tmp1_rv = 0.0;

        let assign2110_e1633: f64 = (locals.var_tmp1 * locals.var_dif_dvd);
        locals.var_dvdssprime_dvd = assign2110_e1633;
        locals.var_dvdssprime_dvd_dn0 = ((locals.var_tmp1_dn0 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn0));
        locals.var_dvdssprime_dvd_dn1 = ((locals.var_tmp1_dn1 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn1));
        locals.var_dvdssprime_dvd_dn2 = ((locals.var_tmp1_dn2 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn2));
        locals.var_dvdssprime_dvd_dn3 = ((locals.var_tmp1_dn3 * locals.var_dif_dvd) + (locals.var_tmp1 * locals.var_dif_dvd_dn3));
        locals.var_dvdssprime_dvd_rv = 0.0;

        let assign2120_e1636: f64 = (locals.var_tmp1 * locals.var_dif_dvs);
        locals.var_dvdssprime_dvs = assign2120_e1636;
        locals.var_dvdssprime_dvs_dn0 = ((locals.var_tmp1_dn0 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn0));
        locals.var_dvdssprime_dvs_dn1 = ((locals.var_tmp1_dn1 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn1));
        locals.var_dvdssprime_dvs_dn2 = ((locals.var_tmp1_dn2 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn2));
        locals.var_dvdssprime_dvs_dn3 = ((locals.var_tmp1_dn3 * locals.var_dif_dvs) + (locals.var_tmp1 * locals.var_dif_dvs_dn3));
        locals.var_dvdssprime_dvs_rv = 0.0;

        let assign2140_e1642: f64 = (locals.var_dirprime_dv * locals.var_inv_vt);
        locals.var_tmp1 = assign2140_e1642;
        locals.var_tmp1_dn0 = (locals.var_dirprime_dv_dn0 * locals.var_inv_vt);
        locals.var_tmp1_dn1 = (locals.var_dirprime_dv_dn1 * locals.var_inv_vt);
        locals.var_tmp1_dn2 = (locals.var_dirprime_dv_dn2 * locals.var_inv_vt);
        locals.var_tmp1_dn3 = (locals.var_dirprime_dv_dn3 * locals.var_inv_vt);
        locals.var_tmp1_rv = 0.0;

        let assign2150_e1645: f64 = (1.0 / locals.var_sqrt_vdssprime_deltav);
        locals.var_tmp2 = assign2150_e1645;
        locals.var_tmp2_dn0 = (-(locals.var_sqrt_vdssprime_deltav_dn0 / (locals.var_sqrt_vdssprime_deltav * locals.var_sqrt_vdssprime_deltav)));
        locals.var_tmp2_dn1 = (-(locals.var_sqrt_vdssprime_deltav_dn1 / (locals.var_sqrt_vdssprime_deltav * locals.var_sqrt_vdssprime_deltav)));
        locals.var_tmp2_dn2 = (-(locals.var_sqrt_vdssprime_deltav_dn2 / (locals.var_sqrt_vdssprime_deltav * locals.var_sqrt_vdssprime_deltav)));
        locals.var_tmp2_dn3 = (-(locals.var_sqrt_vdssprime_deltav_dn3 / (locals.var_sqrt_vdssprime_deltav * locals.var_sqrt_vdssprime_deltav)));
        locals.var_tmp2_rv = 0.0;

        let assign2160_e1648: f64 = (1.0 / locals.var_sqrt_vds_vdssprime_deltav);
        locals.var_tmp3 = assign2160_e1648;
        locals.var_tmp3_dn0 = (-(locals.var_sqrt_vds_vdssprime_deltav_dn0 / (locals.var_sqrt_vds_vdssprime_deltav * locals.var_sqrt_vds_vdssprime_deltav)));
        locals.var_tmp3_dn1 = (-(locals.var_sqrt_vds_vdssprime_deltav_dn1 / (locals.var_sqrt_vds_vdssprime_deltav * locals.var_sqrt_vds_vdssprime_deltav)));
        locals.var_tmp3_dn2 = (-(locals.var_sqrt_vds_vdssprime_deltav_dn2 / (locals.var_sqrt_vds_vdssprime_deltav * locals.var_sqrt_vds_vdssprime_deltav)));
        locals.var_tmp3_dn3 = (-(locals.var_sqrt_vds_vdssprime_deltav_dn3 / (locals.var_sqrt_vds_vdssprime_deltav * locals.var_sqrt_vds_vdssprime_deltav)));
        locals.var_tmp3_rv = 0.0;

        let assign2170_e1652: f64 = (locals.var_dvp_dvd - 0.5);
        let assign2170_e1655: f64 = (locals.var_vdssprime * locals.var_dvdssprime_dvd);
        let assign2170_e1657: f64 = (assign2170_e1655 + locals.var_ddeltav_dvd);
        let assign2170_e1659: f64 = (assign2170_e1657 * locals.var_tmp2);
        let assign2170_e1660: f64 = (assign2170_e1652 - assign2170_e1659);
        let assign2170_e1664: f64 = (0.5 - locals.var_dvdssprime_dvd);
        let assign2170_e1665: f64 = (locals.var_vdsprime * assign2170_e1664);
        let assign2170_e1667: f64 = (assign2170_e1665 + locals.var_ddeltav_dvd);
        let assign2170_e1669: f64 = (assign2170_e1667 * locals.var_tmp3);
        let assign2170_e1670: f64 = (assign2170_e1660 + assign2170_e1669);
        let assign2170_e1671: f64 = (locals.var_tmp1 * assign2170_e1670);
        locals.var_dirprime_dvd = assign2170_e1671;
        locals.var_dirprime_dvd_dn0 = ((locals.var_tmp1_dn0 * assign2170_e1670) + (locals.var_tmp1 * ((locals.var_dvp_dvd_dn0 - (((((locals.var_vdssprime_dn0 * locals.var_dvdssprime_dvd) + (locals.var_vdssprime * locals.var_dvdssprime_dvd_dn0)) + locals.var_ddeltav_dvd_dn0) * locals.var_tmp2) + (assign2170_e1657 * locals.var_tmp2_dn0))) + (((((locals.var_vdsprime_dn0 * assign2170_e1664) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvd_dn0))) + locals.var_ddeltav_dvd_dn0) * locals.var_tmp3) + (assign2170_e1667 * locals.var_tmp3_dn0)))));
        locals.var_dirprime_dvd_dn1 = ((locals.var_tmp1_dn1 * assign2170_e1670) + (locals.var_tmp1 * ((locals.var_dvp_dvd_dn1 - (((((locals.var_vdssprime_dn1 * locals.var_dvdssprime_dvd) + (locals.var_vdssprime * locals.var_dvdssprime_dvd_dn1)) + locals.var_ddeltav_dvd_dn1) * locals.var_tmp2) + (assign2170_e1657 * locals.var_tmp2_dn1))) + (((((locals.var_vdsprime_dn1 * assign2170_e1664) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvd_dn1))) + locals.var_ddeltav_dvd_dn1) * locals.var_tmp3) + (assign2170_e1667 * locals.var_tmp3_dn1)))));
        locals.var_dirprime_dvd_dn2 = ((locals.var_tmp1_dn2 * assign2170_e1670) + (locals.var_tmp1 * ((locals.var_dvp_dvd_dn2 - (((((locals.var_vdssprime_dn2 * locals.var_dvdssprime_dvd) + (locals.var_vdssprime * locals.var_dvdssprime_dvd_dn2)) + locals.var_ddeltav_dvd_dn2) * locals.var_tmp2) + (assign2170_e1657 * locals.var_tmp2_dn2))) + (((((locals.var_vdsprime_dn2 * assign2170_e1664) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvd_dn2))) + locals.var_ddeltav_dvd_dn2) * locals.var_tmp3) + (assign2170_e1667 * locals.var_tmp3_dn2)))));
        locals.var_dirprime_dvd_dn3 = ((locals.var_tmp1_dn3 * assign2170_e1670) + (locals.var_tmp1 * ((locals.var_dvp_dvd_dn3 - (((((locals.var_vdssprime_dn3 * locals.var_dvdssprime_dvd) + (locals.var_vdssprime * locals.var_dvdssprime_dvd_dn3)) + locals.var_ddeltav_dvd_dn3) * locals.var_tmp2) + (assign2170_e1657 * locals.var_tmp2_dn3))) + (((((locals.var_vdsprime_dn3 * assign2170_e1664) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvd_dn3))) + locals.var_ddeltav_dvd_dn3) * locals.var_tmp3) + (assign2170_e1667 * locals.var_tmp3_dn3)))));
        locals.var_dirprime_dvd_rv = 0.0;

        let assign2180_e1675: f64 = (locals.var_dvp_dvs - 0.5);
        let assign2180_e1678: f64 = (locals.var_vdssprime * locals.var_dvdssprime_dvs);
        let assign2180_e1680: f64 = (assign2180_e1678 + locals.var_ddeltav_dvs);
        let assign2180_e1682: f64 = (assign2180_e1680 * locals.var_tmp2);
        let assign2180_e1683: f64 = (assign2180_e1675 - assign2180_e1682);
        let assign2180_e1686: f64 = (-0.5);
        let assign2180_e1688: f64 = (assign2180_e1686 - locals.var_dvdssprime_dvs);
        let assign2180_e1689: f64 = (locals.var_vdsprime * assign2180_e1688);
        let assign2180_e1691: f64 = (assign2180_e1689 + locals.var_ddeltav_dvs);
        let assign2180_e1693: f64 = (assign2180_e1691 * locals.var_tmp3);
        let assign2180_e1694: f64 = (assign2180_e1683 + assign2180_e1693);
        let assign2180_e1695: f64 = (locals.var_tmp1 * assign2180_e1694);
        locals.var_dirprime_dvs = assign2180_e1695;
        locals.var_dirprime_dvs_dn0 = ((locals.var_tmp1_dn0 * assign2180_e1694) + (locals.var_tmp1 * ((locals.var_dvp_dvs_dn0 - (((((locals.var_vdssprime_dn0 * locals.var_dvdssprime_dvs) + (locals.var_vdssprime * locals.var_dvdssprime_dvs_dn0)) + locals.var_ddeltav_dvs_dn0) * locals.var_tmp2) + (assign2180_e1680 * locals.var_tmp2_dn0))) + (((((locals.var_vdsprime_dn0 * assign2180_e1688) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvs_dn0))) + locals.var_ddeltav_dvs_dn0) * locals.var_tmp3) + (assign2180_e1691 * locals.var_tmp3_dn0)))));
        locals.var_dirprime_dvs_dn1 = ((locals.var_tmp1_dn1 * assign2180_e1694) + (locals.var_tmp1 * ((locals.var_dvp_dvs_dn1 - (((((locals.var_vdssprime_dn1 * locals.var_dvdssprime_dvs) + (locals.var_vdssprime * locals.var_dvdssprime_dvs_dn1)) + locals.var_ddeltav_dvs_dn1) * locals.var_tmp2) + (assign2180_e1680 * locals.var_tmp2_dn1))) + (((((locals.var_vdsprime_dn1 * assign2180_e1688) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvs_dn1))) + locals.var_ddeltav_dvs_dn1) * locals.var_tmp3) + (assign2180_e1691 * locals.var_tmp3_dn1)))));
        locals.var_dirprime_dvs_dn2 = ((locals.var_tmp1_dn2 * assign2180_e1694) + (locals.var_tmp1 * ((locals.var_dvp_dvs_dn2 - (((((locals.var_vdssprime_dn2 * locals.var_dvdssprime_dvs) + (locals.var_vdssprime * locals.var_dvdssprime_dvs_dn2)) + locals.var_ddeltav_dvs_dn2) * locals.var_tmp2) + (assign2180_e1680 * locals.var_tmp2_dn2))) + (((((locals.var_vdsprime_dn2 * assign2180_e1688) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvs_dn2))) + locals.var_ddeltav_dvs_dn2) * locals.var_tmp3) + (assign2180_e1691 * locals.var_tmp3_dn2)))));
        locals.var_dirprime_dvs_dn3 = ((locals.var_tmp1_dn3 * assign2180_e1694) + (locals.var_tmp1 * ((locals.var_dvp_dvs_dn3 - (((((locals.var_vdssprime_dn3 * locals.var_dvdssprime_dvs) + (locals.var_vdssprime * locals.var_dvdssprime_dvs_dn3)) + locals.var_ddeltav_dvs_dn3) * locals.var_tmp2) + (assign2180_e1680 * locals.var_tmp2_dn3))) + (((((locals.var_vdsprime_dn3 * assign2180_e1688) + (locals.var_vdsprime * (-locals.var_dvdssprime_dvs_dn3))) + locals.var_ddeltav_dvs_dn3) * locals.var_tmp3) + (assign2180_e1691 * locals.var_tmp3_dn3)))));
        locals.var_dirprime_dvs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2200_e1719: f64 = (locals.var_lc_ucrit + locals.var_vds);
        let assign2200_e1721: f64 = (assign2200_e1719 - locals.var_vip);
        let assign2200_e1722: f64 = (locals.var_lc_lambda / assign2200_e1721);
        locals.var_tmp1 = assign2200_e1722;
        locals.var_tmp1_dn0 = (-((locals.var_lc_lambda * (locals.var_vds_dn0 - locals.var_vip_dn0)) / (assign2200_e1721 * assign2200_e1721)));
        locals.var_tmp1_dn1 = (-((locals.var_lc_lambda * (-locals.var_vip_dn1)) / (assign2200_e1721 * assign2200_e1721)));
        locals.var_tmp1_dn2 = (-((locals.var_lc_lambda * (locals.var_vds_dn2 - locals.var_vip_dn2)) / (assign2200_e1721 * assign2200_e1721)));
        locals.var_tmp1_dn3 = (-((locals.var_lc_lambda * (locals.var_vds_dn3 - locals.var_vip_dn3)) / (assign2200_e1721 * assign2200_e1721)));
        locals.var_tmp1_rv = 0.0;

        let assign2210_e1726: f64 = (0.5 - locals.var_dvip_dvd);
        let assign2210_e1727: f64 = (locals.var_tmp1 * assign2210_e1726);
        locals.var_ddeltal_dvd = assign2210_e1727;
        locals.var_ddeltal_dvd_dn0 = ((locals.var_tmp1_dn0 * assign2210_e1726) + (locals.var_tmp1 * (-locals.var_dvip_dvd_dn0)));
        locals.var_ddeltal_dvd_dn1 = ((locals.var_tmp1_dn1 * assign2210_e1726) + (locals.var_tmp1 * (-locals.var_dvip_dvd_dn1)));
        locals.var_ddeltal_dvd_dn2 = ((locals.var_tmp1_dn2 * assign2210_e1726) + (locals.var_tmp1 * (-locals.var_dvip_dvd_dn2)));
        locals.var_ddeltal_dvd_dn3 = ((locals.var_tmp1_dn3 * assign2210_e1726) + (locals.var_tmp1 * (-locals.var_dvip_dvd_dn3)));
        locals.var_ddeltal_dvd_rv = 0.0;

        let assign2220_e1730: f64 = (-0.5);
        let assign2220_e1732: f64 = (assign2220_e1730 - locals.var_dvip_dvs);
        let assign2220_e1733: f64 = (locals.var_tmp1 * assign2220_e1732);
        locals.var_ddeltal_dvs = assign2220_e1733;
        locals.var_ddeltal_dvs_dn0 = ((locals.var_tmp1_dn0 * assign2220_e1732) + (locals.var_tmp1 * (-locals.var_dvip_dvs_dn0)));
        locals.var_ddeltal_dvs_dn1 = ((locals.var_tmp1_dn1 * assign2220_e1732) + (locals.var_tmp1 * (-locals.var_dvip_dvs_dn1)));
        locals.var_ddeltal_dvs_dn2 = ((locals.var_tmp1_dn2 * assign2220_e1732) + (locals.var_tmp1 * (-locals.var_dvip_dvs_dn2)));
        locals.var_ddeltal_dvs_dn3 = ((locals.var_tmp1_dn3 * assign2220_e1732) + (locals.var_tmp1 * (-locals.var_dvip_dvs_dn3)));
        locals.var_ddeltal_dvs_rv = 0.0;

        let assign2240_e1740: f64 = (1.0 / locals.var_sqrt_lprime_lmin);
        locals.var_tmp1 = assign2240_e1740;
        locals.var_tmp1_dn0 = (-(locals.var_sqrt_lprime_lmin_dn0 / (locals.var_sqrt_lprime_lmin * locals.var_sqrt_lprime_lmin)));
        locals.var_tmp1_dn1 = (-(locals.var_sqrt_lprime_lmin_dn1 / (locals.var_sqrt_lprime_lmin * locals.var_sqrt_lprime_lmin)));
        locals.var_tmp1_dn2 = (-(locals.var_sqrt_lprime_lmin_dn2 / (locals.var_sqrt_lprime_lmin * locals.var_sqrt_lprime_lmin)));
        locals.var_tmp1_dn3 = (-(locals.var_sqrt_lprime_lmin_dn3 / (locals.var_sqrt_lprime_lmin * locals.var_sqrt_lprime_lmin)));
        locals.var_tmp1_rv = 0.0;

        let assign2250_e1743: f64 = (-locals.var_ddeltal_dvd);
        let assign2250_e1746: f64 = (0.5 + locals.var_dvip_dvd);
        let assign2250_e1748: f64 = (assign2250_e1746 * locals.var_inv_ucrit);
        let assign2250_e1749: f64 = (assign2250_e1743 + assign2250_e1748);
        let assign2250_e1750: f64 = (locals.var_tmp1 * assign2250_e1749);
        locals.var_dleq_dvd = assign2250_e1750;
        locals.var_dleq_dvd_dn0 = ((locals.var_tmp1_dn0 * assign2250_e1749) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvd_dn0) + (locals.var_dvip_dvd_dn0 * locals.var_inv_ucrit))));
        locals.var_dleq_dvd_dn1 = ((locals.var_tmp1_dn1 * assign2250_e1749) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvd_dn1) + (locals.var_dvip_dvd_dn1 * locals.var_inv_ucrit))));
        locals.var_dleq_dvd_dn2 = ((locals.var_tmp1_dn2 * assign2250_e1749) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvd_dn2) + (locals.var_dvip_dvd_dn2 * locals.var_inv_ucrit))));
        locals.var_dleq_dvd_dn3 = ((locals.var_tmp1_dn3 * assign2250_e1749) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvd_dn3) + (locals.var_dvip_dvd_dn3 * locals.var_inv_ucrit))));
        locals.var_dleq_dvd_rv = 0.0;

        let assign2260_e1753: f64 = (-locals.var_ddeltal_dvs);
        let assign2260_e1755: f64 = (-0.5);
        let assign2260_e1757: f64 = (assign2260_e1755 + locals.var_dvip_dvs);
        let assign2260_e1759: f64 = (assign2260_e1757 * locals.var_inv_ucrit);
        let assign2260_e1760: f64 = (assign2260_e1753 + assign2260_e1759);
        let assign2260_e1761: f64 = (locals.var_tmp1 * assign2260_e1760);
        locals.var_dleq_dvs = assign2260_e1761;
        locals.var_dleq_dvs_dn0 = ((locals.var_tmp1_dn0 * assign2260_e1760) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvs_dn0) + (locals.var_dvip_dvs_dn0 * locals.var_inv_ucrit))));
        locals.var_dleq_dvs_dn1 = ((locals.var_tmp1_dn1 * assign2260_e1760) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvs_dn1) + (locals.var_dvip_dvs_dn1 * locals.var_inv_ucrit))));
        locals.var_dleq_dvs_dn2 = ((locals.var_tmp1_dn2 * assign2260_e1760) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvs_dn2) + (locals.var_dvip_dvs_dn2 * locals.var_inv_ucrit))));
        locals.var_dleq_dvs_dn3 = ((locals.var_tmp1_dn3 * assign2260_e1760) + (locals.var_tmp1 * ((-locals.var_ddeltal_dvs_dn3) + (locals.var_dvip_dvs_dn3 * locals.var_inv_ucrit))));
        locals.var_dleq_dvs_rv = 0.0;

        let assign2280_e1772: f64 = (locals.var_dir_dv * locals.var_inv_vt);
        locals.var_tmp1 = assign2280_e1772;
        locals.var_tmp1_dn0 = (locals.var_dir_dv_dn0 * locals.var_inv_vt);
        locals.var_tmp1_dn1 = (locals.var_dir_dv_dn1 * locals.var_inv_vt);
        locals.var_tmp1_dn2 = (locals.var_dir_dv_dn2 * locals.var_inv_vt);
        locals.var_tmp1_dn3 = (locals.var_dir_dv_dn3 * locals.var_inv_vt);
        locals.var_tmp1_rv = 0.0;

        let assign2290_e1776: f64 = (locals.var_dvp_dvd - 1.0);
        let assign2290_e1777: f64 = (locals.var_tmp1 * assign2290_e1776);
        locals.var_dir_dvd = assign2290_e1777;
        locals.var_dir_dvd_dn0 = ((locals.var_tmp1_dn0 * assign2290_e1776) + (locals.var_tmp1 * locals.var_dvp_dvd_dn0));
        locals.var_dir_dvd_dn1 = ((locals.var_tmp1_dn1 * assign2290_e1776) + (locals.var_tmp1 * locals.var_dvp_dvd_dn1));
        locals.var_dir_dvd_dn2 = ((locals.var_tmp1_dn2 * assign2290_e1776) + (locals.var_tmp1 * locals.var_dvp_dvd_dn2));
        locals.var_dir_dvd_dn3 = ((locals.var_tmp1_dn3 * assign2290_e1776) + (locals.var_tmp1 * locals.var_dvp_dvd_dn3));
        locals.var_dir_dvd_rv = 0.0;

        let assign2300_e1780: f64 = (locals.var_tmp1 * locals.var_dvp_dvs);
        locals.var_dir_dvs = assign2300_e1780;
        locals.var_dir_dvs_dn0 = ((locals.var_tmp1_dn0 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn0));
        locals.var_dir_dvs_dn1 = ((locals.var_tmp1_dn1 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn1));
        locals.var_dir_dvs_dn2 = ((locals.var_tmp1_dn2 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn2));
        locals.var_dir_dvs_dn3 = ((locals.var_tmp1_dn3 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn3));
        locals.var_dir_dvs_rv = 0.0;

        let assign2320_e1786: f64 = (1.0 + locals.var_n_1);
        let assign2320_e1787: f64 = (-assign2320_e1786);
        let assign2320_e1789: f64 = (assign2320_e1787 * locals.var_vt);
        let assign2320_e1791: f64 = (assign2320_e1789 * 0.66666666);
        let assign2320_e1793: f64 = (assign2320_e1791 / locals.var_sif_sir_2);
        locals.var_tmp1 = assign2320_e1793;
        locals.var_tmp1_dn0 = ((((((-locals.var_n_1_dn0) * locals.var_vt) * 0.66666666) * locals.var_sif_sir_2) - (assign2320_e1791 * locals.var_sif_sir_2_dn0)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2));
        locals.var_tmp1_dn1 = ((((((-locals.var_n_1_dn1) * locals.var_vt) * 0.66666666) * locals.var_sif_sir_2) - (assign2320_e1791 * locals.var_sif_sir_2_dn1)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2));
        locals.var_tmp1_dn2 = ((((((-locals.var_n_1_dn2) * locals.var_vt) * 0.66666666) * locals.var_sif_sir_2) - (assign2320_e1791 * locals.var_sif_sir_2_dn2)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2));
        locals.var_tmp1_dn3 = ((((((-locals.var_n_1_dn3) * locals.var_vt) * 0.66666666) * locals.var_sif_sir_2) - (assign2320_e1791 * locals.var_sif_sir_2_dn3)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2));
        locals.var_tmp1_rv = 0.0;

        let assign2330_e1798: f64 = (2.0 * locals.var_sir);
        let assign2330_e1799: f64 = (locals.var_sif + assign2330_e1798);
        let assign2330_e1800: f64 = (locals.var_tmp1 * assign2330_e1799);
        locals.var_tmp2 = assign2330_e1800;
        locals.var_tmp2_dn0 = ((locals.var_tmp1_dn0 * assign2330_e1799) + (locals.var_tmp1 * (locals.var_sif_dn0 + (2.0 * locals.var_sir_dn0))));
        locals.var_tmp2_dn1 = ((locals.var_tmp1_dn1 * assign2330_e1799) + (locals.var_tmp1 * (locals.var_sif_dn1 + (2.0 * locals.var_sir_dn1))));
        locals.var_tmp2_dn2 = ((locals.var_tmp1_dn2 * assign2330_e1799) + (locals.var_tmp1 * (locals.var_sif_dn2 + (2.0 * locals.var_sir_dn2))));
        locals.var_tmp2_dn3 = ((locals.var_tmp1_dn3 * assign2330_e1799) + (locals.var_tmp1 * (locals.var_sif_dn3 + (2.0 * locals.var_sir_dn3))));
        locals.var_tmp2_rv = 0.0;

        let assign2340_e1805: f64 = (2.0 * locals.var_sif);
        let assign2340_e1806: f64 = (locals.var_sir + assign2340_e1805);
        let assign2340_e1807: f64 = (locals.var_tmp1 * assign2340_e1806);
        locals.var_tmp3 = assign2340_e1807;
        locals.var_tmp3_dn0 = ((locals.var_tmp1_dn0 * assign2340_e1806) + (locals.var_tmp1 * (locals.var_sir_dn0 + (2.0 * locals.var_sif_dn0))));
        locals.var_tmp3_dn1 = ((locals.var_tmp1_dn1 * assign2340_e1806) + (locals.var_tmp1 * (locals.var_sir_dn1 + (2.0 * locals.var_sif_dn1))));
        locals.var_tmp3_dn2 = ((locals.var_tmp1_dn2 * assign2340_e1806) + (locals.var_tmp1 * (locals.var_sir_dn2 + (2.0 * locals.var_sif_dn2))));
        locals.var_tmp3_dn3 = ((locals.var_tmp1_dn3 * assign2340_e1806) + (locals.var_tmp1 * (locals.var_sir_dn3 + (2.0 * locals.var_sif_dn3))));
        locals.var_tmp3_rv = 0.0;

        let assign2350_e1809: f64 = (-locals.var_n_1);
        let assign2350_e1811: f64 = (assign2350_e1809 * locals.var_qi);
        let assign2350_e1814: f64 = (2.0 + locals.var_n_1);
        let assign2350_e1816: f64 = (assign2350_e1814 + locals.var_n_1);
        let assign2350_e1818: f64 = (assign2350_e1816 * locals.var_vp_phi_eps);
        let assign2350_e1819: f64 = (assign2350_e1811 / assign2350_e1818);
        locals.var_tmp1 = assign2350_e1819;
        locals.var_tmp1_dn0 = ((((((-locals.var_n_1_dn0) * locals.var_qi) + (assign2350_e1809 * locals.var_qi_dn0)) * assign2350_e1818) - (assign2350_e1811 * (((locals.var_n_1_dn0 + locals.var_n_1_dn0) * locals.var_vp_phi_eps) + (assign2350_e1816 * locals.var_vp_phi_eps_dn0)))) / (assign2350_e1818 * assign2350_e1818));
        locals.var_tmp1_dn1 = ((((((-locals.var_n_1_dn1) * locals.var_qi) + (assign2350_e1809 * locals.var_qi_dn1)) * assign2350_e1818) - (assign2350_e1811 * (((locals.var_n_1_dn1 + locals.var_n_1_dn1) * locals.var_vp_phi_eps) + (assign2350_e1816 * locals.var_vp_phi_eps_dn1)))) / (assign2350_e1818 * assign2350_e1818));
        locals.var_tmp1_dn2 = ((((((-locals.var_n_1_dn2) * locals.var_qi) + (assign2350_e1809 * locals.var_qi_dn2)) * assign2350_e1818) - (assign2350_e1811 * (((locals.var_n_1_dn2 + locals.var_n_1_dn2) * locals.var_vp_phi_eps) + (assign2350_e1816 * locals.var_vp_phi_eps_dn2)))) / (assign2350_e1818 * assign2350_e1818));
        locals.var_tmp1_dn3 = ((((((-locals.var_n_1_dn3) * locals.var_qi) + (assign2350_e1809 * locals.var_qi_dn3)) * assign2350_e1818) - (assign2350_e1811 * (((locals.var_n_1_dn3 + locals.var_n_1_dn3) * locals.var_vp_phi_eps) + (assign2350_e1816 * locals.var_vp_phi_eps_dn3)))) / (assign2350_e1818 * assign2350_e1818));
        locals.var_tmp1_rv = 0.0;

        let assign2360_e1822: f64 = (locals.var_tmp1 * locals.var_dvp_dvd);
        let assign2360_e1825: f64 = (locals.var_tmp2 * locals.var_dif_dvd);
        let assign2360_e1826: f64 = (assign2360_e1822 + assign2360_e1825);
        let assign2360_e1829: f64 = (locals.var_tmp3 * locals.var_dir_dvd);
        let assign2360_e1830: f64 = (assign2360_e1826 + assign2360_e1829);
        locals.var_dqi_dvd = assign2360_e1830;
        locals.var_dqi_dvd_dn0 = ((((locals.var_tmp1_dn0 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn0)) + ((locals.var_tmp2_dn0 * locals.var_dif_dvd) + (locals.var_tmp2 * locals.var_dif_dvd_dn0))) + ((locals.var_tmp3_dn0 * locals.var_dir_dvd) + (locals.var_tmp3 * locals.var_dir_dvd_dn0)));
        locals.var_dqi_dvd_dn1 = ((((locals.var_tmp1_dn1 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn1)) + ((locals.var_tmp2_dn1 * locals.var_dif_dvd) + (locals.var_tmp2 * locals.var_dif_dvd_dn1))) + ((locals.var_tmp3_dn1 * locals.var_dir_dvd) + (locals.var_tmp3 * locals.var_dir_dvd_dn1)));
        locals.var_dqi_dvd_dn2 = ((((locals.var_tmp1_dn2 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn2)) + ((locals.var_tmp2_dn2 * locals.var_dif_dvd) + (locals.var_tmp2 * locals.var_dif_dvd_dn2))) + ((locals.var_tmp3_dn2 * locals.var_dir_dvd) + (locals.var_tmp3 * locals.var_dir_dvd_dn2)));
        locals.var_dqi_dvd_dn3 = ((((locals.var_tmp1_dn3 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn3)) + ((locals.var_tmp2_dn3 * locals.var_dif_dvd) + (locals.var_tmp2 * locals.var_dif_dvd_dn3))) + ((locals.var_tmp3_dn3 * locals.var_dir_dvd) + (locals.var_tmp3 * locals.var_dir_dvd_dn3)));
        locals.var_dqi_dvd_rv = 0.0;

        let assign2370_e1833: f64 = (locals.var_tmp1 * locals.var_dvp_dvs);
        let assign2370_e1836: f64 = (locals.var_tmp2 * locals.var_dif_dvs);
        let assign2370_e1837: f64 = (assign2370_e1833 + assign2370_e1836);
        let assign2370_e1840: f64 = (locals.var_tmp3 * locals.var_dir_dvs);
        let assign2370_e1841: f64 = (assign2370_e1837 + assign2370_e1840);
        locals.var_dqi_dvs = assign2370_e1841;
        locals.var_dqi_dvs_dn0 = ((((locals.var_tmp1_dn0 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn0)) + ((locals.var_tmp2_dn0 * locals.var_dif_dvs) + (locals.var_tmp2 * locals.var_dif_dvs_dn0))) + ((locals.var_tmp3_dn0 * locals.var_dir_dvs) + (locals.var_tmp3 * locals.var_dir_dvs_dn0)));
        locals.var_dqi_dvs_dn1 = ((((locals.var_tmp1_dn1 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn1)) + ((locals.var_tmp2_dn1 * locals.var_dif_dvs) + (locals.var_tmp2 * locals.var_dif_dvs_dn1))) + ((locals.var_tmp3_dn1 * locals.var_dir_dvs) + (locals.var_tmp3 * locals.var_dir_dvs_dn1)));
        locals.var_dqi_dvs_dn2 = ((((locals.var_tmp1_dn2 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn2)) + ((locals.var_tmp2_dn2 * locals.var_dif_dvs) + (locals.var_tmp2 * locals.var_dif_dvs_dn2))) + ((locals.var_tmp3_dn2 * locals.var_dir_dvs) + (locals.var_tmp3 * locals.var_dir_dvs_dn2)));
        locals.var_dqi_dvs_dn3 = ((((locals.var_tmp1_dn3 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn3)) + ((locals.var_tmp2_dn3 * locals.var_dif_dvs) + (locals.var_tmp2 * locals.var_dif_dvs_dn3))) + ((locals.var_tmp3_dn3 * locals.var_dir_dvs) + (locals.var_tmp3 * locals.var_dir_dvs_dn3)));
        locals.var_dqi_dvs_rv = 0.0;

        let assign2390_e1855: f64 = (1.0 + locals.var_n_1);
        let assign2390_e1860: f64 = (1.0 + locals.var_n_1);
        let assign2390_e1861: f64 = (2.0 * assign2390_e1860);
        let assign2390_e1863: f64 = (assign2390_e1861 * locals.var_vp_phi_eps);
        let assign2390_e1864: f64 = (locals.var_qi / assign2390_e1863);
        let assign2390_e1865: f64 = (assign2390_e1855 - assign2390_e1864);
        locals.var_tmp1 = assign2390_e1865;
        locals.var_tmp1_dn0 = (locals.var_n_1_dn0 - (((locals.var_qi_dn0 * assign2390_e1863) - (locals.var_qi * (((2.0 * locals.var_n_1_dn0) * locals.var_vp_phi_eps) + (assign2390_e1861 * locals.var_vp_phi_eps_dn0)))) / (assign2390_e1863 * assign2390_e1863)));
        locals.var_tmp1_dn1 = (locals.var_n_1_dn1 - (((locals.var_qi_dn1 * assign2390_e1863) - (locals.var_qi * (((2.0 * locals.var_n_1_dn1) * locals.var_vp_phi_eps) + (assign2390_e1861 * locals.var_vp_phi_eps_dn1)))) / (assign2390_e1863 * assign2390_e1863)));
        locals.var_tmp1_dn2 = (locals.var_n_1_dn2 - (((locals.var_qi_dn2 * assign2390_e1863) - (locals.var_qi * (((2.0 * locals.var_n_1_dn2) * locals.var_vp_phi_eps) + (assign2390_e1861 * locals.var_vp_phi_eps_dn2)))) / (assign2390_e1863 * assign2390_e1863)));
        locals.var_tmp1_dn3 = (locals.var_n_1_dn3 - (((locals.var_qi_dn3 * assign2390_e1863) - (locals.var_qi * (((2.0 * locals.var_n_1_dn3) * locals.var_vp_phi_eps) + (assign2390_e1861 * locals.var_vp_phi_eps_dn3)))) / (assign2390_e1863 * assign2390_e1863)));
        locals.var_tmp1_rv = 0.0;

        let assign2400_e1867: f64 = (-locals.var_n_1_n);
        let assign2400_e1870: f64 = (locals.var_tmp1 * locals.var_dvp_dvd);
        let assign2400_e1872: f64 = (assign2400_e1870 + locals.var_dqi_dvd);
        let assign2400_e1873: f64 = (assign2400_e1867 * assign2400_e1872);
        locals.var_dqb_dvd = assign2400_e1873;
        locals.var_dqb_dvd_dn0 = (((-locals.var_n_1_n_dn0) * assign2400_e1872) + (assign2400_e1867 * (((locals.var_tmp1_dn0 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn0)) + locals.var_dqi_dvd_dn0)));
        locals.var_dqb_dvd_dn1 = (((-locals.var_n_1_n_dn1) * assign2400_e1872) + (assign2400_e1867 * (((locals.var_tmp1_dn1 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn1)) + locals.var_dqi_dvd_dn1)));
        locals.var_dqb_dvd_dn2 = (((-locals.var_n_1_n_dn2) * assign2400_e1872) + (assign2400_e1867 * (((locals.var_tmp1_dn2 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn2)) + locals.var_dqi_dvd_dn2)));
        locals.var_dqb_dvd_dn3 = (((-locals.var_n_1_n_dn3) * assign2400_e1872) + (assign2400_e1867 * (((locals.var_tmp1_dn3 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn3)) + locals.var_dqi_dvd_dn3)));
        locals.var_dqb_dvd_rv = 0.0;

        let assign2410_e1875: f64 = (-locals.var_n_1_n);
        let assign2410_e1878: f64 = (locals.var_tmp1 * locals.var_dvp_dvs);
        let assign2410_e1880: f64 = (assign2410_e1878 + locals.var_dqi_dvs);
        let assign2410_e1881: f64 = (assign2410_e1875 * assign2410_e1880);
        locals.var_dqb_dvs = assign2410_e1881;
        locals.var_dqb_dvs_dn0 = (((-locals.var_n_1_n_dn0) * assign2410_e1880) + (assign2410_e1875 * (((locals.var_tmp1_dn0 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn0)) + locals.var_dqi_dvs_dn0)));
        locals.var_dqb_dvs_dn1 = (((-locals.var_n_1_n_dn1) * assign2410_e1880) + (assign2410_e1875 * (((locals.var_tmp1_dn1 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn1)) + locals.var_dqi_dvs_dn1)));
        locals.var_dqb_dvs_dn2 = (((-locals.var_n_1_n_dn2) * assign2410_e1880) + (assign2410_e1875 * (((locals.var_tmp1_dn2 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn2)) + locals.var_dqi_dvs_dn2)));
        locals.var_dqb_dvs_dn3 = (((-locals.var_n_1_n_dn3) * assign2410_e1880) + (assign2410_e1875 * (((locals.var_tmp1_dn3 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn3)) + locals.var_dqi_dvs_dn3)));
        locals.var_dqb_dvs_rv = 0.0;

        let assign2430_e1892: f64 = if p.p22 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard18 = assign2430_e1892;
        locals.var_guard18_rv = 0.0;

        let (assign2440_e1902, assign2440_e1902_d_n0, assign2440_e1902_d_n1, assign2440_e1902_d_n2, assign2440_e1902_d_n3,) = {
    if (locals.var_guard18 != 0.0) {
        let assign2440_e1896: f64 = (p.p21 * locals.var_vpprime);
        let assign2440_e1899: f64 = (locals.var_theta_vp_1 * locals.var_sqrt_vp_vt);
        let assign2440_e1900: f64 = (assign2440_e1896 / assign2440_e1899);
        (assign2440_e1900, ((((p.p21 * locals.var_vpprime_dn0) * assign2440_e1899) - (assign2440_e1896 * ((locals.var_theta_vp_1_dn0 * locals.var_sqrt_vp_vt) + (locals.var_theta_vp_1 * locals.var_sqrt_vp_vt_dn0)))) / (assign2440_e1899 * assign2440_e1899)), ((((p.p21 * locals.var_vpprime_dn1) * assign2440_e1899) - (assign2440_e1896 * ((locals.var_theta_vp_1_dn1 * locals.var_sqrt_vp_vt) + (locals.var_theta_vp_1 * locals.var_sqrt_vp_vt_dn1)))) / (assign2440_e1899 * assign2440_e1899)), ((((p.p21 * locals.var_vpprime_dn2) * assign2440_e1899) - (assign2440_e1896 * ((locals.var_theta_vp_1_dn2 * locals.var_sqrt_vp_vt) + (locals.var_theta_vp_1 * locals.var_sqrt_vp_vt_dn2)))) / (assign2440_e1899 * assign2440_e1899)), ((((p.p21 * locals.var_vpprime_dn3) * assign2440_e1899) - (assign2440_e1896 * ((locals.var_theta_vp_1_dn3 * locals.var_sqrt_vp_vt) + (locals.var_theta_vp_1 * locals.var_sqrt_vp_vt_dn3)))) / (assign2440_e1899 * assign2440_e1899)),)
    } else {
        (locals.var_tmp1, locals.var_tmp1_dn0, locals.var_tmp1_dn1, locals.var_tmp1_dn2, locals.var_tmp1_dn3,)
    }
};
        locals.var_tmp1 = assign2440_e1902;
        locals.var_tmp1_dn0 = assign2440_e1902_d_n0;
        locals.var_tmp1_dn1 = assign2440_e1902_d_n1;
        locals.var_tmp1_dn2 = assign2440_e1902_d_n2;
        locals.var_tmp1_dn3 = assign2440_e1902_d_n3;
        locals.var_tmp1_rv = 0.0;

        let (assign2450_e1908, assign2450_e1908_d_n0, assign2450_e1908_d_n1, assign2450_e1908_d_n2, assign2450_e1908_d_n3,) = {
    if (locals.var_guard18 != 0.0) {
        let assign2450_e1906: f64 = (locals.var_tmp1 * locals.var_dvp_dvd);
        (assign2450_e1906, ((locals.var_tmp1_dn0 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn0)), ((locals.var_tmp1_dn1 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn1)), ((locals.var_tmp1_dn2 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn2)), ((locals.var_tmp1_dn3 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn3)),)
    } else {
        (locals.var_dvpprime_dvd, locals.var_dvpprime_dvd_dn0, locals.var_dvpprime_dvd_dn1, locals.var_dvpprime_dvd_dn2, locals.var_dvpprime_dvd_dn3,)
    }
};
        locals.var_dvpprime_dvd = assign2450_e1908;
        locals.var_dvpprime_dvd_dn0 = assign2450_e1908_d_n0;
        locals.var_dvpprime_dvd_dn1 = assign2450_e1908_d_n1;
        locals.var_dvpprime_dvd_dn2 = assign2450_e1908_d_n2;
        locals.var_dvpprime_dvd_dn3 = assign2450_e1908_d_n3;
        locals.var_dvpprime_dvd_rv = 0.0;

        let (assign2460_e1914, assign2460_e1914_d_n0, assign2460_e1914_d_n1, assign2460_e1914_d_n2, assign2460_e1914_d_n3,) = {
    if (locals.var_guard18 != 0.0) {
        let assign2460_e1912: f64 = (locals.var_tmp1 * locals.var_dvp_dvs);
        (assign2460_e1912, ((locals.var_tmp1_dn0 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn0)), ((locals.var_tmp1_dn1 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn1)), ((locals.var_tmp1_dn2 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn2)), ((locals.var_tmp1_dn3 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn3)),)
    } else {
        (locals.var_dvpprime_dvs, locals.var_dvpprime_dvs_dn0, locals.var_dvpprime_dvs_dn1, locals.var_dvpprime_dvs_dn2, locals.var_dvpprime_dvs_dn3,)
    }
};
        locals.var_dvpprime_dvs = assign2460_e1914;
        locals.var_dvpprime_dvs_dn0 = assign2460_e1914_d_n0;
        locals.var_dvpprime_dvs_dn1 = assign2460_e1914_d_n1;
        locals.var_dvpprime_dvs_dn2 = assign2460_e1914_d_n2;
        locals.var_dvpprime_dvs_dn3 = assign2460_e1914_d_n3;
        locals.var_dvpprime_dvs_rv = 0.0;

        let (assign2480_e1927, assign2480_e1927_d_n0, assign2480_e1927_d_n1, assign2480_e1927_d_n2, assign2480_e1927_d_n3,) = {
    if (locals.var_guard18 != 0.0) {
        let assign2480_e1923: f64 = (-locals.var_dleq_dvd);
        let assign2480_e1925: f64 = (assign2480_e1923 - locals.var_dvpprime_dvd);
        (assign2480_e1925, ((-locals.var_dleq_dvd_dn0) - locals.var_dvpprime_dvd_dn0), ((-locals.var_dleq_dvd_dn1) - locals.var_dvpprime_dvd_dn1), ((-locals.var_dleq_dvd_dn2) - locals.var_dvpprime_dvd_dn2), ((-locals.var_dleq_dvd_dn3) - locals.var_dvpprime_dvd_dn3),)
    } else {
        (locals.var_dbeta_dvd, locals.var_dbeta_dvd_dn0, locals.var_dbeta_dvd_dn1, locals.var_dbeta_dvd_dn2, locals.var_dbeta_dvd_dn3,)
    }
};
        locals.var_dbeta_dvd = assign2480_e1927;
        locals.var_dbeta_dvd_dn0 = assign2480_e1927_d_n0;
        locals.var_dbeta_dvd_dn1 = assign2480_e1927_d_n1;
        locals.var_dbeta_dvd_dn2 = assign2480_e1927_d_n2;
        locals.var_dbeta_dvd_dn3 = assign2480_e1927_d_n3;
        locals.var_dbeta_dvd_rv = 0.0;

        let (assign2490_e1934, assign2490_e1934_d_n0, assign2490_e1934_d_n1, assign2490_e1934_d_n2, assign2490_e1934_d_n3,) = {
    if (locals.var_guard18 != 0.0) {
        let assign2490_e1930: f64 = (-locals.var_dleq_dvs);
        let assign2490_e1932: f64 = (assign2490_e1930 - locals.var_dvpprime_dvs);
        (assign2490_e1932, ((-locals.var_dleq_dvs_dn0) - locals.var_dvpprime_dvs_dn0), ((-locals.var_dleq_dvs_dn1) - locals.var_dvpprime_dvs_dn1), ((-locals.var_dleq_dvs_dn2) - locals.var_dvpprime_dvs_dn2), ((-locals.var_dleq_dvs_dn3) - locals.var_dvpprime_dvs_dn3),)
    } else {
        (locals.var_dbeta_dvs, locals.var_dbeta_dvs_dn0, locals.var_dbeta_dvs_dn1, locals.var_dbeta_dvs_dn2, locals.var_dbeta_dvs_dn3,)
    }
};
        locals.var_dbeta_dvs = assign2490_e1934;
        locals.var_dbeta_dvs_dn0 = assign2490_e1934_d_n0;
        locals.var_dbeta_dvs_dn1 = assign2490_e1934_d_n1;
        locals.var_dbeta_dvs_dn2 = assign2490_e1934_d_n2;
        locals.var_dbeta_dvs_dn3 = assign2490_e1934_d_n3;
        locals.var_dbeta_dvs_rv = 0.0;

        let (assign2510_e1948, assign2510_e1948_d_n0, assign2510_e1948_d_n1, assign2510_e1948_d_n2, assign2510_e1948_d_n3,) = {
    if (locals.var_guard18 == 0.0) {
        let assign2510_e1946: f64 = (locals.var_t0 / locals.var_e0_q_1);
        (assign2510_e1946, (-((locals.var_t0 * locals.var_e0_q_1_dn0) / (locals.var_e0_q_1 * locals.var_e0_q_1))), (-((locals.var_t0 * locals.var_e0_q_1_dn1) / (locals.var_e0_q_1 * locals.var_e0_q_1))), (-((locals.var_t0 * locals.var_e0_q_1_dn2) / (locals.var_e0_q_1 * locals.var_e0_q_1))), (-((locals.var_t0 * locals.var_e0_q_1_dn3) / (locals.var_e0_q_1 * locals.var_e0_q_1))),)
    } else {
        (locals.var_tmp1, locals.var_tmp1_dn0, locals.var_tmp1_dn1, locals.var_tmp1_dn2, locals.var_tmp1_dn3,)
    }
};
        locals.var_tmp1 = assign2510_e1948;
        locals.var_tmp1_dn0 = assign2510_e1948_d_n0;
        locals.var_tmp1_dn1 = assign2510_e1948_d_n1;
        locals.var_tmp1_dn2 = assign2510_e1948_d_n2;
        locals.var_tmp1_dn3 = assign2510_e1948_d_n3;
        locals.var_tmp1_rv = 0.0;

        let (assign2520_e1962, assign2520_e1962_d_n0, assign2520_e1962_d_n1, assign2520_e1962_d_n2, assign2520_e1962_d_n3,) = {
    if (locals.var_guard18 == 0.0) {
        let assign2520_e1952: f64 = (-locals.var_dleq_dvd);
        let assign2520_e1957: f64 = (locals.var_eta_qi * locals.var_dqi_dvd);
        let assign2520_e1958: f64 = (locals.var_dqb_dvd + assign2520_e1957);
        let assign2520_e1959: f64 = (locals.var_tmp1 * assign2520_e1958);
        let assign2520_e1960: f64 = (assign2520_e1952 + assign2520_e1959);
        (assign2520_e1960, ((-locals.var_dleq_dvd_dn0) + ((locals.var_tmp1_dn0 * assign2520_e1958) + (locals.var_tmp1 * (locals.var_dqb_dvd_dn0 + (locals.var_eta_qi * locals.var_dqi_dvd_dn0))))), ((-locals.var_dleq_dvd_dn1) + ((locals.var_tmp1_dn1 * assign2520_e1958) + (locals.var_tmp1 * (locals.var_dqb_dvd_dn1 + (locals.var_eta_qi * locals.var_dqi_dvd_dn1))))), ((-locals.var_dleq_dvd_dn2) + ((locals.var_tmp1_dn2 * assign2520_e1958) + (locals.var_tmp1 * (locals.var_dqb_dvd_dn2 + (locals.var_eta_qi * locals.var_dqi_dvd_dn2))))), ((-locals.var_dleq_dvd_dn3) + ((locals.var_tmp1_dn3 * assign2520_e1958) + (locals.var_tmp1 * (locals.var_dqb_dvd_dn3 + (locals.var_eta_qi * locals.var_dqi_dvd_dn3))))),)
    } else {
        (locals.var_dbeta_dvd, locals.var_dbeta_dvd_dn0, locals.var_dbeta_dvd_dn1, locals.var_dbeta_dvd_dn2, locals.var_dbeta_dvd_dn3,)
    }
};
        locals.var_dbeta_dvd = assign2520_e1962;
        locals.var_dbeta_dvd_dn0 = assign2520_e1962_d_n0;
        locals.var_dbeta_dvd_dn1 = assign2520_e1962_d_n1;
        locals.var_dbeta_dvd_dn2 = assign2520_e1962_d_n2;
        locals.var_dbeta_dvd_dn3 = assign2520_e1962_d_n3;
        locals.var_dbeta_dvd_rv = 0.0;

        let (assign2530_e1976, assign2530_e1976_d_n0, assign2530_e1976_d_n1, assign2530_e1976_d_n2, assign2530_e1976_d_n3,) = {
    if (locals.var_guard18 == 0.0) {
        let assign2530_e1966: f64 = (-locals.var_dleq_dvs);
        let assign2530_e1971: f64 = (locals.var_eta_qi * locals.var_dqi_dvs);
        let assign2530_e1972: f64 = (locals.var_dqb_dvs + assign2530_e1971);
        let assign2530_e1973: f64 = (locals.var_tmp1 * assign2530_e1972);
        let assign2530_e1974: f64 = (assign2530_e1966 + assign2530_e1973);
        (assign2530_e1974, ((-locals.var_dleq_dvs_dn0) + ((locals.var_tmp1_dn0 * assign2530_e1972) + (locals.var_tmp1 * (locals.var_dqb_dvs_dn0 + (locals.var_eta_qi * locals.var_dqi_dvs_dn0))))), ((-locals.var_dleq_dvs_dn1) + ((locals.var_tmp1_dn1 * assign2530_e1972) + (locals.var_tmp1 * (locals.var_dqb_dvs_dn1 + (locals.var_eta_qi * locals.var_dqi_dvs_dn1))))), ((-locals.var_dleq_dvs_dn2) + ((locals.var_tmp1_dn2 * assign2530_e1972) + (locals.var_tmp1 * (locals.var_dqb_dvs_dn2 + (locals.var_eta_qi * locals.var_dqi_dvs_dn2))))), ((-locals.var_dleq_dvs_dn3) + ((locals.var_tmp1_dn3 * assign2530_e1972) + (locals.var_tmp1 * (locals.var_dqb_dvs_dn3 + (locals.var_eta_qi * locals.var_dqi_dvs_dn3))))),)
    } else {
        (locals.var_dbeta_dvs, locals.var_dbeta_dvs_dn0, locals.var_dbeta_dvs_dn1, locals.var_dbeta_dvs_dn2, locals.var_dbeta_dvs_dn3,)
    }
};
        locals.var_dbeta_dvs = assign2530_e1976;
        locals.var_dbeta_dvs_dn0 = assign2530_e1976_d_n0;
        locals.var_dbeta_dvs_dn1 = assign2530_e1976_d_n1;
        locals.var_dbeta_dvs_dn2 = assign2530_e1976_d_n2;
        locals.var_dbeta_dvs_dn3 = assign2530_e1976_d_n3;
        locals.var_dbeta_dvs_rv = 0.0;

        let assign2550_e1992: f64 = (-locals.var_gamma_s);
        let assign2550_e1995: f64 = (4.0 * locals.var_n);
        let assign2550_e1997: f64 = (assign2550_e1995 * locals.var_sqrt_phi_vp);
        let assign2550_e2000: f64 = (locals.var_phi_t + locals.var_vp);
        let assign2550_e2002: f64 = (assign2550_e2000 + locals.var_vt_4);
        let assign2550_e2003: f64 = (assign2550_e1997 * assign2550_e2002);
        let assign2550_e2004: f64 = (assign2550_e1992 / assign2550_e2003);
        locals.var_tmp1 = assign2550_e2004;
        locals.var_tmp1_dn0 = (-((assign2550_e1992 * (((((4.0 * locals.var_n_dn0) * locals.var_sqrt_phi_vp) + (assign2550_e1995 * locals.var_sqrt_phi_vp_dn0)) * assign2550_e2002) + (assign2550_e1997 * (locals.var_phi_t_dn0 + locals.var_vp_dn0)))) / (assign2550_e2003 * assign2550_e2003)));
        locals.var_tmp1_dn1 = (-((assign2550_e1992 * (((((4.0 * locals.var_n_dn1) * locals.var_sqrt_phi_vp) + (assign2550_e1995 * locals.var_sqrt_phi_vp_dn1)) * assign2550_e2002) + (assign2550_e1997 * (locals.var_phi_t_dn1 + locals.var_vp_dn1)))) / (assign2550_e2003 * assign2550_e2003)));
        locals.var_tmp1_dn2 = (-((assign2550_e1992 * (((((4.0 * locals.var_n_dn2) * locals.var_sqrt_phi_vp) + (assign2550_e1995 * locals.var_sqrt_phi_vp_dn2)) * assign2550_e2002) + (assign2550_e1997 * (locals.var_phi_t_dn2 + locals.var_vp_dn2)))) / (assign2550_e2003 * assign2550_e2003)));
        locals.var_tmp1_dn3 = (-((assign2550_e1992 * (((((4.0 * locals.var_n_dn3) * locals.var_sqrt_phi_vp) + (assign2550_e1995 * locals.var_sqrt_phi_vp_dn3)) * assign2550_e2002) + (assign2550_e1997 * (locals.var_phi_t_dn3 + locals.var_vp_dn3)))) / (assign2550_e2003 * assign2550_e2003)));
        locals.var_tmp1_rv = 0.0;

        let assign2560_e2007: f64 = (locals.var_tmp1 * locals.var_dvp_dvd);
        locals.var_dn_dvd = assign2560_e2007;
        locals.var_dn_dvd_dn0 = ((locals.var_tmp1_dn0 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn0));
        locals.var_dn_dvd_dn1 = ((locals.var_tmp1_dn1 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn1));
        locals.var_dn_dvd_dn2 = ((locals.var_tmp1_dn2 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn2));
        locals.var_dn_dvd_dn3 = ((locals.var_tmp1_dn3 * locals.var_dvp_dvd) + (locals.var_tmp1 * locals.var_dvp_dvd_dn3));
        locals.var_dn_dvd_rv = 0.0;

        let assign2570_e2010: f64 = (locals.var_tmp1 * locals.var_dvp_dvs);
        locals.var_dn_dvs = assign2570_e2010;
        locals.var_dn_dvs_dn0 = ((locals.var_tmp1_dn0 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn0));
        locals.var_dn_dvs_dn1 = ((locals.var_tmp1_dn1 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn1));
        locals.var_dn_dvs_dn2 = ((locals.var_tmp1_dn2 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn2));
        locals.var_dn_dvs_dn3 = ((locals.var_tmp1_dn3 * locals.var_dvp_dvs) + (locals.var_tmp1 * locals.var_dvp_dvs_dn3));
        locals.var_dn_dvs_rv = 0.0;

        let assign2590_e2017: f64 = (locals.var_dn_dvd + locals.var_dbeta_dvd);
        let assign2590_e2019: f64 = (assign2590_e2017 * locals.var_if_ir);
        let assign2590_e2021: f64 = (assign2590_e2019 + locals.var_dif_dvd);
        let assign2590_e2023: f64 = (assign2590_e2021 - locals.var_dirprime_dvd);
        let assign2590_e2024: f64 = (locals.var_ispec * assign2590_e2023);
        locals.var_gds = assign2590_e2024;
        locals.var_gds_dn0 = ((locals.var_ispec_dn0 * assign2590_e2023) + (locals.var_ispec * (((((locals.var_dn_dvd_dn0 + locals.var_dbeta_dvd_dn0) * locals.var_if_ir) + (assign2590_e2017 * locals.var_if_ir_dn0)) + locals.var_dif_dvd_dn0) - locals.var_dirprime_dvd_dn0)));
        locals.var_gds_dn1 = ((locals.var_ispec_dn1 * assign2590_e2023) + (locals.var_ispec * (((((locals.var_dn_dvd_dn1 + locals.var_dbeta_dvd_dn1) * locals.var_if_ir) + (assign2590_e2017 * locals.var_if_ir_dn1)) + locals.var_dif_dvd_dn1) - locals.var_dirprime_dvd_dn1)));
        locals.var_gds_dn2 = ((locals.var_ispec_dn2 * assign2590_e2023) + (locals.var_ispec * (((((locals.var_dn_dvd_dn2 + locals.var_dbeta_dvd_dn2) * locals.var_if_ir) + (assign2590_e2017 * locals.var_if_ir_dn2)) + locals.var_dif_dvd_dn2) - locals.var_dirprime_dvd_dn2)));
        locals.var_gds_dn3 = ((locals.var_ispec_dn3 * assign2590_e2023) + (locals.var_ispec * (((((locals.var_dn_dvd_dn3 + locals.var_dbeta_dvd_dn3) * locals.var_if_ir) + (assign2590_e2017 * locals.var_if_ir_dn3)) + locals.var_dif_dvd_dn3) - locals.var_dirprime_dvd_dn3)));
        locals.var_gds_rv = 0.0;

        let assign2600_e2026: f64 = (-locals.var_ispec);
        let assign2600_e2029: f64 = (locals.var_dn_dvs + locals.var_dbeta_dvs);
        let assign2600_e2031: f64 = (assign2600_e2029 * locals.var_if_ir);
        let assign2600_e2033: f64 = (assign2600_e2031 + locals.var_dif_dvs);
        let assign2600_e2035: f64 = (assign2600_e2033 - locals.var_dirprime_dvs);
        let assign2600_e2036: f64 = (assign2600_e2026 * assign2600_e2035);
        locals.var_gms = assign2600_e2036;
        locals.var_gms_dn0 = (((-locals.var_ispec_dn0) * assign2600_e2035) + (assign2600_e2026 * (((((locals.var_dn_dvs_dn0 + locals.var_dbeta_dvs_dn0) * locals.var_if_ir) + (assign2600_e2029 * locals.var_if_ir_dn0)) + locals.var_dif_dvs_dn0) - locals.var_dirprime_dvs_dn0)));
        locals.var_gms_dn1 = (((-locals.var_ispec_dn1) * assign2600_e2035) + (assign2600_e2026 * (((((locals.var_dn_dvs_dn1 + locals.var_dbeta_dvs_dn1) * locals.var_if_ir) + (assign2600_e2029 * locals.var_if_ir_dn1)) + locals.var_dif_dvs_dn1) - locals.var_dirprime_dvs_dn1)));
        locals.var_gms_dn2 = (((-locals.var_ispec_dn2) * assign2600_e2035) + (assign2600_e2026 * (((((locals.var_dn_dvs_dn2 + locals.var_dbeta_dvs_dn2) * locals.var_if_ir) + (assign2600_e2029 * locals.var_if_ir_dn2)) + locals.var_dif_dvs_dn2) - locals.var_dirprime_dvs_dn2)));
        locals.var_gms_dn3 = (((-locals.var_ispec_dn3) * assign2600_e2035) + (assign2600_e2026 * (((((locals.var_dn_dvs_dn3 + locals.var_dbeta_dvs_dn3) * locals.var_if_ir) + (assign2600_e2029 * locals.var_if_ir_dn3)) + locals.var_dif_dvs_dn3) - locals.var_dirprime_dvs_dn3)));
        locals.var_gms_rv = 0.0;

        let assign2630_e2055: f64 = (p.p36 * p.p37);
        let assign2630_e2058: f64 = (locals.var_weff - p.p27);
        let assign2630_e2059: f64 = (assign2630_e2055 / assign2630_e2058);
        locals.var_rseff = assign2630_e2059;
        locals.var_rseff_rv = 0.0;

        let assign2640_e2062: f64 = (p.p36 * p.p37);
        let assign2640_e2065: f64 = (locals.var_weff - p.p27);
        let assign2640_e2066: f64 = (assign2640_e2062 / assign2640_e2065);
        locals.var_rdeff = assign2640_e2066;
        locals.var_rdeff_rv = 0.0;

        let assign2650_e2071: f64 = (locals.var_gms * locals.var_rseff);
        let assign2650_e2072: f64 = (1.0 + assign2650_e2071);
        let assign2650_e2075: f64 = (locals.var_gds * locals.var_rdeff);
        let assign2650_e2076: f64 = (assign2650_e2072 + assign2650_e2075);
        let assign2650_e2077: f64 = (1.0 / assign2650_e2076);
        locals.var_tmp1 = assign2650_e2077;
        locals.var_tmp1_dn0 = (-(((locals.var_gms_dn0 * locals.var_rseff) + (locals.var_gds_dn0 * locals.var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));
        locals.var_tmp1_dn1 = (-(((locals.var_gms_dn1 * locals.var_rseff) + (locals.var_gds_dn1 * locals.var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));
        locals.var_tmp1_dn2 = (-(((locals.var_gms_dn2 * locals.var_rseff) + (locals.var_gds_dn2 * locals.var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));
        locals.var_tmp1_dn3 = (-(((locals.var_gms_dn3 * locals.var_rseff) + (locals.var_gds_dn3 * locals.var_rdeff)) / (assign2650_e2076 * assign2650_e2076)));
        locals.var_tmp1_rv = 0.0;

        let assign2800_e2163: f64 = (locals.var_weff * locals.var_leff);
        let assign2800_e2165: f64 = (assign2800_e2163 * p.p13);
        locals.var_wlcox = assign2800_e2165;
        locals.var_wlcox_rv = 0.0;

        let assign2810_e2168: f64 = (locals.var_sif * locals.var_sif2);
        locals.var_sif3 = assign2810_e2168;
        locals.var_sif3_dn0 = ((locals.var_sif_dn0 * locals.var_sif2) + (locals.var_sif * locals.var_sif2_dn0));
        locals.var_sif3_dn1 = ((locals.var_sif_dn1 * locals.var_sif2) + (locals.var_sif * locals.var_sif2_dn1));
        locals.var_sif3_dn2 = ((locals.var_sif_dn2 * locals.var_sif2) + (locals.var_sif * locals.var_sif2_dn2));
        locals.var_sif3_dn3 = ((locals.var_sif_dn3 * locals.var_sif2) + (locals.var_sif * locals.var_sif2_dn3));
        locals.var_sif3_rv = 0.0;

        let assign2820_e2171: f64 = (locals.var_sir * locals.var_sir2);
        locals.var_sir3 = assign2820_e2171;
        locals.var_sir3_dn0 = ((locals.var_sir_dn0 * locals.var_sir2) + (locals.var_sir * locals.var_sir2_dn0));
        locals.var_sir3_dn1 = ((locals.var_sir_dn1 * locals.var_sir2) + (locals.var_sir * locals.var_sir2_dn1));
        locals.var_sir3_dn2 = ((locals.var_sir_dn2 * locals.var_sir2) + (locals.var_sir * locals.var_sir2_dn2));
        locals.var_sir3_dn3 = ((locals.var_sir_dn3 * locals.var_sir2) + (locals.var_sir * locals.var_sir2_dn3));
        locals.var_sir3_rv = 0.0;

        let assign2830_e2175: f64 = (0.5 * locals.var_vp);
        let assign2830_e2176: f64 = (locals.var_phi_t + assign2830_e2175);
        let assign2830_e2177: f64 = (assign2830_e2176).sqrt();
        locals.var_tmp1 = assign2830_e2177;
        locals.var_tmp1_dn0 = ((locals.var_phi_t_dn0 + (0.5 * locals.var_vp_dn0)) / (2.0 * assign2830_e2177));
        locals.var_tmp1_dn1 = ((locals.var_phi_t_dn1 + (0.5 * locals.var_vp_dn1)) / (2.0 * assign2830_e2177));
        locals.var_tmp1_dn2 = ((locals.var_phi_t_dn2 + (0.5 * locals.var_vp_dn2)) / (2.0 * assign2830_e2177));
        locals.var_tmp1_dn3 = ((locals.var_phi_t_dn3 + (0.5 * locals.var_vp_dn3)) / (2.0 * assign2830_e2177));
        locals.var_tmp1_rv = 0.0;

        let assign2840_e2180: f64 = (locals.var_tmp1 + locals.var_tmp1);
        locals.var_sqrt_phi_vp2_2 = assign2840_e2180;
        locals.var_sqrt_phi_vp2_2_dn0 = (locals.var_tmp1_dn0 + locals.var_tmp1_dn0);
        locals.var_sqrt_phi_vp2_2_dn1 = (locals.var_tmp1_dn1 + locals.var_tmp1_dn1);
        locals.var_sqrt_phi_vp2_2_dn2 = (locals.var_tmp1_dn2 + locals.var_tmp1_dn2);
        locals.var_sqrt_phi_vp2_2_dn3 = (locals.var_tmp1_dn3 + locals.var_tmp1_dn3);
        locals.var_sqrt_phi_vp2_2_rv = 0.0;

        let assign2850_e2184: f64 = (locals.var_gammaprime / locals.var_sqrt_phi_vp2_2);
        let assign2850_e2185: f64 = (1.0 + assign2850_e2184);
        let assign2850_e2187: f64 = (assign2850_e2185 * locals.var_vt);
        let assign2850_e2189: f64 = (assign2850_e2187 * locals.var_wlcox);
        locals.var_n_vt_cox = assign2850_e2189;
        locals.var_n_vt_cox_dn0 = (((((locals.var_gammaprime_dn0 * locals.var_sqrt_phi_vp2_2) - (locals.var_gammaprime * locals.var_sqrt_phi_vp2_2_dn0)) / (locals.var_sqrt_phi_vp2_2 * locals.var_sqrt_phi_vp2_2)) * locals.var_vt) * locals.var_wlcox);
        locals.var_n_vt_cox_dn1 = (((((locals.var_gammaprime_dn1 * locals.var_sqrt_phi_vp2_2) - (locals.var_gammaprime * locals.var_sqrt_phi_vp2_2_dn1)) / (locals.var_sqrt_phi_vp2_2 * locals.var_sqrt_phi_vp2_2)) * locals.var_vt) * locals.var_wlcox);
        locals.var_n_vt_cox_dn2 = (((((locals.var_gammaprime_dn2 * locals.var_sqrt_phi_vp2_2) - (locals.var_gammaprime * locals.var_sqrt_phi_vp2_2_dn2)) / (locals.var_sqrt_phi_vp2_2 * locals.var_sqrt_phi_vp2_2)) * locals.var_vt) * locals.var_wlcox);
        locals.var_n_vt_cox_dn3 = (((((locals.var_gammaprime_dn3 * locals.var_sqrt_phi_vp2_2) - (locals.var_gammaprime * locals.var_sqrt_phi_vp2_2_dn3)) / (locals.var_sqrt_phi_vp2_2 * locals.var_sqrt_phi_vp2_2)) * locals.var_vt) * locals.var_wlcox);
        locals.var_n_vt_cox_rv = 0.0;

        let assign2860_e2191: f64 = (-locals.var_n_vt_cox);
        let assign2860_e2195: f64 = (3.0 * locals.var_sir3);
        let assign2860_e2198: f64 = (6.0 * locals.var_sir2);
        let assign2860_e2200: f64 = (assign2860_e2198 * locals.var_sif);
        let assign2860_e2201: f64 = (assign2860_e2195 + assign2860_e2200);
        let assign2860_e2204: f64 = (4.0 * locals.var_sir);
        let assign2860_e2206: f64 = (assign2860_e2204 * locals.var_sif2);
        let assign2860_e2207: f64 = (assign2860_e2201 + assign2860_e2206);
        let assign2860_e2210: f64 = (2.0 * locals.var_sif3);
        let assign2860_e2211: f64 = (assign2860_e2207 + assign2860_e2210);
        let assign2860_e2212: f64 = (0.266666666 * assign2860_e2211);
        let assign2860_e2214: f64 = (assign2860_e2212 / locals.var_sif_sir_2);
        let assign2860_e2216: f64 = (assign2860_e2214 - 0.5);
        let assign2860_e2217: f64 = (assign2860_e2191 * assign2860_e2216);
        locals.var_qd = assign2860_e2217;
        locals.var_qd_dn0 = (((-locals.var_n_vt_cox_dn0) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * locals.var_sir3_dn0) + (((6.0 * locals.var_sir2_dn0) * locals.var_sif) + (assign2860_e2198 * locals.var_sif_dn0))) + (((4.0 * locals.var_sir_dn0) * locals.var_sif2) + (assign2860_e2204 * locals.var_sif2_dn0))) + (2.0 * locals.var_sif3_dn0))) * locals.var_sif_sir_2) - (assign2860_e2212 * locals.var_sif_sir_2_dn0)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));
        locals.var_qd_dn1 = (((-locals.var_n_vt_cox_dn1) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * locals.var_sir3_dn1) + (((6.0 * locals.var_sir2_dn1) * locals.var_sif) + (assign2860_e2198 * locals.var_sif_dn1))) + (((4.0 * locals.var_sir_dn1) * locals.var_sif2) + (assign2860_e2204 * locals.var_sif2_dn1))) + (2.0 * locals.var_sif3_dn1))) * locals.var_sif_sir_2) - (assign2860_e2212 * locals.var_sif_sir_2_dn1)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));
        locals.var_qd_dn2 = (((-locals.var_n_vt_cox_dn2) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * locals.var_sir3_dn2) + (((6.0 * locals.var_sir2_dn2) * locals.var_sif) + (assign2860_e2198 * locals.var_sif_dn2))) + (((4.0 * locals.var_sir_dn2) * locals.var_sif2) + (assign2860_e2204 * locals.var_sif2_dn2))) + (2.0 * locals.var_sif3_dn2))) * locals.var_sif_sir_2) - (assign2860_e2212 * locals.var_sif_sir_2_dn2)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));
        locals.var_qd_dn3 = (((-locals.var_n_vt_cox_dn3) * assign2860_e2216) + (assign2860_e2191 * ((((0.266666666 * ((((3.0 * locals.var_sir3_dn3) + (((6.0 * locals.var_sir2_dn3) * locals.var_sif) + (assign2860_e2198 * locals.var_sif_dn3))) + (((4.0 * locals.var_sir_dn3) * locals.var_sif2) + (assign2860_e2204 * locals.var_sif2_dn3))) + (2.0 * locals.var_sif3_dn3))) * locals.var_sif_sir_2) - (assign2860_e2212 * locals.var_sif_sir_2_dn3)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));
        locals.var_qd_rv = 0.0;

        let assign2870_e2219: f64 = (-locals.var_n_vt_cox);
        let assign2870_e2223: f64 = (3.0 * locals.var_sif3);
        let assign2870_e2226: f64 = (6.0 * locals.var_sif2);
        let assign2870_e2228: f64 = (assign2870_e2226 * locals.var_sir);
        let assign2870_e2229: f64 = (assign2870_e2223 + assign2870_e2228);
        let assign2870_e2232: f64 = (4.0 * locals.var_sif);
        let assign2870_e2234: f64 = (assign2870_e2232 * locals.var_sir2);
        let assign2870_e2235: f64 = (assign2870_e2229 + assign2870_e2234);
        let assign2870_e2238: f64 = (2.0 * locals.var_sir3);
        let assign2870_e2239: f64 = (assign2870_e2235 + assign2870_e2238);
        let assign2870_e2240: f64 = (0.266666666 * assign2870_e2239);
        let assign2870_e2242: f64 = (assign2870_e2240 / locals.var_sif_sir_2);
        let assign2870_e2244: f64 = (assign2870_e2242 - 0.5);
        let assign2870_e2245: f64 = (assign2870_e2219 * assign2870_e2244);
        locals.var_qs = assign2870_e2245;
        locals.var_qs_dn0 = (((-locals.var_n_vt_cox_dn0) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * locals.var_sif3_dn0) + (((6.0 * locals.var_sif2_dn0) * locals.var_sir) + (assign2870_e2226 * locals.var_sir_dn0))) + (((4.0 * locals.var_sif_dn0) * locals.var_sir2) + (assign2870_e2232 * locals.var_sir2_dn0))) + (2.0 * locals.var_sir3_dn0))) * locals.var_sif_sir_2) - (assign2870_e2240 * locals.var_sif_sir_2_dn0)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));
        locals.var_qs_dn1 = (((-locals.var_n_vt_cox_dn1) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * locals.var_sif3_dn1) + (((6.0 * locals.var_sif2_dn1) * locals.var_sir) + (assign2870_e2226 * locals.var_sir_dn1))) + (((4.0 * locals.var_sif_dn1) * locals.var_sir2) + (assign2870_e2232 * locals.var_sir2_dn1))) + (2.0 * locals.var_sir3_dn1))) * locals.var_sif_sir_2) - (assign2870_e2240 * locals.var_sif_sir_2_dn1)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));
        locals.var_qs_dn2 = (((-locals.var_n_vt_cox_dn2) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * locals.var_sif3_dn2) + (((6.0 * locals.var_sif2_dn2) * locals.var_sir) + (assign2870_e2226 * locals.var_sir_dn2))) + (((4.0 * locals.var_sif_dn2) * locals.var_sir2) + (assign2870_e2232 * locals.var_sir2_dn2))) + (2.0 * locals.var_sir3_dn2))) * locals.var_sif_sir_2) - (assign2870_e2240 * locals.var_sif_sir_2_dn2)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));
        locals.var_qs_dn3 = (((-locals.var_n_vt_cox_dn3) * assign2870_e2244) + (assign2870_e2219 * ((((0.266666666 * ((((3.0 * locals.var_sif3_dn3) + (((6.0 * locals.var_sif2_dn3) * locals.var_sir) + (assign2870_e2226 * locals.var_sir_dn3))) + (((4.0 * locals.var_sif_dn3) * locals.var_sir2) + (assign2870_e2232 * locals.var_sir2_dn3))) + (2.0 * locals.var_sir3_dn3))) * locals.var_sif_sir_2) - (assign2870_e2240 * locals.var_sif_sir_2_dn3)) / (locals.var_sif_sir_2 * locals.var_sif_sir_2))));
        locals.var_qs_rv = 0.0;

        let assign2880_e2248: f64 = (locals.var_qs + locals.var_qd);
        locals.var_qi_1 = assign2880_e2248;
        locals.var_qi_1_dn0 = (locals.var_qs_dn0 + locals.var_qd_dn0);
        locals.var_qi_1_dn1 = (locals.var_qs_dn1 + locals.var_qd_dn1);
        locals.var_qi_1_dn2 = (locals.var_qs_dn2 + locals.var_qd_dn2);
        locals.var_qi_1_dn3 = (locals.var_qs_dn3 + locals.var_qd_dn3);
        locals.var_qi_1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let assign2890_e2251: f64 = (-0.5);
        let assign2890_e2253: f64 = (assign2890_e2251 * locals.var_gammaprime);
        let assign2890_e2255: f64 = (assign2890_e2253 * locals.var_sqrt_phi_vp_2);
        let assign2890_e2257: f64 = (assign2890_e2255 + locals.var_vgprime);
        let assign2890_e2259: f64 = (assign2890_e2257 - locals.var_vgstar);
        let assign2890_e2260: f64 = (locals.var_wlcox * assign2890_e2259);
        let assign2890_e2263: f64 = (locals.var_qi_1 * locals.var_gammaprime);
        let assign2890_e2266: f64 = (locals.var_gammaprime + locals.var_sqrt_phi_vp2_2);
        let assign2890_e2267: f64 = (assign2890_e2263 / assign2890_e2266);
        let assign2890_e2268: f64 = (assign2890_e2260 - assign2890_e2267);
        locals.var_qb_1 = assign2890_e2268;
        locals.var_qb_1_dn0 = ((locals.var_wlcox * (((((assign2890_e2251 * locals.var_gammaprime_dn0) * locals.var_sqrt_phi_vp_2) + (assign2890_e2253 * locals.var_sqrt_phi_vp_2_dn0)) + locals.var_vgprime_dn0) - locals.var_vgstar_dn0)) - (((((locals.var_qi_1_dn0 * locals.var_gammaprime) + (locals.var_qi_1 * locals.var_gammaprime_dn0)) * assign2890_e2266) - (assign2890_e2263 * (locals.var_gammaprime_dn0 + locals.var_sqrt_phi_vp2_2_dn0))) / (assign2890_e2266 * assign2890_e2266)));
        locals.var_qb_1_dn1 = ((locals.var_wlcox * (((((assign2890_e2251 * locals.var_gammaprime_dn1) * locals.var_sqrt_phi_vp_2) + (assign2890_e2253 * locals.var_sqrt_phi_vp_2_dn1)) + locals.var_vgprime_dn1) - locals.var_vgstar_dn1)) - (((((locals.var_qi_1_dn1 * locals.var_gammaprime) + (locals.var_qi_1 * locals.var_gammaprime_dn1)) * assign2890_e2266) - (assign2890_e2263 * (locals.var_gammaprime_dn1 + locals.var_sqrt_phi_vp2_2_dn1))) / (assign2890_e2266 * assign2890_e2266)));
        locals.var_qb_1_dn2 = ((locals.var_wlcox * (((((assign2890_e2251 * locals.var_gammaprime_dn2) * locals.var_sqrt_phi_vp_2) + (assign2890_e2253 * locals.var_sqrt_phi_vp_2_dn2)) + locals.var_vgprime_dn2) - locals.var_vgstar_dn2)) - (((((locals.var_qi_1_dn2 * locals.var_gammaprime) + (locals.var_qi_1 * locals.var_gammaprime_dn2)) * assign2890_e2266) - (assign2890_e2263 * (locals.var_gammaprime_dn2 + locals.var_sqrt_phi_vp2_2_dn2))) / (assign2890_e2266 * assign2890_e2266)));
        locals.var_qb_1_dn3 = ((locals.var_wlcox * (((((assign2890_e2251 * locals.var_gammaprime_dn3) * locals.var_sqrt_phi_vp_2) + (assign2890_e2253 * locals.var_sqrt_phi_vp_2_dn3)) + locals.var_vgprime_dn3) - locals.var_vgstar_dn3)) - (((((locals.var_qi_1_dn3 * locals.var_gammaprime) + (locals.var_qi_1 * locals.var_gammaprime_dn3)) * assign2890_e2266) - (assign2890_e2263 * (locals.var_gammaprime_dn3 + locals.var_sqrt_phi_vp2_2_dn3))) / (assign2890_e2266 * assign2890_e2266)));
        locals.var_qb_1_rv = 0.0;

        let assign2900_e2270: f64 = (-locals.var_qi_1);
        let assign2900_e2272: f64 = (assign2900_e2270 - locals.var_qb_1);
        locals.var_qg = assign2900_e2272;
        locals.var_qg_dn0 = ((-locals.var_qi_1_dn0) - locals.var_qb_1_dn0);
        locals.var_qg_dn1 = ((-locals.var_qi_1_dn1) - locals.var_qb_1_dn1);
        locals.var_qg_dn2 = ((-locals.var_qi_1_dn2) - locals.var_qb_1_dn2);
        locals.var_qg_dn3 = ((-locals.var_qi_1_dn3) - locals.var_qb_1_dn3);
        locals.var_qg_rv = 0.0;

        let assign2910_e2274_q: f64 = locals.var_qd;
        locals.var_ddt_qd = locals.var_qd;
        locals.var_ddt_qd_dn0 = locals.var_qd_dn0;
        locals.var_ddt_qd_dn1 = locals.var_qd_dn1;
        locals.var_ddt_qd_dn2 = locals.var_qd_dn2;
        locals.var_ddt_qd_dn3 = locals.var_qd_dn3;
        locals.var_ddt_qd_rv = assign2910_e2274_q;
        locals.var_ddt_qd_rdn0 = locals.var_qd_dn0;
        locals.var_ddt_qd_rdn1 = locals.var_qd_dn1;
        locals.var_ddt_qd_rdn2 = locals.var_qd_dn2;
        locals.var_ddt_qd_rdn3 = locals.var_qd_dn3;

        let assign2920_e2276_q: f64 = locals.var_qs;
        locals.var_ddt_qs = locals.var_qs;
        locals.var_ddt_qs_dn0 = locals.var_qs_dn0;
        locals.var_ddt_qs_dn1 = locals.var_qs_dn1;
        locals.var_ddt_qs_dn2 = locals.var_qs_dn2;
        locals.var_ddt_qs_dn3 = locals.var_qs_dn3;
        locals.var_ddt_qs_rv = assign2920_e2276_q;
        locals.var_ddt_qs_rdn0 = locals.var_qs_dn0;
        locals.var_ddt_qs_rdn1 = locals.var_qs_dn1;
        locals.var_ddt_qs_rdn2 = locals.var_qs_dn2;
        locals.var_ddt_qs_rdn3 = locals.var_qs_dn3;

        let assign2930_e2279: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign2930_e2279;
        locals.var_guard21_rv = 0.0;

        let assign2960_e2312: f64 = if ((p.p9 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard24 = assign2960_e2312;
        locals.var_guard24_rv = 0.0;

        let (assign2970_e2320,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2970_e2316: f64 = (2.0 * p.p37);
        let assign2970_e2318: f64 = (assign2970_e2316 * locals.var_weff);
        (assign2970_e2318,)
    } else {
        (locals.var_as_i,)
    }
};
        locals.var_as_i = assign2970_e2320;
        locals.var_as_i_rv = 0.0;

        let (assign2980_e2325,) = {
    if (locals.var_guard24 == 0.0) {
        (p.p9,)
    } else {
        (locals.var_as_i,)
    }
};
        locals.var_as_i = assign2980_e2325;
        locals.var_as_i_rv = 0.0;

        let assign2990_e2332: f64 = if ((p.p11 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard25 = assign2990_e2332;
        locals.var_guard25_rv = 0.0;

        let (assign3000_e2342,) = {
    if (locals.var_guard25 != 0.0) {
        let assign3000_e2336: f64 = (4.0 * p.p37);
        let assign3000_e2339: f64 = locals.var_weff;
        let assign3000_e2340: f64 = (assign3000_e2336 + assign3000_e2339);
        (assign3000_e2340,)
    } else {
        (locals.var_ps_i,)
    }
};
        locals.var_ps_i = assign3000_e2342;
        locals.var_ps_i_rv = 0.0;

        let (assign3010_e2347,) = {
    if (locals.var_guard25 == 0.0) {
        (p.p11,)
    } else {
        (locals.var_ps_i,)
    }
};
        locals.var_ps_i = assign3010_e2347;
        locals.var_ps_i_rv = 0.0;

        let assign3020_e2354: f64 = if ((p.p10 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard26 = assign3020_e2354;
        locals.var_guard26_rv = 0.0;

        let (assign3030_e2362,) = {
    if (locals.var_guard26 != 0.0) {
        let assign3030_e2358: f64 = (2.0 * p.p37);
        let assign3030_e2360: f64 = (assign3030_e2358 * locals.var_weff);
        (assign3030_e2360,)
    } else {
        (locals.var_ad_i,)
    }
};
        locals.var_ad_i = assign3030_e2362;
        locals.var_ad_i_rv = 0.0;

        let (assign3040_e2367,) = {
    if (locals.var_guard26 == 0.0) {
        (p.p10,)
    } else {
        (locals.var_ad_i,)
    }
};
        locals.var_ad_i = assign3040_e2367;
        locals.var_ad_i_rv = 0.0;

        let assign3050_e2374: f64 = if ((p.p12 == 0.0) && (p.p37 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard27 = assign3050_e2374;
        locals.var_guard27_rv = 0.0;

        let (assign3060_e2384,) = {
    if (locals.var_guard27 != 0.0) {
        let assign3060_e2378: f64 = (4.0 * p.p37);
        let assign3060_e2381: f64 = locals.var_weff;
        let assign3060_e2382: f64 = (assign3060_e2378 + assign3060_e2381);
        (assign3060_e2382,)
    } else {
        (locals.var_pd_i,)
    }
};
        locals.var_pd_i = assign3060_e2384;
        locals.var_pd_i_rv = 0.0;

        let (assign3070_e2389,) = {
    if (locals.var_guard27 == 0.0) {
        (p.p12,)
    } else {
        (locals.var_pd_i,)
    }
};
        locals.var_pd_i = assign3070_e2389;
        locals.var_pd_i_rv = 0.0;

        let assign3120_e2418: f64 = (p.p69 * locals.var_deltat);
        let assign3120_e2419: f64 = (p.p50 - assign3120_e2418);
        locals.var_pb_t = assign3120_e2419;
        locals.var_pb_t_rv = 0.0;

        let assign3130_e2423: f64 = (p.p70 * locals.var_deltat);
        let assign3130_e2424: f64 = (p.p51 - assign3130_e2423);
        locals.var_pbsw_t = assign3130_e2424;
        locals.var_pbsw_t_rv = 0.0;

        let assign3140_e2428: f64 = (p.p71 * locals.var_deltat);
        let assign3140_e2429: f64 = (p.p52 - assign3140_e2428);
        locals.var_pbswg_t = assign3140_e2429;
        locals.var_pbswg_t_rv = 0.0;

        let assign3150_e2434: f64 = (p.p66 * locals.var_deltat);
        let assign3150_e2435: f64 = (1.0 + assign3150_e2434);
        let assign3150_e2436: f64 = (p.p53 * assign3150_e2435);
        locals.var_cj_t = assign3150_e2436;
        locals.var_cj_t_rv = 0.0;

        let assign3160_e2441: f64 = (p.p67 * locals.var_deltat);
        let assign3160_e2442: f64 = (1.0 + assign3160_e2441);
        let assign3160_e2443: f64 = (p.p54 * assign3160_e2442);
        locals.var_cjsw_t = assign3160_e2443;
        locals.var_cjsw_t_rv = 0.0;

        let assign3170_e2448: f64 = (p.p68 * locals.var_deltat);
        let assign3170_e2449: f64 = (1.0 + assign3170_e2448);
        let assign3170_e2450: f64 = (p.p55 * assign3170_e2449);
        locals.var_cjswg_t = assign3170_e2450;
        locals.var_cjswg_t_rv = 0.0;

        let assign3210_e2480: f64 = (p.p0 * (nv0 - nv3));
        locals.var_v_di_b = assign3210_e2480;
        locals.var_v_di_b_dn0 = p.p0;
        locals.var_v_di_b_dn3 = (-p.p0);
        locals.var_v_di_b_rv = 0.0;

        let assign3220_e2483: f64 = (p.p0 * (nv2 - nv3));
        locals.var_v_si_b = assign3220_e2483;
        locals.var_v_si_b_dn2 = p.p0;
        locals.var_v_si_b_dn3 = (-p.p0);
        locals.var_v_si_b_rv = 0.0;

        let assign3450_e2740: f64 = if locals.var_v_di_b > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign3450_e2740;
        locals.var_guard32_rv = 0.0;

        let (assign3460_e2757, assign3460_e2757_d_n0, assign3460_e2757_d_n3,) = {
    if (locals.var_guard32 != 0.0) {
        let assign3460_e2744: f64 = (locals.var_cj_t * locals.var_ad_i);
        let assign3460_e2746: f64 = (-p.p47);
        let assign3460_e2750: f64 = (locals.var_v_di_b / locals.var_pb_t);
        let assign3460_e2751: f64 = (1.0 + assign3460_e2750);
        let assign3460_e2752: f64 = (assign3460_e2751).ln();
        let assign3460_e2753: f64 = (assign3460_e2746 * assign3460_e2752);
        let assign3460_e2754: f64 = (assign3460_e2753).exp();
        let assign3460_e2755: f64 = (assign3460_e2744 * assign3460_e2754);
        (assign3460_e2755, (assign3460_e2744 * (assign3460_e2754 * (assign3460_e2746 * ((locals.var_v_di_b_dn0 / locals.var_pb_t) / assign3460_e2751)))), (assign3460_e2744 * (assign3460_e2754 * (assign3460_e2746 * ((locals.var_v_di_b_dn3 / locals.var_pb_t) / assign3460_e2751)))),)
    } else {
        (locals.var_csb_d, locals.var_csb_d_dn0, locals.var_csb_d_dn3,)
    }
};
        locals.var_csb_d = assign3460_e2757;
        locals.var_csb_d_dn0 = assign3460_e2757_d_n0;
        locals.var_csb_d_dn3 = assign3460_e2757_d_n3;
        locals.var_csb_d_rv = 0.0;

        let (assign3470_e2774, assign3470_e2774_d_n0, assign3470_e2774_d_n3,) = {
    if (locals.var_guard32 != 0.0) {
        let assign3470_e2761: f64 = (locals.var_cjsw_t * locals.var_pd_i);
        let assign3470_e2763: f64 = (-p.p48);
        let assign3470_e2767: f64 = (locals.var_v_di_b / locals.var_pbsw_t);
        let assign3470_e2768: f64 = (1.0 + assign3470_e2767);
        let assign3470_e2769: f64 = (assign3470_e2768).ln();
        let assign3470_e2770: f64 = (assign3470_e2763 * assign3470_e2769);
        let assign3470_e2771: f64 = (assign3470_e2770).exp();
        let assign3470_e2772: f64 = (assign3470_e2761 * assign3470_e2771);
        (assign3470_e2772, (assign3470_e2761 * (assign3470_e2771 * (assign3470_e2763 * ((locals.var_v_di_b_dn0 / locals.var_pbsw_t) / assign3470_e2768)))), (assign3470_e2761 * (assign3470_e2771 * (assign3470_e2763 * ((locals.var_v_di_b_dn3 / locals.var_pbsw_t) / assign3470_e2768)))),)
    } else {
        (locals.var_cssw_d, locals.var_cssw_d_dn0, locals.var_cssw_d_dn3,)
    }
};
        locals.var_cssw_d = assign3470_e2774;
        locals.var_cssw_d_dn0 = assign3470_e2774_d_n0;
        locals.var_cssw_d_dn3 = assign3470_e2774_d_n3;
        locals.var_cssw_d_rv = 0.0;

        let (assign3480_e2791, assign3480_e2791_d_n0, assign3480_e2791_d_n3,) = {
    if (locals.var_guard32 != 0.0) {
        let assign3480_e2778: f64 = (locals.var_cjswg_t * locals.var_weff);
        let assign3480_e2780: f64 = (-p.p49);
        let assign3480_e2784: f64 = (locals.var_v_di_b / locals.var_pbswg_t);
        let assign3480_e2785: f64 = (1.0 + assign3480_e2784);
        let assign3480_e2786: f64 = (assign3480_e2785).ln();
        let assign3480_e2787: f64 = (assign3480_e2780 * assign3480_e2786);
        let assign3480_e2788: f64 = (assign3480_e2787).exp();
        let assign3480_e2789: f64 = (assign3480_e2778 * assign3480_e2788);
        (assign3480_e2789, (assign3480_e2778 * (assign3480_e2788 * (assign3480_e2780 * ((locals.var_v_di_b_dn0 / locals.var_pbswg_t) / assign3480_e2785)))), (assign3480_e2778 * (assign3480_e2788 * (assign3480_e2780 * ((locals.var_v_di_b_dn3 / locals.var_pbswg_t) / assign3480_e2785)))),)
    } else {
        (locals.var_csswg_d, locals.var_csswg_d_dn0, locals.var_csswg_d_dn3,)
    }
};
        locals.var_csswg_d = assign3480_e2791;
        locals.var_csswg_d_dn0 = assign3480_e2791_d_n0;
        locals.var_csswg_d_dn3 = assign3480_e2791_d_n3;
        locals.var_csswg_d_rv = 0.0;

        let (assign3490_e2806, assign3490_e2806_d_n0, assign3490_e2806_d_n3,) = {
    if (locals.var_guard32 == 0.0) {
        let assign3490_e2796: f64 = (locals.var_cj_t * locals.var_ad_i);
        let assign3490_e2800: f64 = (p.p47 * locals.var_v_di_b);
        let assign3490_e2802: f64 = (assign3490_e2800 / locals.var_pb_t);
        let assign3490_e2803: f64 = (1.0 - assign3490_e2802);
        let assign3490_e2804: f64 = (assign3490_e2796 * assign3490_e2803);
        (assign3490_e2804, (assign3490_e2796 * (-((p.p47 * locals.var_v_di_b_dn0) / locals.var_pb_t))), (assign3490_e2796 * (-((p.p47 * locals.var_v_di_b_dn3) / locals.var_pb_t))),)
    } else {
        (locals.var_csb_d, locals.var_csb_d_dn0, locals.var_csb_d_dn3,)
    }
};
        locals.var_csb_d = assign3490_e2806;
        locals.var_csb_d_dn0 = assign3490_e2806_d_n0;
        locals.var_csb_d_dn3 = assign3490_e2806_d_n3;
        locals.var_csb_d_rv = 0.0;

        let (assign3500_e2821, assign3500_e2821_d_n0, assign3500_e2821_d_n3,) = {
    if (locals.var_guard32 == 0.0) {
        let assign3500_e2811: f64 = (locals.var_cjsw_t * locals.var_pd_i);
        let assign3500_e2815: f64 = (p.p48 * locals.var_v_di_b);
        let assign3500_e2817: f64 = (assign3500_e2815 / locals.var_pbsw_t);
        let assign3500_e2818: f64 = (1.0 - assign3500_e2817);
        let assign3500_e2819: f64 = (assign3500_e2811 * assign3500_e2818);
        (assign3500_e2819, (assign3500_e2811 * (-((p.p48 * locals.var_v_di_b_dn0) / locals.var_pbsw_t))), (assign3500_e2811 * (-((p.p48 * locals.var_v_di_b_dn3) / locals.var_pbsw_t))),)
    } else {
        (locals.var_cssw_d, locals.var_cssw_d_dn0, locals.var_cssw_d_dn3,)
    }
};
        locals.var_cssw_d = assign3500_e2821;
        locals.var_cssw_d_dn0 = assign3500_e2821_d_n0;
        locals.var_cssw_d_dn3 = assign3500_e2821_d_n3;
        locals.var_cssw_d_rv = 0.0;

        let (assign3510_e2836, assign3510_e2836_d_n0, assign3510_e2836_d_n3,) = {
    if (locals.var_guard32 == 0.0) {
        let assign3510_e2826: f64 = (locals.var_cjswg_t * locals.var_weff);
        let assign3510_e2830: f64 = (p.p49 * locals.var_v_di_b);
        let assign3510_e2832: f64 = (assign3510_e2830 / locals.var_pbswg_t);
        let assign3510_e2833: f64 = (1.0 - assign3510_e2832);
        let assign3510_e2834: f64 = (assign3510_e2826 * assign3510_e2833);
        (assign3510_e2834, (assign3510_e2826 * (-((p.p49 * locals.var_v_di_b_dn0) / locals.var_pbswg_t))), (assign3510_e2826 * (-((p.p49 * locals.var_v_di_b_dn3) / locals.var_pbswg_t))),)
    } else {
        (locals.var_csswg_d, locals.var_csswg_d_dn0, locals.var_csswg_d_dn3,)
    }
};
        locals.var_csswg_d = assign3510_e2836;
        locals.var_csswg_d_dn0 = assign3510_e2836_d_n0;
        locals.var_csswg_d_dn3 = assign3510_e2836_d_n3;
        locals.var_csswg_d_rv = 0.0;

        let assign3520_e2839: f64 = (locals.var_csb_d + locals.var_cssw_d);
        let assign3520_e2841: f64 = (assign3520_e2839 + locals.var_csswg_d);
        let assign3520_e2843: f64 = (assign3520_e2841 * locals.var_v_di_b);
        locals.var_qjd = assign3520_e2843;
        locals.var_qjd_dn0 = ((((locals.var_csb_d_dn0 + locals.var_cssw_d_dn0) + locals.var_csswg_d_dn0) * locals.var_v_di_b) + (assign3520_e2841 * locals.var_v_di_b_dn0));
        locals.var_qjd_dn3 = ((((locals.var_csb_d_dn3 + locals.var_cssw_d_dn3) + locals.var_csswg_d_dn3) * locals.var_v_di_b) + (assign3520_e2841 * locals.var_v_di_b_dn3));
        locals.var_qjd_rv = 0.0;

        let assign3530_e2846: f64 = if locals.var_v_si_b > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign3530_e2846;
        locals.var_guard33_rv = 0.0;

        let (assign3540_e2863, assign3540_e2863_d_n2, assign3540_e2863_d_n3,) = {
    if (locals.var_guard33 != 0.0) {
        let assign3540_e2850: f64 = (locals.var_cj_t * locals.var_as_i);
        let assign3540_e2852: f64 = (-p.p47);
        let assign3540_e2856: f64 = (locals.var_v_si_b / locals.var_pb_t);
        let assign3540_e2857: f64 = (1.0 + assign3540_e2856);
        let assign3540_e2858: f64 = (assign3540_e2857).ln();
        let assign3540_e2859: f64 = (assign3540_e2852 * assign3540_e2858);
        let assign3540_e2860: f64 = (assign3540_e2859).exp();
        let assign3540_e2861: f64 = (assign3540_e2850 * assign3540_e2860);
        (assign3540_e2861, (assign3540_e2850 * (assign3540_e2860 * (assign3540_e2852 * ((locals.var_v_si_b_dn2 / locals.var_pb_t) / assign3540_e2857)))), (assign3540_e2850 * (assign3540_e2860 * (assign3540_e2852 * ((locals.var_v_si_b_dn3 / locals.var_pb_t) / assign3540_e2857)))),)
    } else {
        (locals.var_csb_s, locals.var_csb_s_dn2, locals.var_csb_s_dn3,)
    }
};
        locals.var_csb_s = assign3540_e2863;
        locals.var_csb_s_dn2 = assign3540_e2863_d_n2;
        locals.var_csb_s_dn3 = assign3540_e2863_d_n3;
        locals.var_csb_s_rv = 0.0;

        let (assign3550_e2880, assign3550_e2880_d_n2, assign3550_e2880_d_n3,) = {
    if (locals.var_guard33 != 0.0) {
        let assign3550_e2867: f64 = (locals.var_cjsw_t * locals.var_ps_i);
        let assign3550_e2869: f64 = (-p.p48);
        let assign3550_e2873: f64 = (locals.var_v_si_b / locals.var_pbsw_t);
        let assign3550_e2874: f64 = (1.0 + assign3550_e2873);
        let assign3550_e2875: f64 = (assign3550_e2874).ln();
        let assign3550_e2876: f64 = (assign3550_e2869 * assign3550_e2875);
        let assign3550_e2877: f64 = (assign3550_e2876).exp();
        let assign3550_e2878: f64 = (assign3550_e2867 * assign3550_e2877);
        (assign3550_e2878, (assign3550_e2867 * (assign3550_e2877 * (assign3550_e2869 * ((locals.var_v_si_b_dn2 / locals.var_pbsw_t) / assign3550_e2874)))), (assign3550_e2867 * (assign3550_e2877 * (assign3550_e2869 * ((locals.var_v_si_b_dn3 / locals.var_pbsw_t) / assign3550_e2874)))),)
    } else {
        (locals.var_cssw_s, locals.var_cssw_s_dn2, locals.var_cssw_s_dn3,)
    }
};
        locals.var_cssw_s = assign3550_e2880;
        locals.var_cssw_s_dn2 = assign3550_e2880_d_n2;
        locals.var_cssw_s_dn3 = assign3550_e2880_d_n3;
        locals.var_cssw_s_rv = 0.0;

        let (assign3560_e2897, assign3560_e2897_d_n2, assign3560_e2897_d_n3,) = {
    if (locals.var_guard33 != 0.0) {
        let assign3560_e2884: f64 = (locals.var_cjswg_t * locals.var_weff);
        let assign3560_e2886: f64 = (-p.p49);
        let assign3560_e2890: f64 = (locals.var_v_si_b / locals.var_pbswg_t);
        let assign3560_e2891: f64 = (1.0 + assign3560_e2890);
        let assign3560_e2892: f64 = (assign3560_e2891).ln();
        let assign3560_e2893: f64 = (assign3560_e2886 * assign3560_e2892);
        let assign3560_e2894: f64 = (assign3560_e2893).exp();
        let assign3560_e2895: f64 = (assign3560_e2884 * assign3560_e2894);
        (assign3560_e2895, (assign3560_e2884 * (assign3560_e2894 * (assign3560_e2886 * ((locals.var_v_si_b_dn2 / locals.var_pbswg_t) / assign3560_e2891)))), (assign3560_e2884 * (assign3560_e2894 * (assign3560_e2886 * ((locals.var_v_si_b_dn3 / locals.var_pbswg_t) / assign3560_e2891)))),)
    } else {
        (locals.var_csswg_s, locals.var_csswg_s_dn2, locals.var_csswg_s_dn3,)
    }
};
        locals.var_csswg_s = assign3560_e2897;
        locals.var_csswg_s_dn2 = assign3560_e2897_d_n2;
        locals.var_csswg_s_dn3 = assign3560_e2897_d_n3;
        locals.var_csswg_s_rv = 0.0;

        let (assign3570_e2912, assign3570_e2912_d_n2, assign3570_e2912_d_n3,) = {
    if (locals.var_guard33 == 0.0) {
        let assign3570_e2902: f64 = (locals.var_cj_t * locals.var_as_i);
        let assign3570_e2906: f64 = (p.p47 * locals.var_v_si_b);
        let assign3570_e2908: f64 = (assign3570_e2906 / locals.var_pb_t);
        let assign3570_e2909: f64 = (1.0 - assign3570_e2908);
        let assign3570_e2910: f64 = (assign3570_e2902 * assign3570_e2909);
        (assign3570_e2910, (assign3570_e2902 * (-((p.p47 * locals.var_v_si_b_dn2) / locals.var_pb_t))), (assign3570_e2902 * (-((p.p47 * locals.var_v_si_b_dn3) / locals.var_pb_t))),)
    } else {
        (locals.var_csb_s, locals.var_csb_s_dn2, locals.var_csb_s_dn3,)
    }
};
        locals.var_csb_s = assign3570_e2912;
        locals.var_csb_s_dn2 = assign3570_e2912_d_n2;
        locals.var_csb_s_dn3 = assign3570_e2912_d_n3;
        locals.var_csb_s_rv = 0.0;

        let (assign3580_e2927, assign3580_e2927_d_n2, assign3580_e2927_d_n3,) = {
    if (locals.var_guard33 == 0.0) {
        let assign3580_e2917: f64 = (locals.var_cjsw_t * locals.var_ps_i);
        let assign3580_e2921: f64 = (p.p48 * locals.var_v_si_b);
        let assign3580_e2923: f64 = (assign3580_e2921 / locals.var_pbsw_t);
        let assign3580_e2924: f64 = (1.0 - assign3580_e2923);
        let assign3580_e2925: f64 = (assign3580_e2917 * assign3580_e2924);
        (assign3580_e2925, (assign3580_e2917 * (-((p.p48 * locals.var_v_si_b_dn2) / locals.var_pbsw_t))), (assign3580_e2917 * (-((p.p48 * locals.var_v_si_b_dn3) / locals.var_pbsw_t))),)
    } else {
        (locals.var_cssw_s, locals.var_cssw_s_dn2, locals.var_cssw_s_dn3,)
    }
};
        locals.var_cssw_s = assign3580_e2927;
        locals.var_cssw_s_dn2 = assign3580_e2927_d_n2;
        locals.var_cssw_s_dn3 = assign3580_e2927_d_n3;
        locals.var_cssw_s_rv = 0.0;

        let (assign3590_e2942, assign3590_e2942_d_n2, assign3590_e2942_d_n3,) = {
    if (locals.var_guard33 == 0.0) {
        let assign3590_e2932: f64 = (locals.var_cjswg_t * locals.var_weff);
        let assign3590_e2936: f64 = (p.p49 * locals.var_v_si_b);
        let assign3590_e2938: f64 = (assign3590_e2936 / locals.var_pbswg_t);
        let assign3590_e2939: f64 = (1.0 - assign3590_e2938);
        let assign3590_e2940: f64 = (assign3590_e2932 * assign3590_e2939);
        (assign3590_e2940, (assign3590_e2932 * (-((p.p49 * locals.var_v_si_b_dn2) / locals.var_pbswg_t))), (assign3590_e2932 * (-((p.p49 * locals.var_v_si_b_dn3) / locals.var_pbswg_t))),)
    } else {
        (locals.var_csswg_s, locals.var_csswg_s_dn2, locals.var_csswg_s_dn3,)
    }
};
        locals.var_csswg_s = assign3590_e2942;
        locals.var_csswg_s_dn2 = assign3590_e2942_d_n2;
        locals.var_csswg_s_dn3 = assign3590_e2942_d_n3;
        locals.var_csswg_s_rv = 0.0;

        let assign3600_e2945: f64 = (locals.var_csb_s + locals.var_cssw_s);
        let assign3600_e2947: f64 = (assign3600_e2945 + locals.var_csswg_s);
        let assign3600_e2949: f64 = (assign3600_e2947 * locals.var_v_si_b);
        locals.var_qjs = assign3600_e2949;
        locals.var_qjs_dn2 = ((((locals.var_csb_s_dn2 + locals.var_cssw_s_dn2) + locals.var_csswg_s_dn2) * locals.var_v_si_b) + (assign3600_e2947 * locals.var_v_si_b_dn2));
        locals.var_qjs_dn3 = ((((locals.var_csb_s_dn3 + locals.var_cssw_s_dn3) + locals.var_csswg_s_dn3) * locals.var_v_si_b) + (assign3600_e2947 * locals.var_v_si_b_dn3));
        locals.var_qjs_rv = 0.0;

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
        let (eq1_e92, eq1_e92_d_n0, eq1_e92_d_n1, eq1_e92_d_n2, eq1_e92_d_n3,) = {
    if (locals.var_guard21 != 0.0) {
        let eq1_e90: f64 = (p.p0 * locals.var_ddt_qd);
        let eq1_e90_d_n0: f64 = (p.p0 * locals.var_ddt_qd_dn0);
        let eq1_e90_d_n1: f64 = (p.p0 * locals.var_ddt_qd_dn1);
        let eq1_e90_d_n2: f64 = (p.p0 * locals.var_ddt_qd_dn2);
        let eq1_e90_d_n3: f64 = (p.p0 * locals.var_ddt_qd_dn3);
        (eq1_e90, eq1_e90_d_n0, eq1_e90_d_n1, eq1_e90_d_n2, eq1_e90_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e92;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(3),
            multiplicity * (eq1_value),
            [0, 1, 2, 3],
            [multiplicity * (eq1_e92_d_n0), multiplicity * (eq1_e92_d_n1), multiplicity * (eq1_e92_d_n2), multiplicity * (eq1_e92_d_n3)],
            [],
            [],
            1.0,
        );
        let (eq2_e98, eq2_e98_d_n0, eq2_e98_d_n1, eq2_e98_d_n2, eq2_e98_d_n3,) = {
    if (locals.var_guard21 != 0.0) {
        let eq2_e96: f64 = (p.p0 * locals.var_ddt_qs);
        let eq2_e96_d_n0: f64 = (p.p0 * locals.var_ddt_qs_dn0);
        let eq2_e96_d_n1: f64 = (p.p0 * locals.var_ddt_qs_dn1);
        let eq2_e96_d_n2: f64 = (p.p0 * locals.var_ddt_qs_dn2);
        let eq2_e96_d_n3: f64 = (p.p0 * locals.var_ddt_qs_dn3);
        (eq2_e96, eq2_e96_d_n0, eq2_e96_d_n1, eq2_e96_d_n2, eq2_e96_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e98;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(2),
            Some(3),
            multiplicity * (eq2_value),
            [0, 1, 2, 3],
            [multiplicity * (eq2_e98_d_n0), multiplicity * (eq2_e98_d_n1), multiplicity * (eq2_e98_d_n2), multiplicity * (eq2_e98_d_n3)],
            [],
            [],
            1.0,
        );
        let (eq4_e111, eq4_e111_d_n0, eq4_e111_d_n1, eq4_e111_d_n2, eq4_e111_d_n3,) = {
    if (locals.var_guard21 == 0.0) {
        let eq4_e109: f64 = (p.p0 * locals.var_ddt_qd);
        let eq4_e109_d_n0: f64 = (p.p0 * locals.var_ddt_qd_dn0);
        let eq4_e109_d_n1: f64 = (p.p0 * locals.var_ddt_qd_dn1);
        let eq4_e109_d_n2: f64 = (p.p0 * locals.var_ddt_qd_dn2);
        let eq4_e109_d_n3: f64 = (p.p0 * locals.var_ddt_qd_dn3);
        (eq4_e109, eq4_e109_d_n0, eq4_e109_d_n1, eq4_e109_d_n2, eq4_e109_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e111;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(2),
            Some(3),
            multiplicity * (eq4_value),
            [0, 1, 2, 3],
            [multiplicity * (eq4_e111_d_n0), multiplicity * (eq4_e111_d_n1), multiplicity * (eq4_e111_d_n2), multiplicity * (eq4_e111_d_n3)],
            [],
            [],
            1.0,
        );
        let (eq5_e118, eq5_e118_d_n0, eq5_e118_d_n1, eq5_e118_d_n2, eq5_e118_d_n3,) = {
    if (locals.var_guard21 == 0.0) {
        let eq5_e116: f64 = (p.p0 * locals.var_ddt_qs);
        let eq5_e116_d_n0: f64 = (p.p0 * locals.var_ddt_qs_dn0);
        let eq5_e116_d_n1: f64 = (p.p0 * locals.var_ddt_qs_dn1);
        let eq5_e116_d_n2: f64 = (p.p0 * locals.var_ddt_qs_dn2);
        let eq5_e116_d_n3: f64 = (p.p0 * locals.var_ddt_qs_dn3);
        (eq5_e116, eq5_e116_d_n0, eq5_e116_d_n1, eq5_e116_d_n2, eq5_e116_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e118;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(3),
            multiplicity * (eq5_value),
            [0, 1, 2, 3],
            [multiplicity * (eq5_e118_d_n0), multiplicity * (eq5_e118_d_n1), multiplicity * (eq5_e118_d_n2), multiplicity * (eq5_e118_d_n3)],
            [],
            [],
            1.0,
        );
        let eq7_e128: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qg);
        let eq7_e129: f64 = (p.p0 * eq7_e128);
        let eq7_e129_d_n0: f64 = (p.p0 * (locals.var_qg_dn0 * ddt_scale));
        let eq7_e129_d_n1: f64 = (p.p0 * (locals.var_qg_dn1 * ddt_scale));
        let eq7_e129_d_n2: f64 = (p.p0 * (locals.var_qg_dn2 * ddt_scale));
        let eq7_e129_d_n3: f64 = (p.p0 * (locals.var_qg_dn3 * ddt_scale));
        let eq7_value: f64 = eq7_e129;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(3),
            multiplicity * (eq7_value),
            [0, 1, 2, 3],
            [multiplicity * (eq7_e129_d_n0), multiplicity * (eq7_e129_d_n1), multiplicity * (eq7_e129_d_n2), multiplicity * (eq7_e129_d_n3)],
            [],
            [],
            1.0,
        );
        let eq11_e178: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, locals.var_qjd);
        let eq11_e180: f64 = (eq11_e178 * p.p0);
        let eq11_e180_d_n0: f64 = ((locals.var_qjd_dn0 * ddt_scale) * p.p0);
        let eq11_e180_d_n3: f64 = ((locals.var_qjd_dn3 * ddt_scale) * p.p0);
        let eq11_e182: f64 = (eq11_e180 * p.p7);
        let eq11_e182_d_n0: f64 = (eq11_e180_d_n0 * p.p7);
        let eq11_e182_d_n3: f64 = (eq11_e180_d_n3 * p.p7);
        let eq11_value: f64 = eq11_e182;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(3),
            multiplicity * (eq11_value),
            0,
            multiplicity * (eq11_e182_d_n0),
            3,
            multiplicity * (eq11_e182_d_n3),
        );
        let eq12_e184: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, locals.var_qjs);
        let eq12_e186: f64 = (eq12_e184 * p.p0);
        let eq12_e186_d_n2: f64 = ((locals.var_qjs_dn2 * ddt_scale) * p.p0);
        let eq12_e186_d_n3: f64 = ((locals.var_qjs_dn3 * ddt_scale) * p.p0);
        let eq12_e188: f64 = (eq12_e186 * p.p7);
        let eq12_e188_d_n2: f64 = (eq12_e186_d_n2 * p.p7);
        let eq12_e188_d_n3: f64 = (eq12_e186_d_n3 * p.p7);
        let eq12_value: f64 = eq12_e188;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * (eq12_value),
            2,
            multiplicity * (eq12_e188_d_n2),
            3,
            multiplicity * (eq12_e188_d_n3),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let (eq1_e92, eq1_e92_d_n0, eq1_e92_d_n1, eq1_e92_d_n2, eq1_e92_d_n3, eq1_e92_q, eq1_e92_q_d_n0, eq1_e92_q_d_n1, eq1_e92_q_d_n2, eq1_e92_q_d_n3,) = {
    if (locals.var_guard21 != 0.0) {
        let eq1_e89_q: f64 = locals.var_ddt_qd_rv;
        let eq1_e90: f64 = (p.p0 * locals.var_ddt_qd);
        let eq1_e90_d_n0: f64 = (p.p0 * locals.var_ddt_qd_dn0);
        let eq1_e90_d_n1: f64 = (p.p0 * locals.var_ddt_qd_dn1);
        let eq1_e90_d_n2: f64 = (p.p0 * locals.var_ddt_qd_dn2);
        let eq1_e90_d_n3: f64 = (p.p0 * locals.var_ddt_qd_dn3);
        let eq1_e90_q: f64 = (p.p0 * eq1_e89_q);
        let eq1_e90_q_d_n0: f64 = (p.p0 * locals.var_ddt_qd_rdn0);
        let eq1_e90_q_d_n1: f64 = (p.p0 * locals.var_ddt_qd_rdn1);
        let eq1_e90_q_d_n2: f64 = (p.p0 * locals.var_ddt_qd_rdn2);
        let eq1_e90_q_d_n3: f64 = (p.p0 * locals.var_ddt_qd_rdn3);
        (eq1_e90, eq1_e90_d_n0, eq1_e90_d_n1, eq1_e90_d_n2, eq1_e90_d_n3, eq1_e90_q, eq1_e90_q_d_n0, eq1_e90_q_d_n1, eq1_e90_q_d_n2, eq1_e90_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[0]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq1_e92_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq1_e92_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq1_e92_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq1_e92_q_d_n3)),
            ],
        );
        let (eq2_e98, eq2_e98_d_n0, eq2_e98_d_n1, eq2_e98_d_n2, eq2_e98_d_n3, eq2_e98_q, eq2_e98_q_d_n0, eq2_e98_q_d_n1, eq2_e98_q_d_n2, eq2_e98_q_d_n3,) = {
    if (locals.var_guard21 != 0.0) {
        let eq2_e95_q: f64 = locals.var_ddt_qs_rv;
        let eq2_e96: f64 = (p.p0 * locals.var_ddt_qs);
        let eq2_e96_d_n0: f64 = (p.p0 * locals.var_ddt_qs_dn0);
        let eq2_e96_d_n1: f64 = (p.p0 * locals.var_ddt_qs_dn1);
        let eq2_e96_d_n2: f64 = (p.p0 * locals.var_ddt_qs_dn2);
        let eq2_e96_d_n3: f64 = (p.p0 * locals.var_ddt_qs_dn3);
        let eq2_e96_q: f64 = (p.p0 * eq2_e95_q);
        let eq2_e96_q_d_n0: f64 = (p.p0 * locals.var_ddt_qs_rdn0);
        let eq2_e96_q_d_n1: f64 = (p.p0 * locals.var_ddt_qs_rdn1);
        let eq2_e96_q_d_n2: f64 = (p.p0 * locals.var_ddt_qs_rdn2);
        let eq2_e96_q_d_n3: f64 = (p.p0 * locals.var_ddt_qs_rdn3);
        (eq2_e96, eq2_e96_d_n0, eq2_e96_d_n1, eq2_e96_d_n2, eq2_e96_d_n3, eq2_e96_q, eq2_e96_q_d_n0, eq2_e96_q_d_n1, eq2_e96_q_d_n2, eq2_e96_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[2]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq2_e98_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq2_e98_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq2_e98_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq2_e98_q_d_n3)),
            ],
        );
        let (eq4_e111, eq4_e111_d_n0, eq4_e111_d_n1, eq4_e111_d_n2, eq4_e111_d_n3, eq4_e111_q, eq4_e111_q_d_n0, eq4_e111_q_d_n1, eq4_e111_q_d_n2, eq4_e111_q_d_n3,) = {
    if (locals.var_guard21 == 0.0) {
        let eq4_e108_q: f64 = locals.var_ddt_qd_rv;
        let eq4_e109: f64 = (p.p0 * locals.var_ddt_qd);
        let eq4_e109_d_n0: f64 = (p.p0 * locals.var_ddt_qd_dn0);
        let eq4_e109_d_n1: f64 = (p.p0 * locals.var_ddt_qd_dn1);
        let eq4_e109_d_n2: f64 = (p.p0 * locals.var_ddt_qd_dn2);
        let eq4_e109_d_n3: f64 = (p.p0 * locals.var_ddt_qd_dn3);
        let eq4_e109_q: f64 = (p.p0 * eq4_e108_q);
        let eq4_e109_q_d_n0: f64 = (p.p0 * locals.var_ddt_qd_rdn0);
        let eq4_e109_q_d_n1: f64 = (p.p0 * locals.var_ddt_qd_rdn1);
        let eq4_e109_q_d_n2: f64 = (p.p0 * locals.var_ddt_qd_rdn2);
        let eq4_e109_q_d_n3: f64 = (p.p0 * locals.var_ddt_qd_rdn3);
        (eq4_e109, eq4_e109_d_n0, eq4_e109_d_n1, eq4_e109_d_n2, eq4_e109_d_n3, eq4_e109_q, eq4_e109_q_d_n0, eq4_e109_q_d_n1, eq4_e109_q_d_n2, eq4_e109_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[2]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq4_e111_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq4_e111_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq4_e111_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq4_e111_q_d_n3)),
            ],
        );
        let (eq5_e118, eq5_e118_d_n0, eq5_e118_d_n1, eq5_e118_d_n2, eq5_e118_d_n3, eq5_e118_q, eq5_e118_q_d_n0, eq5_e118_q_d_n1, eq5_e118_q_d_n2, eq5_e118_q_d_n3,) = {
    if (locals.var_guard21 == 0.0) {
        let eq5_e115_q: f64 = locals.var_ddt_qs_rv;
        let eq5_e116: f64 = (p.p0 * locals.var_ddt_qs);
        let eq5_e116_d_n0: f64 = (p.p0 * locals.var_ddt_qs_dn0);
        let eq5_e116_d_n1: f64 = (p.p0 * locals.var_ddt_qs_dn1);
        let eq5_e116_d_n2: f64 = (p.p0 * locals.var_ddt_qs_dn2);
        let eq5_e116_d_n3: f64 = (p.p0 * locals.var_ddt_qs_dn3);
        let eq5_e116_q: f64 = (p.p0 * eq5_e115_q);
        let eq5_e116_q_d_n0: f64 = (p.p0 * locals.var_ddt_qs_rdn0);
        let eq5_e116_q_d_n1: f64 = (p.p0 * locals.var_ddt_qs_rdn1);
        let eq5_e116_q_d_n2: f64 = (p.p0 * locals.var_ddt_qs_rdn2);
        let eq5_e116_q_d_n3: f64 = (p.p0 * locals.var_ddt_qs_rdn3);
        (eq5_e116, eq5_e116_d_n0, eq5_e116_d_n1, eq5_e116_d_n2, eq5_e116_d_n3, eq5_e116_q, eq5_e116_q_d_n0, eq5_e116_q_d_n1, eq5_e116_q_d_n2, eq5_e116_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[0]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq5_e118_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq5_e118_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq5_e118_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq5_e118_q_d_n3)),
            ],
        );
        let eq7_e128_q: f64 = locals.var_qg;
        let eq7_e129: f64 = (p.p0 * locals.var_qg);
        let eq7_e129_d_n0: f64 = (p.p0 * locals.var_qg_dn0);
        let eq7_e129_d_n1: f64 = (p.p0 * locals.var_qg_dn1);
        let eq7_e129_d_n2: f64 = (p.p0 * locals.var_qg_dn2);
        let eq7_e129_d_n3: f64 = (p.p0 * locals.var_qg_dn3);
        let eq7_e129_q: f64 = (p.p0 * eq7_e128_q);
        stamper.stamp_current_reactive(
            Some(nodes[1]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq7_e129_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq7_e129_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq7_e129_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq7_e129_d_n3)),
            ],
        );
        let eq11_e178_q: f64 = locals.var_qjd;
        let eq11_e180: f64 = (locals.var_qjd * p.p0);
        let eq11_e180_d_n0: f64 = (locals.var_qjd_dn0 * p.p0);
        let eq11_e180_d_n3: f64 = (locals.var_qjd_dn3 * p.p0);
        let eq11_e180_q: f64 = (eq11_e178_q * p.p0);
        let eq11_e182: f64 = (eq11_e180 * p.p7);
        let eq11_e182_d_n0: f64 = (eq11_e180_d_n0 * p.p7);
        let eq11_e182_d_n3: f64 = (eq11_e180_d_n3 * p.p7);
        let eq11_e182_q: f64 = (eq11_e180_q * p.p7);
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[3]),
            nodes[0],
            multiplicity * (eq11_e182_d_n0),
            nodes[3],
            multiplicity * (eq11_e182_d_n3),
        );
        let eq12_e184_q: f64 = locals.var_qjs;
        let eq12_e186: f64 = (locals.var_qjs * p.p0);
        let eq12_e186_d_n2: f64 = (locals.var_qjs_dn2 * p.p0);
        let eq12_e186_d_n3: f64 = (locals.var_qjs_dn3 * p.p0);
        let eq12_e186_q: f64 = (eq12_e184_q * p.p0);
        let eq12_e188: f64 = (eq12_e186 * p.p7);
        let eq12_e188_d_n2: f64 = (eq12_e186_d_n2 * p.p7);
        let eq12_e188_d_n3: f64 = (eq12_e186_d_n3 * p.p7);
        let eq12_e188_q: f64 = (eq12_e186_q * p.p7);
        stamper.stamp_current_reactive_node2(
            Some(nodes[2]),
            Some(nodes[3]),
            nodes[2],
            multiplicity * (eq12_e188_d_n2),
            nodes[3],
            multiplicity * (eq12_e188_d_n3),
        );
    }
}
