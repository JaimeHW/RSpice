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
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let m=self.multiplicity;
        let multiplicity=m;
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
        let a=1.0;let b=0.0;let i=0.01;let cW=ctx.node_voltage(n[2]);let cZ=(sf[185]+(cW*sf[164]));let d3=(if (cZ<sf[166]){a}else{b});let d6=(((cZ-sf[165])-a)).exp();let d8=(if (d3!=0.0){(sf[165]+d6)}else{cZ});let de=(((if (d8>sf[168]){a}else{b})!=0.0)&&(!(d3!=0.0)));let dh=(((sf[167]-d8)-a)).exp();let dl=((273.15+(if de{(sf[167]-dh)}else{d8}))-sf[13]);let dn=(sf[137]+(sf[141]*dl));let dp=(a+(dl*dn));let dq=0.1;let dt=(if (dp<0.11){a}else{b});let du=10.0;let dy=(((du*(dp-i))-a)).exp();let dC=(sf[107]*(if (dt!=0.0){(i+(dq*dy))}else{dp}));let dF=(ctx.node_voltage(n[0])-ctx.node_voltage(n[1]));let dJ=(if (sf[169]!=0.0){(dF/sf[113])}else{b});let dM=(if (sf[169]!=0.0){(dJ*sf[170])}else{b});let dP=((a+(dM*dM))).sqrt();let dU=(if (sf[169]!=0.0){(sf[171]*(dJ).abs())}else{b});let e9=(if sb[60]{a}else{(if (sf[169]!=0.0){((sf[173]+(sf[114]*(if (sf[169]!=0.0){dP}else{b})))+(sf[115]*(if (sf[169]!=0.0){f64::powf((a+(dU*(dU*dU))),0.3333333333333333)}else{b})))}else{b})});let ea=(dC*e9);let eb=(dF/ea);let ec=(-dF);let el=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (sf[163]*cW));let eo=(if (d3!=0.0){(sf[164]*d6)}else{sf[164]});let es=(if de{(-(dh*(-eo)))}else{eo});let ew=((dn*es)+(dl*(sf[141]*es)));let eL=(dM*sf[180]);let eN=(dM*sf[181]);let eP=(2.0*dP);let f5=(ea*ea);let f6=((ea-(dF*(dC*(if sb[60]{b}else{(if (sf[169]!=0.0){(sf[114]*(if (sf[169]!=0.0){((eL+eL)/eP)}else{b}))}else{b})}))))/f5);let fa=(((-ea)-(dF*(dC*(if sb[60]{b}else{(if (sf[169]!=0.0){(sf[114]*(if (sf[169]!=0.0){((eN+eN)/eP)}else{b}))}else{b})}))))/f5);let fd=((-(dF*(e9*(sf[107]*(if (dt!=0.0){(dq*(dy*(du*ew)))}else{ew})))))/f5);

        stamper.stamp_current_node3_local(
            Some(0),
            Some(1),
            multiplicity * (eb),
            0,
            multiplicity * (f6),
            1,
            multiplicity * (fa),
            2,
            multiplicity * (fd),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if (sf[164]!=0.0){(sf[156]*cW)}else{b})),
            2,
            multiplicity * (sf[182]),
        );
        stamper.stamp_current_node3_local(
            Some(2),
            None,
            multiplicity * ((if (sf[164]!=0.0){(eb*ec)}else{b})),
            0,
            multiplicity * ((if (sf[164]!=0.0){((ec*f6)+(-eb))}else{b})),
            1,
            multiplicity * ((if (sf[164]!=0.0){(eb+(ec*fa))}else{b})),
            2,
            multiplicity * ((if (sf[164]!=0.0){(ec*fd)}else{b})),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if sb[61]{(1000000.0*cW)}else{b})),
            2,
            multiplicity * (sf[183]),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if (sf[164]!=0.0){el}else{b})),
            2,
            multiplicity * ((if (sf[164]!=0.0){(sf[163]*ddt_scale)}else{b})),
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

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        let br=self.branches;
        let branches=br;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p=&(*self.params);
        let m=self.multiplicity;
        let multiplicity=m;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let b=0.0;let el=0.0;

        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * ((if (sf[164]!=0.0){(sf[163]*1.0)}else{b})),
        );
    }
}
