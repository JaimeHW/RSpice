#![allow(dead_code, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let multiplicity = self.multiplicity;
        let v0=1.0;
        let v1=0.0;
        let v252=(ctx.node_voltage(nodes[0])-ctx.node_voltage(nodes[1]));
        let v256=(if (self.scalar_static_f64[146]!=0.0){(v252/self.scalar_static_f64[117])}else{v1});
        let v259=(if (self.scalar_static_f64[146]!=0.0){(v256*self.scalar_static_f64[147])}else{v1});
        let v262=((v0+(v259*v259))).sqrt();
        let v267=(if (self.scalar_static_f64[146]!=0.0){(self.scalar_static_f64[148]*(v256).abs())}else{v1});
        let v283=(self.scalar_static_f64[187]*(if self.scalar_static_bool[60]{v0}else{(if (self.scalar_static_f64[146]!=0.0){((self.scalar_static_f64[150]+(self.scalar_static_f64[118]*(if (self.scalar_static_f64[146]!=0.0){v262}else{v1})))+(self.scalar_static_f64[119]*(if (self.scalar_static_f64[146]!=0.0){f64::powf((v0+(v267*(v267*v267))),0.3333333333333333)}else{v1})))}else{v1})}));
        let v294=(v259*self.scalar_static_f64[157]);
        let v296=(v259*self.scalar_static_f64[158]);
        let v299=(v262*2.0);
        let v314=(v283*v283);

        stamper.stamp_current_node2_local(
            Some(0),
            Some(1),
            multiplicity * ((v252/v283)),
            0,
            multiplicity * (((v283-(v252*(self.scalar_static_f64[187]*(if self.scalar_static_bool[60]{v1}else{(if (self.scalar_static_f64[146]!=0.0){(self.scalar_static_f64[118]*(if (self.scalar_static_f64[146]!=0.0){((v294+v294)/v299)}else{v1}))}else{v1})}))))/v314)),
            1,
            multiplicity * ((((-v283)-(v252*(self.scalar_static_f64[187]*(if self.scalar_static_bool[60]{v1}else{(if (self.scalar_static_f64[146]!=0.0){(self.scalar_static_f64[118]*(if (self.scalar_static_f64[146]!=0.0){((v296+v296)/v299)}else{v1}))}else{v1})}))))/v314)),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(1),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(1),
            multiplicity * (v1),
        );
    }

    pub fn stamp_reactive(&mut self, _ctx: &GeneratedEvalContext<'_>, _stamper: &mut GeneratedReactiveStamper<'_>) {
    }
}
