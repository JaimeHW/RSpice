#![allow(dead_code, non_snake_case, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let m=self.multiplicity;
        let multiplicity=m;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let a=1.0;let b=0.0;let de=(ctx.node_voltage(n[0])-ctx.node_voltage(n[1]));let di=(if ((sf[146])!=0.0){(de/sf[117])}else{b});let dl=(if ((sf[146])!=0.0){(di*sf[147])}else{b});let do_=((a+(dl*dl))).sqrt();let dt=(if ((sf[146])!=0.0){(sf[148]*(di).abs())}else{b});let dJ=(sf[187]*(if sb[60]{a}else{(if ((sf[146])!=0.0){((sf[150]+(sf[118]*(if ((sf[146])!=0.0){do_}else{b})))+(sf[119]*(if ((sf[146])!=0.0){f64::powf((a+(dt*(dt*dt))),0.3333333333333333)}else{b})))}else{b})}));let dU=(dl*sf[157]);let dW=(dl*sf[158]);let dZ=(do_*2.0);let ee=(dJ*dJ);

        stamper.stamp_current_node2_local(
            Some(0),
            Some(1),
            multiplicity * ((de/dJ)),
            0,
            multiplicity * (((dJ-(de*(sf[187]*(if sb[60]{b}else{(if ((sf[146])!=0.0){(sf[118]*(if ((sf[146])!=0.0){((dU+dU)/dZ)}else{b}))}else{b})}))))/ee)),
            1,
            multiplicity * ((((-dJ)-(de*(sf[187]*(if sb[60]{b}else{(if ((sf[146])!=0.0){(sf[118]*(if ((sf[146])!=0.0){((dW+dW)/dZ)}else{b}))}else{b})}))))/ee)),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(1),
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(1),
            multiplicity * (b),
        );
    }

    pub fn stamp_reactive(&mut self, _ctx: &GeneratedEvalContext<'_>, _stamper: &mut GeneratedReactiveStamper<'_>) {
    }
}
