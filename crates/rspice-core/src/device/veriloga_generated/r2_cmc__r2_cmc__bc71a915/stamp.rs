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
        let v237: f64 = (ctx.node_voltage(nodes[0]) - ctx.node_voltage(nodes[1]));
        let v240: f64 = (if self.scalar_v238 { (v237 / self.scalar_v183) } else { v2 });
        let v243: f64 = (if self.scalar_v238 { (v240 * self.scalar_v241) } else { v2 });
        let v246: f64 = (((v5 + (v243 * v243))) as f64).sqrt();
        let v251: f64 = (if self.scalar_v238 { (self.scalar_v248 * ((v240) as f64).abs()) } else { v2 });
        let v264: f64 = (if self.scalar_v238 { ((self.scalar_v259 + (self.scalar_v185 * (if self.scalar_v238 { v246 } else { v2 }))) + (self.scalar_v187 * (if self.scalar_v238 { f64::powf((v5 + (v251 * (v251 * v251))), 0.3333333333333333) } else { v2 }))) } else { v2 });
        let v267: f64 = (self.scalar_v234 * (if self.scalar_v265 { v5 } else { v264 }));
        let v268: f64 = (v237 / v267);
        let v278: f64 = (v243 * self.scalar_v276);
        let v280: f64 = (v243 * self.scalar_v277);
        let v283: f64 = (v246 * 2.0);
        let v298: f64 = (v267 * v267);
        let v299: f64 = ((v267 - (v237 * (self.scalar_v234 * (if self.scalar_v265 { v2 } else { (if self.scalar_v238 { (self.scalar_v185 * (if self.scalar_v238 { ((v278 + v278) / v283) } else { v2 })) } else { v2 }) })))) / v298);
        let v303: f64 = (((-v267) - (v237 * (self.scalar_v234 * (if self.scalar_v265 { v2 } else { (if self.scalar_v238 { (self.scalar_v185 * (if self.scalar_v238 { ((v280 + v280) / v283) } else { v2 })) } else { v2 }) })))) / v298);

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
