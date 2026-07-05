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
    b: f64, d: f64, G: f64, H: f64, W: f64, bK: f64,
    fL: f64, fP: f64, g1: f64, gr: f64, kr: f64, kv: f64,
    kx: f64, kC: f64, kF: f64, kK: f64, kS: f64, kV: f64,
    kY: f64, l2: f64, lD: f64, lE: f64, lG: f64, lJ: bool,
    lK: f64, n6: f64, p4: f64, q2: f64, qr: f64, qu: f64,
    qx: f64, qY: f64, sg: f64, sQ: f64, sR: f64, sW: f64,
    sX: f64, tg: f64, ti: f64, tl: bool, tm: f64, tv: f64,
    u1: f64, u3: f64, u5: f64, ua: bool, ub: f64, ui: f64,
    uj: f64, ul: f64, uq: bool, us: f64, vi: f64, vk: f64,
    vm: f64, vr: bool, vs: f64, vT: f64, w6: f64, wj: f64,
    ww: f64, wD: f64, wE: f64, wH: f64, wJ: f64, wO: bool,
    wP: f64, wV: f64, wZ: f64, x2: f64, xa: f64, xb: f64,
    xc: f64, xe: f64, xg: f64, xk: f64, xl: f64, xn: f64,
    xq: f64, xs: f64, xt: bool, xy: bool, xz: f64, yb: f64,
    yd: f64, yf: f64, yg: f64, yj: f64, yl: f64, yq: bool,
    yr: f64, yw: f64, yz: f64, yB: f64, yJ: f64, yK: f64,
    yL: f64, yN: f64, yS: f64, yT: f64, yV: f64, yX: f64,
    yZ: f64, z0: bool, z5: bool, z6: f64, Ac: f64, At: f64,
    AP: f64, BZ: f64, Cb: f64, Co: bool, Cp: bool, Cq: f64,
    Ct: bool, Cu: f64, Cy: f64, Cz: f64, CB: f64, CF: f64,
    CH: f64, CM: bool, CN: f64, D2: bool, EL: bool, EM: f64,
    EO: f64, EQ: f64, ES: f64, EU: f64, EV: bool, EX: bool,
    F5: f64, F8: bool, F9: f64, Fa: f64, Fg: bool, Fi: f64,
    Fj: f64, Fn: f64, Fp: f64, Fs: f64, Fu: f64, Fz: bool,
    FA: f64, L4: f64, LA: f64, Mh: f64, Mk: f64, Mn: f64,
    Mq: f64, Mu: f64, My: f64, MG: f64, MM: f64, MX: f64,
    ND: f64, NE: f64, NF: f64, NG: f64, PA: f64, PB: f64,
    PC: f64, Uf: f64, Ug: f64, Uh: f64, WD: f64, WE: f64,
    WF: f64, Xk: f64, Xl: f64, Xm: f64, Xt: f64, Xu: f64,
    Xv: f64, XC: f64, XD: f64, XE: f64, Ya: f64, Yb: f64,
    a14: f64, a15: f64, a16: f64, a2y: f64, a2z: f64, a2A: f64,
    a2B: f64, a2E: f64, a2H: f64, a2K: f64, a2N: f64, a2O: f64,
    a2P: f64, a2Q: f64, a2S: f64, a2W: f64, a2Z: f64, a3x: f64,
    a3y: f64, a4u: f64, a4v: f64, a6E: f64, a6F: f64, a6G: f64,
    a7z: f64, a7A: f64, a7B: f64, a7O: f64, a7P: f64, a7Q: f64,
    a8b: f64, a8c: f64, a8d: f64, a8e: f64, a8f: f64, a8w: f64,
    a8x: f64, a8y: f64, a8z: f64, a8A: f64, afY: f64, afZ: f64,
    ag0: f64, ag1: f64, age: f64, agf: f64, agg: f64, agh: f64,
    agi: f64, agj: f64, agk: f64, agl: f64, aim: f64, ain: f64,
    aio: f64, aip: f64, aiq: f64, air: f64, ais: f64, ait: f64,
    ano: f64, anp: f64, anq: f64, anr: f64, aON: f64, aOO: f64,
    aOP: f64, aOQ: f64, aOR: f64, aOS: f64, aRy: f64, aRz: f64,
    aRA: f64, aRB: f64, aRC: f64, aRD: f64, aRR: f64, aRS: f64,
    aRX: f64, aRY: f64, aRZ: f64, aS0: f64, aS1: f64, aS2: f64,
    aSf: f64, aSg: f64, aSh: f64, aSi: f64, aSj: f64, aSk: f64,
    aTb: f64, aTc: f64, aTd: f64, aTe: f64, aTf: f64, aTg: f64,
    aTh: f64, aTi: f64, aTW: f64, aTX: f64, aTY: f64, aTZ: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let b=1.0;let d=0.0;let G=0.001;let H=2.0;let W=0.1;let bK=3.0;let fL=1e-6;let fP=0.5;let g1=4.0;let gr=6.0;let ko=ctx.node_voltage(n[5]);let kp=ctx.node_voltage(n[6]);let kr=(sf[0]*(ko-kp));let ks=ctx.node_voltage(n[7]);let ku=(sf[0]*(ko-ks));let kv=ctx.node_voltage(n[3]);let kx=(sf[0]*(ko-kv));let ky=ctx.node_voltage(n[4]);let kA=(sf[0]*(ky-kv));let kC=(sf[0]*(ky-ko));let kE=(sf[0]*(kp-ks));let kF=ctx.node_voltage(n[2]);let kI=ctx.node_voltage(n[1]);let kK=(sf[0]*(kI-ky));let kP=(sf[0]*(kI-ctx.node_voltage(n[0])));let kQ=ctx.node_voltage(n[9]);let kS=(sf[0]*(kQ-kp));let kV=(sf[0]*(ctx.node_voltage(n[8])-kQ));let kY=(((ku+kC)-kE)-kS);let l2=((kY+(kK+(-kP)))-kV);let l3=(kP+l2);let l4=(sf[381]*ku);let l7=(if (l4<sf[198]){b}else{d});let l8=(l4).exp();let la=(!(l7!=0.0));let lc=(if la{sf[199]}else{d});let lh=(sf[381]*kx);let li=(lh/sf[588]);let lk=(if (li<sf[198]){b}else{d});let ll=(li).exp();let ln=(!(lk!=0.0));let lo=(if ln{sf[199]}else{lc});let ls=(if ln{(lo*(b+(li-sf[198])))}else{(if (lk!=0.0){ll}else{d})});let lt=(sf[381]*kY);let lv=(if (lt<sf[198]){b}else{d});let lw=(lt).exp();let ly=(!(lv!=0.0));let lz=(if ly{sf[199]}else{lo});let lD=(if ly{(lz*(b+(lt-sf[198])))}else{(if (lv!=0.0){lw}else{d})});let lE=(sf[381]*kC);let lG=(if (lE<sf[198]){b}else{d});let lJ=(!(lG!=0.0));let lK=(if lJ{sf[199]}else{lz});let lP=(sf[381]*l3);let lR=(if (lP<sf[198]){b}else{d});let lS=(lP).exp();let lU=(!(lR!=0.0));let lV=(if lU{sf[199]}else{lK});let lZ=(if lU{(lV*(b+(lP-sf[198])))}else{(if (lR!=0.0){lS}else{d})});let m1=(sf[381]*(l3-sf[469]));let m3=(if (m1<sf[198]){b}else{d});let m4=(m1).exp();let m6=(!(m3!=0.0));let m7=(if m6{sf[199]}else{lV});let md=(sf[381]*(kY-sf[469]));let mf=(if (md<sf[198]){b}else{d});let mg=(md).exp();let mi=(!(mf!=0.0));let mj=(if mi{sf[199]}else{m7});let mp=(sf[381]*(ku-sf[469]));let mr=(if (mp<sf[198]){b}else{d});let ms=(mp).exp();let mu=(!(mr!=0.0));let mv=(if mu{sf[199]}else{mj});let mz=(if mu{(mv*(b+(mp-sf[198])))}else{(if (mr!=0.0){ms}else{d})});let mB=(sf[381]*(kr-sf[469]));let mD=(if (mB<sf[198]){b}else{d});let mE=(mB).exp();let mG=(!(mD!=0.0));let mH=(if mG{sf[199]}else{mv});let mL=(if mG{(mH*(b+(mB-sf[198])))}else{(if (mD!=0.0){mE}else{d})});let mO=((b+(g1*mz))).sqrt();let mR=((b+(g1*mL))).sqrt();let mS=(H*mL);let mT=(b+mR);let mU=(mS/mT);let mX=(if (mU<sf[200]){b}else{d});let mY=(if (mX!=0.0){sf[200]}else{mU});let n0=(b+mO);let n1=(n0/mT);let n4=(sf[380]*((mO-mR)-(n1).ln()));let n6=((kE+n4)/sf[564]);let n8=(if (n6>d){b}else{d});let n9=100.0;let nb=(if (kr<n9){b}else{d});let nc=((n8!=0.0)&&(nb!=0.0));let nf=((n8!=0.0)&&(!(nb!=0.0)));let nh=(b+(kr-n9));let nn=(sf[564]*(fP*n6));let np=(b+(sf[381]*nn));let nu=(if (n8!=0.0){((sf[469]+(sf[795]*(np).ln()))-(if nf{(n9+(nh).ln())}else{(if nc{kr}else{d})}))}else{d});let nx=(if (n8!=0.0){sf[796]}else{d});let nz=(if (n8!=0.0){(nx*nx)}else{fL});let nD=(if (nu<d){b}else{d});let nE=((n8!=0.0)&&(nD!=0.0));let nF=(fP*nz);let nH=((nz+(if (n8!=0.0){(nu*nu)}else{sf[616]}))).sqrt();let nI=(nH-nu);let nM=((n8!=0.0)&&(!(nD!=0.0)));let nP=(if nM{(fP*(nu+nH))}else{(if nE{(nF/nI)}else{d})});let nT=(nP+sf[203]);let nU=(nP*nT);let nX=(sf[202]*(nP+sf[797]));let nZ=(if (n8!=0.0){(nU/nX)}else{d});let o1=(if (n8!=0.0){(n6/nZ)}else{d});let o5=(if (n8!=0.0){((o1-b)/sf[204])}else{sf[595]});let o7=(if (o1<b){b}else{d});let o8=((n8!=0.0)&&(o7!=0.0));let o9=(o5).exp();let oa=(b+o9);let og=((n8!=0.0)&&(!(o7!=0.0)));let oi=((-o5)).exp();let oj=(b+oi);let ow=(if (n8!=0.0){((if og{(o1+(sf[204]*(oj).ln()))}else{(if o8{(b+(sf[204]*(oa).ln()))}else{d})})/sf[210])}else{d});let oy=(if (n8!=0.0){(nP/sf[203])}else{d});let oz=(g1*ow);let oA=(oy*oz);let oB=(b+oy);let oE=((b+(oA*oB))).sqrt();let oF=(b+oE);let oG=(H*ow);let oH=(oB*oG);let oJ=(if (n8!=0.0){(oF/oH)}else{d});let oL=(mY*oJ);let oM=((b-oJ)+oL);let oN=(b+oL);let oP=(if (n8!=0.0){(oM/oN)}else{d});let oS=(if (n8!=0.0){(sf[381]*(nn*oP))}else{d});let oV=(b+(mY+oS));let oY=(if (n8!=0.0){((H*oS)+(mY*oV))}else{d});let p1=(if (n8!=0.0){(fP*(oS-b))}else{d});let p4=(if (n8!=0.0){(oY+(p1*p1))}else{d});let p6=(if (oS>=b){b}else{d});
        let p7=((n8!=0.0)&&(p6!=0.0));let p8=(p4).sqrt();let pc=((n8!=0.0)&&(!(p6!=0.0)));let pd=(p8-p1);let pf=(if pc{(oY/pd)}else{(if p7{(p1+p8)}else{d})});let pj=((n8!=0.0)&&((if (pf<sf[211]){b}else{d})!=0.0));let pk=(if pj{sf[211]}else{pf});let pl=(b+pk);let pu=(if (n8!=0.0){(sf[212]*(n6-sf[201]))}else{d});let pB=(((if (n8!=0.0){(n6*sf[801])}else{d})+(pu*pu))).sqrt();let pL=((n8!=0.0)&&sb[20]);let pM=(H*n6);let pN=(n6+nZ);let pS=(n6*sf[201]);let pT=(n6+sf[201]);let pY=(!(n8!=0.0));let pZ=(H*mz);let q2=(if pY{(if la{(lc*(b+(l4-sf[198])))}else{(if (l7!=0.0){l8}else{d})})}else{(if (n8!=0.0){((pk*pl)*sf[799])}else{d})});let qe=(if (((kE).abs()<sf[803])||((n4).abs()<(sf[804]*(mO+mR)))){b}else{d});let qf=(pY&&(qe!=0.0));let qg=(mY+(if pY{(pZ/n0)}else{pk}));let qi=(if qf{(fP*qg)}else{d});let qj=(b+qi);let qn=(pY&&(!(qe!=0.0)));let qp=((ku+n4)-kr);let qr=(if qn{(n4/qp)}else{(if qf{(qi/qj)}else{oP})});let qt=(if pY{sf[802]}else{(if pL{(sf[507]*(W+(pM/pN)))}else{(if ((n8!=0.0)&&(sf[214]!=0.0)){sf[802]}else{d})})});let qu=(if pY{n6}else{(if (n8!=0.0){(pS/pT)}else{d})});let qx=(if pY{(b-(qu/sf[201]))}else{(if (n8!=0.0){(sf[201]/pT)}else{d})});let qE=((kx-sf[805])/sf[806]);let qG=(if (kx<sf[805]){b}else{d});let qH=(qE).exp();let qI=(b+qH);let qN=(!(qG!=0.0));let qP=((-qE)).exp();let qQ=(b+qP);let qU=(if qN{(sf[805]-(sf[806]*(qQ).ln()))}else{(if (qG!=0.0){(kx-(sf[806]*(qI).ln()))}else{d})});let qW=(b-(sf[528]*qU));let qY=f64::powf(qW,sf[218]);let r4=((sf[807]*(b-qY))+(bK*(kx-qU)));let rh=(if sb[26]{ku}else{(if sb[24]{(kr+(if pY{kE}else{(if (n8!=0.0){(pu+pB)}else{d})}))}else{(if (sf[220]!=0.0){kr}else{d})})});let rp=(rh-sf[813]);let rq=(rp/qt);let rs=(if (rh<sf[813]){b}else{d});let rt=(rq).exp();let ru=(b+rt);let rv=(ru).ln();let rz=(!(rs!=0.0));let rB=((-rq)).exp();let rC=(b+rB);let rD=(rC).ln();let rG=(if rz{(sf[813]-(qt*rD))}else{(if (rs!=0.0){(rh-(qt*rv))}else{d})});let rI=f64::powf(qx,sf[223]);let rM=(b-(rG/sf[507]));let rN=f64::powf(rM,sf[224]);let rR=(sf[810]*rI);let rS=(rh-rG);let rX=((sf[809]*((sf[814]*(b-(rI*rN)))+(rR*rS)))+(sf[541]*kr));let s0=(ls*sf[816]);let s2=((b+s0)).sqrt();let s3=(b+s2);let s4=(s0/s3);let s6=f64::powf(q2,sf[817]);let s7=(sf[816]*s6);let s9=((b+s7)).sqrt();let sa=(b+s9);let sb_=(s7/sa);let sf_=(b+(r4/sf[750]));let sg=(rX/sf[748]);let sh=(sf_+sg);let ss=((if sb[28]{(sf[381]*(sf[779]*sf_))}else{d})).exp();let st=((if sb[28]{(sf[381]*(sf[779]*((-rX)/sf[748])))}else{d})).exp();let sz=(if sb[28]{((ss-st)/sf[820])}else{(if (sf[225]!=0.0){sh}else{d})});let sA=0.010000000000000002;let sB=(sz*sz);let sD=(if (sz<d){b}else{d});let sE=0.005000000000000001;let sG=((sA+sB)).sqrt();let sH=(sG-sz);let sK=(!(sD!=0.0));let sN=(if sK{(fP*(sz+sG))}else{(if (sD!=0.0){(sE/sH)}else{d})});let sQ=(b+(fP*(s4+sb_)));let sR=(sN*sQ);let sU=(s6*sf[821]);let sV=(sf[633]*ls);let sW=(sV-sU);let sX=(sW/sR);let sY=0.0001;let sZ=(kx/sY);let t0=(kx<d);let t1=(if t0{b}else{d});let t2=(sZ).exp();let t3=(b+t2);let t7=(!(t1!=0.0));let t9=((-sZ)).exp();let ta=(b+t9);let te=(if t7{(kx+(sY*(ta).ln()))}else{(if (t1!=0.0){(sY*(t3).ln())}else{d})});let tg=(te/sf[227]);let ti=(if (tg<sf[198]){b}else{d});let tl=(!(ti!=0.0));let tm=(if tl{sf[199]}else{mH});let tv=((kx-sf[228])/G);let tR=(lh/sf[143]);let tT=(if (tR<sf[198]){b}else{d});let tU=(tR).exp();let tW=(!(tT!=0.0));let tX=(if tW{sf[199]}else{tm});let u1=(if tW{(tX*(b+(tR-sf[198])))}else{(if (tT!=0.0){tU}else{te})});let u3=(sf[381]*(kx-sf[527]));let u5=(if (u3<sf[198]){b}else{d});let ua=((sf[149]!=0.0)&&(!(u5!=0.0)));let ub=(if ua{sf[199]}else{tX});let ui=((sX/sf[633])-1000.0);let uj=40.0;let ul=(if (ui<uj){b}else{d});let uq=((sf[149]!=0.0)&&(!(ul!=0.0)));let us=(if uq{2.3538526683702e17}else{ub});let v7=(sf[381]*kA);let v8=(v7/sf[147]);let va=(if (v8<sf[198]){b}else{d});let vb=(v8).exp();let vd=(!(va!=0.0));let ve=(if vd{sf[199]}else{us});let vi=(if vd{(ve*(b+(v8-sf[198])))}else{(if (va!=0.0){vb}else{u1})});let vk=(sf[381]*(kA-sf[527]));let vm=(if (vk<sf[198]){b}else{d});let vr=((sf[149]!=0.0)&&(!(vm!=0.0)));let vs=(if vr{sf[199]}else{ve});let vJ=(lh/sf[130]);
        let vL=(if (vJ<sf[198]){b}else{d});let vM=(vJ).exp();let vO=(!(vL!=0.0));let vP=(if vO{sf[199]}else{vs});let vT=(if vO{(vP*(b+(vJ-sf[198])))}else{(if (vL!=0.0){vM}else{vi})});let vW=(v7/sf[165]);let vY=(if (vW<sf[198]){b}else{d});let vZ=(vW).exp();let w1=(!(vY!=0.0));let w2=(if w1{sf[199]}else{vP});let w6=(if w1{(w2*(b+(vW-sf[198])))}else{(if (vY!=0.0){vZ}else{vT})});let w9=(lt/sf[136]);let wb=(if (w9<sf[198]){b}else{d});let wc=(w9).exp();let we=(!(wb!=0.0));let wf=(if we{sf[199]}else{w2});let wj=(if we{(wf*(b+(w9-sf[198])))}else{(if (wb!=0.0){wc}else{w6})});let wm=(v7/sf[169]);let wo=(if (wm<sf[198]){b}else{d});let wp=(wm).exp();let wr=(!(wo!=0.0));let ws=(if wr{sf[199]}else{wf});let ww=(if wr{(ws*(b+(wm-sf[198])))}else{(if (wo!=0.0){wp}else{wj})});let wD=(if (t0&&sb[36]){b}else{d});let wE=(H*qY);let wH=(sf[715]*(b-(sf[20]/wE)));let wJ=(if (wH<sf[198]){b}else{d});let wO=((wD!=0.0)&&(!(wJ!=0.0)));let wP=(if wO{sf[199]}else{ws});let wV=(if (wD!=0.0){(sf[528]*kx)}else{sf[746]});let wX=1e-30;let wZ=(((wV*wV)+wX)).sqrt();let x2=f64::powf(wZ,sf[233]);let xa=(gr*wV);let xb=(wV*xa);let xc=(wV+sf[236]);let xe=((sf[18]*(sf[235]-((bK*wV)*sf[236])))-(xb*xc));let xg=0.16666666666666666;let xk=(sf[715]*(sf[20]*kx));let xl=(sf[405]*(if (wD!=0.0){((x2*xe)*xg)}else{d}));let xn=(if (wD!=0.0){(xk/xl)}else{wV});let xo=-0.001;let xq=(if (xn<xo){b}else{d});let xs=(if (xn<sf[198]){b}else{d});let xt=((wD!=0.0)&&(xq!=0.0));let xy=(xt&&(!(xs!=0.0)));let xz=(if xy{sf[199]}else{wP});let yb=(if (sb[39]&&(kr<d)){b}else{d});let yc=(sf[529]*kr);let yd=(b-yc);let yf=(if (yb!=0.0){f64::powf(yd,sf[224])}else{d});let yg=(H*yf);let yj=(sf[735]*(b-(sf[52]/yg)));let yl=(if (yj<sf[198]){b}else{d});let yq=((yb!=0.0)&&(!(yl!=0.0)));let yr=(if yq{sf[199]}else{xz});let yw=(if (yb!=0.0){yc}else{sf[726]});let yz=((wX+(yw*yw))).sqrt();let yB=f64::powf(yz,sf[237]);let yJ=(gr*yw);let yK=(yw*yJ);let yL=(yw+sf[240]);let yN=((sf[50]*(sf[239]-((bK*yw)*sf[240])))-(yK*yL));let yS=(sf[735]*(sf[52]*kr));let yT=(sf[426]*(if (yb!=0.0){(xg*(yB*yN))}else{d}));let yV=(if (yb!=0.0){(yS/yT)}else{yw});let yX=(if (yV<xo){b}else{d});let yZ=(if (yV<sf[198]){b}else{d});let z0=((yb!=0.0)&&(yX!=0.0));let z5=(z0&&(!(yZ!=0.0)));let z6=(if z5{sf[199]}else{yr});let zB=(lD*sf[816]);let zC=(g1*(if mi{(mj*(b+(md-sf[198])))}else{(if (mf!=0.0){mg}else{d})}));let zD=(zB-sf[816]);let zF=((b+zB)).sqrt();let zG=(b+zF);let zJ=((b+zC)).sqrt();let zK=(b+zJ);let A6=(sf[829]*(lZ-b));let A9=((b+(lZ*sf[828]))).sqrt();let Aa=(b+A9);let Ac=(if (sf[242]!=0.0){(A6/Aa)}else{d});let Ap=(if sb[44]{(l3-sf[837])}else{d});let At=(if sb[44]{(Ap*Ap)}else{sB});let Av=(if (Ap<d){b}else{d});let Aw=(sb[44]&&(Av!=0.0));let Az=((sf[245]+At)).sqrt();let AA=(Az-Ap);let AE=(sb[44]&&(!(Av!=0.0)));let AH=(if AE{(fP*(Ap+Az))}else{(if Aw{(sf[246]/AA)}else{d})});let AK=(AH+(sf[832]+(sf[557]*Ac)));let AP=(if sb[46]{b}else{(if sb[44]{(AH/AK)}else{b})});let BQ=(if (sh<d){b}else{d});let BS=((sA+(sh*sh))).sqrt();let BT=(BS-sh);let BW=(!(BQ!=0.0));let BZ=(if BW{(fP*(sh+BS))}else{(if (BQ!=0.0){(sE/BT)}else{d})});let Cb=(if (sX>d){b}else{d});let Ch=(if (kr<sf[268]){b}else{d});let Ck=((-sX)/sf[269]);let Cm=(if (Ck<sf[198]){b}else{d});let Co=((Ch!=0.0)&&((Cb!=0.0)&&(sf[267]!=0.0)));let Cp=((Cm!=0.0)&&Co);let Cq=(Ck).exp();let Ct=(Co&&(!(Cm!=0.0)));let Cu=(if Ct{sf[199]}else{z6});let Cy=(if Ct{(Cu*(b+(Ck-sf[198])))}else{(if Cp{Cq}else{d})});let Cz=(sf[268]-kr);let CB=(if Co{(Cy*Cz)}else{d});let CF=(sf[838]*f64::powf(CB,sf[270]));let CH=(if (CF<sf[198]){b}else{d});let CM=(Co&&(!(CH!=0.0)));let CN=(if CM{sf[199]}else{Cu});let D2=((Cb!=0.0)&&sb[51]);let EL=((Ch!=0.0)&&((sf[285]!=0.0)&&(D2&&sb[55])));let EM=f64::powf(Cz,sf[270]);let EO=(sX+sf[286]);let EQ=(b-(sX/EO));let ES=f64::powf(EQ,sf[287]);let EU=(if EL{(EM*ES)}else{d});let EV=((sf[279]!=0.0)&&EL);let EX=(sb[53]&&EL);let F1=(if EX{((sX-sf[288])/sf[286])}else{d});let F5=(if EX{((F1-b)/sf[289])}else{tv});let F7=(if (F1<b){b}else{d});let F8=(EX&&(F7!=0.0));let F9=(F5).exp();let Fa=(b+F9);let Fg=(EX&&(!(F7!=0.0)));let Fi=((-F5)).exp();let Fj=(b+Fi);
        let Fn=(if Fg{(F1+(sf[289]*(Fj).ln()))}else{(if F8{(b+(sf[289]*(Fa).ln()))}else{d})});let Fp=f64::powf(Fn,sf[290]);let Fs=(sf[838]*(if EX{(EU*Fp)}else{(if EV{EU}else{d})}));let Fu=(if (Fs<sf[198]){b}else{d});let Fz=(EL&&(!(Fu!=0.0)));let FA=(if Fz{sf[199]}else{CN});let GA=((kA-sf[805])/sf[806]);let GC=(if (kA<sf[805]){b}else{d});let GD=(GA).exp();let GE=(b+GD);let GJ=(!(GC!=0.0));let GL=((-GA)).exp();let GM=(b+GL);let GQ=(if GJ{(sf[805]-(sf[806]*(GM).ln()))}else{(if (GC!=0.0){(kA-(sf[806]*(GE).ln()))}else{d})});let GT=(b-(sf[528]*GQ));let H6=(s4*sf[846]);let H7=(BZ*H6);let H8=(sb_*sf[846]);let H9=(BZ*H8);let Hb=((kY-sf[813])/sf[802]);let Hd=(if (kY<sf[813]){b}else{d});let He=(Hb).exp();let Hf=(b+He);let Hk=(!(Hd!=0.0));let Hm=((-Hb)).exp();let Hn=(b+Hm);let Hr=(if Hk{(sf[813]-(sf[802]*(Hn).ln()))}else{(if (Hd!=0.0){(kY-(sf[802]*(Hf).ln()))}else{d})});let Ht=(b-(Hr/sf[507]));let HI=((l3-sf[813])/sf[802]);let HK=(if (l3<sf[813]){b}else{d});let HL=(HI).exp();let HM=(b+HL);let HR=(!(HK!=0.0));let HT=((-HI)).exp();let HU=(b+HT);let HY=(if HR{(sf[813]-(sf[802]*(HU).ln()))}else{(if (HK!=0.0){(l3-(sf[802]*(HM).ln()))}else{d})});let I0=(b-(HY/sf[507]));let Ik=(kx/sf[851]);let Im=(if (Ik<sf[198]){b}else{d});let In=(Ik).exp();let Ip=(!(Im!=0.0));let Iq=(if Ip{sf[199]}else{FA});let Iv=(sf[850]*(if Ip{(Iq*(b+(Ik-sf[198])))}else{(if (Im!=0.0){In}else{ww})}));let IA=(qr*sf[855]);let IB=(H+qg);let IQ=(sf[381]*((kY-sf[488])/sf[301]));let IS=(if (IQ<sf[198]){b}else{d});let IU=((IS!=0.0)&&sb[60]);let IV=(IQ).exp();let IY=(sb[60]&&(!(IS!=0.0)));let IZ=(if IY{sf[199]}else{Iq});let J5=(lD*sf[857]);let J8=((b+(g1*(if IY{(IZ*(b+(IQ-sf[198])))}else{(if IU{IV}else{d})})))).sqrt();let J9=(b+J8);let Jb=(if sb[60]{(J5/J9)}else{(if (sf[300]!=0.0){((sf[856]*(((zD/zG)*sf[845])+((zC/zK)*sf[854])))/sf[763])}else{d})});let Jk=(if sb[64]{(lZ*sf[816])}else{d});let Jl=(Jk-sf[816]);let Jn=((b+Jk)).sqrt();let Jo=(b+Jn);let Js=(if sb[64]{(g1*(if m6{(m7*(b+(m1-sf[198])))}else{(if (m3!=0.0){m4}else{d})}))}else{d});let Ju=((b+Js)).sqrt();let Jv=(b+Ju);let JH=(sf[381]*(l3-sf[488]));let JJ=(if (JH<sf[198]){b}else{d});let JL=((JJ!=0.0)&&sb[65]);let JM=(JH).exp();let JP=(sb[65]&&(!(JJ!=0.0)));let JQ=(if JP{sf[199]}else{IZ});let JW=(lZ*sf[859]);let JZ=((b+(g1*(if JP{(JQ*(b+(JH-sf[198])))}else{(if JL{JM}else{d})})))).sqrt();let K0=(b+JZ);let K2=(if sb[65]{(JW/K0)}else{(if sb[64]{((sf[858]*((sf[845]*(if sb[64]{(Jl/Jo)}else{d}))+(sf[854]*(if sb[64]{(Js/Jv)}else{d}))))/sf[763])}else{d})});let Kb=(if (sf[305]!=0.0){(f64::powf(qW,sf[306])-bK)}else{d});let Kc=(if (sf[305]!=0.0){qE}else{d});let Ke=(if (Kc<d){b}else{d});let Kf=((sf[305]!=0.0)&&(Ke!=0.0));let Kg=(Kc).exp();let Kh=(b+Kg);let Kl=((sf[305]!=0.0)&&(!(Ke!=0.0)));let Kn=((-Kc)).exp();let Ko=(b+Kn);let Kq=(if Kl{(Kn/Ko)}else{(if Kf{(b/Kh)}else{d})});let Kx=((sf[381]*s0)/sf[588]);let Ky=(fP/s2);let KA=(if (sf[305]!=0.0){(Kx*Ky)}else{d});let KB=(BZ*sf[846]);let KG=(kC*0.2);let KI=((if (sf[305]!=0.0){(Iv/sf[851])}else{d})+((if (sf[305]!=0.0){(sf[842]*(if (sf[305]!=0.0){(bK+(Kb*Kq))}else{d}))}else{d})+(if (sf[305]!=0.0){(KA*KB)}else{d})));let KR=(if (sf[305]!=0.0){(H7+(Iv*sf[307]))}else{d});let L0=(if sb[67]{H7}else{(if (sf[305]!=0.0){(KR*sf[310])}else{d})});let L1=(if sb[67]{H9}else{(if (sf[305]!=0.0){(H9+(KR*sf[309]))}else{d})});let L3=(sU+sV);let L4=(L3/sR);let Le=(if (L4>d){b}else{d});let Lf=(L0+L1);let Li=(!(Le!=0.0));let Lj=(sf[759]*BZ);let Ll=(if Li{(sR*Lj)}else{(if (Le!=0.0){(Lf/L4)}else{d})});let LA=(if sb[75]{d}else{(if sb[73]{(Ll*sf[316])}else{(if (sf[314]!=0.0){(sf[309]*Ll)}else{d})})});let Mh=(sf[0]*((if sb[67]{Iv}else{(if (sf[305]!=0.0){(Iv*sf[308])}else{d})})+((r4*sf[842])+L0)));let Mk=(sf[0]*(sf[843]*((sf[807]*(b-f64::powf(GT,sf[218])))+(bK*(kA-GQ)))));let Mn=(sf[0]*((IA*IB)+((rX*sf[844])+L1)));let Mq=(sf[0]*(if (sf[305]!=0.0){(KG*KI)}else{d}));let Mu=((sf[0]*(kI-kF))*sf[319]);let My=(kP*sf[320]);let MG=(sf[0]*((sf[6]*(sf[296]*(sf[540]*((sf[809]*((sf[814]*(b-f64::powf(I0,sf[224])))+(sf[810]*(l3-HY))))+(sf[541]*l3)))))+(if (sf[302]!=0.0){(AP*K2)}else{d})));
        let MM=(sf[0]*((sf[7]*((sf[540]*((sf[809]*((sf[814]*(b-f64::powf(Ht,sf[224])))+(sf[810]*(kY-Hr))))+(sf[541]*kY)))*sf[296]))+(if (sf[302]!=0.0){(sf[7]*Jb)}else{Jb})));let MX=ctx.node_voltage(n[10]);let Nn=(if ln{(lo*sf[862])}else{(if (lk!=0.0){(ll*sf[862])}else{d})});let No=(if ln{(lo*sf[863])}else{(if (lk!=0.0){(ll*sf[863])}else{d})});let ND=(if ly{(lz*sf[860])}else{(if (lv!=0.0){(lw*sf[860])}else{d})});let NE=(if ly{(lz*sf[864])}else{(if (lv!=0.0){(lw*sf[864])}else{d})});let NF=(if ly{(lz*sf[865])}else{(if (lv!=0.0){(lw*sf[865])}else{d})});let NG=(if ly{(lz*sf[861])}else{(if (lv!=0.0){(lw*sf[861])}else{d})});let O2=(if lU{(lV*sf[864])}else{(if (lR!=0.0){(lS*sf[864])}else{d})});let O3=(if lU{(lV*sf[866])}else{(if (lR!=0.0){(lS*sf[866])}else{d})});let O4=(if lU{(lV*sf[865])}else{(if (lR!=0.0){(lS*sf[865])}else{d})});let O5=(if lU{(lV*sf[861])}else{(if (lR!=0.0){(lS*sf[861])}else{d})});let OI=(if mu{(mv*sf[860])}else{(if (mr!=0.0){(ms*sf[860])}else{d})});let OJ=(if mu{(mv*sf[861])}else{(if (mr!=0.0){(ms*sf[861])}else{d})});let OQ=(if mG{(mH*sf[860])}else{(if (mD!=0.0){(mE*sf[860])}else{d})});let OR=(if mG{(mH*sf[861])}else{(if (mD!=0.0){(mE*sf[861])}else{d})});let OU=(H*mO);let OV=((g1*OI)/OU);let OW=((g1*OJ)/OU);let OZ=(H*mR);let P0=((g1*OQ)/OZ);let P1=((g1*OR)/OZ);let P7=(mT*mT);let Pd=(if (mX!=0.0){d}else{(((mT*(H*OQ))-(mS*P0))/P7)});let Pe=(if (mX!=0.0){d}else{(((mT*(H*OR))-(mS*P1))/P7)});let Pv=(sf[380]*((OV-P0)-((((mT*OV)-(n0*P0))/P7)/n1)));let Pw=(sf[380]*((-P1)-(((-(n0*P1))/P7)/n1)));let Px=(sf[380]*(OW-((OW/mT)/n1)));let Pz=(sf[321]+Px);let PA=(Pv/sf[564]);let PB=((sf[0]+Pw)/sf[564]);let PC=(Pz/sf[564]);let PM=(sf[564]*(fP*PA));let PN=(sf[564]*(fP*PB));let PO=(sf[564]*(fP*PC));let Q0=(if (n8!=0.0){((sf[795]*((sf[381]*PM)/np))-(if nf{(sf[0]/nh)}else{(if nc{sf[0]}else{d})}))}else{d});let Q1=(if (n8!=0.0){((sf[795]*((sf[381]*PN)/np))-(if nf{(sf[321]/nh)}else{(if nc{sf[321]}else{d})}))}else{d});let Q2=(if (n8!=0.0){(sf[795]*((sf[381]*PO)/np))}else{d});let Q3=(nu*Q0);let Q5=(nu*Q1);let Q7=(nu*Q2);let Qc=(H*nH);let Qd=((if (n8!=0.0){(Q3+Q3)}else{d})/Qc);let Qe=((if (n8!=0.0){(Q5+Q5)}else{d})/Qc);let Qf=((if (n8!=0.0){(Q7+Q7)}else{d})/Qc);let Ql=(nI*nI);let QC=(if nM{(fP*(Q0+Qd))}else{(if nE{((-(nF*(Qd-Q0)))/Ql)}else{d})});let QD=(if nM{(fP*(Q1+Qe))}else{(if nE{((-(nF*(Qe-Q1)))/Ql)}else{d})});let QE=(if nM{(fP*(Q2+Qf))}else{(if nE{((-(nF*(Qf-Q2)))/Ql)}else{d})});let QU=(nX*nX);let R4=(if (n8!=0.0){(((nX*((nT*QC)+(nP*QC)))-(nU*(sf[202]*QC)))/QU)}else{d});let R5=(if (n8!=0.0){(((nX*((nT*QD)+(nP*QD)))-(nU*(sf[202]*QD)))/QU)}else{d});let R6=(if (n8!=0.0){(((nX*((nT*QE)+(nP*QE)))-(nU*(sf[202]*QE)))/QU)}else{d});let Ra=(nZ*nZ);let Rk=(if (n8!=0.0){(((nZ*PA)-(n6*R4))/Ra)}else{d});let Rl=(if (n8!=0.0){(((nZ*PB)-(n6*R5))/Ra)}else{d});let Rm=(if (n8!=0.0){(((nZ*PC)-(n6*R6))/Ra)}else{d});let Rq=(if (n8!=0.0){(Rk/sf[204])}else{d});let Rr=(if (n8!=0.0){(Rl/sf[204])}else{d});let Rs=(if (n8!=0.0){(Rm/sf[204])}else{d});let S0=(if (n8!=0.0){((if og{(Rk+(sf[204]*((oi*(-Rq))/oj)))}else{(if o8{(sf[204]*((o9*Rq)/oa))}else{d})})/sf[210])}else{d});let S1=(if (n8!=0.0){((if og{(Rl+(sf[204]*((oi*(-Rr))/oj)))}else{(if o8{(sf[204]*((o9*Rr)/oa))}else{d})})/sf[210])}else{d});let S2=(if (n8!=0.0){((if og{(Rm+(sf[204]*((oi*(-Rs))/oj)))}else{(if o8{(sf[204]*((o9*Rs)/oa))}else{d})})/sf[210])}else{d});let S6=(if (n8!=0.0){(QC/sf[203])}else{d});let S7=(if (n8!=0.0){(QD/sf[203])}else{d});let S8=(if (n8!=0.0){(QE/sf[203])}else{d});let Su=(H*oE);let SN=(oH*oH);let SX=(if (n8!=0.0){(((oH*(((oB*((oz*S6)+(oy*(g1*S0))))+(oA*S6))/Su))-(oF*((oG*S6)+(oB*(H*S0)))))/SN)}else{d});let SY=(if (n8!=0.0){(((oH*(((oB*((oz*S7)+(oy*(g1*S1))))+(oA*S7))/Su))-(oF*((oG*S7)+(oB*(H*S1)))))/SN)}else{d});let SZ=(if (n8!=0.0){(((oH*(((oB*((oz*S8)+(oy*(g1*S2))))+(oA*S8))/Su))-(oF*((oG*S8)+(oB*(H*S2)))))/SN)}else{d});let T5=((oJ*Pd)+(mY*SX));let T8=((oJ*Pe)+(mY*SY));let T9=(mY*SZ);let Tg=(oN*oN);let Tq=(if (n8!=0.0){(((oN*((-SX)+T5))-(oM*T5))/Tg)}else{d});let Tr=(if (n8!=0.0){(((oN*((-SY)+T8))-(oM*T8))/Tg)}else{d});let Ts=(if (n8!=0.0){(((oN*((-SZ)+T9))-(oM*T9))/Tg)}else{d});
        let TF=(if (n8!=0.0){(sf[381]*((oP*PM)+(nn*Tq)))}else{d});let TG=(if (n8!=0.0){(sf[381]*((oP*PN)+(nn*Tr)))}else{d});let TH=(if (n8!=0.0){(sf[381]*((oP*PO)+(nn*Ts)))}else{d});let TX=(if (n8!=0.0){((H*TF)+((oV*Pd)+(mY*(Pd+TF))))}else{d});let TY=(if (n8!=0.0){((H*TG)+((oV*Pe)+(mY*(Pe+TG))))}else{d});let TZ=(if (n8!=0.0){((H*TH)+(mY*TH))}else{d});let U3=(if (n8!=0.0){(fP*TF)}else{d});let U4=(if (n8!=0.0){(fP*TG)}else{d});let U5=(if (n8!=0.0){(fP*TH)}else{d});let U6=(p1*U3);let U8=(p1*U4);let Ua=(p1*U5);let Uf=(if (n8!=0.0){(TX+(U6+U6))}else{d});let Ug=(if (n8!=0.0){(TY+(U8+U8))}else{d});let Uh=(if (n8!=0.0){(TZ+(Ua+Ua))}else{d});let Ui=(H*p8);let Uj=(Uf/Ui);let Uk=(Ug/Ui);let Ul=(Uh/Ui);let Uy=(pd*pd);let UL=(if pj{d}else{(if pc{(((pd*TX)-(oY*(Uj-U3)))/Uy)}else{(if p7{(U3+Uj)}else{d})})});let UM=(if pj{d}else{(if pc{(((pd*TY)-(oY*(Uk-U4)))/Uy)}else{(if p7{(U4+Uk)}else{d})})});let UN=(if pj{d}else{(if pc{(((pd*TZ)-(oY*(Ul-U5)))/Uy)}else{(if p7{(U5+Ul)}else{d})})});let V6=(if (n8!=0.0){(sf[212]*PA)}else{d});let V7=(if (n8!=0.0){(sf[212]*PB)}else{d});let V8=(if (n8!=0.0){(sf[212]*PC)}else{d});let Vf=(pu*V6);let Vh=(pu*V7);let Vj=(pu*V8);let Vo=(H*pB);let VH=(pN*pN);let VX=(sf[201]*PA);let VY=(sf[201]*PB);let VZ=(sf[201]*PC);let W3=(pT*pT);let Wu=(n0*n0);let WC=(if pY{(((n0*(H*OJ))-(pZ*OW))/Wu)}else{UN});let WD=(if pY{(if la{(lc*sf[860])}else{(if (l7!=0.0){(l8*sf[860])}else{d})})}else{(if (n8!=0.0){(sf[799]*((pl*UL)+(pk*UL)))}else{d})});let WE=(if pY{d}else{(if (n8!=0.0){(sf[799]*((pl*UM)+(pk*UM)))}else{d})});let WF=(if pY{(if la{(lc*sf[861])}else{(if (l7!=0.0){(l8*sf[861])}else{d})})}else{(if (n8!=0.0){(sf[799]*((pl*UN)+(pk*UN)))}else{d})});let WG=(Pd+(if pY{(((n0*(H*OI))-(pZ*OV))/Wu)}else{UL}));let WH=(Pe+(if pY{d}else{UM}));let WL=(if qf{(fP*WG)}else{d});let WM=(if qf{(fP*WH)}else{d});let WN=(if qf{(fP*WC)}else{d});let WR=(qj*qj);let Xa=(qp*qp);let Xk=(if qn{(((qp*Pv)-(n4*((sf[0]+Pv)-sf[0])))/Xa)}else{(if qf{(((qj*WL)-(qi*WL))/WR)}else{Tq})});let Xl=(if qn{(((qp*Pw)-(n4*(Pw-sf[321])))/Xa)}else{(if qf{(((qj*WM)-(qi*WM))/WR)}else{Tr})});let Xm=(if qn{(((qp*Px)-(n4*Pz))/Xa)}else{(if qf{(((qj*WN)-(qi*WN))/WR)}else{Ts})});let Xq=(if pY{d}else{(if pL{(sf[507]*(((pN*(H*PA))-(pM*(PA+R4)))/VH))}else{d})});let Xr=(if pY{d}else{(if pL{(sf[507]*(((pN*(H*PB))-(pM*(PB+R5)))/VH))}else{d})});let Xs=(if pY{d}else{(if pL{(sf[507]*(((pN*(H*PC))-(pM*(PC+R6)))/VH))}else{d})});let Xt=(if pY{PA}else{(if (n8!=0.0){(((pT*VX)-(pS*PA))/W3)}else{d})});let Xu=(if pY{PB}else{(if (n8!=0.0){(((pT*VY)-(pS*PB))/W3)}else{d})});let Xv=(if pY{PC}else{(if (n8!=0.0){(((pT*VZ)-(pS*PC))/W3)}else{d})});let XC=(if pY{(-(Xt/sf[201]))}else{(if (n8!=0.0){((-VX)/W3)}else{d})});let XD=(if pY{(-(Xu/sf[201]))}else{(if (n8!=0.0){((-VY)/W3)}else{d})});let XE=(if pY{(-(Xv/sf[201]))}else{(if (n8!=0.0){((-VZ)/W3)}else{d})});let Y1=(if qN{(-(sf[806]*((qP*sf[869])/qQ)))}else{(if (qG!=0.0){(sf[321]-(sf[806]*((qH*sf[867])/qI)))}else{d})});let Y2=(if qN{(-(sf[806]*((qP*sf[870])/qQ)))}else{(if (qG!=0.0){(sf[0]-(sf[806]*((qH*sf[868])/qI)))}else{d})});let Y5=(-(sf[528]*Y1));let Y6=(-(sf[528]*Y2));let Y9=(sf[218]*f64::powf(qW,sf[325]));let Ya=(Y5*Y9);let Yb=(Y6*Y9);let Yk=((sf[807]*(-Ya))+(bK*(sf[321]-Y1)));let Yl=((sf[807]*(-Yb))+(bK*(sf[0]-Y2)));let Yt=(if sb[26]{sf[0]}else{(if sb[24]{(sf[0]+(if pY{d}else{(if (n8!=0.0){(V6+(((if (n8!=0.0){(sf[801]*PA)}else{d})+(Vf+Vf))/Vo))}else{d})}))}else{sf[326]})});let Yu=(if sb[26]{d}else{(if sb[24]{(sf[321]+(if pY{sf[0]}else{(if (n8!=0.0){(V7+(((if (n8!=0.0){(sf[801]*PB)}else{d})+(Vh+Vh))/Vo))}else{d})}))}else{sf[327]})});let Yv=(if sb[26]{sf[321]}else{(if sb[24]{(if pY{sf[321]}else{(if (n8!=0.0){(V8+(((if (n8!=0.0){(sf[801]*PC)}else{d})+(Vj+Vj))/Vo))}else{d})})}else{d})});let Yz=(qt*qt);let YA=(((qt*Yt)-(rp*Xq))/Yz);let YE=(((qt*Yu)-(rp*Xr))/Yz);let YI=(((qt*Yv)-(rp*Xs))/Yz);let Zp=(if rz{(-((rD*Xq)+(qt*((rB*(-YA))/rC))))}else{(if (rs!=0.0){(Yt-((rv*Xq)+(qt*((rt*YA)/ru))))}else{d})});let Zq=(if rz{(-((rD*Xr)+(qt*((rB*(-YE))/rC))))}else{(if (rs!=0.0){(Yu-((rv*Xr)+(qt*((rt*YE)/ru))))}else{d})});
        let Zr=(if rz{(-((rD*Xs)+(qt*((rB*(-YI))/rC))))}else{(if (rs!=0.0){(Yv-((rv*Xs)+(qt*((rt*YI)/ru))))}else{d})});let Zu=(sf[223]*f64::powf(qx,sf[328]));let Zv=(XC*Zu);let Zw=(XD*Zu);let Zx=(XE*Zu);let ZG=(sf[224]*f64::powf(rM,sf[329]));let a0j=(sf[809]*((sf[814]*(-((rN*Zx)+(rI*((-(Zr/sf[507]))*ZG)))))+((rS*(sf[810]*Zx))+(rR*(Yv-Zr)))));let a0m=((sf[809]*((sf[814]*(-((rN*Zv)+(rI*((-(Zp/sf[507]))*ZG)))))+((rS*(sf[810]*Zv))+(rR*(Yt-Zp)))))+sf[871]);let a0n=((sf[809]*((sf[814]*(-((rN*Zw)+(rI*((-(Zq/sf[507]))*ZG)))))+((rS*(sf[810]*Zw))+(rR*(Yu-Zq)))))+sf[872]);let a0o=(sf[816]*Nn);let a0p=(sf[816]*No);let a0q=(H*s2);let a0r=(a0o/a0q);let a0s=(a0p/a0q);let a0w=(s3*s3);let a0x=(((s3*a0o)-(s0*a0r))/a0w);let a0B=(((s3*a0p)-(s0*a0s))/a0w);let a0E=(sf[817]*f64::powf(q2,sf[873]));let a0F=(WD*a0E);let a0G=(WE*a0E);let a0H=(WF*a0E);let a0I=(sf[816]*a0F);let a0J=(sf[816]*a0G);let a0K=(sf[816]*a0H);let a0L=(H*s9);let a0S=(sa*sa);let a0T=(((sa*a0I)-(s7*(a0I/a0L)))/a0S);let a0X=(((sa*a0J)-(s7*(a0J/a0L)))/a0S);let a11=(((sa*a0K)-(s7*(a0K/a0L)))/a0S);let a12=(Yk/sf[750]);let a13=(Yl/sf[750]);let a14=(a0m/sf[748]);let a15=(a0n/sf[748]);let a16=(a0j/sf[748]);let a17=(a13+a14);let a1J=(if sb[28]{((ss*(if sb[28]{(sf[381]*(sf[779]*a12))}else{d}))/sf[820])}else{(if (sf[225]!=0.0){a12}else{d})});let a1K=(if sb[28]{(((ss*(if sb[28]{(sf[381]*(sf[779]*a13))}else{d}))-(st*(if sb[28]{(sf[381]*(sf[779]*((-a0m)/sf[748])))}else{d})))/sf[820])}else{(if (sf[225]!=0.0){a17}else{d})});let a1L=(if sb[28]{((-(st*(if sb[28]{(sf[381]*(sf[779]*((-a0n)/sf[748])))}else{d})))/sf[820])}else{(if (sf[225]!=0.0){a15}else{d})});let a1M=(if sb[28]{((-(st*(if sb[28]{(sf[381]*(sf[779]*((-a0j)/sf[748])))}else{d})))/sf[820])}else{(if (sf[225]!=0.0){a16}else{d})});let a1N=(sz*a1J);let a1O=(a1N+a1N);let a1P=(sz*a1K);let a1Q=(a1P+a1P);let a1R=(sz*a1L);let a1S=(a1R+a1R);let a1T=(sz*a1M);let a1U=(a1T+a1T);let a1V=(H*sG);let a1W=(a1O/a1V);let a1X=(a1Q/a1V);let a1Y=(a1S/a1V);let a1Z=(a1U/a1V);let a26=(sH*sH);let a2y=(fP*a0x);let a2z=(fP*(a0B+a0T));let a2A=(fP*a0X);let a2B=(fP*a11);let a2E=((sQ*(if sK{(fP*(a1J+a1W))}else{(if (sD!=0.0){((-(sE*(a1W-a1J)))/a26)}else{d})}))+(sN*a2y));let a2H=((sQ*(if sK{(fP*(a1K+a1X))}else{(if (sD!=0.0){((-(sE*(a1X-a1K)))/a26)}else{d})}))+(sN*a2z));let a2K=((sQ*(if sK{(fP*(a1L+a1Y))}else{(if (sD!=0.0){((-(sE*(a1Y-a1L)))/a26)}else{d})}))+(sN*a2A));let a2N=((sQ*(if sK{(fP*(a1M+a1Z))}else{(if (sD!=0.0){((-(sE*(a1Z-a1M)))/a26)}else{d})}))+(sN*a2B));let a2O=(sf[821]*a0F);let a2P=(sf[821]*a0G);let a2Q=(sf[821]*a0H);let a2S=(sf[633]*No);let a2W=(sR*(sf[633]*Nn));let a2Z=(sR*sR);let a3x=(if t7{(sf[321]+(sY*((t9*sf[332])/ta)))}else{(if (t1!=0.0){(sY*((t2*sf[330])/t3))}else{d})});let a3y=(if t7{(sf[0]+(sY*((t9*sf[333])/ta)))}else{(if (t1!=0.0){(sY*((t2*sf[331])/t3))}else{d})});let a4u=(if tW{(tX*sf[874])}else{(if (tT!=0.0){(tU*sf[874])}else{a3x})});let a4v=(if tW{(tX*sf[875])}else{(if (tT!=0.0){(tU*sf[875])}else{a3y})});let a6E=(if vd{(ve*sf[876])}else{(if (va!=0.0){(vb*sf[876])}else{a4u})});let a6F=(if vd{(ve*sf[877])}else{(if (va!=0.0){(vb*sf[877])}else{d})});let a6G=(if vd{d}else{(if (va!=0.0){d}else{a4v})});let a7z=(if vO{(vP*sf[878])}else{(if (vL!=0.0){(vM*sf[878])}else{a6E})});let a7A=(if vO{d}else{(if (vL!=0.0){d}else{a6F})});let a7B=(if vO{(vP*sf[879])}else{(if (vL!=0.0){(vM*sf[879])}else{a6G})});let a7O=(if w1{(w2*sf[880])}else{(if (vY!=0.0){(vZ*sf[880])}else{a7z})});let a7P=(if w1{(w2*sf[881])}else{(if (vY!=0.0){(vZ*sf[881])}else{a7A})});let a7Q=(if w1{d}else{(if (vY!=0.0){d}else{a7B})});let a8b=(if we{d}else{(if (wb!=0.0){d}else{a7O})});let a8c=(if we{(wf*sf[882])}else{(if (wb!=0.0){(wc*sf[882])}else{a7P})});let a8d=(if we{(wf*sf[883])}else{(if (wb!=0.0){(wc*sf[883])}else{a7Q})});let a8e=(if we{(wf*sf[884])}else{(if (wb!=0.0){(wc*sf[884])}else{d})});let a8f=(if we{(wf*sf[885])}else{(if (wb!=0.0){(wc*sf[885])}else{d})});let a8w=(if wr{(ws*sf[886])}else{(if (wo!=0.0){(wp*sf[886])}else{a8b})});let a8x=(if wr{(ws*sf[887])}else{(if (wo!=0.0){(wp*sf[887])}else{a8c})});let a8y=(if wr{d}else{(if (wo!=0.0){d}else{a8d})});
        let a8z=(if wr{d}else{(if (wo!=0.0){d}else{a8e})});let a8A=(if wr{d}else{(if (wo!=0.0){d}else{a8f})});let ae2=(sf[816]*ND);let ae3=(sf[816]*NE);let ae4=(sf[816]*NF);let ae5=(sf[816]*NG);let ae6=(g1*(if mi{(mj*sf[860])}else{(if (mf!=0.0){(mg*sf[860])}else{d})}));let ae7=(g1*(if mi{(mj*sf[864])}else{(if (mf!=0.0){(mg*sf[864])}else{d})}));let ae8=(g1*(if mi{(mj*sf[865])}else{(if (mf!=0.0){(mg*sf[865])}else{d})}));let ae9=(g1*(if mi{(mj*sf[861])}else{(if (mf!=0.0){(mg*sf[861])}else{d})}));let aea=(H*zF);let aei=(zG*zG);let aew=(H*zJ);let aeE=(zK*zK);let afC=(H*A9);let afK=(Aa*Aa);let afY=(if (sf[242]!=0.0){(((Aa*(sf[829]*O2))-(A6*((sf[828]*O2)/afC)))/afK)}else{d});let afZ=(if (sf[242]!=0.0){(((Aa*(sf[829]*O3))-(A6*((sf[828]*O3)/afC)))/afK)}else{d});let ag0=(if (sf[242]!=0.0){(((Aa*(sf[829]*O4))-(A6*((sf[828]*O4)/afC)))/afK)}else{d});let ag1=(if (sf[242]!=0.0){(((Aa*(sf[829]*O5))-(A6*((sf[828]*O5)/afC)))/afK)}else{d});let ag6=(Ap*sf[346]);let ag7=(ag6+ag6);let ag8=(Ap*sf[347]);let aga=(Ap*sf[348]);let agb=(aga+aga);let agc=(Ap*sf[349]);let age=(if sb[44]{ag7}else{d});let agf=(if sb[44]{(ag8+ag8)}else{d});let agg=(if sb[44]{d}else{a1O});let agh=(if sb[44]{ag7}else{a1Q});let agi=(if sb[44]{agb}else{a1S});let agj=(if sb[44]{agb}else{a1U});let agk=(if sb[44]{(agc+agc)}else{d});let agl=(if sb[44]{agb}else{d});let agm=(H*Az);let agn=(age/agm);let ago=(agf/agm);let agp=(agg/agm);let agq=(agh/agm);let agr=(agi/agm);let ags=(agj/agm);let agt=(agk/agm);let agu=(agl/agm);let agE=(AA*AA);let aho=(if AE{(fP*(sf[346]+agn))}else{(if Aw{((-(sf[246]*(agn-sf[346])))/agE)}else{d})});let ahp=(if AE{(fP*(sf[347]+ago))}else{(if Aw{((-(sf[246]*(ago-sf[347])))/agE)}else{d})});let ahq=(if AE{(fP*agp)}else{(if Aw{((-(sf[246]*agp))/agE)}else{d})});let ahr=(if AE{(fP*(sf[346]+agq))}else{(if Aw{((-(sf[246]*(agq-sf[346])))/agE)}else{d})});let ahs=(if AE{(fP*(sf[348]+agr))}else{(if Aw{((-(sf[246]*(agr-sf[348])))/agE)}else{d})});let aht=(if AE{(fP*(sf[348]+ags))}else{(if Aw{((-(sf[246]*(ags-sf[348])))/agE)}else{d})});let ahu=(if AE{(fP*(sf[349]+agt))}else{(if Aw{((-(sf[246]*(agt-sf[349])))/agE)}else{d})});let ahv=(if AE{(fP*(sf[348]+agu))}else{(if Aw{((-(sf[246]*(agu-sf[348])))/agE)}else{d})});let ahw=(sf[557]*afY);let ahy=(sf[557]*ag0);let ahK=(AK*AK);let aim=(if sb[46]{d}else{(if sb[44]{(((AK*aho)-(AH*(aho+ahw)))/ahK)}else{d})});let ain=(if sb[46]{d}else{(if sb[44]{(((AK*ahp)-(AH*(ahp+(sf[557]*afZ))))/ahK)}else{d})});let aio=(if sb[46]{d}else{(if sb[44]{(((AK*ahq)-(AH*ahq))/ahK)}else{d})});let aip=(if sb[46]{d}else{(if sb[44]{(((AK*ahr)-(AH*(ahr+ahw)))/ahK)}else{d})});let aiq=(if sb[46]{d}else{(if sb[44]{(((AK*ahs)-(AH*(ahs+ahy)))/ahK)}else{d})});let air=(if sb[46]{d}else{(if sb[44]{(((AK*aht)-(AH*(aht+ahy)))/ahK)}else{d})});let ais=(if sb[46]{d}else{(if sb[44]{(((AK*ahu)-(AH*(ahu+(sf[557]*ag1))))/ahK)}else{d})});let ait=(if sb[46]{d}else{(if sb[44]{(((AK*ahv)-(AH*(ahv+ahy)))/ahK)}else{d})});let amI=(sh*a12);let amK=(sh*a17);let amM=(sh*a15);let amO=(sh*a16);let amQ=(H*BS);let amR=((amI+amI)/amQ);let amS=((amK+amK)/amQ);let amT=((amM+amM)/amQ);let amU=((amO+amO)/amQ);let an1=(BT*BT);let ano=(if BW{(fP*(a12+amR))}else{(if (BQ!=0.0){((-(sE*(amR-a12)))/an1)}else{d})});let anp=(if BW{(fP*(a17+amS))}else{(if (BQ!=0.0){((-(sE*(amS-a17)))/an1)}else{d})});let anq=(if BW{(fP*(a15+amT))}else{(if (BQ!=0.0){((-(sE*(amT-a15)))/an1)}else{d})});let anr=(if BW{(fP*(a16+amU))}else{(if (BQ!=0.0){((-(sE*(amU-a16)))/an1)}else{d})});let aBB=(if GJ{(-(sf[806]*((GL*sf[869])/GM)))}else{(if (GC!=0.0){(sf[321]-(sf[806]*((GD*sf[867])/GE)))}else{d})});let aBC=(if GJ{(-(sf[806]*((GL*sf[870])/GM)))}else{(if (GC!=0.0){(sf[0]-(sf[806]*((GD*sf[868])/GE)))}else{d})});let aBI=(sf[218]*f64::powf(GT,sf[325]));let aC4=((H6*ano)+(BZ*(sf[846]*a0x)));let aC7=((H6*anp)+(BZ*(sf[846]*a0B)));let aC8=(H6*anq);let aC9=(H6*anr);let aCd=(H8*ano);let aCg=((H8*anp)+(BZ*(sf[846]*a0T)));let aCj=((H8*anq)+(BZ*(sf[846]*a0X)));let aCm=((H8*anr)+(BZ*(sf[846]*a11)));let aD5=(if Hk{(-(sf[802]*((Hm*sf[904])/Hn)))}else{(if (Hd!=0.0){(sf[0]-(sf[802]*((He*sf[900])/Hf)))}else{d})});
        let aD6=(if Hk{(-(sf[802]*((Hm*sf[905])/Hn)))}else{(if (Hd!=0.0){(sf[322]-(sf[802]*((He*sf[901])/Hf)))}else{d})});let aD7=(if Hk{(-(sf[802]*((Hm*sf[906])/Hn)))}else{(if (Hd!=0.0){(sf[323]-(sf[802]*((He*sf[902])/Hf)))}else{d})});let aD8=(if Hk{(-(sf[802]*((Hm*sf[907])/Hn)))}else{(if (Hd!=0.0){(sf[321]-(sf[802]*((He*sf[903])/Hf)))}else{d})});let aDi=(sf[224]*f64::powf(Ht,sf[329]));let aEF=(if HR{(-(sf[802]*((HT*sf[905])/HU)))}else{(if (HK!=0.0){(sf[322]-(sf[802]*((HL*sf[901])/HM)))}else{d})});let aEG=(if HR{(-(sf[802]*((HT*sf[911])/HU)))}else{(if (HK!=0.0){(sf[324]-(sf[802]*((HL*sf[910])/HM)))}else{d})});let aEH=(if HR{(-(sf[802]*((HT*sf[906])/HU)))}else{(if (HK!=0.0){(sf[323]-(sf[802]*((HL*sf[902])/HM)))}else{d})});let aEI=(if HR{(-(sf[802]*((HT*sf[907])/HU)))}else{(if (HK!=0.0){(sf[321]-(sf[802]*((HL*sf[903])/HM)))}else{d})});let aES=(sf[224]*f64::powf(I0,sf[329]));let aFy=(sf[6]*(sf[296]*(sf[540]*(sf[908]+(sf[809]*((sf[814]*(-((-(aEF/sf[507]))*aES)))+(sf[810]*(sf[322]-aEF))))))));let aFA=(sf[6]*(sf[296]*(sf[540]*(sf[909]+(sf[809]*((sf[814]*(-((-(aEH/sf[507]))*aES)))+(sf[810]*(sf[323]-aEH))))))));let aFS=(sf[850]*(if Ip{(Iq*sf[913])}else{(if (Im!=0.0){(In*sf[913])}else{a8w})}));let aFT=(sf[850]*(if Ip{d}else{(if (Im!=0.0){d}else{a8x})}));let aFU=(sf[850]*(if Ip{(Iq*sf[914])}else{(if (Im!=0.0){(In*sf[914])}else{a8y})}));let aFV=(sf[850]*(if Ip{d}else{(if (Im!=0.0){d}else{a8z})}));let aFW=(sf[850]*(if Ip{d}else{(if (Im!=0.0){d}else{a8A})}));let aH3=(H*J8);let aHb=(J9*J9);let aHp=(if sb[60]{(((J9*(sf[857]*ND))-(J5*((g1*(if IY{(IZ*sf[915])}else{(if IU{(IV*sf[915])}else{d})}))/aH3)))/aHb)}else{(if (sf[300]!=0.0){((sf[856]*((sf[845]*(((zG*ae2)-(zD*(ae2/aea)))/aei))+(sf[854]*(((zK*ae6)-(zC*(ae6/aew)))/aeE))))/sf[763])}else{d})});let aHq=(if sb[60]{(((J9*(sf[857]*NE))-(J5*((g1*(if IY{(IZ*sf[916])}else{(if IU{(IV*sf[916])}else{d})}))/aH3)))/aHb)}else{(if (sf[300]!=0.0){((sf[856]*((sf[845]*(((zG*ae3)-(zD*(ae3/aea)))/aei))+(sf[854]*(((zK*ae7)-(zC*(ae7/aew)))/aeE))))/sf[763])}else{d})});let aHr=(if sb[60]{(((J9*(sf[857]*NF))-(J5*((g1*(if IY{(IZ*sf[917])}else{(if IU{(IV*sf[917])}else{d})}))/aH3)))/aHb)}else{(if (sf[300]!=0.0){((sf[856]*((sf[845]*(((zG*ae4)-(zD*(ae4/aea)))/aei))+(sf[854]*(((zK*ae8)-(zC*(ae8/aew)))/aeE))))/sf[763])}else{d})});let aHs=(if sb[60]{(((J9*(sf[857]*NG))-(J5*((g1*(if IY{(IZ*sf[918])}else{(if IU{(IV*sf[918])}else{d})}))/aH3)))/aHb)}else{(if (sf[300]!=0.0){((sf[856]*((sf[845]*(((zG*ae5)-(zD*(ae5/aea)))/aei))+(sf[854]*(((zK*ae9)-(zC*(ae9/aew)))/aeE))))/sf[763])}else{d})});let aHF=(if sb[64]{(sf[816]*O2)}else{d});let aHG=(if sb[64]{(sf[816]*O3)}else{d});let aHH=(if sb[64]{(sf[816]*O4)}else{d});let aHI=(if sb[64]{(sf[816]*O5)}else{d});let aHJ=(H*Jn);let aHR=(Jo*Jo);let aId=(if sb[64]{(g1*(if m6{(m7*sf[864])}else{(if (m3!=0.0){(m4*sf[864])}else{d})}))}else{d});let aIe=(if sb[64]{(g1*(if m6{(m7*sf[866])}else{(if (m3!=0.0){(m4*sf[866])}else{d})}))}else{d});let aIf=(if sb[64]{(g1*(if m6{(m7*sf[865])}else{(if (m3!=0.0){(m4*sf[865])}else{d})}))}else{d});let aIg=(if sb[64]{(g1*(if m6{(m7*sf[861])}else{(if (m3!=0.0){(m4*sf[861])}else{d})}))}else{d});let aIh=(H*Ju);let aIp=(Jv*Jv);let aJt=(H*JZ);let aJB=(K0*K0);let aJU=(AP*(if sb[65]{(((K0*(sf[859]*O2))-(JW*((g1*(if JP{(JQ*sf[864])}else{(if JL{(JM*sf[864])}else{d})}))/aJt)))/aJB)}else{(if sb[64]{((sf[858]*((sf[845]*(if sb[64]{(((Jo*aHF)-(Jl*(aHF/aHJ)))/aHR)}else{d}))+(sf[854]*(if sb[64]{(((Jv*aId)-(Js*(aId/aIh)))/aIp)}else{d}))))/sf[763])}else{d})}));let aK3=(AP*(if sb[65]{(((K0*(sf[859]*O4))-(JW*((g1*(if JP{(JQ*sf[865])}else{(if JL{(JM*sf[865])}else{d})}))/aJt)))/aJB)}else{(if sb[64]{((sf[858]*((sf[845]*(if sb[64]{(((Jo*aHH)-(Jl*(aHH/aHJ)))/aHR)}else{d}))+(sf[854]*(if sb[64]{(((Jv*aIf)-(Js*(aIf/aIh)))/aIp)}else{d}))))/sf[763])}else{d})}));let aKm=(sf[306]*f64::powf(qW,sf[363]));let aKw=(Kh*Kh);let aKE=(Kn*sf[921]);let aKF=(Kn*sf[922]);let aKJ=(Ko*Ko);let aL9=(s2*s2);let aLK=(if (sf[305]!=0.0){(aFV/sf[851])}else{d});let aMn=(sf[307]*aFV);let aMt=(if (sf[305]!=0.0){(aC4+(sf[307]*aFS))}else{d});let aMu=(if (sf[305]!=0.0){(sf[307]*aFT)}else{d});
        let aMv=(if (sf[305]!=0.0){(aC7+(sf[307]*aFU))}else{d});let aMw=(if (sf[305]!=0.0){(aC8+aMn)}else{d});let aMx=(if (sf[305]!=0.0){(aC9+aMn)}else{d});let aMy=(if (sf[305]!=0.0){(sf[307]*aFW)}else{d});let aN1=(if sb[67]{aC4}else{(if (sf[305]!=0.0){(sf[310]*aMt)}else{d})});let aN2=(if sb[67]{d}else{(if (sf[305]!=0.0){(sf[310]*aMu)}else{d})});let aN3=(if sb[67]{aC7}else{(if (sf[305]!=0.0){(sf[310]*aMv)}else{d})});let aN4=(if sb[67]{aC8}else{(if (sf[305]!=0.0){(sf[310]*aMw)}else{d})});let aN5=(if sb[67]{aC9}else{(if (sf[305]!=0.0){(sf[310]*aMx)}else{d})});let aN6=(if sb[67]{d}else{(if (sf[305]!=0.0){(sf[310]*aMy)}else{d})});let aN7=(if sb[67]{aCd}else{(if (sf[305]!=0.0){(aCd+(sf[309]*aMt))}else{d})});let aN8=(if sb[67]{d}else{(if (sf[305]!=0.0){(sf[309]*aMu)}else{d})});let aN9=(if sb[67]{aCg}else{(if (sf[305]!=0.0){(aCg+(sf[309]*aMv))}else{d})});let aNa=(if sb[67]{aCj}else{(if (sf[305]!=0.0){(aCj+(sf[309]*aMw))}else{d})});let aNb=(if sb[67]{aCm}else{(if (sf[305]!=0.0){(aCm+(sf[309]*aMx))}else{d})});let aNc=(if sb[67]{d}else{(if (sf[305]!=0.0){(sf[309]*aMy)}else{d})});let aNg=(if sb[67]{aFV}else{(if (sf[305]!=0.0){(sf[308]*aFV)}else{d})});let aNy=(L4*L4);let aOj=(if Li{((Lj*a2E)+(sR*(sf[759]*ano)))}else{(if (Le!=0.0){(((L4*(aN1+aN7))-(Lf*((a2W-(L3*a2E))/a2Z)))/aNy)}else{d})});let aOk=(if Li{d}else{(if (Le!=0.0){((aN2+aN8)/L4)}else{d})});let aOl=(if Li{((Lj*a2H)+(sR*(sf[759]*anp)))}else{(if (Le!=0.0){(((L4*(aN3+aN9))-(Lf*(((sR*(a2O+a2S))-(L3*a2H))/a2Z)))/aNy)}else{d})});let aOm=(if Li{((Lj*a2K)+(sR*(sf[759]*anq)))}else{(if (Le!=0.0){(((L4*(aN4+aNa))-(Lf*(((sR*a2P)-(L3*a2K))/a2Z)))/aNy)}else{d})});let aOn=(if Li{((Lj*a2N)+(sR*(sf[759]*anr)))}else{(if (Le!=0.0){(((L4*(aN5+aNb))-(Lf*(((sR*a2Q)-(L3*a2N))/a2Z)))/aNy)}else{d})});let aOo=(if Li{d}else{(if (Le!=0.0){((aN6+aNc)/L4)}else{d})});let aON=(if sb[75]{d}else{(if sb[73]{(sf[316]*aOj)}else{(if (sf[314]!=0.0){(sf[309]*aOj)}else{d})})});let aOO=(if sb[75]{d}else{(if sb[73]{(sf[316]*aOk)}else{(if (sf[314]!=0.0){(sf[309]*aOk)}else{d})})});let aOP=(if sb[75]{d}else{(if sb[73]{(sf[316]*aOl)}else{(if (sf[314]!=0.0){(sf[309]*aOl)}else{d})})});let aOQ=(if sb[75]{d}else{(if sb[73]{(sf[316]*aOm)}else{(if (sf[314]!=0.0){(sf[309]*aOm)}else{d})})});let aOR=(if sb[75]{d}else{(if sb[73]{(sf[316]*aOn)}else{(if (sf[314]!=0.0){(sf[309]*aOn)}else{d})})});let aOS=(if sb[75]{d}else{(if sb[73]{(sf[316]*aOo)}else{(if (sf[314]!=0.0){(sf[309]*aOo)}else{d})})});let aRy=(sf[0]*((if sb[67]{aFS}else{(if (sf[305]!=0.0){(sf[308]*aFS)}else{d})})+((sf[842]*Yk)+aN1)));let aRz=(sf[0]*(aN2+(if sb[67]{aFT}else{(if (sf[305]!=0.0){(sf[308]*aFT)}else{d})})));let aRA=(sf[0]*((if sb[67]{aFU}else{(if (sf[305]!=0.0){(sf[308]*aFU)}else{d})})+((sf[842]*Yl)+aN3)));let aRB=(sf[0]*(aN4+aNg));let aRC=(sf[0]*(aN5+aNg));let aRD=(sf[0]*(aN6+(if sb[67]{aFW}else{(if (sf[305]!=0.0){(sf[308]*aFW)}else{d})})));let aRR=(sf[0]*(sf[843]*((sf[807]*(-((-(sf[528]*aBB))*aBI)))+(bK*(sf[321]-aBB)))));let aRS=(sf[0]*(sf[843]*((sf[807]*(-((-(sf[528]*aBC))*aBI)))+(bK*(sf[0]-aBC)))));let aRX=(sf[0]*aN7);let aRY=(sf[0]*aN8);let aRZ=(sf[0]*(((IB*(sf[855]*Xk))+(IA*WG))+((sf[844]*a0m)+aN9)));let aS0=(sf[0]*(((IB*(sf[855]*Xl))+(IA*WH))+((sf[844]*a0n)+aNa)));let aS1=(sf[0]*(((IB*(sf[855]*Xm))+(IA*WC))+((sf[844]*a0j)+aNb)));let aS2=(sf[0]*aNc);let aSf=(sf[0]*(if (sf[305]!=0.0){(KG*((if (sf[305]!=0.0){(aFS/sf[851])}else{d})+((if (sf[305]!=0.0){(sf[842]*(if (sf[305]!=0.0){((Kq*(if (sf[305]!=0.0){(Y5*aKm)}else{d}))+(Kb*(if Kl{(((Ko*aKE)-(Kn*aKE))/aKJ)}else{(if Kf{((-(Kg*sf[919]))/aKw)}else{d})})))}else{d}))}else{d})+(if (sf[305]!=0.0){((KB*(if (sf[305]!=0.0){((Ky*((sf[381]*a0o)/sf[588]))+(Kx*((-(fP*a0r))/aL9)))}else{d}))+(KA*(sf[846]*ano)))}else{d}))))}else{d}));let aSg=(sf[0]*(if (sf[305]!=0.0){((KI*sf[364])+(KG*(if (sf[305]!=0.0){(aFT/sf[851])}else{d})))}else{d}));
        let aSh=(sf[0]*(if (sf[305]!=0.0){((KI*sf[365])+(KG*((if (sf[305]!=0.0){(aFU/sf[851])}else{d})+((if (sf[305]!=0.0){(sf[842]*(if (sf[305]!=0.0){((Kq*(if (sf[305]!=0.0){(Y6*aKm)}else{d}))+(Kb*(if Kl{(((Ko*aKF)-(Kn*aKF))/aKJ)}else{(if Kf{((-(Kg*sf[920]))/aKw)}else{d})})))}else{d}))}else{d})+(if (sf[305]!=0.0){((KB*(if (sf[305]!=0.0){((Ky*((sf[381]*a0p)/sf[588]))+(Kx*((-(fP*a0s))/aL9)))}else{d}))+(KA*(sf[846]*anp)))}else{d})))))}else{d}));let aSi=(sf[0]*(if (sf[305]!=0.0){(KG*((if (sf[305]!=0.0){(KA*(sf[846]*anq))}else{d})+aLK))}else{d}));let aSj=(sf[0]*(if (sf[305]!=0.0){(KG*((if (sf[305]!=0.0){(KA*(sf[846]*anr))}else{d})+aLK))}else{d}));let aSk=(sf[0]*(if (sf[305]!=0.0){(KG*(if (sf[305]!=0.0){(aFW/sf[851])}else{d}))}else{d}));let aTb=(sf[0]*(aFy+(if (sf[302]!=0.0){((K2*aim)+aJU)}else{d})));let aTc=(sf[0]*((sf[6]*(sf[296]*(sf[540]*((sf[809]*((sf[814]*(-((-(aEG/sf[507]))*aES)))+(sf[810]*(sf[324]-aEG))))+sf[912]))))+(if (sf[302]!=0.0){((K2*ain)+(AP*(if sb[65]{(((K0*(sf[859]*O3))-(JW*((g1*(if JP{(JQ*sf[866])}else{(if JL{(JM*sf[866])}else{d})}))/aJt)))/aJB)}else{(if sb[64]{((sf[858]*((sf[845]*(if sb[64]{(((Jo*aHG)-(Jl*(aHG/aHJ)))/aHR)}else{d}))+(sf[854]*(if sb[64]{(((Jv*aIe)-(Js*(aIe/aIh)))/aIp)}else{d}))))/sf[763])}else{d})})))}else{d})));let aTd=(sf[0]*(if (sf[302]!=0.0){(K2*aio)}else{d}));let aTe=(sf[0]*(aFy+(if (sf[302]!=0.0){(aJU+(K2*aip))}else{d})));let aTf=(sf[0]*(aFA+(if (sf[302]!=0.0){((K2*aiq)+aK3)}else{d})));let aTg=(sf[0]*(aFA+(if (sf[302]!=0.0){(aK3+(K2*air))}else{d})));let aTh=(sf[0]*((sf[6]*(sf[296]*(sf[540]*(sf[872]+(sf[809]*((sf[814]*(-((-(aEI/sf[507]))*aES)))+(sf[810]*(sf[321]-aEI))))))))+(if (sf[302]!=0.0){((K2*ais)+(AP*(if sb[65]{(((K0*(sf[859]*O5))-(JW*((g1*(if JP{(JQ*sf[861])}else{(if JL{(JM*sf[861])}else{d})}))/aJt)))/aJB)}else{(if sb[64]{((sf[858]*((sf[845]*(if sb[64]{(((Jo*aHI)-(Jl*(aHI/aHJ)))/aHR)}else{d}))+(sf[854]*(if sb[64]{(((Jv*aIg)-(Js*(aIg/aIh)))/aIp)}else{d}))))/sf[763])}else{d})})))}else{d})));let aTi=(sf[0]*(aFA+(if (sf[302]!=0.0){(aK3+(K2*ait))}else{d})));let aTW=(sf[0]*((sf[7]*(sf[296]*(sf[540]*(sf[871]+(sf[809]*((sf[814]*(-((-(aD5/sf[507]))*aDi)))+(sf[810]*(sf[0]-aD5))))))))+(if (sf[302]!=0.0){(sf[7]*aHp)}else{aHp})));let aTX=(sf[0]*((sf[7]*(sf[296]*(sf[540]*((sf[809]*((sf[814]*(-((-(aD6/sf[507]))*aDi)))+(sf[810]*(sf[322]-aD6))))+sf[908]))))+(if (sf[302]!=0.0){(sf[7]*aHq)}else{aHq})));let aTY=(sf[0]*((sf[7]*(sf[296]*(sf[540]*((sf[809]*((sf[814]*(-((-(aD7/sf[507]))*aDi)))+(sf[810]*(sf[323]-aD7))))+sf[909]))))+(if (sf[302]!=0.0){(sf[7]*aHr)}else{aHr})));let aTZ=(sf[0]*((sf[7]*(sf[296]*(sf[540]*(sf[872]+(sf[809]*((sf[814]*(-((-(aD8/sf[507]))*aDi)))+(sf[810]*(sf[321]-aD8))))))))+(if (sf[302]!=0.0){(sf[7]*aHs)}else{aHs})));

        CommonStampValues {
            b, d, G, H, W, bK, fL, fP,
            g1, gr, kr, kv, kx, kC, kF, kK,
            kS, kV, kY, l2, lD, lE, lG, lJ,
            lK, n6, p4, q2, qr, qu, qx, qY,
            sg, sQ, sR, sW, sX, tg, ti, tl,
            tm, tv, u1, u3, u5, ua, ub, ui,
            uj, ul, uq, us, vi, vk, vm, vr,
            vs, vT, w6, wj, ww, wD, wE, wH,
            wJ, wO, wP, wV, wZ, x2, xa, xb,
            xc, xe, xg, xk, xl, xn, xq, xs,
            xt, xy, xz, yb, yd, yf, yg, yj,
            yl, yq, yr, yw, yz, yB, yJ, yK,
            yL, yN, yS, yT, yV, yX, yZ, z0,
            z5, z6, Ac, At, AP, BZ, Cb, Co,
            Cp, Cq, Ct, Cu, Cy, Cz, CB, CF,
            CH, CM, CN, D2, EL, EM, EO, EQ,
            ES, EU, EV, EX, F5, F8, F9, Fa,
            Fg, Fi, Fj, Fn, Fp, Fs, Fu, Fz,
            FA, L4, LA, Mh, Mk, Mn, Mq, Mu,
            My, MG, MM, MX, ND, NE, NF, NG,
            PA, PB, PC, Uf, Ug, Uh, WD, WE,
            WF, Xk, Xl, Xm, Xt, Xu, Xv, XC,
            XD, XE, Ya, Yb, a14, a15, a16, a2y,
            a2z, a2A, a2B, a2E, a2H, a2K, a2N, a2O,
            a2P, a2Q, a2S, a2W, a2Z, a3x, a3y, a4u,
            a4v, a6E, a6F, a6G, a7z, a7A, a7B, a7O,
            a7P, a7Q, a8b, a8c, a8d, a8e, a8f, a8w,
            a8x, a8y, a8z, a8A, afY, afZ, ag0, ag1,
            age, agf, agg, agh, agi, agj, agk, agl,
            aim, ain, aio, aip, aiq, air, ais, ait,
            ano, anp, anq, anr, aON, aOO, aOP, aOQ,
            aOR, aOS, aRy, aRz, aRA, aRB, aRC, aRD,
            aRR, aRS, aRX, aRY, aRZ, aS0, aS1, aS2,
            aSf, aSg, aSh, aSi, aSj, aSk, aTb, aTc,
            aTd, aTe, aTf, aTg, aTh, aTi, aTW, aTX,
            aTY, aTZ,
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let CommonStampValues {
            b, d, G, H, W, bK, fL, fP,
            g1, gr, kr, kv, kx, kC, kF, kK,
            kS, kV, kY, l2, lD, lE, lG, lJ,
            lK, n6, p4, q2, qr, qu, qx, qY,
            sg, sQ, sR, sW, sX, tg, ti, tl,
            tm, tv, u1, u3, u5, ua, ub, ui,
            uj, ul, uq, us, vi, vk, vm, vr,
            vs, vT, w6, wj, ww, wD, wE, wH,
            wJ, wO, wP, wV, wZ, x2, xa, xb,
            xc, xe, xg, xk, xl, xn, xq, xs,
            xt, xy, xz, yb, yd, yf, yg, yj,
            yl, yq, yr, yw, yz, yB, yJ, yK,
            yL, yN, yS, yT, yV, yX, yZ, z0,
            z5, z6, Ac, At, AP, BZ, Cb, Co,
            Cp, Cq, Ct, Cu, Cy, Cz, CB, CF,
            CH, CM, CN, D2, EL, EM, EO, EQ,
            ES, EU, EV, EX, F5, F8, F9, Fa,
            Fg, Fi, Fj, Fn, Fp, Fs, Fu, Fz,
            FA, L4, LA, Mh, Mk, Mn, Mq, Mu,
            My, MG, MM, MX, ND, NE, NF, NG,
            PA, PB, PC, Uf, Ug, Uh, WD, WE,
            WF, Xk, Xl, Xm, Xt, Xu, Xv, XC,
            XD, XE, Ya, Yb, a14, a15, a16, a2y,
            a2z, a2A, a2B, a2E, a2H, a2K, a2N, a2O,
            a2P, a2Q, a2S, a2W, a2Z, a3x, a3y, a4u,
            a4v, a6E, a6F, a6G, a7z, a7A, a7B, a7O,
            a7P, a7Q, a8b, a8c, a8d, a8e, a8f, a8w,
            a8x, a8y, a8z, a8A, afY, afZ, ag0, ag1,
            age, agf, agg, agh, agi, agj, agk, agl,
            aim, ain, aio, aip, aiq, air, ais, ait,
            ano, anp, anq, anr, aON, aOO, aOP, aOQ,
            aOR, aOS, aRy, aRz, aRA, aRB, aRC, aRD,
            aRR, aRS, aRX, aRY, aRZ, aS0, aS1, aS2,
            aSf, aSg, aSh, aSi, aSj, aSk, aTb, aTc,
            aTd, aTe, aTf, aTg, aTh, aTi, aTW, aTX,
            aTY, aTZ,
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
        let lH=(lE).exp();let tj=(tg).exp();let tq=(if tl{(tm*(b+(tg-sf[198])))}else{(if (ti!=0.0){tj}else{d})});let tx=(if (kx<sf[228]){b}else{d});let ty=(tv).exp();let tz=(b+ty);let tE=(!(tx!=0.0));let tG=((-tv)).exp();let tH=(b+tG);let tL=(if tE{(sf[228]-(G*(tH).ln()))}else{(if (tx!=0.0){(kx-(G*(tz).ln()))}else{d})});let tN=(tL*sf[229]);let tO=(sf[228]-tL);let tP={let pb=tO;pb*pb};let u6=((sf[149]!=0.0)&&(u5!=0.0));let u7=(u3).exp();let uf=(if ua{(ub*(b+(u3-sf[198])))}else{(if u6{u7}else{tg})});let um=((sf[149]!=0.0)&&(ul!=0.0));let un=(ui).exp();let uw=(if uq{(us*(b+(ui-uj)))}else{(if um{un}else{tq})});let ux=(u1-b);let uy=(sf[661]*ux);let uA=(ux*sf[822]);let uD=((b+(g1*uf))).sqrt();let uE=(b+uD);let uF=(uA/uE);let uG=(b+sg);let uK=(sf[676]*(q2-b));let uL=(uw*uK);let uM=(b+uw);let v2=(sf[230]*((q2+u1)-H));let vn=((sf[149]!=0.0)&&(vm!=0.0));let vo=(vk).exp();let vx=(vi-b);let vy=(sf[667]*vx);let vA=(vx*sf[823]);let vD=((b+(g1*(if vr{(vs*(b+(vk-sf[198])))}else{(if vn{vo}else{uf})})))).sqrt();let vE=(b+vD);let wl=(sf[653]*(wj-b));let wK=((wD!=0.0)&&(wJ!=0.0));let wL=(wH).exp();let wT=(if wO{(wP*(b+(wH-sf[198])))}else{(if wK{wL}else{d})});let xu=((xs!=0.0)&&xt);let xv=(xn).exp();let xE=(-kx);let xF=(b-(if xy{(xz*(b+(xn-sf[198])))}else{(if xu{xv}else{d})}));let xH=(b+(xF/xn));let xL=((wD!=0.0)&&(!(xq!=0.0)));let xM=(fP*kx);let xN=(xn*xM);let xO=0.3333333333333333;let xP=(xn*xO);let xQ=0.25;let xS=(b+(xn*xQ));let xU=(b+(xP*xS));let xY=((if xL{(xN*xU)}else{(if xt{(xE*xH)}else{d})})*sf[824]);let xZ=(qY*xY);let y4=(!(wD!=0.0));let ym=((yb!=0.0)&&(yl!=0.0));let yn=(yj).exp();let yv=(if yq{(yr*(b+(yj-sf[198])))}else{(if ym{yn}else{d})});let z1=((yZ!=0.0)&&z0);let z2=(yV).exp();let zb=(-kr);let zc=(b-(if z5{(z6*(b+(yV-sf[198])))}else{(if z1{z2}else{d})}));let ze=(b+(zc/yV));let zi=((yb!=0.0)&&(!(yX!=0.0)));let zj=(fP*kr);let zk=(yV*zj);let zl=(xO*yV);let zn=(b+(xQ*yV));let zp=(b+(zl*zn));let zt=((if zi{(zk*zp)}else{(if z0{(zb*ze)}else{d})})*sf[825]);let zu=(yf*zt);let zz=(!(yb!=0.0));let zA=(if zz{d}else{(if (yb!=0.0){(sf[53]*(sf[529]*(yv*zu)))}else{d})});let zO=(sf[826]*(lD-b));let zT=((b+(lD*sf[828]))).sqrt();let zU=(b+zT);let zV=(zO/zU);let A2=(if (sf[242]!=0.0){(sf[7]*zV)}else{zV});let AR=(if (sf[242]!=0.0){(Ac*AP)}else{d});let AW=(if (sf[248]!=0.0){(kr+kC)}else{d});let AY=(-AW);let B2=(if (AY<d){b}else{d});let B3=((sf[248]!=0.0)&&(B2!=0.0));let B6=((sf[249]+(if (sf[248]!=0.0){(AW*AW)}else{At}))).sqrt();let B7=(B6-AY);let Bb=((sf[248]!=0.0)&&(!(B2!=0.0)));let Be=(if Bb{(fP*(AY+B6))}else{(if B3{(sf[250]/B7)}else{d})});let Bv=(if (Be<sf[258]){b}else{d});let Bw=((sf[248]!=0.0)&&(Bv!=0.0));let Bx=(Be/sf[256]);let Bz=(b-f64::powf(Bx,sf[251]));let BD=((sf[248]!=0.0)&&(!(Bv!=0.0)));let BJ=(if sb[48]{b}else{(if BD{(sf[255]+(sf[265]*(Be-sf[258])))}else{(if Bw{(b/Bz)}else{d})})});let C0=(sQ*BZ);let C1=(sf[549]/C0);let C3=(if (C1<sf[16]){b}else{d});let C5=(bK*(if (C3!=0.0){sf[16]}else{C1}));let C8=(kC+(sf[795]*((if lJ{(lK*(b+(lE-sf[198])))}else{(if (lG!=0.0){lH}else{d})})-b)));let CI=(Co&&(CH!=0.0));let CJ=(CF).exp();let CR=(if CM{(CN*(b+(CF-sf[198])))}else{(if CI{CJ}else{d})});let CU=(CB*sf[839]);let D4=(((if (kr<sf[469]){b}else{d})!=0.0)&&((sf[272]!=0.0)&&D2));let Da=(if D4{sf[277]}else{d});let Db=(sf[469]-kr);let Dd=(if D4{(Db/qx)}else{p4});let Dg=(((H*Dd)/Da)).sqrt();let Dh=(if D4{Dg}else{d});let Dl=(D4&&(sf[279]!=0.0));let Do=(D4&&sb[53]);let Dr=(if Do{(b-(fP*qr))}else{d});let Ds=(sf[275]*Dr);let Du=(if Do{(Dr*Ds)}else{(if Dl{sf[275]}else{d})});let Dv=(Dh*Du);let Dz=(((Dh*Dh)+(Du*Du))).sqrt();let DB=(if D4{(Dv/Dz)}else{d});let DD=(if D4{(Db/DB)}else{d});let DE=(fP*DB);let DF=(Da*DE);let DI=(if D4{(DD+(qx*DF))}else{d});let DV=(sf[201]*(if Do{(b+(sf[281]*(b+(H*qr))))}else{d}));let DX=((if Do{sf[284]}else{d})-(sX/DV));let E0=(if Do{(DD-(DF*DX))}else{d});let E1=(E0-DI);let E3=(W*DD);let E4=(DD*E3);let Ea=((if Do{((E1*E1)+((qu*E4)/sf[201]))}else{Dd})).sqrt();let Ed=(if Do{(fP*((DI+E0)+Ea))}else{(if Dl{DI}else{d})});let Ee=(Ed-DD);let Eg=(if D4{(Ee/Ed)}else{d});let Ek=(if ((Eg).abs()>1e-7){b}else{d});let El=(D4&&(Ek!=0.0));
        let En=(if El{(DE/Eg)}else{d});let Ep=(Ed*sf[840]);let Eq=(En*Ep);let Es=(sf[841]/Ed);let Et=(Es).exp();let Ev=(b+(Du/En));let Ex=((Es*Ev)).exp();let Ey=(Et-Ex);let EC=(D4&&(!(Ek!=0.0)));let ED=(sf[4]*Du);let Fv=(EL&&(Fu!=0.0));let Fw=(Fs).exp();let FE=(if Fz{(FA*(b+(Fs-sf[198])))}else{(if Fv{Fw}else{CR})});let FF=(Cz*sf[839]);let FH=(if EL{(FE*FF)}else{(if EC{(Et*ED)}else{(if El{(Eq*Ey)}else{(if Co{(CR*CU)}else{d})})})});let FN=((Cb!=0.0)&&((if (FH>d){b}else{d})!=0.0));let FO=((sf[292]!=0.0)&&FN);let FP=(sf[554]+C5);let FQ=(sX*FP);let FX=(if FO{(((sf[380]/FQ)+(sf[661]*(sR/sf[633])))+(sf[546]/FP))}else{d});let FY=((sf[285]!=0.0)&&FO);let G1=(if FY{((FH-FX)/fL)}else{F5});let G3=(if (FH<FX){b}else{d});let G4=(FY&&(G3!=0.0));let G5=(G1).exp();let G6=(b+G5);let Gc=(FY&&(!(G3!=0.0)));let Ge=((-G1)).exp();let Gf=(b+Ge);let Gj=(if Gc{(FX-(fL*(Gf).ln()))}else{(if G4{(FH-(fL*(G6).ln()))}else{FH})});let Gk=(sX*Gj);let Gn=(FO&&sb[57]);let Go=(FX*Gk);let Gp=(FX+Gj);let Gt=(FN&&sb[58]);let Gu=(if Gt{Gk}else{(if Gn{(Go/Gp)}else{(if FY{Gk}else{d})})});let Lc=(if sb[69]{d}else{(if (sf[312]!=0.0){((Gu/L4)).abs()}else{d})});let M4=(sf[15]*(sf[0]*(-(zA*BJ))));let Mi=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, Mh);let Ml=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, Mk);let Mo=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, Mn);let Mr=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, Mq);let Mv=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, Mu);let Mz=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, My);let MH=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, MG);let MN=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, MM);let MY=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, MX);let a30=((a2W-(sW*a2E))/a2Z);let a34=(((sR*(a2S-a2O))-(sW*a2H))/a2Z);let a38=(((sR*(-a2P))-(sW*a2K))/a2Z);let a3c=(((sR*(-a2Q))-(sW*a2N))/a2Z);let a3z=(a3x/sf[227]);let a3A=(a3y/sf[227]);let a3H=(if tl{(tm*a3z)}else{(if (ti!=0.0){(tj*a3z)}else{d})});let a3I=(if tl{(tm*a3A)}else{(if (ti!=0.0){(tj*a3A)}else{d})});let a47=(if tE{(-(G*((tG*sf[336])/tH)))}else{(if (tx!=0.0){(sf[321]-(G*((ty*sf[334])/tz)))}else{d})});let a48=(if tE{(-(G*((tG*sf[337])/tH)))}else{(if (tx!=0.0){(sf[0]-(G*((ty*sf[335])/tz)))}else{d})});let a4d=(H*tO);let a4C=(if ua{(ub*sf[861])}else{(if u6{(u7*sf[861])}else{a3z})});
        let a4D=(if ua{(ub*sf[860])}else{(if u6{(u7*sf[860])}else{a3A})});let a4E=(a30/sf[633]);let a4F=(a34/sf[633]);let a4G=(a38/sf[633]);let a4H=(a3c/sf[633]);let a4U=(if uq{(us*a4E)}else{(if um{(un*a4E)}else{a3H})});let a4V=(if uq{(us*a4F)}else{(if um{(un*a4F)}else{a3I})});let a4W=(if uq{(us*a4G)}else{(if um{(un*a4G)}else{d})});let a4X=(if uq{(us*a4H)}else{(if um{(un*a4H)}else{d})});let a4Y=(sf[661]*a4u);let a4Z=(sf[661]*a4v);let a54=(H*uD);let a5a=(uE*uE);let a5E=(uM*uM);let a6R=(sf[667]*a6E);let a6S=(sf[667]*a6F);let a6T=(sf[667]*a6G);let a70=(H*vD);let a77=(vE*vE);let a8K=(wE*wE);let a8R=(sf[715]*(-((-(sf[20]*(H*Ya)))/a8K)));let a8S=(sf[715]*(-((-(sf[20]*(H*Yb)))/a8K)));let a93=(if (wD!=0.0){sf[888]}else{d});let a94=(if (wD!=0.0){sf[889]}else{d});let a95=(wV*a93);let a97=(wV*a94);let a99=(H*wZ);let a9e=(sf[233]*f64::powf(wZ,sf[338]));let a9Y=(xl*xl);let aa4=(if (wD!=0.0){(((xl*sf[890])-(xk*(sf[405]*(if (wD!=0.0){(xg*((xe*(((a95+a95)/a99)*a9e))+(x2*((sf[18]*(-(sf[236]*(bK*a93))))-((xc*((xa*a93)+(wV*(gr*a93))))+(xb*a93))))))}else{d}))))/a9Y)}else{a93});let aa5=(if (wD!=0.0){(((xl*sf[891])-(xk*(sf[405]*(if (wD!=0.0){(xg*((xe*(((a97+a97)/a99)*a9e))+(x2*((sf[18]*(-(sf[236]*(bK*a94))))-((xc*((xa*a94)+(wV*(gr*a94))))+(xb*a94))))))}else{d}))))/a9Y)}else{a94});let aaj=(xn*xn);let abo=(sf[224]*f64::powf(yd,sf[329]));let abr=(if (yb!=0.0){(sf[894]*abo)}else{d});let abs=(if (yb!=0.0){(sf[895]*abo)}else{d});let abx=(yg*yg);let abE=(sf[735]*(-((-(sf[52]*(H*abr)))/abx)));let abF=(sf[735]*(-((-(sf[52]*(H*abs)))/abx)));let abO=(if (yb!=0.0){sf[892]}else{d});let abP=(if (yb!=0.0){sf[893]}else{d});let abQ=(yw*abO);let abS=(yw*abP);let abU=(H*yz);let abZ=(sf[237]*f64::powf(yz,sf[343]));let acJ=(yT*yT);let acP=(if (yb!=0.0){(((yT*sf[896])-(yS*(sf[426]*(if (yb!=0.0){(xg*((yN*(((abQ+abQ)/abU)*abZ))+(yB*((sf[50]*(-(sf[240]*(bK*abO))))-((yL*((yJ*abO)+(yw*(gr*abO))))+(yK*abO))))))}else{d}))))/acJ)}else{abO});let acQ=(if (yb!=0.0){(((yT*sf[897])-(yS*(sf[426]*(if (yb!=0.0){(xg*((yN*(((abS+abS)/abU)*abZ))+(yB*((sf[50]*(-(sf[240]*(bK*abP))))-((yL*((yJ*abP)+(yw*(gr*abP))))+(yK*abP))))))}else{d}))))/acJ)}else{abP});let ad4=(yV*yV);let af0=(H*zT);let af8=(zU*zU);let af9=(((zU*(sf[826]*ND))-(zO*((sf[828]*ND)/af0)))/af8);let afd=(((zU*(sf[826]*NE))-(zO*((sf[828]*NE)/af0)))/af8);let afh=(((zU*(sf[826]*NF))-(zO*((sf[828]*NF)/af0)))/af8);let afl=(((zU*(sf[826]*NG))-(zO*((sf[828]*NG)/af0)))/af8);let aiu=(AP*afY);let aiD=(AP*ag0);let aj1=(AW*sf[350]);let aj3=(AW*sf[351]);let aj5=(AW*sf[352]);let ajg=(H*B6);let ajh=((if (sf[248]!=0.0){d}else{age})/ajg);let aji=((if (sf[248]!=0.0){d}else{agf})/ajg);let ajj=((if (sf[248]!=0.0){d}else{agg})/ajg);let ajk=((if (sf[248]!=0.0){(aj1+aj1)}else{age})/ajg);let ajl=((if (sf[248]!=0.0){(aj3+aj3)}else{agh})/ajg);let ajm=((if (sf[248]!=0.0){(aj5+aj5)}else{agi})/ajg);let ajn=((if (sf[248]!=0.0){d}else{agj})/ajg);let ajo=((if (sf[248]!=0.0){d}else{agk})/ajg);let ajp=((if (sf[248]!=0.0){d}else{agl})/ajg);let ajv=(B7*B7);let akg=(if Bb{(fP*ajh)}else{(if B3{((-(sf[250]*ajh))/ajv)}else{d})});let akh=(if Bb{(fP*aji)}else{(if B3{((-(sf[250]*aji))/ajv)}else{d})});let aki=(if Bb{(fP*ajj)}else{(if B3{((-(sf[250]*ajj))/ajv)}else{d})});let akj=(if Bb{(fP*(sf[353]+ajk))}else{(if B3{((-(sf[250]*(ajk-sf[353])))/ajv)}else{d})});let akk=(if Bb{(fP*(sf[354]+ajl))}else{(if B3{((-(sf[250]*(ajl-sf[354])))/ajv)}else{d})});let akl=(if Bb{(fP*(sf[355]+ajm))}else{(if B3{((-(sf[250]*(ajm-sf[355])))/ajv)}else{d})});let akm=(if Bb{(fP*ajn)}else{(if B3{((-(sf[250]*ajn))/ajv)}else{d})});let akn=(if Bb{(fP*ajo)}else{(if B3{((-(sf[250]*ajo))/ajv)}else{d})});let ako=(if Bb{(fP*ajp)}else{(if B3{((-(sf[250]*ajp))/ajv)}else{d})});let akz=(sf[251]*f64::powf(Bx,sf[260]));let akJ=(Bz*Bz);let alk=(if sb[48]{d}else{(if BD{(sf[265]*akg)}else{(if Bw{(((akg/sf[256])*akz)/akJ)}else{d})})});let all=(if sb[48]{d}else{(if BD{(sf[265]*akh)}else{(if Bw{(((akh/sf[256])*akz)/akJ)}else{d})})});let alm=(if sb[48]{d}else{(if BD{(sf[265]*aki)}else{(if Bw{(((aki/sf[256])*akz)/akJ)}else{d})})});
        let aln=(if sb[48]{d}else{(if BD{(sf[265]*akj)}else{(if Bw{(((akj/sf[256])*akz)/akJ)}else{d})})});let alo=(if sb[48]{d}else{(if BD{(sf[265]*akk)}else{(if Bw{(((akk/sf[256])*akz)/akJ)}else{d})})});let alp=(if sb[48]{d}else{(if BD{(sf[265]*akl)}else{(if Bw{(((akl/sf[256])*akz)/akJ)}else{d})})});let alq=(if sb[48]{d}else{(if BD{(sf[265]*akm)}else{(if Bw{(((akm/sf[256])*akz)/akJ)}else{d})})});let alr=(if sb[48]{d}else{(if BD{(sf[265]*akn)}else{(if Bw{(((akn/sf[256])*akz)/akJ)}else{d})})});let als=(if sb[48]{d}else{(if BD{(sf[265]*ako)}else{(if Bw{(((ako/sf[256])*akz)/akJ)}else{d})})});let alP=(BJ*(if (sf[242]!=0.0){(sf[7]*afh)}else{afh}));let am9=(BJ*(sf[653]*a8e));let ami=(BJ*(if (sf[242]!=0.0){(aiu+(Ac*aim))}else{d}));let anG=(C0*C0);let anV=(bK*(if (C3!=0.0){d}else{((-(sf[549]*((BZ*a2y)+(sQ*ano))))/anG)}));let anW=(bK*(if (C3!=0.0){d}else{((-(sf[549]*((BZ*a2z)+(sQ*anp))))/anG)}));let anX=(bK*(if (C3!=0.0){d}else{((-(sf[549]*((BZ*a2A)+(sQ*anq))))/anG)}));let anY=(bK*(if (C3!=0.0){d}else{((-(sf[549]*((BZ*a2B)+(sQ*anr))))/anG)}));let ao5=(C5*C5);let aom=((-a30)/sf[269]);let aon=((-a34)/sf[269]);let aoo=((-a38)/sf[269]);let aop=((-a3c)/sf[269]);let aoO=(if Co{(Cz*(if Ct{(Cu*aom)}else{(if Cp{(Cq*aom)}else{d})}))}else{d});let aoP=(if Co{((Cz*(if Ct{(Cu*aon)}else{(if Cp{(Cq*aon)}else{d})}))+(Cy*sf[321]))}else{d});let aoQ=(if Co{((Cz*(if Ct{(Cu*aoo)}else{(if Cp{(Cq*aoo)}else{d})}))+(sf[0]*Cy))}else{d});let aoR=(if Co{(Cz*(if Ct{(Cu*aop)}else{(if Cp{(Cq*aop)}else{d})}))}else{d});let aoU=(sf[270]*f64::powf(CB,sf[356]));let aoZ=(sf[838]*(aoO*aoU));let ap0=(sf[838]*(aoP*aoU));let ap1=(sf[838]*(aoQ*aoU));let ap2=(sf[838]*(aoR*aoU));let apf=(if CM{(CN*aoZ)}else{(if CI{(CJ*aoZ)}else{d})});let apg=(if CM{(CN*ap0)}else{(if CI{(CJ*ap0)}else{d})});let aph=(if CM{(CN*ap1)}else{(if CI{(CJ*ap1)}else{d})});let api=(if CM{(CN*ap2)}else{(if CI{(CJ*ap2)}else{d})});let apG=(qx*qx);let apP=(if D4{(((qx*sf[321])-(Db*XC))/apG)}else{Uf});let apQ=(if D4{(((sf[0]*qx)-(Db*XD))/apG)}else{Ug});let apR=(if D4{((-(Db*XE))/apG)}else{Uh});let apY=(H*Dg);let aq2=(if D4{(((H*apP)/Da)/apY)}else{d});let aq3=(if D4{(((H*apQ)/Da)/apY)}else{d});let aq4=(if D4{(((H*apR)/Da)/apY)}else{d});let aqb=(if Do{(-(fP*Xk))}else{d});let aqc=(if Do{(-(fP*Xl))}else{d});let aqd=(if Do{(-(fP*Xm))}else{d});let aqq=(if Do{((Ds*aqb)+(Dr*(sf[275]*aqb)))}else{d});let aqr=(if Do{((Ds*aqc)+(Dr*(sf[275]*aqc)))}else{d});let aqs=(if Do{((Ds*aqd)+(Dr*(sf[275]*aqd)))}else{d});let aqC=(Dh*aq2);let aqE=(Dh*aq3);let aqG=(Dh*aq4);let aqI=(Du*aqq);let aqK=(Du*aqr);let aqM=(Du*aqs);let aqR=(H*Dz);let aqY=(Dz*Dz);let ar8=(if D4{(((Dz*((Du*aq2)+(Dh*aqq)))-(Dv*(((aqC+aqC)+(aqI+aqI))/aqR)))/aqY)}else{d});let ar9=(if D4{(((Dz*((Du*aq3)+(Dh*aqr)))-(Dv*(((aqE+aqE)+(aqK+aqK))/aqR)))/aqY)}else{d});let ara=(if D4{(((Dz*((Du*aq4)+(Dh*aqs)))-(Dv*(((aqG+aqG)+(aqM+aqM))/aqR)))/aqY)}else{d});let are=(DB*DB);let arn=(if D4{(((DB*sf[321])-(Db*ar8))/are)}else{d});let aro=(if D4{(((sf[0]*DB)-(Db*ar9))/are)}else{d});let arp=(if D4{((-(Db*ara))/are)}else{d});let arq=(fP*ar8);let arr=(fP*ar9);let ars=(fP*ara);let art=(Da*arq);let aru=(Da*arr);let arv=(Da*ars);let arI=(if D4{(arn+((DF*XC)+(qx*art)))}else{d});let arJ=(if D4{(aro+((DF*XD)+(qx*aru)))}else{d});let arK=(if D4{(arp+((DF*XE)+(qx*arv)))}else{d});let as4=(DV*DV);let asw=(if Do{(-(DF*(-(a30/DV))))}else{d});let asx=(if Do{(arn-((DX*art)+(DF*(-(((DV*a34)-(sX*(sf[201]*(if Do{(sf[281]*(H*Xk))}else{d}))))/as4)))))}else{d});let asy=(if Do{(aro-((DX*aru)+(DF*(-(((DV*a38)-(sX*(sf[201]*(if Do{(sf[281]*(H*Xl))}else{d}))))/as4)))))}else{d});let asz=(if Do{(arp-((DX*arv)+(DF*(-(((DV*a3c)-(sX*(sf[201]*(if Do{(sf[281]*(H*Xm))}else{d}))))/as4)))))}else{d});let asD=(E1*asw);let asF=(E1*(asx-arI));let asH=(E1*(asy-arJ));let asJ=(E1*(asz-arK));let atj=(H*Ea);let atw=(if Do{(fP*(asw+((if Do{(asD+asD)}else{d})/atj)))}else{d});let atx=(if Do{(fP*((arI+asx)+((if Do{((asF+asF)+(((E4*Xt)+(qu*((E3*arn)+(DD*(W*arn)))))/sf[201]))}else{apP})/atj)))}else{(if Dl{arI}else{d})});
        let aty=(if Do{(fP*((arJ+asy)+((if Do{((asH+asH)+(((E4*Xu)+(qu*((E3*aro)+(DD*(W*aro)))))/sf[201]))}else{apQ})/atj)))}else{(if Dl{arJ}else{d})});let atz=(if Do{(fP*((arK+asz)+((if Do{((asJ+asJ)+(((E4*Xv)+(qu*((E3*arp)+(DD*(W*arp)))))/sf[201]))}else{apR})/atj)))}else{(if Dl{arK}else{d})});let atG=(Ed*Ed);let au0=(Eg*Eg);let aue=(if El{((-(DE*(if D4{(((Ed*atw)-(Ee*atw))/atG)}else{d})))/au0)}else{d});let auf=(if El{(((Eg*arq)-(DE*(if D4{(((Ed*(atx-arn))-(Ee*atx))/atG)}else{d})))/au0)}else{d});let aug=(if El{(((Eg*arr)-(DE*(if D4{(((Ed*(aty-aro))-(Ee*aty))/atG)}else{d})))/au0)}else{d});let auh=(if El{(((Eg*ars)-(DE*(if D4{(((Ed*(atz-arp))-(Ee*atz))/atG)}else{d})))/au0)}else{d});let auA=((-(sf[841]*atw))/atG);let auD=((-(sf[841]*atx))/atG);let auG=((-(sf[841]*aty))/atG);let auJ=((-(sf[841]*atz))/atG);let auK=(Et*auA);let auL=(Et*auD);let auM=(Et*auG);let auN=(Et*auJ);let auQ=(En*En);let avW=(sf[270]*f64::powf(Cz,sf[356]));let aw2=(EO*EO);let awm=(sf[287]*f64::powf(EQ,sf[357]));let awz=(if EL{(EM*((-(((EO*a30)-(sX*a30))/aw2))*awm))}else{d});let awA=(if EL{((ES*(sf[321]*avW))+(EM*((-(((EO*a34)-(sX*a34))/aw2))*awm)))}else{d});let awB=(if EL{((ES*(sf[0]*avW))+(EM*((-(((EO*a38)-(sX*a38))/aw2))*awm)))}else{d});let awC=(if EL{(EM*((-(((EO*a3c)-(sX*a3c))/aw2))*awm))}else{d});let awL=(if EX{(a30/sf[286])}else{d});let awM=(if EX{(a34/sf[286])}else{d});let awN=(if EX{(a38/sf[286])}else{d});let awO=(if EX{(a3c/sf[286])}else{d});let awT=(if EX{(awL/sf[289])}else{sf[334]});let awU=(if EX{(awM/sf[289])}else{sf[335]});let awV=(if EX{(awN/sf[289])}else{d});let awW=(if EX{(awO/sf[289])}else{d});let axD=(sf[290]*f64::powf(Fn,sf[358]));let axY=(sf[838]*(if EX{((Fp*awz)+(EU*((if Fg{(awL+(sf[289]*((Fi*(-awT))/Fj)))}else{(if F8{(sf[289]*((F9*awT)/Fa))}else{d})})*axD)))}else{(if EV{awz}else{d})}));let axZ=(sf[838]*(if EX{((Fp*awA)+(EU*((if Fg{(awM+(sf[289]*((Fi*(-awU))/Fj)))}else{(if F8{(sf[289]*((F9*awU)/Fa))}else{d})})*axD)))}else{(if EV{awA}else{d})}));let ay0=(sf[838]*(if EX{((Fp*awB)+(EU*((if Fg{(awN+(sf[289]*((Fi*(-awV))/Fj)))}else{(if F8{(sf[289]*((F9*awV)/Fa))}else{d})})*axD)))}else{(if EV{awB}else{d})}));let ay1=(sf[838]*(if EX{((Fp*awC)+(EU*((if Fg{(awO+(sf[289]*((Fi*(-awW))/Fj)))}else{(if F8{(sf[289]*((F9*awW)/Fa))}else{d})})*axD)))}else{(if EV{awC}else{d})}));let ays=(if EL{(FF*(if Fz{(FA*axY)}else{(if Fv{(Fw*axY)}else{apf})}))}else{(if EC{(ED*auK)}else{(if El{((Ey*((Ep*aue)+(En*(sf[840]*atw))))+(Eq*(auK-(Ex*((Ev*auA)+(Es*((-(Du*aue))/auQ)))))))}else{(if Co{((CU*apf)+(CR*(sf[839]*aoO)))}else{d})})})});let ayt=(if EL{((FF*(if Fz{(FA*axZ)}else{(if Fv{(Fw*axZ)}else{apg})}))+(FE*sf[898]))}else{(if EC{((ED*auL)+(Et*(sf[4]*aqq)))}else{(if El{((Ey*((Ep*auf)+(En*(sf[840]*atx))))+(Eq*(auL-(Ex*((Ev*auD)+(Es*(((En*aqq)-(Du*auf))/auQ)))))))}else{(if Co{((CU*apg)+(CR*(sf[839]*aoP)))}else{d})})})});let ayu=(if EL{((FF*(if Fz{(FA*ay0)}else{(if Fv{(Fw*ay0)}else{aph})}))+(FE*sf[899]))}else{(if EC{((ED*auM)+(Et*(sf[4]*aqr)))}else{(if El{((Ey*((Ep*aug)+(En*(sf[840]*aty))))+(Eq*(auM-(Ex*((Ev*auG)+(Es*(((En*aqr)-(Du*aug))/auQ)))))))}else{(if Co{((CU*aph)+(CR*(sf[839]*aoQ)))}else{d})})})});let ayv=(if EL{(FF*(if Fz{(FA*ay1)}else{(if Fv{(Fw*ay1)}else{api})}))}else{(if EC{((ED*auN)+(Et*(sf[4]*aqs)))}else{(if El{((Ey*((Ep*auh)+(En*(sf[840]*atz))))+(Eq*(auN-(Ex*((Ev*auJ)+(Es*(((En*aqs)-(Du*auh))/auQ)))))))}else{(if Co{((CU*api)+(CR*(sf[839]*aoR)))}else{d})})})});let ayK=(FQ*FQ);let az9=(FP*FP);let azo=(if FO{((((-(sf[380]*((FP*a30)+(sX*anV))))/ayK)+(sf[661]*(a2E/sf[633])))+((-(sf[546]*anV))/az9))}else{d});let azp=(if FO{((((-(sf[380]*((FP*a34)+(sX*anW))))/ayK)+(sf[661]*(a2H/sf[633])))+((-(sf[546]*anW))/az9))}else{d});let azq=(if FO{((((-(sf[380]*((FP*a38)+(sX*anX))))/ayK)+(sf[661]*(a2K/sf[633])))+((-(sf[546]*anX))/az9))}else{d});let azr=(if FO{((((-(sf[380]*((FP*a3c)+(sX*anY))))/ayK)+(sf[661]*(a2N/sf[633])))+((-(sf[546]*anY))/az9))}else{d});let azA=(if FY{((ays-azo)/fL)}else{awT});let azB=(if FY{((ayt-azp)/fL)}else{awU});let azC=(if FY{((ayu-azq)/fL)}else{awV});let azD=(if FY{((ayv-azr)/fL)}else{awW});
        let aAi=(if Gc{(azo-(fL*((Ge*(-azA))/Gf)))}else{(if G4{(ays-(fL*((G5*azA)/G6)))}else{ays})});let aAj=(if Gc{(azp-(fL*((Ge*(-azB))/Gf)))}else{(if G4{(ayt-(fL*((G5*azB)/G6)))}else{ayt})});let aAk=(if Gc{(azq-(fL*((Ge*(-azC))/Gf)))}else{(if G4{(ayu-(fL*((G5*azC)/G6)))}else{ayu})});let aAl=(if Gc{(azr-(fL*((Ge*(-azD))/Gf)))}else{(if G4{(ayv-(fL*((G5*azD)/G6)))}else{ayv})});let aAo=((Gj*a30)+(sX*aAi));let aAr=((Gj*a34)+(sX*aAj));let aAu=((Gj*a38)+(sX*aAk));let aAx=((Gj*a3c)+(sX*aAl));let aAV=(Gp*Gp);let aQ3=(sf[15]*(sf[0]*(sf[699]*a8z)));let aQ7=((((if sb[33]{(sf[661]*((sf[232]*a4u)+(uG*(sf[230]*a4u))))}else{(if sb[31]{a4Y}else{(if (sf[149]!=0.0){((a4Y+(uG*(((uE*(sf[822]*a4u))-(uA*((g1*a4C)/a54)))/a5a)))+(((uM*(uK*a4U))-(uL*a4U))/a5E))}else{d})})})+(sf[646]*a7z))+sf[366])-(if y4{d}else{(if (wD!=0.0){(sf[21]*(sf[528]*((xZ*(if wO{(wP*a8R)}else{(if wK{(wL*a8R)}else{d})}))+(wT*((xY*Ya)+(qY*(sf[824]*(if xL{((xU*((xM*aa4)+(xn*sf[341])))+(xN*((xS*(xO*aa4))+(xP*(xQ*aa4)))))}else{(if xt{((sf[0]*xH)+(xE*(((xn*(-(if xy{(xz*aa4)}else{(if xu{(xv*aa4)}else{d})})))-(xF*aa4))/aaj)))}else{d})}))))))))}else{d})}));let aQ8=((((if sb[33]{(sf[661]*((sf[232]*a4v)+((v2*a14)+(uG*(sf[230]*(WD+a4v))))))}else{(if sb[31]{a4Z}else{(if (sf[149]!=0.0){((a4Z+((uG*(((uE*(sf[822]*a4v))-(uA*((g1*a4D)/a54)))/a5a))+(uF*a14)))+(((uM*((uK*a4V)+(uw*(sf[676]*WD))))-(uL*a4V))/a5E))}else{d})})})+(sf[646]*a7B))+sf[367])-(if y4{d}else{(if (wD!=0.0){(sf[21]*(sf[528]*((xZ*(if wO{(wP*a8S)}else{(if wK{(wL*a8S)}else{d})}))+(wT*((xY*Yb)+(qY*(sf[824]*(if xL{((xU*((xM*aa5)+(xn*sf[342])))+(xN*((xS*(xO*aa5))+(xP*(xQ*aa5)))))}else{(if xt{((xH*sf[321])+(xE*(((xn*(-(if xy{(xz*aa5)}else{(if xu{(xv*aa5)}else{d})})))-(xF*aa5))/aaj)))}else{d})}))))))))}else{d})}));let aQF=(sf[15]*(sf[0]*(-(zA*alk))));let aQG=(sf[15]*(sf[0]*(-(zA*all))));let aQH=(sf[15]*(sf[0]*(-(zA*alm))));let aQI=(sf[15]*(sf[0]*(-(zA*aln))));let aQJ=(sf[15]*(sf[0]*(-((BJ*(if zz{d}else{(if (yb!=0.0){(sf[53]*(sf[529]*((zu*(if yq{(yr*abE)}else{(if ym{(yn*abE)}else{d})}))+(yv*((zt*abr)+(yf*(sf[825]*(if zi{((zp*((zj*acP)+(yV*sf[342])))+(zk*((zn*(xO*acP))+(zl*(xQ*acP)))))}else{(if z0{((ze*sf[321])+(zb*(((yV*(-(if z5{(z6*acP)}else{(if z1{(z2*acP)}else{d})})))-(zc*acP))/ad4)))}else{d})}))))))))}else{d})}))+(zA*alo)))));let aQK=(sf[15]*(sf[0]*(-((BJ*(if zz{d}else{(if (yb!=0.0){(sf[53]*(sf[529]*((zu*(if yq{(yr*abF)}else{(if ym{(yn*abF)}else{d})}))+(yv*((zt*abs)+(yf*(sf[825]*(if zi{((zp*((zj*acQ)+(yV*sf[341])))+(zk*((zn*(xO*acQ))+(zl*(xQ*acQ)))))}else{(if z0{((sf[0]*ze)+(zb*(((yV*(-(if z5{(z6*acQ)}else{(if z1{(z2*acQ)}else{d})})))-(zc*acQ))/ad4)))}else{d})}))))))))}else{d})}))+(zA*alp)))));let aQL=(sf[15]*(sf[0]*(-(zA*alq))));let aQM=(sf[15]*(sf[0]*(-(zA*alr))));let aQN=(sf[15]*(sf[0]*(-(zA*als))));let aRE=ddt_scale;let aTr=(sf[15]*(aRE*aTb));let aU6=(sf[15]*(aRE*aTY));

        stamper.stamp_current_node3_local(
            Some(6),
            Some(7),
            multiplicity * ((sf[15]*(sf[0]*n6))),
            5,
            multiplicity * ((sf[15]*(sf[0]*PA))),
            6,
            multiplicity * ((sf[15]*(sf[0]*PB))),
            7,
            multiplicity * ((sf[15]*(sf[0]*PC))),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*sX))),
            [3, 5, 6, 7],
            [(sf[15]*(sf[0]*a30)), (sf[15]*(sf[0]*a34)), (sf[15]*(sf[0]*a38)), (sf[15]*(sf[0]*a3c))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*((sf[699]*(ww-b))+((if sb[30]{vy}else{(if (sf[149]!=0.0){(vy+(vA/vE))}else{d})})+(sf[693]*(w6-b))))))),
            [3, 4, 5, 6, 7, 9],
            [(sf[15]*(sf[0]*((sf[699]*a8w)+((if sb[30]{a6R}else{(if (sf[149]!=0.0){(a6R+(((vE*(sf[823]*a6E))-(vA*((g1*(if vr{(vs*sf[861])}else{(if vn{(vo*sf[861])}else{a4C})}))/a70)))/a77))}else{d})})+(sf[693]*a7O))))), (sf[15]*(sf[0]*((sf[699]*a8x)+((if sb[30]{a6S}else{(if (sf[149]!=0.0){(a6S+(((vE*(sf[823]*a6F))-(vA*((g1*(if vr{(vs*sf[860])}else{(if vn{(vo*sf[860])}else{d})}))/a70)))/a77))}else{d})})+(sf[693]*a7P))))), (sf[15]*(sf[0]*((sf[699]*a8y)+((if sb[30]{a6T}else{(if (sf[149]!=0.0){(a6T+(((vE*(sf[823]*a6G))-(vA*((g1*(if vr{d}else{(if vn{d}else{a4D})}))/a70)))/a77))}else{d})})+(sf[693]*a7Q))))), aQ3, aQ3, (sf[15]*(sf[0]*(sf[699]*a8A)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*((sf[704]*(tq-b))+((tN*tP)+((((if sb[33]{(sf[661]*((ux*sf[232])+(uG*v2)))}else{(if sb[31]{uy}else{(if (sf[149]!=0.0){((uy+(uF*uG))+(uL/uM))}else{d})})})+(sf[646]*(vT-b)))+(d*kx))-(if y4{d}else{(if (wD!=0.0){(sf[21]*(sf[528]*(wT*xZ)))}else{d})}))))))),
            [3, 4, 5, 6, 7],
            [(sf[15]*(sf[0]*((sf[704]*a3H)+(((tP*(sf[229]*a47))+(tN*((-a47)*a4d)))+aQ7)))), (sf[15]*(sf[0]*(sf[646]*a7A))), (sf[15]*(sf[0]*((sf[704]*a3I)+(((tP*(sf[229]*a48))+(tN*((-a48)*a4d)))+aQ8)))), (sf[15]*(sf[0]*(if sb[33]{(sf[661]*((v2*a15)+(uG*(sf[230]*WE))))}else{(if sb[31]{d}else{(if (sf[149]!=0.0){((uF*a15)+(((uM*((uK*a4W)+(uw*(sf[676]*WE))))-(uL*a4W))/a5E))}else{d})})}))), (sf[15]*(sf[0]*(if sb[33]{(sf[661]*((v2*a16)+(uG*(sf[230]*WF))))}else{(if sb[31]{d}else{(if (sf[149]!=0.0){((uF*a16)+(((uM*((uK*a4X)+(uw*(sf[676]*WF))))-(uL*a4X))/a5E))}else{d})})})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(6),
            multiplicity * ((if (sf[149]!=0.0){M4}else{d})),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(if (sf[149]!=0.0){aQF}else{d}), (if (sf[149]!=0.0){aQG}else{d}), (if (sf[149]!=0.0){aQH}else{d}), (if (sf[149]!=0.0){aQI}else{d}), (if (sf[149]!=0.0){aQJ}else{d}), (if (sf[149]!=0.0){aQK}else{d}), (if (sf[149]!=0.0){aQL}else{d}), (if (sf[149]!=0.0){aQM}else{d}), (if (sf[149]!=0.0){aQN}else{d})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(7),
            multiplicity * ((if sb[30]{M4}else{d})),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(if sb[30]{aQF}else{d}), (if sb[30]{aQG}else{d}), (if sb[30]{aQH}else{d}), (if sb[30]{aQI}else{d}), (if sb[30]{aQJ}else{d}), (if sb[30]{aQK}else{d}), (if sb[30]{aQL}else{d}), (if sb[30]{aQM}else{d}), (if sb[30]{aQN}else{d})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(5),
            multiplicity * ((sf[15]*(sf[0]*(C8/C5)))),
            [3, 4, 5, 6, 7],
            [(sf[15]*(sf[0]*((-(C8*anV))/ao5))), (sf[15]*(sf[0]*((sf[0]+(sf[795]*(if lJ{(lK*sf[860])}else{(if (lG!=0.0){(lH*sf[860])}else{d})})))/C5))), (sf[15]*(sf[0]*(((C5*(sf[321]+(sf[795]*(if lJ{(lK*sf[861])}else{(if (lG!=0.0){(lH*sf[861])}else{d})}))))-(C8*anW))/ao5))), (sf[15]*(sf[0]*((-(C8*anX))/ao5))), (sf[15]*(sf[0]*((-(C8*anY))/ao5)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * ((sf[15]*(sf[0]*(-Gu)))),
            [3, 5, 6, 7],
            [(sf[15]*(sf[0]*(-(if Gt{aAo}else{(if Gn{(((Gp*((Gk*azo)+(FX*aAo)))-(Go*(azo+aAi)))/aAV)}else{(if FY{aAo}else{d})})})))), (sf[15]*(sf[0]*(-(if Gt{aAr}else{(if Gn{(((Gp*((Gk*azp)+(FX*aAr)))-(Go*(azp+aAj)))/aAV)}else{(if FY{aAr}else{d})})})))), (sf[15]*(sf[0]*(-(if Gt{aAu}else{(if Gn{(((Gp*((Gk*azq)+(FX*aAu)))-(Go*(azq+aAk)))/aAV)}else{(if FY{aAu}else{d})})})))), (sf[15]*(sf[0]*(-(if Gt{aAx}else{(if Gn{(((Gp*((Gk*azr)+(FX*aAx)))-(Go*(azr+aAl)))/aAV)}else{(if FY{aAx}else{d})})}))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * ((sf[15]*((sf[0]*(sf[0]*(kF-kv)))/sf[546]))),
            2,
            multiplicity * (sf[925]),
            3,
            multiplicity * (sf[926]),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(4),
            multiplicity * ((sf[15]*((sf[0]*kK)/sf[554]))),
            1,
            multiplicity * (sf[929]),
            4,
            multiplicity * (sf[930]),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * ((sf[15]*Mi)),
            [3, 4, 5, 6, 7, 9],
            [(sf[15]*(aRy*aRE)), (sf[15]*(aRz*aRE)), (sf[15]*(aRA*aRE)), (sf[15]*(aRB*aRE)), (sf[15]*(aRC*aRE)), (sf[15]*(aRD*aRE))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * ((sf[15]*Ml)),
            3,
            multiplicity * ((sf[15]*(aRE*aRR))),
            4,
            multiplicity * ((sf[15]*(aRE*aRS))),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(7),
            multiplicity * ((sf[15]*Mo)),
            [3, 4, 5, 6, 7, 9],
            [(sf[15]*(aRE*aRX)), (sf[15]*(aRE*aRY)), (sf[15]*(aRE*aRZ)), (sf[15]*(aRE*aS0)), (sf[15]*(aRE*aS1)), (sf[15]*(aRE*aS2))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            Some(5),
            multiplicity * ((sf[15]*Mr)),
            [3, 4, 5, 6, 7, 9],
            [(sf[15]*(aRE*aSf)), (sf[15]*(aRE*aSg)), (sf[15]*(aRE*aSh)), (sf[15]*(aRE*aSi)), (sf[15]*(aRE*aSj)), (sf[15]*(aRE*aSk))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * ((sf[15]*Mv)),
            1,
            multiplicity * ((sf[15]*(aRE*sf[372]))),
            2,
            multiplicity * ((sf[15]*(aRE*sf[373]))),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * ((sf[15]*Mz)),
            0,
            multiplicity * ((sf[15]*(aRE*sf[374]))),
            1,
            multiplicity * ((sf[15]*(aRE*sf[375]))),
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(8),
            multiplicity * ((sf[15]*(sf[0]*(AR*BJ)))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(sf[15]*(sf[0]*(ami+(AR*alk)))), (sf[15]*(sf[0]*((BJ*(if (sf[242]!=0.0){((AP*afZ)+(Ac*ain))}else{d}))+(AR*all)))), (sf[15]*(sf[0]*((BJ*(if (sf[242]!=0.0){(Ac*aio)}else{d}))+(AR*alm)))), (sf[15]*(sf[0]*(ami+(AR*aln)))), (sf[15]*(sf[0]*((BJ*(if (sf[242]!=0.0){(aiu+(Ac*aip))}else{d}))+(AR*alo)))), (sf[15]*(sf[0]*((BJ*(if (sf[242]!=0.0){(aiD+(Ac*aiq))}else{d}))+(AR*alp)))), (sf[15]*(sf[0]*((BJ*(if (sf[242]!=0.0){(aiD+(Ac*air))}else{d}))+(AR*alq)))), (sf[15]*(sf[0]*((BJ*(if (sf[242]!=0.0){((AP*ag1)+(Ac*ais))}else{d}))+(AR*alr)))), (sf[15]*(sf[0]*((BJ*(if (sf[242]!=0.0){(aiD+(Ac*ait))}else{d}))+(AR*als))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(8),
            multiplicity * ((sf[15]*(sf[784]*(sf[0]*l2)))),
            [0, 1, 4, 5, 6, 7, 8, 9],
            [sf[935], sf[936], sf[936], sf[936], sf[937], sf[937], sf[938], sf[937]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(8),
            multiplicity * ((sf[15]*MH)),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [aTr, (sf[15]*(aRE*aTc)), (sf[15]*(aRE*aTd)), aTr, (sf[15]*(aRE*aTe)), (sf[15]*(aRE*aTf)), (sf[15]*(aRE*aTg)), (sf[15]*(aRE*aTh)), (sf[15]*(aRE*aTi))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(9),
            multiplicity * ((sf[15]*(sf[0]*((A2*BJ)+((wl*BJ)+(d*kY)))))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(sf[15]*(sf[0]*((A2*alk)+(wl*alk)))), (sf[15]*(sf[0]*((A2*all)+(wl*all)))), (sf[15]*(sf[0]*((A2*alm)+((BJ*(sf[653]*a8b))+(wl*alm))))), (sf[15]*(sf[0]*(((BJ*(if (sf[242]!=0.0){(sf[7]*af9)}else{af9}))+(A2*aln))+(((BJ*(sf[653]*a8c))+(wl*aln))+sf[367])))), (sf[15]*(sf[0]*(((BJ*(if (sf[242]!=0.0){(sf[7]*afd)}else{afd}))+(A2*alo))+(((BJ*(sf[653]*a8d))+(wl*alo))+sf[368])))), (sf[15]*(sf[0]*((alP+(A2*alp))+((am9+(wl*alp))+sf[369])))), (sf[15]*(sf[0]*((alP+(A2*alq))+((am9+(wl*alq))+sf[369])))), (sf[15]*(sf[0]*((A2*alr)+(wl*alr)))), (sf[15]*(sf[0]*(((BJ*(if (sf[242]!=0.0){(sf[7]*afl)}else{afl}))+(A2*als))+(((BJ*(sf[653]*a8f))+(wl*als))+sf[366]))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(9),
            multiplicity * ((sf[15]*MN)),
            [4, 5, 6, 7, 9],
            [(sf[15]*(aRE*aTW)), (sf[15]*(aRE*aTX)), aU6, aU6, (sf[15]*(aRE*aTZ))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(8),
            Some(9),
            multiplicity * ((if (sf[196]!=0.0){(sf[15]*(sf[789]*(sf[0]*kV)))}else{d})),
            8,
            multiplicity * (sf[943]),
            9,
            multiplicity * (sf[944]),
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            d,
        );
        stamper.stamp_current_node2_local(
            Some(9),
            Some(6),
            multiplicity * ((if (sf[197]!=0.0){(sf[15]*(sf[794]*(sf[0]*kS)))}else{d})),
            6,
            multiplicity * (sf[949]),
            9,
            multiplicity * (sf[950]),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(6),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            d,
        );
        stamper.stamp_current_const_local(
            Some(10),
            None,
            multiplicity * (d),
        );
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (MX),
            10,
            multiplicity * (b),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(3),
            multiplicity * ((LA*MY)),
            [3, 4, 5, 6, 7, 9, 10],
            [(MY*aON), (MY*aOO), (MY*aOP), (MY*aOQ), (MY*aOR), (MY*aOS), (LA*aRE)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(5),
            multiplicity * ((Lc*MX)),
            10,
            multiplicity * (Lc),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(3),
            multiplicity * (MX),
            10,
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(3),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(3),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(3),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(8),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(8),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(5),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(8),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(9),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(6),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(8),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(6),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(6),
            multiplicity * (d),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        let br=self.branches;
        let branches=br;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let CommonStampValues {
            b, d, G, H, W, bK, fL, fP,
            g1, gr, kr, kv, kx, kC, kF, kK,
            kS, kV, kY, l2, lD, lE, lG, lJ,
            lK, n6, p4, q2, qr, qu, qx, qY,
            sg, sQ, sR, sW, sX, tg, ti, tl,
            tm, tv, u1, u3, u5, ua, ub, ui,
            uj, ul, uq, us, vi, vk, vm, vr,
            vs, vT, w6, wj, ww, wD, wE, wH,
            wJ, wO, wP, wV, wZ, x2, xa, xb,
            xc, xe, xg, xk, xl, xn, xq, xs,
            xt, xy, xz, yb, yd, yf, yg, yj,
            yl, yq, yr, yw, yz, yB, yJ, yK,
            yL, yN, yS, yT, yV, yX, yZ, z0,
            z5, z6, Ac, At, AP, BZ, Cb, Co,
            Cp, Cq, Ct, Cu, Cy, Cz, CB, CF,
            CH, CM, CN, D2, EL, EM, EO, EQ,
            ES, EU, EV, EX, F5, F8, F9, Fa,
            Fg, Fi, Fj, Fn, Fp, Fs, Fu, Fz,
            FA, L4, LA, Mh, Mk, Mn, Mq, Mu,
            My, MG, MM, MX, ND, NE, NF, NG,
            PA, PB, PC, Uf, Ug, Uh, WD, WE,
            WF, Xk, Xl, Xm, Xt, Xu, Xv, XC,
            XD, XE, Ya, Yb, a14, a15, a16, a2y,
            a2z, a2A, a2B, a2E, a2H, a2K, a2N, a2O,
            a2P, a2Q, a2S, a2W, a2Z, a3x, a3y, a4u,
            a4v, a6E, a6F, a6G, a7z, a7A, a7B, a7O,
            a7P, a7Q, a8b, a8c, a8d, a8e, a8f, a8w,
            a8x, a8y, a8z, a8A, afY, afZ, ag0, ag1,
            age, agf, agg, agh, agi, agj, agk, agl,
            aim, ain, aio, aip, aiq, air, ais, ait,
            ano, anp, anq, anr, aON, aOO, aOP, aOQ,
            aOR, aOS, aRy, aRz, aRA, aRB, aRC, aRD,
            aRR, aRS, aRX, aRY, aRZ, aS0, aS1, aS2,
            aSf, aSg, aSh, aSi, aSj, aSk, aTb, aTc,
            aTd, aTe, aTf, aTg, aTh, aTi, aTW, aTX,
            aTY, aTZ,
        }=self.eval_common_stamp_values(ctx);
        let p=&(*self.params);
        let m=self.multiplicity;
        let multiplicity=m;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let Mi=0.0;let Ml=0.0;let Mo=0.0;let Mr=0.0;let Mv=0.0;let Mz=0.0;let MH=0.0;let MN=0.0;let MY=0.0;let aRE=1.0;let aTr=(sf[15]*(aRE*aTb));let aU6=(sf[15]*(aRE*aTY));

        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(3),
            &[3, 4, 5, 6, 7, 9],
            &[(sf[15]*(aRy*aRE)), (sf[15]*(aRz*aRE)), (sf[15]*(aRA*aRE)), (sf[15]*(aRB*aRE)), (sf[15]*(aRC*aRE)), (sf[15]*(aRD*aRE))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2_local(
            Some(4),
            Some(3),
            3,
            multiplicity * ((sf[15]*(aRE*aRR))),
            4,
            multiplicity * ((sf[15]*(aRE*aRS))),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(7),
            &[3, 4, 5, 6, 7, 9],
            &[(sf[15]*(aRE*aRX)), (sf[15]*(aRE*aRY)), (sf[15]*(aRE*aRZ)), (sf[15]*(aRE*aS0)), (sf[15]*(aRE*aS1)), (sf[15]*(aRE*aS2))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            Some(5),
            &[3, 4, 5, 6, 7, 9],
            &[(sf[15]*(aRE*aSf)), (sf[15]*(aRE*aSg)), (sf[15]*(aRE*aSh)), (sf[15]*(aRE*aSi)), (sf[15]*(aRE*aSj)), (sf[15]*(aRE*aSk))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(2),
            1,
            multiplicity * ((sf[15]*(aRE*sf[372]))),
            2,
            multiplicity * ((sf[15]*(aRE*sf[373]))),
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(0),
            0,
            multiplicity * ((sf[15]*(aRE*sf[374]))),
            1,
            multiplicity * ((sf[15]*(aRE*sf[375]))),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(8),
            &[0, 1, 3, 4, 5, 6, 7, 8, 9],
            &[aTr, (sf[15]*(aRE*aTc)), (sf[15]*(aRE*aTd)), aTr, (sf[15]*(aRE*aTe)), (sf[15]*(aRE*aTf)), (sf[15]*(aRE*aTg)), (sf[15]*(aRE*aTh)), (sf[15]*(aRE*aTi))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            Some(9),
            &[4, 5, 6, 7, 9],
            &[(sf[15]*(aRE*aTW)), (sf[15]*(aRE*aTX)), aU6, aU6, (sf[15]*(aRE*aTZ))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(3),
            &[3, 4, 5, 6, 7, 9, 10],
            &[(MY*aON), (MY*aOO), (MY*aOP), (MY*aOQ), (MY*aOR), (MY*aOS), (LA*aRE)],
            &[],
            &[],
            multiplicity,
        );
    }
}
