#![allow(dead_code, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

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

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let multiplicity = self.multiplicity;
        let timestep = self.timestep;
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
        let v199: f64 = 2.0;
        let v222: f64 = ctx.node_voltage(nodes[2]);
        let v224: f64 = (v222 * self.scalar_v223);
        let v225: f64 = (self.scalar_v24 + v224);
        let v228: bool = (v225 < self.scalar_v227);
        let v229: f64 = (v225 - self.scalar_v226);
        let v230: f64 = (v229 - v5);
        let v231: f64 = ((v230) as f64).exp();
        let v232: f64 = (self.scalar_v226 + v231);
        let v233: f64 = (if v228 { v232 } else { v225 });
        let v236: bool = (v233 > self.scalar_v235);
        let v237: bool = (!v228);
        let v238: bool = (v236 && v237);
        let v239: f64 = (self.scalar_v234 - v233);
        let v240: f64 = (v239 - v5);
        let v241: f64 = ((v240) as f64).exp();
        let v242: f64 = (self.scalar_v234 - v241);
        let v243: f64 = (if v238 { v242 } else { v233 });
        let v244: f64 = (v18 + v243);
        let v245: f64 = (v244 - self.scalar_v20);
        let v246: f64 = (self.scalar_v198 * v245);
        let v247: f64 = (self.scalar_v194 + v246);
        let v248: f64 = (v245 * v247);
        let v249: f64 = (v5 + v248);
        let v250: f64 = 0.1;
        let v251: f64 = 0.11;
        let v252: bool = (v249 < v251);
        let v253: f64 = 10.0;
        let v254: f64 = (v249 - v8);
        let v255: f64 = (v253 * v254);
        let v256: f64 = (v255 - v5);
        let v257: f64 = ((v256) as f64).exp();
        let v258: f64 = (v250 * v257);
        let v259: f64 = (v8 + v258);
        let v260: f64 = (if v252 { v259 } else { v249 });
        let v261: f64 = (self.scalar_v156 * v260);
        let v262: f64 = ctx.node_voltage(nodes[0]);
        let v263: f64 = ctx.node_voltage(nodes[1]);
        let v264: f64 = (v262 - v263);
        let v266: f64 = (v264 / self.scalar_v163);
        let v267: f64 = (if self.scalar_v265 { v266 } else { v2 });
        let v269: f64 = (v267 * self.scalar_v268);
        let v270: f64 = (if self.scalar_v265 { v269 } else { v2 });
        let v271: f64 = (v270 * v270);
        let v272: f64 = (v5 + v271);
        let v273: f64 = ((v272) as f64).sqrt();
        let v274: f64 = (if self.scalar_v265 { v273 } else { v2 });
        let v276: f64 = ((v267) as f64).abs();
        let v277: f64 = (self.scalar_v275 * v276);
        let v278: f64 = (if self.scalar_v265 { v277 } else { v2 });
        let v279: f64 = (v278 * v278);
        let v280: f64 = (v278 * v279);
        let v281: f64 = (v5 + v280);
        let v282: f64 = 0.3333333333333333;
        let v283: f64 = f64::powf(v281, v282);
        let v284: f64 = (if self.scalar_v265 { v283 } else { v2 });
        let v287: f64 = (self.scalar_v165 * v274);
        let v288: f64 = (self.scalar_v286 + v287);
        let v289: f64 = (self.scalar_v167 * v284);
        let v290: f64 = (v288 + v289);
        let v291: f64 = (if self.scalar_v265 { v290 } else { v2 });
        let v293: f64 = (if self.scalar_v292 { v5 } else { v291 });
        let v294: f64 = (v261 * v293);
        let v295: f64 = (v264 / v294);
        let v296: f64 = (-v264);
        let v297: f64 = (v295 * v296);
        let v298: f64 = (self.scalar_v214 * v222);
        let v299: f64 = (self.scalar_v221 * v222);
        let v300: f64 = (if (self.scalar_v223 != 0.0) { v298 } else { v2 });
        let v301: f64 = (if (self.scalar_v223 != 0.0) { v297 } else { v2 });
        let v303: f64 = (v16 * v222);
        let v304: f64 = (if self.scalar_v302 { v303 } else { v2 });
        let v305: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v299);
        let v306: f64 = (if (self.scalar_v223 != 0.0) { v305 } else { v2 });
        let v307: f64 = (self.scalar_v223 * v231);
        let v308: f64 = (if v228 { v307 } else { self.scalar_v223 });
        let v309: f64 = (-v308);
        let v310: f64 = (v241 * v309);
        let v311: f64 = (-v310);
        let v312: f64 = (if v238 { v311 } else { v308 });
        let v313: f64 = (self.scalar_v198 * v312);
        let v314: f64 = (v247 * v312);
        let v315: f64 = (v245 * v313);
        let v316: f64 = (v314 + v315);
        let v317: f64 = (v253 * v316);
        let v318: f64 = (v257 * v317);
        let v319: f64 = (v250 * v318);
        let v320: f64 = (if v252 { v319 } else { v316 });
        let v321: f64 = (self.scalar_v156 * v320);
        let v331: f64 = (v270 * self.scalar_v329);
        let v332: f64 = (v331 + v331);
        let v333: f64 = (v270 * self.scalar_v330);
        let v334: f64 = (v333 + v333);
        let v335: f64 = (v199 * v273);
        let v336: f64 = (v332 / v335);
        let v337: f64 = (v334 / v335);
        let v338: f64 = (if self.scalar_v265 { v336 } else { v2 });
        let v339: f64 = (if self.scalar_v265 { v337 } else { v2 });
        let v340: f64 = (self.scalar_v165 * v338);
        let v341: f64 = (self.scalar_v165 * v339);
        let v342: f64 = (if self.scalar_v265 { v340 } else { v2 });
        let v343: f64 = (if self.scalar_v265 { v341 } else { v2 });
        let v344: f64 = (if self.scalar_v292 { v2 } else { v342 });
        let v345: f64 = (if self.scalar_v292 { v2 } else { v343 });
        let v346: f64 = (v261 * v344);
        let v347: f64 = (v261 * v345);
        let v348: f64 = (v293 * v321);
        let v349: f64 = (v264 * v346);
        let v350: f64 = (v294 - v349);
        let v351: f64 = (v294 * v294);
        let v352: f64 = (v350 / v351);
        let v353: f64 = (-v294);
        let v354: f64 = (v264 * v347);
        let v355: f64 = (v353 - v354);
        let v356: f64 = (v355 / v351);
        let v357: f64 = (v264 * v348);
        let v358: f64 = (-v357);
        let v359: f64 = (v358 / v351);
        let v360: f64 = (v296 * v352);
        let v361: f64 = (-v295);
        let v362: f64 = (v360 + v361);
        let v363: f64 = (v296 * v356);
        let v364: f64 = (v295 + v363);
        let v365: f64 = (v296 * v359);
        let v367: f64 = (if (self.scalar_v223 != 0.0) { v362 } else { v2 });
        let v368: f64 = (if (self.scalar_v223 != 0.0) { v364 } else { v2 });
        let v369: f64 = (if (self.scalar_v223 != 0.0) { v365 } else { v2 });
        let v371: f64 = ddt_scale;
        let v372: f64 = (self.scalar_v221 * v371);
        let v373: f64 = (if (self.scalar_v223 != 0.0) { v372 } else { v2 });

        let d295_dn0: f64 = v352;
        let d295_dn1: f64 = v356;
        let d295_dn2: f64 = v359;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(1),
            multiplicity * (v295),
            0,
            multiplicity * (d295_dn0),
            1,
            multiplicity * (d295_dn1),
            2,
            multiplicity * (d295_dn2),
        );
        let d300_dn2: f64 = self.scalar_v366;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v300),
            2,
            multiplicity * (d300_dn2),
        );
        let d301_dn0: f64 = v367;
        let d301_dn1: f64 = v368;
        let d301_dn2: f64 = v369;
        stamper.stamp_current_node3_local(
            Some(2),
            None,
            multiplicity * (v301),
            0,
            multiplicity * (d301_dn0),
            1,
            multiplicity * (d301_dn1),
            2,
            multiplicity * (d301_dn2),
        );
        let d304_dn2: f64 = self.scalar_v370;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v304),
            2,
            multiplicity * (d304_dn2),
        );
        let d306_dn2: f64 = v373;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v306),
            2,
            multiplicity * (d306_dn2),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(1),
            multiplicity * (v2),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(1),
            multiplicity * (v2),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let v2: f64 = 0.0;
        let v222: f64 = ctx.node_voltage(nodes[2]);
        let v299: f64 = (self.scalar_v221 * v222);
        let v305: f64 = 0.0;
        let v306: f64 = (if (self.scalar_v223 != 0.0) { v305 } else { v2 });
        let v371: f64 = 1.0;
        let v372: f64 = (self.scalar_v221 * v371);
        let v373: f64 = (if (self.scalar_v223 != 0.0) { v372 } else { v2 });

        let d306_dn2: f64 = v373;
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (d306_dn2),
        );
    }
}
