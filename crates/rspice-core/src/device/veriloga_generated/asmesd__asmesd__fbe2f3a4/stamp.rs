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

struct CommonStampValues {
    b: f64, m_: f64, n_: f64, s: f64, t: f64, u: f64,
    v: f64, H: f64, J: f64, M: f64, ae: f64, al: f64,
    am: f64, aq: f64, ar: f64, as_: f64, ck: f64, cn: f64,
    co: f64, cp: f64, cq: f64, cM: f64, d0: f64, d3: f64,
    dw: f64, dz: f64, dE: f64, dG: f64, e6: f64, ed: f64,
    el: f64, ez: f64, eZ: f64, fe: f64, ff: f64, fg: f64,
    fn_: f64, ft: f64, fT: f64, fX: f64, g4: f64, h0: f64,
    hx: f64, hy: f64, hz: f64, ie: f64, is: f64, iz: f64,
    mj: f64, mp: f64, my: f64, n6: f64, n8: f64, na: f64,
    nc: f64, ne: f64, ng: f64, ni: f64, nj: f64, ny: f64,
    nA: f64, nP: f64, rm: f64, rq: f64, rA: f64, rB: f64,
    rC: f64, sn: f64, so: f64, sp: f64, sR: f64, sS: f64,
    sY: f64, tl: f64, tm: f64, tn: f64, tJ: f64, tK: f64,
    tL: f64, um: f64, un: f64, uo: f64, up: f64, uq: f64,
    ur: f64, uw: f64, ux: f64, uy: f64, uz: f64, uU: f64,
    uV: f64, uW: f64, uX: f64, vo: f64, vp: f64, vq: f64,
    vr: f64, vY: f64, vZ: f64, w0: f64, w1: f64, yo: f64,
    yp: f64, yq: f64, yr: f64, yu: f64, yx: f64, yA: f64,
    yB: f64, yC: f64, yF: f64, yI: f64, Ge: f64, Gf: f64,
    Gg: f64, Gk: f64, Gl: f64, Gm: f64, Gs: f64, Gt: f64,
    Gu: f64, Gv: f64, Gw: f64, GC: f64, GD: f64, GE: f64,
    GF: f64, GG: f64, GL: f64, GM: f64, GN: f64, GO: f64,
    GS: f64, GT: f64, GU: f64, GZ: f64, H0: f64, H1: f64,
    H2: f64, H3: f64, H4: f64, H5: f64, H6: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let n=self.nodes;
        let nodes=n;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let b=ctx.node_voltage(n[3]);let e=((ctx.temperature()+b)+sf[0]);let g=1300.0;let h=173.14999999999998;let i=(e>h);let j=(if i{e}else{h});let k=(g<j);let l=(if k{g}else{j});let m_=1.0;let n_=0.0;let s=ctx.node_voltage(n[5]);let t=ctx.node_voltage(n[4]);let u=(s-t);let v=(sf[4]*u);let G=8.6170869e-5;let H=(l*G);let I=(l/sf[8]);let J=(I).ln();let M=((J*sf[9])).exp();let ab=(I-m_);let ac=(sf[25]*ab);let ae=((J*sf[24])+(ac/H));let ai=(ae).exp();let aj=(sf[27]*ai);let al=((J*sf[26])).exp();let am=(sf[28]*al);let aq=((ae/sf[30])).exp();let ar=(sf[29]*aq);let as_=(ar/M);let aD=(sf[33]*(m_+(ab*sf[34])));let aI=(sf[35]*(m_+(ab*sf[36])));let aN=(sf[37]*(m_+(ab*sf[38])));let aS=(sf[39]*(m_+(ab*sf[40])));let aW=300.15;let aY=(l/aW);let b0=0.000702;let b1=(l*b0);let b2=(l*b1);let b4=(l+1108.0);let b7=(-(1.16-(b2/b4)));let b8=1.3806226e-23;let ba=(b8*(l+l));let bf=(-(H+H));let bg=1.5;let bj=1.6021918e-19;let bl=((bg*(aY).ln())+(((b7/ba)+1.3454442398941469e20)*bj));let bm=(bf*bl);let bp=((sf[45]-bm)/sf[44]);let bq=(sf[45]-bp);let bt=0.0004;let by=(m_+(sf[46]*(sf[48]-(bq/bp))));let bz=(sf[41]/by);let bB=(bm+(aY*bp));let bC=(bB-bp);let bF=(bt*(l-aW));let bI=(m_+(sf[46]*(bF-(bC/bp))));let bJ=(bz*bI);let bM=((sf[49]-bm)/sf[44]);let bN=(sf[49]-bM);let bS=(m_+(sf[50]*(sf[48]-(bN/bM))));let bT=(sf[42]/bS);let bV=(bm+(aY*bM));let bW=(bV-bM);let c0=(m_+(sf[50]*(bF-(bW/bM))));let c1=(bT*c0);let c4=((sf[51]-bm)/sf[44]);let c5=(sf[51]-c4);let ca=(m_+(sf[52]*(sf[48]-(c5/c4))));let cb=(sf[43]/ca);let cd=(bm+(aY*c4));let ce=(cd-c4);let ci=(m_+(sf[52]*(bF-(ce/c4))));let cj=(cb*ci);let ck=ctx.node_voltage(n[2]);let cm=(sf[4]*(ck-t));let cn=ctx.node_voltage(n[6]);let co=(s-cn);let cp=(sf[4]*co);let cq=ctx.node_voltage(n[1]);let cs=(sf[4]*(cq-t));let cy=(if (aj>n_){m_}else{n_});let cA=(H*sf[53]);let cC=(if (cy!=0.0){(cp/cA)}else{n_});let cD=(-cp);let cE=(cD-aI);let cG=(H*sf[54]);let cI=(if (cy!=0.0){(cE/cG)}else{n_});let cJ=(-aI);let cL=(if (cy!=0.0){(cJ/cG)}else{n_});let cM=80.0;let cO=(if (cC>cM){m_}else{n_});let cP=((cy!=0.0)&&(cO!=0.0));let cT=(if cP{cM}else{cC});let cV=((cy!=0.0)&&(!(cO!=0.0)));let cW=(if cV{m_}else{(if cP{(m_+(cC-cM))}else{n_})});let cX=(cT).exp();let cZ=(if (cy!=0.0){(cW*cX)}else{cW});let d0=37.0;let d1=(cI>=d0);let d2=(!d1);let d3=-37.0;let d4=(cI<=d3);let d6=(d2&&(!d4));let d7=(cI).exp();let d8=(m_+d7);let da=(d2&&d4);let de=(cL>=d0);let df=(!de);let dg=(cL<=d3);let di=(df&&(!dg));let dj=(cL).exp();let dk=(m_+dj);let dm=(df&&dg);let dr=(if (cy!=0.0){((if d6{(d8).ln()}else{(if da{d7}else{(if d1{cI}else{n_})})})-(if di{(dk).ln()}else{(if dm{dj}else{(if de{cL}else{n_})})}))}else{n_});let ds=(cZ-m_);let du=(aD*dr);let dw=(cp).abs();let dx=f64::powf(dw,aN);let dz=(m_+(sf[55]*dx));let dD=(!(cy!=0.0));let dE=(if dD{n_}else{(if (cy!=0.0){((aj*ds)-(du/dz))}else{n_})});let dG=(if (am>n_){m_}else{n_});let dI=(sf[56]-cp);let dJ=0.001;let dK=(dI>dJ);let dM=(if (dG!=0.0){(if dK{dI}else{dJ})}else{n_});let dO=(cD*sf[56]);let dQ=(H*sf[57]);let dR=(dM*dQ);let dT=(if (dG!=0.0){(dO/dR)}else{cT});let dV=(if (dT>cM){m_}else{n_});let dW=((dG!=0.0)&&(dV!=0.0));let e0=(if dW{cM}else{dT});let e2=((dG!=0.0)&&(!(dV!=0.0)));let e3=(if e2{m_}else{(if dW{(m_+(dT-cM))}else{cZ})});let e4=(e0).exp();let e6=(if (dG!=0.0){(e3*e4)}else{e3});let ed=(if (as_>n_){m_}else{n_});let ee=(H*sf[30]);let eg=(if (ed!=0.0){(cp/ee)}else{e0});let ei=(H*sf[58]);let ek=(if (ed!=0.0){(cE/ei)}else{cI});let el=(cJ/ei);let em=(if (ed!=0.0){el}else{cL});let eo=(if (eg>cM){m_}else{n_});let ep=((ed!=0.0)&&(eo!=0.0));let et=(if ep{cM}else{eg});let ev=((ed!=0.0)&&(!(eo!=0.0)));let ew=(if ev{m_}else{(if ep{(m_+(eg-cM))}else{e6})});let ex=(et).exp();let ez=(if (ed!=0.0){(ew*ex)}else{ew});let eA=(ek>=d0);let eB=(!eA);let eC=(ek<=d3);let eE=(eB&&(!eC));let eF=(ek).exp();let eG=(m_+eF);let eI=(eB&&eC);let eM=(em>=d0);let eN=(!eM);let eO=(em<=d3);let eQ=(eN&&(!eO));let eR=(em).exp();let eS=(m_+eR);let eU=(eN&&eO);let eZ=(if (ed!=0.0){((if eE{(eG).ln()}else{(if eI{eF}else{(if eA{ek}else{n_})})})-(if eQ{(eS).ln()}else{(if eU{eR}else{(if eM{em}else{n_})})}))}else{dr});let f9=(H*sf[59]);
        let fb=(if (cy!=0.0){(v/f9)}else{et});let fd=((-v)-aI);let fe=(fd/ei);let ff=(if (cy!=0.0){fe}else{ek});let fg=(if (cy!=0.0){el}else{em});let fi=(if (fb>cM){m_}else{n_});let fj=((cy!=0.0)&&(fi!=0.0));let fn_=(if fj{cM}else{fb});let fp=((cy!=0.0)&&(!(fi!=0.0)));let fq=(if fp{m_}else{(if fj{(m_+(fb-cM))}else{ez})});let fr=(fn_).exp();let ft=(if (cy!=0.0){(fq*fr)}else{fq});let fu=(ff>=d0);let fv=(!fu);let fw=(ff<=d3);let fy=(fv&&(!fw));let fz=(ff).exp();let fA=(m_+fz);let fC=(fv&&fw);let fG=(fg>=d0);let fH=(!fG);let fI=(fg<=d3);let fK=(fH&&(!fI));let fL=(fg).exp();let fM=(m_+fL);let fO=(fH&&fI);let fT=(if (cy!=0.0){((if fy{(fA).ln()}else{(if fC{fz}else{(if fu{ff}else{n_})})})-(if fK{(fM).ln()}else{(if fO{fL}else{(if fG{fg}else{n_})})}))}else{eZ});let fU=(ft-m_);let fW=(aS*fT);let fX=(v).abs();let fY=f64::powf(fX,aN);let g0=(m_+(sf[55]*fY));let g4=(if dD{n_}else{(if (cy!=0.0){((aj*fU)-(fW/g0))}else{n_})});let h0=ctx.node_voltage(n[9]);let hu=(m_+f64::powf(((m_+(((dE*(sf[20]*(m_+(v*sf[60]))))+(sf[23]*g4))*4.0))).abs(),sf[61]));let hx=((((m_-(sf[17]*cp))-(v*sf[14]))*2.0)/hu);let hy=(g4*hx);let hz=(dE*hx);let ie=(cq-ck);let is=(sf[79]*(m_+((f64::powf((m_+f64::powf(((ie/sf[76])).abs(),sf[77])),sf[78])-m_)*sf[80])));let iz=ctx.node_voltage(n[8]);let iV=(if (cm<=n_){m_}else{n_});let iW=(cd*cj);let iZ=(m_-(cm/cd));let j2=((sf[91]*(iZ).ln())).exp();let j3=(m_-j2);let j7=(!(iV!=0.0));let j8=(cj*cm);let jb=(cm*sf[92]);let jd=(m_+(jb/cd));let jj=(cp+((-bB)*sf[93]));let jl=(if (jj>n_){m_}else{n_});let jr=(if (jl!=0.0){sf[98]}else{n_});let ju=(m_-(sf[95]*(sf[95]*jr)));let jA=(jj*sf[100]);let jC=(sf[95]+(jA/bB));let jG=(!(jl!=0.0));let jI=(m_-(cp/bB));let jL=((sf[99]*(jI).ln())).exp();let jM=(m_-jL);let jP=(if jG{((bB*jM)/sf[99])}else{(if (jl!=0.0){((bB*ju)/sf[99])}else{n_})});let jQ=(if jG{n_}else{(if (jl!=0.0){(jr*(jj*jC))}else{n_})});let jR=(jP+jQ);let jU=(sf[93]*(-bV));let jV=(cs+jU);let jX=(if (jV>n_){m_}else{n_});let k1=(if (jX!=0.0){sf[103]}else{jr});let k4=(m_-(sf[95]*(sf[95]*k1)));let ka=(jV*sf[105]);let kc=(sf[95]+(ka/bV));let kg=(!(jX!=0.0));let ki=(m_-(cs/bV));let kl=((sf[104]*(ki).ln())).exp();let km=(m_-kl);let kp=(if kg{((bV*km)/sf[104])}else{(if (jX!=0.0){((bV*k4)/sf[104])}else{jP})});let kq=(if kg{n_}else{(if (jX!=0.0){(k1*(jV*kc))}else{jQ})});let kr=(kp+kq);let kw=(v+jU);let ky=(if (kw>n_){m_}else{n_});let kz=(if (ky!=0.0){sf[103]}else{k1});let kC=(m_-(sf[95]*(sf[95]*kz)));let kG=(sf[105]*kw);let kI=(sf[95]+(kG/bV));let kM=(!(ky!=0.0));let kO=(m_-(v/bV));let kR=((sf[104]*(kO).ln())).exp();let kS=(m_-kR);let kX=((if kM{((bV*kS)/sf[104])}else{(if (ky!=0.0){((bV*kC)/sf[104])}else{kp})})+(if kM{n_}else{(if (ky!=0.0){(kz*(kw*kI))}else{kq})}));let le=(if sb[9]{n_}else{(if (sf[109]!=0.0){(hz*sf[113])}else{n_})});let mj=(b*sf[134]);let mp=ctx.node_voltage(n[7]);let my=(mp*sf[135]);let n6=(sf[3]*(sf[4]*(bJ*jR)));let n8=(sf[3]*(sf[4]*(dE*is)));let na=(sf[3]*(sf[4]*((c1*kr)*sf[107])));let nc=(sf[3]*(sf[4]*(sf[106]*(c1*kX))));let ne=(sf[3]*(sf[4]*(hy*sf[81])));let ng=(sf[3]*(sf[4]*(if j7{(j8*jd)}else{(if (iV!=0.0){((iW*j3)/sf[91])}else{n_})})));let ni=(sf[3]*(-le));let nj=(sf[3]*le);let nl=(if k{n_}else{(if i{m_}else{n_})});let ny=(G*nl);let nz=(nl/sf[8]);let nA=(nz/I);let nP=((sf[24]*nA)+(((H*(sf[25]*nz))-(ac*ny))/(H*H)));let nS=(sf[27]*(ai*nP));let of=(sf[37]*(sf[38]*nz));let oi=(nl/aW);let oH=((bl*(-(ny+ny)))+(bf*((bg*(oi/aY))+(bj*(((ba*(((b4*((b1*nl)+(l*(b0*nl))))-(b2*nl))/(b4*b4)))-(b7*(b8*(nl+nl))))/(ba*ba))))));let oJ=((-oH)/sf[44]);let oK=(-oJ);let oO=(bp*bp);let oX=(aY*oJ);let oZ=(oH+((bp*oi)+oX));let p5=(bt*nl);let pe=(bM*bM);let po=(oH+(oX+(bM*oi)));let py=((c0*((-(sf[42]*(sf[50]*(-(((bM*oK)-(bN*oJ))/pe)))))/(bS*bS)))+(bT*(sf[50]*(p5-(((bM*(po-oJ))-(bW*oJ))/pe)))));let pC=(c4*c4);let pM=(oH+(oX+(c4*oi)));let pW=((ci*((-(sf[43]*(sf[52]*(-(((c4*oK)-(c5*oJ))/pC)))))/(ca*ca)))+(cb*(sf[52]*(p5-(((c4*(pM-oJ))-(ce*oJ))/pC)))));let q4=(if (cy!=0.0){((-(cp*(sf[53]*ny)))/(cA*cA))}else{n_});let q5=(if (cy!=0.0){(sf[4]/cA)}else{n_});let q6=(if (cy!=0.0){(sf[136]/cA)}else{n_});let q7=(-(sf[35]*(sf[36]*nz)));let q8=(sf[54]*ny);
        let q9=(cG*q7);let qc=(cG*cG);let qg=(if (cy!=0.0){((q9-(cE*q8))/qc)}else{n_});let qh=(if (cy!=0.0){(sf[136]/cG)}else{n_});let qi=(if (cy!=0.0){(sf[4]/cG)}else{n_});let qm=(if (cy!=0.0){((q9-(cJ*q8))/qc)}else{n_});let qq=(if cP{n_}else{q4});let qr=(if cP{n_}else{q5});let qs=(if cP{n_}else{q6});let qt=(if cV{n_}else{(if cP{q4}else{n_})});let qu=(if cV{n_}else{(if cP{q5}else{n_})});let qv=(if cV{n_}else{(if cP{q6}else{n_})});let qI=(if (cy!=0.0){((cX*qt)+(cW*(cX*qq)))}else{qt});let qJ=(if (cy!=0.0){((cX*qu)+(cW*(cX*qr)))}else{qu});let qK=(if (cy!=0.0){((cX*qv)+(cW*(cX*qs)))}else{qv});let qL=(d7*qg);let qM=(d7*qh);let qN=(d7*qi);let r0=(dj*qm);let r6=(if (cy!=0.0){((if d6{(qL/d8)}else{(if da{qL}else{(if d1{qg}else{n_})})})-(if di{(r0/dk)}else{(if dm{r0}else{(if de{qm}else{n_})})}))}else{n_});let r7=(if (cy!=0.0){(if d6{(qM/d8)}else{(if da{qM}else{(if d1{qh}else{n_})})})}else{n_});let r8=(if (cy!=0.0){(if d6{(qN/d8)}else{(if da{qN}else{(if d1{qi}else{n_})})})}else{n_});let rm=(sf[55]*(of*(dx*(dw).ln())));let rq=(dz*dz);let rA=(if dD{n_}else{(if (cy!=0.0){(((ds*nS)+(aj*qI))-(((dz*((dr*(sf[33]*(sf[34]*nz)))+(aD*r6)))-(du*rm))/rq))}else{n_})});let rB=(if dD{n_}else{(if (cy!=0.0){((aj*qJ)-((aD*r7)/dz))}else{n_})});let rC=(if dD{n_}else{(if (cy!=0.0){((aj*qK)-((aD*r8)/dz))}else{n_})});let rP=(dR*dR);let rZ=(if (dG!=0.0){((-(dO*(dM*(sf[57]*ny))))/rP)}else{qq});let s0=(if (dG!=0.0){(((dR*sf[138])-(dO*(dQ*(if (dG!=0.0){(if dK{sf[136]}else{n_})}else{n_}))))/rP)}else{qr});let s1=(if (dG!=0.0){(((dR*sf[139])-(dO*(dQ*(if (dG!=0.0){(if dK{sf[4]}else{n_})}else{n_}))))/rP)}else{qs});let s5=(if dW{n_}else{rZ});let s6=(if dW{n_}else{s0});let s7=(if dW{n_}else{s1});let s8=(if e2{n_}else{(if dW{rZ}else{qI})});let s9=(if e2{n_}else{(if dW{s0}else{qJ})});let sa=(if e2{n_}else{(if dW{s1}else{qK})});let sn=(if (dG!=0.0){((e4*s8)+(e3*(e4*s5)))}else{s8});let so=(if (dG!=0.0){((e4*s9)+(e3*(e4*s6)))}else{s9});let sp=(if (dG!=0.0){((e4*sa)+(e3*(e4*s7)))}else{sa});let sI=(if (ed!=0.0){((-(cp*(sf[30]*ny)))/(ee*ee))}else{s5});let sJ=(if (ed!=0.0){(sf[4]/ee)}else{s6});let sK=(if (ed!=0.0){(sf[136]/ee)}else{s7});let sL=(sf[58]*ny);let sM=(ei*q7);let sP=(ei*ei);let sR=(sf[136]/ei);let sS=(sf[4]/ei);let sT=(if (ed!=0.0){((sM-(cE*sL))/sP)}else{qg});let sU=(if (ed!=0.0){sR}else{qh});let sV=(if (ed!=0.0){sS}else{qi});let sY=((sM-(cJ*sL))/sP);let sZ=(if (ed!=0.0){sY}else{qm});let t3=(if ep{n_}else{sI});let t4=(if ep{n_}else{sJ});let t5=(if ep{n_}else{sK});let t6=(if ev{n_}else{(if ep{sI}else{sn})});let t7=(if ev{n_}else{(if ep{sJ}else{so})});let t8=(if ev{n_}else{(if ep{sK}else{sp})});let tl=(if (ed!=0.0){((ex*t6)+(ew*(ex*t3)))}else{t6});let tm=(if (ed!=0.0){((ex*t7)+(ew*(ex*t4)))}else{t7});let tn=(if (ed!=0.0){((ex*t8)+(ew*(ex*t5)))}else{t8});let to=(eF*sT);let tp=(eF*sU);let tq=(eF*sV);let tD=(eR*sZ);let tJ=(if (ed!=0.0){((if eE{(to/eG)}else{(if eI{to}else{(if eA{sT}else{n_})})})-(if eQ{(tD/eS)}else{(if eU{tD}else{(if eM{sZ}else{n_})})}))}else{r6});let tK=(if (ed!=0.0){(if eE{(tp/eG)}else{(if eI{tp}else{(if eA{sU}else{n_})})})}else{r7});let tL=(if (ed!=0.0){(if eE{(tq/eG)}else{(if eI{tq}else{(if eA{sV}else{n_})})})}else{r8});let ug=(if (cy!=0.0){((-(v*(sf[59]*ny)))/(f9*f9))}else{t3});let uh=(if (cy!=0.0){(sf[136]/f9)}else{n_});let ui=(if (cy!=0.0){(sf[4]/f9)}else{t4});let uj=(if (cy!=0.0){n_}else{t5});let um=((sM-(fd*sL))/sP);let un=(if (cy!=0.0){um}else{sT});let uo=(if (cy!=0.0){sS}else{n_});let up=(if (cy!=0.0){sR}else{sU});let uq=(if (cy!=0.0){n_}else{sV});let ur=(if (cy!=0.0){sY}else{sZ});let uw=(if fj{n_}else{ug});let ux=(if fj{n_}else{uh});let uy=(if fj{n_}else{ui});let uz=(if fj{n_}else{uj});let uA=(if fp{n_}else{(if fj{ug}else{tl})});let uB=(if fp{n_}else{(if fj{uh}else{n_})});let uC=(if fp{n_}else{(if fj{ui}else{tm})});let uD=(if fp{n_}else{(if fj{uj}else{tn})});let uU=(if (cy!=0.0){((fr*uA)+(fq*(fr*uw)))}else{uA});let uV=(if (cy!=0.0){((fr*uB)+(fq*(fr*ux)))}else{uB});let uW=(if (cy!=0.0){((fr*uC)+(fq*(fr*uy)))}else{uC});let uX=(if (cy!=0.0){((fr*uD)+(fq*(fr*uz)))}else{uD});let uY=(fz*un);let uZ=(fz*uo);let v0=(fz*up);let v1=(fz*uq);
        let vi=(fL*ur);let vo=(if (cy!=0.0){((if fy{(uY/fA)}else{(if fC{uY}else{(if fu{un}else{n_})})})-(if fK{(vi/fM)}else{(if fO{vi}else{(if fG{ur}else{n_})})}))}else{tJ});let vp=(if (cy!=0.0){(if fy{(uZ/fA)}else{(if fC{uZ}else{(if fu{uo}else{n_})})})}else{n_});let vq=(if (cy!=0.0){(if fy{(v0/fA)}else{(if fC{v0}else{(if fu{up}else{n_})})})}else{tK});let vr=(if (cy!=0.0){(if fy{(v1/fA)}else{(if fC{v1}else{(if fu{uq}else{n_})})})}else{tL});let vY=(if dD{n_}else{(if (cy!=0.0){(((fU*nS)+(aj*uU))-(((g0*((fT*(sf[39]*(sf[40]*nz)))+(aS*vo)))-(fW*(sf[55]*(of*(fY*(fX).ln())))))/(g0*g0)))}else{n_})});let vZ=(if dD{n_}else{(if (cy!=0.0){((aj*uV)-((aS*vp)/g0))}else{n_})});let w0=(if dD{n_}else{(if (cy!=0.0){((aj*uW)-((aS*vq)/g0))}else{n_})});let w1=(if dD{n_}else{(if (cy!=0.0){((aj*uX)-((aS*vr)/g0))}else{n_})});let yo=(sf[148]/hu);let yp=(sf[149]/hu);let yq=(sf[150]/hu);let yr=(hx*vY);let yu=((hx*vZ)+(g4*yo));let yx=((hx*w0)+(g4*yp));let yA=((hx*w1)+(g4*yq));let yB=(hx*rA);let yC=(dE*yo);let yF=((hx*rB)+(dE*yp));let yI=((hx*rC)+(dE*yq));let zw=(cd*cd);let Am=(sf[93]*(-oZ));let Aw=(bB*bB);let Bh=(if jG{(((jM*oZ)+(bB*(-(jL*(sf[99]*((-((-(cp*oZ))/Aw))/jI))))))/sf[99])}else{(if (jl!=0.0){((ju*oZ)/sf[99])}else{n_})});let Bi=(if jG{((bB*(-(jL*(sf[99]*((-(sf[4]/bB))/jI)))))/sf[99])}else{n_});let Bj=(if jG{((bB*(-(jL*(sf[99]*((-(sf[136]/bB))/jI)))))/sf[99])}else{n_});let Bk=(if jG{n_}else{(if (jl!=0.0){(jr*((jC*Am)+(jj*(((bB*(sf[100]*Am))-(jA*oZ))/Aw))))}else{n_})});let Bl=(if jG{n_}else{(if (jl!=0.0){(jr*((sf[4]*jC)+(jj*(sf[153]/bB))))}else{n_})});let Bm=(if jG{n_}else{(if (jl!=0.0){(jr*((jC*sf[136])+(jj*(sf[154]/bB))))}else{n_})});let Bw=(sf[93]*(-po));let BF=(sf[155]/bV);let BG=(bV*(sf[105]*Bw));let BJ=(bV*bV);let BL=(sf[156]/bV);let C8=(-(sf[4]/bV));let Ca=(-(sf[136]/bV));let Cv=(if kg{((bV*(-(kl*(sf[104]*(C8/ki)))))/sf[104])}else{n_});let Cw=(if kg{(((km*po)+(bV*(-(kl*(sf[104]*((-((-(cs*po))/BJ))/ki))))))/sf[104])}else{(if (jX!=0.0){((k4*po)/sf[104])}else{Bh})});let Cx=(if kg{((bV*(-(kl*(sf[104]*(Ca/ki)))))/sf[104])}else{n_});let Cy=(if kg{n_}else{(if (jX!=0.0){n_}else{Bi})});let Cz=(if kg{n_}else{(if (jX!=0.0){n_}else{Bj})});let CA=(if kg{n_}else{(if (jX!=0.0){(k1*((sf[4]*kc)+(jV*BF)))}else{n_})});let CB=(if kg{n_}else{(if (jX!=0.0){(k1*((kc*Bw)+(jV*((BG-(ka*po))/BJ))))}else{Bk})});let CC=(if kg{n_}else{(if (jX!=0.0){(k1*((kc*sf[136])+(jV*BL)))}else{n_})});let CD=(if kg{n_}else{(if (jX!=0.0){n_}else{Bl})});let CE=(if kg{n_}else{(if (jX!=0.0){n_}else{Bm})});let Ek=(if sb[9]{n_}else{(if (sf[109]!=0.0){(sf[113]*yB)}else{n_})});let El=(if sb[9]{n_}else{(if (sf[109]!=0.0){(sf[113]*yC)}else{n_})});let Em=(if sb[9]{n_}else{(if (sf[109]!=0.0){(sf[113]*yF)}else{n_})});let En=(if sb[9]{n_}else{(if (sf[109]!=0.0){(sf[113]*yI)}else{n_})});let Ge=(sf[3]*(sf[4]*((jR*((bI*((-(sf[41]*(sf[46]*(-(((bp*oK)-(bq*oJ))/oO)))))/(by*by)))+(bz*(sf[46]*(p5-(((bp*(oZ-oJ))-(bC*oJ))/oO))))))+(bJ*(Bh+Bk)))));let Gf=(sf[3]*(sf[4]*(bJ*(Bi+Bl))));let Gg=(sf[3]*(sf[4]*(bJ*(Bj+Bm))));let Gk=(sf[3]*(sf[4]*(is*rA)));let Gl=(sf[3]*(sf[4]*(is*rB)));let Gm=(sf[3]*(sf[4]*(is*rC)));let Gs=(sf[3]*(sf[4]*(sf[107]*(c1*(Cv+CA)))));let Gt=(sf[3]*(sf[4]*(sf[107]*((kr*py)+(c1*(Cw+CB))))));let Gu=(sf[3]*(sf[4]*(sf[107]*(c1*(Cx+CC)))));let Gv=(sf[3]*(sf[4]*(sf[107]*(c1*(Cy+CD)))));let Gw=(sf[3]*(sf[4]*(sf[107]*(c1*(Cz+CE)))));let GC=(sf[3]*(sf[4]*(sf[106]*(c1*((if kM{n_}else{(if (ky!=0.0){n_}else{Cv})})+(if kM{n_}else{(if (ky!=0.0){n_}else{CA})}))))));let GD=(sf[3]*(sf[4]*(sf[106]*((kX*py)+(c1*((if kM{(((kS*po)+(bV*(-(kR*(sf[104]*((-((-(v*po))/BJ))/kO))))))/sf[104])}else{(if (ky!=0.0){((kC*po)/sf[104])}else{Cw})})+(if kM{n_}else{(if (ky!=0.0){(kz*((kI*Bw)+(kw*((BG-(kG*po))/BJ))))}else{CB})})))))));let GE=(sf[3]*(sf[4]*(sf[106]*(c1*((if kM{((bV*(-(kR*(sf[104]*(Ca/kO)))))/sf[104])}else{(if (ky!=0.0){n_}else{Cx})})+(if kM{n_}else{(if (ky!=0.0){(kz*((kI*sf[136])+(kw*BL)))}else{CC})}))))));let GF=(sf[3]*(sf[4]*(sf[106]*(c1*((if kM{((bV*(-(kR*(sf[104]*(C8/kO)))))/sf[104])}else{(if (ky!=0.0){n_}else{Cy})})+(if kM{n_}else{(if (ky!=0.0){(kz*((sf[4]*kI)+(kw*BF)))}else{CD})}))))));
        let GG=(sf[3]*(sf[4]*(sf[106]*(c1*((if kM{n_}else{(if (ky!=0.0){n_}else{Cz})})+(if kM{n_}else{(if (ky!=0.0){n_}else{CE})}))))));let GL=(sf[3]*(sf[4]*(sf[81]*yr)));let GM=(sf[3]*(sf[4]*(sf[81]*yu)));let GN=(sf[3]*(sf[4]*(sf[81]*yx)));let GO=(sf[3]*(sf[4]*(sf[81]*yA)));let GS=(sf[3]*(sf[4]*(if j7{((jd*(sf[4]*cj))+(j8*(sf[151]/cd)))}else{(if (iV!=0.0){((iW*(-(j2*(sf[91]*((-(sf[4]/cd))/iZ)))))/sf[91])}else{n_})})));let GT=(sf[3]*(sf[4]*(if j7{((jd*(cm*pW))+(j8*((-(jb*pM))/zw)))}else{(if (iV!=0.0){(((j3*((cj*pM)+(cd*pW)))+(iW*(-(j2*(sf[91]*((-((-(cm*pM))/zw))/iZ))))))/sf[91])}else{n_})})));let GU=(sf[3]*(sf[4]*(if j7{((jd*(cj*sf[136]))+(j8*(sf[152]/cd)))}else{(if (iV!=0.0){((iW*(-(j2*(sf[91]*((-(sf[136]/cd))/iZ)))))/sf[91])}else{n_})})));let GZ=(sf[3]*(-Ek));let H0=(sf[3]*(-El));let H1=(sf[3]*(-Em));let H2=(sf[3]*(-En));let H3=(sf[3]*Ek);let H4=(sf[3]*El);let H5=(sf[3]*Em);let H6=(sf[3]*En);

