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
        let v222: f64 = ctx.node_voltage(nodes[2]);
        let v225: f64 = (self.scalar_v24 + (v222 * self.scalar_v223));
        let v228: bool = (v225 < self.scalar_v227);
        let v231: f64 = ((((v225 - self.scalar_v226) - v5)) as f64).exp();
        let v233: f64 = (if v228 { (self.scalar_v226 + v231) } else { v225 });
        let v238: bool = ((v233 > self.scalar_v235) && (!v228));
        let v241: f64 = ((((self.scalar_v234 - v233) - v5)) as f64).exp();
        let v245: f64 = ((273.15 + (if v238 { (self.scalar_v234 - v241) } else { v233 })) - self.scalar_v20);
        let v247: f64 = (self.scalar_v194 + (self.scalar_v198 * v245));
        let v249: f64 = (v5 + (v245 * v247));
        let v250: f64 = 0.1;
        let v252: bool = (v249 < 0.11);
        let v253: f64 = 10.0;
        let v257: f64 = ((((v253 * (v249 - v8)) - v5)) as f64).exp();
        let v261: f64 = (self.scalar_v156 * (if v252 { (v8 + (v250 * v257)) } else { v249 }));
        let v264: f64 = (ctx.node_voltage(nodes[0]) - ctx.node_voltage(nodes[1]));
        let v267: f64 = (if self.scalar_v265 { (v264 / self.scalar_v163) } else { v2 });
        let v270: f64 = (if self.scalar_v265 { (v267 * self.scalar_v268) } else { v2 });
        let v273: f64 = (((v5 + (v270 * v270))) as f64).sqrt();
        let v278: f64 = (if self.scalar_v265 { (self.scalar_v275 * ((v267) as f64).abs()) } else { v2 });
        let v291: f64 = (if self.scalar_v265 { ((self.scalar_v286 + (self.scalar_v165 * (if self.scalar_v265 { v273 } else { v2 }))) + (self.scalar_v167 * (if self.scalar_v265 { f64::powf((v5 + (v278 * (v278 * v278))), 0.3333333333333333) } else { v2 }))) } else { v2 });
        let v293: f64 = (if self.scalar_v292 { v5 } else { v291 });
        let v294: f64 = (v261 * v293);
        let v295: f64 = (v264 / v294);
        let v296: f64 = (-v264);
        let v300: f64 = (if (self.scalar_v223 != 0.0) { (self.scalar_v214 * v222) } else { v2 });
        let v301: f64 = (if (self.scalar_v223 != 0.0) { (v295 * v296) } else { v2 });
        let v304: f64 = (if self.scalar_v302 { (1000000.0 * v222) } else { v2 });
        let v305: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (self.scalar_v221 * v222));
        let v306: f64 = (if (self.scalar_v223 != 0.0) { v305 } else { v2 });
        let v308: f64 = (if v228 { (self.scalar_v223 * v231) } else { self.scalar_v223 });
        let v312: f64 = (if v238 { (-(v241 * (-v308))) } else { v308 });
        let v316: f64 = ((v247 * v312) + (v245 * (self.scalar_v198 * v312)));
        let v331: f64 = (v270 * self.scalar_v329);
        let v333: f64 = (v270 * self.scalar_v330);
        let v335: f64 = (2.0 * v273);
        let v351: f64 = (v294 * v294);
        let v352: f64 = ((v294 - (v264 * (v261 * (if self.scalar_v292 { v2 } else { (if self.scalar_v265 { (self.scalar_v165 * (if self.scalar_v265 { ((v331 + v331) / v335) } else { v2 })) } else { v2 }) })))) / v351);
        let v356: f64 = (((-v294) - (v264 * (v261 * (if self.scalar_v292 { v2 } else { (if self.scalar_v265 { (self.scalar_v165 * (if self.scalar_v265 { ((v333 + v333) / v335) } else { v2 })) } else { v2 }) })))) / v351);
        let v359: f64 = ((-(v264 * (v293 * (self.scalar_v156 * (if v252 { (v250 * (v257 * (v253 * v316))) } else { v316 }))))) / v351);
        let v367: f64 = (if (self.scalar_v223 != 0.0) { ((v296 * v352) + (-v295)) } else { v2 });
        let v368: f64 = (if (self.scalar_v223 != 0.0) { (v295 + (v296 * v356)) } else { v2 });
        let v369: f64 = (if (self.scalar_v223 != 0.0) { (v296 * v359) } else { v2 });
        let v373: f64 = (if (self.scalar_v223 != 0.0) { (self.scalar_v221 * ddt_scale) } else { v2 });

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
        let v305: f64 = 0.0;
        let v306: f64 = (if (self.scalar_v223 != 0.0) { v305 } else { v2 });
        let v373: f64 = (if (self.scalar_v223 != 0.0) { (self.scalar_v221 * 1.0) } else { v2 });

        let d306_dn2: f64 = v373;
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (d306_dn2),
        );
    }
}
