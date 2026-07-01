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
        let v200: f64 = 2.0;
        let v223: f64 = ctx.node_voltage(nodes[2]);
        let v225: f64 = (v223 * self.scalar_v224);
        let v226: f64 = (self.scalar_v24 + v225);
        let v229: bool = (v226 < self.scalar_v228);
        let v230: f64 = (v226 - self.scalar_v227);
        let v231: f64 = (v230 - v5);
        let v232: f64 = ((v231) as f64).exp();
        let v233: f64 = (self.scalar_v227 + v232);
        let v234: f64 = (if v229 { v233 } else { v226 });
        let v237: bool = (v234 > self.scalar_v236);
        let v238: bool = (!v229);
        let v239: bool = (v237 && v238);
        let v240: f64 = (self.scalar_v235 - v234);
        let v241: f64 = (v240 - v5);
        let v242: f64 = ((v241) as f64).exp();
        let v243: f64 = (self.scalar_v235 - v242);
        let v244: f64 = (if v239 { v243 } else { v234 });
        let v245: bool = (!v237);
        let v246: bool = (v238 && v245);
        let v247: f64 = (if v246 { v244 } else { v244 });
        let v248: f64 = (v18 + v247);
        let v249: f64 = (v248 - self.scalar_v20);
        let v250: f64 = (self.scalar_v199 * v249);
        let v251: f64 = (self.scalar_v195 + v250);
        let v252: f64 = (v249 * v251);
        let v253: f64 = (v5 + v252);
        let v254: f64 = 0.1;
        let v255: f64 = 0.11;
        let v256: bool = (v253 < v255);
        let v257: f64 = 10.0;
        let v258: f64 = (v253 - v8);
        let v259: f64 = (v257 * v258);
        let v260: f64 = (v259 - v5);
        let v261: f64 = ((v260) as f64).exp();
        let v262: f64 = (v254 * v261);
        let v263: f64 = (v8 + v262);
        let v264: f64 = (if v256 { v263 } else { v253 });
        let v265: bool = (!v256);
        let v266: f64 = (if v265 { v264 } else { v264 });
        let v267: f64 = (self.scalar_v157 * v266);
        let v268: f64 = ctx.node_voltage(nodes[0]);
        let v269: f64 = ctx.node_voltage(nodes[1]);
        let v270: f64 = (v268 - v269);
        let v272: f64 = (v270 / self.scalar_v164);
        let v273: f64 = (if self.scalar_v271 { v272 } else { v2 });
        let v275: f64 = (v273 * self.scalar_v274);
        let v276: f64 = (if self.scalar_v271 { v275 } else { v2 });
        let v277: f64 = (v276 * v276);
        let v278: f64 = (v5 + v277);
        let v279: f64 = ((v278) as f64).sqrt();
        let v280: f64 = (if self.scalar_v271 { v279 } else { v2 });
        let v282: f64 = ((v273) as f64).abs();
        let v283: f64 = (self.scalar_v281 * v282);
        let v284: f64 = (if self.scalar_v271 { v283 } else { v2 });
        let v285: f64 = (v284 * v284);
        let v286: f64 = (v284 * v285);
        let v287: f64 = (v5 + v286);
        let v288: f64 = 0.3333333333333333;
        let v289: f64 = f64::powf(v287, v288);
        let v290: f64 = (if self.scalar_v271 { v289 } else { v2 });
        let v293: f64 = (self.scalar_v166 * v280);
        let v294: f64 = (self.scalar_v292 + v293);
        let v295: f64 = (self.scalar_v168 * v290);
        let v296: f64 = (v294 + v295);
        let v297: f64 = (if self.scalar_v271 { v296 } else { v2 });
        let v299: f64 = (if self.scalar_v298 { v5 } else { v297 });
        let v300: f64 = (v267 * v299);
        let v301: f64 = (v270 / v300);
        let v302: f64 = (-v270);
        let v303: f64 = (v301 * v302);
        let v304: f64 = (self.scalar_v215 * v223);
        let v305: f64 = (self.scalar_v222 * v223);
        let v306: f64 = (if (self.scalar_v224 != 0.0) { v304 } else { v2 });
        let v307: f64 = (if (self.scalar_v224 != 0.0) { v303 } else { v2 });
        let v309: f64 = (v16 * v223);
        let v310: f64 = (if self.scalar_v308 { v309 } else { v2 });
        let v311: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v305);
        let v312: f64 = (if (self.scalar_v224 != 0.0) { v311 } else { v2 });
        let v313: f64 = (self.scalar_v224 * v232);
        let v314: f64 = (if v229 { v313 } else { self.scalar_v224 });
        let v315: f64 = (-v314);
        let v316: f64 = (v242 * v315);
        let v317: f64 = (-v316);
        let v318: f64 = (if v239 { v317 } else { v314 });
        let v319: f64 = (if v246 { v318 } else { v318 });
        let v320: f64 = (self.scalar_v199 * v319);
        let v321: f64 = (v251 * v319);
        let v322: f64 = (v249 * v320);
        let v323: f64 = (v321 + v322);
        let v324: f64 = (v257 * v323);
        let v325: f64 = (v261 * v324);
        let v326: f64 = (v254 * v325);
        let v327: f64 = (if v256 { v326 } else { v323 });
        let v328: f64 = (if v265 { v327 } else { v327 });
        let v329: f64 = (self.scalar_v157 * v328);
        let v339: f64 = (v276 * self.scalar_v337);
        let v340: f64 = (v339 + v339);
        let v341: f64 = (v276 * self.scalar_v338);
        let v342: f64 = (v341 + v341);
        let v343: f64 = (v200 * v279);
        let v344: f64 = (v340 / v343);
        let v345: f64 = (v342 / v343);
        let v346: f64 = (if self.scalar_v271 { v344 } else { v2 });
        let v347: f64 = (if self.scalar_v271 { v345 } else { v2 });
        let v348: f64 = (self.scalar_v166 * v346);
        let v349: f64 = (self.scalar_v166 * v347);
        let v350: f64 = (if self.scalar_v271 { v348 } else { v2 });
        let v351: f64 = (if self.scalar_v271 { v349 } else { v2 });
        let v352: f64 = (if self.scalar_v298 { v2 } else { v350 });
        let v353: f64 = (if self.scalar_v298 { v2 } else { v351 });
        let v354: f64 = (v267 * v352);
        let v355: f64 = (v267 * v353);
        let v356: f64 = (v299 * v329);
        let v357: f64 = (v270 * v354);
        let v358: f64 = (v300 - v357);
        let v359: f64 = (v300 * v300);
        let v360: f64 = (v358 / v359);
        let v361: f64 = (-v300);
        let v362: f64 = (v270 * v355);
        let v363: f64 = (v361 - v362);
        let v364: f64 = (v363 / v359);
        let v365: f64 = (v270 * v356);
        let v366: f64 = (-v365);
        let v367: f64 = (v366 / v359);
        let v368: f64 = (v302 * v360);
        let v369: f64 = (-v301);
        let v370: f64 = (v368 + v369);
        let v371: f64 = (v302 * v364);
        let v372: f64 = (v301 + v371);
        let v373: f64 = (v302 * v367);
        let v375: f64 = (if (self.scalar_v224 != 0.0) { v370 } else { v2 });
        let v376: f64 = (if (self.scalar_v224 != 0.0) { v372 } else { v2 });
        let v377: f64 = (if (self.scalar_v224 != 0.0) { v373 } else { v2 });
        let v379: f64 = ddt_scale;
        let v380: f64 = (self.scalar_v222 * v379);
        let v381: f64 = (if (self.scalar_v224 != 0.0) { v380 } else { v2 });

        let d301_dn0: f64 = v360;
        let d301_dn1: f64 = v364;
        let d301_dn2: f64 = v367;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(1),
            multiplicity * (v301),
            0,
            multiplicity * (d301_dn0),
            1,
            multiplicity * (d301_dn1),
            2,
            multiplicity * (d301_dn2),
        );
        let d306_dn2: f64 = self.scalar_v374;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v306),
            2,
            multiplicity * (d306_dn2),
        );
        let d307_dn0: f64 = v375;
        let d307_dn1: f64 = v376;
        let d307_dn2: f64 = v377;
        stamper.stamp_current_node3_local(
            Some(2),
            None,
            multiplicity * (v307),
            0,
            multiplicity * (d307_dn0),
            1,
            multiplicity * (d307_dn1),
            2,
            multiplicity * (d307_dn2),
        );
        let d310_dn2: f64 = self.scalar_v378;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v310),
            2,
            multiplicity * (d310_dn2),
        );
        let d312_dn2: f64 = v381;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v312),
            2,
            multiplicity * (d312_dn2),
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
        let v223: f64 = ctx.node_voltage(nodes[2]);
        let v305: f64 = (self.scalar_v222 * v223);
        let v311: f64 = 0.0;
        let v312: f64 = (if (self.scalar_v224 != 0.0) { v311 } else { v2 });
        let v379: f64 = 1.0;
        let v380: f64 = (self.scalar_v222 * v379);
        let v381: f64 = (if (self.scalar_v224 != 0.0) { v380 } else { v2 });

        let d312_dn2: f64 = v381;
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (d312_dn2),
        );
    }
}
