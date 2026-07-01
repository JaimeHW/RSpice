#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2810_e2453, assign2810_e2453_d_n4, assign2810_e2453_d_n6, assign2810_e2453_d_n7, assign2810_e2453_d_n8, assign2810_e2453_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard97 == 0.0)) {
        let assign2810_e2429: f64 = (-locals.var_temp3);
        let assign2810_e2431: f64 = (assign2810_e2429 - 80.0);
        let assign2810_e2435: f64 = (-locals.var_temp3);
        let assign2810_e2437: f64 = (assign2810_e2435 - 80.0);
        let assign2810_e2438: f64 = (0.5 * assign2810_e2437);
        let assign2810_e2441: f64 = (-locals.var_temp3);
        let assign2810_e2443: f64 = (assign2810_e2441 - 80.0);
        let assign2810_e2445: f64 = (assign2810_e2443 * 0.3333333333333);
        let assign2810_e2446: f64 = (1.0 + assign2810_e2445);
        let assign2810_e2447: f64 = (assign2810_e2438 * assign2810_e2446);
        let assign2810_e2448: f64 = (1.0 + assign2810_e2447);
        let assign2810_e2449: f64 = (assign2810_e2431 * assign2810_e2448);
        let assign2810_e2450: f64 = (1.0 + assign2810_e2449);
        let assign2810_e2451: f64 = (1.80485e-35 / assign2810_e2450);
        (assign2810_e2451, (-((1.80485e-35 * (((-locals.var_temp3_dn4) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn4)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn4) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-locals.var_temp3_dn6) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn6)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn6) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-locals.var_temp3_dn7) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn7)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn7) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-locals.var_temp3_dn8) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn8)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn8) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-locals.var_temp3_dn9) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn9)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn9) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign2810_e2453;
        locals.var_temp4_dn4 = assign2810_e2453_d_n4;
        locals.var_temp4_dn6 = assign2810_e2453_d_n6;
        locals.var_temp4_dn7 = assign2810_e2453_d_n7;
        locals.var_temp4_dn8 = assign2810_e2453_d_n8;
        locals.var_temp4_dn9 = assign2810_e2453_d_n9;
        locals.var_temp4_rv = 0.0;

        let (assign2820_e2482, assign2820_e2482_d_n4, assign2820_e2482_d_n6, assign2820_e2482_d_n7, assign2820_e2482_d_n8, assign2820_e2482_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2820_e2461: f64 = (p.p242 * locals.var_iwe);
        let assign2820_e2462: f64 = (1.0 + assign2820_e2461);
        let assign2820_e2463: f64 = (p.p241 * assign2820_e2462);
        let assign2820_e2466: f64 = (locals.var_temp2 - 1.0);
        let assign2820_e2467: f64 = (assign2820_e2463 * assign2820_e2466);
        let assign2820_e2469: f64 = (assign2820_e2467 / locals.var_temp1);
        let assign2820_e2470: f64 = (1.0 + assign2820_e2469);
        let assign2820_e2474: f64 = (locals.var_temp4 - 1.0);
        let assign2820_e2475: f64 = (p.p245 * assign2820_e2474);
        let assign2820_e2477: f64 = (assign2820_e2475 / locals.var_temp3);
        let assign2820_e2478: f64 = (assign2820_e2470 + assign2820_e2477);
        let assign2820_e2480: f64 = (assign2820_e2478).max(1e-6);
        (assign2820_e2480, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn4) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn4)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn4) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn4)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn6) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn6) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn6)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn7) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn7) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn7)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn8) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn8) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn8)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn9) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn9)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn9) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn9)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 },)
    } else {
        (locals.var_gpe, locals.var_gpe_dn4, locals.var_gpe_dn6, locals.var_gpe_dn7, locals.var_gpe_dn8, locals.var_gpe_dn9,)
    }
};
        locals.var_gpe = assign2820_e2482;
        locals.var_gpe_dn4 = assign2820_e2482_d_n4;
        locals.var_gpe_dn6 = assign2820_e2482_d_n6;
        locals.var_gpe_dn7 = assign2820_e2482_d_n7;
        locals.var_gpe_dn8 = assign2820_e2482_d_n8;
        locals.var_gpe_dn9 = assign2820_e2482_d_n9;
        locals.var_gpe_rv = 0.0;

        let (assign2830_e2504,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2830_e2488: f64 = (p.p247 * locals.var_iwe);
        let assign2830_e2489: f64 = (1.0 + assign2830_e2488);
        let assign2830_e2492: f64 = (p.p248 * locals.var_iwe);
        let assign2830_e2496: f64 = (locals.var_we / p.p249);
        let assign2830_e2497: f64 = (1.0 + assign2830_e2496);
        let assign2830_e2498: f64 = (assign2830_e2497).ln();
        let assign2830_e2499: f64 = (assign2830_e2492 * assign2830_e2498);
        let assign2830_e2500: f64 = (assign2830_e2489 + assign2830_e2499);
        let assign2830_e2502: f64 = (assign2830_e2500).max(1e-6);
        (assign2830_e2502,)
    } else {
        (locals.var_gwe,)
    }
};
        locals.var_gwe = assign2830_e2504;
        locals.var_gwe_rv = 0.0;

        let (assign2840_e2513, assign2840_e2513_d_n4, assign2840_e2513_d_n6, assign2840_e2513_d_n7, assign2840_e2513_d_n8, assign2840_e2513_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2840_e2509: f64 = (p.p240 / locals.var_gpe);
        let assign2840_e2511: f64 = (assign2840_e2509 * locals.var_gwe);
        (assign2840_e2511, ((-((p.p240 * locals.var_gpe_dn4) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p240 * locals.var_gpe_dn6) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p240 * locals.var_gpe_dn7) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p240 * locals.var_gpe_dn8) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p240 * locals.var_gpe_dn9) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe),)
    } else {
        (locals.var_ge, locals.var_ge_dn4, locals.var_ge_dn6, locals.var_ge_dn7, locals.var_ge_dn8, locals.var_ge_dn9,)
    }
};
        locals.var_ge = assign2840_e2513;
        locals.var_ge_dn4 = assign2840_e2513_d_n4;
        locals.var_ge_dn6 = assign2840_e2513_d_n6;
        locals.var_ge_dn7 = assign2840_e2513_d_n7;
        locals.var_ge_dn8 = assign2840_e2513_d_n8;
        locals.var_ge_dn9 = assign2840_e2513_d_n9;
        locals.var_ge_rv = 0.0;

        let (assign2850_e2522, assign2850_e2522_d_n4, assign2850_e2522_d_n6, assign2850_e2522_d_n7, assign2850_e2522_d_n8, assign2850_e2522_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2850_e2518: f64 = (locals.var_ge * locals.var_we);
        let assign2850_e2520: f64 = (assign2850_e2518 / locals.var_le);
        (assign2850_e2520, ((locals.var_ge_dn4 * locals.var_we) / locals.var_le), ((locals.var_ge_dn6 * locals.var_we) / locals.var_le), ((locals.var_ge_dn7 * locals.var_we) / locals.var_le), ((locals.var_ge_dn8 * locals.var_we) / locals.var_le), ((locals.var_ge_dn9 * locals.var_we) / locals.var_le),)
    } else {
        (locals.var_betn_p, locals.var_betn_p_dn4, locals.var_betn_p_dn6, locals.var_betn_p_dn7, locals.var_betn_p_dn8, locals.var_betn_p_dn9,)
    }
};
        locals.var_betn_p = assign2850_e2522;
        locals.var_betn_p_dn4 = assign2850_e2522_d_n4;
        locals.var_betn_p_dn6 = assign2850_e2522_d_n6;
        locals.var_betn_p_dn7 = assign2850_e2522_d_n7;
        locals.var_betn_p_dn8 = assign2850_e2522_d_n8;
        locals.var_betn_p_dn9 = assign2850_e2522_d_n9;
        locals.var_betn_p_rv = 0.0;

        let (assign2860_e2529, assign2860_e2529_d_n4, assign2860_e2529_d_n6, assign2860_e2529_d_n7, assign2860_e2529_d_n8, assign2860_e2529_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2860_e2527: f64 = (locals.var_betn_p).max(1e-10);
        (assign2860_e2527, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn4 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn6 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn7 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn8 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9,)
    }
};
        locals.var_betn1_t = assign2860_e2529;
        locals.var_betn1_t_dn4 = assign2860_e2529_d_n4;
        locals.var_betn1_t_dn6 = assign2860_e2529_d_n6;
        locals.var_betn1_t_dn7 = assign2860_e2529_d_n7;
        locals.var_betn1_t_dn8 = assign2860_e2529_d_n8;
        locals.var_betn1_t_dn9 = assign2860_e2529_d_n9;
        locals.var_betn1_t_rv = 0.0;

        let (assign2870_e2536, assign2870_e2536_d_n4, assign2870_e2536_d_n6, assign2870_e2536_d_n7, assign2870_e2536_d_n8, assign2870_e2536_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2870_e2534: f64 = (p.p250 * locals.var_betn1_t);
        (assign2870_e2534, (p.p250 * locals.var_betn1_t_dn4), (p.p250 * locals.var_betn1_t_dn6), (p.p250 * locals.var_betn1_t_dn7), (p.p250 * locals.var_betn1_t_dn8), (p.p250 * locals.var_betn1_t_dn9),)
    } else {
        (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9,)
    }
};
        locals.var_betn2_t = assign2870_e2536;
        locals.var_betn2_t_dn4 = assign2870_e2536_d_n4;
        locals.var_betn2_t_dn6 = assign2870_e2536_d_n6;
        locals.var_betn2_t_dn7 = assign2870_e2536_d_n7;
        locals.var_betn2_t_dn8 = assign2870_e2536_d_n8;
        locals.var_betn2_t_dn9 = assign2870_e2536_d_n9;
        locals.var_betn2_t_rv = 0.0;

        let (assign2880_e2559,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2880_e2543: f64 = (p.p252 * locals.var_ile);
        let assign2880_e2544: f64 = (1.0 + assign2880_e2543);
        let assign2880_e2545: f64 = (p.p251 * assign2880_e2544);
        let assign2880_e2549: f64 = (p.p253 * locals.var_iwe);
        let assign2880_e2550: f64 = (1.0 + assign2880_e2549);
        let assign2880_e2551: f64 = (assign2880_e2545 * assign2880_e2550);
        let assign2880_e2555: f64 = (p.p254 * locals.var_iae);
        let assign2880_e2556: f64 = (1.0 + assign2880_e2555);
        let assign2880_e2557: f64 = (assign2880_e2551 * assign2880_e2556);
        (assign2880_e2557,)
    } else {
        (locals.var_stbet_i,)
    }
};
        locals.var_stbet_i = assign2880_e2559;
        locals.var_stbet_i_rv = 0.0;

        let (assign2890_e2582,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2890_e2566: f64 = (locals.var_ile).powf(p.p257);
        let assign2890_e2567: f64 = (p.p256 * assign2890_e2566);
        let assign2890_e2568: f64 = (p.p255 + assign2890_e2567);
        let assign2890_e2572: f64 = (p.p258 * locals.var_iwe);
        let assign2890_e2573: f64 = (1.0 + assign2890_e2572);
        let assign2890_e2574: f64 = (assign2890_e2568 * assign2890_e2573);
        let assign2890_e2578: f64 = (p.p259 * locals.var_iae);
        let assign2890_e2579: f64 = (1.0 + assign2890_e2578);
        let assign2890_e2580: f64 = (assign2890_e2574 * assign2890_e2579);
        (assign2890_e2580,)
    } else {
        (locals.var_cs_p,)
    }
};
        locals.var_cs_p = assign2890_e2582;
        locals.var_cs_p_rv = 0.0;

        let (assign2900_e2589,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2900_e2587: f64 = (locals.var_cs_p).max(0.0);
        (assign2900_e2587,)
    } else {
        (locals.var_cs_t,)
    }
};
        locals.var_cs_t = assign2900_e2589;
        locals.var_cs_t_rv = 0.0;

        let (assign2910_e2594,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p260,)
    } else {
        (locals.var_csfi_i,)
    }
};
        locals.var_csfi_i = assign2910_e2594;
        locals.var_csfi_i_rv = 0.0;

        let (assign2920_e2599,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p261,)
    } else {
        (locals.var_csbi_i,)
    }
};
        locals.var_csbi_i = assign2920_e2599;
        locals.var_csbi_i_rv = 0.0;

        let (assign2930_e2622,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2930_e2606: f64 = (p.p263 * locals.var_ile);
        let assign2930_e2607: f64 = (1.0 + assign2930_e2606);
        let assign2930_e2608: f64 = (p.p262 * assign2930_e2607);
        let assign2930_e2612: f64 = (p.p264 * locals.var_iwe);
        let assign2930_e2613: f64 = (1.0 + assign2930_e2612);
        let assign2930_e2614: f64 = (assign2930_e2608 * assign2930_e2613);
        let assign2930_e2618: f64 = (p.p265 * locals.var_iae);
        let assign2930_e2619: f64 = (1.0 + assign2930_e2618);
        let assign2930_e2620: f64 = (assign2930_e2614 * assign2930_e2619);
        (assign2930_e2620,)
    } else {
        (locals.var_stcs_i,)
    }
};
        locals.var_stcs_i = assign2930_e2622;
        locals.var_stcs_i_rv = 0.0;

        let (assign2940_e2627,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p266,)
    } else {
        (locals.var_thecs_t,)
    }
};
        locals.var_thecs_t = assign2940_e2627;
        locals.var_thecs_t_rv = 0.0;

        let (assign2950_e2632,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p267,)
    } else {
        (locals.var_stthecs_i,)
    }
};
        locals.var_stthecs_i = assign2950_e2632;
        locals.var_stthecs_i_rv = 0.0;

        let (assign2960_e2637,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p268,)
    } else {
        (locals.var_csthr_i,)
    }
};
        locals.var_csthr_i = assign2960_e2637;
        locals.var_csthr_i_rv = 0.0;

        let (assign2970_e2642,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p269,)
    } else {
        (locals.var_csthrb_i,)
    }
};
        locals.var_csthrb_i = assign2970_e2642;
        locals.var_csthrb_i_rv = 0.0;

        let (assign2980_e2647,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p270,)
    } else {
        (locals.var_mue_t,)
    }
};
        locals.var_mue_t = assign2980_e2647;
        locals.var_mue_t_rv = 0.0;

        let (assign2990_e2652,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p271,)
    } else {
        (locals.var_stmue_i,)
    }
};
        locals.var_stmue_i = assign2990_e2652;
        locals.var_stmue_i_rv = 0.0;

        let (assign3000_e2657,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p272,)
    } else {
        (locals.var_themu_t,)
    }
};
        locals.var_themu_t = assign3000_e2657;
        locals.var_themu_t_rv = 0.0;

        let (assign3010_e2662,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p273,)
    } else {
        (locals.var_stthemu_i,)
    }
};
        locals.var_stthemu_i = assign3010_e2662;
        locals.var_stthemu_i_rv = 0.0;

        let (assign3020_e2685,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3020_e2669: f64 = (locals.var_ile).powf(p.p276);
        let assign3020_e2670: f64 = (p.p275 * assign3020_e2669);
        let assign3020_e2671: f64 = (p.p274 + assign3020_e2670);
        let assign3020_e2675: f64 = (p.p277 * locals.var_iwe);
        let assign3020_e2676: f64 = (1.0 + assign3020_e2675);
        let assign3020_e2677: f64 = (assign3020_e2671 * assign3020_e2676);
        let assign3020_e2681: f64 = (p.p278 * locals.var_iae);
        let assign3020_e2682: f64 = (1.0 + assign3020_e2681);
        let assign3020_e2683: f64 = (assign3020_e2677 * assign3020_e2682);
        (assign3020_e2683,)
    } else {
        (locals.var_xcor_t,)
    }
};
        locals.var_xcor_t = assign3020_e2685;
        locals.var_xcor_t_rv = 0.0;

        let (assign3030_e2690,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p279,)
    } else {
        (locals.var_xcorb_i,)
    }
};
        locals.var_xcorb_i = assign3030_e2690;
        locals.var_xcorb_i_rv = 0.0;

        let (assign3040_e2695,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p280,)
    } else {
        (locals.var_stxcor_i,)
    }
};
        locals.var_stxcor_i = assign3040_e2695;
        locals.var_stxcor_i_rv = 0.0;

        let (assign3050_e2700,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p281,)
    } else {
        (locals.var_feta_i,)
    }
};
        locals.var_feta_i = assign3050_e2700;
        locals.var_feta_i_rv = 0.0;

        let (assign3060_e2713,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3060_e2705: f64 = (p.p282 * locals.var_iwe);
        let assign3060_e2709: f64 = (p.p283 * locals.var_iwe);
        let assign3060_e2710: f64 = (1.0 + assign3060_e2709);
        let assign3060_e2711: f64 = (assign3060_e2705 * assign3060_e2710);
        (assign3060_e2711,)
    } else {
        (locals.var_rs_p,)
    }
};
        locals.var_rs_p = assign3060_e2713;
        locals.var_rs_p_rv = 0.0;

        let (assign3070_e2720,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3070_e2718: f64 = (locals.var_rs_p).max(0.0);
        (assign3070_e2718,)
    } else {
        (locals.var_rs_t,)
    }
};
        locals.var_rs_t = assign3070_e2720;
        locals.var_rs_t_rv = 0.0;

        let (assign3080_e2725,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p284,)
    } else {
        (locals.var_rsig_i,)
    }
};
        locals.var_rsig_i = assign3080_e2725;
        locals.var_rsig_i_rv = 0.0;

        let (assign3090_e2730,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p285,)
    } else {
        (locals.var_strs_i,)
    }
};
        locals.var_strs_i = assign3090_e2730;
        locals.var_strs_i_rv = 0.0;

        let (assign3100_e2735,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p286,)
    } else {
        (locals.var_rsg_i,)
    }
};
        locals.var_rsg_i = assign3100_e2735;
        locals.var_rsg_i_rv = 0.0;

        let (assign3110_e2740,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p287,)
    } else {
        (locals.var_thersg_i,)
    }
};
        locals.var_thersg_i = assign3110_e2740;
        locals.var_thersg_i_rv = 0.0;

        let (assign3120_e2745,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p288,)
    } else {
        (locals.var_rsb_i,)
    }
};
        locals.var_rsb_i = assign3120_e2745;
        locals.var_rsb_i_rv = 0.0;

        let (assign3130_e2770, assign3130_e2770_d_n4, assign3130_e2770_d_n6, assign3130_e2770_d_n7, assign3130_e2770_d_n8, assign3130_e2770_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3130_e2753: f64 = (locals.var_ile).powf(p.p291);
        let assign3130_e2754: f64 = (p.p290 * assign3130_e2753);
        let assign3130_e2755: f64 = (p.p289 + assign3130_e2754);
        let assign3130_e2756: f64 = (locals.var_ge * assign3130_e2755);
        let assign3130_e2760: f64 = (p.p292 * locals.var_iwe);
        let assign3130_e2761: f64 = (1.0 + assign3130_e2760);
        let assign3130_e2762: f64 = (assign3130_e2756 * assign3130_e2761);
        let assign3130_e2766: f64 = (p.p293 * locals.var_iae);
        let assign3130_e2767: f64 = (1.0 + assign3130_e2766);
        let assign3130_e2768: f64 = (assign3130_e2762 * assign3130_e2767);
        (assign3130_e2768, (((locals.var_ge_dn4 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((locals.var_ge_dn6 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((locals.var_ge_dn7 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((locals.var_ge_dn8 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((locals.var_ge_dn9 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767),)
    } else {
        (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9,)
    }
};
        locals.var_thesat_p = assign3130_e2770;
        locals.var_thesat_p_dn4 = assign3130_e2770_d_n4;
        locals.var_thesat_p_dn6 = assign3130_e2770_d_n6;
        locals.var_thesat_p_dn7 = assign3130_e2770_d_n7;
        locals.var_thesat_p_dn8 = assign3130_e2770_d_n8;
        locals.var_thesat_p_dn9 = assign3130_e2770_d_n9;
        locals.var_thesat_p_rv = 0.0;

        let (assign3140_e2777, assign3140_e2777_d_n4, assign3140_e2777_d_n6, assign3140_e2777_d_n7, assign3140_e2777_d_n8, assign3140_e2777_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3140_e2775: f64 = (locals.var_thesat_p).max(0.0);
        (assign3140_e2775, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn4 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn6 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn7 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn8 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9,)
    }
};
        locals.var_thesat_t = assign3140_e2777;
        locals.var_thesat_t_dn4 = assign3140_e2777_d_n4;
        locals.var_thesat_t_dn6 = assign3140_e2777_d_n6;
        locals.var_thesat_t_dn7 = assign3140_e2777_d_n7;
        locals.var_thesat_t_dn8 = assign3140_e2777_d_n8;
        locals.var_thesat_t_dn9 = assign3140_e2777_d_n9;
        locals.var_thesat_t_rv = 0.0;

        let (assign3150_e2800,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3150_e2784: f64 = (p.p295 * locals.var_ile);
        let assign3150_e2785: f64 = (1.0 + assign3150_e2784);
        let assign3150_e2786: f64 = (p.p294 * assign3150_e2785);
        let assign3150_e2790: f64 = (p.p296 * locals.var_iwe);
        let assign3150_e2791: f64 = (1.0 + assign3150_e2790);
        let assign3150_e2792: f64 = (assign3150_e2786 * assign3150_e2791);
        let assign3150_e2796: f64 = (p.p297 * locals.var_iae);
        let assign3150_e2797: f64 = (1.0 + assign3150_e2796);
        let assign3150_e2798: f64 = (assign3150_e2792 * assign3150_e2797);
        (assign3150_e2798,)
    } else {
        (locals.var_stthesat_i,)
    }
};
        locals.var_stthesat_i = assign3150_e2800;
        locals.var_stthesat_i_rv = 0.0;

        let (assign3160_e2805,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p298,)
    } else {
        (locals.var_thesat1_i,)
    }
};
        locals.var_thesat1_i = assign3160_e2805;
        locals.var_thesat1_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3170_e2810,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p299,)
    } else {
        (locals.var_thesat2_i,)
    }
};
        locals.var_thesat2_i = assign3170_e2810;
        locals.var_thesat2_i_rv = 0.0;

        let (assign3180_e2831,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3180_e2818: f64 = (locals.var_ile).powf(p.p302);
        let assign3180_e2819: f64 = (p.p301 * assign3180_e2818);
        let assign3180_e2824: f64 = (locals.var_ile).powf(p.p304);
        let assign3180_e2825: f64 = (p.p303 * assign3180_e2824);
        let assign3180_e2826: f64 = (1.0 + assign3180_e2825);
        let assign3180_e2827: f64 = (assign3180_e2819 / assign3180_e2826);
        let assign3180_e2828: f64 = (1.0 + assign3180_e2827);
        let assign3180_e2829: f64 = (p.p300 / assign3180_e2828);
        (assign3180_e2829,)
    } else {
        (locals.var_ax_p,)
    }
};
        locals.var_ax_p = assign3180_e2831;
        locals.var_ax_p_rv = 0.0;

        let (assign3190_e2840,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3190_e2836: f64 = (locals.var_ax_p).max(1.0);
        let assign3190_e2838: f64 = (assign3190_e2836).min(16.0);
        (assign3190_e2838,)
    } else {
        (locals.var_ax_i,)
    }
};
        locals.var_ax_i = assign3190_e2840;
        locals.var_ax_i_rv = 0.0;

        let (assign3200_e2863,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3200_e2846: f64 = (locals.var_ile).powf(p.p306);
        let assign3200_e2847: f64 = (p.p305 * assign3200_e2846);
        let assign3200_e2851: f64 = (p.p309 * locals.var_iwe);
        let assign3200_e2852: f64 = (1.0 + assign3200_e2851);
        let assign3200_e2853: f64 = (assign3200_e2847 * assign3200_e2852);
        let assign3200_e2858: f64 = (locals.var_ile).powf(p.p308);
        let assign3200_e2859: f64 = (p.p307 * assign3200_e2858);
        let assign3200_e2860: f64 = (1.0 + assign3200_e2859);
        let assign3200_e2861: f64 = (assign3200_e2853 / assign3200_e2860);
        (assign3200_e2861,)
    } else {
        (locals.var_alp_p,)
    }
};
        locals.var_alp_p = assign3200_e2863;
        locals.var_alp_p_rv = 0.0;

        let (assign3210_e2870,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3210_e2868: f64 = (locals.var_alp_p).max(0.0);
        (assign3210_e2868,)
    } else {
        (locals.var_alp_i,)
    }
};
        locals.var_alp_i = assign3210_e2870;
        locals.var_alp_i_rv = 0.0;

        let (assign3220_e2893,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3220_e2876: f64 = (locals.var_ile).powf(p.p311);
        let assign3220_e2877: f64 = (p.p310 * assign3220_e2876);
        let assign3220_e2881: f64 = (p.p314 * locals.var_iwe);
        let assign3220_e2882: f64 = (1.0 + assign3220_e2881);
        let assign3220_e2883: f64 = (assign3220_e2877 * assign3220_e2882);
        let assign3220_e2888: f64 = (locals.var_ile).powf(p.p313);
        let assign3220_e2889: f64 = (p.p312 * assign3220_e2888);
        let assign3220_e2890: f64 = (1.0 + assign3220_e2889);
        let assign3220_e2891: f64 = (assign3220_e2883 / assign3220_e2890);
        (assign3220_e2891,)
    } else {
        (locals.var_alp1_p,)
    }
};
        locals.var_alp1_p = assign3220_e2893;
        locals.var_alp1_p_rv = 0.0;

        let (assign3230_e2900,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3230_e2898: f64 = (locals.var_alp1_p).max(0.0);
        (assign3230_e2898,)
    } else {
        (locals.var_alp1_i,)
    }
};
        locals.var_alp1_i = assign3230_e2900;
        locals.var_alp1_i_rv = 0.0;

        let (assign3240_e2905,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p315,)
    } else {
        (locals.var_alpb_i,)
    }
};
        locals.var_alpb_i = assign3240_e2905;
        locals.var_alpb_i_rv = 0.0;

        let (assign3250_e2910,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p316,)
    } else {
        (locals.var_vp_i,)
    }
};
        locals.var_vp_i = assign3250_e2910;
        locals.var_vp_i_rv = 0.0;

        let (assign3260_e2915,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p317,)
    } else {
        (locals.var_vpg_i,)
    }
};
        locals.var_vpg_i = assign3260_e2915;
        locals.var_vpg_i_rv = 0.0;

        let (assign3270_e2920,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p318,)
    } else {
        (locals.var_gco_i,)
    }
};
        locals.var_gco_i = assign3270_e2920;
        locals.var_gco_i_rv = 0.0;

        let (assign3280_e2927,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3280_e2925: f64 = (p.p319 / locals.var_iae);
        (assign3280_e2925,)
    } else {
        (locals.var_iginv_t,)
    }
};
        locals.var_iginv_t = assign3280_e2927;
        locals.var_iginv_t_rv = 0.0;

        let (assign3290_e2934,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3290_e2932: f64 = (p.p320 / locals.var_iwe);
        (assign3290_e2932,)
    } else {
        (locals.var_igovinv_t,)
    }
};
        locals.var_igovinv_t = assign3290_e2934;
        locals.var_igovinv_t_rv = 0.0;

        let (assign3300_e2941,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3300_e2939: f64 = (p.p321 / locals.var_iwe);
        (assign3300_e2939,)
    } else {
        (locals.var_igovinvd_t,)
    }
};
        locals.var_igovinvd_t = assign3300_e2941;
        locals.var_igovinvd_t_rv = 0.0;

        let (assign3330_e2962,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3330_e2960: f64 = (p.p322 / locals.var_iwe);
        (assign3330_e2960,)
    } else {
        (locals.var_igovacc_t,)
    }
};
        locals.var_igovacc_t = assign3330_e2962;
        locals.var_igovacc_t_rv = 0.0;

        let (assign3340_e2969,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3340_e2967: f64 = (p.p323 / locals.var_iwe);
        (assign3340_e2967,)
    } else {
        (locals.var_igovaccd_t,)
    }
};
        locals.var_igovaccd_t = assign3340_e2969;
        locals.var_igovaccd_t_rv = 0.0;

        let (assign3350_e2974,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p324,)
    } else {
        (locals.var_stig_i,)
    }
};
        locals.var_stig_i = assign3350_e2974;
        locals.var_stig_i_rv = 0.0;

        let (assign3360_e2979,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p338,)
    } else {
        (locals.var_stigfn_i,)
    }
};
        locals.var_stigfn_i = assign3360_e2979;
        locals.var_stigfn_i_rv = 0.0;

        let (assign3370_e2984,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p325,)
    } else {
        (locals.var_gc2ch_i,)
    }
};
        locals.var_gc2ch_i = assign3370_e2984;
        locals.var_gc2ch_i_rv = 0.0;

        let (assign3380_e2989,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p326,)
    } else {
        (locals.var_gc3ch_i,)
    }
};
        locals.var_gc3ch_i = assign3380_e2989;
        locals.var_gc3ch_i_rv = 0.0;

        let (assign3390_e2994,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p327,)
    } else {
        (locals.var_gc2ovinv_i,)
    }
};
        locals.var_gc2ovinv_i = assign3390_e2994;
        locals.var_gc2ovinv_i_rv = 0.0;

        let (assign3400_e2999,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p337,)
    } else {
        (locals.var_gcovinvfn_i,)
    }
};
        locals.var_gcovinvfn_i = assign3400_e2999;
        locals.var_gcovinvfn_i_rv = 0.0;

        let (assign3410_e3004,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p328,)
    } else {
        (locals.var_gc3ovinv_i,)
    }
};
        locals.var_gc3ovinv_i = assign3410_e3004;
        locals.var_gc3ovinv_i_rv = 0.0;

        let (assign3420_e3009,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p329,)
    } else {
        (locals.var_gc2ovacc_i,)
    }
};
        locals.var_gc2ovacc_i = assign3420_e3009;
        locals.var_gc2ovacc_i_rv = 0.0;

        let (assign3430_e3014,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p330,)
    } else {
        (locals.var_gc3ovacc_i,)
    }
};
        locals.var_gc3ovacc_i = assign3430_e3014;
        locals.var_gc3ovacc_i_rv = 0.0;

        let (assign3440_e3021,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3440_e3019: f64 = (p.p331 * locals.var_ile);
        (assign3440_e3019,)
    } else {
        (locals.var_gcdov_i,)
    }
};
        locals.var_gcdov_i = assign3440_e3021;
        locals.var_gcdov_i_rv = 0.0;

        let (assign3450_e3026,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p332,)
    } else {
        (locals.var_gcvdov_i,)
    }
};
        locals.var_gcvdov_i = assign3450_e3026;
        locals.var_gcvdov_i_rv = 0.0;

        let (assign3460_e3031,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p333,)
    } else {
        (locals.var_chib_i,)
    }
};
        locals.var_chib_i = assign3460_e3031;
        locals.var_chib_i_rv = 0.0;

        let (assign3470_e3036,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p334,)
    } else {
        (locals.var_niginv_i,)
    }
};
        locals.var_niginv_i = assign3470_e3036;
        locals.var_niginv_i_rv = 0.0;

        let (assign3480_e3045,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3480_e3042: f64 = (p.p341 / locals.var_iwe);
        let assign3480_e3043: f64 = (p.p339 + assign3480_e3042);
        (assign3480_e3043,)
    } else {
        (locals.var_agidl_p,)
    }
};
        locals.var_agidl_p = assign3480_e3045;
        locals.var_agidl_p_rv = 0.0;

        let (assign3490_e3052, assign3490_e3052_d_n4, assign3490_e3052_d_n6, assign3490_e3052_d_n7, assign3490_e3052_d_n8, assign3490_e3052_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3490_e3050: f64 = (locals.var_agidl_p).max(0.0);
        (assign3490_e3050, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_agidl_i, locals.var_agidl_i_dn4, locals.var_agidl_i_dn6, locals.var_agidl_i_dn7, locals.var_agidl_i_dn8, locals.var_agidl_i_dn9,)
    }
};
        locals.var_agidl_i = assign3490_e3052;
        locals.var_agidl_i_dn4 = assign3490_e3052_d_n4;
        locals.var_agidl_i_dn6 = assign3490_e3052_d_n6;
        locals.var_agidl_i_dn7 = assign3490_e3052_d_n7;
        locals.var_agidl_i_dn8 = assign3490_e3052_d_n8;
        locals.var_agidl_i_dn9 = assign3490_e3052_d_n9;
        locals.var_agidl_i_rv = 0.0;

        let (assign3500_e3061,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3500_e3058: f64 = (p.p342 / locals.var_iwe);
        let assign3500_e3059: f64 = (p.p340 + assign3500_e3058);
        (assign3500_e3059,)
    } else {
        (locals.var_agidld_p,)
    }
};
        locals.var_agidld_p = assign3500_e3061;
        locals.var_agidld_p_rv = 0.0;

        let (assign3510_e3068, assign3510_e3068_d_n4, assign3510_e3068_d_n6, assign3510_e3068_d_n7, assign3510_e3068_d_n8, assign3510_e3068_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3510_e3066: f64 = (locals.var_agidld_p).max(0.0);
        (assign3510_e3066, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_agidld_i, locals.var_agidld_i_dn4, locals.var_agidld_i_dn6, locals.var_agidld_i_dn7, locals.var_agidld_i_dn8, locals.var_agidld_i_dn9,)
    }
};
        locals.var_agidld_i = assign3510_e3068;
        locals.var_agidld_i_dn4 = assign3510_e3068_d_n4;
        locals.var_agidld_i_dn6 = assign3510_e3068_d_n6;
        locals.var_agidld_i_dn7 = assign3510_e3068_d_n7;
        locals.var_agidld_i_dn8 = assign3510_e3068_d_n8;
        locals.var_agidld_i_dn9 = assign3510_e3068_d_n9;
        locals.var_agidld_i_rv = 0.0;

        let (assign3520_e3073,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p343,)
    } else {
        (locals.var_bgidl_t,)
    }
};
        locals.var_bgidl_t = assign3520_e3073;
        locals.var_bgidl_t_rv = 0.0;

        let (assign3530_e3078,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p344,)
    } else {
        (locals.var_bgidld_t,)
    }
};
        locals.var_bgidld_t = assign3530_e3078;
        locals.var_bgidld_t_rv = 0.0;

        let (assign3540_e3083,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p345,)
    } else {
        (locals.var_stbgidl_i,)
    }
};
        locals.var_stbgidl_i = assign3540_e3083;
        locals.var_stbgidl_i_rv = 0.0;

        let (assign3550_e3088,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p346,)
    } else {
        (locals.var_stbgidld_i,)
    }
};
        locals.var_stbgidld_i = assign3550_e3088;
        locals.var_stbgidld_i_rv = 0.0;

        let (assign3560_e3093,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p347,)
    } else {
        (locals.var_cgidl_i,)
    }
};
        locals.var_cgidl_i = assign3560_e3093;
        locals.var_cgidl_i_rv = 0.0;

        let (assign3570_e3098,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p348,)
    } else {
        (locals.var_cgidld_i,)
    }
};
        locals.var_cgidld_i = assign3570_e3098;
        locals.var_cgidld_i_rv = 0.0;

        let (assign3580_e3107,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3580_e3104: f64 = (p.p351 * locals.var_ile);
        let assign3580_e3105: f64 = (p.p349 + assign3580_e3104);
        (assign3580_e3105,)
    } else {
        (locals.var_dgidl_i,)
    }
};
        locals.var_dgidl_i = assign3580_e3107;
        locals.var_dgidl_i_rv = 0.0;

        let (assign3590_e3116,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3590_e3113: f64 = (p.p352 * locals.var_ile);
        let assign3590_e3114: f64 = (p.p350 + assign3590_e3113);
        (assign3590_e3114,)
    } else {
        (locals.var_dgidld_i,)
    }
};
        locals.var_dgidld_i = assign3590_e3116;
        locals.var_dgidld_i_rv = 0.0;

        let (assign3620_e3145,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p387,)
    } else {
        (locals.var_a2_t,)
    }
};
        locals.var_a2_t = assign3620_e3145;
        locals.var_a2_t_rv = 0.0;

        let (assign3630_e3150,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p388,)
    } else {
        (locals.var_sta2_i,)
    }
};
        locals.var_sta2_i = assign3630_e3150;
        locals.var_sta2_i_rv = 0.0;

        let (assign3640_e3167,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3640_e3157: f64 = (p.p390 * locals.var_ile);
        let assign3640_e3158: f64 = (1.0 + assign3640_e3157);
        let assign3640_e3159: f64 = (p.p389 * assign3640_e3158);
        let assign3640_e3163: f64 = (p.p391 * locals.var_iwe);
        let assign3640_e3164: f64 = (1.0 + assign3640_e3163);
        let assign3640_e3165: f64 = (assign3640_e3159 * assign3640_e3164);
        (assign3640_e3165,)
    } else {
        (locals.var_a3_p,)
    }
};
        locals.var_a3_p = assign3640_e3167;
        locals.var_a3_p_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3650_e3174,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3650_e3172: f64 = (locals.var_a3_p).max(0.0);
        (assign3650_e3172,)
    } else {
        (locals.var_a3_i,)
    }
};
        locals.var_a3_i = assign3650_e3174;
        locals.var_a3_i_rv = 0.0;

        let (assign3660_e3185,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3660_e3179: f64 = (2.0 * p.p353);
        let assign3660_e3182: f64 = (p.p354 * locals.var_we);
        let assign3660_e3183: f64 = (assign3660_e3179 + assign3660_e3182);
        (assign3660_e3183,)
    } else {
        (locals.var_we_edge,)
    }
};
        locals.var_we_edge = assign3660_e3185;
        locals.var_we_edge_rv = 0.0;

        let (assign3670_e3190,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p355,)
    } else {
        (locals.var_ctedge_i,)
    }
};
        locals.var_ctedge_i = assign3670_e3190;
        locals.var_ctedge_i_rv = 0.0;

        let (assign3680_e3199, assign3680_e3199_d_n4, assign3680_e3199_d_n6, assign3680_e3199_d_n7, assign3680_e3199_d_n8, assign3680_e3199_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3680_e3196: f64 = (locals.var_ile).powf(p.p358);
        let assign3680_e3197: f64 = (p.p357 * assign3680_e3196);
        (assign3680_e3197, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign3680_e3199;
        locals.var_temp_dn4 = assign3680_e3199_d_n4;
        locals.var_temp_dn6 = assign3680_e3199_d_n6;
        locals.var_temp_dn7 = assign3680_e3199_d_n7;
        locals.var_temp_dn8 = assign3680_e3199_d_n8;
        locals.var_temp_dn9 = assign3680_e3199_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign3690_e3214, assign3690_e3214_d_n4, assign3690_e3214_d_n6, assign3690_e3214_d_n7, assign3690_e3214_d_n8, assign3690_e3214_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3690_e3204: f64 = (p.p356 + locals.var_temp);
        let assign3690_e3207: f64 = (p.p359 * locals.var_iwe);
        let assign3690_e3208: f64 = (assign3690_e3204 + assign3690_e3207);
        let assign3690_e3211: f64 = (p.p360 * locals.var_iae);
        let assign3690_e3212: f64 = (assign3690_e3208 + assign3690_e3211);
        (assign3690_e3212, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    } else {
        (locals.var_vfb1edge_t, locals.var_vfb1edge_t_dn4, locals.var_vfb1edge_t_dn6, locals.var_vfb1edge_t_dn7, locals.var_vfb1edge_t_dn8, locals.var_vfb1edge_t_dn9,)
    }
};
        locals.var_vfb1edge_t = assign3690_e3214;
        locals.var_vfb1edge_t_dn4 = assign3690_e3214_d_n4;
        locals.var_vfb1edge_t_dn6 = assign3690_e3214_d_n6;
        locals.var_vfb1edge_t_dn7 = assign3690_e3214_d_n7;
        locals.var_vfb1edge_t_dn8 = assign3690_e3214_d_n8;
        locals.var_vfb1edge_t_dn9 = assign3690_e3214_d_n9;
        locals.var_vfb1edge_t_rv = 0.0;

        let (assign3700_e3219,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p361,)
    } else {
        (locals.var_vfb2edge_t,)
    }
};
        locals.var_vfb2edge_t = assign3700_e3219;
        locals.var_vfb2edge_t_rv = 0.0;

        let (assign3710_e3242,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3710_e3226: f64 = (p.p363 * locals.var_ile);
        let assign3710_e3227: f64 = (1.0 + assign3710_e3226);
        let assign3710_e3228: f64 = (p.p362 * assign3710_e3227);
        let assign3710_e3232: f64 = (p.p364 * locals.var_iwe);
        let assign3710_e3233: f64 = (1.0 + assign3710_e3232);
        let assign3710_e3234: f64 = (assign3710_e3228 * assign3710_e3233);
        let assign3710_e3238: f64 = (p.p365 * locals.var_iae);
        let assign3710_e3239: f64 = (1.0 + assign3710_e3238);
        let assign3710_e3240: f64 = (assign3710_e3234 * assign3710_e3239);
        (assign3710_e3240,)
    } else {
        (locals.var_stvfbedge_i,)
    }
};
        locals.var_stvfbedge_i = assign3710_e3242;
        locals.var_stvfbedge_i_rv = 0.0;

        let (assign3720_e3247,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p366,)
    } else {
        (locals.var_cic1edge_i,)
    }
};
        locals.var_cic1edge_i = assign3720_e3247;
        locals.var_cic1edge_i_rv = 0.0;

        let (assign3730_e3252,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p367,)
    } else {
        (locals.var_cic2edge_i,)
    }
};
        locals.var_cic2edge_i = assign3730_e3252;
        locals.var_cic2edge_i_rv = 0.0;

        let (assign3740_e3269, assign3740_e3269_d_n4, assign3740_e3269_d_n6, assign3740_e3269_d_n7, assign3740_e3269_d_n8, assign3740_e3269_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3740_e3257: f64 = (p.p368 * 2.0);
        let assign3740_e3260: f64 = (locals.var_lambda_le).powf(p.p369);
        let assign3740_e3261: f64 = (assign3740_e3257 * assign3740_e3260);
        let assign3740_e3265: f64 = (p.p370 * locals.var_iwe);
        let assign3740_e3266: f64 = (1.0 + assign3740_e3265);
        let assign3740_e3267: f64 = (assign3740_e3261 * assign3740_e3266);
        (assign3740_e3267, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign3740_e3269;
        locals.var_temp_dn4 = assign3740_e3269_d_n4;
        locals.var_temp_dn6 = assign3740_e3269_d_n6;
        locals.var_temp_dn7 = assign3740_e3269_d_n7;
        locals.var_temp_dn8 = assign3740_e3269_d_n8;
        locals.var_temp_dn9 = assign3740_e3269_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign3750_e3278, assign3750_e3278_d_n4, assign3750_e3278_d_n6, assign3750_e3278_d_n7, assign3750_e3278_d_n8, assign3750_e3278_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3750_e3274: f64 = (locals.var_temp).max(0.0);
        let assign3750_e3276: f64 = (assign3750_e3274).min(5.0);
        (assign3750_e3276, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn4 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn6 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn7 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn8 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn9 } else { 0.0 } } else { 0.0 },)
    } else {
        (locals.var_psce1edge_i, locals.var_psce1edge_i_dn4, locals.var_psce1edge_i_dn6, locals.var_psce1edge_i_dn7, locals.var_psce1edge_i_dn8, locals.var_psce1edge_i_dn9,)
    }
};
        locals.var_psce1edge_i = assign3750_e3278;
        locals.var_psce1edge_i_dn4 = assign3750_e3278_d_n4;
        locals.var_psce1edge_i_dn6 = assign3750_e3278_d_n6;
        locals.var_psce1edge_i_dn7 = assign3750_e3278_d_n7;
        locals.var_psce1edge_i_dn8 = assign3750_e3278_d_n8;
        locals.var_psce1edge_i_dn9 = assign3750_e3278_d_n9;
        locals.var_psce1edge_i_rv = 0.0;

        let (assign3760_e3289, assign3760_e3289_d_n4, assign3760_e3289_d_n6, assign3760_e3289_d_n7, assign3760_e3289_d_n8, assign3760_e3289_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3760_e3283: f64 = (p.p371 * locals.var_psce1edge_i);
        let assign3760_e3285: f64 = (assign3760_e3283 * locals.var_tox2_i);
        let assign3760_e3287: f64 = (assign3760_e3285 / locals.var_tox1_i);
        (assign3760_e3287, (((p.p371 * locals.var_psce1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p371 * locals.var_psce1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p371 * locals.var_psce1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p371 * locals.var_psce1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p371 * locals.var_psce1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_psce2edge_i, locals.var_psce2edge_i_dn4, locals.var_psce2edge_i_dn6, locals.var_psce2edge_i_dn7, locals.var_psce2edge_i_dn8, locals.var_psce2edge_i_dn9,)
    }
};
        locals.var_psce2edge_i = assign3760_e3289;
        locals.var_psce2edge_i_dn4 = assign3760_e3289_d_n4;
        locals.var_psce2edge_i_dn6 = assign3760_e3289_d_n6;
        locals.var_psce2edge_i_dn7 = assign3760_e3289_d_n7;
        locals.var_psce2edge_i_dn8 = assign3760_e3289_d_n8;
        locals.var_psce2edge_i_dn9 = assign3760_e3289_d_n9;
        locals.var_psce2edge_i_rv = 0.0;

        let (assign3770_e3302, assign3770_e3302_d_n4, assign3770_e3302_d_n6, assign3770_e3302_d_n7, assign3770_e3302_d_n8, assign3770_e3302_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3770_e3294: f64 = (locals.var_lambda_le).powf(p.p373);
        let assign3770_e3298: f64 = (p.p374 * locals.var_iwe);
        let assign3770_e3299: f64 = (1.0 + assign3770_e3298);
        let assign3770_e3300: f64 = (assign3770_e3294 * assign3770_e3299);
        (assign3770_e3300, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign3770_e3302;
        locals.var_temp_dn4 = assign3770_e3302_d_n4;
        locals.var_temp_dn6 = assign3770_e3302_d_n6;
        locals.var_temp_dn7 = assign3770_e3302_d_n7;
        locals.var_temp_dn8 = assign3770_e3302_d_n8;
        locals.var_temp_dn9 = assign3770_e3302_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign3780_e3309, assign3780_e3309_d_n4, assign3780_e3309_d_n6, assign3780_e3309_d_n7, assign3780_e3309_d_n8, assign3780_e3309_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3780_e3307: f64 = (p.p372 * locals.var_temp);
        (assign3780_e3307, (p.p372 * locals.var_temp_dn4), (p.p372 * locals.var_temp_dn6), (p.p372 * locals.var_temp_dn7), (p.p372 * locals.var_temp_dn8), (p.p372 * locals.var_temp_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign3780_e3309;
        locals.var_temp_dn4 = assign3780_e3309_d_n4;
        locals.var_temp_dn6 = assign3780_e3309_d_n6;
        locals.var_temp_dn7 = assign3780_e3309_d_n7;
        locals.var_temp_dn8 = assign3780_e3309_d_n8;
        locals.var_temp_dn9 = assign3780_e3309_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign3790_e3316, assign3790_e3316_d_n4, assign3790_e3316_d_n6, assign3790_e3316_d_n7, assign3790_e3316_d_n8, assign3790_e3316_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3790_e3314: f64 = (locals.var_temp).max(0.0);
        (assign3790_e3314, if locals.var_temp >= 0.0 { locals.var_temp_dn4 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn6 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn7 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn8 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn9 } else { 0.0 },)
    } else {
        (locals.var_cf1edge_i, locals.var_cf1edge_i_dn4, locals.var_cf1edge_i_dn6, locals.var_cf1edge_i_dn7, locals.var_cf1edge_i_dn8, locals.var_cf1edge_i_dn9,)
    }
};
        locals.var_cf1edge_i = assign3790_e3316;
        locals.var_cf1edge_i_dn4 = assign3790_e3316_d_n4;
        locals.var_cf1edge_i_dn6 = assign3790_e3316_d_n6;
        locals.var_cf1edge_i_dn7 = assign3790_e3316_d_n7;
        locals.var_cf1edge_i_dn8 = assign3790_e3316_d_n8;
        locals.var_cf1edge_i_dn9 = assign3790_e3316_d_n9;
        locals.var_cf1edge_i_rv = 0.0;

        let (assign3800_e3327, assign3800_e3327_d_n4, assign3800_e3327_d_n6, assign3800_e3327_d_n7, assign3800_e3327_d_n8, assign3800_e3327_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3800_e3321: f64 = (p.p375 * locals.var_cf1edge_i);
        let assign3800_e3323: f64 = (assign3800_e3321 * locals.var_tox2_i);
        let assign3800_e3325: f64 = (assign3800_e3323 / locals.var_tox1_i);
        (assign3800_e3325, (((p.p375 * locals.var_cf1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_cf1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_cf1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_cf1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_cf1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_cf2edge_i, locals.var_cf2edge_i_dn4, locals.var_cf2edge_i_dn6, locals.var_cf2edge_i_dn7, locals.var_cf2edge_i_dn8, locals.var_cf2edge_i_dn9,)
    }
};
        locals.var_cf2edge_i = assign3800_e3327;
        locals.var_cf2edge_i_dn4 = assign3800_e3327_d_n4;
        locals.var_cf2edge_i_dn6 = assign3800_e3327_d_n6;
        locals.var_cf2edge_i_dn7 = assign3800_e3327_d_n7;
        locals.var_cf2edge_i_dn8 = assign3800_e3327_d_n8;
        locals.var_cf2edge_i_dn9 = assign3800_e3327_d_n9;
        locals.var_cf2edge_i_rv = 0.0;

        let (assign3810_e3332,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p376,)
    } else {
        (locals.var_cfdedge_i,)
    }
};
        locals.var_cfdedge_i = assign3810_e3332;
        locals.var_cfdedge_i_rv = 0.0;

        let (assign3820_e3351, assign3820_e3351_d_n4, assign3820_e3351_d_n6, assign3820_e3351_d_n7, assign3820_e3351_d_n8, assign3820_e3351_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3820_e3338: f64 = (p.p377 * p.p378);
        let assign3820_e3340: f64 = (assign3820_e3338 / locals.var_le);
        let assign3820_e3343: f64 = (-locals.var_le);
        let assign3820_e3345: f64 = (assign3820_e3343 / p.p378);
        let assign3820_e3346: f64 = (assign3820_e3345).exp();
        let assign3820_e3347: f64 = (1.0 - assign3820_e3346);
        let assign3820_e3348: f64 = (assign3820_e3340 * assign3820_e3347);
        let assign3820_e3349: f64 = (1.0 + assign3820_e3348);
        (assign3820_e3349, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign3820_e3351;
        locals.var_temp_dn4 = assign3820_e3351_d_n4;
        locals.var_temp_dn6 = assign3820_e3351_d_n6;
        locals.var_temp_dn7 = assign3820_e3351_d_n7;
        locals.var_temp_dn8 = assign3820_e3351_d_n8;
        locals.var_temp_dn9 = assign3820_e3351_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign3830_e3358, assign3830_e3358_d_n4, assign3830_e3358_d_n6, assign3830_e3358_d_n7, assign3830_e3358_d_n8, assign3830_e3358_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3830_e3356: f64 = (locals.var_temp).max(1e-15);
        (assign3830_e3356, if locals.var_temp >= 1e-15 { locals.var_temp_dn4 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn6 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn7 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn8 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn9 } else { 0.0 },)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign3830_e3358;
        locals.var_temp_dn4 = assign3830_e3358_d_n4;
        locals.var_temp_dn6 = assign3830_e3358_d_n6;
        locals.var_temp_dn7 = assign3830_e3358_d_n7;
        locals.var_temp_dn8 = assign3830_e3358_d_n8;
        locals.var_temp_dn9 = assign3830_e3358_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign3840_e3375, assign3840_e3375_d_n4, assign3840_e3375_d_n6, assign3840_e3375_d_n7, assign3840_e3375_d_n8, assign3840_e3375_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3840_e3363: f64 = (p.p240 * locals.var_we_edge);
        let assign3840_e3366: f64 = (locals.var_temp * locals.var_le);
        let assign3840_e3367: f64 = (assign3840_e3363 / assign3840_e3366);
        let assign3840_e3371: f64 = (p.p379 * locals.var_iwe);
        let assign3840_e3372: f64 = (1.0 + assign3840_e3371);
        let assign3840_e3373: f64 = (assign3840_e3367 * assign3840_e3372);
        (assign3840_e3373, ((-((assign3840_e3363 * (locals.var_temp_dn4 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (locals.var_temp_dn6 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (locals.var_temp_dn7 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (locals.var_temp_dn8 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (locals.var_temp_dn9 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372),)
    } else {
        (locals.var_betnedge_t, locals.var_betnedge_t_dn4, locals.var_betnedge_t_dn6, locals.var_betnedge_t_dn7, locals.var_betnedge_t_dn8, locals.var_betnedge_t_dn9,)
    }
};
        locals.var_betnedge_t = assign3840_e3375;
        locals.var_betnedge_t_dn4 = assign3840_e3375_d_n4;
        locals.var_betnedge_t_dn6 = assign3840_e3375_d_n6;
        locals.var_betnedge_t_dn7 = assign3840_e3375_d_n7;
        locals.var_betnedge_t_dn8 = assign3840_e3375_d_n8;
        locals.var_betnedge_t_dn9 = assign3840_e3375_d_n9;
        locals.var_betnedge_t_rv = 0.0;

        let (assign3850_e3394,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3850_e3381: f64 = (p.p381 * locals.var_ile);
        let assign3850_e3382: f64 = (p.p380 + assign3850_e3381);
        let assign3850_e3385: f64 = (p.p382 * locals.var_iwe);
        let assign3850_e3386: f64 = (assign3850_e3382 + assign3850_e3385);
        let assign3850_e3389: f64 = (p.p383 * locals.var_ile);
        let assign3850_e3391: f64 = (assign3850_e3389 * locals.var_iwe);
        let assign3850_e3392: f64 = (assign3850_e3386 + assign3850_e3391);
        (assign3850_e3392,)
    } else {
        (locals.var_stbetedge_i,)
    }
};
        locals.var_stbetedge_i = assign3850_e3394;
        locals.var_stbetedge_i_rv = 0.0;

        let (assign3860_e3401,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3860_e3399: f64 = (locals.var_wecv * locals.var_lecv);
        (assign3860_e3399,)
    } else {
        (locals.var_areaq_i,)
    }
};
        locals.var_areaq_i = assign3860_e3401;
        locals.var_areaq_i_rv = 0.0;

        let (assign3870_e3410, assign3870_e3410_d_n4, assign3870_e3410_d_n6, assign3870_e3410_d_n7, assign3870_e3410_d_n8, assign3870_e3410_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3870_e3407: f64 = (p.p393 * locals.var_lphy);
        let assign3870_e3408: f64 = (p.p392 + assign3870_e3407);
        (assign3870_e3408, (p.p393 * locals.var_lphy_dn4), (p.p393 * locals.var_lphy_dn6), (p.p393 * locals.var_lphy_dn7), (p.p393 * locals.var_lphy_dn8), (p.p393 * locals.var_lphy_dn9),)
    } else {
        (locals.var_cgbov_p, locals.var_cgbov_p_dn4, locals.var_cgbov_p_dn6, locals.var_cgbov_p_dn7, locals.var_cgbov_p_dn8, locals.var_cgbov_p_dn9,)
    }
};
        locals.var_cgbov_p = assign3870_e3410;
        locals.var_cgbov_p_dn4 = assign3870_e3410_d_n4;
        locals.var_cgbov_p_dn6 = assign3870_e3410_d_n6;
        locals.var_cgbov_p_dn7 = assign3870_e3410_d_n7;
        locals.var_cgbov_p_dn8 = assign3870_e3410_d_n8;
        locals.var_cgbov_p_dn9 = assign3870_e3410_d_n9;
        locals.var_cgbov_p_rv = 0.0;

        let (assign3880_e3417, assign3880_e3417_d_n4, assign3880_e3417_d_n6, assign3880_e3417_d_n7, assign3880_e3417_d_n8, assign3880_e3417_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3880_e3415: f64 = (locals.var_cgbov_p).max(0.0);
        (assign3880_e3415, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn4 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn6 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn7 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn8 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cgbov_i, locals.var_cgbov_i_dn4, locals.var_cgbov_i_dn6, locals.var_cgbov_i_dn7, locals.var_cgbov_i_dn8, locals.var_cgbov_i_dn9,)
    }
};
        locals.var_cgbov_i = assign3880_e3417;
        locals.var_cgbov_i_dn4 = assign3880_e3417_d_n4;
        locals.var_cgbov_i_dn6 = assign3880_e3417_d_n6;
        locals.var_cgbov_i_dn7 = assign3880_e3417_d_n7;
        locals.var_cgbov_i_dn8 = assign3880_e3417_d_n8;
        locals.var_cgbov_i_dn9 = assign3880_e3417_d_n9;
        locals.var_cgbov_i_rv = 0.0;

        let (assign3890_e3424,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3890_e3422: f64 = (p.p394 * 1000000.0);
        (assign3890_e3422,)
    } else {
        (locals.var_nsdac_i,)
    }
};
        locals.var_nsdac_i = assign3890_e3424;
        locals.var_nsdac_i_rv = 0.0;

        let (assign3900_e3433,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3900_e3429: f64 = (p.p395 * locals.var_wecv);
        let assign3900_e3431: f64 = (assign3900_e3429 / locals.var_wen);
        (assign3900_e3431,)
    } else {
        (locals.var_fif_i,)
    }
};
        locals.var_fif_i = assign3900_e3433;
        locals.var_fif_i_rv = 0.0;

        let (assign3910_e3438,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p396,)
    } else {
        (locals.var_fsceac_i,)
    }
};
        locals.var_fsceac_i = assign3910_e3438;
        locals.var_fsceac_i_rv = 0.0;

        let (assign3920_e3443, assign3920_e3443_d_n4, assign3920_e3443_d_n6, assign3920_e3443_d_n7, assign3920_e3443_d_n8, assign3920_e3443_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9,)
    } else {
        (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9,)
    }
};
        locals.var_vfbac1_t = assign3920_e3443;
        locals.var_vfbac1_t_dn4 = assign3920_e3443_d_n4;
        locals.var_vfbac1_t_dn6 = assign3920_e3443_d_n6;
        locals.var_vfbac1_t_dn7 = assign3920_e3443_d_n7;
        locals.var_vfbac1_t_dn8 = assign3920_e3443_d_n8;
        locals.var_vfbac1_t_dn9 = assign3920_e3443_d_n9;
        locals.var_vfbac1_t_rv = 0.0;

        let (assign3930_e3448, assign3930_e3448_d_n4, assign3930_e3448_d_n6, assign3930_e3448_d_n7, assign3930_e3448_d_n8, assign3930_e3448_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign3930_e3448;
        locals.var_vfbac2_t_dn4 = assign3930_e3448_d_n4;
        locals.var_vfbac2_t_dn6 = assign3930_e3448_d_n6;
        locals.var_vfbac2_t_dn7 = assign3930_e3448_d_n7;
        locals.var_vfbac2_t_dn8 = assign3930_e3448_d_n8;
        locals.var_vfbac2_t_dn9 = assign3930_e3448_d_n9;
        locals.var_vfbac2_t_rv = 0.0;

        let (assign3940_e3453,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_psce1_i,)
    } else {
        (locals.var_psceac1_i,)
    }
};
        locals.var_psceac1_i = assign3940_e3453;
        locals.var_psceac1_i_rv = 0.0;

        let (assign3950_e3458,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_psce2_i,)
    } else {
        (locals.var_psceac2_i,)
    }
};
        locals.var_psceac2_i = assign3950_e3458;
        locals.var_psceac2_i_rv = 0.0;

        let (assign3960_e3463, assign3960_e3463_d_n4, assign3960_e3463_d_n6, assign3960_e3463_d_n7, assign3960_e3463_d_n8, assign3960_e3463_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9,)
    } else {
        (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9,)
    }
};
        locals.var_cfac_p = assign3960_e3463;
        locals.var_cfac_p_dn4 = assign3960_e3463_d_n4;
        locals.var_cfac_p_dn6 = assign3960_e3463_d_n6;
        locals.var_cfac_p_dn7 = assign3960_e3463_d_n7;
        locals.var_cfac_p_dn8 = assign3960_e3463_d_n8;
        locals.var_cfac_p_dn9 = assign3960_e3463_d_n9;
        locals.var_cfac_p_rv = 0.0;

        let (assign3970_e3468, assign3970_e3468_d_n4, assign3970_e3468_d_n6, assign3970_e3468_d_n7, assign3970_e3468_d_n8, assign3970_e3468_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9,)
    } else {
        (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9,)
    }
};
        locals.var_cfac1_t = assign3970_e3468;
        locals.var_cfac1_t_dn4 = assign3970_e3468_d_n4;
        locals.var_cfac1_t_dn6 = assign3970_e3468_d_n6;
        locals.var_cfac1_t_dn7 = assign3970_e3468_d_n7;
        locals.var_cfac1_t_dn8 = assign3970_e3468_d_n8;
        locals.var_cfac1_t_dn9 = assign3970_e3468_d_n9;
        locals.var_cfac1_t_rv = 0.0;

        let (assign3980_e3473, assign3980_e3473_d_n4, assign3980_e3473_d_n6, assign3980_e3473_d_n7, assign3980_e3473_d_n8, assign3980_e3473_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9,)
    } else {
        (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9,)
    }
};
        locals.var_cfac2_t = assign3980_e3473;
        locals.var_cfac2_t_dn4 = assign3980_e3473_d_n4;
        locals.var_cfac2_t_dn6 = assign3980_e3473_d_n6;
        locals.var_cfac2_t_dn7 = assign3980_e3473_d_n7;
        locals.var_cfac2_t_dn8 = assign3980_e3473_d_n8;
        locals.var_cfac2_t_dn9 = assign3980_e3473_d_n9;
        locals.var_cfac2_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign3990_e3478, assign3990_e3478_d_n4, assign3990_e3478_d_n6, assign3990_e3478_d_n7, assign3990_e3478_d_n8, assign3990_e3478_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9,)
    } else {
        (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9,)
    }
};
        locals.var_thesatac_p = assign3990_e3478;
        locals.var_thesatac_p_dn4 = assign3990_e3478_d_n4;
        locals.var_thesatac_p_dn6 = assign3990_e3478_d_n6;
        locals.var_thesatac_p_dn7 = assign3990_e3478_d_n7;
        locals.var_thesatac_p_dn8 = assign3990_e3478_d_n8;
        locals.var_thesatac_p_dn9 = assign3990_e3478_d_n9;
        locals.var_thesatac_p_rv = 0.0;

        let (assign4000_e3483, assign4000_e3483_d_n4, assign4000_e3483_d_n6, assign4000_e3483_d_n7, assign4000_e3483_d_n8, assign4000_e3483_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9,)
    } else {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9,)
    }
};
        locals.var_thesatac_t = assign4000_e3483;
        locals.var_thesatac_t_dn4 = assign4000_e3483_d_n4;
        locals.var_thesatac_t_dn6 = assign4000_e3483_d_n6;
        locals.var_thesatac_t_dn7 = assign4000_e3483_d_n7;
        locals.var_thesatac_t_dn8 = assign4000_e3483_d_n8;
        locals.var_thesatac_t_dn9 = assign4000_e3483_d_n9;
        locals.var_thesatac_t_rv = 0.0;

        let (assign4010_e3488,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_ax_i,)
    } else {
        (locals.var_axac_i,)
    }
};
        locals.var_axac_i = assign4010_e3488;
        locals.var_axac_i_rv = 0.0;

        let (assign4020_e3493,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_alp_i,)
    } else {
        (locals.var_alpac_i,)
    }
};
        locals.var_alpac_i = assign4020_e3493;
        locals.var_alpac_i_rv = 0.0;

        let assign4030_e3496: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign4030_e3496;
        locals.var_guard98_rv = 0.0;

        let (assign4040_e3503,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p207,)
    } else {
        (locals.var_vfbaco_i,)
    }
};
        locals.var_vfbaco_i = assign4040_e3503;
        locals.var_vfbaco_i_rv = 0.0;

        let assign4050_e3505: f64 = if param_given[397] { 1.0 } else { 0.0 };
        let assign4050_e3507: f64 = if assign4050_e3505 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign4050_e3507;
        locals.var_guard99_rv = 0.0;

        let (assign4060_e3516,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) {
        (p.p397,)
    } else {
        (locals.var_vfbaco_i,)
    }
};
        locals.var_vfbaco_i = assign4060_e3516;
        locals.var_vfbaco_i_rv = 0.0;

        let (assign4070_e3523,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p208,)
    } else {
        (locals.var_vfbacl_i,)
    }
};
        locals.var_vfbacl_i = assign4070_e3523;
        locals.var_vfbacl_i_rv = 0.0;

        let assign4080_e3525: f64 = if param_given[398] { 1.0 } else { 0.0 };
        let assign4080_e3527: f64 = if assign4080_e3525 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard100 = assign4080_e3527;
        locals.var_guard100_rv = 0.0;

        let (assign4090_e3536,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 != 0.0)) {
        (p.p398,)
    } else {
        (locals.var_vfbacl_i,)
    }
};
        locals.var_vfbacl_i = assign4090_e3536;
        locals.var_vfbacl_i_rv = 0.0;

        let (assign4100_e3543,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p209,)
    } else {
        (locals.var_vfbaclexp_i,)
    }
};
        locals.var_vfbaclexp_i = assign4100_e3543;
        locals.var_vfbaclexp_i_rv = 0.0;

        let assign4110_e3545: f64 = if param_given[399] { 1.0 } else { 0.0 };
        let assign4110_e3547: f64 = if assign4110_e3545 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard101 = assign4110_e3547;
        locals.var_guard101_rv = 0.0;

        let (assign4120_e3556,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard101 != 0.0)) {
        (p.p399,)
    } else {
        (locals.var_vfbaclexp_i,)
    }
};
        locals.var_vfbaclexp_i = assign4120_e3556;
        locals.var_vfbaclexp_i_rv = 0.0;

        let (assign4130_e3563,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p212,)
    } else {
        (locals.var_vfbacw_i,)
    }
};
        locals.var_vfbacw_i = assign4130_e3563;
        locals.var_vfbacw_i_rv = 0.0;

        let assign4140_e3565: f64 = if param_given[402] { 1.0 } else { 0.0 };
        let assign4140_e3567: f64 = if assign4140_e3565 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard102 = assign4140_e3567;
        locals.var_guard102_rv = 0.0;

        let (assign4150_e3576,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard102 != 0.0)) {
        (p.p402,)
    } else {
        (locals.var_vfbacw_i,)
    }
};
        locals.var_vfbacw_i = assign4150_e3576;
        locals.var_vfbacw_i_rv = 0.0;

        let (assign4160_e3583,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p213,)
    } else {
        (locals.var_vfbaclw_i,)
    }
};
        locals.var_vfbaclw_i = assign4160_e3583;
        locals.var_vfbaclw_i_rv = 0.0;

        let assign4170_e3585: f64 = if param_given[403] { 1.0 } else { 0.0 };
        let assign4170_e3587: f64 = if assign4170_e3585 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard103 = assign4170_e3587;
        locals.var_guard103_rv = 0.0;

        let (assign4180_e3596,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard103 != 0.0)) {
        (p.p403,)
    } else {
        (locals.var_vfbaclw_i,)
    }
};
        locals.var_vfbaclw_i = assign4180_e3596;
        locals.var_vfbaclw_i_rv = 0.0;

        let (assign4190_e3603,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p210,)
    } else {
        (locals.var_vfbacl2_i,)
    }
};
        locals.var_vfbacl2_i = assign4190_e3603;
        locals.var_vfbacl2_i_rv = 0.0;

        let assign4200_e3605: f64 = if param_given[400] { 1.0 } else { 0.0 };
        let assign4200_e3607: f64 = if assign4200_e3605 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard104 = assign4200_e3607;
        locals.var_guard104_rv = 0.0;

        let (assign4210_e3616,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard104 != 0.0)) {
        (p.p400,)
    } else {
        (locals.var_vfbacl2_i,)
    }
};
        locals.var_vfbacl2_i = assign4210_e3616;
        locals.var_vfbacl2_i_rv = 0.0;

        let (assign4220_e3623,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p211,)
    } else {
        (locals.var_vfbaclexp2_i,)
    }
};
        locals.var_vfbaclexp2_i = assign4220_e3623;
        locals.var_vfbaclexp2_i_rv = 0.0;

        let assign4230_e3625: f64 = if param_given[401] { 1.0 } else { 0.0 };
        let assign4230_e3627: f64 = if assign4230_e3625 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign4230_e3627;
        locals.var_guard105_rv = 0.0;

        let (assign4240_e3636,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard105 != 0.0)) {
        (p.p401,)
    } else {
        (locals.var_vfbaclexp2_i,)
    }
};
        locals.var_vfbaclexp2_i = assign4240_e3636;
        locals.var_vfbaclexp2_i_rv = 0.0;

        let (assign4250_e3655, assign4250_e3655_d_n4, assign4250_e3655_d_n6, assign4250_e3655_d_n7, assign4250_e3655_d_n8, assign4250_e3655_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4250_e3644: f64 = (locals.var_ile).powf(locals.var_vfbaclexp_i);
        let assign4250_e3645: f64 = (locals.var_vfbacl_i * assign4250_e3644);
        let assign4250_e3650: f64 = (locals.var_ile).powf(locals.var_vfbaclexp2_i);
        let assign4250_e3651: f64 = (locals.var_vfbacl2_i * assign4250_e3650);
        let assign4250_e3652: f64 = (1.0 + assign4250_e3651);
        let assign4250_e3653: f64 = (assign4250_e3645 / assign4250_e3652);
        (assign4250_e3653, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign4250_e3655;
        locals.var_temp_dn4 = assign4250_e3655_d_n4;
        locals.var_temp_dn6 = assign4250_e3655_d_n6;
        locals.var_temp_dn7 = assign4250_e3655_d_n7;
        locals.var_temp_dn8 = assign4250_e3655_d_n8;
        locals.var_temp_dn9 = assign4250_e3655_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign4260_e3672, assign4260_e3672_d_n4, assign4260_e3672_d_n6, assign4260_e3672_d_n7, assign4260_e3672_d_n8, assign4260_e3672_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4260_e3662: f64 = (locals.var_vfbaco_i + locals.var_temp);
        let assign4260_e3665: f64 = (locals.var_vfbacw_i * locals.var_iwe);
        let assign4260_e3666: f64 = (assign4260_e3662 + assign4260_e3665);
        let assign4260_e3669: f64 = (locals.var_vfbaclw_i * locals.var_iae);
        let assign4260_e3670: f64 = (assign4260_e3666 + assign4260_e3669);
        (assign4260_e3670, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    } else {
        (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9,)
    }
};
        locals.var_vfbac1_t = assign4260_e3672;
        locals.var_vfbac1_t_dn4 = assign4260_e3672_d_n4;
        locals.var_vfbac1_t_dn6 = assign4260_e3672_d_n6;
        locals.var_vfbac1_t_dn7 = assign4260_e3672_d_n7;
        locals.var_vfbac1_t_dn8 = assign4260_e3672_d_n8;
        locals.var_vfbac1_t_dn9 = assign4260_e3672_d_n9;
        locals.var_vfbac1_t_rv = 0.0;

        let (assign4270_e3679,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p214,)
    } else {
        (locals.var_vfbbaco_i,)
    }
};
        locals.var_vfbbaco_i = assign4270_e3679;
        locals.var_vfbbaco_i_rv = 0.0;

        let assign4280_e3681: f64 = if param_given[404] { 1.0 } else { 0.0 };
        let assign4280_e3683: f64 = if assign4280_e3681 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign4280_e3683;
        locals.var_guard106_rv = 0.0;

        let (assign4290_e3692,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard106 != 0.0)) {
        (p.p404,)
    } else {
        (locals.var_vfbbaco_i,)
    }
};
        locals.var_vfbbaco_i = assign4290_e3692;
        locals.var_vfbbaco_i_rv = 0.0;

        let (assign4300_e3699,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p215,)
    } else {
        (locals.var_vfblbaco_i,)
    }
};
        locals.var_vfblbaco_i = assign4300_e3699;
        locals.var_vfblbaco_i_rv = 0.0;

        let assign4310_e3701: f64 = if param_given[405] { 1.0 } else { 0.0 };
        let assign4310_e3703: f64 = if assign4310_e3701 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign4310_e3703;
        locals.var_guard107_rv = 0.0;

        let (assign4320_e3712,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard107 != 0.0)) {
        (p.p405,)
    } else {
        (locals.var_vfblbaco_i,)
    }
};
        locals.var_vfblbaco_i = assign4320_e3712;
        locals.var_vfblbaco_i_rv = 0.0;

        let (assign4330_e3727, assign4330_e3727_d_n4, assign4330_e3727_d_n6, assign4330_e3727_d_n7, assign4330_e3727_d_n8, assign4330_e3727_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4330_e3720: f64 = (locals.var_vfblbaco_i * locals.var_tox2_i);
        let assign4330_e3722: f64 = (assign4330_e3720 / locals.var_tox1_i);
        let assign4330_e3724: f64 = (assign4330_e3722 * locals.var_temp);
        let assign4330_e3725: f64 = (locals.var_vfbbaco_i + assign4330_e3724);
        (assign4330_e3725, (assign4330_e3722 * locals.var_temp_dn4), (assign4330_e3722 * locals.var_temp_dn6), (assign4330_e3722 * locals.var_temp_dn7), (assign4330_e3722 * locals.var_temp_dn8), (assign4330_e3722 * locals.var_temp_dn9),)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign4330_e3727;
        locals.var_vfbac2_t_dn4 = assign4330_e3727_d_n4;
        locals.var_vfbac2_t_dn6 = assign4330_e3727_d_n6;
        locals.var_vfbac2_t_dn7 = assign4330_e3727_d_n7;
        locals.var_vfbac2_t_dn8 = assign4330_e3727_d_n8;
        locals.var_vfbac2_t_dn9 = assign4330_e3727_d_n9;
        locals.var_vfbac2_t_rv = 0.0;

        let (assign4340_e3734,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p224,)
    } else {
        (locals.var_psceacl_i,)
    }
};
        locals.var_psceacl_i = assign4340_e3734;
        locals.var_psceacl_i_rv = 0.0;

        let assign4350_e3736: f64 = if param_given[406] { 1.0 } else { 0.0 };
        let assign4350_e3738: f64 = if assign4350_e3736 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign4350_e3738;
        locals.var_guard108_rv = 0.0;

        let (assign4360_e3747,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard108 != 0.0)) {
        (p.p406,)
    } else {
        (locals.var_psceacl_i,)
    }
};
        locals.var_psceacl_i = assign4360_e3747;
        locals.var_psceacl_i_rv = 0.0;

        let (assign4370_e3754,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p225,)
    } else {
        (locals.var_psceaclexp_i,)
    }
};
        locals.var_psceaclexp_i = assign4370_e3754;
        locals.var_psceaclexp_i_rv = 0.0;

        let assign4380_e3756: f64 = if param_given[407] { 1.0 } else { 0.0 };
        let assign4380_e3758: f64 = if assign4380_e3756 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign4380_e3758;
        locals.var_guard109_rv = 0.0;

        let (assign4390_e3767,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard109 != 0.0)) {
        (p.p407,)
    } else {
        (locals.var_psceaclexp_i,)
    }
};
        locals.var_psceaclexp_i = assign4390_e3767;
        locals.var_psceaclexp_i_rv = 0.0;

        let (assign4400_e3774,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p226,)
    } else {
        (locals.var_psceacw_i,)
    }
};
        locals.var_psceacw_i = assign4400_e3774;
        locals.var_psceacw_i_rv = 0.0;

        let assign4410_e3776: f64 = if param_given[408] { 1.0 } else { 0.0 };
        let assign4410_e3778: f64 = if assign4410_e3776 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign4410_e3778;
        locals.var_guard110_rv = 0.0;

        let (assign4420_e3787,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard110 != 0.0)) {
        (p.p408,)
    } else {
        (locals.var_psceacw_i,)
    }
};
        locals.var_psceacw_i = assign4420_e3787;
        locals.var_psceacw_i_rv = 0.0;

        let (assign4430_e3806,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4430_e3794: f64 = (locals.var_psceacl_i * 2.0);
        let assign4430_e3797: f64 = (locals.var_lambda_le).powf(locals.var_psceaclexp_i);
        let assign4430_e3798: f64 = (assign4430_e3794 * assign4430_e3797);
        let assign4430_e3802: f64 = (locals.var_psceacw_i * locals.var_iwe);
        let assign4430_e3803: f64 = (1.0 + assign4430_e3802);
        let assign4430_e3804: f64 = (assign4430_e3798 * assign4430_e3803);
        (assign4430_e3804,)
    } else {
        (locals.var_psceac_p,)
    }
};
        locals.var_psceac_p = assign4430_e3806;
        locals.var_psceac_p_rv = 0.0;

        let (assign4440_e3817,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4440_e3813: f64 = (locals.var_psceac_p).max(0.0);
        let assign4440_e3815: f64 = (assign4440_e3813).min(5.0);
        (assign4440_e3815,)
    } else {
        (locals.var_psceac1_i,)
    }
};
        locals.var_psceac1_i = assign4440_e3817;
        locals.var_psceac1_i_rv = 0.0;

        let (assign4450_e3830,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4450_e3824: f64 = (p.p227 * locals.var_psceac1_i);
        let assign4450_e3826: f64 = (assign4450_e3824 * locals.var_tox2_i);
        let assign4450_e3828: f64 = (assign4450_e3826 / locals.var_tox1_i);
        (assign4450_e3828,)
    } else {
        (locals.var_psceac2_i,)
    }
};
        locals.var_psceac2_i = assign4450_e3830;
        locals.var_psceac2_i_rv = 0.0;

        let (assign4460_e3837,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p231,)
    } else {
        (locals.var_cfacl_i,)
    }
};
        locals.var_cfacl_i = assign4460_e3837;
        locals.var_cfacl_i_rv = 0.0;

        let assign4470_e3839: f64 = if param_given[409] { 1.0 } else { 0.0 };
        let assign4470_e3841: f64 = if assign4470_e3839 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign4470_e3841;
        locals.var_guard111_rv = 0.0;

        let (assign4480_e3850,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard111 != 0.0)) {
        (p.p409,)
    } else {
        (locals.var_cfacl_i,)
    }
};
        locals.var_cfacl_i = assign4480_e3850;
        locals.var_cfacl_i_rv = 0.0;

        let (assign4490_e3857,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p232,)
    } else {
        (locals.var_cfaclexp_i,)
    }
};
        locals.var_cfaclexp_i = assign4490_e3857;
        locals.var_cfaclexp_i_rv = 0.0;

        let assign4500_e3859: f64 = if param_given[410] { 1.0 } else { 0.0 };
        let assign4500_e3861: f64 = if assign4500_e3859 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign4500_e3861;
        locals.var_guard112_rv = 0.0;

        let (assign4510_e3870,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard112 != 0.0)) {
        (p.p410,)
    } else {
        (locals.var_cfaclexp_i,)
    }
};
        locals.var_cfaclexp_i = assign4510_e3870;
        locals.var_cfaclexp_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign4520_e3877,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p233,)
    } else {
        (locals.var_cfacw_i,)
    }
};
        locals.var_cfacw_i = assign4520_e3877;
        locals.var_cfacw_i_rv = 0.0;

        let assign4530_e3879: f64 = if param_given[411] { 1.0 } else { 0.0 };
        let assign4530_e3881: f64 = if assign4530_e3879 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign4530_e3881;
        locals.var_guard113_rv = 0.0;

        let (assign4540_e3890,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard113 != 0.0)) {
        (p.p411,)
    } else {
        (locals.var_cfacw_i,)
    }
};
        locals.var_cfacw_i = assign4540_e3890;
        locals.var_cfacw_i_rv = 0.0;

        let (assign4550_e3905, assign4550_e3905_d_n4, assign4550_e3905_d_n6, assign4550_e3905_d_n7, assign4550_e3905_d_n8, assign4550_e3905_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4550_e3897: f64 = (locals.var_lambda_le).powf(locals.var_cfaclexp_i);
        let assign4550_e3901: f64 = (locals.var_cfacw_i * locals.var_iwe);
        let assign4550_e3902: f64 = (1.0 + assign4550_e3901);
        let assign4550_e3903: f64 = (assign4550_e3897 * assign4550_e3902);
        (assign4550_e3903, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign4550_e3905;
        locals.var_temp_dn4 = assign4550_e3905_d_n4;
        locals.var_temp_dn6 = assign4550_e3905_d_n6;
        locals.var_temp_dn7 = assign4550_e3905_d_n7;
        locals.var_temp_dn8 = assign4550_e3905_d_n8;
        locals.var_temp_dn9 = assign4550_e3905_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign4560_e3914, assign4560_e3914_d_n4, assign4560_e3914_d_n6, assign4560_e3914_d_n7, assign4560_e3914_d_n8, assign4560_e3914_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4560_e3912: f64 = (locals.var_cfacl_i * locals.var_temp);
        (assign4560_e3912, (locals.var_cfacl_i * locals.var_temp_dn4), (locals.var_cfacl_i * locals.var_temp_dn6), (locals.var_cfacl_i * locals.var_temp_dn7), (locals.var_cfacl_i * locals.var_temp_dn8), (locals.var_cfacl_i * locals.var_temp_dn9),)
    } else {
        (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9,)
    }
};
        locals.var_cfac_p = assign4560_e3914;
        locals.var_cfac_p_dn4 = assign4560_e3914_d_n4;
        locals.var_cfac_p_dn6 = assign4560_e3914_d_n6;
        locals.var_cfac_p_dn7 = assign4560_e3914_d_n7;
        locals.var_cfac_p_dn8 = assign4560_e3914_d_n8;
        locals.var_cfac_p_dn9 = assign4560_e3914_d_n9;
        locals.var_cfac_p_rv = 0.0;

        let (assign4570_e3923, assign4570_e3923_d_n4, assign4570_e3923_d_n6, assign4570_e3923_d_n7, assign4570_e3923_d_n8, assign4570_e3923_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4570_e3921: f64 = (locals.var_cfac_p).max(0.0);
        (assign4570_e3921, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn4 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn6 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn7 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn8 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9,)
    }
};
        locals.var_cfac1_t = assign4570_e3923;
        locals.var_cfac1_t_dn4 = assign4570_e3923_d_n4;
        locals.var_cfac1_t_dn6 = assign4570_e3923_d_n6;
        locals.var_cfac1_t_dn7 = assign4570_e3923_d_n7;
        locals.var_cfac1_t_dn8 = assign4570_e3923_d_n8;
        locals.var_cfac1_t_dn9 = assign4570_e3923_d_n9;
        locals.var_cfac1_t_rv = 0.0;

        let (assign4580_e3936, assign4580_e3936_d_n4, assign4580_e3936_d_n6, assign4580_e3936_d_n7, assign4580_e3936_d_n8, assign4580_e3936_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4580_e3930: f64 = (p.p234 * locals.var_cfac1_t);
        let assign4580_e3932: f64 = (assign4580_e3930 * locals.var_tox2_i);
        let assign4580_e3934: f64 = (assign4580_e3932 / locals.var_tox1_i);
        (assign4580_e3934, (((p.p234 * locals.var_cfac1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cfac1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cfac1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cfac1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cfac1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9,)
    }
};
        locals.var_cfac2_t = assign4580_e3936;
        locals.var_cfac2_t_dn4 = assign4580_e3936_d_n4;
        locals.var_cfac2_t_dn6 = assign4580_e3936_d_n6;
        locals.var_cfac2_t_dn7 = assign4580_e3936_d_n7;
        locals.var_cfac2_t_dn8 = assign4580_e3936_d_n8;
        locals.var_cfac2_t_dn9 = assign4580_e3936_d_n9;
        locals.var_cfac2_t_rv = 0.0;

        let (assign4590_e3943,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p289,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign4590_e3943;
        locals.var_thesataco_i_rv = 0.0;

        let assign4600_e3945: f64 = if param_given[412] { 1.0 } else { 0.0 };
        let assign4600_e3947: f64 = if assign4600_e3945 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard114 = assign4600_e3947;
        locals.var_guard114_rv = 0.0;

        let (assign4610_e3956,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard114 != 0.0)) {
        (p.p412,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign4610_e3956;
        locals.var_thesataco_i_rv = 0.0;

        let (assign4620_e3963,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p290,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign4620_e3963;
        locals.var_thesatacl_i_rv = 0.0;

        let assign4630_e3965: f64 = if param_given[413] { 1.0 } else { 0.0 };
        let assign4630_e3967: f64 = if assign4630_e3965 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard115 = assign4630_e3967;
        locals.var_guard115_rv = 0.0;

        let (assign4640_e3976,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard115 != 0.0)) {
        (p.p413,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign4640_e3976;
        locals.var_thesatacl_i_rv = 0.0;

        let (assign4650_e3983,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p291,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign4650_e3983;
        locals.var_thesataclexp_i_rv = 0.0;

        let assign4660_e3985: f64 = if param_given[414] { 1.0 } else { 0.0 };
        let assign4660_e3987: f64 = if assign4660_e3985 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign4660_e3987;
        locals.var_guard116_rv = 0.0;

        let (assign4670_e3996,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard116 != 0.0)) {
        (p.p414,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign4670_e3996;
        locals.var_thesataclexp_i_rv = 0.0;

        let (assign4680_e4003,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p292,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign4680_e4003;
        locals.var_thesatacw_i_rv = 0.0;

        let assign4690_e4005: f64 = if param_given[415] { 1.0 } else { 0.0 };
        let assign4690_e4007: f64 = if assign4690_e4005 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign4690_e4007;
        locals.var_guard117_rv = 0.0;

        let (assign4700_e4016,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard117 != 0.0)) {
        (p.p415,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign4700_e4016;
        locals.var_thesatacw_i_rv = 0.0;

        let (assign4710_e4023,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p293,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign4710_e4023;
        locals.var_thesataclw_i_rv = 0.0;

        let assign4720_e4025: f64 = if param_given[416] { 1.0 } else { 0.0 };
        let assign4720_e4027: f64 = if assign4720_e4025 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign4720_e4027;
        locals.var_guard118_rv = 0.0;

        let (assign4730_e4036,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard118 != 0.0)) {
        (p.p416,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign4730_e4036;
        locals.var_thesataclw_i_rv = 0.0;

        let (assign4740_e4063, assign4740_e4063_d_n4, assign4740_e4063_d_n6, assign4740_e4063_d_n7, assign4740_e4063_d_n8, assign4740_e4063_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4740_e4046: f64 = (locals.var_ile).powf(locals.var_thesataclexp_i);
        let assign4740_e4047: f64 = (locals.var_thesatacl_i * assign4740_e4046);
        let assign4740_e4048: f64 = (locals.var_thesataco_i + assign4740_e4047);
        let assign4740_e4049: f64 = (locals.var_ge * assign4740_e4048);
        let assign4740_e4053: f64 = (locals.var_thesatacw_i * locals.var_iwe);
        let assign4740_e4054: f64 = (1.0 + assign4740_e4053);
        let assign4740_e4055: f64 = (assign4740_e4049 * assign4740_e4054);
        let assign4740_e4059: f64 = (locals.var_thesataclw_i * locals.var_iae);
        let assign4740_e4060: f64 = (1.0 + assign4740_e4059);
        let assign4740_e4061: f64 = (assign4740_e4055 * assign4740_e4060);
        (assign4740_e4061, (((locals.var_ge_dn4 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((locals.var_ge_dn6 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((locals.var_ge_dn7 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((locals.var_ge_dn8 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((locals.var_ge_dn9 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060),)
    } else {
        (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9,)
    }
};
        locals.var_thesatac_p = assign4740_e4063;
        locals.var_thesatac_p_dn4 = assign4740_e4063_d_n4;
        locals.var_thesatac_p_dn6 = assign4740_e4063_d_n6;
        locals.var_thesatac_p_dn7 = assign4740_e4063_d_n7;
        locals.var_thesatac_p_dn8 = assign4740_e4063_d_n8;
        locals.var_thesatac_p_dn9 = assign4740_e4063_d_n9;
        locals.var_thesatac_p_rv = 0.0;

        let (assign4750_e4072, assign4750_e4072_d_n4, assign4750_e4072_d_n6, assign4750_e4072_d_n7, assign4750_e4072_d_n8, assign4750_e4072_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4750_e4070: f64 = (locals.var_thesatac_p).max(0.0);
        (assign4750_e4070, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn4 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn6 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn7 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn8 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9,)
    }
};
        locals.var_thesatac_t = assign4750_e4072;
        locals.var_thesatac_t_dn4 = assign4750_e4072_d_n4;
        locals.var_thesatac_t_dn6 = assign4750_e4072_d_n6;
        locals.var_thesatac_t_dn7 = assign4750_e4072_d_n7;
        locals.var_thesatac_t_dn8 = assign4750_e4072_d_n8;
        locals.var_thesatac_t_dn9 = assign4750_e4072_d_n9;
        locals.var_thesatac_t_rv = 0.0;

        let (assign4760_e4079,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p300,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign4760_e4079;
        locals.var_axaco_i_rv = 0.0;

        let assign4770_e4081: f64 = if param_given[417] { 1.0 } else { 0.0 };
        let assign4770_e4083: f64 = if assign4770_e4081 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign4770_e4083;
        locals.var_guard119_rv = 0.0;

        let (assign4780_e4092,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard119 != 0.0)) {
        (p.p417,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign4780_e4092;
        locals.var_axaco_i_rv = 0.0;

        let (assign4790_e4099,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p301,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign4790_e4099;
        locals.var_axacl_i_rv = 0.0;

        let assign4800_e4101: f64 = if param_given[418] { 1.0 } else { 0.0 };
        let assign4800_e4103: f64 = if assign4800_e4101 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign4800_e4103;
        locals.var_guard120_rv = 0.0;

        let (assign4810_e4112,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard120 != 0.0)) {
        (p.p418,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign4810_e4112;
        locals.var_axacl_i_rv = 0.0;

        let (assign4820_e4119,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p302,)
    } else {
        (locals.var_axaclexp_i,)
    }
};
        locals.var_axaclexp_i = assign4820_e4119;
        locals.var_axaclexp_i_rv = 0.0;

        let assign4830_e4121: f64 = if param_given[419] { 1.0 } else { 0.0 };
        let assign4830_e4123: f64 = if assign4830_e4121 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard121 = assign4830_e4123;
        locals.var_guard121_rv = 0.0;

        let (assign4840_e4132,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard121 != 0.0)) {
        (p.p419,)
    } else {
        (locals.var_axaclexp_i,)
    }
};
        locals.var_axaclexp_i = assign4840_e4132;
        locals.var_axaclexp_i_rv = 0.0;

        let (assign4850_e4139,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p303,)
    } else {
        (locals.var_axacl2_i,)
    }
};
        locals.var_axacl2_i = assign4850_e4139;
        locals.var_axacl2_i_rv = 0.0;

        let assign4860_e4141: f64 = if param_given[420] { 1.0 } else { 0.0 };
        let assign4860_e4143: f64 = if assign4860_e4141 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard122 = assign4860_e4143;
        locals.var_guard122_rv = 0.0;

        let (assign4870_e4152,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard122 != 0.0)) {
        (p.p420,)
    } else {
        (locals.var_axacl2_i,)
    }
};
        locals.var_axacl2_i = assign4870_e4152;
        locals.var_axacl2_i_rv = 0.0;

        let (assign4880_e4159,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p304,)
    } else {
        (locals.var_axaclexp2_i,)
    }
};
        locals.var_axaclexp2_i = assign4880_e4159;
        locals.var_axaclexp2_i_rv = 0.0;

        let assign4890_e4161: f64 = if param_given[421] { 1.0 } else { 0.0 };
        let assign4890_e4163: f64 = if assign4890_e4161 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard123 = assign4890_e4163;
        locals.var_guard123_rv = 0.0;

        let (assign4900_e4172,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard123 != 0.0)) {
        (p.p421,)
    } else {
        (locals.var_axaclexp2_i,)
    }
};
        locals.var_axaclexp2_i = assign4900_e4172;
        locals.var_axaclexp2_i_rv = 0.0;

        let (assign4910_e4195,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4910_e4182: f64 = (locals.var_ile).powf(locals.var_axaclexp_i);
        let assign4910_e4183: f64 = (locals.var_axacl_i * assign4910_e4182);
        let assign4910_e4188: f64 = (locals.var_ile).powf(locals.var_axaclexp2_i);
        let assign4910_e4189: f64 = (locals.var_axacl2_i * assign4910_e4188);
        let assign4910_e4190: f64 = (1.0 + assign4910_e4189);
        let assign4910_e4191: f64 = (assign4910_e4183 / assign4910_e4190);
        let assign4910_e4192: f64 = (1.0 + assign4910_e4191);
        let assign4910_e4193: f64 = (locals.var_axaco_i / assign4910_e4192);
        (assign4910_e4193,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign4910_e4195;
        locals.var_axac_p_rv = 0.0;

        let (assign4920_e4206,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4920_e4202: f64 = (locals.var_axac_p).max(1.0);
        let assign4920_e4204: f64 = (assign4920_e4202).min(16.0);
        (assign4920_e4204,)
    } else {
        (locals.var_axac_i,)
    }
};
        locals.var_axac_i = assign4920_e4206;
        locals.var_axac_i_rv = 0.0;

        let (assign4930_e4213,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p305,)
    } else {
        (locals.var_alpacl1_i,)
    }
};
        locals.var_alpacl1_i = assign4930_e4213;
        locals.var_alpacl1_i_rv = 0.0;

        let assign4940_e4215: f64 = if param_given[422] { 1.0 } else { 0.0 };
        let assign4940_e4217: f64 = if assign4940_e4215 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign4940_e4217;
        locals.var_guard124_rv = 0.0;

        let (assign4950_e4226,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard124 != 0.0)) {
        (p.p422,)
    } else {
        (locals.var_alpacl1_i,)
    }
};
        locals.var_alpacl1_i = assign4950_e4226;
        locals.var_alpacl1_i_rv = 0.0;

        let (assign4960_e4233,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p306,)
    } else {
        (locals.var_alpaclexp_i,)
    }
};
        locals.var_alpaclexp_i = assign4960_e4233;
        locals.var_alpaclexp_i_rv = 0.0;

        let assign4970_e4235: f64 = if param_given[423] { 1.0 } else { 0.0 };
        let assign4970_e4237: f64 = if assign4970_e4235 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign4970_e4237;
        locals.var_guard125_rv = 0.0;

        let (assign4980_e4246,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard125 != 0.0)) {
        (p.p423,)
    } else {
        (locals.var_alpaclexp_i,)
    }
};
        locals.var_alpaclexp_i = assign4980_e4246;
        locals.var_alpaclexp_i_rv = 0.0;

        let (assign4990_e4253,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p307,)
    } else {
        (locals.var_alpacl2_i,)
    }
};
        locals.var_alpacl2_i = assign4990_e4253;
        locals.var_alpacl2_i_rv = 0.0;

        let assign5000_e4255: f64 = if param_given[424] { 1.0 } else { 0.0 };
        let assign5000_e4257: f64 = if assign5000_e4255 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign5000_e4257;
        locals.var_guard126_rv = 0.0;

        let (assign5010_e4266,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard126 != 0.0)) {
        (p.p424,)
    } else {
        (locals.var_alpacl2_i,)
    }
};
        locals.var_alpacl2_i = assign5010_e4266;
        locals.var_alpacl2_i_rv = 0.0;

        let (assign5020_e4273,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p308,)
    } else {
        (locals.var_alpaclexp2_i,)
    }
};
        locals.var_alpaclexp2_i = assign5020_e4273;
        locals.var_alpaclexp2_i_rv = 0.0;

        let assign5030_e4275: f64 = if param_given[425] { 1.0 } else { 0.0 };
        let assign5030_e4277: f64 = if assign5030_e4275 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard127 = assign5030_e4277;
        locals.var_guard127_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign5040_e4286,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard127 != 0.0)) {
        (p.p425,)
    } else {
        (locals.var_alpaclexp2_i,)
    }
};
        locals.var_alpaclexp2_i = assign5040_e4286;
        locals.var_alpaclexp2_i_rv = 0.0;

        let (assign5050_e4293,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p309,)
    } else {
        (locals.var_alpacw_i,)
    }
};
        locals.var_alpacw_i = assign5050_e4293;
        locals.var_alpacw_i_rv = 0.0;

        let assign5060_e4295: f64 = if param_given[426] { 1.0 } else { 0.0 };
        let assign5060_e4297: f64 = if assign5060_e4295 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard128 = assign5060_e4297;
        locals.var_guard128_rv = 0.0;

        let (assign5070_e4306,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard128 != 0.0)) {
        (p.p426,)
    } else {
        (locals.var_alpacw_i,)
    }
};
        locals.var_alpacw_i = assign5070_e4306;
        locals.var_alpacw_i_rv = 0.0;

        let (assign5080_e4331,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5080_e4314: f64 = (locals.var_ile).powf(locals.var_alpaclexp_i);
        let assign5080_e4315: f64 = (locals.var_alpacl1_i * assign5080_e4314);
        let assign5080_e4319: f64 = (locals.var_alpacw_i * locals.var_iwe);
        let assign5080_e4320: f64 = (1.0 + assign5080_e4319);
        let assign5080_e4321: f64 = (assign5080_e4315 * assign5080_e4320);
        let assign5080_e4326: f64 = (locals.var_ile).powf(locals.var_alpaclexp2_i);
        let assign5080_e4327: f64 = (locals.var_alpacl2_i * assign5080_e4326);
        let assign5080_e4328: f64 = (1.0 + assign5080_e4327);
        let assign5080_e4329: f64 = (assign5080_e4321 / assign5080_e4328);
        (assign5080_e4329,)
    } else {
        (locals.var_alpac_p,)
    }
};
        locals.var_alpac_p = assign5080_e4331;
        locals.var_alpac_p_rv = 0.0;

        let (assign5090_e4340,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5090_e4338: f64 = (locals.var_alpac_p).max(0.0);
        (assign5090_e4338,)
    } else {
        (locals.var_alpac_i,)
    }
};
        locals.var_alpac_i = assign5090_e4340;
        locals.var_alpac_i_rv = 0.0;

        let (assign5100_e4349, assign5100_e4349_d_n4, assign5100_e4349_d_n6, assign5100_e4349_d_n7, assign5100_e4349_d_n8, assign5100_e4349_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5100_e4345: f64 = (3.45313e-11 / locals.var_tox1_i);
        let assign5100_e4347: f64 = (assign5100_e4345 * locals.var_wecv);
        (assign5100_e4347, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign5100_e4349;
        locals.var_temp_dn4 = assign5100_e4349_d_n4;
        locals.var_temp_dn6 = assign5100_e4349_d_n6;
        locals.var_temp_dn7 = assign5100_e4349_d_n7;
        locals.var_temp_dn8 = assign5100_e4349_d_n8;
        locals.var_temp_dn9 = assign5100_e4349_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign5110_e4356, assign5110_e4356_d_n4, assign5110_e4356_d_n6, assign5110_e4356_d_n7, assign5110_e4356_d_n8, assign5110_e4356_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5110_e4354: f64 = (locals.var_temp * p.p427);
        (assign5110_e4354, (locals.var_temp_dn4 * p.p427), (locals.var_temp_dn6 * p.p427), (locals.var_temp_dn7 * p.p427), (locals.var_temp_dn8 * p.p427), (locals.var_temp_dn9 * p.p427),)
    } else {
        (locals.var_cov_i, locals.var_cov_i_dn4, locals.var_cov_i_dn6, locals.var_cov_i_dn7, locals.var_cov_i_dn8, locals.var_cov_i_dn9,)
    }
};
        locals.var_cov_i = assign5110_e4356;
        locals.var_cov_i_dn4 = assign5110_e4356_d_n4;
        locals.var_cov_i_dn6 = assign5110_e4356_d_n6;
        locals.var_cov_i_dn7 = assign5110_e4356_d_n7;
        locals.var_cov_i_dn8 = assign5110_e4356_d_n8;
        locals.var_cov_i_dn9 = assign5110_e4356_d_n9;
        locals.var_cov_i_rv = 0.0;

        let (assign5120_e4363, assign5120_e4363_d_n4, assign5120_e4363_d_n6, assign5120_e4363_d_n7, assign5120_e4363_d_n8, assign5120_e4363_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5120_e4361: f64 = (locals.var_temp * p.p428);
        (assign5120_e4361, (locals.var_temp_dn4 * p.p428), (locals.var_temp_dn6 * p.p428), (locals.var_temp_dn7 * p.p428), (locals.var_temp_dn8 * p.p428), (locals.var_temp_dn9 * p.p428),)
    } else {
        (locals.var_covd_i, locals.var_covd_i_dn4, locals.var_covd_i_dn6, locals.var_covd_i_dn7, locals.var_covd_i_dn8, locals.var_covd_i_dn9,)
    }
};
        locals.var_covd_i = assign5120_e4363;
        locals.var_covd_i_dn4 = assign5120_e4363_d_n4;
        locals.var_covd_i_dn6 = assign5120_e4363_d_n6;
        locals.var_covd_i_dn7 = assign5120_e4363_d_n7;
        locals.var_covd_i_dn8 = assign5120_e4363_d_n8;
        locals.var_covd_i_dn9 = assign5120_e4363_d_n9;
        locals.var_covd_i_rv = 0.0;

        let (assign5130_e4378,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5130_e4370: f64 = (p.p430 * locals.var_wen);
        let assign5130_e4372: f64 = (assign5130_e4370 / locals.var_wecv);
        let assign5130_e4373: f64 = (1.0 + assign5130_e4372);
        let assign5130_e4375: f64 = (assign5130_e4373).max(0.001);
        let assign5130_e4376: f64 = (p.p429 / assign5130_e4375);
        (assign5130_e4376,)
    } else {
        (locals.var_covdl_i,)
    }
};
        locals.var_covdl_i = assign5130_e4378;
        locals.var_covdl_i_rv = 0.0;

        let (assign5140_e4383,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p431,)
    } else {
        (locals.var_covdlb_i,)
    }
};
        locals.var_covdlb_i = assign5140_e4383;
        locals.var_covdlb_i_rv = 0.0;

        let (assign5150_e4388,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p432,)
    } else {
        (locals.var_dvfbov_i,)
    }
};
        locals.var_dvfbov_i = assign5150_e4388;
        locals.var_dvfbov_i_rv = 0.0;

        let (assign5160_e4397, assign5160_e4397_d_n4, assign5160_e4397_d_n6, assign5160_e4397_d_n7, assign5160_e4397_d_n8, assign5160_e4397_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5160_e4394: f64 = (p.p435 * locals.var_wphy);
        let assign5160_e4395: f64 = (p.p433 + assign5160_e4394);
        (assign5160_e4395, (p.p435 * locals.var_wphy_dn4), (p.p435 * locals.var_wphy_dn6), (p.p435 * locals.var_wphy_dn7), (p.p435 * locals.var_wphy_dn8), (p.p435 * locals.var_wphy_dn9),)
    } else {
        (locals.var_cfr_p, locals.var_cfr_p_dn4, locals.var_cfr_p_dn6, locals.var_cfr_p_dn7, locals.var_cfr_p_dn8, locals.var_cfr_p_dn9,)
    }
};
        locals.var_cfr_p = assign5160_e4397;
        locals.var_cfr_p_dn4 = assign5160_e4397_d_n4;
        locals.var_cfr_p_dn6 = assign5160_e4397_d_n6;
        locals.var_cfr_p_dn7 = assign5160_e4397_d_n7;
        locals.var_cfr_p_dn8 = assign5160_e4397_d_n8;
        locals.var_cfr_p_dn9 = assign5160_e4397_d_n9;
        locals.var_cfr_p_rv = 0.0;

        let (assign5170_e4404, assign5170_e4404_d_n4, assign5170_e4404_d_n6, assign5170_e4404_d_n7, assign5170_e4404_d_n8, assign5170_e4404_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5170_e4402: f64 = (locals.var_cfr_p).max(0.0);
        (assign5170_e4402, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn4 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn6 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn7 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn8 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cfr_i, locals.var_cfr_i_dn4, locals.var_cfr_i_dn6, locals.var_cfr_i_dn7, locals.var_cfr_i_dn8, locals.var_cfr_i_dn9,)
    }
};
        locals.var_cfr_i = assign5170_e4404;
        locals.var_cfr_i_dn4 = assign5170_e4404_d_n4;
        locals.var_cfr_i_dn6 = assign5170_e4404_d_n6;
        locals.var_cfr_i_dn7 = assign5170_e4404_d_n7;
        locals.var_cfr_i_dn8 = assign5170_e4404_d_n8;
        locals.var_cfr_i_dn9 = assign5170_e4404_d_n9;
        locals.var_cfr_i_rv = 0.0;

        let (assign5180_e4413, assign5180_e4413_d_n4, assign5180_e4413_d_n6, assign5180_e4413_d_n7, assign5180_e4413_d_n8, assign5180_e4413_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5180_e4410: f64 = (p.p436 * locals.var_wphy);
        let assign5180_e4411: f64 = (p.p434 + assign5180_e4410);
        (assign5180_e4411, (p.p436 * locals.var_wphy_dn4), (p.p436 * locals.var_wphy_dn6), (p.p436 * locals.var_wphy_dn7), (p.p436 * locals.var_wphy_dn8), (p.p436 * locals.var_wphy_dn9),)
    } else {
        (locals.var_cfrd_p, locals.var_cfrd_p_dn4, locals.var_cfrd_p_dn6, locals.var_cfrd_p_dn7, locals.var_cfrd_p_dn8, locals.var_cfrd_p_dn9,)
    }
};
        locals.var_cfrd_p = assign5180_e4413;
        locals.var_cfrd_p_dn4 = assign5180_e4413_d_n4;
        locals.var_cfrd_p_dn6 = assign5180_e4413_d_n6;
        locals.var_cfrd_p_dn7 = assign5180_e4413_d_n7;
        locals.var_cfrd_p_dn8 = assign5180_e4413_d_n8;
        locals.var_cfrd_p_dn9 = assign5180_e4413_d_n9;
        locals.var_cfrd_p_rv = 0.0;

        let (assign5190_e4420, assign5190_e4420_d_n4, assign5190_e4420_d_n6, assign5190_e4420_d_n7, assign5190_e4420_d_n8, assign5190_e4420_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5190_e4418: f64 = (locals.var_cfrd_p).max(0.0);
        (assign5190_e4418, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn4 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn6 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn7 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn8 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cfrd_i, locals.var_cfrd_i_dn4, locals.var_cfrd_i_dn6, locals.var_cfrd_i_dn7, locals.var_cfrd_i_dn8, locals.var_cfrd_i_dn9,)
    }
};
        locals.var_cfrd_i = assign5190_e4420;
        locals.var_cfrd_i_dn4 = assign5190_e4420_d_n4;
        locals.var_cfrd_i_dn6 = assign5190_e4420_d_n6;
        locals.var_cfrd_i_dn7 = assign5190_e4420_d_n7;
        locals.var_cfrd_i_dn8 = assign5190_e4420_d_n8;
        locals.var_cfrd_i_dn9 = assign5190_e4420_d_n9;
        locals.var_cfrd_i_rv = 0.0;

        let (assign5200_e4433,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5200_e4425: f64 = (p.p437 * locals.var_epsch);
        let assign5200_e4427: f64 = (assign5200_e4425 * locals.var_tsi_i);
        let assign5200_e4429: f64 = (assign5200_e4427 * locals.var_we);
        let assign5200_e4431: f64 = (assign5200_e4429 / locals.var_le);
        (assign5200_e4431,)
    } else {
        (locals.var_csd_i,)
    }
};
        locals.var_csd_i = assign5200_e4433;
        locals.var_csd_i_rv = 0.0;

        let (assign5210_e4438,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p438,)
    } else {
        (locals.var_csdbp_i,)
    }
};
        locals.var_csdbp_i = assign5210_e4438;
        locals.var_csdbp_i_rv = 0.0;

        let (assign5220_e4459, assign5220_e4459_d_n4, assign5220_e4459_d_n6, assign5220_e4459_d_n7, assign5220_e4459_d_n8, assign5220_e4459_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5220_e4444: f64 = (p.p440 * locals.var_lphy);
        let assign5220_e4445: f64 = (1.0 + assign5220_e4444);
        let assign5220_e4448: f64 = (p.p441 * locals.var_wphy);
        let assign5220_e4449: f64 = (assign5220_e4445 + assign5220_e4448);
        let assign5220_e4452: f64 = (p.p442 * locals.var_lphy);
        let assign5220_e4454: f64 = (assign5220_e4452 * locals.var_wphy);
        let assign5220_e4455: f64 = (assign5220_e4449 + assign5220_e4454);
        let assign5220_e4457: f64 = (assign5220_e4455).max(1e-10);
        (assign5220_e4457, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn4) + (p.p441 * locals.var_wphy_dn4)) + (((p.p442 * locals.var_lphy_dn4) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn4))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn6) + (p.p441 * locals.var_wphy_dn6)) + (((p.p442 * locals.var_lphy_dn6) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn6))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn7) + (p.p441 * locals.var_wphy_dn7)) + (((p.p442 * locals.var_lphy_dn7) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn7))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn8) + (p.p441 * locals.var_wphy_dn8)) + (((p.p442 * locals.var_lphy_dn8) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn8))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn9) + (p.p441 * locals.var_wphy_dn9)) + (((p.p442 * locals.var_lphy_dn9) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn9))) } else { 0.0 },)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign5220_e4459;
        locals.var_temp_dn4 = assign5220_e4459_d_n4;
        locals.var_temp_dn6 = assign5220_e4459_d_n6;
        locals.var_temp_dn7 = assign5220_e4459_d_n7;
        locals.var_temp_dn8 = assign5220_e4459_d_n8;
        locals.var_temp_dn9 = assign5220_e4459_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign5230_e4464, assign5230_e4464_d_n4, assign5230_e4464_d_n6, assign5230_e4464_d_n7, assign5230_e4464_d_n8, assign5230_e4464_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign5230_e4464;
        locals.var_temp1_dn4 = assign5230_e4464_d_n4;
        locals.var_temp1_dn6 = assign5230_e4464_d_n6;
        locals.var_temp1_dn7 = assign5230_e4464_d_n7;
        locals.var_temp1_dn8 = assign5230_e4464_d_n8;
        locals.var_temp1_dn9 = assign5230_e4464_d_n9;
        locals.var_temp1_rv = 0.0;

        let assign5240_e4471: f64 = if ((p.p29 > 1.0) && (p.p28 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard129 = assign5240_e4471;
        locals.var_guard129_rv = 0.0;

        let (assign5250_e4483, assign5250_e4483_d_n4, assign5250_e4483_d_n6, assign5250_e4483_d_n7, assign5250_e4483_d_n8, assign5250_e4483_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) {
        let assign5250_e4478: f64 = (p.p28 + p.p20);
        let assign5250_e4479: f64 = (-assign5250_e4478);
        let assign5250_e4481: f64 = (assign5250_e4479 / p.p445);
        (assign5250_e4481, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign5250_e4483;
        locals.var_temp2_dn4 = assign5250_e4483_d_n4;
        locals.var_temp2_dn6 = assign5250_e4483_d_n6;
        locals.var_temp2_dn7 = assign5250_e4483_d_n7;
        locals.var_temp2_dn8 = assign5250_e4483_d_n8;
        locals.var_temp2_dn9 = assign5250_e4483_d_n9;
        locals.var_temp2_rv = 0.0;

        let assign5260_e4485: f64 = (locals.var_temp2).abs();
        let assign5260_e4487: f64 = if assign5260_e4485 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard130 = assign5260_e4487;
        locals.var_guard130_rv = 0.0;

        let (assign5270_e4497, assign5270_e4497_d_n4, assign5270_e4497_d_n6, assign5270_e4497_d_n7, assign5270_e4497_d_n8, assign5270_e4497_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) && (locals.var_guard130 != 0.0)) {
        let assign5270_e4495: f64 = (locals.var_temp2).exp();
        (assign5270_e4495, (assign5270_e4495 * locals.var_temp2_dn4), (assign5270_e4495 * locals.var_temp2_dn6), (assign5270_e4495 * locals.var_temp2_dn7), (assign5270_e4495 * locals.var_temp2_dn8), (assign5270_e4495 * locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign5270_e4497;
        locals.var_temp3_dn4 = assign5270_e4497_d_n4;
        locals.var_temp3_dn6 = assign5270_e4497_d_n6;
        locals.var_temp3_dn7 = assign5270_e4497_d_n7;
        locals.var_temp3_dn8 = assign5270_e4497_d_n8;
        locals.var_temp3_dn9 = assign5270_e4497_d_n9;
        locals.var_temp3_rv = 0.0;

        let assign5280_e4500: f64 = (-80.0);
        let assign5280_e4501: f64 = if locals.var_temp2 < assign5280_e4500 { 1.0 } else { 0.0 };
        locals.var_guard131 = assign5280_e4501;
        locals.var_guard131_rv = 0.0;

        let (assign5290_e4538, assign5290_e4538_d_n4, assign5290_e4538_d_n6, assign5290_e4538_d_n7, assign5290_e4538_d_n8, assign5290_e4538_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) && (locals.var_guard130 == 0.0)) && (locals.var_guard131 != 0.0)) {
        let assign5290_e4514: f64 = (-locals.var_temp2);
        let assign5290_e4516: f64 = (assign5290_e4514 - 80.0);
        let assign5290_e4520: f64 = (-locals.var_temp2);
        let assign5290_e4522: f64 = (assign5290_e4520 - 80.0);
        let assign5290_e4523: f64 = (0.5 * assign5290_e4522);
        let assign5290_e4526: f64 = (-locals.var_temp2);
        let assign5290_e4528: f64 = (assign5290_e4526 - 80.0);
        let assign5290_e4530: f64 = (assign5290_e4528 * 0.3333333333333);
        let assign5290_e4531: f64 = (1.0 + assign5290_e4530);
        let assign5290_e4532: f64 = (assign5290_e4523 * assign5290_e4531);
        let assign5290_e4533: f64 = (1.0 + assign5290_e4532);
        let assign5290_e4534: f64 = (assign5290_e4516 * assign5290_e4533);
        let assign5290_e4535: f64 = (1.0 + assign5290_e4534);
        let assign5290_e4536: f64 = (1.80485e-35 / assign5290_e4535);
        (assign5290_e4536, (-((1.80485e-35 * (((-locals.var_temp2_dn4) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn4)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn4) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-locals.var_temp2_dn6) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn6)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn6) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-locals.var_temp2_dn7) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn7)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn7) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-locals.var_temp2_dn8) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn8)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn8) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-locals.var_temp2_dn9) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn9)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn9) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign5290_e4538;
        locals.var_temp3_dn4 = assign5290_e4538_d_n4;
        locals.var_temp3_dn6 = assign5290_e4538_d_n6;
        locals.var_temp3_dn7 = assign5290_e4538_d_n7;
        locals.var_temp3_dn8 = assign5290_e4538_d_n8;
        locals.var_temp3_dn9 = assign5290_e4538_d_n9;
        locals.var_temp3_rv = 0.0;

        let (assign5300_e4573, assign5300_e4573_d_n4, assign5300_e4573_d_n6, assign5300_e4573_d_n7, assign5300_e4573_d_n8, assign5300_e4573_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) && (locals.var_guard130 == 0.0)) && (locals.var_guard131 == 0.0)) {
        let assign5300_e4553: f64 = (locals.var_temp2 - 80.0);
        let assign5300_e4558: f64 = (locals.var_temp2 - 80.0);
        let assign5300_e4559: f64 = (0.5 * assign5300_e4558);
        let assign5300_e4563: f64 = (locals.var_temp2 - 80.0);
        let assign5300_e4565: f64 = (assign5300_e4563 * 0.3333333333333);
        let assign5300_e4566: f64 = (1.0 + assign5300_e4565);
        let assign5300_e4567: f64 = (assign5300_e4559 * assign5300_e4566);
        let assign5300_e4568: f64 = (1.0 + assign5300_e4567);
        let assign5300_e4569: f64 = (assign5300_e4553 * assign5300_e4568);
        let assign5300_e4570: f64 = (1.0 + assign5300_e4569);
        let assign5300_e4571: f64 = (5.54062e34 * assign5300_e4570);
        (assign5300_e4571, (5.54062e34 * ((locals.var_temp2_dn4 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn4) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn6 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn6) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn7 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn7) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn8 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn8) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn9 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn9) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign5300_e4573;
        locals.var_temp3_dn4 = assign5300_e4573_d_n4;
        locals.var_temp3_dn6 = assign5300_e4573_d_n6;
        locals.var_temp3_dn7 = assign5300_e4573_d_n7;
        locals.var_temp3_dn8 = assign5300_e4573_d_n8;
        locals.var_temp3_dn9 = assign5300_e4573_d_n9;
        locals.var_temp3_rv = 0.0;

        let (assign5310_e4582, assign5310_e4582_d_n4, assign5310_e4582_d_n6, assign5310_e4582_d_n7, assign5310_e4582_d_n8, assign5310_e4582_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) {
        let assign5310_e4580: f64 = (1.0 - locals.var_temp3);
        (assign5310_e4580, (-locals.var_temp3_dn4), (-locals.var_temp3_dn6), (-locals.var_temp3_dn7), (-locals.var_temp3_dn8), (-locals.var_temp3_dn9),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign5310_e4582;
        locals.var_temp4_dn4 = assign5310_e4582_d_n4;
        locals.var_temp4_dn6 = assign5310_e4582_d_n6;
        locals.var_temp4_dn7 = assign5310_e4582_d_n7;
        locals.var_temp4_dn8 = assign5310_e4582_d_n8;
        locals.var_temp4_dn9 = assign5310_e4582_d_n9;
        locals.var_temp4_rv = 0.0;

        let (assign5320_e4607, assign5320_e4607_d_n4, assign5320_e4607_d_n6, assign5320_e4607_d_n7, assign5320_e4607_d_n8, assign5320_e4607_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) {
        let assign5320_e4589: f64 = (2.0 * p.p446);
        let assign5320_e4591: f64 = (assign5320_e4589 * locals.var_temp3);
        let assign5320_e4596: f64 = (locals.var_temp3).powf(p.p29);
        let assign5320_e4597: f64 = (1.0 - assign5320_e4596);
        let assign5320_e4599: f64 = (assign5320_e4597 / p.p29);
        let assign5320_e4600: f64 = (locals.var_temp4 - assign5320_e4599);
        let assign5320_e4601: f64 = (assign5320_e4591 * assign5320_e4600);
        let assign5320_e4604: f64 = (locals.var_temp4 * locals.var_temp4);
        let assign5320_e4605: f64 = (assign5320_e4601 / assign5320_e4604);
        (assign5320_e4605, ((((((assign5320_e4589 * locals.var_temp3_dn4) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn4 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn4)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn4 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn4 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn4)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * locals.var_temp3_dn6) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn6 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn6)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn6 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn6 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn6)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * locals.var_temp3_dn7) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn7 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn7)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn7 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn7 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn7)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * locals.var_temp3_dn8) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn8 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn8)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn8 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn8 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn8)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * locals.var_temp3_dn9) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn9 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn9)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn9 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn9 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn9)))) / (assign5320_e4604 * assign5320_e4604)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign5320_e4607;
        locals.var_temp1_dn4 = assign5320_e4607_d_n4;
        locals.var_temp1_dn6 = assign5320_e4607_d_n6;
        locals.var_temp1_dn7 = assign5320_e4607_d_n7;
        locals.var_temp1_dn8 = assign5320_e4607_d_n8;
        locals.var_temp1_dn9 = assign5320_e4607_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign5330_e4616, assign5330_e4616_d_n4, assign5330_e4616_d_n6, assign5330_e4616_d_n7, assign5330_e4616_d_n8, assign5330_e4616_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5330_e4613: f64 = (1.0 + locals.var_temp1);
        let assign5330_e4614: f64 = (locals.var_temp / assign5330_e4613);
        (assign5330_e4614, (((locals.var_temp_dn4 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn4)) / (assign5330_e4613 * assign5330_e4613)), (((locals.var_temp_dn6 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn6)) / (assign5330_e4613 * assign5330_e4613)), (((locals.var_temp_dn7 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn7)) / (assign5330_e4613 * assign5330_e4613)), (((locals.var_temp_dn8 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn8)) / (assign5330_e4613 * assign5330_e4613)), (((locals.var_temp_dn9 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn9)) / (assign5330_e4613 * assign5330_e4613)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign5330_e4616;
        locals.var_temp_dn4 = assign5330_e4616_d_n4;
        locals.var_temp_dn6 = assign5330_e4616_d_n6;
        locals.var_temp_dn7 = assign5330_e4616_d_n7;
        locals.var_temp_dn8 = assign5330_e4616_d_n8;
        locals.var_temp_dn9 = assign5330_e4616_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign5340_e4623, assign5340_e4623_d_n4, assign5340_e4623_d_n6, assign5340_e4623_d_n7, assign5340_e4623_d_n8, assign5340_e4623_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5340_e4621: f64 = (p.p439 / locals.var_temp);
        (assign5340_e4621, (-((p.p439 * locals.var_temp_dn4) / (locals.var_temp * locals.var_temp))), (-((p.p439 * locals.var_temp_dn6) / (locals.var_temp * locals.var_temp))), (-((p.p439 * locals.var_temp_dn7) / (locals.var_temp * locals.var_temp))), (-((p.p439 * locals.var_temp_dn8) / (locals.var_temp * locals.var_temp))), (-((p.p439 * locals.var_temp_dn9) / (locals.var_temp * locals.var_temp))),)
    } else {
        (locals.var_rth_p, locals.var_rth_p_dn4, locals.var_rth_p_dn6, locals.var_rth_p_dn7, locals.var_rth_p_dn8, locals.var_rth_p_dn9,)
    }
};
        locals.var_rth_p = assign5340_e4623;
        locals.var_rth_p_dn4 = assign5340_e4623_d_n4;
        locals.var_rth_p_dn6 = assign5340_e4623_d_n6;
        locals.var_rth_p_dn7 = assign5340_e4623_d_n7;
        locals.var_rth_p_dn8 = assign5340_e4623_d_n8;
        locals.var_rth_p_dn9 = assign5340_e4623_d_n9;
        locals.var_rth_p_rv = 0.0;

        let (assign5350_e4630, assign5350_e4630_d_n4, assign5350_e4630_d_n6, assign5350_e4630_d_n7, assign5350_e4630_d_n8, assign5350_e4630_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5350_e4628: f64 = (locals.var_rth_p).max(1e-6);
        (assign5350_e4628, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn4 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn6 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn7 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn8 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_rth_t, locals.var_rth_t_dn4, locals.var_rth_t_dn6, locals.var_rth_t_dn7, locals.var_rth_t_dn8, locals.var_rth_t_dn9,)
    }
};
        locals.var_rth_t = assign5350_e4630;
        locals.var_rth_t_dn4 = assign5350_e4630_d_n4;
        locals.var_rth_t_dn6 = assign5350_e4630_d_n6;
        locals.var_rth_t_dn7 = assign5350_e4630_d_n7;
        locals.var_rth_t_dn8 = assign5350_e4630_d_n8;
        locals.var_rth_t_dn9 = assign5350_e4630_d_n9;
        locals.var_rth_t_rv = 0.0;

        let (assign5360_e4635,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p443,)
    } else {
        (locals.var_strth_i,)
    }
};
        locals.var_strth_i = assign5360_e4635;
        locals.var_strth_i_rv = 0.0;

        let (assign5410_e4684,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5410_e4678: f64 = (p.p450 * locals.var_iae);
        let assign5410_e4681: f64 = (p.p451 * locals.var_iwe);
        let assign5410_e4682: f64 = (assign5410_e4678 + assign5410_e4681);
        (assign5410_e4682,)
    } else {
        (locals.var_nfa_p,)
    }
};
        locals.var_nfa_p = assign5410_e4684;
        locals.var_nfa_p_rv = 0.0;

        let (assign5420_e4691,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5420_e4689: f64 = (locals.var_nfa_p).max(0.0);
        (assign5420_e4689,)
    } else {
        (locals.var_nfa_i,)
    }
};
        locals.var_nfa_i = assign5420_e4691;
        locals.var_nfa_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5430_e4698,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5430_e4696: f64 = (p.p452 * locals.var_iae);
        (assign5430_e4696,)
    } else {
        (locals.var_nfb_i,)
    }
};
        locals.var_nfb_i = assign5430_e4698;
        locals.var_nfb_i_rv = 0.0;

        let (assign5440_e4705,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5440_e4703: f64 = (p.p453 * locals.var_iae);
        (assign5440_e4703,)
    } else {
        (locals.var_nfc_i,)
    }
};
        locals.var_nfc_i = assign5440_e4705;
        locals.var_nfc_i_rv = 0.0;

        let (assign5450_e4710,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p454,)
    } else {
        (locals.var_nfe_i,)
    }
};
        locals.var_nfe_i = assign5450_e4710;
        locals.var_nfe_i_rv = 0.0;

        let (assign5460_e4715,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p455,)
    } else {
        (locals.var_nfeb_i,)
    }
};
        locals.var_nfeb_i = assign5460_e4715;
        locals.var_nfeb_i_rv = 0.0;

        let assign5570_e4828: f64 = if ((((p.p457 > 0.0) && (p.p26 > 0.0)) && (p.p27 > 0.0)) && ((p.p29 == 1.0) || ((p.p29 > 1.0) && (p.p28 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign5570_e4828;
        locals.var_guard133_rv = 0.0;

        let assign5580_e4831: f64 = if p.p457 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign5580_e4831;
        locals.var_guard134_rv = 0.0;

        let (assign5590_e4840, assign5590_e4840_d_n4, assign5590_e4840_d_n6, assign5590_e4840_d_n7, assign5590_e4840_d_n8, assign5590_e4840_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9,)
    }
};
        locals.var_tmpa = assign5590_e4840;
        locals.var_tmpa_dn4 = assign5590_e4840_d_n4;
        locals.var_tmpa_dn6 = assign5590_e4840_d_n6;
        locals.var_tmpa_dn7 = assign5590_e4840_d_n7;
        locals.var_tmpa_dn8 = assign5590_e4840_d_n8;
        locals.var_tmpa_dn9 = assign5590_e4840_d_n9;
        locals.var_tmpa_rv = 0.0;

        let (assign5600_e4849,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_tmpb,)
    }
};
        locals.var_tmpb = assign5600_e4849;
        locals.var_tmpb_rv = 0.0;

        let (assign5610_e4858,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_iloop,)
    }
};
        locals.var_iloop = assign5610_e4858;
        locals.var_iloop_rv = 0.0;

        let mut assign5620_loop_guard: usize = 0;
        while {
            let assign5620_cond_e4868: f64 = (p.p29 - 0.5);
            let assign5620_cond_e4870: f64 = if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) && (locals.var_iloop < assign5620_cond_e4868)) { 1.0 } else { 0.0 };
            assign5620_cond_e4870 != 0.0
        } {
            assign5620_loop_guard += 1;
            assert!(assign5620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign5620_body0_e4893, assign5620_body0_e4893_d_n4, assign5620_body0_e4893_d_n6, assign5620_body0_e4893_d_n7, assign5620_body0_e4893_d_n8, assign5620_body0_e4893_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5620_body0_e4882: f64 = (0.5 * p.p20);
        let assign5620_body0_e4883: f64 = (p.p26 + assign5620_body0_e4882);
        let assign5620_body0_e4887: f64 = (p.p28 + p.p20);
        let assign5620_body0_e4888: f64 = (locals.var_iloop * assign5620_body0_e4887);
        let assign5620_body0_e4889: f64 = (assign5620_body0_e4883 + assign5620_body0_e4888);
        let assign5620_body0_e4890: f64 = (1.0 / assign5620_body0_e4889);
        let assign5620_body0_e4891: f64 = (locals.var_tmpa + assign5620_body0_e4890);
        (assign5620_body0_e4891, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9,)
    } else {
        (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9,)
    }
};
            locals.var_tmpa = assign5620_body0_e4893;
            locals.var_tmpa_dn4 = assign5620_body0_e4893_d_n4;
            locals.var_tmpa_dn6 = assign5620_body0_e4893_d_n6;
            locals.var_tmpa_dn7 = assign5620_body0_e4893_d_n7;
            locals.var_tmpa_dn8 = assign5620_body0_e4893_d_n8;
            locals.var_tmpa_dn9 = assign5620_body0_e4893_d_n9;
            locals.var_tmpa_rv = 0.0;
            let (assign5620_body1_e4916,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5620_body1_e4905: f64 = (0.5 * p.p20);
        let assign5620_body1_e4906: f64 = (p.p27 + assign5620_body1_e4905);
        let assign5620_body1_e4910: f64 = (p.p28 + p.p20);
        let assign5620_body1_e4911: f64 = (locals.var_iloop * assign5620_body1_e4910);
        let assign5620_body1_e4912: f64 = (assign5620_body1_e4906 + assign5620_body1_e4911);
        let assign5620_body1_e4913: f64 = (1.0 / assign5620_body1_e4912);
        let assign5620_body1_e4914: f64 = (locals.var_tmpb + assign5620_body1_e4913);
        (assign5620_body1_e4914,)
    } else {
        (locals.var_tmpb,)
    }
};
            locals.var_tmpb = assign5620_body1_e4916;
            locals.var_tmpb_rv = 0.0;
            let (assign5620_body2_e4927,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5620_body2_e4925: f64 = (locals.var_iloop + 1.0);
        (assign5620_body2_e4925,)
    } else {
        (locals.var_iloop,)
    }
};
            locals.var_iloop = assign5620_body2_e4927;
            locals.var_iloop_rv = 0.0;
        }

        let (assign5630_e4938, assign5630_e4938_d_n4, assign5630_e4938_d_n6, assign5630_e4938_d_n7, assign5630_e4938_d_n8, assign5630_e4938_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5630_e4936: f64 = (locals.var_tmpa / p.p29);
        (assign5630_e4936, (locals.var_tmpa_dn4 / p.p29), (locals.var_tmpa_dn6 / p.p29), (locals.var_tmpa_dn7 / p.p29), (locals.var_tmpa_dn8 / p.p29), (locals.var_tmpa_dn9 / p.p29),)
    } else {
        (locals.var_invsa, locals.var_invsa_dn4, locals.var_invsa_dn6, locals.var_invsa_dn7, locals.var_invsa_dn8, locals.var_invsa_dn9,)
    }
};
        locals.var_invsa = assign5630_e4938;
        locals.var_invsa_dn4 = assign5630_e4938_d_n4;
        locals.var_invsa_dn6 = assign5630_e4938_d_n6;
        locals.var_invsa_dn7 = assign5630_e4938_d_n7;
        locals.var_invsa_dn8 = assign5630_e4938_d_n8;
        locals.var_invsa_dn9 = assign5630_e4938_d_n9;
        locals.var_invsa_rv = 0.0;

        let (assign5640_e4949,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5640_e4947: f64 = (locals.var_tmpb / p.p29);
        (assign5640_e4947,)
    } else {
        (locals.var_invsb,)
    }
};
        locals.var_invsb = assign5640_e4949;
        locals.var_invsb_rv = 0.0;

        let (assign5650_e4964,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5650_e4960: f64 = (0.5 * p.p20);
        let assign5650_e4961: f64 = (p.p458 + assign5650_e4960);
        let assign5650_e4962: f64 = (1.0 / assign5650_e4961);
        (assign5650_e4962,)
    } else {
        (locals.var_invsaref,)
    }
};
        locals.var_invsaref = assign5650_e4964;
        locals.var_invsaref_rv = 0.0;

        let (assign5660_e4979,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5660_e4975: f64 = (0.5 * p.p20);
        let assign5660_e4976: f64 = (p.p459 + assign5660_e4975);
        let assign5660_e4977: f64 = (1.0 / assign5660_e4976);
        (assign5660_e4977,)
    } else {
        (locals.var_invsbref,)
    }
};
        locals.var_invsbref = assign5660_e4979;
        locals.var_invsbref_rv = 0.0;

        let (assign5670_e4992,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5670_e4988: f64 = (p.p20 + locals.var_dellps);
        let assign5670_e4990: f64 = (assign5670_e4988).max(1e-9);
        (assign5670_e4990,)
    } else {
        (locals.var_lx,)
    }
};
        locals.var_lx = assign5670_e4992;
        locals.var_lx_rv = 0.0;

        let (assign5680_e5007,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5680_e5001: f64 = (locals.var_w_i + locals.var_delwod);
        let assign5680_e5003: f64 = (assign5680_e5001 + p.p460);
        let assign5680_e5005: f64 = (assign5680_e5003).max(1e-9);
        (assign5680_e5005,)
    } else {
        (locals.var_wx,)
    }
};
        locals.var_wx = assign5680_e5007;
        locals.var_wx_rv = 0.0;

        let (assign5690_e5020,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5690_e5017: f64 = (locals.var_lx).powf(p.p467);
        let assign5690_e5018: f64 = (1.0 / assign5690_e5017);
        (assign5690_e5018,)
    } else {
        (locals.var_templ,)
    }
};
        locals.var_templ = assign5690_e5020;
        locals.var_templ_rv = 0.0;

        let (assign5700_e5033,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5700_e5030: f64 = (locals.var_wx).powf(p.p468);
        let assign5700_e5031: f64 = (1.0 / assign5700_e5030);
        (assign5700_e5031,)
    } else {
        (locals.var_tempw,)
    }
};
        locals.var_tempw = assign5700_e5033;
        locals.var_tempw_rv = 0.0;

        let (assign5710_e5064, assign5710_e5064_d_n4, assign5710_e5064_d_n6, assign5710_e5064_d_n7, assign5710_e5064_d_n8, assign5710_e5064_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5710_e5043: f64 = (p.p464 * locals.var_templ);
        let assign5710_e5044: f64 = (1.0 + assign5710_e5043);
        let assign5710_e5047: f64 = (p.p465 * locals.var_tempw);
        let assign5710_e5048: f64 = (assign5710_e5044 + assign5710_e5047);
        let assign5710_e5051: f64 = (p.p466 * locals.var_templ);
        let assign5710_e5053: f64 = (assign5710_e5051 * locals.var_tempw);
        let assign5710_e5054: f64 = (assign5710_e5048 + assign5710_e5053);
        let assign5710_e5059: f64 = (locals.var_rt - 1.0);
        let assign5710_e5060: f64 = (p.p463 * assign5710_e5059);
        let assign5710_e5061: f64 = (1.0 + assign5710_e5060);
        let assign5710_e5062: f64 = (assign5710_e5054 * assign5710_e5061);
        (assign5710_e5062, (assign5710_e5054 * (p.p463 * locals.var_rt_dn4)), (assign5710_e5054 * (p.p463 * locals.var_rt_dn6)), (assign5710_e5054 * (p.p463 * locals.var_rt_dn7)), (assign5710_e5054 * (p.p463 * locals.var_rt_dn8)), (assign5710_e5054 * (p.p463 * locals.var_rt_dn9)),)
    } else {
        (locals.var_kstressu0, locals.var_kstressu0_dn4, locals.var_kstressu0_dn6, locals.var_kstressu0_dn7, locals.var_kstressu0_dn8, locals.var_kstressu0_dn9,)
    }
};
        locals.var_kstressu0 = assign5710_e5064;
        locals.var_kstressu0_dn4 = assign5710_e5064_d_n4;
        locals.var_kstressu0_dn6 = assign5710_e5064_d_n6;
        locals.var_kstressu0_dn7 = assign5710_e5064_d_n7;
        locals.var_kstressu0_dn8 = assign5710_e5064_d_n8;
        locals.var_kstressu0_dn9 = assign5710_e5064_d_n9;
        locals.var_kstressu0_rv = 0.0;

        let (assign5720_e5079, assign5720_e5079_d_n4, assign5720_e5079_d_n6, assign5720_e5079_d_n7, assign5720_e5079_d_n8, assign5720_e5079_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5720_e5074: f64 = (locals.var_invsa + locals.var_invsb);
        let assign5720_e5075: f64 = (p.p461 * assign5720_e5074);
        let assign5720_e5077: f64 = (assign5720_e5075 / locals.var_kstressu0);
        (assign5720_e5077, ((((p.p461 * locals.var_invsa_dn4) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn4)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p461 * locals.var_invsa_dn6) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn6)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p461 * locals.var_invsa_dn7) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn7)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p461 * locals.var_invsa_dn8) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn8)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p461 * locals.var_invsa_dn9) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn9)) / (locals.var_kstressu0 * locals.var_kstressu0)),)
    } else {
        (locals.var_rhobeta, locals.var_rhobeta_dn4, locals.var_rhobeta_dn6, locals.var_rhobeta_dn7, locals.var_rhobeta_dn8, locals.var_rhobeta_dn9,)
    }
};
        locals.var_rhobeta = assign5720_e5079;
        locals.var_rhobeta_dn4 = assign5720_e5079_d_n4;
        locals.var_rhobeta_dn6 = assign5720_e5079_d_n6;
        locals.var_rhobeta_dn7 = assign5720_e5079_d_n7;
        locals.var_rhobeta_dn8 = assign5720_e5079_d_n8;
        locals.var_rhobeta_dn9 = assign5720_e5079_d_n9;
        locals.var_rhobeta_rv = 0.0;

        let (assign5730_e5094, assign5730_e5094_d_n4, assign5730_e5094_d_n6, assign5730_e5094_d_n7, assign5730_e5094_d_n8, assign5730_e5094_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5730_e5089: f64 = (locals.var_invsaref + locals.var_invsbref);
        let assign5730_e5090: f64 = (p.p461 * assign5730_e5089);
        let assign5730_e5092: f64 = (assign5730_e5090 / locals.var_kstressu0);
        (assign5730_e5092, (-((assign5730_e5090 * locals.var_kstressu0_dn4) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5730_e5090 * locals.var_kstressu0_dn6) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5730_e5090 * locals.var_kstressu0_dn7) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5730_e5090 * locals.var_kstressu0_dn8) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5730_e5090 * locals.var_kstressu0_dn9) / (locals.var_kstressu0 * locals.var_kstressu0))),)
    } else {
        (locals.var_rhobetaref, locals.var_rhobetaref_dn4, locals.var_rhobetaref_dn6, locals.var_rhobetaref_dn7, locals.var_rhobetaref_dn8, locals.var_rhobetaref_dn9,)
    }
};
        locals.var_rhobetaref = assign5730_e5094;
        locals.var_rhobetaref_dn4 = assign5730_e5094_d_n4;
        locals.var_rhobetaref_dn6 = assign5730_e5094_d_n6;
        locals.var_rhobetaref_dn7 = assign5730_e5094_d_n7;
        locals.var_rhobetaref_dn8 = assign5730_e5094_d_n8;
        locals.var_rhobetaref_dn9 = assign5730_e5094_d_n9;
        locals.var_rhobetaref_rv = 0.0;

        let (assign5740_e5107,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5740_e5104: f64 = (locals.var_lx).powf(p.p473);
        let assign5740_e5105: f64 = (1.0 / assign5740_e5104);
        (assign5740_e5105,)
    } else {
        (locals.var_templ,)
    }
};
        locals.var_templ = assign5740_e5107;
        locals.var_templ_rv = 0.0;

        let (assign5750_e5120,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5750_e5117: f64 = (locals.var_wx).powf(p.p474);
        let assign5750_e5118: f64 = (1.0 / assign5750_e5117);
        (assign5750_e5118,)
    } else {
        (locals.var_tempw,)
    }
};
        locals.var_tempw = assign5750_e5120;
        locals.var_tempw_rv = 0.0;

        let (assign5760_e5145,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5760_e5130: f64 = (p.p470 * locals.var_templ);
        let assign5760_e5131: f64 = (1.0 + assign5760_e5130);
        let assign5760_e5134: f64 = (p.p471 * locals.var_tempw);
        let assign5760_e5135: f64 = (assign5760_e5131 + assign5760_e5134);
        let assign5760_e5138: f64 = (p.p472 * locals.var_templ);
        let assign5760_e5140: f64 = (assign5760_e5138 * locals.var_tempw);
        let assign5760_e5141: f64 = (assign5760_e5135 + assign5760_e5140);
        let assign5760_e5143: f64 = (assign5760_e5141).max(1e-20);
        (assign5760_e5143,)
    } else {
        (locals.var_kstressvth0,)
    }
};
        locals.var_kstressvth0 = assign5760_e5145;
        locals.var_kstressvth0_rv = 0.0;

        let (assign5770_e5160, assign5770_e5160_d_n4, assign5770_e5160_d_n6, assign5770_e5160_d_n7, assign5770_e5160_d_n8, assign5770_e5160_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5770_e5154: f64 = (locals.var_invsa + locals.var_invsb);
        let assign5770_e5156: f64 = (assign5770_e5154 - locals.var_invsaref);
        let assign5770_e5158: f64 = (assign5770_e5156 - locals.var_invsbref);
        (assign5770_e5158, locals.var_invsa_dn4, locals.var_invsa_dn6, locals.var_invsa_dn7, locals.var_invsa_dn8, locals.var_invsa_dn9,)
    } else {
        (locals.var_temp0__blk79, locals.var_temp0__blk79_dn4, locals.var_temp0__blk79_dn6, locals.var_temp0__blk79_dn7, locals.var_temp0__blk79_dn8, locals.var_temp0__blk79_dn9,)
    }
};
        locals.var_temp0__blk79 = assign5770_e5160;
        locals.var_temp0__blk79_dn4 = assign5770_e5160_d_n4;
        locals.var_temp0__blk79_dn6 = assign5770_e5160_d_n6;
        locals.var_temp0__blk79_dn7 = assign5770_e5160_d_n7;
        locals.var_temp0__blk79_dn8 = assign5770_e5160_d_n8;
        locals.var_temp0__blk79_dn9 = assign5770_e5160_d_n9;
        locals.var_temp0__blk79_rv = 0.0;

        let (assign5780_e5177, assign5780_e5177_d_n4, assign5780_e5177_d_n6, assign5780_e5177_d_n7, assign5780_e5177_d_n8, assign5780_e5177_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5780_e5170: f64 = (1.0 + locals.var_rhobeta);
        let assign5780_e5171: f64 = (locals.var_betn_p * assign5780_e5170);
        let assign5780_e5174: f64 = (1.0 + locals.var_rhobetaref);
        let assign5780_e5175: f64 = (assign5780_e5171 / assign5780_e5174);
        (assign5780_e5175, (((((locals.var_betn_p_dn4 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn4)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn4)) / (assign5780_e5174 * assign5780_e5174)), (((((locals.var_betn_p_dn6 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn6)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn6)) / (assign5780_e5174 * assign5780_e5174)), (((((locals.var_betn_p_dn7 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn7)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn7)) / (assign5780_e5174 * assign5780_e5174)), (((((locals.var_betn_p_dn8 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn8)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn8)) / (assign5780_e5174 * assign5780_e5174)), (((((locals.var_betn_p_dn9 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn9)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn9)) / (assign5780_e5174 * assign5780_e5174)),)
    } else {
        (locals.var_betn_p, locals.var_betn_p_dn4, locals.var_betn_p_dn6, locals.var_betn_p_dn7, locals.var_betn_p_dn8, locals.var_betn_p_dn9,)
    }
};
        locals.var_betn_p = assign5780_e5177;
        locals.var_betn_p_dn4 = assign5780_e5177_d_n4;
        locals.var_betn_p_dn6 = assign5780_e5177_d_n6;
        locals.var_betn_p_dn7 = assign5780_e5177_d_n7;
        locals.var_betn_p_dn8 = assign5780_e5177_d_n8;
        locals.var_betn_p_dn9 = assign5780_e5177_d_n9;
        locals.var_betn_p_rv = 0.0;

        let (assign5790_e5188, assign5790_e5188_d_n4, assign5790_e5188_d_n6, assign5790_e5188_d_n7, assign5790_e5188_d_n8, assign5790_e5188_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5790_e5186: f64 = (locals.var_betn_p).max(1e-10);
        (assign5790_e5186, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn4 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn6 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn7 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn8 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9,)
    }
};
        locals.var_betn1_t = assign5790_e5188;
        locals.var_betn1_t_dn4 = assign5790_e5188_d_n4;
        locals.var_betn1_t_dn6 = assign5790_e5188_d_n6;
        locals.var_betn1_t_dn7 = assign5790_e5188_d_n7;
        locals.var_betn1_t_dn8 = assign5790_e5188_d_n8;
        locals.var_betn1_t_dn9 = assign5790_e5188_d_n9;
        locals.var_betn1_t_rv = 0.0;

        let (assign5800_e5199, assign5800_e5199_d_n4, assign5800_e5199_d_n6, assign5800_e5199_d_n7, assign5800_e5199_d_n8, assign5800_e5199_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5800_e5197: f64 = (p.p250 * locals.var_betn1_t);
        (assign5800_e5197, (p.p250 * locals.var_betn1_t_dn4), (p.p250 * locals.var_betn1_t_dn6), (p.p250 * locals.var_betn1_t_dn7), (p.p250 * locals.var_betn1_t_dn8), (p.p250 * locals.var_betn1_t_dn9),)
    } else {
        (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9,)
    }
};
        locals.var_betn2_t = assign5800_e5199;
        locals.var_betn2_t_dn4 = assign5800_e5199_d_n4;
        locals.var_betn2_t_dn6 = assign5800_e5199_d_n6;
        locals.var_betn2_t_dn7 = assign5800_e5199_d_n7;
        locals.var_betn2_t_dn8 = assign5800_e5199_d_n8;
        locals.var_betn2_t_dn9 = assign5800_e5199_d_n9;
        locals.var_betn2_t_rv = 0.0;

        let (assign5810_e5226, assign5810_e5226_d_n4, assign5810_e5226_d_n6, assign5810_e5226_d_n7, assign5810_e5226_d_n8, assign5810_e5226_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5810_e5208: f64 = (1.0 + locals.var_rhobeta);
        let assign5810_e5212: f64 = (p.p462 * locals.var_rhobetaref);
        let assign5810_e5213: f64 = (1.0 + assign5810_e5212);
        let assign5810_e5214: f64 = (assign5810_e5208 * assign5810_e5213);
        let assign5810_e5217: f64 = (1.0 + locals.var_rhobetaref);
        let assign5810_e5221: f64 = (p.p462 * locals.var_rhobeta);
        let assign5810_e5222: f64 = (1.0 + assign5810_e5221);
        let assign5810_e5223: f64 = (assign5810_e5217 * assign5810_e5222);
        let assign5810_e5224: f64 = (assign5810_e5214 / assign5810_e5223);
        (assign5810_e5224, (((((locals.var_rhobeta_dn4 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn4))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn4 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn4))))) / (assign5810_e5223 * assign5810_e5223)), (((((locals.var_rhobeta_dn6 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn6))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn6 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn6))))) / (assign5810_e5223 * assign5810_e5223)), (((((locals.var_rhobeta_dn7 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn7))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn7 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn7))))) / (assign5810_e5223 * assign5810_e5223)), (((((locals.var_rhobeta_dn8 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn8))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn8 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn8))))) / (assign5810_e5223 * assign5810_e5223)), (((((locals.var_rhobeta_dn9 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn9))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn9 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn9))))) / (assign5810_e5223 * assign5810_e5223)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign5810_e5226;
        locals.var_temp_dn4 = assign5810_e5226_d_n4;
        locals.var_temp_dn6 = assign5810_e5226_d_n6;
        locals.var_temp_dn7 = assign5810_e5226_d_n7;
        locals.var_temp_dn8 = assign5810_e5226_d_n8;
        locals.var_temp_dn9 = assign5810_e5226_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign5820_e5237, assign5820_e5237_d_n4, assign5820_e5237_d_n6, assign5820_e5237_d_n7, assign5820_e5237_d_n8, assign5820_e5237_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5820_e5235: f64 = (locals.var_thesat_p * locals.var_temp);
        (assign5820_e5235, ((locals.var_thesat_p_dn4 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn4)), ((locals.var_thesat_p_dn6 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn6)), ((locals.var_thesat_p_dn7 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn7)), ((locals.var_thesat_p_dn8 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn8)), ((locals.var_thesat_p_dn9 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn9)),)
    } else {
        (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9,)
    }
};
        locals.var_thesat_p = assign5820_e5237;
        locals.var_thesat_p_dn4 = assign5820_e5237_d_n4;
        locals.var_thesat_p_dn6 = assign5820_e5237_d_n6;
        locals.var_thesat_p_dn7 = assign5820_e5237_d_n7;
        locals.var_thesat_p_dn8 = assign5820_e5237_d_n8;
        locals.var_thesat_p_dn9 = assign5820_e5237_d_n9;
        locals.var_thesat_p_rv = 0.0;

        let (assign5830_e5248, assign5830_e5248_d_n4, assign5830_e5248_d_n6, assign5830_e5248_d_n7, assign5830_e5248_d_n8, assign5830_e5248_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5830_e5246: f64 = (locals.var_thesat_p).max(0.0);
        (assign5830_e5246, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn4 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn6 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn7 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn8 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9,)
    }
};
        locals.var_thesat_t = assign5830_e5248;
        locals.var_thesat_t_dn4 = assign5830_e5248_d_n4;
        locals.var_thesat_t_dn6 = assign5830_e5248_d_n6;
        locals.var_thesat_t_dn7 = assign5830_e5248_d_n7;
        locals.var_thesat_t_dn8 = assign5830_e5248_d_n8;
        locals.var_thesat_t_dn9 = assign5830_e5248_d_n9;
        locals.var_thesat_t_rv = 0.0;

        let (assign5840_e5259, assign5840_e5259_d_n4, assign5840_e5259_d_n6, assign5840_e5259_d_n7, assign5840_e5259_d_n8, assign5840_e5259_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5840_e5257: f64 = (locals.var_thesatac_p * locals.var_temp);
        (assign5840_e5257, ((locals.var_thesatac_p_dn4 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn4)), ((locals.var_thesatac_p_dn6 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn6)), ((locals.var_thesatac_p_dn7 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn7)), ((locals.var_thesatac_p_dn8 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn8)), ((locals.var_thesatac_p_dn9 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn9)),)
    } else {
        (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9,)
    }
};
        locals.var_thesatac_p = assign5840_e5259;
        locals.var_thesatac_p_dn4 = assign5840_e5259_d_n4;
        locals.var_thesatac_p_dn6 = assign5840_e5259_d_n6;
        locals.var_thesatac_p_dn7 = assign5840_e5259_d_n7;
        locals.var_thesatac_p_dn8 = assign5840_e5259_d_n8;
        locals.var_thesatac_p_dn9 = assign5840_e5259_d_n9;
        locals.var_thesatac_p_rv = 0.0;

        let (assign5850_e5270, assign5850_e5270_d_n4, assign5850_e5270_d_n6, assign5850_e5270_d_n7, assign5850_e5270_d_n8, assign5850_e5270_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5850_e5268: f64 = (locals.var_thesatac_p).max(0.0);
        (assign5850_e5268, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn4 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn6 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn7 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn8 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9,)
    }
};
        locals.var_thesatac_t = assign5850_e5270;
        locals.var_thesatac_t_dn4 = assign5850_e5270_d_n4;
        locals.var_thesatac_t_dn6 = assign5850_e5270_d_n6;
        locals.var_thesatac_t_dn7 = assign5850_e5270_d_n7;
        locals.var_thesatac_t_dn8 = assign5850_e5270_d_n8;
        locals.var_thesatac_t_dn9 = assign5850_e5270_d_n9;
        locals.var_thesatac_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5860_e5283, assign5860_e5283_d_n4, assign5860_e5283_d_n6, assign5860_e5283_d_n7, assign5860_e5283_d_n8, assign5860_e5283_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5860_e5279: f64 = (p.p469 * locals.var_temp0__blk79);
        let assign5860_e5281: f64 = (assign5860_e5279 / locals.var_kstressvth0);
        (assign5860_e5281, ((p.p469 * locals.var_temp0__blk79_dn4) / locals.var_kstressvth0), ((p.p469 * locals.var_temp0__blk79_dn6) / locals.var_kstressvth0), ((p.p469 * locals.var_temp0__blk79_dn7) / locals.var_kstressvth0), ((p.p469 * locals.var_temp0__blk79_dn8) / locals.var_kstressvth0), ((p.p469 * locals.var_temp0__blk79_dn9) / locals.var_kstressvth0),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign5860_e5283;
        locals.var_temp_dn4 = assign5860_e5283_d_n4;
        locals.var_temp_dn6 = assign5860_e5283_d_n6;
        locals.var_temp_dn7 = assign5860_e5283_d_n7;
        locals.var_temp_dn8 = assign5860_e5283_d_n8;
        locals.var_temp_dn9 = assign5860_e5283_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign5870_e5294, assign5870_e5294_d_n4, assign5870_e5294_d_n6, assign5870_e5294_d_n7, assign5870_e5294_d_n8, assign5870_e5294_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5870_e5292: f64 = (locals.var_vfb1_t + locals.var_temp);
        (assign5870_e5292, (locals.var_vfb1_t_dn4 + locals.var_temp_dn4), (locals.var_vfb1_t_dn6 + locals.var_temp_dn6), (locals.var_vfb1_t_dn7 + locals.var_temp_dn7), (locals.var_vfb1_t_dn8 + locals.var_temp_dn8), (locals.var_vfb1_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9,)
    }
};
        locals.var_vfb1_t = assign5870_e5294;
        locals.var_vfb1_t_dn4 = assign5870_e5294_d_n4;
        locals.var_vfb1_t_dn6 = assign5870_e5294_d_n6;
        locals.var_vfb1_t_dn7 = assign5870_e5294_d_n7;
        locals.var_vfb1_t_dn8 = assign5870_e5294_d_n8;
        locals.var_vfb1_t_dn9 = assign5870_e5294_d_n9;
        locals.var_vfb1_t_rv = 0.0;

        let (assign5880_e5305, assign5880_e5305_d_n4, assign5880_e5305_d_n6, assign5880_e5305_d_n7, assign5880_e5305_d_n8, assign5880_e5305_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5880_e5303: f64 = (locals.var_vfb2_t + locals.var_temp);
        (assign5880_e5303, (locals.var_vfb2_t_dn4 + locals.var_temp_dn4), (locals.var_vfb2_t_dn6 + locals.var_temp_dn6), (locals.var_vfb2_t_dn7 + locals.var_temp_dn7), (locals.var_vfb2_t_dn8 + locals.var_temp_dn8), (locals.var_vfb2_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    }
};
        locals.var_vfb2_t = assign5880_e5305;
        locals.var_vfb2_t_dn4 = assign5880_e5305_d_n4;
        locals.var_vfb2_t_dn6 = assign5880_e5305_d_n6;
        locals.var_vfb2_t_dn7 = assign5880_e5305_d_n7;
        locals.var_vfb2_t_dn8 = assign5880_e5305_d_n8;
        locals.var_vfb2_t_dn9 = assign5880_e5305_d_n9;
        locals.var_vfb2_t_rv = 0.0;

        let (assign5890_e5316, assign5890_e5316_d_n4, assign5890_e5316_d_n6, assign5890_e5316_d_n7, assign5890_e5316_d_n8, assign5890_e5316_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5890_e5314: f64 = (locals.var_vfbac1_t + locals.var_temp);
        (assign5890_e5314, (locals.var_vfbac1_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac1_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac1_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac1_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac1_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9,)
    }
};
        locals.var_vfbac1_t = assign5890_e5316;
        locals.var_vfbac1_t_dn4 = assign5890_e5316_d_n4;
        locals.var_vfbac1_t_dn6 = assign5890_e5316_d_n6;
        locals.var_vfbac1_t_dn7 = assign5890_e5316_d_n7;
        locals.var_vfbac1_t_dn8 = assign5890_e5316_d_n8;
        locals.var_vfbac1_t_dn9 = assign5890_e5316_d_n9;
        locals.var_vfbac1_t_rv = 0.0;

        let (assign5900_e5327, assign5900_e5327_d_n4, assign5900_e5327_d_n6, assign5900_e5327_d_n7, assign5900_e5327_d_n8, assign5900_e5327_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5900_e5325: f64 = (locals.var_vfbac2_t + locals.var_temp);
        (assign5900_e5325, (locals.var_vfbac2_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac2_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac2_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac2_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac2_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign5900_e5327;
        locals.var_vfbac2_t_dn4 = assign5900_e5327_d_n4;
        locals.var_vfbac2_t_dn6 = assign5900_e5327_d_n6;
        locals.var_vfbac2_t_dn7 = assign5900_e5327_d_n7;
        locals.var_vfbac2_t_dn8 = assign5900_e5327_d_n8;
        locals.var_vfbac2_t_dn9 = assign5900_e5327_d_n9;
        locals.var_vfbac2_t_rv = 0.0;

        let (assign5910_e5342, assign5910_e5342_d_n4, assign5910_e5342_d_n6, assign5910_e5342_d_n7, assign5910_e5342_d_n8, assign5910_e5342_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5910_e5336: f64 = (p.p475 * locals.var_temp0__blk79);
        let assign5910_e5339: f64 = (locals.var_kstressvth0).powf(p.p476);
        let assign5910_e5340: f64 = (assign5910_e5336 / assign5910_e5339);
        (assign5910_e5340, ((p.p475 * locals.var_temp0__blk79_dn4) / assign5910_e5339), ((p.p475 * locals.var_temp0__blk79_dn6) / assign5910_e5339), ((p.p475 * locals.var_temp0__blk79_dn7) / assign5910_e5339), ((p.p475 * locals.var_temp0__blk79_dn8) / assign5910_e5339), ((p.p475 * locals.var_temp0__blk79_dn9) / assign5910_e5339),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign5910_e5342;
        locals.var_temp_dn4 = assign5910_e5342_d_n4;
        locals.var_temp_dn6 = assign5910_e5342_d_n6;
        locals.var_temp_dn7 = assign5910_e5342_d_n7;
        locals.var_temp_dn8 = assign5910_e5342_d_n8;
        locals.var_temp_dn9 = assign5910_e5342_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign5920_e5353, assign5920_e5353_d_n4, assign5920_e5353_d_n6, assign5920_e5353_d_n7, assign5920_e5353_d_n8, assign5920_e5353_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5920_e5351: f64 = (locals.var_cf_p + locals.var_temp);
        (assign5920_e5351, (locals.var_cf_p_dn4 + locals.var_temp_dn4), (locals.var_cf_p_dn6 + locals.var_temp_dn6), (locals.var_cf_p_dn7 + locals.var_temp_dn7), (locals.var_cf_p_dn8 + locals.var_temp_dn8), (locals.var_cf_p_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9,)
    }
};
        locals.var_cf_p = assign5920_e5353;
        locals.var_cf_p_dn4 = assign5920_e5353_d_n4;
        locals.var_cf_p_dn6 = assign5920_e5353_d_n6;
        locals.var_cf_p_dn7 = assign5920_e5353_d_n7;
        locals.var_cf_p_dn8 = assign5920_e5353_d_n8;
        locals.var_cf_p_dn9 = assign5920_e5353_d_n9;
        locals.var_cf_p_rv = 0.0;

        let (assign5930_e5364, assign5930_e5364_d_n4, assign5930_e5364_d_n6, assign5930_e5364_d_n7, assign5930_e5364_d_n8, assign5930_e5364_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5930_e5362: f64 = (locals.var_cf_p).max(0.0);
        (assign5930_e5362, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn4 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn6 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn7 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn8 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9,)
    }
};
        locals.var_cf1_t = assign5930_e5364;
        locals.var_cf1_t_dn4 = assign5930_e5364_d_n4;
        locals.var_cf1_t_dn6 = assign5930_e5364_d_n6;
        locals.var_cf1_t_dn7 = assign5930_e5364_d_n7;
        locals.var_cf1_t_dn8 = assign5930_e5364_d_n8;
        locals.var_cf1_t_dn9 = assign5930_e5364_d_n9;
        locals.var_cf1_t_rv = 0.0;

        let (assign5940_e5375, assign5940_e5375_d_n4, assign5940_e5375_d_n6, assign5940_e5375_d_n7, assign5940_e5375_d_n8, assign5940_e5375_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5940_e5373: f64 = (locals.var_cfac_p + locals.var_temp);
        (assign5940_e5373, (locals.var_cfac_p_dn4 + locals.var_temp_dn4), (locals.var_cfac_p_dn6 + locals.var_temp_dn6), (locals.var_cfac_p_dn7 + locals.var_temp_dn7), (locals.var_cfac_p_dn8 + locals.var_temp_dn8), (locals.var_cfac_p_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9,)
    }
};
        locals.var_cfac_p = assign5940_e5375;
        locals.var_cfac_p_dn4 = assign5940_e5375_d_n4;
        locals.var_cfac_p_dn6 = assign5940_e5375_d_n6;
        locals.var_cfac_p_dn7 = assign5940_e5375_d_n7;
        locals.var_cfac_p_dn8 = assign5940_e5375_d_n8;
        locals.var_cfac_p_dn9 = assign5940_e5375_d_n9;
        locals.var_cfac_p_rv = 0.0;

        let (assign5950_e5386, assign5950_e5386_d_n4, assign5950_e5386_d_n6, assign5950_e5386_d_n7, assign5950_e5386_d_n8, assign5950_e5386_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5950_e5384: f64 = (locals.var_cfac_p).max(0.0);
        (assign5950_e5384, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn4 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn6 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn7 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn8 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9,)
    }
};
        locals.var_cfac1_t = assign5950_e5386;
        locals.var_cfac1_t_dn4 = assign5950_e5386_d_n4;
        locals.var_cfac1_t_dn6 = assign5950_e5386_d_n6;
        locals.var_cfac1_t_dn7 = assign5950_e5386_d_n7;
        locals.var_cfac1_t_dn8 = assign5950_e5386_d_n8;
        locals.var_cfac1_t_dn9 = assign5950_e5386_d_n9;
        locals.var_cfac1_t_rv = 0.0;

        let (assign5960_e5399, assign5960_e5399_d_n4, assign5960_e5399_d_n6, assign5960_e5399_d_n7, assign5960_e5399_d_n8, assign5960_e5399_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5960_e5395: f64 = (p.p234 * locals.var_tox2_i);
        let assign5960_e5397: f64 = (assign5960_e5395 / locals.var_tox1_i);
        (assign5960_e5397, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign5960_e5399;
        locals.var_temp_dn4 = assign5960_e5399_d_n4;
        locals.var_temp_dn6 = assign5960_e5399_d_n6;
        locals.var_temp_dn7 = assign5960_e5399_d_n7;
        locals.var_temp_dn8 = assign5960_e5399_d_n8;
        locals.var_temp_dn9 = assign5960_e5399_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign5970_e5410, assign5970_e5410_d_n4, assign5970_e5410_d_n6, assign5970_e5410_d_n7, assign5970_e5410_d_n8, assign5970_e5410_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5970_e5408: f64 = (locals.var_cf1_t * locals.var_temp);
        (assign5970_e5408, ((locals.var_cf1_t_dn4 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn4)), ((locals.var_cf1_t_dn6 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn6)), ((locals.var_cf1_t_dn7 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn7)), ((locals.var_cf1_t_dn8 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn8)), ((locals.var_cf1_t_dn9 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn9)),)
    } else {
        (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9,)
    }
};
        locals.var_cf2_t = assign5970_e5410;
        locals.var_cf2_t_dn4 = assign5970_e5410_d_n4;
        locals.var_cf2_t_dn6 = assign5970_e5410_d_n6;
        locals.var_cf2_t_dn7 = assign5970_e5410_d_n7;
        locals.var_cf2_t_dn8 = assign5970_e5410_d_n8;
        locals.var_cf2_t_dn9 = assign5970_e5410_d_n9;
        locals.var_cf2_t_rv = 0.0;

        let (assign5980_e5421, assign5980_e5421_d_n4, assign5980_e5421_d_n6, assign5980_e5421_d_n7, assign5980_e5421_d_n8, assign5980_e5421_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5980_e5419: f64 = (locals.var_cfac1_t * locals.var_temp);
        (assign5980_e5419, ((locals.var_cfac1_t_dn4 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn4)), ((locals.var_cfac1_t_dn6 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn6)), ((locals.var_cfac1_t_dn7 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn7)), ((locals.var_cfac1_t_dn8 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn8)), ((locals.var_cfac1_t_dn9 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn9)),)
    } else {
        (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9,)
    }
};
        locals.var_cfac2_t = assign5980_e5421;
        locals.var_cfac2_t_dn4 = assign5980_e5421_d_n4;
        locals.var_cfac2_t_dn6 = assign5980_e5421_d_n6;
        locals.var_cfac2_t_dn7 = assign5980_e5421_d_n7;
        locals.var_cfac2_t_dn8 = assign5980_e5421_d_n8;
        locals.var_cfac2_t_dn9 = assign5980_e5421_d_n9;
        locals.var_cfac2_t_rv = 0.0;

        let (assign5990_e5431, assign5990_e5431_d_n4, assign5990_e5431_d_n6, assign5990_e5431_d_n7, assign5990_e5431_d_n8, assign5990_e5431_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9,)
    }
};
        locals.var_tmpa = assign5990_e5431;
        locals.var_tmpa_dn4 = assign5990_e5431_d_n4;
        locals.var_tmpa_dn6 = assign5990_e5431_d_n6;
        locals.var_tmpa_dn7 = assign5990_e5431_d_n7;
        locals.var_tmpa_dn8 = assign5990_e5431_d_n8;
        locals.var_tmpa_dn9 = assign5990_e5431_d_n9;
        locals.var_tmpa_rv = 0.0;

        let (assign6000_e5441,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_iloop,)
    }
};
        locals.var_iloop = assign6000_e5441;
        locals.var_iloop_rv = 0.0;

        let (assign6010_e5454, assign6010_e5454_d_n4, assign6010_e5454_d_n6, assign6010_e5454_d_n7, assign6010_e5454_d_n8, assign6010_e5454_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6010_e5450: f64 = (-1.0);
        let assign6010_e5452: f64 = (assign6010_e5450 / p.p478);
        (assign6010_e5452, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign6010_e5454;
        locals.var_temp_dn4 = assign6010_e5454_d_n4;
        locals.var_temp_dn6 = assign6010_e5454_d_n6;
        locals.var_temp_dn7 = assign6010_e5454_d_n7;
        locals.var_temp_dn8 = assign6010_e5454_d_n8;
        locals.var_temp_dn9 = assign6010_e5454_d_n9;
        locals.var_temp_rv = 0.0;

        let mut assign6020_loop_guard: usize = 0;
        while {
            let assign6020_cond_e5465: f64 = (p.p29 - 0.5);
            let assign6020_cond_e5467: f64 = if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_iloop < assign6020_cond_e5465)) { 1.0 } else { 0.0 };
            assign6020_cond_e5467 != 0.0
        } {
            assign6020_loop_guard += 1;
            assert!(assign6020_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let assign6020_body0_e5471: f64 = (0.5 * p.p20);
            let assign6020_body0_e5472: f64 = (p.p26 + assign6020_body0_e5471);
            let assign6020_body0_e5476: f64 = (p.p28 + p.p20);
            let assign6020_body0_e5477: f64 = (locals.var_iloop * assign6020_body0_e5476);
            let assign6020_body0_e5478: f64 = (assign6020_body0_e5472 + assign6020_body0_e5477);
            let assign6020_body0_e5479: f64 = (-assign6020_body0_e5478);
            let assign6020_body0_e5481: f64 = (assign6020_body0_e5479 / p.p477);
            let assign6020_body0_e5483: f64 = (-80.0);
            let assign6020_body0_e5484: f64 = if assign6020_body0_e5481 > assign6020_body0_e5483 { 1.0 } else { 0.0 };
            locals.var_guard135 = assign6020_body0_e5484;
            locals.var_guard135_rv = 0.0;
            let (assign6020_body1_e5510, assign6020_body1_e5510_d_n4, assign6020_body1_e5510_d_n6, assign6020_body1_e5510_d_n7, assign6020_body1_e5510_d_n8, assign6020_body1_e5510_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard135 != 0.0)) {
        let assign6020_body1_e5497: f64 = (0.5 * p.p20);
        let assign6020_body1_e5498: f64 = (p.p26 + assign6020_body1_e5497);
        let assign6020_body1_e5502: f64 = (p.p28 + p.p20);
        let assign6020_body1_e5503: f64 = (locals.var_iloop * assign6020_body1_e5502);
        let assign6020_body1_e5504: f64 = (assign6020_body1_e5498 + assign6020_body1_e5503);
        let assign6020_body1_e5505: f64 = (-assign6020_body1_e5504);
        let assign6020_body1_e5507: f64 = (assign6020_body1_e5505 / p.p477);
        let assign6020_body1_e5508: f64 = (assign6020_body1_e5507).exp();
        (assign6020_body1_e5508, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
            locals.var_temp1 = assign6020_body1_e5510;
            locals.var_temp1_dn4 = assign6020_body1_e5510_d_n4;
            locals.var_temp1_dn6 = assign6020_body1_e5510_d_n6;
            locals.var_temp1_dn7 = assign6020_body1_e5510_d_n7;
            locals.var_temp1_dn8 = assign6020_body1_e5510_d_n8;
            locals.var_temp1_dn9 = assign6020_body1_e5510_d_n9;
            locals.var_temp1_rv = 0.0;
            let (assign6020_body2_e5587, assign6020_body2_e5587_d_n4, assign6020_body2_e5587_d_n6, assign6020_body2_e5587_d_n7, assign6020_body2_e5587_d_n8, assign6020_body2_e5587_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard135 == 0.0)) {
        let assign6020_body2_e5526: f64 = (0.5 * p.p20);
        let assign6020_body2_e5527: f64 = (p.p26 + assign6020_body2_e5526);
        let assign6020_body2_e5531: f64 = (p.p28 + p.p20);
        let assign6020_body2_e5532: f64 = (locals.var_iloop * assign6020_body2_e5531);
        let assign6020_body2_e5533: f64 = (assign6020_body2_e5527 + assign6020_body2_e5532);
        let assign6020_body2_e5534: f64 = (-assign6020_body2_e5533);
        let assign6020_body2_e5536: f64 = (assign6020_body2_e5534 / p.p477);
        let assign6020_body2_e5537: f64 = (-assign6020_body2_e5536);
        let assign6020_body2_e5539: f64 = (assign6020_body2_e5537 - 80.0);
        let assign6020_body2_e5545: f64 = (0.5 * p.p20);
        let assign6020_body2_e5546: f64 = (p.p26 + assign6020_body2_e5545);
        let assign6020_body2_e5550: f64 = (p.p28 + p.p20);
        let assign6020_body2_e5551: f64 = (locals.var_iloop * assign6020_body2_e5550);
        let assign6020_body2_e5552: f64 = (assign6020_body2_e5546 + assign6020_body2_e5551);
        let assign6020_body2_e5553: f64 = (-assign6020_body2_e5552);
        let assign6020_body2_e5555: f64 = (assign6020_body2_e5553 / p.p477);
        let assign6020_body2_e5556: f64 = (-assign6020_body2_e5555);
        let assign6020_body2_e5558: f64 = (assign6020_body2_e5556 - 80.0);
        let assign6020_body2_e5559: f64 = (0.5 * assign6020_body2_e5558);
        let assign6020_body2_e5564: f64 = (0.5 * p.p20);
        let assign6020_body2_e5565: f64 = (p.p26 + assign6020_body2_e5564);
        let assign6020_body2_e5569: f64 = (p.p28 + p.p20);
        let assign6020_body2_e5570: f64 = (locals.var_iloop * assign6020_body2_e5569);
        let assign6020_body2_e5571: f64 = (assign6020_body2_e5565 + assign6020_body2_e5570);
        let assign6020_body2_e5572: f64 = (-assign6020_body2_e5571);
        let assign6020_body2_e5574: f64 = (assign6020_body2_e5572 / p.p477);
        let assign6020_body2_e5575: f64 = (-assign6020_body2_e5574);
        let assign6020_body2_e5577: f64 = (assign6020_body2_e5575 - 80.0);
        let assign6020_body2_e5579: f64 = (assign6020_body2_e5577 * 0.3333333333333);
        let assign6020_body2_e5580: f64 = (1.0 + assign6020_body2_e5579);
        let assign6020_body2_e5581: f64 = (assign6020_body2_e5559 * assign6020_body2_e5580);
        let assign6020_body2_e5582: f64 = (1.0 + assign6020_body2_e5581);
        let assign6020_body2_e5583: f64 = (assign6020_body2_e5539 * assign6020_body2_e5582);
        let assign6020_body2_e5584: f64 = (1.0 + assign6020_body2_e5583);
        let assign6020_body2_e5585: f64 = (1.80485e-35 / assign6020_body2_e5584);
        (assign6020_body2_e5585, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
            locals.var_temp1 = assign6020_body2_e5587;
            locals.var_temp1_dn4 = assign6020_body2_e5587_d_n4;
            locals.var_temp1_dn6 = assign6020_body2_e5587_d_n6;
            locals.var_temp1_dn7 = assign6020_body2_e5587_d_n7;
            locals.var_temp1_dn8 = assign6020_body2_e5587_d_n8;
            locals.var_temp1_dn9 = assign6020_body2_e5587_d_n9;
            locals.var_temp1_rv = 0.0;
            let assign6020_body3_e5591: f64 = (0.5 * p.p20);
            let assign6020_body3_e5592: f64 = (p.p27 + assign6020_body3_e5591);
            let assign6020_body3_e5595: f64 = (p.p29 - 1.0);
            let assign6020_body3_e5597: f64 = (assign6020_body3_e5595 - locals.var_iloop);
            let assign6020_body3_e5600: f64 = (p.p28 + p.p20);
            let assign6020_body3_e5601: f64 = (assign6020_body3_e5597 * assign6020_body3_e5600);
            let assign6020_body3_e5602: f64 = (assign6020_body3_e5592 + assign6020_body3_e5601);
            let assign6020_body3_e5603: f64 = (-assign6020_body3_e5602);
            let assign6020_body3_e5605: f64 = (assign6020_body3_e5603 / p.p477);
            let assign6020_body3_e5607: f64 = (-80.0);
            let assign6020_body3_e5608: f64 = if assign6020_body3_e5605 > assign6020_body3_e5607 { 1.0 } else { 0.0 };
            locals.var_guard136 = assign6020_body3_e5608;
            locals.var_guard136_rv = 0.0;
            let (assign6020_body4_e5638, assign6020_body4_e5638_d_n4, assign6020_body4_e5638_d_n6, assign6020_body4_e5638_d_n7, assign6020_body4_e5638_d_n8, assign6020_body4_e5638_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard136 != 0.0)) {
        let assign6020_body4_e5621: f64 = (0.5 * p.p20);
        let assign6020_body4_e5622: f64 = (p.p27 + assign6020_body4_e5621);
        let assign6020_body4_e5625: f64 = (p.p29 - 1.0);
        let assign6020_body4_e5627: f64 = (assign6020_body4_e5625 - locals.var_iloop);
        let assign6020_body4_e5630: f64 = (p.p28 + p.p20);
        let assign6020_body4_e5631: f64 = (assign6020_body4_e5627 * assign6020_body4_e5630);
        let assign6020_body4_e5632: f64 = (assign6020_body4_e5622 + assign6020_body4_e5631);
        let assign6020_body4_e5633: f64 = (-assign6020_body4_e5632);
        let assign6020_body4_e5635: f64 = (assign6020_body4_e5633 / p.p477);
        let assign6020_body4_e5636: f64 = (assign6020_body4_e5635).exp();
        (assign6020_body4_e5636, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
            locals.var_temp2 = assign6020_body4_e5638;
            locals.var_temp2_dn4 = assign6020_body4_e5638_d_n4;
            locals.var_temp2_dn6 = assign6020_body4_e5638_d_n6;
            locals.var_temp2_dn7 = assign6020_body4_e5638_d_n7;
            locals.var_temp2_dn8 = assign6020_body4_e5638_d_n8;
            locals.var_temp2_dn9 = assign6020_body4_e5638_d_n9;
            locals.var_temp2_rv = 0.0;
            let (assign6020_body5_e5727, assign6020_body5_e5727_d_n4, assign6020_body5_e5727_d_n6, assign6020_body5_e5727_d_n7, assign6020_body5_e5727_d_n8, assign6020_body5_e5727_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard136 == 0.0)) {
        let assign6020_body5_e5654: f64 = (0.5 * p.p20);
        let assign6020_body5_e5655: f64 = (p.p27 + assign6020_body5_e5654);
        let assign6020_body5_e5658: f64 = (p.p29 - 1.0);
        let assign6020_body5_e5660: f64 = (assign6020_body5_e5658 - locals.var_iloop);
        let assign6020_body5_e5663: f64 = (p.p28 + p.p20);
        let assign6020_body5_e5664: f64 = (assign6020_body5_e5660 * assign6020_body5_e5663);
        let assign6020_body5_e5665: f64 = (assign6020_body5_e5655 + assign6020_body5_e5664);
        let assign6020_body5_e5666: f64 = (-assign6020_body5_e5665);
        let assign6020_body5_e5668: f64 = (assign6020_body5_e5666 / p.p477);
        let assign6020_body5_e5669: f64 = (-assign6020_body5_e5668);
        let assign6020_body5_e5671: f64 = (assign6020_body5_e5669 - 80.0);
        let assign6020_body5_e5677: f64 = (0.5 * p.p20);
        let assign6020_body5_e5678: f64 = (p.p27 + assign6020_body5_e5677);
        let assign6020_body5_e5681: f64 = (p.p29 - 1.0);
        let assign6020_body5_e5683: f64 = (assign6020_body5_e5681 - locals.var_iloop);
        let assign6020_body5_e5686: f64 = (p.p28 + p.p20);
        let assign6020_body5_e5687: f64 = (assign6020_body5_e5683 * assign6020_body5_e5686);
        let assign6020_body5_e5688: f64 = (assign6020_body5_e5678 + assign6020_body5_e5687);
        let assign6020_body5_e5689: f64 = (-assign6020_body5_e5688);
        let assign6020_body5_e5691: f64 = (assign6020_body5_e5689 / p.p477);
        let assign6020_body5_e5692: f64 = (-assign6020_body5_e5691);
        let assign6020_body5_e5694: f64 = (assign6020_body5_e5692 - 80.0);
        let assign6020_body5_e5695: f64 = (0.5 * assign6020_body5_e5694);
        let assign6020_body5_e5700: f64 = (0.5 * p.p20);
        let assign6020_body5_e5701: f64 = (p.p27 + assign6020_body5_e5700);
        let assign6020_body5_e5704: f64 = (p.p29 - 1.0);
        let assign6020_body5_e5706: f64 = (assign6020_body5_e5704 - locals.var_iloop);
        let assign6020_body5_e5709: f64 = (p.p28 + p.p20);
        let assign6020_body5_e5710: f64 = (assign6020_body5_e5706 * assign6020_body5_e5709);
        let assign6020_body5_e5711: f64 = (assign6020_body5_e5701 + assign6020_body5_e5710);
        let assign6020_body5_e5712: f64 = (-assign6020_body5_e5711);
        let assign6020_body5_e5714: f64 = (assign6020_body5_e5712 / p.p477);
        let assign6020_body5_e5715: f64 = (-assign6020_body5_e5714);
        let assign6020_body5_e5717: f64 = (assign6020_body5_e5715 - 80.0);
        let assign6020_body5_e5719: f64 = (assign6020_body5_e5717 * 0.3333333333333);
        let assign6020_body5_e5720: f64 = (1.0 + assign6020_body5_e5719);
        let assign6020_body5_e5721: f64 = (assign6020_body5_e5695 * assign6020_body5_e5720);
        let assign6020_body5_e5722: f64 = (1.0 + assign6020_body5_e5721);
        let assign6020_body5_e5723: f64 = (assign6020_body5_e5671 * assign6020_body5_e5722);
        let assign6020_body5_e5724: f64 = (1.0 + assign6020_body5_e5723);
        let assign6020_body5_e5725: f64 = (1.80485e-35 / assign6020_body5_e5724);
        (assign6020_body5_e5725, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
            locals.var_temp2 = assign6020_body5_e5727;
            locals.var_temp2_dn4 = assign6020_body5_e5727_d_n4;
            locals.var_temp2_dn6 = assign6020_body5_e5727_d_n6;
            locals.var_temp2_dn7 = assign6020_body5_e5727_d_n7;
            locals.var_temp2_dn8 = assign6020_body5_e5727_d_n8;
            locals.var_temp2_dn9 = assign6020_body5_e5727_d_n9;
            locals.var_temp2_rv = 0.0;
            let (assign6020_body6_e5742, assign6020_body6_e5742_d_n4, assign6020_body6_e5742_d_n6, assign6020_body6_e5742_d_n7, assign6020_body6_e5742_d_n8, assign6020_body6_e5742_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6020_body6_e5737: f64 = (1.0 - locals.var_temp1);
        let assign6020_body6_e5739: f64 = (-p.p478);
        let assign6020_body6_e5740: f64 = (assign6020_body6_e5737).powf(assign6020_body6_e5739);
        (assign6020_body6_e5740, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn4))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn4) / assign6020_body6_e5737))) }, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn6))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn6) / assign6020_body6_e5737))) }, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn7))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn7) / assign6020_body6_e5737))) }, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn8))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn8) / assign6020_body6_e5737))) }, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn9))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn9) / assign6020_body6_e5737))) },)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
            locals.var_temp3 = assign6020_body6_e5742;
            locals.var_temp3_dn4 = assign6020_body6_e5742_d_n4;
            locals.var_temp3_dn6 = assign6020_body6_e5742_d_n6;
            locals.var_temp3_dn7 = assign6020_body6_e5742_d_n7;
            locals.var_temp3_dn8 = assign6020_body6_e5742_d_n8;
            locals.var_temp3_dn9 = assign6020_body6_e5742_d_n9;
            locals.var_temp3_rv = 0.0;
            let (assign6020_body7_e5757, assign6020_body7_e5757_d_n4, assign6020_body7_e5757_d_n6, assign6020_body7_e5757_d_n7, assign6020_body7_e5757_d_n8, assign6020_body7_e5757_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6020_body7_e5752: f64 = (1.0 - locals.var_temp2);
        let assign6020_body7_e5754: f64 = (-p.p478);
        let assign6020_body7_e5755: f64 = (assign6020_body7_e5752).powf(assign6020_body7_e5754);
        (assign6020_body7_e5755, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn4))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn4) / assign6020_body7_e5752))) }, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn6))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn6) / assign6020_body7_e5752))) }, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn7))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn7) / assign6020_body7_e5752))) }, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn8))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn8) / assign6020_body7_e5752))) }, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn9))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn9) / assign6020_body7_e5752))) },)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
            locals.var_temp4 = assign6020_body7_e5757;
            locals.var_temp4_dn4 = assign6020_body7_e5757_d_n4;
            locals.var_temp4_dn6 = assign6020_body7_e5757_d_n6;
            locals.var_temp4_dn7 = assign6020_body7_e5757_d_n7;
            locals.var_temp4_dn8 = assign6020_body7_e5757_d_n8;
            locals.var_temp4_dn9 = assign6020_body7_e5757_d_n9;
            locals.var_temp4_rv = 0.0;
            let (assign6020_body8_e5775, assign6020_body8_e5775_d_n4, assign6020_body8_e5775_d_n6, assign6020_body8_e5775_d_n7, assign6020_body8_e5775_d_n8, assign6020_body8_e5775_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6020_body8_e5769: f64 = (locals.var_temp3 + locals.var_temp4);
        let assign6020_body8_e5770: f64 = (0.5 * assign6020_body8_e5769);
        let assign6020_body8_e5772: f64 = (assign6020_body8_e5770).powf(locals.var_temp);
        let assign6020_body8_e5773: f64 = (locals.var_tmpa + assign6020_body8_e5772);
        (assign6020_body8_e5773, (locals.var_tmpa_dn4 + if locals.var_temp_dn4 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn4 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)) / assign6020_body8_e5770)))) }), (locals.var_tmpa_dn6 + if locals.var_temp_dn6 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn6 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)) / assign6020_body8_e5770)))) }), (locals.var_tmpa_dn7 + if locals.var_temp_dn7 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn7 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)) / assign6020_body8_e5770)))) }), (locals.var_tmpa_dn8 + if locals.var_temp_dn8 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn8 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)) / assign6020_body8_e5770)))) }), (locals.var_tmpa_dn9 + if locals.var_temp_dn9 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn9 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)) / assign6020_body8_e5770)))) }),)
    } else {
        (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9,)
    }
};
            locals.var_tmpa = assign6020_body8_e5775;
            locals.var_tmpa_dn4 = assign6020_body8_e5775_d_n4;
            locals.var_tmpa_dn6 = assign6020_body8_e5775_d_n6;
            locals.var_tmpa_dn7 = assign6020_body8_e5775_d_n7;
            locals.var_tmpa_dn8 = assign6020_body8_e5775_d_n8;
            locals.var_tmpa_dn9 = assign6020_body8_e5775_d_n9;
            locals.var_tmpa_rv = 0.0;
            let (assign6020_body9_e5787,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6020_body9_e5785: f64 = (locals.var_iloop + 1.0);
        (assign6020_body9_e5785,)
    } else {
        (locals.var_iloop,)
    }
};
            locals.var_iloop = assign6020_body9_e5787;
            locals.var_iloop_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6030_e5801, assign6030_e5801_d_n4, assign6030_e5801_d_n6, assign6030_e5801_d_n7, assign6030_e5801_d_n8, assign6030_e5801_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6030_e5798: f64 = (locals.var_tmpa / p.p29);
        let assign6030_e5799: f64 = (1.0 - assign6030_e5798);
        (assign6030_e5799, (-(locals.var_tmpa_dn4 / p.p29)), (-(locals.var_tmpa_dn6 / p.p29)), (-(locals.var_tmpa_dn7 / p.p29)), (-(locals.var_tmpa_dn8 / p.p29)), (-(locals.var_tmpa_dn9 / p.p29)),)
    } else {
        (locals.var_str_g, locals.var_str_g_dn4, locals.var_str_g_dn6, locals.var_str_g_dn7, locals.var_str_g_dn8, locals.var_str_g_dn9,)
    }
};
        locals.var_str_g = assign6030_e5801;
        locals.var_str_g_dn4 = assign6030_e5801_d_n4;
        locals.var_str_g_dn6 = assign6030_e5801_d_n6;
        locals.var_str_g_dn7 = assign6030_e5801_d_n7;
        locals.var_str_g_dn8 = assign6030_e5801_d_n8;
        locals.var_str_g_dn9 = assign6030_e5801_d_n9;
        locals.var_str_g_rv = 0.0;

        let assign6040_e5805: f64 = (0.5 * p.p20);
        let assign6040_e5806: f64 = (p.p458 + assign6040_e5805);
        let assign6040_e5807: f64 = (-assign6040_e5806);
        let assign6040_e5809: f64 = (assign6040_e5807 / p.p477);
        let assign6040_e5811: f64 = (-80.0);
        let assign6040_e5812: f64 = if assign6040_e5809 > assign6040_e5811 { 1.0 } else { 0.0 };
        locals.var_guard137 = assign6040_e5812;
        locals.var_guard137_rv = 0.0;

        let (assign6050_e5832, assign6050_e5832_d_n4, assign6050_e5832_d_n6, assign6050_e5832_d_n7, assign6050_e5832_d_n8, assign6050_e5832_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard137 != 0.0)) {
        let assign6050_e5825: f64 = (0.5 * p.p20);
        let assign6050_e5826: f64 = (p.p458 + assign6050_e5825);
        let assign6050_e5827: f64 = (-assign6050_e5826);
        let assign6050_e5829: f64 = (assign6050_e5827 / p.p477);
        let assign6050_e5830: f64 = (assign6050_e5829).exp();
        (assign6050_e5830, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign6050_e5832;
        locals.var_temp1_dn4 = assign6050_e5832_d_n4;
        locals.var_temp1_dn6 = assign6050_e5832_d_n6;
        locals.var_temp1_dn7 = assign6050_e5832_d_n7;
        locals.var_temp1_dn8 = assign6050_e5832_d_n8;
        locals.var_temp1_dn9 = assign6050_e5832_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign6060_e5891, assign6060_e5891_d_n4, assign6060_e5891_d_n6, assign6060_e5891_d_n7, assign6060_e5891_d_n8, assign6060_e5891_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard137 == 0.0)) {
        let assign6060_e5848: f64 = (0.5 * p.p20);
        let assign6060_e5849: f64 = (p.p458 + assign6060_e5848);
        let assign6060_e5850: f64 = (-assign6060_e5849);
        let assign6060_e5852: f64 = (assign6060_e5850 / p.p477);
        let assign6060_e5853: f64 = (-assign6060_e5852);
        let assign6060_e5855: f64 = (assign6060_e5853 - 80.0);
        let assign6060_e5861: f64 = (0.5 * p.p20);
        let assign6060_e5862: f64 = (p.p458 + assign6060_e5861);
        let assign6060_e5863: f64 = (-assign6060_e5862);
        let assign6060_e5865: f64 = (assign6060_e5863 / p.p477);
        let assign6060_e5866: f64 = (-assign6060_e5865);
        let assign6060_e5868: f64 = (assign6060_e5866 - 80.0);
        let assign6060_e5869: f64 = (0.5 * assign6060_e5868);
        let assign6060_e5874: f64 = (0.5 * p.p20);
        let assign6060_e5875: f64 = (p.p458 + assign6060_e5874);
        let assign6060_e5876: f64 = (-assign6060_e5875);
        let assign6060_e5878: f64 = (assign6060_e5876 / p.p477);
        let assign6060_e5879: f64 = (-assign6060_e5878);
        let assign6060_e5881: f64 = (assign6060_e5879 - 80.0);
        let assign6060_e5883: f64 = (assign6060_e5881 * 0.3333333333333);
        let assign6060_e5884: f64 = (1.0 + assign6060_e5883);
        let assign6060_e5885: f64 = (assign6060_e5869 * assign6060_e5884);
        let assign6060_e5886: f64 = (1.0 + assign6060_e5885);
        let assign6060_e5887: f64 = (assign6060_e5855 * assign6060_e5886);
        let assign6060_e5888: f64 = (1.0 + assign6060_e5887);
        let assign6060_e5889: f64 = (1.80485e-35 / assign6060_e5888);
        (assign6060_e5889, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign6060_e5891;
        locals.var_temp1_dn4 = assign6060_e5891_d_n4;
        locals.var_temp1_dn6 = assign6060_e5891_d_n6;
        locals.var_temp1_dn7 = assign6060_e5891_d_n7;
        locals.var_temp1_dn8 = assign6060_e5891_d_n8;
        locals.var_temp1_dn9 = assign6060_e5891_d_n9;
        locals.var_temp1_rv = 0.0;

        let assign6070_e5895: f64 = (0.5 * p.p20);
        let assign6070_e5896: f64 = (p.p459 + assign6070_e5895);
        let assign6070_e5897: f64 = (-assign6070_e5896);
        let assign6070_e5899: f64 = (assign6070_e5897 / p.p477);
        let assign6070_e5901: f64 = (-80.0);
        let assign6070_e5902: f64 = if assign6070_e5899 > assign6070_e5901 { 1.0 } else { 0.0 };
        locals.var_guard138 = assign6070_e5902;
        locals.var_guard138_rv = 0.0;

        let (assign6080_e5922, assign6080_e5922_d_n4, assign6080_e5922_d_n6, assign6080_e5922_d_n7, assign6080_e5922_d_n8, assign6080_e5922_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard138 != 0.0)) {
        let assign6080_e5915: f64 = (0.5 * p.p20);
        let assign6080_e5916: f64 = (p.p459 + assign6080_e5915);
        let assign6080_e5917: f64 = (-assign6080_e5916);
        let assign6080_e5919: f64 = (assign6080_e5917 / p.p477);
        let assign6080_e5920: f64 = (assign6080_e5919).exp();
        (assign6080_e5920, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign6080_e5922;
        locals.var_temp2_dn4 = assign6080_e5922_d_n4;
        locals.var_temp2_dn6 = assign6080_e5922_d_n6;
        locals.var_temp2_dn7 = assign6080_e5922_d_n7;
        locals.var_temp2_dn8 = assign6080_e5922_d_n8;
        locals.var_temp2_dn9 = assign6080_e5922_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign6090_e5981, assign6090_e5981_d_n4, assign6090_e5981_d_n6, assign6090_e5981_d_n7, assign6090_e5981_d_n8, assign6090_e5981_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard138 == 0.0)) {
        let assign6090_e5938: f64 = (0.5 * p.p20);
        let assign6090_e5939: f64 = (p.p459 + assign6090_e5938);
        let assign6090_e5940: f64 = (-assign6090_e5939);
        let assign6090_e5942: f64 = (assign6090_e5940 / p.p477);
        let assign6090_e5943: f64 = (-assign6090_e5942);
        let assign6090_e5945: f64 = (assign6090_e5943 - 80.0);
        let assign6090_e5951: f64 = (0.5 * p.p20);
        let assign6090_e5952: f64 = (p.p459 + assign6090_e5951);
        let assign6090_e5953: f64 = (-assign6090_e5952);
        let assign6090_e5955: f64 = (assign6090_e5953 / p.p477);
        let assign6090_e5956: f64 = (-assign6090_e5955);
        let assign6090_e5958: f64 = (assign6090_e5956 - 80.0);
        let assign6090_e5959: f64 = (0.5 * assign6090_e5958);
        let assign6090_e5964: f64 = (0.5 * p.p20);
        let assign6090_e5965: f64 = (p.p459 + assign6090_e5964);
        let assign6090_e5966: f64 = (-assign6090_e5965);
        let assign6090_e5968: f64 = (assign6090_e5966 / p.p477);
        let assign6090_e5969: f64 = (-assign6090_e5968);
        let assign6090_e5971: f64 = (assign6090_e5969 - 80.0);
        let assign6090_e5973: f64 = (assign6090_e5971 * 0.3333333333333);
        let assign6090_e5974: f64 = (1.0 + assign6090_e5973);
        let assign6090_e5975: f64 = (assign6090_e5959 * assign6090_e5974);
        let assign6090_e5976: f64 = (1.0 + assign6090_e5975);
        let assign6090_e5977: f64 = (assign6090_e5945 * assign6090_e5976);
        let assign6090_e5978: f64 = (1.0 + assign6090_e5977);
        let assign6090_e5979: f64 = (1.80485e-35 / assign6090_e5978);
        (assign6090_e5979, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign6090_e5981;
        locals.var_temp2_dn4 = assign6090_e5981_d_n4;
        locals.var_temp2_dn6 = assign6090_e5981_d_n6;
        locals.var_temp2_dn7 = assign6090_e5981_d_n7;
        locals.var_temp2_dn8 = assign6090_e5981_d_n8;
        locals.var_temp2_dn9 = assign6090_e5981_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign6100_e5996, assign6100_e5996_d_n4, assign6100_e5996_d_n6, assign6100_e5996_d_n7, assign6100_e5996_d_n8, assign6100_e5996_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6100_e5991: f64 = (1.0 - locals.var_temp1);
        let assign6100_e5993: f64 = (-p.p478);
        let assign6100_e5994: f64 = (assign6100_e5991).powf(assign6100_e5993);
        (assign6100_e5994, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn4))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn4) / assign6100_e5991))) }, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn6))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn6) / assign6100_e5991))) }, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn7))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn7) / assign6100_e5991))) }, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn8))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn8) / assign6100_e5991))) }, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn9))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn9) / assign6100_e5991))) },)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign6100_e5996;
        locals.var_temp3_dn4 = assign6100_e5996_d_n4;
        locals.var_temp3_dn6 = assign6100_e5996_d_n6;
        locals.var_temp3_dn7 = assign6100_e5996_d_n7;
        locals.var_temp3_dn8 = assign6100_e5996_d_n8;
        locals.var_temp3_dn9 = assign6100_e5996_d_n9;
        locals.var_temp3_rv = 0.0;

        let (assign6110_e6011, assign6110_e6011_d_n4, assign6110_e6011_d_n6, assign6110_e6011_d_n7, assign6110_e6011_d_n8, assign6110_e6011_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6110_e6006: f64 = (1.0 - locals.var_temp2);
        let assign6110_e6008: f64 = (-p.p478);
        let assign6110_e6009: f64 = (assign6110_e6006).powf(assign6110_e6008);
        (assign6110_e6009, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn4))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn4) / assign6110_e6006))) }, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn6))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn6) / assign6110_e6006))) }, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn7))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn7) / assign6110_e6006))) }, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn8))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn8) / assign6110_e6006))) }, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn9))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn9) / assign6110_e6006))) },)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign6110_e6011;
        locals.var_temp4_dn4 = assign6110_e6011_d_n4;
        locals.var_temp4_dn6 = assign6110_e6011_d_n6;
        locals.var_temp4_dn7 = assign6110_e6011_d_n7;
        locals.var_temp4_dn8 = assign6110_e6011_d_n8;
        locals.var_temp4_dn9 = assign6110_e6011_d_n9;
        locals.var_temp4_rv = 0.0;

        let (assign6120_e6029, assign6120_e6029_d_n4, assign6120_e6029_d_n6, assign6120_e6029_d_n7, assign6120_e6029_d_n8, assign6120_e6029_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6120_e6023: f64 = (locals.var_temp3 + locals.var_temp4);
        let assign6120_e6024: f64 = (0.5 * assign6120_e6023);
        let assign6120_e6026: f64 = (assign6120_e6024).powf(locals.var_temp);
        let assign6120_e6027: f64 = (1.0 - assign6120_e6026);
        (assign6120_e6027, (-if locals.var_temp_dn4 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn4 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)) / assign6120_e6024)))) }), (-if locals.var_temp_dn6 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn6 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)) / assign6120_e6024)))) }), (-if locals.var_temp_dn7 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn7 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)) / assign6120_e6024)))) }), (-if locals.var_temp_dn8 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn8 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)) / assign6120_e6024)))) }), (-if locals.var_temp_dn9 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn9 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)) / assign6120_e6024)))) }),)
    } else {
        (locals.var_str_gref, locals.var_str_gref_dn4, locals.var_str_gref_dn6, locals.var_str_gref_dn7, locals.var_str_gref_dn8, locals.var_str_gref_dn9,)
    }
};
        locals.var_str_gref = assign6120_e6029;
        locals.var_str_gref_dn4 = assign6120_e6029_d_n4;
        locals.var_str_gref_dn6 = assign6120_e6029_d_n6;
        locals.var_str_gref_dn7 = assign6120_e6029_d_n7;
        locals.var_str_gref_dn8 = assign6120_e6029_d_n8;
        locals.var_str_gref_dn9 = assign6120_e6029_d_n9;
        locals.var_str_gref_rv = 0.0;

        let (assign6130_e6045,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6130_e6039: f64 = (locals.var_w_i + locals.var_delwod);
        let assign6130_e6041: f64 = (assign6130_e6039 + p.p460);
        let assign6130_e6043: f64 = (assign6130_e6041).max(1e-9);
        (assign6130_e6043,)
    } else {
        (locals.var_wx,)
    }
};
        locals.var_wx = assign6130_e6045;
        locals.var_wx_rv = 0.0;

        let (assign6140_e6063, assign6140_e6063_d_n4, assign6140_e6063_d_n6, assign6140_e6063_d_n7, assign6140_e6063_d_n8, assign6140_e6063_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6140_e6058: f64 = (locals.var_rt - 1.0);
        let assign6140_e6059: f64 = (p.p483 * assign6140_e6058);
        let assign6140_e6060: f64 = (1.0 + assign6140_e6059);
        let assign6140_e6061: f64 = (p.p482 / assign6140_e6060);
        (assign6140_e6061, (-((p.p482 * (p.p483 * locals.var_rt_dn4)) / (assign6140_e6060 * assign6140_e6060))), (-((p.p482 * (p.p483 * locals.var_rt_dn6)) / (assign6140_e6060 * assign6140_e6060))), (-((p.p482 * (p.p483 * locals.var_rt_dn7)) / (assign6140_e6060 * assign6140_e6060))), (-((p.p482 * (p.p483 * locals.var_rt_dn8)) / (assign6140_e6060 * assign6140_e6060))), (-((p.p482 * (p.p483 * locals.var_rt_dn9)) / (assign6140_e6060 * assign6140_e6060))),)
    } else {
        (locals.var_ruo, locals.var_ruo_dn4, locals.var_ruo_dn6, locals.var_ruo_dn7, locals.var_ruo_dn8, locals.var_ruo_dn9,)
    }
};
        locals.var_ruo = assign6140_e6063;
        locals.var_ruo_dn4 = assign6140_e6063_d_n4;
        locals.var_ruo_dn6 = assign6140_e6063_d_n6;
        locals.var_ruo_dn7 = assign6140_e6063_d_n7;
        locals.var_ruo_dn8 = assign6140_e6063_d_n8;
        locals.var_ruo_dn9 = assign6140_e6063_d_n9;
        locals.var_ruo_rv = 0.0;

        let (assign6150_e6075, assign6150_e6075_d_n4, assign6150_e6075_d_n6, assign6150_e6075_d_n7, assign6150_e6075_d_n8, assign6150_e6075_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6150_e6073: f64 = (locals.var_ruo * locals.var_str_g);
        (assign6150_e6073, ((locals.var_ruo_dn4 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn4)), ((locals.var_ruo_dn6 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn6)), ((locals.var_ruo_dn7 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn7)), ((locals.var_ruo_dn8 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn8)), ((locals.var_ruo_dn9 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn9)),)
    } else {
        (locals.var_rhobeta, locals.var_rhobeta_dn4, locals.var_rhobeta_dn6, locals.var_rhobeta_dn7, locals.var_rhobeta_dn8, locals.var_rhobeta_dn9,)
    }
};
        locals.var_rhobeta = assign6150_e6075;
        locals.var_rhobeta_dn4 = assign6150_e6075_d_n4;
        locals.var_rhobeta_dn6 = assign6150_e6075_d_n6;
        locals.var_rhobeta_dn7 = assign6150_e6075_d_n7;
        locals.var_rhobeta_dn8 = assign6150_e6075_d_n8;
        locals.var_rhobeta_dn9 = assign6150_e6075_d_n9;
        locals.var_rhobeta_rv = 0.0;

        let (assign6160_e6087, assign6160_e6087_d_n4, assign6160_e6087_d_n6, assign6160_e6087_d_n7, assign6160_e6087_d_n8, assign6160_e6087_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6160_e6085: f64 = (locals.var_ruo * locals.var_str_gref);
        (assign6160_e6085, ((locals.var_ruo_dn4 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn4)), ((locals.var_ruo_dn6 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn6)), ((locals.var_ruo_dn7 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn7)), ((locals.var_ruo_dn8 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn8)), ((locals.var_ruo_dn9 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn9)),)
    } else {
        (locals.var_rhobetaref, locals.var_rhobetaref_dn4, locals.var_rhobetaref_dn6, locals.var_rhobetaref_dn7, locals.var_rhobetaref_dn8, locals.var_rhobetaref_dn9,)
    }
};
        locals.var_rhobetaref = assign6160_e6087;
        locals.var_rhobetaref_dn4 = assign6160_e6087_d_n4;
        locals.var_rhobetaref_dn6 = assign6160_e6087_d_n6;
        locals.var_rhobetaref_dn7 = assign6160_e6087_d_n7;
        locals.var_rhobetaref_dn8 = assign6160_e6087_d_n8;
        locals.var_rhobetaref_dn9 = assign6160_e6087_d_n9;
        locals.var_rhobetaref_rv = 0.0;

        let (assign6170_e6099, assign6170_e6099_d_n4, assign6170_e6099_d_n6, assign6170_e6099_d_n7, assign6170_e6099_d_n8, assign6170_e6099_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6170_e6097: f64 = (locals.var_str_g - locals.var_str_gref);
        (assign6170_e6097, (locals.var_str_g_dn4 - locals.var_str_gref_dn4), (locals.var_str_g_dn6 - locals.var_str_gref_dn6), (locals.var_str_g_dn7 - locals.var_str_gref_dn7), (locals.var_str_g_dn8 - locals.var_str_gref_dn8), (locals.var_str_g_dn9 - locals.var_str_gref_dn9),)
    } else {
        (locals.var_temp0__blk79, locals.var_temp0__blk79_dn4, locals.var_temp0__blk79_dn6, locals.var_temp0__blk79_dn7, locals.var_temp0__blk79_dn8, locals.var_temp0__blk79_dn9,)
    }
};
        locals.var_temp0__blk79 = assign6170_e6099;
        locals.var_temp0__blk79_dn4 = assign6170_e6099_d_n4;
        locals.var_temp0__blk79_dn6 = assign6170_e6099_d_n6;
        locals.var_temp0__blk79_dn7 = assign6170_e6099_d_n7;
        locals.var_temp0__blk79_dn8 = assign6170_e6099_d_n8;
        locals.var_temp0__blk79_dn9 = assign6170_e6099_d_n9;
        locals.var_temp0__blk79_rv = 0.0;

        let (assign6180_e6117,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6180_e6110: f64 = (p.p480 * locals.var_wx);
        let assign6180_e6112: f64 = (assign6180_e6110 / locals.var_wen);
        let assign6180_e6113: f64 = (1.0 + assign6180_e6112);
        let assign6180_e6115: f64 = (assign6180_e6113).max(1e-20);
        (assign6180_e6115,)
    } else {
        (locals.var_kstressvth0,)
    }
};
        locals.var_kstressvth0 = assign6180_e6117;
        locals.var_kstressvth0_rv = 0.0;

        let (assign6190_e6135, assign6190_e6135_d_n4, assign6190_e6135_d_n6, assign6190_e6135_d_n7, assign6190_e6135_d_n8, assign6190_e6135_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6190_e6128: f64 = (1.0 + locals.var_rhobeta);
        let assign6190_e6129: f64 = (locals.var_betn_p * assign6190_e6128);
        let assign6190_e6132: f64 = (1.0 + locals.var_rhobetaref);
        let assign6190_e6133: f64 = (assign6190_e6129 / assign6190_e6132);
        (assign6190_e6133, (((((locals.var_betn_p_dn4 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn4)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn4)) / (assign6190_e6132 * assign6190_e6132)), (((((locals.var_betn_p_dn6 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn6)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn6)) / (assign6190_e6132 * assign6190_e6132)), (((((locals.var_betn_p_dn7 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn7)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn7)) / (assign6190_e6132 * assign6190_e6132)), (((((locals.var_betn_p_dn8 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn8)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn8)) / (assign6190_e6132 * assign6190_e6132)), (((((locals.var_betn_p_dn9 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn9)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn9)) / (assign6190_e6132 * assign6190_e6132)),)
    } else {
        (locals.var_betn_p, locals.var_betn_p_dn4, locals.var_betn_p_dn6, locals.var_betn_p_dn7, locals.var_betn_p_dn8, locals.var_betn_p_dn9,)
    }
};
        locals.var_betn_p = assign6190_e6135;
        locals.var_betn_p_dn4 = assign6190_e6135_d_n4;
        locals.var_betn_p_dn6 = assign6190_e6135_d_n6;
        locals.var_betn_p_dn7 = assign6190_e6135_d_n7;
        locals.var_betn_p_dn8 = assign6190_e6135_d_n8;
        locals.var_betn_p_dn9 = assign6190_e6135_d_n9;
        locals.var_betn_p_rv = 0.0;

        let (assign6200_e6147, assign6200_e6147_d_n4, assign6200_e6147_d_n6, assign6200_e6147_d_n7, assign6200_e6147_d_n8, assign6200_e6147_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6200_e6145: f64 = (locals.var_betn_p).max(1e-10);
        (assign6200_e6145, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn4 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn6 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn7 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn8 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9,)
    }
};
        locals.var_betn1_t = assign6200_e6147;
        locals.var_betn1_t_dn4 = assign6200_e6147_d_n4;
        locals.var_betn1_t_dn6 = assign6200_e6147_d_n6;
        locals.var_betn1_t_dn7 = assign6200_e6147_d_n7;
        locals.var_betn1_t_dn8 = assign6200_e6147_d_n8;
        locals.var_betn1_t_dn9 = assign6200_e6147_d_n9;
        locals.var_betn1_t_rv = 0.0;

        let (assign6210_e6159, assign6210_e6159_d_n4, assign6210_e6159_d_n6, assign6210_e6159_d_n7, assign6210_e6159_d_n8, assign6210_e6159_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6210_e6157: f64 = (p.p250 * locals.var_betn1_t);
        (assign6210_e6157, (p.p250 * locals.var_betn1_t_dn4), (p.p250 * locals.var_betn1_t_dn6), (p.p250 * locals.var_betn1_t_dn7), (p.p250 * locals.var_betn1_t_dn8), (p.p250 * locals.var_betn1_t_dn9),)
    } else {
        (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9,)
    }
};
        locals.var_betn2_t = assign6210_e6159;
        locals.var_betn2_t_dn4 = assign6210_e6159_d_n4;
        locals.var_betn2_t_dn6 = assign6210_e6159_d_n6;
        locals.var_betn2_t_dn7 = assign6210_e6159_d_n7;
        locals.var_betn2_t_dn8 = assign6210_e6159_d_n8;
        locals.var_betn2_t_dn9 = assign6210_e6159_d_n9;
        locals.var_betn2_t_rv = 0.0;

        let (assign6220_e6187, assign6220_e6187_d_n4, assign6220_e6187_d_n6, assign6220_e6187_d_n7, assign6220_e6187_d_n8, assign6220_e6187_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6220_e6169: f64 = (1.0 + locals.var_rhobeta);
        let assign6220_e6173: f64 = (p.p484 * locals.var_rhobetaref);
        let assign6220_e6174: f64 = (1.0 + assign6220_e6173);
        let assign6220_e6175: f64 = (assign6220_e6169 * assign6220_e6174);
        let assign6220_e6178: f64 = (1.0 + locals.var_rhobetaref);
        let assign6220_e6182: f64 = (p.p484 * locals.var_rhobeta);
        let assign6220_e6183: f64 = (1.0 + assign6220_e6182);
        let assign6220_e6184: f64 = (assign6220_e6178 * assign6220_e6183);
        let assign6220_e6185: f64 = (assign6220_e6175 / assign6220_e6184);
        (assign6220_e6185, (((((locals.var_rhobeta_dn4 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn4))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn4 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn4))))) / (assign6220_e6184 * assign6220_e6184)), (((((locals.var_rhobeta_dn6 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn6))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn6 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn6))))) / (assign6220_e6184 * assign6220_e6184)), (((((locals.var_rhobeta_dn7 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn7))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn7 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn7))))) / (assign6220_e6184 * assign6220_e6184)), (((((locals.var_rhobeta_dn8 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn8))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn8 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn8))))) / (assign6220_e6184 * assign6220_e6184)), (((((locals.var_rhobeta_dn9 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn9))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn9 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn9))))) / (assign6220_e6184 * assign6220_e6184)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign6220_e6187;
        locals.var_temp_dn4 = assign6220_e6187_d_n4;
        locals.var_temp_dn6 = assign6220_e6187_d_n6;
        locals.var_temp_dn7 = assign6220_e6187_d_n7;
        locals.var_temp_dn8 = assign6220_e6187_d_n8;
        locals.var_temp_dn9 = assign6220_e6187_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign6230_e6199, assign6230_e6199_d_n4, assign6230_e6199_d_n6, assign6230_e6199_d_n7, assign6230_e6199_d_n8, assign6230_e6199_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6230_e6197: f64 = (locals.var_thesat_p * locals.var_temp);
        (assign6230_e6197, ((locals.var_thesat_p_dn4 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn4)), ((locals.var_thesat_p_dn6 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn6)), ((locals.var_thesat_p_dn7 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn7)), ((locals.var_thesat_p_dn8 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn8)), ((locals.var_thesat_p_dn9 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn9)),)
    } else {
        (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9,)
    }
};
        locals.var_thesat_p = assign6230_e6199;
        locals.var_thesat_p_dn4 = assign6230_e6199_d_n4;
        locals.var_thesat_p_dn6 = assign6230_e6199_d_n6;
        locals.var_thesat_p_dn7 = assign6230_e6199_d_n7;
        locals.var_thesat_p_dn8 = assign6230_e6199_d_n8;
        locals.var_thesat_p_dn9 = assign6230_e6199_d_n9;
        locals.var_thesat_p_rv = 0.0;

        let (assign6240_e6211, assign6240_e6211_d_n4, assign6240_e6211_d_n6, assign6240_e6211_d_n7, assign6240_e6211_d_n8, assign6240_e6211_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6240_e6209: f64 = (locals.var_thesat_p).max(0.0);
        (assign6240_e6209, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn4 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn6 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn7 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn8 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9,)
    }
};
        locals.var_thesat_t = assign6240_e6211;
        locals.var_thesat_t_dn4 = assign6240_e6211_d_n4;
        locals.var_thesat_t_dn6 = assign6240_e6211_d_n6;
        locals.var_thesat_t_dn7 = assign6240_e6211_d_n7;
        locals.var_thesat_t_dn8 = assign6240_e6211_d_n8;
        locals.var_thesat_t_dn9 = assign6240_e6211_d_n9;
        locals.var_thesat_t_rv = 0.0;

        let (assign6250_e6223, assign6250_e6223_d_n4, assign6250_e6223_d_n6, assign6250_e6223_d_n7, assign6250_e6223_d_n8, assign6250_e6223_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6250_e6221: f64 = (locals.var_thesatac_p * locals.var_temp);
        (assign6250_e6221, ((locals.var_thesatac_p_dn4 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn4)), ((locals.var_thesatac_p_dn6 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn6)), ((locals.var_thesatac_p_dn7 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn7)), ((locals.var_thesatac_p_dn8 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn8)), ((locals.var_thesatac_p_dn9 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn9)),)
    } else {
        (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9,)
    }
};
        locals.var_thesatac_p = assign6250_e6223;
        locals.var_thesatac_p_dn4 = assign6250_e6223_d_n4;
        locals.var_thesatac_p_dn6 = assign6250_e6223_d_n6;
        locals.var_thesatac_p_dn7 = assign6250_e6223_d_n7;
        locals.var_thesatac_p_dn8 = assign6250_e6223_d_n8;
        locals.var_thesatac_p_dn9 = assign6250_e6223_d_n9;
        locals.var_thesatac_p_rv = 0.0;

        let (assign6260_e6235, assign6260_e6235_d_n4, assign6260_e6235_d_n6, assign6260_e6235_d_n7, assign6260_e6235_d_n8, assign6260_e6235_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6260_e6233: f64 = (locals.var_thesatac_p).max(0.0);
        (assign6260_e6233, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn4 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn6 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn7 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn8 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9,)
    }
};
        locals.var_thesatac_t = assign6260_e6235;
        locals.var_thesatac_t_dn4 = assign6260_e6235_d_n4;
        locals.var_thesatac_t_dn6 = assign6260_e6235_d_n6;
        locals.var_thesatac_t_dn7 = assign6260_e6235_d_n7;
        locals.var_thesatac_t_dn8 = assign6260_e6235_d_n8;
        locals.var_thesatac_t_dn9 = assign6260_e6235_d_n9;
        locals.var_thesatac_t_rv = 0.0;

        let (assign6270_e6249, assign6270_e6249_d_n4, assign6270_e6249_d_n6, assign6270_e6249_d_n7, assign6270_e6249_d_n8, assign6270_e6249_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6270_e6245: f64 = (p.p479 * locals.var_temp0__blk79);
        let assign6270_e6247: f64 = (assign6270_e6245 / locals.var_kstressvth0);
        (assign6270_e6247, ((p.p479 * locals.var_temp0__blk79_dn4) / locals.var_kstressvth0), ((p.p479 * locals.var_temp0__blk79_dn6) / locals.var_kstressvth0), ((p.p479 * locals.var_temp0__blk79_dn7) / locals.var_kstressvth0), ((p.p479 * locals.var_temp0__blk79_dn8) / locals.var_kstressvth0), ((p.p479 * locals.var_temp0__blk79_dn9) / locals.var_kstressvth0),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign6270_e6249;
        locals.var_temp_dn4 = assign6270_e6249_d_n4;
        locals.var_temp_dn6 = assign6270_e6249_d_n6;
        locals.var_temp_dn7 = assign6270_e6249_d_n7;
        locals.var_temp_dn8 = assign6270_e6249_d_n8;
        locals.var_temp_dn9 = assign6270_e6249_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign6280_e6261, assign6280_e6261_d_n4, assign6280_e6261_d_n6, assign6280_e6261_d_n7, assign6280_e6261_d_n8, assign6280_e6261_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6280_e6259: f64 = (locals.var_vfb1_t + locals.var_temp);
        (assign6280_e6259, (locals.var_vfb1_t_dn4 + locals.var_temp_dn4), (locals.var_vfb1_t_dn6 + locals.var_temp_dn6), (locals.var_vfb1_t_dn7 + locals.var_temp_dn7), (locals.var_vfb1_t_dn8 + locals.var_temp_dn8), (locals.var_vfb1_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9,)
    }
};
        locals.var_vfb1_t = assign6280_e6261;
        locals.var_vfb1_t_dn4 = assign6280_e6261_d_n4;
        locals.var_vfb1_t_dn6 = assign6280_e6261_d_n6;
        locals.var_vfb1_t_dn7 = assign6280_e6261_d_n7;
        locals.var_vfb1_t_dn8 = assign6280_e6261_d_n8;
        locals.var_vfb1_t_dn9 = assign6280_e6261_d_n9;
        locals.var_vfb1_t_rv = 0.0;

        let (assign6290_e6273, assign6290_e6273_d_n4, assign6290_e6273_d_n6, assign6290_e6273_d_n7, assign6290_e6273_d_n8, assign6290_e6273_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6290_e6271: f64 = (locals.var_vfb2_t + locals.var_temp);
        (assign6290_e6271, (locals.var_vfb2_t_dn4 + locals.var_temp_dn4), (locals.var_vfb2_t_dn6 + locals.var_temp_dn6), (locals.var_vfb2_t_dn7 + locals.var_temp_dn7), (locals.var_vfb2_t_dn8 + locals.var_temp_dn8), (locals.var_vfb2_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    }
};
        locals.var_vfb2_t = assign6290_e6273;
        locals.var_vfb2_t_dn4 = assign6290_e6273_d_n4;
        locals.var_vfb2_t_dn6 = assign6290_e6273_d_n6;
        locals.var_vfb2_t_dn7 = assign6290_e6273_d_n7;
        locals.var_vfb2_t_dn8 = assign6290_e6273_d_n8;
        locals.var_vfb2_t_dn9 = assign6290_e6273_d_n9;
        locals.var_vfb2_t_rv = 0.0;

        let (assign6300_e6285, assign6300_e6285_d_n4, assign6300_e6285_d_n6, assign6300_e6285_d_n7, assign6300_e6285_d_n8, assign6300_e6285_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6300_e6283: f64 = (locals.var_vfbac1_t + locals.var_temp);
        (assign6300_e6283, (locals.var_vfbac1_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac1_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac1_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac1_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac1_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9,)
    }
};
        locals.var_vfbac1_t = assign6300_e6285;
        locals.var_vfbac1_t_dn4 = assign6300_e6285_d_n4;
        locals.var_vfbac1_t_dn6 = assign6300_e6285_d_n6;
        locals.var_vfbac1_t_dn7 = assign6300_e6285_d_n7;
        locals.var_vfbac1_t_dn8 = assign6300_e6285_d_n8;
        locals.var_vfbac1_t_dn9 = assign6300_e6285_d_n9;
        locals.var_vfbac1_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6310_e6297, assign6310_e6297_d_n4, assign6310_e6297_d_n6, assign6310_e6297_d_n7, assign6310_e6297_d_n8, assign6310_e6297_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6310_e6295: f64 = (locals.var_vfbac2_t + locals.var_temp);
        (assign6310_e6295, (locals.var_vfbac2_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac2_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac2_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac2_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac2_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign6310_e6297;
        locals.var_vfbac2_t_dn4 = assign6310_e6297_d_n4;
        locals.var_vfbac2_t_dn6 = assign6310_e6297_d_n6;
        locals.var_vfbac2_t_dn7 = assign6310_e6297_d_n7;
        locals.var_vfbac2_t_dn8 = assign6310_e6297_d_n8;
        locals.var_vfbac2_t_dn9 = assign6310_e6297_d_n9;
        locals.var_vfbac2_t_rv = 0.0;

        let (assign6320_e6319, assign6320_e6319_d_n4, assign6320_e6319_d_n6, assign6320_e6319_d_n7, assign6320_e6319_d_n8, assign6320_e6319_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6320_e6307: f64 = (p.p481 * locals.var_temp0__blk79);
        let assign6320_e6310: f64 = (locals.var_lambda_le).powf(p.p232);
        let assign6320_e6311: f64 = (assign6320_e6307 * assign6320_e6310);
        let assign6320_e6315: f64 = (p.p233 * locals.var_iwe);
        let assign6320_e6316: f64 = (1.0 + assign6320_e6315);
        let assign6320_e6317: f64 = (assign6320_e6311 * assign6320_e6316);
        (assign6320_e6317, (((p.p481 * locals.var_temp0__blk79_dn4) * assign6320_e6310) * assign6320_e6316), (((p.p481 * locals.var_temp0__blk79_dn6) * assign6320_e6310) * assign6320_e6316), (((p.p481 * locals.var_temp0__blk79_dn7) * assign6320_e6310) * assign6320_e6316), (((p.p481 * locals.var_temp0__blk79_dn8) * assign6320_e6310) * assign6320_e6316), (((p.p481 * locals.var_temp0__blk79_dn9) * assign6320_e6310) * assign6320_e6316),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign6320_e6319;
        locals.var_temp_dn4 = assign6320_e6319_d_n4;
        locals.var_temp_dn6 = assign6320_e6319_d_n6;
        locals.var_temp_dn7 = assign6320_e6319_d_n7;
        locals.var_temp_dn8 = assign6320_e6319_d_n8;
        locals.var_temp_dn9 = assign6320_e6319_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign6330_e6331, assign6330_e6331_d_n4, assign6330_e6331_d_n6, assign6330_e6331_d_n7, assign6330_e6331_d_n8, assign6330_e6331_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6330_e6329: f64 = (locals.var_cf_p + locals.var_temp);
        (assign6330_e6329, (locals.var_cf_p_dn4 + locals.var_temp_dn4), (locals.var_cf_p_dn6 + locals.var_temp_dn6), (locals.var_cf_p_dn7 + locals.var_temp_dn7), (locals.var_cf_p_dn8 + locals.var_temp_dn8), (locals.var_cf_p_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9,)
    }
};
        locals.var_cf_p = assign6330_e6331;
        locals.var_cf_p_dn4 = assign6330_e6331_d_n4;
        locals.var_cf_p_dn6 = assign6330_e6331_d_n6;
        locals.var_cf_p_dn7 = assign6330_e6331_d_n7;
        locals.var_cf_p_dn8 = assign6330_e6331_d_n8;
        locals.var_cf_p_dn9 = assign6330_e6331_d_n9;
        locals.var_cf_p_rv = 0.0;

        let (assign6340_e6343, assign6340_e6343_d_n4, assign6340_e6343_d_n6, assign6340_e6343_d_n7, assign6340_e6343_d_n8, assign6340_e6343_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6340_e6341: f64 = (locals.var_cf_p).max(0.0);
        (assign6340_e6341, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn4 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn6 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn7 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn8 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9,)
    }
};
        locals.var_cf1_t = assign6340_e6343;
        locals.var_cf1_t_dn4 = assign6340_e6343_d_n4;
        locals.var_cf1_t_dn6 = assign6340_e6343_d_n6;
        locals.var_cf1_t_dn7 = assign6340_e6343_d_n7;
        locals.var_cf1_t_dn8 = assign6340_e6343_d_n8;
        locals.var_cf1_t_dn9 = assign6340_e6343_d_n9;
        locals.var_cf1_t_rv = 0.0;

        let (assign6350_e6355, assign6350_e6355_d_n4, assign6350_e6355_d_n6, assign6350_e6355_d_n7, assign6350_e6355_d_n8, assign6350_e6355_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6350_e6353: f64 = (locals.var_cfac_p + locals.var_temp);
        (assign6350_e6353, (locals.var_cfac_p_dn4 + locals.var_temp_dn4), (locals.var_cfac_p_dn6 + locals.var_temp_dn6), (locals.var_cfac_p_dn7 + locals.var_temp_dn7), (locals.var_cfac_p_dn8 + locals.var_temp_dn8), (locals.var_cfac_p_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9,)
    }
};
        locals.var_cfac_p = assign6350_e6355;
        locals.var_cfac_p_dn4 = assign6350_e6355_d_n4;
        locals.var_cfac_p_dn6 = assign6350_e6355_d_n6;
        locals.var_cfac_p_dn7 = assign6350_e6355_d_n7;
        locals.var_cfac_p_dn8 = assign6350_e6355_d_n8;
        locals.var_cfac_p_dn9 = assign6350_e6355_d_n9;
        locals.var_cfac_p_rv = 0.0;

        let (assign6360_e6367, assign6360_e6367_d_n4, assign6360_e6367_d_n6, assign6360_e6367_d_n7, assign6360_e6367_d_n8, assign6360_e6367_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6360_e6365: f64 = (locals.var_cfac_p).max(0.0);
        (assign6360_e6365, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn4 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn6 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn7 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn8 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9,)
    }
};
        locals.var_cfac1_t = assign6360_e6367;
        locals.var_cfac1_t_dn4 = assign6360_e6367_d_n4;
        locals.var_cfac1_t_dn6 = assign6360_e6367_d_n6;
        locals.var_cfac1_t_dn7 = assign6360_e6367_d_n7;
        locals.var_cfac1_t_dn8 = assign6360_e6367_d_n8;
        locals.var_cfac1_t_dn9 = assign6360_e6367_d_n9;
        locals.var_cfac1_t_rv = 0.0;

        let (assign6370_e6381, assign6370_e6381_d_n4, assign6370_e6381_d_n6, assign6370_e6381_d_n7, assign6370_e6381_d_n8, assign6370_e6381_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6370_e6377: f64 = (p.p234 * locals.var_tox2_i);
        let assign6370_e6379: f64 = (assign6370_e6377 / locals.var_tox1_i);
        (assign6370_e6379, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign6370_e6381;
        locals.var_temp_dn4 = assign6370_e6381_d_n4;
        locals.var_temp_dn6 = assign6370_e6381_d_n6;
        locals.var_temp_dn7 = assign6370_e6381_d_n7;
        locals.var_temp_dn8 = assign6370_e6381_d_n8;
        locals.var_temp_dn9 = assign6370_e6381_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign6380_e6393, assign6380_e6393_d_n4, assign6380_e6393_d_n6, assign6380_e6393_d_n7, assign6380_e6393_d_n8, assign6380_e6393_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6380_e6391: f64 = (locals.var_cf1_t * locals.var_temp);
        (assign6380_e6391, ((locals.var_cf1_t_dn4 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn4)), ((locals.var_cf1_t_dn6 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn6)), ((locals.var_cf1_t_dn7 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn7)), ((locals.var_cf1_t_dn8 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn8)), ((locals.var_cf1_t_dn9 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn9)),)
    } else {
        (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9,)
    }
};
        locals.var_cf2_t = assign6380_e6393;
        locals.var_cf2_t_dn4 = assign6380_e6393_d_n4;
        locals.var_cf2_t_dn6 = assign6380_e6393_d_n6;
        locals.var_cf2_t_dn7 = assign6380_e6393_d_n7;
        locals.var_cf2_t_dn8 = assign6380_e6393_d_n8;
        locals.var_cf2_t_dn9 = assign6380_e6393_d_n9;
        locals.var_cf2_t_rv = 0.0;

        let (assign6390_e6405, assign6390_e6405_d_n4, assign6390_e6405_d_n6, assign6390_e6405_d_n7, assign6390_e6405_d_n8, assign6390_e6405_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6390_e6403: f64 = (locals.var_cfac1_t * locals.var_temp);
        (assign6390_e6403, ((locals.var_cfac1_t_dn4 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn4)), ((locals.var_cfac1_t_dn6 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn6)), ((locals.var_cfac1_t_dn7 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn7)), ((locals.var_cfac1_t_dn8 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn8)), ((locals.var_cfac1_t_dn9 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn9)),)
    } else {
        (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9,)
    }
};
        locals.var_cfac2_t = assign6390_e6405;
        locals.var_cfac2_t_dn4 = assign6390_e6405_d_n4;
        locals.var_cfac2_t_dn6 = assign6390_e6405_d_n6;
        locals.var_cfac2_t_dn7 = assign6390_e6405_d_n7;
        locals.var_cfac2_t_dn8 = assign6390_e6405_d_n8;
        locals.var_cfac2_t_dn9 = assign6390_e6405_d_n9;
        locals.var_cfac2_t_rv = 0.0;

        let assign6400_e6408: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard139 = assign6400_e6408;
        locals.var_guard139_rv = 0.0;

        let (assign6410_e6412,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_nov_i,)
    } else {
        (locals.var_novd_i,)
    }
};
        locals.var_novd_i = assign6410_e6412;
        locals.var_novd_i_rv = 0.0;

        let (assign6420_e6416,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_igovinv_t,)
    } else {
        (locals.var_igovinvd_t,)
    }
};
        locals.var_igovinvd_t = assign6420_e6416;
        locals.var_igovinvd_t_rv = 0.0;

        let (assign6440_e6424,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_igovacc_t,)
    } else {
        (locals.var_igovaccd_t,)
    }
};
        locals.var_igovaccd_t = assign6440_e6424;
        locals.var_igovaccd_t_rv = 0.0;

        let (assign6450_e6428, assign6450_e6428_d_n4, assign6450_e6428_d_n6, assign6450_e6428_d_n7, assign6450_e6428_d_n8, assign6450_e6428_d_n9,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_agidl_i, locals.var_agidl_i_dn4, locals.var_agidl_i_dn6, locals.var_agidl_i_dn7, locals.var_agidl_i_dn8, locals.var_agidl_i_dn9,)
    } else {
        (locals.var_agidld_i, locals.var_agidld_i_dn4, locals.var_agidld_i_dn6, locals.var_agidld_i_dn7, locals.var_agidld_i_dn8, locals.var_agidld_i_dn9,)
    }
};
        locals.var_agidld_i = assign6450_e6428;
        locals.var_agidld_i_dn4 = assign6450_e6428_d_n4;
        locals.var_agidld_i_dn6 = assign6450_e6428_d_n6;
        locals.var_agidld_i_dn7 = assign6450_e6428_d_n7;
        locals.var_agidld_i_dn8 = assign6450_e6428_d_n8;
        locals.var_agidld_i_dn9 = assign6450_e6428_d_n9;
        locals.var_agidld_i_rv = 0.0;

        let (assign6460_e6432,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_bgidl_t,)
    } else {
        (locals.var_bgidld_t,)
    }
};
        locals.var_bgidld_t = assign6460_e6432;
        locals.var_bgidld_t_rv = 0.0;

        let (assign6470_e6436,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_stbgidl_i,)
    } else {
        (locals.var_stbgidld_i,)
    }
};
        locals.var_stbgidld_i = assign6470_e6436;
        locals.var_stbgidld_i_rv = 0.0;

        let (assign6480_e6440,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_cgidl_i,)
    } else {
        (locals.var_cgidld_i,)
    }
};
        locals.var_cgidld_i = assign6480_e6440;
        locals.var_cgidld_i_rv = 0.0;

        let (assign6490_e6444,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_dgidl_i,)
    } else {
        (locals.var_dgidld_i,)
    }
};
        locals.var_dgidld_i = assign6490_e6444;
        locals.var_dgidld_i_rv = 0.0;

        let (assign6500_e6448, assign6500_e6448_d_n4, assign6500_e6448_d_n6, assign6500_e6448_d_n7, assign6500_e6448_d_n8, assign6500_e6448_d_n9,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_cov_i, locals.var_cov_i_dn4, locals.var_cov_i_dn6, locals.var_cov_i_dn7, locals.var_cov_i_dn8, locals.var_cov_i_dn9,)
    } else {
        (locals.var_covd_i, locals.var_covd_i_dn4, locals.var_covd_i_dn6, locals.var_covd_i_dn7, locals.var_covd_i_dn8, locals.var_covd_i_dn9,)
    }
};
        locals.var_covd_i = assign6500_e6448;
        locals.var_covd_i_dn4 = assign6500_e6448_d_n4;
        locals.var_covd_i_dn6 = assign6500_e6448_d_n6;
        locals.var_covd_i_dn7 = assign6500_e6448_d_n7;
        locals.var_covd_i_dn8 = assign6500_e6448_d_n8;
        locals.var_covd_i_dn9 = assign6500_e6448_d_n9;
        locals.var_covd_i_rv = 0.0;

        let (assign6510_e6452, assign6510_e6452_d_n4, assign6510_e6452_d_n6, assign6510_e6452_d_n7, assign6510_e6452_d_n8, assign6510_e6452_d_n9,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_cfr_i, locals.var_cfr_i_dn4, locals.var_cfr_i_dn6, locals.var_cfr_i_dn7, locals.var_cfr_i_dn8, locals.var_cfr_i_dn9,)
    } else {
        (locals.var_cfrd_i, locals.var_cfrd_i_dn4, locals.var_cfrd_i_dn6, locals.var_cfrd_i_dn7, locals.var_cfrd_i_dn8, locals.var_cfrd_i_dn9,)
    }
};
        locals.var_cfrd_i = assign6510_e6452;
        locals.var_cfrd_i_dn4 = assign6510_e6452_d_n4;
        locals.var_cfrd_i_dn6 = assign6510_e6452_d_n6;
        locals.var_cfrd_i_dn7 = assign6510_e6452_d_n7;
        locals.var_cfrd_i_dn8 = assign6510_e6452_d_n8;
        locals.var_cfrd_i_dn9 = assign6510_e6452_d_n9;
        locals.var_cfrd_i_rv = 0.0;

        let assign6520_e6455: f64 = (1.0 - locals.var_xge_i);
        locals.var_one_m_xge = assign6520_e6455;
        locals.var_one_m_xge_rv = 0.0;

        let assign6530_e6458: f64 = (1.04479e-10 * locals.var_one_m_xge);
        let assign6530_e6461: f64 = (1.43438e-10 * locals.var_xge_i);
        let assign6530_e6462: f64 = (assign6530_e6458 + assign6530_e6461);
        locals.var_epsch = assign6530_e6462;
        locals.var_epsch_rv = 0.0;

        let assign6540_e6466: f64 = (0.000473 * locals.var_tkc_sq);
        let assign6540_e6469: f64 = (636.0 + locals.var_tkc);
        let assign6540_e6470: f64 = (assign6540_e6466 / assign6540_e6469);
        let assign6540_e6471: f64 = (1.17 - assign6540_e6470);
        locals.var_egsi = assign6540_e6471;
        locals.var_egsi_dn4 = (-((((0.000473 * locals.var_tkc_sq_dn4) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn4)) / (assign6540_e6469 * assign6540_e6469)));
        locals.var_egsi_dn6 = (-((((0.000473 * locals.var_tkc_sq_dn6) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn6)) / (assign6540_e6469 * assign6540_e6469)));
        locals.var_egsi_dn7 = (-((((0.000473 * locals.var_tkc_sq_dn7) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn7)) / (assign6540_e6469 * assign6540_e6469)));
        locals.var_egsi_dn8 = (-((((0.000473 * locals.var_tkc_sq_dn8) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn8)) / (assign6540_e6469 * assign6540_e6469)));
        locals.var_egsi_dn9 = (-((((0.000473 * locals.var_tkc_sq_dn9) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn9)) / (assign6540_e6469 * assign6540_e6469)));
        locals.var_egsi_rv = 0.0;

        let assign6550_e6475: f64 = (0.0004774 * locals.var_tkc_sq);
        let assign6550_e6478: f64 = (235.0 + locals.var_tkc);
        let assign6550_e6479: f64 = (assign6550_e6475 / assign6550_e6478);
        let assign6550_e6480: f64 = (0.744 - assign6550_e6479);
        locals.var_egge = assign6550_e6480;
        locals.var_egge_dn4 = (-((((0.0004774 * locals.var_tkc_sq_dn4) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn4)) / (assign6550_e6478 * assign6550_e6478)));
        locals.var_egge_dn6 = (-((((0.0004774 * locals.var_tkc_sq_dn6) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn6)) / (assign6550_e6478 * assign6550_e6478)));
        locals.var_egge_dn7 = (-((((0.0004774 * locals.var_tkc_sq_dn7) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn7)) / (assign6550_e6478 * assign6550_e6478)));
        locals.var_egge_dn8 = (-((((0.0004774 * locals.var_tkc_sq_dn8) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn8)) / (assign6550_e6478 * assign6550_e6478)));
        locals.var_egge_dn9 = (-((((0.0004774 * locals.var_tkc_sq_dn9) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn9)) / (assign6550_e6478 * assign6550_e6478)));
        locals.var_egge_rv = 0.0;

        let assign6560_e6483: f64 = (locals.var_egge - locals.var_egsi);
        let assign6560_e6485: f64 = (-0.4);
        let assign6560_e6487: f64 = (assign6560_e6485 * locals.var_one_m_xge);
        let assign6560_e6488: f64 = (assign6560_e6483 + assign6560_e6487);
        let assign6560_e6490: f64 = (assign6560_e6488 * locals.var_xge_i);
        locals.var_deg = assign6560_e6490;
        locals.var_deg_dn4 = ((locals.var_egge_dn4 - locals.var_egsi_dn4) * locals.var_xge_i);
        locals.var_deg_dn6 = ((locals.var_egge_dn6 - locals.var_egsi_dn6) * locals.var_xge_i);
        locals.var_deg_dn7 = ((locals.var_egge_dn7 - locals.var_egsi_dn7) * locals.var_xge_i);
        locals.var_deg_dn8 = ((locals.var_egge_dn8 - locals.var_egsi_dn8) * locals.var_xge_i);
        locals.var_deg_dn9 = ((locals.var_egge_dn9 - locals.var_egsi_dn9) * locals.var_xge_i);
        locals.var_deg_rv = 0.0;

        let assign6570_e6493: f64 = (locals.var_egsi + locals.var_deg);
        locals.var_eg = assign6570_e6493;
        locals.var_eg_dn4 = (locals.var_egsi_dn4 + locals.var_deg_dn4);
        locals.var_eg_dn6 = (locals.var_egsi_dn6 + locals.var_deg_dn6);
        locals.var_eg_dn7 = (locals.var_egsi_dn7 + locals.var_deg_dn7);
        locals.var_eg_dn8 = (locals.var_egsi_dn8 + locals.var_deg_dn8);
        locals.var_eg_dn9 = (locals.var_egsi_dn9 + locals.var_deg_dn9);
        locals.var_eg_rv = 0.0;

        let assign6580_e6496: f64 = (0.5 * locals.var_eg);
        let assign6580_e6498: f64 = (assign6580_e6496 * locals.var_inv_phit0);
        locals.var_eg_2phit0 = assign6580_e6498;
        locals.var_eg_2phit0_dn4 = (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn4));
        locals.var_eg_2phit0_dn6 = (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn6));
        locals.var_eg_2phit0_dn7 = (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn7));
        locals.var_eg_2phit0_dn8 = (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn8));
        locals.var_eg_2phit0_dn9 = (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn9));
        locals.var_eg_2phit0_rv = 0.0;

        locals.var_eg_2phit0_woshe = locals.var_eg_2phit0;
        locals.var_eg_2phit0_woshe_dn4 = locals.var_eg_2phit0_dn4;
        locals.var_eg_2phit0_woshe_dn6 = locals.var_eg_2phit0_dn6;
        locals.var_eg_2phit0_woshe_dn7 = locals.var_eg_2phit0_dn7;
        locals.var_eg_2phit0_woshe_dn8 = locals.var_eg_2phit0_dn8;
        locals.var_eg_2phit0_woshe_dn9 = locals.var_eg_2phit0_dn9;
        locals.var_eg_2phit0_woshe_rv = 0.0;

        let assign6600_e6504: f64 = (10.0 * locals.var_xge_i);
        let assign6600_e6505: f64 = (assign6600_e6504).sqrt();
        let assign6600_e6506: f64 = (1.0 + assign6600_e6505);
        let assign6600_e6507: f64 = (1.0 / assign6600_e6506);
        locals.var_niratio = assign6600_e6507;
        locals.var_niratio_rv = 0.0;

        let assign6610_e6510: f64 = (0.05 * locals.var_xge_i);
        let assign6610_e6513: f64 = (0.5 * locals.var_deg);
        let assign6610_e6514: f64 = (assign6610_e6510 - assign6610_e6513);
        locals.var_dvfbch = assign6610_e6514;
        locals.var_dvfbch_dn4 = (-(0.5 * locals.var_deg_dn4));
        locals.var_dvfbch_dn6 = (-(0.5 * locals.var_deg_dn6));
        locals.var_dvfbch_dn7 = (-(0.5 * locals.var_deg_dn7));
        locals.var_dvfbch_dn8 = (-(0.5 * locals.var_deg_dn8));
        locals.var_dvfbch_dn9 = (-(0.5 * locals.var_deg_dn9));
        locals.var_dvfbch_rv = 0.0;

        let assign6620_e6517: f64 = (1.602176565e-19 * locals.var_nch_i);
        let assign6620_e6519: f64 = (assign6620_e6517 * 0.5);
        let assign6620_e6521: f64 = (assign6620_e6519 * locals.var_tsi_i);
        let assign6620_e6523: f64 = (assign6620_e6521 / 3.45313e-11);
        locals.var_temp = assign6620_e6523;
        locals.var_temp_dn4 = 0.0;
        locals.var_temp_dn6 = 0.0;
        locals.var_temp_dn7 = 0.0;
        locals.var_temp_dn8 = 0.0;
        locals.var_temp_dn9 = 0.0;
        locals.var_temp_rv = 0.0;

        let assign6630_e6526: f64 = if locals.var_typech_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard140 = assign6630_e6526;
        locals.var_guard140_rv = 0.0;

        let (assign6640_e6536, assign6640_e6536_d_n4, assign6640_e6536_d_n6, assign6640_e6536_d_n7, assign6640_e6536_d_n8, assign6640_e6536_d_n9,) = {
    if (locals.var_guard140 != 0.0) {
        let assign6640_e6532: f64 = (p.p13 * 4e-10);
        let assign6640_e6533: f64 = (locals.var_tox1_i + assign6640_e6532);
        let assign6640_e6534: f64 = (locals.var_temp * assign6640_e6533);
        (assign6640_e6534, (locals.var_temp_dn4 * assign6640_e6533), (locals.var_temp_dn6 * assign6640_e6533), (locals.var_temp_dn7 * assign6640_e6533), (locals.var_temp_dn8 * assign6640_e6533), (locals.var_temp_dn9 * assign6640_e6533),)
    } else {
        (locals.var_dvfb1nch, locals.var_dvfb1nch_dn4, locals.var_dvfb1nch_dn6, locals.var_dvfb1nch_dn7, locals.var_dvfb1nch_dn8, locals.var_dvfb1nch_dn9,)
    }
};
        locals.var_dvfb1nch = assign6640_e6536;
        locals.var_dvfb1nch_dn4 = assign6640_e6536_d_n4;
        locals.var_dvfb1nch_dn6 = assign6640_e6536_d_n6;
        locals.var_dvfb1nch_dn7 = assign6640_e6536_d_n7;
        locals.var_dvfb1nch_dn8 = assign6640_e6536_d_n8;
        locals.var_dvfb1nch_dn9 = assign6640_e6536_d_n9;
        locals.var_dvfb1nch_rv = 0.0;

        let (assign6650_e6546, assign6650_e6546_d_n4, assign6650_e6546_d_n6, assign6650_e6546_d_n7, assign6650_e6546_d_n8, assign6650_e6546_d_n9,) = {
    if (locals.var_guard140 != 0.0) {
        let assign6650_e6542: f64 = (p.p13 * 4e-10);
        let assign6650_e6543: f64 = (locals.var_tox2_i + assign6650_e6542);
        let assign6650_e6544: f64 = (locals.var_temp * assign6650_e6543);
        (assign6650_e6544, (locals.var_temp_dn4 * assign6650_e6543), (locals.var_temp_dn6 * assign6650_e6543), (locals.var_temp_dn7 * assign6650_e6543), (locals.var_temp_dn8 * assign6650_e6543), (locals.var_temp_dn9 * assign6650_e6543),)
    } else {
        (locals.var_dvfb2nch, locals.var_dvfb2nch_dn4, locals.var_dvfb2nch_dn6, locals.var_dvfb2nch_dn7, locals.var_dvfb2nch_dn8, locals.var_dvfb2nch_dn9,)
    }
};
        locals.var_dvfb2nch = assign6650_e6546;
        locals.var_dvfb2nch_dn4 = assign6650_e6546_d_n4;
        locals.var_dvfb2nch_dn6 = assign6650_e6546_d_n6;
        locals.var_dvfb2nch_dn7 = assign6650_e6546_d_n7;
        locals.var_dvfb2nch_dn8 = assign6650_e6546_d_n8;
        locals.var_dvfb2nch_dn9 = assign6650_e6546_d_n9;
        locals.var_dvfb2nch_rv = 0.0;

        let (assign6660_e6558, assign6660_e6558_d_n4, assign6660_e6558_d_n6, assign6660_e6558_d_n7, assign6660_e6558_d_n8, assign6660_e6558_d_n9,) = {
    if (locals.var_guard140 == 0.0) {
        let assign6660_e6550: f64 = (-locals.var_temp);
        let assign6660_e6554: f64 = (p.p13 * 4e-10);
        let assign6660_e6555: f64 = (locals.var_tox1_i + assign6660_e6554);
        let assign6660_e6556: f64 = (assign6660_e6550 * assign6660_e6555);
        (assign6660_e6556, ((-locals.var_temp_dn4) * assign6660_e6555), ((-locals.var_temp_dn6) * assign6660_e6555), ((-locals.var_temp_dn7) * assign6660_e6555), ((-locals.var_temp_dn8) * assign6660_e6555), ((-locals.var_temp_dn9) * assign6660_e6555),)
    } else {
        (locals.var_dvfb1nch, locals.var_dvfb1nch_dn4, locals.var_dvfb1nch_dn6, locals.var_dvfb1nch_dn7, locals.var_dvfb1nch_dn8, locals.var_dvfb1nch_dn9,)
    }
};
        locals.var_dvfb1nch = assign6660_e6558;
        locals.var_dvfb1nch_dn4 = assign6660_e6558_d_n4;
        locals.var_dvfb1nch_dn6 = assign6660_e6558_d_n6;
        locals.var_dvfb1nch_dn7 = assign6660_e6558_d_n7;
        locals.var_dvfb1nch_dn8 = assign6660_e6558_d_n8;
        locals.var_dvfb1nch_dn9 = assign6660_e6558_d_n9;
        locals.var_dvfb1nch_rv = 0.0;

        let (assign6670_e6570, assign6670_e6570_d_n4, assign6670_e6570_d_n6, assign6670_e6570_d_n7, assign6670_e6570_d_n8, assign6670_e6570_d_n9,) = {
    if (locals.var_guard140 == 0.0) {
        let assign6670_e6562: f64 = (-locals.var_temp);
        let assign6670_e6566: f64 = (p.p13 * 4e-10);
        let assign6670_e6567: f64 = (locals.var_tox2_i + assign6670_e6566);
        let assign6670_e6568: f64 = (assign6670_e6562 * assign6670_e6567);
        (assign6670_e6568, ((-locals.var_temp_dn4) * assign6670_e6567), ((-locals.var_temp_dn6) * assign6670_e6567), ((-locals.var_temp_dn7) * assign6670_e6567), ((-locals.var_temp_dn8) * assign6670_e6567), ((-locals.var_temp_dn9) * assign6670_e6567),)
    } else {
        (locals.var_dvfb2nch, locals.var_dvfb2nch_dn4, locals.var_dvfb2nch_dn6, locals.var_dvfb2nch_dn7, locals.var_dvfb2nch_dn8, locals.var_dvfb2nch_dn9,)
    }
};
        locals.var_dvfb2nch = assign6670_e6570;
        locals.var_dvfb2nch_dn4 = assign6670_e6570_d_n4;
        locals.var_dvfb2nch_dn6 = assign6670_e6570_d_n6;
        locals.var_dvfb2nch_dn7 = assign6670_e6570_d_n7;
        locals.var_dvfb2nch_dn8 = assign6670_e6570_d_n8;
        locals.var_dvfb2nch_dn9 = assign6670_e6570_d_n9;
        locals.var_dvfb2nch_rv = 0.0;

        let assign6680_e6573: f64 = (locals.var_tkc * 0.0033333333333);
        let assign6680_e6574: f64 = (assign6680_e6573).sqrt();
        locals.var_temp = assign6680_e6574;
        locals.var_temp_dn4 = ((locals.var_tkc_dn4 * 0.0033333333333) / (2.0 * assign6680_e6574));
        locals.var_temp_dn6 = ((locals.var_tkc_dn6 * 0.0033333333333) / (2.0 * assign6680_e6574));
        locals.var_temp_dn7 = ((locals.var_tkc_dn7 * 0.0033333333333) / (2.0 * assign6680_e6574));
        locals.var_temp_dn8 = ((locals.var_tkc_dn8 * 0.0033333333333) / (2.0 * assign6680_e6574));
        locals.var_temp_dn9 = ((locals.var_tkc_dn9 * 0.0033333333333) / (2.0 * assign6680_e6574));
        locals.var_temp_rv = 0.0;

        let assign6690_e6577: f64 = (4.05e25 * locals.var_temp);
        let assign6690_e6579: f64 = (assign6690_e6577 * locals.var_temp);
        let assign6690_e6581: f64 = (assign6690_e6579 * locals.var_temp);
        locals.var_temp1 = assign6690_e6581;
        locals.var_temp1_dn4 = (((((4.05e25 * locals.var_temp_dn4) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn4)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn4));
        locals.var_temp1_dn6 = (((((4.05e25 * locals.var_temp_dn6) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn6)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn6));
        locals.var_temp1_dn7 = (((((4.05e25 * locals.var_temp_dn7) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn7)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn7));
        locals.var_temp1_dn8 = (((((4.05e25 * locals.var_temp_dn8) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn8)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn8));
        locals.var_temp1_dn9 = (((((4.05e25 * locals.var_temp_dn9) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn9)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn9));
        locals.var_temp1_rv = 0.0;

        let assign6700_e6584: f64 = (locals.var_temp1 * locals.var_niratio);
        locals.var_neff = assign6700_e6584;
        locals.var_neff_dn4 = (locals.var_temp1_dn4 * locals.var_niratio);
        locals.var_neff_dn6 = (locals.var_temp1_dn6 * locals.var_niratio);
        locals.var_neff_dn7 = (locals.var_temp1_dn7 * locals.var_niratio);
        locals.var_neff_dn8 = (locals.var_temp1_dn8 * locals.var_niratio);
        locals.var_neff_dn9 = (locals.var_temp1_dn9 * locals.var_niratio);
        locals.var_neff_rv = 0.0;

        let assign6710_e6588: f64 = (0.5 * locals.var_deg);
        let assign6710_e6590: f64 = (assign6710_e6588 * locals.var_inv_phit0);
        let assign6710_e6591: f64 = (assign6710_e6590).exp();
        let assign6710_e6592: f64 = (locals.var_temp1 * assign6710_e6591);
        locals.var_neff_poly = assign6710_e6592;
        locals.var_neff_poly_dn4 = ((locals.var_temp1_dn4 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn4) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn4)))));
        locals.var_neff_poly_dn6 = ((locals.var_temp1_dn6 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn6) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn6)))));
        locals.var_neff_poly_dn7 = ((locals.var_temp1_dn7 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn7) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn7)))));
        locals.var_neff_poly_dn8 = ((locals.var_temp1_dn8 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn8) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn8)))));
        locals.var_neff_poly_dn9 = ((locals.var_temp1_dn9 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn9) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn9)))));
        locals.var_neff_poly_rv = 0.0;

        let assign6720_e6596: f64 = (0.5 * locals.var_deg);
        let assign6720_e6598: f64 = (assign6720_e6596 * locals.var_inv_phit0);
        let assign6720_e6599: f64 = (assign6720_e6598).exp();
        let assign6720_e6600: f64 = (locals.var_temp1 * assign6720_e6599);
        locals.var_neff_sub = assign6720_e6600;
        locals.var_neff_sub_dn4 = ((locals.var_temp1_dn4 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn4) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn4)))));
        locals.var_neff_sub_dn6 = ((locals.var_temp1_dn6 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn6) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn6)))));
        locals.var_neff_sub_dn7 = ((locals.var_temp1_dn7 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn7) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn7)))));
        locals.var_neff_sub_dn8 = ((locals.var_temp1_dn8 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn8) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn8)))));
        locals.var_neff_sub_dn9 = ((locals.var_temp1_dn9 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn9) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn9)))));
        locals.var_neff_sub_rv = 0.0;

        let assign6730_e6603: f64 = (3.45313e-11 / locals.var_tox1_i);
        locals.var_cox1init = assign6730_e6603;
        locals.var_cox1init_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign6740_e6606: f64 = (3.45313e-11 / locals.var_tox2_i);
        locals.var_cox2init = assign6740_e6606;
        locals.var_cox2init_rv = 0.0;

        let assign6750_e6609: f64 = if locals.var_pnce_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard141 = assign6750_e6609;
        locals.var_guard141_rv = 0.0;

        let (assign6760_e6617,) = {
    if (locals.var_guard141 != 0.0) {
        let assign6760_e6614: f64 = (1.0 + locals.var_pnce_i);
        let assign6760_e6615: f64 = (locals.var_cox1init * assign6760_e6614);
        (assign6760_e6615,)
    } else {
        (locals.var_cox1prime,)
    }
};
        locals.var_cox1prime = assign6760_e6617;
        locals.var_cox1prime_rv = 0.0;

        let (assign6770_e6621,) = {
    if (locals.var_guard141 != 0.0) {
        (locals.var_cox2init,)
    } else {
        (locals.var_cox2prime,)
    }
};
        locals.var_cox2prime = assign6770_e6621;
        locals.var_cox2prime_rv = 0.0;

        let (assign6780_e6626,) = {
    if (locals.var_guard141 == 0.0) {
        (locals.var_cox1init,)
    } else {
        (locals.var_cox1prime,)
    }
};
        locals.var_cox1prime = assign6780_e6626;
        locals.var_cox1prime_rv = 0.0;

        let (assign6790_e6635,) = {
    if (locals.var_guard141 == 0.0) {
        let assign6790_e6632: f64 = (1.0 - locals.var_pnce_i);
        let assign6790_e6633: f64 = (locals.var_cox2init * assign6790_e6632);
        (assign6790_e6633,)
    } else {
        (locals.var_cox2prime,)
    }
};
        locals.var_cox2prime = assign6790_e6635;
        locals.var_cox2prime_rv = 0.0;

        let assign6800_e6638: f64 = (locals.var_epsch / locals.var_tsi_i);
        locals.var_csiprime_0 = assign6800_e6638;
        locals.var_csiprime_0_rv = 0.0;

        let assign6810_e6643: f64 = (locals.var_ct_i * locals.var_rtn);
        let assign6810_e6644: f64 = (1.0 + assign6810_e6643);
        let assign6810_e6645: f64 = (locals.var_phit0 * assign6810_e6644);
        locals.var_phit = assign6810_e6645;
        locals.var_phit_dn4 = ((locals.var_phit0_dn4 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn4)));
        locals.var_phit_dn6 = ((locals.var_phit0_dn6 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn6)));
        locals.var_phit_dn7 = ((locals.var_phit0_dn7 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn7)));
        locals.var_phit_dn8 = ((locals.var_phit0_dn8 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn8)));
        locals.var_phit_dn9 = ((locals.var_phit0_dn9 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn9)));
        locals.var_phit_rv = 0.0;

        let assign6820_e6648: f64 = (1.0 / locals.var_phit);
        locals.var_inv_phit = assign6820_e6648;
        locals.var_inv_phit_dn4 = (-(locals.var_phit_dn4 / (locals.var_phit * locals.var_phit)));
        locals.var_inv_phit_dn6 = (-(locals.var_phit_dn6 / (locals.var_phit * locals.var_phit)));
        locals.var_inv_phit_dn7 = (-(locals.var_phit_dn7 / (locals.var_phit * locals.var_phit)));
        locals.var_inv_phit_dn8 = (-(locals.var_phit_dn8 / (locals.var_phit * locals.var_phit)));
        locals.var_inv_phit_dn9 = (-(locals.var_phit_dn9 / (locals.var_phit * locals.var_phit)));
        locals.var_inv_phit_rv = 0.0;

        let assign6830_e6651: f64 = (0.5 * locals.var_eg);
        let assign6830_e6653: f64 = (assign6830_e6651 * locals.var_inv_phit);
        locals.var_eg_2phit = assign6830_e6653;
        locals.var_eg_2phit_dn4 = (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn4));
        locals.var_eg_2phit_dn6 = (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn6));
        locals.var_eg_2phit_dn7 = (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn7));
        locals.var_eg_2phit_dn8 = (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn8));
        locals.var_eg_2phit_dn9 = (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn9));
        locals.var_eg_2phit_rv = 0.0;

        let assign6840_e6656: f64 = (locals.var_cox1prime / locals.var_csiprime_0);
        locals.var_k1_1d = assign6840_e6656;
        locals.var_k1_1d_rv = 0.0;

        let assign6850_e6659: f64 = (locals.var_cox2prime / locals.var_csiprime_0);
        locals.var_k2_1d = assign6850_e6659;
        locals.var_k2_1d_rv = 0.0;

        let assign6860_e6664: f64 = (1.0 / locals.var_k1_1d);
        let assign6860_e6665: f64 = (1.0 + assign6860_e6664);
        let assign6860_e6668: f64 = (1.0 / locals.var_k2_1d);
        let assign6860_e6669: f64 = (assign6860_e6665 + assign6860_e6668);
        let assign6860_e6670: f64 = (1.0 / assign6860_e6669);
        locals.var_keq_1d = assign6860_e6670;
        locals.var_keq_1d_rv = 0.0;

        let assign6870_e6673: f64 = (2.0 * 1.602176565e-19);
        let assign6870_e6675: f64 = (assign6870_e6673 * locals.var_neff);
        let assign6870_e6677: f64 = (assign6870_e6675 * locals.var_epsch);
        let assign6870_e6679: f64 = (assign6870_e6677 * locals.var_inv_phit);
        locals.var_a0_csisq = assign6870_e6679;
        locals.var_a0_csisq_dn4 = ((((assign6870_e6673 * locals.var_neff_dn4) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn4));
        locals.var_a0_csisq_dn6 = ((((assign6870_e6673 * locals.var_neff_dn6) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn6));
        locals.var_a0_csisq_dn7 = ((((assign6870_e6673 * locals.var_neff_dn7) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn7));
        locals.var_a0_csisq_dn8 = ((((assign6870_e6673 * locals.var_neff_dn8) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn8));
        locals.var_a0_csisq_dn9 = ((((assign6870_e6673 * locals.var_neff_dn9) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn9));
        locals.var_a0_csisq_rv = 0.0;

        let assign6880_e6682: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign6880_e6684: f64 = (assign6880_e6682 / locals.var_a0_csisq);
        let assign6880_e6685: f64 = (assign6880_e6684).ln();
        let assign6880_e6687: f64 = (assign6880_e6685 - 0.6931471805599);
        locals.var_xth_1d = assign6880_e6687;
        locals.var_xth_1d_dn4 = ((-((assign6880_e6682 * locals.var_a0_csisq_dn4) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684);
        locals.var_xth_1d_dn6 = ((-((assign6880_e6682 * locals.var_a0_csisq_dn6) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684);
        locals.var_xth_1d_dn7 = ((-((assign6880_e6682 * locals.var_a0_csisq_dn7) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684);
        locals.var_xth_1d_dn8 = ((-((assign6880_e6682 * locals.var_a0_csisq_dn8) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684);
        locals.var_xth_1d_dn9 = ((-((assign6880_e6682 * locals.var_a0_csisq_dn9) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684);
        locals.var_xth_1d_rv = 0.0;

        let assign6890_e6690: f64 = (0.5 * 1.602176565e-19);
        let assign6890_e6692: f64 = (assign6890_e6690 * locals.var_nsddc_i);
        let assign6890_e6694: f64 = (assign6890_e6692 * locals.var_tsi_i);
        let assign6890_e6697: f64 = (locals.var_cox1prime + locals.var_cox2prime);
        let assign6890_e6698: f64 = (assign6890_e6694 / assign6890_e6697);
        let assign6890_e6700: f64 = (assign6890_e6698 * locals.var_inv_phit);
        locals.var_xsddep = assign6890_e6700;
        locals.var_xsddep_dn4 = (assign6890_e6698 * locals.var_inv_phit_dn4);
        locals.var_xsddep_dn6 = (assign6890_e6698 * locals.var_inv_phit_dn6);
        locals.var_xsddep_dn7 = (assign6890_e6698 * locals.var_inv_phit_dn7);
        locals.var_xsddep_dn8 = (assign6890_e6698 * locals.var_inv_phit_dn8);
        locals.var_xsddep_dn9 = (assign6890_e6698 * locals.var_inv_phit_dn9);
        locals.var_xsddep_rv = 0.0;

        let assign6900_e6703: f64 = (locals.var_stcf_i * locals.var_dt);
        locals.var_temp = assign6900_e6703;
        locals.var_temp_dn4 = ((locals.var_stcf_i_dn4 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn4));
        locals.var_temp_dn6 = ((locals.var_stcf_i_dn6 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn6));
        locals.var_temp_dn7 = ((locals.var_stcf_i_dn7 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn7));
        locals.var_temp_dn8 = ((locals.var_stcf_i_dn8 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn8));
        locals.var_temp_dn9 = ((locals.var_stcf_i_dn9 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn9));
        locals.var_temp_rv = 0.0;

        let assign6910_e6706: f64 = (locals.var_cf1_t + locals.var_temp);
        locals.var_cf1_i = assign6910_e6706;
        locals.var_cf1_i_dn4 = (locals.var_cf1_t_dn4 + locals.var_temp_dn4);
        locals.var_cf1_i_dn6 = (locals.var_cf1_t_dn6 + locals.var_temp_dn6);
        locals.var_cf1_i_dn7 = (locals.var_cf1_t_dn7 + locals.var_temp_dn7);
        locals.var_cf1_i_dn8 = (locals.var_cf1_t_dn8 + locals.var_temp_dn8);
        locals.var_cf1_i_dn9 = (locals.var_cf1_t_dn9 + locals.var_temp_dn9);
        locals.var_cf1_i_rv = 0.0;

        let assign6920_e6709: f64 = (locals.var_cf2_t + locals.var_temp);
        locals.var_cf2_i = assign6920_e6709;
        locals.var_cf2_i_dn4 = (locals.var_cf2_t_dn4 + locals.var_temp_dn4);
        locals.var_cf2_i_dn6 = (locals.var_cf2_t_dn6 + locals.var_temp_dn6);
        locals.var_cf2_i_dn7 = (locals.var_cf2_t_dn7 + locals.var_temp_dn7);
        locals.var_cf2_i_dn8 = (locals.var_cf2_t_dn8 + locals.var_temp_dn8);
        locals.var_cf2_i_dn9 = (locals.var_cf2_t_dn9 + locals.var_temp_dn9);
        locals.var_cf2_i_rv = 0.0;

        let assign6930_e6712: f64 = (locals.var_cfac1_t + locals.var_temp);
        locals.var_cfac1_i = assign6930_e6712;
        locals.var_cfac1_i_dn4 = (locals.var_cfac1_t_dn4 + locals.var_temp_dn4);
        locals.var_cfac1_i_dn6 = (locals.var_cfac1_t_dn6 + locals.var_temp_dn6);
        locals.var_cfac1_i_dn7 = (locals.var_cfac1_t_dn7 + locals.var_temp_dn7);
        locals.var_cfac1_i_dn8 = (locals.var_cfac1_t_dn8 + locals.var_temp_dn8);
        locals.var_cfac1_i_dn9 = (locals.var_cfac1_t_dn9 + locals.var_temp_dn9);
        locals.var_cfac1_i_rv = 0.0;

        let assign6940_e6715: f64 = (locals.var_cfac2_t + locals.var_temp);
        locals.var_cfac2_i = assign6940_e6715;
        locals.var_cfac2_i_dn4 = (locals.var_cfac2_t_dn4 + locals.var_temp_dn4);
        locals.var_cfac2_i_dn6 = (locals.var_cfac2_t_dn6 + locals.var_temp_dn6);
        locals.var_cfac2_i_dn7 = (locals.var_cfac2_t_dn7 + locals.var_temp_dn7);
        locals.var_cfac2_i_dn8 = (locals.var_cfac2_t_dn8 + locals.var_temp_dn8);
        locals.var_cfac2_i_dn9 = (locals.var_cfac2_t_dn9 + locals.var_temp_dn9);
        locals.var_cfac2_i_rv = 0.0;

        let assign6950_e6718: f64 = (locals.var_cfd_i * locals.var_inv_phit);
        locals.var_xd0 = assign6950_e6718;
        locals.var_xd0_dn4 = (locals.var_cfd_i * locals.var_inv_phit_dn4);
        locals.var_xd0_dn6 = (locals.var_cfd_i * locals.var_inv_phit_dn6);
        locals.var_xd0_dn7 = (locals.var_cfd_i * locals.var_inv_phit_dn7);
        locals.var_xd0_dn8 = (locals.var_cfd_i * locals.var_inv_phit_dn8);
        locals.var_xd0_dn9 = (locals.var_cfd_i * locals.var_inv_phit_dn9);
        locals.var_xd0_rv = 0.0;

        let assign6960_e6721: f64 = (2.0 * 1.602176565e-19);
        let assign6960_e6723: f64 = (assign6960_e6721 * locals.var_nsub_i);
        let assign6960_e6725: f64 = (assign6960_e6723 * 1.04479e-10);
        let assign6960_e6727: f64 = (assign6960_e6725 * locals.var_inv_phit0);
        let assign6960_e6728: f64 = (assign6960_e6727).sqrt();
        let assign6960_e6730: f64 = (assign6960_e6728 / locals.var_cox2prime);
        locals.var_gfsub = assign6960_e6730;
        locals.var_gfsub_dn4 = (((assign6960_e6725 * locals.var_inv_phit0_dn4) / (2.0 * assign6960_e6728)) / locals.var_cox2prime);
        locals.var_gfsub_dn6 = (((assign6960_e6725 * locals.var_inv_phit0_dn6) / (2.0 * assign6960_e6728)) / locals.var_cox2prime);
        locals.var_gfsub_dn7 = (((assign6960_e6725 * locals.var_inv_phit0_dn7) / (2.0 * assign6960_e6728)) / locals.var_cox2prime);
        locals.var_gfsub_dn8 = (((assign6960_e6725 * locals.var_inv_phit0_dn8) / (2.0 * assign6960_e6728)) / locals.var_cox2prime);
        locals.var_gfsub_dn9 = (((assign6960_e6725 * locals.var_inv_phit0_dn9) / (2.0 * assign6960_e6728)) / locals.var_cox2prime);
        locals.var_gfsub_rv = 0.0;

        let assign6970_e6733: f64 = (locals.var_gfsub * locals.var_gfsub);
        locals.var_gfsub2 = assign6970_e6733;
        locals.var_gfsub2_dn4 = ((locals.var_gfsub_dn4 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn4));
        locals.var_gfsub2_dn6 = ((locals.var_gfsub_dn6 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn6));
        locals.var_gfsub2_dn7 = ((locals.var_gfsub_dn7 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn7));
        locals.var_gfsub2_dn8 = ((locals.var_gfsub_dn8 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn8));
        locals.var_gfsub2_dn9 = ((locals.var_gfsub_dn9 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn9));
        locals.var_gfsub2_rv = 0.0;

        let assign6980_e6736: f64 = (1.0 / locals.var_gfsub2);
        locals.var_inv_gfsub2 = assign6980_e6736;
        locals.var_inv_gfsub2_dn4 = (-(locals.var_gfsub2_dn4 / (locals.var_gfsub2 * locals.var_gfsub2)));
        locals.var_inv_gfsub2_dn6 = (-(locals.var_gfsub2_dn6 / (locals.var_gfsub2 * locals.var_gfsub2)));
        locals.var_inv_gfsub2_dn7 = (-(locals.var_gfsub2_dn7 / (locals.var_gfsub2 * locals.var_gfsub2)));
        locals.var_inv_gfsub2_dn8 = (-(locals.var_gfsub2_dn8 / (locals.var_gfsub2 * locals.var_gfsub2)));
        locals.var_inv_gfsub2_dn9 = (-(locals.var_gfsub2_dn9 / (locals.var_gfsub2 * locals.var_gfsub2)));
        locals.var_inv_gfsub2_rv = 0.0;

        let assign6990_e6740: f64 = (locals.var_gfsub / 1.4142135623731);
        let assign6990_e6741: f64 = (1.0 + assign6990_e6740);
        locals.var_xisub = assign6990_e6741;
        locals.var_xisub_dn4 = (locals.var_gfsub_dn4 / 1.4142135623731);
        locals.var_xisub_dn6 = (locals.var_gfsub_dn6 / 1.4142135623731);
        locals.var_xisub_dn7 = (locals.var_gfsub_dn7 / 1.4142135623731);
        locals.var_xisub_dn8 = (locals.var_gfsub_dn8 / 1.4142135623731);
        locals.var_xisub_dn9 = (locals.var_gfsub_dn9 / 1.4142135623731);
        locals.var_xisub_rv = 0.0;

        let assign7000_e6744: f64 = (1.0 / locals.var_xisub);
        locals.var_inv_xisub = assign7000_e6744;
        locals.var_inv_xisub_dn4 = (-(locals.var_xisub_dn4 / (locals.var_xisub * locals.var_xisub)));
        locals.var_inv_xisub_dn6 = (-(locals.var_xisub_dn6 / (locals.var_xisub * locals.var_xisub)));
        locals.var_inv_xisub_dn7 = (-(locals.var_xisub_dn7 / (locals.var_xisub * locals.var_xisub)));
        locals.var_inv_xisub_dn8 = (-(locals.var_xisub_dn8 / (locals.var_xisub * locals.var_xisub)));
        locals.var_inv_xisub_dn9 = (-(locals.var_xisub_dn9 / (locals.var_xisub * locals.var_xisub)));
        locals.var_inv_xisub_rv = 0.0;

        let assign7010_e6747: f64 = (1e-5 * locals.var_xisub);
        locals.var_margin_sub = assign7010_e6747;
        locals.var_margin_sub_dn4 = (1e-5 * locals.var_xisub_dn4);
        locals.var_margin_sub_dn6 = (1e-5 * locals.var_xisub_dn6);
        locals.var_margin_sub_dn7 = (1e-5 * locals.var_xisub_dn7);
        locals.var_margin_sub_dn8 = (1e-5 * locals.var_xisub_dn8);
        locals.var_margin_sub_dn9 = (1e-5 * locals.var_xisub_dn9);
        locals.var_margin_sub_rv = 0.0;

        let assign7020_e6750: f64 = (locals.var_nsub_i / locals.var_neff_sub);
        let assign7020_e6751: f64 = (assign7020_e6750).ln();
        let assign7020_e6753: f64 = (assign7020_e6751 + locals.var_eg_2phit0);
        locals.var_xb_sub = assign7020_e6753;
        locals.var_xb_sub_dn4 = (((-((locals.var_nsub_i * locals.var_neff_sub_dn4) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn4);
        locals.var_xb_sub_dn6 = (((-((locals.var_nsub_i * locals.var_neff_sub_dn6) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn6);
        locals.var_xb_sub_dn7 = (((-((locals.var_nsub_i * locals.var_neff_sub_dn7) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn7);
        locals.var_xb_sub_dn8 = (((-((locals.var_nsub_i * locals.var_neff_sub_dn8) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn8);
        locals.var_xb_sub_dn9 = (((-((locals.var_nsub_i * locals.var_neff_sub_dn9) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn9);
        locals.var_xb_sub_rv = 0.0;

        let assign7030_e6756: f64 = (2.0 * locals.var_xb_sub);
        locals.var_xn_sub = assign7030_e6756;
        locals.var_xn_sub_dn4 = (2.0 * locals.var_xb_sub_dn4);
        locals.var_xn_sub_dn6 = (2.0 * locals.var_xb_sub_dn6);
        locals.var_xn_sub_dn7 = (2.0 * locals.var_xb_sub_dn7);
        locals.var_xn_sub_dn8 = (2.0 * locals.var_xb_sub_dn8);
        locals.var_xn_sub_dn9 = (2.0 * locals.var_xb_sub_dn9);
        locals.var_xn_sub_rv = 0.0;

        let assign7040_e6759: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard142 = assign7040_e6759;
        locals.var_guard142_rv = 0.0;

        let (assign7050_e6769, assign7050_e6769_d_n4, assign7050_e6769_d_n6, assign7050_e6769_d_n7, assign7050_e6769_d_n8, assign7050_e6769_d_n9,) = {
    if (locals.var_guard142 != 0.0) {
        let assign7050_e6764: f64 = (locals.var_typesub_i * locals.var_phit0);
        let assign7050_e6766: f64 = (assign7050_e6764 * locals.var_xb_sub);
        let assign7050_e6767: f64 = (locals.var_vfb2_t + assign7050_e6766);
        (assign7050_e6767, (locals.var_vfb2_t_dn4 + (((locals.var_typesub_i * locals.var_phit0_dn4) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn4))), (locals.var_vfb2_t_dn6 + (((locals.var_typesub_i * locals.var_phit0_dn6) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn6))), (locals.var_vfb2_t_dn7 + (((locals.var_typesub_i * locals.var_phit0_dn7) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn7))), (locals.var_vfb2_t_dn8 + (((locals.var_typesub_i * locals.var_phit0_dn8) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn8))), (locals.var_vfb2_t_dn9 + (((locals.var_typesub_i * locals.var_phit0_dn9) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn9))),)
    } else {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    }
};
        locals.var_vfb2_t = assign7050_e6769;
        locals.var_vfb2_t_dn4 = assign7050_e6769_d_n4;
        locals.var_vfb2_t_dn6 = assign7050_e6769_d_n6;
        locals.var_vfb2_t_dn7 = assign7050_e6769_d_n7;
        locals.var_vfb2_t_dn8 = assign7050_e6769_d_n8;
        locals.var_vfb2_t_dn9 = assign7050_e6769_d_n9;
        locals.var_vfb2_t_rv = 0.0;

        let (assign7060_e6779, assign7060_e6779_d_n4, assign7060_e6779_d_n6, assign7060_e6779_d_n7, assign7060_e6779_d_n8, assign7060_e6779_d_n9,) = {
    if (locals.var_guard142 != 0.0) {
        let assign7060_e6774: f64 = (locals.var_typesub_i * locals.var_phit0);
        let assign7060_e6776: f64 = (assign7060_e6774 * locals.var_xb_sub);
        let assign7060_e6777: f64 = (locals.var_vfbac2_t + assign7060_e6776);
        (assign7060_e6777, (locals.var_vfbac2_t_dn4 + (((locals.var_typesub_i * locals.var_phit0_dn4) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn4))), (locals.var_vfbac2_t_dn6 + (((locals.var_typesub_i * locals.var_phit0_dn6) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn6))), (locals.var_vfbac2_t_dn7 + (((locals.var_typesub_i * locals.var_phit0_dn7) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn7))), (locals.var_vfbac2_t_dn8 + (((locals.var_typesub_i * locals.var_phit0_dn8) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn8))), (locals.var_vfbac2_t_dn9 + (((locals.var_typesub_i * locals.var_phit0_dn9) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn9))),)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign7060_e6779;
        locals.var_vfbac2_t_dn4 = assign7060_e6779_d_n4;
        locals.var_vfbac2_t_dn6 = assign7060_e6779_d_n6;
        locals.var_vfbac2_t_dn7 = assign7060_e6779_d_n7;
        locals.var_vfbac2_t_dn8 = assign7060_e6779_d_n8;
        locals.var_vfbac2_t_dn9 = assign7060_e6779_d_n9;
        locals.var_vfbac2_t_rv = 0.0;

        locals.var_dvfbpdep = 0.0;
        locals.var_dvfbpdep_dn4 = 0.0;
        locals.var_dvfbpdep_dn6 = 0.0;
        locals.var_dvfbpdep_dn7 = 0.0;
        locals.var_dvfbpdep_dn8 = 0.0;
        locals.var_dvfbpdep_dn9 = 0.0;
        locals.var_dvfbpdep_rv = 0.0;

        let assign7080_e6783: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign7080_e6783;
        locals.var_guard143_rv = 0.0;

        let (assign7090_e6794, assign7090_e6794_d_n4, assign7090_e6794_d_n6, assign7090_e6794_d_n7, assign7090_e6794_d_n8, assign7090_e6794_d_n9,) = {
    if (locals.var_guard143 != 0.0) {
        let assign7090_e6788: f64 = (locals.var_np_i / locals.var_neff_poly);
        let assign7090_e6789: f64 = (assign7090_e6788).ln();
        let assign7090_e6791: f64 = (assign7090_e6789 + locals.var_eg_2phit0);
        let assign7090_e6792: f64 = (locals.var_phit0 * assign7090_e6791);
        (assign7090_e6792, ((locals.var_phit0_dn4 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn4 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn4)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn4))), ((locals.var_phit0_dn6 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn6 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn6)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn6))), ((locals.var_phit0_dn7 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn7 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn7)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn7))), ((locals.var_phit0_dn8 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn8 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn8)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn8))), ((locals.var_phit0_dn9 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn9 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn9)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn9))),)
    } else {
        (locals.var_dvfbpdep, locals.var_dvfbpdep_dn4, locals.var_dvfbpdep_dn6, locals.var_dvfbpdep_dn7, locals.var_dvfbpdep_dn8, locals.var_dvfbpdep_dn9,)
    }
};
        locals.var_dvfbpdep = assign7090_e6794;
        locals.var_dvfbpdep_dn4 = assign7090_e6794_d_n4;
        locals.var_dvfbpdep_dn6 = assign7090_e6794_d_n6;
        locals.var_dvfbpdep_dn7 = assign7090_e6794_d_n7;
        locals.var_dvfbpdep_dn8 = assign7090_e6794_d_n8;
        locals.var_dvfbpdep_dn9 = assign7090_e6794_d_n9;
        locals.var_dvfbpdep_rv = 0.0;

        let assign7100_e6797: f64 = (2.0 * 1.602176565e-19);
        let assign7100_e6799: f64 = (assign7100_e6797 * locals.var_epsch);
        let assign7100_e6801: f64 = (assign7100_e6799 * locals.var_np_i);
        let assign7100_e6802: f64 = (assign7100_e6801).sqrt();
        let assign7100_e6804: f64 = (assign7100_e6802 / locals.var_cox1init);
        locals.var_kp = assign7100_e6804;
        locals.var_kp_dn4 = (((assign7100_e6799 * locals.var_np_i_dn4) / (2.0 * assign7100_e6802)) / locals.var_cox1init);
        locals.var_kp_dn6 = (((assign7100_e6799 * locals.var_np_i_dn6) / (2.0 * assign7100_e6802)) / locals.var_cox1init);
        locals.var_kp_dn7 = (((assign7100_e6799 * locals.var_np_i_dn7) / (2.0 * assign7100_e6802)) / locals.var_cox1init);
        locals.var_kp_dn8 = (((assign7100_e6799 * locals.var_np_i_dn8) / (2.0 * assign7100_e6802)) / locals.var_cox1init);
        locals.var_kp_dn9 = (((assign7100_e6799 * locals.var_np_i_dn9) / (2.0 * assign7100_e6802)) / locals.var_cox1init);
        locals.var_kp_rv = 0.0;

        locals.var_emin = 15.0;
        locals.var_emin_dn4 = 0.0;
        locals.var_emin_dn6 = 0.0;
        locals.var_emin_dn7 = 0.0;
        locals.var_emin_dn8 = 0.0;
        locals.var_emin_dn9 = 0.0;
        locals.var_emin_rv = 0.0;

        let assign7120_e6808: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard144 = assign7120_e6808;
        locals.var_guard144_rv = 0.0;

        let (assign7130_e6833, assign7130_e6833_d_n4, assign7130_e6833_d_n6, assign7130_e6833_d_n7, assign7130_e6833_d_n8, assign7130_e6833_d_n9,) = {
    if (locals.var_guard144 != 0.0) {
        let assign7130_e6814: f64 = (2970.0 / locals.var_tkd);
        let assign7130_e6815: f64 = (15.0 + assign7130_e6814);
        let assign7130_e6819: f64 = (2970.0 / locals.var_tkd);
        let assign7130_e6820: f64 = (15.0 - assign7130_e6819);
        let assign7130_e6824: f64 = (2970.0 / locals.var_tkd);
        let assign7130_e6825: f64 = (15.0 - assign7130_e6824);
        let assign7130_e6826: f64 = (assign7130_e6820 * assign7130_e6825);
        let assign7130_e6828: f64 = (assign7130_e6826 + 1e-6);
        let assign7130_e6829: f64 = (assign7130_e6828).sqrt();
        let assign7130_e6830: f64 = (assign7130_e6815 + assign7130_e6829);
        let assign7130_e6831: f64 = (0.5 * assign7130_e6830);
        (assign7130_e6831, (0.5 * ((-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))) * assign7130_e6825) + (assign7130_e6820 * (-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7130_e6829)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))) * assign7130_e6825) + (assign7130_e6820 * (-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7130_e6829)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))) * assign7130_e6825) + (assign7130_e6820 * (-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7130_e6829)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))) * assign7130_e6825) + (assign7130_e6820 * (-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7130_e6829)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))) * assign7130_e6825) + (assign7130_e6820 * (-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7130_e6829)))),)
    } else {
        (locals.var_emin, locals.var_emin_dn4, locals.var_emin_dn6, locals.var_emin_dn7, locals.var_emin_dn8, locals.var_emin_dn9,)
    }
};
        locals.var_emin = assign7130_e6833;
        locals.var_emin_dn4 = assign7130_e6833_d_n4;
        locals.var_emin_dn6 = assign7130_e6833_d_n6;
        locals.var_emin_dn7 = assign7130_e6833_d_n7;
        locals.var_emin_dn8 = assign7130_e6833_d_n8;
        locals.var_emin_dn9 = assign7130_e6833_d_n9;
        locals.var_emin_rv = 0.0;

        locals.var_dvfbqm = 0.0;
        locals.var_dvfbqm_rv = 0.0;

        locals.var_qq = 0.0;
        locals.var_qq_dn4 = 0.0;
        locals.var_qq_dn6 = 0.0;
        locals.var_qq_dn7 = 0.0;
        locals.var_qq_dn8 = 0.0;
        locals.var_qq_dn9 = 0.0;
        locals.var_qq_rv = 0.0;

        let assign7160_e6838: f64 = (1e18 * locals.var_tsi_i);
        let assign7160_e6840: f64 = (assign7160_e6838 * locals.var_tsi_i);
        locals.var_tsisq = assign7160_e6840;
        locals.var_tsisq_rv = 0.0;

        let assign7170_e6843: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard145 = assign7170_e6843;
        locals.var_guard145_rv = 0.0;

        let assign7180_e6846: f64 = 1.0;
        let assign7180_e6847: f64 = if p.p14 == assign7180_e6846 { 1.0 } else { 0.0 };
        locals.var_guard146 = assign7180_e6847;
        locals.var_guard146_rv = 0.0;

        let (assign7190_e6855,) = {
    if ((locals.var_guard145 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7190_e6853: f64 = (0.409618895 / locals.var_tsisq);
        (assign7190_e6853,)
    } else {
        (locals.var_dvfbqm,)
    }
};
        locals.var_dvfbqm = assign7190_e6855;
        locals.var_dvfbqm_rv = 0.0;

        let (assign7200_e6874, assign7200_e6874_d_n4, assign7200_e6874_d_n6, assign7200_e6874_d_n7, assign7200_e6874_d_n8, assign7200_e6874_d_n9,) = {
    if ((locals.var_guard145 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7200_e6861: f64 = (0.4 * p.p13);
        let assign7200_e6863: f64 = (assign7200_e6861 * 1.27520989);
        let assign7200_e6865: f64 = (-0.3333333333333);
        let assign7200_e6868: f64 = (locals.var_phit * locals.var_tsisq);
        let assign7200_e6869: f64 = (assign7200_e6868).ln();
        let assign7200_e6870: f64 = (assign7200_e6865 * assign7200_e6869);
        let assign7200_e6871: f64 = (assign7200_e6870).exp();
        let assign7200_e6872: f64 = (assign7200_e6863 * assign7200_e6871);
        (assign7200_e6872, (assign7200_e6863 * (assign7200_e6871 * (assign7200_e6865 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign7200_e6868)))), (assign7200_e6863 * (assign7200_e6871 * (assign7200_e6865 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign7200_e6868)))), (assign7200_e6863 * (assign7200_e6871 * (assign7200_e6865 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign7200_e6868)))), (assign7200_e6863 * (assign7200_e6871 * (assign7200_e6865 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign7200_e6868)))), (assign7200_e6863 * (assign7200_e6871 * (assign7200_e6865 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign7200_e6868)))),)
    } else {
        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9,)
    }
};
        locals.var_qq = assign7200_e6874;
        locals.var_qq_dn4 = assign7200_e6874_d_n4;
        locals.var_qq_dn6 = assign7200_e6874_d_n6;
        locals.var_qq_dn7 = assign7200_e6874_d_n7;
        locals.var_qq_dn8 = assign7200_e6874_d_n8;
        locals.var_qq_dn9 = assign7200_e6874_d_n9;
        locals.var_qq_rv = 0.0;

        let (assign7210_e6883,) = {
    if ((locals.var_guard145 != 0.0) && (locals.var_guard146 == 0.0)) {
        let assign7210_e6881: f64 = (0.723134895 / locals.var_tsisq);
        (assign7210_e6881,)
    } else {
        (locals.var_dvfbqm,)
    }
};
        locals.var_dvfbqm = assign7210_e6883;
        locals.var_dvfbqm_rv = 0.0;

        let (assign7220_e6903, assign7220_e6903_d_n4, assign7220_e6903_d_n6, assign7220_e6903_d_n7, assign7220_e6903_d_n8, assign7220_e6903_d_n9,) = {
    if ((locals.var_guard145 != 0.0) && (locals.var_guard146 == 0.0)) {
        let assign7220_e6890: f64 = (0.4 * p.p13);
        let assign7220_e6892: f64 = (assign7220_e6890 * 1.5412087);
        let assign7220_e6894: f64 = (-0.3333333333333);
        let assign7220_e6897: f64 = (locals.var_phit * locals.var_tsisq);
        let assign7220_e6898: f64 = (assign7220_e6897).ln();
        let assign7220_e6899: f64 = (assign7220_e6894 * assign7220_e6898);
        let assign7220_e6900: f64 = (assign7220_e6899).exp();
        let assign7220_e6901: f64 = (assign7220_e6892 * assign7220_e6900);
        (assign7220_e6901, (assign7220_e6892 * (assign7220_e6900 * (assign7220_e6894 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign7220_e6897)))), (assign7220_e6892 * (assign7220_e6900 * (assign7220_e6894 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign7220_e6897)))), (assign7220_e6892 * (assign7220_e6900 * (assign7220_e6894 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign7220_e6897)))), (assign7220_e6892 * (assign7220_e6900 * (assign7220_e6894 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign7220_e6897)))), (assign7220_e6892 * (assign7220_e6900 * (assign7220_e6894 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign7220_e6897)))),)
    } else {
        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9,)
    }
};
        locals.var_qq = assign7220_e6903;
        locals.var_qq_dn4 = assign7220_e6903_d_n4;
        locals.var_qq_dn6 = assign7220_e6903_d_n6;
        locals.var_qq_dn7 = assign7220_e6903_d_n7;
        locals.var_qq_dn8 = assign7220_e6903_d_n8;
        locals.var_qq_dn9 = assign7220_e6903_d_n9;
        locals.var_qq_rv = 0.0;

        let assign7230_e6906: f64 = (p.p14 * locals.var_stvfb_i);
        let assign7230_e6908: f64 = (assign7230_e6906 * locals.var_dt);
        let assign7230_e6910: f64 = (assign7230_e6908 + locals.var_dvfbqm);
        locals.var_temp = assign7230_e6910;
        locals.var_temp_dn4 = (assign7230_e6906 * locals.var_dt_dn4);
        locals.var_temp_dn6 = (assign7230_e6906 * locals.var_dt_dn6);
        locals.var_temp_dn7 = (assign7230_e6906 * locals.var_dt_dn7);
        locals.var_temp_dn8 = (assign7230_e6906 * locals.var_dt_dn8);
        locals.var_temp_dn9 = (assign7230_e6906 * locals.var_dt_dn9);
        locals.var_temp_rv = 0.0;

        let assign7240_e6913: f64 = (locals.var_temp + p.p34);
        let assign7240_e6915: f64 = (assign7240_e6913 - locals.var_dvfbpdep);
        locals.var_temp1 = assign7240_e6915;
        locals.var_temp1_dn4 = (locals.var_temp_dn4 - locals.var_dvfbpdep_dn4);
        locals.var_temp1_dn6 = (locals.var_temp_dn6 - locals.var_dvfbpdep_dn6);
        locals.var_temp1_dn7 = (locals.var_temp_dn7 - locals.var_dvfbpdep_dn7);
        locals.var_temp1_dn8 = (locals.var_temp_dn8 - locals.var_dvfbpdep_dn8);
        locals.var_temp1_dn9 = (locals.var_temp_dn9 - locals.var_dvfbpdep_dn9);
        locals.var_temp1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign7250_e6919: f64 = (locals.var_vfb1_t + locals.var_dvfbch);
        let assign7250_e6921: f64 = (assign7250_e6919 + locals.var_dvfb1nch);
        let assign7250_e6922: f64 = (p.p14 * assign7250_e6921);
        let assign7250_e6924: f64 = (assign7250_e6922 + locals.var_temp1);
        locals.var_vfb1_i = assign7250_e6924;
        locals.var_vfb1_i_dn4 = ((p.p14 * ((locals.var_vfb1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4);
        locals.var_vfb1_i_dn6 = ((p.p14 * ((locals.var_vfb1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6);
        locals.var_vfb1_i_dn7 = ((p.p14 * ((locals.var_vfb1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7);
        locals.var_vfb1_i_dn8 = ((p.p14 * ((locals.var_vfb1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8);
        locals.var_vfb1_i_dn9 = ((p.p14 * ((locals.var_vfb1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9);
        locals.var_vfb1_i_rv = 0.0;

        let assign7260_e6928: f64 = (locals.var_vfb2_t + locals.var_dvfbch);
        let assign7260_e6930: f64 = (assign7260_e6928 + locals.var_dvfb2nch);
        let assign7260_e6931: f64 = (p.p14 * assign7260_e6930);
        let assign7260_e6933: f64 = (assign7260_e6931 + locals.var_temp);
        locals.var_vfb2_i = assign7260_e6933;
        locals.var_vfb2_i_dn4 = ((p.p14 * ((locals.var_vfb2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4);
        locals.var_vfb2_i_dn6 = ((p.p14 * ((locals.var_vfb2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6);
        locals.var_vfb2_i_dn7 = ((p.p14 * ((locals.var_vfb2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7);
        locals.var_vfb2_i_dn8 = ((p.p14 * ((locals.var_vfb2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8);
        locals.var_vfb2_i_dn9 = ((p.p14 * ((locals.var_vfb2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9);
        locals.var_vfb2_i_rv = 0.0;

        let assign7270_e6937: f64 = (locals.var_vfbac1_t + locals.var_dvfbch);
        let assign7270_e6939: f64 = (assign7270_e6937 + locals.var_dvfb1nch);
        let assign7270_e6940: f64 = (p.p14 * assign7270_e6939);
        let assign7270_e6942: f64 = (assign7270_e6940 + locals.var_temp1);
        locals.var_vfbac1_i = assign7270_e6942;
        locals.var_vfbac1_i_dn4 = ((p.p14 * ((locals.var_vfbac1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4);
        locals.var_vfbac1_i_dn6 = ((p.p14 * ((locals.var_vfbac1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6);
        locals.var_vfbac1_i_dn7 = ((p.p14 * ((locals.var_vfbac1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7);
        locals.var_vfbac1_i_dn8 = ((p.p14 * ((locals.var_vfbac1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8);
        locals.var_vfbac1_i_dn9 = ((p.p14 * ((locals.var_vfbac1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9);
        locals.var_vfbac1_i_rv = 0.0;

        let assign7280_e6946: f64 = (locals.var_vfbac2_t + locals.var_dvfbch);
        let assign7280_e6948: f64 = (assign7280_e6946 + locals.var_dvfb2nch);
        let assign7280_e6949: f64 = (p.p14 * assign7280_e6948);
        let assign7280_e6951: f64 = (assign7280_e6949 + locals.var_temp);
        locals.var_vfbac2_i = assign7280_e6951;
        locals.var_vfbac2_i_dn4 = ((p.p14 * ((locals.var_vfbac2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4);
        locals.var_vfbac2_i_dn6 = ((p.p14 * ((locals.var_vfbac2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6);
        locals.var_vfbac2_i_dn7 = ((p.p14 * ((locals.var_vfbac2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7);
        locals.var_vfbac2_i_dn8 = ((p.p14 * ((locals.var_vfbac2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8);
        locals.var_vfbac2_i_dn9 = ((p.p14 * ((locals.var_vfbac2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9);
        locals.var_vfbac2_i_rv = 0.0;

        let assign7290_e6953: f64 = (locals.var_rtn).ln();
        locals.var_lnrtn = assign7290_e6953;
        locals.var_lnrtn_dn4 = (locals.var_rtn_dn4 / locals.var_rtn);
        locals.var_lnrtn_dn6 = (locals.var_rtn_dn6 / locals.var_rtn);
        locals.var_lnrtn_dn7 = (locals.var_rtn_dn7 / locals.var_rtn);
        locals.var_lnrtn_dn8 = (locals.var_rtn_dn8 / locals.var_rtn);
        locals.var_lnrtn_dn9 = (locals.var_rtn_dn9 / locals.var_rtn);
        locals.var_lnrtn_rv = 0.0;

        let assign7300_e6956: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign7300_e6957: f64 = (assign7300_e6956).exp();
        let assign7300_e6959: f64 = (assign7300_e6957 * p.p35);
        locals.var_tf_bet = assign7300_e6959;
        locals.var_tf_bet_dn4 = ((assign7300_e6957 * (locals.var_stbet_i * locals.var_lnrtn_dn4)) * p.p35);
        locals.var_tf_bet_dn6 = ((assign7300_e6957 * (locals.var_stbet_i * locals.var_lnrtn_dn6)) * p.p35);
        locals.var_tf_bet_dn7 = ((assign7300_e6957 * (locals.var_stbet_i * locals.var_lnrtn_dn7)) * p.p35);
        locals.var_tf_bet_dn8 = ((assign7300_e6957 * (locals.var_stbet_i * locals.var_lnrtn_dn8)) * p.p35);
        locals.var_tf_bet_dn9 = ((assign7300_e6957 * (locals.var_stbet_i * locals.var_lnrtn_dn9)) * p.p35);
        locals.var_tf_bet_rv = 0.0;

        let assign7310_e6962: f64 = (locals.var_betn1_t * locals.var_tf_bet);
        locals.var_betn1_i = assign7310_e6962;
        locals.var_betn1_i_dn4 = ((locals.var_betn1_t_dn4 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn4));
        locals.var_betn1_i_dn6 = ((locals.var_betn1_t_dn6 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn6));
        locals.var_betn1_i_dn7 = ((locals.var_betn1_t_dn7 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn7));
        locals.var_betn1_i_dn8 = ((locals.var_betn1_t_dn8 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn8));
        locals.var_betn1_i_dn9 = ((locals.var_betn1_t_dn9 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn9));
        locals.var_betn1_i_rv = 0.0;

        let assign7320_e6965: f64 = (locals.var_betn2_t * locals.var_tf_bet);
        locals.var_betn2_i = assign7320_e6965;
        locals.var_betn2_i_dn4 = ((locals.var_betn2_t_dn4 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn4));
        locals.var_betn2_i_dn6 = ((locals.var_betn2_t_dn6 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn6));
        locals.var_betn2_i_dn7 = ((locals.var_betn2_t_dn7 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn7));
        locals.var_betn2_i_dn8 = ((locals.var_betn2_t_dn8 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn8));
        locals.var_betn2_i_dn9 = ((locals.var_betn2_t_dn9 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn9));
        locals.var_betn2_i_rv = 0.0;

        let assign7330_e6968: f64 = (locals.var_stmue_i * locals.var_lnrtn);
        let assign7330_e6969: f64 = (assign7330_e6968).exp();
        locals.var_tf_mue = assign7330_e6969;
        locals.var_tf_mue_dn4 = (assign7330_e6969 * (locals.var_stmue_i * locals.var_lnrtn_dn4));
        locals.var_tf_mue_dn6 = (assign7330_e6969 * (locals.var_stmue_i * locals.var_lnrtn_dn6));
        locals.var_tf_mue_dn7 = (assign7330_e6969 * (locals.var_stmue_i * locals.var_lnrtn_dn7));
        locals.var_tf_mue_dn8 = (assign7330_e6969 * (locals.var_stmue_i * locals.var_lnrtn_dn8));
        locals.var_tf_mue_dn9 = (assign7330_e6969 * (locals.var_stmue_i * locals.var_lnrtn_dn9));
        locals.var_tf_mue_rv = 0.0;

        let assign7340_e6972: f64 = (locals.var_mue_t * locals.var_tf_mue);
        locals.var_mue_i = assign7340_e6972;
        locals.var_mue_i_dn4 = (locals.var_mue_t * locals.var_tf_mue_dn4);
        locals.var_mue_i_dn6 = (locals.var_mue_t * locals.var_tf_mue_dn6);
        locals.var_mue_i_dn7 = (locals.var_mue_t * locals.var_tf_mue_dn7);
        locals.var_mue_i_dn8 = (locals.var_mue_t * locals.var_tf_mue_dn8);
        locals.var_mue_i_dn9 = (locals.var_mue_t * locals.var_tf_mue_dn9);
        locals.var_mue_i_rv = 0.0;

        let assign7350_e6975: f64 = (locals.var_stthemu_i * locals.var_lnrtn);
        let assign7350_e6976: f64 = (assign7350_e6975).exp();
        locals.var_tf_themu = assign7350_e6976;
        locals.var_tf_themu_dn4 = (assign7350_e6976 * (locals.var_stthemu_i * locals.var_lnrtn_dn4));
        locals.var_tf_themu_dn6 = (assign7350_e6976 * (locals.var_stthemu_i * locals.var_lnrtn_dn6));
        locals.var_tf_themu_dn7 = (assign7350_e6976 * (locals.var_stthemu_i * locals.var_lnrtn_dn7));
        locals.var_tf_themu_dn8 = (assign7350_e6976 * (locals.var_stthemu_i * locals.var_lnrtn_dn8));
        locals.var_tf_themu_dn9 = (assign7350_e6976 * (locals.var_stthemu_i * locals.var_lnrtn_dn9));
        locals.var_tf_themu_rv = 0.0;

        let assign7360_e6979: f64 = (locals.var_themu_t * locals.var_tf_themu);
        locals.var_themu_i = assign7360_e6979;
        locals.var_themu_i_dn4 = (locals.var_themu_t * locals.var_tf_themu_dn4);
        locals.var_themu_i_dn6 = (locals.var_themu_t * locals.var_tf_themu_dn6);
        locals.var_themu_i_dn7 = (locals.var_themu_t * locals.var_tf_themu_dn7);
        locals.var_themu_i_dn8 = (locals.var_themu_t * locals.var_tf_themu_dn8);
        locals.var_themu_i_dn9 = (locals.var_themu_t * locals.var_tf_themu_dn9);
        locals.var_themu_i_rv = 0.0;

        let assign7370_e6982: f64 = (locals.var_stcs_i * locals.var_lnrtn);
        let assign7370_e6983: f64 = (assign7370_e6982).exp();
        locals.var_tf_cs = assign7370_e6983;
        locals.var_tf_cs_dn4 = (assign7370_e6983 * (locals.var_stcs_i * locals.var_lnrtn_dn4));
        locals.var_tf_cs_dn6 = (assign7370_e6983 * (locals.var_stcs_i * locals.var_lnrtn_dn6));
        locals.var_tf_cs_dn7 = (assign7370_e6983 * (locals.var_stcs_i * locals.var_lnrtn_dn7));
        locals.var_tf_cs_dn8 = (assign7370_e6983 * (locals.var_stcs_i * locals.var_lnrtn_dn8));
        locals.var_tf_cs_dn9 = (assign7370_e6983 * (locals.var_stcs_i * locals.var_lnrtn_dn9));
        locals.var_tf_cs_rv = 0.0;

        let assign7380_e6986: f64 = (locals.var_cs_t * locals.var_tf_cs);
        locals.var_cs_i = assign7380_e6986;
        locals.var_cs_i_dn4 = (locals.var_cs_t * locals.var_tf_cs_dn4);
        locals.var_cs_i_dn6 = (locals.var_cs_t * locals.var_tf_cs_dn6);
        locals.var_cs_i_dn7 = (locals.var_cs_t * locals.var_tf_cs_dn7);
        locals.var_cs_i_dn8 = (locals.var_cs_t * locals.var_tf_cs_dn8);
        locals.var_cs_i_dn9 = (locals.var_cs_t * locals.var_tf_cs_dn9);
        locals.var_cs_i_rv = 0.0;

        let assign7390_e6989: f64 = (locals.var_stthecs_i * locals.var_lnrtn);
        let assign7390_e6990: f64 = (assign7390_e6989).exp();
        locals.var_tf_thecs = assign7390_e6990;
        locals.var_tf_thecs_dn4 = (assign7390_e6990 * (locals.var_stthecs_i * locals.var_lnrtn_dn4));
        locals.var_tf_thecs_dn6 = (assign7390_e6990 * (locals.var_stthecs_i * locals.var_lnrtn_dn6));
        locals.var_tf_thecs_dn7 = (assign7390_e6990 * (locals.var_stthecs_i * locals.var_lnrtn_dn7));
        locals.var_tf_thecs_dn8 = (assign7390_e6990 * (locals.var_stthecs_i * locals.var_lnrtn_dn8));
        locals.var_tf_thecs_dn9 = (assign7390_e6990 * (locals.var_stthecs_i * locals.var_lnrtn_dn9));
        locals.var_tf_thecs_rv = 0.0;

        let assign7400_e6993: f64 = (locals.var_thecs_t * locals.var_tf_thecs);
        locals.var_thecs_i = assign7400_e6993;
        locals.var_thecs_i_dn4 = (locals.var_thecs_t * locals.var_tf_thecs_dn4);
        locals.var_thecs_i_dn6 = (locals.var_thecs_t * locals.var_tf_thecs_dn6);
        locals.var_thecs_i_dn7 = (locals.var_thecs_t * locals.var_tf_thecs_dn7);
        locals.var_thecs_i_dn8 = (locals.var_thecs_t * locals.var_tf_thecs_dn8);
        locals.var_thecs_i_dn9 = (locals.var_thecs_t * locals.var_tf_thecs_dn9);
        locals.var_thecs_i_rv = 0.0;

        let assign7410_e6996: f64 = (locals.var_stxcor_i * locals.var_lnrtn);
        let assign7410_e6997: f64 = (assign7410_e6996).exp();
        locals.var_tf_xcor = assign7410_e6997;
        locals.var_tf_xcor_dn4 = (assign7410_e6997 * (locals.var_stxcor_i * locals.var_lnrtn_dn4));
        locals.var_tf_xcor_dn6 = (assign7410_e6997 * (locals.var_stxcor_i * locals.var_lnrtn_dn6));
        locals.var_tf_xcor_dn7 = (assign7410_e6997 * (locals.var_stxcor_i * locals.var_lnrtn_dn7));
        locals.var_tf_xcor_dn8 = (assign7410_e6997 * (locals.var_stxcor_i * locals.var_lnrtn_dn8));
        locals.var_tf_xcor_dn9 = (assign7410_e6997 * (locals.var_stxcor_i * locals.var_lnrtn_dn9));
        locals.var_tf_xcor_rv = 0.0;

        let assign7420_e7000: f64 = (locals.var_xcor_t * locals.var_tf_xcor);
        locals.var_xcor_i = assign7420_e7000;
        locals.var_xcor_i_dn4 = (locals.var_xcor_t * locals.var_tf_xcor_dn4);
        locals.var_xcor_i_dn6 = (locals.var_xcor_t * locals.var_tf_xcor_dn6);
        locals.var_xcor_i_dn7 = (locals.var_xcor_t * locals.var_tf_xcor_dn7);
        locals.var_xcor_i_dn8 = (locals.var_xcor_t * locals.var_tf_xcor_dn8);
        locals.var_xcor_i_dn9 = (locals.var_xcor_t * locals.var_tf_xcor_dn9);
        locals.var_xcor_i_rv = 0.0;

        let assign7430_e7003: f64 = (1e-8 * locals.var_phit);
        let assign7430_e7005: f64 = (assign7430_e7003 / locals.var_tsi_i);
        locals.var_temp = assign7430_e7005;
        locals.var_temp_dn4 = ((1e-8 * locals.var_phit_dn4) / locals.var_tsi_i);
        locals.var_temp_dn6 = ((1e-8 * locals.var_phit_dn6) / locals.var_tsi_i);
        locals.var_temp_dn7 = ((1e-8 * locals.var_phit_dn7) / locals.var_tsi_i);
        locals.var_temp_dn8 = ((1e-8 * locals.var_phit_dn8) / locals.var_tsi_i);
        locals.var_temp_dn9 = ((1e-8 * locals.var_phit_dn9) / locals.var_tsi_i);
        locals.var_temp_rv = 0.0;

        let assign7440_e7008: f64 = (locals.var_temp * locals.var_mue_i);
        locals.var_fmue = assign7440_e7008;
        locals.var_fmue_dn4 = ((locals.var_temp_dn4 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn4));
        locals.var_fmue_dn6 = ((locals.var_temp_dn6 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn6));
        locals.var_fmue_dn7 = ((locals.var_temp_dn7 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn7));
        locals.var_fmue_dn8 = ((locals.var_temp_dn8 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn8));
        locals.var_fmue_dn9 = ((locals.var_temp_dn9 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn9));
        locals.var_fmue_rv = 0.0;

        let assign7450_e7012: f64 = (0.5 * locals.var_csthr_i);
        let assign7450_e7013: f64 = (1.0 / assign7450_e7012);
        locals.var_inv_qi1cs = assign7450_e7013;
        locals.var_inv_qi1cs_rv = 0.0;

        let assign7460_e7016: f64 = (locals.var_inv_qi1cs / locals.var_csthrb_i);
        locals.var_inv_qi2cs = assign7460_e7016;
        locals.var_inv_qi2cs_rv = 0.0;

        let assign7470_e7019: f64 = 1.0;
        let assign7470_e7020: f64 = if p.p14 == assign7470_e7019 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign7470_e7020;
        locals.var_guard147_rv = 0.0;

        let (assign7480_e7026,) = {
    if (locals.var_guard147 != 0.0) {
        let assign7480_e7024: f64 = (0.5 * locals.var_feta_i);
        (assign7480_e7024,)
    } else {
        (locals.var_eta_mu,)
    }
};
        locals.var_eta_mu = assign7480_e7026;
        locals.var_eta_mu_rv = 0.0;

        let (assign7490_e7033,) = {
    if (locals.var_guard147 == 0.0) {
        let assign7490_e7031: f64 = (0.3333333333333 * locals.var_feta_i);
        (assign7490_e7031,)
    } else {
        (locals.var_eta_mu,)
    }
};
        locals.var_eta_mu = assign7490_e7033;
        locals.var_eta_mu_rv = 0.0;

        let assign7500_e7036: f64 = (1.0 - locals.var_eta_mu);
        locals.var_one_m_eta = assign7500_e7036;
        locals.var_one_m_eta_rv = 0.0;

        let assign7510_e7039: f64 = (locals.var_strs_i * locals.var_lnrtn);
        let assign7510_e7040: f64 = (assign7510_e7039).exp();
        locals.var_tf_ther = assign7510_e7040;
        locals.var_tf_ther_dn4 = (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn4));
        locals.var_tf_ther_dn6 = (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn6));
        locals.var_tf_ther_dn7 = (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn7));
        locals.var_tf_ther_dn8 = (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn8));
        locals.var_tf_ther_dn9 = (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn9));
        locals.var_tf_ther_rv = 0.0;

        let assign7520_e7043: f64 = (locals.var_rs_t * locals.var_tf_ther);
        locals.var_rs_i = assign7520_e7043;
        locals.var_rs_i_dn4 = (locals.var_rs_t * locals.var_tf_ther_dn4);
        locals.var_rs_i_dn6 = (locals.var_rs_t * locals.var_tf_ther_dn6);
        locals.var_rs_i_dn7 = (locals.var_rs_t * locals.var_tf_ther_dn7);
        locals.var_rs_i_dn8 = (locals.var_rs_t * locals.var_tf_ther_dn8);
        locals.var_rs_i_dn9 = (locals.var_rs_t * locals.var_tf_ther_dn9);
        locals.var_rs_i_rv = 0.0;

        let assign7530_e7046: f64 = (2.0 * locals.var_rs_i);
        let assign7530_e7048: f64 = (assign7530_e7046 * locals.var_phit);
        locals.var_frs = assign7530_e7048;
        locals.var_frs_dn4 = (((2.0 * locals.var_rs_i_dn4) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn4));
        locals.var_frs_dn6 = (((2.0 * locals.var_rs_i_dn6) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn6));
        locals.var_frs_dn7 = (((2.0 * locals.var_rs_i_dn7) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn7));
        locals.var_frs_dn8 = (((2.0 * locals.var_rs_i_dn8) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn8));
        locals.var_frs_dn9 = (((2.0 * locals.var_rs_i_dn9) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn9));
        locals.var_frs_rv = 0.0;

        let assign7540_e7052: f64 = (16.0 / locals.var_ax_i);
        let assign7540_e7054: f64 = (assign7540_e7052 * 0.6931471805599);
        let assign7540_e7055: f64 = (assign7540_e7054).exp();
        let assign7540_e7057: f64 = (assign7540_e7055 - 1.0);
        let assign7540_e7058: f64 = (assign7540_e7057).ln();
        let assign7540_e7059: f64 = (0.375 * assign7540_e7058);
        let assign7540_e7060: f64 = (assign7540_e7059).exp();
        let assign7540_e7062: f64 = (assign7540_e7060 - 1.0);
        locals.var_gamax = assign7540_e7062;
        locals.var_gamax_rv = 0.0;

        let assign7550_e7066: f64 = (16.0 / locals.var_axac_i);
        let assign7550_e7068: f64 = (assign7550_e7066 * 0.6931471805599);
        let assign7550_e7069: f64 = (assign7550_e7068).exp();
        let assign7550_e7071: f64 = (assign7550_e7069 - 1.0);
        let assign7550_e7072: f64 = (assign7550_e7071).ln();
        let assign7550_e7073: f64 = (0.375 * assign7550_e7072);
        let assign7550_e7074: f64 = (assign7550_e7073).exp();
        let assign7550_e7076: f64 = (assign7550_e7074 - 1.0);
        locals.var_gamax_ac = assign7550_e7076;
        locals.var_gamax_ac_rv = 0.0;

        let assign7560_e7079: f64 = (locals.var_stthesat_i * locals.var_lnrtn);
        let assign7560_e7080: f64 = (assign7560_e7079).exp();
        locals.var_tf_thesat = assign7560_e7080;
        locals.var_tf_thesat_dn4 = (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn4));
        locals.var_tf_thesat_dn6 = (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn6));
        locals.var_tf_thesat_dn7 = (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn7));
        locals.var_tf_thesat_dn8 = (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn8));
        locals.var_tf_thesat_dn9 = (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn9));
        locals.var_tf_thesat_rv = 0.0;

        let assign7570_e7083: f64 = (locals.var_thesat_t * locals.var_tf_thesat);
        let assign7570_e7085: f64 = (assign7570_e7083 * locals.var_tf_bet);
        locals.var_thesat_i = assign7570_e7085;
        locals.var_thesat_i_dn4 = ((((locals.var_thesat_t_dn4 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn4));
        locals.var_thesat_i_dn6 = ((((locals.var_thesat_t_dn6 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn6));
        locals.var_thesat_i_dn7 = ((((locals.var_thesat_t_dn7 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn7));
        locals.var_thesat_i_dn8 = ((((locals.var_thesat_t_dn8 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn8));
        locals.var_thesat_i_dn9 = ((((locals.var_thesat_t_dn9 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn9));
        locals.var_thesat_i_rv = 0.0;

        let assign7580_e7088: f64 = (locals.var_thesat_i * locals.var_phit);
        locals.var_sat_phit = assign7580_e7088;
        locals.var_sat_phit_dn4 = ((locals.var_thesat_i_dn4 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn4));
        locals.var_sat_phit_dn6 = ((locals.var_thesat_i_dn6 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn6));
        locals.var_sat_phit_dn7 = ((locals.var_thesat_i_dn7 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn7));
        locals.var_sat_phit_dn8 = ((locals.var_thesat_i_dn8 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn8));
        locals.var_sat_phit_dn9 = ((locals.var_thesat_i_dn9 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn9));
        locals.var_sat_phit_rv = 0.0;

        let assign7590_e7091: f64 = (locals.var_thesatac_t * locals.var_tf_thesat);
        let assign7590_e7093: f64 = (assign7590_e7091 * locals.var_tf_bet);
        locals.var_thesatac_i = assign7590_e7093;
        locals.var_thesatac_i_dn4 = ((((locals.var_thesatac_t_dn4 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn4));
        locals.var_thesatac_i_dn6 = ((((locals.var_thesatac_t_dn6 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn6));
        locals.var_thesatac_i_dn7 = ((((locals.var_thesatac_t_dn7 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn7));
        locals.var_thesatac_i_dn8 = ((((locals.var_thesatac_t_dn8 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn8));
        locals.var_thesatac_i_dn9 = ((((locals.var_thesatac_t_dn9 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn9));
        locals.var_thesatac_i_rv = 0.0;

        let assign7600_e7096: f64 = (locals.var_thesatac_i * locals.var_phit);
        locals.var_sat_phit_ac = assign7600_e7096;
        locals.var_sat_phit_ac_dn4 = ((locals.var_thesatac_i_dn4 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn4));
        locals.var_sat_phit_ac_dn6 = ((locals.var_thesatac_i_dn6 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn6));
        locals.var_sat_phit_ac_dn7 = ((locals.var_thesatac_i_dn7 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn7));
        locals.var_sat_phit_ac_dn8 = ((locals.var_thesatac_i_dn8 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn8));
        locals.var_sat_phit_ac_dn9 = ((locals.var_thesatac_i_dn9 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn9));
        locals.var_sat_phit_ac_rv = 0.0;

        let assign7610_e7099: f64 = (locals.var_alp1_i * locals.var_inv_phit);
        locals.var_alp1_phit = assign7610_e7099;
        locals.var_alp1_phit_dn4 = (locals.var_alp1_i * locals.var_inv_phit_dn4);
        locals.var_alp1_phit_dn6 = (locals.var_alp1_i * locals.var_inv_phit_dn6);
        locals.var_alp1_phit_dn7 = (locals.var_alp1_i * locals.var_inv_phit_dn7);
        locals.var_alp1_phit_dn8 = (locals.var_alp1_i * locals.var_inv_phit_dn8);
        locals.var_alp1_phit_dn9 = (locals.var_alp1_i * locals.var_inv_phit_dn9);
        locals.var_alp1_phit_rv = 0.0;

        let assign7620_e7101: f64 = (-locals.var_stig_i);
        let assign7620_e7103: f64 = (assign7620_e7101 * locals.var_lnrtn);
        let assign7620_e7104: f64 = (assign7620_e7103).exp();
        locals.var_tf_ig = assign7620_e7104;
        locals.var_tf_ig_dn4 = (assign7620_e7104 * (assign7620_e7101 * locals.var_lnrtn_dn4));
        locals.var_tf_ig_dn6 = (assign7620_e7104 * (assign7620_e7101 * locals.var_lnrtn_dn6));
        locals.var_tf_ig_dn7 = (assign7620_e7104 * (assign7620_e7101 * locals.var_lnrtn_dn7));
        locals.var_tf_ig_dn8 = (assign7620_e7104 * (assign7620_e7101 * locals.var_lnrtn_dn8));
        locals.var_tf_ig_dn9 = (assign7620_e7104 * (assign7620_e7101 * locals.var_lnrtn_dn9));
        locals.var_tf_ig_rv = 0.0;

        let assign7630_e7107: f64 = (locals.var_iginv_t * locals.var_tf_ig);
        locals.var_iginv_i = assign7630_e7107;
        locals.var_iginv_i_dn4 = (locals.var_iginv_t * locals.var_tf_ig_dn4);
        locals.var_iginv_i_dn6 = (locals.var_iginv_t * locals.var_tf_ig_dn6);
        locals.var_iginv_i_dn7 = (locals.var_iginv_t * locals.var_tf_ig_dn7);
        locals.var_iginv_i_dn8 = (locals.var_iginv_t * locals.var_tf_ig_dn8);
        locals.var_iginv_i_dn9 = (locals.var_iginv_t * locals.var_tf_ig_dn9);
        locals.var_iginv_i_rv = 0.0;

        let assign7640_e7110: f64 = (locals.var_igovinv_t * locals.var_tf_ig);
        locals.var_igovinv_i = assign7640_e7110;
        locals.var_igovinv_i_dn4 = (locals.var_igovinv_t * locals.var_tf_ig_dn4);
        locals.var_igovinv_i_dn6 = (locals.var_igovinv_t * locals.var_tf_ig_dn6);
        locals.var_igovinv_i_dn7 = (locals.var_igovinv_t * locals.var_tf_ig_dn7);
        locals.var_igovinv_i_dn8 = (locals.var_igovinv_t * locals.var_tf_ig_dn8);
        locals.var_igovinv_i_dn9 = (locals.var_igovinv_t * locals.var_tf_ig_dn9);
        locals.var_igovinv_i_rv = 0.0;

        let assign7650_e7113: f64 = (locals.var_igovinvd_t * locals.var_tf_ig);
        locals.var_igovinvd_i = assign7650_e7113;
        locals.var_igovinvd_i_dn4 = (locals.var_igovinvd_t * locals.var_tf_ig_dn4);
        locals.var_igovinvd_i_dn6 = (locals.var_igovinvd_t * locals.var_tf_ig_dn6);
        locals.var_igovinvd_i_dn7 = (locals.var_igovinvd_t * locals.var_tf_ig_dn7);
        locals.var_igovinvd_i_dn8 = (locals.var_igovinvd_t * locals.var_tf_ig_dn8);
        locals.var_igovinvd_i_dn9 = (locals.var_igovinvd_t * locals.var_tf_ig_dn9);
        locals.var_igovinvd_i_rv = 0.0;

        let assign7660_e7116: f64 = (locals.var_igovacc_t * locals.var_tf_ig);
        locals.var_igovacc_i = assign7660_e7116;
        locals.var_igovacc_i_dn4 = (locals.var_igovacc_t * locals.var_tf_ig_dn4);
        locals.var_igovacc_i_dn6 = (locals.var_igovacc_t * locals.var_tf_ig_dn6);
        locals.var_igovacc_i_dn7 = (locals.var_igovacc_t * locals.var_tf_ig_dn7);
        locals.var_igovacc_i_dn8 = (locals.var_igovacc_t * locals.var_tf_ig_dn8);
        locals.var_igovacc_i_dn9 = (locals.var_igovacc_t * locals.var_tf_ig_dn9);
        locals.var_igovacc_i_rv = 0.0;

        let assign7670_e7119: f64 = (locals.var_igovaccd_t * locals.var_tf_ig);
        locals.var_igovaccd_i = assign7670_e7119;
        locals.var_igovaccd_i_dn4 = (locals.var_igovaccd_t * locals.var_tf_ig_dn4);
        locals.var_igovaccd_i_dn6 = (locals.var_igovaccd_t * locals.var_tf_ig_dn6);
        locals.var_igovaccd_i_dn7 = (locals.var_igovaccd_t * locals.var_tf_ig_dn7);
        locals.var_igovaccd_i_dn8 = (locals.var_igovaccd_t * locals.var_tf_ig_dn8);
        locals.var_igovaccd_i_dn9 = (locals.var_igovaccd_t * locals.var_tf_ig_dn9);
        locals.var_igovaccd_i_rv = 0.0;

        let assign7680_e7121: f64 = (-locals.var_stigfn_i);
        let assign7680_e7123: f64 = (assign7680_e7121 * locals.var_lnrtn);
        let assign7680_e7124: f64 = (assign7680_e7123).exp();
        locals.var_tf_ig = assign7680_e7124;
        locals.var_tf_ig_dn4 = (assign7680_e7124 * (assign7680_e7121 * locals.var_lnrtn_dn4));
        locals.var_tf_ig_dn6 = (assign7680_e7124 * (assign7680_e7121 * locals.var_lnrtn_dn6));
        locals.var_tf_ig_dn7 = (assign7680_e7124 * (assign7680_e7121 * locals.var_lnrtn_dn7));
        locals.var_tf_ig_dn8 = (assign7680_e7124 * (assign7680_e7121 * locals.var_lnrtn_dn8));
        locals.var_tf_ig_dn9 = (assign7680_e7124 * (assign7680_e7121 * locals.var_lnrtn_dn9));
        locals.var_tf_ig_rv = 0.0;

        let assign7710_e7133: f64 = (1.0 / locals.var_chib_i);
        locals.var_inv_chib = assign7710_e7133;
        locals.var_inv_chib_rv = 0.0;

        let assign7720_e7136: f64 = (4.0 * 0.3333333333333);
        let assign7720_e7139: f64 = (2.0 * 1.602176565e-19);
        let assign7720_e7141: f64 = (assign7720_e7139 * 9.10938291e-31);
        let assign7720_e7143: f64 = (assign7720_e7141 * locals.var_chib_i);
        let assign7720_e7144: f64 = (assign7720_e7143).sqrt();
        let assign7720_e7145: f64 = (assign7720_e7136 * assign7720_e7144);
        let assign7720_e7147: f64 = (assign7720_e7145 / 1.054571726e-34);
        locals.var_tempm = assign7720_e7147;
        locals.var_tempm_dn4 = 0.0;
        locals.var_tempm_dn6 = 0.0;
        locals.var_tempm_dn7 = 0.0;
        locals.var_tempm_dn8 = 0.0;
        locals.var_tempm_dn9 = 0.0;
        locals.var_tempm_rv = 0.0;

        let assign7730_e7150: f64 = (locals.var_tempm * locals.var_toxp_i);
        locals.var_bch = assign7730_e7150;
        locals.var_bch_dn4 = (locals.var_tempm_dn4 * locals.var_toxp_i);
        locals.var_bch_dn6 = (locals.var_tempm_dn6 * locals.var_toxp_i);
        locals.var_bch_dn7 = (locals.var_tempm_dn7 * locals.var_toxp_i);
        locals.var_bch_dn8 = (locals.var_tempm_dn8 * locals.var_toxp_i);
        locals.var_bch_dn9 = (locals.var_tempm_dn9 * locals.var_toxp_i);
        locals.var_bch_rv = 0.0;

        let assign7740_e7153: f64 = (locals.var_tempm * locals.var_toxp_i);
        locals.var_bov = assign7740_e7153;
        locals.var_bov_dn4 = (locals.var_tempm_dn4 * locals.var_toxp_i);
        locals.var_bov_dn6 = (locals.var_tempm_dn6 * locals.var_toxp_i);
        locals.var_bov_dn7 = (locals.var_tempm_dn7 * locals.var_toxp_i);
        locals.var_bov_dn8 = (locals.var_tempm_dn8 * locals.var_toxp_i);
        locals.var_bov_dn9 = (locals.var_tempm_dn9 * locals.var_toxp_i);
        locals.var_bov_rv = 0.0;

        locals.var_gcqch = 0.0;
        locals.var_gcqch_rv = 0.0;

        let assign7760_e7157: f64 = if locals.var_gc3ch_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign7760_e7157;
        locals.var_guard148_rv = 0.0;

        let (assign7770_e7166,) = {
    if (locals.var_guard148 != 0.0) {
        let assign7770_e7160: f64 = (-0.495);
        let assign7770_e7162: f64 = (assign7770_e7160 * locals.var_gc2ch_i);
        let assign7770_e7164: f64 = (assign7770_e7162 / locals.var_gc3ch_i);
        (assign7770_e7164,)
    } else {
        (locals.var_gcqch,)
    }
};
        locals.var_gcqch = assign7770_e7166;
        locals.var_gcqch_rv = 0.0;

        locals.var_gcqovinv = 0.0;
        locals.var_gcqovinv_rv = 0.0;

        let assign7790_e7170: f64 = if locals.var_gc3ovinv_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign7790_e7170;
        locals.var_guard149_rv = 0.0;

        let (assign7800_e7179,) = {
    if (locals.var_guard149 != 0.0) {
        let assign7800_e7173: f64 = (-0.495);
        let assign7800_e7175: f64 = (assign7800_e7173 * locals.var_gc2ovinv_i);
        let assign7800_e7177: f64 = (assign7800_e7175 / locals.var_gc3ovinv_i);
        (assign7800_e7177,)
    } else {
        (locals.var_gcqovinv,)
    }
};
        locals.var_gcqovinv = assign7800_e7179;
        locals.var_gcqovinv_rv = 0.0;

        locals.var_gcqovacc = 0.0;
        locals.var_gcqovacc_rv = 0.0;

        let assign7820_e7183: f64 = if locals.var_gc3ovacc_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign7820_e7183;
        locals.var_guard150_rv = 0.0;

        let (assign7830_e7192,) = {
    if (locals.var_guard150 != 0.0) {
        let assign7830_e7186: f64 = (-0.495);
        let assign7830_e7188: f64 = (assign7830_e7186 * locals.var_gc2ovacc_i);
        let assign7830_e7190: f64 = (assign7830_e7188 / locals.var_gc3ovacc_i);
        (assign7830_e7190,)
    } else {
        (locals.var_gcqovacc,)
    }
};
        locals.var_gcqovacc = assign7830_e7192;
        locals.var_gcqovacc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_18(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let assign7840_e7195: f64 = (0.5 * locals.var_eg);
        locals.var_alpha_b = assign7840_e7195;
        locals.var_alpha_b_dn4 = (0.5 * locals.var_eg_dn4);
        locals.var_alpha_b_dn6 = (0.5 * locals.var_eg_dn6);
        locals.var_alpha_b_dn7 = (0.5 * locals.var_eg_dn7);
        locals.var_alpha_b_dn8 = (0.5 * locals.var_eg_dn8);
        locals.var_alpha_b_dn9 = (0.5 * locals.var_eg_dn9);
        locals.var_alpha_b_rv = 0.0;

        let assign7850_e7198: f64 = (locals.var_gco_i * locals.var_phit);
        locals.var_dch = assign7850_e7198;
        locals.var_dch_dn4 = (locals.var_gco_i * locals.var_phit_dn4);
        locals.var_dch_dn6 = (locals.var_gco_i * locals.var_phit_dn6);
        locals.var_dch_dn7 = (locals.var_gco_i * locals.var_phit_dn7);
        locals.var_dch_dn8 = (locals.var_gco_i * locals.var_phit_dn8);
        locals.var_dch_dn9 = (locals.var_gco_i * locals.var_phit_dn9);
        locals.var_dch_rv = 0.0;

        let assign7860_e7201: f64 = (locals.var_gco_i * locals.var_phit0);
        locals.var_dov = assign7860_e7201;
        locals.var_dov_dn4 = (locals.var_gco_i * locals.var_phit0_dn4);
        locals.var_dov_dn6 = (locals.var_gco_i * locals.var_phit0_dn6);
        locals.var_dov_dn7 = (locals.var_gco_i * locals.var_phit0_dn7);
        locals.var_dov_dn8 = (locals.var_gco_i * locals.var_phit0_dn8);
        locals.var_dov_dn9 = (locals.var_gco_i * locals.var_phit0_dn9);
        locals.var_dov_rv = 0.0;

        let assign7870_e7206: f64 = (locals.var_niginv_i * locals.var_eg_2phit);
        let assign7870_e7207: f64 = (1.0 + assign7870_e7206);
        let assign7870_e7208: f64 = (1.0 / assign7870_e7207);
        locals.var_n_iginv = assign7870_e7208;
        locals.var_n_iginv_dn4 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn4) / (assign7870_e7207 * assign7870_e7207)));
        locals.var_n_iginv_dn6 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn6) / (assign7870_e7207 * assign7870_e7207)));
        locals.var_n_iginv_dn7 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn7) / (assign7870_e7207 * assign7870_e7207)));
        locals.var_n_iginv_dn8 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn8) / (assign7870_e7207 * assign7870_e7207)));
        locals.var_n_iginv_dn9 = (-((locals.var_niginv_i * locals.var_eg_2phit_dn9) / (assign7870_e7207 * assign7870_e7207)));
        locals.var_n_iginv_rv = 0.0;

        let assign7880_e7212: f64 = (locals.var_toxp_i * locals.var_toxp_i);
        let assign7880_e7213: f64 = (4e-18 / assign7880_e7212);
        locals.var_temp = assign7880_e7213;
        locals.var_temp_dn4 = 0.0;
        locals.var_temp_dn6 = 0.0;
        locals.var_temp_dn7 = 0.0;
        locals.var_temp_dn8 = 0.0;
        locals.var_temp_dn9 = 0.0;
        locals.var_temp_rv = 0.0;

        let assign7890_e7216: f64 = (locals.var_agidl_i * locals.var_temp);
        locals.var_agidl_i = assign7890_e7216;
        locals.var_agidl_i_dn4 = ((locals.var_agidl_i_dn4 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn4));
        locals.var_agidl_i_dn6 = ((locals.var_agidl_i_dn6 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn6));
        locals.var_agidl_i_dn7 = ((locals.var_agidl_i_dn7 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn7));
        locals.var_agidl_i_dn8 = ((locals.var_agidl_i_dn8 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn8));
        locals.var_agidl_i_dn9 = ((locals.var_agidl_i_dn9 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn9));
        locals.var_agidl_i_rv = 0.0;

        let assign7900_e7219: f64 = (locals.var_agidld_i * locals.var_temp);
        locals.var_agidld_i = assign7900_e7219;
        locals.var_agidld_i_dn4 = ((locals.var_agidld_i_dn4 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn4));
        locals.var_agidld_i_dn6 = ((locals.var_agidld_i_dn6 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn6));
        locals.var_agidld_i_dn7 = ((locals.var_agidld_i_dn7 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn7));
        locals.var_agidld_i_dn8 = ((locals.var_agidld_i_dn8 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn8));
        locals.var_agidld_i_dn9 = ((locals.var_agidld_i_dn9 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn9));
        locals.var_agidld_i_rv = 0.0;

        let assign7910_e7222: f64 = (locals.var_toxp_i * 500000000.0);
        locals.var_temp = assign7910_e7222;
        locals.var_temp_dn4 = 0.0;
        locals.var_temp_dn6 = 0.0;
        locals.var_temp_dn7 = 0.0;
        locals.var_temp_dn8 = 0.0;
        locals.var_temp_dn9 = 0.0;
        locals.var_temp_rv = 0.0;

        let assign7920_e7227: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign7920_e7228: f64 = (1.0 + assign7920_e7227);
        let assign7920_e7230: f64 = assign7920_e7228;
        let assign7920_e7234: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign7920_e7235: f64 = (1.0 + assign7920_e7234);
        let assign7920_e7237: f64 = assign7920_e7235;
        let assign7920_e7241: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign7920_e7242: f64 = (1.0 + assign7920_e7241);
        let assign7920_e7244: f64 = assign7920_e7242;
        let assign7920_e7245: f64 = (assign7920_e7237 * assign7920_e7244);
        let assign7920_e7247: f64 = (assign7920_e7245 + 0.01);
        let assign7920_e7248: f64 = (assign7920_e7247).sqrt();
        let assign7920_e7249: f64 = (assign7920_e7230 + assign7920_e7248);
        let assign7920_e7250: f64 = (0.5 * assign7920_e7249);
        locals.var_tempm = assign7920_e7250;
        locals.var_tempm_dn4 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn4) + ((((locals.var_stbgidl_i * locals.var_dt_dn4) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn4))) / (2.0 * assign7920_e7248))));
        locals.var_tempm_dn6 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn6) + ((((locals.var_stbgidl_i * locals.var_dt_dn6) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn6))) / (2.0 * assign7920_e7248))));
        locals.var_tempm_dn7 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn7) + ((((locals.var_stbgidl_i * locals.var_dt_dn7) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn7))) / (2.0 * assign7920_e7248))));
        locals.var_tempm_dn8 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn8) + ((((locals.var_stbgidl_i * locals.var_dt_dn8) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn8))) / (2.0 * assign7920_e7248))));
        locals.var_tempm_dn9 = (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn9) + ((((locals.var_stbgidl_i * locals.var_dt_dn9) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn9))) / (2.0 * assign7920_e7248))));
        locals.var_tempm_rv = 0.0;

        let assign7930_e7253: f64 = (locals.var_bgidl_t * locals.var_tempm);
        let assign7930_e7255: f64 = (assign7930_e7253 * locals.var_temp);
        locals.var_bgidl_i = assign7930_e7255;
        locals.var_bgidl_i_dn4 = (((locals.var_bgidl_t * locals.var_tempm_dn4) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn4));
        locals.var_bgidl_i_dn6 = (((locals.var_bgidl_t * locals.var_tempm_dn6) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn6));
        locals.var_bgidl_i_dn7 = (((locals.var_bgidl_t * locals.var_tempm_dn7) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn7));
        locals.var_bgidl_i_dn8 = (((locals.var_bgidl_t * locals.var_tempm_dn8) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn8));
        locals.var_bgidl_i_dn9 = (((locals.var_bgidl_t * locals.var_tempm_dn9) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn9));
        locals.var_bgidl_i_rv = 0.0;

        let assign7940_e7260: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign7940_e7261: f64 = (1.0 + assign7940_e7260);
        let assign7940_e7263: f64 = assign7940_e7261;
        let assign7940_e7267: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign7940_e7268: f64 = (1.0 + assign7940_e7267);
        let assign7940_e7270: f64 = assign7940_e7268;
        let assign7940_e7274: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign7940_e7275: f64 = (1.0 + assign7940_e7274);
        let assign7940_e7277: f64 = assign7940_e7275;
        let assign7940_e7278: f64 = (assign7940_e7270 * assign7940_e7277);
        let assign7940_e7280: f64 = (assign7940_e7278 + 0.01);
        let assign7940_e7281: f64 = (assign7940_e7280).sqrt();
        let assign7940_e7282: f64 = (assign7940_e7263 + assign7940_e7281);
        let assign7940_e7283: f64 = (0.5 * assign7940_e7282);
        locals.var_tempm = assign7940_e7283;
        locals.var_tempm_dn4 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn4) + ((((locals.var_stbgidld_i * locals.var_dt_dn4) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn4))) / (2.0 * assign7940_e7281))));
        locals.var_tempm_dn6 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn6) + ((((locals.var_stbgidld_i * locals.var_dt_dn6) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn6))) / (2.0 * assign7940_e7281))));
        locals.var_tempm_dn7 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn7) + ((((locals.var_stbgidld_i * locals.var_dt_dn7) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn7))) / (2.0 * assign7940_e7281))));
        locals.var_tempm_dn8 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn8) + ((((locals.var_stbgidld_i * locals.var_dt_dn8) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn8))) / (2.0 * assign7940_e7281))));
        locals.var_tempm_dn9 = (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn9) + ((((locals.var_stbgidld_i * locals.var_dt_dn9) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn9))) / (2.0 * assign7940_e7281))));
        locals.var_tempm_rv = 0.0;

        let assign7950_e7286: f64 = (locals.var_bgidld_t * locals.var_tempm);
        let assign7950_e7288: f64 = (assign7950_e7286 * locals.var_temp);
        locals.var_bgidld_i = assign7950_e7288;
        locals.var_bgidld_i_dn4 = (((locals.var_bgidld_t * locals.var_tempm_dn4) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn4));
        locals.var_bgidld_i_dn6 = (((locals.var_bgidld_t * locals.var_tempm_dn6) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn6));
        locals.var_bgidld_i_dn7 = (((locals.var_bgidld_t * locals.var_tempm_dn7) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn7));
        locals.var_bgidld_i_dn8 = (((locals.var_bgidld_t * locals.var_tempm_dn8) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn8));
        locals.var_bgidld_i_dn9 = (((locals.var_bgidld_t * locals.var_tempm_dn9) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn9));
        locals.var_bgidld_i_rv = 0.0;

        let assign7960_e7291: f64 = (-locals.var_sta2_i);
        let assign7960_e7293: f64 = (assign7960_e7291 * locals.var_lnrtn);
        let assign7960_e7294: f64 = (assign7960_e7293).exp();
        let assign7960_e7295: f64 = (locals.var_a2_t * assign7960_e7294);
        locals.var_a2_i = assign7960_e7295;
        locals.var_a2_i_dn4 = (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn4)));
        locals.var_a2_i_dn6 = (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn6)));
        locals.var_a2_i_dn7 = (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn7)));
        locals.var_a2_i_dn8 = (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn8)));
        locals.var_a2_i_dn9 = (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn9)));
        locals.var_a2_i_rv = 0.0;

        let assign7970_e7300: f64 = (locals.var_ctedge_i * locals.var_rtn);
        let assign7970_e7301: f64 = (1.0 + assign7970_e7300);
        let assign7970_e7302: f64 = (locals.var_phit0 * assign7970_e7301);
        locals.var_phit_edge = assign7970_e7302;
        locals.var_phit_edge_dn4 = ((locals.var_phit0_dn4 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn4)));
        locals.var_phit_edge_dn6 = ((locals.var_phit0_dn6 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn6)));
        locals.var_phit_edge_dn7 = ((locals.var_phit0_dn7 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn7)));
        locals.var_phit_edge_dn8 = ((locals.var_phit0_dn8 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn8)));
        locals.var_phit_edge_dn9 = ((locals.var_phit0_dn9 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn9)));
        locals.var_phit_edge_rv = 0.0;

        let assign7980_e7305: f64 = (1.0 / locals.var_phit_edge);
        locals.var_inv_phit_edge = assign7980_e7305;
        locals.var_inv_phit_edge_dn4 = (-(locals.var_phit_edge_dn4 / (locals.var_phit_edge * locals.var_phit_edge)));
        locals.var_inv_phit_edge_dn6 = (-(locals.var_phit_edge_dn6 / (locals.var_phit_edge * locals.var_phit_edge)));
        locals.var_inv_phit_edge_dn7 = (-(locals.var_phit_edge_dn7 / (locals.var_phit_edge * locals.var_phit_edge)));
        locals.var_inv_phit_edge_dn8 = (-(locals.var_phit_edge_dn8 / (locals.var_phit_edge * locals.var_phit_edge)));
        locals.var_inv_phit_edge_dn9 = (-(locals.var_phit_edge_dn9 / (locals.var_phit_edge * locals.var_phit_edge)));
        locals.var_inv_phit_edge_rv = 0.0;

        let assign7990_e7308: f64 = (2.0 * 1.602176565e-19);
        let assign7990_e7310: f64 = (assign7990_e7308 * locals.var_neff);
        let assign7990_e7312: f64 = (assign7990_e7310 * locals.var_epsch);
        let assign7990_e7314: f64 = (assign7990_e7312 * locals.var_inv_phit_edge);
        locals.var_a0_csisq_edge = assign7990_e7314;
        locals.var_a0_csisq_edge_dn4 = ((((assign7990_e7308 * locals.var_neff_dn4) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn4));
        locals.var_a0_csisq_edge_dn6 = ((((assign7990_e7308 * locals.var_neff_dn6) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn6));
        locals.var_a0_csisq_edge_dn7 = ((((assign7990_e7308 * locals.var_neff_dn7) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn7));
        locals.var_a0_csisq_edge_dn8 = ((((assign7990_e7308 * locals.var_neff_dn8) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn8));
        locals.var_a0_csisq_edge_dn9 = ((((assign7990_e7308 * locals.var_neff_dn9) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn9));
        locals.var_a0_csisq_edge_rv = 0.0;

        let assign8000_e7317: f64 = (p.p14 * locals.var_stvfbedge_i);
        let assign8000_e7319: f64 = (assign8000_e7317 * locals.var_dt);
        let assign8000_e7321: f64 = (assign8000_e7319 + locals.var_dvfbqm);
        locals.var_temp = assign8000_e7321;
        locals.var_temp_dn4 = (assign8000_e7317 * locals.var_dt_dn4);
        locals.var_temp_dn6 = (assign8000_e7317 * locals.var_dt_dn6);
        locals.var_temp_dn7 = (assign8000_e7317 * locals.var_dt_dn7);
        locals.var_temp_dn8 = (assign8000_e7317 * locals.var_dt_dn8);
        locals.var_temp_dn9 = (assign8000_e7317 * locals.var_dt_dn9);
        locals.var_temp_rv = 0.0;

        let assign8010_e7325: f64 = (locals.var_vfb1edge_t + locals.var_dvfbch);
        let assign8010_e7327: f64 = (assign8010_e7325 + locals.var_dvfb1nch);
        let assign8010_e7328: f64 = (p.p14 * assign8010_e7327);
        let assign8010_e7330: f64 = (assign8010_e7328 + locals.var_temp);
        let assign8010_e7332: f64 = (assign8010_e7330 + p.p34);
        let assign8010_e7334: f64 = (assign8010_e7332 - locals.var_dvfbpdep);
        locals.var_vfb1edge_i = assign8010_e7334;
        locals.var_vfb1edge_i_dn4 = (((p.p14 * ((locals.var_vfb1edge_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp_dn4) - locals.var_dvfbpdep_dn4);
        locals.var_vfb1edge_i_dn6 = (((p.p14 * ((locals.var_vfb1edge_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp_dn6) - locals.var_dvfbpdep_dn6);
        locals.var_vfb1edge_i_dn7 = (((p.p14 * ((locals.var_vfb1edge_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp_dn7) - locals.var_dvfbpdep_dn7);
        locals.var_vfb1edge_i_dn8 = (((p.p14 * ((locals.var_vfb1edge_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp_dn8) - locals.var_dvfbpdep_dn8);
        locals.var_vfb1edge_i_dn9 = (((p.p14 * ((locals.var_vfb1edge_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp_dn9) - locals.var_dvfbpdep_dn9);
        locals.var_vfb1edge_i_rv = 0.0;

        let assign8020_e7338: f64 = (locals.var_vfb2edge_t + locals.var_dvfbch);
        let assign8020_e7340: f64 = (assign8020_e7338 + locals.var_dvfb2nch);
        let assign8020_e7341: f64 = (p.p14 * assign8020_e7340);
        let assign8020_e7343: f64 = (assign8020_e7341 + locals.var_temp);
        locals.var_vfb2edge_i = assign8020_e7343;
        locals.var_vfb2edge_i_dn4 = ((p.p14 * (locals.var_dvfbch_dn4 + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4);
        locals.var_vfb2edge_i_dn6 = ((p.p14 * (locals.var_dvfbch_dn6 + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6);
        locals.var_vfb2edge_i_dn7 = ((p.p14 * (locals.var_dvfbch_dn7 + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7);
        locals.var_vfb2edge_i_dn8 = ((p.p14 * (locals.var_dvfbch_dn8 + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8);
        locals.var_vfb2edge_i_dn9 = ((p.p14 * (locals.var_dvfbch_dn9 + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9);
        locals.var_vfb2edge_i_rv = 0.0;

        let assign8030_e7346: f64 = (locals.var_stbetedge_i * locals.var_lnrtn);
        let assign8030_e7347: f64 = (assign8030_e7346).exp();
        let assign8030_e7349: f64 = (assign8030_e7347 * p.p35);
        locals.var_temp = assign8030_e7349;
        locals.var_temp_dn4 = ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn4)) * p.p35);
        locals.var_temp_dn6 = ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn6)) * p.p35);
        locals.var_temp_dn7 = ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn7)) * p.p35);
        locals.var_temp_dn8 = ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn8)) * p.p35);
        locals.var_temp_dn9 = ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn9)) * p.p35);
        locals.var_temp_rv = 0.0;

        let assign8040_e7352: f64 = (locals.var_betnedge_t * locals.var_temp);
        locals.var_betnedge_i = assign8040_e7352;
        locals.var_betnedge_i_dn4 = ((locals.var_betnedge_t_dn4 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn4));
        locals.var_betnedge_i_dn6 = ((locals.var_betnedge_t_dn6 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn6));
        locals.var_betnedge_i_dn7 = ((locals.var_betnedge_t_dn7 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn7));
        locals.var_betnedge_i_dn8 = ((locals.var_betnedge_t_dn8 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn8));
        locals.var_betnedge_i_dn9 = ((locals.var_betnedge_t_dn9 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn9));
        locals.var_betnedge_i_rv = 0.0;

        let assign8050_e7355: f64 = (locals.var_areaq_i * locals.var_phit);
        locals.var_area_phit = assign8050_e7355;
        locals.var_area_phit_dn4 = (locals.var_areaq_i * locals.var_phit_dn4);
        locals.var_area_phit_dn6 = (locals.var_areaq_i * locals.var_phit_dn6);
        locals.var_area_phit_dn7 = (locals.var_areaq_i * locals.var_phit_dn7);
        locals.var_area_phit_dn8 = (locals.var_areaq_i * locals.var_phit_dn8);
        locals.var_area_phit_dn9 = (locals.var_areaq_i * locals.var_phit_dn9);
        locals.var_area_phit_rv = 0.0;

        let assign8060_e7358: f64 = (0.25 * 1.602176565e-19);
        let assign8060_e7360: f64 = (assign8060_e7358 * locals.var_nsdac_i);
        let assign8060_e7363: f64 = (locals.var_epsch * locals.var_phit);
        let assign8060_e7364: f64 = (assign8060_e7360 / assign8060_e7363);
        locals.var_inner_sd = assign8060_e7364;
        locals.var_inner_sd_dn4 = (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn4)) / (assign8060_e7363 * assign8060_e7363)));
        locals.var_inner_sd_dn6 = (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn6)) / (assign8060_e7363 * assign8060_e7363)));
        locals.var_inner_sd_dn7 = (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn7)) / (assign8060_e7363 * assign8060_e7363)));
        locals.var_inner_sd_dn8 = (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn8)) / (assign8060_e7363 * assign8060_e7363)));
        locals.var_inner_sd_dn9 = (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn9)) / (assign8060_e7363 * assign8060_e7363)));
        locals.var_inner_sd_rv = 0.0;

        let assign8070_e7367: f64 = (locals.var_nsdac_i / locals.var_neff);
        let assign8070_e7368: f64 = (assign8070_e7367).ln();
        locals.var_xsd = assign8070_e7368;
        locals.var_xsd_dn4 = ((-((locals.var_nsdac_i * locals.var_neff_dn4) / (locals.var_neff * locals.var_neff))) / assign8070_e7367);
        locals.var_xsd_dn6 = ((-((locals.var_nsdac_i * locals.var_neff_dn6) / (locals.var_neff * locals.var_neff))) / assign8070_e7367);
        locals.var_xsd_dn7 = ((-((locals.var_nsdac_i * locals.var_neff_dn7) / (locals.var_neff * locals.var_neff))) / assign8070_e7367);
        locals.var_xsd_dn8 = ((-((locals.var_nsdac_i * locals.var_neff_dn8) / (locals.var_neff * locals.var_neff))) / assign8070_e7367);
        locals.var_xsd_dn9 = ((-((locals.var_nsdac_i * locals.var_neff_dn9) / (locals.var_neff * locals.var_neff))) / assign8070_e7367);
        locals.var_xsd_rv = 0.0;

        let assign8080_e7371: f64 = (locals.var_fif_i * 1.25e-6);
        let assign8080_e7373: f64 = (assign8080_e7371 * locals.var_phit);
        locals.var_fif_phit = assign8080_e7373;
        locals.var_fif_phit_dn4 = (assign8080_e7371 * locals.var_phit_dn4);
        locals.var_fif_phit_dn6 = (assign8080_e7371 * locals.var_phit_dn6);
        locals.var_fif_phit_dn7 = (assign8080_e7371 * locals.var_phit_dn7);
        locals.var_fif_phit_dn8 = (assign8080_e7371 * locals.var_phit_dn8);
        locals.var_fif_phit_dn9 = (assign8080_e7371 * locals.var_phit_dn9);
        locals.var_fif_phit_rv = 0.0;

        let assign8090_e7376: f64 = (locals.var_epsch / 3.45313e-11);
        let assign8090_e7378: f64 = (assign8090_e7376 * locals.var_tsi_i);
        let assign8090_e7381: f64 = (locals.var_tox1_i + 4e-10);
        let assign8090_e7382: f64 = (assign8090_e7378 * assign8090_e7381);
        let assign8090_e7383: f64 = (assign8090_e7382).sqrt();
        locals.var_lambda2d = assign8090_e7383;
        locals.var_lambda2d_rv = 0.0;

        let assign8100_e7386: f64 = (locals.var_strth_i * locals.var_lnrtn);
        let assign8100_e7387: f64 = (assign8100_e7386).exp();
        locals.var_tf_rth = assign8100_e7387;
        locals.var_tf_rth_dn4 = (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn4));
        locals.var_tf_rth_dn6 = (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn6));
        locals.var_tf_rth_dn7 = (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn7));
        locals.var_tf_rth_dn8 = (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn8));
        locals.var_tf_rth_dn9 = (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn9));
        locals.var_tf_rth_rv = 0.0;

        let assign8110_e7390: f64 = (locals.var_rth_t * locals.var_tf_rth);
        locals.var_rth_i = assign8110_e7390;
        locals.var_rth_i_dn4 = ((locals.var_rth_t_dn4 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn4));
        locals.var_rth_i_dn6 = ((locals.var_rth_t_dn6 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn6));
        locals.var_rth_i_dn7 = ((locals.var_rth_t_dn7 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn7));
        locals.var_rth_i_dn8 = ((locals.var_rth_t_dn8 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn8));
        locals.var_rth_i_dn9 = ((locals.var_rth_t_dn9 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn9));
        locals.var_rth_i_rv = 0.0;

        let assign8280_e7463: f64 = if locals.var_swshe_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign8280_e7463;
        locals.var_guard257_rv = 0.0;

        let (assign8290_e7467, assign8290_e7467_d_n4,) = {
    if (locals.var_guard257 != 0.0) {
        ((nv4 - 0.0), 1.0,)
    } else {
        (locals.var_dtc, locals.var_dtc_dn4,)
    }
};
        locals.var_dtc = assign8290_e7467;
        locals.var_dtc_dn4 = assign8290_e7467_d_n4;
        locals.var_dtc_rv = 0.0;

        let (assign8300_e7473, assign8300_e7473_d_n4, assign8300_e7473_d_n6, assign8300_e7473_d_n7, assign8300_e7473_d_n8, assign8300_e7473_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8300_e7471: f64 = (locals.var_tkd + locals.var_dtc);
        (assign8300_e7471, (locals.var_tkd_dn4 + locals.var_dtc_dn4), locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9,)
    } else {
        (locals.var_tkc, locals.var_tkc_dn4, locals.var_tkc_dn6, locals.var_tkc_dn7, locals.var_tkc_dn8, locals.var_tkc_dn9,)
    }
};
        locals.var_tkc = assign8300_e7473;
        locals.var_tkc_dn4 = assign8300_e7473_d_n4;
        locals.var_tkc_dn6 = assign8300_e7473_d_n6;
        locals.var_tkc_dn7 = assign8300_e7473_d_n7;
        locals.var_tkc_dn8 = assign8300_e7473_d_n8;
        locals.var_tkc_dn9 = assign8300_e7473_d_n9;
        locals.var_tkc_rv = 0.0;

        let (assign8310_e7479, assign8310_e7479_d_n4, assign8310_e7479_d_n6, assign8310_e7479_d_n7, assign8310_e7479_d_n8, assign8310_e7479_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8310_e7477: f64 = (locals.var_tkc * locals.var_tkc);
        (assign8310_e7477, ((locals.var_tkc_dn4 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn4)), ((locals.var_tkc_dn6 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn6)), ((locals.var_tkc_dn7 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn7)), ((locals.var_tkc_dn8 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn8)), ((locals.var_tkc_dn9 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn9)),)
    } else {
        (locals.var_tkc_sq, locals.var_tkc_sq_dn4, locals.var_tkc_sq_dn6, locals.var_tkc_sq_dn7, locals.var_tkc_sq_dn8, locals.var_tkc_sq_dn9,)
    }
};
        locals.var_tkc_sq = assign8310_e7479;
        locals.var_tkc_sq_dn4 = assign8310_e7479_d_n4;
        locals.var_tkc_sq_dn6 = assign8310_e7479_d_n6;
        locals.var_tkc_sq_dn7 = assign8310_e7479_d_n7;
        locals.var_tkc_sq_dn8 = assign8310_e7479_d_n8;
        locals.var_tkc_sq_dn9 = assign8310_e7479_d_n9;
        locals.var_tkc_sq_rv = 0.0;

        let (assign8320_e7485, assign8320_e7485_d_n4, assign8320_e7485_d_n6, assign8320_e7485_d_n7, assign8320_e7485_d_n8, assign8320_e7485_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8320_e7483: f64 = (locals.var_tkc - locals.var_tkr);
        (assign8320_e7483, locals.var_tkc_dn4, locals.var_tkc_dn6, locals.var_tkc_dn7, locals.var_tkc_dn8, locals.var_tkc_dn9,)
    } else {
        (locals.var_dt, locals.var_dt_dn4, locals.var_dt_dn6, locals.var_dt_dn7, locals.var_dt_dn8, locals.var_dt_dn9,)
    }
};
        locals.var_dt = assign8320_e7485;
        locals.var_dt_dn4 = assign8320_e7485_d_n4;
        locals.var_dt_dn6 = assign8320_e7485_d_n6;
        locals.var_dt_dn7 = assign8320_e7485_d_n7;
        locals.var_dt_dn8 = assign8320_e7485_d_n8;
        locals.var_dt_dn9 = assign8320_e7485_d_n9;
        locals.var_dt_rv = 0.0;

        let (assign8330_e7491, assign8330_e7491_d_n4, assign8330_e7491_d_n6, assign8330_e7491_d_n7, assign8330_e7491_d_n8, assign8330_e7491_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8330_e7489: f64 = (locals.var_tkc / locals.var_tkr);
        (assign8330_e7489, (locals.var_tkc_dn4 / locals.var_tkr), (locals.var_tkc_dn6 / locals.var_tkr), (locals.var_tkc_dn7 / locals.var_tkr), (locals.var_tkc_dn8 / locals.var_tkr), (locals.var_tkc_dn9 / locals.var_tkr),)
    } else {
        (locals.var_rt, locals.var_rt_dn4, locals.var_rt_dn6, locals.var_rt_dn7, locals.var_rt_dn8, locals.var_rt_dn9,)
    }
};
        locals.var_rt = assign8330_e7491;
        locals.var_rt_dn4 = assign8330_e7491_d_n4;
        locals.var_rt_dn6 = assign8330_e7491_d_n6;
        locals.var_rt_dn7 = assign8330_e7491_d_n7;
        locals.var_rt_dn8 = assign8330_e7491_d_n8;
        locals.var_rt_dn9 = assign8330_e7491_d_n9;
        locals.var_rt_rv = 0.0;

        let (assign8340_e7497, assign8340_e7497_d_n4, assign8340_e7497_d_n6, assign8340_e7497_d_n7, assign8340_e7497_d_n8, assign8340_e7497_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8340_e7495: f64 = (locals.var_tkr / locals.var_tkc);
        (assign8340_e7495, (-((locals.var_tkr * locals.var_tkc_dn4) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn6) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn7) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn8) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn9) / (locals.var_tkc * locals.var_tkc))),)
    } else {
        (locals.var_rtn, locals.var_rtn_dn4, locals.var_rtn_dn6, locals.var_rtn_dn7, locals.var_rtn_dn8, locals.var_rtn_dn9,)
    }
};
        locals.var_rtn = assign8340_e7497;
        locals.var_rtn_dn4 = assign8340_e7497_d_n4;
        locals.var_rtn_dn6 = assign8340_e7497_d_n6;
        locals.var_rtn_dn7 = assign8340_e7497_d_n7;
        locals.var_rtn_dn8 = assign8340_e7497_d_n8;
        locals.var_rtn_dn9 = assign8340_e7497_d_n9;
        locals.var_rtn_rv = 0.0;

        let (assign8350_e7503, assign8350_e7503_d_n4, assign8350_e7503_d_n6, assign8350_e7503_d_n7, assign8350_e7503_d_n8, assign8350_e7503_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8350_e7501: f64 = (locals.var_tkc * 8.617332384961e-5);
        (assign8350_e7501, (locals.var_tkc_dn4 * 8.617332384961e-5), (locals.var_tkc_dn6 * 8.617332384961e-5), (locals.var_tkc_dn7 * 8.617332384961e-5), (locals.var_tkc_dn8 * 8.617332384961e-5), (locals.var_tkc_dn9 * 8.617332384961e-5),)
    } else {
        (locals.var_phit0, locals.var_phit0_dn4, locals.var_phit0_dn6, locals.var_phit0_dn7, locals.var_phit0_dn8, locals.var_phit0_dn9,)
    }
};
        locals.var_phit0 = assign8350_e7503;
        locals.var_phit0_dn4 = assign8350_e7503_d_n4;
        locals.var_phit0_dn6 = assign8350_e7503_d_n6;
        locals.var_phit0_dn7 = assign8350_e7503_d_n7;
        locals.var_phit0_dn8 = assign8350_e7503_d_n8;
        locals.var_phit0_dn9 = assign8350_e7503_d_n9;
        locals.var_phit0_rv = 0.0;

        let (assign8360_e7509, assign8360_e7509_d_n4, assign8360_e7509_d_n6, assign8360_e7509_d_n7, assign8360_e7509_d_n8, assign8360_e7509_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8360_e7507: f64 = (1.0 / locals.var_phit0);
        (assign8360_e7507, (-(locals.var_phit0_dn4 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn6 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn7 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn8 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn9 / (locals.var_phit0 * locals.var_phit0))),)
    } else {
        (locals.var_inv_phit0, locals.var_inv_phit0_dn4, locals.var_inv_phit0_dn6, locals.var_inv_phit0_dn7, locals.var_inv_phit0_dn8, locals.var_inv_phit0_dn9,)
    }
};
        locals.var_inv_phit0 = assign8360_e7509;
        locals.var_inv_phit0_dn4 = assign8360_e7509_d_n4;
        locals.var_inv_phit0_dn6 = assign8360_e7509_d_n6;
        locals.var_inv_phit0_dn7 = assign8360_e7509_d_n7;
        locals.var_inv_phit0_dn8 = assign8360_e7509_d_n8;
        locals.var_inv_phit0_dn9 = assign8360_e7509_d_n9;
        locals.var_inv_phit0_rv = 0.0;

        let assign8370_e7512: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign8370_e7512;
        locals.var_guard258_rv = 0.0;

        let (assign8380_e7545, assign8380_e7545_d_n4, assign8380_e7545_d_n6, assign8380_e7545_d_n7, assign8380_e7545_d_n8, assign8380_e7545_d_n9,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign8380_e7520: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign8380_e7521: f64 = (10.0 / assign8380_e7520);
        let assign8380_e7523: f64 = (assign8380_e7521 + 600.0);
        let assign8380_e7527: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign8380_e7528: f64 = (10.0 / assign8380_e7527);
        let assign8380_e7530: f64 = (assign8380_e7528 - 600.0);
        let assign8380_e7534: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign8380_e7535: f64 = (10.0 / assign8380_e7534);
        let assign8380_e7537: f64 = (assign8380_e7535 - 600.0);
        let assign8380_e7538: f64 = (assign8380_e7530 * assign8380_e7537);
        let assign8380_e7540: f64 = (assign8380_e7538 + 0.01);
        let assign8380_e7541: f64 = (assign8380_e7540).sqrt();
        let assign8380_e7542: f64 = (assign8380_e7523 + assign8380_e7541);
        let assign8380_e7543: f64 = (0.5 * assign8380_e7542);
        (assign8380_e7543, (0.5 * ((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))),)
    } else {
        (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9,)
    }
};
        locals.var_xsatmax = assign8380_e7545;
        locals.var_xsatmax_dn4 = assign8380_e7545_d_n4;
        locals.var_xsatmax_dn6 = assign8380_e7545_d_n6;
        locals.var_xsatmax_dn7 = assign8380_e7545_d_n7;
        locals.var_xsatmax_dn8 = assign8380_e7545_d_n8;
        locals.var_xsatmax_dn9 = assign8380_e7545_d_n9;
        locals.var_xsatmax_rv = 0.0;

        let (assign8390_e7552, assign8390_e7552_d_n4, assign8390_e7552_d_n6, assign8390_e7552_d_n7, assign8390_e7552_d_n8, assign8390_e7552_d_n9,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 == 0.0)) {
        (600.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9,)
    }
};
        locals.var_xsatmax = assign8390_e7552;
        locals.var_xsatmax_dn4 = assign8390_e7552_d_n4;
        locals.var_xsatmax_dn6 = assign8390_e7552_d_n6;
        locals.var_xsatmax_dn7 = assign8390_e7552_d_n7;
        locals.var_xsatmax_dn8 = assign8390_e7552_d_n8;
        locals.var_xsatmax_dn9 = assign8390_e7552_d_n9;
        locals.var_xsatmax_rv = 0.0;

        let (assign8400_e7564, assign8400_e7564_d_n4, assign8400_e7564_d_n6, assign8400_e7564_d_n7, assign8400_e7564_d_n8, assign8400_e7564_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8400_e7557: f64 = (0.000473 * locals.var_tkc_sq);
        let assign8400_e7560: f64 = (636.0 + locals.var_tkc);
        let assign8400_e7561: f64 = (assign8400_e7557 / assign8400_e7560);
        let assign8400_e7562: f64 = (1.17 - assign8400_e7561);
        (assign8400_e7562, (-((((0.000473 * locals.var_tkc_sq_dn4) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn4)) / (assign8400_e7560 * assign8400_e7560))), (-((((0.000473 * locals.var_tkc_sq_dn6) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn6)) / (assign8400_e7560 * assign8400_e7560))), (-((((0.000473 * locals.var_tkc_sq_dn7) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn7)) / (assign8400_e7560 * assign8400_e7560))), (-((((0.000473 * locals.var_tkc_sq_dn8) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn8)) / (assign8400_e7560 * assign8400_e7560))), (-((((0.000473 * locals.var_tkc_sq_dn9) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn9)) / (assign8400_e7560 * assign8400_e7560))),)
    } else {
        (locals.var_egsi, locals.var_egsi_dn4, locals.var_egsi_dn6, locals.var_egsi_dn7, locals.var_egsi_dn8, locals.var_egsi_dn9,)
    }
};
        locals.var_egsi = assign8400_e7564;
        locals.var_egsi_dn4 = assign8400_e7564_d_n4;
        locals.var_egsi_dn6 = assign8400_e7564_d_n6;
        locals.var_egsi_dn7 = assign8400_e7564_d_n7;
        locals.var_egsi_dn8 = assign8400_e7564_d_n8;
        locals.var_egsi_dn9 = assign8400_e7564_d_n9;
        locals.var_egsi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8410_e7576, assign8410_e7576_d_n4, assign8410_e7576_d_n6, assign8410_e7576_d_n7, assign8410_e7576_d_n8, assign8410_e7576_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8410_e7569: f64 = (0.0004774 * locals.var_tkc_sq);
        let assign8410_e7572: f64 = (235.0 + locals.var_tkc);
        let assign8410_e7573: f64 = (assign8410_e7569 / assign8410_e7572);
        let assign8410_e7574: f64 = (0.744 - assign8410_e7573);
        (assign8410_e7574, (-((((0.0004774 * locals.var_tkc_sq_dn4) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn4)) / (assign8410_e7572 * assign8410_e7572))), (-((((0.0004774 * locals.var_tkc_sq_dn6) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn6)) / (assign8410_e7572 * assign8410_e7572))), (-((((0.0004774 * locals.var_tkc_sq_dn7) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn7)) / (assign8410_e7572 * assign8410_e7572))), (-((((0.0004774 * locals.var_tkc_sq_dn8) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn8)) / (assign8410_e7572 * assign8410_e7572))), (-((((0.0004774 * locals.var_tkc_sq_dn9) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn9)) / (assign8410_e7572 * assign8410_e7572))),)
    } else {
        (locals.var_egge, locals.var_egge_dn4, locals.var_egge_dn6, locals.var_egge_dn7, locals.var_egge_dn8, locals.var_egge_dn9,)
    }
};
        locals.var_egge = assign8410_e7576;
        locals.var_egge_dn4 = assign8410_e7576_d_n4;
        locals.var_egge_dn6 = assign8410_e7576_d_n6;
        locals.var_egge_dn7 = assign8410_e7576_d_n7;
        locals.var_egge_dn8 = assign8410_e7576_d_n8;
        locals.var_egge_dn9 = assign8410_e7576_d_n9;
        locals.var_egge_rv = 0.0;

        let (assign8420_e7589, assign8420_e7589_d_n4, assign8420_e7589_d_n6, assign8420_e7589_d_n7, assign8420_e7589_d_n8, assign8420_e7589_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8420_e7580: f64 = (locals.var_egge - locals.var_egsi);
        let assign8420_e7582: f64 = (-0.4);
        let assign8420_e7584: f64 = (assign8420_e7582 * locals.var_one_m_xge);
        let assign8420_e7585: f64 = (assign8420_e7580 + assign8420_e7584);
        let assign8420_e7587: f64 = (assign8420_e7585 * locals.var_xge_i);
        (assign8420_e7587, ((locals.var_egge_dn4 - locals.var_egsi_dn4) * locals.var_xge_i), ((locals.var_egge_dn6 - locals.var_egsi_dn6) * locals.var_xge_i), ((locals.var_egge_dn7 - locals.var_egsi_dn7) * locals.var_xge_i), ((locals.var_egge_dn8 - locals.var_egsi_dn8) * locals.var_xge_i), ((locals.var_egge_dn9 - locals.var_egsi_dn9) * locals.var_xge_i),)
    } else {
        (locals.var_deg, locals.var_deg_dn4, locals.var_deg_dn6, locals.var_deg_dn7, locals.var_deg_dn8, locals.var_deg_dn9,)
    }
};
        locals.var_deg = assign8420_e7589;
        locals.var_deg_dn4 = assign8420_e7589_d_n4;
        locals.var_deg_dn6 = assign8420_e7589_d_n6;
        locals.var_deg_dn7 = assign8420_e7589_d_n7;
        locals.var_deg_dn8 = assign8420_e7589_d_n8;
        locals.var_deg_dn9 = assign8420_e7589_d_n9;
        locals.var_deg_rv = 0.0;

        let (assign8430_e7595, assign8430_e7595_d_n4, assign8430_e7595_d_n6, assign8430_e7595_d_n7, assign8430_e7595_d_n8, assign8430_e7595_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8430_e7593: f64 = (locals.var_egsi + locals.var_deg);
        (assign8430_e7593, (locals.var_egsi_dn4 + locals.var_deg_dn4), (locals.var_egsi_dn6 + locals.var_deg_dn6), (locals.var_egsi_dn7 + locals.var_deg_dn7), (locals.var_egsi_dn8 + locals.var_deg_dn8), (locals.var_egsi_dn9 + locals.var_deg_dn9),)
    } else {
        (locals.var_eg, locals.var_eg_dn4, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9,)
    }
};
        locals.var_eg = assign8430_e7595;
        locals.var_eg_dn4 = assign8430_e7595_d_n4;
        locals.var_eg_dn6 = assign8430_e7595_d_n6;
        locals.var_eg_dn7 = assign8430_e7595_d_n7;
        locals.var_eg_dn8 = assign8430_e7595_d_n8;
        locals.var_eg_dn9 = assign8430_e7595_d_n9;
        locals.var_eg_rv = 0.0;

        let (assign8440_e7603, assign8440_e7603_d_n4, assign8440_e7603_d_n6, assign8440_e7603_d_n7, assign8440_e7603_d_n8, assign8440_e7603_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8440_e7599: f64 = (0.5 * locals.var_eg);
        let assign8440_e7601: f64 = (assign8440_e7599 * locals.var_inv_phit0);
        (assign8440_e7601, (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn4)), (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn6)), (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn7)), (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn8)), (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn9)),)
    } else {
        (locals.var_eg_2phit0, locals.var_eg_2phit0_dn4, locals.var_eg_2phit0_dn6, locals.var_eg_2phit0_dn7, locals.var_eg_2phit0_dn8, locals.var_eg_2phit0_dn9,)
    }
};
        locals.var_eg_2phit0 = assign8440_e7603;
        locals.var_eg_2phit0_dn4 = assign8440_e7603_d_n4;
        locals.var_eg_2phit0_dn6 = assign8440_e7603_d_n6;
        locals.var_eg_2phit0_dn7 = assign8440_e7603_d_n7;
        locals.var_eg_2phit0_dn8 = assign8440_e7603_d_n8;
        locals.var_eg_2phit0_dn9 = assign8440_e7603_d_n9;
        locals.var_eg_2phit0_rv = 0.0;

        let (assign8450_e7613, assign8450_e7613_d_n4, assign8450_e7613_d_n6, assign8450_e7613_d_n7, assign8450_e7613_d_n8, assign8450_e7613_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8450_e7607: f64 = (0.05 * locals.var_xge_i);
        let assign8450_e7610: f64 = (0.5 * locals.var_deg);
        let assign8450_e7611: f64 = (assign8450_e7607 - assign8450_e7610);
        (assign8450_e7611, (-(0.5 * locals.var_deg_dn4)), (-(0.5 * locals.var_deg_dn6)), (-(0.5 * locals.var_deg_dn7)), (-(0.5 * locals.var_deg_dn8)), (-(0.5 * locals.var_deg_dn9)),)
    } else {
        (locals.var_dvfbch, locals.var_dvfbch_dn4, locals.var_dvfbch_dn6, locals.var_dvfbch_dn7, locals.var_dvfbch_dn8, locals.var_dvfbch_dn9,)
    }
};
        locals.var_dvfbch = assign8450_e7613;
        locals.var_dvfbch_dn4 = assign8450_e7613_d_n4;
        locals.var_dvfbch_dn6 = assign8450_e7613_d_n6;
        locals.var_dvfbch_dn7 = assign8450_e7613_d_n7;
        locals.var_dvfbch_dn8 = assign8450_e7613_d_n8;
        locals.var_dvfbch_dn9 = assign8450_e7613_d_n9;
        locals.var_dvfbch_rv = 0.0;

        let (assign8460_e7620, assign8460_e7620_d_n4, assign8460_e7620_d_n6, assign8460_e7620_d_n7, assign8460_e7620_d_n8, assign8460_e7620_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8460_e7617: f64 = (locals.var_tkc * 0.0033333333333);
        let assign8460_e7618: f64 = (assign8460_e7617).sqrt();
        (assign8460_e7618, ((locals.var_tkc_dn4 * 0.0033333333333) / (2.0 * assign8460_e7618)), ((locals.var_tkc_dn6 * 0.0033333333333) / (2.0 * assign8460_e7618)), ((locals.var_tkc_dn7 * 0.0033333333333) / (2.0 * assign8460_e7618)), ((locals.var_tkc_dn8 * 0.0033333333333) / (2.0 * assign8460_e7618)), ((locals.var_tkc_dn9 * 0.0033333333333) / (2.0 * assign8460_e7618)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign8460_e7620;
        locals.var_temp_dn4 = assign8460_e7620_d_n4;
        locals.var_temp_dn6 = assign8460_e7620_d_n6;
        locals.var_temp_dn7 = assign8460_e7620_d_n7;
        locals.var_temp_dn8 = assign8460_e7620_d_n8;
        locals.var_temp_dn9 = assign8460_e7620_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign8470_e7630, assign8470_e7630_d_n4, assign8470_e7630_d_n6, assign8470_e7630_d_n7, assign8470_e7630_d_n8, assign8470_e7630_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8470_e7624: f64 = (4.05e25 * locals.var_temp);
        let assign8470_e7626: f64 = (assign8470_e7624 * locals.var_temp);
        let assign8470_e7628: f64 = (assign8470_e7626 * locals.var_temp);
        (assign8470_e7628, (((((4.05e25 * locals.var_temp_dn4) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn4)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn4)), (((((4.05e25 * locals.var_temp_dn6) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn6)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn6)), (((((4.05e25 * locals.var_temp_dn7) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn7)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn7)), (((((4.05e25 * locals.var_temp_dn8) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn8)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn8)), (((((4.05e25 * locals.var_temp_dn9) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn9)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign8470_e7630;
        locals.var_temp1_dn4 = assign8470_e7630_d_n4;
        locals.var_temp1_dn6 = assign8470_e7630_d_n6;
        locals.var_temp1_dn7 = assign8470_e7630_d_n7;
        locals.var_temp1_dn8 = assign8470_e7630_d_n8;
        locals.var_temp1_dn9 = assign8470_e7630_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign8480_e7636, assign8480_e7636_d_n4, assign8480_e7636_d_n6, assign8480_e7636_d_n7, assign8480_e7636_d_n8, assign8480_e7636_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8480_e7634: f64 = (locals.var_temp1 * locals.var_niratio);
        (assign8480_e7634, (locals.var_temp1_dn4 * locals.var_niratio), (locals.var_temp1_dn6 * locals.var_niratio), (locals.var_temp1_dn7 * locals.var_niratio), (locals.var_temp1_dn8 * locals.var_niratio), (locals.var_temp1_dn9 * locals.var_niratio),)
    } else {
        (locals.var_neff, locals.var_neff_dn4, locals.var_neff_dn6, locals.var_neff_dn7, locals.var_neff_dn8, locals.var_neff_dn9,)
    }
};
        locals.var_neff = assign8480_e7636;
        locals.var_neff_dn4 = assign8480_e7636_d_n4;
        locals.var_neff_dn6 = assign8480_e7636_d_n6;
        locals.var_neff_dn7 = assign8480_e7636_d_n7;
        locals.var_neff_dn8 = assign8480_e7636_d_n8;
        locals.var_neff_dn9 = assign8480_e7636_d_n9;
        locals.var_neff_rv = 0.0;

        let (assign8490_e7646, assign8490_e7646_d_n4, assign8490_e7646_d_n6, assign8490_e7646_d_n7, assign8490_e7646_d_n8, assign8490_e7646_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8490_e7642: f64 = (locals.var_ct_i * locals.var_rtn);
        let assign8490_e7643: f64 = (1.0 + assign8490_e7642);
        let assign8490_e7644: f64 = (locals.var_phit0 * assign8490_e7643);
        (assign8490_e7644, ((locals.var_phit0_dn4 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn4))), ((locals.var_phit0_dn6 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn6))), ((locals.var_phit0_dn7 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn7))), ((locals.var_phit0_dn8 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn8))), ((locals.var_phit0_dn9 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn9))),)
    } else {
        (locals.var_phit, locals.var_phit_dn4, locals.var_phit_dn6, locals.var_phit_dn7, locals.var_phit_dn8, locals.var_phit_dn9,)
    }
};
        locals.var_phit = assign8490_e7646;
        locals.var_phit_dn4 = assign8490_e7646_d_n4;
        locals.var_phit_dn6 = assign8490_e7646_d_n6;
        locals.var_phit_dn7 = assign8490_e7646_d_n7;
        locals.var_phit_dn8 = assign8490_e7646_d_n8;
        locals.var_phit_dn9 = assign8490_e7646_d_n9;
        locals.var_phit_rv = 0.0;

        let (assign8500_e7652, assign8500_e7652_d_n4, assign8500_e7652_d_n6, assign8500_e7652_d_n7, assign8500_e7652_d_n8, assign8500_e7652_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8500_e7650: f64 = (1.0 / locals.var_phit);
        (assign8500_e7650, (-(locals.var_phit_dn4 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn6 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn7 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn8 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn9 / (locals.var_phit * locals.var_phit))),)
    } else {
        (locals.var_inv_phit, locals.var_inv_phit_dn4, locals.var_inv_phit_dn6, locals.var_inv_phit_dn7, locals.var_inv_phit_dn8, locals.var_inv_phit_dn9,)
    }
};
        locals.var_inv_phit = assign8500_e7652;
        locals.var_inv_phit_dn4 = assign8500_e7652_d_n4;
        locals.var_inv_phit_dn6 = assign8500_e7652_d_n6;
        locals.var_inv_phit_dn7 = assign8500_e7652_d_n7;
        locals.var_inv_phit_dn8 = assign8500_e7652_d_n8;
        locals.var_inv_phit_dn9 = assign8500_e7652_d_n9;
        locals.var_inv_phit_rv = 0.0;

        let (assign8510_e7660, assign8510_e7660_d_n4, assign8510_e7660_d_n6, assign8510_e7660_d_n7, assign8510_e7660_d_n8, assign8510_e7660_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8510_e7656: f64 = (0.5 * locals.var_eg);
        let assign8510_e7658: f64 = (assign8510_e7656 * locals.var_inv_phit);
        (assign8510_e7658, (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn4)), (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn6)), (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn7)), (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn8)), (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn9)),)
    } else {
        (locals.var_eg_2phit, locals.var_eg_2phit_dn4, locals.var_eg_2phit_dn6, locals.var_eg_2phit_dn7, locals.var_eg_2phit_dn8, locals.var_eg_2phit_dn9,)
    }
};
        locals.var_eg_2phit = assign8510_e7660;
        locals.var_eg_2phit_dn4 = assign8510_e7660_d_n4;
        locals.var_eg_2phit_dn6 = assign8510_e7660_d_n6;
        locals.var_eg_2phit_dn7 = assign8510_e7660_d_n7;
        locals.var_eg_2phit_dn8 = assign8510_e7660_d_n8;
        locals.var_eg_2phit_dn9 = assign8510_e7660_d_n9;
        locals.var_eg_2phit_rv = 0.0;

        let (assign8520_e7672, assign8520_e7672_d_n4, assign8520_e7672_d_n6, assign8520_e7672_d_n7, assign8520_e7672_d_n8, assign8520_e7672_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8520_e7664: f64 = (2.0 * 1.602176565e-19);
        let assign8520_e7666: f64 = (assign8520_e7664 * locals.var_neff);
        let assign8520_e7668: f64 = (assign8520_e7666 * locals.var_epsch);
        let assign8520_e7670: f64 = (assign8520_e7668 * locals.var_inv_phit);
        (assign8520_e7670, ((((assign8520_e7664 * locals.var_neff_dn4) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn4)), ((((assign8520_e7664 * locals.var_neff_dn6) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn6)), ((((assign8520_e7664 * locals.var_neff_dn7) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn7)), ((((assign8520_e7664 * locals.var_neff_dn8) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn8)), ((((assign8520_e7664 * locals.var_neff_dn9) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn9)),)
    } else {
        (locals.var_a0_csisq, locals.var_a0_csisq_dn4, locals.var_a0_csisq_dn6, locals.var_a0_csisq_dn7, locals.var_a0_csisq_dn8, locals.var_a0_csisq_dn9,)
    }
};
        locals.var_a0_csisq = assign8520_e7672;
        locals.var_a0_csisq_dn4 = assign8520_e7672_d_n4;
        locals.var_a0_csisq_dn6 = assign8520_e7672_d_n6;
        locals.var_a0_csisq_dn7 = assign8520_e7672_d_n7;
        locals.var_a0_csisq_dn8 = assign8520_e7672_d_n8;
        locals.var_a0_csisq_dn9 = assign8520_e7672_d_n9;
        locals.var_a0_csisq_rv = 0.0;

        let (assign8530_e7683, assign8530_e7683_d_n4, assign8530_e7683_d_n6, assign8530_e7683_d_n7, assign8530_e7683_d_n8, assign8530_e7683_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8530_e7676: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign8530_e7678: f64 = (assign8530_e7676 / locals.var_a0_csisq);
        let assign8530_e7679: f64 = (assign8530_e7678).ln();
        let assign8530_e7681: f64 = (assign8530_e7679 - 0.6931471805599);
        (assign8530_e7681, ((-((assign8530_e7676 * locals.var_a0_csisq_dn4) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678), ((-((assign8530_e7676 * locals.var_a0_csisq_dn6) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678), ((-((assign8530_e7676 * locals.var_a0_csisq_dn7) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678), ((-((assign8530_e7676 * locals.var_a0_csisq_dn8) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678), ((-((assign8530_e7676 * locals.var_a0_csisq_dn9) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678),)
    } else {
        (locals.var_xth_1d, locals.var_xth_1d_dn4, locals.var_xth_1d_dn6, locals.var_xth_1d_dn7, locals.var_xth_1d_dn8, locals.var_xth_1d_dn9,)
    }
};
        locals.var_xth_1d = assign8530_e7683;
        locals.var_xth_1d_dn4 = assign8530_e7683_d_n4;
        locals.var_xth_1d_dn6 = assign8530_e7683_d_n6;
        locals.var_xth_1d_dn7 = assign8530_e7683_d_n7;
        locals.var_xth_1d_dn8 = assign8530_e7683_d_n8;
        locals.var_xth_1d_dn9 = assign8530_e7683_d_n9;
        locals.var_xth_1d_rv = 0.0;

        let (assign8540_e7699, assign8540_e7699_d_n4, assign8540_e7699_d_n6, assign8540_e7699_d_n7, assign8540_e7699_d_n8, assign8540_e7699_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8540_e7687: f64 = (0.5 * 1.602176565e-19);
        let assign8540_e7689: f64 = (assign8540_e7687 * locals.var_nsddc_i);
        let assign8540_e7691: f64 = (assign8540_e7689 * locals.var_tsi_i);
        let assign8540_e7694: f64 = (locals.var_cox1prime + locals.var_cox2prime);
        let assign8540_e7695: f64 = (assign8540_e7691 / assign8540_e7694);
        let assign8540_e7697: f64 = (assign8540_e7695 * locals.var_inv_phit);
        (assign8540_e7697, (assign8540_e7695 * locals.var_inv_phit_dn4), (assign8540_e7695 * locals.var_inv_phit_dn6), (assign8540_e7695 * locals.var_inv_phit_dn7), (assign8540_e7695 * locals.var_inv_phit_dn8), (assign8540_e7695 * locals.var_inv_phit_dn9),)
    } else {
        (locals.var_xsddep, locals.var_xsddep_dn4, locals.var_xsddep_dn6, locals.var_xsddep_dn7, locals.var_xsddep_dn8, locals.var_xsddep_dn9,)
    }
};
        locals.var_xsddep = assign8540_e7699;
        locals.var_xsddep_dn4 = assign8540_e7699_d_n4;
        locals.var_xsddep_dn6 = assign8540_e7699_d_n6;
        locals.var_xsddep_dn7 = assign8540_e7699_d_n7;
        locals.var_xsddep_dn8 = assign8540_e7699_d_n8;
        locals.var_xsddep_dn9 = assign8540_e7699_d_n9;
        locals.var_xsddep_rv = 0.0;

        let (assign8550_e7705, assign8550_e7705_d_n4, assign8550_e7705_d_n6, assign8550_e7705_d_n7, assign8550_e7705_d_n8, assign8550_e7705_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8550_e7703: f64 = (locals.var_stcf_i * locals.var_dt);
        (assign8550_e7703, ((locals.var_stcf_i_dn4 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn4)), ((locals.var_stcf_i_dn6 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn6)), ((locals.var_stcf_i_dn7 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn7)), ((locals.var_stcf_i_dn8 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn8)), ((locals.var_stcf_i_dn9 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign8550_e7705;
        locals.var_temp_dn4 = assign8550_e7705_d_n4;
        locals.var_temp_dn6 = assign8550_e7705_d_n6;
        locals.var_temp_dn7 = assign8550_e7705_d_n7;
        locals.var_temp_dn8 = assign8550_e7705_d_n8;
        locals.var_temp_dn9 = assign8550_e7705_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign8560_e7711, assign8560_e7711_d_n4, assign8560_e7711_d_n6, assign8560_e7711_d_n7, assign8560_e7711_d_n8, assign8560_e7711_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8560_e7709: f64 = (locals.var_cf1_t + locals.var_temp);
        (assign8560_e7709, (locals.var_cf1_t_dn4 + locals.var_temp_dn4), (locals.var_cf1_t_dn6 + locals.var_temp_dn6), (locals.var_cf1_t_dn7 + locals.var_temp_dn7), (locals.var_cf1_t_dn8 + locals.var_temp_dn8), (locals.var_cf1_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cf1_i, locals.var_cf1_i_dn4, locals.var_cf1_i_dn6, locals.var_cf1_i_dn7, locals.var_cf1_i_dn8, locals.var_cf1_i_dn9,)
    }
};
        locals.var_cf1_i = assign8560_e7711;
        locals.var_cf1_i_dn4 = assign8560_e7711_d_n4;
        locals.var_cf1_i_dn6 = assign8560_e7711_d_n6;
        locals.var_cf1_i_dn7 = assign8560_e7711_d_n7;
        locals.var_cf1_i_dn8 = assign8560_e7711_d_n8;
        locals.var_cf1_i_dn9 = assign8560_e7711_d_n9;
        locals.var_cf1_i_rv = 0.0;

        let (assign8570_e7717, assign8570_e7717_d_n4, assign8570_e7717_d_n6, assign8570_e7717_d_n7, assign8570_e7717_d_n8, assign8570_e7717_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8570_e7715: f64 = (locals.var_cf2_t + locals.var_temp);
        (assign8570_e7715, (locals.var_cf2_t_dn4 + locals.var_temp_dn4), (locals.var_cf2_t_dn6 + locals.var_temp_dn6), (locals.var_cf2_t_dn7 + locals.var_temp_dn7), (locals.var_cf2_t_dn8 + locals.var_temp_dn8), (locals.var_cf2_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cf2_i, locals.var_cf2_i_dn4, locals.var_cf2_i_dn6, locals.var_cf2_i_dn7, locals.var_cf2_i_dn8, locals.var_cf2_i_dn9,)
    }
};
        locals.var_cf2_i = assign8570_e7717;
        locals.var_cf2_i_dn4 = assign8570_e7717_d_n4;
        locals.var_cf2_i_dn6 = assign8570_e7717_d_n6;
        locals.var_cf2_i_dn7 = assign8570_e7717_d_n7;
        locals.var_cf2_i_dn8 = assign8570_e7717_d_n8;
        locals.var_cf2_i_dn9 = assign8570_e7717_d_n9;
        locals.var_cf2_i_rv = 0.0;

        let (assign8580_e7723, assign8580_e7723_d_n4, assign8580_e7723_d_n6, assign8580_e7723_d_n7, assign8580_e7723_d_n8, assign8580_e7723_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8580_e7721: f64 = (locals.var_cfd_i * locals.var_inv_phit);
        (assign8580_e7721, (locals.var_cfd_i * locals.var_inv_phit_dn4), (locals.var_cfd_i * locals.var_inv_phit_dn6), (locals.var_cfd_i * locals.var_inv_phit_dn7), (locals.var_cfd_i * locals.var_inv_phit_dn8), (locals.var_cfd_i * locals.var_inv_phit_dn9),)
    } else {
        (locals.var_xd0, locals.var_xd0_dn4, locals.var_xd0_dn6, locals.var_xd0_dn7, locals.var_xd0_dn8, locals.var_xd0_dn9,)
    }
};
        locals.var_xd0 = assign8580_e7723;
        locals.var_xd0_dn4 = assign8580_e7723_d_n4;
        locals.var_xd0_dn6 = assign8580_e7723_d_n6;
        locals.var_xd0_dn7 = assign8580_e7723_d_n7;
        locals.var_xd0_dn8 = assign8580_e7723_d_n8;
        locals.var_xd0_dn9 = assign8580_e7723_d_n9;
        locals.var_xd0_rv = 0.0;

        let (assign8590_e7729, assign8590_e7729_d_n4, assign8590_e7729_d_n6, assign8590_e7729_d_n7, assign8590_e7729_d_n8, assign8590_e7729_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8590_e7727: f64 = (locals.var_cfac1_t + locals.var_temp);
        (assign8590_e7727, (locals.var_cfac1_t_dn4 + locals.var_temp_dn4), (locals.var_cfac1_t_dn6 + locals.var_temp_dn6), (locals.var_cfac1_t_dn7 + locals.var_temp_dn7), (locals.var_cfac1_t_dn8 + locals.var_temp_dn8), (locals.var_cfac1_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cfac1_i, locals.var_cfac1_i_dn4, locals.var_cfac1_i_dn6, locals.var_cfac1_i_dn7, locals.var_cfac1_i_dn8, locals.var_cfac1_i_dn9,)
    }
};
        locals.var_cfac1_i = assign8590_e7729;
        locals.var_cfac1_i_dn4 = assign8590_e7729_d_n4;
        locals.var_cfac1_i_dn6 = assign8590_e7729_d_n6;
        locals.var_cfac1_i_dn7 = assign8590_e7729_d_n7;
        locals.var_cfac1_i_dn8 = assign8590_e7729_d_n8;
        locals.var_cfac1_i_dn9 = assign8590_e7729_d_n9;
        locals.var_cfac1_i_rv = 0.0;

        let (assign8600_e7735, assign8600_e7735_d_n4, assign8600_e7735_d_n6, assign8600_e7735_d_n7, assign8600_e7735_d_n8, assign8600_e7735_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8600_e7733: f64 = (locals.var_cfac2_t + locals.var_temp);
        (assign8600_e7733, (locals.var_cfac2_t_dn4 + locals.var_temp_dn4), (locals.var_cfac2_t_dn6 + locals.var_temp_dn6), (locals.var_cfac2_t_dn7 + locals.var_temp_dn7), (locals.var_cfac2_t_dn8 + locals.var_temp_dn8), (locals.var_cfac2_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cfac2_i, locals.var_cfac2_i_dn4, locals.var_cfac2_i_dn6, locals.var_cfac2_i_dn7, locals.var_cfac2_i_dn8, locals.var_cfac2_i_dn9,)
    }
};
        locals.var_cfac2_i = assign8600_e7735;
        locals.var_cfac2_i_dn4 = assign8600_e7735_d_n4;
        locals.var_cfac2_i_dn6 = assign8600_e7735_d_n6;
        locals.var_cfac2_i_dn7 = assign8600_e7735_d_n7;
        locals.var_cfac2_i_dn8 = assign8600_e7735_d_n8;
        locals.var_cfac2_i_dn9 = assign8600_e7735_d_n9;
        locals.var_cfac2_i_rv = 0.0;

        let assign8610_e7738: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign8610_e7738;
        locals.var_guard259_rv = 0.0;

        let (assign8620_e7751, assign8620_e7751_d_n4, assign8620_e7751_d_n6, assign8620_e7751_d_n7, assign8620_e7751_d_n8, assign8620_e7751_d_n9,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard259 != 0.0)) {
        let assign8620_e7745: f64 = (locals.var_np_i / locals.var_neff_poly);
        let assign8620_e7746: f64 = (assign8620_e7745).ln();
        let assign8620_e7748: f64 = (assign8620_e7746 + locals.var_eg_2phit0_woshe);
        let assign8620_e7749: f64 = (locals.var_phit0 * assign8620_e7748);
        (assign8620_e7749, ((locals.var_phit0_dn4 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn4 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn4)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn4))), ((locals.var_phit0_dn6 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn6 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn6)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn6))), ((locals.var_phit0_dn7 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn7 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn7)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn7))), ((locals.var_phit0_dn8 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn8 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn8)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn8))), ((locals.var_phit0_dn9 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn9 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn9)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn9))),)
    } else {
        (locals.var_dvfbpdep, locals.var_dvfbpdep_dn4, locals.var_dvfbpdep_dn6, locals.var_dvfbpdep_dn7, locals.var_dvfbpdep_dn8, locals.var_dvfbpdep_dn9,)
    }
};
        locals.var_dvfbpdep = assign8620_e7751;
        locals.var_dvfbpdep_dn4 = assign8620_e7751_d_n4;
        locals.var_dvfbpdep_dn6 = assign8620_e7751_d_n6;
        locals.var_dvfbpdep_dn7 = assign8620_e7751_d_n7;
        locals.var_dvfbpdep_dn8 = assign8620_e7751_d_n8;
        locals.var_dvfbpdep_dn9 = assign8620_e7751_d_n9;
        locals.var_dvfbpdep_rv = 0.0;

        let assign8630_e7754: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign8630_e7754;
        locals.var_guard260_rv = 0.0;

        let (assign8640_e7781, assign8640_e7781_d_n4, assign8640_e7781_d_n6, assign8640_e7781_d_n7, assign8640_e7781_d_n8, assign8640_e7781_d_n9,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard260 != 0.0)) {
        let assign8640_e7762: f64 = (2970.0 / locals.var_tkd);
        let assign8640_e7763: f64 = (15.0 + assign8640_e7762);
        let assign8640_e7767: f64 = (2970.0 / locals.var_tkd);
        let assign8640_e7768: f64 = (15.0 - assign8640_e7767);
        let assign8640_e7772: f64 = (2970.0 / locals.var_tkd);
        let assign8640_e7773: f64 = (15.0 - assign8640_e7772);
        let assign8640_e7774: f64 = (assign8640_e7768 * assign8640_e7773);
        let assign8640_e7776: f64 = (assign8640_e7774 + 1e-6);
        let assign8640_e7777: f64 = (assign8640_e7776).sqrt();
        let assign8640_e7778: f64 = (assign8640_e7763 + assign8640_e7777);
        let assign8640_e7779: f64 = (0.5 * assign8640_e7778);
        (assign8640_e7779, (0.5 * ((-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))),)
    } else {
        (locals.var_emin, locals.var_emin_dn4, locals.var_emin_dn6, locals.var_emin_dn7, locals.var_emin_dn8, locals.var_emin_dn9,)
    }
};
        locals.var_emin = assign8640_e7781;
        locals.var_emin_dn4 = assign8640_e7781_d_n4;
        locals.var_emin_dn6 = assign8640_e7781_d_n6;
        locals.var_emin_dn7 = assign8640_e7781_d_n7;
        locals.var_emin_dn8 = assign8640_e7781_d_n8;
        locals.var_emin_dn9 = assign8640_e7781_d_n9;
        locals.var_emin_rv = 0.0;

        let (assign8650_e7785, assign8650_e7785_d_n4, assign8650_e7785_d_n6, assign8650_e7785_d_n7, assign8650_e7785_d_n8, assign8650_e7785_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9,)
    }
};
        locals.var_qq = assign8650_e7785;
        locals.var_qq_dn4 = assign8650_e7785_d_n4;
        locals.var_qq_dn6 = assign8650_e7785_d_n6;
        locals.var_qq_dn7 = assign8650_e7785_d_n7;
        locals.var_qq_dn8 = assign8650_e7785_d_n8;
        locals.var_qq_dn9 = assign8650_e7785_d_n9;
        locals.var_qq_rv = 0.0;

        let assign8660_e7788: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign8660_e7788;
        locals.var_guard261_rv = 0.0;

        let assign8670_e7791: f64 = 1.0;
        let assign8670_e7792: f64 = if p.p14 == assign8670_e7791 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign8670_e7792;
        locals.var_guard262_rv = 0.0;

        let (assign8680_e7813, assign8680_e7813_d_n4, assign8680_e7813_d_n6, assign8680_e7813_d_n7, assign8680_e7813_d_n8, assign8680_e7813_d_n9,) = {
    if (((locals.var_guard257 != 0.0) && (locals.var_guard261 != 0.0)) && (locals.var_guard262 != 0.0)) {
        let assign8680_e7800: f64 = (0.4 * p.p13);
        let assign8680_e7802: f64 = (assign8680_e7800 * 1.27520989);
        let assign8680_e7804: f64 = (-0.3333333333333);
        let assign8680_e7807: f64 = (locals.var_phit * locals.var_tsisq);
        let assign8680_e7808: f64 = (assign8680_e7807).ln();
        let assign8680_e7809: f64 = (assign8680_e7804 * assign8680_e7808);
        let assign8680_e7810: f64 = (assign8680_e7809).exp();
        let assign8680_e7811: f64 = (assign8680_e7802 * assign8680_e7810);
        (assign8680_e7811, (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign8680_e7807)))), (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign8680_e7807)))), (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign8680_e7807)))), (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign8680_e7807)))), (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign8680_e7807)))),)
    } else {
        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9,)
    }
};
        locals.var_qq = assign8680_e7813;
        locals.var_qq_dn4 = assign8680_e7813_d_n4;
        locals.var_qq_dn6 = assign8680_e7813_d_n6;
        locals.var_qq_dn7 = assign8680_e7813_d_n7;
        locals.var_qq_dn8 = assign8680_e7813_d_n8;
        locals.var_qq_dn9 = assign8680_e7813_d_n9;
        locals.var_qq_rv = 0.0;

        let (assign8690_e7835, assign8690_e7835_d_n4, assign8690_e7835_d_n6, assign8690_e7835_d_n7, assign8690_e7835_d_n8, assign8690_e7835_d_n9,) = {
    if (((locals.var_guard257 != 0.0) && (locals.var_guard261 != 0.0)) && (locals.var_guard262 == 0.0)) {
        let assign8690_e7822: f64 = (0.4 * p.p13);
        let assign8690_e7824: f64 = (assign8690_e7822 * 1.5412087);
        let assign8690_e7826: f64 = (-0.3333333333333);
        let assign8690_e7829: f64 = (locals.var_phit * locals.var_tsisq);
        let assign8690_e7830: f64 = (assign8690_e7829).ln();
        let assign8690_e7831: f64 = (assign8690_e7826 * assign8690_e7830);
        let assign8690_e7832: f64 = (assign8690_e7831).exp();
        let assign8690_e7833: f64 = (assign8690_e7824 * assign8690_e7832);
        (assign8690_e7833, (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign8690_e7829)))), (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign8690_e7829)))), (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign8690_e7829)))), (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign8690_e7829)))), (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign8690_e7829)))),)
    } else {
        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9,)
    }
};
        locals.var_qq = assign8690_e7835;
        locals.var_qq_dn4 = assign8690_e7835_d_n4;
        locals.var_qq_dn6 = assign8690_e7835_d_n6;
        locals.var_qq_dn7 = assign8690_e7835_d_n7;
        locals.var_qq_dn8 = assign8690_e7835_d_n8;
        locals.var_qq_dn9 = assign8690_e7835_d_n9;
        locals.var_qq_rv = 0.0;

        let (assign8700_e7845, assign8700_e7845_d_n4, assign8700_e7845_d_n6, assign8700_e7845_d_n7, assign8700_e7845_d_n8, assign8700_e7845_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8700_e7839: f64 = (p.p14 * locals.var_stvfb_i);
        let assign8700_e7841: f64 = (assign8700_e7839 * locals.var_dt);
        let assign8700_e7843: f64 = (assign8700_e7841 + locals.var_dvfbqm);
        (assign8700_e7843, (assign8700_e7839 * locals.var_dt_dn4), (assign8700_e7839 * locals.var_dt_dn6), (assign8700_e7839 * locals.var_dt_dn7), (assign8700_e7839 * locals.var_dt_dn8), (assign8700_e7839 * locals.var_dt_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign8700_e7845;
        locals.var_temp_dn4 = assign8700_e7845_d_n4;
        locals.var_temp_dn6 = assign8700_e7845_d_n6;
        locals.var_temp_dn7 = assign8700_e7845_d_n7;
        locals.var_temp_dn8 = assign8700_e7845_d_n8;
        locals.var_temp_dn9 = assign8700_e7845_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign8710_e7853, assign8710_e7853_d_n4, assign8710_e7853_d_n6, assign8710_e7853_d_n7, assign8710_e7853_d_n8, assign8710_e7853_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8710_e7849: f64 = (locals.var_temp + p.p34);
        let assign8710_e7851: f64 = (assign8710_e7849 - locals.var_dvfbpdep);
        (assign8710_e7851, (locals.var_temp_dn4 - locals.var_dvfbpdep_dn4), (locals.var_temp_dn6 - locals.var_dvfbpdep_dn6), (locals.var_temp_dn7 - locals.var_dvfbpdep_dn7), (locals.var_temp_dn8 - locals.var_dvfbpdep_dn8), (locals.var_temp_dn9 - locals.var_dvfbpdep_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign8710_e7853;
        locals.var_temp1_dn4 = assign8710_e7853_d_n4;
        locals.var_temp1_dn6 = assign8710_e7853_d_n6;
        locals.var_temp1_dn7 = assign8710_e7853_d_n7;
        locals.var_temp1_dn8 = assign8710_e7853_d_n8;
        locals.var_temp1_dn9 = assign8710_e7853_d_n9;
        locals.var_temp1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8720_e7865, assign8720_e7865_d_n4, assign8720_e7865_d_n6, assign8720_e7865_d_n7, assign8720_e7865_d_n8, assign8720_e7865_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8720_e7858: f64 = (locals.var_vfb1_t + locals.var_dvfbch);
        let assign8720_e7860: f64 = (assign8720_e7858 + locals.var_dvfb1nch);
        let assign8720_e7861: f64 = (p.p14 * assign8720_e7860);
        let assign8720_e7863: f64 = (assign8720_e7861 + locals.var_temp1);
        (assign8720_e7863, ((p.p14 * ((locals.var_vfb1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4), ((p.p14 * ((locals.var_vfb1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6), ((p.p14 * ((locals.var_vfb1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7), ((p.p14 * ((locals.var_vfb1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8), ((p.p14 * ((locals.var_vfb1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9),)
    } else {
        (locals.var_vfb1_i, locals.var_vfb1_i_dn4, locals.var_vfb1_i_dn6, locals.var_vfb1_i_dn7, locals.var_vfb1_i_dn8, locals.var_vfb1_i_dn9,)
    }
};
        locals.var_vfb1_i = assign8720_e7865;
        locals.var_vfb1_i_dn4 = assign8720_e7865_d_n4;
        locals.var_vfb1_i_dn6 = assign8720_e7865_d_n6;
        locals.var_vfb1_i_dn7 = assign8720_e7865_d_n7;
        locals.var_vfb1_i_dn8 = assign8720_e7865_d_n8;
        locals.var_vfb1_i_dn9 = assign8720_e7865_d_n9;
        locals.var_vfb1_i_rv = 0.0;

        let (assign8730_e7877, assign8730_e7877_d_n4, assign8730_e7877_d_n6, assign8730_e7877_d_n7, assign8730_e7877_d_n8, assign8730_e7877_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8730_e7870: f64 = (locals.var_vfb2_t + locals.var_dvfbch);
        let assign8730_e7872: f64 = (assign8730_e7870 + locals.var_dvfb2nch);
        let assign8730_e7873: f64 = (p.p14 * assign8730_e7872);
        let assign8730_e7875: f64 = (assign8730_e7873 + locals.var_temp);
        (assign8730_e7875, ((p.p14 * ((locals.var_vfb2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfb2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfb2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfb2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfb2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9),)
    } else {
        (locals.var_vfb2_i, locals.var_vfb2_i_dn4, locals.var_vfb2_i_dn6, locals.var_vfb2_i_dn7, locals.var_vfb2_i_dn8, locals.var_vfb2_i_dn9,)
    }
};
        locals.var_vfb2_i = assign8730_e7877;
        locals.var_vfb2_i_dn4 = assign8730_e7877_d_n4;
        locals.var_vfb2_i_dn6 = assign8730_e7877_d_n6;
        locals.var_vfb2_i_dn7 = assign8730_e7877_d_n7;
        locals.var_vfb2_i_dn8 = assign8730_e7877_d_n8;
        locals.var_vfb2_i_dn9 = assign8730_e7877_d_n9;
        locals.var_vfb2_i_rv = 0.0;

        let (assign8740_e7889, assign8740_e7889_d_n4, assign8740_e7889_d_n6, assign8740_e7889_d_n7, assign8740_e7889_d_n8, assign8740_e7889_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8740_e7882: f64 = (locals.var_vfbac1_t + locals.var_dvfbch);
        let assign8740_e7884: f64 = (assign8740_e7882 + locals.var_dvfb1nch);
        let assign8740_e7885: f64 = (p.p14 * assign8740_e7884);
        let assign8740_e7887: f64 = (assign8740_e7885 + locals.var_temp1);
        (assign8740_e7887, ((p.p14 * ((locals.var_vfbac1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4), ((p.p14 * ((locals.var_vfbac1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6), ((p.p14 * ((locals.var_vfbac1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7), ((p.p14 * ((locals.var_vfbac1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8), ((p.p14 * ((locals.var_vfbac1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9),)
    } else {
        (locals.var_vfbac1_i, locals.var_vfbac1_i_dn4, locals.var_vfbac1_i_dn6, locals.var_vfbac1_i_dn7, locals.var_vfbac1_i_dn8, locals.var_vfbac1_i_dn9,)
    }
};
        locals.var_vfbac1_i = assign8740_e7889;
        locals.var_vfbac1_i_dn4 = assign8740_e7889_d_n4;
        locals.var_vfbac1_i_dn6 = assign8740_e7889_d_n6;
        locals.var_vfbac1_i_dn7 = assign8740_e7889_d_n7;
        locals.var_vfbac1_i_dn8 = assign8740_e7889_d_n8;
        locals.var_vfbac1_i_dn9 = assign8740_e7889_d_n9;
        locals.var_vfbac1_i_rv = 0.0;

        let (assign8750_e7901, assign8750_e7901_d_n4, assign8750_e7901_d_n6, assign8750_e7901_d_n7, assign8750_e7901_d_n8, assign8750_e7901_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8750_e7894: f64 = (locals.var_vfbac2_t + locals.var_dvfbch);
        let assign8750_e7896: f64 = (assign8750_e7894 + locals.var_dvfb2nch);
        let assign8750_e7897: f64 = (p.p14 * assign8750_e7896);
        let assign8750_e7899: f64 = (assign8750_e7897 + locals.var_temp);
        (assign8750_e7899, ((p.p14 * ((locals.var_vfbac2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfbac2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfbac2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfbac2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfbac2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9),)
    } else {
        (locals.var_vfbac2_i, locals.var_vfbac2_i_dn4, locals.var_vfbac2_i_dn6, locals.var_vfbac2_i_dn7, locals.var_vfbac2_i_dn8, locals.var_vfbac2_i_dn9,)
    }
};
        locals.var_vfbac2_i = assign8750_e7901;
        locals.var_vfbac2_i_dn4 = assign8750_e7901_d_n4;
        locals.var_vfbac2_i_dn6 = assign8750_e7901_d_n6;
        locals.var_vfbac2_i_dn7 = assign8750_e7901_d_n7;
        locals.var_vfbac2_i_dn8 = assign8750_e7901_d_n8;
        locals.var_vfbac2_i_dn9 = assign8750_e7901_d_n9;
        locals.var_vfbac2_i_rv = 0.0;

        let (assign8760_e7906, assign8760_e7906_d_n4, assign8760_e7906_d_n6, assign8760_e7906_d_n7, assign8760_e7906_d_n8, assign8760_e7906_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8760_e7904: f64 = (locals.var_rtn).ln();
        (assign8760_e7904, (locals.var_rtn_dn4 / locals.var_rtn), (locals.var_rtn_dn6 / locals.var_rtn), (locals.var_rtn_dn7 / locals.var_rtn), (locals.var_rtn_dn8 / locals.var_rtn), (locals.var_rtn_dn9 / locals.var_rtn),)
    } else {
        (locals.var_lnrtn, locals.var_lnrtn_dn4, locals.var_lnrtn_dn6, locals.var_lnrtn_dn7, locals.var_lnrtn_dn8, locals.var_lnrtn_dn9,)
    }
};
        locals.var_lnrtn = assign8760_e7906;
        locals.var_lnrtn_dn4 = assign8760_e7906_d_n4;
        locals.var_lnrtn_dn6 = assign8760_e7906_d_n6;
        locals.var_lnrtn_dn7 = assign8760_e7906_d_n7;
        locals.var_lnrtn_dn8 = assign8760_e7906_d_n8;
        locals.var_lnrtn_dn9 = assign8760_e7906_d_n9;
        locals.var_lnrtn_rv = 0.0;

        let (assign8770_e7915, assign8770_e7915_d_n4, assign8770_e7915_d_n6, assign8770_e7915_d_n7, assign8770_e7915_d_n8, assign8770_e7915_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8770_e7910: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign8770_e7911: f64 = (assign8770_e7910).exp();
        let assign8770_e7913: f64 = (assign8770_e7911 * p.p35);
        (assign8770_e7913, ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn4)) * p.p35), ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn6)) * p.p35), ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn7)) * p.p35), ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn8)) * p.p35), ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn9)) * p.p35),)
    } else {
        (locals.var_tf_bet, locals.var_tf_bet_dn4, locals.var_tf_bet_dn6, locals.var_tf_bet_dn7, locals.var_tf_bet_dn8, locals.var_tf_bet_dn9,)
    }
};
        locals.var_tf_bet = assign8770_e7915;
        locals.var_tf_bet_dn4 = assign8770_e7915_d_n4;
        locals.var_tf_bet_dn6 = assign8770_e7915_d_n6;
        locals.var_tf_bet_dn7 = assign8770_e7915_d_n7;
        locals.var_tf_bet_dn8 = assign8770_e7915_d_n8;
        locals.var_tf_bet_dn9 = assign8770_e7915_d_n9;
        locals.var_tf_bet_rv = 0.0;

        let (assign8780_e7921, assign8780_e7921_d_n4, assign8780_e7921_d_n6, assign8780_e7921_d_n7, assign8780_e7921_d_n8, assign8780_e7921_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8780_e7919: f64 = (locals.var_betn1_t * locals.var_tf_bet);
        (assign8780_e7919, ((locals.var_betn1_t_dn4 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn4)), ((locals.var_betn1_t_dn6 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn6)), ((locals.var_betn1_t_dn7 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn7)), ((locals.var_betn1_t_dn8 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn8)), ((locals.var_betn1_t_dn9 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn9)),)
    } else {
        (locals.var_betn1_i, locals.var_betn1_i_dn4, locals.var_betn1_i_dn6, locals.var_betn1_i_dn7, locals.var_betn1_i_dn8, locals.var_betn1_i_dn9,)
    }
};
        locals.var_betn1_i = assign8780_e7921;
        locals.var_betn1_i_dn4 = assign8780_e7921_d_n4;
        locals.var_betn1_i_dn6 = assign8780_e7921_d_n6;
        locals.var_betn1_i_dn7 = assign8780_e7921_d_n7;
        locals.var_betn1_i_dn8 = assign8780_e7921_d_n8;
        locals.var_betn1_i_dn9 = assign8780_e7921_d_n9;
        locals.var_betn1_i_rv = 0.0;

        let (assign8790_e7927, assign8790_e7927_d_n4, assign8790_e7927_d_n6, assign8790_e7927_d_n7, assign8790_e7927_d_n8, assign8790_e7927_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8790_e7925: f64 = (locals.var_betn2_t * locals.var_tf_bet);
        (assign8790_e7925, ((locals.var_betn2_t_dn4 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn4)), ((locals.var_betn2_t_dn6 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn6)), ((locals.var_betn2_t_dn7 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn7)), ((locals.var_betn2_t_dn8 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn8)), ((locals.var_betn2_t_dn9 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn9)),)
    } else {
        (locals.var_betn2_i, locals.var_betn2_i_dn4, locals.var_betn2_i_dn6, locals.var_betn2_i_dn7, locals.var_betn2_i_dn8, locals.var_betn2_i_dn9,)
    }
};
        locals.var_betn2_i = assign8790_e7927;
        locals.var_betn2_i_dn4 = assign8790_e7927_d_n4;
        locals.var_betn2_i_dn6 = assign8790_e7927_d_n6;
        locals.var_betn2_i_dn7 = assign8790_e7927_d_n7;
        locals.var_betn2_i_dn8 = assign8790_e7927_d_n8;
        locals.var_betn2_i_dn9 = assign8790_e7927_d_n9;
        locals.var_betn2_i_rv = 0.0;

        let (assign8800_e7934, assign8800_e7934_d_n4, assign8800_e7934_d_n6, assign8800_e7934_d_n7, assign8800_e7934_d_n8, assign8800_e7934_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8800_e7931: f64 = (locals.var_stmue_i * locals.var_lnrtn);
        let assign8800_e7932: f64 = (assign8800_e7931).exp();
        (assign8800_e7932, (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn4)), (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn6)), (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn7)), (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn8)), (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_mue, locals.var_tf_mue_dn4, locals.var_tf_mue_dn6, locals.var_tf_mue_dn7, locals.var_tf_mue_dn8, locals.var_tf_mue_dn9,)
    }
};
        locals.var_tf_mue = assign8800_e7934;
        locals.var_tf_mue_dn4 = assign8800_e7934_d_n4;
        locals.var_tf_mue_dn6 = assign8800_e7934_d_n6;
        locals.var_tf_mue_dn7 = assign8800_e7934_d_n7;
        locals.var_tf_mue_dn8 = assign8800_e7934_d_n8;
        locals.var_tf_mue_dn9 = assign8800_e7934_d_n9;
        locals.var_tf_mue_rv = 0.0;

        let (assign8810_e7940, assign8810_e7940_d_n4, assign8810_e7940_d_n6, assign8810_e7940_d_n7, assign8810_e7940_d_n8, assign8810_e7940_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8810_e7938: f64 = (locals.var_mue_t * locals.var_tf_mue);
        (assign8810_e7938, (locals.var_mue_t * locals.var_tf_mue_dn4), (locals.var_mue_t * locals.var_tf_mue_dn6), (locals.var_mue_t * locals.var_tf_mue_dn7), (locals.var_mue_t * locals.var_tf_mue_dn8), (locals.var_mue_t * locals.var_tf_mue_dn9),)
    } else {
        (locals.var_mue_i, locals.var_mue_i_dn4, locals.var_mue_i_dn6, locals.var_mue_i_dn7, locals.var_mue_i_dn8, locals.var_mue_i_dn9,)
    }
};
        locals.var_mue_i = assign8810_e7940;
        locals.var_mue_i_dn4 = assign8810_e7940_d_n4;
        locals.var_mue_i_dn6 = assign8810_e7940_d_n6;
        locals.var_mue_i_dn7 = assign8810_e7940_d_n7;
        locals.var_mue_i_dn8 = assign8810_e7940_d_n8;
        locals.var_mue_i_dn9 = assign8810_e7940_d_n9;
        locals.var_mue_i_rv = 0.0;

        let (assign8820_e7947, assign8820_e7947_d_n4, assign8820_e7947_d_n6, assign8820_e7947_d_n7, assign8820_e7947_d_n8, assign8820_e7947_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8820_e7944: f64 = (locals.var_stthemu_i * locals.var_lnrtn);
        let assign8820_e7945: f64 = (assign8820_e7944).exp();
        (assign8820_e7945, (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn4)), (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn6)), (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn7)), (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn8)), (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_themu, locals.var_tf_themu_dn4, locals.var_tf_themu_dn6, locals.var_tf_themu_dn7, locals.var_tf_themu_dn8, locals.var_tf_themu_dn9,)
    }
};
        locals.var_tf_themu = assign8820_e7947;
        locals.var_tf_themu_dn4 = assign8820_e7947_d_n4;
        locals.var_tf_themu_dn6 = assign8820_e7947_d_n6;
        locals.var_tf_themu_dn7 = assign8820_e7947_d_n7;
        locals.var_tf_themu_dn8 = assign8820_e7947_d_n8;
        locals.var_tf_themu_dn9 = assign8820_e7947_d_n9;
        locals.var_tf_themu_rv = 0.0;

        let (assign8830_e7953, assign8830_e7953_d_n4, assign8830_e7953_d_n6, assign8830_e7953_d_n7, assign8830_e7953_d_n8, assign8830_e7953_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8830_e7951: f64 = (locals.var_themu_t * locals.var_tf_themu);
        (assign8830_e7951, (locals.var_themu_t * locals.var_tf_themu_dn4), (locals.var_themu_t * locals.var_tf_themu_dn6), (locals.var_themu_t * locals.var_tf_themu_dn7), (locals.var_themu_t * locals.var_tf_themu_dn8), (locals.var_themu_t * locals.var_tf_themu_dn9),)
    } else {
        (locals.var_themu_i, locals.var_themu_i_dn4, locals.var_themu_i_dn6, locals.var_themu_i_dn7, locals.var_themu_i_dn8, locals.var_themu_i_dn9,)
    }
};
        locals.var_themu_i = assign8830_e7953;
        locals.var_themu_i_dn4 = assign8830_e7953_d_n4;
        locals.var_themu_i_dn6 = assign8830_e7953_d_n6;
        locals.var_themu_i_dn7 = assign8830_e7953_d_n7;
        locals.var_themu_i_dn8 = assign8830_e7953_d_n8;
        locals.var_themu_i_dn9 = assign8830_e7953_d_n9;
        locals.var_themu_i_rv = 0.0;

        let (assign8840_e7960, assign8840_e7960_d_n4, assign8840_e7960_d_n6, assign8840_e7960_d_n7, assign8840_e7960_d_n8, assign8840_e7960_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8840_e7957: f64 = (locals.var_stcs_i * locals.var_lnrtn);
        let assign8840_e7958: f64 = (assign8840_e7957).exp();
        (assign8840_e7958, (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn4)), (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn6)), (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn7)), (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn8)), (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_cs, locals.var_tf_cs_dn4, locals.var_tf_cs_dn6, locals.var_tf_cs_dn7, locals.var_tf_cs_dn8, locals.var_tf_cs_dn9,)
    }
};
        locals.var_tf_cs = assign8840_e7960;
        locals.var_tf_cs_dn4 = assign8840_e7960_d_n4;
        locals.var_tf_cs_dn6 = assign8840_e7960_d_n6;
        locals.var_tf_cs_dn7 = assign8840_e7960_d_n7;
        locals.var_tf_cs_dn8 = assign8840_e7960_d_n8;
        locals.var_tf_cs_dn9 = assign8840_e7960_d_n9;
        locals.var_tf_cs_rv = 0.0;

        let (assign8850_e7966, assign8850_e7966_d_n4, assign8850_e7966_d_n6, assign8850_e7966_d_n7, assign8850_e7966_d_n8, assign8850_e7966_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8850_e7964: f64 = (locals.var_cs_t * locals.var_tf_cs);
        (assign8850_e7964, (locals.var_cs_t * locals.var_tf_cs_dn4), (locals.var_cs_t * locals.var_tf_cs_dn6), (locals.var_cs_t * locals.var_tf_cs_dn7), (locals.var_cs_t * locals.var_tf_cs_dn8), (locals.var_cs_t * locals.var_tf_cs_dn9),)
    } else {
        (locals.var_cs_i, locals.var_cs_i_dn4, locals.var_cs_i_dn6, locals.var_cs_i_dn7, locals.var_cs_i_dn8, locals.var_cs_i_dn9,)
    }
};
        locals.var_cs_i = assign8850_e7966;
        locals.var_cs_i_dn4 = assign8850_e7966_d_n4;
        locals.var_cs_i_dn6 = assign8850_e7966_d_n6;
        locals.var_cs_i_dn7 = assign8850_e7966_d_n7;
        locals.var_cs_i_dn8 = assign8850_e7966_d_n8;
        locals.var_cs_i_dn9 = assign8850_e7966_d_n9;
        locals.var_cs_i_rv = 0.0;

        let (assign8860_e7973, assign8860_e7973_d_n4, assign8860_e7973_d_n6, assign8860_e7973_d_n7, assign8860_e7973_d_n8, assign8860_e7973_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8860_e7970: f64 = (locals.var_stthecs_i * locals.var_lnrtn);
        let assign8860_e7971: f64 = (assign8860_e7970).exp();
        (assign8860_e7971, (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn4)), (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn6)), (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn7)), (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn8)), (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_thecs, locals.var_tf_thecs_dn4, locals.var_tf_thecs_dn6, locals.var_tf_thecs_dn7, locals.var_tf_thecs_dn8, locals.var_tf_thecs_dn9,)
    }
};
        locals.var_tf_thecs = assign8860_e7973;
        locals.var_tf_thecs_dn4 = assign8860_e7973_d_n4;
        locals.var_tf_thecs_dn6 = assign8860_e7973_d_n6;
        locals.var_tf_thecs_dn7 = assign8860_e7973_d_n7;
        locals.var_tf_thecs_dn8 = assign8860_e7973_d_n8;
        locals.var_tf_thecs_dn9 = assign8860_e7973_d_n9;
        locals.var_tf_thecs_rv = 0.0;

        let (assign8870_e7979, assign8870_e7979_d_n4, assign8870_e7979_d_n6, assign8870_e7979_d_n7, assign8870_e7979_d_n8, assign8870_e7979_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8870_e7977: f64 = (locals.var_thecs_t * locals.var_tf_thecs);
        (assign8870_e7977, (locals.var_thecs_t * locals.var_tf_thecs_dn4), (locals.var_thecs_t * locals.var_tf_thecs_dn6), (locals.var_thecs_t * locals.var_tf_thecs_dn7), (locals.var_thecs_t * locals.var_tf_thecs_dn8), (locals.var_thecs_t * locals.var_tf_thecs_dn9),)
    } else {
        (locals.var_thecs_i, locals.var_thecs_i_dn4, locals.var_thecs_i_dn6, locals.var_thecs_i_dn7, locals.var_thecs_i_dn8, locals.var_thecs_i_dn9,)
    }
};
        locals.var_thecs_i = assign8870_e7979;
        locals.var_thecs_i_dn4 = assign8870_e7979_d_n4;
        locals.var_thecs_i_dn6 = assign8870_e7979_d_n6;
        locals.var_thecs_i_dn7 = assign8870_e7979_d_n7;
        locals.var_thecs_i_dn8 = assign8870_e7979_d_n8;
        locals.var_thecs_i_dn9 = assign8870_e7979_d_n9;
        locals.var_thecs_i_rv = 0.0;

        let (assign8880_e7986, assign8880_e7986_d_n4, assign8880_e7986_d_n6, assign8880_e7986_d_n7, assign8880_e7986_d_n8, assign8880_e7986_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8880_e7983: f64 = (locals.var_stxcor_i * locals.var_lnrtn);
        let assign8880_e7984: f64 = (assign8880_e7983).exp();
        (assign8880_e7984, (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn4)), (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn6)), (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn7)), (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn8)), (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_xcor, locals.var_tf_xcor_dn4, locals.var_tf_xcor_dn6, locals.var_tf_xcor_dn7, locals.var_tf_xcor_dn8, locals.var_tf_xcor_dn9,)
    }
};
        locals.var_tf_xcor = assign8880_e7986;
        locals.var_tf_xcor_dn4 = assign8880_e7986_d_n4;
        locals.var_tf_xcor_dn6 = assign8880_e7986_d_n6;
        locals.var_tf_xcor_dn7 = assign8880_e7986_d_n7;
        locals.var_tf_xcor_dn8 = assign8880_e7986_d_n8;
        locals.var_tf_xcor_dn9 = assign8880_e7986_d_n9;
        locals.var_tf_xcor_rv = 0.0;

        let (assign8890_e7992, assign8890_e7992_d_n4, assign8890_e7992_d_n6, assign8890_e7992_d_n7, assign8890_e7992_d_n8, assign8890_e7992_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8890_e7990: f64 = (locals.var_xcor_t * locals.var_tf_xcor);
        (assign8890_e7990, (locals.var_xcor_t * locals.var_tf_xcor_dn4), (locals.var_xcor_t * locals.var_tf_xcor_dn6), (locals.var_xcor_t * locals.var_tf_xcor_dn7), (locals.var_xcor_t * locals.var_tf_xcor_dn8), (locals.var_xcor_t * locals.var_tf_xcor_dn9),)
    } else {
        (locals.var_xcor_i, locals.var_xcor_i_dn4, locals.var_xcor_i_dn6, locals.var_xcor_i_dn7, locals.var_xcor_i_dn8, locals.var_xcor_i_dn9,)
    }
};
        locals.var_xcor_i = assign8890_e7992;
        locals.var_xcor_i_dn4 = assign8890_e7992_d_n4;
        locals.var_xcor_i_dn6 = assign8890_e7992_d_n6;
        locals.var_xcor_i_dn7 = assign8890_e7992_d_n7;
        locals.var_xcor_i_dn8 = assign8890_e7992_d_n8;
        locals.var_xcor_i_dn9 = assign8890_e7992_d_n9;
        locals.var_xcor_i_rv = 0.0;

        let (assign8900_e8000, assign8900_e8000_d_n4, assign8900_e8000_d_n6, assign8900_e8000_d_n7, assign8900_e8000_d_n8, assign8900_e8000_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8900_e7996: f64 = (1e-8 * locals.var_phit);
        let assign8900_e7998: f64 = (assign8900_e7996 / locals.var_tsi_i);
        (assign8900_e7998, ((1e-8 * locals.var_phit_dn4) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn6) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn7) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn8) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn9) / locals.var_tsi_i),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign8900_e8000;
        locals.var_temp_dn4 = assign8900_e8000_d_n4;
        locals.var_temp_dn6 = assign8900_e8000_d_n6;
        locals.var_temp_dn7 = assign8900_e8000_d_n7;
        locals.var_temp_dn8 = assign8900_e8000_d_n8;
        locals.var_temp_dn9 = assign8900_e8000_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign8910_e8006, assign8910_e8006_d_n4, assign8910_e8006_d_n6, assign8910_e8006_d_n7, assign8910_e8006_d_n8, assign8910_e8006_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8910_e8004: f64 = (locals.var_temp * locals.var_mue_i);
        (assign8910_e8004, ((locals.var_temp_dn4 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn4)), ((locals.var_temp_dn6 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn6)), ((locals.var_temp_dn7 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn7)), ((locals.var_temp_dn8 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn8)), ((locals.var_temp_dn9 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn9)),)
    } else {
        (locals.var_fmue, locals.var_fmue_dn4, locals.var_fmue_dn6, locals.var_fmue_dn7, locals.var_fmue_dn8, locals.var_fmue_dn9,)
    }
};
        locals.var_fmue = assign8910_e8006;
        locals.var_fmue_dn4 = assign8910_e8006_d_n4;
        locals.var_fmue_dn6 = assign8910_e8006_d_n6;
        locals.var_fmue_dn7 = assign8910_e8006_d_n7;
        locals.var_fmue_dn8 = assign8910_e8006_d_n8;
        locals.var_fmue_dn9 = assign8910_e8006_d_n9;
        locals.var_fmue_rv = 0.0;

        let (assign8920_e8013, assign8920_e8013_d_n4, assign8920_e8013_d_n6, assign8920_e8013_d_n7, assign8920_e8013_d_n8, assign8920_e8013_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8920_e8010: f64 = (locals.var_strs_i * locals.var_lnrtn);
        let assign8920_e8011: f64 = (assign8920_e8010).exp();
        (assign8920_e8011, (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn4)), (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn6)), (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn7)), (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn8)), (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_ther, locals.var_tf_ther_dn4, locals.var_tf_ther_dn6, locals.var_tf_ther_dn7, locals.var_tf_ther_dn8, locals.var_tf_ther_dn9,)
    }
};
        locals.var_tf_ther = assign8920_e8013;
        locals.var_tf_ther_dn4 = assign8920_e8013_d_n4;
        locals.var_tf_ther_dn6 = assign8920_e8013_d_n6;
        locals.var_tf_ther_dn7 = assign8920_e8013_d_n7;
        locals.var_tf_ther_dn8 = assign8920_e8013_d_n8;
        locals.var_tf_ther_dn9 = assign8920_e8013_d_n9;
        locals.var_tf_ther_rv = 0.0;

        let (assign8930_e8019, assign8930_e8019_d_n4, assign8930_e8019_d_n6, assign8930_e8019_d_n7, assign8930_e8019_d_n8, assign8930_e8019_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8930_e8017: f64 = (locals.var_rs_t * locals.var_tf_ther);
        (assign8930_e8017, (locals.var_rs_t * locals.var_tf_ther_dn4), (locals.var_rs_t * locals.var_tf_ther_dn6), (locals.var_rs_t * locals.var_tf_ther_dn7), (locals.var_rs_t * locals.var_tf_ther_dn8), (locals.var_rs_t * locals.var_tf_ther_dn9),)
    } else {
        (locals.var_rs_i, locals.var_rs_i_dn4, locals.var_rs_i_dn6, locals.var_rs_i_dn7, locals.var_rs_i_dn8, locals.var_rs_i_dn9,)
    }
};
        locals.var_rs_i = assign8930_e8019;
        locals.var_rs_i_dn4 = assign8930_e8019_d_n4;
        locals.var_rs_i_dn6 = assign8930_e8019_d_n6;
        locals.var_rs_i_dn7 = assign8930_e8019_d_n7;
        locals.var_rs_i_dn8 = assign8930_e8019_d_n8;
        locals.var_rs_i_dn9 = assign8930_e8019_d_n9;
        locals.var_rs_i_rv = 0.0;

        let (assign8940_e8027, assign8940_e8027_d_n4, assign8940_e8027_d_n6, assign8940_e8027_d_n7, assign8940_e8027_d_n8, assign8940_e8027_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8940_e8023: f64 = (2.0 * locals.var_rs_i);
        let assign8940_e8025: f64 = (assign8940_e8023 * locals.var_phit);
        (assign8940_e8025, (((2.0 * locals.var_rs_i_dn4) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn4)), (((2.0 * locals.var_rs_i_dn6) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn6)), (((2.0 * locals.var_rs_i_dn7) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn7)), (((2.0 * locals.var_rs_i_dn8) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn8)), (((2.0 * locals.var_rs_i_dn9) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn9)),)
    } else {
        (locals.var_frs, locals.var_frs_dn4, locals.var_frs_dn6, locals.var_frs_dn7, locals.var_frs_dn8, locals.var_frs_dn9,)
    }
};
        locals.var_frs = assign8940_e8027;
        locals.var_frs_dn4 = assign8940_e8027_d_n4;
        locals.var_frs_dn6 = assign8940_e8027_d_n6;
        locals.var_frs_dn7 = assign8940_e8027_d_n7;
        locals.var_frs_dn8 = assign8940_e8027_d_n8;
        locals.var_frs_dn9 = assign8940_e8027_d_n9;
        locals.var_frs_rv = 0.0;

        let (assign8950_e8034, assign8950_e8034_d_n4, assign8950_e8034_d_n6, assign8950_e8034_d_n7, assign8950_e8034_d_n8, assign8950_e8034_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8950_e8031: f64 = (locals.var_stthesat_i * locals.var_lnrtn);
        let assign8950_e8032: f64 = (assign8950_e8031).exp();
        (assign8950_e8032, (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn4)), (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn6)), (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn7)), (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn8)), (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_thesat, locals.var_tf_thesat_dn4, locals.var_tf_thesat_dn6, locals.var_tf_thesat_dn7, locals.var_tf_thesat_dn8, locals.var_tf_thesat_dn9,)
    }
};
        locals.var_tf_thesat = assign8950_e8034;
        locals.var_tf_thesat_dn4 = assign8950_e8034_d_n4;
        locals.var_tf_thesat_dn6 = assign8950_e8034_d_n6;
        locals.var_tf_thesat_dn7 = assign8950_e8034_d_n7;
        locals.var_tf_thesat_dn8 = assign8950_e8034_d_n8;
        locals.var_tf_thesat_dn9 = assign8950_e8034_d_n9;
        locals.var_tf_thesat_rv = 0.0;

        let (assign8960_e8042, assign8960_e8042_d_n4, assign8960_e8042_d_n6, assign8960_e8042_d_n7, assign8960_e8042_d_n8, assign8960_e8042_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8960_e8038: f64 = (locals.var_thesat_t * locals.var_tf_thesat);
        let assign8960_e8040: f64 = (assign8960_e8038 * locals.var_tf_bet);
        (assign8960_e8040, ((((locals.var_thesat_t_dn4 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn4)), ((((locals.var_thesat_t_dn6 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn6)), ((((locals.var_thesat_t_dn7 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn7)), ((((locals.var_thesat_t_dn8 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn8)), ((((locals.var_thesat_t_dn9 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn9)),)
    } else {
        (locals.var_thesat_i, locals.var_thesat_i_dn4, locals.var_thesat_i_dn6, locals.var_thesat_i_dn7, locals.var_thesat_i_dn8, locals.var_thesat_i_dn9,)
    }
};
        locals.var_thesat_i = assign8960_e8042;
        locals.var_thesat_i_dn4 = assign8960_e8042_d_n4;
        locals.var_thesat_i_dn6 = assign8960_e8042_d_n6;
        locals.var_thesat_i_dn7 = assign8960_e8042_d_n7;
        locals.var_thesat_i_dn8 = assign8960_e8042_d_n8;
        locals.var_thesat_i_dn9 = assign8960_e8042_d_n9;
        locals.var_thesat_i_rv = 0.0;

        let (assign8970_e8048, assign8970_e8048_d_n4, assign8970_e8048_d_n6, assign8970_e8048_d_n7, assign8970_e8048_d_n8, assign8970_e8048_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8970_e8046: f64 = (locals.var_thesat_i * locals.var_phit);
        (assign8970_e8046, ((locals.var_thesat_i_dn4 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn4)), ((locals.var_thesat_i_dn6 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn6)), ((locals.var_thesat_i_dn7 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn7)), ((locals.var_thesat_i_dn8 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn8)), ((locals.var_thesat_i_dn9 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn9)),)
    } else {
        (locals.var_sat_phit, locals.var_sat_phit_dn4, locals.var_sat_phit_dn6, locals.var_sat_phit_dn7, locals.var_sat_phit_dn8, locals.var_sat_phit_dn9,)
    }
};
        locals.var_sat_phit = assign8970_e8048;
        locals.var_sat_phit_dn4 = assign8970_e8048_d_n4;
        locals.var_sat_phit_dn6 = assign8970_e8048_d_n6;
        locals.var_sat_phit_dn7 = assign8970_e8048_d_n7;
        locals.var_sat_phit_dn8 = assign8970_e8048_d_n8;
        locals.var_sat_phit_dn9 = assign8970_e8048_d_n9;
        locals.var_sat_phit_rv = 0.0;

        let (assign8980_e8056, assign8980_e8056_d_n4, assign8980_e8056_d_n6, assign8980_e8056_d_n7, assign8980_e8056_d_n8, assign8980_e8056_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8980_e8052: f64 = (locals.var_thesatac_t * locals.var_tf_thesat);
        let assign8980_e8054: f64 = (assign8980_e8052 * locals.var_tf_bet);
        (assign8980_e8054, ((((locals.var_thesatac_t_dn4 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn4)), ((((locals.var_thesatac_t_dn6 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn6)), ((((locals.var_thesatac_t_dn7 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn7)), ((((locals.var_thesatac_t_dn8 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn8)), ((((locals.var_thesatac_t_dn9 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn9)),)
    } else {
        (locals.var_thesatac_i, locals.var_thesatac_i_dn4, locals.var_thesatac_i_dn6, locals.var_thesatac_i_dn7, locals.var_thesatac_i_dn8, locals.var_thesatac_i_dn9,)
    }
};
        locals.var_thesatac_i = assign8980_e8056;
        locals.var_thesatac_i_dn4 = assign8980_e8056_d_n4;
        locals.var_thesatac_i_dn6 = assign8980_e8056_d_n6;
        locals.var_thesatac_i_dn7 = assign8980_e8056_d_n7;
        locals.var_thesatac_i_dn8 = assign8980_e8056_d_n8;
        locals.var_thesatac_i_dn9 = assign8980_e8056_d_n9;
        locals.var_thesatac_i_rv = 0.0;

        let (assign8990_e8062, assign8990_e8062_d_n4, assign8990_e8062_d_n6, assign8990_e8062_d_n7, assign8990_e8062_d_n8, assign8990_e8062_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign8990_e8060: f64 = (locals.var_thesatac_i * locals.var_phit);
        (assign8990_e8060, ((locals.var_thesatac_i_dn4 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn4)), ((locals.var_thesatac_i_dn6 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn6)), ((locals.var_thesatac_i_dn7 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn7)), ((locals.var_thesatac_i_dn8 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn8)), ((locals.var_thesatac_i_dn9 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn9)),)
    } else {
        (locals.var_sat_phit_ac, locals.var_sat_phit_ac_dn4, locals.var_sat_phit_ac_dn6, locals.var_sat_phit_ac_dn7, locals.var_sat_phit_ac_dn8, locals.var_sat_phit_ac_dn9,)
    }
};
        locals.var_sat_phit_ac = assign8990_e8062;
        locals.var_sat_phit_ac_dn4 = assign8990_e8062_d_n4;
        locals.var_sat_phit_ac_dn6 = assign8990_e8062_d_n6;
        locals.var_sat_phit_ac_dn7 = assign8990_e8062_d_n7;
        locals.var_sat_phit_ac_dn8 = assign8990_e8062_d_n8;
        locals.var_sat_phit_ac_dn9 = assign8990_e8062_d_n9;
        locals.var_sat_phit_ac_rv = 0.0;

        let (assign9000_e8068, assign9000_e8068_d_n4, assign9000_e8068_d_n6, assign9000_e8068_d_n7, assign9000_e8068_d_n8, assign9000_e8068_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9000_e8066: f64 = (locals.var_alp1_i * locals.var_inv_phit);
        (assign9000_e8066, (locals.var_alp1_i * locals.var_inv_phit_dn4), (locals.var_alp1_i * locals.var_inv_phit_dn6), (locals.var_alp1_i * locals.var_inv_phit_dn7), (locals.var_alp1_i * locals.var_inv_phit_dn8), (locals.var_alp1_i * locals.var_inv_phit_dn9),)
    } else {
        (locals.var_alp1_phit, locals.var_alp1_phit_dn4, locals.var_alp1_phit_dn6, locals.var_alp1_phit_dn7, locals.var_alp1_phit_dn8, locals.var_alp1_phit_dn9,)
    }
};
        locals.var_alp1_phit = assign9000_e8068;
        locals.var_alp1_phit_dn4 = assign9000_e8068_d_n4;
        locals.var_alp1_phit_dn6 = assign9000_e8068_d_n6;
        locals.var_alp1_phit_dn7 = assign9000_e8068_d_n7;
        locals.var_alp1_phit_dn8 = assign9000_e8068_d_n8;
        locals.var_alp1_phit_dn9 = assign9000_e8068_d_n9;
        locals.var_alp1_phit_rv = 0.0;

        let (assign9010_e8076, assign9010_e8076_d_n4, assign9010_e8076_d_n6, assign9010_e8076_d_n7, assign9010_e8076_d_n8, assign9010_e8076_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9010_e8071: f64 = (-locals.var_stig_i);
        let assign9010_e8073: f64 = (assign9010_e8071 * locals.var_lnrtn);
        let assign9010_e8074: f64 = (assign9010_e8073).exp();
        (assign9010_e8074, (assign9010_e8074 * (assign9010_e8071 * locals.var_lnrtn_dn4)), (assign9010_e8074 * (assign9010_e8071 * locals.var_lnrtn_dn6)), (assign9010_e8074 * (assign9010_e8071 * locals.var_lnrtn_dn7)), (assign9010_e8074 * (assign9010_e8071 * locals.var_lnrtn_dn8)), (assign9010_e8074 * (assign9010_e8071 * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_ig, locals.var_tf_ig_dn4, locals.var_tf_ig_dn6, locals.var_tf_ig_dn7, locals.var_tf_ig_dn8, locals.var_tf_ig_dn9,)
    }
};
        locals.var_tf_ig = assign9010_e8076;
        locals.var_tf_ig_dn4 = assign9010_e8076_d_n4;
        locals.var_tf_ig_dn6 = assign9010_e8076_d_n6;
        locals.var_tf_ig_dn7 = assign9010_e8076_d_n7;
        locals.var_tf_ig_dn8 = assign9010_e8076_d_n8;
        locals.var_tf_ig_dn9 = assign9010_e8076_d_n9;
        locals.var_tf_ig_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (assign9020_e8082, assign9020_e8082_d_n4, assign9020_e8082_d_n6, assign9020_e8082_d_n7, assign9020_e8082_d_n8, assign9020_e8082_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9020_e8080: f64 = (locals.var_iginv_t * locals.var_tf_ig);
        (assign9020_e8080, (locals.var_iginv_t * locals.var_tf_ig_dn4), (locals.var_iginv_t * locals.var_tf_ig_dn6), (locals.var_iginv_t * locals.var_tf_ig_dn7), (locals.var_iginv_t * locals.var_tf_ig_dn8), (locals.var_iginv_t * locals.var_tf_ig_dn9),)
    } else {
        (locals.var_iginv_i, locals.var_iginv_i_dn4, locals.var_iginv_i_dn6, locals.var_iginv_i_dn7, locals.var_iginv_i_dn8, locals.var_iginv_i_dn9,)
    }
};
        locals.var_iginv_i = assign9020_e8082;
        locals.var_iginv_i_dn4 = assign9020_e8082_d_n4;
        locals.var_iginv_i_dn6 = assign9020_e8082_d_n6;
        locals.var_iginv_i_dn7 = assign9020_e8082_d_n7;
        locals.var_iginv_i_dn8 = assign9020_e8082_d_n8;
        locals.var_iginv_i_dn9 = assign9020_e8082_d_n9;
        locals.var_iginv_i_rv = 0.0;

        let (assign9030_e8088, assign9030_e8088_d_n4, assign9030_e8088_d_n6, assign9030_e8088_d_n7, assign9030_e8088_d_n8, assign9030_e8088_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9030_e8086: f64 = (locals.var_igovinv_t * locals.var_tf_ig);
        (assign9030_e8086, (locals.var_igovinv_t * locals.var_tf_ig_dn4), (locals.var_igovinv_t * locals.var_tf_ig_dn6), (locals.var_igovinv_t * locals.var_tf_ig_dn7), (locals.var_igovinv_t * locals.var_tf_ig_dn8), (locals.var_igovinv_t * locals.var_tf_ig_dn9),)
    } else {
        (locals.var_igovinv_i, locals.var_igovinv_i_dn4, locals.var_igovinv_i_dn6, locals.var_igovinv_i_dn7, locals.var_igovinv_i_dn8, locals.var_igovinv_i_dn9,)
    }
};
        locals.var_igovinv_i = assign9030_e8088;
        locals.var_igovinv_i_dn4 = assign9030_e8088_d_n4;
        locals.var_igovinv_i_dn6 = assign9030_e8088_d_n6;
        locals.var_igovinv_i_dn7 = assign9030_e8088_d_n7;
        locals.var_igovinv_i_dn8 = assign9030_e8088_d_n8;
        locals.var_igovinv_i_dn9 = assign9030_e8088_d_n9;
        locals.var_igovinv_i_rv = 0.0;

        let (assign9040_e8094, assign9040_e8094_d_n4, assign9040_e8094_d_n6, assign9040_e8094_d_n7, assign9040_e8094_d_n8, assign9040_e8094_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9040_e8092: f64 = (locals.var_igovinvd_t * locals.var_tf_ig);
        (assign9040_e8092, (locals.var_igovinvd_t * locals.var_tf_ig_dn4), (locals.var_igovinvd_t * locals.var_tf_ig_dn6), (locals.var_igovinvd_t * locals.var_tf_ig_dn7), (locals.var_igovinvd_t * locals.var_tf_ig_dn8), (locals.var_igovinvd_t * locals.var_tf_ig_dn9),)
    } else {
        (locals.var_igovinvd_i, locals.var_igovinvd_i_dn4, locals.var_igovinvd_i_dn6, locals.var_igovinvd_i_dn7, locals.var_igovinvd_i_dn8, locals.var_igovinvd_i_dn9,)
    }
};
        locals.var_igovinvd_i = assign9040_e8094;
        locals.var_igovinvd_i_dn4 = assign9040_e8094_d_n4;
        locals.var_igovinvd_i_dn6 = assign9040_e8094_d_n6;
        locals.var_igovinvd_i_dn7 = assign9040_e8094_d_n7;
        locals.var_igovinvd_i_dn8 = assign9040_e8094_d_n8;
        locals.var_igovinvd_i_dn9 = assign9040_e8094_d_n9;
        locals.var_igovinvd_i_rv = 0.0;

        let (assign9050_e8100, assign9050_e8100_d_n4, assign9050_e8100_d_n6, assign9050_e8100_d_n7, assign9050_e8100_d_n8, assign9050_e8100_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9050_e8098: f64 = (locals.var_igovacc_t * locals.var_tf_ig);
        (assign9050_e8098, (locals.var_igovacc_t * locals.var_tf_ig_dn4), (locals.var_igovacc_t * locals.var_tf_ig_dn6), (locals.var_igovacc_t * locals.var_tf_ig_dn7), (locals.var_igovacc_t * locals.var_tf_ig_dn8), (locals.var_igovacc_t * locals.var_tf_ig_dn9),)
    } else {
        (locals.var_igovacc_i, locals.var_igovacc_i_dn4, locals.var_igovacc_i_dn6, locals.var_igovacc_i_dn7, locals.var_igovacc_i_dn8, locals.var_igovacc_i_dn9,)
    }
};
        locals.var_igovacc_i = assign9050_e8100;
        locals.var_igovacc_i_dn4 = assign9050_e8100_d_n4;
        locals.var_igovacc_i_dn6 = assign9050_e8100_d_n6;
        locals.var_igovacc_i_dn7 = assign9050_e8100_d_n7;
        locals.var_igovacc_i_dn8 = assign9050_e8100_d_n8;
        locals.var_igovacc_i_dn9 = assign9050_e8100_d_n9;
        locals.var_igovacc_i_rv = 0.0;

        let (assign9060_e8106, assign9060_e8106_d_n4, assign9060_e8106_d_n6, assign9060_e8106_d_n7, assign9060_e8106_d_n8, assign9060_e8106_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9060_e8104: f64 = (locals.var_igovaccd_t * locals.var_tf_ig);
        (assign9060_e8104, (locals.var_igovaccd_t * locals.var_tf_ig_dn4), (locals.var_igovaccd_t * locals.var_tf_ig_dn6), (locals.var_igovaccd_t * locals.var_tf_ig_dn7), (locals.var_igovaccd_t * locals.var_tf_ig_dn8), (locals.var_igovaccd_t * locals.var_tf_ig_dn9),)
    } else {
        (locals.var_igovaccd_i, locals.var_igovaccd_i_dn4, locals.var_igovaccd_i_dn6, locals.var_igovaccd_i_dn7, locals.var_igovaccd_i_dn8, locals.var_igovaccd_i_dn9,)
    }
};
        locals.var_igovaccd_i = assign9060_e8106;
        locals.var_igovaccd_i_dn4 = assign9060_e8106_d_n4;
        locals.var_igovaccd_i_dn6 = assign9060_e8106_d_n6;
        locals.var_igovaccd_i_dn7 = assign9060_e8106_d_n7;
        locals.var_igovaccd_i_dn8 = assign9060_e8106_d_n8;
        locals.var_igovaccd_i_dn9 = assign9060_e8106_d_n9;
        locals.var_igovaccd_i_rv = 0.0;

        let (assign9070_e8114, assign9070_e8114_d_n4, assign9070_e8114_d_n6, assign9070_e8114_d_n7, assign9070_e8114_d_n8, assign9070_e8114_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9070_e8109: f64 = (-locals.var_stigfn_i);
        let assign9070_e8111: f64 = (assign9070_e8109 * locals.var_lnrtn);
        let assign9070_e8112: f64 = (assign9070_e8111).exp();
        (assign9070_e8112, (assign9070_e8112 * (assign9070_e8109 * locals.var_lnrtn_dn4)), (assign9070_e8112 * (assign9070_e8109 * locals.var_lnrtn_dn6)), (assign9070_e8112 * (assign9070_e8109 * locals.var_lnrtn_dn7)), (assign9070_e8112 * (assign9070_e8109 * locals.var_lnrtn_dn8)), (assign9070_e8112 * (assign9070_e8109 * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_ig, locals.var_tf_ig_dn4, locals.var_tf_ig_dn6, locals.var_tf_ig_dn7, locals.var_tf_ig_dn8, locals.var_tf_ig_dn9,)
    }
};
        locals.var_tf_ig = assign9070_e8114;
        locals.var_tf_ig_dn4 = assign9070_e8114_d_n4;
        locals.var_tf_ig_dn6 = assign9070_e8114_d_n6;
        locals.var_tf_ig_dn7 = assign9070_e8114_d_n7;
        locals.var_tf_ig_dn8 = assign9070_e8114_d_n8;
        locals.var_tf_ig_dn9 = assign9070_e8114_d_n9;
        locals.var_tf_ig_rv = 0.0;

        let (assign9100_e8132, assign9100_e8132_d_n4, assign9100_e8132_d_n6, assign9100_e8132_d_n7, assign9100_e8132_d_n8, assign9100_e8132_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9100_e8130: f64 = (0.5 * locals.var_eg);
        (assign9100_e8130, (0.5 * locals.var_eg_dn4), (0.5 * locals.var_eg_dn6), (0.5 * locals.var_eg_dn7), (0.5 * locals.var_eg_dn8), (0.5 * locals.var_eg_dn9),)
    } else {
        (locals.var_alpha_b, locals.var_alpha_b_dn4, locals.var_alpha_b_dn6, locals.var_alpha_b_dn7, locals.var_alpha_b_dn8, locals.var_alpha_b_dn9,)
    }
};
        locals.var_alpha_b = assign9100_e8132;
        locals.var_alpha_b_dn4 = assign9100_e8132_d_n4;
        locals.var_alpha_b_dn6 = assign9100_e8132_d_n6;
        locals.var_alpha_b_dn7 = assign9100_e8132_d_n7;
        locals.var_alpha_b_dn8 = assign9100_e8132_d_n8;
        locals.var_alpha_b_dn9 = assign9100_e8132_d_n9;
        locals.var_alpha_b_rv = 0.0;

        let (assign9110_e8138, assign9110_e8138_d_n4, assign9110_e8138_d_n6, assign9110_e8138_d_n7, assign9110_e8138_d_n8, assign9110_e8138_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9110_e8136: f64 = (locals.var_gco_i * locals.var_phit);
        (assign9110_e8136, (locals.var_gco_i * locals.var_phit_dn4), (locals.var_gco_i * locals.var_phit_dn6), (locals.var_gco_i * locals.var_phit_dn7), (locals.var_gco_i * locals.var_phit_dn8), (locals.var_gco_i * locals.var_phit_dn9),)
    } else {
        (locals.var_dch, locals.var_dch_dn4, locals.var_dch_dn6, locals.var_dch_dn7, locals.var_dch_dn8, locals.var_dch_dn9,)
    }
};
        locals.var_dch = assign9110_e8138;
        locals.var_dch_dn4 = assign9110_e8138_d_n4;
        locals.var_dch_dn6 = assign9110_e8138_d_n6;
        locals.var_dch_dn7 = assign9110_e8138_d_n7;
        locals.var_dch_dn8 = assign9110_e8138_d_n8;
        locals.var_dch_dn9 = assign9110_e8138_d_n9;
        locals.var_dch_rv = 0.0;

        let (assign9120_e8144, assign9120_e8144_d_n4, assign9120_e8144_d_n6, assign9120_e8144_d_n7, assign9120_e8144_d_n8, assign9120_e8144_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9120_e8142: f64 = (locals.var_gco_i * locals.var_phit0);
        (assign9120_e8142, (locals.var_gco_i * locals.var_phit0_dn4), (locals.var_gco_i * locals.var_phit0_dn6), (locals.var_gco_i * locals.var_phit0_dn7), (locals.var_gco_i * locals.var_phit0_dn8), (locals.var_gco_i * locals.var_phit0_dn9),)
    } else {
        (locals.var_dov, locals.var_dov_dn4, locals.var_dov_dn6, locals.var_dov_dn7, locals.var_dov_dn8, locals.var_dov_dn9,)
    }
};
        locals.var_dov = assign9120_e8144;
        locals.var_dov_dn4 = assign9120_e8144_d_n4;
        locals.var_dov_dn6 = assign9120_e8144_d_n6;
        locals.var_dov_dn7 = assign9120_e8144_d_n7;
        locals.var_dov_dn8 = assign9120_e8144_d_n8;
        locals.var_dov_dn9 = assign9120_e8144_d_n9;
        locals.var_dov_rv = 0.0;

        let (assign9130_e8154, assign9130_e8154_d_n4, assign9130_e8154_d_n6, assign9130_e8154_d_n7, assign9130_e8154_d_n8, assign9130_e8154_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9130_e8150: f64 = (locals.var_niginv_i * locals.var_eg_2phit);
        let assign9130_e8151: f64 = (1.0 + assign9130_e8150);
        let assign9130_e8152: f64 = (1.0 / assign9130_e8151);
        (assign9130_e8152, (-((locals.var_niginv_i * locals.var_eg_2phit_dn4) / (assign9130_e8151 * assign9130_e8151))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn6) / (assign9130_e8151 * assign9130_e8151))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn7) / (assign9130_e8151 * assign9130_e8151))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn8) / (assign9130_e8151 * assign9130_e8151))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn9) / (assign9130_e8151 * assign9130_e8151))),)
    } else {
        (locals.var_n_iginv, locals.var_n_iginv_dn4, locals.var_n_iginv_dn6, locals.var_n_iginv_dn7, locals.var_n_iginv_dn8, locals.var_n_iginv_dn9,)
    }
};
        locals.var_n_iginv = assign9130_e8154;
        locals.var_n_iginv_dn4 = assign9130_e8154_d_n4;
        locals.var_n_iginv_dn6 = assign9130_e8154_d_n6;
        locals.var_n_iginv_dn7 = assign9130_e8154_d_n7;
        locals.var_n_iginv_dn8 = assign9130_e8154_d_n8;
        locals.var_n_iginv_dn9 = assign9130_e8154_d_n9;
        locals.var_n_iginv_rv = 0.0;

        let (assign9140_e8160, assign9140_e8160_d_n4, assign9140_e8160_d_n6, assign9140_e8160_d_n7, assign9140_e8160_d_n8, assign9140_e8160_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9140_e8158: f64 = (locals.var_toxp_i * 500000000.0);
        (assign9140_e8158, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign9140_e8160;
        locals.var_temp_dn4 = assign9140_e8160_d_n4;
        locals.var_temp_dn6 = assign9140_e8160_d_n6;
        locals.var_temp_dn7 = assign9140_e8160_d_n7;
        locals.var_temp_dn8 = assign9140_e8160_d_n8;
        locals.var_temp_dn9 = assign9140_e8160_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign9150_e8191, assign9150_e8191_d_n4, assign9150_e8191_d_n6, assign9150_e8191_d_n7, assign9150_e8191_d_n8, assign9150_e8191_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9150_e8166: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign9150_e8167: f64 = (1.0 + assign9150_e8166);
        let assign9150_e8169: f64 = assign9150_e8167;
        let assign9150_e8173: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign9150_e8174: f64 = (1.0 + assign9150_e8173);
        let assign9150_e8176: f64 = assign9150_e8174;
        let assign9150_e8180: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign9150_e8181: f64 = (1.0 + assign9150_e8180);
        let assign9150_e8183: f64 = assign9150_e8181;
        let assign9150_e8184: f64 = (assign9150_e8176 * assign9150_e8183);
        let assign9150_e8186: f64 = (assign9150_e8184 + 0.01);
        let assign9150_e8187: f64 = (assign9150_e8186).sqrt();
        let assign9150_e8188: f64 = (assign9150_e8169 + assign9150_e8187);
        let assign9150_e8189: f64 = (0.5 * assign9150_e8188);
        (assign9150_e8189, (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn4) + ((((locals.var_stbgidl_i * locals.var_dt_dn4) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn4))) / (2.0 * assign9150_e8187)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn6) + ((((locals.var_stbgidl_i * locals.var_dt_dn6) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn6))) / (2.0 * assign9150_e8187)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn7) + ((((locals.var_stbgidl_i * locals.var_dt_dn7) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn7))) / (2.0 * assign9150_e8187)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn8) + ((((locals.var_stbgidl_i * locals.var_dt_dn8) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn8))) / (2.0 * assign9150_e8187)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn9) + ((((locals.var_stbgidl_i * locals.var_dt_dn9) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn9))) / (2.0 * assign9150_e8187)))),)
    } else {
        (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9,)
    }
};
        locals.var_tempm = assign9150_e8191;
        locals.var_tempm_dn4 = assign9150_e8191_d_n4;
        locals.var_tempm_dn6 = assign9150_e8191_d_n6;
        locals.var_tempm_dn7 = assign9150_e8191_d_n7;
        locals.var_tempm_dn8 = assign9150_e8191_d_n8;
        locals.var_tempm_dn9 = assign9150_e8191_d_n9;
        locals.var_tempm_rv = 0.0;

        let (assign9160_e8199, assign9160_e8199_d_n4, assign9160_e8199_d_n6, assign9160_e8199_d_n7, assign9160_e8199_d_n8, assign9160_e8199_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9160_e8195: f64 = (locals.var_bgidl_t * locals.var_tempm);
        let assign9160_e8197: f64 = (assign9160_e8195 * locals.var_temp);
        (assign9160_e8197, (((locals.var_bgidl_t * locals.var_tempm_dn4) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn4)), (((locals.var_bgidl_t * locals.var_tempm_dn6) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn6)), (((locals.var_bgidl_t * locals.var_tempm_dn7) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn7)), (((locals.var_bgidl_t * locals.var_tempm_dn8) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn8)), (((locals.var_bgidl_t * locals.var_tempm_dn9) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn9)),)
    } else {
        (locals.var_bgidl_i, locals.var_bgidl_i_dn4, locals.var_bgidl_i_dn6, locals.var_bgidl_i_dn7, locals.var_bgidl_i_dn8, locals.var_bgidl_i_dn9,)
    }
};
        locals.var_bgidl_i = assign9160_e8199;
        locals.var_bgidl_i_dn4 = assign9160_e8199_d_n4;
        locals.var_bgidl_i_dn6 = assign9160_e8199_d_n6;
        locals.var_bgidl_i_dn7 = assign9160_e8199_d_n7;
        locals.var_bgidl_i_dn8 = assign9160_e8199_d_n8;
        locals.var_bgidl_i_dn9 = assign9160_e8199_d_n9;
        locals.var_bgidl_i_rv = 0.0;

        let (assign9170_e8230, assign9170_e8230_d_n4, assign9170_e8230_d_n6, assign9170_e8230_d_n7, assign9170_e8230_d_n8, assign9170_e8230_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9170_e8205: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign9170_e8206: f64 = (1.0 + assign9170_e8205);
        let assign9170_e8208: f64 = assign9170_e8206;
        let assign9170_e8212: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign9170_e8213: f64 = (1.0 + assign9170_e8212);
        let assign9170_e8215: f64 = assign9170_e8213;
        let assign9170_e8219: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign9170_e8220: f64 = (1.0 + assign9170_e8219);
        let assign9170_e8222: f64 = assign9170_e8220;
        let assign9170_e8223: f64 = (assign9170_e8215 * assign9170_e8222);
        let assign9170_e8225: f64 = (assign9170_e8223 + 0.01);
        let assign9170_e8226: f64 = (assign9170_e8225).sqrt();
        let assign9170_e8227: f64 = (assign9170_e8208 + assign9170_e8226);
        let assign9170_e8228: f64 = (0.5 * assign9170_e8227);
        (assign9170_e8228, (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn4) + ((((locals.var_stbgidld_i * locals.var_dt_dn4) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn4))) / (2.0 * assign9170_e8226)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn6) + ((((locals.var_stbgidld_i * locals.var_dt_dn6) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn6))) / (2.0 * assign9170_e8226)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn7) + ((((locals.var_stbgidld_i * locals.var_dt_dn7) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn7))) / (2.0 * assign9170_e8226)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn8) + ((((locals.var_stbgidld_i * locals.var_dt_dn8) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn8))) / (2.0 * assign9170_e8226)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn9) + ((((locals.var_stbgidld_i * locals.var_dt_dn9) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn9))) / (2.0 * assign9170_e8226)))),)
    } else {
        (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9,)
    }
};
        locals.var_tempm = assign9170_e8230;
        locals.var_tempm_dn4 = assign9170_e8230_d_n4;
        locals.var_tempm_dn6 = assign9170_e8230_d_n6;
        locals.var_tempm_dn7 = assign9170_e8230_d_n7;
        locals.var_tempm_dn8 = assign9170_e8230_d_n8;
        locals.var_tempm_dn9 = assign9170_e8230_d_n9;
        locals.var_tempm_rv = 0.0;

        let (assign9180_e8238, assign9180_e8238_d_n4, assign9180_e8238_d_n6, assign9180_e8238_d_n7, assign9180_e8238_d_n8, assign9180_e8238_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9180_e8234: f64 = (locals.var_bgidld_t * locals.var_tempm);
        let assign9180_e8236: f64 = (assign9180_e8234 * locals.var_temp);
        (assign9180_e8236, (((locals.var_bgidld_t * locals.var_tempm_dn4) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn4)), (((locals.var_bgidld_t * locals.var_tempm_dn6) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn6)), (((locals.var_bgidld_t * locals.var_tempm_dn7) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn7)), (((locals.var_bgidld_t * locals.var_tempm_dn8) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn8)), (((locals.var_bgidld_t * locals.var_tempm_dn9) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn9)),)
    } else {
        (locals.var_bgidld_i, locals.var_bgidld_i_dn4, locals.var_bgidld_i_dn6, locals.var_bgidld_i_dn7, locals.var_bgidld_i_dn8, locals.var_bgidld_i_dn9,)
    }
};
        locals.var_bgidld_i = assign9180_e8238;
        locals.var_bgidld_i_dn4 = assign9180_e8238_d_n4;
        locals.var_bgidld_i_dn6 = assign9180_e8238_d_n6;
        locals.var_bgidld_i_dn7 = assign9180_e8238_d_n7;
        locals.var_bgidld_i_dn8 = assign9180_e8238_d_n8;
        locals.var_bgidld_i_dn9 = assign9180_e8238_d_n9;
        locals.var_bgidld_i_rv = 0.0;

        let (assign9190_e8248, assign9190_e8248_d_n4, assign9190_e8248_d_n6, assign9190_e8248_d_n7, assign9190_e8248_d_n8, assign9190_e8248_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9190_e8242: f64 = (-locals.var_sta2_i);
        let assign9190_e8244: f64 = (assign9190_e8242 * locals.var_lnrtn);
        let assign9190_e8245: f64 = (assign9190_e8244).exp();
        let assign9190_e8246: f64 = (locals.var_a2_t * assign9190_e8245);
        (assign9190_e8246, (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn4))), (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn6))), (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn7))), (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn8))), (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn9))),)
    } else {
        (locals.var_a2_i, locals.var_a2_i_dn4, locals.var_a2_i_dn6, locals.var_a2_i_dn7, locals.var_a2_i_dn8, locals.var_a2_i_dn9,)
    }
};
        locals.var_a2_i = assign9190_e8248;
        locals.var_a2_i_dn4 = assign9190_e8248_d_n4;
        locals.var_a2_i_dn6 = assign9190_e8248_d_n6;
        locals.var_a2_i_dn7 = assign9190_e8248_d_n7;
        locals.var_a2_i_dn8 = assign9190_e8248_d_n8;
        locals.var_a2_i_dn9 = assign9190_e8248_d_n9;
        locals.var_a2_i_rv = 0.0;

        let (assign9200_e8254, assign9200_e8254_d_n4, assign9200_e8254_d_n6, assign9200_e8254_d_n7, assign9200_e8254_d_n8, assign9200_e8254_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9200_e8252: f64 = (locals.var_areaq_i * locals.var_phit);
        (assign9200_e8252, (locals.var_areaq_i * locals.var_phit_dn4), (locals.var_areaq_i * locals.var_phit_dn6), (locals.var_areaq_i * locals.var_phit_dn7), (locals.var_areaq_i * locals.var_phit_dn8), (locals.var_areaq_i * locals.var_phit_dn9),)
    } else {
        (locals.var_area_phit, locals.var_area_phit_dn4, locals.var_area_phit_dn6, locals.var_area_phit_dn7, locals.var_area_phit_dn8, locals.var_area_phit_dn9,)
    }
};
        locals.var_area_phit = assign9200_e8254;
        locals.var_area_phit_dn4 = assign9200_e8254_d_n4;
        locals.var_area_phit_dn6 = assign9200_e8254_d_n6;
        locals.var_area_phit_dn7 = assign9200_e8254_d_n7;
        locals.var_area_phit_dn8 = assign9200_e8254_d_n8;
        locals.var_area_phit_dn9 = assign9200_e8254_d_n9;
        locals.var_area_phit_rv = 0.0;

        let (assign9210_e8266, assign9210_e8266_d_n4, assign9210_e8266_d_n6, assign9210_e8266_d_n7, assign9210_e8266_d_n8, assign9210_e8266_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9210_e8258: f64 = (0.25 * 1.602176565e-19);
        let assign9210_e8260: f64 = (assign9210_e8258 * locals.var_nsdac_i);
        let assign9210_e8263: f64 = (locals.var_epsch * locals.var_phit);
        let assign9210_e8264: f64 = (assign9210_e8260 / assign9210_e8263);
        (assign9210_e8264, (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn4)) / (assign9210_e8263 * assign9210_e8263))), (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn6)) / (assign9210_e8263 * assign9210_e8263))), (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn7)) / (assign9210_e8263 * assign9210_e8263))), (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn8)) / (assign9210_e8263 * assign9210_e8263))), (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn9)) / (assign9210_e8263 * assign9210_e8263))),)
    } else {
        (locals.var_inner_sd, locals.var_inner_sd_dn4, locals.var_inner_sd_dn6, locals.var_inner_sd_dn7, locals.var_inner_sd_dn8, locals.var_inner_sd_dn9,)
    }
};
        locals.var_inner_sd = assign9210_e8266;
        locals.var_inner_sd_dn4 = assign9210_e8266_d_n4;
        locals.var_inner_sd_dn6 = assign9210_e8266_d_n6;
        locals.var_inner_sd_dn7 = assign9210_e8266_d_n7;
        locals.var_inner_sd_dn8 = assign9210_e8266_d_n8;
        locals.var_inner_sd_dn9 = assign9210_e8266_d_n9;
        locals.var_inner_sd_rv = 0.0;

        let (assign9220_e8273, assign9220_e8273_d_n4, assign9220_e8273_d_n6, assign9220_e8273_d_n7, assign9220_e8273_d_n8, assign9220_e8273_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9220_e8270: f64 = (locals.var_nsdac_i / locals.var_neff);
        let assign9220_e8271: f64 = (assign9220_e8270).ln();
        (assign9220_e8271, ((-((locals.var_nsdac_i * locals.var_neff_dn4) / (locals.var_neff * locals.var_neff))) / assign9220_e8270), ((-((locals.var_nsdac_i * locals.var_neff_dn6) / (locals.var_neff * locals.var_neff))) / assign9220_e8270), ((-((locals.var_nsdac_i * locals.var_neff_dn7) / (locals.var_neff * locals.var_neff))) / assign9220_e8270), ((-((locals.var_nsdac_i * locals.var_neff_dn8) / (locals.var_neff * locals.var_neff))) / assign9220_e8270), ((-((locals.var_nsdac_i * locals.var_neff_dn9) / (locals.var_neff * locals.var_neff))) / assign9220_e8270),)
    } else {
        (locals.var_xsd, locals.var_xsd_dn4, locals.var_xsd_dn6, locals.var_xsd_dn7, locals.var_xsd_dn8, locals.var_xsd_dn9,)
    }
};
        locals.var_xsd = assign9220_e8273;
        locals.var_xsd_dn4 = assign9220_e8273_d_n4;
        locals.var_xsd_dn6 = assign9220_e8273_d_n6;
        locals.var_xsd_dn7 = assign9220_e8273_d_n7;
        locals.var_xsd_dn8 = assign9220_e8273_d_n8;
        locals.var_xsd_dn9 = assign9220_e8273_d_n9;
        locals.var_xsd_rv = 0.0;

        let (assign9230_e8281, assign9230_e8281_d_n4, assign9230_e8281_d_n6, assign9230_e8281_d_n7, assign9230_e8281_d_n8, assign9230_e8281_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9230_e8277: f64 = (locals.var_fif_i * 1.25e-6);
        let assign9230_e8279: f64 = (assign9230_e8277 * locals.var_phit);
        (assign9230_e8279, (assign9230_e8277 * locals.var_phit_dn4), (assign9230_e8277 * locals.var_phit_dn6), (assign9230_e8277 * locals.var_phit_dn7), (assign9230_e8277 * locals.var_phit_dn8), (assign9230_e8277 * locals.var_phit_dn9),)
    } else {
        (locals.var_fif_phit, locals.var_fif_phit_dn4, locals.var_fif_phit_dn6, locals.var_fif_phit_dn7, locals.var_fif_phit_dn8, locals.var_fif_phit_dn9,)
    }
};
        locals.var_fif_phit = assign9230_e8281;
        locals.var_fif_phit_dn4 = assign9230_e8281_d_n4;
        locals.var_fif_phit_dn6 = assign9230_e8281_d_n6;
        locals.var_fif_phit_dn7 = assign9230_e8281_d_n7;
        locals.var_fif_phit_dn8 = assign9230_e8281_d_n8;
        locals.var_fif_phit_dn9 = assign9230_e8281_d_n9;
        locals.var_fif_phit_rv = 0.0;

        let (assign9240_e8288, assign9240_e8288_d_n4, assign9240_e8288_d_n6, assign9240_e8288_d_n7, assign9240_e8288_d_n8, assign9240_e8288_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9240_e8285: f64 = (locals.var_strth_i * locals.var_lnrtn);
        let assign9240_e8286: f64 = (assign9240_e8285).exp();
        (assign9240_e8286, (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn4)), (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn6)), (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn7)), (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn8)), (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn9)),)
    } else {
        (locals.var_tf_rth, locals.var_tf_rth_dn4, locals.var_tf_rth_dn6, locals.var_tf_rth_dn7, locals.var_tf_rth_dn8, locals.var_tf_rth_dn9,)
    }
};
        locals.var_tf_rth = assign9240_e8288;
        locals.var_tf_rth_dn4 = assign9240_e8288_d_n4;
        locals.var_tf_rth_dn6 = assign9240_e8288_d_n6;
        locals.var_tf_rth_dn7 = assign9240_e8288_d_n7;
        locals.var_tf_rth_dn8 = assign9240_e8288_d_n8;
        locals.var_tf_rth_dn9 = assign9240_e8288_d_n9;
        locals.var_tf_rth_rv = 0.0;

        let (assign9250_e8294, assign9250_e8294_d_n4, assign9250_e8294_d_n6, assign9250_e8294_d_n7, assign9250_e8294_d_n8, assign9250_e8294_d_n9,) = {
    if (locals.var_guard257 != 0.0) {
        let assign9250_e8292: f64 = (locals.var_rth_t * locals.var_tf_rth);
        (assign9250_e8292, ((locals.var_rth_t_dn4 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn4)), ((locals.var_rth_t_dn6 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn6)), ((locals.var_rth_t_dn7 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn7)), ((locals.var_rth_t_dn8 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn8)), ((locals.var_rth_t_dn9 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn9)),)
    } else {
        (locals.var_rth_i, locals.var_rth_i_dn4, locals.var_rth_i_dn6, locals.var_rth_i_dn7, locals.var_rth_i_dn8, locals.var_rth_i_dn9,)
    }
};
        locals.var_rth_i = assign9250_e8294;
        locals.var_rth_i_dn4 = assign9250_e8294_d_n4;
        locals.var_rth_i_dn6 = assign9250_e8294_d_n6;
        locals.var_rth_i_dn7 = assign9250_e8294_d_n7;
        locals.var_rth_i_dn8 = assign9250_e8294_d_n8;
        locals.var_rth_i_dn9 = assign9250_e8294_d_n9;
        locals.var_rth_i_rv = 0.0;

        let assign9280_e8311: f64 = 1.0;
        let assign9280_e8312: f64 = if p.p14 == assign9280_e8311 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign9280_e8312;
        locals.var_guard263_rv = 0.0;

        let (assign9290_e8316, assign9290_e8316_d_n6, assign9290_e8316_d_n9,) = {
    if (locals.var_guard263 != 0.0) {
        ((nv9 - nv6), -1.0, 1.0,)
    } else {
        (locals.var_vgsu, locals.var_vgsu_dn6, locals.var_vgsu_dn9,)
    }
};
        locals.var_vgsu = assign9290_e8316;
        locals.var_vgsu_dn6 = assign9290_e8316_d_n6;
        locals.var_vgsu_dn9 = assign9290_e8316_d_n9;
        locals.var_vgsu_rv = 0.0;

        let (assign9300_e8320, assign9300_e8320_d_n6, assign9300_e8320_d_n7,) = {
    if (locals.var_guard263 != 0.0) {
        ((nv7 - nv6), -1.0, 1.0,)
    } else {
        (locals.var_vdsu, locals.var_vdsu_dn6, locals.var_vdsu_dn7,)
    }
};
        locals.var_vdsu = assign9300_e8320;
        locals.var_vdsu_dn6 = assign9300_e8320_d_n6;
        locals.var_vdsu_dn7 = assign9300_e8320_d_n7;
        locals.var_vdsu_rv = 0.0;

        let (assign9310_e8324, assign9310_e8324_d_n6, assign9310_e8324_d_n8,) = {
    if (locals.var_guard263 != 0.0) {
        ((nv6 - nv8), 1.0, -1.0,)
    } else {
        (locals.var_vsbu, locals.var_vsbu_dn6, locals.var_vsbu_dn8,)
    }
};
        locals.var_vsbu = assign9310_e8324;
        locals.var_vsbu_dn6 = assign9310_e8324_d_n6;
        locals.var_vsbu_dn8 = assign9310_e8324_d_n8;
        locals.var_vsbu_rv = 0.0;

        let (assign9320_e8330, assign9320_e8330_d_n6, assign9320_e8330_d_n9,) = {
    if (locals.var_guard263 == 0.0) {
        let assign9320_e8328: f64 = (-(nv9 - nv6));
        (assign9320_e8328, 1.0, (-1.0),)
    } else {
        (locals.var_vgsu, locals.var_vgsu_dn6, locals.var_vgsu_dn9,)
    }
};
        locals.var_vgsu = assign9320_e8330;
        locals.var_vgsu_dn6 = assign9320_e8330_d_n6;
        locals.var_vgsu_dn9 = assign9320_e8330_d_n9;
        locals.var_vgsu_rv = 0.0;

        let (assign9330_e8336, assign9330_e8336_d_n6, assign9330_e8336_d_n7,) = {
    if (locals.var_guard263 == 0.0) {
        let assign9330_e8334: f64 = (-(nv7 - nv6));
        (assign9330_e8334, 1.0, (-1.0),)
    } else {
        (locals.var_vdsu, locals.var_vdsu_dn6, locals.var_vdsu_dn7,)
    }
};
        locals.var_vdsu = assign9330_e8336;
        locals.var_vdsu_dn6 = assign9330_e8336_d_n6;
        locals.var_vdsu_dn7 = assign9330_e8336_d_n7;
        locals.var_vdsu_rv = 0.0;

        let (assign9340_e8342, assign9340_e8342_d_n6, assign9340_e8342_d_n8,) = {
    if (locals.var_guard263 == 0.0) {
        let assign9340_e8340: f64 = (-(nv6 - nv8));
        (assign9340_e8340, (-1.0), 1.0,)
    } else {
        (locals.var_vsbu, locals.var_vsbu_dn6, locals.var_vsbu_dn8,)
    }
};
        locals.var_vsbu = assign9340_e8342;
        locals.var_vsbu_dn6 = assign9340_e8342_d_n6;
        locals.var_vsbu_dn8 = assign9340_e8342_d_n8;
        locals.var_vsbu_rv = 0.0;

        let assign9350_e8344: f64 = (-locals.var_vdsu);
        locals.var_vsdu = assign9350_e8344;
        locals.var_vsdu_dn6 = (-locals.var_vdsu_dn6);
        locals.var_vsdu_dn7 = (-locals.var_vdsu_dn7);
        locals.var_vsdu_rv = 0.0;

        let assign9360_e8347: f64 = (locals.var_vgsu + locals.var_vsdu);
        locals.var_vgdu = assign9360_e8347;
        locals.var_vgdu_dn6 = (locals.var_vgsu_dn6 + locals.var_vsdu_dn6);
        locals.var_vgdu_dn7 = locals.var_vsdu_dn7;
        locals.var_vgdu_dn9 = locals.var_vgsu_dn9;
        locals.var_vgdu_rv = 0.0;

        let assign9370_e8350: f64 = (locals.var_vdsu + locals.var_vsbu);
        locals.var_vdbu = assign9370_e8350;
        locals.var_vdbu_dn6 = (locals.var_vdsu_dn6 + locals.var_vsbu_dn6);
        locals.var_vdbu_dn7 = locals.var_vdsu_dn7;
        locals.var_vdbu_dn8 = locals.var_vsbu_dn8;
        locals.var_vdbu_rv = 0.0;

        let assign9380_e8353: f64 = if locals.var_vdsu < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign9380_e8353;
        locals.var_guard264_rv = 0.0;

        let (assign9390_e8358,) = {
    if (locals.var_guard264 != 0.0) {
        let assign9390_e8356: f64 = (-1.0);
        (assign9390_e8356,)
    } else {
        (locals.var_sigvds,)
    }
};
        locals.var_sigvds = assign9390_e8358;
        locals.var_sigvds_rv = 0.0;

    }
}