        CommonStampValues {
            b, m_, n_, s, t, u, v, H,
            J, M, ae, al, am, aq, ar, as_,
            ck, cn, co, cp, cq, cM, d0, d3,
            dw, dz, dE, dG, e6, ed, el, ez,
            eZ, fe, ff, fg, fn_, ft, fT, fX,
            g4, h0, hx, hy, hz, ie, is, iz,
            mj, mp, my, n6, n8, na, nc, ne,
            ng, ni, nj, ny, nA, nP, rm, rq,
            rA, rB, rC, sn, so, sp, sR, sS,
            sY, tl, tm, tn, tJ, tK, tL, um,
            un, uo, up, uq, ur, uw, ux, uy,
            uz, uU, uV, uW, uX, vo, vp, vq,
            vr, vY, vZ, w0, w1, yo, yp, yq,
            yr, yu, yx, yA, yB, yC, yF, yI,
            Ge, Gf, Gg, Gk, Gl, Gm, Gs, Gt,
            Gu, Gv, Gw, GC, GD, GE, GF, GG,
            GL, GM, GN, GO, GS, GT, GU, GZ,
            H0, H1, H2, H3, H4, H5, H6,
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        let CommonStampValues {
            b, m_, n_, s, t, u, v, H,
            J, M, ae, al, am, aq, ar, as_,
            ck, cn, co, cp, cq, cM, d0, d3,
            dw, dz, dE, dG, e6, ed, el, ez,
            eZ, fe, ff, fg, fn_, ft, fT, fX,
            g4, h0, hx, hy, hz, ie, is, iz,
            mj, mp, my, n6, n8, na, nc, ne,
            ng, ni, nj, ny, nA, nP, rm, rq,
            rA, rB, rC, sn, so, sp, sR, sS,
            sY, tl, tm, tn, tJ, tK, tL, um,
            un, uo, up, uq, ur, uw, ux, uy,
            uz, uU, uV, uW, uX, vo, vp, vq,
            vr, vY, vZ, w0, w1, yo, yp, yq,
            yr, yu, yx, yA, yB, yC, yF, yI,
            Ge, Gf, Gg, Gk, Gl, Gm, Gs, Gt,
            Gu, Gv, Gw, GC, GD, GE, GF, GG,
            GL, GM, GN, GO, GS, GT, GU, GZ,
            H0, H1, H2, H3, H4, H5, H6,
        }=self.eval_common_stamp_values(ctx);
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
        let x=(v<n_);let z=(-(if x{v}else{n_}));let D=(m_+(sf[5]*f64::powf(z,sf[6])));let O=(M*sf[10]);let P=(D*O);let R=(M*sf[11]);let aw=((ae/sf[32])).exp();let ax=(sf[31]*aw);let ay=(ax/M);let ct=(cq-s);let cv=(ck-cn);let dN=-1.0;let e7=(e6-m_);let ea=(!(dG!=0.0));let f0=(ez-m_);let f2=(n_*eZ);let f6=(!(ed!=0.0));let g6=(if (ay>n_){m_}else{n_});let g7=(H*sf[32]);let g9=(if (g6!=0.0){(v/g7)}else{fn_});let ga=(if (g6!=0.0){fe}else{ff});let gb=(if (g6!=0.0){el}else{fg});let gd=(if (g9>cM){m_}else{n_});let ge=((g6!=0.0)&&(gd!=0.0));let gk=((g6!=0.0)&&(!(gd!=0.0)));let gl=(if gk{m_}else{(if ge{(m_+(g9-cM))}else{ft})});let gm=((if ge{cM}else{g9})).exp();let gp=(ga>=d0);let gq=(!gp);let gr=(ga<=d3);let gt=(gq&&(!gr));let gu=(ga).exp();let gv=(m_+gu);let gx=(gq&&gr);let gB=(gb>=d0);let gC=(!gB);let gD=(gb<=d3);let gF=(gC&&(!gD));let gG=(gb).exp();let gH=(m_+gG);let gJ=(gC&&gD);let gP=((if (g6!=0.0){(gl*gm)}else{gl})-m_);let gU=(m_+(sf[55]*f64::powf(fX,sf[37])));let gY=(!(g6!=0.0));let h3=1e-9;let h7=(((if (h0<cp){h0}else{cp})/(if (dw>h3){dw}else{h3}))).abs();let h8=(dE-(if ea{n_}else{(if (dG!=0.0){(am*e7)}else{n_})}));let ha=((if f6{n_}else{(if (ed!=0.0){((as_*f0)-(f2/dz))}else{n_})})+(h8/P));let hc=((if gY{n_}else{(if (g6!=0.0){((ay*gP)-((n_*(if (g6!=0.0){((if gt{(gv).ln()}else{(if gx{gu}else{(if gp{ga}else{n_})})})-(if gF{(gH).ln()}else{(if gJ{gG}else{(if gB{gb}else{n_})})}))}else{fT}))/gU))}else{n_})})+(g4/R));let hE=(dE*sf[63]);let hW=((J*sf[69])).exp();let hZ=f64::powf((m_+f64::powf((((sf[4]*ct)/sf[64])).abs(),sf[65])),sf[70]);let i0=((sf[68]*hW)*hZ);let i4=((J*sf[72])).exp();let i5=(sf[71]*i4);let i9=((J*sf[74])).exp();let ic=f64::powf((m_+f64::powf((((sf[4]*cv)/sf[66])).abs(),sf[67])),sf[75]);let id=((sf[73]*i9)*ic);let iF=(m_+f64::powf(((iz).abs()/sf[84]),sf[85]));let iH=(if (sf[83]!=0.0){(i0/iF)}else{i0});let lH=((if (sf[87]!=0.0){(iH+sf[88])}else{iH})/sf[3]);let lM=((if (sf[87]!=0.0){(id+sf[90])}else{id})/sf[3]);let lR=((if (sf[87]!=0.0){(i5+sf[89])}else{i5})/sf[3]);let lU=1e-6;let lX=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, h0);let m4=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, iz);let ma=ctx.node_voltage(n[0]);let me=((-((ha*ie)).abs())-((hc*(cq-ma))).abs());let mk=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, mj);let mt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, mj);let mz=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, my);let mJ=(lH>sf[129]);let mK=(if mJ{lH}else{sf[129]});let mN=(lM>sf[129]);let mO=(if mN{lM}else{sf[129]});let mR=(ma-t);let mS=(lR>sf[129]);let mT=(if mS{lR}else{sf[129]});let nt=(sf[6]*f64::powf(z,sf[137]));let nC=(M*(sf[9]*nA));let nE=(D*(sf[10]*nC));let nF=(O*(sf[5]*((-(if x{sf[136]}else{n_}))*nt)));let nG=(O*(sf[5]*((-(if x{sf[4]}else{n_}))*nt)));let o1=(M*M);let w9=(if (g6!=0.0){((-(v*(sf[32]*ny)))/(g7*g7))}else{uw});let wa=(if (g6!=0.0){(sf[136]/g7)}else{ux});let wb=(if (g6!=0.0){(sf[4]/g7)}else{uy});let wc=(if (g6!=0.0){n_}else{uz});let wd=(if (g6!=0.0){um}else{un});let we=(if (g6!=0.0){sS}else{uo});let wf=(if (g6!=0.0){sR}else{up});let wg=(if (g6!=0.0){n_}else{uq});let wh=(if (g6!=0.0){sY}else{ur});
        let wq=(if gk{n_}else{(if ge{w9}else{uU})});let wr=(if gk{n_}else{(if ge{wa}else{uV})});let ws=(if gk{n_}else{(if ge{wb}else{uW})});let wt=(if gk{n_}else{(if ge{wc}else{uX})});let wO=(gu*wd);let wP=(gu*we);let wQ=(gu*wf);let wR=(gu*wg);let x8=(gG*wh);let xO=(P*P);let z9=(hZ*(sf[68]*(hW*(sf[69]*nA))));let Er=ddt_scale;let EW=(sf[134]*Er);let F6=-0.0;

        stamper.stamp_current_node3_local(
            Some(9),
            None,
            multiplicity * ((-(cp-h0))),
            5,
            multiplicity * (sf[136]),
            6,
            multiplicity * (sf[4]),
            9,
            multiplicity * (m_),
        );
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * ((h0*lU)),
            9,
            multiplicity * (lU),
        );
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * ((sf[133]*lX)),
            9,
            multiplicity * ((sf[133]*Er)),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            None,
            multiplicity * ((if (sf[83]!=0.0){(is*(-(dE/P)))}else{n_})),
            [3, 4, 5, 6],
            [(if (sf[83]!=0.0){(is*(-(((P*rA)-(dE*nE))/xO)))}else{n_}), (if (sf[83]!=0.0){(is*(-((-(dE*nF))/xO)))}else{n_}), (if (sf[83]!=0.0){(is*(-(((P*rB)-(dE*nG))/xO)))}else{n_}), (if (sf[83]!=0.0){(is*(-(rC/P)))}else{n_})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * ((if (sf[83]!=0.0){iz}else{n_})),
            8,
            multiplicity * (sf[157]),
        );
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * ((if (sf[83]!=0.0){(is*m4)}else{n_})),
            8,
            multiplicity * ((if (sf[83]!=0.0){(is*Er)}else{n_})),
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            n_,
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * ((if (sf[116]!=0.0){me}else{n_})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if (sf[116]!=0.0){(b/sf[115])}else{n_})),
            3,
            multiplicity * (sf[159]),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if (sf[116]!=0.0){mk}else{n_})),
            3,
            multiplicity * ((if (sf[116]!=0.0){EW}else{n_})),
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            n_,
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * ((if sb[28]{me}else{n_})),
        );
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * ((if sb[28]{((b-mp)/sf[115])}else{n_})),
            3,
            multiplicity * (sf[161]),
            7,
            multiplicity * (sf[162]),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if sb[28]{mt}else{n_})),
            3,
            multiplicity * ((if sb[28]{EW}else{n_})),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * ((if sb[28]{(mp/sf[117])}else{n_})),
            7,
            multiplicity * (sf[164]),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * ((if sb[28]{mz}else{n_})),
            7,
            multiplicity * ((if sb[28]{(sf[135]*Er)}else{n_})),
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * ((if sb[31]{me}else{n_})),
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            n_,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            n_,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            n_,
        );
        stamper.stamp_current_node1_local(
            Some(5),
            Some(6),
            multiplicity * ((n_*co)),
            6,
            multiplicity * (F6),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            Some(4),
            multiplicity * ((n_*u)),
            4,
            multiplicity * (F6),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            Some(6),
            multiplicity * ((n_*(t-cn))),
            6,
            multiplicity * (F6),
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * ((if (sf[130]!=0.0){(ct/mK)}else{n_})),
            1,
            multiplicity * ((if (sf[130]!=0.0){(m_/mK)}else{n_})),
            3,
            multiplicity * ((if (sf[130]!=0.0){((-(ct*(if mJ{((if (sf[83]!=0.0){(z9/iF)}else{z9})/sf[3])}else{n_})))/(mK*mK))}else{n_})),
            5,
            multiplicity * ((if (sf[130]!=0.0){(dN/mK)}else{n_})),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (n_),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(5),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            n_,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(6),
            multiplicity * ((if (sf[131]!=0.0){(cv/mO)}else{n_})),
            2,
            multiplicity * ((if (sf[131]!=0.0){(m_/mO)}else{n_})),
            3,
            multiplicity * ((if (sf[131]!=0.0){((-(cv*(if mN{((ic*(sf[73]*(i9*(sf[74]*nA))))/sf[3])}else{n_})))/(mO*mO))}else{n_})),
            6,
            multiplicity * ((if (sf[131]!=0.0){(dN/mO)}else{n_})),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(6),
            multiplicity * (n_),
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(6),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            n_,
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(4),
            multiplicity * ((if (sf[132]!=0.0){(mR/mT)}else{n_})),
            0,
            multiplicity * ((if (sf[132]!=0.0){(m_/mT)}else{n_})),
            3,
            multiplicity * ((if (sf[132]!=0.0){((-(mR*(if mS{((sf[71]*(i4*(sf[72]*nA)))/sf[3])}else{n_})))/(mT*mT))}else{n_})),
            4,
            multiplicity * ((if (sf[132]!=0.0){(dN/mT)}else{n_})),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(4),
            multiplicity * (n_),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(4),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            n_,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * ((sf[3]*(sf[4]*ha))),
            [3, 4, 5, 6],
            [(sf[3]*(sf[4]*((if f6{n_}else{(if (ed!=0.0){(((f0*(((M*(sf[29]*(aq*(nP/sf[30]))))-(ar*nC))/o1))+(as_*tl))-(((dz*(n_*tJ))-(f2*rm))/rq))}else{n_})})+(((P*(rA-(if ea{n_}else{(if (dG!=0.0){((e7*(sf[28]*(al*(sf[26]*nA))))+(am*sn))}else{n_})})))-(h8*nE))/xO)))), (sf[3]*(sf[4]*((-(h8*nF))/xO))), (sf[3]*(sf[4]*((if f6{n_}else{(if (ed!=0.0){((as_*tm)-((n_*tK)/dz))}else{n_})})+(((P*(rB-(if ea{n_}else{(if (dG!=0.0){(am*so)}else{n_})})))-(h8*nG))/xO)))), (sf[3]*(sf[4]*((if f6{n_}else{(if (ed!=0.0){((as_*tn)-((n_*tL)/dz))}else{n_})})+((rC-(if ea{n_}else{(if (dG!=0.0){(am*sp)}else{n_})}))/P))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * ((sf[3]*(sf[4]*hc))),
            [3, 4, 5, 6],
            [(sf[3]*(sf[4]*((if gY{n_}else{(if (g6!=0.0){(((gP*(((M*(sf[31]*(aw*(nP/sf[32]))))-(ax*nC))/o1))+(ay*(if (g6!=0.0){((gm*wq)+(gl*(gm*(if ge{n_}else{w9}))))}else{wq})))-((n_*(if (g6!=0.0){((if gt{(wO/gv)}else{(if gx{wO}else{(if gp{wd}else{n_})})})-(if gF{(x8/gH)}else{(if gJ{x8}else{(if gB{wh}else{n_})})}))}else{vo}))/gU))}else{n_})})+(((R*vY)-(g4*(sf[11]*nC)))/(R*R))))), (sf[3]*(sf[4]*((if gY{n_}else{(if (g6!=0.0){((ay*(if (g6!=0.0){((gm*wr)+(gl*(gm*(if ge{n_}else{wa}))))}else{wr}))-((n_*(if (g6!=0.0){(if gt{(wP/gv)}else{(if gx{wP}else{(if gp{we}else{n_})})})}else{vp}))/gU))}else{n_})})+(vZ/R)))), (sf[3]*(sf[4]*((if gY{n_}else{(if (g6!=0.0){((ay*(if (g6!=0.0){((gm*ws)+(gl*(gm*(if ge{n_}else{wb}))))}else{ws}))-((n_*(if (g6!=0.0){(if gt{(wQ/gv)}else{(if gx{wQ}else{(if gp{wf}else{n_})})})}else{vq}))/gU))}else{n_})})+(w0/R)))), (sf[3]*(sf[4]*((if gY{n_}else{(if (g6!=0.0){((ay*(if (g6!=0.0){((gm*wt)+(gl*(gm*(if ge{n_}else{wc}))))}else{wt}))-((n_*(if (g6!=0.0){(if gt{(wR/gv)}else{(if gx{wR}else{(if gp{wg}else{n_})})})}else{vr}))/gU))}else{n_})})+(w1/R))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * ((sf[4]*(sf[3]*(-hy)))),
            [3, 4, 5, 6],
            [(sf[4]*(sf[3]*(-yr))), (sf[4]*(sf[3]*(-yu))), (sf[4]*(sf[3]*(-yx))), (sf[4]*(sf[3]*(-yA)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * ((sf[3]*(sf[4]*(((h7*hz)*sf[62])+(hx*hE))))),
            [3, 4, 5, 6],
            [(sf[3]*(sf[4]*((sf[62]*(h7*yB))+(hx*(sf[63]*rA))))), (sf[3]*(sf[4]*((sf[62]*(h7*yC))+(hE*yo)))), (sf[3]*(sf[4]*((sf[62]*(h7*yF))+((hE*yp)+(hx*(sf[63]*rB)))))), (sf[3]*(sf[4]*((sf[62]*(h7*yI))+((hE*yq)+(hx*(sf[63]*rC))))))],
            [],
            [],
            multiplicity,
        );
        let n6_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, n6);
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (n6_ddt),
            3,
            multiplicity * (((Ge) * ddt_scale)),
            5,
            multiplicity * (((Gf) * ddt_scale)),
            6,
            multiplicity * (((Gg) * ddt_scale)),
        );
        let n8_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, n8);
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (n8_ddt),
            3,
            multiplicity * (((Gk) * ddt_scale)),
            5,
            multiplicity * (((Gl) * ddt_scale)),
            6,
            multiplicity * (((Gm) * ddt_scale)),
        );
        let na_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, na);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(1),
            Some(4),
            multiplicity * (na_ddt),
            [1, 3, 4, 5, 6],
            [((Gs) * ddt_scale), ((Gt) * ddt_scale), ((Gu) * ddt_scale), ((Gv) * ddt_scale), ((Gw) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let nc_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, nc);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(4),
            multiplicity * (nc_ddt),
            [1, 3, 4, 5, 6],
            [((GC) * ddt_scale), ((GD) * ddt_scale), ((GE) * ddt_scale), ((GF) * ddt_scale), ((GG) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let ne_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, ne);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (ne_ddt),
            [3, 4, 5, 6],
            [((GL) * ddt_scale), ((GM) * ddt_scale), ((GN) * ddt_scale), ((GO) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let ng_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, ng);
        stamper.stamp_current_node3_local(
            Some(2),
            Some(4),
            multiplicity * (ng_ddt),
            2,
            multiplicity * (((GS) * ddt_scale)),
            3,
            multiplicity * (((GT) * ddt_scale)),
            4,
            multiplicity * (((GU) * ddt_scale)),
        );
        let ni_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, ni);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (ni_ddt),
            [3, 4, 5, 6],
            [((GZ) * ddt_scale), ((H0) * ddt_scale), ((H1) * ddt_scale), ((H2) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let nj_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, nj);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (nj_ddt),
            [3, 4, 5, 6],
            [((H3) * ddt_scale), ((H4) * ddt_scale), ((H5) * ddt_scale), ((H6) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (n_),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (n_),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(6),
            multiplicity * (n_),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        let br=self.branches;
        let branches=br;
        let CommonStampValues {
            b, m_, n_, s, t, u, v, H,
            J, M, ae, al, am, aq, ar, as_,
            ck, cn, co, cp, cq, cM, d0, d3,
            dw, dz, dE, dG, e6, ed, el, ez,
            eZ, fe, ff, fg, fn_, ft, fT, fX,
            g4, h0, hx, hy, hz, ie, is, iz,
            mj, mp, my, n6, n8, na, nc, ne,
            ng, ni, nj, ny, nA, nP, rm, rq,
            rA, rB, rC, sn, so, sp, sR, sS,
            sY, tl, tm, tn, tJ, tK, tL, um,
            un, uo, up, uq, ur, uw, ux, uy,
            uz, uU, uV, uW, uX, vo, vp, vq,
            vr, vY, vZ, w0, w1, yo, yp, yq,
            yr, yu, yx, yA, yB, yC, yF, yI,
            Ge, Gf, Gg, Gk, Gl, Gm, Gs, Gt,
            Gu, Gv, Gw, GC, GD, GE, GF, GG,
            GL, GM, GN, GO, GS, GT, GU, GZ,
            H0, H1, H2, H3, H4, H5, H6,
        }=self.eval_common_stamp_values(ctx);
        let p=&(*self.params);
        let m=self.multiplicity;
        let multiplicity=m;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let lX=0.0;let m4=0.0;let mk=0.0;let mt=0.0;let mz=0.0;let Er=1.0;let EW=(sf[134]*Er);

        stamper.stamp_current_reactive_node1_local(
            Some(9),
            None,
            9,
            multiplicity * ((sf[133]*Er)),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(8),
            None,
            8,
            multiplicity * ((if (sf[83]!=0.0){(is*Er)}else{n_})),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(3),
            None,
            3,
            multiplicity * ((if (sf[116]!=0.0){EW}else{n_})),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(3),
            None,
            3,
            multiplicity * ((if sb[28]{EW}else{n_})),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(7),
            None,
            7,
            multiplicity * ((if sb[28]{(sf[135]*Er)}else{n_})),
        );
        stamper.stamp_current_reactive_node3_local(
            Some(5),
            Some(6),
            3,
            multiplicity * (Ge),
            5,
            multiplicity * (Gf),
            6,
            multiplicity * (Gg),
        );
        stamper.stamp_current_reactive_node3_local(
            Some(5),
            Some(6),
            3,
            multiplicity * (Gk),
            5,
            multiplicity * (Gl),
            6,
            multiplicity * (Gm),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(4),
            &[1, 3, 4, 5, 6],
            &[Gs, Gt, Gu, Gv, Gw],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[1, 3, 4, 5, 6],
            &[GC, GD, GE, GF, GG],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5, 6],
            &[GL, GM, GN, GO],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3_local(
            Some(2),
            Some(4),
            2,
            multiplicity * (GS),
            3,
            multiplicity * (GT),
            4,
            multiplicity * (GU),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[3, 4, 5, 6],
            &[GZ, H0, H1, H2],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5, 6],
            &[H3, H4, H5, H6],
            &[],
            &[],
            multiplicity,
        );
    }
}
