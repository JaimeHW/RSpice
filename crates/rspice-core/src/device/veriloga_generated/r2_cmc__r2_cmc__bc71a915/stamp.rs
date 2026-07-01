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
        let v241: f64 = ctx.node_voltage(nodes[0]);
        let v242: f64 = ctx.node_voltage(nodes[1]);
        let v243: f64 = (v241 - v242);
        let v245: f64 = (v243 / self.scalar_v187);
        let v246: f64 = (if self.scalar_v244 { v245 } else { v2 });
        let v248: f64 = (v246 * self.scalar_v247);
        let v249: f64 = (if self.scalar_v244 { v248 } else { v2 });
        let v250: f64 = (v249 * v249);
        let v251: f64 = (v5 + v250);
        let v252: f64 = ((v251) as f64).sqrt();
        let v253: f64 = (if self.scalar_v244 { v252 } else { v2 });
        let v255: f64 = ((v246) as f64).abs();
        let v256: f64 = (self.scalar_v254 * v255);
        let v257: f64 = (if self.scalar_v244 { v256 } else { v2 });
        let v258: f64 = (v257 * v257);
        let v259: f64 = (v257 * v258);
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
        let v273: f64 = (self.scalar_v240 * v272);
        let v274: f64 = (v243 / v273);
        let v284: f64 = (v249 * self.scalar_v282);
        let v285: f64 = (v284 + v284);
        let v286: f64 = (v249 * self.scalar_v283);
        let v287: f64 = (v286 + v286);
        let v288: f64 = 2.0;
        let v289: f64 = (v252 * v288);
        let v290: f64 = (v285 / v289);
        let v291: f64 = (v287 / v289);
        let v292: f64 = (if self.scalar_v244 { v290 } else { v2 });
        let v293: f64 = (if self.scalar_v244 { v291 } else { v2 });
        let v294: f64 = (self.scalar_v189 * v292);
        let v295: f64 = (self.scalar_v189 * v293);
        let v296: f64 = (if self.scalar_v244 { v294 } else { v2 });
        let v297: f64 = (if self.scalar_v244 { v295 } else { v2 });
        let v298: f64 = (if self.scalar_v271 { v2 } else { v296 });
        let v299: f64 = (if self.scalar_v271 { v2 } else { v297 });
        let v300: f64 = (self.scalar_v240 * v298);
        let v301: f64 = (self.scalar_v240 * v299);
        let v302: f64 = (v243 * v300);
        let v303: f64 = (v273 - v302);
        let v304: f64 = (v273 * v273);
        let v305: f64 = (v303 / v304);
        let v306: f64 = (-v273);
        let v307: f64 = (v243 * v301);
        let v308: f64 = (v306 - v307);
        let v309: f64 = (v308 / v304);

        let d274_dn0: f64 = v305;
        let d274_dn1: f64 = v309;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(1),
            multiplicity * (v274),
            0,
            multiplicity * (d274_dn0),
            1,
            multiplicity * (d274_dn1),
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
