#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
#[path = "stamp_blocks_0.rs"]
mod stamp_blocks_0;

const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

#[inline]
fn eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    older: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    derivative_current: &mut [f64; STATE_COUNT],
    derivative_previous: &mut [f64; STATE_COUNT],
    ddt_active: bool,
    ddt_scale: f64,
    ddt_previous_value_scale: f64,
    ddt_older_value_scale: f64,
    ddt_previous_derivative_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    let older_value = if initialized[slot] { older[slot] } else { value };
    current[slot] = value;
    if ddt_active {
        let result = value * ddt_scale
            - previous_value * ddt_previous_value_scale
            - older_value * ddt_older_value_scale
            - derivative_previous[slot] * ddt_previous_derivative_scale;
        derivative_current[slot] = result;
        result
    } else {
        current[slot] = value;
        previous[slot] = value;
        older[slot] = value;
        derivative_current[slot] = 0.0;
        derivative_previous[slot] = 0.0;
        initialized[slot] = true;
        0.0
    }
}

#[inline]
fn ddt_jacobian(ddt_active: bool, ddt_scale: f64, derivative: f64) -> f64 {
    if ddt_active {
        derivative * ddt_scale
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

#[derive(Default)]
pub(crate) struct StampLocals {
    pub(crate) var_a_um2: f64,
    pub(crate) var_a_um2_rv: f64,
    pub(crate) var_cth: f64,
    pub(crate) var_cth_rv: f64,
    pub(crate) var_guard41: f64,
    pub(crate) var_guard41_rv: f64,
    pub(crate) var_guard42: f64,
    pub(crate) var_guard42_rv: f64,
    pub(crate) var_guard46: f64,
    pub(crate) var_guard46_rv: f64,
    pub(crate) var_guard47: f64,
    pub(crate) var_guard47_rv: f64,
    pub(crate) var_guard48: f64,
    pub(crate) var_guard48_rv: f64,
    pub(crate) var_guard49: f64,
    pub(crate) var_guard49_rv: f64,
    pub(crate) var_guard51: f64,
    pub(crate) var_guard51_rv: f64,
    pub(crate) var_guard53: f64,
    pub(crate) var_guard53_rv: f64,
    pub(crate) var_guard54: f64,
    pub(crate) var_guard54_rv: f64,
    pub(crate) var_guard55: f64,
    pub(crate) var_guard55_rv: f64,
    pub(crate) var_guard57: f64,
    pub(crate) var_guard57_rv: f64,
    pub(crate) var_guard59: f64,
    pub(crate) var_guard59_rv: f64,
    pub(crate) var_guard60: f64,
    pub(crate) var_guard60_rv: f64,
    pub(crate) var_guard75: f64,
    pub(crate) var_guard75_rv: f64,
    pub(crate) var_guard76: f64,
    pub(crate) var_guard76_rv: f64,
    pub(crate) var_l_um: f64,
    pub(crate) var_l_um_rv: f64,
    pub(crate) var_leff_um: f64,
    pub(crate) var_leff_um_rv: f64,
    pub(crate) var_lfactor: f64,
    pub(crate) var_lfactor_rv: f64,
    pub(crate) var_p_um: f64,
    pub(crate) var_p_um_rv: f64,
    pub(crate) var_qcth: f64,
    pub(crate) var_qcth_dn2: f64,
    pub(crate) var_qcth_rv: f64,
    pub(crate) var_scalefac: f64,
    pub(crate) var_scalefac_rv: f64,
    pub(crate) var_shrinkl: f64,
    pub(crate) var_shrinkl_rv: f64,
    pub(crate) var_vrth: f64,
    pub(crate) var_vrth_dn2: f64,
    pub(crate) var_vrth_rv: f64,
    pub(crate) var_w_um: f64,
    pub(crate) var_w_um_rv: f64,
    pub(crate) var_weff_um: f64,
    pub(crate) var_weff_um_rv: f64,
    pub(crate) var_xleff: f64,
    pub(crate) var_xleff_rv: f64,
}

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let scalar_temperature_static_temperature = (ctx).temperature();
        let scalar_temperature_static_thermal_voltage = (ctx).thermal_voltage();
        self.ensure_temperature_static(scalar_temperature_static_temperature, scalar_temperature_static_thermal_voltage);
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_older = self.ddt_state_older.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_derivative_current = self.ddt_derivative_current.as_mut();
        let ddt_derivative_previous = self.ddt_derivative_previous.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_scale = self.ddt_coefficients.derivative_scale;
        let ddt_previous_value_scale = self.ddt_coefficients.previous_value_scale;
        let ddt_older_value_scale = self.ddt_coefficients.older_value_scale;
        let ddt_previous_derivative_scale = self.ddt_coefficients.previous_derivative_scale;
        let v2: f64 = 0.0;
        let v5: f64 = 1.0;
        let v8: f64 = 0.01;
        let v16: f64 = 1000000.0;
        let v18: f64 = 273.15;
        let v200: f64 = 2.0;
        let v216: f64 = nv2;
        let v218: f64 = (v216 * self.scalar_v217);
        let v219: f64 = (self.scalar_v24 + v218);
        let v222: bool = (v219 < self.scalar_v221);
        let v223: f64 = (v219 - self.scalar_v220);
        let v224: f64 = (v223 - v5);
        let v225: f64 = ((v224) as f64).exp();
        let v226: f64 = (self.scalar_v220 + v225);
        let v227: f64 = (if v222 { v226 } else { v219 });
        let v230: bool = (v227 > self.scalar_v229);
        let v231: bool = (!v222);
        let v232: bool = (v230 && v231);
        let v233: f64 = (self.scalar_v228 - v227);
        let v234: f64 = (v233 - v5);
        let v235: f64 = ((v234) as f64).exp();
        let v236: f64 = (self.scalar_v228 - v235);
        let v237: f64 = (if v232 { v236 } else { v227 });
        let v238: bool = (!v230);
        let v239: bool = (v231 && v238);
        let v240: f64 = (if v239 { v237 } else { v237 });
        let v241: f64 = (v18 + v240);
        let v242: f64 = (v241 - self.scalar_v20);
        let v243: f64 = (self.scalar_v199 * v242);
        let v244: f64 = (self.scalar_v195 + v243);
        let v245: f64 = (v242 * v244);
        let v246: f64 = (v5 + v245);
        let v247: f64 = 0.1;
        let v248: f64 = 0.11;
        let v249: bool = (v246 < v248);
        let v250: f64 = 10.0;
        let v251: f64 = (v246 - v8);
        let v252: f64 = (v250 * v251);
        let v253: f64 = (v252 - v5);
        let v254: f64 = ((v253) as f64).exp();
        let v255: f64 = (v247 * v254);
        let v256: f64 = (v8 + v255);
        let v257: f64 = (if v249 { v256 } else { v246 });
        let v258: bool = (!v249);
        let v259: f64 = (if v258 { v257 } else { v257 });
        let v260: f64 = (self.scalar_v157 * v259);
        let v261: f64 = nv0;
        let v262: f64 = nv1;
        let v263: f64 = (v261 - v262);
        let v265: f64 = (v263 / self.scalar_v164);
        let v266: f64 = (if self.scalar_v264 { v265 } else { v2 });
        let v268: f64 = (v266 * self.scalar_v267);
        let v269: f64 = (if self.scalar_v264 { v268 } else { v2 });
        let v270: f64 = (v269 * v269);
        let v271: f64 = (v5 + v270);
        let v272: f64 = ((v271) as f64).sqrt();
        let v273: f64 = (if self.scalar_v264 { v272 } else { v2 });
        let v275: f64 = ((v266) as f64).abs();
        let v276: f64 = (self.scalar_v274 * v275);
        let v277: f64 = (if self.scalar_v264 { v276 } else { v2 });
        let v278: f64 = (v277 * v277);
        let v279: f64 = (v277 * v278);
        let v280: f64 = (v5 + v279);
        let v281: f64 = 0.3333333333333333;
        let v282: f64 = f64::powf(v280, v281);
        let v283: f64 = (if self.scalar_v264 { v282 } else { v2 });
        let v286: f64 = (self.scalar_v166 * v273);
        let v287: f64 = (self.scalar_v285 + v286);
        let v288: f64 = (self.scalar_v168 * v283);
        let v289: f64 = (v287 + v288);
        let v290: f64 = (if self.scalar_v264 { v289 } else { v2 });
        let v292: f64 = (if self.scalar_v291 { v5 } else { v290 });
        let v293: f64 = (v260 * v292);
        let v294: f64 = (v263 / v293);
        let v295: f64 = (-v263);
        let v296: f64 = (v294 * v295);
        let v297: f64 = (self.scalar_v215 * v216);
        let v298: f64 = (if (self.scalar_v217 != 0.0) { v297 } else { v2 });
        let v299: f64 = (if (self.scalar_v217 != 0.0) { v296 } else { v2 });
        let v301: f64 = (v16 * v216);
        let v302: f64 = (if self.scalar_v300 { v301 } else { v2 });
        let v303: f64 = (self.scalar_v217 * v225);
        let v304: f64 = (if v222 { v303 } else { self.scalar_v217 });
        let v305: f64 = (-v304);
        let v306: f64 = (v235 * v305);
        let v307: f64 = (-v306);
        let v308: f64 = (if v232 { v307 } else { v304 });
        let v309: f64 = (if v239 { v308 } else { v308 });
        let v310: f64 = (self.scalar_v199 * v309);
        let v311: f64 = (v244 * v309);
        let v312: f64 = (v242 * v310);
        let v313: f64 = (v311 + v312);
        let v314: f64 = (v250 * v313);
        let v315: f64 = (v254 * v314);
        let v316: f64 = (v247 * v315);
        let v317: f64 = (if v249 { v316 } else { v313 });
        let v318: f64 = (if v258 { v317 } else { v317 });
        let v319: f64 = (self.scalar_v157 * v318);
        let v329: f64 = (v269 * self.scalar_v327);
        let v330: f64 = (v329 + v329);
        let v331: f64 = (v269 * self.scalar_v328);
        let v332: f64 = (v331 + v331);
        let v333: f64 = (v200 * v272);
        let v334: f64 = (v330 / v333);
        let v335: f64 = (v332 / v333);
        let v336: f64 = (if self.scalar_v264 { v334 } else { v2 });
        let v337: f64 = (if self.scalar_v264 { v335 } else { v2 });
        let v338: f64 = (self.scalar_v166 * v336);
        let v339: f64 = (self.scalar_v166 * v337);
        let v340: f64 = (if self.scalar_v264 { v338 } else { v2 });
        let v341: f64 = (if self.scalar_v264 { v339 } else { v2 });
        let v342: f64 = (if self.scalar_v291 { v2 } else { v340 });
        let v343: f64 = (if self.scalar_v291 { v2 } else { v341 });
        let v344: f64 = (v260 * v342);
        let v345: f64 = (v260 * v343);
        let v346: f64 = (v292 * v319);
        let v347: f64 = (v263 * v344);
        let v348: f64 = (v293 - v347);
        let v349: f64 = (v293 * v293);
        let v350: f64 = (v348 / v349);
        let v351: f64 = (-v293);
        let v352: f64 = (v263 * v345);
        let v353: f64 = (v351 - v352);
        let v354: f64 = (v353 / v349);
        let v355: f64 = (v263 * v346);
        let v356: f64 = (-v355);
        let v357: f64 = (v356 / v349);
        let v358: f64 = (v295 * v350);
        let v359: f64 = (-v294);
        let v360: f64 = (v358 + v359);
        let v361: f64 = (v295 * v354);
        let v362: f64 = (v294 + v361);
        let v363: f64 = (v295 * v357);
        let v365: f64 = (if (self.scalar_v217 != 0.0) { v360 } else { v2 });
        let v366: f64 = (if (self.scalar_v217 != 0.0) { v362 } else { v2 });
        let v367: f64 = (if (self.scalar_v217 != 0.0) { v363 } else { v2 });

        let d294_dn0: f64 = v350;
        let d294_dn1: f64 = v354;
        let d294_dn2: f64 = v357;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(1),
            multiplicity * (v294),
            0,
            multiplicity * (d294_dn0),
            1,
            multiplicity * (d294_dn1),
            2,
            multiplicity * (d294_dn2),
        );
        let d298_dn2: f64 = self.scalar_v364;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v298),
            2,
            multiplicity * (d298_dn2),
        );
        let d299_dn0: f64 = v365;
        let d299_dn1: f64 = v366;
        let d299_dn2: f64 = v367;
        stamper.stamp_current_node3_local(
            Some(2),
            None,
            multiplicity * (v299),
            0,
            multiplicity * (d299_dn0),
            1,
            multiplicity * (d299_dn1),
            2,
            multiplicity * (d299_dn2),
        );
        let d302_dn2: f64 = self.scalar_v368;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v302),
            2,
            multiplicity * (d302_dn2),
        );
        let mut locals = StampLocals::default();

        Self::stamp_transient_block_0(p, param_given, &mut locals);
        Self::stamp_transient_block_1(ctx, nodes, &mut locals);

        let (eq4_e72, eq4_e72_d_n2,) = {
    if (p.p7 != 0.0) {
        let eq4_e70: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qcth);
        (eq4_e70, (locals.var_qcth_dn2 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e72;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (eq4_value),
            2,
            multiplicity * (eq4_e72_d_n2),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let scalar_temperature_static_temperature = (ctx).temperature();
        let scalar_temperature_static_thermal_voltage = (ctx).thermal_voltage();
        self.ensure_temperature_static(scalar_temperature_static_temperature, scalar_temperature_static_thermal_voltage);
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let mut locals = StampLocals::default();

        Self::stamp_reactive_block_0(p, param_given, &mut locals);
        Self::stamp_reactive_block_1(ctx, p, nodes, &mut locals);

        let (eq4_e72, eq4_e72_d_n2, eq4_e72_q,) = {
    if (p.p7 != 0.0) {
        let eq4_e70_q: f64 = locals.var_qcth;
        (locals.var_qcth, locals.var_qcth_dn2, eq4_e70_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (eq4_e72_d_n2),
        );
    }
}
