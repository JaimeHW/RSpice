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

struct CommonStampValues {
    b: f64, m_: f64, n_: f64, s: f64, t: f64, u: f64,
    v: f64, H: f64, J: f64, M: f64, ae: f64, al: f64,
    am: f64, aq: f64, ar: f64, as_: f64, ck: f64, cn: f64,
    co: f64, cp: f64, cq: f64, cM: f64, d0: f64, d3: f64,
    dw: f64, dz: f64, dE: f64, dG: f64, e6: f64, ed: f64,
    el: f64, ez: f64, eZ: f64, fe: f64, ff: f64, fg: f64,
    fn_: f64, ft: f64, fT: f64, fX: f64, g4: f64, h0: f64,
    hx: f64, hy: f64, hz: f64, ie: f64, is: f64, iz: f64,
    lY: f64, m6: f64, ml: f64, mp: f64, mu: f64, mA: f64,
    n7: f64, n9: f64, nb: f64, nd: f64, nf: f64, nh: f64,
    nj: f64, nk: f64, nz: f64, nB: f64, nQ: f64, rn: f64,
    rr: f64, rB: f64, rC: f64, rD: f64, so: f64, sp: f64,
    sq: f64, sS: f64, sT: f64, sZ: f64, tm: f64, tn: f64,
    to: f64, tK: f64, tL: f64, tM: f64, un: f64, uo: f64,
    up: f64, uq: f64, ur: f64, us: f64, ux: f64, uy: f64,
    uz: f64, uA: f64, uV: f64, uW: f64, uX: f64, uY: f64,
    vp: f64, vq: f64, vr: f64, vs: f64, vZ: f64, w0: f64,
    w1: f64, w2: f64, yp: f64, yq: f64, yr: f64, ys: f64,
    yv: f64, yy: f64, yB: f64, yC: f64, yD: f64, yG: f64,
    yJ: f64, Et: f64, EU: f64, EY: f64, F2: f64, F6: f64,
    Gf: f64, Gg: f64, Gh: f64, Gl: f64, Gm: f64, Gn: f64,
    Gt: f64, Gu: f64, Gv: f64, Gw: f64, Gx: f64, GD: f64,
    GE: f64, GF: f64, GG: f64, GH: f64, GM: f64, GN: f64,
    GO: f64, GP: f64, GT: f64, GU: f64, GV: f64, H0: f64,
    H1: f64, H2: f64, H3: f64, H4: f64, H5: f64, H6: f64,
    H7: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values<const REACTIVE: bool>(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let n=self.nodes;
        let nodes=n;
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
        let b=ctx.node_voltage(n[3]);let e=((ctx.temperature()+b)+sf[0]);let g=1300.0;let h=173.14999999999998;let i=(e>h);let j=(if i{e}else{h});let k=(g<j);let l=(if k{g}else{j});let m_=1.0;let n_=0.0;let s=ctx.node_voltage(n[5]);let t=ctx.node_voltage(n[4]);let u=(s-t);let v=(sf[4]*u);let G=8.6170869e-5;let H=(l*G);let I=(l/sf[8]);let J=(I).ln();let M=((J*sf[9])).exp();let ab=(I-m_);let ac=(sf[25]*ab);let ae=((J*sf[24])+(ac/H));let ai=(ae).exp();let aj=(sf[27]*ai);let al=((J*sf[26])).exp();let am=(sf[28]*al);let aq=((ae/sf[30])).exp();let ar=(sf[29]*aq);let as_=(ar/M);let aD=(sf[33]*(m_+(ab*sf[34])));let aI=(sf[35]*(m_+(ab*sf[36])));let aN=(sf[37]*(m_+(ab*sf[38])));let aS=(sf[39]*(m_+(ab*sf[40])));let aW=300.15;let aY=(l/aW);let b0=0.000702;let b1=(l*b0);let b2=(l*b1);let b4=(l+1108.0);let b7=(-(1.16-(b2/b4)));let b8=1.3806226e-23;let ba=(b8*(l+l));let bf=(-(H+H));let bg=1.5;let bj=1.6021918e-19;let bl=((bg*(aY).ln())+(((b7/ba)+1.3454442398941469e20)*bj));let bm=(bf*bl);let bp=((sf[45]-bm)/sf[44]);let bq=(sf[45]-bp);let bt=0.0004;let by=(m_+(sf[46]*(sf[48]-(bq/bp))));let bz=(sf[41]/by);let bB=(bm+(aY*bp));let bC=(bB-bp);let bF=(bt*(l-aW));let bI=(m_+(sf[46]*(bF-(bC/bp))));let bJ=(bz*bI);let bM=((sf[49]-bm)/sf[44]);let bN=(sf[49]-bM);let bS=(m_+(sf[50]*(sf[48]-(bN/bM))));let bT=(sf[42]/bS);let bV=(bm+(aY*bM));let bW=(bV-bM);let c0=(m_+(sf[50]*(bF-(bW/bM))));let c1=(bT*c0);let c4=((sf[51]-bm)/sf[44]);let c5=(sf[51]-c4);let ca=(m_+(sf[52]*(sf[48]-(c5/c4))));let cb=(sf[43]/ca);let cd=(bm+(aY*c4));let ce=(cd-c4);let ci=(m_+(sf[52]*(bF-(ce/c4))));let cj=(cb*ci);let ck=ctx.node_voltage(n[2]);let cm=(sf[4]*(ck-t));let cn=ctx.node_voltage(n[6]);let co=(s-cn);let cp=(sf[4]*co);let cq=ctx.node_voltage(n[1]);let cs=(sf[4]*(cq-t));let cy=(if (aj>n_){m_}else{n_});let cA=(H*sf[53]);let cC=(if ((cy)!=0.0){(cp/cA)}else{n_});let cD=(-cp);let cE=(cD-aI);let cG=(H*sf[54]);let cI=(if ((cy)!=0.0){(cE/cG)}else{n_});let cJ=(-aI);let cL=(if ((cy)!=0.0){(cJ/cG)}else{n_});let cM=80.0;let cO=(if (cC>cM){m_}else{n_});let cP=(((cy)!=0.0)&&((cO)!=0.0));let cT=(if cP{cM}else{cC});let cV=(((cy)!=0.0)&&(!((cO)!=0.0)));let cW=(if cV{m_}else{(if cP{(m_+(cC-cM))}else{n_})});let cX=(cT).exp();let cZ=(if ((cy)!=0.0){(cW*cX)}else{cW});let d0=37.0;let d1=(cI>=d0);let d2=(!d1);let d3=-37.0;let d4=(cI<=d3);let d6=(d2&&(!d4));let d7=(cI).exp();let d8=(m_+d7);let da=(d2&&d4);let de=(cL>=d0);let df=(!de);let dg=(cL<=d3);let di=(df&&(!dg));let dj=(cL).exp();let dk=(m_+dj);let dm=(df&&dg);let dr=(if ((cy)!=0.0){((if d6{(d8).ln()}else{(if da{d7}else{(if d1{cI}else{n_})})})-(if di{(dk).ln()}else{(if dm{dj}else{(if de{cL}else{n_})})}))}else{n_});let ds=(cZ-m_);let du=(aD*dr);let dw=(cp).abs();let dx=f64::powf(dw,aN);let dz=(m_+(sf[55]*dx));let dD=(!((cy)!=0.0));let dE=(if dD{n_}else{(if ((cy)!=0.0){((aj*ds)-(du/dz))}else{n_})});let dG=(if (am>n_){m_}else{n_});let dI=(sf[56]-cp);let dJ=0.001;let dK=(dI>dJ);let dM=(if ((dG)!=0.0){(if dK{dI}else{dJ})}else{n_});let dO=(cD*sf[56]);let dQ=(H*sf[57]);let dR=(dM*dQ);let dT=(if ((dG)!=0.0){(dO/dR)}else{cT});let dV=(if (dT>cM){m_}else{n_});let dW=(((dG)!=0.0)&&((dV)!=0.0));let e0=(if dW{cM}else{dT});let e2=(((dG)!=0.0)&&(!((dV)!=0.0)));let e3=(if e2{m_}else{(if dW{(m_+(dT-cM))}else{cZ})});let e4=(e0).exp();let e6=(if ((dG)!=0.0){(e3*e4)}else{e3});let ed=(if (as_>n_){m_}else{n_});let ee=(H*sf[30]);let eg=(if ((ed)!=0.0){(cp/ee)}else{e0});let ei=(H*sf[58]);let ek=(if ((ed)!=0.0){(cE/ei)}else{cI});let el=(cJ/ei);let em=(if ((ed)!=0.0){el}else{cL});let eo=(if (eg>cM){m_}else{n_});let ep=(((ed)!=0.0)&&((eo)!=0.0));let et=(if ep{cM}else{eg});let ev=(((ed)!=0.0)&&(!((eo)!=0.0)));let ew=(if ev{m_}else{(if ep{(m_+(eg-cM))}else{e6})});let ex=(et).exp();let ez=(if ((ed)!=0.0){(ew*ex)}else{ew});let eA=(ek>=d0);let eB=(!eA);let eC=(ek<=d3);let eE=(eB&&(!eC));let eF=(ek).exp();let eG=(m_+eF);let eI=(eB&&eC);let eM=(em>=d0);let eN=(!eM);let eO=(em<=d3);let eQ=(eN&&(!eO));let eR=(em).exp();let eS=(m_+eR);let eU=(eN&&eO);
        let eZ=(if ((ed)!=0.0){((if eE{(eG).ln()}else{(if eI{eF}else{(if eA{ek}else{n_})})})-(if eQ{(eS).ln()}else{(if eU{eR}else{(if eM{em}else{n_})})}))}else{dr});let f9=(H*sf[59]);let fb=(if ((cy)!=0.0){(v/f9)}else{et});let fd=((-v)-aI);let fe=(fd/ei);let ff=(if ((cy)!=0.0){fe}else{ek});let fg=(if ((cy)!=0.0){el}else{em});let fi=(if (fb>cM){m_}else{n_});let fj=(((cy)!=0.0)&&((fi)!=0.0));let fn_=(if fj{cM}else{fb});let fp=(((cy)!=0.0)&&(!((fi)!=0.0)));let fq=(if fp{m_}else{(if fj{(m_+(fb-cM))}else{ez})});let fr=(fn_).exp();let ft=(if ((cy)!=0.0){(fq*fr)}else{fq});let fu=(ff>=d0);let fv=(!fu);let fw=(ff<=d3);let fy=(fv&&(!fw));let fz=(ff).exp();let fA=(m_+fz);let fC=(fv&&fw);let fG=(fg>=d0);let fH=(!fG);let fI=(fg<=d3);let fK=(fH&&(!fI));let fL=(fg).exp();let fM=(m_+fL);let fO=(fH&&fI);let fT=(if ((cy)!=0.0){((if fy{(fA).ln()}else{(if fC{fz}else{(if fu{ff}else{n_})})})-(if fK{(fM).ln()}else{(if fO{fL}else{(if fG{fg}else{n_})})}))}else{eZ});let fU=(ft-m_);let fW=(aS*fT);let fX=(v).abs();let fY=f64::powf(fX,aN);let g0=(m_+(sf[55]*fY));let g4=(if dD{n_}else{(if ((cy)!=0.0){((aj*fU)-(fW/g0))}else{n_})});let h0=ctx.node_voltage(n[9]);let hu=(m_+f64::powf(((m_+(((dE*(sf[20]*(m_+(v*sf[60]))))+(sf[23]*g4))*4.0))).abs(),sf[61]));let hx=((((m_-(sf[17]*cp))-(v*sf[14]))*2.0)/hu);let hy=(g4*hx);let hz=(dE*hx);let ie=(cq-ck);let is=(sf[79]*(m_+((f64::powf((m_+f64::powf(((ie/sf[76])).abs(),sf[77])),sf[78])-m_)*sf[80])));let iz=ctx.node_voltage(n[8]);let iV=(if (cm<=n_){m_}else{n_});let iW=(cd*cj);let iZ=(m_-(cm/cd));let j2=((sf[91]*(iZ).ln())).exp();let j3=(m_-j2);let j7=(!((iV)!=0.0));let j8=(cj*cm);let jb=(cm*sf[92]);let jd=(m_+(jb/cd));let jj=(cp+((-bB)*sf[93]));let jl=(if (jj>n_){m_}else{n_});let jr=(if ((jl)!=0.0){sf[98]}else{n_});let ju=(m_-(sf[95]*(sf[95]*jr)));let jA=(jj*sf[100]);let jC=(sf[95]+(jA/bB));let jG=(!((jl)!=0.0));let jI=(m_-(cp/bB));let jL=((sf[99]*(jI).ln())).exp();let jM=(m_-jL);let jP=(if jG{((bB*jM)/sf[99])}else{(if ((jl)!=0.0){((bB*ju)/sf[99])}else{n_})});let jQ=(if jG{n_}else{(if ((jl)!=0.0){(jr*(jj*jC))}else{n_})});let jR=(jP+jQ);let jU=(sf[93]*(-bV));let jV=(cs+jU);let jX=(if (jV>n_){m_}else{n_});let k1=(if ((jX)!=0.0){sf[103]}else{jr});let k4=(m_-(sf[95]*(sf[95]*k1)));let ka=(jV*sf[105]);let kc=(sf[95]+(ka/bV));let kg=(!((jX)!=0.0));let ki=(m_-(cs/bV));let kl=((sf[104]*(ki).ln())).exp();let km=(m_-kl);let kp=(if kg{((bV*km)/sf[104])}else{(if ((jX)!=0.0){((bV*k4)/sf[104])}else{jP})});let kq=(if kg{n_}else{(if ((jX)!=0.0){(k1*(jV*kc))}else{jQ})});let kr=(kp+kq);let kw=(v+jU);let ky=(if (kw>n_){m_}else{n_});let kz=(if ((ky)!=0.0){sf[103]}else{k1});let kC=(m_-(sf[95]*(sf[95]*kz)));let kG=(sf[105]*kw);let kI=(sf[95]+(kG/bV));let kM=(!((ky)!=0.0));let kO=(m_-(v/bV));let kR=((sf[104]*(kO).ln())).exp();let kS=(m_-kR);let kX=((if kM{((bV*kS)/sf[104])}else{(if ((ky)!=0.0){((bV*kC)/sf[104])}else{kp})})+(if kM{n_}else{(if ((ky)!=0.0){(kz*(kw*kI))}else{kq})}));let le=(if sb[9]{n_}else{(if ((sf[109])!=0.0){(hz*sf[113])}else{n_})});let lX=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, h0) };let lY=(sf[133]*lX);let m4=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, iz) };let m6=(if ((sf[83])!=0.0){(is*m4)}else{n_});let mj=(b*sf[134]);let mk=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, mj) };let ml=(if ((sf[116])!=0.0){mk}else{n_});let mp=ctx.node_voltage(n[7]);
        let mt=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, mj) };let mu=(if sb[28]{mt}else{n_});let mz=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (mp*sf[135])) };let mA=(if sb[28]{mz}else{n_});let n7=(sf[3]*(sf[4]*(bJ*jR)));let n9=(sf[3]*(sf[4]*(dE*is)));let nb=(sf[3]*(sf[4]*((c1*kr)*sf[107])));let nd=(sf[3]*(sf[4]*(sf[106]*(c1*kX))));let nf=(sf[3]*(sf[4]*(hy*sf[81])));let nh=(sf[3]*(sf[4]*(if j7{(j8*jd)}else{(if ((iV)!=0.0){((iW*j3)/sf[91])}else{n_})})));let nj=(sf[3]*(-le));let nk=(sf[3]*le);let nm=(if k{n_}else{(if i{m_}else{n_})});let nz=(G*nm);let nA=(nm/sf[8]);let nB=(nA/I);let nQ=((sf[24]*nB)+(((H*(sf[25]*nA))-(ac*nz))/(H*H)));let nT=(sf[27]*(ai*nQ));let og=(sf[37]*(sf[38]*nA));let oj=(nm/aW);let oI=((bl*(-(nz+nz)))+(bf*((bg*(oj/aY))+(bj*(((ba*(((b4*((b1*nm)+(l*(b0*nm))))-(b2*nm))/(b4*b4)))-(b7*(b8*(nm+nm))))/(ba*ba))))));let oK=((-oI)/sf[44]);let oL=(-oK);let oP=(bp*bp);let oY=(aY*oK);let p0=(oI+((bp*oj)+oY));let p6=(bt*nm);let pf=(bM*bM);let pp=(oI+(oY+(bM*oj)));let pz=((c0*((-(sf[42]*(sf[50]*(-(((bM*oL)-(bN*oK))/pf)))))/(bS*bS)))+(bT*(sf[50]*(p6-(((bM*(pp-oK))-(bW*oK))/pf)))));let pD=(c4*c4);let pN=(oI+(oY+(c4*oj)));let pX=((ci*((-(sf[43]*(sf[52]*(-(((c4*oL)-(c5*oK))/pD)))))/(ca*ca)))+(cb*(sf[52]*(p6-(((c4*(pN-oK))-(ce*oK))/pD)))));let q5=(if ((cy)!=0.0){((-(cp*(sf[53]*nz)))/(cA*cA))}else{n_});let q6=(if ((cy)!=0.0){(sf[4]/cA)}else{n_});let q7=(if ((cy)!=0.0){(sf[136]/cA)}else{n_});let q8=(-(sf[35]*(sf[36]*nA)));let q9=(sf[54]*nz);let qa=(cG*q8);let qd=(cG*cG);let qh=(if ((cy)!=0.0){((qa-(cE*q9))/qd)}else{n_});let qi=(if ((cy)!=0.0){(sf[136]/cG)}else{n_});let qj=(if ((cy)!=0.0){(sf[4]/cG)}else{n_});let qn=(if ((cy)!=0.0){((qa-(cJ*q9))/qd)}else{n_});let qr=(if cP{n_}else{q5});let qs=(if cP{n_}else{q6});let qt=(if cP{n_}else{q7});let qu=(if cV{n_}else{(if cP{q5}else{n_})});let qv=(if cV{n_}else{(if cP{q6}else{n_})});let qw=(if cV{n_}else{(if cP{q7}else{n_})});let qJ=(if ((cy)!=0.0){((cX*qu)+(cW*(cX*qr)))}else{qu});let qK=(if ((cy)!=0.0){((cX*qv)+(cW*(cX*qs)))}else{qv});let qL=(if ((cy)!=0.0){((cX*qw)+(cW*(cX*qt)))}else{qw});let qM=(d7*qh);let qN=(d7*qi);let qO=(d7*qj);let r1=(dj*qn);let r7=(if ((cy)!=0.0){((if d6{(qM/d8)}else{(if da{qM}else{(if d1{qh}else{n_})})})-(if di{(r1/dk)}else{(if dm{r1}else{(if de{qn}else{n_})})}))}else{n_});let r8=(if ((cy)!=0.0){(if d6{(qN/d8)}else{(if da{qN}else{(if d1{qi}else{n_})})})}else{n_});let r9=(if ((cy)!=0.0){(if d6{(qO/d8)}else{(if da{qO}else{(if d1{qj}else{n_})})})}else{n_});let rn=(sf[55]*(og*(dx*(dw).ln())));let rr=(dz*dz);let rB=(if dD{n_}else{(if ((cy)!=0.0){(((ds*nT)+(aj*qJ))-(((dz*((dr*(sf[33]*(sf[34]*nA)))+(aD*r7)))-(du*rn))/rr))}else{n_})});let rC=(if dD{n_}else{(if ((cy)!=0.0){((aj*qK)-((aD*r8)/dz))}else{n_})});let rD=(if dD{n_}else{(if ((cy)!=0.0){((aj*qL)-((aD*r9)/dz))}else{n_})});let rQ=(dR*dR);let s0=(if ((dG)!=0.0){((-(dO*(dM*(sf[57]*nz))))/rQ)}else{qr});let s1=(if ((dG)!=0.0){(((dR*sf[138])-(dO*(dQ*(if ((dG)!=0.0){(if dK{sf[136]}else{n_})}else{n_}))))/rQ)}else{qs});let s2=(if ((dG)!=0.0){(((dR*sf[139])-(dO*(dQ*(if ((dG)!=0.0){(if dK{sf[4]}else{n_})}else{n_}))))/rQ)}else{qt});let s6=(if dW{n_}else{s0});let s7=(if dW{n_}else{s1});let s8=(if dW{n_}else{s2});let s9=(if e2{n_}else{(if dW{s0}else{qJ})});let sa=(if e2{n_}else{(if dW{s1}else{qK})});let sb_=(if e2{n_}else{(if dW{s2}else{qL})});let so=(if ((dG)!=0.0){((e4*s9)+(e3*(e4*s6)))}else{s9});let sp=(if ((dG)!=0.0){((e4*sa)+(e3*(e4*s7)))}else{sa});let sq=(if ((dG)!=0.0){((e4*sb_)+(e3*(e4*s8)))}else{sb_});let sJ=(if ((ed)!=0.0){((-(cp*(sf[30]*nz)))/(ee*ee))}else{s6});let sK=(if ((ed)!=0.0){(sf[4]/ee)}else{s7});let sL=(if ((ed)!=0.0){(sf[136]/ee)}else{s8});
        let sM=(sf[58]*nz);let sN=(ei*q8);let sQ=(ei*ei);let sS=(sf[136]/ei);let sT=(sf[4]/ei);let sU=(if ((ed)!=0.0){((sN-(cE*sM))/sQ)}else{qh});let sV=(if ((ed)!=0.0){sS}else{qi});let sW=(if ((ed)!=0.0){sT}else{qj});let sZ=((sN-(cJ*sM))/sQ);let t0=(if ((ed)!=0.0){sZ}else{qn});let t4=(if ep{n_}else{sJ});let t5=(if ep{n_}else{sK});let t6=(if ep{n_}else{sL});let t7=(if ev{n_}else{(if ep{sJ}else{so})});let t8=(if ev{n_}else{(if ep{sK}else{sp})});let t9=(if ev{n_}else{(if ep{sL}else{sq})});let tm=(if ((ed)!=0.0){((ex*t7)+(ew*(ex*t4)))}else{t7});let tn=(if ((ed)!=0.0){((ex*t8)+(ew*(ex*t5)))}else{t8});let to=(if ((ed)!=0.0){((ex*t9)+(ew*(ex*t6)))}else{t9});let tp=(eF*sU);let tq=(eF*sV);let tr=(eF*sW);let tE=(eR*t0);let tK=(if ((ed)!=0.0){((if eE{(tp/eG)}else{(if eI{tp}else{(if eA{sU}else{n_})})})-(if eQ{(tE/eS)}else{(if eU{tE}else{(if eM{t0}else{n_})})}))}else{r7});let tL=(if ((ed)!=0.0){(if eE{(tq/eG)}else{(if eI{tq}else{(if eA{sV}else{n_})})})}else{r8});let tM=(if ((ed)!=0.0){(if eE{(tr/eG)}else{(if eI{tr}else{(if eA{sW}else{n_})})})}else{r9});let uh=(if ((cy)!=0.0){((-(v*(sf[59]*nz)))/(f9*f9))}else{t4});let ui=(if ((cy)!=0.0){(sf[136]/f9)}else{n_});let uj=(if ((cy)!=0.0){(sf[4]/f9)}else{t5});let uk=(if ((cy)!=0.0){n_}else{t6});let un=((sN-(fd*sM))/sQ);let uo=(if ((cy)!=0.0){un}else{sU});let up=(if ((cy)!=0.0){sT}else{n_});let uq=(if ((cy)!=0.0){sS}else{sV});let ur=(if ((cy)!=0.0){n_}else{sW});let us=(if ((cy)!=0.0){sZ}else{t0});let ux=(if fj{n_}else{uh});let uy=(if fj{n_}else{ui});let uz=(if fj{n_}else{uj});let uA=(if fj{n_}else{uk});let uB=(if fp{n_}else{(if fj{uh}else{tm})});let uC=(if fp{n_}else{(if fj{ui}else{n_})});let uD=(if fp{n_}else{(if fj{uj}else{tn})});let uE=(if fp{n_}else{(if fj{uk}else{to})});let uV=(if ((cy)!=0.0){((fr*uB)+(fq*(fr*ux)))}else{uB});let uW=(if ((cy)!=0.0){((fr*uC)+(fq*(fr*uy)))}else{uC});let uX=(if ((cy)!=0.0){((fr*uD)+(fq*(fr*uz)))}else{uD});let uY=(if ((cy)!=0.0){((fr*uE)+(fq*(fr*uA)))}else{uE});let uZ=(fz*uo);let v0=(fz*up);let v1=(fz*uq);let v2=(fz*ur);let vj=(fL*us);let vp=(if ((cy)!=0.0){((if fy{(uZ/fA)}else{(if fC{uZ}else{(if fu{uo}else{n_})})})-(if fK{(vj/fM)}else{(if fO{vj}else{(if fG{us}else{n_})})}))}else{tK});let vq=(if ((cy)!=0.0){(if fy{(v0/fA)}else{(if fC{v0}else{(if fu{up}else{n_})})})}else{n_});let vr=(if ((cy)!=0.0){(if fy{(v1/fA)}else{(if fC{v1}else{(if fu{uq}else{n_})})})}else{tL});let vs=(if ((cy)!=0.0){(if fy{(v2/fA)}else{(if fC{v2}else{(if fu{ur}else{n_})})})}else{tM});let vZ=(if dD{n_}else{(if ((cy)!=0.0){(((fU*nT)+(aj*uV))-(((g0*((fT*(sf[39]*(sf[40]*nA)))+(aS*vp)))-(fW*(sf[55]*(og*(fY*(fX).ln())))))/(g0*g0)))}else{n_})});let w0=(if dD{n_}else{(if ((cy)!=0.0){((aj*uW)-((aS*vq)/g0))}else{n_})});let w1=(if dD{n_}else{(if ((cy)!=0.0){((aj*uX)-((aS*vr)/g0))}else{n_})});let w2=(if dD{n_}else{(if ((cy)!=0.0){((aj*uY)-((aS*vs)/g0))}else{n_})});let yp=(sf[148]/hu);let yq=(sf[149]/hu);let yr=(sf[150]/hu);let ys=(hx*vZ);let yv=((hx*w0)+(g4*yp));let yy=((hx*w1)+(g4*yq));let yB=((hx*w2)+(g4*yr));let yC=(hx*rB);let yD=(dE*yp);let yG=((hx*rC)+(dE*yq));let yJ=((hx*rD)+(dE*yr));let zx=(cd*cd);let An=(sf[93]*(-p0));let Ax=(bB*bB);let Bi=(if jG{(((jM*p0)+(bB*(-(jL*(sf[99]*((-((-(cp*p0))/Ax))/jI))))))/sf[99])}else{(if ((jl)!=0.0){((ju*p0)/sf[99])}else{n_})});let Bj=(if jG{((bB*(-(jL*(sf[99]*((-(sf[4]/bB))/jI)))))/sf[99])}else{n_});let Bk=(if jG{((bB*(-(jL*(sf[99]*((-(sf[136]/bB))/jI)))))/sf[99])}else{n_});let Bl=(if jG{n_}else{(if ((jl)!=0.0){(jr*((jC*An)+(jj*(((bB*(sf[100]*An))-(jA*p0))/Ax))))}else{n_})});let Bm=(if jG{n_}else{(if ((jl)!=0.0){(jr*((sf[4]*jC)+(jj*(sf[153]/bB))))}else{n_})});let Bn=(if jG{n_}else{(if ((jl)!=0.0){(jr*((jC*sf[136])+(jj*(sf[154]/bB))))}else{n_})});let Bx=(sf[93]*(-pp));let BG=(sf[155]/bV);let BH=(bV*(sf[105]*Bx));let BK=(bV*bV);let BM=(sf[156]/bV);let C9=(-(sf[4]/bV));let Cb=(-(sf[136]/bV));let Cw=(if kg{((bV*(-(kl*(sf[104]*(C9/ki)))))/sf[104])}else{n_});let Cx=(if kg{(((km*pp)+(bV*(-(kl*(sf[104]*((-((-(cs*pp))/BK))/ki))))))/sf[104])}else{(if ((jX)!=0.0){((k4*pp)/sf[104])}else{Bi})});let Cy=(if kg{((bV*(-(kl*(sf[104]*(Cb/ki)))))/sf[104])}else{n_});
        let Cz=(if kg{n_}else{(if ((jX)!=0.0){n_}else{Bj})});let CA=(if kg{n_}else{(if ((jX)!=0.0){n_}else{Bk})});let CB=(if kg{n_}else{(if ((jX)!=0.0){(k1*((sf[4]*kc)+(jV*BG)))}else{n_})});let CC=(if kg{n_}else{(if ((jX)!=0.0){(k1*((kc*Bx)+(jV*((BH-(ka*pp))/BK))))}else{Bl})});let CD=(if kg{n_}else{(if ((jX)!=0.0){(k1*((kc*sf[136])+(jV*BM)))}else{n_})});let CE=(if kg{n_}else{(if ((jX)!=0.0){n_}else{Bm})});let CF=(if kg{n_}else{(if ((jX)!=0.0){n_}else{Bn})});let El=(if sb[9]{n_}else{(if ((sf[109])!=0.0){(sf[113]*yC)}else{n_})});let Em=(if sb[9]{n_}else{(if ((sf[109])!=0.0){(sf[113]*yD)}else{n_})});let En=(if sb[9]{n_}else{(if ((sf[109])!=0.0){(sf[113]*yG)}else{n_})});let Eo=(if sb[9]{n_}else{(if ((sf[109])!=0.0){(sf[113]*yJ)}else{n_})});let Es=(if REACTIVE { 1.0 } else { ddt_scale });let Et=(sf[133]*Es);let EU=(if ((sf[83])!=0.0){(is*Es)}else{n_});let EX=(sf[134]*Es);let EY=(if ((sf[116])!=0.0){EX}else{n_});let F2=(if sb[28]{EX}else{n_});let F6=(if sb[28]{(sf[135]*Es)}else{n_});let Gf=(sf[3]*(sf[4]*((jR*((bI*((-(sf[41]*(sf[46]*(-(((bp*oL)-(bq*oK))/oP)))))/(by*by)))+(bz*(sf[46]*(p6-(((bp*(p0-oK))-(bC*oK))/oP))))))+(bJ*(Bi+Bl)))));let Gg=(sf[3]*(sf[4]*(bJ*(Bj+Bm))));let Gh=(sf[3]*(sf[4]*(bJ*(Bk+Bn))));let Gl=(sf[3]*(sf[4]*(is*rB)));let Gm=(sf[3]*(sf[4]*(is*rC)));let Gn=(sf[3]*(sf[4]*(is*rD)));let Gt=(sf[3]*(sf[4]*(sf[107]*(c1*(Cw+CB)))));let Gu=(sf[3]*(sf[4]*(sf[107]*((kr*pz)+(c1*(Cx+CC))))));let Gv=(sf[3]*(sf[4]*(sf[107]*(c1*(Cy+CD)))));let Gw=(sf[3]*(sf[4]*(sf[107]*(c1*(Cz+CE)))));let Gx=(sf[3]*(sf[4]*(sf[107]*(c1*(CA+CF)))));let GD=(sf[3]*(sf[4]*(sf[106]*(c1*((if kM{n_}else{(if ((ky)!=0.0){n_}else{Cw})})+(if kM{n_}else{(if ((ky)!=0.0){n_}else{CB})}))))));let GE=(sf[3]*(sf[4]*(sf[106]*((kX*pz)+(c1*((if kM{(((kS*pp)+(bV*(-(kR*(sf[104]*((-((-(v*pp))/BK))/kO))))))/sf[104])}else{(if ((ky)!=0.0){((kC*pp)/sf[104])}else{Cx})})+(if kM{n_}else{(if ((ky)!=0.0){(kz*((kI*Bx)+(kw*((BH-(kG*pp))/BK))))}else{CC})})))))));let GF=(sf[3]*(sf[4]*(sf[106]*(c1*((if kM{((bV*(-(kR*(sf[104]*(Cb/kO)))))/sf[104])}else{(if ((ky)!=0.0){n_}else{Cy})})+(if kM{n_}else{(if ((ky)!=0.0){(kz*((kI*sf[136])+(kw*BM)))}else{CD})}))))));let GG=(sf[3]*(sf[4]*(sf[106]*(c1*((if kM{((bV*(-(kR*(sf[104]*(C9/kO)))))/sf[104])}else{(if ((ky)!=0.0){n_}else{Cz})})+(if kM{n_}else{(if ((ky)!=0.0){(kz*((sf[4]*kI)+(kw*BG)))}else{CE})}))))));let GH=(sf[3]*(sf[4]*(sf[106]*(c1*((if kM{n_}else{(if ((ky)!=0.0){n_}else{CA})})+(if kM{n_}else{(if ((ky)!=0.0){n_}else{CF})}))))));let GM=(sf[3]*(sf[4]*(sf[81]*ys)));let GN=(sf[3]*(sf[4]*(sf[81]*yv)));let GO=(sf[3]*(sf[4]*(sf[81]*yy)));let GP=(sf[3]*(sf[4]*(sf[81]*yB)));let GT=(sf[3]*(sf[4]*(if j7{((jd*(sf[4]*cj))+(j8*(sf[151]/cd)))}else{(if ((iV)!=0.0){((iW*(-(j2*(sf[91]*((-(sf[4]/cd))/iZ)))))/sf[91])}else{n_})})));let GU=(sf[3]*(sf[4]*(if j7{((jd*(cm*pX))+(j8*((-(jb*pN))/zx)))}else{(if ((iV)!=0.0){(((j3*((cj*pN)+(cd*pX)))+(iW*(-(j2*(sf[91]*((-((-(cm*pN))/zx))/iZ))))))/sf[91])}else{n_})})));let GV=(sf[3]*(sf[4]*(if j7{((jd*(cj*sf[136]))+(j8*(sf[152]/cd)))}else{(if ((iV)!=0.0){((iW*(-(j2*(sf[91]*((-(sf[136]/cd))/iZ)))))/sf[91])}else{n_})})));let H0=(sf[3]*(-El));let H1=(sf[3]*(-Em));let H2=(sf[3]*(-En));let H3=(sf[3]*(-Eo));let H4=(sf[3]*El);let H5=(sf[3]*Em);let H6=(sf[3]*En);let H7=(sf[3]*Eo);

        CommonStampValues {
            b, m_, n_, s, t, u, v, H,
            J, M, ae, al, am, aq, ar, as_,
            ck, cn, co, cp, cq, cM, d0, d3,
            dw, dz, dE, dG, e6, ed, el, ez,
            eZ, fe, ff, fg, fn_, ft, fT, fX,
            g4, h0, hx, hy, hz, ie, is, iz,
            lY, m6, ml, mp, mu, mA, n7, n9,
            nb, nd, nf, nh, nj, nk, nz, nB,
            nQ, rn, rr, rB, rC, rD, so, sp,
            sq, sS, sT, sZ, tm, tn, to, tK,
            tL, tM, un, uo, up, uq, ur, us,
            ux, uy, uz, uA, uV, uW, uX, uY,
            vp, vq, vr, vs, vZ, w0, w1, w2,
            yp, yq, yr, ys, yv, yy, yB, yC,
            yD, yG, yJ, Et, EU, EY, F2, F6,
            Gf, Gg, Gh, Gl, Gm, Gn, Gt, Gu,
            Gv, Gw, Gx, GD, GE, GF, GG, GH,
            GM, GN, GO, GP, GT, GU, GV, H0,
            H1, H2, H3, H4, H5, H6, H7,
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
            lY, m6, ml, mp, mu, mA, n7, n9,
            nb, nd, nf, nh, nj, nk, nz, nB,
            nQ, rn, rr, rB, rC, rD, so, sp,
            sq, sS, sT, sZ, tm, tn, to, tK,
            tL, tM, un, uo, up, uq, ur, us,
            ux, uy, uz, uA, uV, uW, uX, uY,
            vp, vq, vr, vs, vZ, w0, w1, w2,
            yp, yq, yr, ys, yv, yy, yB, yC,
            yD, yG, yJ, Et, EU, EY, F2, F6,
            Gf, Gg, Gh, Gl, Gm, Gn, Gt, Gu,
            Gv, Gw, Gx, GD, GE, GF, GG, GH,
            GM, GN, GO, GP, GT, GU, GV, H0,
            H1, H2, H3, H4, H5, H6, H7,
        }=self.eval_common_stamp_values::<false>(ctx);
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
        let x=(v<n_);let z=(-(if x{v}else{n_}));let D=(m_+(sf[5]*f64::powf(z,sf[6])));let O=(M*sf[10]);let P=(D*O);let R=(M*sf[11]);let aw=((ae/sf[32])).exp();let ax=(sf[31]*aw);let ay=(ax/M);let ct=(cq-s);let cv=(ck-cn);let dN=-1.0;let e7=(e6-m_);let ea=(!((dG)!=0.0));let f0=(ez-m_);let f2=(n_*eZ);let f6=(!((ed)!=0.0));let g6=(if (ay>n_){m_}else{n_});let g7=(H*sf[32]);let g9=(if ((g6)!=0.0){(v/g7)}else{fn_});let ga=(if ((g6)!=0.0){fe}else{ff});let gb=(if ((g6)!=0.0){el}else{fg});let gd=(if (g9>cM){m_}else{n_});let ge=(((g6)!=0.0)&&((gd)!=0.0));let gk=(((g6)!=0.0)&&(!((gd)!=0.0)));let gl=(if gk{m_}else{(if ge{(m_+(g9-cM))}else{ft})});let gm=((if ge{cM}else{g9})).exp();let gp=(ga>=d0);let gq=(!gp);let gr=(ga<=d3);let gt=(gq&&(!gr));let gu=(ga).exp();let gv=(m_+gu);let gx=(gq&&gr);let gB=(gb>=d0);let gC=(!gB);let gD=(gb<=d3);let gF=(gC&&(!gD));let gG=(gb).exp();let gH=(m_+gG);let gJ=(gC&&gD);let gP=((if ((g6)!=0.0){(gl*gm)}else{gl})-m_);let gU=(m_+(sf[55]*f64::powf(fX,sf[37])));let gY=(!((g6)!=0.0));let h3=1e-9;let h7=(((if (h0<cp){h0}else{cp})/(if (dw>h3){dw}else{h3}))).abs();let h8=(dE-(if ea{n_}else{(if ((dG)!=0.0){(am*e7)}else{n_})}));let ha=((if f6{n_}else{(if ((ed)!=0.0){((as_*f0)-(f2/dz))}else{n_})})+(h8/P));let hc=((if gY{n_}else{(if ((g6)!=0.0){((ay*gP)-((n_*(if ((g6)!=0.0){((if gt{(gv).ln()}else{(if gx{gu}else{(if gp{ga}else{n_})})})-(if gF{(gH).ln()}else{(if gJ{gG}else{(if gB{gb}else{n_})})}))}else{fT}))/gU))}else{n_})})+(g4/R));let hE=(dE*sf[63]);let hW=((J*sf[69])).exp();let hZ=f64::powf((m_+f64::powf((((sf[4]*ct)/sf[64])).abs(),sf[65])),sf[70]);let i0=((sf[68]*hW)*hZ);let i4=((J*sf[72])).exp();let i5=(sf[71]*i4);let i9=((J*sf[74])).exp();let ic=f64::powf((m_+f64::powf((((sf[4]*cv)/sf[66])).abs(),sf[67])),sf[75]);let id=((sf[73]*i9)*ic);let iF=(m_+f64::powf(((iz).abs()/sf[84]),sf[85]));let iH=(if ((sf[83])!=0.0){(i0/iF)}else{i0});let lH=((if ((sf[87])!=0.0){(iH+sf[88])}else{iH})/sf[3]);let lM=((if ((sf[87])!=0.0){(id+sf[90])}else{id})/sf[3]);let lR=((if ((sf[87])!=0.0){(i5+sf[89])}else{i5})/sf[3]);let lU=1e-6;let ma=ctx.node_voltage(n[0]);let me=((-((ha*ie)).abs())-((hc*(cq-ma))).abs());let mF=ctx.simparam_or("gmin", n_);let mK=(lH>sf[129]);let mL=(if mK{lH}else{sf[129]});let mO=(lM>sf[129]);let mP=(if mO{lM}else{sf[129]});let mS=(ma-t);let mT=(lR>sf[129]);let mU=(if mT{lR}else{sf[129]});let nu=(sf[6]*f64::powf(z,sf[137]));let nD=(M*(sf[9]*nB));let nF=(D*(sf[10]*nD));let nG=(O*(sf[5]*((-(if x{sf[136]}else{n_}))*nu)));let nH=(O*(sf[5]*((-(if x{sf[4]}else{n_}))*nu)));let o2=(M*M);let wa=(if ((g6)!=0.0){((-(v*(sf[32]*nz)))/(g7*g7))}else{ux});let wb=(if ((g6)!=0.0){(sf[136]/g7)}else{uy});let wc=(if ((g6)!=0.0){(sf[4]/g7)}else{uz});let wd=(if ((g6)!=0.0){n_}else{uA});let we=(if ((g6)!=0.0){un}else{uo});let wf=(if ((g6)!=0.0){sT}else{up});let wg=(if ((g6)!=0.0){sS}else{uq});let wh=(if ((g6)!=0.0){n_}else{ur});let wi=(if ((g6)!=0.0){sZ}else{us});let wr=(if gk{n_}else{(if ge{wa}else{uV})});let ws=(if gk{n_}else{(if ge{wb}else{uW})});let wt=(if gk{n_}else{(if ge{wc}else{uX})});let wu=(if gk{n_}else{(if ge{wd}else{uY})});let wP=(gu*we);let wQ=(gu*wf);let wR=(gu*wg);let wS=(gu*wh);let x9=(gG*wi);let xP=(P*P);let za=(hZ*(sf[68]*(hW*(sf[69]*nB))));let F7=(-mF);

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
            multiplicity * (lY),
            9,
            multiplicity * (Et),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            None,
            multiplicity * ((if ((sf[83])!=0.0){(is*(-(dE/P)))}else{n_})),
            [3, 4, 5, 6],
            [(if ((sf[83])!=0.0){(is*(-(((P*rB)-(dE*nF))/xP)))}else{n_}), (if ((sf[83])!=0.0){(is*(-((-(dE*nG))/xP)))}else{n_}), (if ((sf[83])!=0.0){(is*(-(((P*rC)-(dE*nH))/xP)))}else{n_}), (if ((sf[83])!=0.0){(is*(-(rD/P)))}else{n_})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * ((if ((sf[83])!=0.0){iz}else{n_})),
            8,
            multiplicity * (sf[157]),
        );
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * (m6),
            8,
            multiplicity * (EU),
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
            multiplicity * ((if ((sf[116])!=0.0){me}else{n_})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if ((sf[116])!=0.0){(b/sf[115])}else{n_})),
            3,
            multiplicity * (sf[159]),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (ml),
            3,
            multiplicity * (EY),
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
            multiplicity * (mu),
            3,
            multiplicity * (F2),
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
            multiplicity * (mA),
            7,
            multiplicity * (F6),
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
        stamper.stamp_current_node2_local(
            Some(5),
            Some(6),
            multiplicity * ((co*mF)),
            5,
            multiplicity * (mF),
            6,
            multiplicity * (F7),
        );
        stamper.stamp_current_node2_local(
            Some(5),
            Some(4),
            multiplicity * ((u*mF)),
            4,
            multiplicity * (F7),
            5,
            multiplicity * (mF),
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(6),
            multiplicity * ((mF*(t-cn))),
            4,
            multiplicity * (mF),
            6,
            multiplicity * (F7),
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * ((if ((sf[130])!=0.0){(ct/mL)}else{n_})),
            1,
            multiplicity * ((if ((sf[130])!=0.0){(m_/mL)}else{n_})),
            3,
            multiplicity * ((if ((sf[130])!=0.0){((-(ct*(if mK{((if ((sf[83])!=0.0){(za/iF)}else{za})/sf[3])}else{n_})))/(mL*mL))}else{n_})),
            5,
            multiplicity * ((if ((sf[130])!=0.0){(dN/mL)}else{n_})),
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
            multiplicity * ((if ((sf[131])!=0.0){(cv/mP)}else{n_})),
            2,
            multiplicity * ((if ((sf[131])!=0.0){(m_/mP)}else{n_})),
            3,
            multiplicity * ((if ((sf[131])!=0.0){((-(cv*(if mO{((ic*(sf[73]*(i9*(sf[74]*nB))))/sf[3])}else{n_})))/(mP*mP))}else{n_})),
            6,
            multiplicity * ((if ((sf[131])!=0.0){(dN/mP)}else{n_})),
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
            multiplicity * ((if ((sf[132])!=0.0){(mS/mU)}else{n_})),
            0,
            multiplicity * ((if ((sf[132])!=0.0){(m_/mU)}else{n_})),
            3,
            multiplicity * ((if ((sf[132])!=0.0){((-(mS*(if mT{((sf[71]*(i4*(sf[72]*nB)))/sf[3])}else{n_})))/(mU*mU))}else{n_})),
            4,
            multiplicity * ((if ((sf[132])!=0.0){(dN/mU)}else{n_})),
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
            [(sf[3]*(sf[4]*((if f6{n_}else{(if ((ed)!=0.0){(((f0*(((M*(sf[29]*(aq*(nQ/sf[30]))))-(ar*nD))/o2))+(as_*tm))-(((dz*(n_*tK))-(f2*rn))/rr))}else{n_})})+(((P*(rB-(if ea{n_}else{(if ((dG)!=0.0){((e7*(sf[28]*(al*(sf[26]*nB))))+(am*so))}else{n_})})))-(h8*nF))/xP)))), (sf[3]*(sf[4]*((-(h8*nG))/xP))), (sf[3]*(sf[4]*((if f6{n_}else{(if ((ed)!=0.0){((as_*tn)-((n_*tL)/dz))}else{n_})})+(((P*(rC-(if ea{n_}else{(if ((dG)!=0.0){(am*sp)}else{n_})})))-(h8*nH))/xP)))), (sf[3]*(sf[4]*((if f6{n_}else{(if ((ed)!=0.0){((as_*to)-((n_*tM)/dz))}else{n_})})+((rD-(if ea{n_}else{(if ((dG)!=0.0){(am*sq)}else{n_})}))/P))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * ((sf[3]*(sf[4]*hc))),
            [3, 4, 5, 6],
            [(sf[3]*(sf[4]*((if gY{n_}else{(if ((g6)!=0.0){(((gP*(((M*(sf[31]*(aw*(nQ/sf[32]))))-(ax*nD))/o2))+(ay*(if ((g6)!=0.0){((gm*wr)+(gl*(gm*(if ge{n_}else{wa}))))}else{wr})))-((n_*(if ((g6)!=0.0){((if gt{(wP/gv)}else{(if gx{wP}else{(if gp{we}else{n_})})})-(if gF{(x9/gH)}else{(if gJ{x9}else{(if gB{wi}else{n_})})}))}else{vp}))/gU))}else{n_})})+(((R*vZ)-(g4*(sf[11]*nD)))/(R*R))))), (sf[3]*(sf[4]*((if gY{n_}else{(if ((g6)!=0.0){((ay*(if ((g6)!=0.0){((gm*ws)+(gl*(gm*(if ge{n_}else{wb}))))}else{ws}))-((n_*(if ((g6)!=0.0){(if gt{(wQ/gv)}else{(if gx{wQ}else{(if gp{wf}else{n_})})})}else{vq}))/gU))}else{n_})})+(w0/R)))), (sf[3]*(sf[4]*((if gY{n_}else{(if ((g6)!=0.0){((ay*(if ((g6)!=0.0){((gm*wt)+(gl*(gm*(if ge{n_}else{wc}))))}else{wt}))-((n_*(if ((g6)!=0.0){(if gt{(wR/gv)}else{(if gx{wR}else{(if gp{wg}else{n_})})})}else{vr}))/gU))}else{n_})})+(w1/R)))), (sf[3]*(sf[4]*((if gY{n_}else{(if ((g6)!=0.0){((ay*(if ((g6)!=0.0){((gm*wu)+(gl*(gm*(if ge{n_}else{wd}))))}else{wu}))-((n_*(if ((g6)!=0.0){(if gt{(wS/gv)}else{(if gx{wS}else{(if gp{wh}else{n_})})})}else{vs}))/gU))}else{n_})})+(w2/R))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * ((sf[4]*(sf[3]*(-hy)))),
            [3, 4, 5, 6],
            [(sf[4]*(sf[3]*(-ys))), (sf[4]*(sf[3]*(-yv))), (sf[4]*(sf[3]*(-yy))), (sf[4]*(sf[3]*(-yB)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * ((sf[3]*(sf[4]*(((h7*hz)*sf[62])+(hx*hE))))),
            [3, 4, 5, 6],
            [(sf[3]*(sf[4]*((sf[62]*(h7*yC))+(hx*(sf[63]*rB))))), (sf[3]*(sf[4]*((sf[62]*(h7*yD))+(hE*yp)))), (sf[3]*(sf[4]*((sf[62]*(h7*yG))+((hE*yq)+(hx*(sf[63]*rC)))))), (sf[3]*(sf[4]*((sf[62]*(h7*yJ))+((hE*yr)+(hx*(sf[63]*rD))))))],
            [],
            [],
            multiplicity,
        );
        let n7_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, n7);
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (n7_ddt),
            3,
            multiplicity * (((Gf) * ddt_scale)),
            5,
            multiplicity * (((Gg) * ddt_scale)),
            6,
            multiplicity * (((Gh) * ddt_scale)),
        );
        let n9_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, n9);
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (n9_ddt),
            3,
            multiplicity * (((Gl) * ddt_scale)),
            5,
            multiplicity * (((Gm) * ddt_scale)),
            6,
            multiplicity * (((Gn) * ddt_scale)),
        );
        let nb_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, nb);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(1),
            Some(4),
            multiplicity * (nb_ddt),
            [1, 3, 4, 5, 6],
            [((Gt) * ddt_scale), ((Gu) * ddt_scale), ((Gv) * ddt_scale), ((Gw) * ddt_scale), ((Gx) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let nd_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, nd);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(4),
            multiplicity * (nd_ddt),
            [1, 3, 4, 5, 6],
            [((GD) * ddt_scale), ((GE) * ddt_scale), ((GF) * ddt_scale), ((GG) * ddt_scale), ((GH) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let nf_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, nf);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (nf_ddt),
            [3, 4, 5, 6],
            [((GM) * ddt_scale), ((GN) * ddt_scale), ((GO) * ddt_scale), ((GP) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let nh_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, nh);
        stamper.stamp_current_node3_local(
            Some(2),
            Some(4),
            multiplicity * (nh_ddt),
            2,
            multiplicity * (((GT) * ddt_scale)),
            3,
            multiplicity * (((GU) * ddt_scale)),
            4,
            multiplicity * (((GV) * ddt_scale)),
        );
        let nj_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, nj);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (nj_ddt),
            [3, 4, 5, 6],
            [((H0) * ddt_scale), ((H1) * ddt_scale), ((H2) * ddt_scale), ((H3) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let nk_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, nk);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (nk_ddt),
            [3, 4, 5, 6],
            [((H4) * ddt_scale), ((H5) * ddt_scale), ((H6) * ddt_scale), ((H7) * ddt_scale)],
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
            lY, m6, ml, mp, mu, mA, n7, n9,
            nb, nd, nf, nh, nj, nk, nz, nB,
            nQ, rn, rr, rB, rC, rD, so, sp,
            sq, sS, sT, sZ, tm, tn, to, tK,
            tL, tM, un, uo, up, uq, ur, us,
            ux, uy, uz, uA, uV, uW, uX, uY,
            vp, vq, vr, vs, vZ, w0, w1, w2,
            yp, yq, yr, ys, yv, yy, yB, yC,
            yD, yG, yJ, Et, EU, EY, F2, F6,
            Gf, Gg, Gh, Gl, Gm, Gn, Gt, Gu,
            Gv, Gw, Gx, GD, GE, GF, GG, GH,
            GM, GN, GO, GP, GT, GU, GV, H0,
            H1, H2, H3, H4, H5, H6, H7,
        }=self.eval_common_stamp_values::<true>(ctx);
        let p=&(*self.params);
        let m=self.multiplicity;
        let multiplicity=m;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        stamper.stamp_current_reactive_node1_local(
            Some(9),
            None,
            9,
            multiplicity * (Et),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(8),
            None,
            8,
            multiplicity * (EU),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(3),
            None,
            3,
            multiplicity * (EY),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(3),
            None,
            3,
            multiplicity * (F2),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(7),
            None,
            7,
            multiplicity * (F6),
        );
        stamper.stamp_current_reactive_node3_local(
            Some(5),
            Some(6),
            3,
            multiplicity * (Gf),
            5,
            multiplicity * (Gg),
            6,
            multiplicity * (Gh),
        );
        stamper.stamp_current_reactive_node3_local(
            Some(5),
            Some(6),
            3,
            multiplicity * (Gl),
            5,
            multiplicity * (Gm),
            6,
            multiplicity * (Gn),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(4),
            &[1, 3, 4, 5, 6],
            &[Gt, Gu, Gv, Gw, Gx],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[1, 3, 4, 5, 6],
            &[GD, GE, GF, GG, GH],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5, 6],
            &[GM, GN, GO, GP],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3_local(
            Some(2),
            Some(4),
            2,
            multiplicity * (GT),
            3,
            multiplicity * (GU),
            4,
            multiplicity * (GV),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[3, 4, 5, 6],
            &[H0, H1, H2, H3],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5, 6],
            &[H4, H5, H6, H7],
            &[],
            &[],
            multiplicity,
        );
    }
}
