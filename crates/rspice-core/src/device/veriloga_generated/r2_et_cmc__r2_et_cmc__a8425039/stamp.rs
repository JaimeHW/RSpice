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
        let v0=1.0;
        let v1=0.0;
        let v8=0.01;
        let v234=ctx.node_voltage(nodes[2]);
        let v237=(self.scalar_static_f64[185]+(v234*self.scalar_static_f64[164]));
        let v241=(if (v237<self.scalar_static_f64[166]){v0}else{v1});
        let v244=(((v237-self.scalar_static_f64[165])-v0)).exp();
        let v246=(if (v241!=0.0){(self.scalar_static_f64[165]+v244)}else{v237});
        let v252=(((if (v246>self.scalar_static_f64[168]){v0}else{v1})!=0.0)&&(!(v241!=0.0)));
        let v255=(((self.scalar_static_f64[167]-v246)-v0)).exp();
        let v259=((273.15+(if v252{(self.scalar_static_f64[167]-v255)}else{v246}))-self.scalar_static_f64[13]);
        let v261=(self.scalar_static_f64[137]+(self.scalar_static_f64[141]*v259));
        let v263=(v0+(v259*v261));
        let v264=0.1;
        let v267=(if (v263<0.11){v0}else{v1});
        let v268=10.0;
        let v272=(((v268*(v263-v8))-v0)).exp();
        let v276=(self.scalar_static_f64[107]*(if (v267!=0.0){(v8+(v264*v272))}else{v263}));
        let v279=(ctx.node_voltage(nodes[0])-ctx.node_voltage(nodes[1]));
        let v283=(if (self.scalar_static_f64[169]!=0.0){(v279/self.scalar_static_f64[113])}else{v1});
        let v286=(if (self.scalar_static_f64[169]!=0.0){(v283*self.scalar_static_f64[170])}else{v1});
        let v289=((v0+(v286*v286))).sqrt();
        let v294=(if (self.scalar_static_f64[169]!=0.0){(self.scalar_static_f64[171]*(v283).abs())}else{v1});
        let v309=(if self.scalar_static_bool[60]{v0}else{(if (self.scalar_static_f64[169]!=0.0){((self.scalar_static_f64[173]+(self.scalar_static_f64[114]*(if (self.scalar_static_f64[169]!=0.0){v289}else{v1})))+(self.scalar_static_f64[115]*(if (self.scalar_static_f64[169]!=0.0){f64::powf((v0+(v294*(v294*v294))),0.3333333333333333)}else{v1})))}else{v1})});
        let v310=(v276*v309);
        let v311=(v279/v310);
        let v312=(-v279);
        let v321=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (self.scalar_static_f64[163]*v234));
        let v324=(if (v241!=0.0){(self.scalar_static_f64[164]*v244)}else{self.scalar_static_f64[164]});
        let v328=(if v252{(-(v255*(-v324)))}else{v324});
        let v332=((v261*v328)+(v259*(self.scalar_static_f64[141]*v328)));
        let v347=(v286*self.scalar_static_f64[180]);
        let v349=(v286*self.scalar_static_f64[181]);
        let v351=(2.0*v289);
        let v367=(v310*v310);
        let v368=((v310-(v279*(v276*(if self.scalar_static_bool[60]{v1}else{(if (self.scalar_static_f64[169]!=0.0){(self.scalar_static_f64[114]*(if (self.scalar_static_f64[169]!=0.0){((v347+v347)/v351)}else{v1}))}else{v1})}))))/v367);
        let v372=(((-v310)-(v279*(v276*(if self.scalar_static_bool[60]{v1}else{(if (self.scalar_static_f64[169]!=0.0){(self.scalar_static_f64[114]*(if (self.scalar_static_f64[169]!=0.0){((v349+v349)/v351)}else{v1}))}else{v1})}))))/v367);
        let v375=((-(v279*(v309*(self.scalar_static_f64[107]*(if (v267!=0.0){(v264*(v272*(v268*v332)))}else{v332})))))/v367);

        stamper.stamp_current_node3_local(
            Some(0),
            Some(1),
            multiplicity * (v311),
            0,
            multiplicity * (v368),
            1,
            multiplicity * (v372),
            2,
            multiplicity * (v375),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if (self.scalar_static_f64[164]!=0.0){(self.scalar_static_f64[156]*v234)}else{v1})),
            2,
            multiplicity * (self.scalar_static_f64[182]),
        );
        stamper.stamp_current_node3_local(
            Some(2),
            None,
            multiplicity * ((if (self.scalar_static_f64[164]!=0.0){(v311*v312)}else{v1})),
            0,
            multiplicity * ((if (self.scalar_static_f64[164]!=0.0){((v312*v368)+(-v311))}else{v1})),
            1,
            multiplicity * ((if (self.scalar_static_f64[164]!=0.0){(v311+(v312*v372))}else{v1})),
            2,
            multiplicity * ((if (self.scalar_static_f64[164]!=0.0){(v312*v375)}else{v1})),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if self.scalar_static_bool[61]{(1000000.0*v234)}else{v1})),
            2,
            multiplicity * (self.scalar_static_f64[183]),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if (self.scalar_static_f64[164]!=0.0){v321}else{v1})),
            2,
            multiplicity * ((if (self.scalar_static_f64[164]!=0.0){(self.scalar_static_f64[163]*ddt_scale)}else{v1})),
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

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let v1=0.0;
        let v321=0.0;

        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * ((if (self.scalar_static_f64[164]!=0.0){(self.scalar_static_f64[163]*1.0)}else{v1})),
        );
    }
}
