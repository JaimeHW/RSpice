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
        let a=1.0;let b=0.0;let g=ctx.simparam_or("scale", a);let j=0.01;let p_=ctx.simparam_or("shrink", b);let u=1000000.0;let v=(((if sb[0]{g}else{sf[2]})*(if sb[1]{(a-(j*p_))}else{sf[7]}))*u);let a8=(v*sf[27]);let a9=(if sb[14]{a8}else{b});let ad=1e99;let ag=(v*sf[25]);let ah=(if sb[16]{ag}else{b});let aj=(if sb[16]{(sf[19]+ah)}else{b});let al=(if (aj>b){a}else{b});let am=(sb[16]&&((al)!=0.0));let aq=(if am{(aj*sf[30])}else{(if sb[14]{(a9+sf[28])}else{b})});let av=(sb[16]&&(!((al)!=0.0)));let aw=(if av{a8}else{(if am{(aq-sf[28])}else{a9})});let aJ=(if sb[21]{a8}else{aw});let aU=(if sb[25]{ag}else{(if sb[21]{b}else{ah})});let b0=(if sb[27]{a8}else{(if sb[25]{b}else{aJ})});let b2=(if sb[27]{(sf[28]+b0)}else{(if sb[25]{b}else{(if sb[21]{(sf[28]+aJ)}else{(if av{(sf[28]+aw)}else{aq})})})});let b4=(if (b2>b){a}else{b});let b5=(sb[27]&&((b4)!=0.0));let b8=(if b5{(b2*sf[34])}else{(if sb[25]{(sf[19]+aU)}else{(if sb[21]{b}else{aj})})});let bd=(sb[27]&&(!((b4)!=0.0)));let be=(if bd{ag}else{(if b5{(b8-sf[19])}else{aU})});let bn=(if sb[30]{ag}else{be});let bw=(if sb[32]{a8}else{(if sb[30]{b}else{b0})});let bC=(if sb[34]{a8}else{bw});let bE=(if sb[34]{(sf[28]+bC)}else{(if sb[32]{(sf[28]+bw)}else{(if sb[30]{b}else{b2})})});let bF=(if sb[34]{ag}else{(if sb[32]{b}else{bn})});let bH=(if sb[34]{(sf[19]+bF)}else{(if sb[32]{b}else{(if sb[30]{(sf[19]+bn)}else{(if bd{(sf[19]+be)}else{b8})})})});let bJ=(if (bE>b){a}else{b});let bL=(if (bH>b){a}else{b});let bM=(sb[34]&&((bJ)!=0.0));let bW=(if (sb[34]&&(!((bJ)!=0.0))){ad}else{(if (bM&&(!((bL)!=0.0))){b}else{(if (((bL)!=0.0)&&bM){(sf[29]*(bH/bE))}else{(if sb[32]{b}else{(if sb[30]{ad}else{(if bd{ad}else{(if b5{sf[24]}else{(if sb[25]{ad}else{(if sb[21]{b}else{(if av{b}else{(if am{sf[24]}else{b})})})})})})})})})})});let c3=(if sb[35]{(bF+sf[37])}else{(if ((sf[36])!=0.0){(bH+sf[37])}else{b})});let cc=(((sf[13])!=0.0)&&((bL)!=0.0));let cg=(if cc{(sf[40]+(sf[42]/bH))}else{sf[40]});let ck=(if cc{(sf[41]+(sf[43]/bH))}else{sf[41]});let cm=(((sf[16])!=0.0)&&(sb[4]&&((bL)!=0.0)));let cq=(if cm{(cg+(sf[44]/bH))}else{cg});let cu=(if cm{(ck+(sf[45]/bH))}else{ck});let cC=(if ((bJ)!=0.0){(cu+(sf[47]/bE))}else{cu});let cD=2.0;let cH=(bF*cD);let cK=(if sb[7]{cH}else{(if sb[5]{(bC+cH)}else{(if ((sf[13])!=0.0){(cD*(bC+bF))}else{b})})});let cL=(bC*bF);let cS=((sf[48]+(cK*sf[49]))+(cL*sf[50]));let cZ=((sf[51]+(cK*sf[52]))+(cL*sf[53]));let d0=ctx.node_voltage(n[2]);let d3=(sf[65]+(d0*sf[54]));let d7=(if (d3<sf[56]){a}else{b});let da=(((d3-sf[55])-a)).exp();let dc=(if ((d7)!=0.0){(sf[55]+da)}else{d3});let di=((((if (dc>sf[58]){a}else{b}))!=0.0)&&(!((d7)!=0.0)));let dl=(((sf[57]-dc)-a)).exp();let dp=((273.15+(if di{(sf[57]-dl)}else{dc}))-sf[9]);let dr=((if ((bJ)!=0.0){(cq+(sf[46]/bE))}else{cq})+(cC*dp));let dt=(a+(dp*dr));let du=0.1;let dx=(if (dt<0.11){a}else{b});let dy=10.0;let dC=(((dy*(dt-j))-a)).exp();let dG=(bW*(if ((dx)!=0.0){(j+(du*dC))}else{dt}));let dJ=(ctx.node_voltage(n[0])-ctx.node_voltage(n[1]));let dL=(if ((bW>b)&&sb[38]){a}else{b});let dN=(if ((dL)!=0.0){(dJ/c3)}else{b});let dQ=(if ((dL)!=0.0){(dN*sf[59])}else{b});let dT=((a+(dQ*dQ))).sqrt();let dY=(if ((dL)!=0.0){(sf[60]*(dN).abs())}else{b});let ec=(!((dL)!=0.0));let ed=(if ec{a}else{(if ((dL)!=0.0){((sf[62]+(sf[38]*(if ((dL)!=0.0){dT}else{b})))+(sf[39]*(if ((dL)!=0.0){f64::powf((a+(dY*(dY*dY))),0.3333333333333333)}else{b})))}else{b})});let ee=(dG*ed);let ef=(dJ/ee);let eg=(-dJ);let ep=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (cZ*d0));let es=(if ((d7)!=0.0){(sf[54]*da)}else{sf[54]});let ew=(if di{(-(dl*(-es)))}else{es});let eA=((dr*ew)+(dp*(cC*ew)));let eP=(dQ*(if ((dL)!=0.0){(sf[59]*(if ((dL)!=0.0){(a/c3)}else{b}))}else{b}));let eR=(dQ*(if ((dL)!=0.0){(sf[59]*(if ((dL)!=0.0){(-1.0/c3)}else{b}))}else{b}));let eT=(cD*dT);let f9=(ee*ee);let fa=((ee-(dJ*(dG*(if ec{b}else{(if ((dL)!=0.0){(sf[38]*(if ((dL)!=0.0){((eP+eP)/eT)}else{b}))}else{b})}))))/f9);
        let fe=(((-ee)-(dJ*(dG*(if ec{b}else{(if ((dL)!=0.0){(sf[38]*(if ((dL)!=0.0){((eR+eR)/eT)}else{b}))}else{b})}))))/f9);let fh=((-(dJ*(ed*(bW*(if ((dx)!=0.0){(du*(dC*(dy*eA)))}else{eA})))))/f9);

        stamper.stamp_current_node3_local(
            Some(0),
            Some(1),
            multiplicity * (ef),
            0,
            multiplicity * (fa),
            1,
            multiplicity * (fe),
            2,
            multiplicity * (fh),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if ((sf[54])!=0.0){(cS*d0)}else{b})),
            2,
            multiplicity * ((if ((sf[54])!=0.0){cS}else{b})),
        );
        stamper.stamp_current_node3_local(
            Some(2),
            None,
            multiplicity * ((if ((sf[54])!=0.0){(ef*eg)}else{b})),
            0,
            multiplicity * ((if ((sf[54])!=0.0){((eg*fa)+(-ef))}else{b})),
            1,
            multiplicity * ((if ((sf[54])!=0.0){(ef+(eg*fe))}else{b})),
            2,
            multiplicity * ((if ((sf[54])!=0.0){(eg*fh)}else{b})),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if sb[39]{(u*d0)}else{b})),
            2,
            multiplicity * (sf[63]),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if ((sf[54])!=0.0){ep}else{b})),
            2,
            multiplicity * ((if ((sf[54])!=0.0){(cZ*ddt_scale)}else{b})),
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
        let a=1.0;let b=0.0;let g=ctx.simparam_or("scale", a);let p_=ctx.simparam_or("shrink", b);let v=(((if sb[0]{g}else{sf[2]})*(if sb[1]{(a-(0.01*p_))}else{sf[7]}))*1000000.0);let a8=(v*sf[27]);let a9=(if sb[14]{a8}else{b});let ag=(v*sf[25]);let ah=(if sb[16]{ag}else{b});let aj=(if sb[16]{(sf[19]+ah)}else{b});let al=(if (aj>b){a}else{b});let am=(sb[16]&&((al)!=0.0));let aq=(if am{(aj*sf[30])}else{(if sb[14]{(a9+sf[28])}else{b})});let av=(sb[16]&&(!((al)!=0.0)));let aw=(if av{a8}else{(if am{(aq-sf[28])}else{a9})});let aJ=(if sb[21]{a8}else{aw});let aU=(if sb[25]{ag}else{(if sb[21]{b}else{ah})});let b0=(if sb[27]{a8}else{(if sb[25]{b}else{aJ})});let b2=(if sb[27]{(sf[28]+b0)}else{(if sb[25]{b}else{(if sb[21]{(sf[28]+aJ)}else{(if av{(sf[28]+aw)}else{aq})})})});let b4=(if (b2>b){a}else{b});let b5=(sb[27]&&((b4)!=0.0));let bC=(if sb[34]{a8}else{(if sb[32]{a8}else{(if sb[30]{b}else{b0})})});let bF=(if sb[34]{ag}else{(if sb[32]{b}else{(if sb[30]{ag}else{(if (sb[27]&&(!((b4)!=0.0))){ag}else{(if b5{((if b5{(b2*sf[34])}else{(if sb[25]{(sf[19]+aU)}else{(if sb[21]{b}else{aj})})})-sf[19])}else{aU})})})})});let cD=2.0;let cH=(bF*cD);let cZ=((sf[51]+((if sb[7]{cH}else{(if sb[5]{(bC+cH)}else{(if ((sf[13])!=0.0){(cD*(bC+bF))}else{b})})})*sf[52]))+((bC*bF)*sf[53]));let ep=0.0;

        stamper.stamp_current_reactive_node1_local(
            Some(2),
            None,
            2,
            multiplicity * ((if ((sf[54])!=0.0){(cZ*1.0)}else{b})),
        );
    }
}
