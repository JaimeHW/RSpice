#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign00_e2035: f64 = ctx_temp;
        let assign00_e2037: f64 = (assign00_e2035 + p.p0);
        locals.var_devtemp = assign00_e2037;
        locals.var_devtemp_dn4 = 0.0;
        locals.var_devtemp_dn5 = 0.0;
        locals.var_devtemp_dn6 = 0.0;

        let assign10_e2040: f64 = (p.p126 + 273.15);
        locals.var_tnom = assign10_e2040;

        locals.var_b4soiln = p.p336;

        locals.var_b4soisoimod = p.p21;

        locals.var_b4soirbody = p.p348;

        locals.var_b4soicf = p.p213;

        locals.var_b4soicgso = p.p127;
        locals.var_b4soicgso_dn3 = 0.0;
        locals.var_b4soicgso_dn4 = 0.0;
        locals.var_b4soicgso_dn5 = 0.0;
        locals.var_b4soicgso_dn6 = 0.0;
        locals.var_b4soicgso_dn7 = 0.0;
        locals.var_b4soicgso_dn8 = 0.0;
        locals.var_b4soicgso_dn9 = 0.0;
        locals.var_b4soicgso_dn10 = 0.0;
        locals.var_b4soicgso_dn11 = 0.0;
        locals.var_b4soicgso_dn12 = 0.0;

        locals.var_b4soigatesidewalljctdpotential = p.p182;

        locals.var_b4soicgeo = p.p350;

        locals.var_b4soicsdmin = p.p355;
        locals.var_b4soicsdmin_dn3 = 0.0;
        locals.var_b4soicsdmin_dn4 = 0.0;
        locals.var_b4soicsdmin_dn5 = 0.0;
        locals.var_b4soicsdmin_dn6 = 0.0;
        locals.var_b4soicsdmin_dn7 = 0.0;
        locals.var_b4soicsdmin_dn8 = 0.0;
        locals.var_b4soicsdmin_dn9 = 0.0;
        locals.var_b4soicsdmin_dn10 = 0.0;
        locals.var_b4soicsdmin_dn11 = 0.0;
        locals.var_b4soicsdmin_dn12 = 0.0;

        locals.var_b4soiwlod = p.p234;

        locals.var_b4soikvsat = p.p236;

        locals.var_b4soicfrcoeff = p.p373;

        locals.var_b4soigatesidewalljctspotential = p.p181;

        let (assign140_e2056,) = {
    if (p.p41 != 0.0) {
        (3.9,)
    } else {
        (locals.var_epsrox,)
    }
};
        locals.var_epsrox = assign140_e2056;

        let (assign150_e2060,) = {
    if (p.p41 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_toxe,)
    }
};
        locals.var_toxe = assign150_e2060;

        let (assign160_e2066,) = {
    if (p.p41 != 0.0) {
        let assign160_e2064: f64 = (8.85418e-12 * p.p47);
        (assign160_e2064,)
    } else {
        (locals.var_epssub,)
    }
};
        locals.var_epssub = assign160_e2066;

        let (assign170_e2075,) = {
    if (p.p41 != 0.0) {
        let assign170_e2070: f64 = (2000000.0 * 1.602176462e-19);
        let assign170_e2072: f64 = (assign170_e2070 * locals.var_epssub);
        let assign170_e2073: f64 = (assign170_e2072).sqrt();
        (assign170_e2073,)
    } else {
        (locals.var_sqrt2qeps,)
    }
};
        locals.var_sqrt2qeps = assign170_e2075;

        let (assign180_e2083,) = {
    if (p.p41 != 0.0) {
        let assign180_e2079: f64 = (locals.var_epsrox * 8.85418e-12);
        let assign180_e2081: f64 = (assign180_e2079 / locals.var_toxe);
        (assign180_e2081,)
    } else {
        (locals.var_b4soicox,)
    }
};
        locals.var_b4soicox = assign180_e2083;

        let (assign190_e2088,) = {
    if (p.p41 == 0.0) {
        (p.p46,)
    } else {
        (locals.var_epsrox,)
    }
};
        locals.var_epsrox = assign190_e2088;

        let (assign200_e2093,) = {
    if (p.p41 == 0.0) {
        (p.p66,)
    } else {
        (locals.var_toxe,)
    }
};
        locals.var_toxe = assign200_e2093;

        let (assign210_e2098,) = {
    if (p.p41 == 0.0) {
        (1.03594e-10,)
    } else {
        (locals.var_epssub,)
    }
};
        locals.var_epssub = assign210_e2098;

        let (assign220_e2103,) = {
    if (p.p41 == 0.0) {
        (5.753e-12,)
    } else {
        (locals.var_sqrt2qeps,)
    }
};
        locals.var_sqrt2qeps = assign220_e2103;

        let (assign230_e2110,) = {
    if (p.p41 == 0.0) {
        let assign230_e2108: f64 = (3.453133e-11 / p.p66);
        (assign230_e2108,)
    } else {
        (locals.var_b4soicox,)
    }
};
        locals.var_b4soicox = assign230_e2110;

        let assign240_e2113: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard396 = assign240_e2113;

        let (assign490_e2232,) = {
    if (locals.var_guard396 != 0.0) {
        (0.0,)
    } else {
        (locals.var_b4soibodymod,)
    }
};
        locals.var_b4soibodymod = assign490_e2232;

        let assign500_e2234: f64 = 1.0;
        let assign500_e2236: f64 = if assign500_e2234 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign500_e2236;

        let (assign510_e2243,) = {
    if ((locals.var_guard396 == 0.0) && (locals.var_guard421 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_b4soibodymod,)
    }
};
        locals.var_b4soibodymod = assign510_e2243;

        let assign530_e2247: f64 = 1.0;
        let assign530_e2249: f64 = if assign530_e2247 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign530_e2249;

        let assign540_e2256: f64 = if ((locals.var_b4soirbody == 0.0) && (p.p349 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard424 = assign540_e2256;

        let (assign550_e2268,) = {
    if ((((locals.var_guard396 == 0.0) && (locals.var_guard421 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_b4soibodymod,)
    }
};
        locals.var_b4soibodymod = assign550_e2268;

        let (assign560_e2281,) = {
    if ((((locals.var_guard396 == 0.0) && (locals.var_guard421 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_b4soibodymod,)
    }
};
        locals.var_b4soibodymod = assign560_e2281;

        let assign570_e2288: f64 = if ((locals.var_b4soirbody == 0.0) && (p.p349 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard425 = assign570_e2288;

        let (assign580_e2301,) = {
    if ((((locals.var_guard396 == 0.0) && (locals.var_guard421 == 0.0)) && (locals.var_guard423 == 0.0)) && (locals.var_guard425 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_b4soirbody,)
    }
};
        locals.var_b4soirbody = assign580_e2301;

        let (assign590_e2314,) = {
    if ((((locals.var_guard396 == 0.0) && (locals.var_guard421 == 0.0)) && (locals.var_guard423 == 0.0)) && (locals.var_guard425 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_b4soibodymod,)
    }
};
        locals.var_b4soibodymod = assign590_e2314;

        let (assign600_e2328,) = {
    if ((((locals.var_guard396 == 0.0) && (locals.var_guard421 == 0.0)) && (locals.var_guard423 == 0.0)) && (locals.var_guard425 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_b4soibodymod,)
    }
};
        locals.var_b4soibodymod = assign600_e2328;

        let assign610_e2330: f64 = if param_given[213] { 1.0 } else { 0.0 };
        locals.var_guard426 = assign610_e2330;

        let (assign620_e2334,) = {
    if (locals.var_guard426 != 0.0) {
        (p.p213,)
    } else {
        (locals.var_b4soicf,)
    }
};
        locals.var_b4soicf = assign620_e2334;

        let (assign630_e2350,) = {
    if (locals.var_guard426 == 0.0) {
        let assign630_e2339: f64 = (2.0 * 3.453133e-11);
        let assign630_e2341: f64 = (assign630_e2339 / 3.141592653589793);
        let assign630_e2345: f64 = (4e-7 / p.p66);
        let assign630_e2346: f64 = (1.0 + assign630_e2345);
        let assign630_e2347: f64 = (assign630_e2346).ln();
        let assign630_e2348: f64 = (assign630_e2341 * assign630_e2347);
        (assign630_e2348,)
    } else {
        (locals.var_b4soicf,)
    }
};
        locals.var_b4soicf = assign630_e2350;

        let assign640_e2353: f64 = if locals.var_b4soigatesidewalljctspotential < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard498 = assign640_e2353;

        let (assign650_e2357,) = {
    if (locals.var_guard498 != 0.0) {
        (0.1,)
    } else {
        (locals.var_b4soigatesidewalljctspotential,)
    }
};
        locals.var_b4soigatesidewalljctspotential = assign650_e2357;

        let assign660_e2360: f64 = if locals.var_b4soigatesidewalljctdpotential < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard499 = assign660_e2360;

        let (assign670_e2364,) = {
    if (locals.var_guard499 != 0.0) {
        (0.1,)
    } else {
        (locals.var_b4soigatesidewalljctdpotential,)
    }
};
        locals.var_b4soigatesidewalljctdpotential = assign670_e2364;

        let assign680_e2367: f64 = (p.p126 + 273.15);
        locals.var_tnom = assign680_e2367;

        let assign690_e2370: f64 = (locals.var_devtemp / locals.var_tnom);
        locals.var_tempratio__blk441 = assign690_e2370;
        locals.var_tempratio__blk441_dn4 = (locals.var_devtemp_dn4 / locals.var_tnom);
        locals.var_tempratio__blk441_dn5 = (locals.var_devtemp_dn5 / locals.var_tnom);
        locals.var_tempratio__blk441_dn6 = (locals.var_devtemp_dn6 / locals.var_tnom);

        let (assign700_e2381,) = {
    if (p.p41 != 0.0) {
        let assign700_e2375: f64 = (locals.var_epsrox * 8.85418e-12);
        let assign700_e2376: f64 = (locals.var_epssub / assign700_e2375);
        let assign700_e2378: f64 = (assign700_e2376 * locals.var_toxe);
        let assign700_e2379: f64 = (assign700_e2378).sqrt();
        (assign700_e2379,)
    } else {
        (locals.var_b4soifactor1,)
    }
};
        locals.var_b4soifactor1 = assign700_e2381;

        let (assign710_e2391,) = {
    if (p.p41 == 0.0) {
        let assign710_e2386: f64 = (1.03594e-10 / 3.453133e-11);
        let assign710_e2388: f64 = (assign710_e2386 * p.p66);
        let assign710_e2389: f64 = (assign710_e2388).sqrt();
        (assign710_e2389,)
    } else {
        (locals.var_b4soifactor1,)
    }
};
        locals.var_b4soifactor1 = assign710_e2391;

        let assign720_e2394: f64 = if p.p41 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard500 = assign720_e2394;

        let (assign730_e2400,) = {
    if (locals.var_guard500 != 0.0) {
        let assign730_e2398: f64 = (8.617087e-5 * locals.var_tnom);
        (assign730_e2398,)
    } else {
        (locals.var_vtm0,)
    }
};
        locals.var_vtm0 = assign730_e2400;

        let (assign740_e2414,) = {
    if (locals.var_guard500 != 0.0) {
        let assign740_e2405: f64 = (0.000702 * locals.var_tnom);
        let assign740_e2407: f64 = (assign740_e2405 * locals.var_tnom);
        let assign740_e2410: f64 = (locals.var_tnom + 1108.0);
        let assign740_e2411: f64 = (assign740_e2407 / assign740_e2410);
        let assign740_e2412: f64 = (1.16 - assign740_e2411);
        (assign740_e2412,)
    } else {
        (locals.var_eg0,)
    }
};
        locals.var_eg0 = assign740_e2414;

        let (assign750_e2418,) = {
    if (locals.var_guard500 != 0.0) {
        (locals.var_eg0,)
    } else {
        (locals.var_b4soieg0,)
    }
};
        locals.var_b4soieg0 = assign750_e2418;

        let (assign760_e2424, assign760_e2424_d_n4, assign760_e2424_d_n5, assign760_e2424_d_n6,) = {
    if (locals.var_guard500 != 0.0) {
        let assign760_e2422: f64 = (8.617087e-5 * locals.var_devtemp);
        (assign760_e2422, (8.617087e-5 * locals.var_devtemp_dn4), (8.617087e-5 * locals.var_devtemp_dn5), (8.617087e-5 * locals.var_devtemp_dn6),)
    } else {
        (locals.var_b4soivtm, locals.var_b4soivtm_dn4, locals.var_b4soivtm_dn5, locals.var_b4soivtm_dn6,)
    }
};
        locals.var_b4soivtm = assign760_e2424;
        locals.var_b4soivtm_dn4 = assign760_e2424_d_n4;
        locals.var_b4soivtm_dn5 = assign760_e2424_d_n5;
        locals.var_b4soivtm_dn6 = assign760_e2424_d_n6;

        let (assign770_e2438, assign770_e2438_d_n4, assign770_e2438_d_n5, assign770_e2438_d_n6,) = {
    if (locals.var_guard500 != 0.0) {
        let assign770_e2429: f64 = (0.000702 * locals.var_devtemp);
        let assign770_e2431: f64 = (assign770_e2429 * locals.var_devtemp);
        let assign770_e2434: f64 = (locals.var_devtemp + 1108.0);
        let assign770_e2435: f64 = (assign770_e2431 / assign770_e2434);
        let assign770_e2436: f64 = (1.16 - assign770_e2435);
        (assign770_e2436, (-((((((0.000702 * locals.var_devtemp_dn4) * locals.var_devtemp) + (assign770_e2429 * locals.var_devtemp_dn4)) * assign770_e2434) - (assign770_e2431 * locals.var_devtemp_dn4)) / (assign770_e2434 * assign770_e2434))), (-((((((0.000702 * locals.var_devtemp_dn5) * locals.var_devtemp) + (assign770_e2429 * locals.var_devtemp_dn5)) * assign770_e2434) - (assign770_e2431 * locals.var_devtemp_dn5)) / (assign770_e2434 * assign770_e2434))), (-((((((0.000702 * locals.var_devtemp_dn6) * locals.var_devtemp) + (assign770_e2429 * locals.var_devtemp_dn6)) * assign770_e2434) - (assign770_e2431 * locals.var_devtemp_dn6)) / (assign770_e2434 * assign770_e2434))),)
    } else {
        (locals.var_eg, locals.var_eg_dn4, locals.var_eg_dn5, locals.var_eg_dn6,)
    }
};
        locals.var_eg = assign770_e2438;
        locals.var_eg_dn4 = assign770_e2438_d_n4;
        locals.var_eg_dn5 = assign770_e2438_d_n5;
        locals.var_eg_dn6 = assign770_e2438_d_n6;

        let (assign780_e2442, assign780_e2442_d_n4, assign780_e2442_d_n5, assign780_e2442_d_n6,) = {
    if (locals.var_guard500 != 0.0) {
        (locals.var_eg, locals.var_eg_dn4, locals.var_eg_dn5, locals.var_eg_dn6,)
    } else {
        (locals.var_b4soieg, locals.var_b4soieg_dn4, locals.var_b4soieg_dn5, locals.var_b4soieg_dn6,)
    }
};
        locals.var_b4soieg = assign780_e2442;
        locals.var_b4soieg_dn4 = assign780_e2442_d_n4;
        locals.var_b4soieg_dn5 = assign780_e2442_d_n5;
        locals.var_b4soieg_dn6 = assign780_e2442_d_n6;

        let (assign790_e2479, assign790_e2479_d_n4, assign790_e2479_d_n5, assign790_e2479_d_n6,) = {
    if (locals.var_guard500 != 0.0) {
        let assign790_e2447: f64 = (locals.var_devtemp / 300.15);
        let assign790_e2448: f64 = (14500000000.0 * assign790_e2447);
        let assign790_e2451: f64 = (locals.var_devtemp / 300.15);
        let assign790_e2452: f64 = (assign790_e2451).sqrt();
        let assign790_e2453: f64 = (assign790_e2448 * assign790_e2452);
        let (assign790_e2469, assign790_e2469_d_n4, assign790_e2469_d_n5, assign790_e2469_d_n6,) = {
            if (assign790_e2453 > 1e-38) {
                let assign790_e2459: f64 = (locals.var_devtemp / 300.15);
                let assign790_e2460: f64 = (14500000000.0 * assign790_e2459);
                let assign790_e2463: f64 = (locals.var_devtemp / 300.15);
                let assign790_e2464: f64 = (assign790_e2463).sqrt();
                let assign790_e2465: f64 = (assign790_e2460 * assign790_e2464);
                let assign790_e2466: f64 = (assign790_e2465).ln();
                (assign790_e2466, ((((14500000000.0 * (locals.var_devtemp_dn4 / 300.15)) * assign790_e2464) + (assign790_e2460 * ((locals.var_devtemp_dn4 / 300.15) / (2.0 * assign790_e2464)))) / assign790_e2465), ((((14500000000.0 * (locals.var_devtemp_dn5 / 300.15)) * assign790_e2464) + (assign790_e2460 * ((locals.var_devtemp_dn5 / 300.15) / (2.0 * assign790_e2464)))) / assign790_e2465), ((((14500000000.0 * (locals.var_devtemp_dn6 / 300.15)) * assign790_e2464) + (assign790_e2460 * ((locals.var_devtemp_dn6 / 300.15) / (2.0 * assign790_e2464)))) / assign790_e2465),)
            } else {
                let assign790_e2468: f64 = (-87.49823353377374);
                (assign790_e2468, 0.0, 0.0, 0.0,)
            }
        };
        let assign790_e2471: f64 = (assign790_e2469 + 21.5565981);
        let assign790_e2475: f64 = (2.0 * locals.var_b4soivtm);
        let assign790_e2476: f64 = (locals.var_eg / assign790_e2475);
        let assign790_e2477: f64 = (assign790_e2471 - assign790_e2476);
        (assign790_e2477, (assign790_e2469_d_n4 - (((locals.var_eg_dn4 * assign790_e2475) - (locals.var_eg * (2.0 * locals.var_b4soivtm_dn4))) / (assign790_e2475 * assign790_e2475))), (assign790_e2469_d_n5 - (((locals.var_eg_dn5 * assign790_e2475) - (locals.var_eg * (2.0 * locals.var_b4soivtm_dn5))) / (assign790_e2475 * assign790_e2475))), (assign790_e2469_d_n6 - (((locals.var_eg_dn6 * assign790_e2475) - (locals.var_eg * (2.0 * locals.var_b4soivtm_dn6))) / (assign790_e2475 * assign790_e2475))),)
    } else {
        (locals.var_lln_ni, locals.var_lln_ni_dn4, locals.var_lln_ni_dn5, locals.var_lln_ni_dn6,)
    }
};
        locals.var_lln_ni = assign790_e2479;
        locals.var_lln_ni_dn4 = assign790_e2479_d_n4;
        locals.var_lln_ni_dn5 = assign790_e2479_d_n5;
        locals.var_lln_ni_dn6 = assign790_e2479_d_n6;

        let (assign800_e2486,) = {
    if (locals.var_guard500 == 0.0) {
        let assign800_e2484: f64 = (8.617087e-5 * locals.var_tnom);
        (assign800_e2484,)
    } else {
        (locals.var_vtm0,)
    }
};
        locals.var_vtm0 = assign800_e2486;

        let (assign810_e2501,) = {
    if (locals.var_guard500 == 0.0) {
        let assign810_e2492: f64 = (p.p50 * locals.var_tnom);
        let assign810_e2494: f64 = (assign810_e2492 * locals.var_tnom);
        let assign810_e2497: f64 = (locals.var_tnom + p.p51);
        let assign810_e2498: f64 = (assign810_e2494 / assign810_e2497);
        let assign810_e2499: f64 = (p.p49 - assign810_e2498);
        (assign810_e2499,)
    } else {
        (locals.var_eg0,)
    }
};
        locals.var_eg0 = assign810_e2501;

        let (assign820_e2506,) = {
    if (locals.var_guard500 == 0.0) {
        (locals.var_eg0,)
    } else {
        (locals.var_b4soieg0,)
    }
};
        locals.var_b4soieg0 = assign820_e2506;

        let (assign830_e2513, assign830_e2513_d_n4, assign830_e2513_d_n5, assign830_e2513_d_n6,) = {
    if (locals.var_guard500 == 0.0) {
        let assign830_e2511: f64 = (8.617087e-5 * locals.var_devtemp);
        (assign830_e2511, (8.617087e-5 * locals.var_devtemp_dn4), (8.617087e-5 * locals.var_devtemp_dn5), (8.617087e-5 * locals.var_devtemp_dn6),)
    } else {
        (locals.var_b4soivtm, locals.var_b4soivtm_dn4, locals.var_b4soivtm_dn5, locals.var_b4soivtm_dn6,)
    }
};
        locals.var_b4soivtm = assign830_e2513;
        locals.var_b4soivtm_dn4 = assign830_e2513_d_n4;
        locals.var_b4soivtm_dn5 = assign830_e2513_d_n5;
        locals.var_b4soivtm_dn6 = assign830_e2513_d_n6;

        let (assign840_e2528, assign840_e2528_d_n4, assign840_e2528_d_n5, assign840_e2528_d_n6,) = {
    if (locals.var_guard500 == 0.0) {
        let assign840_e2519: f64 = (p.p50 * locals.var_devtemp);
        let assign840_e2521: f64 = (assign840_e2519 * locals.var_devtemp);
        let assign840_e2524: f64 = (locals.var_devtemp + p.p51);
        let assign840_e2525: f64 = (assign840_e2521 / assign840_e2524);
        let assign840_e2526: f64 = (p.p49 - assign840_e2525);
        (assign840_e2526, (-((((((p.p50 * locals.var_devtemp_dn4) * locals.var_devtemp) + (assign840_e2519 * locals.var_devtemp_dn4)) * assign840_e2524) - (assign840_e2521 * locals.var_devtemp_dn4)) / (assign840_e2524 * assign840_e2524))), (-((((((p.p50 * locals.var_devtemp_dn5) * locals.var_devtemp) + (assign840_e2519 * locals.var_devtemp_dn5)) * assign840_e2524) - (assign840_e2521 * locals.var_devtemp_dn5)) / (assign840_e2524 * assign840_e2524))), (-((((((p.p50 * locals.var_devtemp_dn6) * locals.var_devtemp) + (assign840_e2519 * locals.var_devtemp_dn6)) * assign840_e2524) - (assign840_e2521 * locals.var_devtemp_dn6)) / (assign840_e2524 * assign840_e2524))),)
    } else {
        (locals.var_eg, locals.var_eg_dn4, locals.var_eg_dn5, locals.var_eg_dn6,)
    }
};
        locals.var_eg = assign840_e2528;
        locals.var_eg_dn4 = assign840_e2528_d_n4;
        locals.var_eg_dn5 = assign840_e2528_d_n5;
        locals.var_eg_dn6 = assign840_e2528_d_n6;

        let (assign850_e2533, assign850_e2533_d_n4, assign850_e2533_d_n5, assign850_e2533_d_n6,) = {
    if (locals.var_guard500 == 0.0) {
        (locals.var_eg, locals.var_eg_dn4, locals.var_eg_dn5, locals.var_eg_dn6,)
    } else {
        (locals.var_b4soieg, locals.var_b4soieg_dn4, locals.var_b4soieg_dn5, locals.var_b4soieg_dn6,)
    }
};
        locals.var_b4soieg = assign850_e2533;
        locals.var_b4soieg_dn4 = assign850_e2533_d_n4;
        locals.var_b4soieg_dn5 = assign850_e2533_d_n5;
        locals.var_b4soieg_dn6 = assign850_e2533_d_n6;

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign860_e2575, assign860_e2575_d_n4, assign860_e2575_d_n5, assign860_e2575_d_n6,) = {
    if (locals.var_guard500 == 0.0) {
        let assign860_e2539: f64 = (locals.var_devtemp / locals.var_tnom);
        let assign860_e2540: f64 = (p.p48 * assign860_e2539);
        let assign860_e2543: f64 = (locals.var_devtemp / locals.var_tnom);
        let assign860_e2544: f64 = (assign860_e2543).sqrt();
        let assign860_e2545: f64 = (assign860_e2540 * assign860_e2544);
        let (assign860_e2561, assign860_e2561_d_n4, assign860_e2561_d_n5, assign860_e2561_d_n6,) = {
            if (assign860_e2545 > 1e-38) {
                let assign860_e2551: f64 = (locals.var_devtemp / locals.var_tnom);
                let assign860_e2552: f64 = (p.p48 * assign860_e2551);
                let assign860_e2555: f64 = (locals.var_devtemp / locals.var_tnom);
                let assign860_e2556: f64 = (assign860_e2555).sqrt();
                let assign860_e2557: f64 = (assign860_e2552 * assign860_e2556);
                let assign860_e2558: f64 = (assign860_e2557).ln();
                (assign860_e2558, ((((p.p48 * (locals.var_devtemp_dn4 / locals.var_tnom)) * assign860_e2556) + (assign860_e2552 * ((locals.var_devtemp_dn4 / locals.var_tnom) / (2.0 * assign860_e2556)))) / assign860_e2557), ((((p.p48 * (locals.var_devtemp_dn5 / locals.var_tnom)) * assign860_e2556) + (assign860_e2552 * ((locals.var_devtemp_dn5 / locals.var_tnom) / (2.0 * assign860_e2556)))) / assign860_e2557), ((((p.p48 * (locals.var_devtemp_dn6 / locals.var_tnom)) * assign860_e2556) + (assign860_e2552 * ((locals.var_devtemp_dn6 / locals.var_tnom) / (2.0 * assign860_e2556)))) / assign860_e2557),)
            } else {
                let assign860_e2560: f64 = (-87.49823353377374);
                (assign860_e2560, 0.0, 0.0, 0.0,)
            }
        };
        let assign860_e2565: f64 = (2.0 * locals.var_vtm0);
        let assign860_e2566: f64 = (locals.var_eg0 / assign860_e2565);
        let assign860_e2570: f64 = (2.0 * locals.var_b4soivtm);
        let assign860_e2571: f64 = (locals.var_eg / assign860_e2570);
        let assign860_e2572: f64 = (assign860_e2566 - assign860_e2571);
        let assign860_e2573: f64 = (assign860_e2561 + assign860_e2572);
        (assign860_e2573, (assign860_e2561_d_n4 + (-(((locals.var_eg_dn4 * assign860_e2570) - (locals.var_eg * (2.0 * locals.var_b4soivtm_dn4))) / (assign860_e2570 * assign860_e2570)))), (assign860_e2561_d_n5 + (-(((locals.var_eg_dn5 * assign860_e2570) - (locals.var_eg * (2.0 * locals.var_b4soivtm_dn5))) / (assign860_e2570 * assign860_e2570)))), (assign860_e2561_d_n6 + (-(((locals.var_eg_dn6 * assign860_e2570) - (locals.var_eg * (2.0 * locals.var_b4soivtm_dn6))) / (assign860_e2570 * assign860_e2570)))),)
    } else {
        (locals.var_lln_ni, locals.var_lln_ni_dn4, locals.var_lln_ni_dn5, locals.var_lln_ni_dn6,)
    }
};
        locals.var_lln_ni = assign860_e2575;
        locals.var_lln_ni_dn4 = assign860_e2575_d_n4;
        locals.var_lln_ni_dn5 = assign860_e2575_d_n5;
        locals.var_lln_ni_dn6 = assign860_e2575_d_n6;

        let assign870_e2578: f64 = (p.p16 * p.p349);
        locals.var_b4soirbodyext = assign870_e2578;

        locals.var_ldrn = p.p1;

        let assign890_e2582: f64 = (p.p2 / p.p3);
        locals.var_wdrn = assign890_e2582;

        let assign900_e2585: f64 = (locals.var_ldrn).powf(p.p190);
        locals.var_t0 = assign900_e2585;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;

        let assign910_e2588: f64 = (locals.var_wdrn).powf(p.p193);
        locals.var_t1 = assign910_e2588;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;

        let assign920_e2591: f64 = (p.p188 / locals.var_t0);
        let assign920_e2594: f64 = (p.p191 / locals.var_t1);
        let assign920_e2595: f64 = (assign920_e2591 + assign920_e2594);
        let assign920_e2599: f64 = (locals.var_t0 * locals.var_t1);
        let assign920_e2600: f64 = (p.p194 / assign920_e2599);
        let assign920_e2601: f64 = (assign920_e2595 + assign920_e2600);
        locals.var_tmp1 = assign920_e2601;
        locals.var_tmp1_dn3 = (((-((p.p188 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p191 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p194 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign920_e2599 * assign920_e2599))));
        locals.var_tmp1_dn4 = (((-((p.p188 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p191 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p194 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign920_e2599 * assign920_e2599))));
        locals.var_tmp1_dn5 = (((-((p.p188 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p191 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p194 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign920_e2599 * assign920_e2599))));
        locals.var_tmp1_dn6 = (((-((p.p188 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p191 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p194 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign920_e2599 * assign920_e2599))));
        locals.var_tmp1_dn7 = (((-((p.p188 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p191 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p194 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign920_e2599 * assign920_e2599))));
        locals.var_tmp1_dn8 = (((-((p.p188 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p191 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p194 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign920_e2599 * assign920_e2599))));
        locals.var_tmp1_dn9 = (((-((p.p188 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p191 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p194 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign920_e2599 * assign920_e2599))));
        locals.var_tmp1_dn10 = (((-((p.p188 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p191 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p194 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign920_e2599 * assign920_e2599))));
        locals.var_tmp1_dn11 = (((-((p.p188 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p191 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p194 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign920_e2599 * assign920_e2599))));
        locals.var_tmp1_dn12 = (((-((p.p188 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))) + (-((p.p191 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1)))) + (-((p.p194 * ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12))) / (assign920_e2599 * assign920_e2599))));

        let assign930_e2604: f64 = (p.p187 + locals.var_tmp1);
        locals.var_pparam_b4soidl = assign930_e2604;
        locals.var_pparam_b4soidl_dn3 = locals.var_tmp1_dn3;
        locals.var_pparam_b4soidl_dn4 = locals.var_tmp1_dn4;
        locals.var_pparam_b4soidl_dn5 = locals.var_tmp1_dn5;
        locals.var_pparam_b4soidl_dn6 = locals.var_tmp1_dn6;
        locals.var_pparam_b4soidl_dn7 = locals.var_tmp1_dn7;
        locals.var_pparam_b4soidl_dn8 = locals.var_tmp1_dn8;
        locals.var_pparam_b4soidl_dn9 = locals.var_tmp1_dn9;
        locals.var_pparam_b4soidl_dn10 = locals.var_tmp1_dn10;
        locals.var_pparam_b4soidl_dn11 = locals.var_tmp1_dn11;
        locals.var_pparam_b4soidl_dn12 = locals.var_tmp1_dn12;

        let assign940_e2607: f64 = (p.p189 / locals.var_t0);
        let assign940_e2610: f64 = (p.p192 / locals.var_t1);
        let assign940_e2611: f64 = (assign940_e2607 + assign940_e2610);
        let assign940_e2615: f64 = (locals.var_t0 * locals.var_t1);
        let assign940_e2616: f64 = (p.p195 / assign940_e2615);
        let assign940_e2617: f64 = (assign940_e2611 + assign940_e2616);
        locals.var_tmp1 = assign940_e2617;
        locals.var_tmp1_dn3 = (((-((p.p189 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p192 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p195 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign940_e2615 * assign940_e2615))));
        locals.var_tmp1_dn4 = (((-((p.p189 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p192 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p195 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign940_e2615 * assign940_e2615))));
        locals.var_tmp1_dn5 = (((-((p.p189 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p192 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p195 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign940_e2615 * assign940_e2615))));
        locals.var_tmp1_dn6 = (((-((p.p189 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p192 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p195 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign940_e2615 * assign940_e2615))));
        locals.var_tmp1_dn7 = (((-((p.p189 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p192 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p195 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign940_e2615 * assign940_e2615))));
        locals.var_tmp1_dn8 = (((-((p.p189 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p192 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p195 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign940_e2615 * assign940_e2615))));
        locals.var_tmp1_dn9 = (((-((p.p189 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p192 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p195 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign940_e2615 * assign940_e2615))));
        locals.var_tmp1_dn10 = (((-((p.p189 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p192 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p195 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign940_e2615 * assign940_e2615))));
        locals.var_tmp1_dn11 = (((-((p.p189 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p192 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p195 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign940_e2615 * assign940_e2615))));
        locals.var_tmp1_dn12 = (((-((p.p189 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))) + (-((p.p192 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1)))) + (-((p.p195 * ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12))) / (assign940_e2615 * assign940_e2615))));

        let assign950_e2620: f64 = (p.p217 + locals.var_tmp1);
        locals.var_pparam_b4soidlc = assign950_e2620;
        locals.var_pparam_b4soidlc_dn3 = locals.var_tmp1_dn3;
        locals.var_pparam_b4soidlc_dn4 = locals.var_tmp1_dn4;
        locals.var_pparam_b4soidlc_dn5 = locals.var_tmp1_dn5;
        locals.var_pparam_b4soidlc_dn6 = locals.var_tmp1_dn6;
        locals.var_pparam_b4soidlc_dn7 = locals.var_tmp1_dn7;
        locals.var_pparam_b4soidlc_dn8 = locals.var_tmp1_dn8;
        locals.var_pparam_b4soidlc_dn9 = locals.var_tmp1_dn9;
        locals.var_pparam_b4soidlc_dn10 = locals.var_tmp1_dn10;
        locals.var_pparam_b4soidlc_dn11 = locals.var_tmp1_dn11;
        locals.var_pparam_b4soidlc_dn12 = locals.var_tmp1_dn12;

        let assign960_e2623: f64 = (p.p410 + locals.var_tmp1);
        locals.var_pparam_b4soidlcig = assign960_e2623;
        locals.var_pparam_b4soidlcig_dn3 = locals.var_tmp1_dn3;
        locals.var_pparam_b4soidlcig_dn4 = locals.var_tmp1_dn4;
        locals.var_pparam_b4soidlcig_dn5 = locals.var_tmp1_dn5;
        locals.var_pparam_b4soidlcig_dn6 = locals.var_tmp1_dn6;
        locals.var_pparam_b4soidlcig_dn7 = locals.var_tmp1_dn7;
        locals.var_pparam_b4soidlcig_dn8 = locals.var_tmp1_dn8;
        locals.var_pparam_b4soidlcig_dn9 = locals.var_tmp1_dn9;
        locals.var_pparam_b4soidlcig_dn10 = locals.var_tmp1_dn10;
        locals.var_pparam_b4soidlcig_dn11 = locals.var_tmp1_dn11;
        locals.var_pparam_b4soidlcig_dn12 = locals.var_tmp1_dn12;

        let assign970_e2626: f64 = if locals.var_pparam_b4soidlcig < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard501 = assign970_e2626;

        let (assign980_e2630, assign980_e2630_d_n3, assign980_e2630_d_n4, assign980_e2630_d_n5, assign980_e2630_d_n6, assign980_e2630_d_n7, assign980_e2630_d_n8, assign980_e2630_d_n9, assign980_e2630_d_n10, assign980_e2630_d_n11, assign980_e2630_d_n12,) = {
    if (locals.var_guard501 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soidlcig, locals.var_pparam_b4soidlcig_dn3, locals.var_pparam_b4soidlcig_dn4, locals.var_pparam_b4soidlcig_dn5, locals.var_pparam_b4soidlcig_dn6, locals.var_pparam_b4soidlcig_dn7, locals.var_pparam_b4soidlcig_dn8, locals.var_pparam_b4soidlcig_dn9, locals.var_pparam_b4soidlcig_dn10, locals.var_pparam_b4soidlcig_dn11, locals.var_pparam_b4soidlcig_dn12,)
    }
};
        locals.var_pparam_b4soidlcig = assign980_e2630;
        locals.var_pparam_b4soidlcig_dn3 = assign980_e2630_d_n3;
        locals.var_pparam_b4soidlcig_dn4 = assign980_e2630_d_n4;
        locals.var_pparam_b4soidlcig_dn5 = assign980_e2630_d_n5;
        locals.var_pparam_b4soidlcig_dn6 = assign980_e2630_d_n6;
        locals.var_pparam_b4soidlcig_dn7 = assign980_e2630_d_n7;
        locals.var_pparam_b4soidlcig_dn8 = assign980_e2630_d_n8;
        locals.var_pparam_b4soidlcig_dn9 = assign980_e2630_d_n9;
        locals.var_pparam_b4soidlcig_dn10 = assign980_e2630_d_n10;
        locals.var_pparam_b4soidlcig_dn11 = assign980_e2630_d_n11;
        locals.var_pparam_b4soidlcig_dn12 = assign980_e2630_d_n12;

        let assign990_e2633: f64 = (locals.var_ldrn).powf(p.p202);
        locals.var_t2 = assign990_e2633;
        locals.var_t2_dn3 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn12 = 0.0;

        let assign1000_e2636: f64 = (locals.var_wdrn).powf(p.p205);
        locals.var_t3 = assign1000_e2636;
        locals.var_t3_dn3 = 0.0;
        locals.var_t3_dn4 = 0.0;
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn9 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn11 = 0.0;
        locals.var_t3_dn12 = 0.0;

        let assign1010_e2639: f64 = (p.p200 / locals.var_t2);
        let assign1010_e2642: f64 = (p.p203 / locals.var_t3);
        let assign1010_e2643: f64 = (assign1010_e2639 + assign1010_e2642);
        let assign1010_e2647: f64 = (locals.var_t2 * locals.var_t3);
        let assign1010_e2648: f64 = (p.p206 / assign1010_e2647);
        let assign1010_e2649: f64 = (assign1010_e2643 + assign1010_e2648);
        locals.var_tmp2 = assign1010_e2649;
        locals.var_tmp2_dn3 = (((-((p.p200 * locals.var_t2_dn3) / (locals.var_t2 * locals.var_t2))) + (-((p.p203 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3)))) + (-((p.p206 * ((locals.var_t2_dn3 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn3))) / (assign1010_e2647 * assign1010_e2647))));
        locals.var_tmp2_dn4 = (((-((p.p200 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))) + (-((p.p203 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))) + (-((p.p206 * ((locals.var_t2_dn4 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn4))) / (assign1010_e2647 * assign1010_e2647))));
        locals.var_tmp2_dn5 = (((-((p.p200 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))) + (-((p.p203 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))) + (-((p.p206 * ((locals.var_t2_dn5 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn5))) / (assign1010_e2647 * assign1010_e2647))));
        locals.var_tmp2_dn6 = (((-((p.p200 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))) + (-((p.p203 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))) + (-((p.p206 * ((locals.var_t2_dn6 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn6))) / (assign1010_e2647 * assign1010_e2647))));
        locals.var_tmp2_dn7 = (((-((p.p200 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))) + (-((p.p203 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))) + (-((p.p206 * ((locals.var_t2_dn7 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn7))) / (assign1010_e2647 * assign1010_e2647))));
        locals.var_tmp2_dn8 = (((-((p.p200 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))) + (-((p.p203 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))) + (-((p.p206 * ((locals.var_t2_dn8 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn8))) / (assign1010_e2647 * assign1010_e2647))));
        locals.var_tmp2_dn9 = (((-((p.p200 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))) + (-((p.p203 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))) + (-((p.p206 * ((locals.var_t2_dn9 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn9))) / (assign1010_e2647 * assign1010_e2647))));
        locals.var_tmp2_dn10 = (((-((p.p200 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))) + (-((p.p203 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))) + (-((p.p206 * ((locals.var_t2_dn10 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn10))) / (assign1010_e2647 * assign1010_e2647))));
        locals.var_tmp2_dn11 = (((-((p.p200 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))) + (-((p.p203 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))) + (-((p.p206 * ((locals.var_t2_dn11 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn11))) / (assign1010_e2647 * assign1010_e2647))));
        locals.var_tmp2_dn12 = (((-((p.p200 * locals.var_t2_dn12) / (locals.var_t2 * locals.var_t2))) + (-((p.p203 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3)))) + (-((p.p206 * ((locals.var_t2_dn12 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn12))) / (assign1010_e2647 * assign1010_e2647))));

        let assign1020_e2652: f64 = (p.p197 + locals.var_tmp2);
        locals.var_pparam_b4soidw = assign1020_e2652;
        locals.var_pparam_b4soidw_dn3 = locals.var_tmp2_dn3;
        locals.var_pparam_b4soidw_dn4 = locals.var_tmp2_dn4;
        locals.var_pparam_b4soidw_dn5 = locals.var_tmp2_dn5;
        locals.var_pparam_b4soidw_dn6 = locals.var_tmp2_dn6;
        locals.var_pparam_b4soidw_dn7 = locals.var_tmp2_dn7;
        locals.var_pparam_b4soidw_dn8 = locals.var_tmp2_dn8;
        locals.var_pparam_b4soidw_dn9 = locals.var_tmp2_dn9;
        locals.var_pparam_b4soidw_dn10 = locals.var_tmp2_dn10;
        locals.var_pparam_b4soidw_dn11 = locals.var_tmp2_dn11;
        locals.var_pparam_b4soidw_dn12 = locals.var_tmp2_dn12;

        let assign1030_e2655: f64 = (p.p201 / locals.var_t2);
        let assign1030_e2658: f64 = (p.p204 / locals.var_t3);
        let assign1030_e2659: f64 = (assign1030_e2655 + assign1030_e2658);
        let assign1030_e2663: f64 = (locals.var_t2 * locals.var_t3);
        let assign1030_e2664: f64 = (p.p207 / assign1030_e2663);
        let assign1030_e2665: f64 = (assign1030_e2659 + assign1030_e2664);
        locals.var_tmp2 = assign1030_e2665;
        locals.var_tmp2_dn3 = (((-((p.p201 * locals.var_t2_dn3) / (locals.var_t2 * locals.var_t2))) + (-((p.p204 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3)))) + (-((p.p207 * ((locals.var_t2_dn3 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn3))) / (assign1030_e2663 * assign1030_e2663))));
        locals.var_tmp2_dn4 = (((-((p.p201 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))) + (-((p.p204 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))) + (-((p.p207 * ((locals.var_t2_dn4 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn4))) / (assign1030_e2663 * assign1030_e2663))));
        locals.var_tmp2_dn5 = (((-((p.p201 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))) + (-((p.p204 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))) + (-((p.p207 * ((locals.var_t2_dn5 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn5))) / (assign1030_e2663 * assign1030_e2663))));
        locals.var_tmp2_dn6 = (((-((p.p201 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))) + (-((p.p204 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))) + (-((p.p207 * ((locals.var_t2_dn6 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn6))) / (assign1030_e2663 * assign1030_e2663))));
        locals.var_tmp2_dn7 = (((-((p.p201 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))) + (-((p.p204 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))) + (-((p.p207 * ((locals.var_t2_dn7 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn7))) / (assign1030_e2663 * assign1030_e2663))));
        locals.var_tmp2_dn8 = (((-((p.p201 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))) + (-((p.p204 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))) + (-((p.p207 * ((locals.var_t2_dn8 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn8))) / (assign1030_e2663 * assign1030_e2663))));
        locals.var_tmp2_dn9 = (((-((p.p201 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))) + (-((p.p204 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))) + (-((p.p207 * ((locals.var_t2_dn9 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn9))) / (assign1030_e2663 * assign1030_e2663))));
        locals.var_tmp2_dn10 = (((-((p.p201 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))) + (-((p.p204 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))) + (-((p.p207 * ((locals.var_t2_dn10 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn10))) / (assign1030_e2663 * assign1030_e2663))));
        locals.var_tmp2_dn11 = (((-((p.p201 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))) + (-((p.p204 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))) + (-((p.p207 * ((locals.var_t2_dn11 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn11))) / (assign1030_e2663 * assign1030_e2663))));
        locals.var_tmp2_dn12 = (((-((p.p201 * locals.var_t2_dn12) / (locals.var_t2 * locals.var_t2))) + (-((p.p204 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3)))) + (-((p.p207 * ((locals.var_t2_dn12 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn12))) / (assign1030_e2663 * assign1030_e2663))));

        let assign1040_e2668: f64 = (p.p216 + locals.var_tmp2);
        locals.var_pparam_b4soidwc = assign1040_e2668;
        locals.var_pparam_b4soidwc_dn3 = locals.var_tmp2_dn3;
        locals.var_pparam_b4soidwc_dn4 = locals.var_tmp2_dn4;
        locals.var_pparam_b4soidwc_dn5 = locals.var_tmp2_dn5;
        locals.var_pparam_b4soidwc_dn6 = locals.var_tmp2_dn6;
        locals.var_pparam_b4soidwc_dn7 = locals.var_tmp2_dn7;
        locals.var_pparam_b4soidwc_dn8 = locals.var_tmp2_dn8;
        locals.var_pparam_b4soidwc_dn9 = locals.var_tmp2_dn9;
        locals.var_pparam_b4soidwc_dn10 = locals.var_tmp2_dn10;
        locals.var_pparam_b4soidwc_dn11 = locals.var_tmp2_dn11;
        locals.var_pparam_b4soidwc_dn12 = locals.var_tmp2_dn12;

        let assign1050_e2672: f64 = (2.0 * locals.var_pparam_b4soidl);
        let assign1050_e2673: f64 = (p.p1 - assign1050_e2672);
        locals.var_pparam_b4soileff = assign1050_e2673;
        locals.var_pparam_b4soileff_dn3 = (-(2.0 * locals.var_pparam_b4soidl_dn3));
        locals.var_pparam_b4soileff_dn4 = (-(2.0 * locals.var_pparam_b4soidl_dn4));
        locals.var_pparam_b4soileff_dn5 = (-(2.0 * locals.var_pparam_b4soidl_dn5));
        locals.var_pparam_b4soileff_dn6 = (-(2.0 * locals.var_pparam_b4soidl_dn6));
        locals.var_pparam_b4soileff_dn7 = (-(2.0 * locals.var_pparam_b4soidl_dn7));
        locals.var_pparam_b4soileff_dn8 = (-(2.0 * locals.var_pparam_b4soidl_dn8));
        locals.var_pparam_b4soileff_dn9 = (-(2.0 * locals.var_pparam_b4soidl_dn9));
        locals.var_pparam_b4soileff_dn10 = (-(2.0 * locals.var_pparam_b4soidl_dn10));
        locals.var_pparam_b4soileff_dn11 = (-(2.0 * locals.var_pparam_b4soidl_dn11));
        locals.var_pparam_b4soileff_dn12 = (-(2.0 * locals.var_pparam_b4soidl_dn12));

        let assign1070_e2679: f64 = (p.p2 / p.p3);
        let assign1070_e2682: f64 = (p.p22 * p.p303);
        let assign1070_e2683: f64 = (assign1070_e2679 - assign1070_e2682);
        let assign1070_e2686: f64 = (2.0 - p.p22);
        let assign1070_e2688: f64 = (assign1070_e2686 * locals.var_pparam_b4soidw);
        let assign1070_e2689: f64 = (assign1070_e2683 - assign1070_e2688);
        locals.var_pparam_b4soiweff = assign1070_e2689;
        locals.var_pparam_b4soiweff_dn3 = (-(assign1070_e2686 * locals.var_pparam_b4soidw_dn3));
        locals.var_pparam_b4soiweff_dn4 = (-(assign1070_e2686 * locals.var_pparam_b4soidw_dn4));
        locals.var_pparam_b4soiweff_dn5 = (-(assign1070_e2686 * locals.var_pparam_b4soidw_dn5));
        locals.var_pparam_b4soiweff_dn6 = (-(assign1070_e2686 * locals.var_pparam_b4soidw_dn6));
        locals.var_pparam_b4soiweff_dn7 = (-(assign1070_e2686 * locals.var_pparam_b4soidw_dn7));
        locals.var_pparam_b4soiweff_dn8 = (-(assign1070_e2686 * locals.var_pparam_b4soidw_dn8));
        locals.var_pparam_b4soiweff_dn9 = (-(assign1070_e2686 * locals.var_pparam_b4soidw_dn9));
        locals.var_pparam_b4soiweff_dn10 = (-(assign1070_e2686 * locals.var_pparam_b4soidw_dn10));
        locals.var_pparam_b4soiweff_dn11 = (-(assign1070_e2686 * locals.var_pparam_b4soidw_dn11));
        locals.var_pparam_b4soiweff_dn12 = (-(assign1070_e2686 * locals.var_pparam_b4soidw_dn12));

        let assign1090_e2695: f64 = (locals.var_pparam_b4soiweff / p.p23);
        let assign1090_e2697: f64 = (assign1090_e2695 + p.p24);
        locals.var_pparam_b4soiwdiod = assign1090_e2697;
        locals.var_pparam_b4soiwdiod_dn3 = (locals.var_pparam_b4soiweff_dn3 / p.p23);
        locals.var_pparam_b4soiwdiod_dn4 = (locals.var_pparam_b4soiweff_dn4 / p.p23);
        locals.var_pparam_b4soiwdiod_dn5 = (locals.var_pparam_b4soiweff_dn5 / p.p23);
        locals.var_pparam_b4soiwdiod_dn6 = (locals.var_pparam_b4soiweff_dn6 / p.p23);
        locals.var_pparam_b4soiwdiod_dn7 = (locals.var_pparam_b4soiweff_dn7 / p.p23);
        locals.var_pparam_b4soiwdiod_dn8 = (locals.var_pparam_b4soiweff_dn8 / p.p23);
        locals.var_pparam_b4soiwdiod_dn9 = (locals.var_pparam_b4soiweff_dn9 / p.p23);
        locals.var_pparam_b4soiwdiod_dn10 = (locals.var_pparam_b4soiweff_dn10 / p.p23);
        locals.var_pparam_b4soiwdiod_dn11 = (locals.var_pparam_b4soiweff_dn11 / p.p23);
        locals.var_pparam_b4soiwdiod_dn12 = (locals.var_pparam_b4soiweff_dn12 / p.p23);

        let assign1100_e2700: f64 = (locals.var_pparam_b4soiweff / p.p23);
        let assign1100_e2702: f64 = (assign1100_e2700 + p.p25);
        locals.var_pparam_b4soiwdios = assign1100_e2702;
        locals.var_pparam_b4soiwdios_dn3 = (locals.var_pparam_b4soiweff_dn3 / p.p23);
        locals.var_pparam_b4soiwdios_dn4 = (locals.var_pparam_b4soiweff_dn4 / p.p23);
        locals.var_pparam_b4soiwdios_dn5 = (locals.var_pparam_b4soiweff_dn5 / p.p23);
        locals.var_pparam_b4soiwdios_dn6 = (locals.var_pparam_b4soiweff_dn6 / p.p23);
        locals.var_pparam_b4soiwdios_dn7 = (locals.var_pparam_b4soiweff_dn7 / p.p23);
        locals.var_pparam_b4soiwdios_dn8 = (locals.var_pparam_b4soiweff_dn8 / p.p23);
        locals.var_pparam_b4soiwdios_dn9 = (locals.var_pparam_b4soiweff_dn9 / p.p23);
        locals.var_pparam_b4soiwdios_dn10 = (locals.var_pparam_b4soiweff_dn10 / p.p23);
        locals.var_pparam_b4soiwdios_dn11 = (locals.var_pparam_b4soiweff_dn11 / p.p23);
        locals.var_pparam_b4soiwdios_dn12 = (locals.var_pparam_b4soiweff_dn12 / p.p23);

        let assign1110_e2706: f64 = (2.0 * locals.var_pparam_b4soidlc);
        let assign1110_e2707: f64 = (p.p1 - assign1110_e2706);
        locals.var_pparam_b4soileffcv = assign1110_e2707;
        locals.var_pparam_b4soileffcv_dn3 = (-(2.0 * locals.var_pparam_b4soidlc_dn3));
        locals.var_pparam_b4soileffcv_dn4 = (-(2.0 * locals.var_pparam_b4soidlc_dn4));
        locals.var_pparam_b4soileffcv_dn5 = (-(2.0 * locals.var_pparam_b4soidlc_dn5));
        locals.var_pparam_b4soileffcv_dn6 = (-(2.0 * locals.var_pparam_b4soidlc_dn6));
        locals.var_pparam_b4soileffcv_dn7 = (-(2.0 * locals.var_pparam_b4soidlc_dn7));
        locals.var_pparam_b4soileffcv_dn8 = (-(2.0 * locals.var_pparam_b4soidlc_dn8));
        locals.var_pparam_b4soileffcv_dn9 = (-(2.0 * locals.var_pparam_b4soidlc_dn9));
        locals.var_pparam_b4soileffcv_dn10 = (-(2.0 * locals.var_pparam_b4soidlc_dn10));
        locals.var_pparam_b4soileffcv_dn11 = (-(2.0 * locals.var_pparam_b4soidlc_dn11));
        locals.var_pparam_b4soileffcv_dn12 = (-(2.0 * locals.var_pparam_b4soidlc_dn12));

        let assign1130_e2713: f64 = (p.p2 / p.p3);
        let assign1130_e2716: f64 = (p.p22 * p.p303);
        let assign1130_e2717: f64 = (assign1130_e2713 - assign1130_e2716);
        let assign1130_e2720: f64 = (2.0 - p.p22);
        let assign1130_e2722: f64 = (assign1130_e2720 * locals.var_pparam_b4soidwc);
        let assign1130_e2723: f64 = (assign1130_e2717 - assign1130_e2722);
        locals.var_pparam_b4soiweffcv = assign1130_e2723;
        locals.var_pparam_b4soiweffcv_dn3 = (-(assign1130_e2720 * locals.var_pparam_b4soidwc_dn3));
        locals.var_pparam_b4soiweffcv_dn4 = (-(assign1130_e2720 * locals.var_pparam_b4soidwc_dn4));
        locals.var_pparam_b4soiweffcv_dn5 = (-(assign1130_e2720 * locals.var_pparam_b4soidwc_dn5));
        locals.var_pparam_b4soiweffcv_dn6 = (-(assign1130_e2720 * locals.var_pparam_b4soidwc_dn6));
        locals.var_pparam_b4soiweffcv_dn7 = (-(assign1130_e2720 * locals.var_pparam_b4soidwc_dn7));
        locals.var_pparam_b4soiweffcv_dn8 = (-(assign1130_e2720 * locals.var_pparam_b4soidwc_dn8));
        locals.var_pparam_b4soiweffcv_dn9 = (-(assign1130_e2720 * locals.var_pparam_b4soidwc_dn9));
        locals.var_pparam_b4soiweffcv_dn10 = (-(assign1130_e2720 * locals.var_pparam_b4soidwc_dn10));
        locals.var_pparam_b4soiweffcv_dn11 = (-(assign1130_e2720 * locals.var_pparam_b4soidwc_dn11));
        locals.var_pparam_b4soiweffcv_dn12 = (-(assign1130_e2720 * locals.var_pparam_b4soidwc_dn12));

        let assign1150_e2729: f64 = (locals.var_pparam_b4soiweffcv / p.p23);
        let assign1150_e2731: f64 = (assign1150_e2729 + p.p24);
        locals.var_pparam_b4soiwdiodcv = assign1150_e2731;
        locals.var_pparam_b4soiwdiodcv_dn3 = (locals.var_pparam_b4soiweffcv_dn3 / p.p23);
        locals.var_pparam_b4soiwdiodcv_dn4 = (locals.var_pparam_b4soiweffcv_dn4 / p.p23);
        locals.var_pparam_b4soiwdiodcv_dn5 = (locals.var_pparam_b4soiweffcv_dn5 / p.p23);
        locals.var_pparam_b4soiwdiodcv_dn6 = (locals.var_pparam_b4soiweffcv_dn6 / p.p23);
        locals.var_pparam_b4soiwdiodcv_dn7 = (locals.var_pparam_b4soiweffcv_dn7 / p.p23);
        locals.var_pparam_b4soiwdiodcv_dn8 = (locals.var_pparam_b4soiweffcv_dn8 / p.p23);
        locals.var_pparam_b4soiwdiodcv_dn9 = (locals.var_pparam_b4soiweffcv_dn9 / p.p23);
        locals.var_pparam_b4soiwdiodcv_dn10 = (locals.var_pparam_b4soiweffcv_dn10 / p.p23);
        locals.var_pparam_b4soiwdiodcv_dn11 = (locals.var_pparam_b4soiweffcv_dn11 / p.p23);
        locals.var_pparam_b4soiwdiodcv_dn12 = (locals.var_pparam_b4soiweffcv_dn12 / p.p23);

        let assign1160_e2734: f64 = (locals.var_pparam_b4soiweffcv / p.p23);
        let assign1160_e2736: f64 = (assign1160_e2734 + p.p25);
        locals.var_pparam_b4soiwdioscv = assign1160_e2736;
        locals.var_pparam_b4soiwdioscv_dn3 = (locals.var_pparam_b4soiweffcv_dn3 / p.p23);
        locals.var_pparam_b4soiwdioscv_dn4 = (locals.var_pparam_b4soiweffcv_dn4 / p.p23);
        locals.var_pparam_b4soiwdioscv_dn5 = (locals.var_pparam_b4soiweffcv_dn5 / p.p23);
        locals.var_pparam_b4soiwdioscv_dn6 = (locals.var_pparam_b4soiweffcv_dn6 / p.p23);
        locals.var_pparam_b4soiwdioscv_dn7 = (locals.var_pparam_b4soiweffcv_dn7 / p.p23);
        locals.var_pparam_b4soiwdioscv_dn8 = (locals.var_pparam_b4soiweffcv_dn8 / p.p23);
        locals.var_pparam_b4soiwdioscv_dn9 = (locals.var_pparam_b4soiweffcv_dn9 / p.p23);
        locals.var_pparam_b4soiwdioscv_dn10 = (locals.var_pparam_b4soiweffcv_dn10 / p.p23);
        locals.var_pparam_b4soiwdioscv_dn11 = (locals.var_pparam_b4soiweffcv_dn11 / p.p23);
        locals.var_pparam_b4soiwdioscv_dn12 = (locals.var_pparam_b4soiweffcv_dn12 / p.p23);

        let assign1170_e2740: f64 = (2.0 * locals.var_pparam_b4soidlc);
        let assign1170_e2741: f64 = (p.p1 - assign1170_e2740);
        let assign1170_e2743: f64 = (assign1170_e2741 - p.p360);
        locals.var_pparam_b4soileffcvb = assign1170_e2743;
        locals.var_pparam_b4soileffcvb_dn3 = (-(2.0 * locals.var_pparam_b4soidlc_dn3));
        locals.var_pparam_b4soileffcvb_dn4 = (-(2.0 * locals.var_pparam_b4soidlc_dn4));
        locals.var_pparam_b4soileffcvb_dn5 = (-(2.0 * locals.var_pparam_b4soidlc_dn5));
        locals.var_pparam_b4soileffcvb_dn6 = (-(2.0 * locals.var_pparam_b4soidlc_dn6));
        locals.var_pparam_b4soileffcvb_dn7 = (-(2.0 * locals.var_pparam_b4soidlc_dn7));
        locals.var_pparam_b4soileffcvb_dn8 = (-(2.0 * locals.var_pparam_b4soidlc_dn8));
        locals.var_pparam_b4soileffcvb_dn9 = (-(2.0 * locals.var_pparam_b4soidlc_dn9));
        locals.var_pparam_b4soileffcvb_dn10 = (-(2.0 * locals.var_pparam_b4soidlc_dn10));
        locals.var_pparam_b4soileffcvb_dn11 = (-(2.0 * locals.var_pparam_b4soidlc_dn11));
        locals.var_pparam_b4soileffcvb_dn12 = (-(2.0 * locals.var_pparam_b4soidlc_dn12));

        let assign1190_e2750: f64 = (2.0 * p.p372);
        let assign1190_e2751: f64 = (locals.var_pparam_b4soileffcvb + assign1190_e2750);
        locals.var_pparam_b4soileffcvbg = assign1190_e2751;
        locals.var_pparam_b4soileffcvbg_dn3 = locals.var_pparam_b4soileffcvb_dn3;
        locals.var_pparam_b4soileffcvbg_dn4 = locals.var_pparam_b4soileffcvb_dn4;
        locals.var_pparam_b4soileffcvbg_dn5 = locals.var_pparam_b4soileffcvb_dn5;
        locals.var_pparam_b4soileffcvbg_dn6 = locals.var_pparam_b4soileffcvb_dn6;
        locals.var_pparam_b4soileffcvbg_dn7 = locals.var_pparam_b4soileffcvb_dn7;
        locals.var_pparam_b4soileffcvbg_dn8 = locals.var_pparam_b4soileffcvb_dn8;
        locals.var_pparam_b4soileffcvbg_dn9 = locals.var_pparam_b4soileffcvb_dn9;
        locals.var_pparam_b4soileffcvbg_dn10 = locals.var_pparam_b4soileffcvb_dn10;
        locals.var_pparam_b4soileffcvbg_dn11 = locals.var_pparam_b4soileffcvb_dn11;
        locals.var_pparam_b4soileffcvbg_dn12 = locals.var_pparam_b4soileffcvb_dn12;

        locals.var_pparam_b4soigamma1 = p.p85;
        locals.var_pparam_b4soigamma1_dn3 = 0.0;
        locals.var_pparam_b4soigamma1_dn4 = 0.0;
        locals.var_pparam_b4soigamma1_dn5 = 0.0;
        locals.var_pparam_b4soigamma1_dn6 = 0.0;
        locals.var_pparam_b4soigamma1_dn7 = 0.0;
        locals.var_pparam_b4soigamma1_dn8 = 0.0;
        locals.var_pparam_b4soigamma1_dn9 = 0.0;
        locals.var_pparam_b4soigamma1_dn10 = 0.0;
        locals.var_pparam_b4soigamma1_dn11 = 0.0;
        locals.var_pparam_b4soigamma1_dn12 = 0.0;

        locals.var_pparam_b4soigamma2 = p.p86;
        locals.var_pparam_b4soigamma2_dn3 = 0.0;
        locals.var_pparam_b4soigamma2_dn4 = 0.0;
        locals.var_pparam_b4soigamma2_dn5 = 0.0;
        locals.var_pparam_b4soigamma2_dn6 = 0.0;
        locals.var_pparam_b4soigamma2_dn7 = 0.0;
        locals.var_pparam_b4soigamma2_dn8 = 0.0;
        locals.var_pparam_b4soigamma2_dn9 = 0.0;
        locals.var_pparam_b4soigamma2_dn10 = 0.0;
        locals.var_pparam_b4soigamma2_dn11 = 0.0;
        locals.var_pparam_b4soigamma2_dn12 = 0.0;

        locals.var_pparam_b4soivbx = p.p87;
        locals.var_pparam_b4soivbx_dn3 = 0.0;
        locals.var_pparam_b4soivbx_dn4 = 0.0;
        locals.var_pparam_b4soivbx_dn5 = 0.0;
        locals.var_pparam_b4soivbx_dn6 = 0.0;
        locals.var_pparam_b4soivbx_dn7 = 0.0;
        locals.var_pparam_b4soivbx_dn8 = 0.0;
        locals.var_pparam_b4soivbx_dn9 = 0.0;
        locals.var_pparam_b4soivbx_dn10 = 0.0;
        locals.var_pparam_b4soivbx_dn11 = 0.0;
        locals.var_pparam_b4soivbx_dn12 = 0.0;

        locals.var_pparam_b4soivbm = p.p88;

        locals.var_pparam_b4soixt = p.p89;

        locals.var_pparam_b4soicf = locals.var_b4soicf;

        locals.var_pparam_b4soiclc = p.p214;

        locals.var_pparam_b4soicle = p.p215;

        let assign1290_e2765: f64 = if locals.var_pparam_b4soicle == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard508 = assign1290_e2765;

        let (assign1300_e2769, assign1300_e2769_d_n3, assign1300_e2769_d_n4, assign1300_e2769_d_n5, assign1300_e2769_d_n6, assign1300_e2769_d_n7, assign1300_e2769_d_n8, assign1300_e2769_d_n9, assign1300_e2769_d_n10, assign1300_e2769_d_n11, assign1300_e2769_d_n12,) = {
    if (locals.var_guard508 != 0.0) {
        (2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soiabulkcvfactor, locals.var_pparam_b4soiabulkcvfactor_dn3, locals.var_pparam_b4soiabulkcvfactor_dn4, locals.var_pparam_b4soiabulkcvfactor_dn5, locals.var_pparam_b4soiabulkcvfactor_dn6, locals.var_pparam_b4soiabulkcvfactor_dn7, locals.var_pparam_b4soiabulkcvfactor_dn8, locals.var_pparam_b4soiabulkcvfactor_dn9, locals.var_pparam_b4soiabulkcvfactor_dn10, locals.var_pparam_b4soiabulkcvfactor_dn11, locals.var_pparam_b4soiabulkcvfactor_dn12,)
    }
};
        locals.var_pparam_b4soiabulkcvfactor = assign1300_e2769;
        locals.var_pparam_b4soiabulkcvfactor_dn3 = assign1300_e2769_d_n3;
        locals.var_pparam_b4soiabulkcvfactor_dn4 = assign1300_e2769_d_n4;
        locals.var_pparam_b4soiabulkcvfactor_dn5 = assign1300_e2769_d_n5;
        locals.var_pparam_b4soiabulkcvfactor_dn6 = assign1300_e2769_d_n6;
        locals.var_pparam_b4soiabulkcvfactor_dn7 = assign1300_e2769_d_n7;
        locals.var_pparam_b4soiabulkcvfactor_dn8 = assign1300_e2769_d_n8;
        locals.var_pparam_b4soiabulkcvfactor_dn9 = assign1300_e2769_d_n9;
        locals.var_pparam_b4soiabulkcvfactor_dn10 = assign1300_e2769_d_n10;
        locals.var_pparam_b4soiabulkcvfactor_dn11 = assign1300_e2769_d_n11;
        locals.var_pparam_b4soiabulkcvfactor_dn12 = assign1300_e2769_d_n12;

        let (assign1310_e2780, assign1310_e2780_d_n3, assign1310_e2780_d_n4, assign1310_e2780_d_n5, assign1310_e2780_d_n6, assign1310_e2780_d_n7, assign1310_e2780_d_n8, assign1310_e2780_d_n9, assign1310_e2780_d_n10, assign1310_e2780_d_n11, assign1310_e2780_d_n12,) = {
    if (locals.var_guard508 == 0.0) {
        let assign1310_e2775: f64 = (locals.var_pparam_b4soiclc / locals.var_pparam_b4soileff);
        let assign1310_e2777: f64 = (assign1310_e2775).powf(locals.var_pparam_b4soicle);
        let assign1310_e2778: f64 = (1.0 + assign1310_e2777);
        (assign1310_e2778, if 0.0 == 0.0 && ((locals.var_pparam_b4soicle) as f64).is_finite() && ((locals.var_pparam_b4soicle) as f64).fract() == 0.0 { if locals.var_pparam_b4soicle == 0.0 { 0.0 } else { (locals.var_pparam_b4soicle * ((assign1310_e2775).powf(locals.var_pparam_b4soicle - 1.0) * (-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn3) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))))) } } else { (assign1310_e2777 * (locals.var_pparam_b4soicle * ((-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn3) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) / assign1310_e2775))) }, if 0.0 == 0.0 && ((locals.var_pparam_b4soicle) as f64).is_finite() && ((locals.var_pparam_b4soicle) as f64).fract() == 0.0 { if locals.var_pparam_b4soicle == 0.0 { 0.0 } else { (locals.var_pparam_b4soicle * ((assign1310_e2775).powf(locals.var_pparam_b4soicle - 1.0) * (-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn4) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))))) } } else { (assign1310_e2777 * (locals.var_pparam_b4soicle * ((-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn4) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) / assign1310_e2775))) }, if 0.0 == 0.0 && ((locals.var_pparam_b4soicle) as f64).is_finite() && ((locals.var_pparam_b4soicle) as f64).fract() == 0.0 { if locals.var_pparam_b4soicle == 0.0 { 0.0 } else { (locals.var_pparam_b4soicle * ((assign1310_e2775).powf(locals.var_pparam_b4soicle - 1.0) * (-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn5) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))))) } } else { (assign1310_e2777 * (locals.var_pparam_b4soicle * ((-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn5) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) / assign1310_e2775))) }, if 0.0 == 0.0 && ((locals.var_pparam_b4soicle) as f64).is_finite() && ((locals.var_pparam_b4soicle) as f64).fract() == 0.0 { if locals.var_pparam_b4soicle == 0.0 { 0.0 } else { (locals.var_pparam_b4soicle * ((assign1310_e2775).powf(locals.var_pparam_b4soicle - 1.0) * (-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn6) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))))) } } else { (assign1310_e2777 * (locals.var_pparam_b4soicle * ((-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn6) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) / assign1310_e2775))) }, if 0.0 == 0.0 && ((locals.var_pparam_b4soicle) as f64).is_finite() && ((locals.var_pparam_b4soicle) as f64).fract() == 0.0 { if locals.var_pparam_b4soicle == 0.0 { 0.0 } else { (locals.var_pparam_b4soicle * ((assign1310_e2775).powf(locals.var_pparam_b4soicle - 1.0) * (-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn7) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))))) } } else { (assign1310_e2777 * (locals.var_pparam_b4soicle * ((-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn7) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) / assign1310_e2775))) }, if 0.0 == 0.0 && ((locals.var_pparam_b4soicle) as f64).is_finite() && ((locals.var_pparam_b4soicle) as f64).fract() == 0.0 { if locals.var_pparam_b4soicle == 0.0 { 0.0 } else { (locals.var_pparam_b4soicle * ((assign1310_e2775).powf(locals.var_pparam_b4soicle - 1.0) * (-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn8) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))))) } } else { (assign1310_e2777 * (locals.var_pparam_b4soicle * ((-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn8) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) / assign1310_e2775))) }, if 0.0 == 0.0 && ((locals.var_pparam_b4soicle) as f64).is_finite() && ((locals.var_pparam_b4soicle) as f64).fract() == 0.0 { if locals.var_pparam_b4soicle == 0.0 { 0.0 } else { (locals.var_pparam_b4soicle * ((assign1310_e2775).powf(locals.var_pparam_b4soicle - 1.0) * (-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn9) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))))) } } else { (assign1310_e2777 * (locals.var_pparam_b4soicle * ((-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn9) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) / assign1310_e2775))) }, if 0.0 == 0.0 && ((locals.var_pparam_b4soicle) as f64).is_finite() && ((locals.var_pparam_b4soicle) as f64).fract() == 0.0 { if locals.var_pparam_b4soicle == 0.0 { 0.0 } else { (locals.var_pparam_b4soicle * ((assign1310_e2775).powf(locals.var_pparam_b4soicle - 1.0) * (-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn10) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))))) } } else { (assign1310_e2777 * (locals.var_pparam_b4soicle * ((-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn10) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) / assign1310_e2775))) }, if 0.0 == 0.0 && ((locals.var_pparam_b4soicle) as f64).is_finite() && ((locals.var_pparam_b4soicle) as f64).fract() == 0.0 { if locals.var_pparam_b4soicle == 0.0 { 0.0 } else { (locals.var_pparam_b4soicle * ((assign1310_e2775).powf(locals.var_pparam_b4soicle - 1.0) * (-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn11) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))))) } } else { (assign1310_e2777 * (locals.var_pparam_b4soicle * ((-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn11) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) / assign1310_e2775))) }, if 0.0 == 0.0 && ((locals.var_pparam_b4soicle) as f64).is_finite() && ((locals.var_pparam_b4soicle) as f64).fract() == 0.0 { if locals.var_pparam_b4soicle == 0.0 { 0.0 } else { (locals.var_pparam_b4soicle * ((assign1310_e2775).powf(locals.var_pparam_b4soicle - 1.0) * (-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn12) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))))) } } else { (assign1310_e2777 * (locals.var_pparam_b4soicle * ((-((locals.var_pparam_b4soiclc * locals.var_pparam_b4soileff_dn12) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) / assign1310_e2775))) },)
    } else {
        (locals.var_pparam_b4soiabulkcvfactor, locals.var_pparam_b4soiabulkcvfactor_dn3, locals.var_pparam_b4soiabulkcvfactor_dn4, locals.var_pparam_b4soiabulkcvfactor_dn5, locals.var_pparam_b4soiabulkcvfactor_dn6, locals.var_pparam_b4soiabulkcvfactor_dn7, locals.var_pparam_b4soiabulkcvfactor_dn8, locals.var_pparam_b4soiabulkcvfactor_dn9, locals.var_pparam_b4soiabulkcvfactor_dn10, locals.var_pparam_b4soiabulkcvfactor_dn11, locals.var_pparam_b4soiabulkcvfactor_dn12,)
    }
};
        locals.var_pparam_b4soiabulkcvfactor = assign1310_e2780;
        locals.var_pparam_b4soiabulkcvfactor_dn3 = assign1310_e2780_d_n3;
        locals.var_pparam_b4soiabulkcvfactor_dn4 = assign1310_e2780_d_n4;
        locals.var_pparam_b4soiabulkcvfactor_dn5 = assign1310_e2780_d_n5;
        locals.var_pparam_b4soiabulkcvfactor_dn6 = assign1310_e2780_d_n6;
        locals.var_pparam_b4soiabulkcvfactor_dn7 = assign1310_e2780_d_n7;
        locals.var_pparam_b4soiabulkcvfactor_dn8 = assign1310_e2780_d_n8;
        locals.var_pparam_b4soiabulkcvfactor_dn9 = assign1310_e2780_d_n9;
        locals.var_pparam_b4soiabulkcvfactor_dn10 = assign1310_e2780_d_n10;
        locals.var_pparam_b4soiabulkcvfactor_dn11 = assign1310_e2780_d_n11;
        locals.var_pparam_b4soiabulkcvfactor_dn12 = assign1310_e2780_d_n12;

        let assign1320_e2783: f64 = if p.p65 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard509 = assign1320_e2783;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1330_e2789, assign1330_e2789_d_n3, assign1330_e2789_d_n4, assign1330_e2789_d_n5, assign1330_e2789_d_n6, assign1330_e2789_d_n7, assign1330_e2789_d_n8, assign1330_e2789_d_n9, assign1330_e2789_d_n10, assign1330_e2789_d_n11, assign1330_e2789_d_n12,) = {
    if (locals.var_guard509 != 0.0) {
        let assign1330_e2787: f64 = (1e-6 / locals.var_pparam_b4soileff);
        (assign1330_e2787, (-((1e-6 * locals.var_pparam_b4soileff_dn3) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-((1e-6 * locals.var_pparam_b4soileff_dn4) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-((1e-6 * locals.var_pparam_b4soileff_dn5) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-((1e-6 * locals.var_pparam_b4soileff_dn6) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-((1e-6 * locals.var_pparam_b4soileff_dn7) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-((1e-6 * locals.var_pparam_b4soileff_dn8) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-((1e-6 * locals.var_pparam_b4soileff_dn9) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-((1e-6 * locals.var_pparam_b4soileff_dn10) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-((1e-6 * locals.var_pparam_b4soileff_dn11) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-((1e-6 * locals.var_pparam_b4soileff_dn12) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))),)
    } else {
        (locals.var_inv_l, locals.var_inv_l_dn3, locals.var_inv_l_dn4, locals.var_inv_l_dn5, locals.var_inv_l_dn6, locals.var_inv_l_dn7, locals.var_inv_l_dn8, locals.var_inv_l_dn9, locals.var_inv_l_dn10, locals.var_inv_l_dn11, locals.var_inv_l_dn12,)
    }
};
        locals.var_inv_l = assign1330_e2789;
        locals.var_inv_l_dn3 = assign1330_e2789_d_n3;
        locals.var_inv_l_dn4 = assign1330_e2789_d_n4;
        locals.var_inv_l_dn5 = assign1330_e2789_d_n5;
        locals.var_inv_l_dn6 = assign1330_e2789_d_n6;
        locals.var_inv_l_dn7 = assign1330_e2789_d_n7;
        locals.var_inv_l_dn8 = assign1330_e2789_d_n8;
        locals.var_inv_l_dn9 = assign1330_e2789_d_n9;
        locals.var_inv_l_dn10 = assign1330_e2789_d_n10;
        locals.var_inv_l_dn11 = assign1330_e2789_d_n11;
        locals.var_inv_l_dn12 = assign1330_e2789_d_n12;

        let (assign1340_e2795, assign1340_e2795_d_n3, assign1340_e2795_d_n4, assign1340_e2795_d_n5, assign1340_e2795_d_n6, assign1340_e2795_d_n7, assign1340_e2795_d_n8, assign1340_e2795_d_n9, assign1340_e2795_d_n10, assign1340_e2795_d_n11, assign1340_e2795_d_n12,) = {
    if (locals.var_guard509 != 0.0) {
        let assign1340_e2793: f64 = (1e-6 / locals.var_pparam_b4soiweff);
        (assign1340_e2793, (-((1e-6 * locals.var_pparam_b4soiweff_dn3) / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-((1e-6 * locals.var_pparam_b4soiweff_dn4) / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-((1e-6 * locals.var_pparam_b4soiweff_dn5) / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-((1e-6 * locals.var_pparam_b4soiweff_dn6) / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-((1e-6 * locals.var_pparam_b4soiweff_dn7) / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-((1e-6 * locals.var_pparam_b4soiweff_dn8) / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-((1e-6 * locals.var_pparam_b4soiweff_dn9) / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-((1e-6 * locals.var_pparam_b4soiweff_dn10) / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-((1e-6 * locals.var_pparam_b4soiweff_dn11) / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-((1e-6 * locals.var_pparam_b4soiweff_dn12) / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))),)
    } else {
        (locals.var_inv_w, locals.var_inv_w_dn3, locals.var_inv_w_dn4, locals.var_inv_w_dn5, locals.var_inv_w_dn6, locals.var_inv_w_dn7, locals.var_inv_w_dn8, locals.var_inv_w_dn9, locals.var_inv_w_dn10, locals.var_inv_w_dn11, locals.var_inv_w_dn12,)
    }
};
        locals.var_inv_w = assign1340_e2795;
        locals.var_inv_w_dn3 = assign1340_e2795_d_n3;
        locals.var_inv_w_dn4 = assign1340_e2795_d_n4;
        locals.var_inv_w_dn5 = assign1340_e2795_d_n5;
        locals.var_inv_w_dn6 = assign1340_e2795_d_n6;
        locals.var_inv_w_dn7 = assign1340_e2795_d_n7;
        locals.var_inv_w_dn8 = assign1340_e2795_d_n8;
        locals.var_inv_w_dn9 = assign1340_e2795_d_n9;
        locals.var_inv_w_dn10 = assign1340_e2795_d_n10;
        locals.var_inv_w_dn11 = assign1340_e2795_d_n11;
        locals.var_inv_w_dn12 = assign1340_e2795_d_n12;

        let (assign1350_e2803, assign1350_e2803_d_n3, assign1350_e2803_d_n4, assign1350_e2803_d_n5, assign1350_e2803_d_n6, assign1350_e2803_d_n7, assign1350_e2803_d_n8, assign1350_e2803_d_n9, assign1350_e2803_d_n10, assign1350_e2803_d_n11, assign1350_e2803_d_n12,) = {
    if (locals.var_guard509 != 0.0) {
        let assign1350_e2800: f64 = (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff);
        let assign1350_e2801: f64 = (1e-12 / assign1350_e2800);
        (assign1350_e2801, (-((1e-12 * ((locals.var_pparam_b4soileff_dn3 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn3))) / (assign1350_e2800 * assign1350_e2800))), (-((1e-12 * ((locals.var_pparam_b4soileff_dn4 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn4))) / (assign1350_e2800 * assign1350_e2800))), (-((1e-12 * ((locals.var_pparam_b4soileff_dn5 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn5))) / (assign1350_e2800 * assign1350_e2800))), (-((1e-12 * ((locals.var_pparam_b4soileff_dn6 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn6))) / (assign1350_e2800 * assign1350_e2800))), (-((1e-12 * ((locals.var_pparam_b4soileff_dn7 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn7))) / (assign1350_e2800 * assign1350_e2800))), (-((1e-12 * ((locals.var_pparam_b4soileff_dn8 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn8))) / (assign1350_e2800 * assign1350_e2800))), (-((1e-12 * ((locals.var_pparam_b4soileff_dn9 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn9))) / (assign1350_e2800 * assign1350_e2800))), (-((1e-12 * ((locals.var_pparam_b4soileff_dn10 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn10))) / (assign1350_e2800 * assign1350_e2800))), (-((1e-12 * ((locals.var_pparam_b4soileff_dn11 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn11))) / (assign1350_e2800 * assign1350_e2800))), (-((1e-12 * ((locals.var_pparam_b4soileff_dn12 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn12))) / (assign1350_e2800 * assign1350_e2800))),)
    } else {
        (locals.var_inv_lw, locals.var_inv_lw_dn3, locals.var_inv_lw_dn4, locals.var_inv_lw_dn5, locals.var_inv_lw_dn6, locals.var_inv_lw_dn7, locals.var_inv_lw_dn8, locals.var_inv_lw_dn9, locals.var_inv_lw_dn10, locals.var_inv_lw_dn11, locals.var_inv_lw_dn12,)
    }
};
        locals.var_inv_lw = assign1350_e2803;
        locals.var_inv_lw_dn3 = assign1350_e2803_d_n3;
        locals.var_inv_lw_dn4 = assign1350_e2803_d_n4;
        locals.var_inv_lw_dn5 = assign1350_e2803_d_n5;
        locals.var_inv_lw_dn6 = assign1350_e2803_d_n6;
        locals.var_inv_lw_dn7 = assign1350_e2803_d_n7;
        locals.var_inv_lw_dn8 = assign1350_e2803_d_n8;
        locals.var_inv_lw_dn9 = assign1350_e2803_d_n9;
        locals.var_inv_lw_dn10 = assign1350_e2803_d_n10;
        locals.var_inv_lw_dn11 = assign1350_e2803_d_n11;
        locals.var_inv_lw_dn12 = assign1350_e2803_d_n12;

        let (assign1360_e2810, assign1360_e2810_d_n3, assign1360_e2810_d_n4, assign1360_e2810_d_n5, assign1360_e2810_d_n6, assign1360_e2810_d_n7, assign1360_e2810_d_n8, assign1360_e2810_d_n9, assign1360_e2810_d_n10, assign1360_e2810_d_n11, assign1360_e2810_d_n12,) = {
    if (locals.var_guard509 == 0.0) {
        let assign1360_e2808: f64 = (1.0 / locals.var_pparam_b4soileff);
        (assign1360_e2808, (-(locals.var_pparam_b4soileff_dn3 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-(locals.var_pparam_b4soileff_dn4 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-(locals.var_pparam_b4soileff_dn5 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-(locals.var_pparam_b4soileff_dn6 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-(locals.var_pparam_b4soileff_dn7 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-(locals.var_pparam_b4soileff_dn8 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-(locals.var_pparam_b4soileff_dn9 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-(locals.var_pparam_b4soileff_dn10 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-(locals.var_pparam_b4soileff_dn11 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))), (-(locals.var_pparam_b4soileff_dn12 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))),)
    } else {
        (locals.var_inv_l, locals.var_inv_l_dn3, locals.var_inv_l_dn4, locals.var_inv_l_dn5, locals.var_inv_l_dn6, locals.var_inv_l_dn7, locals.var_inv_l_dn8, locals.var_inv_l_dn9, locals.var_inv_l_dn10, locals.var_inv_l_dn11, locals.var_inv_l_dn12,)
    }
};
        locals.var_inv_l = assign1360_e2810;
        locals.var_inv_l_dn3 = assign1360_e2810_d_n3;
        locals.var_inv_l_dn4 = assign1360_e2810_d_n4;
        locals.var_inv_l_dn5 = assign1360_e2810_d_n5;
        locals.var_inv_l_dn6 = assign1360_e2810_d_n6;
        locals.var_inv_l_dn7 = assign1360_e2810_d_n7;
        locals.var_inv_l_dn8 = assign1360_e2810_d_n8;
        locals.var_inv_l_dn9 = assign1360_e2810_d_n9;
        locals.var_inv_l_dn10 = assign1360_e2810_d_n10;
        locals.var_inv_l_dn11 = assign1360_e2810_d_n11;
        locals.var_inv_l_dn12 = assign1360_e2810_d_n12;

        let (assign1370_e2817, assign1370_e2817_d_n3, assign1370_e2817_d_n4, assign1370_e2817_d_n5, assign1370_e2817_d_n6, assign1370_e2817_d_n7, assign1370_e2817_d_n8, assign1370_e2817_d_n9, assign1370_e2817_d_n10, assign1370_e2817_d_n11, assign1370_e2817_d_n12,) = {
    if (locals.var_guard509 == 0.0) {
        let assign1370_e2815: f64 = (1.0 / locals.var_pparam_b4soiweff);
        (assign1370_e2815, (-(locals.var_pparam_b4soiweff_dn3 / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-(locals.var_pparam_b4soiweff_dn4 / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-(locals.var_pparam_b4soiweff_dn5 / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-(locals.var_pparam_b4soiweff_dn6 / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-(locals.var_pparam_b4soiweff_dn7 / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-(locals.var_pparam_b4soiweff_dn8 / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-(locals.var_pparam_b4soiweff_dn9 / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-(locals.var_pparam_b4soiweff_dn10 / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-(locals.var_pparam_b4soiweff_dn11 / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))), (-(locals.var_pparam_b4soiweff_dn12 / (locals.var_pparam_b4soiweff * locals.var_pparam_b4soiweff))),)
    } else {
        (locals.var_inv_w, locals.var_inv_w_dn3, locals.var_inv_w_dn4, locals.var_inv_w_dn5, locals.var_inv_w_dn6, locals.var_inv_w_dn7, locals.var_inv_w_dn8, locals.var_inv_w_dn9, locals.var_inv_w_dn10, locals.var_inv_w_dn11, locals.var_inv_w_dn12,)
    }
};
        locals.var_inv_w = assign1370_e2817;
        locals.var_inv_w_dn3 = assign1370_e2817_d_n3;
        locals.var_inv_w_dn4 = assign1370_e2817_d_n4;
        locals.var_inv_w_dn5 = assign1370_e2817_d_n5;
        locals.var_inv_w_dn6 = assign1370_e2817_d_n6;
        locals.var_inv_w_dn7 = assign1370_e2817_d_n7;
        locals.var_inv_w_dn8 = assign1370_e2817_d_n8;
        locals.var_inv_w_dn9 = assign1370_e2817_d_n9;
        locals.var_inv_w_dn10 = assign1370_e2817_d_n10;
        locals.var_inv_w_dn11 = assign1370_e2817_d_n11;
        locals.var_inv_w_dn12 = assign1370_e2817_d_n12;

        let (assign1380_e2826, assign1380_e2826_d_n3, assign1380_e2826_d_n4, assign1380_e2826_d_n5, assign1380_e2826_d_n6, assign1380_e2826_d_n7, assign1380_e2826_d_n8, assign1380_e2826_d_n9, assign1380_e2826_d_n10, assign1380_e2826_d_n11, assign1380_e2826_d_n12,) = {
    if (locals.var_guard509 == 0.0) {
        let assign1380_e2823: f64 = (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff);
        let assign1380_e2824: f64 = (1.0 / assign1380_e2823);
        (assign1380_e2824, (-(((locals.var_pparam_b4soileff_dn3 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn3)) / (assign1380_e2823 * assign1380_e2823))), (-(((locals.var_pparam_b4soileff_dn4 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn4)) / (assign1380_e2823 * assign1380_e2823))), (-(((locals.var_pparam_b4soileff_dn5 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn5)) / (assign1380_e2823 * assign1380_e2823))), (-(((locals.var_pparam_b4soileff_dn6 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn6)) / (assign1380_e2823 * assign1380_e2823))), (-(((locals.var_pparam_b4soileff_dn7 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn7)) / (assign1380_e2823 * assign1380_e2823))), (-(((locals.var_pparam_b4soileff_dn8 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn8)) / (assign1380_e2823 * assign1380_e2823))), (-(((locals.var_pparam_b4soileff_dn9 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn9)) / (assign1380_e2823 * assign1380_e2823))), (-(((locals.var_pparam_b4soileff_dn10 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn10)) / (assign1380_e2823 * assign1380_e2823))), (-(((locals.var_pparam_b4soileff_dn11 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn11)) / (assign1380_e2823 * assign1380_e2823))), (-(((locals.var_pparam_b4soileff_dn12 * locals.var_pparam_b4soiweff) + (locals.var_pparam_b4soileff * locals.var_pparam_b4soiweff_dn12)) / (assign1380_e2823 * assign1380_e2823))),)
    } else {
        (locals.var_inv_lw, locals.var_inv_lw_dn3, locals.var_inv_lw_dn4, locals.var_inv_lw_dn5, locals.var_inv_lw_dn6, locals.var_inv_lw_dn7, locals.var_inv_lw_dn8, locals.var_inv_lw_dn9, locals.var_inv_lw_dn10, locals.var_inv_lw_dn11, locals.var_inv_lw_dn12,)
    }
};
        locals.var_inv_lw = assign1380_e2826;
        locals.var_inv_lw_dn3 = assign1380_e2826_d_n3;
        locals.var_inv_lw_dn4 = assign1380_e2826_d_n4;
        locals.var_inv_lw_dn5 = assign1380_e2826_d_n5;
        locals.var_inv_lw_dn6 = assign1380_e2826_d_n6;
        locals.var_inv_lw_dn7 = assign1380_e2826_d_n7;
        locals.var_inv_lw_dn8 = assign1380_e2826_d_n8;
        locals.var_inv_lw_dn9 = assign1380_e2826_d_n9;
        locals.var_inv_lw_dn10 = assign1380_e2826_d_n10;
        locals.var_inv_lw_dn11 = assign1380_e2826_d_n11;
        locals.var_inv_lw_dn12 = assign1380_e2826_d_n12;

        let assign1390_e2830: f64 = (p.p488 * locals.var_inv_l);
        let assign1390_e2831: f64 = (p.p82 + assign1390_e2830);
        let assign1390_e2834: f64 = (p.p678 * locals.var_inv_w);
        let assign1390_e2835: f64 = (assign1390_e2831 + assign1390_e2834);
        let assign1390_e2838: f64 = (p.p868 * locals.var_inv_lw);
        let assign1390_e2839: f64 = (assign1390_e2835 + assign1390_e2838);
        locals.var_pparam_b4soinpeak = assign1390_e2839;
        locals.var_pparam_b4soinpeak_dn3 = (((p.p488 * locals.var_inv_l_dn3) + (p.p678 * locals.var_inv_w_dn3)) + (p.p868 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soinpeak_dn4 = (((p.p488 * locals.var_inv_l_dn4) + (p.p678 * locals.var_inv_w_dn4)) + (p.p868 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soinpeak_dn5 = (((p.p488 * locals.var_inv_l_dn5) + (p.p678 * locals.var_inv_w_dn5)) + (p.p868 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soinpeak_dn6 = (((p.p488 * locals.var_inv_l_dn6) + (p.p678 * locals.var_inv_w_dn6)) + (p.p868 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soinpeak_dn7 = (((p.p488 * locals.var_inv_l_dn7) + (p.p678 * locals.var_inv_w_dn7)) + (p.p868 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soinpeak_dn8 = (((p.p488 * locals.var_inv_l_dn8) + (p.p678 * locals.var_inv_w_dn8)) + (p.p868 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soinpeak_dn9 = (((p.p488 * locals.var_inv_l_dn9) + (p.p678 * locals.var_inv_w_dn9)) + (p.p868 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soinpeak_dn10 = (((p.p488 * locals.var_inv_l_dn10) + (p.p678 * locals.var_inv_w_dn10)) + (p.p868 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soinpeak_dn11 = (((p.p488 * locals.var_inv_l_dn11) + (p.p678 * locals.var_inv_w_dn11)) + (p.p868 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soinpeak_dn12 = (((p.p488 * locals.var_inv_l_dn12) + (p.p678 * locals.var_inv_w_dn12)) + (p.p868 * locals.var_inv_lw_dn12));

        let assign1400_e2843: f64 = (p.p489 * locals.var_inv_l);
        let assign1400_e2844: f64 = (p.p81 + assign1400_e2843);
        let assign1400_e2847: f64 = (p.p679 * locals.var_inv_w);
        let assign1400_e2848: f64 = (assign1400_e2844 + assign1400_e2847);
        let assign1400_e2851: f64 = (p.p869 * locals.var_inv_lw);
        let assign1400_e2852: f64 = (assign1400_e2848 + assign1400_e2851);
        locals.var_pparam_b4soinsub = assign1400_e2852;
        locals.var_pparam_b4soinsub_dn3 = (((p.p489 * locals.var_inv_l_dn3) + (p.p679 * locals.var_inv_w_dn3)) + (p.p869 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soinsub_dn4 = (((p.p489 * locals.var_inv_l_dn4) + (p.p679 * locals.var_inv_w_dn4)) + (p.p869 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soinsub_dn5 = (((p.p489 * locals.var_inv_l_dn5) + (p.p679 * locals.var_inv_w_dn5)) + (p.p869 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soinsub_dn6 = (((p.p489 * locals.var_inv_l_dn6) + (p.p679 * locals.var_inv_w_dn6)) + (p.p869 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soinsub_dn7 = (((p.p489 * locals.var_inv_l_dn7) + (p.p679 * locals.var_inv_w_dn7)) + (p.p869 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soinsub_dn8 = (((p.p489 * locals.var_inv_l_dn8) + (p.p679 * locals.var_inv_w_dn8)) + (p.p869 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soinsub_dn9 = (((p.p489 * locals.var_inv_l_dn9) + (p.p679 * locals.var_inv_w_dn9)) + (p.p869 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soinsub_dn10 = (((p.p489 * locals.var_inv_l_dn10) + (p.p679 * locals.var_inv_w_dn10)) + (p.p869 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soinsub_dn11 = (((p.p489 * locals.var_inv_l_dn11) + (p.p679 * locals.var_inv_w_dn11)) + (p.p869 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soinsub_dn12 = (((p.p489 * locals.var_inv_l_dn12) + (p.p679 * locals.var_inv_w_dn12)) + (p.p869 * locals.var_inv_lw_dn12));

        let assign1420_e2859: f64 = (p.p490 * locals.var_inv_l);
        let assign1420_e2860: f64 = (p.p83 + assign1420_e2859);
        let assign1420_e2863: f64 = (p.p680 * locals.var_inv_w);
        let assign1420_e2864: f64 = (assign1420_e2860 + assign1420_e2863);
        let assign1420_e2867: f64 = (p.p871 * locals.var_inv_lw);
        let assign1420_e2868: f64 = (assign1420_e2864 + assign1420_e2867);
        locals.var_pparam_b4soingate = assign1420_e2868;
        locals.var_pparam_b4soingate_dn3 = (((p.p490 * locals.var_inv_l_dn3) + (p.p680 * locals.var_inv_w_dn3)) + (p.p871 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soingate_dn4 = (((p.p490 * locals.var_inv_l_dn4) + (p.p680 * locals.var_inv_w_dn4)) + (p.p871 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soingate_dn5 = (((p.p490 * locals.var_inv_l_dn5) + (p.p680 * locals.var_inv_w_dn5)) + (p.p871 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soingate_dn6 = (((p.p490 * locals.var_inv_l_dn6) + (p.p680 * locals.var_inv_w_dn6)) + (p.p871 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soingate_dn7 = (((p.p490 * locals.var_inv_l_dn7) + (p.p680 * locals.var_inv_w_dn7)) + (p.p871 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soingate_dn8 = (((p.p490 * locals.var_inv_l_dn8) + (p.p680 * locals.var_inv_w_dn8)) + (p.p871 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soingate_dn9 = (((p.p490 * locals.var_inv_l_dn9) + (p.p680 * locals.var_inv_w_dn9)) + (p.p871 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soingate_dn10 = (((p.p490 * locals.var_inv_l_dn10) + (p.p680 * locals.var_inv_w_dn10)) + (p.p871 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soingate_dn11 = (((p.p490 * locals.var_inv_l_dn11) + (p.p680 * locals.var_inv_w_dn11)) + (p.p871 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soingate_dn12 = (((p.p490 * locals.var_inv_l_dn12) + (p.p680 * locals.var_inv_w_dn12)) + (p.p871 * locals.var_inv_lw_dn12));

        let assign1430_e2872: f64 = (p.p491 * locals.var_inv_l);
        let assign1430_e2873: f64 = (p.p84 + assign1430_e2872);
        let assign1430_e2876: f64 = (p.p681 * locals.var_inv_w);
        let assign1430_e2877: f64 = (assign1430_e2873 + assign1430_e2876);
        let assign1430_e2880: f64 = (p.p870 * locals.var_inv_lw);
        let assign1430_e2881: f64 = (assign1430_e2877 + assign1430_e2880);
        locals.var_pparam_b4soinsd = assign1430_e2881;
        locals.var_pparam_b4soinsd_dn3 = (((p.p491 * locals.var_inv_l_dn3) + (p.p681 * locals.var_inv_w_dn3)) + (p.p870 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soinsd_dn4 = (((p.p491 * locals.var_inv_l_dn4) + (p.p681 * locals.var_inv_w_dn4)) + (p.p870 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soinsd_dn5 = (((p.p491 * locals.var_inv_l_dn5) + (p.p681 * locals.var_inv_w_dn5)) + (p.p870 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soinsd_dn6 = (((p.p491 * locals.var_inv_l_dn6) + (p.p681 * locals.var_inv_w_dn6)) + (p.p870 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soinsd_dn7 = (((p.p491 * locals.var_inv_l_dn7) + (p.p681 * locals.var_inv_w_dn7)) + (p.p870 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soinsd_dn8 = (((p.p491 * locals.var_inv_l_dn8) + (p.p681 * locals.var_inv_w_dn8)) + (p.p870 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soinsd_dn9 = (((p.p491 * locals.var_inv_l_dn9) + (p.p681 * locals.var_inv_w_dn9)) + (p.p870 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soinsd_dn10 = (((p.p491 * locals.var_inv_l_dn10) + (p.p681 * locals.var_inv_w_dn10)) + (p.p870 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soinsd_dn11 = (((p.p491 * locals.var_inv_l_dn11) + (p.p681 * locals.var_inv_w_dn11)) + (p.p870 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soinsd_dn12 = (((p.p491 * locals.var_inv_l_dn12) + (p.p681 * locals.var_inv_w_dn12)) + (p.p870 * locals.var_inv_lw_dn12));

        let assign1440_e2885: f64 = (p.p492 * locals.var_inv_l);
        let assign1440_e2886: f64 = (p.p108 + assign1440_e2885);
        let assign1440_e2889: f64 = (p.p682 * locals.var_inv_w);
        let assign1440_e2890: f64 = (assign1440_e2886 + assign1440_e2889);
        let assign1440_e2893: f64 = (p.p872 * locals.var_inv_lw);
        let assign1440_e2894: f64 = (assign1440_e2890 + assign1440_e2893);
        locals.var_pparam_b4soivth0 = assign1440_e2894;
        locals.var_pparam_b4soivth0_dn3 = (((p.p492 * locals.var_inv_l_dn3) + (p.p682 * locals.var_inv_w_dn3)) + (p.p872 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivth0_dn4 = (((p.p492 * locals.var_inv_l_dn4) + (p.p682 * locals.var_inv_w_dn4)) + (p.p872 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivth0_dn5 = (((p.p492 * locals.var_inv_l_dn5) + (p.p682 * locals.var_inv_w_dn5)) + (p.p872 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivth0_dn6 = (((p.p492 * locals.var_inv_l_dn6) + (p.p682 * locals.var_inv_w_dn6)) + (p.p872 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivth0_dn7 = (((p.p492 * locals.var_inv_l_dn7) + (p.p682 * locals.var_inv_w_dn7)) + (p.p872 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivth0_dn8 = (((p.p492 * locals.var_inv_l_dn8) + (p.p682 * locals.var_inv_w_dn8)) + (p.p872 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivth0_dn9 = (((p.p492 * locals.var_inv_l_dn9) + (p.p682 * locals.var_inv_w_dn9)) + (p.p872 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivth0_dn10 = (((p.p492 * locals.var_inv_l_dn10) + (p.p682 * locals.var_inv_w_dn10)) + (p.p872 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivth0_dn11 = (((p.p492 * locals.var_inv_l_dn11) + (p.p682 * locals.var_inv_w_dn11)) + (p.p872 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivth0_dn12 = (((p.p492 * locals.var_inv_l_dn12) + (p.p682 * locals.var_inv_w_dn12)) + (p.p872 * locals.var_inv_lw_dn12));

        let assign1450_e2898: f64 = (p.p493 * locals.var_inv_l);
        let assign1450_e2899: f64 = (p.p109 + assign1450_e2898);
        let assign1450_e2902: f64 = (p.p683 * locals.var_inv_w);
        let assign1450_e2903: f64 = (assign1450_e2899 + assign1450_e2902);
        let assign1450_e2906: f64 = (p.p873 * locals.var_inv_lw);
        let assign1450_e2907: f64 = (assign1450_e2903 + assign1450_e2906);
        locals.var_pparam_b4soivfb = assign1450_e2907;
        locals.var_pparam_b4soivfb_dn3 = (((p.p493 * locals.var_inv_l_dn3) + (p.p683 * locals.var_inv_w_dn3)) + (p.p873 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivfb_dn4 = (((p.p493 * locals.var_inv_l_dn4) + (p.p683 * locals.var_inv_w_dn4)) + (p.p873 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivfb_dn5 = (((p.p493 * locals.var_inv_l_dn5) + (p.p683 * locals.var_inv_w_dn5)) + (p.p873 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivfb_dn6 = (((p.p493 * locals.var_inv_l_dn6) + (p.p683 * locals.var_inv_w_dn6)) + (p.p873 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivfb_dn7 = (((p.p493 * locals.var_inv_l_dn7) + (p.p683 * locals.var_inv_w_dn7)) + (p.p873 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivfb_dn8 = (((p.p493 * locals.var_inv_l_dn8) + (p.p683 * locals.var_inv_w_dn8)) + (p.p873 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivfb_dn9 = (((p.p493 * locals.var_inv_l_dn9) + (p.p683 * locals.var_inv_w_dn9)) + (p.p873 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivfb_dn10 = (((p.p493 * locals.var_inv_l_dn10) + (p.p683 * locals.var_inv_w_dn10)) + (p.p873 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivfb_dn11 = (((p.p493 * locals.var_inv_l_dn11) + (p.p683 * locals.var_inv_w_dn11)) + (p.p873 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivfb_dn12 = (((p.p493 * locals.var_inv_l_dn12) + (p.p683 * locals.var_inv_w_dn12)) + (p.p873 * locals.var_inv_lw_dn12));

        let assign1460_e2911: f64 = (p.p494 * locals.var_inv_l);
        let assign1460_e2912: f64 = (p.p90 + assign1460_e2911);
        let assign1460_e2915: f64 = (p.p684 * locals.var_inv_w);
        let assign1460_e2916: f64 = (assign1460_e2912 + assign1460_e2915);
        let assign1460_e2919: f64 = (p.p874 * locals.var_inv_lw);
        let assign1460_e2920: f64 = (assign1460_e2916 + assign1460_e2919);
        locals.var_pparam_b4soik1 = assign1460_e2920;
        locals.var_pparam_b4soik1_dn3 = (((p.p494 * locals.var_inv_l_dn3) + (p.p684 * locals.var_inv_w_dn3)) + (p.p874 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soik1_dn4 = (((p.p494 * locals.var_inv_l_dn4) + (p.p684 * locals.var_inv_w_dn4)) + (p.p874 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soik1_dn5 = (((p.p494 * locals.var_inv_l_dn5) + (p.p684 * locals.var_inv_w_dn5)) + (p.p874 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soik1_dn6 = (((p.p494 * locals.var_inv_l_dn6) + (p.p684 * locals.var_inv_w_dn6)) + (p.p874 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soik1_dn7 = (((p.p494 * locals.var_inv_l_dn7) + (p.p684 * locals.var_inv_w_dn7)) + (p.p874 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soik1_dn8 = (((p.p494 * locals.var_inv_l_dn8) + (p.p684 * locals.var_inv_w_dn8)) + (p.p874 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soik1_dn9 = (((p.p494 * locals.var_inv_l_dn9) + (p.p684 * locals.var_inv_w_dn9)) + (p.p874 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soik1_dn10 = (((p.p494 * locals.var_inv_l_dn10) + (p.p684 * locals.var_inv_w_dn10)) + (p.p874 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soik1_dn11 = (((p.p494 * locals.var_inv_l_dn11) + (p.p684 * locals.var_inv_w_dn11)) + (p.p874 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soik1_dn12 = (((p.p494 * locals.var_inv_l_dn12) + (p.p684 * locals.var_inv_w_dn12)) + (p.p874 * locals.var_inv_lw_dn12));

        let assign1470_e2924: f64 = (p.p497 * locals.var_inv_l);
        let assign1470_e2925: f64 = (p.p94 + assign1470_e2924);
        let assign1470_e2928: f64 = (p.p687 * locals.var_inv_w);
        let assign1470_e2929: f64 = (assign1470_e2925 + assign1470_e2928);
        let assign1470_e2932: f64 = (p.p877 * locals.var_inv_lw);
        let assign1470_e2933: f64 = (assign1470_e2929 + assign1470_e2932);
        locals.var_pparam_b4soik2 = assign1470_e2933;
        locals.var_pparam_b4soik2_dn3 = (((p.p497 * locals.var_inv_l_dn3) + (p.p687 * locals.var_inv_w_dn3)) + (p.p877 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soik2_dn4 = (((p.p497 * locals.var_inv_l_dn4) + (p.p687 * locals.var_inv_w_dn4)) + (p.p877 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soik2_dn5 = (((p.p497 * locals.var_inv_l_dn5) + (p.p687 * locals.var_inv_w_dn5)) + (p.p877 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soik2_dn6 = (((p.p497 * locals.var_inv_l_dn6) + (p.p687 * locals.var_inv_w_dn6)) + (p.p877 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soik2_dn7 = (((p.p497 * locals.var_inv_l_dn7) + (p.p687 * locals.var_inv_w_dn7)) + (p.p877 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soik2_dn8 = (((p.p497 * locals.var_inv_l_dn8) + (p.p687 * locals.var_inv_w_dn8)) + (p.p877 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soik2_dn9 = (((p.p497 * locals.var_inv_l_dn9) + (p.p687 * locals.var_inv_w_dn9)) + (p.p877 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soik2_dn10 = (((p.p497 * locals.var_inv_l_dn10) + (p.p687 * locals.var_inv_w_dn10)) + (p.p877 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soik2_dn11 = (((p.p497 * locals.var_inv_l_dn11) + (p.p687 * locals.var_inv_w_dn11)) + (p.p877 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soik2_dn12 = (((p.p497 * locals.var_inv_l_dn12) + (p.p687 * locals.var_inv_w_dn12)) + (p.p877 * locals.var_inv_lw_dn12));

        let assign1480_e2937: f64 = (p.p495 * locals.var_inv_l);
        let assign1480_e2938: f64 = (p.p300 + assign1480_e2937);
        let assign1480_e2941: f64 = (p.p685 * locals.var_inv_w);
        let assign1480_e2942: f64 = (assign1480_e2938 + assign1480_e2941);
        let assign1480_e2945: f64 = (p.p875 * locals.var_inv_lw);
        let assign1480_e2946: f64 = (assign1480_e2942 + assign1480_e2945);
        locals.var_pparam_b4soik1w1 = assign1480_e2946;
        locals.var_pparam_b4soik1w1_dn3 = (((p.p495 * locals.var_inv_l_dn3) + (p.p685 * locals.var_inv_w_dn3)) + (p.p875 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soik1w1_dn4 = (((p.p495 * locals.var_inv_l_dn4) + (p.p685 * locals.var_inv_w_dn4)) + (p.p875 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soik1w1_dn5 = (((p.p495 * locals.var_inv_l_dn5) + (p.p685 * locals.var_inv_w_dn5)) + (p.p875 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soik1w1_dn6 = (((p.p495 * locals.var_inv_l_dn6) + (p.p685 * locals.var_inv_w_dn6)) + (p.p875 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soik1w1_dn7 = (((p.p495 * locals.var_inv_l_dn7) + (p.p685 * locals.var_inv_w_dn7)) + (p.p875 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soik1w1_dn8 = (((p.p495 * locals.var_inv_l_dn8) + (p.p685 * locals.var_inv_w_dn8)) + (p.p875 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soik1w1_dn9 = (((p.p495 * locals.var_inv_l_dn9) + (p.p685 * locals.var_inv_w_dn9)) + (p.p875 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soik1w1_dn10 = (((p.p495 * locals.var_inv_l_dn10) + (p.p685 * locals.var_inv_w_dn10)) + (p.p875 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soik1w1_dn11 = (((p.p495 * locals.var_inv_l_dn11) + (p.p685 * locals.var_inv_w_dn11)) + (p.p875 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soik1w1_dn12 = (((p.p495 * locals.var_inv_l_dn12) + (p.p685 * locals.var_inv_w_dn12)) + (p.p875 * locals.var_inv_lw_dn12));

        let assign1490_e2950: f64 = (p.p496 * locals.var_inv_l);
        let assign1490_e2951: f64 = (p.p301 + assign1490_e2950);
        let assign1490_e2954: f64 = (p.p686 * locals.var_inv_w);
        let assign1490_e2955: f64 = (assign1490_e2951 + assign1490_e2954);
        let assign1490_e2958: f64 = (p.p876 * locals.var_inv_lw);
        let assign1490_e2959: f64 = (assign1490_e2955 + assign1490_e2958);
        locals.var_pparam_b4soik1w2 = assign1490_e2959;
        locals.var_pparam_b4soik1w2_dn3 = (((p.p496 * locals.var_inv_l_dn3) + (p.p686 * locals.var_inv_w_dn3)) + (p.p876 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soik1w2_dn4 = (((p.p496 * locals.var_inv_l_dn4) + (p.p686 * locals.var_inv_w_dn4)) + (p.p876 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soik1w2_dn5 = (((p.p496 * locals.var_inv_l_dn5) + (p.p686 * locals.var_inv_w_dn5)) + (p.p876 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soik1w2_dn6 = (((p.p496 * locals.var_inv_l_dn6) + (p.p686 * locals.var_inv_w_dn6)) + (p.p876 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soik1w2_dn7 = (((p.p496 * locals.var_inv_l_dn7) + (p.p686 * locals.var_inv_w_dn7)) + (p.p876 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soik1w2_dn8 = (((p.p496 * locals.var_inv_l_dn8) + (p.p686 * locals.var_inv_w_dn8)) + (p.p876 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soik1w2_dn9 = (((p.p496 * locals.var_inv_l_dn9) + (p.p686 * locals.var_inv_w_dn9)) + (p.p876 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soik1w2_dn10 = (((p.p496 * locals.var_inv_l_dn10) + (p.p686 * locals.var_inv_w_dn10)) + (p.p876 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soik1w2_dn11 = (((p.p496 * locals.var_inv_l_dn11) + (p.p686 * locals.var_inv_w_dn11)) + (p.p876 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soik1w2_dn12 = (((p.p496 * locals.var_inv_l_dn12) + (p.p686 * locals.var_inv_w_dn12)) + (p.p876 * locals.var_inv_lw_dn12));

        let assign1500_e2963: f64 = (p.p498 * locals.var_inv_l);
        let assign1500_e2964: f64 = (p.p95 + assign1500_e2963);
        let assign1500_e2967: f64 = (p.p688 * locals.var_inv_w);
        let assign1500_e2968: f64 = (assign1500_e2964 + assign1500_e2967);
        let assign1500_e2971: f64 = (p.p878 * locals.var_inv_lw);
        let assign1500_e2972: f64 = (assign1500_e2968 + assign1500_e2971);
        locals.var_pparam_b4soik3 = assign1500_e2972;
        locals.var_pparam_b4soik3_dn3 = (((p.p498 * locals.var_inv_l_dn3) + (p.p688 * locals.var_inv_w_dn3)) + (p.p878 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soik3_dn4 = (((p.p498 * locals.var_inv_l_dn4) + (p.p688 * locals.var_inv_w_dn4)) + (p.p878 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soik3_dn5 = (((p.p498 * locals.var_inv_l_dn5) + (p.p688 * locals.var_inv_w_dn5)) + (p.p878 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soik3_dn6 = (((p.p498 * locals.var_inv_l_dn6) + (p.p688 * locals.var_inv_w_dn6)) + (p.p878 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soik3_dn7 = (((p.p498 * locals.var_inv_l_dn7) + (p.p688 * locals.var_inv_w_dn7)) + (p.p878 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soik3_dn8 = (((p.p498 * locals.var_inv_l_dn8) + (p.p688 * locals.var_inv_w_dn8)) + (p.p878 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soik3_dn9 = (((p.p498 * locals.var_inv_l_dn9) + (p.p688 * locals.var_inv_w_dn9)) + (p.p878 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soik3_dn10 = (((p.p498 * locals.var_inv_l_dn10) + (p.p688 * locals.var_inv_w_dn10)) + (p.p878 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soik3_dn11 = (((p.p498 * locals.var_inv_l_dn11) + (p.p688 * locals.var_inv_w_dn11)) + (p.p878 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soik3_dn12 = (((p.p498 * locals.var_inv_l_dn12) + (p.p688 * locals.var_inv_w_dn12)) + (p.p878 * locals.var_inv_lw_dn12));

        let assign1510_e2976: f64 = (p.p499 * locals.var_inv_l);
        let assign1510_e2977: f64 = (p.p96 + assign1510_e2976);
        let assign1510_e2980: f64 = (p.p689 * locals.var_inv_w);
        let assign1510_e2981: f64 = (assign1510_e2977 + assign1510_e2980);
        let assign1510_e2984: f64 = (p.p879 * locals.var_inv_lw);
        let assign1510_e2985: f64 = (assign1510_e2981 + assign1510_e2984);
        locals.var_pparam_b4soik3b = assign1510_e2985;
        locals.var_pparam_b4soik3b_dn3 = (((p.p499 * locals.var_inv_l_dn3) + (p.p689 * locals.var_inv_w_dn3)) + (p.p879 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soik3b_dn4 = (((p.p499 * locals.var_inv_l_dn4) + (p.p689 * locals.var_inv_w_dn4)) + (p.p879 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soik3b_dn5 = (((p.p499 * locals.var_inv_l_dn5) + (p.p689 * locals.var_inv_w_dn5)) + (p.p879 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soik3b_dn6 = (((p.p499 * locals.var_inv_l_dn6) + (p.p689 * locals.var_inv_w_dn6)) + (p.p879 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soik3b_dn7 = (((p.p499 * locals.var_inv_l_dn7) + (p.p689 * locals.var_inv_w_dn7)) + (p.p879 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soik3b_dn8 = (((p.p499 * locals.var_inv_l_dn8) + (p.p689 * locals.var_inv_w_dn8)) + (p.p879 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soik3b_dn9 = (((p.p499 * locals.var_inv_l_dn9) + (p.p689 * locals.var_inv_w_dn9)) + (p.p879 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soik3b_dn10 = (((p.p499 * locals.var_inv_l_dn10) + (p.p689 * locals.var_inv_w_dn10)) + (p.p879 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soik3b_dn11 = (((p.p499 * locals.var_inv_l_dn11) + (p.p689 * locals.var_inv_w_dn11)) + (p.p879 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soik3b_dn12 = (((p.p499 * locals.var_inv_l_dn12) + (p.p689 * locals.var_inv_w_dn12)) + (p.p879 * locals.var_inv_lw_dn12));

        let assign1520_e2989: f64 = (p.p500 * locals.var_inv_l);
        let assign1520_e2990: f64 = (p.p371 + assign1520_e2989);
        let assign1520_e2993: f64 = (p.p690 * locals.var_inv_w);
        let assign1520_e2994: f64 = (assign1520_e2990 + assign1520_e2993);
        let assign1520_e2997: f64 = (p.p880 * locals.var_inv_lw);
        let assign1520_e2998: f64 = (assign1520_e2994 + assign1520_e2997);
        locals.var_pparam_b4soikb1 = assign1520_e2998;
        locals.var_pparam_b4soikb1_dn3 = (((p.p500 * locals.var_inv_l_dn3) + (p.p690 * locals.var_inv_w_dn3)) + (p.p880 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soikb1_dn4 = (((p.p500 * locals.var_inv_l_dn4) + (p.p690 * locals.var_inv_w_dn4)) + (p.p880 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soikb1_dn5 = (((p.p500 * locals.var_inv_l_dn5) + (p.p690 * locals.var_inv_w_dn5)) + (p.p880 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soikb1_dn6 = (((p.p500 * locals.var_inv_l_dn6) + (p.p690 * locals.var_inv_w_dn6)) + (p.p880 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soikb1_dn7 = (((p.p500 * locals.var_inv_l_dn7) + (p.p690 * locals.var_inv_w_dn7)) + (p.p880 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soikb1_dn8 = (((p.p500 * locals.var_inv_l_dn8) + (p.p690 * locals.var_inv_w_dn8)) + (p.p880 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soikb1_dn9 = (((p.p500 * locals.var_inv_l_dn9) + (p.p690 * locals.var_inv_w_dn9)) + (p.p880 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soikb1_dn10 = (((p.p500 * locals.var_inv_l_dn10) + (p.p690 * locals.var_inv_w_dn10)) + (p.p880 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soikb1_dn11 = (((p.p500 * locals.var_inv_l_dn11) + (p.p690 * locals.var_inv_w_dn11)) + (p.p880 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soikb1_dn12 = (((p.p500 * locals.var_inv_l_dn12) + (p.p690 * locals.var_inv_w_dn12)) + (p.p880 * locals.var_inv_lw_dn12));

        let assign1530_e3002: f64 = (p.p501 * locals.var_inv_l);
        let assign1530_e3003: f64 = (p.p97 + assign1530_e3002);
        let assign1530_e3006: f64 = (p.p691 * locals.var_inv_w);
        let assign1530_e3007: f64 = (assign1530_e3003 + assign1530_e3006);
        let assign1530_e3010: f64 = (p.p881 * locals.var_inv_lw);
        let assign1530_e3011: f64 = (assign1530_e3007 + assign1530_e3010);
        locals.var_pparam_b4soiw0 = assign1530_e3011;
        locals.var_pparam_b4soiw0_dn3 = (((p.p501 * locals.var_inv_l_dn3) + (p.p691 * locals.var_inv_w_dn3)) + (p.p881 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiw0_dn4 = (((p.p501 * locals.var_inv_l_dn4) + (p.p691 * locals.var_inv_w_dn4)) + (p.p881 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiw0_dn5 = (((p.p501 * locals.var_inv_l_dn5) + (p.p691 * locals.var_inv_w_dn5)) + (p.p881 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiw0_dn6 = (((p.p501 * locals.var_inv_l_dn6) + (p.p691 * locals.var_inv_w_dn6)) + (p.p881 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiw0_dn7 = (((p.p501 * locals.var_inv_l_dn7) + (p.p691 * locals.var_inv_w_dn7)) + (p.p881 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiw0_dn8 = (((p.p501 * locals.var_inv_l_dn8) + (p.p691 * locals.var_inv_w_dn8)) + (p.p881 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiw0_dn9 = (((p.p501 * locals.var_inv_l_dn9) + (p.p691 * locals.var_inv_w_dn9)) + (p.p881 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiw0_dn10 = (((p.p501 * locals.var_inv_l_dn10) + (p.p691 * locals.var_inv_w_dn10)) + (p.p881 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiw0_dn11 = (((p.p501 * locals.var_inv_l_dn11) + (p.p691 * locals.var_inv_w_dn11)) + (p.p881 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiw0_dn12 = (((p.p501 * locals.var_inv_l_dn12) + (p.p691 * locals.var_inv_w_dn12)) + (p.p881 * locals.var_inv_lw_dn12));

        let assign1540_e3015: f64 = (p.p1024 * locals.var_inv_l);
        let assign1540_e3016: f64 = (p.p1021 + assign1540_e3015);
        let assign1540_e3019: f64 = (p.p1027 * locals.var_inv_w);
        let assign1540_e3020: f64 = (assign1540_e3016 + assign1540_e3019);
        let assign1540_e3023: f64 = (p.p1030 * locals.var_inv_lw);
        let assign1540_e3024: f64 = (assign1540_e3020 + assign1540_e3023);
        locals.var_pparam_b4soilpe0 = assign1540_e3024;
        locals.var_pparam_b4soilpe0_dn3 = (((p.p1024 * locals.var_inv_l_dn3) + (p.p1027 * locals.var_inv_w_dn3)) + (p.p1030 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soilpe0_dn4 = (((p.p1024 * locals.var_inv_l_dn4) + (p.p1027 * locals.var_inv_w_dn4)) + (p.p1030 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soilpe0_dn5 = (((p.p1024 * locals.var_inv_l_dn5) + (p.p1027 * locals.var_inv_w_dn5)) + (p.p1030 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soilpe0_dn6 = (((p.p1024 * locals.var_inv_l_dn6) + (p.p1027 * locals.var_inv_w_dn6)) + (p.p1030 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soilpe0_dn7 = (((p.p1024 * locals.var_inv_l_dn7) + (p.p1027 * locals.var_inv_w_dn7)) + (p.p1030 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soilpe0_dn8 = (((p.p1024 * locals.var_inv_l_dn8) + (p.p1027 * locals.var_inv_w_dn8)) + (p.p1030 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soilpe0_dn9 = (((p.p1024 * locals.var_inv_l_dn9) + (p.p1027 * locals.var_inv_w_dn9)) + (p.p1030 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soilpe0_dn10 = (((p.p1024 * locals.var_inv_l_dn10) + (p.p1027 * locals.var_inv_w_dn10)) + (p.p1030 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soilpe0_dn11 = (((p.p1024 * locals.var_inv_l_dn11) + (p.p1027 * locals.var_inv_w_dn11)) + (p.p1030 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soilpe0_dn12 = (((p.p1024 * locals.var_inv_l_dn12) + (p.p1027 * locals.var_inv_w_dn12)) + (p.p1030 * locals.var_inv_lw_dn12));

        let assign1550_e3028: f64 = (p.p502 * locals.var_inv_l);
        let assign1550_e3029: f64 = (p.p98 + assign1550_e3028);
        let assign1550_e3032: f64 = (p.p692 * locals.var_inv_w);
        let assign1550_e3033: f64 = (assign1550_e3029 + assign1550_e3032);
        let assign1550_e3036: f64 = (p.p882 * locals.var_inv_lw);
        let assign1550_e3037: f64 = (assign1550_e3033 + assign1550_e3036);
        locals.var_pparam_b4soilpeb = assign1550_e3037;
        locals.var_pparam_b4soilpeb_dn3 = (((p.p502 * locals.var_inv_l_dn3) + (p.p692 * locals.var_inv_w_dn3)) + (p.p882 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soilpeb_dn4 = (((p.p502 * locals.var_inv_l_dn4) + (p.p692 * locals.var_inv_w_dn4)) + (p.p882 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soilpeb_dn5 = (((p.p502 * locals.var_inv_l_dn5) + (p.p692 * locals.var_inv_w_dn5)) + (p.p882 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soilpeb_dn6 = (((p.p502 * locals.var_inv_l_dn6) + (p.p692 * locals.var_inv_w_dn6)) + (p.p882 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soilpeb_dn7 = (((p.p502 * locals.var_inv_l_dn7) + (p.p692 * locals.var_inv_w_dn7)) + (p.p882 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soilpeb_dn8 = (((p.p502 * locals.var_inv_l_dn8) + (p.p692 * locals.var_inv_w_dn8)) + (p.p882 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soilpeb_dn9 = (((p.p502 * locals.var_inv_l_dn9) + (p.p692 * locals.var_inv_w_dn9)) + (p.p882 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soilpeb_dn10 = (((p.p502 * locals.var_inv_l_dn10) + (p.p692 * locals.var_inv_w_dn10)) + (p.p882 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soilpeb_dn11 = (((p.p502 * locals.var_inv_l_dn11) + (p.p692 * locals.var_inv_w_dn11)) + (p.p882 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soilpeb_dn12 = (((p.p502 * locals.var_inv_l_dn12) + (p.p692 * locals.var_inv_w_dn12)) + (p.p882 * locals.var_inv_lw_dn12));

        let assign1560_e3041: f64 = (p.p503 * locals.var_inv_l);
        let assign1560_e3042: f64 = (p.p99 + assign1560_e3041);
        let assign1560_e3045: f64 = (p.p693 * locals.var_inv_w);
        let assign1560_e3046: f64 = (assign1560_e3042 + assign1560_e3045);
        let assign1560_e3049: f64 = (p.p883 * locals.var_inv_lw);
        let assign1560_e3050: f64 = (assign1560_e3046 + assign1560_e3049);
        locals.var_pparam_b4soidvt0 = assign1560_e3050;
        locals.var_pparam_b4soidvt0_dn3 = (((p.p503 * locals.var_inv_l_dn3) + (p.p693 * locals.var_inv_w_dn3)) + (p.p883 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidvt0_dn4 = (((p.p503 * locals.var_inv_l_dn4) + (p.p693 * locals.var_inv_w_dn4)) + (p.p883 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidvt0_dn5 = (((p.p503 * locals.var_inv_l_dn5) + (p.p693 * locals.var_inv_w_dn5)) + (p.p883 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidvt0_dn6 = (((p.p503 * locals.var_inv_l_dn6) + (p.p693 * locals.var_inv_w_dn6)) + (p.p883 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidvt0_dn7 = (((p.p503 * locals.var_inv_l_dn7) + (p.p693 * locals.var_inv_w_dn7)) + (p.p883 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidvt0_dn8 = (((p.p503 * locals.var_inv_l_dn8) + (p.p693 * locals.var_inv_w_dn8)) + (p.p883 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidvt0_dn9 = (((p.p503 * locals.var_inv_l_dn9) + (p.p693 * locals.var_inv_w_dn9)) + (p.p883 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidvt0_dn10 = (((p.p503 * locals.var_inv_l_dn10) + (p.p693 * locals.var_inv_w_dn10)) + (p.p883 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidvt0_dn11 = (((p.p503 * locals.var_inv_l_dn11) + (p.p693 * locals.var_inv_w_dn11)) + (p.p883 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidvt0_dn12 = (((p.p503 * locals.var_inv_l_dn12) + (p.p693 * locals.var_inv_w_dn12)) + (p.p883 * locals.var_inv_lw_dn12));

        let assign1570_e3054: f64 = (p.p504 * locals.var_inv_l);
        let assign1570_e3055: f64 = (p.p100 + assign1570_e3054);
        let assign1570_e3058: f64 = (p.p694 * locals.var_inv_w);
        let assign1570_e3059: f64 = (assign1570_e3055 + assign1570_e3058);
        let assign1570_e3062: f64 = (p.p884 * locals.var_inv_lw);
        let assign1570_e3063: f64 = (assign1570_e3059 + assign1570_e3062);
        locals.var_pparam_b4soidvt1 = assign1570_e3063;
        locals.var_pparam_b4soidvt1_dn3 = (((p.p504 * locals.var_inv_l_dn3) + (p.p694 * locals.var_inv_w_dn3)) + (p.p884 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidvt1_dn4 = (((p.p504 * locals.var_inv_l_dn4) + (p.p694 * locals.var_inv_w_dn4)) + (p.p884 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidvt1_dn5 = (((p.p504 * locals.var_inv_l_dn5) + (p.p694 * locals.var_inv_w_dn5)) + (p.p884 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidvt1_dn6 = (((p.p504 * locals.var_inv_l_dn6) + (p.p694 * locals.var_inv_w_dn6)) + (p.p884 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidvt1_dn7 = (((p.p504 * locals.var_inv_l_dn7) + (p.p694 * locals.var_inv_w_dn7)) + (p.p884 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidvt1_dn8 = (((p.p504 * locals.var_inv_l_dn8) + (p.p694 * locals.var_inv_w_dn8)) + (p.p884 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidvt1_dn9 = (((p.p504 * locals.var_inv_l_dn9) + (p.p694 * locals.var_inv_w_dn9)) + (p.p884 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidvt1_dn10 = (((p.p504 * locals.var_inv_l_dn10) + (p.p694 * locals.var_inv_w_dn10)) + (p.p884 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidvt1_dn11 = (((p.p504 * locals.var_inv_l_dn11) + (p.p694 * locals.var_inv_w_dn11)) + (p.p884 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidvt1_dn12 = (((p.p504 * locals.var_inv_l_dn12) + (p.p694 * locals.var_inv_w_dn12)) + (p.p884 * locals.var_inv_lw_dn12));

        let assign1580_e3067: f64 = (p.p505 * locals.var_inv_l);
        let assign1580_e3068: f64 = (p.p101 + assign1580_e3067);
        let assign1580_e3071: f64 = (p.p695 * locals.var_inv_w);
        let assign1580_e3072: f64 = (assign1580_e3068 + assign1580_e3071);
        let assign1580_e3075: f64 = (p.p885 * locals.var_inv_lw);
        let assign1580_e3076: f64 = (assign1580_e3072 + assign1580_e3075);
        locals.var_pparam_b4soidvt2 = assign1580_e3076;
        locals.var_pparam_b4soidvt2_dn3 = (((p.p505 * locals.var_inv_l_dn3) + (p.p695 * locals.var_inv_w_dn3)) + (p.p885 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidvt2_dn4 = (((p.p505 * locals.var_inv_l_dn4) + (p.p695 * locals.var_inv_w_dn4)) + (p.p885 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidvt2_dn5 = (((p.p505 * locals.var_inv_l_dn5) + (p.p695 * locals.var_inv_w_dn5)) + (p.p885 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidvt2_dn6 = (((p.p505 * locals.var_inv_l_dn6) + (p.p695 * locals.var_inv_w_dn6)) + (p.p885 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidvt2_dn7 = (((p.p505 * locals.var_inv_l_dn7) + (p.p695 * locals.var_inv_w_dn7)) + (p.p885 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidvt2_dn8 = (((p.p505 * locals.var_inv_l_dn8) + (p.p695 * locals.var_inv_w_dn8)) + (p.p885 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidvt2_dn9 = (((p.p505 * locals.var_inv_l_dn9) + (p.p695 * locals.var_inv_w_dn9)) + (p.p885 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidvt2_dn10 = (((p.p505 * locals.var_inv_l_dn10) + (p.p695 * locals.var_inv_w_dn10)) + (p.p885 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidvt2_dn11 = (((p.p505 * locals.var_inv_l_dn11) + (p.p695 * locals.var_inv_w_dn11)) + (p.p885 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidvt2_dn12 = (((p.p505 * locals.var_inv_l_dn12) + (p.p695 * locals.var_inv_w_dn12)) + (p.p885 * locals.var_inv_lw_dn12));

        let assign1590_e3080: f64 = (p.p506 * locals.var_inv_l);
        let assign1590_e3081: f64 = (p.p102 + assign1590_e3080);
        let assign1590_e3084: f64 = (p.p696 * locals.var_inv_w);
        let assign1590_e3085: f64 = (assign1590_e3081 + assign1590_e3084);
        let assign1590_e3088: f64 = (p.p886 * locals.var_inv_lw);
        let assign1590_e3089: f64 = (assign1590_e3085 + assign1590_e3088);
        locals.var_pparam_b4soidvt0w = assign1590_e3089;
        locals.var_pparam_b4soidvt0w_dn3 = (((p.p506 * locals.var_inv_l_dn3) + (p.p696 * locals.var_inv_w_dn3)) + (p.p886 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidvt0w_dn4 = (((p.p506 * locals.var_inv_l_dn4) + (p.p696 * locals.var_inv_w_dn4)) + (p.p886 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidvt0w_dn5 = (((p.p506 * locals.var_inv_l_dn5) + (p.p696 * locals.var_inv_w_dn5)) + (p.p886 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidvt0w_dn6 = (((p.p506 * locals.var_inv_l_dn6) + (p.p696 * locals.var_inv_w_dn6)) + (p.p886 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidvt0w_dn7 = (((p.p506 * locals.var_inv_l_dn7) + (p.p696 * locals.var_inv_w_dn7)) + (p.p886 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidvt0w_dn8 = (((p.p506 * locals.var_inv_l_dn8) + (p.p696 * locals.var_inv_w_dn8)) + (p.p886 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidvt0w_dn9 = (((p.p506 * locals.var_inv_l_dn9) + (p.p696 * locals.var_inv_w_dn9)) + (p.p886 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidvt0w_dn10 = (((p.p506 * locals.var_inv_l_dn10) + (p.p696 * locals.var_inv_w_dn10)) + (p.p886 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidvt0w_dn11 = (((p.p506 * locals.var_inv_l_dn11) + (p.p696 * locals.var_inv_w_dn11)) + (p.p886 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidvt0w_dn12 = (((p.p506 * locals.var_inv_l_dn12) + (p.p696 * locals.var_inv_w_dn12)) + (p.p886 * locals.var_inv_lw_dn12));

        let assign1600_e3093: f64 = (p.p507 * locals.var_inv_l);
        let assign1600_e3094: f64 = (p.p103 + assign1600_e3093);
        let assign1600_e3097: f64 = (p.p697 * locals.var_inv_w);
        let assign1600_e3098: f64 = (assign1600_e3094 + assign1600_e3097);
        let assign1600_e3101: f64 = (p.p887 * locals.var_inv_lw);
        let assign1600_e3102: f64 = (assign1600_e3098 + assign1600_e3101);
        locals.var_pparam_b4soidvt1w = assign1600_e3102;
        locals.var_pparam_b4soidvt1w_dn3 = (((p.p507 * locals.var_inv_l_dn3) + (p.p697 * locals.var_inv_w_dn3)) + (p.p887 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidvt1w_dn4 = (((p.p507 * locals.var_inv_l_dn4) + (p.p697 * locals.var_inv_w_dn4)) + (p.p887 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidvt1w_dn5 = (((p.p507 * locals.var_inv_l_dn5) + (p.p697 * locals.var_inv_w_dn5)) + (p.p887 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidvt1w_dn6 = (((p.p507 * locals.var_inv_l_dn6) + (p.p697 * locals.var_inv_w_dn6)) + (p.p887 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidvt1w_dn7 = (((p.p507 * locals.var_inv_l_dn7) + (p.p697 * locals.var_inv_w_dn7)) + (p.p887 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidvt1w_dn8 = (((p.p507 * locals.var_inv_l_dn8) + (p.p697 * locals.var_inv_w_dn8)) + (p.p887 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidvt1w_dn9 = (((p.p507 * locals.var_inv_l_dn9) + (p.p697 * locals.var_inv_w_dn9)) + (p.p887 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidvt1w_dn10 = (((p.p507 * locals.var_inv_l_dn10) + (p.p697 * locals.var_inv_w_dn10)) + (p.p887 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidvt1w_dn11 = (((p.p507 * locals.var_inv_l_dn11) + (p.p697 * locals.var_inv_w_dn11)) + (p.p887 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidvt1w_dn12 = (((p.p507 * locals.var_inv_l_dn12) + (p.p697 * locals.var_inv_w_dn12)) + (p.p887 * locals.var_inv_lw_dn12));

    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign1610_e3106: f64 = (p.p507 * locals.var_inv_l);
        let assign1610_e3107: f64 = (p.p103 + assign1610_e3106);
        let assign1610_e3110: f64 = (p.p697 * locals.var_inv_w);
        let assign1610_e3111: f64 = (assign1610_e3107 + assign1610_e3110);
        let assign1610_e3114: f64 = (p.p887 * locals.var_inv_lw);
        let assign1610_e3115: f64 = (assign1610_e3111 + assign1610_e3114);
        locals.var_pparam_b4soidvt1w = assign1610_e3115;
        locals.var_pparam_b4soidvt1w_dn3 = (((p.p507 * locals.var_inv_l_dn3) + (p.p697 * locals.var_inv_w_dn3)) + (p.p887 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidvt1w_dn4 = (((p.p507 * locals.var_inv_l_dn4) + (p.p697 * locals.var_inv_w_dn4)) + (p.p887 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidvt1w_dn5 = (((p.p507 * locals.var_inv_l_dn5) + (p.p697 * locals.var_inv_w_dn5)) + (p.p887 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidvt1w_dn6 = (((p.p507 * locals.var_inv_l_dn6) + (p.p697 * locals.var_inv_w_dn6)) + (p.p887 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidvt1w_dn7 = (((p.p507 * locals.var_inv_l_dn7) + (p.p697 * locals.var_inv_w_dn7)) + (p.p887 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidvt1w_dn8 = (((p.p507 * locals.var_inv_l_dn8) + (p.p697 * locals.var_inv_w_dn8)) + (p.p887 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidvt1w_dn9 = (((p.p507 * locals.var_inv_l_dn9) + (p.p697 * locals.var_inv_w_dn9)) + (p.p887 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidvt1w_dn10 = (((p.p507 * locals.var_inv_l_dn10) + (p.p697 * locals.var_inv_w_dn10)) + (p.p887 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidvt1w_dn11 = (((p.p507 * locals.var_inv_l_dn11) + (p.p697 * locals.var_inv_w_dn11)) + (p.p887 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidvt1w_dn12 = (((p.p507 * locals.var_inv_l_dn12) + (p.p697 * locals.var_inv_w_dn12)) + (p.p887 * locals.var_inv_lw_dn12));

        let assign1620_e3119: f64 = (p.p508 * locals.var_inv_l);
        let assign1620_e3120: f64 = (p.p104 + assign1620_e3119);
        let assign1620_e3123: f64 = (p.p698 * locals.var_inv_w);
        let assign1620_e3124: f64 = (assign1620_e3120 + assign1620_e3123);
        let assign1620_e3127: f64 = (p.p888 * locals.var_inv_lw);
        let assign1620_e3128: f64 = (assign1620_e3124 + assign1620_e3127);
        locals.var_pparam_b4soidvt2w = assign1620_e3128;
        locals.var_pparam_b4soidvt2w_dn3 = (((p.p508 * locals.var_inv_l_dn3) + (p.p698 * locals.var_inv_w_dn3)) + (p.p888 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidvt2w_dn4 = (((p.p508 * locals.var_inv_l_dn4) + (p.p698 * locals.var_inv_w_dn4)) + (p.p888 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidvt2w_dn5 = (((p.p508 * locals.var_inv_l_dn5) + (p.p698 * locals.var_inv_w_dn5)) + (p.p888 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidvt2w_dn6 = (((p.p508 * locals.var_inv_l_dn6) + (p.p698 * locals.var_inv_w_dn6)) + (p.p888 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidvt2w_dn7 = (((p.p508 * locals.var_inv_l_dn7) + (p.p698 * locals.var_inv_w_dn7)) + (p.p888 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidvt2w_dn8 = (((p.p508 * locals.var_inv_l_dn8) + (p.p698 * locals.var_inv_w_dn8)) + (p.p888 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidvt2w_dn9 = (((p.p508 * locals.var_inv_l_dn9) + (p.p698 * locals.var_inv_w_dn9)) + (p.p888 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidvt2w_dn10 = (((p.p508 * locals.var_inv_l_dn10) + (p.p698 * locals.var_inv_w_dn10)) + (p.p888 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidvt2w_dn11 = (((p.p508 * locals.var_inv_l_dn11) + (p.p698 * locals.var_inv_w_dn11)) + (p.p888 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidvt2w_dn12 = (((p.p508 * locals.var_inv_l_dn12) + (p.p698 * locals.var_inv_w_dn12)) + (p.p888 * locals.var_inv_lw_dn12));

        let assign1630_e3132: f64 = (p.p509 * locals.var_inv_l);
        let assign1630_e3133: f64 = (p.p116 + assign1630_e3132);
        let assign1630_e3136: f64 = (p.p699 * locals.var_inv_w);
        let assign1630_e3137: f64 = (assign1630_e3133 + assign1630_e3136);
        let assign1630_e3140: f64 = (p.p889 * locals.var_inv_lw);
        let assign1630_e3141: f64 = (assign1630_e3137 + assign1630_e3140);
        locals.var_pparam_b4soiu0 = assign1630_e3141;
        locals.var_pparam_b4soiu0_dn3 = (((p.p509 * locals.var_inv_l_dn3) + (p.p699 * locals.var_inv_w_dn3)) + (p.p889 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiu0_dn4 = (((p.p509 * locals.var_inv_l_dn4) + (p.p699 * locals.var_inv_w_dn4)) + (p.p889 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiu0_dn5 = (((p.p509 * locals.var_inv_l_dn5) + (p.p699 * locals.var_inv_w_dn5)) + (p.p889 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiu0_dn6 = (((p.p509 * locals.var_inv_l_dn6) + (p.p699 * locals.var_inv_w_dn6)) + (p.p889 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiu0_dn7 = (((p.p509 * locals.var_inv_l_dn7) + (p.p699 * locals.var_inv_w_dn7)) + (p.p889 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiu0_dn8 = (((p.p509 * locals.var_inv_l_dn8) + (p.p699 * locals.var_inv_w_dn8)) + (p.p889 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiu0_dn9 = (((p.p509 * locals.var_inv_l_dn9) + (p.p699 * locals.var_inv_w_dn9)) + (p.p889 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiu0_dn10 = (((p.p509 * locals.var_inv_l_dn10) + (p.p699 * locals.var_inv_w_dn10)) + (p.p889 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiu0_dn11 = (((p.p509 * locals.var_inv_l_dn11) + (p.p699 * locals.var_inv_w_dn11)) + (p.p889 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiu0_dn12 = (((p.p509 * locals.var_inv_l_dn12) + (p.p699 * locals.var_inv_w_dn12)) + (p.p889 * locals.var_inv_lw_dn12));

        let assign1640_e3145: f64 = (p.p511 * locals.var_inv_l);
        let assign1640_e3146: f64 = (p.p110 + assign1640_e3145);
        let assign1640_e3149: f64 = (p.p701 * locals.var_inv_w);
        let assign1640_e3150: f64 = (assign1640_e3146 + assign1640_e3149);
        let assign1640_e3153: f64 = (p.p891 * locals.var_inv_lw);
        let assign1640_e3154: f64 = (assign1640_e3150 + assign1640_e3153);
        locals.var_pparam_b4soiua = assign1640_e3154;
        locals.var_pparam_b4soiua_dn3 = (((p.p511 * locals.var_inv_l_dn3) + (p.p701 * locals.var_inv_w_dn3)) + (p.p891 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiua_dn4 = (((p.p511 * locals.var_inv_l_dn4) + (p.p701 * locals.var_inv_w_dn4)) + (p.p891 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiua_dn5 = (((p.p511 * locals.var_inv_l_dn5) + (p.p701 * locals.var_inv_w_dn5)) + (p.p891 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiua_dn6 = (((p.p511 * locals.var_inv_l_dn6) + (p.p701 * locals.var_inv_w_dn6)) + (p.p891 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiua_dn7 = (((p.p511 * locals.var_inv_l_dn7) + (p.p701 * locals.var_inv_w_dn7)) + (p.p891 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiua_dn8 = (((p.p511 * locals.var_inv_l_dn8) + (p.p701 * locals.var_inv_w_dn8)) + (p.p891 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiua_dn9 = (((p.p511 * locals.var_inv_l_dn9) + (p.p701 * locals.var_inv_w_dn9)) + (p.p891 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiua_dn10 = (((p.p511 * locals.var_inv_l_dn10) + (p.p701 * locals.var_inv_w_dn10)) + (p.p891 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiua_dn11 = (((p.p511 * locals.var_inv_l_dn11) + (p.p701 * locals.var_inv_w_dn11)) + (p.p891 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiua_dn12 = (((p.p511 * locals.var_inv_l_dn12) + (p.p701 * locals.var_inv_w_dn12)) + (p.p891 * locals.var_inv_lw_dn12));

        let assign1650_e3158: f64 = (p.p512 * locals.var_inv_l);
        let assign1650_e3159: f64 = (p.p112 + assign1650_e3158);
        let assign1650_e3162: f64 = (p.p702 * locals.var_inv_w);
        let assign1650_e3163: f64 = (assign1650_e3159 + assign1650_e3162);
        let assign1650_e3166: f64 = (p.p892 * locals.var_inv_lw);
        let assign1650_e3167: f64 = (assign1650_e3163 + assign1650_e3166);
        locals.var_pparam_b4soiub = assign1650_e3167;
        locals.var_pparam_b4soiub_dn3 = (((p.p512 * locals.var_inv_l_dn3) + (p.p702 * locals.var_inv_w_dn3)) + (p.p892 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiub_dn4 = (((p.p512 * locals.var_inv_l_dn4) + (p.p702 * locals.var_inv_w_dn4)) + (p.p892 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiub_dn5 = (((p.p512 * locals.var_inv_l_dn5) + (p.p702 * locals.var_inv_w_dn5)) + (p.p892 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiub_dn6 = (((p.p512 * locals.var_inv_l_dn6) + (p.p702 * locals.var_inv_w_dn6)) + (p.p892 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiub_dn7 = (((p.p512 * locals.var_inv_l_dn7) + (p.p702 * locals.var_inv_w_dn7)) + (p.p892 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiub_dn8 = (((p.p512 * locals.var_inv_l_dn8) + (p.p702 * locals.var_inv_w_dn8)) + (p.p892 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiub_dn9 = (((p.p512 * locals.var_inv_l_dn9) + (p.p702 * locals.var_inv_w_dn9)) + (p.p892 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiub_dn10 = (((p.p512 * locals.var_inv_l_dn10) + (p.p702 * locals.var_inv_w_dn10)) + (p.p892 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiub_dn11 = (((p.p512 * locals.var_inv_l_dn11) + (p.p702 * locals.var_inv_w_dn11)) + (p.p892 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiub_dn12 = (((p.p512 * locals.var_inv_l_dn12) + (p.p702 * locals.var_inv_w_dn12)) + (p.p892 * locals.var_inv_lw_dn12));

        let assign1660_e3171: f64 = (p.p513 * locals.var_inv_l);
        let assign1660_e3172: f64 = (p.p114 + assign1660_e3171);
        let assign1660_e3175: f64 = (p.p703 * locals.var_inv_w);
        let assign1660_e3176: f64 = (assign1660_e3172 + assign1660_e3175);
        let assign1660_e3179: f64 = (p.p893 * locals.var_inv_lw);
        let assign1660_e3180: f64 = (assign1660_e3176 + assign1660_e3179);
        locals.var_pparam_b4soiuc = assign1660_e3180;
        locals.var_pparam_b4soiuc_dn3 = (((p.p513 * locals.var_inv_l_dn3) + (p.p703 * locals.var_inv_w_dn3)) + (p.p893 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiuc_dn4 = (((p.p513 * locals.var_inv_l_dn4) + (p.p703 * locals.var_inv_w_dn4)) + (p.p893 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiuc_dn5 = (((p.p513 * locals.var_inv_l_dn5) + (p.p703 * locals.var_inv_w_dn5)) + (p.p893 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiuc_dn6 = (((p.p513 * locals.var_inv_l_dn6) + (p.p703 * locals.var_inv_w_dn6)) + (p.p893 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiuc_dn7 = (((p.p513 * locals.var_inv_l_dn7) + (p.p703 * locals.var_inv_w_dn7)) + (p.p893 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiuc_dn8 = (((p.p513 * locals.var_inv_l_dn8) + (p.p703 * locals.var_inv_w_dn8)) + (p.p893 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiuc_dn9 = (((p.p513 * locals.var_inv_l_dn9) + (p.p703 * locals.var_inv_w_dn9)) + (p.p893 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiuc_dn10 = (((p.p513 * locals.var_inv_l_dn10) + (p.p703 * locals.var_inv_w_dn10)) + (p.p893 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiuc_dn11 = (((p.p513 * locals.var_inv_l_dn11) + (p.p703 * locals.var_inv_w_dn11)) + (p.p893 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiuc_dn12 = (((p.p513 * locals.var_inv_l_dn12) + (p.p703 * locals.var_inv_w_dn12)) + (p.p893 * locals.var_inv_lw_dn12));

        let assign1670_e3184: f64 = (p.p518 * locals.var_inv_l);
        let assign1670_e3185: f64 = (p.p74 + assign1670_e3184);
        let assign1670_e3188: f64 = (p.p708 * locals.var_inv_w);
        let assign1670_e3189: f64 = (assign1670_e3185 + assign1670_e3188);
        let assign1670_e3192: f64 = (p.p898 * locals.var_inv_lw);
        let assign1670_e3193: f64 = (assign1670_e3189 + assign1670_e3192);
        locals.var_pparam_b4soivsat = assign1670_e3193;
        locals.var_pparam_b4soivsat_dn3 = (((p.p518 * locals.var_inv_l_dn3) + (p.p708 * locals.var_inv_w_dn3)) + (p.p898 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivsat_dn4 = (((p.p518 * locals.var_inv_l_dn4) + (p.p708 * locals.var_inv_w_dn4)) + (p.p898 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivsat_dn5 = (((p.p518 * locals.var_inv_l_dn5) + (p.p708 * locals.var_inv_w_dn5)) + (p.p898 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivsat_dn6 = (((p.p518 * locals.var_inv_l_dn6) + (p.p708 * locals.var_inv_w_dn6)) + (p.p898 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivsat_dn7 = (((p.p518 * locals.var_inv_l_dn7) + (p.p708 * locals.var_inv_w_dn7)) + (p.p898 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivsat_dn8 = (((p.p518 * locals.var_inv_l_dn8) + (p.p708 * locals.var_inv_w_dn8)) + (p.p898 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivsat_dn9 = (((p.p518 * locals.var_inv_l_dn9) + (p.p708 * locals.var_inv_w_dn9)) + (p.p898 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivsat_dn10 = (((p.p518 * locals.var_inv_l_dn10) + (p.p708 * locals.var_inv_w_dn10)) + (p.p898 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivsat_dn11 = (((p.p518 * locals.var_inv_l_dn11) + (p.p708 * locals.var_inv_w_dn11)) + (p.p898 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivsat_dn12 = (((p.p518 * locals.var_inv_l_dn12) + (p.p708 * locals.var_inv_w_dn12)) + (p.p898 * locals.var_inv_lw_dn12));

        let assign1680_e3197: f64 = (p.p519 * locals.var_inv_l);
        let assign1680_e3198: f64 = (p.p76 + assign1680_e3197);
        let assign1680_e3201: f64 = (p.p709 * locals.var_inv_w);
        let assign1680_e3202: f64 = (assign1680_e3198 + assign1680_e3201);
        let assign1680_e3205: f64 = (p.p899 * locals.var_inv_lw);
        let assign1680_e3206: f64 = (assign1680_e3202 + assign1680_e3205);
        locals.var_pparam_b4soia0 = assign1680_e3206;
        locals.var_pparam_b4soia0_dn3 = (((p.p519 * locals.var_inv_l_dn3) + (p.p709 * locals.var_inv_w_dn3)) + (p.p899 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soia0_dn4 = (((p.p519 * locals.var_inv_l_dn4) + (p.p709 * locals.var_inv_w_dn4)) + (p.p899 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soia0_dn5 = (((p.p519 * locals.var_inv_l_dn5) + (p.p709 * locals.var_inv_w_dn5)) + (p.p899 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soia0_dn6 = (((p.p519 * locals.var_inv_l_dn6) + (p.p709 * locals.var_inv_w_dn6)) + (p.p899 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soia0_dn7 = (((p.p519 * locals.var_inv_l_dn7) + (p.p709 * locals.var_inv_w_dn7)) + (p.p899 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soia0_dn8 = (((p.p519 * locals.var_inv_l_dn8) + (p.p709 * locals.var_inv_w_dn8)) + (p.p899 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soia0_dn9 = (((p.p519 * locals.var_inv_l_dn9) + (p.p709 * locals.var_inv_w_dn9)) + (p.p899 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soia0_dn10 = (((p.p519 * locals.var_inv_l_dn10) + (p.p709 * locals.var_inv_w_dn10)) + (p.p899 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soia0_dn11 = (((p.p519 * locals.var_inv_l_dn11) + (p.p709 * locals.var_inv_w_dn11)) + (p.p899 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soia0_dn12 = (((p.p519 * locals.var_inv_l_dn12) + (p.p709 * locals.var_inv_w_dn12)) + (p.p899 * locals.var_inv_lw_dn12));

        let assign1690_e3210: f64 = (p.p520 * locals.var_inv_l);
        let assign1690_e3211: f64 = (p.p77 + assign1690_e3210);
        let assign1690_e3214: f64 = (p.p710 * locals.var_inv_w);
        let assign1690_e3215: f64 = (assign1690_e3211 + assign1690_e3214);
        let assign1690_e3218: f64 = (p.p900 * locals.var_inv_lw);
        let assign1690_e3219: f64 = (assign1690_e3215 + assign1690_e3218);
        locals.var_pparam_b4soiags = assign1690_e3219;
        locals.var_pparam_b4soiags_dn3 = (((p.p520 * locals.var_inv_l_dn3) + (p.p710 * locals.var_inv_w_dn3)) + (p.p900 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiags_dn4 = (((p.p520 * locals.var_inv_l_dn4) + (p.p710 * locals.var_inv_w_dn4)) + (p.p900 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiags_dn5 = (((p.p520 * locals.var_inv_l_dn5) + (p.p710 * locals.var_inv_w_dn5)) + (p.p900 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiags_dn6 = (((p.p520 * locals.var_inv_l_dn6) + (p.p710 * locals.var_inv_w_dn6)) + (p.p900 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiags_dn7 = (((p.p520 * locals.var_inv_l_dn7) + (p.p710 * locals.var_inv_w_dn7)) + (p.p900 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiags_dn8 = (((p.p520 * locals.var_inv_l_dn8) + (p.p710 * locals.var_inv_w_dn8)) + (p.p900 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiags_dn9 = (((p.p520 * locals.var_inv_l_dn9) + (p.p710 * locals.var_inv_w_dn9)) + (p.p900 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiags_dn10 = (((p.p520 * locals.var_inv_l_dn10) + (p.p710 * locals.var_inv_w_dn10)) + (p.p900 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiags_dn11 = (((p.p520 * locals.var_inv_l_dn11) + (p.p710 * locals.var_inv_w_dn11)) + (p.p900 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiags_dn12 = (((p.p520 * locals.var_inv_l_dn12) + (p.p710 * locals.var_inv_w_dn12)) + (p.p900 * locals.var_inv_lw_dn12));

        let assign1700_e3223: f64 = (p.p521 * locals.var_inv_l);
        let assign1700_e3224: f64 = (p.p208 + assign1700_e3223);
        let assign1700_e3227: f64 = (p.p711 * locals.var_inv_w);
        let assign1700_e3228: f64 = (assign1700_e3224 + assign1700_e3227);
        let assign1700_e3231: f64 = (p.p901 * locals.var_inv_lw);
        let assign1700_e3232: f64 = (assign1700_e3228 + assign1700_e3231);
        locals.var_pparam_b4soib0 = assign1700_e3232;
        locals.var_pparam_b4soib0_dn3 = (((p.p521 * locals.var_inv_l_dn3) + (p.p711 * locals.var_inv_w_dn3)) + (p.p901 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soib0_dn4 = (((p.p521 * locals.var_inv_l_dn4) + (p.p711 * locals.var_inv_w_dn4)) + (p.p901 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soib0_dn5 = (((p.p521 * locals.var_inv_l_dn5) + (p.p711 * locals.var_inv_w_dn5)) + (p.p901 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soib0_dn6 = (((p.p521 * locals.var_inv_l_dn6) + (p.p711 * locals.var_inv_w_dn6)) + (p.p901 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soib0_dn7 = (((p.p521 * locals.var_inv_l_dn7) + (p.p711 * locals.var_inv_w_dn7)) + (p.p901 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soib0_dn8 = (((p.p521 * locals.var_inv_l_dn8) + (p.p711 * locals.var_inv_w_dn8)) + (p.p901 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soib0_dn9 = (((p.p521 * locals.var_inv_l_dn9) + (p.p711 * locals.var_inv_w_dn9)) + (p.p901 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soib0_dn10 = (((p.p521 * locals.var_inv_l_dn10) + (p.p711 * locals.var_inv_w_dn10)) + (p.p901 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soib0_dn11 = (((p.p521 * locals.var_inv_l_dn11) + (p.p711 * locals.var_inv_w_dn11)) + (p.p901 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soib0_dn12 = (((p.p521 * locals.var_inv_l_dn12) + (p.p711 * locals.var_inv_w_dn12)) + (p.p901 * locals.var_inv_lw_dn12));

        let assign1710_e3236: f64 = (p.p522 * locals.var_inv_l);
        let assign1710_e3237: f64 = (p.p209 + assign1710_e3236);
        let assign1710_e3240: f64 = (p.p712 * locals.var_inv_w);
        let assign1710_e3241: f64 = (assign1710_e3237 + assign1710_e3240);
        let assign1710_e3244: f64 = (p.p902 * locals.var_inv_lw);
        let assign1710_e3245: f64 = (assign1710_e3241 + assign1710_e3244);
        locals.var_pparam_b4soib1 = assign1710_e3245;
        locals.var_pparam_b4soib1_dn3 = (((p.p522 * locals.var_inv_l_dn3) + (p.p712 * locals.var_inv_w_dn3)) + (p.p902 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soib1_dn4 = (((p.p522 * locals.var_inv_l_dn4) + (p.p712 * locals.var_inv_w_dn4)) + (p.p902 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soib1_dn5 = (((p.p522 * locals.var_inv_l_dn5) + (p.p712 * locals.var_inv_w_dn5)) + (p.p902 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soib1_dn6 = (((p.p522 * locals.var_inv_l_dn6) + (p.p712 * locals.var_inv_w_dn6)) + (p.p902 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soib1_dn7 = (((p.p522 * locals.var_inv_l_dn7) + (p.p712 * locals.var_inv_w_dn7)) + (p.p902 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soib1_dn8 = (((p.p522 * locals.var_inv_l_dn8) + (p.p712 * locals.var_inv_w_dn8)) + (p.p902 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soib1_dn9 = (((p.p522 * locals.var_inv_l_dn9) + (p.p712 * locals.var_inv_w_dn9)) + (p.p902 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soib1_dn10 = (((p.p522 * locals.var_inv_l_dn10) + (p.p712 * locals.var_inv_w_dn10)) + (p.p902 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soib1_dn11 = (((p.p522 * locals.var_inv_l_dn11) + (p.p712 * locals.var_inv_w_dn11)) + (p.p902 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soib1_dn12 = (((p.p522 * locals.var_inv_l_dn12) + (p.p712 * locals.var_inv_w_dn12)) + (p.p902 * locals.var_inv_lw_dn12));

        let assign1720_e3249: f64 = (p.p523 * locals.var_inv_l);
        let assign1720_e3250: f64 = (p.p80 + assign1720_e3249);
        let assign1720_e3253: f64 = (p.p713 * locals.var_inv_w);
        let assign1720_e3254: f64 = (assign1720_e3250 + assign1720_e3253);
        let assign1720_e3257: f64 = (p.p903 * locals.var_inv_lw);
        let assign1720_e3258: f64 = (assign1720_e3254 + assign1720_e3257);
        locals.var_pparam_b4soiketa = assign1720_e3258;
        locals.var_pparam_b4soiketa_dn3 = (((p.p523 * locals.var_inv_l_dn3) + (p.p713 * locals.var_inv_w_dn3)) + (p.p903 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiketa_dn4 = (((p.p523 * locals.var_inv_l_dn4) + (p.p713 * locals.var_inv_w_dn4)) + (p.p903 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiketa_dn5 = (((p.p523 * locals.var_inv_l_dn5) + (p.p713 * locals.var_inv_w_dn5)) + (p.p903 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiketa_dn6 = (((p.p523 * locals.var_inv_l_dn6) + (p.p713 * locals.var_inv_w_dn6)) + (p.p903 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiketa_dn7 = (((p.p523 * locals.var_inv_l_dn7) + (p.p713 * locals.var_inv_w_dn7)) + (p.p903 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiketa_dn8 = (((p.p523 * locals.var_inv_l_dn8) + (p.p713 * locals.var_inv_w_dn8)) + (p.p903 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiketa_dn9 = (((p.p523 * locals.var_inv_l_dn9) + (p.p713 * locals.var_inv_w_dn9)) + (p.p903 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiketa_dn10 = (((p.p523 * locals.var_inv_l_dn10) + (p.p713 * locals.var_inv_w_dn10)) + (p.p903 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiketa_dn11 = (((p.p523 * locals.var_inv_l_dn11) + (p.p713 * locals.var_inv_w_dn11)) + (p.p903 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiketa_dn12 = (((p.p523 * locals.var_inv_l_dn12) + (p.p713 * locals.var_inv_w_dn12)) + (p.p903 * locals.var_inv_lw_dn12));

        let assign1730_e3262: f64 = (p.p524 * locals.var_inv_l);
        let assign1730_e3263: f64 = (p.p302 + assign1730_e3262);
        let assign1730_e3266: f64 = (p.p714 * locals.var_inv_w);
        let assign1730_e3267: f64 = (assign1730_e3263 + assign1730_e3266);
        let assign1730_e3270: f64 = (p.p904 * locals.var_inv_lw);
        let assign1730_e3271: f64 = (assign1730_e3267 + assign1730_e3270);
        locals.var_pparam_b4soiketas = assign1730_e3271;
        locals.var_pparam_b4soiketas_dn3 = (((p.p524 * locals.var_inv_l_dn3) + (p.p714 * locals.var_inv_w_dn3)) + (p.p904 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiketas_dn4 = (((p.p524 * locals.var_inv_l_dn4) + (p.p714 * locals.var_inv_w_dn4)) + (p.p904 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiketas_dn5 = (((p.p524 * locals.var_inv_l_dn5) + (p.p714 * locals.var_inv_w_dn5)) + (p.p904 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiketas_dn6 = (((p.p524 * locals.var_inv_l_dn6) + (p.p714 * locals.var_inv_w_dn6)) + (p.p904 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiketas_dn7 = (((p.p524 * locals.var_inv_l_dn7) + (p.p714 * locals.var_inv_w_dn7)) + (p.p904 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiketas_dn8 = (((p.p524 * locals.var_inv_l_dn8) + (p.p714 * locals.var_inv_w_dn8)) + (p.p904 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiketas_dn9 = (((p.p524 * locals.var_inv_l_dn9) + (p.p714 * locals.var_inv_w_dn9)) + (p.p904 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiketas_dn10 = (((p.p524 * locals.var_inv_l_dn10) + (p.p714 * locals.var_inv_w_dn10)) + (p.p904 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiketas_dn11 = (((p.p524 * locals.var_inv_l_dn11) + (p.p714 * locals.var_inv_w_dn11)) + (p.p904 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiketas_dn12 = (((p.p524 * locals.var_inv_l_dn12) + (p.p714 * locals.var_inv_w_dn12)) + (p.p904 * locals.var_inv_lw_dn12));

        let assign1740_e3275: f64 = (p.p525 * locals.var_inv_l);
        let assign1740_e3276: f64 = (p.p78 + assign1740_e3275);
        let assign1740_e3279: f64 = (p.p715 * locals.var_inv_w);
        let assign1740_e3280: f64 = (assign1740_e3276 + assign1740_e3279);
        let assign1740_e3283: f64 = (p.p905 * locals.var_inv_lw);
        let assign1740_e3284: f64 = (assign1740_e3280 + assign1740_e3283);
        locals.var_pparam_b4soia1 = assign1740_e3284;
        locals.var_pparam_b4soia1_dn3 = (((p.p525 * locals.var_inv_l_dn3) + (p.p715 * locals.var_inv_w_dn3)) + (p.p905 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soia1_dn4 = (((p.p525 * locals.var_inv_l_dn4) + (p.p715 * locals.var_inv_w_dn4)) + (p.p905 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soia1_dn5 = (((p.p525 * locals.var_inv_l_dn5) + (p.p715 * locals.var_inv_w_dn5)) + (p.p905 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soia1_dn6 = (((p.p525 * locals.var_inv_l_dn6) + (p.p715 * locals.var_inv_w_dn6)) + (p.p905 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soia1_dn7 = (((p.p525 * locals.var_inv_l_dn7) + (p.p715 * locals.var_inv_w_dn7)) + (p.p905 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soia1_dn8 = (((p.p525 * locals.var_inv_l_dn8) + (p.p715 * locals.var_inv_w_dn8)) + (p.p905 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soia1_dn9 = (((p.p525 * locals.var_inv_l_dn9) + (p.p715 * locals.var_inv_w_dn9)) + (p.p905 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soia1_dn10 = (((p.p525 * locals.var_inv_l_dn10) + (p.p715 * locals.var_inv_w_dn10)) + (p.p905 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soia1_dn11 = (((p.p525 * locals.var_inv_l_dn11) + (p.p715 * locals.var_inv_w_dn11)) + (p.p905 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soia1_dn12 = (((p.p525 * locals.var_inv_l_dn12) + (p.p715 * locals.var_inv_w_dn12)) + (p.p905 * locals.var_inv_lw_dn12));

        let assign1750_e3288: f64 = (p.p526 * locals.var_inv_l);
        let assign1750_e3289: f64 = (p.p79 + assign1750_e3288);
        let assign1750_e3292: f64 = (p.p716 * locals.var_inv_w);
        let assign1750_e3293: f64 = (assign1750_e3289 + assign1750_e3292);
        let assign1750_e3296: f64 = (p.p906 * locals.var_inv_lw);
        let assign1750_e3297: f64 = (assign1750_e3293 + assign1750_e3296);
        locals.var_pparam_b4soia2 = assign1750_e3297;
        locals.var_pparam_b4soia2_dn3 = (((p.p526 * locals.var_inv_l_dn3) + (p.p716 * locals.var_inv_w_dn3)) + (p.p906 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soia2_dn4 = (((p.p526 * locals.var_inv_l_dn4) + (p.p716 * locals.var_inv_w_dn4)) + (p.p906 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soia2_dn5 = (((p.p526 * locals.var_inv_l_dn5) + (p.p716 * locals.var_inv_w_dn5)) + (p.p906 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soia2_dn6 = (((p.p526 * locals.var_inv_l_dn6) + (p.p716 * locals.var_inv_w_dn6)) + (p.p906 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soia2_dn7 = (((p.p526 * locals.var_inv_l_dn7) + (p.p716 * locals.var_inv_w_dn7)) + (p.p906 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soia2_dn8 = (((p.p526 * locals.var_inv_l_dn8) + (p.p716 * locals.var_inv_w_dn8)) + (p.p906 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soia2_dn9 = (((p.p526 * locals.var_inv_l_dn9) + (p.p716 * locals.var_inv_w_dn9)) + (p.p906 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soia2_dn10 = (((p.p526 * locals.var_inv_l_dn10) + (p.p716 * locals.var_inv_w_dn10)) + (p.p906 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soia2_dn11 = (((p.p526 * locals.var_inv_l_dn11) + (p.p716 * locals.var_inv_w_dn11)) + (p.p906 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soia2_dn12 = (((p.p526 * locals.var_inv_l_dn12) + (p.p716 * locals.var_inv_w_dn12)) + (p.p906 * locals.var_inv_lw_dn12));

        let assign1760_e3301: f64 = (p.p527 * locals.var_inv_l);
        let assign1760_e3302: f64 = (p.p132 + assign1760_e3301);
        let assign1760_e3305: f64 = (p.p717 * locals.var_inv_w);
        let assign1760_e3306: f64 = (assign1760_e3302 + assign1760_e3305);
        let assign1760_e3309: f64 = (p.p907 * locals.var_inv_lw);
        let assign1760_e3310: f64 = (assign1760_e3306 + assign1760_e3309);
        locals.var_pparam_b4soirdsw = assign1760_e3310;
        locals.var_pparam_b4soirdsw_dn3 = (((p.p527 * locals.var_inv_l_dn3) + (p.p717 * locals.var_inv_w_dn3)) + (p.p907 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soirdsw_dn4 = (((p.p527 * locals.var_inv_l_dn4) + (p.p717 * locals.var_inv_w_dn4)) + (p.p907 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soirdsw_dn5 = (((p.p527 * locals.var_inv_l_dn5) + (p.p717 * locals.var_inv_w_dn5)) + (p.p907 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soirdsw_dn6 = (((p.p527 * locals.var_inv_l_dn6) + (p.p717 * locals.var_inv_w_dn6)) + (p.p907 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soirdsw_dn7 = (((p.p527 * locals.var_inv_l_dn7) + (p.p717 * locals.var_inv_w_dn7)) + (p.p907 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soirdsw_dn8 = (((p.p527 * locals.var_inv_l_dn8) + (p.p717 * locals.var_inv_w_dn8)) + (p.p907 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soirdsw_dn9 = (((p.p527 * locals.var_inv_l_dn9) + (p.p717 * locals.var_inv_w_dn9)) + (p.p907 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soirdsw_dn10 = (((p.p527 * locals.var_inv_l_dn10) + (p.p717 * locals.var_inv_w_dn10)) + (p.p907 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soirdsw_dn11 = (((p.p527 * locals.var_inv_l_dn11) + (p.p717 * locals.var_inv_w_dn11)) + (p.p907 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soirdsw_dn12 = (((p.p527 * locals.var_inv_l_dn12) + (p.p717 * locals.var_inv_w_dn12)) + (p.p907 * locals.var_inv_lw_dn12));

        let assign1770_e3314: f64 = (p.p528 * locals.var_inv_l);
        let assign1770_e3315: f64 = (p.p133 + assign1770_e3314);
        let assign1770_e3318: f64 = (p.p718 * locals.var_inv_w);
        let assign1770_e3319: f64 = (assign1770_e3315 + assign1770_e3318);
        let assign1770_e3322: f64 = (p.p908 * locals.var_inv_lw);
        let assign1770_e3323: f64 = (assign1770_e3319 + assign1770_e3322);
        locals.var_pparam_b4soirsw = assign1770_e3323;
        locals.var_pparam_b4soirsw_dn3 = (((p.p528 * locals.var_inv_l_dn3) + (p.p718 * locals.var_inv_w_dn3)) + (p.p908 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soirsw_dn4 = (((p.p528 * locals.var_inv_l_dn4) + (p.p718 * locals.var_inv_w_dn4)) + (p.p908 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soirsw_dn5 = (((p.p528 * locals.var_inv_l_dn5) + (p.p718 * locals.var_inv_w_dn5)) + (p.p908 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soirsw_dn6 = (((p.p528 * locals.var_inv_l_dn6) + (p.p718 * locals.var_inv_w_dn6)) + (p.p908 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soirsw_dn7 = (((p.p528 * locals.var_inv_l_dn7) + (p.p718 * locals.var_inv_w_dn7)) + (p.p908 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soirsw_dn8 = (((p.p528 * locals.var_inv_l_dn8) + (p.p718 * locals.var_inv_w_dn8)) + (p.p908 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soirsw_dn9 = (((p.p528 * locals.var_inv_l_dn9) + (p.p718 * locals.var_inv_w_dn9)) + (p.p908 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soirsw_dn10 = (((p.p528 * locals.var_inv_l_dn10) + (p.p718 * locals.var_inv_w_dn10)) + (p.p908 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soirsw_dn11 = (((p.p528 * locals.var_inv_l_dn11) + (p.p718 * locals.var_inv_w_dn11)) + (p.p908 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soirsw_dn12 = (((p.p528 * locals.var_inv_l_dn12) + (p.p718 * locals.var_inv_w_dn12)) + (p.p908 * locals.var_inv_lw_dn12));

        let assign1780_e3327: f64 = (p.p529 * locals.var_inv_l);
        let assign1780_e3328: f64 = (p.p134 + assign1780_e3327);
        let assign1780_e3331: f64 = (p.p719 * locals.var_inv_w);
        let assign1780_e3332: f64 = (assign1780_e3328 + assign1780_e3331);
        let assign1780_e3335: f64 = (p.p909 * locals.var_inv_lw);
        let assign1780_e3336: f64 = (assign1780_e3332 + assign1780_e3335);
        locals.var_pparam_b4soirdw = assign1780_e3336;
        locals.var_pparam_b4soirdw_dn3 = (((p.p529 * locals.var_inv_l_dn3) + (p.p719 * locals.var_inv_w_dn3)) + (p.p909 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soirdw_dn4 = (((p.p529 * locals.var_inv_l_dn4) + (p.p719 * locals.var_inv_w_dn4)) + (p.p909 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soirdw_dn5 = (((p.p529 * locals.var_inv_l_dn5) + (p.p719 * locals.var_inv_w_dn5)) + (p.p909 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soirdw_dn6 = (((p.p529 * locals.var_inv_l_dn6) + (p.p719 * locals.var_inv_w_dn6)) + (p.p909 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soirdw_dn7 = (((p.p529 * locals.var_inv_l_dn7) + (p.p719 * locals.var_inv_w_dn7)) + (p.p909 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soirdw_dn8 = (((p.p529 * locals.var_inv_l_dn8) + (p.p719 * locals.var_inv_w_dn8)) + (p.p909 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soirdw_dn9 = (((p.p529 * locals.var_inv_l_dn9) + (p.p719 * locals.var_inv_w_dn9)) + (p.p909 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soirdw_dn10 = (((p.p529 * locals.var_inv_l_dn10) + (p.p719 * locals.var_inv_w_dn10)) + (p.p909 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soirdw_dn11 = (((p.p529 * locals.var_inv_l_dn11) + (p.p719 * locals.var_inv_w_dn11)) + (p.p909 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soirdw_dn12 = (((p.p529 * locals.var_inv_l_dn12) + (p.p719 * locals.var_inv_w_dn12)) + (p.p909 * locals.var_inv_lw_dn12));

        let assign1790_e3340: f64 = (p.p530 * locals.var_inv_l);
        let assign1790_e3341: f64 = (p.p142 + assign1790_e3340);
        let assign1790_e3344: f64 = (p.p720 * locals.var_inv_w);
        let assign1790_e3345: f64 = (assign1790_e3341 + assign1790_e3344);
        let assign1790_e3348: f64 = (p.p910 * locals.var_inv_lw);
        let assign1790_e3349: f64 = (assign1790_e3345 + assign1790_e3348);
        locals.var_pparam_b4soiprwb = assign1790_e3349;
        locals.var_pparam_b4soiprwb_dn3 = (((p.p530 * locals.var_inv_l_dn3) + (p.p720 * locals.var_inv_w_dn3)) + (p.p910 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiprwb_dn4 = (((p.p530 * locals.var_inv_l_dn4) + (p.p720 * locals.var_inv_w_dn4)) + (p.p910 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiprwb_dn5 = (((p.p530 * locals.var_inv_l_dn5) + (p.p720 * locals.var_inv_w_dn5)) + (p.p910 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiprwb_dn6 = (((p.p530 * locals.var_inv_l_dn6) + (p.p720 * locals.var_inv_w_dn6)) + (p.p910 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiprwb_dn7 = (((p.p530 * locals.var_inv_l_dn7) + (p.p720 * locals.var_inv_w_dn7)) + (p.p910 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiprwb_dn8 = (((p.p530 * locals.var_inv_l_dn8) + (p.p720 * locals.var_inv_w_dn8)) + (p.p910 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiprwb_dn9 = (((p.p530 * locals.var_inv_l_dn9) + (p.p720 * locals.var_inv_w_dn9)) + (p.p910 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiprwb_dn10 = (((p.p530 * locals.var_inv_l_dn10) + (p.p720 * locals.var_inv_w_dn10)) + (p.p910 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiprwb_dn11 = (((p.p530 * locals.var_inv_l_dn11) + (p.p720 * locals.var_inv_w_dn11)) + (p.p910 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiprwb_dn12 = (((p.p530 * locals.var_inv_l_dn12) + (p.p720 * locals.var_inv_w_dn12)) + (p.p910 * locals.var_inv_lw_dn12));

        let assign1800_e3353: f64 = (p.p531 * locals.var_inv_l);
        let assign1800_e3354: f64 = (p.p143 + assign1800_e3353);
        let assign1800_e3357: f64 = (p.p721 * locals.var_inv_w);
        let assign1800_e3358: f64 = (assign1800_e3354 + assign1800_e3357);
        let assign1800_e3361: f64 = (p.p911 * locals.var_inv_lw);
        let assign1800_e3362: f64 = (assign1800_e3358 + assign1800_e3361);
        locals.var_pparam_b4soiprwe = assign1800_e3362;
        locals.var_pparam_b4soiprwe_dn3 = (((p.p531 * locals.var_inv_l_dn3) + (p.p721 * locals.var_inv_w_dn3)) + (p.p911 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiprwe_dn4 = (((p.p531 * locals.var_inv_l_dn4) + (p.p721 * locals.var_inv_w_dn4)) + (p.p911 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiprwe_dn5 = (((p.p531 * locals.var_inv_l_dn5) + (p.p721 * locals.var_inv_w_dn5)) + (p.p911 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiprwe_dn6 = (((p.p531 * locals.var_inv_l_dn6) + (p.p721 * locals.var_inv_w_dn6)) + (p.p911 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiprwe_dn7 = (((p.p531 * locals.var_inv_l_dn7) + (p.p721 * locals.var_inv_w_dn7)) + (p.p911 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiprwe_dn8 = (((p.p531 * locals.var_inv_l_dn8) + (p.p721 * locals.var_inv_w_dn8)) + (p.p911 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiprwe_dn9 = (((p.p531 * locals.var_inv_l_dn9) + (p.p721 * locals.var_inv_w_dn9)) + (p.p911 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiprwe_dn10 = (((p.p531 * locals.var_inv_l_dn10) + (p.p721 * locals.var_inv_w_dn10)) + (p.p911 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiprwe_dn11 = (((p.p531 * locals.var_inv_l_dn11) + (p.p721 * locals.var_inv_w_dn11)) + (p.p911 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiprwe_dn12 = (((p.p531 * locals.var_inv_l_dn12) + (p.p721 * locals.var_inv_w_dn12)) + (p.p911 * locals.var_inv_lw_dn12));

        let assign1810_e3366: f64 = (p.p532 * locals.var_inv_l);
        let assign1810_e3367: f64 = (p.p141 + assign1810_e3366);
        let assign1810_e3370: f64 = (p.p722 * locals.var_inv_w);
        let assign1810_e3371: f64 = (assign1810_e3367 + assign1810_e3370);
        let assign1810_e3374: f64 = (p.p912 * locals.var_inv_lw);
        let assign1810_e3375: f64 = (assign1810_e3371 + assign1810_e3374);
        locals.var_pparam_b4soiprwg = assign1810_e3375;
        locals.var_pparam_b4soiprwg_dn3 = (((p.p532 * locals.var_inv_l_dn3) + (p.p722 * locals.var_inv_w_dn3)) + (p.p912 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiprwg_dn4 = (((p.p532 * locals.var_inv_l_dn4) + (p.p722 * locals.var_inv_w_dn4)) + (p.p912 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiprwg_dn5 = (((p.p532 * locals.var_inv_l_dn5) + (p.p722 * locals.var_inv_w_dn5)) + (p.p912 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiprwg_dn6 = (((p.p532 * locals.var_inv_l_dn6) + (p.p722 * locals.var_inv_w_dn6)) + (p.p912 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiprwg_dn7 = (((p.p532 * locals.var_inv_l_dn7) + (p.p722 * locals.var_inv_w_dn7)) + (p.p912 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiprwg_dn8 = (((p.p532 * locals.var_inv_l_dn8) + (p.p722 * locals.var_inv_w_dn8)) + (p.p912 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiprwg_dn9 = (((p.p532 * locals.var_inv_l_dn9) + (p.p722 * locals.var_inv_w_dn9)) + (p.p912 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiprwg_dn10 = (((p.p532 * locals.var_inv_l_dn10) + (p.p722 * locals.var_inv_w_dn10)) + (p.p912 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiprwg_dn11 = (((p.p532 * locals.var_inv_l_dn11) + (p.p722 * locals.var_inv_w_dn11)) + (p.p912 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiprwg_dn12 = (((p.p532 * locals.var_inv_l_dn12) + (p.p722 * locals.var_inv_w_dn12)) + (p.p912 * locals.var_inv_lw_dn12));

        let assign1820_e3379: f64 = (p.p533 * locals.var_inv_l);
        let assign1820_e3380: f64 = (p.p196 + assign1820_e3379);
        let assign1820_e3383: f64 = (p.p723 * locals.var_inv_w);
        let assign1820_e3384: f64 = (assign1820_e3380 + assign1820_e3383);
        let assign1820_e3387: f64 = (p.p913 * locals.var_inv_lw);
        let assign1820_e3388: f64 = (assign1820_e3384 + assign1820_e3387);
        locals.var_pparam_b4soiwr = assign1820_e3388;
        locals.var_pparam_b4soiwr_dn3 = (((p.p533 * locals.var_inv_l_dn3) + (p.p723 * locals.var_inv_w_dn3)) + (p.p913 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiwr_dn4 = (((p.p533 * locals.var_inv_l_dn4) + (p.p723 * locals.var_inv_w_dn4)) + (p.p913 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiwr_dn5 = (((p.p533 * locals.var_inv_l_dn5) + (p.p723 * locals.var_inv_w_dn5)) + (p.p913 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiwr_dn6 = (((p.p533 * locals.var_inv_l_dn6) + (p.p723 * locals.var_inv_w_dn6)) + (p.p913 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiwr_dn7 = (((p.p533 * locals.var_inv_l_dn7) + (p.p723 * locals.var_inv_w_dn7)) + (p.p913 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiwr_dn8 = (((p.p533 * locals.var_inv_l_dn8) + (p.p723 * locals.var_inv_w_dn8)) + (p.p913 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiwr_dn9 = (((p.p533 * locals.var_inv_l_dn9) + (p.p723 * locals.var_inv_w_dn9)) + (p.p913 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiwr_dn10 = (((p.p533 * locals.var_inv_l_dn10) + (p.p723 * locals.var_inv_w_dn10)) + (p.p913 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiwr_dn11 = (((p.p533 * locals.var_inv_l_dn11) + (p.p723 * locals.var_inv_w_dn11)) + (p.p913 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiwr_dn12 = (((p.p533 * locals.var_inv_l_dn12) + (p.p723 * locals.var_inv_w_dn12)) + (p.p913 * locals.var_inv_lw_dn12));

        let assign1830_e3392: f64 = (p.p534 * locals.var_inv_l);
        let assign1830_e3393: f64 = (p.p73 + assign1830_e3392);
        let assign1830_e3396: f64 = (p.p724 * locals.var_inv_w);
        let assign1830_e3397: f64 = (assign1830_e3393 + assign1830_e3396);
        let assign1830_e3400: f64 = (p.p914 * locals.var_inv_lw);
        let assign1830_e3401: f64 = (assign1830_e3397 + assign1830_e3400);
        locals.var_pparam_b4soinfactor = assign1830_e3401;
        locals.var_pparam_b4soinfactor_dn3 = (((p.p534 * locals.var_inv_l_dn3) + (p.p724 * locals.var_inv_w_dn3)) + (p.p914 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soinfactor_dn4 = (((p.p534 * locals.var_inv_l_dn4) + (p.p724 * locals.var_inv_w_dn4)) + (p.p914 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soinfactor_dn5 = (((p.p534 * locals.var_inv_l_dn5) + (p.p724 * locals.var_inv_w_dn5)) + (p.p914 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soinfactor_dn6 = (((p.p534 * locals.var_inv_l_dn6) + (p.p724 * locals.var_inv_w_dn6)) + (p.p914 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soinfactor_dn7 = (((p.p534 * locals.var_inv_l_dn7) + (p.p724 * locals.var_inv_w_dn7)) + (p.p914 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soinfactor_dn8 = (((p.p534 * locals.var_inv_l_dn8) + (p.p724 * locals.var_inv_w_dn8)) + (p.p914 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soinfactor_dn9 = (((p.p534 * locals.var_inv_l_dn9) + (p.p724 * locals.var_inv_w_dn9)) + (p.p914 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soinfactor_dn10 = (((p.p534 * locals.var_inv_l_dn10) + (p.p724 * locals.var_inv_w_dn10)) + (p.p914 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soinfactor_dn11 = (((p.p534 * locals.var_inv_l_dn11) + (p.p724 * locals.var_inv_w_dn11)) + (p.p914 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soinfactor_dn12 = (((p.p534 * locals.var_inv_l_dn12) + (p.p724 * locals.var_inv_w_dn12)) + (p.p914 * locals.var_inv_lw_dn12));

        let assign1840_e3405: f64 = (p.p535 * locals.var_inv_l);
        let assign1840_e3406: f64 = (p.p198 + assign1840_e3405);
        let assign1840_e3409: f64 = (p.p725 * locals.var_inv_w);
        let assign1840_e3410: f64 = (assign1840_e3406 + assign1840_e3409);
        let assign1840_e3413: f64 = (p.p915 * locals.var_inv_lw);
        let assign1840_e3414: f64 = (assign1840_e3410 + assign1840_e3413);
        locals.var_pparam_b4soidwg = assign1840_e3414;
        locals.var_pparam_b4soidwg_dn3 = (((p.p535 * locals.var_inv_l_dn3) + (p.p725 * locals.var_inv_w_dn3)) + (p.p915 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidwg_dn4 = (((p.p535 * locals.var_inv_l_dn4) + (p.p725 * locals.var_inv_w_dn4)) + (p.p915 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidwg_dn5 = (((p.p535 * locals.var_inv_l_dn5) + (p.p725 * locals.var_inv_w_dn5)) + (p.p915 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidwg_dn6 = (((p.p535 * locals.var_inv_l_dn6) + (p.p725 * locals.var_inv_w_dn6)) + (p.p915 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidwg_dn7 = (((p.p535 * locals.var_inv_l_dn7) + (p.p725 * locals.var_inv_w_dn7)) + (p.p915 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidwg_dn8 = (((p.p535 * locals.var_inv_l_dn8) + (p.p725 * locals.var_inv_w_dn8)) + (p.p915 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidwg_dn9 = (((p.p535 * locals.var_inv_l_dn9) + (p.p725 * locals.var_inv_w_dn9)) + (p.p915 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidwg_dn10 = (((p.p535 * locals.var_inv_l_dn10) + (p.p725 * locals.var_inv_w_dn10)) + (p.p915 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidwg_dn11 = (((p.p535 * locals.var_inv_l_dn11) + (p.p725 * locals.var_inv_w_dn11)) + (p.p915 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidwg_dn12 = (((p.p535 * locals.var_inv_l_dn12) + (p.p725 * locals.var_inv_w_dn12)) + (p.p915 * locals.var_inv_lw_dn12));

        let assign1850_e3418: f64 = (p.p536 * locals.var_inv_l);
        let assign1850_e3419: f64 = (p.p199 + assign1850_e3418);
        let assign1850_e3422: f64 = (p.p726 * locals.var_inv_w);
        let assign1850_e3423: f64 = (assign1850_e3419 + assign1850_e3422);
        let assign1850_e3426: f64 = (p.p916 * locals.var_inv_lw);
        let assign1850_e3427: f64 = (assign1850_e3423 + assign1850_e3426);
        locals.var_pparam_b4soidwb = assign1850_e3427;
        locals.var_pparam_b4soidwb_dn3 = (((p.p536 * locals.var_inv_l_dn3) + (p.p726 * locals.var_inv_w_dn3)) + (p.p916 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidwb_dn4 = (((p.p536 * locals.var_inv_l_dn4) + (p.p726 * locals.var_inv_w_dn4)) + (p.p916 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidwb_dn5 = (((p.p536 * locals.var_inv_l_dn5) + (p.p726 * locals.var_inv_w_dn5)) + (p.p916 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidwb_dn6 = (((p.p536 * locals.var_inv_l_dn6) + (p.p726 * locals.var_inv_w_dn6)) + (p.p916 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidwb_dn7 = (((p.p536 * locals.var_inv_l_dn7) + (p.p726 * locals.var_inv_w_dn7)) + (p.p916 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidwb_dn8 = (((p.p536 * locals.var_inv_l_dn8) + (p.p726 * locals.var_inv_w_dn8)) + (p.p916 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidwb_dn9 = (((p.p536 * locals.var_inv_l_dn9) + (p.p726 * locals.var_inv_w_dn9)) + (p.p916 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidwb_dn10 = (((p.p536 * locals.var_inv_l_dn10) + (p.p726 * locals.var_inv_w_dn10)) + (p.p916 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidwb_dn11 = (((p.p536 * locals.var_inv_l_dn11) + (p.p726 * locals.var_inv_w_dn11)) + (p.p916 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidwb_dn12 = (((p.p536 * locals.var_inv_l_dn12) + (p.p726 * locals.var_inv_w_dn12)) + (p.p916 * locals.var_inv_lw_dn12));

        let assign1860_e3431: f64 = (p.p537 * locals.var_inv_l);
        let assign1860_e3432: f64 = (p.p125 + assign1860_e3431);
        let assign1860_e3435: f64 = (p.p727 * locals.var_inv_w);
        let assign1860_e3436: f64 = (assign1860_e3432 + assign1860_e3435);
        let assign1860_e3439: f64 = (p.p917 * locals.var_inv_lw);
        let assign1860_e3440: f64 = (assign1860_e3436 + assign1860_e3439);
        locals.var_pparam_b4soivoff = assign1860_e3440;
        locals.var_pparam_b4soivoff_dn3 = (((p.p537 * locals.var_inv_l_dn3) + (p.p727 * locals.var_inv_w_dn3)) + (p.p917 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivoff_dn4 = (((p.p537 * locals.var_inv_l_dn4) + (p.p727 * locals.var_inv_w_dn4)) + (p.p917 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivoff_dn5 = (((p.p537 * locals.var_inv_l_dn5) + (p.p727 * locals.var_inv_w_dn5)) + (p.p917 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivoff_dn6 = (((p.p537 * locals.var_inv_l_dn6) + (p.p727 * locals.var_inv_w_dn6)) + (p.p917 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivoff_dn7 = (((p.p537 * locals.var_inv_l_dn7) + (p.p727 * locals.var_inv_w_dn7)) + (p.p917 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivoff_dn8 = (((p.p537 * locals.var_inv_l_dn8) + (p.p727 * locals.var_inv_w_dn8)) + (p.p917 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivoff_dn9 = (((p.p537 * locals.var_inv_l_dn9) + (p.p727 * locals.var_inv_w_dn9)) + (p.p917 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivoff_dn10 = (((p.p537 * locals.var_inv_l_dn10) + (p.p727 * locals.var_inv_w_dn10)) + (p.p917 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivoff_dn11 = (((p.p537 * locals.var_inv_l_dn11) + (p.p727 * locals.var_inv_w_dn11)) + (p.p917 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivoff_dn12 = (((p.p537 * locals.var_inv_l_dn12) + (p.p727 * locals.var_inv_w_dn12)) + (p.p917 * locals.var_inv_lw_dn12));

        let assign1870_e3444: f64 = (p.p538 * locals.var_inv_l);
        let assign1870_e3445: f64 = (p.p145 + assign1870_e3444);
        let assign1870_e3448: f64 = (p.p728 * locals.var_inv_w);
        let assign1870_e3449: f64 = (assign1870_e3445 + assign1870_e3448);
        let assign1870_e3452: f64 = (p.p918 * locals.var_inv_lw);
        let assign1870_e3453: f64 = (assign1870_e3449 + assign1870_e3452);
        locals.var_pparam_b4soieta0 = assign1870_e3453;
        locals.var_pparam_b4soieta0_dn3 = (((p.p538 * locals.var_inv_l_dn3) + (p.p728 * locals.var_inv_w_dn3)) + (p.p918 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soieta0_dn4 = (((p.p538 * locals.var_inv_l_dn4) + (p.p728 * locals.var_inv_w_dn4)) + (p.p918 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soieta0_dn5 = (((p.p538 * locals.var_inv_l_dn5) + (p.p728 * locals.var_inv_w_dn5)) + (p.p918 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soieta0_dn6 = (((p.p538 * locals.var_inv_l_dn6) + (p.p728 * locals.var_inv_w_dn6)) + (p.p918 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soieta0_dn7 = (((p.p538 * locals.var_inv_l_dn7) + (p.p728 * locals.var_inv_w_dn7)) + (p.p918 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soieta0_dn8 = (((p.p538 * locals.var_inv_l_dn8) + (p.p728 * locals.var_inv_w_dn8)) + (p.p918 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soieta0_dn9 = (((p.p538 * locals.var_inv_l_dn9) + (p.p728 * locals.var_inv_w_dn9)) + (p.p918 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soieta0_dn10 = (((p.p538 * locals.var_inv_l_dn10) + (p.p728 * locals.var_inv_w_dn10)) + (p.p918 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soieta0_dn11 = (((p.p538 * locals.var_inv_l_dn11) + (p.p728 * locals.var_inv_w_dn11)) + (p.p918 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soieta0_dn12 = (((p.p538 * locals.var_inv_l_dn12) + (p.p728 * locals.var_inv_w_dn12)) + (p.p918 * locals.var_inv_lw_dn12));

        let assign1880_e3457: f64 = (p.p539 * locals.var_inv_l);
        let assign1880_e3458: f64 = (p.p146 + assign1880_e3457);
        let assign1880_e3461: f64 = (p.p729 * locals.var_inv_w);
        let assign1880_e3462: f64 = (assign1880_e3458 + assign1880_e3461);
        let assign1880_e3465: f64 = (p.p919 * locals.var_inv_lw);
        let assign1880_e3466: f64 = (assign1880_e3462 + assign1880_e3465);
        locals.var_pparam_b4soietab = assign1880_e3466;
        locals.var_pparam_b4soietab_dn3 = (((p.p539 * locals.var_inv_l_dn3) + (p.p729 * locals.var_inv_w_dn3)) + (p.p919 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soietab_dn4 = (((p.p539 * locals.var_inv_l_dn4) + (p.p729 * locals.var_inv_w_dn4)) + (p.p919 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soietab_dn5 = (((p.p539 * locals.var_inv_l_dn5) + (p.p729 * locals.var_inv_w_dn5)) + (p.p919 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soietab_dn6 = (((p.p539 * locals.var_inv_l_dn6) + (p.p729 * locals.var_inv_w_dn6)) + (p.p919 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soietab_dn7 = (((p.p539 * locals.var_inv_l_dn7) + (p.p729 * locals.var_inv_w_dn7)) + (p.p919 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soietab_dn8 = (((p.p539 * locals.var_inv_l_dn8) + (p.p729 * locals.var_inv_w_dn8)) + (p.p919 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soietab_dn9 = (((p.p539 * locals.var_inv_l_dn9) + (p.p729 * locals.var_inv_w_dn9)) + (p.p919 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soietab_dn10 = (((p.p539 * locals.var_inv_l_dn10) + (p.p729 * locals.var_inv_w_dn10)) + (p.p919 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soietab_dn11 = (((p.p539 * locals.var_inv_l_dn11) + (p.p729 * locals.var_inv_w_dn11)) + (p.p919 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soietab_dn12 = (((p.p539 * locals.var_inv_l_dn12) + (p.p729 * locals.var_inv_w_dn12)) + (p.p919 * locals.var_inv_lw_dn12));

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign1890_e3470: f64 = (p.p540 * locals.var_inv_l);
        let assign1890_e3471: f64 = (p.p147 + assign1890_e3470);
        let assign1890_e3474: f64 = (p.p730 * locals.var_inv_w);
        let assign1890_e3475: f64 = (assign1890_e3471 + assign1890_e3474);
        let assign1890_e3478: f64 = (p.p920 * locals.var_inv_lw);
        let assign1890_e3479: f64 = (assign1890_e3475 + assign1890_e3478);
        locals.var_pparam_b4soieta0cv = assign1890_e3479;
        locals.var_pparam_b4soieta0cv_dn3 = (((p.p540 * locals.var_inv_l_dn3) + (p.p730 * locals.var_inv_w_dn3)) + (p.p920 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soieta0cv_dn4 = (((p.p540 * locals.var_inv_l_dn4) + (p.p730 * locals.var_inv_w_dn4)) + (p.p920 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soieta0cv_dn5 = (((p.p540 * locals.var_inv_l_dn5) + (p.p730 * locals.var_inv_w_dn5)) + (p.p920 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soieta0cv_dn6 = (((p.p540 * locals.var_inv_l_dn6) + (p.p730 * locals.var_inv_w_dn6)) + (p.p920 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soieta0cv_dn7 = (((p.p540 * locals.var_inv_l_dn7) + (p.p730 * locals.var_inv_w_dn7)) + (p.p920 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soieta0cv_dn8 = (((p.p540 * locals.var_inv_l_dn8) + (p.p730 * locals.var_inv_w_dn8)) + (p.p920 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soieta0cv_dn9 = (((p.p540 * locals.var_inv_l_dn9) + (p.p730 * locals.var_inv_w_dn9)) + (p.p920 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soieta0cv_dn10 = (((p.p540 * locals.var_inv_l_dn10) + (p.p730 * locals.var_inv_w_dn10)) + (p.p920 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soieta0cv_dn11 = (((p.p540 * locals.var_inv_l_dn11) + (p.p730 * locals.var_inv_w_dn11)) + (p.p920 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soieta0cv_dn12 = (((p.p540 * locals.var_inv_l_dn12) + (p.p730 * locals.var_inv_w_dn12)) + (p.p920 * locals.var_inv_lw_dn12));

        let assign1900_e3483: f64 = (p.p541 * locals.var_inv_l);
        let assign1900_e3484: f64 = (p.p148 + assign1900_e3483);
        let assign1900_e3487: f64 = (p.p731 * locals.var_inv_w);
        let assign1900_e3488: f64 = (assign1900_e3484 + assign1900_e3487);
        let assign1900_e3491: f64 = (p.p921 * locals.var_inv_lw);
        let assign1900_e3492: f64 = (assign1900_e3488 + assign1900_e3491);
        locals.var_pparam_b4soietabcv = assign1900_e3492;
        locals.var_pparam_b4soietabcv_dn3 = (((p.p541 * locals.var_inv_l_dn3) + (p.p731 * locals.var_inv_w_dn3)) + (p.p921 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soietabcv_dn4 = (((p.p541 * locals.var_inv_l_dn4) + (p.p731 * locals.var_inv_w_dn4)) + (p.p921 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soietabcv_dn5 = (((p.p541 * locals.var_inv_l_dn5) + (p.p731 * locals.var_inv_w_dn5)) + (p.p921 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soietabcv_dn6 = (((p.p541 * locals.var_inv_l_dn6) + (p.p731 * locals.var_inv_w_dn6)) + (p.p921 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soietabcv_dn7 = (((p.p541 * locals.var_inv_l_dn7) + (p.p731 * locals.var_inv_w_dn7)) + (p.p921 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soietabcv_dn8 = (((p.p541 * locals.var_inv_l_dn8) + (p.p731 * locals.var_inv_w_dn8)) + (p.p921 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soietabcv_dn9 = (((p.p541 * locals.var_inv_l_dn9) + (p.p731 * locals.var_inv_w_dn9)) + (p.p921 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soietabcv_dn10 = (((p.p541 * locals.var_inv_l_dn10) + (p.p731 * locals.var_inv_w_dn10)) + (p.p921 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soietabcv_dn11 = (((p.p541 * locals.var_inv_l_dn11) + (p.p731 * locals.var_inv_w_dn11)) + (p.p921 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soietabcv_dn12 = (((p.p541 * locals.var_inv_l_dn12) + (p.p731 * locals.var_inv_w_dn12)) + (p.p921 * locals.var_inv_lw_dn12));

        let assign1910_e3496: f64 = (p.p542 * locals.var_inv_l);
        let assign1910_e3497: f64 = (p.p106 + assign1910_e3496);
        let assign1910_e3500: f64 = (p.p732 * locals.var_inv_w);
        let assign1910_e3501: f64 = (assign1910_e3497 + assign1910_e3500);
        let assign1910_e3504: f64 = (p.p922 * locals.var_inv_lw);
        let assign1910_e3505: f64 = (assign1910_e3501 + assign1910_e3504);
        locals.var_pparam_b4soidsub = assign1910_e3505;
        locals.var_pparam_b4soidsub_dn3 = (((p.p542 * locals.var_inv_l_dn3) + (p.p732 * locals.var_inv_w_dn3)) + (p.p922 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidsub_dn4 = (((p.p542 * locals.var_inv_l_dn4) + (p.p732 * locals.var_inv_w_dn4)) + (p.p922 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidsub_dn5 = (((p.p542 * locals.var_inv_l_dn5) + (p.p732 * locals.var_inv_w_dn5)) + (p.p922 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidsub_dn6 = (((p.p542 * locals.var_inv_l_dn6) + (p.p732 * locals.var_inv_w_dn6)) + (p.p922 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidsub_dn7 = (((p.p542 * locals.var_inv_l_dn7) + (p.p732 * locals.var_inv_w_dn7)) + (p.p922 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidsub_dn8 = (((p.p542 * locals.var_inv_l_dn8) + (p.p732 * locals.var_inv_w_dn8)) + (p.p922 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidsub_dn9 = (((p.p542 * locals.var_inv_l_dn9) + (p.p732 * locals.var_inv_w_dn9)) + (p.p922 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidsub_dn10 = (((p.p542 * locals.var_inv_l_dn10) + (p.p732 * locals.var_inv_w_dn10)) + (p.p922 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidsub_dn11 = (((p.p542 * locals.var_inv_l_dn11) + (p.p732 * locals.var_inv_w_dn11)) + (p.p922 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidsub_dn12 = (((p.p542 * locals.var_inv_l_dn12) + (p.p732 * locals.var_inv_w_dn12)) + (p.p922 * locals.var_inv_lw_dn12));

        let assign1920_e3509: f64 = (p.p543 * locals.var_inv_l);
        let assign1920_e3510: f64 = (p.p72 + assign1920_e3509);
        let assign1920_e3513: f64 = (p.p733 * locals.var_inv_w);
        let assign1920_e3514: f64 = (assign1920_e3510 + assign1920_e3513);
        let assign1920_e3517: f64 = (p.p923 * locals.var_inv_lw);
        let assign1920_e3518: f64 = (assign1920_e3514 + assign1920_e3517);
        locals.var_pparam_b4soicit = assign1920_e3518;
        locals.var_pparam_b4soicit_dn3 = (((p.p543 * locals.var_inv_l_dn3) + (p.p733 * locals.var_inv_w_dn3)) + (p.p923 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soicit_dn4 = (((p.p543 * locals.var_inv_l_dn4) + (p.p733 * locals.var_inv_w_dn4)) + (p.p923 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soicit_dn5 = (((p.p543 * locals.var_inv_l_dn5) + (p.p733 * locals.var_inv_w_dn5)) + (p.p923 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soicit_dn6 = (((p.p543 * locals.var_inv_l_dn6) + (p.p733 * locals.var_inv_w_dn6)) + (p.p923 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soicit_dn7 = (((p.p543 * locals.var_inv_l_dn7) + (p.p733 * locals.var_inv_w_dn7)) + (p.p923 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soicit_dn8 = (((p.p543 * locals.var_inv_l_dn8) + (p.p733 * locals.var_inv_w_dn8)) + (p.p923 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soicit_dn9 = (((p.p543 * locals.var_inv_l_dn9) + (p.p733 * locals.var_inv_w_dn9)) + (p.p923 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soicit_dn10 = (((p.p543 * locals.var_inv_l_dn10) + (p.p733 * locals.var_inv_w_dn10)) + (p.p923 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soicit_dn11 = (((p.p543 * locals.var_inv_l_dn11) + (p.p733 * locals.var_inv_w_dn11)) + (p.p923 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soicit_dn12 = (((p.p543 * locals.var_inv_l_dn12) + (p.p733 * locals.var_inv_w_dn12)) + (p.p923 * locals.var_inv_lw_dn12));

        let assign1930_e3522: f64 = (p.p544 * locals.var_inv_l);
        let assign1930_e3523: f64 = (p.p69 + assign1930_e3522);
        let assign1930_e3526: f64 = (p.p734 * locals.var_inv_w);
        let assign1930_e3527: f64 = (assign1930_e3523 + assign1930_e3526);
        let assign1930_e3530: f64 = (p.p924 * locals.var_inv_lw);
        let assign1930_e3531: f64 = (assign1930_e3527 + assign1930_e3530);
        locals.var_pparam_b4soicdsc = assign1930_e3531;
        locals.var_pparam_b4soicdsc_dn3 = (((p.p544 * locals.var_inv_l_dn3) + (p.p734 * locals.var_inv_w_dn3)) + (p.p924 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soicdsc_dn4 = (((p.p544 * locals.var_inv_l_dn4) + (p.p734 * locals.var_inv_w_dn4)) + (p.p924 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soicdsc_dn5 = (((p.p544 * locals.var_inv_l_dn5) + (p.p734 * locals.var_inv_w_dn5)) + (p.p924 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soicdsc_dn6 = (((p.p544 * locals.var_inv_l_dn6) + (p.p734 * locals.var_inv_w_dn6)) + (p.p924 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soicdsc_dn7 = (((p.p544 * locals.var_inv_l_dn7) + (p.p734 * locals.var_inv_w_dn7)) + (p.p924 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soicdsc_dn8 = (((p.p544 * locals.var_inv_l_dn8) + (p.p734 * locals.var_inv_w_dn8)) + (p.p924 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soicdsc_dn9 = (((p.p544 * locals.var_inv_l_dn9) + (p.p734 * locals.var_inv_w_dn9)) + (p.p924 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soicdsc_dn10 = (((p.p544 * locals.var_inv_l_dn10) + (p.p734 * locals.var_inv_w_dn10)) + (p.p924 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soicdsc_dn11 = (((p.p544 * locals.var_inv_l_dn11) + (p.p734 * locals.var_inv_w_dn11)) + (p.p924 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soicdsc_dn12 = (((p.p544 * locals.var_inv_l_dn12) + (p.p734 * locals.var_inv_w_dn12)) + (p.p924 * locals.var_inv_lw_dn12));

        let assign1940_e3535: f64 = (p.p545 * locals.var_inv_l);
        let assign1940_e3536: f64 = (p.p70 + assign1940_e3535);
        let assign1940_e3539: f64 = (p.p735 * locals.var_inv_w);
        let assign1940_e3540: f64 = (assign1940_e3536 + assign1940_e3539);
        let assign1940_e3543: f64 = (p.p925 * locals.var_inv_lw);
        let assign1940_e3544: f64 = (assign1940_e3540 + assign1940_e3543);
        locals.var_pparam_b4soicdscb = assign1940_e3544;
        locals.var_pparam_b4soicdscb_dn3 = (((p.p545 * locals.var_inv_l_dn3) + (p.p735 * locals.var_inv_w_dn3)) + (p.p925 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soicdscb_dn4 = (((p.p545 * locals.var_inv_l_dn4) + (p.p735 * locals.var_inv_w_dn4)) + (p.p925 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soicdscb_dn5 = (((p.p545 * locals.var_inv_l_dn5) + (p.p735 * locals.var_inv_w_dn5)) + (p.p925 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soicdscb_dn6 = (((p.p545 * locals.var_inv_l_dn6) + (p.p735 * locals.var_inv_w_dn6)) + (p.p925 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soicdscb_dn7 = (((p.p545 * locals.var_inv_l_dn7) + (p.p735 * locals.var_inv_w_dn7)) + (p.p925 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soicdscb_dn8 = (((p.p545 * locals.var_inv_l_dn8) + (p.p735 * locals.var_inv_w_dn8)) + (p.p925 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soicdscb_dn9 = (((p.p545 * locals.var_inv_l_dn9) + (p.p735 * locals.var_inv_w_dn9)) + (p.p925 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soicdscb_dn10 = (((p.p545 * locals.var_inv_l_dn10) + (p.p735 * locals.var_inv_w_dn10)) + (p.p925 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soicdscb_dn11 = (((p.p545 * locals.var_inv_l_dn11) + (p.p735 * locals.var_inv_w_dn11)) + (p.p925 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soicdscb_dn12 = (((p.p545 * locals.var_inv_l_dn12) + (p.p735 * locals.var_inv_w_dn12)) + (p.p925 * locals.var_inv_lw_dn12));

        let assign1950_e3548: f64 = (p.p546 * locals.var_inv_l);
        let assign1950_e3549: f64 = (p.p71 + assign1950_e3548);
        let assign1950_e3552: f64 = (p.p736 * locals.var_inv_w);
        let assign1950_e3553: f64 = (assign1950_e3549 + assign1950_e3552);
        let assign1950_e3556: f64 = (p.p926 * locals.var_inv_lw);
        let assign1950_e3557: f64 = (assign1950_e3553 + assign1950_e3556);
        locals.var_pparam_b4soicdscd = assign1950_e3557;
        locals.var_pparam_b4soicdscd_dn3 = (((p.p546 * locals.var_inv_l_dn3) + (p.p736 * locals.var_inv_w_dn3)) + (p.p926 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soicdscd_dn4 = (((p.p546 * locals.var_inv_l_dn4) + (p.p736 * locals.var_inv_w_dn4)) + (p.p926 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soicdscd_dn5 = (((p.p546 * locals.var_inv_l_dn5) + (p.p736 * locals.var_inv_w_dn5)) + (p.p926 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soicdscd_dn6 = (((p.p546 * locals.var_inv_l_dn6) + (p.p736 * locals.var_inv_w_dn6)) + (p.p926 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soicdscd_dn7 = (((p.p546 * locals.var_inv_l_dn7) + (p.p736 * locals.var_inv_w_dn7)) + (p.p926 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soicdscd_dn8 = (((p.p546 * locals.var_inv_l_dn8) + (p.p736 * locals.var_inv_w_dn8)) + (p.p926 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soicdscd_dn9 = (((p.p546 * locals.var_inv_l_dn9) + (p.p736 * locals.var_inv_w_dn9)) + (p.p926 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soicdscd_dn10 = (((p.p546 * locals.var_inv_l_dn10) + (p.p736 * locals.var_inv_w_dn10)) + (p.p926 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soicdscd_dn11 = (((p.p546 * locals.var_inv_l_dn11) + (p.p736 * locals.var_inv_w_dn11)) + (p.p926 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soicdscd_dn12 = (((p.p546 * locals.var_inv_l_dn12) + (p.p736 * locals.var_inv_w_dn12)) + (p.p926 * locals.var_inv_lw_dn12));

        let assign1960_e3561: f64 = (p.p547 * locals.var_inv_l);
        let assign1960_e3562: f64 = (p.p149 + assign1960_e3561);
        let assign1960_e3565: f64 = (p.p737 * locals.var_inv_w);
        let assign1960_e3566: f64 = (assign1960_e3562 + assign1960_e3565);
        let assign1960_e3569: f64 = (p.p927 * locals.var_inv_lw);
        let assign1960_e3570: f64 = (assign1960_e3566 + assign1960_e3569);
        locals.var_pparam_b4soipclm = assign1960_e3570;
        locals.var_pparam_b4soipclm_dn3 = (((p.p547 * locals.var_inv_l_dn3) + (p.p737 * locals.var_inv_w_dn3)) + (p.p927 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soipclm_dn4 = (((p.p547 * locals.var_inv_l_dn4) + (p.p737 * locals.var_inv_w_dn4)) + (p.p927 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soipclm_dn5 = (((p.p547 * locals.var_inv_l_dn5) + (p.p737 * locals.var_inv_w_dn5)) + (p.p927 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soipclm_dn6 = (((p.p547 * locals.var_inv_l_dn6) + (p.p737 * locals.var_inv_w_dn6)) + (p.p927 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soipclm_dn7 = (((p.p547 * locals.var_inv_l_dn7) + (p.p737 * locals.var_inv_w_dn7)) + (p.p927 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soipclm_dn8 = (((p.p547 * locals.var_inv_l_dn8) + (p.p737 * locals.var_inv_w_dn8)) + (p.p927 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soipclm_dn9 = (((p.p547 * locals.var_inv_l_dn9) + (p.p737 * locals.var_inv_w_dn9)) + (p.p927 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soipclm_dn10 = (((p.p547 * locals.var_inv_l_dn10) + (p.p737 * locals.var_inv_w_dn10)) + (p.p927 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soipclm_dn11 = (((p.p547 * locals.var_inv_l_dn11) + (p.p737 * locals.var_inv_w_dn11)) + (p.p927 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soipclm_dn12 = (((p.p547 * locals.var_inv_l_dn12) + (p.p737 * locals.var_inv_w_dn12)) + (p.p927 * locals.var_inv_lw_dn12));

        let assign1970_e3574: f64 = (p.p548 * locals.var_inv_l);
        let assign1970_e3575: f64 = (p.p150 + assign1970_e3574);
        let assign1970_e3578: f64 = (p.p738 * locals.var_inv_w);
        let assign1970_e3579: f64 = (assign1970_e3575 + assign1970_e3578);
        let assign1970_e3582: f64 = (p.p928 * locals.var_inv_lw);
        let assign1970_e3583: f64 = (assign1970_e3579 + assign1970_e3582);
        locals.var_pparam_b4soipdibl1 = assign1970_e3583;
        locals.var_pparam_b4soipdibl1_dn3 = (((p.p548 * locals.var_inv_l_dn3) + (p.p738 * locals.var_inv_w_dn3)) + (p.p928 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soipdibl1_dn4 = (((p.p548 * locals.var_inv_l_dn4) + (p.p738 * locals.var_inv_w_dn4)) + (p.p928 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soipdibl1_dn5 = (((p.p548 * locals.var_inv_l_dn5) + (p.p738 * locals.var_inv_w_dn5)) + (p.p928 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soipdibl1_dn6 = (((p.p548 * locals.var_inv_l_dn6) + (p.p738 * locals.var_inv_w_dn6)) + (p.p928 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soipdibl1_dn7 = (((p.p548 * locals.var_inv_l_dn7) + (p.p738 * locals.var_inv_w_dn7)) + (p.p928 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soipdibl1_dn8 = (((p.p548 * locals.var_inv_l_dn8) + (p.p738 * locals.var_inv_w_dn8)) + (p.p928 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soipdibl1_dn9 = (((p.p548 * locals.var_inv_l_dn9) + (p.p738 * locals.var_inv_w_dn9)) + (p.p928 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soipdibl1_dn10 = (((p.p548 * locals.var_inv_l_dn10) + (p.p738 * locals.var_inv_w_dn10)) + (p.p928 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soipdibl1_dn11 = (((p.p548 * locals.var_inv_l_dn11) + (p.p738 * locals.var_inv_w_dn11)) + (p.p928 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soipdibl1_dn12 = (((p.p548 * locals.var_inv_l_dn12) + (p.p738 * locals.var_inv_w_dn12)) + (p.p928 * locals.var_inv_lw_dn12));

        let assign1980_e3587: f64 = (p.p549 * locals.var_inv_l);
        let assign1980_e3588: f64 = (p.p151 + assign1980_e3587);
        let assign1980_e3591: f64 = (p.p739 * locals.var_inv_w);
        let assign1980_e3592: f64 = (assign1980_e3588 + assign1980_e3591);
        let assign1980_e3595: f64 = (p.p929 * locals.var_inv_lw);
        let assign1980_e3596: f64 = (assign1980_e3592 + assign1980_e3595);
        locals.var_pparam_b4soipdibl2 = assign1980_e3596;
        locals.var_pparam_b4soipdibl2_dn3 = (((p.p549 * locals.var_inv_l_dn3) + (p.p739 * locals.var_inv_w_dn3)) + (p.p929 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soipdibl2_dn4 = (((p.p549 * locals.var_inv_l_dn4) + (p.p739 * locals.var_inv_w_dn4)) + (p.p929 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soipdibl2_dn5 = (((p.p549 * locals.var_inv_l_dn5) + (p.p739 * locals.var_inv_w_dn5)) + (p.p929 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soipdibl2_dn6 = (((p.p549 * locals.var_inv_l_dn6) + (p.p739 * locals.var_inv_w_dn6)) + (p.p929 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soipdibl2_dn7 = (((p.p549 * locals.var_inv_l_dn7) + (p.p739 * locals.var_inv_w_dn7)) + (p.p929 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soipdibl2_dn8 = (((p.p549 * locals.var_inv_l_dn8) + (p.p739 * locals.var_inv_w_dn8)) + (p.p929 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soipdibl2_dn9 = (((p.p549 * locals.var_inv_l_dn9) + (p.p739 * locals.var_inv_w_dn9)) + (p.p929 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soipdibl2_dn10 = (((p.p549 * locals.var_inv_l_dn10) + (p.p739 * locals.var_inv_w_dn10)) + (p.p929 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soipdibl2_dn11 = (((p.p549 * locals.var_inv_l_dn11) + (p.p739 * locals.var_inv_w_dn11)) + (p.p929 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soipdibl2_dn12 = (((p.p549 * locals.var_inv_l_dn12) + (p.p739 * locals.var_inv_w_dn12)) + (p.p929 * locals.var_inv_lw_dn12));

        let assign1990_e3600: f64 = (p.p550 * locals.var_inv_l);
        let assign1990_e3601: f64 = (p.p152 + assign1990_e3600);
        let assign1990_e3604: f64 = (p.p740 * locals.var_inv_w);
        let assign1990_e3605: f64 = (assign1990_e3601 + assign1990_e3604);
        let assign1990_e3608: f64 = (p.p930 * locals.var_inv_lw);
        let assign1990_e3609: f64 = (assign1990_e3605 + assign1990_e3608);
        locals.var_pparam_b4soipdiblb = assign1990_e3609;
        locals.var_pparam_b4soipdiblb_dn3 = (((p.p550 * locals.var_inv_l_dn3) + (p.p740 * locals.var_inv_w_dn3)) + (p.p930 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soipdiblb_dn4 = (((p.p550 * locals.var_inv_l_dn4) + (p.p740 * locals.var_inv_w_dn4)) + (p.p930 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soipdiblb_dn5 = (((p.p550 * locals.var_inv_l_dn5) + (p.p740 * locals.var_inv_w_dn5)) + (p.p930 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soipdiblb_dn6 = (((p.p550 * locals.var_inv_l_dn6) + (p.p740 * locals.var_inv_w_dn6)) + (p.p930 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soipdiblb_dn7 = (((p.p550 * locals.var_inv_l_dn7) + (p.p740 * locals.var_inv_w_dn7)) + (p.p930 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soipdiblb_dn8 = (((p.p550 * locals.var_inv_l_dn8) + (p.p740 * locals.var_inv_w_dn8)) + (p.p930 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soipdiblb_dn9 = (((p.p550 * locals.var_inv_l_dn9) + (p.p740 * locals.var_inv_w_dn9)) + (p.p930 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soipdiblb_dn10 = (((p.p550 * locals.var_inv_l_dn10) + (p.p740 * locals.var_inv_w_dn10)) + (p.p930 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soipdiblb_dn11 = (((p.p550 * locals.var_inv_l_dn11) + (p.p740 * locals.var_inv_w_dn11)) + (p.p930 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soipdiblb_dn12 = (((p.p550 * locals.var_inv_l_dn12) + (p.p740 * locals.var_inv_w_dn12)) + (p.p930 * locals.var_inv_lw_dn12));

        let assign2000_e3613: f64 = (p.p551 * locals.var_inv_l);
        let assign2000_e3614: f64 = (p.p105 + assign2000_e3613);
        let assign2000_e3617: f64 = (p.p741 * locals.var_inv_w);
        let assign2000_e3618: f64 = (assign2000_e3614 + assign2000_e3617);
        let assign2000_e3621: f64 = (p.p931 * locals.var_inv_lw);
        let assign2000_e3622: f64 = (assign2000_e3618 + assign2000_e3621);
        locals.var_pparam_b4soidrout = assign2000_e3622;
        locals.var_pparam_b4soidrout_dn3 = (((p.p551 * locals.var_inv_l_dn3) + (p.p741 * locals.var_inv_w_dn3)) + (p.p931 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidrout_dn4 = (((p.p551 * locals.var_inv_l_dn4) + (p.p741 * locals.var_inv_w_dn4)) + (p.p931 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidrout_dn5 = (((p.p551 * locals.var_inv_l_dn5) + (p.p741 * locals.var_inv_w_dn5)) + (p.p931 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidrout_dn6 = (((p.p551 * locals.var_inv_l_dn6) + (p.p741 * locals.var_inv_w_dn6)) + (p.p931 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidrout_dn7 = (((p.p551 * locals.var_inv_l_dn7) + (p.p741 * locals.var_inv_w_dn7)) + (p.p931 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidrout_dn8 = (((p.p551 * locals.var_inv_l_dn8) + (p.p741 * locals.var_inv_w_dn8)) + (p.p931 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidrout_dn9 = (((p.p551 * locals.var_inv_l_dn9) + (p.p741 * locals.var_inv_w_dn9)) + (p.p931 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidrout_dn10 = (((p.p551 * locals.var_inv_l_dn10) + (p.p741 * locals.var_inv_w_dn10)) + (p.p931 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidrout_dn11 = (((p.p551 * locals.var_inv_l_dn11) + (p.p741 * locals.var_inv_w_dn11)) + (p.p931 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidrout_dn12 = (((p.p551 * locals.var_inv_l_dn12) + (p.p741 * locals.var_inv_w_dn12)) + (p.p931 * locals.var_inv_lw_dn12));

        let assign2010_e3626: f64 = (p.p552 * locals.var_inv_l);
        let assign2010_e3627: f64 = (p.p153 + assign2010_e3626);
        let assign2010_e3630: f64 = (p.p742 * locals.var_inv_w);
        let assign2010_e3631: f64 = (assign2010_e3627 + assign2010_e3630);
        let assign2010_e3634: f64 = (p.p932 * locals.var_inv_lw);
        let assign2010_e3635: f64 = (assign2010_e3631 + assign2010_e3634);
        locals.var_pparam_b4soipvag = assign2010_e3635;
        locals.var_pparam_b4soipvag_dn3 = (((p.p552 * locals.var_inv_l_dn3) + (p.p742 * locals.var_inv_w_dn3)) + (p.p932 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soipvag_dn4 = (((p.p552 * locals.var_inv_l_dn4) + (p.p742 * locals.var_inv_w_dn4)) + (p.p932 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soipvag_dn5 = (((p.p552 * locals.var_inv_l_dn5) + (p.p742 * locals.var_inv_w_dn5)) + (p.p932 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soipvag_dn6 = (((p.p552 * locals.var_inv_l_dn6) + (p.p742 * locals.var_inv_w_dn6)) + (p.p932 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soipvag_dn7 = (((p.p552 * locals.var_inv_l_dn7) + (p.p742 * locals.var_inv_w_dn7)) + (p.p932 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soipvag_dn8 = (((p.p552 * locals.var_inv_l_dn8) + (p.p742 * locals.var_inv_w_dn8)) + (p.p932 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soipvag_dn9 = (((p.p552 * locals.var_inv_l_dn9) + (p.p742 * locals.var_inv_w_dn9)) + (p.p932 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soipvag_dn10 = (((p.p552 * locals.var_inv_l_dn10) + (p.p742 * locals.var_inv_w_dn10)) + (p.p932 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soipvag_dn11 = (((p.p552 * locals.var_inv_l_dn11) + (p.p742 * locals.var_inv_w_dn11)) + (p.p932 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soipvag_dn12 = (((p.p552 * locals.var_inv_l_dn12) + (p.p742 * locals.var_inv_w_dn12)) + (p.p932 * locals.var_inv_lw_dn12));

        let assign2020_e3639: f64 = (p.p553 * locals.var_inv_l);
        let assign2020_e3640: f64 = (p.p130 + assign2020_e3639);
        let assign2020_e3643: f64 = (p.p743 * locals.var_inv_w);
        let assign2020_e3644: f64 = (assign2020_e3640 + assign2020_e3643);
        let assign2020_e3647: f64 = (p.p933 * locals.var_inv_lw);
        let assign2020_e3648: f64 = (assign2020_e3644 + assign2020_e3647);
        locals.var_pparam_b4soidelta = assign2020_e3648;
        locals.var_pparam_b4soidelta_dn3 = (((p.p553 * locals.var_inv_l_dn3) + (p.p743 * locals.var_inv_w_dn3)) + (p.p933 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidelta_dn4 = (((p.p553 * locals.var_inv_l_dn4) + (p.p743 * locals.var_inv_w_dn4)) + (p.p933 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidelta_dn5 = (((p.p553 * locals.var_inv_l_dn5) + (p.p743 * locals.var_inv_w_dn5)) + (p.p933 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidelta_dn6 = (((p.p553 * locals.var_inv_l_dn6) + (p.p743 * locals.var_inv_w_dn6)) + (p.p933 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidelta_dn7 = (((p.p553 * locals.var_inv_l_dn7) + (p.p743 * locals.var_inv_w_dn7)) + (p.p933 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidelta_dn8 = (((p.p553 * locals.var_inv_l_dn8) + (p.p743 * locals.var_inv_w_dn8)) + (p.p933 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidelta_dn9 = (((p.p553 * locals.var_inv_l_dn9) + (p.p743 * locals.var_inv_w_dn9)) + (p.p933 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidelta_dn10 = (((p.p553 * locals.var_inv_l_dn10) + (p.p743 * locals.var_inv_w_dn10)) + (p.p933 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidelta_dn11 = (((p.p553 * locals.var_inv_l_dn11) + (p.p743 * locals.var_inv_w_dn11)) + (p.p933 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidelta_dn12 = (((p.p553 * locals.var_inv_l_dn12) + (p.p743 * locals.var_inv_w_dn12)) + (p.p933 * locals.var_inv_lw_dn12));

        let assign2030_e3652: f64 = (p.p554 * locals.var_inv_l);
        let assign2030_e3653: f64 = (p.p218 + assign2030_e3652);
        let assign2030_e3656: f64 = (p.p744 * locals.var_inv_w);
        let assign2030_e3657: f64 = (assign2030_e3653 + assign2030_e3656);
        let assign2030_e3660: f64 = (p.p934 * locals.var_inv_lw);
        let assign2030_e3661: f64 = (assign2030_e3657 + assign2030_e3660);
        locals.var_pparam_b4soialpha0 = assign2030_e3661;
        locals.var_pparam_b4soialpha0_dn3 = (((p.p554 * locals.var_inv_l_dn3) + (p.p744 * locals.var_inv_w_dn3)) + (p.p934 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soialpha0_dn4 = (((p.p554 * locals.var_inv_l_dn4) + (p.p744 * locals.var_inv_w_dn4)) + (p.p934 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soialpha0_dn5 = (((p.p554 * locals.var_inv_l_dn5) + (p.p744 * locals.var_inv_w_dn5)) + (p.p934 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soialpha0_dn6 = (((p.p554 * locals.var_inv_l_dn6) + (p.p744 * locals.var_inv_w_dn6)) + (p.p934 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soialpha0_dn7 = (((p.p554 * locals.var_inv_l_dn7) + (p.p744 * locals.var_inv_w_dn7)) + (p.p934 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soialpha0_dn8 = (((p.p554 * locals.var_inv_l_dn8) + (p.p744 * locals.var_inv_w_dn8)) + (p.p934 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soialpha0_dn9 = (((p.p554 * locals.var_inv_l_dn9) + (p.p744 * locals.var_inv_w_dn9)) + (p.p934 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soialpha0_dn10 = (((p.p554 * locals.var_inv_l_dn10) + (p.p744 * locals.var_inv_w_dn10)) + (p.p934 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soialpha0_dn11 = (((p.p554 * locals.var_inv_l_dn11) + (p.p744 * locals.var_inv_w_dn11)) + (p.p934 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soialpha0_dn12 = (((p.p554 * locals.var_inv_l_dn12) + (p.p744 * locals.var_inv_w_dn12)) + (p.p934 * locals.var_inv_lw_dn12));

        let assign2040_e3665: f64 = (p.p555 * locals.var_inv_l);
        let assign2040_e3666: f64 = (p.p314 + assign2040_e3665);
        let assign2040_e3669: f64 = (p.p745 * locals.var_inv_w);
        let assign2040_e3670: f64 = (assign2040_e3666 + assign2040_e3669);
        let assign2040_e3673: f64 = (p.p935 * locals.var_inv_lw);
        let assign2040_e3674: f64 = (assign2040_e3670 + assign2040_e3673);
        locals.var_pparam_b4soifbjtii = assign2040_e3674;
        locals.var_pparam_b4soifbjtii_dn3 = (((p.p555 * locals.var_inv_l_dn3) + (p.p745 * locals.var_inv_w_dn3)) + (p.p935 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soifbjtii_dn4 = (((p.p555 * locals.var_inv_l_dn4) + (p.p745 * locals.var_inv_w_dn4)) + (p.p935 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soifbjtii_dn5 = (((p.p555 * locals.var_inv_l_dn5) + (p.p745 * locals.var_inv_w_dn5)) + (p.p935 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soifbjtii_dn6 = (((p.p555 * locals.var_inv_l_dn6) + (p.p745 * locals.var_inv_w_dn6)) + (p.p935 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soifbjtii_dn7 = (((p.p555 * locals.var_inv_l_dn7) + (p.p745 * locals.var_inv_w_dn7)) + (p.p935 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soifbjtii_dn8 = (((p.p555 * locals.var_inv_l_dn8) + (p.p745 * locals.var_inv_w_dn8)) + (p.p935 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soifbjtii_dn9 = (((p.p555 * locals.var_inv_l_dn9) + (p.p745 * locals.var_inv_w_dn9)) + (p.p935 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soifbjtii_dn10 = (((p.p555 * locals.var_inv_l_dn10) + (p.p745 * locals.var_inv_w_dn10)) + (p.p935 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soifbjtii_dn11 = (((p.p555 * locals.var_inv_l_dn11) + (p.p745 * locals.var_inv_w_dn11)) + (p.p935 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soifbjtii_dn12 = (((p.p555 * locals.var_inv_l_dn12) + (p.p745 * locals.var_inv_w_dn12)) + (p.p935 * locals.var_inv_lw_dn12));

        let assign2050_e3678: f64 = (p.p558 * locals.var_inv_l);
        let assign2050_e3679: f64 = (p.p315 + assign2050_e3678);
        let assign2050_e3682: f64 = (p.p748 * locals.var_inv_w);
        let assign2050_e3683: f64 = (assign2050_e3679 + assign2050_e3682);
        let assign2050_e3686: f64 = (p.p938 * locals.var_inv_lw);
        let assign2050_e3687: f64 = (assign2050_e3683 + assign2050_e3686);
        locals.var_pparam_b4soiebjtii = assign2050_e3687;
        locals.var_pparam_b4soiebjtii_dn3 = (((p.p558 * locals.var_inv_l_dn3) + (p.p748 * locals.var_inv_w_dn3)) + (p.p938 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiebjtii_dn4 = (((p.p558 * locals.var_inv_l_dn4) + (p.p748 * locals.var_inv_w_dn4)) + (p.p938 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiebjtii_dn5 = (((p.p558 * locals.var_inv_l_dn5) + (p.p748 * locals.var_inv_w_dn5)) + (p.p938 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiebjtii_dn6 = (((p.p558 * locals.var_inv_l_dn6) + (p.p748 * locals.var_inv_w_dn6)) + (p.p938 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiebjtii_dn7 = (((p.p558 * locals.var_inv_l_dn7) + (p.p748 * locals.var_inv_w_dn7)) + (p.p938 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiebjtii_dn8 = (((p.p558 * locals.var_inv_l_dn8) + (p.p748 * locals.var_inv_w_dn8)) + (p.p938 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiebjtii_dn9 = (((p.p558 * locals.var_inv_l_dn9) + (p.p748 * locals.var_inv_w_dn9)) + (p.p938 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiebjtii_dn10 = (((p.p558 * locals.var_inv_l_dn10) + (p.p748 * locals.var_inv_w_dn10)) + (p.p938 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiebjtii_dn11 = (((p.p558 * locals.var_inv_l_dn11) + (p.p748 * locals.var_inv_w_dn11)) + (p.p938 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiebjtii_dn12 = (((p.p558 * locals.var_inv_l_dn12) + (p.p748 * locals.var_inv_w_dn12)) + (p.p938 * locals.var_inv_lw_dn12));

        let assign2060_e3691: f64 = (p.p557 * locals.var_inv_l);
        let assign2060_e3692: f64 = (p.p316 + assign2060_e3691);
        let assign2060_e3695: f64 = (p.p747 * locals.var_inv_w);
        let assign2060_e3696: f64 = (assign2060_e3692 + assign2060_e3695);
        let assign2060_e3699: f64 = (p.p937 * locals.var_inv_lw);
        let assign2060_e3700: f64 = (assign2060_e3696 + assign2060_e3699);
        locals.var_pparam_b4soicbjtii = assign2060_e3700;
        locals.var_pparam_b4soicbjtii_dn3 = (((p.p557 * locals.var_inv_l_dn3) + (p.p747 * locals.var_inv_w_dn3)) + (p.p937 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soicbjtii_dn4 = (((p.p557 * locals.var_inv_l_dn4) + (p.p747 * locals.var_inv_w_dn4)) + (p.p937 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soicbjtii_dn5 = (((p.p557 * locals.var_inv_l_dn5) + (p.p747 * locals.var_inv_w_dn5)) + (p.p937 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soicbjtii_dn6 = (((p.p557 * locals.var_inv_l_dn6) + (p.p747 * locals.var_inv_w_dn6)) + (p.p937 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soicbjtii_dn7 = (((p.p557 * locals.var_inv_l_dn7) + (p.p747 * locals.var_inv_w_dn7)) + (p.p937 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soicbjtii_dn8 = (((p.p557 * locals.var_inv_l_dn8) + (p.p747 * locals.var_inv_w_dn8)) + (p.p937 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soicbjtii_dn9 = (((p.p557 * locals.var_inv_l_dn9) + (p.p747 * locals.var_inv_w_dn9)) + (p.p937 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soicbjtii_dn10 = (((p.p557 * locals.var_inv_l_dn10) + (p.p747 * locals.var_inv_w_dn10)) + (p.p937 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soicbjtii_dn11 = (((p.p557 * locals.var_inv_l_dn11) + (p.p747 * locals.var_inv_w_dn11)) + (p.p937 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soicbjtii_dn12 = (((p.p557 * locals.var_inv_l_dn12) + (p.p747 * locals.var_inv_w_dn12)) + (p.p937 * locals.var_inv_lw_dn12));

        let assign2070_e3704: f64 = (p.p560 * locals.var_inv_l);
        let assign2070_e3705: f64 = (p.p317 + assign2070_e3704);
        let assign2070_e3708: f64 = (p.p750 * locals.var_inv_w);
        let assign2070_e3709: f64 = (assign2070_e3705 + assign2070_e3708);
        let assign2070_e3712: f64 = (p.p940 * locals.var_inv_lw);
        let assign2070_e3713: f64 = (assign2070_e3709 + assign2070_e3712);
        locals.var_pparam_b4soivbci = assign2070_e3713;
        locals.var_pparam_b4soivbci_dn3 = (((p.p560 * locals.var_inv_l_dn3) + (p.p750 * locals.var_inv_w_dn3)) + (p.p940 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivbci_dn4 = (((p.p560 * locals.var_inv_l_dn4) + (p.p750 * locals.var_inv_w_dn4)) + (p.p940 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivbci_dn5 = (((p.p560 * locals.var_inv_l_dn5) + (p.p750 * locals.var_inv_w_dn5)) + (p.p940 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivbci_dn6 = (((p.p560 * locals.var_inv_l_dn6) + (p.p750 * locals.var_inv_w_dn6)) + (p.p940 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivbci_dn7 = (((p.p560 * locals.var_inv_l_dn7) + (p.p750 * locals.var_inv_w_dn7)) + (p.p940 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivbci_dn8 = (((p.p560 * locals.var_inv_l_dn8) + (p.p750 * locals.var_inv_w_dn8)) + (p.p940 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivbci_dn9 = (((p.p560 * locals.var_inv_l_dn9) + (p.p750 * locals.var_inv_w_dn9)) + (p.p940 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivbci_dn10 = (((p.p560 * locals.var_inv_l_dn10) + (p.p750 * locals.var_inv_w_dn10)) + (p.p940 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivbci_dn11 = (((p.p560 * locals.var_inv_l_dn11) + (p.p750 * locals.var_inv_w_dn11)) + (p.p940 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivbci_dn12 = (((p.p560 * locals.var_inv_l_dn12) + (p.p750 * locals.var_inv_w_dn12)) + (p.p940 * locals.var_inv_lw_dn12));

        let assign2080_e3717: f64 = (p.p556 * locals.var_inv_l);
        let assign2080_e3718: f64 = (p.p318 + assign2080_e3717);
        let assign2080_e3721: f64 = (p.p746 * locals.var_inv_w);
        let assign2080_e3722: f64 = (assign2080_e3718 + assign2080_e3721);
        let assign2080_e3725: f64 = (p.p936 * locals.var_inv_lw);
        let assign2080_e3726: f64 = (assign2080_e3722 + assign2080_e3725);
        locals.var_pparam_b4soiabjtii = assign2080_e3726;
        locals.var_pparam_b4soiabjtii_dn3 = (((p.p556 * locals.var_inv_l_dn3) + (p.p746 * locals.var_inv_w_dn3)) + (p.p936 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiabjtii_dn4 = (((p.p556 * locals.var_inv_l_dn4) + (p.p746 * locals.var_inv_w_dn4)) + (p.p936 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiabjtii_dn5 = (((p.p556 * locals.var_inv_l_dn5) + (p.p746 * locals.var_inv_w_dn5)) + (p.p936 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiabjtii_dn6 = (((p.p556 * locals.var_inv_l_dn6) + (p.p746 * locals.var_inv_w_dn6)) + (p.p936 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiabjtii_dn7 = (((p.p556 * locals.var_inv_l_dn7) + (p.p746 * locals.var_inv_w_dn7)) + (p.p936 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiabjtii_dn8 = (((p.p556 * locals.var_inv_l_dn8) + (p.p746 * locals.var_inv_w_dn8)) + (p.p936 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiabjtii_dn9 = (((p.p556 * locals.var_inv_l_dn9) + (p.p746 * locals.var_inv_w_dn9)) + (p.p936 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiabjtii_dn10 = (((p.p556 * locals.var_inv_l_dn10) + (p.p746 * locals.var_inv_w_dn10)) + (p.p936 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiabjtii_dn11 = (((p.p556 * locals.var_inv_l_dn11) + (p.p746 * locals.var_inv_w_dn11)) + (p.p936 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiabjtii_dn12 = (((p.p556 * locals.var_inv_l_dn12) + (p.p746 * locals.var_inv_w_dn12)) + (p.p936 * locals.var_inv_lw_dn12));

        let assign2090_e3730: f64 = (p.p559 * locals.var_inv_l);
        let assign2090_e3731: f64 = (p.p319 + assign2090_e3730);
        let assign2090_e3734: f64 = (p.p749 * locals.var_inv_w);
        let assign2090_e3735: f64 = (assign2090_e3731 + assign2090_e3734);
        let assign2090_e3738: f64 = (p.p939 * locals.var_inv_lw);
        let assign2090_e3739: f64 = (assign2090_e3735 + assign2090_e3738);
        locals.var_pparam_b4soimbjtii = assign2090_e3739;
        locals.var_pparam_b4soimbjtii_dn3 = (((p.p559 * locals.var_inv_l_dn3) + (p.p749 * locals.var_inv_w_dn3)) + (p.p939 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soimbjtii_dn4 = (((p.p559 * locals.var_inv_l_dn4) + (p.p749 * locals.var_inv_w_dn4)) + (p.p939 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soimbjtii_dn5 = (((p.p559 * locals.var_inv_l_dn5) + (p.p749 * locals.var_inv_w_dn5)) + (p.p939 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soimbjtii_dn6 = (((p.p559 * locals.var_inv_l_dn6) + (p.p749 * locals.var_inv_w_dn6)) + (p.p939 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soimbjtii_dn7 = (((p.p559 * locals.var_inv_l_dn7) + (p.p749 * locals.var_inv_w_dn7)) + (p.p939 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soimbjtii_dn8 = (((p.p559 * locals.var_inv_l_dn8) + (p.p749 * locals.var_inv_w_dn8)) + (p.p939 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soimbjtii_dn9 = (((p.p559 * locals.var_inv_l_dn9) + (p.p749 * locals.var_inv_w_dn9)) + (p.p939 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soimbjtii_dn10 = (((p.p559 * locals.var_inv_l_dn10) + (p.p749 * locals.var_inv_w_dn10)) + (p.p939 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soimbjtii_dn11 = (((p.p559 * locals.var_inv_l_dn11) + (p.p749 * locals.var_inv_w_dn11)) + (p.p939 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soimbjtii_dn12 = (((p.p559 * locals.var_inv_l_dn12) + (p.p749 * locals.var_inv_w_dn12)) + (p.p939 * locals.var_inv_lw_dn12));

        let assign2100_e3743: f64 = (p.p561 * locals.var_inv_l);
        let assign2100_e3744: f64 = (p.p304 + assign2100_e3743);
        let assign2100_e3747: f64 = (p.p751 * locals.var_inv_w);
        let assign2100_e3748: f64 = (assign2100_e3744 + assign2100_e3747);
        let assign2100_e3751: f64 = (p.p941 * locals.var_inv_lw);
        let assign2100_e3752: f64 = (assign2100_e3748 + assign2100_e3751);
        locals.var_pparam_b4soibeta0 = assign2100_e3752;
        locals.var_pparam_b4soibeta0_dn3 = (((p.p561 * locals.var_inv_l_dn3) + (p.p751 * locals.var_inv_w_dn3)) + (p.p941 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soibeta0_dn4 = (((p.p561 * locals.var_inv_l_dn4) + (p.p751 * locals.var_inv_w_dn4)) + (p.p941 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soibeta0_dn5 = (((p.p561 * locals.var_inv_l_dn5) + (p.p751 * locals.var_inv_w_dn5)) + (p.p941 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soibeta0_dn6 = (((p.p561 * locals.var_inv_l_dn6) + (p.p751 * locals.var_inv_w_dn6)) + (p.p941 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soibeta0_dn7 = (((p.p561 * locals.var_inv_l_dn7) + (p.p751 * locals.var_inv_w_dn7)) + (p.p941 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soibeta0_dn8 = (((p.p561 * locals.var_inv_l_dn8) + (p.p751 * locals.var_inv_w_dn8)) + (p.p941 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soibeta0_dn9 = (((p.p561 * locals.var_inv_l_dn9) + (p.p751 * locals.var_inv_w_dn9)) + (p.p941 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soibeta0_dn10 = (((p.p561 * locals.var_inv_l_dn10) + (p.p751 * locals.var_inv_w_dn10)) + (p.p941 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soibeta0_dn11 = (((p.p561 * locals.var_inv_l_dn11) + (p.p751 * locals.var_inv_w_dn11)) + (p.p941 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soibeta0_dn12 = (((p.p561 * locals.var_inv_l_dn12) + (p.p751 * locals.var_inv_w_dn12)) + (p.p941 * locals.var_inv_lw_dn12));

        let assign2110_e3756: f64 = (p.p562 * locals.var_inv_l);
        let assign2110_e3757: f64 = (p.p305 + assign2110_e3756);
        let assign2110_e3760: f64 = (p.p752 * locals.var_inv_w);
        let assign2110_e3761: f64 = (assign2110_e3757 + assign2110_e3760);
        let assign2110_e3764: f64 = (p.p942 * locals.var_inv_lw);
        let assign2110_e3765: f64 = (assign2110_e3761 + assign2110_e3764);
        locals.var_pparam_b4soibeta1 = assign2110_e3765;
        locals.var_pparam_b4soibeta1_dn3 = (((p.p562 * locals.var_inv_l_dn3) + (p.p752 * locals.var_inv_w_dn3)) + (p.p942 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soibeta1_dn4 = (((p.p562 * locals.var_inv_l_dn4) + (p.p752 * locals.var_inv_w_dn4)) + (p.p942 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soibeta1_dn5 = (((p.p562 * locals.var_inv_l_dn5) + (p.p752 * locals.var_inv_w_dn5)) + (p.p942 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soibeta1_dn6 = (((p.p562 * locals.var_inv_l_dn6) + (p.p752 * locals.var_inv_w_dn6)) + (p.p942 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soibeta1_dn7 = (((p.p562 * locals.var_inv_l_dn7) + (p.p752 * locals.var_inv_w_dn7)) + (p.p942 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soibeta1_dn8 = (((p.p562 * locals.var_inv_l_dn8) + (p.p752 * locals.var_inv_w_dn8)) + (p.p942 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soibeta1_dn9 = (((p.p562 * locals.var_inv_l_dn9) + (p.p752 * locals.var_inv_w_dn9)) + (p.p942 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soibeta1_dn10 = (((p.p562 * locals.var_inv_l_dn10) + (p.p752 * locals.var_inv_w_dn10)) + (p.p942 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soibeta1_dn11 = (((p.p562 * locals.var_inv_l_dn11) + (p.p752 * locals.var_inv_w_dn11)) + (p.p942 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soibeta1_dn12 = (((p.p562 * locals.var_inv_l_dn12) + (p.p752 * locals.var_inv_w_dn12)) + (p.p942 * locals.var_inv_lw_dn12));

        let assign2120_e3769: f64 = (p.p563 * locals.var_inv_l);
        let assign2120_e3770: f64 = (p.p306 + assign2120_e3769);
        let assign2120_e3773: f64 = (p.p753 * locals.var_inv_w);
        let assign2120_e3774: f64 = (assign2120_e3770 + assign2120_e3773);
        let assign2120_e3777: f64 = (p.p943 * locals.var_inv_lw);
        let assign2120_e3778: f64 = (assign2120_e3774 + assign2120_e3777);
        locals.var_pparam_b4soibeta2 = assign2120_e3778;
        locals.var_pparam_b4soibeta2_dn3 = (((p.p563 * locals.var_inv_l_dn3) + (p.p753 * locals.var_inv_w_dn3)) + (p.p943 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soibeta2_dn4 = (((p.p563 * locals.var_inv_l_dn4) + (p.p753 * locals.var_inv_w_dn4)) + (p.p943 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soibeta2_dn5 = (((p.p563 * locals.var_inv_l_dn5) + (p.p753 * locals.var_inv_w_dn5)) + (p.p943 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soibeta2_dn6 = (((p.p563 * locals.var_inv_l_dn6) + (p.p753 * locals.var_inv_w_dn6)) + (p.p943 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soibeta2_dn7 = (((p.p563 * locals.var_inv_l_dn7) + (p.p753 * locals.var_inv_w_dn7)) + (p.p943 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soibeta2_dn8 = (((p.p563 * locals.var_inv_l_dn8) + (p.p753 * locals.var_inv_w_dn8)) + (p.p943 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soibeta2_dn9 = (((p.p563 * locals.var_inv_l_dn9) + (p.p753 * locals.var_inv_w_dn9)) + (p.p943 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soibeta2_dn10 = (((p.p563 * locals.var_inv_l_dn10) + (p.p753 * locals.var_inv_w_dn10)) + (p.p943 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soibeta2_dn11 = (((p.p563 * locals.var_inv_l_dn11) + (p.p753 * locals.var_inv_w_dn11)) + (p.p943 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soibeta2_dn12 = (((p.p563 * locals.var_inv_l_dn12) + (p.p753 * locals.var_inv_w_dn12)) + (p.p943 * locals.var_inv_lw_dn12));

        let assign2130_e3782: f64 = (p.p564 * locals.var_inv_l);
        let assign2130_e3783: f64 = (p.p307 + assign2130_e3782);
        let assign2130_e3786: f64 = (p.p754 * locals.var_inv_w);
        let assign2130_e3787: f64 = (assign2130_e3783 + assign2130_e3786);
        let assign2130_e3790: f64 = (p.p944 * locals.var_inv_lw);
        let assign2130_e3791: f64 = (assign2130_e3787 + assign2130_e3790);
        locals.var_pparam_b4soivdsatii0 = assign2130_e3791;
        locals.var_pparam_b4soivdsatii0_dn3 = (((p.p564 * locals.var_inv_l_dn3) + (p.p754 * locals.var_inv_w_dn3)) + (p.p944 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivdsatii0_dn4 = (((p.p564 * locals.var_inv_l_dn4) + (p.p754 * locals.var_inv_w_dn4)) + (p.p944 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivdsatii0_dn5 = (((p.p564 * locals.var_inv_l_dn5) + (p.p754 * locals.var_inv_w_dn5)) + (p.p944 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivdsatii0_dn6 = (((p.p564 * locals.var_inv_l_dn6) + (p.p754 * locals.var_inv_w_dn6)) + (p.p944 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivdsatii0_dn7 = (((p.p564 * locals.var_inv_l_dn7) + (p.p754 * locals.var_inv_w_dn7)) + (p.p944 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivdsatii0_dn8 = (((p.p564 * locals.var_inv_l_dn8) + (p.p754 * locals.var_inv_w_dn8)) + (p.p944 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivdsatii0_dn9 = (((p.p564 * locals.var_inv_l_dn9) + (p.p754 * locals.var_inv_w_dn9)) + (p.p944 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivdsatii0_dn10 = (((p.p564 * locals.var_inv_l_dn10) + (p.p754 * locals.var_inv_w_dn10)) + (p.p944 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivdsatii0_dn11 = (((p.p564 * locals.var_inv_l_dn11) + (p.p754 * locals.var_inv_w_dn11)) + (p.p944 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivdsatii0_dn12 = (((p.p564 * locals.var_inv_l_dn12) + (p.p754 * locals.var_inv_w_dn12)) + (p.p944 * locals.var_inv_lw_dn12));

        let assign2140_e3795: f64 = (p.p565 * locals.var_inv_l);
        let assign2140_e3796: f64 = (p.p309 + assign2140_e3795);
        let assign2140_e3799: f64 = (p.p755 * locals.var_inv_w);
        let assign2140_e3800: f64 = (assign2140_e3796 + assign2140_e3799);
        let assign2140_e3803: f64 = (p.p945 * locals.var_inv_lw);
        let assign2140_e3804: f64 = (assign2140_e3800 + assign2140_e3803);
        locals.var_pparam_b4soilii = assign2140_e3804;
        locals.var_pparam_b4soilii_dn3 = (((p.p565 * locals.var_inv_l_dn3) + (p.p755 * locals.var_inv_w_dn3)) + (p.p945 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soilii_dn4 = (((p.p565 * locals.var_inv_l_dn4) + (p.p755 * locals.var_inv_w_dn4)) + (p.p945 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soilii_dn5 = (((p.p565 * locals.var_inv_l_dn5) + (p.p755 * locals.var_inv_w_dn5)) + (p.p945 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soilii_dn6 = (((p.p565 * locals.var_inv_l_dn6) + (p.p755 * locals.var_inv_w_dn6)) + (p.p945 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soilii_dn7 = (((p.p565 * locals.var_inv_l_dn7) + (p.p755 * locals.var_inv_w_dn7)) + (p.p945 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soilii_dn8 = (((p.p565 * locals.var_inv_l_dn8) + (p.p755 * locals.var_inv_w_dn8)) + (p.p945 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soilii_dn9 = (((p.p565 * locals.var_inv_l_dn9) + (p.p755 * locals.var_inv_w_dn9)) + (p.p945 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soilii_dn10 = (((p.p565 * locals.var_inv_l_dn10) + (p.p755 * locals.var_inv_w_dn10)) + (p.p945 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soilii_dn11 = (((p.p565 * locals.var_inv_l_dn11) + (p.p755 * locals.var_inv_w_dn11)) + (p.p945 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soilii_dn12 = (((p.p565 * locals.var_inv_l_dn12) + (p.p755 * locals.var_inv_w_dn12)) + (p.p945 * locals.var_inv_lw_dn12));

        let assign2150_e3808: f64 = (p.p566 * locals.var_inv_l);
        let assign2150_e3809: f64 = (p.p321 + assign2150_e3808);
        let assign2150_e3812: f64 = (p.p756 * locals.var_inv_w);
        let assign2150_e3813: f64 = (assign2150_e3809 + assign2150_e3812);
        let assign2150_e3816: f64 = (p.p946 * locals.var_inv_lw);
        let assign2150_e3817: f64 = (assign2150_e3813 + assign2150_e3816);
        locals.var_pparam_b4soiesatii = assign2150_e3817;
        locals.var_pparam_b4soiesatii_dn3 = (((p.p566 * locals.var_inv_l_dn3) + (p.p756 * locals.var_inv_w_dn3)) + (p.p946 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiesatii_dn4 = (((p.p566 * locals.var_inv_l_dn4) + (p.p756 * locals.var_inv_w_dn4)) + (p.p946 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiesatii_dn5 = (((p.p566 * locals.var_inv_l_dn5) + (p.p756 * locals.var_inv_w_dn5)) + (p.p946 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiesatii_dn6 = (((p.p566 * locals.var_inv_l_dn6) + (p.p756 * locals.var_inv_w_dn6)) + (p.p946 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiesatii_dn7 = (((p.p566 * locals.var_inv_l_dn7) + (p.p756 * locals.var_inv_w_dn7)) + (p.p946 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiesatii_dn8 = (((p.p566 * locals.var_inv_l_dn8) + (p.p756 * locals.var_inv_w_dn8)) + (p.p946 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiesatii_dn9 = (((p.p566 * locals.var_inv_l_dn9) + (p.p756 * locals.var_inv_w_dn9)) + (p.p946 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiesatii_dn10 = (((p.p566 * locals.var_inv_l_dn10) + (p.p756 * locals.var_inv_w_dn10)) + (p.p946 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiesatii_dn11 = (((p.p566 * locals.var_inv_l_dn11) + (p.p756 * locals.var_inv_w_dn11)) + (p.p946 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiesatii_dn12 = (((p.p566 * locals.var_inv_l_dn12) + (p.p756 * locals.var_inv_w_dn12)) + (p.p946 * locals.var_inv_lw_dn12));

        let assign2160_e3821: f64 = (p.p567 * locals.var_inv_l);
        let assign2160_e3822: f64 = (p.p310 + assign2160_e3821);
        let assign2160_e3825: f64 = (p.p757 * locals.var_inv_w);
        let assign2160_e3826: f64 = (assign2160_e3822 + assign2160_e3825);
        let assign2160_e3829: f64 = (p.p947 * locals.var_inv_lw);
        let assign2160_e3830: f64 = (assign2160_e3826 + assign2160_e3829);
        locals.var_pparam_b4soisii0 = assign2160_e3830;
        locals.var_pparam_b4soisii0_dn3 = (((p.p567 * locals.var_inv_l_dn3) + (p.p757 * locals.var_inv_w_dn3)) + (p.p947 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soisii0_dn4 = (((p.p567 * locals.var_inv_l_dn4) + (p.p757 * locals.var_inv_w_dn4)) + (p.p947 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soisii0_dn5 = (((p.p567 * locals.var_inv_l_dn5) + (p.p757 * locals.var_inv_w_dn5)) + (p.p947 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soisii0_dn6 = (((p.p567 * locals.var_inv_l_dn6) + (p.p757 * locals.var_inv_w_dn6)) + (p.p947 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soisii0_dn7 = (((p.p567 * locals.var_inv_l_dn7) + (p.p757 * locals.var_inv_w_dn7)) + (p.p947 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soisii0_dn8 = (((p.p567 * locals.var_inv_l_dn8) + (p.p757 * locals.var_inv_w_dn8)) + (p.p947 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soisii0_dn9 = (((p.p567 * locals.var_inv_l_dn9) + (p.p757 * locals.var_inv_w_dn9)) + (p.p947 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soisii0_dn10 = (((p.p567 * locals.var_inv_l_dn10) + (p.p757 * locals.var_inv_w_dn10)) + (p.p947 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soisii0_dn11 = (((p.p567 * locals.var_inv_l_dn11) + (p.p757 * locals.var_inv_w_dn11)) + (p.p947 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soisii0_dn12 = (((p.p567 * locals.var_inv_l_dn12) + (p.p757 * locals.var_inv_w_dn12)) + (p.p947 * locals.var_inv_lw_dn12));

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2170_e3834: f64 = (p.p568 * locals.var_inv_l);
        let assign2170_e3835: f64 = (p.p311 + assign2170_e3834);
        let assign2170_e3838: f64 = (p.p758 * locals.var_inv_w);
        let assign2170_e3839: f64 = (assign2170_e3835 + assign2170_e3838);
        let assign2170_e3842: f64 = (p.p948 * locals.var_inv_lw);
        let assign2170_e3843: f64 = (assign2170_e3839 + assign2170_e3842);
        locals.var_pparam_b4soisii1 = assign2170_e3843;
        locals.var_pparam_b4soisii1_dn3 = (((p.p568 * locals.var_inv_l_dn3) + (p.p758 * locals.var_inv_w_dn3)) + (p.p948 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soisii1_dn4 = (((p.p568 * locals.var_inv_l_dn4) + (p.p758 * locals.var_inv_w_dn4)) + (p.p948 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soisii1_dn5 = (((p.p568 * locals.var_inv_l_dn5) + (p.p758 * locals.var_inv_w_dn5)) + (p.p948 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soisii1_dn6 = (((p.p568 * locals.var_inv_l_dn6) + (p.p758 * locals.var_inv_w_dn6)) + (p.p948 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soisii1_dn7 = (((p.p568 * locals.var_inv_l_dn7) + (p.p758 * locals.var_inv_w_dn7)) + (p.p948 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soisii1_dn8 = (((p.p568 * locals.var_inv_l_dn8) + (p.p758 * locals.var_inv_w_dn8)) + (p.p948 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soisii1_dn9 = (((p.p568 * locals.var_inv_l_dn9) + (p.p758 * locals.var_inv_w_dn9)) + (p.p948 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soisii1_dn10 = (((p.p568 * locals.var_inv_l_dn10) + (p.p758 * locals.var_inv_w_dn10)) + (p.p948 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soisii1_dn11 = (((p.p568 * locals.var_inv_l_dn11) + (p.p758 * locals.var_inv_w_dn11)) + (p.p948 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soisii1_dn12 = (((p.p568 * locals.var_inv_l_dn12) + (p.p758 * locals.var_inv_w_dn12)) + (p.p948 * locals.var_inv_lw_dn12));

        let assign2180_e3847: f64 = (p.p569 * locals.var_inv_l);
        let assign2180_e3848: f64 = (p.p312 + assign2180_e3847);
        let assign2180_e3851: f64 = (p.p759 * locals.var_inv_w);
        let assign2180_e3852: f64 = (assign2180_e3848 + assign2180_e3851);
        let assign2180_e3855: f64 = (p.p949 * locals.var_inv_lw);
        let assign2180_e3856: f64 = (assign2180_e3852 + assign2180_e3855);
        locals.var_pparam_b4soisii2 = assign2180_e3856;
        locals.var_pparam_b4soisii2_dn3 = (((p.p569 * locals.var_inv_l_dn3) + (p.p759 * locals.var_inv_w_dn3)) + (p.p949 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soisii2_dn4 = (((p.p569 * locals.var_inv_l_dn4) + (p.p759 * locals.var_inv_w_dn4)) + (p.p949 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soisii2_dn5 = (((p.p569 * locals.var_inv_l_dn5) + (p.p759 * locals.var_inv_w_dn5)) + (p.p949 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soisii2_dn6 = (((p.p569 * locals.var_inv_l_dn6) + (p.p759 * locals.var_inv_w_dn6)) + (p.p949 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soisii2_dn7 = (((p.p569 * locals.var_inv_l_dn7) + (p.p759 * locals.var_inv_w_dn7)) + (p.p949 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soisii2_dn8 = (((p.p569 * locals.var_inv_l_dn8) + (p.p759 * locals.var_inv_w_dn8)) + (p.p949 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soisii2_dn9 = (((p.p569 * locals.var_inv_l_dn9) + (p.p759 * locals.var_inv_w_dn9)) + (p.p949 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soisii2_dn10 = (((p.p569 * locals.var_inv_l_dn10) + (p.p759 * locals.var_inv_w_dn10)) + (p.p949 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soisii2_dn11 = (((p.p569 * locals.var_inv_l_dn11) + (p.p759 * locals.var_inv_w_dn11)) + (p.p949 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soisii2_dn12 = (((p.p569 * locals.var_inv_l_dn12) + (p.p759 * locals.var_inv_w_dn12)) + (p.p949 * locals.var_inv_lw_dn12));

        let assign2190_e3860: f64 = (p.p570 * locals.var_inv_l);
        let assign2190_e3861: f64 = (p.p313 + assign2190_e3860);
        let assign2190_e3864: f64 = (p.p760 * locals.var_inv_w);
        let assign2190_e3865: f64 = (assign2190_e3861 + assign2190_e3864);
        let assign2190_e3868: f64 = (p.p950 * locals.var_inv_lw);
        let assign2190_e3869: f64 = (assign2190_e3865 + assign2190_e3868);
        locals.var_pparam_b4soisiid = assign2190_e3869;
        locals.var_pparam_b4soisiid_dn3 = (((p.p570 * locals.var_inv_l_dn3) + (p.p760 * locals.var_inv_w_dn3)) + (p.p950 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soisiid_dn4 = (((p.p570 * locals.var_inv_l_dn4) + (p.p760 * locals.var_inv_w_dn4)) + (p.p950 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soisiid_dn5 = (((p.p570 * locals.var_inv_l_dn5) + (p.p760 * locals.var_inv_w_dn5)) + (p.p950 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soisiid_dn6 = (((p.p570 * locals.var_inv_l_dn6) + (p.p760 * locals.var_inv_w_dn6)) + (p.p950 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soisiid_dn7 = (((p.p570 * locals.var_inv_l_dn7) + (p.p760 * locals.var_inv_w_dn7)) + (p.p950 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soisiid_dn8 = (((p.p570 * locals.var_inv_l_dn8) + (p.p760 * locals.var_inv_w_dn8)) + (p.p950 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soisiid_dn9 = (((p.p570 * locals.var_inv_l_dn9) + (p.p760 * locals.var_inv_w_dn9)) + (p.p950 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soisiid_dn10 = (((p.p570 * locals.var_inv_l_dn10) + (p.p760 * locals.var_inv_w_dn10)) + (p.p950 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soisiid_dn11 = (((p.p570 * locals.var_inv_l_dn11) + (p.p760 * locals.var_inv_w_dn11)) + (p.p950 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soisiid_dn12 = (((p.p570 * locals.var_inv_l_dn12) + (p.p760 * locals.var_inv_w_dn12)) + (p.p950 * locals.var_inv_lw_dn12));

        let assign2200_e3873: f64 = (p.p571 * locals.var_inv_l);
        let assign2200_e3874: f64 = (p.p158 + assign2200_e3873);
        let assign2200_e3877: f64 = (p.p761 * locals.var_inv_w);
        let assign2200_e3878: f64 = (assign2200_e3874 + assign2200_e3877);
        let assign2200_e3881: f64 = (p.p951 * locals.var_inv_lw);
        let assign2200_e3882: f64 = (assign2200_e3878 + assign2200_e3881);
        locals.var_pparam_b4soiagidl = assign2200_e3882;
        locals.var_pparam_b4soiagidl_dn3 = (((p.p571 * locals.var_inv_l_dn3) + (p.p761 * locals.var_inv_w_dn3)) + (p.p951 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiagidl_dn4 = (((p.p571 * locals.var_inv_l_dn4) + (p.p761 * locals.var_inv_w_dn4)) + (p.p951 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiagidl_dn5 = (((p.p571 * locals.var_inv_l_dn5) + (p.p761 * locals.var_inv_w_dn5)) + (p.p951 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiagidl_dn6 = (((p.p571 * locals.var_inv_l_dn6) + (p.p761 * locals.var_inv_w_dn6)) + (p.p951 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiagidl_dn7 = (((p.p571 * locals.var_inv_l_dn7) + (p.p761 * locals.var_inv_w_dn7)) + (p.p951 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiagidl_dn8 = (((p.p571 * locals.var_inv_l_dn8) + (p.p761 * locals.var_inv_w_dn8)) + (p.p951 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiagidl_dn9 = (((p.p571 * locals.var_inv_l_dn9) + (p.p761 * locals.var_inv_w_dn9)) + (p.p951 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiagidl_dn10 = (((p.p571 * locals.var_inv_l_dn10) + (p.p761 * locals.var_inv_w_dn10)) + (p.p951 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiagidl_dn11 = (((p.p571 * locals.var_inv_l_dn11) + (p.p761 * locals.var_inv_w_dn11)) + (p.p951 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiagidl_dn12 = (((p.p571 * locals.var_inv_l_dn12) + (p.p761 * locals.var_inv_w_dn12)) + (p.p951 * locals.var_inv_lw_dn12));

        let assign2210_e3886: f64 = (p.p572 * locals.var_inv_l);
        let assign2210_e3887: f64 = (p.p159 + assign2210_e3886);
        let assign2210_e3890: f64 = (p.p762 * locals.var_inv_w);
        let assign2210_e3891: f64 = (assign2210_e3887 + assign2210_e3890);
        let assign2210_e3894: f64 = (p.p952 * locals.var_inv_lw);
        let assign2210_e3895: f64 = (assign2210_e3891 + assign2210_e3894);
        locals.var_pparam_b4soibgidl = assign2210_e3895;
        locals.var_pparam_b4soibgidl_dn3 = (((p.p572 * locals.var_inv_l_dn3) + (p.p762 * locals.var_inv_w_dn3)) + (p.p952 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soibgidl_dn4 = (((p.p572 * locals.var_inv_l_dn4) + (p.p762 * locals.var_inv_w_dn4)) + (p.p952 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soibgidl_dn5 = (((p.p572 * locals.var_inv_l_dn5) + (p.p762 * locals.var_inv_w_dn5)) + (p.p952 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soibgidl_dn6 = (((p.p572 * locals.var_inv_l_dn6) + (p.p762 * locals.var_inv_w_dn6)) + (p.p952 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soibgidl_dn7 = (((p.p572 * locals.var_inv_l_dn7) + (p.p762 * locals.var_inv_w_dn7)) + (p.p952 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soibgidl_dn8 = (((p.p572 * locals.var_inv_l_dn8) + (p.p762 * locals.var_inv_w_dn8)) + (p.p952 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soibgidl_dn9 = (((p.p572 * locals.var_inv_l_dn9) + (p.p762 * locals.var_inv_w_dn9)) + (p.p952 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soibgidl_dn10 = (((p.p572 * locals.var_inv_l_dn10) + (p.p762 * locals.var_inv_w_dn10)) + (p.p952 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soibgidl_dn11 = (((p.p572 * locals.var_inv_l_dn11) + (p.p762 * locals.var_inv_w_dn11)) + (p.p952 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soibgidl_dn12 = (((p.p572 * locals.var_inv_l_dn12) + (p.p762 * locals.var_inv_w_dn12)) + (p.p952 * locals.var_inv_lw_dn12));

        let assign2220_e3899: f64 = (p.p573 * locals.var_inv_l);
        let assign2220_e3900: f64 = (p.p160 + assign2220_e3899);
        let assign2220_e3903: f64 = (p.p763 * locals.var_inv_w);
        let assign2220_e3904: f64 = (assign2220_e3900 + assign2220_e3903);
        let assign2220_e3907: f64 = (p.p953 * locals.var_inv_lw);
        let assign2220_e3908: f64 = (assign2220_e3904 + assign2220_e3907);
        locals.var_pparam_b4soibgidl1 = assign2220_e3908;
        locals.var_pparam_b4soibgidl1_dn3 = (((p.p573 * locals.var_inv_l_dn3) + (p.p763 * locals.var_inv_w_dn3)) + (p.p953 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soibgidl1_dn4 = (((p.p573 * locals.var_inv_l_dn4) + (p.p763 * locals.var_inv_w_dn4)) + (p.p953 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soibgidl1_dn5 = (((p.p573 * locals.var_inv_l_dn5) + (p.p763 * locals.var_inv_w_dn5)) + (p.p953 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soibgidl1_dn6 = (((p.p573 * locals.var_inv_l_dn6) + (p.p763 * locals.var_inv_w_dn6)) + (p.p953 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soibgidl1_dn7 = (((p.p573 * locals.var_inv_l_dn7) + (p.p763 * locals.var_inv_w_dn7)) + (p.p953 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soibgidl1_dn8 = (((p.p573 * locals.var_inv_l_dn8) + (p.p763 * locals.var_inv_w_dn8)) + (p.p953 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soibgidl1_dn9 = (((p.p573 * locals.var_inv_l_dn9) + (p.p763 * locals.var_inv_w_dn9)) + (p.p953 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soibgidl1_dn10 = (((p.p573 * locals.var_inv_l_dn10) + (p.p763 * locals.var_inv_w_dn10)) + (p.p953 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soibgidl1_dn11 = (((p.p573 * locals.var_inv_l_dn11) + (p.p763 * locals.var_inv_w_dn11)) + (p.p953 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soibgidl1_dn12 = (((p.p573 * locals.var_inv_l_dn12) + (p.p763 * locals.var_inv_w_dn12)) + (p.p953 * locals.var_inv_lw_dn12));

        let assign2230_e3912: f64 = (p.p574 * locals.var_inv_l);
        let assign2230_e3913: f64 = (p.p161 + assign2230_e3912);
        let assign2230_e3916: f64 = (p.p764 * locals.var_inv_w);
        let assign2230_e3917: f64 = (assign2230_e3913 + assign2230_e3916);
        let assign2230_e3920: f64 = (p.p954 * locals.var_inv_lw);
        let assign2230_e3921: f64 = (assign2230_e3917 + assign2230_e3920);
        locals.var_pparam_b4soicgidl = assign2230_e3921;
        locals.var_pparam_b4soicgidl_dn3 = (((p.p574 * locals.var_inv_l_dn3) + (p.p764 * locals.var_inv_w_dn3)) + (p.p954 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soicgidl_dn4 = (((p.p574 * locals.var_inv_l_dn4) + (p.p764 * locals.var_inv_w_dn4)) + (p.p954 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soicgidl_dn5 = (((p.p574 * locals.var_inv_l_dn5) + (p.p764 * locals.var_inv_w_dn5)) + (p.p954 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soicgidl_dn6 = (((p.p574 * locals.var_inv_l_dn6) + (p.p764 * locals.var_inv_w_dn6)) + (p.p954 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soicgidl_dn7 = (((p.p574 * locals.var_inv_l_dn7) + (p.p764 * locals.var_inv_w_dn7)) + (p.p954 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soicgidl_dn8 = (((p.p574 * locals.var_inv_l_dn8) + (p.p764 * locals.var_inv_w_dn8)) + (p.p954 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soicgidl_dn9 = (((p.p574 * locals.var_inv_l_dn9) + (p.p764 * locals.var_inv_w_dn9)) + (p.p954 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soicgidl_dn10 = (((p.p574 * locals.var_inv_l_dn10) + (p.p764 * locals.var_inv_w_dn10)) + (p.p954 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soicgidl_dn11 = (((p.p574 * locals.var_inv_l_dn11) + (p.p764 * locals.var_inv_w_dn11)) + (p.p954 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soicgidl_dn12 = (((p.p574 * locals.var_inv_l_dn12) + (p.p764 * locals.var_inv_w_dn12)) + (p.p954 * locals.var_inv_lw_dn12));

        let assign2240_e3925: f64 = (p.p1025 * locals.var_inv_l);
        let assign2240_e3926: f64 = (p.p1022 + assign2240_e3925);
        let assign2240_e3929: f64 = (p.p1028 * locals.var_inv_w);
        let assign2240_e3930: f64 = (assign2240_e3926 + assign2240_e3929);
        let assign2240_e3933: f64 = (p.p1031 * locals.var_inv_lw);
        let assign2240_e3934: f64 = (assign2240_e3930 + assign2240_e3933);
        locals.var_pparam_b4soiegidl = assign2240_e3934;
        locals.var_pparam_b4soiegidl_dn3 = (((p.p1025 * locals.var_inv_l_dn3) + (p.p1028 * locals.var_inv_w_dn3)) + (p.p1031 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiegidl_dn4 = (((p.p1025 * locals.var_inv_l_dn4) + (p.p1028 * locals.var_inv_w_dn4)) + (p.p1031 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiegidl_dn5 = (((p.p1025 * locals.var_inv_l_dn5) + (p.p1028 * locals.var_inv_w_dn5)) + (p.p1031 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiegidl_dn6 = (((p.p1025 * locals.var_inv_l_dn6) + (p.p1028 * locals.var_inv_w_dn6)) + (p.p1031 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiegidl_dn7 = (((p.p1025 * locals.var_inv_l_dn7) + (p.p1028 * locals.var_inv_w_dn7)) + (p.p1031 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiegidl_dn8 = (((p.p1025 * locals.var_inv_l_dn8) + (p.p1028 * locals.var_inv_w_dn8)) + (p.p1031 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiegidl_dn9 = (((p.p1025 * locals.var_inv_l_dn9) + (p.p1028 * locals.var_inv_w_dn9)) + (p.p1031 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiegidl_dn10 = (((p.p1025 * locals.var_inv_l_dn10) + (p.p1028 * locals.var_inv_w_dn10)) + (p.p1031 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiegidl_dn11 = (((p.p1025 * locals.var_inv_l_dn11) + (p.p1028 * locals.var_inv_w_dn11)) + (p.p1031 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiegidl_dn12 = (((p.p1025 * locals.var_inv_l_dn12) + (p.p1028 * locals.var_inv_w_dn12)) + (p.p1031 * locals.var_inv_lw_dn12));

        let assign2250_e3938: f64 = (p.p575 * locals.var_inv_l);
        let assign2250_e3939: f64 = (p.p162 + assign2250_e3938);
        let assign2250_e3942: f64 = (p.p765 * locals.var_inv_w);
        let assign2250_e3943: f64 = (assign2250_e3939 + assign2250_e3942);
        let assign2250_e3946: f64 = (p.p955 * locals.var_inv_lw);
        let assign2250_e3947: f64 = (assign2250_e3943 + assign2250_e3946);
        locals.var_pparam_b4soirgidl = assign2250_e3947;
        locals.var_pparam_b4soirgidl_dn3 = (((p.p575 * locals.var_inv_l_dn3) + (p.p765 * locals.var_inv_w_dn3)) + (p.p955 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soirgidl_dn4 = (((p.p575 * locals.var_inv_l_dn4) + (p.p765 * locals.var_inv_w_dn4)) + (p.p955 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soirgidl_dn5 = (((p.p575 * locals.var_inv_l_dn5) + (p.p765 * locals.var_inv_w_dn5)) + (p.p955 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soirgidl_dn6 = (((p.p575 * locals.var_inv_l_dn6) + (p.p765 * locals.var_inv_w_dn6)) + (p.p955 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soirgidl_dn7 = (((p.p575 * locals.var_inv_l_dn7) + (p.p765 * locals.var_inv_w_dn7)) + (p.p955 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soirgidl_dn8 = (((p.p575 * locals.var_inv_l_dn8) + (p.p765 * locals.var_inv_w_dn8)) + (p.p955 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soirgidl_dn9 = (((p.p575 * locals.var_inv_l_dn9) + (p.p765 * locals.var_inv_w_dn9)) + (p.p955 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soirgidl_dn10 = (((p.p575 * locals.var_inv_l_dn10) + (p.p765 * locals.var_inv_w_dn10)) + (p.p955 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soirgidl_dn11 = (((p.p575 * locals.var_inv_l_dn11) + (p.p765 * locals.var_inv_w_dn11)) + (p.p955 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soirgidl_dn12 = (((p.p575 * locals.var_inv_l_dn12) + (p.p765 * locals.var_inv_w_dn12)) + (p.p955 * locals.var_inv_lw_dn12));

        let assign2260_e3951: f64 = (p.p576 * locals.var_inv_l);
        let assign2260_e3952: f64 = (p.p163 + assign2260_e3951);
        let assign2260_e3955: f64 = (p.p766 * locals.var_inv_w);
        let assign2260_e3956: f64 = (assign2260_e3952 + assign2260_e3955);
        let assign2260_e3959: f64 = (p.p956 * locals.var_inv_lw);
        let assign2260_e3960: f64 = (assign2260_e3956 + assign2260_e3959);
        locals.var_pparam_b4soikgidl = assign2260_e3960;
        locals.var_pparam_b4soikgidl_dn3 = (((p.p576 * locals.var_inv_l_dn3) + (p.p766 * locals.var_inv_w_dn3)) + (p.p956 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soikgidl_dn4 = (((p.p576 * locals.var_inv_l_dn4) + (p.p766 * locals.var_inv_w_dn4)) + (p.p956 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soikgidl_dn5 = (((p.p576 * locals.var_inv_l_dn5) + (p.p766 * locals.var_inv_w_dn5)) + (p.p956 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soikgidl_dn6 = (((p.p576 * locals.var_inv_l_dn6) + (p.p766 * locals.var_inv_w_dn6)) + (p.p956 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soikgidl_dn7 = (((p.p576 * locals.var_inv_l_dn7) + (p.p766 * locals.var_inv_w_dn7)) + (p.p956 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soikgidl_dn8 = (((p.p576 * locals.var_inv_l_dn8) + (p.p766 * locals.var_inv_w_dn8)) + (p.p956 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soikgidl_dn9 = (((p.p576 * locals.var_inv_l_dn9) + (p.p766 * locals.var_inv_w_dn9)) + (p.p956 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soikgidl_dn10 = (((p.p576 * locals.var_inv_l_dn10) + (p.p766 * locals.var_inv_w_dn10)) + (p.p956 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soikgidl_dn11 = (((p.p576 * locals.var_inv_l_dn11) + (p.p766 * locals.var_inv_w_dn11)) + (p.p956 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soikgidl_dn12 = (((p.p576 * locals.var_inv_l_dn12) + (p.p766 * locals.var_inv_w_dn12)) + (p.p956 * locals.var_inv_lw_dn12));

        let assign2270_e3964: f64 = (p.p577 * locals.var_inv_l);
        let assign2270_e3965: f64 = (p.p164 + assign2270_e3964);
        let assign2270_e3968: f64 = (p.p767 * locals.var_inv_w);
        let assign2270_e3969: f64 = (assign2270_e3965 + assign2270_e3968);
        let assign2270_e3972: f64 = (p.p957 * locals.var_inv_lw);
        let assign2270_e3973: f64 = (assign2270_e3969 + assign2270_e3972);
        locals.var_pparam_b4soifgidl = assign2270_e3973;
        locals.var_pparam_b4soifgidl_dn3 = (((p.p577 * locals.var_inv_l_dn3) + (p.p767 * locals.var_inv_w_dn3)) + (p.p957 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soifgidl_dn4 = (((p.p577 * locals.var_inv_l_dn4) + (p.p767 * locals.var_inv_w_dn4)) + (p.p957 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soifgidl_dn5 = (((p.p577 * locals.var_inv_l_dn5) + (p.p767 * locals.var_inv_w_dn5)) + (p.p957 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soifgidl_dn6 = (((p.p577 * locals.var_inv_l_dn6) + (p.p767 * locals.var_inv_w_dn6)) + (p.p957 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soifgidl_dn7 = (((p.p577 * locals.var_inv_l_dn7) + (p.p767 * locals.var_inv_w_dn7)) + (p.p957 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soifgidl_dn8 = (((p.p577 * locals.var_inv_l_dn8) + (p.p767 * locals.var_inv_w_dn8)) + (p.p957 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soifgidl_dn9 = (((p.p577 * locals.var_inv_l_dn9) + (p.p767 * locals.var_inv_w_dn9)) + (p.p957 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soifgidl_dn10 = (((p.p577 * locals.var_inv_l_dn10) + (p.p767 * locals.var_inv_w_dn10)) + (p.p957 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soifgidl_dn11 = (((p.p577 * locals.var_inv_l_dn11) + (p.p767 * locals.var_inv_w_dn11)) + (p.p957 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soifgidl_dn12 = (((p.p577 * locals.var_inv_l_dn12) + (p.p767 * locals.var_inv_w_dn12)) + (p.p957 * locals.var_inv_lw_dn12));

        let assign2280_e3977: f64 = (p.p578 * locals.var_inv_l);
        let assign2280_e3978: f64 = (p.p165 + assign2280_e3977);
        let assign2280_e3981: f64 = (p.p768 * locals.var_inv_w);
        let assign2280_e3982: f64 = (assign2280_e3978 + assign2280_e3981);
        let assign2280_e3985: f64 = (p.p958 * locals.var_inv_lw);
        let assign2280_e3986: f64 = (assign2280_e3982 + assign2280_e3985);
        locals.var_pparam_b4soiagisl = assign2280_e3986;
        locals.var_pparam_b4soiagisl_dn3 = (((p.p578 * locals.var_inv_l_dn3) + (p.p768 * locals.var_inv_w_dn3)) + (p.p958 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiagisl_dn4 = (((p.p578 * locals.var_inv_l_dn4) + (p.p768 * locals.var_inv_w_dn4)) + (p.p958 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiagisl_dn5 = (((p.p578 * locals.var_inv_l_dn5) + (p.p768 * locals.var_inv_w_dn5)) + (p.p958 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiagisl_dn6 = (((p.p578 * locals.var_inv_l_dn6) + (p.p768 * locals.var_inv_w_dn6)) + (p.p958 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiagisl_dn7 = (((p.p578 * locals.var_inv_l_dn7) + (p.p768 * locals.var_inv_w_dn7)) + (p.p958 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiagisl_dn8 = (((p.p578 * locals.var_inv_l_dn8) + (p.p768 * locals.var_inv_w_dn8)) + (p.p958 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiagisl_dn9 = (((p.p578 * locals.var_inv_l_dn9) + (p.p768 * locals.var_inv_w_dn9)) + (p.p958 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiagisl_dn10 = (((p.p578 * locals.var_inv_l_dn10) + (p.p768 * locals.var_inv_w_dn10)) + (p.p958 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiagisl_dn11 = (((p.p578 * locals.var_inv_l_dn11) + (p.p768 * locals.var_inv_w_dn11)) + (p.p958 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiagisl_dn12 = (((p.p578 * locals.var_inv_l_dn12) + (p.p768 * locals.var_inv_w_dn12)) + (p.p958 * locals.var_inv_lw_dn12));

        let assign2290_e3990: f64 = (p.p579 * locals.var_inv_l);
        let assign2290_e3991: f64 = (p.p166 + assign2290_e3990);
        let assign2290_e3994: f64 = (p.p769 * locals.var_inv_w);
        let assign2290_e3995: f64 = (assign2290_e3991 + assign2290_e3994);
        let assign2290_e3998: f64 = (p.p959 * locals.var_inv_lw);
        let assign2290_e3999: f64 = (assign2290_e3995 + assign2290_e3998);
        locals.var_pparam_b4soibgisl = assign2290_e3999;
        locals.var_pparam_b4soibgisl_dn3 = (((p.p579 * locals.var_inv_l_dn3) + (p.p769 * locals.var_inv_w_dn3)) + (p.p959 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soibgisl_dn4 = (((p.p579 * locals.var_inv_l_dn4) + (p.p769 * locals.var_inv_w_dn4)) + (p.p959 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soibgisl_dn5 = (((p.p579 * locals.var_inv_l_dn5) + (p.p769 * locals.var_inv_w_dn5)) + (p.p959 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soibgisl_dn6 = (((p.p579 * locals.var_inv_l_dn6) + (p.p769 * locals.var_inv_w_dn6)) + (p.p959 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soibgisl_dn7 = (((p.p579 * locals.var_inv_l_dn7) + (p.p769 * locals.var_inv_w_dn7)) + (p.p959 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soibgisl_dn8 = (((p.p579 * locals.var_inv_l_dn8) + (p.p769 * locals.var_inv_w_dn8)) + (p.p959 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soibgisl_dn9 = (((p.p579 * locals.var_inv_l_dn9) + (p.p769 * locals.var_inv_w_dn9)) + (p.p959 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soibgisl_dn10 = (((p.p579 * locals.var_inv_l_dn10) + (p.p769 * locals.var_inv_w_dn10)) + (p.p959 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soibgisl_dn11 = (((p.p579 * locals.var_inv_l_dn11) + (p.p769 * locals.var_inv_w_dn11)) + (p.p959 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soibgisl_dn12 = (((p.p579 * locals.var_inv_l_dn12) + (p.p769 * locals.var_inv_w_dn12)) + (p.p959 * locals.var_inv_lw_dn12));

        let assign2300_e4003: f64 = (p.p580 * locals.var_inv_l);
        let assign2300_e4004: f64 = (p.p167 + assign2300_e4003);
        let assign2300_e4007: f64 = (p.p770 * locals.var_inv_w);
        let assign2300_e4008: f64 = (assign2300_e4004 + assign2300_e4007);
        let assign2300_e4011: f64 = (p.p960 * locals.var_inv_lw);
        let assign2300_e4012: f64 = (assign2300_e4008 + assign2300_e4011);
        locals.var_pparam_b4soibgisl1 = assign2300_e4012;
        locals.var_pparam_b4soibgisl1_dn3 = (((p.p580 * locals.var_inv_l_dn3) + (p.p770 * locals.var_inv_w_dn3)) + (p.p960 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soibgisl1_dn4 = (((p.p580 * locals.var_inv_l_dn4) + (p.p770 * locals.var_inv_w_dn4)) + (p.p960 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soibgisl1_dn5 = (((p.p580 * locals.var_inv_l_dn5) + (p.p770 * locals.var_inv_w_dn5)) + (p.p960 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soibgisl1_dn6 = (((p.p580 * locals.var_inv_l_dn6) + (p.p770 * locals.var_inv_w_dn6)) + (p.p960 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soibgisl1_dn7 = (((p.p580 * locals.var_inv_l_dn7) + (p.p770 * locals.var_inv_w_dn7)) + (p.p960 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soibgisl1_dn8 = (((p.p580 * locals.var_inv_l_dn8) + (p.p770 * locals.var_inv_w_dn8)) + (p.p960 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soibgisl1_dn9 = (((p.p580 * locals.var_inv_l_dn9) + (p.p770 * locals.var_inv_w_dn9)) + (p.p960 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soibgisl1_dn10 = (((p.p580 * locals.var_inv_l_dn10) + (p.p770 * locals.var_inv_w_dn10)) + (p.p960 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soibgisl1_dn11 = (((p.p580 * locals.var_inv_l_dn11) + (p.p770 * locals.var_inv_w_dn11)) + (p.p960 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soibgisl1_dn12 = (((p.p580 * locals.var_inv_l_dn12) + (p.p770 * locals.var_inv_w_dn12)) + (p.p960 * locals.var_inv_lw_dn12));

        let assign2310_e4016: f64 = (p.p581 * locals.var_inv_l);
        let assign2310_e4017: f64 = (p.p168 + assign2310_e4016);
        let assign2310_e4020: f64 = (p.p771 * locals.var_inv_w);
        let assign2310_e4021: f64 = (assign2310_e4017 + assign2310_e4020);
        let assign2310_e4024: f64 = (p.p961 * locals.var_inv_lw);
        let assign2310_e4025: f64 = (assign2310_e4021 + assign2310_e4024);
        locals.var_pparam_b4soicgisl = assign2310_e4025;
        locals.var_pparam_b4soicgisl_dn3 = (((p.p581 * locals.var_inv_l_dn3) + (p.p771 * locals.var_inv_w_dn3)) + (p.p961 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soicgisl_dn4 = (((p.p581 * locals.var_inv_l_dn4) + (p.p771 * locals.var_inv_w_dn4)) + (p.p961 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soicgisl_dn5 = (((p.p581 * locals.var_inv_l_dn5) + (p.p771 * locals.var_inv_w_dn5)) + (p.p961 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soicgisl_dn6 = (((p.p581 * locals.var_inv_l_dn6) + (p.p771 * locals.var_inv_w_dn6)) + (p.p961 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soicgisl_dn7 = (((p.p581 * locals.var_inv_l_dn7) + (p.p771 * locals.var_inv_w_dn7)) + (p.p961 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soicgisl_dn8 = (((p.p581 * locals.var_inv_l_dn8) + (p.p771 * locals.var_inv_w_dn8)) + (p.p961 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soicgisl_dn9 = (((p.p581 * locals.var_inv_l_dn9) + (p.p771 * locals.var_inv_w_dn9)) + (p.p961 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soicgisl_dn10 = (((p.p581 * locals.var_inv_l_dn10) + (p.p771 * locals.var_inv_w_dn10)) + (p.p961 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soicgisl_dn11 = (((p.p581 * locals.var_inv_l_dn11) + (p.p771 * locals.var_inv_w_dn11)) + (p.p961 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soicgisl_dn12 = (((p.p581 * locals.var_inv_l_dn12) + (p.p771 * locals.var_inv_w_dn12)) + (p.p961 * locals.var_inv_lw_dn12));

        let assign2320_e4029: f64 = (p.p1026 * locals.var_inv_l);
        let assign2320_e4030: f64 = (p.p1023 + assign2320_e4029);
        let assign2320_e4033: f64 = (p.p1029 * locals.var_inv_w);
        let assign2320_e4034: f64 = (assign2320_e4030 + assign2320_e4033);
        let assign2320_e4037: f64 = (p.p1032 * locals.var_inv_lw);
        let assign2320_e4038: f64 = (assign2320_e4034 + assign2320_e4037);
        locals.var_pparam_b4soiegisl = assign2320_e4038;
        locals.var_pparam_b4soiegisl_dn3 = (((p.p1026 * locals.var_inv_l_dn3) + (p.p1029 * locals.var_inv_w_dn3)) + (p.p1032 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiegisl_dn4 = (((p.p1026 * locals.var_inv_l_dn4) + (p.p1029 * locals.var_inv_w_dn4)) + (p.p1032 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiegisl_dn5 = (((p.p1026 * locals.var_inv_l_dn5) + (p.p1029 * locals.var_inv_w_dn5)) + (p.p1032 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiegisl_dn6 = (((p.p1026 * locals.var_inv_l_dn6) + (p.p1029 * locals.var_inv_w_dn6)) + (p.p1032 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiegisl_dn7 = (((p.p1026 * locals.var_inv_l_dn7) + (p.p1029 * locals.var_inv_w_dn7)) + (p.p1032 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiegisl_dn8 = (((p.p1026 * locals.var_inv_l_dn8) + (p.p1029 * locals.var_inv_w_dn8)) + (p.p1032 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiegisl_dn9 = (((p.p1026 * locals.var_inv_l_dn9) + (p.p1029 * locals.var_inv_w_dn9)) + (p.p1032 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiegisl_dn10 = (((p.p1026 * locals.var_inv_l_dn10) + (p.p1029 * locals.var_inv_w_dn10)) + (p.p1032 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiegisl_dn11 = (((p.p1026 * locals.var_inv_l_dn11) + (p.p1029 * locals.var_inv_w_dn11)) + (p.p1032 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiegisl_dn12 = (((p.p1026 * locals.var_inv_l_dn12) + (p.p1029 * locals.var_inv_w_dn12)) + (p.p1032 * locals.var_inv_lw_dn12));

        let assign2330_e4042: f64 = (p.p582 * locals.var_inv_l);
        let assign2330_e4043: f64 = (p.p169 + assign2330_e4042);
        let assign2330_e4046: f64 = (p.p772 * locals.var_inv_w);
        let assign2330_e4047: f64 = (assign2330_e4043 + assign2330_e4046);
        let assign2330_e4050: f64 = (p.p962 * locals.var_inv_lw);
        let assign2330_e4051: f64 = (assign2330_e4047 + assign2330_e4050);
        locals.var_pparam_b4soirgisl = assign2330_e4051;
        locals.var_pparam_b4soirgisl_dn3 = (((p.p582 * locals.var_inv_l_dn3) + (p.p772 * locals.var_inv_w_dn3)) + (p.p962 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soirgisl_dn4 = (((p.p582 * locals.var_inv_l_dn4) + (p.p772 * locals.var_inv_w_dn4)) + (p.p962 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soirgisl_dn5 = (((p.p582 * locals.var_inv_l_dn5) + (p.p772 * locals.var_inv_w_dn5)) + (p.p962 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soirgisl_dn6 = (((p.p582 * locals.var_inv_l_dn6) + (p.p772 * locals.var_inv_w_dn6)) + (p.p962 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soirgisl_dn7 = (((p.p582 * locals.var_inv_l_dn7) + (p.p772 * locals.var_inv_w_dn7)) + (p.p962 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soirgisl_dn8 = (((p.p582 * locals.var_inv_l_dn8) + (p.p772 * locals.var_inv_w_dn8)) + (p.p962 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soirgisl_dn9 = (((p.p582 * locals.var_inv_l_dn9) + (p.p772 * locals.var_inv_w_dn9)) + (p.p962 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soirgisl_dn10 = (((p.p582 * locals.var_inv_l_dn10) + (p.p772 * locals.var_inv_w_dn10)) + (p.p962 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soirgisl_dn11 = (((p.p582 * locals.var_inv_l_dn11) + (p.p772 * locals.var_inv_w_dn11)) + (p.p962 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soirgisl_dn12 = (((p.p582 * locals.var_inv_l_dn12) + (p.p772 * locals.var_inv_w_dn12)) + (p.p962 * locals.var_inv_lw_dn12));

        let assign2340_e4055: f64 = (p.p583 * locals.var_inv_l);
        let assign2340_e4056: f64 = (p.p170 + assign2340_e4055);
        let assign2340_e4059: f64 = (p.p773 * locals.var_inv_w);
        let assign2340_e4060: f64 = (assign2340_e4056 + assign2340_e4059);
        let assign2340_e4063: f64 = (p.p963 * locals.var_inv_lw);
        let assign2340_e4064: f64 = (assign2340_e4060 + assign2340_e4063);
        locals.var_pparam_b4soikgisl = assign2340_e4064;
        locals.var_pparam_b4soikgisl_dn3 = (((p.p583 * locals.var_inv_l_dn3) + (p.p773 * locals.var_inv_w_dn3)) + (p.p963 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soikgisl_dn4 = (((p.p583 * locals.var_inv_l_dn4) + (p.p773 * locals.var_inv_w_dn4)) + (p.p963 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soikgisl_dn5 = (((p.p583 * locals.var_inv_l_dn5) + (p.p773 * locals.var_inv_w_dn5)) + (p.p963 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soikgisl_dn6 = (((p.p583 * locals.var_inv_l_dn6) + (p.p773 * locals.var_inv_w_dn6)) + (p.p963 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soikgisl_dn7 = (((p.p583 * locals.var_inv_l_dn7) + (p.p773 * locals.var_inv_w_dn7)) + (p.p963 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soikgisl_dn8 = (((p.p583 * locals.var_inv_l_dn8) + (p.p773 * locals.var_inv_w_dn8)) + (p.p963 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soikgisl_dn9 = (((p.p583 * locals.var_inv_l_dn9) + (p.p773 * locals.var_inv_w_dn9)) + (p.p963 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soikgisl_dn10 = (((p.p583 * locals.var_inv_l_dn10) + (p.p773 * locals.var_inv_w_dn10)) + (p.p963 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soikgisl_dn11 = (((p.p583 * locals.var_inv_l_dn11) + (p.p773 * locals.var_inv_w_dn11)) + (p.p963 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soikgisl_dn12 = (((p.p583 * locals.var_inv_l_dn12) + (p.p773 * locals.var_inv_w_dn12)) + (p.p963 * locals.var_inv_lw_dn12));

        let assign2350_e4068: f64 = (p.p584 * locals.var_inv_l);
        let assign2350_e4069: f64 = (p.p171 + assign2350_e4068);
        let assign2350_e4072: f64 = (p.p774 * locals.var_inv_w);
        let assign2350_e4073: f64 = (assign2350_e4069 + assign2350_e4072);
        let assign2350_e4076: f64 = (p.p964 * locals.var_inv_lw);
        let assign2350_e4077: f64 = (assign2350_e4073 + assign2350_e4076);
        locals.var_pparam_b4soifgisl = assign2350_e4077;
        locals.var_pparam_b4soifgisl_dn3 = (((p.p584 * locals.var_inv_l_dn3) + (p.p774 * locals.var_inv_w_dn3)) + (p.p964 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soifgisl_dn4 = (((p.p584 * locals.var_inv_l_dn4) + (p.p774 * locals.var_inv_w_dn4)) + (p.p964 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soifgisl_dn5 = (((p.p584 * locals.var_inv_l_dn5) + (p.p774 * locals.var_inv_w_dn5)) + (p.p964 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soifgisl_dn6 = (((p.p584 * locals.var_inv_l_dn6) + (p.p774 * locals.var_inv_w_dn6)) + (p.p964 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soifgisl_dn7 = (((p.p584 * locals.var_inv_l_dn7) + (p.p774 * locals.var_inv_w_dn7)) + (p.p964 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soifgisl_dn8 = (((p.p584 * locals.var_inv_l_dn8) + (p.p774 * locals.var_inv_w_dn8)) + (p.p964 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soifgisl_dn9 = (((p.p584 * locals.var_inv_l_dn9) + (p.p774 * locals.var_inv_w_dn9)) + (p.p964 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soifgisl_dn10 = (((p.p584 * locals.var_inv_l_dn10) + (p.p774 * locals.var_inv_w_dn10)) + (p.p964 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soifgisl_dn11 = (((p.p584 * locals.var_inv_l_dn11) + (p.p774 * locals.var_inv_w_dn11)) + (p.p964 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soifgisl_dn12 = (((p.p584 * locals.var_inv_l_dn12) + (p.p774 * locals.var_inv_w_dn12)) + (p.p964 * locals.var_inv_lw_dn12));

        let assign2360_e4081: f64 = (p.p585 * locals.var_inv_l);
        let assign2360_e4082: f64 = (p.p322 + assign2360_e4081);
        let assign2360_e4085: f64 = (p.p775 * locals.var_inv_w);
        let assign2360_e4086: f64 = (assign2360_e4082 + assign2360_e4085);
        let assign2360_e4089: f64 = (p.p965 * locals.var_inv_lw);
        let assign2360_e4090: f64 = (assign2360_e4086 + assign2360_e4089);
        locals.var_pparam_b4sointun = assign2360_e4090;
        locals.var_pparam_b4sointun_dn3 = (((p.p585 * locals.var_inv_l_dn3) + (p.p775 * locals.var_inv_w_dn3)) + (p.p965 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4sointun_dn4 = (((p.p585 * locals.var_inv_l_dn4) + (p.p775 * locals.var_inv_w_dn4)) + (p.p965 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4sointun_dn5 = (((p.p585 * locals.var_inv_l_dn5) + (p.p775 * locals.var_inv_w_dn5)) + (p.p965 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4sointun_dn6 = (((p.p585 * locals.var_inv_l_dn6) + (p.p775 * locals.var_inv_w_dn6)) + (p.p965 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4sointun_dn7 = (((p.p585 * locals.var_inv_l_dn7) + (p.p775 * locals.var_inv_w_dn7)) + (p.p965 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4sointun_dn8 = (((p.p585 * locals.var_inv_l_dn8) + (p.p775 * locals.var_inv_w_dn8)) + (p.p965 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4sointun_dn9 = (((p.p585 * locals.var_inv_l_dn9) + (p.p775 * locals.var_inv_w_dn9)) + (p.p965 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4sointun_dn10 = (((p.p585 * locals.var_inv_l_dn10) + (p.p775 * locals.var_inv_w_dn10)) + (p.p965 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4sointun_dn11 = (((p.p585 * locals.var_inv_l_dn11) + (p.p775 * locals.var_inv_w_dn11)) + (p.p965 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4sointun_dn12 = (((p.p585 * locals.var_inv_l_dn12) + (p.p775 * locals.var_inv_w_dn12)) + (p.p965 * locals.var_inv_lw_dn12));

        let assign2370_e4094: f64 = (p.p586 * locals.var_inv_l);
        let assign2370_e4095: f64 = (p.p323 + assign2370_e4094);
        let assign2370_e4098: f64 = (p.p776 * locals.var_inv_w);
        let assign2370_e4099: f64 = (assign2370_e4095 + assign2370_e4098);
        let assign2370_e4102: f64 = (p.p966 * locals.var_inv_lw);
        let assign2370_e4103: f64 = (assign2370_e4099 + assign2370_e4102);
        locals.var_pparam_b4sointund = assign2370_e4103;
        locals.var_pparam_b4sointund_dn3 = (((p.p586 * locals.var_inv_l_dn3) + (p.p776 * locals.var_inv_w_dn3)) + (p.p966 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4sointund_dn4 = (((p.p586 * locals.var_inv_l_dn4) + (p.p776 * locals.var_inv_w_dn4)) + (p.p966 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4sointund_dn5 = (((p.p586 * locals.var_inv_l_dn5) + (p.p776 * locals.var_inv_w_dn5)) + (p.p966 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4sointund_dn6 = (((p.p586 * locals.var_inv_l_dn6) + (p.p776 * locals.var_inv_w_dn6)) + (p.p966 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4sointund_dn7 = (((p.p586 * locals.var_inv_l_dn7) + (p.p776 * locals.var_inv_w_dn7)) + (p.p966 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4sointund_dn8 = (((p.p586 * locals.var_inv_l_dn8) + (p.p776 * locals.var_inv_w_dn8)) + (p.p966 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4sointund_dn9 = (((p.p586 * locals.var_inv_l_dn9) + (p.p776 * locals.var_inv_w_dn9)) + (p.p966 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4sointund_dn10 = (((p.p586 * locals.var_inv_l_dn10) + (p.p776 * locals.var_inv_w_dn10)) + (p.p966 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4sointund_dn11 = (((p.p586 * locals.var_inv_l_dn11) + (p.p776 * locals.var_inv_w_dn11)) + (p.p966 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4sointund_dn12 = (((p.p586 * locals.var_inv_l_dn12) + (p.p776 * locals.var_inv_w_dn12)) + (p.p966 * locals.var_inv_lw_dn12));

        let assign2380_e4107: f64 = (p.p587 * locals.var_inv_l);
        let assign2380_e4108: f64 = (p.p172 + assign2380_e4107);
        let assign2380_e4111: f64 = (p.p777 * locals.var_inv_w);
        let assign2380_e4112: f64 = (assign2380_e4108 + assign2380_e4111);
        let assign2380_e4115: f64 = (p.p967 * locals.var_inv_lw);
        let assign2380_e4116: f64 = (assign2380_e4112 + assign2380_e4115);
        locals.var_pparam_b4soindiode = assign2380_e4116;
        locals.var_pparam_b4soindiode_dn3 = (((p.p587 * locals.var_inv_l_dn3) + (p.p777 * locals.var_inv_w_dn3)) + (p.p967 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soindiode_dn4 = (((p.p587 * locals.var_inv_l_dn4) + (p.p777 * locals.var_inv_w_dn4)) + (p.p967 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soindiode_dn5 = (((p.p587 * locals.var_inv_l_dn5) + (p.p777 * locals.var_inv_w_dn5)) + (p.p967 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soindiode_dn6 = (((p.p587 * locals.var_inv_l_dn6) + (p.p777 * locals.var_inv_w_dn6)) + (p.p967 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soindiode_dn7 = (((p.p587 * locals.var_inv_l_dn7) + (p.p777 * locals.var_inv_w_dn7)) + (p.p967 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soindiode_dn8 = (((p.p587 * locals.var_inv_l_dn8) + (p.p777 * locals.var_inv_w_dn8)) + (p.p967 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soindiode_dn9 = (((p.p587 * locals.var_inv_l_dn9) + (p.p777 * locals.var_inv_w_dn9)) + (p.p967 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soindiode_dn10 = (((p.p587 * locals.var_inv_l_dn10) + (p.p777 * locals.var_inv_w_dn10)) + (p.p967 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soindiode_dn11 = (((p.p587 * locals.var_inv_l_dn11) + (p.p777 * locals.var_inv_w_dn11)) + (p.p967 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soindiode_dn12 = (((p.p587 * locals.var_inv_l_dn12) + (p.p777 * locals.var_inv_w_dn12)) + (p.p967 * locals.var_inv_lw_dn12));

        let assign2390_e4120: f64 = (p.p588 * locals.var_inv_l);
        let assign2390_e4121: f64 = (p.p173 + assign2390_e4120);
        let assign2390_e4124: f64 = (p.p778 * locals.var_inv_w);
        let assign2390_e4125: f64 = (assign2390_e4121 + assign2390_e4124);
        let assign2390_e4128: f64 = (p.p968 * locals.var_inv_lw);
        let assign2390_e4129: f64 = (assign2390_e4125 + assign2390_e4128);
        locals.var_pparam_b4soindioded = assign2390_e4129;
        locals.var_pparam_b4soindioded_dn3 = (((p.p588 * locals.var_inv_l_dn3) + (p.p778 * locals.var_inv_w_dn3)) + (p.p968 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soindioded_dn4 = (((p.p588 * locals.var_inv_l_dn4) + (p.p778 * locals.var_inv_w_dn4)) + (p.p968 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soindioded_dn5 = (((p.p588 * locals.var_inv_l_dn5) + (p.p778 * locals.var_inv_w_dn5)) + (p.p968 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soindioded_dn6 = (((p.p588 * locals.var_inv_l_dn6) + (p.p778 * locals.var_inv_w_dn6)) + (p.p968 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soindioded_dn7 = (((p.p588 * locals.var_inv_l_dn7) + (p.p778 * locals.var_inv_w_dn7)) + (p.p968 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soindioded_dn8 = (((p.p588 * locals.var_inv_l_dn8) + (p.p778 * locals.var_inv_w_dn8)) + (p.p968 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soindioded_dn9 = (((p.p588 * locals.var_inv_l_dn9) + (p.p778 * locals.var_inv_w_dn9)) + (p.p968 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soindioded_dn10 = (((p.p588 * locals.var_inv_l_dn10) + (p.p778 * locals.var_inv_w_dn10)) + (p.p968 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soindioded_dn11 = (((p.p588 * locals.var_inv_l_dn11) + (p.p778 * locals.var_inv_w_dn11)) + (p.p968 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soindioded_dn12 = (((p.p588 * locals.var_inv_l_dn12) + (p.p778 * locals.var_inv_w_dn12)) + (p.p968 * locals.var_inv_lw_dn12));

        let assign2400_e4133: f64 = (p.p589 * locals.var_inv_l);
        let assign2400_e4134: f64 = (p.p324 + assign2400_e4133);
        let assign2400_e4137: f64 = (p.p779 * locals.var_inv_w);
        let assign2400_e4138: f64 = (assign2400_e4134 + assign2400_e4137);
        let assign2400_e4141: f64 = (p.p969 * locals.var_inv_lw);
        let assign2400_e4142: f64 = (assign2400_e4138 + assign2400_e4141);
        locals.var_pparam_b4soinrecf0 = assign2400_e4142;
        locals.var_pparam_b4soinrecf0_dn3 = (((p.p589 * locals.var_inv_l_dn3) + (p.p779 * locals.var_inv_w_dn3)) + (p.p969 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soinrecf0_dn4 = (((p.p589 * locals.var_inv_l_dn4) + (p.p779 * locals.var_inv_w_dn4)) + (p.p969 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soinrecf0_dn5 = (((p.p589 * locals.var_inv_l_dn5) + (p.p779 * locals.var_inv_w_dn5)) + (p.p969 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soinrecf0_dn6 = (((p.p589 * locals.var_inv_l_dn6) + (p.p779 * locals.var_inv_w_dn6)) + (p.p969 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soinrecf0_dn7 = (((p.p589 * locals.var_inv_l_dn7) + (p.p779 * locals.var_inv_w_dn7)) + (p.p969 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soinrecf0_dn8 = (((p.p589 * locals.var_inv_l_dn8) + (p.p779 * locals.var_inv_w_dn8)) + (p.p969 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soinrecf0_dn9 = (((p.p589 * locals.var_inv_l_dn9) + (p.p779 * locals.var_inv_w_dn9)) + (p.p969 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soinrecf0_dn10 = (((p.p589 * locals.var_inv_l_dn10) + (p.p779 * locals.var_inv_w_dn10)) + (p.p969 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soinrecf0_dn11 = (((p.p589 * locals.var_inv_l_dn11) + (p.p779 * locals.var_inv_w_dn11)) + (p.p969 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soinrecf0_dn12 = (((p.p589 * locals.var_inv_l_dn12) + (p.p779 * locals.var_inv_w_dn12)) + (p.p969 * locals.var_inv_lw_dn12));

        let assign2410_e4146: f64 = (p.p590 * locals.var_inv_l);
        let assign2410_e4147: f64 = (p.p325 + assign2410_e4146);
        let assign2410_e4150: f64 = (p.p780 * locals.var_inv_w);
        let assign2410_e4151: f64 = (assign2410_e4147 + assign2410_e4150);
        let assign2410_e4154: f64 = (p.p970 * locals.var_inv_lw);
        let assign2410_e4155: f64 = (assign2410_e4151 + assign2410_e4154);
        locals.var_pparam_b4soinrecf0d = assign2410_e4155;
        locals.var_pparam_b4soinrecf0d_dn3 = (((p.p590 * locals.var_inv_l_dn3) + (p.p780 * locals.var_inv_w_dn3)) + (p.p970 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soinrecf0d_dn4 = (((p.p590 * locals.var_inv_l_dn4) + (p.p780 * locals.var_inv_w_dn4)) + (p.p970 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soinrecf0d_dn5 = (((p.p590 * locals.var_inv_l_dn5) + (p.p780 * locals.var_inv_w_dn5)) + (p.p970 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soinrecf0d_dn6 = (((p.p590 * locals.var_inv_l_dn6) + (p.p780 * locals.var_inv_w_dn6)) + (p.p970 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soinrecf0d_dn7 = (((p.p590 * locals.var_inv_l_dn7) + (p.p780 * locals.var_inv_w_dn7)) + (p.p970 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soinrecf0d_dn8 = (((p.p590 * locals.var_inv_l_dn8) + (p.p780 * locals.var_inv_w_dn8)) + (p.p970 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soinrecf0d_dn9 = (((p.p590 * locals.var_inv_l_dn9) + (p.p780 * locals.var_inv_w_dn9)) + (p.p970 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soinrecf0d_dn10 = (((p.p590 * locals.var_inv_l_dn10) + (p.p780 * locals.var_inv_w_dn10)) + (p.p970 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soinrecf0d_dn11 = (((p.p590 * locals.var_inv_l_dn11) + (p.p780 * locals.var_inv_w_dn11)) + (p.p970 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soinrecf0d_dn12 = (((p.p590 * locals.var_inv_l_dn12) + (p.p780 * locals.var_inv_w_dn12)) + (p.p970 * locals.var_inv_lw_dn12));

        let assign2420_e4159: f64 = (p.p591 * locals.var_inv_l);
        let assign2420_e4160: f64 = (p.p326 + assign2420_e4159);
        let assign2420_e4163: f64 = (p.p781 * locals.var_inv_w);
        let assign2420_e4164: f64 = (assign2420_e4160 + assign2420_e4163);
        let assign2420_e4167: f64 = (p.p971 * locals.var_inv_lw);
        let assign2420_e4168: f64 = (assign2420_e4164 + assign2420_e4167);
        locals.var_pparam_b4soinrecr0 = assign2420_e4168;
        locals.var_pparam_b4soinrecr0_dn3 = (((p.p591 * locals.var_inv_l_dn3) + (p.p781 * locals.var_inv_w_dn3)) + (p.p971 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soinrecr0_dn4 = (((p.p591 * locals.var_inv_l_dn4) + (p.p781 * locals.var_inv_w_dn4)) + (p.p971 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soinrecr0_dn5 = (((p.p591 * locals.var_inv_l_dn5) + (p.p781 * locals.var_inv_w_dn5)) + (p.p971 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soinrecr0_dn6 = (((p.p591 * locals.var_inv_l_dn6) + (p.p781 * locals.var_inv_w_dn6)) + (p.p971 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soinrecr0_dn7 = (((p.p591 * locals.var_inv_l_dn7) + (p.p781 * locals.var_inv_w_dn7)) + (p.p971 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soinrecr0_dn8 = (((p.p591 * locals.var_inv_l_dn8) + (p.p781 * locals.var_inv_w_dn8)) + (p.p971 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soinrecr0_dn9 = (((p.p591 * locals.var_inv_l_dn9) + (p.p781 * locals.var_inv_w_dn9)) + (p.p971 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soinrecr0_dn10 = (((p.p591 * locals.var_inv_l_dn10) + (p.p781 * locals.var_inv_w_dn10)) + (p.p971 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soinrecr0_dn11 = (((p.p591 * locals.var_inv_l_dn11) + (p.p781 * locals.var_inv_w_dn11)) + (p.p971 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soinrecr0_dn12 = (((p.p591 * locals.var_inv_l_dn12) + (p.p781 * locals.var_inv_w_dn12)) + (p.p971 * locals.var_inv_lw_dn12));

        let assign2430_e4172: f64 = (p.p592 * locals.var_inv_l);
        let assign2430_e4173: f64 = (p.p327 + assign2430_e4172);
        let assign2430_e4176: f64 = (p.p782 * locals.var_inv_w);
        let assign2430_e4177: f64 = (assign2430_e4173 + assign2430_e4176);
        let assign2430_e4180: f64 = (p.p972 * locals.var_inv_lw);
        let assign2430_e4181: f64 = (assign2430_e4177 + assign2430_e4180);
        locals.var_pparam_b4soinrecr0d = assign2430_e4181;
        locals.var_pparam_b4soinrecr0d_dn3 = (((p.p592 * locals.var_inv_l_dn3) + (p.p782 * locals.var_inv_w_dn3)) + (p.p972 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soinrecr0d_dn4 = (((p.p592 * locals.var_inv_l_dn4) + (p.p782 * locals.var_inv_w_dn4)) + (p.p972 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soinrecr0d_dn5 = (((p.p592 * locals.var_inv_l_dn5) + (p.p782 * locals.var_inv_w_dn5)) + (p.p972 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soinrecr0d_dn6 = (((p.p592 * locals.var_inv_l_dn6) + (p.p782 * locals.var_inv_w_dn6)) + (p.p972 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soinrecr0d_dn7 = (((p.p592 * locals.var_inv_l_dn7) + (p.p782 * locals.var_inv_w_dn7)) + (p.p972 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soinrecr0d_dn8 = (((p.p592 * locals.var_inv_l_dn8) + (p.p782 * locals.var_inv_w_dn8)) + (p.p972 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soinrecr0d_dn9 = (((p.p592 * locals.var_inv_l_dn9) + (p.p782 * locals.var_inv_w_dn9)) + (p.p972 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soinrecr0d_dn10 = (((p.p592 * locals.var_inv_l_dn10) + (p.p782 * locals.var_inv_w_dn10)) + (p.p972 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soinrecr0d_dn11 = (((p.p592 * locals.var_inv_l_dn11) + (p.p782 * locals.var_inv_w_dn11)) + (p.p972 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soinrecr0d_dn12 = (((p.p592 * locals.var_inv_l_dn12) + (p.p782 * locals.var_inv_w_dn12)) + (p.p972 * locals.var_inv_lw_dn12));

        let assign2440_e4185: f64 = (p.p593 * locals.var_inv_l);
        let assign2440_e4186: f64 = (p.p328 + assign2440_e4185);
        let assign2440_e4189: f64 = (p.p783 * locals.var_inv_w);
        let assign2440_e4190: f64 = (assign2440_e4186 + assign2440_e4189);
        let assign2440_e4193: f64 = (p.p973 * locals.var_inv_lw);
        let assign2440_e4194: f64 = (assign2440_e4190 + assign2440_e4193);
        locals.var_pparam_b4soiisbjt = assign2440_e4194;
        locals.var_pparam_b4soiisbjt_dn3 = (((p.p593 * locals.var_inv_l_dn3) + (p.p783 * locals.var_inv_w_dn3)) + (p.p973 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiisbjt_dn4 = (((p.p593 * locals.var_inv_l_dn4) + (p.p783 * locals.var_inv_w_dn4)) + (p.p973 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiisbjt_dn5 = (((p.p593 * locals.var_inv_l_dn5) + (p.p783 * locals.var_inv_w_dn5)) + (p.p973 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiisbjt_dn6 = (((p.p593 * locals.var_inv_l_dn6) + (p.p783 * locals.var_inv_w_dn6)) + (p.p973 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiisbjt_dn7 = (((p.p593 * locals.var_inv_l_dn7) + (p.p783 * locals.var_inv_w_dn7)) + (p.p973 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiisbjt_dn8 = (((p.p593 * locals.var_inv_l_dn8) + (p.p783 * locals.var_inv_w_dn8)) + (p.p973 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiisbjt_dn9 = (((p.p593 * locals.var_inv_l_dn9) + (p.p783 * locals.var_inv_w_dn9)) + (p.p973 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiisbjt_dn10 = (((p.p593 * locals.var_inv_l_dn10) + (p.p783 * locals.var_inv_w_dn10)) + (p.p973 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiisbjt_dn11 = (((p.p593 * locals.var_inv_l_dn11) + (p.p783 * locals.var_inv_w_dn11)) + (p.p973 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiisbjt_dn12 = (((p.p593 * locals.var_inv_l_dn12) + (p.p783 * locals.var_inv_w_dn12)) + (p.p973 * locals.var_inv_lw_dn12));

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2450_e4198: f64 = (p.p594 * locals.var_inv_l);
        let assign2450_e4199: f64 = (p.p329 + assign2450_e4198);
        let assign2450_e4202: f64 = (p.p784 * locals.var_inv_w);
        let assign2450_e4203: f64 = (assign2450_e4199 + assign2450_e4202);
        let assign2450_e4206: f64 = (p.p974 * locals.var_inv_lw);
        let assign2450_e4207: f64 = (assign2450_e4203 + assign2450_e4206);
        locals.var_pparam_b4soiidbjt = assign2450_e4207;
        locals.var_pparam_b4soiidbjt_dn3 = (((p.p594 * locals.var_inv_l_dn3) + (p.p784 * locals.var_inv_w_dn3)) + (p.p974 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiidbjt_dn4 = (((p.p594 * locals.var_inv_l_dn4) + (p.p784 * locals.var_inv_w_dn4)) + (p.p974 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiidbjt_dn5 = (((p.p594 * locals.var_inv_l_dn5) + (p.p784 * locals.var_inv_w_dn5)) + (p.p974 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiidbjt_dn6 = (((p.p594 * locals.var_inv_l_dn6) + (p.p784 * locals.var_inv_w_dn6)) + (p.p974 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiidbjt_dn7 = (((p.p594 * locals.var_inv_l_dn7) + (p.p784 * locals.var_inv_w_dn7)) + (p.p974 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiidbjt_dn8 = (((p.p594 * locals.var_inv_l_dn8) + (p.p784 * locals.var_inv_w_dn8)) + (p.p974 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiidbjt_dn9 = (((p.p594 * locals.var_inv_l_dn9) + (p.p784 * locals.var_inv_w_dn9)) + (p.p974 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiidbjt_dn10 = (((p.p594 * locals.var_inv_l_dn10) + (p.p784 * locals.var_inv_w_dn10)) + (p.p974 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiidbjt_dn11 = (((p.p594 * locals.var_inv_l_dn11) + (p.p784 * locals.var_inv_w_dn11)) + (p.p974 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiidbjt_dn12 = (((p.p594 * locals.var_inv_l_dn12) + (p.p784 * locals.var_inv_w_dn12)) + (p.p974 * locals.var_inv_lw_dn12));

        let assign2460_e4211: f64 = (p.p595 * locals.var_inv_l);
        let assign2460_e4212: f64 = (p.p330 + assign2460_e4211);
        let assign2460_e4215: f64 = (p.p785 * locals.var_inv_w);
        let assign2460_e4216: f64 = (assign2460_e4212 + assign2460_e4215);
        let assign2460_e4219: f64 = (p.p975 * locals.var_inv_lw);
        let assign2460_e4220: f64 = (assign2460_e4216 + assign2460_e4219);
        locals.var_pparam_b4soiisdif = assign2460_e4220;
        locals.var_pparam_b4soiisdif_dn3 = (((p.p595 * locals.var_inv_l_dn3) + (p.p785 * locals.var_inv_w_dn3)) + (p.p975 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiisdif_dn4 = (((p.p595 * locals.var_inv_l_dn4) + (p.p785 * locals.var_inv_w_dn4)) + (p.p975 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiisdif_dn5 = (((p.p595 * locals.var_inv_l_dn5) + (p.p785 * locals.var_inv_w_dn5)) + (p.p975 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiisdif_dn6 = (((p.p595 * locals.var_inv_l_dn6) + (p.p785 * locals.var_inv_w_dn6)) + (p.p975 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiisdif_dn7 = (((p.p595 * locals.var_inv_l_dn7) + (p.p785 * locals.var_inv_w_dn7)) + (p.p975 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiisdif_dn8 = (((p.p595 * locals.var_inv_l_dn8) + (p.p785 * locals.var_inv_w_dn8)) + (p.p975 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiisdif_dn9 = (((p.p595 * locals.var_inv_l_dn9) + (p.p785 * locals.var_inv_w_dn9)) + (p.p975 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiisdif_dn10 = (((p.p595 * locals.var_inv_l_dn10) + (p.p785 * locals.var_inv_w_dn10)) + (p.p975 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiisdif_dn11 = (((p.p595 * locals.var_inv_l_dn11) + (p.p785 * locals.var_inv_w_dn11)) + (p.p975 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiisdif_dn12 = (((p.p595 * locals.var_inv_l_dn12) + (p.p785 * locals.var_inv_w_dn12)) + (p.p975 * locals.var_inv_lw_dn12));

        let assign2470_e4224: f64 = (p.p596 * locals.var_inv_l);
        let assign2470_e4225: f64 = (p.p331 + assign2470_e4224);
        let assign2470_e4228: f64 = (p.p786 * locals.var_inv_w);
        let assign2470_e4229: f64 = (assign2470_e4225 + assign2470_e4228);
        let assign2470_e4232: f64 = (p.p976 * locals.var_inv_lw);
        let assign2470_e4233: f64 = (assign2470_e4229 + assign2470_e4232);
        locals.var_pparam_b4soiiddif = assign2470_e4233;
        locals.var_pparam_b4soiiddif_dn3 = (((p.p596 * locals.var_inv_l_dn3) + (p.p786 * locals.var_inv_w_dn3)) + (p.p976 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiiddif_dn4 = (((p.p596 * locals.var_inv_l_dn4) + (p.p786 * locals.var_inv_w_dn4)) + (p.p976 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiiddif_dn5 = (((p.p596 * locals.var_inv_l_dn5) + (p.p786 * locals.var_inv_w_dn5)) + (p.p976 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiiddif_dn6 = (((p.p596 * locals.var_inv_l_dn6) + (p.p786 * locals.var_inv_w_dn6)) + (p.p976 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiiddif_dn7 = (((p.p596 * locals.var_inv_l_dn7) + (p.p786 * locals.var_inv_w_dn7)) + (p.p976 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiiddif_dn8 = (((p.p596 * locals.var_inv_l_dn8) + (p.p786 * locals.var_inv_w_dn8)) + (p.p976 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiiddif_dn9 = (((p.p596 * locals.var_inv_l_dn9) + (p.p786 * locals.var_inv_w_dn9)) + (p.p976 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiiddif_dn10 = (((p.p596 * locals.var_inv_l_dn10) + (p.p786 * locals.var_inv_w_dn10)) + (p.p976 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiiddif_dn11 = (((p.p596 * locals.var_inv_l_dn11) + (p.p786 * locals.var_inv_w_dn11)) + (p.p976 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiiddif_dn12 = (((p.p596 * locals.var_inv_l_dn12) + (p.p786 * locals.var_inv_w_dn12)) + (p.p976 * locals.var_inv_lw_dn12));

        let assign2480_e4237: f64 = (p.p597 * locals.var_inv_l);
        let assign2480_e4238: f64 = (p.p332 + assign2480_e4237);
        let assign2480_e4241: f64 = (p.p787 * locals.var_inv_w);
        let assign2480_e4242: f64 = (assign2480_e4238 + assign2480_e4241);
        let assign2480_e4245: f64 = (p.p977 * locals.var_inv_lw);
        let assign2480_e4246: f64 = (assign2480_e4242 + assign2480_e4245);
        locals.var_pparam_b4soiisrec = assign2480_e4246;
        locals.var_pparam_b4soiisrec_dn3 = (((p.p597 * locals.var_inv_l_dn3) + (p.p787 * locals.var_inv_w_dn3)) + (p.p977 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiisrec_dn4 = (((p.p597 * locals.var_inv_l_dn4) + (p.p787 * locals.var_inv_w_dn4)) + (p.p977 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiisrec_dn5 = (((p.p597 * locals.var_inv_l_dn5) + (p.p787 * locals.var_inv_w_dn5)) + (p.p977 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiisrec_dn6 = (((p.p597 * locals.var_inv_l_dn6) + (p.p787 * locals.var_inv_w_dn6)) + (p.p977 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiisrec_dn7 = (((p.p597 * locals.var_inv_l_dn7) + (p.p787 * locals.var_inv_w_dn7)) + (p.p977 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiisrec_dn8 = (((p.p597 * locals.var_inv_l_dn8) + (p.p787 * locals.var_inv_w_dn8)) + (p.p977 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiisrec_dn9 = (((p.p597 * locals.var_inv_l_dn9) + (p.p787 * locals.var_inv_w_dn9)) + (p.p977 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiisrec_dn10 = (((p.p597 * locals.var_inv_l_dn10) + (p.p787 * locals.var_inv_w_dn10)) + (p.p977 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiisrec_dn11 = (((p.p597 * locals.var_inv_l_dn11) + (p.p787 * locals.var_inv_w_dn11)) + (p.p977 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiisrec_dn12 = (((p.p597 * locals.var_inv_l_dn12) + (p.p787 * locals.var_inv_w_dn12)) + (p.p977 * locals.var_inv_lw_dn12));

        let assign2490_e4250: f64 = (p.p599 * locals.var_inv_l);
        let assign2490_e4251: f64 = (p.p334 + assign2490_e4250);
        let assign2490_e4254: f64 = (p.p789 * locals.var_inv_w);
        let assign2490_e4255: f64 = (assign2490_e4251 + assign2490_e4254);
        let assign2490_e4258: f64 = (p.p979 * locals.var_inv_lw);
        let assign2490_e4259: f64 = (assign2490_e4255 + assign2490_e4258);
        locals.var_pparam_b4soiistun = assign2490_e4259;
        locals.var_pparam_b4soiistun_dn3 = (((p.p599 * locals.var_inv_l_dn3) + (p.p789 * locals.var_inv_w_dn3)) + (p.p979 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiistun_dn4 = (((p.p599 * locals.var_inv_l_dn4) + (p.p789 * locals.var_inv_w_dn4)) + (p.p979 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiistun_dn5 = (((p.p599 * locals.var_inv_l_dn5) + (p.p789 * locals.var_inv_w_dn5)) + (p.p979 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiistun_dn6 = (((p.p599 * locals.var_inv_l_dn6) + (p.p789 * locals.var_inv_w_dn6)) + (p.p979 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiistun_dn7 = (((p.p599 * locals.var_inv_l_dn7) + (p.p789 * locals.var_inv_w_dn7)) + (p.p979 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiistun_dn8 = (((p.p599 * locals.var_inv_l_dn8) + (p.p789 * locals.var_inv_w_dn8)) + (p.p979 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiistun_dn9 = (((p.p599 * locals.var_inv_l_dn9) + (p.p789 * locals.var_inv_w_dn9)) + (p.p979 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiistun_dn10 = (((p.p599 * locals.var_inv_l_dn10) + (p.p789 * locals.var_inv_w_dn10)) + (p.p979 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiistun_dn11 = (((p.p599 * locals.var_inv_l_dn11) + (p.p789 * locals.var_inv_w_dn11)) + (p.p979 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiistun_dn12 = (((p.p599 * locals.var_inv_l_dn12) + (p.p789 * locals.var_inv_w_dn12)) + (p.p979 * locals.var_inv_lw_dn12));

        let assign2500_e4263: f64 = (p.p598 * locals.var_inv_l);
        let assign2500_e4264: f64 = (p.p333 + assign2500_e4263);
        let assign2500_e4267: f64 = (p.p788 * locals.var_inv_w);
        let assign2500_e4268: f64 = (assign2500_e4264 + assign2500_e4267);
        let assign2500_e4271: f64 = (p.p978 * locals.var_inv_lw);
        let assign2500_e4272: f64 = (assign2500_e4268 + assign2500_e4271);
        locals.var_pparam_b4soiidrec = assign2500_e4272;
        locals.var_pparam_b4soiidrec_dn3 = (((p.p598 * locals.var_inv_l_dn3) + (p.p788 * locals.var_inv_w_dn3)) + (p.p978 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiidrec_dn4 = (((p.p598 * locals.var_inv_l_dn4) + (p.p788 * locals.var_inv_w_dn4)) + (p.p978 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiidrec_dn5 = (((p.p598 * locals.var_inv_l_dn5) + (p.p788 * locals.var_inv_w_dn5)) + (p.p978 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiidrec_dn6 = (((p.p598 * locals.var_inv_l_dn6) + (p.p788 * locals.var_inv_w_dn6)) + (p.p978 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiidrec_dn7 = (((p.p598 * locals.var_inv_l_dn7) + (p.p788 * locals.var_inv_w_dn7)) + (p.p978 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiidrec_dn8 = (((p.p598 * locals.var_inv_l_dn8) + (p.p788 * locals.var_inv_w_dn8)) + (p.p978 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiidrec_dn9 = (((p.p598 * locals.var_inv_l_dn9) + (p.p788 * locals.var_inv_w_dn9)) + (p.p978 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiidrec_dn10 = (((p.p598 * locals.var_inv_l_dn10) + (p.p788 * locals.var_inv_w_dn10)) + (p.p978 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiidrec_dn11 = (((p.p598 * locals.var_inv_l_dn11) + (p.p788 * locals.var_inv_w_dn11)) + (p.p978 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiidrec_dn12 = (((p.p598 * locals.var_inv_l_dn12) + (p.p788 * locals.var_inv_w_dn12)) + (p.p978 * locals.var_inv_lw_dn12));

        let assign2510_e4276: f64 = (p.p600 * locals.var_inv_l);
        let assign2510_e4277: f64 = (p.p335 + assign2510_e4276);
        let assign2510_e4280: f64 = (p.p790 * locals.var_inv_w);
        let assign2510_e4281: f64 = (assign2510_e4277 + assign2510_e4280);
        let assign2510_e4284: f64 = (p.p980 * locals.var_inv_lw);
        let assign2510_e4285: f64 = (assign2510_e4281 + assign2510_e4284);
        locals.var_pparam_b4soiidtun = assign2510_e4285;
        locals.var_pparam_b4soiidtun_dn3 = (((p.p600 * locals.var_inv_l_dn3) + (p.p790 * locals.var_inv_w_dn3)) + (p.p980 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiidtun_dn4 = (((p.p600 * locals.var_inv_l_dn4) + (p.p790 * locals.var_inv_w_dn4)) + (p.p980 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiidtun_dn5 = (((p.p600 * locals.var_inv_l_dn5) + (p.p790 * locals.var_inv_w_dn5)) + (p.p980 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiidtun_dn6 = (((p.p600 * locals.var_inv_l_dn6) + (p.p790 * locals.var_inv_w_dn6)) + (p.p980 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiidtun_dn7 = (((p.p600 * locals.var_inv_l_dn7) + (p.p790 * locals.var_inv_w_dn7)) + (p.p980 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiidtun_dn8 = (((p.p600 * locals.var_inv_l_dn8) + (p.p790 * locals.var_inv_w_dn8)) + (p.p980 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiidtun_dn9 = (((p.p600 * locals.var_inv_l_dn9) + (p.p790 * locals.var_inv_w_dn9)) + (p.p980 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiidtun_dn10 = (((p.p600 * locals.var_inv_l_dn10) + (p.p790 * locals.var_inv_w_dn10)) + (p.p980 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiidtun_dn11 = (((p.p600 * locals.var_inv_l_dn11) + (p.p790 * locals.var_inv_w_dn11)) + (p.p980 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiidtun_dn12 = (((p.p600 * locals.var_inv_l_dn12) + (p.p790 * locals.var_inv_w_dn12)) + (p.p980 * locals.var_inv_lw_dn12));

        let assign2520_e4289: f64 = (p.p600 * locals.var_inv_l);
        let assign2520_e4290: f64 = (p.p335 + assign2520_e4289);
        let assign2520_e4293: f64 = (p.p790 * locals.var_inv_w);
        let assign2520_e4294: f64 = (assign2520_e4290 + assign2520_e4293);
        let assign2520_e4297: f64 = (p.p980 * locals.var_inv_lw);
        let assign2520_e4298: f64 = (assign2520_e4294 + assign2520_e4297);
        locals.var_pparam_b4soiidtun = assign2520_e4298;
        locals.var_pparam_b4soiidtun_dn3 = (((p.p600 * locals.var_inv_l_dn3) + (p.p790 * locals.var_inv_w_dn3)) + (p.p980 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiidtun_dn4 = (((p.p600 * locals.var_inv_l_dn4) + (p.p790 * locals.var_inv_w_dn4)) + (p.p980 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiidtun_dn5 = (((p.p600 * locals.var_inv_l_dn5) + (p.p790 * locals.var_inv_w_dn5)) + (p.p980 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiidtun_dn6 = (((p.p600 * locals.var_inv_l_dn6) + (p.p790 * locals.var_inv_w_dn6)) + (p.p980 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiidtun_dn7 = (((p.p600 * locals.var_inv_l_dn7) + (p.p790 * locals.var_inv_w_dn7)) + (p.p980 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiidtun_dn8 = (((p.p600 * locals.var_inv_l_dn8) + (p.p790 * locals.var_inv_w_dn8)) + (p.p980 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiidtun_dn9 = (((p.p600 * locals.var_inv_l_dn9) + (p.p790 * locals.var_inv_w_dn9)) + (p.p980 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiidtun_dn10 = (((p.p600 * locals.var_inv_l_dn10) + (p.p790 * locals.var_inv_w_dn10)) + (p.p980 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiidtun_dn11 = (((p.p600 * locals.var_inv_l_dn11) + (p.p790 * locals.var_inv_w_dn11)) + (p.p980 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiidtun_dn12 = (((p.p600 * locals.var_inv_l_dn12) + (p.p790 * locals.var_inv_w_dn12)) + (p.p980 * locals.var_inv_lw_dn12));

        let assign2530_e4302: f64 = (p.p601 * locals.var_inv_l);
        let assign2530_e4303: f64 = (p.p337 + assign2530_e4302);
        let assign2530_e4306: f64 = (p.p791 * locals.var_inv_w);
        let assign2530_e4307: f64 = (assign2530_e4303 + assign2530_e4306);
        let assign2530_e4310: f64 = (p.p981 * locals.var_inv_lw);
        let assign2530_e4311: f64 = (assign2530_e4307 + assign2530_e4310);
        locals.var_pparam_b4soivrec0 = assign2530_e4311;
        locals.var_pparam_b4soivrec0_dn3 = (((p.p601 * locals.var_inv_l_dn3) + (p.p791 * locals.var_inv_w_dn3)) + (p.p981 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivrec0_dn4 = (((p.p601 * locals.var_inv_l_dn4) + (p.p791 * locals.var_inv_w_dn4)) + (p.p981 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivrec0_dn5 = (((p.p601 * locals.var_inv_l_dn5) + (p.p791 * locals.var_inv_w_dn5)) + (p.p981 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivrec0_dn6 = (((p.p601 * locals.var_inv_l_dn6) + (p.p791 * locals.var_inv_w_dn6)) + (p.p981 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivrec0_dn7 = (((p.p601 * locals.var_inv_l_dn7) + (p.p791 * locals.var_inv_w_dn7)) + (p.p981 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivrec0_dn8 = (((p.p601 * locals.var_inv_l_dn8) + (p.p791 * locals.var_inv_w_dn8)) + (p.p981 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivrec0_dn9 = (((p.p601 * locals.var_inv_l_dn9) + (p.p791 * locals.var_inv_w_dn9)) + (p.p981 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivrec0_dn10 = (((p.p601 * locals.var_inv_l_dn10) + (p.p791 * locals.var_inv_w_dn10)) + (p.p981 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivrec0_dn11 = (((p.p601 * locals.var_inv_l_dn11) + (p.p791 * locals.var_inv_w_dn11)) + (p.p981 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivrec0_dn12 = (((p.p601 * locals.var_inv_l_dn12) + (p.p791 * locals.var_inv_w_dn12)) + (p.p981 * locals.var_inv_lw_dn12));

        let assign2540_e4315: f64 = (p.p602 * locals.var_inv_l);
        let assign2540_e4316: f64 = (p.p338 + assign2540_e4315);
        let assign2540_e4319: f64 = (p.p792 * locals.var_inv_w);
        let assign2540_e4320: f64 = (assign2540_e4316 + assign2540_e4319);
        let assign2540_e4323: f64 = (p.p982 * locals.var_inv_lw);
        let assign2540_e4324: f64 = (assign2540_e4320 + assign2540_e4323);
        locals.var_pparam_b4soivrec0d = assign2540_e4324;
        locals.var_pparam_b4soivrec0d_dn3 = (((p.p602 * locals.var_inv_l_dn3) + (p.p792 * locals.var_inv_w_dn3)) + (p.p982 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivrec0d_dn4 = (((p.p602 * locals.var_inv_l_dn4) + (p.p792 * locals.var_inv_w_dn4)) + (p.p982 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivrec0d_dn5 = (((p.p602 * locals.var_inv_l_dn5) + (p.p792 * locals.var_inv_w_dn5)) + (p.p982 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivrec0d_dn6 = (((p.p602 * locals.var_inv_l_dn6) + (p.p792 * locals.var_inv_w_dn6)) + (p.p982 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivrec0d_dn7 = (((p.p602 * locals.var_inv_l_dn7) + (p.p792 * locals.var_inv_w_dn7)) + (p.p982 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivrec0d_dn8 = (((p.p602 * locals.var_inv_l_dn8) + (p.p792 * locals.var_inv_w_dn8)) + (p.p982 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivrec0d_dn9 = (((p.p602 * locals.var_inv_l_dn9) + (p.p792 * locals.var_inv_w_dn9)) + (p.p982 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivrec0d_dn10 = (((p.p602 * locals.var_inv_l_dn10) + (p.p792 * locals.var_inv_w_dn10)) + (p.p982 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivrec0d_dn11 = (((p.p602 * locals.var_inv_l_dn11) + (p.p792 * locals.var_inv_w_dn11)) + (p.p982 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivrec0d_dn12 = (((p.p602 * locals.var_inv_l_dn12) + (p.p792 * locals.var_inv_w_dn12)) + (p.p982 * locals.var_inv_lw_dn12));

        let assign2550_e4328: f64 = (p.p603 * locals.var_inv_l);
        let assign2550_e4329: f64 = (p.p339 + assign2550_e4328);
        let assign2550_e4332: f64 = (p.p793 * locals.var_inv_w);
        let assign2550_e4333: f64 = (assign2550_e4329 + assign2550_e4332);
        let assign2550_e4336: f64 = (p.p983 * locals.var_inv_lw);
        let assign2550_e4337: f64 = (assign2550_e4333 + assign2550_e4336);
        locals.var_pparam_b4soivtun0 = assign2550_e4337;
        locals.var_pparam_b4soivtun0_dn3 = (((p.p603 * locals.var_inv_l_dn3) + (p.p793 * locals.var_inv_w_dn3)) + (p.p983 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivtun0_dn4 = (((p.p603 * locals.var_inv_l_dn4) + (p.p793 * locals.var_inv_w_dn4)) + (p.p983 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivtun0_dn5 = (((p.p603 * locals.var_inv_l_dn5) + (p.p793 * locals.var_inv_w_dn5)) + (p.p983 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivtun0_dn6 = (((p.p603 * locals.var_inv_l_dn6) + (p.p793 * locals.var_inv_w_dn6)) + (p.p983 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivtun0_dn7 = (((p.p603 * locals.var_inv_l_dn7) + (p.p793 * locals.var_inv_w_dn7)) + (p.p983 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivtun0_dn8 = (((p.p603 * locals.var_inv_l_dn8) + (p.p793 * locals.var_inv_w_dn8)) + (p.p983 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivtun0_dn9 = (((p.p603 * locals.var_inv_l_dn9) + (p.p793 * locals.var_inv_w_dn9)) + (p.p983 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivtun0_dn10 = (((p.p603 * locals.var_inv_l_dn10) + (p.p793 * locals.var_inv_w_dn10)) + (p.p983 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivtun0_dn11 = (((p.p603 * locals.var_inv_l_dn11) + (p.p793 * locals.var_inv_w_dn11)) + (p.p983 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivtun0_dn12 = (((p.p603 * locals.var_inv_l_dn12) + (p.p793 * locals.var_inv_w_dn12)) + (p.p983 * locals.var_inv_lw_dn12));

        let assign2560_e4341: f64 = (p.p604 * locals.var_inv_l);
        let assign2560_e4342: f64 = (p.p340 + assign2560_e4341);
        let assign2560_e4345: f64 = (p.p794 * locals.var_inv_w);
        let assign2560_e4346: f64 = (assign2560_e4342 + assign2560_e4345);
        let assign2560_e4349: f64 = (p.p984 * locals.var_inv_lw);
        let assign2560_e4350: f64 = (assign2560_e4346 + assign2560_e4349);
        locals.var_pparam_b4soivtun0d = assign2560_e4350;
        locals.var_pparam_b4soivtun0d_dn3 = (((p.p604 * locals.var_inv_l_dn3) + (p.p794 * locals.var_inv_w_dn3)) + (p.p984 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivtun0d_dn4 = (((p.p604 * locals.var_inv_l_dn4) + (p.p794 * locals.var_inv_w_dn4)) + (p.p984 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivtun0d_dn5 = (((p.p604 * locals.var_inv_l_dn5) + (p.p794 * locals.var_inv_w_dn5)) + (p.p984 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivtun0d_dn6 = (((p.p604 * locals.var_inv_l_dn6) + (p.p794 * locals.var_inv_w_dn6)) + (p.p984 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivtun0d_dn7 = (((p.p604 * locals.var_inv_l_dn7) + (p.p794 * locals.var_inv_w_dn7)) + (p.p984 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivtun0d_dn8 = (((p.p604 * locals.var_inv_l_dn8) + (p.p794 * locals.var_inv_w_dn8)) + (p.p984 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivtun0d_dn9 = (((p.p604 * locals.var_inv_l_dn9) + (p.p794 * locals.var_inv_w_dn9)) + (p.p984 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivtun0d_dn10 = (((p.p604 * locals.var_inv_l_dn10) + (p.p794 * locals.var_inv_w_dn10)) + (p.p984 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivtun0d_dn11 = (((p.p604 * locals.var_inv_l_dn11) + (p.p794 * locals.var_inv_w_dn11)) + (p.p984 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivtun0d_dn12 = (((p.p604 * locals.var_inv_l_dn12) + (p.p794 * locals.var_inv_w_dn12)) + (p.p984 * locals.var_inv_lw_dn12));

        let assign2570_e4354: f64 = (p.p605 * locals.var_inv_l);
        let assign2570_e4355: f64 = (p.p341 + assign2570_e4354);
        let assign2570_e4358: f64 = (p.p795 * locals.var_inv_w);
        let assign2570_e4359: f64 = (assign2570_e4355 + assign2570_e4358);
        let assign2570_e4362: f64 = (p.p985 * locals.var_inv_lw);
        let assign2570_e4363: f64 = (assign2570_e4359 + assign2570_e4362);
        locals.var_pparam_b4soinbjt = assign2570_e4363;
        locals.var_pparam_b4soinbjt_dn3 = (((p.p605 * locals.var_inv_l_dn3) + (p.p795 * locals.var_inv_w_dn3)) + (p.p985 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soinbjt_dn4 = (((p.p605 * locals.var_inv_l_dn4) + (p.p795 * locals.var_inv_w_dn4)) + (p.p985 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soinbjt_dn5 = (((p.p605 * locals.var_inv_l_dn5) + (p.p795 * locals.var_inv_w_dn5)) + (p.p985 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soinbjt_dn6 = (((p.p605 * locals.var_inv_l_dn6) + (p.p795 * locals.var_inv_w_dn6)) + (p.p985 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soinbjt_dn7 = (((p.p605 * locals.var_inv_l_dn7) + (p.p795 * locals.var_inv_w_dn7)) + (p.p985 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soinbjt_dn8 = (((p.p605 * locals.var_inv_l_dn8) + (p.p795 * locals.var_inv_w_dn8)) + (p.p985 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soinbjt_dn9 = (((p.p605 * locals.var_inv_l_dn9) + (p.p795 * locals.var_inv_w_dn9)) + (p.p985 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soinbjt_dn10 = (((p.p605 * locals.var_inv_l_dn10) + (p.p795 * locals.var_inv_w_dn10)) + (p.p985 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soinbjt_dn11 = (((p.p605 * locals.var_inv_l_dn11) + (p.p795 * locals.var_inv_w_dn11)) + (p.p985 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soinbjt_dn12 = (((p.p605 * locals.var_inv_l_dn12) + (p.p795 * locals.var_inv_w_dn12)) + (p.p985 * locals.var_inv_lw_dn12));

        let assign2580_e4367: f64 = (p.p606 * locals.var_inv_l);
        let assign2580_e4368: f64 = (p.p342 + assign2580_e4367);
        let assign2580_e4371: f64 = (p.p796 * locals.var_inv_w);
        let assign2580_e4372: f64 = (assign2580_e4368 + assign2580_e4371);
        let assign2580_e4375: f64 = (p.p986 * locals.var_inv_lw);
        let assign2580_e4376: f64 = (assign2580_e4372 + assign2580_e4375);
        locals.var_pparam_b4soilbjt0 = assign2580_e4376;
        locals.var_pparam_b4soilbjt0_dn3 = (((p.p606 * locals.var_inv_l_dn3) + (p.p796 * locals.var_inv_w_dn3)) + (p.p986 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soilbjt0_dn4 = (((p.p606 * locals.var_inv_l_dn4) + (p.p796 * locals.var_inv_w_dn4)) + (p.p986 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soilbjt0_dn5 = (((p.p606 * locals.var_inv_l_dn5) + (p.p796 * locals.var_inv_w_dn5)) + (p.p986 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soilbjt0_dn6 = (((p.p606 * locals.var_inv_l_dn6) + (p.p796 * locals.var_inv_w_dn6)) + (p.p986 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soilbjt0_dn7 = (((p.p606 * locals.var_inv_l_dn7) + (p.p796 * locals.var_inv_w_dn7)) + (p.p986 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soilbjt0_dn8 = (((p.p606 * locals.var_inv_l_dn8) + (p.p796 * locals.var_inv_w_dn8)) + (p.p986 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soilbjt0_dn9 = (((p.p606 * locals.var_inv_l_dn9) + (p.p796 * locals.var_inv_w_dn9)) + (p.p986 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soilbjt0_dn10 = (((p.p606 * locals.var_inv_l_dn10) + (p.p796 * locals.var_inv_w_dn10)) + (p.p986 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soilbjt0_dn11 = (((p.p606 * locals.var_inv_l_dn11) + (p.p796 * locals.var_inv_w_dn11)) + (p.p986 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soilbjt0_dn12 = (((p.p606 * locals.var_inv_l_dn12) + (p.p796 * locals.var_inv_w_dn12)) + (p.p986 * locals.var_inv_lw_dn12));

        let assign2590_e4380: f64 = (p.p607 * locals.var_inv_l);
        let assign2590_e4381: f64 = (p.p344 + assign2590_e4380);
        let assign2590_e4384: f64 = (p.p797 * locals.var_inv_w);
        let assign2590_e4385: f64 = (assign2590_e4381 + assign2590_e4384);
        let assign2590_e4388: f64 = (p.p987 * locals.var_inv_lw);
        let assign2590_e4389: f64 = (assign2590_e4385 + assign2590_e4388);
        locals.var_pparam_b4soivabjt = assign2590_e4389;
        locals.var_pparam_b4soivabjt_dn3 = (((p.p607 * locals.var_inv_l_dn3) + (p.p797 * locals.var_inv_w_dn3)) + (p.p987 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivabjt_dn4 = (((p.p607 * locals.var_inv_l_dn4) + (p.p797 * locals.var_inv_w_dn4)) + (p.p987 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivabjt_dn5 = (((p.p607 * locals.var_inv_l_dn5) + (p.p797 * locals.var_inv_w_dn5)) + (p.p987 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivabjt_dn6 = (((p.p607 * locals.var_inv_l_dn6) + (p.p797 * locals.var_inv_w_dn6)) + (p.p987 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivabjt_dn7 = (((p.p607 * locals.var_inv_l_dn7) + (p.p797 * locals.var_inv_w_dn7)) + (p.p987 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivabjt_dn8 = (((p.p607 * locals.var_inv_l_dn8) + (p.p797 * locals.var_inv_w_dn8)) + (p.p987 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivabjt_dn9 = (((p.p607 * locals.var_inv_l_dn9) + (p.p797 * locals.var_inv_w_dn9)) + (p.p987 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivabjt_dn10 = (((p.p607 * locals.var_inv_l_dn10) + (p.p797 * locals.var_inv_w_dn10)) + (p.p987 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivabjt_dn11 = (((p.p607 * locals.var_inv_l_dn11) + (p.p797 * locals.var_inv_w_dn11)) + (p.p987 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivabjt_dn12 = (((p.p607 * locals.var_inv_l_dn12) + (p.p797 * locals.var_inv_w_dn12)) + (p.p987 * locals.var_inv_lw_dn12));

        let assign2600_e4393: f64 = (p.p608 * locals.var_inv_l);
        let assign2600_e4394: f64 = (p.p345 + assign2600_e4393);
        let assign2600_e4397: f64 = (p.p798 * locals.var_inv_w);
        let assign2600_e4398: f64 = (assign2600_e4394 + assign2600_e4397);
        let assign2600_e4401: f64 = (p.p988 * locals.var_inv_lw);
        let assign2600_e4402: f64 = (assign2600_e4398 + assign2600_e4401);
        locals.var_pparam_b4soiaely = assign2600_e4402;
        locals.var_pparam_b4soiaely_dn3 = (((p.p608 * locals.var_inv_l_dn3) + (p.p798 * locals.var_inv_w_dn3)) + (p.p988 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiaely_dn4 = (((p.p608 * locals.var_inv_l_dn4) + (p.p798 * locals.var_inv_w_dn4)) + (p.p988 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiaely_dn5 = (((p.p608 * locals.var_inv_l_dn5) + (p.p798 * locals.var_inv_w_dn5)) + (p.p988 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiaely_dn6 = (((p.p608 * locals.var_inv_l_dn6) + (p.p798 * locals.var_inv_w_dn6)) + (p.p988 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiaely_dn7 = (((p.p608 * locals.var_inv_l_dn7) + (p.p798 * locals.var_inv_w_dn7)) + (p.p988 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiaely_dn8 = (((p.p608 * locals.var_inv_l_dn8) + (p.p798 * locals.var_inv_w_dn8)) + (p.p988 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiaely_dn9 = (((p.p608 * locals.var_inv_l_dn9) + (p.p798 * locals.var_inv_w_dn9)) + (p.p988 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiaely_dn10 = (((p.p608 * locals.var_inv_l_dn10) + (p.p798 * locals.var_inv_w_dn10)) + (p.p988 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiaely_dn11 = (((p.p608 * locals.var_inv_l_dn11) + (p.p798 * locals.var_inv_w_dn11)) + (p.p988 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiaely_dn12 = (((p.p608 * locals.var_inv_l_dn12) + (p.p798 * locals.var_inv_w_dn12)) + (p.p988 * locals.var_inv_lw_dn12));

        let assign2610_e4406: f64 = (p.p609 * locals.var_inv_l);
        let assign2610_e4407: f64 = (p.p346 + assign2610_e4406);
        let assign2610_e4410: f64 = (p.p799 * locals.var_inv_w);
        let assign2610_e4411: f64 = (assign2610_e4407 + assign2610_e4410);
        let assign2610_e4414: f64 = (p.p989 * locals.var_inv_lw);
        let assign2610_e4415: f64 = (assign2610_e4411 + assign2610_e4414);
        locals.var_pparam_b4soiahli = assign2610_e4415;
        locals.var_pparam_b4soiahli_dn3 = (((p.p609 * locals.var_inv_l_dn3) + (p.p799 * locals.var_inv_w_dn3)) + (p.p989 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiahli_dn4 = (((p.p609 * locals.var_inv_l_dn4) + (p.p799 * locals.var_inv_w_dn4)) + (p.p989 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiahli_dn5 = (((p.p609 * locals.var_inv_l_dn5) + (p.p799 * locals.var_inv_w_dn5)) + (p.p989 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiahli_dn6 = (((p.p609 * locals.var_inv_l_dn6) + (p.p799 * locals.var_inv_w_dn6)) + (p.p989 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiahli_dn7 = (((p.p609 * locals.var_inv_l_dn7) + (p.p799 * locals.var_inv_w_dn7)) + (p.p989 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiahli_dn8 = (((p.p609 * locals.var_inv_l_dn8) + (p.p799 * locals.var_inv_w_dn8)) + (p.p989 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiahli_dn9 = (((p.p609 * locals.var_inv_l_dn9) + (p.p799 * locals.var_inv_w_dn9)) + (p.p989 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiahli_dn10 = (((p.p609 * locals.var_inv_l_dn10) + (p.p799 * locals.var_inv_w_dn10)) + (p.p989 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiahli_dn11 = (((p.p609 * locals.var_inv_l_dn11) + (p.p799 * locals.var_inv_w_dn11)) + (p.p989 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiahli_dn12 = (((p.p609 * locals.var_inv_l_dn12) + (p.p799 * locals.var_inv_w_dn12)) + (p.p989 * locals.var_inv_lw_dn12));

        let assign2620_e4419: f64 = (p.p610 * locals.var_inv_l);
        let assign2620_e4420: f64 = (p.p347 + assign2620_e4419);
        let assign2620_e4423: f64 = (p.p800 * locals.var_inv_w);
        let assign2620_e4424: f64 = (assign2620_e4420 + assign2620_e4423);
        let assign2620_e4427: f64 = (p.p990 * locals.var_inv_lw);
        let assign2620_e4428: f64 = (assign2620_e4424 + assign2620_e4427);
        locals.var_pparam_b4soiahlid = assign2620_e4428;
        locals.var_pparam_b4soiahlid_dn3 = (((p.p610 * locals.var_inv_l_dn3) + (p.p800 * locals.var_inv_w_dn3)) + (p.p990 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiahlid_dn4 = (((p.p610 * locals.var_inv_l_dn4) + (p.p800 * locals.var_inv_w_dn4)) + (p.p990 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiahlid_dn5 = (((p.p610 * locals.var_inv_l_dn5) + (p.p800 * locals.var_inv_w_dn5)) + (p.p990 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiahlid_dn6 = (((p.p610 * locals.var_inv_l_dn6) + (p.p800 * locals.var_inv_w_dn6)) + (p.p990 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiahlid_dn7 = (((p.p610 * locals.var_inv_l_dn7) + (p.p800 * locals.var_inv_w_dn7)) + (p.p990 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiahlid_dn8 = (((p.p610 * locals.var_inv_l_dn8) + (p.p800 * locals.var_inv_w_dn8)) + (p.p990 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiahlid_dn9 = (((p.p610 * locals.var_inv_l_dn9) + (p.p800 * locals.var_inv_w_dn9)) + (p.p990 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiahlid_dn10 = (((p.p610 * locals.var_inv_l_dn10) + (p.p800 * locals.var_inv_w_dn10)) + (p.p990 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiahlid_dn11 = (((p.p610 * locals.var_inv_l_dn11) + (p.p800 * locals.var_inv_w_dn11)) + (p.p990 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiahlid_dn12 = (((p.p610 * locals.var_inv_l_dn12) + (p.p800 * locals.var_inv_w_dn12)) + (p.p990 * locals.var_inv_lw_dn12));

        let assign2630_e4432: f64 = (p.p443 * locals.var_inv_l);
        let assign2630_e4433: f64 = (p.p157 + assign2630_e4432);
        let assign2630_e4436: f64 = (p.p633 * locals.var_inv_w);
        let assign2630_e4437: f64 = (assign2630_e4433 + assign2630_e4436);
        let assign2630_e4440: f64 = (p.p823 * locals.var_inv_lw);
        let assign2630_e4441: f64 = (assign2630_e4437 + assign2630_e4440);
        locals.var_pparam_b4soixj = assign2630_e4441;
        locals.var_pparam_b4soixj_dn3 = (((p.p443 * locals.var_inv_l_dn3) + (p.p633 * locals.var_inv_w_dn3)) + (p.p823 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soixj_dn4 = (((p.p443 * locals.var_inv_l_dn4) + (p.p633 * locals.var_inv_w_dn4)) + (p.p823 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soixj_dn5 = (((p.p443 * locals.var_inv_l_dn5) + (p.p633 * locals.var_inv_w_dn5)) + (p.p823 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soixj_dn6 = (((p.p443 * locals.var_inv_l_dn6) + (p.p633 * locals.var_inv_w_dn6)) + (p.p823 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soixj_dn7 = (((p.p443 * locals.var_inv_l_dn7) + (p.p633 * locals.var_inv_w_dn7)) + (p.p823 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soixj_dn8 = (((p.p443 * locals.var_inv_l_dn8) + (p.p633 * locals.var_inv_w_dn8)) + (p.p823 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soixj_dn9 = (((p.p443 * locals.var_inv_l_dn9) + (p.p633 * locals.var_inv_w_dn9)) + (p.p823 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soixj_dn10 = (((p.p443 * locals.var_inv_l_dn10) + (p.p633 * locals.var_inv_w_dn10)) + (p.p823 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soixj_dn11 = (((p.p443 * locals.var_inv_l_dn11) + (p.p633 * locals.var_inv_w_dn11)) + (p.p823 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soixj_dn12 = (((p.p443 * locals.var_inv_l_dn12) + (p.p633 * locals.var_inv_w_dn12)) + (p.p823 * locals.var_inv_lw_dn12));

        let assign2640_e4445: f64 = (p.p444 * locals.var_inv_l);
        let assign2640_e4446: f64 = (p.p383 + assign2640_e4445);
        let assign2640_e4449: f64 = (p.p634 * locals.var_inv_w);
        let assign2640_e4450: f64 = (assign2640_e4446 + assign2640_e4449);
        let assign2640_e4453: f64 = (p.p824 * locals.var_inv_lw);
        let assign2640_e4454: f64 = (assign2640_e4450 + assign2640_e4453);
        locals.var_pparam_b4soialphagb1 = assign2640_e4454;
        locals.var_pparam_b4soialphagb1_dn3 = (((p.p444 * locals.var_inv_l_dn3) + (p.p634 * locals.var_inv_w_dn3)) + (p.p824 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soialphagb1_dn4 = (((p.p444 * locals.var_inv_l_dn4) + (p.p634 * locals.var_inv_w_dn4)) + (p.p824 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soialphagb1_dn5 = (((p.p444 * locals.var_inv_l_dn5) + (p.p634 * locals.var_inv_w_dn5)) + (p.p824 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soialphagb1_dn6 = (((p.p444 * locals.var_inv_l_dn6) + (p.p634 * locals.var_inv_w_dn6)) + (p.p824 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soialphagb1_dn7 = (((p.p444 * locals.var_inv_l_dn7) + (p.p634 * locals.var_inv_w_dn7)) + (p.p824 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soialphagb1_dn8 = (((p.p444 * locals.var_inv_l_dn8) + (p.p634 * locals.var_inv_w_dn8)) + (p.p824 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soialphagb1_dn9 = (((p.p444 * locals.var_inv_l_dn9) + (p.p634 * locals.var_inv_w_dn9)) + (p.p824 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soialphagb1_dn10 = (((p.p444 * locals.var_inv_l_dn10) + (p.p634 * locals.var_inv_w_dn10)) + (p.p824 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soialphagb1_dn11 = (((p.p444 * locals.var_inv_l_dn11) + (p.p634 * locals.var_inv_w_dn11)) + (p.p824 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soialphagb1_dn12 = (((p.p444 * locals.var_inv_l_dn12) + (p.p634 * locals.var_inv_w_dn12)) + (p.p824 * locals.var_inv_lw_dn12));

        let assign2650_e4458: f64 = (p.p445 * locals.var_inv_l);
        let assign2650_e4459: f64 = (p.p384 + assign2650_e4458);
        let assign2650_e4462: f64 = (p.p635 * locals.var_inv_w);
        let assign2650_e4463: f64 = (assign2650_e4459 + assign2650_e4462);
        let assign2650_e4466: f64 = (p.p825 * locals.var_inv_lw);
        let assign2650_e4467: f64 = (assign2650_e4463 + assign2650_e4466);
        locals.var_pparam_b4soialphagb1_t = assign2650_e4467;
        locals.var_pparam_b4soialphagb1_t_dn3 = (((p.p445 * locals.var_inv_l_dn3) + (p.p635 * locals.var_inv_w_dn3)) + (p.p825 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soialphagb1_t_dn4 = (((p.p445 * locals.var_inv_l_dn4) + (p.p635 * locals.var_inv_w_dn4)) + (p.p825 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soialphagb1_t_dn5 = (((p.p445 * locals.var_inv_l_dn5) + (p.p635 * locals.var_inv_w_dn5)) + (p.p825 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soialphagb1_t_dn6 = (((p.p445 * locals.var_inv_l_dn6) + (p.p635 * locals.var_inv_w_dn6)) + (p.p825 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soialphagb1_t_dn7 = (((p.p445 * locals.var_inv_l_dn7) + (p.p635 * locals.var_inv_w_dn7)) + (p.p825 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soialphagb1_t_dn8 = (((p.p445 * locals.var_inv_l_dn8) + (p.p635 * locals.var_inv_w_dn8)) + (p.p825 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soialphagb1_t_dn9 = (((p.p445 * locals.var_inv_l_dn9) + (p.p635 * locals.var_inv_w_dn9)) + (p.p825 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soialphagb1_t_dn10 = (((p.p445 * locals.var_inv_l_dn10) + (p.p635 * locals.var_inv_w_dn10)) + (p.p825 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soialphagb1_t_dn11 = (((p.p445 * locals.var_inv_l_dn11) + (p.p635 * locals.var_inv_w_dn11)) + (p.p825 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soialphagb1_t_dn12 = (((p.p445 * locals.var_inv_l_dn12) + (p.p635 * locals.var_inv_w_dn12)) + (p.p825 * locals.var_inv_lw_dn12));

        let assign2660_e4471: f64 = (p.p447 * locals.var_inv_l);
        let assign2660_e4472: f64 = (p.p388 + assign2660_e4471);
        let assign2660_e4475: f64 = (p.p637 * locals.var_inv_w);
        let assign2660_e4476: f64 = (assign2660_e4472 + assign2660_e4475);
        let assign2660_e4479: f64 = (p.p827 * locals.var_inv_lw);
        let assign2660_e4480: f64 = (assign2660_e4476 + assign2660_e4479);
        locals.var_pparam_b4soialphagb2 = assign2660_e4480;
        locals.var_pparam_b4soialphagb2_dn3 = (((p.p447 * locals.var_inv_l_dn3) + (p.p637 * locals.var_inv_w_dn3)) + (p.p827 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soialphagb2_dn4 = (((p.p447 * locals.var_inv_l_dn4) + (p.p637 * locals.var_inv_w_dn4)) + (p.p827 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soialphagb2_dn5 = (((p.p447 * locals.var_inv_l_dn5) + (p.p637 * locals.var_inv_w_dn5)) + (p.p827 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soialphagb2_dn6 = (((p.p447 * locals.var_inv_l_dn6) + (p.p637 * locals.var_inv_w_dn6)) + (p.p827 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soialphagb2_dn7 = (((p.p447 * locals.var_inv_l_dn7) + (p.p637 * locals.var_inv_w_dn7)) + (p.p827 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soialphagb2_dn8 = (((p.p447 * locals.var_inv_l_dn8) + (p.p637 * locals.var_inv_w_dn8)) + (p.p827 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soialphagb2_dn9 = (((p.p447 * locals.var_inv_l_dn9) + (p.p637 * locals.var_inv_w_dn9)) + (p.p827 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soialphagb2_dn10 = (((p.p447 * locals.var_inv_l_dn10) + (p.p637 * locals.var_inv_w_dn10)) + (p.p827 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soialphagb2_dn11 = (((p.p447 * locals.var_inv_l_dn11) + (p.p637 * locals.var_inv_w_dn11)) + (p.p827 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soialphagb2_dn12 = (((p.p447 * locals.var_inv_l_dn12) + (p.p637 * locals.var_inv_w_dn12)) + (p.p827 * locals.var_inv_lw_dn12));

        let assign2670_e4484: f64 = (p.p448 * locals.var_inv_l);
        let assign2670_e4485: f64 = (p.p389 + assign2670_e4484);
        let assign2670_e4488: f64 = (p.p638 * locals.var_inv_w);
        let assign2670_e4489: f64 = (assign2670_e4485 + assign2670_e4488);
        let assign2670_e4492: f64 = (p.p828 * locals.var_inv_lw);
        let assign2670_e4493: f64 = (assign2670_e4489 + assign2670_e4492);
        locals.var_pparam_b4soialphagb2_t = assign2670_e4493;
        locals.var_pparam_b4soialphagb2_t_dn3 = (((p.p448 * locals.var_inv_l_dn3) + (p.p638 * locals.var_inv_w_dn3)) + (p.p828 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soialphagb2_t_dn4 = (((p.p448 * locals.var_inv_l_dn4) + (p.p638 * locals.var_inv_w_dn4)) + (p.p828 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soialphagb2_t_dn5 = (((p.p448 * locals.var_inv_l_dn5) + (p.p638 * locals.var_inv_w_dn5)) + (p.p828 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soialphagb2_t_dn6 = (((p.p448 * locals.var_inv_l_dn6) + (p.p638 * locals.var_inv_w_dn6)) + (p.p828 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soialphagb2_t_dn7 = (((p.p448 * locals.var_inv_l_dn7) + (p.p638 * locals.var_inv_w_dn7)) + (p.p828 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soialphagb2_t_dn8 = (((p.p448 * locals.var_inv_l_dn8) + (p.p638 * locals.var_inv_w_dn8)) + (p.p828 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soialphagb2_t_dn9 = (((p.p448 * locals.var_inv_l_dn9) + (p.p638 * locals.var_inv_w_dn9)) + (p.p828 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soialphagb2_t_dn10 = (((p.p448 * locals.var_inv_l_dn10) + (p.p638 * locals.var_inv_w_dn10)) + (p.p828 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soialphagb2_t_dn11 = (((p.p448 * locals.var_inv_l_dn11) + (p.p638 * locals.var_inv_w_dn11)) + (p.p828 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soialphagb2_t_dn12 = (((p.p448 * locals.var_inv_l_dn12) + (p.p638 * locals.var_inv_w_dn12)) + (p.p828 * locals.var_inv_lw_dn12));

        let assign2680_e4497: f64 = (p.p446 * locals.var_inv_l);
        let assign2680_e4498: f64 = (p.p385 + assign2680_e4497);
        let assign2680_e4501: f64 = (p.p636 * locals.var_inv_w);
        let assign2680_e4502: f64 = (assign2680_e4498 + assign2680_e4501);
        let assign2680_e4505: f64 = (p.p826 * locals.var_inv_lw);
        let assign2680_e4506: f64 = (assign2680_e4502 + assign2680_e4505);
        locals.var_pparam_b4soibetagb1 = assign2680_e4506;
        locals.var_pparam_b4soibetagb1_dn3 = (((p.p446 * locals.var_inv_l_dn3) + (p.p636 * locals.var_inv_w_dn3)) + (p.p826 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soibetagb1_dn4 = (((p.p446 * locals.var_inv_l_dn4) + (p.p636 * locals.var_inv_w_dn4)) + (p.p826 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soibetagb1_dn5 = (((p.p446 * locals.var_inv_l_dn5) + (p.p636 * locals.var_inv_w_dn5)) + (p.p826 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soibetagb1_dn6 = (((p.p446 * locals.var_inv_l_dn6) + (p.p636 * locals.var_inv_w_dn6)) + (p.p826 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soibetagb1_dn7 = (((p.p446 * locals.var_inv_l_dn7) + (p.p636 * locals.var_inv_w_dn7)) + (p.p826 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soibetagb1_dn8 = (((p.p446 * locals.var_inv_l_dn8) + (p.p636 * locals.var_inv_w_dn8)) + (p.p826 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soibetagb1_dn9 = (((p.p446 * locals.var_inv_l_dn9) + (p.p636 * locals.var_inv_w_dn9)) + (p.p826 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soibetagb1_dn10 = (((p.p446 * locals.var_inv_l_dn10) + (p.p636 * locals.var_inv_w_dn10)) + (p.p826 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soibetagb1_dn11 = (((p.p446 * locals.var_inv_l_dn11) + (p.p636 * locals.var_inv_w_dn11)) + (p.p826 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soibetagb1_dn12 = (((p.p446 * locals.var_inv_l_dn12) + (p.p636 * locals.var_inv_w_dn12)) + (p.p826 * locals.var_inv_lw_dn12));

        let assign2690_e4510: f64 = (p.p449 * locals.var_inv_l);
        let assign2690_e4511: f64 = (p.p390 + assign2690_e4510);
        let assign2690_e4514: f64 = (p.p639 * locals.var_inv_w);
        let assign2690_e4515: f64 = (assign2690_e4511 + assign2690_e4514);
        let assign2690_e4518: f64 = (p.p829 * locals.var_inv_lw);
        let assign2690_e4519: f64 = (assign2690_e4515 + assign2690_e4518);
        locals.var_pparam_b4soibetagb2 = assign2690_e4519;
        locals.var_pparam_b4soibetagb2_dn3 = (((p.p449 * locals.var_inv_l_dn3) + (p.p639 * locals.var_inv_w_dn3)) + (p.p829 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soibetagb2_dn4 = (((p.p449 * locals.var_inv_l_dn4) + (p.p639 * locals.var_inv_w_dn4)) + (p.p829 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soibetagb2_dn5 = (((p.p449 * locals.var_inv_l_dn5) + (p.p639 * locals.var_inv_w_dn5)) + (p.p829 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soibetagb2_dn6 = (((p.p449 * locals.var_inv_l_dn6) + (p.p639 * locals.var_inv_w_dn6)) + (p.p829 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soibetagb2_dn7 = (((p.p449 * locals.var_inv_l_dn7) + (p.p639 * locals.var_inv_w_dn7)) + (p.p829 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soibetagb2_dn8 = (((p.p449 * locals.var_inv_l_dn8) + (p.p639 * locals.var_inv_w_dn8)) + (p.p829 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soibetagb2_dn9 = (((p.p449 * locals.var_inv_l_dn9) + (p.p639 * locals.var_inv_w_dn9)) + (p.p829 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soibetagb2_dn10 = (((p.p449 * locals.var_inv_l_dn10) + (p.p639 * locals.var_inv_w_dn10)) + (p.p829 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soibetagb2_dn11 = (((p.p449 * locals.var_inv_l_dn11) + (p.p639 * locals.var_inv_w_dn11)) + (p.p829 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soibetagb2_dn12 = (((p.p449 * locals.var_inv_l_dn12) + (p.p639 * locals.var_inv_w_dn12)) + (p.p829 * locals.var_inv_lw_dn12));

        let assign2700_e4523: f64 = (p.p457 * locals.var_inv_l);
        let assign2700_e4524: f64 = (p.p352 + assign2700_e4523);
        let assign2700_e4527: f64 = (p.p647 * locals.var_inv_w);
        let assign2700_e4528: f64 = (assign2700_e4524 + assign2700_e4527);
        let assign2700_e4531: f64 = (p.p837 * locals.var_inv_lw);
        let assign2700_e4532: f64 = (assign2700_e4528 + assign2700_e4531);
        locals.var_pparam_b4soindif = assign2700_e4532;
        locals.var_pparam_b4soindif_dn3 = (((p.p457 * locals.var_inv_l_dn3) + (p.p647 * locals.var_inv_w_dn3)) + (p.p837 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soindif_dn4 = (((p.p457 * locals.var_inv_l_dn4) + (p.p647 * locals.var_inv_w_dn4)) + (p.p837 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soindif_dn5 = (((p.p457 * locals.var_inv_l_dn5) + (p.p647 * locals.var_inv_w_dn5)) + (p.p837 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soindif_dn6 = (((p.p457 * locals.var_inv_l_dn6) + (p.p647 * locals.var_inv_w_dn6)) + (p.p837 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soindif_dn7 = (((p.p457 * locals.var_inv_l_dn7) + (p.p647 * locals.var_inv_w_dn7)) + (p.p837 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soindif_dn8 = (((p.p457 * locals.var_inv_l_dn8) + (p.p647 * locals.var_inv_w_dn8)) + (p.p837 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soindif_dn9 = (((p.p457 * locals.var_inv_l_dn9) + (p.p647 * locals.var_inv_w_dn9)) + (p.p837 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soindif_dn10 = (((p.p457 * locals.var_inv_l_dn10) + (p.p647 * locals.var_inv_w_dn10)) + (p.p837 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soindif_dn11 = (((p.p457 * locals.var_inv_l_dn11) + (p.p647 * locals.var_inv_w_dn11)) + (p.p837 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soindif_dn12 = (((p.p457 * locals.var_inv_l_dn12) + (p.p647 * locals.var_inv_w_dn12)) + (p.p837 * locals.var_inv_lw_dn12));

        let assign2710_e4536: f64 = (p.p467 * locals.var_inv_l);
        let assign2710_e4537: f64 = (p.p358 + assign2710_e4536);
        let assign2710_e4540: f64 = (p.p657 * locals.var_inv_w);
        let assign2710_e4541: f64 = (assign2710_e4537 + assign2710_e4540);
        let assign2710_e4544: f64 = (p.p847 * locals.var_inv_lw);
        let assign2710_e4545: f64 = (assign2710_e4541 + assign2710_e4544);
        locals.var_pparam_b4sointrecf = assign2710_e4545;
        locals.var_pparam_b4sointrecf_dn3 = (((p.p467 * locals.var_inv_l_dn3) + (p.p657 * locals.var_inv_w_dn3)) + (p.p847 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4sointrecf_dn4 = (((p.p467 * locals.var_inv_l_dn4) + (p.p657 * locals.var_inv_w_dn4)) + (p.p847 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4sointrecf_dn5 = (((p.p467 * locals.var_inv_l_dn5) + (p.p657 * locals.var_inv_w_dn5)) + (p.p847 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4sointrecf_dn6 = (((p.p467 * locals.var_inv_l_dn6) + (p.p657 * locals.var_inv_w_dn6)) + (p.p847 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4sointrecf_dn7 = (((p.p467 * locals.var_inv_l_dn7) + (p.p657 * locals.var_inv_w_dn7)) + (p.p847 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4sointrecf_dn8 = (((p.p467 * locals.var_inv_l_dn8) + (p.p657 * locals.var_inv_w_dn8)) + (p.p847 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4sointrecf_dn9 = (((p.p467 * locals.var_inv_l_dn9) + (p.p657 * locals.var_inv_w_dn9)) + (p.p847 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4sointrecf_dn10 = (((p.p467 * locals.var_inv_l_dn10) + (p.p657 * locals.var_inv_w_dn10)) + (p.p847 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4sointrecf_dn11 = (((p.p467 * locals.var_inv_l_dn11) + (p.p657 * locals.var_inv_w_dn11)) + (p.p847 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4sointrecf_dn12 = (((p.p467 * locals.var_inv_l_dn12) + (p.p657 * locals.var_inv_w_dn12)) + (p.p847 * locals.var_inv_lw_dn12));

        let assign2720_e4549: f64 = (p.p468 * locals.var_inv_l);
        let assign2720_e4550: f64 = (p.p359 + assign2720_e4549);
        let assign2720_e4553: f64 = (p.p658 * locals.var_inv_w);
        let assign2720_e4554: f64 = (assign2720_e4550 + assign2720_e4553);
        let assign2720_e4557: f64 = (p.p848 * locals.var_inv_lw);
        let assign2720_e4558: f64 = (assign2720_e4554 + assign2720_e4557);
        locals.var_pparam_b4sointrecr = assign2720_e4558;
        locals.var_pparam_b4sointrecr_dn3 = (((p.p468 * locals.var_inv_l_dn3) + (p.p658 * locals.var_inv_w_dn3)) + (p.p848 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4sointrecr_dn4 = (((p.p468 * locals.var_inv_l_dn4) + (p.p658 * locals.var_inv_w_dn4)) + (p.p848 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4sointrecr_dn5 = (((p.p468 * locals.var_inv_l_dn5) + (p.p658 * locals.var_inv_w_dn5)) + (p.p848 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4sointrecr_dn6 = (((p.p468 * locals.var_inv_l_dn6) + (p.p658 * locals.var_inv_w_dn6)) + (p.p848 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4sointrecr_dn7 = (((p.p468 * locals.var_inv_l_dn7) + (p.p658 * locals.var_inv_w_dn7)) + (p.p848 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4sointrecr_dn8 = (((p.p468 * locals.var_inv_l_dn8) + (p.p658 * locals.var_inv_w_dn8)) + (p.p848 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4sointrecr_dn9 = (((p.p468 * locals.var_inv_l_dn9) + (p.p658 * locals.var_inv_w_dn9)) + (p.p848 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4sointrecr_dn10 = (((p.p468 * locals.var_inv_l_dn10) + (p.p658 * locals.var_inv_w_dn10)) + (p.p848 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4sointrecr_dn11 = (((p.p468 * locals.var_inv_l_dn11) + (p.p658 * locals.var_inv_w_dn11)) + (p.p848 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4sointrecr_dn12 = (((p.p468 * locals.var_inv_l_dn12) + (p.p658 * locals.var_inv_w_dn12)) + (p.p848 * locals.var_inv_lw_dn12));

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2730_e4562: f64 = (p.p469 * locals.var_inv_l);
        let assign2730_e4563: f64 = (p.p174 + assign2730_e4562);
        let assign2730_e4566: f64 = (p.p659 * locals.var_inv_w);
        let assign2730_e4567: f64 = (assign2730_e4563 + assign2730_e4566);
        let assign2730_e4570: f64 = (p.p849 * locals.var_inv_lw);
        let assign2730_e4571: f64 = (assign2730_e4567 + assign2730_e4570);
        locals.var_pparam_b4soixbjt = assign2730_e4571;
        locals.var_pparam_b4soixbjt_dn3 = (((p.p469 * locals.var_inv_l_dn3) + (p.p659 * locals.var_inv_w_dn3)) + (p.p849 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soixbjt_dn4 = (((p.p469 * locals.var_inv_l_dn4) + (p.p659 * locals.var_inv_w_dn4)) + (p.p849 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soixbjt_dn5 = (((p.p469 * locals.var_inv_l_dn5) + (p.p659 * locals.var_inv_w_dn5)) + (p.p849 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soixbjt_dn6 = (((p.p469 * locals.var_inv_l_dn6) + (p.p659 * locals.var_inv_w_dn6)) + (p.p849 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soixbjt_dn7 = (((p.p469 * locals.var_inv_l_dn7) + (p.p659 * locals.var_inv_w_dn7)) + (p.p849 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soixbjt_dn8 = (((p.p469 * locals.var_inv_l_dn8) + (p.p659 * locals.var_inv_w_dn8)) + (p.p849 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soixbjt_dn9 = (((p.p469 * locals.var_inv_l_dn9) + (p.p659 * locals.var_inv_w_dn9)) + (p.p849 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soixbjt_dn10 = (((p.p469 * locals.var_inv_l_dn10) + (p.p659 * locals.var_inv_w_dn10)) + (p.p849 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soixbjt_dn11 = (((p.p469 * locals.var_inv_l_dn11) + (p.p659 * locals.var_inv_w_dn11)) + (p.p849 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soixbjt_dn12 = (((p.p469 * locals.var_inv_l_dn12) + (p.p659 * locals.var_inv_w_dn12)) + (p.p849 * locals.var_inv_lw_dn12));

        let assign2740_e4575: f64 = (p.p470 * locals.var_inv_l);
        let assign2740_e4576: f64 = (p.p175 + assign2740_e4575);
        let assign2740_e4579: f64 = (p.p660 * locals.var_inv_w);
        let assign2740_e4580: f64 = (assign2740_e4576 + assign2740_e4579);
        let assign2740_e4583: f64 = (p.p850 * locals.var_inv_lw);
        let assign2740_e4584: f64 = (assign2740_e4580 + assign2740_e4583);
        locals.var_pparam_b4soixdif = assign2740_e4584;
        locals.var_pparam_b4soixdif_dn3 = (((p.p470 * locals.var_inv_l_dn3) + (p.p660 * locals.var_inv_w_dn3)) + (p.p850 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soixdif_dn4 = (((p.p470 * locals.var_inv_l_dn4) + (p.p660 * locals.var_inv_w_dn4)) + (p.p850 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soixdif_dn5 = (((p.p470 * locals.var_inv_l_dn5) + (p.p660 * locals.var_inv_w_dn5)) + (p.p850 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soixdif_dn6 = (((p.p470 * locals.var_inv_l_dn6) + (p.p660 * locals.var_inv_w_dn6)) + (p.p850 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soixdif_dn7 = (((p.p470 * locals.var_inv_l_dn7) + (p.p660 * locals.var_inv_w_dn7)) + (p.p850 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soixdif_dn8 = (((p.p470 * locals.var_inv_l_dn8) + (p.p660 * locals.var_inv_w_dn8)) + (p.p850 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soixdif_dn9 = (((p.p470 * locals.var_inv_l_dn9) + (p.p660 * locals.var_inv_w_dn9)) + (p.p850 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soixdif_dn10 = (((p.p470 * locals.var_inv_l_dn10) + (p.p660 * locals.var_inv_w_dn10)) + (p.p850 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soixdif_dn11 = (((p.p470 * locals.var_inv_l_dn11) + (p.p660 * locals.var_inv_w_dn11)) + (p.p850 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soixdif_dn12 = (((p.p470 * locals.var_inv_l_dn12) + (p.p660 * locals.var_inv_w_dn12)) + (p.p850 * locals.var_inv_lw_dn12));

        let assign2750_e4588: f64 = (p.p471 * locals.var_inv_l);
        let assign2750_e4589: f64 = (p.p176 + assign2750_e4588);
        let assign2750_e4592: f64 = (p.p661 * locals.var_inv_w);
        let assign2750_e4593: f64 = (assign2750_e4589 + assign2750_e4592);
        let assign2750_e4596: f64 = (p.p851 * locals.var_inv_lw);
        let assign2750_e4597: f64 = (assign2750_e4593 + assign2750_e4596);
        locals.var_pparam_b4soixrec = assign2750_e4597;
        locals.var_pparam_b4soixrec_dn3 = (((p.p471 * locals.var_inv_l_dn3) + (p.p661 * locals.var_inv_w_dn3)) + (p.p851 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soixrec_dn4 = (((p.p471 * locals.var_inv_l_dn4) + (p.p661 * locals.var_inv_w_dn4)) + (p.p851 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soixrec_dn5 = (((p.p471 * locals.var_inv_l_dn5) + (p.p661 * locals.var_inv_w_dn5)) + (p.p851 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soixrec_dn6 = (((p.p471 * locals.var_inv_l_dn6) + (p.p661 * locals.var_inv_w_dn6)) + (p.p851 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soixrec_dn7 = (((p.p471 * locals.var_inv_l_dn7) + (p.p661 * locals.var_inv_w_dn7)) + (p.p851 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soixrec_dn8 = (((p.p471 * locals.var_inv_l_dn8) + (p.p661 * locals.var_inv_w_dn8)) + (p.p851 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soixrec_dn9 = (((p.p471 * locals.var_inv_l_dn9) + (p.p661 * locals.var_inv_w_dn9)) + (p.p851 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soixrec_dn10 = (((p.p471 * locals.var_inv_l_dn10) + (p.p661 * locals.var_inv_w_dn10)) + (p.p851 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soixrec_dn11 = (((p.p471 * locals.var_inv_l_dn11) + (p.p661 * locals.var_inv_w_dn11)) + (p.p851 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soixrec_dn12 = (((p.p471 * locals.var_inv_l_dn12) + (p.p661 * locals.var_inv_w_dn12)) + (p.p851 * locals.var_inv_lw_dn12));

        let assign2760_e4601: f64 = (p.p472 * locals.var_inv_l);
        let assign2760_e4602: f64 = (p.p177 + assign2760_e4601);
        let assign2760_e4605: f64 = (p.p662 * locals.var_inv_w);
        let assign2760_e4606: f64 = (assign2760_e4602 + assign2760_e4605);
        let assign2760_e4609: f64 = (p.p852 * locals.var_inv_lw);
        let assign2760_e4610: f64 = (assign2760_e4606 + assign2760_e4609);
        locals.var_pparam_b4soixtun = assign2760_e4610;
        locals.var_pparam_b4soixtun_dn3 = (((p.p472 * locals.var_inv_l_dn3) + (p.p662 * locals.var_inv_w_dn3)) + (p.p852 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soixtun_dn4 = (((p.p472 * locals.var_inv_l_dn4) + (p.p662 * locals.var_inv_w_dn4)) + (p.p852 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soixtun_dn5 = (((p.p472 * locals.var_inv_l_dn5) + (p.p662 * locals.var_inv_w_dn5)) + (p.p852 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soixtun_dn6 = (((p.p472 * locals.var_inv_l_dn6) + (p.p662 * locals.var_inv_w_dn6)) + (p.p852 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soixtun_dn7 = (((p.p472 * locals.var_inv_l_dn7) + (p.p662 * locals.var_inv_w_dn7)) + (p.p852 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soixtun_dn8 = (((p.p472 * locals.var_inv_l_dn8) + (p.p662 * locals.var_inv_w_dn8)) + (p.p852 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soixtun_dn9 = (((p.p472 * locals.var_inv_l_dn9) + (p.p662 * locals.var_inv_w_dn9)) + (p.p852 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soixtun_dn10 = (((p.p472 * locals.var_inv_l_dn10) + (p.p662 * locals.var_inv_w_dn10)) + (p.p852 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soixtun_dn11 = (((p.p472 * locals.var_inv_l_dn11) + (p.p662 * locals.var_inv_w_dn11)) + (p.p852 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soixtun_dn12 = (((p.p472 * locals.var_inv_l_dn12) + (p.p662 * locals.var_inv_w_dn12)) + (p.p852 * locals.var_inv_lw_dn12));

        let assign2770_e4614: f64 = (p.p473 * locals.var_inv_l);
        let assign2770_e4615: f64 = (p.p178 + assign2770_e4614);
        let assign2770_e4618: f64 = (p.p663 * locals.var_inv_w);
        let assign2770_e4619: f64 = (assign2770_e4615 + assign2770_e4618);
        let assign2770_e4622: f64 = (p.p853 * locals.var_inv_lw);
        let assign2770_e4623: f64 = (assign2770_e4619 + assign2770_e4622);
        locals.var_pparam_b4soixdifd = assign2770_e4623;
        locals.var_pparam_b4soixdifd_dn3 = (((p.p473 * locals.var_inv_l_dn3) + (p.p663 * locals.var_inv_w_dn3)) + (p.p853 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soixdifd_dn4 = (((p.p473 * locals.var_inv_l_dn4) + (p.p663 * locals.var_inv_w_dn4)) + (p.p853 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soixdifd_dn5 = (((p.p473 * locals.var_inv_l_dn5) + (p.p663 * locals.var_inv_w_dn5)) + (p.p853 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soixdifd_dn6 = (((p.p473 * locals.var_inv_l_dn6) + (p.p663 * locals.var_inv_w_dn6)) + (p.p853 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soixdifd_dn7 = (((p.p473 * locals.var_inv_l_dn7) + (p.p663 * locals.var_inv_w_dn7)) + (p.p853 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soixdifd_dn8 = (((p.p473 * locals.var_inv_l_dn8) + (p.p663 * locals.var_inv_w_dn8)) + (p.p853 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soixdifd_dn9 = (((p.p473 * locals.var_inv_l_dn9) + (p.p663 * locals.var_inv_w_dn9)) + (p.p853 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soixdifd_dn10 = (((p.p473 * locals.var_inv_l_dn10) + (p.p663 * locals.var_inv_w_dn10)) + (p.p853 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soixdifd_dn11 = (((p.p473 * locals.var_inv_l_dn11) + (p.p663 * locals.var_inv_w_dn11)) + (p.p853 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soixdifd_dn12 = (((p.p473 * locals.var_inv_l_dn12) + (p.p663 * locals.var_inv_w_dn12)) + (p.p853 * locals.var_inv_lw_dn12));

        let assign2780_e4627: f64 = (p.p474 * locals.var_inv_l);
        let assign2780_e4628: f64 = (p.p179 + assign2780_e4627);
        let assign2780_e4631: f64 = (p.p664 * locals.var_inv_w);
        let assign2780_e4632: f64 = (assign2780_e4628 + assign2780_e4631);
        let assign2780_e4635: f64 = (p.p854 * locals.var_inv_lw);
        let assign2780_e4636: f64 = (assign2780_e4632 + assign2780_e4635);
        locals.var_pparam_b4soixrecd = assign2780_e4636;
        locals.var_pparam_b4soixrecd_dn3 = (((p.p474 * locals.var_inv_l_dn3) + (p.p664 * locals.var_inv_w_dn3)) + (p.p854 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soixrecd_dn4 = (((p.p474 * locals.var_inv_l_dn4) + (p.p664 * locals.var_inv_w_dn4)) + (p.p854 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soixrecd_dn5 = (((p.p474 * locals.var_inv_l_dn5) + (p.p664 * locals.var_inv_w_dn5)) + (p.p854 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soixrecd_dn6 = (((p.p474 * locals.var_inv_l_dn6) + (p.p664 * locals.var_inv_w_dn6)) + (p.p854 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soixrecd_dn7 = (((p.p474 * locals.var_inv_l_dn7) + (p.p664 * locals.var_inv_w_dn7)) + (p.p854 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soixrecd_dn8 = (((p.p474 * locals.var_inv_l_dn8) + (p.p664 * locals.var_inv_w_dn8)) + (p.p854 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soixrecd_dn9 = (((p.p474 * locals.var_inv_l_dn9) + (p.p664 * locals.var_inv_w_dn9)) + (p.p854 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soixrecd_dn10 = (((p.p474 * locals.var_inv_l_dn10) + (p.p664 * locals.var_inv_w_dn10)) + (p.p854 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soixrecd_dn11 = (((p.p474 * locals.var_inv_l_dn11) + (p.p664 * locals.var_inv_w_dn11)) + (p.p854 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soixrecd_dn12 = (((p.p474 * locals.var_inv_l_dn12) + (p.p664 * locals.var_inv_w_dn12)) + (p.p854 * locals.var_inv_lw_dn12));

        let assign2790_e4640: f64 = (p.p475 * locals.var_inv_l);
        let assign2790_e4641: f64 = (p.p180 + assign2790_e4640);
        let assign2790_e4644: f64 = (p.p665 * locals.var_inv_w);
        let assign2790_e4645: f64 = (assign2790_e4641 + assign2790_e4644);
        let assign2790_e4648: f64 = (p.p855 * locals.var_inv_lw);
        let assign2790_e4649: f64 = (assign2790_e4645 + assign2790_e4648);
        locals.var_pparam_b4soixtund = assign2790_e4649;
        locals.var_pparam_b4soixtund_dn3 = (((p.p475 * locals.var_inv_l_dn3) + (p.p665 * locals.var_inv_w_dn3)) + (p.p855 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soixtund_dn4 = (((p.p475 * locals.var_inv_l_dn4) + (p.p665 * locals.var_inv_w_dn4)) + (p.p855 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soixtund_dn5 = (((p.p475 * locals.var_inv_l_dn5) + (p.p665 * locals.var_inv_w_dn5)) + (p.p855 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soixtund_dn6 = (((p.p475 * locals.var_inv_l_dn6) + (p.p665 * locals.var_inv_w_dn6)) + (p.p855 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soixtund_dn7 = (((p.p475 * locals.var_inv_l_dn7) + (p.p665 * locals.var_inv_w_dn7)) + (p.p855 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soixtund_dn8 = (((p.p475 * locals.var_inv_l_dn8) + (p.p665 * locals.var_inv_w_dn8)) + (p.p855 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soixtund_dn9 = (((p.p475 * locals.var_inv_l_dn9) + (p.p665 * locals.var_inv_w_dn9)) + (p.p855 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soixtund_dn10 = (((p.p475 * locals.var_inv_l_dn10) + (p.p665 * locals.var_inv_w_dn10)) + (p.p855 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soixtund_dn11 = (((p.p475 * locals.var_inv_l_dn11) + (p.p665 * locals.var_inv_w_dn11)) + (p.p855 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soixtund_dn12 = (((p.p475 * locals.var_inv_l_dn12) + (p.p665 * locals.var_inv_w_dn12)) + (p.p855 * locals.var_inv_lw_dn12));

        let assign2800_e4653: f64 = (p.p455 * locals.var_inv_l);
        let assign2800_e4654: f64 = (p.p211 + assign2800_e4653);
        let assign2800_e4657: f64 = (p.p645 * locals.var_inv_w);
        let assign2800_e4658: f64 = (assign2800_e4654 + assign2800_e4657);
        let assign2800_e4661: f64 = (p.p835 * locals.var_inv_lw);
        let assign2800_e4662: f64 = (assign2800_e4658 + assign2800_e4661);
        locals.var_pparam_b4soicgdl = assign2800_e4662;
        locals.var_pparam_b4soicgdl_dn3 = (((p.p455 * locals.var_inv_l_dn3) + (p.p645 * locals.var_inv_w_dn3)) + (p.p835 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soicgdl_dn4 = (((p.p455 * locals.var_inv_l_dn4) + (p.p645 * locals.var_inv_w_dn4)) + (p.p835 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soicgdl_dn5 = (((p.p455 * locals.var_inv_l_dn5) + (p.p645 * locals.var_inv_w_dn5)) + (p.p835 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soicgdl_dn6 = (((p.p455 * locals.var_inv_l_dn6) + (p.p645 * locals.var_inv_w_dn6)) + (p.p835 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soicgdl_dn7 = (((p.p455 * locals.var_inv_l_dn7) + (p.p645 * locals.var_inv_w_dn7)) + (p.p835 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soicgdl_dn8 = (((p.p455 * locals.var_inv_l_dn8) + (p.p645 * locals.var_inv_w_dn8)) + (p.p835 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soicgdl_dn9 = (((p.p455 * locals.var_inv_l_dn9) + (p.p645 * locals.var_inv_w_dn9)) + (p.p835 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soicgdl_dn10 = (((p.p455 * locals.var_inv_l_dn10) + (p.p645 * locals.var_inv_w_dn10)) + (p.p835 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soicgdl_dn11 = (((p.p455 * locals.var_inv_l_dn11) + (p.p645 * locals.var_inv_w_dn11)) + (p.p835 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soicgdl_dn12 = (((p.p455 * locals.var_inv_l_dn12) + (p.p645 * locals.var_inv_w_dn12)) + (p.p835 * locals.var_inv_lw_dn12));

        let assign2810_e4666: f64 = (p.p454 * locals.var_inv_l);
        let assign2810_e4667: f64 = (p.p210 + assign2810_e4666);
        let assign2810_e4670: f64 = (p.p644 * locals.var_inv_w);
        let assign2810_e4671: f64 = (assign2810_e4667 + assign2810_e4670);
        let assign2810_e4674: f64 = (p.p834 * locals.var_inv_lw);
        let assign2810_e4675: f64 = (assign2810_e4671 + assign2810_e4674);
        locals.var_pparam_b4soicgsl = assign2810_e4675;
        locals.var_pparam_b4soicgsl_dn3 = (((p.p454 * locals.var_inv_l_dn3) + (p.p644 * locals.var_inv_w_dn3)) + (p.p834 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soicgsl_dn4 = (((p.p454 * locals.var_inv_l_dn4) + (p.p644 * locals.var_inv_w_dn4)) + (p.p834 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soicgsl_dn5 = (((p.p454 * locals.var_inv_l_dn5) + (p.p644 * locals.var_inv_w_dn5)) + (p.p834 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soicgsl_dn6 = (((p.p454 * locals.var_inv_l_dn6) + (p.p644 * locals.var_inv_w_dn6)) + (p.p834 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soicgsl_dn7 = (((p.p454 * locals.var_inv_l_dn7) + (p.p644 * locals.var_inv_w_dn7)) + (p.p834 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soicgsl_dn8 = (((p.p454 * locals.var_inv_l_dn8) + (p.p644 * locals.var_inv_w_dn8)) + (p.p834 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soicgsl_dn9 = (((p.p454 * locals.var_inv_l_dn9) + (p.p644 * locals.var_inv_w_dn9)) + (p.p834 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soicgsl_dn10 = (((p.p454 * locals.var_inv_l_dn10) + (p.p644 * locals.var_inv_w_dn10)) + (p.p834 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soicgsl_dn11 = (((p.p454 * locals.var_inv_l_dn11) + (p.p644 * locals.var_inv_w_dn11)) + (p.p834 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soicgsl_dn12 = (((p.p454 * locals.var_inv_l_dn12) + (p.p644 * locals.var_inv_w_dn12)) + (p.p834 * locals.var_inv_lw_dn12));

        let assign2820_e4679: f64 = (p.p456 * locals.var_inv_l);
        let assign2820_e4680: f64 = (p.p212 + assign2820_e4679);
        let assign2820_e4683: f64 = (p.p646 * locals.var_inv_w);
        let assign2820_e4684: f64 = (assign2820_e4680 + assign2820_e4683);
        let assign2820_e4687: f64 = (p.p836 * locals.var_inv_lw);
        let assign2820_e4688: f64 = (assign2820_e4684 + assign2820_e4687);
        locals.var_pparam_b4soickappa = assign2820_e4688;
        locals.var_pparam_b4soickappa_dn3 = (((p.p456 * locals.var_inv_l_dn3) + (p.p646 * locals.var_inv_w_dn3)) + (p.p836 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soickappa_dn4 = (((p.p456 * locals.var_inv_l_dn4) + (p.p646 * locals.var_inv_w_dn4)) + (p.p836 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soickappa_dn5 = (((p.p456 * locals.var_inv_l_dn5) + (p.p646 * locals.var_inv_w_dn5)) + (p.p836 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soickappa_dn6 = (((p.p456 * locals.var_inv_l_dn6) + (p.p646 * locals.var_inv_w_dn6)) + (p.p836 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soickappa_dn7 = (((p.p456 * locals.var_inv_l_dn7) + (p.p646 * locals.var_inv_w_dn7)) + (p.p836 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soickappa_dn8 = (((p.p456 * locals.var_inv_l_dn8) + (p.p646 * locals.var_inv_w_dn8)) + (p.p836 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soickappa_dn9 = (((p.p456 * locals.var_inv_l_dn9) + (p.p646 * locals.var_inv_w_dn9)) + (p.p836 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soickappa_dn10 = (((p.p456 * locals.var_inv_l_dn10) + (p.p646 * locals.var_inv_w_dn10)) + (p.p836 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soickappa_dn11 = (((p.p456 * locals.var_inv_l_dn11) + (p.p646 * locals.var_inv_w_dn11)) + (p.p836 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soickappa_dn12 = (((p.p456 * locals.var_inv_l_dn12) + (p.p646 * locals.var_inv_w_dn12)) + (p.p836 * locals.var_inv_lw_dn12));

        let assign2830_e4692: f64 = (p.p458 * locals.var_inv_l);
        let assign2830_e4693: f64 = (p.p118 + assign2830_e4692);
        let assign2830_e4696: f64 = (p.p648 * locals.var_inv_w);
        let assign2830_e4697: f64 = (assign2830_e4693 + assign2830_e4696);
        let assign2830_e4700: f64 = (p.p838 * locals.var_inv_lw);
        let assign2830_e4701: f64 = (assign2830_e4697 + assign2830_e4700);
        locals.var_pparam_b4soiute = assign2830_e4701;
        locals.var_pparam_b4soiute_dn3 = (((p.p458 * locals.var_inv_l_dn3) + (p.p648 * locals.var_inv_w_dn3)) + (p.p838 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiute_dn4 = (((p.p458 * locals.var_inv_l_dn4) + (p.p648 * locals.var_inv_w_dn4)) + (p.p838 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiute_dn5 = (((p.p458 * locals.var_inv_l_dn5) + (p.p648 * locals.var_inv_w_dn5)) + (p.p838 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiute_dn6 = (((p.p458 * locals.var_inv_l_dn6) + (p.p648 * locals.var_inv_w_dn6)) + (p.p838 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiute_dn7 = (((p.p458 * locals.var_inv_l_dn7) + (p.p648 * locals.var_inv_w_dn7)) + (p.p838 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiute_dn8 = (((p.p458 * locals.var_inv_l_dn8) + (p.p648 * locals.var_inv_w_dn8)) + (p.p838 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiute_dn9 = (((p.p458 * locals.var_inv_l_dn9) + (p.p648 * locals.var_inv_w_dn9)) + (p.p838 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiute_dn10 = (((p.p458 * locals.var_inv_l_dn10) + (p.p648 * locals.var_inv_w_dn10)) + (p.p838 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiute_dn11 = (((p.p458 * locals.var_inv_l_dn11) + (p.p648 * locals.var_inv_w_dn11)) + (p.p838 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiute_dn12 = (((p.p458 * locals.var_inv_l_dn12) + (p.p648 * locals.var_inv_w_dn12)) + (p.p838 * locals.var_inv_lw_dn12));

        let assign2840_e4705: f64 = (p.p514 * locals.var_inv_l);
        let assign2840_e4706: f64 = (p.p121 + assign2840_e4705);
        let assign2840_e4709: f64 = (p.p704 * locals.var_inv_w);
        let assign2840_e4710: f64 = (assign2840_e4706 + assign2840_e4709);
        let assign2840_e4713: f64 = (p.p894 * locals.var_inv_lw);
        let assign2840_e4714: f64 = (assign2840_e4710 + assign2840_e4713);
        locals.var_pparam_b4soiud = assign2840_e4714;
        locals.var_pparam_b4soiud_dn3 = (((p.p514 * locals.var_inv_l_dn3) + (p.p704 * locals.var_inv_w_dn3)) + (p.p894 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiud_dn4 = (((p.p514 * locals.var_inv_l_dn4) + (p.p704 * locals.var_inv_w_dn4)) + (p.p894 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiud_dn5 = (((p.p514 * locals.var_inv_l_dn5) + (p.p704 * locals.var_inv_w_dn5)) + (p.p894 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiud_dn6 = (((p.p514 * locals.var_inv_l_dn6) + (p.p704 * locals.var_inv_w_dn6)) + (p.p894 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiud_dn7 = (((p.p514 * locals.var_inv_l_dn7) + (p.p704 * locals.var_inv_w_dn7)) + (p.p894 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiud_dn8 = (((p.p514 * locals.var_inv_l_dn8) + (p.p704 * locals.var_inv_w_dn8)) + (p.p894 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiud_dn9 = (((p.p514 * locals.var_inv_l_dn9) + (p.p704 * locals.var_inv_w_dn9)) + (p.p894 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiud_dn10 = (((p.p514 * locals.var_inv_l_dn10) + (p.p704 * locals.var_inv_w_dn10)) + (p.p894 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiud_dn11 = (((p.p514 * locals.var_inv_l_dn11) + (p.p704 * locals.var_inv_w_dn11)) + (p.p894 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiud_dn12 = (((p.p514 * locals.var_inv_l_dn12) + (p.p704 * locals.var_inv_w_dn12)) + (p.p894 * locals.var_inv_lw_dn12));

        let assign2850_e4718: f64 = (p.p515 * locals.var_inv_l);
        let assign2850_e4719: f64 = (p.p122 + assign2850_e4718);
        let assign2850_e4722: f64 = (p.p705 * locals.var_inv_w);
        let assign2850_e4723: f64 = (assign2850_e4719 + assign2850_e4722);
        let assign2850_e4726: f64 = (p.p895 * locals.var_inv_lw);
        let assign2850_e4727: f64 = (assign2850_e4723 + assign2850_e4726);
        locals.var_pparam_b4soiud1 = assign2850_e4727;
        locals.var_pparam_b4soiud1_dn3 = (((p.p515 * locals.var_inv_l_dn3) + (p.p705 * locals.var_inv_w_dn3)) + (p.p895 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiud1_dn4 = (((p.p515 * locals.var_inv_l_dn4) + (p.p705 * locals.var_inv_w_dn4)) + (p.p895 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiud1_dn5 = (((p.p515 * locals.var_inv_l_dn5) + (p.p705 * locals.var_inv_w_dn5)) + (p.p895 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiud1_dn6 = (((p.p515 * locals.var_inv_l_dn6) + (p.p705 * locals.var_inv_w_dn6)) + (p.p895 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiud1_dn7 = (((p.p515 * locals.var_inv_l_dn7) + (p.p705 * locals.var_inv_w_dn7)) + (p.p895 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiud1_dn8 = (((p.p515 * locals.var_inv_l_dn8) + (p.p705 * locals.var_inv_w_dn8)) + (p.p895 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiud1_dn9 = (((p.p515 * locals.var_inv_l_dn9) + (p.p705 * locals.var_inv_w_dn9)) + (p.p895 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiud1_dn10 = (((p.p515 * locals.var_inv_l_dn10) + (p.p705 * locals.var_inv_w_dn10)) + (p.p895 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiud1_dn11 = (((p.p515 * locals.var_inv_l_dn11) + (p.p705 * locals.var_inv_w_dn11)) + (p.p895 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiud1_dn12 = (((p.p515 * locals.var_inv_l_dn12) + (p.p705 * locals.var_inv_w_dn12)) + (p.p895 * locals.var_inv_lw_dn12));

        let assign2860_e4731: f64 = (p.p510 * locals.var_inv_l);
        let assign2860_e4732: f64 = (p.p117 + assign2860_e4731);
        let assign2860_e4735: f64 = (p.p700 * locals.var_inv_w);
        let assign2860_e4736: f64 = (assign2860_e4732 + assign2860_e4735);
        let assign2860_e4739: f64 = (p.p890 * locals.var_inv_lw);
        let assign2860_e4740: f64 = (assign2860_e4736 + assign2860_e4739);
        locals.var_pparam_b4soieu = assign2860_e4740;
        locals.var_pparam_b4soieu_dn3 = (((p.p510 * locals.var_inv_l_dn3) + (p.p700 * locals.var_inv_w_dn3)) + (p.p890 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soieu_dn4 = (((p.p510 * locals.var_inv_l_dn4) + (p.p700 * locals.var_inv_w_dn4)) + (p.p890 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soieu_dn5 = (((p.p510 * locals.var_inv_l_dn5) + (p.p700 * locals.var_inv_w_dn5)) + (p.p890 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soieu_dn6 = (((p.p510 * locals.var_inv_l_dn6) + (p.p700 * locals.var_inv_w_dn6)) + (p.p890 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soieu_dn7 = (((p.p510 * locals.var_inv_l_dn7) + (p.p700 * locals.var_inv_w_dn7)) + (p.p890 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soieu_dn8 = (((p.p510 * locals.var_inv_l_dn8) + (p.p700 * locals.var_inv_w_dn8)) + (p.p890 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soieu_dn9 = (((p.p510 * locals.var_inv_l_dn9) + (p.p700 * locals.var_inv_w_dn9)) + (p.p890 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soieu_dn10 = (((p.p510 * locals.var_inv_l_dn10) + (p.p700 * locals.var_inv_w_dn10)) + (p.p890 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soieu_dn11 = (((p.p510 * locals.var_inv_l_dn11) + (p.p700 * locals.var_inv_w_dn11)) + (p.p890 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soieu_dn12 = (((p.p510 * locals.var_inv_l_dn12) + (p.p700 * locals.var_inv_w_dn12)) + (p.p890 * locals.var_inv_lw_dn12));

        let assign2870_e4744: f64 = (p.p517 * locals.var_inv_l);
        let assign2870_e4745: f64 = (p.p119 + assign2870_e4744);
        let assign2870_e4748: f64 = (p.p707 * locals.var_inv_w);
        let assign2870_e4749: f64 = (assign2870_e4745 + assign2870_e4748);
        let assign2870_e4752: f64 = (p.p897 * locals.var_inv_lw);
        let assign2870_e4753: f64 = (assign2870_e4749 + assign2870_e4752);
        locals.var_pparam_b4soiucs = assign2870_e4753;
        locals.var_pparam_b4soiucs_dn3 = (((p.p517 * locals.var_inv_l_dn3) + (p.p707 * locals.var_inv_w_dn3)) + (p.p897 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiucs_dn4 = (((p.p517 * locals.var_inv_l_dn4) + (p.p707 * locals.var_inv_w_dn4)) + (p.p897 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiucs_dn5 = (((p.p517 * locals.var_inv_l_dn5) + (p.p707 * locals.var_inv_w_dn5)) + (p.p897 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiucs_dn6 = (((p.p517 * locals.var_inv_l_dn6) + (p.p707 * locals.var_inv_w_dn6)) + (p.p897 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiucs_dn7 = (((p.p517 * locals.var_inv_l_dn7) + (p.p707 * locals.var_inv_w_dn7)) + (p.p897 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiucs_dn8 = (((p.p517 * locals.var_inv_l_dn8) + (p.p707 * locals.var_inv_w_dn8)) + (p.p897 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiucs_dn9 = (((p.p517 * locals.var_inv_l_dn9) + (p.p707 * locals.var_inv_w_dn9)) + (p.p897 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiucs_dn10 = (((p.p517 * locals.var_inv_l_dn10) + (p.p707 * locals.var_inv_w_dn10)) + (p.p897 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiucs_dn11 = (((p.p517 * locals.var_inv_l_dn11) + (p.p707 * locals.var_inv_w_dn11)) + (p.p897 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiucs_dn12 = (((p.p517 * locals.var_inv_l_dn12) + (p.p707 * locals.var_inv_w_dn12)) + (p.p897 * locals.var_inv_lw_dn12));

        let assign2880_e4757: f64 = (p.p516 * locals.var_inv_l);
        let assign2880_e4758: f64 = (p.p120 + assign2880_e4757);
        let assign2880_e4761: f64 = (p.p706 * locals.var_inv_w);
        let assign2880_e4762: f64 = (assign2880_e4758 + assign2880_e4761);
        let assign2880_e4765: f64 = (p.p896 * locals.var_inv_lw);
        let assign2880_e4766: f64 = (assign2880_e4762 + assign2880_e4765);
        locals.var_pparam_b4soiucste = assign2880_e4766;
        locals.var_pparam_b4soiucste_dn3 = (((p.p516 * locals.var_inv_l_dn3) + (p.p706 * locals.var_inv_w_dn3)) + (p.p896 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiucste_dn4 = (((p.p516 * locals.var_inv_l_dn4) + (p.p706 * locals.var_inv_w_dn4)) + (p.p896 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiucste_dn5 = (((p.p516 * locals.var_inv_l_dn5) + (p.p706 * locals.var_inv_w_dn5)) + (p.p896 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiucste_dn6 = (((p.p516 * locals.var_inv_l_dn6) + (p.p706 * locals.var_inv_w_dn6)) + (p.p896 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiucste_dn7 = (((p.p516 * locals.var_inv_l_dn7) + (p.p706 * locals.var_inv_w_dn7)) + (p.p896 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiucste_dn8 = (((p.p516 * locals.var_inv_l_dn8) + (p.p706 * locals.var_inv_w_dn8)) + (p.p896 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiucste_dn9 = (((p.p516 * locals.var_inv_l_dn9) + (p.p706 * locals.var_inv_w_dn9)) + (p.p896 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiucste_dn10 = (((p.p516 * locals.var_inv_l_dn10) + (p.p706 * locals.var_inv_w_dn10)) + (p.p896 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiucste_dn11 = (((p.p516 * locals.var_inv_l_dn11) + (p.p706 * locals.var_inv_w_dn11)) + (p.p896 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiucste_dn12 = (((p.p516 * locals.var_inv_l_dn12) + (p.p706 * locals.var_inv_w_dn12)) + (p.p896 * locals.var_inv_lw_dn12));

        let assign2890_e4770: f64 = (p.p459 * locals.var_inv_l);
        let assign2890_e4771: f64 = (p.p91 + assign2890_e4770);
        let assign2890_e4774: f64 = (p.p649 * locals.var_inv_w);
        let assign2890_e4775: f64 = (assign2890_e4771 + assign2890_e4774);
        let assign2890_e4778: f64 = (p.p839 * locals.var_inv_lw);
        let assign2890_e4779: f64 = (assign2890_e4775 + assign2890_e4778);
        locals.var_pparam_b4soikt1 = assign2890_e4779;
        locals.var_pparam_b4soikt1_dn3 = (((p.p459 * locals.var_inv_l_dn3) + (p.p649 * locals.var_inv_w_dn3)) + (p.p839 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soikt1_dn4 = (((p.p459 * locals.var_inv_l_dn4) + (p.p649 * locals.var_inv_w_dn4)) + (p.p839 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soikt1_dn5 = (((p.p459 * locals.var_inv_l_dn5) + (p.p649 * locals.var_inv_w_dn5)) + (p.p839 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soikt1_dn6 = (((p.p459 * locals.var_inv_l_dn6) + (p.p649 * locals.var_inv_w_dn6)) + (p.p839 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soikt1_dn7 = (((p.p459 * locals.var_inv_l_dn7) + (p.p649 * locals.var_inv_w_dn7)) + (p.p839 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soikt1_dn8 = (((p.p459 * locals.var_inv_l_dn8) + (p.p649 * locals.var_inv_w_dn8)) + (p.p839 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soikt1_dn9 = (((p.p459 * locals.var_inv_l_dn9) + (p.p649 * locals.var_inv_w_dn9)) + (p.p839 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soikt1_dn10 = (((p.p459 * locals.var_inv_l_dn10) + (p.p649 * locals.var_inv_w_dn10)) + (p.p839 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soikt1_dn11 = (((p.p459 * locals.var_inv_l_dn11) + (p.p649 * locals.var_inv_w_dn11)) + (p.p839 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soikt1_dn12 = (((p.p459 * locals.var_inv_l_dn12) + (p.p649 * locals.var_inv_w_dn12)) + (p.p839 * locals.var_inv_lw_dn12));

        let assign2900_e4783: f64 = (p.p461 * locals.var_inv_l);
        let assign2900_e4784: f64 = (p.p93 + assign2900_e4783);
        let assign2900_e4787: f64 = (p.p651 * locals.var_inv_w);
        let assign2900_e4788: f64 = (assign2900_e4784 + assign2900_e4787);
        let assign2900_e4791: f64 = (p.p841 * locals.var_inv_lw);
        let assign2900_e4792: f64 = (assign2900_e4788 + assign2900_e4791);
        locals.var_pparam_b4soikt2 = assign2900_e4792;
        locals.var_pparam_b4soikt2_dn3 = (((p.p461 * locals.var_inv_l_dn3) + (p.p651 * locals.var_inv_w_dn3)) + (p.p841 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soikt2_dn4 = (((p.p461 * locals.var_inv_l_dn4) + (p.p651 * locals.var_inv_w_dn4)) + (p.p841 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soikt2_dn5 = (((p.p461 * locals.var_inv_l_dn5) + (p.p651 * locals.var_inv_w_dn5)) + (p.p841 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soikt2_dn6 = (((p.p461 * locals.var_inv_l_dn6) + (p.p651 * locals.var_inv_w_dn6)) + (p.p841 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soikt2_dn7 = (((p.p461 * locals.var_inv_l_dn7) + (p.p651 * locals.var_inv_w_dn7)) + (p.p841 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soikt2_dn8 = (((p.p461 * locals.var_inv_l_dn8) + (p.p651 * locals.var_inv_w_dn8)) + (p.p841 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soikt2_dn9 = (((p.p461 * locals.var_inv_l_dn9) + (p.p651 * locals.var_inv_w_dn9)) + (p.p841 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soikt2_dn10 = (((p.p461 * locals.var_inv_l_dn10) + (p.p651 * locals.var_inv_w_dn10)) + (p.p841 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soikt2_dn11 = (((p.p461 * locals.var_inv_l_dn11) + (p.p651 * locals.var_inv_w_dn11)) + (p.p841 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soikt2_dn12 = (((p.p461 * locals.var_inv_l_dn12) + (p.p651 * locals.var_inv_w_dn12)) + (p.p841 * locals.var_inv_lw_dn12));

        let assign2910_e4796: f64 = (p.p460 * locals.var_inv_l);
        let assign2910_e4797: f64 = (p.p92 + assign2910_e4796);
        let assign2910_e4800: f64 = (p.p650 * locals.var_inv_w);
        let assign2910_e4801: f64 = (assign2910_e4797 + assign2910_e4800);
        let assign2910_e4804: f64 = (p.p840 * locals.var_inv_lw);
        let assign2910_e4805: f64 = (assign2910_e4801 + assign2910_e4804);
        locals.var_pparam_b4soikt1l = assign2910_e4805;
        locals.var_pparam_b4soikt1l_dn3 = (((p.p460 * locals.var_inv_l_dn3) + (p.p650 * locals.var_inv_w_dn3)) + (p.p840 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soikt1l_dn4 = (((p.p460 * locals.var_inv_l_dn4) + (p.p650 * locals.var_inv_w_dn4)) + (p.p840 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soikt1l_dn5 = (((p.p460 * locals.var_inv_l_dn5) + (p.p650 * locals.var_inv_w_dn5)) + (p.p840 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soikt1l_dn6 = (((p.p460 * locals.var_inv_l_dn6) + (p.p650 * locals.var_inv_w_dn6)) + (p.p840 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soikt1l_dn7 = (((p.p460 * locals.var_inv_l_dn7) + (p.p650 * locals.var_inv_w_dn7)) + (p.p840 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soikt1l_dn8 = (((p.p460 * locals.var_inv_l_dn8) + (p.p650 * locals.var_inv_w_dn8)) + (p.p840 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soikt1l_dn9 = (((p.p460 * locals.var_inv_l_dn9) + (p.p650 * locals.var_inv_w_dn9)) + (p.p840 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soikt1l_dn10 = (((p.p460 * locals.var_inv_l_dn10) + (p.p650 * locals.var_inv_w_dn10)) + (p.p840 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soikt1l_dn11 = (((p.p460 * locals.var_inv_l_dn11) + (p.p650 * locals.var_inv_w_dn11)) + (p.p840 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soikt1l_dn12 = (((p.p460 * locals.var_inv_l_dn12) + (p.p650 * locals.var_inv_w_dn12)) + (p.p840 * locals.var_inv_lw_dn12));

        let assign2920_e4809: f64 = (p.p462 * locals.var_inv_l);
        let assign2920_e4810: f64 = (p.p111 + assign2920_e4809);
        let assign2920_e4813: f64 = (p.p652 * locals.var_inv_w);
        let assign2920_e4814: f64 = (assign2920_e4810 + assign2920_e4813);
        let assign2920_e4817: f64 = (p.p842 * locals.var_inv_lw);
        let assign2920_e4818: f64 = (assign2920_e4814 + assign2920_e4817);
        locals.var_pparam_b4soiua1 = assign2920_e4818;
        locals.var_pparam_b4soiua1_dn3 = (((p.p462 * locals.var_inv_l_dn3) + (p.p652 * locals.var_inv_w_dn3)) + (p.p842 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiua1_dn4 = (((p.p462 * locals.var_inv_l_dn4) + (p.p652 * locals.var_inv_w_dn4)) + (p.p842 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiua1_dn5 = (((p.p462 * locals.var_inv_l_dn5) + (p.p652 * locals.var_inv_w_dn5)) + (p.p842 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiua1_dn6 = (((p.p462 * locals.var_inv_l_dn6) + (p.p652 * locals.var_inv_w_dn6)) + (p.p842 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiua1_dn7 = (((p.p462 * locals.var_inv_l_dn7) + (p.p652 * locals.var_inv_w_dn7)) + (p.p842 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiua1_dn8 = (((p.p462 * locals.var_inv_l_dn8) + (p.p652 * locals.var_inv_w_dn8)) + (p.p842 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiua1_dn9 = (((p.p462 * locals.var_inv_l_dn9) + (p.p652 * locals.var_inv_w_dn9)) + (p.p842 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiua1_dn10 = (((p.p462 * locals.var_inv_l_dn10) + (p.p652 * locals.var_inv_w_dn10)) + (p.p842 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiua1_dn11 = (((p.p462 * locals.var_inv_l_dn11) + (p.p652 * locals.var_inv_w_dn11)) + (p.p842 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiua1_dn12 = (((p.p462 * locals.var_inv_l_dn12) + (p.p652 * locals.var_inv_w_dn12)) + (p.p842 * locals.var_inv_lw_dn12));

        let assign2930_e4822: f64 = (p.p463 * locals.var_inv_l);
        let assign2930_e4823: f64 = (p.p113 + assign2930_e4822);
        let assign2930_e4826: f64 = (p.p653 * locals.var_inv_w);
        let assign2930_e4827: f64 = (assign2930_e4823 + assign2930_e4826);
        let assign2930_e4830: f64 = (p.p843 * locals.var_inv_lw);
        let assign2930_e4831: f64 = (assign2930_e4827 + assign2930_e4830);
        locals.var_pparam_b4soiub1 = assign2930_e4831;
        locals.var_pparam_b4soiub1_dn3 = (((p.p463 * locals.var_inv_l_dn3) + (p.p653 * locals.var_inv_w_dn3)) + (p.p843 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiub1_dn4 = (((p.p463 * locals.var_inv_l_dn4) + (p.p653 * locals.var_inv_w_dn4)) + (p.p843 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiub1_dn5 = (((p.p463 * locals.var_inv_l_dn5) + (p.p653 * locals.var_inv_w_dn5)) + (p.p843 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiub1_dn6 = (((p.p463 * locals.var_inv_l_dn6) + (p.p653 * locals.var_inv_w_dn6)) + (p.p843 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiub1_dn7 = (((p.p463 * locals.var_inv_l_dn7) + (p.p653 * locals.var_inv_w_dn7)) + (p.p843 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiub1_dn8 = (((p.p463 * locals.var_inv_l_dn8) + (p.p653 * locals.var_inv_w_dn8)) + (p.p843 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiub1_dn9 = (((p.p463 * locals.var_inv_l_dn9) + (p.p653 * locals.var_inv_w_dn9)) + (p.p843 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiub1_dn10 = (((p.p463 * locals.var_inv_l_dn10) + (p.p653 * locals.var_inv_w_dn10)) + (p.p843 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiub1_dn11 = (((p.p463 * locals.var_inv_l_dn11) + (p.p653 * locals.var_inv_w_dn11)) + (p.p843 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiub1_dn12 = (((p.p463 * locals.var_inv_l_dn12) + (p.p653 * locals.var_inv_w_dn12)) + (p.p843 * locals.var_inv_lw_dn12));

        let assign2940_e4835: f64 = (p.p464 * locals.var_inv_l);
        let assign2940_e4836: f64 = (p.p115 + assign2940_e4835);
        let assign2940_e4839: f64 = (p.p654 * locals.var_inv_w);
        let assign2940_e4840: f64 = (assign2940_e4836 + assign2940_e4839);
        let assign2940_e4843: f64 = (p.p844 * locals.var_inv_lw);
        let assign2940_e4844: f64 = (assign2940_e4840 + assign2940_e4843);
        locals.var_pparam_b4soiuc1 = assign2940_e4844;
        locals.var_pparam_b4soiuc1_dn3 = (((p.p464 * locals.var_inv_l_dn3) + (p.p654 * locals.var_inv_w_dn3)) + (p.p844 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiuc1_dn4 = (((p.p464 * locals.var_inv_l_dn4) + (p.p654 * locals.var_inv_w_dn4)) + (p.p844 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiuc1_dn5 = (((p.p464 * locals.var_inv_l_dn5) + (p.p654 * locals.var_inv_w_dn5)) + (p.p844 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiuc1_dn6 = (((p.p464 * locals.var_inv_l_dn6) + (p.p654 * locals.var_inv_w_dn6)) + (p.p844 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiuc1_dn7 = (((p.p464 * locals.var_inv_l_dn7) + (p.p654 * locals.var_inv_w_dn7)) + (p.p844 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiuc1_dn8 = (((p.p464 * locals.var_inv_l_dn8) + (p.p654 * locals.var_inv_w_dn8)) + (p.p844 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiuc1_dn9 = (((p.p464 * locals.var_inv_l_dn9) + (p.p654 * locals.var_inv_w_dn9)) + (p.p844 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiuc1_dn10 = (((p.p464 * locals.var_inv_l_dn10) + (p.p654 * locals.var_inv_w_dn10)) + (p.p844 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiuc1_dn11 = (((p.p464 * locals.var_inv_l_dn11) + (p.p654 * locals.var_inv_w_dn11)) + (p.p844 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiuc1_dn12 = (((p.p464 * locals.var_inv_l_dn12) + (p.p654 * locals.var_inv_w_dn12)) + (p.p844 * locals.var_inv_lw_dn12));

        let assign2950_e4848: f64 = (p.p465 * locals.var_inv_l);
        let assign2950_e4849: f64 = (p.p75 + assign2950_e4848);
        let assign2950_e4852: f64 = (p.p655 * locals.var_inv_w);
        let assign2950_e4853: f64 = (assign2950_e4849 + assign2950_e4852);
        let assign2950_e4856: f64 = (p.p845 * locals.var_inv_lw);
        let assign2950_e4857: f64 = (assign2950_e4853 + assign2950_e4856);
        locals.var_pparam_b4soiat = assign2950_e4857;
        locals.var_pparam_b4soiat_dn3 = (((p.p465 * locals.var_inv_l_dn3) + (p.p655 * locals.var_inv_w_dn3)) + (p.p845 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiat_dn4 = (((p.p465 * locals.var_inv_l_dn4) + (p.p655 * locals.var_inv_w_dn4)) + (p.p845 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiat_dn5 = (((p.p465 * locals.var_inv_l_dn5) + (p.p655 * locals.var_inv_w_dn5)) + (p.p845 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiat_dn6 = (((p.p465 * locals.var_inv_l_dn6) + (p.p655 * locals.var_inv_w_dn6)) + (p.p845 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiat_dn7 = (((p.p465 * locals.var_inv_l_dn7) + (p.p655 * locals.var_inv_w_dn7)) + (p.p845 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiat_dn8 = (((p.p465 * locals.var_inv_l_dn8) + (p.p655 * locals.var_inv_w_dn8)) + (p.p845 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiat_dn9 = (((p.p465 * locals.var_inv_l_dn9) + (p.p655 * locals.var_inv_w_dn9)) + (p.p845 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiat_dn10 = (((p.p465 * locals.var_inv_l_dn10) + (p.p655 * locals.var_inv_w_dn10)) + (p.p845 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiat_dn11 = (((p.p465 * locals.var_inv_l_dn11) + (p.p655 * locals.var_inv_w_dn11)) + (p.p845 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiat_dn12 = (((p.p465 * locals.var_inv_l_dn12) + (p.p655 * locals.var_inv_w_dn12)) + (p.p845 * locals.var_inv_lw_dn12));

        let assign2960_e4861: f64 = (p.p466 * locals.var_inv_l);
        let assign2960_e4862: f64 = (p.p144 + assign2960_e4861);
        let assign2960_e4865: f64 = (p.p656 * locals.var_inv_w);
        let assign2960_e4866: f64 = (assign2960_e4862 + assign2960_e4865);
        let assign2960_e4869: f64 = (p.p846 * locals.var_inv_lw);
        let assign2960_e4870: f64 = (assign2960_e4866 + assign2960_e4869);
        locals.var_pparam_b4soiprt = assign2960_e4870;
        locals.var_pparam_b4soiprt_dn3 = (((p.p466 * locals.var_inv_l_dn3) + (p.p656 * locals.var_inv_w_dn3)) + (p.p846 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiprt_dn4 = (((p.p466 * locals.var_inv_l_dn4) + (p.p656 * locals.var_inv_w_dn4)) + (p.p846 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiprt_dn5 = (((p.p466 * locals.var_inv_l_dn5) + (p.p656 * locals.var_inv_w_dn5)) + (p.p846 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiprt_dn6 = (((p.p466 * locals.var_inv_l_dn6) + (p.p656 * locals.var_inv_w_dn6)) + (p.p846 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiprt_dn7 = (((p.p466 * locals.var_inv_l_dn7) + (p.p656 * locals.var_inv_w_dn7)) + (p.p846 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiprt_dn8 = (((p.p466 * locals.var_inv_l_dn8) + (p.p656 * locals.var_inv_w_dn8)) + (p.p846 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiprt_dn9 = (((p.p466 * locals.var_inv_l_dn9) + (p.p656 * locals.var_inv_w_dn9)) + (p.p846 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiprt_dn10 = (((p.p466 * locals.var_inv_l_dn10) + (p.p656 * locals.var_inv_w_dn10)) + (p.p846 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiprt_dn11 = (((p.p466 * locals.var_inv_l_dn11) + (p.p656 * locals.var_inv_w_dn11)) + (p.p846 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiprt_dn12 = (((p.p466 * locals.var_inv_l_dn12) + (p.p656 * locals.var_inv_w_dn12)) + (p.p846 * locals.var_inv_lw_dn12));

        let assign2970_e4874: f64 = (p.p484 * locals.var_inv_l);
        let assign2970_e4875: f64 = (p.p406 + assign2970_e4874);
        let assign2970_e4878: f64 = (p.p674 * locals.var_inv_w);
        let assign2970_e4879: f64 = (assign2970_e4875 + assign2970_e4878);
        let assign2970_e4882: f64 = (p.p864 * locals.var_inv_lw);
        let assign2970_e4883: f64 = (assign2970_e4879 + assign2970_e4882);
        locals.var_pparam_b4soinigc = assign2970_e4883;
        locals.var_pparam_b4soinigc_dn3 = (((p.p484 * locals.var_inv_l_dn3) + (p.p674 * locals.var_inv_w_dn3)) + (p.p864 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soinigc_dn4 = (((p.p484 * locals.var_inv_l_dn4) + (p.p674 * locals.var_inv_w_dn4)) + (p.p864 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soinigc_dn5 = (((p.p484 * locals.var_inv_l_dn5) + (p.p674 * locals.var_inv_w_dn5)) + (p.p864 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soinigc_dn6 = (((p.p484 * locals.var_inv_l_dn6) + (p.p674 * locals.var_inv_w_dn6)) + (p.p864 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soinigc_dn7 = (((p.p484 * locals.var_inv_l_dn7) + (p.p674 * locals.var_inv_w_dn7)) + (p.p864 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soinigc_dn8 = (((p.p484 * locals.var_inv_l_dn8) + (p.p674 * locals.var_inv_w_dn8)) + (p.p864 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soinigc_dn9 = (((p.p484 * locals.var_inv_l_dn9) + (p.p674 * locals.var_inv_w_dn9)) + (p.p864 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soinigc_dn10 = (((p.p484 * locals.var_inv_l_dn10) + (p.p674 * locals.var_inv_w_dn10)) + (p.p864 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soinigc_dn11 = (((p.p484 * locals.var_inv_l_dn11) + (p.p674 * locals.var_inv_w_dn11)) + (p.p864 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soinigc_dn12 = (((p.p484 * locals.var_inv_l_dn12) + (p.p674 * locals.var_inv_w_dn12)) + (p.p864 * locals.var_inv_lw_dn12));

        let assign2980_e4887: f64 = (p.p476 * locals.var_inv_l);
        let assign2980_e4888: f64 = (p.p398 + assign2980_e4887);
        let assign2980_e4891: f64 = (p.p666 * locals.var_inv_w);
        let assign2980_e4892: f64 = (assign2980_e4888 + assign2980_e4891);
        let assign2980_e4895: f64 = (p.p856 * locals.var_inv_lw);
        let assign2980_e4896: f64 = (assign2980_e4892 + assign2980_e4895);
        locals.var_pparam_b4soiaigc = assign2980_e4896;
        locals.var_pparam_b4soiaigc_dn3 = (((p.p476 * locals.var_inv_l_dn3) + (p.p666 * locals.var_inv_w_dn3)) + (p.p856 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiaigc_dn4 = (((p.p476 * locals.var_inv_l_dn4) + (p.p666 * locals.var_inv_w_dn4)) + (p.p856 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiaigc_dn5 = (((p.p476 * locals.var_inv_l_dn5) + (p.p666 * locals.var_inv_w_dn5)) + (p.p856 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiaigc_dn6 = (((p.p476 * locals.var_inv_l_dn6) + (p.p666 * locals.var_inv_w_dn6)) + (p.p856 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiaigc_dn7 = (((p.p476 * locals.var_inv_l_dn7) + (p.p666 * locals.var_inv_w_dn7)) + (p.p856 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiaigc_dn8 = (((p.p476 * locals.var_inv_l_dn8) + (p.p666 * locals.var_inv_w_dn8)) + (p.p856 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiaigc_dn9 = (((p.p476 * locals.var_inv_l_dn9) + (p.p666 * locals.var_inv_w_dn9)) + (p.p856 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiaigc_dn10 = (((p.p476 * locals.var_inv_l_dn10) + (p.p666 * locals.var_inv_w_dn10)) + (p.p856 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiaigc_dn11 = (((p.p476 * locals.var_inv_l_dn11) + (p.p666 * locals.var_inv_w_dn11)) + (p.p856 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiaigc_dn12 = (((p.p476 * locals.var_inv_l_dn12) + (p.p666 * locals.var_inv_w_dn12)) + (p.p856 * locals.var_inv_lw_dn12));

        let assign2990_e4900: f64 = (p.p477 * locals.var_inv_l);
        let assign2990_e4901: f64 = (p.p399 + assign2990_e4900);
        let assign2990_e4904: f64 = (p.p667 * locals.var_inv_w);
        let assign2990_e4905: f64 = (assign2990_e4901 + assign2990_e4904);
        let assign2990_e4908: f64 = (p.p857 * locals.var_inv_lw);
        let assign2990_e4909: f64 = (assign2990_e4905 + assign2990_e4908);
        locals.var_pparam_b4soiaigc1 = assign2990_e4909;
        locals.var_pparam_b4soiaigc1_dn3 = (((p.p477 * locals.var_inv_l_dn3) + (p.p667 * locals.var_inv_w_dn3)) + (p.p857 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiaigc1_dn4 = (((p.p477 * locals.var_inv_l_dn4) + (p.p667 * locals.var_inv_w_dn4)) + (p.p857 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiaigc1_dn5 = (((p.p477 * locals.var_inv_l_dn5) + (p.p667 * locals.var_inv_w_dn5)) + (p.p857 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiaigc1_dn6 = (((p.p477 * locals.var_inv_l_dn6) + (p.p667 * locals.var_inv_w_dn6)) + (p.p857 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiaigc1_dn7 = (((p.p477 * locals.var_inv_l_dn7) + (p.p667 * locals.var_inv_w_dn7)) + (p.p857 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiaigc1_dn8 = (((p.p477 * locals.var_inv_l_dn8) + (p.p667 * locals.var_inv_w_dn8)) + (p.p857 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiaigc1_dn9 = (((p.p477 * locals.var_inv_l_dn9) + (p.p667 * locals.var_inv_w_dn9)) + (p.p857 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiaigc1_dn10 = (((p.p477 * locals.var_inv_l_dn10) + (p.p667 * locals.var_inv_w_dn10)) + (p.p857 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiaigc1_dn11 = (((p.p477 * locals.var_inv_l_dn11) + (p.p667 * locals.var_inv_w_dn11)) + (p.p857 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiaigc1_dn12 = (((p.p477 * locals.var_inv_l_dn12) + (p.p667 * locals.var_inv_w_dn12)) + (p.p857 * locals.var_inv_lw_dn12));

        let assign3000_e4913: f64 = (p.p478 * locals.var_inv_l);
        let assign3000_e4914: f64 = (p.p400 + assign3000_e4913);
        let assign3000_e4917: f64 = (p.p668 * locals.var_inv_w);
        let assign3000_e4918: f64 = (assign3000_e4914 + assign3000_e4917);
        let assign3000_e4921: f64 = (p.p858 * locals.var_inv_lw);
        let assign3000_e4922: f64 = (assign3000_e4918 + assign3000_e4921);
        locals.var_pparam_b4soibigc = assign3000_e4922;
        locals.var_pparam_b4soibigc_dn3 = (((p.p478 * locals.var_inv_l_dn3) + (p.p668 * locals.var_inv_w_dn3)) + (p.p858 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soibigc_dn4 = (((p.p478 * locals.var_inv_l_dn4) + (p.p668 * locals.var_inv_w_dn4)) + (p.p858 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soibigc_dn5 = (((p.p478 * locals.var_inv_l_dn5) + (p.p668 * locals.var_inv_w_dn5)) + (p.p858 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soibigc_dn6 = (((p.p478 * locals.var_inv_l_dn6) + (p.p668 * locals.var_inv_w_dn6)) + (p.p858 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soibigc_dn7 = (((p.p478 * locals.var_inv_l_dn7) + (p.p668 * locals.var_inv_w_dn7)) + (p.p858 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soibigc_dn8 = (((p.p478 * locals.var_inv_l_dn8) + (p.p668 * locals.var_inv_w_dn8)) + (p.p858 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soibigc_dn9 = (((p.p478 * locals.var_inv_l_dn9) + (p.p668 * locals.var_inv_w_dn9)) + (p.p858 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soibigc_dn10 = (((p.p478 * locals.var_inv_l_dn10) + (p.p668 * locals.var_inv_w_dn10)) + (p.p858 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soibigc_dn11 = (((p.p478 * locals.var_inv_l_dn11) + (p.p668 * locals.var_inv_w_dn11)) + (p.p858 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soibigc_dn12 = (((p.p478 * locals.var_inv_l_dn12) + (p.p668 * locals.var_inv_w_dn12)) + (p.p858 * locals.var_inv_lw_dn12));

    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3010_e4926: f64 = (p.p479 * locals.var_inv_l);
        let assign3010_e4927: f64 = (p.p401 + assign3010_e4926);
        let assign3010_e4930: f64 = (p.p669 * locals.var_inv_w);
        let assign3010_e4931: f64 = (assign3010_e4927 + assign3010_e4930);
        let assign3010_e4934: f64 = (p.p859 * locals.var_inv_lw);
        let assign3010_e4935: f64 = (assign3010_e4931 + assign3010_e4934);
        locals.var_pparam_b4soicigc = assign3010_e4935;
        locals.var_pparam_b4soicigc_dn3 = (((p.p479 * locals.var_inv_l_dn3) + (p.p669 * locals.var_inv_w_dn3)) + (p.p859 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soicigc_dn4 = (((p.p479 * locals.var_inv_l_dn4) + (p.p669 * locals.var_inv_w_dn4)) + (p.p859 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soicigc_dn5 = (((p.p479 * locals.var_inv_l_dn5) + (p.p669 * locals.var_inv_w_dn5)) + (p.p859 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soicigc_dn6 = (((p.p479 * locals.var_inv_l_dn6) + (p.p669 * locals.var_inv_w_dn6)) + (p.p859 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soicigc_dn7 = (((p.p479 * locals.var_inv_l_dn7) + (p.p669 * locals.var_inv_w_dn7)) + (p.p859 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soicigc_dn8 = (((p.p479 * locals.var_inv_l_dn8) + (p.p669 * locals.var_inv_w_dn8)) + (p.p859 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soicigc_dn9 = (((p.p479 * locals.var_inv_l_dn9) + (p.p669 * locals.var_inv_w_dn9)) + (p.p859 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soicigc_dn10 = (((p.p479 * locals.var_inv_l_dn10) + (p.p669 * locals.var_inv_w_dn10)) + (p.p859 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soicigc_dn11 = (((p.p479 * locals.var_inv_l_dn11) + (p.p669 * locals.var_inv_w_dn11)) + (p.p859 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soicigc_dn12 = (((p.p479 * locals.var_inv_l_dn12) + (p.p669 * locals.var_inv_w_dn12)) + (p.p859 * locals.var_inv_lw_dn12));

        let assign3020_e4939: f64 = (p.p480 * locals.var_inv_l);
        let assign3020_e4940: f64 = (p.p402 + assign3020_e4939);
        let assign3020_e4943: f64 = (p.p670 * locals.var_inv_w);
        let assign3020_e4944: f64 = (assign3020_e4940 + assign3020_e4943);
        let assign3020_e4947: f64 = (p.p860 * locals.var_inv_lw);
        let assign3020_e4948: f64 = (assign3020_e4944 + assign3020_e4947);
        locals.var_pparam_b4soiaigsd = assign3020_e4948;
        locals.var_pparam_b4soiaigsd_dn3 = (((p.p480 * locals.var_inv_l_dn3) + (p.p670 * locals.var_inv_w_dn3)) + (p.p860 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiaigsd_dn4 = (((p.p480 * locals.var_inv_l_dn4) + (p.p670 * locals.var_inv_w_dn4)) + (p.p860 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiaigsd_dn5 = (((p.p480 * locals.var_inv_l_dn5) + (p.p670 * locals.var_inv_w_dn5)) + (p.p860 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiaigsd_dn6 = (((p.p480 * locals.var_inv_l_dn6) + (p.p670 * locals.var_inv_w_dn6)) + (p.p860 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiaigsd_dn7 = (((p.p480 * locals.var_inv_l_dn7) + (p.p670 * locals.var_inv_w_dn7)) + (p.p860 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiaigsd_dn8 = (((p.p480 * locals.var_inv_l_dn8) + (p.p670 * locals.var_inv_w_dn8)) + (p.p860 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiaigsd_dn9 = (((p.p480 * locals.var_inv_l_dn9) + (p.p670 * locals.var_inv_w_dn9)) + (p.p860 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiaigsd_dn10 = (((p.p480 * locals.var_inv_l_dn10) + (p.p670 * locals.var_inv_w_dn10)) + (p.p860 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiaigsd_dn11 = (((p.p480 * locals.var_inv_l_dn11) + (p.p670 * locals.var_inv_w_dn11)) + (p.p860 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiaigsd_dn12 = (((p.p480 * locals.var_inv_l_dn12) + (p.p670 * locals.var_inv_w_dn12)) + (p.p860 * locals.var_inv_lw_dn12));

        let assign3030_e4952: f64 = (p.p481 * locals.var_inv_l);
        let assign3030_e4953: f64 = (p.p403 + assign3030_e4952);
        let assign3030_e4956: f64 = (p.p671 * locals.var_inv_w);
        let assign3030_e4957: f64 = (assign3030_e4953 + assign3030_e4956);
        let assign3030_e4960: f64 = (p.p861 * locals.var_inv_lw);
        let assign3030_e4961: f64 = (assign3030_e4957 + assign3030_e4960);
        locals.var_pparam_b4soiaigsd1 = assign3030_e4961;
        locals.var_pparam_b4soiaigsd1_dn3 = (((p.p481 * locals.var_inv_l_dn3) + (p.p671 * locals.var_inv_w_dn3)) + (p.p861 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiaigsd1_dn4 = (((p.p481 * locals.var_inv_l_dn4) + (p.p671 * locals.var_inv_w_dn4)) + (p.p861 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiaigsd1_dn5 = (((p.p481 * locals.var_inv_l_dn5) + (p.p671 * locals.var_inv_w_dn5)) + (p.p861 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiaigsd1_dn6 = (((p.p481 * locals.var_inv_l_dn6) + (p.p671 * locals.var_inv_w_dn6)) + (p.p861 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiaigsd1_dn7 = (((p.p481 * locals.var_inv_l_dn7) + (p.p671 * locals.var_inv_w_dn7)) + (p.p861 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiaigsd1_dn8 = (((p.p481 * locals.var_inv_l_dn8) + (p.p671 * locals.var_inv_w_dn8)) + (p.p861 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiaigsd1_dn9 = (((p.p481 * locals.var_inv_l_dn9) + (p.p671 * locals.var_inv_w_dn9)) + (p.p861 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiaigsd1_dn10 = (((p.p481 * locals.var_inv_l_dn10) + (p.p671 * locals.var_inv_w_dn10)) + (p.p861 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiaigsd1_dn11 = (((p.p481 * locals.var_inv_l_dn11) + (p.p671 * locals.var_inv_w_dn11)) + (p.p861 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiaigsd1_dn12 = (((p.p481 * locals.var_inv_l_dn12) + (p.p671 * locals.var_inv_w_dn12)) + (p.p861 * locals.var_inv_lw_dn12));

        let assign3040_e4965: f64 = (p.p482 * locals.var_inv_l);
        let assign3040_e4966: f64 = (p.p404 + assign3040_e4965);
        let assign3040_e4969: f64 = (p.p672 * locals.var_inv_w);
        let assign3040_e4970: f64 = (assign3040_e4966 + assign3040_e4969);
        let assign3040_e4973: f64 = (p.p862 * locals.var_inv_lw);
        let assign3040_e4974: f64 = (assign3040_e4970 + assign3040_e4973);
        locals.var_pparam_b4soibigsd = assign3040_e4974;
        locals.var_pparam_b4soibigsd_dn3 = (((p.p482 * locals.var_inv_l_dn3) + (p.p672 * locals.var_inv_w_dn3)) + (p.p862 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soibigsd_dn4 = (((p.p482 * locals.var_inv_l_dn4) + (p.p672 * locals.var_inv_w_dn4)) + (p.p862 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soibigsd_dn5 = (((p.p482 * locals.var_inv_l_dn5) + (p.p672 * locals.var_inv_w_dn5)) + (p.p862 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soibigsd_dn6 = (((p.p482 * locals.var_inv_l_dn6) + (p.p672 * locals.var_inv_w_dn6)) + (p.p862 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soibigsd_dn7 = (((p.p482 * locals.var_inv_l_dn7) + (p.p672 * locals.var_inv_w_dn7)) + (p.p862 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soibigsd_dn8 = (((p.p482 * locals.var_inv_l_dn8) + (p.p672 * locals.var_inv_w_dn8)) + (p.p862 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soibigsd_dn9 = (((p.p482 * locals.var_inv_l_dn9) + (p.p672 * locals.var_inv_w_dn9)) + (p.p862 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soibigsd_dn10 = (((p.p482 * locals.var_inv_l_dn10) + (p.p672 * locals.var_inv_w_dn10)) + (p.p862 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soibigsd_dn11 = (((p.p482 * locals.var_inv_l_dn11) + (p.p672 * locals.var_inv_w_dn11)) + (p.p862 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soibigsd_dn12 = (((p.p482 * locals.var_inv_l_dn12) + (p.p672 * locals.var_inv_w_dn12)) + (p.p862 * locals.var_inv_lw_dn12));

        let assign3050_e4978: f64 = (p.p483 * locals.var_inv_l);
        let assign3050_e4979: f64 = (p.p405 + assign3050_e4978);
        let assign3050_e4982: f64 = (p.p673 * locals.var_inv_w);
        let assign3050_e4983: f64 = (assign3050_e4979 + assign3050_e4982);
        let assign3050_e4986: f64 = (p.p863 * locals.var_inv_lw);
        let assign3050_e4987: f64 = (assign3050_e4983 + assign3050_e4986);
        locals.var_pparam_b4soicigsd = assign3050_e4987;
        locals.var_pparam_b4soicigsd_dn3 = (((p.p483 * locals.var_inv_l_dn3) + (p.p673 * locals.var_inv_w_dn3)) + (p.p863 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soicigsd_dn4 = (((p.p483 * locals.var_inv_l_dn4) + (p.p673 * locals.var_inv_w_dn4)) + (p.p863 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soicigsd_dn5 = (((p.p483 * locals.var_inv_l_dn5) + (p.p673 * locals.var_inv_w_dn5)) + (p.p863 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soicigsd_dn6 = (((p.p483 * locals.var_inv_l_dn6) + (p.p673 * locals.var_inv_w_dn6)) + (p.p863 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soicigsd_dn7 = (((p.p483 * locals.var_inv_l_dn7) + (p.p673 * locals.var_inv_w_dn7)) + (p.p863 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soicigsd_dn8 = (((p.p483 * locals.var_inv_l_dn8) + (p.p673 * locals.var_inv_w_dn8)) + (p.p863 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soicigsd_dn9 = (((p.p483 * locals.var_inv_l_dn9) + (p.p673 * locals.var_inv_w_dn9)) + (p.p863 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soicigsd_dn10 = (((p.p483 * locals.var_inv_l_dn10) + (p.p673 * locals.var_inv_w_dn10)) + (p.p863 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soicigsd_dn11 = (((p.p483 * locals.var_inv_l_dn11) + (p.p673 * locals.var_inv_w_dn11)) + (p.p863 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soicigsd_dn12 = (((p.p483 * locals.var_inv_l_dn12) + (p.p673 * locals.var_inv_w_dn12)) + (p.p863 * locals.var_inv_lw_dn12));

        let assign3060_e4991: f64 = (p.p485 * locals.var_inv_l);
        let assign3060_e4992: f64 = (p.p407 + assign3060_e4991);
        let assign3060_e4995: f64 = (p.p675 * locals.var_inv_w);
        let assign3060_e4996: f64 = (assign3060_e4992 + assign3060_e4995);
        let assign3060_e4999: f64 = (p.p865 * locals.var_inv_lw);
        let assign3060_e5000: f64 = (assign3060_e4996 + assign3060_e4999);
        locals.var_pparam_b4soipigcd = assign3060_e5000;
        locals.var_pparam_b4soipigcd_dn3 = (((p.p485 * locals.var_inv_l_dn3) + (p.p675 * locals.var_inv_w_dn3)) + (p.p865 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soipigcd_dn4 = (((p.p485 * locals.var_inv_l_dn4) + (p.p675 * locals.var_inv_w_dn4)) + (p.p865 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soipigcd_dn5 = (((p.p485 * locals.var_inv_l_dn5) + (p.p675 * locals.var_inv_w_dn5)) + (p.p865 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soipigcd_dn6 = (((p.p485 * locals.var_inv_l_dn6) + (p.p675 * locals.var_inv_w_dn6)) + (p.p865 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soipigcd_dn7 = (((p.p485 * locals.var_inv_l_dn7) + (p.p675 * locals.var_inv_w_dn7)) + (p.p865 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soipigcd_dn8 = (((p.p485 * locals.var_inv_l_dn8) + (p.p675 * locals.var_inv_w_dn8)) + (p.p865 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soipigcd_dn9 = (((p.p485 * locals.var_inv_l_dn9) + (p.p675 * locals.var_inv_w_dn9)) + (p.p865 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soipigcd_dn10 = (((p.p485 * locals.var_inv_l_dn10) + (p.p675 * locals.var_inv_w_dn10)) + (p.p865 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soipigcd_dn11 = (((p.p485 * locals.var_inv_l_dn11) + (p.p675 * locals.var_inv_w_dn11)) + (p.p865 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soipigcd_dn12 = (((p.p485 * locals.var_inv_l_dn12) + (p.p675 * locals.var_inv_w_dn12)) + (p.p865 * locals.var_inv_lw_dn12));

        let assign3070_e5004: f64 = (p.p486 * locals.var_inv_l);
        let assign3070_e5005: f64 = (p.p408 + assign3070_e5004);
        let assign3070_e5008: f64 = (p.p676 * locals.var_inv_w);
        let assign3070_e5009: f64 = (assign3070_e5005 + assign3070_e5008);
        let assign3070_e5012: f64 = (p.p866 * locals.var_inv_lw);
        let assign3070_e5013: f64 = (assign3070_e5009 + assign3070_e5012);
        locals.var_pparam_b4soipoxedge = assign3070_e5013;
        locals.var_pparam_b4soipoxedge_dn3 = (((p.p486 * locals.var_inv_l_dn3) + (p.p676 * locals.var_inv_w_dn3)) + (p.p866 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soipoxedge_dn4 = (((p.p486 * locals.var_inv_l_dn4) + (p.p676 * locals.var_inv_w_dn4)) + (p.p866 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soipoxedge_dn5 = (((p.p486 * locals.var_inv_l_dn5) + (p.p676 * locals.var_inv_w_dn5)) + (p.p866 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soipoxedge_dn6 = (((p.p486 * locals.var_inv_l_dn6) + (p.p676 * locals.var_inv_w_dn6)) + (p.p866 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soipoxedge_dn7 = (((p.p486 * locals.var_inv_l_dn7) + (p.p676 * locals.var_inv_w_dn7)) + (p.p866 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soipoxedge_dn8 = (((p.p486 * locals.var_inv_l_dn8) + (p.p676 * locals.var_inv_w_dn8)) + (p.p866 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soipoxedge_dn9 = (((p.p486 * locals.var_inv_l_dn9) + (p.p676 * locals.var_inv_w_dn9)) + (p.p866 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soipoxedge_dn10 = (((p.p486 * locals.var_inv_l_dn10) + (p.p676 * locals.var_inv_w_dn10)) + (p.p866 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soipoxedge_dn11 = (((p.p486 * locals.var_inv_l_dn11) + (p.p676 * locals.var_inv_w_dn11)) + (p.p866 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soipoxedge_dn12 = (((p.p486 * locals.var_inv_l_dn12) + (p.p676 * locals.var_inv_w_dn12)) + (p.p866 * locals.var_inv_lw_dn12));

        let assign3080_e5017: f64 = (p.p487 * locals.var_inv_l);
        let assign3080_e5018: f64 = (p.p409 + assign3080_e5017);
        let assign3080_e5021: f64 = (p.p677 * locals.var_inv_w);
        let assign3080_e5022: f64 = (assign3080_e5018 + assign3080_e5021);
        let assign3080_e5025: f64 = (p.p867 * locals.var_inv_lw);
        let assign3080_e5026: f64 = (assign3080_e5022 + assign3080_e5025);
        locals.var_pparam_b4soiigt = assign3080_e5026;
        locals.var_pparam_b4soiigt_dn3 = (((p.p487 * locals.var_inv_l_dn3) + (p.p677 * locals.var_inv_w_dn3)) + (p.p867 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiigt_dn4 = (((p.p487 * locals.var_inv_l_dn4) + (p.p677 * locals.var_inv_w_dn4)) + (p.p867 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiigt_dn5 = (((p.p487 * locals.var_inv_l_dn5) + (p.p677 * locals.var_inv_w_dn5)) + (p.p867 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiigt_dn6 = (((p.p487 * locals.var_inv_l_dn6) + (p.p677 * locals.var_inv_w_dn6)) + (p.p867 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiigt_dn7 = (((p.p487 * locals.var_inv_l_dn7) + (p.p677 * locals.var_inv_w_dn7)) + (p.p867 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiigt_dn8 = (((p.p487 * locals.var_inv_l_dn8) + (p.p677 * locals.var_inv_w_dn8)) + (p.p867 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiigt_dn9 = (((p.p487 * locals.var_inv_l_dn9) + (p.p677 * locals.var_inv_w_dn9)) + (p.p867 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiigt_dn10 = (((p.p487 * locals.var_inv_l_dn10) + (p.p677 * locals.var_inv_w_dn10)) + (p.p867 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiigt_dn11 = (((p.p487 * locals.var_inv_l_dn11) + (p.p677 * locals.var_inv_w_dn11)) + (p.p867 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiigt_dn12 = (((p.p487 * locals.var_inv_l_dn12) + (p.p677 * locals.var_inv_w_dn12)) + (p.p867 * locals.var_inv_lw_dn12));

        let assign3090_e5030: f64 = (p.p618 * locals.var_inv_l);
        let assign3090_e5031: f64 = (p.p422 + assign3090_e5030);
        let assign3090_e5034: f64 = (p.p808 * locals.var_inv_w);
        let assign3090_e5035: f64 = (assign3090_e5031 + assign3090_e5034);
        let assign3090_e5038: f64 = (p.p998 * locals.var_inv_lw);
        let assign3090_e5039: f64 = (assign3090_e5035 + assign3090_e5038);
        locals.var_pparam_b4soixrcrg1 = assign3090_e5039;
        locals.var_pparam_b4soixrcrg1_dn3 = (((p.p618 * locals.var_inv_l_dn3) + (p.p808 * locals.var_inv_w_dn3)) + (p.p998 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soixrcrg1_dn4 = (((p.p618 * locals.var_inv_l_dn4) + (p.p808 * locals.var_inv_w_dn4)) + (p.p998 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soixrcrg1_dn5 = (((p.p618 * locals.var_inv_l_dn5) + (p.p808 * locals.var_inv_w_dn5)) + (p.p998 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soixrcrg1_dn6 = (((p.p618 * locals.var_inv_l_dn6) + (p.p808 * locals.var_inv_w_dn6)) + (p.p998 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soixrcrg1_dn7 = (((p.p618 * locals.var_inv_l_dn7) + (p.p808 * locals.var_inv_w_dn7)) + (p.p998 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soixrcrg1_dn8 = (((p.p618 * locals.var_inv_l_dn8) + (p.p808 * locals.var_inv_w_dn8)) + (p.p998 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soixrcrg1_dn9 = (((p.p618 * locals.var_inv_l_dn9) + (p.p808 * locals.var_inv_w_dn9)) + (p.p998 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soixrcrg1_dn10 = (((p.p618 * locals.var_inv_l_dn10) + (p.p808 * locals.var_inv_w_dn10)) + (p.p998 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soixrcrg1_dn11 = (((p.p618 * locals.var_inv_l_dn11) + (p.p808 * locals.var_inv_w_dn11)) + (p.p998 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soixrcrg1_dn12 = (((p.p618 * locals.var_inv_l_dn12) + (p.p808 * locals.var_inv_w_dn12)) + (p.p998 * locals.var_inv_lw_dn12));

        let assign3100_e5043: f64 = (p.p619 * locals.var_inv_l);
        let assign3100_e5044: f64 = (p.p423 + assign3100_e5043);
        let assign3100_e5047: f64 = (p.p809 * locals.var_inv_w);
        let assign3100_e5048: f64 = (assign3100_e5044 + assign3100_e5047);
        let assign3100_e5051: f64 = (p.p999 * locals.var_inv_lw);
        let assign3100_e5052: f64 = (assign3100_e5048 + assign3100_e5051);
        locals.var_pparam_b4soixrcrg2 = assign3100_e5052;
        locals.var_pparam_b4soixrcrg2_dn3 = (((p.p619 * locals.var_inv_l_dn3) + (p.p809 * locals.var_inv_w_dn3)) + (p.p999 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soixrcrg2_dn4 = (((p.p619 * locals.var_inv_l_dn4) + (p.p809 * locals.var_inv_w_dn4)) + (p.p999 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soixrcrg2_dn5 = (((p.p619 * locals.var_inv_l_dn5) + (p.p809 * locals.var_inv_w_dn5)) + (p.p999 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soixrcrg2_dn6 = (((p.p619 * locals.var_inv_l_dn6) + (p.p809 * locals.var_inv_w_dn6)) + (p.p999 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soixrcrg2_dn7 = (((p.p619 * locals.var_inv_l_dn7) + (p.p809 * locals.var_inv_w_dn7)) + (p.p999 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soixrcrg2_dn8 = (((p.p619 * locals.var_inv_l_dn8) + (p.p809 * locals.var_inv_w_dn8)) + (p.p999 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soixrcrg2_dn9 = (((p.p619 * locals.var_inv_l_dn9) + (p.p809 * locals.var_inv_w_dn9)) + (p.p999 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soixrcrg2_dn10 = (((p.p619 * locals.var_inv_l_dn10) + (p.p809 * locals.var_inv_w_dn10)) + (p.p999 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soixrcrg2_dn11 = (((p.p619 * locals.var_inv_l_dn11) + (p.p809 * locals.var_inv_w_dn11)) + (p.p999 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soixrcrg2_dn12 = (((p.p619 * locals.var_inv_l_dn12) + (p.p809 * locals.var_inv_w_dn12)) + (p.p999 * locals.var_inv_lw_dn12));

        let assign3110_e5056: f64 = (p.p620 * locals.var_inv_l);
        let assign3110_e5057: f64 = (p.p413 + assign3110_e5056);
        let assign3110_e5060: f64 = (p.p810 * locals.var_inv_w);
        let assign3110_e5061: f64 = (assign3110_e5057 + assign3110_e5060);
        let assign3110_e5064: f64 = (p.p1000 * locals.var_inv_lw);
        let assign3110_e5065: f64 = (assign3110_e5061 + assign3110_e5064);
        locals.var_pparam_b4soivbsa = assign3110_e5065;
        locals.var_pparam_b4soivbsa_dn3 = (((p.p620 * locals.var_inv_l_dn3) + (p.p810 * locals.var_inv_w_dn3)) + (p.p1000 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivbsa_dn4 = (((p.p620 * locals.var_inv_l_dn4) + (p.p810 * locals.var_inv_w_dn4)) + (p.p1000 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivbsa_dn5 = (((p.p620 * locals.var_inv_l_dn5) + (p.p810 * locals.var_inv_w_dn5)) + (p.p1000 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivbsa_dn6 = (((p.p620 * locals.var_inv_l_dn6) + (p.p810 * locals.var_inv_w_dn6)) + (p.p1000 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivbsa_dn7 = (((p.p620 * locals.var_inv_l_dn7) + (p.p810 * locals.var_inv_w_dn7)) + (p.p1000 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivbsa_dn8 = (((p.p620 * locals.var_inv_l_dn8) + (p.p810 * locals.var_inv_w_dn8)) + (p.p1000 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivbsa_dn9 = (((p.p620 * locals.var_inv_l_dn9) + (p.p810 * locals.var_inv_w_dn9)) + (p.p1000 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivbsa_dn10 = (((p.p620 * locals.var_inv_l_dn10) + (p.p810 * locals.var_inv_w_dn10)) + (p.p1000 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivbsa_dn11 = (((p.p620 * locals.var_inv_l_dn11) + (p.p810 * locals.var_inv_w_dn11)) + (p.p1000 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivbsa_dn12 = (((p.p620 * locals.var_inv_l_dn12) + (p.p810 * locals.var_inv_w_dn12)) + (p.p1000 * locals.var_inv_lw_dn12));

        let assign3120_e5069: f64 = (p.p621 * locals.var_inv_l);
        let assign3120_e5070: f64 = (p.p433 + assign3120_e5069);
        let assign3120_e5073: f64 = (p.p811 * locals.var_inv_w);
        let assign3120_e5074: f64 = (assign3120_e5070 + assign3120_e5073);
        let assign3120_e5077: f64 = (p.p1001 * locals.var_inv_lw);
        let assign3120_e5078: f64 = (assign3120_e5074 + assign3120_e5077);
        locals.var_pparam_b4soivsce = assign3120_e5078;
        locals.var_pparam_b4soivsce_dn3 = (((p.p621 * locals.var_inv_l_dn3) + (p.p811 * locals.var_inv_w_dn3)) + (p.p1001 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivsce_dn4 = (((p.p621 * locals.var_inv_l_dn4) + (p.p811 * locals.var_inv_w_dn4)) + (p.p1001 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivsce_dn5 = (((p.p621 * locals.var_inv_l_dn5) + (p.p811 * locals.var_inv_w_dn5)) + (p.p1001 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivsce_dn6 = (((p.p621 * locals.var_inv_l_dn6) + (p.p811 * locals.var_inv_w_dn6)) + (p.p1001 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivsce_dn7 = (((p.p621 * locals.var_inv_l_dn7) + (p.p811 * locals.var_inv_w_dn7)) + (p.p1001 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivsce_dn8 = (((p.p621 * locals.var_inv_l_dn8) + (p.p811 * locals.var_inv_w_dn8)) + (p.p1001 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivsce_dn9 = (((p.p621 * locals.var_inv_l_dn9) + (p.p811 * locals.var_inv_w_dn9)) + (p.p1001 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivsce_dn10 = (((p.p621 * locals.var_inv_l_dn10) + (p.p811 * locals.var_inv_w_dn10)) + (p.p1001 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivsce_dn11 = (((p.p621 * locals.var_inv_l_dn11) + (p.p811 * locals.var_inv_w_dn11)) + (p.p1001 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivsce_dn12 = (((p.p621 * locals.var_inv_l_dn12) + (p.p811 * locals.var_inv_w_dn12)) + (p.p1001 * locals.var_inv_lw_dn12));

        let assign3130_e5082: f64 = (p.p622 * locals.var_inv_l);
        let assign3130_e5083: f64 = (p.p434 + assign3130_e5082);
        let assign3130_e5086: f64 = (p.p812 * locals.var_inv_w);
        let assign3130_e5087: f64 = (assign3130_e5083 + assign3130_e5086);
        let assign3130_e5090: f64 = (p.p1002 * locals.var_inv_lw);
        let assign3130_e5091: f64 = (assign3130_e5087 + assign3130_e5090);
        locals.var_pparam_b4soicdsbs = assign3130_e5091;
        locals.var_pparam_b4soicdsbs_dn3 = (((p.p622 * locals.var_inv_l_dn3) + (p.p812 * locals.var_inv_w_dn3)) + (p.p1002 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soicdsbs_dn4 = (((p.p622 * locals.var_inv_l_dn4) + (p.p812 * locals.var_inv_w_dn4)) + (p.p1002 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soicdsbs_dn5 = (((p.p622 * locals.var_inv_l_dn5) + (p.p812 * locals.var_inv_w_dn5)) + (p.p1002 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soicdsbs_dn6 = (((p.p622 * locals.var_inv_l_dn6) + (p.p812 * locals.var_inv_w_dn6)) + (p.p1002 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soicdsbs_dn7 = (((p.p622 * locals.var_inv_l_dn7) + (p.p812 * locals.var_inv_w_dn7)) + (p.p1002 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soicdsbs_dn8 = (((p.p622 * locals.var_inv_l_dn8) + (p.p812 * locals.var_inv_w_dn8)) + (p.p1002 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soicdsbs_dn9 = (((p.p622 * locals.var_inv_l_dn9) + (p.p812 * locals.var_inv_w_dn9)) + (p.p1002 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soicdsbs_dn10 = (((p.p622 * locals.var_inv_l_dn10) + (p.p812 * locals.var_inv_w_dn10)) + (p.p1002 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soicdsbs_dn11 = (((p.p622 * locals.var_inv_l_dn11) + (p.p812 * locals.var_inv_w_dn11)) + (p.p1002 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soicdsbs_dn12 = (((p.p622 * locals.var_inv_l_dn12) + (p.p812 * locals.var_inv_w_dn12)) + (p.p1002 * locals.var_inv_lw_dn12));

        let assign3140_e5095: f64 = (p.p623 * locals.var_inv_l);
        let assign3140_e5096: f64 = (p.p414 + assign3140_e5095);
        let assign3140_e5099: f64 = (p.p813 * locals.var_inv_w);
        let assign3140_e5100: f64 = (assign3140_e5096 + assign3140_e5099);
        let assign3140_e5103: f64 = (p.p1003 * locals.var_inv_lw);
        let assign3140_e5104: f64 = (assign3140_e5100 + assign3140_e5103);
        locals.var_pparam_b4soinofffd = assign3140_e5104;
        locals.var_pparam_b4soinofffd_dn3 = (((p.p623 * locals.var_inv_l_dn3) + (p.p813 * locals.var_inv_w_dn3)) + (p.p1003 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soinofffd_dn4 = (((p.p623 * locals.var_inv_l_dn4) + (p.p813 * locals.var_inv_w_dn4)) + (p.p1003 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soinofffd_dn5 = (((p.p623 * locals.var_inv_l_dn5) + (p.p813 * locals.var_inv_w_dn5)) + (p.p1003 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soinofffd_dn6 = (((p.p623 * locals.var_inv_l_dn6) + (p.p813 * locals.var_inv_w_dn6)) + (p.p1003 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soinofffd_dn7 = (((p.p623 * locals.var_inv_l_dn7) + (p.p813 * locals.var_inv_w_dn7)) + (p.p1003 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soinofffd_dn8 = (((p.p623 * locals.var_inv_l_dn8) + (p.p813 * locals.var_inv_w_dn8)) + (p.p1003 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soinofffd_dn9 = (((p.p623 * locals.var_inv_l_dn9) + (p.p813 * locals.var_inv_w_dn9)) + (p.p1003 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soinofffd_dn10 = (((p.p623 * locals.var_inv_l_dn10) + (p.p813 * locals.var_inv_w_dn10)) + (p.p1003 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soinofffd_dn11 = (((p.p623 * locals.var_inv_l_dn11) + (p.p813 * locals.var_inv_w_dn11)) + (p.p1003 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soinofffd_dn12 = (((p.p623 * locals.var_inv_l_dn12) + (p.p813 * locals.var_inv_w_dn12)) + (p.p1003 * locals.var_inv_lw_dn12));

        let assign3150_e5108: f64 = (p.p624 * locals.var_inv_l);
        let assign3150_e5109: f64 = (p.p415 + assign3150_e5108);
        let assign3150_e5112: f64 = (p.p814 * locals.var_inv_w);
        let assign3150_e5113: f64 = (assign3150_e5109 + assign3150_e5112);
        let assign3150_e5116: f64 = (p.p1004 * locals.var_inv_lw);
        let assign3150_e5117: f64 = (assign3150_e5113 + assign3150_e5116);
        locals.var_pparam_b4soivofffd = assign3150_e5117;
        locals.var_pparam_b4soivofffd_dn3 = (((p.p624 * locals.var_inv_l_dn3) + (p.p814 * locals.var_inv_w_dn3)) + (p.p1004 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivofffd_dn4 = (((p.p624 * locals.var_inv_l_dn4) + (p.p814 * locals.var_inv_w_dn4)) + (p.p1004 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivofffd_dn5 = (((p.p624 * locals.var_inv_l_dn5) + (p.p814 * locals.var_inv_w_dn5)) + (p.p1004 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivofffd_dn6 = (((p.p624 * locals.var_inv_l_dn6) + (p.p814 * locals.var_inv_w_dn6)) + (p.p1004 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivofffd_dn7 = (((p.p624 * locals.var_inv_l_dn7) + (p.p814 * locals.var_inv_w_dn7)) + (p.p1004 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivofffd_dn8 = (((p.p624 * locals.var_inv_l_dn8) + (p.p814 * locals.var_inv_w_dn8)) + (p.p1004 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivofffd_dn9 = (((p.p624 * locals.var_inv_l_dn9) + (p.p814 * locals.var_inv_w_dn9)) + (p.p1004 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivofffd_dn10 = (((p.p624 * locals.var_inv_l_dn10) + (p.p814 * locals.var_inv_w_dn10)) + (p.p1004 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivofffd_dn11 = (((p.p624 * locals.var_inv_l_dn11) + (p.p814 * locals.var_inv_w_dn11)) + (p.p1004 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivofffd_dn12 = (((p.p624 * locals.var_inv_l_dn12) + (p.p814 * locals.var_inv_w_dn12)) + (p.p1004 * locals.var_inv_lw_dn12));

        let assign3160_e5121: f64 = (p.p625 * locals.var_inv_l);
        let assign3160_e5122: f64 = (p.p416 + assign3160_e5121);
        let assign3160_e5125: f64 = (p.p815 * locals.var_inv_w);
        let assign3160_e5126: f64 = (assign3160_e5122 + assign3160_e5125);
        let assign3160_e5129: f64 = (p.p1005 * locals.var_inv_lw);
        let assign3160_e5130: f64 = (assign3160_e5126 + assign3160_e5129);
        locals.var_pparam_b4soik1b = assign3160_e5130;
        locals.var_pparam_b4soik1b_dn3 = (((p.p625 * locals.var_inv_l_dn3) + (p.p815 * locals.var_inv_w_dn3)) + (p.p1005 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soik1b_dn4 = (((p.p625 * locals.var_inv_l_dn4) + (p.p815 * locals.var_inv_w_dn4)) + (p.p1005 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soik1b_dn5 = (((p.p625 * locals.var_inv_l_dn5) + (p.p815 * locals.var_inv_w_dn5)) + (p.p1005 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soik1b_dn6 = (((p.p625 * locals.var_inv_l_dn6) + (p.p815 * locals.var_inv_w_dn6)) + (p.p1005 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soik1b_dn7 = (((p.p625 * locals.var_inv_l_dn7) + (p.p815 * locals.var_inv_w_dn7)) + (p.p1005 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soik1b_dn8 = (((p.p625 * locals.var_inv_l_dn8) + (p.p815 * locals.var_inv_w_dn8)) + (p.p1005 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soik1b_dn9 = (((p.p625 * locals.var_inv_l_dn9) + (p.p815 * locals.var_inv_w_dn9)) + (p.p1005 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soik1b_dn10 = (((p.p625 * locals.var_inv_l_dn10) + (p.p815 * locals.var_inv_w_dn10)) + (p.p1005 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soik1b_dn11 = (((p.p625 * locals.var_inv_l_dn11) + (p.p815 * locals.var_inv_w_dn11)) + (p.p1005 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soik1b_dn12 = (((p.p625 * locals.var_inv_l_dn12) + (p.p815 * locals.var_inv_w_dn12)) + (p.p1005 * locals.var_inv_lw_dn12));

        let assign3170_e5134: f64 = (p.p626 * locals.var_inv_l);
        let assign3170_e5135: f64 = (p.p417 + assign3170_e5134);
        let assign3170_e5138: f64 = (p.p816 * locals.var_inv_w);
        let assign3170_e5139: f64 = (assign3170_e5135 + assign3170_e5138);
        let assign3170_e5142: f64 = (p.p1006 * locals.var_inv_lw);
        let assign3170_e5143: f64 = (assign3170_e5139 + assign3170_e5142);
        locals.var_pparam_b4soik2b = assign3170_e5143;
        locals.var_pparam_b4soik2b_dn3 = (((p.p626 * locals.var_inv_l_dn3) + (p.p816 * locals.var_inv_w_dn3)) + (p.p1006 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soik2b_dn4 = (((p.p626 * locals.var_inv_l_dn4) + (p.p816 * locals.var_inv_w_dn4)) + (p.p1006 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soik2b_dn5 = (((p.p626 * locals.var_inv_l_dn5) + (p.p816 * locals.var_inv_w_dn5)) + (p.p1006 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soik2b_dn6 = (((p.p626 * locals.var_inv_l_dn6) + (p.p816 * locals.var_inv_w_dn6)) + (p.p1006 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soik2b_dn7 = (((p.p626 * locals.var_inv_l_dn7) + (p.p816 * locals.var_inv_w_dn7)) + (p.p1006 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soik2b_dn8 = (((p.p626 * locals.var_inv_l_dn8) + (p.p816 * locals.var_inv_w_dn8)) + (p.p1006 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soik2b_dn9 = (((p.p626 * locals.var_inv_l_dn9) + (p.p816 * locals.var_inv_w_dn9)) + (p.p1006 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soik2b_dn10 = (((p.p626 * locals.var_inv_l_dn10) + (p.p816 * locals.var_inv_w_dn10)) + (p.p1006 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soik2b_dn11 = (((p.p626 * locals.var_inv_l_dn11) + (p.p816 * locals.var_inv_w_dn11)) + (p.p1006 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soik2b_dn12 = (((p.p626 * locals.var_inv_l_dn12) + (p.p816 * locals.var_inv_w_dn12)) + (p.p1006 * locals.var_inv_lw_dn12));

        let assign3180_e5147: f64 = (p.p627 * locals.var_inv_l);
        let assign3180_e5148: f64 = (p.p418 + assign3180_e5147);
        let assign3180_e5151: f64 = (p.p817 * locals.var_inv_w);
        let assign3180_e5152: f64 = (assign3180_e5148 + assign3180_e5151);
        let assign3180_e5155: f64 = (p.p1007 * locals.var_inv_lw);
        let assign3180_e5156: f64 = (assign3180_e5152 + assign3180_e5155);
        locals.var_pparam_b4soidk2b = assign3180_e5156;
        locals.var_pparam_b4soidk2b_dn3 = (((p.p627 * locals.var_inv_l_dn3) + (p.p817 * locals.var_inv_w_dn3)) + (p.p1007 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidk2b_dn4 = (((p.p627 * locals.var_inv_l_dn4) + (p.p817 * locals.var_inv_w_dn4)) + (p.p1007 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidk2b_dn5 = (((p.p627 * locals.var_inv_l_dn5) + (p.p817 * locals.var_inv_w_dn5)) + (p.p1007 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidk2b_dn6 = (((p.p627 * locals.var_inv_l_dn6) + (p.p817 * locals.var_inv_w_dn6)) + (p.p1007 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidk2b_dn7 = (((p.p627 * locals.var_inv_l_dn7) + (p.p817 * locals.var_inv_w_dn7)) + (p.p1007 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidk2b_dn8 = (((p.p627 * locals.var_inv_l_dn8) + (p.p817 * locals.var_inv_w_dn8)) + (p.p1007 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidk2b_dn9 = (((p.p627 * locals.var_inv_l_dn9) + (p.p817 * locals.var_inv_w_dn9)) + (p.p1007 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidk2b_dn10 = (((p.p627 * locals.var_inv_l_dn10) + (p.p817 * locals.var_inv_w_dn10)) + (p.p1007 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidk2b_dn11 = (((p.p627 * locals.var_inv_l_dn11) + (p.p817 * locals.var_inv_w_dn11)) + (p.p1007 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidk2b_dn12 = (((p.p627 * locals.var_inv_l_dn12) + (p.p817 * locals.var_inv_w_dn12)) + (p.p1007 * locals.var_inv_lw_dn12));

        let assign3190_e5160: f64 = (p.p628 * locals.var_inv_l);
        let assign3190_e5161: f64 = (p.p419 + assign3190_e5160);
        let assign3190_e5164: f64 = (p.p818 * locals.var_inv_w);
        let assign3190_e5165: f64 = (assign3190_e5161 + assign3190_e5164);
        let assign3190_e5168: f64 = (p.p1008 * locals.var_inv_lw);
        let assign3190_e5169: f64 = (assign3190_e5165 + assign3190_e5168);
        locals.var_pparam_b4soidvbd0 = assign3190_e5169;
        locals.var_pparam_b4soidvbd0_dn3 = (((p.p628 * locals.var_inv_l_dn3) + (p.p818 * locals.var_inv_w_dn3)) + (p.p1008 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidvbd0_dn4 = (((p.p628 * locals.var_inv_l_dn4) + (p.p818 * locals.var_inv_w_dn4)) + (p.p1008 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidvbd0_dn5 = (((p.p628 * locals.var_inv_l_dn5) + (p.p818 * locals.var_inv_w_dn5)) + (p.p1008 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidvbd0_dn6 = (((p.p628 * locals.var_inv_l_dn6) + (p.p818 * locals.var_inv_w_dn6)) + (p.p1008 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidvbd0_dn7 = (((p.p628 * locals.var_inv_l_dn7) + (p.p818 * locals.var_inv_w_dn7)) + (p.p1008 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidvbd0_dn8 = (((p.p628 * locals.var_inv_l_dn8) + (p.p818 * locals.var_inv_w_dn8)) + (p.p1008 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidvbd0_dn9 = (((p.p628 * locals.var_inv_l_dn9) + (p.p818 * locals.var_inv_w_dn9)) + (p.p1008 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidvbd0_dn10 = (((p.p628 * locals.var_inv_l_dn10) + (p.p818 * locals.var_inv_w_dn10)) + (p.p1008 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidvbd0_dn11 = (((p.p628 * locals.var_inv_l_dn11) + (p.p818 * locals.var_inv_w_dn11)) + (p.p1008 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidvbd0_dn12 = (((p.p628 * locals.var_inv_l_dn12) + (p.p818 * locals.var_inv_w_dn12)) + (p.p1008 * locals.var_inv_lw_dn12));

        let assign3200_e5173: f64 = (p.p629 * locals.var_inv_l);
        let assign3200_e5174: f64 = (p.p420 + assign3200_e5173);
        let assign3200_e5177: f64 = (p.p819 * locals.var_inv_w);
        let assign3200_e5178: f64 = (assign3200_e5174 + assign3200_e5177);
        let assign3200_e5181: f64 = (p.p1009 * locals.var_inv_lw);
        let assign3200_e5182: f64 = (assign3200_e5178 + assign3200_e5181);
        locals.var_pparam_b4soidvbd1 = assign3200_e5182;
        locals.var_pparam_b4soidvbd1_dn3 = (((p.p629 * locals.var_inv_l_dn3) + (p.p819 * locals.var_inv_w_dn3)) + (p.p1009 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidvbd1_dn4 = (((p.p629 * locals.var_inv_l_dn4) + (p.p819 * locals.var_inv_w_dn4)) + (p.p1009 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidvbd1_dn5 = (((p.p629 * locals.var_inv_l_dn5) + (p.p819 * locals.var_inv_w_dn5)) + (p.p1009 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidvbd1_dn6 = (((p.p629 * locals.var_inv_l_dn6) + (p.p819 * locals.var_inv_w_dn6)) + (p.p1009 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidvbd1_dn7 = (((p.p629 * locals.var_inv_l_dn7) + (p.p819 * locals.var_inv_w_dn7)) + (p.p1009 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidvbd1_dn8 = (((p.p629 * locals.var_inv_l_dn8) + (p.p819 * locals.var_inv_w_dn8)) + (p.p1009 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidvbd1_dn9 = (((p.p629 * locals.var_inv_l_dn9) + (p.p819 * locals.var_inv_w_dn9)) + (p.p1009 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidvbd1_dn10 = (((p.p629 * locals.var_inv_l_dn10) + (p.p819 * locals.var_inv_w_dn10)) + (p.p1009 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidvbd1_dn11 = (((p.p629 * locals.var_inv_l_dn11) + (p.p819 * locals.var_inv_w_dn11)) + (p.p1009 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidvbd1_dn12 = (((p.p629 * locals.var_inv_l_dn12) + (p.p819 * locals.var_inv_w_dn12)) + (p.p1009 * locals.var_inv_lw_dn12));

        let assign3210_e5186: f64 = (p.p630 * locals.var_inv_l);
        let assign3210_e5187: f64 = (p.p421 + assign3210_e5186);
        let assign3210_e5190: f64 = (p.p820 * locals.var_inv_w);
        let assign3210_e5191: f64 = (assign3210_e5187 + assign3210_e5190);
        let assign3210_e5194: f64 = (p.p1010 * locals.var_inv_lw);
        let assign3210_e5195: f64 = (assign3210_e5191 + assign3210_e5194);
        locals.var_pparam_b4soimoinfd = assign3210_e5195;
        locals.var_pparam_b4soimoinfd_dn3 = (((p.p630 * locals.var_inv_l_dn3) + (p.p820 * locals.var_inv_w_dn3)) + (p.p1010 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soimoinfd_dn4 = (((p.p630 * locals.var_inv_l_dn4) + (p.p820 * locals.var_inv_w_dn4)) + (p.p1010 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soimoinfd_dn5 = (((p.p630 * locals.var_inv_l_dn5) + (p.p820 * locals.var_inv_w_dn5)) + (p.p1010 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soimoinfd_dn6 = (((p.p630 * locals.var_inv_l_dn6) + (p.p820 * locals.var_inv_w_dn6)) + (p.p1010 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soimoinfd_dn7 = (((p.p630 * locals.var_inv_l_dn7) + (p.p820 * locals.var_inv_w_dn7)) + (p.p1010 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soimoinfd_dn8 = (((p.p630 * locals.var_inv_l_dn8) + (p.p820 * locals.var_inv_w_dn8)) + (p.p1010 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soimoinfd_dn9 = (((p.p630 * locals.var_inv_l_dn9) + (p.p820 * locals.var_inv_w_dn9)) + (p.p1010 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soimoinfd_dn10 = (((p.p630 * locals.var_inv_l_dn10) + (p.p820 * locals.var_inv_w_dn10)) + (p.p1010 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soimoinfd_dn11 = (((p.p630 * locals.var_inv_l_dn11) + (p.p820 * locals.var_inv_w_dn11)) + (p.p1010 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soimoinfd_dn12 = (((p.p630 * locals.var_inv_l_dn12) + (p.p820 * locals.var_inv_w_dn12)) + (p.p1010 * locals.var_inv_lw_dn12));

        let assign3220_e5199: f64 = (p.p631 * locals.var_inv_l);
        let assign3220_e5200: f64 = (p.p411 + assign3220_e5199);
        let assign3220_e5203: f64 = (p.p821 * locals.var_inv_w);
        let assign3220_e5204: f64 = (assign3220_e5200 + assign3220_e5203);
        let assign3220_e5207: f64 = (p.p1011 * locals.var_inv_lw);
        let assign3220_e5208: f64 = (assign3220_e5204 + assign3220_e5207);
        locals.var_pparam_b4soivbs0pd = assign3220_e5208;

        let assign3230_e5212: f64 = (p.p632 * locals.var_inv_l);
        let assign3230_e5213: f64 = (p.p412 + assign3230_e5212);
        let assign3230_e5216: f64 = (p.p822 * locals.var_inv_w);
        let assign3230_e5217: f64 = (assign3230_e5213 + assign3230_e5216);
        let assign3230_e5220: f64 = (p.p1012 * locals.var_inv_lw);
        let assign3230_e5221: f64 = (assign3230_e5217 + assign3230_e5220);
        locals.var_pparam_b4soivbs0fd = assign3230_e5221;

        let assign3240_e5225: f64 = (p.p611 * locals.var_inv_l);
        let assign3240_e5226: f64 = (p.p353 + assign3240_e5225);
        let assign3240_e5229: f64 = (p.p801 * locals.var_inv_w);
        let assign3240_e5230: f64 = (assign3240_e5226 + assign3240_e5229);
        let assign3240_e5233: f64 = (p.p991 * locals.var_inv_lw);
        let assign3240_e5234: f64 = (assign3240_e5230 + assign3240_e5233);
        locals.var_pparam_b4soivsdfb = assign3240_e5234;
        locals.var_pparam_b4soivsdfb_dn3 = (((p.p611 * locals.var_inv_l_dn3) + (p.p801 * locals.var_inv_w_dn3)) + (p.p991 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivsdfb_dn4 = (((p.p611 * locals.var_inv_l_dn4) + (p.p801 * locals.var_inv_w_dn4)) + (p.p991 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivsdfb_dn5 = (((p.p611 * locals.var_inv_l_dn5) + (p.p801 * locals.var_inv_w_dn5)) + (p.p991 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivsdfb_dn6 = (((p.p611 * locals.var_inv_l_dn6) + (p.p801 * locals.var_inv_w_dn6)) + (p.p991 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivsdfb_dn7 = (((p.p611 * locals.var_inv_l_dn7) + (p.p801 * locals.var_inv_w_dn7)) + (p.p991 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivsdfb_dn8 = (((p.p611 * locals.var_inv_l_dn8) + (p.p801 * locals.var_inv_w_dn8)) + (p.p991 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivsdfb_dn9 = (((p.p611 * locals.var_inv_l_dn9) + (p.p801 * locals.var_inv_w_dn9)) + (p.p991 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivsdfb_dn10 = (((p.p611 * locals.var_inv_l_dn10) + (p.p801 * locals.var_inv_w_dn10)) + (p.p991 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivsdfb_dn11 = (((p.p611 * locals.var_inv_l_dn11) + (p.p801 * locals.var_inv_w_dn11)) + (p.p991 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivsdfb_dn12 = (((p.p611 * locals.var_inv_l_dn12) + (p.p801 * locals.var_inv_w_dn12)) + (p.p991 * locals.var_inv_lw_dn12));

        let assign3250_e5238: f64 = (p.p612 * locals.var_inv_l);
        let assign3250_e5239: f64 = (p.p354 + assign3250_e5238);
        let assign3250_e5242: f64 = (p.p802 * locals.var_inv_w);
        let assign3250_e5243: f64 = (assign3250_e5239 + assign3250_e5242);
        let assign3250_e5246: f64 = (p.p992 * locals.var_inv_lw);
        let assign3250_e5247: f64 = (assign3250_e5243 + assign3250_e5246);
        locals.var_pparam_b4soivsdth = assign3250_e5247;
        locals.var_pparam_b4soivsdth_dn3 = (((p.p612 * locals.var_inv_l_dn3) + (p.p802 * locals.var_inv_w_dn3)) + (p.p992 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivsdth_dn4 = (((p.p612 * locals.var_inv_l_dn4) + (p.p802 * locals.var_inv_w_dn4)) + (p.p992 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivsdth_dn5 = (((p.p612 * locals.var_inv_l_dn5) + (p.p802 * locals.var_inv_w_dn5)) + (p.p992 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivsdth_dn6 = (((p.p612 * locals.var_inv_l_dn6) + (p.p802 * locals.var_inv_w_dn6)) + (p.p992 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivsdth_dn7 = (((p.p612 * locals.var_inv_l_dn7) + (p.p802 * locals.var_inv_w_dn7)) + (p.p992 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivsdth_dn8 = (((p.p612 * locals.var_inv_l_dn8) + (p.p802 * locals.var_inv_w_dn8)) + (p.p992 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivsdth_dn9 = (((p.p612 * locals.var_inv_l_dn9) + (p.p802 * locals.var_inv_w_dn9)) + (p.p992 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivsdth_dn10 = (((p.p612 * locals.var_inv_l_dn10) + (p.p802 * locals.var_inv_w_dn10)) + (p.p992 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivsdth_dn11 = (((p.p612 * locals.var_inv_l_dn11) + (p.p802 * locals.var_inv_w_dn11)) + (p.p992 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivsdth_dn12 = (((p.p612 * locals.var_inv_l_dn12) + (p.p802 * locals.var_inv_w_dn12)) + (p.p992 * locals.var_inv_lw_dn12));

        let assign3260_e5251: f64 = (p.p613 * locals.var_inv_l);
        let assign3260_e5252: f64 = (p.p370 + assign3260_e5251);
        let assign3260_e5255: f64 = (p.p803 * locals.var_inv_w);
        let assign3260_e5256: f64 = (assign3260_e5252 + assign3260_e5255);
        let assign3260_e5259: f64 = (p.p993 * locals.var_inv_lw);
        let assign3260_e5260: f64 = (assign3260_e5256 + assign3260_e5259);
        locals.var_pparam_b4soidelvt = assign3260_e5260;
        locals.var_pparam_b4soidelvt_dn3 = (((p.p613 * locals.var_inv_l_dn3) + (p.p803 * locals.var_inv_w_dn3)) + (p.p993 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidelvt_dn4 = (((p.p613 * locals.var_inv_l_dn4) + (p.p803 * locals.var_inv_w_dn4)) + (p.p993 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidelvt_dn5 = (((p.p613 * locals.var_inv_l_dn5) + (p.p803 * locals.var_inv_w_dn5)) + (p.p993 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidelvt_dn6 = (((p.p613 * locals.var_inv_l_dn6) + (p.p803 * locals.var_inv_w_dn6)) + (p.p993 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidelvt_dn7 = (((p.p613 * locals.var_inv_l_dn7) + (p.p803 * locals.var_inv_w_dn7)) + (p.p993 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidelvt_dn8 = (((p.p613 * locals.var_inv_l_dn8) + (p.p803 * locals.var_inv_w_dn8)) + (p.p993 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidelvt_dn9 = (((p.p613 * locals.var_inv_l_dn9) + (p.p803 * locals.var_inv_w_dn9)) + (p.p993 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidelvt_dn10 = (((p.p613 * locals.var_inv_l_dn10) + (p.p803 * locals.var_inv_w_dn10)) + (p.p993 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidelvt_dn11 = (((p.p613 * locals.var_inv_l_dn11) + (p.p803 * locals.var_inv_w_dn11)) + (p.p993 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidelvt_dn12 = (((p.p613 * locals.var_inv_l_dn12) + (p.p803 * locals.var_inv_w_dn12)) + (p.p993 * locals.var_inv_lw_dn12));

        let assign3270_e5264: f64 = (p.p614 * locals.var_inv_l);
        let assign3270_e5265: f64 = (p.p366 + assign3270_e5264);
        let assign3270_e5268: f64 = (p.p804 * locals.var_inv_w);
        let assign3270_e5269: f64 = (assign3270_e5265 + assign3270_e5268);
        let assign3270_e5272: f64 = (p.p994 * locals.var_inv_lw);
        let assign3270_e5273: f64 = (assign3270_e5269 + assign3270_e5272);
        locals.var_pparam_b4soiacde = assign3270_e5273;
        locals.var_pparam_b4soiacde_dn3 = (((p.p614 * locals.var_inv_l_dn3) + (p.p804 * locals.var_inv_w_dn3)) + (p.p994 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiacde_dn4 = (((p.p614 * locals.var_inv_l_dn4) + (p.p804 * locals.var_inv_w_dn4)) + (p.p994 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiacde_dn5 = (((p.p614 * locals.var_inv_l_dn5) + (p.p804 * locals.var_inv_w_dn5)) + (p.p994 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiacde_dn6 = (((p.p614 * locals.var_inv_l_dn6) + (p.p804 * locals.var_inv_w_dn6)) + (p.p994 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiacde_dn7 = (((p.p614 * locals.var_inv_l_dn7) + (p.p804 * locals.var_inv_w_dn7)) + (p.p994 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiacde_dn8 = (((p.p614 * locals.var_inv_l_dn8) + (p.p804 * locals.var_inv_w_dn8)) + (p.p994 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiacde_dn9 = (((p.p614 * locals.var_inv_l_dn9) + (p.p804 * locals.var_inv_w_dn9)) + (p.p994 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiacde_dn10 = (((p.p614 * locals.var_inv_l_dn10) + (p.p804 * locals.var_inv_w_dn10)) + (p.p994 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiacde_dn11 = (((p.p614 * locals.var_inv_l_dn11) + (p.p804 * locals.var_inv_w_dn11)) + (p.p994 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiacde_dn12 = (((p.p614 * locals.var_inv_l_dn12) + (p.p804 * locals.var_inv_w_dn12)) + (p.p994 * locals.var_inv_lw_dn12));

        let assign3280_e5277: f64 = (locals.var_pparam_b4soinpeak / 2e16);
        let assign3280_e5279: f64 = (-0.25);
        let assign3280_e5280: f64 = (assign3280_e5277).powf(assign3280_e5279);
        let assign3280_e5281: f64 = (locals.var_pparam_b4soiacde * assign3280_e5280);
        locals.var_pparam_b4soiacde = assign3280_e5281;
        locals.var_pparam_b4soiacde_dn3 = ((locals.var_pparam_b4soiacde_dn3 * assign3280_e5280) + (locals.var_pparam_b4soiacde * if 0.0 == 0.0 && ((assign3280_e5279) as f64).is_finite() && ((assign3280_e5279) as f64).fract() == 0.0 { if assign3280_e5279 == 0.0 { 0.0 } else { (assign3280_e5279 * ((assign3280_e5277).powf(assign3280_e5279 - 1.0) * (locals.var_pparam_b4soinpeak_dn3 / 2e16))) } } else { (assign3280_e5280 * (assign3280_e5279 * ((locals.var_pparam_b4soinpeak_dn3 / 2e16) / assign3280_e5277))) }));
        locals.var_pparam_b4soiacde_dn4 = ((locals.var_pparam_b4soiacde_dn4 * assign3280_e5280) + (locals.var_pparam_b4soiacde * if 0.0 == 0.0 && ((assign3280_e5279) as f64).is_finite() && ((assign3280_e5279) as f64).fract() == 0.0 { if assign3280_e5279 == 0.0 { 0.0 } else { (assign3280_e5279 * ((assign3280_e5277).powf(assign3280_e5279 - 1.0) * (locals.var_pparam_b4soinpeak_dn4 / 2e16))) } } else { (assign3280_e5280 * (assign3280_e5279 * ((locals.var_pparam_b4soinpeak_dn4 / 2e16) / assign3280_e5277))) }));
        locals.var_pparam_b4soiacde_dn5 = ((locals.var_pparam_b4soiacde_dn5 * assign3280_e5280) + (locals.var_pparam_b4soiacde * if 0.0 == 0.0 && ((assign3280_e5279) as f64).is_finite() && ((assign3280_e5279) as f64).fract() == 0.0 { if assign3280_e5279 == 0.0 { 0.0 } else { (assign3280_e5279 * ((assign3280_e5277).powf(assign3280_e5279 - 1.0) * (locals.var_pparam_b4soinpeak_dn5 / 2e16))) } } else { (assign3280_e5280 * (assign3280_e5279 * ((locals.var_pparam_b4soinpeak_dn5 / 2e16) / assign3280_e5277))) }));
        locals.var_pparam_b4soiacde_dn6 = ((locals.var_pparam_b4soiacde_dn6 * assign3280_e5280) + (locals.var_pparam_b4soiacde * if 0.0 == 0.0 && ((assign3280_e5279) as f64).is_finite() && ((assign3280_e5279) as f64).fract() == 0.0 { if assign3280_e5279 == 0.0 { 0.0 } else { (assign3280_e5279 * ((assign3280_e5277).powf(assign3280_e5279 - 1.0) * (locals.var_pparam_b4soinpeak_dn6 / 2e16))) } } else { (assign3280_e5280 * (assign3280_e5279 * ((locals.var_pparam_b4soinpeak_dn6 / 2e16) / assign3280_e5277))) }));
        locals.var_pparam_b4soiacde_dn7 = ((locals.var_pparam_b4soiacde_dn7 * assign3280_e5280) + (locals.var_pparam_b4soiacde * if 0.0 == 0.0 && ((assign3280_e5279) as f64).is_finite() && ((assign3280_e5279) as f64).fract() == 0.0 { if assign3280_e5279 == 0.0 { 0.0 } else { (assign3280_e5279 * ((assign3280_e5277).powf(assign3280_e5279 - 1.0) * (locals.var_pparam_b4soinpeak_dn7 / 2e16))) } } else { (assign3280_e5280 * (assign3280_e5279 * ((locals.var_pparam_b4soinpeak_dn7 / 2e16) / assign3280_e5277))) }));
        locals.var_pparam_b4soiacde_dn8 = ((locals.var_pparam_b4soiacde_dn8 * assign3280_e5280) + (locals.var_pparam_b4soiacde * if 0.0 == 0.0 && ((assign3280_e5279) as f64).is_finite() && ((assign3280_e5279) as f64).fract() == 0.0 { if assign3280_e5279 == 0.0 { 0.0 } else { (assign3280_e5279 * ((assign3280_e5277).powf(assign3280_e5279 - 1.0) * (locals.var_pparam_b4soinpeak_dn8 / 2e16))) } } else { (assign3280_e5280 * (assign3280_e5279 * ((locals.var_pparam_b4soinpeak_dn8 / 2e16) / assign3280_e5277))) }));
        locals.var_pparam_b4soiacde_dn9 = ((locals.var_pparam_b4soiacde_dn9 * assign3280_e5280) + (locals.var_pparam_b4soiacde * if 0.0 == 0.0 && ((assign3280_e5279) as f64).is_finite() && ((assign3280_e5279) as f64).fract() == 0.0 { if assign3280_e5279 == 0.0 { 0.0 } else { (assign3280_e5279 * ((assign3280_e5277).powf(assign3280_e5279 - 1.0) * (locals.var_pparam_b4soinpeak_dn9 / 2e16))) } } else { (assign3280_e5280 * (assign3280_e5279 * ((locals.var_pparam_b4soinpeak_dn9 / 2e16) / assign3280_e5277))) }));
        locals.var_pparam_b4soiacde_dn10 = ((locals.var_pparam_b4soiacde_dn10 * assign3280_e5280) + (locals.var_pparam_b4soiacde * if 0.0 == 0.0 && ((assign3280_e5279) as f64).is_finite() && ((assign3280_e5279) as f64).fract() == 0.0 { if assign3280_e5279 == 0.0 { 0.0 } else { (assign3280_e5279 * ((assign3280_e5277).powf(assign3280_e5279 - 1.0) * (locals.var_pparam_b4soinpeak_dn10 / 2e16))) } } else { (assign3280_e5280 * (assign3280_e5279 * ((locals.var_pparam_b4soinpeak_dn10 / 2e16) / assign3280_e5277))) }));
        locals.var_pparam_b4soiacde_dn11 = ((locals.var_pparam_b4soiacde_dn11 * assign3280_e5280) + (locals.var_pparam_b4soiacde * if 0.0 == 0.0 && ((assign3280_e5279) as f64).is_finite() && ((assign3280_e5279) as f64).fract() == 0.0 { if assign3280_e5279 == 0.0 { 0.0 } else { (assign3280_e5279 * ((assign3280_e5277).powf(assign3280_e5279 - 1.0) * (locals.var_pparam_b4soinpeak_dn11 / 2e16))) } } else { (assign3280_e5280 * (assign3280_e5279 * ((locals.var_pparam_b4soinpeak_dn11 / 2e16) / assign3280_e5277))) }));
        locals.var_pparam_b4soiacde_dn12 = ((locals.var_pparam_b4soiacde_dn12 * assign3280_e5280) + (locals.var_pparam_b4soiacde * if 0.0 == 0.0 && ((assign3280_e5279) as f64).is_finite() && ((assign3280_e5279) as f64).fract() == 0.0 { if assign3280_e5279 == 0.0 { 0.0 } else { (assign3280_e5279 * ((assign3280_e5277).powf(assign3280_e5279 - 1.0) * (locals.var_pparam_b4soinpeak_dn12 / 2e16))) } } else { (assign3280_e5280 * (assign3280_e5279 * ((locals.var_pparam_b4soinpeak_dn12 / 2e16) / assign3280_e5277))) }));

        let assign3290_e5285: f64 = (p.p615 * locals.var_inv_l);
        let assign3290_e5286: f64 = (p.p367 + assign3290_e5285);
        let assign3290_e5289: f64 = (p.p805 * locals.var_inv_w);
        let assign3290_e5290: f64 = (assign3290_e5286 + assign3290_e5289);
        let assign3290_e5293: f64 = (p.p995 * locals.var_inv_lw);
        let assign3290_e5294: f64 = (assign3290_e5290 + assign3290_e5293);
        locals.var_pparam_b4soimoin = assign3290_e5294;
        locals.var_pparam_b4soimoin_dn3 = (((p.p615 * locals.var_inv_l_dn3) + (p.p805 * locals.var_inv_w_dn3)) + (p.p995 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soimoin_dn4 = (((p.p615 * locals.var_inv_l_dn4) + (p.p805 * locals.var_inv_w_dn4)) + (p.p995 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soimoin_dn5 = (((p.p615 * locals.var_inv_l_dn5) + (p.p805 * locals.var_inv_w_dn5)) + (p.p995 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soimoin_dn6 = (((p.p615 * locals.var_inv_l_dn6) + (p.p805 * locals.var_inv_w_dn6)) + (p.p995 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soimoin_dn7 = (((p.p615 * locals.var_inv_l_dn7) + (p.p805 * locals.var_inv_w_dn7)) + (p.p995 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soimoin_dn8 = (((p.p615 * locals.var_inv_l_dn8) + (p.p805 * locals.var_inv_w_dn8)) + (p.p995 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soimoin_dn9 = (((p.p615 * locals.var_inv_l_dn9) + (p.p805 * locals.var_inv_w_dn9)) + (p.p995 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soimoin_dn10 = (((p.p615 * locals.var_inv_l_dn10) + (p.p805 * locals.var_inv_w_dn10)) + (p.p995 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soimoin_dn11 = (((p.p615 * locals.var_inv_l_dn11) + (p.p805 * locals.var_inv_w_dn11)) + (p.p995 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soimoin_dn12 = (((p.p615 * locals.var_inv_l_dn12) + (p.p805 * locals.var_inv_w_dn12)) + (p.p995 * locals.var_inv_lw_dn12));

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3300_e5298: f64 = (p.p616 * locals.var_inv_l);
        let assign3300_e5299: f64 = (p.p368 + assign3300_e5298);
        let assign3300_e5302: f64 = (p.p806 * locals.var_inv_w);
        let assign3300_e5303: f64 = (assign3300_e5299 + assign3300_e5302);
        let assign3300_e5306: f64 = (p.p996 * locals.var_inv_lw);
        let assign3300_e5307: f64 = (assign3300_e5303 + assign3300_e5306);
        locals.var_pparam_b4soinoff = assign3300_e5307;
        locals.var_pparam_b4soinoff_dn3 = (((p.p616 * locals.var_inv_l_dn3) + (p.p806 * locals.var_inv_w_dn3)) + (p.p996 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soinoff_dn4 = (((p.p616 * locals.var_inv_l_dn4) + (p.p806 * locals.var_inv_w_dn4)) + (p.p996 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soinoff_dn5 = (((p.p616 * locals.var_inv_l_dn5) + (p.p806 * locals.var_inv_w_dn5)) + (p.p996 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soinoff_dn6 = (((p.p616 * locals.var_inv_l_dn6) + (p.p806 * locals.var_inv_w_dn6)) + (p.p996 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soinoff_dn7 = (((p.p616 * locals.var_inv_l_dn7) + (p.p806 * locals.var_inv_w_dn7)) + (p.p996 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soinoff_dn8 = (((p.p616 * locals.var_inv_l_dn8) + (p.p806 * locals.var_inv_w_dn8)) + (p.p996 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soinoff_dn9 = (((p.p616 * locals.var_inv_l_dn9) + (p.p806 * locals.var_inv_w_dn9)) + (p.p996 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soinoff_dn10 = (((p.p616 * locals.var_inv_l_dn10) + (p.p806 * locals.var_inv_w_dn10)) + (p.p996 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soinoff_dn11 = (((p.p616 * locals.var_inv_l_dn11) + (p.p806 * locals.var_inv_w_dn11)) + (p.p996 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soinoff_dn12 = (((p.p616 * locals.var_inv_l_dn12) + (p.p806 * locals.var_inv_w_dn12)) + (p.p996 * locals.var_inv_lw_dn12));

        let assign3310_e5311: f64 = (p.p617 * locals.var_inv_l);
        let assign3310_e5312: f64 = (p.p369 + assign3310_e5311);
        let assign3310_e5315: f64 = (p.p807 * locals.var_inv_w);
        let assign3310_e5316: f64 = (assign3310_e5312 + assign3310_e5315);
        let assign3310_e5319: f64 = (p.p997 * locals.var_inv_lw);
        let assign3310_e5320: f64 = (assign3310_e5316 + assign3310_e5319);
        locals.var_pparam_b4soinoff2 = assign3310_e5320;
        locals.var_pparam_b4soinoff2_dn3 = (((p.p617 * locals.var_inv_l_dn3) + (p.p807 * locals.var_inv_w_dn3)) + (p.p997 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soinoff2_dn4 = (((p.p617 * locals.var_inv_l_dn4) + (p.p807 * locals.var_inv_w_dn4)) + (p.p997 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soinoff2_dn5 = (((p.p617 * locals.var_inv_l_dn5) + (p.p807 * locals.var_inv_w_dn5)) + (p.p997 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soinoff2_dn6 = (((p.p617 * locals.var_inv_l_dn6) + (p.p807 * locals.var_inv_w_dn6)) + (p.p997 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soinoff2_dn7 = (((p.p617 * locals.var_inv_l_dn7) + (p.p807 * locals.var_inv_w_dn7)) + (p.p997 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soinoff2_dn8 = (((p.p617 * locals.var_inv_l_dn8) + (p.p807 * locals.var_inv_w_dn8)) + (p.p997 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soinoff2_dn9 = (((p.p617 * locals.var_inv_l_dn9) + (p.p807 * locals.var_inv_w_dn9)) + (p.p997 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soinoff2_dn10 = (((p.p617 * locals.var_inv_l_dn10) + (p.p807 * locals.var_inv_w_dn10)) + (p.p997 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soinoff2_dn11 = (((p.p617 * locals.var_inv_l_dn11) + (p.p807 * locals.var_inv_w_dn11)) + (p.p997 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soinoff2_dn12 = (((p.p617 * locals.var_inv_l_dn12) + (p.p807 * locals.var_inv_w_dn12)) + (p.p997 * locals.var_inv_lw_dn12));

        let assign3320_e5324: f64 = (p.p259 * locals.var_inv_l);
        let assign3320_e5325: f64 = (p.p258 + assign3320_e5324);
        let assign3320_e5328: f64 = (p.p260 * locals.var_inv_w);
        let assign3320_e5329: f64 = (assign3320_e5325 + assign3320_e5328);
        let assign3320_e5332: f64 = (p.p261 * locals.var_inv_lw);
        let assign3320_e5333: f64 = (assign3320_e5329 + assign3320_e5332);
        locals.var_pparam_b4soidvtp0 = assign3320_e5333;
        locals.var_pparam_b4soidvtp0_dn3 = (((p.p259 * locals.var_inv_l_dn3) + (p.p260 * locals.var_inv_w_dn3)) + (p.p261 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidvtp0_dn4 = (((p.p259 * locals.var_inv_l_dn4) + (p.p260 * locals.var_inv_w_dn4)) + (p.p261 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidvtp0_dn5 = (((p.p259 * locals.var_inv_l_dn5) + (p.p260 * locals.var_inv_w_dn5)) + (p.p261 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidvtp0_dn6 = (((p.p259 * locals.var_inv_l_dn6) + (p.p260 * locals.var_inv_w_dn6)) + (p.p261 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidvtp0_dn7 = (((p.p259 * locals.var_inv_l_dn7) + (p.p260 * locals.var_inv_w_dn7)) + (p.p261 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidvtp0_dn8 = (((p.p259 * locals.var_inv_l_dn8) + (p.p260 * locals.var_inv_w_dn8)) + (p.p261 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidvtp0_dn9 = (((p.p259 * locals.var_inv_l_dn9) + (p.p260 * locals.var_inv_w_dn9)) + (p.p261 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidvtp0_dn10 = (((p.p259 * locals.var_inv_l_dn10) + (p.p260 * locals.var_inv_w_dn10)) + (p.p261 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidvtp0_dn11 = (((p.p259 * locals.var_inv_l_dn11) + (p.p260 * locals.var_inv_w_dn11)) + (p.p261 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidvtp0_dn12 = (((p.p259 * locals.var_inv_l_dn12) + (p.p260 * locals.var_inv_w_dn12)) + (p.p261 * locals.var_inv_lw_dn12));

        let assign3330_e5337: f64 = (p.p263 * locals.var_inv_l);
        let assign3330_e5338: f64 = (p.p262 + assign3330_e5337);
        let assign3330_e5341: f64 = (p.p264 * locals.var_inv_w);
        let assign3330_e5342: f64 = (assign3330_e5338 + assign3330_e5341);
        let assign3330_e5345: f64 = (p.p265 * locals.var_inv_lw);
        let assign3330_e5346: f64 = (assign3330_e5342 + assign3330_e5345);
        locals.var_pparam_b4soidvtp1 = assign3330_e5346;
        locals.var_pparam_b4soidvtp1_dn3 = (((p.p263 * locals.var_inv_l_dn3) + (p.p264 * locals.var_inv_w_dn3)) + (p.p265 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidvtp1_dn4 = (((p.p263 * locals.var_inv_l_dn4) + (p.p264 * locals.var_inv_w_dn4)) + (p.p265 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidvtp1_dn5 = (((p.p263 * locals.var_inv_l_dn5) + (p.p264 * locals.var_inv_w_dn5)) + (p.p265 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidvtp1_dn6 = (((p.p263 * locals.var_inv_l_dn6) + (p.p264 * locals.var_inv_w_dn6)) + (p.p265 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidvtp1_dn7 = (((p.p263 * locals.var_inv_l_dn7) + (p.p264 * locals.var_inv_w_dn7)) + (p.p265 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidvtp1_dn8 = (((p.p263 * locals.var_inv_l_dn8) + (p.p264 * locals.var_inv_w_dn8)) + (p.p265 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidvtp1_dn9 = (((p.p263 * locals.var_inv_l_dn9) + (p.p264 * locals.var_inv_w_dn9)) + (p.p265 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidvtp1_dn10 = (((p.p263 * locals.var_inv_l_dn10) + (p.p264 * locals.var_inv_w_dn10)) + (p.p265 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidvtp1_dn11 = (((p.p263 * locals.var_inv_l_dn11) + (p.p264 * locals.var_inv_w_dn11)) + (p.p265 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidvtp1_dn12 = (((p.p263 * locals.var_inv_l_dn12) + (p.p264 * locals.var_inv_w_dn12)) + (p.p265 * locals.var_inv_lw_dn12));

        let assign3340_e5350: f64 = (p.p267 * locals.var_inv_l);
        let assign3340_e5351: f64 = (p.p266 + assign3340_e5350);
        let assign3340_e5354: f64 = (p.p268 * locals.var_inv_w);
        let assign3340_e5355: f64 = (assign3340_e5351 + assign3340_e5354);
        let assign3340_e5358: f64 = (p.p269 * locals.var_inv_lw);
        let assign3340_e5359: f64 = (assign3340_e5355 + assign3340_e5358);
        locals.var_pparam_b4soidvtp2 = assign3340_e5359;
        locals.var_pparam_b4soidvtp2_dn3 = (((p.p267 * locals.var_inv_l_dn3) + (p.p268 * locals.var_inv_w_dn3)) + (p.p269 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidvtp2_dn4 = (((p.p267 * locals.var_inv_l_dn4) + (p.p268 * locals.var_inv_w_dn4)) + (p.p269 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidvtp2_dn5 = (((p.p267 * locals.var_inv_l_dn5) + (p.p268 * locals.var_inv_w_dn5)) + (p.p269 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidvtp2_dn6 = (((p.p267 * locals.var_inv_l_dn6) + (p.p268 * locals.var_inv_w_dn6)) + (p.p269 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidvtp2_dn7 = (((p.p267 * locals.var_inv_l_dn7) + (p.p268 * locals.var_inv_w_dn7)) + (p.p269 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidvtp2_dn8 = (((p.p267 * locals.var_inv_l_dn8) + (p.p268 * locals.var_inv_w_dn8)) + (p.p269 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidvtp2_dn9 = (((p.p267 * locals.var_inv_l_dn9) + (p.p268 * locals.var_inv_w_dn9)) + (p.p269 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidvtp2_dn10 = (((p.p267 * locals.var_inv_l_dn10) + (p.p268 * locals.var_inv_w_dn10)) + (p.p269 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidvtp2_dn11 = (((p.p267 * locals.var_inv_l_dn11) + (p.p268 * locals.var_inv_w_dn11)) + (p.p269 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidvtp2_dn12 = (((p.p267 * locals.var_inv_l_dn12) + (p.p268 * locals.var_inv_w_dn12)) + (p.p269 * locals.var_inv_lw_dn12));

        let assign3350_e5363: f64 = (p.p271 * locals.var_inv_l);
        let assign3350_e5364: f64 = (p.p270 + assign3350_e5363);
        let assign3350_e5367: f64 = (p.p272 * locals.var_inv_w);
        let assign3350_e5368: f64 = (assign3350_e5364 + assign3350_e5367);
        let assign3350_e5371: f64 = (p.p273 * locals.var_inv_lw);
        let assign3350_e5372: f64 = (assign3350_e5368 + assign3350_e5371);
        locals.var_pparam_b4soidvtp3 = assign3350_e5372;
        locals.var_pparam_b4soidvtp3_dn3 = (((p.p271 * locals.var_inv_l_dn3) + (p.p272 * locals.var_inv_w_dn3)) + (p.p273 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidvtp3_dn4 = (((p.p271 * locals.var_inv_l_dn4) + (p.p272 * locals.var_inv_w_dn4)) + (p.p273 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidvtp3_dn5 = (((p.p271 * locals.var_inv_l_dn5) + (p.p272 * locals.var_inv_w_dn5)) + (p.p273 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidvtp3_dn6 = (((p.p271 * locals.var_inv_l_dn6) + (p.p272 * locals.var_inv_w_dn6)) + (p.p273 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidvtp3_dn7 = (((p.p271 * locals.var_inv_l_dn7) + (p.p272 * locals.var_inv_w_dn7)) + (p.p273 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidvtp3_dn8 = (((p.p271 * locals.var_inv_l_dn8) + (p.p272 * locals.var_inv_w_dn8)) + (p.p273 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidvtp3_dn9 = (((p.p271 * locals.var_inv_l_dn9) + (p.p272 * locals.var_inv_w_dn9)) + (p.p273 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidvtp3_dn10 = (((p.p271 * locals.var_inv_l_dn10) + (p.p272 * locals.var_inv_w_dn10)) + (p.p273 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidvtp3_dn11 = (((p.p271 * locals.var_inv_l_dn11) + (p.p272 * locals.var_inv_w_dn11)) + (p.p273 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidvtp3_dn12 = (((p.p271 * locals.var_inv_l_dn12) + (p.p272 * locals.var_inv_w_dn12)) + (p.p273 * locals.var_inv_lw_dn12));

        let assign3360_e5376: f64 = (p.p275 * locals.var_inv_l);
        let assign3360_e5377: f64 = (p.p274 + assign3360_e5376);
        let assign3360_e5380: f64 = (p.p276 * locals.var_inv_w);
        let assign3360_e5381: f64 = (assign3360_e5377 + assign3360_e5380);
        let assign3360_e5384: f64 = (p.p277 * locals.var_inv_lw);
        let assign3360_e5385: f64 = (assign3360_e5381 + assign3360_e5384);
        locals.var_pparam_b4soidvtp4 = assign3360_e5385;
        locals.var_pparam_b4soidvtp4_dn3 = (((p.p275 * locals.var_inv_l_dn3) + (p.p276 * locals.var_inv_w_dn3)) + (p.p277 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soidvtp4_dn4 = (((p.p275 * locals.var_inv_l_dn4) + (p.p276 * locals.var_inv_w_dn4)) + (p.p277 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soidvtp4_dn5 = (((p.p275 * locals.var_inv_l_dn5) + (p.p276 * locals.var_inv_w_dn5)) + (p.p277 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soidvtp4_dn6 = (((p.p275 * locals.var_inv_l_dn6) + (p.p276 * locals.var_inv_w_dn6)) + (p.p277 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soidvtp4_dn7 = (((p.p275 * locals.var_inv_l_dn7) + (p.p276 * locals.var_inv_w_dn7)) + (p.p277 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soidvtp4_dn8 = (((p.p275 * locals.var_inv_l_dn8) + (p.p276 * locals.var_inv_w_dn8)) + (p.p277 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soidvtp4_dn9 = (((p.p275 * locals.var_inv_l_dn9) + (p.p276 * locals.var_inv_w_dn9)) + (p.p277 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soidvtp4_dn10 = (((p.p275 * locals.var_inv_l_dn10) + (p.p276 * locals.var_inv_w_dn10)) + (p.p277 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soidvtp4_dn11 = (((p.p275 * locals.var_inv_l_dn11) + (p.p276 * locals.var_inv_w_dn11)) + (p.p277 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soidvtp4_dn12 = (((p.p275 * locals.var_inv_l_dn12) + (p.p276 * locals.var_inv_w_dn12)) + (p.p277 * locals.var_inv_lw_dn12));

        let assign3370_e5389: f64 = (p.p279 * locals.var_inv_l);
        let assign3370_e5390: f64 = (p.p278 + assign3370_e5389);
        let assign3370_e5393: f64 = (p.p280 * locals.var_inv_w);
        let assign3370_e5394: f64 = (assign3370_e5390 + assign3370_e5393);
        let assign3370_e5397: f64 = (p.p281 * locals.var_inv_lw);
        let assign3370_e5398: f64 = (assign3370_e5394 + assign3370_e5397);
        locals.var_pparam_b4soiminv = assign3370_e5398;
        locals.var_pparam_b4soiminv_dn3 = (((p.p279 * locals.var_inv_l_dn3) + (p.p280 * locals.var_inv_w_dn3)) + (p.p281 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiminv_dn4 = (((p.p279 * locals.var_inv_l_dn4) + (p.p280 * locals.var_inv_w_dn4)) + (p.p281 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiminv_dn5 = (((p.p279 * locals.var_inv_l_dn5) + (p.p280 * locals.var_inv_w_dn5)) + (p.p281 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiminv_dn6 = (((p.p279 * locals.var_inv_l_dn6) + (p.p280 * locals.var_inv_w_dn6)) + (p.p281 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiminv_dn7 = (((p.p279 * locals.var_inv_l_dn7) + (p.p280 * locals.var_inv_w_dn7)) + (p.p281 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiminv_dn8 = (((p.p279 * locals.var_inv_l_dn8) + (p.p280 * locals.var_inv_w_dn8)) + (p.p281 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiminv_dn9 = (((p.p279 * locals.var_inv_l_dn9) + (p.p280 * locals.var_inv_w_dn9)) + (p.p281 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiminv_dn10 = (((p.p279 * locals.var_inv_l_dn10) + (p.p280 * locals.var_inv_w_dn10)) + (p.p281 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiminv_dn11 = (((p.p279 * locals.var_inv_l_dn11) + (p.p280 * locals.var_inv_w_dn11)) + (p.p281 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiminv_dn12 = (((p.p279 * locals.var_inv_l_dn12) + (p.p280 * locals.var_inv_w_dn12)) + (p.p281 * locals.var_inv_lw_dn12));

        let assign3380_e5402: f64 = (p.p436 * locals.var_inv_l);
        let assign3380_e5403: f64 = (p.p435 + assign3380_e5402);
        let assign3380_e5406: f64 = (p.p437 * locals.var_inv_w);
        let assign3380_e5407: f64 = (assign3380_e5403 + assign3380_e5406);
        let assign3380_e5410: f64 = (p.p438 * locals.var_inv_lw);
        let assign3380_e5411: f64 = (assign3380_e5407 + assign3380_e5410);
        locals.var_pparam_b4soiminvcv = assign3380_e5411;
        locals.var_pparam_b4soiminvcv_dn3 = (((p.p436 * locals.var_inv_l_dn3) + (p.p437 * locals.var_inv_w_dn3)) + (p.p438 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiminvcv_dn4 = (((p.p436 * locals.var_inv_l_dn4) + (p.p437 * locals.var_inv_w_dn4)) + (p.p438 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiminvcv_dn5 = (((p.p436 * locals.var_inv_l_dn5) + (p.p437 * locals.var_inv_w_dn5)) + (p.p438 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiminvcv_dn6 = (((p.p436 * locals.var_inv_l_dn6) + (p.p437 * locals.var_inv_w_dn6)) + (p.p438 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiminvcv_dn7 = (((p.p436 * locals.var_inv_l_dn7) + (p.p437 * locals.var_inv_w_dn7)) + (p.p438 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiminvcv_dn8 = (((p.p436 * locals.var_inv_l_dn8) + (p.p437 * locals.var_inv_w_dn8)) + (p.p438 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiminvcv_dn9 = (((p.p436 * locals.var_inv_l_dn9) + (p.p437 * locals.var_inv_w_dn9)) + (p.p438 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiminvcv_dn10 = (((p.p436 * locals.var_inv_l_dn10) + (p.p437 * locals.var_inv_w_dn10)) + (p.p438 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiminvcv_dn11 = (((p.p436 * locals.var_inv_l_dn11) + (p.p437 * locals.var_inv_w_dn11)) + (p.p438 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiminvcv_dn12 = (((p.p436 * locals.var_inv_l_dn12) + (p.p437 * locals.var_inv_w_dn12)) + (p.p438 * locals.var_inv_lw_dn12));

        let assign3390_e5415: f64 = (p.p440 * locals.var_inv_l);
        let assign3390_e5416: f64 = (p.p439 + assign3390_e5415);
        let assign3390_e5419: f64 = (p.p441 * locals.var_inv_w);
        let assign3390_e5420: f64 = (assign3390_e5416 + assign3390_e5419);
        let assign3390_e5423: f64 = (p.p442 * locals.var_inv_lw);
        let assign3390_e5424: f64 = (assign3390_e5420 + assign3390_e5423);
        locals.var_pparam_b4soivoffcv = assign3390_e5424;
        locals.var_pparam_b4soivoffcv_dn3 = (((p.p440 * locals.var_inv_l_dn3) + (p.p441 * locals.var_inv_w_dn3)) + (p.p442 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soivoffcv_dn4 = (((p.p440 * locals.var_inv_l_dn4) + (p.p441 * locals.var_inv_w_dn4)) + (p.p442 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soivoffcv_dn5 = (((p.p440 * locals.var_inv_l_dn5) + (p.p441 * locals.var_inv_w_dn5)) + (p.p442 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soivoffcv_dn6 = (((p.p440 * locals.var_inv_l_dn6) + (p.p441 * locals.var_inv_w_dn6)) + (p.p442 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soivoffcv_dn7 = (((p.p440 * locals.var_inv_l_dn7) + (p.p441 * locals.var_inv_w_dn7)) + (p.p442 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soivoffcv_dn8 = (((p.p440 * locals.var_inv_l_dn8) + (p.p441 * locals.var_inv_w_dn8)) + (p.p442 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soivoffcv_dn9 = (((p.p440 * locals.var_inv_l_dn9) + (p.p441 * locals.var_inv_w_dn9)) + (p.p442 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soivoffcv_dn10 = (((p.p440 * locals.var_inv_l_dn10) + (p.p441 * locals.var_inv_w_dn10)) + (p.p442 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soivoffcv_dn11 = (((p.p440 * locals.var_inv_l_dn11) + (p.p441 * locals.var_inv_w_dn11)) + (p.p442 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soivoffcv_dn12 = (((p.p440 * locals.var_inv_l_dn12) + (p.p441 * locals.var_inv_w_dn12)) + (p.p442 * locals.var_inv_lw_dn12));

        let assign3400_e5428: f64 = (p.p286 * locals.var_inv_l);
        let assign3400_e5429: f64 = (p.p285 + assign3400_e5428);
        let assign3400_e5432: f64 = (p.p289 * locals.var_inv_w);
        let assign3400_e5433: f64 = (assign3400_e5429 + assign3400_e5432);
        let assign3400_e5436: f64 = (p.p292 * locals.var_inv_lw);
        let assign3400_e5437: f64 = (assign3400_e5433 + assign3400_e5436);
        locals.var_pparam_b4soifprout = assign3400_e5437;
        locals.var_pparam_b4soifprout_dn3 = (((p.p286 * locals.var_inv_l_dn3) + (p.p289 * locals.var_inv_w_dn3)) + (p.p292 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soifprout_dn4 = (((p.p286 * locals.var_inv_l_dn4) + (p.p289 * locals.var_inv_w_dn4)) + (p.p292 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soifprout_dn5 = (((p.p286 * locals.var_inv_l_dn5) + (p.p289 * locals.var_inv_w_dn5)) + (p.p292 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soifprout_dn6 = (((p.p286 * locals.var_inv_l_dn6) + (p.p289 * locals.var_inv_w_dn6)) + (p.p292 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soifprout_dn7 = (((p.p286 * locals.var_inv_l_dn7) + (p.p289 * locals.var_inv_w_dn7)) + (p.p292 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soifprout_dn8 = (((p.p286 * locals.var_inv_l_dn8) + (p.p289 * locals.var_inv_w_dn8)) + (p.p292 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soifprout_dn9 = (((p.p286 * locals.var_inv_l_dn9) + (p.p289 * locals.var_inv_w_dn9)) + (p.p292 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soifprout_dn10 = (((p.p286 * locals.var_inv_l_dn10) + (p.p289 * locals.var_inv_w_dn10)) + (p.p292 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soifprout_dn11 = (((p.p286 * locals.var_inv_l_dn11) + (p.p289 * locals.var_inv_w_dn11)) + (p.p292 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soifprout_dn12 = (((p.p286 * locals.var_inv_l_dn12) + (p.p289 * locals.var_inv_w_dn12)) + (p.p292 * locals.var_inv_lw_dn12));

        let assign3410_e5441: f64 = (p.p287 * locals.var_inv_l);
        let assign3410_e5442: f64 = (p.p282 + assign3410_e5441);
        let assign3410_e5445: f64 = (p.p290 * locals.var_inv_w);
        let assign3410_e5446: f64 = (assign3410_e5442 + assign3410_e5445);
        let assign3410_e5449: f64 = (p.p293 * locals.var_inv_lw);
        let assign3410_e5450: f64 = (assign3410_e5446 + assign3410_e5449);
        locals.var_pparam_b4soipdits = assign3410_e5450;
        locals.var_pparam_b4soipdits_dn3 = (((p.p287 * locals.var_inv_l_dn3) + (p.p290 * locals.var_inv_w_dn3)) + (p.p293 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soipdits_dn4 = (((p.p287 * locals.var_inv_l_dn4) + (p.p290 * locals.var_inv_w_dn4)) + (p.p293 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soipdits_dn5 = (((p.p287 * locals.var_inv_l_dn5) + (p.p290 * locals.var_inv_w_dn5)) + (p.p293 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soipdits_dn6 = (((p.p287 * locals.var_inv_l_dn6) + (p.p290 * locals.var_inv_w_dn6)) + (p.p293 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soipdits_dn7 = (((p.p287 * locals.var_inv_l_dn7) + (p.p290 * locals.var_inv_w_dn7)) + (p.p293 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soipdits_dn8 = (((p.p287 * locals.var_inv_l_dn8) + (p.p290 * locals.var_inv_w_dn8)) + (p.p293 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soipdits_dn9 = (((p.p287 * locals.var_inv_l_dn9) + (p.p290 * locals.var_inv_w_dn9)) + (p.p293 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soipdits_dn10 = (((p.p287 * locals.var_inv_l_dn10) + (p.p290 * locals.var_inv_w_dn10)) + (p.p293 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soipdits_dn11 = (((p.p287 * locals.var_inv_l_dn11) + (p.p290 * locals.var_inv_w_dn11)) + (p.p293 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soipdits_dn12 = (((p.p287 * locals.var_inv_l_dn12) + (p.p290 * locals.var_inv_w_dn12)) + (p.p293 * locals.var_inv_lw_dn12));

        let assign3420_e5454: f64 = (p.p288 * locals.var_inv_l);
        let assign3420_e5455: f64 = (p.p284 + assign3420_e5454);
        let assign3420_e5458: f64 = (p.p291 * locals.var_inv_w);
        let assign3420_e5459: f64 = (assign3420_e5455 + assign3420_e5458);
        let assign3420_e5462: f64 = (p.p294 * locals.var_inv_lw);
        let assign3420_e5463: f64 = (assign3420_e5459 + assign3420_e5462);
        locals.var_pparam_b4soipditsd = assign3420_e5463;
        locals.var_pparam_b4soipditsd_dn3 = (((p.p288 * locals.var_inv_l_dn3) + (p.p291 * locals.var_inv_w_dn3)) + (p.p294 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soipditsd_dn4 = (((p.p288 * locals.var_inv_l_dn4) + (p.p291 * locals.var_inv_w_dn4)) + (p.p294 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soipditsd_dn5 = (((p.p288 * locals.var_inv_l_dn5) + (p.p291 * locals.var_inv_w_dn5)) + (p.p294 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soipditsd_dn6 = (((p.p288 * locals.var_inv_l_dn6) + (p.p291 * locals.var_inv_w_dn6)) + (p.p294 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soipditsd_dn7 = (((p.p288 * locals.var_inv_l_dn7) + (p.p291 * locals.var_inv_w_dn7)) + (p.p294 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soipditsd_dn8 = (((p.p288 * locals.var_inv_l_dn8) + (p.p291 * locals.var_inv_w_dn8)) + (p.p294 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soipditsd_dn9 = (((p.p288 * locals.var_inv_l_dn9) + (p.p291 * locals.var_inv_w_dn9)) + (p.p294 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soipditsd_dn10 = (((p.p288 * locals.var_inv_l_dn10) + (p.p291 * locals.var_inv_w_dn10)) + (p.p294 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soipditsd_dn11 = (((p.p288 * locals.var_inv_l_dn11) + (p.p291 * locals.var_inv_w_dn11)) + (p.p294 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soipditsd_dn12 = (((p.p288 * locals.var_inv_l_dn12) + (p.p291 * locals.var_inv_w_dn12)) + (p.p294 * locals.var_inv_lw_dn12));

        let assign3430_e5467: f64 = (p.p450 * locals.var_inv_l);
        let assign3430_e5468: f64 = (p.p392 + assign3430_e5467);
        let assign3430_e5471: f64 = (p.p640 * locals.var_inv_w);
        let assign3430_e5472: f64 = (assign3430_e5468 + assign3430_e5471);
        let assign3430_e5475: f64 = (p.p830 * locals.var_inv_lw);
        let assign3430_e5476: f64 = (assign3430_e5472 + assign3430_e5475);
        locals.var_pparam_b4soiaigbcp2 = assign3430_e5476;
        locals.var_pparam_b4soiaigbcp2_dn3 = (((p.p450 * locals.var_inv_l_dn3) + (p.p640 * locals.var_inv_w_dn3)) + (p.p830 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiaigbcp2_dn4 = (((p.p450 * locals.var_inv_l_dn4) + (p.p640 * locals.var_inv_w_dn4)) + (p.p830 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiaigbcp2_dn5 = (((p.p450 * locals.var_inv_l_dn5) + (p.p640 * locals.var_inv_w_dn5)) + (p.p830 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiaigbcp2_dn6 = (((p.p450 * locals.var_inv_l_dn6) + (p.p640 * locals.var_inv_w_dn6)) + (p.p830 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiaigbcp2_dn7 = (((p.p450 * locals.var_inv_l_dn7) + (p.p640 * locals.var_inv_w_dn7)) + (p.p830 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiaigbcp2_dn8 = (((p.p450 * locals.var_inv_l_dn8) + (p.p640 * locals.var_inv_w_dn8)) + (p.p830 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiaigbcp2_dn9 = (((p.p450 * locals.var_inv_l_dn9) + (p.p640 * locals.var_inv_w_dn9)) + (p.p830 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiaigbcp2_dn10 = (((p.p450 * locals.var_inv_l_dn10) + (p.p640 * locals.var_inv_w_dn10)) + (p.p830 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiaigbcp2_dn11 = (((p.p450 * locals.var_inv_l_dn11) + (p.p640 * locals.var_inv_w_dn11)) + (p.p830 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiaigbcp2_dn12 = (((p.p450 * locals.var_inv_l_dn12) + (p.p640 * locals.var_inv_w_dn12)) + (p.p830 * locals.var_inv_lw_dn12));

        let assign3440_e5480: f64 = (p.p451 * locals.var_inv_l);
        let assign3440_e5481: f64 = (p.p393 + assign3440_e5480);
        let assign3440_e5484: f64 = (p.p641 * locals.var_inv_w);
        let assign3440_e5485: f64 = (assign3440_e5481 + assign3440_e5484);
        let assign3440_e5488: f64 = (p.p831 * locals.var_inv_lw);
        let assign3440_e5489: f64 = (assign3440_e5485 + assign3440_e5488);
        locals.var_pparam_b4soiaigbcp2_t = assign3440_e5489;
        locals.var_pparam_b4soiaigbcp2_t_dn3 = (((p.p451 * locals.var_inv_l_dn3) + (p.p641 * locals.var_inv_w_dn3)) + (p.p831 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soiaigbcp2_t_dn4 = (((p.p451 * locals.var_inv_l_dn4) + (p.p641 * locals.var_inv_w_dn4)) + (p.p831 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soiaigbcp2_t_dn5 = (((p.p451 * locals.var_inv_l_dn5) + (p.p641 * locals.var_inv_w_dn5)) + (p.p831 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soiaigbcp2_t_dn6 = (((p.p451 * locals.var_inv_l_dn6) + (p.p641 * locals.var_inv_w_dn6)) + (p.p831 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soiaigbcp2_t_dn7 = (((p.p451 * locals.var_inv_l_dn7) + (p.p641 * locals.var_inv_w_dn7)) + (p.p831 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soiaigbcp2_t_dn8 = (((p.p451 * locals.var_inv_l_dn8) + (p.p641 * locals.var_inv_w_dn8)) + (p.p831 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soiaigbcp2_t_dn9 = (((p.p451 * locals.var_inv_l_dn9) + (p.p641 * locals.var_inv_w_dn9)) + (p.p831 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soiaigbcp2_t_dn10 = (((p.p451 * locals.var_inv_l_dn10) + (p.p641 * locals.var_inv_w_dn10)) + (p.p831 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soiaigbcp2_t_dn11 = (((p.p451 * locals.var_inv_l_dn11) + (p.p641 * locals.var_inv_w_dn11)) + (p.p831 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soiaigbcp2_t_dn12 = (((p.p451 * locals.var_inv_l_dn12) + (p.p641 * locals.var_inv_w_dn12)) + (p.p831 * locals.var_inv_lw_dn12));

        let assign3450_e5493: f64 = (p.p452 * locals.var_inv_l);
        let assign3450_e5494: f64 = (p.p394 + assign3450_e5493);
        let assign3450_e5497: f64 = (p.p642 * locals.var_inv_w);
        let assign3450_e5498: f64 = (assign3450_e5494 + assign3450_e5497);
        let assign3450_e5501: f64 = (p.p832 * locals.var_inv_lw);
        let assign3450_e5502: f64 = (assign3450_e5498 + assign3450_e5501);
        locals.var_pparam_b4soibigbcp2 = assign3450_e5502;
        locals.var_pparam_b4soibigbcp2_dn3 = (((p.p452 * locals.var_inv_l_dn3) + (p.p642 * locals.var_inv_w_dn3)) + (p.p832 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soibigbcp2_dn4 = (((p.p452 * locals.var_inv_l_dn4) + (p.p642 * locals.var_inv_w_dn4)) + (p.p832 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soibigbcp2_dn5 = (((p.p452 * locals.var_inv_l_dn5) + (p.p642 * locals.var_inv_w_dn5)) + (p.p832 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soibigbcp2_dn6 = (((p.p452 * locals.var_inv_l_dn6) + (p.p642 * locals.var_inv_w_dn6)) + (p.p832 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soibigbcp2_dn7 = (((p.p452 * locals.var_inv_l_dn7) + (p.p642 * locals.var_inv_w_dn7)) + (p.p832 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soibigbcp2_dn8 = (((p.p452 * locals.var_inv_l_dn8) + (p.p642 * locals.var_inv_w_dn8)) + (p.p832 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soibigbcp2_dn9 = (((p.p452 * locals.var_inv_l_dn9) + (p.p642 * locals.var_inv_w_dn9)) + (p.p832 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soibigbcp2_dn10 = (((p.p452 * locals.var_inv_l_dn10) + (p.p642 * locals.var_inv_w_dn10)) + (p.p832 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soibigbcp2_dn11 = (((p.p452 * locals.var_inv_l_dn11) + (p.p642 * locals.var_inv_w_dn11)) + (p.p832 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soibigbcp2_dn12 = (((p.p452 * locals.var_inv_l_dn12) + (p.p642 * locals.var_inv_w_dn12)) + (p.p832 * locals.var_inv_lw_dn12));

        let assign3460_e5506: f64 = (p.p453 * locals.var_inv_l);
        let assign3460_e5507: f64 = (p.p395 + assign3460_e5506);
        let assign3460_e5510: f64 = (p.p643 * locals.var_inv_w);
        let assign3460_e5511: f64 = (assign3460_e5507 + assign3460_e5510);
        let assign3460_e5514: f64 = (p.p833 * locals.var_inv_lw);
        let assign3460_e5515: f64 = (assign3460_e5511 + assign3460_e5514);
        locals.var_pparam_b4soicigbcp2 = assign3460_e5515;
        locals.var_pparam_b4soicigbcp2_dn3 = (((p.p453 * locals.var_inv_l_dn3) + (p.p643 * locals.var_inv_w_dn3)) + (p.p833 * locals.var_inv_lw_dn3));
        locals.var_pparam_b4soicigbcp2_dn4 = (((p.p453 * locals.var_inv_l_dn4) + (p.p643 * locals.var_inv_w_dn4)) + (p.p833 * locals.var_inv_lw_dn4));
        locals.var_pparam_b4soicigbcp2_dn5 = (((p.p453 * locals.var_inv_l_dn5) + (p.p643 * locals.var_inv_w_dn5)) + (p.p833 * locals.var_inv_lw_dn5));
        locals.var_pparam_b4soicigbcp2_dn6 = (((p.p453 * locals.var_inv_l_dn6) + (p.p643 * locals.var_inv_w_dn6)) + (p.p833 * locals.var_inv_lw_dn6));
        locals.var_pparam_b4soicigbcp2_dn7 = (((p.p453 * locals.var_inv_l_dn7) + (p.p643 * locals.var_inv_w_dn7)) + (p.p833 * locals.var_inv_lw_dn7));
        locals.var_pparam_b4soicigbcp2_dn8 = (((p.p453 * locals.var_inv_l_dn8) + (p.p643 * locals.var_inv_w_dn8)) + (p.p833 * locals.var_inv_lw_dn8));
        locals.var_pparam_b4soicigbcp2_dn9 = (((p.p453 * locals.var_inv_l_dn9) + (p.p643 * locals.var_inv_w_dn9)) + (p.p833 * locals.var_inv_lw_dn9));
        locals.var_pparam_b4soicigbcp2_dn10 = (((p.p453 * locals.var_inv_l_dn10) + (p.p643 * locals.var_inv_w_dn10)) + (p.p833 * locals.var_inv_lw_dn10));
        locals.var_pparam_b4soicigbcp2_dn11 = (((p.p453 * locals.var_inv_l_dn11) + (p.p643 * locals.var_inv_w_dn11)) + (p.p833 * locals.var_inv_lw_dn11));
        locals.var_pparam_b4soicigbcp2_dn12 = (((p.p453 * locals.var_inv_l_dn12) + (p.p643 * locals.var_inv_w_dn12)) + (p.p833 * locals.var_inv_lw_dn12));

        let assign3470_e5518: f64 = (locals.var_pparam_b4soiminv).atan();
        let assign3470_e5520: f64 = (assign3470_e5518 / 3.141592653589793);
        let assign3470_e5521: f64 = (0.5 + assign3470_e5520);
        locals.var_pparam_b4soimstar = assign3470_e5521;
        locals.var_pparam_b4soimstar_dn3 = ((locals.var_pparam_b4soiminv_dn3 / (1.0 + (locals.var_pparam_b4soiminv * locals.var_pparam_b4soiminv))) / 3.141592653589793);
        locals.var_pparam_b4soimstar_dn4 = ((locals.var_pparam_b4soiminv_dn4 / (1.0 + (locals.var_pparam_b4soiminv * locals.var_pparam_b4soiminv))) / 3.141592653589793);
        locals.var_pparam_b4soimstar_dn5 = ((locals.var_pparam_b4soiminv_dn5 / (1.0 + (locals.var_pparam_b4soiminv * locals.var_pparam_b4soiminv))) / 3.141592653589793);
        locals.var_pparam_b4soimstar_dn6 = ((locals.var_pparam_b4soiminv_dn6 / (1.0 + (locals.var_pparam_b4soiminv * locals.var_pparam_b4soiminv))) / 3.141592653589793);
        locals.var_pparam_b4soimstar_dn7 = ((locals.var_pparam_b4soiminv_dn7 / (1.0 + (locals.var_pparam_b4soiminv * locals.var_pparam_b4soiminv))) / 3.141592653589793);
        locals.var_pparam_b4soimstar_dn8 = ((locals.var_pparam_b4soiminv_dn8 / (1.0 + (locals.var_pparam_b4soiminv * locals.var_pparam_b4soiminv))) / 3.141592653589793);
        locals.var_pparam_b4soimstar_dn9 = ((locals.var_pparam_b4soiminv_dn9 / (1.0 + (locals.var_pparam_b4soiminv * locals.var_pparam_b4soiminv))) / 3.141592653589793);
        locals.var_pparam_b4soimstar_dn10 = ((locals.var_pparam_b4soiminv_dn10 / (1.0 + (locals.var_pparam_b4soiminv * locals.var_pparam_b4soiminv))) / 3.141592653589793);
        locals.var_pparam_b4soimstar_dn11 = ((locals.var_pparam_b4soiminv_dn11 / (1.0 + (locals.var_pparam_b4soiminv * locals.var_pparam_b4soiminv))) / 3.141592653589793);
        locals.var_pparam_b4soimstar_dn12 = ((locals.var_pparam_b4soiminv_dn12 / (1.0 + (locals.var_pparam_b4soiminv * locals.var_pparam_b4soiminv))) / 3.141592653589793);

        let assign3490_e5531: f64 = (locals.var_pparam_b4soiminvcv).atan();
        let assign3490_e5533: f64 = (assign3490_e5531 / 3.141592653589793);
        let assign3490_e5534: f64 = (0.5 + assign3490_e5533);
        locals.var_pparam_b4soimstarcv = assign3490_e5534;
        locals.var_pparam_b4soimstarcv_dn3 = ((locals.var_pparam_b4soiminvcv_dn3 / (1.0 + (locals.var_pparam_b4soiminvcv * locals.var_pparam_b4soiminvcv))) / 3.141592653589793);
        locals.var_pparam_b4soimstarcv_dn4 = ((locals.var_pparam_b4soiminvcv_dn4 / (1.0 + (locals.var_pparam_b4soiminvcv * locals.var_pparam_b4soiminvcv))) / 3.141592653589793);
        locals.var_pparam_b4soimstarcv_dn5 = ((locals.var_pparam_b4soiminvcv_dn5 / (1.0 + (locals.var_pparam_b4soiminvcv * locals.var_pparam_b4soiminvcv))) / 3.141592653589793);
        locals.var_pparam_b4soimstarcv_dn6 = ((locals.var_pparam_b4soiminvcv_dn6 / (1.0 + (locals.var_pparam_b4soiminvcv * locals.var_pparam_b4soiminvcv))) / 3.141592653589793);
        locals.var_pparam_b4soimstarcv_dn7 = ((locals.var_pparam_b4soiminvcv_dn7 / (1.0 + (locals.var_pparam_b4soiminvcv * locals.var_pparam_b4soiminvcv))) / 3.141592653589793);
        locals.var_pparam_b4soimstarcv_dn8 = ((locals.var_pparam_b4soiminvcv_dn8 / (1.0 + (locals.var_pparam_b4soiminvcv * locals.var_pparam_b4soiminvcv))) / 3.141592653589793);
        locals.var_pparam_b4soimstarcv_dn9 = ((locals.var_pparam_b4soiminvcv_dn9 / (1.0 + (locals.var_pparam_b4soiminvcv * locals.var_pparam_b4soiminvcv))) / 3.141592653589793);
        locals.var_pparam_b4soimstarcv_dn10 = ((locals.var_pparam_b4soiminvcv_dn10 / (1.0 + (locals.var_pparam_b4soiminvcv * locals.var_pparam_b4soiminvcv))) / 3.141592653589793);
        locals.var_pparam_b4soimstarcv_dn11 = ((locals.var_pparam_b4soiminvcv_dn11 / (1.0 + (locals.var_pparam_b4soiminvcv * locals.var_pparam_b4soiminvcv))) / 3.141592653589793);
        locals.var_pparam_b4soimstarcv_dn12 = ((locals.var_pparam_b4soiminvcv_dn12 / (1.0 + (locals.var_pparam_b4soiminvcv * locals.var_pparam_b4soiminvcv))) / 3.141592653589793);

        let assign3500_e5537: f64 = (locals.var_tempratio__blk441 - 1.0);
        locals.var_trm1 = assign3500_e5537;
        locals.var_trm1_dn4 = locals.var_tempratio__blk441_dn4;
        locals.var_trm1_dn5 = locals.var_tempratio__blk441_dn5;
        locals.var_trm1_dn6 = locals.var_tempratio__blk441_dn6;

        locals.var_pparam_b4soiuatemp = locals.var_pparam_b4soiua;
        locals.var_pparam_b4soiuatemp_dn3 = locals.var_pparam_b4soiua_dn3;
        locals.var_pparam_b4soiuatemp_dn4 = locals.var_pparam_b4soiua_dn4;
        locals.var_pparam_b4soiuatemp_dn5 = locals.var_pparam_b4soiua_dn5;
        locals.var_pparam_b4soiuatemp_dn6 = locals.var_pparam_b4soiua_dn6;
        locals.var_pparam_b4soiuatemp_dn7 = locals.var_pparam_b4soiua_dn7;
        locals.var_pparam_b4soiuatemp_dn8 = locals.var_pparam_b4soiua_dn8;
        locals.var_pparam_b4soiuatemp_dn9 = locals.var_pparam_b4soiua_dn9;
        locals.var_pparam_b4soiuatemp_dn10 = locals.var_pparam_b4soiua_dn10;
        locals.var_pparam_b4soiuatemp_dn11 = locals.var_pparam_b4soiua_dn11;
        locals.var_pparam_b4soiuatemp_dn12 = locals.var_pparam_b4soiua_dn12;

        locals.var_pparam_b4soiubtemp = locals.var_pparam_b4soiub;
        locals.var_pparam_b4soiubtemp_dn3 = locals.var_pparam_b4soiub_dn3;
        locals.var_pparam_b4soiubtemp_dn4 = locals.var_pparam_b4soiub_dn4;
        locals.var_pparam_b4soiubtemp_dn5 = locals.var_pparam_b4soiub_dn5;
        locals.var_pparam_b4soiubtemp_dn6 = locals.var_pparam_b4soiub_dn6;
        locals.var_pparam_b4soiubtemp_dn7 = locals.var_pparam_b4soiub_dn7;
        locals.var_pparam_b4soiubtemp_dn8 = locals.var_pparam_b4soiub_dn8;
        locals.var_pparam_b4soiubtemp_dn9 = locals.var_pparam_b4soiub_dn9;
        locals.var_pparam_b4soiubtemp_dn10 = locals.var_pparam_b4soiub_dn10;
        locals.var_pparam_b4soiubtemp_dn11 = locals.var_pparam_b4soiub_dn11;
        locals.var_pparam_b4soiubtemp_dn12 = locals.var_pparam_b4soiub_dn12;

        locals.var_pparam_b4soiuctemp = locals.var_pparam_b4soiuc;
        locals.var_pparam_b4soiuctemp_dn3 = locals.var_pparam_b4soiuc_dn3;
        locals.var_pparam_b4soiuctemp_dn4 = locals.var_pparam_b4soiuc_dn4;
        locals.var_pparam_b4soiuctemp_dn5 = locals.var_pparam_b4soiuc_dn5;
        locals.var_pparam_b4soiuctemp_dn6 = locals.var_pparam_b4soiuc_dn6;
        locals.var_pparam_b4soiuctemp_dn7 = locals.var_pparam_b4soiuc_dn7;
        locals.var_pparam_b4soiuctemp_dn8 = locals.var_pparam_b4soiuc_dn8;
        locals.var_pparam_b4soiuctemp_dn9 = locals.var_pparam_b4soiuc_dn9;
        locals.var_pparam_b4soiuctemp_dn10 = locals.var_pparam_b4soiuc_dn10;
        locals.var_pparam_b4soiuctemp_dn11 = locals.var_pparam_b4soiuc_dn11;
        locals.var_pparam_b4soiuctemp_dn12 = locals.var_pparam_b4soiuc_dn12;

        let assign3540_e5543: f64 = (locals.var_pparam_b4soiweff * 1000000.0);
        let assign3540_e5545: f64 = (assign3540_e5543).powf(locals.var_pparam_b4soiwr);
        locals.var_pparam_b4soirds0denom = assign3540_e5545;
        locals.var_pparam_b4soirds0denom_dn3 = if locals.var_pparam_b4soiwr_dn3 == 0.0 && ((locals.var_pparam_b4soiwr) as f64).is_finite() && ((locals.var_pparam_b4soiwr) as f64).fract() == 0.0 { if locals.var_pparam_b4soiwr == 0.0 { 0.0 } else { (locals.var_pparam_b4soiwr * ((assign3540_e5543).powf(locals.var_pparam_b4soiwr - 1.0) * (locals.var_pparam_b4soiweff_dn3 * 1000000.0))) } } else { (assign3540_e5545 * ((locals.var_pparam_b4soiwr_dn3 * (assign3540_e5543).ln()) + (locals.var_pparam_b4soiwr * ((locals.var_pparam_b4soiweff_dn3 * 1000000.0) / assign3540_e5543)))) };
        locals.var_pparam_b4soirds0denom_dn4 = if locals.var_pparam_b4soiwr_dn4 == 0.0 && ((locals.var_pparam_b4soiwr) as f64).is_finite() && ((locals.var_pparam_b4soiwr) as f64).fract() == 0.0 { if locals.var_pparam_b4soiwr == 0.0 { 0.0 } else { (locals.var_pparam_b4soiwr * ((assign3540_e5543).powf(locals.var_pparam_b4soiwr - 1.0) * (locals.var_pparam_b4soiweff_dn4 * 1000000.0))) } } else { (assign3540_e5545 * ((locals.var_pparam_b4soiwr_dn4 * (assign3540_e5543).ln()) + (locals.var_pparam_b4soiwr * ((locals.var_pparam_b4soiweff_dn4 * 1000000.0) / assign3540_e5543)))) };
        locals.var_pparam_b4soirds0denom_dn5 = if locals.var_pparam_b4soiwr_dn5 == 0.0 && ((locals.var_pparam_b4soiwr) as f64).is_finite() && ((locals.var_pparam_b4soiwr) as f64).fract() == 0.0 { if locals.var_pparam_b4soiwr == 0.0 { 0.0 } else { (locals.var_pparam_b4soiwr * ((assign3540_e5543).powf(locals.var_pparam_b4soiwr - 1.0) * (locals.var_pparam_b4soiweff_dn5 * 1000000.0))) } } else { (assign3540_e5545 * ((locals.var_pparam_b4soiwr_dn5 * (assign3540_e5543).ln()) + (locals.var_pparam_b4soiwr * ((locals.var_pparam_b4soiweff_dn5 * 1000000.0) / assign3540_e5543)))) };
        locals.var_pparam_b4soirds0denom_dn6 = if locals.var_pparam_b4soiwr_dn6 == 0.0 && ((locals.var_pparam_b4soiwr) as f64).is_finite() && ((locals.var_pparam_b4soiwr) as f64).fract() == 0.0 { if locals.var_pparam_b4soiwr == 0.0 { 0.0 } else { (locals.var_pparam_b4soiwr * ((assign3540_e5543).powf(locals.var_pparam_b4soiwr - 1.0) * (locals.var_pparam_b4soiweff_dn6 * 1000000.0))) } } else { (assign3540_e5545 * ((locals.var_pparam_b4soiwr_dn6 * (assign3540_e5543).ln()) + (locals.var_pparam_b4soiwr * ((locals.var_pparam_b4soiweff_dn6 * 1000000.0) / assign3540_e5543)))) };
        locals.var_pparam_b4soirds0denom_dn7 = if locals.var_pparam_b4soiwr_dn7 == 0.0 && ((locals.var_pparam_b4soiwr) as f64).is_finite() && ((locals.var_pparam_b4soiwr) as f64).fract() == 0.0 { if locals.var_pparam_b4soiwr == 0.0 { 0.0 } else { (locals.var_pparam_b4soiwr * ((assign3540_e5543).powf(locals.var_pparam_b4soiwr - 1.0) * (locals.var_pparam_b4soiweff_dn7 * 1000000.0))) } } else { (assign3540_e5545 * ((locals.var_pparam_b4soiwr_dn7 * (assign3540_e5543).ln()) + (locals.var_pparam_b4soiwr * ((locals.var_pparam_b4soiweff_dn7 * 1000000.0) / assign3540_e5543)))) };
        locals.var_pparam_b4soirds0denom_dn8 = if locals.var_pparam_b4soiwr_dn8 == 0.0 && ((locals.var_pparam_b4soiwr) as f64).is_finite() && ((locals.var_pparam_b4soiwr) as f64).fract() == 0.0 { if locals.var_pparam_b4soiwr == 0.0 { 0.0 } else { (locals.var_pparam_b4soiwr * ((assign3540_e5543).powf(locals.var_pparam_b4soiwr - 1.0) * (locals.var_pparam_b4soiweff_dn8 * 1000000.0))) } } else { (assign3540_e5545 * ((locals.var_pparam_b4soiwr_dn8 * (assign3540_e5543).ln()) + (locals.var_pparam_b4soiwr * ((locals.var_pparam_b4soiweff_dn8 * 1000000.0) / assign3540_e5543)))) };
        locals.var_pparam_b4soirds0denom_dn9 = if locals.var_pparam_b4soiwr_dn9 == 0.0 && ((locals.var_pparam_b4soiwr) as f64).is_finite() && ((locals.var_pparam_b4soiwr) as f64).fract() == 0.0 { if locals.var_pparam_b4soiwr == 0.0 { 0.0 } else { (locals.var_pparam_b4soiwr * ((assign3540_e5543).powf(locals.var_pparam_b4soiwr - 1.0) * (locals.var_pparam_b4soiweff_dn9 * 1000000.0))) } } else { (assign3540_e5545 * ((locals.var_pparam_b4soiwr_dn9 * (assign3540_e5543).ln()) + (locals.var_pparam_b4soiwr * ((locals.var_pparam_b4soiweff_dn9 * 1000000.0) / assign3540_e5543)))) };
        locals.var_pparam_b4soirds0denom_dn10 = if locals.var_pparam_b4soiwr_dn10 == 0.0 && ((locals.var_pparam_b4soiwr) as f64).is_finite() && ((locals.var_pparam_b4soiwr) as f64).fract() == 0.0 { if locals.var_pparam_b4soiwr == 0.0 { 0.0 } else { (locals.var_pparam_b4soiwr * ((assign3540_e5543).powf(locals.var_pparam_b4soiwr - 1.0) * (locals.var_pparam_b4soiweff_dn10 * 1000000.0))) } } else { (assign3540_e5545 * ((locals.var_pparam_b4soiwr_dn10 * (assign3540_e5543).ln()) + (locals.var_pparam_b4soiwr * ((locals.var_pparam_b4soiweff_dn10 * 1000000.0) / assign3540_e5543)))) };
        locals.var_pparam_b4soirds0denom_dn11 = if locals.var_pparam_b4soiwr_dn11 == 0.0 && ((locals.var_pparam_b4soiwr) as f64).is_finite() && ((locals.var_pparam_b4soiwr) as f64).fract() == 0.0 { if locals.var_pparam_b4soiwr == 0.0 { 0.0 } else { (locals.var_pparam_b4soiwr * ((assign3540_e5543).powf(locals.var_pparam_b4soiwr - 1.0) * (locals.var_pparam_b4soiweff_dn11 * 1000000.0))) } } else { (assign3540_e5545 * ((locals.var_pparam_b4soiwr_dn11 * (assign3540_e5543).ln()) + (locals.var_pparam_b4soiwr * ((locals.var_pparam_b4soiweff_dn11 * 1000000.0) / assign3540_e5543)))) };
        locals.var_pparam_b4soirds0denom_dn12 = if locals.var_pparam_b4soiwr_dn12 == 0.0 && ((locals.var_pparam_b4soiwr) as f64).is_finite() && ((locals.var_pparam_b4soiwr) as f64).fract() == 0.0 { if locals.var_pparam_b4soiwr == 0.0 { 0.0 } else { (locals.var_pparam_b4soiwr * ((assign3540_e5543).powf(locals.var_pparam_b4soiwr - 1.0) * (locals.var_pparam_b4soiweff_dn12 * 1000000.0))) } } else { (assign3540_e5545 * ((locals.var_pparam_b4soiwr_dn12 * (assign3540_e5543).ln()) + (locals.var_pparam_b4soiwr * ((locals.var_pparam_b4soiweff_dn12 * 1000000.0) / assign3540_e5543)))) };

        let assign3550_e5550: f64 = (locals.var_pparam_b4soiweff + p.p377);
        let assign3550_e5551: f64 = (p.p3 * assign3550_e5550);
        let assign3550_e5552: f64 = (p.p14 / assign3550_e5551);
        let assign3550_e5554: f64 = (assign3550_e5552 * p.p23);
        locals.var_pparam_b4soirth = assign3550_e5554;
        locals.var_pparam_b4soirth_dn3 = ((-((p.p14 * (p.p3 * locals.var_pparam_b4soiweff_dn3)) / (assign3550_e5551 * assign3550_e5551))) * p.p23);
        locals.var_pparam_b4soirth_dn4 = ((-((p.p14 * (p.p3 * locals.var_pparam_b4soiweff_dn4)) / (assign3550_e5551 * assign3550_e5551))) * p.p23);
        locals.var_pparam_b4soirth_dn5 = ((-((p.p14 * (p.p3 * locals.var_pparam_b4soiweff_dn5)) / (assign3550_e5551 * assign3550_e5551))) * p.p23);
        locals.var_pparam_b4soirth_dn6 = ((-((p.p14 * (p.p3 * locals.var_pparam_b4soiweff_dn6)) / (assign3550_e5551 * assign3550_e5551))) * p.p23);
        locals.var_pparam_b4soirth_dn7 = ((-((p.p14 * (p.p3 * locals.var_pparam_b4soiweff_dn7)) / (assign3550_e5551 * assign3550_e5551))) * p.p23);
        locals.var_pparam_b4soirth_dn8 = ((-((p.p14 * (p.p3 * locals.var_pparam_b4soiweff_dn8)) / (assign3550_e5551 * assign3550_e5551))) * p.p23);
        locals.var_pparam_b4soirth_dn9 = ((-((p.p14 * (p.p3 * locals.var_pparam_b4soiweff_dn9)) / (assign3550_e5551 * assign3550_e5551))) * p.p23);
        locals.var_pparam_b4soirth_dn10 = ((-((p.p14 * (p.p3 * locals.var_pparam_b4soiweff_dn10)) / (assign3550_e5551 * assign3550_e5551))) * p.p23);
        locals.var_pparam_b4soirth_dn11 = ((-((p.p14 * (p.p3 * locals.var_pparam_b4soiweff_dn11)) / (assign3550_e5551 * assign3550_e5551))) * p.p23);
        locals.var_pparam_b4soirth_dn12 = ((-((p.p14 * (p.p3 * locals.var_pparam_b4soiweff_dn12)) / (assign3550_e5551 * assign3550_e5551))) * p.p23);

        let assign3560_e5559: f64 = (locals.var_pparam_b4soiweff + p.p377);
        let assign3560_e5560: f64 = (p.p3 * assign3560_e5559);
        let assign3560_e5561: f64 = (p.p15 * assign3560_e5560);
        let assign3560_e5563: f64 = (assign3560_e5561 / p.p23);
        locals.var_pparam_b4soicth = assign3560_e5563;
        locals.var_pparam_b4soicth_dn3 = ((p.p15 * (p.p3 * locals.var_pparam_b4soiweff_dn3)) / p.p23);
        locals.var_pparam_b4soicth_dn4 = ((p.p15 * (p.p3 * locals.var_pparam_b4soiweff_dn4)) / p.p23);
        locals.var_pparam_b4soicth_dn5 = ((p.p15 * (p.p3 * locals.var_pparam_b4soiweff_dn5)) / p.p23);
        locals.var_pparam_b4soicth_dn6 = ((p.p15 * (p.p3 * locals.var_pparam_b4soiweff_dn6)) / p.p23);
        locals.var_pparam_b4soicth_dn7 = ((p.p15 * (p.p3 * locals.var_pparam_b4soiweff_dn7)) / p.p23);
        locals.var_pparam_b4soicth_dn8 = ((p.p15 * (p.p3 * locals.var_pparam_b4soiweff_dn8)) / p.p23);
        locals.var_pparam_b4soicth_dn9 = ((p.p15 * (p.p3 * locals.var_pparam_b4soiweff_dn9)) / p.p23);
        locals.var_pparam_b4soicth_dn10 = ((p.p15 * (p.p3 * locals.var_pparam_b4soiweff_dn10)) / p.p23);
        locals.var_pparam_b4soicth_dn11 = ((p.p15 * (p.p3 * locals.var_pparam_b4soiweff_dn11)) / p.p23);
        locals.var_pparam_b4soicth_dn12 = ((p.p15 * (p.p3 * locals.var_pparam_b4soiweff_dn12)) / p.p23);

        let assign3570_e5566: f64 = if locals.var_b4soirbody == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard512 = assign3570_e5566;

        let (assign3580_e5570, assign3580_e5570_d_n3, assign3580_e5570_d_n4, assign3580_e5570_d_n5, assign3580_e5570_d_n6, assign3580_e5570_d_n7, assign3580_e5570_d_n8, assign3580_e5570_d_n9, assign3580_e5570_d_n10, assign3580_e5570_d_n11, assign3580_e5570_d_n12,) = {
    if (locals.var_guard512 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soirbody, locals.var_pparam_b4soirbody_dn3, locals.var_pparam_b4soirbody_dn4, locals.var_pparam_b4soirbody_dn5, locals.var_pparam_b4soirbody_dn6, locals.var_pparam_b4soirbody_dn7, locals.var_pparam_b4soirbody_dn8, locals.var_pparam_b4soirbody_dn9, locals.var_pparam_b4soirbody_dn10, locals.var_pparam_b4soirbody_dn11, locals.var_pparam_b4soirbody_dn12,)
    }
};
        locals.var_pparam_b4soirbody = assign3580_e5570;
        locals.var_pparam_b4soirbody_dn3 = assign3580_e5570_d_n3;
        locals.var_pparam_b4soirbody_dn4 = assign3580_e5570_d_n4;
        locals.var_pparam_b4soirbody_dn5 = assign3580_e5570_d_n5;
        locals.var_pparam_b4soirbody_dn6 = assign3580_e5570_d_n6;
        locals.var_pparam_b4soirbody_dn7 = assign3580_e5570_d_n7;
        locals.var_pparam_b4soirbody_dn8 = assign3580_e5570_d_n8;
        locals.var_pparam_b4soirbody_dn9 = assign3580_e5570_d_n9;
        locals.var_pparam_b4soirbody_dn10 = assign3580_e5570_d_n10;
        locals.var_pparam_b4soirbody_dn11 = assign3580_e5570_d_n11;
        locals.var_pparam_b4soirbody_dn12 = assign3580_e5570_d_n12;

        let (assign3590_e5593, assign3590_e5593_d_n3, assign3590_e5593_d_n4, assign3590_e5593_d_n5, assign3590_e5593_d_n6, assign3590_e5593_d_n7, assign3590_e5593_d_n8, assign3590_e5593_d_n9, assign3590_e5593_d_n10, assign3590_e5593_d_n11, assign3590_e5593_d_n12,) = {
    if (locals.var_guard512 == 0.0) {
        let assign3590_e5575: f64 = (p.p17 * locals.var_b4soirbody);
        let assign3590_e5577: f64 = (assign3590_e5575 * p.p378);
        let assign3590_e5580: f64 = (2.0 * locals.var_b4soirbody);
        let assign3590_e5583: f64 = (p.p378 * locals.var_pparam_b4soileff);
        let assign3590_e5584: f64 = (assign3590_e5580 + assign3590_e5583);
        let assign3590_e5585: f64 = (assign3590_e5577 / assign3590_e5584);
        let assign3590_e5587: f64 = (assign3590_e5585 * locals.var_pparam_b4soiweff);
        let assign3590_e5589: f64 = (assign3590_e5587 / p.p23);
        let assign3590_e5591: f64 = (assign3590_e5589 / p.p3);
        (assign3590_e5591, (((((-((assign3590_e5577 * (p.p378 * locals.var_pparam_b4soileff_dn3)) / (assign3590_e5584 * assign3590_e5584))) * locals.var_pparam_b4soiweff) + (assign3590_e5585 * locals.var_pparam_b4soiweff_dn3)) / p.p23) / p.p3), (((((-((assign3590_e5577 * (p.p378 * locals.var_pparam_b4soileff_dn4)) / (assign3590_e5584 * assign3590_e5584))) * locals.var_pparam_b4soiweff) + (assign3590_e5585 * locals.var_pparam_b4soiweff_dn4)) / p.p23) / p.p3), (((((-((assign3590_e5577 * (p.p378 * locals.var_pparam_b4soileff_dn5)) / (assign3590_e5584 * assign3590_e5584))) * locals.var_pparam_b4soiweff) + (assign3590_e5585 * locals.var_pparam_b4soiweff_dn5)) / p.p23) / p.p3), (((((-((assign3590_e5577 * (p.p378 * locals.var_pparam_b4soileff_dn6)) / (assign3590_e5584 * assign3590_e5584))) * locals.var_pparam_b4soiweff) + (assign3590_e5585 * locals.var_pparam_b4soiweff_dn6)) / p.p23) / p.p3), (((((-((assign3590_e5577 * (p.p378 * locals.var_pparam_b4soileff_dn7)) / (assign3590_e5584 * assign3590_e5584))) * locals.var_pparam_b4soiweff) + (assign3590_e5585 * locals.var_pparam_b4soiweff_dn7)) / p.p23) / p.p3), (((((-((assign3590_e5577 * (p.p378 * locals.var_pparam_b4soileff_dn8)) / (assign3590_e5584 * assign3590_e5584))) * locals.var_pparam_b4soiweff) + (assign3590_e5585 * locals.var_pparam_b4soiweff_dn8)) / p.p23) / p.p3), (((((-((assign3590_e5577 * (p.p378 * locals.var_pparam_b4soileff_dn9)) / (assign3590_e5584 * assign3590_e5584))) * locals.var_pparam_b4soiweff) + (assign3590_e5585 * locals.var_pparam_b4soiweff_dn9)) / p.p23) / p.p3), (((((-((assign3590_e5577 * (p.p378 * locals.var_pparam_b4soileff_dn10)) / (assign3590_e5584 * assign3590_e5584))) * locals.var_pparam_b4soiweff) + (assign3590_e5585 * locals.var_pparam_b4soiweff_dn10)) / p.p23) / p.p3), (((((-((assign3590_e5577 * (p.p378 * locals.var_pparam_b4soileff_dn11)) / (assign3590_e5584 * assign3590_e5584))) * locals.var_pparam_b4soiweff) + (assign3590_e5585 * locals.var_pparam_b4soiweff_dn11)) / p.p23) / p.p3), (((((-((assign3590_e5577 * (p.p378 * locals.var_pparam_b4soileff_dn12)) / (assign3590_e5584 * assign3590_e5584))) * locals.var_pparam_b4soiweff) + (assign3590_e5585 * locals.var_pparam_b4soiweff_dn12)) / p.p23) / p.p3),)
    } else {
        (locals.var_pparam_b4soirbody, locals.var_pparam_b4soirbody_dn3, locals.var_pparam_b4soirbody_dn4, locals.var_pparam_b4soirbody_dn5, locals.var_pparam_b4soirbody_dn6, locals.var_pparam_b4soirbody_dn7, locals.var_pparam_b4soirbody_dn8, locals.var_pparam_b4soirbody_dn9, locals.var_pparam_b4soirbody_dn10, locals.var_pparam_b4soirbody_dn11, locals.var_pparam_b4soirbody_dn12,)
    }
};
        locals.var_pparam_b4soirbody = assign3590_e5593;
        locals.var_pparam_b4soirbody_dn3 = assign3590_e5593_d_n3;
        locals.var_pparam_b4soirbody_dn4 = assign3590_e5593_d_n4;
        locals.var_pparam_b4soirbody_dn5 = assign3590_e5593_d_n5;
        locals.var_pparam_b4soirbody_dn6 = assign3590_e5593_d_n6;
        locals.var_pparam_b4soirbody_dn7 = assign3590_e5593_d_n7;
        locals.var_pparam_b4soirbody_dn8 = assign3590_e5593_d_n8;
        locals.var_pparam_b4soirbody_dn9 = assign3590_e5593_d_n9;
        locals.var_pparam_b4soirbody_dn10 = assign3590_e5593_d_n10;
        locals.var_pparam_b4soirbody_dn11 = assign3590_e5593_d_n11;
        locals.var_pparam_b4soirbody_dn12 = assign3590_e5593_d_n12;

        let assign3600_e5596: f64 = (p.p380 / p.p376);
        let assign3600_e5598: f64 = (assign3600_e5596).powf(p.p379);
        let __rspice_inv_cse_0: f64 = 1.0 / p.p376;
        let assign3600_e5600: f64 = (assign3600_e5598 * __rspice_inv_cse_0);
        let assign3600_e5602: f64 = (assign3600_e5600 * __rspice_inv_cse_0);
        locals.var_pparam_b4soioxideratio = assign3600_e5602;

        let assign3610_e5606: f64 = (locals.var_pparam_b4soiua1 * locals.var_trm1);
        let assign3610_e5607: f64 = (locals.var_pparam_b4soiua + assign3610_e5606);
        locals.var_pparam_b4soiua = assign3610_e5607;
        locals.var_pparam_b4soiua_dn3 = (locals.var_pparam_b4soiua_dn3 + (locals.var_pparam_b4soiua1_dn3 * locals.var_trm1));
        locals.var_pparam_b4soiua_dn4 = (locals.var_pparam_b4soiua_dn4 + ((locals.var_pparam_b4soiua1_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiua1 * locals.var_trm1_dn4)));
        locals.var_pparam_b4soiua_dn5 = (locals.var_pparam_b4soiua_dn5 + ((locals.var_pparam_b4soiua1_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiua1 * locals.var_trm1_dn5)));
        locals.var_pparam_b4soiua_dn6 = (locals.var_pparam_b4soiua_dn6 + ((locals.var_pparam_b4soiua1_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiua1 * locals.var_trm1_dn6)));
        locals.var_pparam_b4soiua_dn7 = (locals.var_pparam_b4soiua_dn7 + (locals.var_pparam_b4soiua1_dn7 * locals.var_trm1));
        locals.var_pparam_b4soiua_dn8 = (locals.var_pparam_b4soiua_dn8 + (locals.var_pparam_b4soiua1_dn8 * locals.var_trm1));
        locals.var_pparam_b4soiua_dn9 = (locals.var_pparam_b4soiua_dn9 + (locals.var_pparam_b4soiua1_dn9 * locals.var_trm1));
        locals.var_pparam_b4soiua_dn10 = (locals.var_pparam_b4soiua_dn10 + (locals.var_pparam_b4soiua1_dn10 * locals.var_trm1));
        locals.var_pparam_b4soiua_dn11 = (locals.var_pparam_b4soiua_dn11 + (locals.var_pparam_b4soiua1_dn11 * locals.var_trm1));
        locals.var_pparam_b4soiua_dn12 = (locals.var_pparam_b4soiua_dn12 + (locals.var_pparam_b4soiua1_dn12 * locals.var_trm1));

        let assign3620_e5611: f64 = (locals.var_pparam_b4soiub1 * locals.var_trm1);
        let assign3620_e5612: f64 = (locals.var_pparam_b4soiub + assign3620_e5611);
        locals.var_pparam_b4soiub = assign3620_e5612;
        locals.var_pparam_b4soiub_dn3 = (locals.var_pparam_b4soiub_dn3 + (locals.var_pparam_b4soiub1_dn3 * locals.var_trm1));
        locals.var_pparam_b4soiub_dn4 = (locals.var_pparam_b4soiub_dn4 + ((locals.var_pparam_b4soiub1_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiub1 * locals.var_trm1_dn4)));
        locals.var_pparam_b4soiub_dn5 = (locals.var_pparam_b4soiub_dn5 + ((locals.var_pparam_b4soiub1_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiub1 * locals.var_trm1_dn5)));
        locals.var_pparam_b4soiub_dn6 = (locals.var_pparam_b4soiub_dn6 + ((locals.var_pparam_b4soiub1_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiub1 * locals.var_trm1_dn6)));
        locals.var_pparam_b4soiub_dn7 = (locals.var_pparam_b4soiub_dn7 + (locals.var_pparam_b4soiub1_dn7 * locals.var_trm1));
        locals.var_pparam_b4soiub_dn8 = (locals.var_pparam_b4soiub_dn8 + (locals.var_pparam_b4soiub1_dn8 * locals.var_trm1));
        locals.var_pparam_b4soiub_dn9 = (locals.var_pparam_b4soiub_dn9 + (locals.var_pparam_b4soiub1_dn9 * locals.var_trm1));
        locals.var_pparam_b4soiub_dn10 = (locals.var_pparam_b4soiub_dn10 + (locals.var_pparam_b4soiub1_dn10 * locals.var_trm1));
        locals.var_pparam_b4soiub_dn11 = (locals.var_pparam_b4soiub_dn11 + (locals.var_pparam_b4soiub1_dn11 * locals.var_trm1));
        locals.var_pparam_b4soiub_dn12 = (locals.var_pparam_b4soiub_dn12 + (locals.var_pparam_b4soiub1_dn12 * locals.var_trm1));

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign3630_e5616: f64 = (locals.var_pparam_b4soiuc1 * locals.var_trm1);
        let assign3630_e5617: f64 = (locals.var_pparam_b4soiuc + assign3630_e5616);
        locals.var_pparam_b4soiuc = assign3630_e5617;
        locals.var_pparam_b4soiuc_dn3 = (locals.var_pparam_b4soiuc_dn3 + (locals.var_pparam_b4soiuc1_dn3 * locals.var_trm1));
        locals.var_pparam_b4soiuc_dn4 = (locals.var_pparam_b4soiuc_dn4 + ((locals.var_pparam_b4soiuc1_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiuc1 * locals.var_trm1_dn4)));
        locals.var_pparam_b4soiuc_dn5 = (locals.var_pparam_b4soiuc_dn5 + ((locals.var_pparam_b4soiuc1_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiuc1 * locals.var_trm1_dn5)));
        locals.var_pparam_b4soiuc_dn6 = (locals.var_pparam_b4soiuc_dn6 + ((locals.var_pparam_b4soiuc1_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiuc1 * locals.var_trm1_dn6)));
        locals.var_pparam_b4soiuc_dn7 = (locals.var_pparam_b4soiuc_dn7 + (locals.var_pparam_b4soiuc1_dn7 * locals.var_trm1));
        locals.var_pparam_b4soiuc_dn8 = (locals.var_pparam_b4soiuc_dn8 + (locals.var_pparam_b4soiuc1_dn8 * locals.var_trm1));
        locals.var_pparam_b4soiuc_dn9 = (locals.var_pparam_b4soiuc_dn9 + (locals.var_pparam_b4soiuc1_dn9 * locals.var_trm1));
        locals.var_pparam_b4soiuc_dn10 = (locals.var_pparam_b4soiuc_dn10 + (locals.var_pparam_b4soiuc1_dn10 * locals.var_trm1));
        locals.var_pparam_b4soiuc_dn11 = (locals.var_pparam_b4soiuc_dn11 + (locals.var_pparam_b4soiuc1_dn11 * locals.var_trm1));
        locals.var_pparam_b4soiuc_dn12 = (locals.var_pparam_b4soiuc_dn12 + (locals.var_pparam_b4soiuc1_dn12 * locals.var_trm1));

        let assign3640_e5620: f64 = if locals.var_pparam_b4soiu0 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard513 = assign3640_e5620;

        let (assign3650_e5626, assign3650_e5626_d_n3, assign3650_e5626_d_n4, assign3650_e5626_d_n5, assign3650_e5626_d_n6, assign3650_e5626_d_n7, assign3650_e5626_d_n8, assign3650_e5626_d_n9, assign3650_e5626_d_n10, assign3650_e5626_d_n11, assign3650_e5626_d_n12,) = {
    if (locals.var_guard513 != 0.0) {
        let assign3650_e5624: f64 = (locals.var_pparam_b4soiu0 / 10000.0);
        (assign3650_e5624, (locals.var_pparam_b4soiu0_dn3 / 10000.0), (locals.var_pparam_b4soiu0_dn4 / 10000.0), (locals.var_pparam_b4soiu0_dn5 / 10000.0), (locals.var_pparam_b4soiu0_dn6 / 10000.0), (locals.var_pparam_b4soiu0_dn7 / 10000.0), (locals.var_pparam_b4soiu0_dn8 / 10000.0), (locals.var_pparam_b4soiu0_dn9 / 10000.0), (locals.var_pparam_b4soiu0_dn10 / 10000.0), (locals.var_pparam_b4soiu0_dn11 / 10000.0), (locals.var_pparam_b4soiu0_dn12 / 10000.0),)
    } else {
        (locals.var_pparam_b4soiu0, locals.var_pparam_b4soiu0_dn3, locals.var_pparam_b4soiu0_dn4, locals.var_pparam_b4soiu0_dn5, locals.var_pparam_b4soiu0_dn6, locals.var_pparam_b4soiu0_dn7, locals.var_pparam_b4soiu0_dn8, locals.var_pparam_b4soiu0_dn9, locals.var_pparam_b4soiu0_dn10, locals.var_pparam_b4soiu0_dn11, locals.var_pparam_b4soiu0_dn12,)
    }
};
        locals.var_pparam_b4soiu0 = assign3650_e5626;
        locals.var_pparam_b4soiu0_dn3 = assign3650_e5626_d_n3;
        locals.var_pparam_b4soiu0_dn4 = assign3650_e5626_d_n4;
        locals.var_pparam_b4soiu0_dn5 = assign3650_e5626_d_n5;
        locals.var_pparam_b4soiu0_dn6 = assign3650_e5626_d_n6;
        locals.var_pparam_b4soiu0_dn7 = assign3650_e5626_d_n7;
        locals.var_pparam_b4soiu0_dn8 = assign3650_e5626_d_n8;
        locals.var_pparam_b4soiu0_dn9 = assign3650_e5626_d_n9;
        locals.var_pparam_b4soiu0_dn10 = assign3650_e5626_d_n10;
        locals.var_pparam_b4soiu0_dn11 = assign3650_e5626_d_n11;
        locals.var_pparam_b4soiu0_dn12 = assign3650_e5626_d_n12;

        let assign3660_e5630: f64 = (locals.var_tempratio__blk441).powf(locals.var_pparam_b4soiute);
        let assign3660_e5631: f64 = (locals.var_pparam_b4soiu0 * assign3660_e5630);
        locals.var_pparam_b4soiu0temp = assign3660_e5631;
        locals.var_pparam_b4soiu0temp_dn3 = ((locals.var_pparam_b4soiu0_dn3 * assign3660_e5630) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn3 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign3660_e5630 * (locals.var_pparam_b4soiute_dn3 * (locals.var_tempratio__blk441).ln())) }));
        locals.var_pparam_b4soiu0temp_dn4 = ((locals.var_pparam_b4soiu0_dn4 * assign3660_e5630) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn4 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { if locals.var_pparam_b4soiute == 0.0 { 0.0 } else { (locals.var_pparam_b4soiute * ((locals.var_tempratio__blk441).powf(locals.var_pparam_b4soiute - 1.0) * locals.var_tempratio__blk441_dn4)) } } else { (assign3660_e5630 * ((locals.var_pparam_b4soiute_dn4 * (locals.var_tempratio__blk441).ln()) + (locals.var_pparam_b4soiute * (locals.var_tempratio__blk441_dn4 / locals.var_tempratio__blk441)))) }));
        locals.var_pparam_b4soiu0temp_dn5 = ((locals.var_pparam_b4soiu0_dn5 * assign3660_e5630) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn5 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { if locals.var_pparam_b4soiute == 0.0 { 0.0 } else { (locals.var_pparam_b4soiute * ((locals.var_tempratio__blk441).powf(locals.var_pparam_b4soiute - 1.0) * locals.var_tempratio__blk441_dn5)) } } else { (assign3660_e5630 * ((locals.var_pparam_b4soiute_dn5 * (locals.var_tempratio__blk441).ln()) + (locals.var_pparam_b4soiute * (locals.var_tempratio__blk441_dn5 / locals.var_tempratio__blk441)))) }));
        locals.var_pparam_b4soiu0temp_dn6 = ((locals.var_pparam_b4soiu0_dn6 * assign3660_e5630) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn6 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { if locals.var_pparam_b4soiute == 0.0 { 0.0 } else { (locals.var_pparam_b4soiute * ((locals.var_tempratio__blk441).powf(locals.var_pparam_b4soiute - 1.0) * locals.var_tempratio__blk441_dn6)) } } else { (assign3660_e5630 * ((locals.var_pparam_b4soiute_dn6 * (locals.var_tempratio__blk441).ln()) + (locals.var_pparam_b4soiute * (locals.var_tempratio__blk441_dn6 / locals.var_tempratio__blk441)))) }));
        locals.var_pparam_b4soiu0temp_dn7 = ((locals.var_pparam_b4soiu0_dn7 * assign3660_e5630) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn7 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign3660_e5630 * (locals.var_pparam_b4soiute_dn7 * (locals.var_tempratio__blk441).ln())) }));
        locals.var_pparam_b4soiu0temp_dn8 = ((locals.var_pparam_b4soiu0_dn8 * assign3660_e5630) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn8 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign3660_e5630 * (locals.var_pparam_b4soiute_dn8 * (locals.var_tempratio__blk441).ln())) }));
        locals.var_pparam_b4soiu0temp_dn9 = ((locals.var_pparam_b4soiu0_dn9 * assign3660_e5630) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn9 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign3660_e5630 * (locals.var_pparam_b4soiute_dn9 * (locals.var_tempratio__blk441).ln())) }));
        locals.var_pparam_b4soiu0temp_dn10 = ((locals.var_pparam_b4soiu0_dn10 * assign3660_e5630) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn10 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign3660_e5630 * (locals.var_pparam_b4soiute_dn10 * (locals.var_tempratio__blk441).ln())) }));
        locals.var_pparam_b4soiu0temp_dn11 = ((locals.var_pparam_b4soiu0_dn11 * assign3660_e5630) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn11 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign3660_e5630 * (locals.var_pparam_b4soiute_dn11 * (locals.var_tempratio__blk441).ln())) }));
        locals.var_pparam_b4soiu0temp_dn12 = ((locals.var_pparam_b4soiu0_dn12 * assign3660_e5630) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn12 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign3660_e5630 * (locals.var_pparam_b4soiute_dn12 * (locals.var_tempratio__blk441).ln())) }));

        let assign3670_e5635: f64 = (locals.var_pparam_b4soiat * locals.var_trm1);
        let assign3670_e5636: f64 = (locals.var_pparam_b4soivsat - assign3670_e5635);
        locals.var_pparam_b4soivsattemp = assign3670_e5636;
        locals.var_pparam_b4soivsattemp_dn3 = (locals.var_pparam_b4soivsat_dn3 - (locals.var_pparam_b4soiat_dn3 * locals.var_trm1));
        locals.var_pparam_b4soivsattemp_dn4 = (locals.var_pparam_b4soivsat_dn4 - ((locals.var_pparam_b4soiat_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiat * locals.var_trm1_dn4)));
        locals.var_pparam_b4soivsattemp_dn5 = (locals.var_pparam_b4soivsat_dn5 - ((locals.var_pparam_b4soiat_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiat * locals.var_trm1_dn5)));
        locals.var_pparam_b4soivsattemp_dn6 = (locals.var_pparam_b4soivsat_dn6 - ((locals.var_pparam_b4soiat_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiat * locals.var_trm1_dn6)));
        locals.var_pparam_b4soivsattemp_dn7 = (locals.var_pparam_b4soivsat_dn7 - (locals.var_pparam_b4soiat_dn7 * locals.var_trm1));
        locals.var_pparam_b4soivsattemp_dn8 = (locals.var_pparam_b4soivsat_dn8 - (locals.var_pparam_b4soiat_dn8 * locals.var_trm1));
        locals.var_pparam_b4soivsattemp_dn9 = (locals.var_pparam_b4soivsat_dn9 - (locals.var_pparam_b4soiat_dn9 * locals.var_trm1));
        locals.var_pparam_b4soivsattemp_dn10 = (locals.var_pparam_b4soivsat_dn10 - (locals.var_pparam_b4soiat_dn10 * locals.var_trm1));
        locals.var_pparam_b4soivsattemp_dn11 = (locals.var_pparam_b4soivsat_dn11 - (locals.var_pparam_b4soiat_dn11 * locals.var_trm1));
        locals.var_pparam_b4soivsattemp_dn12 = (locals.var_pparam_b4soivsat_dn12 - (locals.var_pparam_b4soiat_dn12 * locals.var_trm1));

        let assign3680_e5640: f64 = (locals.var_pparam_b4soiprt * locals.var_trm1);
        let assign3680_e5641: f64 = (locals.var_pparam_b4soirdsw + assign3680_e5640);
        let assign3680_e5643: f64 = (assign3680_e5641 / locals.var_pparam_b4soirds0denom);
        locals.var_pparam_b4soirds0 = assign3680_e5643;
        locals.var_pparam_b4soirds0_dn3 = ((((locals.var_pparam_b4soirdsw_dn3 + (locals.var_pparam_b4soiprt_dn3 * locals.var_trm1)) * locals.var_pparam_b4soirds0denom) - (assign3680_e5641 * locals.var_pparam_b4soirds0denom_dn3)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom));
        locals.var_pparam_b4soirds0_dn4 = ((((locals.var_pparam_b4soirdsw_dn4 + ((locals.var_pparam_b4soiprt_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiprt * locals.var_trm1_dn4))) * locals.var_pparam_b4soirds0denom) - (assign3680_e5641 * locals.var_pparam_b4soirds0denom_dn4)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom));
        locals.var_pparam_b4soirds0_dn5 = ((((locals.var_pparam_b4soirdsw_dn5 + ((locals.var_pparam_b4soiprt_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiprt * locals.var_trm1_dn5))) * locals.var_pparam_b4soirds0denom) - (assign3680_e5641 * locals.var_pparam_b4soirds0denom_dn5)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom));
        locals.var_pparam_b4soirds0_dn6 = ((((locals.var_pparam_b4soirdsw_dn6 + ((locals.var_pparam_b4soiprt_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiprt * locals.var_trm1_dn6))) * locals.var_pparam_b4soirds0denom) - (assign3680_e5641 * locals.var_pparam_b4soirds0denom_dn6)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom));
        locals.var_pparam_b4soirds0_dn7 = ((((locals.var_pparam_b4soirdsw_dn7 + (locals.var_pparam_b4soiprt_dn7 * locals.var_trm1)) * locals.var_pparam_b4soirds0denom) - (assign3680_e5641 * locals.var_pparam_b4soirds0denom_dn7)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom));
        locals.var_pparam_b4soirds0_dn8 = ((((locals.var_pparam_b4soirdsw_dn8 + (locals.var_pparam_b4soiprt_dn8 * locals.var_trm1)) * locals.var_pparam_b4soirds0denom) - (assign3680_e5641 * locals.var_pparam_b4soirds0denom_dn8)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom));
        locals.var_pparam_b4soirds0_dn9 = ((((locals.var_pparam_b4soirdsw_dn9 + (locals.var_pparam_b4soiprt_dn9 * locals.var_trm1)) * locals.var_pparam_b4soirds0denom) - (assign3680_e5641 * locals.var_pparam_b4soirds0denom_dn9)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom));
        locals.var_pparam_b4soirds0_dn10 = ((((locals.var_pparam_b4soirdsw_dn10 + (locals.var_pparam_b4soiprt_dn10 * locals.var_trm1)) * locals.var_pparam_b4soirds0denom) - (assign3680_e5641 * locals.var_pparam_b4soirds0denom_dn10)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom));
        locals.var_pparam_b4soirds0_dn11 = ((((locals.var_pparam_b4soirdsw_dn11 + (locals.var_pparam_b4soiprt_dn11 * locals.var_trm1)) * locals.var_pparam_b4soirds0denom) - (assign3680_e5641 * locals.var_pparam_b4soirds0denom_dn11)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom));
        locals.var_pparam_b4soirds0_dn12 = ((((locals.var_pparam_b4soirdsw_dn12 + (locals.var_pparam_b4soiprt_dn12 * locals.var_trm1)) * locals.var_pparam_b4soirds0denom) - (assign3680_e5641 * locals.var_pparam_b4soirds0denom_dn12)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom));

        let assign3690_e5646: f64 = if p.p429 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard514 = assign3690_e5646;

        let (assign3700_e5652, assign3700_e5652_d_n3, assign3700_e5652_d_n4, assign3700_e5652_d_n5, assign3700_e5652_d_n6, assign3700_e5652_d_n7, assign3700_e5652_d_n8, assign3700_e5652_d_n9, assign3700_e5652_d_n10, assign3700_e5652_d_n11, assign3700_e5652_d_n12,) = {
    if (locals.var_guard514 != 0.0) {
        let assign3700_e5650: f64 = (locals.var_pparam_b4soirds0denom * p.p3);
        (assign3700_e5650, (locals.var_pparam_b4soirds0denom_dn3 * p.p3), (locals.var_pparam_b4soirds0denom_dn4 * p.p3), (locals.var_pparam_b4soirds0denom_dn5 * p.p3), (locals.var_pparam_b4soirds0denom_dn6 * p.p3), (locals.var_pparam_b4soirds0denom_dn7 * p.p3), (locals.var_pparam_b4soirds0denom_dn8 * p.p3), (locals.var_pparam_b4soirds0denom_dn9 * p.p3), (locals.var_pparam_b4soirds0denom_dn10 * p.p3), (locals.var_pparam_b4soirds0denom_dn11 * p.p3), (locals.var_pparam_b4soirds0denom_dn12 * p.p3),)
    } else {
        (locals.var_powweffwr, locals.var_powweffwr_dn3, locals.var_powweffwr_dn4, locals.var_powweffwr_dn5, locals.var_powweffwr_dn6, locals.var_powweffwr_dn7, locals.var_powweffwr_dn8, locals.var_powweffwr_dn9, locals.var_powweffwr_dn10, locals.var_powweffwr_dn11, locals.var_powweffwr_dn12,)
    }
};
        locals.var_powweffwr = assign3700_e5652;
        locals.var_powweffwr_dn3 = assign3700_e5652_d_n3;
        locals.var_powweffwr_dn4 = assign3700_e5652_d_n4;
        locals.var_powweffwr_dn5 = assign3700_e5652_d_n5;
        locals.var_powweffwr_dn6 = assign3700_e5652_d_n6;
        locals.var_powweffwr_dn7 = assign3700_e5652_d_n7;
        locals.var_powweffwr_dn8 = assign3700_e5652_d_n8;
        locals.var_powweffwr_dn9 = assign3700_e5652_d_n9;
        locals.var_powweffwr_dn10 = assign3700_e5652_d_n10;
        locals.var_powweffwr_dn11 = assign3700_e5652_d_n11;
        locals.var_powweffwr_dn12 = assign3700_e5652_d_n12;

        let (assign3710_e5658, assign3710_e5658_d_n3, assign3710_e5658_d_n4, assign3710_e5658_d_n5, assign3710_e5658_d_n6, assign3710_e5658_d_n7, assign3710_e5658_d_n8, assign3710_e5658_d_n9, assign3710_e5658_d_n10, assign3710_e5658_d_n11, assign3710_e5658_d_n12,) = {
    if (locals.var_guard514 != 0.0) {
        let assign3710_e5656: f64 = (locals.var_pparam_b4soiprt * locals.var_trm1);
        (assign3710_e5656, (locals.var_pparam_b4soiprt_dn3 * locals.var_trm1), ((locals.var_pparam_b4soiprt_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiprt * locals.var_trm1_dn4)), ((locals.var_pparam_b4soiprt_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiprt * locals.var_trm1_dn5)), ((locals.var_pparam_b4soiprt_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiprt * locals.var_trm1_dn6)), (locals.var_pparam_b4soiprt_dn7 * locals.var_trm1), (locals.var_pparam_b4soiprt_dn8 * locals.var_trm1), (locals.var_pparam_b4soiprt_dn9 * locals.var_trm1), (locals.var_pparam_b4soiprt_dn10 * locals.var_trm1), (locals.var_pparam_b4soiprt_dn11 * locals.var_trm1), (locals.var_pparam_b4soiprt_dn12 * locals.var_trm1),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    }
};
        locals.var_t10 = assign3710_e5658;
        locals.var_t10_dn3 = assign3710_e5658_d_n3;
        locals.var_t10_dn4 = assign3710_e5658_d_n4;
        locals.var_t10_dn5 = assign3710_e5658_d_n5;
        locals.var_t10_dn6 = assign3710_e5658_d_n6;
        locals.var_t10_dn7 = assign3710_e5658_d_n7;
        locals.var_t10_dn8 = assign3710_e5658_d_n8;
        locals.var_t10_dn9 = assign3710_e5658_d_n9;
        locals.var_t10_dn10 = assign3710_e5658_d_n10;
        locals.var_t10_dn11 = assign3710_e5658_d_n11;
        locals.var_t10_dn12 = assign3710_e5658_d_n12;

        let (assign3720_e5664, assign3720_e5664_d_n3, assign3720_e5664_d_n4, assign3720_e5664_d_n5, assign3720_e5664_d_n6, assign3720_e5664_d_n7, assign3720_e5664_d_n8, assign3720_e5664_d_n9, assign3720_e5664_d_n10, assign3720_e5664_d_n11, assign3720_e5664_d_n12,) = {
    if (locals.var_guard514 != 0.0) {
        let assign3720_e5662: f64 = (locals.var_pparam_b4soirdw + locals.var_t10);
        (assign3720_e5662, (locals.var_pparam_b4soirdw_dn3 + locals.var_t10_dn3), (locals.var_pparam_b4soirdw_dn4 + locals.var_t10_dn4), (locals.var_pparam_b4soirdw_dn5 + locals.var_t10_dn5), (locals.var_pparam_b4soirdw_dn6 + locals.var_t10_dn6), (locals.var_pparam_b4soirdw_dn7 + locals.var_t10_dn7), (locals.var_pparam_b4soirdw_dn8 + locals.var_t10_dn8), (locals.var_pparam_b4soirdw_dn9 + locals.var_t10_dn9), (locals.var_pparam_b4soirdw_dn10 + locals.var_t10_dn10), (locals.var_pparam_b4soirdw_dn11 + locals.var_t10_dn11), (locals.var_pparam_b4soirdw_dn12 + locals.var_t10_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign3720_e5664;
        locals.var_t1_dn3 = assign3720_e5664_d_n3;
        locals.var_t1_dn4 = assign3720_e5664_d_n4;
        locals.var_t1_dn5 = assign3720_e5664_d_n5;
        locals.var_t1_dn6 = assign3720_e5664_d_n6;
        locals.var_t1_dn7 = assign3720_e5664_d_n7;
        locals.var_t1_dn8 = assign3720_e5664_d_n8;
        locals.var_t1_dn9 = assign3720_e5664_d_n9;
        locals.var_t1_dn10 = assign3720_e5664_d_n10;
        locals.var_t1_dn11 = assign3720_e5664_d_n11;
        locals.var_t1_dn12 = assign3720_e5664_d_n12;

        let (assign3730_e5670, assign3730_e5670_d_n3, assign3730_e5670_d_n4, assign3730_e5670_d_n5, assign3730_e5670_d_n6, assign3730_e5670_d_n7, assign3730_e5670_d_n8, assign3730_e5670_d_n9, assign3730_e5670_d_n10, assign3730_e5670_d_n11, assign3730_e5670_d_n12,) = {
    if (locals.var_guard514 != 0.0) {
        let assign3730_e5668: f64 = (p.p140 + locals.var_t10);
        (assign3730_e5668, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign3730_e5670;
        locals.var_t2_dn3 = assign3730_e5670_d_n3;
        locals.var_t2_dn4 = assign3730_e5670_d_n4;
        locals.var_t2_dn5 = assign3730_e5670_d_n5;
        locals.var_t2_dn6 = assign3730_e5670_d_n6;
        locals.var_t2_dn7 = assign3730_e5670_d_n7;
        locals.var_t2_dn8 = assign3730_e5670_d_n8;
        locals.var_t2_dn9 = assign3730_e5670_d_n9;
        locals.var_t2_dn10 = assign3730_e5670_d_n10;
        locals.var_t2_dn11 = assign3730_e5670_d_n11;
        locals.var_t2_dn12 = assign3730_e5670_d_n12;

        let assign3740_e5673: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard515 = assign3740_e5673;

        let (assign3750_e5679, assign3750_e5679_d_n3, assign3750_e5679_d_n4, assign3750_e5679_d_n5, assign3750_e5679_d_n6, assign3750_e5679_d_n7, assign3750_e5679_d_n8, assign3750_e5679_d_n9, assign3750_e5679_d_n10, assign3750_e5679_d_n11, assign3750_e5679_d_n12,) = {
    if ((locals.var_guard514 != 0.0) && (locals.var_guard515 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign3750_e5679;
        locals.var_t1_dn3 = assign3750_e5679_d_n3;
        locals.var_t1_dn4 = assign3750_e5679_d_n4;
        locals.var_t1_dn5 = assign3750_e5679_d_n5;
        locals.var_t1_dn6 = assign3750_e5679_d_n6;
        locals.var_t1_dn7 = assign3750_e5679_d_n7;
        locals.var_t1_dn8 = assign3750_e5679_d_n8;
        locals.var_t1_dn9 = assign3750_e5679_d_n9;
        locals.var_t1_dn10 = assign3750_e5679_d_n10;
        locals.var_t1_dn11 = assign3750_e5679_d_n11;
        locals.var_t1_dn12 = assign3750_e5679_d_n12;

        let assign3760_e5682: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard516 = assign3760_e5682;

        let (assign3770_e5688, assign3770_e5688_d_n3, assign3770_e5688_d_n4, assign3770_e5688_d_n5, assign3770_e5688_d_n6, assign3770_e5688_d_n7, assign3770_e5688_d_n8, assign3770_e5688_d_n9, assign3770_e5688_d_n10, assign3770_e5688_d_n11, assign3770_e5688_d_n12,) = {
    if ((locals.var_guard514 != 0.0) && (locals.var_guard516 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign3770_e5688;
        locals.var_t2_dn3 = assign3770_e5688_d_n3;
        locals.var_t2_dn4 = assign3770_e5688_d_n4;
        locals.var_t2_dn5 = assign3770_e5688_d_n5;
        locals.var_t2_dn6 = assign3770_e5688_d_n6;
        locals.var_t2_dn7 = assign3770_e5688_d_n7;
        locals.var_t2_dn8 = assign3770_e5688_d_n8;
        locals.var_t2_dn9 = assign3770_e5688_d_n9;
        locals.var_t2_dn10 = assign3770_e5688_d_n10;
        locals.var_t2_dn11 = assign3770_e5688_d_n11;
        locals.var_t2_dn12 = assign3770_e5688_d_n12;

        let (assign3780_e5694, assign3780_e5694_d_n3, assign3780_e5694_d_n4, assign3780_e5694_d_n5, assign3780_e5694_d_n6, assign3780_e5694_d_n7, assign3780_e5694_d_n8, assign3780_e5694_d_n9, assign3780_e5694_d_n10, assign3780_e5694_d_n11, assign3780_e5694_d_n12,) = {
    if (locals.var_guard514 != 0.0) {
        let assign3780_e5692: f64 = (locals.var_t1 / locals.var_powweffwr);
        (assign3780_e5692, (((locals.var_t1_dn3 * locals.var_powweffwr) - (locals.var_t1 * locals.var_powweffwr_dn3)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t1_dn4 * locals.var_powweffwr) - (locals.var_t1 * locals.var_powweffwr_dn4)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t1_dn5 * locals.var_powweffwr) - (locals.var_t1 * locals.var_powweffwr_dn5)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t1_dn6 * locals.var_powweffwr) - (locals.var_t1 * locals.var_powweffwr_dn6)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t1_dn7 * locals.var_powweffwr) - (locals.var_t1 * locals.var_powweffwr_dn7)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t1_dn8 * locals.var_powweffwr) - (locals.var_t1 * locals.var_powweffwr_dn8)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t1_dn9 * locals.var_powweffwr) - (locals.var_t1 * locals.var_powweffwr_dn9)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t1_dn10 * locals.var_powweffwr) - (locals.var_t1 * locals.var_powweffwr_dn10)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t1_dn11 * locals.var_powweffwr) - (locals.var_t1 * locals.var_powweffwr_dn11)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t1_dn12 * locals.var_powweffwr) - (locals.var_t1 * locals.var_powweffwr_dn12)) / (locals.var_powweffwr * locals.var_powweffwr)),)
    } else {
        (locals.var_pparam_b4soird0, locals.var_pparam_b4soird0_dn3, locals.var_pparam_b4soird0_dn4, locals.var_pparam_b4soird0_dn5, locals.var_pparam_b4soird0_dn6, locals.var_pparam_b4soird0_dn7, locals.var_pparam_b4soird0_dn8, locals.var_pparam_b4soird0_dn9, locals.var_pparam_b4soird0_dn10, locals.var_pparam_b4soird0_dn11, locals.var_pparam_b4soird0_dn12,)
    }
};
        locals.var_pparam_b4soird0 = assign3780_e5694;
        locals.var_pparam_b4soird0_dn3 = assign3780_e5694_d_n3;
        locals.var_pparam_b4soird0_dn4 = assign3780_e5694_d_n4;
        locals.var_pparam_b4soird0_dn5 = assign3780_e5694_d_n5;
        locals.var_pparam_b4soird0_dn6 = assign3780_e5694_d_n6;
        locals.var_pparam_b4soird0_dn7 = assign3780_e5694_d_n7;
        locals.var_pparam_b4soird0_dn8 = assign3780_e5694_d_n8;
        locals.var_pparam_b4soird0_dn9 = assign3780_e5694_d_n9;
        locals.var_pparam_b4soird0_dn10 = assign3780_e5694_d_n10;
        locals.var_pparam_b4soird0_dn11 = assign3780_e5694_d_n11;
        locals.var_pparam_b4soird0_dn12 = assign3780_e5694_d_n12;

        let (assign3790_e5700, assign3790_e5700_d_n3, assign3790_e5700_d_n4, assign3790_e5700_d_n5, assign3790_e5700_d_n6, assign3790_e5700_d_n7, assign3790_e5700_d_n8, assign3790_e5700_d_n9, assign3790_e5700_d_n10, assign3790_e5700_d_n11, assign3790_e5700_d_n12,) = {
    if (locals.var_guard514 != 0.0) {
        let assign3790_e5698: f64 = (locals.var_t2 / locals.var_powweffwr);
        (assign3790_e5698, (((locals.var_t2_dn3 * locals.var_powweffwr) - (locals.var_t2 * locals.var_powweffwr_dn3)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t2_dn4 * locals.var_powweffwr) - (locals.var_t2 * locals.var_powweffwr_dn4)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t2_dn5 * locals.var_powweffwr) - (locals.var_t2 * locals.var_powweffwr_dn5)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t2_dn6 * locals.var_powweffwr) - (locals.var_t2 * locals.var_powweffwr_dn6)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t2_dn7 * locals.var_powweffwr) - (locals.var_t2 * locals.var_powweffwr_dn7)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t2_dn8 * locals.var_powweffwr) - (locals.var_t2 * locals.var_powweffwr_dn8)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t2_dn9 * locals.var_powweffwr) - (locals.var_t2 * locals.var_powweffwr_dn9)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t2_dn10 * locals.var_powweffwr) - (locals.var_t2 * locals.var_powweffwr_dn10)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t2_dn11 * locals.var_powweffwr) - (locals.var_t2 * locals.var_powweffwr_dn11)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t2_dn12 * locals.var_powweffwr) - (locals.var_t2 * locals.var_powweffwr_dn12)) / (locals.var_powweffwr * locals.var_powweffwr)),)
    } else {
        (locals.var_pparam_b4soirdwmin, locals.var_pparam_b4soirdwmin_dn3, locals.var_pparam_b4soirdwmin_dn4, locals.var_pparam_b4soirdwmin_dn5, locals.var_pparam_b4soirdwmin_dn6, locals.var_pparam_b4soirdwmin_dn7, locals.var_pparam_b4soirdwmin_dn8, locals.var_pparam_b4soirdwmin_dn9, locals.var_pparam_b4soirdwmin_dn10, locals.var_pparam_b4soirdwmin_dn11, locals.var_pparam_b4soirdwmin_dn12,)
    }
};
        locals.var_pparam_b4soirdwmin = assign3790_e5700;
        locals.var_pparam_b4soirdwmin_dn3 = assign3790_e5700_d_n3;
        locals.var_pparam_b4soirdwmin_dn4 = assign3790_e5700_d_n4;
        locals.var_pparam_b4soirdwmin_dn5 = assign3790_e5700_d_n5;
        locals.var_pparam_b4soirdwmin_dn6 = assign3790_e5700_d_n6;
        locals.var_pparam_b4soirdwmin_dn7 = assign3790_e5700_d_n7;
        locals.var_pparam_b4soirdwmin_dn8 = assign3790_e5700_d_n8;
        locals.var_pparam_b4soirdwmin_dn9 = assign3790_e5700_d_n9;
        locals.var_pparam_b4soirdwmin_dn10 = assign3790_e5700_d_n10;
        locals.var_pparam_b4soirdwmin_dn11 = assign3790_e5700_d_n11;
        locals.var_pparam_b4soirdwmin_dn12 = assign3790_e5700_d_n12;

        let (assign3800_e5706, assign3800_e5706_d_n3, assign3800_e5706_d_n4, assign3800_e5706_d_n5, assign3800_e5706_d_n6, assign3800_e5706_d_n7, assign3800_e5706_d_n8, assign3800_e5706_d_n9, assign3800_e5706_d_n10, assign3800_e5706_d_n11, assign3800_e5706_d_n12,) = {
    if (locals.var_guard514 != 0.0) {
        let assign3800_e5704: f64 = (locals.var_pparam_b4soirsw + locals.var_t10);
        (assign3800_e5704, (locals.var_pparam_b4soirsw_dn3 + locals.var_t10_dn3), (locals.var_pparam_b4soirsw_dn4 + locals.var_t10_dn4), (locals.var_pparam_b4soirsw_dn5 + locals.var_t10_dn5), (locals.var_pparam_b4soirsw_dn6 + locals.var_t10_dn6), (locals.var_pparam_b4soirsw_dn7 + locals.var_t10_dn7), (locals.var_pparam_b4soirsw_dn8 + locals.var_t10_dn8), (locals.var_pparam_b4soirsw_dn9 + locals.var_t10_dn9), (locals.var_pparam_b4soirsw_dn10 + locals.var_t10_dn10), (locals.var_pparam_b4soirsw_dn11 + locals.var_t10_dn11), (locals.var_pparam_b4soirsw_dn12 + locals.var_t10_dn12),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign3800_e5706;
        locals.var_t3_dn3 = assign3800_e5706_d_n3;
        locals.var_t3_dn4 = assign3800_e5706_d_n4;
        locals.var_t3_dn5 = assign3800_e5706_d_n5;
        locals.var_t3_dn6 = assign3800_e5706_d_n6;
        locals.var_t3_dn7 = assign3800_e5706_d_n7;
        locals.var_t3_dn8 = assign3800_e5706_d_n8;
        locals.var_t3_dn9 = assign3800_e5706_d_n9;
        locals.var_t3_dn10 = assign3800_e5706_d_n10;
        locals.var_t3_dn11 = assign3800_e5706_d_n11;
        locals.var_t3_dn12 = assign3800_e5706_d_n12;

        let (assign3810_e5712, assign3810_e5712_d_n3, assign3810_e5712_d_n4, assign3810_e5712_d_n5, assign3810_e5712_d_n6, assign3810_e5712_d_n7, assign3810_e5712_d_n8, assign3810_e5712_d_n9, assign3810_e5712_d_n10, assign3810_e5712_d_n11, assign3810_e5712_d_n12,) = {
    if (locals.var_guard514 != 0.0) {
        let assign3810_e5710: f64 = (p.p139 + locals.var_t10);
        (assign3810_e5710, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign3810_e5712;
        locals.var_t4_dn3 = assign3810_e5712_d_n3;
        locals.var_t4_dn4 = assign3810_e5712_d_n4;
        locals.var_t4_dn5 = assign3810_e5712_d_n5;
        locals.var_t4_dn6 = assign3810_e5712_d_n6;
        locals.var_t4_dn7 = assign3810_e5712_d_n7;
        locals.var_t4_dn8 = assign3810_e5712_d_n8;
        locals.var_t4_dn9 = assign3810_e5712_d_n9;
        locals.var_t4_dn10 = assign3810_e5712_d_n10;
        locals.var_t4_dn11 = assign3810_e5712_d_n11;
        locals.var_t4_dn12 = assign3810_e5712_d_n12;

        let assign3820_e5715: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard517 = assign3820_e5715;

        let (assign3830_e5721, assign3830_e5721_d_n3, assign3830_e5721_d_n4, assign3830_e5721_d_n5, assign3830_e5721_d_n6, assign3830_e5721_d_n7, assign3830_e5721_d_n8, assign3830_e5721_d_n9, assign3830_e5721_d_n10, assign3830_e5721_d_n11, assign3830_e5721_d_n12,) = {
    if ((locals.var_guard514 != 0.0) && (locals.var_guard517 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign3830_e5721;
        locals.var_t3_dn3 = assign3830_e5721_d_n3;
        locals.var_t3_dn4 = assign3830_e5721_d_n4;
        locals.var_t3_dn5 = assign3830_e5721_d_n5;
        locals.var_t3_dn6 = assign3830_e5721_d_n6;
        locals.var_t3_dn7 = assign3830_e5721_d_n7;
        locals.var_t3_dn8 = assign3830_e5721_d_n8;
        locals.var_t3_dn9 = assign3830_e5721_d_n9;
        locals.var_t3_dn10 = assign3830_e5721_d_n10;
        locals.var_t3_dn11 = assign3830_e5721_d_n11;
        locals.var_t3_dn12 = assign3830_e5721_d_n12;

        let assign3840_e5724: f64 = if locals.var_t4 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard518 = assign3840_e5724;

        let (assign3850_e5730, assign3850_e5730_d_n3, assign3850_e5730_d_n4, assign3850_e5730_d_n5, assign3850_e5730_d_n6, assign3850_e5730_d_n7, assign3850_e5730_d_n8, assign3850_e5730_d_n9, assign3850_e5730_d_n10, assign3850_e5730_d_n11, assign3850_e5730_d_n12,) = {
    if ((locals.var_guard514 != 0.0) && (locals.var_guard518 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign3850_e5730;
        locals.var_t4_dn3 = assign3850_e5730_d_n3;
        locals.var_t4_dn4 = assign3850_e5730_d_n4;
        locals.var_t4_dn5 = assign3850_e5730_d_n5;
        locals.var_t4_dn6 = assign3850_e5730_d_n6;
        locals.var_t4_dn7 = assign3850_e5730_d_n7;
        locals.var_t4_dn8 = assign3850_e5730_d_n8;
        locals.var_t4_dn9 = assign3850_e5730_d_n9;
        locals.var_t4_dn10 = assign3850_e5730_d_n10;
        locals.var_t4_dn11 = assign3850_e5730_d_n11;
        locals.var_t4_dn12 = assign3850_e5730_d_n12;

        let (assign3860_e5736, assign3860_e5736_d_n3, assign3860_e5736_d_n4, assign3860_e5736_d_n5, assign3860_e5736_d_n6, assign3860_e5736_d_n7, assign3860_e5736_d_n8, assign3860_e5736_d_n9, assign3860_e5736_d_n10, assign3860_e5736_d_n11, assign3860_e5736_d_n12,) = {
    if (locals.var_guard514 != 0.0) {
        let assign3860_e5734: f64 = (locals.var_t3 / locals.var_powweffwr);
        (assign3860_e5734, (((locals.var_t3_dn3 * locals.var_powweffwr) - (locals.var_t3 * locals.var_powweffwr_dn3)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t3_dn4 * locals.var_powweffwr) - (locals.var_t3 * locals.var_powweffwr_dn4)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t3_dn5 * locals.var_powweffwr) - (locals.var_t3 * locals.var_powweffwr_dn5)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t3_dn6 * locals.var_powweffwr) - (locals.var_t3 * locals.var_powweffwr_dn6)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t3_dn7 * locals.var_powweffwr) - (locals.var_t3 * locals.var_powweffwr_dn7)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t3_dn8 * locals.var_powweffwr) - (locals.var_t3 * locals.var_powweffwr_dn8)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t3_dn9 * locals.var_powweffwr) - (locals.var_t3 * locals.var_powweffwr_dn9)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t3_dn10 * locals.var_powweffwr) - (locals.var_t3 * locals.var_powweffwr_dn10)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t3_dn11 * locals.var_powweffwr) - (locals.var_t3 * locals.var_powweffwr_dn11)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t3_dn12 * locals.var_powweffwr) - (locals.var_t3 * locals.var_powweffwr_dn12)) / (locals.var_powweffwr * locals.var_powweffwr)),)
    } else {
        (locals.var_pparam_b4soirs0, locals.var_pparam_b4soirs0_dn3, locals.var_pparam_b4soirs0_dn4, locals.var_pparam_b4soirs0_dn5, locals.var_pparam_b4soirs0_dn6, locals.var_pparam_b4soirs0_dn7, locals.var_pparam_b4soirs0_dn8, locals.var_pparam_b4soirs0_dn9, locals.var_pparam_b4soirs0_dn10, locals.var_pparam_b4soirs0_dn11, locals.var_pparam_b4soirs0_dn12,)
    }
};
        locals.var_pparam_b4soirs0 = assign3860_e5736;
        locals.var_pparam_b4soirs0_dn3 = assign3860_e5736_d_n3;
        locals.var_pparam_b4soirs0_dn4 = assign3860_e5736_d_n4;
        locals.var_pparam_b4soirs0_dn5 = assign3860_e5736_d_n5;
        locals.var_pparam_b4soirs0_dn6 = assign3860_e5736_d_n6;
        locals.var_pparam_b4soirs0_dn7 = assign3860_e5736_d_n7;
        locals.var_pparam_b4soirs0_dn8 = assign3860_e5736_d_n8;
        locals.var_pparam_b4soirs0_dn9 = assign3860_e5736_d_n9;
        locals.var_pparam_b4soirs0_dn10 = assign3860_e5736_d_n10;
        locals.var_pparam_b4soirs0_dn11 = assign3860_e5736_d_n11;
        locals.var_pparam_b4soirs0_dn12 = assign3860_e5736_d_n12;

        let (assign3870_e5742, assign3870_e5742_d_n3, assign3870_e5742_d_n4, assign3870_e5742_d_n5, assign3870_e5742_d_n6, assign3870_e5742_d_n7, assign3870_e5742_d_n8, assign3870_e5742_d_n9, assign3870_e5742_d_n10, assign3870_e5742_d_n11, assign3870_e5742_d_n12,) = {
    if (locals.var_guard514 != 0.0) {
        let assign3870_e5740: f64 = (locals.var_t4 / locals.var_powweffwr);
        (assign3870_e5740, (((locals.var_t4_dn3 * locals.var_powweffwr) - (locals.var_t4 * locals.var_powweffwr_dn3)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t4_dn4 * locals.var_powweffwr) - (locals.var_t4 * locals.var_powweffwr_dn4)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t4_dn5 * locals.var_powweffwr) - (locals.var_t4 * locals.var_powweffwr_dn5)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t4_dn6 * locals.var_powweffwr) - (locals.var_t4 * locals.var_powweffwr_dn6)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t4_dn7 * locals.var_powweffwr) - (locals.var_t4 * locals.var_powweffwr_dn7)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t4_dn8 * locals.var_powweffwr) - (locals.var_t4 * locals.var_powweffwr_dn8)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t4_dn9 * locals.var_powweffwr) - (locals.var_t4 * locals.var_powweffwr_dn9)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t4_dn10 * locals.var_powweffwr) - (locals.var_t4 * locals.var_powweffwr_dn10)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t4_dn11 * locals.var_powweffwr) - (locals.var_t4 * locals.var_powweffwr_dn11)) / (locals.var_powweffwr * locals.var_powweffwr)), (((locals.var_t4_dn12 * locals.var_powweffwr) - (locals.var_t4 * locals.var_powweffwr_dn12)) / (locals.var_powweffwr * locals.var_powweffwr)),)
    } else {
        (locals.var_pparam_b4soirswmin, locals.var_pparam_b4soirswmin_dn3, locals.var_pparam_b4soirswmin_dn4, locals.var_pparam_b4soirswmin_dn5, locals.var_pparam_b4soirswmin_dn6, locals.var_pparam_b4soirswmin_dn7, locals.var_pparam_b4soirswmin_dn8, locals.var_pparam_b4soirswmin_dn9, locals.var_pparam_b4soirswmin_dn10, locals.var_pparam_b4soirswmin_dn11, locals.var_pparam_b4soirswmin_dn12,)
    }
};
        locals.var_pparam_b4soirswmin = assign3870_e5742;
        locals.var_pparam_b4soirswmin_dn3 = assign3870_e5742_d_n3;
        locals.var_pparam_b4soirswmin_dn4 = assign3870_e5742_d_n4;
        locals.var_pparam_b4soirswmin_dn5 = assign3870_e5742_d_n5;
        locals.var_pparam_b4soirswmin_dn6 = assign3870_e5742_d_n6;
        locals.var_pparam_b4soirswmin_dn7 = assign3870_e5742_d_n7;
        locals.var_pparam_b4soirswmin_dn8 = assign3870_e5742_d_n8;
        locals.var_pparam_b4soirswmin_dn9 = assign3870_e5742_d_n9;
        locals.var_pparam_b4soirswmin_dn10 = assign3870_e5742_d_n10;
        locals.var_pparam_b4soirswmin_dn11 = assign3870_e5742_d_n11;
        locals.var_pparam_b4soirswmin_dn12 = assign3870_e5742_d_n12;

        let (assign3880_e5747, assign3880_e5747_d_n3, assign3880_e5747_d_n4, assign3880_e5747_d_n5, assign3880_e5747_d_n6, assign3880_e5747_d_n7, assign3880_e5747_d_n8, assign3880_e5747_d_n9, assign3880_e5747_d_n10, assign3880_e5747_d_n11, assign3880_e5747_d_n12,) = {
    if (locals.var_guard514 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soird0, locals.var_pparam_b4soird0_dn3, locals.var_pparam_b4soird0_dn4, locals.var_pparam_b4soird0_dn5, locals.var_pparam_b4soird0_dn6, locals.var_pparam_b4soird0_dn7, locals.var_pparam_b4soird0_dn8, locals.var_pparam_b4soird0_dn9, locals.var_pparam_b4soird0_dn10, locals.var_pparam_b4soird0_dn11, locals.var_pparam_b4soird0_dn12,)
    }
};
        locals.var_pparam_b4soird0 = assign3880_e5747;
        locals.var_pparam_b4soird0_dn3 = assign3880_e5747_d_n3;
        locals.var_pparam_b4soird0_dn4 = assign3880_e5747_d_n4;
        locals.var_pparam_b4soird0_dn5 = assign3880_e5747_d_n5;
        locals.var_pparam_b4soird0_dn6 = assign3880_e5747_d_n6;
        locals.var_pparam_b4soird0_dn7 = assign3880_e5747_d_n7;
        locals.var_pparam_b4soird0_dn8 = assign3880_e5747_d_n8;
        locals.var_pparam_b4soird0_dn9 = assign3880_e5747_d_n9;
        locals.var_pparam_b4soird0_dn10 = assign3880_e5747_d_n10;
        locals.var_pparam_b4soird0_dn11 = assign3880_e5747_d_n11;
        locals.var_pparam_b4soird0_dn12 = assign3880_e5747_d_n12;

        let (assign3890_e5752, assign3890_e5752_d_n3, assign3890_e5752_d_n4, assign3890_e5752_d_n5, assign3890_e5752_d_n6, assign3890_e5752_d_n7, assign3890_e5752_d_n8, assign3890_e5752_d_n9, assign3890_e5752_d_n10, assign3890_e5752_d_n11, assign3890_e5752_d_n12,) = {
    if (locals.var_guard514 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soirdwmin, locals.var_pparam_b4soirdwmin_dn3, locals.var_pparam_b4soirdwmin_dn4, locals.var_pparam_b4soirdwmin_dn5, locals.var_pparam_b4soirdwmin_dn6, locals.var_pparam_b4soirdwmin_dn7, locals.var_pparam_b4soirdwmin_dn8, locals.var_pparam_b4soirdwmin_dn9, locals.var_pparam_b4soirdwmin_dn10, locals.var_pparam_b4soirdwmin_dn11, locals.var_pparam_b4soirdwmin_dn12,)
    }
};
        locals.var_pparam_b4soirdwmin = assign3890_e5752;
        locals.var_pparam_b4soirdwmin_dn3 = assign3890_e5752_d_n3;
        locals.var_pparam_b4soirdwmin_dn4 = assign3890_e5752_d_n4;
        locals.var_pparam_b4soirdwmin_dn5 = assign3890_e5752_d_n5;
        locals.var_pparam_b4soirdwmin_dn6 = assign3890_e5752_d_n6;
        locals.var_pparam_b4soirdwmin_dn7 = assign3890_e5752_d_n7;
        locals.var_pparam_b4soirdwmin_dn8 = assign3890_e5752_d_n8;
        locals.var_pparam_b4soirdwmin_dn9 = assign3890_e5752_d_n9;
        locals.var_pparam_b4soirdwmin_dn10 = assign3890_e5752_d_n10;
        locals.var_pparam_b4soirdwmin_dn11 = assign3890_e5752_d_n11;
        locals.var_pparam_b4soirdwmin_dn12 = assign3890_e5752_d_n12;

        let (assign3900_e5757, assign3900_e5757_d_n3, assign3900_e5757_d_n4, assign3900_e5757_d_n5, assign3900_e5757_d_n6, assign3900_e5757_d_n7, assign3900_e5757_d_n8, assign3900_e5757_d_n9, assign3900_e5757_d_n10, assign3900_e5757_d_n11, assign3900_e5757_d_n12,) = {
    if (locals.var_guard514 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soirs0, locals.var_pparam_b4soirs0_dn3, locals.var_pparam_b4soirs0_dn4, locals.var_pparam_b4soirs0_dn5, locals.var_pparam_b4soirs0_dn6, locals.var_pparam_b4soirs0_dn7, locals.var_pparam_b4soirs0_dn8, locals.var_pparam_b4soirs0_dn9, locals.var_pparam_b4soirs0_dn10, locals.var_pparam_b4soirs0_dn11, locals.var_pparam_b4soirs0_dn12,)
    }
};
        locals.var_pparam_b4soirs0 = assign3900_e5757;
        locals.var_pparam_b4soirs0_dn3 = assign3900_e5757_d_n3;
        locals.var_pparam_b4soirs0_dn4 = assign3900_e5757_d_n4;
        locals.var_pparam_b4soirs0_dn5 = assign3900_e5757_d_n5;
        locals.var_pparam_b4soirs0_dn6 = assign3900_e5757_d_n6;
        locals.var_pparam_b4soirs0_dn7 = assign3900_e5757_d_n7;
        locals.var_pparam_b4soirs0_dn8 = assign3900_e5757_d_n8;
        locals.var_pparam_b4soirs0_dn9 = assign3900_e5757_d_n9;
        locals.var_pparam_b4soirs0_dn10 = assign3900_e5757_d_n10;
        locals.var_pparam_b4soirs0_dn11 = assign3900_e5757_d_n11;
        locals.var_pparam_b4soirs0_dn12 = assign3900_e5757_d_n12;

        let (assign3910_e5762, assign3910_e5762_d_n3, assign3910_e5762_d_n4, assign3910_e5762_d_n5, assign3910_e5762_d_n6, assign3910_e5762_d_n7, assign3910_e5762_d_n8, assign3910_e5762_d_n9, assign3910_e5762_d_n10, assign3910_e5762_d_n11, assign3910_e5762_d_n12,) = {
    if (locals.var_guard514 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soirswmin, locals.var_pparam_b4soirswmin_dn3, locals.var_pparam_b4soirswmin_dn4, locals.var_pparam_b4soirswmin_dn5, locals.var_pparam_b4soirswmin_dn6, locals.var_pparam_b4soirswmin_dn7, locals.var_pparam_b4soirswmin_dn8, locals.var_pparam_b4soirswmin_dn9, locals.var_pparam_b4soirswmin_dn10, locals.var_pparam_b4soirswmin_dn11, locals.var_pparam_b4soirswmin_dn12,)
    }
};
        locals.var_pparam_b4soirswmin = assign3910_e5762;
        locals.var_pparam_b4soirswmin_dn3 = assign3910_e5762_d_n3;
        locals.var_pparam_b4soirswmin_dn4 = assign3910_e5762_d_n4;
        locals.var_pparam_b4soirswmin_dn5 = assign3910_e5762_d_n5;
        locals.var_pparam_b4soirswmin_dn6 = assign3910_e5762_d_n6;
        locals.var_pparam_b4soirswmin_dn7 = assign3910_e5762_d_n7;
        locals.var_pparam_b4soirswmin_dn8 = assign3910_e5762_d_n8;
        locals.var_pparam_b4soirswmin_dn9 = assign3910_e5762_d_n9;
        locals.var_pparam_b4soirswmin_dn10 = assign3910_e5762_d_n10;
        locals.var_pparam_b4soirswmin_dn11 = assign3910_e5762_d_n11;
        locals.var_pparam_b4soirswmin_dn12 = assign3910_e5762_d_n12;

        let assign3920_e5764: f64 = if param_given[128] { 1.0 } else { 0.0 };
        locals.var_guard519 = assign3920_e5764;

        let (assign3930_e5768, assign3930_e5768_d_n3, assign3930_e5768_d_n4, assign3930_e5768_d_n5, assign3930_e5768_d_n6, assign3930_e5768_d_n7, assign3930_e5768_d_n8, assign3930_e5768_d_n9, assign3930_e5768_d_n10, assign3930_e5768_d_n11, assign3930_e5768_d_n12,) = {
    if (locals.var_guard519 != 0.0) {
        (p.p128, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soicgdo, locals.var_b4soicgdo_dn3, locals.var_b4soicgdo_dn4, locals.var_b4soicgdo_dn5, locals.var_b4soicgdo_dn6, locals.var_b4soicgdo_dn7, locals.var_b4soicgdo_dn8, locals.var_b4soicgdo_dn9, locals.var_b4soicgdo_dn10, locals.var_b4soicgdo_dn11, locals.var_b4soicgdo_dn12,)
    }
};
        locals.var_b4soicgdo = assign3930_e5768;
        locals.var_b4soicgdo_dn3 = assign3930_e5768_d_n3;
        locals.var_b4soicgdo_dn4 = assign3930_e5768_d_n4;
        locals.var_b4soicgdo_dn5 = assign3930_e5768_d_n5;
        locals.var_b4soicgdo_dn6 = assign3930_e5768_d_n6;
        locals.var_b4soicgdo_dn7 = assign3930_e5768_d_n7;
        locals.var_b4soicgdo_dn8 = assign3930_e5768_d_n8;
        locals.var_b4soicgdo_dn9 = assign3930_e5768_d_n9;
        locals.var_b4soicgdo_dn10 = assign3930_e5768_d_n10;
        locals.var_b4soicgdo_dn11 = assign3930_e5768_d_n11;
        locals.var_b4soicgdo_dn12 = assign3930_e5768_d_n12;

        let assign3940_e5774: f64 = if (param_given[217] && (p.p217 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard520 = assign3940_e5774;

        let (assign3950_e5785, assign3950_e5785_d_n3, assign3950_e5785_d_n4, assign3950_e5785_d_n5, assign3950_e5785_d_n6, assign3950_e5785_d_n7, assign3950_e5785_d_n8, assign3950_e5785_d_n9, assign3950_e5785_d_n10, assign3950_e5785_d_n11, assign3950_e5785_d_n12,) = {
    if ((locals.var_guard519 == 0.0) && (locals.var_guard520 != 0.0)) {
        let assign3950_e5781: f64 = (p.p217 * locals.var_b4soicox);
        let assign3950_e5783: f64 = (assign3950_e5781 - locals.var_pparam_b4soicgdl);
        (assign3950_e5783, (-locals.var_pparam_b4soicgdl_dn3), (-locals.var_pparam_b4soicgdl_dn4), (-locals.var_pparam_b4soicgdl_dn5), (-locals.var_pparam_b4soicgdl_dn6), (-locals.var_pparam_b4soicgdl_dn7), (-locals.var_pparam_b4soicgdl_dn8), (-locals.var_pparam_b4soicgdl_dn9), (-locals.var_pparam_b4soicgdl_dn10), (-locals.var_pparam_b4soicgdl_dn11), (-locals.var_pparam_b4soicgdl_dn12),)
    } else {
        (locals.var_b4soicgdo, locals.var_b4soicgdo_dn3, locals.var_b4soicgdo_dn4, locals.var_b4soicgdo_dn5, locals.var_b4soicgdo_dn6, locals.var_b4soicgdo_dn7, locals.var_b4soicgdo_dn8, locals.var_b4soicgdo_dn9, locals.var_b4soicgdo_dn10, locals.var_b4soicgdo_dn11, locals.var_b4soicgdo_dn12,)
    }
};
        locals.var_b4soicgdo = assign3950_e5785;
        locals.var_b4soicgdo_dn3 = assign3950_e5785_d_n3;
        locals.var_b4soicgdo_dn4 = assign3950_e5785_d_n4;
        locals.var_b4soicgdo_dn5 = assign3950_e5785_d_n5;
        locals.var_b4soicgdo_dn6 = assign3950_e5785_d_n6;
        locals.var_b4soicgdo_dn7 = assign3950_e5785_d_n7;
        locals.var_b4soicgdo_dn8 = assign3950_e5785_d_n8;
        locals.var_b4soicgdo_dn9 = assign3950_e5785_d_n9;
        locals.var_b4soicgdo_dn10 = assign3950_e5785_d_n10;
        locals.var_b4soicgdo_dn11 = assign3950_e5785_d_n11;
        locals.var_b4soicgdo_dn12 = assign3950_e5785_d_n12;

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign3960_e5797, assign3960_e5797_d_n3, assign3960_e5797_d_n4, assign3960_e5797_d_n5, assign3960_e5797_d_n6, assign3960_e5797_d_n7, assign3960_e5797_d_n8, assign3960_e5797_d_n9, assign3960_e5797_d_n10, assign3960_e5797_d_n11, assign3960_e5797_d_n12,) = {
    if ((locals.var_guard519 == 0.0) && (locals.var_guard520 == 0.0)) {
        let assign3960_e5793: f64 = (0.6 * p.p157);
        let assign3960_e5795: f64 = (assign3960_e5793 * locals.var_b4soicox);
        (assign3960_e5795, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soicgdo, locals.var_b4soicgdo_dn3, locals.var_b4soicgdo_dn4, locals.var_b4soicgdo_dn5, locals.var_b4soicgdo_dn6, locals.var_b4soicgdo_dn7, locals.var_b4soicgdo_dn8, locals.var_b4soicgdo_dn9, locals.var_b4soicgdo_dn10, locals.var_b4soicgdo_dn11, locals.var_b4soicgdo_dn12,)
    }
};
        locals.var_b4soicgdo = assign3960_e5797;
        locals.var_b4soicgdo_dn3 = assign3960_e5797_d_n3;
        locals.var_b4soicgdo_dn4 = assign3960_e5797_d_n4;
        locals.var_b4soicgdo_dn5 = assign3960_e5797_d_n5;
        locals.var_b4soicgdo_dn6 = assign3960_e5797_d_n6;
        locals.var_b4soicgdo_dn7 = assign3960_e5797_d_n7;
        locals.var_b4soicgdo_dn8 = assign3960_e5797_d_n8;
        locals.var_b4soicgdo_dn9 = assign3960_e5797_d_n9;
        locals.var_b4soicgdo_dn10 = assign3960_e5797_d_n10;
        locals.var_b4soicgdo_dn11 = assign3960_e5797_d_n11;
        locals.var_b4soicgdo_dn12 = assign3960_e5797_d_n12;

        let assign3970_e5799: f64 = if param_given[127] { 1.0 } else { 0.0 };
        locals.var_guard521 = assign3970_e5799;

        let (assign3980_e5803, assign3980_e5803_d_n3, assign3980_e5803_d_n4, assign3980_e5803_d_n5, assign3980_e5803_d_n6, assign3980_e5803_d_n7, assign3980_e5803_d_n8, assign3980_e5803_d_n9, assign3980_e5803_d_n10, assign3980_e5803_d_n11, assign3980_e5803_d_n12,) = {
    if (locals.var_guard521 != 0.0) {
        (p.p127, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soicgso, locals.var_b4soicgso_dn3, locals.var_b4soicgso_dn4, locals.var_b4soicgso_dn5, locals.var_b4soicgso_dn6, locals.var_b4soicgso_dn7, locals.var_b4soicgso_dn8, locals.var_b4soicgso_dn9, locals.var_b4soicgso_dn10, locals.var_b4soicgso_dn11, locals.var_b4soicgso_dn12,)
    }
};
        locals.var_b4soicgso = assign3980_e5803;
        locals.var_b4soicgso_dn3 = assign3980_e5803_d_n3;
        locals.var_b4soicgso_dn4 = assign3980_e5803_d_n4;
        locals.var_b4soicgso_dn5 = assign3980_e5803_d_n5;
        locals.var_b4soicgso_dn6 = assign3980_e5803_d_n6;
        locals.var_b4soicgso_dn7 = assign3980_e5803_d_n7;
        locals.var_b4soicgso_dn8 = assign3980_e5803_d_n8;
        locals.var_b4soicgso_dn9 = assign3980_e5803_d_n9;
        locals.var_b4soicgso_dn10 = assign3980_e5803_d_n10;
        locals.var_b4soicgso_dn11 = assign3980_e5803_d_n11;
        locals.var_b4soicgso_dn12 = assign3980_e5803_d_n12;

        let assign3990_e5809: f64 = if (param_given[217] && (p.p217 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard522 = assign3990_e5809;

        let (assign4000_e5820, assign4000_e5820_d_n3, assign4000_e5820_d_n4, assign4000_e5820_d_n5, assign4000_e5820_d_n6, assign4000_e5820_d_n7, assign4000_e5820_d_n8, assign4000_e5820_d_n9, assign4000_e5820_d_n10, assign4000_e5820_d_n11, assign4000_e5820_d_n12,) = {
    if ((locals.var_guard521 == 0.0) && (locals.var_guard522 != 0.0)) {
        let assign4000_e5816: f64 = (p.p217 * locals.var_b4soicox);
        let assign4000_e5818: f64 = (assign4000_e5816 - locals.var_pparam_b4soicgsl);
        (assign4000_e5818, (-locals.var_pparam_b4soicgsl_dn3), (-locals.var_pparam_b4soicgsl_dn4), (-locals.var_pparam_b4soicgsl_dn5), (-locals.var_pparam_b4soicgsl_dn6), (-locals.var_pparam_b4soicgsl_dn7), (-locals.var_pparam_b4soicgsl_dn8), (-locals.var_pparam_b4soicgsl_dn9), (-locals.var_pparam_b4soicgsl_dn10), (-locals.var_pparam_b4soicgsl_dn11), (-locals.var_pparam_b4soicgsl_dn12),)
    } else {
        (locals.var_b4soicgso, locals.var_b4soicgso_dn3, locals.var_b4soicgso_dn4, locals.var_b4soicgso_dn5, locals.var_b4soicgso_dn6, locals.var_b4soicgso_dn7, locals.var_b4soicgso_dn8, locals.var_b4soicgso_dn9, locals.var_b4soicgso_dn10, locals.var_b4soicgso_dn11, locals.var_b4soicgso_dn12,)
    }
};
        locals.var_b4soicgso = assign4000_e5820;
        locals.var_b4soicgso_dn3 = assign4000_e5820_d_n3;
        locals.var_b4soicgso_dn4 = assign4000_e5820_d_n4;
        locals.var_b4soicgso_dn5 = assign4000_e5820_d_n5;
        locals.var_b4soicgso_dn6 = assign4000_e5820_d_n6;
        locals.var_b4soicgso_dn7 = assign4000_e5820_d_n7;
        locals.var_b4soicgso_dn8 = assign4000_e5820_d_n8;
        locals.var_b4soicgso_dn9 = assign4000_e5820_d_n9;
        locals.var_b4soicgso_dn10 = assign4000_e5820_d_n10;
        locals.var_b4soicgso_dn11 = assign4000_e5820_d_n11;
        locals.var_b4soicgso_dn12 = assign4000_e5820_d_n12;

        let (assign4010_e5832, assign4010_e5832_d_n3, assign4010_e5832_d_n4, assign4010_e5832_d_n5, assign4010_e5832_d_n6, assign4010_e5832_d_n7, assign4010_e5832_d_n8, assign4010_e5832_d_n9, assign4010_e5832_d_n10, assign4010_e5832_d_n11, assign4010_e5832_d_n12,) = {
    if ((locals.var_guard521 == 0.0) && (locals.var_guard522 == 0.0)) {
        let assign4010_e5828: f64 = (0.6 * p.p157);
        let assign4010_e5830: f64 = (assign4010_e5828 * locals.var_b4soicox);
        (assign4010_e5830, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soicgso, locals.var_b4soicgso_dn3, locals.var_b4soicgso_dn4, locals.var_b4soicgso_dn5, locals.var_b4soicgso_dn6, locals.var_b4soicgso_dn7, locals.var_b4soicgso_dn8, locals.var_b4soicgso_dn9, locals.var_b4soicgso_dn10, locals.var_b4soicgso_dn11, locals.var_b4soicgso_dn12,)
    }
};
        locals.var_b4soicgso = assign4010_e5832;
        locals.var_b4soicgso_dn3 = assign4010_e5832_d_n3;
        locals.var_b4soicgso_dn4 = assign4010_e5832_d_n4;
        locals.var_b4soicgso_dn5 = assign4010_e5832_d_n5;
        locals.var_b4soicgso_dn6 = assign4010_e5832_d_n6;
        locals.var_b4soicgso_dn7 = assign4010_e5832_d_n7;
        locals.var_b4soicgso_dn8 = assign4010_e5832_d_n8;
        locals.var_b4soicgso_dn9 = assign4010_e5832_d_n9;
        locals.var_b4soicgso_dn10 = assign4010_e5832_d_n10;
        locals.var_b4soicgso_dn11 = assign4010_e5832_d_n11;
        locals.var_b4soicgso_dn12 = assign4010_e5832_d_n12;

        let assign4020_e5835: f64 = if locals.var_b4soicgdo < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard523 = assign4020_e5835;

        let (assign4030_e5839, assign4030_e5839_d_n3, assign4030_e5839_d_n4, assign4030_e5839_d_n5, assign4030_e5839_d_n6, assign4030_e5839_d_n7, assign4030_e5839_d_n8, assign4030_e5839_d_n9, assign4030_e5839_d_n10, assign4030_e5839_d_n11, assign4030_e5839_d_n12,) = {
    if (locals.var_guard523 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soicgdo, locals.var_b4soicgdo_dn3, locals.var_b4soicgdo_dn4, locals.var_b4soicgdo_dn5, locals.var_b4soicgdo_dn6, locals.var_b4soicgdo_dn7, locals.var_b4soicgdo_dn8, locals.var_b4soicgdo_dn9, locals.var_b4soicgdo_dn10, locals.var_b4soicgdo_dn11, locals.var_b4soicgdo_dn12,)
    }
};
        locals.var_b4soicgdo = assign4030_e5839;
        locals.var_b4soicgdo_dn3 = assign4030_e5839_d_n3;
        locals.var_b4soicgdo_dn4 = assign4030_e5839_d_n4;
        locals.var_b4soicgdo_dn5 = assign4030_e5839_d_n5;
        locals.var_b4soicgdo_dn6 = assign4030_e5839_d_n6;
        locals.var_b4soicgdo_dn7 = assign4030_e5839_d_n7;
        locals.var_b4soicgdo_dn8 = assign4030_e5839_d_n8;
        locals.var_b4soicgdo_dn9 = assign4030_e5839_d_n9;
        locals.var_b4soicgdo_dn10 = assign4030_e5839_d_n10;
        locals.var_b4soicgdo_dn11 = assign4030_e5839_d_n11;
        locals.var_b4soicgdo_dn12 = assign4030_e5839_d_n12;

        let assign4040_e5842: f64 = if locals.var_b4soicgso < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard524 = assign4040_e5842;

        let (assign4050_e5846, assign4050_e5846_d_n3, assign4050_e5846_d_n4, assign4050_e5846_d_n5, assign4050_e5846_d_n6, assign4050_e5846_d_n7, assign4050_e5846_d_n8, assign4050_e5846_d_n9, assign4050_e5846_d_n10, assign4050_e5846_d_n11, assign4050_e5846_d_n12,) = {
    if (locals.var_guard524 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soicgso, locals.var_b4soicgso_dn3, locals.var_b4soicgso_dn4, locals.var_b4soicgso_dn5, locals.var_b4soicgso_dn6, locals.var_b4soicgso_dn7, locals.var_b4soicgso_dn8, locals.var_b4soicgso_dn9, locals.var_b4soicgso_dn10, locals.var_b4soicgso_dn11, locals.var_b4soicgso_dn12,)
    }
};
        locals.var_b4soicgso = assign4050_e5846;
        locals.var_b4soicgso_dn3 = assign4050_e5846_d_n3;
        locals.var_b4soicgso_dn4 = assign4050_e5846_d_n4;
        locals.var_b4soicgso_dn5 = assign4050_e5846_d_n5;
        locals.var_b4soicgso_dn6 = assign4050_e5846_d_n6;
        locals.var_b4soicgso_dn7 = assign4050_e5846_d_n7;
        locals.var_b4soicgso_dn8 = assign4050_e5846_d_n8;
        locals.var_b4soicgso_dn9 = assign4050_e5846_d_n9;
        locals.var_b4soicgso_dn10 = assign4050_e5846_d_n10;
        locals.var_b4soicgso_dn11 = assign4050_e5846_d_n11;
        locals.var_b4soicgso_dn12 = assign4050_e5846_d_n12;

        let assign4060_e5849: f64 = if locals.var_b4soicgeo < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard525 = assign4060_e5849;

        let (assign4070_e5853,) = {
    if (locals.var_guard525 != 0.0) {
        (0.0,)
    } else {
        (locals.var_b4soicgeo,)
    }
};
        locals.var_b4soicgeo = assign4070_e5853;

        let assign4080_e5856: f64 = (locals.var_b4soicgdo + locals.var_pparam_b4soicf);
        let assign4080_e5858: f64 = (assign4080_e5856 * locals.var_pparam_b4soiwdiodcv);
        locals.var_pparam_b4soicgdo = assign4080_e5858;
        locals.var_pparam_b4soicgdo_dn3 = ((locals.var_b4soicgdo_dn3 * locals.var_pparam_b4soiwdiodcv) + (assign4080_e5856 * locals.var_pparam_b4soiwdiodcv_dn3));
        locals.var_pparam_b4soicgdo_dn4 = ((locals.var_b4soicgdo_dn4 * locals.var_pparam_b4soiwdiodcv) + (assign4080_e5856 * locals.var_pparam_b4soiwdiodcv_dn4));
        locals.var_pparam_b4soicgdo_dn5 = ((locals.var_b4soicgdo_dn5 * locals.var_pparam_b4soiwdiodcv) + (assign4080_e5856 * locals.var_pparam_b4soiwdiodcv_dn5));
        locals.var_pparam_b4soicgdo_dn6 = ((locals.var_b4soicgdo_dn6 * locals.var_pparam_b4soiwdiodcv) + (assign4080_e5856 * locals.var_pparam_b4soiwdiodcv_dn6));
        locals.var_pparam_b4soicgdo_dn7 = ((locals.var_b4soicgdo_dn7 * locals.var_pparam_b4soiwdiodcv) + (assign4080_e5856 * locals.var_pparam_b4soiwdiodcv_dn7));
        locals.var_pparam_b4soicgdo_dn8 = ((locals.var_b4soicgdo_dn8 * locals.var_pparam_b4soiwdiodcv) + (assign4080_e5856 * locals.var_pparam_b4soiwdiodcv_dn8));
        locals.var_pparam_b4soicgdo_dn9 = ((locals.var_b4soicgdo_dn9 * locals.var_pparam_b4soiwdiodcv) + (assign4080_e5856 * locals.var_pparam_b4soiwdiodcv_dn9));
        locals.var_pparam_b4soicgdo_dn10 = ((locals.var_b4soicgdo_dn10 * locals.var_pparam_b4soiwdiodcv) + (assign4080_e5856 * locals.var_pparam_b4soiwdiodcv_dn10));
        locals.var_pparam_b4soicgdo_dn11 = ((locals.var_b4soicgdo_dn11 * locals.var_pparam_b4soiwdiodcv) + (assign4080_e5856 * locals.var_pparam_b4soiwdiodcv_dn11));
        locals.var_pparam_b4soicgdo_dn12 = ((locals.var_b4soicgdo_dn12 * locals.var_pparam_b4soiwdiodcv) + (assign4080_e5856 * locals.var_pparam_b4soiwdiodcv_dn12));

        let assign4090_e5861: f64 = (locals.var_b4soicgso + locals.var_pparam_b4soicf);
        let assign4090_e5863: f64 = (assign4090_e5861 * locals.var_pparam_b4soiwdioscv);
        locals.var_pparam_b4soicgso = assign4090_e5863;
        locals.var_pparam_b4soicgso_dn3 = ((locals.var_b4soicgso_dn3 * locals.var_pparam_b4soiwdioscv) + (assign4090_e5861 * locals.var_pparam_b4soiwdioscv_dn3));
        locals.var_pparam_b4soicgso_dn4 = ((locals.var_b4soicgso_dn4 * locals.var_pparam_b4soiwdioscv) + (assign4090_e5861 * locals.var_pparam_b4soiwdioscv_dn4));
        locals.var_pparam_b4soicgso_dn5 = ((locals.var_b4soicgso_dn5 * locals.var_pparam_b4soiwdioscv) + (assign4090_e5861 * locals.var_pparam_b4soiwdioscv_dn5));
        locals.var_pparam_b4soicgso_dn6 = ((locals.var_b4soicgso_dn6 * locals.var_pparam_b4soiwdioscv) + (assign4090_e5861 * locals.var_pparam_b4soiwdioscv_dn6));
        locals.var_pparam_b4soicgso_dn7 = ((locals.var_b4soicgso_dn7 * locals.var_pparam_b4soiwdioscv) + (assign4090_e5861 * locals.var_pparam_b4soiwdioscv_dn7));
        locals.var_pparam_b4soicgso_dn8 = ((locals.var_b4soicgso_dn8 * locals.var_pparam_b4soiwdioscv) + (assign4090_e5861 * locals.var_pparam_b4soiwdioscv_dn8));
        locals.var_pparam_b4soicgso_dn9 = ((locals.var_b4soicgso_dn9 * locals.var_pparam_b4soiwdioscv) + (assign4090_e5861 * locals.var_pparam_b4soiwdioscv_dn9));
        locals.var_pparam_b4soicgso_dn10 = ((locals.var_b4soicgso_dn10 * locals.var_pparam_b4soiwdioscv) + (assign4090_e5861 * locals.var_pparam_b4soiwdioscv_dn10));
        locals.var_pparam_b4soicgso_dn11 = ((locals.var_b4soicgso_dn11 * locals.var_pparam_b4soiwdioscv) + (assign4090_e5861 * locals.var_pparam_b4soiwdioscv_dn11));
        locals.var_pparam_b4soicgso_dn12 = ((locals.var_b4soicgso_dn12 * locals.var_pparam_b4soiwdioscv) + (assign4090_e5861 * locals.var_pparam_b4soiwdioscv_dn12));

        let assign4100_e5866: f64 = (locals.var_b4soicgeo * locals.var_pparam_b4soileffcv);
        let assign4100_e5868: f64 = (assign4100_e5866 * p.p3);
        locals.var_pparam_b4soicgeo = assign4100_e5868;
        locals.var_pparam_b4soicgeo_dn3 = ((locals.var_b4soicgeo * locals.var_pparam_b4soileffcv_dn3) * p.p3);
        locals.var_pparam_b4soicgeo_dn4 = ((locals.var_b4soicgeo * locals.var_pparam_b4soileffcv_dn4) * p.p3);
        locals.var_pparam_b4soicgeo_dn5 = ((locals.var_b4soicgeo * locals.var_pparam_b4soileffcv_dn5) * p.p3);
        locals.var_pparam_b4soicgeo_dn6 = ((locals.var_b4soicgeo * locals.var_pparam_b4soileffcv_dn6) * p.p3);
        locals.var_pparam_b4soicgeo_dn7 = ((locals.var_b4soicgeo * locals.var_pparam_b4soileffcv_dn7) * p.p3);
        locals.var_pparam_b4soicgeo_dn8 = ((locals.var_b4soicgeo * locals.var_pparam_b4soileffcv_dn8) * p.p3);
        locals.var_pparam_b4soicgeo_dn9 = ((locals.var_b4soicgeo * locals.var_pparam_b4soileffcv_dn9) * p.p3);
        locals.var_pparam_b4soicgeo_dn10 = ((locals.var_b4soicgeo * locals.var_pparam_b4soileffcv_dn10) * p.p3);
        locals.var_pparam_b4soicgeo_dn11 = ((locals.var_b4soicgeo * locals.var_pparam_b4soileffcv_dn11) * p.p3);
        locals.var_pparam_b4soicgeo_dn12 = ((locals.var_b4soicgeo * locals.var_pparam_b4soileffcv_dn12) * p.p3);

        let assign4110_e5874: f64 = if ((!param_given[82]) && param_given[85]) { 1.0 } else { 0.0 };
        locals.var_guard526 = assign4110_e5874;

        let (assign4120_e5880, assign4120_e5880_d_n3, assign4120_e5880_d_n4, assign4120_e5880_d_n5, assign4120_e5880_d_n6, assign4120_e5880_d_n7, assign4120_e5880_d_n8, assign4120_e5880_d_n9, assign4120_e5880_d_n10, assign4120_e5880_d_n11, assign4120_e5880_d_n12,) = {
    if (locals.var_guard526 != 0.0) {
        let assign4120_e5878: f64 = (locals.var_pparam_b4soigamma1 * locals.var_b4soicox);
        (assign4120_e5878, (locals.var_pparam_b4soigamma1_dn3 * locals.var_b4soicox), (locals.var_pparam_b4soigamma1_dn4 * locals.var_b4soicox), (locals.var_pparam_b4soigamma1_dn5 * locals.var_b4soicox), (locals.var_pparam_b4soigamma1_dn6 * locals.var_b4soicox), (locals.var_pparam_b4soigamma1_dn7 * locals.var_b4soicox), (locals.var_pparam_b4soigamma1_dn8 * locals.var_b4soicox), (locals.var_pparam_b4soigamma1_dn9 * locals.var_b4soicox), (locals.var_pparam_b4soigamma1_dn10 * locals.var_b4soicox), (locals.var_pparam_b4soigamma1_dn11 * locals.var_b4soicox), (locals.var_pparam_b4soigamma1_dn12 * locals.var_b4soicox),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign4120_e5880;
        locals.var_t0_dn3 = assign4120_e5880_d_n3;
        locals.var_t0_dn4 = assign4120_e5880_d_n4;
        locals.var_t0_dn5 = assign4120_e5880_d_n5;
        locals.var_t0_dn6 = assign4120_e5880_d_n6;
        locals.var_t0_dn7 = assign4120_e5880_d_n7;
        locals.var_t0_dn8 = assign4120_e5880_d_n8;
        locals.var_t0_dn9 = assign4120_e5880_d_n9;
        locals.var_t0_dn10 = assign4120_e5880_d_n10;
        locals.var_t0_dn11 = assign4120_e5880_d_n11;
        locals.var_t0_dn12 = assign4120_e5880_d_n12;

        let (assign4130_e5888, assign4130_e5888_d_n3, assign4130_e5888_d_n4, assign4130_e5888_d_n5, assign4130_e5888_d_n6, assign4130_e5888_d_n7, assign4130_e5888_d_n8, assign4130_e5888_d_n9, assign4130_e5888_d_n10, assign4130_e5888_d_n11, assign4130_e5888_d_n12,) = {
    if (locals.var_guard526 != 0.0) {
        let assign4130_e5884: f64 = (3.021e22 * locals.var_t0);
        let assign4130_e5886: f64 = (assign4130_e5884 * locals.var_t0);
        (assign4130_e5886, (((3.021e22 * locals.var_t0_dn3) * locals.var_t0) + (assign4130_e5884 * locals.var_t0_dn3)), (((3.021e22 * locals.var_t0_dn4) * locals.var_t0) + (assign4130_e5884 * locals.var_t0_dn4)), (((3.021e22 * locals.var_t0_dn5) * locals.var_t0) + (assign4130_e5884 * locals.var_t0_dn5)), (((3.021e22 * locals.var_t0_dn6) * locals.var_t0) + (assign4130_e5884 * locals.var_t0_dn6)), (((3.021e22 * locals.var_t0_dn7) * locals.var_t0) + (assign4130_e5884 * locals.var_t0_dn7)), (((3.021e22 * locals.var_t0_dn8) * locals.var_t0) + (assign4130_e5884 * locals.var_t0_dn8)), (((3.021e22 * locals.var_t0_dn9) * locals.var_t0) + (assign4130_e5884 * locals.var_t0_dn9)), (((3.021e22 * locals.var_t0_dn10) * locals.var_t0) + (assign4130_e5884 * locals.var_t0_dn10)), (((3.021e22 * locals.var_t0_dn11) * locals.var_t0) + (assign4130_e5884 * locals.var_t0_dn11)), (((3.021e22 * locals.var_t0_dn12) * locals.var_t0) + (assign4130_e5884 * locals.var_t0_dn12)),)
    } else {
        (locals.var_pparam_b4soinpeak, locals.var_pparam_b4soinpeak_dn3, locals.var_pparam_b4soinpeak_dn4, locals.var_pparam_b4soinpeak_dn5, locals.var_pparam_b4soinpeak_dn6, locals.var_pparam_b4soinpeak_dn7, locals.var_pparam_b4soinpeak_dn8, locals.var_pparam_b4soinpeak_dn9, locals.var_pparam_b4soinpeak_dn10, locals.var_pparam_b4soinpeak_dn11, locals.var_pparam_b4soinpeak_dn12,)
    }
};
        locals.var_pparam_b4soinpeak = assign4130_e5888;
        locals.var_pparam_b4soinpeak_dn3 = assign4130_e5888_d_n3;
        locals.var_pparam_b4soinpeak_dn4 = assign4130_e5888_d_n4;
        locals.var_pparam_b4soinpeak_dn5 = assign4130_e5888_d_n5;
        locals.var_pparam_b4soinpeak_dn6 = assign4130_e5888_d_n6;
        locals.var_pparam_b4soinpeak_dn7 = assign4130_e5888_d_n7;
        locals.var_pparam_b4soinpeak_dn8 = assign4130_e5888_d_n8;
        locals.var_pparam_b4soinpeak_dn9 = assign4130_e5888_d_n9;
        locals.var_pparam_b4soinpeak_dn10 = assign4130_e5888_d_n10;
        locals.var_pparam_b4soinpeak_dn11 = assign4130_e5888_d_n11;
        locals.var_pparam_b4soinpeak_dn12 = assign4130_e5888_d_n12;

        let assign4140_e5891: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard527 = assign4140_e5891;

        let (assign4150_e5909,) = {
    if ((locals.var_guard527 != 0.0) && (p.p41 != 0.0)) {
        let assign4150_e5897: f64 = (p.p49 - 0.1);
        let assign4150_e5899: f64 = (assign4150_e5897 / 1.602176462e-19);
        let assign4150_e5901: f64 = (assign4150_e5899 * 2e-6);
        let assign4150_e5903: f64 = (assign4150_e5901 * locals.var_epssub);
        let assign4150_e5906: f64 = (p.p156 * p.p156);
        let assign4150_e5907: f64 = (assign4150_e5903 / assign4150_e5906);
        (assign4150_e5907,)
    } else {
        (locals.var_nchmax,)
    }
};
        locals.var_nchmax = assign4150_e5909;

        let assign4160_e5912: f64 = if locals.var_pparam_b4soinpeak > locals.var_nchmax { 1.0 } else { 0.0 };
        locals.var_guard528 = assign4160_e5912;

        let (assign4170_e5920, assign4170_e5920_d_n3, assign4170_e5920_d_n4, assign4170_e5920_d_n5, assign4170_e5920_d_n6, assign4170_e5920_d_n7, assign4170_e5920_d_n8, assign4170_e5920_d_n9, assign4170_e5920_d_n10, assign4170_e5920_d_n11, assign4170_e5920_d_n12,) = {
    if (((locals.var_guard527 != 0.0) && (p.p41 != 0.0)) && (locals.var_guard528 != 0.0)) {
        (locals.var_nchmax, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soinpeak, locals.var_pparam_b4soinpeak_dn3, locals.var_pparam_b4soinpeak_dn4, locals.var_pparam_b4soinpeak_dn5, locals.var_pparam_b4soinpeak_dn6, locals.var_pparam_b4soinpeak_dn7, locals.var_pparam_b4soinpeak_dn8, locals.var_pparam_b4soinpeak_dn9, locals.var_pparam_b4soinpeak_dn10, locals.var_pparam_b4soinpeak_dn11, locals.var_pparam_b4soinpeak_dn12,)
    }
};
        locals.var_pparam_b4soinpeak = assign4170_e5920;
        locals.var_pparam_b4soinpeak_dn3 = assign4170_e5920_d_n3;
        locals.var_pparam_b4soinpeak_dn4 = assign4170_e5920_d_n4;
        locals.var_pparam_b4soinpeak_dn5 = assign4170_e5920_d_n5;
        locals.var_pparam_b4soinpeak_dn6 = assign4170_e5920_d_n6;
        locals.var_pparam_b4soinpeak_dn7 = assign4170_e5920_d_n7;
        locals.var_pparam_b4soinpeak_dn8 = assign4170_e5920_d_n8;
        locals.var_pparam_b4soinpeak_dn9 = assign4170_e5920_d_n9;
        locals.var_pparam_b4soinpeak_dn10 = assign4170_e5920_d_n10;
        locals.var_pparam_b4soinpeak_dn11 = assign4170_e5920_d_n11;
        locals.var_pparam_b4soinpeak_dn12 = assign4170_e5920_d_n12;

        let (assign4180_e5939,) = {
    if ((locals.var_guard527 != 0.0) && (p.p41 == 0.0)) {
        let assign4180_e5927: f64 = (1.12 - 0.1);
        let assign4180_e5929: f64 = (assign4180_e5927 / 1.602176462e-19);
        let assign4180_e5931: f64 = (assign4180_e5929 * 2e-6);
        let assign4180_e5933: f64 = (assign4180_e5931 * locals.var_epssub);
        let assign4180_e5936: f64 = (p.p155 * p.p155);
        let assign4180_e5937: f64 = (assign4180_e5933 / assign4180_e5936);
        (assign4180_e5937,)
    } else {
        (locals.var_nchmax,)
    }
};
        locals.var_nchmax = assign4180_e5939;

        let assign4190_e5942: f64 = if locals.var_pparam_b4soinpeak > locals.var_nchmax { 1.0 } else { 0.0 };
        locals.var_guard529 = assign4190_e5942;

        let (assign4200_e5951, assign4200_e5951_d_n3, assign4200_e5951_d_n4, assign4200_e5951_d_n5, assign4200_e5951_d_n6, assign4200_e5951_d_n7, assign4200_e5951_d_n8, assign4200_e5951_d_n9, assign4200_e5951_d_n10, assign4200_e5951_d_n11, assign4200_e5951_d_n12,) = {
    if (((locals.var_guard527 != 0.0) && (p.p41 == 0.0)) && (locals.var_guard529 != 0.0)) {
        (locals.var_nchmax, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soinpeak, locals.var_pparam_b4soinpeak_dn3, locals.var_pparam_b4soinpeak_dn4, locals.var_pparam_b4soinpeak_dn5, locals.var_pparam_b4soinpeak_dn6, locals.var_pparam_b4soinpeak_dn7, locals.var_pparam_b4soinpeak_dn8, locals.var_pparam_b4soinpeak_dn9, locals.var_pparam_b4soinpeak_dn10, locals.var_pparam_b4soinpeak_dn11, locals.var_pparam_b4soinpeak_dn12,)
    }
};
        locals.var_pparam_b4soinpeak = assign4200_e5951;
        locals.var_pparam_b4soinpeak_dn3 = assign4200_e5951_d_n3;
        locals.var_pparam_b4soinpeak_dn4 = assign4200_e5951_d_n4;
        locals.var_pparam_b4soinpeak_dn5 = assign4200_e5951_d_n5;
        locals.var_pparam_b4soinpeak_dn6 = assign4200_e5951_d_n6;
        locals.var_pparam_b4soinpeak_dn7 = assign4200_e5951_d_n7;
        locals.var_pparam_b4soinpeak_dn8 = assign4200_e5951_d_n8;
        locals.var_pparam_b4soinpeak_dn9 = assign4200_e5951_d_n9;
        locals.var_pparam_b4soinpeak_dn10 = assign4200_e5951_d_n10;
        locals.var_pparam_b4soinpeak_dn11 = assign4200_e5951_d_n11;
        locals.var_pparam_b4soinpeak_dn12 = assign4200_e5951_d_n12;

        let assign4210_e5954: f64 = (3.453133e-11 / p.p154);
        locals.var_b4soicbox = assign4210_e5954;

        let (assign4220_e5960,) = {
    if (p.p41 != 0.0) {
        let assign4220_e5958: f64 = (1.03594e-10 / p.p156);
        (assign4220_e5958,)
    } else {
        (locals.var_b4soicsi,)
    }
};
        locals.var_b4soicsi = assign4220_e5960;

        let (assign4230_e5967,) = {
    if (p.p41 == 0.0) {
        let assign4230_e5965: f64 = (1.03594e-10 / p.p155);
        (assign4230_e5965,)
    } else {
        (locals.var_b4soicsi,)
    }
};
        locals.var_b4soicsi = assign4230_e5967;

        let (assign4240_e5983,) = {
    if (p.p41 != 0.0) {
        let assign4240_e5971: f64 = (1.602176462e-19 * locals.var_pparam_b4soinpeak);
        let assign4240_e5975: f64 = (p.p1021 / p.p1);
        let assign4240_e5976: f64 = (1.0 + assign4240_e5975);
        let assign4240_e5977: f64 = (assign4240_e5971 * assign4240_e5976);
        let assign4240_e5979: f64 = (assign4240_e5977 * 1000000.0);
        let assign4240_e5981: f64 = (assign4240_e5979 * p.p156);
        (assign4240_e5981,)
    } else {
        (locals.var_qsi,)
    }
};
        locals.var_qsi = assign4240_e5983;

        let (assign4250_e6000,) = {
    if (p.p41 == 0.0) {
        let assign4250_e5988: f64 = (1.602176462e-19 * locals.var_pparam_b4soinpeak);
        let assign4250_e5992: f64 = (p.p1021 / p.p1);
        let assign4250_e5993: f64 = (1.0 + assign4250_e5992);
        let assign4250_e5994: f64 = (assign4250_e5988 * assign4250_e5993);
        let assign4250_e5996: f64 = (assign4250_e5994 * 1000000.0);
        let assign4250_e5998: f64 = (assign4250_e5996 * p.p155);
        (assign4250_e5998,)
    } else {
        (locals.var_qsi,)
    }
};
        locals.var_qsi = assign4250_e6000;

        let assign4260_e6004: f64 = (0.5 * locals.var_qsi);
        let assign4260_e6006: f64 = (assign4260_e6004 / locals.var_b4soicsi);
        let assign4260_e6007: f64 = (0.8 - assign4260_e6006);
        let assign4260_e6009: f64 = (assign4260_e6007 + locals.var_pparam_b4soivbsa);
        locals.var_vbs0t = assign4260_e6009;

        let assign4270_e6012: f64 = if locals.var_b4soisoimod == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard530 = assign4270_e6012;

        let assign4280_e6015: f64 = if locals.var_vbs0t > locals.var_pparam_b4soivbs0fd { 1.0 } else { 0.0 };
        locals.var_guard531 = assign4280_e6015;

        let (assign4290_e6021,) = {
    if ((locals.var_guard530 != 0.0) && (locals.var_guard531 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_b4soisoimod,)
    }
};
        locals.var_b4soisoimod = assign4290_e6021;

        let assign4300_e6024: f64 = if locals.var_vbs0t < locals.var_pparam_b4soivbs0pd { 1.0 } else { 0.0 };
        locals.var_guard532 = assign4300_e6024;

        let (assign4310_e6033,) = {
    if (((locals.var_guard530 != 0.0) && (locals.var_guard531 == 0.0)) && (locals.var_guard532 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_b4soisoimod,)
    }
};
        locals.var_b4soisoimod = assign4310_e6033;

        let (assign4320_e6043,) = {
    if (((locals.var_guard530 != 0.0) && (locals.var_guard531 == 0.0)) && (locals.var_guard532 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_b4soisoimod,)
    }
};
        locals.var_b4soisoimod = assign4320_e6043;

        let assign4330_e6046: f64 = (1.115 / locals.var_b4soivtm);
        let assign4330_e6048: f64 = (assign4330_e6046 * locals.var_trm1);
        locals.var_t4 = assign4330_e6048;
        locals.var_t4_dn3 = 0.0;
        locals.var_t4_dn4 = (((-((1.115 * locals.var_b4soivtm_dn4) / (locals.var_b4soivtm * locals.var_b4soivtm))) * locals.var_trm1) + (assign4330_e6046 * locals.var_trm1_dn4));
        locals.var_t4_dn5 = (((-((1.115 * locals.var_b4soivtm_dn5) / (locals.var_b4soivtm * locals.var_b4soivtm))) * locals.var_trm1) + (assign4330_e6046 * locals.var_trm1_dn5));
        locals.var_t4_dn6 = (((-((1.115 * locals.var_b4soivtm_dn6) / (locals.var_b4soivtm * locals.var_b4soivtm))) * locals.var_trm1) + (assign4330_e6046 * locals.var_trm1_dn6));
        locals.var_t4_dn7 = 0.0;
        locals.var_t4_dn8 = 0.0;
        locals.var_t4_dn9 = 0.0;
        locals.var_t4_dn10 = 0.0;
        locals.var_t4_dn11 = 0.0;
        locals.var_t4_dn12 = 0.0;

        let assign4340_e6051: f64 = (locals.var_pparam_b4soixbjt * locals.var_t4);
        let assign4340_e6053: f64 = (assign4340_e6051 / locals.var_pparam_b4soindiode);
        locals.var_t7 = assign4340_e6053;
        locals.var_t7_dn3 = (((((locals.var_pparam_b4soixbjt_dn3 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn3)) * locals.var_pparam_b4soindiode) - (assign4340_e6051 * locals.var_pparam_b4soindiode_dn3)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn4 = (((((locals.var_pparam_b4soixbjt_dn4 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn4)) * locals.var_pparam_b4soindiode) - (assign4340_e6051 * locals.var_pparam_b4soindiode_dn4)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn5 = (((((locals.var_pparam_b4soixbjt_dn5 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn5)) * locals.var_pparam_b4soindiode) - (assign4340_e6051 * locals.var_pparam_b4soindiode_dn5)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn6 = (((((locals.var_pparam_b4soixbjt_dn6 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn6)) * locals.var_pparam_b4soindiode) - (assign4340_e6051 * locals.var_pparam_b4soindiode_dn6)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn7 = (((((locals.var_pparam_b4soixbjt_dn7 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn7)) * locals.var_pparam_b4soindiode) - (assign4340_e6051 * locals.var_pparam_b4soindiode_dn7)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn8 = (((((locals.var_pparam_b4soixbjt_dn8 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn8)) * locals.var_pparam_b4soindiode) - (assign4340_e6051 * locals.var_pparam_b4soindiode_dn8)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn9 = (((((locals.var_pparam_b4soixbjt_dn9 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn9)) * locals.var_pparam_b4soindiode) - (assign4340_e6051 * locals.var_pparam_b4soindiode_dn9)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn10 = (((((locals.var_pparam_b4soixbjt_dn10 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn10)) * locals.var_pparam_b4soindiode) - (assign4340_e6051 * locals.var_pparam_b4soindiode_dn10)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn11 = (((((locals.var_pparam_b4soixbjt_dn11 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn11)) * locals.var_pparam_b4soindiode) - (assign4340_e6051 * locals.var_pparam_b4soindiode_dn11)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn12 = (((((locals.var_pparam_b4soixbjt_dn12 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn12)) * locals.var_pparam_b4soindiode) - (assign4340_e6051 * locals.var_pparam_b4soindiode_dn12)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));

        let assign4350_e6056: f64 = if locals.var_t7 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard533 = assign4350_e6056;

        let (assign4360_e6066, assign4360_e6066_d_n3, assign4360_e6066_d_n4, assign4360_e6066_d_n5, assign4360_e6066_d_n6, assign4360_e6066_d_n7, assign4360_e6066_d_n8, assign4360_e6066_d_n9, assign4360_e6066_d_n10, assign4360_e6066_d_n11, assign4360_e6066_d_n12,) = {
    if (locals.var_guard533 != 0.0) {
        let assign4360_e6061: f64 = (1.0 + locals.var_t7);
        let assign4360_e6063: f64 = (assign4360_e6061 - 100.0);
        let assign4360_e6064: f64 = (2.688117142e43 * assign4360_e6063);
        (assign4360_e6064, (2.688117142e43 * locals.var_t7_dn3), (2.688117142e43 * locals.var_t7_dn4), (2.688117142e43 * locals.var_t7_dn5), (2.688117142e43 * locals.var_t7_dn6), (2.688117142e43 * locals.var_t7_dn7), (2.688117142e43 * locals.var_t7_dn8), (2.688117142e43 * locals.var_t7_dn9), (2.688117142e43 * locals.var_t7_dn10), (2.688117142e43 * locals.var_t7_dn11), (2.688117142e43 * locals.var_t7_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign4360_e6066;
        locals.var_t0_dn3 = assign4360_e6066_d_n3;
        locals.var_t0_dn4 = assign4360_e6066_d_n4;
        locals.var_t0_dn5 = assign4360_e6066_d_n5;
        locals.var_t0_dn6 = assign4360_e6066_d_n6;
        locals.var_t0_dn7 = assign4360_e6066_d_n7;
        locals.var_t0_dn8 = assign4360_e6066_d_n8;
        locals.var_t0_dn9 = assign4360_e6066_d_n9;
        locals.var_t0_dn10 = assign4360_e6066_d_n10;
        locals.var_t0_dn11 = assign4360_e6066_d_n11;
        locals.var_t0_dn12 = assign4360_e6066_d_n12;

        let assign4370_e6069: f64 = (-100.0);
        let assign4370_e6070: f64 = if locals.var_t7 < assign4370_e6069 { 1.0 } else { 0.0 };
        locals.var_guard534 = assign4370_e6070;

        let (assign4380_e6077, assign4380_e6077_d_n3, assign4380_e6077_d_n4, assign4380_e6077_d_n5, assign4380_e6077_d_n6, assign4380_e6077_d_n7, assign4380_e6077_d_n8, assign4380_e6077_d_n9, assign4380_e6077_d_n10, assign4380_e6077_d_n11, assign4380_e6077_d_n12,) = {
    if ((locals.var_guard533 == 0.0) && (locals.var_guard534 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign4380_e6077;
        locals.var_t0_dn3 = assign4380_e6077_d_n3;
        locals.var_t0_dn4 = assign4380_e6077_d_n4;
        locals.var_t0_dn5 = assign4380_e6077_d_n5;
        locals.var_t0_dn6 = assign4380_e6077_d_n6;
        locals.var_t0_dn7 = assign4380_e6077_d_n7;
        locals.var_t0_dn8 = assign4380_e6077_d_n8;
        locals.var_t0_dn9 = assign4380_e6077_d_n9;
        locals.var_t0_dn10 = assign4380_e6077_d_n10;
        locals.var_t0_dn11 = assign4380_e6077_d_n11;
        locals.var_t0_dn12 = assign4380_e6077_d_n12;

        let (assign4390_e6086, assign4390_e6086_d_n3, assign4390_e6086_d_n4, assign4390_e6086_d_n5, assign4390_e6086_d_n6, assign4390_e6086_d_n7, assign4390_e6086_d_n8, assign4390_e6086_d_n9, assign4390_e6086_d_n10, assign4390_e6086_d_n11, assign4390_e6086_d_n12,) = {
    if ((locals.var_guard533 == 0.0) && (locals.var_guard534 == 0.0)) {
        let assign4390_e6084: f64 = (locals.var_t7).exp();
        (assign4390_e6084, (assign4390_e6084 * locals.var_t7_dn3), (assign4390_e6084 * locals.var_t7_dn4), (assign4390_e6084 * locals.var_t7_dn5), (assign4390_e6084 * locals.var_t7_dn6), (assign4390_e6084 * locals.var_t7_dn7), (assign4390_e6084 * locals.var_t7_dn8), (assign4390_e6084 * locals.var_t7_dn9), (assign4390_e6084 * locals.var_t7_dn10), (assign4390_e6084 * locals.var_t7_dn11), (assign4390_e6084 * locals.var_t7_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign4390_e6086;
        locals.var_t0_dn3 = assign4390_e6086_d_n3;
        locals.var_t0_dn4 = assign4390_e6086_d_n4;
        locals.var_t0_dn5 = assign4390_e6086_d_n5;
        locals.var_t0_dn6 = assign4390_e6086_d_n6;
        locals.var_t0_dn7 = assign4390_e6086_d_n7;
        locals.var_t0_dn8 = assign4390_e6086_d_n8;
        locals.var_t0_dn9 = assign4390_e6086_d_n9;
        locals.var_t0_dn10 = assign4390_e6086_d_n10;
        locals.var_t0_dn11 = assign4390_e6086_d_n11;
        locals.var_t0_dn12 = assign4390_e6086_d_n12;

        let assign4400_e6089: f64 = (locals.var_pparam_b4soixdif * locals.var_t4);
        let assign4400_e6091: f64 = (assign4400_e6089 / locals.var_pparam_b4soindiode);
        locals.var_t7 = assign4400_e6091;
        locals.var_t7_dn3 = (((((locals.var_pparam_b4soixdif_dn3 * locals.var_t4) + (locals.var_pparam_b4soixdif * locals.var_t4_dn3)) * locals.var_pparam_b4soindiode) - (assign4400_e6089 * locals.var_pparam_b4soindiode_dn3)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn4 = (((((locals.var_pparam_b4soixdif_dn4 * locals.var_t4) + (locals.var_pparam_b4soixdif * locals.var_t4_dn4)) * locals.var_pparam_b4soindiode) - (assign4400_e6089 * locals.var_pparam_b4soindiode_dn4)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn5 = (((((locals.var_pparam_b4soixdif_dn5 * locals.var_t4) + (locals.var_pparam_b4soixdif * locals.var_t4_dn5)) * locals.var_pparam_b4soindiode) - (assign4400_e6089 * locals.var_pparam_b4soindiode_dn5)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn6 = (((((locals.var_pparam_b4soixdif_dn6 * locals.var_t4) + (locals.var_pparam_b4soixdif * locals.var_t4_dn6)) * locals.var_pparam_b4soindiode) - (assign4400_e6089 * locals.var_pparam_b4soindiode_dn6)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn7 = (((((locals.var_pparam_b4soixdif_dn7 * locals.var_t4) + (locals.var_pparam_b4soixdif * locals.var_t4_dn7)) * locals.var_pparam_b4soindiode) - (assign4400_e6089 * locals.var_pparam_b4soindiode_dn7)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn8 = (((((locals.var_pparam_b4soixdif_dn8 * locals.var_t4) + (locals.var_pparam_b4soixdif * locals.var_t4_dn8)) * locals.var_pparam_b4soindiode) - (assign4400_e6089 * locals.var_pparam_b4soindiode_dn8)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn9 = (((((locals.var_pparam_b4soixdif_dn9 * locals.var_t4) + (locals.var_pparam_b4soixdif * locals.var_t4_dn9)) * locals.var_pparam_b4soindiode) - (assign4400_e6089 * locals.var_pparam_b4soindiode_dn9)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn10 = (((((locals.var_pparam_b4soixdif_dn10 * locals.var_t4) + (locals.var_pparam_b4soixdif * locals.var_t4_dn10)) * locals.var_pparam_b4soindiode) - (assign4400_e6089 * locals.var_pparam_b4soindiode_dn10)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn11 = (((((locals.var_pparam_b4soixdif_dn11 * locals.var_t4) + (locals.var_pparam_b4soixdif * locals.var_t4_dn11)) * locals.var_pparam_b4soindiode) - (assign4400_e6089 * locals.var_pparam_b4soindiode_dn11)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));
        locals.var_t7_dn12 = (((((locals.var_pparam_b4soixdif_dn12 * locals.var_t4) + (locals.var_pparam_b4soixdif * locals.var_t4_dn12)) * locals.var_pparam_b4soindiode) - (assign4400_e6089 * locals.var_pparam_b4soindiode_dn12)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode));

    }

    pub(super) fn stamp_transient_block_12(
        locals: &mut StampLocals,
    ) {
        let assign4410_e6094: f64 = if locals.var_t7 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard535 = assign4410_e6094;

        let (assign4420_e6104, assign4420_e6104_d_n3, assign4420_e6104_d_n4, assign4420_e6104_d_n5, assign4420_e6104_d_n6, assign4420_e6104_d_n7, assign4420_e6104_d_n8, assign4420_e6104_d_n9, assign4420_e6104_d_n10, assign4420_e6104_d_n11, assign4420_e6104_d_n12,) = {
    if (locals.var_guard535 != 0.0) {
        let assign4420_e6099: f64 = (1.0 + locals.var_t7);
        let assign4420_e6101: f64 = (assign4420_e6099 - 100.0);
        let assign4420_e6102: f64 = (2.688117142e43 * assign4420_e6101);
        (assign4420_e6102, (2.688117142e43 * locals.var_t7_dn3), (2.688117142e43 * locals.var_t7_dn4), (2.688117142e43 * locals.var_t7_dn5), (2.688117142e43 * locals.var_t7_dn6), (2.688117142e43 * locals.var_t7_dn7), (2.688117142e43 * locals.var_t7_dn8), (2.688117142e43 * locals.var_t7_dn9), (2.688117142e43 * locals.var_t7_dn10), (2.688117142e43 * locals.var_t7_dn11), (2.688117142e43 * locals.var_t7_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign4420_e6104;
        locals.var_t1_dn3 = assign4420_e6104_d_n3;
        locals.var_t1_dn4 = assign4420_e6104_d_n4;
        locals.var_t1_dn5 = assign4420_e6104_d_n5;
        locals.var_t1_dn6 = assign4420_e6104_d_n6;
        locals.var_t1_dn7 = assign4420_e6104_d_n7;
        locals.var_t1_dn8 = assign4420_e6104_d_n8;
        locals.var_t1_dn9 = assign4420_e6104_d_n9;
        locals.var_t1_dn10 = assign4420_e6104_d_n10;
        locals.var_t1_dn11 = assign4420_e6104_d_n11;
        locals.var_t1_dn12 = assign4420_e6104_d_n12;

        let assign4430_e6107: f64 = (-100.0);
        let assign4430_e6108: f64 = if locals.var_t7 < assign4430_e6107 { 1.0 } else { 0.0 };
        locals.var_guard536 = assign4430_e6108;

        let (assign4440_e6115, assign4440_e6115_d_n3, assign4440_e6115_d_n4, assign4440_e6115_d_n5, assign4440_e6115_d_n6, assign4440_e6115_d_n7, assign4440_e6115_d_n8, assign4440_e6115_d_n9, assign4440_e6115_d_n10, assign4440_e6115_d_n11, assign4440_e6115_d_n12,) = {
    if ((locals.var_guard535 == 0.0) && (locals.var_guard536 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign4440_e6115;
        locals.var_t1_dn3 = assign4440_e6115_d_n3;
        locals.var_t1_dn4 = assign4440_e6115_d_n4;
        locals.var_t1_dn5 = assign4440_e6115_d_n5;
        locals.var_t1_dn6 = assign4440_e6115_d_n6;
        locals.var_t1_dn7 = assign4440_e6115_d_n7;
        locals.var_t1_dn8 = assign4440_e6115_d_n8;
        locals.var_t1_dn9 = assign4440_e6115_d_n9;
        locals.var_t1_dn10 = assign4440_e6115_d_n10;
        locals.var_t1_dn11 = assign4440_e6115_d_n11;
        locals.var_t1_dn12 = assign4440_e6115_d_n12;

        let (assign4450_e6124, assign4450_e6124_d_n3, assign4450_e6124_d_n4, assign4450_e6124_d_n5, assign4450_e6124_d_n6, assign4450_e6124_d_n7, assign4450_e6124_d_n8, assign4450_e6124_d_n9, assign4450_e6124_d_n10, assign4450_e6124_d_n11, assign4450_e6124_d_n12,) = {
    if ((locals.var_guard535 == 0.0) && (locals.var_guard536 == 0.0)) {
        let assign4450_e6122: f64 = (locals.var_t7).exp();
        (assign4450_e6122, (assign4450_e6122 * locals.var_t7_dn3), (assign4450_e6122 * locals.var_t7_dn4), (assign4450_e6122 * locals.var_t7_dn5), (assign4450_e6122 * locals.var_t7_dn6), (assign4450_e6122 * locals.var_t7_dn7), (assign4450_e6122 * locals.var_t7_dn8), (assign4450_e6122 * locals.var_t7_dn9), (assign4450_e6122 * locals.var_t7_dn10), (assign4450_e6122 * locals.var_t7_dn11), (assign4450_e6122 * locals.var_t7_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign4450_e6124;
        locals.var_t1_dn3 = assign4450_e6124_d_n3;
        locals.var_t1_dn4 = assign4450_e6124_d_n4;
        locals.var_t1_dn5 = assign4450_e6124_d_n5;
        locals.var_t1_dn6 = assign4450_e6124_d_n6;
        locals.var_t1_dn7 = assign4450_e6124_d_n7;
        locals.var_t1_dn8 = assign4450_e6124_d_n8;
        locals.var_t1_dn9 = assign4450_e6124_d_n9;
        locals.var_t1_dn10 = assign4450_e6124_d_n10;
        locals.var_t1_dn11 = assign4450_e6124_d_n11;
        locals.var_t1_dn12 = assign4450_e6124_d_n12;

        let assign4460_e6127: f64 = (locals.var_pparam_b4soixrec * locals.var_t4);
        let assign4460_e6129: f64 = (assign4460_e6127 / locals.var_pparam_b4soinrecf0);
        locals.var_t7 = assign4460_e6129;
        locals.var_t7_dn3 = (((((locals.var_pparam_b4soixrec_dn3 * locals.var_t4) + (locals.var_pparam_b4soixrec * locals.var_t4_dn3)) * locals.var_pparam_b4soinrecf0) - (assign4460_e6127 * locals.var_pparam_b4soinrecf0_dn3)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0));
        locals.var_t7_dn4 = (((((locals.var_pparam_b4soixrec_dn4 * locals.var_t4) + (locals.var_pparam_b4soixrec * locals.var_t4_dn4)) * locals.var_pparam_b4soinrecf0) - (assign4460_e6127 * locals.var_pparam_b4soinrecf0_dn4)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0));
        locals.var_t7_dn5 = (((((locals.var_pparam_b4soixrec_dn5 * locals.var_t4) + (locals.var_pparam_b4soixrec * locals.var_t4_dn5)) * locals.var_pparam_b4soinrecf0) - (assign4460_e6127 * locals.var_pparam_b4soinrecf0_dn5)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0));
        locals.var_t7_dn6 = (((((locals.var_pparam_b4soixrec_dn6 * locals.var_t4) + (locals.var_pparam_b4soixrec * locals.var_t4_dn6)) * locals.var_pparam_b4soinrecf0) - (assign4460_e6127 * locals.var_pparam_b4soinrecf0_dn6)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0));
        locals.var_t7_dn7 = (((((locals.var_pparam_b4soixrec_dn7 * locals.var_t4) + (locals.var_pparam_b4soixrec * locals.var_t4_dn7)) * locals.var_pparam_b4soinrecf0) - (assign4460_e6127 * locals.var_pparam_b4soinrecf0_dn7)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0));
        locals.var_t7_dn8 = (((((locals.var_pparam_b4soixrec_dn8 * locals.var_t4) + (locals.var_pparam_b4soixrec * locals.var_t4_dn8)) * locals.var_pparam_b4soinrecf0) - (assign4460_e6127 * locals.var_pparam_b4soinrecf0_dn8)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0));
        locals.var_t7_dn9 = (((((locals.var_pparam_b4soixrec_dn9 * locals.var_t4) + (locals.var_pparam_b4soixrec * locals.var_t4_dn9)) * locals.var_pparam_b4soinrecf0) - (assign4460_e6127 * locals.var_pparam_b4soinrecf0_dn9)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0));
        locals.var_t7_dn10 = (((((locals.var_pparam_b4soixrec_dn10 * locals.var_t4) + (locals.var_pparam_b4soixrec * locals.var_t4_dn10)) * locals.var_pparam_b4soinrecf0) - (assign4460_e6127 * locals.var_pparam_b4soinrecf0_dn10)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0));
        locals.var_t7_dn11 = (((((locals.var_pparam_b4soixrec_dn11 * locals.var_t4) + (locals.var_pparam_b4soixrec * locals.var_t4_dn11)) * locals.var_pparam_b4soinrecf0) - (assign4460_e6127 * locals.var_pparam_b4soinrecf0_dn11)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0));
        locals.var_t7_dn12 = (((((locals.var_pparam_b4soixrec_dn12 * locals.var_t4) + (locals.var_pparam_b4soixrec * locals.var_t4_dn12)) * locals.var_pparam_b4soinrecf0) - (assign4460_e6127 * locals.var_pparam_b4soinrecf0_dn12)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0));

        let assign4470_e6132: f64 = if locals.var_t7 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard537 = assign4470_e6132;

        let (assign4480_e6142, assign4480_e6142_d_n3, assign4480_e6142_d_n4, assign4480_e6142_d_n5, assign4480_e6142_d_n6, assign4480_e6142_d_n7, assign4480_e6142_d_n8, assign4480_e6142_d_n9, assign4480_e6142_d_n10, assign4480_e6142_d_n11, assign4480_e6142_d_n12,) = {
    if (locals.var_guard537 != 0.0) {
        let assign4480_e6137: f64 = (1.0 + locals.var_t7);
        let assign4480_e6139: f64 = (assign4480_e6137 - 100.0);
        let assign4480_e6140: f64 = (2.688117142e43 * assign4480_e6139);
        (assign4480_e6140, (2.688117142e43 * locals.var_t7_dn3), (2.688117142e43 * locals.var_t7_dn4), (2.688117142e43 * locals.var_t7_dn5), (2.688117142e43 * locals.var_t7_dn6), (2.688117142e43 * locals.var_t7_dn7), (2.688117142e43 * locals.var_t7_dn8), (2.688117142e43 * locals.var_t7_dn9), (2.688117142e43 * locals.var_t7_dn10), (2.688117142e43 * locals.var_t7_dn11), (2.688117142e43 * locals.var_t7_dn12),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign4480_e6142;
        locals.var_t2_dn3 = assign4480_e6142_d_n3;
        locals.var_t2_dn4 = assign4480_e6142_d_n4;
        locals.var_t2_dn5 = assign4480_e6142_d_n5;
        locals.var_t2_dn6 = assign4480_e6142_d_n6;
        locals.var_t2_dn7 = assign4480_e6142_d_n7;
        locals.var_t2_dn8 = assign4480_e6142_d_n8;
        locals.var_t2_dn9 = assign4480_e6142_d_n9;
        locals.var_t2_dn10 = assign4480_e6142_d_n10;
        locals.var_t2_dn11 = assign4480_e6142_d_n11;
        locals.var_t2_dn12 = assign4480_e6142_d_n12;

        let assign4490_e6145: f64 = (-100.0);
        let assign4490_e6146: f64 = if locals.var_t7 < assign4490_e6145 { 1.0 } else { 0.0 };
        locals.var_guard538 = assign4490_e6146;

        let (assign4500_e6153, assign4500_e6153_d_n3, assign4500_e6153_d_n4, assign4500_e6153_d_n5, assign4500_e6153_d_n6, assign4500_e6153_d_n7, assign4500_e6153_d_n8, assign4500_e6153_d_n9, assign4500_e6153_d_n10, assign4500_e6153_d_n11, assign4500_e6153_d_n12,) = {
    if ((locals.var_guard537 == 0.0) && (locals.var_guard538 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign4500_e6153;
        locals.var_t2_dn3 = assign4500_e6153_d_n3;
        locals.var_t2_dn4 = assign4500_e6153_d_n4;
        locals.var_t2_dn5 = assign4500_e6153_d_n5;
        locals.var_t2_dn6 = assign4500_e6153_d_n6;
        locals.var_t2_dn7 = assign4500_e6153_d_n7;
        locals.var_t2_dn8 = assign4500_e6153_d_n8;
        locals.var_t2_dn9 = assign4500_e6153_d_n9;
        locals.var_t2_dn10 = assign4500_e6153_d_n10;
        locals.var_t2_dn11 = assign4500_e6153_d_n11;
        locals.var_t2_dn12 = assign4500_e6153_d_n12;

        let (assign4510_e6162, assign4510_e6162_d_n3, assign4510_e6162_d_n4, assign4510_e6162_d_n5, assign4510_e6162_d_n6, assign4510_e6162_d_n7, assign4510_e6162_d_n8, assign4510_e6162_d_n9, assign4510_e6162_d_n10, assign4510_e6162_d_n11, assign4510_e6162_d_n12,) = {
    if ((locals.var_guard537 == 0.0) && (locals.var_guard538 == 0.0)) {
        let assign4510_e6160: f64 = (locals.var_t7).exp();
        (assign4510_e6160, (assign4510_e6160 * locals.var_t7_dn3), (assign4510_e6160 * locals.var_t7_dn4), (assign4510_e6160 * locals.var_t7_dn5), (assign4510_e6160 * locals.var_t7_dn6), (assign4510_e6160 * locals.var_t7_dn7), (assign4510_e6160 * locals.var_t7_dn8), (assign4510_e6160 * locals.var_t7_dn9), (assign4510_e6160 * locals.var_t7_dn10), (assign4510_e6160 * locals.var_t7_dn11), (assign4510_e6160 * locals.var_t7_dn12),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign4510_e6162;
        locals.var_t2_dn3 = assign4510_e6162_d_n3;
        locals.var_t2_dn4 = assign4510_e6162_d_n4;
        locals.var_t2_dn5 = assign4510_e6162_d_n5;
        locals.var_t2_dn6 = assign4510_e6162_d_n6;
        locals.var_t2_dn7 = assign4510_e6162_d_n7;
        locals.var_t2_dn8 = assign4510_e6162_d_n8;
        locals.var_t2_dn9 = assign4510_e6162_d_n9;
        locals.var_t2_dn10 = assign4510_e6162_d_n10;
        locals.var_t2_dn11 = assign4510_e6162_d_n11;
        locals.var_t2_dn12 = assign4510_e6162_d_n12;

        let assign4520_e6165: f64 = (locals.var_pparam_b4soiahli * locals.var_t0);
        locals.var_pparam_b4soiahli0s = assign4520_e6165;
        locals.var_pparam_b4soiahli0s_dn3 = ((locals.var_pparam_b4soiahli_dn3 * locals.var_t0) + (locals.var_pparam_b4soiahli * locals.var_t0_dn3));
        locals.var_pparam_b4soiahli0s_dn4 = ((locals.var_pparam_b4soiahli_dn4 * locals.var_t0) + (locals.var_pparam_b4soiahli * locals.var_t0_dn4));
        locals.var_pparam_b4soiahli0s_dn5 = ((locals.var_pparam_b4soiahli_dn5 * locals.var_t0) + (locals.var_pparam_b4soiahli * locals.var_t0_dn5));
        locals.var_pparam_b4soiahli0s_dn6 = ((locals.var_pparam_b4soiahli_dn6 * locals.var_t0) + (locals.var_pparam_b4soiahli * locals.var_t0_dn6));
        locals.var_pparam_b4soiahli0s_dn7 = ((locals.var_pparam_b4soiahli_dn7 * locals.var_t0) + (locals.var_pparam_b4soiahli * locals.var_t0_dn7));
        locals.var_pparam_b4soiahli0s_dn8 = ((locals.var_pparam_b4soiahli_dn8 * locals.var_t0) + (locals.var_pparam_b4soiahli * locals.var_t0_dn8));
        locals.var_pparam_b4soiahli0s_dn9 = ((locals.var_pparam_b4soiahli_dn9 * locals.var_t0) + (locals.var_pparam_b4soiahli * locals.var_t0_dn9));
        locals.var_pparam_b4soiahli0s_dn10 = ((locals.var_pparam_b4soiahli_dn10 * locals.var_t0) + (locals.var_pparam_b4soiahli * locals.var_t0_dn10));
        locals.var_pparam_b4soiahli0s_dn11 = ((locals.var_pparam_b4soiahli_dn11 * locals.var_t0) + (locals.var_pparam_b4soiahli * locals.var_t0_dn11));
        locals.var_pparam_b4soiahli0s_dn12 = ((locals.var_pparam_b4soiahli_dn12 * locals.var_t0) + (locals.var_pparam_b4soiahli * locals.var_t0_dn12));

        let assign4530_e6168: f64 = (locals.var_pparam_b4soiisbjt * locals.var_t0);
        locals.var_pparam_b4soijbjts = assign4530_e6168;
        locals.var_pparam_b4soijbjts_dn3 = ((locals.var_pparam_b4soiisbjt_dn3 * locals.var_t0) + (locals.var_pparam_b4soiisbjt * locals.var_t0_dn3));
        locals.var_pparam_b4soijbjts_dn4 = ((locals.var_pparam_b4soiisbjt_dn4 * locals.var_t0) + (locals.var_pparam_b4soiisbjt * locals.var_t0_dn4));
        locals.var_pparam_b4soijbjts_dn5 = ((locals.var_pparam_b4soiisbjt_dn5 * locals.var_t0) + (locals.var_pparam_b4soiisbjt * locals.var_t0_dn5));
        locals.var_pparam_b4soijbjts_dn6 = ((locals.var_pparam_b4soiisbjt_dn6 * locals.var_t0) + (locals.var_pparam_b4soiisbjt * locals.var_t0_dn6));
        locals.var_pparam_b4soijbjts_dn7 = ((locals.var_pparam_b4soiisbjt_dn7 * locals.var_t0) + (locals.var_pparam_b4soiisbjt * locals.var_t0_dn7));
        locals.var_pparam_b4soijbjts_dn8 = ((locals.var_pparam_b4soiisbjt_dn8 * locals.var_t0) + (locals.var_pparam_b4soiisbjt * locals.var_t0_dn8));
        locals.var_pparam_b4soijbjts_dn9 = ((locals.var_pparam_b4soiisbjt_dn9 * locals.var_t0) + (locals.var_pparam_b4soiisbjt * locals.var_t0_dn9));
        locals.var_pparam_b4soijbjts_dn10 = ((locals.var_pparam_b4soiisbjt_dn10 * locals.var_t0) + (locals.var_pparam_b4soiisbjt * locals.var_t0_dn10));
        locals.var_pparam_b4soijbjts_dn11 = ((locals.var_pparam_b4soiisbjt_dn11 * locals.var_t0) + (locals.var_pparam_b4soiisbjt * locals.var_t0_dn11));
        locals.var_pparam_b4soijbjts_dn12 = ((locals.var_pparam_b4soiisbjt_dn12 * locals.var_t0) + (locals.var_pparam_b4soiisbjt * locals.var_t0_dn12));

        let assign4540_e6171: f64 = (locals.var_pparam_b4soiisdif * locals.var_t1);
        locals.var_pparam_b4soijdifs = assign4540_e6171;
        locals.var_pparam_b4soijdifs_dn3 = ((locals.var_pparam_b4soiisdif_dn3 * locals.var_t1) + (locals.var_pparam_b4soiisdif * locals.var_t1_dn3));
        locals.var_pparam_b4soijdifs_dn4 = ((locals.var_pparam_b4soiisdif_dn4 * locals.var_t1) + (locals.var_pparam_b4soiisdif * locals.var_t1_dn4));
        locals.var_pparam_b4soijdifs_dn5 = ((locals.var_pparam_b4soiisdif_dn5 * locals.var_t1) + (locals.var_pparam_b4soiisdif * locals.var_t1_dn5));
        locals.var_pparam_b4soijdifs_dn6 = ((locals.var_pparam_b4soiisdif_dn6 * locals.var_t1) + (locals.var_pparam_b4soiisdif * locals.var_t1_dn6));
        locals.var_pparam_b4soijdifs_dn7 = ((locals.var_pparam_b4soiisdif_dn7 * locals.var_t1) + (locals.var_pparam_b4soiisdif * locals.var_t1_dn7));
        locals.var_pparam_b4soijdifs_dn8 = ((locals.var_pparam_b4soiisdif_dn8 * locals.var_t1) + (locals.var_pparam_b4soiisdif * locals.var_t1_dn8));
        locals.var_pparam_b4soijdifs_dn9 = ((locals.var_pparam_b4soiisdif_dn9 * locals.var_t1) + (locals.var_pparam_b4soiisdif * locals.var_t1_dn9));
        locals.var_pparam_b4soijdifs_dn10 = ((locals.var_pparam_b4soiisdif_dn10 * locals.var_t1) + (locals.var_pparam_b4soiisdif * locals.var_t1_dn10));
        locals.var_pparam_b4soijdifs_dn11 = ((locals.var_pparam_b4soiisdif_dn11 * locals.var_t1) + (locals.var_pparam_b4soiisdif * locals.var_t1_dn11));
        locals.var_pparam_b4soijdifs_dn12 = ((locals.var_pparam_b4soiisdif_dn12 * locals.var_t1) + (locals.var_pparam_b4soiisdif * locals.var_t1_dn12));

        let assign4550_e6174: f64 = (locals.var_pparam_b4soiisrec * locals.var_t2);
        locals.var_pparam_b4soijrecs = assign4550_e6174;
        locals.var_pparam_b4soijrecs_dn3 = ((locals.var_pparam_b4soiisrec_dn3 * locals.var_t2) + (locals.var_pparam_b4soiisrec * locals.var_t2_dn3));
        locals.var_pparam_b4soijrecs_dn4 = ((locals.var_pparam_b4soiisrec_dn4 * locals.var_t2) + (locals.var_pparam_b4soiisrec * locals.var_t2_dn4));
        locals.var_pparam_b4soijrecs_dn5 = ((locals.var_pparam_b4soiisrec_dn5 * locals.var_t2) + (locals.var_pparam_b4soiisrec * locals.var_t2_dn5));
        locals.var_pparam_b4soijrecs_dn6 = ((locals.var_pparam_b4soiisrec_dn6 * locals.var_t2) + (locals.var_pparam_b4soiisrec * locals.var_t2_dn6));
        locals.var_pparam_b4soijrecs_dn7 = ((locals.var_pparam_b4soiisrec_dn7 * locals.var_t2) + (locals.var_pparam_b4soiisrec * locals.var_t2_dn7));
        locals.var_pparam_b4soijrecs_dn8 = ((locals.var_pparam_b4soiisrec_dn8 * locals.var_t2) + (locals.var_pparam_b4soiisrec * locals.var_t2_dn8));
        locals.var_pparam_b4soijrecs_dn9 = ((locals.var_pparam_b4soiisrec_dn9 * locals.var_t2) + (locals.var_pparam_b4soiisrec * locals.var_t2_dn9));
        locals.var_pparam_b4soijrecs_dn10 = ((locals.var_pparam_b4soiisrec_dn10 * locals.var_t2) + (locals.var_pparam_b4soiisrec * locals.var_t2_dn10));
        locals.var_pparam_b4soijrecs_dn11 = ((locals.var_pparam_b4soiisrec_dn11 * locals.var_t2) + (locals.var_pparam_b4soiisrec * locals.var_t2_dn11));
        locals.var_pparam_b4soijrecs_dn12 = ((locals.var_pparam_b4soiisrec_dn12 * locals.var_t2) + (locals.var_pparam_b4soiisrec * locals.var_t2_dn12));

        let assign4560_e6177: f64 = (locals.var_pparam_b4soixtun * locals.var_trm1);
        locals.var_t7 = assign4560_e6177;
        locals.var_t7_dn3 = (locals.var_pparam_b4soixtun_dn3 * locals.var_trm1);
        locals.var_t7_dn4 = ((locals.var_pparam_b4soixtun_dn4 * locals.var_trm1) + (locals.var_pparam_b4soixtun * locals.var_trm1_dn4));
        locals.var_t7_dn5 = ((locals.var_pparam_b4soixtun_dn5 * locals.var_trm1) + (locals.var_pparam_b4soixtun * locals.var_trm1_dn5));
        locals.var_t7_dn6 = ((locals.var_pparam_b4soixtun_dn6 * locals.var_trm1) + (locals.var_pparam_b4soixtun * locals.var_trm1_dn6));
        locals.var_t7_dn7 = (locals.var_pparam_b4soixtun_dn7 * locals.var_trm1);
        locals.var_t7_dn8 = (locals.var_pparam_b4soixtun_dn8 * locals.var_trm1);
        locals.var_t7_dn9 = (locals.var_pparam_b4soixtun_dn9 * locals.var_trm1);
        locals.var_t7_dn10 = (locals.var_pparam_b4soixtun_dn10 * locals.var_trm1);
        locals.var_t7_dn11 = (locals.var_pparam_b4soixtun_dn11 * locals.var_trm1);
        locals.var_t7_dn12 = (locals.var_pparam_b4soixtun_dn12 * locals.var_trm1);

        let assign4570_e6180: f64 = if locals.var_t7 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard539 = assign4570_e6180;

        let (assign4580_e6190, assign4580_e6190_d_n3, assign4580_e6190_d_n4, assign4580_e6190_d_n5, assign4580_e6190_d_n6, assign4580_e6190_d_n7, assign4580_e6190_d_n8, assign4580_e6190_d_n9, assign4580_e6190_d_n10, assign4580_e6190_d_n11, assign4580_e6190_d_n12,) = {
    if (locals.var_guard539 != 0.0) {
        let assign4580_e6185: f64 = (1.0 + locals.var_t7);
        let assign4580_e6187: f64 = (assign4580_e6185 - 100.0);
        let assign4580_e6188: f64 = (2.688117142e43 * assign4580_e6187);
        (assign4580_e6188, (2.688117142e43 * locals.var_t7_dn3), (2.688117142e43 * locals.var_t7_dn4), (2.688117142e43 * locals.var_t7_dn5), (2.688117142e43 * locals.var_t7_dn6), (2.688117142e43 * locals.var_t7_dn7), (2.688117142e43 * locals.var_t7_dn8), (2.688117142e43 * locals.var_t7_dn9), (2.688117142e43 * locals.var_t7_dn10), (2.688117142e43 * locals.var_t7_dn11), (2.688117142e43 * locals.var_t7_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign4580_e6190;
        locals.var_t0_dn3 = assign4580_e6190_d_n3;
        locals.var_t0_dn4 = assign4580_e6190_d_n4;
        locals.var_t0_dn5 = assign4580_e6190_d_n5;
        locals.var_t0_dn6 = assign4580_e6190_d_n6;
        locals.var_t0_dn7 = assign4580_e6190_d_n7;
        locals.var_t0_dn8 = assign4580_e6190_d_n8;
        locals.var_t0_dn9 = assign4580_e6190_d_n9;
        locals.var_t0_dn10 = assign4580_e6190_d_n10;
        locals.var_t0_dn11 = assign4580_e6190_d_n11;
        locals.var_t0_dn12 = assign4580_e6190_d_n12;

        let assign4590_e6193: f64 = (-100.0);
        let assign4590_e6194: f64 = if locals.var_t7 < assign4590_e6193 { 1.0 } else { 0.0 };
        locals.var_guard540 = assign4590_e6194;

        let (assign4600_e6201, assign4600_e6201_d_n3, assign4600_e6201_d_n4, assign4600_e6201_d_n5, assign4600_e6201_d_n6, assign4600_e6201_d_n7, assign4600_e6201_d_n8, assign4600_e6201_d_n9, assign4600_e6201_d_n10, assign4600_e6201_d_n11, assign4600_e6201_d_n12,) = {
    if ((locals.var_guard539 == 0.0) && (locals.var_guard540 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign4600_e6201;
        locals.var_t0_dn3 = assign4600_e6201_d_n3;
        locals.var_t0_dn4 = assign4600_e6201_d_n4;
        locals.var_t0_dn5 = assign4600_e6201_d_n5;
        locals.var_t0_dn6 = assign4600_e6201_d_n6;
        locals.var_t0_dn7 = assign4600_e6201_d_n7;
        locals.var_t0_dn8 = assign4600_e6201_d_n8;
        locals.var_t0_dn9 = assign4600_e6201_d_n9;
        locals.var_t0_dn10 = assign4600_e6201_d_n10;
        locals.var_t0_dn11 = assign4600_e6201_d_n11;
        locals.var_t0_dn12 = assign4600_e6201_d_n12;

        let (assign4610_e6210, assign4610_e6210_d_n3, assign4610_e6210_d_n4, assign4610_e6210_d_n5, assign4610_e6210_d_n6, assign4610_e6210_d_n7, assign4610_e6210_d_n8, assign4610_e6210_d_n9, assign4610_e6210_d_n10, assign4610_e6210_d_n11, assign4610_e6210_d_n12,) = {
    if ((locals.var_guard539 == 0.0) && (locals.var_guard540 == 0.0)) {
        let assign4610_e6208: f64 = (locals.var_t7).exp();
        (assign4610_e6208, (assign4610_e6208 * locals.var_t7_dn3), (assign4610_e6208 * locals.var_t7_dn4), (assign4610_e6208 * locals.var_t7_dn5), (assign4610_e6208 * locals.var_t7_dn6), (assign4610_e6208 * locals.var_t7_dn7), (assign4610_e6208 * locals.var_t7_dn8), (assign4610_e6208 * locals.var_t7_dn9), (assign4610_e6208 * locals.var_t7_dn10), (assign4610_e6208 * locals.var_t7_dn11), (assign4610_e6208 * locals.var_t7_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign4610_e6210;
        locals.var_t0_dn3 = assign4610_e6210_d_n3;
        locals.var_t0_dn4 = assign4610_e6210_d_n4;
        locals.var_t0_dn5 = assign4610_e6210_d_n5;
        locals.var_t0_dn6 = assign4610_e6210_d_n6;
        locals.var_t0_dn7 = assign4610_e6210_d_n7;
        locals.var_t0_dn8 = assign4610_e6210_d_n8;
        locals.var_t0_dn9 = assign4610_e6210_d_n9;
        locals.var_t0_dn10 = assign4610_e6210_d_n10;
        locals.var_t0_dn11 = assign4610_e6210_d_n11;
        locals.var_t0_dn12 = assign4610_e6210_d_n12;

        let assign4620_e6213: f64 = (locals.var_pparam_b4soiistun * locals.var_t0);
        locals.var_pparam_b4soijtuns = assign4620_e6213;
        locals.var_pparam_b4soijtuns_dn3 = ((locals.var_pparam_b4soiistun_dn3 * locals.var_t0) + (locals.var_pparam_b4soiistun * locals.var_t0_dn3));
        locals.var_pparam_b4soijtuns_dn4 = ((locals.var_pparam_b4soiistun_dn4 * locals.var_t0) + (locals.var_pparam_b4soiistun * locals.var_t0_dn4));
        locals.var_pparam_b4soijtuns_dn5 = ((locals.var_pparam_b4soiistun_dn5 * locals.var_t0) + (locals.var_pparam_b4soiistun * locals.var_t0_dn5));
        locals.var_pparam_b4soijtuns_dn6 = ((locals.var_pparam_b4soiistun_dn6 * locals.var_t0) + (locals.var_pparam_b4soiistun * locals.var_t0_dn6));
        locals.var_pparam_b4soijtuns_dn7 = ((locals.var_pparam_b4soiistun_dn7 * locals.var_t0) + (locals.var_pparam_b4soiistun * locals.var_t0_dn7));
        locals.var_pparam_b4soijtuns_dn8 = ((locals.var_pparam_b4soiistun_dn8 * locals.var_t0) + (locals.var_pparam_b4soiistun * locals.var_t0_dn8));
        locals.var_pparam_b4soijtuns_dn9 = ((locals.var_pparam_b4soiistun_dn9 * locals.var_t0) + (locals.var_pparam_b4soiistun * locals.var_t0_dn9));
        locals.var_pparam_b4soijtuns_dn10 = ((locals.var_pparam_b4soiistun_dn10 * locals.var_t0) + (locals.var_pparam_b4soiistun * locals.var_t0_dn10));
        locals.var_pparam_b4soijtuns_dn11 = ((locals.var_pparam_b4soiistun_dn11 * locals.var_t0) + (locals.var_pparam_b4soiistun * locals.var_t0_dn11));
        locals.var_pparam_b4soijtuns_dn12 = ((locals.var_pparam_b4soiistun_dn12 * locals.var_t0) + (locals.var_pparam_b4soiistun * locals.var_t0_dn12));

        let assign4630_e6216: f64 = (locals.var_pparam_b4soixbjt * locals.var_t4);
        let assign4630_e6218: f64 = (assign4630_e6216 / locals.var_pparam_b4soindioded);
        locals.var_t7 = assign4630_e6218;
        locals.var_t7_dn3 = (((((locals.var_pparam_b4soixbjt_dn3 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn3)) * locals.var_pparam_b4soindioded) - (assign4630_e6216 * locals.var_pparam_b4soindioded_dn3)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn4 = (((((locals.var_pparam_b4soixbjt_dn4 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn4)) * locals.var_pparam_b4soindioded) - (assign4630_e6216 * locals.var_pparam_b4soindioded_dn4)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn5 = (((((locals.var_pparam_b4soixbjt_dn5 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn5)) * locals.var_pparam_b4soindioded) - (assign4630_e6216 * locals.var_pparam_b4soindioded_dn5)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn6 = (((((locals.var_pparam_b4soixbjt_dn6 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn6)) * locals.var_pparam_b4soindioded) - (assign4630_e6216 * locals.var_pparam_b4soindioded_dn6)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn7 = (((((locals.var_pparam_b4soixbjt_dn7 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn7)) * locals.var_pparam_b4soindioded) - (assign4630_e6216 * locals.var_pparam_b4soindioded_dn7)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn8 = (((((locals.var_pparam_b4soixbjt_dn8 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn8)) * locals.var_pparam_b4soindioded) - (assign4630_e6216 * locals.var_pparam_b4soindioded_dn8)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn9 = (((((locals.var_pparam_b4soixbjt_dn9 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn9)) * locals.var_pparam_b4soindioded) - (assign4630_e6216 * locals.var_pparam_b4soindioded_dn9)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn10 = (((((locals.var_pparam_b4soixbjt_dn10 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn10)) * locals.var_pparam_b4soindioded) - (assign4630_e6216 * locals.var_pparam_b4soindioded_dn10)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn11 = (((((locals.var_pparam_b4soixbjt_dn11 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn11)) * locals.var_pparam_b4soindioded) - (assign4630_e6216 * locals.var_pparam_b4soindioded_dn11)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn12 = (((((locals.var_pparam_b4soixbjt_dn12 * locals.var_t4) + (locals.var_pparam_b4soixbjt * locals.var_t4_dn12)) * locals.var_pparam_b4soindioded) - (assign4630_e6216 * locals.var_pparam_b4soindioded_dn12)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));

        let assign4640_e6221: f64 = if locals.var_t7 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard541 = assign4640_e6221;

        let (assign4650_e6231, assign4650_e6231_d_n3, assign4650_e6231_d_n4, assign4650_e6231_d_n5, assign4650_e6231_d_n6, assign4650_e6231_d_n7, assign4650_e6231_d_n8, assign4650_e6231_d_n9, assign4650_e6231_d_n10, assign4650_e6231_d_n11, assign4650_e6231_d_n12,) = {
    if (locals.var_guard541 != 0.0) {
        let assign4650_e6226: f64 = (1.0 + locals.var_t7);
        let assign4650_e6228: f64 = (assign4650_e6226 - 100.0);
        let assign4650_e6229: f64 = (2.688117142e43 * assign4650_e6228);
        (assign4650_e6229, (2.688117142e43 * locals.var_t7_dn3), (2.688117142e43 * locals.var_t7_dn4), (2.688117142e43 * locals.var_t7_dn5), (2.688117142e43 * locals.var_t7_dn6), (2.688117142e43 * locals.var_t7_dn7), (2.688117142e43 * locals.var_t7_dn8), (2.688117142e43 * locals.var_t7_dn9), (2.688117142e43 * locals.var_t7_dn10), (2.688117142e43 * locals.var_t7_dn11), (2.688117142e43 * locals.var_t7_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign4650_e6231;
        locals.var_t0_dn3 = assign4650_e6231_d_n3;
        locals.var_t0_dn4 = assign4650_e6231_d_n4;
        locals.var_t0_dn5 = assign4650_e6231_d_n5;
        locals.var_t0_dn6 = assign4650_e6231_d_n6;
        locals.var_t0_dn7 = assign4650_e6231_d_n7;
        locals.var_t0_dn8 = assign4650_e6231_d_n8;
        locals.var_t0_dn9 = assign4650_e6231_d_n9;
        locals.var_t0_dn10 = assign4650_e6231_d_n10;
        locals.var_t0_dn11 = assign4650_e6231_d_n11;
        locals.var_t0_dn12 = assign4650_e6231_d_n12;

        let assign4660_e6234: f64 = (-100.0);
        let assign4660_e6235: f64 = if locals.var_t7 < assign4660_e6234 { 1.0 } else { 0.0 };
        locals.var_guard542 = assign4660_e6235;

        let (assign4670_e6242, assign4670_e6242_d_n3, assign4670_e6242_d_n4, assign4670_e6242_d_n5, assign4670_e6242_d_n6, assign4670_e6242_d_n7, assign4670_e6242_d_n8, assign4670_e6242_d_n9, assign4670_e6242_d_n10, assign4670_e6242_d_n11, assign4670_e6242_d_n12,) = {
    if ((locals.var_guard541 == 0.0) && (locals.var_guard542 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign4670_e6242;
        locals.var_t0_dn3 = assign4670_e6242_d_n3;
        locals.var_t0_dn4 = assign4670_e6242_d_n4;
        locals.var_t0_dn5 = assign4670_e6242_d_n5;
        locals.var_t0_dn6 = assign4670_e6242_d_n6;
        locals.var_t0_dn7 = assign4670_e6242_d_n7;
        locals.var_t0_dn8 = assign4670_e6242_d_n8;
        locals.var_t0_dn9 = assign4670_e6242_d_n9;
        locals.var_t0_dn10 = assign4670_e6242_d_n10;
        locals.var_t0_dn11 = assign4670_e6242_d_n11;
        locals.var_t0_dn12 = assign4670_e6242_d_n12;

        let (assign4680_e6251, assign4680_e6251_d_n3, assign4680_e6251_d_n4, assign4680_e6251_d_n5, assign4680_e6251_d_n6, assign4680_e6251_d_n7, assign4680_e6251_d_n8, assign4680_e6251_d_n9, assign4680_e6251_d_n10, assign4680_e6251_d_n11, assign4680_e6251_d_n12,) = {
    if ((locals.var_guard541 == 0.0) && (locals.var_guard542 == 0.0)) {
        let assign4680_e6249: f64 = (locals.var_t7).exp();
        (assign4680_e6249, (assign4680_e6249 * locals.var_t7_dn3), (assign4680_e6249 * locals.var_t7_dn4), (assign4680_e6249 * locals.var_t7_dn5), (assign4680_e6249 * locals.var_t7_dn6), (assign4680_e6249 * locals.var_t7_dn7), (assign4680_e6249 * locals.var_t7_dn8), (assign4680_e6249 * locals.var_t7_dn9), (assign4680_e6249 * locals.var_t7_dn10), (assign4680_e6249 * locals.var_t7_dn11), (assign4680_e6249 * locals.var_t7_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign4680_e6251;
        locals.var_t0_dn3 = assign4680_e6251_d_n3;
        locals.var_t0_dn4 = assign4680_e6251_d_n4;
        locals.var_t0_dn5 = assign4680_e6251_d_n5;
        locals.var_t0_dn6 = assign4680_e6251_d_n6;
        locals.var_t0_dn7 = assign4680_e6251_d_n7;
        locals.var_t0_dn8 = assign4680_e6251_d_n8;
        locals.var_t0_dn9 = assign4680_e6251_d_n9;
        locals.var_t0_dn10 = assign4680_e6251_d_n10;
        locals.var_t0_dn11 = assign4680_e6251_d_n11;
        locals.var_t0_dn12 = assign4680_e6251_d_n12;

        let assign4690_e6254: f64 = (locals.var_pparam_b4soixdifd * locals.var_t4);
        let assign4690_e6256: f64 = (assign4690_e6254 / locals.var_pparam_b4soindioded);
        locals.var_t7 = assign4690_e6256;
        locals.var_t7_dn3 = (((((locals.var_pparam_b4soixdifd_dn3 * locals.var_t4) + (locals.var_pparam_b4soixdifd * locals.var_t4_dn3)) * locals.var_pparam_b4soindioded) - (assign4690_e6254 * locals.var_pparam_b4soindioded_dn3)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn4 = (((((locals.var_pparam_b4soixdifd_dn4 * locals.var_t4) + (locals.var_pparam_b4soixdifd * locals.var_t4_dn4)) * locals.var_pparam_b4soindioded) - (assign4690_e6254 * locals.var_pparam_b4soindioded_dn4)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn5 = (((((locals.var_pparam_b4soixdifd_dn5 * locals.var_t4) + (locals.var_pparam_b4soixdifd * locals.var_t4_dn5)) * locals.var_pparam_b4soindioded) - (assign4690_e6254 * locals.var_pparam_b4soindioded_dn5)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn6 = (((((locals.var_pparam_b4soixdifd_dn6 * locals.var_t4) + (locals.var_pparam_b4soixdifd * locals.var_t4_dn6)) * locals.var_pparam_b4soindioded) - (assign4690_e6254 * locals.var_pparam_b4soindioded_dn6)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn7 = (((((locals.var_pparam_b4soixdifd_dn7 * locals.var_t4) + (locals.var_pparam_b4soixdifd * locals.var_t4_dn7)) * locals.var_pparam_b4soindioded) - (assign4690_e6254 * locals.var_pparam_b4soindioded_dn7)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn8 = (((((locals.var_pparam_b4soixdifd_dn8 * locals.var_t4) + (locals.var_pparam_b4soixdifd * locals.var_t4_dn8)) * locals.var_pparam_b4soindioded) - (assign4690_e6254 * locals.var_pparam_b4soindioded_dn8)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn9 = (((((locals.var_pparam_b4soixdifd_dn9 * locals.var_t4) + (locals.var_pparam_b4soixdifd * locals.var_t4_dn9)) * locals.var_pparam_b4soindioded) - (assign4690_e6254 * locals.var_pparam_b4soindioded_dn9)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn10 = (((((locals.var_pparam_b4soixdifd_dn10 * locals.var_t4) + (locals.var_pparam_b4soixdifd * locals.var_t4_dn10)) * locals.var_pparam_b4soindioded) - (assign4690_e6254 * locals.var_pparam_b4soindioded_dn10)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn11 = (((((locals.var_pparam_b4soixdifd_dn11 * locals.var_t4) + (locals.var_pparam_b4soixdifd * locals.var_t4_dn11)) * locals.var_pparam_b4soindioded) - (assign4690_e6254 * locals.var_pparam_b4soindioded_dn11)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));
        locals.var_t7_dn12 = (((((locals.var_pparam_b4soixdifd_dn12 * locals.var_t4) + (locals.var_pparam_b4soixdifd * locals.var_t4_dn12)) * locals.var_pparam_b4soindioded) - (assign4690_e6254 * locals.var_pparam_b4soindioded_dn12)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded));

        let assign4700_e6259: f64 = if locals.var_t7 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard543 = assign4700_e6259;

        let (assign4710_e6269, assign4710_e6269_d_n3, assign4710_e6269_d_n4, assign4710_e6269_d_n5, assign4710_e6269_d_n6, assign4710_e6269_d_n7, assign4710_e6269_d_n8, assign4710_e6269_d_n9, assign4710_e6269_d_n10, assign4710_e6269_d_n11, assign4710_e6269_d_n12,) = {
    if (locals.var_guard543 != 0.0) {
        let assign4710_e6264: f64 = (1.0 + locals.var_t7);
        let assign4710_e6266: f64 = (assign4710_e6264 - 100.0);
        let assign4710_e6267: f64 = (2.688117142e43 * assign4710_e6266);
        (assign4710_e6267, (2.688117142e43 * locals.var_t7_dn3), (2.688117142e43 * locals.var_t7_dn4), (2.688117142e43 * locals.var_t7_dn5), (2.688117142e43 * locals.var_t7_dn6), (2.688117142e43 * locals.var_t7_dn7), (2.688117142e43 * locals.var_t7_dn8), (2.688117142e43 * locals.var_t7_dn9), (2.688117142e43 * locals.var_t7_dn10), (2.688117142e43 * locals.var_t7_dn11), (2.688117142e43 * locals.var_t7_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign4710_e6269;
        locals.var_t1_dn3 = assign4710_e6269_d_n3;
        locals.var_t1_dn4 = assign4710_e6269_d_n4;
        locals.var_t1_dn5 = assign4710_e6269_d_n5;
        locals.var_t1_dn6 = assign4710_e6269_d_n6;
        locals.var_t1_dn7 = assign4710_e6269_d_n7;
        locals.var_t1_dn8 = assign4710_e6269_d_n8;
        locals.var_t1_dn9 = assign4710_e6269_d_n9;
        locals.var_t1_dn10 = assign4710_e6269_d_n10;
        locals.var_t1_dn11 = assign4710_e6269_d_n11;
        locals.var_t1_dn12 = assign4710_e6269_d_n12;

        let assign4720_e6272: f64 = (-100.0);
        let assign4720_e6273: f64 = if locals.var_t7 < assign4720_e6272 { 1.0 } else { 0.0 };
        locals.var_guard544 = assign4720_e6273;

        let (assign4730_e6280, assign4730_e6280_d_n3, assign4730_e6280_d_n4, assign4730_e6280_d_n5, assign4730_e6280_d_n6, assign4730_e6280_d_n7, assign4730_e6280_d_n8, assign4730_e6280_d_n9, assign4730_e6280_d_n10, assign4730_e6280_d_n11, assign4730_e6280_d_n12,) = {
    if ((locals.var_guard543 == 0.0) && (locals.var_guard544 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign4730_e6280;
        locals.var_t1_dn3 = assign4730_e6280_d_n3;
        locals.var_t1_dn4 = assign4730_e6280_d_n4;
        locals.var_t1_dn5 = assign4730_e6280_d_n5;
        locals.var_t1_dn6 = assign4730_e6280_d_n6;
        locals.var_t1_dn7 = assign4730_e6280_d_n7;
        locals.var_t1_dn8 = assign4730_e6280_d_n8;
        locals.var_t1_dn9 = assign4730_e6280_d_n9;
        locals.var_t1_dn10 = assign4730_e6280_d_n10;
        locals.var_t1_dn11 = assign4730_e6280_d_n11;
        locals.var_t1_dn12 = assign4730_e6280_d_n12;

        let (assign4740_e6289, assign4740_e6289_d_n3, assign4740_e6289_d_n4, assign4740_e6289_d_n5, assign4740_e6289_d_n6, assign4740_e6289_d_n7, assign4740_e6289_d_n8, assign4740_e6289_d_n9, assign4740_e6289_d_n10, assign4740_e6289_d_n11, assign4740_e6289_d_n12,) = {
    if ((locals.var_guard543 == 0.0) && (locals.var_guard544 == 0.0)) {
        let assign4740_e6287: f64 = (locals.var_t7).exp();
        (assign4740_e6287, (assign4740_e6287 * locals.var_t7_dn3), (assign4740_e6287 * locals.var_t7_dn4), (assign4740_e6287 * locals.var_t7_dn5), (assign4740_e6287 * locals.var_t7_dn6), (assign4740_e6287 * locals.var_t7_dn7), (assign4740_e6287 * locals.var_t7_dn8), (assign4740_e6287 * locals.var_t7_dn9), (assign4740_e6287 * locals.var_t7_dn10), (assign4740_e6287 * locals.var_t7_dn11), (assign4740_e6287 * locals.var_t7_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign4740_e6289;
        locals.var_t1_dn3 = assign4740_e6289_d_n3;
        locals.var_t1_dn4 = assign4740_e6289_d_n4;
        locals.var_t1_dn5 = assign4740_e6289_d_n5;
        locals.var_t1_dn6 = assign4740_e6289_d_n6;
        locals.var_t1_dn7 = assign4740_e6289_d_n7;
        locals.var_t1_dn8 = assign4740_e6289_d_n8;
        locals.var_t1_dn9 = assign4740_e6289_d_n9;
        locals.var_t1_dn10 = assign4740_e6289_d_n10;
        locals.var_t1_dn11 = assign4740_e6289_d_n11;
        locals.var_t1_dn12 = assign4740_e6289_d_n12;

        let assign4750_e6292: f64 = (locals.var_pparam_b4soixrecd * locals.var_t4);
        let assign4750_e6294: f64 = (assign4750_e6292 / locals.var_pparam_b4soinrecf0d);
        locals.var_t7 = assign4750_e6294;
        locals.var_t7_dn3 = (((((locals.var_pparam_b4soixrecd_dn3 * locals.var_t4) + (locals.var_pparam_b4soixrecd * locals.var_t4_dn3)) * locals.var_pparam_b4soinrecf0d) - (assign4750_e6292 * locals.var_pparam_b4soinrecf0d_dn3)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d));
        locals.var_t7_dn4 = (((((locals.var_pparam_b4soixrecd_dn4 * locals.var_t4) + (locals.var_pparam_b4soixrecd * locals.var_t4_dn4)) * locals.var_pparam_b4soinrecf0d) - (assign4750_e6292 * locals.var_pparam_b4soinrecf0d_dn4)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d));
        locals.var_t7_dn5 = (((((locals.var_pparam_b4soixrecd_dn5 * locals.var_t4) + (locals.var_pparam_b4soixrecd * locals.var_t4_dn5)) * locals.var_pparam_b4soinrecf0d) - (assign4750_e6292 * locals.var_pparam_b4soinrecf0d_dn5)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d));
        locals.var_t7_dn6 = (((((locals.var_pparam_b4soixrecd_dn6 * locals.var_t4) + (locals.var_pparam_b4soixrecd * locals.var_t4_dn6)) * locals.var_pparam_b4soinrecf0d) - (assign4750_e6292 * locals.var_pparam_b4soinrecf0d_dn6)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d));
        locals.var_t7_dn7 = (((((locals.var_pparam_b4soixrecd_dn7 * locals.var_t4) + (locals.var_pparam_b4soixrecd * locals.var_t4_dn7)) * locals.var_pparam_b4soinrecf0d) - (assign4750_e6292 * locals.var_pparam_b4soinrecf0d_dn7)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d));
        locals.var_t7_dn8 = (((((locals.var_pparam_b4soixrecd_dn8 * locals.var_t4) + (locals.var_pparam_b4soixrecd * locals.var_t4_dn8)) * locals.var_pparam_b4soinrecf0d) - (assign4750_e6292 * locals.var_pparam_b4soinrecf0d_dn8)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d));
        locals.var_t7_dn9 = (((((locals.var_pparam_b4soixrecd_dn9 * locals.var_t4) + (locals.var_pparam_b4soixrecd * locals.var_t4_dn9)) * locals.var_pparam_b4soinrecf0d) - (assign4750_e6292 * locals.var_pparam_b4soinrecf0d_dn9)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d));
        locals.var_t7_dn10 = (((((locals.var_pparam_b4soixrecd_dn10 * locals.var_t4) + (locals.var_pparam_b4soixrecd * locals.var_t4_dn10)) * locals.var_pparam_b4soinrecf0d) - (assign4750_e6292 * locals.var_pparam_b4soinrecf0d_dn10)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d));
        locals.var_t7_dn11 = (((((locals.var_pparam_b4soixrecd_dn11 * locals.var_t4) + (locals.var_pparam_b4soixrecd * locals.var_t4_dn11)) * locals.var_pparam_b4soinrecf0d) - (assign4750_e6292 * locals.var_pparam_b4soinrecf0d_dn11)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d));
        locals.var_t7_dn12 = (((((locals.var_pparam_b4soixrecd_dn12 * locals.var_t4) + (locals.var_pparam_b4soixrecd * locals.var_t4_dn12)) * locals.var_pparam_b4soinrecf0d) - (assign4750_e6292 * locals.var_pparam_b4soinrecf0d_dn12)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d));

        let assign4760_e6297: f64 = if locals.var_t7 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard545 = assign4760_e6297;

        let (assign4770_e6307, assign4770_e6307_d_n3, assign4770_e6307_d_n4, assign4770_e6307_d_n5, assign4770_e6307_d_n6, assign4770_e6307_d_n7, assign4770_e6307_d_n8, assign4770_e6307_d_n9, assign4770_e6307_d_n10, assign4770_e6307_d_n11, assign4770_e6307_d_n12,) = {
    if (locals.var_guard545 != 0.0) {
        let assign4770_e6302: f64 = (1.0 + locals.var_t7);
        let assign4770_e6304: f64 = (assign4770_e6302 - 100.0);
        let assign4770_e6305: f64 = (2.688117142e43 * assign4770_e6304);
        (assign4770_e6305, (2.688117142e43 * locals.var_t7_dn3), (2.688117142e43 * locals.var_t7_dn4), (2.688117142e43 * locals.var_t7_dn5), (2.688117142e43 * locals.var_t7_dn6), (2.688117142e43 * locals.var_t7_dn7), (2.688117142e43 * locals.var_t7_dn8), (2.688117142e43 * locals.var_t7_dn9), (2.688117142e43 * locals.var_t7_dn10), (2.688117142e43 * locals.var_t7_dn11), (2.688117142e43 * locals.var_t7_dn12),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign4770_e6307;
        locals.var_t2_dn3 = assign4770_e6307_d_n3;
        locals.var_t2_dn4 = assign4770_e6307_d_n4;
        locals.var_t2_dn5 = assign4770_e6307_d_n5;
        locals.var_t2_dn6 = assign4770_e6307_d_n6;
        locals.var_t2_dn7 = assign4770_e6307_d_n7;
        locals.var_t2_dn8 = assign4770_e6307_d_n8;
        locals.var_t2_dn9 = assign4770_e6307_d_n9;
        locals.var_t2_dn10 = assign4770_e6307_d_n10;
        locals.var_t2_dn11 = assign4770_e6307_d_n11;
        locals.var_t2_dn12 = assign4770_e6307_d_n12;

        let assign4780_e6310: f64 = (-100.0);
        let assign4780_e6311: f64 = if locals.var_t7 < assign4780_e6310 { 1.0 } else { 0.0 };
        locals.var_guard546 = assign4780_e6311;

    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign4790_e6318, assign4790_e6318_d_n3, assign4790_e6318_d_n4, assign4790_e6318_d_n5, assign4790_e6318_d_n6, assign4790_e6318_d_n7, assign4790_e6318_d_n8, assign4790_e6318_d_n9, assign4790_e6318_d_n10, assign4790_e6318_d_n11, assign4790_e6318_d_n12,) = {
    if ((locals.var_guard545 == 0.0) && (locals.var_guard546 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign4790_e6318;
        locals.var_t2_dn3 = assign4790_e6318_d_n3;
        locals.var_t2_dn4 = assign4790_e6318_d_n4;
        locals.var_t2_dn5 = assign4790_e6318_d_n5;
        locals.var_t2_dn6 = assign4790_e6318_d_n6;
        locals.var_t2_dn7 = assign4790_e6318_d_n7;
        locals.var_t2_dn8 = assign4790_e6318_d_n8;
        locals.var_t2_dn9 = assign4790_e6318_d_n9;
        locals.var_t2_dn10 = assign4790_e6318_d_n10;
        locals.var_t2_dn11 = assign4790_e6318_d_n11;
        locals.var_t2_dn12 = assign4790_e6318_d_n12;

        let (assign4800_e6327, assign4800_e6327_d_n3, assign4800_e6327_d_n4, assign4800_e6327_d_n5, assign4800_e6327_d_n6, assign4800_e6327_d_n7, assign4800_e6327_d_n8, assign4800_e6327_d_n9, assign4800_e6327_d_n10, assign4800_e6327_d_n11, assign4800_e6327_d_n12,) = {
    if ((locals.var_guard545 == 0.0) && (locals.var_guard546 == 0.0)) {
        let assign4800_e6325: f64 = (locals.var_t7).exp();
        (assign4800_e6325, (assign4800_e6325 * locals.var_t7_dn3), (assign4800_e6325 * locals.var_t7_dn4), (assign4800_e6325 * locals.var_t7_dn5), (assign4800_e6325 * locals.var_t7_dn6), (assign4800_e6325 * locals.var_t7_dn7), (assign4800_e6325 * locals.var_t7_dn8), (assign4800_e6325 * locals.var_t7_dn9), (assign4800_e6325 * locals.var_t7_dn10), (assign4800_e6325 * locals.var_t7_dn11), (assign4800_e6325 * locals.var_t7_dn12),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign4800_e6327;
        locals.var_t2_dn3 = assign4800_e6327_d_n3;
        locals.var_t2_dn4 = assign4800_e6327_d_n4;
        locals.var_t2_dn5 = assign4800_e6327_d_n5;
        locals.var_t2_dn6 = assign4800_e6327_d_n6;
        locals.var_t2_dn7 = assign4800_e6327_d_n7;
        locals.var_t2_dn8 = assign4800_e6327_d_n8;
        locals.var_t2_dn9 = assign4800_e6327_d_n9;
        locals.var_t2_dn10 = assign4800_e6327_d_n10;
        locals.var_t2_dn11 = assign4800_e6327_d_n11;
        locals.var_t2_dn12 = assign4800_e6327_d_n12;

        let assign4810_e6330: f64 = (locals.var_pparam_b4soiahlid * locals.var_t0);
        locals.var_pparam_b4soiahli0d = assign4810_e6330;
        locals.var_pparam_b4soiahli0d_dn3 = ((locals.var_pparam_b4soiahlid_dn3 * locals.var_t0) + (locals.var_pparam_b4soiahlid * locals.var_t0_dn3));
        locals.var_pparam_b4soiahli0d_dn4 = ((locals.var_pparam_b4soiahlid_dn4 * locals.var_t0) + (locals.var_pparam_b4soiahlid * locals.var_t0_dn4));
        locals.var_pparam_b4soiahli0d_dn5 = ((locals.var_pparam_b4soiahlid_dn5 * locals.var_t0) + (locals.var_pparam_b4soiahlid * locals.var_t0_dn5));
        locals.var_pparam_b4soiahli0d_dn6 = ((locals.var_pparam_b4soiahlid_dn6 * locals.var_t0) + (locals.var_pparam_b4soiahlid * locals.var_t0_dn6));
        locals.var_pparam_b4soiahli0d_dn7 = ((locals.var_pparam_b4soiahlid_dn7 * locals.var_t0) + (locals.var_pparam_b4soiahlid * locals.var_t0_dn7));
        locals.var_pparam_b4soiahli0d_dn8 = ((locals.var_pparam_b4soiahlid_dn8 * locals.var_t0) + (locals.var_pparam_b4soiahlid * locals.var_t0_dn8));
        locals.var_pparam_b4soiahli0d_dn9 = ((locals.var_pparam_b4soiahlid_dn9 * locals.var_t0) + (locals.var_pparam_b4soiahlid * locals.var_t0_dn9));
        locals.var_pparam_b4soiahli0d_dn10 = ((locals.var_pparam_b4soiahlid_dn10 * locals.var_t0) + (locals.var_pparam_b4soiahlid * locals.var_t0_dn10));
        locals.var_pparam_b4soiahli0d_dn11 = ((locals.var_pparam_b4soiahlid_dn11 * locals.var_t0) + (locals.var_pparam_b4soiahlid * locals.var_t0_dn11));
        locals.var_pparam_b4soiahli0d_dn12 = ((locals.var_pparam_b4soiahlid_dn12 * locals.var_t0) + (locals.var_pparam_b4soiahlid * locals.var_t0_dn12));

        let assign4820_e6333: f64 = (locals.var_pparam_b4soiidbjt * locals.var_t0);
        locals.var_pparam_b4soijbjtd = assign4820_e6333;
        locals.var_pparam_b4soijbjtd_dn3 = ((locals.var_pparam_b4soiidbjt_dn3 * locals.var_t0) + (locals.var_pparam_b4soiidbjt * locals.var_t0_dn3));
        locals.var_pparam_b4soijbjtd_dn4 = ((locals.var_pparam_b4soiidbjt_dn4 * locals.var_t0) + (locals.var_pparam_b4soiidbjt * locals.var_t0_dn4));
        locals.var_pparam_b4soijbjtd_dn5 = ((locals.var_pparam_b4soiidbjt_dn5 * locals.var_t0) + (locals.var_pparam_b4soiidbjt * locals.var_t0_dn5));
        locals.var_pparam_b4soijbjtd_dn6 = ((locals.var_pparam_b4soiidbjt_dn6 * locals.var_t0) + (locals.var_pparam_b4soiidbjt * locals.var_t0_dn6));
        locals.var_pparam_b4soijbjtd_dn7 = ((locals.var_pparam_b4soiidbjt_dn7 * locals.var_t0) + (locals.var_pparam_b4soiidbjt * locals.var_t0_dn7));
        locals.var_pparam_b4soijbjtd_dn8 = ((locals.var_pparam_b4soiidbjt_dn8 * locals.var_t0) + (locals.var_pparam_b4soiidbjt * locals.var_t0_dn8));
        locals.var_pparam_b4soijbjtd_dn9 = ((locals.var_pparam_b4soiidbjt_dn9 * locals.var_t0) + (locals.var_pparam_b4soiidbjt * locals.var_t0_dn9));
        locals.var_pparam_b4soijbjtd_dn10 = ((locals.var_pparam_b4soiidbjt_dn10 * locals.var_t0) + (locals.var_pparam_b4soiidbjt * locals.var_t0_dn10));
        locals.var_pparam_b4soijbjtd_dn11 = ((locals.var_pparam_b4soiidbjt_dn11 * locals.var_t0) + (locals.var_pparam_b4soiidbjt * locals.var_t0_dn11));
        locals.var_pparam_b4soijbjtd_dn12 = ((locals.var_pparam_b4soiidbjt_dn12 * locals.var_t0) + (locals.var_pparam_b4soiidbjt * locals.var_t0_dn12));

        let assign4830_e6336: f64 = (locals.var_pparam_b4soiiddif * locals.var_t1);
        locals.var_pparam_b4soijdifd = assign4830_e6336;
        locals.var_pparam_b4soijdifd_dn3 = ((locals.var_pparam_b4soiiddif_dn3 * locals.var_t1) + (locals.var_pparam_b4soiiddif * locals.var_t1_dn3));
        locals.var_pparam_b4soijdifd_dn4 = ((locals.var_pparam_b4soiiddif_dn4 * locals.var_t1) + (locals.var_pparam_b4soiiddif * locals.var_t1_dn4));
        locals.var_pparam_b4soijdifd_dn5 = ((locals.var_pparam_b4soiiddif_dn5 * locals.var_t1) + (locals.var_pparam_b4soiiddif * locals.var_t1_dn5));
        locals.var_pparam_b4soijdifd_dn6 = ((locals.var_pparam_b4soiiddif_dn6 * locals.var_t1) + (locals.var_pparam_b4soiiddif * locals.var_t1_dn6));
        locals.var_pparam_b4soijdifd_dn7 = ((locals.var_pparam_b4soiiddif_dn7 * locals.var_t1) + (locals.var_pparam_b4soiiddif * locals.var_t1_dn7));
        locals.var_pparam_b4soijdifd_dn8 = ((locals.var_pparam_b4soiiddif_dn8 * locals.var_t1) + (locals.var_pparam_b4soiiddif * locals.var_t1_dn8));
        locals.var_pparam_b4soijdifd_dn9 = ((locals.var_pparam_b4soiiddif_dn9 * locals.var_t1) + (locals.var_pparam_b4soiiddif * locals.var_t1_dn9));
        locals.var_pparam_b4soijdifd_dn10 = ((locals.var_pparam_b4soiiddif_dn10 * locals.var_t1) + (locals.var_pparam_b4soiiddif * locals.var_t1_dn10));
        locals.var_pparam_b4soijdifd_dn11 = ((locals.var_pparam_b4soiiddif_dn11 * locals.var_t1) + (locals.var_pparam_b4soiiddif * locals.var_t1_dn11));
        locals.var_pparam_b4soijdifd_dn12 = ((locals.var_pparam_b4soiiddif_dn12 * locals.var_t1) + (locals.var_pparam_b4soiiddif * locals.var_t1_dn12));

        let assign4840_e6339: f64 = (locals.var_pparam_b4soiidrec * locals.var_t2);
        locals.var_pparam_b4soijrecd = assign4840_e6339;
        locals.var_pparam_b4soijrecd_dn3 = ((locals.var_pparam_b4soiidrec_dn3 * locals.var_t2) + (locals.var_pparam_b4soiidrec * locals.var_t2_dn3));
        locals.var_pparam_b4soijrecd_dn4 = ((locals.var_pparam_b4soiidrec_dn4 * locals.var_t2) + (locals.var_pparam_b4soiidrec * locals.var_t2_dn4));
        locals.var_pparam_b4soijrecd_dn5 = ((locals.var_pparam_b4soiidrec_dn5 * locals.var_t2) + (locals.var_pparam_b4soiidrec * locals.var_t2_dn5));
        locals.var_pparam_b4soijrecd_dn6 = ((locals.var_pparam_b4soiidrec_dn6 * locals.var_t2) + (locals.var_pparam_b4soiidrec * locals.var_t2_dn6));
        locals.var_pparam_b4soijrecd_dn7 = ((locals.var_pparam_b4soiidrec_dn7 * locals.var_t2) + (locals.var_pparam_b4soiidrec * locals.var_t2_dn7));
        locals.var_pparam_b4soijrecd_dn8 = ((locals.var_pparam_b4soiidrec_dn8 * locals.var_t2) + (locals.var_pparam_b4soiidrec * locals.var_t2_dn8));
        locals.var_pparam_b4soijrecd_dn9 = ((locals.var_pparam_b4soiidrec_dn9 * locals.var_t2) + (locals.var_pparam_b4soiidrec * locals.var_t2_dn9));
        locals.var_pparam_b4soijrecd_dn10 = ((locals.var_pparam_b4soiidrec_dn10 * locals.var_t2) + (locals.var_pparam_b4soiidrec * locals.var_t2_dn10));
        locals.var_pparam_b4soijrecd_dn11 = ((locals.var_pparam_b4soiidrec_dn11 * locals.var_t2) + (locals.var_pparam_b4soiidrec * locals.var_t2_dn11));
        locals.var_pparam_b4soijrecd_dn12 = ((locals.var_pparam_b4soiidrec_dn12 * locals.var_t2) + (locals.var_pparam_b4soiidrec * locals.var_t2_dn12));

        let assign4850_e6342: f64 = (locals.var_pparam_b4soixtund * locals.var_trm1);
        locals.var_t7 = assign4850_e6342;
        locals.var_t7_dn3 = (locals.var_pparam_b4soixtund_dn3 * locals.var_trm1);
        locals.var_t7_dn4 = ((locals.var_pparam_b4soixtund_dn4 * locals.var_trm1) + (locals.var_pparam_b4soixtund * locals.var_trm1_dn4));
        locals.var_t7_dn5 = ((locals.var_pparam_b4soixtund_dn5 * locals.var_trm1) + (locals.var_pparam_b4soixtund * locals.var_trm1_dn5));
        locals.var_t7_dn6 = ((locals.var_pparam_b4soixtund_dn6 * locals.var_trm1) + (locals.var_pparam_b4soixtund * locals.var_trm1_dn6));
        locals.var_t7_dn7 = (locals.var_pparam_b4soixtund_dn7 * locals.var_trm1);
        locals.var_t7_dn8 = (locals.var_pparam_b4soixtund_dn8 * locals.var_trm1);
        locals.var_t7_dn9 = (locals.var_pparam_b4soixtund_dn9 * locals.var_trm1);
        locals.var_t7_dn10 = (locals.var_pparam_b4soixtund_dn10 * locals.var_trm1);
        locals.var_t7_dn11 = (locals.var_pparam_b4soixtund_dn11 * locals.var_trm1);
        locals.var_t7_dn12 = (locals.var_pparam_b4soixtund_dn12 * locals.var_trm1);

        let assign4860_e6345: f64 = if locals.var_t7 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard547 = assign4860_e6345;

        let (assign4870_e6355, assign4870_e6355_d_n3, assign4870_e6355_d_n4, assign4870_e6355_d_n5, assign4870_e6355_d_n6, assign4870_e6355_d_n7, assign4870_e6355_d_n8, assign4870_e6355_d_n9, assign4870_e6355_d_n10, assign4870_e6355_d_n11, assign4870_e6355_d_n12,) = {
    if (locals.var_guard547 != 0.0) {
        let assign4870_e6350: f64 = (1.0 + locals.var_t7);
        let assign4870_e6352: f64 = (assign4870_e6350 - 100.0);
        let assign4870_e6353: f64 = (2.688117142e43 * assign4870_e6352);
        (assign4870_e6353, (2.688117142e43 * locals.var_t7_dn3), (2.688117142e43 * locals.var_t7_dn4), (2.688117142e43 * locals.var_t7_dn5), (2.688117142e43 * locals.var_t7_dn6), (2.688117142e43 * locals.var_t7_dn7), (2.688117142e43 * locals.var_t7_dn8), (2.688117142e43 * locals.var_t7_dn9), (2.688117142e43 * locals.var_t7_dn10), (2.688117142e43 * locals.var_t7_dn11), (2.688117142e43 * locals.var_t7_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign4870_e6355;
        locals.var_t0_dn3 = assign4870_e6355_d_n3;
        locals.var_t0_dn4 = assign4870_e6355_d_n4;
        locals.var_t0_dn5 = assign4870_e6355_d_n5;
        locals.var_t0_dn6 = assign4870_e6355_d_n6;
        locals.var_t0_dn7 = assign4870_e6355_d_n7;
        locals.var_t0_dn8 = assign4870_e6355_d_n8;
        locals.var_t0_dn9 = assign4870_e6355_d_n9;
        locals.var_t0_dn10 = assign4870_e6355_d_n10;
        locals.var_t0_dn11 = assign4870_e6355_d_n11;
        locals.var_t0_dn12 = assign4870_e6355_d_n12;

        let assign4880_e6358: f64 = (-100.0);
        let assign4880_e6359: f64 = if locals.var_t7 < assign4880_e6358 { 1.0 } else { 0.0 };
        locals.var_guard548 = assign4880_e6359;

        let (assign4890_e6366, assign4890_e6366_d_n3, assign4890_e6366_d_n4, assign4890_e6366_d_n5, assign4890_e6366_d_n6, assign4890_e6366_d_n7, assign4890_e6366_d_n8, assign4890_e6366_d_n9, assign4890_e6366_d_n10, assign4890_e6366_d_n11, assign4890_e6366_d_n12,) = {
    if ((locals.var_guard547 == 0.0) && (locals.var_guard548 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign4890_e6366;
        locals.var_t0_dn3 = assign4890_e6366_d_n3;
        locals.var_t0_dn4 = assign4890_e6366_d_n4;
        locals.var_t0_dn5 = assign4890_e6366_d_n5;
        locals.var_t0_dn6 = assign4890_e6366_d_n6;
        locals.var_t0_dn7 = assign4890_e6366_d_n7;
        locals.var_t0_dn8 = assign4890_e6366_d_n8;
        locals.var_t0_dn9 = assign4890_e6366_d_n9;
        locals.var_t0_dn10 = assign4890_e6366_d_n10;
        locals.var_t0_dn11 = assign4890_e6366_d_n11;
        locals.var_t0_dn12 = assign4890_e6366_d_n12;

        let (assign4900_e6375, assign4900_e6375_d_n3, assign4900_e6375_d_n4, assign4900_e6375_d_n5, assign4900_e6375_d_n6, assign4900_e6375_d_n7, assign4900_e6375_d_n8, assign4900_e6375_d_n9, assign4900_e6375_d_n10, assign4900_e6375_d_n11, assign4900_e6375_d_n12,) = {
    if ((locals.var_guard547 == 0.0) && (locals.var_guard548 == 0.0)) {
        let assign4900_e6373: f64 = (locals.var_t7).exp();
        (assign4900_e6373, (assign4900_e6373 * locals.var_t7_dn3), (assign4900_e6373 * locals.var_t7_dn4), (assign4900_e6373 * locals.var_t7_dn5), (assign4900_e6373 * locals.var_t7_dn6), (assign4900_e6373 * locals.var_t7_dn7), (assign4900_e6373 * locals.var_t7_dn8), (assign4900_e6373 * locals.var_t7_dn9), (assign4900_e6373 * locals.var_t7_dn10), (assign4900_e6373 * locals.var_t7_dn11), (assign4900_e6373 * locals.var_t7_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign4900_e6375;
        locals.var_t0_dn3 = assign4900_e6375_d_n3;
        locals.var_t0_dn4 = assign4900_e6375_d_n4;
        locals.var_t0_dn5 = assign4900_e6375_d_n5;
        locals.var_t0_dn6 = assign4900_e6375_d_n6;
        locals.var_t0_dn7 = assign4900_e6375_d_n7;
        locals.var_t0_dn8 = assign4900_e6375_d_n8;
        locals.var_t0_dn9 = assign4900_e6375_d_n9;
        locals.var_t0_dn10 = assign4900_e6375_d_n10;
        locals.var_t0_dn11 = assign4900_e6375_d_n11;
        locals.var_t0_dn12 = assign4900_e6375_d_n12;

        let assign4910_e6378: f64 = (locals.var_pparam_b4soiidtun * locals.var_t0);
        locals.var_pparam_b4soijtund = assign4910_e6378;
        locals.var_pparam_b4soijtund_dn3 = ((locals.var_pparam_b4soiidtun_dn3 * locals.var_t0) + (locals.var_pparam_b4soiidtun * locals.var_t0_dn3));
        locals.var_pparam_b4soijtund_dn4 = ((locals.var_pparam_b4soiidtun_dn4 * locals.var_t0) + (locals.var_pparam_b4soiidtun * locals.var_t0_dn4));
        locals.var_pparam_b4soijtund_dn5 = ((locals.var_pparam_b4soiidtun_dn5 * locals.var_t0) + (locals.var_pparam_b4soiidtun * locals.var_t0_dn5));
        locals.var_pparam_b4soijtund_dn6 = ((locals.var_pparam_b4soiidtun_dn6 * locals.var_t0) + (locals.var_pparam_b4soiidtun * locals.var_t0_dn6));
        locals.var_pparam_b4soijtund_dn7 = ((locals.var_pparam_b4soiidtun_dn7 * locals.var_t0) + (locals.var_pparam_b4soiidtun * locals.var_t0_dn7));
        locals.var_pparam_b4soijtund_dn8 = ((locals.var_pparam_b4soiidtun_dn8 * locals.var_t0) + (locals.var_pparam_b4soiidtun * locals.var_t0_dn8));
        locals.var_pparam_b4soijtund_dn9 = ((locals.var_pparam_b4soiidtun_dn9 * locals.var_t0) + (locals.var_pparam_b4soiidtun * locals.var_t0_dn9));
        locals.var_pparam_b4soijtund_dn10 = ((locals.var_pparam_b4soiidtun_dn10 * locals.var_t0) + (locals.var_pparam_b4soiidtun * locals.var_t0_dn10));
        locals.var_pparam_b4soijtund_dn11 = ((locals.var_pparam_b4soiidtun_dn11 * locals.var_t0) + (locals.var_pparam_b4soiidtun * locals.var_t0_dn11));
        locals.var_pparam_b4soijtund_dn12 = ((locals.var_pparam_b4soiidtun_dn12 * locals.var_t0) + (locals.var_pparam_b4soiidtun * locals.var_t0_dn12));

        let assign4920_e6381: f64 = if locals.var_pparam_b4soinsub > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard549 = assign4920_e6381;

        let (assign4930_e6401, assign4930_e6401_d_n3, assign4930_e6401_d_n4, assign4930_e6401_d_n5, assign4930_e6401_d_n6, assign4930_e6401_d_n7, assign4930_e6401_d_n8, assign4930_e6401_d_n9, assign4930_e6401_d_n10, assign4930_e6401_d_n11, assign4930_e6401_d_n12,) = {
    if (locals.var_guard549 != 0.0) {
        let assign4930_e6384: f64 = (-p.p37);
        let assign4930_e6386: f64 = (assign4930_e6384 * locals.var_b4soivtm);
        let assign4930_e6389: f64 = (locals.var_pparam_b4soinpeak / locals.var_pparam_b4soinsub);
        let (assign4930_e6398, assign4930_e6398_d_n3, assign4930_e6398_d_n4, assign4930_e6398_d_n5, assign4930_e6398_d_n6, assign4930_e6398_d_n7, assign4930_e6398_d_n8, assign4930_e6398_d_n9, assign4930_e6398_d_n10, assign4930_e6398_d_n11, assign4930_e6398_d_n12,) = {
            if (assign4930_e6389 > 1e-38) {
                let assign4930_e6394: f64 = (locals.var_pparam_b4soinpeak / locals.var_pparam_b4soinsub);
                let assign4930_e6395: f64 = (assign4930_e6394).ln();
                (assign4930_e6395, ((((locals.var_pparam_b4soinpeak_dn3 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn3)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign4930_e6394), ((((locals.var_pparam_b4soinpeak_dn4 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn4)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign4930_e6394), ((((locals.var_pparam_b4soinpeak_dn5 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn5)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign4930_e6394), ((((locals.var_pparam_b4soinpeak_dn6 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn6)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign4930_e6394), ((((locals.var_pparam_b4soinpeak_dn7 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn7)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign4930_e6394), ((((locals.var_pparam_b4soinpeak_dn8 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn8)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign4930_e6394), ((((locals.var_pparam_b4soinpeak_dn9 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn9)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign4930_e6394), ((((locals.var_pparam_b4soinpeak_dn10 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn10)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign4930_e6394), ((((locals.var_pparam_b4soinpeak_dn11 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn11)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign4930_e6394), ((((locals.var_pparam_b4soinpeak_dn12 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn12)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign4930_e6394),)
            } else {
                let assign4930_e6397: f64 = (-87.49823353377374);
                (assign4930_e6397, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign4930_e6399: f64 = (assign4930_e6386 * assign4930_e6398);
        (assign4930_e6399, (assign4930_e6386 * assign4930_e6398_d_n3), (((assign4930_e6384 * locals.var_b4soivtm_dn4) * assign4930_e6398) + (assign4930_e6386 * assign4930_e6398_d_n4)), (((assign4930_e6384 * locals.var_b4soivtm_dn5) * assign4930_e6398) + (assign4930_e6386 * assign4930_e6398_d_n5)), (((assign4930_e6384 * locals.var_b4soivtm_dn6) * assign4930_e6398) + (assign4930_e6386 * assign4930_e6398_d_n6)), (assign4930_e6386 * assign4930_e6398_d_n7), (assign4930_e6386 * assign4930_e6398_d_n8), (assign4930_e6386 * assign4930_e6398_d_n9), (assign4930_e6386 * assign4930_e6398_d_n10), (assign4930_e6386 * assign4930_e6398_d_n11), (assign4930_e6386 * assign4930_e6398_d_n12),)
    } else {
        (locals.var_pparam_b4soivfbb, locals.var_pparam_b4soivfbb_dn3, locals.var_pparam_b4soivfbb_dn4, locals.var_pparam_b4soivfbb_dn5, locals.var_pparam_b4soivfbb_dn6, locals.var_pparam_b4soivfbb_dn7, locals.var_pparam_b4soivfbb_dn8, locals.var_pparam_b4soivfbb_dn9, locals.var_pparam_b4soivfbb_dn10, locals.var_pparam_b4soivfbb_dn11, locals.var_pparam_b4soivfbb_dn12,)
    }
};
        locals.var_pparam_b4soivfbb = assign4930_e6401;
        locals.var_pparam_b4soivfbb_dn3 = assign4930_e6401_d_n3;
        locals.var_pparam_b4soivfbb_dn4 = assign4930_e6401_d_n4;
        locals.var_pparam_b4soivfbb_dn5 = assign4930_e6401_d_n5;
        locals.var_pparam_b4soivfbb_dn6 = assign4930_e6401_d_n6;
        locals.var_pparam_b4soivfbb_dn7 = assign4930_e6401_d_n7;
        locals.var_pparam_b4soivfbb_dn8 = assign4930_e6401_d_n8;
        locals.var_pparam_b4soivfbb_dn9 = assign4930_e6401_d_n9;
        locals.var_pparam_b4soivfbb_dn10 = assign4930_e6401_d_n10;
        locals.var_pparam_b4soivfbb_dn11 = assign4930_e6401_d_n11;
        locals.var_pparam_b4soivfbb_dn12 = assign4930_e6401_d_n12;

        let (assign4940_e6428, assign4940_e6428_d_n3, assign4940_e6428_d_n4, assign4940_e6428_d_n5, assign4940_e6428_d_n6, assign4940_e6428_d_n7, assign4940_e6428_d_n8, assign4940_e6428_d_n9, assign4940_e6428_d_n10, assign4940_e6428_d_n11, assign4940_e6428_d_n12,) = {
    if (locals.var_guard549 == 0.0) {
        let assign4940_e6405: f64 = (-p.p37);
        let assign4940_e6407: f64 = (assign4940_e6405 * locals.var_b4soivtm);
        let assign4940_e6409: f64 = (-locals.var_pparam_b4soinpeak);
        let assign4940_e6411: f64 = (assign4940_e6409 * locals.var_pparam_b4soinsub);
        let (assign4940_e6421, assign4940_e6421_d_n3, assign4940_e6421_d_n4, assign4940_e6421_d_n5, assign4940_e6421_d_n6, assign4940_e6421_d_n7, assign4940_e6421_d_n8, assign4940_e6421_d_n9, assign4940_e6421_d_n10, assign4940_e6421_d_n11, assign4940_e6421_d_n12,) = {
            if (assign4940_e6411 > 1e-38) {
                let assign4940_e6415: f64 = (-locals.var_pparam_b4soinpeak);
                let assign4940_e6417: f64 = (assign4940_e6415 * locals.var_pparam_b4soinsub);
                let assign4940_e6418: f64 = (assign4940_e6417).ln();
                (assign4940_e6418, ((((-locals.var_pparam_b4soinpeak_dn3) * locals.var_pparam_b4soinsub) + (assign4940_e6415 * locals.var_pparam_b4soinsub_dn3)) / assign4940_e6417), ((((-locals.var_pparam_b4soinpeak_dn4) * locals.var_pparam_b4soinsub) + (assign4940_e6415 * locals.var_pparam_b4soinsub_dn4)) / assign4940_e6417), ((((-locals.var_pparam_b4soinpeak_dn5) * locals.var_pparam_b4soinsub) + (assign4940_e6415 * locals.var_pparam_b4soinsub_dn5)) / assign4940_e6417), ((((-locals.var_pparam_b4soinpeak_dn6) * locals.var_pparam_b4soinsub) + (assign4940_e6415 * locals.var_pparam_b4soinsub_dn6)) / assign4940_e6417), ((((-locals.var_pparam_b4soinpeak_dn7) * locals.var_pparam_b4soinsub) + (assign4940_e6415 * locals.var_pparam_b4soinsub_dn7)) / assign4940_e6417), ((((-locals.var_pparam_b4soinpeak_dn8) * locals.var_pparam_b4soinsub) + (assign4940_e6415 * locals.var_pparam_b4soinsub_dn8)) / assign4940_e6417), ((((-locals.var_pparam_b4soinpeak_dn9) * locals.var_pparam_b4soinsub) + (assign4940_e6415 * locals.var_pparam_b4soinsub_dn9)) / assign4940_e6417), ((((-locals.var_pparam_b4soinpeak_dn10) * locals.var_pparam_b4soinsub) + (assign4940_e6415 * locals.var_pparam_b4soinsub_dn10)) / assign4940_e6417), ((((-locals.var_pparam_b4soinpeak_dn11) * locals.var_pparam_b4soinsub) + (assign4940_e6415 * locals.var_pparam_b4soinsub_dn11)) / assign4940_e6417), ((((-locals.var_pparam_b4soinpeak_dn12) * locals.var_pparam_b4soinsub) + (assign4940_e6415 * locals.var_pparam_b4soinsub_dn12)) / assign4940_e6417),)
            } else {
                let assign4940_e6420: f64 = (-87.49823353377374);
                (assign4940_e6420, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign4940_e6424: f64 = (2.0 * locals.var_lln_ni);
        let assign4940_e6425: f64 = (assign4940_e6421 - assign4940_e6424);
        let assign4940_e6426: f64 = (assign4940_e6407 * assign4940_e6425);
        (assign4940_e6426, (assign4940_e6407 * assign4940_e6421_d_n3), (((assign4940_e6405 * locals.var_b4soivtm_dn4) * assign4940_e6425) + (assign4940_e6407 * (assign4940_e6421_d_n4 - (2.0 * locals.var_lln_ni_dn4)))), (((assign4940_e6405 * locals.var_b4soivtm_dn5) * assign4940_e6425) + (assign4940_e6407 * (assign4940_e6421_d_n5 - (2.0 * locals.var_lln_ni_dn5)))), (((assign4940_e6405 * locals.var_b4soivtm_dn6) * assign4940_e6425) + (assign4940_e6407 * (assign4940_e6421_d_n6 - (2.0 * locals.var_lln_ni_dn6)))), (assign4940_e6407 * assign4940_e6421_d_n7), (assign4940_e6407 * assign4940_e6421_d_n8), (assign4940_e6407 * assign4940_e6421_d_n9), (assign4940_e6407 * assign4940_e6421_d_n10), (assign4940_e6407 * assign4940_e6421_d_n11), (assign4940_e6407 * assign4940_e6421_d_n12),)
    } else {
        (locals.var_pparam_b4soivfbb, locals.var_pparam_b4soivfbb_dn3, locals.var_pparam_b4soivfbb_dn4, locals.var_pparam_b4soivfbb_dn5, locals.var_pparam_b4soivfbb_dn6, locals.var_pparam_b4soivfbb_dn7, locals.var_pparam_b4soivfbb_dn8, locals.var_pparam_b4soivfbb_dn9, locals.var_pparam_b4soivfbb_dn10, locals.var_pparam_b4soivfbb_dn11, locals.var_pparam_b4soivfbb_dn12,)
    }
};
        locals.var_pparam_b4soivfbb = assign4940_e6428;
        locals.var_pparam_b4soivfbb_dn3 = assign4940_e6428_d_n3;
        locals.var_pparam_b4soivfbb_dn4 = assign4940_e6428_d_n4;
        locals.var_pparam_b4soivfbb_dn5 = assign4940_e6428_d_n5;
        locals.var_pparam_b4soivfbb_dn6 = assign4940_e6428_d_n6;
        locals.var_pparam_b4soivfbb_dn7 = assign4940_e6428_d_n7;
        locals.var_pparam_b4soivfbb_dn8 = assign4940_e6428_d_n8;
        locals.var_pparam_b4soivfbb_dn9 = assign4940_e6428_d_n9;
        locals.var_pparam_b4soivfbb_dn10 = assign4940_e6428_d_n10;
        locals.var_pparam_b4soivfbb_dn11 = assign4940_e6428_d_n11;
        locals.var_pparam_b4soivfbb_dn12 = assign4940_e6428_d_n12;

        let assign4950_e6431: f64 = if (!param_given[353]) { 1.0 } else { 0.0 };
        locals.var_guard550 = assign4950_e6431;

        let assign4960_e6434: f64 = if locals.var_pparam_b4soinsub > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard551 = assign4960_e6434;

        let (assign4970_e6464, assign4970_e6464_d_n3, assign4970_e6464_d_n4, assign4970_e6464_d_n5, assign4970_e6464_d_n6, assign4970_e6464_d_n7, assign4970_e6464_d_n8, assign4970_e6464_d_n9, assign4970_e6464_d_n10, assign4970_e6464_d_n11, assign4970_e6464_d_n12,) = {
    if ((locals.var_guard550 != 0.0) && (locals.var_guard551 != 0.0)) {
        let assign4970_e6439: f64 = (-p.p37);
        let assign4970_e6443: f64 = (1e20 * locals.var_pparam_b4soinsub);
        let (assign4970_e6452, assign4970_e6452_d_n3, assign4970_e6452_d_n4, assign4970_e6452_d_n5, assign4970_e6452_d_n6, assign4970_e6452_d_n7, assign4970_e6452_d_n8, assign4970_e6452_d_n9, assign4970_e6452_d_n10, assign4970_e6452_d_n11, assign4970_e6452_d_n12,) = {
            if (assign4970_e6443 > 1e-38) {
                let assign4970_e6448: f64 = (1e20 * locals.var_pparam_b4soinsub);
                let assign4970_e6449: f64 = (assign4970_e6448).ln();
                (assign4970_e6449, ((1e20 * locals.var_pparam_b4soinsub_dn3) / assign4970_e6448), ((1e20 * locals.var_pparam_b4soinsub_dn4) / assign4970_e6448), ((1e20 * locals.var_pparam_b4soinsub_dn5) / assign4970_e6448), ((1e20 * locals.var_pparam_b4soinsub_dn6) / assign4970_e6448), ((1e20 * locals.var_pparam_b4soinsub_dn7) / assign4970_e6448), ((1e20 * locals.var_pparam_b4soinsub_dn8) / assign4970_e6448), ((1e20 * locals.var_pparam_b4soinsub_dn9) / assign4970_e6448), ((1e20 * locals.var_pparam_b4soinsub_dn10) / assign4970_e6448), ((1e20 * locals.var_pparam_b4soinsub_dn11) / assign4970_e6448), ((1e20 * locals.var_pparam_b4soinsub_dn12) / assign4970_e6448),)
            } else {
                let assign4970_e6451: f64 = (-87.49823353377374);
                (assign4970_e6451, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign4970_e6453: f64 = (locals.var_b4soivtm * assign4970_e6452);
        let assign4970_e6456: f64 = (locals.var_b4soivtm * 2.0);
        let assign4970_e6458: f64 = (assign4970_e6456 * locals.var_lln_ni);
        let assign4970_e6459: f64 = (assign4970_e6453 - assign4970_e6458);
        let assign4970_e6461: f64 = (assign4970_e6459 - 0.3);
        let assign4970_e6462: f64 = (assign4970_e6439 * assign4970_e6461);
        (assign4970_e6462, (assign4970_e6439 * (locals.var_b4soivtm * assign4970_e6452_d_n3)), (assign4970_e6439 * (((locals.var_b4soivtm_dn4 * assign4970_e6452) + (locals.var_b4soivtm * assign4970_e6452_d_n4)) - (((locals.var_b4soivtm_dn4 * 2.0) * locals.var_lln_ni) + (assign4970_e6456 * locals.var_lln_ni_dn4)))), (assign4970_e6439 * (((locals.var_b4soivtm_dn5 * assign4970_e6452) + (locals.var_b4soivtm * assign4970_e6452_d_n5)) - (((locals.var_b4soivtm_dn5 * 2.0) * locals.var_lln_ni) + (assign4970_e6456 * locals.var_lln_ni_dn5)))), (assign4970_e6439 * (((locals.var_b4soivtm_dn6 * assign4970_e6452) + (locals.var_b4soivtm * assign4970_e6452_d_n6)) - (((locals.var_b4soivtm_dn6 * 2.0) * locals.var_lln_ni) + (assign4970_e6456 * locals.var_lln_ni_dn6)))), (assign4970_e6439 * (locals.var_b4soivtm * assign4970_e6452_d_n7)), (assign4970_e6439 * (locals.var_b4soivtm * assign4970_e6452_d_n8)), (assign4970_e6439 * (locals.var_b4soivtm * assign4970_e6452_d_n9)), (assign4970_e6439 * (locals.var_b4soivtm * assign4970_e6452_d_n10)), (assign4970_e6439 * (locals.var_b4soivtm * assign4970_e6452_d_n11)), (assign4970_e6439 * (locals.var_b4soivtm * assign4970_e6452_d_n12)),)
    } else {
        (locals.var_pparam_b4soivsdfb, locals.var_pparam_b4soivsdfb_dn3, locals.var_pparam_b4soivsdfb_dn4, locals.var_pparam_b4soivsdfb_dn5, locals.var_pparam_b4soivsdfb_dn6, locals.var_pparam_b4soivsdfb_dn7, locals.var_pparam_b4soivsdfb_dn8, locals.var_pparam_b4soivsdfb_dn9, locals.var_pparam_b4soivsdfb_dn10, locals.var_pparam_b4soivsdfb_dn11, locals.var_pparam_b4soivsdfb_dn12,)
    }
};
        locals.var_pparam_b4soivsdfb = assign4970_e6464;
        locals.var_pparam_b4soivsdfb_dn3 = assign4970_e6464_d_n3;
        locals.var_pparam_b4soivsdfb_dn4 = assign4970_e6464_d_n4;
        locals.var_pparam_b4soivsdfb_dn5 = assign4970_e6464_d_n5;
        locals.var_pparam_b4soivsdfb_dn6 = assign4970_e6464_d_n6;
        locals.var_pparam_b4soivsdfb_dn7 = assign4970_e6464_d_n7;
        locals.var_pparam_b4soivsdfb_dn8 = assign4970_e6464_d_n8;
        locals.var_pparam_b4soivsdfb_dn9 = assign4970_e6464_d_n9;
        locals.var_pparam_b4soivsdfb_dn10 = assign4970_e6464_d_n10;
        locals.var_pparam_b4soivsdfb_dn11 = assign4970_e6464_d_n11;
        locals.var_pparam_b4soivsdfb_dn12 = assign4970_e6464_d_n12;

        let assign4980_e6467: f64 = if locals.var_pparam_b4soinsub < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard552 = assign4980_e6467;

        let (assign4990_e6496, assign4990_e6496_d_n3, assign4990_e6496_d_n4, assign4990_e6496_d_n5, assign4990_e6496_d_n6, assign4990_e6496_d_n7, assign4990_e6496_d_n8, assign4990_e6496_d_n9, assign4990_e6496_d_n10, assign4990_e6496_d_n11, assign4990_e6496_d_n12,) = {
    if (((locals.var_guard550 != 0.0) && (locals.var_guard551 == 0.0)) && (locals.var_guard552 != 0.0)) {
        let assign4990_e6475: f64 = (-p.p37);
        let assign4990_e6478: f64 = (-1e20);
        let assign4990_e6480: f64 = (assign4990_e6478 / locals.var_pparam_b4soinsub);
        let (assign4990_e6490, assign4990_e6490_d_n3, assign4990_e6490_d_n4, assign4990_e6490_d_n5, assign4990_e6490_d_n6, assign4990_e6490_d_n7, assign4990_e6490_d_n8, assign4990_e6490_d_n9, assign4990_e6490_d_n10, assign4990_e6490_d_n11, assign4990_e6490_d_n12,) = {
            if (assign4990_e6480 > 1e-38) {
                let assign4990_e6484: f64 = (-1e20);
                let assign4990_e6486: f64 = (assign4990_e6484 / locals.var_pparam_b4soinsub);
                let assign4990_e6487: f64 = (assign4990_e6486).ln();
                (assign4990_e6487, ((-((assign4990_e6484 * locals.var_pparam_b4soinsub_dn3) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub))) / assign4990_e6486), ((-((assign4990_e6484 * locals.var_pparam_b4soinsub_dn4) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub))) / assign4990_e6486), ((-((assign4990_e6484 * locals.var_pparam_b4soinsub_dn5) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub))) / assign4990_e6486), ((-((assign4990_e6484 * locals.var_pparam_b4soinsub_dn6) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub))) / assign4990_e6486), ((-((assign4990_e6484 * locals.var_pparam_b4soinsub_dn7) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub))) / assign4990_e6486), ((-((assign4990_e6484 * locals.var_pparam_b4soinsub_dn8) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub))) / assign4990_e6486), ((-((assign4990_e6484 * locals.var_pparam_b4soinsub_dn9) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub))) / assign4990_e6486), ((-((assign4990_e6484 * locals.var_pparam_b4soinsub_dn10) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub))) / assign4990_e6486), ((-((assign4990_e6484 * locals.var_pparam_b4soinsub_dn11) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub))) / assign4990_e6486), ((-((assign4990_e6484 * locals.var_pparam_b4soinsub_dn12) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub))) / assign4990_e6486),)
            } else {
                let assign4990_e6489: f64 = (-87.49823353377374);
                (assign4990_e6489, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign4990_e6491: f64 = (locals.var_b4soivtm * assign4990_e6490);
        let assign4990_e6493: f64 = (assign4990_e6491 + 0.3);
        let assign4990_e6494: f64 = (assign4990_e6475 * assign4990_e6493);
        (assign4990_e6494, (assign4990_e6475 * (locals.var_b4soivtm * assign4990_e6490_d_n3)), (assign4990_e6475 * ((locals.var_b4soivtm_dn4 * assign4990_e6490) + (locals.var_b4soivtm * assign4990_e6490_d_n4))), (assign4990_e6475 * ((locals.var_b4soivtm_dn5 * assign4990_e6490) + (locals.var_b4soivtm * assign4990_e6490_d_n5))), (assign4990_e6475 * ((locals.var_b4soivtm_dn6 * assign4990_e6490) + (locals.var_b4soivtm * assign4990_e6490_d_n6))), (assign4990_e6475 * (locals.var_b4soivtm * assign4990_e6490_d_n7)), (assign4990_e6475 * (locals.var_b4soivtm * assign4990_e6490_d_n8)), (assign4990_e6475 * (locals.var_b4soivtm * assign4990_e6490_d_n9)), (assign4990_e6475 * (locals.var_b4soivtm * assign4990_e6490_d_n10)), (assign4990_e6475 * (locals.var_b4soivtm * assign4990_e6490_d_n11)), (assign4990_e6475 * (locals.var_b4soivtm * assign4990_e6490_d_n12)),)
    } else {
        (locals.var_pparam_b4soivsdfb, locals.var_pparam_b4soivsdfb_dn3, locals.var_pparam_b4soivsdfb_dn4, locals.var_pparam_b4soivsdfb_dn5, locals.var_pparam_b4soivsdfb_dn6, locals.var_pparam_b4soivsdfb_dn7, locals.var_pparam_b4soivsdfb_dn8, locals.var_pparam_b4soivsdfb_dn9, locals.var_pparam_b4soivsdfb_dn10, locals.var_pparam_b4soivsdfb_dn11, locals.var_pparam_b4soivsdfb_dn12,)
    }
};
        locals.var_pparam_b4soivsdfb = assign4990_e6496;
        locals.var_pparam_b4soivsdfb_dn3 = assign4990_e6496_d_n3;
        locals.var_pparam_b4soivsdfb_dn4 = assign4990_e6496_d_n4;
        locals.var_pparam_b4soivsdfb_dn5 = assign4990_e6496_d_n5;
        locals.var_pparam_b4soivsdfb_dn6 = assign4990_e6496_d_n6;
        locals.var_pparam_b4soivsdfb_dn7 = assign4990_e6496_d_n7;
        locals.var_pparam_b4soivsdfb_dn8 = assign4990_e6496_d_n8;
        locals.var_pparam_b4soivsdfb_dn9 = assign4990_e6496_d_n9;
        locals.var_pparam_b4soivsdfb_dn10 = assign4990_e6496_d_n10;
        locals.var_pparam_b4soivsdfb_dn11 = assign4990_e6496_d_n11;
        locals.var_pparam_b4soivsdfb_dn12 = assign4990_e6496_d_n12;

        let assign5000_e6499: f64 = (2.0 * locals.var_b4soivtm);
        let assign5000_e6501: f64 = (locals.var_pparam_b4soinsub).abs();
        let (assign5000_e6509, assign5000_e6509_d_n3, assign5000_e6509_d_n4, assign5000_e6509_d_n5, assign5000_e6509_d_n6, assign5000_e6509_d_n7, assign5000_e6509_d_n8, assign5000_e6509_d_n9, assign5000_e6509_d_n10, assign5000_e6509_d_n11, assign5000_e6509_d_n12,) = {
    if (assign5000_e6501 > 1e-38) {
        let assign5000_e6505: f64 = (locals.var_pparam_b4soinsub).abs();
        let assign5000_e6506: f64 = (assign5000_e6505).ln();
        (assign5000_e6506, (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn3 } else { (-locals.var_pparam_b4soinsub_dn3) } / assign5000_e6505), (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn4 } else { (-locals.var_pparam_b4soinsub_dn4) } / assign5000_e6505), (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn5 } else { (-locals.var_pparam_b4soinsub_dn5) } / assign5000_e6505), (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn6 } else { (-locals.var_pparam_b4soinsub_dn6) } / assign5000_e6505), (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn7 } else { (-locals.var_pparam_b4soinsub_dn7) } / assign5000_e6505), (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn8 } else { (-locals.var_pparam_b4soinsub_dn8) } / assign5000_e6505), (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn9 } else { (-locals.var_pparam_b4soinsub_dn9) } / assign5000_e6505), (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn10 } else { (-locals.var_pparam_b4soinsub_dn10) } / assign5000_e6505), (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn11 } else { (-locals.var_pparam_b4soinsub_dn11) } / assign5000_e6505), (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn12 } else { (-locals.var_pparam_b4soinsub_dn12) } / assign5000_e6505),)
    } else {
        let assign5000_e6508: f64 = (-87.49823353377374);
        (assign5000_e6508, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let assign5000_e6511: f64 = (assign5000_e6509 - locals.var_lln_ni);
        let assign5000_e6512: f64 = (assign5000_e6499 * assign5000_e6511);
        locals.var_sdphi = assign5000_e6512;
        locals.var_sdphi_dn3 = (assign5000_e6499 * assign5000_e6509_d_n3);
        locals.var_sdphi_dn4 = (((2.0 * locals.var_b4soivtm_dn4) * assign5000_e6511) + (assign5000_e6499 * (assign5000_e6509_d_n4 - locals.var_lln_ni_dn4)));
        locals.var_sdphi_dn5 = (((2.0 * locals.var_b4soivtm_dn5) * assign5000_e6511) + (assign5000_e6499 * (assign5000_e6509_d_n5 - locals.var_lln_ni_dn5)));
        locals.var_sdphi_dn6 = (((2.0 * locals.var_b4soivtm_dn6) * assign5000_e6511) + (assign5000_e6499 * (assign5000_e6509_d_n6 - locals.var_lln_ni_dn6)));
        locals.var_sdphi_dn7 = (assign5000_e6499 * assign5000_e6509_d_n7);
        locals.var_sdphi_dn8 = (assign5000_e6499 * assign5000_e6509_d_n8);
        locals.var_sdphi_dn9 = (assign5000_e6499 * assign5000_e6509_d_n9);
        locals.var_sdphi_dn10 = (assign5000_e6499 * assign5000_e6509_d_n10);
        locals.var_sdphi_dn11 = (assign5000_e6499 * assign5000_e6509_d_n11);
        locals.var_sdphi_dn12 = (assign5000_e6499 * assign5000_e6509_d_n12);

        let assign5010_e6515: f64 = (locals.var_pparam_b4soinsub).abs();
        let assign5010_e6516: f64 = (assign5010_e6515).sqrt();
        let assign5010_e6517: f64 = (locals.var_sqrt2qeps * assign5010_e6516);
        let assign5010_e6519: f64 = (assign5010_e6517 / locals.var_b4soicbox);
        locals.var_sdgamma = assign5010_e6519;
        locals.var_sdgamma_dn3 = ((locals.var_sqrt2qeps * (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn3 } else { (-locals.var_pparam_b4soinsub_dn3) } / (2.0 * assign5010_e6516))) / locals.var_b4soicbox);
        locals.var_sdgamma_dn4 = ((locals.var_sqrt2qeps * (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn4 } else { (-locals.var_pparam_b4soinsub_dn4) } / (2.0 * assign5010_e6516))) / locals.var_b4soicbox);
        locals.var_sdgamma_dn5 = ((locals.var_sqrt2qeps * (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn5 } else { (-locals.var_pparam_b4soinsub_dn5) } / (2.0 * assign5010_e6516))) / locals.var_b4soicbox);
        locals.var_sdgamma_dn6 = ((locals.var_sqrt2qeps * (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn6 } else { (-locals.var_pparam_b4soinsub_dn6) } / (2.0 * assign5010_e6516))) / locals.var_b4soicbox);
        locals.var_sdgamma_dn7 = ((locals.var_sqrt2qeps * (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn7 } else { (-locals.var_pparam_b4soinsub_dn7) } / (2.0 * assign5010_e6516))) / locals.var_b4soicbox);
        locals.var_sdgamma_dn8 = ((locals.var_sqrt2qeps * (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn8 } else { (-locals.var_pparam_b4soinsub_dn8) } / (2.0 * assign5010_e6516))) / locals.var_b4soicbox);
        locals.var_sdgamma_dn9 = ((locals.var_sqrt2qeps * (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn9 } else { (-locals.var_pparam_b4soinsub_dn9) } / (2.0 * assign5010_e6516))) / locals.var_b4soicbox);
        locals.var_sdgamma_dn10 = ((locals.var_sqrt2qeps * (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn10 } else { (-locals.var_pparam_b4soinsub_dn10) } / (2.0 * assign5010_e6516))) / locals.var_b4soicbox);
        locals.var_sdgamma_dn11 = ((locals.var_sqrt2qeps * (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn11 } else { (-locals.var_pparam_b4soinsub_dn11) } / (2.0 * assign5010_e6516))) / locals.var_b4soicbox);
        locals.var_sdgamma_dn12 = ((locals.var_sqrt2qeps * (if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn12 } else { (-locals.var_pparam_b4soinsub_dn12) } / (2.0 * assign5010_e6516))) / locals.var_b4soicbox);

        let assign5020_e6522: f64 = if (!param_given[354]) { 1.0 } else { 0.0 };
        locals.var_guard553 = assign5020_e6522;

        let assign5030_e6537: f64 = if (((locals.var_pparam_b4soinsub > 0.0) && (p.p37 > 0.0)) || ((locals.var_pparam_b4soinsub < 0.0) && (p.p37 < 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard554 = assign5030_e6537;

        let (assign5040_e6550, assign5040_e6550_d_n3, assign5040_e6550_d_n4, assign5040_e6550_d_n5, assign5040_e6550_d_n6, assign5040_e6550_d_n7, assign5040_e6550_d_n8, assign5040_e6550_d_n9, assign5040_e6550_d_n10, assign5040_e6550_d_n11, assign5040_e6550_d_n12,) = {
    if ((locals.var_guard553 != 0.0) && (locals.var_guard554 != 0.0)) {
        let assign5040_e6543: f64 = (locals.var_pparam_b4soivsdfb + locals.var_sdphi);
        let assign5040_e6546: f64 = (locals.var_sdphi).sqrt();
        let assign5040_e6547: f64 = (locals.var_sdgamma * assign5040_e6546);
        let assign5040_e6548: f64 = (assign5040_e6543 + assign5040_e6547);
        (assign5040_e6548, ((locals.var_pparam_b4soivsdfb_dn3 + locals.var_sdphi_dn3) + ((locals.var_sdgamma_dn3 * assign5040_e6546) + (locals.var_sdgamma * (locals.var_sdphi_dn3 / (2.0 * assign5040_e6546))))), ((locals.var_pparam_b4soivsdfb_dn4 + locals.var_sdphi_dn4) + ((locals.var_sdgamma_dn4 * assign5040_e6546) + (locals.var_sdgamma * (locals.var_sdphi_dn4 / (2.0 * assign5040_e6546))))), ((locals.var_pparam_b4soivsdfb_dn5 + locals.var_sdphi_dn5) + ((locals.var_sdgamma_dn5 * assign5040_e6546) + (locals.var_sdgamma * (locals.var_sdphi_dn5 / (2.0 * assign5040_e6546))))), ((locals.var_pparam_b4soivsdfb_dn6 + locals.var_sdphi_dn6) + ((locals.var_sdgamma_dn6 * assign5040_e6546) + (locals.var_sdgamma * (locals.var_sdphi_dn6 / (2.0 * assign5040_e6546))))), ((locals.var_pparam_b4soivsdfb_dn7 + locals.var_sdphi_dn7) + ((locals.var_sdgamma_dn7 * assign5040_e6546) + (locals.var_sdgamma * (locals.var_sdphi_dn7 / (2.0 * assign5040_e6546))))), ((locals.var_pparam_b4soivsdfb_dn8 + locals.var_sdphi_dn8) + ((locals.var_sdgamma_dn8 * assign5040_e6546) + (locals.var_sdgamma * (locals.var_sdphi_dn8 / (2.0 * assign5040_e6546))))), ((locals.var_pparam_b4soivsdfb_dn9 + locals.var_sdphi_dn9) + ((locals.var_sdgamma_dn9 * assign5040_e6546) + (locals.var_sdgamma * (locals.var_sdphi_dn9 / (2.0 * assign5040_e6546))))), ((locals.var_pparam_b4soivsdfb_dn10 + locals.var_sdphi_dn10) + ((locals.var_sdgamma_dn10 * assign5040_e6546) + (locals.var_sdgamma * (locals.var_sdphi_dn10 / (2.0 * assign5040_e6546))))), ((locals.var_pparam_b4soivsdfb_dn11 + locals.var_sdphi_dn11) + ((locals.var_sdgamma_dn11 * assign5040_e6546) + (locals.var_sdgamma * (locals.var_sdphi_dn11 / (2.0 * assign5040_e6546))))), ((locals.var_pparam_b4soivsdfb_dn12 + locals.var_sdphi_dn12) + ((locals.var_sdgamma_dn12 * assign5040_e6546) + (locals.var_sdgamma * (locals.var_sdphi_dn12 / (2.0 * assign5040_e6546))))),)
    } else {
        (locals.var_pparam_b4soivsdth, locals.var_pparam_b4soivsdth_dn3, locals.var_pparam_b4soivsdth_dn4, locals.var_pparam_b4soivsdth_dn5, locals.var_pparam_b4soivsdth_dn6, locals.var_pparam_b4soivsdth_dn7, locals.var_pparam_b4soivsdth_dn8, locals.var_pparam_b4soivsdth_dn9, locals.var_pparam_b4soivsdth_dn10, locals.var_pparam_b4soivsdth_dn11, locals.var_pparam_b4soivsdth_dn12,)
    }
};
        locals.var_pparam_b4soivsdth = assign5040_e6550;
        locals.var_pparam_b4soivsdth_dn3 = assign5040_e6550_d_n3;
        locals.var_pparam_b4soivsdth_dn4 = assign5040_e6550_d_n4;
        locals.var_pparam_b4soivsdth_dn5 = assign5040_e6550_d_n5;
        locals.var_pparam_b4soivsdth_dn6 = assign5040_e6550_d_n6;
        locals.var_pparam_b4soivsdth_dn7 = assign5040_e6550_d_n7;
        locals.var_pparam_b4soivsdth_dn8 = assign5040_e6550_d_n8;
        locals.var_pparam_b4soivsdth_dn9 = assign5040_e6550_d_n9;
        locals.var_pparam_b4soivsdth_dn10 = assign5040_e6550_d_n10;
        locals.var_pparam_b4soivsdth_dn11 = assign5040_e6550_d_n11;
        locals.var_pparam_b4soivsdth_dn12 = assign5040_e6550_d_n12;

        let (assign5050_e6564, assign5050_e6564_d_n3, assign5050_e6564_d_n4, assign5050_e6564_d_n5, assign5050_e6564_d_n6, assign5050_e6564_d_n7, assign5050_e6564_d_n8, assign5050_e6564_d_n9, assign5050_e6564_d_n10, assign5050_e6564_d_n11, assign5050_e6564_d_n12,) = {
    if ((locals.var_guard553 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign5050_e6557: f64 = (locals.var_pparam_b4soivsdfb - locals.var_sdphi);
        let assign5050_e6560: f64 = (locals.var_sdphi).sqrt();
        let assign5050_e6561: f64 = (locals.var_sdgamma * assign5050_e6560);
        let assign5050_e6562: f64 = (assign5050_e6557 - assign5050_e6561);
        (assign5050_e6562, ((locals.var_pparam_b4soivsdfb_dn3 - locals.var_sdphi_dn3) - ((locals.var_sdgamma_dn3 * assign5050_e6560) + (locals.var_sdgamma * (locals.var_sdphi_dn3 / (2.0 * assign5050_e6560))))), ((locals.var_pparam_b4soivsdfb_dn4 - locals.var_sdphi_dn4) - ((locals.var_sdgamma_dn4 * assign5050_e6560) + (locals.var_sdgamma * (locals.var_sdphi_dn4 / (2.0 * assign5050_e6560))))), ((locals.var_pparam_b4soivsdfb_dn5 - locals.var_sdphi_dn5) - ((locals.var_sdgamma_dn5 * assign5050_e6560) + (locals.var_sdgamma * (locals.var_sdphi_dn5 / (2.0 * assign5050_e6560))))), ((locals.var_pparam_b4soivsdfb_dn6 - locals.var_sdphi_dn6) - ((locals.var_sdgamma_dn6 * assign5050_e6560) + (locals.var_sdgamma * (locals.var_sdphi_dn6 / (2.0 * assign5050_e6560))))), ((locals.var_pparam_b4soivsdfb_dn7 - locals.var_sdphi_dn7) - ((locals.var_sdgamma_dn7 * assign5050_e6560) + (locals.var_sdgamma * (locals.var_sdphi_dn7 / (2.0 * assign5050_e6560))))), ((locals.var_pparam_b4soivsdfb_dn8 - locals.var_sdphi_dn8) - ((locals.var_sdgamma_dn8 * assign5050_e6560) + (locals.var_sdgamma * (locals.var_sdphi_dn8 / (2.0 * assign5050_e6560))))), ((locals.var_pparam_b4soivsdfb_dn9 - locals.var_sdphi_dn9) - ((locals.var_sdgamma_dn9 * assign5050_e6560) + (locals.var_sdgamma * (locals.var_sdphi_dn9 / (2.0 * assign5050_e6560))))), ((locals.var_pparam_b4soivsdfb_dn10 - locals.var_sdphi_dn10) - ((locals.var_sdgamma_dn10 * assign5050_e6560) + (locals.var_sdgamma * (locals.var_sdphi_dn10 / (2.0 * assign5050_e6560))))), ((locals.var_pparam_b4soivsdfb_dn11 - locals.var_sdphi_dn11) - ((locals.var_sdgamma_dn11 * assign5050_e6560) + (locals.var_sdgamma * (locals.var_sdphi_dn11 / (2.0 * assign5050_e6560))))), ((locals.var_pparam_b4soivsdfb_dn12 - locals.var_sdphi_dn12) - ((locals.var_sdgamma_dn12 * assign5050_e6560) + (locals.var_sdgamma * (locals.var_sdphi_dn12 / (2.0 * assign5050_e6560))))),)
    } else {
        (locals.var_pparam_b4soivsdth, locals.var_pparam_b4soivsdth_dn3, locals.var_pparam_b4soivsdth_dn4, locals.var_pparam_b4soivsdth_dn5, locals.var_pparam_b4soivsdth_dn6, locals.var_pparam_b4soivsdth_dn7, locals.var_pparam_b4soivsdth_dn8, locals.var_pparam_b4soivsdth_dn9, locals.var_pparam_b4soivsdth_dn10, locals.var_pparam_b4soivsdth_dn11, locals.var_pparam_b4soivsdth_dn12,)
    }
};
        locals.var_pparam_b4soivsdth = assign5050_e6564;
        locals.var_pparam_b4soivsdth_dn3 = assign5050_e6564_d_n3;
        locals.var_pparam_b4soivsdth_dn4 = assign5050_e6564_d_n4;
        locals.var_pparam_b4soivsdth_dn5 = assign5050_e6564_d_n5;
        locals.var_pparam_b4soivsdth_dn6 = assign5050_e6564_d_n6;
        locals.var_pparam_b4soivsdth_dn7 = assign5050_e6564_d_n7;
        locals.var_pparam_b4soivsdth_dn8 = assign5050_e6564_d_n8;
        locals.var_pparam_b4soivsdth_dn9 = assign5050_e6564_d_n9;
        locals.var_pparam_b4soivsdth_dn10 = assign5050_e6564_d_n10;
        locals.var_pparam_b4soivsdth_dn11 = assign5050_e6564_d_n11;
        locals.var_pparam_b4soivsdth_dn12 = assign5050_e6564_d_n12;

        let assign5060_e6567: f64 = if (!param_given[355]) { 1.0 } else { 0.0 };
        locals.var_guard555 = assign5060_e6567;

        let (assign5070_e6583, assign5070_e6583_d_n3, assign5070_e6583_d_n4, assign5070_e6583_d_n5, assign5070_e6583_d_n6, assign5070_e6583_d_n7, assign5070_e6583_d_n8, assign5070_e6583_d_n9, assign5070_e6583_d_n10, assign5070_e6583_d_n11, assign5070_e6583_d_n12,) = {
    if (locals.var_guard555 != 0.0) {
        let assign5070_e6571: f64 = (2.0 * locals.var_epssub);
        let assign5070_e6573: f64 = (assign5070_e6571 * locals.var_sdphi);
        let assign5070_e6576: f64 = (locals.var_pparam_b4soinsub).abs();
        let assign5070_e6577: f64 = (1.602176462e-19 * assign5070_e6576);
        let assign5070_e6579: f64 = (assign5070_e6577 * 1000000.0);
        let assign5070_e6580: f64 = (assign5070_e6573 / assign5070_e6579);
        let assign5070_e6581: f64 = (assign5070_e6580).sqrt();
        (assign5070_e6581, (((((assign5070_e6571 * locals.var_sdphi_dn3) * assign5070_e6579) - (assign5070_e6573 * ((1.602176462e-19 * if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn3 } else { (-locals.var_pparam_b4soinsub_dn3) }) * 1000000.0))) / (assign5070_e6579 * assign5070_e6579)) / (2.0 * assign5070_e6581)), (((((assign5070_e6571 * locals.var_sdphi_dn4) * assign5070_e6579) - (assign5070_e6573 * ((1.602176462e-19 * if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn4 } else { (-locals.var_pparam_b4soinsub_dn4) }) * 1000000.0))) / (assign5070_e6579 * assign5070_e6579)) / (2.0 * assign5070_e6581)), (((((assign5070_e6571 * locals.var_sdphi_dn5) * assign5070_e6579) - (assign5070_e6573 * ((1.602176462e-19 * if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn5 } else { (-locals.var_pparam_b4soinsub_dn5) }) * 1000000.0))) / (assign5070_e6579 * assign5070_e6579)) / (2.0 * assign5070_e6581)), (((((assign5070_e6571 * locals.var_sdphi_dn6) * assign5070_e6579) - (assign5070_e6573 * ((1.602176462e-19 * if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn6 } else { (-locals.var_pparam_b4soinsub_dn6) }) * 1000000.0))) / (assign5070_e6579 * assign5070_e6579)) / (2.0 * assign5070_e6581)), (((((assign5070_e6571 * locals.var_sdphi_dn7) * assign5070_e6579) - (assign5070_e6573 * ((1.602176462e-19 * if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn7 } else { (-locals.var_pparam_b4soinsub_dn7) }) * 1000000.0))) / (assign5070_e6579 * assign5070_e6579)) / (2.0 * assign5070_e6581)), (((((assign5070_e6571 * locals.var_sdphi_dn8) * assign5070_e6579) - (assign5070_e6573 * ((1.602176462e-19 * if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn8 } else { (-locals.var_pparam_b4soinsub_dn8) }) * 1000000.0))) / (assign5070_e6579 * assign5070_e6579)) / (2.0 * assign5070_e6581)), (((((assign5070_e6571 * locals.var_sdphi_dn9) * assign5070_e6579) - (assign5070_e6573 * ((1.602176462e-19 * if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn9 } else { (-locals.var_pparam_b4soinsub_dn9) }) * 1000000.0))) / (assign5070_e6579 * assign5070_e6579)) / (2.0 * assign5070_e6581)), (((((assign5070_e6571 * locals.var_sdphi_dn10) * assign5070_e6579) - (assign5070_e6573 * ((1.602176462e-19 * if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn10 } else { (-locals.var_pparam_b4soinsub_dn10) }) * 1000000.0))) / (assign5070_e6579 * assign5070_e6579)) / (2.0 * assign5070_e6581)), (((((assign5070_e6571 * locals.var_sdphi_dn11) * assign5070_e6579) - (assign5070_e6573 * ((1.602176462e-19 * if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn11 } else { (-locals.var_pparam_b4soinsub_dn11) }) * 1000000.0))) / (assign5070_e6579 * assign5070_e6579)) / (2.0 * assign5070_e6581)), (((((assign5070_e6571 * locals.var_sdphi_dn12) * assign5070_e6579) - (assign5070_e6573 * ((1.602176462e-19 * if locals.var_pparam_b4soinsub >= 0.0 { locals.var_pparam_b4soinsub_dn12 } else { (-locals.var_pparam_b4soinsub_dn12) }) * 1000000.0))) / (assign5070_e6579 * assign5070_e6579)) / (2.0 * assign5070_e6581)),)
    } else {
        (locals.var_tmp, locals.var_tmp_dn3, locals.var_tmp_dn4, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, locals.var_tmp_dn9, locals.var_tmp_dn10, locals.var_tmp_dn11, locals.var_tmp_dn12,)
    }
};
        locals.var_tmp = assign5070_e6583;
        locals.var_tmp_dn3 = assign5070_e6583_d_n3;
        locals.var_tmp_dn4 = assign5070_e6583_d_n4;
        locals.var_tmp_dn5 = assign5070_e6583_d_n5;
        locals.var_tmp_dn6 = assign5070_e6583_d_n6;
        locals.var_tmp_dn7 = assign5070_e6583_d_n7;
        locals.var_tmp_dn8 = assign5070_e6583_d_n8;
        locals.var_tmp_dn9 = assign5070_e6583_d_n9;
        locals.var_tmp_dn10 = assign5070_e6583_d_n10;
        locals.var_tmp_dn11 = assign5070_e6583_d_n11;
        locals.var_tmp_dn12 = assign5070_e6583_d_n12;

        let (assign5080_e6589, assign5080_e6589_d_n3, assign5080_e6589_d_n4, assign5080_e6589_d_n5, assign5080_e6589_d_n6, assign5080_e6589_d_n7, assign5080_e6589_d_n8, assign5080_e6589_d_n9, assign5080_e6589_d_n10, assign5080_e6589_d_n11, assign5080_e6589_d_n12,) = {
    if (locals.var_guard555 != 0.0) {
        let assign5080_e6587: f64 = (locals.var_epssub / locals.var_tmp);
        (assign5080_e6587, (-((locals.var_epssub * locals.var_tmp_dn3) / (locals.var_tmp * locals.var_tmp))), (-((locals.var_epssub * locals.var_tmp_dn4) / (locals.var_tmp * locals.var_tmp))), (-((locals.var_epssub * locals.var_tmp_dn5) / (locals.var_tmp * locals.var_tmp))), (-((locals.var_epssub * locals.var_tmp_dn6) / (locals.var_tmp * locals.var_tmp))), (-((locals.var_epssub * locals.var_tmp_dn7) / (locals.var_tmp * locals.var_tmp))), (-((locals.var_epssub * locals.var_tmp_dn8) / (locals.var_tmp * locals.var_tmp))), (-((locals.var_epssub * locals.var_tmp_dn9) / (locals.var_tmp * locals.var_tmp))), (-((locals.var_epssub * locals.var_tmp_dn10) / (locals.var_tmp * locals.var_tmp))), (-((locals.var_epssub * locals.var_tmp_dn11) / (locals.var_tmp * locals.var_tmp))), (-((locals.var_epssub * locals.var_tmp_dn12) / (locals.var_tmp * locals.var_tmp))),)
    } else {
        (locals.var_tmp1, locals.var_tmp1_dn3, locals.var_tmp1_dn4, locals.var_tmp1_dn5, locals.var_tmp1_dn6, locals.var_tmp1_dn7, locals.var_tmp1_dn8, locals.var_tmp1_dn9, locals.var_tmp1_dn10, locals.var_tmp1_dn11, locals.var_tmp1_dn12,)
    }
};
        locals.var_tmp1 = assign5080_e6589;
        locals.var_tmp1_dn3 = assign5080_e6589_d_n3;
        locals.var_tmp1_dn4 = assign5080_e6589_d_n4;
        locals.var_tmp1_dn5 = assign5080_e6589_d_n5;
        locals.var_tmp1_dn6 = assign5080_e6589_d_n6;
        locals.var_tmp1_dn7 = assign5080_e6589_d_n7;
        locals.var_tmp1_dn8 = assign5080_e6589_d_n8;
        locals.var_tmp1_dn9 = assign5080_e6589_d_n9;
        locals.var_tmp1_dn10 = assign5080_e6589_d_n10;
        locals.var_tmp1_dn11 = assign5080_e6589_d_n11;
        locals.var_tmp1_dn12 = assign5080_e6589_d_n12;

        let (assign5090_e6599, assign5090_e6599_d_n3, assign5090_e6599_d_n4, assign5090_e6599_d_n5, assign5090_e6599_d_n6, assign5090_e6599_d_n7, assign5090_e6599_d_n8, assign5090_e6599_d_n9, assign5090_e6599_d_n10, assign5090_e6599_d_n11, assign5090_e6599_d_n12,) = {
    if (locals.var_guard555 != 0.0) {
        let assign5090_e6593: f64 = (locals.var_tmp1 * locals.var_b4soicbox);
        let assign5090_e6596: f64 = (locals.var_tmp1 + locals.var_b4soicbox);
        let assign5090_e6597: f64 = (assign5090_e6593 / assign5090_e6596);
        (assign5090_e6597, ((((locals.var_tmp1_dn3 * locals.var_b4soicbox) * assign5090_e6596) - (assign5090_e6593 * locals.var_tmp1_dn3)) / (assign5090_e6596 * assign5090_e6596)), ((((locals.var_tmp1_dn4 * locals.var_b4soicbox) * assign5090_e6596) - (assign5090_e6593 * locals.var_tmp1_dn4)) / (assign5090_e6596 * assign5090_e6596)), ((((locals.var_tmp1_dn5 * locals.var_b4soicbox) * assign5090_e6596) - (assign5090_e6593 * locals.var_tmp1_dn5)) / (assign5090_e6596 * assign5090_e6596)), ((((locals.var_tmp1_dn6 * locals.var_b4soicbox) * assign5090_e6596) - (assign5090_e6593 * locals.var_tmp1_dn6)) / (assign5090_e6596 * assign5090_e6596)), ((((locals.var_tmp1_dn7 * locals.var_b4soicbox) * assign5090_e6596) - (assign5090_e6593 * locals.var_tmp1_dn7)) / (assign5090_e6596 * assign5090_e6596)), ((((locals.var_tmp1_dn8 * locals.var_b4soicbox) * assign5090_e6596) - (assign5090_e6593 * locals.var_tmp1_dn8)) / (assign5090_e6596 * assign5090_e6596)), ((((locals.var_tmp1_dn9 * locals.var_b4soicbox) * assign5090_e6596) - (assign5090_e6593 * locals.var_tmp1_dn9)) / (assign5090_e6596 * assign5090_e6596)), ((((locals.var_tmp1_dn10 * locals.var_b4soicbox) * assign5090_e6596) - (assign5090_e6593 * locals.var_tmp1_dn10)) / (assign5090_e6596 * assign5090_e6596)), ((((locals.var_tmp1_dn11 * locals.var_b4soicbox) * assign5090_e6596) - (assign5090_e6593 * locals.var_tmp1_dn11)) / (assign5090_e6596 * assign5090_e6596)), ((((locals.var_tmp1_dn12 * locals.var_b4soicbox) * assign5090_e6596) - (assign5090_e6593 * locals.var_tmp1_dn12)) / (assign5090_e6596 * assign5090_e6596)),)
    } else {
        (locals.var_b4soicsdmin, locals.var_b4soicsdmin_dn3, locals.var_b4soicsdmin_dn4, locals.var_b4soicsdmin_dn5, locals.var_b4soicsdmin_dn6, locals.var_b4soicsdmin_dn7, locals.var_b4soicsdmin_dn8, locals.var_b4soicsdmin_dn9, locals.var_b4soicsdmin_dn10, locals.var_b4soicsdmin_dn11, locals.var_b4soicsdmin_dn12,)
    }
};
        locals.var_b4soicsdmin = assign5090_e6599;
        locals.var_b4soicsdmin_dn3 = assign5090_e6599_d_n3;
        locals.var_b4soicsdmin_dn4 = assign5090_e6599_d_n4;
        locals.var_b4soicsdmin_dn5 = assign5090_e6599_d_n5;
        locals.var_b4soicsdmin_dn6 = assign5090_e6599_d_n6;
        locals.var_b4soicsdmin_dn7 = assign5090_e6599_d_n7;
        locals.var_b4soicsdmin_dn8 = assign5090_e6599_d_n8;
        locals.var_b4soicsdmin_dn9 = assign5090_e6599_d_n9;
        locals.var_b4soicsdmin_dn10 = assign5090_e6599_d_n10;
        locals.var_b4soicsdmin_dn11 = assign5090_e6599_d_n11;
        locals.var_b4soicsdmin_dn12 = assign5090_e6599_d_n12;

    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign5100_e6602: f64 = (2.0 * locals.var_b4soivtm);
        let (assign5100_e6610, assign5100_e6610_d_n3, assign5100_e6610_d_n4, assign5100_e6610_d_n5, assign5100_e6610_d_n6, assign5100_e6610_d_n7, assign5100_e6610_d_n8, assign5100_e6610_d_n9, assign5100_e6610_d_n10, assign5100_e6610_d_n11, assign5100_e6610_d_n12,) = {
    if (locals.var_pparam_b4soinpeak > 1e-38) {
        let assign5100_e6607: f64 = (locals.var_pparam_b4soinpeak).ln();
        (assign5100_e6607, (locals.var_pparam_b4soinpeak_dn3 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn4 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn5 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn6 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn7 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn8 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn9 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn10 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn11 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn12 / locals.var_pparam_b4soinpeak),)
    } else {
        let assign5100_e6609: f64 = (-87.49823353377374);
        (assign5100_e6609, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let assign5100_e6612: f64 = (assign5100_e6610 - locals.var_lln_ni);
        let assign5100_e6613: f64 = (assign5100_e6602 * assign5100_e6612);
        locals.var_pparam_b4soiphi = assign5100_e6613;
        locals.var_pparam_b4soiphi_dn3 = (assign5100_e6602 * assign5100_e6610_d_n3);
        locals.var_pparam_b4soiphi_dn4 = (((2.0 * locals.var_b4soivtm_dn4) * assign5100_e6612) + (assign5100_e6602 * (assign5100_e6610_d_n4 - locals.var_lln_ni_dn4)));
        locals.var_pparam_b4soiphi_dn5 = (((2.0 * locals.var_b4soivtm_dn5) * assign5100_e6612) + (assign5100_e6602 * (assign5100_e6610_d_n5 - locals.var_lln_ni_dn5)));
        locals.var_pparam_b4soiphi_dn6 = (((2.0 * locals.var_b4soivtm_dn6) * assign5100_e6612) + (assign5100_e6602 * (assign5100_e6610_d_n6 - locals.var_lln_ni_dn6)));
        locals.var_pparam_b4soiphi_dn7 = (assign5100_e6602 * assign5100_e6610_d_n7);
        locals.var_pparam_b4soiphi_dn8 = (assign5100_e6602 * assign5100_e6610_d_n8);
        locals.var_pparam_b4soiphi_dn9 = (assign5100_e6602 * assign5100_e6610_d_n9);
        locals.var_pparam_b4soiphi_dn10 = (assign5100_e6602 * assign5100_e6610_d_n10);
        locals.var_pparam_b4soiphi_dn11 = (assign5100_e6602 * assign5100_e6610_d_n11);
        locals.var_pparam_b4soiphi_dn12 = (assign5100_e6602 * assign5100_e6610_d_n12);

        let assign5110_e6615: f64 = (locals.var_pparam_b4soiphi).sqrt();
        locals.var_pparam_b4soisqrtphi = assign5110_e6615;
        locals.var_pparam_b4soisqrtphi_dn3 = (locals.var_pparam_b4soiphi_dn3 / (2.0 * assign5110_e6615));
        locals.var_pparam_b4soisqrtphi_dn4 = (locals.var_pparam_b4soiphi_dn4 / (2.0 * assign5110_e6615));
        locals.var_pparam_b4soisqrtphi_dn5 = (locals.var_pparam_b4soiphi_dn5 / (2.0 * assign5110_e6615));
        locals.var_pparam_b4soisqrtphi_dn6 = (locals.var_pparam_b4soiphi_dn6 / (2.0 * assign5110_e6615));
        locals.var_pparam_b4soisqrtphi_dn7 = (locals.var_pparam_b4soiphi_dn7 / (2.0 * assign5110_e6615));
        locals.var_pparam_b4soisqrtphi_dn8 = (locals.var_pparam_b4soiphi_dn8 / (2.0 * assign5110_e6615));
        locals.var_pparam_b4soisqrtphi_dn9 = (locals.var_pparam_b4soiphi_dn9 / (2.0 * assign5110_e6615));
        locals.var_pparam_b4soisqrtphi_dn10 = (locals.var_pparam_b4soiphi_dn10 / (2.0 * assign5110_e6615));
        locals.var_pparam_b4soisqrtphi_dn11 = (locals.var_pparam_b4soiphi_dn11 / (2.0 * assign5110_e6615));
        locals.var_pparam_b4soisqrtphi_dn12 = (locals.var_pparam_b4soiphi_dn12 / (2.0 * assign5110_e6615));

        let assign5120_e6618: f64 = (2.0 * locals.var_epssub);
        let assign5120_e6621: f64 = (1.602176462e-19 * locals.var_pparam_b4soinpeak);
        let assign5120_e6623: f64 = (assign5120_e6621 * 1000000.0);
        let assign5120_e6624: f64 = (assign5120_e6618 / assign5120_e6623);
        let assign5120_e6625: f64 = (assign5120_e6624).sqrt();
        let assign5120_e6627: f64 = (assign5120_e6625 * locals.var_pparam_b4soisqrtphi);
        locals.var_pparam_b4soixdep0 = assign5120_e6627;
        locals.var_pparam_b4soixdep0_dn3 = ((((-((assign5120_e6618 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn3) * 1000000.0)) / (assign5120_e6623 * assign5120_e6623))) / (2.0 * assign5120_e6625)) * locals.var_pparam_b4soisqrtphi) + (assign5120_e6625 * locals.var_pparam_b4soisqrtphi_dn3));
        locals.var_pparam_b4soixdep0_dn4 = ((((-((assign5120_e6618 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn4) * 1000000.0)) / (assign5120_e6623 * assign5120_e6623))) / (2.0 * assign5120_e6625)) * locals.var_pparam_b4soisqrtphi) + (assign5120_e6625 * locals.var_pparam_b4soisqrtphi_dn4));
        locals.var_pparam_b4soixdep0_dn5 = ((((-((assign5120_e6618 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn5) * 1000000.0)) / (assign5120_e6623 * assign5120_e6623))) / (2.0 * assign5120_e6625)) * locals.var_pparam_b4soisqrtphi) + (assign5120_e6625 * locals.var_pparam_b4soisqrtphi_dn5));
        locals.var_pparam_b4soixdep0_dn6 = ((((-((assign5120_e6618 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn6) * 1000000.0)) / (assign5120_e6623 * assign5120_e6623))) / (2.0 * assign5120_e6625)) * locals.var_pparam_b4soisqrtphi) + (assign5120_e6625 * locals.var_pparam_b4soisqrtphi_dn6));
        locals.var_pparam_b4soixdep0_dn7 = ((((-((assign5120_e6618 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn7) * 1000000.0)) / (assign5120_e6623 * assign5120_e6623))) / (2.0 * assign5120_e6625)) * locals.var_pparam_b4soisqrtphi) + (assign5120_e6625 * locals.var_pparam_b4soisqrtphi_dn7));
        locals.var_pparam_b4soixdep0_dn8 = ((((-((assign5120_e6618 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn8) * 1000000.0)) / (assign5120_e6623 * assign5120_e6623))) / (2.0 * assign5120_e6625)) * locals.var_pparam_b4soisqrtphi) + (assign5120_e6625 * locals.var_pparam_b4soisqrtphi_dn8));
        locals.var_pparam_b4soixdep0_dn9 = ((((-((assign5120_e6618 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn9) * 1000000.0)) / (assign5120_e6623 * assign5120_e6623))) / (2.0 * assign5120_e6625)) * locals.var_pparam_b4soisqrtphi) + (assign5120_e6625 * locals.var_pparam_b4soisqrtphi_dn9));
        locals.var_pparam_b4soixdep0_dn10 = ((((-((assign5120_e6618 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn10) * 1000000.0)) / (assign5120_e6623 * assign5120_e6623))) / (2.0 * assign5120_e6625)) * locals.var_pparam_b4soisqrtphi) + (assign5120_e6625 * locals.var_pparam_b4soisqrtphi_dn10));
        locals.var_pparam_b4soixdep0_dn11 = ((((-((assign5120_e6618 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn11) * 1000000.0)) / (assign5120_e6623 * assign5120_e6623))) / (2.0 * assign5120_e6625)) * locals.var_pparam_b4soisqrtphi) + (assign5120_e6625 * locals.var_pparam_b4soisqrtphi_dn11));
        locals.var_pparam_b4soixdep0_dn12 = ((((-((assign5120_e6618 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn12) * 1000000.0)) / (assign5120_e6623 * assign5120_e6623))) / (2.0 * assign5120_e6625)) * locals.var_pparam_b4soisqrtphi) + (assign5120_e6625 * locals.var_pparam_b4soisqrtphi_dn12));

        let assign5130_e6629: f64 = (locals.var_pparam_b4soixdep0).sqrt();
        locals.var_pparam_b4soisqrtxdep0 = assign5130_e6629;
        locals.var_pparam_b4soisqrtxdep0_dn3 = (locals.var_pparam_b4soixdep0_dn3 / (2.0 * assign5130_e6629));
        locals.var_pparam_b4soisqrtxdep0_dn4 = (locals.var_pparam_b4soixdep0_dn4 / (2.0 * assign5130_e6629));
        locals.var_pparam_b4soisqrtxdep0_dn5 = (locals.var_pparam_b4soixdep0_dn5 / (2.0 * assign5130_e6629));
        locals.var_pparam_b4soisqrtxdep0_dn6 = (locals.var_pparam_b4soixdep0_dn6 / (2.0 * assign5130_e6629));
        locals.var_pparam_b4soisqrtxdep0_dn7 = (locals.var_pparam_b4soixdep0_dn7 / (2.0 * assign5130_e6629));
        locals.var_pparam_b4soisqrtxdep0_dn8 = (locals.var_pparam_b4soixdep0_dn8 / (2.0 * assign5130_e6629));
        locals.var_pparam_b4soisqrtxdep0_dn9 = (locals.var_pparam_b4soixdep0_dn9 / (2.0 * assign5130_e6629));
        locals.var_pparam_b4soisqrtxdep0_dn10 = (locals.var_pparam_b4soixdep0_dn10 / (2.0 * assign5130_e6629));
        locals.var_pparam_b4soisqrtxdep0_dn11 = (locals.var_pparam_b4soixdep0_dn11 / (2.0 * assign5130_e6629));
        locals.var_pparam_b4soisqrtxdep0_dn12 = (locals.var_pparam_b4soixdep0_dn12 / (2.0 * assign5130_e6629));

        let assign5140_e6632: f64 = if p.p41 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard556 = assign5140_e6632;

        let (assign5150_e6645, assign5150_e6645_d_n3, assign5150_e6645_d_n4, assign5150_e6645_d_n5, assign5150_e6645_d_n6, assign5150_e6645_d_n7, assign5150_e6645_d_n8, assign5150_e6645_d_n9, assign5150_e6645_d_n10, assign5150_e6645_d_n11, assign5150_e6645_d_n12,) = {
    if (locals.var_guard556 != 0.0) {
        let assign5150_e6636: f64 = (3.0 * 3.9);
        let assign5150_e6638: f64 = (assign5150_e6636 / locals.var_epsrox);
        let assign5150_e6640: f64 = (assign5150_e6638 * locals.var_pparam_b4soixj);
        let assign5150_e6642: f64 = (assign5150_e6640 * p.p66);
        let assign5150_e6643: f64 = (assign5150_e6642).sqrt();
        (assign5150_e6643, (((assign5150_e6638 * locals.var_pparam_b4soixj_dn3) * p.p66) / (2.0 * assign5150_e6643)), (((assign5150_e6638 * locals.var_pparam_b4soixj_dn4) * p.p66) / (2.0 * assign5150_e6643)), (((assign5150_e6638 * locals.var_pparam_b4soixj_dn5) * p.p66) / (2.0 * assign5150_e6643)), (((assign5150_e6638 * locals.var_pparam_b4soixj_dn6) * p.p66) / (2.0 * assign5150_e6643)), (((assign5150_e6638 * locals.var_pparam_b4soixj_dn7) * p.p66) / (2.0 * assign5150_e6643)), (((assign5150_e6638 * locals.var_pparam_b4soixj_dn8) * p.p66) / (2.0 * assign5150_e6643)), (((assign5150_e6638 * locals.var_pparam_b4soixj_dn9) * p.p66) / (2.0 * assign5150_e6643)), (((assign5150_e6638 * locals.var_pparam_b4soixj_dn10) * p.p66) / (2.0 * assign5150_e6643)), (((assign5150_e6638 * locals.var_pparam_b4soixj_dn11) * p.p66) / (2.0 * assign5150_e6643)), (((assign5150_e6638 * locals.var_pparam_b4soixj_dn12) * p.p66) / (2.0 * assign5150_e6643)),)
    } else {
        (locals.var_pparam_b4soilitl, locals.var_pparam_b4soilitl_dn3, locals.var_pparam_b4soilitl_dn4, locals.var_pparam_b4soilitl_dn5, locals.var_pparam_b4soilitl_dn6, locals.var_pparam_b4soilitl_dn7, locals.var_pparam_b4soilitl_dn8, locals.var_pparam_b4soilitl_dn9, locals.var_pparam_b4soilitl_dn10, locals.var_pparam_b4soilitl_dn11, locals.var_pparam_b4soilitl_dn12,)
    }
};
        locals.var_pparam_b4soilitl = assign5150_e6645;
        locals.var_pparam_b4soilitl_dn3 = assign5150_e6645_d_n3;
        locals.var_pparam_b4soilitl_dn4 = assign5150_e6645_d_n4;
        locals.var_pparam_b4soilitl_dn5 = assign5150_e6645_d_n5;
        locals.var_pparam_b4soilitl_dn6 = assign5150_e6645_d_n6;
        locals.var_pparam_b4soilitl_dn7 = assign5150_e6645_d_n7;
        locals.var_pparam_b4soilitl_dn8 = assign5150_e6645_d_n8;
        locals.var_pparam_b4soilitl_dn9 = assign5150_e6645_d_n9;
        locals.var_pparam_b4soilitl_dn10 = assign5150_e6645_d_n10;
        locals.var_pparam_b4soilitl_dn11 = assign5150_e6645_d_n11;
        locals.var_pparam_b4soilitl_dn12 = assign5150_e6645_d_n12;

        let (assign5160_e6659, assign5160_e6659_d_n3, assign5160_e6659_d_n4, assign5160_e6659_d_n5, assign5160_e6659_d_n6, assign5160_e6659_d_n7, assign5160_e6659_d_n8, assign5160_e6659_d_n9, assign5160_e6659_d_n10, assign5160_e6659_d_n11, assign5160_e6659_d_n12,) = {
    if (locals.var_guard556 == 0.0) {
        let assign5160_e6650: f64 = (locals.var_epssub * locals.var_pparam_b4soixj);
        let assign5160_e6652: f64 = (assign5160_e6650 * locals.var_toxe);
        let assign5160_e6655: f64 = (locals.var_epsrox * 8.85418e-12);
        let assign5160_e6656: f64 = (assign5160_e6652 / assign5160_e6655);
        let assign5160_e6657: f64 = (assign5160_e6656).sqrt();
        (assign5160_e6657, ((((locals.var_epssub * locals.var_pparam_b4soixj_dn3) * locals.var_toxe) / assign5160_e6655) / (2.0 * assign5160_e6657)), ((((locals.var_epssub * locals.var_pparam_b4soixj_dn4) * locals.var_toxe) / assign5160_e6655) / (2.0 * assign5160_e6657)), ((((locals.var_epssub * locals.var_pparam_b4soixj_dn5) * locals.var_toxe) / assign5160_e6655) / (2.0 * assign5160_e6657)), ((((locals.var_epssub * locals.var_pparam_b4soixj_dn6) * locals.var_toxe) / assign5160_e6655) / (2.0 * assign5160_e6657)), ((((locals.var_epssub * locals.var_pparam_b4soixj_dn7) * locals.var_toxe) / assign5160_e6655) / (2.0 * assign5160_e6657)), ((((locals.var_epssub * locals.var_pparam_b4soixj_dn8) * locals.var_toxe) / assign5160_e6655) / (2.0 * assign5160_e6657)), ((((locals.var_epssub * locals.var_pparam_b4soixj_dn9) * locals.var_toxe) / assign5160_e6655) / (2.0 * assign5160_e6657)), ((((locals.var_epssub * locals.var_pparam_b4soixj_dn10) * locals.var_toxe) / assign5160_e6655) / (2.0 * assign5160_e6657)), ((((locals.var_epssub * locals.var_pparam_b4soixj_dn11) * locals.var_toxe) / assign5160_e6655) / (2.0 * assign5160_e6657)), ((((locals.var_epssub * locals.var_pparam_b4soixj_dn12) * locals.var_toxe) / assign5160_e6655) / (2.0 * assign5160_e6657)),)
    } else {
        (locals.var_pparam_b4soilitl, locals.var_pparam_b4soilitl_dn3, locals.var_pparam_b4soilitl_dn4, locals.var_pparam_b4soilitl_dn5, locals.var_pparam_b4soilitl_dn6, locals.var_pparam_b4soilitl_dn7, locals.var_pparam_b4soilitl_dn8, locals.var_pparam_b4soilitl_dn9, locals.var_pparam_b4soilitl_dn10, locals.var_pparam_b4soilitl_dn11, locals.var_pparam_b4soilitl_dn12,)
    }
};
        locals.var_pparam_b4soilitl = assign5160_e6659;
        locals.var_pparam_b4soilitl_dn3 = assign5160_e6659_d_n3;
        locals.var_pparam_b4soilitl_dn4 = assign5160_e6659_d_n4;
        locals.var_pparam_b4soilitl_dn5 = assign5160_e6659_d_n5;
        locals.var_pparam_b4soilitl_dn6 = assign5160_e6659_d_n6;
        locals.var_pparam_b4soilitl_dn7 = assign5160_e6659_d_n7;
        locals.var_pparam_b4soilitl_dn8 = assign5160_e6659_d_n8;
        locals.var_pparam_b4soilitl_dn9 = assign5160_e6659_d_n9;
        locals.var_pparam_b4soilitl_dn10 = assign5160_e6659_d_n10;
        locals.var_pparam_b4soilitl_dn11 = assign5160_e6659_d_n11;
        locals.var_pparam_b4soilitl_dn12 = assign5160_e6659_d_n12;

        let assign5170_e6663: f64 = (1e20 * locals.var_pparam_b4soinpeak);
        let (assign5170_e6672, assign5170_e6672_d_n3, assign5170_e6672_d_n4, assign5170_e6672_d_n5, assign5170_e6672_d_n6, assign5170_e6672_d_n7, assign5170_e6672_d_n8, assign5170_e6672_d_n9, assign5170_e6672_d_n10, assign5170_e6672_d_n11, assign5170_e6672_d_n12,) = {
    if (assign5170_e6663 > 1e-38) {
        let assign5170_e6668: f64 = (1e20 * locals.var_pparam_b4soinpeak);
        let assign5170_e6669: f64 = (assign5170_e6668).ln();
        (assign5170_e6669, ((1e20 * locals.var_pparam_b4soinpeak_dn3) / assign5170_e6668), ((1e20 * locals.var_pparam_b4soinpeak_dn4) / assign5170_e6668), ((1e20 * locals.var_pparam_b4soinpeak_dn5) / assign5170_e6668), ((1e20 * locals.var_pparam_b4soinpeak_dn6) / assign5170_e6668), ((1e20 * locals.var_pparam_b4soinpeak_dn7) / assign5170_e6668), ((1e20 * locals.var_pparam_b4soinpeak_dn8) / assign5170_e6668), ((1e20 * locals.var_pparam_b4soinpeak_dn9) / assign5170_e6668), ((1e20 * locals.var_pparam_b4soinpeak_dn10) / assign5170_e6668), ((1e20 * locals.var_pparam_b4soinpeak_dn11) / assign5170_e6668), ((1e20 * locals.var_pparam_b4soinpeak_dn12) / assign5170_e6668),)
    } else {
        let assign5170_e6671: f64 = (-87.49823353377374);
        (assign5170_e6671, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let assign5170_e6675: f64 = (2.0 * locals.var_lln_ni);
        let assign5170_e6676: f64 = (assign5170_e6672 - assign5170_e6675);
        let assign5170_e6677: f64 = (locals.var_b4soivtm * assign5170_e6676);
        locals.var_pparam_b4soivbi = assign5170_e6677;
        locals.var_pparam_b4soivbi_dn3 = (locals.var_b4soivtm * assign5170_e6672_d_n3);
        locals.var_pparam_b4soivbi_dn4 = ((locals.var_b4soivtm_dn4 * assign5170_e6676) + (locals.var_b4soivtm * (assign5170_e6672_d_n4 - (2.0 * locals.var_lln_ni_dn4))));
        locals.var_pparam_b4soivbi_dn5 = ((locals.var_b4soivtm_dn5 * assign5170_e6676) + (locals.var_b4soivtm * (assign5170_e6672_d_n5 - (2.0 * locals.var_lln_ni_dn5))));
        locals.var_pparam_b4soivbi_dn6 = ((locals.var_b4soivtm_dn6 * assign5170_e6676) + (locals.var_b4soivtm * (assign5170_e6672_d_n6 - (2.0 * locals.var_lln_ni_dn6))));
        locals.var_pparam_b4soivbi_dn7 = (locals.var_b4soivtm * assign5170_e6672_d_n7);
        locals.var_pparam_b4soivbi_dn8 = (locals.var_b4soivtm * assign5170_e6672_d_n8);
        locals.var_pparam_b4soivbi_dn9 = (locals.var_b4soivtm * assign5170_e6672_d_n9);
        locals.var_pparam_b4soivbi_dn10 = (locals.var_b4soivtm * assign5170_e6672_d_n10);
        locals.var_pparam_b4soivbi_dn11 = (locals.var_b4soivtm * assign5170_e6672_d_n11);
        locals.var_pparam_b4soivbi_dn12 = (locals.var_b4soivtm * assign5170_e6672_d_n12);

        let assign5180_e6680: f64 = (1.602176462e-19 * locals.var_epssub);
        let assign5180_e6682: f64 = (assign5180_e6680 * locals.var_pparam_b4soinpeak);
        let assign5180_e6684: f64 = (assign5180_e6682 * 1000000.0);
        let assign5180_e6686: f64 = (assign5180_e6684 / 2.0);
        let assign5180_e6688: f64 = (assign5180_e6686 / locals.var_pparam_b4soiphi);
        let assign5180_e6689: f64 = (assign5180_e6688).sqrt();
        locals.var_pparam_b4soicdep0 = assign5180_e6689;
        locals.var_pparam_b4soicdep0_dn3 = (((((((assign5180_e6680 * locals.var_pparam_b4soinpeak_dn3) * 1000000.0) / 2.0) * locals.var_pparam_b4soiphi) - (assign5180_e6686 * locals.var_pparam_b4soiphi_dn3)) / (locals.var_pparam_b4soiphi * locals.var_pparam_b4soiphi)) / (2.0 * assign5180_e6689));
        locals.var_pparam_b4soicdep0_dn4 = (((((((assign5180_e6680 * locals.var_pparam_b4soinpeak_dn4) * 1000000.0) / 2.0) * locals.var_pparam_b4soiphi) - (assign5180_e6686 * locals.var_pparam_b4soiphi_dn4)) / (locals.var_pparam_b4soiphi * locals.var_pparam_b4soiphi)) / (2.0 * assign5180_e6689));
        locals.var_pparam_b4soicdep0_dn5 = (((((((assign5180_e6680 * locals.var_pparam_b4soinpeak_dn5) * 1000000.0) / 2.0) * locals.var_pparam_b4soiphi) - (assign5180_e6686 * locals.var_pparam_b4soiphi_dn5)) / (locals.var_pparam_b4soiphi * locals.var_pparam_b4soiphi)) / (2.0 * assign5180_e6689));
        locals.var_pparam_b4soicdep0_dn6 = (((((((assign5180_e6680 * locals.var_pparam_b4soinpeak_dn6) * 1000000.0) / 2.0) * locals.var_pparam_b4soiphi) - (assign5180_e6686 * locals.var_pparam_b4soiphi_dn6)) / (locals.var_pparam_b4soiphi * locals.var_pparam_b4soiphi)) / (2.0 * assign5180_e6689));
        locals.var_pparam_b4soicdep0_dn7 = (((((((assign5180_e6680 * locals.var_pparam_b4soinpeak_dn7) * 1000000.0) / 2.0) * locals.var_pparam_b4soiphi) - (assign5180_e6686 * locals.var_pparam_b4soiphi_dn7)) / (locals.var_pparam_b4soiphi * locals.var_pparam_b4soiphi)) / (2.0 * assign5180_e6689));
        locals.var_pparam_b4soicdep0_dn8 = (((((((assign5180_e6680 * locals.var_pparam_b4soinpeak_dn8) * 1000000.0) / 2.0) * locals.var_pparam_b4soiphi) - (assign5180_e6686 * locals.var_pparam_b4soiphi_dn8)) / (locals.var_pparam_b4soiphi * locals.var_pparam_b4soiphi)) / (2.0 * assign5180_e6689));
        locals.var_pparam_b4soicdep0_dn9 = (((((((assign5180_e6680 * locals.var_pparam_b4soinpeak_dn9) * 1000000.0) / 2.0) * locals.var_pparam_b4soiphi) - (assign5180_e6686 * locals.var_pparam_b4soiphi_dn9)) / (locals.var_pparam_b4soiphi * locals.var_pparam_b4soiphi)) / (2.0 * assign5180_e6689));
        locals.var_pparam_b4soicdep0_dn10 = (((((((assign5180_e6680 * locals.var_pparam_b4soinpeak_dn10) * 1000000.0) / 2.0) * locals.var_pparam_b4soiphi) - (assign5180_e6686 * locals.var_pparam_b4soiphi_dn10)) / (locals.var_pparam_b4soiphi * locals.var_pparam_b4soiphi)) / (2.0 * assign5180_e6689));
        locals.var_pparam_b4soicdep0_dn11 = (((((((assign5180_e6680 * locals.var_pparam_b4soinpeak_dn11) * 1000000.0) / 2.0) * locals.var_pparam_b4soiphi) - (assign5180_e6686 * locals.var_pparam_b4soiphi_dn11)) / (locals.var_pparam_b4soiphi * locals.var_pparam_b4soiphi)) / (2.0 * assign5180_e6689));
        locals.var_pparam_b4soicdep0_dn12 = (((((((assign5180_e6680 * locals.var_pparam_b4soinpeak_dn12) * 1000000.0) / 2.0) * locals.var_pparam_b4soiphi) - (assign5180_e6686 * locals.var_pparam_b4soiphi_dn12)) / (locals.var_pparam_b4soiphi * locals.var_pparam_b4soiphi)) / (2.0 * assign5180_e6689));

        let assign5190_e6692: f64 = if p.p41 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard557 = assign5190_e6692;

        let assign5200_e6695: f64 = if locals.var_pparam_b4soingate > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard558 = assign5200_e6695;

        let (assign5210_e6714, assign5210_e6714_d_n3, assign5210_e6714_d_n4, assign5210_e6714_d_n5, assign5210_e6714_d_n6, assign5210_e6714_d_n7, assign5210_e6714_d_n8, assign5210_e6714_d_n9, assign5210_e6714_d_n10, assign5210_e6714_d_n11, assign5210_e6714_d_n12,) = {
    if ((locals.var_guard557 != 0.0) && (locals.var_guard558 != 0.0)) {
        let assign5210_e6702: f64 = (locals.var_pparam_b4soingate / 1e20);
        let (assign5210_e6711, assign5210_e6711_d_n3, assign5210_e6711_d_n4, assign5210_e6711_d_n5, assign5210_e6711_d_n6, assign5210_e6711_d_n7, assign5210_e6711_d_n8, assign5210_e6711_d_n9, assign5210_e6711_d_n10, assign5210_e6711_d_n11, assign5210_e6711_d_n12,) = {
            if (assign5210_e6702 > 1e-38) {
                let assign5210_e6707: f64 = (locals.var_pparam_b4soingate / 1e20);
                let assign5210_e6708: f64 = (assign5210_e6707).ln();
                (assign5210_e6708, ((locals.var_pparam_b4soingate_dn3 / 1e20) / assign5210_e6707), ((locals.var_pparam_b4soingate_dn4 / 1e20) / assign5210_e6707), ((locals.var_pparam_b4soingate_dn5 / 1e20) / assign5210_e6707), ((locals.var_pparam_b4soingate_dn6 / 1e20) / assign5210_e6707), ((locals.var_pparam_b4soingate_dn7 / 1e20) / assign5210_e6707), ((locals.var_pparam_b4soingate_dn8 / 1e20) / assign5210_e6707), ((locals.var_pparam_b4soingate_dn9 / 1e20) / assign5210_e6707), ((locals.var_pparam_b4soingate_dn10 / 1e20) / assign5210_e6707), ((locals.var_pparam_b4soingate_dn11 / 1e20) / assign5210_e6707), ((locals.var_pparam_b4soingate_dn12 / 1e20) / assign5210_e6707),)
            } else {
                let assign5210_e6710: f64 = (-87.49823353377374);
                (assign5210_e6710, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign5210_e6712: f64 = (locals.var_vtm0 * assign5210_e6711);
        (assign5210_e6712, (locals.var_vtm0 * assign5210_e6711_d_n3), (locals.var_vtm0 * assign5210_e6711_d_n4), (locals.var_vtm0 * assign5210_e6711_d_n5), (locals.var_vtm0 * assign5210_e6711_d_n6), (locals.var_vtm0 * assign5210_e6711_d_n7), (locals.var_vtm0 * assign5210_e6711_d_n8), (locals.var_vtm0 * assign5210_e6711_d_n9), (locals.var_vtm0 * assign5210_e6711_d_n10), (locals.var_vtm0 * assign5210_e6711_d_n11), (locals.var_vtm0 * assign5210_e6711_d_n12),)
    } else {
        (locals.var_pparam_b4soivfbsd, locals.var_pparam_b4soivfbsd_dn3, locals.var_pparam_b4soivfbsd_dn4, locals.var_pparam_b4soivfbsd_dn5, locals.var_pparam_b4soivfbsd_dn6, locals.var_pparam_b4soivfbsd_dn7, locals.var_pparam_b4soivfbsd_dn8, locals.var_pparam_b4soivfbsd_dn9, locals.var_pparam_b4soivfbsd_dn10, locals.var_pparam_b4soivfbsd_dn11, locals.var_pparam_b4soivfbsd_dn12,)
    }
};
        locals.var_pparam_b4soivfbsd = assign5210_e6714;
        locals.var_pparam_b4soivfbsd_dn3 = assign5210_e6714_d_n3;
        locals.var_pparam_b4soivfbsd_dn4 = assign5210_e6714_d_n4;
        locals.var_pparam_b4soivfbsd_dn5 = assign5210_e6714_d_n5;
        locals.var_pparam_b4soivfbsd_dn6 = assign5210_e6714_d_n6;
        locals.var_pparam_b4soivfbsd_dn7 = assign5210_e6714_d_n7;
        locals.var_pparam_b4soivfbsd_dn8 = assign5210_e6714_d_n8;
        locals.var_pparam_b4soivfbsd_dn9 = assign5210_e6714_d_n9;
        locals.var_pparam_b4soivfbsd_dn10 = assign5210_e6714_d_n10;
        locals.var_pparam_b4soivfbsd_dn11 = assign5210_e6714_d_n11;
        locals.var_pparam_b4soivfbsd_dn12 = assign5210_e6714_d_n12;

        let (assign5220_e6721, assign5220_e6721_d_n3, assign5220_e6721_d_n4, assign5220_e6721_d_n5, assign5220_e6721_d_n6, assign5220_e6721_d_n7, assign5220_e6721_d_n8, assign5220_e6721_d_n9, assign5220_e6721_d_n10, assign5220_e6721_d_n11, assign5220_e6721_d_n12,) = {
    if ((locals.var_guard557 != 0.0) && (locals.var_guard558 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soivfbsd, locals.var_pparam_b4soivfbsd_dn3, locals.var_pparam_b4soivfbsd_dn4, locals.var_pparam_b4soivfbsd_dn5, locals.var_pparam_b4soivfbsd_dn6, locals.var_pparam_b4soivfbsd_dn7, locals.var_pparam_b4soivfbsd_dn8, locals.var_pparam_b4soivfbsd_dn9, locals.var_pparam_b4soivfbsd_dn10, locals.var_pparam_b4soivfbsd_dn11, locals.var_pparam_b4soivfbsd_dn12,)
    }
};
        locals.var_pparam_b4soivfbsd = assign5220_e6721;
        locals.var_pparam_b4soivfbsd_dn3 = assign5220_e6721_d_n3;
        locals.var_pparam_b4soivfbsd_dn4 = assign5220_e6721_d_n4;
        locals.var_pparam_b4soivfbsd_dn5 = assign5220_e6721_d_n5;
        locals.var_pparam_b4soivfbsd_dn6 = assign5220_e6721_d_n6;
        locals.var_pparam_b4soivfbsd_dn7 = assign5220_e6721_d_n7;
        locals.var_pparam_b4soivfbsd_dn8 = assign5220_e6721_d_n8;
        locals.var_pparam_b4soivfbsd_dn9 = assign5220_e6721_d_n9;
        locals.var_pparam_b4soivfbsd_dn10 = assign5220_e6721_d_n10;
        locals.var_pparam_b4soivfbsd_dn11 = assign5220_e6721_d_n11;
        locals.var_pparam_b4soivfbsd_dn12 = assign5220_e6721_d_n12;

        let (assign5230_e6737, assign5230_e6737_d_n3, assign5230_e6737_d_n4, assign5230_e6737_d_n5, assign5230_e6737_d_n6, assign5230_e6737_d_n7, assign5230_e6737_d_n8, assign5230_e6737_d_n9, assign5230_e6737_d_n10, assign5230_e6737_d_n11, assign5230_e6737_d_n12,) = {
    if (locals.var_guard557 == 0.0) {
        let (assign5230_e6732, assign5230_e6732_d_n3, assign5230_e6732_d_n4, assign5230_e6732_d_n5, assign5230_e6732_d_n6, assign5230_e6732_d_n7, assign5230_e6732_d_n8, assign5230_e6732_d_n9, assign5230_e6732_d_n10, assign5230_e6732_d_n11, assign5230_e6732_d_n12,) = {
            if (locals.var_pparam_b4soinsd > 1e-38) {
                let assign5230_e6729: f64 = (locals.var_pparam_b4soinsd).ln();
                (assign5230_e6729, (locals.var_pparam_b4soinsd_dn3 / locals.var_pparam_b4soinsd), (locals.var_pparam_b4soinsd_dn4 / locals.var_pparam_b4soinsd), (locals.var_pparam_b4soinsd_dn5 / locals.var_pparam_b4soinsd), (locals.var_pparam_b4soinsd_dn6 / locals.var_pparam_b4soinsd), (locals.var_pparam_b4soinsd_dn7 / locals.var_pparam_b4soinsd), (locals.var_pparam_b4soinsd_dn8 / locals.var_pparam_b4soinsd), (locals.var_pparam_b4soinsd_dn9 / locals.var_pparam_b4soinsd), (locals.var_pparam_b4soinsd_dn10 / locals.var_pparam_b4soinsd), (locals.var_pparam_b4soinsd_dn11 / locals.var_pparam_b4soinsd), (locals.var_pparam_b4soinsd_dn12 / locals.var_pparam_b4soinsd),)
            } else {
                let assign5230_e6731: f64 = (-87.49823353377374);
                (assign5230_e6731, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign5230_e6734: f64 = (assign5230_e6732 - locals.var_lln_ni);
        let assign5230_e6735: f64 = (locals.var_vtm0 * assign5230_e6734);
        (assign5230_e6735, (locals.var_vtm0 * assign5230_e6732_d_n3), (locals.var_vtm0 * (assign5230_e6732_d_n4 - locals.var_lln_ni_dn4)), (locals.var_vtm0 * (assign5230_e6732_d_n5 - locals.var_lln_ni_dn5)), (locals.var_vtm0 * (assign5230_e6732_d_n6 - locals.var_lln_ni_dn6)), (locals.var_vtm0 * assign5230_e6732_d_n7), (locals.var_vtm0 * assign5230_e6732_d_n8), (locals.var_vtm0 * assign5230_e6732_d_n9), (locals.var_vtm0 * assign5230_e6732_d_n10), (locals.var_vtm0 * assign5230_e6732_d_n11), (locals.var_vtm0 * assign5230_e6732_d_n12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign5230_e6737;
        locals.var_t0_dn3 = assign5230_e6737_d_n3;
        locals.var_t0_dn4 = assign5230_e6737_d_n4;
        locals.var_t0_dn5 = assign5230_e6737_d_n5;
        locals.var_t0_dn6 = assign5230_e6737_d_n6;
        locals.var_t0_dn7 = assign5230_e6737_d_n7;
        locals.var_t0_dn8 = assign5230_e6737_d_n8;
        locals.var_t0_dn9 = assign5230_e6737_d_n9;
        locals.var_t0_dn10 = assign5230_e6737_d_n10;
        locals.var_t0_dn11 = assign5230_e6737_d_n11;
        locals.var_t0_dn12 = assign5230_e6737_d_n12;

        let (assign5240_e6744, assign5240_e6744_d_n3, assign5240_e6744_d_n4, assign5240_e6744_d_n5, assign5240_e6744_d_n6, assign5240_e6744_d_n7, assign5240_e6744_d_n8, assign5240_e6744_d_n9, assign5240_e6744_d_n10, assign5240_e6744_d_n11, assign5240_e6744_d_n12,) = {
    if (locals.var_guard557 == 0.0) {
        let assign5240_e6742: f64 = (0.5 * locals.var_eg0);
        (assign5240_e6742, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign5240_e6744;
        locals.var_t1_dn3 = assign5240_e6744_d_n3;
        locals.var_t1_dn4 = assign5240_e6744_d_n4;
        locals.var_t1_dn5 = assign5240_e6744_d_n5;
        locals.var_t1_dn6 = assign5240_e6744_d_n6;
        locals.var_t1_dn7 = assign5240_e6744_d_n7;
        locals.var_t1_dn8 = assign5240_e6744_d_n8;
        locals.var_t1_dn9 = assign5240_e6744_d_n9;
        locals.var_t1_dn10 = assign5240_e6744_d_n10;
        locals.var_t1_dn11 = assign5240_e6744_d_n11;
        locals.var_t1_dn12 = assign5240_e6744_d_n12;

        let assign5250_e6747: f64 = if locals.var_t0 > locals.var_t1 { 1.0 } else { 0.0 };
        locals.var_guard559 = assign5250_e6747;

        let (assign5260_e6754, assign5260_e6754_d_n3, assign5260_e6754_d_n4, assign5260_e6754_d_n5, assign5260_e6754_d_n6, assign5260_e6754_d_n7, assign5260_e6754_d_n8, assign5260_e6754_d_n9, assign5260_e6754_d_n10, assign5260_e6754_d_n11, assign5260_e6754_d_n12,) = {
    if ((locals.var_guard557 == 0.0) && (locals.var_guard559 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign5260_e6754;
        locals.var_t0_dn3 = assign5260_e6754_d_n3;
        locals.var_t0_dn4 = assign5260_e6754_d_n4;
        locals.var_t0_dn5 = assign5260_e6754_d_n5;
        locals.var_t0_dn6 = assign5260_e6754_d_n6;
        locals.var_t0_dn7 = assign5260_e6754_d_n7;
        locals.var_t0_dn8 = assign5260_e6754_d_n8;
        locals.var_t0_dn9 = assign5260_e6754_d_n9;
        locals.var_t0_dn10 = assign5260_e6754_d_n10;
        locals.var_t0_dn11 = assign5260_e6754_d_n11;
        locals.var_t0_dn12 = assign5260_e6754_d_n12;

        let (assign5270_e6765, assign5270_e6765_d_n3, assign5270_e6765_d_n4, assign5270_e6765_d_n5, assign5270_e6765_d_n6, assign5270_e6765_d_n7, assign5270_e6765_d_n8, assign5270_e6765_d_n9, assign5270_e6765_d_n10, assign5270_e6765_d_n11, assign5270_e6765_d_n12,) = {
    if (locals.var_guard557 == 0.0) {
        let assign5270_e6759: f64 = (p.p53 + locals.var_t1);
        let assign5270_e6762: f64 = (p.p37 * locals.var_t0);
        let assign5270_e6763: f64 = (assign5270_e6759 - assign5270_e6762);
        (assign5270_e6763, (locals.var_t1_dn3 - (p.p37 * locals.var_t0_dn3)), (locals.var_t1_dn4 - (p.p37 * locals.var_t0_dn4)), (locals.var_t1_dn5 - (p.p37 * locals.var_t0_dn5)), (locals.var_t1_dn6 - (p.p37 * locals.var_t0_dn6)), (locals.var_t1_dn7 - (p.p37 * locals.var_t0_dn7)), (locals.var_t1_dn8 - (p.p37 * locals.var_t0_dn8)), (locals.var_t1_dn9 - (p.p37 * locals.var_t0_dn9)), (locals.var_t1_dn10 - (p.p37 * locals.var_t0_dn10)), (locals.var_t1_dn11 - (p.p37 * locals.var_t0_dn11)), (locals.var_t1_dn12 - (p.p37 * locals.var_t0_dn12)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign5270_e6765;
        locals.var_t2_dn3 = assign5270_e6765_d_n3;
        locals.var_t2_dn4 = assign5270_e6765_d_n4;
        locals.var_t2_dn5 = assign5270_e6765_d_n5;
        locals.var_t2_dn6 = assign5270_e6765_d_n6;
        locals.var_t2_dn7 = assign5270_e6765_d_n7;
        locals.var_t2_dn8 = assign5270_e6765_d_n8;
        locals.var_t2_dn9 = assign5270_e6765_d_n9;
        locals.var_t2_dn10 = assign5270_e6765_d_n10;
        locals.var_t2_dn11 = assign5270_e6765_d_n11;
        locals.var_t2_dn12 = assign5270_e6765_d_n12;

        let (assign5280_e6772, assign5280_e6772_d_n3, assign5280_e6772_d_n4, assign5280_e6772_d_n5, assign5280_e6772_d_n6, assign5280_e6772_d_n7, assign5280_e6772_d_n8, assign5280_e6772_d_n9, assign5280_e6772_d_n10, assign5280_e6772_d_n11, assign5280_e6772_d_n12,) = {
    if (locals.var_guard557 == 0.0) {
        let assign5280_e6770: f64 = (p.p52 - locals.var_t2);
        (assign5280_e6770, (-locals.var_t2_dn3), (-locals.var_t2_dn4), (-locals.var_t2_dn5), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn8), (-locals.var_t2_dn9), (-locals.var_t2_dn10), (-locals.var_t2_dn11), (-locals.var_t2_dn12),)
    } else {
        (locals.var_pparam_b4soivfbsd, locals.var_pparam_b4soivfbsd_dn3, locals.var_pparam_b4soivfbsd_dn4, locals.var_pparam_b4soivfbsd_dn5, locals.var_pparam_b4soivfbsd_dn6, locals.var_pparam_b4soivfbsd_dn7, locals.var_pparam_b4soivfbsd_dn8, locals.var_pparam_b4soivfbsd_dn9, locals.var_pparam_b4soivfbsd_dn10, locals.var_pparam_b4soivfbsd_dn11, locals.var_pparam_b4soivfbsd_dn12,)
    }
};
        locals.var_pparam_b4soivfbsd = assign5280_e6772;
        locals.var_pparam_b4soivfbsd_dn3 = assign5280_e6772_d_n3;
        locals.var_pparam_b4soivfbsd_dn4 = assign5280_e6772_d_n4;
        locals.var_pparam_b4soivfbsd_dn5 = assign5280_e6772_d_n5;
        locals.var_pparam_b4soivfbsd_dn6 = assign5280_e6772_d_n6;
        locals.var_pparam_b4soivfbsd_dn7 = assign5280_e6772_d_n7;
        locals.var_pparam_b4soivfbsd_dn8 = assign5280_e6772_d_n8;
        locals.var_pparam_b4soivfbsd_dn9 = assign5280_e6772_d_n9;
        locals.var_pparam_b4soivfbsd_dn10 = assign5280_e6772_d_n10;
        locals.var_pparam_b4soivfbsd_dn11 = assign5280_e6772_d_n11;
        locals.var_pparam_b4soivfbsd_dn12 = assign5280_e6772_d_n12;

        let assign5290_e6776: f64 = (p.p380 / p.p376);
        let (assign5290_e6785,) = {
    if (assign5290_e6776 > 1e-38) {
        let assign5290_e6781: f64 = (p.p380 / p.p376);
        let assign5290_e6782: f64 = (assign5290_e6781).ln();
        (assign5290_e6782,)
    } else {
        let assign5290_e6784: f64 = (-87.49823353377374);
        (assign5290_e6784,)
    }
};
        let assign5290_e6786: f64 = (p.p379 * assign5290_e6785);
        let assign5290_e6787: f64 = (assign5290_e6786).exp();
        let __rspice_inv_cse_0: f64 = 1.0 / p.p376;
        let assign5290_e6789: f64 = (assign5290_e6787 * __rspice_inv_cse_0);
        let assign5290_e6791: f64 = (assign5290_e6789 * __rspice_inv_cse_0);
        locals.var_pparam_b4soitoxratio = assign5290_e6791;

        let assign5300_e6796: f64 = (p.p376 * locals.var_pparam_b4soipoxedge);
        let assign5300_e6797: f64 = (p.p380 / assign5300_e6796);
        let (assign5300_e6808, assign5300_e6808_d_n3, assign5300_e6808_d_n4, assign5300_e6808_d_n5, assign5300_e6808_d_n6, assign5300_e6808_d_n7, assign5300_e6808_d_n8, assign5300_e6808_d_n9, assign5300_e6808_d_n10, assign5300_e6808_d_n11, assign5300_e6808_d_n12,) = {
    if (assign5300_e6797 > 1e-38) {
        let assign5300_e6803: f64 = (p.p376 * locals.var_pparam_b4soipoxedge);
        let assign5300_e6804: f64 = (p.p380 / assign5300_e6803);
        let assign5300_e6805: f64 = (assign5300_e6804).ln();
        (assign5300_e6805, ((-((p.p380 * (p.p376 * locals.var_pparam_b4soipoxedge_dn3)) / (assign5300_e6803 * assign5300_e6803))) / assign5300_e6804), ((-((p.p380 * (p.p376 * locals.var_pparam_b4soipoxedge_dn4)) / (assign5300_e6803 * assign5300_e6803))) / assign5300_e6804), ((-((p.p380 * (p.p376 * locals.var_pparam_b4soipoxedge_dn5)) / (assign5300_e6803 * assign5300_e6803))) / assign5300_e6804), ((-((p.p380 * (p.p376 * locals.var_pparam_b4soipoxedge_dn6)) / (assign5300_e6803 * assign5300_e6803))) / assign5300_e6804), ((-((p.p380 * (p.p376 * locals.var_pparam_b4soipoxedge_dn7)) / (assign5300_e6803 * assign5300_e6803))) / assign5300_e6804), ((-((p.p380 * (p.p376 * locals.var_pparam_b4soipoxedge_dn8)) / (assign5300_e6803 * assign5300_e6803))) / assign5300_e6804), ((-((p.p380 * (p.p376 * locals.var_pparam_b4soipoxedge_dn9)) / (assign5300_e6803 * assign5300_e6803))) / assign5300_e6804), ((-((p.p380 * (p.p376 * locals.var_pparam_b4soipoxedge_dn10)) / (assign5300_e6803 * assign5300_e6803))) / assign5300_e6804), ((-((p.p380 * (p.p376 * locals.var_pparam_b4soipoxedge_dn11)) / (assign5300_e6803 * assign5300_e6803))) / assign5300_e6804), ((-((p.p380 * (p.p376 * locals.var_pparam_b4soipoxedge_dn12)) / (assign5300_e6803 * assign5300_e6803))) / assign5300_e6804),)
    } else {
        let assign5300_e6807: f64 = (-87.49823353377374);
        (assign5300_e6807, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let assign5300_e6809: f64 = (p.p379 * assign5300_e6808);
        let assign5300_e6810: f64 = (assign5300_e6809).exp();
        let __rspice_inv_cse_1: f64 = 1.0 / p.p376;
        let assign5300_e6812: f64 = (assign5300_e6810 * __rspice_inv_cse_1);
        let assign5300_e6814: f64 = (assign5300_e6812 * __rspice_inv_cse_1);
        let __rspice_inv_cse_2: f64 = 1.0 / locals.var_pparam_b4soipoxedge;
        let assign5300_e6816: f64 = (assign5300_e6814 * __rspice_inv_cse_2);
        let assign5300_e6818: f64 = (assign5300_e6816 * __rspice_inv_cse_2);
        locals.var_pparam_b4soitoxratioedge = assign5300_e6818;
        locals.var_pparam_b4soitoxratioedge_dn3 = (((((((((assign5300_e6810 * (p.p379 * assign5300_e6808_d_n3)) / p.p376) / p.p376) * locals.var_pparam_b4soipoxedge) - (assign5300_e6814 * locals.var_pparam_b4soipoxedge_dn3)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge)) * locals.var_pparam_b4soipoxedge) - (assign5300_e6816 * locals.var_pparam_b4soipoxedge_dn3)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge));
        locals.var_pparam_b4soitoxratioedge_dn4 = (((((((((assign5300_e6810 * (p.p379 * assign5300_e6808_d_n4)) / p.p376) / p.p376) * locals.var_pparam_b4soipoxedge) - (assign5300_e6814 * locals.var_pparam_b4soipoxedge_dn4)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge)) * locals.var_pparam_b4soipoxedge) - (assign5300_e6816 * locals.var_pparam_b4soipoxedge_dn4)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge));
        locals.var_pparam_b4soitoxratioedge_dn5 = (((((((((assign5300_e6810 * (p.p379 * assign5300_e6808_d_n5)) / p.p376) / p.p376) * locals.var_pparam_b4soipoxedge) - (assign5300_e6814 * locals.var_pparam_b4soipoxedge_dn5)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge)) * locals.var_pparam_b4soipoxedge) - (assign5300_e6816 * locals.var_pparam_b4soipoxedge_dn5)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge));
        locals.var_pparam_b4soitoxratioedge_dn6 = (((((((((assign5300_e6810 * (p.p379 * assign5300_e6808_d_n6)) / p.p376) / p.p376) * locals.var_pparam_b4soipoxedge) - (assign5300_e6814 * locals.var_pparam_b4soipoxedge_dn6)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge)) * locals.var_pparam_b4soipoxedge) - (assign5300_e6816 * locals.var_pparam_b4soipoxedge_dn6)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge));
        locals.var_pparam_b4soitoxratioedge_dn7 = (((((((((assign5300_e6810 * (p.p379 * assign5300_e6808_d_n7)) / p.p376) / p.p376) * locals.var_pparam_b4soipoxedge) - (assign5300_e6814 * locals.var_pparam_b4soipoxedge_dn7)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge)) * locals.var_pparam_b4soipoxedge) - (assign5300_e6816 * locals.var_pparam_b4soipoxedge_dn7)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge));
        locals.var_pparam_b4soitoxratioedge_dn8 = (((((((((assign5300_e6810 * (p.p379 * assign5300_e6808_d_n8)) / p.p376) / p.p376) * locals.var_pparam_b4soipoxedge) - (assign5300_e6814 * locals.var_pparam_b4soipoxedge_dn8)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge)) * locals.var_pparam_b4soipoxedge) - (assign5300_e6816 * locals.var_pparam_b4soipoxedge_dn8)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge));
        locals.var_pparam_b4soitoxratioedge_dn9 = (((((((((assign5300_e6810 * (p.p379 * assign5300_e6808_d_n9)) / p.p376) / p.p376) * locals.var_pparam_b4soipoxedge) - (assign5300_e6814 * locals.var_pparam_b4soipoxedge_dn9)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge)) * locals.var_pparam_b4soipoxedge) - (assign5300_e6816 * locals.var_pparam_b4soipoxedge_dn9)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge));
        locals.var_pparam_b4soitoxratioedge_dn10 = (((((((((assign5300_e6810 * (p.p379 * assign5300_e6808_d_n10)) / p.p376) / p.p376) * locals.var_pparam_b4soipoxedge) - (assign5300_e6814 * locals.var_pparam_b4soipoxedge_dn10)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge)) * locals.var_pparam_b4soipoxedge) - (assign5300_e6816 * locals.var_pparam_b4soipoxedge_dn10)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge));
        locals.var_pparam_b4soitoxratioedge_dn11 = (((((((((assign5300_e6810 * (p.p379 * assign5300_e6808_d_n11)) / p.p376) / p.p376) * locals.var_pparam_b4soipoxedge) - (assign5300_e6814 * locals.var_pparam_b4soipoxedge_dn11)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge)) * locals.var_pparam_b4soipoxedge) - (assign5300_e6816 * locals.var_pparam_b4soipoxedge_dn11)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge));
        locals.var_pparam_b4soitoxratioedge_dn12 = (((((((((assign5300_e6810 * (p.p379 * assign5300_e6808_d_n12)) / p.p376) / p.p376) * locals.var_pparam_b4soipoxedge) - (assign5300_e6814 * locals.var_pparam_b4soipoxedge_dn12)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge)) * locals.var_pparam_b4soipoxedge) - (assign5300_e6816 * locals.var_pparam_b4soipoxedge_dn12)) / (locals.var_pparam_b4soipoxedge * locals.var_pparam_b4soipoxedge));

        let (assign5310_e6824,) = {
    if (p.p37 == 1.0) {
        (p.p1040,)
    } else {
        (p.p1039,)
    }
};
        locals.var_pparam_b4soiaechvb = assign5310_e6824;
        locals.var_pparam_b4soiaechvb_dn3 = 0.0;
        locals.var_pparam_b4soiaechvb_dn4 = 0.0;
        locals.var_pparam_b4soiaechvb_dn5 = 0.0;
        locals.var_pparam_b4soiaechvb_dn6 = 0.0;
        locals.var_pparam_b4soiaechvb_dn7 = 0.0;
        locals.var_pparam_b4soiaechvb_dn8 = 0.0;
        locals.var_pparam_b4soiaechvb_dn9 = 0.0;
        locals.var_pparam_b4soiaechvb_dn10 = 0.0;
        locals.var_pparam_b4soiaechvb_dn11 = 0.0;
        locals.var_pparam_b4soiaechvb_dn12 = 0.0;

        let (assign5320_e6830,) = {
    if (p.p37 == 1.0) {
        (p.p1042,)
    } else {
        (p.p1041,)
    }
};
        locals.var_pparam_b4soibechvb = assign5320_e6830;

        let assign5330_e6834: f64 = (locals.var_pparam_b4soiweff / p.p23);
        let assign5330_e6836: f64 = (assign5330_e6834 + p.p25);
        let assign5330_e6837: f64 = (locals.var_pparam_b4soiaechvb * assign5330_e6836);
        let assign5330_e6839: f64 = (assign5330_e6837 * locals.var_pparam_b4soidlcig);
        let assign5330_e6841: f64 = (assign5330_e6839 * locals.var_pparam_b4soitoxratioedge);
        locals.var_pparam_b4soiaechvbedges = assign5330_e6841;
        locals.var_pparam_b4soiaechvbedges_dn3 = ((((((locals.var_pparam_b4soiaechvb_dn3 * assign5330_e6836) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn3 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5330_e6837 * locals.var_pparam_b4soidlcig_dn3)) * locals.var_pparam_b4soitoxratioedge) + (assign5330_e6839 * locals.var_pparam_b4soitoxratioedge_dn3));
        locals.var_pparam_b4soiaechvbedges_dn4 = ((((((locals.var_pparam_b4soiaechvb_dn4 * assign5330_e6836) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn4 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5330_e6837 * locals.var_pparam_b4soidlcig_dn4)) * locals.var_pparam_b4soitoxratioedge) + (assign5330_e6839 * locals.var_pparam_b4soitoxratioedge_dn4));
        locals.var_pparam_b4soiaechvbedges_dn5 = ((((((locals.var_pparam_b4soiaechvb_dn5 * assign5330_e6836) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn5 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5330_e6837 * locals.var_pparam_b4soidlcig_dn5)) * locals.var_pparam_b4soitoxratioedge) + (assign5330_e6839 * locals.var_pparam_b4soitoxratioedge_dn5));
        locals.var_pparam_b4soiaechvbedges_dn6 = ((((((locals.var_pparam_b4soiaechvb_dn6 * assign5330_e6836) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn6 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5330_e6837 * locals.var_pparam_b4soidlcig_dn6)) * locals.var_pparam_b4soitoxratioedge) + (assign5330_e6839 * locals.var_pparam_b4soitoxratioedge_dn6));
        locals.var_pparam_b4soiaechvbedges_dn7 = ((((((locals.var_pparam_b4soiaechvb_dn7 * assign5330_e6836) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn7 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5330_e6837 * locals.var_pparam_b4soidlcig_dn7)) * locals.var_pparam_b4soitoxratioedge) + (assign5330_e6839 * locals.var_pparam_b4soitoxratioedge_dn7));
        locals.var_pparam_b4soiaechvbedges_dn8 = ((((((locals.var_pparam_b4soiaechvb_dn8 * assign5330_e6836) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn8 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5330_e6837 * locals.var_pparam_b4soidlcig_dn8)) * locals.var_pparam_b4soitoxratioedge) + (assign5330_e6839 * locals.var_pparam_b4soitoxratioedge_dn8));
        locals.var_pparam_b4soiaechvbedges_dn9 = ((((((locals.var_pparam_b4soiaechvb_dn9 * assign5330_e6836) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn9 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5330_e6837 * locals.var_pparam_b4soidlcig_dn9)) * locals.var_pparam_b4soitoxratioedge) + (assign5330_e6839 * locals.var_pparam_b4soitoxratioedge_dn9));
        locals.var_pparam_b4soiaechvbedges_dn10 = ((((((locals.var_pparam_b4soiaechvb_dn10 * assign5330_e6836) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn10 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5330_e6837 * locals.var_pparam_b4soidlcig_dn10)) * locals.var_pparam_b4soitoxratioedge) + (assign5330_e6839 * locals.var_pparam_b4soitoxratioedge_dn10));
        locals.var_pparam_b4soiaechvbedges_dn11 = ((((((locals.var_pparam_b4soiaechvb_dn11 * assign5330_e6836) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn11 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5330_e6837 * locals.var_pparam_b4soidlcig_dn11)) * locals.var_pparam_b4soitoxratioedge) + (assign5330_e6839 * locals.var_pparam_b4soitoxratioedge_dn11));
        locals.var_pparam_b4soiaechvbedges_dn12 = ((((((locals.var_pparam_b4soiaechvb_dn12 * assign5330_e6836) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn12 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5330_e6837 * locals.var_pparam_b4soidlcig_dn12)) * locals.var_pparam_b4soitoxratioedge) + (assign5330_e6839 * locals.var_pparam_b4soitoxratioedge_dn12));

        let assign5340_e6845: f64 = (locals.var_pparam_b4soiweff / p.p23);
        let assign5340_e6847: f64 = (assign5340_e6845 + p.p24);
        let assign5340_e6848: f64 = (locals.var_pparam_b4soiaechvb * assign5340_e6847);
        let assign5340_e6850: f64 = (assign5340_e6848 * locals.var_pparam_b4soidlcig);
        let assign5340_e6852: f64 = (assign5340_e6850 * locals.var_pparam_b4soitoxratioedge);
        locals.var_pparam_b4soiaechvbedged = assign5340_e6852;
        locals.var_pparam_b4soiaechvbedged_dn3 = ((((((locals.var_pparam_b4soiaechvb_dn3 * assign5340_e6847) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn3 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5340_e6848 * locals.var_pparam_b4soidlcig_dn3)) * locals.var_pparam_b4soitoxratioedge) + (assign5340_e6850 * locals.var_pparam_b4soitoxratioedge_dn3));
        locals.var_pparam_b4soiaechvbedged_dn4 = ((((((locals.var_pparam_b4soiaechvb_dn4 * assign5340_e6847) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn4 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5340_e6848 * locals.var_pparam_b4soidlcig_dn4)) * locals.var_pparam_b4soitoxratioedge) + (assign5340_e6850 * locals.var_pparam_b4soitoxratioedge_dn4));
        locals.var_pparam_b4soiaechvbedged_dn5 = ((((((locals.var_pparam_b4soiaechvb_dn5 * assign5340_e6847) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn5 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5340_e6848 * locals.var_pparam_b4soidlcig_dn5)) * locals.var_pparam_b4soitoxratioedge) + (assign5340_e6850 * locals.var_pparam_b4soitoxratioedge_dn5));
        locals.var_pparam_b4soiaechvbedged_dn6 = ((((((locals.var_pparam_b4soiaechvb_dn6 * assign5340_e6847) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn6 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5340_e6848 * locals.var_pparam_b4soidlcig_dn6)) * locals.var_pparam_b4soitoxratioedge) + (assign5340_e6850 * locals.var_pparam_b4soitoxratioedge_dn6));
        locals.var_pparam_b4soiaechvbedged_dn7 = ((((((locals.var_pparam_b4soiaechvb_dn7 * assign5340_e6847) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn7 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5340_e6848 * locals.var_pparam_b4soidlcig_dn7)) * locals.var_pparam_b4soitoxratioedge) + (assign5340_e6850 * locals.var_pparam_b4soitoxratioedge_dn7));
        locals.var_pparam_b4soiaechvbedged_dn8 = ((((((locals.var_pparam_b4soiaechvb_dn8 * assign5340_e6847) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn8 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5340_e6848 * locals.var_pparam_b4soidlcig_dn8)) * locals.var_pparam_b4soitoxratioedge) + (assign5340_e6850 * locals.var_pparam_b4soitoxratioedge_dn8));
        locals.var_pparam_b4soiaechvbedged_dn9 = ((((((locals.var_pparam_b4soiaechvb_dn9 * assign5340_e6847) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn9 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5340_e6848 * locals.var_pparam_b4soidlcig_dn9)) * locals.var_pparam_b4soitoxratioedge) + (assign5340_e6850 * locals.var_pparam_b4soitoxratioedge_dn9));
        locals.var_pparam_b4soiaechvbedged_dn10 = ((((((locals.var_pparam_b4soiaechvb_dn10 * assign5340_e6847) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn10 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5340_e6848 * locals.var_pparam_b4soidlcig_dn10)) * locals.var_pparam_b4soitoxratioedge) + (assign5340_e6850 * locals.var_pparam_b4soitoxratioedge_dn10));
        locals.var_pparam_b4soiaechvbedged_dn11 = ((((((locals.var_pparam_b4soiaechvb_dn11 * assign5340_e6847) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn11 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5340_e6848 * locals.var_pparam_b4soidlcig_dn11)) * locals.var_pparam_b4soitoxratioedge) + (assign5340_e6850 * locals.var_pparam_b4soitoxratioedge_dn11));
        locals.var_pparam_b4soiaechvbedged_dn12 = ((((((locals.var_pparam_b4soiaechvb_dn12 * assign5340_e6847) + (locals.var_pparam_b4soiaechvb * (locals.var_pparam_b4soiweff_dn12 / p.p23))) * locals.var_pparam_b4soidlcig) + (assign5340_e6848 * locals.var_pparam_b4soidlcig_dn12)) * locals.var_pparam_b4soitoxratioedge) + (assign5340_e6850 * locals.var_pparam_b4soitoxratioedge_dn12));

        let assign5350_e6854: f64 = (-locals.var_pparam_b4soibechvb);
        let assign5350_e6856: f64 = (assign5350_e6854 * p.p376);
        let assign5350_e6858: f64 = (assign5350_e6856 * locals.var_pparam_b4soipoxedge);
        locals.var_pparam_b4soibechvbedge = assign5350_e6858;
        locals.var_pparam_b4soibechvbedge_dn3 = (assign5350_e6856 * locals.var_pparam_b4soipoxedge_dn3);
        locals.var_pparam_b4soibechvbedge_dn4 = (assign5350_e6856 * locals.var_pparam_b4soipoxedge_dn4);
        locals.var_pparam_b4soibechvbedge_dn5 = (assign5350_e6856 * locals.var_pparam_b4soipoxedge_dn5);
        locals.var_pparam_b4soibechvbedge_dn6 = (assign5350_e6856 * locals.var_pparam_b4soipoxedge_dn6);
        locals.var_pparam_b4soibechvbedge_dn7 = (assign5350_e6856 * locals.var_pparam_b4soipoxedge_dn7);
        locals.var_pparam_b4soibechvbedge_dn8 = (assign5350_e6856 * locals.var_pparam_b4soipoxedge_dn8);
        locals.var_pparam_b4soibechvbedge_dn9 = (assign5350_e6856 * locals.var_pparam_b4soipoxedge_dn9);
        locals.var_pparam_b4soibechvbedge_dn10 = (assign5350_e6856 * locals.var_pparam_b4soipoxedge_dn10);
        locals.var_pparam_b4soibechvbedge_dn11 = (assign5350_e6856 * locals.var_pparam_b4soipoxedge_dn11);
        locals.var_pparam_b4soibechvbedge_dn12 = (assign5350_e6856 * locals.var_pparam_b4soipoxedge_dn12);

        let assign5360_e6861: f64 = (locals.var_pparam_b4soiaechvb * locals.var_pparam_b4soitoxratio);
        let assign5360_e6864: f64 = (locals.var_pparam_b4soiweff / p.p23);
        let assign5360_e6866: f64 = (assign5360_e6864 * locals.var_pparam_b4soileff);
        let assign5360_e6869: f64 = (p.p28 / p.p3);
        let assign5360_e6870: f64 = (assign5360_e6866 + assign5360_e6869);
        let assign5360_e6871: f64 = (assign5360_e6861 * assign5360_e6870);
        locals.var_pparam_b4soiaechvb = assign5360_e6871;
        locals.var_pparam_b4soiaechvb_dn3 = (((locals.var_pparam_b4soiaechvb_dn3 * locals.var_pparam_b4soitoxratio) * assign5360_e6870) + (assign5360_e6861 * (((locals.var_pparam_b4soiweff_dn3 / p.p23) * locals.var_pparam_b4soileff) + (assign5360_e6864 * locals.var_pparam_b4soileff_dn3))));
        locals.var_pparam_b4soiaechvb_dn4 = (((locals.var_pparam_b4soiaechvb_dn4 * locals.var_pparam_b4soitoxratio) * assign5360_e6870) + (assign5360_e6861 * (((locals.var_pparam_b4soiweff_dn4 / p.p23) * locals.var_pparam_b4soileff) + (assign5360_e6864 * locals.var_pparam_b4soileff_dn4))));
        locals.var_pparam_b4soiaechvb_dn5 = (((locals.var_pparam_b4soiaechvb_dn5 * locals.var_pparam_b4soitoxratio) * assign5360_e6870) + (assign5360_e6861 * (((locals.var_pparam_b4soiweff_dn5 / p.p23) * locals.var_pparam_b4soileff) + (assign5360_e6864 * locals.var_pparam_b4soileff_dn5))));
        locals.var_pparam_b4soiaechvb_dn6 = (((locals.var_pparam_b4soiaechvb_dn6 * locals.var_pparam_b4soitoxratio) * assign5360_e6870) + (assign5360_e6861 * (((locals.var_pparam_b4soiweff_dn6 / p.p23) * locals.var_pparam_b4soileff) + (assign5360_e6864 * locals.var_pparam_b4soileff_dn6))));
        locals.var_pparam_b4soiaechvb_dn7 = (((locals.var_pparam_b4soiaechvb_dn7 * locals.var_pparam_b4soitoxratio) * assign5360_e6870) + (assign5360_e6861 * (((locals.var_pparam_b4soiweff_dn7 / p.p23) * locals.var_pparam_b4soileff) + (assign5360_e6864 * locals.var_pparam_b4soileff_dn7))));
        locals.var_pparam_b4soiaechvb_dn8 = (((locals.var_pparam_b4soiaechvb_dn8 * locals.var_pparam_b4soitoxratio) * assign5360_e6870) + (assign5360_e6861 * (((locals.var_pparam_b4soiweff_dn8 / p.p23) * locals.var_pparam_b4soileff) + (assign5360_e6864 * locals.var_pparam_b4soileff_dn8))));
        locals.var_pparam_b4soiaechvb_dn9 = (((locals.var_pparam_b4soiaechvb_dn9 * locals.var_pparam_b4soitoxratio) * assign5360_e6870) + (assign5360_e6861 * (((locals.var_pparam_b4soiweff_dn9 / p.p23) * locals.var_pparam_b4soileff) + (assign5360_e6864 * locals.var_pparam_b4soileff_dn9))));
        locals.var_pparam_b4soiaechvb_dn10 = (((locals.var_pparam_b4soiaechvb_dn10 * locals.var_pparam_b4soitoxratio) * assign5360_e6870) + (assign5360_e6861 * (((locals.var_pparam_b4soiweff_dn10 / p.p23) * locals.var_pparam_b4soileff) + (assign5360_e6864 * locals.var_pparam_b4soileff_dn10))));
        locals.var_pparam_b4soiaechvb_dn11 = (((locals.var_pparam_b4soiaechvb_dn11 * locals.var_pparam_b4soitoxratio) * assign5360_e6870) + (assign5360_e6861 * (((locals.var_pparam_b4soiweff_dn11 / p.p23) * locals.var_pparam_b4soileff) + (assign5360_e6864 * locals.var_pparam_b4soileff_dn11))));
        locals.var_pparam_b4soiaechvb_dn12 = (((locals.var_pparam_b4soiaechvb_dn12 * locals.var_pparam_b4soitoxratio) * assign5360_e6870) + (assign5360_e6861 * (((locals.var_pparam_b4soiweff_dn12 / p.p23) * locals.var_pparam_b4soileff) + (assign5360_e6864 * locals.var_pparam_b4soileff_dn12))));

        let assign5370_e6874: f64 = (-p.p376);
        let assign5370_e6875: f64 = (locals.var_pparam_b4soibechvb * assign5370_e6874);
        locals.var_pparam_b4soibechvb = assign5370_e6875;

        let assign5380_e6880: f64 = if (param_given[90] || param_given[94]) { 1.0 } else { 0.0 };
        locals.var_guard560 = assign5380_e6880;

        let assign5390_e6883: f64 = if (!param_given[90]) { 1.0 } else { 0.0 };
        locals.var_guard561 = assign5390_e6883;

        let (assign5400_e6889, assign5400_e6889_d_n3, assign5400_e6889_d_n4, assign5400_e6889_d_n5, assign5400_e6889_d_n6, assign5400_e6889_d_n7, assign5400_e6889_d_n8, assign5400_e6889_d_n9, assign5400_e6889_d_n10, assign5400_e6889_d_n11, assign5400_e6889_d_n12,) = {
    if ((locals.var_guard560 != 0.0) && (locals.var_guard561 != 0.0)) {
        (0.53, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soik1, locals.var_pparam_b4soik1_dn3, locals.var_pparam_b4soik1_dn4, locals.var_pparam_b4soik1_dn5, locals.var_pparam_b4soik1_dn6, locals.var_pparam_b4soik1_dn7, locals.var_pparam_b4soik1_dn8, locals.var_pparam_b4soik1_dn9, locals.var_pparam_b4soik1_dn10, locals.var_pparam_b4soik1_dn11, locals.var_pparam_b4soik1_dn12,)
    }
};
        locals.var_pparam_b4soik1 = assign5400_e6889;
        locals.var_pparam_b4soik1_dn3 = assign5400_e6889_d_n3;
        locals.var_pparam_b4soik1_dn4 = assign5400_e6889_d_n4;
        locals.var_pparam_b4soik1_dn5 = assign5400_e6889_d_n5;
        locals.var_pparam_b4soik1_dn6 = assign5400_e6889_d_n6;
        locals.var_pparam_b4soik1_dn7 = assign5400_e6889_d_n7;
        locals.var_pparam_b4soik1_dn8 = assign5400_e6889_d_n8;
        locals.var_pparam_b4soik1_dn9 = assign5400_e6889_d_n9;
        locals.var_pparam_b4soik1_dn10 = assign5400_e6889_d_n10;
        locals.var_pparam_b4soik1_dn11 = assign5400_e6889_d_n11;
        locals.var_pparam_b4soik1_dn12 = assign5400_e6889_d_n12;

        let assign5410_e6892: f64 = if (!param_given[94]) { 1.0 } else { 0.0 };
        locals.var_guard562 = assign5410_e6892;

    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign5420_e6899, assign5420_e6899_d_n3, assign5420_e6899_d_n4, assign5420_e6899_d_n5, assign5420_e6899_d_n6, assign5420_e6899_d_n7, assign5420_e6899_d_n8, assign5420_e6899_d_n9, assign5420_e6899_d_n10, assign5420_e6899_d_n11, assign5420_e6899_d_n12,) = {
    if ((locals.var_guard560 != 0.0) && (locals.var_guard562 != 0.0)) {
        let assign5420_e6897: f64 = (-0.0186);
        (assign5420_e6897, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soik2, locals.var_pparam_b4soik2_dn3, locals.var_pparam_b4soik2_dn4, locals.var_pparam_b4soik2_dn5, locals.var_pparam_b4soik2_dn6, locals.var_pparam_b4soik2_dn7, locals.var_pparam_b4soik2_dn8, locals.var_pparam_b4soik2_dn9, locals.var_pparam_b4soik2_dn10, locals.var_pparam_b4soik2_dn11, locals.var_pparam_b4soik2_dn12,)
    }
};
        locals.var_pparam_b4soik2 = assign5420_e6899;
        locals.var_pparam_b4soik2_dn3 = assign5420_e6899_d_n3;
        locals.var_pparam_b4soik2_dn4 = assign5420_e6899_d_n4;
        locals.var_pparam_b4soik2_dn5 = assign5420_e6899_d_n5;
        locals.var_pparam_b4soik2_dn6 = assign5420_e6899_d_n6;
        locals.var_pparam_b4soik2_dn7 = assign5420_e6899_d_n7;
        locals.var_pparam_b4soik2_dn8 = assign5420_e6899_d_n8;
        locals.var_pparam_b4soik2_dn9 = assign5420_e6899_d_n9;
        locals.var_pparam_b4soik2_dn10 = assign5420_e6899_d_n10;
        locals.var_pparam_b4soik2_dn11 = assign5420_e6899_d_n11;
        locals.var_pparam_b4soik2_dn12 = assign5420_e6899_d_n12;

        let assign5480_e6912: f64 = if (!param_given[87]) { 1.0 } else { 0.0 };
        locals.var_guard568 = assign5480_e6912;

        let (assign5490_e6927, assign5490_e6927_d_n3, assign5490_e6927_d_n4, assign5490_e6927_d_n5, assign5490_e6927_d_n6, assign5490_e6927_d_n7, assign5490_e6927_d_n8, assign5490_e6927_d_n9, assign5490_e6927_d_n10, assign5490_e6927_d_n11, assign5490_e6927_d_n12,) = {
    if (((locals.var_guard560 == 0.0) && (locals.var_guard568 != 0.0)) && (p.p41 != 0.0)) {
        let assign5490_e6922: f64 = (2.0 * locals.var_epssub);
        let assign5490_e6923: f64 = (1.602176462e-19 / assign5490_e6922);
        let assign5490_e6925: f64 = (assign5490_e6923 * 1000000.0);
        (assign5490_e6925, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign5490_e6927;
        locals.var_t0_dn3 = assign5490_e6927_d_n3;
        locals.var_t0_dn4 = assign5490_e6927_d_n4;
        locals.var_t0_dn5 = assign5490_e6927_d_n5;
        locals.var_t0_dn6 = assign5490_e6927_d_n6;
        locals.var_t0_dn7 = assign5490_e6927_d_n7;
        locals.var_t0_dn8 = assign5490_e6927_d_n8;
        locals.var_t0_dn9 = assign5490_e6927_d_n9;
        locals.var_t0_dn10 = assign5490_e6927_d_n10;
        locals.var_t0_dn11 = assign5490_e6927_d_n11;
        locals.var_t0_dn12 = assign5490_e6927_d_n12;

        let (assign5500_e6937, assign5500_e6937_d_n3, assign5500_e6937_d_n4, assign5500_e6937_d_n5, assign5500_e6937_d_n6, assign5500_e6937_d_n7, assign5500_e6937_d_n8, assign5500_e6937_d_n9, assign5500_e6937_d_n10, assign5500_e6937_d_n11, assign5500_e6937_d_n12,) = {
    if (((locals.var_guard560 == 0.0) && (locals.var_guard568 != 0.0)) && (p.p41 == 0.0)) {
        (0.00077348, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign5500_e6937;
        locals.var_t0_dn3 = assign5500_e6937_d_n3;
        locals.var_t0_dn4 = assign5500_e6937_d_n4;
        locals.var_t0_dn5 = assign5500_e6937_d_n5;
        locals.var_t0_dn6 = assign5500_e6937_d_n6;
        locals.var_t0_dn7 = assign5500_e6937_d_n7;
        locals.var_t0_dn8 = assign5500_e6937_d_n8;
        locals.var_t0_dn9 = assign5500_e6937_d_n9;
        locals.var_t0_dn10 = assign5500_e6937_d_n10;
        locals.var_t0_dn11 = assign5500_e6937_d_n11;
        locals.var_t0_dn12 = assign5500_e6937_d_n12;

        let (assign5510_e6952, assign5510_e6952_d_n3, assign5510_e6952_d_n4, assign5510_e6952_d_n5, assign5510_e6952_d_n6, assign5510_e6952_d_n7, assign5510_e6952_d_n8, assign5510_e6952_d_n9, assign5510_e6952_d_n10, assign5510_e6952_d_n11, assign5510_e6952_d_n12,) = {
    if ((locals.var_guard560 == 0.0) && (locals.var_guard568 != 0.0)) {
        let assign5510_e6945: f64 = (locals.var_t0 * locals.var_pparam_b4soinpeak);
        let assign5510_e6947: f64 = (assign5510_e6945 * locals.var_pparam_b4soixt);
        let assign5510_e6949: f64 = (assign5510_e6947 * locals.var_pparam_b4soixt);
        let assign5510_e6950: f64 = (locals.var_pparam_b4soiphi - assign5510_e6949);
        (assign5510_e6950, (locals.var_pparam_b4soiphi_dn3 - ((((locals.var_t0_dn3 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn3)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn4 - ((((locals.var_t0_dn4 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn4)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn5 - ((((locals.var_t0_dn5 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn5)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn6 - ((((locals.var_t0_dn6 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn6)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn7 - ((((locals.var_t0_dn7 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn7)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn8 - ((((locals.var_t0_dn8 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn8)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn9 - ((((locals.var_t0_dn9 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn9)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn10 - ((((locals.var_t0_dn10 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn10)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn11 - ((((locals.var_t0_dn11 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn11)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn12 - ((((locals.var_t0_dn12 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn12)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)),)
    } else {
        (locals.var_pparam_b4soivbx, locals.var_pparam_b4soivbx_dn3, locals.var_pparam_b4soivbx_dn4, locals.var_pparam_b4soivbx_dn5, locals.var_pparam_b4soivbx_dn6, locals.var_pparam_b4soivbx_dn7, locals.var_pparam_b4soivbx_dn8, locals.var_pparam_b4soivbx_dn9, locals.var_pparam_b4soivbx_dn10, locals.var_pparam_b4soivbx_dn11, locals.var_pparam_b4soivbx_dn12,)
    }
};
        locals.var_pparam_b4soivbx = assign5510_e6952;
        locals.var_pparam_b4soivbx_dn3 = assign5510_e6952_d_n3;
        locals.var_pparam_b4soivbx_dn4 = assign5510_e6952_d_n4;
        locals.var_pparam_b4soivbx_dn5 = assign5510_e6952_d_n5;
        locals.var_pparam_b4soivbx_dn6 = assign5510_e6952_d_n6;
        locals.var_pparam_b4soivbx_dn7 = assign5510_e6952_d_n7;
        locals.var_pparam_b4soivbx_dn8 = assign5510_e6952_d_n8;
        locals.var_pparam_b4soivbx_dn9 = assign5510_e6952_d_n9;
        locals.var_pparam_b4soivbx_dn10 = assign5510_e6952_d_n10;
        locals.var_pparam_b4soivbx_dn11 = assign5510_e6952_d_n11;
        locals.var_pparam_b4soivbx_dn12 = assign5510_e6952_d_n12;

        let assign5520_e6955: f64 = if locals.var_pparam_b4soivbx > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard569 = assign5520_e6955;

        let (assign5530_e6963, assign5530_e6963_d_n3, assign5530_e6963_d_n4, assign5530_e6963_d_n5, assign5530_e6963_d_n6, assign5530_e6963_d_n7, assign5530_e6963_d_n8, assign5530_e6963_d_n9, assign5530_e6963_d_n10, assign5530_e6963_d_n11, assign5530_e6963_d_n12,) = {
    if ((locals.var_guard560 == 0.0) && (locals.var_guard569 != 0.0)) {
        let assign5530_e6961: f64 = (-locals.var_pparam_b4soivbx);
        (assign5530_e6961, (-locals.var_pparam_b4soivbx_dn3), (-locals.var_pparam_b4soivbx_dn4), (-locals.var_pparam_b4soivbx_dn5), (-locals.var_pparam_b4soivbx_dn6), (-locals.var_pparam_b4soivbx_dn7), (-locals.var_pparam_b4soivbx_dn8), (-locals.var_pparam_b4soivbx_dn9), (-locals.var_pparam_b4soivbx_dn10), (-locals.var_pparam_b4soivbx_dn11), (-locals.var_pparam_b4soivbx_dn12),)
    } else {
        (locals.var_pparam_b4soivbx, locals.var_pparam_b4soivbx_dn3, locals.var_pparam_b4soivbx_dn4, locals.var_pparam_b4soivbx_dn5, locals.var_pparam_b4soivbx_dn6, locals.var_pparam_b4soivbx_dn7, locals.var_pparam_b4soivbx_dn8, locals.var_pparam_b4soivbx_dn9, locals.var_pparam_b4soivbx_dn10, locals.var_pparam_b4soivbx_dn11, locals.var_pparam_b4soivbx_dn12,)
    }
};
        locals.var_pparam_b4soivbx = assign5530_e6963;
        locals.var_pparam_b4soivbx_dn3 = assign5530_e6963_d_n3;
        locals.var_pparam_b4soivbx_dn4 = assign5530_e6963_d_n4;
        locals.var_pparam_b4soivbx_dn5 = assign5530_e6963_d_n5;
        locals.var_pparam_b4soivbx_dn6 = assign5530_e6963_d_n6;
        locals.var_pparam_b4soivbx_dn7 = assign5530_e6963_d_n7;
        locals.var_pparam_b4soivbx_dn8 = assign5530_e6963_d_n8;
        locals.var_pparam_b4soivbx_dn9 = assign5530_e6963_d_n9;
        locals.var_pparam_b4soivbx_dn10 = assign5530_e6963_d_n10;
        locals.var_pparam_b4soivbx_dn11 = assign5530_e6963_d_n11;
        locals.var_pparam_b4soivbx_dn12 = assign5530_e6963_d_n12;

        let assign5540_e6966: f64 = if locals.var_pparam_b4soivbm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard570 = assign5540_e6966;

        let (assign5550_e6974,) = {
    if ((locals.var_guard560 == 0.0) && (locals.var_guard570 != 0.0)) {
        let assign5550_e6972: f64 = (-locals.var_pparam_b4soivbm);
        (assign5550_e6972,)
    } else {
        (locals.var_pparam_b4soivbm,)
    }
};
        locals.var_pparam_b4soivbm = assign5550_e6974;

        let assign5560_e6977: f64 = if (!param_given[85]) { 1.0 } else { 0.0 };
        locals.var_guard571 = assign5560_e6977;

        let (assign5570_e6989, assign5570_e6989_d_n3, assign5570_e6989_d_n4, assign5570_e6989_d_n5, assign5570_e6989_d_n6, assign5570_e6989_d_n7, assign5570_e6989_d_n8, assign5570_e6989_d_n9, assign5570_e6989_d_n10, assign5570_e6989_d_n11, assign5570_e6989_d_n12,) = {
    if ((locals.var_guard560 == 0.0) && (locals.var_guard571 != 0.0)) {
        let assign5570_e6984: f64 = (locals.var_pparam_b4soinpeak).sqrt();
        let assign5570_e6985: f64 = (locals.var_sqrt2qeps * assign5570_e6984);
        let assign5570_e6987: f64 = (assign5570_e6985 / locals.var_b4soicox);
        (assign5570_e6987, ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn3 / (2.0 * assign5570_e6984))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn4 / (2.0 * assign5570_e6984))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn5 / (2.0 * assign5570_e6984))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn6 / (2.0 * assign5570_e6984))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn7 / (2.0 * assign5570_e6984))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn8 / (2.0 * assign5570_e6984))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn9 / (2.0 * assign5570_e6984))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn10 / (2.0 * assign5570_e6984))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn11 / (2.0 * assign5570_e6984))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn12 / (2.0 * assign5570_e6984))) / locals.var_b4soicox),)
    } else {
        (locals.var_pparam_b4soigamma1, locals.var_pparam_b4soigamma1_dn3, locals.var_pparam_b4soigamma1_dn4, locals.var_pparam_b4soigamma1_dn5, locals.var_pparam_b4soigamma1_dn6, locals.var_pparam_b4soigamma1_dn7, locals.var_pparam_b4soigamma1_dn8, locals.var_pparam_b4soigamma1_dn9, locals.var_pparam_b4soigamma1_dn10, locals.var_pparam_b4soigamma1_dn11, locals.var_pparam_b4soigamma1_dn12,)
    }
};
        locals.var_pparam_b4soigamma1 = assign5570_e6989;
        locals.var_pparam_b4soigamma1_dn3 = assign5570_e6989_d_n3;
        locals.var_pparam_b4soigamma1_dn4 = assign5570_e6989_d_n4;
        locals.var_pparam_b4soigamma1_dn5 = assign5570_e6989_d_n5;
        locals.var_pparam_b4soigamma1_dn6 = assign5570_e6989_d_n6;
        locals.var_pparam_b4soigamma1_dn7 = assign5570_e6989_d_n7;
        locals.var_pparam_b4soigamma1_dn8 = assign5570_e6989_d_n8;
        locals.var_pparam_b4soigamma1_dn9 = assign5570_e6989_d_n9;
        locals.var_pparam_b4soigamma1_dn10 = assign5570_e6989_d_n10;
        locals.var_pparam_b4soigamma1_dn11 = assign5570_e6989_d_n11;
        locals.var_pparam_b4soigamma1_dn12 = assign5570_e6989_d_n12;

        let assign5580_e6992: f64 = if (!param_given[86]) { 1.0 } else { 0.0 };
        locals.var_guard572 = assign5580_e6992;

        let (assign5590_e7004, assign5590_e7004_d_n3, assign5590_e7004_d_n4, assign5590_e7004_d_n5, assign5590_e7004_d_n6, assign5590_e7004_d_n7, assign5590_e7004_d_n8, assign5590_e7004_d_n9, assign5590_e7004_d_n10, assign5590_e7004_d_n11, assign5590_e7004_d_n12,) = {
    if ((locals.var_guard560 == 0.0) && (locals.var_guard572 != 0.0)) {
        let assign5590_e6999: f64 = (locals.var_pparam_b4soinsub).sqrt();
        let assign5590_e7000: f64 = (locals.var_sqrt2qeps * assign5590_e6999);
        let assign5590_e7002: f64 = (assign5590_e7000 / locals.var_b4soicox);
        (assign5590_e7002, ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn3 / (2.0 * assign5590_e6999))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn4 / (2.0 * assign5590_e6999))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn5 / (2.0 * assign5590_e6999))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn6 / (2.0 * assign5590_e6999))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn7 / (2.0 * assign5590_e6999))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn8 / (2.0 * assign5590_e6999))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn9 / (2.0 * assign5590_e6999))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn10 / (2.0 * assign5590_e6999))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn11 / (2.0 * assign5590_e6999))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn12 / (2.0 * assign5590_e6999))) / locals.var_b4soicox),)
    } else {
        (locals.var_pparam_b4soigamma2, locals.var_pparam_b4soigamma2_dn3, locals.var_pparam_b4soigamma2_dn4, locals.var_pparam_b4soigamma2_dn5, locals.var_pparam_b4soigamma2_dn6, locals.var_pparam_b4soigamma2_dn7, locals.var_pparam_b4soigamma2_dn8, locals.var_pparam_b4soigamma2_dn9, locals.var_pparam_b4soigamma2_dn10, locals.var_pparam_b4soigamma2_dn11, locals.var_pparam_b4soigamma2_dn12,)
    }
};
        locals.var_pparam_b4soigamma2 = assign5590_e7004;
        locals.var_pparam_b4soigamma2_dn3 = assign5590_e7004_d_n3;
        locals.var_pparam_b4soigamma2_dn4 = assign5590_e7004_d_n4;
        locals.var_pparam_b4soigamma2_dn5 = assign5590_e7004_d_n5;
        locals.var_pparam_b4soigamma2_dn6 = assign5590_e7004_d_n6;
        locals.var_pparam_b4soigamma2_dn7 = assign5590_e7004_d_n7;
        locals.var_pparam_b4soigamma2_dn8 = assign5590_e7004_d_n8;
        locals.var_pparam_b4soigamma2_dn9 = assign5590_e7004_d_n9;
        locals.var_pparam_b4soigamma2_dn10 = assign5590_e7004_d_n10;
        locals.var_pparam_b4soigamma2_dn11 = assign5590_e7004_d_n11;
        locals.var_pparam_b4soigamma2_dn12 = assign5590_e7004_d_n12;

        let (assign5600_e7011, assign5600_e7011_d_n3, assign5600_e7011_d_n4, assign5600_e7011_d_n5, assign5600_e7011_d_n6, assign5600_e7011_d_n7, assign5600_e7011_d_n8, assign5600_e7011_d_n9, assign5600_e7011_d_n10, assign5600_e7011_d_n11, assign5600_e7011_d_n12,) = {
    if (locals.var_guard560 == 0.0) {
        let assign5600_e7009: f64 = (locals.var_pparam_b4soigamma1 - locals.var_pparam_b4soigamma2);
        (assign5600_e7009, (locals.var_pparam_b4soigamma1_dn3 - locals.var_pparam_b4soigamma2_dn3), (locals.var_pparam_b4soigamma1_dn4 - locals.var_pparam_b4soigamma2_dn4), (locals.var_pparam_b4soigamma1_dn5 - locals.var_pparam_b4soigamma2_dn5), (locals.var_pparam_b4soigamma1_dn6 - locals.var_pparam_b4soigamma2_dn6), (locals.var_pparam_b4soigamma1_dn7 - locals.var_pparam_b4soigamma2_dn7), (locals.var_pparam_b4soigamma1_dn8 - locals.var_pparam_b4soigamma2_dn8), (locals.var_pparam_b4soigamma1_dn9 - locals.var_pparam_b4soigamma2_dn9), (locals.var_pparam_b4soigamma1_dn10 - locals.var_pparam_b4soigamma2_dn10), (locals.var_pparam_b4soigamma1_dn11 - locals.var_pparam_b4soigamma2_dn11), (locals.var_pparam_b4soigamma1_dn12 - locals.var_pparam_b4soigamma2_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign5600_e7011;
        locals.var_t0_dn3 = assign5600_e7011_d_n3;
        locals.var_t0_dn4 = assign5600_e7011_d_n4;
        locals.var_t0_dn5 = assign5600_e7011_d_n5;
        locals.var_t0_dn6 = assign5600_e7011_d_n6;
        locals.var_t0_dn7 = assign5600_e7011_d_n7;
        locals.var_t0_dn8 = assign5600_e7011_d_n8;
        locals.var_t0_dn9 = assign5600_e7011_d_n9;
        locals.var_t0_dn10 = assign5600_e7011_d_n10;
        locals.var_t0_dn11 = assign5600_e7011_d_n11;
        locals.var_t0_dn12 = assign5600_e7011_d_n12;

        let (assign5610_e7021, assign5610_e7021_d_n3, assign5610_e7021_d_n4, assign5610_e7021_d_n5, assign5610_e7021_d_n6, assign5610_e7021_d_n7, assign5610_e7021_d_n8, assign5610_e7021_d_n9, assign5610_e7021_d_n10, assign5610_e7021_d_n11, assign5610_e7021_d_n12,) = {
    if (locals.var_guard560 == 0.0) {
        let assign5610_e7016: f64 = (locals.var_pparam_b4soiphi - locals.var_pparam_b4soivbx);
        let assign5610_e7017: f64 = (assign5610_e7016).sqrt();
        let assign5610_e7019: f64 = (assign5610_e7017 - locals.var_pparam_b4soisqrtphi);
        (assign5610_e7019, (((locals.var_pparam_b4soiphi_dn3 - locals.var_pparam_b4soivbx_dn3) / (2.0 * assign5610_e7017)) - locals.var_pparam_b4soisqrtphi_dn3), (((locals.var_pparam_b4soiphi_dn4 - locals.var_pparam_b4soivbx_dn4) / (2.0 * assign5610_e7017)) - locals.var_pparam_b4soisqrtphi_dn4), (((locals.var_pparam_b4soiphi_dn5 - locals.var_pparam_b4soivbx_dn5) / (2.0 * assign5610_e7017)) - locals.var_pparam_b4soisqrtphi_dn5), (((locals.var_pparam_b4soiphi_dn6 - locals.var_pparam_b4soivbx_dn6) / (2.0 * assign5610_e7017)) - locals.var_pparam_b4soisqrtphi_dn6), (((locals.var_pparam_b4soiphi_dn7 - locals.var_pparam_b4soivbx_dn7) / (2.0 * assign5610_e7017)) - locals.var_pparam_b4soisqrtphi_dn7), (((locals.var_pparam_b4soiphi_dn8 - locals.var_pparam_b4soivbx_dn8) / (2.0 * assign5610_e7017)) - locals.var_pparam_b4soisqrtphi_dn8), (((locals.var_pparam_b4soiphi_dn9 - locals.var_pparam_b4soivbx_dn9) / (2.0 * assign5610_e7017)) - locals.var_pparam_b4soisqrtphi_dn9), (((locals.var_pparam_b4soiphi_dn10 - locals.var_pparam_b4soivbx_dn10) / (2.0 * assign5610_e7017)) - locals.var_pparam_b4soisqrtphi_dn10), (((locals.var_pparam_b4soiphi_dn11 - locals.var_pparam_b4soivbx_dn11) / (2.0 * assign5610_e7017)) - locals.var_pparam_b4soisqrtphi_dn11), (((locals.var_pparam_b4soiphi_dn12 - locals.var_pparam_b4soivbx_dn12) / (2.0 * assign5610_e7017)) - locals.var_pparam_b4soisqrtphi_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign5610_e7021;
        locals.var_t1_dn3 = assign5610_e7021_d_n3;
        locals.var_t1_dn4 = assign5610_e7021_d_n4;
        locals.var_t1_dn5 = assign5610_e7021_d_n5;
        locals.var_t1_dn6 = assign5610_e7021_d_n6;
        locals.var_t1_dn7 = assign5610_e7021_d_n7;
        locals.var_t1_dn8 = assign5610_e7021_d_n8;
        locals.var_t1_dn9 = assign5610_e7021_d_n9;
        locals.var_t1_dn10 = assign5610_e7021_d_n10;
        locals.var_t1_dn11 = assign5610_e7021_d_n11;
        locals.var_t1_dn12 = assign5610_e7021_d_n12;

        let (assign5620_e7033, assign5620_e7033_d_n3, assign5620_e7033_d_n4, assign5620_e7033_d_n5, assign5620_e7033_d_n6, assign5620_e7033_d_n7, assign5620_e7033_d_n8, assign5620_e7033_d_n9, assign5620_e7033_d_n10, assign5620_e7033_d_n11, assign5620_e7033_d_n12,) = {
    if (locals.var_guard560 == 0.0) {
        let assign5620_e7027: f64 = (locals.var_pparam_b4soiphi - locals.var_pparam_b4soivbm);
        let assign5620_e7028: f64 = (assign5620_e7027).sqrt();
        let assign5620_e7030: f64 = (assign5620_e7028 - locals.var_pparam_b4soisqrtphi);
        let assign5620_e7031: f64 = (locals.var_pparam_b4soisqrtphi * assign5620_e7030);
        (assign5620_e7031, ((locals.var_pparam_b4soisqrtphi_dn3 * assign5620_e7030) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn3 / (2.0 * assign5620_e7028)) - locals.var_pparam_b4soisqrtphi_dn3))), ((locals.var_pparam_b4soisqrtphi_dn4 * assign5620_e7030) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn4 / (2.0 * assign5620_e7028)) - locals.var_pparam_b4soisqrtphi_dn4))), ((locals.var_pparam_b4soisqrtphi_dn5 * assign5620_e7030) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn5 / (2.0 * assign5620_e7028)) - locals.var_pparam_b4soisqrtphi_dn5))), ((locals.var_pparam_b4soisqrtphi_dn6 * assign5620_e7030) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn6 / (2.0 * assign5620_e7028)) - locals.var_pparam_b4soisqrtphi_dn6))), ((locals.var_pparam_b4soisqrtphi_dn7 * assign5620_e7030) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn7 / (2.0 * assign5620_e7028)) - locals.var_pparam_b4soisqrtphi_dn7))), ((locals.var_pparam_b4soisqrtphi_dn8 * assign5620_e7030) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn8 / (2.0 * assign5620_e7028)) - locals.var_pparam_b4soisqrtphi_dn8))), ((locals.var_pparam_b4soisqrtphi_dn9 * assign5620_e7030) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn9 / (2.0 * assign5620_e7028)) - locals.var_pparam_b4soisqrtphi_dn9))), ((locals.var_pparam_b4soisqrtphi_dn10 * assign5620_e7030) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn10 / (2.0 * assign5620_e7028)) - locals.var_pparam_b4soisqrtphi_dn10))), ((locals.var_pparam_b4soisqrtphi_dn11 * assign5620_e7030) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn11 / (2.0 * assign5620_e7028)) - locals.var_pparam_b4soisqrtphi_dn11))), ((locals.var_pparam_b4soisqrtphi_dn12 * assign5620_e7030) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn12 / (2.0 * assign5620_e7028)) - locals.var_pparam_b4soisqrtphi_dn12))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign5620_e7033;
        locals.var_t2_dn3 = assign5620_e7033_d_n3;
        locals.var_t2_dn4 = assign5620_e7033_d_n4;
        locals.var_t2_dn5 = assign5620_e7033_d_n5;
        locals.var_t2_dn6 = assign5620_e7033_d_n6;
        locals.var_t2_dn7 = assign5620_e7033_d_n7;
        locals.var_t2_dn8 = assign5620_e7033_d_n8;
        locals.var_t2_dn9 = assign5620_e7033_d_n9;
        locals.var_t2_dn10 = assign5620_e7033_d_n10;
        locals.var_t2_dn11 = assign5620_e7033_d_n11;
        locals.var_t2_dn12 = assign5620_e7033_d_n12;

        let (assign5630_e7046, assign5630_e7046_d_n3, assign5630_e7046_d_n4, assign5630_e7046_d_n5, assign5630_e7046_d_n6, assign5630_e7046_d_n7, assign5630_e7046_d_n8, assign5630_e7046_d_n9, assign5630_e7046_d_n10, assign5630_e7046_d_n11, assign5630_e7046_d_n12,) = {
    if (locals.var_guard560 == 0.0) {
        let assign5630_e7038: f64 = (locals.var_t0 * locals.var_t1);
        let assign5630_e7041: f64 = (2.0 * locals.var_t2);
        let assign5630_e7043: f64 = (assign5630_e7041 + locals.var_pparam_b4soivbm);
        let assign5630_e7044: f64 = (assign5630_e7038 / assign5630_e7043);
        (assign5630_e7044, (((((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)) * assign5630_e7043) - (assign5630_e7038 * (2.0 * locals.var_t2_dn3))) / (assign5630_e7043 * assign5630_e7043)), (((((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)) * assign5630_e7043) - (assign5630_e7038 * (2.0 * locals.var_t2_dn4))) / (assign5630_e7043 * assign5630_e7043)), (((((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)) * assign5630_e7043) - (assign5630_e7038 * (2.0 * locals.var_t2_dn5))) / (assign5630_e7043 * assign5630_e7043)), (((((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)) * assign5630_e7043) - (assign5630_e7038 * (2.0 * locals.var_t2_dn6))) / (assign5630_e7043 * assign5630_e7043)), (((((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)) * assign5630_e7043) - (assign5630_e7038 * (2.0 * locals.var_t2_dn7))) / (assign5630_e7043 * assign5630_e7043)), (((((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)) * assign5630_e7043) - (assign5630_e7038 * (2.0 * locals.var_t2_dn8))) / (assign5630_e7043 * assign5630_e7043)), (((((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)) * assign5630_e7043) - (assign5630_e7038 * (2.0 * locals.var_t2_dn9))) / (assign5630_e7043 * assign5630_e7043)), (((((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)) * assign5630_e7043) - (assign5630_e7038 * (2.0 * locals.var_t2_dn10))) / (assign5630_e7043 * assign5630_e7043)), (((((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)) * assign5630_e7043) - (assign5630_e7038 * (2.0 * locals.var_t2_dn11))) / (assign5630_e7043 * assign5630_e7043)), (((((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12)) * assign5630_e7043) - (assign5630_e7038 * (2.0 * locals.var_t2_dn12))) / (assign5630_e7043 * assign5630_e7043)),)
    } else {
        (locals.var_pparam_b4soik2, locals.var_pparam_b4soik2_dn3, locals.var_pparam_b4soik2_dn4, locals.var_pparam_b4soik2_dn5, locals.var_pparam_b4soik2_dn6, locals.var_pparam_b4soik2_dn7, locals.var_pparam_b4soik2_dn8, locals.var_pparam_b4soik2_dn9, locals.var_pparam_b4soik2_dn10, locals.var_pparam_b4soik2_dn11, locals.var_pparam_b4soik2_dn12,)
    }
};
        locals.var_pparam_b4soik2 = assign5630_e7046;
        locals.var_pparam_b4soik2_dn3 = assign5630_e7046_d_n3;
        locals.var_pparam_b4soik2_dn4 = assign5630_e7046_d_n4;
        locals.var_pparam_b4soik2_dn5 = assign5630_e7046_d_n5;
        locals.var_pparam_b4soik2_dn6 = assign5630_e7046_d_n6;
        locals.var_pparam_b4soik2_dn7 = assign5630_e7046_d_n7;
        locals.var_pparam_b4soik2_dn8 = assign5630_e7046_d_n8;
        locals.var_pparam_b4soik2_dn9 = assign5630_e7046_d_n9;
        locals.var_pparam_b4soik2_dn10 = assign5630_e7046_d_n10;
        locals.var_pparam_b4soik2_dn11 = assign5630_e7046_d_n11;
        locals.var_pparam_b4soik2_dn12 = assign5630_e7046_d_n12;

        let (assign5640_e7060, assign5640_e7060_d_n3, assign5640_e7060_d_n4, assign5640_e7060_d_n5, assign5640_e7060_d_n6, assign5640_e7060_d_n7, assign5640_e7060_d_n8, assign5640_e7060_d_n9, assign5640_e7060_d_n10, assign5640_e7060_d_n11, assign5640_e7060_d_n12,) = {
    if (locals.var_guard560 == 0.0) {
        let assign5640_e7052: f64 = (2.0 * locals.var_pparam_b4soik2);
        let assign5640_e7055: f64 = (locals.var_pparam_b4soiphi - locals.var_pparam_b4soivbm);
        let assign5640_e7056: f64 = (assign5640_e7055).sqrt();
        let assign5640_e7057: f64 = (assign5640_e7052 * assign5640_e7056);
        let assign5640_e7058: f64 = (locals.var_pparam_b4soigamma2 - assign5640_e7057);
        (assign5640_e7058, (locals.var_pparam_b4soigamma2_dn3 - (((2.0 * locals.var_pparam_b4soik2_dn3) * assign5640_e7056) + (assign5640_e7052 * (locals.var_pparam_b4soiphi_dn3 / (2.0 * assign5640_e7056))))), (locals.var_pparam_b4soigamma2_dn4 - (((2.0 * locals.var_pparam_b4soik2_dn4) * assign5640_e7056) + (assign5640_e7052 * (locals.var_pparam_b4soiphi_dn4 / (2.0 * assign5640_e7056))))), (locals.var_pparam_b4soigamma2_dn5 - (((2.0 * locals.var_pparam_b4soik2_dn5) * assign5640_e7056) + (assign5640_e7052 * (locals.var_pparam_b4soiphi_dn5 / (2.0 * assign5640_e7056))))), (locals.var_pparam_b4soigamma2_dn6 - (((2.0 * locals.var_pparam_b4soik2_dn6) * assign5640_e7056) + (assign5640_e7052 * (locals.var_pparam_b4soiphi_dn6 / (2.0 * assign5640_e7056))))), (locals.var_pparam_b4soigamma2_dn7 - (((2.0 * locals.var_pparam_b4soik2_dn7) * assign5640_e7056) + (assign5640_e7052 * (locals.var_pparam_b4soiphi_dn7 / (2.0 * assign5640_e7056))))), (locals.var_pparam_b4soigamma2_dn8 - (((2.0 * locals.var_pparam_b4soik2_dn8) * assign5640_e7056) + (assign5640_e7052 * (locals.var_pparam_b4soiphi_dn8 / (2.0 * assign5640_e7056))))), (locals.var_pparam_b4soigamma2_dn9 - (((2.0 * locals.var_pparam_b4soik2_dn9) * assign5640_e7056) + (assign5640_e7052 * (locals.var_pparam_b4soiphi_dn9 / (2.0 * assign5640_e7056))))), (locals.var_pparam_b4soigamma2_dn10 - (((2.0 * locals.var_pparam_b4soik2_dn10) * assign5640_e7056) + (assign5640_e7052 * (locals.var_pparam_b4soiphi_dn10 / (2.0 * assign5640_e7056))))), (locals.var_pparam_b4soigamma2_dn11 - (((2.0 * locals.var_pparam_b4soik2_dn11) * assign5640_e7056) + (assign5640_e7052 * (locals.var_pparam_b4soiphi_dn11 / (2.0 * assign5640_e7056))))), (locals.var_pparam_b4soigamma2_dn12 - (((2.0 * locals.var_pparam_b4soik2_dn12) * assign5640_e7056) + (assign5640_e7052 * (locals.var_pparam_b4soiphi_dn12 / (2.0 * assign5640_e7056))))),)
    } else {
        (locals.var_pparam_b4soik1, locals.var_pparam_b4soik1_dn3, locals.var_pparam_b4soik1_dn4, locals.var_pparam_b4soik1_dn5, locals.var_pparam_b4soik1_dn6, locals.var_pparam_b4soik1_dn7, locals.var_pparam_b4soik1_dn8, locals.var_pparam_b4soik1_dn9, locals.var_pparam_b4soik1_dn10, locals.var_pparam_b4soik1_dn11, locals.var_pparam_b4soik1_dn12,)
    }
};
        locals.var_pparam_b4soik1 = assign5640_e7060;
        locals.var_pparam_b4soik1_dn3 = assign5640_e7060_d_n3;
        locals.var_pparam_b4soik1_dn4 = assign5640_e7060_d_n4;
        locals.var_pparam_b4soik1_dn5 = assign5640_e7060_d_n5;
        locals.var_pparam_b4soik1_dn6 = assign5640_e7060_d_n6;
        locals.var_pparam_b4soik1_dn7 = assign5640_e7060_d_n7;
        locals.var_pparam_b4soik1_dn8 = assign5640_e7060_d_n8;
        locals.var_pparam_b4soik1_dn9 = assign5640_e7060_d_n9;
        locals.var_pparam_b4soik1_dn10 = assign5640_e7060_d_n10;
        locals.var_pparam_b4soik1_dn11 = assign5640_e7060_d_n11;
        locals.var_pparam_b4soik1_dn12 = assign5640_e7060_d_n12;

        let assign5650_e7063: f64 = (locals.var_pparam_b4soiweff + locals.var_pparam_b4soik1w2);
        locals.var_t0 = assign5650_e7063;
        locals.var_t0_dn3 = (locals.var_pparam_b4soiweff_dn3 + locals.var_pparam_b4soik1w2_dn3);
        locals.var_t0_dn4 = (locals.var_pparam_b4soiweff_dn4 + locals.var_pparam_b4soik1w2_dn4);
        locals.var_t0_dn5 = (locals.var_pparam_b4soiweff_dn5 + locals.var_pparam_b4soik1w2_dn5);
        locals.var_t0_dn6 = (locals.var_pparam_b4soiweff_dn6 + locals.var_pparam_b4soik1w2_dn6);
        locals.var_t0_dn7 = (locals.var_pparam_b4soiweff_dn7 + locals.var_pparam_b4soik1w2_dn7);
        locals.var_t0_dn8 = (locals.var_pparam_b4soiweff_dn8 + locals.var_pparam_b4soik1w2_dn8);
        locals.var_t0_dn9 = (locals.var_pparam_b4soiweff_dn9 + locals.var_pparam_b4soik1w2_dn9);
        locals.var_t0_dn10 = (locals.var_pparam_b4soiweff_dn10 + locals.var_pparam_b4soik1w2_dn10);
        locals.var_t0_dn11 = (locals.var_pparam_b4soiweff_dn11 + locals.var_pparam_b4soik1w2_dn11);
        locals.var_t0_dn12 = (locals.var_pparam_b4soiweff_dn12 + locals.var_pparam_b4soik1w2_dn12);

        let assign5660_e7066: f64 = if locals.var_t0 < 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard573 = assign5660_e7066;

        let (assign5670_e7070, assign5670_e7070_d_n3, assign5670_e7070_d_n4, assign5670_e7070_d_n5, assign5670_e7070_d_n6, assign5670_e7070_d_n7, assign5670_e7070_d_n8, assign5670_e7070_d_n9, assign5670_e7070_d_n10, assign5670_e7070_d_n11, assign5670_e7070_d_n12,) = {
    if (locals.var_guard573 != 0.0) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign5670_e7070;
        locals.var_t0_dn3 = assign5670_e7070_d_n3;
        locals.var_t0_dn4 = assign5670_e7070_d_n4;
        locals.var_t0_dn5 = assign5670_e7070_d_n5;
        locals.var_t0_dn6 = assign5670_e7070_d_n6;
        locals.var_t0_dn7 = assign5670_e7070_d_n7;
        locals.var_t0_dn8 = assign5670_e7070_d_n8;
        locals.var_t0_dn9 = assign5670_e7070_d_n9;
        locals.var_t0_dn10 = assign5670_e7070_d_n10;
        locals.var_t0_dn11 = assign5670_e7070_d_n11;
        locals.var_t0_dn12 = assign5670_e7070_d_n12;

        let assign5680_e7075: f64 = (locals.var_pparam_b4soik1w1 / locals.var_t0);
        let assign5680_e7076: f64 = (1.0 + assign5680_e7075);
        let assign5680_e7077: f64 = (locals.var_pparam_b4soik1 * assign5680_e7076);
        locals.var_pparam_b4soik1eff = assign5680_e7077;
        locals.var_pparam_b4soik1eff_dn3 = ((locals.var_pparam_b4soik1_dn3 * assign5680_e7076) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn3 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn4 = ((locals.var_pparam_b4soik1_dn4 * assign5680_e7076) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn4 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn5 = ((locals.var_pparam_b4soik1_dn5 * assign5680_e7076) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn5 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn6 = ((locals.var_pparam_b4soik1_dn6 * assign5680_e7076) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn6 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn7 = ((locals.var_pparam_b4soik1_dn7 * assign5680_e7076) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn7 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn8 = ((locals.var_pparam_b4soik1_dn8 * assign5680_e7076) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn8 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn9 = ((locals.var_pparam_b4soik1_dn9 * assign5680_e7076) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn9 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn10 = ((locals.var_pparam_b4soik1_dn10 * assign5680_e7076) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn10 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn11 = ((locals.var_pparam_b4soik1_dn11 * assign5680_e7076) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn11 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn12 = ((locals.var_pparam_b4soik1_dn12 * assign5680_e7076) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn12 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0))));

        let assign5690_e7080: f64 = if (!param_given[109]) { 1.0 } else { 0.0 };
        locals.var_guard574 = assign5690_e7080;

        let assign5700_e7085: f64 = if (param_given[108] || param_given[107]) { 1.0 } else { 0.0 };
        locals.var_guard575 = assign5700_e7085;

        let (assign5710_e7099, assign5710_e7099_d_n3, assign5710_e7099_d_n4, assign5710_e7099_d_n5, assign5710_e7099_d_n6, assign5710_e7099_d_n7, assign5710_e7099_d_n8, assign5710_e7099_d_n9, assign5710_e7099_d_n10, assign5710_e7099_d_n11, assign5710_e7099_d_n12,) = {
    if ((locals.var_guard574 != 0.0) && (locals.var_guard575 != 0.0)) {
        let assign5710_e7091: f64 = (p.p37 * locals.var_pparam_b4soivth0);
        let assign5710_e7093: f64 = (assign5710_e7091 - locals.var_pparam_b4soiphi);
        let assign5710_e7096: f64 = (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi);
        let assign5710_e7097: f64 = (assign5710_e7093 - assign5710_e7096);
        (assign5710_e7097, (((p.p37 * locals.var_pparam_b4soivth0_dn3) - locals.var_pparam_b4soiphi_dn3) - ((locals.var_pparam_b4soik1eff_dn3 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn3))), (((p.p37 * locals.var_pparam_b4soivth0_dn4) - locals.var_pparam_b4soiphi_dn4) - ((locals.var_pparam_b4soik1eff_dn4 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn4))), (((p.p37 * locals.var_pparam_b4soivth0_dn5) - locals.var_pparam_b4soiphi_dn5) - ((locals.var_pparam_b4soik1eff_dn5 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn5))), (((p.p37 * locals.var_pparam_b4soivth0_dn6) - locals.var_pparam_b4soiphi_dn6) - ((locals.var_pparam_b4soik1eff_dn6 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn6))), (((p.p37 * locals.var_pparam_b4soivth0_dn7) - locals.var_pparam_b4soiphi_dn7) - ((locals.var_pparam_b4soik1eff_dn7 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn7))), (((p.p37 * locals.var_pparam_b4soivth0_dn8) - locals.var_pparam_b4soiphi_dn8) - ((locals.var_pparam_b4soik1eff_dn8 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn8))), (((p.p37 * locals.var_pparam_b4soivth0_dn9) - locals.var_pparam_b4soiphi_dn9) - ((locals.var_pparam_b4soik1eff_dn9 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn9))), (((p.p37 * locals.var_pparam_b4soivth0_dn10) - locals.var_pparam_b4soiphi_dn10) - ((locals.var_pparam_b4soik1eff_dn10 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn10))), (((p.p37 * locals.var_pparam_b4soivth0_dn11) - locals.var_pparam_b4soiphi_dn11) - ((locals.var_pparam_b4soik1eff_dn11 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn11))), (((p.p37 * locals.var_pparam_b4soivth0_dn12) - locals.var_pparam_b4soiphi_dn12) - ((locals.var_pparam_b4soik1eff_dn12 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn12))),)
    } else {
        (locals.var_pparam_b4soivfb, locals.var_pparam_b4soivfb_dn3, locals.var_pparam_b4soivfb_dn4, locals.var_pparam_b4soivfb_dn5, locals.var_pparam_b4soivfb_dn6, locals.var_pparam_b4soivfb_dn7, locals.var_pparam_b4soivfb_dn8, locals.var_pparam_b4soivfb_dn9, locals.var_pparam_b4soivfb_dn10, locals.var_pparam_b4soivfb_dn11, locals.var_pparam_b4soivfb_dn12,)
    }
};
        locals.var_pparam_b4soivfb = assign5710_e7099;
        locals.var_pparam_b4soivfb_dn3 = assign5710_e7099_d_n3;
        locals.var_pparam_b4soivfb_dn4 = assign5710_e7099_d_n4;
        locals.var_pparam_b4soivfb_dn5 = assign5710_e7099_d_n5;
        locals.var_pparam_b4soivfb_dn6 = assign5710_e7099_d_n6;
        locals.var_pparam_b4soivfb_dn7 = assign5710_e7099_d_n7;
        locals.var_pparam_b4soivfb_dn8 = assign5710_e7099_d_n8;
        locals.var_pparam_b4soivfb_dn9 = assign5710_e7099_d_n9;
        locals.var_pparam_b4soivfb_dn10 = assign5710_e7099_d_n10;
        locals.var_pparam_b4soivfb_dn11 = assign5710_e7099_d_n11;
        locals.var_pparam_b4soivfb_dn12 = assign5710_e7099_d_n12;

        let (assign5720_e7107, assign5720_e7107_d_n3, assign5720_e7107_d_n4, assign5720_e7107_d_n5, assign5720_e7107_d_n6, assign5720_e7107_d_n7, assign5720_e7107_d_n8, assign5720_e7107_d_n9, assign5720_e7107_d_n10, assign5720_e7107_d_n11, assign5720_e7107_d_n12,) = {
    if ((locals.var_guard574 != 0.0) && (locals.var_guard575 == 0.0)) {
        let assign5720_e7105: f64 = (-1.0);
        (assign5720_e7105, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soivfb, locals.var_pparam_b4soivfb_dn3, locals.var_pparam_b4soivfb_dn4, locals.var_pparam_b4soivfb_dn5, locals.var_pparam_b4soivfb_dn6, locals.var_pparam_b4soivfb_dn7, locals.var_pparam_b4soivfb_dn8, locals.var_pparam_b4soivfb_dn9, locals.var_pparam_b4soivfb_dn10, locals.var_pparam_b4soivfb_dn11, locals.var_pparam_b4soivfb_dn12,)
    }
};
        locals.var_pparam_b4soivfb = assign5720_e7107;
        locals.var_pparam_b4soivfb_dn3 = assign5720_e7107_d_n3;
        locals.var_pparam_b4soivfb_dn4 = assign5720_e7107_d_n4;
        locals.var_pparam_b4soivfb_dn5 = assign5720_e7107_d_n5;
        locals.var_pparam_b4soivfb_dn6 = assign5720_e7107_d_n6;
        locals.var_pparam_b4soivfb_dn7 = assign5720_e7107_d_n7;
        locals.var_pparam_b4soivfb_dn8 = assign5720_e7107_d_n8;
        locals.var_pparam_b4soivfb_dn9 = assign5720_e7107_d_n9;
        locals.var_pparam_b4soivfb_dn10 = assign5720_e7107_d_n10;
        locals.var_pparam_b4soivfb_dn11 = assign5720_e7107_d_n11;
        locals.var_pparam_b4soivfb_dn12 = assign5720_e7107_d_n12;

        let assign5730_e7110: f64 = if (!param_given[108]) { 1.0 } else { 0.0 };
        locals.var_guard576 = assign5730_e7110;

        let (assign5740_e7122, assign5740_e7122_d_n3, assign5740_e7122_d_n4, assign5740_e7122_d_n5, assign5740_e7122_d_n6, assign5740_e7122_d_n7, assign5740_e7122_d_n8, assign5740_e7122_d_n9, assign5740_e7122_d_n10, assign5740_e7122_d_n11, assign5740_e7122_d_n12,) = {
    if (locals.var_guard576 != 0.0) {
        let assign5740_e7115: f64 = (locals.var_pparam_b4soivfb + locals.var_pparam_b4soiphi);
        let assign5740_e7118: f64 = (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi);
        let assign5740_e7119: f64 = (assign5740_e7115 + assign5740_e7118);
        let assign5740_e7120: f64 = (p.p37 * assign5740_e7119);
        (assign5740_e7120, (p.p37 * ((locals.var_pparam_b4soivfb_dn3 + locals.var_pparam_b4soiphi_dn3) + ((locals.var_pparam_b4soik1eff_dn3 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn3)))), (p.p37 * ((locals.var_pparam_b4soivfb_dn4 + locals.var_pparam_b4soiphi_dn4) + ((locals.var_pparam_b4soik1eff_dn4 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn4)))), (p.p37 * ((locals.var_pparam_b4soivfb_dn5 + locals.var_pparam_b4soiphi_dn5) + ((locals.var_pparam_b4soik1eff_dn5 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn5)))), (p.p37 * ((locals.var_pparam_b4soivfb_dn6 + locals.var_pparam_b4soiphi_dn6) + ((locals.var_pparam_b4soik1eff_dn6 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn6)))), (p.p37 * ((locals.var_pparam_b4soivfb_dn7 + locals.var_pparam_b4soiphi_dn7) + ((locals.var_pparam_b4soik1eff_dn7 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn7)))), (p.p37 * ((locals.var_pparam_b4soivfb_dn8 + locals.var_pparam_b4soiphi_dn8) + ((locals.var_pparam_b4soik1eff_dn8 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn8)))), (p.p37 * ((locals.var_pparam_b4soivfb_dn9 + locals.var_pparam_b4soiphi_dn9) + ((locals.var_pparam_b4soik1eff_dn9 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn9)))), (p.p37 * ((locals.var_pparam_b4soivfb_dn10 + locals.var_pparam_b4soiphi_dn10) + ((locals.var_pparam_b4soik1eff_dn10 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn10)))), (p.p37 * ((locals.var_pparam_b4soivfb_dn11 + locals.var_pparam_b4soiphi_dn11) + ((locals.var_pparam_b4soik1eff_dn11 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn11)))), (p.p37 * ((locals.var_pparam_b4soivfb_dn12 + locals.var_pparam_b4soiphi_dn12) + ((locals.var_pparam_b4soik1eff_dn12 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn12)))),)
    } else {
        (locals.var_pparam_b4soivth0, locals.var_pparam_b4soivth0_dn3, locals.var_pparam_b4soivth0_dn4, locals.var_pparam_b4soivth0_dn5, locals.var_pparam_b4soivth0_dn6, locals.var_pparam_b4soivth0_dn7, locals.var_pparam_b4soivth0_dn8, locals.var_pparam_b4soivth0_dn9, locals.var_pparam_b4soivth0_dn10, locals.var_pparam_b4soivth0_dn11, locals.var_pparam_b4soivth0_dn12,)
    }
};
        locals.var_pparam_b4soivth0 = assign5740_e7122;
        locals.var_pparam_b4soivth0_dn3 = assign5740_e7122_d_n3;
        locals.var_pparam_b4soivth0_dn4 = assign5740_e7122_d_n4;
        locals.var_pparam_b4soivth0_dn5 = assign5740_e7122_d_n5;
        locals.var_pparam_b4soivth0_dn6 = assign5740_e7122_d_n6;
        locals.var_pparam_b4soivth0_dn7 = assign5740_e7122_d_n7;
        locals.var_pparam_b4soivth0_dn8 = assign5740_e7122_d_n8;
        locals.var_pparam_b4soivth0_dn9 = assign5740_e7122_d_n9;
        locals.var_pparam_b4soivth0_dn10 = assign5740_e7122_d_n10;
        locals.var_pparam_b4soivth0_dn11 = assign5740_e7122_d_n11;
        locals.var_pparam_b4soivth0_dn12 = assign5740_e7122_d_n12;

        let assign5750_e7125: f64 = (locals.var_pparam_b4soik1eff * p.p66);
        let assign5750_e7127: f64 = (assign5750_e7125 / p.p67);
        locals.var_here_b4soik1ox = assign5750_e7127;
        locals.var_here_b4soik1ox_dn3 = ((locals.var_pparam_b4soik1eff_dn3 * p.p66) / p.p67);
        locals.var_here_b4soik1ox_dn4 = ((locals.var_pparam_b4soik1eff_dn4 * p.p66) / p.p67);
        locals.var_here_b4soik1ox_dn5 = ((locals.var_pparam_b4soik1eff_dn5 * p.p66) / p.p67);
        locals.var_here_b4soik1ox_dn6 = ((locals.var_pparam_b4soik1eff_dn6 * p.p66) / p.p67);
        locals.var_here_b4soik1ox_dn7 = ((locals.var_pparam_b4soik1eff_dn7 * p.p66) / p.p67);
        locals.var_here_b4soik1ox_dn8 = ((locals.var_pparam_b4soik1eff_dn8 * p.p66) / p.p67);
        locals.var_here_b4soik1ox_dn9 = ((locals.var_pparam_b4soik1eff_dn9 * p.p66) / p.p67);
        locals.var_here_b4soik1ox_dn10 = ((locals.var_pparam_b4soik1eff_dn10 * p.p66) / p.p67);
        locals.var_here_b4soik1ox_dn11 = ((locals.var_pparam_b4soik1eff_dn11 * p.p66) / p.p67);
        locals.var_here_b4soik1ox_dn12 = ((locals.var_pparam_b4soik1eff_dn12 * p.p66) / p.p67);

        let assign5760_e7130: f64 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0);
        locals.var_t1 = assign5760_e7130;
        locals.var_t1_dn3 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn3);
        locals.var_t1_dn4 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn4);
        locals.var_t1_dn5 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn5);
        locals.var_t1_dn6 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn6);
        locals.var_t1_dn7 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn7);
        locals.var_t1_dn8 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn8);
        locals.var_t1_dn9 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn9);
        locals.var_t1_dn10 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn10);
        locals.var_t1_dn11 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn11);
        locals.var_t1_dn12 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn12);

        let assign5770_e7132: f64 = (-0.5);
        let assign5770_e7134: f64 = (assign5770_e7132 * locals.var_pparam_b4soidsub);
        let assign5770_e7136: f64 = (assign5770_e7134 * locals.var_pparam_b4soileff);
        let assign5770_e7138: f64 = (assign5770_e7136 / locals.var_t1);
        let assign5770_e7139: f64 = (assign5770_e7138).exp();
        locals.var_t0 = assign5770_e7139;
        locals.var_t0_dn3 = (assign5770_e7139 * ((((((assign5770_e7132 * locals.var_pparam_b4soidsub_dn3) * locals.var_pparam_b4soileff) + (assign5770_e7134 * locals.var_pparam_b4soileff_dn3)) * locals.var_t1) - (assign5770_e7136 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn4 = (assign5770_e7139 * ((((((assign5770_e7132 * locals.var_pparam_b4soidsub_dn4) * locals.var_pparam_b4soileff) + (assign5770_e7134 * locals.var_pparam_b4soileff_dn4)) * locals.var_t1) - (assign5770_e7136 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn5 = (assign5770_e7139 * ((((((assign5770_e7132 * locals.var_pparam_b4soidsub_dn5) * locals.var_pparam_b4soileff) + (assign5770_e7134 * locals.var_pparam_b4soileff_dn5)) * locals.var_t1) - (assign5770_e7136 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn6 = (assign5770_e7139 * ((((((assign5770_e7132 * locals.var_pparam_b4soidsub_dn6) * locals.var_pparam_b4soileff) + (assign5770_e7134 * locals.var_pparam_b4soileff_dn6)) * locals.var_t1) - (assign5770_e7136 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn7 = (assign5770_e7139 * ((((((assign5770_e7132 * locals.var_pparam_b4soidsub_dn7) * locals.var_pparam_b4soileff) + (assign5770_e7134 * locals.var_pparam_b4soileff_dn7)) * locals.var_t1) - (assign5770_e7136 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn8 = (assign5770_e7139 * ((((((assign5770_e7132 * locals.var_pparam_b4soidsub_dn8) * locals.var_pparam_b4soileff) + (assign5770_e7134 * locals.var_pparam_b4soileff_dn8)) * locals.var_t1) - (assign5770_e7136 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn9 = (assign5770_e7139 * ((((((assign5770_e7132 * locals.var_pparam_b4soidsub_dn9) * locals.var_pparam_b4soileff) + (assign5770_e7134 * locals.var_pparam_b4soileff_dn9)) * locals.var_t1) - (assign5770_e7136 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn10 = (assign5770_e7139 * ((((((assign5770_e7132 * locals.var_pparam_b4soidsub_dn10) * locals.var_pparam_b4soileff) + (assign5770_e7134 * locals.var_pparam_b4soileff_dn10)) * locals.var_t1) - (assign5770_e7136 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn11 = (assign5770_e7139 * ((((((assign5770_e7132 * locals.var_pparam_b4soidsub_dn11) * locals.var_pparam_b4soileff) + (assign5770_e7134 * locals.var_pparam_b4soileff_dn11)) * locals.var_t1) - (assign5770_e7136 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn12 = (assign5770_e7139 * ((((((assign5770_e7132 * locals.var_pparam_b4soidsub_dn12) * locals.var_pparam_b4soileff) + (assign5770_e7134 * locals.var_pparam_b4soileff_dn12)) * locals.var_t1) - (assign5770_e7136 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)));

        let assign5780_e7143: f64 = (2.0 * locals.var_t0);
        let assign5780_e7145: f64 = (assign5780_e7143 * locals.var_t0);
        let assign5780_e7146: f64 = (locals.var_t0 + assign5780_e7145);
        locals.var_pparam_b4soitheta0vb0 = assign5780_e7146;
        locals.var_pparam_b4soitheta0vb0_dn3 = (locals.var_t0_dn3 + (((2.0 * locals.var_t0_dn3) * locals.var_t0) + (assign5780_e7143 * locals.var_t0_dn3)));
        locals.var_pparam_b4soitheta0vb0_dn4 = (locals.var_t0_dn4 + (((2.0 * locals.var_t0_dn4) * locals.var_t0) + (assign5780_e7143 * locals.var_t0_dn4)));
        locals.var_pparam_b4soitheta0vb0_dn5 = (locals.var_t0_dn5 + (((2.0 * locals.var_t0_dn5) * locals.var_t0) + (assign5780_e7143 * locals.var_t0_dn5)));
        locals.var_pparam_b4soitheta0vb0_dn6 = (locals.var_t0_dn6 + (((2.0 * locals.var_t0_dn6) * locals.var_t0) + (assign5780_e7143 * locals.var_t0_dn6)));
        locals.var_pparam_b4soitheta0vb0_dn7 = (locals.var_t0_dn7 + (((2.0 * locals.var_t0_dn7) * locals.var_t0) + (assign5780_e7143 * locals.var_t0_dn7)));
        locals.var_pparam_b4soitheta0vb0_dn8 = (locals.var_t0_dn8 + (((2.0 * locals.var_t0_dn8) * locals.var_t0) + (assign5780_e7143 * locals.var_t0_dn8)));
        locals.var_pparam_b4soitheta0vb0_dn9 = (locals.var_t0_dn9 + (((2.0 * locals.var_t0_dn9) * locals.var_t0) + (assign5780_e7143 * locals.var_t0_dn9)));
        locals.var_pparam_b4soitheta0vb0_dn10 = (locals.var_t0_dn10 + (((2.0 * locals.var_t0_dn10) * locals.var_t0) + (assign5780_e7143 * locals.var_t0_dn10)));
        locals.var_pparam_b4soitheta0vb0_dn11 = (locals.var_t0_dn11 + (((2.0 * locals.var_t0_dn11) * locals.var_t0) + (assign5780_e7143 * locals.var_t0_dn11)));
        locals.var_pparam_b4soitheta0vb0_dn12 = (locals.var_t0_dn12 + (((2.0 * locals.var_t0_dn12) * locals.var_t0) + (assign5780_e7143 * locals.var_t0_dn12)));

        let assign5790_e7148: f64 = (-0.5);
        let assign5790_e7150: f64 = (assign5790_e7148 * locals.var_pparam_b4soidrout);
        let assign5790_e7152: f64 = (assign5790_e7150 * locals.var_pparam_b4soileff);
        let assign5790_e7154: f64 = (assign5790_e7152 / locals.var_t1);
        let assign5790_e7155: f64 = (assign5790_e7154).exp();
        locals.var_t0 = assign5790_e7155;
        locals.var_t0_dn3 = (assign5790_e7155 * ((((((assign5790_e7148 * locals.var_pparam_b4soidrout_dn3) * locals.var_pparam_b4soileff) + (assign5790_e7150 * locals.var_pparam_b4soileff_dn3)) * locals.var_t1) - (assign5790_e7152 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn4 = (assign5790_e7155 * ((((((assign5790_e7148 * locals.var_pparam_b4soidrout_dn4) * locals.var_pparam_b4soileff) + (assign5790_e7150 * locals.var_pparam_b4soileff_dn4)) * locals.var_t1) - (assign5790_e7152 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn5 = (assign5790_e7155 * ((((((assign5790_e7148 * locals.var_pparam_b4soidrout_dn5) * locals.var_pparam_b4soileff) + (assign5790_e7150 * locals.var_pparam_b4soileff_dn5)) * locals.var_t1) - (assign5790_e7152 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn6 = (assign5790_e7155 * ((((((assign5790_e7148 * locals.var_pparam_b4soidrout_dn6) * locals.var_pparam_b4soileff) + (assign5790_e7150 * locals.var_pparam_b4soileff_dn6)) * locals.var_t1) - (assign5790_e7152 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn7 = (assign5790_e7155 * ((((((assign5790_e7148 * locals.var_pparam_b4soidrout_dn7) * locals.var_pparam_b4soileff) + (assign5790_e7150 * locals.var_pparam_b4soileff_dn7)) * locals.var_t1) - (assign5790_e7152 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn8 = (assign5790_e7155 * ((((((assign5790_e7148 * locals.var_pparam_b4soidrout_dn8) * locals.var_pparam_b4soileff) + (assign5790_e7150 * locals.var_pparam_b4soileff_dn8)) * locals.var_t1) - (assign5790_e7152 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn9 = (assign5790_e7155 * ((((((assign5790_e7148 * locals.var_pparam_b4soidrout_dn9) * locals.var_pparam_b4soileff) + (assign5790_e7150 * locals.var_pparam_b4soileff_dn9)) * locals.var_t1) - (assign5790_e7152 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn10 = (assign5790_e7155 * ((((((assign5790_e7148 * locals.var_pparam_b4soidrout_dn10) * locals.var_pparam_b4soileff) + (assign5790_e7150 * locals.var_pparam_b4soileff_dn10)) * locals.var_t1) - (assign5790_e7152 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn11 = (assign5790_e7155 * ((((((assign5790_e7148 * locals.var_pparam_b4soidrout_dn11) * locals.var_pparam_b4soileff) + (assign5790_e7150 * locals.var_pparam_b4soileff_dn11)) * locals.var_t1) - (assign5790_e7152 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn12 = (assign5790_e7155 * ((((((assign5790_e7148 * locals.var_pparam_b4soidrout_dn12) * locals.var_pparam_b4soileff) + (assign5790_e7150 * locals.var_pparam_b4soileff_dn12)) * locals.var_t1) - (assign5790_e7152 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)));

        let assign5800_e7159: f64 = (2.0 * locals.var_t0);
        let assign5800_e7161: f64 = (assign5800_e7159 * locals.var_t0);
        let assign5800_e7162: f64 = (locals.var_t0 + assign5800_e7161);
        locals.var_t2 = assign5800_e7162;
        locals.var_t2_dn3 = (locals.var_t0_dn3 + (((2.0 * locals.var_t0_dn3) * locals.var_t0) + (assign5800_e7159 * locals.var_t0_dn3)));
        locals.var_t2_dn4 = (locals.var_t0_dn4 + (((2.0 * locals.var_t0_dn4) * locals.var_t0) + (assign5800_e7159 * locals.var_t0_dn4)));
        locals.var_t2_dn5 = (locals.var_t0_dn5 + (((2.0 * locals.var_t0_dn5) * locals.var_t0) + (assign5800_e7159 * locals.var_t0_dn5)));
        locals.var_t2_dn6 = (locals.var_t0_dn6 + (((2.0 * locals.var_t0_dn6) * locals.var_t0) + (assign5800_e7159 * locals.var_t0_dn6)));
        locals.var_t2_dn7 = (locals.var_t0_dn7 + (((2.0 * locals.var_t0_dn7) * locals.var_t0) + (assign5800_e7159 * locals.var_t0_dn7)));
        locals.var_t2_dn8 = (locals.var_t0_dn8 + (((2.0 * locals.var_t0_dn8) * locals.var_t0) + (assign5800_e7159 * locals.var_t0_dn8)));
        locals.var_t2_dn9 = (locals.var_t0_dn9 + (((2.0 * locals.var_t0_dn9) * locals.var_t0) + (assign5800_e7159 * locals.var_t0_dn9)));
        locals.var_t2_dn10 = (locals.var_t0_dn10 + (((2.0 * locals.var_t0_dn10) * locals.var_t0) + (assign5800_e7159 * locals.var_t0_dn10)));
        locals.var_t2_dn11 = (locals.var_t0_dn11 + (((2.0 * locals.var_t0_dn11) * locals.var_t0) + (assign5800_e7159 * locals.var_t0_dn11)));
        locals.var_t2_dn12 = (locals.var_t0_dn12 + (((2.0 * locals.var_t0_dn12) * locals.var_t0) + (assign5800_e7159 * locals.var_t0_dn12)));

    }
}
