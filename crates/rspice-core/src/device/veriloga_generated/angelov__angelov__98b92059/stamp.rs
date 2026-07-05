#![allow(dead_code, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

const LIMEXP_MAX: f64 = 5.54062238439351e34;

#[inline]
fn scalar_limexp(arg: f64) -> f64 {
    if arg < 80.0 { arg.exp() } else { LIMEXP_MAX * (1.0 + arg - 80.0) }
}

#[inline]
fn scalar_limexp_derivative(arg: f64) -> f64 {
    if arg < 80.0 { arg.exp() } else { LIMEXP_MAX }
}

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
        let br=self.branches;
        let branches=br;
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
        let a=ctx.node_voltage(n[8]);let b=ctx.node_voltage(n[5]);let c=(a-b);let d=ctx.node_voltage(n[4]);let e=ctx.node_voltage(n[3]);let f=(d-e);let g=(-f);let h=(e-b);let i=ctx.node_voltage(n[7]);let j=(i-e);let k=ctx.node_voltage(n[13]);let l=0.0;let E=ctx.node_voltage(n[11]);let H=(if (sf[10]!=0.0){(sf[160]+(E).abs())}else{sf[160]});let L=((H-sf[9])).abs();let Q=1.0;let R=(if ((L>l)||sb[2]){Q}else{l});let aZ=(!(R!=0.0));let b0=(if aZ{sf[12]}else{(if (R!=0.0){(sf[12]*(Q+(L*sf[13])))}else{l})});let b1=(if aZ{sf[14]}else{(if (R!=0.0){(sf[14]*(Q+(L*sf[15])))}else{l})});let b2=(if aZ{sf[16]}else{(if (R!=0.0){(sf[16]*(Q+(L*sf[17])))}else{l})});let b3=(if aZ{sf[18]}else{(if (R!=0.0){(sf[18]*(Q+(L*sf[19])))}else{l})});let b4=(if aZ{sf[20]}else{(if (R!=0.0){(sf[20]*(Q+(L*sf[21])))}else{l})});let b5=(if aZ{sf[22]}else{(if (R!=0.0){(sf[22]*(Q+(L*sf[23])))}else{l})});let b6=(if aZ{sf[24]}else{(if (R!=0.0){(sf[24]*(Q+(L*sf[25])))}else{l})});let b8=(if aZ{sf[28]}else{(if (R!=0.0){(sf[28]+(L*sf[30]))}else{l})});let b9=(if aZ{sf[31]}else{(if (R!=0.0){(sf[31]+(L*sf[33]))}else{l})});let ba=(if aZ{sf[34]}else{(if (R!=0.0){(sf[34]+(L*sf[35]))}else{l})});let bb=(if aZ{sf[36]}else{(if (R!=0.0){(sf[36]+(L*sf[37]))}else{l})});let bh=0.5;let bo=(if sb[5]{sf[43]}else{(if (sf[40]!=0.0){(sf[42]/(H*8.617333262145179e-5))}else{l})});let bq=(h*sf[44]);let br_=(bq).cosh();let bt=(br_*br_);let bw=(b1*(Q+(sf[45]/bt)));let bB=((h*sf[47])).tanh();let bG=(sf[48]*(g-sf[36]));let bH=(g-bb);let bJ=((((if aZ{sf[26]}else{(if (R!=0.0){(sf[26]+(L*sf[27]))}else{l})})-sf[46])+(sf[46]*bB))-(bG*bH));let bK=(c-bJ);let bL=(bK*bK);let bR=(bK*sf[50]);let bT=(((bw*bK)+(bL*sf[49]))+(bL*bR));let bU=(bT).tanh();let bV=(Q+bU);let bX=(-bT);let c1=((bh*(scalar_limexp(bT)-scalar_limexp(bX)))).tanh();let c5=(sf[51]+(sf[47]*bV));let c7=((h*c5)).tanh();let cj=(b0*bV);let ck=(c7*cj);let cp=(b2*scalar_limexp(bH));let cq=((Q+(h*sf[57]))+cp);let cv=(f-bJ);let cw=(if sb[11]{cv}else{br_});let cy=(if sb[11]{(cw*cw)}else{bK});let cA=(if sb[11]{(cw*cy)}else{bL});let cG=(if sb[11]{(((bw*cw)+(sf[49]*cy))+(sf[50]*cA))}else{l});let cH=(cG).tanh();let cJ=(if sb[11]{(Q+cH)}else{l});let cM=(if sb[11]{(sf[51]+(sf[47]*cJ))}else{l});let cQ=(if sb[11]{(sf[57]+(bV*sf[58]))}else{l});let cR=(Q+c7);let cS=(cj*cR);let cV=(h-bb);let cX=(b2*scalar_limexp(cV));let cY=((Q+(h*cQ))+cX);let d0=(if sb[11]{(cS*cY)}else{l});let d3=(if sb[11]{(sf[57]+(cJ*sf[58]))}else{l});let d5=((h*cM)).tanh();let d7=(b0*cJ);let d8=(Q-(if sb[11]{d5}else{l}));let d9=(d7*d8);let db=(Q-(h*d3));let dd=(if sb[11]{(d9*db)}else{l});let dk=(if sb[14]{bK}else{cw});let dm=(if sb[14]{(dk*dk)}else{cy});let dp=(sf[50]*dm);let dr=((dk+(sf[49]*dm))+(dk*dp));let dt=(if sb[14]{(bw*dr)}else{bT});let dv=(-dt);let dz=((bh*(scalar_limexp(dt)-scalar_limexp(dv)))).tanh();let dB=(if sb[14]{(Q+dz)}else{(Q+c1)});let dE=(if sb[14]{(sf[51]+(sf[47]*dB))}else{l});let dG=((h*dE)).tanh();let dH=(if sb[14]{dG}else{l});let dK=(if sb[14]{(sf[57]+(sf[58]*dB))}else{cQ});let dL=(b0*dB);let dM=(dH*dL);let dP=(cp+(Q+(h*dK)));let dV=(if sb[17]{bK}else{dk});let dX=(if sb[17]{(dV*dV)}else{dm});let e0=(sf[50]*dX);let e2=((dV+(sf[49]*dX))+(dV*e0));let e4=(if sb[17]{(bw*e2)}else{dt});let e5=(if sb[17]{cv}else{cA});let e7=(if sb[17]{(e5*e5)}else{l});let ea=(sf[50]*e5);let ec=((e5+(sf[49]*e7))+(e7*ea));let ee=(if sb[17]{(bw*ec)}else{cG});let eg=(-e4);let ek=((bh*(scalar_limexp(e4)-scalar_limexp(eg)))).tanh();let em=(if sb[17]{(Q+ek)}else{dB});let eo=(-ee);let es=((bh*(scalar_limexp(ee)-scalar_limexp(eo)))).tanh();let eu=(if sb[17]{(Q+es)}else{l});let ex=(if sb[17]{(sf[51]+(sf[47]*em))}else{dE});let eA=(if sb[17]{(sf[51]+(sf[47]*eu))}else{l});let eC=((h*ex)).tanh();let eF=((h*eA)).tanh();let eJ=(if sb[17]{(sf[57]+(sf[58]*eu))}else{l});let eM=(if sb[17]{(sf[57]+(sf[58]*em))}else{l});let eN=(b0*em);let eO=(Q+(if sb[17]{eC}else{dH}));let eP=(eN*eO);let eS=(cX+(Q+(h*eM)));let eV=(b0*eu);let eW=(Q-(if sb[17]{eF}else{l}));let eX=(eV*eW);let eZ=(Q-(h*eJ));let f8=(Q+bV);let fe=(bV*sf[62]);let fl=(Q+em);
        let fo=(if sb[19]{(sf[60]+(b5/fl))}else{(if (sf[59]!=0.0){(sf[60]+(b5/f8))}else{l})});let fp=(em*sf[62]);let fr=(if sb[19]{(sf[61]+fp)}else{(if (sf[59]!=0.0){(sf[61]+fe)}else{l})});let ft=(if sb[19]{(sf[63]+fp)}else{(if (sf[59]!=0.0){(fe+sf[63])}else{l})});let fv=(if ((L!=0.0)||sb[2]){Q}else{l});let fy=(Q+(L*sf[64]));let fD=(!(fv!=0.0));let fE=(if fD{fr}else{(if (fv!=0.0){(fr*fy)}else{l})});let fF=(if fD{ft}else{(if (fv!=0.0){(ft*fy)}else{l})});let fJ=-1.0;let fP=(c-ba);let fR=(j-ba);let fX=(if sb[21]{scalar_limexp((ba*(-bo)))}else{(if (sf[66]!=0.0){scalar_limexp((bo*((-ba)).tanh()))}else{dV})});let g1=(fP).tanh();let g3=(fR).tanh();let ga=(bo*(if sb[25]{fP}else{(if sb[23]{g1}else{(if (sf[66]!=0.0){fP}else{l})})}));let gd=(sf[68]*(scalar_limexp(ga)-fX));let ge=(bo*(if sb[25]{fR}else{(if sb[23]{g3}else{(if (sf[66]!=0.0){fR}else{l})})}));let gl=(h*sf[69]);let gm=((b8+(c*sf[29]))+gl);let gn=(gm).tanh();let gt=((sf[70]+(h*sf[71]))).tanh();let gu=(Q+gt);let gz=((sf[72]-(h*sf[73]))).tanh();let gB=((Q+gz)-sf[69]);let gE=((b9+(j*sf[32]))-gl);let gF=(gE).tanh();let gG=(Q+gF);let gU=(b3*(Q+gn));let h8=(if sb[33]{(gu-sf[69])}else{gu});let h9=(b8+gl);let hb=(if sb[33]{(h9).cosh()}else{l});let hf=(if sb[33]{(gm).cosh()}else{l});let hl=((gm+(if sb[33]{(hf).ln()}else{l}))-(if sb[33]{(h9+(if sb[33]{(hb).ln()}else{l}))}else{l}));let hu=(b9-gl);let hw=(if sb[33]{(hu).cosh()}else{hb});let hA=(if sb[33]{(gE).cosh()}else{hf});let hG=((gE+(if sb[33]{(hA).ln()}else{l}))-(if sb[33]{(hu+(if sb[33]{(hw).ln()}else{l}))}else{l}));let Bh=(gm).sinh();let Bn=(if sb[33]{(sf[29]*Bh)}else{l});let BY=(if sb[33]{(sf[78]+(b3*(sf[82]+((h8*(sf[29]+(if sb[33]{(Bn/hf)}else{l})))/sf[29]))))}else{l});let hP=BY;let hQ=(if sb[33]{hP}else{(if sb[30]{(sf[78]+(gu*gU))}else{sf[79]})});let C8=(gE).sinh();let CU=(if sb[33]{(sf[80]+(b4*(sf[82]+((gB*(sf[32]+(if sb[33]{((if sb[33]{(sf[32]*C8)}else{l})/hA)}else{l})))/sf[32]))))}else{l});let hR=CU;let hS=(if sb[33]{hR}else{(if sb[30]{(sf[80]+(b4*((gB*gG)+sf[82])))}else{sf[81]})});let iC=(if sb[49]{((b3*((H*5.5226012e-23)*sf[104]))*sf[106])}else{l});let iG=(if sb[49]{((Q-(iC*iC))).sqrt()}else{l});let iI=3.141592653589793;let iK=(if sb[49]{((-iC)*iI)}else{l});let iW=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (if sb[33]{((b4*(((gB*hG)/sf[32])+(j*sf[82])))+(j*sf[80]))}else{l}));let iY=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (if sb[33]{((b3*(((h8*hl)/sf[29])+(c*sf[82])))+(c*sf[78]))}else{l}));let j2=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (j*hS));let j5=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (c*hQ));let jd=ctx.node_voltage(n[10]);let jg=(jd-b);let jk=ctx.node_voltage(n[9]);let jB=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, (sf[92]*ctx.branch_current(br[6])));let jH=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, (sf[92]*ctx.branch_current(br[8])));let jJ=ctx.branch_current(br[10]);let jP=ctx.branch_current(br[14]);
        let jU=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, (sf[96]*ctx.branch_current(br[15])));let k0=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, (sf[96]*ctx.branch_current(br[17])));let k2=ctx.node_voltage(n[14]);let k3=(if sb[49]{k2}else{l});let k4=ctx.node_voltage(n[15]);let ka=(-(if sb[49]{(iC*iI)}else{l}));let kc=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, (k2*ka));let kg=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, (E*sf[114]));let kw=(bq).sinh();let kx=(sf[44]*kw);let ky=(sf[115]*kw);let kz=(br_*kx);let kB=(br_*ky);let kF=(bt*bt);let kK=(b1*((-(sf[45]*(kz+kz)))/kF));let kL=(b1*((-(sf[45]*(kB+kB)))/kF));let kO=(Q-(bB*bB));let kS=(sf[46]*(sf[116]*kO));let kY=((bH*sf[117])+(-bG));let kZ=((sf[46]*(sf[47]*kO))-(bG+(sf[48]*bH)));let l1=(-kZ);let l2=(fJ-kS);let l3=(bK*l1);let l4=(l3+l3);let l5=(bK*kY);let l6=(l5+l5);let l7=(bK*l2);let l8=(l7+l7);let l9=(bK+bK);let lE=((((bK*kK)+(bw*l1))+(sf[49]*l4))+((bR*l4)+(bL*(sf[50]*l1))));let lF=(((bw*kY)+(sf[49]*l6))+((bR*l6)+(bL*(sf[50]*kY))));let lG=((((bK*kL)+(bw*l2))+(sf[49]*l8))+((bR*l8)+(bL*(sf[50]*l2))));let lH=((bw+(sf[49]*l9))+((bR*l9)+(bL*sf[50])));let lJ=(Q-(bU*bU));let lK=(lE*lJ);let lL=(lF*lJ);let lM=(lG*lJ);let lN=(lH*lJ);let lO=scalar_limexp_derivative(bT);let lX=scalar_limexp_derivative(bX);let mb=(Q-(c1*c1));let ms=(Q-(c7*c7));let mx=(b0*lK);let my=(b0*lL);let mz=(b0*lM);let mA=(b0*lN);let mB=(cj*((c5+(h*(sf[47]*lK)))*ms));let mE=(cj*((h*(sf[47]*lL))*ms));let mH=(cj*(((-c5)+(h*(sf[47]*lM)))*ms));let mK=(cj*((h*(sf[47]*lN))*ms));let mO=scalar_limexp_derivative(bH);let mQ=(b2*mO);let mR=(b2*(-mO));let n7=(fJ-kZ);let n8=(Q-(-kY));let n9=(-kS);let na=(if sb[11]{n7}else{kx});let nb=(if sb[11]{n8}else{l});let nc=(if sb[11]{n9}else{ky});let nd=(cw*na);let nf=(cw*nb);let nh=(cw*nc);let nj=(if sb[11]{(nd+nd)}else{l1});let nk=(if sb[11]{(nf+nf)}else{kY});let nl=(if sb[11]{(nh+nh)}else{l2});let nx=(if sb[11]{((cy*na)+(cw*nj))}else{l4});let ny=(if sb[11]{((cy*nb)+(cw*nk))}else{l6});let nz=(if sb[11]{((cy*nc)+(cw*nl))}else{l8});let nA=(if sb[11]{(cw*sf[119])}else{l9});let nX=(if sb[11]{((((cw*kK)+(bw*na))+(sf[49]*nj))+(sf[50]*nx))}else{l});let nY=(if sb[11]{(((bw*nb)+(sf[49]*nk))+(sf[50]*ny))}else{l});let nZ=(if sb[11]{((((cw*kL)+(bw*nc))+(sf[49]*nl))+(sf[50]*nz))}else{l});let o0=(if sb[11]{(sf[120]+(sf[50]*nA))}else{l});let o2=(Q-(cH*cH));let o7=(if sb[11]{(nX*o2)}else{l});let o8=(if sb[11]{(nY*o2)}else{l});let o9=(if sb[11]{(nZ*o2)}else{l});let oa=(if sb[11]{(o0*o2)}else{l});let on=(if sb[11]{(sf[58]*lK)}else{l});let oo=(if sb[11]{(sf[58]*lL)}else{l});let op=(if sb[11]{(sf[58]*lM)}else{l});let oq=(if sb[11]{(sf[58]*lN)}else{l});let oG=scalar_limexp_derivative(cV);let oI=(b2*oG);let oJ=(b2*(-oG));let oY=(if sb[11]{((cY*(mB+(cR*mx)))+(cS*((cQ+(h*on))+oI)))}else{l});let oZ=(if sb[11]{((cY*(mE+(cR*my)))+(cS*(h*oo)))}else{l});let p0=(if sb[11]{((cY*(mH+(cR*mz)))+(cS*(((-cQ)+(h*op))+oJ)))}else{l});let p1=(if sb[11]{((cY*(mK+(cR*mA)))+(cS*(h*oq)))}else{l});let pi=(Q-(d5*d5));let q8=(if sb[11]{((db*((d8*(b0*o7))+(d7*(-(if sb[11]{((cM+(h*(if sb[11]{(sf[47]*o7)}else{l})))*pi)}else{l})))))+(d9*(-(d3+(h*(if sb[11]{(sf[58]*o7)}else{l}))))))}else{l});
        let q9=(if sb[11]{((db*((d8*(b0*o8))+(d7*(-(if sb[11]{((h*(if sb[11]{(sf[47]*o8)}else{l}))*pi)}else{l})))))+(d9*(-(h*(if sb[11]{(sf[58]*o8)}else{l})))))}else{l});let qa=(if sb[11]{((db*((d8*(b0*o9))+(d7*(-(if sb[11]{(((-cM)+(h*(if sb[11]{(sf[47]*o9)}else{l})))*pi)}else{l})))))+(d9*(-((-d3)+(h*(if sb[11]{(sf[58]*o9)}else{l}))))))}else{l});let qb=(if sb[11]{((db*((d8*(b0*oa))+(d7*(-(if sb[11]{((h*(if sb[11]{(sf[47]*oa)}else{l}))*pi)}else{l})))))+(d9*(-(h*(if sb[11]{(sf[58]*oa)}else{l})))))}else{l});let qo=(if sb[14]{l1}else{na});let qp=(if sb[14]{kY}else{nb});let qq=(if sb[14]{l2}else{nc});let qs=(dk*qo);let qu=(dk*qp);let qw=(dk*qq);let qy=(dk*sf[121]);let qA=(if sb[14]{(qs+qs)}else{nj});let qB=(if sb[14]{(qu+qu)}else{nk});let qC=(if sb[14]{(qw+qw)}else{nl});let qD=(if sb[14]{(qy+qy)}else{sf[119]});let re=(if sb[14]{((dr*kK)+(bw*((qo+(sf[49]*qA))+((dp*qo)+(dk*(sf[50]*qA))))))}else{lE});let rf=(if sb[14]{(bw*((qp+(sf[49]*qB))+((dp*qp)+(dk*(sf[50]*qB)))))}else{lF});let rg=(if sb[14]{((dr*kL)+(bw*((qq+(sf[49]*qC))+((dp*qq)+(dk*(sf[50]*qC))))))}else{lG});let rh=(if sb[14]{(bw*((sf[121]+(sf[49]*qD))+((dp*sf[121])+(dk*(sf[50]*qD)))))}else{lH});let ri=scalar_limexp_derivative(dt);let rr=scalar_limexp_derivative(dv);let rF=(Q-(dz*dz));let rK=(if sb[14]{((bh*((re*ri)-((-re)*rr)))*rF)}else{((bh*((lE*lO)-((-lE)*lX)))*mb)});let rL=(if sb[14]{((bh*((rf*ri)-((-rf)*rr)))*rF)}else{((bh*((lF*lO)-((-lF)*lX)))*mb)});let rM=(if sb[14]{((bh*((rg*ri)-((-rg)*rr)))*rF)}else{((bh*((lG*lO)-((-lG)*lX)))*mb)});let rN=(if sb[14]{((bh*((rh*ri)-((-rh)*rr)))*rF)}else{((bh*((lH*lO)-((-lH)*lX)))*mb)});let rS=(if sb[14]{(sf[47]*rK)}else{l});let rT=(if sb[14]{(sf[47]*rL)}else{l});let rU=(if sb[14]{(sf[47]*rM)}else{l});let rV=(if sb[14]{(sf[47]*rN)}else{l});let s4=(Q-(dG*dG));let s9=(if sb[14]{((dE+(h*rS))*s4)}else{l});let sa=(if sb[14]{((h*rT)*s4)}else{l});let sb_=(if sb[14]{(((-dE)+(h*rU))*s4)}else{l});let sc=(if sb[14]{((h*rV)*s4)}else{l});let t0=(if sb[17]{l1}else{qo});let t1=(if sb[17]{kY}else{qp});let t2=(if sb[17]{l2}else{qq});let t4=(dV*t0);let t6=(dV*t1);let t8=(dV*t2);let ta=(dV*sf[122]);let tc=(if sb[17]{(t4+t4)}else{qA});let td=(if sb[17]{(t6+t6)}else{qB});let te=(if sb[17]{(t8+t8)}else{qC});let tf=(if sb[17]{(ta+ta)}else{qD});let tQ=(if sb[17]{((e2*kK)+(bw*((t0+(sf[49]*tc))+((e0*t0)+(dV*(sf[50]*tc))))))}else{re});let tR=(if sb[17]{(bw*((t1+(sf[49]*td))+((e0*t1)+(dV*(sf[50]*td)))))}else{rf});let tS=(if sb[17]{((e2*kL)+(bw*((t2+(sf[49]*te))+((e0*t2)+(dV*(sf[50]*te))))))}else{rg});let tT=(if sb[17]{(bw*((sf[122]+(sf[49]*tf))+((e0*sf[122])+(dV*(sf[50]*tf)))))}else{rh});let tU=(if sb[17]{n7}else{nx});let tV=(if sb[17]{n8}else{ny});let tW=(if sb[17]{n9}else{nz});let tX=(if sb[17]{l}else{nA});let tY=(e5*tU);let u0=(e5*tV);let u2=(e5*tW);let u4=(e5*tX);let u6=(if sb[17]{(tY+tY)}else{l});let u7=(if sb[17]{(u0+u0)}else{l});let u8_=(if sb[17]{(u2+u2)}else{l});let u9=(if sb[17]{(u4+u4)}else{l});let uK=(if sb[17]{((ec*kK)+(bw*((tU+(sf[49]*u6))+((ea*u6)+(e7*(sf[50]*tU))))))}else{nX});let uL=(if sb[17]{(bw*((tV+(sf[49]*u7))+((ea*u7)+(e7*(sf[50]*tV)))))}else{nY});let uM=(if sb[17]{((ec*kL)+(bw*((tW+(sf[49]*u8_))+((ea*u8_)+(e7*(sf[50]*tW))))))}else{nZ});let uN=(if sb[17]{(bw*((tX+(sf[49]*u9))+((ea*u9)+(e7*(sf[50]*tX)))))}else{o0});let uO=scalar_limexp_derivative(e4);let uX=scalar_limexp_derivative(eg);let vb=(Q-(ek*ek));let vg=(if sb[17]{((bh*((tQ*uO)-((-tQ)*uX)))*vb)}else{rK});let vh=(if sb[17]{((bh*((tR*uO)-((-tR)*uX)))*vb)}else{rL});let vi=(if sb[17]{((bh*((tS*uO)-((-tS)*uX)))*vb)}else{rM});let vj=(if sb[17]{((bh*((tT*uO)-((-tT)*uX)))*vb)}else{rN});let vk=scalar_limexp_derivative(ee);let vt=scalar_limexp_derivative(eo);let vH=(Q-(es*es));let vM=(if sb[17]{((bh*((uK*vk)-((-uK)*vt)))*vH)}else{l});let vN=(if sb[17]{((bh*((uL*vk)-((-uL)*vt)))*vH)}else{l});let vO=(if sb[17]{((bh*((uM*vk)-((-uM)*vt)))*vH)}else{l});let vP=(if sb[17]{((bh*((uN*vk)-((-uN)*vt)))*vH)}else{l});let we=(Q-(eC*eC));let wv=(Q-(eF*eF));
        let ys=(if sb[17]{(bh*((if sb[17]{((eS*((eO*(b0*vg))+(eN*(if sb[17]{((ex+(h*(if sb[17]{(sf[47]*vg)}else{rS})))*we)}else{s9}))))+(eP*(oI+(eM+(h*(if sb[17]{(sf[58]*vg)}else{l}))))))}else{oY})-(if sb[17]{((eZ*((eW*(b0*vM))+(eV*(-(if sb[17]{((eA+(h*(if sb[17]{(sf[47]*vM)}else{l})))*wv)}else{l})))))+(eX*(-(eJ+(h*(if sb[17]{(sf[58]*vM)}else{l}))))))}else{q8})))}else{(if sb[14]{((dP*((dL*s9)+(dH*(b0*rK))))+(dM*(mQ+(dK+(h*(if sb[14]{(sf[58]*rK)}else{on}))))))}else{(if sb[11]{(bh*(oY-q8))}else{(if (sf[53]!=0.0){((cq*(mB+(c7*mx)))+(ck*(sf[57]+mQ)))}else{l})})})});let yu=(if sb[17]{(bh*((if sb[17]{((eS*((eO*(b0*vi))+(eN*(if sb[17]{(((-ex)+(h*(if sb[17]{(sf[47]*vi)}else{rU})))*we)}else{sb_}))))+(eP*(oJ+((-eM)+(h*(if sb[17]{(sf[58]*vi)}else{l}))))))}else{p0})-(if sb[17]{((eZ*((eW*(b0*vO))+(eV*(-(if sb[17]{(((-eA)+(h*(if sb[17]{(sf[47]*vO)}else{l})))*wv)}else{l})))))+(eX*(-((-eJ)+(h*(if sb[17]{(sf[58]*vO)}else{l}))))))}else{qa})))}else{(if sb[14]{((dP*((dL*sb_)+(dH*(b0*rM))))+(dM*((-dK)+(h*(if sb[14]{(sf[58]*rM)}else{op})))))}else{(if sb[11]{(bh*(p0-qa))}else{(if (sf[53]!=0.0){((cq*(mH+(c7*mz)))+(ck*sf[118]))}else{l})})})});let yy=(f8*f8);let yX=(fl*fl);let zg=(if sb[19]{(sf[62]*vg)}else{(if (sf[59]!=0.0){(sf[62]*lK)}else{l})});let zh=(if sb[19]{(sf[62]*vh)}else{(if (sf[59]!=0.0){(sf[62]*lL)}else{l})});let zi=(if sb[19]{(sf[62]*vi)}else{(if (sf[59]!=0.0){(sf[62]*lM)}else{l})});let zj=(if sb[19]{(sf[62]*vj)}else{(if (sf[59]!=0.0){(sf[62]*lN)}else{l})});let zs=(if fD{zg}else{(if (fv!=0.0){(fy*zg)}else{l})});let zt=(if fD{zh}else{(if (fv!=0.0){(fy*zh)}else{l})});let zu=(if fD{zi}else{(if (fv!=0.0){(fy*zi)}else{l})});let zv=(if fD{zj}else{(if (fv!=0.0){(fy*zj)}else{l})});let zC=(if sb[21]{l}else{(if (sf[66]!=0.0){l}else{t0})});let zE=(if sb[21]{l}else{(if (sf[66]!=0.0){l}else{t2})});let zH=(Q-(g1*g1));let zM=(Q-(g3*g3));let zW=scalar_limexp_derivative(ga);let A4=(sf[68]*(-(if sb[21]{l}else{(if (sf[66]!=0.0){l}else{t1})})));let A9=scalar_limexp_derivative(ge);let An=(Q-(gn*gn));let At=(Q-(gt*gt));let Au=(sf[71]*At);let Av=(sf[132]*At);let Ay=(Q-(gz*gz));let Az=(sf[133]*Ay);let AA=(sf[73]*Ay);let AE=(Q-(gF*gF));let B8=(h9).sinh();let Bb=(if sb[33]{(sf[69]*B8)}else{l});let Bc=(if sb[33]{(sf[130]*B8)}else{l});let Bl=(if sb[33]{(sf[69]*Bh)}else{l});let Bm=(if sb[33]{(sf[131]*Bh)}else{l});let BZ=(hu).sinh();let D3=(-(if sb[17]{(bh*((if sb[17]{((eS*((eO*(b0*vh))+(eN*(if sb[17]{((h*(if sb[17]{(sf[47]*vh)}else{rT}))*we)}else{sa}))))+(eP*(h*(if sb[17]{(sf[58]*vh)}else{l}))))}else{oZ})-(if sb[17]{((eZ*((eW*(b0*vN))+(eV*(-(if sb[17]{((h*(if sb[17]{(sf[47]*vN)}else{l}))*wv)}else{l})))))+(eX*(-(h*(if sb[17]{(sf[58]*vN)}else{l})))))}else{q9})))}else{(if sb[14]{((dP*((dL*sa)+(dH*(b0*rL))))+(dM*(mR+(h*(if sb[14]{(sf[58]*rL)}else{oo})))))}else{(if sb[11]{(bh*(oZ-q9))}else{(if (sf[53]!=0.0){((cq*(mE+(c7*my)))+(ck*mR))}else{l})})})}));let D6=ddt_scale;let DO=(fo*fo);let Ek=(sf[92]*D6);let EF=(sf[96]*D6);

        stamper.stamp_current_sparse_local::<4, 0>(
            Some(12),
            None,
            multiplicity * ((-(if sb[17]{(bh*((if sb[17]{(eP*eS)}else{d0})-(if sb[17]{(eX*eZ)}else{dd})))}else{(if sb[14]{(dM*dP)}else{(if sb[11]{(bh*(d0-dd))}else{(if (sf[53]!=0.0){(ck*cq)}else{l})})})}))),
            [3, 4, 5, 8],
            [(-ys), D3, (-yu), (-(if sb[17]{(bh*((if sb[17]{((eS*((eO*(b0*vj))+(eN*(if sb[17]{((h*(if sb[17]{(sf[47]*vj)}else{rV}))*we)}else{sc}))))+(eP*(h*(if sb[17]{(sf[58]*vj)}else{l}))))}else{p1})-(if sb[17]{((eZ*((eW*(b0*vP))+(eV*(-(if sb[17]{((h*(if sb[17]{(sf[47]*vP)}else{l}))*wv)}else{l})))))+(eX*(-(h*(if sb[17]{(sf[58]*vP)}else{l})))))}else{qb})))}else{(if sb[14]{((dP*((dL*sc)+(dH*(b0*rN))))+(dM*(h*(if sb[14]{(sf[58]*rN)}else{oq}))))}else{(if sb[11]{(bh*(p1-qb))}else{(if (sf[53]!=0.0){(cq*(mK+(c7*mA)))}else{l})})})}))],
            [],
            [],
            multiplicity,
        );
        let iS_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (sf[108]*ctx.node_voltage(n[12])));
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (iS_ddt),
            12,
            multiplicity * (((sf[108]) * ddt_scale)),
        );
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (k),
            13,
            multiplicity * (Q),
        );
        let iV_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (sf[109]*ctx.branch_current(br[0])));
        stamper.stamp_potential_branch_local(
            Some(12),
            Some(13),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            0,
            iV_ddt,
            0,
            ((sf[109]) * ddt_scale),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            Some(5),
            multiplicity * (k),
            13,
            multiplicity * (Q),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (gd),
            [3, 4, 5, 8],
            [(sf[68]*(-zC)), A4, (sf[68]*(((bo*(if sb[25]{fJ}else{(if sb[23]{(-zH)}else{sf[124]})}))*zW)-zE)), (sf[68]*(((bo*(if sb[25]{Q}else{(if sb[23]{zH}else{sf[125]})}))*zW)-sf[126]))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * ((sf[68]*(scalar_limexp(ge)-fX))),
            [3, 4, 5, 7, 8],
            [(sf[68]*(((bo*(if sb[25]{fJ}else{(if sb[23]{(-zM)}else{sf[124]})}))*A9)-zC)), A4, (sf[68]*(-zE)), (sf[68]*((bo*(if sb[25]{Q}else{(if sb[23]{zM}else{sf[125]})}))*A9)), sf[128]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * ((if (sf[77]!=0.0){iW}else{l})),
            [3, 5, 7, 8],
            [(if (sf[77]!=0.0){((if sb[33]{((b4*(sf[136]+(((hG*Az)+(gB*((sf[135]+(if sb[33]{((if sb[33]{(sf[135]*C8)}else{Bl})/hA)}else{l}))-(if sb[33]{(sf[130]+(if sb[33]{((if sb[33]{(sf[130]*BZ)}else{Bb})/hw)}else{l}))}else{l}))))/sf[32])))+sf[138])}else{l})*D6)}else{l}), (if (sf[77]!=0.0){((if sb[33]{(b4*(((hG*AA)+(gB*((sf[69]+(if sb[33]{((if sb[33]{(sf[69]*C8)}else{Bm})/hA)}else{l}))-(if sb[33]{(sf[69]+(if sb[33]{((if sb[33]{(sf[69]*BZ)}else{Bc})/hw)}else{l}))}else{l}))))/sf[32]))}else{l})*D6)}else{l}), (if (sf[77]!=0.0){(CU*D6)}else{l}), (if (sf[77]!=0.0){((if sb[33]{(b4*((gB*(if sb[33]{((if sb[33]{l}else{Bn})/hA)}else{l}))/sf[32]))}else{l})*D6)}else{l})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * ((if (sf[77]!=0.0){iY}else{l})),
            3,
            multiplicity * ((if (sf[77]!=0.0){((if sb[33]{(b3*(((hl*Au)+(h8*((sf[69]+(if sb[33]{(Bl/hf)}else{l}))-(if sb[33]{(sf[69]+(if sb[33]{(Bb/hb)}else{l}))}else{l}))))/sf[29]))}else{l})*D6)}else{l})),
            5,
            multiplicity * ((if (sf[77]!=0.0){((if sb[33]{((b3*((((hl*Av)+(h8*((sf[131]+(if sb[33]{(Bm/hf)}else{l}))-(if sb[33]{(sf[130]+(if sb[33]{(Bc/hb)}else{l}))}else{l}))))/sf[29])+sf[136]))+sf[137])}else{l})*D6)}else{l})),
            8,
            multiplicity * ((if (sf[77]!=0.0){(BY*D6)}else{l})),
        );
        stamper.stamp_current_node3_local(
            Some(7),
            Some(3),
            multiplicity * ((if sb[51]{j2}else{l})),
            3,
            multiplicity * ((if sb[51]{(D6*((-hS)+(j*(if sb[33]{l}else{(if sb[30]{(b4*((gG*Az)+(gB*(sf[135]*AE))))}else{l})}))))}else{l})),
            5,
            multiplicity * ((if sb[51]{(D6*(j*(if sb[33]{l}else{(if sb[30]{(b4*((gG*AA)+(gB*(sf[69]*AE))))}else{l})})))}else{l})),
            7,
            multiplicity * ((if sb[51]{(D6*(hS+(j*(if sb[33]{l}else{(if sb[30]{(b4*(gB*(sf[32]*AE)))}else{l})}))))}else{l})),
        );
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * ((if sb[51]{j5}else{l})),
            3,
            multiplicity * ((if sb[51]{(D6*(c*(if sb[33]{l}else{(if sb[30]{((gU*Au)+(gu*(b3*(sf[69]*An))))}else{l})})))}else{l})),
            5,
            multiplicity * ((if sb[51]{(D6*((-hQ)+(c*(if sb[33]{l}else{(if sb[30]{((gU*Av)+(gu*(b3*(sf[131]*An))))}else{l})}))))}else{l})),
            8,
            multiplicity * ((if sb[51]{(D6*(hQ+(c*(if sb[33]{l}else{(if sb[30]{(gu*(b3*(sf[29]*An)))}else{l})}))))}else{l})),
        );
        let ja_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (sf[110]*(ctx.node_voltage(n[1])-e)));
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (ja_ddt),
            1,
            multiplicity * (((sf[110]) * ddt_scale)),
            3,
            multiplicity * (((sf[139]) * ddt_scale)),
        );
        let jc_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, (h*sf[111]));
        stamper.stamp_current_node2_local(
            Some(3),
            Some(5),
            multiplicity * (jc_ddt),
            3,
            multiplicity * (((sf[111]) * ddt_scale)),
            5,
            multiplicity * (((sf[140]) * ddt_scale)),
        );
        let jf_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, (b6*(e-jd)));
        stamper.stamp_current_node2_local(
            Some(3),
            Some(10),
            multiplicity * (jf_ddt),
            3,
            multiplicity * (((b6) * ddt_scale)),
            10,
            multiplicity * ((((-b6)) * ddt_scale)),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * ((if (sf[83]!=0.0){(jg/fo)}else{l})),
            [3, 4, 5, 8, 10],
            [(if (sf[83]!=0.0){((-(jg*(if sb[19]{((-(b5*vg))/yX)}else{(if (sf[59]!=0.0){((-(b5*lK))/yy)}else{l})})))/DO)}else{l}), (if (sf[83]!=0.0){((-(jg*(if sb[19]{((-(b5*vh))/yX)}else{(if (sf[59]!=0.0){((-(b5*lL))/yy)}else{l})})))/DO)}else{l}), (if (sf[83]!=0.0){(((-fo)-(jg*(if sb[19]{((-(b5*vi))/yX)}else{(if (sf[59]!=0.0){((-(b5*lM))/yy)}else{l})})))/DO)}else{l}), (if (sf[83]!=0.0){((-(jg*(if sb[19]{((-(b5*vj))/yX)}else{(if (sf[59]!=0.0){((-(b5*lN))/yy)}else{l})})))/DO)}else{l}), (if (sf[83]!=0.0){(Q/fo)}else{l})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(5),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            l,
        );
        let jm_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, (sf[112]*(jk-a)));
        stamper.stamp_current_node2_local(
            Some(9),
            Some(8),
            multiplicity * (jm_ddt),
            8,
            multiplicity * (((sf[141]) * ddt_scale)),
            9,
            multiplicity * (((sf[112]) * ddt_scale)),
        );
        stamper.stamp_current_node2_local(
            Some(9),
            Some(5),
            multiplicity * ((if (sf[85]!=0.0){((jk-b)/sf[84])}else{l})),
            5,
            multiplicity * (sf[144]),
            9,
            multiplicity * (sf[145]),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(5),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            l,
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(7),
            multiplicity * ((if (sf[87]!=0.0){((d-i)/sf[86])}else{l})),
            4,
            multiplicity * (sf[148]),
            7,
            multiplicity * (sf[149]),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(7),
            multiplicity * (l),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(7),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            l,
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(8),
            multiplicity * ((if (sf[89]!=0.0){((d-a)/sf[88])}else{l})),
            4,
            multiplicity * (sf[152]),
            8,
            multiplicity * (sf[153]),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            l,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            5,
            (if (sf[91]!=0.0){(sf[90]*ctx.branch_current(br[5]))}else{l}),
            5,
            sf[154],
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            6,
            (if (sf[91]!=0.0){jB}else{l}),
            6,
            (if (sf[91]!=0.0){Ek}else{l}),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            l,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            8,
            (if sb[53]{jH}else{l}),
            8,
            (if sb[53]{Ek}else{l}),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            l,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            10,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<4, 1>(
            10,
            (if (sf[94]!=0.0){(fF*jJ)}else{l}),
            [3, 4, 5, 8],
            [(if (sf[94]!=0.0){(jJ*zs)}else{l}), (if (sf[94]!=0.0){(jJ*zt)}else{l}), (if (sf[94]!=0.0){(jJ*zu)}else{l}), (if (sf[94]!=0.0){(jJ*zv)}else{l})],
            [10],
            [(if (sf[94]!=0.0){fF}else{l})],
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            11,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            11,
            l,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            l,
        );
        let jO_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, (sf[113]*ctx.branch_current(br[13])));
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(2),
            13,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            13,
            jO_ddt,
            13,
            ((sf[113]) * ddt_scale),
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            14,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<4, 1>(
            14,
            (if (sf[95]!=0.0){(fE*jP)}else{l}),
            [3, 4, 5, 8],
            [(if (sf[95]!=0.0){(jP*zs)}else{l}), (if (sf[95]!=0.0){(jP*zt)}else{l}), (if (sf[95]!=0.0){(jP*zu)}else{l}), (if (sf[95]!=0.0){(jP*zv)}else{l})],
            [14],
            [(if (sf[95]!=0.0){fE}else{l})],
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            15,
            (if (sf[95]!=0.0){jU}else{l}),
            15,
            (if (sf[95]!=0.0){EF}else{l}),
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            l,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            17,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            17,
            (if sb[55]{k0}else{l}),
            17,
            (if sb[55]{EF}else{l}),
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            18,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            18,
            l,
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (l),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (l),
        );
        stamper.stamp_current_const_local(
            Some(14),
            None,
            multiplicity * (l),
        );
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (k3),
            14,
            multiplicity * (sf[155]),
        );
        stamper.stamp_current_const_local(
            Some(15),
            None,
            multiplicity * (l),
        );
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * ((if sb[49]{k4}else{l})),
            15,
            multiplicity * (sf[155]),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            Some(5),
            multiplicity * (k3),
            14,
            multiplicity * (sf[155]),
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * ((if sb[49]{((iK*k2)+(iG*k4))}else{l})),
            14,
            multiplicity * ((if sb[49]{iK}else{l})),
            15,
            multiplicity * ((if sb[49]{iG}else{l})),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            Some(3),
            multiplicity * ((if sb[49]{kc}else{l})),
            14,
            multiplicity * ((if sb[49]{(ka*D6)}else{l})),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (l),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (l),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (l),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (l),
        );
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (k2),
            14,
            multiplicity * (Q),
        );
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (k4),
            15,
            multiplicity * (Q),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (l),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (l),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (l),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (l),
        );
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * ((if (sf[107]!=0.0){kg}else{l})),
            11,
            multiplicity * ((if (sf[107]!=0.0){(sf[114]*D6)}else{l})),
        );
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * ((if (sf[107]!=0.0){(-(((h*(-k))+(c*gd))).abs())}else{l})),
        );
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * ((if (sf[107]!=0.0){(E/sf[11])}else{l})),
            11,
            multiplicity * (sf[157]),
        );
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * ((if sb[56]{(E*1e-12)}else{l})),
            11,
            multiplicity * (sf[158]),
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
        let a=ctx.node_voltage(n[8]);let b=ctx.node_voltage(n[5]);let c=(a-b);let e=ctx.node_voltage(n[3]);let h=(e-b);let j=(ctx.node_voltage(n[7])-e);let l=0.0;let E=ctx.node_voltage(n[11]);let H=(if (sf[10]!=0.0){(sf[160]+(E).abs())}else{sf[160]});let L=((H-sf[9])).abs();let Q=1.0;let R=(if ((L>l)||sb[2]){Q}else{l});let aZ=(!(R!=0.0));let b3=(if aZ{sf[18]}else{(if (R!=0.0){(sf[18]*(Q+(L*sf[19])))}else{l})});let b4=(if aZ{sf[20]}else{(if (R!=0.0){(sf[20]*(Q+(L*sf[21])))}else{l})});let b6=(if aZ{sf[24]}else{(if (R!=0.0){(sf[24]*(Q+(L*sf[25])))}else{l})});let b8=(if aZ{sf[28]}else{(if (R!=0.0){(sf[28]+(L*sf[30]))}else{l})});let b9=(if aZ{sf[31]}else{(if (R!=0.0){(sf[31]+(L*sf[33]))}else{l})});let gl=(h*sf[69]);let gm=((b8+(c*sf[29]))+gl);let gn=(gm).tanh();let gt=((sf[70]+(h*sf[71]))).tanh();let gu=(Q+gt);let gz=((sf[72]-(h*sf[73]))).tanh();let gB=((Q+gz)-sf[69]);let gE=((b9+(j*sf[32]))-gl);let gF=(gE).tanh();let gG=(Q+gF);let gU=(b3*(Q+gn));let h8=(if sb[33]{(gu-sf[69])}else{gu});let h9=(b8+gl);let hb=(if sb[33]{(h9).cosh()}else{l});let hf=(if sb[33]{(gm).cosh()}else{l});let hl=((gm+(if sb[33]{(hf).ln()}else{l}))-(if sb[33]{(h9+(if sb[33]{(hb).ln()}else{l}))}else{l}));let hu=(b9-gl);let hw=(if sb[33]{(hu).cosh()}else{hb});let hA=(if sb[33]{(gE).cosh()}else{hf});let hG=((gE+(if sb[33]{(hA).ln()}else{l}))-(if sb[33]{(hu+(if sb[33]{(hw).ln()}else{l}))}else{l}));let Bh=(gm).sinh();let Bn=(if sb[33]{(sf[29]*Bh)}else{l});let BY=(if sb[33]{(sf[78]+(b3*(sf[82]+((h8*(sf[29]+(if sb[33]{(Bn/hf)}else{l})))/sf[29]))))}else{l});let hP=BY;let hQ=(if sb[33]{hP}else{(if sb[30]{(sf[78]+(gu*gU))}else{sf[79]})});let C8=(gE).sinh();let CU=(if sb[33]{(sf[80]+(b4*(sf[82]+((gB*(sf[32]+(if sb[33]{((if sb[33]{(sf[32]*C8)}else{l})/hA)}else{l})))/sf[32]))))}else{l});let hR=CU;let hS=(if sb[33]{hR}else{(if sb[30]{(sf[80]+(b4*((gB*gG)+sf[82])))}else{sf[81]})});let iW=0.0;let iY=0.0;let j2=0.0;let j5=0.0;let jB=0.0;let jH=0.0;let jU=0.0;let k0=0.0;let ka=(-(if sb[49]{((if sb[49]{((b3*((H*5.5226012e-23)*sf[104]))*sf[106])}else{l})*3.141592653589793)}else{l}));let kc=0.0;let kg=0.0;let An=(Q-(gn*gn));let At=(Q-(gt*gt));let Au=(sf[71]*At);let Av=(sf[132]*At);let Ay=(Q-(gz*gz));let Az=(sf[133]*Ay);let AA=(sf[73]*Ay);let AE=(Q-(gF*gF));let B8=(h9).sinh();let Bb=(if sb[33]{(sf[69]*B8)}else{l});let Bc=(if sb[33]{(sf[130]*B8)}else{l});let Bl=(if sb[33]{(sf[69]*Bh)}else{l});let Bm=(if sb[33]{(sf[131]*Bh)}else{l});let BZ=(hu).sinh();let D6=1.0;let Ek=(sf[92]*D6);let EF=(sf[96]*D6);

        stamper.stamp_current_reactive_node1_local(
            Some(12),
            None,
            12,
            multiplicity * (sf[108]),
        );
        stamper.stamp_current_reactive_branch1_local(
            Some(12),
            Some(13),
            0,
            multiplicity * (sf[109]),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(3),
            &[3, 5, 7, 8],
            &[(if (sf[77]!=0.0){((if sb[33]{((b4*(sf[136]+(((hG*Az)+(gB*((sf[135]+(if sb[33]{((if sb[33]{(sf[135]*C8)}else{Bl})/hA)}else{l}))-(if sb[33]{(sf[130]+(if sb[33]{((if sb[33]{(sf[130]*BZ)}else{Bb})/hw)}else{l}))}else{l}))))/sf[32])))+sf[138])}else{l})*D6)}else{l}), (if (sf[77]!=0.0){((if sb[33]{(b4*(((hG*AA)+(gB*((sf[69]+(if sb[33]{((if sb[33]{(sf[69]*C8)}else{Bm})/hA)}else{l}))-(if sb[33]{(sf[69]+(if sb[33]{((if sb[33]{(sf[69]*BZ)}else{Bc})/hw)}else{l}))}else{l}))))/sf[32]))}else{l})*D6)}else{l}), (if (sf[77]!=0.0){(CU*D6)}else{l}), (if (sf[77]!=0.0){((if sb[33]{(b4*((gB*(if sb[33]{((if sb[33]{l}else{Bn})/hA)}else{l}))/sf[32]))}else{l})*D6)}else{l})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3_local(
            Some(8),
            Some(5),
            3,
            multiplicity * ((if (sf[77]!=0.0){((if sb[33]{(b3*(((hl*Au)+(h8*((sf[69]+(if sb[33]{(Bl/hf)}else{l}))-(if sb[33]{(sf[69]+(if sb[33]{(Bb/hb)}else{l}))}else{l}))))/sf[29]))}else{l})*D6)}else{l})),
            5,
            multiplicity * ((if (sf[77]!=0.0){((if sb[33]{((b3*((((hl*Av)+(h8*((sf[131]+(if sb[33]{(Bm/hf)}else{l}))-(if sb[33]{(sf[130]+(if sb[33]{(Bc/hb)}else{l}))}else{l}))))/sf[29])+sf[136]))+sf[137])}else{l})*D6)}else{l})),
            8,
            multiplicity * ((if (sf[77]!=0.0){(BY*D6)}else{l})),
        );
        stamper.stamp_current_reactive_node3_local(
            Some(7),
            Some(3),
            3,
            multiplicity * ((if sb[51]{(D6*((-hS)+(j*(if sb[33]{l}else{(if sb[30]{(b4*((gG*Az)+(gB*(sf[135]*AE))))}else{l})}))))}else{l})),
            5,
            multiplicity * ((if sb[51]{(D6*(j*(if sb[33]{l}else{(if sb[30]{(b4*((gG*AA)+(gB*(sf[69]*AE))))}else{l})})))}else{l})),
            7,
            multiplicity * ((if sb[51]{(D6*(hS+(j*(if sb[33]{l}else{(if sb[30]{(b4*(gB*(sf[32]*AE)))}else{l})}))))}else{l})),
        );
        stamper.stamp_current_reactive_node3_local(
            Some(8),
            Some(5),
            3,
            multiplicity * ((if sb[51]{(D6*(c*(if sb[33]{l}else{(if sb[30]{((gU*Au)+(gu*(b3*(sf[69]*An))))}else{l})})))}else{l})),
            5,
            multiplicity * ((if sb[51]{(D6*((-hQ)+(c*(if sb[33]{l}else{(if sb[30]{((gU*Av)+(gu*(b3*(sf[131]*An))))}else{l})}))))}else{l})),
            8,
            multiplicity * ((if sb[51]{(D6*(hQ+(c*(if sb[33]{l}else{(if sb[30]{(gu*(b3*(sf[29]*An)))}else{l})}))))}else{l})),
        );
        stamper.stamp_current_reactive_node2_local(
            Some(4),
            Some(3),
            1,
            multiplicity * (sf[110]),
            3,
            multiplicity * (sf[139]),
        );
        stamper.stamp_current_reactive_node2_local(
            Some(3),
            Some(5),
            3,
            multiplicity * (sf[111]),
            5,
            multiplicity * (sf[140]),
        );
        stamper.stamp_current_reactive_node2_local(
            Some(3),
            Some(10),
            3,
            multiplicity * (b6),
            10,
            multiplicity * ((-b6)),
        );
        stamper.stamp_current_reactive_node2_local(
            Some(9),
            Some(8),
            8,
            multiplicity * (sf[141]),
            9,
            multiplicity * (sf[112]),
        );
        stamper.stamp_current_reactive_branch1_local(
            Some(1),
            Some(4),
            6,
            multiplicity * ((if (sf[91]!=0.0){Ek}else{l})),
        );
        stamper.stamp_current_reactive_branch1_local(
            Some(1),
            Some(4),
            8,
            multiplicity * ((if sb[53]{Ek}else{l})),
        );
        stamper.stamp_current_reactive_branch1_local(
            Some(6),
            Some(2),
            13,
            multiplicity * (sf[113]),
        );
        stamper.stamp_current_reactive_branch1_local(
            Some(3),
            Some(0),
            15,
            multiplicity * ((if (sf[95]!=0.0){EF}else{l})),
        );
        stamper.stamp_current_reactive_branch1_local(
            Some(3),
            Some(0),
            17,
            multiplicity * ((if sb[55]{EF}else{l})),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(4),
            Some(3),
            14,
            multiplicity * ((if sb[49]{(ka*D6)}else{l})),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(11),
            None,
            11,
            multiplicity * ((if (sf[107]!=0.0){(sf[114]*D6)}else{l})),
        );
    }
}
