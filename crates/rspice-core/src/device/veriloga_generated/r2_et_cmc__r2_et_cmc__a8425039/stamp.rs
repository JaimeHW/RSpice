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
        let v2=0.0;
        let v5=1.0;
        let v8=0.01;
        let v222=ctx.node_voltage(nodes[2]);
        let v225=(self.scalar_static_f64[172]+(v222*self.scalar_static_f64[152]));
        let v228=(v225<self.scalar_static_f64[154]);
        let v231=(((v225-self.scalar_static_f64[153])-v5)).exp();
        let v233=(if v228{(self.scalar_static_f64[153]+v231)}else{v225});
        let v238=((v233>self.scalar_static_f64[156])&&(!v228));
        let v241=(((self.scalar_static_f64[155]-v233)-v5)).exp();
        let v245=((273.15+(if v238{(self.scalar_static_f64[155]-v241)}else{v233}))-self.scalar_static_f64[13]);
        let v247=(self.scalar_static_f64[125]+(self.scalar_static_f64[129]*v245));
        let v249=(v5+(v245*v247));
        let v250=0.1;
        let v252=(v249<0.11);
        let v253=10.0;
        let v257=(((v253*(v249-v8))-v5)).exp();
        let v261=(self.scalar_static_f64[95]*(if v252{(v8+(v250*v257))}else{v249}));
        let v264=(ctx.node_voltage(nodes[0])-ctx.node_voltage(nodes[1]));
        let v267=(if self.scalar_static_bool[59]{(v264/self.scalar_static_f64[101])}else{v2});
        let v270=(if self.scalar_static_bool[59]{(v267*self.scalar_static_f64[157])}else{v2});
        let v273=((v5+(v270*v270))).sqrt();
        let v278=(if self.scalar_static_bool[59]{(self.scalar_static_f64[158]*(v267).abs())}else{v2});
        let v293=(if self.scalar_static_bool[60]{v5}else{(if self.scalar_static_bool[59]{((self.scalar_static_f64[160]+(self.scalar_static_f64[102]*(if self.scalar_static_bool[59]{v273}else{v2})))+(self.scalar_static_f64[103]*(if self.scalar_static_bool[59]{f64::powf((v5+(v278*(v278*v278))),0.3333333333333333)}else{v2})))}else{v2})});
        let v294=(v261*v293);
        let v295=(v264/v294);
        let v296=(-v264);
        let v305=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (self.scalar_static_f64[151]*v222));
        let v308=(if v228{(self.scalar_static_f64[152]*v231)}else{self.scalar_static_f64[152]});
        let v312=(if v238{(-(v241*(-v308)))}else{v308});
        let v316=((v247*v312)+(v245*(self.scalar_static_f64[129]*v312)));
        let v331=(v270*self.scalar_static_f64[167]);
        let v333=(v270*self.scalar_static_f64[168]);
        let v335=(2.0*v273);
        let v351=(v294*v294);
        let v352=((v294-(v264*(v261*(if self.scalar_static_bool[60]{v2}else{(if self.scalar_static_bool[59]{(self.scalar_static_f64[102]*(if self.scalar_static_bool[59]{((v331+v331)/v335)}else{v2}))}else{v2})}))))/v351);
        let v356=(((-v294)-(v264*(v261*(if self.scalar_static_bool[60]{v2}else{(if self.scalar_static_bool[59]{(self.scalar_static_f64[102]*(if self.scalar_static_bool[59]{((v333+v333)/v335)}else{v2}))}else{v2})}))))/v351);
        let v359=((-(v264*(v293*(self.scalar_static_f64[95]*(if v252{(v250*(v257*(v253*v316)))}else{v316})))))/v351);

        stamper.stamp_current_node3_local(
            Some(0),
            Some(1),
            multiplicity * (v295),
            0,
            multiplicity * (v352),
            1,
            multiplicity * (v356),
            2,
            multiplicity * (v359),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if (self.scalar_static_f64[152]!=0.0){(self.scalar_static_f64[144]*v222)}else{v2})),
            2,
            multiplicity * (self.scalar_static_f64[169]),
        );
        stamper.stamp_current_node3_local(
            Some(2),
            None,
            multiplicity * ((if (self.scalar_static_f64[152]!=0.0){(v295*v296)}else{v2})),
            0,
            multiplicity * ((if (self.scalar_static_f64[152]!=0.0){((v296*v352)+(-v295))}else{v2})),
            1,
            multiplicity * ((if (self.scalar_static_f64[152]!=0.0){(v295+(v296*v356))}else{v2})),
            2,
            multiplicity * ((if (self.scalar_static_f64[152]!=0.0){(v296*v359)}else{v2})),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if self.scalar_static_bool[61]{(1000000.0*v222)}else{v2})),
            2,
            multiplicity * (self.scalar_static_f64[170]),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if (self.scalar_static_f64[152]!=0.0){v305}else{v2})),
            2,
            multiplicity * ((if (self.scalar_static_f64[152]!=0.0){(self.scalar_static_f64[151]*ddt_scale)}else{v2})),
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
        let v2=0.0;
        let v305=0.0;

        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * ((if (self.scalar_static_f64[152]!=0.0){(self.scalar_static_f64[151]*1.0)}else{v2})),
        );
    }
}
