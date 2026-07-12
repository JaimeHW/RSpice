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
    b: f64, d: f64, H: f64, I: f64, X: f64, bL: f64,
    fM: f64, fQ: f64, g2: f64, gs: f64, ks: f64, kw: f64,
    ky: f64, kD: f64, kG: f64, kL: f64, kT: f64, kW: f64,
    kZ: f64, l3: f64, lE: f64, lF: f64, lH: f64, lK: bool,
    lL: f64, n7: f64, p5: f64, q3: f64, qs: f64, qv: f64,
    qy: f64, qZ: f64, sh: f64, sR: f64, sS: f64, sX: f64,
    sY: f64, th: f64, tj: f64, tm: bool, tn: f64, tw: f64,
    u2: f64, u4: f64, u6: f64, ub: bool, uc: f64, uj: f64,
    uk: f64, um: f64, ur: bool, ut: f64, vj: f64, vl: f64,
    vn: f64, vs: bool, vt: f64, vU: f64, w7: f64, wk: f64,
    wx: f64, wE: f64, wF: f64, wI: f64, wK: f64, wP: bool,
    wQ: f64, wW: f64, x0: f64, x3: f64, xb: f64, xc: f64,
    xd: f64, xf: f64, xh: f64, xl: f64, xm: f64, xo: f64,
    xr: f64, xt: f64, xu: bool, xz: bool, xA: f64, yc: f64,
    ye: f64, yg: f64, yh: f64, yk: f64, ym: f64, yr: bool,
    ys: f64, yx: f64, yA: f64, yC: f64, yK: f64, yL: f64,
    yM: f64, yO: f64, yT: f64, yU: f64, yW: f64, yY: f64,
    z0: f64, z1: bool, z6: bool, z7: f64, Ad: f64, Au: f64,
    AQ: f64, C0: f64, Cc: f64, Cp: bool, Cq: bool, Cr: f64,
    Cu: bool, Cv: f64, Cz: f64, CA: f64, CC: f64, CG: f64,
    CI: f64, CN: bool, CO: f64, D3: bool, EM: bool, EN: f64,
    EP: f64, ER: f64, ET: f64, EV: f64, EW: bool, EY: bool,
    F6: f64, F9: bool, Fa: f64, Fb: f64, Fh: bool, Fj: f64,
    Fk: f64, Fo: f64, Fq: f64, Ft: f64, Fv: f64, FA: bool,
    FB: f64, L5: f64, Mk: f64, Mn: f64, Mq: f64, Mt: f64,
    Mx: f64, MB: f64, MJ: f64, MP: f64, MY: f64, N0: f64,
    NE: f64, NF: f64, NG: f64, NH: f64, PB: f64, PC: f64,
    PD: f64, Ug: f64, Uh: f64, Ui: f64, WE: f64, WF: f64,
    WG: f64, Xl: f64, Xm: f64, Xn: f64, Xu: f64, Xv: f64,
    Xw: f64, XD: f64, XE: f64, XF: f64, Yb: f64, Yc: f64,
    a15: f64, a16: f64, a17: f64, a2z: f64, a2A: f64, a2B: f64,
    a2C: f64, a2F: f64, a2I: f64, a2L: f64, a2O: f64, a2P: f64,
    a2Q: f64, a2R: f64, a2T: f64, a2X: f64, a30: f64, a3y: f64,
    a3z: f64, a4v: f64, a4w: f64, a6F: f64, a6G: f64, a6H: f64,
    a7A: f64, a7B: f64, a7C: f64, a7P: f64, a7Q: f64, a7R: f64,
    a8c: f64, a8d: f64, a8e: f64, a8f: f64, a8g: f64, a8x: f64,
    a8y: f64, a8z: f64, a8A: f64, a8B: f64, afZ: f64, ag0: f64,
    ag1: f64, ag2: f64, agf: f64, agg: f64, agh: f64, agi: f64,
    agj: f64, agk: f64, agl: f64, agm: f64, ain: f64, aio: f64,
    aip: f64, aiq: f64, air: f64, ais: f64, ait: f64, aiu: f64,
    anp: f64, anq: f64, anr: f64, ans: f64, aRM: f64, aRN: f64,
    aRO: f64, aRP: f64, aRQ: f64, aRR: f64, aRW: f64, aRX: f64,
    aSa: f64, aSb: f64, aSc: f64, aSd: f64, aSe: f64, aSf: f64,
    aSs: f64, aSt: f64, aSu: f64, aSv: f64, aSw: f64, aSx: f64,
    aSC: f64, aSD: f64, aSI: f64, aSJ: f64, aTs: f64, aTt: f64,
    aTu: f64, aTv: f64, aTw: f64, aTx: f64, aTy: f64, aTz: f64,
    aU5: f64, aU6: f64, aU7: f64, aU8: f64, aUl: f64, aUm: f64,
    aUn: f64, aUo: f64, aUp: f64, aUq: f64, aUr: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values<const REACTIVE: bool>(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
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
        let b=1.0;let d=0.0;let H=0.001;let I=2.0;let X=0.1;let bL=3.0;let fM=1e-6;let fQ=0.5;let g2=4.0;let gs=6.0;let kp=ctx.node_voltage(n[5]);let kq=ctx.node_voltage(n[6]);let ks=(sf[0]*(kp-kq));let kt=ctx.node_voltage(n[7]);let kv=(sf[0]*(kp-kt));let kw=ctx.node_voltage(n[3]);let ky=(sf[0]*(kp-kw));let kz=ctx.node_voltage(n[4]);let kB=(sf[0]*(kz-kw));let kD=(sf[0]*(kz-kp));let kF=(sf[0]*(kq-kt));let kG=ctx.node_voltage(n[2]);let kJ=ctx.node_voltage(n[1]);let kL=(sf[0]*(kJ-kz));let kQ=(sf[0]*(kJ-ctx.node_voltage(n[0])));let kR=ctx.node_voltage(n[9]);let kT=(sf[0]*(kR-kq));let kW=(sf[0]*(ctx.node_voltage(n[8])-kR));let kZ=(((kv+kD)-kF)-kT);let l3=((kZ+(kL+(-kQ)))-kW);let l4=(kQ+l3);let l5=(sf[377]*kv);let l8=(if (l5<sf[198]){b}else{d});let l9=(l5).exp();let lb=(!((l8)!=0.0));let ld=(if lb{sf[199]}else{d});let li=(sf[377]*ky);let lj=(li/sf[584]);let ll=(if (lj<sf[198]){b}else{d});let lm=(lj).exp();let lo=(!((ll)!=0.0));let lp=(if lo{sf[199]}else{ld});let lt=(if lo{(lp*(b+(lj-sf[198])))}else{(if ((ll)!=0.0){lm}else{d})});let lu=(sf[377]*kZ);let lw=(if (lu<sf[198]){b}else{d});let lx=(lu).exp();let lz=(!((lw)!=0.0));let lA=(if lz{sf[199]}else{lp});let lE=(if lz{(lA*(b+(lu-sf[198])))}else{(if ((lw)!=0.0){lx}else{d})});let lF=(sf[377]*kD);let lH=(if (lF<sf[198]){b}else{d});let lK=(!((lH)!=0.0));let lL=(if lK{sf[199]}else{lA});let lQ=(sf[377]*l4);let lS=(if (lQ<sf[198]){b}else{d});let lT=(lQ).exp();let lV=(!((lS)!=0.0));let lW=(if lV{sf[199]}else{lL});let m0=(if lV{(lW*(b+(lQ-sf[198])))}else{(if ((lS)!=0.0){lT}else{d})});let m2=(sf[377]*(l4-sf[465]));let m4=(if (m2<sf[198]){b}else{d});let m5=(m2).exp();let m7=(!((m4)!=0.0));let m8=(if m7{sf[199]}else{lW});let me=(sf[377]*(kZ-sf[465]));let mg=(if (me<sf[198]){b}else{d});let mh=(me).exp();let mj=(!((mg)!=0.0));let mk=(if mj{sf[199]}else{m8});let mq=(sf[377]*(kv-sf[465]));let ms=(if (mq<sf[198]){b}else{d});let mt=(mq).exp();let mv=(!((ms)!=0.0));let mw=(if mv{sf[199]}else{mk});let mA=(if mv{(mw*(b+(mq-sf[198])))}else{(if ((ms)!=0.0){mt}else{d})});let mC=(sf[377]*(ks-sf[465]));let mE=(if (mC<sf[198]){b}else{d});let mF=(mC).exp();let mH=(!((mE)!=0.0));let mI=(if mH{sf[199]}else{mw});let mM=(if mH{(mI*(b+(mC-sf[198])))}else{(if ((mE)!=0.0){mF}else{d})});let mP=((b+(g2*mA))).sqrt();let mS=((b+(g2*mM))).sqrt();let mT=(I*mM);let mU=(b+mS);let mV=(mT/mU);let mY=(if (mV<sf[200]){b}else{d});let mZ=(if ((mY)!=0.0){sf[200]}else{mV});let n1=(b+mP);let n2=(n1/mU);let n5=(sf[376]*((mP-mS)-(n2).ln()));let n7=((kF+n5)/sf[560]);let n9=(if (n7>d){b}else{d});let na=100.0;let nc=(if (ks<na){b}else{d});let nd=(((n9)!=0.0)&&((nc)!=0.0));let ng=(((n9)!=0.0)&&(!((nc)!=0.0)));let ni=(b+(ks-na));let no=(sf[560]*(fQ*n7));let nq=(b+(sf[377]*no));let nv=(if ((n9)!=0.0){((sf[465]+(sf[791]*(nq).ln()))-(if ng{(na+(ni).ln())}else{(if nd{ks}else{d})}))}else{d});let ny=(if ((n9)!=0.0){sf[792]}else{d});let nA=(if ((n9)!=0.0){(ny*ny)}else{fM});let nE=(if (nv<d){b}else{d});let nF=(((n9)!=0.0)&&((nE)!=0.0));let nG=(fQ*nA);let nI=((nA+(if ((n9)!=0.0){(nv*nv)}else{sf[612]}))).sqrt();let nJ=(nI-nv);let nN=(((n9)!=0.0)&&(!((nE)!=0.0)));let nQ=(if nN{(fQ*(nv+nI))}else{(if nF{(nG/nJ)}else{d})});let nU=(nQ+sf[203]);let nV=(nQ*nU);let nY=(sf[202]*(nQ+sf[793]));let o0=(if ((n9)!=0.0){(nV/nY)}else{d});let o2=(if ((n9)!=0.0){(n7/o0)}else{d});let o6=(if ((n9)!=0.0){((o2-b)/sf[204])}else{sf[591]});let o8=(if (o2<b){b}else{d});let o9=(((n9)!=0.0)&&((o8)!=0.0));let oa=(o6).exp();let ob=(b+oa);let oh=(((n9)!=0.0)&&(!((o8)!=0.0)));let oj=((-o6)).exp();let ok=(b+oj);let ox=(if ((n9)!=0.0){((if oh{(o2+(sf[204]*(ok).ln()))}else{(if o9{(b+(sf[204]*(ob).ln()))}else{d})})/sf[210])}else{d});let oz=(if ((n9)!=0.0){(nQ/sf[203])}else{d});let oA=(g2*ox);let oB=(oz*oA);let oC=(b+oz);let oF=((b+(oB*oC))).sqrt();let oG=(b+oF);let oH=(I*ox);let oI=(oC*oH);let oK=(if ((n9)!=0.0){(oG/oI)}else{d});let oM=(mZ*oK);let oN=((b-oK)+oM);let oO=(b+oM);let oQ=(if ((n9)!=0.0){(oN/oO)}else{d});let oT=(if ((n9)!=0.0){(sf[377]*(no*oQ))}else{d});let oW=(b+(mZ+oT));let oZ=(if ((n9)!=0.0){((I*oT)+(mZ*oW))}else{d});let p2=(if ((n9)!=0.0){(fQ*(oT-b))}else{d});
        let p5=(if ((n9)!=0.0){(oZ+(p2*p2))}else{d});let p7=(if (oT>=b){b}else{d});let p8=(((n9)!=0.0)&&((p7)!=0.0));let p9=(p5).sqrt();let pd=(((n9)!=0.0)&&(!((p7)!=0.0)));let pe=(p9-p2);let pg_=(if pd{(oZ/pe)}else{(if p8{(p2+p9)}else{d})});let pk=(((n9)!=0.0)&&(((if (pg_<sf[211]){b}else{d}))!=0.0));let pl=(if pk{sf[211]}else{pg_});let pm=(b+pl);let pv=(if ((n9)!=0.0){(sf[212]*(n7-sf[201]))}else{d});let pC=(((if ((n9)!=0.0){(n7*sf[797])}else{d})+(pv*pv))).sqrt();let pM=(((n9)!=0.0)&&sb[20]);let pN=(I*n7);let pO=(n7+o0);let pT=(n7*sf[201]);let pU=(n7+sf[201]);let pZ=(!((n9)!=0.0));let q0=(I*mA);let q3=(if pZ{(if lb{(ld*(b+(l5-sf[198])))}else{(if ((l8)!=0.0){l9}else{d})})}else{(if ((n9)!=0.0){((pl*pm)*sf[795])}else{d})});let qf=(if (((kF).abs()<sf[799])||((n5).abs()<(sf[800]*(mP+mS)))){b}else{d});let qg=(pZ&&((qf)!=0.0));let qh=(mZ+(if pZ{(q0/n1)}else{pl}));let qj=(if qg{(fQ*qh)}else{d});let qk=(b+qj);let qo=(pZ&&(!((qf)!=0.0)));let qq=((kv+n5)-ks);let qs=(if qo{(n5/qq)}else{(if qg{(qj/qk)}else{oQ})});let qu=(if pZ{sf[798]}else{(if pM{(sf[503]*(X+(pN/pO)))}else{(if (((n9)!=0.0)&&((sf[214])!=0.0)){sf[798]}else{d})})});let qv=(if pZ{n7}else{(if ((n9)!=0.0){(pT/pU)}else{d})});let qy=(if pZ{(b-(qv/sf[201]))}else{(if ((n9)!=0.0){(sf[201]/pU)}else{d})});let qF=((ky-sf[801])/sf[802]);let qH=(if (ky<sf[801]){b}else{d});let qI=(qF).exp();let qJ=(b+qI);let qO=(!((qH)!=0.0));let qQ=((-qF)).exp();let qR=(b+qQ);let qV=(if qO{(sf[801]-(sf[802]*(qR).ln()))}else{(if ((qH)!=0.0){(ky-(sf[802]*(qJ).ln()))}else{d})});let qX=(b-(sf[524]*qV));let qZ=f64::powf(qX,sf[218]);let r5=((sf[803]*(b-qZ))+(bL*(ky-qV)));let ri=(if sb[26]{kv}else{(if sb[24]{(ks+(if pZ{kF}else{(if ((n9)!=0.0){(pv+pC)}else{d})}))}else{(if ((sf[220])!=0.0){ks}else{d})})});let rq=(ri-sf[809]);let rr=(rq/qu);let rt=(if (ri<sf[809]){b}else{d});let ru=(rr).exp();let rv=(b+ru);let rw=(rv).ln();let rA=(!((rt)!=0.0));let rC=((-rr)).exp();let rD=(b+rC);let rE=(rD).ln();let rH=(if rA{(sf[809]-(qu*rE))}else{(if ((rt)!=0.0){(ri-(qu*rw))}else{d})});let rJ=f64::powf(qy,sf[223]);let rN=(b-(rH/sf[503]));let rO=f64::powf(rN,sf[224]);let rS=(sf[806]*rJ);let rT=(ri-rH);let rY=((sf[805]*((sf[810]*(b-(rJ*rO)))+(rS*rT)))+(sf[537]*ks));let s1=(lt*sf[812]);let s3=((b+s1)).sqrt();let s4=(b+s3);let s5=(s1/s4);let s7=f64::powf(q3,sf[813]);let s8=(sf[812]*s7);let sa=((b+s8)).sqrt();let sb_=(b+sa);let sc=(s8/sb_);let sg=(b+(r5/sf[746]));let sh=(rY/sf[744]);let si=(sg+sh);let st=((if sb[28]{(sf[377]*(sf[775]*sg))}else{d})).exp();let su=((if sb[28]{(sf[377]*(sf[775]*((-rY)/sf[744])))}else{d})).exp();let sA=(if sb[28]{((st-su)/sf[816])}else{(if ((sf[225])!=0.0){si}else{d})});let sB=0.010000000000000002;let sC=(sA*sA);let sE=(if (sA<d){b}else{d});let sF=0.005000000000000001;let sH=((sB+sC)).sqrt();let sI=(sH-sA);let sL=(!((sE)!=0.0));let sO=(if sL{(fQ*(sA+sH))}else{(if ((sE)!=0.0){(sF/sI)}else{d})});let sR=(b+(fQ*(s5+sc)));let sS=(sO*sR);let sV=(s7*sf[817]);let sW=(sf[629]*lt);let sX=(sW-sV);let sY=(sX/sS);let sZ=0.0001;let t0=(ky/sZ);let t1=(ky<d);let t2=(if t1{b}else{d});let t3=(t0).exp();let t4=(b+t3);let t8=(!((t2)!=0.0));let ta=((-t0)).exp();let tb=(b+ta);let tf=(if t8{(ky+(sZ*(tb).ln()))}else{(if ((t2)!=0.0){(sZ*(t4).ln())}else{d})});let th=(tf/sf[227]);let tj=(if (th<sf[198]){b}else{d});let tm=(!((tj)!=0.0));let tn=(if tm{sf[199]}else{mI});let tw=((ky-sf[228])/H);let tS=(li/sf[143]);let tU=(if (tS<sf[198]){b}else{d});let tV=(tS).exp();let tX=(!((tU)!=0.0));let tY=(if tX{sf[199]}else{tn});let u2=(if tX{(tY*(b+(tS-sf[198])))}else{(if ((tU)!=0.0){tV}else{tf})});let u4=(sf[377]*(ky-sf[523]));let u6=(if (u4<sf[198]){b}else{d});let ub=(((sf[149])!=0.0)&&(!((u6)!=0.0)));let uc=(if ub{sf[199]}else{tY});let uj=((sY/sf[629])-1000.0);let uk=40.0;let um=(if (uj<uk){b}else{d});let ur=(((sf[149])!=0.0)&&(!((um)!=0.0)));let ut=(if ur{2.3538526683702e17}else{uc});let v8=(sf[377]*kB);let v9=(v8/sf[147]);let vb=(if (v9<sf[198]){b}else{d});let vc=(v9).exp();let ve=(!((vb)!=0.0));let vf=(if ve{sf[199]}else{ut});let vj=(if ve{(vf*(b+(v9-sf[198])))}else{(if ((vb)!=0.0){vc}else{u2})});let vl=(sf[377]*(kB-sf[523]));
        let vn=(if (vl<sf[198]){b}else{d});let vs=(((sf[149])!=0.0)&&(!((vn)!=0.0)));let vt=(if vs{sf[199]}else{vf});let vK=(li/sf[130]);let vM=(if (vK<sf[198]){b}else{d});let vN=(vK).exp();let vP=(!((vM)!=0.0));let vQ=(if vP{sf[199]}else{vt});let vU=(if vP{(vQ*(b+(vK-sf[198])))}else{(if ((vM)!=0.0){vN}else{vj})});let vX=(v8/sf[165]);let vZ=(if (vX<sf[198]){b}else{d});let w0=(vX).exp();let w2=(!((vZ)!=0.0));let w3=(if w2{sf[199]}else{vQ});let w7=(if w2{(w3*(b+(vX-sf[198])))}else{(if ((vZ)!=0.0){w0}else{vU})});let wa=(lu/sf[136]);let wc=(if (wa<sf[198]){b}else{d});let wd=(wa).exp();let wf=(!((wc)!=0.0));let wg=(if wf{sf[199]}else{w3});let wk=(if wf{(wg*(b+(wa-sf[198])))}else{(if ((wc)!=0.0){wd}else{w7})});let wn=(v8/sf[169]);let wp=(if (wn<sf[198]){b}else{d});let wq=(wn).exp();let ws=(!((wp)!=0.0));let wt=(if ws{sf[199]}else{wg});let wx=(if ws{(wt*(b+(wn-sf[198])))}else{(if ((wp)!=0.0){wq}else{wk})});let wE=(if (t1&&sb[36]){b}else{d});let wF=(I*qZ);let wI=(sf[711]*(b-(sf[20]/wF)));let wK=(if (wI<sf[198]){b}else{d});let wP=(((wE)!=0.0)&&(!((wK)!=0.0)));let wQ=(if wP{sf[199]}else{wt});let wW=(if ((wE)!=0.0){(sf[524]*ky)}else{sf[742]});let wY=1e-30;let x0=(((wW*wW)+wY)).sqrt();let x3=f64::powf(x0,sf[233]);let xb=(gs*wW);let xc=(wW*xb);let xd=(wW+sf[236]);let xf=((sf[18]*(sf[235]-((bL*wW)*sf[236])))-(xc*xd));let xh=0.16666666666666666;let xl=(sf[711]*(sf[20]*ky));let xm=(sf[401]*(if ((wE)!=0.0){((x3*xf)*xh)}else{d}));let xo=(if ((wE)!=0.0){(xl/xm)}else{wW});let xp=-0.001;let xr=(if (xo<xp){b}else{d});let xt=(if (xo<sf[198]){b}else{d});let xu=(((wE)!=0.0)&&((xr)!=0.0));let xz=(xu&&(!((xt)!=0.0)));let xA=(if xz{sf[199]}else{wQ});let yc=(if (sb[39]&&(ks<d)){b}else{d});let yd=(sf[525]*ks);let ye=(b-yd);let yg=(if ((yc)!=0.0){f64::powf(ye,sf[224])}else{d});let yh=(I*yg);let yk=(sf[731]*(b-(sf[52]/yh)));let ym=(if (yk<sf[198]){b}else{d});let yr=(((yc)!=0.0)&&(!((ym)!=0.0)));let ys=(if yr{sf[199]}else{xA});let yx=(if ((yc)!=0.0){yd}else{sf[722]});let yA=((wY+(yx*yx))).sqrt();let yC=f64::powf(yA,sf[237]);let yK=(gs*yx);let yL=(yx*yK);let yM=(yx+sf[240]);let yO=((sf[50]*(sf[239]-((bL*yx)*sf[240])))-(yL*yM));let yT=(sf[731]*(sf[52]*ks));let yU=(sf[422]*(if ((yc)!=0.0){(xh*(yC*yO))}else{d}));let yW=(if ((yc)!=0.0){(yT/yU)}else{yx});let yY=(if (yW<xp){b}else{d});let z0=(if (yW<sf[198]){b}else{d});let z1=(((yc)!=0.0)&&((yY)!=0.0));let z6=(z1&&(!((z0)!=0.0)));let z7=(if z6{sf[199]}else{ys});let zC=(lE*sf[812]);let zD=(g2*(if mj{(mk*(b+(me-sf[198])))}else{(if ((mg)!=0.0){mh}else{d})}));let zE=(zC-sf[812]);let zG=((b+zC)).sqrt();let zH=(b+zG);let zK=((b+zD)).sqrt();let zL=(b+zK);let A7=(sf[825]*(m0-b));let Aa=((b+(m0*sf[824]))).sqrt();let Ab=(b+Aa);let Ad=(if ((sf[242])!=0.0){(A7/Ab)}else{d});let Aq=(if sb[44]{(l4-sf[833])}else{d});let Au=(if sb[44]{(Aq*Aq)}else{sC});let Aw=(if (Aq<d){b}else{d});let Ax=(sb[44]&&((Aw)!=0.0));let AA=((sf[245]+Au)).sqrt();let AB=(AA-Aq);let AF=(sb[44]&&(!((Aw)!=0.0)));let AI=(if AF{(fQ*(Aq+AA))}else{(if Ax{(sf[246]/AB)}else{d})});let AL=(AI+(sf[828]+(sf[553]*Ad)));let AQ=(if sb[46]{b}else{(if sb[44]{(AI/AL)}else{b})});let BR=(if (si<d){b}else{d});let BT=((sB+(si*si))).sqrt();let BU=(BT-si);let BX=(!((BR)!=0.0));let C0=(if BX{(fQ*(si+BT))}else{(if ((BR)!=0.0){(sF/BU)}else{d})});let Cc=(if (sY>d){b}else{d});let Ci=(if (ks<sf[268]){b}else{d});let Cl=((-sY)/sf[269]);let Cn=(if (Cl<sf[198]){b}else{d});let Cp=(((Ci)!=0.0)&&(((Cc)!=0.0)&&((sf[267])!=0.0)));let Cq=(((Cn)!=0.0)&&Cp);let Cr=(Cl).exp();let Cu=(Cp&&(!((Cn)!=0.0)));let Cv=(if Cu{sf[199]}else{z7});let Cz=(if Cu{(Cv*(b+(Cl-sf[198])))}else{(if Cq{Cr}else{d})});let CA=(sf[268]-ks);let CC=(if Cp{(Cz*CA)}else{d});let CG=(sf[834]*f64::powf(CC,sf[270]));let CI=(if (CG<sf[198]){b}else{d});let CN=(Cp&&(!((CI)!=0.0)));let CO=(if CN{sf[199]}else{Cv});let D3=(((Cc)!=0.0)&&sb[51]);let EM=(((Ci)!=0.0)&&(((sf[285])!=0.0)&&(D3&&sb[55])));let EN=f64::powf(CA,sf[270]);let EP=(sY+sf[286]);let ER=(b-(sY/EP));let ET=f64::powf(ER,sf[287]);let EV=(if EM{(EN*ET)}else{d});let EW=(((sf[279])!=0.0)&&EM);let EY=(sb[53]&&EM);let F2=(if EY{((sY-sf[288])/sf[286])}else{d});
        let F6=(if EY{((F2-b)/sf[289])}else{tw});let F8=(if (F2<b){b}else{d});let F9=(EY&&((F8)!=0.0));let Fa=(F6).exp();let Fb=(b+Fa);let Fh=(EY&&(!((F8)!=0.0)));let Fj=((-F6)).exp();let Fk=(b+Fj);let Fo=(if Fh{(F2+(sf[289]*(Fk).ln()))}else{(if F9{(b+(sf[289]*(Fb).ln()))}else{d})});let Fq=f64::powf(Fo,sf[290]);let Ft=(sf[834]*(if EY{(EV*Fq)}else{(if EW{EV}else{d})}));let Fv=(if (Ft<sf[198]){b}else{d});let FA=(EM&&(!((Fv)!=0.0)));let FB=(if FA{sf[199]}else{CO});let GB=((kB-sf[801])/sf[802]);let GD=(if (kB<sf[801]){b}else{d});let GE=(GB).exp();let GF=(b+GE);let GK=(!((GD)!=0.0));let GM=((-GB)).exp();let GN=(b+GM);let GR=(if GK{(sf[801]-(sf[802]*(GN).ln()))}else{(if ((GD)!=0.0){(kB-(sf[802]*(GF).ln()))}else{d})});let GU=(b-(sf[524]*GR));let H7=(s5*sf[842]);let H8=(C0*H7);let H9=(sc*sf[842]);let Ha=(C0*H9);let Hc=((kZ-sf[809])/sf[798]);let He=(if (kZ<sf[809]){b}else{d});let Hf=(Hc).exp();let Hg=(b+Hf);let Hl=(!((He)!=0.0));let Hn=((-Hc)).exp();let Ho=(b+Hn);let Hs=(if Hl{(sf[809]-(sf[798]*(Ho).ln()))}else{(if ((He)!=0.0){(kZ-(sf[798]*(Hg).ln()))}else{d})});let Hu=(b-(Hs/sf[503]));let HJ=((l4-sf[809])/sf[798]);let HL=(if (l4<sf[809]){b}else{d});let HM=(HJ).exp();let HN=(b+HM);let HS=(!((HL)!=0.0));let HU=((-HJ)).exp();let HV=(b+HU);let HZ=(if HS{(sf[809]-(sf[798]*(HV).ln()))}else{(if ((HL)!=0.0){(l4-(sf[798]*(HN).ln()))}else{d})});let I1=(b-(HZ/sf[503]));let Il=(ky/sf[847]);let In=(if (Il<sf[198]){b}else{d});let Io=(Il).exp();let Iq=(!((In)!=0.0));let Ir=(if Iq{sf[199]}else{FB});let Iw=(sf[846]*(if Iq{(Ir*(b+(Il-sf[198])))}else{(if ((In)!=0.0){Io}else{wx})}));let IB=(qs*sf[851]);let IC=(I+qh);let IR=(sf[377]*((kZ-sf[484])/sf[301]));let IT=(if (IR<sf[198]){b}else{d});let IV=(((IT)!=0.0)&&sb[60]);let IW=(IR).exp();let IZ=(sb[60]&&(!((IT)!=0.0)));let J0=(if IZ{sf[199]}else{Ir});let J6=(lE*sf[853]);let J9=((b+(g2*(if IZ{(J0*(b+(IR-sf[198])))}else{(if IV{IW}else{d})})))).sqrt();let Ja=(b+J9);let Jc=(if sb[60]{(J6/Ja)}else{(if ((sf[300])!=0.0){((sf[852]*(((zE/zH)*sf[841])+((zD/zL)*sf[850])))/sf[759])}else{d})});let Jl=(if sb[64]{(m0*sf[812])}else{d});let Jm=(Jl-sf[812]);let Jo=((b+Jl)).sqrt();let Jp=(b+Jo);let Jt=(if sb[64]{(g2*(if m7{(m8*(b+(m2-sf[198])))}else{(if ((m4)!=0.0){m5}else{d})}))}else{d});let Jv=((b+Jt)).sqrt();let Jw=(b+Jv);let JI=(sf[377]*(l4-sf[484]));let JK=(if (JI<sf[198]){b}else{d});let JM=(((JK)!=0.0)&&sb[65]);let JN=(JI).exp();let JQ=(sb[65]&&(!((JK)!=0.0)));let JR=(if JQ{sf[199]}else{J0});let JX=(m0*sf[855]);let K0=((b+(g2*(if JQ{(JR*(b+(JI-sf[198])))}else{(if JM{JN}else{d})})))).sqrt();let K1=(b+K0);let K3=(if sb[65]{(JX/K1)}else{(if sb[64]{((sf[854]*((sf[841]*(if sb[64]{(Jm/Jp)}else{d}))+(sf[850]*(if sb[64]{(Jt/Jw)}else{d}))))/sf[759])}else{d})});let Kc=(if ((sf[305])!=0.0){(f64::powf(qX,sf[306])-bL)}else{d});let Kd=(if ((sf[305])!=0.0){qF}else{d});let Kf=(if (Kd<d){b}else{d});let Kg=(((sf[305])!=0.0)&&((Kf)!=0.0));let Kh=(Kd).exp();let Ki=(b+Kh);let Km=(((sf[305])!=0.0)&&(!((Kf)!=0.0)));let Ko=((-Kd)).exp();let Kp=(b+Ko);let Kr=(if Km{(Ko/Kp)}else{(if Kg{(b/Ki)}else{d})});let Ky=((sf[377]*s1)/sf[584]);let Kz=(fQ/s3);let KB=(if ((sf[305])!=0.0){(Ky*Kz)}else{d});let KC=(C0*sf[842]);let KH=(kD*0.2);let KJ=((if ((sf[305])!=0.0){(Iw/sf[847])}else{d})+((if ((sf[305])!=0.0){(sf[838]*(if ((sf[305])!=0.0){(bL+(Kc*Kr))}else{d}))}else{d})+(if ((sf[305])!=0.0){(KB*KC)}else{d})));let KS=(if ((sf[305])!=0.0){(H8+(Iw*sf[307]))}else{d});let L1=(if sb[67]{H8}else{(if ((sf[305])!=0.0){(KS*sf[310])}else{d})});let L2=(if sb[67]{Ha}else{(if ((sf[305])!=0.0){(Ha+(KS*sf[309]))}else{d})});let L4=(sV+sW);let L5=(L4/sS);let Lf=(if (L5>d){b}else{d});let Lg=(L1+L2);let Lj=(!((Lf)!=0.0));let Lk=(sf[755]*C0);let Lm=(if Lj{(sS*Lk)}else{(if ((Lf)!=0.0){(Lg/L5)}else{d})});let LB=(if sb[75]{d}else{(if sb[73]{(Lm*sf[316])}else{(if ((sf[314])!=0.0){(sf[309]*Lm)}else{d})})});
        let Mj=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (sf[0]*((if sb[67]{Iw}else{(if ((sf[305])!=0.0){(Iw*sf[308])}else{d})})+((r5*sf[838])+L1)))) };let Mk=(sf[15]*Mj);let Mm=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (sf[0]*(sf[839]*((sf[803]*(b-f64::powf(GU,sf[218])))+(bL*(kB-GR)))))) };let Mn=(sf[15]*Mm);let Mp=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (sf[0]*((IB*IC)+((rY*sf[840])+L2)))) };let Mq=(sf[15]*Mp);let Ms=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (sf[0]*(if ((sf[305])!=0.0){(KH*KJ)}else{d}))) };let Mt=(sf[15]*Ms);let Mw=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, ((sf[0]*(kJ-kG))*sf[319])) };let Mx=(sf[15]*Mw);let MA=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (kQ*sf[320])) };let MB=(sf[15]*MA);let MI=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (sf[0]*((sf[6]*(sf[296]*(sf[536]*((sf[805]*((sf[810]*(b-f64::powf(I1,sf[224])))+(sf[806]*(l4-HZ))))+(sf[537]*l4)))))+(if ((sf[302])!=0.0){(AQ*K3)}else{d})))) };let MJ=(sf[15]*MI);let MO=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, (sf[0]*((sf[7]*((sf[536]*((sf[805]*((sf[810]*(b-f64::powf(Hu,sf[224])))+(sf[806]*(kZ-Hs))))+(sf[537]*kZ)))*sf[296]))+(if ((sf[302])!=0.0){(sf[7]*Jc)}else{Jc})))) };let MP=(sf[15]*MO);let MY=ctx.node_voltage(n[10]);let MZ=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, MY) };let N0=(LB*MZ);let No=(if lo{(lp*sf[858])}else{(if ((ll)!=0.0){(lm*sf[858])}else{d})});let Np=(if lo{(lp*sf[859])}else{(if ((ll)!=0.0){(lm*sf[859])}else{d})});let NE=(if lz{(lA*sf[856])}else{(if ((lw)!=0.0){(lx*sf[856])}else{d})});let NF=(if lz{(lA*sf[860])}else{(if ((lw)!=0.0){(lx*sf[860])}else{d})});let NG=(if lz{(lA*sf[861])}else{(if ((lw)!=0.0){(lx*sf[861])}else{d})});let NH=(if lz{(lA*sf[857])}else{(if ((lw)!=0.0){(lx*sf[857])}else{d})});let O3=(if lV{(lW*sf[860])}else{(if ((lS)!=0.0){(lT*sf[860])}else{d})});let O4=(if lV{(lW*sf[862])}else{(if ((lS)!=0.0){(lT*sf[862])}else{d})});let O5=(if lV{(lW*sf[861])}else{(if ((lS)!=0.0){(lT*sf[861])}else{d})});let O6=(if lV{(lW*sf[857])}else{(if ((lS)!=0.0){(lT*sf[857])}else{d})});
        let OJ=(if mv{(mw*sf[856])}else{(if ((ms)!=0.0){(mt*sf[856])}else{d})});let OK=(if mv{(mw*sf[857])}else{(if ((ms)!=0.0){(mt*sf[857])}else{d})});let OR=(if mH{(mI*sf[856])}else{(if ((mE)!=0.0){(mF*sf[856])}else{d})});let OS=(if mH{(mI*sf[857])}else{(if ((mE)!=0.0){(mF*sf[857])}else{d})});let OV=(I*mP);let OW=((g2*OJ)/OV);let OX=((g2*OK)/OV);let P0=(I*mS);let P1=((g2*OR)/P0);let P2=((g2*OS)/P0);let P8=(mU*mU);let Pe=(if ((mY)!=0.0){d}else{(((mU*(I*OR))-(mT*P1))/P8)});let Pf=(if ((mY)!=0.0){d}else{(((mU*(I*OS))-(mT*P2))/P8)});let Pw=(sf[376]*((OW-P1)-((((mU*OW)-(n1*P1))/P8)/n2)));let Px=(sf[376]*((-P2)-(((-(n1*P2))/P8)/n2)));let Py=(sf[376]*(OX-((OX/mU)/n2)));let PA=(sf[321]+Py);let PB=(Pw/sf[560]);let PC=((sf[0]+Px)/sf[560]);let PD=(PA/sf[560]);let PN=(sf[560]*(fQ*PB));let PO=(sf[560]*(fQ*PC));let PP=(sf[560]*(fQ*PD));let Q1=(if ((n9)!=0.0){((sf[791]*((sf[377]*PN)/nq))-(if ng{(sf[0]/ni)}else{(if nd{sf[0]}else{d})}))}else{d});let Q2=(if ((n9)!=0.0){((sf[791]*((sf[377]*PO)/nq))-(if ng{(sf[321]/ni)}else{(if nd{sf[321]}else{d})}))}else{d});let Q3=(if ((n9)!=0.0){(sf[791]*((sf[377]*PP)/nq))}else{d});let Q4=(nv*Q1);let Q6=(nv*Q2);let Q8=(nv*Q3);let Qd=(I*nI);let Qe=((if ((n9)!=0.0){(Q4+Q4)}else{d})/Qd);let Qf=((if ((n9)!=0.0){(Q6+Q6)}else{d})/Qd);let Qg=((if ((n9)!=0.0){(Q8+Q8)}else{d})/Qd);let Qm=(nJ*nJ);let QD=(if nN{(fQ*(Q1+Qe))}else{(if nF{((-(nG*(Qe-Q1)))/Qm)}else{d})});let QE=(if nN{(fQ*(Q2+Qf))}else{(if nF{((-(nG*(Qf-Q2)))/Qm)}else{d})});let QF=(if nN{(fQ*(Q3+Qg))}else{(if nF{((-(nG*(Qg-Q3)))/Qm)}else{d})});let QV=(nY*nY);let R5=(if ((n9)!=0.0){(((nY*((nU*QD)+(nQ*QD)))-(nV*(sf[202]*QD)))/QV)}else{d});let R6=(if ((n9)!=0.0){(((nY*((nU*QE)+(nQ*QE)))-(nV*(sf[202]*QE)))/QV)}else{d});let R7=(if ((n9)!=0.0){(((nY*((nU*QF)+(nQ*QF)))-(nV*(sf[202]*QF)))/QV)}else{d});let Rb=(o0*o0);let Rl=(if ((n9)!=0.0){(((o0*PB)-(n7*R5))/Rb)}else{d});let Rm=(if ((n9)!=0.0){(((o0*PC)-(n7*R6))/Rb)}else{d});let Rn=(if ((n9)!=0.0){(((o0*PD)-(n7*R7))/Rb)}else{d});let Rr=(if ((n9)!=0.0){(Rl/sf[204])}else{d});let Rs=(if ((n9)!=0.0){(Rm/sf[204])}else{d});let Rt=(if ((n9)!=0.0){(Rn/sf[204])}else{d});let S1=(if ((n9)!=0.0){((if oh{(Rl+(sf[204]*((oj*(-Rr))/ok)))}else{(if o9{(sf[204]*((oa*Rr)/ob))}else{d})})/sf[210])}else{d});let S2=(if ((n9)!=0.0){((if oh{(Rm+(sf[204]*((oj*(-Rs))/ok)))}else{(if o9{(sf[204]*((oa*Rs)/ob))}else{d})})/sf[210])}else{d});let S3=(if ((n9)!=0.0){((if oh{(Rn+(sf[204]*((oj*(-Rt))/ok)))}else{(if o9{(sf[204]*((oa*Rt)/ob))}else{d})})/sf[210])}else{d});let S7=(if ((n9)!=0.0){(QD/sf[203])}else{d});let S8=(if ((n9)!=0.0){(QE/sf[203])}else{d});let S9=(if ((n9)!=0.0){(QF/sf[203])}else{d});let Sv=(I*oF);let SO=(oI*oI);let SY=(if ((n9)!=0.0){(((oI*(((oC*((oA*S7)+(oz*(g2*S1))))+(oB*S7))/Sv))-(oG*((oH*S7)+(oC*(I*S1)))))/SO)}else{d});let SZ=(if ((n9)!=0.0){(((oI*(((oC*((oA*S8)+(oz*(g2*S2))))+(oB*S8))/Sv))-(oG*((oH*S8)+(oC*(I*S2)))))/SO)}else{d});let T0=(if ((n9)!=0.0){(((oI*(((oC*((oA*S9)+(oz*(g2*S3))))+(oB*S9))/Sv))-(oG*((oH*S9)+(oC*(I*S3)))))/SO)}else{d});let T6=((oK*Pe)+(mZ*SY));let T9=((oK*Pf)+(mZ*SZ));let Ta=(mZ*T0);let Th=(oO*oO);let Tr=(if ((n9)!=0.0){(((oO*((-SY)+T6))-(oN*T6))/Th)}else{d});let Ts=(if ((n9)!=0.0){(((oO*((-SZ)+T9))-(oN*T9))/Th)}else{d});let Tt=(if ((n9)!=0.0){(((oO*((-T0)+Ta))-(oN*Ta))/Th)}else{d});let TG=(if ((n9)!=0.0){(sf[377]*((oQ*PN)+(no*Tr)))}else{d});let TH=(if ((n9)!=0.0){(sf[377]*((oQ*PO)+(no*Ts)))}else{d});let TI=(if ((n9)!=0.0){(sf[377]*((oQ*PP)+(no*Tt)))}else{d});let TY=(if ((n9)!=0.0){((I*TG)+((oW*Pe)+(mZ*(Pe+TG))))}else{d});let TZ=(if ((n9)!=0.0){((I*TH)+((oW*Pf)+(mZ*(Pf+TH))))}else{d});let U0=(if ((n9)!=0.0){((I*TI)+(mZ*TI))}else{d});let U4=(if ((n9)!=0.0){(fQ*TG)}else{d});let U5=(if ((n9)!=0.0){(fQ*TH)}else{d});let U6=(if ((n9)!=0.0){(fQ*TI)}else{d});let U7=(p2*U4);let U9=(p2*U5);let Ub=(p2*U6);let Ug=(if ((n9)!=0.0){(TY+(U7+U7))}else{d});let Uh=(if ((n9)!=0.0){(TZ+(U9+U9))}else{d});let Ui=(if ((n9)!=0.0){(U0+(Ub+Ub))}else{d});let Uj=(I*p9);let Uk=(Ug/Uj);let Ul=(Uh/Uj);let Um=(Ui/Uj);let Uz=(pe*pe);let UM=(if pk{d}else{(if pd{(((pe*TY)-(oZ*(Uk-U4)))/Uz)}else{(if p8{(U4+Uk)}else{d})})});
        let UN=(if pk{d}else{(if pd{(((pe*TZ)-(oZ*(Ul-U5)))/Uz)}else{(if p8{(U5+Ul)}else{d})})});let UO=(if pk{d}else{(if pd{(((pe*U0)-(oZ*(Um-U6)))/Uz)}else{(if p8{(U6+Um)}else{d})})});let V7=(if ((n9)!=0.0){(sf[212]*PB)}else{d});let V8=(if ((n9)!=0.0){(sf[212]*PC)}else{d});let V9=(if ((n9)!=0.0){(sf[212]*PD)}else{d});let Vg=(pv*V7);let Vi=(pv*V8);let Vk=(pv*V9);let Vp=(I*pC);let VI=(pO*pO);let VY=(sf[201]*PB);let VZ=(sf[201]*PC);let W0=(sf[201]*PD);let W4=(pU*pU);let Wv=(n1*n1);let WD=(if pZ{(((n1*(I*OK))-(q0*OX))/Wv)}else{UO});let WE=(if pZ{(if lb{(ld*sf[856])}else{(if ((l8)!=0.0){(l9*sf[856])}else{d})})}else{(if ((n9)!=0.0){(sf[795]*((pm*UM)+(pl*UM)))}else{d})});let WF=(if pZ{d}else{(if ((n9)!=0.0){(sf[795]*((pm*UN)+(pl*UN)))}else{d})});let WG=(if pZ{(if lb{(ld*sf[857])}else{(if ((l8)!=0.0){(l9*sf[857])}else{d})})}else{(if ((n9)!=0.0){(sf[795]*((pm*UO)+(pl*UO)))}else{d})});let WH=(Pe+(if pZ{(((n1*(I*OJ))-(q0*OW))/Wv)}else{UM}));let WI=(Pf+(if pZ{d}else{UN}));let WM=(if qg{(fQ*WH)}else{d});let WN=(if qg{(fQ*WI)}else{d});let WO=(if qg{(fQ*WD)}else{d});let WS=(qk*qk);let Xb=(qq*qq);let Xl=(if qo{(((qq*Pw)-(n5*((sf[0]+Pw)-sf[0])))/Xb)}else{(if qg{(((qk*WM)-(qj*WM))/WS)}else{Tr})});let Xm=(if qo{(((qq*Px)-(n5*(Px-sf[321])))/Xb)}else{(if qg{(((qk*WN)-(qj*WN))/WS)}else{Ts})});let Xn=(if qo{(((qq*Py)-(n5*PA))/Xb)}else{(if qg{(((qk*WO)-(qj*WO))/WS)}else{Tt})});let Xr=(if pZ{d}else{(if pM{(sf[503]*(((pO*(I*PB))-(pN*(PB+R5)))/VI))}else{d})});let Xs=(if pZ{d}else{(if pM{(sf[503]*(((pO*(I*PC))-(pN*(PC+R6)))/VI))}else{d})});let Xt=(if pZ{d}else{(if pM{(sf[503]*(((pO*(I*PD))-(pN*(PD+R7)))/VI))}else{d})});let Xu=(if pZ{PB}else{(if ((n9)!=0.0){(((pU*VY)-(pT*PB))/W4)}else{d})});let Xv=(if pZ{PC}else{(if ((n9)!=0.0){(((pU*VZ)-(pT*PC))/W4)}else{d})});let Xw=(if pZ{PD}else{(if ((n9)!=0.0){(((pU*W0)-(pT*PD))/W4)}else{d})});let XD=(if pZ{(-(Xu/sf[201]))}else{(if ((n9)!=0.0){((-VY)/W4)}else{d})});let XE=(if pZ{(-(Xv/sf[201]))}else{(if ((n9)!=0.0){((-VZ)/W4)}else{d})});let XF=(if pZ{(-(Xw/sf[201]))}else{(if ((n9)!=0.0){((-W0)/W4)}else{d})});let Y2=(if qO{(-(sf[802]*((qQ*sf[865])/qR)))}else{(if ((qH)!=0.0){(sf[321]-(sf[802]*((qI*sf[863])/qJ)))}else{d})});let Y3=(if qO{(-(sf[802]*((qQ*sf[866])/qR)))}else{(if ((qH)!=0.0){(sf[0]-(sf[802]*((qI*sf[864])/qJ)))}else{d})});let Y6=(-(sf[524]*Y2));let Y7=(-(sf[524]*Y3));let Ya=(sf[218]*f64::powf(qX,sf[325]));let Yb=(Y6*Ya);let Yc=(Y7*Ya);let Yl=((sf[803]*(-Yb))+(bL*(sf[321]-Y2)));let Ym=((sf[803]*(-Yc))+(bL*(sf[0]-Y3)));let Yu=(if sb[26]{sf[0]}else{(if sb[24]{(sf[0]+(if pZ{d}else{(if ((n9)!=0.0){(V7+(((if ((n9)!=0.0){(sf[797]*PB)}else{d})+(Vg+Vg))/Vp))}else{d})}))}else{sf[326]})});let Yv=(if sb[26]{d}else{(if sb[24]{(sf[321]+(if pZ{sf[0]}else{(if ((n9)!=0.0){(V8+(((if ((n9)!=0.0){(sf[797]*PC)}else{d})+(Vi+Vi))/Vp))}else{d})}))}else{sf[327]})});let Yw=(if sb[26]{sf[321]}else{(if sb[24]{(if pZ{sf[321]}else{(if ((n9)!=0.0){(V9+(((if ((n9)!=0.0){(sf[797]*PD)}else{d})+(Vk+Vk))/Vp))}else{d})})}else{d})});let YA=(qu*qu);let YB=(((qu*Yu)-(rq*Xr))/YA);let YF=(((qu*Yv)-(rq*Xs))/YA);let YJ=(((qu*Yw)-(rq*Xt))/YA);let Zq=(if rA{(-((rE*Xr)+(qu*((rC*(-YB))/rD))))}else{(if ((rt)!=0.0){(Yu-((rw*Xr)+(qu*((ru*YB)/rv))))}else{d})});let Zr=(if rA{(-((rE*Xs)+(qu*((rC*(-YF))/rD))))}else{(if ((rt)!=0.0){(Yv-((rw*Xs)+(qu*((ru*YF)/rv))))}else{d})});let Zs=(if rA{(-((rE*Xt)+(qu*((rC*(-YJ))/rD))))}else{(if ((rt)!=0.0){(Yw-((rw*Xt)+(qu*((ru*YJ)/rv))))}else{d})});let Zv=(sf[223]*f64::powf(qy,sf[328]));let Zw=(XD*Zv);let Zx=(XE*Zv);let Zy=(XF*Zv);let ZH=(sf[224]*f64::powf(rN,sf[329]));let a0k=(sf[805]*((sf[810]*(-((rO*Zy)+(rJ*((-(Zs/sf[503]))*ZH)))))+((rT*(sf[806]*Zy))+(rS*(Yw-Zs)))));let a0n=((sf[805]*((sf[810]*(-((rO*Zw)+(rJ*((-(Zq/sf[503]))*ZH)))))+((rT*(sf[806]*Zw))+(rS*(Yu-Zq)))))+sf[867]);let a0o=((sf[805]*((sf[810]*(-((rO*Zx)+(rJ*((-(Zr/sf[503]))*ZH)))))+((rT*(sf[806]*Zx))+(rS*(Yv-Zr)))))+sf[868]);let a0p=(sf[812]*No);let a0q=(sf[812]*Np);let a0r=(I*s3);let a0s=(a0p/a0r);let a0t=(a0q/a0r);let a0x=(s4*s4);let a0y=(((s4*a0p)-(s1*a0s))/a0x);let a0C=(((s4*a0q)-(s1*a0t))/a0x);let a0F=(sf[813]*f64::powf(q3,sf[869]));
        let a0G=(WE*a0F);let a0H=(WF*a0F);let a0I=(WG*a0F);let a0J=(sf[812]*a0G);let a0K=(sf[812]*a0H);let a0L=(sf[812]*a0I);let a0M=(I*sa);let a0T=(sb_*sb_);let a0U=(((sb_*a0J)-(s8*(a0J/a0M)))/a0T);let a0Y=(((sb_*a0K)-(s8*(a0K/a0M)))/a0T);let a12=(((sb_*a0L)-(s8*(a0L/a0M)))/a0T);let a13=(Yl/sf[746]);let a14=(Ym/sf[746]);let a15=(a0n/sf[744]);let a16=(a0o/sf[744]);let a17=(a0k/sf[744]);let a18=(a14+a15);let a1K=(if sb[28]{((st*(if sb[28]{(sf[377]*(sf[775]*a13))}else{d}))/sf[816])}else{(if ((sf[225])!=0.0){a13}else{d})});let a1L=(if sb[28]{(((st*(if sb[28]{(sf[377]*(sf[775]*a14))}else{d}))-(su*(if sb[28]{(sf[377]*(sf[775]*((-a0n)/sf[744])))}else{d})))/sf[816])}else{(if ((sf[225])!=0.0){a18}else{d})});let a1M=(if sb[28]{((-(su*(if sb[28]{(sf[377]*(sf[775]*((-a0o)/sf[744])))}else{d})))/sf[816])}else{(if ((sf[225])!=0.0){a16}else{d})});let a1N=(if sb[28]{((-(su*(if sb[28]{(sf[377]*(sf[775]*((-a0k)/sf[744])))}else{d})))/sf[816])}else{(if ((sf[225])!=0.0){a17}else{d})});let a1O=(sA*a1K);let a1P=(a1O+a1O);let a1Q=(sA*a1L);let a1R=(a1Q+a1Q);let a1S=(sA*a1M);let a1T=(a1S+a1S);let a1U=(sA*a1N);let a1V=(a1U+a1U);let a1W=(I*sH);let a1X=(a1P/a1W);let a1Y=(a1R/a1W);let a1Z=(a1T/a1W);let a20=(a1V/a1W);let a27=(sI*sI);let a2z=(fQ*a0y);let a2A=(fQ*(a0C+a0U));let a2B=(fQ*a0Y);let a2C=(fQ*a12);let a2F=((sR*(if sL{(fQ*(a1K+a1X))}else{(if ((sE)!=0.0){((-(sF*(a1X-a1K)))/a27)}else{d})}))+(sO*a2z));let a2I=((sR*(if sL{(fQ*(a1L+a1Y))}else{(if ((sE)!=0.0){((-(sF*(a1Y-a1L)))/a27)}else{d})}))+(sO*a2A));let a2L=((sR*(if sL{(fQ*(a1M+a1Z))}else{(if ((sE)!=0.0){((-(sF*(a1Z-a1M)))/a27)}else{d})}))+(sO*a2B));let a2O=((sR*(if sL{(fQ*(a1N+a20))}else{(if ((sE)!=0.0){((-(sF*(a20-a1N)))/a27)}else{d})}))+(sO*a2C));let a2P=(sf[817]*a0G);let a2Q=(sf[817]*a0H);let a2R=(sf[817]*a0I);let a2T=(sf[629]*Np);let a2X=(sS*(sf[629]*No));let a30=(sS*sS);let a3y=(if t8{(sf[321]+(sZ*((ta*sf[332])/tb)))}else{(if ((t2)!=0.0){(sZ*((t3*sf[330])/t4))}else{d})});let a3z=(if t8{(sf[0]+(sZ*((ta*sf[333])/tb)))}else{(if ((t2)!=0.0){(sZ*((t3*sf[331])/t4))}else{d})});let a4v=(if tX{(tY*sf[870])}else{(if ((tU)!=0.0){(tV*sf[870])}else{a3y})});let a4w=(if tX{(tY*sf[871])}else{(if ((tU)!=0.0){(tV*sf[871])}else{a3z})});let a6F=(if ve{(vf*sf[872])}else{(if ((vb)!=0.0){(vc*sf[872])}else{a4v})});let a6G=(if ve{(vf*sf[873])}else{(if ((vb)!=0.0){(vc*sf[873])}else{d})});let a6H=(if ve{d}else{(if ((vb)!=0.0){d}else{a4w})});let a7A=(if vP{(vQ*sf[874])}else{(if ((vM)!=0.0){(vN*sf[874])}else{a6F})});let a7B=(if vP{d}else{(if ((vM)!=0.0){d}else{a6G})});let a7C=(if vP{(vQ*sf[875])}else{(if ((vM)!=0.0){(vN*sf[875])}else{a6H})});let a7P=(if w2{(w3*sf[876])}else{(if ((vZ)!=0.0){(w0*sf[876])}else{a7A})});let a7Q=(if w2{(w3*sf[877])}else{(if ((vZ)!=0.0){(w0*sf[877])}else{a7B})});let a7R=(if w2{d}else{(if ((vZ)!=0.0){d}else{a7C})});let a8c=(if wf{d}else{(if ((wc)!=0.0){d}else{a7P})});let a8d=(if wf{(wg*sf[878])}else{(if ((wc)!=0.0){(wd*sf[878])}else{a7Q})});let a8e=(if wf{(wg*sf[879])}else{(if ((wc)!=0.0){(wd*sf[879])}else{a7R})});let a8f=(if wf{(wg*sf[880])}else{(if ((wc)!=0.0){(wd*sf[880])}else{d})});let a8g=(if wf{(wg*sf[881])}else{(if ((wc)!=0.0){(wd*sf[881])}else{d})});let a8x=(if ws{(wt*sf[882])}else{(if ((wp)!=0.0){(wq*sf[882])}else{a8c})});let a8y=(if ws{(wt*sf[883])}else{(if ((wp)!=0.0){(wq*sf[883])}else{a8d})});let a8z=(if ws{d}else{(if ((wp)!=0.0){d}else{a8e})});let a8A=(if ws{d}else{(if ((wp)!=0.0){d}else{a8f})});let a8B=(if ws{d}else{(if ((wp)!=0.0){d}else{a8g})});let ae3=(sf[812]*NE);let ae4=(sf[812]*NF);let ae5=(sf[812]*NG);let ae6=(sf[812]*NH);let ae7=(g2*(if mj{(mk*sf[856])}else{(if ((mg)!=0.0){(mh*sf[856])}else{d})}));let ae8=(g2*(if mj{(mk*sf[860])}else{(if ((mg)!=0.0){(mh*sf[860])}else{d})}));let ae9=(g2*(if mj{(mk*sf[861])}else{(if ((mg)!=0.0){(mh*sf[861])}else{d})}));let aea=(g2*(if mj{(mk*sf[857])}else{(if ((mg)!=0.0){(mh*sf[857])}else{d})}));let aeb=(I*zG);let aej=(zH*zH);let aex=(I*zK);let aeF=(zL*zL);let afD=(I*Aa);let afL=(Ab*Ab);let afZ=(if ((sf[242])!=0.0){(((Ab*(sf[825]*O3))-(A7*((sf[824]*O3)/afD)))/afL)}else{d});
        let ag0=(if ((sf[242])!=0.0){(((Ab*(sf[825]*O4))-(A7*((sf[824]*O4)/afD)))/afL)}else{d});let ag1=(if ((sf[242])!=0.0){(((Ab*(sf[825]*O5))-(A7*((sf[824]*O5)/afD)))/afL)}else{d});let ag2=(if ((sf[242])!=0.0){(((Ab*(sf[825]*O6))-(A7*((sf[824]*O6)/afD)))/afL)}else{d});let ag7=(Aq*sf[346]);let ag8=(ag7+ag7);let ag9=(Aq*sf[347]);let agb=(Aq*sf[348]);let agc=(agb+agb);let agd=(Aq*sf[349]);let agf=(if sb[44]{ag8}else{d});let agg=(if sb[44]{(ag9+ag9)}else{d});let agh=(if sb[44]{d}else{a1P});let agi=(if sb[44]{ag8}else{a1R});let agj=(if sb[44]{agc}else{a1T});let agk=(if sb[44]{agc}else{a1V});let agl=(if sb[44]{(agd+agd)}else{d});let agm=(if sb[44]{agc}else{d});let agn=(I*AA);let ago=(agf/agn);let agp=(agg/agn);let agq=(agh/agn);let agr=(agi/agn);let ags=(agj/agn);let agt=(agk/agn);let agu=(agl/agn);let agv=(agm/agn);let agF=(AB*AB);let ahp=(if AF{(fQ*(sf[346]+ago))}else{(if Ax{((-(sf[246]*(ago-sf[346])))/agF)}else{d})});let ahq=(if AF{(fQ*(sf[347]+agp))}else{(if Ax{((-(sf[246]*(agp-sf[347])))/agF)}else{d})});let ahr=(if AF{(fQ*agq)}else{(if Ax{((-(sf[246]*agq))/agF)}else{d})});let ahs=(if AF{(fQ*(sf[346]+agr))}else{(if Ax{((-(sf[246]*(agr-sf[346])))/agF)}else{d})});let aht=(if AF{(fQ*(sf[348]+ags))}else{(if Ax{((-(sf[246]*(ags-sf[348])))/agF)}else{d})});let ahu=(if AF{(fQ*(sf[348]+agt))}else{(if Ax{((-(sf[246]*(agt-sf[348])))/agF)}else{d})});let ahv=(if AF{(fQ*(sf[349]+agu))}else{(if Ax{((-(sf[246]*(agu-sf[349])))/agF)}else{d})});let ahw=(if AF{(fQ*(sf[348]+agv))}else{(if Ax{((-(sf[246]*(agv-sf[348])))/agF)}else{d})});let ahx=(sf[553]*afZ);let ahz=(sf[553]*ag1);let ahL=(AL*AL);let ain=(if sb[46]{d}else{(if sb[44]{(((AL*ahp)-(AI*(ahp+ahx)))/ahL)}else{d})});let aio=(if sb[46]{d}else{(if sb[44]{(((AL*ahq)-(AI*(ahq+(sf[553]*ag0))))/ahL)}else{d})});let aip=(if sb[46]{d}else{(if sb[44]{(((AL*ahr)-(AI*ahr))/ahL)}else{d})});let aiq=(if sb[46]{d}else{(if sb[44]{(((AL*ahs)-(AI*(ahs+ahx)))/ahL)}else{d})});let air=(if sb[46]{d}else{(if sb[44]{(((AL*aht)-(AI*(aht+ahz)))/ahL)}else{d})});let ais=(if sb[46]{d}else{(if sb[44]{(((AL*ahu)-(AI*(ahu+ahz)))/ahL)}else{d})});let ait=(if sb[46]{d}else{(if sb[44]{(((AL*ahv)-(AI*(ahv+(sf[553]*ag2))))/ahL)}else{d})});let aiu=(if sb[46]{d}else{(if sb[44]{(((AL*ahw)-(AI*(ahw+ahz)))/ahL)}else{d})});let amJ=(si*a13);let amL=(si*a18);let amN=(si*a16);let amP=(si*a17);let amR=(I*BT);let amS=((amJ+amJ)/amR);let amT=((amL+amL)/amR);let amU=((amN+amN)/amR);let amV=((amP+amP)/amR);let an2=(BU*BU);let anp=(if BX{(fQ*(a13+amS))}else{(if ((BR)!=0.0){((-(sF*(amS-a13)))/an2)}else{d})});let anq=(if BX{(fQ*(a18+amT))}else{(if ((BR)!=0.0){((-(sF*(amT-a18)))/an2)}else{d})});let anr=(if BX{(fQ*(a16+amU))}else{(if ((BR)!=0.0){((-(sF*(amU-a16)))/an2)}else{d})});let ans=(if BX{(fQ*(a17+amV))}else{(if ((BR)!=0.0){((-(sF*(amV-a17)))/an2)}else{d})});let aBC=(if GK{(-(sf[802]*((GM*sf[865])/GN)))}else{(if ((GD)!=0.0){(sf[321]-(sf[802]*((GE*sf[863])/GF)))}else{d})});let aBD=(if GK{(-(sf[802]*((GM*sf[866])/GN)))}else{(if ((GD)!=0.0){(sf[0]-(sf[802]*((GE*sf[864])/GF)))}else{d})});let aBJ=(sf[218]*f64::powf(GU,sf[325]));let aC5=((H7*anp)+(C0*(sf[842]*a0y)));let aC8=((H7*anq)+(C0*(sf[842]*a0C)));let aC9=(H7*anr);let aCa=(H7*ans);let aCe=(H9*anp);let aCh=((H9*anq)+(C0*(sf[842]*a0U)));let aCk=((H9*anr)+(C0*(sf[842]*a0Y)));let aCn=((H9*ans)+(C0*(sf[842]*a12)));let aD6=(if Hl{(-(sf[798]*((Hn*sf[900])/Ho)))}else{(if ((He)!=0.0){(sf[0]-(sf[798]*((Hf*sf[896])/Hg)))}else{d})});let aD7=(if Hl{(-(sf[798]*((Hn*sf[901])/Ho)))}else{(if ((He)!=0.0){(sf[322]-(sf[798]*((Hf*sf[897])/Hg)))}else{d})});let aD8=(if Hl{(-(sf[798]*((Hn*sf[902])/Ho)))}else{(if ((He)!=0.0){(sf[323]-(sf[798]*((Hf*sf[898])/Hg)))}else{d})});let aD9=(if Hl{(-(sf[798]*((Hn*sf[903])/Ho)))}else{(if ((He)!=0.0){(sf[321]-(sf[798]*((Hf*sf[899])/Hg)))}else{d})});let aDj=(sf[224]*f64::powf(Hu,sf[329]));let aEG=(if HS{(-(sf[798]*((HU*sf[901])/HV)))}else{(if ((HL)!=0.0){(sf[322]-(sf[798]*((HM*sf[897])/HN)))}else{d})});let aEH=(if HS{(-(sf[798]*((HU*sf[907])/HV)))}else{(if ((HL)!=0.0){(sf[324]-(sf[798]*((HM*sf[906])/HN)))}else{d})});
        let aEI=(if HS{(-(sf[798]*((HU*sf[902])/HV)))}else{(if ((HL)!=0.0){(sf[323]-(sf[798]*((HM*sf[898])/HN)))}else{d})});let aEJ=(if HS{(-(sf[798]*((HU*sf[903])/HV)))}else{(if ((HL)!=0.0){(sf[321]-(sf[798]*((HM*sf[899])/HN)))}else{d})});let aET=(sf[224]*f64::powf(I1,sf[329]));let aFz=(sf[6]*(sf[296]*(sf[536]*(sf[904]+(sf[805]*((sf[810]*(-((-(aEG/sf[503]))*aET)))+(sf[806]*(sf[322]-aEG))))))));let aFB=(sf[6]*(sf[296]*(sf[536]*(sf[905]+(sf[805]*((sf[810]*(-((-(aEI/sf[503]))*aET)))+(sf[806]*(sf[323]-aEI))))))));let aFT=(sf[846]*(if Iq{(Ir*sf[909])}else{(if ((In)!=0.0){(Io*sf[909])}else{a8x})}));let aFU=(sf[846]*(if Iq{d}else{(if ((In)!=0.0){d}else{a8y})}));let aFV=(sf[846]*(if Iq{(Ir*sf[910])}else{(if ((In)!=0.0){(Io*sf[910])}else{a8z})}));let aFW=(sf[846]*(if Iq{d}else{(if ((In)!=0.0){d}else{a8A})}));let aFX=(sf[846]*(if Iq{d}else{(if ((In)!=0.0){d}else{a8B})}));let aH4=(I*J9);let aHc=(Ja*Ja);let aHq=(if sb[60]{(((Ja*(sf[853]*NE))-(J6*((g2*(if IZ{(J0*sf[911])}else{(if IV{(IW*sf[911])}else{d})}))/aH4)))/aHc)}else{(if ((sf[300])!=0.0){((sf[852]*((sf[841]*(((zH*ae3)-(zE*(ae3/aeb)))/aej))+(sf[850]*(((zL*ae7)-(zD*(ae7/aex)))/aeF))))/sf[759])}else{d})});let aHr=(if sb[60]{(((Ja*(sf[853]*NF))-(J6*((g2*(if IZ{(J0*sf[912])}else{(if IV{(IW*sf[912])}else{d})}))/aH4)))/aHc)}else{(if ((sf[300])!=0.0){((sf[852]*((sf[841]*(((zH*ae4)-(zE*(ae4/aeb)))/aej))+(sf[850]*(((zL*ae8)-(zD*(ae8/aex)))/aeF))))/sf[759])}else{d})});let aHs=(if sb[60]{(((Ja*(sf[853]*NG))-(J6*((g2*(if IZ{(J0*sf[913])}else{(if IV{(IW*sf[913])}else{d})}))/aH4)))/aHc)}else{(if ((sf[300])!=0.0){((sf[852]*((sf[841]*(((zH*ae5)-(zE*(ae5/aeb)))/aej))+(sf[850]*(((zL*ae9)-(zD*(ae9/aex)))/aeF))))/sf[759])}else{d})});let aHt=(if sb[60]{(((Ja*(sf[853]*NH))-(J6*((g2*(if IZ{(J0*sf[914])}else{(if IV{(IW*sf[914])}else{d})}))/aH4)))/aHc)}else{(if ((sf[300])!=0.0){((sf[852]*((sf[841]*(((zH*ae6)-(zE*(ae6/aeb)))/aej))+(sf[850]*(((zL*aea)-(zD*(aea/aex)))/aeF))))/sf[759])}else{d})});let aHG=(if sb[64]{(sf[812]*O3)}else{d});let aHH=(if sb[64]{(sf[812]*O4)}else{d});let aHI=(if sb[64]{(sf[812]*O5)}else{d});let aHJ=(if sb[64]{(sf[812]*O6)}else{d});let aHK=(I*Jo);let aHS=(Jp*Jp);let aIe=(if sb[64]{(g2*(if m7{(m8*sf[860])}else{(if ((m4)!=0.0){(m5*sf[860])}else{d})}))}else{d});let aIf=(if sb[64]{(g2*(if m7{(m8*sf[862])}else{(if ((m4)!=0.0){(m5*sf[862])}else{d})}))}else{d});let aIg=(if sb[64]{(g2*(if m7{(m8*sf[861])}else{(if ((m4)!=0.0){(m5*sf[861])}else{d})}))}else{d});let aIh=(if sb[64]{(g2*(if m7{(m8*sf[857])}else{(if ((m4)!=0.0){(m5*sf[857])}else{d})}))}else{d});let aIi=(I*Jv);let aIq=(Jw*Jw);let aJu=(I*K0);let aJC=(K1*K1);let aJV=(AQ*(if sb[65]{(((K1*(sf[855]*O3))-(JX*((g2*(if JQ{(JR*sf[860])}else{(if JM{(JN*sf[860])}else{d})}))/aJu)))/aJC)}else{(if sb[64]{((sf[854]*((sf[841]*(if sb[64]{(((Jp*aHG)-(Jm*(aHG/aHK)))/aHS)}else{d}))+(sf[850]*(if sb[64]{(((Jw*aIe)-(Jt*(aIe/aIi)))/aIq)}else{d}))))/sf[759])}else{d})}));let aK4=(AQ*(if sb[65]{(((K1*(sf[855]*O5))-(JX*((g2*(if JQ{(JR*sf[861])}else{(if JM{(JN*sf[861])}else{d})}))/aJu)))/aJC)}else{(if sb[64]{((sf[854]*((sf[841]*(if sb[64]{(((Jp*aHI)-(Jm*(aHI/aHK)))/aHS)}else{d}))+(sf[850]*(if sb[64]{(((Jw*aIg)-(Jt*(aIg/aIi)))/aIq)}else{d}))))/sf[759])}else{d})}));let aKn=(sf[306]*f64::powf(qX,sf[363]));let aKx=(Ki*Ki);let aKF=(Ko*sf[917]);let aKG=(Ko*sf[918]);let aKK=(Kp*Kp);let aLa=(s3*s3);let aLL=(if ((sf[305])!=0.0){(aFW/sf[847])}else{d});let aMo=(sf[307]*aFW);let aMu=(if ((sf[305])!=0.0){(aC5+(sf[307]*aFT))}else{d});let aMv=(if ((sf[305])!=0.0){(sf[307]*aFU)}else{d});let aMw=(if ((sf[305])!=0.0){(aC8+(sf[307]*aFV))}else{d});let aMx=(if ((sf[305])!=0.0){(aC9+aMo)}else{d});let aMy=(if ((sf[305])!=0.0){(aCa+aMo)}else{d});let aMz=(if ((sf[305])!=0.0){(sf[307]*aFX)}else{d});let aN2=(if sb[67]{aC5}else{(if ((sf[305])!=0.0){(sf[310]*aMu)}else{d})});let aN3=(if sb[67]{d}else{(if ((sf[305])!=0.0){(sf[310]*aMv)}else{d})});let aN4=(if sb[67]{aC8}else{(if ((sf[305])!=0.0){(sf[310]*aMw)}else{d})});let aN5=(if sb[67]{aC9}else{(if ((sf[305])!=0.0){(sf[310]*aMx)}else{d})});let aN6=(if sb[67]{aCa}else{(if ((sf[305])!=0.0){(sf[310]*aMy)}else{d})});
        let aN7=(if sb[67]{d}else{(if ((sf[305])!=0.0){(sf[310]*aMz)}else{d})});let aN8=(if sb[67]{aCe}else{(if ((sf[305])!=0.0){(aCe+(sf[309]*aMu))}else{d})});let aN9=(if sb[67]{d}else{(if ((sf[305])!=0.0){(sf[309]*aMv)}else{d})});let aNa=(if sb[67]{aCh}else{(if ((sf[305])!=0.0){(aCh+(sf[309]*aMw))}else{d})});let aNb=(if sb[67]{aCk}else{(if ((sf[305])!=0.0){(aCk+(sf[309]*aMx))}else{d})});let aNc=(if sb[67]{aCn}else{(if ((sf[305])!=0.0){(aCn+(sf[309]*aMy))}else{d})});let aNd=(if sb[67]{d}else{(if ((sf[305])!=0.0){(sf[309]*aMz)}else{d})});let aNh=(if sb[67]{aFW}else{(if ((sf[305])!=0.0){(sf[308]*aFW)}else{d})});let aNz=(L5*L5);let aOk=(if Lj{((Lk*a2F)+(sS*(sf[755]*anp)))}else{(if ((Lf)!=0.0){(((L5*(aN2+aN8))-(Lg*((a2X-(L4*a2F))/a30)))/aNz)}else{d})});let aOl=(if Lj{d}else{(if ((Lf)!=0.0){((aN3+aN9)/L5)}else{d})});let aOm=(if Lj{((Lk*a2I)+(sS*(sf[755]*anq)))}else{(if ((Lf)!=0.0){(((L5*(aN4+aNa))-(Lg*(((sS*(a2P+a2T))-(L4*a2I))/a30)))/aNz)}else{d})});let aOn=(if Lj{((Lk*a2L)+(sS*(sf[755]*anr)))}else{(if ((Lf)!=0.0){(((L5*(aN5+aNb))-(Lg*(((sS*a2Q)-(L4*a2L))/a30)))/aNz)}else{d})});let aOo=(if Lj{((Lk*a2O)+(sS*(sf[755]*ans)))}else{(if ((Lf)!=0.0){(((L5*(aN6+aNc))-(Lg*(((sS*a2R)-(L4*a2O))/a30)))/aNz)}else{d})});let aOp=(if Lj{d}else{(if ((Lf)!=0.0){((aN7+aNd)/L5)}else{d})});let aRF=(if REACTIVE { 1.0 } else { ddt_scale });let aRM=(sf[15]*((sf[0]*((if sb[67]{aFT}else{(if ((sf[305])!=0.0){(sf[308]*aFT)}else{d})})+((sf[838]*Yl)+aN2)))*aRF));let aRN=(sf[15]*((sf[0]*(aN3+(if sb[67]{aFU}else{(if ((sf[305])!=0.0){(sf[308]*aFU)}else{d})})))*aRF));let aRO=(sf[15]*((sf[0]*((if sb[67]{aFV}else{(if ((sf[305])!=0.0){(sf[308]*aFV)}else{d})})+((sf[838]*Ym)+aN4)))*aRF));let aRP=(sf[15]*((sf[0]*(aN5+aNh))*aRF));let aRQ=(sf[15]*((sf[0]*(aN6+aNh))*aRF));let aRR=(sf[15]*((sf[0]*(aN7+(if sb[67]{aFX}else{(if ((sf[305])!=0.0){(sf[308]*aFX)}else{d})})))*aRF));let aRW=(sf[15]*(aRF*(sf[0]*(sf[839]*((sf[803]*(-((-(sf[524]*aBC))*aBJ)))+(bL*(sf[321]-aBC)))))));let aRX=(sf[15]*(aRF*(sf[0]*(sf[839]*((sf[803]*(-((-(sf[524]*aBD))*aBJ)))+(bL*(sf[0]-aBD)))))));let aSa=(sf[15]*(aRF*(sf[0]*aN8)));let aSb=(sf[15]*(aRF*(sf[0]*aN9)));let aSc=(sf[15]*(aRF*(sf[0]*(((IC*(sf[851]*Xl))+(IB*WH))+((sf[840]*a0n)+aNa)))));let aSd=(sf[15]*(aRF*(sf[0]*(((IC*(sf[851]*Xm))+(IB*WI))+((sf[840]*a0o)+aNb)))));let aSe=(sf[15]*(aRF*(sf[0]*(((IC*(sf[851]*Xn))+(IB*WD))+((sf[840]*a0k)+aNc)))));let aSf=(sf[15]*(aRF*(sf[0]*aNd)));let aSs=(sf[15]*(aRF*(sf[0]*(if ((sf[305])!=0.0){(KH*((if ((sf[305])!=0.0){(aFT/sf[847])}else{d})+((if ((sf[305])!=0.0){(sf[838]*(if ((sf[305])!=0.0){((Kr*(if ((sf[305])!=0.0){(Y6*aKn)}else{d}))+(Kc*(if Km{(((Kp*aKF)-(Ko*aKF))/aKK)}else{(if Kg{((-(Kh*sf[915]))/aKx)}else{d})})))}else{d}))}else{d})+(if ((sf[305])!=0.0){((KC*(if ((sf[305])!=0.0){((Kz*((sf[377]*a0p)/sf[584]))+(Ky*((-(fQ*a0s))/aLa)))}else{d}))+(KB*(sf[842]*anp)))}else{d}))))}else{d}))));let aSt=(sf[15]*(aRF*(sf[0]*(if ((sf[305])!=0.0){((KJ*sf[364])+(KH*(if ((sf[305])!=0.0){(aFU/sf[847])}else{d})))}else{d}))));let aSu=(sf[15]*(aRF*(sf[0]*(if ((sf[305])!=0.0){((KJ*sf[365])+(KH*((if ((sf[305])!=0.0){(aFV/sf[847])}else{d})+((if ((sf[305])!=0.0){(sf[838]*(if ((sf[305])!=0.0){((Kr*(if ((sf[305])!=0.0){(Y7*aKn)}else{d}))+(Kc*(if Km{(((Kp*aKG)-(Ko*aKG))/aKK)}else{(if Kg{((-(Kh*sf[916]))/aKx)}else{d})})))}else{d}))}else{d})+(if ((sf[305])!=0.0){((KC*(if ((sf[305])!=0.0){((Kz*((sf[377]*a0q)/sf[584]))+(Ky*((-(fQ*a0t))/aLa)))}else{d}))+(KB*(sf[842]*anq)))}else{d})))))}else{d}))));let aSv=(sf[15]*(aRF*(sf[0]*(if ((sf[305])!=0.0){(KH*((if ((sf[305])!=0.0){(KB*(sf[842]*anr))}else{d})+aLL))}else{d}))));let aSw=(sf[15]*(aRF*(sf[0]*(if ((sf[305])!=0.0){(KH*((if ((sf[305])!=0.0){(KB*(sf[842]*ans))}else{d})+aLL))}else{d}))));let aSx=(sf[15]*(aRF*(sf[0]*(if ((sf[305])!=0.0){(KH*(if ((sf[305])!=0.0){(aFX/sf[847])}else{d}))}else{d}))));let aSC=(sf[15]*(aRF*sf[368]));let aSD=(sf[15]*(aRF*sf[369]));let aSI=(sf[15]*(aRF*sf[370]));let aSJ=(sf[15]*(aRF*sf[371]));let aTs=(sf[15]*(aRF*(sf[0]*(aFz+(if ((sf[302])!=0.0){((K3*ain)+aJV)}else{d})))));
        let aTt=(sf[15]*(aRF*(sf[0]*((sf[6]*(sf[296]*(sf[536]*((sf[805]*((sf[810]*(-((-(aEH/sf[503]))*aET)))+(sf[806]*(sf[324]-aEH))))+sf[908]))))+(if ((sf[302])!=0.0){((K3*aio)+(AQ*(if sb[65]{(((K1*(sf[855]*O4))-(JX*((g2*(if JQ{(JR*sf[862])}else{(if JM{(JN*sf[862])}else{d})}))/aJu)))/aJC)}else{(if sb[64]{((sf[854]*((sf[841]*(if sb[64]{(((Jp*aHH)-(Jm*(aHH/aHK)))/aHS)}else{d}))+(sf[850]*(if sb[64]{(((Jw*aIf)-(Jt*(aIf/aIi)))/aIq)}else{d}))))/sf[759])}else{d})})))}else{d})))));let aTu=(sf[15]*(aRF*(sf[0]*(if ((sf[302])!=0.0){(K3*aip)}else{d}))));let aTv=(sf[15]*(aRF*(sf[0]*(aFz+(if ((sf[302])!=0.0){(aJV+(K3*aiq))}else{d})))));let aTw=(sf[15]*(aRF*(sf[0]*(aFB+(if ((sf[302])!=0.0){((K3*air)+aK4)}else{d})))));let aTx=(sf[15]*(aRF*(sf[0]*(aFB+(if ((sf[302])!=0.0){(aK4+(K3*ais))}else{d})))));let aTy=(sf[15]*(aRF*(sf[0]*((sf[6]*(sf[296]*(sf[536]*(sf[868]+(sf[805]*((sf[810]*(-((-(aEJ/sf[503]))*aET)))+(sf[806]*(sf[321]-aEJ))))))))+(if ((sf[302])!=0.0){((K3*ait)+(AQ*(if sb[65]{(((K1*(sf[855]*O6))-(JX*((g2*(if JQ{(JR*sf[857])}else{(if JM{(JN*sf[857])}else{d})}))/aJu)))/aJC)}else{(if sb[64]{((sf[854]*((sf[841]*(if sb[64]{(((Jp*aHJ)-(Jm*(aHJ/aHK)))/aHS)}else{d}))+(sf[850]*(if sb[64]{(((Jw*aIh)-(Jt*(aIh/aIi)))/aIq)}else{d}))))/sf[759])}else{d})})))}else{d})))));let aTz=(sf[15]*(aRF*(sf[0]*(aFB+(if ((sf[302])!=0.0){(aK4+(K3*aiu))}else{d})))));let aU5=(sf[15]*(aRF*(sf[0]*((sf[7]*(sf[296]*(sf[536]*(sf[867]+(sf[805]*((sf[810]*(-((-(aD6/sf[503]))*aDj)))+(sf[806]*(sf[0]-aD6))))))))+(if ((sf[302])!=0.0){(sf[7]*aHq)}else{aHq})))));let aU6=(sf[15]*(aRF*(sf[0]*((sf[7]*(sf[296]*(sf[536]*((sf[805]*((sf[810]*(-((-(aD7/sf[503]))*aDj)))+(sf[806]*(sf[322]-aD7))))+sf[904]))))+(if ((sf[302])!=0.0){(sf[7]*aHr)}else{aHr})))));let aU7=(sf[15]*(aRF*(sf[0]*((sf[7]*(sf[296]*(sf[536]*((sf[805]*((sf[810]*(-((-(aD8/sf[503]))*aDj)))+(sf[806]*(sf[323]-aD8))))+sf[905]))))+(if ((sf[302])!=0.0){(sf[7]*aHs)}else{aHs})))));let aU8=(sf[15]*(aRF*(sf[0]*((sf[7]*(sf[296]*(sf[536]*(sf[868]+(sf[805]*((sf[810]*(-((-(aD9/sf[503]))*aDj)))+(sf[806]*(sf[321]-aD9))))))))+(if ((sf[302])!=0.0){(sf[7]*aHt)}else{aHt})))));let aUl=(MZ*(if sb[75]{d}else{(if sb[73]{(sf[316]*aOk)}else{(if ((sf[314])!=0.0){(sf[309]*aOk)}else{d})})}));let aUm=(MZ*(if sb[75]{d}else{(if sb[73]{(sf[316]*aOl)}else{(if ((sf[314])!=0.0){(sf[309]*aOl)}else{d})})}));let aUn=(MZ*(if sb[75]{d}else{(if sb[73]{(sf[316]*aOm)}else{(if ((sf[314])!=0.0){(sf[309]*aOm)}else{d})})}));let aUo=(MZ*(if sb[75]{d}else{(if sb[73]{(sf[316]*aOn)}else{(if ((sf[314])!=0.0){(sf[309]*aOn)}else{d})})}));let aUp=(MZ*(if sb[75]{d}else{(if sb[73]{(sf[316]*aOo)}else{(if ((sf[314])!=0.0){(sf[309]*aOo)}else{d})})}));let aUq=(MZ*(if sb[75]{d}else{(if sb[73]{(sf[316]*aOp)}else{(if ((sf[314])!=0.0){(sf[309]*aOp)}else{d})})}));let aUr=(LB*aRF);

        CommonStampValues {
            b, d, H, I, X, bL, fM, fQ,
            g2, gs, ks, kw, ky, kD, kG, kL,
            kT, kW, kZ, l3, lE, lF, lH, lK,
            lL, n7, p5, q3, qs, qv, qy, qZ,
            sh, sR, sS, sX, sY, th, tj, tm,
            tn, tw, u2, u4, u6, ub, uc, uj,
            uk, um, ur, ut, vj, vl, vn, vs,
            vt, vU, w7, wk, wx, wE, wF, wI,
            wK, wP, wQ, wW, x0, x3, xb, xc,
            xd, xf, xh, xl, xm, xo, xr, xt,
            xu, xz, xA, yc, ye, yg, yh, yk,
            ym, yr, ys, yx, yA, yC, yK, yL,
            yM, yO, yT, yU, yW, yY, z0, z1,
            z6, z7, Ad, Au, AQ, C0, Cc, Cp,
            Cq, Cr, Cu, Cv, Cz, CA, CC, CG,
            CI, CN, CO, D3, EM, EN, EP, ER,
            ET, EV, EW, EY, F6, F9, Fa, Fb,
            Fh, Fj, Fk, Fo, Fq, Ft, Fv, FA,
            FB, L5, Mk, Mn, Mq, Mt, Mx, MB,
            MJ, MP, MY, N0, NE, NF, NG, NH,
            PB, PC, PD, Ug, Uh, Ui, WE, WF,
            WG, Xl, Xm, Xn, Xu, Xv, Xw, XD,
            XE, XF, Yb, Yc, a15, a16, a17, a2z,
            a2A, a2B, a2C, a2F, a2I, a2L, a2O, a2P,
            a2Q, a2R, a2T, a2X, a30, a3y, a3z, a4v,
            a4w, a6F, a6G, a6H, a7A, a7B, a7C, a7P,
            a7Q, a7R, a8c, a8d, a8e, a8f, a8g, a8x,
            a8y, a8z, a8A, a8B, afZ, ag0, ag1, ag2,
            agf, agg, agh, agi, agj, agk, agl, agm,
            ain, aio, aip, aiq, air, ais, ait, aiu,
            anp, anq, anr, ans, aRM, aRN, aRO, aRP,
            aRQ, aRR, aRW, aRX, aSa, aSb, aSc, aSd,
            aSe, aSf, aSs, aSt, aSu, aSv, aSw, aSx,
            aSC, aSD, aSI, aSJ, aTs, aTt, aTu, aTv,
            aTw, aTx, aTy, aTz, aU5, aU6, aU7, aU8,
            aUl, aUm, aUn, aUo, aUp, aUq, aUr,
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let CommonStampValues {
            b, d, H, I, X, bL, fM, fQ,
            g2, gs, ks, kw, ky, kD, kG, kL,
            kT, kW, kZ, l3, lE, lF, lH, lK,
            lL, n7, p5, q3, qs, qv, qy, qZ,
            sh, sR, sS, sX, sY, th, tj, tm,
            tn, tw, u2, u4, u6, ub, uc, uj,
            uk, um, ur, ut, vj, vl, vn, vs,
            vt, vU, w7, wk, wx, wE, wF, wI,
            wK, wP, wQ, wW, x0, x3, xb, xc,
            xd, xf, xh, xl, xm, xo, xr, xt,
            xu, xz, xA, yc, ye, yg, yh, yk,
            ym, yr, ys, yx, yA, yC, yK, yL,
            yM, yO, yT, yU, yW, yY, z0, z1,
            z6, z7, Ad, Au, AQ, C0, Cc, Cp,
            Cq, Cr, Cu, Cv, Cz, CA, CC, CG,
            CI, CN, CO, D3, EM, EN, EP, ER,
            ET, EV, EW, EY, F6, F9, Fa, Fb,
            Fh, Fj, Fk, Fo, Fq, Ft, Fv, FA,
            FB, L5, Mk, Mn, Mq, Mt, Mx, MB,
            MJ, MP, MY, N0, NE, NF, NG, NH,
            PB, PC, PD, Ug, Uh, Ui, WE, WF,
            WG, Xl, Xm, Xn, Xu, Xv, Xw, XD,
            XE, XF, Yb, Yc, a15, a16, a17, a2z,
            a2A, a2B, a2C, a2F, a2I, a2L, a2O, a2P,
            a2Q, a2R, a2T, a2X, a30, a3y, a3z, a4v,
            a4w, a6F, a6G, a6H, a7A, a7B, a7C, a7P,
            a7Q, a7R, a8c, a8d, a8e, a8f, a8g, a8x,
            a8y, a8z, a8A, a8B, afZ, ag0, ag1, ag2,
            agf, agg, agh, agi, agj, agk, agl, agm,
            ain, aio, aip, aiq, air, ais, ait, aiu,
            anp, anq, anr, ans, aRM, aRN, aRO, aRP,
            aRQ, aRR, aRW, aRX, aSa, aSb, aSc, aSd,
            aSe, aSf, aSs, aSt, aSu, aSv, aSw, aSx,
            aSC, aSD, aSI, aSJ, aTs, aTt, aTu, aTv,
            aTw, aTx, aTy, aTz, aU5, aU6, aU7, aU8,
            aUl, aUm, aUn, aUo, aUp, aUq, aUr,
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
        let w=ctx.simparam_or("gmin", d);let lI=(lF).exp();let tk=(th).exp();let tr=(if tm{(tn*(b+(th-sf[198])))}else{(if ((tj)!=0.0){tk}else{d})});let ty=(if (ky<sf[228]){b}else{d});let tz=(tw).exp();let tA=(b+tz);let tF=(!((ty)!=0.0));let tH=((-tw)).exp();let tI=(b+tH);let tM=(if tF{(sf[228]-(H*(tI).ln()))}else{(if ((ty)!=0.0){(ky-(H*(tA).ln()))}else{d})});let tO=(tM*sf[229]);let tP=(sf[228]-tM);let tQ={let pb=tP;pb*pb};let u7=(((sf[149])!=0.0)&&((u6)!=0.0));let u8_=(u4).exp();let ug=(if ub{(uc*(b+(u4-sf[198])))}else{(if u7{u8_}else{th})});let un=(((sf[149])!=0.0)&&((um)!=0.0));let uo=(uj).exp();let ux=(if ur{(ut*(b+(uj-uk)))}else{(if un{uo}else{tr})});let uy=(u2-b);let uz=(sf[657]*uy);let uB=(uy*sf[818]);let uE=((b+(g2*ug))).sqrt();let uF=(b+uE);let uG=(uB/uF);let uH=(b+sh);let uL=(sf[672]*(q3-b));let uM=(ux*uL);let uN=(b+ux);let v3=(sf[230]*((q3+u2)-I));let vo=(((sf[149])!=0.0)&&((vn)!=0.0));let vp=(vl).exp();let vy=(vj-b);let vz=(sf[663]*vy);let vB=(vy*sf[819]);let vE=((b+(g2*(if vs{(vt*(b+(vl-sf[198])))}else{(if vo{vp}else{ug})})))).sqrt();let vF=(b+vE);let wm=(sf[649]*(wk-b));let wL=(((wE)!=0.0)&&((wK)!=0.0));let wM=(wI).exp();let wU=(if wP{(wQ*(b+(wI-sf[198])))}else{(if wL{wM}else{d})});let xv=(((xt)!=0.0)&&xu);let xw=(xo).exp();let xF=(-ky);let xG=(b-(if xz{(xA*(b+(xo-sf[198])))}else{(if xv{xw}else{d})}));let xI=(b+(xG/xo));let xM=(((wE)!=0.0)&&(!((xr)!=0.0)));let xN=(fQ*ky);let xO=(xo*xN);let xP=0.3333333333333333;let xQ=(xo*xP);let xR=0.25;let xT=(b+(xo*xR));let xV=(b+(xQ*xT));let xZ=((if xM{(xO*xV)}else{(if xu{(xF*xI)}else{d})})*sf[820]);let y0=(qZ*xZ);let y5=(!((wE)!=0.0));let yn=(((yc)!=0.0)&&((ym)!=0.0));let yo=(yk).exp();let yw=(if yr{(ys*(b+(yk-sf[198])))}else{(if yn{yo}else{d})});let z2=(((z0)!=0.0)&&z1);let z3=(yW).exp();let zc=(-ks);let zd=(b-(if z6{(z7*(b+(yW-sf[198])))}else{(if z2{z3}else{d})}));let zf=(b+(zd/yW));let zj=(((yc)!=0.0)&&(!((yY)!=0.0)));let zk=(fQ*ks);let zl=(yW*zk);let zm=(xP*yW);let zo=(b+(xR*yW));let zq=(b+(zm*zo));let zu=((if zj{(zl*zq)}else{(if z1{(zc*zf)}else{d})})*sf[821]);let zv=(yg*zu);let zA=(!((yc)!=0.0));let zB=(if zA{d}else{(if ((yc)!=0.0){(sf[53]*(sf[525]*(yw*zv)))}else{d})});let zP=(sf[822]*(lE-b));let zU=((b+(lE*sf[824]))).sqrt();let zV=(b+zU);let zW=(zP/zV);let A3=(if ((sf[242])!=0.0){(sf[7]*zW)}else{zW});let AS=(if ((sf[242])!=0.0){(Ad*AQ)}else{d});let AX=(if ((sf[248])!=0.0){(ks+kD)}else{d});let AZ=(-AX);let B3=(if (AZ<d){b}else{d});let B4=(((sf[248])!=0.0)&&((B3)!=0.0));let B7=((sf[249]+(if ((sf[248])!=0.0){(AX*AX)}else{Au}))).sqrt();let B8=(B7-AZ);let Bc=(((sf[248])!=0.0)&&(!((B3)!=0.0)));let Bf=(if Bc{(fQ*(AZ+B7))}else{(if B4{(sf[250]/B8)}else{d})});let Bw=(if (Bf<sf[258]){b}else{d});let Bx=(((sf[248])!=0.0)&&((Bw)!=0.0));let By=(Bf/sf[256]);let BA=(b-f64::powf(By,sf[251]));let BE=(((sf[248])!=0.0)&&(!((Bw)!=0.0)));let BK=(if sb[48]{b}else{(if BE{(sf[255]+(sf[265]*(Bf-sf[258])))}else{(if Bx{(b/BA)}else{d})})});let C1=(sR*C0);let C2=(sf[545]/C1);let C4=(if (C2<sf[16]){b}else{d});let C6=(bL*(if ((C4)!=0.0){sf[16]}else{C2}));let C9=(kD+(sf[791]*((if lK{(lL*(b+(lF-sf[198])))}else{(if ((lH)!=0.0){lI}else{d})})-b)));let CJ=(Cp&&((CI)!=0.0));let CK=(CG).exp();let CS=(if CN{(CO*(b+(CG-sf[198])))}else{(if CJ{CK}else{d})});let CV=(CC*sf[835]);let D5=((((if (ks<sf[465]){b}else{d}))!=0.0)&&(((sf[272])!=0.0)&&D3));let Db=(if D5{sf[277]}else{d});let Dc=(sf[465]-ks);let De=(if D5{(Dc/qy)}else{p5});let Dh=(((I*De)/Db)).sqrt();let Di=(if D5{Dh}else{d});let Dm=(D5&&((sf[279])!=0.0));let Dp=(D5&&sb[53]);let Ds=(if Dp{(b-(fQ*qs))}else{d});let Dt=(sf[275]*Ds);let Dv=(if Dp{(Ds*Dt)}else{(if Dm{sf[275]}else{d})});let Dw=(Di*Dv);let DA=(((Di*Di)+(Dv*Dv))).sqrt();let DC=(if D5{(Dw/DA)}else{d});let DE=(if D5{(Dc/DC)}else{d});let DF=(fQ*DC);let DG=(Db*DF);let DJ=(if D5{(DE+(qy*DG))}else{d});let DW=(sf[201]*(if Dp{(b+(sf[281]*(b+(I*qs))))}else{d}));let DY=((if Dp{sf[284]}else{d})-(sY/DW));let E1=(if Dp{(DE-(DG*DY))}else{d});let E2=(E1-DJ);let E4=(X*DE);let E5=(DE*E4);let Eb=((if Dp{((E2*E2)+((qv*E5)/sf[201]))}else{De})).sqrt();let Ee=(if Dp{(fQ*((DJ+E1)+Eb))}else{(if Dm{DJ}else{d})});
        let Ef=(Ee-DE);let Eh=(if D5{(Ef/Ee)}else{d});let El=(if ((Eh).abs()>1e-7){b}else{d});let Em=(D5&&((El)!=0.0));let Eo=(if Em{(DF/Eh)}else{d});let Eq=(Ee*sf[836]);let Er=(Eo*Eq);let Et=(sf[837]/Ee);let Eu=(Et).exp();let Ew=(b+(Dv/Eo));let Ey=((Et*Ew)).exp();let Ez=(Eu-Ey);let ED=(D5&&(!((El)!=0.0)));let EE=(sf[4]*Dv);let Fw=(EM&&((Fv)!=0.0));let Fx=(Ft).exp();let FF=(if FA{(FB*(b+(Ft-sf[198])))}else{(if Fw{Fx}else{CS})});let FG=(CA*sf[835]);let FI=(if EM{(FF*FG)}else{(if ED{(Eu*EE)}else{(if Em{(Er*Ez)}else{(if Cp{(CS*CV)}else{d})})})});let FO=(((Cc)!=0.0)&&(((if (FI>d){b}else{d}))!=0.0));let FP=(((sf[292])!=0.0)&&FO);let FQ=(sf[550]+C6);let FR=(sY*FQ);let FY=(if FP{(((sf[376]/FR)+(sf[657]*(sS/sf[629])))+(sf[542]/FQ))}else{d});let FZ=(((sf[285])!=0.0)&&FP);let G2=(if FZ{((FI-FY)/fM)}else{F6});let G4=(if (FI<FY){b}else{d});let G5=(FZ&&((G4)!=0.0));let G6=(G2).exp();let G7=(b+G6);let Gd=(FZ&&(!((G4)!=0.0)));let Gf=((-G2)).exp();let Gg=(b+Gf);let Gk=(if Gd{(FY-(fM*(Gg).ln()))}else{(if G5{(FI-(fM*(G7).ln()))}else{FI})});let Gl=(sY*Gk);let Go=(FP&&sb[57]);let Gp=(FY*Gl);let Gq=(FY+Gk);let Gu=(FO&&sb[58]);let Gv=(if Gu{Gl}else{(if Go{(Gp/Gq)}else{(if FZ{Gl}else{d})})});let Ld=(if sb[69]{d}else{(if ((sf[312])!=0.0){((Gv/L5)).abs()}else{d})});let M5=(sf[15]*(sf[0]*(-(zB*BK))));let a31=((a2X-(sX*a2F))/a30);let a35=(((sS*(a2T-a2P))-(sX*a2I))/a30);let a39=(((sS*(-a2Q))-(sX*a2L))/a30);let a3d=(((sS*(-a2R))-(sX*a2O))/a30);let a3A=(a3y/sf[227]);let a3B=(a3z/sf[227]);let a3I=(if tm{(tn*a3A)}else{(if ((tj)!=0.0){(tk*a3A)}else{d})});let a3J=(if tm{(tn*a3B)}else{(if ((tj)!=0.0){(tk*a3B)}else{d})});let a48=(if tF{(-(H*((tH*sf[336])/tI)))}else{(if ((ty)!=0.0){(sf[321]-(H*((tz*sf[334])/tA)))}else{d})});let a49=(if tF{(-(H*((tH*sf[337])/tI)))}else{(if ((ty)!=0.0){(sf[0]-(H*((tz*sf[335])/tA)))}else{d})});let a4e=(I*tP);let a4D=(if ub{(uc*sf[857])}else{(if u7{(u8_*sf[857])}else{a3A})});let a4E=(if ub{(uc*sf[856])}else{(if u7{(u8_*sf[856])}else{a3B})});let a4F=(a31/sf[629]);let a4G=(a35/sf[629]);let a4H=(a39/sf[629]);let a4I=(a3d/sf[629]);let a4V=(if ur{(ut*a4F)}else{(if un{(uo*a4F)}else{a3I})});let a4W=(if ur{(ut*a4G)}else{(if un{(uo*a4G)}else{a3J})});let a4X=(if ur{(ut*a4H)}else{(if un{(uo*a4H)}else{d})});let a4Y=(if ur{(ut*a4I)}else{(if un{(uo*a4I)}else{d})});let a4Z=(sf[657]*a4v);let a50=(sf[657]*a4w);let a55=(I*uE);let a5b=(uF*uF);let a5F=(uN*uN);let a6S=(sf[663]*a6F);let a6T=(sf[663]*a6G);let a6U=(sf[663]*a6H);let a71=(I*vE);let a78=(vF*vF);let a8L=(wF*wF);let a8S=(sf[711]*(-((-(sf[20]*(I*Yb)))/a8L)));let a8T=(sf[711]*(-((-(sf[20]*(I*Yc)))/a8L)));let a94=(if ((wE)!=0.0){sf[884]}else{d});let a95=(if ((wE)!=0.0){sf[885]}else{d});let a96=(wW*a94);let a98=(wW*a95);let a9a=(I*x0);let a9f=(sf[233]*f64::powf(x0,sf[338]));let a9Z=(xm*xm);let aa5=(if ((wE)!=0.0){(((xm*sf[886])-(xl*(sf[401]*(if ((wE)!=0.0){(xh*((xf*(((a96+a96)/a9a)*a9f))+(x3*((sf[18]*(-(sf[236]*(bL*a94))))-((xd*((xb*a94)+(wW*(gs*a94))))+(xc*a94))))))}else{d}))))/a9Z)}else{a94});let aa6=(if ((wE)!=0.0){(((xm*sf[887])-(xl*(sf[401]*(if ((wE)!=0.0){(xh*((xf*(((a98+a98)/a9a)*a9f))+(x3*((sf[18]*(-(sf[236]*(bL*a95))))-((xd*((xb*a95)+(wW*(gs*a95))))+(xc*a95))))))}else{d}))))/a9Z)}else{a95});let aak=(xo*xo);let abp=(sf[224]*f64::powf(ye,sf[329]));let abs=(if ((yc)!=0.0){(sf[890]*abp)}else{d});let abt=(if ((yc)!=0.0){(sf[891]*abp)}else{d});let aby=(yh*yh);let abF=(sf[731]*(-((-(sf[52]*(I*abs)))/aby)));let abG=(sf[731]*(-((-(sf[52]*(I*abt)))/aby)));let abP=(if ((yc)!=0.0){sf[888]}else{d});let abQ=(if ((yc)!=0.0){sf[889]}else{d});let abR=(yx*abP);let abT=(yx*abQ);let abV=(I*yA);let ac0=(sf[237]*f64::powf(yA,sf[343]));let acK=(yU*yU);let acQ=(if ((yc)!=0.0){(((yU*sf[892])-(yT*(sf[422]*(if ((yc)!=0.0){(xh*((yO*(((abR+abR)/abV)*ac0))+(yC*((sf[50]*(-(sf[240]*(bL*abP))))-((yM*((yK*abP)+(yx*(gs*abP))))+(yL*abP))))))}else{d}))))/acK)}else{abP});let acR=(if ((yc)!=0.0){(((yU*sf[893])-(yT*(sf[422]*(if ((yc)!=0.0){(xh*((yO*(((abT+abT)/abV)*ac0))+(yC*((sf[50]*(-(sf[240]*(bL*abQ))))-((yM*((yK*abQ)+(yx*(gs*abQ))))+(yL*abQ))))))}else{d}))))/acK)}else{abQ});let ad5=(yW*yW);let af1=(I*zU);
        let af9=(zV*zV);let afa=(((zV*(sf[822]*NE))-(zP*((sf[824]*NE)/af1)))/af9);let afe=(((zV*(sf[822]*NF))-(zP*((sf[824]*NF)/af1)))/af9);let afi=(((zV*(sf[822]*NG))-(zP*((sf[824]*NG)/af1)))/af9);let afm=(((zV*(sf[822]*NH))-(zP*((sf[824]*NH)/af1)))/af9);let aiv=(AQ*afZ);let aiE=(AQ*ag1);let aj2=(AX*sf[350]);let aj4=(AX*sf[351]);let aj6=(AX*sf[352]);let ajh=(I*B7);let aji=((if ((sf[248])!=0.0){d}else{agf})/ajh);let ajj=((if ((sf[248])!=0.0){d}else{agg})/ajh);let ajk=((if ((sf[248])!=0.0){d}else{agh})/ajh);let ajl=((if ((sf[248])!=0.0){(aj2+aj2)}else{agf})/ajh);let ajm=((if ((sf[248])!=0.0){(aj4+aj4)}else{agi})/ajh);let ajn=((if ((sf[248])!=0.0){(aj6+aj6)}else{agj})/ajh);let ajo=((if ((sf[248])!=0.0){d}else{agk})/ajh);let ajp=((if ((sf[248])!=0.0){d}else{agl})/ajh);let ajq=((if ((sf[248])!=0.0){d}else{agm})/ajh);let ajw=(B8*B8);let akh=(if Bc{(fQ*aji)}else{(if B4{((-(sf[250]*aji))/ajw)}else{d})});let aki=(if Bc{(fQ*ajj)}else{(if B4{((-(sf[250]*ajj))/ajw)}else{d})});let akj=(if Bc{(fQ*ajk)}else{(if B4{((-(sf[250]*ajk))/ajw)}else{d})});let akk=(if Bc{(fQ*(sf[353]+ajl))}else{(if B4{((-(sf[250]*(ajl-sf[353])))/ajw)}else{d})});let akl=(if Bc{(fQ*(sf[354]+ajm))}else{(if B4{((-(sf[250]*(ajm-sf[354])))/ajw)}else{d})});let akm=(if Bc{(fQ*(sf[355]+ajn))}else{(if B4{((-(sf[250]*(ajn-sf[355])))/ajw)}else{d})});let akn=(if Bc{(fQ*ajo)}else{(if B4{((-(sf[250]*ajo))/ajw)}else{d})});let ako=(if Bc{(fQ*ajp)}else{(if B4{((-(sf[250]*ajp))/ajw)}else{d})});let akp=(if Bc{(fQ*ajq)}else{(if B4{((-(sf[250]*ajq))/ajw)}else{d})});let akA=(sf[251]*f64::powf(By,sf[260]));let akK=(BA*BA);let all=(if sb[48]{d}else{(if BE{(sf[265]*akh)}else{(if Bx{(((akh/sf[256])*akA)/akK)}else{d})})});let alm=(if sb[48]{d}else{(if BE{(sf[265]*aki)}else{(if Bx{(((aki/sf[256])*akA)/akK)}else{d})})});let aln=(if sb[48]{d}else{(if BE{(sf[265]*akj)}else{(if Bx{(((akj/sf[256])*akA)/akK)}else{d})})});let alo=(if sb[48]{d}else{(if BE{(sf[265]*akk)}else{(if Bx{(((akk/sf[256])*akA)/akK)}else{d})})});let alp=(if sb[48]{d}else{(if BE{(sf[265]*akl)}else{(if Bx{(((akl/sf[256])*akA)/akK)}else{d})})});let alq=(if sb[48]{d}else{(if BE{(sf[265]*akm)}else{(if Bx{(((akm/sf[256])*akA)/akK)}else{d})})});let alr=(if sb[48]{d}else{(if BE{(sf[265]*akn)}else{(if Bx{(((akn/sf[256])*akA)/akK)}else{d})})});let als=(if sb[48]{d}else{(if BE{(sf[265]*ako)}else{(if Bx{(((ako/sf[256])*akA)/akK)}else{d})})});let alt=(if sb[48]{d}else{(if BE{(sf[265]*akp)}else{(if Bx{(((akp/sf[256])*akA)/akK)}else{d})})});let alQ=(BK*(if ((sf[242])!=0.0){(sf[7]*afi)}else{afi}));let ama=(BK*(sf[649]*a8f));let amj=(BK*(if ((sf[242])!=0.0){(aiv+(Ad*ain))}else{d}));let anH=(C1*C1);let anW=(bL*(if ((C4)!=0.0){d}else{((-(sf[545]*((C0*a2z)+(sR*anp))))/anH)}));let anX=(bL*(if ((C4)!=0.0){d}else{((-(sf[545]*((C0*a2A)+(sR*anq))))/anH)}));let anY=(bL*(if ((C4)!=0.0){d}else{((-(sf[545]*((C0*a2B)+(sR*anr))))/anH)}));let anZ=(bL*(if ((C4)!=0.0){d}else{((-(sf[545]*((C0*a2C)+(sR*ans))))/anH)}));let ao6=(C6*C6);let aon=((-a31)/sf[269]);let aoo=((-a35)/sf[269]);let aop=((-a39)/sf[269]);let aoq=((-a3d)/sf[269]);let aoP=(if Cp{(CA*(if Cu{(Cv*aon)}else{(if Cq{(Cr*aon)}else{d})}))}else{d});let aoQ=(if Cp{((CA*(if Cu{(Cv*aoo)}else{(if Cq{(Cr*aoo)}else{d})}))+(Cz*sf[321]))}else{d});let aoR=(if Cp{((CA*(if Cu{(Cv*aop)}else{(if Cq{(Cr*aop)}else{d})}))+(sf[0]*Cz))}else{d});let aoS=(if Cp{(CA*(if Cu{(Cv*aoq)}else{(if Cq{(Cr*aoq)}else{d})}))}else{d});let aoV=(sf[270]*f64::powf(CC,sf[356]));let ap0=(sf[834]*(aoP*aoV));let ap1=(sf[834]*(aoQ*aoV));let ap2=(sf[834]*(aoR*aoV));let ap3=(sf[834]*(aoS*aoV));let apg=(if CN{(CO*ap0)}else{(if CJ{(CK*ap0)}else{d})});let aph=(if CN{(CO*ap1)}else{(if CJ{(CK*ap1)}else{d})});let api=(if CN{(CO*ap2)}else{(if CJ{(CK*ap2)}else{d})});let apj=(if CN{(CO*ap3)}else{(if CJ{(CK*ap3)}else{d})});let apH=(qy*qy);let apQ=(if D5{(((qy*sf[321])-(Dc*XD))/apH)}else{Ug});let apR=(if D5{(((sf[0]*qy)-(Dc*XE))/apH)}else{Uh});let apS=(if D5{((-(Dc*XF))/apH)}else{Ui});let apZ=(I*Dh);let aq3=(if D5{(((I*apQ)/Db)/apZ)}else{d});let aq4=(if D5{(((I*apR)/Db)/apZ)}else{d});let aq5=(if D5{(((I*apS)/Db)/apZ)}else{d});
        let aqc=(if Dp{(-(fQ*Xl))}else{d});let aqd=(if Dp{(-(fQ*Xm))}else{d});let aqe=(if Dp{(-(fQ*Xn))}else{d});let aqr=(if Dp{((Dt*aqc)+(Ds*(sf[275]*aqc)))}else{d});let aqs=(if Dp{((Dt*aqd)+(Ds*(sf[275]*aqd)))}else{d});let aqt=(if Dp{((Dt*aqe)+(Ds*(sf[275]*aqe)))}else{d});let aqD=(Di*aq3);let aqF=(Di*aq4);let aqH=(Di*aq5);let aqJ=(Dv*aqr);let aqL=(Dv*aqs);let aqN=(Dv*aqt);let aqS=(I*DA);let aqZ=(DA*DA);let ar9=(if D5{(((DA*((Dv*aq3)+(Di*aqr)))-(Dw*(((aqD+aqD)+(aqJ+aqJ))/aqS)))/aqZ)}else{d});let ara=(if D5{(((DA*((Dv*aq4)+(Di*aqs)))-(Dw*(((aqF+aqF)+(aqL+aqL))/aqS)))/aqZ)}else{d});let arb=(if D5{(((DA*((Dv*aq5)+(Di*aqt)))-(Dw*(((aqH+aqH)+(aqN+aqN))/aqS)))/aqZ)}else{d});let arf=(DC*DC);let aro=(if D5{(((DC*sf[321])-(Dc*ar9))/arf)}else{d});let arp=(if D5{(((sf[0]*DC)-(Dc*ara))/arf)}else{d});let arq=(if D5{((-(Dc*arb))/arf)}else{d});let arr=(fQ*ar9);let ars=(fQ*ara);let art=(fQ*arb);let aru=(Db*arr);let arv=(Db*ars);let arw=(Db*art);let arJ=(if D5{(aro+((DG*XD)+(qy*aru)))}else{d});let arK=(if D5{(arp+((DG*XE)+(qy*arv)))}else{d});let arL=(if D5{(arq+((DG*XF)+(qy*arw)))}else{d});let as5=(DW*DW);let asx=(if Dp{(-(DG*(-(a31/DW))))}else{d});let asy=(if Dp{(aro-((DY*aru)+(DG*(-(((DW*a35)-(sY*(sf[201]*(if Dp{(sf[281]*(I*Xl))}else{d}))))/as5)))))}else{d});let asz=(if Dp{(arp-((DY*arv)+(DG*(-(((DW*a39)-(sY*(sf[201]*(if Dp{(sf[281]*(I*Xm))}else{d}))))/as5)))))}else{d});let asA=(if Dp{(arq-((DY*arw)+(DG*(-(((DW*a3d)-(sY*(sf[201]*(if Dp{(sf[281]*(I*Xn))}else{d}))))/as5)))))}else{d});let asE=(E2*asx);let asG=(E2*(asy-arJ));let asI=(E2*(asz-arK));let asK=(E2*(asA-arL));let atk=(I*Eb);let atx=(if Dp{(fQ*(asx+((if Dp{(asE+asE)}else{d})/atk)))}else{d});let aty=(if Dp{(fQ*((arJ+asy)+((if Dp{((asG+asG)+(((E5*Xu)+(qv*((E4*aro)+(DE*(X*aro)))))/sf[201]))}else{apQ})/atk)))}else{(if Dm{arJ}else{d})});let atz=(if Dp{(fQ*((arK+asz)+((if Dp{((asI+asI)+(((E5*Xv)+(qv*((E4*arp)+(DE*(X*arp)))))/sf[201]))}else{apR})/atk)))}else{(if Dm{arK}else{d})});let atA=(if Dp{(fQ*((arL+asA)+((if Dp{((asK+asK)+(((E5*Xw)+(qv*((E4*arq)+(DE*(X*arq)))))/sf[201]))}else{apS})/atk)))}else{(if Dm{arL}else{d})});let atH=(Ee*Ee);let au1=(Eh*Eh);let auf=(if Em{((-(DF*(if D5{(((Ee*atx)-(Ef*atx))/atH)}else{d})))/au1)}else{d});let aug=(if Em{(((Eh*arr)-(DF*(if D5{(((Ee*(aty-aro))-(Ef*aty))/atH)}else{d})))/au1)}else{d});let auh=(if Em{(((Eh*ars)-(DF*(if D5{(((Ee*(atz-arp))-(Ef*atz))/atH)}else{d})))/au1)}else{d});let aui=(if Em{(((Eh*art)-(DF*(if D5{(((Ee*(atA-arq))-(Ef*atA))/atH)}else{d})))/au1)}else{d});let auB=((-(sf[837]*atx))/atH);let auE=((-(sf[837]*aty))/atH);let auH=((-(sf[837]*atz))/atH);let auK=((-(sf[837]*atA))/atH);let auL=(Eu*auB);let auM=(Eu*auE);let auN=(Eu*auH);let auO=(Eu*auK);let auR=(Eo*Eo);let avX=(sf[270]*f64::powf(CA,sf[356]));let aw3=(EP*EP);let awn=(sf[287]*f64::powf(ER,sf[357]));let awA=(if EM{(EN*((-(((EP*a31)-(sY*a31))/aw3))*awn))}else{d});let awB=(if EM{((ET*(sf[321]*avX))+(EN*((-(((EP*a35)-(sY*a35))/aw3))*awn)))}else{d});let awC=(if EM{((ET*(sf[0]*avX))+(EN*((-(((EP*a39)-(sY*a39))/aw3))*awn)))}else{d});let awD=(if EM{(EN*((-(((EP*a3d)-(sY*a3d))/aw3))*awn))}else{d});let awM=(if EY{(a31/sf[286])}else{d});let awN=(if EY{(a35/sf[286])}else{d});let awO=(if EY{(a39/sf[286])}else{d});let awP=(if EY{(a3d/sf[286])}else{d});let awU=(if EY{(awM/sf[289])}else{sf[334]});let awV=(if EY{(awN/sf[289])}else{sf[335]});let awW=(if EY{(awO/sf[289])}else{d});let awX=(if EY{(awP/sf[289])}else{d});let axE=(sf[290]*f64::powf(Fo,sf[358]));let axZ=(sf[834]*(if EY{((Fq*awA)+(EV*((if Fh{(awM+(sf[289]*((Fj*(-awU))/Fk)))}else{(if F9{(sf[289]*((Fa*awU)/Fb))}else{d})})*axE)))}else{(if EW{awA}else{d})}));let ay0=(sf[834]*(if EY{((Fq*awB)+(EV*((if Fh{(awN+(sf[289]*((Fj*(-awV))/Fk)))}else{(if F9{(sf[289]*((Fa*awV)/Fb))}else{d})})*axE)))}else{(if EW{awB}else{d})}));let ay1=(sf[834]*(if EY{((Fq*awC)+(EV*((if Fh{(awO+(sf[289]*((Fj*(-awW))/Fk)))}else{(if F9{(sf[289]*((Fa*awW)/Fb))}else{d})})*axE)))}else{(if EW{awC}else{d})}));let ay2=(sf[834]*(if EY{((Fq*awD)+(EV*((if Fh{(awP+(sf[289]*((Fj*(-awX))/Fk)))}else{(if F9{(sf[289]*((Fa*awX)/Fb))}else{d})})*axE)))}else{(if EW{awD}else{d})}));
        let ayt=(if EM{(FG*(if FA{(FB*axZ)}else{(if Fw{(Fx*axZ)}else{apg})}))}else{(if ED{(EE*auL)}else{(if Em{((Ez*((Eq*auf)+(Eo*(sf[836]*atx))))+(Er*(auL-(Ey*((Ew*auB)+(Et*((-(Dv*auf))/auR)))))))}else{(if Cp{((CV*apg)+(CS*(sf[835]*aoP)))}else{d})})})});let ayu=(if EM{((FG*(if FA{(FB*ay0)}else{(if Fw{(Fx*ay0)}else{aph})}))+(FF*sf[894]))}else{(if ED{((EE*auM)+(Eu*(sf[4]*aqr)))}else{(if Em{((Ez*((Eq*aug)+(Eo*(sf[836]*aty))))+(Er*(auM-(Ey*((Ew*auE)+(Et*(((Eo*aqr)-(Dv*aug))/auR)))))))}else{(if Cp{((CV*aph)+(CS*(sf[835]*aoQ)))}else{d})})})});let ayv=(if EM{((FG*(if FA{(FB*ay1)}else{(if Fw{(Fx*ay1)}else{api})}))+(FF*sf[895]))}else{(if ED{((EE*auN)+(Eu*(sf[4]*aqs)))}else{(if Em{((Ez*((Eq*auh)+(Eo*(sf[836]*atz))))+(Er*(auN-(Ey*((Ew*auH)+(Et*(((Eo*aqs)-(Dv*auh))/auR)))))))}else{(if Cp{((CV*api)+(CS*(sf[835]*aoR)))}else{d})})})});let ayw=(if EM{(FG*(if FA{(FB*ay2)}else{(if Fw{(Fx*ay2)}else{apj})}))}else{(if ED{((EE*auO)+(Eu*(sf[4]*aqt)))}else{(if Em{((Ez*((Eq*aui)+(Eo*(sf[836]*atA))))+(Er*(auO-(Ey*((Ew*auK)+(Et*(((Eo*aqt)-(Dv*aui))/auR)))))))}else{(if Cp{((CV*apj)+(CS*(sf[835]*aoS)))}else{d})})})});let ayL=(FR*FR);let aza=(FQ*FQ);let azp=(if FP{((((-(sf[376]*((FQ*a31)+(sY*anW))))/ayL)+(sf[657]*(a2F/sf[629])))+((-(sf[542]*anW))/aza))}else{d});let azq=(if FP{((((-(sf[376]*((FQ*a35)+(sY*anX))))/ayL)+(sf[657]*(a2I/sf[629])))+((-(sf[542]*anX))/aza))}else{d});let azr=(if FP{((((-(sf[376]*((FQ*a39)+(sY*anY))))/ayL)+(sf[657]*(a2L/sf[629])))+((-(sf[542]*anY))/aza))}else{d});let azs=(if FP{((((-(sf[376]*((FQ*a3d)+(sY*anZ))))/ayL)+(sf[657]*(a2O/sf[629])))+((-(sf[542]*anZ))/aza))}else{d});let azB=(if FZ{((ayt-azp)/fM)}else{awU});let azC=(if FZ{((ayu-azq)/fM)}else{awV});let azD=(if FZ{((ayv-azr)/fM)}else{awW});let azE=(if FZ{((ayw-azs)/fM)}else{awX});let aAj=(if Gd{(azp-(fM*((Gf*(-azB))/Gg)))}else{(if G5{(ayt-(fM*((G6*azB)/G7)))}else{ayt})});let aAk=(if Gd{(azq-(fM*((Gf*(-azC))/Gg)))}else{(if G5{(ayu-(fM*((G6*azC)/G7)))}else{ayu})});let aAl=(if Gd{(azr-(fM*((Gf*(-azD))/Gg)))}else{(if G5{(ayv-(fM*((G6*azD)/G7)))}else{ayv})});let aAm=(if Gd{(azs-(fM*((Gf*(-azE))/Gg)))}else{(if G5{(ayw-(fM*((G6*azE)/G7)))}else{ayw})});let aAp=((Gk*a31)+(sY*aAj));let aAs=((Gk*a35)+(sY*aAk));let aAv=((Gk*a39)+(sY*aAl));let aAy=((Gk*a3d)+(sY*aAm));let aAW=(Gq*Gq);let aP2=(w*sf[321]);let aP3=(sf[0]*w);let aP5=(w*sf[323]);let aQ4=(sf[15]*(sf[0]*(sf[695]*a8A)));let aQ8=((((if sb[33]{(sf[657]*((sf[232]*a4v)+(uH*(sf[230]*a4v))))}else{(if sb[31]{a4Z}else{(if ((sf[149])!=0.0){((a4Z+(uH*(((uF*(sf[818]*a4v))-(uB*((g2*a4D)/a55)))/a5b)))+(((uN*(uL*a4V))-(uM*a4V))/a5F))}else{d})})})+(sf[642]*a7A))+aP2)-(if y5{d}else{(if ((wE)!=0.0){(sf[21]*(sf[524]*((y0*(if wP{(wQ*a8S)}else{(if wL{(wM*a8S)}else{d})}))+(wU*((xZ*Yb)+(qZ*(sf[820]*(if xM{((xV*((xN*aa5)+(xo*sf[341])))+(xO*((xT*(xP*aa5))+(xQ*(xR*aa5)))))}else{(if xu{((sf[0]*xI)+(xF*(((xo*(-(if xz{(xA*aa5)}else{(if xv{(xw*aa5)}else{d})})))-(xG*aa5))/aak)))}else{d})}))))))))}else{d})}));let aQ9=((((if sb[33]{(sf[657]*((sf[232]*a4w)+((v3*a15)+(uH*(sf[230]*(WE+a4w))))))}else{(if sb[31]{a50}else{(if ((sf[149])!=0.0){((a50+((uH*(((uF*(sf[818]*a4w))-(uB*((g2*a4E)/a55)))/a5b))+(uG*a15)))+(((uN*((uL*a4W)+(ux*(sf[672]*WE))))-(uM*a4W))/a5F))}else{d})})})+(sf[642]*a7C))+aP3)-(if y5{d}else{(if ((wE)!=0.0){(sf[21]*(sf[524]*((y0*(if wP{(wQ*a8T)}else{(if wL{(wM*a8T)}else{d})}))+(wU*((xZ*Yc)+(qZ*(sf[820]*(if xM{((xV*((xN*aa6)+(xo*sf[342])))+(xO*((xT*(xP*aa6))+(xQ*(xR*aa6)))))}else{(if xu{((xI*sf[321])+(xF*(((xo*(-(if xz{(xA*aa6)}else{(if xv{(xw*aa6)}else{d})})))-(xG*aa6))/aak)))}else{d})}))))))))}else{d})}));let aQG=(sf[15]*(sf[0]*(-(zB*all))));let aQH=(sf[15]*(sf[0]*(-(zB*alm))));let aQI=(sf[15]*(sf[0]*(-(zB*aln))));let aQJ=(sf[15]*(sf[0]*(-(zB*alo))));let aQK=(sf[15]*(sf[0]*(-((BK*(if zA{d}else{(if ((yc)!=0.0){(sf[53]*(sf[525]*((zv*(if yr{(ys*abF)}else{(if yn{(yo*abF)}else{d})}))+(yw*((zu*abs)+(yg*(sf[821]*(if zj{((zq*((zk*acQ)+(yW*sf[342])))+(zl*((zo*(xP*acQ))+(zm*(xR*acQ)))))}else{(if z1{((zf*sf[321])+(zc*(((yW*(-(if z6{(z7*acQ)}else{(if z2{(z3*acQ)}else{d})})))-(zd*acQ))/ad5)))}else{d})}))))))))}else{d})}))+(zB*alp)))));
        let aQL=(sf[15]*(sf[0]*(-((BK*(if zA{d}else{(if ((yc)!=0.0){(sf[53]*(sf[525]*((zv*(if yr{(ys*abG)}else{(if yn{(yo*abG)}else{d})}))+(yw*((zu*abt)+(yg*(sf[821]*(if zj{((zq*((zk*acR)+(yW*sf[341])))+(zl*((zo*(xP*acR))+(zm*(xR*acR)))))}else{(if z1{((sf[0]*zf)+(zc*(((yW*(-(if z6{(z7*acR)}else{(if z2{(z3*acR)}else{d})})))-(zd*acR))/ad5)))}else{d})}))))))))}else{d})}))+(zB*alq)))));let aQM=(sf[15]*(sf[0]*(-(zB*alr))));let aQN=(sf[15]*(sf[0]*(-(zB*als))));let aQO=(sf[15]*(sf[0]*(-(zB*alt))));

        stamper.stamp_current_node3_local(
            Some(6),
            Some(7),
            multiplicity * ((sf[15]*(sf[0]*n7))),
            5,
            multiplicity * ((sf[15]*(sf[0]*PB))),
            6,
            multiplicity * ((sf[15]*(sf[0]*PC))),
            7,
            multiplicity * ((sf[15]*(sf[0]*PD))),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*sY))),
            [3, 5, 6, 7],
            [(sf[15]*(sf[0]*a31)), (sf[15]*(sf[0]*a35)), (sf[15]*(sf[0]*a39)), (sf[15]*(sf[0]*a3d))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*((sf[695]*(wx-b))+((if sb[30]{vz}else{(if ((sf[149])!=0.0){(vz+(vB/vF))}else{d})})+(sf[689]*(w7-b))))))),
            [3, 4, 5, 6, 7, 9],
            [(sf[15]*(sf[0]*((sf[695]*a8x)+((if sb[30]{a6S}else{(if ((sf[149])!=0.0){(a6S+(((vF*(sf[819]*a6F))-(vB*((g2*(if vs{(vt*sf[857])}else{(if vo{(vp*sf[857])}else{a4D})}))/a71)))/a78))}else{d})})+(sf[689]*a7P))))), (sf[15]*(sf[0]*((sf[695]*a8y)+((if sb[30]{a6T}else{(if ((sf[149])!=0.0){(a6T+(((vF*(sf[819]*a6G))-(vB*((g2*(if vs{(vt*sf[856])}else{(if vo{(vp*sf[856])}else{d})}))/a71)))/a78))}else{d})})+(sf[689]*a7Q))))), (sf[15]*(sf[0]*((sf[695]*a8z)+((if sb[30]{a6U}else{(if ((sf[149])!=0.0){(a6U+(((vF*(sf[819]*a6H))-(vB*((g2*(if vs{d}else{(if vo{d}else{a4E})}))/a71)))/a78))}else{d})})+(sf[689]*a7R))))), aQ4, aQ4, (sf[15]*(sf[0]*(sf[695]*a8B)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*((sf[700]*(tr-b))+((tO*tQ)+((((if sb[33]{(sf[657]*((uy*sf[232])+(uH*v3)))}else{(if sb[31]{uz}else{(if ((sf[149])!=0.0){((uz+(uG*uH))+(uM/uN))}else{d})})})+(sf[642]*(vU-b)))+(w*ky))-(if y5{d}else{(if ((wE)!=0.0){(sf[21]*(sf[524]*(wU*y0)))}else{d})}))))))),
            [3, 4, 5, 6, 7],
            [(sf[15]*(sf[0]*((sf[700]*a3I)+(((tQ*(sf[229]*a48))+(tO*((-a48)*a4e)))+aQ8)))), (sf[15]*(sf[0]*(sf[642]*a7B))), (sf[15]*(sf[0]*((sf[700]*a3J)+(((tQ*(sf[229]*a49))+(tO*((-a49)*a4e)))+aQ9)))), (sf[15]*(sf[0]*(if sb[33]{(sf[657]*((v3*a16)+(uH*(sf[230]*WF))))}else{(if sb[31]{d}else{(if ((sf[149])!=0.0){((uG*a16)+(((uN*((uL*a4X)+(ux*(sf[672]*WF))))-(uM*a4X))/a5F))}else{d})})}))), (sf[15]*(sf[0]*(if sb[33]{(sf[657]*((v3*a17)+(uH*(sf[230]*WG))))}else{(if sb[31]{d}else{(if ((sf[149])!=0.0){((uG*a17)+(((uN*((uL*a4Y)+(ux*(sf[672]*WG))))-(uM*a4Y))/a5F))}else{d})})})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(6),
            multiplicity * ((if ((sf[149])!=0.0){M5}else{d})),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(if ((sf[149])!=0.0){aQG}else{d}), (if ((sf[149])!=0.0){aQH}else{d}), (if ((sf[149])!=0.0){aQI}else{d}), (if ((sf[149])!=0.0){aQJ}else{d}), (if ((sf[149])!=0.0){aQK}else{d}), (if ((sf[149])!=0.0){aQL}else{d}), (if ((sf[149])!=0.0){aQM}else{d}), (if ((sf[149])!=0.0){aQN}else{d}), (if ((sf[149])!=0.0){aQO}else{d})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(7),
            multiplicity * ((if sb[30]{M5}else{d})),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(if sb[30]{aQG}else{d}), (if sb[30]{aQH}else{d}), (if sb[30]{aQI}else{d}), (if sb[30]{aQJ}else{d}), (if sb[30]{aQK}else{d}), (if sb[30]{aQL}else{d}), (if sb[30]{aQM}else{d}), (if sb[30]{aQN}else{d}), (if sb[30]{aQO}else{d})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(5),
            multiplicity * ((sf[15]*(sf[0]*(C9/C6)))),
            [3, 4, 5, 6, 7],
            [(sf[15]*(sf[0]*((-(C9*anW))/ao6))), (sf[15]*(sf[0]*((sf[0]+(sf[791]*(if lK{(lL*sf[856])}else{(if ((lH)!=0.0){(lI*sf[856])}else{d})})))/C6))), (sf[15]*(sf[0]*(((C6*(sf[321]+(sf[791]*(if lK{(lL*sf[857])}else{(if ((lH)!=0.0){(lI*sf[857])}else{d})}))))-(C9*anX))/ao6))), (sf[15]*(sf[0]*((-(C9*anY))/ao6))), (sf[15]*(sf[0]*((-(C9*anZ))/ao6)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * ((sf[15]*(sf[0]*(-Gv)))),
            [3, 5, 6, 7],
            [(sf[15]*(sf[0]*(-(if Gu{aAp}else{(if Go{(((Gq*((Gl*azp)+(FY*aAp)))-(Gp*(azp+aAj)))/aAW)}else{(if FZ{aAp}else{d})})})))), (sf[15]*(sf[0]*(-(if Gu{aAs}else{(if Go{(((Gq*((Gl*azq)+(FY*aAs)))-(Gp*(azq+aAk)))/aAW)}else{(if FZ{aAs}else{d})})})))), (sf[15]*(sf[0]*(-(if Gu{aAv}else{(if Go{(((Gq*((Gl*azr)+(FY*aAv)))-(Gp*(azr+aAl)))/aAW)}else{(if FZ{aAv}else{d})})})))), (sf[15]*(sf[0]*(-(if Gu{aAy}else{(if Go{(((Gq*((Gl*azs)+(FY*aAy)))-(Gp*(azs+aAm)))/aAW)}else{(if FZ{aAy}else{d})})}))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * ((sf[15]*((sf[0]*(sf[0]*(kG-kw)))/sf[542]))),
            2,
            multiplicity * (sf[921]),
            3,
            multiplicity * (sf[922]),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(4),
            multiplicity * ((sf[15]*((sf[0]*kL)/sf[550]))),
            1,
            multiplicity * (sf[925]),
            4,
            multiplicity * (sf[926]),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * (Mk),
            [3, 4, 5, 6, 7, 9],
            [aRM, aRN, aRO, aRP, aRQ, aRR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (Mn),
            3,
            multiplicity * (aRW),
            4,
            multiplicity * (aRX),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(7),
            multiplicity * (Mq),
            [3, 4, 5, 6, 7, 9],
            [aSa, aSb, aSc, aSd, aSe, aSf],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            Some(5),
            multiplicity * (Mt),
            [3, 4, 5, 6, 7, 9],
            [aSs, aSt, aSu, aSv, aSw, aSx],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (Mx),
            1,
            multiplicity * (aSC),
            2,
            multiplicity * (aSD),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (MB),
            0,
            multiplicity * (aSI),
            1,
            multiplicity * (aSJ),
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(8),
            multiplicity * ((sf[15]*(sf[0]*(AS*BK)))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(sf[15]*(sf[0]*(amj+(AS*all)))), (sf[15]*(sf[0]*((BK*(if ((sf[242])!=0.0){((AQ*ag0)+(Ad*aio))}else{d}))+(AS*alm)))), (sf[15]*(sf[0]*((BK*(if ((sf[242])!=0.0){(Ad*aip)}else{d}))+(AS*aln)))), (sf[15]*(sf[0]*(amj+(AS*alo)))), (sf[15]*(sf[0]*((BK*(if ((sf[242])!=0.0){(aiv+(Ad*aiq))}else{d}))+(AS*alp)))), (sf[15]*(sf[0]*((BK*(if ((sf[242])!=0.0){(aiE+(Ad*air))}else{d}))+(AS*alq)))), (sf[15]*(sf[0]*((BK*(if ((sf[242])!=0.0){(aiE+(Ad*ais))}else{d}))+(AS*alr)))), (sf[15]*(sf[0]*((BK*(if ((sf[242])!=0.0){((AQ*ag2)+(Ad*ait))}else{d}))+(AS*als)))), (sf[15]*(sf[0]*((BK*(if ((sf[242])!=0.0){(aiE+(Ad*aiu))}else{d}))+(AS*alt))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(8),
            multiplicity * ((sf[15]*(sf[780]*(sf[0]*l3)))),
            [0, 1, 4, 5, 6, 7, 8, 9],
            [sf[931], sf[932], sf[932], sf[932], sf[933], sf[933], sf[934], sf[933]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(8),
            multiplicity * (MJ),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [aTs, aTt, aTu, aTs, aTv, aTw, aTx, aTy, aTz],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(9),
            multiplicity * ((sf[15]*(sf[0]*((A3*BK)+((wm*BK)+(w*kZ)))))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(sf[15]*(sf[0]*((A3*all)+(wm*all)))), (sf[15]*(sf[0]*((A3*alm)+(wm*alm)))), (sf[15]*(sf[0]*((A3*aln)+((BK*(sf[649]*a8c))+(wm*aln))))), (sf[15]*(sf[0]*(((BK*(if ((sf[242])!=0.0){(sf[7]*afa)}else{afa}))+(A3*alo))+(((BK*(sf[649]*a8d))+(wm*alo))+aP3)))), (sf[15]*(sf[0]*(((BK*(if ((sf[242])!=0.0){(sf[7]*afe)}else{afe}))+(A3*alp))+(((BK*(sf[649]*a8e))+(wm*alp))+(w*sf[322]))))), (sf[15]*(sf[0]*((alQ+(A3*alq))+((ama+(wm*alq))+aP5)))), (sf[15]*(sf[0]*((alQ+(A3*alr))+((ama+(wm*alr))+aP5)))), (sf[15]*(sf[0]*((A3*als)+(wm*als)))), (sf[15]*(sf[0]*(((BK*(if ((sf[242])!=0.0){(sf[7]*afm)}else{afm}))+(A3*alt))+(((BK*(sf[649]*a8g))+(wm*alt))+aP2))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(9),
            multiplicity * (MP),
            [4, 5, 6, 7, 9],
            [aU5, aU6, aU7, aU7, aU8],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(8),
            Some(9),
            multiplicity * ((if ((sf[196])!=0.0){(sf[15]*(sf[785]*(sf[0]*kW)))}else{d})),
            8,
            multiplicity * (sf[939]),
            9,
            multiplicity * (sf[940]),
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
            multiplicity * ((if ((sf[197])!=0.0){(sf[15]*(sf[790]*(sf[0]*kT)))}else{d})),
            6,
            multiplicity * (sf[945]),
            9,
            multiplicity * (sf[946]),
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
            multiplicity * (MY),
            10,
            multiplicity * (b),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(3),
            multiplicity * (N0),
            [3, 4, 5, 6, 7, 9, 10],
            [aUl, aUm, aUn, aUo, aUp, aUq, aUr],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(5),
            multiplicity * ((Ld*MY)),
            10,
            multiplicity * (Ld),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(3),
            multiplicity * (MY),
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
            b, d, H, I, X, bL, fM, fQ,
            g2, gs, ks, kw, ky, kD, kG, kL,
            kT, kW, kZ, l3, lE, lF, lH, lK,
            lL, n7, p5, q3, qs, qv, qy, qZ,
            sh, sR, sS, sX, sY, th, tj, tm,
            tn, tw, u2, u4, u6, ub, uc, uj,
            uk, um, ur, ut, vj, vl, vn, vs,
            vt, vU, w7, wk, wx, wE, wF, wI,
            wK, wP, wQ, wW, x0, x3, xb, xc,
            xd, xf, xh, xl, xm, xo, xr, xt,
            xu, xz, xA, yc, ye, yg, yh, yk,
            ym, yr, ys, yx, yA, yC, yK, yL,
            yM, yO, yT, yU, yW, yY, z0, z1,
            z6, z7, Ad, Au, AQ, C0, Cc, Cp,
            Cq, Cr, Cu, Cv, Cz, CA, CC, CG,
            CI, CN, CO, D3, EM, EN, EP, ER,
            ET, EV, EW, EY, F6, F9, Fa, Fb,
            Fh, Fj, Fk, Fo, Fq, Ft, Fv, FA,
            FB, L5, Mk, Mn, Mq, Mt, Mx, MB,
            MJ, MP, MY, N0, NE, NF, NG, NH,
            PB, PC, PD, Ug, Uh, Ui, WE, WF,
            WG, Xl, Xm, Xn, Xu, Xv, Xw, XD,
            XE, XF, Yb, Yc, a15, a16, a17, a2z,
            a2A, a2B, a2C, a2F, a2I, a2L, a2O, a2P,
            a2Q, a2R, a2T, a2X, a30, a3y, a3z, a4v,
            a4w, a6F, a6G, a6H, a7A, a7B, a7C, a7P,
            a7Q, a7R, a8c, a8d, a8e, a8f, a8g, a8x,
            a8y, a8z, a8A, a8B, afZ, ag0, ag1, ag2,
            agf, agg, agh, agi, agj, agk, agl, agm,
            ain, aio, aip, aiq, air, ais, ait, aiu,
            anp, anq, anr, ans, aRM, aRN, aRO, aRP,
            aRQ, aRR, aRW, aRX, aSa, aSb, aSc, aSd,
            aSe, aSf, aSs, aSt, aSu, aSv, aSw, aSx,
            aSC, aSD, aSI, aSJ, aTs, aTt, aTu, aTv,
            aTw, aTx, aTy, aTz, aU5, aU6, aU7, aU8,
            aUl, aUm, aUn, aUo, aUp, aUq, aUr,
        }=self.eval_common_stamp_values::<true>(ctx);
        let p=&(*self.params);
        let m=self.multiplicity;
        let multiplicity=m;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(3),
            &[3, 4, 5, 6, 7, 9],
            &[aRM, aRN, aRO, aRP, aRQ, aRR],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2_local(
            Some(4),
            Some(3),
            3,
            multiplicity * (aRW),
            4,
            multiplicity * (aRX),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(7),
            &[3, 4, 5, 6, 7, 9],
            &[aSa, aSb, aSc, aSd, aSe, aSf],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            Some(5),
            &[3, 4, 5, 6, 7, 9],
            &[aSs, aSt, aSu, aSv, aSw, aSx],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(2),
            1,
            multiplicity * (aSC),
            2,
            multiplicity * (aSD),
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(0),
            0,
            multiplicity * (aSI),
            1,
            multiplicity * (aSJ),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(8),
            &[0, 1, 3, 4, 5, 6, 7, 8, 9],
            &[aTs, aTt, aTu, aTs, aTv, aTw, aTx, aTy, aTz],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            Some(9),
            &[4, 5, 6, 7, 9],
            &[aU5, aU6, aU7, aU7, aU8],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(3),
            &[3, 4, 5, 6, 7, 9, 10],
            &[aUl, aUm, aUn, aUo, aUp, aUq, aUr],
            &[],
            &[],
            multiplicity,
        );
    }
}
