#![allow(dead_code, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let multiplicity = self.multiplicity;
        let v2: f64 = 0.0;
        let v5: f64 = 1.0;
        let v235: f64 = ctx.node_voltage(nodes[0]);
        let v236: f64 = ctx.node_voltage(nodes[1]);
        let v237: f64 = (v235 - v236);
        let v239: f64 = (v237 / self.scalar_v183);
        let v240: f64 = (if self.scalar_v238 { v239 } else { v2 });
        let v242: f64 = (v240 * self.scalar_v241);
        let v243: f64 = (if self.scalar_v238 { v242 } else { v2 });
        let v244: f64 = (v243 * v243);
        let v245: f64 = (v5 + v244);
        let v246: f64 = ((v245) as f64).sqrt();
        let v247: f64 = (if self.scalar_v238 { v246 } else { v2 });
        let v249: f64 = ((v240) as f64).abs();
        let v250: f64 = (self.scalar_v248 * v249);
        let v251: f64 = (if self.scalar_v238 { v250 } else { v2 });
        let v252: f64 = (v251 * v251);
        let v253: f64 = (v251 * v252);
        let v254: f64 = (v5 + v253);
        let v255: f64 = 0.3333333333333333;
        let v256: f64 = f64::powf(v254, v255);
        let v257: f64 = (if self.scalar_v238 { v256 } else { v2 });
        let v260: f64 = (self.scalar_v185 * v247);
        let v261: f64 = (self.scalar_v259 + v260);
        let v262: f64 = (self.scalar_v187 * v257);
        let v263: f64 = (v261 + v262);
        let v264: f64 = (if self.scalar_v238 { v263 } else { v2 });
        let v266: f64 = (if self.scalar_v265 { v5 } else { v264 });
        let v267: f64 = (self.scalar_v234 * v266);
        let v268: f64 = (v237 / v267);
        let v278: f64 = (v243 * self.scalar_v276);
        let v279: f64 = (v278 + v278);
        let v280: f64 = (v243 * self.scalar_v277);
        let v281: f64 = (v280 + v280);
        let v282: f64 = 2.0;
        let v283: f64 = (v246 * v282);
        let v284: f64 = (v279 / v283);
        let v285: f64 = (v281 / v283);
        let v286: f64 = (if self.scalar_v238 { v284 } else { v2 });
        let v287: f64 = (if self.scalar_v238 { v285 } else { v2 });
        let v288: f64 = (self.scalar_v185 * v286);
        let v289: f64 = (self.scalar_v185 * v287);
        let v290: f64 = (if self.scalar_v238 { v288 } else { v2 });
        let v291: f64 = (if self.scalar_v238 { v289 } else { v2 });
        let v292: f64 = (if self.scalar_v265 { v2 } else { v290 });
        let v293: f64 = (if self.scalar_v265 { v2 } else { v291 });
        let v294: f64 = (self.scalar_v234 * v292);
        let v295: f64 = (self.scalar_v234 * v293);
        let v296: f64 = (v237 * v294);
        let v297: f64 = (v267 - v296);
        let v298: f64 = (v267 * v267);
        let v299: f64 = (v297 / v298);
        let v300: f64 = (-v267);
        let v301: f64 = (v237 * v295);
        let v302: f64 = (v300 - v301);
        let v303: f64 = (v302 / v298);

        let d268_dn0: f64 = v299;
        let d268_dn1: f64 = v303;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(1),
            multiplicity * (v268),
            0,
            multiplicity * (d268_dn0),
            1,
            multiplicity * (d268_dn1),
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

    pub fn stamp_reactive(&mut self, _ctx: &GeneratedEvalContext<'_>, _stamper: &mut GeneratedReactiveStamper<'_>) {
    }
}
