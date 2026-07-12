#![allow(dead_code, non_snake_case, unused_imports, unused_parens, unused_variables)]

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
        let a=1.0;let b=0.0;let g=ctx.simparam_or("scale", a);let j=0.01;let p_=ctx.simparam_or("shrink", b);let u=1000000.0;let v=(((if sb[0]{g}else{sf[2]})*(if sb[1]{(a-(j*p_))}else{sf[7]}))*u);let a8=(v*sf[27]);let ab=(a8+sf[28]);let ad=1e99;let ag=(v*sf[25]);let ah=(if sb[16]{ag}else{b});let ai=(sf[19]+ag);let aj=(if sb[16]{ai}else{b});let al=(if (aj>b){a}else{b});let am=(sb[16]&&((al)!=0.0));let ax=(sb[16]&&(!((al)!=0.0)));let aZ=(if sb[27]{a8}else{(if sb[25]{b}else{(if sb[21]{a8}else{(if ax{a8}else{(if am{((aj*sf[30])-sf[28])}else{(if sb[14]{a8}else{b})})})})})});let b0=(if sb[27]{ab}else{(if sb[25]{b}else{(if sb[21]{ab}else{(if ax{ab}else{(if am{(sf[30]*(sf[19]+ah))}else{(if sb[14]{ab}else{b})})})})})});let b2=(if (b0>b){a}else{b});let b3=(sb[27]&&((b2)!=0.0));let bd=(sb[27]&&(!((b2)!=0.0)));let bz=(if sb[34]{a8}else{(if sb[32]{a8}else{(if sb[30]{b}else{aZ})})});let bA=(if sb[34]{ab}else{(if sb[32]{ab}else{(if sb[30]{b}else{b0})})});let bB=(if sb[34]{ag}else{(if sb[32]{b}else{(if sb[30]{ag}else{(if bd{ag}else{(if b3{((b0*sf[34])-sf[19])}else{(if sb[25]{ag}else{(if sb[21]{b}else{ah})})})})})})});let bC=(if sb[34]{ai}else{(if sb[32]{b}else{(if sb[30]{ai}else{(if bd{ai}else{(if b3{(sf[34]*(sf[28]+aZ))}else{(if sb[25]{ai}else{(if sb[21]{b}else{aj})})})})})})});let bE=(if (bA>b){a}else{b});let bG=(if (bC>b){a}else{b});let bH=(sb[34]&&((bE)!=0.0));let bT=(if (sb[34]&&(!((bE)!=0.0))){ad}else{(if (bH&&(!((bG)!=0.0))){b}else{(if (((bG)!=0.0)&&bH){(sf[29]*((sf[19]+bB)/(sf[28]+bz)))}else{(if sb[32]{b}else{(if sb[30]{ad}else{(if bd{ad}else{(if b3{sf[24]}else{(if sb[25]{ad}else{(if sb[21]{b}else{(if ax{b}else{(if am{sf[24]}else{b})})})})})})})})})})});let bZ=(if sb[35]{(bB+sf[37])}else{(bC+sf[37])});let c8=(((sf[13])!=0.0)&&((bG)!=0.0));let cc=(if c8{(sf[40]+(sf[42]/bC))}else{sf[40]});let cg=(if c8{(sf[41]+(sf[43]/bC))}else{sf[41]});let ci=(((sf[16])!=0.0)&&(sb[4]&&((bG)!=0.0)));let cy=(if ((bE)!=0.0){(sf[41]+(sf[47]/bA))}else{(if ci{(cg+(sf[45]/bC))}else{cg})});let cz=2.0;let cD=(bB*cz);let cG=(if sb[7]{cD}else{(if sb[5]{(bz+cD)}else{(if ((sf[13])!=0.0){(cz*(bz+bB))}else{b})})});let cH=(bz*bB);let cO=((sf[48]+(cG*sf[49]))+(cH*sf[50]));let cV=((sf[51]+(cG*sf[52]))+(cH*sf[53]));let cW=ctx.node_voltage(n[2]);let cZ=(sf[65]+(cW*sf[54]));let d3=(if (cZ<sf[56]){a}else{b});let d6=(((cZ-sf[55])-a)).exp();let d8=(if ((d3)!=0.0){(sf[55]+d6)}else{cZ});let de=((((if (d8>sf[58]){a}else{b}))!=0.0)&&(!((d3)!=0.0)));let dh=(((sf[57]-d8)-a)).exp();let dl=((273.15+(if de{(sf[57]-dh)}else{d8}))-sf[9]);let dn=((if ((bE)!=0.0){(sf[40]+(sf[46]/bA))}else{(if ci{(cc+(sf[44]/bC))}else{cc})})+(cy*dl));let dp=(a+(dl*dn));let dq=0.1;let du=10.0;let dy=(((du*(dp-j))-a)).exp();let dB=(!(((if (dp<0.11){a}else{b}))!=0.0));let dD=(bT*(if dB{dp}else{(j+(dq*dy))}));let dG=(ctx.node_voltage(n[0])-ctx.node_voltage(n[1]));let dI=(if ((bT>b)&&sb[38]){a}else{b});let dK=(if ((dI)!=0.0){(dG/bZ)}else{b});let dM=(dK*sf[59]);let dP=((a+(dM*dM))).sqrt();let dT=(sf[60]*(dK).abs());let e2=(!((dI)!=0.0));let e7=(if e2{a}else{((sf[62]+(sf[38]*(if ((dI)!=0.0){dP}else{b})))+(sf[39]*(if ((dI)!=0.0){f64::powf((a+(dT*(dT*dT))),0.3333333333333333)}else{b})))});let e8=(dD*e7);let e9=(dG/e8);let ea=(-dG);let ej=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (cV*cW));let em=(if ((d3)!=0.0){(sf[54]*d6)}else{sf[54]});let eq=(if de{(-(dh*(-em)))}else{em});let eu=((dn*eq)+(dl*(cy*eq)));let eH=(dM*(sf[59]*(if ((dI)!=0.0){(a/bZ)}else{b})));let eJ=(dM*(sf[59]*(if ((dI)!=0.0){(-1.0/bZ)}else{b})));let eL=(cz*dP);let eZ=(e8*e8);let f0=((e8-(dG*(dD*(if e2{b}else{(sf[38]*(if ((dI)!=0.0){((eH+eH)/eL)}else{b}))}))))/eZ);let f4=(((-e8)-(dG*(dD*(if e2{b}else{(sf[38]*(if ((dI)!=0.0){((eJ+eJ)/eL)}else{b}))}))))/eZ);let f7=((-(dG*(e7*(bT*(if dB{eu}else{(dq*(dy*(du*eu)))})))))/eZ);

        stamper.stamp_current_node3_local(
            Some(0),
            Some(1),
            multiplicity * (e9),
            0,
            multiplicity * (f0),
            1,
            multiplicity * (f4),
            2,
            multiplicity * (f7),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if ((sf[54])!=0.0){(cO*cW)}else{b})),
            2,
            multiplicity * ((if ((sf[54])!=0.0){cO}else{b})),
        );
        stamper.stamp_current_node3_local(
            Some(2),
            None,
            multiplicity * ((if ((sf[54])!=0.0){(e9*ea)}else{b})),
            0,
            multiplicity * ((if ((sf[54])!=0.0){((ea*f0)+(-e9))}else{b})),
            1,
            multiplicity * ((if ((sf[54])!=0.0){(e9+(ea*f4))}else{b})),
            2,
            multiplicity * ((if ((sf[54])!=0.0){(ea*f7)}else{b})),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if sb[39]{(u*cW)}else{b})),
            2,
            multiplicity * (sf[63]),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if ((sf[54])!=0.0){ej}else{b})),
            2,
            multiplicity * ((if ((sf[54])!=0.0){(cV*ddt_scale)}else{b})),
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
        let a=1.0;let b=0.0;let g=ctx.simparam_or("scale", a);let p_=ctx.simparam_or("shrink", b);let v=(((if sb[0]{g}else{sf[2]})*(if sb[1]{(a-(0.01*p_))}else{sf[7]}))*1000000.0);let a8=(v*sf[27]);let ab=(a8+sf[28]);let ag=(v*sf[25]);let ah=(if sb[16]{ag}else{b});let aj=(if sb[16]{(sf[19]+ag)}else{b});let al=(if (aj>b){a}else{b});let am=(sb[16]&&((al)!=0.0));let ax=(sb[16]&&(!((al)!=0.0)));let b0=(if sb[27]{ab}else{(if sb[25]{b}else{(if sb[21]{ab}else{(if ax{ab}else{(if am{(sf[30]*(sf[19]+ah))}else{(if sb[14]{ab}else{b})})})})})});let b2=(if (b0>b){a}else{b});let bz=(if sb[34]{a8}else{(if sb[32]{a8}else{(if sb[30]{b}else{(if sb[27]{a8}else{(if sb[25]{b}else{(if sb[21]{a8}else{(if ax{a8}else{(if am{((aj*sf[30])-sf[28])}else{(if sb[14]{a8}else{b})})})})})})})})});let bB=(if sb[34]{ag}else{(if sb[32]{b}else{(if sb[30]{ag}else{(if (sb[27]&&(!((b2)!=0.0))){ag}else{(if (sb[27]&&((b2)!=0.0)){((b0*sf[34])-sf[19])}else{(if sb[25]{ag}else{(if sb[21]{b}else{ah})})})})})})});let cz=2.0;let cD=(bB*cz);let cV=((sf[51]+((if sb[7]{cD}else{(if sb[5]{(bz+cD)}else{(if ((sf[13])!=0.0){(cz*(bz+bB))}else{b})})})*sf[52]))+((bz*bB)*sf[53]));let ej=0.0;

        stamper.stamp_current_reactive_node1_local(
            Some(2),
            None,
            2,
            multiplicity * ((if ((sf[54])!=0.0){(cV*1.0)}else{b})),
        );
    }
}
