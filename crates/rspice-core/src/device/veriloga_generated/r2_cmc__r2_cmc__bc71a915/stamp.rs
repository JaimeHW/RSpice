#![allow(dead_code, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let multiplicity = self.multiplicity;
        let v2=0.0;
        let v5=1.0;
        let v237=(ctx.node_voltage(nodes[0])-ctx.node_voltage(nodes[1]));
        let v240=(if self.scalar_static_bool[59]{(v237/self.scalar_static_f64[105])}else{v2});
        let v243=(if self.scalar_static_bool[59]{(v240*self.scalar_static_f64[134])}else{v2});
        let v246=((v5+(v243*v243))).sqrt();
        let v251=(if self.scalar_static_bool[59]{(self.scalar_static_f64[135]*(v240).abs())}else{v2});
        let v267=(self.scalar_static_f64[171]*(if self.scalar_static_bool[60]{v5}else{(if self.scalar_static_bool[59]{((self.scalar_static_f64[137]+(self.scalar_static_f64[106]*(if self.scalar_static_bool[59]{v246}else{v2})))+(self.scalar_static_f64[107]*(if self.scalar_static_bool[59]{f64::powf((v5+(v251*(v251*v251))),0.3333333333333333)}else{v2})))}else{v2})}));
        let v278=(v243*self.scalar_static_f64[144]);
        let v280=(v243*self.scalar_static_f64[145]);
        let v283=(v246*2.0);
        let v298=(v267*v267);

        stamper.stamp_current_node2_local(
            Some(0),
            Some(1),
            multiplicity * ((v237/v267)),
            0,
            multiplicity * (((v267-(v237*(self.scalar_static_f64[171]*(if self.scalar_static_bool[60]{v2}else{(if self.scalar_static_bool[59]{(self.scalar_static_f64[106]*(if self.scalar_static_bool[59]{((v278+v278)/v283)}else{v2}))}else{v2})}))))/v298)),
            1,
            multiplicity * ((((-v267)-(v237*(self.scalar_static_f64[171]*(if self.scalar_static_bool[60]{v2}else{(if self.scalar_static_bool[59]{(self.scalar_static_f64[106]*(if self.scalar_static_bool[59]{((v280+v280)/v283)}else{v2}))}else{v2})}))))/v298)),
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
