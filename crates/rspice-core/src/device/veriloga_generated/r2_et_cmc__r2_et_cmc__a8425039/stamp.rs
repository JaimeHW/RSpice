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
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let v0=1.0;
        let v1=0.0;
        let v8=0.01;
        let v6i=ctx.node_voltage(nodes[2]);
        let v6l=(sf[185]+(v6i*sf[164]));
        let v6p=(if (v6l<sf[166]){v0}else{v1});
        let v6s=(((v6l-sf[165])-v0)).exp();
        let v6u=(if (v6p!=0.0){(sf[165]+v6s)}else{v6l});
        let v70=(((if (v6u>sf[168]){v0}else{v1})!=0.0)&&(!(v6p!=0.0)));
        let v73=(((sf[167]-v6u)-v0)).exp();
        let v77=((273.15+(if v70{(sf[167]-v73)}else{v6u}))-sf[13]);
        let v79=(sf[137]+(sf[141]*v77));
        let v7b=(v0+(v77*v79));
        let v7c=0.1;
        let v7f=(if (v7b<0.11){v0}else{v1});
        let v7g=10.0;
        let v7k=(((v7g*(v7b-v8))-v0)).exp();
        let v7o=(sf[107]*(if (v7f!=0.0){(v8+(v7c*v7k))}else{v7b}));
        let v7r=(ctx.node_voltage(nodes[0])-ctx.node_voltage(nodes[1]));
        let v7v=(if (sf[169]!=0.0){(v7r/sf[113])}else{v1});
        let v7y=(if (sf[169]!=0.0){(v7v*sf[170])}else{v1});
        let v81=((v0+(v7y*v7y))).sqrt();
        let v86=(if (sf[169]!=0.0){(sf[171]*(v7v).abs())}else{v1});
        let v8l=(if sb[60]{v0}else{(if (sf[169]!=0.0){((sf[173]+(sf[114]*(if (sf[169]!=0.0){v81}else{v1})))+(sf[115]*(if (sf[169]!=0.0){f64::powf((v0+(v86*(v86*v86))),0.3333333333333333)}else{v1})))}else{v1})});
        let v8m=(v7o*v8l);
        let v8n=(v7r/v8m);
        let v8o=(-v7r);
        let v8x=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (sf[163]*v6i));
        let v90=(if (v6p!=0.0){(sf[164]*v6s)}else{sf[164]});
        let v94=(if v70{(-(v73*(-v90)))}else{v90});
        let v98=((v79*v94)+(v77*(sf[141]*v94)));
        let v9n=(v7y*sf[180]);
        let v9p=(v7y*sf[181]);
        let v9r=(2.0*v81);
        let va7=(v8m*v8m);
        let va8=((v8m-(v7r*(v7o*(if sb[60]{v1}else{(if (sf[169]!=0.0){(sf[114]*(if (sf[169]!=0.0){((v9n+v9n)/v9r)}else{v1}))}else{v1})}))))/va7);
        let vac=(((-v8m)-(v7r*(v7o*(if sb[60]{v1}else{(if (sf[169]!=0.0){(sf[114]*(if (sf[169]!=0.0){((v9p+v9p)/v9r)}else{v1}))}else{v1})}))))/va7);
        let vaf=((-(v7r*(v8l*(sf[107]*(if (v7f!=0.0){(v7c*(v7k*(v7g*v98)))}else{v98})))))/va7);

        stamper.stamp_current_node3_local(
            Some(0),
            Some(1),
            multiplicity * (v8n),
            0,
            multiplicity * (va8),
            1,
            multiplicity * (vac),
            2,
            multiplicity * (vaf),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if (sf[164]!=0.0){(sf[156]*v6i)}else{v1})),
            2,
            multiplicity * (sf[182]),
        );
        stamper.stamp_current_node3_local(
            Some(2),
            None,
            multiplicity * ((if (sf[164]!=0.0){(v8n*v8o)}else{v1})),
            0,
            multiplicity * ((if (sf[164]!=0.0){((v8o*va8)+(-v8n))}else{v1})),
            1,
            multiplicity * ((if (sf[164]!=0.0){(v8n+(v8o*vac))}else{v1})),
            2,
            multiplicity * ((if (sf[164]!=0.0){(v8o*vaf)}else{v1})),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if sb[61]{(1000000.0*v6i)}else{v1})),
            2,
            multiplicity * (sf[183]),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if (sf[164]!=0.0){v8x}else{v1})),
            2,
            multiplicity * ((if (sf[164]!=0.0){(sf[163]*ddt_scale)}else{v1})),
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
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let v1=0.0;
        let v8x=0.0;

        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * ((if (sf[164]!=0.0){(sf[163]*1.0)}else{v1})),
        );
    }
}
