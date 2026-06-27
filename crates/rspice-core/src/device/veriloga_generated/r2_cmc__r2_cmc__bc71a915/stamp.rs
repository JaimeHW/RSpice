#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

use crate::device::veriloga_generated::support::{AdValue as GenericAdValue, ReactiveScratch as GenericReactiveScratch, Scratch as GenericScratch};

type A = GenericAdValue<{ Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type Scratch = GenericScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type ReactiveScratch = GenericReactiveScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

#[inline]
fn eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    ddt_active: bool,
    ddt_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    current[slot] = value;
    if ddt_active {
        (value - previous_value) * ddt_scale
    } else {
        previous[slot] = value;
        initialized[slot] = true;
        0.0
    }
}

#[inline]
fn ddt_jacobian(timestep: f64, derivative: f64) -> f64 {
    if timestep.abs() > Instance::DDT_EPSILON {
        derivative / timestep
    } else {
        0.0
    }
}

#[inline]
fn eval_idt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    ddt_active: bool,
    idt_scale: f64,
    slot: usize,
    value: f64,
    ic: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated idt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { ic };
    let current_value = if ddt_active {
        previous_value + value * idt_scale
    } else {
        ic
    };
    current[slot] = current_value;
    if !ddt_active {
        previous[slot] = current_value;
        initialized[slot] = true;
    }
    current_value
}

