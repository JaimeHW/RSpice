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
    b: f64, d: f64, H: f64, I: f64, X: f64, bL: f64, 
    gf: f64, gj: f64, gv: f64, gV: f64, ln: f64, lr: f64, 
    lt: f64, ly: f64, lB: f64, lE: f64, lJ: f64, lR: f64, 
    lU: f64, lX: f64, m1: f64, mh: f64, mE: f64, mF: f64, 
    mH: f64, mK: bool, mL: f64, n1: f64, n3: f64, n6: bool, 
    n7: f64, nn: f64, np: f64, ns: bool, nt: f64, oE: f64, 
    qC: f64, rA: f64, rZ: f64, s2: f64, s5: f64, sw: f64, 
    tO: f64, uo: f64, up: f64, uu: f64, uv: f64, uO: f64, 
    uQ: f64, uT: bool, uU: f64, v3: f64, vz: f64, vB: f64, 
    vD: f64, vI: bool, vJ: f64, vQ: f64, vR: f64, vT: f64, 
    vY: bool, w0: f64, wQ: f64, wS: f64, wU: f64, wZ: bool, 
    x0: f64, xr: f64, xE: f64, xR: f64, y4: f64, yb: f64, 
    yc: f64, yf: f64, yh: f64, ym: bool, yn: f64, yt: f64, 
    yx: f64, yA: f64, yI: f64, yJ: f64, yK: f64, yM: f64, 
    yO: f64, yS: f64, yT: f64, yV: f64, yY: f64, z0: f64, 
    z1: bool, z6: bool, z7: f64, zJ: f64, zL: f64, zN: f64, 
    zO: f64, zR: f64, zT: f64, zY: bool, zZ: f64, A4: f64, 
    A7: f64, A9: f64, Ah: f64, Ai: f64, Aj: f64, Al: f64, 
    Aq: f64, Ar: f64, At: f64, Av: f64, Ax: f64, Ay: bool, 
    AD: bool, AE: f64, CL: f64, D9: f64, Dr: f64, DO: f64, 
    F0: f64, Fc: f64, Fp: bool, Fq: bool, Fr: f64, Fu: bool, 
    Fv: f64, Fz: f64, FA: f64, FC: f64, FG: f64, FI: f64, 
    FN: bool, FO: f64, G3: bool, HM: bool, HN: f64, HP: f64, 
    HR: f64, HT: f64, HV: f64, HW: bool, HY: bool, I6: f64, 
    I9: bool, Ia: f64, Ib: f64, Ih: bool, Ij: f64, Ik: f64, 
    Io: f64, Iq: f64, It: f64, Iv: f64, IA: bool, IB: f64, 
    OD: f64, P9: f64, PY: f64, Q1: f64, Q4: f64, Q7: f64, 
    Qa: f64, Qe: f64, Qi: f64, Qq: f64, Qw: f64, QH: f64, 
    QX: f64, QY: f64, Rn: f64, Ro: f64, Rp: f64, Rq: f64, 
    TQ: f64, TR: f64, TS: f64, Yv: f64, Yw: f64, Yx: f64, 
    a0T: f64, a0U: f64, a0V: f64, a1A: f64, a1B: f64, a1C: f64, 
    a1J: f64, a1K: f64, a1L: f64, a1S: f64, a1T: f64, a1U: f64, 
    a2q: f64, a2r: f64, a5k: f64, a5l: f64, a5m: f64, a6O: f64, 
    a6P: f64, a6Q: f64, a6R: f64, a6U: f64, a6X: f64, a70: f64, 
    a73: f64, a74: f64, a75: f64, a76: f64, a78: f64, a7c: f64, 
    a7f: f64, a7N: f64, a7O: f64, a8L: f64, a8M: f64, aaV: f64, 
    aaW: f64, aaX: f64, abQ: f64, abR: f64, abS: f64, ac5: f64, 
    ac6: f64, ac7: f64, acs: f64, act: f64, acu: f64, acv: f64, 
    acw: f64, acN: f64, acO: f64, acP: f64, acQ: f64, acR: f64, 
    an3: f64, an4: f64, an5: f64, an6: f64, aor: f64, aos: f64, 
    aot: f64, aou: f64, aov: f64, aow: f64, aoJ: f64, aoK: f64, 
    aoL: f64, aoM: f64, aoN: f64, aoO: f64, aoP: f64, aoQ: f64, 
    ar2: f64, ar3: f64, ar4: f64, ar5: f64, ar6: f64, ar7: f64, 
    ar8: f64, ar9: f64, ara: f64, awE: f64, awF: f64, awG: f64, 
    awH: f64, aYO: f64, aYP: f64, aYQ: f64, aYR: f64, aYS: f64, 
    aYT: f64, b2d: f64, b2e: f64, b2f: f64, b2g: f64, b2h: f64, 
    b2i: f64, b2w: f64, b2x: f64, b2C: f64, b2D: f64, b2E: f64, 
    b2F: f64, b2G: f64, b2H: f64, b2U: f64, b2V: f64, b30: f64, 
    b31: f64, b32: f64, b33: f64, b34: f64, b35: f64, b3Y: f64, 
    b3Z: f64, b40: f64, b41: f64, b42: f64, b43: f64, b44: f64, 
    b45: f64, b46: f64, b4M: f64, b4N: f64, b4O: f64, b4P: f64, 
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let b=1.0;let d=0.0;let H=0.001;let I=2.0;let X=0.1;let bL=3.0;let gf=1e-6;let gj=0.5;let gv=4.0;let gV=6.0;let lk=ctx.node_voltage(n[6]);let ll=ctx.node_voltage(n[7]);let ln=(sf[0]*(lk-ll));let lo=ctx.node_voltage(n[8]);let lq=(sf[0]*(lk-lo));let lr=ctx.node_voltage(n[4]);let lt=(sf[0]*(lk-lr));let lu=ctx.node_voltage(n[5]);let lw=(sf[0]*(lu-lr));let ly=(sf[0]*(lu-lk));let lB=(sf[0]*(ctx.node_voltage(n[3])-ll));let lD=(sf[0]*(ll-lo));let lE=ctx.node_voltage(n[2]);let lH=ctx.node_voltage(n[1]);let lJ=(sf[0]*(lH-lu));let lO=(sf[0]*(lH-ctx.node_voltage(n[0])));let lP=ctx.node_voltage(n[10]);let lR=(sf[0]*(lP-ll));let lU=(sf[0]*(ctx.node_voltage(n[9])-lP));let lX=(((lq+ly)-lD)-lR);let m1=((lX+(lJ+(-lO)))-lU);let m2=(lO+m1);let m3=(lB-lR);let m5=(sf[412]*lq);let m8=(if (m5<sf[214]){b}else{d});let m9=(m5).exp();let mb=(!(m8!=0.0));let md=(if mb{sf[215]}else{d});let mh=(if mb{(md*(b+(m5-sf[214])))}else{(if (m8!=0.0){m9}else{d})});let mi=(sf[412]*lt);let mj=(mi/sf[642]);let ml=(if (mj<sf[214]){b}else{d});let mm=(mj).exp();let mo=(!(ml!=0.0));let mp=(if mo{sf[215]}else{md});let mt=(if mo{(mp*(b+(mj-sf[214])))}else{(if (ml!=0.0){mm}else{d})});let mu=(sf[412]*lX);let mw=(if (mu<sf[214]){b}else{d});let mx=(mu).exp();let mz=(!(mw!=0.0));let mA=(if mz{sf[215]}else{mp});let mE=(if mz{(mA*(b+(mu-sf[214])))}else{(if (mw!=0.0){mx}else{d})});let mF=(sf[412]*ly);let mH=(if (mF<sf[214]){b}else{d});let mK=(!(mH!=0.0));let mL=(if mK{sf[215]}else{mA});let mQ=(sf[412]*m2);let mS=(if (mQ<sf[214]){b}else{d});let mT=(mQ).exp();let mV=(!(mS!=0.0));let mW=(if mV{sf[215]}else{mL});let n0=(if mV{(mW*(b+(mQ-sf[214])))}else{(if (mS!=0.0){mT}else{d})});let n1=(sf[412]*lB);let n3=(if (n1<sf[214]){b}else{d});let n6=(!(n3!=0.0));let n7=(if n6{sf[215]}else{mW});let nc=(sf[412]*(m3-lU));let ne=(if (nc<sf[214]){b}else{d});let nf=(nc).exp();let nh=(!(ne!=0.0));let ni=(if nh{sf[215]}else{n7});let nm=(if nh{(ni*(b+(nc-sf[214])))}else{(if (ne!=0.0){nf}else{d})});let nn=(sf[412]*m3);let np=(if (nn<sf[214]){b}else{d});let ns=(!(np!=0.0));let nt=(if ns{sf[215]}else{ni});let nz=(sf[412]*(m2-sf[500]));let nB=(if (nz<sf[214]){b}else{d});let nC=(nz).exp();let nE=(!(nB!=0.0));let nF=(if nE{sf[215]}else{nt});let nL=(sf[412]*(lX-sf[500]));let nN=(if (nL<sf[214]){b}else{d});let nO=(nL).exp();let nQ=(!(nN!=0.0));let nR=(if nQ{sf[215]}else{nF});let nX=(sf[412]*(lq-sf[500]));let nZ=(if (nX<sf[214]){b}else{d});let o0=(nX).exp();let o2=(!(nZ!=0.0));let o3=(if o2{sf[215]}else{nR});let o7=(if o2{(o3*(b+(nX-sf[214])))}else{(if (nZ!=0.0){o0}else{d})});let o9=(sf[412]*(ln-sf[500]));let ob=(if (o9<sf[214]){b}else{d});let oc=(o9).exp();let oe=(!(ob!=0.0));let of=(if oe{sf[215]}else{o3});let oj=(if oe{(of*(b+(o9-sf[214])))}else{(if (ob!=0.0){oc}else{d})});let om=((b+(gv*o7))).sqrt();let op=((b+(gv*oj))).sqrt();let oq=(I*oj);let or=(b+op);let os=(oq/or);let ov=(if (os<sf[216]){b}else{d});let ow=(if (ov!=0.0){sf[216]}else{os});let oy=(b+om);let oz=(oy/or);let oC=(sf[411]*((om-op)-(oz).ln()));let oE=((lD+oC)/sf[618]);let oG=(if (oE>d){b}else{d});let oH=100.0;let oJ=(if (ln<oH){b}else{d});let oK=((oG!=0.0)&&(oJ!=0.0));let oN=((oG!=0.0)&&(!(oJ!=0.0)));let oP=(b+(ln-oH));let oV=(sf[618]*(gj*oE));let oX=(b+(sf[412]*oV));let p2=(if (oG!=0.0){((sf[500]+(sf[865]*(oX).ln()))-(if oN{(oH+(oP).ln())}else{(if oK{ln}else{d})}))}else{d});let p5=(if (oG!=0.0){sf[866]}else{d});let p7=(if (oG!=0.0){(p5*p5)}else{gf});let pb=(if (p2<d){b}else{d});let pc=((oG!=0.0)&&(pb!=0.0));let pd=(gj*p7);let pf=((p7+(if (oG!=0.0){(p2*p2)}else{sf[670]}))).sqrt();let pg_=(pf-p2);let pk=((oG!=0.0)&&(!(pb!=0.0)));let pn=(if pk{(gj*(p2+pf))}else{(if pc{(pd/pg_)}else{d})});let pr=(pn+sf[219]);let ps=(pn*pr);let pv=(sf[218]*(pn+sf[867]));let px=(if (oG!=0.0){(ps/pv)}else{d});let pz=(if (oG!=0.0){(oE/px)}else{d});let pD=(if (oG!=0.0){((pz-b)/sf[220])}else{sf[649]});let pF=(if (pz<b){b}else{d});let pG=((oG!=0.0)&&(pF!=0.0));let pH=(pD).exp();let pI=(b+pH);let pO=((oG!=0.0)&&(!(pF!=0.0)));let pQ=((-pD)).exp();let pR=(b+pQ);
        let q4=(if (oG!=0.0){((if pO{(pz+(sf[220]*(pR).ln()))}else{(if pG{(b+(sf[220]*(pI).ln()))}else{d})})/sf[226])}else{d});let q6=(if (oG!=0.0){(pn/sf[219])}else{d});let q7=(gv*q4);let q8=(q6*q7);let q9=(b+q6);let qc=((b+(q8*q9))).sqrt();let qd=(b+qc);let qe=(I*q4);let qf=(q9*qe);let qh=(if (oG!=0.0){(qd/qf)}else{d});let qj=(ow*qh);let qk=((b-qh)+qj);let ql=(b+qj);let qn=(if (oG!=0.0){(qk/ql)}else{d});let qq=(if (oG!=0.0){(sf[412]*(oV*qn))}else{d});let qt=(b+(ow+qq));let qw=(if (oG!=0.0){((I*qq)+(ow*qt))}else{d});let qz=(if (oG!=0.0){(gj*(qq-b))}else{d});let qC=(if (oG!=0.0){(qw+(qz*qz))}else{d});let qE=(if (qq>=b){b}else{d});let qF=((oG!=0.0)&&(qE!=0.0));let qG=(qC).sqrt();let qK=((oG!=0.0)&&(!(qE!=0.0)));let qL=(qG-qz);let qN=(if qK{(qw/qL)}else{(if qF{(qz+qG)}else{d})});let qR=((oG!=0.0)&&((if (qN<sf[227]){b}else{d})!=0.0));let qS=(if qR{sf[227]}else{qN});let qT=(b+qS);let r2=(if (oG!=0.0){(sf[228]*(oE-sf[217]))}else{d});let r9=(((if (oG!=0.0){(oE*sf[871])}else{d})+(r2*r2))).sqrt();let rj=((oG!=0.0)&&sb[20]);let rk=(I*oE);let rl=(oE+px);let rq=(oE*sf[217]);let rr=(oE+sf[217]);let rw=(!(oG!=0.0));let rx=(I*o7);let rA=(if rw{mh}else{(if (oG!=0.0){((qS*qT)*sf[869])}else{d})});let rM=(if (((lD).abs()<sf[873])||((oC).abs()<(sf[874]*(om+op)))){b}else{d});let rN=(rw&&(rM!=0.0));let rO=(ow+(if rw{(rx/oy)}else{qS}));let rQ=(if rN{(gj*rO)}else{d});let rR=(b+rQ);let rV=(rw&&(!(rM!=0.0)));let rX=((lq+oC)-ln);let rZ=(if rV{(oC/rX)}else{(if rN{(rQ/rR)}else{qn})});let s1=(if rw{sf[872]}else{(if rj{(sf[538]*(X+(rk/rl)))}else{(if ((oG!=0.0)&&(sf[230]!=0.0)){sf[872]}else{d})})});let s2=(if rw{oE}else{(if (oG!=0.0){(rq/rr)}else{d})});let s5=(if rw{(b-(s2/sf[217]))}else{(if (oG!=0.0){(sf[217]/rr)}else{d})});let sc=((lt-sf[875])/sf[876]);let se=(if (lt<sf[875]){b}else{d});let sf_=(sc).exp();let sg=(b+sf_);let sl=(!(se!=0.0));let sn=((-sc)).exp();let so=(b+sn);let ss=(if sl{(sf[875]-(sf[876]*(so).ln()))}else{(if (se!=0.0){(lt-(sf[876]*(sg).ln()))}else{d})});let su=(b-(sf[579]*ss));let sw=f64::powf(su,sf[234]);let sC=((sf[877]*(b-sw))+(bL*(lt-ss)));let sP=(if sb[26]{lq}else{(if sb[24]{(ln+(if rw{lD}else{(if (oG!=0.0){(r2+r9)}else{d})}))}else{(if (sf[236]!=0.0){ln}else{d})})});let sX=(sP-sf[883]);let sY=(sX/s1);let t0=(if (sP<sf[883]){b}else{d});let t1=(sY).exp();let t2=(b+t1);let t3=(t2).ln();let t7=(!(t0!=0.0));let t9=((-sY)).exp();let ta=(b+t9);let tb=(ta).ln();let te=(if t7{(sf[883]-(s1*tb))}else{(if (t0!=0.0){(sP-(s1*t3))}else{d})});let tg=f64::powf(s5,sf[239]);let tk=(b-(te/sf[538]));let tl=f64::powf(tk,sf[240]);let tp=(sf[880]*tg);let tq=(sP-te);let tv=((sf[879]*((sf[884]*(b-(tg*tl)))+(tp*tq)))+(sf[595]*ln));let ty=(mt*sf[886]);let tA=((b+ty)).sqrt();let tB=(b+tA);let tC=(ty/tB);let tE=f64::powf(rA,sf[887]);let tF=(sf[886]*tE);let tH=((b+tF)).sqrt();let tI=(b+tH);let tJ=(tF/tI);let tN=(b+(sC/sf[804]));let tO=(tv/sf[802]);let tP=(tN+tO);let u0=((if sb[28]{(sf[412]*(sf[849]*tN))}else{d})).exp();let u1=((if sb[28]{(sf[412]*(sf[849]*((-tv)/sf[802])))}else{d})).exp();let u7=(if sb[28]{((u0-u1)/sf[890])}else{(if (sf[241]!=0.0){tP}else{d})});let u8_=0.010000000000000002;let u9=(u7*u7);let ub=(if (u7<d){b}else{d});let uc=0.005000000000000001;let ue=((u8_+u9)).sqrt();let uf=(ue-u7);let ui=(!(ub!=0.0));let ul=(if ui{(gj*(u7+ue))}else{(if (ub!=0.0){(uc/uf)}else{d})});let uo=(b+(gj*(tC+tJ)));let up=(ul*uo);let us=(tE*sf[891]);let ut=(sf[687]*mt);let uu=(ut-us);let uv=(uu/up);let uw=0.0001;let ux=(lt/uw);let uy=(lt<d);let uz=(if uy{b}else{d});let uA=(ux).exp();let uB=(b+uA);let uF=(!(uz!=0.0));let uH=((-ux)).exp();let uI=(b+uH);let uM=(if uF{(lt+(uw*(uI).ln()))}else{(if (uz!=0.0){(uw*(uB).ln())}else{d})});let uO=(uM/sf[243]);let uQ=(if (uO<sf[214]){b}else{d});let uT=(!(uQ!=0.0));let uU=(if uT{sf[215]}else{of});let v3=((lt-sf[244])/H);let vp=(mi/sf[148]);let vr=(if (vp<sf[214]){b}else{d});let vs=(vp).exp();let vu=(!(vr!=0.0));let vv=(if vu{sf[215]}else{uU});let vz=(if vu{(vv*(b+(vp-sf[214])))}else{(if (vr!=0.0){vs}else{uM})});let vB=(sf[412]*(lt-sf[558]));let vD=(if (vB<sf[214]){b}else{d});let vI=((sf[154]!=0.0)&&(!(vD!=0.0)));
        let vJ=(if vI{sf[215]}else{vv});let vQ=((uv/sf[687])-1000.0);let vR=40.0;let vT=(if (vQ<vR){b}else{d});let vY=((sf[154]!=0.0)&&(!(vT!=0.0)));let w0=(if vY{2.3538526683702e17}else{vJ});let wF=(sf[412]*lw);let wG=(wF/sf[152]);let wI=(if (wG<sf[214]){b}else{d});let wJ=(wG).exp();let wL=(!(wI!=0.0));let wM=(if wL{sf[215]}else{w0});let wQ=(if wL{(wM*(b+(wG-sf[214])))}else{(if (wI!=0.0){wJ}else{vz})});let wS=(sf[412]*(lw-sf[558]));let wU=(if (wS<sf[214]){b}else{d});let wZ=((sf[154]!=0.0)&&(!(wU!=0.0)));let x0=(if wZ{sf[215]}else{wM});let xh=(mi/sf[135]);let xj=(if (xh<sf[214]){b}else{d});let xk=(xh).exp();let xm=(!(xj!=0.0));let xn=(if xm{sf[215]}else{x0});let xr=(if xm{(xn*(b+(xh-sf[214])))}else{(if (xj!=0.0){xk}else{wQ})});let xu=(wF/sf[170]);let xw=(if (xu<sf[214]){b}else{d});let xx=(xu).exp();let xz=(!(xw!=0.0));let xA=(if xz{sf[215]}else{xn});let xE=(if xz{(xA*(b+(xu-sf[214])))}else{(if (xw!=0.0){xx}else{xr})});let xH=(mu/sf[141]);let xJ=(if (xH<sf[214]){b}else{d});let xK=(xH).exp();let xM=(!(xJ!=0.0));let xN=(if xM{sf[215]}else{xA});let xR=(if xM{(xN*(b+(xH-sf[214])))}else{(if (xJ!=0.0){xK}else{xE})});let xU=(wF/sf[174]);let xW=(if (xU<sf[214]){b}else{d});let xX=(xU).exp();let xZ=(!(xW!=0.0));let y0=(if xZ{sf[215]}else{xN});let y4=(if xZ{(y0*(b+(xU-sf[214])))}else{(if (xW!=0.0){xX}else{xR})});let yb=(if (uy&&sb[36]){b}else{d});let yc=(I*sw);let yf=(sf[769]*(b-(sf[21]/yc)));let yh=(if (yf<sf[214]){b}else{d});let ym=((yb!=0.0)&&(!(yh!=0.0)));let yn=(if ym{sf[215]}else{y0});let yt=(if (yb!=0.0){(sf[579]*lt)}else{sf[800]});let yv=1e-30;let yx=(((yt*yt)+yv)).sqrt();let yA=f64::powf(yx,sf[249]);let yI=(gV*yt);let yJ=(yt*yI);let yK=(yt+sf[252]);let yM=((sf[19]*(sf[251]-((bL*yt)*sf[252])))-(yJ*yK));let yO=0.16666666666666666;let yS=(sf[769]*(sf[21]*lt));let yT=(sf[436]*(if (yb!=0.0){((yA*yM)*yO)}else{d}));let yV=(if (yb!=0.0){(yS/yT)}else{yt});let yW=-0.001;let yY=(if (yV<yW){b}else{d});let z0=(if (yV<sf[214]){b}else{d});let z1=((yb!=0.0)&&(yY!=0.0));let z6=(z1&&(!(z0!=0.0)));let z7=(if z6{sf[215]}else{yn});let zJ=(if (sb[39]&&(ln<d)){b}else{d});let zK=(sf[580]*ln);let zL=(b-zK);let zN=(if (zJ!=0.0){f64::powf(zL,sf[240])}else{d});let zO=(I*zN);let zR=(sf[789]*(b-(sf[53]/zO)));let zT=(if (zR<sf[214]){b}else{d});let zY=((zJ!=0.0)&&(!(zT!=0.0)));let zZ=(if zY{sf[215]}else{z7});let A4=(if (zJ!=0.0){zK}else{sf[780]});let A7=((yv+(A4*A4))).sqrt();let A9=f64::powf(A7,sf[253]);let Ah=(gV*A4);let Ai=(A4*Ah);let Aj=(A4+sf[256]);let Al=((sf[51]*(sf[255]-((bL*A4)*sf[256])))-(Ai*Aj));let Aq=(sf[789]*(sf[53]*ln));let Ar=(sf[457]*(if (zJ!=0.0){(yO*(A9*Al))}else{d}));let At=(if (zJ!=0.0){(Aq/Ar)}else{A4});let Av=(if (At<yW){b}else{d});let Ax=(if (At<sf[214]){b}else{d});let Ay=((zJ!=0.0)&&(Av!=0.0));let AD=(Ay&&(!(Ax!=0.0)));let AE=(if AD{sf[215]}else{zZ});let B9=(mE*sf[886]);let Ba=(gv*(if nQ{(nR*(b+(nL-sf[214])))}else{(if (nN!=0.0){nO}else{d})}));let Bb=(B9-sf[886]);let Bd=((b+B9)).sqrt();let Be=(b+Bd);let Bh=((b+Ba)).sqrt();let Bi=(b+Bh);let CE=(n0-b);let CF=(sf[906]*CE);let CI=((b+(n0*sf[898]))).sqrt();let CJ=(b+CI);let CL=(if (sf[266]!=0.0){(CF/CJ)}else{d});let CR=(sf[907]*(n0-nm));let CY=((b+(sf[909]*(n0+(nm*sf[261]))))).sqrt();let CZ=(b+CY);let D3=(CE*sf[907]);let D6=((b+(n0*sf[909]))).sqrt();let D7=(b+D6);let D9=(if sb[46]{(D3/D7)}else{(if sb[45]{(CR/CZ)}else{d})});let Dn=(if sb[48]{(m2-sf[918])}else{d});let Dr=(if sb[48]{(Dn*Dn)}else{u9});let Dt=(if (Dn<d){b}else{d});let Du=(sb[48]&&(Dt!=0.0));let Dx=((sf[271]+Dr)).sqrt();let Dy=(Dx-Dn);let DC=(sb[48]&&(!(Dt!=0.0)));let DF=(if DC{(gj*(Dn+Dx))}else{(if Du{(sf[272]/Dy)}else{d})});let DJ=(DF+(sf[913]+(sf[611]*(CL+D9))));let DO=(if sb[50]{b}else{(if sb[48]{(DF/DJ)}else{b})});let ER=(if (tP<d){b}else{d});let ET=((u8_+(tP*tP))).sqrt();let EU=(ET-tP);let EX=(!(ER!=0.0));let F0=(if EX{(gj*(tP+ET))}else{(if (ER!=0.0){(uc/EU)}else{d})});let Fc=(if (uv>d){b}else{d});let Fi=(if (ln<sf[294]){b}else{d});let Fl=((-uv)/sf[295]);let Fn=(if (Fl<sf[214]){b}else{d});let Fp=((Fi!=0.0)&&((Fc!=0.0)&&(sf[293]!=0.0)));let Fq=((Fn!=0.0)&&Fp);let Fr=(Fl).exp();let Fu=(Fp&&(!(Fn!=0.0)));
        let Fv=(if Fu{sf[215]}else{AE});let Fz=(if Fu{(Fv*(b+(Fl-sf[214])))}else{(if Fq{Fr}else{d})});let FA=(sf[294]-ln);let FC=(if Fp{(Fz*FA)}else{d});let FG=(sf[919]*f64::powf(FC,sf[296]));let FI=(if (FG<sf[214]){b}else{d});let FN=(Fp&&(!(FI!=0.0)));let FO=(if FN{sf[215]}else{Fv});let G3=((Fc!=0.0)&&sb[55]);let HM=((Fi!=0.0)&&((sf[311]!=0.0)&&(G3&&sb[59])));let HN=f64::powf(FA,sf[296]);let HP=(uv+sf[312]);let HR=(b-(uv/HP));let HT=f64::powf(HR,sf[313]);let HV=(if HM{(HN*HT)}else{d});let HW=((sf[305]!=0.0)&&HM);let HY=(sb[57]&&HM);let I2=(if HY{((uv-sf[314])/sf[312])}else{d});let I6=(if HY{((I2-b)/sf[315])}else{v3});let I8=(if (I2<b){b}else{d});let I9=(HY&&(I8!=0.0));let Ia=(I6).exp();let Ib=(b+Ia);let Ih=(HY&&(!(I8!=0.0)));let Ij=((-I6)).exp();let Ik=(b+Ij);let Io=(if Ih{(I2+(sf[315]*(Ik).ln()))}else{(if I9{(b+(sf[315]*(Ib).ln()))}else{d})});let Iq=f64::powf(Io,sf[316]);let It=(sf[919]*(if HY{(HV*Iq)}else{(if HW{HV}else{d})}));let Iv=(if (It<sf[214]){b}else{d});let IA=(HM&&(!(Iv!=0.0)));let IB=(if IA{sf[215]}else{FO});let JB=((lw-sf[875])/sf[876]);let JD=(if (lw<sf[875]){b}else{d});let JE=(JB).exp();let JF=(b+JE);let JK=(!(JD!=0.0));let JM=((-JB)).exp();let JN=(b+JM);let JR=(if JK{(sf[875]-(sf[876]*(JN).ln()))}else{(if (JD!=0.0){(lw-(sf[876]*(JF).ln()))}else{d})});let JU=(b-(sf[579]*JR));let K7=(tC*sf[927]);let K8=(F0*K7);let K9=(tJ*sf[927]);let Ka=(F0*K9);let Kc=((lX-sf[883])/sf[872]);let Ke=(if (lX<sf[883]){b}else{d});let Kf=(Kc).exp();let Kg=(b+Kf);let Kl=(!(Ke!=0.0));let Kn=((-Kc)).exp();let Ko=(b+Kn);let Ks=(if Kl{(sf[883]-(sf[872]*(Ko).ln()))}else{(if (Ke!=0.0){(lX-(sf[872]*(Kg).ln()))}else{d})});let Ku=(b-(Ks/sf[538]));let KJ=((m2-sf[883])/sf[872]);let KL=(if (m2<sf[883]){b}else{d});let KM=(KJ).exp();let KN=(b+KM);let KS=(!(KL!=0.0));let KU=((-KJ)).exp();let KV=(b+KU);let KZ=(if KS{(sf[883]-(sf[872]*(KV).ln()))}else{(if (KL!=0.0){(m2-(sf[872]*(KN).ln()))}else{d})});let L1=(b-(KZ/sf[538]));let Lk=((lB-sf[929])/sf[928]);let Lm=(if (lB<sf[929]){b}else{d});let Ln=(Lk).exp();let Lo=(b+Ln);let Lt=(!(Lm!=0.0));let Lv=((-Lk)).exp();let Lw=(b+Lv);let LA=(if Lt{(sf[929]-(sf[928]*(Lw).ln()))}else{(if (Lm!=0.0){(lB-(sf[928]*(Lo).ln()))}else{d})});let LE=(b-(LA/sf[578]));let LT=(lt/sf[935]);let LV=(if (LT<sf[214]){b}else{d});let LW=(LT).exp();let LY=(!(LV!=0.0));let LZ=(if LY{sf[215]}else{IB});let M4=(sf[934]*(if LY{(LZ*(b+(LT-sf[214])))}else{(if (LV!=0.0){LW}else{y4})}));let M9=(rZ*sf[939]);let Ma=(I+rO);let Mp=(sf[412]*((lX-sf[519])/sf[331]));let Mr=(if (Mp<sf[214]){b}else{d});let Mt=((Mr!=0.0)&&sb[64]);let Mu=(Mp).exp();let Mx=(sb[64]&&(!(Mr!=0.0)));let My=(if Mx{sf[215]}else{LZ});let ME=(mE*sf[941]);let MH=((b+(gv*(if Mx{(My*(b+(Mp-sf[214])))}else{(if Mt{Mu}else{d})})))).sqrt();let MI=(b+MH);let MK=(if sb[64]{(ME/MI)}else{(if (sf[330]!=0.0){((sf[940]*(((Bb/Be)*sf[926])+((Ba/Bi)*sf[938])))/sf[833])}else{d})});let MT=(if sb[68]{(n0*sf[886])}else{d});let MU=(MT-sf[886]);let MW=((b+MT)).sqrt();let MX=(b+MW);let N1=(if sb[68]{(gv*(if nE{(nF*(b+(nz-sf[214])))}else{(if (nB!=0.0){nC}else{d})}))}else{d});let N3=((b+N1)).sqrt();let N4=(b+N3);let Ng=(sf[412]*(m2-sf[519]));let Ni=(if (Ng<sf[214]){b}else{d});let Nk=((Ni!=0.0)&&sb[69]);let Nl=(Ng).exp();let No=(sb[69]&&(!(Ni!=0.0)));let Np=(if No{sf[215]}else{My});let Nv=(n0*sf[943]);let Ny=((b+(gv*(if No{(Np*(b+(Ng-sf[214])))}else{(if Nk{Nl}else{d})})))).sqrt();let Nz=(b+Ny);let NB=(if sb[69]{(Nv/Nz)}else{(if sb[68]{((sf[942]*((sf[926]*(if sb[68]{(MU/MX)}else{d}))+(sf[938]*(if sb[68]{(N1/N4)}else{d}))))/sf[833])}else{d})});let NK=(if (sf[335]!=0.0){(f64::powf(su,sf[336])-bL)}else{d});let NL=(if (sf[335]!=0.0){sc}else{d});let NN=(if (NL<d){b}else{d});let NO=((sf[335]!=0.0)&&(NN!=0.0));let NP=(NL).exp();let NQ=(b+NP);let NU=((sf[335]!=0.0)&&(!(NN!=0.0)));let NW=((-NL)).exp();let NX=(b+NW);let NZ=(if NU{(NW/NX)}else{(if NO{(b/NQ)}else{d})});let O6=((sf[412]*ty)/sf[642]);let O7=(gj/tA);let O9=(if (sf[335]!=0.0){(O6*O7)}else{d});let Oa=(F0*sf[927]);let Of=(ly*0.2);
        let Oh=((if (sf[335]!=0.0){(M4/sf[935])}else{d})+((if (sf[335]!=0.0){(sf[923]*(if (sf[335]!=0.0){(bL+(NK*NZ))}else{d}))}else{d})+(if (sf[335]!=0.0){(O9*Oa)}else{d})));let Oq=(if (sf[335]!=0.0){(K8+(M4*sf[337]))}else{d});let Oz=(if sb[71]{K8}else{(if (sf[335]!=0.0){(Oq*sf[340])}else{d})});let OA=(if sb[71]{Ka}else{(if (sf[335]!=0.0){(Ka+(Oq*sf[339]))}else{d})});let OC=(us+ut);let OD=(OC/up);let ON=(if (OD>d){b}else{d});let OO=(Oz+OA);let OR=(!(ON!=0.0));let OS=(sf[829]*F0);let OU=(if OR{(up*OS)}else{(if (ON!=0.0){(OO/OD)}else{d})});let P9=(if sb[79]{d}else{(if sb[77]{(OU*sf[346])}else{(if (sf[344]!=0.0){(sf[339]*OU)}else{d})})});let PY=(sf[0]*((if sb[71]{M4}else{(if (sf[335]!=0.0){(M4*sf[338])}else{d})})+((sC*sf[923])+Oz)));let Q1=(sf[0]*(sf[924]*((sf[877]*(b-f64::powf(JU,sf[234])))+(bL*(lw-JR)))));let Q4=(sf[0]*((M9*Ma)+((tv*sf[925])+OA)));let Q7=(sf[0]*(sf[588]*((sf[930]*(b-f64::powf(LE,sf[326])))+(I*(lB-LA)))));let Qa=(sf[0]*(if (sf[335]!=0.0){(Of*Oh)}else{d}));let Qe=((sf[0]*(lH-lE))*sf[349]);let Qi=(lO*sf[350]);let Qq=(sf[0]*((sf[6]*(sf[322]*(sf[594]*((sf[879]*((sf[884]*(b-f64::powf(L1,sf[240])))+(sf[880]*(m2-KZ))))+(sf[595]*m2)))))+(if (sf[332]!=0.0){(DO*NB)}else{d})));let Qw=(sf[0]*((sf[7]*((sf[594]*((sf[879]*((sf[884]*(b-f64::powf(Ku,sf[240])))+(sf[880]*(lX-Ks))))+(sf[595]*lX)))*sf[322]))+(if (sf[332]!=0.0){(sf[7]*MK)}else{MK})));let QH=ctx.node_voltage(n[11]);let QX=(if mb{(md*sf[944])}else{(if (m8!=0.0){(m9*sf[944])}else{d})});let QY=(if mb{(md*sf[945])}else{(if (m8!=0.0){(m9*sf[945])}else{d})});let R7=(if mo{(mp*sf[946])}else{(if (ml!=0.0){(mm*sf[946])}else{d})});let R8=(if mo{(mp*sf[947])}else{(if (ml!=0.0){(mm*sf[947])}else{d})});let Rn=(if mz{(mA*sf[944])}else{(if (mw!=0.0){(mx*sf[944])}else{d})});let Ro=(if mz{(mA*sf[948])}else{(if (mw!=0.0){(mx*sf[948])}else{d})});let Rp=(if mz{(mA*sf[949])}else{(if (mw!=0.0){(mx*sf[949])}else{d})});let Rq=(if mz{(mA*sf[945])}else{(if (mw!=0.0){(mx*sf[945])}else{d})});let RM=(if mV{(mW*sf[948])}else{(if (mS!=0.0){(mT*sf[948])}else{d})});let RN=(if mV{(mW*sf[950])}else{(if (mS!=0.0){(mT*sf[950])}else{d})});let RO=(if mV{(mW*sf[949])}else{(if (mS!=0.0){(mT*sf[949])}else{d})});let RP=(if mV{(mW*sf[945])}else{(if (mS!=0.0){(mT*sf[945])}else{d})});let S7=(if nh{(ni*sf[944])}else{(if (ne!=0.0){(nf*sf[944])}else{d})});let S8=(if nh{(ni*sf[949])}else{(if (ne!=0.0){(nf*sf[949])}else{d})});let S9=(if nh{(ni*sf[945])}else{(if (ne!=0.0){(nf*sf[945])}else{d})});let SY=(if o2{(o3*sf[944])}else{(if (nZ!=0.0){(o0*sf[944])}else{d})});let SZ=(if o2{(o3*sf[945])}else{(if (nZ!=0.0){(o0*sf[945])}else{d})});let T6=(if oe{(of*sf[944])}else{(if (ob!=0.0){(oc*sf[944])}else{d})});let T7=(if oe{(of*sf[945])}else{(if (ob!=0.0){(oc*sf[945])}else{d})});let Ta=(I*om);let Tb=((gv*SY)/Ta);let Tc=((gv*SZ)/Ta);let Tf=(I*op);let Tg=((gv*T6)/Tf);let Th=((gv*T7)/Tf);let Tn=(or*or);let Tt=(if (ov!=0.0){d}else{(((or*(I*T6))-(oq*Tg))/Tn)});let Tu=(if (ov!=0.0){d}else{(((or*(I*T7))-(oq*Th))/Tn)});let TL=(sf[411]*((Tb-Tg)-((((or*Tb)-(oy*Tg))/Tn)/oz)));let TM=(sf[411]*((-Th)-(((-(oy*Th))/Tn)/oz)));let TN=(sf[411]*(Tc-((Tc/or)/oz)));let TP=(sf[351]+TN);let TQ=(TL/sf[618]);let TR=((sf[0]+TM)/sf[618]);let TS=(TP/sf[618]);let U2=(sf[618]*(gj*TQ));let U3=(sf[618]*(gj*TR));let U4=(sf[618]*(gj*TS));let Ug=(if (oG!=0.0){((sf[865]*((sf[412]*U2)/oX))-(if oN{(sf[0]/oP)}else{(if oK{sf[0]}else{d})}))}else{d});let Uh=(if (oG!=0.0){((sf[865]*((sf[412]*U3)/oX))-(if oN{(sf[351]/oP)}else{(if oK{sf[351]}else{d})}))}else{d});let Ui=(if (oG!=0.0){(sf[865]*((sf[412]*U4)/oX))}else{d});let Uj=(p2*Ug);let Ul=(p2*Uh);let Un=(p2*Ui);let Us=(I*pf);let Ut=((if (oG!=0.0){(Uj+Uj)}else{d})/Us);let Uu=((if (oG!=0.0){(Ul+Ul)}else{d})/Us);let Uv=((if (oG!=0.0){(Un+Un)}else{d})/Us);let UB=(pg_*pg_);let US=(if pk{(gj*(Ug+Ut))}else{(if pc{((-(pd*(Ut-Ug)))/UB)}else{d})});let UT=(if pk{(gj*(Uh+Uu))}else{(if pc{((-(pd*(Uu-Uh)))/UB)}else{d})});let UU=(if pk{(gj*(Ui+Uv))}else{(if pc{((-(pd*(Uv-Ui)))/UB)}else{d})});let Va=(pv*pv);let Vk=(if (oG!=0.0){(((pv*((pr*US)+(pn*US)))-(ps*(sf[218]*US)))/Va)}else{d});
        let Vl=(if (oG!=0.0){(((pv*((pr*UT)+(pn*UT)))-(ps*(sf[218]*UT)))/Va)}else{d});let Vm=(if (oG!=0.0){(((pv*((pr*UU)+(pn*UU)))-(ps*(sf[218]*UU)))/Va)}else{d});let Vq=(px*px);let VA=(if (oG!=0.0){(((px*TQ)-(oE*Vk))/Vq)}else{d});let VB=(if (oG!=0.0){(((px*TR)-(oE*Vl))/Vq)}else{d});let VC=(if (oG!=0.0){(((px*TS)-(oE*Vm))/Vq)}else{d});let VG=(if (oG!=0.0){(VA/sf[220])}else{d});let VH=(if (oG!=0.0){(VB/sf[220])}else{d});let VI=(if (oG!=0.0){(VC/sf[220])}else{d});let Wg=(if (oG!=0.0){((if pO{(VA+(sf[220]*((pQ*(-VG))/pR)))}else{(if pG{(sf[220]*((pH*VG)/pI))}else{d})})/sf[226])}else{d});let Wh=(if (oG!=0.0){((if pO{(VB+(sf[220]*((pQ*(-VH))/pR)))}else{(if pG{(sf[220]*((pH*VH)/pI))}else{d})})/sf[226])}else{d});let Wi=(if (oG!=0.0){((if pO{(VC+(sf[220]*((pQ*(-VI))/pR)))}else{(if pG{(sf[220]*((pH*VI)/pI))}else{d})})/sf[226])}else{d});let Wm=(if (oG!=0.0){(US/sf[219])}else{d});let Wn=(if (oG!=0.0){(UT/sf[219])}else{d});let Wo=(if (oG!=0.0){(UU/sf[219])}else{d});let WK=(I*qc);let X3=(qf*qf);let Xd=(if (oG!=0.0){(((qf*(((q9*((q7*Wm)+(q6*(gv*Wg))))+(q8*Wm))/WK))-(qd*((qe*Wm)+(q9*(I*Wg)))))/X3)}else{d});let Xe=(if (oG!=0.0){(((qf*(((q9*((q7*Wn)+(q6*(gv*Wh))))+(q8*Wn))/WK))-(qd*((qe*Wn)+(q9*(I*Wh)))))/X3)}else{d});let Xf=(if (oG!=0.0){(((qf*(((q9*((q7*Wo)+(q6*(gv*Wi))))+(q8*Wo))/WK))-(qd*((qe*Wo)+(q9*(I*Wi)))))/X3)}else{d});let Xl=((qh*Tt)+(ow*Xd));let Xo=((qh*Tu)+(ow*Xe));let Xp=(ow*Xf);let Xw=(ql*ql);let XG=(if (oG!=0.0){(((ql*((-Xd)+Xl))-(qk*Xl))/Xw)}else{d});let XH=(if (oG!=0.0){(((ql*((-Xe)+Xo))-(qk*Xo))/Xw)}else{d});let XI=(if (oG!=0.0){(((ql*((-Xf)+Xp))-(qk*Xp))/Xw)}else{d});let XV=(if (oG!=0.0){(sf[412]*((qn*U2)+(oV*XG)))}else{d});let XW=(if (oG!=0.0){(sf[412]*((qn*U3)+(oV*XH)))}else{d});let XX=(if (oG!=0.0){(sf[412]*((qn*U4)+(oV*XI)))}else{d});let Yd=(if (oG!=0.0){((I*XV)+((qt*Tt)+(ow*(Tt+XV))))}else{d});let Ye=(if (oG!=0.0){((I*XW)+((qt*Tu)+(ow*(Tu+XW))))}else{d});let Yf=(if (oG!=0.0){((I*XX)+(ow*XX))}else{d});let Yj=(if (oG!=0.0){(gj*XV)}else{d});let Yk=(if (oG!=0.0){(gj*XW)}else{d});let Yl=(if (oG!=0.0){(gj*XX)}else{d});let Ym=(qz*Yj);let Yo=(qz*Yk);let Yq=(qz*Yl);let Yv=(if (oG!=0.0){(Yd+(Ym+Ym))}else{d});let Yw=(if (oG!=0.0){(Ye+(Yo+Yo))}else{d});let Yx=(if (oG!=0.0){(Yf+(Yq+Yq))}else{d});let Yy=(I*qG);let Yz=(Yv/Yy);let YA=(Yw/Yy);let YB=(Yx/Yy);let YO=(qL*qL);let Z1=(if qR{d}else{(if qK{(((qL*Yd)-(qw*(Yz-Yj)))/YO)}else{(if qF{(Yj+Yz)}else{d})})});let Z2=(if qR{d}else{(if qK{(((qL*Ye)-(qw*(YA-Yk)))/YO)}else{(if qF{(Yk+YA)}else{d})})});let Z3=(if qR{d}else{(if qK{(((qL*Yf)-(qw*(YB-Yl)))/YO)}else{(if qF{(Yl+YB)}else{d})})});let Zm=(if (oG!=0.0){(sf[228]*TQ)}else{d});let Zn=(if (oG!=0.0){(sf[228]*TR)}else{d});let Zo=(if (oG!=0.0){(sf[228]*TS)}else{d});let Zv=(r2*Zm);let Zx=(r2*Zn);let Zz=(r2*Zo);let ZE=(I*r9);let ZX=(rl*rl);let a0d=(sf[217]*TQ);let a0e=(sf[217]*TR);let a0f=(sf[217]*TS);let a0j=(rr*rr);let a0K=(oy*oy);let a0S=(if rw{(((oy*(I*SZ))-(rx*Tc))/a0K)}else{Z3});let a0T=(if rw{QX}else{(if (oG!=0.0){(sf[869]*((qT*Z1)+(qS*Z1)))}else{d})});let a0U=(if rw{d}else{(if (oG!=0.0){(sf[869]*((qT*Z2)+(qS*Z2)))}else{d})});let a0V=(if rw{QY}else{(if (oG!=0.0){(sf[869]*((qT*Z3)+(qS*Z3)))}else{d})});let a0W=(Tt+(if rw{(((oy*(I*SY))-(rx*Tb))/a0K)}else{Z1}));let a0X=(Tu+(if rw{d}else{Z2}));let a11=(if rN{(gj*a0W)}else{d});let a12=(if rN{(gj*a0X)}else{d});let a13=(if rN{(gj*a0S)}else{d});let a17=(rR*rR);let a1q=(rX*rX);let a1A=(if rV{(((rX*TL)-(oC*((sf[0]+TL)-sf[0])))/a1q)}else{(if rN{(((rR*a11)-(rQ*a11))/a17)}else{XG})});let a1B=(if rV{(((rX*TM)-(oC*(TM-sf[351])))/a1q)}else{(if rN{(((rR*a12)-(rQ*a12))/a17)}else{XH})});let a1C=(if rV{(((rX*TN)-(oC*TP))/a1q)}else{(if rN{(((rR*a13)-(rQ*a13))/a17)}else{XI})});let a1G=(if rw{d}else{(if rj{(sf[538]*(((rl*(I*TQ))-(rk*(TQ+Vk)))/ZX))}else{d})});let a1H=(if rw{d}else{(if rj{(sf[538]*(((rl*(I*TR))-(rk*(TR+Vl)))/ZX))}else{d})});let a1I=(if rw{d}else{(if rj{(sf[538]*(((rl*(I*TS))-(rk*(TS+Vm)))/ZX))}else{d})});let a1J=(if rw{TQ}else{(if (oG!=0.0){(((rr*a0d)-(rq*TQ))/a0j)}else{d})});let a1K=(if rw{TR}else{(if (oG!=0.0){(((rr*a0e)-(rq*TR))/a0j)}else{d})});
        let a1L=(if rw{TS}else{(if (oG!=0.0){(((rr*a0f)-(rq*TS))/a0j)}else{d})});let a1S=(if rw{(-(a1J/sf[217]))}else{(if (oG!=0.0){((-a0d)/a0j)}else{d})});let a1T=(if rw{(-(a1K/sf[217]))}else{(if (oG!=0.0){((-a0e)/a0j)}else{d})});let a1U=(if rw{(-(a1L/sf[217]))}else{(if (oG!=0.0){((-a0f)/a0j)}else{d})});let a2h=(if sl{(-(sf[876]*((sn*sf[953])/so)))}else{(if (se!=0.0){(sf[351]-(sf[876]*((sf_*sf[951])/sg)))}else{d})});let a2i=(if sl{(-(sf[876]*((sn*sf[954])/so)))}else{(if (se!=0.0){(sf[0]-(sf[876]*((sf_*sf[952])/sg)))}else{d})});let a2l=(-(sf[579]*a2h));let a2m=(-(sf[579]*a2i));let a2p=(sf[234]*f64::powf(su,sf[355]));let a2q=(a2l*a2p);let a2r=(a2m*a2p);let a2A=((sf[877]*(-a2q))+(bL*(sf[351]-a2h)));let a2B=((sf[877]*(-a2r))+(bL*(sf[0]-a2i)));let a2J=(if sb[26]{sf[0]}else{(if sb[24]{(sf[0]+(if rw{d}else{(if (oG!=0.0){(Zm+(((if (oG!=0.0){(sf[871]*TQ)}else{d})+(Zv+Zv))/ZE))}else{d})}))}else{sf[356]})});let a2K=(if sb[26]{d}else{(if sb[24]{(sf[351]+(if rw{sf[0]}else{(if (oG!=0.0){(Zn+(((if (oG!=0.0){(sf[871]*TR)}else{d})+(Zx+Zx))/ZE))}else{d})}))}else{sf[357]})});let a2L=(if sb[26]{sf[351]}else{(if sb[24]{(if rw{sf[351]}else{(if (oG!=0.0){(Zo+(((if (oG!=0.0){(sf[871]*TS)}else{d})+(Zz+Zz))/ZE))}else{d})})}else{d})});let a2P=(s1*s1);let a2Q=(((s1*a2J)-(sX*a1G))/a2P);let a2U=(((s1*a2K)-(sX*a1H))/a2P);let a2Y=(((s1*a2L)-(sX*a1I))/a2P);let a3F=(if t7{(-((tb*a1G)+(s1*((t9*(-a2Q))/ta))))}else{(if (t0!=0.0){(a2J-((t3*a1G)+(s1*((t1*a2Q)/t2))))}else{d})});let a3G=(if t7{(-((tb*a1H)+(s1*((t9*(-a2U))/ta))))}else{(if (t0!=0.0){(a2K-((t3*a1H)+(s1*((t1*a2U)/t2))))}else{d})});let a3H=(if t7{(-((tb*a1I)+(s1*((t9*(-a2Y))/ta))))}else{(if (t0!=0.0){(a2L-((t3*a1I)+(s1*((t1*a2Y)/t2))))}else{d})});let a3K=(sf[239]*f64::powf(s5,sf[358]));let a3L=(a1S*a3K);let a3M=(a1T*a3K);let a3N=(a1U*a3K);let a3W=(sf[240]*f64::powf(tk,sf[359]));let a4z=(sf[879]*((sf[884]*(-((tl*a3N)+(tg*((-(a3H/sf[538]))*a3W)))))+((tq*(sf[880]*a3N))+(tp*(a2L-a3H)))));let a4C=((sf[879]*((sf[884]*(-((tl*a3L)+(tg*((-(a3F/sf[538]))*a3W)))))+((tq*(sf[880]*a3L))+(tp*(a2J-a3F)))))+sf[955]);let a4D=((sf[879]*((sf[884]*(-((tl*a3M)+(tg*((-(a3G/sf[538]))*a3W)))))+((tq*(sf[880]*a3M))+(tp*(a2K-a3G)))))+sf[956]);let a4E=(sf[886]*R7);let a4F=(sf[886]*R8);let a4G=(I*tA);let a4H=(a4E/a4G);let a4I=(a4F/a4G);let a4M=(tB*tB);let a4N=(((tB*a4E)-(ty*a4H))/a4M);let a4R=(((tB*a4F)-(ty*a4I))/a4M);let a4U=(sf[887]*f64::powf(rA,sf[957]));let a4V=(a0T*a4U);let a4W=(a0U*a4U);let a4X=(a0V*a4U);let a4Y=(sf[886]*a4V);let a4Z=(sf[886]*a4W);let a50=(sf[886]*a4X);let a51=(I*tH);let a58=(tI*tI);let a59=(((tI*a4Y)-(tF*(a4Y/a51)))/a58);let a5d=(((tI*a4Z)-(tF*(a4Z/a51)))/a58);let a5h=(((tI*a50)-(tF*(a50/a51)))/a58);let a5i=(a2A/sf[804]);let a5j=(a2B/sf[804]);let a5k=(a4C/sf[802]);let a5l=(a4D/sf[802]);let a5m=(a4z/sf[802]);let a5n=(a5j+a5k);let a5Z=(if sb[28]{((u0*(if sb[28]{(sf[412]*(sf[849]*a5i))}else{d}))/sf[890])}else{(if (sf[241]!=0.0){a5i}else{d})});let a60=(if sb[28]{(((u0*(if sb[28]{(sf[412]*(sf[849]*a5j))}else{d}))-(u1*(if sb[28]{(sf[412]*(sf[849]*((-a4C)/sf[802])))}else{d})))/sf[890])}else{(if (sf[241]!=0.0){a5n}else{d})});let a61=(if sb[28]{((-(u1*(if sb[28]{(sf[412]*(sf[849]*((-a4D)/sf[802])))}else{d})))/sf[890])}else{(if (sf[241]!=0.0){a5l}else{d})});let a62=(if sb[28]{((-(u1*(if sb[28]{(sf[412]*(sf[849]*((-a4z)/sf[802])))}else{d})))/sf[890])}else{(if (sf[241]!=0.0){a5m}else{d})});let a63=(u7*a5Z);let a64=(a63+a63);let a65=(u7*a60);let a66=(a65+a65);let a67=(u7*a61);let a68=(a67+a67);let a69=(u7*a62);let a6a=(a69+a69);let a6b=(I*ue);let a6c=(a64/a6b);let a6d=(a66/a6b);let a6e=(a68/a6b);let a6f=(a6a/a6b);let a6m=(uf*uf);let a6O=(gj*a4N);let a6P=(gj*(a4R+a59));let a6Q=(gj*a5d);let a6R=(gj*a5h);let a6U=((uo*(if ui{(gj*(a5Z+a6c))}else{(if (ub!=0.0){((-(uc*(a6c-a5Z)))/a6m)}else{d})}))+(ul*a6O));let a6X=((uo*(if ui{(gj*(a60+a6d))}else{(if (ub!=0.0){((-(uc*(a6d-a60)))/a6m)}else{d})}))+(ul*a6P));let a70=((uo*(if ui{(gj*(a61+a6e))}else{(if (ub!=0.0){((-(uc*(a6e-a61)))/a6m)}else{d})}))+(ul*a6Q));let a73=((uo*(if ui{(gj*(a62+a6f))}else{(if (ub!=0.0){((-(uc*(a6f-a62)))/a6m)}else{d})}))+(ul*a6R));let a74=(sf[891]*a4V);
        let a75=(sf[891]*a4W);let a76=(sf[891]*a4X);let a78=(sf[687]*R8);let a7c=(up*(sf[687]*R7));let a7f=(up*up);let a7N=(if uF{(sf[351]+(uw*((uH*sf[362])/uI)))}else{(if (uz!=0.0){(uw*((uA*sf[360])/uB))}else{d})});let a7O=(if uF{(sf[0]+(uw*((uH*sf[363])/uI)))}else{(if (uz!=0.0){(uw*((uA*sf[361])/uB))}else{d})});let a8L=(if vu{(vv*sf[958])}else{(if (vr!=0.0){(vs*sf[958])}else{a7N})});let a8M=(if vu{(vv*sf[959])}else{(if (vr!=0.0){(vs*sf[959])}else{a7O})});let aaV=(if wL{(wM*sf[960])}else{(if (wI!=0.0){(wJ*sf[960])}else{a8L})});let aaW=(if wL{(wM*sf[961])}else{(if (wI!=0.0){(wJ*sf[961])}else{d})});let aaX=(if wL{d}else{(if (wI!=0.0){d}else{a8M})});let abQ=(if xm{(xn*sf[962])}else{(if (xj!=0.0){(xk*sf[962])}else{aaV})});let abR=(if xm{d}else{(if (xj!=0.0){d}else{aaW})});let abS=(if xm{(xn*sf[963])}else{(if (xj!=0.0){(xk*sf[963])}else{aaX})});let ac5=(if xz{(xA*sf[964])}else{(if (xw!=0.0){(xx*sf[964])}else{abQ})});let ac6=(if xz{(xA*sf[965])}else{(if (xw!=0.0){(xx*sf[965])}else{abR})});let ac7=(if xz{d}else{(if (xw!=0.0){d}else{abS})});let acs=(if xM{d}else{(if (xJ!=0.0){d}else{ac5})});let act=(if xM{(xN*sf[966])}else{(if (xJ!=0.0){(xK*sf[966])}else{ac6})});let acu=(if xM{(xN*sf[967])}else{(if (xJ!=0.0){(xK*sf[967])}else{ac7})});let acv=(if xM{(xN*sf[968])}else{(if (xJ!=0.0){(xK*sf[968])}else{d})});let acw=(if xM{(xN*sf[969])}else{(if (xJ!=0.0){(xK*sf[969])}else{d})});let acN=(if xZ{(y0*sf[970])}else{(if (xW!=0.0){(xX*sf[970])}else{acs})});let acO=(if xZ{(y0*sf[971])}else{(if (xW!=0.0){(xX*sf[971])}else{act})});let acP=(if xZ{d}else{(if (xW!=0.0){d}else{acu})});let acQ=(if xZ{d}else{(if (xW!=0.0){d}else{acv})});let acR=(if xZ{d}else{(if (xW!=0.0){d}else{acw})});let aij=(sf[886]*Rn);let aik=(sf[886]*Ro);let ail=(sf[886]*Rp);let aim=(sf[886]*Rq);let ain=(gv*(if nQ{(nR*sf[944])}else{(if (nN!=0.0){(nO*sf[944])}else{d})}));let aio=(gv*(if nQ{(nR*sf[948])}else{(if (nN!=0.0){(nO*sf[948])}else{d})}));let aip=(gv*(if nQ{(nR*sf[949])}else{(if (nN!=0.0){(nO*sf[949])}else{d})}));let aiq=(gv*(if nQ{(nR*sf[945])}else{(if (nN!=0.0){(nO*sf[945])}else{d})}));let air=(I*Bd);let aiz=(Be*Be);let aiN=(I*Bh);let aiV=(Bi*Bi);let amH=(I*CI);let amP=(CJ*CJ);let an3=(if (sf[266]!=0.0){(((CJ*(sf[906]*RM))-(CF*((sf[898]*RM)/amH)))/amP)}else{d});let an4=(if (sf[266]!=0.0){(((CJ*(sf[906]*RN))-(CF*((sf[898]*RN)/amH)))/amP)}else{d});let an5=(if (sf[266]!=0.0){(((CJ*(sf[906]*RO))-(CF*((sf[898]*RO)/amH)))/amP)}else{d});let an6=(if (sf[266]!=0.0){(((CJ*(sf[906]*RP))-(CF*((sf[898]*RP)/amH)))/amP)}else{d});let ana=(sf[907]*RM);let anb=(sf[907]*RN);let ane=(sf[907]*RO);let anl=(sf[909]*RM);let anm=(sf[909]*RN);let anp=(sf[909]*RO);let anr=(I*CY);let anB=(CZ*CZ);let ao5=(I*D6);let aod=(D7*D7);let aom=(((D7*ane)-(D3*(anp/ao5)))/aod);let aor=(if sb[46]{(((D7*ana)-(D3*(anl/ao5)))/aod)}else{(if sb[45]{(((CZ*ana)-(CR*(anl/anr)))/anB)}else{d})});let aos=(if sb[46]{(((D7*anb)-(D3*(anm/ao5)))/aod)}else{(if sb[45]{(((CZ*anb)-(CR*(anm/anr)))/anB)}else{d})});let aot=(if sb[46]{d}else{(if sb[45]{(((CZ*(sf[907]*(-S7)))-(CR*((sf[909]*(sf[261]*S7))/anr)))/anB)}else{d})});let aou=(if sb[46]{aom}else{(if sb[45]{(((CZ*(sf[907]*(RO-S8)))-(CR*((sf[909]*(RO+(sf[261]*S8)))/anr)))/anB)}else{d})});let aov=(if sb[46]{aom}else{(if sb[45]{(((CZ*ane)-(CR*(anp/anr)))/anB)}else{d})});let aow=(if sb[46]{(((D7*(sf[907]*RP))-(D3*((sf[909]*RP)/ao5)))/aod)}else{(if sb[45]{(((CZ*(sf[907]*(RP-S9)))-(CR*((sf[909]*(RP+(sf[261]*S9)))/anr)))/anB)}else{d})});let aoB=(Dn*sf[378]);let aoC=(aoB+aoB);let aoD=(Dn*sf[379]);let aoF=(Dn*sf[380]);let aoG=(aoF+aoF);let aoH=(Dn*sf[381]);let aoJ=(if sb[48]{aoC}else{d});let aoK=(if sb[48]{(aoD+aoD)}else{d});let aoL=(if sb[48]{d}else{a64});let aoM=(if sb[48]{aoC}else{a66});let aoN=(if sb[48]{aoG}else{a68});let aoO=(if sb[48]{aoG}else{a6a});let aoP=(if sb[48]{(aoH+aoH)}else{d});let aoQ=(if sb[48]{aoG}else{d});let aoR=(I*Dx);let aoS=(aoJ/aoR);let aoT=(aoK/aoR);let aoU=(aoL/aoR);let aoV=(aoM/aoR);let aoW=(aoN/aoR);let aoX=(aoO/aoR);let aoY=(aoP/aoR);let aoZ=(aoQ/aoR);let ap9=(Dy*Dy);
        let apT=(if DC{(gj*(sf[378]+aoS))}else{(if Du{((-(sf[272]*(aoS-sf[378])))/ap9)}else{d})});let apU=(if DC{(gj*(sf[379]+aoT))}else{(if Du{((-(sf[272]*(aoT-sf[379])))/ap9)}else{d})});let apV=(if DC{(gj*aoU)}else{(if Du{((-(sf[272]*aoU))/ap9)}else{d})});let apW=(if DC{(gj*(sf[378]+aoV))}else{(if Du{((-(sf[272]*(aoV-sf[378])))/ap9)}else{d})});let apX=(if DC{(gj*(sf[380]+aoW))}else{(if Du{((-(sf[272]*(aoW-sf[380])))/ap9)}else{d})});let apY=(if DC{(gj*(sf[380]+aoX))}else{(if Du{((-(sf[272]*(aoX-sf[380])))/ap9)}else{d})});let apZ=(if DC{(gj*(sf[381]+aoY))}else{(if Du{((-(sf[272]*(aoY-sf[381])))/ap9)}else{d})});let aq0=(if DC{(gj*(sf[380]+aoZ))}else{(if Du{((-(sf[272]*(aoZ-sf[380])))/ap9)}else{d})});let aq6=(sf[611]*(an3+aor));let aq9=(sf[611]*(an5+aou));let aqm=(DJ*DJ);let ar2=(if sb[50]{d}else{(if sb[48]{(((DJ*apT)-(DF*(apT+aq6)))/aqm)}else{d})});let ar3=(if sb[50]{d}else{(if sb[48]{(((DJ*apU)-(DF*(apU+(sf[611]*(an4+aos)))))/aqm)}else{d})});let ar4=(if sb[50]{d}else{(if sb[48]{((-(DF*(sf[611]*aot)))/aqm)}else{d})});let ar5=(if sb[50]{d}else{(if sb[48]{(((DJ*apV)-(DF*apV))/aqm)}else{d})});let ar6=(if sb[50]{d}else{(if sb[48]{(((DJ*apW)-(DF*(apW+aq6)))/aqm)}else{d})});let ar7=(if sb[50]{d}else{(if sb[48]{(((DJ*apX)-(DF*(apX+aq9)))/aqm)}else{d})});let ar8=(if sb[50]{d}else{(if sb[48]{(((DJ*apY)-(DF*(apY+(sf[611]*(an5+aov)))))/aqm)}else{d})});let ar9=(if sb[50]{d}else{(if sb[48]{(((DJ*apZ)-(DF*(apZ+(sf[611]*(an6+aow)))))/aqm)}else{d})});let ara=(if sb[50]{d}else{(if sb[48]{(((DJ*aq0)-(DF*(aq0+aq9)))/aqm)}else{d})});let avY=(tP*a5i);let aw0=(tP*a5n);let aw2=(tP*a5l);let aw4=(tP*a5m);let aw6=(I*ET);let aw7=((avY+avY)/aw6);let aw8=((aw0+aw0)/aw6);let aw9=((aw2+aw2)/aw6);let awa=((aw4+aw4)/aw6);let awh=(EU*EU);let awE=(if EX{(gj*(a5i+aw7))}else{(if (ER!=0.0){((-(uc*(aw7-a5i)))/awh)}else{d})});let awF=(if EX{(gj*(a5n+aw8))}else{(if (ER!=0.0){((-(uc*(aw8-a5n)))/awh)}else{d})});let awG=(if EX{(gj*(a5l+aw9))}else{(if (ER!=0.0){((-(uc*(aw9-a5l)))/awh)}else{d})});let awH=(if EX{(gj*(a5m+awa))}else{(if (ER!=0.0){((-(uc*(awa-a5m)))/awh)}else{d})});let aKR=(if JK{(-(sf[876]*((JM*sf[953])/JN)))}else{(if (JD!=0.0){(sf[351]-(sf[876]*((JE*sf[951])/JF)))}else{d})});let aKS=(if JK{(-(sf[876]*((JM*sf[954])/JN)))}else{(if (JD!=0.0){(sf[0]-(sf[876]*((JE*sf[952])/JF)))}else{d})});let aKY=(sf[234]*f64::powf(JU,sf[355]));let aLk=((K7*awE)+(F0*(sf[927]*a4N)));let aLn=((K7*awF)+(F0*(sf[927]*a4R)));let aLo=(K7*awG);let aLp=(K7*awH);let aLt=(K9*awE);let aLw=((K9*awF)+(F0*(sf[927]*a59)));let aLz=((K9*awG)+(F0*(sf[927]*a5d)));let aLC=((K9*awH)+(F0*(sf[927]*a5h)));let aMl=(if Kl{(-(sf[872]*((Kn*sf[988])/Ko)))}else{(if (Ke!=0.0){(sf[0]-(sf[872]*((Kf*sf[984])/Kg)))}else{d})});let aMm=(if Kl{(-(sf[872]*((Kn*sf[989])/Ko)))}else{(if (Ke!=0.0){(sf[352]-(sf[872]*((Kf*sf[985])/Kg)))}else{d})});let aMn=(if Kl{(-(sf[872]*((Kn*sf[990])/Ko)))}else{(if (Ke!=0.0){(sf[353]-(sf[872]*((Kf*sf[986])/Kg)))}else{d})});let aMo=(if Kl{(-(sf[872]*((Kn*sf[991])/Ko)))}else{(if (Ke!=0.0){(sf[351]-(sf[872]*((Kf*sf[987])/Kg)))}else{d})});let aMy=(sf[240]*f64::powf(Ku,sf[359]));let aNV=(if KS{(-(sf[872]*((KU*sf[989])/KV)))}else{(if (KL!=0.0){(sf[352]-(sf[872]*((KM*sf[985])/KN)))}else{d})});let aNW=(if KS{(-(sf[872]*((KU*sf[995])/KV)))}else{(if (KL!=0.0){(sf[354]-(sf[872]*((KM*sf[994])/KN)))}else{d})});let aNX=(if KS{(-(sf[872]*((KU*sf[990])/KV)))}else{(if (KL!=0.0){(sf[353]-(sf[872]*((KM*sf[986])/KN)))}else{d})});let aNY=(if KS{(-(sf[872]*((KU*sf[991])/KV)))}else{(if (KL!=0.0){(sf[351]-(sf[872]*((KM*sf[987])/KN)))}else{d})});let aO8=(sf[240]*f64::powf(L1,sf[359]));let aOO=(sf[6]*(sf[322]*(sf[594]*(sf[992]+(sf[879]*((sf[884]*(-((-(aNV/sf[538]))*aO8)))+(sf[880]*(sf[352]-aNV))))))));let aOQ=(sf[6]*(sf[322]*(sf[594]*(sf[993]+(sf[879]*((sf[884]*(-((-(aNX/sf[538]))*aO8)))+(sf[880]*(sf[353]-aNX))))))));let aPe=(if Lt{(-(sf[928]*((Lv*sf[999])/Lw)))}else{(if (Lm!=0.0){(sf[0]-(sf[928]*((Ln*sf[997])/Lo)))}else{d})});let aPf=(if Lt{(-(sf[928]*((Lv*sf[1000])/Lw)))}else{(if (Lm!=0.0){(sf[351]-(sf[928]*((Ln*sf[998])/Lo)))}else{d})});let aPm=(sf[326]*f64::powf(LE,sf[391]));
        let aPR=(sf[934]*(if LY{(LZ*sf[1001])}else{(if (LV!=0.0){(LW*sf[1001])}else{acN})}));let aPS=(sf[934]*(if LY{d}else{(if (LV!=0.0){d}else{acO})}));let aPT=(sf[934]*(if LY{(LZ*sf[1002])}else{(if (LV!=0.0){(LW*sf[1002])}else{acP})}));let aPU=(sf[934]*(if LY{d}else{(if (LV!=0.0){d}else{acQ})}));let aPV=(sf[934]*(if LY{d}else{(if (LV!=0.0){d}else{acR})}));let aR2=(I*MH);let aRa=(MI*MI);let aRo=(if sb[64]{(((MI*(sf[941]*Rn))-(ME*((gv*(if Mx{(My*sf[1003])}else{(if Mt{(Mu*sf[1003])}else{d})}))/aR2)))/aRa)}else{(if (sf[330]!=0.0){((sf[940]*((sf[926]*(((Be*aij)-(Bb*(aij/air)))/aiz))+(sf[938]*(((Bi*ain)-(Ba*(ain/aiN)))/aiV))))/sf[833])}else{d})});let aRp=(if sb[64]{(((MI*(sf[941]*Ro))-(ME*((gv*(if Mx{(My*sf[1004])}else{(if Mt{(Mu*sf[1004])}else{d})}))/aR2)))/aRa)}else{(if (sf[330]!=0.0){((sf[940]*((sf[926]*(((Be*aik)-(Bb*(aik/air)))/aiz))+(sf[938]*(((Bi*aio)-(Ba*(aio/aiN)))/aiV))))/sf[833])}else{d})});let aRq=(if sb[64]{(((MI*(sf[941]*Rp))-(ME*((gv*(if Mx{(My*sf[1005])}else{(if Mt{(Mu*sf[1005])}else{d})}))/aR2)))/aRa)}else{(if (sf[330]!=0.0){((sf[940]*((sf[926]*(((Be*ail)-(Bb*(ail/air)))/aiz))+(sf[938]*(((Bi*aip)-(Ba*(aip/aiN)))/aiV))))/sf[833])}else{d})});let aRr=(if sb[64]{(((MI*(sf[941]*Rq))-(ME*((gv*(if Mx{(My*sf[1006])}else{(if Mt{(Mu*sf[1006])}else{d})}))/aR2)))/aRa)}else{(if (sf[330]!=0.0){((sf[940]*((sf[926]*(((Be*aim)-(Bb*(aim/air)))/aiz))+(sf[938]*(((Bi*aiq)-(Ba*(aiq/aiN)))/aiV))))/sf[833])}else{d})});let aRE=(if sb[68]{(sf[886]*RM)}else{d});let aRF=(if sb[68]{(sf[886]*RN)}else{d});let aRG=(if sb[68]{(sf[886]*RO)}else{d});let aRH=(if sb[68]{(sf[886]*RP)}else{d});let aRI=(I*MW);let aRQ=(MX*MX);let aSc=(if sb[68]{(gv*(if nE{(nF*sf[948])}else{(if (nB!=0.0){(nC*sf[948])}else{d})}))}else{d});let aSd=(if sb[68]{(gv*(if nE{(nF*sf[950])}else{(if (nB!=0.0){(nC*sf[950])}else{d})}))}else{d});let aSe=(if sb[68]{(gv*(if nE{(nF*sf[949])}else{(if (nB!=0.0){(nC*sf[949])}else{d})}))}else{d});let aSf=(if sb[68]{(gv*(if nE{(nF*sf[945])}else{(if (nB!=0.0){(nC*sf[945])}else{d})}))}else{d});let aSg=(I*N3);let aSo=(N4*N4);let aTs=(I*Ny);let aTA=(Nz*Nz);let aTT=(DO*(if sb[69]{(((Nz*(sf[943]*RM))-(Nv*((gv*(if No{(Np*sf[948])}else{(if Nk{(Nl*sf[948])}else{d})}))/aTs)))/aTA)}else{(if sb[68]{((sf[942]*((sf[926]*(if sb[68]{(((MX*aRE)-(MU*(aRE/aRI)))/aRQ)}else{d}))+(sf[938]*(if sb[68]{(((N4*aSc)-(N1*(aSc/aSg)))/aSo)}else{d}))))/sf[833])}else{d})}));let aU3=(DO*(if sb[69]{(((Nz*(sf[943]*RO))-(Nv*((gv*(if No{(Np*sf[949])}else{(if Nk{(Nl*sf[949])}else{d})}))/aTs)))/aTA)}else{(if sb[68]{((sf[942]*((sf[926]*(if sb[68]{(((MX*aRG)-(MU*(aRG/aRI)))/aRQ)}else{d}))+(sf[938]*(if sb[68]{(((N4*aSe)-(N1*(aSe/aSg)))/aSo)}else{d}))))/sf[833])}else{d})}));let aUn=(sf[336]*f64::powf(su,sf[396]));let aUx=(NQ*NQ);let aUF=(NW*sf[1009]);let aUG=(NW*sf[1010]);let aUK=(NX*NX);let aVa=(tA*tA);let aVL=(if (sf[335]!=0.0){(aPU/sf[935])}else{d});let aWo=(sf[337]*aPU);let aWu=(if (sf[335]!=0.0){(aLk+(sf[337]*aPR))}else{d});let aWv=(if (sf[335]!=0.0){(sf[337]*aPS)}else{d});let aWw=(if (sf[335]!=0.0){(aLn+(sf[337]*aPT))}else{d});let aWx=(if (sf[335]!=0.0){(aLo+aWo)}else{d});let aWy=(if (sf[335]!=0.0){(aLp+aWo)}else{d});let aWz=(if (sf[335]!=0.0){(sf[337]*aPV)}else{d});let aX2=(if sb[71]{aLk}else{(if (sf[335]!=0.0){(sf[340]*aWu)}else{d})});let aX3=(if sb[71]{d}else{(if (sf[335]!=0.0){(sf[340]*aWv)}else{d})});let aX4=(if sb[71]{aLn}else{(if (sf[335]!=0.0){(sf[340]*aWw)}else{d})});let aX5=(if sb[71]{aLo}else{(if (sf[335]!=0.0){(sf[340]*aWx)}else{d})});let aX6=(if sb[71]{aLp}else{(if (sf[335]!=0.0){(sf[340]*aWy)}else{d})});let aX7=(if sb[71]{d}else{(if (sf[335]!=0.0){(sf[340]*aWz)}else{d})});let aX8=(if sb[71]{aLt}else{(if (sf[335]!=0.0){(aLt+(sf[339]*aWu))}else{d})});let aX9=(if sb[71]{d}else{(if (sf[335]!=0.0){(sf[339]*aWv)}else{d})});let aXa=(if sb[71]{aLw}else{(if (sf[335]!=0.0){(aLw+(sf[339]*aWw))}else{d})});let aXb=(if sb[71]{aLz}else{(if (sf[335]!=0.0){(aLz+(sf[339]*aWx))}else{d})});let aXc=(if sb[71]{aLC}else{(if (sf[335]!=0.0){(aLC+(sf[339]*aWy))}else{d})});let aXd=(if sb[71]{d}else{(if (sf[335]!=0.0){(sf[339]*aWz)}else{d})});
        let aXh=(if sb[71]{aPU}else{(if (sf[335]!=0.0){(sf[338]*aPU)}else{d})});let aXz=(OD*OD);let aYk=(if OR{((OS*a6U)+(up*(sf[829]*awE)))}else{(if (ON!=0.0){(((OD*(aX2+aX8))-(OO*((a7c-(OC*a6U))/a7f)))/aXz)}else{d})});let aYl=(if OR{d}else{(if (ON!=0.0){((aX3+aX9)/OD)}else{d})});let aYm=(if OR{((OS*a6X)+(up*(sf[829]*awF)))}else{(if (ON!=0.0){(((OD*(aX4+aXa))-(OO*(((up*(a74+a78))-(OC*a6X))/a7f)))/aXz)}else{d})});let aYn=(if OR{((OS*a70)+(up*(sf[829]*awG)))}else{(if (ON!=0.0){(((OD*(aX5+aXb))-(OO*(((up*a75)-(OC*a70))/a7f)))/aXz)}else{d})});let aYo=(if OR{((OS*a73)+(up*(sf[829]*awH)))}else{(if (ON!=0.0){(((OD*(aX6+aXc))-(OO*(((up*a76)-(OC*a73))/a7f)))/aXz)}else{d})});let aYp=(if OR{d}else{(if (ON!=0.0){((aX7+aXd)/OD)}else{d})});let aYO=(if sb[79]{d}else{(if sb[77]{(sf[346]*aYk)}else{(if (sf[344]!=0.0){(sf[339]*aYk)}else{d})})});let aYP=(if sb[79]{d}else{(if sb[77]{(sf[346]*aYl)}else{(if (sf[344]!=0.0){(sf[339]*aYl)}else{d})})});let aYQ=(if sb[79]{d}else{(if sb[77]{(sf[346]*aYm)}else{(if (sf[344]!=0.0){(sf[339]*aYm)}else{d})})});let aYR=(if sb[79]{d}else{(if sb[77]{(sf[346]*aYn)}else{(if (sf[344]!=0.0){(sf[339]*aYn)}else{d})})});let aYS=(if sb[79]{d}else{(if sb[77]{(sf[346]*aYo)}else{(if (sf[344]!=0.0){(sf[339]*aYo)}else{d})})});let aYT=(if sb[79]{d}else{(if sb[77]{(sf[346]*aYp)}else{(if (sf[344]!=0.0){(sf[339]*aYp)}else{d})})});let b2d=(sf[0]*((if sb[71]{aPR}else{(if (sf[335]!=0.0){(sf[338]*aPR)}else{d})})+((sf[923]*a2A)+aX2)));let b2e=(sf[0]*(aX3+(if sb[71]{aPS}else{(if (sf[335]!=0.0){(sf[338]*aPS)}else{d})})));let b2f=(sf[0]*((if sb[71]{aPT}else{(if (sf[335]!=0.0){(sf[338]*aPT)}else{d})})+((sf[923]*a2B)+aX4)));let b2g=(sf[0]*(aX5+aXh));let b2h=(sf[0]*(aX6+aXh));let b2i=(sf[0]*(aX7+(if sb[71]{aPV}else{(if (sf[335]!=0.0){(sf[338]*aPV)}else{d})})));let b2w=(sf[0]*(sf[924]*((sf[877]*(-((-(sf[579]*aKR))*aKY)))+(bL*(sf[351]-aKR)))));let b2x=(sf[0]*(sf[924]*((sf[877]*(-((-(sf[579]*aKS))*aKY)))+(bL*(sf[0]-aKS)))));let b2C=(sf[0]*aX8);let b2D=(sf[0]*aX9);let b2E=(sf[0]*(((Ma*(sf[939]*a1A))+(M9*a0W))+((sf[925]*a4C)+aXa)));let b2F=(sf[0]*(((Ma*(sf[939]*a1B))+(M9*a0X))+((sf[925]*a4D)+aXb)));let b2G=(sf[0]*(((Ma*(sf[939]*a1C))+(M9*a0S))+((sf[925]*a4z)+aXc)));let b2H=(sf[0]*aXd);let b2U=(sf[0]*(sf[588]*((sf[930]*(-((-(aPe/sf[578]))*aPm)))+(I*(sf[0]-aPe)))));let b2V=(sf[0]*(sf[588]*((sf[930]*(-((-(aPf/sf[578]))*aPm)))+(I*(sf[351]-aPf)))));let b30=(sf[0]*(if (sf[335]!=0.0){(Of*((if (sf[335]!=0.0){(aPR/sf[935])}else{d})+((if (sf[335]!=0.0){(sf[923]*(if (sf[335]!=0.0){((NZ*(if (sf[335]!=0.0){(a2l*aUn)}else{d}))+(NK*(if NU{(((NX*aUF)-(NW*aUF))/aUK)}else{(if NO{((-(NP*sf[1007]))/aUx)}else{d})})))}else{d}))}else{d})+(if (sf[335]!=0.0){((Oa*(if (sf[335]!=0.0){((O7*((sf[412]*a4E)/sf[642]))+(O6*((-(gj*a4H))/aVa)))}else{d}))+(O9*(sf[927]*awE)))}else{d}))))}else{d}));let b31=(sf[0]*(if (sf[335]!=0.0){((Oh*sf[397])+(Of*(if (sf[335]!=0.0){(aPS/sf[935])}else{d})))}else{d}));let b32=(sf[0]*(if (sf[335]!=0.0){((Oh*sf[398])+(Of*((if (sf[335]!=0.0){(aPT/sf[935])}else{d})+((if (sf[335]!=0.0){(sf[923]*(if (sf[335]!=0.0){((NZ*(if (sf[335]!=0.0){(a2m*aUn)}else{d}))+(NK*(if NU{(((NX*aUG)-(NW*aUG))/aUK)}else{(if NO{((-(NP*sf[1008]))/aUx)}else{d})})))}else{d}))}else{d})+(if (sf[335]!=0.0){((Oa*(if (sf[335]!=0.0){((O7*((sf[412]*a4F)/sf[642]))+(O6*((-(gj*a4I))/aVa)))}else{d}))+(O9*(sf[927]*awF)))}else{d})))))}else{d}));let b33=(sf[0]*(if (sf[335]!=0.0){(Of*((if (sf[335]!=0.0){(O9*(sf[927]*awG))}else{d})+aVL))}else{d}));let b34=(sf[0]*(if (sf[335]!=0.0){(Of*((if (sf[335]!=0.0){(O9*(sf[927]*awH))}else{d})+aVL))}else{d}));let b35=(sf[0]*(if (sf[335]!=0.0){(Of*(if (sf[335]!=0.0){(aPV/sf[935])}else{d}))}else{d}));let b3Y=(sf[0]*(aOO+(if (sf[332]!=0.0){((NB*ar2)+aTT)}else{d})));
        let b3Z=(sf[0]*((sf[6]*(sf[322]*(sf[594]*((sf[879]*((sf[884]*(-((-(aNW/sf[538]))*aO8)))+(sf[880]*(sf[354]-aNW))))+sf[996]))))+(if (sf[332]!=0.0){((NB*ar3)+(DO*(if sb[69]{(((Nz*(sf[943]*RN))-(Nv*((gv*(if No{(Np*sf[950])}else{(if Nk{(Nl*sf[950])}else{d})}))/aTs)))/aTA)}else{(if sb[68]{((sf[942]*((sf[926]*(if sb[68]{(((MX*aRF)-(MU*(aRF/aRI)))/aRQ)}else{d}))+(sf[938]*(if sb[68]{(((N4*aSd)-(N1*(aSd/aSg)))/aSo)}else{d}))))/sf[833])}else{d})})))}else{d})));let b40=(sf[0]*(if (sf[332]!=0.0){(NB*ar4)}else{d}));let b41=(sf[0]*(if (sf[332]!=0.0){(NB*ar5)}else{d}));let b42=(sf[0]*(aOO+(if (sf[332]!=0.0){(aTT+(NB*ar6))}else{d})));let b43=(sf[0]*(aOQ+(if (sf[332]!=0.0){((NB*ar7)+aU3)}else{d})));let b44=(sf[0]*(aOQ+(if (sf[332]!=0.0){(aU3+(NB*ar8))}else{d})));let b45=(sf[0]*((sf[6]*(sf[322]*(sf[594]*(sf[956]+(sf[879]*((sf[884]*(-((-(aNY/sf[538]))*aO8)))+(sf[880]*(sf[351]-aNY))))))))+(if (sf[332]!=0.0){((NB*ar9)+(DO*(if sb[69]{(((Nz*(sf[943]*RP))-(Nv*((gv*(if No{(Np*sf[945])}else{(if Nk{(Nl*sf[945])}else{d})}))/aTs)))/aTA)}else{(if sb[68]{((sf[942]*((sf[926]*(if sb[68]{(((MX*aRH)-(MU*(aRH/aRI)))/aRQ)}else{d}))+(sf[938]*(if sb[68]{(((N4*aSf)-(N1*(aSf/aSg)))/aSo)}else{d}))))/sf[833])}else{d})})))}else{d})));let b46=(sf[0]*(aOQ+(if (sf[332]!=0.0){(aU3+(NB*ara))}else{d})));let b4M=(sf[0]*((sf[7]*(sf[322]*(sf[594]*(sf[955]+(sf[879]*((sf[884]*(-((-(aMl/sf[538]))*aMy)))+(sf[880]*(sf[0]-aMl))))))))+(if (sf[332]!=0.0){(sf[7]*aRo)}else{aRo})));let b4N=(sf[0]*((sf[7]*(sf[322]*(sf[594]*((sf[879]*((sf[884]*(-((-(aMm/sf[538]))*aMy)))+(sf[880]*(sf[352]-aMm))))+sf[992]))))+(if (sf[332]!=0.0){(sf[7]*aRp)}else{aRp})));let b4O=(sf[0]*((sf[7]*(sf[322]*(sf[594]*((sf[879]*((sf[884]*(-((-(aMn/sf[538]))*aMy)))+(sf[880]*(sf[353]-aMn))))+sf[993]))))+(if (sf[332]!=0.0){(sf[7]*aRq)}else{aRq})));let b4P=(sf[0]*((sf[7]*(sf[322]*(sf[594]*(sf[956]+(sf[879]*((sf[884]*(-((-(aMo/sf[538]))*aMy)))+(sf[880]*(sf[351]-aMo))))))))+(if (sf[332]!=0.0){(sf[7]*aRr)}else{aRr})));

        CommonStampValues {
            b, d, H, I, X, bL, gf, gj, 
            gv, gV, ln, lr, lt, ly, lB, lE, 
            lJ, lR, lU, lX, m1, mh, mE, mF, 
            mH, mK, mL, n1, n3, n6, n7, nn, 
            np, ns, nt, oE, qC, rA, rZ, s2, 
            s5, sw, tO, uo, up, uu, uv, uO, 
            uQ, uT, uU, v3, vz, vB, vD, vI, 
            vJ, vQ, vR, vT, vY, w0, wQ, wS, 
            wU, wZ, x0, xr, xE, xR, y4, yb, 
            yc, yf, yh, ym, yn, yt, yx, yA, 
            yI, yJ, yK, yM, yO, yS, yT, yV, 
            yY, z0, z1, z6, z7, zJ, zL, zN, 
            zO, zR, zT, zY, zZ, A4, A7, A9, 
            Ah, Ai, Aj, Al, Aq, Ar, At, Av, 
            Ax, Ay, AD, AE, CL, D9, Dr, DO, 
            F0, Fc, Fp, Fq, Fr, Fu, Fv, Fz, 
            FA, FC, FG, FI, FN, FO, G3, HM, 
            HN, HP, HR, HT, HV, HW, HY, I6, 
            I9, Ia, Ib, Ih, Ij, Ik, Io, Iq, 
            It, Iv, IA, IB, OD, P9, PY, Q1, 
            Q4, Q7, Qa, Qe, Qi, Qq, Qw, QH, 
            QX, QY, Rn, Ro, Rp, Rq, TQ, TR, 
            TS, Yv, Yw, Yx, a0T, a0U, a0V, a1A, 
            a1B, a1C, a1J, a1K, a1L, a1S, a1T, a1U, 
            a2q, a2r, a5k, a5l, a5m, a6O, a6P, a6Q, 
            a6R, a6U, a6X, a70, a73, a74, a75, a76, 
            a78, a7c, a7f, a7N, a7O, a8L, a8M, aaV, 
            aaW, aaX, abQ, abR, abS, ac5, ac6, ac7, 
            acs, act, acu, acv, acw, acN, acO, acP, 
            acQ, acR, an3, an4, an5, an6, aor, aos, 
            aot, aou, aov, aow, aoJ, aoK, aoL, aoM, 
            aoN, aoO, aoP, aoQ, ar2, ar3, ar4, ar5, 
            ar6, ar7, ar8, ar9, ara, awE, awF, awG, 
            awH, aYO, aYP, aYQ, aYR, aYS, aYT, b2d, 
            b2e, b2f, b2g, b2h, b2i, b2w, b2x, b2C, 
            b2D, b2E, b2F, b2G, b2H, b2U, b2V, b30, 
            b31, b32, b33, b34, b35, b3Y, b3Z, b40, 
            b41, b42, b43, b44, b45, b46, b4M, b4N, 
            b4O, b4P, 
        }
    }

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
        let CommonStampValues {
            b, d, H, I, X, bL, gf, gj, 
            gv, gV, ln, lr, lt, ly, lB, lE, 
            lJ, lR, lU, lX, m1, mh, mE, mF, 
            mH, mK, mL, n1, n3, n6, n7, nn, 
            np, ns, nt, oE, qC, rA, rZ, s2, 
            s5, sw, tO, uo, up, uu, uv, uO, 
            uQ, uT, uU, v3, vz, vB, vD, vI, 
            vJ, vQ, vR, vT, vY, w0, wQ, wS, 
            wU, wZ, x0, xr, xE, xR, y4, yb, 
            yc, yf, yh, ym, yn, yt, yx, yA, 
            yI, yJ, yK, yM, yO, yS, yT, yV, 
            yY, z0, z1, z6, z7, zJ, zL, zN, 
            zO, zR, zT, zY, zZ, A4, A7, A9, 
            Ah, Ai, Aj, Al, Aq, Ar, At, Av, 
            Ax, Ay, AD, AE, CL, D9, Dr, DO, 
            F0, Fc, Fp, Fq, Fr, Fu, Fv, Fz, 
            FA, FC, FG, FI, FN, FO, G3, HM, 
            HN, HP, HR, HT, HV, HW, HY, I6, 
            I9, Ia, Ib, Ih, Ij, Ik, Io, Iq, 
            It, Iv, IA, IB, OD, P9, PY, Q1, 
            Q4, Q7, Qa, Qe, Qi, Qq, Qw, QH, 
            QX, QY, Rn, Ro, Rp, Rq, TQ, TR, 
            TS, Yv, Yw, Yx, a0T, a0U, a0V, a1A, 
            a1B, a1C, a1J, a1K, a1L, a1S, a1T, a1U, 
            a2q, a2r, a5k, a5l, a5m, a6O, a6P, a6Q, 
            a6R, a6U, a6X, a70, a73, a74, a75, a76, 
            a78, a7c, a7f, a7N, a7O, a8L, a8M, aaV, 
            aaW, aaX, abQ, abR, abS, ac5, ac6, ac7, 
            acs, act, acu, acv, acw, acN, acO, acP, 
            acQ, acR, an3, an4, an5, an6, aor, aos, 
            aot, aou, aov, aow, aoJ, aoK, aoL, aoM, 
            aoN, aoO, aoP, aoQ, ar2, ar3, ar4, ar5, 
            ar6, ar7, ar8, ar9, ara, awE, awF, awG, 
            awH, aYO, aYP, aYQ, aYR, aYS, aYT, b2d, 
            b2e, b2f, b2g, b2h, b2i, b2w, b2x, b2C, 
            b2D, b2E, b2F, b2G, b2H, b2U, b2V, b30, 
            b31, b32, b33, b34, b35, b3Y, b3Z, b40, 
            b41, b42, b43, b44, b45, b46, b4M, b4N, 
            b4O, b4P, 
        }=self.eval_common_stamp_values(ctx);
        let mI=(mF).exp();let n4=(n1).exp();let nb=(if n6{(n7*(b+(n1-sf[214])))}else{(if (n3!=0.0){n4}else{d})});let nq=(nn).exp();let nx=(if ns{(nt*(b+(nn-sf[214])))}else{(if (np!=0.0){nq}else{d})});let uR=(uO).exp();let uY=(if uT{(uU*(b+(uO-sf[214])))}else{(if (uQ!=0.0){uR}else{d})});let v5=(if (lt<sf[244]){b}else{d});let v6=(v3).exp();let v7=(b+v6);let vc=(!(v5!=0.0));let ve=((-v3)).exp();let vf=(b+ve);let vj=(if vc{(sf[244]-(H*(vf).ln()))}else{(if (v5!=0.0){(lt-(H*(v7).ln()))}else{d})});let vl=(vj*sf[245]);let vm=(sf[244]-vj);let vn=f64::powf(vm,I);let vE=((sf[154]!=0.0)&&(vD!=0.0));let vF=(vB).exp();let vN=(if vI{(vJ*(b+(vB-sf[214])))}else{(if vE{vF}else{uO})});let vU=((sf[154]!=0.0)&&(vT!=0.0));let vV=(vQ).exp();let w4=(if vY{(w0*(b+(vQ-vR)))}else{(if vU{vV}else{uY})});let w5=(vz-b);let w6=(sf[715]*w5);let w8=(w5*sf[892]);let wb=((b+(gv*vN))).sqrt();let wc=(b+wb);let wd=(w8/wc);let we=(b+tO);let wi=(sf[730]*(rA-b));let wj=(w4*wi);let wk=(b+w4);let wA=(sf[246]*((rA+vz)-I));let wV=((sf[154]!=0.0)&&(wU!=0.0));let wW=(wS).exp();let x5=(wQ-b);let x6=(sf[721]*x5);let x8=(x5*sf[893]);let xb=((b+(gv*(if wZ{(x0*(b+(wS-sf[214])))}else{(if wV{wW}else{vN})})))).sqrt();let xc=(b+xb);let xT=(sf[707]*(xR-b));let yi=((yb!=0.0)&&(yh!=0.0));let yj=(yf).exp();let yr=(if ym{(yn*(b+(yf-sf[214])))}else{(if yi{yj}else{d})});let z2=((z0!=0.0)&&z1);let z3=(yV).exp();let zc=(-lt);let zd=(b-(if z6{(z7*(b+(yV-sf[214])))}else{(if z2{z3}else{d})}));let zf=(b+(zd/yV));let zj=((yb!=0.0)&&(!(yY!=0.0)));let zk=(gj*lt);let zl=(yV*zk);let zm=0.3333333333333333;let zn=(yV*zm);let zo=0.25;let zq=(b+(yV*zo));let zs=(b+(zn*zq));let zw=((if zj{(zl*zs)}else{(if z1{(zc*zf)}else{d})})*sf[894]);let zx=(sw*zw);let zC=(!(yb!=0.0));let zU=((zJ!=0.0)&&(zT!=0.0));let zV=(zR).exp();let A3=(if zY{(zZ*(b+(zR-sf[214])))}else{(if zU{zV}else{d})});let Az=((Ax!=0.0)&&Ay);let AA=(At).exp();let AJ=(-ln);let AK=(b-(if AD{(AE*(b+(At-sf[214])))}else{(if Az{AA}else{d})}));let AM=(b+(AK/At));let AQ=((zJ!=0.0)&&(!(Av!=0.0)));let AR=(gj*ln);let AS=(At*AR);let AT=(zm*At);let AV=(b+(zo*At));let AX=(b+(AT*AV));let B1=((if AQ{(AS*AX)}else{(if Ay{(AJ*AM)}else{d})})*sf[895]);let B2=(zN*B1);let B7=(!(zJ!=0.0));let B8=(if B7{d}else{(if (zJ!=0.0){(sf[54]*(sf[580]*(A3*B2)))}else{d})});let Bl=(mE-b);let Bm=(sf[896]*Bl);let Br=((b+(mE*sf[898]))).sqrt();let Bs=(b+Br);let Bt=(Bm/Bs);let BB=(sf[899]*(mh-nb));let BJ=((b+(sf[901]*(mh+(nb*sf[261]))))).sqrt();let BK=(b+BJ);let BR=(sf[902]*(mE-nx));let BW=((b+(sf[901]*(mE+(nx*sf[261]))))).sqrt();let BX=(b+BW);let C2=(sf[899]*(mh-b));let C5=((b+(mh*sf[901]))).sqrt();let C6=(b+C5);let C9=(Bl*sf[902]);let Cc=((b+(mE*sf[901]))).sqrt();let Cd=(b+Cc);let Cf=(if sb[41]{(C9/Cd)}else{(if (sf[258]!=0.0){(BR/BX)}else{d})});let Ci=(sf[903]*(nb-b));let Co=((b+(nb*sf[905]))).sqrt();let Cp=(b+Co);let Cz=(if (sf[266]!=0.0){(sf[7]*Bt)}else{Bt});let DQ=(if (sf[266]!=0.0){(CL*DO)}else{d});let DX=(if (sf[274]!=0.0){(ln+ly)}else{d});let DZ=(-DX);let E3=(if (DZ<d){b}else{d});let E4=((sf[274]!=0.0)&&(E3!=0.0));let E7=((sf[275]+(if (sf[274]!=0.0){(DX*DX)}else{Dr}))).sqrt();let E8=(E7-DZ);let Ec=((sf[274]!=0.0)&&(!(E3!=0.0)));let Ef=(if Ec{(gj*(DZ+E7))}else{(if E4{(sf[276]/E8)}else{d})});let Ew=(if (Ef<sf[284]){b}else{d});let Ex=((sf[274]!=0.0)&&(Ew!=0.0));let Ey=(Ef/sf[282]);let EA=(b-f64::powf(Ey,sf[277]));let EE=((sf[274]!=0.0)&&(!(Ew!=0.0)));let EK=(if sb[52]{b}else{(if EE{(sf[281]+(sf[291]*(Ef-sf[284])))}else{(if Ex{(b/EA)}else{d})})});let F1=(uo*F0);let F2=(sf[603]/F1);let F4=(if (F2<sf[16]){b}else{d});let F6=(bL*(if (F4!=0.0){sf[16]}else{F2}));let F9=(ly+(sf[865]*((if mK{(mL*(b+(mF-sf[214])))}else{(if (mH!=0.0){mI}else{d})})-b)));let FJ=(Fp&&(FI!=0.0));let FK=(FG).exp();let FS=(if FN{(FO*(b+(FG-sf[214])))}else{(if FJ{FK}else{d})});let FV=(FC*sf[920]);let G5=(((if (ln<sf[500]){b}else{d})!=0.0)&&((sf[298]!=0.0)&&G3));let Gb=(if G5{sf[303]}else{d});let Gc=(sf[500]-ln);let Ge=(if G5{(Gc/s5)}else{qC});let Gh=(((I*Ge)/Gb)).sqrt();let Gi=(if G5{Gh}else{d});let Gm=(G5&&(sf[305]!=0.0));let Gp=(G5&&sb[57]);let Gs=(if Gp{(b-(gj*rZ))}else{d});let Gt=(sf[301]*Gs);
        let Gv=(if Gp{(Gs*Gt)}else{(if Gm{sf[301]}else{d})});let Gw=(Gi*Gv);let GA=(((Gi*Gi)+(Gv*Gv))).sqrt();let GC=(if G5{(Gw/GA)}else{d});let GE=(if G5{(Gc/GC)}else{d});let GF=(gj*GC);let GG=(Gb*GF);let GJ=(if G5{(GE+(s5*GG))}else{d});let GW=(sf[217]*(if Gp{(b+(sf[307]*(b+(I*rZ))))}else{d}));let GY=((if Gp{sf[310]}else{d})-(uv/GW));let H1=(if Gp{(GE-(GG*GY))}else{d});let H2=(H1-GJ);let H4=(X*GE);let H5=(GE*H4);let Hb=((if Gp{((H2*H2)+((s2*H5)/sf[217]))}else{Ge})).sqrt();let He=(if Gp{(gj*((GJ+H1)+Hb))}else{(if Gm{GJ}else{d})});let Hf=(He-GE);let Hh=(if G5{(Hf/He)}else{d});let Hl=(if ((Hh).abs()>1e-7){b}else{d});let Hm=(G5&&(Hl!=0.0));let Ho=(if Hm{(GF/Hh)}else{d});let Hq=(He*sf[921]);let Hr=(Ho*Hq);let Ht=(sf[922]/He);let Hu=(Ht).exp();let Hw=(b+(Gv/Ho));let Hy=((Ht*Hw)).exp();let Hz=(Hu-Hy);let HD=(G5&&(!(Hl!=0.0)));let HE=(sf[4]*Gv);let Iw=(HM&&(Iv!=0.0));let Ix=(It).exp();let IF=(if IA{(IB*(b+(It-sf[214])))}else{(if Iw{Ix}else{FS})});let IG=(FA*sf[920]);let II=(if HM{(IF*IG)}else{(if HD{(Hu*HE)}else{(if Hm{(Hr*Hz)}else{(if Fp{(FS*FV)}else{d})})})});let IO=((Fc!=0.0)&&((if (II>d){b}else{d})!=0.0));let IP=((sf[318]!=0.0)&&IO);let IQ=(sf[608]+F6);let IR=(uv*IQ);let IY=(if IP{(((sf[411]/IR)+(sf[715]*(up/sf[687])))+(sf[600]/IQ))}else{d});let IZ=((sf[311]!=0.0)&&IP);let J2=(if IZ{((II-IY)/gf)}else{I6});let J4=(if (II<IY){b}else{d});let J5=(IZ&&(J4!=0.0));let J6=(J2).exp();let J7=(b+J6);let Jd=(IZ&&(!(J4!=0.0)));let Jf=((-J2)).exp();let Jg=(b+Jf);let Jk=(if Jd{(IY-(gf*(Jg).ln()))}else{(if J5{(II-(gf*(J7).ln()))}else{II})});let Jl=(uv*Jk);let Jo=(IP&&sb[61]);let Jp=(IY*Jl);let Jq=(IY+Jk);let Ju=(IO&&sb[62]);let Jv=(if Ju{Jl}else{(if Jo{(Jp/Jq)}else{(if IZ{Jl}else{d})})});let OL=(if sb[73]{d}else{(if (sf[342]!=0.0){((Jv/OD)).abs()}else{d})});let PD=(sf[15]*(sf[0]*(-(B8*EK))));let PZ=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, PY);let Q2=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, Q1);let Q5=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, Q4);let Q8=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, Q7);let Qb=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, Qa);let Qf=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, Qe);let Qj=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, Qi);let Qr=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, Qq);let Qx=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, Qw);
        let QI=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, QH);let RW=(if n6{(n7*sf[944])}else{(if (n3!=0.0){(n4*sf[944])}else{d})});let RX=(if n6{(n7*sf[945])}else{(if (n3!=0.0){(n4*sf[945])}else{d})});let Sj=(if ns{(nt*sf[944])}else{(if (np!=0.0){(nq*sf[944])}else{d})});let Sk=(if ns{(nt*sf[949])}else{(if (np!=0.0){(nq*sf[949])}else{d})});let Sl=(if ns{(nt*sf[945])}else{(if (np!=0.0){(nq*sf[945])}else{d})});let a7g=((a7c-(uu*a6U))/a7f);let a7k=(((up*(a78-a74))-(uu*a6X))/a7f);let a7o=(((up*(-a75))-(uu*a70))/a7f);let a7s=(((up*(-a76))-(uu*a73))/a7f);let a7P=(a7N/sf[243]);let a7Q=(a7O/sf[243]);let a7X=(if uT{(uU*a7P)}else{(if (uQ!=0.0){(uR*a7P)}else{d})});let a7Y=(if uT{(uU*a7Q)}else{(if (uQ!=0.0){(uR*a7Q)}else{d})});let a8n=(if vc{(-(H*((ve*sf[366])/vf)))}else{(if (v5!=0.0){(sf[351]-(H*((v6*sf[364])/v7)))}else{d})});let a8o=(if vc{(-(H*((ve*sf[367])/vf)))}else{(if (v5!=0.0){(sf[0]-(H*((v6*sf[365])/v7)))}else{d})});let a8u=(I*f64::powf(vm,b));let a8T=(if vI{(vJ*sf[945])}else{(if vE{(vF*sf[945])}else{a7P})});let a8U=(if vI{(vJ*sf[944])}else{(if vE{(vF*sf[944])}else{a7Q})});let a8V=(a7g/sf[687]);let a8W=(a7k/sf[687]);let a8X=(a7o/sf[687]);let a8Y=(a7s/sf[687]);let a9b=(if vY{(w0*a8V)}else{(if vU{(vV*a8V)}else{a7X})});let a9c=(if vY{(w0*a8W)}else{(if vU{(vV*a8W)}else{a7Y})});let a9d=(if vY{(w0*a8X)}else{(if vU{(vV*a8X)}else{d})});let a9e=(if vY{(w0*a8Y)}else{(if vU{(vV*a8Y)}else{d})});let a9f=(sf[715]*a8L);let a9g=(sf[715]*a8M);let a9l=(I*wb);let a9r=(wc*wc);let a9V=(wk*wk);let ab8=(sf[721]*aaV);let ab9=(sf[721]*aaW);let aba=(sf[721]*aaX);let abh=(I*xb);let abo=(xc*xc);let ad1=(yc*yc);let ad8=(sf[769]*(-((-(sf[21]*(I*a2q)))/ad1)));let ad9=(sf[769]*(-((-(sf[21]*(I*a2r)))/ad1)));let adk=(if (yb!=0.0){sf[972]}else{d});let adl=(if (yb!=0.0){sf[973]}else{d});let adm=(yt*adk);let ado=(yt*adl);let adq=(I*yx);let adv=(sf[249]*f64::powf(yx,sf[368]));let aef=(yT*yT);let ael=(if (yb!=0.0){(((yT*sf[974])-(yS*(sf[436]*(if (yb!=0.0){(yO*((yM*(((adm+adm)/adq)*adv))+(yA*((sf[19]*(-(sf[252]*(bL*adk))))-((yK*((yI*adk)+(yt*(gV*adk))))+(yJ*adk))))))}else{d}))))/aef)}else{adk});let aem=(if (yb!=0.0){(((yT*sf[975])-(yS*(sf[436]*(if (yb!=0.0){(yO*((yM*(((ado+ado)/adq)*adv))+(yA*((sf[19]*(-(sf[252]*(bL*adl))))-((yK*((yI*adl)+(yt*(gV*adl))))+(yJ*adl))))))}else{d}))))/aef)}else{adl});let aeA=(yV*yV);let afF=(sf[240]*f64::powf(zL,sf[359]));let afI=(if (zJ!=0.0){(sf[978]*afF)}else{d});let afJ=(if (zJ!=0.0){(sf[979]*afF)}else{d});let afO=(zO*zO);let afV=(sf[789]*(-((-(sf[53]*(I*afI)))/afO)));let afW=(sf[789]*(-((-(sf[53]*(I*afJ)))/afO)));let ag5=(if (zJ!=0.0){sf[976]}else{d});let ag6=(if (zJ!=0.0){sf[977]}else{d});let ag7=(A4*ag5);let ag9=(A4*ag6);let agb=(I*A7);let agg=(sf[253]*f64::powf(A7,sf[373]));let ah0=(Ar*Ar);let ah6=(if (zJ!=0.0){(((Ar*sf[980])-(Aq*(sf[457]*(if (zJ!=0.0){(yO*((Al*(((ag7+ag7)/agb)*agg))+(A9*((sf[51]*(-(sf[256]*(bL*ag5))))-((Aj*((Ah*ag5)+(A4*(gV*ag5))))+(Ai*ag5))))))}else{d}))))/ah0)}else{ag5});let ah7=(if (zJ!=0.0){(((Ar*sf[981])-(Aq*(sf[457]*(if (zJ!=0.0){(yO*((Al*(((ag9+ag9)/agb)*agg))+(A9*((sf[51]*(-(sf[256]*(bL*ag6))))-((Aj*((Ah*ag6)+(A4*(gV*ag6))))+(Ai*ag6))))))}else{d}))))/ah0)}else{ag6});let ahl=(At*At);let ajh=(I*Br);let ajp=(Bs*Bs);let ajq=(((Bs*(sf[896]*Rn))-(Bm*((sf[898]*Rn)/ajh)))/ajp);let aju=(((Bs*(sf[896]*Ro))-(Bm*((sf[898]*Ro)/ajh)))/ajp);let ajy=(((Bs*(sf[896]*Rp))-(Bm*((sf[898]*Rp)/ajh)))/ajp);let ajC=(((Bs*(sf[896]*Rq))-(Bm*((sf[898]*Rq)/ajh)))/ajp);let ajG=(sf[899]*QX);let ajI=(sf[899]*QY);let ajM=(sf[901]*QX);let ajO=(sf[901]*QY);let ajP=(I*BJ);let ajX=(BK*BK);let akj=(sf[902]*Rn);let akk=(sf[902]*Ro);let akm=(sf[902]*Rp);let aku=(sf[901]*Rn);let akv=(sf[901]*Ro);let akx=(sf[901]*Rp);let akz=(I*BW);let akJ=(BX*BX);let alb=(I*C5);let alh=(C6*C6);let alt=(I*Cc);let alB=(Cd*Cd);let alK=(((Cd*akm)-(C9*(akx/alt)))/alB);
        let alP=(if sb[41]{d}else{(if (sf[258]!=0.0){(((BX*(sf[902]*(-Sj)))-(BR*((sf[901]*(sf[261]*Sj))/akz)))/akJ)}else{d})});let alQ=(if sb[41]{(((Cd*akj)-(C9*(aku/alt)))/alB)}else{(if (sf[258]!=0.0){(((BX*akj)-(BR*(aku/akz)))/akJ)}else{d})});let alR=(if sb[41]{(((Cd*akk)-(C9*(akv/alt)))/alB)}else{(if (sf[258]!=0.0){(((BX*akk)-(BR*(akv/akz)))/akJ)}else{d})});let alS=(if sb[41]{alK}else{(if (sf[258]!=0.0){(((BX*(sf[902]*(Rp-Sk)))-(BR*((sf[901]*(Rp+(sf[261]*Sk)))/akz)))/akJ)}else{d})});let alT=(if sb[41]{alK}else{(if (sf[258]!=0.0){(((BX*akm)-(BR*(akx/akz)))/akJ)}else{d})});let alU=(if sb[41]{(((Cd*(sf[902]*Rq))-(C9*((sf[901]*Rq)/alt)))/alB)}else{(if (sf[258]!=0.0){(((BX*(sf[902]*(Rq-Sl)))-(BR*((sf[901]*(Rq+(sf[261]*Sl)))/akz)))/akJ)}else{d})});let alZ=(I*Co);let am5=(Cp*Cp);let arb=(DO*an3);let arl=(DO*an5);let arE=(DO*aor);let arQ=(DO*aou);let asg=(DX*sf[382]);let asi=(DX*sf[383]);let ask=(DX*sf[384]);let asv=(I*E7);let asw=((if (sf[274]!=0.0){d}else{aoJ})/asv);let asx=((if (sf[274]!=0.0){d}else{aoK})/asv);let asy=((if (sf[274]!=0.0){d}else{aoL})/asv);let asz=((if (sf[274]!=0.0){(asg+asg)}else{aoJ})/asv);let asA=((if (sf[274]!=0.0){(asi+asi)}else{aoM})/asv);let asB=((if (sf[274]!=0.0){(ask+ask)}else{aoN})/asv);let asC=((if (sf[274]!=0.0){d}else{aoO})/asv);let asD=((if (sf[274]!=0.0){d}else{aoP})/asv);let asE=((if (sf[274]!=0.0){d}else{aoQ})/asv);let asK=(E8*E8);let atv=(if Ec{(gj*asw)}else{(if E4{((-(sf[276]*asw))/asK)}else{d})});let atw=(if Ec{(gj*asx)}else{(if E4{((-(sf[276]*asx))/asK)}else{d})});let atx=(if Ec{(gj*asy)}else{(if E4{((-(sf[276]*asy))/asK)}else{d})});let aty=(if Ec{(gj*(sf[385]+asz))}else{(if E4{((-(sf[276]*(asz-sf[385])))/asK)}else{d})});let atz=(if Ec{(gj*(sf[386]+asA))}else{(if E4{((-(sf[276]*(asA-sf[386])))/asK)}else{d})});let atA=(if Ec{(gj*(sf[387]+asB))}else{(if E4{((-(sf[276]*(asB-sf[387])))/asK)}else{d})});let atB=(if Ec{(gj*asC)}else{(if E4{((-(sf[276]*asC))/asK)}else{d})});let atC=(if Ec{(gj*asD)}else{(if E4{((-(sf[276]*asD))/asK)}else{d})});let atD=(if Ec{(gj*asE)}else{(if E4{((-(sf[276]*asE))/asK)}else{d})});let atO=(sf[277]*f64::powf(Ey,sf[286]));let atY=(EA*EA);let auz=(if sb[52]{d}else{(if EE{(sf[291]*atv)}else{(if Ex{(((atv/sf[282])*atO)/atY)}else{d})})});let auA=(if sb[52]{d}else{(if EE{(sf[291]*atw)}else{(if Ex{(((atw/sf[282])*atO)/atY)}else{d})})});let auB=(if sb[52]{d}else{(if EE{(sf[291]*atx)}else{(if Ex{(((atx/sf[282])*atO)/atY)}else{d})})});let auC=(if sb[52]{d}else{(if EE{(sf[291]*aty)}else{(if Ex{(((aty/sf[282])*atO)/atY)}else{d})})});let auD=(if sb[52]{d}else{(if EE{(sf[291]*atz)}else{(if Ex{(((atz/sf[282])*atO)/atY)}else{d})})});let auE=(if sb[52]{d}else{(if EE{(sf[291]*atA)}else{(if Ex{(((atA/sf[282])*atO)/atY)}else{d})})});let auF=(if sb[52]{d}else{(if EE{(sf[291]*atB)}else{(if Ex{(((atB/sf[282])*atO)/atY)}else{d})})});let auG=(if sb[52]{d}else{(if EE{(sf[291]*atC)}else{(if Ex{(((atC/sf[282])*atO)/atY)}else{d})})});let auH=(if sb[52]{d}else{(if EE{(sf[291]*atD)}else{(if Ex{(((atD/sf[282])*atO)/atY)}else{d})})});let av4=(EK*(if (sf[266]!=0.0){(sf[7]*ajy)}else{ajy}));let avo=(EK*(sf[707]*acv));let avx=(EK*(if (sf[266]!=0.0){(arb+(CL*ar2))}else{d}));let awW=(F1*F1);let axb=(bL*(if (F4!=0.0){d}else{((-(sf[603]*((F0*a6O)+(uo*awE))))/awW)}));let axc=(bL*(if (F4!=0.0){d}else{((-(sf[603]*((F0*a6P)+(uo*awF))))/awW)}));let axd=(bL*(if (F4!=0.0){d}else{((-(sf[603]*((F0*a6Q)+(uo*awG))))/awW)}));let axe=(bL*(if (F4!=0.0){d}else{((-(sf[603]*((F0*a6R)+(uo*awH))))/awW)}));let axl=(F6*F6);let axC=((-a7g)/sf[295]);let axD=((-a7k)/sf[295]);let axE=((-a7o)/sf[295]);let axF=((-a7s)/sf[295]);let ay4=(if Fp{(FA*(if Fu{(Fv*axC)}else{(if Fq{(Fr*axC)}else{d})}))}else{d});let ay5=(if Fp{((FA*(if Fu{(Fv*axD)}else{(if Fq{(Fr*axD)}else{d})}))+(Fz*sf[351]))}else{d});let ay6=(if Fp{((FA*(if Fu{(Fv*axE)}else{(if Fq{(Fr*axE)}else{d})}))+(sf[0]*Fz))}else{d});let ay7=(if Fp{(FA*(if Fu{(Fv*axF)}else{(if Fq{(Fr*axF)}else{d})}))}else{d});let aya=(sf[296]*f64::powf(FC,sf[388]));let ayf=(sf[919]*(ay4*aya));let ayg=(sf[919]*(ay5*aya));let ayh=(sf[919]*(ay6*aya));let ayi=(sf[919]*(ay7*aya));
        let ayv=(if FN{(FO*ayf)}else{(if FJ{(FK*ayf)}else{d})});let ayw=(if FN{(FO*ayg)}else{(if FJ{(FK*ayg)}else{d})});let ayx=(if FN{(FO*ayh)}else{(if FJ{(FK*ayh)}else{d})});let ayy=(if FN{(FO*ayi)}else{(if FJ{(FK*ayi)}else{d})});let ayW=(s5*s5);let az5=(if G5{(((s5*sf[351])-(Gc*a1S))/ayW)}else{Yv});let az6=(if G5{(((sf[0]*s5)-(Gc*a1T))/ayW)}else{Yw});let az7=(if G5{((-(Gc*a1U))/ayW)}else{Yx});let aze=(I*Gh);let azi=(if G5{(((I*az5)/Gb)/aze)}else{d});let azj=(if G5{(((I*az6)/Gb)/aze)}else{d});let azk=(if G5{(((I*az7)/Gb)/aze)}else{d});let azr=(if Gp{(-(gj*a1A))}else{d});let azs=(if Gp{(-(gj*a1B))}else{d});let azt=(if Gp{(-(gj*a1C))}else{d});let azG=(if Gp{((Gt*azr)+(Gs*(sf[301]*azr)))}else{d});let azH=(if Gp{((Gt*azs)+(Gs*(sf[301]*azs)))}else{d});let azI=(if Gp{((Gt*azt)+(Gs*(sf[301]*azt)))}else{d});let azS=(Gi*azi);let azU=(Gi*azj);let azW=(Gi*azk);let azY=(Gv*azG);let aA0=(Gv*azH);let aA2=(Gv*azI);let aA7=(I*GA);let aAe=(GA*GA);let aAo=(if G5{(((GA*((Gv*azi)+(Gi*azG)))-(Gw*(((azS+azS)+(azY+azY))/aA7)))/aAe)}else{d});let aAp=(if G5{(((GA*((Gv*azj)+(Gi*azH)))-(Gw*(((azU+azU)+(aA0+aA0))/aA7)))/aAe)}else{d});let aAq=(if G5{(((GA*((Gv*azk)+(Gi*azI)))-(Gw*(((azW+azW)+(aA2+aA2))/aA7)))/aAe)}else{d});let aAu=(GC*GC);let aAD=(if G5{(((GC*sf[351])-(Gc*aAo))/aAu)}else{d});let aAE=(if G5{(((sf[0]*GC)-(Gc*aAp))/aAu)}else{d});let aAF=(if G5{((-(Gc*aAq))/aAu)}else{d});let aAG=(gj*aAo);let aAH=(gj*aAp);let aAI=(gj*aAq);let aAJ=(Gb*aAG);let aAK=(Gb*aAH);let aAL=(Gb*aAI);let aAY=(if G5{(aAD+((GG*a1S)+(s5*aAJ)))}else{d});let aAZ=(if G5{(aAE+((GG*a1T)+(s5*aAK)))}else{d});let aB0=(if G5{(aAF+((GG*a1U)+(s5*aAL)))}else{d});let aBk=(GW*GW);let aBM=(if Gp{(-(GG*(-(a7g/GW))))}else{d});let aBN=(if Gp{(aAD-((GY*aAJ)+(GG*(-(((GW*a7k)-(uv*(sf[217]*(if Gp{(sf[307]*(I*a1A))}else{d}))))/aBk)))))}else{d});let aBO=(if Gp{(aAE-((GY*aAK)+(GG*(-(((GW*a7o)-(uv*(sf[217]*(if Gp{(sf[307]*(I*a1B))}else{d}))))/aBk)))))}else{d});let aBP=(if Gp{(aAF-((GY*aAL)+(GG*(-(((GW*a7s)-(uv*(sf[217]*(if Gp{(sf[307]*(I*a1C))}else{d}))))/aBk)))))}else{d});let aBT=(H2*aBM);let aBV=(H2*(aBN-aAY));let aBX=(H2*(aBO-aAZ));let aBZ=(H2*(aBP-aB0));let aCz=(I*Hb);let aCM=(if Gp{(gj*(aBM+((if Gp{(aBT+aBT)}else{d})/aCz)))}else{d});let aCN=(if Gp{(gj*((aAY+aBN)+((if Gp{((aBV+aBV)+(((H5*a1J)+(s2*((H4*aAD)+(GE*(X*aAD)))))/sf[217]))}else{az5})/aCz)))}else{(if Gm{aAY}else{d})});let aCO=(if Gp{(gj*((aAZ+aBO)+((if Gp{((aBX+aBX)+(((H5*a1K)+(s2*((H4*aAE)+(GE*(X*aAE)))))/sf[217]))}else{az6})/aCz)))}else{(if Gm{aAZ}else{d})});let aCP=(if Gp{(gj*((aB0+aBP)+((if Gp{((aBZ+aBZ)+(((H5*a1L)+(s2*((H4*aAF)+(GE*(X*aAF)))))/sf[217]))}else{az7})/aCz)))}else{(if Gm{aB0}else{d})});let aCW=(He*He);let aDg=(Hh*Hh);let aDu=(if Hm{((-(GF*(if G5{(((He*aCM)-(Hf*aCM))/aCW)}else{d})))/aDg)}else{d});let aDv=(if Hm{(((Hh*aAG)-(GF*(if G5{(((He*(aCN-aAD))-(Hf*aCN))/aCW)}else{d})))/aDg)}else{d});let aDw=(if Hm{(((Hh*aAH)-(GF*(if G5{(((He*(aCO-aAE))-(Hf*aCO))/aCW)}else{d})))/aDg)}else{d});let aDx=(if Hm{(((Hh*aAI)-(GF*(if G5{(((He*(aCP-aAF))-(Hf*aCP))/aCW)}else{d})))/aDg)}else{d});let aDQ=((-(sf[922]*aCM))/aCW);let aDT=((-(sf[922]*aCN))/aCW);let aDW=((-(sf[922]*aCO))/aCW);let aDZ=((-(sf[922]*aCP))/aCW);let aE0=(Hu*aDQ);let aE1=(Hu*aDT);let aE2=(Hu*aDW);let aE3=(Hu*aDZ);let aE6=(Ho*Ho);let aFc=(sf[296]*f64::powf(FA,sf[388]));let aFi=(HP*HP);let aFC=(sf[313]*f64::powf(HR,sf[389]));let aFP=(if HM{(HN*((-(((HP*a7g)-(uv*a7g))/aFi))*aFC))}else{d});let aFQ=(if HM{((HT*(sf[351]*aFc))+(HN*((-(((HP*a7k)-(uv*a7k))/aFi))*aFC)))}else{d});let aFR=(if HM{((HT*(sf[0]*aFc))+(HN*((-(((HP*a7o)-(uv*a7o))/aFi))*aFC)))}else{d});let aFS=(if HM{(HN*((-(((HP*a7s)-(uv*a7s))/aFi))*aFC))}else{d});let aG1=(if HY{(a7g/sf[312])}else{d});let aG2=(if HY{(a7k/sf[312])}else{d});let aG3=(if HY{(a7o/sf[312])}else{d});let aG4=(if HY{(a7s/sf[312])}else{d});let aG9=(if HY{(aG1/sf[315])}else{sf[364]});let aGa=(if HY{(aG2/sf[315])}else{sf[365]});let aGb=(if HY{(aG3/sf[315])}else{d});let aGc=(if HY{(aG4/sf[315])}else{d});let aGT=(sf[316]*f64::powf(Io,sf[390]));
        let aHe=(sf[919]*(if HY{((Iq*aFP)+(HV*((if Ih{(aG1+(sf[315]*((Ij*(-aG9))/Ik)))}else{(if I9{(sf[315]*((Ia*aG9)/Ib))}else{d})})*aGT)))}else{(if HW{aFP}else{d})}));let aHf=(sf[919]*(if HY{((Iq*aFQ)+(HV*((if Ih{(aG2+(sf[315]*((Ij*(-aGa))/Ik)))}else{(if I9{(sf[315]*((Ia*aGa)/Ib))}else{d})})*aGT)))}else{(if HW{aFQ}else{d})}));let aHg=(sf[919]*(if HY{((Iq*aFR)+(HV*((if Ih{(aG3+(sf[315]*((Ij*(-aGb))/Ik)))}else{(if I9{(sf[315]*((Ia*aGb)/Ib))}else{d})})*aGT)))}else{(if HW{aFR}else{d})}));let aHh=(sf[919]*(if HY{((Iq*aFS)+(HV*((if Ih{(aG4+(sf[315]*((Ij*(-aGc))/Ik)))}else{(if I9{(sf[315]*((Ia*aGc)/Ib))}else{d})})*aGT)))}else{(if HW{aFS}else{d})}));let aHI=(if HM{(IG*(if IA{(IB*aHe)}else{(if Iw{(Ix*aHe)}else{ayv})}))}else{(if HD{(HE*aE0)}else{(if Hm{((Hz*((Hq*aDu)+(Ho*(sf[921]*aCM))))+(Hr*(aE0-(Hy*((Hw*aDQ)+(Ht*((-(Gv*aDu))/aE6)))))))}else{(if Fp{((FV*ayv)+(FS*(sf[920]*ay4)))}else{d})})})});let aHJ=(if HM{((IG*(if IA{(IB*aHf)}else{(if Iw{(Ix*aHf)}else{ayw})}))+(IF*sf[982]))}else{(if HD{((HE*aE1)+(Hu*(sf[4]*azG)))}else{(if Hm{((Hz*((Hq*aDv)+(Ho*(sf[921]*aCN))))+(Hr*(aE1-(Hy*((Hw*aDT)+(Ht*(((Ho*azG)-(Gv*aDv))/aE6)))))))}else{(if Fp{((FV*ayw)+(FS*(sf[920]*ay5)))}else{d})})})});let aHK=(if HM{((IG*(if IA{(IB*aHg)}else{(if Iw{(Ix*aHg)}else{ayx})}))+(IF*sf[983]))}else{(if HD{((HE*aE2)+(Hu*(sf[4]*azH)))}else{(if Hm{((Hz*((Hq*aDw)+(Ho*(sf[921]*aCO))))+(Hr*(aE2-(Hy*((Hw*aDW)+(Ht*(((Ho*azH)-(Gv*aDw))/aE6)))))))}else{(if Fp{((FV*ayx)+(FS*(sf[920]*ay6)))}else{d})})})});let aHL=(if HM{(IG*(if IA{(IB*aHh)}else{(if Iw{(Ix*aHh)}else{ayy})}))}else{(if HD{((HE*aE3)+(Hu*(sf[4]*azI)))}else{(if Hm{((Hz*((Hq*aDx)+(Ho*(sf[921]*aCP))))+(Hr*(aE3-(Hy*((Hw*aDZ)+(Ht*(((Ho*azI)-(Gv*aDx))/aE6)))))))}else{(if Fp{((FV*ayy)+(FS*(sf[920]*ay7)))}else{d})})})});let aI0=(IR*IR);let aIp=(IQ*IQ);let aIE=(if IP{((((-(sf[411]*((IQ*a7g)+(uv*axb))))/aI0)+(sf[715]*(a6U/sf[687])))+((-(sf[600]*axb))/aIp))}else{d});let aIF=(if IP{((((-(sf[411]*((IQ*a7k)+(uv*axc))))/aI0)+(sf[715]*(a6X/sf[687])))+((-(sf[600]*axc))/aIp))}else{d});let aIG=(if IP{((((-(sf[411]*((IQ*a7o)+(uv*axd))))/aI0)+(sf[715]*(a70/sf[687])))+((-(sf[600]*axd))/aIp))}else{d});let aIH=(if IP{((((-(sf[411]*((IQ*a7s)+(uv*axe))))/aI0)+(sf[715]*(a73/sf[687])))+((-(sf[600]*axe))/aIp))}else{d});let aIQ=(if IZ{((aHI-aIE)/gf)}else{aG9});let aIR=(if IZ{((aHJ-aIF)/gf)}else{aGa});let aIS=(if IZ{((aHK-aIG)/gf)}else{aGb});let aIT=(if IZ{((aHL-aIH)/gf)}else{aGc});let aJy=(if Jd{(aIE-(gf*((Jf*(-aIQ))/Jg)))}else{(if J5{(aHI-(gf*((J6*aIQ)/J7)))}else{aHI})});let aJz=(if Jd{(aIF-(gf*((Jf*(-aIR))/Jg)))}else{(if J5{(aHJ-(gf*((J6*aIR)/J7)))}else{aHJ})});let aJA=(if Jd{(aIG-(gf*((Jf*(-aIS))/Jg)))}else{(if J5{(aHK-(gf*((J6*aIS)/J7)))}else{aHK})});let aJB=(if Jd{(aIH-(gf*((Jf*(-aIT))/Jg)))}else{(if J5{(aHL-(gf*((J6*aIT)/J7)))}else{aHL})});let aJE=((Jk*a7g)+(uv*aJy));let aJH=((Jk*a7k)+(uv*aJz));let aJK=((Jk*a7o)+(uv*aJA));let aJN=((Jk*a7s)+(uv*aJB));let aKb=(Jq*Jq);let b02=(sf[15]*(sf[0]*(sf[753]*acQ)));let b06=((sf[377]+((if sb[33]{(sf[715]*((sf[248]*a8L)+(we*(sf[246]*a8L))))}else{(if sb[31]{a9f}else{(if (sf[154]!=0.0){((a9f+(we*(((wc*(sf[892]*a8L))-(w8*((gv*a8T)/a9l)))/a9r)))+(((wk*(wi*a9b))-(wj*a9b))/a9V))}else{d})})})+(sf[700]*abQ)))-(if zC{d}else{(if (yb!=0.0){(sf[22]*(sf[579]*((zx*(if ym{(yn*ad8)}else{(if yi{(yj*ad8)}else{d})}))+(yr*((zw*a2q)+(sw*(sf[894]*(if zj{((zs*((zk*ael)+(yV*sf[371])))+(zl*((zq*(zm*ael))+(zn*(zo*ael)))))}else{(if z1{((sf[0]*zf)+(zc*(((yV*(-(if z6{(z7*ael)}else{(if z2{(z3*ael)}else{d})})))-(zd*ael))/aeA)))}else{d})}))))))))}else{d})}));
        let b07=((sf[376]+((if sb[33]{(sf[715]*((sf[248]*a8M)+((wA*a5k)+(we*(sf[246]*(a0T+a8M))))))}else{(if sb[31]{a9g}else{(if (sf[154]!=0.0){((a9g+((we*(((wc*(sf[892]*a8M))-(w8*((gv*a8U)/a9l)))/a9r))+(wd*a5k)))+(((wk*((wi*a9c)+(w4*(sf[730]*a0T))))-(wj*a9c))/a9V))}else{d})})})+(sf[700]*abS)))-(if zC{d}else{(if (yb!=0.0){(sf[22]*(sf[579]*((zx*(if ym{(yn*ad9)}else{(if yi{(yj*ad9)}else{d})}))+(yr*((zw*a2r)+(sw*(sf[894]*(if zj{((zs*((zk*aem)+(yV*sf[372])))+(zl*((zq*(zm*aem))+(zn*(zo*aem)))))}else{(if z1{((zf*sf[351])+(zc*(((yV*(-(if z6{(z7*aem)}else{(if z2{(z3*aem)}else{d})})))-(zd*aem))/aeA)))}else{d})}))))))))}else{d})}));let b0E=(sf[15]*(sf[0]*(-(B8*auz))));let b0F=(sf[15]*(sf[0]*(-(B8*auA))));let b0G=(sf[15]*(sf[0]*(-(B8*auB))));let b0H=(sf[15]*(sf[0]*(-(B8*auC))));let b0I=(sf[15]*(sf[0]*(-((EK*(if B7{d}else{(if (zJ!=0.0){(sf[54]*(sf[580]*((B2*(if zY{(zZ*afV)}else{(if zU{(zV*afV)}else{d})}))+(A3*((B1*afI)+(zN*(sf[895]*(if AQ{((AX*((AR*ah6)+(At*sf[372])))+(AS*((AV*(zm*ah6))+(AT*(zo*ah6)))))}else{(if Ay{((AM*sf[351])+(AJ*(((At*(-(if AD{(AE*ah6)}else{(if Az{(AA*ah6)}else{d})})))-(AK*ah6))/ahl)))}else{d})}))))))))}else{d})}))+(B8*auD)))));let b0J=(sf[15]*(sf[0]*(-((EK*(if B7{d}else{(if (zJ!=0.0){(sf[54]*(sf[580]*((B2*(if zY{(zZ*afW)}else{(if zU{(zV*afW)}else{d})}))+(A3*((B1*afJ)+(zN*(sf[895]*(if AQ{((AX*((AR*ah7)+(At*sf[371])))+(AS*((AV*(zm*ah7))+(AT*(zo*ah7)))))}else{(if Ay{((sf[0]*AM)+(AJ*(((At*(-(if AD{(AE*ah7)}else{(if Az{(AA*ah7)}else{d})})))-(AK*ah7))/ahl)))}else{d})}))))))))}else{d})}))+(B8*auE)))));let b0K=(sf[15]*(sf[0]*(-(B8*auF))));let b0L=(sf[15]*(sf[0]*(-(B8*auG))));let b0M=(sf[15]*(sf[0]*(-(B8*auH))));let b1y=(sf[15]*(sf[0]*(if (sf[266]!=0.0){(arE+(D9*ar2))}else{d})));let b2j=ddt_scale;let b4g=(sf[15]*(b2j*b3Y));let b4W=(sf[15]*(b2j*b4O));

        stamper.stamp_current_node3_local(
            Some(7),
            Some(8),
            multiplicity * ((sf[15]*(sf[0]*oE))),
            6,
            multiplicity * ((sf[15]*(sf[0]*TQ))),
            7,
            multiplicity * ((sf[15]*(sf[0]*TR))),
            8,
            multiplicity * ((sf[15]*(sf[0]*TS))),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(4),
            multiplicity * ((sf[15]*(sf[0]*uv))),
            [4, 6, 7, 8],
            [(sf[15]*(sf[0]*a7g)), (sf[15]*(sf[0]*a7k)), (sf[15]*(sf[0]*a7o)), (sf[15]*(sf[0]*a7s))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(4),
            multiplicity * ((sf[15]*(sf[0]*((sf[753]*(y4-b))+((if sb[30]{x6}else{(if (sf[154]!=0.0){(x6+(x8/xc))}else{d})})+(sf[747]*(xE-b))))))),
            [4, 5, 6, 7, 8, 10],
            [(sf[15]*(sf[0]*((sf[753]*acN)+((if sb[30]{ab8}else{(if (sf[154]!=0.0){(ab8+(((xc*(sf[893]*aaV))-(x8*((gv*(if wZ{(x0*sf[945])}else{(if wV{(wW*sf[945])}else{a8T})}))/abh)))/abo))}else{d})})+(sf[747]*ac5))))), (sf[15]*(sf[0]*((sf[753]*acO)+((if sb[30]{ab9}else{(if (sf[154]!=0.0){(ab9+(((xc*(sf[893]*aaW))-(x8*((gv*(if wZ{(x0*sf[944])}else{(if wV{(wW*sf[944])}else{d})}))/abh)))/abo))}else{d})})+(sf[747]*ac6))))), (sf[15]*(sf[0]*((sf[753]*acP)+((if sb[30]{aba}else{(if (sf[154]!=0.0){(aba+(((xc*(sf[893]*aaX))-(x8*((gv*(if wZ{d}else{(if wV{d}else{a8U})}))/abh)))/abo))}else{d})})+(sf[747]*ac7))))), b02, b02, (sf[15]*(sf[0]*(sf[753]*acR)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(4),
            multiplicity * ((sf[15]*(sf[0]*((sf[758]*(uY-b))+((vl*vn)+((((if sb[33]{(sf[715]*((w5*sf[248])+(we*wA)))}else{(if sb[31]{w6}else{(if (sf[154]!=0.0){((w6+(wd*we))+(wj/wk))}else{d})})})+(sf[700]*(xr-b)))+(d*lt))-(if zC{d}else{(if (yb!=0.0){(sf[22]*(sf[579]*(yr*zx)))}else{d})}))))))),
            [4, 5, 6, 7, 8],
            [(sf[15]*(sf[0]*((sf[758]*a7X)+(((vn*(sf[245]*a8n))+(vl*((-a8n)*a8u)))+b06)))), (sf[15]*(sf[0]*(sf[700]*abR))), (sf[15]*(sf[0]*((sf[758]*a7Y)+(((vn*(sf[245]*a8o))+(vl*((-a8o)*a8u)))+b07)))), (sf[15]*(sf[0]*(if sb[33]{(sf[715]*((wA*a5l)+(we*(sf[246]*a0U))))}else{(if sb[31]{d}else{(if (sf[154]!=0.0){((wd*a5l)+(((wk*((wi*a9d)+(w4*(sf[730]*a0U))))-(wj*a9d))/a9V))}else{d})})}))), (sf[15]*(sf[0]*(if sb[33]{(sf[715]*((wA*a5m)+(we*(sf[246]*a0V))))}else{(if sb[31]{d}else{(if (sf[154]!=0.0){((wd*a5m)+(((wk*((wi*a9e)+(w4*(sf[730]*a0V))))-(wj*a9e))/a9V))}else{d})})})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * ((if (sf[154]!=0.0){PD}else{d})),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [(if (sf[154]!=0.0){b0E}else{d}), (if (sf[154]!=0.0){b0F}else{d}), (if (sf[154]!=0.0){b0G}else{d}), (if (sf[154]!=0.0){b0H}else{d}), (if (sf[154]!=0.0){b0I}else{d}), (if (sf[154]!=0.0){b0J}else{d}), (if (sf[154]!=0.0){b0K}else{d}), (if (sf[154]!=0.0){b0L}else{d}), (if (sf[154]!=0.0){b0M}else{d})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(8),
            multiplicity * ((if sb[30]{PD}else{d})),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [(if sb[30]{b0E}else{d}), (if sb[30]{b0F}else{d}), (if sb[30]{b0G}else{d}), (if sb[30]{b0H}else{d}), (if sb[30]{b0I}else{d}), (if sb[30]{b0J}else{d}), (if sb[30]{b0K}else{d}), (if sb[30]{b0L}else{d}), (if sb[30]{b0M}else{d})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*(if (sf[266]!=0.0){(sf[7]*Cf)}else{Cf})))),
            [3, 5, 6, 7, 8, 10],
            [(sf[15]*(sf[0]*(if (sf[266]!=0.0){(sf[7]*alP)}else{alP}))), (sf[15]*(sf[0]*(if (sf[266]!=0.0){(sf[7]*alQ)}else{alQ}))), (sf[15]*(sf[0]*(if (sf[266]!=0.0){(sf[7]*alR)}else{alR}))), (sf[15]*(sf[0]*(if (sf[266]!=0.0){(sf[7]*alS)}else{alS}))), (sf[15]*(sf[0]*(if (sf[266]!=0.0){(sf[7]*alT)}else{alT}))), (sf[15]*(sf[0]*(if (sf[266]!=0.0){(sf[7]*alU)}else{alU})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*(if sb[41]{(C2/C6)}else{(if (sf[258]!=0.0){(BB/BK)}else{d})})))),
            [3, 6, 7, 8],
            [(sf[15]*(sf[0]*(if sb[41]{d}else{(if (sf[258]!=0.0){(((BK*(sf[899]*(-RW)))-(BB*((sf[901]*(sf[261]*RW))/ajP)))/ajX)}else{d})}))), (sf[15]*(sf[0]*(if sb[41]{(((C6*ajG)-(C2*(ajM/alb)))/alh)}else{(if (sf[258]!=0.0){(((BK*ajG)-(BB*(ajM/ajP)))/ajX)}else{d})}))), (sf[15]*(sf[0]*(if sb[41]{d}else{(if (sf[258]!=0.0){(((BK*(sf[899]*(-RX)))-(BB*((sf[901]*(sf[261]*RX))/ajP)))/ajX)}else{d})}))), (sf[15]*(sf[0]*(if sb[41]{(((C6*ajI)-(C2*(ajO/alb)))/alh)}else{(if (sf[258]!=0.0){(((BK*ajI)-(BB*(ajO/ajP)))/ajX)}else{d})})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*(if (sf[266]!=0.0){(D9*DO)}else{d})))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [b1y, (sf[15]*(sf[0]*(if (sf[266]!=0.0){((DO*aos)+(D9*ar3))}else{d}))), (sf[15]*(sf[0]*(if (sf[266]!=0.0){((DO*aot)+(D9*ar4))}else{d}))), (sf[15]*(sf[0]*(if (sf[266]!=0.0){(D9*ar5)}else{d}))), b1y, (sf[15]*(sf[0]*(if (sf[266]!=0.0){(arE+(D9*ar6))}else{d}))), (sf[15]*(sf[0]*(if (sf[266]!=0.0){(arQ+(D9*ar7))}else{d}))), (sf[15]*(sf[0]*(if (sf[266]!=0.0){((DO*aov)+(D9*ar8))}else{d}))), (sf[15]*(sf[0]*(if (sf[266]!=0.0){((DO*aow)+(D9*ar9))}else{d}))), (sf[15]*(sf[0]*(if (sf[266]!=0.0){(arQ+(D9*ara))}else{d})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * ((sf[15]*(sf[0]*((Ci/Cp)+(d*lB))))),
            3,
            multiplicity * ((sf[15]*(sf[0]*((((Cp*(sf[903]*RW))-(Ci*((sf[905]*RW)/alZ)))/am5)+sf[376])))),
            7,
            multiplicity * ((sf[15]*(sf[0]*((((Cp*(sf[903]*RX))-(Ci*((sf[905]*RX)/alZ)))/am5)+sf[377])))),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * ((sf[15]*(sf[0]*(F9/F6)))),
            [4, 5, 6, 7, 8],
            [(sf[15]*(sf[0]*((-(F9*axb))/axl))), (sf[15]*(sf[0]*((sf[0]+(sf[865]*(if mK{(mL*sf[944])}else{(if (mH!=0.0){(mI*sf[944])}else{d})})))/F6))), (sf[15]*(sf[0]*(((F6*(sf[351]+(sf[865]*(if mK{(mL*sf[945])}else{(if (mH!=0.0){(mI*sf[945])}else{d})}))))-(F9*axc))/axl))), (sf[15]*(sf[0]*((-(F9*axd))/axl))), (sf[15]*(sf[0]*((-(F9*axe))/axl)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * ((sf[15]*(sf[0]*(-Jv)))),
            [4, 6, 7, 8],
            [(sf[15]*(sf[0]*(-(if Ju{aJE}else{(if Jo{(((Jq*((Jl*aIE)+(IY*aJE)))-(Jp*(aIE+aJy)))/aKb)}else{(if IZ{aJE}else{d})})})))), (sf[15]*(sf[0]*(-(if Ju{aJH}else{(if Jo{(((Jq*((Jl*aIF)+(IY*aJH)))-(Jp*(aIF+aJz)))/aKb)}else{(if IZ{aJH}else{d})})})))), (sf[15]*(sf[0]*(-(if Ju{aJK}else{(if Jo{(((Jq*((Jl*aIG)+(IY*aJK)))-(Jp*(aIG+aJA)))/aKb)}else{(if IZ{aJK}else{d})})})))), (sf[15]*(sf[0]*(-(if Ju{aJN}else{(if Jo{(((Jq*((Jl*aIH)+(IY*aJN)))-(Jp*(aIH+aJB)))/aKb)}else{(if IZ{aJN}else{d})})}))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(4),
            multiplicity * ((sf[15]*((sf[0]*(sf[0]*(lE-lr)))/sf[600]))),
            2,
            multiplicity * (sf[1013]),
            4,
            multiplicity * (sf[1014]),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(5),
            multiplicity * ((sf[15]*((sf[0]*lJ)/sf[608]))),
            1,
            multiplicity * (sf[1017]),
            5,
            multiplicity * (sf[1018]),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(4),
            multiplicity * ((sf[15]*PZ)),
            [4, 5, 6, 7, 8, 10],
            [(sf[15]*(b2d*b2j)), (sf[15]*(b2e*b2j)), (sf[15]*(b2f*b2j)), (sf[15]*(b2g*b2j)), (sf[15]*(b2h*b2j)), (sf[15]*(b2i*b2j))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(5),
            Some(4),
            multiplicity * ((sf[15]*Q2)),
            4,
            multiplicity * ((sf[15]*(b2j*b2w))),
            5,
            multiplicity * ((sf[15]*(b2j*b2x))),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(8),
            multiplicity * ((sf[15]*Q5)),
            [4, 5, 6, 7, 8, 10],
            [(sf[15]*(b2j*b2C)), (sf[15]*(b2j*b2D)), (sf[15]*(b2j*b2E)), (sf[15]*(b2j*b2F)), (sf[15]*(b2j*b2G)), (sf[15]*(b2j*b2H))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * ((sf[15]*Q8)),
            3,
            multiplicity * ((sf[15]*(b2j*b2U))),
            7,
            multiplicity * ((sf[15]*(b2j*b2V))),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * ((sf[15]*Qb)),
            [4, 5, 6, 7, 8, 10],
            [(sf[15]*(b2j*b30)), (sf[15]*(b2j*b31)), (sf[15]*(b2j*b32)), (sf[15]*(b2j*b33)), (sf[15]*(b2j*b34)), (sf[15]*(b2j*b35))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * ((sf[15]*Qf)),
            1,
            multiplicity * ((sf[15]*(b2j*sf[403]))),
            2,
            multiplicity * ((sf[15]*(b2j*sf[404]))),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * ((sf[15]*Qj)),
            0,
            multiplicity * ((sf[15]*(b2j*sf[405]))),
            1,
            multiplicity * ((sf[15]*(b2j*sf[406]))),
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * ((sf[15]*(sf[0]*(DQ*EK)))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [(sf[15]*(sf[0]*(avx+(DQ*auz)))), (sf[15]*(sf[0]*((EK*(if (sf[266]!=0.0){((DO*an4)+(CL*ar3))}else{d}))+(DQ*auA)))), (sf[15]*(sf[0]*(EK*(if (sf[266]!=0.0){(CL*ar4)}else{d})))), (sf[15]*(sf[0]*((EK*(if (sf[266]!=0.0){(CL*ar5)}else{d}))+(DQ*auB)))), (sf[15]*(sf[0]*(avx+(DQ*auC)))), (sf[15]*(sf[0]*((EK*(if (sf[266]!=0.0){(arb+(CL*ar6))}else{d}))+(DQ*auD)))), (sf[15]*(sf[0]*((EK*(if (sf[266]!=0.0){(arl+(CL*ar7))}else{d}))+(DQ*auE)))), (sf[15]*(sf[0]*((EK*(if (sf[266]!=0.0){(arl+(CL*ar8))}else{d}))+(DQ*auF)))), (sf[15]*(sf[0]*((EK*(if (sf[266]!=0.0){((DO*an6)+(CL*ar9))}else{d}))+(DQ*auG)))), (sf[15]*(sf[0]*((EK*(if (sf[266]!=0.0){(arl+(CL*ara))}else{d}))+(DQ*auH))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(9),
            multiplicity * ((sf[15]*(sf[854]*(sf[0]*m1)))),
            [0, 1, 5, 6, 7, 8, 9, 10],
            [sf[1023], sf[1024], sf[1024], sf[1024], sf[1025], sf[1025], sf[1026], sf[1025]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * ((sf[15]*Qr)),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [b4g, (sf[15]*(b2j*b3Z)), (sf[15]*(b2j*b40)), (sf[15]*(b2j*b41)), b4g, (sf[15]*(b2j*b42)), (sf[15]*(b2j*b43)), (sf[15]*(b2j*b44)), (sf[15]*(b2j*b45)), (sf[15]*(b2j*b46))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(10),
            multiplicity * ((sf[15]*(sf[0]*((Cz*EK)+((xT*EK)+(d*lX)))))),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [(sf[15]*(sf[0]*((Cz*auz)+(xT*auz)))), (sf[15]*(sf[0]*((Cz*auA)+(xT*auA)))), (sf[15]*(sf[0]*((Cz*auB)+((EK*(sf[707]*acs))+(xT*auB))))), (sf[15]*(sf[0]*(((EK*(if (sf[266]!=0.0){(sf[7]*ajq)}else{ajq}))+(Cz*auC))+(sf[376]+((EK*(sf[707]*act))+(xT*auC)))))), (sf[15]*(sf[0]*(((EK*(if (sf[266]!=0.0){(sf[7]*aju)}else{aju}))+(Cz*auD))+(((EK*(sf[707]*acu))+(xT*auD))+sf[399])))), (sf[15]*(sf[0]*((av4+(Cz*auE))+((avo+(xT*auE))+sf[400])))), (sf[15]*(sf[0]*((av4+(Cz*auF))+((avo+(xT*auF))+sf[400])))), (sf[15]*(sf[0]*((Cz*auG)+(xT*auG)))), (sf[15]*(sf[0]*(((EK*(if (sf[266]!=0.0){(sf[7]*ajC)}else{ajC}))+(Cz*auH))+(sf[377]+((EK*(sf[707]*acw))+(xT*auH))))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(10),
            multiplicity * ((sf[15]*Qx)),
            [5, 6, 7, 8, 10],
            [(sf[15]*(b2j*b4M)), (sf[15]*(b2j*b4N)), b4W, b4W, (sf[15]*(b2j*b4P))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(9),
            Some(10),
            multiplicity * ((if (sf[212]!=0.0){(sf[15]*(sf[859]*(sf[0]*lU)))}else{d})),
            9,
            multiplicity * (sf[1031]),
            10,
            multiplicity * (sf[1032]),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(10),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            d,
        );
        stamper.stamp_current_node2_local(
            Some(10),
            Some(7),
            multiplicity * ((if (sf[213]!=0.0){(sf[15]*(sf[864]*(sf[0]*lR)))}else{d})),
            7,
            multiplicity * (sf[1037]),
            10,
            multiplicity * (sf[1038]),
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(7),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            d,
        );
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * (d),
        );
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (QH),
            11,
            multiplicity * (b),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(4),
            multiplicity * ((P9*QI)),
            [4, 5, 6, 7, 8, 10, 11],
            [(QI*aYO), (QI*aYP), (QI*aYQ), (QI*aYR), (QI*aYS), (QI*aYT), (P9*b2j)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(8),
            Some(6),
            multiplicity * ((OL*QH)),
            11,
            multiplicity * (OL),
        );
        stamper.stamp_current_node1_local(
            Some(8),
            Some(4),
            multiplicity * (QH),
            11,
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(4),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(4),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(4),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(6),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(3),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(3),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(10),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(7),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(7),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(10),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(7),
            multiplicity * (d),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(7),
            multiplicity * (d),
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
        let CommonStampValues {
            b, d, H, I, X, bL, gf, gj, 
            gv, gV, ln, lr, lt, ly, lB, lE, 
            lJ, lR, lU, lX, m1, mh, mE, mF, 
            mH, mK, mL, n1, n3, n6, n7, nn, 
            np, ns, nt, oE, qC, rA, rZ, s2, 
            s5, sw, tO, uo, up, uu, uv, uO, 
            uQ, uT, uU, v3, vz, vB, vD, vI, 
            vJ, vQ, vR, vT, vY, w0, wQ, wS, 
            wU, wZ, x0, xr, xE, xR, y4, yb, 
            yc, yf, yh, ym, yn, yt, yx, yA, 
            yI, yJ, yK, yM, yO, yS, yT, yV, 
            yY, z0, z1, z6, z7, zJ, zL, zN, 
            zO, zR, zT, zY, zZ, A4, A7, A9, 
            Ah, Ai, Aj, Al, Aq, Ar, At, Av, 
            Ax, Ay, AD, AE, CL, D9, Dr, DO, 
            F0, Fc, Fp, Fq, Fr, Fu, Fv, Fz, 
            FA, FC, FG, FI, FN, FO, G3, HM, 
            HN, HP, HR, HT, HV, HW, HY, I6, 
            I9, Ia, Ib, Ih, Ij, Ik, Io, Iq, 
            It, Iv, IA, IB, OD, P9, PY, Q1, 
            Q4, Q7, Qa, Qe, Qi, Qq, Qw, QH, 
            QX, QY, Rn, Ro, Rp, Rq, TQ, TR, 
            TS, Yv, Yw, Yx, a0T, a0U, a0V, a1A, 
            a1B, a1C, a1J, a1K, a1L, a1S, a1T, a1U, 
            a2q, a2r, a5k, a5l, a5m, a6O, a6P, a6Q, 
            a6R, a6U, a6X, a70, a73, a74, a75, a76, 
            a78, a7c, a7f, a7N, a7O, a8L, a8M, aaV, 
            aaW, aaX, abQ, abR, abS, ac5, ac6, ac7, 
            acs, act, acu, acv, acw, acN, acO, acP, 
            acQ, acR, an3, an4, an5, an6, aor, aos, 
            aot, aou, aov, aow, aoJ, aoK, aoL, aoM, 
            aoN, aoO, aoP, aoQ, ar2, ar3, ar4, ar5, 
            ar6, ar7, ar8, ar9, ara, awE, awF, awG, 
            awH, aYO, aYP, aYQ, aYR, aYS, aYT, b2d, 
            b2e, b2f, b2g, b2h, b2i, b2w, b2x, b2C, 
            b2D, b2E, b2F, b2G, b2H, b2U, b2V, b30, 
            b31, b32, b33, b34, b35, b3Y, b3Z, b40, 
            b41, b42, b43, b44, b45, b46, b4M, b4N, 
            b4O, b4P, 
        }=self.eval_common_stamp_values(ctx);
        let PZ=0.0;let Q2=0.0;let Q5=0.0;let Q8=0.0;let Qb=0.0;let Qf=0.0;let Qj=0.0;let Qr=0.0;let Qx=0.0;let QI=0.0;let b2j=1.0;let b4g=(sf[15]*(b2j*b3Y));let b4W=(sf[15]*(b2j*b4O));

        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(sf[15]*(b2d*b2j)), (sf[15]*(b2e*b2j)), (sf[15]*(b2f*b2j)), (sf[15]*(b2g*b2j)), (sf[15]*(b2h*b2j)), (sf[15]*(b2i*b2j))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes[4],
            multiplicity * ((sf[15]*(b2j*b2w))),
            nodes[5],
            multiplicity * ((sf[15]*(b2j*b2x))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(sf[15]*(b2j*b2C)), (sf[15]*(b2j*b2D)), (sf[15]*(b2j*b2E)), (sf[15]*(b2j*b2F)), (sf[15]*(b2j*b2G)), (sf[15]*(b2j*b2H))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes[3],
            multiplicity * ((sf[15]*(b2j*b2U))),
            nodes[7],
            multiplicity * ((sf[15]*(b2j*b2V))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(sf[15]*(b2j*b30)), (sf[15]*(b2j*b31)), (sf[15]*(b2j*b32)), (sf[15]*(b2j*b33)), (sf[15]*(b2j*b34)), (sf[15]*(b2j*b35))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * ((sf[15]*(b2j*sf[403]))),
            nodes[2],
            multiplicity * ((sf[15]*(b2j*sf[404]))),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * ((sf[15]*(b2j*sf[405]))),
            nodes[1],
            multiplicity * ((sf[15]*(b2j*sf[406]))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            &[nodes[0], nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10]],
            &[b4g, (sf[15]*(b2j*b3Z)), (sf[15]*(b2j*b40)), (sf[15]*(b2j*b41)), b4g, (sf[15]*(b2j*b42)), (sf[15]*(b2j*b43)), (sf[15]*(b2j*b44)), (sf[15]*(b2j*b45)), (sf[15]*(b2j*b46))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            &[nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(sf[15]*(b2j*b4M)), (sf[15]*(b2j*b4N)), b4W, b4W, (sf[15]*(b2j*b4P))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10], nodes[11]],
            &[(QI*aYO), (QI*aYP), (QI*aYQ), (QI*aYR), (QI*aYS), (QI*aYT), (P9*b2j)],
            &[],
            &[],
            multiplicity,
        );
    }
}
