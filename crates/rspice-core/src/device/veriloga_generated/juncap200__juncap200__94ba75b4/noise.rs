#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 1] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_A_K_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 2, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "A", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "K", is_internal: false }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1])];
            let v0 = 0e0f64;
            let v1 = 1.0447941624768001e-10f64;
            let v2 = parameters[62];
            let v3 = 5e-1f64;
            let v5 = 1e0f64;
            let v6 = 2.7315e2f64;
            let v7 = parameters[13];
            let v9 = 1.6021918e-19f64;
            let v10 = 8.61726105451295e-5f64;
            let v13 = 7.02e-4f64;
            let v17 = 1.108e3f64;
            let v20 = parameters[24];
            let v22 = parameters[25];
            let v24 = parameters[26];
            let v26 = parameters[21];
            let v28 = parameters[22];
            let v30 = parameters[23];
            let v35 = parameters[15];
            let v37 = parameters[33];
            let v39 = parameters[16];
            let v41 = parameters[34];
            let v43 = parameters[17];
            let v48 = parameters[18];
            let v50 = parameters[19];
            let v52 = parameters[20];
            let v54 = 2.9214664e-1f64;
            let v55 = 5.178164370971076e-1f64;
            let v56 = 2e0f64;
            let v57 = 3e0f64;
            let v58 = 2.6992878119627894e-1f64;
            let v59 = 4.3792457880372104e-1f64;
            let v60 = parameters[14];
            let v63 = parameters[53];
            let v67 = parameters[54];
            let v71 = parameters[55];
            let v75 = parameters[50];
            let v77 = parameters[51];
            let v79 = parameters[52];
            let v102 = parameters[56];
            let v104 = parameters[57];
            let v107 = parameters[58];
            let v110 = parameters[59];
            let v116 = 1e-18f64;
            let v119 = 5e-2f64;
            let v124 = 9.5e-1f64;
            let v129 = temperature;
            let v130 = parameters[2];
            let v132 = parameters[9];
            let v134 = 2.3149999999999977e1f64;
            let v166 = parameters[27];
            let v169 = parameters[28];
            let v172 = parameters[29];
            let v230 = 3.2e1f64;
            let v231 = parameters[38];
            let v233 = 9.1093826e-31f64;
            let v240 = 3.1637150399999996e-34f64;
            let v242 = parameters[39];
            let v250 = 3.1637150399999996e-34f64;
            let v252 = parameters[40];
            let v260 = 3.1637150399999996e-34f64;
            let v262 = parameters[44];
            let v263 = parameters[47];
            let v268 = parameters[45];
            let v269 = parameters[48];
            let v273 = parameters[46];
            let v274 = parameters[49];
            let v284 = parameters[3];
            let v287 = parameters[4];
            let v290 = parameters[5];
            let v293 = parameters[6];
            let v298 = parameters[12];
            let v303 = 1e8f64;
            let v323 = 2.3025850929940458e2f64;
            let v327 = 1e-100f64;
            let v328 = -2.3025850929940458e2f64;
            let v330 = -2.3025850929940458e2f64;
            let v332 = -2.3025850929940458e2f64;
            let v334 = 3.333333333333333e-1f64;
            let v343 = 1e100f64;
            let v367 = 1e-1f64;
            let v376 = -4e-1f64;
            let v377 = parameters[63];
            let v379 = -6.5e-1f64;
            let v381 = -8e-1f64;
            let v383 = 2e-1f64;
            let v387 = 4e0f64;
            let v389 = -5e-1f64;
            let v394 = -5e-1f64;
            let v397 = -5e-1f64;
            let v400 = -2.3025850929940458e2f64;
            let v401 = -5e-1f64;
            let v404 = -2.3025850929940458e2f64;
            let v405 = -5e-1f64;
            let v408 = -2.3025850929940458e2f64;
            let v409 = -5e-1f64;
            let v420 = -5e-1f64;
            let v423 = -5e-1f64;
            let v426 = -5e-1f64;
            let v497 = 1e-6f64;
            let v498 = 4e-12f64;
            let v505 = parameters[30];
            let v507 = parameters[35];
            let v546 = 6.66666666666667e-1f64;
            let v558 = -1e0f64;
            let v571 = 3.75e-1f64;
            let v597 = -2.3025850929940458e2f64;
            let v600 = -2.3025850929940458e2f64;
            let v602 = -2.3025850929940458e2f64;
            let v604 = -2.3025850929940458e2f64;
            let v624 = -2.3025850929940458e2f64;
            let v627 = -2.3025850929940458e2f64;
            let v629 = -2.3025850929940458e2f64;
            let v631 = -2.3025850929940458e2f64;
            let v644 = 8.86226925452758e-1f64;
            let v653 = parameters[41];
            let v674 = -2.3025850929940458e2f64;
            let v676 = -2.3025850929940458e2f64;
            let v678 = -2.3025850929940458e2f64;
            let v703 = 1e3f64;
            let v724 = parameters[10];
            let v736 = parameters[31];
            let v738 = parameters[36];
            let v787 = -1e0f64;
            let v826 = -2.3025850929940458e2f64;
            let v829 = -2.3025850929940458e2f64;
            let v831 = -2.3025850929940458e2f64;
            let v833 = -2.3025850929940458e2f64;
            let v853 = -2.3025850929940458e2f64;
            let v856 = -2.3025850929940458e2f64;
            let v858 = -2.3025850929940458e2f64;
            let v860 = -2.3025850929940458e2f64;
            let v873 = 8.86226925452758e-1f64;
            let v883 = parameters[42];
            let v903 = -2.3025850929940458e2f64;
            let v905 = -2.3025850929940458e2f64;
            let v907 = -2.3025850929940458e2f64;
            let v962 = parameters[32];
            let v964 = parameters[37];
            let v1013 = -1e0f64;
            let v1052 = -2.3025850929940458e2f64;
            let v1055 = -2.3025850929940458e2f64;
            let v1057 = -2.3025850929940458e2f64;
            let v1059 = -2.3025850929940458e2f64;
            let v1079 = -2.3025850929940458e2f64;
            let v1082 = -2.3025850929940458e2f64;
            let v1084 = -2.3025850929940458e2f64;
            let v1086 = -2.3025850929940458e2f64;
            let v1099 = 8.86226925452758e-1f64;
            let v1109 = parameters[43];
            let v1129 = -2.3025850929940458e2f64;
            let v1131 = -2.3025850929940458e2f64;
            let v1133 = -2.3025850929940458e2f64;
            let v1196 = -5e-1f64;
            let v1201 = -5e-1f64;
            let v1204 = -5e-1f64;
            let v1207 = -2.3025850929940458e2f64;
            let v1208 = -5e-1f64;
            let v1211 = -2.3025850929940458e2f64;
            let v1212 = -5e-1f64;
            let v1215 = -2.3025850929940458e2f64;
            let v1216 = -5e-1f64;
            let v1227 = -5e-1f64;
            let v1230 = -5e-1f64;
            let v1233 = -5e-1f64;
            let v1302 = 4e-12f64;
            let v1361 = -1e0f64;
            let v1400 = -2.3025850929940458e2f64;
            let v1403 = -2.3025850929940458e2f64;
            let v1405 = -2.3025850929940458e2f64;
            let v1407 = -2.3025850929940458e2f64;
            let v1427 = -2.3025850929940458e2f64;
            let v1430 = -2.3025850929940458e2f64;
            let v1432 = -2.3025850929940458e2f64;
            let v1434 = -2.3025850929940458e2f64;
            let v1447 = 8.86226925452758e-1f64;
            let v1477 = -2.3025850929940458e2f64;
            let v1479 = -2.3025850929940458e2f64;
            let v1481 = -2.3025850929940458e2f64;
            let v1586 = -1e0f64;
            let v1625 = -2.3025850929940458e2f64;
            let v1628 = -2.3025850929940458e2f64;
            let v1630 = -2.3025850929940458e2f64;
            let v1632 = -2.3025850929940458e2f64;
            let v1652 = -2.3025850929940458e2f64;
            let v1655 = -2.3025850929940458e2f64;
            let v1657 = -2.3025850929940458e2f64;
            let v1659 = -2.3025850929940458e2f64;
            let v1672 = 8.86226925452758e-1f64;
            let v1701 = -2.3025850929940458e2f64;
            let v1703 = -2.3025850929940458e2f64;
            let v1705 = -2.3025850929940458e2f64;
            let v1809 = -1e0f64;
            let v1848 = -2.3025850929940458e2f64;
            let v1851 = -2.3025850929940458e2f64;
            let v1853 = -2.3025850929940458e2f64;
            let v1855 = -2.3025850929940458e2f64;
            let v1875 = -2.3025850929940458e2f64;
            let v1878 = -2.3025850929940458e2f64;
            let v1880 = -2.3025850929940458e2f64;
            let v1882 = -2.3025850929940458e2f64;
            let v1895 = 8.86226925452758e-1f64;
            let v1924 = -2.3025850929940458e2f64;
            let v1926 = -2.3025850929940458e2f64;
            let v1928 = -2.3025850929940458e2f64;
            let v1991 = -5e-1f64;
            let v1996 = -5e-1f64;
            let v1999 = -5e-1f64;
            let v2002 = -2.3025850929940458e2f64;
            let v2003 = -5e-1f64;
            let v2006 = -2.3025850929940458e2f64;
            let v2007 = -5e-1f64;
            let v2010 = -2.3025850929940458e2f64;
            let v2011 = -5e-1f64;
            let v2022 = -5e-1f64;
            let v2025 = -5e-1f64;
            let v2028 = -5e-1f64;
            let v2097 = 4e-12f64;
            let v2156 = -1e0f64;
            let v2195 = -2.3025850929940458e2f64;
            let v2198 = -2.3025850929940458e2f64;
            let v2200 = -2.3025850929940458e2f64;
            let v2202 = -2.3025850929940458e2f64;
            let v2222 = -2.3025850929940458e2f64;
            let v2225 = -2.3025850929940458e2f64;
            let v2227 = -2.3025850929940458e2f64;
            let v2229 = -2.3025850929940458e2f64;
            let v2242 = 8.86226925452758e-1f64;
            let v2272 = -2.3025850929940458e2f64;
            let v2274 = -2.3025850929940458e2f64;
            let v2276 = -2.3025850929940458e2f64;
            let v2381 = -1e0f64;
            let v2420 = -2.3025850929940458e2f64;
            let v2423 = -2.3025850929940458e2f64;
            let v2425 = -2.3025850929940458e2f64;
            let v2427 = -2.3025850929940458e2f64;
            let v2447 = -2.3025850929940458e2f64;
            let v2450 = -2.3025850929940458e2f64;
            let v2452 = -2.3025850929940458e2f64;
            let v2454 = -2.3025850929940458e2f64;
            let v2467 = 8.86226925452758e-1f64;
            let v2496 = -2.3025850929940458e2f64;
            let v2498 = -2.3025850929940458e2f64;
            let v2500 = -2.3025850929940458e2f64;
            let v2604 = -1e0f64;
            let v2643 = -2.3025850929940458e2f64;
            let v2646 = -2.3025850929940458e2f64;
            let v2648 = -2.3025850929940458e2f64;
            let v2650 = -2.3025850929940458e2f64;
            let v2670 = -2.3025850929940458e2f64;
            let v2673 = -2.3025850929940458e2f64;
            let v2675 = -2.3025850929940458e2f64;
            let v2677 = -2.3025850929940458e2f64;
            let v2690 = 8.86226925452758e-1f64;
            let v2719 = -2.3025850929940458e2f64;
            let v2721 = -2.3025850929940458e2f64;
            let v2723 = -2.3025850929940458e2f64;
            let v2786 = -5e-1f64;
            let v2791 = -5e-1f64;
            let v2794 = -5e-1f64;
            let v2797 = -2.3025850929940458e2f64;
            let v2798 = -5e-1f64;
            let v2801 = -2.3025850929940458e2f64;
            let v2802 = -5e-1f64;
            let v2805 = -2.3025850929940458e2f64;
            let v2806 = -5e-1f64;
            let v2817 = -5e-1f64;
            let v2820 = -5e-1f64;
            let v2823 = -5e-1f64;
            let v2846 = 1.0f64;
            let v2857 = -1e-1f64;
            let v2891 = -1.000000082740371e-11f64;
            let v2946 = -1e0f64;
            let v2985 = -2.3025850929940458e2f64;
            let v2988 = -2.3025850929940458e2f64;
            let v2990 = -2.3025850929940458e2f64;
            let v2992 = -2.3025850929940458e2f64;
            let v3012 = -2.3025850929940458e2f64;
            let v3015 = -2.3025850929940458e2f64;
            let v3017 = -2.3025850929940458e2f64;
            let v3019 = -2.3025850929940458e2f64;
            let v3032 = 8.86226925452758e-1f64;
            let v3062 = -2.3025850929940458e2f64;
            let v3064 = -2.3025850929940458e2f64;
            let v3066 = -2.3025850929940458e2f64;
            let v3171 = -1e0f64;
            let v3210 = -2.3025850929940458e2f64;
            let v3213 = -2.3025850929940458e2f64;
            let v3215 = -2.3025850929940458e2f64;
            let v3217 = -2.3025850929940458e2f64;
            let v3237 = -2.3025850929940458e2f64;
            let v3240 = -2.3025850929940458e2f64;
            let v3242 = -2.3025850929940458e2f64;
            let v3244 = -2.3025850929940458e2f64;
            let v3257 = 8.86226925452758e-1f64;
            let v3286 = -2.3025850929940458e2f64;
            let v3288 = -2.3025850929940458e2f64;
            let v3290 = -2.3025850929940458e2f64;
            let v3394 = -1e0f64;
            let v3433 = -2.3025850929940458e2f64;
            let v3436 = -2.3025850929940458e2f64;
            let v3438 = -2.3025850929940458e2f64;
            let v3440 = -2.3025850929940458e2f64;
            let v3460 = -2.3025850929940458e2f64;
            let v3463 = -2.3025850929940458e2f64;
            let v3465 = -2.3025850929940458e2f64;
            let v3467 = -2.3025850929940458e2f64;
            let v3480 = 8.86226925452758e-1f64;
            let v3509 = -2.3025850929940458e2f64;
            let v3511 = -2.3025850929940458e2f64;
            let v3513 = -2.3025850929940458e2f64;
            let v3576 = -5e-1f64;
            let v3581 = -5e-1f64;
            let v3584 = -5e-1f64;
            let v3587 = -2.3025850929940458e2f64;
            let v3588 = -5e-1f64;
            let v3591 = -2.3025850929940458e2f64;
            let v3592 = -5e-1f64;
            let v3595 = -2.3025850929940458e2f64;
            let v3596 = -5e-1f64;
            let v3607 = -5e-1f64;
            let v3610 = -5e-1f64;
            let v3613 = -5e-1f64;
            let v3636 = 1.0f64;
            let v3647 = -2e-1f64;
            let v3681 = -5.000000413701855e-12f64;
            let v3736 = -1e0f64;
            let v3775 = -2.3025850929940458e2f64;
            let v3778 = -2.3025850929940458e2f64;
            let v3780 = -2.3025850929940458e2f64;
            let v3782 = -2.3025850929940458e2f64;
            let v3802 = -2.3025850929940458e2f64;
            let v3805 = -2.3025850929940458e2f64;
            let v3807 = -2.3025850929940458e2f64;
            let v3809 = -2.3025850929940458e2f64;
            let v3822 = 8.86226925452758e-1f64;
            let v3852 = -2.3025850929940458e2f64;
            let v3854 = -2.3025850929940458e2f64;
            let v3856 = -2.3025850929940458e2f64;
            let v3961 = -1e0f64;
            let v4000 = -2.3025850929940458e2f64;
            let v4003 = -2.3025850929940458e2f64;
            let v4005 = -2.3025850929940458e2f64;
            let v4007 = -2.3025850929940458e2f64;
            let v4027 = -2.3025850929940458e2f64;
            let v4030 = -2.3025850929940458e2f64;
            let v4032 = -2.3025850929940458e2f64;
            let v4034 = -2.3025850929940458e2f64;
            let v4047 = 8.86226925452758e-1f64;
            let v4076 = -2.3025850929940458e2f64;
            let v4078 = -2.3025850929940458e2f64;
            let v4080 = -2.3025850929940458e2f64;
            let v4184 = -1e0f64;
            let v4223 = -2.3025850929940458e2f64;
            let v4226 = -2.3025850929940458e2f64;
            let v4228 = -2.3025850929940458e2f64;
            let v4230 = -2.3025850929940458e2f64;
            let v4250 = -2.3025850929940458e2f64;
            let v4253 = -2.3025850929940458e2f64;
            let v4255 = -2.3025850929940458e2f64;
            let v4257 = -2.3025850929940458e2f64;
            let v4270 = 8.86226925452758e-1f64;
            let v4299 = -2.3025850929940458e2f64;
            let v4301 = -2.3025850929940458e2f64;
            let v4303 = -2.3025850929940458e2f64;
            let v4381 = 1e-3f64;
            let v4395 = -1e-1f64;
            let v4483 = -5e-1f64;
            let v4495 = parameters[64];
            let v4506 = 1e-21f64;
            let v4531 = parameters[1];
            let v4532 = node_potentials[0];
            let v4533 = node_potentials[1];
            let v4537 = -2.3025850929940458e2f64;
            let v4539 = -2.3025850929940458e2f64;
            let v4558 = -2.3025850929940458e2f64;
            let v4560 = -2.3025850929940458e2f64;
            let v4594 = -2.3025850929940458e2f64;
            let v4596 = -2.3025850929940458e2f64;
            let v4631 = -5e-1f64;
            let v4636 = -5e-1f64;
            let v4639 = -5e-1f64;
            let v4642 = -2.3025850929940458e2f64;
            let v4643 = -5e-1f64;
            let v4646 = -2.3025850929940458e2f64;
            let v4647 = -5e-1f64;
            let v4650 = -2.3025850929940458e2f64;
            let v4651 = -5e-1f64;
            let v4662 = -5e-1f64;
            let v4665 = -5e-1f64;
            let v4668 = -5e-1f64;
            let v4737 = 4e-12f64;
            let v4794 = -1e0f64;
            let v4832 = -2.3025850929940458e2f64;
            let v4835 = -2.3025850929940458e2f64;
            let v4837 = -2.3025850929940458e2f64;
            let v4839 = -2.3025850929940458e2f64;
            let v4859 = -2.3025850929940458e2f64;
            let v4862 = -2.3025850929940458e2f64;
            let v4864 = -2.3025850929940458e2f64;
            let v4866 = -2.3025850929940458e2f64;
            let v4879 = 8.86226925452758e-1f64;
            let v4908 = -2.3025850929940458e2f64;
            let v4910 = -2.3025850929940458e2f64;
            let v4912 = -2.3025850929940458e2f64;
            let v5018 = -1e0f64;
            let v5057 = -2.3025850929940458e2f64;
            let v5060 = -2.3025850929940458e2f64;
            let v5062 = -2.3025850929940458e2f64;
            let v5064 = -2.3025850929940458e2f64;
            let v5084 = -2.3025850929940458e2f64;
            let v5087 = -2.3025850929940458e2f64;
            let v5089 = -2.3025850929940458e2f64;
            let v5091 = -2.3025850929940458e2f64;
            let v5104 = 8.86226925452758e-1f64;
            let v5133 = -2.3025850929940458e2f64;
            let v5135 = -2.3025850929940458e2f64;
            let v5137 = -2.3025850929940458e2f64;
            let v5242 = -1e0f64;
            let v5281 = -2.3025850929940458e2f64;
            let v5284 = -2.3025850929940458e2f64;
            let v5286 = -2.3025850929940458e2f64;
            let v5288 = -2.3025850929940458e2f64;
            let v5308 = -2.3025850929940458e2f64;
            let v5311 = -2.3025850929940458e2f64;
            let v5313 = -2.3025850929940458e2f64;
            let v5315 = -2.3025850929940458e2f64;
            let v5328 = 8.86226925452758e-1f64;
            let v5357 = -2.3025850929940458e2f64;
            let v5359 = -2.3025850929940458e2f64;
            let v5361 = -2.3025850929940458e2f64;
            let v5415 = parameters[60];
            let v5418 = parameters[61];
            let v5420 = 3.7e1f64;
            let v5421 = -3.7e1f64;
            let v5428 = 0e0f64;
            let v5439 = parameters[7];
            let v5441 = 3.2043836e-19f64;
            let v4 = if v2 > v3 { 1.0 } else { 0.0 };
            let v374: f64;
            if v4 != 0.0 {
                v374 = v5;
            } else {
                v374 = v0;
            }
            let v8 = v6 + v7;
            let v11 = v10 * v8;
            let v12 = v5 / v11;
            let v19 = (-((v13 * v8) * v8)) / (v17 + v8);
            let v21 = v20 + v19;
            let v23 = v22 + v19;
            let v25 = v24 + v19;
            let v27 = v5 - v26;
            let v29 = v5 - v28;
            let v31 = v5 - v30;
            let v32 = v5 / v27;
            let v33 = v5 / v29;
            let v34 = v5 / v31;
            let v36 = v1 / v35;
            let v40 = (v37 * v1) / v39;
            let v44 = (v41 * v1) / v43;
            let v45 = v5 / v36;
            let v46 = v5 / v40;
            let v47 = v5 / v44;
            let v49 = v5 / v48;
            let v51 = v5 / v50;
            let v53 = v5 / v52;
            let v62 = v5 - (v5 / v60);
            let v66 = v5 / (v5 - (v62.powf(v63)));
            let v70 = v5 / (v5 - (v62.powf(v67)));
            let v74 = v5 / (v5 - (v62.powf(v71)));
            let v76 = v5 / v75;
            let v78 = v5 / v77;
            let v80 = v5 / v79;
            let v87 = ((-((v66 * v66) * (v62.powf((v63 - v5))))) * v63) * v76;
            let v94 = ((-((v70 * v70) * (v62.powf((v67 - v5))))) * v67) * v78;
            let v101 = ((-((v74 * v74) * (v62.powf((v71 - v5))))) * v71) * v80;
            let v112 = if (if (if (if v102 != v5 { 1.0 } else { 0.0 }) != 0.0 || (if v104 != v5 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v107 != v5 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v110 != v5 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v113: f64;
            if v112 != 0.0 {
                v113 = v5;
            } else {
                v113 = v0;
            }
            let v114 = if v113 == v5 { 1.0 } else { 0.0 };
            let v5427: f64;
            if v114 != 0.0 {
                let v117 = if (v43 * v102) > v116 { 1.0 } else { 0.0 };
                if v117 != 0.0 {
                } else {
                }
                let v120 = if (v52 * v104) > v119 { 1.0 } else { 0.0 };
                if v120 != 0.0 {
                } else {
                }
                let v121 = v30 * v107;
                let v122 = if v121 > v119 { 1.0 } else { 0.0 };
                let v123: f64;
                if v122 != 0.0 {
                    v123 = v121;
                } else {
                    v123 = v119;
                }
                let v125 = if v123 < v124 { 1.0 } else { 0.0 };
                let v127: f64;
                if v125 != 0.0 {
                    let v126: f64;
                    if v122 != 0.0 {
                        v126 = v121;
                    } else {
                        v126 = v119;
                    }
                    v127 = v126;
                } else {
                    v127 = v124;
                }
                let v128 = v5 - v127;
                v5427 = v128;
            } else {
                v5427 = v5428;
            }
            let v135 = if ((v129 + v130) + v132) >= v134 { ((v129 + v130) + v132) } else { v134 };
            let v136 = v135 / v8;
            let v137 = v10 * v135;
            let v138 = v5 / v137;
            let v143 = (-((v13 * v135) * v135)) / (v17 + v135);
            let v144 = v20 + v143;
            let v145 = v22 + v143;
            let v146 = v24 + v143;
            let v147 = v136 * (v136.sqrt());
            let v153 = v147 * ((v3 * ((v21 * v12) - (v144 * v138))).exp());
            let v159 = v147 * ((v3 * ((v23 * v12) - (v145 * v138))).exp());
            let v165 = v147 * ((v3 * ((v25 * v12) - (v146 * v138))).exp());
            let v168 = (v166 * v153) * v153;
            let v171 = (v169 * v159) * v159;
            let v174 = (v172 * v165) * v165;
            let v176 = v56 * v137;
            let v179 = (v48 * v136) - (v176 * (v153.ln()));
            let v183 = (v50 * v136) - (v176 * (v159.ln()));
            let v187 = (v52 * v136) - (v176 * (v165.ln()));
            let v194 = v179 + (v137 * ((v5 + (((v119 - v179) * v138).exp())).ln()));
            let v201 = v183 + (v137 * ((v5 + (((v119 - v183) * v138).exp())).ln()));
            let v208 = v187 + (v137 * ((v5 + (((v119 - v187) * v138).exp())).ln()));
            let v214 = v35 * ((v48 * (v5 / v194)).powf(v26));
            let v217 = v39 * ((v50 * (v5 / v201)).powf(v28));
            let v220 = v43 * ((v52 * (v5 / v208)).powf(v30));
            let v222 = if (v3 * v144) >= v137 { (v3 * v144) } else { v137 };
            let v224 = if (v3 * v145) >= v137 { (v3 * v145) } else { v137 };
            let v226 = if (v3 * v146) >= v137 { (v3 * v146) } else { v137 };
            let v227 = v222 * v138;
            let v228 = v224 * v138;
            let v229 = v226 * v138;
            let v241 = (((((v230 * v231) * v233) * v9) * ((v222 * v222) * v222)).sqrt()) / v240;
            let v251 = (((((v230 * v242) * v233) * v9) * ((v224 * v224) * v224)).sqrt()) / v250;
            let v261 = (((((v230 * v252) * v233) * v9) * ((v226 * v226) * v226)).sqrt()) / v260;
            let v264 = v135 - v8;
            let v267 = v262 * (v5 + (v263 * v264));
            let v272 = v268 * (v5 + (v269 * v264));
            let v277 = v273 * (v5 + (v274 * v264));
            let v278 = if v267 > v0 { 1.0 } else { 0.0 };
            let v279: f64;
            if v278 != 0.0 {
                v279 = v267;
            } else {
                v279 = v0;
            }
            let v280 = if v272 > v0 { 1.0 } else { 0.0 };
            let v281: f64;
            if v280 != 0.0 {
                v281 = v272;
            } else {
                v281 = v0;
            }
            let v282 = if v277 > v0 { 1.0 } else { 0.0 };
            let v283: f64;
            if v282 != 0.0 {
                v283 = v277;
            } else {
                v283 = v0;
            }
            if v114 != 0.0 {
            } else {
            }
            let v285 = if v284 > v0 { 1.0 } else { 0.0 };
            let v286: f64;
            if v285 != 0.0 {
                v286 = v284;
            } else {
                v286 = v0;
            }
            let v288 = if v287 > v0 { 1.0 } else { 0.0 };
            let v289: f64;
            if v288 != 0.0 {
                v289 = v287;
            } else {
                v289 = v0;
            }
            let v291 = if v290 > v0 { 1.0 } else { 0.0 };
            let v292: f64;
            if v291 != 0.0 {
                v292 = v290;
            } else {
                v292 = v0;
            }
            let v294 = if v293 > v0 { 1.0 } else { 0.0 };
            let v295: f64;
            if v294 != 0.0 {
                v295 = v293;
            } else {
                v295 = v0;
            }
            let v296 = v168 * v286;
            let v297 = if v296 > v0 { 1.0 } else { 0.0 };
            let v316: f64;
            if v297 != 0.0 {
                let v302 = v137 * (((v298 / v296) + v5).ln());
                v316 = v302;
            } else {
                v316 = v303;
            }
            let v304 = v171 * v289;
            let v305 = if v304 > v0 { 1.0 } else { 0.0 };
            let v317: f64;
            if v305 != 0.0 {
                let v309 = v137 * (((v298 / v304) + v5).ln());
                v317 = v309;
            } else {
                v317 = v303;
            }
            let v310 = v174 * v292;
            let v311 = if v310 > v0 { 1.0 } else { 0.0 };
            let v319: f64;
            if v311 != 0.0 {
                let v315 = v137 * (((v298 / v310) + v5).ln());
                v319 = v315;
            } else {
                v319 = v303;
            }
            let v320 = if (if v316 <= v317 { v316 } else { v317 }) <= v319 { (if v316 <= v317 { v316 } else { v317 }) } else { v319 };
            let v321 = v320 * v138;
            let v324 = if (v321.abs()) < v323 { 1.0 } else { 0.0 };
            let v444: f64;
            if v324 != 0.0 {
                let v325 = v321.exp();
                v444 = v325;
            } else {
                let v326 = if v321 < v0 { 1.0 } else { 0.0 };
                let v445: f64;
                if v326 != 0.0 {
                    let v342 = v327 / (v5 + ((v328 - v321) * (v5 + (v3 * ((v330 - v321) * (v5 + ((v332 - v321) * v334)))))));
                    v445 = v342;
                } else {
                    let v344 = v321 - v323;
                    let v352 = v343 * (v5 + (v344 * (v5 + (v3 * (v344 * (v5 + (v344 * v334)))))));
                    v445 = v352;
                }
                v444 = v445;
            }
            let v353 = if v286 == v0 { 1.0 } else { 0.0 };
            let v362: f64;
            let v368: f64;
            if v353 != 0.0 {
                let v354 = v201 + v208;
                let v355 = v50 + v52;
                v362 = v354;
                v368 = v355;
            } else {
                v362 = v194;
                v368 = v48;
            }
            let v356 = if v289 == v0 { 1.0 } else { 0.0 };
            let v363: f64;
            let v369: f64;
            if v356 != 0.0 {
                let v357 = v194 + v208;
                let v358 = v48 + v52;
                v363 = v357;
                v369 = v358;
            } else {
                v363 = v201;
                v369 = v50;
            }
            let v359 = if v292 == v0 { 1.0 } else { 0.0 };
            let v365: f64;
            let v371: f64;
            if v359 != 0.0 {
                let v360 = v194 + v201;
                let v361 = v48 + v50;
                v365 = v360;
                v371 = v361;
            } else {
                v365 = v208;
                v371 = v52;
            }
            let v366 = if (if v362 <= v363 { v362 } else { v363 }) <= v365 { (if v362 <= v363 { v362 } else { v363 }) } else { v365 };
            let v373 = (if (if v368 <= v369 { v368 } else { v369 }) <= v371 { (if v368 <= v369 { v368 } else { v369 }) } else { v371 }) - v119;
            let v375 = if v374 == v5 { 1.0 } else { 0.0 };
            let v4543: f64;
            let v4545: f64;
            let v4552: f64;
            let v4555: f64;
            let v4564: f64;
            let v4566: f64;
            let v4573: f64;
            let v4576: f64;
            let v4582: f64;
            let v4583: f64;
            let v4600: f64;
            let v4602: f64;
            let v4615: f64;
            let v4619: f64;
            let v4623: f64;
            if v375 != 0.0 {
                let v378 = v376 * v377;
                let v380 = v379 * v377;
                let v382 = v381 * v377;
                let v386 = if (if (if v353 != 0.0 && v356 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v359 != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                let v503: f64;
                let v510: f64;
                let v512: f64;
                let v535: f64;
                let v656: f64;
                let v705: f64;
                if v386 != 0.0 {
                    let v388 = if v378 < v320 { 1.0 } else { 0.0 };
                    let v449: f64;
                    let v452: f64;
                    let v463: f64;
                    if v388 != 0.0 {
                        let v390 = v378 * v138;
                        let v393 = if ((v389 * v390).abs()) < v323 { 1.0 } else { 0.0 };
                        let v437: f64;
                        if v393 != 0.0 {
                            let v396 = (v394 * v390).exp();
                            v437 = v396;
                        } else {
                            let v399 = if (v397 * v390) < v0 { 1.0 } else { 0.0 };
                            let v438: f64;
                            if v399 != 0.0 {
                                let v419 = v327 / (v5 + ((v400 - (v401 * v390)) * (v5 + (v3 * ((v404 - (v405 * v390)) * (v5 + ((v408 - (v409 * v390)) * v334)))))));
                                v438 = v419;
                            } else {
                                let v436 = v343 * (v5 + (((v420 * v390) - v323) * (v5 + (v3 * (((v423 * v390) - v323) * (v5 + (((v426 * v390) - v323) * v334)))))));
                                v438 = v436;
                            }
                            v437 = v438;
                        }
                        let v439 = v5 / v437;
                        let v440 = v439 * v439;
                        v449 = v440;
                        v452 = v437;
                        v463 = v439;
                    } else {
                        let v446 = (v5 + ((v378 - v320) * v138)) * v444;
                        let v447 = v446.sqrt();
                        let v448 = v5 / v447;
                        v449 = v446;
                        v452 = v448;
                        v463 = v447;
                    }
                    let v450 = v449 - v5;
                    let v451 = if v378 > v0 { 1.0 } else { 0.0 };
                    let v476: f64;
                    if v451 != 0.0 {
                        let v461 = v56 * (v137 * (((v56 + v452) + (((v452 + v5) * (v452 + v57)).sqrt())).ln()));
                        v476 = v461;
                    } else {
                        let v475 = (-v378) + (v56 * (v137 * ((((v56 * v463) + v5) + (((v5 + v463) * (v5 + (v57 * v463))).sqrt())).ln())));
                        v476 = v475;
                    }
                    let v477 = v366 - v476;
                    let v479 = v378 - v477;
                    let v486 = v3 * ((v378 + v477) - (((v479 * v479) + ((v387 * v137) * v137)).sqrt()));
                    let v488 = v378 - v373;
                    let v495 = v3 * ((v378 + v373) - (((v488 * v488) + ((v387 * v11) * v11)).sqrt()));
                    let v502 = v3 * (v378 - (((v378 * v378) + v498).sqrt()));
                    v503 = v450;
                    v510 = v486;
                    v512 = v476;
                    v535 = v463;
                    v656 = v495;
                    v705 = v502;
                } else {
                    v503 = v0;
                    v510 = v0;
                    v512 = v0;
                    v535 = v0;
                    v656 = v0;
                    v705 = v0;
                }
                let v770: f64;
                let v773: f64;
                let v796: f64;
                let v879: f64;
                let v1187: f64;
                if v353 != 0.0 {
                    v770 = v0;
                    v773 = v0;
                    v796 = v0;
                    v879 = v0;
                    v1187 = v0;
                } else {
                    let v504 = v168 * v503;
                    let v508 = if v507 == v0 { 1.0 } else { 0.0 };
                    let v509 = if (if v505 == v0 { 1.0 } else { 0.0 }) != 0.0 && v508 != 0.0 { 1.0 } else { 0.0 };
                    let v541: f64;
                    let v543: f64;
                    let v566: f64;
                    let v649: f64;
                    let v725: f64;
                    if v509 != 0.0 {
                        v541 = v0;
                        v543 = v0;
                        v566 = v0;
                        v649 = v0;
                        v725 = v0;
                    } else {
                        let v511 = v194 - v510;
                        let v516 = v5 - ((v5 - (v512 / v511)).sqrt());
                        let v517 = if v26 == v3 { 1.0 } else { 0.0 };
                        let v527: f64;
                        if v517 != 0.0 {
                            v527 = v0;
                        } else {
                            let v526 = ((((v516 * v516) * (v516.ln())) / (v5 - v516)) + v516) * (v5 - (v56 * v26));
                            v527 = v526;
                        }
                        let v528 = v516 + v527;
                        let v533: f64;
                        if v517 != 0.0 {
                            let v530 = (v511 * v49).sqrt();
                            v533 = v530;
                        } else {
                            let v532 = (v511 * v49).powf(v26);
                            v533 = v532;
                        }
                        let v534 = v36 * v533;
                        let v538 = v153 * ((v535 - v5) * v534);
                        let v540 = v505 * (v538 * v528);
                        v541 = v534;
                        v543 = v511;
                        v566 = v528;
                        v649 = v538;
                        v725 = v540;
                    }
                    let v727: f64;
                    if v508 != 0.0 {
                        v727 = v0;
                    } else {
                        let v545 = v241 * ((v541 * v27) / v543);
                        let v548 = (v546 * v227) / v545;
                        let v549 = v548 * v548;
                        let v550 = v549 * v549;
                        let v553 = (v550 / (v550 + v5)).sqrt();
                        let v554 = v553.sqrt();
                        let v555 = v553 * v554;
                        let v557 = (-v26) * v32;
                        let v559 = if v557 == v558 { 1.0 } else { 0.0 };
                        let v567: f64;
                        if v559 != 0.0 {
                            let v562 = v5 / (v5 + (v545 * v555));
                            v567 = v562;
                        } else {
                            let v565 = (v5 + (v545 * v555)).powf(v557);
                            v567 = v565;
                        }
                        let v570 = (v566 * v567) / (v566 + v567);
                        let v574 = (v571 * (v545 / v554)).sqrt();
                        let v584 = (((v227 * v548) * v554) - (v227 * v553)) + (v3 * (v545 * v555));
                        let v586 = (((v56 * (v548 * v554)) - v553) - v5) * v574;
                        let v587 = v586 * v586;
                        let v588 = if v586 > v0 { 1.0 } else { 0.0 };
                        let v614: f64;
                        if v588 != 0.0 {
                            let v591 = v5 / (v5 + (v55 * v586));
                            v614 = v591;
                        } else {
                            let v594 = v5 / (v5 - (v55 * v586));
                            v614 = v594;
                        }
                        let v596 = (-v587) + v584;
                        let v598 = if v596 > v597 { 1.0 } else { 0.0 };
                        let v622: f64;
                        if v598 != 0.0 {
                            let v599 = v596.exp();
                            v622 = v599;
                        } else {
                            let v613 = v327 / (v5 + ((v600 - v596) * (v5 + (v3 * ((v602 - v596) * (v5 + ((v604 - v596) * v334)))))));
                            v622 = v613;
                        }
                        let v616 = v614 * v614;
                        let v623 = (((v54 * v614) + (v58 * v616)) + (v59 * (v616 * v614))) * v622;
                        let v645: f64;
                        if v588 != 0.0 {
                            v645 = v623;
                        } else {
                            let v625 = if v584 > v624 { 1.0 } else { 0.0 };
                            let v641: f64;
                            if v625 != 0.0 {
                                let v626 = v584.exp();
                                v641 = v626;
                            } else {
                                let v640 = v327 / (v5 + ((v627 - v584) * (v5 + (v3 * ((v629 - v584) * (v5 + ((v631 - v584) * v334)))))));
                                v641 = v640;
                            }
                            let v643 = (v56 * v641) - v623;
                            v645 = v643;
                        }
                        let v652 = v507 * ((v649 * (v644 * ((v227 * v645) / v574))) * v570);
                        v727 = v652;
                    }
                    let v654 = if v653 == v0 { 1.0 } else { 0.0 };
                    let v729: f64;
                    if v654 != 0.0 {
                        v729 = v0;
                    } else {
                        let v655 = if v26 == v3 { 1.0 } else { 0.0 };
                        let v665: f64;
                        if v655 != 0.0 {
                            let v659 = ((v48 - v656) * v49).sqrt();
                            v665 = v659;
                        } else {
                            let v662 = ((v48 - v656) * v49).powf(v26);
                            v665 = v662;
                        }
                        let v667 = v32 * (((v48 - v656) * v45) / v665);
                        let v669 = (-v279) / v667;
                        let v671 = if (v669.abs()) < v323 { 1.0 } else { 0.0 };
                        let v699: f64;
                        if v671 != 0.0 {
                            let v672 = v669.exp();
                            v699 = v672;
                        } else {
                            let v673 = if v669 < v0 { 1.0 } else { 0.0 };
                            let v700: f64;
                            if v673 != 0.0 {
                                let v687 = v327 / (v5 + ((v674 - v669) * (v5 + (v3 * ((v676 - v669) * (v5 + ((v678 - v669) * v334)))))));
                                v700 = v687;
                            } else {
                                let v688 = v669 - v323;
                                let v696 = v343 * (v5 + (v688 * (v5 + (v3 * (v688 * (v5 + (v688 * v334)))))));
                                v700 = v696;
                            }
                            v699 = v700;
                        }
                        let v702 = v653 * (((v378 * v667) * v667) * v699);
                        v729 = v702;
                    }
                    let v704 = if v75 > v703 { 1.0 } else { 0.0 };
                    let v732: f64;
                    if v704 != 0.0 {
                        v732 = v5;
                    } else {
                        let v708 = if v705 > ((-v62) * v75) { 1.0 } else { 0.0 };
                        let v733: f64;
                        if v708 != 0.0 {
                            let v709 = if v63 == v387 { 1.0 } else { 0.0 };
                            let v717: f64;
                            if v709 != 0.0 {
                                let v710 = v705 * v76;
                                let v713 = ((v710 * v710) * v710) * v710;
                                v717 = v713;
                            } else {
                                let v716 = ((v705 * v76).abs()).powf(v63);
                                v717 = v716;
                            }
                            let v719 = v5 / (v5 - v717);
                            v733 = v719;
                        } else {
                            let v723 = v66 + ((v705 + (v62 * v75)) * v87);
                            v733 = v723;
                        }
                        v732 = v733;
                    }
                    let v734 = (v724 * (((v504 + v725) + v727) + v729)) * v732;
                    v770 = v541;
                    v773 = v543;
                    v796 = v566;
                    v879 = v649;
                    v1187 = v734;
                }
                let v996: f64;
                let v999: f64;
                let v1022: f64;
                let v1105: f64;
                let v1189: f64;
                if v356 != 0.0 {
                    v996 = v770;
                    v999 = v773;
                    v1022 = v796;
                    v1105 = v879;
                    v1189 = v0;
                } else {
                    let v735 = v171 * v503;
                    let v739 = if v738 == v0 { 1.0 } else { 0.0 };
                    let v740 = if (if v736 == v0 { 1.0 } else { 0.0 }) != 0.0 && v739 != 0.0 { 1.0 } else { 0.0 };
                    let v769: f64;
                    let v772: f64;
                    let v795: f64;
                    let v878: f64;
                    let v951: f64;
                    if v740 != 0.0 {
                        v769 = v770;
                        v772 = v773;
                        v795 = v796;
                        v878 = v879;
                        v951 = v0;
                    } else {
                        let v741 = v201 - v510;
                        let v745 = v5 - ((v5 - (v512 / v741)).sqrt());
                        let v746 = if v28 == v3 { 1.0 } else { 0.0 };
                        let v756: f64;
                        if v746 != 0.0 {
                            v756 = v0;
                        } else {
                            let v755 = ((((v745 * v745) * (v745.ln())) / (v5 - v745)) + v745) * (v5 - (v56 * v28));
                            v756 = v755;
                        }
                        let v757 = v745 + v756;
                        let v762: f64;
                        if v746 != 0.0 {
                            let v759 = (v741 * v51).sqrt();
                            v762 = v759;
                        } else {
                            let v761 = (v741 * v51).powf(v28);
                            v762 = v761;
                        }
                        let v763 = v40 * v762;
                        let v766 = v159 * ((v535 - v5) * v763);
                        let v768 = v736 * (v766 * v757);
                        v769 = v763;
                        v772 = v741;
                        v795 = v757;
                        v878 = v766;
                        v951 = v768;
                    }
                    let v953: f64;
                    if v739 != 0.0 {
                        v953 = v0;
                    } else {
                        let v775 = v251 * ((v769 * v29) / v772);
                        let v777 = (v546 * v228) / v775;
                        let v778 = v777 * v777;
                        let v779 = v778 * v778;
                        let v782 = (v779 / (v779 + v5)).sqrt();
                        let v783 = v782.sqrt();
                        let v784 = v782 * v783;
                        let v786 = (-v28) * v33;
                        let v788 = if v786 == v787 { 1.0 } else { 0.0 };
                        let v797: f64;
                        if v788 != 0.0 {
                            let v791 = v5 / (v5 + (v775 * v784));
                            v797 = v791;
                        } else {
                            let v794 = (v5 + (v775 * v784)).powf(v786);
                            v797 = v794;
                        }
                        let v800 = (v795 * v797) / (v795 + v797);
                        let v803 = (v571 * (v775 / v783)).sqrt();
                        let v813 = (((v228 * v777) * v783) - (v228 * v782)) + (v3 * (v775 * v784));
                        let v815 = (((v56 * (v777 * v783)) - v782) - v5) * v803;
                        let v816 = v815 * v815;
                        let v817 = if v815 > v0 { 1.0 } else { 0.0 };
                        let v843: f64;
                        if v817 != 0.0 {
                            let v820 = v5 / (v5 + (v55 * v815));
                            v843 = v820;
                        } else {
                            let v823 = v5 / (v5 - (v55 * v815));
                            v843 = v823;
                        }
                        let v825 = (-v816) + v813;
                        let v827 = if v825 > v826 { 1.0 } else { 0.0 };
                        let v851: f64;
                        if v827 != 0.0 {
                            let v828 = v825.exp();
                            v851 = v828;
                        } else {
                            let v842 = v327 / (v5 + ((v829 - v825) * (v5 + (v3 * ((v831 - v825) * (v5 + ((v833 - v825) * v334)))))));
                            v851 = v842;
                        }
                        let v845 = v843 * v843;
                        let v852 = (((v54 * v843) + (v58 * v845)) + (v59 * (v845 * v843))) * v851;
                        let v874: f64;
                        if v817 != 0.0 {
                            v874 = v852;
                        } else {
                            let v854 = if v813 > v853 { 1.0 } else { 0.0 };
                            let v870: f64;
                            if v854 != 0.0 {
                                let v855 = v813.exp();
                                v870 = v855;
                            } else {
                                let v869 = v327 / (v5 + ((v856 - v813) * (v5 + (v3 * ((v858 - v813) * (v5 + ((v860 - v813) * v334)))))));
                                v870 = v869;
                            }
                            let v872 = (v56 * v870) - v852;
                            v874 = v872;
                        }
                        let v882 = v738 * ((v878 * (v873 * ((v228 * v874) / v803))) * v800);
                        v953 = v882;
                    }
                    let v884 = if v883 == v0 { 1.0 } else { 0.0 };
                    let v955: f64;
                    if v884 != 0.0 {
                        v955 = v0;
                    } else {
                        let v885 = if v28 == v3 { 1.0 } else { 0.0 };
                        let v894: f64;
                        if v885 != 0.0 {
                            let v888 = ((v50 - v656) * v51).sqrt();
                            v894 = v888;
                        } else {
                            let v891 = ((v50 - v656) * v51).powf(v28);
                            v894 = v891;
                        }
                        let v896 = v33 * (((v50 - v656) * v46) / v894);
                        let v898 = (-v281) / v896;
                        let v900 = if (v898.abs()) < v323 { 1.0 } else { 0.0 };
                        let v928: f64;
                        if v900 != 0.0 {
                            let v901 = v898.exp();
                            v928 = v901;
                        } else {
                            let v902 = if v898 < v0 { 1.0 } else { 0.0 };
                            let v929: f64;
                            if v902 != 0.0 {
                                let v916 = v327 / (v5 + ((v903 - v898) * (v5 + (v3 * ((v905 - v898) * (v5 + ((v907 - v898) * v334)))))));
                                v929 = v916;
                            } else {
                                let v917 = v898 - v323;
                                let v925 = v343 * (v5 + (v917 * (v5 + (v3 * (v917 * (v5 + (v917 * v334)))))));
                                v929 = v925;
                            }
                            v928 = v929;
                        }
                        let v931 = v883 * (((v378 * v896) * v896) * v928);
                        v955 = v931;
                    }
                    let v932 = if v77 > v703 { 1.0 } else { 0.0 };
                    let v958: f64;
                    if v932 != 0.0 {
                        v958 = v5;
                    } else {
                        let v935 = if v705 > ((-v62) * v77) { 1.0 } else { 0.0 };
                        let v959: f64;
                        if v935 != 0.0 {
                            let v936 = if v67 == v387 { 1.0 } else { 0.0 };
                            let v944: f64;
                            if v936 != 0.0 {
                                let v937 = v705 * v78;
                                let v940 = ((v937 * v937) * v937) * v937;
                                v944 = v940;
                            } else {
                                let v943 = ((v705 * v78).abs()).powf(v67);
                                v944 = v943;
                            }
                            let v946 = v5 / (v5 - v944);
                            v959 = v946;
                        } else {
                            let v950 = v70 + ((v705 + (v62 * v77)) * v94);
                            v959 = v950;
                        }
                        v958 = v959;
                    }
                    let v960 = (v724 * (((v735 + v951) + v953) + v955)) * v958;
                    v996 = v769;
                    v999 = v772;
                    v1022 = v795;
                    v1105 = v878;
                    v1189 = v960;
                }
                let v1192: f64;
                let v1344: f64;
                let v1347: f64;
                let v1370: f64;
                let v1453: f64;
                if v359 != 0.0 {
                    v1192 = v0;
                    v1344 = v996;
                    v1347 = v999;
                    v1370 = v1022;
                    v1453 = v1105;
                } else {
                    let v961 = v174 * v503;
                    let v965 = if v964 == v0 { 1.0 } else { 0.0 };
                    let v966 = if (if v962 == v0 { 1.0 } else { 0.0 }) != 0.0 && v965 != 0.0 { 1.0 } else { 0.0 };
                    let v995: f64;
                    let v998: f64;
                    let v1021: f64;
                    let v1104: f64;
                    let v1177: f64;
                    if v966 != 0.0 {
                        v995 = v996;
                        v998 = v999;
                        v1021 = v1022;
                        v1104 = v1105;
                        v1177 = v0;
                    } else {
                        let v967 = v208 - v510;
                        let v971 = v5 - ((v5 - (v512 / v967)).sqrt());
                        let v972 = if v30 == v3 { 1.0 } else { 0.0 };
                        let v982: f64;
                        if v972 != 0.0 {
                            v982 = v0;
                        } else {
                            let v981 = ((((v971 * v971) * (v971.ln())) / (v5 - v971)) + v971) * (v5 - (v56 * v30));
                            v982 = v981;
                        }
                        let v983 = v971 + v982;
                        let v988: f64;
                        if v972 != 0.0 {
                            let v985 = (v967 * v53).sqrt();
                            v988 = v985;
                        } else {
                            let v987 = (v967 * v53).powf(v30);
                            v988 = v987;
                        }
                        let v989 = v44 * v988;
                        let v992 = v165 * ((v535 - v5) * v989);
                        let v994 = v962 * (v992 * v983);
                        v995 = v989;
                        v998 = v967;
                        v1021 = v983;
                        v1104 = v992;
                        v1177 = v994;
                    }
                    let v1179: f64;
                    if v965 != 0.0 {
                        v1179 = v0;
                    } else {
                        let v1001 = v261 * ((v995 * v31) / v998);
                        let v1003 = (v546 * v229) / v1001;
                        let v1004 = v1003 * v1003;
                        let v1005 = v1004 * v1004;
                        let v1008 = (v1005 / (v1005 + v5)).sqrt();
                        let v1009 = v1008.sqrt();
                        let v1010 = v1008 * v1009;
                        let v1012 = (-v30) * v34;
                        let v1014 = if v1012 == v1013 { 1.0 } else { 0.0 };
                        let v1023: f64;
                        if v1014 != 0.0 {
                            let v1017 = v5 / (v5 + (v1001 * v1010));
                            v1023 = v1017;
                        } else {
                            let v1020 = (v5 + (v1001 * v1010)).powf(v1012);
                            v1023 = v1020;
                        }
                        let v1026 = (v1021 * v1023) / (v1021 + v1023);
                        let v1029 = (v571 * (v1001 / v1009)).sqrt();
                        let v1039 = (((v229 * v1003) * v1009) - (v229 * v1008)) + (v3 * (v1001 * v1010));
                        let v1041 = (((v56 * (v1003 * v1009)) - v1008) - v5) * v1029;
                        let v1042 = v1041 * v1041;
                        let v1043 = if v1041 > v0 { 1.0 } else { 0.0 };
                        let v1069: f64;
                        if v1043 != 0.0 {
                            let v1046 = v5 / (v5 + (v55 * v1041));
                            v1069 = v1046;
                        } else {
                            let v1049 = v5 / (v5 - (v55 * v1041));
                            v1069 = v1049;
                        }
                        let v1051 = (-v1042) + v1039;
                        let v1053 = if v1051 > v1052 { 1.0 } else { 0.0 };
                        let v1077: f64;
                        if v1053 != 0.0 {
                            let v1054 = v1051.exp();
                            v1077 = v1054;
                        } else {
                            let v1068 = v327 / (v5 + ((v1055 - v1051) * (v5 + (v3 * ((v1057 - v1051) * (v5 + ((v1059 - v1051) * v334)))))));
                            v1077 = v1068;
                        }
                        let v1071 = v1069 * v1069;
                        let v1078 = (((v54 * v1069) + (v58 * v1071)) + (v59 * (v1071 * v1069))) * v1077;
                        let v1100: f64;
                        if v1043 != 0.0 {
                            v1100 = v1078;
                        } else {
                            let v1080 = if v1039 > v1079 { 1.0 } else { 0.0 };
                            let v1096: f64;
                            if v1080 != 0.0 {
                                let v1081 = v1039.exp();
                                v1096 = v1081;
                            } else {
                                let v1095 = v327 / (v5 + ((v1082 - v1039) * (v5 + (v3 * ((v1084 - v1039) * (v5 + ((v1086 - v1039) * v334)))))));
                                v1096 = v1095;
                            }
                            let v1098 = (v56 * v1096) - v1078;
                            v1100 = v1098;
                        }
                        let v1108 = v964 * ((v1104 * (v1099 * ((v229 * v1100) / v1029))) * v1026);
                        v1179 = v1108;
                    }
                    let v1110 = if v1109 == v0 { 1.0 } else { 0.0 };
                    let v1181: f64;
                    if v1110 != 0.0 {
                        v1181 = v0;
                    } else {
                        let v1111 = if v30 == v3 { 1.0 } else { 0.0 };
                        let v1120: f64;
                        if v1111 != 0.0 {
                            let v1114 = ((v52 - v656) * v53).sqrt();
                            v1120 = v1114;
                        } else {
                            let v1117 = ((v52 - v656) * v53).powf(v30);
                            v1120 = v1117;
                        }
                        let v1122 = v34 * (((v52 - v656) * v47) / v1120);
                        let v1124 = (-v283) / v1122;
                        let v1126 = if (v1124.abs()) < v323 { 1.0 } else { 0.0 };
                        let v1154: f64;
                        if v1126 != 0.0 {
                            let v1127 = v1124.exp();
                            v1154 = v1127;
                        } else {
                            let v1128 = if v1124 < v0 { 1.0 } else { 0.0 };
                            let v1155: f64;
                            if v1128 != 0.0 {
                                let v1142 = v327 / (v5 + ((v1129 - v1124) * (v5 + (v3 * ((v1131 - v1124) * (v5 + ((v1133 - v1124) * v334)))))));
                                v1155 = v1142;
                            } else {
                                let v1143 = v1124 - v323;
                                let v1151 = v343 * (v5 + (v1143 * (v5 + (v3 * (v1143 * (v5 + (v1143 * v334)))))));
                                v1155 = v1151;
                            }
                            v1154 = v1155;
                        }
                        let v1157 = v1109 * (((v378 * v1122) * v1122) * v1154);
                        v1181 = v1157;
                    }
                    let v1158 = if v79 > v703 { 1.0 } else { 0.0 };
                    let v1184: f64;
                    if v1158 != 0.0 {
                        v1184 = v5;
                    } else {
                        let v1161 = if v705 > ((-v62) * v79) { 1.0 } else { 0.0 };
                        let v1185: f64;
                        if v1161 != 0.0 {
                            let v1162 = if v71 == v387 { 1.0 } else { 0.0 };
                            let v1170: f64;
                            if v1162 != 0.0 {
                                let v1163 = v705 * v80;
                                let v1166 = ((v1163 * v1163) * v1163) * v1163;
                                v1170 = v1166;
                            } else {
                                let v1169 = ((v705 * v80).abs()).powf(v71);
                                v1170 = v1169;
                            }
                            let v1172 = v5 / (v5 - v1170);
                            v1185 = v1172;
                        } else {
                            let v1176 = v74 + ((v705 + (v62 * v79)) * v101);
                            v1185 = v1176;
                        }
                        v1184 = v1185;
                    }
                    let v1186 = (v724 * (((v961 + v1177) + v1179) + v1181)) * v1184;
                    v1192 = v1186;
                    v1344 = v995;
                    v1347 = v998;
                    v1370 = v1021;
                    v1453 = v1104;
                }
                let v1194 = ((v286 * v1187) + (v289 * v1189)) + (v292 * v1192);
                let v1307: f64;
                let v1312: f64;
                let v1314: f64;
                let v1337: f64;
                let v1459: f64;
                let v1507: f64;
                if v386 != 0.0 {
                    let v1195 = if v380 < v320 { 1.0 } else { 0.0 };
                    let v1254: f64;
                    let v1257: f64;
                    let v1268: f64;
                    if v1195 != 0.0 {
                        let v1197 = v380 * v138;
                        let v1200 = if ((v1196 * v1197).abs()) < v323 { 1.0 } else { 0.0 };
                        let v1244: f64;
                        if v1200 != 0.0 {
                            let v1203 = (v1201 * v1197).exp();
                            v1244 = v1203;
                        } else {
                            let v1206 = if (v1204 * v1197) < v0 { 1.0 } else { 0.0 };
                            let v1245: f64;
                            if v1206 != 0.0 {
                                let v1226 = v327 / (v5 + ((v1207 - (v1208 * v1197)) * (v5 + (v3 * ((v1211 - (v1212 * v1197)) * (v5 + ((v1215 - (v1216 * v1197)) * v334)))))));
                                v1245 = v1226;
                            } else {
                                let v1243 = v343 * (v5 + (((v1227 * v1197) - v323) * (v5 + (v3 * (((v1230 * v1197) - v323) * (v5 + (((v1233 * v1197) - v323) * v334)))))));
                                v1245 = v1243;
                            }
                            v1244 = v1245;
                        }
                        let v1246 = v5 / v1244;
                        let v1247 = v1246 * v1246;
                        v1254 = v1247;
                        v1257 = v1244;
                        v1268 = v1246;
                    } else {
                        let v1251 = (v5 + ((v380 - v320) * v138)) * v444;
                        let v1252 = v1251.sqrt();
                        let v1253 = v5 / v1252;
                        v1254 = v1251;
                        v1257 = v1253;
                        v1268 = v1252;
                    }
                    let v1255 = v1254 - v5;
                    let v1256 = if v380 > v0 { 1.0 } else { 0.0 };
                    let v1281: f64;
                    if v1256 != 0.0 {
                        let v1266 = v56 * (v137 * (((v56 + v1257) + (((v1257 + v5) * (v1257 + v57)).sqrt())).ln()));
                        v1281 = v1266;
                    } else {
                        let v1280 = (-v380) + (v56 * (v137 * ((((v56 * v1268) + v5) + (((v5 + v1268) * (v5 + (v57 * v1268))).sqrt())).ln())));
                        v1281 = v1280;
                    }
                    let v1282 = v366 - v1281;
                    let v1284 = v380 - v1282;
                    let v1291 = v3 * ((v380 + v1282) - (((v1284 * v1284) + ((v387 * v137) * v137)).sqrt()));
                    let v1293 = v380 - v373;
                    let v1300 = v3 * ((v380 + v373) - (((v1293 * v1293) + ((v387 * v11) * v11)).sqrt()));
                    let v1306 = v3 * (v380 - (((v380 * v380) + v1302).sqrt()));
                    v1307 = v1255;
                    v1312 = v1291;
                    v1314 = v1281;
                    v1337 = v1268;
                    v1459 = v1300;
                    v1507 = v1306;
                } else {
                    v1307 = v503;
                    v1312 = v510;
                    v1314 = v0;
                    v1337 = v535;
                    v1459 = v0;
                    v1507 = v705;
                }
                let v1569: f64;
                let v1572: f64;
                let v1595: f64;
                let v1678: f64;
                let v1982: f64;
                if v353 != 0.0 {
                    v1569 = v1344;
                    v1572 = v1347;
                    v1595 = v1370;
                    v1678 = v1453;
                    v1982 = v0;
                } else {
                    let v1308 = v168 * v1307;
                    let v1310 = if v507 == v0 { 1.0 } else { 0.0 };
                    let v1311 = if (if v505 == v0 { 1.0 } else { 0.0 }) != 0.0 && v1310 != 0.0 { 1.0 } else { 0.0 };
                    let v1343: f64;
                    let v1346: f64;
                    let v1369: f64;
                    let v1452: f64;
                    let v1526: f64;
                    if v1311 != 0.0 {
                        v1343 = v1344;
                        v1346 = v1347;
                        v1369 = v1370;
                        v1452 = v1453;
                        v1526 = v0;
                    } else {
                        let v1313 = v194 - v1312;
                        let v1318 = v5 - ((v5 - (v1314 / v1313)).sqrt());
                        let v1319 = if v26 == v3 { 1.0 } else { 0.0 };
                        let v1329: f64;
                        if v1319 != 0.0 {
                            v1329 = v0;
                        } else {
                            let v1328 = ((((v1318 * v1318) * (v1318.ln())) / (v5 - v1318)) + v1318) * (v5 - (v56 * v26));
                            v1329 = v1328;
                        }
                        let v1330 = v1318 + v1329;
                        let v1335: f64;
                        if v1319 != 0.0 {
                            let v1332 = (v1313 * v49).sqrt();
                            v1335 = v1332;
                        } else {
                            let v1334 = (v1313 * v49).powf(v26);
                            v1335 = v1334;
                        }
                        let v1336 = v36 * v1335;
                        let v1340 = v153 * ((v1337 - v5) * v1336);
                        let v1342 = v505 * (v1340 * v1330);
                        v1343 = v1336;
                        v1346 = v1313;
                        v1369 = v1330;
                        v1452 = v1340;
                        v1526 = v1342;
                    }
                    let v1528: f64;
                    if v1310 != 0.0 {
                        v1528 = v0;
                    } else {
                        let v1349 = v241 * ((v1343 * v27) / v1346);
                        let v1351 = (v546 * v227) / v1349;
                        let v1352 = v1351 * v1351;
                        let v1353 = v1352 * v1352;
                        let v1356 = (v1353 / (v1353 + v5)).sqrt();
                        let v1357 = v1356.sqrt();
                        let v1358 = v1356 * v1357;
                        let v1360 = (-v26) * v32;
                        let v1362 = if v1360 == v1361 { 1.0 } else { 0.0 };
                        let v1371: f64;
                        if v1362 != 0.0 {
                            let v1365 = v5 / (v5 + (v1349 * v1358));
                            v1371 = v1365;
                        } else {
                            let v1368 = (v5 + (v1349 * v1358)).powf(v1360);
                            v1371 = v1368;
                        }
                        let v1374 = (v1369 * v1371) / (v1369 + v1371);
                        let v1377 = (v571 * (v1349 / v1357)).sqrt();
                        let v1387 = (((v227 * v1351) * v1357) - (v227 * v1356)) + (v3 * (v1349 * v1358));
                        let v1389 = (((v56 * (v1351 * v1357)) - v1356) - v5) * v1377;
                        let v1390 = v1389 * v1389;
                        let v1391 = if v1389 > v0 { 1.0 } else { 0.0 };
                        let v1417: f64;
                        if v1391 != 0.0 {
                            let v1394 = v5 / (v5 + (v55 * v1389));
                            v1417 = v1394;
                        } else {
                            let v1397 = v5 / (v5 - (v55 * v1389));
                            v1417 = v1397;
                        }
                        let v1399 = (-v1390) + v1387;
                        let v1401 = if v1399 > v1400 { 1.0 } else { 0.0 };
                        let v1425: f64;
                        if v1401 != 0.0 {
                            let v1402 = v1399.exp();
                            v1425 = v1402;
                        } else {
                            let v1416 = v327 / (v5 + ((v1403 - v1399) * (v5 + (v3 * ((v1405 - v1399) * (v5 + ((v1407 - v1399) * v334)))))));
                            v1425 = v1416;
                        }
                        let v1419 = v1417 * v1417;
                        let v1426 = (((v54 * v1417) + (v58 * v1419)) + (v59 * (v1419 * v1417))) * v1425;
                        let v1448: f64;
                        if v1391 != 0.0 {
                            v1448 = v1426;
                        } else {
                            let v1428 = if v1387 > v1427 { 1.0 } else { 0.0 };
                            let v1444: f64;
                            if v1428 != 0.0 {
                                let v1429 = v1387.exp();
                                v1444 = v1429;
                            } else {
                                let v1443 = v327 / (v5 + ((v1430 - v1387) * (v5 + (v3 * ((v1432 - v1387) * (v5 + ((v1434 - v1387) * v334)))))));
                                v1444 = v1443;
                            }
                            let v1446 = (v56 * v1444) - v1426;
                            v1448 = v1446;
                        }
                        let v1456 = v507 * ((v1452 * (v1447 * ((v227 * v1448) / v1377))) * v1374);
                        v1528 = v1456;
                    }
                    let v1457 = if v653 == v0 { 1.0 } else { 0.0 };
                    let v1530: f64;
                    if v1457 != 0.0 {
                        v1530 = v0;
                    } else {
                        let v1458 = if v26 == v3 { 1.0 } else { 0.0 };
                        let v1468: f64;
                        if v1458 != 0.0 {
                            let v1462 = ((v48 - v1459) * v49).sqrt();
                            v1468 = v1462;
                        } else {
                            let v1465 = ((v48 - v1459) * v49).powf(v26);
                            v1468 = v1465;
                        }
                        let v1470 = v32 * (((v48 - v1459) * v45) / v1468);
                        let v1472 = (-v279) / v1470;
                        let v1474 = if (v1472.abs()) < v323 { 1.0 } else { 0.0 };
                        let v1502: f64;
                        if v1474 != 0.0 {
                            let v1475 = v1472.exp();
                            v1502 = v1475;
                        } else {
                            let v1476 = if v1472 < v0 { 1.0 } else { 0.0 };
                            let v1503: f64;
                            if v1476 != 0.0 {
                                let v1490 = v327 / (v5 + ((v1477 - v1472) * (v5 + (v3 * ((v1479 - v1472) * (v5 + ((v1481 - v1472) * v334)))))));
                                v1503 = v1490;
                            } else {
                                let v1491 = v1472 - v323;
                                let v1499 = v343 * (v5 + (v1491 * (v5 + (v3 * (v1491 * (v5 + (v1491 * v334)))))));
                                v1503 = v1499;
                            }
                            v1502 = v1503;
                        }
                        let v1505 = v653 * (((v380 * v1470) * v1470) * v1502);
                        v1530 = v1505;
                    }
                    let v1506 = if v75 > v703 { 1.0 } else { 0.0 };
                    let v1533: f64;
                    if v1506 != 0.0 {
                        v1533 = v5;
                    } else {
                        let v1510 = if v1507 > ((-v62) * v75) { 1.0 } else { 0.0 };
                        let v1534: f64;
                        if v1510 != 0.0 {
                            let v1511 = if v63 == v387 { 1.0 } else { 0.0 };
                            let v1519: f64;
                            if v1511 != 0.0 {
                                let v1512 = v1507 * v76;
                                let v1515 = ((v1512 * v1512) * v1512) * v1512;
                                v1519 = v1515;
                            } else {
                                let v1518 = ((v1507 * v76).abs()).powf(v63);
                                v1519 = v1518;
                            }
                            let v1521 = v5 / (v5 - v1519);
                            v1534 = v1521;
                        } else {
                            let v1525 = v66 + ((v1507 + (v62 * v75)) * v87);
                            v1534 = v1525;
                        }
                        v1533 = v1534;
                    }
                    let v1535 = (v724 * (((v1308 + v1526) + v1528) + v1530)) * v1533;
                    v1569 = v1343;
                    v1572 = v1346;
                    v1595 = v1369;
                    v1678 = v1452;
                    v1982 = v1535;
                }
                let v1792: f64;
                let v1795: f64;
                let v1818: f64;
                let v1901: f64;
                let v1984: f64;
                if v356 != 0.0 {
                    v1792 = v1569;
                    v1795 = v1572;
                    v1818 = v1595;
                    v1901 = v1678;
                    v1984 = v0;
                } else {
                    let v1536 = v171 * v1307;
                    let v1538 = if v738 == v0 { 1.0 } else { 0.0 };
                    let v1539 = if (if v736 == v0 { 1.0 } else { 0.0 }) != 0.0 && v1538 != 0.0 { 1.0 } else { 0.0 };
                    let v1568: f64;
                    let v1571: f64;
                    let v1594: f64;
                    let v1677: f64;
                    let v1749: f64;
                    if v1539 != 0.0 {
                        v1568 = v1569;
                        v1571 = v1572;
                        v1594 = v1595;
                        v1677 = v1678;
                        v1749 = v0;
                    } else {
                        let v1540 = v201 - v1312;
                        let v1544 = v5 - ((v5 - (v1314 / v1540)).sqrt());
                        let v1545 = if v28 == v3 { 1.0 } else { 0.0 };
                        let v1555: f64;
                        if v1545 != 0.0 {
                            v1555 = v0;
                        } else {
                            let v1554 = ((((v1544 * v1544) * (v1544.ln())) / (v5 - v1544)) + v1544) * (v5 - (v56 * v28));
                            v1555 = v1554;
                        }
                        let v1556 = v1544 + v1555;
                        let v1561: f64;
                        if v1545 != 0.0 {
                            let v1558 = (v1540 * v51).sqrt();
                            v1561 = v1558;
                        } else {
                            let v1560 = (v1540 * v51).powf(v28);
                            v1561 = v1560;
                        }
                        let v1562 = v40 * v1561;
                        let v1565 = v159 * ((v1337 - v5) * v1562);
                        let v1567 = v736 * (v1565 * v1556);
                        v1568 = v1562;
                        v1571 = v1540;
                        v1594 = v1556;
                        v1677 = v1565;
                        v1749 = v1567;
                    }
                    let v1751: f64;
                    if v1538 != 0.0 {
                        v1751 = v0;
                    } else {
                        let v1574 = v251 * ((v1568 * v29) / v1571);
                        let v1576 = (v546 * v228) / v1574;
                        let v1577 = v1576 * v1576;
                        let v1578 = v1577 * v1577;
                        let v1581 = (v1578 / (v1578 + v5)).sqrt();
                        let v1582 = v1581.sqrt();
                        let v1583 = v1581 * v1582;
                        let v1585 = (-v28) * v33;
                        let v1587 = if v1585 == v1586 { 1.0 } else { 0.0 };
                        let v1596: f64;
                        if v1587 != 0.0 {
                            let v1590 = v5 / (v5 + (v1574 * v1583));
                            v1596 = v1590;
                        } else {
                            let v1593 = (v5 + (v1574 * v1583)).powf(v1585);
                            v1596 = v1593;
                        }
                        let v1599 = (v1594 * v1596) / (v1594 + v1596);
                        let v1602 = (v571 * (v1574 / v1582)).sqrt();
                        let v1612 = (((v228 * v1576) * v1582) - (v228 * v1581)) + (v3 * (v1574 * v1583));
                        let v1614 = (((v56 * (v1576 * v1582)) - v1581) - v5) * v1602;
                        let v1615 = v1614 * v1614;
                        let v1616 = if v1614 > v0 { 1.0 } else { 0.0 };
                        let v1642: f64;
                        if v1616 != 0.0 {
                            let v1619 = v5 / (v5 + (v55 * v1614));
                            v1642 = v1619;
                        } else {
                            let v1622 = v5 / (v5 - (v55 * v1614));
                            v1642 = v1622;
                        }
                        let v1624 = (-v1615) + v1612;
                        let v1626 = if v1624 > v1625 { 1.0 } else { 0.0 };
                        let v1650: f64;
                        if v1626 != 0.0 {
                            let v1627 = v1624.exp();
                            v1650 = v1627;
                        } else {
                            let v1641 = v327 / (v5 + ((v1628 - v1624) * (v5 + (v3 * ((v1630 - v1624) * (v5 + ((v1632 - v1624) * v334)))))));
                            v1650 = v1641;
                        }
                        let v1644 = v1642 * v1642;
                        let v1651 = (((v54 * v1642) + (v58 * v1644)) + (v59 * (v1644 * v1642))) * v1650;
                        let v1673: f64;
                        if v1616 != 0.0 {
                            v1673 = v1651;
                        } else {
                            let v1653 = if v1612 > v1652 { 1.0 } else { 0.0 };
                            let v1669: f64;
                            if v1653 != 0.0 {
                                let v1654 = v1612.exp();
                                v1669 = v1654;
                            } else {
                                let v1668 = v327 / (v5 + ((v1655 - v1612) * (v5 + (v3 * ((v1657 - v1612) * (v5 + ((v1659 - v1612) * v334)))))));
                                v1669 = v1668;
                            }
                            let v1671 = (v56 * v1669) - v1651;
                            v1673 = v1671;
                        }
                        let v1681 = v738 * ((v1677 * (v1672 * ((v228 * v1673) / v1602))) * v1599);
                        v1751 = v1681;
                    }
                    let v1682 = if v883 == v0 { 1.0 } else { 0.0 };
                    let v1753: f64;
                    if v1682 != 0.0 {
                        v1753 = v0;
                    } else {
                        let v1683 = if v28 == v3 { 1.0 } else { 0.0 };
                        let v1692: f64;
                        if v1683 != 0.0 {
                            let v1686 = ((v50 - v1459) * v51).sqrt();
                            v1692 = v1686;
                        } else {
                            let v1689 = ((v50 - v1459) * v51).powf(v28);
                            v1692 = v1689;
                        }
                        let v1694 = v33 * (((v50 - v1459) * v46) / v1692);
                        let v1696 = (-v281) / v1694;
                        let v1698 = if (v1696.abs()) < v323 { 1.0 } else { 0.0 };
                        let v1726: f64;
                        if v1698 != 0.0 {
                            let v1699 = v1696.exp();
                            v1726 = v1699;
                        } else {
                            let v1700 = if v1696 < v0 { 1.0 } else { 0.0 };
                            let v1727: f64;
                            if v1700 != 0.0 {
                                let v1714 = v327 / (v5 + ((v1701 - v1696) * (v5 + (v3 * ((v1703 - v1696) * (v5 + ((v1705 - v1696) * v334)))))));
                                v1727 = v1714;
                            } else {
                                let v1715 = v1696 - v323;
                                let v1723 = v343 * (v5 + (v1715 * (v5 + (v3 * (v1715 * (v5 + (v1715 * v334)))))));
                                v1727 = v1723;
                            }
                            v1726 = v1727;
                        }
                        let v1729 = v883 * (((v380 * v1694) * v1694) * v1726);
                        v1753 = v1729;
                    }
                    let v1730 = if v77 > v703 { 1.0 } else { 0.0 };
                    let v1756: f64;
                    if v1730 != 0.0 {
                        v1756 = v5;
                    } else {
                        let v1733 = if v1507 > ((-v62) * v77) { 1.0 } else { 0.0 };
                        let v1757: f64;
                        if v1733 != 0.0 {
                            let v1734 = if v67 == v387 { 1.0 } else { 0.0 };
                            let v1742: f64;
                            if v1734 != 0.0 {
                                let v1735 = v1507 * v78;
                                let v1738 = ((v1735 * v1735) * v1735) * v1735;
                                v1742 = v1738;
                            } else {
                                let v1741 = ((v1507 * v78).abs()).powf(v67);
                                v1742 = v1741;
                            }
                            let v1744 = v5 / (v5 - v1742);
                            v1757 = v1744;
                        } else {
                            let v1748 = v70 + ((v1507 + (v62 * v77)) * v94);
                            v1757 = v1748;
                        }
                        v1756 = v1757;
                    }
                    let v1758 = (v724 * (((v1536 + v1749) + v1751) + v1753)) * v1756;
                    v1792 = v1568;
                    v1795 = v1571;
                    v1818 = v1594;
                    v1901 = v1677;
                    v1984 = v1758;
                }
                let v1987: f64;
                let v2139: f64;
                let v2142: f64;
                let v2165: f64;
                let v2248: f64;
                if v359 != 0.0 {
                    v1987 = v0;
                    v2139 = v1792;
                    v2142 = v1795;
                    v2165 = v1818;
                    v2248 = v1901;
                } else {
                    let v1759 = v174 * v1307;
                    let v1761 = if v964 == v0 { 1.0 } else { 0.0 };
                    let v1762 = if (if v962 == v0 { 1.0 } else { 0.0 }) != 0.0 && v1761 != 0.0 { 1.0 } else { 0.0 };
                    let v1791: f64;
                    let v1794: f64;
                    let v1817: f64;
                    let v1900: f64;
                    let v1972: f64;
                    if v1762 != 0.0 {
                        v1791 = v1792;
                        v1794 = v1795;
                        v1817 = v1818;
                        v1900 = v1901;
                        v1972 = v0;
                    } else {
                        let v1763 = v208 - v1312;
                        let v1767 = v5 - ((v5 - (v1314 / v1763)).sqrt());
                        let v1768 = if v30 == v3 { 1.0 } else { 0.0 };
                        let v1778: f64;
                        if v1768 != 0.0 {
                            v1778 = v0;
                        } else {
                            let v1777 = ((((v1767 * v1767) * (v1767.ln())) / (v5 - v1767)) + v1767) * (v5 - (v56 * v30));
                            v1778 = v1777;
                        }
                        let v1779 = v1767 + v1778;
                        let v1784: f64;
                        if v1768 != 0.0 {
                            let v1781 = (v1763 * v53).sqrt();
                            v1784 = v1781;
                        } else {
                            let v1783 = (v1763 * v53).powf(v30);
                            v1784 = v1783;
                        }
                        let v1785 = v44 * v1784;
                        let v1788 = v165 * ((v1337 - v5) * v1785);
                        let v1790 = v962 * (v1788 * v1779);
                        v1791 = v1785;
                        v1794 = v1763;
                        v1817 = v1779;
                        v1900 = v1788;
                        v1972 = v1790;
                    }
                    let v1974: f64;
                    if v1761 != 0.0 {
                        v1974 = v0;
                    } else {
                        let v1797 = v261 * ((v1791 * v31) / v1794);
                        let v1799 = (v546 * v229) / v1797;
                        let v1800 = v1799 * v1799;
                        let v1801 = v1800 * v1800;
                        let v1804 = (v1801 / (v1801 + v5)).sqrt();
                        let v1805 = v1804.sqrt();
                        let v1806 = v1804 * v1805;
                        let v1808 = (-v30) * v34;
                        let v1810 = if v1808 == v1809 { 1.0 } else { 0.0 };
                        let v1819: f64;
                        if v1810 != 0.0 {
                            let v1813 = v5 / (v5 + (v1797 * v1806));
                            v1819 = v1813;
                        } else {
                            let v1816 = (v5 + (v1797 * v1806)).powf(v1808);
                            v1819 = v1816;
                        }
                        let v1822 = (v1817 * v1819) / (v1817 + v1819);
                        let v1825 = (v571 * (v1797 / v1805)).sqrt();
                        let v1835 = (((v229 * v1799) * v1805) - (v229 * v1804)) + (v3 * (v1797 * v1806));
                        let v1837 = (((v56 * (v1799 * v1805)) - v1804) - v5) * v1825;
                        let v1838 = v1837 * v1837;
                        let v1839 = if v1837 > v0 { 1.0 } else { 0.0 };
                        let v1865: f64;
                        if v1839 != 0.0 {
                            let v1842 = v5 / (v5 + (v55 * v1837));
                            v1865 = v1842;
                        } else {
                            let v1845 = v5 / (v5 - (v55 * v1837));
                            v1865 = v1845;
                        }
                        let v1847 = (-v1838) + v1835;
                        let v1849 = if v1847 > v1848 { 1.0 } else { 0.0 };
                        let v1873: f64;
                        if v1849 != 0.0 {
                            let v1850 = v1847.exp();
                            v1873 = v1850;
                        } else {
                            let v1864 = v327 / (v5 + ((v1851 - v1847) * (v5 + (v3 * ((v1853 - v1847) * (v5 + ((v1855 - v1847) * v334)))))));
                            v1873 = v1864;
                        }
                        let v1867 = v1865 * v1865;
                        let v1874 = (((v54 * v1865) + (v58 * v1867)) + (v59 * (v1867 * v1865))) * v1873;
                        let v1896: f64;
                        if v1839 != 0.0 {
                            v1896 = v1874;
                        } else {
                            let v1876 = if v1835 > v1875 { 1.0 } else { 0.0 };
                            let v1892: f64;
                            if v1876 != 0.0 {
                                let v1877 = v1835.exp();
                                v1892 = v1877;
                            } else {
                                let v1891 = v327 / (v5 + ((v1878 - v1835) * (v5 + (v3 * ((v1880 - v1835) * (v5 + ((v1882 - v1835) * v334)))))));
                                v1892 = v1891;
                            }
                            let v1894 = (v56 * v1892) - v1874;
                            v1896 = v1894;
                        }
                        let v1904 = v964 * ((v1900 * (v1895 * ((v229 * v1896) / v1825))) * v1822);
                        v1974 = v1904;
                    }
                    let v1905 = if v1109 == v0 { 1.0 } else { 0.0 };
                    let v1976: f64;
                    if v1905 != 0.0 {
                        v1976 = v0;
                    } else {
                        let v1906 = if v30 == v3 { 1.0 } else { 0.0 };
                        let v1915: f64;
                        if v1906 != 0.0 {
                            let v1909 = ((v52 - v1459) * v53).sqrt();
                            v1915 = v1909;
                        } else {
                            let v1912 = ((v52 - v1459) * v53).powf(v30);
                            v1915 = v1912;
                        }
                        let v1917 = v34 * (((v52 - v1459) * v47) / v1915);
                        let v1919 = (-v283) / v1917;
                        let v1921 = if (v1919.abs()) < v323 { 1.0 } else { 0.0 };
                        let v1949: f64;
                        if v1921 != 0.0 {
                            let v1922 = v1919.exp();
                            v1949 = v1922;
                        } else {
                            let v1923 = if v1919 < v0 { 1.0 } else { 0.0 };
                            let v1950: f64;
                            if v1923 != 0.0 {
                                let v1937 = v327 / (v5 + ((v1924 - v1919) * (v5 + (v3 * ((v1926 - v1919) * (v5 + ((v1928 - v1919) * v334)))))));
                                v1950 = v1937;
                            } else {
                                let v1938 = v1919 - v323;
                                let v1946 = v343 * (v5 + (v1938 * (v5 + (v3 * (v1938 * (v5 + (v1938 * v334)))))));
                                v1950 = v1946;
                            }
                            v1949 = v1950;
                        }
                        let v1952 = v1109 * (((v380 * v1917) * v1917) * v1949);
                        v1976 = v1952;
                    }
                    let v1953 = if v79 > v703 { 1.0 } else { 0.0 };
                    let v1979: f64;
                    if v1953 != 0.0 {
                        v1979 = v5;
                    } else {
                        let v1956 = if v1507 > ((-v62) * v79) { 1.0 } else { 0.0 };
                        let v1980: f64;
                        if v1956 != 0.0 {
                            let v1957 = if v71 == v387 { 1.0 } else { 0.0 };
                            let v1965: f64;
                            if v1957 != 0.0 {
                                let v1958 = v1507 * v80;
                                let v1961 = ((v1958 * v1958) * v1958) * v1958;
                                v1965 = v1961;
                            } else {
                                let v1964 = ((v1507 * v80).abs()).powf(v71);
                                v1965 = v1964;
                            }
                            let v1967 = v5 / (v5 - v1965);
                            v1980 = v1967;
                        } else {
                            let v1971 = v74 + ((v1507 + (v62 * v79)) * v101);
                            v1980 = v1971;
                        }
                        v1979 = v1980;
                    }
                    let v1981 = (v724 * (((v1759 + v1972) + v1974) + v1976)) * v1979;
                    v1987 = v1981;
                    v2139 = v1791;
                    v2142 = v1794;
                    v2165 = v1817;
                    v2248 = v1900;
                }
                let v1989 = ((v286 * v1982) + (v289 * v1984)) + (v292 * v1987);
                let v2102: f64;
                let v2107: f64;
                let v2109: f64;
                let v2132: f64;
                let v2254: f64;
                let v2302: f64;
                if v386 != 0.0 {
                    let v1990 = if v382 < v320 { 1.0 } else { 0.0 };
                    let v2049: f64;
                    let v2052: f64;
                    let v2063: f64;
                    if v1990 != 0.0 {
                        let v1992 = v382 * v138;
                        let v1995 = if ((v1991 * v1992).abs()) < v323 { 1.0 } else { 0.0 };
                        let v2039: f64;
                        if v1995 != 0.0 {
                            let v1998 = (v1996 * v1992).exp();
                            v2039 = v1998;
                        } else {
                            let v2001 = if (v1999 * v1992) < v0 { 1.0 } else { 0.0 };
                            let v2040: f64;
                            if v2001 != 0.0 {
                                let v2021 = v327 / (v5 + ((v2002 - (v2003 * v1992)) * (v5 + (v3 * ((v2006 - (v2007 * v1992)) * (v5 + ((v2010 - (v2011 * v1992)) * v334)))))));
                                v2040 = v2021;
                            } else {
                                let v2038 = v343 * (v5 + (((v2022 * v1992) - v323) * (v5 + (v3 * (((v2025 * v1992) - v323) * (v5 + (((v2028 * v1992) - v323) * v334)))))));
                                v2040 = v2038;
                            }
                            v2039 = v2040;
                        }
                        let v2041 = v5 / v2039;
                        let v2042 = v2041 * v2041;
                        v2049 = v2042;
                        v2052 = v2039;
                        v2063 = v2041;
                    } else {
                        let v2046 = (v5 + ((v382 - v320) * v138)) * v444;
                        let v2047 = v2046.sqrt();
                        let v2048 = v5 / v2047;
                        v2049 = v2046;
                        v2052 = v2048;
                        v2063 = v2047;
                    }
                    let v2050 = v2049 - v5;
                    let v2051 = if v382 > v0 { 1.0 } else { 0.0 };
                    let v2076: f64;
                    if v2051 != 0.0 {
                        let v2061 = v56 * (v137 * (((v56 + v2052) + (((v2052 + v5) * (v2052 + v57)).sqrt())).ln()));
                        v2076 = v2061;
                    } else {
                        let v2075 = (-v382) + (v56 * (v137 * ((((v56 * v2063) + v5) + (((v5 + v2063) * (v5 + (v57 * v2063))).sqrt())).ln())));
                        v2076 = v2075;
                    }
                    let v2077 = v366 - v2076;
                    let v2079 = v382 - v2077;
                    let v2086 = v3 * ((v382 + v2077) - (((v2079 * v2079) + ((v387 * v137) * v137)).sqrt()));
                    let v2088 = v382 - v373;
                    let v2095 = v3 * ((v382 + v373) - (((v2088 * v2088) + ((v387 * v11) * v11)).sqrt()));
                    let v2101 = v3 * (v382 - (((v382 * v382) + v2097).sqrt()));
                    v2102 = v2050;
                    v2107 = v2086;
                    v2109 = v2076;
                    v2132 = v2063;
                    v2254 = v2095;
                    v2302 = v2101;
                } else {
                    v2102 = v1307;
                    v2107 = v1312;
                    v2109 = v0;
                    v2132 = v1337;
                    v2254 = v0;
                    v2302 = v1507;
                }
                let v2364: f64;
                let v2367: f64;
                let v2390: f64;
                let v2473: f64;
                let v2777: f64;
                if v353 != 0.0 {
                    v2364 = v2139;
                    v2367 = v2142;
                    v2390 = v2165;
                    v2473 = v2248;
                    v2777 = v0;
                } else {
                    let v2103 = v168 * v2102;
                    let v2105 = if v507 == v0 { 1.0 } else { 0.0 };
                    let v2106 = if (if v505 == v0 { 1.0 } else { 0.0 }) != 0.0 && v2105 != 0.0 { 1.0 } else { 0.0 };
                    let v2138: f64;
                    let v2141: f64;
                    let v2164: f64;
                    let v2247: f64;
                    let v2321: f64;
                    if v2106 != 0.0 {
                        v2138 = v2139;
                        v2141 = v2142;
                        v2164 = v2165;
                        v2247 = v2248;
                        v2321 = v0;
                    } else {
                        let v2108 = v194 - v2107;
                        let v2113 = v5 - ((v5 - (v2109 / v2108)).sqrt());
                        let v2114 = if v26 == v3 { 1.0 } else { 0.0 };
                        let v2124: f64;
                        if v2114 != 0.0 {
                            v2124 = v0;
                        } else {
                            let v2123 = ((((v2113 * v2113) * (v2113.ln())) / (v5 - v2113)) + v2113) * (v5 - (v56 * v26));
                            v2124 = v2123;
                        }
                        let v2125 = v2113 + v2124;
                        let v2130: f64;
                        if v2114 != 0.0 {
                            let v2127 = (v2108 * v49).sqrt();
                            v2130 = v2127;
                        } else {
                            let v2129 = (v2108 * v49).powf(v26);
                            v2130 = v2129;
                        }
                        let v2131 = v36 * v2130;
                        let v2135 = v153 * ((v2132 - v5) * v2131);
                        let v2137 = v505 * (v2135 * v2125);
                        v2138 = v2131;
                        v2141 = v2108;
                        v2164 = v2125;
                        v2247 = v2135;
                        v2321 = v2137;
                    }
                    let v2323: f64;
                    if v2105 != 0.0 {
                        v2323 = v0;
                    } else {
                        let v2144 = v241 * ((v2138 * v27) / v2141);
                        let v2146 = (v546 * v227) / v2144;
                        let v2147 = v2146 * v2146;
                        let v2148 = v2147 * v2147;
                        let v2151 = (v2148 / (v2148 + v5)).sqrt();
                        let v2152 = v2151.sqrt();
                        let v2153 = v2151 * v2152;
                        let v2155 = (-v26) * v32;
                        let v2157 = if v2155 == v2156 { 1.0 } else { 0.0 };
                        let v2166: f64;
                        if v2157 != 0.0 {
                            let v2160 = v5 / (v5 + (v2144 * v2153));
                            v2166 = v2160;
                        } else {
                            let v2163 = (v5 + (v2144 * v2153)).powf(v2155);
                            v2166 = v2163;
                        }
                        let v2169 = (v2164 * v2166) / (v2164 + v2166);
                        let v2172 = (v571 * (v2144 / v2152)).sqrt();
                        let v2182 = (((v227 * v2146) * v2152) - (v227 * v2151)) + (v3 * (v2144 * v2153));
                        let v2184 = (((v56 * (v2146 * v2152)) - v2151) - v5) * v2172;
                        let v2185 = v2184 * v2184;
                        let v2186 = if v2184 > v0 { 1.0 } else { 0.0 };
                        let v2212: f64;
                        if v2186 != 0.0 {
                            let v2189 = v5 / (v5 + (v55 * v2184));
                            v2212 = v2189;
                        } else {
                            let v2192 = v5 / (v5 - (v55 * v2184));
                            v2212 = v2192;
                        }
                        let v2194 = (-v2185) + v2182;
                        let v2196 = if v2194 > v2195 { 1.0 } else { 0.0 };
                        let v2220: f64;
                        if v2196 != 0.0 {
                            let v2197 = v2194.exp();
                            v2220 = v2197;
                        } else {
                            let v2211 = v327 / (v5 + ((v2198 - v2194) * (v5 + (v3 * ((v2200 - v2194) * (v5 + ((v2202 - v2194) * v334)))))));
                            v2220 = v2211;
                        }
                        let v2214 = v2212 * v2212;
                        let v2221 = (((v54 * v2212) + (v58 * v2214)) + (v59 * (v2214 * v2212))) * v2220;
                        let v2243: f64;
                        if v2186 != 0.0 {
                            v2243 = v2221;
                        } else {
                            let v2223 = if v2182 > v2222 { 1.0 } else { 0.0 };
                            let v2239: f64;
                            if v2223 != 0.0 {
                                let v2224 = v2182.exp();
                                v2239 = v2224;
                            } else {
                                let v2238 = v327 / (v5 + ((v2225 - v2182) * (v5 + (v3 * ((v2227 - v2182) * (v5 + ((v2229 - v2182) * v334)))))));
                                v2239 = v2238;
                            }
                            let v2241 = (v56 * v2239) - v2221;
                            v2243 = v2241;
                        }
                        let v2251 = v507 * ((v2247 * (v2242 * ((v227 * v2243) / v2172))) * v2169);
                        v2323 = v2251;
                    }
                    let v2252 = if v653 == v0 { 1.0 } else { 0.0 };
                    let v2325: f64;
                    if v2252 != 0.0 {
                        v2325 = v0;
                    } else {
                        let v2253 = if v26 == v3 { 1.0 } else { 0.0 };
                        let v2263: f64;
                        if v2253 != 0.0 {
                            let v2257 = ((v48 - v2254) * v49).sqrt();
                            v2263 = v2257;
                        } else {
                            let v2260 = ((v48 - v2254) * v49).powf(v26);
                            v2263 = v2260;
                        }
                        let v2265 = v32 * (((v48 - v2254) * v45) / v2263);
                        let v2267 = (-v279) / v2265;
                        let v2269 = if (v2267.abs()) < v323 { 1.0 } else { 0.0 };
                        let v2297: f64;
                        if v2269 != 0.0 {
                            let v2270 = v2267.exp();
                            v2297 = v2270;
                        } else {
                            let v2271 = if v2267 < v0 { 1.0 } else { 0.0 };
                            let v2298: f64;
                            if v2271 != 0.0 {
                                let v2285 = v327 / (v5 + ((v2272 - v2267) * (v5 + (v3 * ((v2274 - v2267) * (v5 + ((v2276 - v2267) * v334)))))));
                                v2298 = v2285;
                            } else {
                                let v2286 = v2267 - v323;
                                let v2294 = v343 * (v5 + (v2286 * (v5 + (v3 * (v2286 * (v5 + (v2286 * v334)))))));
                                v2298 = v2294;
                            }
                            v2297 = v2298;
                        }
                        let v2300 = v653 * (((v382 * v2265) * v2265) * v2297);
                        v2325 = v2300;
                    }
                    let v2301 = if v75 > v703 { 1.0 } else { 0.0 };
                    let v2328: f64;
                    if v2301 != 0.0 {
                        v2328 = v5;
                    } else {
                        let v2305 = if v2302 > ((-v62) * v75) { 1.0 } else { 0.0 };
                        let v2329: f64;
                        if v2305 != 0.0 {
                            let v2306 = if v63 == v387 { 1.0 } else { 0.0 };
                            let v2314: f64;
                            if v2306 != 0.0 {
                                let v2307 = v2302 * v76;
                                let v2310 = ((v2307 * v2307) * v2307) * v2307;
                                v2314 = v2310;
                            } else {
                                let v2313 = ((v2302 * v76).abs()).powf(v63);
                                v2314 = v2313;
                            }
                            let v2316 = v5 / (v5 - v2314);
                            v2329 = v2316;
                        } else {
                            let v2320 = v66 + ((v2302 + (v62 * v75)) * v87);
                            v2329 = v2320;
                        }
                        v2328 = v2329;
                    }
                    let v2330 = (v724 * (((v2103 + v2321) + v2323) + v2325)) * v2328;
                    v2364 = v2138;
                    v2367 = v2141;
                    v2390 = v2164;
                    v2473 = v2247;
                    v2777 = v2330;
                }
                let v2587: f64;
                let v2590: f64;
                let v2613: f64;
                let v2696: f64;
                let v2779: f64;
                if v356 != 0.0 {
                    v2587 = v2364;
                    v2590 = v2367;
                    v2613 = v2390;
                    v2696 = v2473;
                    v2779 = v0;
                } else {
                    let v2331 = v171 * v2102;
                    let v2333 = if v738 == v0 { 1.0 } else { 0.0 };
                    let v2334 = if (if v736 == v0 { 1.0 } else { 0.0 }) != 0.0 && v2333 != 0.0 { 1.0 } else { 0.0 };
                    let v2363: f64;
                    let v2366: f64;
                    let v2389: f64;
                    let v2472: f64;
                    let v2544: f64;
                    if v2334 != 0.0 {
                        v2363 = v2364;
                        v2366 = v2367;
                        v2389 = v2390;
                        v2472 = v2473;
                        v2544 = v0;
                    } else {
                        let v2335 = v201 - v2107;
                        let v2339 = v5 - ((v5 - (v2109 / v2335)).sqrt());
                        let v2340 = if v28 == v3 { 1.0 } else { 0.0 };
                        let v2350: f64;
                        if v2340 != 0.0 {
                            v2350 = v0;
                        } else {
                            let v2349 = ((((v2339 * v2339) * (v2339.ln())) / (v5 - v2339)) + v2339) * (v5 - (v56 * v28));
                            v2350 = v2349;
                        }
                        let v2351 = v2339 + v2350;
                        let v2356: f64;
                        if v2340 != 0.0 {
                            let v2353 = (v2335 * v51).sqrt();
                            v2356 = v2353;
                        } else {
                            let v2355 = (v2335 * v51).powf(v28);
                            v2356 = v2355;
                        }
                        let v2357 = v40 * v2356;
                        let v2360 = v159 * ((v2132 - v5) * v2357);
                        let v2362 = v736 * (v2360 * v2351);
                        v2363 = v2357;
                        v2366 = v2335;
                        v2389 = v2351;
                        v2472 = v2360;
                        v2544 = v2362;
                    }
                    let v2546: f64;
                    if v2333 != 0.0 {
                        v2546 = v0;
                    } else {
                        let v2369 = v251 * ((v2363 * v29) / v2366);
                        let v2371 = (v546 * v228) / v2369;
                        let v2372 = v2371 * v2371;
                        let v2373 = v2372 * v2372;
                        let v2376 = (v2373 / (v2373 + v5)).sqrt();
                        let v2377 = v2376.sqrt();
                        let v2378 = v2376 * v2377;
                        let v2380 = (-v28) * v33;
                        let v2382 = if v2380 == v2381 { 1.0 } else { 0.0 };
                        let v2391: f64;
                        if v2382 != 0.0 {
                            let v2385 = v5 / (v5 + (v2369 * v2378));
                            v2391 = v2385;
                        } else {
                            let v2388 = (v5 + (v2369 * v2378)).powf(v2380);
                            v2391 = v2388;
                        }
                        let v2394 = (v2389 * v2391) / (v2389 + v2391);
                        let v2397 = (v571 * (v2369 / v2377)).sqrt();
                        let v2407 = (((v228 * v2371) * v2377) - (v228 * v2376)) + (v3 * (v2369 * v2378));
                        let v2409 = (((v56 * (v2371 * v2377)) - v2376) - v5) * v2397;
                        let v2410 = v2409 * v2409;
                        let v2411 = if v2409 > v0 { 1.0 } else { 0.0 };
                        let v2437: f64;
                        if v2411 != 0.0 {
                            let v2414 = v5 / (v5 + (v55 * v2409));
                            v2437 = v2414;
                        } else {
                            let v2417 = v5 / (v5 - (v55 * v2409));
                            v2437 = v2417;
                        }
                        let v2419 = (-v2410) + v2407;
                        let v2421 = if v2419 > v2420 { 1.0 } else { 0.0 };
                        let v2445: f64;
                        if v2421 != 0.0 {
                            let v2422 = v2419.exp();
                            v2445 = v2422;
                        } else {
                            let v2436 = v327 / (v5 + ((v2423 - v2419) * (v5 + (v3 * ((v2425 - v2419) * (v5 + ((v2427 - v2419) * v334)))))));
                            v2445 = v2436;
                        }
                        let v2439 = v2437 * v2437;
                        let v2446 = (((v54 * v2437) + (v58 * v2439)) + (v59 * (v2439 * v2437))) * v2445;
                        let v2468: f64;
                        if v2411 != 0.0 {
                            v2468 = v2446;
                        } else {
                            let v2448 = if v2407 > v2447 { 1.0 } else { 0.0 };
                            let v2464: f64;
                            if v2448 != 0.0 {
                                let v2449 = v2407.exp();
                                v2464 = v2449;
                            } else {
                                let v2463 = v327 / (v5 + ((v2450 - v2407) * (v5 + (v3 * ((v2452 - v2407) * (v5 + ((v2454 - v2407) * v334)))))));
                                v2464 = v2463;
                            }
                            let v2466 = (v56 * v2464) - v2446;
                            v2468 = v2466;
                        }
                        let v2476 = v738 * ((v2472 * (v2467 * ((v228 * v2468) / v2397))) * v2394);
                        v2546 = v2476;
                    }
                    let v2477 = if v883 == v0 { 1.0 } else { 0.0 };
                    let v2548: f64;
                    if v2477 != 0.0 {
                        v2548 = v0;
                    } else {
                        let v2478 = if v28 == v3 { 1.0 } else { 0.0 };
                        let v2487: f64;
                        if v2478 != 0.0 {
                            let v2481 = ((v50 - v2254) * v51).sqrt();
                            v2487 = v2481;
                        } else {
                            let v2484 = ((v50 - v2254) * v51).powf(v28);
                            v2487 = v2484;
                        }
                        let v2489 = v33 * (((v50 - v2254) * v46) / v2487);
                        let v2491 = (-v281) / v2489;
                        let v2493 = if (v2491.abs()) < v323 { 1.0 } else { 0.0 };
                        let v2521: f64;
                        if v2493 != 0.0 {
                            let v2494 = v2491.exp();
                            v2521 = v2494;
                        } else {
                            let v2495 = if v2491 < v0 { 1.0 } else { 0.0 };
                            let v2522: f64;
                            if v2495 != 0.0 {
                                let v2509 = v327 / (v5 + ((v2496 - v2491) * (v5 + (v3 * ((v2498 - v2491) * (v5 + ((v2500 - v2491) * v334)))))));
                                v2522 = v2509;
                            } else {
                                let v2510 = v2491 - v323;
                                let v2518 = v343 * (v5 + (v2510 * (v5 + (v3 * (v2510 * (v5 + (v2510 * v334)))))));
                                v2522 = v2518;
                            }
                            v2521 = v2522;
                        }
                        let v2524 = v883 * (((v382 * v2489) * v2489) * v2521);
                        v2548 = v2524;
                    }
                    let v2525 = if v77 > v703 { 1.0 } else { 0.0 };
                    let v2551: f64;
                    if v2525 != 0.0 {
                        v2551 = v5;
                    } else {
                        let v2528 = if v2302 > ((-v62) * v77) { 1.0 } else { 0.0 };
                        let v2552: f64;
                        if v2528 != 0.0 {
                            let v2529 = if v67 == v387 { 1.0 } else { 0.0 };
                            let v2537: f64;
                            if v2529 != 0.0 {
                                let v2530 = v2302 * v78;
                                let v2533 = ((v2530 * v2530) * v2530) * v2530;
                                v2537 = v2533;
                            } else {
                                let v2536 = ((v2302 * v78).abs()).powf(v67);
                                v2537 = v2536;
                            }
                            let v2539 = v5 / (v5 - v2537);
                            v2552 = v2539;
                        } else {
                            let v2543 = v70 + ((v2302 + (v62 * v77)) * v94);
                            v2552 = v2543;
                        }
                        v2551 = v2552;
                    }
                    let v2553 = (v724 * (((v2331 + v2544) + v2546) + v2548)) * v2551;
                    v2587 = v2363;
                    v2590 = v2366;
                    v2613 = v2389;
                    v2696 = v2472;
                    v2779 = v2553;
                }
                let v2782: f64;
                let v2929: f64;
                let v2932: f64;
                let v2955: f64;
                let v3038: f64;
                if v359 != 0.0 {
                    v2782 = v0;
                    v2929 = v2587;
                    v2932 = v2590;
                    v2955 = v2613;
                    v3038 = v2696;
                } else {
                    let v2554 = v174 * v2102;
                    let v2556 = if v964 == v0 { 1.0 } else { 0.0 };
                    let v2557 = if (if v962 == v0 { 1.0 } else { 0.0 }) != 0.0 && v2556 != 0.0 { 1.0 } else { 0.0 };
                    let v2586: f64;
                    let v2589: f64;
                    let v2612: f64;
                    let v2695: f64;
                    let v2767: f64;
                    if v2557 != 0.0 {
                        v2586 = v2587;
                        v2589 = v2590;
                        v2612 = v2613;
                        v2695 = v2696;
                        v2767 = v0;
                    } else {
                        let v2558 = v208 - v2107;
                        let v2562 = v5 - ((v5 - (v2109 / v2558)).sqrt());
                        let v2563 = if v30 == v3 { 1.0 } else { 0.0 };
                        let v2573: f64;
                        if v2563 != 0.0 {
                            v2573 = v0;
                        } else {
                            let v2572 = ((((v2562 * v2562) * (v2562.ln())) / (v5 - v2562)) + v2562) * (v5 - (v56 * v30));
                            v2573 = v2572;
                        }
                        let v2574 = v2562 + v2573;
                        let v2579: f64;
                        if v2563 != 0.0 {
                            let v2576 = (v2558 * v53).sqrt();
                            v2579 = v2576;
                        } else {
                            let v2578 = (v2558 * v53).powf(v30);
                            v2579 = v2578;
                        }
                        let v2580 = v44 * v2579;
                        let v2583 = v165 * ((v2132 - v5) * v2580);
                        let v2585 = v962 * (v2583 * v2574);
                        v2586 = v2580;
                        v2589 = v2558;
                        v2612 = v2574;
                        v2695 = v2583;
                        v2767 = v2585;
                    }
                    let v2769: f64;
                    if v2556 != 0.0 {
                        v2769 = v0;
                    } else {
                        let v2592 = v261 * ((v2586 * v31) / v2589);
                        let v2594 = (v546 * v229) / v2592;
                        let v2595 = v2594 * v2594;
                        let v2596 = v2595 * v2595;
                        let v2599 = (v2596 / (v2596 + v5)).sqrt();
                        let v2600 = v2599.sqrt();
                        let v2601 = v2599 * v2600;
                        let v2603 = (-v30) * v34;
                        let v2605 = if v2603 == v2604 { 1.0 } else { 0.0 };
                        let v2614: f64;
                        if v2605 != 0.0 {
                            let v2608 = v5 / (v5 + (v2592 * v2601));
                            v2614 = v2608;
                        } else {
                            let v2611 = (v5 + (v2592 * v2601)).powf(v2603);
                            v2614 = v2611;
                        }
                        let v2617 = (v2612 * v2614) / (v2612 + v2614);
                        let v2620 = (v571 * (v2592 / v2600)).sqrt();
                        let v2630 = (((v229 * v2594) * v2600) - (v229 * v2599)) + (v3 * (v2592 * v2601));
                        let v2632 = (((v56 * (v2594 * v2600)) - v2599) - v5) * v2620;
                        let v2633 = v2632 * v2632;
                        let v2634 = if v2632 > v0 { 1.0 } else { 0.0 };
                        let v2660: f64;
                        if v2634 != 0.0 {
                            let v2637 = v5 / (v5 + (v55 * v2632));
                            v2660 = v2637;
                        } else {
                            let v2640 = v5 / (v5 - (v55 * v2632));
                            v2660 = v2640;
                        }
                        let v2642 = (-v2633) + v2630;
                        let v2644 = if v2642 > v2643 { 1.0 } else { 0.0 };
                        let v2668: f64;
                        if v2644 != 0.0 {
                            let v2645 = v2642.exp();
                            v2668 = v2645;
                        } else {
                            let v2659 = v327 / (v5 + ((v2646 - v2642) * (v5 + (v3 * ((v2648 - v2642) * (v5 + ((v2650 - v2642) * v334)))))));
                            v2668 = v2659;
                        }
                        let v2662 = v2660 * v2660;
                        let v2669 = (((v54 * v2660) + (v58 * v2662)) + (v59 * (v2662 * v2660))) * v2668;
                        let v2691: f64;
                        if v2634 != 0.0 {
                            v2691 = v2669;
                        } else {
                            let v2671 = if v2630 > v2670 { 1.0 } else { 0.0 };
                            let v2687: f64;
                            if v2671 != 0.0 {
                                let v2672 = v2630.exp();
                                v2687 = v2672;
                            } else {
                                let v2686 = v327 / (v5 + ((v2673 - v2630) * (v5 + (v3 * ((v2675 - v2630) * (v5 + ((v2677 - v2630) * v334)))))));
                                v2687 = v2686;
                            }
                            let v2689 = (v56 * v2687) - v2669;
                            v2691 = v2689;
                        }
                        let v2699 = v964 * ((v2695 * (v2690 * ((v229 * v2691) / v2620))) * v2617);
                        v2769 = v2699;
                    }
                    let v2700 = if v1109 == v0 { 1.0 } else { 0.0 };
                    let v2771: f64;
                    if v2700 != 0.0 {
                        v2771 = v0;
                    } else {
                        let v2701 = if v30 == v3 { 1.0 } else { 0.0 };
                        let v2710: f64;
                        if v2701 != 0.0 {
                            let v2704 = ((v52 - v2254) * v53).sqrt();
                            v2710 = v2704;
                        } else {
                            let v2707 = ((v52 - v2254) * v53).powf(v30);
                            v2710 = v2707;
                        }
                        let v2712 = v34 * (((v52 - v2254) * v47) / v2710);
                        let v2714 = (-v283) / v2712;
                        let v2716 = if (v2714.abs()) < v323 { 1.0 } else { 0.0 };
                        let v2744: f64;
                        if v2716 != 0.0 {
                            let v2717 = v2714.exp();
                            v2744 = v2717;
                        } else {
                            let v2718 = if v2714 < v0 { 1.0 } else { 0.0 };
                            let v2745: f64;
                            if v2718 != 0.0 {
                                let v2732 = v327 / (v5 + ((v2719 - v2714) * (v5 + (v3 * ((v2721 - v2714) * (v5 + ((v2723 - v2714) * v334)))))));
                                v2745 = v2732;
                            } else {
                                let v2733 = v2714 - v323;
                                let v2741 = v343 * (v5 + (v2733 * (v5 + (v3 * (v2733 * (v5 + (v2733 * v334)))))));
                                v2745 = v2741;
                            }
                            v2744 = v2745;
                        }
                        let v2747 = v1109 * (((v382 * v2712) * v2712) * v2744);
                        v2771 = v2747;
                    }
                    let v2748 = if v79 > v703 { 1.0 } else { 0.0 };
                    let v2774: f64;
                    if v2748 != 0.0 {
                        v2774 = v5;
                    } else {
                        let v2751 = if v2302 > ((-v62) * v79) { 1.0 } else { 0.0 };
                        let v2775: f64;
                        if v2751 != 0.0 {
                            let v2752 = if v71 == v387 { 1.0 } else { 0.0 };
                            let v2760: f64;
                            if v2752 != 0.0 {
                                let v2753 = v2302 * v80;
                                let v2756 = ((v2753 * v2753) * v2753) * v2753;
                                v2760 = v2756;
                            } else {
                                let v2759 = ((v2302 * v80).abs()).powf(v71);
                                v2760 = v2759;
                            }
                            let v2762 = v5 / (v5 - v2760);
                            v2775 = v2762;
                        } else {
                            let v2766 = v74 + ((v2302 + (v62 * v79)) * v101);
                            v2775 = v2766;
                        }
                        v2774 = v2775;
                    }
                    let v2776 = (v724 * (((v2554 + v2767) + v2769) + v2771)) * v2774;
                    v2782 = v2776;
                    v2929 = v2586;
                    v2932 = v2589;
                    v2955 = v2612;
                    v3038 = v2695;
                }
                let v2784 = ((v286 * v2777) + (v289 * v2779)) + (v292 * v2782);
                let v2892: f64;
                let v2897: f64;
                let v2899: f64;
                let v2922: f64;
                let v3044: f64;
                let v3092: f64;
                if v386 != 0.0 {
                    let v2785 = if v367 < v320 { 1.0 } else { 0.0 };
                    let v2844: f64;
                    let v2847: f64;
                    let v2858: f64;
                    if v2785 != 0.0 {
                        let v2787 = v367 * v138;
                        let v2790 = if ((v2786 * v2787).abs()) < v323 { 1.0 } else { 0.0 };
                        let v2834: f64;
                        if v2790 != 0.0 {
                            let v2793 = (v2791 * v2787).exp();
                            v2834 = v2793;
                        } else {
                            let v2796 = if (v2794 * v2787) < v0 { 1.0 } else { 0.0 };
                            let v2835: f64;
                            if v2796 != 0.0 {
                                let v2816 = v327 / (v5 + ((v2797 - (v2798 * v2787)) * (v5 + (v3 * ((v2801 - (v2802 * v2787)) * (v5 + ((v2805 - (v2806 * v2787)) * v334)))))));
                                v2835 = v2816;
                            } else {
                                let v2833 = v343 * (v5 + (((v2817 * v2787) - v323) * (v5 + (v3 * (((v2820 * v2787) - v323) * (v5 + (((v2823 * v2787) - v323) * v334)))))));
                                v2835 = v2833;
                            }
                            v2834 = v2835;
                        }
                        let v2836 = v5 / v2834;
                        let v2837 = v2836 * v2836;
                        v2844 = v2837;
                        v2847 = v2834;
                        v2858 = v2836;
                    } else {
                        let v2841 = (v5 + ((v367 - v320) * v138)) * v444;
                        let v2842 = v2841.sqrt();
                        let v2843 = v5 / v2842;
                        v2844 = v2841;
                        v2847 = v2843;
                        v2858 = v2842;
                    }
                    let v2845 = v2844 - v5;
                    let v2871: f64;
                    if v2846 != 0.0 {
                        let v2856 = v56 * (v137 * (((v56 + v2847) + (((v2847 + v5) * (v2847 + v57)).sqrt())).ln()));
                        v2871 = v2856;
                    } else {
                        let v2870 = v2857 + (v56 * (v137 * ((((v56 * v2858) + v5) + (((v5 + v2858) * (v5 + (v57 * v2858))).sqrt())).ln())));
                        v2871 = v2870;
                    }
                    let v2872 = v366 - v2871;
                    let v2874 = v367 - v2872;
                    let v2881 = v3 * ((v367 + v2872) - (((v2874 * v2874) + ((v387 * v137) * v137)).sqrt()));
                    let v2883 = v367 - v373;
                    let v2890 = v3 * ((v367 + v373) - (((v2883 * v2883) + ((v387 * v11) * v11)).sqrt()));
                    v2892 = v2845;
                    v2897 = v2881;
                    v2899 = v2871;
                    v2922 = v2858;
                    v3044 = v2890;
                    v3092 = v2891;
                } else {
                    v2892 = v2102;
                    v2897 = v2107;
                    v2899 = v0;
                    v2922 = v2132;
                    v3044 = v0;
                    v3092 = v2302;
                }
                let v3154: f64;
                let v3157: f64;
                let v3180: f64;
                let v3263: f64;
                let v3567: f64;
                if v353 != 0.0 {
                    v3154 = v2929;
                    v3157 = v2932;
                    v3180 = v2955;
                    v3263 = v3038;
                    v3567 = v0;
                } else {
                    let v2893 = v168 * v2892;
                    let v2895 = if v507 == v0 { 1.0 } else { 0.0 };
                    let v2896 = if (if v505 == v0 { 1.0 } else { 0.0 }) != 0.0 && v2895 != 0.0 { 1.0 } else { 0.0 };
                    let v2928: f64;
                    let v2931: f64;
                    let v2954: f64;
                    let v3037: f64;
                    let v3111: f64;
                    if v2896 != 0.0 {
                        v2928 = v2929;
                        v2931 = v2932;
                        v2954 = v2955;
                        v3037 = v3038;
                        v3111 = v0;
                    } else {
                        let v2898 = v194 - v2897;
                        let v2903 = v5 - ((v5 - (v2899 / v2898)).sqrt());
                        let v2904 = if v26 == v3 { 1.0 } else { 0.0 };
                        let v2914: f64;
                        if v2904 != 0.0 {
                            v2914 = v0;
                        } else {
                            let v2913 = ((((v2903 * v2903) * (v2903.ln())) / (v5 - v2903)) + v2903) * (v5 - (v56 * v26));
                            v2914 = v2913;
                        }
                        let v2915 = v2903 + v2914;
                        let v2920: f64;
                        if v2904 != 0.0 {
                            let v2917 = (v2898 * v49).sqrt();
                            v2920 = v2917;
                        } else {
                            let v2919 = (v2898 * v49).powf(v26);
                            v2920 = v2919;
                        }
                        let v2921 = v36 * v2920;
                        let v2925 = v153 * ((v2922 - v5) * v2921);
                        let v2927 = v505 * (v2925 * v2915);
                        v2928 = v2921;
                        v2931 = v2898;
                        v2954 = v2915;
                        v3037 = v2925;
                        v3111 = v2927;
                    }
                    let v3113: f64;
                    if v2895 != 0.0 {
                        v3113 = v0;
                    } else {
                        let v2934 = v241 * ((v2928 * v27) / v2931);
                        let v2936 = (v546 * v227) / v2934;
                        let v2937 = v2936 * v2936;
                        let v2938 = v2937 * v2937;
                        let v2941 = (v2938 / (v2938 + v5)).sqrt();
                        let v2942 = v2941.sqrt();
                        let v2943 = v2941 * v2942;
                        let v2945 = (-v26) * v32;
                        let v2947 = if v2945 == v2946 { 1.0 } else { 0.0 };
                        let v2956: f64;
                        if v2947 != 0.0 {
                            let v2950 = v5 / (v5 + (v2934 * v2943));
                            v2956 = v2950;
                        } else {
                            let v2953 = (v5 + (v2934 * v2943)).powf(v2945);
                            v2956 = v2953;
                        }
                        let v2959 = (v2954 * v2956) / (v2954 + v2956);
                        let v2962 = (v571 * (v2934 / v2942)).sqrt();
                        let v2972 = (((v227 * v2936) * v2942) - (v227 * v2941)) + (v3 * (v2934 * v2943));
                        let v2974 = (((v56 * (v2936 * v2942)) - v2941) - v5) * v2962;
                        let v2975 = v2974 * v2974;
                        let v2976 = if v2974 > v0 { 1.0 } else { 0.0 };
                        let v3002: f64;
                        if v2976 != 0.0 {
                            let v2979 = v5 / (v5 + (v55 * v2974));
                            v3002 = v2979;
                        } else {
                            let v2982 = v5 / (v5 - (v55 * v2974));
                            v3002 = v2982;
                        }
                        let v2984 = (-v2975) + v2972;
                        let v2986 = if v2984 > v2985 { 1.0 } else { 0.0 };
                        let v3010: f64;
                        if v2986 != 0.0 {
                            let v2987 = v2984.exp();
                            v3010 = v2987;
                        } else {
                            let v3001 = v327 / (v5 + ((v2988 - v2984) * (v5 + (v3 * ((v2990 - v2984) * (v5 + ((v2992 - v2984) * v334)))))));
                            v3010 = v3001;
                        }
                        let v3004 = v3002 * v3002;
                        let v3011 = (((v54 * v3002) + (v58 * v3004)) + (v59 * (v3004 * v3002))) * v3010;
                        let v3033: f64;
                        if v2976 != 0.0 {
                            v3033 = v3011;
                        } else {
                            let v3013 = if v2972 > v3012 { 1.0 } else { 0.0 };
                            let v3029: f64;
                            if v3013 != 0.0 {
                                let v3014 = v2972.exp();
                                v3029 = v3014;
                            } else {
                                let v3028 = v327 / (v5 + ((v3015 - v2972) * (v5 + (v3 * ((v3017 - v2972) * (v5 + ((v3019 - v2972) * v334)))))));
                                v3029 = v3028;
                            }
                            let v3031 = (v56 * v3029) - v3011;
                            v3033 = v3031;
                        }
                        let v3041 = v507 * ((v3037 * (v3032 * ((v227 * v3033) / v2962))) * v2959);
                        v3113 = v3041;
                    }
                    let v3042 = if v653 == v0 { 1.0 } else { 0.0 };
                    let v3115: f64;
                    if v3042 != 0.0 {
                        v3115 = v0;
                    } else {
                        let v3043 = if v26 == v3 { 1.0 } else { 0.0 };
                        let v3053: f64;
                        if v3043 != 0.0 {
                            let v3047 = ((v48 - v3044) * v49).sqrt();
                            v3053 = v3047;
                        } else {
                            let v3050 = ((v48 - v3044) * v49).powf(v26);
                            v3053 = v3050;
                        }
                        let v3055 = v32 * (((v48 - v3044) * v45) / v3053);
                        let v3057 = (-v279) / v3055;
                        let v3059 = if (v3057.abs()) < v323 { 1.0 } else { 0.0 };
                        let v3087: f64;
                        if v3059 != 0.0 {
                            let v3060 = v3057.exp();
                            v3087 = v3060;
                        } else {
                            let v3061 = if v3057 < v0 { 1.0 } else { 0.0 };
                            let v3088: f64;
                            if v3061 != 0.0 {
                                let v3075 = v327 / (v5 + ((v3062 - v3057) * (v5 + (v3 * ((v3064 - v3057) * (v5 + ((v3066 - v3057) * v334)))))));
                                v3088 = v3075;
                            } else {
                                let v3076 = v3057 - v323;
                                let v3084 = v343 * (v5 + (v3076 * (v5 + (v3 * (v3076 * (v5 + (v3076 * v334)))))));
                                v3088 = v3084;
                            }
                            v3087 = v3088;
                        }
                        let v3090 = v653 * (((v367 * v3055) * v3055) * v3087);
                        v3115 = v3090;
                    }
                    let v3091 = if v75 > v703 { 1.0 } else { 0.0 };
                    let v3118: f64;
                    if v3091 != 0.0 {
                        v3118 = v5;
                    } else {
                        let v3095 = if v3092 > ((-v62) * v75) { 1.0 } else { 0.0 };
                        let v3119: f64;
                        if v3095 != 0.0 {
                            let v3096 = if v63 == v387 { 1.0 } else { 0.0 };
                            let v3104: f64;
                            if v3096 != 0.0 {
                                let v3097 = v3092 * v76;
                                let v3100 = ((v3097 * v3097) * v3097) * v3097;
                                v3104 = v3100;
                            } else {
                                let v3103 = ((v3092 * v76).abs()).powf(v63);
                                v3104 = v3103;
                            }
                            let v3106 = v5 / (v5 - v3104);
                            v3119 = v3106;
                        } else {
                            let v3110 = v66 + ((v3092 + (v62 * v75)) * v87);
                            v3119 = v3110;
                        }
                        v3118 = v3119;
                    }
                    let v3120 = (v724 * (((v2893 + v3111) + v3113) + v3115)) * v3118;
                    v3154 = v2928;
                    v3157 = v2931;
                    v3180 = v2954;
                    v3263 = v3037;
                    v3567 = v3120;
                }
                let v3377: f64;
                let v3380: f64;
                let v3403: f64;
                let v3486: f64;
                let v3569: f64;
                if v356 != 0.0 {
                    v3377 = v3154;
                    v3380 = v3157;
                    v3403 = v3180;
                    v3486 = v3263;
                    v3569 = v0;
                } else {
                    let v3121 = v171 * v2892;
                    let v3123 = if v738 == v0 { 1.0 } else { 0.0 };
                    let v3124 = if (if v736 == v0 { 1.0 } else { 0.0 }) != 0.0 && v3123 != 0.0 { 1.0 } else { 0.0 };
                    let v3153: f64;
                    let v3156: f64;
                    let v3179: f64;
                    let v3262: f64;
                    let v3334: f64;
                    if v3124 != 0.0 {
                        v3153 = v3154;
                        v3156 = v3157;
                        v3179 = v3180;
                        v3262 = v3263;
                        v3334 = v0;
                    } else {
                        let v3125 = v201 - v2897;
                        let v3129 = v5 - ((v5 - (v2899 / v3125)).sqrt());
                        let v3130 = if v28 == v3 { 1.0 } else { 0.0 };
                        let v3140: f64;
                        if v3130 != 0.0 {
                            v3140 = v0;
                        } else {
                            let v3139 = ((((v3129 * v3129) * (v3129.ln())) / (v5 - v3129)) + v3129) * (v5 - (v56 * v28));
                            v3140 = v3139;
                        }
                        let v3141 = v3129 + v3140;
                        let v3146: f64;
                        if v3130 != 0.0 {
                            let v3143 = (v3125 * v51).sqrt();
                            v3146 = v3143;
                        } else {
                            let v3145 = (v3125 * v51).powf(v28);
                            v3146 = v3145;
                        }
                        let v3147 = v40 * v3146;
                        let v3150 = v159 * ((v2922 - v5) * v3147);
                        let v3152 = v736 * (v3150 * v3141);
                        v3153 = v3147;
                        v3156 = v3125;
                        v3179 = v3141;
                        v3262 = v3150;
                        v3334 = v3152;
                    }
                    let v3336: f64;
                    if v3123 != 0.0 {
                        v3336 = v0;
                    } else {
                        let v3159 = v251 * ((v3153 * v29) / v3156);
                        let v3161 = (v546 * v228) / v3159;
                        let v3162 = v3161 * v3161;
                        let v3163 = v3162 * v3162;
                        let v3166 = (v3163 / (v3163 + v5)).sqrt();
                        let v3167 = v3166.sqrt();
                        let v3168 = v3166 * v3167;
                        let v3170 = (-v28) * v33;
                        let v3172 = if v3170 == v3171 { 1.0 } else { 0.0 };
                        let v3181: f64;
                        if v3172 != 0.0 {
                            let v3175 = v5 / (v5 + (v3159 * v3168));
                            v3181 = v3175;
                        } else {
                            let v3178 = (v5 + (v3159 * v3168)).powf(v3170);
                            v3181 = v3178;
                        }
                        let v3184 = (v3179 * v3181) / (v3179 + v3181);
                        let v3187 = (v571 * (v3159 / v3167)).sqrt();
                        let v3197 = (((v228 * v3161) * v3167) - (v228 * v3166)) + (v3 * (v3159 * v3168));
                        let v3199 = (((v56 * (v3161 * v3167)) - v3166) - v5) * v3187;
                        let v3200 = v3199 * v3199;
                        let v3201 = if v3199 > v0 { 1.0 } else { 0.0 };
                        let v3227: f64;
                        if v3201 != 0.0 {
                            let v3204 = v5 / (v5 + (v55 * v3199));
                            v3227 = v3204;
                        } else {
                            let v3207 = v5 / (v5 - (v55 * v3199));
                            v3227 = v3207;
                        }
                        let v3209 = (-v3200) + v3197;
                        let v3211 = if v3209 > v3210 { 1.0 } else { 0.0 };
                        let v3235: f64;
                        if v3211 != 0.0 {
                            let v3212 = v3209.exp();
                            v3235 = v3212;
                        } else {
                            let v3226 = v327 / (v5 + ((v3213 - v3209) * (v5 + (v3 * ((v3215 - v3209) * (v5 + ((v3217 - v3209) * v334)))))));
                            v3235 = v3226;
                        }
                        let v3229 = v3227 * v3227;
                        let v3236 = (((v54 * v3227) + (v58 * v3229)) + (v59 * (v3229 * v3227))) * v3235;
                        let v3258: f64;
                        if v3201 != 0.0 {
                            v3258 = v3236;
                        } else {
                            let v3238 = if v3197 > v3237 { 1.0 } else { 0.0 };
                            let v3254: f64;
                            if v3238 != 0.0 {
                                let v3239 = v3197.exp();
                                v3254 = v3239;
                            } else {
                                let v3253 = v327 / (v5 + ((v3240 - v3197) * (v5 + (v3 * ((v3242 - v3197) * (v5 + ((v3244 - v3197) * v334)))))));
                                v3254 = v3253;
                            }
                            let v3256 = (v56 * v3254) - v3236;
                            v3258 = v3256;
                        }
                        let v3266 = v738 * ((v3262 * (v3257 * ((v228 * v3258) / v3187))) * v3184);
                        v3336 = v3266;
                    }
                    let v3267 = if v883 == v0 { 1.0 } else { 0.0 };
                    let v3338: f64;
                    if v3267 != 0.0 {
                        v3338 = v0;
                    } else {
                        let v3268 = if v28 == v3 { 1.0 } else { 0.0 };
                        let v3277: f64;
                        if v3268 != 0.0 {
                            let v3271 = ((v50 - v3044) * v51).sqrt();
                            v3277 = v3271;
                        } else {
                            let v3274 = ((v50 - v3044) * v51).powf(v28);
                            v3277 = v3274;
                        }
                        let v3279 = v33 * (((v50 - v3044) * v46) / v3277);
                        let v3281 = (-v281) / v3279;
                        let v3283 = if (v3281.abs()) < v323 { 1.0 } else { 0.0 };
                        let v3311: f64;
                        if v3283 != 0.0 {
                            let v3284 = v3281.exp();
                            v3311 = v3284;
                        } else {
                            let v3285 = if v3281 < v0 { 1.0 } else { 0.0 };
                            let v3312: f64;
                            if v3285 != 0.0 {
                                let v3299 = v327 / (v5 + ((v3286 - v3281) * (v5 + (v3 * ((v3288 - v3281) * (v5 + ((v3290 - v3281) * v334)))))));
                                v3312 = v3299;
                            } else {
                                let v3300 = v3281 - v323;
                                let v3308 = v343 * (v5 + (v3300 * (v5 + (v3 * (v3300 * (v5 + (v3300 * v334)))))));
                                v3312 = v3308;
                            }
                            v3311 = v3312;
                        }
                        let v3314 = v883 * (((v367 * v3279) * v3279) * v3311);
                        v3338 = v3314;
                    }
                    let v3315 = if v77 > v703 { 1.0 } else { 0.0 };
                    let v3341: f64;
                    if v3315 != 0.0 {
                        v3341 = v5;
                    } else {
                        let v3318 = if v3092 > ((-v62) * v77) { 1.0 } else { 0.0 };
                        let v3342: f64;
                        if v3318 != 0.0 {
                            let v3319 = if v67 == v387 { 1.0 } else { 0.0 };
                            let v3327: f64;
                            if v3319 != 0.0 {
                                let v3320 = v3092 * v78;
                                let v3323 = ((v3320 * v3320) * v3320) * v3320;
                                v3327 = v3323;
                            } else {
                                let v3326 = ((v3092 * v78).abs()).powf(v67);
                                v3327 = v3326;
                            }
                            let v3329 = v5 / (v5 - v3327);
                            v3342 = v3329;
                        } else {
                            let v3333 = v70 + ((v3092 + (v62 * v77)) * v94);
                            v3342 = v3333;
                        }
                        v3341 = v3342;
                    }
                    let v3343 = (v724 * (((v3121 + v3334) + v3336) + v3338)) * v3341;
                    v3377 = v3153;
                    v3380 = v3156;
                    v3403 = v3179;
                    v3486 = v3262;
                    v3569 = v3343;
                }
                let v3572: f64;
                let v3719: f64;
                let v3722: f64;
                let v3745: f64;
                let v3828: f64;
                if v359 != 0.0 {
                    v3572 = v0;
                    v3719 = v3377;
                    v3722 = v3380;
                    v3745 = v3403;
                    v3828 = v3486;
                } else {
                    let v3344 = v174 * v2892;
                    let v3346 = if v964 == v0 { 1.0 } else { 0.0 };
                    let v3347 = if (if v962 == v0 { 1.0 } else { 0.0 }) != 0.0 && v3346 != 0.0 { 1.0 } else { 0.0 };
                    let v3376: f64;
                    let v3379: f64;
                    let v3402: f64;
                    let v3485: f64;
                    let v3557: f64;
                    if v3347 != 0.0 {
                        v3376 = v3377;
                        v3379 = v3380;
                        v3402 = v3403;
                        v3485 = v3486;
                        v3557 = v0;
                    } else {
                        let v3348 = v208 - v2897;
                        let v3352 = v5 - ((v5 - (v2899 / v3348)).sqrt());
                        let v3353 = if v30 == v3 { 1.0 } else { 0.0 };
                        let v3363: f64;
                        if v3353 != 0.0 {
                            v3363 = v0;
                        } else {
                            let v3362 = ((((v3352 * v3352) * (v3352.ln())) / (v5 - v3352)) + v3352) * (v5 - (v56 * v30));
                            v3363 = v3362;
                        }
                        let v3364 = v3352 + v3363;
                        let v3369: f64;
                        if v3353 != 0.0 {
                            let v3366 = (v3348 * v53).sqrt();
                            v3369 = v3366;
                        } else {
                            let v3368 = (v3348 * v53).powf(v30);
                            v3369 = v3368;
                        }
                        let v3370 = v44 * v3369;
                        let v3373 = v165 * ((v2922 - v5) * v3370);
                        let v3375 = v962 * (v3373 * v3364);
                        v3376 = v3370;
                        v3379 = v3348;
                        v3402 = v3364;
                        v3485 = v3373;
                        v3557 = v3375;
                    }
                    let v3559: f64;
                    if v3346 != 0.0 {
                        v3559 = v0;
                    } else {
                        let v3382 = v261 * ((v3376 * v31) / v3379);
                        let v3384 = (v546 * v229) / v3382;
                        let v3385 = v3384 * v3384;
                        let v3386 = v3385 * v3385;
                        let v3389 = (v3386 / (v3386 + v5)).sqrt();
                        let v3390 = v3389.sqrt();
                        let v3391 = v3389 * v3390;
                        let v3393 = (-v30) * v34;
                        let v3395 = if v3393 == v3394 { 1.0 } else { 0.0 };
                        let v3404: f64;
                        if v3395 != 0.0 {
                            let v3398 = v5 / (v5 + (v3382 * v3391));
                            v3404 = v3398;
                        } else {
                            let v3401 = (v5 + (v3382 * v3391)).powf(v3393);
                            v3404 = v3401;
                        }
                        let v3407 = (v3402 * v3404) / (v3402 + v3404);
                        let v3410 = (v571 * (v3382 / v3390)).sqrt();
                        let v3420 = (((v229 * v3384) * v3390) - (v229 * v3389)) + (v3 * (v3382 * v3391));
                        let v3422 = (((v56 * (v3384 * v3390)) - v3389) - v5) * v3410;
                        let v3423 = v3422 * v3422;
                        let v3424 = if v3422 > v0 { 1.0 } else { 0.0 };
                        let v3450: f64;
                        if v3424 != 0.0 {
                            let v3427 = v5 / (v5 + (v55 * v3422));
                            v3450 = v3427;
                        } else {
                            let v3430 = v5 / (v5 - (v55 * v3422));
                            v3450 = v3430;
                        }
                        let v3432 = (-v3423) + v3420;
                        let v3434 = if v3432 > v3433 { 1.0 } else { 0.0 };
                        let v3458: f64;
                        if v3434 != 0.0 {
                            let v3435 = v3432.exp();
                            v3458 = v3435;
                        } else {
                            let v3449 = v327 / (v5 + ((v3436 - v3432) * (v5 + (v3 * ((v3438 - v3432) * (v5 + ((v3440 - v3432) * v334)))))));
                            v3458 = v3449;
                        }
                        let v3452 = v3450 * v3450;
                        let v3459 = (((v54 * v3450) + (v58 * v3452)) + (v59 * (v3452 * v3450))) * v3458;
                        let v3481: f64;
                        if v3424 != 0.0 {
                            v3481 = v3459;
                        } else {
                            let v3461 = if v3420 > v3460 { 1.0 } else { 0.0 };
                            let v3477: f64;
                            if v3461 != 0.0 {
                                let v3462 = v3420.exp();
                                v3477 = v3462;
                            } else {
                                let v3476 = v327 / (v5 + ((v3463 - v3420) * (v5 + (v3 * ((v3465 - v3420) * (v5 + ((v3467 - v3420) * v334)))))));
                                v3477 = v3476;
                            }
                            let v3479 = (v56 * v3477) - v3459;
                            v3481 = v3479;
                        }
                        let v3489 = v964 * ((v3485 * (v3480 * ((v229 * v3481) / v3410))) * v3407);
                        v3559 = v3489;
                    }
                    let v3490 = if v1109 == v0 { 1.0 } else { 0.0 };
                    let v3561: f64;
                    if v3490 != 0.0 {
                        v3561 = v0;
                    } else {
                        let v3491 = if v30 == v3 { 1.0 } else { 0.0 };
                        let v3500: f64;
                        if v3491 != 0.0 {
                            let v3494 = ((v52 - v3044) * v53).sqrt();
                            v3500 = v3494;
                        } else {
                            let v3497 = ((v52 - v3044) * v53).powf(v30);
                            v3500 = v3497;
                        }
                        let v3502 = v34 * (((v52 - v3044) * v47) / v3500);
                        let v3504 = (-v283) / v3502;
                        let v3506 = if (v3504.abs()) < v323 { 1.0 } else { 0.0 };
                        let v3534: f64;
                        if v3506 != 0.0 {
                            let v3507 = v3504.exp();
                            v3534 = v3507;
                        } else {
                            let v3508 = if v3504 < v0 { 1.0 } else { 0.0 };
                            let v3535: f64;
                            if v3508 != 0.0 {
                                let v3522 = v327 / (v5 + ((v3509 - v3504) * (v5 + (v3 * ((v3511 - v3504) * (v5 + ((v3513 - v3504) * v334)))))));
                                v3535 = v3522;
                            } else {
                                let v3523 = v3504 - v323;
                                let v3531 = v343 * (v5 + (v3523 * (v5 + (v3 * (v3523 * (v5 + (v3523 * v334)))))));
                                v3535 = v3531;
                            }
                            v3534 = v3535;
                        }
                        let v3537 = v1109 * (((v367 * v3502) * v3502) * v3534);
                        v3561 = v3537;
                    }
                    let v3538 = if v79 > v703 { 1.0 } else { 0.0 };
                    let v3564: f64;
                    if v3538 != 0.0 {
                        v3564 = v5;
                    } else {
                        let v3541 = if v3092 > ((-v62) * v79) { 1.0 } else { 0.0 };
                        let v3565: f64;
                        if v3541 != 0.0 {
                            let v3542 = if v71 == v387 { 1.0 } else { 0.0 };
                            let v3550: f64;
                            if v3542 != 0.0 {
                                let v3543 = v3092 * v80;
                                let v3546 = ((v3543 * v3543) * v3543) * v3543;
                                v3550 = v3546;
                            } else {
                                let v3549 = ((v3092 * v80).abs()).powf(v71);
                                v3550 = v3549;
                            }
                            let v3552 = v5 / (v5 - v3550);
                            v3565 = v3552;
                        } else {
                            let v3556 = v74 + ((v3092 + (v62 * v79)) * v101);
                            v3565 = v3556;
                        }
                        v3564 = v3565;
                    }
                    let v3566 = (v724 * (((v3344 + v3557) + v3559) + v3561)) * v3564;
                    v3572 = v3566;
                    v3719 = v3376;
                    v3722 = v3379;
                    v3745 = v3402;
                    v3828 = v3485;
                }
                let v3574 = ((v286 * v3567) + (v289 * v3569)) + (v292 * v3572);
                let v3682: f64;
                let v3687: f64;
                let v3689: f64;
                let v3712: f64;
                let v3834: f64;
                let v3882: f64;
                if v386 != 0.0 {
                    let v3575 = if v383 < v320 { 1.0 } else { 0.0 };
                    let v3634: f64;
                    let v3637: f64;
                    let v3648: f64;
                    if v3575 != 0.0 {
                        let v3577 = v383 * v138;
                        let v3580 = if ((v3576 * v3577).abs()) < v323 { 1.0 } else { 0.0 };
                        let v3624: f64;
                        if v3580 != 0.0 {
                            let v3583 = (v3581 * v3577).exp();
                            v3624 = v3583;
                        } else {
                            let v3586 = if (v3584 * v3577) < v0 { 1.0 } else { 0.0 };
                            let v3625: f64;
                            if v3586 != 0.0 {
                                let v3606 = v327 / (v5 + ((v3587 - (v3588 * v3577)) * (v5 + (v3 * ((v3591 - (v3592 * v3577)) * (v5 + ((v3595 - (v3596 * v3577)) * v334)))))));
                                v3625 = v3606;
                            } else {
                                let v3623 = v343 * (v5 + (((v3607 * v3577) - v323) * (v5 + (v3 * (((v3610 * v3577) - v323) * (v5 + (((v3613 * v3577) - v323) * v334)))))));
                                v3625 = v3623;
                            }
                            v3624 = v3625;
                        }
                        let v3626 = v5 / v3624;
                        let v3627 = v3626 * v3626;
                        v3634 = v3627;
                        v3637 = v3624;
                        v3648 = v3626;
                    } else {
                        let v3631 = (v5 + ((v383 - v320) * v138)) * v444;
                        let v3632 = v3631.sqrt();
                        let v3633 = v5 / v3632;
                        v3634 = v3631;
                        v3637 = v3633;
                        v3648 = v3632;
                    }
                    let v3635 = v3634 - v5;
                    let v3661: f64;
                    if v3636 != 0.0 {
                        let v3646 = v56 * (v137 * (((v56 + v3637) + (((v3637 + v5) * (v3637 + v57)).sqrt())).ln()));
                        v3661 = v3646;
                    } else {
                        let v3660 = v3647 + (v56 * (v137 * ((((v56 * v3648) + v5) + (((v5 + v3648) * (v5 + (v57 * v3648))).sqrt())).ln())));
                        v3661 = v3660;
                    }
                    let v3662 = v366 - v3661;
                    let v3664 = v383 - v3662;
                    let v3671 = v3 * ((v383 + v3662) - (((v3664 * v3664) + ((v387 * v137) * v137)).sqrt()));
                    let v3673 = v383 - v373;
                    let v3680 = v3 * ((v383 + v373) - (((v3673 * v3673) + ((v387 * v11) * v11)).sqrt()));
                    v3682 = v3635;
                    v3687 = v3671;
                    v3689 = v3661;
                    v3712 = v3648;
                    v3834 = v3680;
                    v3882 = v3681;
                } else {
                    v3682 = v2892;
                    v3687 = v2897;
                    v3689 = v0;
                    v3712 = v2922;
                    v3834 = v0;
                    v3882 = v3092;
                }
                let v3944: f64;
                let v3947: f64;
                let v3970: f64;
                let v4053: f64;
                let v4357: f64;
                if v353 != 0.0 {
                    v3944 = v3719;
                    v3947 = v3722;
                    v3970 = v3745;
                    v4053 = v3828;
                    v4357 = v0;
                } else {
                    let v3683 = v168 * v3682;
                    let v3685 = if v507 == v0 { 1.0 } else { 0.0 };
                    let v3686 = if (if v505 == v0 { 1.0 } else { 0.0 }) != 0.0 && v3685 != 0.0 { 1.0 } else { 0.0 };
                    let v3718: f64;
                    let v3721: f64;
                    let v3744: f64;
                    let v3827: f64;
                    let v3901: f64;
                    if v3686 != 0.0 {
                        v3718 = v3719;
                        v3721 = v3722;
                        v3744 = v3745;
                        v3827 = v3828;
                        v3901 = v0;
                    } else {
                        let v3688 = v194 - v3687;
                        let v3693 = v5 - ((v5 - (v3689 / v3688)).sqrt());
                        let v3694 = if v26 == v3 { 1.0 } else { 0.0 };
                        let v3704: f64;
                        if v3694 != 0.0 {
                            v3704 = v0;
                        } else {
                            let v3703 = ((((v3693 * v3693) * (v3693.ln())) / (v5 - v3693)) + v3693) * (v5 - (v56 * v26));
                            v3704 = v3703;
                        }
                        let v3705 = v3693 + v3704;
                        let v3710: f64;
                        if v3694 != 0.0 {
                            let v3707 = (v3688 * v49).sqrt();
                            v3710 = v3707;
                        } else {
                            let v3709 = (v3688 * v49).powf(v26);
                            v3710 = v3709;
                        }
                        let v3711 = v36 * v3710;
                        let v3715 = v153 * ((v3712 - v5) * v3711);
                        let v3717 = v505 * (v3715 * v3705);
                        v3718 = v3711;
                        v3721 = v3688;
                        v3744 = v3705;
                        v3827 = v3715;
                        v3901 = v3717;
                    }
                    let v3903: f64;
                    if v3685 != 0.0 {
                        v3903 = v0;
                    } else {
                        let v3724 = v241 * ((v3718 * v27) / v3721);
                        let v3726 = (v546 * v227) / v3724;
                        let v3727 = v3726 * v3726;
                        let v3728 = v3727 * v3727;
                        let v3731 = (v3728 / (v3728 + v5)).sqrt();
                        let v3732 = v3731.sqrt();
                        let v3733 = v3731 * v3732;
                        let v3735 = (-v26) * v32;
                        let v3737 = if v3735 == v3736 { 1.0 } else { 0.0 };
                        let v3746: f64;
                        if v3737 != 0.0 {
                            let v3740 = v5 / (v5 + (v3724 * v3733));
                            v3746 = v3740;
                        } else {
                            let v3743 = (v5 + (v3724 * v3733)).powf(v3735);
                            v3746 = v3743;
                        }
                        let v3749 = (v3744 * v3746) / (v3744 + v3746);
                        let v3752 = (v571 * (v3724 / v3732)).sqrt();
                        let v3762 = (((v227 * v3726) * v3732) - (v227 * v3731)) + (v3 * (v3724 * v3733));
                        let v3764 = (((v56 * (v3726 * v3732)) - v3731) - v5) * v3752;
                        let v3765 = v3764 * v3764;
                        let v3766 = if v3764 > v0 { 1.0 } else { 0.0 };
                        let v3792: f64;
                        if v3766 != 0.0 {
                            let v3769 = v5 / (v5 + (v55 * v3764));
                            v3792 = v3769;
                        } else {
                            let v3772 = v5 / (v5 - (v55 * v3764));
                            v3792 = v3772;
                        }
                        let v3774 = (-v3765) + v3762;
                        let v3776 = if v3774 > v3775 { 1.0 } else { 0.0 };
                        let v3800: f64;
                        if v3776 != 0.0 {
                            let v3777 = v3774.exp();
                            v3800 = v3777;
                        } else {
                            let v3791 = v327 / (v5 + ((v3778 - v3774) * (v5 + (v3 * ((v3780 - v3774) * (v5 + ((v3782 - v3774) * v334)))))));
                            v3800 = v3791;
                        }
                        let v3794 = v3792 * v3792;
                        let v3801 = (((v54 * v3792) + (v58 * v3794)) + (v59 * (v3794 * v3792))) * v3800;
                        let v3823: f64;
                        if v3766 != 0.0 {
                            v3823 = v3801;
                        } else {
                            let v3803 = if v3762 > v3802 { 1.0 } else { 0.0 };
                            let v3819: f64;
                            if v3803 != 0.0 {
                                let v3804 = v3762.exp();
                                v3819 = v3804;
                            } else {
                                let v3818 = v327 / (v5 + ((v3805 - v3762) * (v5 + (v3 * ((v3807 - v3762) * (v5 + ((v3809 - v3762) * v334)))))));
                                v3819 = v3818;
                            }
                            let v3821 = (v56 * v3819) - v3801;
                            v3823 = v3821;
                        }
                        let v3831 = v507 * ((v3827 * (v3822 * ((v227 * v3823) / v3752))) * v3749);
                        v3903 = v3831;
                    }
                    let v3832 = if v653 == v0 { 1.0 } else { 0.0 };
                    let v3905: f64;
                    if v3832 != 0.0 {
                        v3905 = v0;
                    } else {
                        let v3833 = if v26 == v3 { 1.0 } else { 0.0 };
                        let v3843: f64;
                        if v3833 != 0.0 {
                            let v3837 = ((v48 - v3834) * v49).sqrt();
                            v3843 = v3837;
                        } else {
                            let v3840 = ((v48 - v3834) * v49).powf(v26);
                            v3843 = v3840;
                        }
                        let v3845 = v32 * (((v48 - v3834) * v45) / v3843);
                        let v3847 = (-v279) / v3845;
                        let v3849 = if (v3847.abs()) < v323 { 1.0 } else { 0.0 };
                        let v3877: f64;
                        if v3849 != 0.0 {
                            let v3850 = v3847.exp();
                            v3877 = v3850;
                        } else {
                            let v3851 = if v3847 < v0 { 1.0 } else { 0.0 };
                            let v3878: f64;
                            if v3851 != 0.0 {
                                let v3865 = v327 / (v5 + ((v3852 - v3847) * (v5 + (v3 * ((v3854 - v3847) * (v5 + ((v3856 - v3847) * v334)))))));
                                v3878 = v3865;
                            } else {
                                let v3866 = v3847 - v323;
                                let v3874 = v343 * (v5 + (v3866 * (v5 + (v3 * (v3866 * (v5 + (v3866 * v334)))))));
                                v3878 = v3874;
                            }
                            v3877 = v3878;
                        }
                        let v3880 = v653 * (((v383 * v3845) * v3845) * v3877);
                        v3905 = v3880;
                    }
                    let v3881 = if v75 > v703 { 1.0 } else { 0.0 };
                    let v3908: f64;
                    if v3881 != 0.0 {
                        v3908 = v5;
                    } else {
                        let v3885 = if v3882 > ((-v62) * v75) { 1.0 } else { 0.0 };
                        let v3909: f64;
                        if v3885 != 0.0 {
                            let v3886 = if v63 == v387 { 1.0 } else { 0.0 };
                            let v3894: f64;
                            if v3886 != 0.0 {
                                let v3887 = v3882 * v76;
                                let v3890 = ((v3887 * v3887) * v3887) * v3887;
                                v3894 = v3890;
                            } else {
                                let v3893 = ((v3882 * v76).abs()).powf(v63);
                                v3894 = v3893;
                            }
                            let v3896 = v5 / (v5 - v3894);
                            v3909 = v3896;
                        } else {
                            let v3900 = v66 + ((v3882 + (v62 * v75)) * v87);
                            v3909 = v3900;
                        }
                        v3908 = v3909;
                    }
                    let v3910 = (v724 * (((v3683 + v3901) + v3903) + v3905)) * v3908;
                    v3944 = v3718;
                    v3947 = v3721;
                    v3970 = v3744;
                    v4053 = v3827;
                    v4357 = v3910;
                }
                let v4167: f64;
                let v4170: f64;
                let v4193: f64;
                let v4276: f64;
                let v4359: f64;
                if v356 != 0.0 {
                    v4167 = v3944;
                    v4170 = v3947;
                    v4193 = v3970;
                    v4276 = v4053;
                    v4359 = v0;
                } else {
                    let v3911 = v171 * v3682;
                    let v3913 = if v738 == v0 { 1.0 } else { 0.0 };
                    let v3914 = if (if v736 == v0 { 1.0 } else { 0.0 }) != 0.0 && v3913 != 0.0 { 1.0 } else { 0.0 };
                    let v3943: f64;
                    let v3946: f64;
                    let v3969: f64;
                    let v4052: f64;
                    let v4124: f64;
                    if v3914 != 0.0 {
                        v3943 = v3944;
                        v3946 = v3947;
                        v3969 = v3970;
                        v4052 = v4053;
                        v4124 = v0;
                    } else {
                        let v3915 = v201 - v3687;
                        let v3919 = v5 - ((v5 - (v3689 / v3915)).sqrt());
                        let v3920 = if v28 == v3 { 1.0 } else { 0.0 };
                        let v3930: f64;
                        if v3920 != 0.0 {
                            v3930 = v0;
                        } else {
                            let v3929 = ((((v3919 * v3919) * (v3919.ln())) / (v5 - v3919)) + v3919) * (v5 - (v56 * v28));
                            v3930 = v3929;
                        }
                        let v3931 = v3919 + v3930;
                        let v3936: f64;
                        if v3920 != 0.0 {
                            let v3933 = (v3915 * v51).sqrt();
                            v3936 = v3933;
                        } else {
                            let v3935 = (v3915 * v51).powf(v28);
                            v3936 = v3935;
                        }
                        let v3937 = v40 * v3936;
                        let v3940 = v159 * ((v3712 - v5) * v3937);
                        let v3942 = v736 * (v3940 * v3931);
                        v3943 = v3937;
                        v3946 = v3915;
                        v3969 = v3931;
                        v4052 = v3940;
                        v4124 = v3942;
                    }
                    let v4126: f64;
                    if v3913 != 0.0 {
                        v4126 = v0;
                    } else {
                        let v3949 = v251 * ((v3943 * v29) / v3946);
                        let v3951 = (v546 * v228) / v3949;
                        let v3952 = v3951 * v3951;
                        let v3953 = v3952 * v3952;
                        let v3956 = (v3953 / (v3953 + v5)).sqrt();
                        let v3957 = v3956.sqrt();
                        let v3958 = v3956 * v3957;
                        let v3960 = (-v28) * v33;
                        let v3962 = if v3960 == v3961 { 1.0 } else { 0.0 };
                        let v3971: f64;
                        if v3962 != 0.0 {
                            let v3965 = v5 / (v5 + (v3949 * v3958));
                            v3971 = v3965;
                        } else {
                            let v3968 = (v5 + (v3949 * v3958)).powf(v3960);
                            v3971 = v3968;
                        }
                        let v3974 = (v3969 * v3971) / (v3969 + v3971);
                        let v3977 = (v571 * (v3949 / v3957)).sqrt();
                        let v3987 = (((v228 * v3951) * v3957) - (v228 * v3956)) + (v3 * (v3949 * v3958));
                        let v3989 = (((v56 * (v3951 * v3957)) - v3956) - v5) * v3977;
                        let v3990 = v3989 * v3989;
                        let v3991 = if v3989 > v0 { 1.0 } else { 0.0 };
                        let v4017: f64;
                        if v3991 != 0.0 {
                            let v3994 = v5 / (v5 + (v55 * v3989));
                            v4017 = v3994;
                        } else {
                            let v3997 = v5 / (v5 - (v55 * v3989));
                            v4017 = v3997;
                        }
                        let v3999 = (-v3990) + v3987;
                        let v4001 = if v3999 > v4000 { 1.0 } else { 0.0 };
                        let v4025: f64;
                        if v4001 != 0.0 {
                            let v4002 = v3999.exp();
                            v4025 = v4002;
                        } else {
                            let v4016 = v327 / (v5 + ((v4003 - v3999) * (v5 + (v3 * ((v4005 - v3999) * (v5 + ((v4007 - v3999) * v334)))))));
                            v4025 = v4016;
                        }
                        let v4019 = v4017 * v4017;
                        let v4026 = (((v54 * v4017) + (v58 * v4019)) + (v59 * (v4019 * v4017))) * v4025;
                        let v4048: f64;
                        if v3991 != 0.0 {
                            v4048 = v4026;
                        } else {
                            let v4028 = if v3987 > v4027 { 1.0 } else { 0.0 };
                            let v4044: f64;
                            if v4028 != 0.0 {
                                let v4029 = v3987.exp();
                                v4044 = v4029;
                            } else {
                                let v4043 = v327 / (v5 + ((v4030 - v3987) * (v5 + (v3 * ((v4032 - v3987) * (v5 + ((v4034 - v3987) * v334)))))));
                                v4044 = v4043;
                            }
                            let v4046 = (v56 * v4044) - v4026;
                            v4048 = v4046;
                        }
                        let v4056 = v738 * ((v4052 * (v4047 * ((v228 * v4048) / v3977))) * v3974);
                        v4126 = v4056;
                    }
                    let v4057 = if v883 == v0 { 1.0 } else { 0.0 };
                    let v4128: f64;
                    if v4057 != 0.0 {
                        v4128 = v0;
                    } else {
                        let v4058 = if v28 == v3 { 1.0 } else { 0.0 };
                        let v4067: f64;
                        if v4058 != 0.0 {
                            let v4061 = ((v50 - v3834) * v51).sqrt();
                            v4067 = v4061;
                        } else {
                            let v4064 = ((v50 - v3834) * v51).powf(v28);
                            v4067 = v4064;
                        }
                        let v4069 = v33 * (((v50 - v3834) * v46) / v4067);
                        let v4071 = (-v281) / v4069;
                        let v4073 = if (v4071.abs()) < v323 { 1.0 } else { 0.0 };
                        let v4101: f64;
                        if v4073 != 0.0 {
                            let v4074 = v4071.exp();
                            v4101 = v4074;
                        } else {
                            let v4075 = if v4071 < v0 { 1.0 } else { 0.0 };
                            let v4102: f64;
                            if v4075 != 0.0 {
                                let v4089 = v327 / (v5 + ((v4076 - v4071) * (v5 + (v3 * ((v4078 - v4071) * (v5 + ((v4080 - v4071) * v334)))))));
                                v4102 = v4089;
                            } else {
                                let v4090 = v4071 - v323;
                                let v4098 = v343 * (v5 + (v4090 * (v5 + (v3 * (v4090 * (v5 + (v4090 * v334)))))));
                                v4102 = v4098;
                            }
                            v4101 = v4102;
                        }
                        let v4104 = v883 * (((v383 * v4069) * v4069) * v4101);
                        v4128 = v4104;
                    }
                    let v4105 = if v77 > v703 { 1.0 } else { 0.0 };
                    let v4131: f64;
                    if v4105 != 0.0 {
                        v4131 = v5;
                    } else {
                        let v4108 = if v3882 > ((-v62) * v77) { 1.0 } else { 0.0 };
                        let v4132: f64;
                        if v4108 != 0.0 {
                            let v4109 = if v67 == v387 { 1.0 } else { 0.0 };
                            let v4117: f64;
                            if v4109 != 0.0 {
                                let v4110 = v3882 * v78;
                                let v4113 = ((v4110 * v4110) * v4110) * v4110;
                                v4117 = v4113;
                            } else {
                                let v4116 = ((v3882 * v78).abs()).powf(v67);
                                v4117 = v4116;
                            }
                            let v4119 = v5 / (v5 - v4117);
                            v4132 = v4119;
                        } else {
                            let v4123 = v70 + ((v3882 + (v62 * v77)) * v94);
                            v4132 = v4123;
                        }
                        v4131 = v4132;
                    }
                    let v4133 = (v724 * (((v3911 + v4124) + v4126) + v4128)) * v4131;
                    v4167 = v3943;
                    v4170 = v3946;
                    v4193 = v3969;
                    v4276 = v4052;
                    v4359 = v4133;
                }
                let v4362: f64;
                if v359 != 0.0 {
                    v4362 = v0;
                } else {
                    let v4134 = v174 * v3682;
                    let v4136 = if v964 == v0 { 1.0 } else { 0.0 };
                    let v4137 = if (if v962 == v0 { 1.0 } else { 0.0 }) != 0.0 && v4136 != 0.0 { 1.0 } else { 0.0 };
                    let v4166: f64;
                    let v4169: f64;
                    let v4192: f64;
                    let v4275: f64;
                    let v4347: f64;
                    if v4137 != 0.0 {
                        v4166 = v4167;
                        v4169 = v4170;
                        v4192 = v4193;
                        v4275 = v4276;
                        v4347 = v0;
                    } else {
                        let v4138 = v208 - v3687;
                        let v4142 = v5 - ((v5 - (v3689 / v4138)).sqrt());
                        let v4143 = if v30 == v3 { 1.0 } else { 0.0 };
                        let v4153: f64;
                        if v4143 != 0.0 {
                            v4153 = v0;
                        } else {
                            let v4152 = ((((v4142 * v4142) * (v4142.ln())) / (v5 - v4142)) + v4142) * (v5 - (v56 * v30));
                            v4153 = v4152;
                        }
                        let v4154 = v4142 + v4153;
                        let v4159: f64;
                        if v4143 != 0.0 {
                            let v4156 = (v4138 * v53).sqrt();
                            v4159 = v4156;
                        } else {
                            let v4158 = (v4138 * v53).powf(v30);
                            v4159 = v4158;
                        }
                        let v4160 = v44 * v4159;
                        let v4163 = v165 * ((v3712 - v5) * v4160);
                        let v4165 = v962 * (v4163 * v4154);
                        v4166 = v4160;
                        v4169 = v4138;
                        v4192 = v4154;
                        v4275 = v4163;
                        v4347 = v4165;
                    }
                    let v4349: f64;
                    if v4136 != 0.0 {
                        v4349 = v0;
                    } else {
                        let v4172 = v261 * ((v4166 * v31) / v4169);
                        let v4174 = (v546 * v229) / v4172;
                        let v4175 = v4174 * v4174;
                        let v4176 = v4175 * v4175;
                        let v4179 = (v4176 / (v4176 + v5)).sqrt();
                        let v4180 = v4179.sqrt();
                        let v4181 = v4179 * v4180;
                        let v4183 = (-v30) * v34;
                        let v4185 = if v4183 == v4184 { 1.0 } else { 0.0 };
                        let v4194: f64;
                        if v4185 != 0.0 {
                            let v4188 = v5 / (v5 + (v4172 * v4181));
                            v4194 = v4188;
                        } else {
                            let v4191 = (v5 + (v4172 * v4181)).powf(v4183);
                            v4194 = v4191;
                        }
                        let v4197 = (v4192 * v4194) / (v4192 + v4194);
                        let v4200 = (v571 * (v4172 / v4180)).sqrt();
                        let v4210 = (((v229 * v4174) * v4180) - (v229 * v4179)) + (v3 * (v4172 * v4181));
                        let v4212 = (((v56 * (v4174 * v4180)) - v4179) - v5) * v4200;
                        let v4213 = v4212 * v4212;
                        let v4214 = if v4212 > v0 { 1.0 } else { 0.0 };
                        let v4240: f64;
                        if v4214 != 0.0 {
                            let v4217 = v5 / (v5 + (v55 * v4212));
                            v4240 = v4217;
                        } else {
                            let v4220 = v5 / (v5 - (v55 * v4212));
                            v4240 = v4220;
                        }
                        let v4222 = (-v4213) + v4210;
                        let v4224 = if v4222 > v4223 { 1.0 } else { 0.0 };
                        let v4248: f64;
                        if v4224 != 0.0 {
                            let v4225 = v4222.exp();
                            v4248 = v4225;
                        } else {
                            let v4239 = v327 / (v5 + ((v4226 - v4222) * (v5 + (v3 * ((v4228 - v4222) * (v5 + ((v4230 - v4222) * v334)))))));
                            v4248 = v4239;
                        }
                        let v4242 = v4240 * v4240;
                        let v4249 = (((v54 * v4240) + (v58 * v4242)) + (v59 * (v4242 * v4240))) * v4248;
                        let v4271: f64;
                        if v4214 != 0.0 {
                            v4271 = v4249;
                        } else {
                            let v4251 = if v4210 > v4250 { 1.0 } else { 0.0 };
                            let v4267: f64;
                            if v4251 != 0.0 {
                                let v4252 = v4210.exp();
                                v4267 = v4252;
                            } else {
                                let v4266 = v327 / (v5 + ((v4253 - v4210) * (v5 + (v3 * ((v4255 - v4210) * (v5 + ((v4257 - v4210) * v334)))))));
                                v4267 = v4266;
                            }
                            let v4269 = (v56 * v4267) - v4249;
                            v4271 = v4269;
                        }
                        let v4279 = v964 * ((v4275 * (v4270 * ((v229 * v4271) / v4200))) * v4197);
                        v4349 = v4279;
                    }
                    let v4280 = if v1109 == v0 { 1.0 } else { 0.0 };
                    let v4351: f64;
                    if v4280 != 0.0 {
                        v4351 = v0;
                    } else {
                        let v4281 = if v30 == v3 { 1.0 } else { 0.0 };
                        let v4290: f64;
                        if v4281 != 0.0 {
                            let v4284 = ((v52 - v3834) * v53).sqrt();
                            v4290 = v4284;
                        } else {
                            let v4287 = ((v52 - v3834) * v53).powf(v30);
                            v4290 = v4287;
                        }
                        let v4292 = v34 * (((v52 - v3834) * v47) / v4290);
                        let v4294 = (-v283) / v4292;
                        let v4296 = if (v4294.abs()) < v323 { 1.0 } else { 0.0 };
                        let v4324: f64;
                        if v4296 != 0.0 {
                            let v4297 = v4294.exp();
                            v4324 = v4297;
                        } else {
                            let v4298 = if v4294 < v0 { 1.0 } else { 0.0 };
                            let v4325: f64;
                            if v4298 != 0.0 {
                                let v4312 = v327 / (v5 + ((v4299 - v4294) * (v5 + (v3 * ((v4301 - v4294) * (v5 + ((v4303 - v4294) * v334)))))));
                                v4325 = v4312;
                            } else {
                                let v4313 = v4294 - v323;
                                let v4321 = v343 * (v5 + (v4313 * (v5 + (v3 * (v4313 * (v5 + (v4313 * v334)))))));
                                v4325 = v4321;
                            }
                            v4324 = v4325;
                        }
                        let v4327 = v1109 * (((v383 * v4292) * v4292) * v4324);
                        v4351 = v4327;
                    }
                    let v4328 = if v79 > v703 { 1.0 } else { 0.0 };
                    let v4354: f64;
                    if v4328 != 0.0 {
                        v4354 = v5;
                    } else {
                        let v4331 = if v3882 > ((-v62) * v79) { 1.0 } else { 0.0 };
                        let v4355: f64;
                        if v4331 != 0.0 {
                            let v4332 = if v71 == v387 { 1.0 } else { 0.0 };
                            let v4340: f64;
                            if v4332 != 0.0 {
                                let v4333 = v3882 * v80;
                                let v4336 = ((v4333 * v4333) * v4333) * v4333;
                                v4340 = v4336;
                            } else {
                                let v4339 = ((v3882 * v80).abs()).powf(v71);
                                v4340 = v4339;
                            }
                            let v4342 = v5 / (v5 - v4340);
                            v4355 = v4342;
                        } else {
                            let v4346 = v74 + ((v3882 + (v62 * v79)) * v101);
                            v4355 = v4346;
                        }
                        v4354 = v4355;
                    }
                    let v4356 = (v724 * (((v4134 + v4347) + v4349) + v4351)) * v4354;
                    v4362 = v4356;
                }
                let v4364 = ((v286 * v4357) + (v289 * v4359)) + (v292 * v4362);
                let v4366 = (v296 + v304) + v310;
                let v4367 = v367 * v138;
                let v4371 = v3574 - (v4366 * ((v4367.exp()) - v5));
                let v4376 = v4364 - (v4366 * (((v383 * v138).exp()) - v5));
                let v4510: f64;
                let v4514: f64;
                let v4556: f64;
                let v4577: f64;
                let v4584: f64;
                if v386 != 0.0 {
                    let v4379 = if (if v3574 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v4364 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4406: f64;
                    let v4408: f64;
                    if v4379 != 0.0 {
                        let v4391 = if (if (if (if (if (v4371 / v3574) > v4381 { 1.0 } else { 0.0 }) != 0.0 || (if (v4376 / v4364) > v4381 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v4371 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v4376 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v4376 > v4371 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v4407: f64;
                        let v4409: f64;
                        if v4391 != 0.0 {
                            let v4396 = (v137 * ((v4371 / v4376).ln())) / v4395;
                            let v4400 = v4371 / (((v4367 * v4396).exp()) - v5);
                            v4407 = v4400;
                            v4409 = v4396;
                        } else {
                            v4407 = v0;
                            v4409 = v5;
                        }
                        v4406 = v4407;
                        v4408 = v4409;
                    } else {
                        v4406 = v0;
                        v4408 = v5;
                    }
                    let v4401 = v378 * v138;
                    let v4414 = (v1194 - (v4366 * ((v4401.exp()) - v5))) - (v4406 * (((v4401 * v4408).exp()) - v5));
                    let v4415 = v380 * v138;
                    let v4424 = (v1989 - (v4366 * ((v4415.exp()) - v5))) - (v4406 * (((v4415 * v4408).exp()) - v5));
                    let v4425 = v382 * v138;
                    let v4434 = (v2784 - (v4366 * ((v4425.exp()) - v5))) - (v4406 * (((v4425 * v4408).exp()) - v5));
                    let v4439 = if (if (if v1194 < v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1989 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v2784 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4515: f64;
                    let v4578: f64;
                    let v4585: f64;
                    if v4439 != 0.0 {
                        let v4453 = if (if (if (if (if (if (v4414 / v1194) > v4381 { 1.0 } else { 0.0 }) != 0.0 || (if (v4424 / v1989) > v4381 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (v4434 / v2784) > v4381 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v4414 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v4424 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v4434 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v4516: f64;
                        let v4579: f64;
                        let v4586: f64;
                        if v4453 != 0.0 {
                            let v4454 = v4414 / v4424;
                            let v4458 = v378 - v380;
                            let v4460 = v380 - v378;
                            let v4474 = (((-v137) * (v4454.ln())) / v4458) + (((v137 * (v4454 - v5)) * ((v4454.powf((v380 / v4460))) - v5)) / ((((v4454.powf((v378 / v4458))) * v4460) + (v4454 * v378)) - v380));
                            let v4477 = if ((v4425 * v4474).abs()) < v497 { 1.0 } else { 0.0 };
                            let v4517: f64;
                            let v4580: f64;
                            let v4587: f64;
                            if v4477 != 0.0 {
                                let v4482 = v4434 * ((v5 / v382) + ((v3 * v138) * v4474));
                                let v4487 = (((v4483 * v4434) * v4474) * v138) / v382;
                                v4517 = v4482;
                                v4580 = v5;
                                v4587 = v4487;
                            } else {
                                let v4494 = (-v4434) / (((((-v382) * v138) * v4474).exp()) - v5);
                                v4517 = v4494;
                                v4580 = v0;
                                v4587 = v4474;
                            }
                            v4516 = v4517;
                            v4579 = v4580;
                            v4586 = v4587;
                        } else {
                            v4516 = v0;
                            v4579 = v0;
                            v4586 = v5;
                        }
                        v4515 = v4516;
                        v4578 = v4579;
                        v4585 = v4586;
                    } else {
                        v4515 = v0;
                        v4578 = v0;
                        v4585 = v5;
                    }
                    v4510 = v4406;
                    v4514 = v4515;
                    v4556 = v4408;
                    v4577 = v4578;
                    v4584 = v4585;
                } else {
                    v4510 = v0;
                    v4514 = v0;
                    v4556 = v5;
                    v4577 = v0;
                    v4584 = v5;
                }
                let v4496 = v286 * v214;
                let v4497 = v289 * v217;
                let v4499 = v292 * v220;
                let v4501 = v4495 * ((v4496 + v4497) + v4499);
                let v4502 = if v4496 <= v4501 { 1.0 } else { 0.0 };
                let v4616: f64;
                if v4502 != 0.0 {
                    v4616 = v0;
                } else {
                    v4616 = v5;
                }
                let v4503 = if v4497 <= v4501 { 1.0 } else { 0.0 };
                let v4620: f64;
                if v4503 != 0.0 {
                    v4620 = v0;
                } else {
                    v4620 = v5;
                }
                let v4504 = if v4499 <= v4501 { 1.0 } else { 0.0 };
                let v4624: f64;
                if v4504 != 0.0 {
                    v4624 = v0;
                } else {
                    v4624 = v5;
                }
                let v4522: f64;
                let v4525: f64;
                let v4528: f64;
                if v386 != 0.0 {
                    let v4505 = v3 * v298;
                    let v4509 = (v4505 / (v4366 + v4506)).ln();
                    let v4513 = (v4505 / (v4510 + v4506)).ln();
                    let v4521 = (v4505 / ((v4514.abs()) + v4506)).ln();
                    v4522 = v4509;
                    v4525 = v4513;
                    v4528 = v4521;
                } else {
                    v4522 = v0;
                    v4525 = v0;
                    v4528 = v0;
                }
                let v4523 = if v4522 <= v323 { v4522 } else { v323 };
                let v4524 = v4523.exp();
                let v4526 = if v4525 <= v323 { v4525 } else { v323 };
                let v4527 = v4526.exp();
                let v4529 = if v4528 <= v323 { v4528 } else { v323 };
                let v4530 = v4529.exp();
                v4543 = v4523;
                v4545 = v4524;
                v4552 = v4366;
                v4555 = v4556;
                v4564 = v4526;
                v4566 = v4527;
                v4573 = v4510;
                v4576 = v4577;
                v4582 = v4514;
                v4583 = v4584;
                v4600 = v4529;
                v4602 = v4530;
                v4615 = v4616;
                v4619 = v4620;
                v4623 = v4624;
            } else {
                v4543 = v0;
                v4545 = v0;
                v4552 = v0;
                v4555 = v5;
                v4564 = v0;
                v4566 = v0;
                v4573 = v0;
                v4576 = v0;
                v4582 = v0;
                v4583 = v5;
                v4600 = v0;
                v4602 = v0;
                v4615 = v5;
                v4619 = v5;
                v4623 = v5;
            }
            let v4535 = v4531 * (v4532 - v4533);
            let v5440: f64;
            if v375 != 0.0 {
                let v4536 = v4535 * v138;
                let v4538 = if v4536 < v4537 { 1.0 } else { 0.0 };
                let v4551: f64;
                if v4538 != 0.0 {
                    let v4542 = v327 / ((v4539 - v4536) + v5);
                    v4551 = v4542;
                } else {
                    let v4544 = if v4536 > v4543 { 1.0 } else { 0.0 };
                    let v4550: f64;
                    if v4544 != 0.0 {
                        let v4548 = v4545 * ((v4536 - v4543) + v5);
                        v4550 = v4548;
                    } else {
                        let v4549 = v4536.exp();
                        v4550 = v4549;
                    }
                    v4551 = v4550;
                }
                let v4554 = v4552 * (v4551 - v5);
                let v4557 = v4536 * v4555;
                let v4559 = if v4557 < v4558 { 1.0 } else { 0.0 };
                let v4572: f64;
                if v4559 != 0.0 {
                    let v4563 = v327 / ((v4560 - v4557) + v5);
                    v4572 = v4563;
                } else {
                    let v4565 = if v4557 > v4564 { 1.0 } else { 0.0 };
                    let v4571: f64;
                    if v4565 != 0.0 {
                        let v4569 = v4566 * ((v4557 - v4564) + v5);
                        v4571 = v4569;
                    } else {
                        let v4570 = v4557.exp();
                        v4571 = v4570;
                    }
                    v4572 = v4571;
                }
                let v4575 = v4573 * (v4572 - v5);
                let v4581 = if v4576 > v0 { 1.0 } else { 0.0 };
                let v4613: f64;
                if v4581 != 0.0 {
                    let v4590 = v4535 * (v4582 + (v4535 * v4583));
                    v4613 = v4590;
                } else {
                    let v4593 = ((-v4535) * v138) * v4583;
                    let v4595 = if v4593 < v4594 { 1.0 } else { 0.0 };
                    let v4608: f64;
                    if v4595 != 0.0 {
                        let v4599 = v327 / ((v4596 - v4593) + v5);
                        v4608 = v4599;
                    } else {
                        let v4601 = if v4593 > v4600 { 1.0 } else { 0.0 };
                        let v4607: f64;
                        if v4601 != 0.0 {
                            let v4605 = v4602 * ((v4593 - v4600) + v5);
                            v4607 = v4605;
                        } else {
                            let v4606 = v4593.exp();
                            v4607 = v4606;
                        }
                        v4608 = v4607;
                    }
                    let v4611 = (-v4582) * (v4608 - v5);
                    v4613 = v4611;
                }
                let v4614 = (v4554 + v4575) + v4613;
                let v4617 = if v4615 > v3 { 1.0 } else { 0.0 };
                if v4617 != 0.0 {
                    let v4618 = if v27 == v3 { 1.0 } else { 0.0 };
                    if v4618 != 0.0 {
                    } else {
                    }
                } else {
                }
                let v4621 = if v4619 > v3 { 1.0 } else { 0.0 };
                if v4621 != 0.0 {
                    let v4622 = if v29 == v3 { 1.0 } else { 0.0 };
                    if v4622 != 0.0 {
                    } else {
                    }
                } else {
                }
                let v4625 = if v4623 > v3 { 1.0 } else { 0.0 };
                if v4625 != 0.0 {
                    let v4626 = if v31 == v3 { 1.0 } else { 0.0 };
                    if v4626 != 0.0 {
                    } else {
                    }
                } else {
                }
                v5440 = v4614;
            } else {
                let v4629 = if (if (if v353 != 0.0 && v356 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v359 != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                let v4742: f64;
                let v4747: f64;
                let v4749: f64;
                let v4772: f64;
                let v4890: f64;
                let v4938: f64;
                if v4629 != 0.0 {
                    let v4630 = if v4535 < v320 { 1.0 } else { 0.0 };
                    let v4689: f64;
                    let v4692: f64;
                    let v4703: f64;
                    if v4630 != 0.0 {
                        let v4632 = v4535 * v138;
                        let v4635 = if ((v4631 * v4632).abs()) < v323 { 1.0 } else { 0.0 };
                        let v4679: f64;
                        if v4635 != 0.0 {
                            let v4638 = (v4636 * v4632).exp();
                            v4679 = v4638;
                        } else {
                            let v4641 = if (v4639 * v4632) < v0 { 1.0 } else { 0.0 };
                            let v4680: f64;
                            if v4641 != 0.0 {
                                let v4661 = v327 / (v5 + ((v4642 - (v4643 * v4632)) * (v5 + (v3 * ((v4646 - (v4647 * v4632)) * (v5 + ((v4650 - (v4651 * v4632)) * v334)))))));
                                v4680 = v4661;
                            } else {
                                let v4678 = v343 * (v5 + (((v4662 * v4632) - v323) * (v5 + (v3 * (((v4665 * v4632) - v323) * (v5 + (((v4668 * v4632) - v323) * v334)))))));
                                v4680 = v4678;
                            }
                            v4679 = v4680;
                        }
                        let v4681 = v5 / v4679;
                        let v4682 = v4681 * v4681;
                        v4689 = v4682;
                        v4692 = v4679;
                        v4703 = v4681;
                    } else {
                        let v4686 = (v5 + ((v4535 - v320) * v138)) * v444;
                        let v4687 = v4686.sqrt();
                        let v4688 = v5 / v4687;
                        v4689 = v4686;
                        v4692 = v4688;
                        v4703 = v4687;
                    }
                    let v4690 = v4689 - v5;
                    let v4691 = if v4535 > v0 { 1.0 } else { 0.0 };
                    let v4716: f64;
                    if v4691 != 0.0 {
                        let v4701 = v56 * (v137 * (((v56 + v4692) + (((v4692 + v5) * (v4692 + v57)).sqrt())).ln()));
                        v4716 = v4701;
                    } else {
                        let v4715 = (-v4535) + (v56 * (v137 * ((((v56 * v4703) + v5) + (((v5 + v4703) * (v5 + (v57 * v4703))).sqrt())).ln())));
                        v4716 = v4715;
                    }
                    let v4717 = v366 - v4716;
                    let v4719 = v4535 - v4717;
                    let v4726 = v3 * ((v4535 + v4717) - (((v4719 * v4719) + ((v387 * v137) * v137)).sqrt()));
                    let v4728 = v4535 - v373;
                    let v4735 = v3 * ((v4535 + v373) - (((v4728 * v4728) + ((v387 * v11) * v11)).sqrt()));
                    let v4741 = v3 * (v4535 - (((v4535 * v4535) + v4737).sqrt()));
                    v4742 = v4690;
                    v4747 = v4726;
                    v4749 = v4716;
                    v4772 = v4703;
                    v4890 = v4735;
                    v4938 = v4741;
                } else {
                    v4742 = v0;
                    v4747 = v0;
                    v4749 = v0;
                    v4772 = v0;
                    v4890 = v0;
                    v4938 = v0;
                }
                let v5001: f64;
                let v5004: f64;
                let v5027: f64;
                let v5110: f64;
                let v5431: f64;
                if v353 != 0.0 {
                    v5001 = v0;
                    v5004 = v0;
                    v5027 = v0;
                    v5110 = v0;
                    v5431 = v0;
                } else {
                    let v4743 = v168 * v4742;
                    let v4745 = if v507 == v0 { 1.0 } else { 0.0 };
                    let v4746 = if (if v505 == v0 { 1.0 } else { 0.0 }) != 0.0 && v4745 != 0.0 { 1.0 } else { 0.0 };
                    let v4778: f64;
                    let v4780: f64;
                    let v4802: f64;
                    let v4884: f64;
                    let v4957: f64;
                    if v4746 != 0.0 {
                        v4778 = v0;
                        v4780 = v0;
                        v4802 = v0;
                        v4884 = v0;
                        v4957 = v0;
                    } else {
                        let v4748 = v194 - v4747;
                        let v4753 = v5 - ((v5 - (v4749 / v4748)).sqrt());
                        let v4754 = if v26 == v3 { 1.0 } else { 0.0 };
                        let v4764: f64;
                        if v4754 != 0.0 {
                            v4764 = v0;
                        } else {
                            let v4763 = ((((v4753 * v4753) * (v4753.ln())) / (v5 - v4753)) + v4753) * (v5 - (v56 * v26));
                            v4764 = v4763;
                        }
                        let v4765 = v4753 + v4764;
                        let v4770: f64;
                        if v4754 != 0.0 {
                            let v4767 = (v4748 * v49).sqrt();
                            v4770 = v4767;
                        } else {
                            let v4769 = (v4748 * v49).powf(v26);
                            v4770 = v4769;
                        }
                        let v4771 = v36 * v4770;
                        let v4775 = v153 * ((v4772 - v5) * v4771);
                        let v4777 = v505 * (v4775 * v4765);
                        v4778 = v4771;
                        v4780 = v4748;
                        v4802 = v4765;
                        v4884 = v4775;
                        v4957 = v4777;
                    }
                    let v4959: f64;
                    if v4745 != 0.0 {
                        v4959 = v0;
                    } else {
                        let v4782 = v241 * ((v4778 * v27) / v4780);
                        let v4784 = (v546 * v227) / v4782;
                        let v4785 = v4784 * v4784;
                        let v4786 = v4785 * v4785;
                        let v4789 = (v4786 / (v4786 + v5)).sqrt();
                        let v4790 = v4789.sqrt();
                        let v4791 = v4789 * v4790;
                        let v4793 = (-v26) * v32;
                        let v4795 = if v4793 == v4794 { 1.0 } else { 0.0 };
                        let v4803: f64;
                        if v4795 != 0.0 {
                            let v4798 = v5 / (v5 + (v4782 * v4791));
                            v4803 = v4798;
                        } else {
                            let v4801 = (v5 + (v4782 * v4791)).powf(v4793);
                            v4803 = v4801;
                        }
                        let v4806 = (v4802 * v4803) / (v4802 + v4803);
                        let v4809 = (v571 * (v4782 / v4790)).sqrt();
                        let v4819 = (((v227 * v4784) * v4790) - (v227 * v4789)) + (v3 * (v4782 * v4791));
                        let v4821 = (((v56 * (v4784 * v4790)) - v4789) - v5) * v4809;
                        let v4822 = v4821 * v4821;
                        let v4823 = if v4821 > v0 { 1.0 } else { 0.0 };
                        let v4849: f64;
                        if v4823 != 0.0 {
                            let v4826 = v5 / (v5 + (v55 * v4821));
                            v4849 = v4826;
                        } else {
                            let v4829 = v5 / (v5 - (v55 * v4821));
                            v4849 = v4829;
                        }
                        let v4831 = (-v4822) + v4819;
                        let v4833 = if v4831 > v4832 { 1.0 } else { 0.0 };
                        let v4857: f64;
                        if v4833 != 0.0 {
                            let v4834 = v4831.exp();
                            v4857 = v4834;
                        } else {
                            let v4848 = v327 / (v5 + ((v4835 - v4831) * (v5 + (v3 * ((v4837 - v4831) * (v5 + ((v4839 - v4831) * v334)))))));
                            v4857 = v4848;
                        }
                        let v4851 = v4849 * v4849;
                        let v4858 = (((v54 * v4849) + (v58 * v4851)) + (v59 * (v4851 * v4849))) * v4857;
                        let v4880: f64;
                        if v4823 != 0.0 {
                            v4880 = v4858;
                        } else {
                            let v4860 = if v4819 > v4859 { 1.0 } else { 0.0 };
                            let v4876: f64;
                            if v4860 != 0.0 {
                                let v4861 = v4819.exp();
                                v4876 = v4861;
                            } else {
                                let v4875 = v327 / (v5 + ((v4862 - v4819) * (v5 + (v3 * ((v4864 - v4819) * (v5 + ((v4866 - v4819) * v334)))))));
                                v4876 = v4875;
                            }
                            let v4878 = (v56 * v4876) - v4858;
                            v4880 = v4878;
                        }
                        let v4887 = v507 * ((v4884 * (v4879 * ((v227 * v4880) / v4809))) * v4806);
                        v4959 = v4887;
                    }
                    let v4888 = if v653 == v0 { 1.0 } else { 0.0 };
                    let v4961: f64;
                    if v4888 != 0.0 {
                        v4961 = v0;
                    } else {
                        let v4889 = if v26 == v3 { 1.0 } else { 0.0 };
                        let v4899: f64;
                        if v4889 != 0.0 {
                            let v4893 = ((v48 - v4890) * v49).sqrt();
                            v4899 = v4893;
                        } else {
                            let v4896 = ((v48 - v4890) * v49).powf(v26);
                            v4899 = v4896;
                        }
                        let v4901 = v32 * (((v48 - v4890) * v45) / v4899);
                        let v4903 = (-v279) / v4901;
                        let v4905 = if (v4903.abs()) < v323 { 1.0 } else { 0.0 };
                        let v4933: f64;
                        if v4905 != 0.0 {
                            let v4906 = v4903.exp();
                            v4933 = v4906;
                        } else {
                            let v4907 = if v4903 < v0 { 1.0 } else { 0.0 };
                            let v4934: f64;
                            if v4907 != 0.0 {
                                let v4921 = v327 / (v5 + ((v4908 - v4903) * (v5 + (v3 * ((v4910 - v4903) * (v5 + ((v4912 - v4903) * v334)))))));
                                v4934 = v4921;
                            } else {
                                let v4922 = v4903 - v323;
                                let v4930 = v343 * (v5 + (v4922 * (v5 + (v3 * (v4922 * (v5 + (v4922 * v334)))))));
                                v4934 = v4930;
                            }
                            v4933 = v4934;
                        }
                        let v4936 = v653 * (((v4535 * v4901) * v4901) * v4933);
                        v4961 = v4936;
                    }
                    let v4937 = if v75 > v703 { 1.0 } else { 0.0 };
                    let v4964: f64;
                    if v4937 != 0.0 {
                        v4964 = v5;
                    } else {
                        let v4941 = if v4938 > ((-v62) * v75) { 1.0 } else { 0.0 };
                        let v4965: f64;
                        if v4941 != 0.0 {
                            let v4942 = if v63 == v387 { 1.0 } else { 0.0 };
                            let v4950: f64;
                            if v4942 != 0.0 {
                                let v4943 = v4938 * v76;
                                let v4946 = ((v4943 * v4943) * v4943) * v4943;
                                v4950 = v4946;
                            } else {
                                let v4949 = ((v4938 * v76).abs()).powf(v63);
                                v4950 = v4949;
                            }
                            let v4952 = v5 / (v5 - v4950);
                            v4965 = v4952;
                        } else {
                            let v4956 = v66 + ((v4938 + (v62 * v75)) * v87);
                            v4965 = v4956;
                        }
                        v4964 = v4965;
                    }
                    let v4966 = (v724 * (((v4743 + v4957) + v4959) + v4961)) * v4964;
                    let v4967 = if v27 == v3 { 1.0 } else { 0.0 };
                    if v4967 != 0.0 {
                    } else {
                    }
                    v5001 = v4778;
                    v5004 = v4780;
                    v5027 = v4802;
                    v5110 = v4884;
                    v5431 = v4966;
                }
                let v5225: f64;
                let v5228: f64;
                let v5251: f64;
                let v5334: f64;
                let v5433: f64;
                if v356 != 0.0 {
                    v5225 = v5001;
                    v5228 = v5004;
                    v5251 = v5027;
                    v5334 = v5110;
                    v5433 = v0;
                } else {
                    let v4968 = v171 * v4742;
                    let v4970 = if v738 == v0 { 1.0 } else { 0.0 };
                    let v4971 = if (if v736 == v0 { 1.0 } else { 0.0 }) != 0.0 && v4970 != 0.0 { 1.0 } else { 0.0 };
                    let v5000: f64;
                    let v5003: f64;
                    let v5026: f64;
                    let v5109: f64;
                    let v5181: f64;
                    if v4971 != 0.0 {
                        v5000 = v5001;
                        v5003 = v5004;
                        v5026 = v5027;
                        v5109 = v5110;
                        v5181 = v0;
                    } else {
                        let v4972 = v201 - v4747;
                        let v4976 = v5 - ((v5 - (v4749 / v4972)).sqrt());
                        let v4977 = if v28 == v3 { 1.0 } else { 0.0 };
                        let v4987: f64;
                        if v4977 != 0.0 {
                            v4987 = v0;
                        } else {
                            let v4986 = ((((v4976 * v4976) * (v4976.ln())) / (v5 - v4976)) + v4976) * (v5 - (v56 * v28));
                            v4987 = v4986;
                        }
                        let v4988 = v4976 + v4987;
                        let v4993: f64;
                        if v4977 != 0.0 {
                            let v4990 = (v4972 * v51).sqrt();
                            v4993 = v4990;
                        } else {
                            let v4992 = (v4972 * v51).powf(v28);
                            v4993 = v4992;
                        }
                        let v4994 = v40 * v4993;
                        let v4997 = v159 * ((v4772 - v5) * v4994);
                        let v4999 = v736 * (v4997 * v4988);
                        v5000 = v4994;
                        v5003 = v4972;
                        v5026 = v4988;
                        v5109 = v4997;
                        v5181 = v4999;
                    }
                    let v5183: f64;
                    if v4970 != 0.0 {
                        v5183 = v0;
                    } else {
                        let v5006 = v251 * ((v5000 * v29) / v5003);
                        let v5008 = (v546 * v228) / v5006;
                        let v5009 = v5008 * v5008;
                        let v5010 = v5009 * v5009;
                        let v5013 = (v5010 / (v5010 + v5)).sqrt();
                        let v5014 = v5013.sqrt();
                        let v5015 = v5013 * v5014;
                        let v5017 = (-v28) * v33;
                        let v5019 = if v5017 == v5018 { 1.0 } else { 0.0 };
                        let v5028: f64;
                        if v5019 != 0.0 {
                            let v5022 = v5 / (v5 + (v5006 * v5015));
                            v5028 = v5022;
                        } else {
                            let v5025 = (v5 + (v5006 * v5015)).powf(v5017);
                            v5028 = v5025;
                        }
                        let v5031 = (v5026 * v5028) / (v5026 + v5028);
                        let v5034 = (v571 * (v5006 / v5014)).sqrt();
                        let v5044 = (((v228 * v5008) * v5014) - (v228 * v5013)) + (v3 * (v5006 * v5015));
                        let v5046 = (((v56 * (v5008 * v5014)) - v5013) - v5) * v5034;
                        let v5047 = v5046 * v5046;
                        let v5048 = if v5046 > v0 { 1.0 } else { 0.0 };
                        let v5074: f64;
                        if v5048 != 0.0 {
                            let v5051 = v5 / (v5 + (v55 * v5046));
                            v5074 = v5051;
                        } else {
                            let v5054 = v5 / (v5 - (v55 * v5046));
                            v5074 = v5054;
                        }
                        let v5056 = (-v5047) + v5044;
                        let v5058 = if v5056 > v5057 { 1.0 } else { 0.0 };
                        let v5082: f64;
                        if v5058 != 0.0 {
                            let v5059 = v5056.exp();
                            v5082 = v5059;
                        } else {
                            let v5073 = v327 / (v5 + ((v5060 - v5056) * (v5 + (v3 * ((v5062 - v5056) * (v5 + ((v5064 - v5056) * v334)))))));
                            v5082 = v5073;
                        }
                        let v5076 = v5074 * v5074;
                        let v5083 = (((v54 * v5074) + (v58 * v5076)) + (v59 * (v5076 * v5074))) * v5082;
                        let v5105: f64;
                        if v5048 != 0.0 {
                            v5105 = v5083;
                        } else {
                            let v5085 = if v5044 > v5084 { 1.0 } else { 0.0 };
                            let v5101: f64;
                            if v5085 != 0.0 {
                                let v5086 = v5044.exp();
                                v5101 = v5086;
                            } else {
                                let v5100 = v327 / (v5 + ((v5087 - v5044) * (v5 + (v3 * ((v5089 - v5044) * (v5 + ((v5091 - v5044) * v334)))))));
                                v5101 = v5100;
                            }
                            let v5103 = (v56 * v5101) - v5083;
                            v5105 = v5103;
                        }
                        let v5113 = v738 * ((v5109 * (v5104 * ((v228 * v5105) / v5034))) * v5031);
                        v5183 = v5113;
                    }
                    let v5114 = if v883 == v0 { 1.0 } else { 0.0 };
                    let v5185: f64;
                    if v5114 != 0.0 {
                        v5185 = v0;
                    } else {
                        let v5115 = if v28 == v3 { 1.0 } else { 0.0 };
                        let v5124: f64;
                        if v5115 != 0.0 {
                            let v5118 = ((v50 - v4890) * v51).sqrt();
                            v5124 = v5118;
                        } else {
                            let v5121 = ((v50 - v4890) * v51).powf(v28);
                            v5124 = v5121;
                        }
                        let v5126 = v33 * (((v50 - v4890) * v46) / v5124);
                        let v5128 = (-v281) / v5126;
                        let v5130 = if (v5128.abs()) < v323 { 1.0 } else { 0.0 };
                        let v5158: f64;
                        if v5130 != 0.0 {
                            let v5131 = v5128.exp();
                            v5158 = v5131;
                        } else {
                            let v5132 = if v5128 < v0 { 1.0 } else { 0.0 };
                            let v5159: f64;
                            if v5132 != 0.0 {
                                let v5146 = v327 / (v5 + ((v5133 - v5128) * (v5 + (v3 * ((v5135 - v5128) * (v5 + ((v5137 - v5128) * v334)))))));
                                v5159 = v5146;
                            } else {
                                let v5147 = v5128 - v323;
                                let v5155 = v343 * (v5 + (v5147 * (v5 + (v3 * (v5147 * (v5 + (v5147 * v334)))))));
                                v5159 = v5155;
                            }
                            v5158 = v5159;
                        }
                        let v5161 = v883 * (((v4535 * v5126) * v5126) * v5158);
                        v5185 = v5161;
                    }
                    let v5162 = if v77 > v703 { 1.0 } else { 0.0 };
                    let v5188: f64;
                    if v5162 != 0.0 {
                        v5188 = v5;
                    } else {
                        let v5165 = if v4938 > ((-v62) * v77) { 1.0 } else { 0.0 };
                        let v5189: f64;
                        if v5165 != 0.0 {
                            let v5166 = if v67 == v387 { 1.0 } else { 0.0 };
                            let v5174: f64;
                            if v5166 != 0.0 {
                                let v5167 = v4938 * v78;
                                let v5170 = ((v5167 * v5167) * v5167) * v5167;
                                v5174 = v5170;
                            } else {
                                let v5173 = ((v4938 * v78).abs()).powf(v67);
                                v5174 = v5173;
                            }
                            let v5176 = v5 / (v5 - v5174);
                            v5189 = v5176;
                        } else {
                            let v5180 = v70 + ((v4938 + (v62 * v77)) * v94);
                            v5189 = v5180;
                        }
                        v5188 = v5189;
                    }
                    let v5190 = (v724 * (((v4968 + v5181) + v5183) + v5185)) * v5188;
                    let v5191 = if v29 == v3 { 1.0 } else { 0.0 };
                    if v5191 != 0.0 {
                    } else {
                    }
                    v5225 = v5000;
                    v5228 = v5003;
                    v5251 = v5026;
                    v5334 = v5109;
                    v5433 = v5190;
                }
                let v5436: f64;
                if v359 != 0.0 {
                    v5436 = v0;
                } else {
                    let v5192 = v174 * v4742;
                    let v5194 = if v964 == v0 { 1.0 } else { 0.0 };
                    let v5195 = if (if v962 == v0 { 1.0 } else { 0.0 }) != 0.0 && v5194 != 0.0 { 1.0 } else { 0.0 };
                    let v5224: f64;
                    let v5227: f64;
                    let v5250: f64;
                    let v5333: f64;
                    let v5405: f64;
                    if v5195 != 0.0 {
                        v5224 = v5225;
                        v5227 = v5228;
                        v5250 = v5251;
                        v5333 = v5334;
                        v5405 = v0;
                    } else {
                        let v5196 = v208 - v4747;
                        let v5200 = v5 - ((v5 - (v4749 / v5196)).sqrt());
                        let v5201 = if v30 == v3 { 1.0 } else { 0.0 };
                        let v5211: f64;
                        if v5201 != 0.0 {
                            v5211 = v0;
                        } else {
                            let v5210 = ((((v5200 * v5200) * (v5200.ln())) / (v5 - v5200)) + v5200) * (v5 - (v56 * v30));
                            v5211 = v5210;
                        }
                        let v5212 = v5200 + v5211;
                        let v5217: f64;
                        if v5201 != 0.0 {
                            let v5214 = (v5196 * v53).sqrt();
                            v5217 = v5214;
                        } else {
                            let v5216 = (v5196 * v53).powf(v30);
                            v5217 = v5216;
                        }
                        let v5218 = v44 * v5217;
                        let v5221 = v165 * ((v4772 - v5) * v5218);
                        let v5223 = v962 * (v5221 * v5212);
                        v5224 = v5218;
                        v5227 = v5196;
                        v5250 = v5212;
                        v5333 = v5221;
                        v5405 = v5223;
                    }
                    let v5407: f64;
                    if v5194 != 0.0 {
                        v5407 = v0;
                    } else {
                        let v5230 = v261 * ((v5224 * v31) / v5227);
                        let v5232 = (v546 * v229) / v5230;
                        let v5233 = v5232 * v5232;
                        let v5234 = v5233 * v5233;
                        let v5237 = (v5234 / (v5234 + v5)).sqrt();
                        let v5238 = v5237.sqrt();
                        let v5239 = v5237 * v5238;
                        let v5241 = (-v30) * v34;
                        let v5243 = if v5241 == v5242 { 1.0 } else { 0.0 };
                        let v5252: f64;
                        if v5243 != 0.0 {
                            let v5246 = v5 / (v5 + (v5230 * v5239));
                            v5252 = v5246;
                        } else {
                            let v5249 = (v5 + (v5230 * v5239)).powf(v5241);
                            v5252 = v5249;
                        }
                        let v5255 = (v5250 * v5252) / (v5250 + v5252);
                        let v5258 = (v571 * (v5230 / v5238)).sqrt();
                        let v5268 = (((v229 * v5232) * v5238) - (v229 * v5237)) + (v3 * (v5230 * v5239));
                        let v5270 = (((v56 * (v5232 * v5238)) - v5237) - v5) * v5258;
                        let v5271 = v5270 * v5270;
                        let v5272 = if v5270 > v0 { 1.0 } else { 0.0 };
                        let v5298: f64;
                        if v5272 != 0.0 {
                            let v5275 = v5 / (v5 + (v55 * v5270));
                            v5298 = v5275;
                        } else {
                            let v5278 = v5 / (v5 - (v55 * v5270));
                            v5298 = v5278;
                        }
                        let v5280 = (-v5271) + v5268;
                        let v5282 = if v5280 > v5281 { 1.0 } else { 0.0 };
                        let v5306: f64;
                        if v5282 != 0.0 {
                            let v5283 = v5280.exp();
                            v5306 = v5283;
                        } else {
                            let v5297 = v327 / (v5 + ((v5284 - v5280) * (v5 + (v3 * ((v5286 - v5280) * (v5 + ((v5288 - v5280) * v334)))))));
                            v5306 = v5297;
                        }
                        let v5300 = v5298 * v5298;
                        let v5307 = (((v54 * v5298) + (v58 * v5300)) + (v59 * (v5300 * v5298))) * v5306;
                        let v5329: f64;
                        if v5272 != 0.0 {
                            v5329 = v5307;
                        } else {
                            let v5309 = if v5268 > v5308 { 1.0 } else { 0.0 };
                            let v5325: f64;
                            if v5309 != 0.0 {
                                let v5310 = v5268.exp();
                                v5325 = v5310;
                            } else {
                                let v5324 = v327 / (v5 + ((v5311 - v5268) * (v5 + (v3 * ((v5313 - v5268) * (v5 + ((v5315 - v5268) * v334)))))));
                                v5325 = v5324;
                            }
                            let v5327 = (v56 * v5325) - v5307;
                            v5329 = v5327;
                        }
                        let v5337 = v964 * ((v5333 * (v5328 * ((v229 * v5329) / v5258))) * v5255);
                        v5407 = v5337;
                    }
                    let v5338 = if v1109 == v0 { 1.0 } else { 0.0 };
                    let v5409: f64;
                    if v5338 != 0.0 {
                        v5409 = v0;
                    } else {
                        let v5339 = if v30 == v3 { 1.0 } else { 0.0 };
                        let v5348: f64;
                        if v5339 != 0.0 {
                            let v5342 = ((v52 - v4890) * v53).sqrt();
                            v5348 = v5342;
                        } else {
                            let v5345 = ((v52 - v4890) * v53).powf(v30);
                            v5348 = v5345;
                        }
                        let v5350 = v34 * (((v52 - v4890) * v47) / v5348);
                        let v5352 = (-v283) / v5350;
                        let v5354 = if (v5352.abs()) < v323 { 1.0 } else { 0.0 };
                        let v5382: f64;
                        if v5354 != 0.0 {
                            let v5355 = v5352.exp();
                            v5382 = v5355;
                        } else {
                            let v5356 = if v5352 < v0 { 1.0 } else { 0.0 };
                            let v5383: f64;
                            if v5356 != 0.0 {
                                let v5370 = v327 / (v5 + ((v5357 - v5352) * (v5 + (v3 * ((v5359 - v5352) * (v5 + ((v5361 - v5352) * v334)))))));
                                v5383 = v5370;
                            } else {
                                let v5371 = v5352 - v323;
                                let v5379 = v343 * (v5 + (v5371 * (v5 + (v3 * (v5371 * (v5 + (v5371 * v334)))))));
                                v5383 = v5379;
                            }
                            v5382 = v5383;
                        }
                        let v5385 = v1109 * (((v4535 * v5350) * v5350) * v5382);
                        v5409 = v5385;
                    }
                    let v5386 = if v79 > v703 { 1.0 } else { 0.0 };
                    let v5412: f64;
                    if v5386 != 0.0 {
                        v5412 = v5;
                    } else {
                        let v5389 = if v4938 > ((-v62) * v79) { 1.0 } else { 0.0 };
                        let v5413: f64;
                        if v5389 != 0.0 {
                            let v5390 = if v71 == v387 { 1.0 } else { 0.0 };
                            let v5398: f64;
                            if v5390 != 0.0 {
                                let v5391 = v4938 * v80;
                                let v5394 = ((v5391 * v5391) * v5391) * v5391;
                                v5398 = v5394;
                            } else {
                                let v5397 = ((v4938 * v80).abs()).powf(v71);
                                v5398 = v5397;
                            }
                            let v5400 = v5 / (v5 - v5398);
                            v5413 = v5400;
                        } else {
                            let v5404 = v74 + ((v4938 + (v62 * v79)) * v101);
                            v5413 = v5404;
                        }
                        v5412 = v5413;
                    }
                    let v5414 = (v724 * (((v5192 + v5405) + v5407) + v5409)) * v5412;
                    if v114 != 0.0 {
                        let v5416 = if v4535 < v5415 { 1.0 } else { 0.0 };
                        if v5416 != 0.0 {
                            let v5422 = if ((v4535 - v5415) / v5418) < v5421 { 1.0 } else { 0.0 };
                            if v5422 != 0.0 {
                            } else {
                            }
                        } else {
                            let v5425 = if ((v4535 - v5415) / v5418) > v5420 { 1.0 } else { 0.0 };
                            if v5425 != 0.0 {
                            } else {
                            }
                        }
                        let v5426 = if v31 == v3 { 1.0 } else { 0.0 };
                        if v5426 != 0.0 {
                        } else {
                        }
                        let v5429 = if v5427 == v3 { 1.0 } else { 0.0 };
                        if v5429 != 0.0 {
                        } else {
                        }
                    } else {
                        let v5430 = if v31 == v3 { 1.0 } else { 0.0 };
                        if v5430 != 0.0 {
                        } else {
                        }
                    }
                    v5436 = v5414;
                }
                let v5438 = ((v286 * v5431) + (v289 * v5433)) + (v292 * v5436);
                v5440 = v5438;
            }
            let v5445 = (v295 * v5439) * (v5441 * (v5440.abs()));
            if v375 != 0.0 {
            } else {
            }
        {
            let psd = v5445;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