#[inline]
fn idt_jacobian(timestep: f64, derivative: f64) -> f64 {
    if timestep.abs() > Instance::DDT_EPSILON {
        derivative * timestep
    } else {
        0.0
    }
}

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let ctx_temp = ctx.temperature();
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let v2: f64 = 0.0;
        let v5: f64 = 1.0;
        let v8: f64 = 0.01;
        let v18: f64 = 273.15;
        let v21: f64 = ctx_temp;
        let v23: f64 = (v21 + self.scalar_v22);
        let v24: f64 = (v23 - v18);
        let v27: bool = (v24 < self.scalar_v26);
        let v28: f64 = (v24 - self.scalar_v25);
        let v29: f64 = (v28 - v5);
        let v30: f64 = v29.exp();
        let v31: f64 = (self.scalar_v25 + v30);
        let v32: f64 = (if v27 { v31 } else { v24 });
        let v35: bool = (v32 > self.scalar_v34);
        let v36: bool = (!v27);
        let v37: bool = (v36 && v35);
        let v38: f64 = (self.scalar_v33 - v32);
        let v39: f64 = (v38 - v5);
        let v40: f64 = v39.exp();
        let v41: f64 = (self.scalar_v33 - v40);
        let v42: f64 = (if v37 { v41 } else { v32 });
        let v43: bool = (!v35);
        let v44: bool = (v36 && v43);
        let v45: f64 = (if v44 { v42 } else { v42 });
        let v46: f64 = (v45 + v18);
        let v47: f64 = (v46 - self.scalar_v20);
        let v223: f64 = (v47 * self.scalar_v222);
        let v224: f64 = (self.scalar_v218 + v223);
        let v225: f64 = (v47 * v224);
        let v226: f64 = (v5 + v225);
        let v227: f64 = 0.1;
        let v228: f64 = 0.11;
        let v229: bool = (v226 < v228);
        let v230: f64 = 10.0;
        let v231: f64 = (v226 - v8);
        let v232: f64 = (v230 * v231);
        let v233: f64 = (v232 - v5);
        let v234: f64 = v233.exp();
        let v235: f64 = (v227 * v234);
        let v236: f64 = (v8 + v235);
        let v237: f64 = (if v229 { v236 } else { v226 });
        let v238: bool = (!v229);
        let v239: f64 = (if v238 { v237 } else { v237 });
        let v240: f64 = (self.scalar_v180 * v239);
        let v241: f64 = nv0;
        let v242: f64 = nv1;
        let v243: f64 = (v241 - v242);
        let v245: f64 = (v243 / self.scalar_v187);
        let v246: f64 = (if self.scalar_v244 { v245 } else { v2 });
        let v248: f64 = (self.scalar_v247 * v246);
        let v249: f64 = (if self.scalar_v244 { v248 } else { v2 });
        let v250: f64 = (v249 * v249);
        let v251: f64 = (v5 + v250);
        let v252: f64 = v251.sqrt();
        let v253: f64 = (if self.scalar_v244 { v252 } else { v2 });
        let v255: f64 = v246.abs();
        let v256: f64 = (self.scalar_v254 * v255);
        let v257: f64 = (if self.scalar_v244 { v256 } else { v2 });
        let v258: f64 = (v257 * v257);
        let v259: f64 = (v258 * v257);
        let v260: f64 = (v5 + v259);
        let v261: f64 = 0.3333333333333333;
        let v262: f64 = f64::powf(v260, v261);
        let v263: f64 = (if self.scalar_v244 { v262 } else { v2 });
        let v266: f64 = (self.scalar_v189 * v253);
        let v267: f64 = (self.scalar_v265 + v266);
        let v268: f64 = (self.scalar_v191 * v263);
        let v269: f64 = (v267 + v268);
        let v270: f64 = (if self.scalar_v244 { v269 } else { v2 });
        let v272: f64 = (if self.scalar_v271 { v5 } else { v270 });
        let v273: f64 = (v240 * v272);
        let v274: f64 = (v243 / v273);
        let v275: f64 = -1.0;
        let v284: f64 = (self.scalar_v282 * v249);
        let v285: f64 = (v249 * self.scalar_v282);
        let v286: f64 = (v284 + v285);
        let v287: f64 = (self.scalar_v283 * v249);
        let v288: f64 = (v249 * self.scalar_v283);
        let v289: f64 = (v287 + v288);
        let v290: f64 = 2.0;
        let v291: f64 = (v290 * v252);
        let v292: f64 = (v286 / v291);
        let v293: f64 = (v289 / v291);
        let v294: f64 = (if self.scalar_v244 { v292 } else { v2 });
        let v295: f64 = (if self.scalar_v244 { v293 } else { v2 });
        let v296: f64 = (self.scalar_v189 * v294);
        let v297: f64 = (self.scalar_v189 * v295);
        let v298: f64 = (if self.scalar_v244 { v296 } else { v2 });
        let v299: f64 = (if self.scalar_v244 { v297 } else { v2 });
        let v300: f64 = (if self.scalar_v271 { v2 } else { v298 });
        let v301: f64 = (if self.scalar_v271 { v2 } else { v299 });
        let v302: f64 = (v240 * v300);
        let v303: f64 = (v240 * v301);
        let v304: f64 = (v243 * v302);
        let v305: f64 = (v273 - v304);
        let v306: f64 = (v273 * v273);
        let v307: f64 = (v305 / v306);
        let v308: f64 = (v275 * v273);
        let v309: f64 = (v243 * v303);
        let v310: f64 = (v308 - v309);
        let v311: f64 = (v310 / v306);

        let d274_dn0: f64 = v307;
        let d274_dn1: f64 = v311;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(1),
            multiplicity * (v274),
            0,
            multiplicity * (d274_dn0),
            1,
            multiplicity * (d274_dn1),
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        s.b[39] = param_given[9];
        s.v[39] = if s.b[39] { 1.0 } else { 0.0 };

        if s.b[39] {
            s.store_scalar(10, p.p9);
        }

        if (!s.b[39]) {
            s.store_scalar(10, 1.0);
        }

        s.b[40] = param_given[10];
        s.v[40] = if s.b[40] { 1.0 } else { 0.0 };

        if s.b[40] {
            s.store_scalar(11, (1.0 - (0.01 * p.p10)));
        }

        if (!s.b[40]) {
            s.store_scalar(11, 1.0);
        }

        s.store_scaled_mul(15, 11, 10, 1000000.0);

        s.v[8] = (273.15 + p.p15);

        s.v[25] = ((ctx_temp + p.p5) - 273.15);

        s.b[44] = (s.v[25] < (p.p34 + 1.0));
        s.v[44] = if s.b[44] { 1.0 } else { 0.0 };

        if s.b[44] {
            s.store_scalar(25, (p.p34 + ((((s.v[25] - p.p34) - 1.0)) as f64).exp()));
        }

        s.b[45] = (s.v[25] > (p.p35 - 1.0));
        s.v[45] = if s.b[45] { 1.0 } else { 0.0 };

        if ((!s.b[44]) && s.b[45]) {
            s.store_sub_from_scalar_ad(25, p.p35, A::exp(A::offset(A::sub_from_scalar(p.p35, s.ad_value(25)), (-1.0))));
        }

        if ((!s.b[44]) && (!s.b[45])) {
        }

        s.store_offset(9, 25, 273.15);

        s.store_offset(12, 9, (-s.v[8]));

        s.store_offset_scaled(22, 12, ((p.p42) * (p.p29)), p.p29);

        s.b[46] = (s.v[22] < 0.0);
        s.v[46] = if s.b[46] { 1.0 } else { 0.0 };

        if s.b[46] {
            s.store_scalar(22, 0.0);
        }

        s.b[47] = ((p.p3 != 0.0) && (p.p4 != 0.0));
        s.v[47] = if s.b[47] { 1.0 } else { 0.0 };

        if s.b[47] {
            s.store_scalar(14, p.p22);
        }

        s.b[48] = ((p.p3 != 0.0) || (p.p4 != 0.0));
        s.v[48] = if s.b[48] { 1.0 } else { 0.0 };

        if ((!s.b[47]) && s.b[48]) {
            s.store_scalar(14, (p.p22 * 0.5));
        }

        if ((!s.b[47]) && (!s.b[48])) {
            s.store_scalar(14, 0.0);
        }

        s.b[49] = ((param_given[1] && param_given[2]) && (!param_given[0]));
        s.v[49] = if s.b[49] { 1.0 } else { 0.0 };

        s.b[50] = ((p.p2 == 0.0) || (p.p1 == 0.0));
        s.v[50] = if s.b[50] { 1.0 } else { 0.0 };

        if (s.b[49] && s.b[50]) {
            s.store_scalar(16, 0.0);
            s.store_scalar(3, 0.0);
            s.store_scale(17, 15, p.p0);
            s.store_offset(4, 17, p.p21);
            s.store_scalar(5, 0.0);
            s.store_scalar(19, 1e99);
        }

        if (s.b[49] && (!s.b[50])) {
            s.store_scale(16, 15, p.p1);
            s.store_add(3, 16, 14);
        }

        s.b[52] = (s.v[3] > 0.0);
        s.v[52] = if s.b[52] { 1.0 } else { 0.0 };

        if ((s.b[49] && (!s.b[50])) && s.b[52]) {
            s.store_scale(4, 3, (p.p16 / p.p2));
            s.store_offset(17, 4, (-p.p21));
            s.store_scalar(5, p.p2);
            s.store_div_from_scalar(19, 1.0, 5);
        }

        if ((s.b[49] && (!s.b[50])) && (!s.b[52])) {
            s.store_scale(17, 15, p.p0);
            s.store_offset(4, 17, p.p21);
            s.store_scalar(5, 0.0);
            s.store_scalar(19, 1e99);
        }

        s.b[54] = (param_given[2] && (!param_given[1]));
        s.v[54] = if s.b[54] { 1.0 } else { 0.0 };

        s.b[55] = (p.p2 == 0.0);
        s.v[55] = if s.b[55] { 1.0 } else { 0.0 };

        if (((!s.b[49]) && s.b[54]) && s.b[55]) {
            s.store_scalar(16, 0.0);
            s.store_scalar(3, 0.0);
            s.store_scale(17, 15, p.p0);
            s.store_offset(4, 17, p.p21);
            s.store_scalar(5, 0.0);
            s.store_scalar(19, 1e99);
        }

        s.b[56] = (p.p0 == 0.0);
        s.v[56] = if s.b[56] { 1.0 } else { 0.0 };

        if ((((!s.b[49]) && s.b[54]) && (!s.b[55])) && s.b[56]) {
            s.store_scalar(17, 0.0);
            s.store_scalar(4, 0.0);
            s.store_scale(16, 15, p.p1);
            s.store_add(3, 16, 14);
            s.store_scalar(5, 1e99);
            s.store_scalar(19, 0.0);
        }

        if ((((!s.b[49]) && s.b[54]) && (!s.b[55])) && (!s.b[56])) {
            s.store_scale(17, 15, p.p0);
            s.store_offset(4, 17, p.p21);
        }

        s.b[58] = (s.v[4] > 0.0);
        s.v[58] = if s.b[58] { 1.0 } else { 0.0 };

        if (((((!s.b[49]) && s.b[54]) && (!s.b[55])) && (!s.b[56])) && s.b[58]) {
            s.store_scale(3, 4, (p.p2 / p.p16));
            s.store_sub(16, 3, 14);
            s.store_scalar(5, p.p2);
            s.store_div_from_scalar(19, 1.0, 5);
        }

        if (((((!s.b[49]) && s.b[54]) && (!s.b[55])) && (!s.b[56])) && (!s.b[58])) {
            s.store_scale(16, 15, p.p1);
            s.store_add(3, 16, 14);
            s.store_scalar(5, 1e99);
            s.store_scalar(19, 0.0);
        }

        s.b[60] = (p.p0 == 0.0);
        s.v[60] = if s.b[60] { 1.0 } else { 0.0 };

        if (((!s.b[49]) && (!s.b[54])) && s.b[60]) {
            s.store_scalar(17, 0.0);
            s.store_scalar(4, 0.0);
            s.store_scale(16, 15, p.p1);
            s.store_add(3, 16, 14);
            s.store_scalar(5, 1e99);
            s.store_scalar(19, 0.0);
        }

        s.b[61] = (p.p1 == 0.0);
        s.v[61] = if s.b[61] { 1.0 } else { 0.0 };

        if ((((!s.b[49]) && (!s.b[54])) && (!s.b[60])) && s.b[61]) {
            s.store_scalar(16, 0.0);
            s.store_scalar(3, 0.0);
            s.store_scale(17, 15, p.p0);
            s.store_offset(4, 17, p.p21);
            s.store_scalar(5, 0.0);
            s.store_scalar(19, 1e99);
        }

        if ((((!s.b[49]) && (!s.b[54])) && (!s.b[60])) && (!s.b[61])) {
            s.store_scale(17, 15, p.p0);
            s.store_offset(4, 17, p.p21);
            s.store_scale(16, 15, p.p1);
            s.store_add(3, 16, 14);
        }

        s.b[63] = (s.v[4] > 0.0);
        s.v[63] = if s.b[63] { 1.0 } else { 0.0 };

        s.b[65] = (s.v[3] > 0.0);
        s.v[65] = if s.b[65] { 1.0 } else { 0.0 };

        if ((((((!s.b[49]) && (!s.b[54])) && (!s.b[60])) && (!s.b[61])) && s.b[63]) && s.b[65]) {
            s.store_scaled_div(5, 3, 4, p.p16);
            s.store_div_from_scalar(19, 1.0, 5);
        }

        if ((((((!s.b[49]) && (!s.b[54])) && (!s.b[60])) && (!s.b[61])) && s.b[63]) && (!s.b[65])) {
            s.store_scalar(5, 0.0);
            s.store_scalar(19, 1e99);
        }

        if (((((!s.b[49]) && (!s.b[54])) && (!s.b[60])) && (!s.b[61])) && (!s.b[63])) {
            s.store_scalar(5, 1e99);
            s.store_scalar(19, 0.0);
        }

        if (p.p24 != 0.0) {
            s.store_offset(18, 3, p.p23);
        }

        if (p.p24 == 0.0) {
            s.store_offset(18, 16, p.p23);
        }

        s.v[34] = p.p36;

        s.v[35] = p.p37;

        s.b[71] = (s.v[3] > 0.0);
        s.v[71] = if s.b[71] { 1.0 } else { 0.0 };

        s.b[72] = ((p.p3 != 0.0) && (p.p4 != 0.0));
        s.v[72] = if s.b[72] { 1.0 } else { 0.0 };

        if (s.b[71] && s.b[72]) {
            s.store_offset_div_from_scalar_ad(34, p.p38, s.ad_value(3), s.v[34]);
            s.store_offset_div_from_scalar_ad(35, p.p39, s.ad_value(3), s.v[35]);
        }

        s.b[73] = ((p.p3 != 0.0) || (p.p4 != 0.0));
        s.v[73] = if s.b[73] { 1.0 } else { 0.0 };

        if ((s.b[71] && (!s.b[72])) && s.b[73]) {
            s.store_add_ad_rhs(34, 34, A::div_from_scalar((0.5 * p.p38), s.ad_value(3)));
            s.store_add_ad_rhs(35, 35, A::div_from_scalar((0.5 * p.p39), s.ad_value(3)));
        }

        s.b[74] = (s.v[4] > 0.0);
        s.v[74] = if s.b[74] { 1.0 } else { 0.0 };

        if s.b[74] {
            s.store_add_ad_rhs(34, 34, A::div_from_scalar(p.p40, s.ad_value(4)));
            s.store_add_ad_rhs(35, 35, A::div_from_scalar(p.p41, s.ad_value(4)));
        }

        s.store_offset_mul_ad(13, s.ad_value(12), A::add_scaled_product(s.ad_value(34), 1.0, s.ad_value(12), s.ad_value(35), 1.0), 1.0);

        s.b[76] = (s.v[13] < (0.01 + 0.1));
        s.v[76] = if s.b[76] { 1.0 } else { 0.0 };

        if s.b[76] {
            s.store_offset_scaled_ad(13, A::exp(A::scale_offset(s.ad_value(13), 10.0, (((((-0.01)) * (10.0))) + ((-1.0))))), 0.1, 0.01);
        }

        if (!s.b[76]) {
        }

        s.store_mul(20, 5, 13);

        s.store_div(21, 19, 13);

        s.store_voltage(30, ctx, nodes, Some(0), Some(1));

        s.b[77] = ((s.v[5] > 0.0) && ((p.p28 > 0.0) || (p.p26 > 0.0)));
        s.v[77] = if s.b[77] { 1.0 } else { 0.0 };

        if s.b[77] {
            s.store_div(31, 30, 18);
            s.store_scale(32, 31, p.p27);
            s.store_sqrt_square_offset(23, 32, 1.0);
            s.store_scaled_abs(33, 31, p.p25);
            s.store_powf_ad(24, A::offset(A::mul(A::square(s.ad_value(33)), s.ad_value(33)), 1.0), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(29, A::scale_offset(s.ad_value(23), p.p28, ((1.0 - p.p28) - p.p26)), 24, p.p26);
        }

        if (!s.b[77]) {
            s.store_scalar(29, 1.0);
        }

        s.store_mul(6, 20, 29);

        s.copy_ad(0, 30);

        s.store_div(1, 0, 6);

        s.b[80] = (((p.p6 != 0.0) && (s.v[5] > 0.0)) && (s.v[19] > 0.0));
        s.v[80] = if s.b[80] { 1.0 } else { 0.0 };

        if s.b[80] {
            s.store_div_scaled_product_indices(26, 9, 21, (4.0 * 1.3806505e-23), 29, 1.0);
        }

        s.b[81] = (((p.p32 != 0.0) && (s.v[3] > 0.0)) && (s.v[4] > 0.0));
        s.v[81] = if s.b[81] { 1.0 } else { 0.0 };

        if (s.b[80] && s.b[81]) {
            s.store_div_scaled_product3_mixed_iaii(27, 22, A::powf(A::abs(A::div(s.ad_value(1), s.ad_value(4))), p.p30), 4, 1.0, 3, 1.0);
        }

        s.b[82] = ((s.v[16] > 0.0) && (s.v[17] > 0.0));
        s.v[82] = if s.b[82] { 1.0 } else { 0.0 };

        if ((s.b[80] && (!s.b[81])) && s.b[82]) {
            s.store_div_scaled_product3_mixed_iaii(27, 22, A::powf(A::abs(A::div(s.ad_value(1), s.ad_value(17))), p.p30), 17, 1.0, 16, 1.0);
        }

        if ((s.b[80] && (!s.b[81])) && (!s.b[82])) {
            s.store_scalar(27, 0.0);
        }

        s.b[83] = (s.v[1] < 0.0);
        s.v[83] = if s.b[83] { 1.0 } else { 0.0 };

        if (s.b[80] && s.b[83]) {
            s.store_neg(27, 27);
        }

        if (!s.b[80]) {
            s.store_scalar(26, 0.0);
            s.store_scalar(27, 0.0);
        }

        s.b[84] = ((s.v[5] > 0.0) && (s.v[19] > 0.0));
        s.v[84] = if s.b[84] { 1.0 } else { 0.0 };

        if s.b[84] {
            s.store_mul(6, 20, 29);
        }

        if (!s.b[84]) {
            s.copy_ad(6, 5);
        }

        let eq1_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(0),
            Some(1),
            multiplicity * (eq1_value),
        );
        let eq2_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(0),
            Some(1),
            multiplicity * (eq2_value),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
    }
}
