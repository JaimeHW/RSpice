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
        let nv4 = ctx.node_voltage(nodes[4]);
        let assign00_e2186: f64 = 0.0;
        locals.var_gmin = assign00_e2186;

        let assign10_e2189: f64 = (p.p5 + 273.15);
        locals.var_tnomk = assign10_e2189;

        let assign20_e2190: f64 = ctx_temp;
        locals.var_tambk = assign20_e2190;

        locals.var_tsh = (nv4 - 0.0);
        locals.var_tsh_dn4 = 1.0;

        let assign50_e2198: f64 = (locals.var_tambk + p.p3);
        let assign50_e2200: f64 = (assign50_e2198 + locals.var_tsh);
        locals.var_tdut = assign50_e2200;
        locals.var_tdut_dn4 = locals.var_tsh_dn4;

        let assign60_e2203: f64 = (-270.0);
        let assign60_e2205: f64 = (assign60_e2203 + 273.15);
        let assign60_e2206: f64 = if locals.var_tdut < assign60_e2205 { 1.0 } else { 0.0 };
        locals.var_guard2 = assign60_e2206;

        let (assign70_e2213, assign70_e2213_d_n4,) = {
    if (locals.var_guard2 != 0.0) {
        let assign70_e2209: f64 = (-270.0);
        let assign70_e2211: f64 = (assign70_e2209 + 273.15);
        (assign70_e2211, 0.0,)
    } else {
        (locals.var_tdut, locals.var_tdut_dn4,)
    }
};
        locals.var_tdut = assign70_e2213;
        locals.var_tdut_dn4 = assign70_e2213_d_n4;

        let assign80_e2217: f64 = (1500.0 + 273.15);
        let assign80_e2218: f64 = if locals.var_tdut > assign80_e2217 { 1.0 } else { 0.0 };
        locals.var_guard3 = assign80_e2218;

        let (assign90_e2227, assign90_e2227_d_n4,) = {
    if ((locals.var_guard2 == 0.0) && (locals.var_guard3 != 0.0)) {
        let assign90_e2225: f64 = (1500.0 + 273.15);
        (assign90_e2225, 0.0,)
    } else {
        (locals.var_tdut, locals.var_tdut_dn4,)
    }
};
        locals.var_tdut = assign90_e2227;
        locals.var_tdut_dn4 = assign90_e2227_d_n4;

        locals.var_rsi = 0.0;
        locals.var_rsi_dn4 = 0.0;

        locals.var_rdi = 0.0;
        locals.var_rdi_dn4 = 0.0;

        let assign120_e2232: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard4 = assign120_e2232;

        let (assign130_e2240,) = {
    if (locals.var_guard4 != 0.0) {
        let assign130_e2236: f64 = (p.p30 / p.p0);
        let assign130_e2238: f64 = (assign130_e2236 / p.p2);
        (assign130_e2238,)
    } else {
        (locals.var_rcs_w,)
    }
};
        locals.var_rcs_w = assign130_e2240;

        let (assign140_e2248,) = {
    if (locals.var_guard4 != 0.0) {
        let assign140_e2244: f64 = (p.p31 / p.p0);
        let assign140_e2246: f64 = (assign140_e2244 / p.p2);
        (assign140_e2246,)
    } else {
        (locals.var_rcd_w,)
    }
};
        locals.var_rcd_w = assign140_e2248;

        let (assign150_e2263,) = {
    if (locals.var_guard4 == 0.0) {
        let assign150_e2253: f64 = (p.p30 / p.p0);
        let assign150_e2256: f64 = (p.p29 * p.p54);
        let assign150_e2258: f64 = (assign150_e2256 / p.p0);
        let assign150_e2259: f64 = (assign150_e2253 + assign150_e2258);
        let assign150_e2261: f64 = (assign150_e2259 / p.p2);
        (assign150_e2261,)
    } else {
        (locals.var_rcs_w,)
    }
};
        locals.var_rcs_w = assign150_e2263;

        let (assign160_e2278,) = {
    if (locals.var_guard4 == 0.0) {
        let assign160_e2268: f64 = (p.p31 / p.p0);
        let assign160_e2271: f64 = (p.p29 * p.p66);
        let assign160_e2273: f64 = (assign160_e2271 / p.p0);
        let assign160_e2274: f64 = (assign160_e2268 + assign160_e2273);
        let assign160_e2276: f64 = (assign160_e2274 / p.p2);
        (assign160_e2276,)
    } else {
        (locals.var_rcd_w,)
    }
};
        locals.var_rcd_w = assign160_e2278;

        let assign170_e2285: f64 = if ((locals.var_rcs_w >= p.p353) && (locals.var_rcs_w > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard5 = assign170_e2285;

        let (assign180_e2307, assign180_e2307_d_n4,) = {
    if (locals.var_guard5 != 0.0) {
        let assign180_e2292: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign180_e2293: f64 = (p.p48 * assign180_e2292);
        let assign180_e2294: f64 = (1.0 + assign180_e2293);
        let assign180_e2298: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign180_e2299: f64 = (p.p49 * assign180_e2298);
        let assign180_e2302: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign180_e2303: f64 = (assign180_e2299 * assign180_e2302);
        let assign180_e2304: f64 = (assign180_e2294 + assign180_e2303);
        let assign180_e2305: f64 = (locals.var_rcs_w * assign180_e2304);
        (assign180_e2305, (locals.var_rcs_w * ((p.p48 * locals.var_tdut_dn4) + (((p.p49 * locals.var_tdut_dn4) * assign180_e2302) + (assign180_e2299 * locals.var_tdut_dn4)))),)
    } else {
        (locals.var_rsi, locals.var_rsi_dn4,)
    }
};
        locals.var_rsi = assign180_e2307;
        locals.var_rsi_dn4 = assign180_e2307_d_n4;

        let assign190_e2311: f64 = (0.1 * locals.var_rcs_w);
        let assign190_e2312: f64 = if locals.var_rsi < assign190_e2311 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign190_e2312;

        let (assign200_e2320, assign200_e2320_d_n4,) = {
    if ((locals.var_guard5 != 0.0) && (locals.var_guard6 != 0.0)) {
        let assign200_e2318: f64 = (0.1 * locals.var_rcs_w);
        (assign200_e2318, 0.0,)
    } else {
        (locals.var_rsi, locals.var_rsi_dn4,)
    }
};
        locals.var_rsi = assign200_e2320;
        locals.var_rsi_dn4 = assign200_e2320_d_n4;

        let (assign210_e2325, assign210_e2325_d_n4,) = {
    if (locals.var_guard5 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_rsi, locals.var_rsi_dn4,)
    }
};
        locals.var_rsi = assign210_e2325;
        locals.var_rsi_dn4 = assign210_e2325_d_n4;

        let assign220_e2332: f64 = if ((locals.var_rcd_w >= p.p353) && (locals.var_rcd_w > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard7 = assign220_e2332;

        let (assign230_e2354, assign230_e2354_d_n4,) = {
    if (locals.var_guard7 != 0.0) {
        let assign230_e2339: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign230_e2340: f64 = (p.p48 * assign230_e2339);
        let assign230_e2341: f64 = (1.0 + assign230_e2340);
        let assign230_e2345: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign230_e2346: f64 = (p.p49 * assign230_e2345);
        let assign230_e2349: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign230_e2350: f64 = (assign230_e2346 * assign230_e2349);
        let assign230_e2351: f64 = (assign230_e2341 + assign230_e2350);
        let assign230_e2352: f64 = (locals.var_rcd_w * assign230_e2351);
        (assign230_e2352, (locals.var_rcd_w * ((p.p48 * locals.var_tdut_dn4) + (((p.p49 * locals.var_tdut_dn4) * assign230_e2349) + (assign230_e2346 * locals.var_tdut_dn4)))),)
    } else {
        (locals.var_rdi, locals.var_rdi_dn4,)
    }
};
        locals.var_rdi = assign230_e2354;
        locals.var_rdi_dn4 = assign230_e2354_d_n4;

        let assign240_e2358: f64 = (0.1 * locals.var_rcd_w);
        let assign240_e2359: f64 = if locals.var_rdi < assign240_e2358 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign240_e2359;

        let (assign250_e2367, assign250_e2367_d_n4,) = {
    if ((locals.var_guard7 != 0.0) && (locals.var_guard8 != 0.0)) {
        let assign250_e2365: f64 = (0.1 * locals.var_rcd_w);
        (assign250_e2365, 0.0,)
    } else {
        (locals.var_rdi, locals.var_rdi_dn4,)
    }
};
        locals.var_rdi = assign250_e2367;
        locals.var_rdi_dn4 = assign250_e2367_d_n4;

        let (assign260_e2372, assign260_e2372_d_n4,) = {
    if (locals.var_guard7 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_rdi, locals.var_rdi_dn4,)
    }
};
        locals.var_rdi = assign260_e2372;
        locals.var_rdi_dn4 = assign260_e2372_d_n4;

        let assign290_e2401: f64 = (1.38062e-23 * locals.var_tdut);
        let assign290_e2403: f64 = (assign290_e2401 / 1.60219e-19);
        locals.var_phit = assign290_e2403;
        locals.var_phit_dn4 = ((1.38062e-23 * locals.var_tdut_dn4) / 1.60219e-19);

        let assign300_e2408: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign300_e2409: f64 = (p.p336 * assign300_e2408);
        let assign300_e2410: f64 = (1.0 + assign300_e2409);
        locals.var_ttrapfac = assign300_e2410;
        locals.var_ttrapfac_dn4 = (p.p336 * locals.var_tdut_dn4);

        let assign310_e2413: f64 = if locals.var_ttrapfac < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard9 = assign310_e2413;

        let (assign320_e2417, assign320_e2417_d_n4,) = {
    if (locals.var_guard9 != 0.0) {
        (0.1, 0.0,)
    } else {
        (locals.var_ttrapfac, locals.var_ttrapfac_dn4,)
    }
};
        locals.var_ttrapfac = assign320_e2417;
        locals.var_ttrapfac_dn4 = assign320_e2417_d_n4;

        let assign330_e2420: f64 = (locals.var_tdut / locals.var_tnomk);
        let assign330_e2422: f64 = (assign330_e2420).powf(3.0);
        locals.var_tfacdiode = assign330_e2422;
        locals.var_tfacdiode_dn4 = if 0.0 == 0.0 && ((3.0) as f64).is_finite() && ((3.0) as f64).fract() == 0.0 { if 3.0 == 0.0 { 0.0 } else { (3.0 * ((assign330_e2420).powf(3.0 - 1.0) * (locals.var_tdut_dn4 / locals.var_tnomk))) } } else { (assign330_e2422 * (3.0 * ((locals.var_tdut_dn4 / locals.var_tnomk) / assign330_e2420))) };

        let assign340_e2428: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign340_e2429: f64 = (p.p21 * assign340_e2428);
        let assign340_e2430: f64 = (1.0 + assign340_e2429);
        let (assign340_e2441, assign340_e2441_d_n4,) = {
    if (assign340_e2430 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign340_e2438: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign340_e2439: f64 = (p.p21 * assign340_e2438);
        let assign340_e2440: f64 = (1.0 + assign340_e2439);
        (assign340_e2440, (p.p21 * locals.var_tdut_dn4),)
    }
};
        let assign340_e2442: f64 = (p.p9 * assign340_e2441);
        locals.var_cofsmt = assign340_e2442;
        locals.var_cofsmt_dn4 = (p.p9 * assign340_e2441_d_n4);

        let assign350_e2448: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign350_e2449: f64 = (p.p22 * assign350_e2448);
        let assign350_e2450: f64 = (1.0 + assign350_e2449);
        let (assign350_e2461, assign350_e2461_d_n4,) = {
    if (assign350_e2450 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign350_e2458: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign350_e2459: f64 = (p.p22 * assign350_e2458);
        let assign350_e2460: f64 = (1.0 + assign350_e2459);
        (assign350_e2460, (p.p22 * locals.var_tdut_dn4),)
    }
};
        let assign350_e2462: f64 = (p.p10 * assign350_e2461);
        locals.var_cofdmt = assign350_e2462;
        locals.var_cofdmt_dn4 = (p.p10 * assign350_e2461_d_n4);

        let assign360_e2468: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign360_e2469: f64 = (p.p23 * assign360_e2468);
        let assign360_e2470: f64 = (1.0 + assign360_e2469);
        let (assign360_e2481, assign360_e2481_d_n4,) = {
    if (assign360_e2470 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign360_e2478: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign360_e2479: f64 = (p.p23 * assign360_e2478);
        let assign360_e2480: f64 = (1.0 + assign360_e2479);
        (assign360_e2480, (p.p23 * locals.var_tdut_dn4),)
    }
};
        let assign360_e2482: f64 = (p.p11 * assign360_e2481);
        locals.var_cofdsmt = assign360_e2482;
        locals.var_cofdsmt_dn4 = (p.p11 * assign360_e2481_d_n4);

        let assign370_e2488: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign370_e2489: f64 = (p.p24 * assign370_e2488);
        let assign370_e2490: f64 = (1.0 + assign370_e2489);
        let (assign370_e2501, assign370_e2501_d_n4,) = {
    if (assign370_e2490 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign370_e2498: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign370_e2499: f64 = (p.p24 * assign370_e2498);
        let assign370_e2500: f64 = (1.0 + assign370_e2499);
        (assign370_e2500, (p.p24 * locals.var_tdut_dn4),)
    }
};
        let assign370_e2502: f64 = (p.p13 * assign370_e2501);
        locals.var_cofssubmt = assign370_e2502;
        locals.var_cofssubmt_dn4 = (p.p13 * assign370_e2501_d_n4);

        let assign380_e2508: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign380_e2509: f64 = (p.p25 * assign380_e2508);
        let assign380_e2510: f64 = (1.0 + assign380_e2509);
        let (assign380_e2521, assign380_e2521_d_n4,) = {
    if (assign380_e2510 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign380_e2518: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign380_e2519: f64 = (p.p25 * assign380_e2518);
        let assign380_e2520: f64 = (1.0 + assign380_e2519);
        (assign380_e2520, (p.p25 * locals.var_tdut_dn4),)
    }
};
        let assign380_e2522: f64 = (p.p12 * assign380_e2521);
        locals.var_cofdsubmt = assign380_e2522;
        locals.var_cofdsubmt_dn4 = (p.p12 * assign380_e2521_d_n4);

        let assign390_e2528: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign390_e2529: f64 = (p.p26 * assign390_e2528);
        let assign390_e2530: f64 = (1.0 + assign390_e2529);
        let (assign390_e2541, assign390_e2541_d_n4,) = {
    if (assign390_e2530 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign390_e2538: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign390_e2539: f64 = (p.p26 * assign390_e2538);
        let assign390_e2540: f64 = (1.0 + assign390_e2539);
        (assign390_e2540, (p.p26 * locals.var_tdut_dn4),)
    }
};
        let assign390_e2542: f64 = (p.p14 * assign390_e2541);
        locals.var_cofgsubmt = assign390_e2542;
        locals.var_cofgsubmt_dn4 = (p.p14 * assign390_e2541_d_n4);

        let assign400_e2548: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign400_e2549: f64 = (p.p21 * assign400_e2548);
        let assign400_e2550: f64 = (1.0 + assign400_e2549);
        let (assign400_e2561, assign400_e2561_d_n4,) = {
    if (assign400_e2550 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign400_e2558: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign400_e2559: f64 = (p.p21 * assign400_e2558);
        let assign400_e2560: f64 = (1.0 + assign400_e2559);
        (assign400_e2560, (p.p21 * locals.var_tdut_dn4),)
    }
};
        let assign400_e2562: f64 = (p.p15 * assign400_e2561);
        locals.var_cofsmt0 = assign400_e2562;
        locals.var_cofsmt0_dn4 = (p.p15 * assign400_e2561_d_n4);

        let assign410_e2568: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign410_e2569: f64 = (p.p22 * assign410_e2568);
        let assign410_e2570: f64 = (1.0 + assign410_e2569);
        let (assign410_e2581, assign410_e2581_d_n4,) = {
    if (assign410_e2570 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign410_e2578: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign410_e2579: f64 = (p.p22 * assign410_e2578);
        let assign410_e2580: f64 = (1.0 + assign410_e2579);
        (assign410_e2580, (p.p22 * locals.var_tdut_dn4),)
    }
};
        let assign410_e2582: f64 = (p.p16 * assign410_e2581);
        locals.var_cofdmt0 = assign410_e2582;
        locals.var_cofdmt0_dn4 = (p.p16 * assign410_e2581_d_n4);

        let assign420_e2588: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign420_e2589: f64 = (p.p23 * assign420_e2588);
        let assign420_e2590: f64 = (1.0 + assign420_e2589);
        let (assign420_e2601, assign420_e2601_d_n4,) = {
    if (assign420_e2590 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign420_e2598: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign420_e2599: f64 = (p.p23 * assign420_e2598);
        let assign420_e2600: f64 = (1.0 + assign420_e2599);
        (assign420_e2600, (p.p23 * locals.var_tdut_dn4),)
    }
};
        let assign420_e2602: f64 = (p.p17 * assign420_e2601);
        locals.var_cofdsmt0 = assign420_e2602;
        locals.var_cofdsmt0_dn4 = (p.p17 * assign420_e2601_d_n4);

        let assign430_e2608: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign430_e2609: f64 = (p.p24 * assign430_e2608);
        let assign430_e2610: f64 = (1.0 + assign430_e2609);
        let (assign430_e2621, assign430_e2621_d_n4,) = {
    if (assign430_e2610 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign430_e2618: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign430_e2619: f64 = (p.p24 * assign430_e2618);
        let assign430_e2620: f64 = (1.0 + assign430_e2619);
        (assign430_e2620, (p.p24 * locals.var_tdut_dn4),)
    }
};
        let assign430_e2622: f64 = (p.p19 * assign430_e2621);
        locals.var_cofssubmt0 = assign430_e2622;
        locals.var_cofssubmt0_dn4 = (p.p19 * assign430_e2621_d_n4);

        let assign440_e2628: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign440_e2629: f64 = (p.p25 * assign440_e2628);
        let assign440_e2630: f64 = (1.0 + assign440_e2629);
        let (assign440_e2641, assign440_e2641_d_n4,) = {
    if (assign440_e2630 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign440_e2638: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign440_e2639: f64 = (p.p25 * assign440_e2638);
        let assign440_e2640: f64 = (1.0 + assign440_e2639);
        (assign440_e2640, (p.p25 * locals.var_tdut_dn4),)
    }
};
        let assign440_e2642: f64 = (p.p18 * assign440_e2641);
        locals.var_cofdsubmt0 = assign440_e2642;
        locals.var_cofdsubmt0_dn4 = (p.p18 * assign440_e2641_d_n4);

        let assign450_e2648: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign450_e2649: f64 = (p.p26 * assign450_e2648);
        let assign450_e2650: f64 = (1.0 + assign450_e2649);
        let (assign450_e2661, assign450_e2661_d_n4,) = {
    if (assign450_e2650 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign450_e2658: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign450_e2659: f64 = (p.p26 * assign450_e2658);
        let assign450_e2660: f64 = (1.0 + assign450_e2659);
        (assign450_e2660, (p.p26 * locals.var_tdut_dn4),)
    }
};
        let assign450_e2662: f64 = (p.p20 * assign450_e2661);
        locals.var_cofgsubmt0 = assign450_e2662;
        locals.var_cofgsubmt0_dn4 = (p.p20 * assign450_e2661_d_n4);

        let assign460_e2668: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign460_e2669: f64 = (p.p8 * assign460_e2668);
        let assign460_e2670: f64 = (1.0 + assign460_e2669);
        let (assign460_e2681, assign460_e2681_d_n4,) = {
    if (assign460_e2670 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign460_e2678: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign460_e2679: f64 = (p.p8 * assign460_e2678);
        let assign460_e2680: f64 = (1.0 + assign460_e2679);
        (assign460_e2680, (p.p8 * locals.var_tdut_dn4),)
    }
};
        let assign460_e2682: f64 = (p.p7 * assign460_e2681);
        locals.var_cgt = assign460_e2682;
        locals.var_cgt_dn4 = (p.p7 * assign460_e2681_d_n4);

        let assign470_e2688: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign470_e2689: f64 = (p.p82 * assign470_e2688);
        let assign470_e2690: f64 = (1.0 + assign470_e2689);
        let (assign470_e2701, assign470_e2701_d_n4,) = {
    if (assign470_e2690 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign470_e2698: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign470_e2699: f64 = (p.p82 * assign470_e2698);
        let assign470_e2700: f64 = (1.0 + assign470_e2699);
        (assign470_e2700, (p.p82 * locals.var_tdut_dn4),)
    }
};
        let assign470_e2702: f64 = (p.p81 * assign470_e2701);
        locals.var_cgfps1t = assign470_e2702;
        locals.var_cgfps1t_dn4 = (p.p81 * assign470_e2701_d_n4);

        let assign480_e2708: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign480_e2709: f64 = (p.p104 * assign480_e2708);
        let assign480_e2710: f64 = (1.0 + assign480_e2709);
        let (assign480_e2721, assign480_e2721_d_n4,) = {
    if (assign480_e2710 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign480_e2718: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign480_e2719: f64 = (p.p104 * assign480_e2718);
        let assign480_e2720: f64 = (1.0 + assign480_e2719);
        (assign480_e2720, (p.p104 * locals.var_tdut_dn4),)
    }
};
        let assign480_e2722: f64 = (p.p103 * assign480_e2721);
        locals.var_cgfps2t = assign480_e2722;
        locals.var_cgfps2t_dn4 = (p.p103 * assign480_e2721_d_n4);

        let assign490_e2728: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign490_e2729: f64 = (p.p126 * assign490_e2728);
        let assign490_e2730: f64 = (1.0 + assign490_e2729);
        let (assign490_e2741, assign490_e2741_d_n4,) = {
    if (assign490_e2730 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign490_e2738: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign490_e2739: f64 = (p.p126 * assign490_e2738);
        let assign490_e2740: f64 = (1.0 + assign490_e2739);
        (assign490_e2740, (p.p126 * locals.var_tdut_dn4),)
    }
};
        let assign490_e2742: f64 = (p.p125 * assign490_e2741);
        locals.var_cgfps3t = assign490_e2742;
        locals.var_cgfps3t_dn4 = (p.p125 * assign490_e2741_d_n4);

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let assign500_e2748: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign500_e2749: f64 = (p.p148 * assign500_e2748);
        let assign500_e2750: f64 = (1.0 + assign500_e2749);
        let (assign500_e2761, assign500_e2761_d_n4,) = {
    if (assign500_e2750 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign500_e2758: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign500_e2759: f64 = (p.p148 * assign500_e2758);
        let assign500_e2760: f64 = (1.0 + assign500_e2759);
        (assign500_e2760, (p.p148 * locals.var_tdut_dn4),)
    }
};
        let assign500_e2762: f64 = (p.p147 * assign500_e2761);
        locals.var_cgfps4t = assign500_e2762;
        locals.var_cgfps4t_dn4 = (p.p147 * assign500_e2761_d_n4);

        let assign510_e2768: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign510_e2769: f64 = (p.p87 * assign510_e2768);
        let assign510_e2770: f64 = (1.0 + assign510_e2769);
        let (assign510_e2781, assign510_e2781_d_n4,) = {
    if (assign510_e2770 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign510_e2778: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign510_e2779: f64 = (p.p87 * assign510_e2778);
        let assign510_e2780: f64 = (1.0 + assign510_e2779);
        (assign510_e2780, (p.p87 * locals.var_tdut_dn4),)
    }
};
        let assign510_e2782: f64 = (p.p86 * assign510_e2781);
        locals.var_ccfps1t = assign510_e2782;
        locals.var_ccfps1t_dn4 = (p.p86 * assign510_e2781_d_n4);

        let assign520_e2788: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign520_e2789: f64 = (p.p109 * assign520_e2788);
        let assign520_e2790: f64 = (1.0 + assign520_e2789);
        let (assign520_e2801, assign520_e2801_d_n4,) = {
    if (assign520_e2790 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign520_e2798: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign520_e2799: f64 = (p.p109 * assign520_e2798);
        let assign520_e2800: f64 = (1.0 + assign520_e2799);
        (assign520_e2800, (p.p109 * locals.var_tdut_dn4),)
    }
};
        let assign520_e2802: f64 = (p.p108 * assign520_e2801);
        locals.var_ccfps2t = assign520_e2802;
        locals.var_ccfps2t_dn4 = (p.p108 * assign520_e2801_d_n4);

        let assign530_e2808: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign530_e2809: f64 = (p.p131 * assign530_e2808);
        let assign530_e2810: f64 = (1.0 + assign530_e2809);
        let (assign530_e2821, assign530_e2821_d_n4,) = {
    if (assign530_e2810 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign530_e2818: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign530_e2819: f64 = (p.p131 * assign530_e2818);
        let assign530_e2820: f64 = (1.0 + assign530_e2819);
        (assign530_e2820, (p.p131 * locals.var_tdut_dn4),)
    }
};
        let assign530_e2822: f64 = (p.p130 * assign530_e2821);
        locals.var_ccfps3t = assign530_e2822;
        locals.var_ccfps3t_dn4 = (p.p130 * assign530_e2821_d_n4);

        let assign540_e2828: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign540_e2829: f64 = (p.p153 * assign540_e2828);
        let assign540_e2830: f64 = (1.0 + assign540_e2829);
        let (assign540_e2841, assign540_e2841_d_n4,) = {
    if (assign540_e2830 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign540_e2838: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign540_e2839: f64 = (p.p153 * assign540_e2838);
        let assign540_e2840: f64 = (1.0 + assign540_e2839);
        (assign540_e2840, (p.p153 * locals.var_tdut_dn4),)
    }
};
        let assign540_e2842: f64 = (p.p152 * assign540_e2841);
        locals.var_ccfps4t = assign540_e2842;
        locals.var_ccfps4t_dn4 = (p.p152 * assign540_e2841_d_n4);

        let assign550_e2848: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign550_e2849: f64 = (p.p89 * assign550_e2848);
        let assign550_e2850: f64 = (1.0 + assign550_e2849);
        let (assign550_e2861, assign550_e2861_d_n4,) = {
    if (assign550_e2850 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign550_e2858: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign550_e2859: f64 = (p.p89 * assign550_e2858);
        let assign550_e2860: f64 = (1.0 + assign550_e2859);
        (assign550_e2860, (p.p89 * locals.var_tdut_dn4),)
    }
};
        let assign550_e2862: f64 = (p.p88 * assign550_e2861);
        locals.var_cbfps1t = assign550_e2862;
        locals.var_cbfps1t_dn4 = (p.p88 * assign550_e2861_d_n4);

        let assign560_e2868: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign560_e2869: f64 = (p.p111 * assign560_e2868);
        let assign560_e2870: f64 = (1.0 + assign560_e2869);
        let (assign560_e2881, assign560_e2881_d_n4,) = {
    if (assign560_e2870 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign560_e2878: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign560_e2879: f64 = (p.p111 * assign560_e2878);
        let assign560_e2880: f64 = (1.0 + assign560_e2879);
        (assign560_e2880, (p.p111 * locals.var_tdut_dn4),)
    }
};
        let assign560_e2882: f64 = (p.p110 * assign560_e2881);
        locals.var_cbfps2t = assign560_e2882;
        locals.var_cbfps2t_dn4 = (p.p110 * assign560_e2881_d_n4);

        let assign570_e2888: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign570_e2889: f64 = (p.p133 * assign570_e2888);
        let assign570_e2890: f64 = (1.0 + assign570_e2889);
        let (assign570_e2901, assign570_e2901_d_n4,) = {
    if (assign570_e2890 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign570_e2898: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign570_e2899: f64 = (p.p133 * assign570_e2898);
        let assign570_e2900: f64 = (1.0 + assign570_e2899);
        (assign570_e2900, (p.p133 * locals.var_tdut_dn4),)
    }
};
        let assign570_e2902: f64 = (p.p132 * assign570_e2901);
        locals.var_cbfps3t = assign570_e2902;
        locals.var_cbfps3t_dn4 = (p.p132 * assign570_e2901_d_n4);

        let assign580_e2908: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign580_e2909: f64 = (p.p155 * assign580_e2908);
        let assign580_e2910: f64 = (1.0 + assign580_e2909);
        let (assign580_e2921, assign580_e2921_d_n4,) = {
    if (assign580_e2910 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign580_e2918: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign580_e2919: f64 = (p.p155 * assign580_e2918);
        let assign580_e2920: f64 = (1.0 + assign580_e2919);
        (assign580_e2920, (p.p155 * locals.var_tdut_dn4),)
    }
};
        let assign580_e2922: f64 = (p.p154 * assign580_e2921);
        locals.var_cbfps4t = assign580_e2922;
        locals.var_cbfps4t_dn4 = (p.p154 * assign580_e2921_d_n4);

        let assign590_e2928: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign590_e2929: f64 = (p.p170 * assign590_e2928);
        let assign590_e2930: f64 = (1.0 + assign590_e2929);
        let (assign590_e2941, assign590_e2941_d_n4,) = {
    if (assign590_e2930 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign590_e2938: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign590_e2939: f64 = (p.p170 * assign590_e2938);
        let assign590_e2940: f64 = (1.0 + assign590_e2939);
        (assign590_e2940, (p.p170 * locals.var_tdut_dn4),)
    }
};
        let assign590_e2942: f64 = (p.p169 * assign590_e2941);
        locals.var_cgfp1t = assign590_e2942;
        locals.var_cgfp1t_dn4 = (p.p169 * assign590_e2941_d_n4);

        let assign600_e2948: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign600_e2949: f64 = (p.p192 * assign600_e2948);
        let assign600_e2950: f64 = (1.0 + assign600_e2949);
        let (assign600_e2961, assign600_e2961_d_n4,) = {
    if (assign600_e2950 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign600_e2958: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign600_e2959: f64 = (p.p192 * assign600_e2958);
        let assign600_e2960: f64 = (1.0 + assign600_e2959);
        (assign600_e2960, (p.p192 * locals.var_tdut_dn4),)
    }
};
        let assign600_e2962: f64 = (p.p191 * assign600_e2961);
        locals.var_cgfp2t = assign600_e2962;
        locals.var_cgfp2t_dn4 = (p.p191 * assign600_e2961_d_n4);

        let assign610_e2968: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign610_e2969: f64 = (p.p214 * assign610_e2968);
        let assign610_e2970: f64 = (1.0 + assign610_e2969);
        let (assign610_e2981, assign610_e2981_d_n4,) = {
    if (assign610_e2970 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign610_e2978: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign610_e2979: f64 = (p.p214 * assign610_e2978);
        let assign610_e2980: f64 = (1.0 + assign610_e2979);
        (assign610_e2980, (p.p214 * locals.var_tdut_dn4),)
    }
};
        let assign610_e2982: f64 = (p.p213 * assign610_e2981);
        locals.var_cgfp3t = assign610_e2982;
        locals.var_cgfp3t_dn4 = (p.p213 * assign610_e2981_d_n4);

        let assign620_e2988: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign620_e2989: f64 = (p.p236 * assign620_e2988);
        let assign620_e2990: f64 = (1.0 + assign620_e2989);
        let (assign620_e3001, assign620_e3001_d_n4,) = {
    if (assign620_e2990 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign620_e2998: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign620_e2999: f64 = (p.p236 * assign620_e2998);
        let assign620_e3000: f64 = (1.0 + assign620_e2999);
        (assign620_e3000, (p.p236 * locals.var_tdut_dn4),)
    }
};
        let assign620_e3002: f64 = (p.p235 * assign620_e3001);
        locals.var_cgfp4t = assign620_e3002;
        locals.var_cgfp4t_dn4 = (p.p235 * assign620_e3001_d_n4);

        let assign630_e3008: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign630_e3009: f64 = (p.p175 * assign630_e3008);
        let assign630_e3010: f64 = (1.0 + assign630_e3009);
        let (assign630_e3021, assign630_e3021_d_n4,) = {
    if (assign630_e3010 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign630_e3018: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign630_e3019: f64 = (p.p175 * assign630_e3018);
        let assign630_e3020: f64 = (1.0 + assign630_e3019);
        (assign630_e3020, (p.p175 * locals.var_tdut_dn4),)
    }
};
        let assign630_e3022: f64 = (p.p174 * assign630_e3021);
        locals.var_ccfp1t = assign630_e3022;
        locals.var_ccfp1t_dn4 = (p.p174 * assign630_e3021_d_n4);

        let assign640_e3028: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign640_e3029: f64 = (p.p197 * assign640_e3028);
        let assign640_e3030: f64 = (1.0 + assign640_e3029);
        let (assign640_e3041, assign640_e3041_d_n4,) = {
    if (assign640_e3030 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign640_e3038: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign640_e3039: f64 = (p.p197 * assign640_e3038);
        let assign640_e3040: f64 = (1.0 + assign640_e3039);
        (assign640_e3040, (p.p197 * locals.var_tdut_dn4),)
    }
};
        let assign640_e3042: f64 = (p.p196 * assign640_e3041);
        locals.var_ccfp2t = assign640_e3042;
        locals.var_ccfp2t_dn4 = (p.p196 * assign640_e3041_d_n4);

        let assign650_e3048: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign650_e3049: f64 = (p.p219 * assign650_e3048);
        let assign650_e3050: f64 = (1.0 + assign650_e3049);
        let (assign650_e3061, assign650_e3061_d_n4,) = {
    if (assign650_e3050 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign650_e3058: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign650_e3059: f64 = (p.p219 * assign650_e3058);
        let assign650_e3060: f64 = (1.0 + assign650_e3059);
        (assign650_e3060, (p.p219 * locals.var_tdut_dn4),)
    }
};
        let assign650_e3062: f64 = (p.p218 * assign650_e3061);
        locals.var_ccfp3t = assign650_e3062;
        locals.var_ccfp3t_dn4 = (p.p218 * assign650_e3061_d_n4);

        let assign660_e3068: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign660_e3069: f64 = (p.p241 * assign660_e3068);
        let assign660_e3070: f64 = (1.0 + assign660_e3069);
        let (assign660_e3081, assign660_e3081_d_n4,) = {
    if (assign660_e3070 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign660_e3078: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign660_e3079: f64 = (p.p241 * assign660_e3078);
        let assign660_e3080: f64 = (1.0 + assign660_e3079);
        (assign660_e3080, (p.p241 * locals.var_tdut_dn4),)
    }
};
        let assign660_e3082: f64 = (p.p240 * assign660_e3081);
        locals.var_ccfp4t = assign660_e3082;
        locals.var_ccfp4t_dn4 = (p.p240 * assign660_e3081_d_n4);

        let assign670_e3088: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign670_e3089: f64 = (p.p177 * assign670_e3088);
        let assign670_e3090: f64 = (1.0 + assign670_e3089);
        let (assign670_e3101, assign670_e3101_d_n4,) = {
    if (assign670_e3090 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign670_e3098: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign670_e3099: f64 = (p.p177 * assign670_e3098);
        let assign670_e3100: f64 = (1.0 + assign670_e3099);
        (assign670_e3100, (p.p177 * locals.var_tdut_dn4),)
    }
};
        let assign670_e3102: f64 = (p.p176 * assign670_e3101);
        locals.var_cbfp1t = assign670_e3102;
        locals.var_cbfp1t_dn4 = (p.p176 * assign670_e3101_d_n4);

        let assign680_e3108: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign680_e3109: f64 = (p.p199 * assign680_e3108);
        let assign680_e3110: f64 = (1.0 + assign680_e3109);
        let (assign680_e3121, assign680_e3121_d_n4,) = {
    if (assign680_e3110 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign680_e3118: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign680_e3119: f64 = (p.p199 * assign680_e3118);
        let assign680_e3120: f64 = (1.0 + assign680_e3119);
        (assign680_e3120, (p.p199 * locals.var_tdut_dn4),)
    }
};
        let assign680_e3122: f64 = (p.p198 * assign680_e3121);
        locals.var_cbfp2t = assign680_e3122;
        locals.var_cbfp2t_dn4 = (p.p198 * assign680_e3121_d_n4);

        let assign690_e3128: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign690_e3129: f64 = (p.p221 * assign690_e3128);
        let assign690_e3130: f64 = (1.0 + assign690_e3129);
        let (assign690_e3141, assign690_e3141_d_n4,) = {
    if (assign690_e3130 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign690_e3138: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign690_e3139: f64 = (p.p221 * assign690_e3138);
        let assign690_e3140: f64 = (1.0 + assign690_e3139);
        (assign690_e3140, (p.p221 * locals.var_tdut_dn4),)
    }
};
        let assign690_e3142: f64 = (p.p220 * assign690_e3141);
        locals.var_cbfp3t = assign690_e3142;
        locals.var_cbfp3t_dn4 = (p.p220 * assign690_e3141_d_n4);

        let assign700_e3148: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign700_e3149: f64 = (p.p243 * assign700_e3148);
        let assign700_e3150: f64 = (1.0 + assign700_e3149);
        let (assign700_e3161, assign700_e3161_d_n4,) = {
    if (assign700_e3150 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign700_e3158: f64 = (locals.var_tdut - locals.var_tnomk);
        let assign700_e3159: f64 = (p.p243 * assign700_e3158);
        let assign700_e3160: f64 = (1.0 + assign700_e3159);
        (assign700_e3160, (p.p243 * locals.var_tdut_dn4),)
    }
};
        let assign700_e3162: f64 = (p.p242 * assign700_e3161);
        locals.var_cbfp4t = assign700_e3162;
        locals.var_cbfp4t_dn4 = (p.p242 * assign700_e3161_d_n4);

        let assign710_e3165: f64 = (p.p6 * (nv5 - nv9));
        locals.var_vdsi = assign710_e3165;
        locals.var_vdsi_dn5 = p.p6;
        locals.var_vdsi_dn9 = (-p.p6);

        let assign720_e3168: f64 = (p.p6 * (nv8 - nv9));
        locals.var_vgsi = assign720_e3168;
        locals.var_vgsi_dn8 = p.p6;
        locals.var_vgsi_dn9 = (-p.p6);

        let assign730_e3171: f64 = if p.p52 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard10 = assign730_e3171;

        let assign740_e3174: f64 = (p.p6 * (nv19 - nv0));
        let assign740_e3177: f64 = (p.p6 * (nv19 - nv2));
        let assign740_e3178: f64 = if assign740_e3174 <= assign740_e3177 { 1.0 } else { 0.0 };
        locals.var_guard11 = assign740_e3178;

        let (assign750_e3186, assign750_e3186_d_n0, assign750_e3186_d_n2, assign750_e3186_d_n19,) = {
    if ((locals.var_guard10 != 0.0) && (locals.var_guard11 != 0.0)) {
        let assign750_e3184: f64 = (p.p6 * (nv19 - nv2));
        (assign750_e3184, 0.0, (-p.p6), p.p6,)
    } else {
        (locals.var_vsars, locals.var_vsars_dn0, locals.var_vsars_dn2, locals.var_vsars_dn19,)
    }
};
        locals.var_vsars = assign750_e3186;
        locals.var_vsars_dn0 = assign750_e3186_d_n0;
        locals.var_vsars_dn2 = assign750_e3186_d_n2;
        locals.var_vsars_dn19 = assign750_e3186_d_n19;

        let (assign760_e3195, assign760_e3195_d_n0, assign760_e3195_d_n2, assign760_e3195_d_n19,) = {
    if ((locals.var_guard10 != 0.0) && (locals.var_guard11 == 0.0)) {
        let assign760_e3193: f64 = (p.p6 * (nv19 - nv0));
        (assign760_e3193, (-p.p6), 0.0, p.p6,)
    } else {
        (locals.var_vsars, locals.var_vsars_dn0, locals.var_vsars_dn2, locals.var_vsars_dn19,)
    }
};
        locals.var_vsars = assign760_e3195;
        locals.var_vsars_dn0 = assign760_e3195_d_n0;
        locals.var_vsars_dn2 = assign760_e3195_d_n2;
        locals.var_vsars_dn19 = assign760_e3195_d_n19;

        let (assign770_e3267, assign770_e3267_d_n0, assign770_e3267_d_n2, assign770_e3267_d_n19,) = {
    if (locals.var_guard10 == 0.0) {
        let (assign770_e3265, assign770_e3265_d_n0, assign770_e3265_d_n2, assign770_e3265_d_n19,) = {
            if (p.p52 != 0.0) {
                let assign770_e3205: f64 = (p.p6 * (nv19 - nv0));
                let assign770_e3208: f64 = (p.p6 * (nv19 - nv2));
                let assign770_e3209: f64 = (assign770_e3205 + assign770_e3208);
                let assign770_e3212: f64 = (p.p6 * (nv19 - nv0));
                let assign770_e3215: f64 = (p.p6 * (nv19 - nv2));
                let assign770_e3216: f64 = (assign770_e3212 - assign770_e3215);
                let assign770_e3219: f64 = (0.001 / p.p53);
                let assign770_e3222: f64 = (p.p6 * (nv19 - nv0));
                let assign770_e3225: f64 = (p.p6 * (nv19 - nv2));
                let assign770_e3226: f64 = (assign770_e3222 - assign770_e3225);
                let assign770_e3227: f64 = (assign770_e3219 * assign770_e3226);
                let assign770_e3228: f64 = (assign770_e3227).tanh();
                let assign770_e3229: f64 = (assign770_e3216 * assign770_e3228);
                let assign770_e3230: f64 = (assign770_e3209 + assign770_e3229);
                let assign770_e3231: f64 = (0.5 * assign770_e3230);
                (assign770_e3231, (0.5 * ((-p.p6) + (((-p.p6) * assign770_e3228) + (assign770_e3216 * ((assign770_e3219 * (-p.p6)) / ((assign770_e3227).cosh() * (assign770_e3227).cosh())))))), (0.5 * ((-p.p6) + (((-(-p.p6)) * assign770_e3228) + (assign770_e3216 * ((assign770_e3219 * (-(-p.p6))) / ((assign770_e3227).cosh() * (assign770_e3227).cosh())))))), (0.5 * ((p.p6 + p.p6) + (((p.p6 - p.p6) * assign770_e3228) + (assign770_e3216 * ((assign770_e3219 * (p.p6 - p.p6)) / ((assign770_e3227).cosh() * (assign770_e3227).cosh())))))),)
            } else {
                let (assign770_e3264, assign770_e3264_d_n0, assign770_e3264_d_n2, assign770_e3264_d_n19,) = {
                    if (p.p52 == 0.0) {
                        let assign770_e3238: f64 = (p.p6 * (nv19 - nv0));
                        let assign770_e3241: f64 = (p.p6 * (nv19 - nv2));
                        let assign770_e3242: f64 = (assign770_e3238 + assign770_e3241);
                        let assign770_e3245: f64 = (p.p6 * (nv19 - nv0));
                        let assign770_e3248: f64 = (p.p6 * (nv19 - nv2));
                        let assign770_e3249: f64 = (assign770_e3245 - assign770_e3248);
                        let assign770_e3252: f64 = (p.p6 * (nv19 - nv0));
                        let assign770_e3255: f64 = (p.p6 * (nv19 - nv2));
                        let assign770_e3256: f64 = (assign770_e3252 - assign770_e3255);
                        let assign770_e3257: f64 = (assign770_e3249 * assign770_e3256);
                        let assign770_e3259: f64 = (assign770_e3257 + p.p53);
                        let assign770_e3260: f64 = (assign770_e3259).sqrt();
                        let assign770_e3261: f64 = (assign770_e3242 + assign770_e3260);
                        let assign770_e3262: f64 = (0.5 * assign770_e3261);
                        (assign770_e3262, (0.5 * ((-p.p6) + ((((-p.p6) * assign770_e3256) + (assign770_e3249 * (-p.p6))) / (2.0 * assign770_e3260)))), (0.5 * ((-p.p6) + ((((-(-p.p6)) * assign770_e3256) + (assign770_e3249 * (-(-p.p6)))) / (2.0 * assign770_e3260)))), (0.5 * ((p.p6 + p.p6) + ((((p.p6 - p.p6) * assign770_e3256) + (assign770_e3249 * (p.p6 - p.p6))) / (2.0 * assign770_e3260)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign770_e3264, assign770_e3264_d_n0, assign770_e3264_d_n2, assign770_e3264_d_n19,)
            }
        };
        (assign770_e3265, assign770_e3265_d_n0, assign770_e3265_d_n2, assign770_e3265_d_n19,)
    } else {
        (locals.var_vsars, locals.var_vsars_dn0, locals.var_vsars_dn2, locals.var_vsars_dn19,)
    }
};
        locals.var_vsars = assign770_e3267;
        locals.var_vsars_dn0 = assign770_e3267_d_n0;
        locals.var_vsars_dn2 = assign770_e3267_d_n2;
        locals.var_vsars_dn19 = assign770_e3267_d_n19;

        let assign780_e3272: f64 = (p.p29 * p.p56);
        let assign780_e3274: f64 = (assign780_e3272 * p.p33);
        let assign780_e3275: f64 = (1.0 / assign780_e3274);
        let assign780_e3276: f64 = (p.p55 + assign780_e3275);
        locals.var_vigs = assign780_e3276;

        let assign790_e3279: f64 = (p.p6 * (nv13 - nv19));
        locals.var_vdsrs = assign790_e3279;
        locals.var_vdsrs_dn13 = p.p6;
        locals.var_vdsrs_dn19 = (-p.p6);

        let assign800_e3282: f64 = (locals.var_vigs - locals.var_vsars);
        locals.var_vgsrs = assign800_e3282;
        locals.var_vgsrs_dn0 = (-locals.var_vsars_dn0);
        locals.var_vgsrs_dn2 = (-locals.var_vsars_dn2);
        locals.var_vgsrs_dn19 = (-locals.var_vsars_dn19);

        locals.var_vtcollapse = 0.0;
        locals.var_vtcollapse_dn20 = 0.0;

        locals.var_drsht = 1.0;
        locals.var_drsht_dn4 = 0.0;
        locals.var_drsht_dn20 = 0.0;

        locals.var_vdlinput = 0.0;
        locals.var_vdlinput_dn22 = 0.0;

        locals.var_vglinput = 0.0;
        locals.var_vglinput_dn25 = 0.0;

        locals.var_vdloutput = 0.0;
        locals.var_vdloutput_dn23 = 0.0;

        locals.var_vgloutput = 0.0;
        locals.var_vgloutput_dn26 = 0.0;

        locals.var_chargefracd = 0.0;
        locals.var_chargefracd_dn22 = 0.0;
        locals.var_chargefracd_dn23 = 0.0;

        locals.var_chargefracg = 0.0;
        locals.var_chargefracg_dn25 = 0.0;
        locals.var_chargefracg_dn26 = 0.0;

        locals.var_chargefrac = 1.0;
        locals.var_chargefrac_dn22 = 0.0;
        locals.var_chargefrac_dn23 = 0.0;
        locals.var_chargefrac_dn25 = 0.0;
        locals.var_chargefrac_dn26 = 0.0;

        let assign910_e3295: f64 = if p.p328 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard12 = assign910_e3295;

    }

    pub(super) fn stamp_transient_block_2(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let nv23 = ctx.node_voltage(nodes[23]);
        let nv25 = ctx.node_voltage(nodes[25]);
        let nv26 = ctx.node_voltage(nodes[26]);
        let (assign930_e3402, assign930_e3402_d_n20,) = {
    if (locals.var_guard12 != 0.0) {
        ((nv20 - 0.0), 1.0,)
    } else {
        (locals.var_vtcollapse, locals.var_vtcollapse_dn20,)
    }
};
        locals.var_vtcollapse = assign930_e3402;
        locals.var_vtcollapse_dn20 = assign930_e3402_d_n20;

        let (assign940_e3410, assign940_e3410_d_n4, assign940_e3410_d_n20,) = {
    if (locals.var_guard12 != 0.0) {
        let assign940_e3407: f64 = (locals.var_vtcollapse * locals.var_ttrapfac);
        let assign940_e3408: f64 = (1.0 + assign940_e3407);
        (assign940_e3408, (locals.var_vtcollapse * locals.var_ttrapfac_dn4), (locals.var_vtcollapse_dn20 * locals.var_ttrapfac),)
    } else {
        (locals.var_drsht, locals.var_drsht_dn4, locals.var_drsht_dn20,)
    }
};
        locals.var_drsht = assign940_e3410;
        locals.var_drsht_dn4 = assign940_e3410_d_n4;
        locals.var_drsht_dn20 = assign940_e3410_d_n20;

        let assign950_e3413: f64 = if p.p328 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign950_e3413;

        let (assign960_e3420, assign960_e3420_d_n22,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        ((nv22 - 0.0), 1.0,)
    } else {
        (locals.var_vdlinput, locals.var_vdlinput_dn22,)
    }
};
        locals.var_vdlinput = assign960_e3420;
        locals.var_vdlinput_dn22 = assign960_e3420_d_n22;

        let (assign970_e3427, assign970_e3427_d_n23,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        ((nv23 - 0.0), 1.0,)
    } else {
        (locals.var_vdloutput, locals.var_vdloutput_dn23,)
    }
};
        locals.var_vdloutput = assign970_e3427;
        locals.var_vdloutput_dn23 = assign970_e3427_d_n23;

        let (assign980_e3439, assign980_e3439_d_n22, assign980_e3439_d_n23,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        let assign980_e3434: f64 = (locals.var_vdloutput - locals.var_vdlinput);
        let assign980_e3435: f64 = (assign980_e3434).abs();
        let assign980_e3437: f64 = (assign980_e3435 / p.p338);
        (assign980_e3437, (if assign980_e3434 >= 0.0 { (-locals.var_vdlinput_dn22) } else { (-(-locals.var_vdlinput_dn22)) } / p.p338), (if assign980_e3434 >= 0.0 { locals.var_vdloutput_dn23 } else { (-locals.var_vdloutput_dn23) } / p.p338),)
    } else {
        (locals.var_chargefracd, locals.var_chargefracd_dn22, locals.var_chargefracd_dn23,)
    }
};
        locals.var_chargefracd = assign980_e3439;
        locals.var_chargefracd_dn22 = assign980_e3439_d_n22;
        locals.var_chargefracd_dn23 = assign980_e3439_d_n23;

        let (assign990_e3446, assign990_e3446_d_n25,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        ((nv25 - 0.0), 1.0,)
    } else {
        (locals.var_vglinput, locals.var_vglinput_dn25,)
    }
};
        locals.var_vglinput = assign990_e3446;
        locals.var_vglinput_dn25 = assign990_e3446_d_n25;

        let (assign1000_e3453, assign1000_e3453_d_n26,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        ((nv26 - 0.0), 1.0,)
    } else {
        (locals.var_vgloutput, locals.var_vgloutput_dn26,)
    }
};
        locals.var_vgloutput = assign1000_e3453;
        locals.var_vgloutput_dn26 = assign1000_e3453_d_n26;

        let (assign1010_e3465, assign1010_e3465_d_n25, assign1010_e3465_d_n26,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        let assign1010_e3460: f64 = (locals.var_vgloutput - locals.var_vglinput);
        let assign1010_e3461: f64 = (assign1010_e3460).abs();
        let assign1010_e3463: f64 = (assign1010_e3461 / p.p337);
        (assign1010_e3463, (if assign1010_e3460 >= 0.0 { (-locals.var_vglinput_dn25) } else { (-(-locals.var_vglinput_dn25)) } / p.p337), (if assign1010_e3460 >= 0.0 { locals.var_vgloutput_dn26 } else { (-locals.var_vgloutput_dn26) } / p.p337),)
    } else {
        (locals.var_chargefracg, locals.var_chargefracg_dn25, locals.var_chargefracg_dn26,)
    }
};
        locals.var_chargefracg = assign1010_e3465;
        locals.var_chargefracg_dn25 = assign1010_e3465_d_n25;
        locals.var_chargefracg_dn26 = assign1010_e3465_d_n26;

        let (assign1020_e3478, assign1020_e3478_d_n22, assign1020_e3478_d_n23, assign1020_e3478_d_n25, assign1020_e3478_d_n26,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        let assign1020_e3473: f64 = (1.0 + locals.var_chargefracd);
        let assign1020_e3475: f64 = (assign1020_e3473 + locals.var_chargefracg);
        let assign1020_e3476: f64 = (1.0 / assign1020_e3475);
        (assign1020_e3476, (-(locals.var_chargefracd_dn22 / (assign1020_e3475 * assign1020_e3475))), (-(locals.var_chargefracd_dn23 / (assign1020_e3475 * assign1020_e3475))), (-(locals.var_chargefracg_dn25 / (assign1020_e3475 * assign1020_e3475))), (-(locals.var_chargefracg_dn26 / (assign1020_e3475 * assign1020_e3475))),)
    } else {
        (locals.var_chargefrac, locals.var_chargefrac_dn22, locals.var_chargefrac_dn23, locals.var_chargefrac_dn25, locals.var_chargefrac_dn26,)
    }
};
        locals.var_chargefrac = assign1020_e3478;
        locals.var_chargefrac_dn22 = assign1020_e3478_d_n22;
        locals.var_chargefrac_dn23 = assign1020_e3478_d_n23;
        locals.var_chargefrac_dn25 = assign1020_e3478_d_n25;
        locals.var_chargefrac_dn26 = assign1020_e3478_d_n26;

        let assign1030_e3481: f64 = if p.p52 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign1030_e3481;

        let assign1040_e3484: f64 = (p.p6 * (nv17 - nv0));
        let assign1040_e3487: f64 = (p.p6 * (nv17 - nv2));
        let assign1040_e3488: f64 = if assign1040_e3484 <= assign1040_e3487 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1040_e3488;

        let (assign1050_e3496, assign1050_e3496_d_n0, assign1050_e3496_d_n2, assign1050_e3496_d_n17,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 != 0.0)) {
        let assign1050_e3494: f64 = (p.p6 * (nv17 - nv2));
        (assign1050_e3494, 0.0, (-p.p6), p.p6,)
    } else {
        (locals.var_vdars, locals.var_vdars_dn0, locals.var_vdars_dn2, locals.var_vdars_dn17,)
    }
};
        locals.var_vdars = assign1050_e3496;
        locals.var_vdars_dn0 = assign1050_e3496_d_n0;
        locals.var_vdars_dn2 = assign1050_e3496_d_n2;
        locals.var_vdars_dn17 = assign1050_e3496_d_n17;

        let (assign1060_e3505, assign1060_e3505_d_n0, assign1060_e3505_d_n2, assign1060_e3505_d_n17,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
        let assign1060_e3503: f64 = (p.p6 * (nv17 - nv0));
        (assign1060_e3503, (-p.p6), 0.0, p.p6,)
    } else {
        (locals.var_vdars, locals.var_vdars_dn0, locals.var_vdars_dn2, locals.var_vdars_dn17,)
    }
};
        locals.var_vdars = assign1060_e3505;
        locals.var_vdars_dn0 = assign1060_e3505_d_n0;
        locals.var_vdars_dn2 = assign1060_e3505_d_n2;
        locals.var_vdars_dn17 = assign1060_e3505_d_n17;

        let (assign1070_e3577, assign1070_e3577_d_n0, assign1070_e3577_d_n2, assign1070_e3577_d_n17,) = {
    if (locals.var_guard14 == 0.0) {
        let (assign1070_e3575, assign1070_e3575_d_n0, assign1070_e3575_d_n2, assign1070_e3575_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign1070_e3515: f64 = (p.p6 * (nv17 - nv0));
                let assign1070_e3518: f64 = (p.p6 * (nv17 - nv2));
                let assign1070_e3519: f64 = (assign1070_e3515 + assign1070_e3518);
                let assign1070_e3522: f64 = (p.p6 * (nv17 - nv0));
                let assign1070_e3525: f64 = (p.p6 * (nv17 - nv2));
                let assign1070_e3526: f64 = (assign1070_e3522 - assign1070_e3525);
                let assign1070_e3529: f64 = (0.001 / p.p53);
                let assign1070_e3532: f64 = (p.p6 * (nv17 - nv0));
                let assign1070_e3535: f64 = (p.p6 * (nv17 - nv2));
                let assign1070_e3536: f64 = (assign1070_e3532 - assign1070_e3535);
                let assign1070_e3537: f64 = (assign1070_e3529 * assign1070_e3536);
                let assign1070_e3538: f64 = (assign1070_e3537).tanh();
                let assign1070_e3539: f64 = (assign1070_e3526 * assign1070_e3538);
                let assign1070_e3540: f64 = (assign1070_e3519 + assign1070_e3539);
                let assign1070_e3541: f64 = (0.5 * assign1070_e3540);
                (assign1070_e3541, (0.5 * ((-p.p6) + (((-p.p6) * assign1070_e3538) + (assign1070_e3526 * ((assign1070_e3529 * (-p.p6)) / ((assign1070_e3537).cosh() * (assign1070_e3537).cosh())))))), (0.5 * ((-p.p6) + (((-(-p.p6)) * assign1070_e3538) + (assign1070_e3526 * ((assign1070_e3529 * (-(-p.p6))) / ((assign1070_e3537).cosh() * (assign1070_e3537).cosh())))))), (0.5 * ((p.p6 + p.p6) + (((p.p6 - p.p6) * assign1070_e3538) + (assign1070_e3526 * ((assign1070_e3529 * (p.p6 - p.p6)) / ((assign1070_e3537).cosh() * (assign1070_e3537).cosh())))))),)
            } else {
                let (assign1070_e3574, assign1070_e3574_d_n0, assign1070_e3574_d_n2, assign1070_e3574_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign1070_e3548: f64 = (p.p6 * (nv17 - nv0));
                        let assign1070_e3551: f64 = (p.p6 * (nv17 - nv2));
                        let assign1070_e3552: f64 = (assign1070_e3548 + assign1070_e3551);
                        let assign1070_e3555: f64 = (p.p6 * (nv17 - nv0));
                        let assign1070_e3558: f64 = (p.p6 * (nv17 - nv2));
                        let assign1070_e3559: f64 = (assign1070_e3555 - assign1070_e3558);
                        let assign1070_e3562: f64 = (p.p6 * (nv17 - nv0));
                        let assign1070_e3565: f64 = (p.p6 * (nv17 - nv2));
                        let assign1070_e3566: f64 = (assign1070_e3562 - assign1070_e3565);
                        let assign1070_e3567: f64 = (assign1070_e3559 * assign1070_e3566);
                        let assign1070_e3569: f64 = (assign1070_e3567 + p.p53);
                        let assign1070_e3570: f64 = (assign1070_e3569).sqrt();
                        let assign1070_e3571: f64 = (assign1070_e3552 + assign1070_e3570);
                        let assign1070_e3572: f64 = (0.5 * assign1070_e3571);
                        (assign1070_e3572, (0.5 * ((-p.p6) + ((((-p.p6) * assign1070_e3566) + (assign1070_e3559 * (-p.p6))) / (2.0 * assign1070_e3570)))), (0.5 * ((-p.p6) + ((((-(-p.p6)) * assign1070_e3566) + (assign1070_e3559 * (-(-p.p6)))) / (2.0 * assign1070_e3570)))), (0.5 * ((p.p6 + p.p6) + ((((p.p6 - p.p6) * assign1070_e3566) + (assign1070_e3559 * (p.p6 - p.p6))) / (2.0 * assign1070_e3570)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign1070_e3574, assign1070_e3574_d_n0, assign1070_e3574_d_n2, assign1070_e3574_d_n17,)
            }
        };
        (assign1070_e3575, assign1070_e3575_d_n0, assign1070_e3575_d_n2, assign1070_e3575_d_n17,)
    } else {
        (locals.var_vdars, locals.var_vdars_dn0, locals.var_vdars_dn2, locals.var_vdars_dn17,)
    }
};
        locals.var_vdars = assign1070_e3577;
        locals.var_vdars_dn0 = assign1070_e3577_d_n0;
        locals.var_vdars_dn2 = assign1070_e3577_d_n2;
        locals.var_vdars_dn17 = assign1070_e3577_d_n17;

        let assign1080_e3582: f64 = (locals.var_drsht * p.p29);
        let assign1080_e3584: f64 = (assign1080_e3582 * p.p68);
        let assign1080_e3586: f64 = (assign1080_e3584 * p.p33);
        let assign1080_e3587: f64 = (1.0 / assign1080_e3586);
        let assign1080_e3588: f64 = (p.p67 + assign1080_e3587);
        locals.var_vigd = assign1080_e3588;
        locals.var_vigd_dn4 = (-((((locals.var_drsht_dn4 * p.p29) * p.p68) * p.p33) / (assign1080_e3586 * assign1080_e3586)));
        locals.var_vigd_dn20 = (-((((locals.var_drsht_dn20 * p.p29) * p.p68) * p.p33) / (assign1080_e3586 * assign1080_e3586)));

        let assign1090_e3591: f64 = (p.p6 * (nv18 - nv17));
        locals.var_vdsrd = assign1090_e3591;
        locals.var_vdsrd_dn17 = (-p.p6);
        locals.var_vdsrd_dn18 = p.p6;

        let assign1100_e3594: f64 = (locals.var_vigd - locals.var_vdars);
        locals.var_vgsrd = assign1100_e3594;
        locals.var_vgsrd_dn0 = (-locals.var_vdars_dn0);
        locals.var_vgsrd_dn2 = (-locals.var_vdars_dn2);
        locals.var_vgsrd_dn4 = locals.var_vigd_dn4;
        locals.var_vgsrd_dn17 = (-locals.var_vdars_dn17);
        locals.var_vgsrd_dn20 = locals.var_vigd_dn20;

        let assign1110_e3597: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1110_e3597;

        let (assign1120_e3603, assign1120_e3603_d_n2, assign1120_e3603_d_n7, assign1120_e3603_d_n10,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1120_e3601: f64 = (p.p6 * (nv7 - nv10));
        (assign1120_e3601, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vgsfps1, locals.var_vgsfps1_dn2, locals.var_vgsfps1_dn7, locals.var_vgsfps1_dn10,)
    }
};
        locals.var_vgsfps1 = assign1120_e3603;
        locals.var_vgsfps1_dn2 = assign1120_e3603_d_n2;
        locals.var_vgsfps1_dn7 = assign1120_e3603_d_n7;
        locals.var_vgsfps1_dn10 = assign1120_e3603_d_n10;

        let (assign1130_e3609, assign1130_e3609_d_n2, assign1130_e3609_d_n7, assign1130_e3609_d_n10,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1130_e3607: f64 = (p.p6 * (nv2 - nv10));
        (assign1130_e3607, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vcfps1, locals.var_vcfps1_dn2, locals.var_vcfps1_dn7, locals.var_vcfps1_dn10,)
    }
};
        locals.var_vcfps1 = assign1130_e3609;
        locals.var_vcfps1_dn2 = assign1130_e3609_d_n2;
        locals.var_vcfps1_dn7 = assign1130_e3609_d_n7;
        locals.var_vcfps1_dn10 = assign1130_e3609_d_n10;

        let (assign1140_e3616, assign1140_e3616_d_n2, assign1140_e3616_d_n7, assign1140_e3616_d_n10,) = {
    if (locals.var_guard16 == 0.0) {
        let assign1140_e3614: f64 = (p.p6 * (nv2 - nv10));
        (assign1140_e3614, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vgsfps1, locals.var_vgsfps1_dn2, locals.var_vgsfps1_dn7, locals.var_vgsfps1_dn10,)
    }
};
        locals.var_vgsfps1 = assign1140_e3616;
        locals.var_vgsfps1_dn2 = assign1140_e3616_d_n2;
        locals.var_vgsfps1_dn7 = assign1140_e3616_d_n7;
        locals.var_vgsfps1_dn10 = assign1140_e3616_d_n10;

        let (assign1150_e3623, assign1150_e3623_d_n2, assign1150_e3623_d_n7, assign1150_e3623_d_n10,) = {
    if (locals.var_guard16 == 0.0) {
        let assign1150_e3621: f64 = (p.p6 * (nv7 - nv10));
        (assign1150_e3621, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vcfps1, locals.var_vcfps1_dn2, locals.var_vcfps1_dn7, locals.var_vcfps1_dn10,)
    }
};
        locals.var_vcfps1 = assign1150_e3623;
        locals.var_vcfps1_dn2 = assign1150_e3623_d_n2;
        locals.var_vcfps1_dn7 = assign1150_e3623_d_n7;
        locals.var_vcfps1_dn10 = assign1150_e3623_d_n10;

        let assign1160_e3626: f64 = (p.p6 * (nv9 - nv10));
        locals.var_vdsfps1 = assign1160_e3626;
        locals.var_vdsfps1_dn9 = p.p6;
        locals.var_vdsfps1_dn10 = (-p.p6);

        let assign1170_e3629: f64 = (p.p6 * (nv3 - nv10));
        locals.var_vbfps1 = assign1170_e3629;
        locals.var_vbfps1_dn3 = p.p6;
        locals.var_vbfps1_dn10 = (-p.p6);

        let assign1180_e3632: f64 = if p.p100 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1180_e3632;

        let (assign1190_e3638, assign1190_e3638_d_n2, assign1190_e3638_d_n7, assign1190_e3638_d_n11,) = {
    if (locals.var_guard17 != 0.0) {
        let assign1190_e3636: f64 = (p.p6 * (nv7 - nv11));
        (assign1190_e3636, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vgsfps2, locals.var_vgsfps2_dn2, locals.var_vgsfps2_dn7, locals.var_vgsfps2_dn11,)
    }
};
        locals.var_vgsfps2 = assign1190_e3638;
        locals.var_vgsfps2_dn2 = assign1190_e3638_d_n2;
        locals.var_vgsfps2_dn7 = assign1190_e3638_d_n7;
        locals.var_vgsfps2_dn11 = assign1190_e3638_d_n11;

        let (assign1200_e3644, assign1200_e3644_d_n2, assign1200_e3644_d_n7, assign1200_e3644_d_n11,) = {
    if (locals.var_guard17 != 0.0) {
        let assign1200_e3642: f64 = (p.p6 * (nv2 - nv11));
        (assign1200_e3642, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vcfps2, locals.var_vcfps2_dn2, locals.var_vcfps2_dn7, locals.var_vcfps2_dn11,)
    }
};
        locals.var_vcfps2 = assign1200_e3644;
        locals.var_vcfps2_dn2 = assign1200_e3644_d_n2;
        locals.var_vcfps2_dn7 = assign1200_e3644_d_n7;
        locals.var_vcfps2_dn11 = assign1200_e3644_d_n11;

        let (assign1210_e3651, assign1210_e3651_d_n2, assign1210_e3651_d_n7, assign1210_e3651_d_n11,) = {
    if (locals.var_guard17 == 0.0) {
        let assign1210_e3649: f64 = (p.p6 * (nv2 - nv11));
        (assign1210_e3649, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vgsfps2, locals.var_vgsfps2_dn2, locals.var_vgsfps2_dn7, locals.var_vgsfps2_dn11,)
    }
};
        locals.var_vgsfps2 = assign1210_e3651;
        locals.var_vgsfps2_dn2 = assign1210_e3651_d_n2;
        locals.var_vgsfps2_dn7 = assign1210_e3651_d_n7;
        locals.var_vgsfps2_dn11 = assign1210_e3651_d_n11;

        let (assign1220_e3658, assign1220_e3658_d_n2, assign1220_e3658_d_n7, assign1220_e3658_d_n11,) = {
    if (locals.var_guard17 == 0.0) {
        let assign1220_e3656: f64 = (p.p6 * (nv7 - nv11));
        (assign1220_e3656, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vcfps2, locals.var_vcfps2_dn2, locals.var_vcfps2_dn7, locals.var_vcfps2_dn11,)
    }
};
        locals.var_vcfps2 = assign1220_e3658;
        locals.var_vcfps2_dn2 = assign1220_e3658_d_n2;
        locals.var_vcfps2_dn7 = assign1220_e3658_d_n7;
        locals.var_vcfps2_dn11 = assign1220_e3658_d_n11;

        let assign1230_e3661: f64 = (p.p6 * (nv10 - nv11));
        locals.var_vdsfps2 = assign1230_e3661;
        locals.var_vdsfps2_dn10 = p.p6;
        locals.var_vdsfps2_dn11 = (-p.p6);

        let assign1240_e3664: f64 = (p.p6 * (nv3 - nv11));
        locals.var_vbfps2 = assign1240_e3664;
        locals.var_vbfps2_dn3 = p.p6;
        locals.var_vbfps2_dn11 = (-p.p6);

        let assign1250_e3667: f64 = if p.p122 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard18 = assign1250_e3667;

        let (assign1260_e3673, assign1260_e3673_d_n2, assign1260_e3673_d_n7, assign1260_e3673_d_n12,) = {
    if (locals.var_guard18 != 0.0) {
        let assign1260_e3671: f64 = (p.p6 * (nv7 - nv12));
        (assign1260_e3671, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vgsfps3, locals.var_vgsfps3_dn2, locals.var_vgsfps3_dn7, locals.var_vgsfps3_dn12,)
    }
};
        locals.var_vgsfps3 = assign1260_e3673;
        locals.var_vgsfps3_dn2 = assign1260_e3673_d_n2;
        locals.var_vgsfps3_dn7 = assign1260_e3673_d_n7;
        locals.var_vgsfps3_dn12 = assign1260_e3673_d_n12;

        let (assign1270_e3679, assign1270_e3679_d_n2, assign1270_e3679_d_n7, assign1270_e3679_d_n12,) = {
    if (locals.var_guard18 != 0.0) {
        let assign1270_e3677: f64 = (p.p6 * (nv2 - nv12));
        (assign1270_e3677, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vcfps3, locals.var_vcfps3_dn2, locals.var_vcfps3_dn7, locals.var_vcfps3_dn12,)
    }
};
        locals.var_vcfps3 = assign1270_e3679;
        locals.var_vcfps3_dn2 = assign1270_e3679_d_n2;
        locals.var_vcfps3_dn7 = assign1270_e3679_d_n7;
        locals.var_vcfps3_dn12 = assign1270_e3679_d_n12;

        let (assign1280_e3686, assign1280_e3686_d_n2, assign1280_e3686_d_n7, assign1280_e3686_d_n12,) = {
    if (locals.var_guard18 == 0.0) {
        let assign1280_e3684: f64 = (p.p6 * (nv2 - nv12));
        (assign1280_e3684, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vgsfps3, locals.var_vgsfps3_dn2, locals.var_vgsfps3_dn7, locals.var_vgsfps3_dn12,)
    }
};
        locals.var_vgsfps3 = assign1280_e3686;
        locals.var_vgsfps3_dn2 = assign1280_e3686_d_n2;
        locals.var_vgsfps3_dn7 = assign1280_e3686_d_n7;
        locals.var_vgsfps3_dn12 = assign1280_e3686_d_n12;

        let (assign1290_e3693, assign1290_e3693_d_n2, assign1290_e3693_d_n7, assign1290_e3693_d_n12,) = {
    if (locals.var_guard18 == 0.0) {
        let assign1290_e3691: f64 = (p.p6 * (nv7 - nv12));
        (assign1290_e3691, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vcfps3, locals.var_vcfps3_dn2, locals.var_vcfps3_dn7, locals.var_vcfps3_dn12,)
    }
};
        locals.var_vcfps3 = assign1290_e3693;
        locals.var_vcfps3_dn2 = assign1290_e3693_d_n2;
        locals.var_vcfps3_dn7 = assign1290_e3693_d_n7;
        locals.var_vcfps3_dn12 = assign1290_e3693_d_n12;

        let assign1300_e3696: f64 = (p.p6 * (nv11 - nv12));
        locals.var_vdsfps3 = assign1300_e3696;
        locals.var_vdsfps3_dn11 = p.p6;
        locals.var_vdsfps3_dn12 = (-p.p6);

        let assign1310_e3699: f64 = (p.p6 * (nv3 - nv12));
        locals.var_vbfps3 = assign1310_e3699;
        locals.var_vbfps3_dn3 = p.p6;
        locals.var_vbfps3_dn12 = (-p.p6);

        let assign1320_e3702: f64 = if p.p144 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard19 = assign1320_e3702;

        let (assign1330_e3708, assign1330_e3708_d_n2, assign1330_e3708_d_n7, assign1330_e3708_d_n13,) = {
    if (locals.var_guard19 != 0.0) {
        let assign1330_e3706: f64 = (p.p6 * (nv7 - nv13));
        (assign1330_e3706, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vgsfps4, locals.var_vgsfps4_dn2, locals.var_vgsfps4_dn7, locals.var_vgsfps4_dn13,)
    }
};
        locals.var_vgsfps4 = assign1330_e3708;
        locals.var_vgsfps4_dn2 = assign1330_e3708_d_n2;
        locals.var_vgsfps4_dn7 = assign1330_e3708_d_n7;
        locals.var_vgsfps4_dn13 = assign1330_e3708_d_n13;

        let (assign1340_e3714, assign1340_e3714_d_n2, assign1340_e3714_d_n7, assign1340_e3714_d_n13,) = {
    if (locals.var_guard19 != 0.0) {
        let assign1340_e3712: f64 = (p.p6 * (nv2 - nv13));
        (assign1340_e3712, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vcfps4, locals.var_vcfps4_dn2, locals.var_vcfps4_dn7, locals.var_vcfps4_dn13,)
    }
};
        locals.var_vcfps4 = assign1340_e3714;
        locals.var_vcfps4_dn2 = assign1340_e3714_d_n2;
        locals.var_vcfps4_dn7 = assign1340_e3714_d_n7;
        locals.var_vcfps4_dn13 = assign1340_e3714_d_n13;

        let (assign1350_e3721, assign1350_e3721_d_n2, assign1350_e3721_d_n7, assign1350_e3721_d_n13,) = {
    if (locals.var_guard19 == 0.0) {
        let assign1350_e3719: f64 = (p.p6 * (nv2 - nv13));
        (assign1350_e3719, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vgsfps4, locals.var_vgsfps4_dn2, locals.var_vgsfps4_dn7, locals.var_vgsfps4_dn13,)
    }
};
        locals.var_vgsfps4 = assign1350_e3721;
        locals.var_vgsfps4_dn2 = assign1350_e3721_d_n2;
        locals.var_vgsfps4_dn7 = assign1350_e3721_d_n7;
        locals.var_vgsfps4_dn13 = assign1350_e3721_d_n13;

        let (assign1360_e3728, assign1360_e3728_d_n2, assign1360_e3728_d_n7, assign1360_e3728_d_n13,) = {
    if (locals.var_guard19 == 0.0) {
        let assign1360_e3726: f64 = (p.p6 * (nv7 - nv13));
        (assign1360_e3726, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vcfps4, locals.var_vcfps4_dn2, locals.var_vcfps4_dn7, locals.var_vcfps4_dn13,)
    }
};
        locals.var_vcfps4 = assign1360_e3728;
        locals.var_vcfps4_dn2 = assign1360_e3728_d_n2;
        locals.var_vcfps4_dn7 = assign1360_e3728_d_n7;
        locals.var_vcfps4_dn13 = assign1360_e3728_d_n13;

        let assign1370_e3731: f64 = (p.p6 * (nv12 - nv13));
        locals.var_vdsfps4 = assign1370_e3731;
        locals.var_vdsfps4_dn12 = p.p6;
        locals.var_vdsfps4_dn13 = (-p.p6);

        let assign1380_e3734: f64 = (p.p6 * (nv3 - nv13));
        locals.var_vbfps4 = assign1380_e3734;
        locals.var_vbfps4_dn3 = p.p6;
        locals.var_vbfps4_dn13 = (-p.p6);

        let assign1390_e3737: f64 = if p.p166 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign1390_e3737;

        let (assign1400_e3743, assign1400_e3743_d_n2, assign1400_e3743_d_n5, assign1400_e3743_d_n7,) = {
    if (locals.var_guard20 != 0.0) {
        let assign1400_e3741: f64 = (p.p6 * (nv7 - nv5));
        (assign1400_e3741, 0.0, (-p.p6), p.p6,)
    } else {
        (locals.var_vgsfp1, locals.var_vgsfp1_dn2, locals.var_vgsfp1_dn5, locals.var_vgsfp1_dn7,)
    }
};
        locals.var_vgsfp1 = assign1400_e3743;
        locals.var_vgsfp1_dn2 = assign1400_e3743_d_n2;
        locals.var_vgsfp1_dn5 = assign1400_e3743_d_n5;
        locals.var_vgsfp1_dn7 = assign1400_e3743_d_n7;

        let (assign1410_e3749, assign1410_e3749_d_n2, assign1410_e3749_d_n5, assign1410_e3749_d_n7,) = {
    if (locals.var_guard20 != 0.0) {
        let assign1410_e3747: f64 = (p.p6 * (nv2 - nv5));
        (assign1410_e3747, p.p6, (-p.p6), 0.0,)
    } else {
        (locals.var_vcfp1, locals.var_vcfp1_dn2, locals.var_vcfp1_dn5, locals.var_vcfp1_dn7,)
    }
};
        locals.var_vcfp1 = assign1410_e3749;
        locals.var_vcfp1_dn2 = assign1410_e3749_d_n2;
        locals.var_vcfp1_dn5 = assign1410_e3749_d_n5;
        locals.var_vcfp1_dn7 = assign1410_e3749_d_n7;

    }

    pub(super) fn stamp_transient_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (assign1420_e3756, assign1420_e3756_d_n2, assign1420_e3756_d_n5, assign1420_e3756_d_n7,) = {
    if (locals.var_guard20 == 0.0) {
        let assign1420_e3754: f64 = (p.p6 * (nv2 - nv5));
        (assign1420_e3754, p.p6, (-p.p6), 0.0,)
    } else {
        (locals.var_vgsfp1, locals.var_vgsfp1_dn2, locals.var_vgsfp1_dn5, locals.var_vgsfp1_dn7,)
    }
};
        locals.var_vgsfp1 = assign1420_e3756;
        locals.var_vgsfp1_dn2 = assign1420_e3756_d_n2;
        locals.var_vgsfp1_dn5 = assign1420_e3756_d_n5;
        locals.var_vgsfp1_dn7 = assign1420_e3756_d_n7;

        let (assign1430_e3763, assign1430_e3763_d_n2, assign1430_e3763_d_n5, assign1430_e3763_d_n7,) = {
    if (locals.var_guard20 == 0.0) {
        let assign1430_e3761: f64 = (p.p6 * (nv7 - nv5));
        (assign1430_e3761, 0.0, (-p.p6), p.p6,)
    } else {
        (locals.var_vcfp1, locals.var_vcfp1_dn2, locals.var_vcfp1_dn5, locals.var_vcfp1_dn7,)
    }
};
        locals.var_vcfp1 = assign1430_e3763;
        locals.var_vcfp1_dn2 = assign1430_e3763_d_n2;
        locals.var_vcfp1_dn5 = assign1430_e3763_d_n5;
        locals.var_vcfp1_dn7 = assign1430_e3763_d_n7;

        let assign1440_e3766: f64 = (p.p6 * (nv14 - nv5));
        locals.var_vdsfp1 = assign1440_e3766;
        locals.var_vdsfp1_dn5 = (-p.p6);
        locals.var_vdsfp1_dn14 = p.p6;

        let assign1450_e3769: f64 = (p.p6 * (nv3 - nv5));
        locals.var_vbfp1 = assign1450_e3769;
        locals.var_vbfp1_dn3 = p.p6;
        locals.var_vbfp1_dn5 = (-p.p6);

        let assign1460_e3772: f64 = if p.p188 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign1460_e3772;

        let (assign1470_e3778, assign1470_e3778_d_n2, assign1470_e3778_d_n7, assign1470_e3778_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign1470_e3776: f64 = (p.p6 * (nv7 - nv14));
        (assign1470_e3776, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vgsfp2, locals.var_vgsfp2_dn2, locals.var_vgsfp2_dn7, locals.var_vgsfp2_dn14,)
    }
};
        locals.var_vgsfp2 = assign1470_e3778;
        locals.var_vgsfp2_dn2 = assign1470_e3778_d_n2;
        locals.var_vgsfp2_dn7 = assign1470_e3778_d_n7;
        locals.var_vgsfp2_dn14 = assign1470_e3778_d_n14;

        let (assign1480_e3784, assign1480_e3784_d_n2, assign1480_e3784_d_n7, assign1480_e3784_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign1480_e3782: f64 = (p.p6 * (nv2 - nv14));
        (assign1480_e3782, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vcfp2, locals.var_vcfp2_dn2, locals.var_vcfp2_dn7, locals.var_vcfp2_dn14,)
    }
};
        locals.var_vcfp2 = assign1480_e3784;
        locals.var_vcfp2_dn2 = assign1480_e3784_d_n2;
        locals.var_vcfp2_dn7 = assign1480_e3784_d_n7;
        locals.var_vcfp2_dn14 = assign1480_e3784_d_n14;

        let (assign1490_e3791, assign1490_e3791_d_n2, assign1490_e3791_d_n7, assign1490_e3791_d_n14,) = {
    if (locals.var_guard21 == 0.0) {
        let assign1490_e3789: f64 = (p.p6 * (nv2 - nv14));
        (assign1490_e3789, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vgsfp2, locals.var_vgsfp2_dn2, locals.var_vgsfp2_dn7, locals.var_vgsfp2_dn14,)
    }
};
        locals.var_vgsfp2 = assign1490_e3791;
        locals.var_vgsfp2_dn2 = assign1490_e3791_d_n2;
        locals.var_vgsfp2_dn7 = assign1490_e3791_d_n7;
        locals.var_vgsfp2_dn14 = assign1490_e3791_d_n14;

        let (assign1500_e3798, assign1500_e3798_d_n2, assign1500_e3798_d_n7, assign1500_e3798_d_n14,) = {
    if (locals.var_guard21 == 0.0) {
        let assign1500_e3796: f64 = (p.p6 * (nv7 - nv14));
        (assign1500_e3796, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vcfp2, locals.var_vcfp2_dn2, locals.var_vcfp2_dn7, locals.var_vcfp2_dn14,)
    }
};
        locals.var_vcfp2 = assign1500_e3798;
        locals.var_vcfp2_dn2 = assign1500_e3798_d_n2;
        locals.var_vcfp2_dn7 = assign1500_e3798_d_n7;
        locals.var_vcfp2_dn14 = assign1500_e3798_d_n14;

        let assign1510_e3801: f64 = (p.p6 * (nv15 - nv14));
        locals.var_vdsfp2 = assign1510_e3801;
        locals.var_vdsfp2_dn14 = (-p.p6);
        locals.var_vdsfp2_dn15 = p.p6;

        let assign1520_e3804: f64 = (p.p6 * (nv3 - nv14));
        locals.var_vbfp2 = assign1520_e3804;
        locals.var_vbfp2_dn3 = p.p6;
        locals.var_vbfp2_dn14 = (-p.p6);

        let assign1530_e3807: f64 = if p.p210 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign1530_e3807;

        let (assign1540_e3813, assign1540_e3813_d_n2, assign1540_e3813_d_n7, assign1540_e3813_d_n15,) = {
    if (locals.var_guard22 != 0.0) {
        let assign1540_e3811: f64 = (p.p6 * (nv7 - nv15));
        (assign1540_e3811, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vgsfp3, locals.var_vgsfp3_dn2, locals.var_vgsfp3_dn7, locals.var_vgsfp3_dn15,)
    }
};
        locals.var_vgsfp3 = assign1540_e3813;
        locals.var_vgsfp3_dn2 = assign1540_e3813_d_n2;
        locals.var_vgsfp3_dn7 = assign1540_e3813_d_n7;
        locals.var_vgsfp3_dn15 = assign1540_e3813_d_n15;

        let (assign1550_e3819, assign1550_e3819_d_n2, assign1550_e3819_d_n7, assign1550_e3819_d_n15,) = {
    if (locals.var_guard22 != 0.0) {
        let assign1550_e3817: f64 = (p.p6 * (nv2 - nv15));
        (assign1550_e3817, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vcfp3, locals.var_vcfp3_dn2, locals.var_vcfp3_dn7, locals.var_vcfp3_dn15,)
    }
};
        locals.var_vcfp3 = assign1550_e3819;
        locals.var_vcfp3_dn2 = assign1550_e3819_d_n2;
        locals.var_vcfp3_dn7 = assign1550_e3819_d_n7;
        locals.var_vcfp3_dn15 = assign1550_e3819_d_n15;

        let (assign1560_e3826, assign1560_e3826_d_n2, assign1560_e3826_d_n7, assign1560_e3826_d_n15,) = {
    if (locals.var_guard22 == 0.0) {
        let assign1560_e3824: f64 = (p.p6 * (nv2 - nv15));
        (assign1560_e3824, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vgsfp3, locals.var_vgsfp3_dn2, locals.var_vgsfp3_dn7, locals.var_vgsfp3_dn15,)
    }
};
        locals.var_vgsfp3 = assign1560_e3826;
        locals.var_vgsfp3_dn2 = assign1560_e3826_d_n2;
        locals.var_vgsfp3_dn7 = assign1560_e3826_d_n7;
        locals.var_vgsfp3_dn15 = assign1560_e3826_d_n15;

        let (assign1570_e3833, assign1570_e3833_d_n2, assign1570_e3833_d_n7, assign1570_e3833_d_n15,) = {
    if (locals.var_guard22 == 0.0) {
        let assign1570_e3831: f64 = (p.p6 * (nv7 - nv15));
        (assign1570_e3831, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vcfp3, locals.var_vcfp3_dn2, locals.var_vcfp3_dn7, locals.var_vcfp3_dn15,)
    }
};
        locals.var_vcfp3 = assign1570_e3833;
        locals.var_vcfp3_dn2 = assign1570_e3833_d_n2;
        locals.var_vcfp3_dn7 = assign1570_e3833_d_n7;
        locals.var_vcfp3_dn15 = assign1570_e3833_d_n15;

        let assign1580_e3836: f64 = (p.p6 * (nv16 - nv15));
        locals.var_vdsfp3 = assign1580_e3836;
        locals.var_vdsfp3_dn15 = (-p.p6);
        locals.var_vdsfp3_dn16 = p.p6;

        let assign1590_e3839: f64 = (p.p6 * (nv3 - nv15));
        locals.var_vbfp3 = assign1590_e3839;
        locals.var_vbfp3_dn3 = p.p6;
        locals.var_vbfp3_dn15 = (-p.p6);

        let assign1600_e3842: f64 = if p.p232 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard23 = assign1600_e3842;

        let (assign1610_e3848, assign1610_e3848_d_n2, assign1610_e3848_d_n7, assign1610_e3848_d_n16,) = {
    if (locals.var_guard23 != 0.0) {
        let assign1610_e3846: f64 = (p.p6 * (nv7 - nv16));
        (assign1610_e3846, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vgsfp4, locals.var_vgsfp4_dn2, locals.var_vgsfp4_dn7, locals.var_vgsfp4_dn16,)
    }
};
        locals.var_vgsfp4 = assign1610_e3848;
        locals.var_vgsfp4_dn2 = assign1610_e3848_d_n2;
        locals.var_vgsfp4_dn7 = assign1610_e3848_d_n7;
        locals.var_vgsfp4_dn16 = assign1610_e3848_d_n16;

        let (assign1620_e3854, assign1620_e3854_d_n2, assign1620_e3854_d_n7, assign1620_e3854_d_n16,) = {
    if (locals.var_guard23 != 0.0) {
        let assign1620_e3852: f64 = (p.p6 * (nv2 - nv16));
        (assign1620_e3852, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vcfp4, locals.var_vcfp4_dn2, locals.var_vcfp4_dn7, locals.var_vcfp4_dn16,)
    }
};
        locals.var_vcfp4 = assign1620_e3854;
        locals.var_vcfp4_dn2 = assign1620_e3854_d_n2;
        locals.var_vcfp4_dn7 = assign1620_e3854_d_n7;
        locals.var_vcfp4_dn16 = assign1620_e3854_d_n16;

        let (assign1630_e3861, assign1630_e3861_d_n2, assign1630_e3861_d_n7, assign1630_e3861_d_n16,) = {
    if (locals.var_guard23 == 0.0) {
        let assign1630_e3859: f64 = (p.p6 * (nv2 - nv16));
        (assign1630_e3859, p.p6, 0.0, (-p.p6),)
    } else {
        (locals.var_vgsfp4, locals.var_vgsfp4_dn2, locals.var_vgsfp4_dn7, locals.var_vgsfp4_dn16,)
    }
};
        locals.var_vgsfp4 = assign1630_e3861;
        locals.var_vgsfp4_dn2 = assign1630_e3861_d_n2;
        locals.var_vgsfp4_dn7 = assign1630_e3861_d_n7;
        locals.var_vgsfp4_dn16 = assign1630_e3861_d_n16;

        let (assign1640_e3868, assign1640_e3868_d_n2, assign1640_e3868_d_n7, assign1640_e3868_d_n16,) = {
    if (locals.var_guard23 == 0.0) {
        let assign1640_e3866: f64 = (p.p6 * (nv7 - nv16));
        (assign1640_e3866, 0.0, p.p6, (-p.p6),)
    } else {
        (locals.var_vcfp4, locals.var_vcfp4_dn2, locals.var_vcfp4_dn7, locals.var_vcfp4_dn16,)
    }
};
        locals.var_vcfp4 = assign1640_e3868;
        locals.var_vcfp4_dn2 = assign1640_e3868_d_n2;
        locals.var_vcfp4_dn7 = assign1640_e3868_d_n7;
        locals.var_vcfp4_dn16 = assign1640_e3868_d_n16;

        let assign1650_e3871: f64 = (p.p6 * (nv17 - nv16));
        locals.var_vdsfp4 = assign1650_e3871;
        locals.var_vdsfp4_dn16 = (-p.p6);
        locals.var_vdsfp4_dn17 = p.p6;

        let assign1660_e3874: f64 = (p.p6 * (nv3 - nv16));
        locals.var_vbfp4 = assign1660_e3874;
        locals.var_vbfp4_dn3 = p.p6;
        locals.var_vbfp4_dn16 = (-p.p6);

        locals.var_idsfp4 = 0.0;
        locals.var_idsfp4_dn2 = 0.0;
        locals.var_idsfp4_dn3 = 0.0;
        locals.var_idsfp4_dn4 = 0.0;
        locals.var_idsfp4_dn7 = 0.0;
        locals.var_idsfp4_dn16 = 0.0;
        locals.var_idsfp4_dn17 = 0.0;

        locals.var_qgsfp4 = 0.0;
        locals.var_qgsfp4_dn2 = 0.0;
        locals.var_qgsfp4_dn4 = 0.0;
        locals.var_qgsfp4_dn7 = 0.0;
        locals.var_qgsfp4_dn16 = 0.0;
        locals.var_qgsfp4_dn17 = 0.0;

        locals.var_qgdfp4 = 0.0;
        locals.var_qgdfp4_dn2 = 0.0;
        locals.var_qgdfp4_dn4 = 0.0;
        locals.var_qgdfp4_dn7 = 0.0;
        locals.var_qgdfp4_dn16 = 0.0;
        locals.var_qgdfp4_dn17 = 0.0;

        locals.var_qcfp4 = 0.0;
        locals.var_qcfp4_dn2 = 0.0;
        locals.var_qcfp4_dn3 = 0.0;
        locals.var_qcfp4_dn4 = 0.0;
        locals.var_qcfp4_dn7 = 0.0;
        locals.var_qcfp4_dn16 = 0.0;
        locals.var_qcfp4_dn17 = 0.0;

        locals.var_qbfp4 = 0.0;
        locals.var_qbfp4_dn2 = 0.0;
        locals.var_qbfp4_dn3 = 0.0;
        locals.var_qbfp4_dn4 = 0.0;
        locals.var_qbfp4_dn7 = 0.0;
        locals.var_qbfp4_dn16 = 0.0;
        locals.var_qbfp4_dn17 = 0.0;

        locals.var_qsfp4 = 0.0;
        locals.var_qsfp4_dn2 = 0.0;
        locals.var_qsfp4_dn3 = 0.0;
        locals.var_qsfp4_dn4 = 0.0;
        locals.var_qsfp4_dn7 = 0.0;
        locals.var_qsfp4_dn16 = 0.0;
        locals.var_qsfp4_dn17 = 0.0;

        let assign1750_e3885: f64 = if p.p233 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard24 = assign1750_e3885;

        let (assign1760_e3889, assign1760_e3889_d_n2, assign1760_e3889_d_n3, assign1760_e3889_d_n4, assign1760_e3889_d_n7, assign1760_e3889_d_n16, assign1760_e3889_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__return, locals.var_fn25_calc_iq__return_dn2, locals.var_fn25_calc_iq__return_dn3, locals.var_fn25_calc_iq__return_dn4, locals.var_fn25_calc_iq__return_dn7, locals.var_fn25_calc_iq__return_dn16, locals.var_fn25_calc_iq__return_dn17,)
    }
};
        locals.var_fn25_calc_iq__return = assign1760_e3889;
        locals.var_fn25_calc_iq__return_dn2 = assign1760_e3889_d_n2;
        locals.var_fn25_calc_iq__return_dn3 = assign1760_e3889_d_n3;
        locals.var_fn25_calc_iq__return_dn4 = assign1760_e3889_d_n4;
        locals.var_fn25_calc_iq__return_dn7 = assign1760_e3889_d_n7;
        locals.var_fn25_calc_iq__return_dn16 = assign1760_e3889_d_n16;
        locals.var_fn25_calc_iq__return_dn17 = assign1760_e3889_d_n17;

        let (assign1770_e3893, assign1770_e3893_d_n2, assign1770_e3893_d_n3, assign1770_e3893_d_n4, assign1770_e3893_d_n7, assign1770_e3893_d_n16, assign1770_e3893_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__idsout, locals.var_fn25_calc_iq__idsout_dn2, locals.var_fn25_calc_iq__idsout_dn3, locals.var_fn25_calc_iq__idsout_dn4, locals.var_fn25_calc_iq__idsout_dn7, locals.var_fn25_calc_iq__idsout_dn16, locals.var_fn25_calc_iq__idsout_dn17,)
    }
};
        locals.var_fn25_calc_iq__idsout = assign1770_e3893;
        locals.var_fn25_calc_iq__idsout_dn2 = assign1770_e3893_d_n2;
        locals.var_fn25_calc_iq__idsout_dn3 = assign1770_e3893_d_n3;
        locals.var_fn25_calc_iq__idsout_dn4 = assign1770_e3893_d_n4;
        locals.var_fn25_calc_iq__idsout_dn7 = assign1770_e3893_d_n7;
        locals.var_fn25_calc_iq__idsout_dn16 = assign1770_e3893_d_n16;
        locals.var_fn25_calc_iq__idsout_dn17 = assign1770_e3893_d_n17;

        let (assign1780_e3897, assign1780_e3897_d_n2, assign1780_e3897_d_n4, assign1780_e3897_d_n7, assign1780_e3897_d_n16, assign1780_e3897_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qgsout, locals.var_fn25_calc_iq__qgsout_dn2, locals.var_fn25_calc_iq__qgsout_dn4, locals.var_fn25_calc_iq__qgsout_dn7, locals.var_fn25_calc_iq__qgsout_dn16, locals.var_fn25_calc_iq__qgsout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qgsout = assign1780_e3897;
        locals.var_fn25_calc_iq__qgsout_dn2 = assign1780_e3897_d_n2;
        locals.var_fn25_calc_iq__qgsout_dn4 = assign1780_e3897_d_n4;
        locals.var_fn25_calc_iq__qgsout_dn7 = assign1780_e3897_d_n7;
        locals.var_fn25_calc_iq__qgsout_dn16 = assign1780_e3897_d_n16;
        locals.var_fn25_calc_iq__qgsout_dn17 = assign1780_e3897_d_n17;

        let (assign1790_e3901, assign1790_e3901_d_n2, assign1790_e3901_d_n4, assign1790_e3901_d_n7, assign1790_e3901_d_n16, assign1790_e3901_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qgdout, locals.var_fn25_calc_iq__qgdout_dn2, locals.var_fn25_calc_iq__qgdout_dn4, locals.var_fn25_calc_iq__qgdout_dn7, locals.var_fn25_calc_iq__qgdout_dn16, locals.var_fn25_calc_iq__qgdout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qgdout = assign1790_e3901;
        locals.var_fn25_calc_iq__qgdout_dn2 = assign1790_e3901_d_n2;
        locals.var_fn25_calc_iq__qgdout_dn4 = assign1790_e3901_d_n4;
        locals.var_fn25_calc_iq__qgdout_dn7 = assign1790_e3901_d_n7;
        locals.var_fn25_calc_iq__qgdout_dn16 = assign1790_e3901_d_n16;
        locals.var_fn25_calc_iq__qgdout_dn17 = assign1790_e3901_d_n17;

        let (assign1800_e3905, assign1800_e3905_d_n2, assign1800_e3905_d_n3, assign1800_e3905_d_n4, assign1800_e3905_d_n7, assign1800_e3905_d_n16, assign1800_e3905_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qcout, locals.var_fn25_calc_iq__qcout_dn2, locals.var_fn25_calc_iq__qcout_dn3, locals.var_fn25_calc_iq__qcout_dn4, locals.var_fn25_calc_iq__qcout_dn7, locals.var_fn25_calc_iq__qcout_dn16, locals.var_fn25_calc_iq__qcout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qcout = assign1800_e3905;
        locals.var_fn25_calc_iq__qcout_dn2 = assign1800_e3905_d_n2;
        locals.var_fn25_calc_iq__qcout_dn3 = assign1800_e3905_d_n3;
        locals.var_fn25_calc_iq__qcout_dn4 = assign1800_e3905_d_n4;
        locals.var_fn25_calc_iq__qcout_dn7 = assign1800_e3905_d_n7;
        locals.var_fn25_calc_iq__qcout_dn16 = assign1800_e3905_d_n16;
        locals.var_fn25_calc_iq__qcout_dn17 = assign1800_e3905_d_n17;

        let (assign1810_e3909, assign1810_e3909_d_n2, assign1810_e3909_d_n3, assign1810_e3909_d_n4, assign1810_e3909_d_n7, assign1810_e3909_d_n16, assign1810_e3909_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qbout, locals.var_fn25_calc_iq__qbout_dn2, locals.var_fn25_calc_iq__qbout_dn3, locals.var_fn25_calc_iq__qbout_dn4, locals.var_fn25_calc_iq__qbout_dn7, locals.var_fn25_calc_iq__qbout_dn16, locals.var_fn25_calc_iq__qbout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qbout = assign1810_e3909;
        locals.var_fn25_calc_iq__qbout_dn2 = assign1810_e3909_d_n2;
        locals.var_fn25_calc_iq__qbout_dn3 = assign1810_e3909_d_n3;
        locals.var_fn25_calc_iq__qbout_dn4 = assign1810_e3909_d_n4;
        locals.var_fn25_calc_iq__qbout_dn7 = assign1810_e3909_d_n7;
        locals.var_fn25_calc_iq__qbout_dn16 = assign1810_e3909_d_n16;
        locals.var_fn25_calc_iq__qbout_dn17 = assign1810_e3909_d_n17;

        let (assign1820_e3913, assign1820_e3913_d_n2, assign1820_e3913_d_n3, assign1820_e3913_d_n4, assign1820_e3913_d_n7, assign1820_e3913_d_n16, assign1820_e3913_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qsout, locals.var_fn25_calc_iq__qsout_dn2, locals.var_fn25_calc_iq__qsout_dn3, locals.var_fn25_calc_iq__qsout_dn4, locals.var_fn25_calc_iq__qsout_dn7, locals.var_fn25_calc_iq__qsout_dn16, locals.var_fn25_calc_iq__qsout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qsout = assign1820_e3913;
        locals.var_fn25_calc_iq__qsout_dn2 = assign1820_e3913_d_n2;
        locals.var_fn25_calc_iq__qsout_dn3 = assign1820_e3913_d_n3;
        locals.var_fn25_calc_iq__qsout_dn4 = assign1820_e3913_d_n4;
        locals.var_fn25_calc_iq__qsout_dn7 = assign1820_e3913_d_n7;
        locals.var_fn25_calc_iq__qsout_dn16 = assign1820_e3913_d_n16;
        locals.var_fn25_calc_iq__qsout_dn17 = assign1820_e3913_d_n17;

        let (assign1830_e3917, assign1830_e3917_d_n4, assign1830_e3917_d_n16, assign1830_e3917_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vtdibl, locals.var_fn25_calc_iq__vtdibl_dn4, locals.var_fn25_calc_iq__vtdibl_dn16, locals.var_fn25_calc_iq__vtdibl_dn17,)
    }
};
        locals.var_fn25_calc_iq__vtdibl = assign1830_e3917;
        locals.var_fn25_calc_iq__vtdibl_dn4 = assign1830_e3917_d_n4;
        locals.var_fn25_calc_iq__vtdibl_dn16 = assign1830_e3917_d_n16;
        locals.var_fn25_calc_iq__vtdibl_dn17 = assign1830_e3917_d_n17;

        let (assign1840_e3921, assign1840_e3921_d_n2, assign1840_e3921_d_n3, assign1840_e3921_d_n4, assign1840_e3921_d_n7, assign1840_e3921_d_n16, assign1840_e3921_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsat1, locals.var_fn25_calc_iq__vdsat1_dn2, locals.var_fn25_calc_iq__vdsat1_dn3, locals.var_fn25_calc_iq__vdsat1_dn4, locals.var_fn25_calc_iq__vdsat1_dn7, locals.var_fn25_calc_iq__vdsat1_dn16, locals.var_fn25_calc_iq__vdsat1_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsat1 = assign1840_e3921;
        locals.var_fn25_calc_iq__vdsat1_dn2 = assign1840_e3921_d_n2;
        locals.var_fn25_calc_iq__vdsat1_dn3 = assign1840_e3921_d_n3;
        locals.var_fn25_calc_iq__vdsat1_dn4 = assign1840_e3921_d_n4;
        locals.var_fn25_calc_iq__vdsat1_dn7 = assign1840_e3921_d_n7;
        locals.var_fn25_calc_iq__vdsat1_dn16 = assign1840_e3921_d_n16;
        locals.var_fn25_calc_iq__vdsat1_dn17 = assign1840_e3921_d_n17;

        let (assign1850_e3925, assign1850_e3925_d_n2, assign1850_e3925_d_n7, assign1850_e3925_d_n16,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_vgsfp4, locals.var_vgsfp4_dn2, locals.var_vgsfp4_dn7, locals.var_vgsfp4_dn16,)
    } else {
        (locals.var_fn25_calc_iq__vgsin, locals.var_fn25_calc_iq__vgsin_dn2, locals.var_fn25_calc_iq__vgsin_dn7, locals.var_fn25_calc_iq__vgsin_dn16,)
    }
};
        locals.var_fn25_calc_iq__vgsin = assign1850_e3925;
        locals.var_fn25_calc_iq__vgsin_dn2 = assign1850_e3925_d_n2;
        locals.var_fn25_calc_iq__vgsin_dn7 = assign1850_e3925_d_n7;
        locals.var_fn25_calc_iq__vgsin_dn16 = assign1850_e3925_d_n16;

        let (assign1860_e3929, assign1860_e3929_d_n16, assign1860_e3929_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_vdsfp4, locals.var_vdsfp4_dn16, locals.var_vdsfp4_dn17,)
    } else {
        (locals.var_fn25_calc_iq__vdsin, locals.var_fn25_calc_iq__vdsin_dn16, locals.var_fn25_calc_iq__vdsin_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsin = assign1860_e3929;
        locals.var_fn25_calc_iq__vdsin_dn16 = assign1860_e3929_d_n16;
        locals.var_fn25_calc_iq__vdsin_dn17 = assign1860_e3929_d_n17;

        let (assign1870_e3933,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p239,)
    } else {
        (locals.var_fn25_calc_iq__qcbflag,)
    }
};
        locals.var_fn25_calc_iq__qcbflag = assign1870_e3933;

        let (assign1880_e3937, assign1880_e3937_d_n2, assign1880_e3937_d_n7, assign1880_e3937_d_n16,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_vcfp4, locals.var_vcfp4_dn2, locals.var_vcfp4_dn7, locals.var_vcfp4_dn16,)
    } else {
        (locals.var_fn25_calc_iq__vcin, locals.var_fn25_calc_iq__vcin_dn2, locals.var_fn25_calc_iq__vcin_dn7, locals.var_fn25_calc_iq__vcin_dn16,)
    }
};
        locals.var_fn25_calc_iq__vcin = assign1880_e3937;
        locals.var_fn25_calc_iq__vcin_dn2 = assign1880_e3937_d_n2;
        locals.var_fn25_calc_iq__vcin_dn7 = assign1880_e3937_d_n7;
        locals.var_fn25_calc_iq__vcin_dn16 = assign1880_e3937_d_n16;

        let (assign1890_e3941, assign1890_e3941_d_n3, assign1890_e3941_d_n16,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_vbfp4, locals.var_vbfp4_dn3, locals.var_vbfp4_dn16,)
    } else {
        (locals.var_fn25_calc_iq__vbin, locals.var_fn25_calc_iq__vbin_dn3, locals.var_fn25_calc_iq__vbin_dn16,)
    }
};
        locals.var_fn25_calc_iq__vbin = assign1890_e3941;
        locals.var_fn25_calc_iq__vbin_dn3 = assign1890_e3941_d_n3;
        locals.var_fn25_calc_iq__vbin_dn16 = assign1890_e3941_d_n16;

        let (assign1900_e3945,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p237,)
    } else {
        (locals.var_fn25_calc_iq__qgsflag,)
    }
};
        locals.var_fn25_calc_iq__qgsflag = assign1900_e3945;

        let (assign1910_e3949, assign1910_e3949_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn25_calc_iq__tambin, locals.var_fn25_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn25_calc_iq__tambin = assign1910_e3949;
        locals.var_fn25_calc_iq__tambin_dn4 = assign1910_e3949_d_n4;

        let (assign1920_e3953,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn25_calc_iq__tnomin,)
    }
};
        locals.var_fn25_calc_iq__tnomin = assign1920_e3953;

        let (assign1930_e3957, assign1930_e3957_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn25_calc_iq__phitin, locals.var_fn25_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn25_calc_iq__phitin = assign1930_e3957;
        locals.var_fn25_calc_iq__phitin_dn4 = assign1930_e3957_d_n4;

        let (assign1940_e3961,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn25_calc_iq__w,)
    }
};
        locals.var_fn25_calc_iq__w = assign1940_e3961;

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1950_e3965,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p233,)
    } else {
        (locals.var_fn25_calc_iq__lin,)
    }
};
        locals.var_fn25_calc_iq__lin = assign1950_e3965;

        let (assign1960_e3969, assign1960_e3969_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_cgfp4t, locals.var_cgfp4t_dn4,)
    } else {
        (locals.var_fn25_calc_iq__cgin, locals.var_fn25_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn25_calc_iq__cgin = assign1960_e3969;
        locals.var_fn25_calc_iq__cgin_dn4 = assign1960_e3969_d_n4;

        let (assign1970_e3973,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p238,)
    } else {
        (locals.var_fn25_calc_iq__cs,)
    }
};
        locals.var_fn25_calc_iq__cs = assign1970_e3973;

        let (assign1980_e3977, assign1980_e3977_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_ccfp4t, locals.var_ccfp4t_dn4,)
    } else {
        (locals.var_fn25_calc_iq__cc, locals.var_fn25_calc_iq__cc_dn4,)
    }
};
        locals.var_fn25_calc_iq__cc = assign1980_e3977;
        locals.var_fn25_calc_iq__cc_dn4 = assign1980_e3977_d_n4;

        let (assign1990_e3981, assign1990_e3981_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_cbfp4t, locals.var_cbfp4t_dn4,)
    } else {
        (locals.var_fn25_calc_iq__cb, locals.var_fn25_calc_iq__cb_dn4,)
    }
};
        locals.var_fn25_calc_iq__cb = assign1990_e3981;
        locals.var_fn25_calc_iq__cb_dn4 = assign1990_e3981_d_n4;

        let (assign2000_e3985,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p234,)
    } else {
        (locals.var_fn25_calc_iq__vto,)
    }
};
        locals.var_fn25_calc_iq__vto = assign2000_e3985;

        let (assign2010_e3989,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p248,)
    } else {
        (locals.var_fn25_calc_iq__ss,)
    }
};
        locals.var_fn25_calc_iq__ss = assign2010_e3989;

        let (assign2020_e3993,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p247,)
    } else {
        (locals.var_fn25_calc_iq__delta1,)
    }
};
        locals.var_fn25_calc_iq__delta1 = assign2020_e3993;

        let (assign2030_e3997,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn25_calc_iq__delta2,)
    }
};
        locals.var_fn25_calc_iq__delta2 = assign2030_e3997;

        let (assign2040_e4001,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p249,)
    } else {
        (locals.var_fn25_calc_iq__nd,)
    }
};
        locals.var_fn25_calc_iq__nd = assign2040_e4001;

        let (assign2050_e4005,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p253,)
    } else {
        (locals.var_fn25_calc_iq__alpha,)
    }
};
        locals.var_fn25_calc_iq__alpha = assign2050_e4005;

        let (assign2060_e4009,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p244,)
    } else {
        (locals.var_fn25_calc_iq__vel0,)
    }
};
        locals.var_fn25_calc_iq__vel0 = assign2060_e4009;

        let (assign2070_e4013,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p245,)
    } else {
        (locals.var_fn25_calc_iq__mu0,)
    }
};
        locals.var_fn25_calc_iq__mu0 = assign2070_e4013;

        let (assign2080_e4017,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p246,)
    } else {
        (locals.var_fn25_calc_iq__beta,)
    }
};
        locals.var_fn25_calc_iq__beta = assign2080_e4017;

        let (assign2090_e4021,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p252,)
    } else {
        (locals.var_fn25_calc_iq__mtheta,)
    }
};
        locals.var_fn25_calc_iq__mtheta = assign2090_e4021;

        let (assign2100_e4025,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p251,)
    } else {
        (locals.var_fn25_calc_iq__vtheta,)
    }
};
        locals.var_fn25_calc_iq__vtheta = assign2100_e4025;

        let (assign2110_e4029,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p250,)
    } else {
        (locals.var_fn25_calc_iq__vtzeta,)
    }
};
        locals.var_fn25_calc_iq__vtzeta = assign2110_e4029;

        let (assign2120_e4033,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn25_calc_iq__dibsat,)
    }
};
        locals.var_fn25_calc_iq__dibsat = assign2120_e4033;

        let (assign2130_e4037,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn25_calc_iq__epsilon,)
    }
};
        locals.var_fn25_calc_iq__epsilon = assign2130_e4037;

        let (assign2140_e4041,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn25_calc_iq__vzeta,)
    }
};
        locals.var_fn25_calc_iq__vzeta = assign2140_e4041;

        let (assign2150_e4045,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn25_calc_iq__lambda,)
    }
};
        locals.var_fn25_calc_iq__lambda = assign2150_e4045;

        let (assign2160_e4049,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn25_calc_iq__ngf,)
    }
};
        locals.var_fn25_calc_iq__ngf = assign2160_e4049;

        let (assign2170_e4053,) = {
    if (locals.var_guard24 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn25_calc_iq__type,)
    }
};
        locals.var_fn25_calc_iq__type = assign2170_e4053;

        let (assign2180_e4057,) = {
    if (locals.var_guard24 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn25_calc_iq__trapfracdl,)
    }
};
        locals.var_fn25_calc_iq__trapfracdl = assign2180_e4057;

        let (assign2190_e4061, assign2190_e4061_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__alpha_phit, locals.var_fn25_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn25_calc_iq__alpha_phit = assign2190_e4061;
        locals.var_fn25_calc_iq__alpha_phit_dn4 = assign2190_e4061_d_n4;

        let (assign2200_e4065, assign2200_e4065_d_n16, assign2200_e4065_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__delta, locals.var_fn25_calc_iq__delta_dn16, locals.var_fn25_calc_iq__delta_dn17,)
    }
};
        locals.var_fn25_calc_iq__delta = assign2200_e4065;
        locals.var_fn25_calc_iq__delta_dn16 = assign2200_e4065_d_n16;
        locals.var_fn25_calc_iq__delta_dn17 = assign2200_e4065_d_n17;

        let (assign2210_e4069, assign2210_e4069_d_n4, assign2210_e4069_d_n16, assign2210_e4069_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__n, locals.var_fn25_calc_iq__n_dn4, locals.var_fn25_calc_iq__n_dn16, locals.var_fn25_calc_iq__n_dn17,)
    }
};
        locals.var_fn25_calc_iq__n = assign2210_e4069;
        locals.var_fn25_calc_iq__n_dn4 = assign2210_e4069_d_n4;
        locals.var_fn25_calc_iq__n_dn16 = assign2210_e4069_d_n16;
        locals.var_fn25_calc_iq__n_dn17 = assign2210_e4069_d_n17;

        let (assign2220_e4073, assign2220_e4073_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vtof, locals.var_fn25_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn25_calc_iq__vtof = assign2220_e4073;
        locals.var_fn25_calc_iq__vtof_dn4 = assign2220_e4073_d_n4;

        let (assign2230_e4077, assign2230_e4077_d_n16, assign2230_e4077_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vsatdibl, locals.var_fn25_calc_iq__vsatdibl_dn16, locals.var_fn25_calc_iq__vsatdibl_dn17,)
    }
};
        locals.var_fn25_calc_iq__vsatdibl = assign2230_e4077;
        locals.var_fn25_calc_iq__vsatdibl_dn16 = assign2230_e4077_d_n16;
        locals.var_fn25_calc_iq__vsatdibl_dn17 = assign2230_e4077_d_n17;

        let (assign2240_e4081, assign2240_e4081_d_n2, assign2240_e4081_d_n3, assign2240_e4081_d_n4, assign2240_e4081_d_n7, assign2240_e4081_d_n16, assign2240_e4081_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffs, locals.var_fn25_calc_iq__ffs_dn2, locals.var_fn25_calc_iq__ffs_dn3, locals.var_fn25_calc_iq__ffs_dn4, locals.var_fn25_calc_iq__ffs_dn7, locals.var_fn25_calc_iq__ffs_dn16, locals.var_fn25_calc_iq__ffs_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs = assign2240_e4081;
        locals.var_fn25_calc_iq__ffs_dn2 = assign2240_e4081_d_n2;
        locals.var_fn25_calc_iq__ffs_dn3 = assign2240_e4081_d_n3;
        locals.var_fn25_calc_iq__ffs_dn4 = assign2240_e4081_d_n4;
        locals.var_fn25_calc_iq__ffs_dn7 = assign2240_e4081_d_n7;
        locals.var_fn25_calc_iq__ffs_dn16 = assign2240_e4081_d_n16;
        locals.var_fn25_calc_iq__ffs_dn17 = assign2240_e4081_d_n17;

        let (assign2250_e4085, assign2250_e4085_d_n4, assign2250_e4085_d_n16, assign2250_e4085_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__two_n_phit, locals.var_fn25_calc_iq__two_n_phit_dn4, locals.var_fn25_calc_iq__two_n_phit_dn16, locals.var_fn25_calc_iq__two_n_phit_dn17,)
    }
};
        locals.var_fn25_calc_iq__two_n_phit = assign2250_e4085;
        locals.var_fn25_calc_iq__two_n_phit_dn4 = assign2250_e4085_d_n4;
        locals.var_fn25_calc_iq__two_n_phit_dn16 = assign2250_e4085_d_n16;
        locals.var_fn25_calc_iq__two_n_phit_dn17 = assign2250_e4085_d_n17;

        let (assign2260_e4089, assign2260_e4089_d_n4, assign2260_e4089_d_n16, assign2260_e4089_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qref, locals.var_fn25_calc_iq__qref_dn4, locals.var_fn25_calc_iq__qref_dn16, locals.var_fn25_calc_iq__qref_dn17,)
    }
};
        locals.var_fn25_calc_iq__qref = assign2260_e4089;
        locals.var_fn25_calc_iq__qref_dn4 = assign2260_e4089_d_n4;
        locals.var_fn25_calc_iq__qref_dn16 = assign2260_e4089_d_n16;
        locals.var_fn25_calc_iq__qref_dn17 = assign2260_e4089_d_n17;

        let (assign2270_e4093, assign2270_e4093_d_n2, assign2270_e4093_d_n3, assign2270_e4093_d_n4, assign2270_e4093_d_n7, assign2270_e4093_d_n16, assign2270_e4093_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__etas, locals.var_fn25_calc_iq__etas_dn2, locals.var_fn25_calc_iq__etas_dn3, locals.var_fn25_calc_iq__etas_dn4, locals.var_fn25_calc_iq__etas_dn7, locals.var_fn25_calc_iq__etas_dn16, locals.var_fn25_calc_iq__etas_dn17,)
    }
};
        locals.var_fn25_calc_iq__etas = assign2270_e4093;
        locals.var_fn25_calc_iq__etas_dn2 = assign2270_e4093_d_n2;
        locals.var_fn25_calc_iq__etas_dn3 = assign2270_e4093_d_n3;
        locals.var_fn25_calc_iq__etas_dn4 = assign2270_e4093_d_n4;
        locals.var_fn25_calc_iq__etas_dn7 = assign2270_e4093_d_n7;
        locals.var_fn25_calc_iq__etas_dn16 = assign2270_e4093_d_n16;
        locals.var_fn25_calc_iq__etas_dn17 = assign2270_e4093_d_n17;

        let (assign2280_e4097, assign2280_e4097_d_n2, assign2280_e4097_d_n3, assign2280_e4097_d_n4, assign2280_e4097_d_n7, assign2280_e4097_d_n16, assign2280_e4097_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qinvs, locals.var_fn25_calc_iq__qinvs_dn2, locals.var_fn25_calc_iq__qinvs_dn3, locals.var_fn25_calc_iq__qinvs_dn4, locals.var_fn25_calc_iq__qinvs_dn7, locals.var_fn25_calc_iq__qinvs_dn16, locals.var_fn25_calc_iq__qinvs_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs = assign2280_e4097;
        locals.var_fn25_calc_iq__qinvs_dn2 = assign2280_e4097_d_n2;
        locals.var_fn25_calc_iq__qinvs_dn3 = assign2280_e4097_d_n3;
        locals.var_fn25_calc_iq__qinvs_dn4 = assign2280_e4097_d_n4;
        locals.var_fn25_calc_iq__qinvs_dn7 = assign2280_e4097_d_n7;
        locals.var_fn25_calc_iq__qinvs_dn16 = assign2280_e4097_d_n16;
        locals.var_fn25_calc_iq__qinvs_dn17 = assign2280_e4097_d_n17;

        let (assign2290_e4101, assign2290_e4101_d_n2, assign2290_e4101_d_n3, assign2290_e4101_d_n4, assign2290_e4101_d_n7, assign2290_e4101_d_n16, assign2290_e4101_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__muf, locals.var_fn25_calc_iq__muf_dn2, locals.var_fn25_calc_iq__muf_dn3, locals.var_fn25_calc_iq__muf_dn4, locals.var_fn25_calc_iq__muf_dn7, locals.var_fn25_calc_iq__muf_dn16, locals.var_fn25_calc_iq__muf_dn17,)
    }
};
        locals.var_fn25_calc_iq__muf = assign2290_e4101;
        locals.var_fn25_calc_iq__muf_dn2 = assign2290_e4101_d_n2;
        locals.var_fn25_calc_iq__muf_dn3 = assign2290_e4101_d_n3;
        locals.var_fn25_calc_iq__muf_dn4 = assign2290_e4101_d_n4;
        locals.var_fn25_calc_iq__muf_dn7 = assign2290_e4101_d_n7;
        locals.var_fn25_calc_iq__muf_dn16 = assign2290_e4101_d_n16;
        locals.var_fn25_calc_iq__muf_dn17 = assign2290_e4101_d_n17;

        let (assign2300_e4105, assign2300_e4105_d_n2, assign2300_e4105_d_n3, assign2300_e4105_d_n4, assign2300_e4105_d_n7, assign2300_e4105_d_n16, assign2300_e4105_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vx, locals.var_fn25_calc_iq__vx_dn2, locals.var_fn25_calc_iq__vx_dn3, locals.var_fn25_calc_iq__vx_dn4, locals.var_fn25_calc_iq__vx_dn7, locals.var_fn25_calc_iq__vx_dn16, locals.var_fn25_calc_iq__vx_dn17,)
    }
};
        locals.var_fn25_calc_iq__vx = assign2300_e4105;
        locals.var_fn25_calc_iq__vx_dn2 = assign2300_e4105_d_n2;
        locals.var_fn25_calc_iq__vx_dn3 = assign2300_e4105_d_n3;
        locals.var_fn25_calc_iq__vx_dn4 = assign2300_e4105_d_n4;
        locals.var_fn25_calc_iq__vx_dn7 = assign2300_e4105_d_n7;
        locals.var_fn25_calc_iq__vx_dn16 = assign2300_e4105_d_n16;
        locals.var_fn25_calc_iq__vx_dn17 = assign2300_e4105_d_n17;

        let (assign2310_e4109, assign2310_e4109_d_n2, assign2310_e4109_d_n3, assign2310_e4109_d_n4, assign2310_e4109_d_n7, assign2310_e4109_d_n16, assign2310_e4109_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vxf, locals.var_fn25_calc_iq__vxf_dn2, locals.var_fn25_calc_iq__vxf_dn3, locals.var_fn25_calc_iq__vxf_dn4, locals.var_fn25_calc_iq__vxf_dn7, locals.var_fn25_calc_iq__vxf_dn16, locals.var_fn25_calc_iq__vxf_dn17,)
    }
};
        locals.var_fn25_calc_iq__vxf = assign2310_e4109;
        locals.var_fn25_calc_iq__vxf_dn2 = assign2310_e4109_d_n2;
        locals.var_fn25_calc_iq__vxf_dn3 = assign2310_e4109_d_n3;
        locals.var_fn25_calc_iq__vxf_dn4 = assign2310_e4109_d_n4;
        locals.var_fn25_calc_iq__vxf_dn7 = assign2310_e4109_d_n7;
        locals.var_fn25_calc_iq__vxf_dn16 = assign2310_e4109_d_n16;
        locals.var_fn25_calc_iq__vxf_dn17 = assign2310_e4109_d_n17;

        let (assign2320_e4113, assign2320_e4113_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__n0, locals.var_fn25_calc_iq__n0_dn4,)
    }
};
        locals.var_fn25_calc_iq__n0 = assign2320_e4113;
        locals.var_fn25_calc_iq__n0_dn4 = assign2320_e4113_d_n4;

        let (assign2330_e4117, assign2330_e4117_d_n2, assign2330_e4117_d_n4, assign2330_e4117_d_n7, assign2330_e4117_d_n16, assign2330_e4117_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffs0, locals.var_fn25_calc_iq__ffs0_dn2, locals.var_fn25_calc_iq__ffs0_dn4, locals.var_fn25_calc_iq__ffs0_dn7, locals.var_fn25_calc_iq__ffs0_dn16, locals.var_fn25_calc_iq__ffs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs0 = assign2330_e4117;
        locals.var_fn25_calc_iq__ffs0_dn2 = assign2330_e4117_d_n2;
        locals.var_fn25_calc_iq__ffs0_dn4 = assign2330_e4117_d_n4;
        locals.var_fn25_calc_iq__ffs0_dn7 = assign2330_e4117_d_n7;
        locals.var_fn25_calc_iq__ffs0_dn16 = assign2330_e4117_d_n16;
        locals.var_fn25_calc_iq__ffs0_dn17 = assign2330_e4117_d_n17;

        let (assign2340_e4121, assign2340_e4121_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__two_n_phit0, locals.var_fn25_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn25_calc_iq__two_n_phit0 = assign2340_e4121;
        locals.var_fn25_calc_iq__two_n_phit0_dn4 = assign2340_e4121_d_n4;

        let (assign2350_e4125, assign2350_e4125_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qref0, locals.var_fn25_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn25_calc_iq__qref0 = assign2350_e4125;
        locals.var_fn25_calc_iq__qref0_dn4 = assign2350_e4125_d_n4;

        let (assign2360_e4129, assign2360_e4129_d_n2, assign2360_e4129_d_n4, assign2360_e4129_d_n7, assign2360_e4129_d_n16, assign2360_e4129_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__etas0, locals.var_fn25_calc_iq__etas0_dn2, locals.var_fn25_calc_iq__etas0_dn4, locals.var_fn25_calc_iq__etas0_dn7, locals.var_fn25_calc_iq__etas0_dn16, locals.var_fn25_calc_iq__etas0_dn17,)
    }
};
        locals.var_fn25_calc_iq__etas0 = assign2360_e4129;
        locals.var_fn25_calc_iq__etas0_dn2 = assign2360_e4129_d_n2;
        locals.var_fn25_calc_iq__etas0_dn4 = assign2360_e4129_d_n4;
        locals.var_fn25_calc_iq__etas0_dn7 = assign2360_e4129_d_n7;
        locals.var_fn25_calc_iq__etas0_dn16 = assign2360_e4129_d_n16;
        locals.var_fn25_calc_iq__etas0_dn17 = assign2360_e4129_d_n17;

        let (assign2370_e4133, assign2370_e4133_d_n2, assign2370_e4133_d_n4, assign2370_e4133_d_n7, assign2370_e4133_d_n16, assign2370_e4133_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qinvs0, locals.var_fn25_calc_iq__qinvs0_dn2, locals.var_fn25_calc_iq__qinvs0_dn4, locals.var_fn25_calc_iq__qinvs0_dn7, locals.var_fn25_calc_iq__qinvs0_dn16, locals.var_fn25_calc_iq__qinvs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs0 = assign2370_e4133;
        locals.var_fn25_calc_iq__qinvs0_dn2 = assign2370_e4133_d_n2;
        locals.var_fn25_calc_iq__qinvs0_dn4 = assign2370_e4133_d_n4;
        locals.var_fn25_calc_iq__qinvs0_dn7 = assign2370_e4133_d_n7;
        locals.var_fn25_calc_iq__qinvs0_dn16 = assign2370_e4133_d_n16;
        locals.var_fn25_calc_iq__qinvs0_dn17 = assign2370_e4133_d_n17;

        let (assign2380_e4137, assign2380_e4137_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__muf0, locals.var_fn25_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn25_calc_iq__muf0 = assign2380_e4137;
        locals.var_fn25_calc_iq__muf0_dn4 = assign2380_e4137_d_n4;

        let (assign2390_e4141, assign2390_e4141_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vx0, locals.var_fn25_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn25_calc_iq__vx0 = assign2390_e4141;
        locals.var_fn25_calc_iq__vx0_dn4 = assign2390_e4141_d_n4;

        let (assign2400_e4145, assign2400_e4145_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__tfacmobin, locals.var_fn25_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn25_calc_iq__tfacmobin = assign2400_e4145;
        locals.var_fn25_calc_iq__tfacmobin_dn4 = assign2400_e4145_d_n4;

        let (assign2410_e4149, assign2410_e4149_d_n2, assign2410_e4149_d_n3, assign2410_e4149_d_n4, assign2410_e4149_d_n7, assign2410_e4149_d_n16, assign2410_e4149_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ff, locals.var_fn25_calc_iq__ff_dn2, locals.var_fn25_calc_iq__ff_dn3, locals.var_fn25_calc_iq__ff_dn4, locals.var_fn25_calc_iq__ff_dn7, locals.var_fn25_calc_iq__ff_dn16, locals.var_fn25_calc_iq__ff_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff = assign2410_e4149;
        locals.var_fn25_calc_iq__ff_dn2 = assign2410_e4149_d_n2;
        locals.var_fn25_calc_iq__ff_dn3 = assign2410_e4149_d_n3;
        locals.var_fn25_calc_iq__ff_dn4 = assign2410_e4149_d_n4;
        locals.var_fn25_calc_iq__ff_dn7 = assign2410_e4149_d_n7;
        locals.var_fn25_calc_iq__ff_dn16 = assign2410_e4149_d_n16;
        locals.var_fn25_calc_iq__ff_dn17 = assign2410_e4149_d_n17;

    }

    pub(super) fn stamp_transient_block_5(
        locals: &mut StampLocals,
    ) {
        let (assign2420_e4153, assign2420_e4153_d_n2, assign2420_e4153_d_n3, assign2420_e4153_d_n4, assign2420_e4153_d_n7, assign2420_e4153_d_n16, assign2420_e4153_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__eta, locals.var_fn25_calc_iq__eta_dn2, locals.var_fn25_calc_iq__eta_dn3, locals.var_fn25_calc_iq__eta_dn4, locals.var_fn25_calc_iq__eta_dn7, locals.var_fn25_calc_iq__eta_dn16, locals.var_fn25_calc_iq__eta_dn17,)
    }
};
        locals.var_fn25_calc_iq__eta = assign2420_e4153;
        locals.var_fn25_calc_iq__eta_dn2 = assign2420_e4153_d_n2;
        locals.var_fn25_calc_iq__eta_dn3 = assign2420_e4153_d_n3;
        locals.var_fn25_calc_iq__eta_dn4 = assign2420_e4153_d_n4;
        locals.var_fn25_calc_iq__eta_dn7 = assign2420_e4153_d_n7;
        locals.var_fn25_calc_iq__eta_dn16 = assign2420_e4153_d_n16;
        locals.var_fn25_calc_iq__eta_dn17 = assign2420_e4153_d_n17;

        let (assign2430_e4157, assign2430_e4157_d_n2, assign2430_e4157_d_n3, assign2430_e4157_d_n4, assign2430_e4157_d_n7, assign2430_e4157_d_n16, assign2430_e4157_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qinvv, locals.var_fn25_calc_iq__qinvv_dn2, locals.var_fn25_calc_iq__qinvv_dn3, locals.var_fn25_calc_iq__qinvv_dn4, locals.var_fn25_calc_iq__qinvv_dn7, locals.var_fn25_calc_iq__qinvv_dn16, locals.var_fn25_calc_iq__qinvv_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv = assign2430_e4157;
        locals.var_fn25_calc_iq__qinvv_dn2 = assign2430_e4157_d_n2;
        locals.var_fn25_calc_iq__qinvv_dn3 = assign2430_e4157_d_n3;
        locals.var_fn25_calc_iq__qinvv_dn4 = assign2430_e4157_d_n4;
        locals.var_fn25_calc_iq__qinvv_dn7 = assign2430_e4157_d_n7;
        locals.var_fn25_calc_iq__qinvv_dn16 = assign2430_e4157_d_n16;
        locals.var_fn25_calc_iq__qinvv_dn17 = assign2430_e4157_d_n17;

        let (assign2440_e4161, assign2440_e4161_d_n2, assign2440_e4161_d_n4, assign2440_e4161_d_n7, assign2440_e4161_d_n16, assign2440_e4161_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ff0, locals.var_fn25_calc_iq__ff0_dn2, locals.var_fn25_calc_iq__ff0_dn4, locals.var_fn25_calc_iq__ff0_dn7, locals.var_fn25_calc_iq__ff0_dn16, locals.var_fn25_calc_iq__ff0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff0 = assign2440_e4161;
        locals.var_fn25_calc_iq__ff0_dn2 = assign2440_e4161_d_n2;
        locals.var_fn25_calc_iq__ff0_dn4 = assign2440_e4161_d_n4;
        locals.var_fn25_calc_iq__ff0_dn7 = assign2440_e4161_d_n7;
        locals.var_fn25_calc_iq__ff0_dn16 = assign2440_e4161_d_n16;
        locals.var_fn25_calc_iq__ff0_dn17 = assign2440_e4161_d_n17;

        let (assign2450_e4165, assign2450_e4165_d_n2, assign2450_e4165_d_n4, assign2450_e4165_d_n7, assign2450_e4165_d_n16, assign2450_e4165_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__eta0, locals.var_fn25_calc_iq__eta0_dn2, locals.var_fn25_calc_iq__eta0_dn4, locals.var_fn25_calc_iq__eta0_dn7, locals.var_fn25_calc_iq__eta0_dn16, locals.var_fn25_calc_iq__eta0_dn17,)
    }
};
        locals.var_fn25_calc_iq__eta0 = assign2450_e4165;
        locals.var_fn25_calc_iq__eta0_dn2 = assign2450_e4165_d_n2;
        locals.var_fn25_calc_iq__eta0_dn4 = assign2450_e4165_d_n4;
        locals.var_fn25_calc_iq__eta0_dn7 = assign2450_e4165_d_n7;
        locals.var_fn25_calc_iq__eta0_dn16 = assign2450_e4165_d_n16;
        locals.var_fn25_calc_iq__eta0_dn17 = assign2450_e4165_d_n17;

        let (assign2460_e4169, assign2460_e4169_d_n2, assign2460_e4169_d_n4, assign2460_e4169_d_n7, assign2460_e4169_d_n16, assign2460_e4169_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qinvv0, locals.var_fn25_calc_iq__qinvv0_dn2, locals.var_fn25_calc_iq__qinvv0_dn4, locals.var_fn25_calc_iq__qinvv0_dn7, locals.var_fn25_calc_iq__qinvv0_dn16, locals.var_fn25_calc_iq__qinvv0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv0 = assign2460_e4169;
        locals.var_fn25_calc_iq__qinvv0_dn2 = assign2460_e4169_d_n2;
        locals.var_fn25_calc_iq__qinvv0_dn4 = assign2460_e4169_d_n4;
        locals.var_fn25_calc_iq__qinvv0_dn7 = assign2460_e4169_d_n7;
        locals.var_fn25_calc_iq__qinvv0_dn16 = assign2460_e4169_d_n16;
        locals.var_fn25_calc_iq__qinvv0_dn17 = assign2460_e4169_d_n17;

        let (assign2470_e4173, assign2470_e4173_d_n2, assign2470_e4173_d_n3, assign2470_e4173_d_n4, assign2470_e4173_d_n7, assign2470_e4173_d_n16, assign2470_e4173_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsats, locals.var_fn25_calc_iq__vdsats_dn2, locals.var_fn25_calc_iq__vdsats_dn3, locals.var_fn25_calc_iq__vdsats_dn4, locals.var_fn25_calc_iq__vdsats_dn7, locals.var_fn25_calc_iq__vdsats_dn16, locals.var_fn25_calc_iq__vdsats_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsats = assign2470_e4173;
        locals.var_fn25_calc_iq__vdsats_dn2 = assign2470_e4173_d_n2;
        locals.var_fn25_calc_iq__vdsats_dn3 = assign2470_e4173_d_n3;
        locals.var_fn25_calc_iq__vdsats_dn4 = assign2470_e4173_d_n4;
        locals.var_fn25_calc_iq__vdsats_dn7 = assign2470_e4173_d_n7;
        locals.var_fn25_calc_iq__vdsats_dn16 = assign2470_e4173_d_n16;
        locals.var_fn25_calc_iq__vdsats_dn17 = assign2470_e4173_d_n17;

        let (assign2480_e4177, assign2480_e4177_d_n2, assign2480_e4177_d_n3, assign2480_e4177_d_n4, assign2480_e4177_d_n7, assign2480_e4177_d_n16, assign2480_e4177_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsats1, locals.var_fn25_calc_iq__vdsats1_dn2, locals.var_fn25_calc_iq__vdsats1_dn3, locals.var_fn25_calc_iq__vdsats1_dn4, locals.var_fn25_calc_iq__vdsats1_dn7, locals.var_fn25_calc_iq__vdsats1_dn16, locals.var_fn25_calc_iq__vdsats1_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsats1 = assign2480_e4177;
        locals.var_fn25_calc_iq__vdsats1_dn2 = assign2480_e4177_d_n2;
        locals.var_fn25_calc_iq__vdsats1_dn3 = assign2480_e4177_d_n3;
        locals.var_fn25_calc_iq__vdsats1_dn4 = assign2480_e4177_d_n4;
        locals.var_fn25_calc_iq__vdsats1_dn7 = assign2480_e4177_d_n7;
        locals.var_fn25_calc_iq__vdsats1_dn16 = assign2480_e4177_d_n16;
        locals.var_fn25_calc_iq__vdsats1_dn17 = assign2480_e4177_d_n17;

        let (assign2490_e4181, assign2490_e4181_d_n2, assign2490_e4181_d_n3, assign2490_e4181_d_n4, assign2490_e4181_d_n7, assign2490_e4181_d_n16, assign2490_e4181_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsat, locals.var_fn25_calc_iq__vdsat_dn2, locals.var_fn25_calc_iq__vdsat_dn3, locals.var_fn25_calc_iq__vdsat_dn4, locals.var_fn25_calc_iq__vdsat_dn7, locals.var_fn25_calc_iq__vdsat_dn16, locals.var_fn25_calc_iq__vdsat_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsat = assign2490_e4181;
        locals.var_fn25_calc_iq__vdsat_dn2 = assign2490_e4181_d_n2;
        locals.var_fn25_calc_iq__vdsat_dn3 = assign2490_e4181_d_n3;
        locals.var_fn25_calc_iq__vdsat_dn4 = assign2490_e4181_d_n4;
        locals.var_fn25_calc_iq__vdsat_dn7 = assign2490_e4181_d_n7;
        locals.var_fn25_calc_iq__vdsat_dn16 = assign2490_e4181_d_n16;
        locals.var_fn25_calc_iq__vdsat_dn17 = assign2490_e4181_d_n17;

        let (assign2500_e4185, assign2500_e4185_d_n2, assign2500_e4185_d_n3, assign2500_e4185_d_n4, assign2500_e4185_d_n7, assign2500_e4185_d_n16, assign2500_e4185_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__fsd, locals.var_fn25_calc_iq__fsd_dn2, locals.var_fn25_calc_iq__fsd_dn3, locals.var_fn25_calc_iq__fsd_dn4, locals.var_fn25_calc_iq__fsd_dn7, locals.var_fn25_calc_iq__fsd_dn16, locals.var_fn25_calc_iq__fsd_dn17,)
    }
};
        locals.var_fn25_calc_iq__fsd = assign2500_e4185;
        locals.var_fn25_calc_iq__fsd_dn2 = assign2500_e4185_d_n2;
        locals.var_fn25_calc_iq__fsd_dn3 = assign2500_e4185_d_n3;
        locals.var_fn25_calc_iq__fsd_dn4 = assign2500_e4185_d_n4;
        locals.var_fn25_calc_iq__fsd_dn7 = assign2500_e4185_d_n7;
        locals.var_fn25_calc_iq__fsd_dn16 = assign2500_e4185_d_n16;
        locals.var_fn25_calc_iq__fsd_dn17 = assign2500_e4185_d_n17;

        let (assign2510_e4189, assign2510_e4189_d_n2, assign2510_e4189_d_n3, assign2510_e4189_d_n4, assign2510_e4189_d_n7, assign2510_e4189_d_n16, assign2510_e4189_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdx, locals.var_fn25_calc_iq__vdx_dn2, locals.var_fn25_calc_iq__vdx_dn3, locals.var_fn25_calc_iq__vdx_dn4, locals.var_fn25_calc_iq__vdx_dn7, locals.var_fn25_calc_iq__vdx_dn16, locals.var_fn25_calc_iq__vdx_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdx = assign2510_e4189;
        locals.var_fn25_calc_iq__vdx_dn2 = assign2510_e4189_d_n2;
        locals.var_fn25_calc_iq__vdx_dn3 = assign2510_e4189_d_n3;
        locals.var_fn25_calc_iq__vdx_dn4 = assign2510_e4189_d_n4;
        locals.var_fn25_calc_iq__vdx_dn7 = assign2510_e4189_d_n7;
        locals.var_fn25_calc_iq__vdx_dn16 = assign2510_e4189_d_n16;
        locals.var_fn25_calc_iq__vdx_dn17 = assign2510_e4189_d_n17;

        let (assign2520_e4193, assign2520_e4193_d_n2, assign2520_e4193_d_n3, assign2520_e4193_d_n4, assign2520_e4193_d_n7, assign2520_e4193_d_n16, assign2520_e4193_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__fds, locals.var_fn25_calc_iq__fds_dn2, locals.var_fn25_calc_iq__fds_dn3, locals.var_fn25_calc_iq__fds_dn4, locals.var_fn25_calc_iq__fds_dn7, locals.var_fn25_calc_iq__fds_dn16, locals.var_fn25_calc_iq__fds_dn17,)
    }
};
        locals.var_fn25_calc_iq__fds = assign2520_e4193;
        locals.var_fn25_calc_iq__fds_dn2 = assign2520_e4193_d_n2;
        locals.var_fn25_calc_iq__fds_dn3 = assign2520_e4193_d_n3;
        locals.var_fn25_calc_iq__fds_dn4 = assign2520_e4193_d_n4;
        locals.var_fn25_calc_iq__fds_dn7 = assign2520_e4193_d_n7;
        locals.var_fn25_calc_iq__fds_dn16 = assign2520_e4193_d_n16;
        locals.var_fn25_calc_iq__fds_dn17 = assign2520_e4193_d_n17;

        let (assign2530_e4197, assign2530_e4197_d_n2, assign2530_e4197_d_n3, assign2530_e4197_d_n4, assign2530_e4197_d_n7, assign2530_e4197_d_n16, assign2530_e4197_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vsx, locals.var_fn25_calc_iq__vsx_dn2, locals.var_fn25_calc_iq__vsx_dn3, locals.var_fn25_calc_iq__vsx_dn4, locals.var_fn25_calc_iq__vsx_dn7, locals.var_fn25_calc_iq__vsx_dn16, locals.var_fn25_calc_iq__vsx_dn17,)
    }
};
        locals.var_fn25_calc_iq__vsx = assign2530_e4197;
        locals.var_fn25_calc_iq__vsx_dn2 = assign2530_e4197_d_n2;
        locals.var_fn25_calc_iq__vsx_dn3 = assign2530_e4197_d_n3;
        locals.var_fn25_calc_iq__vsx_dn4 = assign2530_e4197_d_n4;
        locals.var_fn25_calc_iq__vsx_dn7 = assign2530_e4197_d_n7;
        locals.var_fn25_calc_iq__vsx_dn16 = assign2530_e4197_d_n16;
        locals.var_fn25_calc_iq__vsx_dn17 = assign2530_e4197_d_n17;

        let (assign2540_e4201, assign2540_e4201_d_n2, assign2540_e4201_d_n3, assign2540_e4201_d_n4, assign2540_e4201_d_n7, assign2540_e4201_d_n16, assign2540_e4201_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffd, locals.var_fn25_calc_iq__ffd_dn2, locals.var_fn25_calc_iq__ffd_dn3, locals.var_fn25_calc_iq__ffd_dn4, locals.var_fn25_calc_iq__ffd_dn7, locals.var_fn25_calc_iq__ffd_dn16, locals.var_fn25_calc_iq__ffd_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd = assign2540_e4201;
        locals.var_fn25_calc_iq__ffd_dn2 = assign2540_e4201_d_n2;
        locals.var_fn25_calc_iq__ffd_dn3 = assign2540_e4201_d_n3;
        locals.var_fn25_calc_iq__ffd_dn4 = assign2540_e4201_d_n4;
        locals.var_fn25_calc_iq__ffd_dn7 = assign2540_e4201_d_n7;
        locals.var_fn25_calc_iq__ffd_dn16 = assign2540_e4201_d_n16;
        locals.var_fn25_calc_iq__ffd_dn17 = assign2540_e4201_d_n17;

        let (assign2550_e4205, assign2550_e4205_d_n2, assign2550_e4205_d_n3, assign2550_e4205_d_n4, assign2550_e4205_d_n7, assign2550_e4205_d_n16, assign2550_e4205_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__etad, locals.var_fn25_calc_iq__etad_dn2, locals.var_fn25_calc_iq__etad_dn3, locals.var_fn25_calc_iq__etad_dn4, locals.var_fn25_calc_iq__etad_dn7, locals.var_fn25_calc_iq__etad_dn16, locals.var_fn25_calc_iq__etad_dn17,)
    }
};
        locals.var_fn25_calc_iq__etad = assign2550_e4205;
        locals.var_fn25_calc_iq__etad_dn2 = assign2550_e4205_d_n2;
        locals.var_fn25_calc_iq__etad_dn3 = assign2550_e4205_d_n3;
        locals.var_fn25_calc_iq__etad_dn4 = assign2550_e4205_d_n4;
        locals.var_fn25_calc_iq__etad_dn7 = assign2550_e4205_d_n7;
        locals.var_fn25_calc_iq__etad_dn16 = assign2550_e4205_d_n16;
        locals.var_fn25_calc_iq__etad_dn17 = assign2550_e4205_d_n17;

        let (assign2560_e4209, assign2560_e4209_d_n2, assign2560_e4209_d_n3, assign2560_e4209_d_n4, assign2560_e4209_d_n7, assign2560_e4209_d_n16, assign2560_e4209_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qinvd, locals.var_fn25_calc_iq__qinvd_dn2, locals.var_fn25_calc_iq__qinvd_dn3, locals.var_fn25_calc_iq__qinvd_dn4, locals.var_fn25_calc_iq__qinvd_dn7, locals.var_fn25_calc_iq__qinvd_dn16, locals.var_fn25_calc_iq__qinvd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd = assign2560_e4209;
        locals.var_fn25_calc_iq__qinvd_dn2 = assign2560_e4209_d_n2;
        locals.var_fn25_calc_iq__qinvd_dn3 = assign2560_e4209_d_n3;
        locals.var_fn25_calc_iq__qinvd_dn4 = assign2560_e4209_d_n4;
        locals.var_fn25_calc_iq__qinvd_dn7 = assign2560_e4209_d_n7;
        locals.var_fn25_calc_iq__qinvd_dn16 = assign2560_e4209_d_n16;
        locals.var_fn25_calc_iq__qinvd_dn17 = assign2560_e4209_d_n17;

        let (assign2570_e4213, assign2570_e4213_d_n2, assign2570_e4213_d_n3, assign2570_e4213_d_n4, assign2570_e4213_d_n7, assign2570_e4213_d_n16, assign2570_e4213_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsc, locals.var_fn25_calc_iq__vdsc_dn2, locals.var_fn25_calc_iq__vdsc_dn3, locals.var_fn25_calc_iq__vdsc_dn4, locals.var_fn25_calc_iq__vdsc_dn7, locals.var_fn25_calc_iq__vdsc_dn16, locals.var_fn25_calc_iq__vdsc_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsc = assign2570_e4213;
        locals.var_fn25_calc_iq__vdsc_dn2 = assign2570_e4213_d_n2;
        locals.var_fn25_calc_iq__vdsc_dn3 = assign2570_e4213_d_n3;
        locals.var_fn25_calc_iq__vdsc_dn4 = assign2570_e4213_d_n4;
        locals.var_fn25_calc_iq__vdsc_dn7 = assign2570_e4213_d_n7;
        locals.var_fn25_calc_iq__vdsc_dn16 = assign2570_e4213_d_n16;
        locals.var_fn25_calc_iq__vdsc_dn17 = assign2570_e4213_d_n17;

        let (assign2580_e4217, assign2580_e4217_d_n2, assign2580_e4217_d_n3, assign2580_e4217_d_n4, assign2580_e4217_d_n7, assign2580_e4217_d_n16, assign2580_e4217_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__fsat, locals.var_fn25_calc_iq__fsat_dn2, locals.var_fn25_calc_iq__fsat_dn3, locals.var_fn25_calc_iq__fsat_dn4, locals.var_fn25_calc_iq__fsat_dn7, locals.var_fn25_calc_iq__fsat_dn16, locals.var_fn25_calc_iq__fsat_dn17,)
    }
};
        locals.var_fn25_calc_iq__fsat = assign2580_e4217;
        locals.var_fn25_calc_iq__fsat_dn2 = assign2580_e4217_d_n2;
        locals.var_fn25_calc_iq__fsat_dn3 = assign2580_e4217_d_n3;
        locals.var_fn25_calc_iq__fsat_dn4 = assign2580_e4217_d_n4;
        locals.var_fn25_calc_iq__fsat_dn7 = assign2580_e4217_d_n7;
        locals.var_fn25_calc_iq__fsat_dn16 = assign2580_e4217_d_n16;
        locals.var_fn25_calc_iq__fsat_dn17 = assign2580_e4217_d_n17;

        let (assign2590_e4221, assign2590_e4221_d_n2, assign2590_e4221_d_n3, assign2590_e4221_d_n4, assign2590_e4221_d_n7, assign2590_e4221_d_n16, assign2590_e4221_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vel, locals.var_fn25_calc_iq__vel_dn2, locals.var_fn25_calc_iq__vel_dn3, locals.var_fn25_calc_iq__vel_dn4, locals.var_fn25_calc_iq__vel_dn7, locals.var_fn25_calc_iq__vel_dn16, locals.var_fn25_calc_iq__vel_dn17,)
    }
};
        locals.var_fn25_calc_iq__vel = assign2590_e4221;
        locals.var_fn25_calc_iq__vel_dn2 = assign2590_e4221_d_n2;
        locals.var_fn25_calc_iq__vel_dn3 = assign2590_e4221_d_n3;
        locals.var_fn25_calc_iq__vel_dn4 = assign2590_e4221_d_n4;
        locals.var_fn25_calc_iq__vel_dn7 = assign2590_e4221_d_n7;
        locals.var_fn25_calc_iq__vel_dn16 = assign2590_e4221_d_n16;
        locals.var_fn25_calc_iq__vel_dn17 = assign2590_e4221_d_n17;

        let (assign2600_e4225, assign2600_e4225_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsats0, locals.var_fn25_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn25_calc_iq__vdsats0 = assign2600_e4225;
        locals.var_fn25_calc_iq__vdsats0_dn4 = assign2600_e4225_d_n4;

        let (assign2610_e4229, assign2610_e4229_d_n2, assign2610_e4229_d_n4, assign2610_e4229_d_n7, assign2610_e4229_d_n16, assign2610_e4229_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsats10, locals.var_fn25_calc_iq__vdsats10_dn2, locals.var_fn25_calc_iq__vdsats10_dn4, locals.var_fn25_calc_iq__vdsats10_dn7, locals.var_fn25_calc_iq__vdsats10_dn16, locals.var_fn25_calc_iq__vdsats10_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsats10 = assign2610_e4229;
        locals.var_fn25_calc_iq__vdsats10_dn2 = assign2610_e4229_d_n2;
        locals.var_fn25_calc_iq__vdsats10_dn4 = assign2610_e4229_d_n4;
        locals.var_fn25_calc_iq__vdsats10_dn7 = assign2610_e4229_d_n7;
        locals.var_fn25_calc_iq__vdsats10_dn16 = assign2610_e4229_d_n16;
        locals.var_fn25_calc_iq__vdsats10_dn17 = assign2610_e4229_d_n17;

        let (assign2620_e4233, assign2620_e4233_d_n2, assign2620_e4233_d_n4, assign2620_e4233_d_n7, assign2620_e4233_d_n16, assign2620_e4233_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdsat10, locals.var_fn25_calc_iq__vdsat10_dn2, locals.var_fn25_calc_iq__vdsat10_dn4, locals.var_fn25_calc_iq__vdsat10_dn7, locals.var_fn25_calc_iq__vdsat10_dn16, locals.var_fn25_calc_iq__vdsat10_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsat10 = assign2620_e4233;
        locals.var_fn25_calc_iq__vdsat10_dn2 = assign2620_e4233_d_n2;
        locals.var_fn25_calc_iq__vdsat10_dn4 = assign2620_e4233_d_n4;
        locals.var_fn25_calc_iq__vdsat10_dn7 = assign2620_e4233_d_n7;
        locals.var_fn25_calc_iq__vdsat10_dn16 = assign2620_e4233_d_n16;
        locals.var_fn25_calc_iq__vdsat10_dn17 = assign2620_e4233_d_n17;

        let (assign2630_e4237, assign2630_e4237_d_n2, assign2630_e4237_d_n4, assign2630_e4237_d_n7, assign2630_e4237_d_n16, assign2630_e4237_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__fsd0, locals.var_fn25_calc_iq__fsd0_dn2, locals.var_fn25_calc_iq__fsd0_dn4, locals.var_fn25_calc_iq__fsd0_dn7, locals.var_fn25_calc_iq__fsd0_dn16, locals.var_fn25_calc_iq__fsd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__fsd0 = assign2630_e4237;
        locals.var_fn25_calc_iq__fsd0_dn2 = assign2630_e4237_d_n2;
        locals.var_fn25_calc_iq__fsd0_dn4 = assign2630_e4237_d_n4;
        locals.var_fn25_calc_iq__fsd0_dn7 = assign2630_e4237_d_n7;
        locals.var_fn25_calc_iq__fsd0_dn16 = assign2630_e4237_d_n16;
        locals.var_fn25_calc_iq__fsd0_dn17 = assign2630_e4237_d_n17;

        let (assign2640_e4241, assign2640_e4241_d_n2, assign2640_e4241_d_n4, assign2640_e4241_d_n7, assign2640_e4241_d_n16, assign2640_e4241_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vdx0, locals.var_fn25_calc_iq__vdx0_dn2, locals.var_fn25_calc_iq__vdx0_dn4, locals.var_fn25_calc_iq__vdx0_dn7, locals.var_fn25_calc_iq__vdx0_dn16, locals.var_fn25_calc_iq__vdx0_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdx0 = assign2640_e4241;
        locals.var_fn25_calc_iq__vdx0_dn2 = assign2640_e4241_d_n2;
        locals.var_fn25_calc_iq__vdx0_dn4 = assign2640_e4241_d_n4;
        locals.var_fn25_calc_iq__vdx0_dn7 = assign2640_e4241_d_n7;
        locals.var_fn25_calc_iq__vdx0_dn16 = assign2640_e4241_d_n16;
        locals.var_fn25_calc_iq__vdx0_dn17 = assign2640_e4241_d_n17;

        let (assign2650_e4245, assign2650_e4245_d_n2, assign2650_e4245_d_n4, assign2650_e4245_d_n7, assign2650_e4245_d_n16, assign2650_e4245_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__fds0, locals.var_fn25_calc_iq__fds0_dn2, locals.var_fn25_calc_iq__fds0_dn4, locals.var_fn25_calc_iq__fds0_dn7, locals.var_fn25_calc_iq__fds0_dn16, locals.var_fn25_calc_iq__fds0_dn17,)
    }
};
        locals.var_fn25_calc_iq__fds0 = assign2650_e4245;
        locals.var_fn25_calc_iq__fds0_dn2 = assign2650_e4245_d_n2;
        locals.var_fn25_calc_iq__fds0_dn4 = assign2650_e4245_d_n4;
        locals.var_fn25_calc_iq__fds0_dn7 = assign2650_e4245_d_n7;
        locals.var_fn25_calc_iq__fds0_dn16 = assign2650_e4245_d_n16;
        locals.var_fn25_calc_iq__fds0_dn17 = assign2650_e4245_d_n17;

        let (assign2660_e4249, assign2660_e4249_d_n2, assign2660_e4249_d_n4, assign2660_e4249_d_n7, assign2660_e4249_d_n16, assign2660_e4249_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vsx0, locals.var_fn25_calc_iq__vsx0_dn2, locals.var_fn25_calc_iq__vsx0_dn4, locals.var_fn25_calc_iq__vsx0_dn7, locals.var_fn25_calc_iq__vsx0_dn16, locals.var_fn25_calc_iq__vsx0_dn17,)
    }
};
        locals.var_fn25_calc_iq__vsx0 = assign2660_e4249;
        locals.var_fn25_calc_iq__vsx0_dn2 = assign2660_e4249_d_n2;
        locals.var_fn25_calc_iq__vsx0_dn4 = assign2660_e4249_d_n4;
        locals.var_fn25_calc_iq__vsx0_dn7 = assign2660_e4249_d_n7;
        locals.var_fn25_calc_iq__vsx0_dn16 = assign2660_e4249_d_n16;
        locals.var_fn25_calc_iq__vsx0_dn17 = assign2660_e4249_d_n17;

        let (assign2670_e4253, assign2670_e4253_d_n2, assign2670_e4253_d_n4, assign2670_e4253_d_n7, assign2670_e4253_d_n16, assign2670_e4253_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffd0, locals.var_fn25_calc_iq__ffd0_dn2, locals.var_fn25_calc_iq__ffd0_dn4, locals.var_fn25_calc_iq__ffd0_dn7, locals.var_fn25_calc_iq__ffd0_dn16, locals.var_fn25_calc_iq__ffd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd0 = assign2670_e4253;
        locals.var_fn25_calc_iq__ffd0_dn2 = assign2670_e4253_d_n2;
        locals.var_fn25_calc_iq__ffd0_dn4 = assign2670_e4253_d_n4;
        locals.var_fn25_calc_iq__ffd0_dn7 = assign2670_e4253_d_n7;
        locals.var_fn25_calc_iq__ffd0_dn16 = assign2670_e4253_d_n16;
        locals.var_fn25_calc_iq__ffd0_dn17 = assign2670_e4253_d_n17;

        let (assign2680_e4257, assign2680_e4257_d_n2, assign2680_e4257_d_n4, assign2680_e4257_d_n7, assign2680_e4257_d_n16, assign2680_e4257_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__etad0, locals.var_fn25_calc_iq__etad0_dn2, locals.var_fn25_calc_iq__etad0_dn4, locals.var_fn25_calc_iq__etad0_dn7, locals.var_fn25_calc_iq__etad0_dn16, locals.var_fn25_calc_iq__etad0_dn17,)
    }
};
        locals.var_fn25_calc_iq__etad0 = assign2680_e4257;
        locals.var_fn25_calc_iq__etad0_dn2 = assign2680_e4257_d_n2;
        locals.var_fn25_calc_iq__etad0_dn4 = assign2680_e4257_d_n4;
        locals.var_fn25_calc_iq__etad0_dn7 = assign2680_e4257_d_n7;
        locals.var_fn25_calc_iq__etad0_dn16 = assign2680_e4257_d_n16;
        locals.var_fn25_calc_iq__etad0_dn17 = assign2680_e4257_d_n17;

        let (assign2690_e4261, assign2690_e4261_d_n2, assign2690_e4261_d_n4, assign2690_e4261_d_n7, assign2690_e4261_d_n16, assign2690_e4261_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qinvd0, locals.var_fn25_calc_iq__qinvd0_dn2, locals.var_fn25_calc_iq__qinvd0_dn4, locals.var_fn25_calc_iq__qinvd0_dn7, locals.var_fn25_calc_iq__qinvd0_dn16, locals.var_fn25_calc_iq__qinvd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd0 = assign2690_e4261;
        locals.var_fn25_calc_iq__qinvd0_dn2 = assign2690_e4261_d_n2;
        locals.var_fn25_calc_iq__qinvd0_dn4 = assign2690_e4261_d_n4;
        locals.var_fn25_calc_iq__qinvd0_dn7 = assign2690_e4261_d_n7;
        locals.var_fn25_calc_iq__qinvd0_dn16 = assign2690_e4261_d_n16;
        locals.var_fn25_calc_iq__qinvd0_dn17 = assign2690_e4261_d_n17;

        let (assign2700_e4265, assign2700_e4265_d_n2, assign2700_e4265_d_n4, assign2700_e4265_d_n7, assign2700_e4265_d_n16, assign2700_e4265_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qs2, locals.var_fn25_calc_iq__qs2_dn2, locals.var_fn25_calc_iq__qs2_dn4, locals.var_fn25_calc_iq__qs2_dn7, locals.var_fn25_calc_iq__qs2_dn16, locals.var_fn25_calc_iq__qs2_dn17,)
    }
};
        locals.var_fn25_calc_iq__qs2 = assign2700_e4265;
        locals.var_fn25_calc_iq__qs2_dn2 = assign2700_e4265_d_n2;
        locals.var_fn25_calc_iq__qs2_dn4 = assign2700_e4265_d_n4;
        locals.var_fn25_calc_iq__qs2_dn7 = assign2700_e4265_d_n7;
        locals.var_fn25_calc_iq__qs2_dn16 = assign2700_e4265_d_n16;
        locals.var_fn25_calc_iq__qs2_dn17 = assign2700_e4265_d_n17;

        let (assign2710_e4269, assign2710_e4269_d_n2, assign2710_e4269_d_n4, assign2710_e4269_d_n7, assign2710_e4269_d_n16, assign2710_e4269_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qs3, locals.var_fn25_calc_iq__qs3_dn2, locals.var_fn25_calc_iq__qs3_dn4, locals.var_fn25_calc_iq__qs3_dn7, locals.var_fn25_calc_iq__qs3_dn16, locals.var_fn25_calc_iq__qs3_dn17,)
    }
};
        locals.var_fn25_calc_iq__qs3 = assign2710_e4269;
        locals.var_fn25_calc_iq__qs3_dn2 = assign2710_e4269_d_n2;
        locals.var_fn25_calc_iq__qs3_dn4 = assign2710_e4269_d_n4;
        locals.var_fn25_calc_iq__qs3_dn7 = assign2710_e4269_d_n7;
        locals.var_fn25_calc_iq__qs3_dn16 = assign2710_e4269_d_n16;
        locals.var_fn25_calc_iq__qs3_dn17 = assign2710_e4269_d_n17;

        let (assign2720_e4273, assign2720_e4273_d_n2, assign2720_e4273_d_n4, assign2720_e4273_d_n7, assign2720_e4273_d_n16, assign2720_e4273_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qd2, locals.var_fn25_calc_iq__qd2_dn2, locals.var_fn25_calc_iq__qd2_dn4, locals.var_fn25_calc_iq__qd2_dn7, locals.var_fn25_calc_iq__qd2_dn16, locals.var_fn25_calc_iq__qd2_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd2 = assign2720_e4273;
        locals.var_fn25_calc_iq__qd2_dn2 = assign2720_e4273_d_n2;
        locals.var_fn25_calc_iq__qd2_dn4 = assign2720_e4273_d_n4;
        locals.var_fn25_calc_iq__qd2_dn7 = assign2720_e4273_d_n7;
        locals.var_fn25_calc_iq__qd2_dn16 = assign2720_e4273_d_n16;
        locals.var_fn25_calc_iq__qd2_dn17 = assign2720_e4273_d_n17;

        let (assign2730_e4277, assign2730_e4277_d_n2, assign2730_e4277_d_n4, assign2730_e4277_d_n7, assign2730_e4277_d_n16, assign2730_e4277_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qd3, locals.var_fn25_calc_iq__qd3_dn2, locals.var_fn25_calc_iq__qd3_dn4, locals.var_fn25_calc_iq__qd3_dn7, locals.var_fn25_calc_iq__qd3_dn16, locals.var_fn25_calc_iq__qd3_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd3 = assign2730_e4277;
        locals.var_fn25_calc_iq__qd3_dn2 = assign2730_e4277_d_n2;
        locals.var_fn25_calc_iq__qd3_dn4 = assign2730_e4277_d_n4;
        locals.var_fn25_calc_iq__qd3_dn7 = assign2730_e4277_d_n7;
        locals.var_fn25_calc_iq__qd3_dn16 = assign2730_e4277_d_n16;
        locals.var_fn25_calc_iq__qd3_dn17 = assign2730_e4277_d_n17;

        let (assign2740_e4281, assign2740_e4281_d_n2, assign2740_e4281_d_n4, assign2740_e4281_d_n7, assign2740_e4281_d_n16, assign2740_e4281_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qsqd, locals.var_fn25_calc_iq__qsqd_dn2, locals.var_fn25_calc_iq__qsqd_dn4, locals.var_fn25_calc_iq__qsqd_dn7, locals.var_fn25_calc_iq__qsqd_dn16, locals.var_fn25_calc_iq__qsqd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qsqd = assign2740_e4281;
        locals.var_fn25_calc_iq__qsqd_dn2 = assign2740_e4281_d_n2;
        locals.var_fn25_calc_iq__qsqd_dn4 = assign2740_e4281_d_n4;
        locals.var_fn25_calc_iq__qsqd_dn7 = assign2740_e4281_d_n7;
        locals.var_fn25_calc_iq__qsqd_dn16 = assign2740_e4281_d_n16;
        locals.var_fn25_calc_iq__qsqd_dn17 = assign2740_e4281_d_n17;

        let (assign2750_e4285, assign2750_e4285_d_n2, assign2750_e4285_d_n4, assign2750_e4285_d_n7, assign2750_e4285_d_n16, assign2750_e4285_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qinvdd, locals.var_fn25_calc_iq__qinvdd_dn2, locals.var_fn25_calc_iq__qinvdd_dn4, locals.var_fn25_calc_iq__qinvdd_dn7, locals.var_fn25_calc_iq__qinvdd_dn16, locals.var_fn25_calc_iq__qinvdd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvdd = assign2750_e4285;
        locals.var_fn25_calc_iq__qinvdd_dn2 = assign2750_e4285_d_n2;
        locals.var_fn25_calc_iq__qinvdd_dn4 = assign2750_e4285_d_n4;
        locals.var_fn25_calc_iq__qinvdd_dn7 = assign2750_e4285_d_n7;
        locals.var_fn25_calc_iq__qinvdd_dn16 = assign2750_e4285_d_n16;
        locals.var_fn25_calc_iq__qinvdd_dn17 = assign2750_e4285_d_n17;

        let (assign2760_e4289, assign2760_e4289_d_n2, assign2760_e4289_d_n4, assign2760_e4289_d_n7, assign2760_e4289_d_n16, assign2760_e4289_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qd1, locals.var_fn25_calc_iq__qd1_dn2, locals.var_fn25_calc_iq__qd1_dn4, locals.var_fn25_calc_iq__qd1_dn7, locals.var_fn25_calc_iq__qd1_dn16, locals.var_fn25_calc_iq__qd1_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd1 = assign2760_e4289;
        locals.var_fn25_calc_iq__qd1_dn2 = assign2760_e4289_d_n2;
        locals.var_fn25_calc_iq__qd1_dn4 = assign2760_e4289_d_n4;
        locals.var_fn25_calc_iq__qd1_dn7 = assign2760_e4289_d_n7;
        locals.var_fn25_calc_iq__qd1_dn16 = assign2760_e4289_d_n16;
        locals.var_fn25_calc_iq__qd1_dn17 = assign2760_e4289_d_n17;

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2770_e4293, assign2770_e4293_d_n2, assign2770_e4293_d_n4, assign2770_e4293_d_n7, assign2770_e4293_d_n16, assign2770_e4293_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qs, locals.var_fn25_calc_iq__qs_dn2, locals.var_fn25_calc_iq__qs_dn4, locals.var_fn25_calc_iq__qs_dn7, locals.var_fn25_calc_iq__qs_dn16, locals.var_fn25_calc_iq__qs_dn17,)
    }
};
        locals.var_fn25_calc_iq__qs = assign2770_e4293;
        locals.var_fn25_calc_iq__qs_dn2 = assign2770_e4293_d_n2;
        locals.var_fn25_calc_iq__qs_dn4 = assign2770_e4293_d_n4;
        locals.var_fn25_calc_iq__qs_dn7 = assign2770_e4293_d_n7;
        locals.var_fn25_calc_iq__qs_dn16 = assign2770_e4293_d_n16;
        locals.var_fn25_calc_iq__qs_dn17 = assign2770_e4293_d_n17;

        let (assign2780_e4297, assign2780_e4297_d_n2, assign2780_e4297_d_n4, assign2780_e4297_d_n7, assign2780_e4297_d_n16, assign2780_e4297_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qd, locals.var_fn25_calc_iq__qd_dn2, locals.var_fn25_calc_iq__qd_dn4, locals.var_fn25_calc_iq__qd_dn7, locals.var_fn25_calc_iq__qd_dn16, locals.var_fn25_calc_iq__qd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd = assign2780_e4297;
        locals.var_fn25_calc_iq__qd_dn2 = assign2780_e4297_d_n2;
        locals.var_fn25_calc_iq__qd_dn4 = assign2780_e4297_d_n4;
        locals.var_fn25_calc_iq__qd_dn7 = assign2780_e4297_d_n7;
        locals.var_fn25_calc_iq__qd_dn16 = assign2780_e4297_d_n16;
        locals.var_fn25_calc_iq__qd_dn17 = assign2780_e4297_d_n17;

        let (assign2790_e4301, assign2790_e4301_d_n2, assign2790_e4301_d_n4, assign2790_e4301_d_n7, assign2790_e4301_d_n16,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__etac, locals.var_fn25_calc_iq__etac_dn2, locals.var_fn25_calc_iq__etac_dn4, locals.var_fn25_calc_iq__etac_dn7, locals.var_fn25_calc_iq__etac_dn16,)
    }
};
        locals.var_fn25_calc_iq__etac = assign2790_e4301;
        locals.var_fn25_calc_iq__etac_dn2 = assign2790_e4301_d_n2;
        locals.var_fn25_calc_iq__etac_dn4 = assign2790_e4301_d_n4;
        locals.var_fn25_calc_iq__etac_dn7 = assign2790_e4301_d_n7;
        locals.var_fn25_calc_iq__etac_dn16 = assign2790_e4301_d_n16;

        let (assign2800_e4305, assign2800_e4305_d_n3, assign2800_e4305_d_n4, assign2800_e4305_d_n16,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__etab, locals.var_fn25_calc_iq__etab_dn3, locals.var_fn25_calc_iq__etab_dn4, locals.var_fn25_calc_iq__etab_dn16,)
    }
};
        locals.var_fn25_calc_iq__etab = assign2800_e4305;
        locals.var_fn25_calc_iq__etab_dn3 = assign2800_e4305_d_n3;
        locals.var_fn25_calc_iq__etab_dn4 = assign2800_e4305_d_n4;
        locals.var_fn25_calc_iq__etab_dn16 = assign2800_e4305_d_n16;

        let (assign2810_e4309, assign2810_e4309_d_n2, assign2810_e4309_d_n4, assign2810_e4309_d_n7, assign2810_e4309_d_n16,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__etags, locals.var_fn25_calc_iq__etags_dn2, locals.var_fn25_calc_iq__etags_dn4, locals.var_fn25_calc_iq__etags_dn7, locals.var_fn25_calc_iq__etags_dn16,)
    }
};
        locals.var_fn25_calc_iq__etags = assign2810_e4309;
        locals.var_fn25_calc_iq__etags_dn2 = assign2810_e4309_d_n2;
        locals.var_fn25_calc_iq__etags_dn4 = assign2810_e4309_d_n4;
        locals.var_fn25_calc_iq__etags_dn7 = assign2810_e4309_d_n7;
        locals.var_fn25_calc_iq__etags_dn16 = assign2810_e4309_d_n16;

        let (assign2820_e4313, assign2820_e4313_d_n2, assign2820_e4313_d_n3, assign2820_e4313_d_n4, assign2820_e4313_d_n7, assign2820_e4313_d_n16, assign2820_e4313_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign2820_e4313;
        locals.var_fn25_calc_iq__exparg_dn2 = assign2820_e4313_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign2820_e4313_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign2820_e4313_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign2820_e4313_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign2820_e4313_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign2820_e4313_d_n17;

        let (assign2830_e4317, assign2830_e4317_d_n2, assign2830_e4317_d_n3, assign2830_e4317_d_n4, assign2830_e4317_d_n7, assign2830_e4317_d_n16, assign2830_e4317_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__myarg, locals.var_fn25_calc_iq__myarg_dn2, locals.var_fn25_calc_iq__myarg_dn3, locals.var_fn25_calc_iq__myarg_dn4, locals.var_fn25_calc_iq__myarg_dn7, locals.var_fn25_calc_iq__myarg_dn16, locals.var_fn25_calc_iq__myarg_dn17,)
    }
};
        locals.var_fn25_calc_iq__myarg = assign2830_e4317;
        locals.var_fn25_calc_iq__myarg_dn2 = assign2830_e4317_d_n2;
        locals.var_fn25_calc_iq__myarg_dn3 = assign2830_e4317_d_n3;
        locals.var_fn25_calc_iq__myarg_dn4 = assign2830_e4317_d_n4;
        locals.var_fn25_calc_iq__myarg_dn7 = assign2830_e4317_d_n7;
        locals.var_fn25_calc_iq__myarg_dn16 = assign2830_e4317_d_n16;
        locals.var_fn25_calc_iq__myarg_dn17 = assign2830_e4317_d_n17;

        let (assign2840_e4321, assign2840_e4321_d_n16, assign2840_e4321_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__absvdsin, locals.var_fn25_calc_iq__absvdsin_dn16, locals.var_fn25_calc_iq__absvdsin_dn17,)
    }
};
        locals.var_fn25_calc_iq__absvdsin = assign2840_e4321;
        locals.var_fn25_calc_iq__absvdsin_dn16 = assign2840_e4321_d_n16;
        locals.var_fn25_calc_iq__absvdsin_dn17 = assign2840_e4321_d_n17;

        let (assign2850_e4325, assign2850_e4325_d_n2, assign2850_e4325_d_n7, assign2850_e4325_d_n16, assign2850_e4325_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vgdin, locals.var_fn25_calc_iq__vgdin_dn2, locals.var_fn25_calc_iq__vgdin_dn7, locals.var_fn25_calc_iq__vgdin_dn16, locals.var_fn25_calc_iq__vgdin_dn17,)
    }
};
        locals.var_fn25_calc_iq__vgdin = assign2850_e4325;
        locals.var_fn25_calc_iq__vgdin_dn2 = assign2850_e4325_d_n2;
        locals.var_fn25_calc_iq__vgdin_dn7 = assign2850_e4325_d_n7;
        locals.var_fn25_calc_iq__vgdin_dn16 = assign2850_e4325_d_n16;
        locals.var_fn25_calc_iq__vgdin_dn17 = assign2850_e4325_d_n17;

        let (assign2860_e4329, assign2860_e4329_d_n2, assign2860_e4329_d_n4, assign2860_e4329_d_n7, assign2860_e4329_d_n16, assign2860_e4329_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg0, locals.var_fn25_calc_iq__exparg0_dn2, locals.var_fn25_calc_iq__exparg0_dn4, locals.var_fn25_calc_iq__exparg0_dn7, locals.var_fn25_calc_iq__exparg0_dn16, locals.var_fn25_calc_iq__exparg0_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg0 = assign2860_e4329;
        locals.var_fn25_calc_iq__exparg0_dn2 = assign2860_e4329_d_n2;
        locals.var_fn25_calc_iq__exparg0_dn4 = assign2860_e4329_d_n4;
        locals.var_fn25_calc_iq__exparg0_dn7 = assign2860_e4329_d_n7;
        locals.var_fn25_calc_iq__exparg0_dn16 = assign2860_e4329_d_n16;
        locals.var_fn25_calc_iq__exparg0_dn17 = assign2860_e4329_d_n17;

        let (assign2870_e4333, assign2870_e4333_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__myarg0, locals.var_fn25_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn25_calc_iq__myarg0 = assign2870_e4333;
        locals.var_fn25_calc_iq__myarg0_dn4 = assign2870_e4333_d_n4;

        let (assign2880_e4360, assign2880_e4360_d_n16, assign2880_e4360_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign2880_e4358, assign2880_e4358_d_n16, assign2880_e4358_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign2880_e4342: f64 = (0.001 / p.p53);
                let assign2880_e4344: f64 = (assign2880_e4342 * locals.var_fn25_calc_iq__vdsin);
                let assign2880_e4345: f64 = (assign2880_e4344).tanh();
                let assign2880_e4346: f64 = (locals.var_fn25_calc_iq__vdsin * assign2880_e4345);
                (assign2880_e4346, ((locals.var_fn25_calc_iq__vdsin_dn16 * assign2880_e4345) + (locals.var_fn25_calc_iq__vdsin * ((assign2880_e4342 * locals.var_fn25_calc_iq__vdsin_dn16) / ((assign2880_e4344).cosh() * (assign2880_e4344).cosh())))), ((locals.var_fn25_calc_iq__vdsin_dn17 * assign2880_e4345) + (locals.var_fn25_calc_iq__vdsin * ((assign2880_e4342 * locals.var_fn25_calc_iq__vdsin_dn17) / ((assign2880_e4344).cosh() * (assign2880_e4344).cosh())))),)
            } else {
                let (assign2880_e4357, assign2880_e4357_d_n16, assign2880_e4357_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign2880_e4352: f64 = (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsin);
                        let assign2880_e4354: f64 = (assign2880_e4352 + p.p53);
                        let assign2880_e4355: f64 = (assign2880_e4354).sqrt();
                        (assign2880_e4355, (((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsin) + (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsin_dn16)) / (2.0 * assign2880_e4355)), (((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsin) + (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsin_dn17)) / (2.0 * assign2880_e4355)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign2880_e4357, assign2880_e4357_d_n16, assign2880_e4357_d_n17,)
            }
        };
        (assign2880_e4358, assign2880_e4358_d_n16, assign2880_e4358_d_n17,)
    } else {
        (locals.var_fn25_calc_iq__absvdsin, locals.var_fn25_calc_iq__absvdsin_dn16, locals.var_fn25_calc_iq__absvdsin_dn17,)
    }
};
        locals.var_fn25_calc_iq__absvdsin = assign2880_e4360;
        locals.var_fn25_calc_iq__absvdsin_dn16 = assign2880_e4360_d_n16;
        locals.var_fn25_calc_iq__absvdsin_dn17 = assign2880_e4360_d_n17;

        let (assign2890_e4366, assign2890_e4366_d_n2, assign2890_e4366_d_n7, assign2890_e4366_d_n16, assign2890_e4366_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2890_e4364: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vdsin);
        (assign2890_e4364, locals.var_fn25_calc_iq__vgsin_dn2, locals.var_fn25_calc_iq__vgsin_dn7, (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vdsin_dn16), (-locals.var_fn25_calc_iq__vdsin_dn17),)
    } else {
        (locals.var_fn25_calc_iq__vgdin, locals.var_fn25_calc_iq__vgdin_dn2, locals.var_fn25_calc_iq__vgdin_dn7, locals.var_fn25_calc_iq__vgdin_dn16, locals.var_fn25_calc_iq__vgdin_dn17,)
    }
};
        locals.var_fn25_calc_iq__vgdin = assign2890_e4366;
        locals.var_fn25_calc_iq__vgdin_dn2 = assign2890_e4366_d_n2;
        locals.var_fn25_calc_iq__vgdin_dn7 = assign2890_e4366_d_n7;
        locals.var_fn25_calc_iq__vgdin_dn16 = assign2890_e4366_d_n16;
        locals.var_fn25_calc_iq__vgdin_dn17 = assign2890_e4366_d_n17;

        let (assign2900_e4372, assign2900_e4372_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2900_e4370: f64 = (locals.var_fn25_calc_iq__alpha * locals.var_fn25_calc_iq__phitin);
        (assign2900_e4370, (locals.var_fn25_calc_iq__alpha * locals.var_fn25_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn25_calc_iq__alpha_phit, locals.var_fn25_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn25_calc_iq__alpha_phit = assign2900_e4372;
        locals.var_fn25_calc_iq__alpha_phit_dn4 = assign2900_e4372_d_n4;

        let (assign2910_e4384, assign2910_e4384_d_n4, assign2910_e4384_d_n16, assign2910_e4384_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2910_e4377: f64 = (2.302585092994046 * locals.var_fn25_calc_iq__phitin);
        let assign2910_e4378: f64 = (locals.var_fn25_calc_iq__ss / assign2910_e4377);
        let assign2910_e4381: f64 = (locals.var_fn25_calc_iq__nd * locals.var_fn25_calc_iq__absvdsin);
        let assign2910_e4382: f64 = (assign2910_e4378 + assign2910_e4381);
        (assign2910_e4382, (-((locals.var_fn25_calc_iq__ss * (2.302585092994046 * locals.var_fn25_calc_iq__phitin_dn4)) / (assign2910_e4377 * assign2910_e4377))), (locals.var_fn25_calc_iq__nd * locals.var_fn25_calc_iq__absvdsin_dn16), (locals.var_fn25_calc_iq__nd * locals.var_fn25_calc_iq__absvdsin_dn17),)
    } else {
        (locals.var_fn25_calc_iq__n, locals.var_fn25_calc_iq__n_dn4, locals.var_fn25_calc_iq__n_dn16, locals.var_fn25_calc_iq__n_dn17,)
    }
};
        locals.var_fn25_calc_iq__n = assign2910_e4384;
        locals.var_fn25_calc_iq__n_dn4 = assign2910_e4384_d_n4;
        locals.var_fn25_calc_iq__n_dn16 = assign2910_e4384_d_n16;
        locals.var_fn25_calc_iq__n_dn17 = assign2910_e4384_d_n17;

        let (assign2920_e4394, assign2920_e4394_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2920_e4390: f64 = (locals.var_fn25_calc_iq__tambin - locals.var_fn25_calc_iq__tnomin);
        let assign2920_e4391: f64 = (locals.var_fn25_calc_iq__vtzeta * assign2920_e4390);
        let assign2920_e4392: f64 = (locals.var_fn25_calc_iq__vto + assign2920_e4391);
        (assign2920_e4392, (locals.var_fn25_calc_iq__vtzeta * locals.var_fn25_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn25_calc_iq__vtof, locals.var_fn25_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn25_calc_iq__vtof = assign2920_e4394;
        locals.var_fn25_calc_iq__vtof_dn4 = assign2920_e4394_d_n4;

        let (assign2930_e4402, assign2930_e4402_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2930_e4398: f64 = (locals.var_fn25_calc_iq__tambin / locals.var_fn25_calc_iq__tnomin);
        let assign2930_e4400: f64 = (assign2930_e4398).powf(locals.var_fn25_calc_iq__epsilon);
        (assign2930_e4400, if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn25_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__epsilon * ((assign2930_e4398).powf(locals.var_fn25_calc_iq__epsilon - 1.0) * (locals.var_fn25_calc_iq__tambin_dn4 / locals.var_fn25_calc_iq__tnomin))) } } else { (assign2930_e4400 * (locals.var_fn25_calc_iq__epsilon * ((locals.var_fn25_calc_iq__tambin_dn4 / locals.var_fn25_calc_iq__tnomin) / assign2930_e4398))) },)
    } else {
        (locals.var_fn25_calc_iq__tfacmobin, locals.var_fn25_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn25_calc_iq__tfacmobin = assign2930_e4402;
        locals.var_fn25_calc_iq__tfacmobin_dn4 = assign2930_e4402_d_n4;

        let assign2940_e4405: f64 = if locals.var_fn25_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign2940_e4405;

        let (assign2950_e4423, assign2950_e4423_d_n16, assign2950_e4423_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard26 != 0.0)) {
        let assign2950_e4413: f64 = (locals.var_fn25_calc_iq__absvdsin / locals.var_fn25_calc_iq__dibsat);
        let assign2950_e4415: f64 = (assign2950_e4413).powf(locals.var_fn25_calc_iq__beta);
        let assign2950_e4416: f64 = (1.0 + assign2950_e4415);
        let assign2950_e4419: f64 = (1.0 / locals.var_fn25_calc_iq__beta);
        let assign2950_e4420: f64 = (assign2950_e4416).powf(assign2950_e4419);
        let assign2950_e4421: f64 = (locals.var_fn25_calc_iq__absvdsin / assign2950_e4420);
        (assign2950_e4421, (((locals.var_fn25_calc_iq__absvdsin_dn16 * assign2950_e4420) - (locals.var_fn25_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign2950_e4419) as f64).is_finite() && ((assign2950_e4419) as f64).fract() == 0.0 { if assign2950_e4419 == 0.0 { 0.0 } else { (assign2950_e4419 * ((assign2950_e4416).powf(assign2950_e4419 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign2950_e4413).powf(locals.var_fn25_calc_iq__beta - 1.0) * (locals.var_fn25_calc_iq__absvdsin_dn16 / locals.var_fn25_calc_iq__dibsat))) } } else { (assign2950_e4415 * (locals.var_fn25_calc_iq__beta * ((locals.var_fn25_calc_iq__absvdsin_dn16 / locals.var_fn25_calc_iq__dibsat) / assign2950_e4413))) })) } } else { (assign2950_e4420 * (assign2950_e4419 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign2950_e4413).powf(locals.var_fn25_calc_iq__beta - 1.0) * (locals.var_fn25_calc_iq__absvdsin_dn16 / locals.var_fn25_calc_iq__dibsat))) } } else { (assign2950_e4415 * (locals.var_fn25_calc_iq__beta * ((locals.var_fn25_calc_iq__absvdsin_dn16 / locals.var_fn25_calc_iq__dibsat) / assign2950_e4413))) } / assign2950_e4416))) })) / (assign2950_e4420 * assign2950_e4420)), (((locals.var_fn25_calc_iq__absvdsin_dn17 * assign2950_e4420) - (locals.var_fn25_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign2950_e4419) as f64).is_finite() && ((assign2950_e4419) as f64).fract() == 0.0 { if assign2950_e4419 == 0.0 { 0.0 } else { (assign2950_e4419 * ((assign2950_e4416).powf(assign2950_e4419 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign2950_e4413).powf(locals.var_fn25_calc_iq__beta - 1.0) * (locals.var_fn25_calc_iq__absvdsin_dn17 / locals.var_fn25_calc_iq__dibsat))) } } else { (assign2950_e4415 * (locals.var_fn25_calc_iq__beta * ((locals.var_fn25_calc_iq__absvdsin_dn17 / locals.var_fn25_calc_iq__dibsat) / assign2950_e4413))) })) } } else { (assign2950_e4420 * (assign2950_e4419 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign2950_e4413).powf(locals.var_fn25_calc_iq__beta - 1.0) * (locals.var_fn25_calc_iq__absvdsin_dn17 / locals.var_fn25_calc_iq__dibsat))) } } else { (assign2950_e4415 * (locals.var_fn25_calc_iq__beta * ((locals.var_fn25_calc_iq__absvdsin_dn17 / locals.var_fn25_calc_iq__dibsat) / assign2950_e4413))) } / assign2950_e4416))) })) / (assign2950_e4420 * assign2950_e4420)),)
    } else {
        (locals.var_fn25_calc_iq__vsatdibl, locals.var_fn25_calc_iq__vsatdibl_dn16, locals.var_fn25_calc_iq__vsatdibl_dn17,)
    }
};
        locals.var_fn25_calc_iq__vsatdibl = assign2950_e4423;
        locals.var_fn25_calc_iq__vsatdibl_dn16 = assign2950_e4423_d_n16;
        locals.var_fn25_calc_iq__vsatdibl_dn17 = assign2950_e4423_d_n17;

        let (assign2960_e4430, assign2960_e4430_d_n16, assign2960_e4430_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard26 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__vsatdibl, locals.var_fn25_calc_iq__vsatdibl_dn16, locals.var_fn25_calc_iq__vsatdibl_dn17,)
    }
};
        locals.var_fn25_calc_iq__vsatdibl = assign2960_e4430;
        locals.var_fn25_calc_iq__vsatdibl_dn16 = assign2960_e4430_d_n16;
        locals.var_fn25_calc_iq__vsatdibl_dn17 = assign2960_e4430_d_n17;

        let (assign2970_e4440, assign2970_e4440_d_n16, assign2970_e4440_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2970_e4435: f64 = (locals.var_fn25_calc_iq__vsatdibl * locals.var_fn25_calc_iq__delta2);
        let assign2970_e4436: f64 = (locals.var_fn25_calc_iq__delta1 - assign2970_e4435);
        let assign2970_e4438: f64 = (assign2970_e4436 * locals.var_fn25_calc_iq__absvdsin);
        (assign2970_e4438, (((-(locals.var_fn25_calc_iq__vsatdibl_dn16 * locals.var_fn25_calc_iq__delta2)) * locals.var_fn25_calc_iq__absvdsin) + (assign2970_e4436 * locals.var_fn25_calc_iq__absvdsin_dn16)), (((-(locals.var_fn25_calc_iq__vsatdibl_dn17 * locals.var_fn25_calc_iq__delta2)) * locals.var_fn25_calc_iq__absvdsin) + (assign2970_e4436 * locals.var_fn25_calc_iq__absvdsin_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__delta, locals.var_fn25_calc_iq__delta_dn16, locals.var_fn25_calc_iq__delta_dn17,)
    }
};
        locals.var_fn25_calc_iq__delta = assign2970_e4440;
        locals.var_fn25_calc_iq__delta_dn16 = assign2970_e4440_d_n16;
        locals.var_fn25_calc_iq__delta_dn17 = assign2970_e4440_d_n17;

        let (assign2980_e4446, assign2980_e4446_d_n4, assign2980_e4446_d_n16, assign2980_e4446_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2980_e4444: f64 = (locals.var_fn25_calc_iq__vtof - locals.var_fn25_calc_iq__delta);
        (assign2980_e4444, locals.var_fn25_calc_iq__vtof_dn4, (-locals.var_fn25_calc_iq__delta_dn16), (-locals.var_fn25_calc_iq__delta_dn17),)
    } else {
        (locals.var_fn25_calc_iq__vtdibl, locals.var_fn25_calc_iq__vtdibl_dn4, locals.var_fn25_calc_iq__vtdibl_dn16, locals.var_fn25_calc_iq__vtdibl_dn17,)
    }
};
        locals.var_fn25_calc_iq__vtdibl = assign2980_e4446;
        locals.var_fn25_calc_iq__vtdibl_dn4 = assign2980_e4446_d_n4;
        locals.var_fn25_calc_iq__vtdibl_dn16 = assign2980_e4446_d_n16;
        locals.var_fn25_calc_iq__vtdibl_dn17 = assign2980_e4446_d_n17;

        let (assign2990_e4454, assign2990_e4454_d_n4, assign2990_e4454_d_n16, assign2990_e4454_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign2990_e4450: f64 = (2.0 * locals.var_fn25_calc_iq__n);
        let assign2990_e4452: f64 = (assign2990_e4450 * locals.var_fn25_calc_iq__phitin);
        (assign2990_e4452, (((2.0 * locals.var_fn25_calc_iq__n_dn4) * locals.var_fn25_calc_iq__phitin) + (assign2990_e4450 * locals.var_fn25_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn25_calc_iq__n_dn16) * locals.var_fn25_calc_iq__phitin), ((2.0 * locals.var_fn25_calc_iq__n_dn17) * locals.var_fn25_calc_iq__phitin),)
    } else {
        (locals.var_fn25_calc_iq__two_n_phit, locals.var_fn25_calc_iq__two_n_phit_dn4, locals.var_fn25_calc_iq__two_n_phit_dn16, locals.var_fn25_calc_iq__two_n_phit_dn17,)
    }
};
        locals.var_fn25_calc_iq__two_n_phit = assign2990_e4454;
        locals.var_fn25_calc_iq__two_n_phit_dn4 = assign2990_e4454_d_n4;
        locals.var_fn25_calc_iq__two_n_phit_dn16 = assign2990_e4454_d_n16;
        locals.var_fn25_calc_iq__two_n_phit_dn17 = assign2990_e4454_d_n17;

        let (assign3000_e4460, assign3000_e4460_d_n4, assign3000_e4460_d_n16, assign3000_e4460_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3000_e4458: f64 = (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__two_n_phit);
        (assign3000_e4458, ((locals.var_fn25_calc_iq__cgin_dn4 * locals.var_fn25_calc_iq__two_n_phit) + (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__two_n_phit_dn4)), (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__two_n_phit_dn16), (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__two_n_phit_dn17),)
    } else {
        (locals.var_fn25_calc_iq__qref, locals.var_fn25_calc_iq__qref_dn4, locals.var_fn25_calc_iq__qref_dn16, locals.var_fn25_calc_iq__qref_dn17,)
    }
};
        locals.var_fn25_calc_iq__qref = assign3000_e4460;
        locals.var_fn25_calc_iq__qref_dn4 = assign3000_e4460_d_n4;
        locals.var_fn25_calc_iq__qref_dn16 = assign3000_e4460_d_n16;
        locals.var_fn25_calc_iq__qref_dn17 = assign3000_e4460_d_n17;

        let (assign3010_e4470, assign3010_e4470_d_n2, assign3010_e4470_d_n3, assign3010_e4470_d_n4, assign3010_e4470_d_n7, assign3010_e4470_d_n16, assign3010_e4470_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3010_e4465: f64 = (p.p51 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3010_e4467: f64 = (assign3010_e4465 / 2.0);
        let assign3010_e4468: f64 = (locals.var_fn25_calc_iq__vtdibl - assign3010_e4467);
        (assign3010_e4468, 0.0, 0.0, (locals.var_fn25_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn25_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn25_calc_iq__vtdibl_dn16, locals.var_fn25_calc_iq__vtdibl_dn17,)
    } else {
        (locals.var_fn25_calc_iq__myarg, locals.var_fn25_calc_iq__myarg_dn2, locals.var_fn25_calc_iq__myarg_dn3, locals.var_fn25_calc_iq__myarg_dn4, locals.var_fn25_calc_iq__myarg_dn7, locals.var_fn25_calc_iq__myarg_dn16, locals.var_fn25_calc_iq__myarg_dn17,)
    }
};
        locals.var_fn25_calc_iq__myarg = assign3010_e4470;
        locals.var_fn25_calc_iq__myarg_dn2 = assign3010_e4470_d_n2;
        locals.var_fn25_calc_iq__myarg_dn3 = assign3010_e4470_d_n3;
        locals.var_fn25_calc_iq__myarg_dn4 = assign3010_e4470_d_n4;
        locals.var_fn25_calc_iq__myarg_dn7 = assign3010_e4470_d_n7;
        locals.var_fn25_calc_iq__myarg_dn16 = assign3010_e4470_d_n16;
        locals.var_fn25_calc_iq__myarg_dn17 = assign3010_e4470_d_n17;

        let (assign3020_e4521, assign3020_e4521_d_n2, assign3020_e4521_d_n3, assign3020_e4521_d_n4, assign3020_e4521_d_n7, assign3020_e4521_d_n16, assign3020_e4521_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3020_e4515, assign3020_e4515_d_n2, assign3020_e4515_d_n7, assign3020_e4515_d_n16, assign3020_e4515_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3020_e4479: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                let assign3020_e4482: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3020_e4485: f64 = (0.001 / p.p53);
                let assign3020_e4488: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3020_e4489: f64 = (assign3020_e4485 * assign3020_e4488);
                let assign3020_e4490: f64 = (assign3020_e4489).tanh();
                let assign3020_e4491: f64 = (assign3020_e4482 * assign3020_e4490);
                let assign3020_e4492: f64 = (assign3020_e4479 + assign3020_e4491);
                let assign3020_e4493: f64 = (0.5 * assign3020_e4492);
                (assign3020_e4493, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + (((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3020_e4490) + (assign3020_e4482 * ((assign3020_e4485 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2)) / ((assign3020_e4489).cosh() * (assign3020_e4489).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + (((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3020_e4490) + (assign3020_e4482 * ((assign3020_e4485 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7)) / ((assign3020_e4489).cosh() * (assign3020_e4489).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + (((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3020_e4490) + (assign3020_e4482 * ((assign3020_e4485 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16)) / ((assign3020_e4489).cosh() * (assign3020_e4489).cosh())))))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + (((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3020_e4490) + (assign3020_e4482 * ((assign3020_e4485 * (-locals.var_fn25_calc_iq__vgdin_dn17)) / ((assign3020_e4489).cosh() * (assign3020_e4489).cosh())))))),)
            } else {
                let (assign3020_e4514, assign3020_e4514_d_n2, assign3020_e4514_d_n7, assign3020_e4514_d_n16, assign3020_e4514_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3020_e4500: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                        let assign3020_e4503: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3020_e4506: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3020_e4507: f64 = (assign3020_e4503 * assign3020_e4506);
                        let assign3020_e4509: f64 = (assign3020_e4507 + p.p53);
                        let assign3020_e4510: f64 = (assign3020_e4509).sqrt();
                        let assign3020_e4511: f64 = (assign3020_e4500 + assign3020_e4510);
                        let assign3020_e4512: f64 = (0.5 * assign3020_e4511);
                        (assign3020_e4512, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + ((((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3020_e4506) + (assign3020_e4503 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2))) / (2.0 * assign3020_e4510)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + ((((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3020_e4506) + (assign3020_e4503 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7))) / (2.0 * assign3020_e4510)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + ((((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3020_e4506) + (assign3020_e4503 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16))) / (2.0 * assign3020_e4510)))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + ((((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3020_e4506) + (assign3020_e4503 * (-locals.var_fn25_calc_iq__vgdin_dn17))) / (2.0 * assign3020_e4510)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3020_e4514, assign3020_e4514_d_n2, assign3020_e4514_d_n7, assign3020_e4514_d_n16, assign3020_e4514_d_n17,)
            }
        };
        let assign3020_e4517: f64 = (assign3020_e4515 - locals.var_fn25_calc_iq__myarg);
        let assign3020_e4519: f64 = (assign3020_e4517 / locals.var_fn25_calc_iq__alpha_phit);
        (assign3020_e4519, ((assign3020_e4515_d_n2 - locals.var_fn25_calc_iq__myarg_dn2) / locals.var_fn25_calc_iq__alpha_phit), ((-locals.var_fn25_calc_iq__myarg_dn3) / locals.var_fn25_calc_iq__alpha_phit), ((((-locals.var_fn25_calc_iq__myarg_dn4) * locals.var_fn25_calc_iq__alpha_phit) - (assign3020_e4517 * locals.var_fn25_calc_iq__alpha_phit_dn4)) / (locals.var_fn25_calc_iq__alpha_phit * locals.var_fn25_calc_iq__alpha_phit)), ((assign3020_e4515_d_n7 - locals.var_fn25_calc_iq__myarg_dn7) / locals.var_fn25_calc_iq__alpha_phit), ((assign3020_e4515_d_n16 - locals.var_fn25_calc_iq__myarg_dn16) / locals.var_fn25_calc_iq__alpha_phit), ((assign3020_e4515_d_n17 - locals.var_fn25_calc_iq__myarg_dn17) / locals.var_fn25_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign3020_e4521;
        locals.var_fn25_calc_iq__exparg_dn2 = assign3020_e4521_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign3020_e4521_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign3020_e4521_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign3020_e4521_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign3020_e4521_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign3020_e4521_d_n17;

        let assign3030_e4524: f64 = if locals.var_fn25_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard27 = assign3030_e4524;

        let (assign3040_e4530, assign3040_e4530_d_n2, assign3040_e4530_d_n3, assign3040_e4530_d_n4, assign3040_e4530_d_n7, assign3040_e4530_d_n16, assign3040_e4530_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard27 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ff, locals.var_fn25_calc_iq__ff_dn2, locals.var_fn25_calc_iq__ff_dn3, locals.var_fn25_calc_iq__ff_dn4, locals.var_fn25_calc_iq__ff_dn7, locals.var_fn25_calc_iq__ff_dn16, locals.var_fn25_calc_iq__ff_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff = assign3040_e4530;
        locals.var_fn25_calc_iq__ff_dn2 = assign3040_e4530_d_n2;
        locals.var_fn25_calc_iq__ff_dn3 = assign3040_e4530_d_n3;
        locals.var_fn25_calc_iq__ff_dn4 = assign3040_e4530_d_n4;
        locals.var_fn25_calc_iq__ff_dn7 = assign3040_e4530_d_n7;
        locals.var_fn25_calc_iq__ff_dn16 = assign3040_e4530_d_n16;
        locals.var_fn25_calc_iq__ff_dn17 = assign3040_e4530_d_n17;

        let assign3050_e4533: f64 = (-50.0);
        let assign3050_e4534: f64 = if locals.var_fn25_calc_iq__exparg < assign3050_e4533 { 1.0 } else { 0.0 };
        locals.var_guard28 = assign3050_e4534;

        let (assign3060_e4543, assign3060_e4543_d_n2, assign3060_e4543_d_n3, assign3060_e4543_d_n4, assign3060_e4543_d_n7, assign3060_e4543_d_n16, assign3060_e4543_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard27 == 0.0)) && (locals.var_guard28 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ff, locals.var_fn25_calc_iq__ff_dn2, locals.var_fn25_calc_iq__ff_dn3, locals.var_fn25_calc_iq__ff_dn4, locals.var_fn25_calc_iq__ff_dn7, locals.var_fn25_calc_iq__ff_dn16, locals.var_fn25_calc_iq__ff_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff = assign3060_e4543;
        locals.var_fn25_calc_iq__ff_dn2 = assign3060_e4543_d_n2;
        locals.var_fn25_calc_iq__ff_dn3 = assign3060_e4543_d_n3;
        locals.var_fn25_calc_iq__ff_dn4 = assign3060_e4543_d_n4;
        locals.var_fn25_calc_iq__ff_dn7 = assign3060_e4543_d_n7;
        locals.var_fn25_calc_iq__ff_dn16 = assign3060_e4543_d_n16;
        locals.var_fn25_calc_iq__ff_dn17 = assign3060_e4543_d_n17;

        let (assign3070_e4558, assign3070_e4558_d_n2, assign3070_e4558_d_n3, assign3070_e4558_d_n4, assign3070_e4558_d_n7, assign3070_e4558_d_n16, assign3070_e4558_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard27 == 0.0)) && (locals.var_guard28 == 0.0)) {
        let assign3070_e4554: f64 = (locals.var_fn25_calc_iq__exparg).exp();
        let assign3070_e4555: f64 = (1.0 + assign3070_e4554);
        let assign3070_e4556: f64 = (1.0 / assign3070_e4555);
        (assign3070_e4556, (-((assign3070_e4554 * locals.var_fn25_calc_iq__exparg_dn2) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * locals.var_fn25_calc_iq__exparg_dn3) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * locals.var_fn25_calc_iq__exparg_dn4) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * locals.var_fn25_calc_iq__exparg_dn7) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * locals.var_fn25_calc_iq__exparg_dn16) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * locals.var_fn25_calc_iq__exparg_dn17) / (assign3070_e4555 * assign3070_e4555))),)
    } else {
        (locals.var_fn25_calc_iq__ff, locals.var_fn25_calc_iq__ff_dn2, locals.var_fn25_calc_iq__ff_dn3, locals.var_fn25_calc_iq__ff_dn4, locals.var_fn25_calc_iq__ff_dn7, locals.var_fn25_calc_iq__ff_dn16, locals.var_fn25_calc_iq__ff_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff = assign3070_e4558;
        locals.var_fn25_calc_iq__ff_dn2 = assign3070_e4558_d_n2;
        locals.var_fn25_calc_iq__ff_dn3 = assign3070_e4558_d_n3;
        locals.var_fn25_calc_iq__ff_dn4 = assign3070_e4558_d_n4;
        locals.var_fn25_calc_iq__ff_dn7 = assign3070_e4558_d_n7;
        locals.var_fn25_calc_iq__ff_dn16 = assign3070_e4558_d_n16;
        locals.var_fn25_calc_iq__ff_dn17 = assign3070_e4558_d_n17;

        let (assign3080_e4617, assign3080_e4617_d_n2, assign3080_e4617_d_n3, assign3080_e4617_d_n4, assign3080_e4617_d_n7, assign3080_e4617_d_n16, assign3080_e4617_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3080_e4603, assign3080_e4603_d_n2, assign3080_e4603_d_n7, assign3080_e4603_d_n16, assign3080_e4603_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3080_e4567: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                let assign3080_e4570: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3080_e4573: f64 = (0.001 / p.p53);
                let assign3080_e4576: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3080_e4577: f64 = (assign3080_e4573 * assign3080_e4576);
                let assign3080_e4578: f64 = (assign3080_e4577).tanh();
                let assign3080_e4579: f64 = (assign3080_e4570 * assign3080_e4578);
                let assign3080_e4580: f64 = (assign3080_e4567 + assign3080_e4579);
                let assign3080_e4581: f64 = (0.5 * assign3080_e4580);
                (assign3080_e4581, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + (((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3080_e4578) + (assign3080_e4570 * ((assign3080_e4573 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2)) / ((assign3080_e4577).cosh() * (assign3080_e4577).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + (((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3080_e4578) + (assign3080_e4570 * ((assign3080_e4573 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7)) / ((assign3080_e4577).cosh() * (assign3080_e4577).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + (((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3080_e4578) + (assign3080_e4570 * ((assign3080_e4573 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16)) / ((assign3080_e4577).cosh() * (assign3080_e4577).cosh())))))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + (((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3080_e4578) + (assign3080_e4570 * ((assign3080_e4573 * (-locals.var_fn25_calc_iq__vgdin_dn17)) / ((assign3080_e4577).cosh() * (assign3080_e4577).cosh())))))),)
            } else {
                let (assign3080_e4602, assign3080_e4602_d_n2, assign3080_e4602_d_n7, assign3080_e4602_d_n16, assign3080_e4602_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3080_e4588: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                        let assign3080_e4591: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3080_e4594: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3080_e4595: f64 = (assign3080_e4591 * assign3080_e4594);
                        let assign3080_e4597: f64 = (assign3080_e4595 + p.p53);
                        let assign3080_e4598: f64 = (assign3080_e4597).sqrt();
                        let assign3080_e4599: f64 = (assign3080_e4588 + assign3080_e4598);
                        let assign3080_e4600: f64 = (0.5 * assign3080_e4599);
                        (assign3080_e4600, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + ((((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3080_e4594) + (assign3080_e4591 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2))) / (2.0 * assign3080_e4598)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + ((((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3080_e4594) + (assign3080_e4591 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7))) / (2.0 * assign3080_e4598)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + ((((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3080_e4594) + (assign3080_e4591 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16))) / (2.0 * assign3080_e4598)))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + ((((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3080_e4594) + (assign3080_e4591 * (-locals.var_fn25_calc_iq__vgdin_dn17))) / (2.0 * assign3080_e4598)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3080_e4602, assign3080_e4602_d_n2, assign3080_e4602_d_n7, assign3080_e4602_d_n16, assign3080_e4602_d_n17,)
            }
        };
        let assign3080_e4607: f64 = (p.p51 * 0.1);
        let assign3080_e4609: f64 = (assign3080_e4607 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3080_e4611: f64 = (assign3080_e4609 * locals.var_fn25_calc_iq__ff);
        let assign3080_e4612: f64 = (locals.var_fn25_calc_iq__vtdibl - assign3080_e4611);
        let assign3080_e4613: f64 = (assign3080_e4603 - assign3080_e4612);
        let assign3080_e4615: f64 = (assign3080_e4613 / locals.var_fn25_calc_iq__two_n_phit);
        (assign3080_e4615, ((assign3080_e4603_d_n2 - (-(assign3080_e4609 * locals.var_fn25_calc_iq__ff_dn2))) / locals.var_fn25_calc_iq__two_n_phit), ((-(-(assign3080_e4609 * locals.var_fn25_calc_iq__ff_dn3))) / locals.var_fn25_calc_iq__two_n_phit), ((((-(locals.var_fn25_calc_iq__vtdibl_dn4 - (((assign3080_e4607 * locals.var_fn25_calc_iq__alpha_phit_dn4) * locals.var_fn25_calc_iq__ff) + (assign3080_e4609 * locals.var_fn25_calc_iq__ff_dn4)))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3080_e4613 * locals.var_fn25_calc_iq__two_n_phit_dn4)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)), ((assign3080_e4603_d_n7 - (-(assign3080_e4609 * locals.var_fn25_calc_iq__ff_dn7))) / locals.var_fn25_calc_iq__two_n_phit), ((((assign3080_e4603_d_n16 - (locals.var_fn25_calc_iq__vtdibl_dn16 - (assign3080_e4609 * locals.var_fn25_calc_iq__ff_dn16))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3080_e4613 * locals.var_fn25_calc_iq__two_n_phit_dn16)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)), ((((assign3080_e4603_d_n17 - (locals.var_fn25_calc_iq__vtdibl_dn17 - (assign3080_e4609 * locals.var_fn25_calc_iq__ff_dn17))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3080_e4613 * locals.var_fn25_calc_iq__two_n_phit_dn17)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn25_calc_iq__eta, locals.var_fn25_calc_iq__eta_dn2, locals.var_fn25_calc_iq__eta_dn3, locals.var_fn25_calc_iq__eta_dn4, locals.var_fn25_calc_iq__eta_dn7, locals.var_fn25_calc_iq__eta_dn16, locals.var_fn25_calc_iq__eta_dn17,)
    }
};
        locals.var_fn25_calc_iq__eta = assign3080_e4617;
        locals.var_fn25_calc_iq__eta_dn2 = assign3080_e4617_d_n2;
        locals.var_fn25_calc_iq__eta_dn3 = assign3080_e4617_d_n3;
        locals.var_fn25_calc_iq__eta_dn4 = assign3080_e4617_d_n4;
        locals.var_fn25_calc_iq__eta_dn7 = assign3080_e4617_d_n7;
        locals.var_fn25_calc_iq__eta_dn16 = assign3080_e4617_d_n16;
        locals.var_fn25_calc_iq__eta_dn17 = assign3080_e4617_d_n17;

        let assign3090_e4620: f64 = if locals.var_fn25_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign3090_e4620;

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3100_e4628, assign3100_e4628_d_n2, assign3100_e4628_d_n3, assign3100_e4628_d_n4, assign3100_e4628_d_n7, assign3100_e4628_d_n16, assign3100_e4628_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard29 != 0.0)) {
        let assign3100_e4626: f64 = (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__eta);
        (assign3100_e4626, (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__eta_dn2), (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__eta_dn3), ((locals.var_fn25_calc_iq__qref_dn4 * locals.var_fn25_calc_iq__eta) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__eta_dn4)), (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__eta_dn7), ((locals.var_fn25_calc_iq__qref_dn16 * locals.var_fn25_calc_iq__eta) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__eta_dn16)), ((locals.var_fn25_calc_iq__qref_dn17 * locals.var_fn25_calc_iq__eta) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__eta_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qinvv, locals.var_fn25_calc_iq__qinvv_dn2, locals.var_fn25_calc_iq__qinvv_dn3, locals.var_fn25_calc_iq__qinvv_dn4, locals.var_fn25_calc_iq__qinvv_dn7, locals.var_fn25_calc_iq__qinvv_dn16, locals.var_fn25_calc_iq__qinvv_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv = assign3100_e4628;
        locals.var_fn25_calc_iq__qinvv_dn2 = assign3100_e4628_d_n2;
        locals.var_fn25_calc_iq__qinvv_dn3 = assign3100_e4628_d_n3;
        locals.var_fn25_calc_iq__qinvv_dn4 = assign3100_e4628_d_n4;
        locals.var_fn25_calc_iq__qinvv_dn7 = assign3100_e4628_d_n7;
        locals.var_fn25_calc_iq__qinvv_dn16 = assign3100_e4628_d_n16;
        locals.var_fn25_calc_iq__qinvv_dn17 = assign3100_e4628_d_n17;

        let assign3110_e4631: f64 = (-50.0);
        let assign3110_e4632: f64 = if locals.var_fn25_calc_iq__eta < assign3110_e4631 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign3110_e4632;

        let (assign3120_e4644, assign3120_e4644_d_n2, assign3120_e4644_d_n3, assign3120_e4644_d_n4, assign3120_e4644_d_n7, assign3120_e4644_d_n16, assign3120_e4644_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard29 == 0.0)) && (locals.var_guard30 != 0.0)) {
        let assign3120_e4641: f64 = (locals.var_fn25_calc_iq__eta).exp();
        let assign3120_e4642: f64 = (locals.var_fn25_calc_iq__qref * assign3120_e4641);
        (assign3120_e4642, (locals.var_fn25_calc_iq__qref * (assign3120_e4641 * locals.var_fn25_calc_iq__eta_dn2)), (locals.var_fn25_calc_iq__qref * (assign3120_e4641 * locals.var_fn25_calc_iq__eta_dn3)), ((locals.var_fn25_calc_iq__qref_dn4 * assign3120_e4641) + (locals.var_fn25_calc_iq__qref * (assign3120_e4641 * locals.var_fn25_calc_iq__eta_dn4))), (locals.var_fn25_calc_iq__qref * (assign3120_e4641 * locals.var_fn25_calc_iq__eta_dn7)), ((locals.var_fn25_calc_iq__qref_dn16 * assign3120_e4641) + (locals.var_fn25_calc_iq__qref * (assign3120_e4641 * locals.var_fn25_calc_iq__eta_dn16))), ((locals.var_fn25_calc_iq__qref_dn17 * assign3120_e4641) + (locals.var_fn25_calc_iq__qref * (assign3120_e4641 * locals.var_fn25_calc_iq__eta_dn17))),)
    } else {
        (locals.var_fn25_calc_iq__qinvv, locals.var_fn25_calc_iq__qinvv_dn2, locals.var_fn25_calc_iq__qinvv_dn3, locals.var_fn25_calc_iq__qinvv_dn4, locals.var_fn25_calc_iq__qinvv_dn7, locals.var_fn25_calc_iq__qinvv_dn16, locals.var_fn25_calc_iq__qinvv_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv = assign3120_e4644;
        locals.var_fn25_calc_iq__qinvv_dn2 = assign3120_e4644_d_n2;
        locals.var_fn25_calc_iq__qinvv_dn3 = assign3120_e4644_d_n3;
        locals.var_fn25_calc_iq__qinvv_dn4 = assign3120_e4644_d_n4;
        locals.var_fn25_calc_iq__qinvv_dn7 = assign3120_e4644_d_n7;
        locals.var_fn25_calc_iq__qinvv_dn16 = assign3120_e4644_d_n16;
        locals.var_fn25_calc_iq__qinvv_dn17 = assign3120_e4644_d_n17;

        let (assign3130_e4660, assign3130_e4660_d_n2, assign3130_e4660_d_n3, assign3130_e4660_d_n4, assign3130_e4660_d_n7, assign3130_e4660_d_n16, assign3130_e4660_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard29 == 0.0)) && (locals.var_guard30 == 0.0)) {
        let assign3130_e4655: f64 = (locals.var_fn25_calc_iq__eta).exp();
        let assign3130_e4656: f64 = (1.0 + assign3130_e4655);
        let assign3130_e4657: f64 = (assign3130_e4656).ln();
        let assign3130_e4658: f64 = (locals.var_fn25_calc_iq__qref * assign3130_e4657);
        (assign3130_e4658, (locals.var_fn25_calc_iq__qref * ((assign3130_e4655 * locals.var_fn25_calc_iq__eta_dn2) / assign3130_e4656)), (locals.var_fn25_calc_iq__qref * ((assign3130_e4655 * locals.var_fn25_calc_iq__eta_dn3) / assign3130_e4656)), ((locals.var_fn25_calc_iq__qref_dn4 * assign3130_e4657) + (locals.var_fn25_calc_iq__qref * ((assign3130_e4655 * locals.var_fn25_calc_iq__eta_dn4) / assign3130_e4656))), (locals.var_fn25_calc_iq__qref * ((assign3130_e4655 * locals.var_fn25_calc_iq__eta_dn7) / assign3130_e4656)), ((locals.var_fn25_calc_iq__qref_dn16 * assign3130_e4657) + (locals.var_fn25_calc_iq__qref * ((assign3130_e4655 * locals.var_fn25_calc_iq__eta_dn16) / assign3130_e4656))), ((locals.var_fn25_calc_iq__qref_dn17 * assign3130_e4657) + (locals.var_fn25_calc_iq__qref * ((assign3130_e4655 * locals.var_fn25_calc_iq__eta_dn17) / assign3130_e4656))),)
    } else {
        (locals.var_fn25_calc_iq__qinvv, locals.var_fn25_calc_iq__qinvv_dn2, locals.var_fn25_calc_iq__qinvv_dn3, locals.var_fn25_calc_iq__qinvv_dn4, locals.var_fn25_calc_iq__qinvv_dn7, locals.var_fn25_calc_iq__qinvv_dn16, locals.var_fn25_calc_iq__qinvv_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv = assign3130_e4660;
        locals.var_fn25_calc_iq__qinvv_dn2 = assign3130_e4660_d_n2;
        locals.var_fn25_calc_iq__qinvv_dn3 = assign3130_e4660_d_n3;
        locals.var_fn25_calc_iq__qinvv_dn4 = assign3130_e4660_d_n4;
        locals.var_fn25_calc_iq__qinvv_dn7 = assign3130_e4660_d_n7;
        locals.var_fn25_calc_iq__qinvv_dn16 = assign3130_e4660_d_n16;
        locals.var_fn25_calc_iq__qinvv_dn17 = assign3130_e4660_d_n17;

        let (assign3140_e4674, assign3140_e4674_d_n2, assign3140_e4674_d_n3, assign3140_e4674_d_n4, assign3140_e4674_d_n7, assign3140_e4674_d_n16, assign3140_e4674_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3140_e4667: f64 = (locals.var_fn25_calc_iq__mtheta * locals.var_fn25_calc_iq__qinvv);
        let assign3140_e4669: f64 = (assign3140_e4667 / locals.var_fn25_calc_iq__cgin);
        let assign3140_e4670: f64 = (1.0 + assign3140_e4669);
        let assign3140_e4671: f64 = (locals.var_fn25_calc_iq__tfacmobin * assign3140_e4670);
        let assign3140_e4672: f64 = (locals.var_fn25_calc_iq__mu0 / assign3140_e4671);
        (assign3140_e4672, (-((locals.var_fn25_calc_iq__mu0 * (locals.var_fn25_calc_iq__tfacmobin * ((locals.var_fn25_calc_iq__mtheta * locals.var_fn25_calc_iq__qinvv_dn2) / locals.var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))), (-((locals.var_fn25_calc_iq__mu0 * (locals.var_fn25_calc_iq__tfacmobin * ((locals.var_fn25_calc_iq__mtheta * locals.var_fn25_calc_iq__qinvv_dn3) / locals.var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))), (-((locals.var_fn25_calc_iq__mu0 * ((locals.var_fn25_calc_iq__tfacmobin_dn4 * assign3140_e4670) + (locals.var_fn25_calc_iq__tfacmobin * ((((locals.var_fn25_calc_iq__mtheta * locals.var_fn25_calc_iq__qinvv_dn4) * locals.var_fn25_calc_iq__cgin) - (assign3140_e4667 * locals.var_fn25_calc_iq__cgin_dn4)) / (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__cgin))))) / (assign3140_e4671 * assign3140_e4671))), (-((locals.var_fn25_calc_iq__mu0 * (locals.var_fn25_calc_iq__tfacmobin * ((locals.var_fn25_calc_iq__mtheta * locals.var_fn25_calc_iq__qinvv_dn7) / locals.var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))), (-((locals.var_fn25_calc_iq__mu0 * (locals.var_fn25_calc_iq__tfacmobin * ((locals.var_fn25_calc_iq__mtheta * locals.var_fn25_calc_iq__qinvv_dn16) / locals.var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))), (-((locals.var_fn25_calc_iq__mu0 * (locals.var_fn25_calc_iq__tfacmobin * ((locals.var_fn25_calc_iq__mtheta * locals.var_fn25_calc_iq__qinvv_dn17) / locals.var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))),)
    } else {
        (locals.var_fn25_calc_iq__muf, locals.var_fn25_calc_iq__muf_dn2, locals.var_fn25_calc_iq__muf_dn3, locals.var_fn25_calc_iq__muf_dn4, locals.var_fn25_calc_iq__muf_dn7, locals.var_fn25_calc_iq__muf_dn16, locals.var_fn25_calc_iq__muf_dn17,)
    }
};
        locals.var_fn25_calc_iq__muf = assign3140_e4674;
        locals.var_fn25_calc_iq__muf_dn2 = assign3140_e4674_d_n2;
        locals.var_fn25_calc_iq__muf_dn3 = assign3140_e4674_d_n3;
        locals.var_fn25_calc_iq__muf_dn4 = assign3140_e4674_d_n4;
        locals.var_fn25_calc_iq__muf_dn7 = assign3140_e4674_d_n7;
        locals.var_fn25_calc_iq__muf_dn16 = assign3140_e4674_d_n16;
        locals.var_fn25_calc_iq__muf_dn17 = assign3140_e4674_d_n17;

        let (assign3150_e4706, assign3150_e4706_d_n2, assign3150_e4706_d_n3, assign3150_e4706_d_n4, assign3150_e4706_d_n7, assign3150_e4706_d_n16, assign3150_e4706_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3150_e4680: f64 = (locals.var_fn25_calc_iq__vzeta * locals.var_fn25_calc_iq__tnomin);
        let assign3150_e4681: f64 = (1.0 + assign3150_e4680);
        let assign3150_e4685: f64 = (locals.var_fn25_calc_iq__vzeta * locals.var_fn25_calc_iq__tambin);
        let assign3150_e4686: f64 = (1.0 + assign3150_e4685);
        let assign3150_e4687: f64 = (assign3150_e4681 / assign3150_e4686);
        let assign3150_e4688: f64 = (locals.var_fn25_calc_iq__vel0 * assign3150_e4687);
        let assign3150_e4692: f64 = (locals.var_fn25_calc_iq__lambda * locals.var_fn25_calc_iq__absvdsin);
        let assign3150_e4694: f64 = (assign3150_e4692 / locals.var_fn25_calc_iq__lin);
        let assign3150_e4695: f64 = (1.0 + assign3150_e4694);
        let assign3150_e4696: f64 = (assign3150_e4688 * assign3150_e4695);
        let assign3150_e4700: f64 = (locals.var_fn25_calc_iq__vtheta * locals.var_fn25_calc_iq__qinvv);
        let assign3150_e4702: f64 = (assign3150_e4700 / locals.var_fn25_calc_iq__cgin);
        let assign3150_e4703: f64 = (1.0 + assign3150_e4702);
        let assign3150_e4704: f64 = (assign3150_e4696 / assign3150_e4703);
        (assign3150_e4704, (-((assign3150_e4696 * ((locals.var_fn25_calc_iq__vtheta * locals.var_fn25_calc_iq__qinvv_dn2) / locals.var_fn25_calc_iq__cgin)) / (assign3150_e4703 * assign3150_e4703))), (-((assign3150_e4696 * ((locals.var_fn25_calc_iq__vtheta * locals.var_fn25_calc_iq__qinvv_dn3) / locals.var_fn25_calc_iq__cgin)) / (assign3150_e4703 * assign3150_e4703))), (((((locals.var_fn25_calc_iq__vel0 * (-((assign3150_e4681 * (locals.var_fn25_calc_iq__vzeta * locals.var_fn25_calc_iq__tambin_dn4)) / (assign3150_e4686 * assign3150_e4686)))) * assign3150_e4695) * assign3150_e4703) - (assign3150_e4696 * ((((locals.var_fn25_calc_iq__vtheta * locals.var_fn25_calc_iq__qinvv_dn4) * locals.var_fn25_calc_iq__cgin) - (assign3150_e4700 * locals.var_fn25_calc_iq__cgin_dn4)) / (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__cgin)))) / (assign3150_e4703 * assign3150_e4703)), (-((assign3150_e4696 * ((locals.var_fn25_calc_iq__vtheta * locals.var_fn25_calc_iq__qinvv_dn7) / locals.var_fn25_calc_iq__cgin)) / (assign3150_e4703 * assign3150_e4703))), ((((assign3150_e4688 * ((locals.var_fn25_calc_iq__lambda * locals.var_fn25_calc_iq__absvdsin_dn16) / locals.var_fn25_calc_iq__lin)) * assign3150_e4703) - (assign3150_e4696 * ((locals.var_fn25_calc_iq__vtheta * locals.var_fn25_calc_iq__qinvv_dn16) / locals.var_fn25_calc_iq__cgin))) / (assign3150_e4703 * assign3150_e4703)), ((((assign3150_e4688 * ((locals.var_fn25_calc_iq__lambda * locals.var_fn25_calc_iq__absvdsin_dn17) / locals.var_fn25_calc_iq__lin)) * assign3150_e4703) - (assign3150_e4696 * ((locals.var_fn25_calc_iq__vtheta * locals.var_fn25_calc_iq__qinvv_dn17) / locals.var_fn25_calc_iq__cgin))) / (assign3150_e4703 * assign3150_e4703)),)
    } else {
        (locals.var_fn25_calc_iq__vx, locals.var_fn25_calc_iq__vx_dn2, locals.var_fn25_calc_iq__vx_dn3, locals.var_fn25_calc_iq__vx_dn4, locals.var_fn25_calc_iq__vx_dn7, locals.var_fn25_calc_iq__vx_dn16, locals.var_fn25_calc_iq__vx_dn17,)
    }
};
        locals.var_fn25_calc_iq__vx = assign3150_e4706;
        locals.var_fn25_calc_iq__vx_dn2 = assign3150_e4706_d_n2;
        locals.var_fn25_calc_iq__vx_dn3 = assign3150_e4706_d_n3;
        locals.var_fn25_calc_iq__vx_dn4 = assign3150_e4706_d_n4;
        locals.var_fn25_calc_iq__vx_dn7 = assign3150_e4706_d_n7;
        locals.var_fn25_calc_iq__vx_dn16 = assign3150_e4706_d_n16;
        locals.var_fn25_calc_iq__vx_dn17 = assign3150_e4706_d_n17;

        let (assign3160_e4724, assign3160_e4724_d_n2, assign3160_e4724_d_n3, assign3160_e4724_d_n4, assign3160_e4724_d_n7, assign3160_e4724_d_n16, assign3160_e4724_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3160_e4710: f64 = (2.0 * locals.var_fn25_calc_iq__ff);
        let assign3160_e4712: f64 = (assign3160_e4710 * locals.var_fn25_calc_iq__phitin);
        let assign3160_e4714: f64 = (assign3160_e4712 * locals.var_fn25_calc_iq__muf);
        let assign3160_e4716: f64 = (assign3160_e4714 / locals.var_fn25_calc_iq__lin);
        let assign3160_e4719: f64 = (1.0 - locals.var_fn25_calc_iq__ff);
        let assign3160_e4721: f64 = (assign3160_e4719 * locals.var_fn25_calc_iq__vx);
        let assign3160_e4722: f64 = (assign3160_e4716 + assign3160_e4721);
        (assign3160_e4722, ((((((2.0 * locals.var_fn25_calc_iq__ff_dn2) * locals.var_fn25_calc_iq__phitin) * locals.var_fn25_calc_iq__muf) + (assign3160_e4712 * locals.var_fn25_calc_iq__muf_dn2)) / locals.var_fn25_calc_iq__lin) + (((-locals.var_fn25_calc_iq__ff_dn2) * locals.var_fn25_calc_iq__vx) + (assign3160_e4719 * locals.var_fn25_calc_iq__vx_dn2))), ((((((2.0 * locals.var_fn25_calc_iq__ff_dn3) * locals.var_fn25_calc_iq__phitin) * locals.var_fn25_calc_iq__muf) + (assign3160_e4712 * locals.var_fn25_calc_iq__muf_dn3)) / locals.var_fn25_calc_iq__lin) + (((-locals.var_fn25_calc_iq__ff_dn3) * locals.var_fn25_calc_iq__vx) + (assign3160_e4719 * locals.var_fn25_calc_iq__vx_dn3))), (((((((2.0 * locals.var_fn25_calc_iq__ff_dn4) * locals.var_fn25_calc_iq__phitin) + (assign3160_e4710 * locals.var_fn25_calc_iq__phitin_dn4)) * locals.var_fn25_calc_iq__muf) + (assign3160_e4712 * locals.var_fn25_calc_iq__muf_dn4)) / locals.var_fn25_calc_iq__lin) + (((-locals.var_fn25_calc_iq__ff_dn4) * locals.var_fn25_calc_iq__vx) + (assign3160_e4719 * locals.var_fn25_calc_iq__vx_dn4))), ((((((2.0 * locals.var_fn25_calc_iq__ff_dn7) * locals.var_fn25_calc_iq__phitin) * locals.var_fn25_calc_iq__muf) + (assign3160_e4712 * locals.var_fn25_calc_iq__muf_dn7)) / locals.var_fn25_calc_iq__lin) + (((-locals.var_fn25_calc_iq__ff_dn7) * locals.var_fn25_calc_iq__vx) + (assign3160_e4719 * locals.var_fn25_calc_iq__vx_dn7))), ((((((2.0 * locals.var_fn25_calc_iq__ff_dn16) * locals.var_fn25_calc_iq__phitin) * locals.var_fn25_calc_iq__muf) + (assign3160_e4712 * locals.var_fn25_calc_iq__muf_dn16)) / locals.var_fn25_calc_iq__lin) + (((-locals.var_fn25_calc_iq__ff_dn16) * locals.var_fn25_calc_iq__vx) + (assign3160_e4719 * locals.var_fn25_calc_iq__vx_dn16))), ((((((2.0 * locals.var_fn25_calc_iq__ff_dn17) * locals.var_fn25_calc_iq__phitin) * locals.var_fn25_calc_iq__muf) + (assign3160_e4712 * locals.var_fn25_calc_iq__muf_dn17)) / locals.var_fn25_calc_iq__lin) + (((-locals.var_fn25_calc_iq__ff_dn17) * locals.var_fn25_calc_iq__vx) + (assign3160_e4719 * locals.var_fn25_calc_iq__vx_dn17))),)
    } else {
        (locals.var_fn25_calc_iq__vxf, locals.var_fn25_calc_iq__vxf_dn2, locals.var_fn25_calc_iq__vxf_dn3, locals.var_fn25_calc_iq__vxf_dn4, locals.var_fn25_calc_iq__vxf_dn7, locals.var_fn25_calc_iq__vxf_dn16, locals.var_fn25_calc_iq__vxf_dn17,)
    }
};
        locals.var_fn25_calc_iq__vxf = assign3160_e4724;
        locals.var_fn25_calc_iq__vxf_dn2 = assign3160_e4724_d_n2;
        locals.var_fn25_calc_iq__vxf_dn3 = assign3160_e4724_d_n3;
        locals.var_fn25_calc_iq__vxf_dn4 = assign3160_e4724_d_n4;
        locals.var_fn25_calc_iq__vxf_dn7 = assign3160_e4724_d_n7;
        locals.var_fn25_calc_iq__vxf_dn16 = assign3160_e4724_d_n16;
        locals.var_fn25_calc_iq__vxf_dn17 = assign3160_e4724_d_n17;

        let (assign3170_e4732, assign3170_e4732_d_n2, assign3170_e4732_d_n3, assign3170_e4732_d_n4, assign3170_e4732_d_n7, assign3170_e4732_d_n16, assign3170_e4732_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3170_e4728: f64 = (locals.var_fn25_calc_iq__vx * locals.var_fn25_calc_iq__lin);
        let assign3170_e4730: f64 = (assign3170_e4728 / locals.var_fn25_calc_iq__muf);
        (assign3170_e4730, ((((locals.var_fn25_calc_iq__vx_dn2 * locals.var_fn25_calc_iq__lin) * locals.var_fn25_calc_iq__muf) - (assign3170_e4728 * locals.var_fn25_calc_iq__muf_dn2)) / (locals.var_fn25_calc_iq__muf * locals.var_fn25_calc_iq__muf)), ((((locals.var_fn25_calc_iq__vx_dn3 * locals.var_fn25_calc_iq__lin) * locals.var_fn25_calc_iq__muf) - (assign3170_e4728 * locals.var_fn25_calc_iq__muf_dn3)) / (locals.var_fn25_calc_iq__muf * locals.var_fn25_calc_iq__muf)), ((((locals.var_fn25_calc_iq__vx_dn4 * locals.var_fn25_calc_iq__lin) * locals.var_fn25_calc_iq__muf) - (assign3170_e4728 * locals.var_fn25_calc_iq__muf_dn4)) / (locals.var_fn25_calc_iq__muf * locals.var_fn25_calc_iq__muf)), ((((locals.var_fn25_calc_iq__vx_dn7 * locals.var_fn25_calc_iq__lin) * locals.var_fn25_calc_iq__muf) - (assign3170_e4728 * locals.var_fn25_calc_iq__muf_dn7)) / (locals.var_fn25_calc_iq__muf * locals.var_fn25_calc_iq__muf)), ((((locals.var_fn25_calc_iq__vx_dn16 * locals.var_fn25_calc_iq__lin) * locals.var_fn25_calc_iq__muf) - (assign3170_e4728 * locals.var_fn25_calc_iq__muf_dn16)) / (locals.var_fn25_calc_iq__muf * locals.var_fn25_calc_iq__muf)), ((((locals.var_fn25_calc_iq__vx_dn17 * locals.var_fn25_calc_iq__lin) * locals.var_fn25_calc_iq__muf) - (assign3170_e4728 * locals.var_fn25_calc_iq__muf_dn17)) / (locals.var_fn25_calc_iq__muf * locals.var_fn25_calc_iq__muf)),)
    } else {
        (locals.var_fn25_calc_iq__vdsats, locals.var_fn25_calc_iq__vdsats_dn2, locals.var_fn25_calc_iq__vdsats_dn3, locals.var_fn25_calc_iq__vdsats_dn4, locals.var_fn25_calc_iq__vdsats_dn7, locals.var_fn25_calc_iq__vdsats_dn16, locals.var_fn25_calc_iq__vdsats_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsats = assign3170_e4732;
        locals.var_fn25_calc_iq__vdsats_dn2 = assign3170_e4732_d_n2;
        locals.var_fn25_calc_iq__vdsats_dn3 = assign3170_e4732_d_n3;
        locals.var_fn25_calc_iq__vdsats_dn4 = assign3170_e4732_d_n4;
        locals.var_fn25_calc_iq__vdsats_dn7 = assign3170_e4732_d_n7;
        locals.var_fn25_calc_iq__vdsats_dn16 = assign3170_e4732_d_n16;
        locals.var_fn25_calc_iq__vdsats_dn17 = assign3170_e4732_d_n17;

        let (assign3180_e4749, assign3180_e4749_d_n2, assign3180_e4749_d_n3, assign3180_e4749_d_n4, assign3180_e4749_d_n7, assign3180_e4749_d_n16, assign3180_e4749_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3180_e4738: f64 = (2.0 * locals.var_fn25_calc_iq__qinvv);
        let assign3180_e4740: f64 = (assign3180_e4738 / locals.var_fn25_calc_iq__cgin);
        let assign3180_e4742: f64 = (assign3180_e4740 / locals.var_fn25_calc_iq__vdsats);
        let assign3180_e4743: f64 = (1.0 + assign3180_e4742);
        let assign3180_e4744: f64 = (assign3180_e4743).sqrt();
        let assign3180_e4745: f64 = (locals.var_fn25_calc_iq__vdsats * assign3180_e4744);
        let assign3180_e4747: f64 = (assign3180_e4745 - locals.var_fn25_calc_iq__vdsats);
        (assign3180_e4747, (((locals.var_fn25_calc_iq__vdsats_dn2 * assign3180_e4744) + (locals.var_fn25_calc_iq__vdsats * ((((((2.0 * locals.var_fn25_calc_iq__qinvv_dn2) / locals.var_fn25_calc_iq__cgin) * locals.var_fn25_calc_iq__vdsats) - (assign3180_e4740 * locals.var_fn25_calc_iq__vdsats_dn2)) / (locals.var_fn25_calc_iq__vdsats * locals.var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - locals.var_fn25_calc_iq__vdsats_dn2), (((locals.var_fn25_calc_iq__vdsats_dn3 * assign3180_e4744) + (locals.var_fn25_calc_iq__vdsats * ((((((2.0 * locals.var_fn25_calc_iq__qinvv_dn3) / locals.var_fn25_calc_iq__cgin) * locals.var_fn25_calc_iq__vdsats) - (assign3180_e4740 * locals.var_fn25_calc_iq__vdsats_dn3)) / (locals.var_fn25_calc_iq__vdsats * locals.var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - locals.var_fn25_calc_iq__vdsats_dn3), (((locals.var_fn25_calc_iq__vdsats_dn4 * assign3180_e4744) + (locals.var_fn25_calc_iq__vdsats * ((((((((2.0 * locals.var_fn25_calc_iq__qinvv_dn4) * locals.var_fn25_calc_iq__cgin) - (assign3180_e4738 * locals.var_fn25_calc_iq__cgin_dn4)) / (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__cgin)) * locals.var_fn25_calc_iq__vdsats) - (assign3180_e4740 * locals.var_fn25_calc_iq__vdsats_dn4)) / (locals.var_fn25_calc_iq__vdsats * locals.var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - locals.var_fn25_calc_iq__vdsats_dn4), (((locals.var_fn25_calc_iq__vdsats_dn7 * assign3180_e4744) + (locals.var_fn25_calc_iq__vdsats * ((((((2.0 * locals.var_fn25_calc_iq__qinvv_dn7) / locals.var_fn25_calc_iq__cgin) * locals.var_fn25_calc_iq__vdsats) - (assign3180_e4740 * locals.var_fn25_calc_iq__vdsats_dn7)) / (locals.var_fn25_calc_iq__vdsats * locals.var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - locals.var_fn25_calc_iq__vdsats_dn7), (((locals.var_fn25_calc_iq__vdsats_dn16 * assign3180_e4744) + (locals.var_fn25_calc_iq__vdsats * ((((((2.0 * locals.var_fn25_calc_iq__qinvv_dn16) / locals.var_fn25_calc_iq__cgin) * locals.var_fn25_calc_iq__vdsats) - (assign3180_e4740 * locals.var_fn25_calc_iq__vdsats_dn16)) / (locals.var_fn25_calc_iq__vdsats * locals.var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - locals.var_fn25_calc_iq__vdsats_dn16), (((locals.var_fn25_calc_iq__vdsats_dn17 * assign3180_e4744) + (locals.var_fn25_calc_iq__vdsats * ((((((2.0 * locals.var_fn25_calc_iq__qinvv_dn17) / locals.var_fn25_calc_iq__cgin) * locals.var_fn25_calc_iq__vdsats) - (assign3180_e4740 * locals.var_fn25_calc_iq__vdsats_dn17)) / (locals.var_fn25_calc_iq__vdsats * locals.var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - locals.var_fn25_calc_iq__vdsats_dn17),)
    } else {
        (locals.var_fn25_calc_iq__vdsats1, locals.var_fn25_calc_iq__vdsats1_dn2, locals.var_fn25_calc_iq__vdsats1_dn3, locals.var_fn25_calc_iq__vdsats1_dn4, locals.var_fn25_calc_iq__vdsats1_dn7, locals.var_fn25_calc_iq__vdsats1_dn16, locals.var_fn25_calc_iq__vdsats1_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsats1 = assign3180_e4749;
        locals.var_fn25_calc_iq__vdsats1_dn2 = assign3180_e4749_d_n2;
        locals.var_fn25_calc_iq__vdsats1_dn3 = assign3180_e4749_d_n3;
        locals.var_fn25_calc_iq__vdsats1_dn4 = assign3180_e4749_d_n4;
        locals.var_fn25_calc_iq__vdsats1_dn7 = assign3180_e4749_d_n7;
        locals.var_fn25_calc_iq__vdsats1_dn16 = assign3180_e4749_d_n16;
        locals.var_fn25_calc_iq__vdsats1_dn17 = assign3180_e4749_d_n17;

        let (assign3190_e4761, assign3190_e4761_d_n2, assign3190_e4761_d_n3, assign3190_e4761_d_n4, assign3190_e4761_d_n7, assign3190_e4761_d_n16, assign3190_e4761_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3190_e4754: f64 = (1.0 - locals.var_fn25_calc_iq__ff);
        let assign3190_e4755: f64 = (locals.var_fn25_calc_iq__vdsats * assign3190_e4754);
        let assign3190_e4758: f64 = (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff);
        let assign3190_e4759: f64 = (assign3190_e4755 + assign3190_e4758);
        (assign3190_e4759, (((locals.var_fn25_calc_iq__vdsats_dn2 * assign3190_e4754) + (locals.var_fn25_calc_iq__vdsats * (-locals.var_fn25_calc_iq__ff_dn2))) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn2)), (((locals.var_fn25_calc_iq__vdsats_dn3 * assign3190_e4754) + (locals.var_fn25_calc_iq__vdsats * (-locals.var_fn25_calc_iq__ff_dn3))) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn3)), (((locals.var_fn25_calc_iq__vdsats_dn4 * assign3190_e4754) + (locals.var_fn25_calc_iq__vdsats * (-locals.var_fn25_calc_iq__ff_dn4))) + ((locals.var_fn25_calc_iq__two_n_phit_dn4 * locals.var_fn25_calc_iq__ff) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn4))), (((locals.var_fn25_calc_iq__vdsats_dn7 * assign3190_e4754) + (locals.var_fn25_calc_iq__vdsats * (-locals.var_fn25_calc_iq__ff_dn7))) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn7)), (((locals.var_fn25_calc_iq__vdsats_dn16 * assign3190_e4754) + (locals.var_fn25_calc_iq__vdsats * (-locals.var_fn25_calc_iq__ff_dn16))) + ((locals.var_fn25_calc_iq__two_n_phit_dn16 * locals.var_fn25_calc_iq__ff) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn16))), (((locals.var_fn25_calc_iq__vdsats_dn17 * assign3190_e4754) + (locals.var_fn25_calc_iq__vdsats * (-locals.var_fn25_calc_iq__ff_dn17))) + ((locals.var_fn25_calc_iq__two_n_phit_dn17 * locals.var_fn25_calc_iq__ff) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn17))),)
    } else {
        (locals.var_fn25_calc_iq__vdsat, locals.var_fn25_calc_iq__vdsat_dn2, locals.var_fn25_calc_iq__vdsat_dn3, locals.var_fn25_calc_iq__vdsat_dn4, locals.var_fn25_calc_iq__vdsat_dn7, locals.var_fn25_calc_iq__vdsat_dn16, locals.var_fn25_calc_iq__vdsat_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsat = assign3190_e4761;
        locals.var_fn25_calc_iq__vdsat_dn2 = assign3190_e4761_d_n2;
        locals.var_fn25_calc_iq__vdsat_dn3 = assign3190_e4761_d_n3;
        locals.var_fn25_calc_iq__vdsat_dn4 = assign3190_e4761_d_n4;
        locals.var_fn25_calc_iq__vdsat_dn7 = assign3190_e4761_d_n7;
        locals.var_fn25_calc_iq__vdsat_dn16 = assign3190_e4761_d_n16;
        locals.var_fn25_calc_iq__vdsat_dn17 = assign3190_e4761_d_n17;

        let (assign3200_e4773, assign3200_e4773_d_n2, assign3200_e4773_d_n3, assign3200_e4773_d_n4, assign3200_e4773_d_n7, assign3200_e4773_d_n16, assign3200_e4773_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3200_e4766: f64 = (1.0 - locals.var_fn25_calc_iq__ff);
        let assign3200_e4767: f64 = (locals.var_fn25_calc_iq__vdsats1 * assign3200_e4766);
        let assign3200_e4770: f64 = (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff);
        let assign3200_e4771: f64 = (assign3200_e4767 + assign3200_e4770);
        (assign3200_e4771, (((locals.var_fn25_calc_iq__vdsats1_dn2 * assign3200_e4766) + (locals.var_fn25_calc_iq__vdsats1 * (-locals.var_fn25_calc_iq__ff_dn2))) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn2)), (((locals.var_fn25_calc_iq__vdsats1_dn3 * assign3200_e4766) + (locals.var_fn25_calc_iq__vdsats1 * (-locals.var_fn25_calc_iq__ff_dn3))) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn3)), (((locals.var_fn25_calc_iq__vdsats1_dn4 * assign3200_e4766) + (locals.var_fn25_calc_iq__vdsats1 * (-locals.var_fn25_calc_iq__ff_dn4))) + ((locals.var_fn25_calc_iq__two_n_phit_dn4 * locals.var_fn25_calc_iq__ff) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn4))), (((locals.var_fn25_calc_iq__vdsats1_dn7 * assign3200_e4766) + (locals.var_fn25_calc_iq__vdsats1 * (-locals.var_fn25_calc_iq__ff_dn7))) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn7)), (((locals.var_fn25_calc_iq__vdsats1_dn16 * assign3200_e4766) + (locals.var_fn25_calc_iq__vdsats1 * (-locals.var_fn25_calc_iq__ff_dn16))) + ((locals.var_fn25_calc_iq__two_n_phit_dn16 * locals.var_fn25_calc_iq__ff) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn16))), (((locals.var_fn25_calc_iq__vdsats1_dn17 * assign3200_e4766) + (locals.var_fn25_calc_iq__vdsats1 * (-locals.var_fn25_calc_iq__ff_dn17))) + ((locals.var_fn25_calc_iq__two_n_phit_dn17 * locals.var_fn25_calc_iq__ff) + (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__ff_dn17))),)
    } else {
        (locals.var_fn25_calc_iq__vdsat1, locals.var_fn25_calc_iq__vdsat1_dn2, locals.var_fn25_calc_iq__vdsat1_dn3, locals.var_fn25_calc_iq__vdsat1_dn4, locals.var_fn25_calc_iq__vdsat1_dn7, locals.var_fn25_calc_iq__vdsat1_dn16, locals.var_fn25_calc_iq__vdsat1_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsat1 = assign3200_e4773;
        locals.var_fn25_calc_iq__vdsat1_dn2 = assign3200_e4773_d_n2;
        locals.var_fn25_calc_iq__vdsat1_dn3 = assign3200_e4773_d_n3;
        locals.var_fn25_calc_iq__vdsat1_dn4 = assign3200_e4773_d_n4;
        locals.var_fn25_calc_iq__vdsat1_dn7 = assign3200_e4773_d_n7;
        locals.var_fn25_calc_iq__vdsat1_dn16 = assign3200_e4773_d_n16;
        locals.var_fn25_calc_iq__vdsat1_dn17 = assign3200_e4773_d_n17;

        let (assign3210_e4842, assign3210_e4842_d_n2, assign3210_e4842_d_n3, assign3210_e4842_d_n4, assign3210_e4842_d_n7, assign3210_e4842_d_n16, assign3210_e4842_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3210_e4832, assign3210_e4832_d_n2, assign3210_e4832_d_n3, assign3210_e4832_d_n4, assign3210_e4832_d_n7, assign3210_e4832_d_n16, assign3210_e4832_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3210_e4785: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat1);
                let assign3210_e4786: f64 = assign3210_e4785;
                let assign3210_e4790: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat1);
                let assign3210_e4791: f64 = (-assign3210_e4790);
                let assign3210_e4794: f64 = (0.001 / p.p53);
                let assign3210_e4798: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat1);
                let assign3210_e4799: f64 = (-assign3210_e4798);
                let assign3210_e4800: f64 = (assign3210_e4794 * assign3210_e4799);
                let assign3210_e4801: f64 = (assign3210_e4800).tanh();
                let assign3210_e4802: f64 = (assign3210_e4791 * assign3210_e4801);
                let assign3210_e4803: f64 = (assign3210_e4786 + assign3210_e4802);
                let assign3210_e4804: f64 = (0.5 * assign3210_e4803);
                (assign3210_e4804, (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + (((-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + (((-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))),)
            } else {
                let (assign3210_e4831, assign3210_e4831_d_n2, assign3210_e4831_d_n3, assign3210_e4831_d_n4, assign3210_e4831_d_n7, assign3210_e4831_d_n16, assign3210_e4831_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3210_e4812: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat1);
                        let assign3210_e4813: f64 = assign3210_e4812;
                        let assign3210_e4817: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat1);
                        let assign3210_e4818: f64 = (-assign3210_e4817);
                        let assign3210_e4822: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat1);
                        let assign3210_e4823: f64 = (-assign3210_e4822);
                        let assign3210_e4824: f64 = (assign3210_e4818 * assign3210_e4823);
                        let assign3210_e4826: f64 = (assign3210_e4824 + p.p53);
                        let assign3210_e4827: f64 = (assign3210_e4826).sqrt();
                        let assign3210_e4828: f64 = (assign3210_e4813 + assign3210_e4827);
                        let assign3210_e4829: f64 = (0.5 * assign3210_e4828);
                        (assign3210_e4829, (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4823) + (assign3210_e4818 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3210_e4827)))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4823) + (assign3210_e4818 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3210_e4827)))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4823) + (assign3210_e4818 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3210_e4827)))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3210_e4823) + (assign3210_e4818 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3210_e4827)))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + ((((-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3210_e4823) + (assign3210_e4818 * (-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / (2.0 * assign3210_e4827)))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + ((((-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3210_e4823) + (assign3210_e4818 * (-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat1) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / (2.0 * assign3210_e4827)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3210_e4831, assign3210_e4831_d_n2, assign3210_e4831_d_n3, assign3210_e4831_d_n4, assign3210_e4831_d_n7, assign3210_e4831_d_n16, assign3210_e4831_d_n17,)
            }
        };
        let assign3210_e4834: f64 = (assign3210_e4832).powf(locals.var_fn25_calc_iq__beta);
        let assign3210_e4835: f64 = (1.0 + assign3210_e4834);
        let assign3210_e4838: f64 = (1.0 / locals.var_fn25_calc_iq__beta);
        let assign3210_e4839: f64 = (assign3210_e4835).powf(assign3210_e4838);
        let assign3210_e4840: f64 = (1.0 / assign3210_e4839);
        (assign3210_e4840, (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n2)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n2 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n2)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n2 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n3)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n3 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n3)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n3 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n4)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n4 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n4)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n4 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n7)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n7 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n7)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n7 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n16)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n16 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n16)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n16 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n17)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n17 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3210_e4832).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n17)) } } else { (assign3210_e4834 * (locals.var_fn25_calc_iq__beta * (assign3210_e4832_d_n17 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))),)
    } else {
        (locals.var_fn25_calc_iq__fsd, locals.var_fn25_calc_iq__fsd_dn2, locals.var_fn25_calc_iq__fsd_dn3, locals.var_fn25_calc_iq__fsd_dn4, locals.var_fn25_calc_iq__fsd_dn7, locals.var_fn25_calc_iq__fsd_dn16, locals.var_fn25_calc_iq__fsd_dn17,)
    }
};
        locals.var_fn25_calc_iq__fsd = assign3210_e4842;
        locals.var_fn25_calc_iq__fsd_dn2 = assign3210_e4842_d_n2;
        locals.var_fn25_calc_iq__fsd_dn3 = assign3210_e4842_d_n3;
        locals.var_fn25_calc_iq__fsd_dn4 = assign3210_e4842_d_n4;
        locals.var_fn25_calc_iq__fsd_dn7 = assign3210_e4842_d_n7;
        locals.var_fn25_calc_iq__fsd_dn16 = assign3210_e4842_d_n16;
        locals.var_fn25_calc_iq__fsd_dn17 = assign3210_e4842_d_n17;

        let (assign3220_e4848, assign3220_e4848_d_n2, assign3220_e4848_d_n3, assign3220_e4848_d_n4, assign3220_e4848_d_n7, assign3220_e4848_d_n16, assign3220_e4848_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3220_e4846: f64 = (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd);
        (assign3220_e4846, (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd_dn2), (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd_dn3), (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd_dn4), (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd_dn7), ((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__fsd) + (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd_dn16)), ((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__fsd) + (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__vdx, locals.var_fn25_calc_iq__vdx_dn2, locals.var_fn25_calc_iq__vdx_dn3, locals.var_fn25_calc_iq__vdx_dn4, locals.var_fn25_calc_iq__vdx_dn7, locals.var_fn25_calc_iq__vdx_dn16, locals.var_fn25_calc_iq__vdx_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdx = assign3220_e4848;
        locals.var_fn25_calc_iq__vdx_dn2 = assign3220_e4848_d_n2;
        locals.var_fn25_calc_iq__vdx_dn3 = assign3220_e4848_d_n3;
        locals.var_fn25_calc_iq__vdx_dn4 = assign3220_e4848_d_n4;
        locals.var_fn25_calc_iq__vdx_dn7 = assign3220_e4848_d_n7;
        locals.var_fn25_calc_iq__vdx_dn16 = assign3220_e4848_d_n16;
        locals.var_fn25_calc_iq__vdx_dn17 = assign3220_e4848_d_n17;

        let (assign3230_e4923, assign3230_e4923_d_n2, assign3230_e4923_d_n3, assign3230_e4923_d_n4, assign3230_e4923_d_n7, assign3230_e4923_d_n16, assign3230_e4923_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3230_e4913, assign3230_e4913_d_n2, assign3230_e4913_d_n3, assign3230_e4913_d_n4, assign3230_e4913_d_n7, assign3230_e4913_d_n16, assign3230_e4913_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3230_e4859: f64 = (-locals.var_fn25_calc_iq__vdsin);
                let assign3230_e4861: f64 = (assign3230_e4859 / locals.var_fn25_calc_iq__vdsat1);
                let assign3230_e4862: f64 = assign3230_e4861;
                let assign3230_e4865: f64 = (-locals.var_fn25_calc_iq__vdsin);
                let assign3230_e4867: f64 = (assign3230_e4865 / locals.var_fn25_calc_iq__vdsat1);
                let assign3230_e4868: f64 = (-assign3230_e4867);
                let assign3230_e4871: f64 = (0.001 / p.p53);
                let assign3230_e4874: f64 = (-locals.var_fn25_calc_iq__vdsin);
                let assign3230_e4876: f64 = (assign3230_e4874 / locals.var_fn25_calc_iq__vdsat1);
                let assign3230_e4877: f64 = (-assign3230_e4876);
                let assign3230_e4878: f64 = (assign3230_e4871 * assign3230_e4877);
                let assign3230_e4879: f64 = (assign3230_e4878).tanh();
                let assign3230_e4880: f64 = (assign3230_e4868 * assign3230_e4879);
                let assign3230_e4881: f64 = (assign3230_e4862 + assign3230_e4880);
                let assign3230_e4882: f64 = (0.5 * assign3230_e4881);
                (assign3230_e4882, (0.5 * ((-((assign3230_e4859 * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((assign3230_e4865 * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-(-((assign3230_e4874 * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * ((-((assign3230_e4859 * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((assign3230_e4865 * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-(-((assign3230_e4874 * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * ((-((assign3230_e4859 * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((assign3230_e4865 * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-(-((assign3230_e4874 * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * ((-((assign3230_e4859 * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + (((-(-((assign3230_e4865 * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-(-((assign3230_e4874 * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4859 * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + (((-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4865 * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4874 * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4859 * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + (((-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4865 * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4874 * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))),)
            } else {
                let (assign3230_e4912, assign3230_e4912_d_n2, assign3230_e4912_d_n3, assign3230_e4912_d_n4, assign3230_e4912_d_n7, assign3230_e4912_d_n16, assign3230_e4912_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3230_e4889: f64 = (-locals.var_fn25_calc_iq__vdsin);
                        let assign3230_e4891: f64 = (assign3230_e4889 / locals.var_fn25_calc_iq__vdsat1);
                        let assign3230_e4892: f64 = assign3230_e4891;
                        let assign3230_e4895: f64 = (-locals.var_fn25_calc_iq__vdsin);
                        let assign3230_e4897: f64 = (assign3230_e4895 / locals.var_fn25_calc_iq__vdsat1);
                        let assign3230_e4898: f64 = (-assign3230_e4897);
                        let assign3230_e4901: f64 = (-locals.var_fn25_calc_iq__vdsin);
                        let assign3230_e4903: f64 = (assign3230_e4901 / locals.var_fn25_calc_iq__vdsat1);
                        let assign3230_e4904: f64 = (-assign3230_e4903);
                        let assign3230_e4905: f64 = (assign3230_e4898 * assign3230_e4904);
                        let assign3230_e4907: f64 = (assign3230_e4905 + p.p53);
                        let assign3230_e4908: f64 = (assign3230_e4907).sqrt();
                        let assign3230_e4909: f64 = (assign3230_e4892 + assign3230_e4908);
                        let assign3230_e4910: f64 = (0.5 * assign3230_e4909);
                        (assign3230_e4910, (0.5 * ((-((assign3230_e4889 * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((assign3230_e4895 * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4904) + (assign3230_e4898 * (-(-((assign3230_e4901 * locals.var_fn25_calc_iq__vdsat1_dn2) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3230_e4908)))), (0.5 * ((-((assign3230_e4889 * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((assign3230_e4895 * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4904) + (assign3230_e4898 * (-(-((assign3230_e4901 * locals.var_fn25_calc_iq__vdsat1_dn3) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3230_e4908)))), (0.5 * ((-((assign3230_e4889 * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((assign3230_e4895 * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4904) + (assign3230_e4898 * (-(-((assign3230_e4901 * locals.var_fn25_calc_iq__vdsat1_dn4) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3230_e4908)))), (0.5 * ((-((assign3230_e4889 * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) + ((((-(-((assign3230_e4895 * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))) * assign3230_e4904) + (assign3230_e4898 * (-(-((assign3230_e4901 * locals.var_fn25_calc_iq__vdsat1_dn7) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3230_e4908)))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4889 * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + ((((-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4895 * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3230_e4904) + (assign3230_e4898 * (-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4901 * locals.var_fn25_calc_iq__vdsat1_dn16)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / (2.0 * assign3230_e4908)))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4889 * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1)) + ((((-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4895 * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))) * assign3230_e4904) + (assign3230_e4898 * (-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat1) - (assign3230_e4901 * locals.var_fn25_calc_iq__vdsat1_dn17)) / (locals.var_fn25_calc_iq__vdsat1 * locals.var_fn25_calc_iq__vdsat1))))) / (2.0 * assign3230_e4908)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3230_e4912, assign3230_e4912_d_n2, assign3230_e4912_d_n3, assign3230_e4912_d_n4, assign3230_e4912_d_n7, assign3230_e4912_d_n16, assign3230_e4912_d_n17,)
            }
        };
        let assign3230_e4915: f64 = (assign3230_e4913).powf(locals.var_fn25_calc_iq__beta);
        let assign3230_e4916: f64 = (1.0 + assign3230_e4915);
        let assign3230_e4919: f64 = (1.0 / locals.var_fn25_calc_iq__beta);
        let assign3230_e4920: f64 = (assign3230_e4916).powf(assign3230_e4919);
        let assign3230_e4921: f64 = (1.0 / assign3230_e4920);
        (assign3230_e4921, (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n2)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n2 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n2)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n2 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n3)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n3 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n3)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n3 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n4)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n4 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n4)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n4 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n7)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n7 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n7)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n7 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n16)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n16 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n16)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n16 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n17)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n17 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3230_e4913).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n17)) } } else { (assign3230_e4915 * (locals.var_fn25_calc_iq__beta * (assign3230_e4913_d_n17 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))),)
    } else {
        (locals.var_fn25_calc_iq__fds, locals.var_fn25_calc_iq__fds_dn2, locals.var_fn25_calc_iq__fds_dn3, locals.var_fn25_calc_iq__fds_dn4, locals.var_fn25_calc_iq__fds_dn7, locals.var_fn25_calc_iq__fds_dn16, locals.var_fn25_calc_iq__fds_dn17,)
    }
};
        locals.var_fn25_calc_iq__fds = assign3230_e4923;
        locals.var_fn25_calc_iq__fds_dn2 = assign3230_e4923_d_n2;
        locals.var_fn25_calc_iq__fds_dn3 = assign3230_e4923_d_n3;
        locals.var_fn25_calc_iq__fds_dn4 = assign3230_e4923_d_n4;
        locals.var_fn25_calc_iq__fds_dn7 = assign3230_e4923_d_n7;
        locals.var_fn25_calc_iq__fds_dn16 = assign3230_e4923_d_n16;
        locals.var_fn25_calc_iq__fds_dn17 = assign3230_e4923_d_n17;

        let (assign3240_e4930, assign3240_e4930_d_n2, assign3240_e4930_d_n3, assign3240_e4930_d_n4, assign3240_e4930_d_n7, assign3240_e4930_d_n16, assign3240_e4930_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3240_e4926: f64 = (-locals.var_fn25_calc_iq__vdsin);
        let assign3240_e4928: f64 = (assign3240_e4926 * locals.var_fn25_calc_iq__fds);
        (assign3240_e4928, (assign3240_e4926 * locals.var_fn25_calc_iq__fds_dn2), (assign3240_e4926 * locals.var_fn25_calc_iq__fds_dn3), (assign3240_e4926 * locals.var_fn25_calc_iq__fds_dn4), (assign3240_e4926 * locals.var_fn25_calc_iq__fds_dn7), (((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__fds) + (assign3240_e4926 * locals.var_fn25_calc_iq__fds_dn16)), (((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__fds) + (assign3240_e4926 * locals.var_fn25_calc_iq__fds_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__vsx, locals.var_fn25_calc_iq__vsx_dn2, locals.var_fn25_calc_iq__vsx_dn3, locals.var_fn25_calc_iq__vsx_dn4, locals.var_fn25_calc_iq__vsx_dn7, locals.var_fn25_calc_iq__vsx_dn16, locals.var_fn25_calc_iq__vsx_dn17,)
    }
};
        locals.var_fn25_calc_iq__vsx = assign3240_e4930;
        locals.var_fn25_calc_iq__vsx_dn2 = assign3240_e4930_d_n2;
        locals.var_fn25_calc_iq__vsx_dn3 = assign3240_e4930_d_n3;
        locals.var_fn25_calc_iq__vsx_dn4 = assign3240_e4930_d_n4;
        locals.var_fn25_calc_iq__vsx_dn7 = assign3240_e4930_d_n7;
        locals.var_fn25_calc_iq__vsx_dn16 = assign3240_e4930_d_n16;
        locals.var_fn25_calc_iq__vsx_dn17 = assign3240_e4930_d_n17;

        let (assign3250_e4938, assign3250_e4938_d_n2, assign3250_e4938_d_n3, assign3250_e4938_d_n4, assign3250_e4938_d_n7, assign3250_e4938_d_n16, assign3250_e4938_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3250_e4934: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__myarg);
        let assign3250_e4936: f64 = (assign3250_e4934 / locals.var_fn25_calc_iq__alpha_phit);
        (assign3250_e4936, ((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__myarg_dn2) / locals.var_fn25_calc_iq__alpha_phit), ((-locals.var_fn25_calc_iq__myarg_dn3) / locals.var_fn25_calc_iq__alpha_phit), ((((-locals.var_fn25_calc_iq__myarg_dn4) * locals.var_fn25_calc_iq__alpha_phit) - (assign3250_e4934 * locals.var_fn25_calc_iq__alpha_phit_dn4)) / (locals.var_fn25_calc_iq__alpha_phit * locals.var_fn25_calc_iq__alpha_phit)), ((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__myarg_dn7) / locals.var_fn25_calc_iq__alpha_phit), ((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__myarg_dn16) / locals.var_fn25_calc_iq__alpha_phit), ((-locals.var_fn25_calc_iq__myarg_dn17) / locals.var_fn25_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign3250_e4938;
        locals.var_fn25_calc_iq__exparg_dn2 = assign3250_e4938_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign3250_e4938_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign3250_e4938_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign3250_e4938_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign3250_e4938_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign3250_e4938_d_n17;

        let assign3260_e4941: f64 = if locals.var_fn25_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign3260_e4941;

        let (assign3270_e4947, assign3270_e4947_d_n2, assign3270_e4947_d_n3, assign3270_e4947_d_n4, assign3270_e4947_d_n7, assign3270_e4947_d_n16, assign3270_e4947_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard31 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffs, locals.var_fn25_calc_iq__ffs_dn2, locals.var_fn25_calc_iq__ffs_dn3, locals.var_fn25_calc_iq__ffs_dn4, locals.var_fn25_calc_iq__ffs_dn7, locals.var_fn25_calc_iq__ffs_dn16, locals.var_fn25_calc_iq__ffs_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs = assign3270_e4947;
        locals.var_fn25_calc_iq__ffs_dn2 = assign3270_e4947_d_n2;
        locals.var_fn25_calc_iq__ffs_dn3 = assign3270_e4947_d_n3;
        locals.var_fn25_calc_iq__ffs_dn4 = assign3270_e4947_d_n4;
        locals.var_fn25_calc_iq__ffs_dn7 = assign3270_e4947_d_n7;
        locals.var_fn25_calc_iq__ffs_dn16 = assign3270_e4947_d_n16;
        locals.var_fn25_calc_iq__ffs_dn17 = assign3270_e4947_d_n17;

        let assign3280_e4950: f64 = (-50.0);
        let assign3280_e4951: f64 = if locals.var_fn25_calc_iq__exparg < assign3280_e4950 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign3280_e4951;

        let (assign3290_e4960, assign3290_e4960_d_n2, assign3290_e4960_d_n3, assign3290_e4960_d_n4, assign3290_e4960_d_n7, assign3290_e4960_d_n16, assign3290_e4960_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard31 == 0.0)) && (locals.var_guard32 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffs, locals.var_fn25_calc_iq__ffs_dn2, locals.var_fn25_calc_iq__ffs_dn3, locals.var_fn25_calc_iq__ffs_dn4, locals.var_fn25_calc_iq__ffs_dn7, locals.var_fn25_calc_iq__ffs_dn16, locals.var_fn25_calc_iq__ffs_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs = assign3290_e4960;
        locals.var_fn25_calc_iq__ffs_dn2 = assign3290_e4960_d_n2;
        locals.var_fn25_calc_iq__ffs_dn3 = assign3290_e4960_d_n3;
        locals.var_fn25_calc_iq__ffs_dn4 = assign3290_e4960_d_n4;
        locals.var_fn25_calc_iq__ffs_dn7 = assign3290_e4960_d_n7;
        locals.var_fn25_calc_iq__ffs_dn16 = assign3290_e4960_d_n16;
        locals.var_fn25_calc_iq__ffs_dn17 = assign3290_e4960_d_n17;

        let (assign3300_e4975, assign3300_e4975_d_n2, assign3300_e4975_d_n3, assign3300_e4975_d_n4, assign3300_e4975_d_n7, assign3300_e4975_d_n16, assign3300_e4975_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard31 == 0.0)) && (locals.var_guard32 == 0.0)) {
        let assign3300_e4971: f64 = (locals.var_fn25_calc_iq__exparg).exp();
        let assign3300_e4972: f64 = (1.0 + assign3300_e4971);
        let assign3300_e4973: f64 = (1.0 / assign3300_e4972);
        (assign3300_e4973, (-((assign3300_e4971 * locals.var_fn25_calc_iq__exparg_dn2) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * locals.var_fn25_calc_iq__exparg_dn3) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * locals.var_fn25_calc_iq__exparg_dn4) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * locals.var_fn25_calc_iq__exparg_dn7) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * locals.var_fn25_calc_iq__exparg_dn16) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * locals.var_fn25_calc_iq__exparg_dn17) / (assign3300_e4972 * assign3300_e4972))),)
    } else {
        (locals.var_fn25_calc_iq__ffs, locals.var_fn25_calc_iq__ffs_dn2, locals.var_fn25_calc_iq__ffs_dn3, locals.var_fn25_calc_iq__ffs_dn4, locals.var_fn25_calc_iq__ffs_dn7, locals.var_fn25_calc_iq__ffs_dn16, locals.var_fn25_calc_iq__ffs_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs = assign3300_e4975;
        locals.var_fn25_calc_iq__ffs_dn2 = assign3300_e4975_d_n2;
        locals.var_fn25_calc_iq__ffs_dn3 = assign3300_e4975_d_n3;
        locals.var_fn25_calc_iq__ffs_dn4 = assign3300_e4975_d_n4;
        locals.var_fn25_calc_iq__ffs_dn7 = assign3300_e4975_d_n7;
        locals.var_fn25_calc_iq__ffs_dn16 = assign3300_e4975_d_n16;
        locals.var_fn25_calc_iq__ffs_dn17 = assign3300_e4975_d_n17;

        let (assign3310_e4993, assign3310_e4993_d_n2, assign3310_e4993_d_n3, assign3310_e4993_d_n4, assign3310_e4993_d_n7, assign3310_e4993_d_n16, assign3310_e4993_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3310_e4979: f64 = (locals.var_fn25_calc_iq__vgdin - locals.var_fn25_calc_iq__vsx);
        let assign3310_e4983: f64 = (p.p51 * 0.1);
        let assign3310_e4985: f64 = (assign3310_e4983 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3310_e4987: f64 = (assign3310_e4985 * locals.var_fn25_calc_iq__ffs);
        let assign3310_e4988: f64 = (locals.var_fn25_calc_iq__vtdibl - assign3310_e4987);
        let assign3310_e4989: f64 = (assign3310_e4979 - assign3310_e4988);
        let assign3310_e4991: f64 = (assign3310_e4989 / locals.var_fn25_calc_iq__two_n_phit);
        (assign3310_e4991, (((locals.var_fn25_calc_iq__vgdin_dn2 - locals.var_fn25_calc_iq__vsx_dn2) - (-(assign3310_e4985 * locals.var_fn25_calc_iq__ffs_dn2))) / locals.var_fn25_calc_iq__two_n_phit), (((-locals.var_fn25_calc_iq__vsx_dn3) - (-(assign3310_e4985 * locals.var_fn25_calc_iq__ffs_dn3))) / locals.var_fn25_calc_iq__two_n_phit), (((((-locals.var_fn25_calc_iq__vsx_dn4) - (locals.var_fn25_calc_iq__vtdibl_dn4 - (((assign3310_e4983 * locals.var_fn25_calc_iq__alpha_phit_dn4) * locals.var_fn25_calc_iq__ffs) + (assign3310_e4985 * locals.var_fn25_calc_iq__ffs_dn4)))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3310_e4989 * locals.var_fn25_calc_iq__two_n_phit_dn4)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)), (((locals.var_fn25_calc_iq__vgdin_dn7 - locals.var_fn25_calc_iq__vsx_dn7) - (-(assign3310_e4985 * locals.var_fn25_calc_iq__ffs_dn7))) / locals.var_fn25_calc_iq__two_n_phit), (((((locals.var_fn25_calc_iq__vgdin_dn16 - locals.var_fn25_calc_iq__vsx_dn16) - (locals.var_fn25_calc_iq__vtdibl_dn16 - (assign3310_e4985 * locals.var_fn25_calc_iq__ffs_dn16))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3310_e4989 * locals.var_fn25_calc_iq__two_n_phit_dn16)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)), (((((locals.var_fn25_calc_iq__vgdin_dn17 - locals.var_fn25_calc_iq__vsx_dn17) - (locals.var_fn25_calc_iq__vtdibl_dn17 - (assign3310_e4985 * locals.var_fn25_calc_iq__ffs_dn17))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3310_e4989 * locals.var_fn25_calc_iq__two_n_phit_dn17)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn25_calc_iq__etas, locals.var_fn25_calc_iq__etas_dn2, locals.var_fn25_calc_iq__etas_dn3, locals.var_fn25_calc_iq__etas_dn4, locals.var_fn25_calc_iq__etas_dn7, locals.var_fn25_calc_iq__etas_dn16, locals.var_fn25_calc_iq__etas_dn17,)
    }
};
        locals.var_fn25_calc_iq__etas = assign3310_e4993;
        locals.var_fn25_calc_iq__etas_dn2 = assign3310_e4993_d_n2;
        locals.var_fn25_calc_iq__etas_dn3 = assign3310_e4993_d_n3;
        locals.var_fn25_calc_iq__etas_dn4 = assign3310_e4993_d_n4;
        locals.var_fn25_calc_iq__etas_dn7 = assign3310_e4993_d_n7;
        locals.var_fn25_calc_iq__etas_dn16 = assign3310_e4993_d_n16;
        locals.var_fn25_calc_iq__etas_dn17 = assign3310_e4993_d_n17;

        let assign3320_e4996: f64 = if locals.var_fn25_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign3320_e4996;

        let (assign3330_e5004, assign3330_e5004_d_n2, assign3330_e5004_d_n3, assign3330_e5004_d_n4, assign3330_e5004_d_n7, assign3330_e5004_d_n16, assign3330_e5004_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard33 != 0.0)) {
        let assign3330_e5002: f64 = (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etas);
        (assign3330_e5002, (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etas_dn2), (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etas_dn3), ((locals.var_fn25_calc_iq__qref_dn4 * locals.var_fn25_calc_iq__etas) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etas_dn4)), (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etas_dn7), ((locals.var_fn25_calc_iq__qref_dn16 * locals.var_fn25_calc_iq__etas) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etas_dn16)), ((locals.var_fn25_calc_iq__qref_dn17 * locals.var_fn25_calc_iq__etas) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etas_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qinvs, locals.var_fn25_calc_iq__qinvs_dn2, locals.var_fn25_calc_iq__qinvs_dn3, locals.var_fn25_calc_iq__qinvs_dn4, locals.var_fn25_calc_iq__qinvs_dn7, locals.var_fn25_calc_iq__qinvs_dn16, locals.var_fn25_calc_iq__qinvs_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs = assign3330_e5004;
        locals.var_fn25_calc_iq__qinvs_dn2 = assign3330_e5004_d_n2;
        locals.var_fn25_calc_iq__qinvs_dn3 = assign3330_e5004_d_n3;
        locals.var_fn25_calc_iq__qinvs_dn4 = assign3330_e5004_d_n4;
        locals.var_fn25_calc_iq__qinvs_dn7 = assign3330_e5004_d_n7;
        locals.var_fn25_calc_iq__qinvs_dn16 = assign3330_e5004_d_n16;
        locals.var_fn25_calc_iq__qinvs_dn17 = assign3330_e5004_d_n17;

        let assign3340_e5007: f64 = (-50.0);
        let assign3340_e5008: f64 = if locals.var_fn25_calc_iq__etas < assign3340_e5007 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign3340_e5008;

        let (assign3350_e5020, assign3350_e5020_d_n2, assign3350_e5020_d_n3, assign3350_e5020_d_n4, assign3350_e5020_d_n7, assign3350_e5020_d_n16, assign3350_e5020_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard33 == 0.0)) && (locals.var_guard34 != 0.0)) {
        let assign3350_e5017: f64 = (locals.var_fn25_calc_iq__etas).exp();
        let assign3350_e5018: f64 = (locals.var_fn25_calc_iq__qref * assign3350_e5017);
        (assign3350_e5018, (locals.var_fn25_calc_iq__qref * (assign3350_e5017 * locals.var_fn25_calc_iq__etas_dn2)), (locals.var_fn25_calc_iq__qref * (assign3350_e5017 * locals.var_fn25_calc_iq__etas_dn3)), ((locals.var_fn25_calc_iq__qref_dn4 * assign3350_e5017) + (locals.var_fn25_calc_iq__qref * (assign3350_e5017 * locals.var_fn25_calc_iq__etas_dn4))), (locals.var_fn25_calc_iq__qref * (assign3350_e5017 * locals.var_fn25_calc_iq__etas_dn7)), ((locals.var_fn25_calc_iq__qref_dn16 * assign3350_e5017) + (locals.var_fn25_calc_iq__qref * (assign3350_e5017 * locals.var_fn25_calc_iq__etas_dn16))), ((locals.var_fn25_calc_iq__qref_dn17 * assign3350_e5017) + (locals.var_fn25_calc_iq__qref * (assign3350_e5017 * locals.var_fn25_calc_iq__etas_dn17))),)
    } else {
        (locals.var_fn25_calc_iq__qinvs, locals.var_fn25_calc_iq__qinvs_dn2, locals.var_fn25_calc_iq__qinvs_dn3, locals.var_fn25_calc_iq__qinvs_dn4, locals.var_fn25_calc_iq__qinvs_dn7, locals.var_fn25_calc_iq__qinvs_dn16, locals.var_fn25_calc_iq__qinvs_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs = assign3350_e5020;
        locals.var_fn25_calc_iq__qinvs_dn2 = assign3350_e5020_d_n2;
        locals.var_fn25_calc_iq__qinvs_dn3 = assign3350_e5020_d_n3;
        locals.var_fn25_calc_iq__qinvs_dn4 = assign3350_e5020_d_n4;
        locals.var_fn25_calc_iq__qinvs_dn7 = assign3350_e5020_d_n7;
        locals.var_fn25_calc_iq__qinvs_dn16 = assign3350_e5020_d_n16;
        locals.var_fn25_calc_iq__qinvs_dn17 = assign3350_e5020_d_n17;

        let (assign3360_e5036, assign3360_e5036_d_n2, assign3360_e5036_d_n3, assign3360_e5036_d_n4, assign3360_e5036_d_n7, assign3360_e5036_d_n16, assign3360_e5036_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard33 == 0.0)) && (locals.var_guard34 == 0.0)) {
        let assign3360_e5031: f64 = (locals.var_fn25_calc_iq__etas).exp();
        let assign3360_e5032: f64 = (1.0 + assign3360_e5031);
        let assign3360_e5033: f64 = (assign3360_e5032).ln();
        let assign3360_e5034: f64 = (locals.var_fn25_calc_iq__qref * assign3360_e5033);
        (assign3360_e5034, (locals.var_fn25_calc_iq__qref * ((assign3360_e5031 * locals.var_fn25_calc_iq__etas_dn2) / assign3360_e5032)), (locals.var_fn25_calc_iq__qref * ((assign3360_e5031 * locals.var_fn25_calc_iq__etas_dn3) / assign3360_e5032)), ((locals.var_fn25_calc_iq__qref_dn4 * assign3360_e5033) + (locals.var_fn25_calc_iq__qref * ((assign3360_e5031 * locals.var_fn25_calc_iq__etas_dn4) / assign3360_e5032))), (locals.var_fn25_calc_iq__qref * ((assign3360_e5031 * locals.var_fn25_calc_iq__etas_dn7) / assign3360_e5032)), ((locals.var_fn25_calc_iq__qref_dn16 * assign3360_e5033) + (locals.var_fn25_calc_iq__qref * ((assign3360_e5031 * locals.var_fn25_calc_iq__etas_dn16) / assign3360_e5032))), ((locals.var_fn25_calc_iq__qref_dn17 * assign3360_e5033) + (locals.var_fn25_calc_iq__qref * ((assign3360_e5031 * locals.var_fn25_calc_iq__etas_dn17) / assign3360_e5032))),)
    } else {
        (locals.var_fn25_calc_iq__qinvs, locals.var_fn25_calc_iq__qinvs_dn2, locals.var_fn25_calc_iq__qinvs_dn3, locals.var_fn25_calc_iq__qinvs_dn4, locals.var_fn25_calc_iq__qinvs_dn7, locals.var_fn25_calc_iq__qinvs_dn16, locals.var_fn25_calc_iq__qinvs_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs = assign3360_e5036;
        locals.var_fn25_calc_iq__qinvs_dn2 = assign3360_e5036_d_n2;
        locals.var_fn25_calc_iq__qinvs_dn3 = assign3360_e5036_d_n3;
        locals.var_fn25_calc_iq__qinvs_dn4 = assign3360_e5036_d_n4;
        locals.var_fn25_calc_iq__qinvs_dn7 = assign3360_e5036_d_n7;
        locals.var_fn25_calc_iq__qinvs_dn16 = assign3360_e5036_d_n16;
        locals.var_fn25_calc_iq__qinvs_dn17 = assign3360_e5036_d_n17;

    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3370_e5044, assign3370_e5044_d_n2, assign3370_e5044_d_n3, assign3370_e5044_d_n4, assign3370_e5044_d_n7, assign3370_e5044_d_n16, assign3370_e5044_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3370_e5040: f64 = (locals.var_fn25_calc_iq__vgdin - locals.var_fn25_calc_iq__myarg);
        let assign3370_e5042: f64 = (assign3370_e5040 / locals.var_fn25_calc_iq__alpha_phit);
        (assign3370_e5042, ((locals.var_fn25_calc_iq__vgdin_dn2 - locals.var_fn25_calc_iq__myarg_dn2) / locals.var_fn25_calc_iq__alpha_phit), ((-locals.var_fn25_calc_iq__myarg_dn3) / locals.var_fn25_calc_iq__alpha_phit), ((((-locals.var_fn25_calc_iq__myarg_dn4) * locals.var_fn25_calc_iq__alpha_phit) - (assign3370_e5040 * locals.var_fn25_calc_iq__alpha_phit_dn4)) / (locals.var_fn25_calc_iq__alpha_phit * locals.var_fn25_calc_iq__alpha_phit)), ((locals.var_fn25_calc_iq__vgdin_dn7 - locals.var_fn25_calc_iq__myarg_dn7) / locals.var_fn25_calc_iq__alpha_phit), ((locals.var_fn25_calc_iq__vgdin_dn16 - locals.var_fn25_calc_iq__myarg_dn16) / locals.var_fn25_calc_iq__alpha_phit), ((locals.var_fn25_calc_iq__vgdin_dn17 - locals.var_fn25_calc_iq__myarg_dn17) / locals.var_fn25_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign3370_e5044;
        locals.var_fn25_calc_iq__exparg_dn2 = assign3370_e5044_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign3370_e5044_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign3370_e5044_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign3370_e5044_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign3370_e5044_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign3370_e5044_d_n17;

        let assign3380_e5047: f64 = if locals.var_fn25_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign3380_e5047;

        let (assign3390_e5053, assign3390_e5053_d_n2, assign3390_e5053_d_n3, assign3390_e5053_d_n4, assign3390_e5053_d_n7, assign3390_e5053_d_n16, assign3390_e5053_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard35 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffd, locals.var_fn25_calc_iq__ffd_dn2, locals.var_fn25_calc_iq__ffd_dn3, locals.var_fn25_calc_iq__ffd_dn4, locals.var_fn25_calc_iq__ffd_dn7, locals.var_fn25_calc_iq__ffd_dn16, locals.var_fn25_calc_iq__ffd_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd = assign3390_e5053;
        locals.var_fn25_calc_iq__ffd_dn2 = assign3390_e5053_d_n2;
        locals.var_fn25_calc_iq__ffd_dn3 = assign3390_e5053_d_n3;
        locals.var_fn25_calc_iq__ffd_dn4 = assign3390_e5053_d_n4;
        locals.var_fn25_calc_iq__ffd_dn7 = assign3390_e5053_d_n7;
        locals.var_fn25_calc_iq__ffd_dn16 = assign3390_e5053_d_n16;
        locals.var_fn25_calc_iq__ffd_dn17 = assign3390_e5053_d_n17;

        let assign3400_e5056: f64 = (-50.0);
        let assign3400_e5057: f64 = if locals.var_fn25_calc_iq__exparg < assign3400_e5056 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign3400_e5057;

        let (assign3410_e5066, assign3410_e5066_d_n2, assign3410_e5066_d_n3, assign3410_e5066_d_n4, assign3410_e5066_d_n7, assign3410_e5066_d_n16, assign3410_e5066_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard35 == 0.0)) && (locals.var_guard36 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffd, locals.var_fn25_calc_iq__ffd_dn2, locals.var_fn25_calc_iq__ffd_dn3, locals.var_fn25_calc_iq__ffd_dn4, locals.var_fn25_calc_iq__ffd_dn7, locals.var_fn25_calc_iq__ffd_dn16, locals.var_fn25_calc_iq__ffd_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd = assign3410_e5066;
        locals.var_fn25_calc_iq__ffd_dn2 = assign3410_e5066_d_n2;
        locals.var_fn25_calc_iq__ffd_dn3 = assign3410_e5066_d_n3;
        locals.var_fn25_calc_iq__ffd_dn4 = assign3410_e5066_d_n4;
        locals.var_fn25_calc_iq__ffd_dn7 = assign3410_e5066_d_n7;
        locals.var_fn25_calc_iq__ffd_dn16 = assign3410_e5066_d_n16;
        locals.var_fn25_calc_iq__ffd_dn17 = assign3410_e5066_d_n17;

        let (assign3420_e5081, assign3420_e5081_d_n2, assign3420_e5081_d_n3, assign3420_e5081_d_n4, assign3420_e5081_d_n7, assign3420_e5081_d_n16, assign3420_e5081_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard35 == 0.0)) && (locals.var_guard36 == 0.0)) {
        let assign3420_e5077: f64 = (locals.var_fn25_calc_iq__exparg).exp();
        let assign3420_e5078: f64 = (1.0 + assign3420_e5077);
        let assign3420_e5079: f64 = (1.0 / assign3420_e5078);
        (assign3420_e5079, (-((assign3420_e5077 * locals.var_fn25_calc_iq__exparg_dn2) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * locals.var_fn25_calc_iq__exparg_dn3) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * locals.var_fn25_calc_iq__exparg_dn4) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * locals.var_fn25_calc_iq__exparg_dn7) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * locals.var_fn25_calc_iq__exparg_dn16) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * locals.var_fn25_calc_iq__exparg_dn17) / (assign3420_e5078 * assign3420_e5078))),)
    } else {
        (locals.var_fn25_calc_iq__ffd, locals.var_fn25_calc_iq__ffd_dn2, locals.var_fn25_calc_iq__ffd_dn3, locals.var_fn25_calc_iq__ffd_dn4, locals.var_fn25_calc_iq__ffd_dn7, locals.var_fn25_calc_iq__ffd_dn16, locals.var_fn25_calc_iq__ffd_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd = assign3420_e5081;
        locals.var_fn25_calc_iq__ffd_dn2 = assign3420_e5081_d_n2;
        locals.var_fn25_calc_iq__ffd_dn3 = assign3420_e5081_d_n3;
        locals.var_fn25_calc_iq__ffd_dn4 = assign3420_e5081_d_n4;
        locals.var_fn25_calc_iq__ffd_dn7 = assign3420_e5081_d_n7;
        locals.var_fn25_calc_iq__ffd_dn16 = assign3420_e5081_d_n16;
        locals.var_fn25_calc_iq__ffd_dn17 = assign3420_e5081_d_n17;

        let (assign3430_e5099, assign3430_e5099_d_n2, assign3430_e5099_d_n3, assign3430_e5099_d_n4, assign3430_e5099_d_n7, assign3430_e5099_d_n16, assign3430_e5099_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3430_e5085: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vdx);
        let assign3430_e5089: f64 = (p.p51 * 0.1);
        let assign3430_e5091: f64 = (assign3430_e5089 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3430_e5093: f64 = (assign3430_e5091 * locals.var_fn25_calc_iq__ffd);
        let assign3430_e5094: f64 = (locals.var_fn25_calc_iq__vtdibl - assign3430_e5093);
        let assign3430_e5095: f64 = (assign3430_e5085 - assign3430_e5094);
        let assign3430_e5097: f64 = (assign3430_e5095 / locals.var_fn25_calc_iq__two_n_phit);
        (assign3430_e5097, (((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vdx_dn2) - (-(assign3430_e5091 * locals.var_fn25_calc_iq__ffd_dn2))) / locals.var_fn25_calc_iq__two_n_phit), (((-locals.var_fn25_calc_iq__vdx_dn3) - (-(assign3430_e5091 * locals.var_fn25_calc_iq__ffd_dn3))) / locals.var_fn25_calc_iq__two_n_phit), (((((-locals.var_fn25_calc_iq__vdx_dn4) - (locals.var_fn25_calc_iq__vtdibl_dn4 - (((assign3430_e5089 * locals.var_fn25_calc_iq__alpha_phit_dn4) * locals.var_fn25_calc_iq__ffd) + (assign3430_e5091 * locals.var_fn25_calc_iq__ffd_dn4)))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3430_e5095 * locals.var_fn25_calc_iq__two_n_phit_dn4)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)), (((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vdx_dn7) - (-(assign3430_e5091 * locals.var_fn25_calc_iq__ffd_dn7))) / locals.var_fn25_calc_iq__two_n_phit), (((((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vdx_dn16) - (locals.var_fn25_calc_iq__vtdibl_dn16 - (assign3430_e5091 * locals.var_fn25_calc_iq__ffd_dn16))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3430_e5095 * locals.var_fn25_calc_iq__two_n_phit_dn16)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)), (((((-locals.var_fn25_calc_iq__vdx_dn17) - (locals.var_fn25_calc_iq__vtdibl_dn17 - (assign3430_e5091 * locals.var_fn25_calc_iq__ffd_dn17))) * locals.var_fn25_calc_iq__two_n_phit) - (assign3430_e5095 * locals.var_fn25_calc_iq__two_n_phit_dn17)) / (locals.var_fn25_calc_iq__two_n_phit * locals.var_fn25_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn25_calc_iq__etad, locals.var_fn25_calc_iq__etad_dn2, locals.var_fn25_calc_iq__etad_dn3, locals.var_fn25_calc_iq__etad_dn4, locals.var_fn25_calc_iq__etad_dn7, locals.var_fn25_calc_iq__etad_dn16, locals.var_fn25_calc_iq__etad_dn17,)
    }
};
        locals.var_fn25_calc_iq__etad = assign3430_e5099;
        locals.var_fn25_calc_iq__etad_dn2 = assign3430_e5099_d_n2;
        locals.var_fn25_calc_iq__etad_dn3 = assign3430_e5099_d_n3;
        locals.var_fn25_calc_iq__etad_dn4 = assign3430_e5099_d_n4;
        locals.var_fn25_calc_iq__etad_dn7 = assign3430_e5099_d_n7;
        locals.var_fn25_calc_iq__etad_dn16 = assign3430_e5099_d_n16;
        locals.var_fn25_calc_iq__etad_dn17 = assign3430_e5099_d_n17;

        let assign3440_e5102: f64 = if locals.var_fn25_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign3440_e5102;

        let (assign3450_e5110, assign3450_e5110_d_n2, assign3450_e5110_d_n3, assign3450_e5110_d_n4, assign3450_e5110_d_n7, assign3450_e5110_d_n16, assign3450_e5110_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard37 != 0.0)) {
        let assign3450_e5108: f64 = (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etad);
        (assign3450_e5108, (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etad_dn2), (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etad_dn3), ((locals.var_fn25_calc_iq__qref_dn4 * locals.var_fn25_calc_iq__etad) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etad_dn4)), (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etad_dn7), ((locals.var_fn25_calc_iq__qref_dn16 * locals.var_fn25_calc_iq__etad) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etad_dn16)), ((locals.var_fn25_calc_iq__qref_dn17 * locals.var_fn25_calc_iq__etad) + (locals.var_fn25_calc_iq__qref * locals.var_fn25_calc_iq__etad_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qinvd, locals.var_fn25_calc_iq__qinvd_dn2, locals.var_fn25_calc_iq__qinvd_dn3, locals.var_fn25_calc_iq__qinvd_dn4, locals.var_fn25_calc_iq__qinvd_dn7, locals.var_fn25_calc_iq__qinvd_dn16, locals.var_fn25_calc_iq__qinvd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd = assign3450_e5110;
        locals.var_fn25_calc_iq__qinvd_dn2 = assign3450_e5110_d_n2;
        locals.var_fn25_calc_iq__qinvd_dn3 = assign3450_e5110_d_n3;
        locals.var_fn25_calc_iq__qinvd_dn4 = assign3450_e5110_d_n4;
        locals.var_fn25_calc_iq__qinvd_dn7 = assign3450_e5110_d_n7;
        locals.var_fn25_calc_iq__qinvd_dn16 = assign3450_e5110_d_n16;
        locals.var_fn25_calc_iq__qinvd_dn17 = assign3450_e5110_d_n17;

        let assign3460_e5113: f64 = (-50.0);
        let assign3460_e5114: f64 = if locals.var_fn25_calc_iq__etad < assign3460_e5113 { 1.0 } else { 0.0 };
        locals.var_guard38 = assign3460_e5114;

        let (assign3470_e5126, assign3470_e5126_d_n2, assign3470_e5126_d_n3, assign3470_e5126_d_n4, assign3470_e5126_d_n7, assign3470_e5126_d_n16, assign3470_e5126_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard37 == 0.0)) && (locals.var_guard38 != 0.0)) {
        let assign3470_e5123: f64 = (locals.var_fn25_calc_iq__etad).exp();
        let assign3470_e5124: f64 = (locals.var_fn25_calc_iq__qref * assign3470_e5123);
        (assign3470_e5124, (locals.var_fn25_calc_iq__qref * (assign3470_e5123 * locals.var_fn25_calc_iq__etad_dn2)), (locals.var_fn25_calc_iq__qref * (assign3470_e5123 * locals.var_fn25_calc_iq__etad_dn3)), ((locals.var_fn25_calc_iq__qref_dn4 * assign3470_e5123) + (locals.var_fn25_calc_iq__qref * (assign3470_e5123 * locals.var_fn25_calc_iq__etad_dn4))), (locals.var_fn25_calc_iq__qref * (assign3470_e5123 * locals.var_fn25_calc_iq__etad_dn7)), ((locals.var_fn25_calc_iq__qref_dn16 * assign3470_e5123) + (locals.var_fn25_calc_iq__qref * (assign3470_e5123 * locals.var_fn25_calc_iq__etad_dn16))), ((locals.var_fn25_calc_iq__qref_dn17 * assign3470_e5123) + (locals.var_fn25_calc_iq__qref * (assign3470_e5123 * locals.var_fn25_calc_iq__etad_dn17))),)
    } else {
        (locals.var_fn25_calc_iq__qinvd, locals.var_fn25_calc_iq__qinvd_dn2, locals.var_fn25_calc_iq__qinvd_dn3, locals.var_fn25_calc_iq__qinvd_dn4, locals.var_fn25_calc_iq__qinvd_dn7, locals.var_fn25_calc_iq__qinvd_dn16, locals.var_fn25_calc_iq__qinvd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd = assign3470_e5126;
        locals.var_fn25_calc_iq__qinvd_dn2 = assign3470_e5126_d_n2;
        locals.var_fn25_calc_iq__qinvd_dn3 = assign3470_e5126_d_n3;
        locals.var_fn25_calc_iq__qinvd_dn4 = assign3470_e5126_d_n4;
        locals.var_fn25_calc_iq__qinvd_dn7 = assign3470_e5126_d_n7;
        locals.var_fn25_calc_iq__qinvd_dn16 = assign3470_e5126_d_n16;
        locals.var_fn25_calc_iq__qinvd_dn17 = assign3470_e5126_d_n17;

        let (assign3480_e5142, assign3480_e5142_d_n2, assign3480_e5142_d_n3, assign3480_e5142_d_n4, assign3480_e5142_d_n7, assign3480_e5142_d_n16, assign3480_e5142_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard37 == 0.0)) && (locals.var_guard38 == 0.0)) {
        let assign3480_e5137: f64 = (locals.var_fn25_calc_iq__etad).exp();
        let assign3480_e5138: f64 = (1.0 + assign3480_e5137);
        let assign3480_e5139: f64 = (assign3480_e5138).ln();
        let assign3480_e5140: f64 = (locals.var_fn25_calc_iq__qref * assign3480_e5139);
        (assign3480_e5140, (locals.var_fn25_calc_iq__qref * ((assign3480_e5137 * locals.var_fn25_calc_iq__etad_dn2) / assign3480_e5138)), (locals.var_fn25_calc_iq__qref * ((assign3480_e5137 * locals.var_fn25_calc_iq__etad_dn3) / assign3480_e5138)), ((locals.var_fn25_calc_iq__qref_dn4 * assign3480_e5139) + (locals.var_fn25_calc_iq__qref * ((assign3480_e5137 * locals.var_fn25_calc_iq__etad_dn4) / assign3480_e5138))), (locals.var_fn25_calc_iq__qref * ((assign3480_e5137 * locals.var_fn25_calc_iq__etad_dn7) / assign3480_e5138)), ((locals.var_fn25_calc_iq__qref_dn16 * assign3480_e5139) + (locals.var_fn25_calc_iq__qref * ((assign3480_e5137 * locals.var_fn25_calc_iq__etad_dn16) / assign3480_e5138))), ((locals.var_fn25_calc_iq__qref_dn17 * assign3480_e5139) + (locals.var_fn25_calc_iq__qref * ((assign3480_e5137 * locals.var_fn25_calc_iq__etad_dn17) / assign3480_e5138))),)
    } else {
        (locals.var_fn25_calc_iq__qinvd, locals.var_fn25_calc_iq__qinvd_dn2, locals.var_fn25_calc_iq__qinvd_dn3, locals.var_fn25_calc_iq__qinvd_dn4, locals.var_fn25_calc_iq__qinvd_dn7, locals.var_fn25_calc_iq__qinvd_dn16, locals.var_fn25_calc_iq__qinvd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd = assign3480_e5142;
        locals.var_fn25_calc_iq__qinvd_dn2 = assign3480_e5142_d_n2;
        locals.var_fn25_calc_iq__qinvd_dn3 = assign3480_e5142_d_n3;
        locals.var_fn25_calc_iq__qinvd_dn4 = assign3480_e5142_d_n4;
        locals.var_fn25_calc_iq__qinvd_dn7 = assign3480_e5142_d_n7;
        locals.var_fn25_calc_iq__qinvd_dn16 = assign3480_e5142_d_n16;
        locals.var_fn25_calc_iq__qinvd_dn17 = assign3480_e5142_d_n17;

        let (assign3490_e5150, assign3490_e5150_d_n2, assign3490_e5150_d_n3, assign3490_e5150_d_n4, assign3490_e5150_d_n7, assign3490_e5150_d_n16, assign3490_e5150_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3490_e5146: f64 = (locals.var_fn25_calc_iq__qinvs - locals.var_fn25_calc_iq__qinvd);
        let assign3490_e5148: f64 = (assign3490_e5146 / locals.var_fn25_calc_iq__cgin);
        (assign3490_e5148, ((locals.var_fn25_calc_iq__qinvs_dn2 - locals.var_fn25_calc_iq__qinvd_dn2) / locals.var_fn25_calc_iq__cgin), ((locals.var_fn25_calc_iq__qinvs_dn3 - locals.var_fn25_calc_iq__qinvd_dn3) / locals.var_fn25_calc_iq__cgin), ((((locals.var_fn25_calc_iq__qinvs_dn4 - locals.var_fn25_calc_iq__qinvd_dn4) * locals.var_fn25_calc_iq__cgin) - (assign3490_e5146 * locals.var_fn25_calc_iq__cgin_dn4)) / (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__cgin)), ((locals.var_fn25_calc_iq__qinvs_dn7 - locals.var_fn25_calc_iq__qinvd_dn7) / locals.var_fn25_calc_iq__cgin), ((locals.var_fn25_calc_iq__qinvs_dn16 - locals.var_fn25_calc_iq__qinvd_dn16) / locals.var_fn25_calc_iq__cgin), ((locals.var_fn25_calc_iq__qinvs_dn17 - locals.var_fn25_calc_iq__qinvd_dn17) / locals.var_fn25_calc_iq__cgin),)
    } else {
        (locals.var_fn25_calc_iq__vdsc, locals.var_fn25_calc_iq__vdsc_dn2, locals.var_fn25_calc_iq__vdsc_dn3, locals.var_fn25_calc_iq__vdsc_dn4, locals.var_fn25_calc_iq__vdsc_dn7, locals.var_fn25_calc_iq__vdsc_dn16, locals.var_fn25_calc_iq__vdsc_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsc = assign3490_e5150;
        locals.var_fn25_calc_iq__vdsc_dn2 = assign3490_e5150_d_n2;
        locals.var_fn25_calc_iq__vdsc_dn3 = assign3490_e5150_d_n3;
        locals.var_fn25_calc_iq__vdsc_dn4 = assign3490_e5150_d_n4;
        locals.var_fn25_calc_iq__vdsc_dn7 = assign3490_e5150_d_n7;
        locals.var_fn25_calc_iq__vdsc_dn16 = assign3490_e5150_d_n16;
        locals.var_fn25_calc_iq__vdsc_dn17 = assign3490_e5150_d_n17;

        let (assign3500_e5156, assign3500_e5156_d_n2, assign3500_e5156_d_n3, assign3500_e5156_d_n4, assign3500_e5156_d_n7, assign3500_e5156_d_n16, assign3500_e5156_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3500_e5154: f64 = (locals.var_fn25_calc_iq__vdsc / locals.var_fn25_calc_iq__vdsat);
        (assign3500_e5154, (((locals.var_fn25_calc_iq__vdsc_dn2 * locals.var_fn25_calc_iq__vdsat) - (locals.var_fn25_calc_iq__vdsc * locals.var_fn25_calc_iq__vdsat_dn2)) / (locals.var_fn25_calc_iq__vdsat * locals.var_fn25_calc_iq__vdsat)), (((locals.var_fn25_calc_iq__vdsc_dn3 * locals.var_fn25_calc_iq__vdsat) - (locals.var_fn25_calc_iq__vdsc * locals.var_fn25_calc_iq__vdsat_dn3)) / (locals.var_fn25_calc_iq__vdsat * locals.var_fn25_calc_iq__vdsat)), (((locals.var_fn25_calc_iq__vdsc_dn4 * locals.var_fn25_calc_iq__vdsat) - (locals.var_fn25_calc_iq__vdsc * locals.var_fn25_calc_iq__vdsat_dn4)) / (locals.var_fn25_calc_iq__vdsat * locals.var_fn25_calc_iq__vdsat)), (((locals.var_fn25_calc_iq__vdsc_dn7 * locals.var_fn25_calc_iq__vdsat) - (locals.var_fn25_calc_iq__vdsc * locals.var_fn25_calc_iq__vdsat_dn7)) / (locals.var_fn25_calc_iq__vdsat * locals.var_fn25_calc_iq__vdsat)), (((locals.var_fn25_calc_iq__vdsc_dn16 * locals.var_fn25_calc_iq__vdsat) - (locals.var_fn25_calc_iq__vdsc * locals.var_fn25_calc_iq__vdsat_dn16)) / (locals.var_fn25_calc_iq__vdsat * locals.var_fn25_calc_iq__vdsat)), (((locals.var_fn25_calc_iq__vdsc_dn17 * locals.var_fn25_calc_iq__vdsat) - (locals.var_fn25_calc_iq__vdsc * locals.var_fn25_calc_iq__vdsat_dn17)) / (locals.var_fn25_calc_iq__vdsat * locals.var_fn25_calc_iq__vdsat)),)
    } else {
        (locals.var_fn25_calc_iq__myarg, locals.var_fn25_calc_iq__myarg_dn2, locals.var_fn25_calc_iq__myarg_dn3, locals.var_fn25_calc_iq__myarg_dn4, locals.var_fn25_calc_iq__myarg_dn7, locals.var_fn25_calc_iq__myarg_dn16, locals.var_fn25_calc_iq__myarg_dn17,)
    }
};
        locals.var_fn25_calc_iq__myarg = assign3500_e5156;
        locals.var_fn25_calc_iq__myarg_dn2 = assign3500_e5156_d_n2;
        locals.var_fn25_calc_iq__myarg_dn3 = assign3500_e5156_d_n3;
        locals.var_fn25_calc_iq__myarg_dn4 = assign3500_e5156_d_n4;
        locals.var_fn25_calc_iq__myarg_dn7 = assign3500_e5156_d_n7;
        locals.var_fn25_calc_iq__myarg_dn16 = assign3500_e5156_d_n16;
        locals.var_fn25_calc_iq__myarg_dn17 = assign3500_e5156_d_n17;

        let (assign3510_e5193, assign3510_e5193_d_n2, assign3510_e5193_d_n3, assign3510_e5193_d_n4, assign3510_e5193_d_n7, assign3510_e5193_d_n16, assign3510_e5193_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3510_e5183, assign3510_e5183_d_n2, assign3510_e5183_d_n3, assign3510_e5183_d_n4, assign3510_e5183_d_n7, assign3510_e5183_d_n16, assign3510_e5183_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3510_e5167: f64 = (0.001 / p.p53);
                let assign3510_e5169: f64 = (assign3510_e5167 * locals.var_fn25_calc_iq__myarg);
                let assign3510_e5170: f64 = (assign3510_e5169).tanh();
                let assign3510_e5171: f64 = (locals.var_fn25_calc_iq__myarg * assign3510_e5170);
                (assign3510_e5171, ((locals.var_fn25_calc_iq__myarg_dn2 * assign3510_e5170) + (locals.var_fn25_calc_iq__myarg * ((assign3510_e5167 * locals.var_fn25_calc_iq__myarg_dn2) / ((assign3510_e5169).cosh() * (assign3510_e5169).cosh())))), ((locals.var_fn25_calc_iq__myarg_dn3 * assign3510_e5170) + (locals.var_fn25_calc_iq__myarg * ((assign3510_e5167 * locals.var_fn25_calc_iq__myarg_dn3) / ((assign3510_e5169).cosh() * (assign3510_e5169).cosh())))), ((locals.var_fn25_calc_iq__myarg_dn4 * assign3510_e5170) + (locals.var_fn25_calc_iq__myarg * ((assign3510_e5167 * locals.var_fn25_calc_iq__myarg_dn4) / ((assign3510_e5169).cosh() * (assign3510_e5169).cosh())))), ((locals.var_fn25_calc_iq__myarg_dn7 * assign3510_e5170) + (locals.var_fn25_calc_iq__myarg * ((assign3510_e5167 * locals.var_fn25_calc_iq__myarg_dn7) / ((assign3510_e5169).cosh() * (assign3510_e5169).cosh())))), ((locals.var_fn25_calc_iq__myarg_dn16 * assign3510_e5170) + (locals.var_fn25_calc_iq__myarg * ((assign3510_e5167 * locals.var_fn25_calc_iq__myarg_dn16) / ((assign3510_e5169).cosh() * (assign3510_e5169).cosh())))), ((locals.var_fn25_calc_iq__myarg_dn17 * assign3510_e5170) + (locals.var_fn25_calc_iq__myarg * ((assign3510_e5167 * locals.var_fn25_calc_iq__myarg_dn17) / ((assign3510_e5169).cosh() * (assign3510_e5169).cosh())))),)
            } else {
                let (assign3510_e5182, assign3510_e5182_d_n2, assign3510_e5182_d_n3, assign3510_e5182_d_n4, assign3510_e5182_d_n7, assign3510_e5182_d_n16, assign3510_e5182_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3510_e5177: f64 = (locals.var_fn25_calc_iq__myarg * locals.var_fn25_calc_iq__myarg);
                        let assign3510_e5179: f64 = (assign3510_e5177 + p.p53);
                        let assign3510_e5180: f64 = (assign3510_e5179).sqrt();
                        (assign3510_e5180, (((locals.var_fn25_calc_iq__myarg_dn2 * locals.var_fn25_calc_iq__myarg) + (locals.var_fn25_calc_iq__myarg * locals.var_fn25_calc_iq__myarg_dn2)) / (2.0 * assign3510_e5180)), (((locals.var_fn25_calc_iq__myarg_dn3 * locals.var_fn25_calc_iq__myarg) + (locals.var_fn25_calc_iq__myarg * locals.var_fn25_calc_iq__myarg_dn3)) / (2.0 * assign3510_e5180)), (((locals.var_fn25_calc_iq__myarg_dn4 * locals.var_fn25_calc_iq__myarg) + (locals.var_fn25_calc_iq__myarg * locals.var_fn25_calc_iq__myarg_dn4)) / (2.0 * assign3510_e5180)), (((locals.var_fn25_calc_iq__myarg_dn7 * locals.var_fn25_calc_iq__myarg) + (locals.var_fn25_calc_iq__myarg * locals.var_fn25_calc_iq__myarg_dn7)) / (2.0 * assign3510_e5180)), (((locals.var_fn25_calc_iq__myarg_dn16 * locals.var_fn25_calc_iq__myarg) + (locals.var_fn25_calc_iq__myarg * locals.var_fn25_calc_iq__myarg_dn16)) / (2.0 * assign3510_e5180)), (((locals.var_fn25_calc_iq__myarg_dn17 * locals.var_fn25_calc_iq__myarg) + (locals.var_fn25_calc_iq__myarg * locals.var_fn25_calc_iq__myarg_dn17)) / (2.0 * assign3510_e5180)),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3510_e5182, assign3510_e5182_d_n2, assign3510_e5182_d_n3, assign3510_e5182_d_n4, assign3510_e5182_d_n7, assign3510_e5182_d_n16, assign3510_e5182_d_n17,)
            }
        };
        let assign3510_e5185: f64 = (assign3510_e5183).powf(locals.var_fn25_calc_iq__beta);
        let assign3510_e5186: f64 = (1.0 + assign3510_e5185);
        let assign3510_e5189: f64 = (1.0 / locals.var_fn25_calc_iq__beta);
        let assign3510_e5190: f64 = (assign3510_e5186).powf(assign3510_e5189);
        let assign3510_e5191: f64 = (locals.var_fn25_calc_iq__myarg / assign3510_e5190);
        (assign3510_e5191, (((locals.var_fn25_calc_iq__myarg_dn2 * assign3510_e5190) - (locals.var_fn25_calc_iq__myarg * if 0.0 == 0.0 && ((assign3510_e5189) as f64).is_finite() && ((assign3510_e5189) as f64).fract() == 0.0 { if assign3510_e5189 == 0.0 { 0.0 } else { (assign3510_e5189 * ((assign3510_e5186).powf(assign3510_e5189 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3510_e5183).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3510_e5183_d_n2)) } } else { (assign3510_e5185 * (locals.var_fn25_calc_iq__beta * (assign3510_e5183_d_n2 / assign3510_e5183))) })) } } else { (assign3510_e5190 * (assign3510_e5189 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3510_e5183).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3510_e5183_d_n2)) } } else { (assign3510_e5185 * (locals.var_fn25_calc_iq__beta * (assign3510_e5183_d_n2 / assign3510_e5183))) } / assign3510_e5186))) })) / (assign3510_e5190 * assign3510_e5190)), (((locals.var_fn25_calc_iq__myarg_dn3 * assign3510_e5190) - (locals.var_fn25_calc_iq__myarg * if 0.0 == 0.0 && ((assign3510_e5189) as f64).is_finite() && ((assign3510_e5189) as f64).fract() == 0.0 { if assign3510_e5189 == 0.0 { 0.0 } else { (assign3510_e5189 * ((assign3510_e5186).powf(assign3510_e5189 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3510_e5183).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3510_e5183_d_n3)) } } else { (assign3510_e5185 * (locals.var_fn25_calc_iq__beta * (assign3510_e5183_d_n3 / assign3510_e5183))) })) } } else { (assign3510_e5190 * (assign3510_e5189 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3510_e5183).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3510_e5183_d_n3)) } } else { (assign3510_e5185 * (locals.var_fn25_calc_iq__beta * (assign3510_e5183_d_n3 / assign3510_e5183))) } / assign3510_e5186))) })) / (assign3510_e5190 * assign3510_e5190)), (((locals.var_fn25_calc_iq__myarg_dn4 * assign3510_e5190) - (locals.var_fn25_calc_iq__myarg * if 0.0 == 0.0 && ((assign3510_e5189) as f64).is_finite() && ((assign3510_e5189) as f64).fract() == 0.0 { if assign3510_e5189 == 0.0 { 0.0 } else { (assign3510_e5189 * ((assign3510_e5186).powf(assign3510_e5189 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3510_e5183).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3510_e5183_d_n4)) } } else { (assign3510_e5185 * (locals.var_fn25_calc_iq__beta * (assign3510_e5183_d_n4 / assign3510_e5183))) })) } } else { (assign3510_e5190 * (assign3510_e5189 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3510_e5183).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3510_e5183_d_n4)) } } else { (assign3510_e5185 * (locals.var_fn25_calc_iq__beta * (assign3510_e5183_d_n4 / assign3510_e5183))) } / assign3510_e5186))) })) / (assign3510_e5190 * assign3510_e5190)), (((locals.var_fn25_calc_iq__myarg_dn7 * assign3510_e5190) - (locals.var_fn25_calc_iq__myarg * if 0.0 == 0.0 && ((assign3510_e5189) as f64).is_finite() && ((assign3510_e5189) as f64).fract() == 0.0 { if assign3510_e5189 == 0.0 { 0.0 } else { (assign3510_e5189 * ((assign3510_e5186).powf(assign3510_e5189 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3510_e5183).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3510_e5183_d_n7)) } } else { (assign3510_e5185 * (locals.var_fn25_calc_iq__beta * (assign3510_e5183_d_n7 / assign3510_e5183))) })) } } else { (assign3510_e5190 * (assign3510_e5189 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3510_e5183).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3510_e5183_d_n7)) } } else { (assign3510_e5185 * (locals.var_fn25_calc_iq__beta * (assign3510_e5183_d_n7 / assign3510_e5183))) } / assign3510_e5186))) })) / (assign3510_e5190 * assign3510_e5190)), (((locals.var_fn25_calc_iq__myarg_dn16 * assign3510_e5190) - (locals.var_fn25_calc_iq__myarg * if 0.0 == 0.0 && ((assign3510_e5189) as f64).is_finite() && ((assign3510_e5189) as f64).fract() == 0.0 { if assign3510_e5189 == 0.0 { 0.0 } else { (assign3510_e5189 * ((assign3510_e5186).powf(assign3510_e5189 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3510_e5183).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3510_e5183_d_n16)) } } else { (assign3510_e5185 * (locals.var_fn25_calc_iq__beta * (assign3510_e5183_d_n16 / assign3510_e5183))) })) } } else { (assign3510_e5190 * (assign3510_e5189 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3510_e5183).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3510_e5183_d_n16)) } } else { (assign3510_e5185 * (locals.var_fn25_calc_iq__beta * (assign3510_e5183_d_n16 / assign3510_e5183))) } / assign3510_e5186))) })) / (assign3510_e5190 * assign3510_e5190)), (((locals.var_fn25_calc_iq__myarg_dn17 * assign3510_e5190) - (locals.var_fn25_calc_iq__myarg * if 0.0 == 0.0 && ((assign3510_e5189) as f64).is_finite() && ((assign3510_e5189) as f64).fract() == 0.0 { if assign3510_e5189 == 0.0 { 0.0 } else { (assign3510_e5189 * ((assign3510_e5186).powf(assign3510_e5189 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3510_e5183).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3510_e5183_d_n17)) } } else { (assign3510_e5185 * (locals.var_fn25_calc_iq__beta * (assign3510_e5183_d_n17 / assign3510_e5183))) })) } } else { (assign3510_e5190 * (assign3510_e5189 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3510_e5183).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3510_e5183_d_n17)) } } else { (assign3510_e5185 * (locals.var_fn25_calc_iq__beta * (assign3510_e5183_d_n17 / assign3510_e5183))) } / assign3510_e5186))) })) / (assign3510_e5190 * assign3510_e5190)),)
    } else {
        (locals.var_fn25_calc_iq__fsat, locals.var_fn25_calc_iq__fsat_dn2, locals.var_fn25_calc_iq__fsat_dn3, locals.var_fn25_calc_iq__fsat_dn4, locals.var_fn25_calc_iq__fsat_dn7, locals.var_fn25_calc_iq__fsat_dn16, locals.var_fn25_calc_iq__fsat_dn17,)
    }
};
        locals.var_fn25_calc_iq__fsat = assign3510_e5193;
        locals.var_fn25_calc_iq__fsat_dn2 = assign3510_e5193_d_n2;
        locals.var_fn25_calc_iq__fsat_dn3 = assign3510_e5193_d_n3;
        locals.var_fn25_calc_iq__fsat_dn4 = assign3510_e5193_d_n4;
        locals.var_fn25_calc_iq__fsat_dn7 = assign3510_e5193_d_n7;
        locals.var_fn25_calc_iq__fsat_dn16 = assign3510_e5193_d_n16;
        locals.var_fn25_calc_iq__fsat_dn17 = assign3510_e5193_d_n17;

        let (assign3520_e5199, assign3520_e5199_d_n2, assign3520_e5199_d_n3, assign3520_e5199_d_n4, assign3520_e5199_d_n7, assign3520_e5199_d_n16, assign3520_e5199_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3520_e5197: f64 = (locals.var_fn25_calc_iq__vxf * locals.var_fn25_calc_iq__fsat);
        (assign3520_e5197, ((locals.var_fn25_calc_iq__vxf_dn2 * locals.var_fn25_calc_iq__fsat) + (locals.var_fn25_calc_iq__vxf * locals.var_fn25_calc_iq__fsat_dn2)), ((locals.var_fn25_calc_iq__vxf_dn3 * locals.var_fn25_calc_iq__fsat) + (locals.var_fn25_calc_iq__vxf * locals.var_fn25_calc_iq__fsat_dn3)), ((locals.var_fn25_calc_iq__vxf_dn4 * locals.var_fn25_calc_iq__fsat) + (locals.var_fn25_calc_iq__vxf * locals.var_fn25_calc_iq__fsat_dn4)), ((locals.var_fn25_calc_iq__vxf_dn7 * locals.var_fn25_calc_iq__fsat) + (locals.var_fn25_calc_iq__vxf * locals.var_fn25_calc_iq__fsat_dn7)), ((locals.var_fn25_calc_iq__vxf_dn16 * locals.var_fn25_calc_iq__fsat) + (locals.var_fn25_calc_iq__vxf * locals.var_fn25_calc_iq__fsat_dn16)), ((locals.var_fn25_calc_iq__vxf_dn17 * locals.var_fn25_calc_iq__fsat) + (locals.var_fn25_calc_iq__vxf * locals.var_fn25_calc_iq__fsat_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__vel, locals.var_fn25_calc_iq__vel_dn2, locals.var_fn25_calc_iq__vel_dn3, locals.var_fn25_calc_iq__vel_dn4, locals.var_fn25_calc_iq__vel_dn7, locals.var_fn25_calc_iq__vel_dn16, locals.var_fn25_calc_iq__vel_dn17,)
    }
};
        locals.var_fn25_calc_iq__vel = assign3520_e5199;
        locals.var_fn25_calc_iq__vel_dn2 = assign3520_e5199_d_n2;
        locals.var_fn25_calc_iq__vel_dn3 = assign3520_e5199_d_n3;
        locals.var_fn25_calc_iq__vel_dn4 = assign3520_e5199_d_n4;
        locals.var_fn25_calc_iq__vel_dn7 = assign3520_e5199_d_n7;
        locals.var_fn25_calc_iq__vel_dn16 = assign3520_e5199_d_n16;
        locals.var_fn25_calc_iq__vel_dn17 = assign3520_e5199_d_n17;

        let (assign3530_e5217, assign3530_e5217_d_n2, assign3530_e5217_d_n3, assign3530_e5217_d_n4, assign3530_e5217_d_n7, assign3530_e5217_d_n16, assign3530_e5217_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3530_e5203: f64 = (locals.var_fn25_calc_iq__type * locals.var_fn25_calc_iq__w);
        let assign3530_e5205: f64 = (assign3530_e5203 * locals.var_fn25_calc_iq__ngf);
        let assign3530_e5207: f64 = (assign3530_e5205 * 0.5);
        let assign3530_e5210: f64 = (locals.var_fn25_calc_iq__qinvs + locals.var_fn25_calc_iq__qinvd);
        let assign3530_e5211: f64 = (assign3530_e5207 * assign3530_e5210);
        let assign3530_e5213: f64 = (assign3530_e5211 * locals.var_fn25_calc_iq__vel);
        let assign3530_e5215: f64 = (assign3530_e5213 * locals.var_fn25_calc_iq__trapfracdl);
        (assign3530_e5215, ((((assign3530_e5207 * (locals.var_fn25_calc_iq__qinvs_dn2 + locals.var_fn25_calc_iq__qinvd_dn2)) * locals.var_fn25_calc_iq__vel) + (assign3530_e5211 * locals.var_fn25_calc_iq__vel_dn2)) * locals.var_fn25_calc_iq__trapfracdl), ((((assign3530_e5207 * (locals.var_fn25_calc_iq__qinvs_dn3 + locals.var_fn25_calc_iq__qinvd_dn3)) * locals.var_fn25_calc_iq__vel) + (assign3530_e5211 * locals.var_fn25_calc_iq__vel_dn3)) * locals.var_fn25_calc_iq__trapfracdl), ((((assign3530_e5207 * (locals.var_fn25_calc_iq__qinvs_dn4 + locals.var_fn25_calc_iq__qinvd_dn4)) * locals.var_fn25_calc_iq__vel) + (assign3530_e5211 * locals.var_fn25_calc_iq__vel_dn4)) * locals.var_fn25_calc_iq__trapfracdl), ((((assign3530_e5207 * (locals.var_fn25_calc_iq__qinvs_dn7 + locals.var_fn25_calc_iq__qinvd_dn7)) * locals.var_fn25_calc_iq__vel) + (assign3530_e5211 * locals.var_fn25_calc_iq__vel_dn7)) * locals.var_fn25_calc_iq__trapfracdl), ((((assign3530_e5207 * (locals.var_fn25_calc_iq__qinvs_dn16 + locals.var_fn25_calc_iq__qinvd_dn16)) * locals.var_fn25_calc_iq__vel) + (assign3530_e5211 * locals.var_fn25_calc_iq__vel_dn16)) * locals.var_fn25_calc_iq__trapfracdl), ((((assign3530_e5207 * (locals.var_fn25_calc_iq__qinvs_dn17 + locals.var_fn25_calc_iq__qinvd_dn17)) * locals.var_fn25_calc_iq__vel) + (assign3530_e5211 * locals.var_fn25_calc_iq__vel_dn17)) * locals.var_fn25_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn25_calc_iq__idsout, locals.var_fn25_calc_iq__idsout_dn2, locals.var_fn25_calc_iq__idsout_dn3, locals.var_fn25_calc_iq__idsout_dn4, locals.var_fn25_calc_iq__idsout_dn7, locals.var_fn25_calc_iq__idsout_dn16, locals.var_fn25_calc_iq__idsout_dn17,)
    }
};
        locals.var_fn25_calc_iq__idsout = assign3530_e5217;
        locals.var_fn25_calc_iq__idsout_dn2 = assign3530_e5217_d_n2;
        locals.var_fn25_calc_iq__idsout_dn3 = assign3530_e5217_d_n3;
        locals.var_fn25_calc_iq__idsout_dn4 = assign3530_e5217_d_n4;
        locals.var_fn25_calc_iq__idsout_dn7 = assign3530_e5217_d_n7;
        locals.var_fn25_calc_iq__idsout_dn16 = assign3530_e5217_d_n16;
        locals.var_fn25_calc_iq__idsout_dn17 = assign3530_e5217_d_n17;

        let (assign3540_e5225, assign3540_e5225_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3540_e5222: f64 = (2.302585092994046 * locals.var_fn25_calc_iq__phitin);
        let assign3540_e5223: f64 = (locals.var_fn25_calc_iq__ss / assign3540_e5222);
        (assign3540_e5223, (-((locals.var_fn25_calc_iq__ss * (2.302585092994046 * locals.var_fn25_calc_iq__phitin_dn4)) / (assign3540_e5222 * assign3540_e5222))),)
    } else {
        (locals.var_fn25_calc_iq__n0, locals.var_fn25_calc_iq__n0_dn4,)
    }
};
        locals.var_fn25_calc_iq__n0 = assign3540_e5225;
        locals.var_fn25_calc_iq__n0_dn4 = assign3540_e5225_d_n4;

        let (assign3550_e5233, assign3550_e5233_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3550_e5229: f64 = (2.0 * locals.var_fn25_calc_iq__n0);
        let assign3550_e5231: f64 = (assign3550_e5229 * locals.var_fn25_calc_iq__phitin);
        (assign3550_e5231, (((2.0 * locals.var_fn25_calc_iq__n0_dn4) * locals.var_fn25_calc_iq__phitin) + (assign3550_e5229 * locals.var_fn25_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn25_calc_iq__two_n_phit0, locals.var_fn25_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn25_calc_iq__two_n_phit0 = assign3550_e5233;
        locals.var_fn25_calc_iq__two_n_phit0_dn4 = assign3550_e5233_d_n4;

        let (assign3560_e5239, assign3560_e5239_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3560_e5237: f64 = (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__two_n_phit0);
        (assign3560_e5237, ((locals.var_fn25_calc_iq__cgin_dn4 * locals.var_fn25_calc_iq__two_n_phit0) + (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn25_calc_iq__qref0, locals.var_fn25_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn25_calc_iq__qref0 = assign3560_e5239;
        locals.var_fn25_calc_iq__qref0_dn4 = assign3560_e5239_d_n4;

        let (assign3570_e5249, assign3570_e5249_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3570_e5244: f64 = (p.p51 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3570_e5246: f64 = (assign3570_e5244 / 2.0);
        let assign3570_e5247: f64 = (locals.var_fn25_calc_iq__vtof - assign3570_e5246);
        (assign3570_e5247, (locals.var_fn25_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn25_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn25_calc_iq__myarg0, locals.var_fn25_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn25_calc_iq__myarg0 = assign3570_e5249;
        locals.var_fn25_calc_iq__myarg0_dn4 = assign3570_e5249_d_n4;

        let (assign3580_e5300, assign3580_e5300_d_n2, assign3580_e5300_d_n4, assign3580_e5300_d_n7, assign3580_e5300_d_n16, assign3580_e5300_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3580_e5294, assign3580_e5294_d_n2, assign3580_e5294_d_n7, assign3580_e5294_d_n16, assign3580_e5294_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3580_e5258: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                let assign3580_e5261: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3580_e5264: f64 = (0.001 / p.p53);
                let assign3580_e5267: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3580_e5268: f64 = (assign3580_e5264 * assign3580_e5267);
                let assign3580_e5269: f64 = (assign3580_e5268).tanh();
                let assign3580_e5270: f64 = (assign3580_e5261 * assign3580_e5269);
                let assign3580_e5271: f64 = (assign3580_e5258 + assign3580_e5270);
                let assign3580_e5272: f64 = (0.5 * assign3580_e5271);
                (assign3580_e5272, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + (((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3580_e5269) + (assign3580_e5261 * ((assign3580_e5264 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2)) / ((assign3580_e5268).cosh() * (assign3580_e5268).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + (((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3580_e5269) + (assign3580_e5261 * ((assign3580_e5264 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7)) / ((assign3580_e5268).cosh() * (assign3580_e5268).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + (((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3580_e5269) + (assign3580_e5261 * ((assign3580_e5264 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16)) / ((assign3580_e5268).cosh() * (assign3580_e5268).cosh())))))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + (((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3580_e5269) + (assign3580_e5261 * ((assign3580_e5264 * (-locals.var_fn25_calc_iq__vgdin_dn17)) / ((assign3580_e5268).cosh() * (assign3580_e5268).cosh())))))),)
            } else {
                let (assign3580_e5293, assign3580_e5293_d_n2, assign3580_e5293_d_n7, assign3580_e5293_d_n16, assign3580_e5293_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3580_e5279: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                        let assign3580_e5282: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3580_e5285: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3580_e5286: f64 = (assign3580_e5282 * assign3580_e5285);
                        let assign3580_e5288: f64 = (assign3580_e5286 + p.p53);
                        let assign3580_e5289: f64 = (assign3580_e5288).sqrt();
                        let assign3580_e5290: f64 = (assign3580_e5279 + assign3580_e5289);
                        let assign3580_e5291: f64 = (0.5 * assign3580_e5290);
                        (assign3580_e5291, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + ((((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3580_e5285) + (assign3580_e5282 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2))) / (2.0 * assign3580_e5289)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + ((((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3580_e5285) + (assign3580_e5282 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7))) / (2.0 * assign3580_e5289)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + ((((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3580_e5285) + (assign3580_e5282 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16))) / (2.0 * assign3580_e5289)))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + ((((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3580_e5285) + (assign3580_e5282 * (-locals.var_fn25_calc_iq__vgdin_dn17))) / (2.0 * assign3580_e5289)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3580_e5293, assign3580_e5293_d_n2, assign3580_e5293_d_n7, assign3580_e5293_d_n16, assign3580_e5293_d_n17,)
            }
        };
        let assign3580_e5296: f64 = (assign3580_e5294 - locals.var_fn25_calc_iq__myarg0);
        let assign3580_e5298: f64 = (assign3580_e5296 / locals.var_fn25_calc_iq__alpha_phit);
        (assign3580_e5298, (assign3580_e5294_d_n2 / locals.var_fn25_calc_iq__alpha_phit), ((((-locals.var_fn25_calc_iq__myarg0_dn4) * locals.var_fn25_calc_iq__alpha_phit) - (assign3580_e5296 * locals.var_fn25_calc_iq__alpha_phit_dn4)) / (locals.var_fn25_calc_iq__alpha_phit * locals.var_fn25_calc_iq__alpha_phit)), (assign3580_e5294_d_n7 / locals.var_fn25_calc_iq__alpha_phit), (assign3580_e5294_d_n16 / locals.var_fn25_calc_iq__alpha_phit), (assign3580_e5294_d_n17 / locals.var_fn25_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn25_calc_iq__exparg0, locals.var_fn25_calc_iq__exparg0_dn2, locals.var_fn25_calc_iq__exparg0_dn4, locals.var_fn25_calc_iq__exparg0_dn7, locals.var_fn25_calc_iq__exparg0_dn16, locals.var_fn25_calc_iq__exparg0_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg0 = assign3580_e5300;
        locals.var_fn25_calc_iq__exparg0_dn2 = assign3580_e5300_d_n2;
        locals.var_fn25_calc_iq__exparg0_dn4 = assign3580_e5300_d_n4;
        locals.var_fn25_calc_iq__exparg0_dn7 = assign3580_e5300_d_n7;
        locals.var_fn25_calc_iq__exparg0_dn16 = assign3580_e5300_d_n16;
        locals.var_fn25_calc_iq__exparg0_dn17 = assign3580_e5300_d_n17;

        let assign3590_e5303: f64 = if locals.var_fn25_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign3590_e5303;

        let (assign3600_e5309, assign3600_e5309_d_n2, assign3600_e5309_d_n4, assign3600_e5309_d_n7, assign3600_e5309_d_n16, assign3600_e5309_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard39 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ff0, locals.var_fn25_calc_iq__ff0_dn2, locals.var_fn25_calc_iq__ff0_dn4, locals.var_fn25_calc_iq__ff0_dn7, locals.var_fn25_calc_iq__ff0_dn16, locals.var_fn25_calc_iq__ff0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff0 = assign3600_e5309;
        locals.var_fn25_calc_iq__ff0_dn2 = assign3600_e5309_d_n2;
        locals.var_fn25_calc_iq__ff0_dn4 = assign3600_e5309_d_n4;
        locals.var_fn25_calc_iq__ff0_dn7 = assign3600_e5309_d_n7;
        locals.var_fn25_calc_iq__ff0_dn16 = assign3600_e5309_d_n16;
        locals.var_fn25_calc_iq__ff0_dn17 = assign3600_e5309_d_n17;

        let assign3610_e5312: f64 = (-50.0);
        let assign3610_e5313: f64 = if locals.var_fn25_calc_iq__exparg0 < assign3610_e5312 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign3610_e5313;

        let (assign3620_e5322, assign3620_e5322_d_n2, assign3620_e5322_d_n4, assign3620_e5322_d_n7, assign3620_e5322_d_n16, assign3620_e5322_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard39 == 0.0)) && (locals.var_guard40 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ff0, locals.var_fn25_calc_iq__ff0_dn2, locals.var_fn25_calc_iq__ff0_dn4, locals.var_fn25_calc_iq__ff0_dn7, locals.var_fn25_calc_iq__ff0_dn16, locals.var_fn25_calc_iq__ff0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff0 = assign3620_e5322;
        locals.var_fn25_calc_iq__ff0_dn2 = assign3620_e5322_d_n2;
        locals.var_fn25_calc_iq__ff0_dn4 = assign3620_e5322_d_n4;
        locals.var_fn25_calc_iq__ff0_dn7 = assign3620_e5322_d_n7;
        locals.var_fn25_calc_iq__ff0_dn16 = assign3620_e5322_d_n16;
        locals.var_fn25_calc_iq__ff0_dn17 = assign3620_e5322_d_n17;

        let (assign3630_e5337, assign3630_e5337_d_n2, assign3630_e5337_d_n4, assign3630_e5337_d_n7, assign3630_e5337_d_n16, assign3630_e5337_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard39 == 0.0)) && (locals.var_guard40 == 0.0)) {
        let assign3630_e5333: f64 = (locals.var_fn25_calc_iq__exparg0).exp();
        let assign3630_e5334: f64 = (1.0 + assign3630_e5333);
        let assign3630_e5335: f64 = (1.0 / assign3630_e5334);
        (assign3630_e5335, (-((assign3630_e5333 * locals.var_fn25_calc_iq__exparg0_dn2) / (assign3630_e5334 * assign3630_e5334))), (-((assign3630_e5333 * locals.var_fn25_calc_iq__exparg0_dn4) / (assign3630_e5334 * assign3630_e5334))), (-((assign3630_e5333 * locals.var_fn25_calc_iq__exparg0_dn7) / (assign3630_e5334 * assign3630_e5334))), (-((assign3630_e5333 * locals.var_fn25_calc_iq__exparg0_dn16) / (assign3630_e5334 * assign3630_e5334))), (-((assign3630_e5333 * locals.var_fn25_calc_iq__exparg0_dn17) / (assign3630_e5334 * assign3630_e5334))),)
    } else {
        (locals.var_fn25_calc_iq__ff0, locals.var_fn25_calc_iq__ff0_dn2, locals.var_fn25_calc_iq__ff0_dn4, locals.var_fn25_calc_iq__ff0_dn7, locals.var_fn25_calc_iq__ff0_dn16, locals.var_fn25_calc_iq__ff0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ff0 = assign3630_e5337;
        locals.var_fn25_calc_iq__ff0_dn2 = assign3630_e5337_d_n2;
        locals.var_fn25_calc_iq__ff0_dn4 = assign3630_e5337_d_n4;
        locals.var_fn25_calc_iq__ff0_dn7 = assign3630_e5337_d_n7;
        locals.var_fn25_calc_iq__ff0_dn16 = assign3630_e5337_d_n16;
        locals.var_fn25_calc_iq__ff0_dn17 = assign3630_e5337_d_n17;

        let (assign3640_e5396, assign3640_e5396_d_n2, assign3640_e5396_d_n4, assign3640_e5396_d_n7, assign3640_e5396_d_n16, assign3640_e5396_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3640_e5382, assign3640_e5382_d_n2, assign3640_e5382_d_n7, assign3640_e5382_d_n16, assign3640_e5382_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3640_e5346: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                let assign3640_e5349: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3640_e5352: f64 = (0.001 / p.p53);
                let assign3640_e5355: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                let assign3640_e5356: f64 = (assign3640_e5352 * assign3640_e5355);
                let assign3640_e5357: f64 = (assign3640_e5356).tanh();
                let assign3640_e5358: f64 = (assign3640_e5349 * assign3640_e5357);
                let assign3640_e5359: f64 = (assign3640_e5346 + assign3640_e5358);
                let assign3640_e5360: f64 = (0.5 * assign3640_e5359);
                (assign3640_e5360, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + (((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3640_e5357) + (assign3640_e5349 * ((assign3640_e5352 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2)) / ((assign3640_e5356).cosh() * (assign3640_e5356).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + (((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3640_e5357) + (assign3640_e5349 * ((assign3640_e5352 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7)) / ((assign3640_e5356).cosh() * (assign3640_e5356).cosh())))))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + (((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3640_e5357) + (assign3640_e5349 * ((assign3640_e5352 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16)) / ((assign3640_e5356).cosh() * (assign3640_e5356).cosh())))))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + (((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3640_e5357) + (assign3640_e5349 * ((assign3640_e5352 * (-locals.var_fn25_calc_iq__vgdin_dn17)) / ((assign3640_e5356).cosh() * (assign3640_e5356).cosh())))))),)
            } else {
                let (assign3640_e5381, assign3640_e5381_d_n2, assign3640_e5381_d_n7, assign3640_e5381_d_n16, assign3640_e5381_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3640_e5367: f64 = (locals.var_fn25_calc_iq__vgsin + locals.var_fn25_calc_iq__vgdin);
                        let assign3640_e5370: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3640_e5373: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vgdin);
                        let assign3640_e5374: f64 = (assign3640_e5370 * assign3640_e5373);
                        let assign3640_e5376: f64 = (assign3640_e5374 + p.p53);
                        let assign3640_e5377: f64 = (assign3640_e5376).sqrt();
                        let assign3640_e5378: f64 = (assign3640_e5367 + assign3640_e5377);
                        let assign3640_e5379: f64 = (0.5 * assign3640_e5378);
                        (assign3640_e5379, (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn2 + locals.var_fn25_calc_iq__vgdin_dn2) + ((((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2) * assign3640_e5373) + (assign3640_e5370 * (locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vgdin_dn2))) / (2.0 * assign3640_e5377)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn7 + locals.var_fn25_calc_iq__vgdin_dn7) + ((((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7) * assign3640_e5373) + (assign3640_e5370 * (locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vgdin_dn7))) / (2.0 * assign3640_e5377)))), (0.5 * ((locals.var_fn25_calc_iq__vgsin_dn16 + locals.var_fn25_calc_iq__vgdin_dn16) + ((((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16) * assign3640_e5373) + (assign3640_e5370 * (locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vgdin_dn16))) / (2.0 * assign3640_e5377)))), (0.5 * (locals.var_fn25_calc_iq__vgdin_dn17 + ((((-locals.var_fn25_calc_iq__vgdin_dn17) * assign3640_e5373) + (assign3640_e5370 * (-locals.var_fn25_calc_iq__vgdin_dn17))) / (2.0 * assign3640_e5377)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3640_e5381, assign3640_e5381_d_n2, assign3640_e5381_d_n7, assign3640_e5381_d_n16, assign3640_e5381_d_n17,)
            }
        };
        let assign3640_e5386: f64 = (p.p51 * 0.1);
        let assign3640_e5388: f64 = (assign3640_e5386 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3640_e5390: f64 = (assign3640_e5388 * locals.var_fn25_calc_iq__ff0);
        let assign3640_e5391: f64 = (locals.var_fn25_calc_iq__vtof - assign3640_e5390);
        let assign3640_e5392: f64 = (assign3640_e5382 - assign3640_e5391);
        let assign3640_e5394: f64 = (assign3640_e5392 / locals.var_fn25_calc_iq__two_n_phit0);
        (assign3640_e5394, ((assign3640_e5382_d_n2 - (-(assign3640_e5388 * locals.var_fn25_calc_iq__ff0_dn2))) / locals.var_fn25_calc_iq__two_n_phit0), ((((-(locals.var_fn25_calc_iq__vtof_dn4 - (((assign3640_e5386 * locals.var_fn25_calc_iq__alpha_phit_dn4) * locals.var_fn25_calc_iq__ff0) + (assign3640_e5388 * locals.var_fn25_calc_iq__ff0_dn4)))) * locals.var_fn25_calc_iq__two_n_phit0) - (assign3640_e5392 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) / (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__two_n_phit0)), ((assign3640_e5382_d_n7 - (-(assign3640_e5388 * locals.var_fn25_calc_iq__ff0_dn7))) / locals.var_fn25_calc_iq__two_n_phit0), ((assign3640_e5382_d_n16 - (-(assign3640_e5388 * locals.var_fn25_calc_iq__ff0_dn16))) / locals.var_fn25_calc_iq__two_n_phit0), ((assign3640_e5382_d_n17 - (-(assign3640_e5388 * locals.var_fn25_calc_iq__ff0_dn17))) / locals.var_fn25_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn25_calc_iq__eta0, locals.var_fn25_calc_iq__eta0_dn2, locals.var_fn25_calc_iq__eta0_dn4, locals.var_fn25_calc_iq__eta0_dn7, locals.var_fn25_calc_iq__eta0_dn16, locals.var_fn25_calc_iq__eta0_dn17,)
    }
};
        locals.var_fn25_calc_iq__eta0 = assign3640_e5396;
        locals.var_fn25_calc_iq__eta0_dn2 = assign3640_e5396_d_n2;
        locals.var_fn25_calc_iq__eta0_dn4 = assign3640_e5396_d_n4;
        locals.var_fn25_calc_iq__eta0_dn7 = assign3640_e5396_d_n7;
        locals.var_fn25_calc_iq__eta0_dn16 = assign3640_e5396_d_n16;
        locals.var_fn25_calc_iq__eta0_dn17 = assign3640_e5396_d_n17;

        let assign3650_e5399: f64 = if locals.var_fn25_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign3650_e5399;

        let (assign3660_e5407, assign3660_e5407_d_n2, assign3660_e5407_d_n4, assign3660_e5407_d_n7, assign3660_e5407_d_n16, assign3660_e5407_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard41 != 0.0)) {
        let assign3660_e5405: f64 = (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__eta0);
        (assign3660_e5405, (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__eta0_dn2), ((locals.var_fn25_calc_iq__qref0_dn4 * locals.var_fn25_calc_iq__eta0) + (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__eta0_dn4)), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__eta0_dn7), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__eta0_dn16), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__eta0_dn17),)
    } else {
        (locals.var_fn25_calc_iq__qinvv0, locals.var_fn25_calc_iq__qinvv0_dn2, locals.var_fn25_calc_iq__qinvv0_dn4, locals.var_fn25_calc_iq__qinvv0_dn7, locals.var_fn25_calc_iq__qinvv0_dn16, locals.var_fn25_calc_iq__qinvv0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv0 = assign3660_e5407;
        locals.var_fn25_calc_iq__qinvv0_dn2 = assign3660_e5407_d_n2;
        locals.var_fn25_calc_iq__qinvv0_dn4 = assign3660_e5407_d_n4;
        locals.var_fn25_calc_iq__qinvv0_dn7 = assign3660_e5407_d_n7;
        locals.var_fn25_calc_iq__qinvv0_dn16 = assign3660_e5407_d_n16;
        locals.var_fn25_calc_iq__qinvv0_dn17 = assign3660_e5407_d_n17;

        let assign3670_e5410: f64 = (-50.0);
        let assign3670_e5411: f64 = if locals.var_fn25_calc_iq__eta0 < assign3670_e5410 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign3670_e5411;

        let (assign3680_e5423, assign3680_e5423_d_n2, assign3680_e5423_d_n4, assign3680_e5423_d_n7, assign3680_e5423_d_n16, assign3680_e5423_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard41 == 0.0)) && (locals.var_guard42 != 0.0)) {
        let assign3680_e5420: f64 = (locals.var_fn25_calc_iq__eta0).exp();
        let assign3680_e5421: f64 = (locals.var_fn25_calc_iq__qref0 * assign3680_e5420);
        (assign3680_e5421, (locals.var_fn25_calc_iq__qref0 * (assign3680_e5420 * locals.var_fn25_calc_iq__eta0_dn2)), ((locals.var_fn25_calc_iq__qref0_dn4 * assign3680_e5420) + (locals.var_fn25_calc_iq__qref0 * (assign3680_e5420 * locals.var_fn25_calc_iq__eta0_dn4))), (locals.var_fn25_calc_iq__qref0 * (assign3680_e5420 * locals.var_fn25_calc_iq__eta0_dn7)), (locals.var_fn25_calc_iq__qref0 * (assign3680_e5420 * locals.var_fn25_calc_iq__eta0_dn16)), (locals.var_fn25_calc_iq__qref0 * (assign3680_e5420 * locals.var_fn25_calc_iq__eta0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qinvv0, locals.var_fn25_calc_iq__qinvv0_dn2, locals.var_fn25_calc_iq__qinvv0_dn4, locals.var_fn25_calc_iq__qinvv0_dn7, locals.var_fn25_calc_iq__qinvv0_dn16, locals.var_fn25_calc_iq__qinvv0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv0 = assign3680_e5423;
        locals.var_fn25_calc_iq__qinvv0_dn2 = assign3680_e5423_d_n2;
        locals.var_fn25_calc_iq__qinvv0_dn4 = assign3680_e5423_d_n4;
        locals.var_fn25_calc_iq__qinvv0_dn7 = assign3680_e5423_d_n7;
        locals.var_fn25_calc_iq__qinvv0_dn16 = assign3680_e5423_d_n16;
        locals.var_fn25_calc_iq__qinvv0_dn17 = assign3680_e5423_d_n17;

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3690_e5439, assign3690_e5439_d_n2, assign3690_e5439_d_n4, assign3690_e5439_d_n7, assign3690_e5439_d_n16, assign3690_e5439_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard41 == 0.0)) && (locals.var_guard42 == 0.0)) {
        let assign3690_e5434: f64 = (locals.var_fn25_calc_iq__eta0).exp();
        let assign3690_e5435: f64 = (1.0 + assign3690_e5434);
        let assign3690_e5436: f64 = (assign3690_e5435).ln();
        let assign3690_e5437: f64 = (locals.var_fn25_calc_iq__qref0 * assign3690_e5436);
        (assign3690_e5437, (locals.var_fn25_calc_iq__qref0 * ((assign3690_e5434 * locals.var_fn25_calc_iq__eta0_dn2) / assign3690_e5435)), ((locals.var_fn25_calc_iq__qref0_dn4 * assign3690_e5436) + (locals.var_fn25_calc_iq__qref0 * ((assign3690_e5434 * locals.var_fn25_calc_iq__eta0_dn4) / assign3690_e5435))), (locals.var_fn25_calc_iq__qref0 * ((assign3690_e5434 * locals.var_fn25_calc_iq__eta0_dn7) / assign3690_e5435)), (locals.var_fn25_calc_iq__qref0 * ((assign3690_e5434 * locals.var_fn25_calc_iq__eta0_dn16) / assign3690_e5435)), (locals.var_fn25_calc_iq__qref0 * ((assign3690_e5434 * locals.var_fn25_calc_iq__eta0_dn17) / assign3690_e5435)),)
    } else {
        (locals.var_fn25_calc_iq__qinvv0, locals.var_fn25_calc_iq__qinvv0_dn2, locals.var_fn25_calc_iq__qinvv0_dn4, locals.var_fn25_calc_iq__qinvv0_dn7, locals.var_fn25_calc_iq__qinvv0_dn16, locals.var_fn25_calc_iq__qinvv0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvv0 = assign3690_e5439;
        locals.var_fn25_calc_iq__qinvv0_dn2 = assign3690_e5439_d_n2;
        locals.var_fn25_calc_iq__qinvv0_dn4 = assign3690_e5439_d_n4;
        locals.var_fn25_calc_iq__qinvv0_dn7 = assign3690_e5439_d_n7;
        locals.var_fn25_calc_iq__qinvv0_dn16 = assign3690_e5439_d_n16;
        locals.var_fn25_calc_iq__qinvv0_dn17 = assign3690_e5439_d_n17;

        let (assign3700_e5445, assign3700_e5445_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3700_e5443: f64 = (locals.var_fn25_calc_iq__mu0 / locals.var_fn25_calc_iq__tfacmobin);
        (assign3700_e5443, (-((locals.var_fn25_calc_iq__mu0 * locals.var_fn25_calc_iq__tfacmobin_dn4) / (locals.var_fn25_calc_iq__tfacmobin * locals.var_fn25_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn25_calc_iq__muf0, locals.var_fn25_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn25_calc_iq__muf0 = assign3700_e5445;
        locals.var_fn25_calc_iq__muf0_dn4 = assign3700_e5445_d_n4;

        let (assign3710_e5461, assign3710_e5461_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3710_e5451: f64 = (locals.var_fn25_calc_iq__vzeta * locals.var_fn25_calc_iq__tnomin);
        let assign3710_e5452: f64 = (1.0 + assign3710_e5451);
        let assign3710_e5456: f64 = (locals.var_fn25_calc_iq__vzeta * locals.var_fn25_calc_iq__tambin);
        let assign3710_e5457: f64 = (1.0 + assign3710_e5456);
        let assign3710_e5458: f64 = (assign3710_e5452 / assign3710_e5457);
        let assign3710_e5459: f64 = (locals.var_fn25_calc_iq__vel0 * assign3710_e5458);
        (assign3710_e5459, (locals.var_fn25_calc_iq__vel0 * (-((assign3710_e5452 * (locals.var_fn25_calc_iq__vzeta * locals.var_fn25_calc_iq__tambin_dn4)) / (assign3710_e5457 * assign3710_e5457)))),)
    } else {
        (locals.var_fn25_calc_iq__vx0, locals.var_fn25_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn25_calc_iq__vx0 = assign3710_e5461;
        locals.var_fn25_calc_iq__vx0_dn4 = assign3710_e5461_d_n4;

        let (assign3720_e5469, assign3720_e5469_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3720_e5465: f64 = (locals.var_fn25_calc_iq__vx0 * locals.var_fn25_calc_iq__lin);
        let assign3720_e5467: f64 = (assign3720_e5465 / locals.var_fn25_calc_iq__muf0);
        (assign3720_e5467, ((((locals.var_fn25_calc_iq__vx0_dn4 * locals.var_fn25_calc_iq__lin) * locals.var_fn25_calc_iq__muf0) - (assign3720_e5465 * locals.var_fn25_calc_iq__muf0_dn4)) / (locals.var_fn25_calc_iq__muf0 * locals.var_fn25_calc_iq__muf0)),)
    } else {
        (locals.var_fn25_calc_iq__vdsats0, locals.var_fn25_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn25_calc_iq__vdsats0 = assign3720_e5469;
        locals.var_fn25_calc_iq__vdsats0_dn4 = assign3720_e5469_d_n4;

        let (assign3730_e5486, assign3730_e5486_d_n2, assign3730_e5486_d_n4, assign3730_e5486_d_n7, assign3730_e5486_d_n16, assign3730_e5486_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3730_e5475: f64 = (2.0 * locals.var_fn25_calc_iq__qinvv0);
        let assign3730_e5477: f64 = (assign3730_e5475 / locals.var_fn25_calc_iq__cgin);
        let assign3730_e5479: f64 = (assign3730_e5477 / locals.var_fn25_calc_iq__vdsats0);
        let assign3730_e5480: f64 = (1.0 + assign3730_e5479);
        let assign3730_e5481: f64 = (assign3730_e5480).sqrt();
        let assign3730_e5482: f64 = (locals.var_fn25_calc_iq__vdsats0 * assign3730_e5481);
        let assign3730_e5484: f64 = (assign3730_e5482 - locals.var_fn25_calc_iq__vdsats0);
        (assign3730_e5484, (locals.var_fn25_calc_iq__vdsats0 * ((((2.0 * locals.var_fn25_calc_iq__qinvv0_dn2) / locals.var_fn25_calc_iq__cgin) / locals.var_fn25_calc_iq__vdsats0) / (2.0 * assign3730_e5481))), (((locals.var_fn25_calc_iq__vdsats0_dn4 * assign3730_e5481) + (locals.var_fn25_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn25_calc_iq__qinvv0_dn4) * locals.var_fn25_calc_iq__cgin) - (assign3730_e5475 * locals.var_fn25_calc_iq__cgin_dn4)) / (locals.var_fn25_calc_iq__cgin * locals.var_fn25_calc_iq__cgin)) * locals.var_fn25_calc_iq__vdsats0) - (assign3730_e5477 * locals.var_fn25_calc_iq__vdsats0_dn4)) / (locals.var_fn25_calc_iq__vdsats0 * locals.var_fn25_calc_iq__vdsats0)) / (2.0 * assign3730_e5481)))) - locals.var_fn25_calc_iq__vdsats0_dn4), (locals.var_fn25_calc_iq__vdsats0 * ((((2.0 * locals.var_fn25_calc_iq__qinvv0_dn7) / locals.var_fn25_calc_iq__cgin) / locals.var_fn25_calc_iq__vdsats0) / (2.0 * assign3730_e5481))), (locals.var_fn25_calc_iq__vdsats0 * ((((2.0 * locals.var_fn25_calc_iq__qinvv0_dn16) / locals.var_fn25_calc_iq__cgin) / locals.var_fn25_calc_iq__vdsats0) / (2.0 * assign3730_e5481))), (locals.var_fn25_calc_iq__vdsats0 * ((((2.0 * locals.var_fn25_calc_iq__qinvv0_dn17) / locals.var_fn25_calc_iq__cgin) / locals.var_fn25_calc_iq__vdsats0) / (2.0 * assign3730_e5481))),)
    } else {
        (locals.var_fn25_calc_iq__vdsats10, locals.var_fn25_calc_iq__vdsats10_dn2, locals.var_fn25_calc_iq__vdsats10_dn4, locals.var_fn25_calc_iq__vdsats10_dn7, locals.var_fn25_calc_iq__vdsats10_dn16, locals.var_fn25_calc_iq__vdsats10_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsats10 = assign3730_e5486;
        locals.var_fn25_calc_iq__vdsats10_dn2 = assign3730_e5486_d_n2;
        locals.var_fn25_calc_iq__vdsats10_dn4 = assign3730_e5486_d_n4;
        locals.var_fn25_calc_iq__vdsats10_dn7 = assign3730_e5486_d_n7;
        locals.var_fn25_calc_iq__vdsats10_dn16 = assign3730_e5486_d_n16;
        locals.var_fn25_calc_iq__vdsats10_dn17 = assign3730_e5486_d_n17;

        let (assign3740_e5498, assign3740_e5498_d_n2, assign3740_e5498_d_n4, assign3740_e5498_d_n7, assign3740_e5498_d_n16, assign3740_e5498_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3740_e5491: f64 = (1.0 - locals.var_fn25_calc_iq__ff0);
        let assign3740_e5492: f64 = (locals.var_fn25_calc_iq__vdsats10 * assign3740_e5491);
        let assign3740_e5495: f64 = (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__ff0);
        let assign3740_e5496: f64 = (assign3740_e5492 + assign3740_e5495);
        (assign3740_e5496, (((locals.var_fn25_calc_iq__vdsats10_dn2 * assign3740_e5491) + (locals.var_fn25_calc_iq__vdsats10 * (-locals.var_fn25_calc_iq__ff0_dn2))) + (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__ff0_dn2)), (((locals.var_fn25_calc_iq__vdsats10_dn4 * assign3740_e5491) + (locals.var_fn25_calc_iq__vdsats10 * (-locals.var_fn25_calc_iq__ff0_dn4))) + ((locals.var_fn25_calc_iq__two_n_phit0_dn4 * locals.var_fn25_calc_iq__ff0) + (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__ff0_dn4))), (((locals.var_fn25_calc_iq__vdsats10_dn7 * assign3740_e5491) + (locals.var_fn25_calc_iq__vdsats10 * (-locals.var_fn25_calc_iq__ff0_dn7))) + (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__ff0_dn7)), (((locals.var_fn25_calc_iq__vdsats10_dn16 * assign3740_e5491) + (locals.var_fn25_calc_iq__vdsats10 * (-locals.var_fn25_calc_iq__ff0_dn16))) + (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__ff0_dn16)), (((locals.var_fn25_calc_iq__vdsats10_dn17 * assign3740_e5491) + (locals.var_fn25_calc_iq__vdsats10 * (-locals.var_fn25_calc_iq__ff0_dn17))) + (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__ff0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__vdsat10, locals.var_fn25_calc_iq__vdsat10_dn2, locals.var_fn25_calc_iq__vdsat10_dn4, locals.var_fn25_calc_iq__vdsat10_dn7, locals.var_fn25_calc_iq__vdsat10_dn16, locals.var_fn25_calc_iq__vdsat10_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdsat10 = assign3740_e5498;
        locals.var_fn25_calc_iq__vdsat10_dn2 = assign3740_e5498_d_n2;
        locals.var_fn25_calc_iq__vdsat10_dn4 = assign3740_e5498_d_n4;
        locals.var_fn25_calc_iq__vdsat10_dn7 = assign3740_e5498_d_n7;
        locals.var_fn25_calc_iq__vdsat10_dn16 = assign3740_e5498_d_n16;
        locals.var_fn25_calc_iq__vdsat10_dn17 = assign3740_e5498_d_n17;

        let (assign3750_e5567, assign3750_e5567_d_n2, assign3750_e5567_d_n4, assign3750_e5567_d_n7, assign3750_e5567_d_n16, assign3750_e5567_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3750_e5557, assign3750_e5557_d_n2, assign3750_e5557_d_n4, assign3750_e5557_d_n7, assign3750_e5557_d_n16, assign3750_e5557_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3750_e5510: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat10);
                let assign3750_e5511: f64 = assign3750_e5510;
                let assign3750_e5515: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat10);
                let assign3750_e5516: f64 = (-assign3750_e5515);
                let assign3750_e5519: f64 = (0.001 / p.p53);
                let assign3750_e5523: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat10);
                let assign3750_e5524: f64 = (-assign3750_e5523);
                let assign3750_e5525: f64 = (assign3750_e5519 * assign3750_e5524);
                let assign3750_e5526: f64 = (assign3750_e5525).tanh();
                let assign3750_e5527: f64 = (assign3750_e5516 * assign3750_e5526);
                let assign3750_e5528: f64 = (assign3750_e5511 + assign3750_e5527);
                let assign3750_e5529: f64 = (0.5 * assign3750_e5528);
                (assign3750_e5529, (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + (((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + (((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + (((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + (((-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + (((-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))),)
            } else {
                let (assign3750_e5556, assign3750_e5556_d_n2, assign3750_e5556_d_n4, assign3750_e5556_d_n7, assign3750_e5556_d_n16, assign3750_e5556_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3750_e5537: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat10);
                        let assign3750_e5538: f64 = assign3750_e5537;
                        let assign3750_e5542: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat10);
                        let assign3750_e5543: f64 = (-assign3750_e5542);
                        let assign3750_e5547: f64 = (locals.var_fn25_calc_iq__vdsin / locals.var_fn25_calc_iq__vdsat10);
                        let assign3750_e5548: f64 = (-assign3750_e5547);
                        let assign3750_e5549: f64 = (assign3750_e5543 * assign3750_e5548);
                        let assign3750_e5551: f64 = (assign3750_e5549 + p.p53);
                        let assign3750_e5552: f64 = (assign3750_e5551).sqrt();
                        let assign3750_e5553: f64 = (assign3750_e5538 + assign3750_e5552);
                        let assign3750_e5554: f64 = (0.5 * assign3750_e5553);
                        (assign3750_e5554, (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + ((((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3750_e5548) + (assign3750_e5543 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3750_e5552)))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + ((((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3750_e5548) + (assign3750_e5543 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3750_e5552)))), (0.5 * ((-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + ((((-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3750_e5548) + (assign3750_e5543 * (-(-((locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3750_e5552)))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + ((((-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3750_e5548) + (assign3750_e5543 * (-(((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / (2.0 * assign3750_e5552)))), (0.5 * ((((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + ((((-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3750_e5548) + (assign3750_e5543 * (-(((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__vdsat10) - (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / (2.0 * assign3750_e5552)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3750_e5556, assign3750_e5556_d_n2, assign3750_e5556_d_n4, assign3750_e5556_d_n7, assign3750_e5556_d_n16, assign3750_e5556_d_n17,)
            }
        };
        let assign3750_e5559: f64 = (assign3750_e5557).powf(locals.var_fn25_calc_iq__beta);
        let assign3750_e5560: f64 = (1.0 + assign3750_e5559);
        let assign3750_e5563: f64 = (1.0 / locals.var_fn25_calc_iq__beta);
        let assign3750_e5564: f64 = (assign3750_e5560).powf(assign3750_e5563);
        let assign3750_e5565: f64 = (1.0 / assign3750_e5564);
        (assign3750_e5565, (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n2)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n2 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n2)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n2 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))), (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n4)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n4 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n4)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n4 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))), (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n7)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n7 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n7)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n7 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))), (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n16)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n16 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n16)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n16 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))), (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n17)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n17 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3750_e5557).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n17)) } } else { (assign3750_e5559 * (locals.var_fn25_calc_iq__beta * (assign3750_e5557_d_n17 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))),)
    } else {
        (locals.var_fn25_calc_iq__fsd0, locals.var_fn25_calc_iq__fsd0_dn2, locals.var_fn25_calc_iq__fsd0_dn4, locals.var_fn25_calc_iq__fsd0_dn7, locals.var_fn25_calc_iq__fsd0_dn16, locals.var_fn25_calc_iq__fsd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__fsd0 = assign3750_e5567;
        locals.var_fn25_calc_iq__fsd0_dn2 = assign3750_e5567_d_n2;
        locals.var_fn25_calc_iq__fsd0_dn4 = assign3750_e5567_d_n4;
        locals.var_fn25_calc_iq__fsd0_dn7 = assign3750_e5567_d_n7;
        locals.var_fn25_calc_iq__fsd0_dn16 = assign3750_e5567_d_n16;
        locals.var_fn25_calc_iq__fsd0_dn17 = assign3750_e5567_d_n17;

        let (assign3760_e5573, assign3760_e5573_d_n2, assign3760_e5573_d_n4, assign3760_e5573_d_n7, assign3760_e5573_d_n16, assign3760_e5573_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3760_e5571: f64 = (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd0);
        (assign3760_e5571, (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd0_dn2), (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd0_dn4), (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd0_dn7), ((locals.var_fn25_calc_iq__vdsin_dn16 * locals.var_fn25_calc_iq__fsd0) + (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd0_dn16)), ((locals.var_fn25_calc_iq__vdsin_dn17 * locals.var_fn25_calc_iq__fsd0) + (locals.var_fn25_calc_iq__vdsin * locals.var_fn25_calc_iq__fsd0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__vdx0, locals.var_fn25_calc_iq__vdx0_dn2, locals.var_fn25_calc_iq__vdx0_dn4, locals.var_fn25_calc_iq__vdx0_dn7, locals.var_fn25_calc_iq__vdx0_dn16, locals.var_fn25_calc_iq__vdx0_dn17,)
    }
};
        locals.var_fn25_calc_iq__vdx0 = assign3760_e5573;
        locals.var_fn25_calc_iq__vdx0_dn2 = assign3760_e5573_d_n2;
        locals.var_fn25_calc_iq__vdx0_dn4 = assign3760_e5573_d_n4;
        locals.var_fn25_calc_iq__vdx0_dn7 = assign3760_e5573_d_n7;
        locals.var_fn25_calc_iq__vdx0_dn16 = assign3760_e5573_d_n16;
        locals.var_fn25_calc_iq__vdx0_dn17 = assign3760_e5573_d_n17;

        let (assign3770_e5648, assign3770_e5648_d_n2, assign3770_e5648_d_n4, assign3770_e5648_d_n7, assign3770_e5648_d_n16, assign3770_e5648_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let (assign3770_e5638, assign3770_e5638_d_n2, assign3770_e5638_d_n4, assign3770_e5638_d_n7, assign3770_e5638_d_n16, assign3770_e5638_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3770_e5584: f64 = (-locals.var_fn25_calc_iq__vdsin);
                let assign3770_e5586: f64 = (assign3770_e5584 / locals.var_fn25_calc_iq__vdsat10);
                let assign3770_e5587: f64 = assign3770_e5586;
                let assign3770_e5590: f64 = (-locals.var_fn25_calc_iq__vdsin);
                let assign3770_e5592: f64 = (assign3770_e5590 / locals.var_fn25_calc_iq__vdsat10);
                let assign3770_e5593: f64 = (-assign3770_e5592);
                let assign3770_e5596: f64 = (0.001 / p.p53);
                let assign3770_e5599: f64 = (-locals.var_fn25_calc_iq__vdsin);
                let assign3770_e5601: f64 = (assign3770_e5599 / locals.var_fn25_calc_iq__vdsat10);
                let assign3770_e5602: f64 = (-assign3770_e5601);
                let assign3770_e5603: f64 = (assign3770_e5596 * assign3770_e5602);
                let assign3770_e5604: f64 = (assign3770_e5603).tanh();
                let assign3770_e5605: f64 = (assign3770_e5593 * assign3770_e5604);
                let assign3770_e5606: f64 = (assign3770_e5587 + assign3770_e5605);
                let assign3770_e5607: f64 = (0.5 * assign3770_e5606);
                (assign3770_e5607, (0.5 * ((-((assign3770_e5584 * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + (((-(-((assign3770_e5590 * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-(-((assign3770_e5599 * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))), (0.5 * ((-((assign3770_e5584 * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + (((-(-((assign3770_e5590 * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-(-((assign3770_e5599 * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))), (0.5 * ((-((assign3770_e5584 * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + (((-(-((assign3770_e5590 * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-(-((assign3770_e5599 * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5584 * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + (((-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5590 * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5599 * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5584 * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + (((-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5590 * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5599 * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))),)
            } else {
                let (assign3770_e5637, assign3770_e5637_d_n2, assign3770_e5637_d_n4, assign3770_e5637_d_n7, assign3770_e5637_d_n16, assign3770_e5637_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3770_e5614: f64 = (-locals.var_fn25_calc_iq__vdsin);
                        let assign3770_e5616: f64 = (assign3770_e5614 / locals.var_fn25_calc_iq__vdsat10);
                        let assign3770_e5617: f64 = assign3770_e5616;
                        let assign3770_e5620: f64 = (-locals.var_fn25_calc_iq__vdsin);
                        let assign3770_e5622: f64 = (assign3770_e5620 / locals.var_fn25_calc_iq__vdsat10);
                        let assign3770_e5623: f64 = (-assign3770_e5622);
                        let assign3770_e5626: f64 = (-locals.var_fn25_calc_iq__vdsin);
                        let assign3770_e5628: f64 = (assign3770_e5626 / locals.var_fn25_calc_iq__vdsat10);
                        let assign3770_e5629: f64 = (-assign3770_e5628);
                        let assign3770_e5630: f64 = (assign3770_e5623 * assign3770_e5629);
                        let assign3770_e5632: f64 = (assign3770_e5630 + p.p53);
                        let assign3770_e5633: f64 = (assign3770_e5632).sqrt();
                        let assign3770_e5634: f64 = (assign3770_e5617 + assign3770_e5633);
                        let assign3770_e5635: f64 = (0.5 * assign3770_e5634);
                        (assign3770_e5635, (0.5 * ((-((assign3770_e5614 * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + ((((-(-((assign3770_e5620 * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3770_e5629) + (assign3770_e5623 * (-(-((assign3770_e5626 * locals.var_fn25_calc_iq__vdsat10_dn2) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3770_e5633)))), (0.5 * ((-((assign3770_e5614 * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + ((((-(-((assign3770_e5620 * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3770_e5629) + (assign3770_e5623 * (-(-((assign3770_e5626 * locals.var_fn25_calc_iq__vdsat10_dn4) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3770_e5633)))), (0.5 * ((-((assign3770_e5614 * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) + ((((-(-((assign3770_e5620 * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))) * assign3770_e5629) + (assign3770_e5623 * (-(-((assign3770_e5626 * locals.var_fn25_calc_iq__vdsat10_dn7) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3770_e5633)))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5614 * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + ((((-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5620 * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3770_e5629) + (assign3770_e5623 * (-((((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5626 * locals.var_fn25_calc_iq__vdsat10_dn16)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / (2.0 * assign3770_e5633)))), (0.5 * (((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5614 * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10)) + ((((-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5620 * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))) * assign3770_e5629) + (assign3770_e5623 * (-((((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__vdsat10) - (assign3770_e5626 * locals.var_fn25_calc_iq__vdsat10_dn17)) / (locals.var_fn25_calc_iq__vdsat10 * locals.var_fn25_calc_iq__vdsat10))))) / (2.0 * assign3770_e5633)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3770_e5637, assign3770_e5637_d_n2, assign3770_e5637_d_n4, assign3770_e5637_d_n7, assign3770_e5637_d_n16, assign3770_e5637_d_n17,)
            }
        };
        let assign3770_e5640: f64 = (assign3770_e5638).powf(locals.var_fn25_calc_iq__beta);
        let assign3770_e5641: f64 = (1.0 + assign3770_e5640);
        let assign3770_e5644: f64 = (1.0 / locals.var_fn25_calc_iq__beta);
        let assign3770_e5645: f64 = (assign3770_e5641).powf(assign3770_e5644);
        let assign3770_e5646: f64 = (1.0 / assign3770_e5645);
        (assign3770_e5646, (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n2)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n2 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n2)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n2 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))), (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n4)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n4 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n4)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n4 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))), (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n7)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n7 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n7)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n7 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))), (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n16)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n16 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n16)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n16 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))), (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n17)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n17 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((locals.var_fn25_calc_iq__beta) as f64).is_finite() && ((locals.var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn25_calc_iq__beta * ((assign3770_e5638).powf(locals.var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n17)) } } else { (assign3770_e5640 * (locals.var_fn25_calc_iq__beta * (assign3770_e5638_d_n17 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))),)
    } else {
        (locals.var_fn25_calc_iq__fds0, locals.var_fn25_calc_iq__fds0_dn2, locals.var_fn25_calc_iq__fds0_dn4, locals.var_fn25_calc_iq__fds0_dn7, locals.var_fn25_calc_iq__fds0_dn16, locals.var_fn25_calc_iq__fds0_dn17,)
    }
};
        locals.var_fn25_calc_iq__fds0 = assign3770_e5648;
        locals.var_fn25_calc_iq__fds0_dn2 = assign3770_e5648_d_n2;
        locals.var_fn25_calc_iq__fds0_dn4 = assign3770_e5648_d_n4;
        locals.var_fn25_calc_iq__fds0_dn7 = assign3770_e5648_d_n7;
        locals.var_fn25_calc_iq__fds0_dn16 = assign3770_e5648_d_n16;
        locals.var_fn25_calc_iq__fds0_dn17 = assign3770_e5648_d_n17;

        let (assign3780_e5655, assign3780_e5655_d_n2, assign3780_e5655_d_n4, assign3780_e5655_d_n7, assign3780_e5655_d_n16, assign3780_e5655_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3780_e5651: f64 = (-locals.var_fn25_calc_iq__vdsin);
        let assign3780_e5653: f64 = (assign3780_e5651 * locals.var_fn25_calc_iq__fds0);
        (assign3780_e5653, (assign3780_e5651 * locals.var_fn25_calc_iq__fds0_dn2), (assign3780_e5651 * locals.var_fn25_calc_iq__fds0_dn4), (assign3780_e5651 * locals.var_fn25_calc_iq__fds0_dn7), (((-locals.var_fn25_calc_iq__vdsin_dn16) * locals.var_fn25_calc_iq__fds0) + (assign3780_e5651 * locals.var_fn25_calc_iq__fds0_dn16)), (((-locals.var_fn25_calc_iq__vdsin_dn17) * locals.var_fn25_calc_iq__fds0) + (assign3780_e5651 * locals.var_fn25_calc_iq__fds0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__vsx0, locals.var_fn25_calc_iq__vsx0_dn2, locals.var_fn25_calc_iq__vsx0_dn4, locals.var_fn25_calc_iq__vsx0_dn7, locals.var_fn25_calc_iq__vsx0_dn16, locals.var_fn25_calc_iq__vsx0_dn17,)
    }
};
        locals.var_fn25_calc_iq__vsx0 = assign3780_e5655;
        locals.var_fn25_calc_iq__vsx0_dn2 = assign3780_e5655_d_n2;
        locals.var_fn25_calc_iq__vsx0_dn4 = assign3780_e5655_d_n4;
        locals.var_fn25_calc_iq__vsx0_dn7 = assign3780_e5655_d_n7;
        locals.var_fn25_calc_iq__vsx0_dn16 = assign3780_e5655_d_n16;
        locals.var_fn25_calc_iq__vsx0_dn17 = assign3780_e5655_d_n17;

        let (assign3790_e5663, assign3790_e5663_d_n2, assign3790_e5663_d_n4, assign3790_e5663_d_n7, assign3790_e5663_d_n16, assign3790_e5663_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3790_e5659: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__myarg0);
        let assign3790_e5661: f64 = (assign3790_e5659 / locals.var_fn25_calc_iq__alpha_phit);
        (assign3790_e5661, (locals.var_fn25_calc_iq__vgsin_dn2 / locals.var_fn25_calc_iq__alpha_phit), ((((-locals.var_fn25_calc_iq__myarg0_dn4) * locals.var_fn25_calc_iq__alpha_phit) - (assign3790_e5659 * locals.var_fn25_calc_iq__alpha_phit_dn4)) / (locals.var_fn25_calc_iq__alpha_phit * locals.var_fn25_calc_iq__alpha_phit)), (locals.var_fn25_calc_iq__vgsin_dn7 / locals.var_fn25_calc_iq__alpha_phit), (locals.var_fn25_calc_iq__vgsin_dn16 / locals.var_fn25_calc_iq__alpha_phit), 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg0, locals.var_fn25_calc_iq__exparg0_dn2, locals.var_fn25_calc_iq__exparg0_dn4, locals.var_fn25_calc_iq__exparg0_dn7, locals.var_fn25_calc_iq__exparg0_dn16, locals.var_fn25_calc_iq__exparg0_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg0 = assign3790_e5663;
        locals.var_fn25_calc_iq__exparg0_dn2 = assign3790_e5663_d_n2;
        locals.var_fn25_calc_iq__exparg0_dn4 = assign3790_e5663_d_n4;
        locals.var_fn25_calc_iq__exparg0_dn7 = assign3790_e5663_d_n7;
        locals.var_fn25_calc_iq__exparg0_dn16 = assign3790_e5663_d_n16;
        locals.var_fn25_calc_iq__exparg0_dn17 = assign3790_e5663_d_n17;

        let assign3800_e5666: f64 = if locals.var_fn25_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign3800_e5666;

        let (assign3810_e5672, assign3810_e5672_d_n2, assign3810_e5672_d_n4, assign3810_e5672_d_n7, assign3810_e5672_d_n16, assign3810_e5672_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard43 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffs0, locals.var_fn25_calc_iq__ffs0_dn2, locals.var_fn25_calc_iq__ffs0_dn4, locals.var_fn25_calc_iq__ffs0_dn7, locals.var_fn25_calc_iq__ffs0_dn16, locals.var_fn25_calc_iq__ffs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs0 = assign3810_e5672;
        locals.var_fn25_calc_iq__ffs0_dn2 = assign3810_e5672_d_n2;
        locals.var_fn25_calc_iq__ffs0_dn4 = assign3810_e5672_d_n4;
        locals.var_fn25_calc_iq__ffs0_dn7 = assign3810_e5672_d_n7;
        locals.var_fn25_calc_iq__ffs0_dn16 = assign3810_e5672_d_n16;
        locals.var_fn25_calc_iq__ffs0_dn17 = assign3810_e5672_d_n17;

        let assign3820_e5675: f64 = (-50.0);
        let assign3820_e5676: f64 = if locals.var_fn25_calc_iq__exparg0 < assign3820_e5675 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign3820_e5676;

        let (assign3830_e5685, assign3830_e5685_d_n2, assign3830_e5685_d_n4, assign3830_e5685_d_n7, assign3830_e5685_d_n16, assign3830_e5685_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard43 == 0.0)) && (locals.var_guard44 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffs0, locals.var_fn25_calc_iq__ffs0_dn2, locals.var_fn25_calc_iq__ffs0_dn4, locals.var_fn25_calc_iq__ffs0_dn7, locals.var_fn25_calc_iq__ffs0_dn16, locals.var_fn25_calc_iq__ffs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs0 = assign3830_e5685;
        locals.var_fn25_calc_iq__ffs0_dn2 = assign3830_e5685_d_n2;
        locals.var_fn25_calc_iq__ffs0_dn4 = assign3830_e5685_d_n4;
        locals.var_fn25_calc_iq__ffs0_dn7 = assign3830_e5685_d_n7;
        locals.var_fn25_calc_iq__ffs0_dn16 = assign3830_e5685_d_n16;
        locals.var_fn25_calc_iq__ffs0_dn17 = assign3830_e5685_d_n17;

        let (assign3840_e5700, assign3840_e5700_d_n2, assign3840_e5700_d_n4, assign3840_e5700_d_n7, assign3840_e5700_d_n16, assign3840_e5700_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard43 == 0.0)) && (locals.var_guard44 == 0.0)) {
        let assign3840_e5696: f64 = (locals.var_fn25_calc_iq__exparg0).exp();
        let assign3840_e5697: f64 = (1.0 + assign3840_e5696);
        let assign3840_e5698: f64 = (1.0 / assign3840_e5697);
        (assign3840_e5698, (-((assign3840_e5696 * locals.var_fn25_calc_iq__exparg0_dn2) / (assign3840_e5697 * assign3840_e5697))), (-((assign3840_e5696 * locals.var_fn25_calc_iq__exparg0_dn4) / (assign3840_e5697 * assign3840_e5697))), (-((assign3840_e5696 * locals.var_fn25_calc_iq__exparg0_dn7) / (assign3840_e5697 * assign3840_e5697))), (-((assign3840_e5696 * locals.var_fn25_calc_iq__exparg0_dn16) / (assign3840_e5697 * assign3840_e5697))), (-((assign3840_e5696 * locals.var_fn25_calc_iq__exparg0_dn17) / (assign3840_e5697 * assign3840_e5697))),)
    } else {
        (locals.var_fn25_calc_iq__ffs0, locals.var_fn25_calc_iq__ffs0_dn2, locals.var_fn25_calc_iq__ffs0_dn4, locals.var_fn25_calc_iq__ffs0_dn7, locals.var_fn25_calc_iq__ffs0_dn16, locals.var_fn25_calc_iq__ffs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffs0 = assign3840_e5700;
        locals.var_fn25_calc_iq__ffs0_dn2 = assign3840_e5700_d_n2;
        locals.var_fn25_calc_iq__ffs0_dn4 = assign3840_e5700_d_n4;
        locals.var_fn25_calc_iq__ffs0_dn7 = assign3840_e5700_d_n7;
        locals.var_fn25_calc_iq__ffs0_dn16 = assign3840_e5700_d_n16;
        locals.var_fn25_calc_iq__ffs0_dn17 = assign3840_e5700_d_n17;

        let (assign3850_e5718, assign3850_e5718_d_n2, assign3850_e5718_d_n4, assign3850_e5718_d_n7, assign3850_e5718_d_n16, assign3850_e5718_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3850_e5704: f64 = (locals.var_fn25_calc_iq__vgdin - locals.var_fn25_calc_iq__vsx0);
        let assign3850_e5708: f64 = (p.p51 * 0.1);
        let assign3850_e5710: f64 = (assign3850_e5708 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3850_e5712: f64 = (assign3850_e5710 * locals.var_fn25_calc_iq__ffs0);
        let assign3850_e5713: f64 = (locals.var_fn25_calc_iq__vtof - assign3850_e5712);
        let assign3850_e5714: f64 = (assign3850_e5704 - assign3850_e5713);
        let assign3850_e5716: f64 = (assign3850_e5714 / locals.var_fn25_calc_iq__two_n_phit0);
        (assign3850_e5716, (((locals.var_fn25_calc_iq__vgdin_dn2 - locals.var_fn25_calc_iq__vsx0_dn2) - (-(assign3850_e5710 * locals.var_fn25_calc_iq__ffs0_dn2))) / locals.var_fn25_calc_iq__two_n_phit0), (((((-locals.var_fn25_calc_iq__vsx0_dn4) - (locals.var_fn25_calc_iq__vtof_dn4 - (((assign3850_e5708 * locals.var_fn25_calc_iq__alpha_phit_dn4) * locals.var_fn25_calc_iq__ffs0) + (assign3850_e5710 * locals.var_fn25_calc_iq__ffs0_dn4)))) * locals.var_fn25_calc_iq__two_n_phit0) - (assign3850_e5714 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) / (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__two_n_phit0)), (((locals.var_fn25_calc_iq__vgdin_dn7 - locals.var_fn25_calc_iq__vsx0_dn7) - (-(assign3850_e5710 * locals.var_fn25_calc_iq__ffs0_dn7))) / locals.var_fn25_calc_iq__two_n_phit0), (((locals.var_fn25_calc_iq__vgdin_dn16 - locals.var_fn25_calc_iq__vsx0_dn16) - (-(assign3850_e5710 * locals.var_fn25_calc_iq__ffs0_dn16))) / locals.var_fn25_calc_iq__two_n_phit0), (((locals.var_fn25_calc_iq__vgdin_dn17 - locals.var_fn25_calc_iq__vsx0_dn17) - (-(assign3850_e5710 * locals.var_fn25_calc_iq__ffs0_dn17))) / locals.var_fn25_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn25_calc_iq__etas0, locals.var_fn25_calc_iq__etas0_dn2, locals.var_fn25_calc_iq__etas0_dn4, locals.var_fn25_calc_iq__etas0_dn7, locals.var_fn25_calc_iq__etas0_dn16, locals.var_fn25_calc_iq__etas0_dn17,)
    }
};
        locals.var_fn25_calc_iq__etas0 = assign3850_e5718;
        locals.var_fn25_calc_iq__etas0_dn2 = assign3850_e5718_d_n2;
        locals.var_fn25_calc_iq__etas0_dn4 = assign3850_e5718_d_n4;
        locals.var_fn25_calc_iq__etas0_dn7 = assign3850_e5718_d_n7;
        locals.var_fn25_calc_iq__etas0_dn16 = assign3850_e5718_d_n16;
        locals.var_fn25_calc_iq__etas0_dn17 = assign3850_e5718_d_n17;

        let assign3860_e5721: f64 = if locals.var_fn25_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign3860_e5721;

        let (assign3870_e5729, assign3870_e5729_d_n2, assign3870_e5729_d_n4, assign3870_e5729_d_n7, assign3870_e5729_d_n16, assign3870_e5729_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard45 != 0.0)) {
        let assign3870_e5727: f64 = (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etas0);
        (assign3870_e5727, (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etas0_dn2), ((locals.var_fn25_calc_iq__qref0_dn4 * locals.var_fn25_calc_iq__etas0) + (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etas0_dn4)), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etas0_dn7), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etas0_dn16), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etas0_dn17),)
    } else {
        (locals.var_fn25_calc_iq__qinvs0, locals.var_fn25_calc_iq__qinvs0_dn2, locals.var_fn25_calc_iq__qinvs0_dn4, locals.var_fn25_calc_iq__qinvs0_dn7, locals.var_fn25_calc_iq__qinvs0_dn16, locals.var_fn25_calc_iq__qinvs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs0 = assign3870_e5729;
        locals.var_fn25_calc_iq__qinvs0_dn2 = assign3870_e5729_d_n2;
        locals.var_fn25_calc_iq__qinvs0_dn4 = assign3870_e5729_d_n4;
        locals.var_fn25_calc_iq__qinvs0_dn7 = assign3870_e5729_d_n7;
        locals.var_fn25_calc_iq__qinvs0_dn16 = assign3870_e5729_d_n16;
        locals.var_fn25_calc_iq__qinvs0_dn17 = assign3870_e5729_d_n17;

        let assign3880_e5732: f64 = (-50.0);
        let assign3880_e5733: f64 = if locals.var_fn25_calc_iq__etas0 < assign3880_e5732 { 1.0 } else { 0.0 };
        locals.var_guard46 = assign3880_e5733;

        let (assign3890_e5745, assign3890_e5745_d_n2, assign3890_e5745_d_n4, assign3890_e5745_d_n7, assign3890_e5745_d_n16, assign3890_e5745_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard45 == 0.0)) && (locals.var_guard46 != 0.0)) {
        let assign3890_e5742: f64 = (locals.var_fn25_calc_iq__etas0).exp();
        let assign3890_e5743: f64 = (locals.var_fn25_calc_iq__qref0 * assign3890_e5742);
        (assign3890_e5743, (locals.var_fn25_calc_iq__qref0 * (assign3890_e5742 * locals.var_fn25_calc_iq__etas0_dn2)), ((locals.var_fn25_calc_iq__qref0_dn4 * assign3890_e5742) + (locals.var_fn25_calc_iq__qref0 * (assign3890_e5742 * locals.var_fn25_calc_iq__etas0_dn4))), (locals.var_fn25_calc_iq__qref0 * (assign3890_e5742 * locals.var_fn25_calc_iq__etas0_dn7)), (locals.var_fn25_calc_iq__qref0 * (assign3890_e5742 * locals.var_fn25_calc_iq__etas0_dn16)), (locals.var_fn25_calc_iq__qref0 * (assign3890_e5742 * locals.var_fn25_calc_iq__etas0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qinvs0, locals.var_fn25_calc_iq__qinvs0_dn2, locals.var_fn25_calc_iq__qinvs0_dn4, locals.var_fn25_calc_iq__qinvs0_dn7, locals.var_fn25_calc_iq__qinvs0_dn16, locals.var_fn25_calc_iq__qinvs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs0 = assign3890_e5745;
        locals.var_fn25_calc_iq__qinvs0_dn2 = assign3890_e5745_d_n2;
        locals.var_fn25_calc_iq__qinvs0_dn4 = assign3890_e5745_d_n4;
        locals.var_fn25_calc_iq__qinvs0_dn7 = assign3890_e5745_d_n7;
        locals.var_fn25_calc_iq__qinvs0_dn16 = assign3890_e5745_d_n16;
        locals.var_fn25_calc_iq__qinvs0_dn17 = assign3890_e5745_d_n17;

        let (assign3900_e5761, assign3900_e5761_d_n2, assign3900_e5761_d_n4, assign3900_e5761_d_n7, assign3900_e5761_d_n16, assign3900_e5761_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard45 == 0.0)) && (locals.var_guard46 == 0.0)) {
        let assign3900_e5756: f64 = (locals.var_fn25_calc_iq__etas0).exp();
        let assign3900_e5757: f64 = (1.0 + assign3900_e5756);
        let assign3900_e5758: f64 = (assign3900_e5757).ln();
        let assign3900_e5759: f64 = (locals.var_fn25_calc_iq__qref0 * assign3900_e5758);
        (assign3900_e5759, (locals.var_fn25_calc_iq__qref0 * ((assign3900_e5756 * locals.var_fn25_calc_iq__etas0_dn2) / assign3900_e5757)), ((locals.var_fn25_calc_iq__qref0_dn4 * assign3900_e5758) + (locals.var_fn25_calc_iq__qref0 * ((assign3900_e5756 * locals.var_fn25_calc_iq__etas0_dn4) / assign3900_e5757))), (locals.var_fn25_calc_iq__qref0 * ((assign3900_e5756 * locals.var_fn25_calc_iq__etas0_dn7) / assign3900_e5757)), (locals.var_fn25_calc_iq__qref0 * ((assign3900_e5756 * locals.var_fn25_calc_iq__etas0_dn16) / assign3900_e5757)), (locals.var_fn25_calc_iq__qref0 * ((assign3900_e5756 * locals.var_fn25_calc_iq__etas0_dn17) / assign3900_e5757)),)
    } else {
        (locals.var_fn25_calc_iq__qinvs0, locals.var_fn25_calc_iq__qinvs0_dn2, locals.var_fn25_calc_iq__qinvs0_dn4, locals.var_fn25_calc_iq__qinvs0_dn7, locals.var_fn25_calc_iq__qinvs0_dn16, locals.var_fn25_calc_iq__qinvs0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvs0 = assign3900_e5761;
        locals.var_fn25_calc_iq__qinvs0_dn2 = assign3900_e5761_d_n2;
        locals.var_fn25_calc_iq__qinvs0_dn4 = assign3900_e5761_d_n4;
        locals.var_fn25_calc_iq__qinvs0_dn7 = assign3900_e5761_d_n7;
        locals.var_fn25_calc_iq__qinvs0_dn16 = assign3900_e5761_d_n16;
        locals.var_fn25_calc_iq__qinvs0_dn17 = assign3900_e5761_d_n17;

        let (assign3910_e5769, assign3910_e5769_d_n2, assign3910_e5769_d_n4, assign3910_e5769_d_n7, assign3910_e5769_d_n16, assign3910_e5769_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3910_e5765: f64 = (locals.var_fn25_calc_iq__vgdin - locals.var_fn25_calc_iq__myarg0);
        let assign3910_e5767: f64 = (assign3910_e5765 / locals.var_fn25_calc_iq__alpha_phit);
        (assign3910_e5767, (locals.var_fn25_calc_iq__vgdin_dn2 / locals.var_fn25_calc_iq__alpha_phit), ((((-locals.var_fn25_calc_iq__myarg0_dn4) * locals.var_fn25_calc_iq__alpha_phit) - (assign3910_e5765 * locals.var_fn25_calc_iq__alpha_phit_dn4)) / (locals.var_fn25_calc_iq__alpha_phit * locals.var_fn25_calc_iq__alpha_phit)), (locals.var_fn25_calc_iq__vgdin_dn7 / locals.var_fn25_calc_iq__alpha_phit), (locals.var_fn25_calc_iq__vgdin_dn16 / locals.var_fn25_calc_iq__alpha_phit), (locals.var_fn25_calc_iq__vgdin_dn17 / locals.var_fn25_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn25_calc_iq__exparg0, locals.var_fn25_calc_iq__exparg0_dn2, locals.var_fn25_calc_iq__exparg0_dn4, locals.var_fn25_calc_iq__exparg0_dn7, locals.var_fn25_calc_iq__exparg0_dn16, locals.var_fn25_calc_iq__exparg0_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg0 = assign3910_e5769;
        locals.var_fn25_calc_iq__exparg0_dn2 = assign3910_e5769_d_n2;
        locals.var_fn25_calc_iq__exparg0_dn4 = assign3910_e5769_d_n4;
        locals.var_fn25_calc_iq__exparg0_dn7 = assign3910_e5769_d_n7;
        locals.var_fn25_calc_iq__exparg0_dn16 = assign3910_e5769_d_n16;
        locals.var_fn25_calc_iq__exparg0_dn17 = assign3910_e5769_d_n17;

        let assign3920_e5772: f64 = if locals.var_fn25_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard47 = assign3920_e5772;

        let (assign3930_e5778, assign3930_e5778_d_n2, assign3930_e5778_d_n4, assign3930_e5778_d_n7, assign3930_e5778_d_n16, assign3930_e5778_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard47 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffd0, locals.var_fn25_calc_iq__ffd0_dn2, locals.var_fn25_calc_iq__ffd0_dn4, locals.var_fn25_calc_iq__ffd0_dn7, locals.var_fn25_calc_iq__ffd0_dn16, locals.var_fn25_calc_iq__ffd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd0 = assign3930_e5778;
        locals.var_fn25_calc_iq__ffd0_dn2 = assign3930_e5778_d_n2;
        locals.var_fn25_calc_iq__ffd0_dn4 = assign3930_e5778_d_n4;
        locals.var_fn25_calc_iq__ffd0_dn7 = assign3930_e5778_d_n7;
        locals.var_fn25_calc_iq__ffd0_dn16 = assign3930_e5778_d_n16;
        locals.var_fn25_calc_iq__ffd0_dn17 = assign3930_e5778_d_n17;

        let assign3940_e5781: f64 = (-50.0);
        let assign3940_e5782: f64 = if locals.var_fn25_calc_iq__exparg0 < assign3940_e5781 { 1.0 } else { 0.0 };
        locals.var_guard48 = assign3940_e5782;

        let (assign3950_e5791, assign3950_e5791_d_n2, assign3950_e5791_d_n4, assign3950_e5791_d_n7, assign3950_e5791_d_n16, assign3950_e5791_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard47 == 0.0)) && (locals.var_guard48 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__ffd0, locals.var_fn25_calc_iq__ffd0_dn2, locals.var_fn25_calc_iq__ffd0_dn4, locals.var_fn25_calc_iq__ffd0_dn7, locals.var_fn25_calc_iq__ffd0_dn16, locals.var_fn25_calc_iq__ffd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd0 = assign3950_e5791;
        locals.var_fn25_calc_iq__ffd0_dn2 = assign3950_e5791_d_n2;
        locals.var_fn25_calc_iq__ffd0_dn4 = assign3950_e5791_d_n4;
        locals.var_fn25_calc_iq__ffd0_dn7 = assign3950_e5791_d_n7;
        locals.var_fn25_calc_iq__ffd0_dn16 = assign3950_e5791_d_n16;
        locals.var_fn25_calc_iq__ffd0_dn17 = assign3950_e5791_d_n17;

        let (assign3960_e5806, assign3960_e5806_d_n2, assign3960_e5806_d_n4, assign3960_e5806_d_n7, assign3960_e5806_d_n16, assign3960_e5806_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard47 == 0.0)) && (locals.var_guard48 == 0.0)) {
        let assign3960_e5802: f64 = (locals.var_fn25_calc_iq__exparg0).exp();
        let assign3960_e5803: f64 = (1.0 + assign3960_e5802);
        let assign3960_e5804: f64 = (1.0 / assign3960_e5803);
        (assign3960_e5804, (-((assign3960_e5802 * locals.var_fn25_calc_iq__exparg0_dn2) / (assign3960_e5803 * assign3960_e5803))), (-((assign3960_e5802 * locals.var_fn25_calc_iq__exparg0_dn4) / (assign3960_e5803 * assign3960_e5803))), (-((assign3960_e5802 * locals.var_fn25_calc_iq__exparg0_dn7) / (assign3960_e5803 * assign3960_e5803))), (-((assign3960_e5802 * locals.var_fn25_calc_iq__exparg0_dn16) / (assign3960_e5803 * assign3960_e5803))), (-((assign3960_e5802 * locals.var_fn25_calc_iq__exparg0_dn17) / (assign3960_e5803 * assign3960_e5803))),)
    } else {
        (locals.var_fn25_calc_iq__ffd0, locals.var_fn25_calc_iq__ffd0_dn2, locals.var_fn25_calc_iq__ffd0_dn4, locals.var_fn25_calc_iq__ffd0_dn7, locals.var_fn25_calc_iq__ffd0_dn16, locals.var_fn25_calc_iq__ffd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__ffd0 = assign3960_e5806;
        locals.var_fn25_calc_iq__ffd0_dn2 = assign3960_e5806_d_n2;
        locals.var_fn25_calc_iq__ffd0_dn4 = assign3960_e5806_d_n4;
        locals.var_fn25_calc_iq__ffd0_dn7 = assign3960_e5806_d_n7;
        locals.var_fn25_calc_iq__ffd0_dn16 = assign3960_e5806_d_n16;
        locals.var_fn25_calc_iq__ffd0_dn17 = assign3960_e5806_d_n17;

        let (assign3970_e5824, assign3970_e5824_d_n2, assign3970_e5824_d_n4, assign3970_e5824_d_n7, assign3970_e5824_d_n16, assign3970_e5824_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign3970_e5810: f64 = (locals.var_fn25_calc_iq__vgsin - locals.var_fn25_calc_iq__vdx0);
        let assign3970_e5814: f64 = (p.p51 * 0.1);
        let assign3970_e5816: f64 = (assign3970_e5814 * locals.var_fn25_calc_iq__alpha_phit);
        let assign3970_e5818: f64 = (assign3970_e5816 * locals.var_fn25_calc_iq__ffd0);
        let assign3970_e5819: f64 = (locals.var_fn25_calc_iq__vtof - assign3970_e5818);
        let assign3970_e5820: f64 = (assign3970_e5810 - assign3970_e5819);
        let assign3970_e5822: f64 = (assign3970_e5820 / locals.var_fn25_calc_iq__two_n_phit0);
        (assign3970_e5822, (((locals.var_fn25_calc_iq__vgsin_dn2 - locals.var_fn25_calc_iq__vdx0_dn2) - (-(assign3970_e5816 * locals.var_fn25_calc_iq__ffd0_dn2))) / locals.var_fn25_calc_iq__two_n_phit0), (((((-locals.var_fn25_calc_iq__vdx0_dn4) - (locals.var_fn25_calc_iq__vtof_dn4 - (((assign3970_e5814 * locals.var_fn25_calc_iq__alpha_phit_dn4) * locals.var_fn25_calc_iq__ffd0) + (assign3970_e5816 * locals.var_fn25_calc_iq__ffd0_dn4)))) * locals.var_fn25_calc_iq__two_n_phit0) - (assign3970_e5820 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) / (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__two_n_phit0)), (((locals.var_fn25_calc_iq__vgsin_dn7 - locals.var_fn25_calc_iq__vdx0_dn7) - (-(assign3970_e5816 * locals.var_fn25_calc_iq__ffd0_dn7))) / locals.var_fn25_calc_iq__two_n_phit0), (((locals.var_fn25_calc_iq__vgsin_dn16 - locals.var_fn25_calc_iq__vdx0_dn16) - (-(assign3970_e5816 * locals.var_fn25_calc_iq__ffd0_dn16))) / locals.var_fn25_calc_iq__two_n_phit0), (((-locals.var_fn25_calc_iq__vdx0_dn17) - (-(assign3970_e5816 * locals.var_fn25_calc_iq__ffd0_dn17))) / locals.var_fn25_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn25_calc_iq__etad0, locals.var_fn25_calc_iq__etad0_dn2, locals.var_fn25_calc_iq__etad0_dn4, locals.var_fn25_calc_iq__etad0_dn7, locals.var_fn25_calc_iq__etad0_dn16, locals.var_fn25_calc_iq__etad0_dn17,)
    }
};
        locals.var_fn25_calc_iq__etad0 = assign3970_e5824;
        locals.var_fn25_calc_iq__etad0_dn2 = assign3970_e5824_d_n2;
        locals.var_fn25_calc_iq__etad0_dn4 = assign3970_e5824_d_n4;
        locals.var_fn25_calc_iq__etad0_dn7 = assign3970_e5824_d_n7;
        locals.var_fn25_calc_iq__etad0_dn16 = assign3970_e5824_d_n16;
        locals.var_fn25_calc_iq__etad0_dn17 = assign3970_e5824_d_n17;

        let assign3980_e5827: f64 = if locals.var_fn25_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard49 = assign3980_e5827;

        let (assign3990_e5835, assign3990_e5835_d_n2, assign3990_e5835_d_n4, assign3990_e5835_d_n7, assign3990_e5835_d_n16, assign3990_e5835_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard49 != 0.0)) {
        let assign3990_e5833: f64 = (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etad0);
        (assign3990_e5833, (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etad0_dn2), ((locals.var_fn25_calc_iq__qref0_dn4 * locals.var_fn25_calc_iq__etad0) + (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etad0_dn4)), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etad0_dn7), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etad0_dn16), (locals.var_fn25_calc_iq__qref0 * locals.var_fn25_calc_iq__etad0_dn17),)
    } else {
        (locals.var_fn25_calc_iq__qinvd0, locals.var_fn25_calc_iq__qinvd0_dn2, locals.var_fn25_calc_iq__qinvd0_dn4, locals.var_fn25_calc_iq__qinvd0_dn7, locals.var_fn25_calc_iq__qinvd0_dn16, locals.var_fn25_calc_iq__qinvd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd0 = assign3990_e5835;
        locals.var_fn25_calc_iq__qinvd0_dn2 = assign3990_e5835_d_n2;
        locals.var_fn25_calc_iq__qinvd0_dn4 = assign3990_e5835_d_n4;
        locals.var_fn25_calc_iq__qinvd0_dn7 = assign3990_e5835_d_n7;
        locals.var_fn25_calc_iq__qinvd0_dn16 = assign3990_e5835_d_n16;
        locals.var_fn25_calc_iq__qinvd0_dn17 = assign3990_e5835_d_n17;

        let assign4000_e5838: f64 = (-50.0);
        let assign4000_e5839: f64 = if locals.var_fn25_calc_iq__etad0 < assign4000_e5838 { 1.0 } else { 0.0 };
        locals.var_guard50 = assign4000_e5839;

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4010_e5851, assign4010_e5851_d_n2, assign4010_e5851_d_n4, assign4010_e5851_d_n7, assign4010_e5851_d_n16, assign4010_e5851_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard50 != 0.0)) {
        let assign4010_e5848: f64 = (locals.var_fn25_calc_iq__etad0).exp();
        let assign4010_e5849: f64 = (locals.var_fn25_calc_iq__qref0 * assign4010_e5848);
        (assign4010_e5849, (locals.var_fn25_calc_iq__qref0 * (assign4010_e5848 * locals.var_fn25_calc_iq__etad0_dn2)), ((locals.var_fn25_calc_iq__qref0_dn4 * assign4010_e5848) + (locals.var_fn25_calc_iq__qref0 * (assign4010_e5848 * locals.var_fn25_calc_iq__etad0_dn4))), (locals.var_fn25_calc_iq__qref0 * (assign4010_e5848 * locals.var_fn25_calc_iq__etad0_dn7)), (locals.var_fn25_calc_iq__qref0 * (assign4010_e5848 * locals.var_fn25_calc_iq__etad0_dn16)), (locals.var_fn25_calc_iq__qref0 * (assign4010_e5848 * locals.var_fn25_calc_iq__etad0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qinvd0, locals.var_fn25_calc_iq__qinvd0_dn2, locals.var_fn25_calc_iq__qinvd0_dn4, locals.var_fn25_calc_iq__qinvd0_dn7, locals.var_fn25_calc_iq__qinvd0_dn16, locals.var_fn25_calc_iq__qinvd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd0 = assign4010_e5851;
        locals.var_fn25_calc_iq__qinvd0_dn2 = assign4010_e5851_d_n2;
        locals.var_fn25_calc_iq__qinvd0_dn4 = assign4010_e5851_d_n4;
        locals.var_fn25_calc_iq__qinvd0_dn7 = assign4010_e5851_d_n7;
        locals.var_fn25_calc_iq__qinvd0_dn16 = assign4010_e5851_d_n16;
        locals.var_fn25_calc_iq__qinvd0_dn17 = assign4010_e5851_d_n17;

        let (assign4020_e5867, assign4020_e5867_d_n2, assign4020_e5867_d_n4, assign4020_e5867_d_n7, assign4020_e5867_d_n16, assign4020_e5867_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard49 == 0.0)) && (locals.var_guard50 == 0.0)) {
        let assign4020_e5862: f64 = (locals.var_fn25_calc_iq__etad0).exp();
        let assign4020_e5863: f64 = (1.0 + assign4020_e5862);
        let assign4020_e5864: f64 = (assign4020_e5863).ln();
        let assign4020_e5865: f64 = (locals.var_fn25_calc_iq__qref0 * assign4020_e5864);
        (assign4020_e5865, (locals.var_fn25_calc_iq__qref0 * ((assign4020_e5862 * locals.var_fn25_calc_iq__etad0_dn2) / assign4020_e5863)), ((locals.var_fn25_calc_iq__qref0_dn4 * assign4020_e5864) + (locals.var_fn25_calc_iq__qref0 * ((assign4020_e5862 * locals.var_fn25_calc_iq__etad0_dn4) / assign4020_e5863))), (locals.var_fn25_calc_iq__qref0 * ((assign4020_e5862 * locals.var_fn25_calc_iq__etad0_dn7) / assign4020_e5863)), (locals.var_fn25_calc_iq__qref0 * ((assign4020_e5862 * locals.var_fn25_calc_iq__etad0_dn16) / assign4020_e5863)), (locals.var_fn25_calc_iq__qref0 * ((assign4020_e5862 * locals.var_fn25_calc_iq__etad0_dn17) / assign4020_e5863)),)
    } else {
        (locals.var_fn25_calc_iq__qinvd0, locals.var_fn25_calc_iq__qinvd0_dn2, locals.var_fn25_calc_iq__qinvd0_dn4, locals.var_fn25_calc_iq__qinvd0_dn7, locals.var_fn25_calc_iq__qinvd0_dn16, locals.var_fn25_calc_iq__qinvd0_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvd0 = assign4020_e5867;
        locals.var_fn25_calc_iq__qinvd0_dn2 = assign4020_e5867_d_n2;
        locals.var_fn25_calc_iq__qinvd0_dn4 = assign4020_e5867_d_n4;
        locals.var_fn25_calc_iq__qinvd0_dn7 = assign4020_e5867_d_n7;
        locals.var_fn25_calc_iq__qinvd0_dn16 = assign4020_e5867_d_n16;
        locals.var_fn25_calc_iq__qinvd0_dn17 = assign4020_e5867_d_n17;

        let (assign4030_e5875, assign4030_e5875_d_n2, assign4030_e5875_d_n4, assign4030_e5875_d_n7, assign4030_e5875_d_n16, assign4030_e5875_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4030_e5871: f64 = (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvs0);
        let assign4030_e5873: f64 = (assign4030_e5871 + 1e-38);
        (assign4030_e5873, ((locals.var_fn25_calc_iq__qinvs0_dn2 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvs0_dn2)), ((locals.var_fn25_calc_iq__qinvs0_dn4 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvs0_dn4)), ((locals.var_fn25_calc_iq__qinvs0_dn7 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvs0_dn7)), ((locals.var_fn25_calc_iq__qinvs0_dn16 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvs0_dn16)), ((locals.var_fn25_calc_iq__qinvs0_dn17 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvs0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qs2, locals.var_fn25_calc_iq__qs2_dn2, locals.var_fn25_calc_iq__qs2_dn4, locals.var_fn25_calc_iq__qs2_dn7, locals.var_fn25_calc_iq__qs2_dn16, locals.var_fn25_calc_iq__qs2_dn17,)
    }
};
        locals.var_fn25_calc_iq__qs2 = assign4030_e5875;
        locals.var_fn25_calc_iq__qs2_dn2 = assign4030_e5875_d_n2;
        locals.var_fn25_calc_iq__qs2_dn4 = assign4030_e5875_d_n4;
        locals.var_fn25_calc_iq__qs2_dn7 = assign4030_e5875_d_n7;
        locals.var_fn25_calc_iq__qs2_dn16 = assign4030_e5875_d_n16;
        locals.var_fn25_calc_iq__qs2_dn17 = assign4030_e5875_d_n17;

        let (assign4040_e5883, assign4040_e5883_d_n2, assign4040_e5883_d_n4, assign4040_e5883_d_n7, assign4040_e5883_d_n16, assign4040_e5883_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4040_e5879: f64 = (locals.var_fn25_calc_iq__qs2 * locals.var_fn25_calc_iq__qinvs0);
        let assign4040_e5881: f64 = (assign4040_e5879 + 1e-57);
        (assign4040_e5881, ((locals.var_fn25_calc_iq__qs2_dn2 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qs2 * locals.var_fn25_calc_iq__qinvs0_dn2)), ((locals.var_fn25_calc_iq__qs2_dn4 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qs2 * locals.var_fn25_calc_iq__qinvs0_dn4)), ((locals.var_fn25_calc_iq__qs2_dn7 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qs2 * locals.var_fn25_calc_iq__qinvs0_dn7)), ((locals.var_fn25_calc_iq__qs2_dn16 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qs2 * locals.var_fn25_calc_iq__qinvs0_dn16)), ((locals.var_fn25_calc_iq__qs2_dn17 * locals.var_fn25_calc_iq__qinvs0) + (locals.var_fn25_calc_iq__qs2 * locals.var_fn25_calc_iq__qinvs0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qs3, locals.var_fn25_calc_iq__qs3_dn2, locals.var_fn25_calc_iq__qs3_dn4, locals.var_fn25_calc_iq__qs3_dn7, locals.var_fn25_calc_iq__qs3_dn16, locals.var_fn25_calc_iq__qs3_dn17,)
    }
};
        locals.var_fn25_calc_iq__qs3 = assign4040_e5883;
        locals.var_fn25_calc_iq__qs3_dn2 = assign4040_e5883_d_n2;
        locals.var_fn25_calc_iq__qs3_dn4 = assign4040_e5883_d_n4;
        locals.var_fn25_calc_iq__qs3_dn7 = assign4040_e5883_d_n7;
        locals.var_fn25_calc_iq__qs3_dn16 = assign4040_e5883_d_n16;
        locals.var_fn25_calc_iq__qs3_dn17 = assign4040_e5883_d_n17;

        let (assign4050_e5891, assign4050_e5891_d_n2, assign4050_e5891_d_n4, assign4050_e5891_d_n7, assign4050_e5891_d_n16, assign4050_e5891_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4050_e5887: f64 = (locals.var_fn25_calc_iq__qinvd0 * locals.var_fn25_calc_iq__qinvd0);
        let assign4050_e5889: f64 = (assign4050_e5887 + 1e-38);
        (assign4050_e5889, ((locals.var_fn25_calc_iq__qinvd0_dn2 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvd0 * locals.var_fn25_calc_iq__qinvd0_dn2)), ((locals.var_fn25_calc_iq__qinvd0_dn4 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvd0 * locals.var_fn25_calc_iq__qinvd0_dn4)), ((locals.var_fn25_calc_iq__qinvd0_dn7 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvd0 * locals.var_fn25_calc_iq__qinvd0_dn7)), ((locals.var_fn25_calc_iq__qinvd0_dn16 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvd0 * locals.var_fn25_calc_iq__qinvd0_dn16)), ((locals.var_fn25_calc_iq__qinvd0_dn17 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvd0 * locals.var_fn25_calc_iq__qinvd0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qd2, locals.var_fn25_calc_iq__qd2_dn2, locals.var_fn25_calc_iq__qd2_dn4, locals.var_fn25_calc_iq__qd2_dn7, locals.var_fn25_calc_iq__qd2_dn16, locals.var_fn25_calc_iq__qd2_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd2 = assign4050_e5891;
        locals.var_fn25_calc_iq__qd2_dn2 = assign4050_e5891_d_n2;
        locals.var_fn25_calc_iq__qd2_dn4 = assign4050_e5891_d_n4;
        locals.var_fn25_calc_iq__qd2_dn7 = assign4050_e5891_d_n7;
        locals.var_fn25_calc_iq__qd2_dn16 = assign4050_e5891_d_n16;
        locals.var_fn25_calc_iq__qd2_dn17 = assign4050_e5891_d_n17;

        let (assign4060_e5899, assign4060_e5899_d_n2, assign4060_e5899_d_n4, assign4060_e5899_d_n7, assign4060_e5899_d_n16, assign4060_e5899_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4060_e5895: f64 = (locals.var_fn25_calc_iq__qd2 * locals.var_fn25_calc_iq__qinvd0);
        let assign4060_e5897: f64 = (assign4060_e5895 + 1e-57);
        (assign4060_e5897, ((locals.var_fn25_calc_iq__qd2_dn2 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qd2 * locals.var_fn25_calc_iq__qinvd0_dn2)), ((locals.var_fn25_calc_iq__qd2_dn4 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qd2 * locals.var_fn25_calc_iq__qinvd0_dn4)), ((locals.var_fn25_calc_iq__qd2_dn7 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qd2 * locals.var_fn25_calc_iq__qinvd0_dn7)), ((locals.var_fn25_calc_iq__qd2_dn16 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qd2 * locals.var_fn25_calc_iq__qinvd0_dn16)), ((locals.var_fn25_calc_iq__qd2_dn17 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qd2 * locals.var_fn25_calc_iq__qinvd0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qd3, locals.var_fn25_calc_iq__qd3_dn2, locals.var_fn25_calc_iq__qd3_dn4, locals.var_fn25_calc_iq__qd3_dn7, locals.var_fn25_calc_iq__qd3_dn16, locals.var_fn25_calc_iq__qd3_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd3 = assign4060_e5899;
        locals.var_fn25_calc_iq__qd3_dn2 = assign4060_e5899_d_n2;
        locals.var_fn25_calc_iq__qd3_dn4 = assign4060_e5899_d_n4;
        locals.var_fn25_calc_iq__qd3_dn7 = assign4060_e5899_d_n7;
        locals.var_fn25_calc_iq__qd3_dn16 = assign4060_e5899_d_n16;
        locals.var_fn25_calc_iq__qd3_dn17 = assign4060_e5899_d_n17;

        let (assign4070_e5907, assign4070_e5907_d_n2, assign4070_e5907_d_n4, assign4070_e5907_d_n7, assign4070_e5907_d_n16, assign4070_e5907_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4070_e5903: f64 = (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvd0);
        let assign4070_e5905: f64 = (assign4070_e5903 + 1e-38);
        (assign4070_e5905, ((locals.var_fn25_calc_iq__qinvs0_dn2 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvd0_dn2)), ((locals.var_fn25_calc_iq__qinvs0_dn4 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvd0_dn4)), ((locals.var_fn25_calc_iq__qinvs0_dn7 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvd0_dn7)), ((locals.var_fn25_calc_iq__qinvs0_dn16 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvd0_dn16)), ((locals.var_fn25_calc_iq__qinvs0_dn17 * locals.var_fn25_calc_iq__qinvd0) + (locals.var_fn25_calc_iq__qinvs0 * locals.var_fn25_calc_iq__qinvd0_dn17)),)
    } else {
        (locals.var_fn25_calc_iq__qsqd, locals.var_fn25_calc_iq__qsqd_dn2, locals.var_fn25_calc_iq__qsqd_dn4, locals.var_fn25_calc_iq__qsqd_dn7, locals.var_fn25_calc_iq__qsqd_dn16, locals.var_fn25_calc_iq__qsqd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qsqd = assign4070_e5907;
        locals.var_fn25_calc_iq__qsqd_dn2 = assign4070_e5907_d_n2;
        locals.var_fn25_calc_iq__qsqd_dn4 = assign4070_e5907_d_n4;
        locals.var_fn25_calc_iq__qsqd_dn7 = assign4070_e5907_d_n7;
        locals.var_fn25_calc_iq__qsqd_dn16 = assign4070_e5907_d_n16;
        locals.var_fn25_calc_iq__qsqd_dn17 = assign4070_e5907_d_n17;

        let (assign4080_e5925, assign4080_e5925_d_n2, assign4080_e5925_d_n4, assign4080_e5925_d_n7, assign4080_e5925_d_n16, assign4080_e5925_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4080_e5911: f64 = (2.0 / 3.0);
        let assign4080_e5914: f64 = (locals.var_fn25_calc_iq__qs2 + locals.var_fn25_calc_iq__qd2);
        let assign4080_e5916: f64 = (assign4080_e5914 + locals.var_fn25_calc_iq__qsqd);
        let assign4080_e5917: f64 = (assign4080_e5911 * assign4080_e5916);
        let assign4080_e5920: f64 = (locals.var_fn25_calc_iq__qinvs0 + locals.var_fn25_calc_iq__qinvd0);
        let assign4080_e5922: f64 = (assign4080_e5920 + 2e-19);
        let assign4080_e5923: f64 = (assign4080_e5917 / assign4080_e5922);
        (assign4080_e5923, ((((assign4080_e5911 * ((locals.var_fn25_calc_iq__qs2_dn2 + locals.var_fn25_calc_iq__qd2_dn2) + locals.var_fn25_calc_iq__qsqd_dn2)) * assign4080_e5922) - (assign4080_e5917 * (locals.var_fn25_calc_iq__qinvs0_dn2 + locals.var_fn25_calc_iq__qinvd0_dn2))) / (assign4080_e5922 * assign4080_e5922)), ((((assign4080_e5911 * ((locals.var_fn25_calc_iq__qs2_dn4 + locals.var_fn25_calc_iq__qd2_dn4) + locals.var_fn25_calc_iq__qsqd_dn4)) * assign4080_e5922) - (assign4080_e5917 * (locals.var_fn25_calc_iq__qinvs0_dn4 + locals.var_fn25_calc_iq__qinvd0_dn4))) / (assign4080_e5922 * assign4080_e5922)), ((((assign4080_e5911 * ((locals.var_fn25_calc_iq__qs2_dn7 + locals.var_fn25_calc_iq__qd2_dn7) + locals.var_fn25_calc_iq__qsqd_dn7)) * assign4080_e5922) - (assign4080_e5917 * (locals.var_fn25_calc_iq__qinvs0_dn7 + locals.var_fn25_calc_iq__qinvd0_dn7))) / (assign4080_e5922 * assign4080_e5922)), ((((assign4080_e5911 * ((locals.var_fn25_calc_iq__qs2_dn16 + locals.var_fn25_calc_iq__qd2_dn16) + locals.var_fn25_calc_iq__qsqd_dn16)) * assign4080_e5922) - (assign4080_e5917 * (locals.var_fn25_calc_iq__qinvs0_dn16 + locals.var_fn25_calc_iq__qinvd0_dn16))) / (assign4080_e5922 * assign4080_e5922)), ((((assign4080_e5911 * ((locals.var_fn25_calc_iq__qs2_dn17 + locals.var_fn25_calc_iq__qd2_dn17) + locals.var_fn25_calc_iq__qsqd_dn17)) * assign4080_e5922) - (assign4080_e5917 * (locals.var_fn25_calc_iq__qinvs0_dn17 + locals.var_fn25_calc_iq__qinvd0_dn17))) / (assign4080_e5922 * assign4080_e5922)),)
    } else {
        (locals.var_fn25_calc_iq__qinvdd, locals.var_fn25_calc_iq__qinvdd_dn2, locals.var_fn25_calc_iq__qinvdd_dn4, locals.var_fn25_calc_iq__qinvdd_dn7, locals.var_fn25_calc_iq__qinvdd_dn16, locals.var_fn25_calc_iq__qinvdd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qinvdd = assign4080_e5925;
        locals.var_fn25_calc_iq__qinvdd_dn2 = assign4080_e5925_d_n2;
        locals.var_fn25_calc_iq__qinvdd_dn4 = assign4080_e5925_d_n4;
        locals.var_fn25_calc_iq__qinvdd_dn7 = assign4080_e5925_d_n7;
        locals.var_fn25_calc_iq__qinvdd_dn16 = assign4080_e5925_d_n16;
        locals.var_fn25_calc_iq__qinvdd_dn17 = assign4080_e5925_d_n17;

        let (assign4090_e5959, assign4090_e5959_d_n2, assign4090_e5959_d_n4, assign4090_e5959_d_n7, assign4090_e5959_d_n16, assign4090_e5959_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4090_e5930: f64 = (2.0 * locals.var_fn25_calc_iq__qs3);
        let assign4090_e5933: f64 = (3.0 * locals.var_fn25_calc_iq__qd3);
        let assign4090_e5934: f64 = (assign4090_e5930 + assign4090_e5933);
        let assign4090_e5937: f64 = (4.0 * locals.var_fn25_calc_iq__qs2);
        let assign4090_e5939: f64 = (assign4090_e5937 * locals.var_fn25_calc_iq__qinvd0);
        let assign4090_e5940: f64 = (assign4090_e5934 + assign4090_e5939);
        let assign4090_e5943: f64 = (6.0 * locals.var_fn25_calc_iq__qd2);
        let assign4090_e5945: f64 = (assign4090_e5943 * locals.var_fn25_calc_iq__qinvs0);
        let assign4090_e5946: f64 = (assign4090_e5940 + assign4090_e5945);
        let assign4090_e5947: f64 = (2.0 * assign4090_e5946);
        let assign4090_e5951: f64 = (locals.var_fn25_calc_iq__qs2 + locals.var_fn25_calc_iq__qd2);
        let assign4090_e5954: f64 = (2.0 * locals.var_fn25_calc_iq__qsqd);
        let assign4090_e5955: f64 = (assign4090_e5951 + assign4090_e5954);
        let assign4090_e5956: f64 = (15.0 * assign4090_e5955);
        let assign4090_e5957: f64 = (assign4090_e5947 / assign4090_e5956);
        (assign4090_e5957, ((((2.0 * ((((2.0 * locals.var_fn25_calc_iq__qs3_dn2) + (3.0 * locals.var_fn25_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn25_calc_iq__qs2_dn2) * locals.var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * locals.var_fn25_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn25_calc_iq__qd2_dn2) * locals.var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * locals.var_fn25_calc_iq__qinvs0_dn2)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((locals.var_fn25_calc_iq__qs2_dn2 + locals.var_fn25_calc_iq__qd2_dn2) + (2.0 * locals.var_fn25_calc_iq__qsqd_dn2))))) / (assign4090_e5956 * assign4090_e5956)), ((((2.0 * ((((2.0 * locals.var_fn25_calc_iq__qs3_dn4) + (3.0 * locals.var_fn25_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn25_calc_iq__qs2_dn4) * locals.var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * locals.var_fn25_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn25_calc_iq__qd2_dn4) * locals.var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * locals.var_fn25_calc_iq__qinvs0_dn4)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((locals.var_fn25_calc_iq__qs2_dn4 + locals.var_fn25_calc_iq__qd2_dn4) + (2.0 * locals.var_fn25_calc_iq__qsqd_dn4))))) / (assign4090_e5956 * assign4090_e5956)), ((((2.0 * ((((2.0 * locals.var_fn25_calc_iq__qs3_dn7) + (3.0 * locals.var_fn25_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn25_calc_iq__qs2_dn7) * locals.var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * locals.var_fn25_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn25_calc_iq__qd2_dn7) * locals.var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * locals.var_fn25_calc_iq__qinvs0_dn7)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((locals.var_fn25_calc_iq__qs2_dn7 + locals.var_fn25_calc_iq__qd2_dn7) + (2.0 * locals.var_fn25_calc_iq__qsqd_dn7))))) / (assign4090_e5956 * assign4090_e5956)), ((((2.0 * ((((2.0 * locals.var_fn25_calc_iq__qs3_dn16) + (3.0 * locals.var_fn25_calc_iq__qd3_dn16)) + (((4.0 * locals.var_fn25_calc_iq__qs2_dn16) * locals.var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * locals.var_fn25_calc_iq__qinvd0_dn16))) + (((6.0 * locals.var_fn25_calc_iq__qd2_dn16) * locals.var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * locals.var_fn25_calc_iq__qinvs0_dn16)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((locals.var_fn25_calc_iq__qs2_dn16 + locals.var_fn25_calc_iq__qd2_dn16) + (2.0 * locals.var_fn25_calc_iq__qsqd_dn16))))) / (assign4090_e5956 * assign4090_e5956)), ((((2.0 * ((((2.0 * locals.var_fn25_calc_iq__qs3_dn17) + (3.0 * locals.var_fn25_calc_iq__qd3_dn17)) + (((4.0 * locals.var_fn25_calc_iq__qs2_dn17) * locals.var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * locals.var_fn25_calc_iq__qinvd0_dn17))) + (((6.0 * locals.var_fn25_calc_iq__qd2_dn17) * locals.var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * locals.var_fn25_calc_iq__qinvs0_dn17)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((locals.var_fn25_calc_iq__qs2_dn17 + locals.var_fn25_calc_iq__qd2_dn17) + (2.0 * locals.var_fn25_calc_iq__qsqd_dn17))))) / (assign4090_e5956 * assign4090_e5956)),)
    } else {
        (locals.var_fn25_calc_iq__qd1, locals.var_fn25_calc_iq__qd1_dn2, locals.var_fn25_calc_iq__qd1_dn4, locals.var_fn25_calc_iq__qd1_dn7, locals.var_fn25_calc_iq__qd1_dn16, locals.var_fn25_calc_iq__qd1_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd1 = assign4090_e5959;
        locals.var_fn25_calc_iq__qd1_dn2 = assign4090_e5959_d_n2;
        locals.var_fn25_calc_iq__qd1_dn4 = assign4090_e5959_d_n4;
        locals.var_fn25_calc_iq__qd1_dn7 = assign4090_e5959_d_n7;
        locals.var_fn25_calc_iq__qd1_dn16 = assign4090_e5959_d_n16;
        locals.var_fn25_calc_iq__qd1_dn17 = assign4090_e5959_d_n17;

        let (assign4100_e5965, assign4100_e5965_d_n2, assign4100_e5965_d_n4, assign4100_e5965_d_n7, assign4100_e5965_d_n16, assign4100_e5965_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4100_e5963: f64 = (locals.var_fn25_calc_iq__qinvdd - locals.var_fn25_calc_iq__qd1);
        (assign4100_e5963, (locals.var_fn25_calc_iq__qinvdd_dn2 - locals.var_fn25_calc_iq__qd1_dn2), (locals.var_fn25_calc_iq__qinvdd_dn4 - locals.var_fn25_calc_iq__qd1_dn4), (locals.var_fn25_calc_iq__qinvdd_dn7 - locals.var_fn25_calc_iq__qd1_dn7), (locals.var_fn25_calc_iq__qinvdd_dn16 - locals.var_fn25_calc_iq__qd1_dn16), (locals.var_fn25_calc_iq__qinvdd_dn17 - locals.var_fn25_calc_iq__qd1_dn17),)
    } else {
        (locals.var_fn25_calc_iq__qs, locals.var_fn25_calc_iq__qs_dn2, locals.var_fn25_calc_iq__qs_dn4, locals.var_fn25_calc_iq__qs_dn7, locals.var_fn25_calc_iq__qs_dn16, locals.var_fn25_calc_iq__qs_dn17,)
    }
};
        locals.var_fn25_calc_iq__qs = assign4100_e5965;
        locals.var_fn25_calc_iq__qs_dn2 = assign4100_e5965_d_n2;
        locals.var_fn25_calc_iq__qs_dn4 = assign4100_e5965_d_n4;
        locals.var_fn25_calc_iq__qs_dn7 = assign4100_e5965_d_n7;
        locals.var_fn25_calc_iq__qs_dn16 = assign4100_e5965_d_n16;
        locals.var_fn25_calc_iq__qs_dn17 = assign4100_e5965_d_n17;

        let (assign4110_e5969, assign4110_e5969_d_n2, assign4110_e5969_d_n4, assign4110_e5969_d_n7, assign4110_e5969_d_n16, assign4110_e5969_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__qd1, locals.var_fn25_calc_iq__qd1_dn2, locals.var_fn25_calc_iq__qd1_dn4, locals.var_fn25_calc_iq__qd1_dn7, locals.var_fn25_calc_iq__qd1_dn16, locals.var_fn25_calc_iq__qd1_dn17,)
    } else {
        (locals.var_fn25_calc_iq__qd, locals.var_fn25_calc_iq__qd_dn2, locals.var_fn25_calc_iq__qd_dn4, locals.var_fn25_calc_iq__qd_dn7, locals.var_fn25_calc_iq__qd_dn16, locals.var_fn25_calc_iq__qd_dn17,)
    }
};
        locals.var_fn25_calc_iq__qd = assign4110_e5969;
        locals.var_fn25_calc_iq__qd_dn2 = assign4110_e5969_d_n2;
        locals.var_fn25_calc_iq__qd_dn4 = assign4110_e5969_d_n4;
        locals.var_fn25_calc_iq__qd_dn7 = assign4110_e5969_d_n7;
        locals.var_fn25_calc_iq__qd_dn16 = assign4110_e5969_d_n16;
        locals.var_fn25_calc_iq__qd_dn17 = assign4110_e5969_d_n17;

        let (assign4120_e5983, assign4120_e5983_d_n2, assign4120_e5983_d_n4, assign4120_e5983_d_n7, assign4120_e5983_d_n16, assign4120_e5983_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4120_e5973: f64 = (locals.var_fn25_calc_iq__w * locals.var_fn25_calc_iq__ngf);
        let assign4120_e5975: f64 = (assign4120_e5973 * locals.var_fn25_calc_iq__lin);
        let assign4120_e5977: f64 = (assign4120_e5975 * locals.var_fn25_calc_iq__type);
        let assign4120_e5979: f64 = (assign4120_e5977 * locals.var_fn25_calc_iq__qs);
        let assign4120_e5981: f64 = (assign4120_e5979 * locals.var_fn25_calc_iq__trapfracdl);
        (assign4120_e5981, ((assign4120_e5977 * locals.var_fn25_calc_iq__qs_dn2) * locals.var_fn25_calc_iq__trapfracdl), ((assign4120_e5977 * locals.var_fn25_calc_iq__qs_dn4) * locals.var_fn25_calc_iq__trapfracdl), ((assign4120_e5977 * locals.var_fn25_calc_iq__qs_dn7) * locals.var_fn25_calc_iq__trapfracdl), ((assign4120_e5977 * locals.var_fn25_calc_iq__qs_dn16) * locals.var_fn25_calc_iq__trapfracdl), ((assign4120_e5977 * locals.var_fn25_calc_iq__qs_dn17) * locals.var_fn25_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn25_calc_iq__qgsout, locals.var_fn25_calc_iq__qgsout_dn2, locals.var_fn25_calc_iq__qgsout_dn4, locals.var_fn25_calc_iq__qgsout_dn7, locals.var_fn25_calc_iq__qgsout_dn16, locals.var_fn25_calc_iq__qgsout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qgsout = assign4120_e5983;
        locals.var_fn25_calc_iq__qgsout_dn2 = assign4120_e5983_d_n2;
        locals.var_fn25_calc_iq__qgsout_dn4 = assign4120_e5983_d_n4;
        locals.var_fn25_calc_iq__qgsout_dn7 = assign4120_e5983_d_n7;
        locals.var_fn25_calc_iq__qgsout_dn16 = assign4120_e5983_d_n16;
        locals.var_fn25_calc_iq__qgsout_dn17 = assign4120_e5983_d_n17;

        let (assign4130_e5997, assign4130_e5997_d_n2, assign4130_e5997_d_n4, assign4130_e5997_d_n7, assign4130_e5997_d_n16, assign4130_e5997_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        let assign4130_e5987: f64 = (locals.var_fn25_calc_iq__w * locals.var_fn25_calc_iq__ngf);
        let assign4130_e5989: f64 = (assign4130_e5987 * locals.var_fn25_calc_iq__lin);
        let assign4130_e5991: f64 = (assign4130_e5989 * locals.var_fn25_calc_iq__type);
        let assign4130_e5993: f64 = (assign4130_e5991 * locals.var_fn25_calc_iq__qd);
        let assign4130_e5995: f64 = (assign4130_e5993 * locals.var_fn25_calc_iq__trapfracdl);
        (assign4130_e5995, ((assign4130_e5991 * locals.var_fn25_calc_iq__qd_dn2) * locals.var_fn25_calc_iq__trapfracdl), ((assign4130_e5991 * locals.var_fn25_calc_iq__qd_dn4) * locals.var_fn25_calc_iq__trapfracdl), ((assign4130_e5991 * locals.var_fn25_calc_iq__qd_dn7) * locals.var_fn25_calc_iq__trapfracdl), ((assign4130_e5991 * locals.var_fn25_calc_iq__qd_dn16) * locals.var_fn25_calc_iq__trapfracdl), ((assign4130_e5991 * locals.var_fn25_calc_iq__qd_dn17) * locals.var_fn25_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn25_calc_iq__qgdout, locals.var_fn25_calc_iq__qgdout_dn2, locals.var_fn25_calc_iq__qgdout_dn4, locals.var_fn25_calc_iq__qgdout_dn7, locals.var_fn25_calc_iq__qgdout_dn16, locals.var_fn25_calc_iq__qgdout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qgdout = assign4130_e5997;
        locals.var_fn25_calc_iq__qgdout_dn2 = assign4130_e5997_d_n2;
        locals.var_fn25_calc_iq__qgdout_dn4 = assign4130_e5997_d_n4;
        locals.var_fn25_calc_iq__qgdout_dn7 = assign4130_e5997_d_n7;
        locals.var_fn25_calc_iq__qgdout_dn16 = assign4130_e5997_d_n16;
        locals.var_fn25_calc_iq__qgdout_dn17 = assign4130_e5997_d_n17;

        let assign4140_e6000: f64 = if locals.var_fn25_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard51 = assign4140_e6000;

        let (assign4150_e6016, assign4150_e6016_d_n2, assign4150_e6016_d_n4, assign4150_e6016_d_n7, assign4150_e6016_d_n16,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign4150_e6008: f64 = (p.p51 * 0.5);
        let assign4150_e6010: f64 = (assign4150_e6008 * locals.var_fn25_calc_iq__alpha_phit);
        let assign4150_e6011: f64 = (locals.var_fn25_calc_iq__vtof - assign4150_e6010);
        let assign4150_e6012: f64 = (locals.var_fn25_calc_iq__vcin - assign4150_e6011);
        let assign4150_e6014: f64 = (assign4150_e6012 / locals.var_fn25_calc_iq__two_n_phit0);
        (assign4150_e6014, (locals.var_fn25_calc_iq__vcin_dn2 / locals.var_fn25_calc_iq__two_n_phit0), ((((-(locals.var_fn25_calc_iq__vtof_dn4 - (assign4150_e6008 * locals.var_fn25_calc_iq__alpha_phit_dn4))) * locals.var_fn25_calc_iq__two_n_phit0) - (assign4150_e6012 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) / (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__two_n_phit0)), (locals.var_fn25_calc_iq__vcin_dn7 / locals.var_fn25_calc_iq__two_n_phit0), (locals.var_fn25_calc_iq__vcin_dn16 / locals.var_fn25_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn25_calc_iq__etac, locals.var_fn25_calc_iq__etac_dn2, locals.var_fn25_calc_iq__etac_dn4, locals.var_fn25_calc_iq__etac_dn7, locals.var_fn25_calc_iq__etac_dn16,)
    }
};
        locals.var_fn25_calc_iq__etac = assign4150_e6016;
        locals.var_fn25_calc_iq__etac_dn2 = assign4150_e6016_d_n2;
        locals.var_fn25_calc_iq__etac_dn4 = assign4150_e6016_d_n4;
        locals.var_fn25_calc_iq__etac_dn7 = assign4150_e6016_d_n7;
        locals.var_fn25_calc_iq__etac_dn16 = assign4150_e6016_d_n16;

        let assign4160_e6019: f64 = if locals.var_fn25_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard52 = assign4160_e6019;

        let (assign4170_e6027, assign4170_e6027_d_n2, assign4170_e6027_d_n3, assign4170_e6027_d_n4, assign4170_e6027_d_n7, assign4170_e6027_d_n16, assign4170_e6027_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard52 != 0.0)) {
        (locals.var_fn25_calc_iq__etac, locals.var_fn25_calc_iq__etac_dn2, 0.0, locals.var_fn25_calc_iq__etac_dn4, locals.var_fn25_calc_iq__etac_dn7, locals.var_fn25_calc_iq__etac_dn16, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4170_e6027;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4170_e6027_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4170_e6027_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4170_e6027_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4170_e6027_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4170_e6027_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4170_e6027_d_n17;

        let assign4180_e6030: f64 = (-50.0);
        let assign4180_e6031: f64 = if locals.var_fn25_calc_iq__etac < assign4180_e6030 { 1.0 } else { 0.0 };
        locals.var_guard53 = assign4180_e6031;

        let (assign4190_e6043, assign4190_e6043_d_n2, assign4190_e6043_d_n3, assign4190_e6043_d_n4, assign4190_e6043_d_n7, assign4190_e6043_d_n16, assign4190_e6043_d_n17,) = {
    if ((((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard52 == 0.0)) && (locals.var_guard53 != 0.0)) {
        let assign4190_e6041: f64 = (locals.var_fn25_calc_iq__etac).exp();
        (assign4190_e6041, (assign4190_e6041 * locals.var_fn25_calc_iq__etac_dn2), 0.0, (assign4190_e6041 * locals.var_fn25_calc_iq__etac_dn4), (assign4190_e6041 * locals.var_fn25_calc_iq__etac_dn7), (assign4190_e6041 * locals.var_fn25_calc_iq__etac_dn16), 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4190_e6043;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4190_e6043_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4190_e6043_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4190_e6043_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4190_e6043_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4190_e6043_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4190_e6043_d_n17;

        let (assign4200_e6059, assign4200_e6059_d_n2, assign4200_e6059_d_n3, assign4200_e6059_d_n4, assign4200_e6059_d_n7, assign4200_e6059_d_n16, assign4200_e6059_d_n17,) = {
    if ((((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard52 == 0.0)) && (locals.var_guard53 == 0.0)) {
        let assign4200_e6055: f64 = (locals.var_fn25_calc_iq__etac).exp();
        let assign4200_e6056: f64 = (1.0 + assign4200_e6055);
        let assign4200_e6057: f64 = (assign4200_e6056).ln();
        (assign4200_e6057, ((assign4200_e6055 * locals.var_fn25_calc_iq__etac_dn2) / assign4200_e6056), 0.0, ((assign4200_e6055 * locals.var_fn25_calc_iq__etac_dn4) / assign4200_e6056), ((assign4200_e6055 * locals.var_fn25_calc_iq__etac_dn7) / assign4200_e6056), ((assign4200_e6055 * locals.var_fn25_calc_iq__etac_dn16) / assign4200_e6056), 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4200_e6059;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4200_e6059_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4200_e6059_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4200_e6059_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4200_e6059_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4200_e6059_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4200_e6059_d_n17;

        let (assign4210_e6077, assign4210_e6077_d_n2, assign4210_e6077_d_n3, assign4210_e6077_d_n4, assign4210_e6077_d_n7, assign4210_e6077_d_n16, assign4210_e6077_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign4210_e6065: f64 = (locals.var_fn25_calc_iq__w * locals.var_fn25_calc_iq__ngf);
        let assign4210_e6067: f64 = (assign4210_e6065 * locals.var_fn25_calc_iq__type);
        let assign4210_e6069: f64 = (assign4210_e6067 * locals.var_fn25_calc_iq__cc);
        let assign4210_e6071: f64 = (assign4210_e6069 * locals.var_fn25_calc_iq__two_n_phit0);
        let assign4210_e6073: f64 = (assign4210_e6071 * locals.var_fn25_calc_iq__exparg);
        let assign4210_e6075: f64 = (assign4210_e6073 * locals.var_fn25_calc_iq__trapfracdl);
        (assign4210_e6075, ((assign4210_e6071 * locals.var_fn25_calc_iq__exparg_dn2) * locals.var_fn25_calc_iq__trapfracdl), ((assign4210_e6071 * locals.var_fn25_calc_iq__exparg_dn3) * locals.var_fn25_calc_iq__trapfracdl), ((((((assign4210_e6067 * locals.var_fn25_calc_iq__cc_dn4) * locals.var_fn25_calc_iq__two_n_phit0) + (assign4210_e6069 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) * locals.var_fn25_calc_iq__exparg) + (assign4210_e6071 * locals.var_fn25_calc_iq__exparg_dn4)) * locals.var_fn25_calc_iq__trapfracdl), ((assign4210_e6071 * locals.var_fn25_calc_iq__exparg_dn7) * locals.var_fn25_calc_iq__trapfracdl), ((assign4210_e6071 * locals.var_fn25_calc_iq__exparg_dn16) * locals.var_fn25_calc_iq__trapfracdl), ((assign4210_e6071 * locals.var_fn25_calc_iq__exparg_dn17) * locals.var_fn25_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn25_calc_iq__qcout, locals.var_fn25_calc_iq__qcout_dn2, locals.var_fn25_calc_iq__qcout_dn3, locals.var_fn25_calc_iq__qcout_dn4, locals.var_fn25_calc_iq__qcout_dn7, locals.var_fn25_calc_iq__qcout_dn16, locals.var_fn25_calc_iq__qcout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qcout = assign4210_e6077;
        locals.var_fn25_calc_iq__qcout_dn2 = assign4210_e6077_d_n2;
        locals.var_fn25_calc_iq__qcout_dn3 = assign4210_e6077_d_n3;
        locals.var_fn25_calc_iq__qcout_dn4 = assign4210_e6077_d_n4;
        locals.var_fn25_calc_iq__qcout_dn7 = assign4210_e6077_d_n7;
        locals.var_fn25_calc_iq__qcout_dn16 = assign4210_e6077_d_n16;
        locals.var_fn25_calc_iq__qcout_dn17 = assign4210_e6077_d_n17;

        let (assign4220_e6093, assign4220_e6093_d_n3, assign4220_e6093_d_n4, assign4220_e6093_d_n16,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign4220_e6085: f64 = (p.p51 * 0.5);
        let assign4220_e6087: f64 = (assign4220_e6085 * locals.var_fn25_calc_iq__alpha_phit);
        let assign4220_e6088: f64 = (locals.var_fn25_calc_iq__vtof - assign4220_e6087);
        let assign4220_e6089: f64 = (locals.var_fn25_calc_iq__vbin - assign4220_e6088);
        let assign4220_e6091: f64 = (assign4220_e6089 / locals.var_fn25_calc_iq__two_n_phit0);
        (assign4220_e6091, (locals.var_fn25_calc_iq__vbin_dn3 / locals.var_fn25_calc_iq__two_n_phit0), ((((-(locals.var_fn25_calc_iq__vtof_dn4 - (assign4220_e6085 * locals.var_fn25_calc_iq__alpha_phit_dn4))) * locals.var_fn25_calc_iq__two_n_phit0) - (assign4220_e6089 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) / (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__two_n_phit0)), (locals.var_fn25_calc_iq__vbin_dn16 / locals.var_fn25_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn25_calc_iq__etab, locals.var_fn25_calc_iq__etab_dn3, locals.var_fn25_calc_iq__etab_dn4, locals.var_fn25_calc_iq__etab_dn16,)
    }
};
        locals.var_fn25_calc_iq__etab = assign4220_e6093;
        locals.var_fn25_calc_iq__etab_dn3 = assign4220_e6093_d_n3;
        locals.var_fn25_calc_iq__etab_dn4 = assign4220_e6093_d_n4;
        locals.var_fn25_calc_iq__etab_dn16 = assign4220_e6093_d_n16;

        let assign4230_e6096: f64 = if locals.var_fn25_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard54 = assign4230_e6096;

        let (assign4240_e6104, assign4240_e6104_d_n2, assign4240_e6104_d_n3, assign4240_e6104_d_n4, assign4240_e6104_d_n7, assign4240_e6104_d_n16, assign4240_e6104_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard54 != 0.0)) {
        (locals.var_fn25_calc_iq__etab, 0.0, locals.var_fn25_calc_iq__etab_dn3, locals.var_fn25_calc_iq__etab_dn4, 0.0, locals.var_fn25_calc_iq__etab_dn16, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4240_e6104;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4240_e6104_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4240_e6104_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4240_e6104_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4240_e6104_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4240_e6104_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4240_e6104_d_n17;

        let assign4250_e6107: f64 = (-50.0);
        let assign4250_e6108: f64 = if locals.var_fn25_calc_iq__etab < assign4250_e6107 { 1.0 } else { 0.0 };
        locals.var_guard55 = assign4250_e6108;

        let (assign4260_e6120, assign4260_e6120_d_n2, assign4260_e6120_d_n3, assign4260_e6120_d_n4, assign4260_e6120_d_n7, assign4260_e6120_d_n16, assign4260_e6120_d_n17,) = {
    if ((((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 != 0.0)) {
        let assign4260_e6118: f64 = (locals.var_fn25_calc_iq__etab).exp();
        (assign4260_e6118, 0.0, (assign4260_e6118 * locals.var_fn25_calc_iq__etab_dn3), (assign4260_e6118 * locals.var_fn25_calc_iq__etab_dn4), 0.0, (assign4260_e6118 * locals.var_fn25_calc_iq__etab_dn16), 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4260_e6120;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4260_e6120_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4260_e6120_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4260_e6120_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4260_e6120_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4260_e6120_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4260_e6120_d_n17;

        let (assign4270_e6136, assign4270_e6136_d_n2, assign4270_e6136_d_n3, assign4270_e6136_d_n4, assign4270_e6136_d_n7, assign4270_e6136_d_n16, assign4270_e6136_d_n17,) = {
    if ((((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 == 0.0)) {
        let assign4270_e6132: f64 = (locals.var_fn25_calc_iq__etab).exp();
        let assign4270_e6133: f64 = (1.0 + assign4270_e6132);
        let assign4270_e6134: f64 = (assign4270_e6133).ln();
        (assign4270_e6134, 0.0, ((assign4270_e6132 * locals.var_fn25_calc_iq__etab_dn3) / assign4270_e6133), ((assign4270_e6132 * locals.var_fn25_calc_iq__etab_dn4) / assign4270_e6133), 0.0, ((assign4270_e6132 * locals.var_fn25_calc_iq__etab_dn16) / assign4270_e6133), 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4270_e6136;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4270_e6136_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4270_e6136_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4270_e6136_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4270_e6136_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4270_e6136_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4270_e6136_d_n17;

        let (assign4280_e6154, assign4280_e6154_d_n2, assign4280_e6154_d_n3, assign4280_e6154_d_n4, assign4280_e6154_d_n7, assign4280_e6154_d_n16, assign4280_e6154_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign4280_e6142: f64 = (locals.var_fn25_calc_iq__w * locals.var_fn25_calc_iq__ngf);
        let assign4280_e6144: f64 = (assign4280_e6142 * locals.var_fn25_calc_iq__type);
        let assign4280_e6146: f64 = (assign4280_e6144 * locals.var_fn25_calc_iq__cb);
        let assign4280_e6148: f64 = (assign4280_e6146 * locals.var_fn25_calc_iq__two_n_phit0);
        let assign4280_e6150: f64 = (assign4280_e6148 * locals.var_fn25_calc_iq__exparg);
        let assign4280_e6152: f64 = (assign4280_e6150 * locals.var_fn25_calc_iq__trapfracdl);
        (assign4280_e6152, ((assign4280_e6148 * locals.var_fn25_calc_iq__exparg_dn2) * locals.var_fn25_calc_iq__trapfracdl), ((assign4280_e6148 * locals.var_fn25_calc_iq__exparg_dn3) * locals.var_fn25_calc_iq__trapfracdl), ((((((assign4280_e6144 * locals.var_fn25_calc_iq__cb_dn4) * locals.var_fn25_calc_iq__two_n_phit0) + (assign4280_e6146 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) * locals.var_fn25_calc_iq__exparg) + (assign4280_e6148 * locals.var_fn25_calc_iq__exparg_dn4)) * locals.var_fn25_calc_iq__trapfracdl), ((assign4280_e6148 * locals.var_fn25_calc_iq__exparg_dn7) * locals.var_fn25_calc_iq__trapfracdl), ((assign4280_e6148 * locals.var_fn25_calc_iq__exparg_dn16) * locals.var_fn25_calc_iq__trapfracdl), ((assign4280_e6148 * locals.var_fn25_calc_iq__exparg_dn17) * locals.var_fn25_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn25_calc_iq__qbout, locals.var_fn25_calc_iq__qbout_dn2, locals.var_fn25_calc_iq__qbout_dn3, locals.var_fn25_calc_iq__qbout_dn4, locals.var_fn25_calc_iq__qbout_dn7, locals.var_fn25_calc_iq__qbout_dn16, locals.var_fn25_calc_iq__qbout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qbout = assign4280_e6154;
        locals.var_fn25_calc_iq__qbout_dn2 = assign4280_e6154_d_n2;
        locals.var_fn25_calc_iq__qbout_dn3 = assign4280_e6154_d_n3;
        locals.var_fn25_calc_iq__qbout_dn4 = assign4280_e6154_d_n4;
        locals.var_fn25_calc_iq__qbout_dn7 = assign4280_e6154_d_n7;
        locals.var_fn25_calc_iq__qbout_dn16 = assign4280_e6154_d_n16;
        locals.var_fn25_calc_iq__qbout_dn17 = assign4280_e6154_d_n17;

        let (assign4290_e6161, assign4290_e6161_d_n2, assign4290_e6161_d_n3, assign4290_e6161_d_n4, assign4290_e6161_d_n7, assign4290_e6161_d_n16, assign4290_e6161_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard51 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qcout, locals.var_fn25_calc_iq__qcout_dn2, locals.var_fn25_calc_iq__qcout_dn3, locals.var_fn25_calc_iq__qcout_dn4, locals.var_fn25_calc_iq__qcout_dn7, locals.var_fn25_calc_iq__qcout_dn16, locals.var_fn25_calc_iq__qcout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qcout = assign4290_e6161;
        locals.var_fn25_calc_iq__qcout_dn2 = assign4290_e6161_d_n2;
        locals.var_fn25_calc_iq__qcout_dn3 = assign4290_e6161_d_n3;
        locals.var_fn25_calc_iq__qcout_dn4 = assign4290_e6161_d_n4;
        locals.var_fn25_calc_iq__qcout_dn7 = assign4290_e6161_d_n7;
        locals.var_fn25_calc_iq__qcout_dn16 = assign4290_e6161_d_n16;
        locals.var_fn25_calc_iq__qcout_dn17 = assign4290_e6161_d_n17;

        let (assign4300_e6168, assign4300_e6168_d_n2, assign4300_e6168_d_n3, assign4300_e6168_d_n4, assign4300_e6168_d_n7, assign4300_e6168_d_n16, assign4300_e6168_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard51 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qbout, locals.var_fn25_calc_iq__qbout_dn2, locals.var_fn25_calc_iq__qbout_dn3, locals.var_fn25_calc_iq__qbout_dn4, locals.var_fn25_calc_iq__qbout_dn7, locals.var_fn25_calc_iq__qbout_dn16, locals.var_fn25_calc_iq__qbout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qbout = assign4300_e6168;
        locals.var_fn25_calc_iq__qbout_dn2 = assign4300_e6168_d_n2;
        locals.var_fn25_calc_iq__qbout_dn3 = assign4300_e6168_d_n3;
        locals.var_fn25_calc_iq__qbout_dn4 = assign4300_e6168_d_n4;
        locals.var_fn25_calc_iq__qbout_dn7 = assign4300_e6168_d_n7;
        locals.var_fn25_calc_iq__qbout_dn16 = assign4300_e6168_d_n16;
        locals.var_fn25_calc_iq__qbout_dn17 = assign4300_e6168_d_n17;

        let assign4310_e6171: f64 = if locals.var_fn25_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard56 = assign4310_e6171;

        let (assign4320_e6187, assign4320_e6187_d_n2, assign4320_e6187_d_n4, assign4320_e6187_d_n7, assign4320_e6187_d_n16,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard56 != 0.0)) {
        let assign4320_e6179: f64 = (p.p51 * 0.5);
        let assign4320_e6181: f64 = (assign4320_e6179 * locals.var_fn25_calc_iq__alpha_phit);
        let assign4320_e6182: f64 = (locals.var_fn25_calc_iq__vtof - assign4320_e6181);
        let assign4320_e6183: f64 = (locals.var_fn25_calc_iq__vgsin - assign4320_e6182);
        let assign4320_e6185: f64 = (assign4320_e6183 / locals.var_fn25_calc_iq__two_n_phit0);
        (assign4320_e6185, (locals.var_fn25_calc_iq__vgsin_dn2 / locals.var_fn25_calc_iq__two_n_phit0), ((((-(locals.var_fn25_calc_iq__vtof_dn4 - (assign4320_e6179 * locals.var_fn25_calc_iq__alpha_phit_dn4))) * locals.var_fn25_calc_iq__two_n_phit0) - (assign4320_e6183 * locals.var_fn25_calc_iq__two_n_phit0_dn4)) / (locals.var_fn25_calc_iq__two_n_phit0 * locals.var_fn25_calc_iq__two_n_phit0)), (locals.var_fn25_calc_iq__vgsin_dn7 / locals.var_fn25_calc_iq__two_n_phit0), (locals.var_fn25_calc_iq__vgsin_dn16 / locals.var_fn25_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn25_calc_iq__etags, locals.var_fn25_calc_iq__etags_dn2, locals.var_fn25_calc_iq__etags_dn4, locals.var_fn25_calc_iq__etags_dn7, locals.var_fn25_calc_iq__etags_dn16,)
    }
};
        locals.var_fn25_calc_iq__etags = assign4320_e6187;
        locals.var_fn25_calc_iq__etags_dn2 = assign4320_e6187_d_n2;
        locals.var_fn25_calc_iq__etags_dn4 = assign4320_e6187_d_n4;
        locals.var_fn25_calc_iq__etags_dn7 = assign4320_e6187_d_n7;
        locals.var_fn25_calc_iq__etags_dn16 = assign4320_e6187_d_n16;

        let assign4330_e6190: f64 = if locals.var_fn25_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard57 = assign4330_e6190;

        let (assign4340_e6198, assign4340_e6198_d_n2, assign4340_e6198_d_n3, assign4340_e6198_d_n4, assign4340_e6198_d_n7, assign4340_e6198_d_n16, assign4340_e6198_d_n17,) = {
    if (((locals.var_guard24 != 0.0) && (locals.var_guard56 != 0.0)) && (locals.var_guard57 != 0.0)) {
        (locals.var_fn25_calc_iq__etags, locals.var_fn25_calc_iq__etags_dn2, 0.0, locals.var_fn25_calc_iq__etags_dn4, locals.var_fn25_calc_iq__etags_dn7, locals.var_fn25_calc_iq__etags_dn16, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4340_e6198;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4340_e6198_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4340_e6198_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4340_e6198_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4340_e6198_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4340_e6198_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4340_e6198_d_n17;

        let assign4350_e6201: f64 = (-50.0);
        let assign4350_e6202: f64 = if locals.var_fn25_calc_iq__etags < assign4350_e6201 { 1.0 } else { 0.0 };
        locals.var_guard58 = assign4350_e6202;

        let (assign4360_e6214, assign4360_e6214_d_n2, assign4360_e6214_d_n3, assign4360_e6214_d_n4, assign4360_e6214_d_n7, assign4360_e6214_d_n16, assign4360_e6214_d_n17,) = {
    if ((((locals.var_guard24 != 0.0) && (locals.var_guard56 != 0.0)) && (locals.var_guard57 == 0.0)) && (locals.var_guard58 != 0.0)) {
        let assign4360_e6212: f64 = (locals.var_fn25_calc_iq__etags).exp();
        (assign4360_e6212, (assign4360_e6212 * locals.var_fn25_calc_iq__etags_dn2), 0.0, (assign4360_e6212 * locals.var_fn25_calc_iq__etags_dn4), (assign4360_e6212 * locals.var_fn25_calc_iq__etags_dn7), (assign4360_e6212 * locals.var_fn25_calc_iq__etags_dn16), 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4360_e6214;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4360_e6214_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4360_e6214_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4360_e6214_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4360_e6214_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4360_e6214_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4360_e6214_d_n17;

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4370_e6230, assign4370_e6230_d_n2, assign4370_e6230_d_n3, assign4370_e6230_d_n4, assign4370_e6230_d_n7, assign4370_e6230_d_n16, assign4370_e6230_d_n17,) = {
    if ((((locals.var_guard24 != 0.0) && (locals.var_guard56 != 0.0)) && (locals.var_guard57 == 0.0)) && (locals.var_guard58 == 0.0)) {
        let assign4370_e6226: f64 = (locals.var_fn25_calc_iq__etags).exp();
        let assign4370_e6227: f64 = (1.0 + assign4370_e6226);
        let assign4370_e6228: f64 = (assign4370_e6227).ln();
        (assign4370_e6228, ((assign4370_e6226 * locals.var_fn25_calc_iq__etags_dn2) / assign4370_e6227), 0.0, ((assign4370_e6226 * locals.var_fn25_calc_iq__etags_dn4) / assign4370_e6227), ((assign4370_e6226 * locals.var_fn25_calc_iq__etags_dn7) / assign4370_e6227), ((assign4370_e6226 * locals.var_fn25_calc_iq__etags_dn16) / assign4370_e6227), 0.0,)
    } else {
        (locals.var_fn25_calc_iq__exparg, locals.var_fn25_calc_iq__exparg_dn2, locals.var_fn25_calc_iq__exparg_dn3, locals.var_fn25_calc_iq__exparg_dn4, locals.var_fn25_calc_iq__exparg_dn7, locals.var_fn25_calc_iq__exparg_dn16, locals.var_fn25_calc_iq__exparg_dn17,)
    }
};
        locals.var_fn25_calc_iq__exparg = assign4370_e6230;
        locals.var_fn25_calc_iq__exparg_dn2 = assign4370_e6230_d_n2;
        locals.var_fn25_calc_iq__exparg_dn3 = assign4370_e6230_d_n3;
        locals.var_fn25_calc_iq__exparg_dn4 = assign4370_e6230_d_n4;
        locals.var_fn25_calc_iq__exparg_dn7 = assign4370_e6230_d_n7;
        locals.var_fn25_calc_iq__exparg_dn16 = assign4370_e6230_d_n16;
        locals.var_fn25_calc_iq__exparg_dn17 = assign4370_e6230_d_n17;

        let (assign4380_e6248, assign4380_e6248_d_n2, assign4380_e6248_d_n3, assign4380_e6248_d_n4, assign4380_e6248_d_n7, assign4380_e6248_d_n16, assign4380_e6248_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard56 != 0.0)) {
        let assign4380_e6236: f64 = (locals.var_fn25_calc_iq__w * locals.var_fn25_calc_iq__ngf);
        let assign4380_e6238: f64 = (assign4380_e6236 * locals.var_fn25_calc_iq__type);
        let assign4380_e6240: f64 = (assign4380_e6238 * locals.var_fn25_calc_iq__cs);
        let assign4380_e6242: f64 = (assign4380_e6240 * locals.var_fn25_calc_iq__two_n_phit0);
        let assign4380_e6244: f64 = (assign4380_e6242 * locals.var_fn25_calc_iq__exparg);
        let assign4380_e6246: f64 = (assign4380_e6244 * locals.var_fn25_calc_iq__trapfracdl);
        (assign4380_e6246, ((assign4380_e6242 * locals.var_fn25_calc_iq__exparg_dn2) * locals.var_fn25_calc_iq__trapfracdl), ((assign4380_e6242 * locals.var_fn25_calc_iq__exparg_dn3) * locals.var_fn25_calc_iq__trapfracdl), ((((assign4380_e6240 * locals.var_fn25_calc_iq__two_n_phit0_dn4) * locals.var_fn25_calc_iq__exparg) + (assign4380_e6242 * locals.var_fn25_calc_iq__exparg_dn4)) * locals.var_fn25_calc_iq__trapfracdl), ((assign4380_e6242 * locals.var_fn25_calc_iq__exparg_dn7) * locals.var_fn25_calc_iq__trapfracdl), ((assign4380_e6242 * locals.var_fn25_calc_iq__exparg_dn16) * locals.var_fn25_calc_iq__trapfracdl), ((assign4380_e6242 * locals.var_fn25_calc_iq__exparg_dn17) * locals.var_fn25_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn25_calc_iq__qsout, locals.var_fn25_calc_iq__qsout_dn2, locals.var_fn25_calc_iq__qsout_dn3, locals.var_fn25_calc_iq__qsout_dn4, locals.var_fn25_calc_iq__qsout_dn7, locals.var_fn25_calc_iq__qsout_dn16, locals.var_fn25_calc_iq__qsout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qsout = assign4380_e6248;
        locals.var_fn25_calc_iq__qsout_dn2 = assign4380_e6248_d_n2;
        locals.var_fn25_calc_iq__qsout_dn3 = assign4380_e6248_d_n3;
        locals.var_fn25_calc_iq__qsout_dn4 = assign4380_e6248_d_n4;
        locals.var_fn25_calc_iq__qsout_dn7 = assign4380_e6248_d_n7;
        locals.var_fn25_calc_iq__qsout_dn16 = assign4380_e6248_d_n16;
        locals.var_fn25_calc_iq__qsout_dn17 = assign4380_e6248_d_n17;

        let (assign4390_e6255, assign4390_e6255_d_n2, assign4390_e6255_d_n3, assign4390_e6255_d_n4, assign4390_e6255_d_n7, assign4390_e6255_d_n16, assign4390_e6255_d_n17,) = {
    if ((locals.var_guard24 != 0.0) && (locals.var_guard56 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn25_calc_iq__qsout, locals.var_fn25_calc_iq__qsout_dn2, locals.var_fn25_calc_iq__qsout_dn3, locals.var_fn25_calc_iq__qsout_dn4, locals.var_fn25_calc_iq__qsout_dn7, locals.var_fn25_calc_iq__qsout_dn16, locals.var_fn25_calc_iq__qsout_dn17,)
    }
};
        locals.var_fn25_calc_iq__qsout = assign4390_e6255;
        locals.var_fn25_calc_iq__qsout_dn2 = assign4390_e6255_d_n2;
        locals.var_fn25_calc_iq__qsout_dn3 = assign4390_e6255_d_n3;
        locals.var_fn25_calc_iq__qsout_dn4 = assign4390_e6255_d_n4;
        locals.var_fn25_calc_iq__qsout_dn7 = assign4390_e6255_d_n7;
        locals.var_fn25_calc_iq__qsout_dn16 = assign4390_e6255_d_n16;
        locals.var_fn25_calc_iq__qsout_dn17 = assign4390_e6255_d_n17;

        let (assign4400_e6259, assign4400_e6259_d_n2, assign4400_e6259_d_n3, assign4400_e6259_d_n4, assign4400_e6259_d_n7, assign4400_e6259_d_n16, assign4400_e6259_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__idsout, locals.var_fn25_calc_iq__idsout_dn2, locals.var_fn25_calc_iq__idsout_dn3, locals.var_fn25_calc_iq__idsout_dn4, locals.var_fn25_calc_iq__idsout_dn7, locals.var_fn25_calc_iq__idsout_dn16, locals.var_fn25_calc_iq__idsout_dn17,)
    } else {
        (locals.var_fn25_calc_iq__return, locals.var_fn25_calc_iq__return_dn2, locals.var_fn25_calc_iq__return_dn3, locals.var_fn25_calc_iq__return_dn4, locals.var_fn25_calc_iq__return_dn7, locals.var_fn25_calc_iq__return_dn16, locals.var_fn25_calc_iq__return_dn17,)
    }
};
        locals.var_fn25_calc_iq__return = assign4400_e6259;
        locals.var_fn25_calc_iq__return_dn2 = assign4400_e6259_d_n2;
        locals.var_fn25_calc_iq__return_dn3 = assign4400_e6259_d_n3;
        locals.var_fn25_calc_iq__return_dn4 = assign4400_e6259_d_n4;
        locals.var_fn25_calc_iq__return_dn7 = assign4400_e6259_d_n7;
        locals.var_fn25_calc_iq__return_dn16 = assign4400_e6259_d_n16;
        locals.var_fn25_calc_iq__return_dn17 = assign4400_e6259_d_n17;

        let (assign4410_e6263, assign4410_e6263_d_n2, assign4410_e6263_d_n3, assign4410_e6263_d_n4, assign4410_e6263_d_n7, assign4410_e6263_d_n16, assign4410_e6263_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__idsout, locals.var_fn25_calc_iq__idsout_dn2, locals.var_fn25_calc_iq__idsout_dn3, locals.var_fn25_calc_iq__idsout_dn4, locals.var_fn25_calc_iq__idsout_dn7, locals.var_fn25_calc_iq__idsout_dn16, locals.var_fn25_calc_iq__idsout_dn17,)
    } else {
        (locals.var_idsfp4, locals.var_idsfp4_dn2, locals.var_idsfp4_dn3, locals.var_idsfp4_dn4, locals.var_idsfp4_dn7, locals.var_idsfp4_dn16, locals.var_idsfp4_dn17,)
    }
};
        locals.var_idsfp4 = assign4410_e6263;
        locals.var_idsfp4_dn2 = assign4410_e6263_d_n2;
        locals.var_idsfp4_dn3 = assign4410_e6263_d_n3;
        locals.var_idsfp4_dn4 = assign4410_e6263_d_n4;
        locals.var_idsfp4_dn7 = assign4410_e6263_d_n7;
        locals.var_idsfp4_dn16 = assign4410_e6263_d_n16;
        locals.var_idsfp4_dn17 = assign4410_e6263_d_n17;

        let (assign4420_e6267, assign4420_e6267_d_n2, assign4420_e6267_d_n4, assign4420_e6267_d_n7, assign4420_e6267_d_n16, assign4420_e6267_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__qgsout, locals.var_fn25_calc_iq__qgsout_dn2, locals.var_fn25_calc_iq__qgsout_dn4, locals.var_fn25_calc_iq__qgsout_dn7, locals.var_fn25_calc_iq__qgsout_dn16, locals.var_fn25_calc_iq__qgsout_dn17,)
    } else {
        (locals.var_qgsfp4, locals.var_qgsfp4_dn2, locals.var_qgsfp4_dn4, locals.var_qgsfp4_dn7, locals.var_qgsfp4_dn16, locals.var_qgsfp4_dn17,)
    }
};
        locals.var_qgsfp4 = assign4420_e6267;
        locals.var_qgsfp4_dn2 = assign4420_e6267_d_n2;
        locals.var_qgsfp4_dn4 = assign4420_e6267_d_n4;
        locals.var_qgsfp4_dn7 = assign4420_e6267_d_n7;
        locals.var_qgsfp4_dn16 = assign4420_e6267_d_n16;
        locals.var_qgsfp4_dn17 = assign4420_e6267_d_n17;

        let (assign4430_e6271, assign4430_e6271_d_n2, assign4430_e6271_d_n4, assign4430_e6271_d_n7, assign4430_e6271_d_n16, assign4430_e6271_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__qgdout, locals.var_fn25_calc_iq__qgdout_dn2, locals.var_fn25_calc_iq__qgdout_dn4, locals.var_fn25_calc_iq__qgdout_dn7, locals.var_fn25_calc_iq__qgdout_dn16, locals.var_fn25_calc_iq__qgdout_dn17,)
    } else {
        (locals.var_qgdfp4, locals.var_qgdfp4_dn2, locals.var_qgdfp4_dn4, locals.var_qgdfp4_dn7, locals.var_qgdfp4_dn16, locals.var_qgdfp4_dn17,)
    }
};
        locals.var_qgdfp4 = assign4430_e6271;
        locals.var_qgdfp4_dn2 = assign4430_e6271_d_n2;
        locals.var_qgdfp4_dn4 = assign4430_e6271_d_n4;
        locals.var_qgdfp4_dn7 = assign4430_e6271_d_n7;
        locals.var_qgdfp4_dn16 = assign4430_e6271_d_n16;
        locals.var_qgdfp4_dn17 = assign4430_e6271_d_n17;

        let (assign4440_e6275, assign4440_e6275_d_n2, assign4440_e6275_d_n3, assign4440_e6275_d_n4, assign4440_e6275_d_n7, assign4440_e6275_d_n16, assign4440_e6275_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__qcout, locals.var_fn25_calc_iq__qcout_dn2, locals.var_fn25_calc_iq__qcout_dn3, locals.var_fn25_calc_iq__qcout_dn4, locals.var_fn25_calc_iq__qcout_dn7, locals.var_fn25_calc_iq__qcout_dn16, locals.var_fn25_calc_iq__qcout_dn17,)
    } else {
        (locals.var_qcfp4, locals.var_qcfp4_dn2, locals.var_qcfp4_dn3, locals.var_qcfp4_dn4, locals.var_qcfp4_dn7, locals.var_qcfp4_dn16, locals.var_qcfp4_dn17,)
    }
};
        locals.var_qcfp4 = assign4440_e6275;
        locals.var_qcfp4_dn2 = assign4440_e6275_d_n2;
        locals.var_qcfp4_dn3 = assign4440_e6275_d_n3;
        locals.var_qcfp4_dn4 = assign4440_e6275_d_n4;
        locals.var_qcfp4_dn7 = assign4440_e6275_d_n7;
        locals.var_qcfp4_dn16 = assign4440_e6275_d_n16;
        locals.var_qcfp4_dn17 = assign4440_e6275_d_n17;

        let (assign4450_e6279, assign4450_e6279_d_n2, assign4450_e6279_d_n3, assign4450_e6279_d_n4, assign4450_e6279_d_n7, assign4450_e6279_d_n16, assign4450_e6279_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__qbout, locals.var_fn25_calc_iq__qbout_dn2, locals.var_fn25_calc_iq__qbout_dn3, locals.var_fn25_calc_iq__qbout_dn4, locals.var_fn25_calc_iq__qbout_dn7, locals.var_fn25_calc_iq__qbout_dn16, locals.var_fn25_calc_iq__qbout_dn17,)
    } else {
        (locals.var_qbfp4, locals.var_qbfp4_dn2, locals.var_qbfp4_dn3, locals.var_qbfp4_dn4, locals.var_qbfp4_dn7, locals.var_qbfp4_dn16, locals.var_qbfp4_dn17,)
    }
};
        locals.var_qbfp4 = assign4450_e6279;
        locals.var_qbfp4_dn2 = assign4450_e6279_d_n2;
        locals.var_qbfp4_dn3 = assign4450_e6279_d_n3;
        locals.var_qbfp4_dn4 = assign4450_e6279_d_n4;
        locals.var_qbfp4_dn7 = assign4450_e6279_d_n7;
        locals.var_qbfp4_dn16 = assign4450_e6279_d_n16;
        locals.var_qbfp4_dn17 = assign4450_e6279_d_n17;

        let (assign4460_e6283, assign4460_e6283_d_n2, assign4460_e6283_d_n3, assign4460_e6283_d_n4, assign4460_e6283_d_n7, assign4460_e6283_d_n16, assign4460_e6283_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__qsout, locals.var_fn25_calc_iq__qsout_dn2, locals.var_fn25_calc_iq__qsout_dn3, locals.var_fn25_calc_iq__qsout_dn4, locals.var_fn25_calc_iq__qsout_dn7, locals.var_fn25_calc_iq__qsout_dn16, locals.var_fn25_calc_iq__qsout_dn17,)
    } else {
        (locals.var_qsfp4, locals.var_qsfp4_dn2, locals.var_qsfp4_dn3, locals.var_qsfp4_dn4, locals.var_qsfp4_dn7, locals.var_qsfp4_dn16, locals.var_qsfp4_dn17,)
    }
};
        locals.var_qsfp4 = assign4460_e6283;
        locals.var_qsfp4_dn2 = assign4460_e6283_d_n2;
        locals.var_qsfp4_dn3 = assign4460_e6283_d_n3;
        locals.var_qsfp4_dn4 = assign4460_e6283_d_n4;
        locals.var_qsfp4_dn7 = assign4460_e6283_d_n7;
        locals.var_qsfp4_dn16 = assign4460_e6283_d_n16;
        locals.var_qsfp4_dn17 = assign4460_e6283_d_n17;

        let (assign4490_e6295, assign4490_e6295_d_n2, assign4490_e6295_d_n3, assign4490_e6295_d_n4, assign4490_e6295_d_n7, assign4490_e6295_d_n16, assign4490_e6295_d_n17,) = {
    if (locals.var_guard24 != 0.0) {
        (locals.var_fn25_calc_iq__return, locals.var_fn25_calc_iq__return_dn2, locals.var_fn25_calc_iq__return_dn3, locals.var_fn25_calc_iq__return_dn4, locals.var_fn25_calc_iq__return_dn7, locals.var_fn25_calc_iq__return_dn16, locals.var_fn25_calc_iq__return_dn17,)
    } else {
        (locals.var_idsfp4, locals.var_idsfp4_dn2, locals.var_idsfp4_dn3, locals.var_idsfp4_dn4, locals.var_idsfp4_dn7, locals.var_idsfp4_dn16, locals.var_idsfp4_dn17,)
    }
};
        locals.var_idsfp4 = assign4490_e6295;
        locals.var_idsfp4_dn2 = assign4490_e6295_d_n2;
        locals.var_idsfp4_dn3 = assign4490_e6295_d_n3;
        locals.var_idsfp4_dn4 = assign4490_e6295_d_n4;
        locals.var_idsfp4_dn7 = assign4490_e6295_d_n7;
        locals.var_idsfp4_dn16 = assign4490_e6295_d_n16;
        locals.var_idsfp4_dn17 = assign4490_e6295_d_n17;

        let assign4500_e6298: f64 = if p.p232 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard59 = assign4500_e6298;

        locals.var_idsfp3 = 0.0;
        locals.var_idsfp3_dn2 = 0.0;
        locals.var_idsfp3_dn3 = 0.0;
        locals.var_idsfp3_dn4 = 0.0;
        locals.var_idsfp3_dn7 = 0.0;
        locals.var_idsfp3_dn15 = 0.0;
        locals.var_idsfp3_dn16 = 0.0;

        locals.var_qgsfp3 = 0.0;
        locals.var_qgsfp3_dn2 = 0.0;
        locals.var_qgsfp3_dn4 = 0.0;
        locals.var_qgsfp3_dn7 = 0.0;
        locals.var_qgsfp3_dn15 = 0.0;
        locals.var_qgsfp3_dn16 = 0.0;

        locals.var_qgdfp3 = 0.0;
        locals.var_qgdfp3_dn2 = 0.0;
        locals.var_qgdfp3_dn4 = 0.0;
        locals.var_qgdfp3_dn7 = 0.0;
        locals.var_qgdfp3_dn15 = 0.0;
        locals.var_qgdfp3_dn16 = 0.0;

        locals.var_qcfp3 = 0.0;
        locals.var_qcfp3_dn2 = 0.0;
        locals.var_qcfp3_dn3 = 0.0;
        locals.var_qcfp3_dn4 = 0.0;
        locals.var_qcfp3_dn7 = 0.0;
        locals.var_qcfp3_dn15 = 0.0;
        locals.var_qcfp3_dn16 = 0.0;

        locals.var_qbfp3 = 0.0;
        locals.var_qbfp3_dn2 = 0.0;
        locals.var_qbfp3_dn3 = 0.0;
        locals.var_qbfp3_dn4 = 0.0;
        locals.var_qbfp3_dn7 = 0.0;
        locals.var_qbfp3_dn15 = 0.0;
        locals.var_qbfp3_dn16 = 0.0;

        locals.var_qsfp3 = 0.0;
        locals.var_qsfp3_dn2 = 0.0;
        locals.var_qsfp3_dn3 = 0.0;
        locals.var_qsfp3_dn4 = 0.0;
        locals.var_qsfp3_dn7 = 0.0;
        locals.var_qsfp3_dn15 = 0.0;
        locals.var_qsfp3_dn16 = 0.0;

        let assign4590_e6309: f64 = if p.p211 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard60 = assign4590_e6309;

        let (assign4600_e6313, assign4600_e6313_d_n2, assign4600_e6313_d_n3, assign4600_e6313_d_n4, assign4600_e6313_d_n7, assign4600_e6313_d_n15, assign4600_e6313_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__return, locals.var_fn61_calc_iq__return_dn2, locals.var_fn61_calc_iq__return_dn3, locals.var_fn61_calc_iq__return_dn4, locals.var_fn61_calc_iq__return_dn7, locals.var_fn61_calc_iq__return_dn15, locals.var_fn61_calc_iq__return_dn16,)
    }
};
        locals.var_fn61_calc_iq__return = assign4600_e6313;
        locals.var_fn61_calc_iq__return_dn2 = assign4600_e6313_d_n2;
        locals.var_fn61_calc_iq__return_dn3 = assign4600_e6313_d_n3;
        locals.var_fn61_calc_iq__return_dn4 = assign4600_e6313_d_n4;
        locals.var_fn61_calc_iq__return_dn7 = assign4600_e6313_d_n7;
        locals.var_fn61_calc_iq__return_dn15 = assign4600_e6313_d_n15;
        locals.var_fn61_calc_iq__return_dn16 = assign4600_e6313_d_n16;

        let (assign4610_e6317, assign4610_e6317_d_n2, assign4610_e6317_d_n3, assign4610_e6317_d_n4, assign4610_e6317_d_n7, assign4610_e6317_d_n15, assign4610_e6317_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__idsout, locals.var_fn61_calc_iq__idsout_dn2, locals.var_fn61_calc_iq__idsout_dn3, locals.var_fn61_calc_iq__idsout_dn4, locals.var_fn61_calc_iq__idsout_dn7, locals.var_fn61_calc_iq__idsout_dn15, locals.var_fn61_calc_iq__idsout_dn16,)
    }
};
        locals.var_fn61_calc_iq__idsout = assign4610_e6317;
        locals.var_fn61_calc_iq__idsout_dn2 = assign4610_e6317_d_n2;
        locals.var_fn61_calc_iq__idsout_dn3 = assign4610_e6317_d_n3;
        locals.var_fn61_calc_iq__idsout_dn4 = assign4610_e6317_d_n4;
        locals.var_fn61_calc_iq__idsout_dn7 = assign4610_e6317_d_n7;
        locals.var_fn61_calc_iq__idsout_dn15 = assign4610_e6317_d_n15;
        locals.var_fn61_calc_iq__idsout_dn16 = assign4610_e6317_d_n16;

        let (assign4620_e6321, assign4620_e6321_d_n2, assign4620_e6321_d_n4, assign4620_e6321_d_n7, assign4620_e6321_d_n15, assign4620_e6321_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qgsout, locals.var_fn61_calc_iq__qgsout_dn2, locals.var_fn61_calc_iq__qgsout_dn4, locals.var_fn61_calc_iq__qgsout_dn7, locals.var_fn61_calc_iq__qgsout_dn15, locals.var_fn61_calc_iq__qgsout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qgsout = assign4620_e6321;
        locals.var_fn61_calc_iq__qgsout_dn2 = assign4620_e6321_d_n2;
        locals.var_fn61_calc_iq__qgsout_dn4 = assign4620_e6321_d_n4;
        locals.var_fn61_calc_iq__qgsout_dn7 = assign4620_e6321_d_n7;
        locals.var_fn61_calc_iq__qgsout_dn15 = assign4620_e6321_d_n15;
        locals.var_fn61_calc_iq__qgsout_dn16 = assign4620_e6321_d_n16;

        let (assign4630_e6325, assign4630_e6325_d_n2, assign4630_e6325_d_n4, assign4630_e6325_d_n7, assign4630_e6325_d_n15, assign4630_e6325_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qgdout, locals.var_fn61_calc_iq__qgdout_dn2, locals.var_fn61_calc_iq__qgdout_dn4, locals.var_fn61_calc_iq__qgdout_dn7, locals.var_fn61_calc_iq__qgdout_dn15, locals.var_fn61_calc_iq__qgdout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qgdout = assign4630_e6325;
        locals.var_fn61_calc_iq__qgdout_dn2 = assign4630_e6325_d_n2;
        locals.var_fn61_calc_iq__qgdout_dn4 = assign4630_e6325_d_n4;
        locals.var_fn61_calc_iq__qgdout_dn7 = assign4630_e6325_d_n7;
        locals.var_fn61_calc_iq__qgdout_dn15 = assign4630_e6325_d_n15;
        locals.var_fn61_calc_iq__qgdout_dn16 = assign4630_e6325_d_n16;

        let (assign4640_e6329, assign4640_e6329_d_n2, assign4640_e6329_d_n3, assign4640_e6329_d_n4, assign4640_e6329_d_n7, assign4640_e6329_d_n15, assign4640_e6329_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qcout, locals.var_fn61_calc_iq__qcout_dn2, locals.var_fn61_calc_iq__qcout_dn3, locals.var_fn61_calc_iq__qcout_dn4, locals.var_fn61_calc_iq__qcout_dn7, locals.var_fn61_calc_iq__qcout_dn15, locals.var_fn61_calc_iq__qcout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qcout = assign4640_e6329;
        locals.var_fn61_calc_iq__qcout_dn2 = assign4640_e6329_d_n2;
        locals.var_fn61_calc_iq__qcout_dn3 = assign4640_e6329_d_n3;
        locals.var_fn61_calc_iq__qcout_dn4 = assign4640_e6329_d_n4;
        locals.var_fn61_calc_iq__qcout_dn7 = assign4640_e6329_d_n7;
        locals.var_fn61_calc_iq__qcout_dn15 = assign4640_e6329_d_n15;
        locals.var_fn61_calc_iq__qcout_dn16 = assign4640_e6329_d_n16;

        let (assign4650_e6333, assign4650_e6333_d_n2, assign4650_e6333_d_n3, assign4650_e6333_d_n4, assign4650_e6333_d_n7, assign4650_e6333_d_n15, assign4650_e6333_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qbout, locals.var_fn61_calc_iq__qbout_dn2, locals.var_fn61_calc_iq__qbout_dn3, locals.var_fn61_calc_iq__qbout_dn4, locals.var_fn61_calc_iq__qbout_dn7, locals.var_fn61_calc_iq__qbout_dn15, locals.var_fn61_calc_iq__qbout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qbout = assign4650_e6333;
        locals.var_fn61_calc_iq__qbout_dn2 = assign4650_e6333_d_n2;
        locals.var_fn61_calc_iq__qbout_dn3 = assign4650_e6333_d_n3;
        locals.var_fn61_calc_iq__qbout_dn4 = assign4650_e6333_d_n4;
        locals.var_fn61_calc_iq__qbout_dn7 = assign4650_e6333_d_n7;
        locals.var_fn61_calc_iq__qbout_dn15 = assign4650_e6333_d_n15;
        locals.var_fn61_calc_iq__qbout_dn16 = assign4650_e6333_d_n16;

        let (assign4660_e6337, assign4660_e6337_d_n2, assign4660_e6337_d_n3, assign4660_e6337_d_n4, assign4660_e6337_d_n7, assign4660_e6337_d_n15, assign4660_e6337_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qsout, locals.var_fn61_calc_iq__qsout_dn2, locals.var_fn61_calc_iq__qsout_dn3, locals.var_fn61_calc_iq__qsout_dn4, locals.var_fn61_calc_iq__qsout_dn7, locals.var_fn61_calc_iq__qsout_dn15, locals.var_fn61_calc_iq__qsout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qsout = assign4660_e6337;
        locals.var_fn61_calc_iq__qsout_dn2 = assign4660_e6337_d_n2;
        locals.var_fn61_calc_iq__qsout_dn3 = assign4660_e6337_d_n3;
        locals.var_fn61_calc_iq__qsout_dn4 = assign4660_e6337_d_n4;
        locals.var_fn61_calc_iq__qsout_dn7 = assign4660_e6337_d_n7;
        locals.var_fn61_calc_iq__qsout_dn15 = assign4660_e6337_d_n15;
        locals.var_fn61_calc_iq__qsout_dn16 = assign4660_e6337_d_n16;

        let (assign4670_e6341, assign4670_e6341_d_n4, assign4670_e6341_d_n15, assign4670_e6341_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vtdibl, locals.var_fn61_calc_iq__vtdibl_dn4, locals.var_fn61_calc_iq__vtdibl_dn15, locals.var_fn61_calc_iq__vtdibl_dn16,)
    }
};
        locals.var_fn61_calc_iq__vtdibl = assign4670_e6341;
        locals.var_fn61_calc_iq__vtdibl_dn4 = assign4670_e6341_d_n4;
        locals.var_fn61_calc_iq__vtdibl_dn15 = assign4670_e6341_d_n15;
        locals.var_fn61_calc_iq__vtdibl_dn16 = assign4670_e6341_d_n16;

        let (assign4680_e6345, assign4680_e6345_d_n2, assign4680_e6345_d_n3, assign4680_e6345_d_n4, assign4680_e6345_d_n7, assign4680_e6345_d_n15, assign4680_e6345_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsat1, locals.var_fn61_calc_iq__vdsat1_dn2, locals.var_fn61_calc_iq__vdsat1_dn3, locals.var_fn61_calc_iq__vdsat1_dn4, locals.var_fn61_calc_iq__vdsat1_dn7, locals.var_fn61_calc_iq__vdsat1_dn15, locals.var_fn61_calc_iq__vdsat1_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsat1 = assign4680_e6345;
        locals.var_fn61_calc_iq__vdsat1_dn2 = assign4680_e6345_d_n2;
        locals.var_fn61_calc_iq__vdsat1_dn3 = assign4680_e6345_d_n3;
        locals.var_fn61_calc_iq__vdsat1_dn4 = assign4680_e6345_d_n4;
        locals.var_fn61_calc_iq__vdsat1_dn7 = assign4680_e6345_d_n7;
        locals.var_fn61_calc_iq__vdsat1_dn15 = assign4680_e6345_d_n15;
        locals.var_fn61_calc_iq__vdsat1_dn16 = assign4680_e6345_d_n16;

        let (assign4690_e6349, assign4690_e6349_d_n2, assign4690_e6349_d_n7, assign4690_e6349_d_n15,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_vgsfp3, locals.var_vgsfp3_dn2, locals.var_vgsfp3_dn7, locals.var_vgsfp3_dn15,)
    } else {
        (locals.var_fn61_calc_iq__vgsin, locals.var_fn61_calc_iq__vgsin_dn2, locals.var_fn61_calc_iq__vgsin_dn7, locals.var_fn61_calc_iq__vgsin_dn15,)
    }
};
        locals.var_fn61_calc_iq__vgsin = assign4690_e6349;
        locals.var_fn61_calc_iq__vgsin_dn2 = assign4690_e6349_d_n2;
        locals.var_fn61_calc_iq__vgsin_dn7 = assign4690_e6349_d_n7;
        locals.var_fn61_calc_iq__vgsin_dn15 = assign4690_e6349_d_n15;

        let (assign4700_e6353, assign4700_e6353_d_n15, assign4700_e6353_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_vdsfp3, locals.var_vdsfp3_dn15, locals.var_vdsfp3_dn16,)
    } else {
        (locals.var_fn61_calc_iq__vdsin, locals.var_fn61_calc_iq__vdsin_dn15, locals.var_fn61_calc_iq__vdsin_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsin = assign4700_e6353;
        locals.var_fn61_calc_iq__vdsin_dn15 = assign4700_e6353_d_n15;
        locals.var_fn61_calc_iq__vdsin_dn16 = assign4700_e6353_d_n16;

        let (assign4710_e6357,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p217,)
    } else {
        (locals.var_fn61_calc_iq__qcbflag,)
    }
};
        locals.var_fn61_calc_iq__qcbflag = assign4710_e6357;

        let (assign4720_e6361, assign4720_e6361_d_n2, assign4720_e6361_d_n7, assign4720_e6361_d_n15,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_vcfp3, locals.var_vcfp3_dn2, locals.var_vcfp3_dn7, locals.var_vcfp3_dn15,)
    } else {
        (locals.var_fn61_calc_iq__vcin, locals.var_fn61_calc_iq__vcin_dn2, locals.var_fn61_calc_iq__vcin_dn7, locals.var_fn61_calc_iq__vcin_dn15,)
    }
};
        locals.var_fn61_calc_iq__vcin = assign4720_e6361;
        locals.var_fn61_calc_iq__vcin_dn2 = assign4720_e6361_d_n2;
        locals.var_fn61_calc_iq__vcin_dn7 = assign4720_e6361_d_n7;
        locals.var_fn61_calc_iq__vcin_dn15 = assign4720_e6361_d_n15;

        let (assign4730_e6365, assign4730_e6365_d_n3, assign4730_e6365_d_n15,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_vbfp3, locals.var_vbfp3_dn3, locals.var_vbfp3_dn15,)
    } else {
        (locals.var_fn61_calc_iq__vbin, locals.var_fn61_calc_iq__vbin_dn3, locals.var_fn61_calc_iq__vbin_dn15,)
    }
};
        locals.var_fn61_calc_iq__vbin = assign4730_e6365;
        locals.var_fn61_calc_iq__vbin_dn3 = assign4730_e6365_d_n3;
        locals.var_fn61_calc_iq__vbin_dn15 = assign4730_e6365_d_n15;

        let (assign4740_e6369,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p215,)
    } else {
        (locals.var_fn61_calc_iq__qgsflag,)
    }
};
        locals.var_fn61_calc_iq__qgsflag = assign4740_e6369;

        let (assign4750_e6373, assign4750_e6373_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn61_calc_iq__tambin, locals.var_fn61_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn61_calc_iq__tambin = assign4750_e6373;
        locals.var_fn61_calc_iq__tambin_dn4 = assign4750_e6373_d_n4;

        let (assign4760_e6377,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn61_calc_iq__tnomin,)
    }
};
        locals.var_fn61_calc_iq__tnomin = assign4760_e6377;

        let (assign4770_e6381, assign4770_e6381_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn61_calc_iq__phitin, locals.var_fn61_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn61_calc_iq__phitin = assign4770_e6381;
        locals.var_fn61_calc_iq__phitin_dn4 = assign4770_e6381_d_n4;

        let (assign4780_e6385,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn61_calc_iq__w,)
    }
};
        locals.var_fn61_calc_iq__w = assign4780_e6385;

        let (assign4790_e6389,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p211,)
    } else {
        (locals.var_fn61_calc_iq__lin,)
    }
};
        locals.var_fn61_calc_iq__lin = assign4790_e6389;

        let (assign4800_e6393, assign4800_e6393_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_cgfp3t, locals.var_cgfp3t_dn4,)
    } else {
        (locals.var_fn61_calc_iq__cgin, locals.var_fn61_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn61_calc_iq__cgin = assign4800_e6393;
        locals.var_fn61_calc_iq__cgin_dn4 = assign4800_e6393_d_n4;

        let (assign4810_e6397,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p216,)
    } else {
        (locals.var_fn61_calc_iq__cs,)
    }
};
        locals.var_fn61_calc_iq__cs = assign4810_e6397;

        let (assign4820_e6401, assign4820_e6401_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_ccfp3t, locals.var_ccfp3t_dn4,)
    } else {
        (locals.var_fn61_calc_iq__cc, locals.var_fn61_calc_iq__cc_dn4,)
    }
};
        locals.var_fn61_calc_iq__cc = assign4820_e6401;
        locals.var_fn61_calc_iq__cc_dn4 = assign4820_e6401_d_n4;

        let (assign4830_e6405, assign4830_e6405_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_cbfp3t, locals.var_cbfp3t_dn4,)
    } else {
        (locals.var_fn61_calc_iq__cb, locals.var_fn61_calc_iq__cb_dn4,)
    }
};
        locals.var_fn61_calc_iq__cb = assign4830_e6405;
        locals.var_fn61_calc_iq__cb_dn4 = assign4830_e6405_d_n4;

    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4840_e6409,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p212,)
    } else {
        (locals.var_fn61_calc_iq__vto,)
    }
};
        locals.var_fn61_calc_iq__vto = assign4840_e6409;

        let (assign4850_e6413,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p226,)
    } else {
        (locals.var_fn61_calc_iq__ss,)
    }
};
        locals.var_fn61_calc_iq__ss = assign4850_e6413;

        let (assign4860_e6417,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p225,)
    } else {
        (locals.var_fn61_calc_iq__delta1,)
    }
};
        locals.var_fn61_calc_iq__delta1 = assign4860_e6417;

        let (assign4870_e6421,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn61_calc_iq__delta2,)
    }
};
        locals.var_fn61_calc_iq__delta2 = assign4870_e6421;

        let (assign4880_e6425,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p227,)
    } else {
        (locals.var_fn61_calc_iq__nd,)
    }
};
        locals.var_fn61_calc_iq__nd = assign4880_e6425;

        let (assign4890_e6429,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p231,)
    } else {
        (locals.var_fn61_calc_iq__alpha,)
    }
};
        locals.var_fn61_calc_iq__alpha = assign4890_e6429;

        let (assign4900_e6433,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p222,)
    } else {
        (locals.var_fn61_calc_iq__vel0,)
    }
};
        locals.var_fn61_calc_iq__vel0 = assign4900_e6433;

        let (assign4910_e6437,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p223,)
    } else {
        (locals.var_fn61_calc_iq__mu0,)
    }
};
        locals.var_fn61_calc_iq__mu0 = assign4910_e6437;

        let (assign4920_e6441,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p224,)
    } else {
        (locals.var_fn61_calc_iq__beta,)
    }
};
        locals.var_fn61_calc_iq__beta = assign4920_e6441;

        let (assign4930_e6445,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p230,)
    } else {
        (locals.var_fn61_calc_iq__mtheta,)
    }
};
        locals.var_fn61_calc_iq__mtheta = assign4930_e6445;

        let (assign4940_e6449,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p229,)
    } else {
        (locals.var_fn61_calc_iq__vtheta,)
    }
};
        locals.var_fn61_calc_iq__vtheta = assign4940_e6449;

        let (assign4950_e6453,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p228,)
    } else {
        (locals.var_fn61_calc_iq__vtzeta,)
    }
};
        locals.var_fn61_calc_iq__vtzeta = assign4950_e6453;

        let (assign4960_e6457,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn61_calc_iq__dibsat,)
    }
};
        locals.var_fn61_calc_iq__dibsat = assign4960_e6457;

        let (assign4970_e6461,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn61_calc_iq__epsilon,)
    }
};
        locals.var_fn61_calc_iq__epsilon = assign4970_e6461;

        let (assign4980_e6465,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn61_calc_iq__vzeta,)
    }
};
        locals.var_fn61_calc_iq__vzeta = assign4980_e6465;

        let (assign4990_e6469,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn61_calc_iq__lambda,)
    }
};
        locals.var_fn61_calc_iq__lambda = assign4990_e6469;

        let (assign5000_e6473,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn61_calc_iq__ngf,)
    }
};
        locals.var_fn61_calc_iq__ngf = assign5000_e6473;

        let (assign5010_e6477,) = {
    if (locals.var_guard60 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn61_calc_iq__type,)
    }
};
        locals.var_fn61_calc_iq__type = assign5010_e6477;

        let (assign5020_e6481,) = {
    if (locals.var_guard60 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn61_calc_iq__trapfracdl,)
    }
};
        locals.var_fn61_calc_iq__trapfracdl = assign5020_e6481;

        let (assign5030_e6485, assign5030_e6485_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__alpha_phit, locals.var_fn61_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn61_calc_iq__alpha_phit = assign5030_e6485;
        locals.var_fn61_calc_iq__alpha_phit_dn4 = assign5030_e6485_d_n4;

        let (assign5040_e6489, assign5040_e6489_d_n15, assign5040_e6489_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__delta, locals.var_fn61_calc_iq__delta_dn15, locals.var_fn61_calc_iq__delta_dn16,)
    }
};
        locals.var_fn61_calc_iq__delta = assign5040_e6489;
        locals.var_fn61_calc_iq__delta_dn15 = assign5040_e6489_d_n15;
        locals.var_fn61_calc_iq__delta_dn16 = assign5040_e6489_d_n16;

        let (assign5050_e6493, assign5050_e6493_d_n4, assign5050_e6493_d_n15, assign5050_e6493_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__n, locals.var_fn61_calc_iq__n_dn4, locals.var_fn61_calc_iq__n_dn15, locals.var_fn61_calc_iq__n_dn16,)
    }
};
        locals.var_fn61_calc_iq__n = assign5050_e6493;
        locals.var_fn61_calc_iq__n_dn4 = assign5050_e6493_d_n4;
        locals.var_fn61_calc_iq__n_dn15 = assign5050_e6493_d_n15;
        locals.var_fn61_calc_iq__n_dn16 = assign5050_e6493_d_n16;

        let (assign5060_e6497, assign5060_e6497_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vtof, locals.var_fn61_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn61_calc_iq__vtof = assign5060_e6497;
        locals.var_fn61_calc_iq__vtof_dn4 = assign5060_e6497_d_n4;

        let (assign5070_e6501, assign5070_e6501_d_n15, assign5070_e6501_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vsatdibl, locals.var_fn61_calc_iq__vsatdibl_dn15, locals.var_fn61_calc_iq__vsatdibl_dn16,)
    }
};
        locals.var_fn61_calc_iq__vsatdibl = assign5070_e6501;
        locals.var_fn61_calc_iq__vsatdibl_dn15 = assign5070_e6501_d_n15;
        locals.var_fn61_calc_iq__vsatdibl_dn16 = assign5070_e6501_d_n16;

        let (assign5080_e6505, assign5080_e6505_d_n2, assign5080_e6505_d_n3, assign5080_e6505_d_n4, assign5080_e6505_d_n7, assign5080_e6505_d_n15, assign5080_e6505_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffs, locals.var_fn61_calc_iq__ffs_dn2, locals.var_fn61_calc_iq__ffs_dn3, locals.var_fn61_calc_iq__ffs_dn4, locals.var_fn61_calc_iq__ffs_dn7, locals.var_fn61_calc_iq__ffs_dn15, locals.var_fn61_calc_iq__ffs_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffs = assign5080_e6505;
        locals.var_fn61_calc_iq__ffs_dn2 = assign5080_e6505_d_n2;
        locals.var_fn61_calc_iq__ffs_dn3 = assign5080_e6505_d_n3;
        locals.var_fn61_calc_iq__ffs_dn4 = assign5080_e6505_d_n4;
        locals.var_fn61_calc_iq__ffs_dn7 = assign5080_e6505_d_n7;
        locals.var_fn61_calc_iq__ffs_dn15 = assign5080_e6505_d_n15;
        locals.var_fn61_calc_iq__ffs_dn16 = assign5080_e6505_d_n16;

        let (assign5090_e6509, assign5090_e6509_d_n4, assign5090_e6509_d_n15, assign5090_e6509_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__two_n_phit, locals.var_fn61_calc_iq__two_n_phit_dn4, locals.var_fn61_calc_iq__two_n_phit_dn15, locals.var_fn61_calc_iq__two_n_phit_dn16,)
    }
};
        locals.var_fn61_calc_iq__two_n_phit = assign5090_e6509;
        locals.var_fn61_calc_iq__two_n_phit_dn4 = assign5090_e6509_d_n4;
        locals.var_fn61_calc_iq__two_n_phit_dn15 = assign5090_e6509_d_n15;
        locals.var_fn61_calc_iq__two_n_phit_dn16 = assign5090_e6509_d_n16;

        let (assign5100_e6513, assign5100_e6513_d_n4, assign5100_e6513_d_n15, assign5100_e6513_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qref, locals.var_fn61_calc_iq__qref_dn4, locals.var_fn61_calc_iq__qref_dn15, locals.var_fn61_calc_iq__qref_dn16,)
    }
};
        locals.var_fn61_calc_iq__qref = assign5100_e6513;
        locals.var_fn61_calc_iq__qref_dn4 = assign5100_e6513_d_n4;
        locals.var_fn61_calc_iq__qref_dn15 = assign5100_e6513_d_n15;
        locals.var_fn61_calc_iq__qref_dn16 = assign5100_e6513_d_n16;

        let (assign5110_e6517, assign5110_e6517_d_n2, assign5110_e6517_d_n3, assign5110_e6517_d_n4, assign5110_e6517_d_n7, assign5110_e6517_d_n15, assign5110_e6517_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__etas, locals.var_fn61_calc_iq__etas_dn2, locals.var_fn61_calc_iq__etas_dn3, locals.var_fn61_calc_iq__etas_dn4, locals.var_fn61_calc_iq__etas_dn7, locals.var_fn61_calc_iq__etas_dn15, locals.var_fn61_calc_iq__etas_dn16,)
    }
};
        locals.var_fn61_calc_iq__etas = assign5110_e6517;
        locals.var_fn61_calc_iq__etas_dn2 = assign5110_e6517_d_n2;
        locals.var_fn61_calc_iq__etas_dn3 = assign5110_e6517_d_n3;
        locals.var_fn61_calc_iq__etas_dn4 = assign5110_e6517_d_n4;
        locals.var_fn61_calc_iq__etas_dn7 = assign5110_e6517_d_n7;
        locals.var_fn61_calc_iq__etas_dn15 = assign5110_e6517_d_n15;
        locals.var_fn61_calc_iq__etas_dn16 = assign5110_e6517_d_n16;

        let (assign5120_e6521, assign5120_e6521_d_n2, assign5120_e6521_d_n3, assign5120_e6521_d_n4, assign5120_e6521_d_n7, assign5120_e6521_d_n15, assign5120_e6521_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qinvs, locals.var_fn61_calc_iq__qinvs_dn2, locals.var_fn61_calc_iq__qinvs_dn3, locals.var_fn61_calc_iq__qinvs_dn4, locals.var_fn61_calc_iq__qinvs_dn7, locals.var_fn61_calc_iq__qinvs_dn15, locals.var_fn61_calc_iq__qinvs_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvs = assign5120_e6521;
        locals.var_fn61_calc_iq__qinvs_dn2 = assign5120_e6521_d_n2;
        locals.var_fn61_calc_iq__qinvs_dn3 = assign5120_e6521_d_n3;
        locals.var_fn61_calc_iq__qinvs_dn4 = assign5120_e6521_d_n4;
        locals.var_fn61_calc_iq__qinvs_dn7 = assign5120_e6521_d_n7;
        locals.var_fn61_calc_iq__qinvs_dn15 = assign5120_e6521_d_n15;
        locals.var_fn61_calc_iq__qinvs_dn16 = assign5120_e6521_d_n16;

        let (assign5130_e6525, assign5130_e6525_d_n2, assign5130_e6525_d_n3, assign5130_e6525_d_n4, assign5130_e6525_d_n7, assign5130_e6525_d_n15, assign5130_e6525_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__muf, locals.var_fn61_calc_iq__muf_dn2, locals.var_fn61_calc_iq__muf_dn3, locals.var_fn61_calc_iq__muf_dn4, locals.var_fn61_calc_iq__muf_dn7, locals.var_fn61_calc_iq__muf_dn15, locals.var_fn61_calc_iq__muf_dn16,)
    }
};
        locals.var_fn61_calc_iq__muf = assign5130_e6525;
        locals.var_fn61_calc_iq__muf_dn2 = assign5130_e6525_d_n2;
        locals.var_fn61_calc_iq__muf_dn3 = assign5130_e6525_d_n3;
        locals.var_fn61_calc_iq__muf_dn4 = assign5130_e6525_d_n4;
        locals.var_fn61_calc_iq__muf_dn7 = assign5130_e6525_d_n7;
        locals.var_fn61_calc_iq__muf_dn15 = assign5130_e6525_d_n15;
        locals.var_fn61_calc_iq__muf_dn16 = assign5130_e6525_d_n16;

        let (assign5140_e6529, assign5140_e6529_d_n2, assign5140_e6529_d_n3, assign5140_e6529_d_n4, assign5140_e6529_d_n7, assign5140_e6529_d_n15, assign5140_e6529_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vx, locals.var_fn61_calc_iq__vx_dn2, locals.var_fn61_calc_iq__vx_dn3, locals.var_fn61_calc_iq__vx_dn4, locals.var_fn61_calc_iq__vx_dn7, locals.var_fn61_calc_iq__vx_dn15, locals.var_fn61_calc_iq__vx_dn16,)
    }
};
        locals.var_fn61_calc_iq__vx = assign5140_e6529;
        locals.var_fn61_calc_iq__vx_dn2 = assign5140_e6529_d_n2;
        locals.var_fn61_calc_iq__vx_dn3 = assign5140_e6529_d_n3;
        locals.var_fn61_calc_iq__vx_dn4 = assign5140_e6529_d_n4;
        locals.var_fn61_calc_iq__vx_dn7 = assign5140_e6529_d_n7;
        locals.var_fn61_calc_iq__vx_dn15 = assign5140_e6529_d_n15;
        locals.var_fn61_calc_iq__vx_dn16 = assign5140_e6529_d_n16;

        let (assign5150_e6533, assign5150_e6533_d_n2, assign5150_e6533_d_n3, assign5150_e6533_d_n4, assign5150_e6533_d_n7, assign5150_e6533_d_n15, assign5150_e6533_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vxf, locals.var_fn61_calc_iq__vxf_dn2, locals.var_fn61_calc_iq__vxf_dn3, locals.var_fn61_calc_iq__vxf_dn4, locals.var_fn61_calc_iq__vxf_dn7, locals.var_fn61_calc_iq__vxf_dn15, locals.var_fn61_calc_iq__vxf_dn16,)
    }
};
        locals.var_fn61_calc_iq__vxf = assign5150_e6533;
        locals.var_fn61_calc_iq__vxf_dn2 = assign5150_e6533_d_n2;
        locals.var_fn61_calc_iq__vxf_dn3 = assign5150_e6533_d_n3;
        locals.var_fn61_calc_iq__vxf_dn4 = assign5150_e6533_d_n4;
        locals.var_fn61_calc_iq__vxf_dn7 = assign5150_e6533_d_n7;
        locals.var_fn61_calc_iq__vxf_dn15 = assign5150_e6533_d_n15;
        locals.var_fn61_calc_iq__vxf_dn16 = assign5150_e6533_d_n16;

        let (assign5160_e6537, assign5160_e6537_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__n0, locals.var_fn61_calc_iq__n0_dn4,)
    }
};
        locals.var_fn61_calc_iq__n0 = assign5160_e6537;
        locals.var_fn61_calc_iq__n0_dn4 = assign5160_e6537_d_n4;

        let (assign5170_e6541, assign5170_e6541_d_n2, assign5170_e6541_d_n4, assign5170_e6541_d_n7, assign5170_e6541_d_n15, assign5170_e6541_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffs0, locals.var_fn61_calc_iq__ffs0_dn2, locals.var_fn61_calc_iq__ffs0_dn4, locals.var_fn61_calc_iq__ffs0_dn7, locals.var_fn61_calc_iq__ffs0_dn15, locals.var_fn61_calc_iq__ffs0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffs0 = assign5170_e6541;
        locals.var_fn61_calc_iq__ffs0_dn2 = assign5170_e6541_d_n2;
        locals.var_fn61_calc_iq__ffs0_dn4 = assign5170_e6541_d_n4;
        locals.var_fn61_calc_iq__ffs0_dn7 = assign5170_e6541_d_n7;
        locals.var_fn61_calc_iq__ffs0_dn15 = assign5170_e6541_d_n15;
        locals.var_fn61_calc_iq__ffs0_dn16 = assign5170_e6541_d_n16;

        let (assign5180_e6545, assign5180_e6545_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__two_n_phit0, locals.var_fn61_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn61_calc_iq__two_n_phit0 = assign5180_e6545;
        locals.var_fn61_calc_iq__two_n_phit0_dn4 = assign5180_e6545_d_n4;

        let (assign5190_e6549, assign5190_e6549_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qref0, locals.var_fn61_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn61_calc_iq__qref0 = assign5190_e6549;
        locals.var_fn61_calc_iq__qref0_dn4 = assign5190_e6549_d_n4;

        let (assign5200_e6553, assign5200_e6553_d_n2, assign5200_e6553_d_n4, assign5200_e6553_d_n7, assign5200_e6553_d_n15, assign5200_e6553_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__etas0, locals.var_fn61_calc_iq__etas0_dn2, locals.var_fn61_calc_iq__etas0_dn4, locals.var_fn61_calc_iq__etas0_dn7, locals.var_fn61_calc_iq__etas0_dn15, locals.var_fn61_calc_iq__etas0_dn16,)
    }
};
        locals.var_fn61_calc_iq__etas0 = assign5200_e6553;
        locals.var_fn61_calc_iq__etas0_dn2 = assign5200_e6553_d_n2;
        locals.var_fn61_calc_iq__etas0_dn4 = assign5200_e6553_d_n4;
        locals.var_fn61_calc_iq__etas0_dn7 = assign5200_e6553_d_n7;
        locals.var_fn61_calc_iq__etas0_dn15 = assign5200_e6553_d_n15;
        locals.var_fn61_calc_iq__etas0_dn16 = assign5200_e6553_d_n16;

        let (assign5210_e6557, assign5210_e6557_d_n2, assign5210_e6557_d_n4, assign5210_e6557_d_n7, assign5210_e6557_d_n15, assign5210_e6557_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qinvs0, locals.var_fn61_calc_iq__qinvs0_dn2, locals.var_fn61_calc_iq__qinvs0_dn4, locals.var_fn61_calc_iq__qinvs0_dn7, locals.var_fn61_calc_iq__qinvs0_dn15, locals.var_fn61_calc_iq__qinvs0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvs0 = assign5210_e6557;
        locals.var_fn61_calc_iq__qinvs0_dn2 = assign5210_e6557_d_n2;
        locals.var_fn61_calc_iq__qinvs0_dn4 = assign5210_e6557_d_n4;
        locals.var_fn61_calc_iq__qinvs0_dn7 = assign5210_e6557_d_n7;
        locals.var_fn61_calc_iq__qinvs0_dn15 = assign5210_e6557_d_n15;
        locals.var_fn61_calc_iq__qinvs0_dn16 = assign5210_e6557_d_n16;

        let (assign5220_e6561, assign5220_e6561_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__muf0, locals.var_fn61_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn61_calc_iq__muf0 = assign5220_e6561;
        locals.var_fn61_calc_iq__muf0_dn4 = assign5220_e6561_d_n4;

        let (assign5230_e6565, assign5230_e6565_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vx0, locals.var_fn61_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn61_calc_iq__vx0 = assign5230_e6565;
        locals.var_fn61_calc_iq__vx0_dn4 = assign5230_e6565_d_n4;

        let (assign5240_e6569, assign5240_e6569_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__tfacmobin, locals.var_fn61_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn61_calc_iq__tfacmobin = assign5240_e6569;
        locals.var_fn61_calc_iq__tfacmobin_dn4 = assign5240_e6569_d_n4;

        let (assign5250_e6573, assign5250_e6573_d_n2, assign5250_e6573_d_n3, assign5250_e6573_d_n4, assign5250_e6573_d_n7, assign5250_e6573_d_n15, assign5250_e6573_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ff, locals.var_fn61_calc_iq__ff_dn2, locals.var_fn61_calc_iq__ff_dn3, locals.var_fn61_calc_iq__ff_dn4, locals.var_fn61_calc_iq__ff_dn7, locals.var_fn61_calc_iq__ff_dn15, locals.var_fn61_calc_iq__ff_dn16,)
    }
};
        locals.var_fn61_calc_iq__ff = assign5250_e6573;
        locals.var_fn61_calc_iq__ff_dn2 = assign5250_e6573_d_n2;
        locals.var_fn61_calc_iq__ff_dn3 = assign5250_e6573_d_n3;
        locals.var_fn61_calc_iq__ff_dn4 = assign5250_e6573_d_n4;
        locals.var_fn61_calc_iq__ff_dn7 = assign5250_e6573_d_n7;
        locals.var_fn61_calc_iq__ff_dn15 = assign5250_e6573_d_n15;
        locals.var_fn61_calc_iq__ff_dn16 = assign5250_e6573_d_n16;

        let (assign5260_e6577, assign5260_e6577_d_n2, assign5260_e6577_d_n3, assign5260_e6577_d_n4, assign5260_e6577_d_n7, assign5260_e6577_d_n15, assign5260_e6577_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__eta, locals.var_fn61_calc_iq__eta_dn2, locals.var_fn61_calc_iq__eta_dn3, locals.var_fn61_calc_iq__eta_dn4, locals.var_fn61_calc_iq__eta_dn7, locals.var_fn61_calc_iq__eta_dn15, locals.var_fn61_calc_iq__eta_dn16,)
    }
};
        locals.var_fn61_calc_iq__eta = assign5260_e6577;
        locals.var_fn61_calc_iq__eta_dn2 = assign5260_e6577_d_n2;
        locals.var_fn61_calc_iq__eta_dn3 = assign5260_e6577_d_n3;
        locals.var_fn61_calc_iq__eta_dn4 = assign5260_e6577_d_n4;
        locals.var_fn61_calc_iq__eta_dn7 = assign5260_e6577_d_n7;
        locals.var_fn61_calc_iq__eta_dn15 = assign5260_e6577_d_n15;
        locals.var_fn61_calc_iq__eta_dn16 = assign5260_e6577_d_n16;

        let (assign5270_e6581, assign5270_e6581_d_n2, assign5270_e6581_d_n3, assign5270_e6581_d_n4, assign5270_e6581_d_n7, assign5270_e6581_d_n15, assign5270_e6581_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qinvv, locals.var_fn61_calc_iq__qinvv_dn2, locals.var_fn61_calc_iq__qinvv_dn3, locals.var_fn61_calc_iq__qinvv_dn4, locals.var_fn61_calc_iq__qinvv_dn7, locals.var_fn61_calc_iq__qinvv_dn15, locals.var_fn61_calc_iq__qinvv_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvv = assign5270_e6581;
        locals.var_fn61_calc_iq__qinvv_dn2 = assign5270_e6581_d_n2;
        locals.var_fn61_calc_iq__qinvv_dn3 = assign5270_e6581_d_n3;
        locals.var_fn61_calc_iq__qinvv_dn4 = assign5270_e6581_d_n4;
        locals.var_fn61_calc_iq__qinvv_dn7 = assign5270_e6581_d_n7;
        locals.var_fn61_calc_iq__qinvv_dn15 = assign5270_e6581_d_n15;
        locals.var_fn61_calc_iq__qinvv_dn16 = assign5270_e6581_d_n16;

        let (assign5280_e6585, assign5280_e6585_d_n2, assign5280_e6585_d_n4, assign5280_e6585_d_n7, assign5280_e6585_d_n15, assign5280_e6585_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ff0, locals.var_fn61_calc_iq__ff0_dn2, locals.var_fn61_calc_iq__ff0_dn4, locals.var_fn61_calc_iq__ff0_dn7, locals.var_fn61_calc_iq__ff0_dn15, locals.var_fn61_calc_iq__ff0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ff0 = assign5280_e6585;
        locals.var_fn61_calc_iq__ff0_dn2 = assign5280_e6585_d_n2;
        locals.var_fn61_calc_iq__ff0_dn4 = assign5280_e6585_d_n4;
        locals.var_fn61_calc_iq__ff0_dn7 = assign5280_e6585_d_n7;
        locals.var_fn61_calc_iq__ff0_dn15 = assign5280_e6585_d_n15;
        locals.var_fn61_calc_iq__ff0_dn16 = assign5280_e6585_d_n16;

    }

    pub(super) fn stamp_transient_block_13(
        locals: &mut StampLocals,
    ) {
        let (assign5290_e6589, assign5290_e6589_d_n2, assign5290_e6589_d_n4, assign5290_e6589_d_n7, assign5290_e6589_d_n15, assign5290_e6589_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__eta0, locals.var_fn61_calc_iq__eta0_dn2, locals.var_fn61_calc_iq__eta0_dn4, locals.var_fn61_calc_iq__eta0_dn7, locals.var_fn61_calc_iq__eta0_dn15, locals.var_fn61_calc_iq__eta0_dn16,)
    }
};
        locals.var_fn61_calc_iq__eta0 = assign5290_e6589;
        locals.var_fn61_calc_iq__eta0_dn2 = assign5290_e6589_d_n2;
        locals.var_fn61_calc_iq__eta0_dn4 = assign5290_e6589_d_n4;
        locals.var_fn61_calc_iq__eta0_dn7 = assign5290_e6589_d_n7;
        locals.var_fn61_calc_iq__eta0_dn15 = assign5290_e6589_d_n15;
        locals.var_fn61_calc_iq__eta0_dn16 = assign5290_e6589_d_n16;

        let (assign5300_e6593, assign5300_e6593_d_n2, assign5300_e6593_d_n4, assign5300_e6593_d_n7, assign5300_e6593_d_n15, assign5300_e6593_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qinvv0, locals.var_fn61_calc_iq__qinvv0_dn2, locals.var_fn61_calc_iq__qinvv0_dn4, locals.var_fn61_calc_iq__qinvv0_dn7, locals.var_fn61_calc_iq__qinvv0_dn15, locals.var_fn61_calc_iq__qinvv0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvv0 = assign5300_e6593;
        locals.var_fn61_calc_iq__qinvv0_dn2 = assign5300_e6593_d_n2;
        locals.var_fn61_calc_iq__qinvv0_dn4 = assign5300_e6593_d_n4;
        locals.var_fn61_calc_iq__qinvv0_dn7 = assign5300_e6593_d_n7;
        locals.var_fn61_calc_iq__qinvv0_dn15 = assign5300_e6593_d_n15;
        locals.var_fn61_calc_iq__qinvv0_dn16 = assign5300_e6593_d_n16;

        let (assign5310_e6597, assign5310_e6597_d_n2, assign5310_e6597_d_n3, assign5310_e6597_d_n4, assign5310_e6597_d_n7, assign5310_e6597_d_n15, assign5310_e6597_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsats, locals.var_fn61_calc_iq__vdsats_dn2, locals.var_fn61_calc_iq__vdsats_dn3, locals.var_fn61_calc_iq__vdsats_dn4, locals.var_fn61_calc_iq__vdsats_dn7, locals.var_fn61_calc_iq__vdsats_dn15, locals.var_fn61_calc_iq__vdsats_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsats = assign5310_e6597;
        locals.var_fn61_calc_iq__vdsats_dn2 = assign5310_e6597_d_n2;
        locals.var_fn61_calc_iq__vdsats_dn3 = assign5310_e6597_d_n3;
        locals.var_fn61_calc_iq__vdsats_dn4 = assign5310_e6597_d_n4;
        locals.var_fn61_calc_iq__vdsats_dn7 = assign5310_e6597_d_n7;
        locals.var_fn61_calc_iq__vdsats_dn15 = assign5310_e6597_d_n15;
        locals.var_fn61_calc_iq__vdsats_dn16 = assign5310_e6597_d_n16;

        let (assign5320_e6601, assign5320_e6601_d_n2, assign5320_e6601_d_n3, assign5320_e6601_d_n4, assign5320_e6601_d_n7, assign5320_e6601_d_n15, assign5320_e6601_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsats1, locals.var_fn61_calc_iq__vdsats1_dn2, locals.var_fn61_calc_iq__vdsats1_dn3, locals.var_fn61_calc_iq__vdsats1_dn4, locals.var_fn61_calc_iq__vdsats1_dn7, locals.var_fn61_calc_iq__vdsats1_dn15, locals.var_fn61_calc_iq__vdsats1_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsats1 = assign5320_e6601;
        locals.var_fn61_calc_iq__vdsats1_dn2 = assign5320_e6601_d_n2;
        locals.var_fn61_calc_iq__vdsats1_dn3 = assign5320_e6601_d_n3;
        locals.var_fn61_calc_iq__vdsats1_dn4 = assign5320_e6601_d_n4;
        locals.var_fn61_calc_iq__vdsats1_dn7 = assign5320_e6601_d_n7;
        locals.var_fn61_calc_iq__vdsats1_dn15 = assign5320_e6601_d_n15;
        locals.var_fn61_calc_iq__vdsats1_dn16 = assign5320_e6601_d_n16;

        let (assign5330_e6605, assign5330_e6605_d_n2, assign5330_e6605_d_n3, assign5330_e6605_d_n4, assign5330_e6605_d_n7, assign5330_e6605_d_n15, assign5330_e6605_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsat, locals.var_fn61_calc_iq__vdsat_dn2, locals.var_fn61_calc_iq__vdsat_dn3, locals.var_fn61_calc_iq__vdsat_dn4, locals.var_fn61_calc_iq__vdsat_dn7, locals.var_fn61_calc_iq__vdsat_dn15, locals.var_fn61_calc_iq__vdsat_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsat = assign5330_e6605;
        locals.var_fn61_calc_iq__vdsat_dn2 = assign5330_e6605_d_n2;
        locals.var_fn61_calc_iq__vdsat_dn3 = assign5330_e6605_d_n3;
        locals.var_fn61_calc_iq__vdsat_dn4 = assign5330_e6605_d_n4;
        locals.var_fn61_calc_iq__vdsat_dn7 = assign5330_e6605_d_n7;
        locals.var_fn61_calc_iq__vdsat_dn15 = assign5330_e6605_d_n15;
        locals.var_fn61_calc_iq__vdsat_dn16 = assign5330_e6605_d_n16;

        let (assign5340_e6609, assign5340_e6609_d_n2, assign5340_e6609_d_n3, assign5340_e6609_d_n4, assign5340_e6609_d_n7, assign5340_e6609_d_n15, assign5340_e6609_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__fsd, locals.var_fn61_calc_iq__fsd_dn2, locals.var_fn61_calc_iq__fsd_dn3, locals.var_fn61_calc_iq__fsd_dn4, locals.var_fn61_calc_iq__fsd_dn7, locals.var_fn61_calc_iq__fsd_dn15, locals.var_fn61_calc_iq__fsd_dn16,)
    }
};
        locals.var_fn61_calc_iq__fsd = assign5340_e6609;
        locals.var_fn61_calc_iq__fsd_dn2 = assign5340_e6609_d_n2;
        locals.var_fn61_calc_iq__fsd_dn3 = assign5340_e6609_d_n3;
        locals.var_fn61_calc_iq__fsd_dn4 = assign5340_e6609_d_n4;
        locals.var_fn61_calc_iq__fsd_dn7 = assign5340_e6609_d_n7;
        locals.var_fn61_calc_iq__fsd_dn15 = assign5340_e6609_d_n15;
        locals.var_fn61_calc_iq__fsd_dn16 = assign5340_e6609_d_n16;

        let (assign5350_e6613, assign5350_e6613_d_n2, assign5350_e6613_d_n3, assign5350_e6613_d_n4, assign5350_e6613_d_n7, assign5350_e6613_d_n15, assign5350_e6613_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdx, locals.var_fn61_calc_iq__vdx_dn2, locals.var_fn61_calc_iq__vdx_dn3, locals.var_fn61_calc_iq__vdx_dn4, locals.var_fn61_calc_iq__vdx_dn7, locals.var_fn61_calc_iq__vdx_dn15, locals.var_fn61_calc_iq__vdx_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdx = assign5350_e6613;
        locals.var_fn61_calc_iq__vdx_dn2 = assign5350_e6613_d_n2;
        locals.var_fn61_calc_iq__vdx_dn3 = assign5350_e6613_d_n3;
        locals.var_fn61_calc_iq__vdx_dn4 = assign5350_e6613_d_n4;
        locals.var_fn61_calc_iq__vdx_dn7 = assign5350_e6613_d_n7;
        locals.var_fn61_calc_iq__vdx_dn15 = assign5350_e6613_d_n15;
        locals.var_fn61_calc_iq__vdx_dn16 = assign5350_e6613_d_n16;

        let (assign5360_e6617, assign5360_e6617_d_n2, assign5360_e6617_d_n3, assign5360_e6617_d_n4, assign5360_e6617_d_n7, assign5360_e6617_d_n15, assign5360_e6617_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__fds, locals.var_fn61_calc_iq__fds_dn2, locals.var_fn61_calc_iq__fds_dn3, locals.var_fn61_calc_iq__fds_dn4, locals.var_fn61_calc_iq__fds_dn7, locals.var_fn61_calc_iq__fds_dn15, locals.var_fn61_calc_iq__fds_dn16,)
    }
};
        locals.var_fn61_calc_iq__fds = assign5360_e6617;
        locals.var_fn61_calc_iq__fds_dn2 = assign5360_e6617_d_n2;
        locals.var_fn61_calc_iq__fds_dn3 = assign5360_e6617_d_n3;
        locals.var_fn61_calc_iq__fds_dn4 = assign5360_e6617_d_n4;
        locals.var_fn61_calc_iq__fds_dn7 = assign5360_e6617_d_n7;
        locals.var_fn61_calc_iq__fds_dn15 = assign5360_e6617_d_n15;
        locals.var_fn61_calc_iq__fds_dn16 = assign5360_e6617_d_n16;

        let (assign5370_e6621, assign5370_e6621_d_n2, assign5370_e6621_d_n3, assign5370_e6621_d_n4, assign5370_e6621_d_n7, assign5370_e6621_d_n15, assign5370_e6621_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vsx, locals.var_fn61_calc_iq__vsx_dn2, locals.var_fn61_calc_iq__vsx_dn3, locals.var_fn61_calc_iq__vsx_dn4, locals.var_fn61_calc_iq__vsx_dn7, locals.var_fn61_calc_iq__vsx_dn15, locals.var_fn61_calc_iq__vsx_dn16,)
    }
};
        locals.var_fn61_calc_iq__vsx = assign5370_e6621;
        locals.var_fn61_calc_iq__vsx_dn2 = assign5370_e6621_d_n2;
        locals.var_fn61_calc_iq__vsx_dn3 = assign5370_e6621_d_n3;
        locals.var_fn61_calc_iq__vsx_dn4 = assign5370_e6621_d_n4;
        locals.var_fn61_calc_iq__vsx_dn7 = assign5370_e6621_d_n7;
        locals.var_fn61_calc_iq__vsx_dn15 = assign5370_e6621_d_n15;
        locals.var_fn61_calc_iq__vsx_dn16 = assign5370_e6621_d_n16;

        let (assign5380_e6625, assign5380_e6625_d_n2, assign5380_e6625_d_n3, assign5380_e6625_d_n4, assign5380_e6625_d_n7, assign5380_e6625_d_n15, assign5380_e6625_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffd, locals.var_fn61_calc_iq__ffd_dn2, locals.var_fn61_calc_iq__ffd_dn3, locals.var_fn61_calc_iq__ffd_dn4, locals.var_fn61_calc_iq__ffd_dn7, locals.var_fn61_calc_iq__ffd_dn15, locals.var_fn61_calc_iq__ffd_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffd = assign5380_e6625;
        locals.var_fn61_calc_iq__ffd_dn2 = assign5380_e6625_d_n2;
        locals.var_fn61_calc_iq__ffd_dn3 = assign5380_e6625_d_n3;
        locals.var_fn61_calc_iq__ffd_dn4 = assign5380_e6625_d_n4;
        locals.var_fn61_calc_iq__ffd_dn7 = assign5380_e6625_d_n7;
        locals.var_fn61_calc_iq__ffd_dn15 = assign5380_e6625_d_n15;
        locals.var_fn61_calc_iq__ffd_dn16 = assign5380_e6625_d_n16;

        let (assign5390_e6629, assign5390_e6629_d_n2, assign5390_e6629_d_n3, assign5390_e6629_d_n4, assign5390_e6629_d_n7, assign5390_e6629_d_n15, assign5390_e6629_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__etad, locals.var_fn61_calc_iq__etad_dn2, locals.var_fn61_calc_iq__etad_dn3, locals.var_fn61_calc_iq__etad_dn4, locals.var_fn61_calc_iq__etad_dn7, locals.var_fn61_calc_iq__etad_dn15, locals.var_fn61_calc_iq__etad_dn16,)
    }
};
        locals.var_fn61_calc_iq__etad = assign5390_e6629;
        locals.var_fn61_calc_iq__etad_dn2 = assign5390_e6629_d_n2;
        locals.var_fn61_calc_iq__etad_dn3 = assign5390_e6629_d_n3;
        locals.var_fn61_calc_iq__etad_dn4 = assign5390_e6629_d_n4;
        locals.var_fn61_calc_iq__etad_dn7 = assign5390_e6629_d_n7;
        locals.var_fn61_calc_iq__etad_dn15 = assign5390_e6629_d_n15;
        locals.var_fn61_calc_iq__etad_dn16 = assign5390_e6629_d_n16;

        let (assign5400_e6633, assign5400_e6633_d_n2, assign5400_e6633_d_n3, assign5400_e6633_d_n4, assign5400_e6633_d_n7, assign5400_e6633_d_n15, assign5400_e6633_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qinvd, locals.var_fn61_calc_iq__qinvd_dn2, locals.var_fn61_calc_iq__qinvd_dn3, locals.var_fn61_calc_iq__qinvd_dn4, locals.var_fn61_calc_iq__qinvd_dn7, locals.var_fn61_calc_iq__qinvd_dn15, locals.var_fn61_calc_iq__qinvd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvd = assign5400_e6633;
        locals.var_fn61_calc_iq__qinvd_dn2 = assign5400_e6633_d_n2;
        locals.var_fn61_calc_iq__qinvd_dn3 = assign5400_e6633_d_n3;
        locals.var_fn61_calc_iq__qinvd_dn4 = assign5400_e6633_d_n4;
        locals.var_fn61_calc_iq__qinvd_dn7 = assign5400_e6633_d_n7;
        locals.var_fn61_calc_iq__qinvd_dn15 = assign5400_e6633_d_n15;
        locals.var_fn61_calc_iq__qinvd_dn16 = assign5400_e6633_d_n16;

        let (assign5410_e6637, assign5410_e6637_d_n2, assign5410_e6637_d_n3, assign5410_e6637_d_n4, assign5410_e6637_d_n7, assign5410_e6637_d_n15, assign5410_e6637_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsc, locals.var_fn61_calc_iq__vdsc_dn2, locals.var_fn61_calc_iq__vdsc_dn3, locals.var_fn61_calc_iq__vdsc_dn4, locals.var_fn61_calc_iq__vdsc_dn7, locals.var_fn61_calc_iq__vdsc_dn15, locals.var_fn61_calc_iq__vdsc_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsc = assign5410_e6637;
        locals.var_fn61_calc_iq__vdsc_dn2 = assign5410_e6637_d_n2;
        locals.var_fn61_calc_iq__vdsc_dn3 = assign5410_e6637_d_n3;
        locals.var_fn61_calc_iq__vdsc_dn4 = assign5410_e6637_d_n4;
        locals.var_fn61_calc_iq__vdsc_dn7 = assign5410_e6637_d_n7;
        locals.var_fn61_calc_iq__vdsc_dn15 = assign5410_e6637_d_n15;
        locals.var_fn61_calc_iq__vdsc_dn16 = assign5410_e6637_d_n16;

        let (assign5420_e6641, assign5420_e6641_d_n2, assign5420_e6641_d_n3, assign5420_e6641_d_n4, assign5420_e6641_d_n7, assign5420_e6641_d_n15, assign5420_e6641_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__fsat, locals.var_fn61_calc_iq__fsat_dn2, locals.var_fn61_calc_iq__fsat_dn3, locals.var_fn61_calc_iq__fsat_dn4, locals.var_fn61_calc_iq__fsat_dn7, locals.var_fn61_calc_iq__fsat_dn15, locals.var_fn61_calc_iq__fsat_dn16,)
    }
};
        locals.var_fn61_calc_iq__fsat = assign5420_e6641;
        locals.var_fn61_calc_iq__fsat_dn2 = assign5420_e6641_d_n2;
        locals.var_fn61_calc_iq__fsat_dn3 = assign5420_e6641_d_n3;
        locals.var_fn61_calc_iq__fsat_dn4 = assign5420_e6641_d_n4;
        locals.var_fn61_calc_iq__fsat_dn7 = assign5420_e6641_d_n7;
        locals.var_fn61_calc_iq__fsat_dn15 = assign5420_e6641_d_n15;
        locals.var_fn61_calc_iq__fsat_dn16 = assign5420_e6641_d_n16;

        let (assign5430_e6645, assign5430_e6645_d_n2, assign5430_e6645_d_n3, assign5430_e6645_d_n4, assign5430_e6645_d_n7, assign5430_e6645_d_n15, assign5430_e6645_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vel, locals.var_fn61_calc_iq__vel_dn2, locals.var_fn61_calc_iq__vel_dn3, locals.var_fn61_calc_iq__vel_dn4, locals.var_fn61_calc_iq__vel_dn7, locals.var_fn61_calc_iq__vel_dn15, locals.var_fn61_calc_iq__vel_dn16,)
    }
};
        locals.var_fn61_calc_iq__vel = assign5430_e6645;
        locals.var_fn61_calc_iq__vel_dn2 = assign5430_e6645_d_n2;
        locals.var_fn61_calc_iq__vel_dn3 = assign5430_e6645_d_n3;
        locals.var_fn61_calc_iq__vel_dn4 = assign5430_e6645_d_n4;
        locals.var_fn61_calc_iq__vel_dn7 = assign5430_e6645_d_n7;
        locals.var_fn61_calc_iq__vel_dn15 = assign5430_e6645_d_n15;
        locals.var_fn61_calc_iq__vel_dn16 = assign5430_e6645_d_n16;

        let (assign5440_e6649, assign5440_e6649_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsats0, locals.var_fn61_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn61_calc_iq__vdsats0 = assign5440_e6649;
        locals.var_fn61_calc_iq__vdsats0_dn4 = assign5440_e6649_d_n4;

        let (assign5450_e6653, assign5450_e6653_d_n2, assign5450_e6653_d_n4, assign5450_e6653_d_n7, assign5450_e6653_d_n15, assign5450_e6653_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsats10, locals.var_fn61_calc_iq__vdsats10_dn2, locals.var_fn61_calc_iq__vdsats10_dn4, locals.var_fn61_calc_iq__vdsats10_dn7, locals.var_fn61_calc_iq__vdsats10_dn15, locals.var_fn61_calc_iq__vdsats10_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsats10 = assign5450_e6653;
        locals.var_fn61_calc_iq__vdsats10_dn2 = assign5450_e6653_d_n2;
        locals.var_fn61_calc_iq__vdsats10_dn4 = assign5450_e6653_d_n4;
        locals.var_fn61_calc_iq__vdsats10_dn7 = assign5450_e6653_d_n7;
        locals.var_fn61_calc_iq__vdsats10_dn15 = assign5450_e6653_d_n15;
        locals.var_fn61_calc_iq__vdsats10_dn16 = assign5450_e6653_d_n16;

        let (assign5460_e6657, assign5460_e6657_d_n2, assign5460_e6657_d_n4, assign5460_e6657_d_n7, assign5460_e6657_d_n15, assign5460_e6657_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsat10, locals.var_fn61_calc_iq__vdsat10_dn2, locals.var_fn61_calc_iq__vdsat10_dn4, locals.var_fn61_calc_iq__vdsat10_dn7, locals.var_fn61_calc_iq__vdsat10_dn15, locals.var_fn61_calc_iq__vdsat10_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsat10 = assign5460_e6657;
        locals.var_fn61_calc_iq__vdsat10_dn2 = assign5460_e6657_d_n2;
        locals.var_fn61_calc_iq__vdsat10_dn4 = assign5460_e6657_d_n4;
        locals.var_fn61_calc_iq__vdsat10_dn7 = assign5460_e6657_d_n7;
        locals.var_fn61_calc_iq__vdsat10_dn15 = assign5460_e6657_d_n15;
        locals.var_fn61_calc_iq__vdsat10_dn16 = assign5460_e6657_d_n16;

        let (assign5470_e6661, assign5470_e6661_d_n2, assign5470_e6661_d_n4, assign5470_e6661_d_n7, assign5470_e6661_d_n15, assign5470_e6661_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__fsd0, locals.var_fn61_calc_iq__fsd0_dn2, locals.var_fn61_calc_iq__fsd0_dn4, locals.var_fn61_calc_iq__fsd0_dn7, locals.var_fn61_calc_iq__fsd0_dn15, locals.var_fn61_calc_iq__fsd0_dn16,)
    }
};
        locals.var_fn61_calc_iq__fsd0 = assign5470_e6661;
        locals.var_fn61_calc_iq__fsd0_dn2 = assign5470_e6661_d_n2;
        locals.var_fn61_calc_iq__fsd0_dn4 = assign5470_e6661_d_n4;
        locals.var_fn61_calc_iq__fsd0_dn7 = assign5470_e6661_d_n7;
        locals.var_fn61_calc_iq__fsd0_dn15 = assign5470_e6661_d_n15;
        locals.var_fn61_calc_iq__fsd0_dn16 = assign5470_e6661_d_n16;

        let (assign5480_e6665, assign5480_e6665_d_n2, assign5480_e6665_d_n4, assign5480_e6665_d_n7, assign5480_e6665_d_n15, assign5480_e6665_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdx0, locals.var_fn61_calc_iq__vdx0_dn2, locals.var_fn61_calc_iq__vdx0_dn4, locals.var_fn61_calc_iq__vdx0_dn7, locals.var_fn61_calc_iq__vdx0_dn15, locals.var_fn61_calc_iq__vdx0_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdx0 = assign5480_e6665;
        locals.var_fn61_calc_iq__vdx0_dn2 = assign5480_e6665_d_n2;
        locals.var_fn61_calc_iq__vdx0_dn4 = assign5480_e6665_d_n4;
        locals.var_fn61_calc_iq__vdx0_dn7 = assign5480_e6665_d_n7;
        locals.var_fn61_calc_iq__vdx0_dn15 = assign5480_e6665_d_n15;
        locals.var_fn61_calc_iq__vdx0_dn16 = assign5480_e6665_d_n16;

        let (assign5490_e6669, assign5490_e6669_d_n2, assign5490_e6669_d_n4, assign5490_e6669_d_n7, assign5490_e6669_d_n15, assign5490_e6669_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__fds0, locals.var_fn61_calc_iq__fds0_dn2, locals.var_fn61_calc_iq__fds0_dn4, locals.var_fn61_calc_iq__fds0_dn7, locals.var_fn61_calc_iq__fds0_dn15, locals.var_fn61_calc_iq__fds0_dn16,)
    }
};
        locals.var_fn61_calc_iq__fds0 = assign5490_e6669;
        locals.var_fn61_calc_iq__fds0_dn2 = assign5490_e6669_d_n2;
        locals.var_fn61_calc_iq__fds0_dn4 = assign5490_e6669_d_n4;
        locals.var_fn61_calc_iq__fds0_dn7 = assign5490_e6669_d_n7;
        locals.var_fn61_calc_iq__fds0_dn15 = assign5490_e6669_d_n15;
        locals.var_fn61_calc_iq__fds0_dn16 = assign5490_e6669_d_n16;

        let (assign5500_e6673, assign5500_e6673_d_n2, assign5500_e6673_d_n4, assign5500_e6673_d_n7, assign5500_e6673_d_n15, assign5500_e6673_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vsx0, locals.var_fn61_calc_iq__vsx0_dn2, locals.var_fn61_calc_iq__vsx0_dn4, locals.var_fn61_calc_iq__vsx0_dn7, locals.var_fn61_calc_iq__vsx0_dn15, locals.var_fn61_calc_iq__vsx0_dn16,)
    }
};
        locals.var_fn61_calc_iq__vsx0 = assign5500_e6673;
        locals.var_fn61_calc_iq__vsx0_dn2 = assign5500_e6673_d_n2;
        locals.var_fn61_calc_iq__vsx0_dn4 = assign5500_e6673_d_n4;
        locals.var_fn61_calc_iq__vsx0_dn7 = assign5500_e6673_d_n7;
        locals.var_fn61_calc_iq__vsx0_dn15 = assign5500_e6673_d_n15;
        locals.var_fn61_calc_iq__vsx0_dn16 = assign5500_e6673_d_n16;

        let (assign5510_e6677, assign5510_e6677_d_n2, assign5510_e6677_d_n4, assign5510_e6677_d_n7, assign5510_e6677_d_n15, assign5510_e6677_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffd0, locals.var_fn61_calc_iq__ffd0_dn2, locals.var_fn61_calc_iq__ffd0_dn4, locals.var_fn61_calc_iq__ffd0_dn7, locals.var_fn61_calc_iq__ffd0_dn15, locals.var_fn61_calc_iq__ffd0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffd0 = assign5510_e6677;
        locals.var_fn61_calc_iq__ffd0_dn2 = assign5510_e6677_d_n2;
        locals.var_fn61_calc_iq__ffd0_dn4 = assign5510_e6677_d_n4;
        locals.var_fn61_calc_iq__ffd0_dn7 = assign5510_e6677_d_n7;
        locals.var_fn61_calc_iq__ffd0_dn15 = assign5510_e6677_d_n15;
        locals.var_fn61_calc_iq__ffd0_dn16 = assign5510_e6677_d_n16;

        let (assign5520_e6681, assign5520_e6681_d_n2, assign5520_e6681_d_n4, assign5520_e6681_d_n7, assign5520_e6681_d_n15, assign5520_e6681_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__etad0, locals.var_fn61_calc_iq__etad0_dn2, locals.var_fn61_calc_iq__etad0_dn4, locals.var_fn61_calc_iq__etad0_dn7, locals.var_fn61_calc_iq__etad0_dn15, locals.var_fn61_calc_iq__etad0_dn16,)
    }
};
        locals.var_fn61_calc_iq__etad0 = assign5520_e6681;
        locals.var_fn61_calc_iq__etad0_dn2 = assign5520_e6681_d_n2;
        locals.var_fn61_calc_iq__etad0_dn4 = assign5520_e6681_d_n4;
        locals.var_fn61_calc_iq__etad0_dn7 = assign5520_e6681_d_n7;
        locals.var_fn61_calc_iq__etad0_dn15 = assign5520_e6681_d_n15;
        locals.var_fn61_calc_iq__etad0_dn16 = assign5520_e6681_d_n16;

        let (assign5530_e6685, assign5530_e6685_d_n2, assign5530_e6685_d_n4, assign5530_e6685_d_n7, assign5530_e6685_d_n15, assign5530_e6685_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qinvd0, locals.var_fn61_calc_iq__qinvd0_dn2, locals.var_fn61_calc_iq__qinvd0_dn4, locals.var_fn61_calc_iq__qinvd0_dn7, locals.var_fn61_calc_iq__qinvd0_dn15, locals.var_fn61_calc_iq__qinvd0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvd0 = assign5530_e6685;
        locals.var_fn61_calc_iq__qinvd0_dn2 = assign5530_e6685_d_n2;
        locals.var_fn61_calc_iq__qinvd0_dn4 = assign5530_e6685_d_n4;
        locals.var_fn61_calc_iq__qinvd0_dn7 = assign5530_e6685_d_n7;
        locals.var_fn61_calc_iq__qinvd0_dn15 = assign5530_e6685_d_n15;
        locals.var_fn61_calc_iq__qinvd0_dn16 = assign5530_e6685_d_n16;

        let (assign5540_e6689, assign5540_e6689_d_n2, assign5540_e6689_d_n4, assign5540_e6689_d_n7, assign5540_e6689_d_n15, assign5540_e6689_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qs2, locals.var_fn61_calc_iq__qs2_dn2, locals.var_fn61_calc_iq__qs2_dn4, locals.var_fn61_calc_iq__qs2_dn7, locals.var_fn61_calc_iq__qs2_dn15, locals.var_fn61_calc_iq__qs2_dn16,)
    }
};
        locals.var_fn61_calc_iq__qs2 = assign5540_e6689;
        locals.var_fn61_calc_iq__qs2_dn2 = assign5540_e6689_d_n2;
        locals.var_fn61_calc_iq__qs2_dn4 = assign5540_e6689_d_n4;
        locals.var_fn61_calc_iq__qs2_dn7 = assign5540_e6689_d_n7;
        locals.var_fn61_calc_iq__qs2_dn15 = assign5540_e6689_d_n15;
        locals.var_fn61_calc_iq__qs2_dn16 = assign5540_e6689_d_n16;

        let (assign5550_e6693, assign5550_e6693_d_n2, assign5550_e6693_d_n4, assign5550_e6693_d_n7, assign5550_e6693_d_n15, assign5550_e6693_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qs3, locals.var_fn61_calc_iq__qs3_dn2, locals.var_fn61_calc_iq__qs3_dn4, locals.var_fn61_calc_iq__qs3_dn7, locals.var_fn61_calc_iq__qs3_dn15, locals.var_fn61_calc_iq__qs3_dn16,)
    }
};
        locals.var_fn61_calc_iq__qs3 = assign5550_e6693;
        locals.var_fn61_calc_iq__qs3_dn2 = assign5550_e6693_d_n2;
        locals.var_fn61_calc_iq__qs3_dn4 = assign5550_e6693_d_n4;
        locals.var_fn61_calc_iq__qs3_dn7 = assign5550_e6693_d_n7;
        locals.var_fn61_calc_iq__qs3_dn15 = assign5550_e6693_d_n15;
        locals.var_fn61_calc_iq__qs3_dn16 = assign5550_e6693_d_n16;

        let (assign5560_e6697, assign5560_e6697_d_n2, assign5560_e6697_d_n4, assign5560_e6697_d_n7, assign5560_e6697_d_n15, assign5560_e6697_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qd2, locals.var_fn61_calc_iq__qd2_dn2, locals.var_fn61_calc_iq__qd2_dn4, locals.var_fn61_calc_iq__qd2_dn7, locals.var_fn61_calc_iq__qd2_dn15, locals.var_fn61_calc_iq__qd2_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd2 = assign5560_e6697;
        locals.var_fn61_calc_iq__qd2_dn2 = assign5560_e6697_d_n2;
        locals.var_fn61_calc_iq__qd2_dn4 = assign5560_e6697_d_n4;
        locals.var_fn61_calc_iq__qd2_dn7 = assign5560_e6697_d_n7;
        locals.var_fn61_calc_iq__qd2_dn15 = assign5560_e6697_d_n15;
        locals.var_fn61_calc_iq__qd2_dn16 = assign5560_e6697_d_n16;

        let (assign5570_e6701, assign5570_e6701_d_n2, assign5570_e6701_d_n4, assign5570_e6701_d_n7, assign5570_e6701_d_n15, assign5570_e6701_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qd3, locals.var_fn61_calc_iq__qd3_dn2, locals.var_fn61_calc_iq__qd3_dn4, locals.var_fn61_calc_iq__qd3_dn7, locals.var_fn61_calc_iq__qd3_dn15, locals.var_fn61_calc_iq__qd3_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd3 = assign5570_e6701;
        locals.var_fn61_calc_iq__qd3_dn2 = assign5570_e6701_d_n2;
        locals.var_fn61_calc_iq__qd3_dn4 = assign5570_e6701_d_n4;
        locals.var_fn61_calc_iq__qd3_dn7 = assign5570_e6701_d_n7;
        locals.var_fn61_calc_iq__qd3_dn15 = assign5570_e6701_d_n15;
        locals.var_fn61_calc_iq__qd3_dn16 = assign5570_e6701_d_n16;

        let (assign5580_e6705, assign5580_e6705_d_n2, assign5580_e6705_d_n4, assign5580_e6705_d_n7, assign5580_e6705_d_n15, assign5580_e6705_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qsqd, locals.var_fn61_calc_iq__qsqd_dn2, locals.var_fn61_calc_iq__qsqd_dn4, locals.var_fn61_calc_iq__qsqd_dn7, locals.var_fn61_calc_iq__qsqd_dn15, locals.var_fn61_calc_iq__qsqd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qsqd = assign5580_e6705;
        locals.var_fn61_calc_iq__qsqd_dn2 = assign5580_e6705_d_n2;
        locals.var_fn61_calc_iq__qsqd_dn4 = assign5580_e6705_d_n4;
        locals.var_fn61_calc_iq__qsqd_dn7 = assign5580_e6705_d_n7;
        locals.var_fn61_calc_iq__qsqd_dn15 = assign5580_e6705_d_n15;
        locals.var_fn61_calc_iq__qsqd_dn16 = assign5580_e6705_d_n16;

        let (assign5590_e6709, assign5590_e6709_d_n2, assign5590_e6709_d_n4, assign5590_e6709_d_n7, assign5590_e6709_d_n15, assign5590_e6709_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qinvdd, locals.var_fn61_calc_iq__qinvdd_dn2, locals.var_fn61_calc_iq__qinvdd_dn4, locals.var_fn61_calc_iq__qinvdd_dn7, locals.var_fn61_calc_iq__qinvdd_dn15, locals.var_fn61_calc_iq__qinvdd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvdd = assign5590_e6709;
        locals.var_fn61_calc_iq__qinvdd_dn2 = assign5590_e6709_d_n2;
        locals.var_fn61_calc_iq__qinvdd_dn4 = assign5590_e6709_d_n4;
        locals.var_fn61_calc_iq__qinvdd_dn7 = assign5590_e6709_d_n7;
        locals.var_fn61_calc_iq__qinvdd_dn15 = assign5590_e6709_d_n15;
        locals.var_fn61_calc_iq__qinvdd_dn16 = assign5590_e6709_d_n16;

        let (assign5600_e6713, assign5600_e6713_d_n2, assign5600_e6713_d_n4, assign5600_e6713_d_n7, assign5600_e6713_d_n15, assign5600_e6713_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qd1, locals.var_fn61_calc_iq__qd1_dn2, locals.var_fn61_calc_iq__qd1_dn4, locals.var_fn61_calc_iq__qd1_dn7, locals.var_fn61_calc_iq__qd1_dn15, locals.var_fn61_calc_iq__qd1_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd1 = assign5600_e6713;
        locals.var_fn61_calc_iq__qd1_dn2 = assign5600_e6713_d_n2;
        locals.var_fn61_calc_iq__qd1_dn4 = assign5600_e6713_d_n4;
        locals.var_fn61_calc_iq__qd1_dn7 = assign5600_e6713_d_n7;
        locals.var_fn61_calc_iq__qd1_dn15 = assign5600_e6713_d_n15;
        locals.var_fn61_calc_iq__qd1_dn16 = assign5600_e6713_d_n16;

        let (assign5610_e6717, assign5610_e6717_d_n2, assign5610_e6717_d_n4, assign5610_e6717_d_n7, assign5610_e6717_d_n15, assign5610_e6717_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qs, locals.var_fn61_calc_iq__qs_dn2, locals.var_fn61_calc_iq__qs_dn4, locals.var_fn61_calc_iq__qs_dn7, locals.var_fn61_calc_iq__qs_dn15, locals.var_fn61_calc_iq__qs_dn16,)
    }
};
        locals.var_fn61_calc_iq__qs = assign5610_e6717;
        locals.var_fn61_calc_iq__qs_dn2 = assign5610_e6717_d_n2;
        locals.var_fn61_calc_iq__qs_dn4 = assign5610_e6717_d_n4;
        locals.var_fn61_calc_iq__qs_dn7 = assign5610_e6717_d_n7;
        locals.var_fn61_calc_iq__qs_dn15 = assign5610_e6717_d_n15;
        locals.var_fn61_calc_iq__qs_dn16 = assign5610_e6717_d_n16;

        let (assign5620_e6721, assign5620_e6721_d_n2, assign5620_e6721_d_n4, assign5620_e6721_d_n7, assign5620_e6721_d_n15, assign5620_e6721_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qd, locals.var_fn61_calc_iq__qd_dn2, locals.var_fn61_calc_iq__qd_dn4, locals.var_fn61_calc_iq__qd_dn7, locals.var_fn61_calc_iq__qd_dn15, locals.var_fn61_calc_iq__qd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd = assign5620_e6721;
        locals.var_fn61_calc_iq__qd_dn2 = assign5620_e6721_d_n2;
        locals.var_fn61_calc_iq__qd_dn4 = assign5620_e6721_d_n4;
        locals.var_fn61_calc_iq__qd_dn7 = assign5620_e6721_d_n7;
        locals.var_fn61_calc_iq__qd_dn15 = assign5620_e6721_d_n15;
        locals.var_fn61_calc_iq__qd_dn16 = assign5620_e6721_d_n16;

        let (assign5630_e6725, assign5630_e6725_d_n2, assign5630_e6725_d_n4, assign5630_e6725_d_n7, assign5630_e6725_d_n15,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__etac, locals.var_fn61_calc_iq__etac_dn2, locals.var_fn61_calc_iq__etac_dn4, locals.var_fn61_calc_iq__etac_dn7, locals.var_fn61_calc_iq__etac_dn15,)
    }
};
        locals.var_fn61_calc_iq__etac = assign5630_e6725;
        locals.var_fn61_calc_iq__etac_dn2 = assign5630_e6725_d_n2;
        locals.var_fn61_calc_iq__etac_dn4 = assign5630_e6725_d_n4;
        locals.var_fn61_calc_iq__etac_dn7 = assign5630_e6725_d_n7;
        locals.var_fn61_calc_iq__etac_dn15 = assign5630_e6725_d_n15;

        let (assign5640_e6729, assign5640_e6729_d_n3, assign5640_e6729_d_n4, assign5640_e6729_d_n15,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__etab, locals.var_fn61_calc_iq__etab_dn3, locals.var_fn61_calc_iq__etab_dn4, locals.var_fn61_calc_iq__etab_dn15,)
    }
};
        locals.var_fn61_calc_iq__etab = assign5640_e6729;
        locals.var_fn61_calc_iq__etab_dn3 = assign5640_e6729_d_n3;
        locals.var_fn61_calc_iq__etab_dn4 = assign5640_e6729_d_n4;
        locals.var_fn61_calc_iq__etab_dn15 = assign5640_e6729_d_n15;

    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5650_e6733, assign5650_e6733_d_n2, assign5650_e6733_d_n4, assign5650_e6733_d_n7, assign5650_e6733_d_n15,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__etags, locals.var_fn61_calc_iq__etags_dn2, locals.var_fn61_calc_iq__etags_dn4, locals.var_fn61_calc_iq__etags_dn7, locals.var_fn61_calc_iq__etags_dn15,)
    }
};
        locals.var_fn61_calc_iq__etags = assign5650_e6733;
        locals.var_fn61_calc_iq__etags_dn2 = assign5650_e6733_d_n2;
        locals.var_fn61_calc_iq__etags_dn4 = assign5650_e6733_d_n4;
        locals.var_fn61_calc_iq__etags_dn7 = assign5650_e6733_d_n7;
        locals.var_fn61_calc_iq__etags_dn15 = assign5650_e6733_d_n15;

        let (assign5660_e6737, assign5660_e6737_d_n2, assign5660_e6737_d_n3, assign5660_e6737_d_n4, assign5660_e6737_d_n7, assign5660_e6737_d_n15, assign5660_e6737_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign5660_e6737;
        locals.var_fn61_calc_iq__exparg_dn2 = assign5660_e6737_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign5660_e6737_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign5660_e6737_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign5660_e6737_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign5660_e6737_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign5660_e6737_d_n16;

        let (assign5670_e6741, assign5670_e6741_d_n2, assign5670_e6741_d_n3, assign5670_e6741_d_n4, assign5670_e6741_d_n7, assign5670_e6741_d_n15, assign5670_e6741_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__myarg, locals.var_fn61_calc_iq__myarg_dn2, locals.var_fn61_calc_iq__myarg_dn3, locals.var_fn61_calc_iq__myarg_dn4, locals.var_fn61_calc_iq__myarg_dn7, locals.var_fn61_calc_iq__myarg_dn15, locals.var_fn61_calc_iq__myarg_dn16,)
    }
};
        locals.var_fn61_calc_iq__myarg = assign5670_e6741;
        locals.var_fn61_calc_iq__myarg_dn2 = assign5670_e6741_d_n2;
        locals.var_fn61_calc_iq__myarg_dn3 = assign5670_e6741_d_n3;
        locals.var_fn61_calc_iq__myarg_dn4 = assign5670_e6741_d_n4;
        locals.var_fn61_calc_iq__myarg_dn7 = assign5670_e6741_d_n7;
        locals.var_fn61_calc_iq__myarg_dn15 = assign5670_e6741_d_n15;
        locals.var_fn61_calc_iq__myarg_dn16 = assign5670_e6741_d_n16;

        let (assign5680_e6745, assign5680_e6745_d_n15, assign5680_e6745_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__absvdsin, locals.var_fn61_calc_iq__absvdsin_dn15, locals.var_fn61_calc_iq__absvdsin_dn16,)
    }
};
        locals.var_fn61_calc_iq__absvdsin = assign5680_e6745;
        locals.var_fn61_calc_iq__absvdsin_dn15 = assign5680_e6745_d_n15;
        locals.var_fn61_calc_iq__absvdsin_dn16 = assign5680_e6745_d_n16;

        let (assign5690_e6749, assign5690_e6749_d_n2, assign5690_e6749_d_n7, assign5690_e6749_d_n15, assign5690_e6749_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vgdin, locals.var_fn61_calc_iq__vgdin_dn2, locals.var_fn61_calc_iq__vgdin_dn7, locals.var_fn61_calc_iq__vgdin_dn15, locals.var_fn61_calc_iq__vgdin_dn16,)
    }
};
        locals.var_fn61_calc_iq__vgdin = assign5690_e6749;
        locals.var_fn61_calc_iq__vgdin_dn2 = assign5690_e6749_d_n2;
        locals.var_fn61_calc_iq__vgdin_dn7 = assign5690_e6749_d_n7;
        locals.var_fn61_calc_iq__vgdin_dn15 = assign5690_e6749_d_n15;
        locals.var_fn61_calc_iq__vgdin_dn16 = assign5690_e6749_d_n16;

        let (assign5700_e6753, assign5700_e6753_d_n2, assign5700_e6753_d_n4, assign5700_e6753_d_n7, assign5700_e6753_d_n15, assign5700_e6753_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg0, locals.var_fn61_calc_iq__exparg0_dn2, locals.var_fn61_calc_iq__exparg0_dn4, locals.var_fn61_calc_iq__exparg0_dn7, locals.var_fn61_calc_iq__exparg0_dn15, locals.var_fn61_calc_iq__exparg0_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg0 = assign5700_e6753;
        locals.var_fn61_calc_iq__exparg0_dn2 = assign5700_e6753_d_n2;
        locals.var_fn61_calc_iq__exparg0_dn4 = assign5700_e6753_d_n4;
        locals.var_fn61_calc_iq__exparg0_dn7 = assign5700_e6753_d_n7;
        locals.var_fn61_calc_iq__exparg0_dn15 = assign5700_e6753_d_n15;
        locals.var_fn61_calc_iq__exparg0_dn16 = assign5700_e6753_d_n16;

        let (assign5710_e6757, assign5710_e6757_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__myarg0, locals.var_fn61_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn61_calc_iq__myarg0 = assign5710_e6757;
        locals.var_fn61_calc_iq__myarg0_dn4 = assign5710_e6757_d_n4;

        let (assign5720_e6784, assign5720_e6784_d_n15, assign5720_e6784_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let (assign5720_e6782, assign5720_e6782_d_n15, assign5720_e6782_d_n16,) = {
            if (p.p52 != 0.0) {
                let assign5720_e6766: f64 = (0.001 / p.p53);
                let assign5720_e6768: f64 = (assign5720_e6766 * locals.var_fn61_calc_iq__vdsin);
                let assign5720_e6769: f64 = (assign5720_e6768).tanh();
                let assign5720_e6770: f64 = (locals.var_fn61_calc_iq__vdsin * assign5720_e6769);
                (assign5720_e6770, ((locals.var_fn61_calc_iq__vdsin_dn15 * assign5720_e6769) + (locals.var_fn61_calc_iq__vdsin * ((assign5720_e6766 * locals.var_fn61_calc_iq__vdsin_dn15) / ((assign5720_e6768).cosh() * (assign5720_e6768).cosh())))), ((locals.var_fn61_calc_iq__vdsin_dn16 * assign5720_e6769) + (locals.var_fn61_calc_iq__vdsin * ((assign5720_e6766 * locals.var_fn61_calc_iq__vdsin_dn16) / ((assign5720_e6768).cosh() * (assign5720_e6768).cosh())))),)
            } else {
                let (assign5720_e6781, assign5720_e6781_d_n15, assign5720_e6781_d_n16,) = {
                    if (p.p52 == 0.0) {
                        let assign5720_e6776: f64 = (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsin);
                        let assign5720_e6778: f64 = (assign5720_e6776 + p.p53);
                        let assign5720_e6779: f64 = (assign5720_e6778).sqrt();
                        (assign5720_e6779, (((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsin) + (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsin_dn15)) / (2.0 * assign5720_e6779)), (((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsin) + (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsin_dn16)) / (2.0 * assign5720_e6779)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign5720_e6781, assign5720_e6781_d_n15, assign5720_e6781_d_n16,)
            }
        };
        (assign5720_e6782, assign5720_e6782_d_n15, assign5720_e6782_d_n16,)
    } else {
        (locals.var_fn61_calc_iq__absvdsin, locals.var_fn61_calc_iq__absvdsin_dn15, locals.var_fn61_calc_iq__absvdsin_dn16,)
    }
};
        locals.var_fn61_calc_iq__absvdsin = assign5720_e6784;
        locals.var_fn61_calc_iq__absvdsin_dn15 = assign5720_e6784_d_n15;
        locals.var_fn61_calc_iq__absvdsin_dn16 = assign5720_e6784_d_n16;

        let (assign5730_e6790, assign5730_e6790_d_n2, assign5730_e6790_d_n7, assign5730_e6790_d_n15, assign5730_e6790_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5730_e6788: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vdsin);
        (assign5730_e6788, locals.var_fn61_calc_iq__vgsin_dn2, locals.var_fn61_calc_iq__vgsin_dn7, (locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vdsin_dn15), (-locals.var_fn61_calc_iq__vdsin_dn16),)
    } else {
        (locals.var_fn61_calc_iq__vgdin, locals.var_fn61_calc_iq__vgdin_dn2, locals.var_fn61_calc_iq__vgdin_dn7, locals.var_fn61_calc_iq__vgdin_dn15, locals.var_fn61_calc_iq__vgdin_dn16,)
    }
};
        locals.var_fn61_calc_iq__vgdin = assign5730_e6790;
        locals.var_fn61_calc_iq__vgdin_dn2 = assign5730_e6790_d_n2;
        locals.var_fn61_calc_iq__vgdin_dn7 = assign5730_e6790_d_n7;
        locals.var_fn61_calc_iq__vgdin_dn15 = assign5730_e6790_d_n15;
        locals.var_fn61_calc_iq__vgdin_dn16 = assign5730_e6790_d_n16;

        let (assign5740_e6796, assign5740_e6796_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5740_e6794: f64 = (locals.var_fn61_calc_iq__alpha * locals.var_fn61_calc_iq__phitin);
        (assign5740_e6794, (locals.var_fn61_calc_iq__alpha * locals.var_fn61_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn61_calc_iq__alpha_phit, locals.var_fn61_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn61_calc_iq__alpha_phit = assign5740_e6796;
        locals.var_fn61_calc_iq__alpha_phit_dn4 = assign5740_e6796_d_n4;

        let (assign5750_e6808, assign5750_e6808_d_n4, assign5750_e6808_d_n15, assign5750_e6808_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5750_e6801: f64 = (2.302585092994046 * locals.var_fn61_calc_iq__phitin);
        let assign5750_e6802: f64 = (locals.var_fn61_calc_iq__ss / assign5750_e6801);
        let assign5750_e6805: f64 = (locals.var_fn61_calc_iq__nd * locals.var_fn61_calc_iq__absvdsin);
        let assign5750_e6806: f64 = (assign5750_e6802 + assign5750_e6805);
        (assign5750_e6806, (-((locals.var_fn61_calc_iq__ss * (2.302585092994046 * locals.var_fn61_calc_iq__phitin_dn4)) / (assign5750_e6801 * assign5750_e6801))), (locals.var_fn61_calc_iq__nd * locals.var_fn61_calc_iq__absvdsin_dn15), (locals.var_fn61_calc_iq__nd * locals.var_fn61_calc_iq__absvdsin_dn16),)
    } else {
        (locals.var_fn61_calc_iq__n, locals.var_fn61_calc_iq__n_dn4, locals.var_fn61_calc_iq__n_dn15, locals.var_fn61_calc_iq__n_dn16,)
    }
};
        locals.var_fn61_calc_iq__n = assign5750_e6808;
        locals.var_fn61_calc_iq__n_dn4 = assign5750_e6808_d_n4;
        locals.var_fn61_calc_iq__n_dn15 = assign5750_e6808_d_n15;
        locals.var_fn61_calc_iq__n_dn16 = assign5750_e6808_d_n16;

        let (assign5760_e6818, assign5760_e6818_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5760_e6814: f64 = (locals.var_fn61_calc_iq__tambin - locals.var_fn61_calc_iq__tnomin);
        let assign5760_e6815: f64 = (locals.var_fn61_calc_iq__vtzeta * assign5760_e6814);
        let assign5760_e6816: f64 = (locals.var_fn61_calc_iq__vto + assign5760_e6815);
        (assign5760_e6816, (locals.var_fn61_calc_iq__vtzeta * locals.var_fn61_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn61_calc_iq__vtof, locals.var_fn61_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn61_calc_iq__vtof = assign5760_e6818;
        locals.var_fn61_calc_iq__vtof_dn4 = assign5760_e6818_d_n4;

        let (assign5770_e6826, assign5770_e6826_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5770_e6822: f64 = (locals.var_fn61_calc_iq__tambin / locals.var_fn61_calc_iq__tnomin);
        let assign5770_e6824: f64 = (assign5770_e6822).powf(locals.var_fn61_calc_iq__epsilon);
        (assign5770_e6824, if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn61_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__epsilon * ((assign5770_e6822).powf(locals.var_fn61_calc_iq__epsilon - 1.0) * (locals.var_fn61_calc_iq__tambin_dn4 / locals.var_fn61_calc_iq__tnomin))) } } else { (assign5770_e6824 * (locals.var_fn61_calc_iq__epsilon * ((locals.var_fn61_calc_iq__tambin_dn4 / locals.var_fn61_calc_iq__tnomin) / assign5770_e6822))) },)
    } else {
        (locals.var_fn61_calc_iq__tfacmobin, locals.var_fn61_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn61_calc_iq__tfacmobin = assign5770_e6826;
        locals.var_fn61_calc_iq__tfacmobin_dn4 = assign5770_e6826_d_n4;

        let assign5780_e6829: f64 = if locals.var_fn61_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard62 = assign5780_e6829;

        let (assign5790_e6847, assign5790_e6847_d_n15, assign5790_e6847_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard62 != 0.0)) {
        let assign5790_e6837: f64 = (locals.var_fn61_calc_iq__absvdsin / locals.var_fn61_calc_iq__dibsat);
        let assign5790_e6839: f64 = (assign5790_e6837).powf(locals.var_fn61_calc_iq__beta);
        let assign5790_e6840: f64 = (1.0 + assign5790_e6839);
        let assign5790_e6843: f64 = (1.0 / locals.var_fn61_calc_iq__beta);
        let assign5790_e6844: f64 = (assign5790_e6840).powf(assign5790_e6843);
        let assign5790_e6845: f64 = (locals.var_fn61_calc_iq__absvdsin / assign5790_e6844);
        (assign5790_e6845, (((locals.var_fn61_calc_iq__absvdsin_dn15 * assign5790_e6844) - (locals.var_fn61_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign5790_e6843) as f64).is_finite() && ((assign5790_e6843) as f64).fract() == 0.0 { if assign5790_e6843 == 0.0 { 0.0 } else { (assign5790_e6843 * ((assign5790_e6840).powf(assign5790_e6843 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign5790_e6837).powf(locals.var_fn61_calc_iq__beta - 1.0) * (locals.var_fn61_calc_iq__absvdsin_dn15 / locals.var_fn61_calc_iq__dibsat))) } } else { (assign5790_e6839 * (locals.var_fn61_calc_iq__beta * ((locals.var_fn61_calc_iq__absvdsin_dn15 / locals.var_fn61_calc_iq__dibsat) / assign5790_e6837))) })) } } else { (assign5790_e6844 * (assign5790_e6843 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign5790_e6837).powf(locals.var_fn61_calc_iq__beta - 1.0) * (locals.var_fn61_calc_iq__absvdsin_dn15 / locals.var_fn61_calc_iq__dibsat))) } } else { (assign5790_e6839 * (locals.var_fn61_calc_iq__beta * ((locals.var_fn61_calc_iq__absvdsin_dn15 / locals.var_fn61_calc_iq__dibsat) / assign5790_e6837))) } / assign5790_e6840))) })) / (assign5790_e6844 * assign5790_e6844)), (((locals.var_fn61_calc_iq__absvdsin_dn16 * assign5790_e6844) - (locals.var_fn61_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign5790_e6843) as f64).is_finite() && ((assign5790_e6843) as f64).fract() == 0.0 { if assign5790_e6843 == 0.0 { 0.0 } else { (assign5790_e6843 * ((assign5790_e6840).powf(assign5790_e6843 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign5790_e6837).powf(locals.var_fn61_calc_iq__beta - 1.0) * (locals.var_fn61_calc_iq__absvdsin_dn16 / locals.var_fn61_calc_iq__dibsat))) } } else { (assign5790_e6839 * (locals.var_fn61_calc_iq__beta * ((locals.var_fn61_calc_iq__absvdsin_dn16 / locals.var_fn61_calc_iq__dibsat) / assign5790_e6837))) })) } } else { (assign5790_e6844 * (assign5790_e6843 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign5790_e6837).powf(locals.var_fn61_calc_iq__beta - 1.0) * (locals.var_fn61_calc_iq__absvdsin_dn16 / locals.var_fn61_calc_iq__dibsat))) } } else { (assign5790_e6839 * (locals.var_fn61_calc_iq__beta * ((locals.var_fn61_calc_iq__absvdsin_dn16 / locals.var_fn61_calc_iq__dibsat) / assign5790_e6837))) } / assign5790_e6840))) })) / (assign5790_e6844 * assign5790_e6844)),)
    } else {
        (locals.var_fn61_calc_iq__vsatdibl, locals.var_fn61_calc_iq__vsatdibl_dn15, locals.var_fn61_calc_iq__vsatdibl_dn16,)
    }
};
        locals.var_fn61_calc_iq__vsatdibl = assign5790_e6847;
        locals.var_fn61_calc_iq__vsatdibl_dn15 = assign5790_e6847_d_n15;
        locals.var_fn61_calc_iq__vsatdibl_dn16 = assign5790_e6847_d_n16;

        let (assign5800_e6854, assign5800_e6854_d_n15, assign5800_e6854_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard62 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vsatdibl, locals.var_fn61_calc_iq__vsatdibl_dn15, locals.var_fn61_calc_iq__vsatdibl_dn16,)
    }
};
        locals.var_fn61_calc_iq__vsatdibl = assign5800_e6854;
        locals.var_fn61_calc_iq__vsatdibl_dn15 = assign5800_e6854_d_n15;
        locals.var_fn61_calc_iq__vsatdibl_dn16 = assign5800_e6854_d_n16;

        let (assign5810_e6864, assign5810_e6864_d_n15, assign5810_e6864_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5810_e6859: f64 = (locals.var_fn61_calc_iq__vsatdibl * locals.var_fn61_calc_iq__delta2);
        let assign5810_e6860: f64 = (locals.var_fn61_calc_iq__delta1 - assign5810_e6859);
        let assign5810_e6862: f64 = (assign5810_e6860 * locals.var_fn61_calc_iq__absvdsin);
        (assign5810_e6862, (((-(locals.var_fn61_calc_iq__vsatdibl_dn15 * locals.var_fn61_calc_iq__delta2)) * locals.var_fn61_calc_iq__absvdsin) + (assign5810_e6860 * locals.var_fn61_calc_iq__absvdsin_dn15)), (((-(locals.var_fn61_calc_iq__vsatdibl_dn16 * locals.var_fn61_calc_iq__delta2)) * locals.var_fn61_calc_iq__absvdsin) + (assign5810_e6860 * locals.var_fn61_calc_iq__absvdsin_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__delta, locals.var_fn61_calc_iq__delta_dn15, locals.var_fn61_calc_iq__delta_dn16,)
    }
};
        locals.var_fn61_calc_iq__delta = assign5810_e6864;
        locals.var_fn61_calc_iq__delta_dn15 = assign5810_e6864_d_n15;
        locals.var_fn61_calc_iq__delta_dn16 = assign5810_e6864_d_n16;

        let (assign5820_e6870, assign5820_e6870_d_n4, assign5820_e6870_d_n15, assign5820_e6870_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5820_e6868: f64 = (locals.var_fn61_calc_iq__vtof - locals.var_fn61_calc_iq__delta);
        (assign5820_e6868, locals.var_fn61_calc_iq__vtof_dn4, (-locals.var_fn61_calc_iq__delta_dn15), (-locals.var_fn61_calc_iq__delta_dn16),)
    } else {
        (locals.var_fn61_calc_iq__vtdibl, locals.var_fn61_calc_iq__vtdibl_dn4, locals.var_fn61_calc_iq__vtdibl_dn15, locals.var_fn61_calc_iq__vtdibl_dn16,)
    }
};
        locals.var_fn61_calc_iq__vtdibl = assign5820_e6870;
        locals.var_fn61_calc_iq__vtdibl_dn4 = assign5820_e6870_d_n4;
        locals.var_fn61_calc_iq__vtdibl_dn15 = assign5820_e6870_d_n15;
        locals.var_fn61_calc_iq__vtdibl_dn16 = assign5820_e6870_d_n16;

        let (assign5830_e6878, assign5830_e6878_d_n4, assign5830_e6878_d_n15, assign5830_e6878_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5830_e6874: f64 = (2.0 * locals.var_fn61_calc_iq__n);
        let assign5830_e6876: f64 = (assign5830_e6874 * locals.var_fn61_calc_iq__phitin);
        (assign5830_e6876, (((2.0 * locals.var_fn61_calc_iq__n_dn4) * locals.var_fn61_calc_iq__phitin) + (assign5830_e6874 * locals.var_fn61_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn61_calc_iq__n_dn15) * locals.var_fn61_calc_iq__phitin), ((2.0 * locals.var_fn61_calc_iq__n_dn16) * locals.var_fn61_calc_iq__phitin),)
    } else {
        (locals.var_fn61_calc_iq__two_n_phit, locals.var_fn61_calc_iq__two_n_phit_dn4, locals.var_fn61_calc_iq__two_n_phit_dn15, locals.var_fn61_calc_iq__two_n_phit_dn16,)
    }
};
        locals.var_fn61_calc_iq__two_n_phit = assign5830_e6878;
        locals.var_fn61_calc_iq__two_n_phit_dn4 = assign5830_e6878_d_n4;
        locals.var_fn61_calc_iq__two_n_phit_dn15 = assign5830_e6878_d_n15;
        locals.var_fn61_calc_iq__two_n_phit_dn16 = assign5830_e6878_d_n16;

        let (assign5840_e6884, assign5840_e6884_d_n4, assign5840_e6884_d_n15, assign5840_e6884_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5840_e6882: f64 = (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__two_n_phit);
        (assign5840_e6882, ((locals.var_fn61_calc_iq__cgin_dn4 * locals.var_fn61_calc_iq__two_n_phit) + (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__two_n_phit_dn4)), (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__two_n_phit_dn15), (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__two_n_phit_dn16),)
    } else {
        (locals.var_fn61_calc_iq__qref, locals.var_fn61_calc_iq__qref_dn4, locals.var_fn61_calc_iq__qref_dn15, locals.var_fn61_calc_iq__qref_dn16,)
    }
};
        locals.var_fn61_calc_iq__qref = assign5840_e6884;
        locals.var_fn61_calc_iq__qref_dn4 = assign5840_e6884_d_n4;
        locals.var_fn61_calc_iq__qref_dn15 = assign5840_e6884_d_n15;
        locals.var_fn61_calc_iq__qref_dn16 = assign5840_e6884_d_n16;

        let (assign5850_e6894, assign5850_e6894_d_n2, assign5850_e6894_d_n3, assign5850_e6894_d_n4, assign5850_e6894_d_n7, assign5850_e6894_d_n15, assign5850_e6894_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5850_e6889: f64 = (p.p51 * locals.var_fn61_calc_iq__alpha_phit);
        let assign5850_e6891: f64 = (assign5850_e6889 / 2.0);
        let assign5850_e6892: f64 = (locals.var_fn61_calc_iq__vtdibl - assign5850_e6891);
        (assign5850_e6892, 0.0, 0.0, (locals.var_fn61_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn61_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn61_calc_iq__vtdibl_dn15, locals.var_fn61_calc_iq__vtdibl_dn16,)
    } else {
        (locals.var_fn61_calc_iq__myarg, locals.var_fn61_calc_iq__myarg_dn2, locals.var_fn61_calc_iq__myarg_dn3, locals.var_fn61_calc_iq__myarg_dn4, locals.var_fn61_calc_iq__myarg_dn7, locals.var_fn61_calc_iq__myarg_dn15, locals.var_fn61_calc_iq__myarg_dn16,)
    }
};
        locals.var_fn61_calc_iq__myarg = assign5850_e6894;
        locals.var_fn61_calc_iq__myarg_dn2 = assign5850_e6894_d_n2;
        locals.var_fn61_calc_iq__myarg_dn3 = assign5850_e6894_d_n3;
        locals.var_fn61_calc_iq__myarg_dn4 = assign5850_e6894_d_n4;
        locals.var_fn61_calc_iq__myarg_dn7 = assign5850_e6894_d_n7;
        locals.var_fn61_calc_iq__myarg_dn15 = assign5850_e6894_d_n15;
        locals.var_fn61_calc_iq__myarg_dn16 = assign5850_e6894_d_n16;

        let (assign5860_e6945, assign5860_e6945_d_n2, assign5860_e6945_d_n3, assign5860_e6945_d_n4, assign5860_e6945_d_n7, assign5860_e6945_d_n15, assign5860_e6945_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let (assign5860_e6939, assign5860_e6939_d_n2, assign5860_e6939_d_n7, assign5860_e6939_d_n15, assign5860_e6939_d_n16,) = {
            if (p.p52 != 0.0) {
                let assign5860_e6903: f64 = (locals.var_fn61_calc_iq__vgsin + locals.var_fn61_calc_iq__vgdin);
                let assign5860_e6906: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                let assign5860_e6909: f64 = (0.001 / p.p53);
                let assign5860_e6912: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                let assign5860_e6913: f64 = (assign5860_e6909 * assign5860_e6912);
                let assign5860_e6914: f64 = (assign5860_e6913).tanh();
                let assign5860_e6915: f64 = (assign5860_e6906 * assign5860_e6914);
                let assign5860_e6916: f64 = (assign5860_e6903 + assign5860_e6915);
                let assign5860_e6917: f64 = (0.5 * assign5860_e6916);
                (assign5860_e6917, (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn2 + locals.var_fn61_calc_iq__vgdin_dn2) + (((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2) * assign5860_e6914) + (assign5860_e6906 * ((assign5860_e6909 * (locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2)) / ((assign5860_e6913).cosh() * (assign5860_e6913).cosh())))))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn7 + locals.var_fn61_calc_iq__vgdin_dn7) + (((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7) * assign5860_e6914) + (assign5860_e6906 * ((assign5860_e6909 * (locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7)) / ((assign5860_e6913).cosh() * (assign5860_e6913).cosh())))))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn15 + locals.var_fn61_calc_iq__vgdin_dn15) + (((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15) * assign5860_e6914) + (assign5860_e6906 * ((assign5860_e6909 * (locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15)) / ((assign5860_e6913).cosh() * (assign5860_e6913).cosh())))))), (0.5 * (locals.var_fn61_calc_iq__vgdin_dn16 + (((-locals.var_fn61_calc_iq__vgdin_dn16) * assign5860_e6914) + (assign5860_e6906 * ((assign5860_e6909 * (-locals.var_fn61_calc_iq__vgdin_dn16)) / ((assign5860_e6913).cosh() * (assign5860_e6913).cosh())))))),)
            } else {
                let (assign5860_e6938, assign5860_e6938_d_n2, assign5860_e6938_d_n7, assign5860_e6938_d_n15, assign5860_e6938_d_n16,) = {
                    if (p.p52 == 0.0) {
                        let assign5860_e6924: f64 = (locals.var_fn61_calc_iq__vgsin + locals.var_fn61_calc_iq__vgdin);
                        let assign5860_e6927: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                        let assign5860_e6930: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                        let assign5860_e6931: f64 = (assign5860_e6927 * assign5860_e6930);
                        let assign5860_e6933: f64 = (assign5860_e6931 + p.p53);
                        let assign5860_e6934: f64 = (assign5860_e6933).sqrt();
                        let assign5860_e6935: f64 = (assign5860_e6924 + assign5860_e6934);
                        let assign5860_e6936: f64 = (0.5 * assign5860_e6935);
                        (assign5860_e6936, (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn2 + locals.var_fn61_calc_iq__vgdin_dn2) + ((((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2) * assign5860_e6930) + (assign5860_e6927 * (locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2))) / (2.0 * assign5860_e6934)))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn7 + locals.var_fn61_calc_iq__vgdin_dn7) + ((((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7) * assign5860_e6930) + (assign5860_e6927 * (locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7))) / (2.0 * assign5860_e6934)))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn15 + locals.var_fn61_calc_iq__vgdin_dn15) + ((((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15) * assign5860_e6930) + (assign5860_e6927 * (locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15))) / (2.0 * assign5860_e6934)))), (0.5 * (locals.var_fn61_calc_iq__vgdin_dn16 + ((((-locals.var_fn61_calc_iq__vgdin_dn16) * assign5860_e6930) + (assign5860_e6927 * (-locals.var_fn61_calc_iq__vgdin_dn16))) / (2.0 * assign5860_e6934)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign5860_e6938, assign5860_e6938_d_n2, assign5860_e6938_d_n7, assign5860_e6938_d_n15, assign5860_e6938_d_n16,)
            }
        };
        let assign5860_e6941: f64 = (assign5860_e6939 - locals.var_fn61_calc_iq__myarg);
        let assign5860_e6943: f64 = (assign5860_e6941 / locals.var_fn61_calc_iq__alpha_phit);
        (assign5860_e6943, ((assign5860_e6939_d_n2 - locals.var_fn61_calc_iq__myarg_dn2) / locals.var_fn61_calc_iq__alpha_phit), ((-locals.var_fn61_calc_iq__myarg_dn3) / locals.var_fn61_calc_iq__alpha_phit), ((((-locals.var_fn61_calc_iq__myarg_dn4) * locals.var_fn61_calc_iq__alpha_phit) - (assign5860_e6941 * locals.var_fn61_calc_iq__alpha_phit_dn4)) / (locals.var_fn61_calc_iq__alpha_phit * locals.var_fn61_calc_iq__alpha_phit)), ((assign5860_e6939_d_n7 - locals.var_fn61_calc_iq__myarg_dn7) / locals.var_fn61_calc_iq__alpha_phit), ((assign5860_e6939_d_n15 - locals.var_fn61_calc_iq__myarg_dn15) / locals.var_fn61_calc_iq__alpha_phit), ((assign5860_e6939_d_n16 - locals.var_fn61_calc_iq__myarg_dn16) / locals.var_fn61_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign5860_e6945;
        locals.var_fn61_calc_iq__exparg_dn2 = assign5860_e6945_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign5860_e6945_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign5860_e6945_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign5860_e6945_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign5860_e6945_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign5860_e6945_d_n16;

        let assign5870_e6948: f64 = if locals.var_fn61_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard63 = assign5870_e6948;

        let (assign5880_e6954, assign5880_e6954_d_n2, assign5880_e6954_d_n3, assign5880_e6954_d_n4, assign5880_e6954_d_n7, assign5880_e6954_d_n15, assign5880_e6954_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard63 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ff, locals.var_fn61_calc_iq__ff_dn2, locals.var_fn61_calc_iq__ff_dn3, locals.var_fn61_calc_iq__ff_dn4, locals.var_fn61_calc_iq__ff_dn7, locals.var_fn61_calc_iq__ff_dn15, locals.var_fn61_calc_iq__ff_dn16,)
    }
};
        locals.var_fn61_calc_iq__ff = assign5880_e6954;
        locals.var_fn61_calc_iq__ff_dn2 = assign5880_e6954_d_n2;
        locals.var_fn61_calc_iq__ff_dn3 = assign5880_e6954_d_n3;
        locals.var_fn61_calc_iq__ff_dn4 = assign5880_e6954_d_n4;
        locals.var_fn61_calc_iq__ff_dn7 = assign5880_e6954_d_n7;
        locals.var_fn61_calc_iq__ff_dn15 = assign5880_e6954_d_n15;
        locals.var_fn61_calc_iq__ff_dn16 = assign5880_e6954_d_n16;

        let assign5890_e6957: f64 = (-50.0);
        let assign5890_e6958: f64 = if locals.var_fn61_calc_iq__exparg < assign5890_e6957 { 1.0 } else { 0.0 };
        locals.var_guard64 = assign5890_e6958;

        let (assign5900_e6967, assign5900_e6967_d_n2, assign5900_e6967_d_n3, assign5900_e6967_d_n4, assign5900_e6967_d_n7, assign5900_e6967_d_n15, assign5900_e6967_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard63 == 0.0)) && (locals.var_guard64 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ff, locals.var_fn61_calc_iq__ff_dn2, locals.var_fn61_calc_iq__ff_dn3, locals.var_fn61_calc_iq__ff_dn4, locals.var_fn61_calc_iq__ff_dn7, locals.var_fn61_calc_iq__ff_dn15, locals.var_fn61_calc_iq__ff_dn16,)
    }
};
        locals.var_fn61_calc_iq__ff = assign5900_e6967;
        locals.var_fn61_calc_iq__ff_dn2 = assign5900_e6967_d_n2;
        locals.var_fn61_calc_iq__ff_dn3 = assign5900_e6967_d_n3;
        locals.var_fn61_calc_iq__ff_dn4 = assign5900_e6967_d_n4;
        locals.var_fn61_calc_iq__ff_dn7 = assign5900_e6967_d_n7;
        locals.var_fn61_calc_iq__ff_dn15 = assign5900_e6967_d_n15;
        locals.var_fn61_calc_iq__ff_dn16 = assign5900_e6967_d_n16;

        let (assign5910_e6982, assign5910_e6982_d_n2, assign5910_e6982_d_n3, assign5910_e6982_d_n4, assign5910_e6982_d_n7, assign5910_e6982_d_n15, assign5910_e6982_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard63 == 0.0)) && (locals.var_guard64 == 0.0)) {
        let assign5910_e6978: f64 = (locals.var_fn61_calc_iq__exparg).exp();
        let assign5910_e6979: f64 = (1.0 + assign5910_e6978);
        let assign5910_e6980: f64 = (1.0 / assign5910_e6979);
        (assign5910_e6980, (-((assign5910_e6978 * locals.var_fn61_calc_iq__exparg_dn2) / (assign5910_e6979 * assign5910_e6979))), (-((assign5910_e6978 * locals.var_fn61_calc_iq__exparg_dn3) / (assign5910_e6979 * assign5910_e6979))), (-((assign5910_e6978 * locals.var_fn61_calc_iq__exparg_dn4) / (assign5910_e6979 * assign5910_e6979))), (-((assign5910_e6978 * locals.var_fn61_calc_iq__exparg_dn7) / (assign5910_e6979 * assign5910_e6979))), (-((assign5910_e6978 * locals.var_fn61_calc_iq__exparg_dn15) / (assign5910_e6979 * assign5910_e6979))), (-((assign5910_e6978 * locals.var_fn61_calc_iq__exparg_dn16) / (assign5910_e6979 * assign5910_e6979))),)
    } else {
        (locals.var_fn61_calc_iq__ff, locals.var_fn61_calc_iq__ff_dn2, locals.var_fn61_calc_iq__ff_dn3, locals.var_fn61_calc_iq__ff_dn4, locals.var_fn61_calc_iq__ff_dn7, locals.var_fn61_calc_iq__ff_dn15, locals.var_fn61_calc_iq__ff_dn16,)
    }
};
        locals.var_fn61_calc_iq__ff = assign5910_e6982;
        locals.var_fn61_calc_iq__ff_dn2 = assign5910_e6982_d_n2;
        locals.var_fn61_calc_iq__ff_dn3 = assign5910_e6982_d_n3;
        locals.var_fn61_calc_iq__ff_dn4 = assign5910_e6982_d_n4;
        locals.var_fn61_calc_iq__ff_dn7 = assign5910_e6982_d_n7;
        locals.var_fn61_calc_iq__ff_dn15 = assign5910_e6982_d_n15;
        locals.var_fn61_calc_iq__ff_dn16 = assign5910_e6982_d_n16;

        let (assign5920_e7041, assign5920_e7041_d_n2, assign5920_e7041_d_n3, assign5920_e7041_d_n4, assign5920_e7041_d_n7, assign5920_e7041_d_n15, assign5920_e7041_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let (assign5920_e7027, assign5920_e7027_d_n2, assign5920_e7027_d_n7, assign5920_e7027_d_n15, assign5920_e7027_d_n16,) = {
            if (p.p52 != 0.0) {
                let assign5920_e6991: f64 = (locals.var_fn61_calc_iq__vgsin + locals.var_fn61_calc_iq__vgdin);
                let assign5920_e6994: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                let assign5920_e6997: f64 = (0.001 / p.p53);
                let assign5920_e7000: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                let assign5920_e7001: f64 = (assign5920_e6997 * assign5920_e7000);
                let assign5920_e7002: f64 = (assign5920_e7001).tanh();
                let assign5920_e7003: f64 = (assign5920_e6994 * assign5920_e7002);
                let assign5920_e7004: f64 = (assign5920_e6991 + assign5920_e7003);
                let assign5920_e7005: f64 = (0.5 * assign5920_e7004);
                (assign5920_e7005, (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn2 + locals.var_fn61_calc_iq__vgdin_dn2) + (((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2) * assign5920_e7002) + (assign5920_e6994 * ((assign5920_e6997 * (locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2)) / ((assign5920_e7001).cosh() * (assign5920_e7001).cosh())))))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn7 + locals.var_fn61_calc_iq__vgdin_dn7) + (((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7) * assign5920_e7002) + (assign5920_e6994 * ((assign5920_e6997 * (locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7)) / ((assign5920_e7001).cosh() * (assign5920_e7001).cosh())))))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn15 + locals.var_fn61_calc_iq__vgdin_dn15) + (((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15) * assign5920_e7002) + (assign5920_e6994 * ((assign5920_e6997 * (locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15)) / ((assign5920_e7001).cosh() * (assign5920_e7001).cosh())))))), (0.5 * (locals.var_fn61_calc_iq__vgdin_dn16 + (((-locals.var_fn61_calc_iq__vgdin_dn16) * assign5920_e7002) + (assign5920_e6994 * ((assign5920_e6997 * (-locals.var_fn61_calc_iq__vgdin_dn16)) / ((assign5920_e7001).cosh() * (assign5920_e7001).cosh())))))),)
            } else {
                let (assign5920_e7026, assign5920_e7026_d_n2, assign5920_e7026_d_n7, assign5920_e7026_d_n15, assign5920_e7026_d_n16,) = {
                    if (p.p52 == 0.0) {
                        let assign5920_e7012: f64 = (locals.var_fn61_calc_iq__vgsin + locals.var_fn61_calc_iq__vgdin);
                        let assign5920_e7015: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                        let assign5920_e7018: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                        let assign5920_e7019: f64 = (assign5920_e7015 * assign5920_e7018);
                        let assign5920_e7021: f64 = (assign5920_e7019 + p.p53);
                        let assign5920_e7022: f64 = (assign5920_e7021).sqrt();
                        let assign5920_e7023: f64 = (assign5920_e7012 + assign5920_e7022);
                        let assign5920_e7024: f64 = (0.5 * assign5920_e7023);
                        (assign5920_e7024, (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn2 + locals.var_fn61_calc_iq__vgdin_dn2) + ((((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2) * assign5920_e7018) + (assign5920_e7015 * (locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2))) / (2.0 * assign5920_e7022)))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn7 + locals.var_fn61_calc_iq__vgdin_dn7) + ((((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7) * assign5920_e7018) + (assign5920_e7015 * (locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7))) / (2.0 * assign5920_e7022)))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn15 + locals.var_fn61_calc_iq__vgdin_dn15) + ((((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15) * assign5920_e7018) + (assign5920_e7015 * (locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15))) / (2.0 * assign5920_e7022)))), (0.5 * (locals.var_fn61_calc_iq__vgdin_dn16 + ((((-locals.var_fn61_calc_iq__vgdin_dn16) * assign5920_e7018) + (assign5920_e7015 * (-locals.var_fn61_calc_iq__vgdin_dn16))) / (2.0 * assign5920_e7022)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign5920_e7026, assign5920_e7026_d_n2, assign5920_e7026_d_n7, assign5920_e7026_d_n15, assign5920_e7026_d_n16,)
            }
        };
        let assign5920_e7031: f64 = (p.p51 * 0.1);
        let assign5920_e7033: f64 = (assign5920_e7031 * locals.var_fn61_calc_iq__alpha_phit);
        let assign5920_e7035: f64 = (assign5920_e7033 * locals.var_fn61_calc_iq__ff);
        let assign5920_e7036: f64 = (locals.var_fn61_calc_iq__vtdibl - assign5920_e7035);
        let assign5920_e7037: f64 = (assign5920_e7027 - assign5920_e7036);
        let assign5920_e7039: f64 = (assign5920_e7037 / locals.var_fn61_calc_iq__two_n_phit);
        (assign5920_e7039, ((assign5920_e7027_d_n2 - (-(assign5920_e7033 * locals.var_fn61_calc_iq__ff_dn2))) / locals.var_fn61_calc_iq__two_n_phit), ((-(-(assign5920_e7033 * locals.var_fn61_calc_iq__ff_dn3))) / locals.var_fn61_calc_iq__two_n_phit), ((((-(locals.var_fn61_calc_iq__vtdibl_dn4 - (((assign5920_e7031 * locals.var_fn61_calc_iq__alpha_phit_dn4) * locals.var_fn61_calc_iq__ff) + (assign5920_e7033 * locals.var_fn61_calc_iq__ff_dn4)))) * locals.var_fn61_calc_iq__two_n_phit) - (assign5920_e7037 * locals.var_fn61_calc_iq__two_n_phit_dn4)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)), ((assign5920_e7027_d_n7 - (-(assign5920_e7033 * locals.var_fn61_calc_iq__ff_dn7))) / locals.var_fn61_calc_iq__two_n_phit), ((((assign5920_e7027_d_n15 - (locals.var_fn61_calc_iq__vtdibl_dn15 - (assign5920_e7033 * locals.var_fn61_calc_iq__ff_dn15))) * locals.var_fn61_calc_iq__two_n_phit) - (assign5920_e7037 * locals.var_fn61_calc_iq__two_n_phit_dn15)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)), ((((assign5920_e7027_d_n16 - (locals.var_fn61_calc_iq__vtdibl_dn16 - (assign5920_e7033 * locals.var_fn61_calc_iq__ff_dn16))) * locals.var_fn61_calc_iq__two_n_phit) - (assign5920_e7037 * locals.var_fn61_calc_iq__two_n_phit_dn16)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn61_calc_iq__eta, locals.var_fn61_calc_iq__eta_dn2, locals.var_fn61_calc_iq__eta_dn3, locals.var_fn61_calc_iq__eta_dn4, locals.var_fn61_calc_iq__eta_dn7, locals.var_fn61_calc_iq__eta_dn15, locals.var_fn61_calc_iq__eta_dn16,)
    }
};
        locals.var_fn61_calc_iq__eta = assign5920_e7041;
        locals.var_fn61_calc_iq__eta_dn2 = assign5920_e7041_d_n2;
        locals.var_fn61_calc_iq__eta_dn3 = assign5920_e7041_d_n3;
        locals.var_fn61_calc_iq__eta_dn4 = assign5920_e7041_d_n4;
        locals.var_fn61_calc_iq__eta_dn7 = assign5920_e7041_d_n7;
        locals.var_fn61_calc_iq__eta_dn15 = assign5920_e7041_d_n15;
        locals.var_fn61_calc_iq__eta_dn16 = assign5920_e7041_d_n16;

        let assign5930_e7044: f64 = if locals.var_fn61_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard65 = assign5930_e7044;

        let (assign5940_e7052, assign5940_e7052_d_n2, assign5940_e7052_d_n3, assign5940_e7052_d_n4, assign5940_e7052_d_n7, assign5940_e7052_d_n15, assign5940_e7052_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard65 != 0.0)) {
        let assign5940_e7050: f64 = (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__eta);
        (assign5940_e7050, (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__eta_dn2), (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__eta_dn3), ((locals.var_fn61_calc_iq__qref_dn4 * locals.var_fn61_calc_iq__eta) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__eta_dn4)), (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__eta_dn7), ((locals.var_fn61_calc_iq__qref_dn15 * locals.var_fn61_calc_iq__eta) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__eta_dn15)), ((locals.var_fn61_calc_iq__qref_dn16 * locals.var_fn61_calc_iq__eta) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__eta_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qinvv, locals.var_fn61_calc_iq__qinvv_dn2, locals.var_fn61_calc_iq__qinvv_dn3, locals.var_fn61_calc_iq__qinvv_dn4, locals.var_fn61_calc_iq__qinvv_dn7, locals.var_fn61_calc_iq__qinvv_dn15, locals.var_fn61_calc_iq__qinvv_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvv = assign5940_e7052;
        locals.var_fn61_calc_iq__qinvv_dn2 = assign5940_e7052_d_n2;
        locals.var_fn61_calc_iq__qinvv_dn3 = assign5940_e7052_d_n3;
        locals.var_fn61_calc_iq__qinvv_dn4 = assign5940_e7052_d_n4;
        locals.var_fn61_calc_iq__qinvv_dn7 = assign5940_e7052_d_n7;
        locals.var_fn61_calc_iq__qinvv_dn15 = assign5940_e7052_d_n15;
        locals.var_fn61_calc_iq__qinvv_dn16 = assign5940_e7052_d_n16;

        let assign5950_e7055: f64 = (-50.0);
        let assign5950_e7056: f64 = if locals.var_fn61_calc_iq__eta < assign5950_e7055 { 1.0 } else { 0.0 };
        locals.var_guard66 = assign5950_e7056;

        let (assign5960_e7068, assign5960_e7068_d_n2, assign5960_e7068_d_n3, assign5960_e7068_d_n4, assign5960_e7068_d_n7, assign5960_e7068_d_n15, assign5960_e7068_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard65 == 0.0)) && (locals.var_guard66 != 0.0)) {
        let assign5960_e7065: f64 = (locals.var_fn61_calc_iq__eta).exp();
        let assign5960_e7066: f64 = (locals.var_fn61_calc_iq__qref * assign5960_e7065);
        (assign5960_e7066, (locals.var_fn61_calc_iq__qref * (assign5960_e7065 * locals.var_fn61_calc_iq__eta_dn2)), (locals.var_fn61_calc_iq__qref * (assign5960_e7065 * locals.var_fn61_calc_iq__eta_dn3)), ((locals.var_fn61_calc_iq__qref_dn4 * assign5960_e7065) + (locals.var_fn61_calc_iq__qref * (assign5960_e7065 * locals.var_fn61_calc_iq__eta_dn4))), (locals.var_fn61_calc_iq__qref * (assign5960_e7065 * locals.var_fn61_calc_iq__eta_dn7)), ((locals.var_fn61_calc_iq__qref_dn15 * assign5960_e7065) + (locals.var_fn61_calc_iq__qref * (assign5960_e7065 * locals.var_fn61_calc_iq__eta_dn15))), ((locals.var_fn61_calc_iq__qref_dn16 * assign5960_e7065) + (locals.var_fn61_calc_iq__qref * (assign5960_e7065 * locals.var_fn61_calc_iq__eta_dn16))),)
    } else {
        (locals.var_fn61_calc_iq__qinvv, locals.var_fn61_calc_iq__qinvv_dn2, locals.var_fn61_calc_iq__qinvv_dn3, locals.var_fn61_calc_iq__qinvv_dn4, locals.var_fn61_calc_iq__qinvv_dn7, locals.var_fn61_calc_iq__qinvv_dn15, locals.var_fn61_calc_iq__qinvv_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvv = assign5960_e7068;
        locals.var_fn61_calc_iq__qinvv_dn2 = assign5960_e7068_d_n2;
        locals.var_fn61_calc_iq__qinvv_dn3 = assign5960_e7068_d_n3;
        locals.var_fn61_calc_iq__qinvv_dn4 = assign5960_e7068_d_n4;
        locals.var_fn61_calc_iq__qinvv_dn7 = assign5960_e7068_d_n7;
        locals.var_fn61_calc_iq__qinvv_dn15 = assign5960_e7068_d_n15;
        locals.var_fn61_calc_iq__qinvv_dn16 = assign5960_e7068_d_n16;

        let (assign5970_e7084, assign5970_e7084_d_n2, assign5970_e7084_d_n3, assign5970_e7084_d_n4, assign5970_e7084_d_n7, assign5970_e7084_d_n15, assign5970_e7084_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard65 == 0.0)) && (locals.var_guard66 == 0.0)) {
        let assign5970_e7079: f64 = (locals.var_fn61_calc_iq__eta).exp();
        let assign5970_e7080: f64 = (1.0 + assign5970_e7079);
        let assign5970_e7081: f64 = (assign5970_e7080).ln();
        let assign5970_e7082: f64 = (locals.var_fn61_calc_iq__qref * assign5970_e7081);
        (assign5970_e7082, (locals.var_fn61_calc_iq__qref * ((assign5970_e7079 * locals.var_fn61_calc_iq__eta_dn2) / assign5970_e7080)), (locals.var_fn61_calc_iq__qref * ((assign5970_e7079 * locals.var_fn61_calc_iq__eta_dn3) / assign5970_e7080)), ((locals.var_fn61_calc_iq__qref_dn4 * assign5970_e7081) + (locals.var_fn61_calc_iq__qref * ((assign5970_e7079 * locals.var_fn61_calc_iq__eta_dn4) / assign5970_e7080))), (locals.var_fn61_calc_iq__qref * ((assign5970_e7079 * locals.var_fn61_calc_iq__eta_dn7) / assign5970_e7080)), ((locals.var_fn61_calc_iq__qref_dn15 * assign5970_e7081) + (locals.var_fn61_calc_iq__qref * ((assign5970_e7079 * locals.var_fn61_calc_iq__eta_dn15) / assign5970_e7080))), ((locals.var_fn61_calc_iq__qref_dn16 * assign5970_e7081) + (locals.var_fn61_calc_iq__qref * ((assign5970_e7079 * locals.var_fn61_calc_iq__eta_dn16) / assign5970_e7080))),)
    } else {
        (locals.var_fn61_calc_iq__qinvv, locals.var_fn61_calc_iq__qinvv_dn2, locals.var_fn61_calc_iq__qinvv_dn3, locals.var_fn61_calc_iq__qinvv_dn4, locals.var_fn61_calc_iq__qinvv_dn7, locals.var_fn61_calc_iq__qinvv_dn15, locals.var_fn61_calc_iq__qinvv_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvv = assign5970_e7084;
        locals.var_fn61_calc_iq__qinvv_dn2 = assign5970_e7084_d_n2;
        locals.var_fn61_calc_iq__qinvv_dn3 = assign5970_e7084_d_n3;
        locals.var_fn61_calc_iq__qinvv_dn4 = assign5970_e7084_d_n4;
        locals.var_fn61_calc_iq__qinvv_dn7 = assign5970_e7084_d_n7;
        locals.var_fn61_calc_iq__qinvv_dn15 = assign5970_e7084_d_n15;
        locals.var_fn61_calc_iq__qinvv_dn16 = assign5970_e7084_d_n16;

    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5980_e7098, assign5980_e7098_d_n2, assign5980_e7098_d_n3, assign5980_e7098_d_n4, assign5980_e7098_d_n7, assign5980_e7098_d_n15, assign5980_e7098_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5980_e7091: f64 = (locals.var_fn61_calc_iq__mtheta * locals.var_fn61_calc_iq__qinvv);
        let assign5980_e7093: f64 = (assign5980_e7091 / locals.var_fn61_calc_iq__cgin);
        let assign5980_e7094: f64 = (1.0 + assign5980_e7093);
        let assign5980_e7095: f64 = (locals.var_fn61_calc_iq__tfacmobin * assign5980_e7094);
        let assign5980_e7096: f64 = (locals.var_fn61_calc_iq__mu0 / assign5980_e7095);
        (assign5980_e7096, (-((locals.var_fn61_calc_iq__mu0 * (locals.var_fn61_calc_iq__tfacmobin * ((locals.var_fn61_calc_iq__mtheta * locals.var_fn61_calc_iq__qinvv_dn2) / locals.var_fn61_calc_iq__cgin))) / (assign5980_e7095 * assign5980_e7095))), (-((locals.var_fn61_calc_iq__mu0 * (locals.var_fn61_calc_iq__tfacmobin * ((locals.var_fn61_calc_iq__mtheta * locals.var_fn61_calc_iq__qinvv_dn3) / locals.var_fn61_calc_iq__cgin))) / (assign5980_e7095 * assign5980_e7095))), (-((locals.var_fn61_calc_iq__mu0 * ((locals.var_fn61_calc_iq__tfacmobin_dn4 * assign5980_e7094) + (locals.var_fn61_calc_iq__tfacmobin * ((((locals.var_fn61_calc_iq__mtheta * locals.var_fn61_calc_iq__qinvv_dn4) * locals.var_fn61_calc_iq__cgin) - (assign5980_e7091 * locals.var_fn61_calc_iq__cgin_dn4)) / (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__cgin))))) / (assign5980_e7095 * assign5980_e7095))), (-((locals.var_fn61_calc_iq__mu0 * (locals.var_fn61_calc_iq__tfacmobin * ((locals.var_fn61_calc_iq__mtheta * locals.var_fn61_calc_iq__qinvv_dn7) / locals.var_fn61_calc_iq__cgin))) / (assign5980_e7095 * assign5980_e7095))), (-((locals.var_fn61_calc_iq__mu0 * (locals.var_fn61_calc_iq__tfacmobin * ((locals.var_fn61_calc_iq__mtheta * locals.var_fn61_calc_iq__qinvv_dn15) / locals.var_fn61_calc_iq__cgin))) / (assign5980_e7095 * assign5980_e7095))), (-((locals.var_fn61_calc_iq__mu0 * (locals.var_fn61_calc_iq__tfacmobin * ((locals.var_fn61_calc_iq__mtheta * locals.var_fn61_calc_iq__qinvv_dn16) / locals.var_fn61_calc_iq__cgin))) / (assign5980_e7095 * assign5980_e7095))),)
    } else {
        (locals.var_fn61_calc_iq__muf, locals.var_fn61_calc_iq__muf_dn2, locals.var_fn61_calc_iq__muf_dn3, locals.var_fn61_calc_iq__muf_dn4, locals.var_fn61_calc_iq__muf_dn7, locals.var_fn61_calc_iq__muf_dn15, locals.var_fn61_calc_iq__muf_dn16,)
    }
};
        locals.var_fn61_calc_iq__muf = assign5980_e7098;
        locals.var_fn61_calc_iq__muf_dn2 = assign5980_e7098_d_n2;
        locals.var_fn61_calc_iq__muf_dn3 = assign5980_e7098_d_n3;
        locals.var_fn61_calc_iq__muf_dn4 = assign5980_e7098_d_n4;
        locals.var_fn61_calc_iq__muf_dn7 = assign5980_e7098_d_n7;
        locals.var_fn61_calc_iq__muf_dn15 = assign5980_e7098_d_n15;
        locals.var_fn61_calc_iq__muf_dn16 = assign5980_e7098_d_n16;

        let (assign5990_e7130, assign5990_e7130_d_n2, assign5990_e7130_d_n3, assign5990_e7130_d_n4, assign5990_e7130_d_n7, assign5990_e7130_d_n15, assign5990_e7130_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5990_e7104: f64 = (locals.var_fn61_calc_iq__vzeta * locals.var_fn61_calc_iq__tnomin);
        let assign5990_e7105: f64 = (1.0 + assign5990_e7104);
        let assign5990_e7109: f64 = (locals.var_fn61_calc_iq__vzeta * locals.var_fn61_calc_iq__tambin);
        let assign5990_e7110: f64 = (1.0 + assign5990_e7109);
        let assign5990_e7111: f64 = (assign5990_e7105 / assign5990_e7110);
        let assign5990_e7112: f64 = (locals.var_fn61_calc_iq__vel0 * assign5990_e7111);
        let assign5990_e7116: f64 = (locals.var_fn61_calc_iq__lambda * locals.var_fn61_calc_iq__absvdsin);
        let assign5990_e7118: f64 = (assign5990_e7116 / locals.var_fn61_calc_iq__lin);
        let assign5990_e7119: f64 = (1.0 + assign5990_e7118);
        let assign5990_e7120: f64 = (assign5990_e7112 * assign5990_e7119);
        let assign5990_e7124: f64 = (locals.var_fn61_calc_iq__vtheta * locals.var_fn61_calc_iq__qinvv);
        let assign5990_e7126: f64 = (assign5990_e7124 / locals.var_fn61_calc_iq__cgin);
        let assign5990_e7127: f64 = (1.0 + assign5990_e7126);
        let assign5990_e7128: f64 = (assign5990_e7120 / assign5990_e7127);
        (assign5990_e7128, (-((assign5990_e7120 * ((locals.var_fn61_calc_iq__vtheta * locals.var_fn61_calc_iq__qinvv_dn2) / locals.var_fn61_calc_iq__cgin)) / (assign5990_e7127 * assign5990_e7127))), (-((assign5990_e7120 * ((locals.var_fn61_calc_iq__vtheta * locals.var_fn61_calc_iq__qinvv_dn3) / locals.var_fn61_calc_iq__cgin)) / (assign5990_e7127 * assign5990_e7127))), (((((locals.var_fn61_calc_iq__vel0 * (-((assign5990_e7105 * (locals.var_fn61_calc_iq__vzeta * locals.var_fn61_calc_iq__tambin_dn4)) / (assign5990_e7110 * assign5990_e7110)))) * assign5990_e7119) * assign5990_e7127) - (assign5990_e7120 * ((((locals.var_fn61_calc_iq__vtheta * locals.var_fn61_calc_iq__qinvv_dn4) * locals.var_fn61_calc_iq__cgin) - (assign5990_e7124 * locals.var_fn61_calc_iq__cgin_dn4)) / (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__cgin)))) / (assign5990_e7127 * assign5990_e7127)), (-((assign5990_e7120 * ((locals.var_fn61_calc_iq__vtheta * locals.var_fn61_calc_iq__qinvv_dn7) / locals.var_fn61_calc_iq__cgin)) / (assign5990_e7127 * assign5990_e7127))), ((((assign5990_e7112 * ((locals.var_fn61_calc_iq__lambda * locals.var_fn61_calc_iq__absvdsin_dn15) / locals.var_fn61_calc_iq__lin)) * assign5990_e7127) - (assign5990_e7120 * ((locals.var_fn61_calc_iq__vtheta * locals.var_fn61_calc_iq__qinvv_dn15) / locals.var_fn61_calc_iq__cgin))) / (assign5990_e7127 * assign5990_e7127)), ((((assign5990_e7112 * ((locals.var_fn61_calc_iq__lambda * locals.var_fn61_calc_iq__absvdsin_dn16) / locals.var_fn61_calc_iq__lin)) * assign5990_e7127) - (assign5990_e7120 * ((locals.var_fn61_calc_iq__vtheta * locals.var_fn61_calc_iq__qinvv_dn16) / locals.var_fn61_calc_iq__cgin))) / (assign5990_e7127 * assign5990_e7127)),)
    } else {
        (locals.var_fn61_calc_iq__vx, locals.var_fn61_calc_iq__vx_dn2, locals.var_fn61_calc_iq__vx_dn3, locals.var_fn61_calc_iq__vx_dn4, locals.var_fn61_calc_iq__vx_dn7, locals.var_fn61_calc_iq__vx_dn15, locals.var_fn61_calc_iq__vx_dn16,)
    }
};
        locals.var_fn61_calc_iq__vx = assign5990_e7130;
        locals.var_fn61_calc_iq__vx_dn2 = assign5990_e7130_d_n2;
        locals.var_fn61_calc_iq__vx_dn3 = assign5990_e7130_d_n3;
        locals.var_fn61_calc_iq__vx_dn4 = assign5990_e7130_d_n4;
        locals.var_fn61_calc_iq__vx_dn7 = assign5990_e7130_d_n7;
        locals.var_fn61_calc_iq__vx_dn15 = assign5990_e7130_d_n15;
        locals.var_fn61_calc_iq__vx_dn16 = assign5990_e7130_d_n16;

        let (assign6000_e7148, assign6000_e7148_d_n2, assign6000_e7148_d_n3, assign6000_e7148_d_n4, assign6000_e7148_d_n7, assign6000_e7148_d_n15, assign6000_e7148_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6000_e7134: f64 = (2.0 * locals.var_fn61_calc_iq__ff);
        let assign6000_e7136: f64 = (assign6000_e7134 * locals.var_fn61_calc_iq__phitin);
        let assign6000_e7138: f64 = (assign6000_e7136 * locals.var_fn61_calc_iq__muf);
        let assign6000_e7140: f64 = (assign6000_e7138 / locals.var_fn61_calc_iq__lin);
        let assign6000_e7143: f64 = (1.0 - locals.var_fn61_calc_iq__ff);
        let assign6000_e7145: f64 = (assign6000_e7143 * locals.var_fn61_calc_iq__vx);
        let assign6000_e7146: f64 = (assign6000_e7140 + assign6000_e7145);
        (assign6000_e7146, ((((((2.0 * locals.var_fn61_calc_iq__ff_dn2) * locals.var_fn61_calc_iq__phitin) * locals.var_fn61_calc_iq__muf) + (assign6000_e7136 * locals.var_fn61_calc_iq__muf_dn2)) / locals.var_fn61_calc_iq__lin) + (((-locals.var_fn61_calc_iq__ff_dn2) * locals.var_fn61_calc_iq__vx) + (assign6000_e7143 * locals.var_fn61_calc_iq__vx_dn2))), ((((((2.0 * locals.var_fn61_calc_iq__ff_dn3) * locals.var_fn61_calc_iq__phitin) * locals.var_fn61_calc_iq__muf) + (assign6000_e7136 * locals.var_fn61_calc_iq__muf_dn3)) / locals.var_fn61_calc_iq__lin) + (((-locals.var_fn61_calc_iq__ff_dn3) * locals.var_fn61_calc_iq__vx) + (assign6000_e7143 * locals.var_fn61_calc_iq__vx_dn3))), (((((((2.0 * locals.var_fn61_calc_iq__ff_dn4) * locals.var_fn61_calc_iq__phitin) + (assign6000_e7134 * locals.var_fn61_calc_iq__phitin_dn4)) * locals.var_fn61_calc_iq__muf) + (assign6000_e7136 * locals.var_fn61_calc_iq__muf_dn4)) / locals.var_fn61_calc_iq__lin) + (((-locals.var_fn61_calc_iq__ff_dn4) * locals.var_fn61_calc_iq__vx) + (assign6000_e7143 * locals.var_fn61_calc_iq__vx_dn4))), ((((((2.0 * locals.var_fn61_calc_iq__ff_dn7) * locals.var_fn61_calc_iq__phitin) * locals.var_fn61_calc_iq__muf) + (assign6000_e7136 * locals.var_fn61_calc_iq__muf_dn7)) / locals.var_fn61_calc_iq__lin) + (((-locals.var_fn61_calc_iq__ff_dn7) * locals.var_fn61_calc_iq__vx) + (assign6000_e7143 * locals.var_fn61_calc_iq__vx_dn7))), ((((((2.0 * locals.var_fn61_calc_iq__ff_dn15) * locals.var_fn61_calc_iq__phitin) * locals.var_fn61_calc_iq__muf) + (assign6000_e7136 * locals.var_fn61_calc_iq__muf_dn15)) / locals.var_fn61_calc_iq__lin) + (((-locals.var_fn61_calc_iq__ff_dn15) * locals.var_fn61_calc_iq__vx) + (assign6000_e7143 * locals.var_fn61_calc_iq__vx_dn15))), ((((((2.0 * locals.var_fn61_calc_iq__ff_dn16) * locals.var_fn61_calc_iq__phitin) * locals.var_fn61_calc_iq__muf) + (assign6000_e7136 * locals.var_fn61_calc_iq__muf_dn16)) / locals.var_fn61_calc_iq__lin) + (((-locals.var_fn61_calc_iq__ff_dn16) * locals.var_fn61_calc_iq__vx) + (assign6000_e7143 * locals.var_fn61_calc_iq__vx_dn16))),)
    } else {
        (locals.var_fn61_calc_iq__vxf, locals.var_fn61_calc_iq__vxf_dn2, locals.var_fn61_calc_iq__vxf_dn3, locals.var_fn61_calc_iq__vxf_dn4, locals.var_fn61_calc_iq__vxf_dn7, locals.var_fn61_calc_iq__vxf_dn15, locals.var_fn61_calc_iq__vxf_dn16,)
    }
};
        locals.var_fn61_calc_iq__vxf = assign6000_e7148;
        locals.var_fn61_calc_iq__vxf_dn2 = assign6000_e7148_d_n2;
        locals.var_fn61_calc_iq__vxf_dn3 = assign6000_e7148_d_n3;
        locals.var_fn61_calc_iq__vxf_dn4 = assign6000_e7148_d_n4;
        locals.var_fn61_calc_iq__vxf_dn7 = assign6000_e7148_d_n7;
        locals.var_fn61_calc_iq__vxf_dn15 = assign6000_e7148_d_n15;
        locals.var_fn61_calc_iq__vxf_dn16 = assign6000_e7148_d_n16;

        let (assign6010_e7156, assign6010_e7156_d_n2, assign6010_e7156_d_n3, assign6010_e7156_d_n4, assign6010_e7156_d_n7, assign6010_e7156_d_n15, assign6010_e7156_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6010_e7152: f64 = (locals.var_fn61_calc_iq__vx * locals.var_fn61_calc_iq__lin);
        let assign6010_e7154: f64 = (assign6010_e7152 / locals.var_fn61_calc_iq__muf);
        (assign6010_e7154, ((((locals.var_fn61_calc_iq__vx_dn2 * locals.var_fn61_calc_iq__lin) * locals.var_fn61_calc_iq__muf) - (assign6010_e7152 * locals.var_fn61_calc_iq__muf_dn2)) / (locals.var_fn61_calc_iq__muf * locals.var_fn61_calc_iq__muf)), ((((locals.var_fn61_calc_iq__vx_dn3 * locals.var_fn61_calc_iq__lin) * locals.var_fn61_calc_iq__muf) - (assign6010_e7152 * locals.var_fn61_calc_iq__muf_dn3)) / (locals.var_fn61_calc_iq__muf * locals.var_fn61_calc_iq__muf)), ((((locals.var_fn61_calc_iq__vx_dn4 * locals.var_fn61_calc_iq__lin) * locals.var_fn61_calc_iq__muf) - (assign6010_e7152 * locals.var_fn61_calc_iq__muf_dn4)) / (locals.var_fn61_calc_iq__muf * locals.var_fn61_calc_iq__muf)), ((((locals.var_fn61_calc_iq__vx_dn7 * locals.var_fn61_calc_iq__lin) * locals.var_fn61_calc_iq__muf) - (assign6010_e7152 * locals.var_fn61_calc_iq__muf_dn7)) / (locals.var_fn61_calc_iq__muf * locals.var_fn61_calc_iq__muf)), ((((locals.var_fn61_calc_iq__vx_dn15 * locals.var_fn61_calc_iq__lin) * locals.var_fn61_calc_iq__muf) - (assign6010_e7152 * locals.var_fn61_calc_iq__muf_dn15)) / (locals.var_fn61_calc_iq__muf * locals.var_fn61_calc_iq__muf)), ((((locals.var_fn61_calc_iq__vx_dn16 * locals.var_fn61_calc_iq__lin) * locals.var_fn61_calc_iq__muf) - (assign6010_e7152 * locals.var_fn61_calc_iq__muf_dn16)) / (locals.var_fn61_calc_iq__muf * locals.var_fn61_calc_iq__muf)),)
    } else {
        (locals.var_fn61_calc_iq__vdsats, locals.var_fn61_calc_iq__vdsats_dn2, locals.var_fn61_calc_iq__vdsats_dn3, locals.var_fn61_calc_iq__vdsats_dn4, locals.var_fn61_calc_iq__vdsats_dn7, locals.var_fn61_calc_iq__vdsats_dn15, locals.var_fn61_calc_iq__vdsats_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsats = assign6010_e7156;
        locals.var_fn61_calc_iq__vdsats_dn2 = assign6010_e7156_d_n2;
        locals.var_fn61_calc_iq__vdsats_dn3 = assign6010_e7156_d_n3;
        locals.var_fn61_calc_iq__vdsats_dn4 = assign6010_e7156_d_n4;
        locals.var_fn61_calc_iq__vdsats_dn7 = assign6010_e7156_d_n7;
        locals.var_fn61_calc_iq__vdsats_dn15 = assign6010_e7156_d_n15;
        locals.var_fn61_calc_iq__vdsats_dn16 = assign6010_e7156_d_n16;

        let (assign6020_e7173, assign6020_e7173_d_n2, assign6020_e7173_d_n3, assign6020_e7173_d_n4, assign6020_e7173_d_n7, assign6020_e7173_d_n15, assign6020_e7173_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6020_e7162: f64 = (2.0 * locals.var_fn61_calc_iq__qinvv);
        let assign6020_e7164: f64 = (assign6020_e7162 / locals.var_fn61_calc_iq__cgin);
        let assign6020_e7166: f64 = (assign6020_e7164 / locals.var_fn61_calc_iq__vdsats);
        let assign6020_e7167: f64 = (1.0 + assign6020_e7166);
        let assign6020_e7168: f64 = (assign6020_e7167).sqrt();
        let assign6020_e7169: f64 = (locals.var_fn61_calc_iq__vdsats * assign6020_e7168);
        let assign6020_e7171: f64 = (assign6020_e7169 - locals.var_fn61_calc_iq__vdsats);
        (assign6020_e7171, (((locals.var_fn61_calc_iq__vdsats_dn2 * assign6020_e7168) + (locals.var_fn61_calc_iq__vdsats * ((((((2.0 * locals.var_fn61_calc_iq__qinvv_dn2) / locals.var_fn61_calc_iq__cgin) * locals.var_fn61_calc_iq__vdsats) - (assign6020_e7164 * locals.var_fn61_calc_iq__vdsats_dn2)) / (locals.var_fn61_calc_iq__vdsats * locals.var_fn61_calc_iq__vdsats)) / (2.0 * assign6020_e7168)))) - locals.var_fn61_calc_iq__vdsats_dn2), (((locals.var_fn61_calc_iq__vdsats_dn3 * assign6020_e7168) + (locals.var_fn61_calc_iq__vdsats * ((((((2.0 * locals.var_fn61_calc_iq__qinvv_dn3) / locals.var_fn61_calc_iq__cgin) * locals.var_fn61_calc_iq__vdsats) - (assign6020_e7164 * locals.var_fn61_calc_iq__vdsats_dn3)) / (locals.var_fn61_calc_iq__vdsats * locals.var_fn61_calc_iq__vdsats)) / (2.0 * assign6020_e7168)))) - locals.var_fn61_calc_iq__vdsats_dn3), (((locals.var_fn61_calc_iq__vdsats_dn4 * assign6020_e7168) + (locals.var_fn61_calc_iq__vdsats * ((((((((2.0 * locals.var_fn61_calc_iq__qinvv_dn4) * locals.var_fn61_calc_iq__cgin) - (assign6020_e7162 * locals.var_fn61_calc_iq__cgin_dn4)) / (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__cgin)) * locals.var_fn61_calc_iq__vdsats) - (assign6020_e7164 * locals.var_fn61_calc_iq__vdsats_dn4)) / (locals.var_fn61_calc_iq__vdsats * locals.var_fn61_calc_iq__vdsats)) / (2.0 * assign6020_e7168)))) - locals.var_fn61_calc_iq__vdsats_dn4), (((locals.var_fn61_calc_iq__vdsats_dn7 * assign6020_e7168) + (locals.var_fn61_calc_iq__vdsats * ((((((2.0 * locals.var_fn61_calc_iq__qinvv_dn7) / locals.var_fn61_calc_iq__cgin) * locals.var_fn61_calc_iq__vdsats) - (assign6020_e7164 * locals.var_fn61_calc_iq__vdsats_dn7)) / (locals.var_fn61_calc_iq__vdsats * locals.var_fn61_calc_iq__vdsats)) / (2.0 * assign6020_e7168)))) - locals.var_fn61_calc_iq__vdsats_dn7), (((locals.var_fn61_calc_iq__vdsats_dn15 * assign6020_e7168) + (locals.var_fn61_calc_iq__vdsats * ((((((2.0 * locals.var_fn61_calc_iq__qinvv_dn15) / locals.var_fn61_calc_iq__cgin) * locals.var_fn61_calc_iq__vdsats) - (assign6020_e7164 * locals.var_fn61_calc_iq__vdsats_dn15)) / (locals.var_fn61_calc_iq__vdsats * locals.var_fn61_calc_iq__vdsats)) / (2.0 * assign6020_e7168)))) - locals.var_fn61_calc_iq__vdsats_dn15), (((locals.var_fn61_calc_iq__vdsats_dn16 * assign6020_e7168) + (locals.var_fn61_calc_iq__vdsats * ((((((2.0 * locals.var_fn61_calc_iq__qinvv_dn16) / locals.var_fn61_calc_iq__cgin) * locals.var_fn61_calc_iq__vdsats) - (assign6020_e7164 * locals.var_fn61_calc_iq__vdsats_dn16)) / (locals.var_fn61_calc_iq__vdsats * locals.var_fn61_calc_iq__vdsats)) / (2.0 * assign6020_e7168)))) - locals.var_fn61_calc_iq__vdsats_dn16),)
    } else {
        (locals.var_fn61_calc_iq__vdsats1, locals.var_fn61_calc_iq__vdsats1_dn2, locals.var_fn61_calc_iq__vdsats1_dn3, locals.var_fn61_calc_iq__vdsats1_dn4, locals.var_fn61_calc_iq__vdsats1_dn7, locals.var_fn61_calc_iq__vdsats1_dn15, locals.var_fn61_calc_iq__vdsats1_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsats1 = assign6020_e7173;
        locals.var_fn61_calc_iq__vdsats1_dn2 = assign6020_e7173_d_n2;
        locals.var_fn61_calc_iq__vdsats1_dn3 = assign6020_e7173_d_n3;
        locals.var_fn61_calc_iq__vdsats1_dn4 = assign6020_e7173_d_n4;
        locals.var_fn61_calc_iq__vdsats1_dn7 = assign6020_e7173_d_n7;
        locals.var_fn61_calc_iq__vdsats1_dn15 = assign6020_e7173_d_n15;
        locals.var_fn61_calc_iq__vdsats1_dn16 = assign6020_e7173_d_n16;

        let (assign6030_e7185, assign6030_e7185_d_n2, assign6030_e7185_d_n3, assign6030_e7185_d_n4, assign6030_e7185_d_n7, assign6030_e7185_d_n15, assign6030_e7185_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6030_e7178: f64 = (1.0 - locals.var_fn61_calc_iq__ff);
        let assign6030_e7179: f64 = (locals.var_fn61_calc_iq__vdsats * assign6030_e7178);
        let assign6030_e7182: f64 = (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff);
        let assign6030_e7183: f64 = (assign6030_e7179 + assign6030_e7182);
        (assign6030_e7183, (((locals.var_fn61_calc_iq__vdsats_dn2 * assign6030_e7178) + (locals.var_fn61_calc_iq__vdsats * (-locals.var_fn61_calc_iq__ff_dn2))) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn2)), (((locals.var_fn61_calc_iq__vdsats_dn3 * assign6030_e7178) + (locals.var_fn61_calc_iq__vdsats * (-locals.var_fn61_calc_iq__ff_dn3))) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn3)), (((locals.var_fn61_calc_iq__vdsats_dn4 * assign6030_e7178) + (locals.var_fn61_calc_iq__vdsats * (-locals.var_fn61_calc_iq__ff_dn4))) + ((locals.var_fn61_calc_iq__two_n_phit_dn4 * locals.var_fn61_calc_iq__ff) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn4))), (((locals.var_fn61_calc_iq__vdsats_dn7 * assign6030_e7178) + (locals.var_fn61_calc_iq__vdsats * (-locals.var_fn61_calc_iq__ff_dn7))) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn7)), (((locals.var_fn61_calc_iq__vdsats_dn15 * assign6030_e7178) + (locals.var_fn61_calc_iq__vdsats * (-locals.var_fn61_calc_iq__ff_dn15))) + ((locals.var_fn61_calc_iq__two_n_phit_dn15 * locals.var_fn61_calc_iq__ff) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn15))), (((locals.var_fn61_calc_iq__vdsats_dn16 * assign6030_e7178) + (locals.var_fn61_calc_iq__vdsats * (-locals.var_fn61_calc_iq__ff_dn16))) + ((locals.var_fn61_calc_iq__two_n_phit_dn16 * locals.var_fn61_calc_iq__ff) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn16))),)
    } else {
        (locals.var_fn61_calc_iq__vdsat, locals.var_fn61_calc_iq__vdsat_dn2, locals.var_fn61_calc_iq__vdsat_dn3, locals.var_fn61_calc_iq__vdsat_dn4, locals.var_fn61_calc_iq__vdsat_dn7, locals.var_fn61_calc_iq__vdsat_dn15, locals.var_fn61_calc_iq__vdsat_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsat = assign6030_e7185;
        locals.var_fn61_calc_iq__vdsat_dn2 = assign6030_e7185_d_n2;
        locals.var_fn61_calc_iq__vdsat_dn3 = assign6030_e7185_d_n3;
        locals.var_fn61_calc_iq__vdsat_dn4 = assign6030_e7185_d_n4;
        locals.var_fn61_calc_iq__vdsat_dn7 = assign6030_e7185_d_n7;
        locals.var_fn61_calc_iq__vdsat_dn15 = assign6030_e7185_d_n15;
        locals.var_fn61_calc_iq__vdsat_dn16 = assign6030_e7185_d_n16;

        let (assign6040_e7197, assign6040_e7197_d_n2, assign6040_e7197_d_n3, assign6040_e7197_d_n4, assign6040_e7197_d_n7, assign6040_e7197_d_n15, assign6040_e7197_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6040_e7190: f64 = (1.0 - locals.var_fn61_calc_iq__ff);
        let assign6040_e7191: f64 = (locals.var_fn61_calc_iq__vdsats1 * assign6040_e7190);
        let assign6040_e7194: f64 = (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff);
        let assign6040_e7195: f64 = (assign6040_e7191 + assign6040_e7194);
        (assign6040_e7195, (((locals.var_fn61_calc_iq__vdsats1_dn2 * assign6040_e7190) + (locals.var_fn61_calc_iq__vdsats1 * (-locals.var_fn61_calc_iq__ff_dn2))) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn2)), (((locals.var_fn61_calc_iq__vdsats1_dn3 * assign6040_e7190) + (locals.var_fn61_calc_iq__vdsats1 * (-locals.var_fn61_calc_iq__ff_dn3))) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn3)), (((locals.var_fn61_calc_iq__vdsats1_dn4 * assign6040_e7190) + (locals.var_fn61_calc_iq__vdsats1 * (-locals.var_fn61_calc_iq__ff_dn4))) + ((locals.var_fn61_calc_iq__two_n_phit_dn4 * locals.var_fn61_calc_iq__ff) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn4))), (((locals.var_fn61_calc_iq__vdsats1_dn7 * assign6040_e7190) + (locals.var_fn61_calc_iq__vdsats1 * (-locals.var_fn61_calc_iq__ff_dn7))) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn7)), (((locals.var_fn61_calc_iq__vdsats1_dn15 * assign6040_e7190) + (locals.var_fn61_calc_iq__vdsats1 * (-locals.var_fn61_calc_iq__ff_dn15))) + ((locals.var_fn61_calc_iq__two_n_phit_dn15 * locals.var_fn61_calc_iq__ff) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn15))), (((locals.var_fn61_calc_iq__vdsats1_dn16 * assign6040_e7190) + (locals.var_fn61_calc_iq__vdsats1 * (-locals.var_fn61_calc_iq__ff_dn16))) + ((locals.var_fn61_calc_iq__two_n_phit_dn16 * locals.var_fn61_calc_iq__ff) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn16))),)
    } else {
        (locals.var_fn61_calc_iq__vdsat1, locals.var_fn61_calc_iq__vdsat1_dn2, locals.var_fn61_calc_iq__vdsat1_dn3, locals.var_fn61_calc_iq__vdsat1_dn4, locals.var_fn61_calc_iq__vdsat1_dn7, locals.var_fn61_calc_iq__vdsat1_dn15, locals.var_fn61_calc_iq__vdsat1_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsat1 = assign6040_e7197;
        locals.var_fn61_calc_iq__vdsat1_dn2 = assign6040_e7197_d_n2;
        locals.var_fn61_calc_iq__vdsat1_dn3 = assign6040_e7197_d_n3;
        locals.var_fn61_calc_iq__vdsat1_dn4 = assign6040_e7197_d_n4;
        locals.var_fn61_calc_iq__vdsat1_dn7 = assign6040_e7197_d_n7;
        locals.var_fn61_calc_iq__vdsat1_dn15 = assign6040_e7197_d_n15;
        locals.var_fn61_calc_iq__vdsat1_dn16 = assign6040_e7197_d_n16;

        let (assign6050_e7266, assign6050_e7266_d_n2, assign6050_e7266_d_n3, assign6050_e7266_d_n4, assign6050_e7266_d_n7, assign6050_e7266_d_n15, assign6050_e7266_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let (assign6050_e7256, assign6050_e7256_d_n2, assign6050_e7256_d_n3, assign6050_e7256_d_n4, assign6050_e7256_d_n7, assign6050_e7256_d_n15, assign6050_e7256_d_n16,) = {
            if (p.p52 != 0.0) {
                let assign6050_e7209: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat1);
                let assign6050_e7210: f64 = assign6050_e7209;
                let assign6050_e7214: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat1);
                let assign6050_e7215: f64 = (-assign6050_e7214);
                let assign6050_e7218: f64 = (0.001 / p.p53);
                let assign6050_e7222: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat1);
                let assign6050_e7223: f64 = (-assign6050_e7222);
                let assign6050_e7224: f64 = (assign6050_e7218 * assign6050_e7223);
                let assign6050_e7225: f64 = (assign6050_e7224).tanh();
                let assign6050_e7226: f64 = (assign6050_e7215 * assign6050_e7225);
                let assign6050_e7227: f64 = (assign6050_e7210 + assign6050_e7226);
                let assign6050_e7228: f64 = (0.5 * assign6050_e7227);
                (assign6050_e7228, (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7225) + (assign6050_e7215 * ((assign6050_e7218 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6050_e7224).cosh() * (assign6050_e7224).cosh())))))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7225) + (assign6050_e7215 * ((assign6050_e7218 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6050_e7224).cosh() * (assign6050_e7224).cosh())))))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7225) + (assign6050_e7215 * ((assign6050_e7218 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6050_e7224).cosh() * (assign6050_e7224).cosh())))))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7225) + (assign6050_e7215 * ((assign6050_e7218 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6050_e7224).cosh() * (assign6050_e7224).cosh())))))), (0.5 * ((((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + (((-(((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6050_e7225) + (assign6050_e7215 * ((assign6050_e7218 * (-(((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) / ((assign6050_e7224).cosh() * (assign6050_e7224).cosh())))))), (0.5 * ((((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + (((-(((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6050_e7225) + (assign6050_e7215 * ((assign6050_e7218 * (-(((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) / ((assign6050_e7224).cosh() * (assign6050_e7224).cosh())))))),)
            } else {
                let (assign6050_e7255, assign6050_e7255_d_n2, assign6050_e7255_d_n3, assign6050_e7255_d_n4, assign6050_e7255_d_n7, assign6050_e7255_d_n15, assign6050_e7255_d_n16,) = {
                    if (p.p52 == 0.0) {
                        let assign6050_e7236: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat1);
                        let assign6050_e7237: f64 = assign6050_e7236;
                        let assign6050_e7241: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat1);
                        let assign6050_e7242: f64 = (-assign6050_e7241);
                        let assign6050_e7246: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat1);
                        let assign6050_e7247: f64 = (-assign6050_e7246);
                        let assign6050_e7248: f64 = (assign6050_e7242 * assign6050_e7247);
                        let assign6050_e7250: f64 = (assign6050_e7248 + p.p53);
                        let assign6050_e7251: f64 = (assign6050_e7250).sqrt();
                        let assign6050_e7252: f64 = (assign6050_e7237 + assign6050_e7251);
                        let assign6050_e7253: f64 = (0.5 * assign6050_e7252);
                        (assign6050_e7253, (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7247) + (assign6050_e7242 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6050_e7251)))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7247) + (assign6050_e7242 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6050_e7251)))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7247) + (assign6050_e7242 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6050_e7251)))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7247) + (assign6050_e7242 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6050_e7251)))), (0.5 * ((((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + ((((-(((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6050_e7247) + (assign6050_e7242 * (-(((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / (2.0 * assign6050_e7251)))), (0.5 * ((((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + ((((-(((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6050_e7247) + (assign6050_e7242 * (-(((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / (2.0 * assign6050_e7251)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign6050_e7255, assign6050_e7255_d_n2, assign6050_e7255_d_n3, assign6050_e7255_d_n4, assign6050_e7255_d_n7, assign6050_e7255_d_n15, assign6050_e7255_d_n16,)
            }
        };
        let assign6050_e7258: f64 = (assign6050_e7256).powf(locals.var_fn61_calc_iq__beta);
        let assign6050_e7259: f64 = (1.0 + assign6050_e7258);
        let assign6050_e7262: f64 = (1.0 / locals.var_fn61_calc_iq__beta);
        let assign6050_e7263: f64 = (assign6050_e7259).powf(assign6050_e7262);
        let assign6050_e7264: f64 = (1.0 / assign6050_e7263);
        (assign6050_e7264, (-(if 0.0 == 0.0 && ((assign6050_e7262) as f64).is_finite() && ((assign6050_e7262) as f64).fract() == 0.0 { if assign6050_e7262 == 0.0 { 0.0 } else { (assign6050_e7262 * ((assign6050_e7259).powf(assign6050_e7262 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n2)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n2 / assign6050_e7256))) })) } } else { (assign6050_e7263 * (assign6050_e7262 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n2)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n2 / assign6050_e7256))) } / assign6050_e7259))) } / (assign6050_e7263 * assign6050_e7263))), (-(if 0.0 == 0.0 && ((assign6050_e7262) as f64).is_finite() && ((assign6050_e7262) as f64).fract() == 0.0 { if assign6050_e7262 == 0.0 { 0.0 } else { (assign6050_e7262 * ((assign6050_e7259).powf(assign6050_e7262 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n3)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n3 / assign6050_e7256))) })) } } else { (assign6050_e7263 * (assign6050_e7262 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n3)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n3 / assign6050_e7256))) } / assign6050_e7259))) } / (assign6050_e7263 * assign6050_e7263))), (-(if 0.0 == 0.0 && ((assign6050_e7262) as f64).is_finite() && ((assign6050_e7262) as f64).fract() == 0.0 { if assign6050_e7262 == 0.0 { 0.0 } else { (assign6050_e7262 * ((assign6050_e7259).powf(assign6050_e7262 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n4)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n4 / assign6050_e7256))) })) } } else { (assign6050_e7263 * (assign6050_e7262 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n4)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n4 / assign6050_e7256))) } / assign6050_e7259))) } / (assign6050_e7263 * assign6050_e7263))), (-(if 0.0 == 0.0 && ((assign6050_e7262) as f64).is_finite() && ((assign6050_e7262) as f64).fract() == 0.0 { if assign6050_e7262 == 0.0 { 0.0 } else { (assign6050_e7262 * ((assign6050_e7259).powf(assign6050_e7262 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n7)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n7 / assign6050_e7256))) })) } } else { (assign6050_e7263 * (assign6050_e7262 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n7)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n7 / assign6050_e7256))) } / assign6050_e7259))) } / (assign6050_e7263 * assign6050_e7263))), (-(if 0.0 == 0.0 && ((assign6050_e7262) as f64).is_finite() && ((assign6050_e7262) as f64).fract() == 0.0 { if assign6050_e7262 == 0.0 { 0.0 } else { (assign6050_e7262 * ((assign6050_e7259).powf(assign6050_e7262 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n15)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n15 / assign6050_e7256))) })) } } else { (assign6050_e7263 * (assign6050_e7262 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n15)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n15 / assign6050_e7256))) } / assign6050_e7259))) } / (assign6050_e7263 * assign6050_e7263))), (-(if 0.0 == 0.0 && ((assign6050_e7262) as f64).is_finite() && ((assign6050_e7262) as f64).fract() == 0.0 { if assign6050_e7262 == 0.0 { 0.0 } else { (assign6050_e7262 * ((assign6050_e7259).powf(assign6050_e7262 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n16)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n16 / assign6050_e7256))) })) } } else { (assign6050_e7263 * (assign6050_e7262 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n16)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n16 / assign6050_e7256))) } / assign6050_e7259))) } / (assign6050_e7263 * assign6050_e7263))),)
    } else {
        (locals.var_fn61_calc_iq__fsd, locals.var_fn61_calc_iq__fsd_dn2, locals.var_fn61_calc_iq__fsd_dn3, locals.var_fn61_calc_iq__fsd_dn4, locals.var_fn61_calc_iq__fsd_dn7, locals.var_fn61_calc_iq__fsd_dn15, locals.var_fn61_calc_iq__fsd_dn16,)
    }
};
        locals.var_fn61_calc_iq__fsd = assign6050_e7266;
        locals.var_fn61_calc_iq__fsd_dn2 = assign6050_e7266_d_n2;
        locals.var_fn61_calc_iq__fsd_dn3 = assign6050_e7266_d_n3;
        locals.var_fn61_calc_iq__fsd_dn4 = assign6050_e7266_d_n4;
        locals.var_fn61_calc_iq__fsd_dn7 = assign6050_e7266_d_n7;
        locals.var_fn61_calc_iq__fsd_dn15 = assign6050_e7266_d_n15;
        locals.var_fn61_calc_iq__fsd_dn16 = assign6050_e7266_d_n16;

        let (assign6060_e7272, assign6060_e7272_d_n2, assign6060_e7272_d_n3, assign6060_e7272_d_n4, assign6060_e7272_d_n7, assign6060_e7272_d_n15, assign6060_e7272_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6060_e7270: f64 = (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd);
        (assign6060_e7270, (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd_dn2), (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd_dn3), (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd_dn4), (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd_dn7), ((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__fsd) + (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd_dn15)), ((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__fsd) + (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__vdx, locals.var_fn61_calc_iq__vdx_dn2, locals.var_fn61_calc_iq__vdx_dn3, locals.var_fn61_calc_iq__vdx_dn4, locals.var_fn61_calc_iq__vdx_dn7, locals.var_fn61_calc_iq__vdx_dn15, locals.var_fn61_calc_iq__vdx_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdx = assign6060_e7272;
        locals.var_fn61_calc_iq__vdx_dn2 = assign6060_e7272_d_n2;
        locals.var_fn61_calc_iq__vdx_dn3 = assign6060_e7272_d_n3;
        locals.var_fn61_calc_iq__vdx_dn4 = assign6060_e7272_d_n4;
        locals.var_fn61_calc_iq__vdx_dn7 = assign6060_e7272_d_n7;
        locals.var_fn61_calc_iq__vdx_dn15 = assign6060_e7272_d_n15;
        locals.var_fn61_calc_iq__vdx_dn16 = assign6060_e7272_d_n16;

        let (assign6070_e7347, assign6070_e7347_d_n2, assign6070_e7347_d_n3, assign6070_e7347_d_n4, assign6070_e7347_d_n7, assign6070_e7347_d_n15, assign6070_e7347_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let (assign6070_e7337, assign6070_e7337_d_n2, assign6070_e7337_d_n3, assign6070_e7337_d_n4, assign6070_e7337_d_n7, assign6070_e7337_d_n15, assign6070_e7337_d_n16,) = {
            if (p.p52 != 0.0) {
                let assign6070_e7283: f64 = (-locals.var_fn61_calc_iq__vdsin);
                let assign6070_e7285: f64 = (assign6070_e7283 / locals.var_fn61_calc_iq__vdsat1);
                let assign6070_e7286: f64 = assign6070_e7285;
                let assign6070_e7289: f64 = (-locals.var_fn61_calc_iq__vdsin);
                let assign6070_e7291: f64 = (assign6070_e7289 / locals.var_fn61_calc_iq__vdsat1);
                let assign6070_e7292: f64 = (-assign6070_e7291);
                let assign6070_e7295: f64 = (0.001 / p.p53);
                let assign6070_e7298: f64 = (-locals.var_fn61_calc_iq__vdsin);
                let assign6070_e7300: f64 = (assign6070_e7298 / locals.var_fn61_calc_iq__vdsat1);
                let assign6070_e7301: f64 = (-assign6070_e7300);
                let assign6070_e7302: f64 = (assign6070_e7295 * assign6070_e7301);
                let assign6070_e7303: f64 = (assign6070_e7302).tanh();
                let assign6070_e7304: f64 = (assign6070_e7292 * assign6070_e7303);
                let assign6070_e7305: f64 = (assign6070_e7286 + assign6070_e7304);
                let assign6070_e7306: f64 = (0.5 * assign6070_e7305);
                (assign6070_e7306, (0.5 * ((-((assign6070_e7283 * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((assign6070_e7289 * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7303) + (assign6070_e7292 * ((assign6070_e7295 * (-(-((assign6070_e7298 * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6070_e7302).cosh() * (assign6070_e7302).cosh())))))), (0.5 * ((-((assign6070_e7283 * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((assign6070_e7289 * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7303) + (assign6070_e7292 * ((assign6070_e7295 * (-(-((assign6070_e7298 * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6070_e7302).cosh() * (assign6070_e7302).cosh())))))), (0.5 * ((-((assign6070_e7283 * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((assign6070_e7289 * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7303) + (assign6070_e7292 * ((assign6070_e7295 * (-(-((assign6070_e7298 * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6070_e7302).cosh() * (assign6070_e7302).cosh())))))), (0.5 * ((-((assign6070_e7283 * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((assign6070_e7289 * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7303) + (assign6070_e7292 * ((assign6070_e7295 * (-(-((assign6070_e7298 * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6070_e7302).cosh() * (assign6070_e7302).cosh())))))), (0.5 * (((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7283 * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + (((-((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7289 * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6070_e7303) + (assign6070_e7292 * ((assign6070_e7295 * (-((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7298 * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) / ((assign6070_e7302).cosh() * (assign6070_e7302).cosh())))))), (0.5 * (((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7283 * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + (((-((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7289 * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6070_e7303) + (assign6070_e7292 * ((assign6070_e7295 * (-((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7298 * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) / ((assign6070_e7302).cosh() * (assign6070_e7302).cosh())))))),)
            } else {
                let (assign6070_e7336, assign6070_e7336_d_n2, assign6070_e7336_d_n3, assign6070_e7336_d_n4, assign6070_e7336_d_n7, assign6070_e7336_d_n15, assign6070_e7336_d_n16,) = {
                    if (p.p52 == 0.0) {
                        let assign6070_e7313: f64 = (-locals.var_fn61_calc_iq__vdsin);
                        let assign6070_e7315: f64 = (assign6070_e7313 / locals.var_fn61_calc_iq__vdsat1);
                        let assign6070_e7316: f64 = assign6070_e7315;
                        let assign6070_e7319: f64 = (-locals.var_fn61_calc_iq__vdsin);
                        let assign6070_e7321: f64 = (assign6070_e7319 / locals.var_fn61_calc_iq__vdsat1);
                        let assign6070_e7322: f64 = (-assign6070_e7321);
                        let assign6070_e7325: f64 = (-locals.var_fn61_calc_iq__vdsin);
                        let assign6070_e7327: f64 = (assign6070_e7325 / locals.var_fn61_calc_iq__vdsat1);
                        let assign6070_e7328: f64 = (-assign6070_e7327);
                        let assign6070_e7329: f64 = (assign6070_e7322 * assign6070_e7328);
                        let assign6070_e7331: f64 = (assign6070_e7329 + p.p53);
                        let assign6070_e7332: f64 = (assign6070_e7331).sqrt();
                        let assign6070_e7333: f64 = (assign6070_e7316 + assign6070_e7332);
                        let assign6070_e7334: f64 = (0.5 * assign6070_e7333);
                        (assign6070_e7334, (0.5 * ((-((assign6070_e7313 * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((assign6070_e7319 * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7328) + (assign6070_e7322 * (-(-((assign6070_e7325 * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6070_e7332)))), (0.5 * ((-((assign6070_e7313 * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((assign6070_e7319 * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7328) + (assign6070_e7322 * (-(-((assign6070_e7325 * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6070_e7332)))), (0.5 * ((-((assign6070_e7313 * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((assign6070_e7319 * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7328) + (assign6070_e7322 * (-(-((assign6070_e7325 * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6070_e7332)))), (0.5 * ((-((assign6070_e7313 * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((assign6070_e7319 * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7328) + (assign6070_e7322 * (-(-((assign6070_e7325 * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6070_e7332)))), (0.5 * (((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7313 * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + ((((-((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7319 * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6070_e7328) + (assign6070_e7322 * (-((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7325 * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / (2.0 * assign6070_e7332)))), (0.5 * (((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7313 * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + ((((-((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7319 * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6070_e7328) + (assign6070_e7322 * (-((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7325 * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / (2.0 * assign6070_e7332)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign6070_e7336, assign6070_e7336_d_n2, assign6070_e7336_d_n3, assign6070_e7336_d_n4, assign6070_e7336_d_n7, assign6070_e7336_d_n15, assign6070_e7336_d_n16,)
            }
        };
        let assign6070_e7339: f64 = (assign6070_e7337).powf(locals.var_fn61_calc_iq__beta);
        let assign6070_e7340: f64 = (1.0 + assign6070_e7339);
        let assign6070_e7343: f64 = (1.0 / locals.var_fn61_calc_iq__beta);
        let assign6070_e7344: f64 = (assign6070_e7340).powf(assign6070_e7343);
        let assign6070_e7345: f64 = (1.0 / assign6070_e7344);
        (assign6070_e7345, (-(if 0.0 == 0.0 && ((assign6070_e7343) as f64).is_finite() && ((assign6070_e7343) as f64).fract() == 0.0 { if assign6070_e7343 == 0.0 { 0.0 } else { (assign6070_e7343 * ((assign6070_e7340).powf(assign6070_e7343 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n2)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n2 / assign6070_e7337))) })) } } else { (assign6070_e7344 * (assign6070_e7343 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n2)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n2 / assign6070_e7337))) } / assign6070_e7340))) } / (assign6070_e7344 * assign6070_e7344))), (-(if 0.0 == 0.0 && ((assign6070_e7343) as f64).is_finite() && ((assign6070_e7343) as f64).fract() == 0.0 { if assign6070_e7343 == 0.0 { 0.0 } else { (assign6070_e7343 * ((assign6070_e7340).powf(assign6070_e7343 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n3)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n3 / assign6070_e7337))) })) } } else { (assign6070_e7344 * (assign6070_e7343 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n3)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n3 / assign6070_e7337))) } / assign6070_e7340))) } / (assign6070_e7344 * assign6070_e7344))), (-(if 0.0 == 0.0 && ((assign6070_e7343) as f64).is_finite() && ((assign6070_e7343) as f64).fract() == 0.0 { if assign6070_e7343 == 0.0 { 0.0 } else { (assign6070_e7343 * ((assign6070_e7340).powf(assign6070_e7343 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n4)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n4 / assign6070_e7337))) })) } } else { (assign6070_e7344 * (assign6070_e7343 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n4)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n4 / assign6070_e7337))) } / assign6070_e7340))) } / (assign6070_e7344 * assign6070_e7344))), (-(if 0.0 == 0.0 && ((assign6070_e7343) as f64).is_finite() && ((assign6070_e7343) as f64).fract() == 0.0 { if assign6070_e7343 == 0.0 { 0.0 } else { (assign6070_e7343 * ((assign6070_e7340).powf(assign6070_e7343 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n7)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n7 / assign6070_e7337))) })) } } else { (assign6070_e7344 * (assign6070_e7343 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n7)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n7 / assign6070_e7337))) } / assign6070_e7340))) } / (assign6070_e7344 * assign6070_e7344))), (-(if 0.0 == 0.0 && ((assign6070_e7343) as f64).is_finite() && ((assign6070_e7343) as f64).fract() == 0.0 { if assign6070_e7343 == 0.0 { 0.0 } else { (assign6070_e7343 * ((assign6070_e7340).powf(assign6070_e7343 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n15)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n15 / assign6070_e7337))) })) } } else { (assign6070_e7344 * (assign6070_e7343 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n15)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n15 / assign6070_e7337))) } / assign6070_e7340))) } / (assign6070_e7344 * assign6070_e7344))), (-(if 0.0 == 0.0 && ((assign6070_e7343) as f64).is_finite() && ((assign6070_e7343) as f64).fract() == 0.0 { if assign6070_e7343 == 0.0 { 0.0 } else { (assign6070_e7343 * ((assign6070_e7340).powf(assign6070_e7343 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n16)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n16 / assign6070_e7337))) })) } } else { (assign6070_e7344 * (assign6070_e7343 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n16)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n16 / assign6070_e7337))) } / assign6070_e7340))) } / (assign6070_e7344 * assign6070_e7344))),)
    } else {
        (locals.var_fn61_calc_iq__fds, locals.var_fn61_calc_iq__fds_dn2, locals.var_fn61_calc_iq__fds_dn3, locals.var_fn61_calc_iq__fds_dn4, locals.var_fn61_calc_iq__fds_dn7, locals.var_fn61_calc_iq__fds_dn15, locals.var_fn61_calc_iq__fds_dn16,)
    }
};
        locals.var_fn61_calc_iq__fds = assign6070_e7347;
        locals.var_fn61_calc_iq__fds_dn2 = assign6070_e7347_d_n2;
        locals.var_fn61_calc_iq__fds_dn3 = assign6070_e7347_d_n3;
        locals.var_fn61_calc_iq__fds_dn4 = assign6070_e7347_d_n4;
        locals.var_fn61_calc_iq__fds_dn7 = assign6070_e7347_d_n7;
        locals.var_fn61_calc_iq__fds_dn15 = assign6070_e7347_d_n15;
        locals.var_fn61_calc_iq__fds_dn16 = assign6070_e7347_d_n16;

        let (assign6080_e7354, assign6080_e7354_d_n2, assign6080_e7354_d_n3, assign6080_e7354_d_n4, assign6080_e7354_d_n7, assign6080_e7354_d_n15, assign6080_e7354_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6080_e7350: f64 = (-locals.var_fn61_calc_iq__vdsin);
        let assign6080_e7352: f64 = (assign6080_e7350 * locals.var_fn61_calc_iq__fds);
        (assign6080_e7352, (assign6080_e7350 * locals.var_fn61_calc_iq__fds_dn2), (assign6080_e7350 * locals.var_fn61_calc_iq__fds_dn3), (assign6080_e7350 * locals.var_fn61_calc_iq__fds_dn4), (assign6080_e7350 * locals.var_fn61_calc_iq__fds_dn7), (((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__fds) + (assign6080_e7350 * locals.var_fn61_calc_iq__fds_dn15)), (((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__fds) + (assign6080_e7350 * locals.var_fn61_calc_iq__fds_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__vsx, locals.var_fn61_calc_iq__vsx_dn2, locals.var_fn61_calc_iq__vsx_dn3, locals.var_fn61_calc_iq__vsx_dn4, locals.var_fn61_calc_iq__vsx_dn7, locals.var_fn61_calc_iq__vsx_dn15, locals.var_fn61_calc_iq__vsx_dn16,)
    }
};
        locals.var_fn61_calc_iq__vsx = assign6080_e7354;
        locals.var_fn61_calc_iq__vsx_dn2 = assign6080_e7354_d_n2;
        locals.var_fn61_calc_iq__vsx_dn3 = assign6080_e7354_d_n3;
        locals.var_fn61_calc_iq__vsx_dn4 = assign6080_e7354_d_n4;
        locals.var_fn61_calc_iq__vsx_dn7 = assign6080_e7354_d_n7;
        locals.var_fn61_calc_iq__vsx_dn15 = assign6080_e7354_d_n15;
        locals.var_fn61_calc_iq__vsx_dn16 = assign6080_e7354_d_n16;

        let (assign6090_e7362, assign6090_e7362_d_n2, assign6090_e7362_d_n3, assign6090_e7362_d_n4, assign6090_e7362_d_n7, assign6090_e7362_d_n15, assign6090_e7362_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6090_e7358: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__myarg);
        let assign6090_e7360: f64 = (assign6090_e7358 / locals.var_fn61_calc_iq__alpha_phit);
        (assign6090_e7360, ((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__myarg_dn2) / locals.var_fn61_calc_iq__alpha_phit), ((-locals.var_fn61_calc_iq__myarg_dn3) / locals.var_fn61_calc_iq__alpha_phit), ((((-locals.var_fn61_calc_iq__myarg_dn4) * locals.var_fn61_calc_iq__alpha_phit) - (assign6090_e7358 * locals.var_fn61_calc_iq__alpha_phit_dn4)) / (locals.var_fn61_calc_iq__alpha_phit * locals.var_fn61_calc_iq__alpha_phit)), ((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__myarg_dn7) / locals.var_fn61_calc_iq__alpha_phit), ((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__myarg_dn15) / locals.var_fn61_calc_iq__alpha_phit), ((-locals.var_fn61_calc_iq__myarg_dn16) / locals.var_fn61_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign6090_e7362;
        locals.var_fn61_calc_iq__exparg_dn2 = assign6090_e7362_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign6090_e7362_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign6090_e7362_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign6090_e7362_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign6090_e7362_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign6090_e7362_d_n16;

        let assign6100_e7365: f64 = if locals.var_fn61_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard67 = assign6100_e7365;

        let (assign6110_e7371, assign6110_e7371_d_n2, assign6110_e7371_d_n3, assign6110_e7371_d_n4, assign6110_e7371_d_n7, assign6110_e7371_d_n15, assign6110_e7371_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard67 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffs, locals.var_fn61_calc_iq__ffs_dn2, locals.var_fn61_calc_iq__ffs_dn3, locals.var_fn61_calc_iq__ffs_dn4, locals.var_fn61_calc_iq__ffs_dn7, locals.var_fn61_calc_iq__ffs_dn15, locals.var_fn61_calc_iq__ffs_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffs = assign6110_e7371;
        locals.var_fn61_calc_iq__ffs_dn2 = assign6110_e7371_d_n2;
        locals.var_fn61_calc_iq__ffs_dn3 = assign6110_e7371_d_n3;
        locals.var_fn61_calc_iq__ffs_dn4 = assign6110_e7371_d_n4;
        locals.var_fn61_calc_iq__ffs_dn7 = assign6110_e7371_d_n7;
        locals.var_fn61_calc_iq__ffs_dn15 = assign6110_e7371_d_n15;
        locals.var_fn61_calc_iq__ffs_dn16 = assign6110_e7371_d_n16;

        let assign6120_e7374: f64 = (-50.0);
        let assign6120_e7375: f64 = if locals.var_fn61_calc_iq__exparg < assign6120_e7374 { 1.0 } else { 0.0 };
        locals.var_guard68 = assign6120_e7375;

        let (assign6130_e7384, assign6130_e7384_d_n2, assign6130_e7384_d_n3, assign6130_e7384_d_n4, assign6130_e7384_d_n7, assign6130_e7384_d_n15, assign6130_e7384_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard67 == 0.0)) && (locals.var_guard68 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffs, locals.var_fn61_calc_iq__ffs_dn2, locals.var_fn61_calc_iq__ffs_dn3, locals.var_fn61_calc_iq__ffs_dn4, locals.var_fn61_calc_iq__ffs_dn7, locals.var_fn61_calc_iq__ffs_dn15, locals.var_fn61_calc_iq__ffs_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffs = assign6130_e7384;
        locals.var_fn61_calc_iq__ffs_dn2 = assign6130_e7384_d_n2;
        locals.var_fn61_calc_iq__ffs_dn3 = assign6130_e7384_d_n3;
        locals.var_fn61_calc_iq__ffs_dn4 = assign6130_e7384_d_n4;
        locals.var_fn61_calc_iq__ffs_dn7 = assign6130_e7384_d_n7;
        locals.var_fn61_calc_iq__ffs_dn15 = assign6130_e7384_d_n15;
        locals.var_fn61_calc_iq__ffs_dn16 = assign6130_e7384_d_n16;

        let (assign6140_e7399, assign6140_e7399_d_n2, assign6140_e7399_d_n3, assign6140_e7399_d_n4, assign6140_e7399_d_n7, assign6140_e7399_d_n15, assign6140_e7399_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard67 == 0.0)) && (locals.var_guard68 == 0.0)) {
        let assign6140_e7395: f64 = (locals.var_fn61_calc_iq__exparg).exp();
        let assign6140_e7396: f64 = (1.0 + assign6140_e7395);
        let assign6140_e7397: f64 = (1.0 / assign6140_e7396);
        (assign6140_e7397, (-((assign6140_e7395 * locals.var_fn61_calc_iq__exparg_dn2) / (assign6140_e7396 * assign6140_e7396))), (-((assign6140_e7395 * locals.var_fn61_calc_iq__exparg_dn3) / (assign6140_e7396 * assign6140_e7396))), (-((assign6140_e7395 * locals.var_fn61_calc_iq__exparg_dn4) / (assign6140_e7396 * assign6140_e7396))), (-((assign6140_e7395 * locals.var_fn61_calc_iq__exparg_dn7) / (assign6140_e7396 * assign6140_e7396))), (-((assign6140_e7395 * locals.var_fn61_calc_iq__exparg_dn15) / (assign6140_e7396 * assign6140_e7396))), (-((assign6140_e7395 * locals.var_fn61_calc_iq__exparg_dn16) / (assign6140_e7396 * assign6140_e7396))),)
    } else {
        (locals.var_fn61_calc_iq__ffs, locals.var_fn61_calc_iq__ffs_dn2, locals.var_fn61_calc_iq__ffs_dn3, locals.var_fn61_calc_iq__ffs_dn4, locals.var_fn61_calc_iq__ffs_dn7, locals.var_fn61_calc_iq__ffs_dn15, locals.var_fn61_calc_iq__ffs_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffs = assign6140_e7399;
        locals.var_fn61_calc_iq__ffs_dn2 = assign6140_e7399_d_n2;
        locals.var_fn61_calc_iq__ffs_dn3 = assign6140_e7399_d_n3;
        locals.var_fn61_calc_iq__ffs_dn4 = assign6140_e7399_d_n4;
        locals.var_fn61_calc_iq__ffs_dn7 = assign6140_e7399_d_n7;
        locals.var_fn61_calc_iq__ffs_dn15 = assign6140_e7399_d_n15;
        locals.var_fn61_calc_iq__ffs_dn16 = assign6140_e7399_d_n16;

        let (assign6150_e7417, assign6150_e7417_d_n2, assign6150_e7417_d_n3, assign6150_e7417_d_n4, assign6150_e7417_d_n7, assign6150_e7417_d_n15, assign6150_e7417_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6150_e7403: f64 = (locals.var_fn61_calc_iq__vgdin - locals.var_fn61_calc_iq__vsx);
        let assign6150_e7407: f64 = (p.p51 * 0.1);
        let assign6150_e7409: f64 = (assign6150_e7407 * locals.var_fn61_calc_iq__alpha_phit);
        let assign6150_e7411: f64 = (assign6150_e7409 * locals.var_fn61_calc_iq__ffs);
        let assign6150_e7412: f64 = (locals.var_fn61_calc_iq__vtdibl - assign6150_e7411);
        let assign6150_e7413: f64 = (assign6150_e7403 - assign6150_e7412);
        let assign6150_e7415: f64 = (assign6150_e7413 / locals.var_fn61_calc_iq__two_n_phit);
        (assign6150_e7415, (((locals.var_fn61_calc_iq__vgdin_dn2 - locals.var_fn61_calc_iq__vsx_dn2) - (-(assign6150_e7409 * locals.var_fn61_calc_iq__ffs_dn2))) / locals.var_fn61_calc_iq__two_n_phit), (((-locals.var_fn61_calc_iq__vsx_dn3) - (-(assign6150_e7409 * locals.var_fn61_calc_iq__ffs_dn3))) / locals.var_fn61_calc_iq__two_n_phit), (((((-locals.var_fn61_calc_iq__vsx_dn4) - (locals.var_fn61_calc_iq__vtdibl_dn4 - (((assign6150_e7407 * locals.var_fn61_calc_iq__alpha_phit_dn4) * locals.var_fn61_calc_iq__ffs) + (assign6150_e7409 * locals.var_fn61_calc_iq__ffs_dn4)))) * locals.var_fn61_calc_iq__two_n_phit) - (assign6150_e7413 * locals.var_fn61_calc_iq__two_n_phit_dn4)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)), (((locals.var_fn61_calc_iq__vgdin_dn7 - locals.var_fn61_calc_iq__vsx_dn7) - (-(assign6150_e7409 * locals.var_fn61_calc_iq__ffs_dn7))) / locals.var_fn61_calc_iq__two_n_phit), (((((locals.var_fn61_calc_iq__vgdin_dn15 - locals.var_fn61_calc_iq__vsx_dn15) - (locals.var_fn61_calc_iq__vtdibl_dn15 - (assign6150_e7409 * locals.var_fn61_calc_iq__ffs_dn15))) * locals.var_fn61_calc_iq__two_n_phit) - (assign6150_e7413 * locals.var_fn61_calc_iq__two_n_phit_dn15)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)), (((((locals.var_fn61_calc_iq__vgdin_dn16 - locals.var_fn61_calc_iq__vsx_dn16) - (locals.var_fn61_calc_iq__vtdibl_dn16 - (assign6150_e7409 * locals.var_fn61_calc_iq__ffs_dn16))) * locals.var_fn61_calc_iq__two_n_phit) - (assign6150_e7413 * locals.var_fn61_calc_iq__two_n_phit_dn16)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn61_calc_iq__etas, locals.var_fn61_calc_iq__etas_dn2, locals.var_fn61_calc_iq__etas_dn3, locals.var_fn61_calc_iq__etas_dn4, locals.var_fn61_calc_iq__etas_dn7, locals.var_fn61_calc_iq__etas_dn15, locals.var_fn61_calc_iq__etas_dn16,)
    }
};
        locals.var_fn61_calc_iq__etas = assign6150_e7417;
        locals.var_fn61_calc_iq__etas_dn2 = assign6150_e7417_d_n2;
        locals.var_fn61_calc_iq__etas_dn3 = assign6150_e7417_d_n3;
        locals.var_fn61_calc_iq__etas_dn4 = assign6150_e7417_d_n4;
        locals.var_fn61_calc_iq__etas_dn7 = assign6150_e7417_d_n7;
        locals.var_fn61_calc_iq__etas_dn15 = assign6150_e7417_d_n15;
        locals.var_fn61_calc_iq__etas_dn16 = assign6150_e7417_d_n16;

        let assign6160_e7420: f64 = if locals.var_fn61_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard69 = assign6160_e7420;

        let (assign6170_e7428, assign6170_e7428_d_n2, assign6170_e7428_d_n3, assign6170_e7428_d_n4, assign6170_e7428_d_n7, assign6170_e7428_d_n15, assign6170_e7428_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard69 != 0.0)) {
        let assign6170_e7426: f64 = (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etas);
        (assign6170_e7426, (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etas_dn2), (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etas_dn3), ((locals.var_fn61_calc_iq__qref_dn4 * locals.var_fn61_calc_iq__etas) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etas_dn4)), (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etas_dn7), ((locals.var_fn61_calc_iq__qref_dn15 * locals.var_fn61_calc_iq__etas) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etas_dn15)), ((locals.var_fn61_calc_iq__qref_dn16 * locals.var_fn61_calc_iq__etas) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etas_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qinvs, locals.var_fn61_calc_iq__qinvs_dn2, locals.var_fn61_calc_iq__qinvs_dn3, locals.var_fn61_calc_iq__qinvs_dn4, locals.var_fn61_calc_iq__qinvs_dn7, locals.var_fn61_calc_iq__qinvs_dn15, locals.var_fn61_calc_iq__qinvs_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvs = assign6170_e7428;
        locals.var_fn61_calc_iq__qinvs_dn2 = assign6170_e7428_d_n2;
        locals.var_fn61_calc_iq__qinvs_dn3 = assign6170_e7428_d_n3;
        locals.var_fn61_calc_iq__qinvs_dn4 = assign6170_e7428_d_n4;
        locals.var_fn61_calc_iq__qinvs_dn7 = assign6170_e7428_d_n7;
        locals.var_fn61_calc_iq__qinvs_dn15 = assign6170_e7428_d_n15;
        locals.var_fn61_calc_iq__qinvs_dn16 = assign6170_e7428_d_n16;

        let assign6180_e7431: f64 = (-50.0);
        let assign6180_e7432: f64 = if locals.var_fn61_calc_iq__etas < assign6180_e7431 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign6180_e7432;

        let (assign6190_e7444, assign6190_e7444_d_n2, assign6190_e7444_d_n3, assign6190_e7444_d_n4, assign6190_e7444_d_n7, assign6190_e7444_d_n15, assign6190_e7444_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard69 == 0.0)) && (locals.var_guard70 != 0.0)) {
        let assign6190_e7441: f64 = (locals.var_fn61_calc_iq__etas).exp();
        let assign6190_e7442: f64 = (locals.var_fn61_calc_iq__qref * assign6190_e7441);
        (assign6190_e7442, (locals.var_fn61_calc_iq__qref * (assign6190_e7441 * locals.var_fn61_calc_iq__etas_dn2)), (locals.var_fn61_calc_iq__qref * (assign6190_e7441 * locals.var_fn61_calc_iq__etas_dn3)), ((locals.var_fn61_calc_iq__qref_dn4 * assign6190_e7441) + (locals.var_fn61_calc_iq__qref * (assign6190_e7441 * locals.var_fn61_calc_iq__etas_dn4))), (locals.var_fn61_calc_iq__qref * (assign6190_e7441 * locals.var_fn61_calc_iq__etas_dn7)), ((locals.var_fn61_calc_iq__qref_dn15 * assign6190_e7441) + (locals.var_fn61_calc_iq__qref * (assign6190_e7441 * locals.var_fn61_calc_iq__etas_dn15))), ((locals.var_fn61_calc_iq__qref_dn16 * assign6190_e7441) + (locals.var_fn61_calc_iq__qref * (assign6190_e7441 * locals.var_fn61_calc_iq__etas_dn16))),)
    } else {
        (locals.var_fn61_calc_iq__qinvs, locals.var_fn61_calc_iq__qinvs_dn2, locals.var_fn61_calc_iq__qinvs_dn3, locals.var_fn61_calc_iq__qinvs_dn4, locals.var_fn61_calc_iq__qinvs_dn7, locals.var_fn61_calc_iq__qinvs_dn15, locals.var_fn61_calc_iq__qinvs_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvs = assign6190_e7444;
        locals.var_fn61_calc_iq__qinvs_dn2 = assign6190_e7444_d_n2;
        locals.var_fn61_calc_iq__qinvs_dn3 = assign6190_e7444_d_n3;
        locals.var_fn61_calc_iq__qinvs_dn4 = assign6190_e7444_d_n4;
        locals.var_fn61_calc_iq__qinvs_dn7 = assign6190_e7444_d_n7;
        locals.var_fn61_calc_iq__qinvs_dn15 = assign6190_e7444_d_n15;
        locals.var_fn61_calc_iq__qinvs_dn16 = assign6190_e7444_d_n16;

        let (assign6200_e7460, assign6200_e7460_d_n2, assign6200_e7460_d_n3, assign6200_e7460_d_n4, assign6200_e7460_d_n7, assign6200_e7460_d_n15, assign6200_e7460_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard69 == 0.0)) && (locals.var_guard70 == 0.0)) {
        let assign6200_e7455: f64 = (locals.var_fn61_calc_iq__etas).exp();
        let assign6200_e7456: f64 = (1.0 + assign6200_e7455);
        let assign6200_e7457: f64 = (assign6200_e7456).ln();
        let assign6200_e7458: f64 = (locals.var_fn61_calc_iq__qref * assign6200_e7457);
        (assign6200_e7458, (locals.var_fn61_calc_iq__qref * ((assign6200_e7455 * locals.var_fn61_calc_iq__etas_dn2) / assign6200_e7456)), (locals.var_fn61_calc_iq__qref * ((assign6200_e7455 * locals.var_fn61_calc_iq__etas_dn3) / assign6200_e7456)), ((locals.var_fn61_calc_iq__qref_dn4 * assign6200_e7457) + (locals.var_fn61_calc_iq__qref * ((assign6200_e7455 * locals.var_fn61_calc_iq__etas_dn4) / assign6200_e7456))), (locals.var_fn61_calc_iq__qref * ((assign6200_e7455 * locals.var_fn61_calc_iq__etas_dn7) / assign6200_e7456)), ((locals.var_fn61_calc_iq__qref_dn15 * assign6200_e7457) + (locals.var_fn61_calc_iq__qref * ((assign6200_e7455 * locals.var_fn61_calc_iq__etas_dn15) / assign6200_e7456))), ((locals.var_fn61_calc_iq__qref_dn16 * assign6200_e7457) + (locals.var_fn61_calc_iq__qref * ((assign6200_e7455 * locals.var_fn61_calc_iq__etas_dn16) / assign6200_e7456))),)
    } else {
        (locals.var_fn61_calc_iq__qinvs, locals.var_fn61_calc_iq__qinvs_dn2, locals.var_fn61_calc_iq__qinvs_dn3, locals.var_fn61_calc_iq__qinvs_dn4, locals.var_fn61_calc_iq__qinvs_dn7, locals.var_fn61_calc_iq__qinvs_dn15, locals.var_fn61_calc_iq__qinvs_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvs = assign6200_e7460;
        locals.var_fn61_calc_iq__qinvs_dn2 = assign6200_e7460_d_n2;
        locals.var_fn61_calc_iq__qinvs_dn3 = assign6200_e7460_d_n3;
        locals.var_fn61_calc_iq__qinvs_dn4 = assign6200_e7460_d_n4;
        locals.var_fn61_calc_iq__qinvs_dn7 = assign6200_e7460_d_n7;
        locals.var_fn61_calc_iq__qinvs_dn15 = assign6200_e7460_d_n15;
        locals.var_fn61_calc_iq__qinvs_dn16 = assign6200_e7460_d_n16;

        let (assign6210_e7468, assign6210_e7468_d_n2, assign6210_e7468_d_n3, assign6210_e7468_d_n4, assign6210_e7468_d_n7, assign6210_e7468_d_n15, assign6210_e7468_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6210_e7464: f64 = (locals.var_fn61_calc_iq__vgdin - locals.var_fn61_calc_iq__myarg);
        let assign6210_e7466: f64 = (assign6210_e7464 / locals.var_fn61_calc_iq__alpha_phit);
        (assign6210_e7466, ((locals.var_fn61_calc_iq__vgdin_dn2 - locals.var_fn61_calc_iq__myarg_dn2) / locals.var_fn61_calc_iq__alpha_phit), ((-locals.var_fn61_calc_iq__myarg_dn3) / locals.var_fn61_calc_iq__alpha_phit), ((((-locals.var_fn61_calc_iq__myarg_dn4) * locals.var_fn61_calc_iq__alpha_phit) - (assign6210_e7464 * locals.var_fn61_calc_iq__alpha_phit_dn4)) / (locals.var_fn61_calc_iq__alpha_phit * locals.var_fn61_calc_iq__alpha_phit)), ((locals.var_fn61_calc_iq__vgdin_dn7 - locals.var_fn61_calc_iq__myarg_dn7) / locals.var_fn61_calc_iq__alpha_phit), ((locals.var_fn61_calc_iq__vgdin_dn15 - locals.var_fn61_calc_iq__myarg_dn15) / locals.var_fn61_calc_iq__alpha_phit), ((locals.var_fn61_calc_iq__vgdin_dn16 - locals.var_fn61_calc_iq__myarg_dn16) / locals.var_fn61_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign6210_e7468;
        locals.var_fn61_calc_iq__exparg_dn2 = assign6210_e7468_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign6210_e7468_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign6210_e7468_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign6210_e7468_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign6210_e7468_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign6210_e7468_d_n16;

        let assign6220_e7471: f64 = if locals.var_fn61_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard71 = assign6220_e7471;

        let (assign6230_e7477, assign6230_e7477_d_n2, assign6230_e7477_d_n3, assign6230_e7477_d_n4, assign6230_e7477_d_n7, assign6230_e7477_d_n15, assign6230_e7477_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard71 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffd, locals.var_fn61_calc_iq__ffd_dn2, locals.var_fn61_calc_iq__ffd_dn3, locals.var_fn61_calc_iq__ffd_dn4, locals.var_fn61_calc_iq__ffd_dn7, locals.var_fn61_calc_iq__ffd_dn15, locals.var_fn61_calc_iq__ffd_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffd = assign6230_e7477;
        locals.var_fn61_calc_iq__ffd_dn2 = assign6230_e7477_d_n2;
        locals.var_fn61_calc_iq__ffd_dn3 = assign6230_e7477_d_n3;
        locals.var_fn61_calc_iq__ffd_dn4 = assign6230_e7477_d_n4;
        locals.var_fn61_calc_iq__ffd_dn7 = assign6230_e7477_d_n7;
        locals.var_fn61_calc_iq__ffd_dn15 = assign6230_e7477_d_n15;
        locals.var_fn61_calc_iq__ffd_dn16 = assign6230_e7477_d_n16;

        let assign6240_e7480: f64 = (-50.0);
        let assign6240_e7481: f64 = if locals.var_fn61_calc_iq__exparg < assign6240_e7480 { 1.0 } else { 0.0 };
        locals.var_guard72 = assign6240_e7481;

        let (assign6250_e7490, assign6250_e7490_d_n2, assign6250_e7490_d_n3, assign6250_e7490_d_n4, assign6250_e7490_d_n7, assign6250_e7490_d_n15, assign6250_e7490_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard71 == 0.0)) && (locals.var_guard72 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffd, locals.var_fn61_calc_iq__ffd_dn2, locals.var_fn61_calc_iq__ffd_dn3, locals.var_fn61_calc_iq__ffd_dn4, locals.var_fn61_calc_iq__ffd_dn7, locals.var_fn61_calc_iq__ffd_dn15, locals.var_fn61_calc_iq__ffd_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffd = assign6250_e7490;
        locals.var_fn61_calc_iq__ffd_dn2 = assign6250_e7490_d_n2;
        locals.var_fn61_calc_iq__ffd_dn3 = assign6250_e7490_d_n3;
        locals.var_fn61_calc_iq__ffd_dn4 = assign6250_e7490_d_n4;
        locals.var_fn61_calc_iq__ffd_dn7 = assign6250_e7490_d_n7;
        locals.var_fn61_calc_iq__ffd_dn15 = assign6250_e7490_d_n15;
        locals.var_fn61_calc_iq__ffd_dn16 = assign6250_e7490_d_n16;

    }
}
