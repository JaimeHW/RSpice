#![allow(dead_code, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let multiplicity = self.multiplicity;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let v0=1.0;
        let v1=0.0;
        let v70=(ctx.node_voltage(nodes[0])-ctx.node_voltage(nodes[1]));
        let v74=(if (sf[146]!=0.0){(v70/sf[117])}else{v1});
        let v77=(if (sf[146]!=0.0){(v74*sf[147])}else{v1});
        let v7a=((v0+(v77*v77))).sqrt();
        let v7f=(if (sf[146]!=0.0){(sf[148]*(v74).abs())}else{v1});
        let v7v=(sf[187]*(if sb[60]{v0}else{(if (sf[146]!=0.0){((sf[150]+(sf[118]*(if (sf[146]!=0.0){v7a}else{v1})))+(sf[119]*(if (sf[146]!=0.0){f64::powf((v0+(v7f*(v7f*v7f))),0.3333333333333333)}else{v1})))}else{v1})}));
        let v86=(v77*sf[157]);
        let v88=(v77*sf[158]);
        let v8b=(v7a*2.0);
        let v8q=(v7v*v7v);

        stamper.stamp_current_node2_local(
            Some(0),
            Some(1),
            multiplicity * ((v70/v7v)),
            0,
            multiplicity * (((v7v-(v70*(sf[187]*(if sb[60]{v1}else{(if (sf[146]!=0.0){(sf[118]*(if (sf[146]!=0.0){((v86+v86)/v8b)}else{v1}))}else{v1})}))))/v8q)),
            1,
            multiplicity * ((((-v7v)-(v70*(sf[187]*(if sb[60]{v1}else{(if (sf[146]!=0.0){(sf[118]*(if (sf[146]!=0.0){((v88+v88)/v8b)}else{v1}))}else{v1})}))))/v8q)),
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
