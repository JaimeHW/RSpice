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
    b: f64, d: f64, N: f64, O: f64, a3: f64, bR: f64,
    gl: f64, gp: f64, gB: f64, h1: f64, lt: f64, lx: f64,
    lz: f64, lE: f64, lH: f64, lK: f64, lP: f64, lX: f64,
    m0: f64, m3: f64, m7: f64, mn: f64, mK: f64, mL: f64,
    mN: f64, mQ: bool, mR: f64, n7: f64, n9: f64, nc: bool,
    nd: f64, nt: f64, nv: f64, ny: bool, nz: f64, oK: f64,
    qI: f64, rG: f64, s5: f64, s8: f64, sb_: f64, sC: f64,
    tU: f64, uu: f64, uv: f64, uA: f64, uB: f64, uU: f64,
    uW: f64, uZ: bool, v0: f64, v9: f64, vF: f64, vH: f64,
    vJ: f64, vO: bool, vP: f64, vW: f64, vX: f64, vZ: f64,
    w4: bool, w6: f64, wW: f64, wY: f64, x0: f64, x5: bool,
    x6: f64, xx: f64, xK: f64, xX: f64, ya: f64, yh: f64,
    yi: f64, yl: f64, yn: f64, ys: bool, yt: f64, yz: f64,
    yD: f64, yG: f64, yO: f64, yP: f64, yQ: f64, yS: f64,
    yU: f64, yY: f64, yZ: f64, z1: f64, z4: f64, z6: f64,
    z7: bool, zc: bool, zd: f64, zP: f64, zR: f64, zT: f64,
    zU: f64, zX: f64, zZ: f64, A4: bool, A5: f64, Aa: f64,
    Ad: f64, Af: f64, An: f64, Ao: f64, Ap: f64, Ar: f64,
    Aw: f64, Ax: f64, Az: f64, AB: f64, AD: f64, AE: bool,
    AJ: bool, AK: f64, CR: f64, Df: f64, Dx: f64, DU: f64,
    F6: f64, Fi: f64, Fv: bool, Fw: bool, Fx: f64, FA: bool,
    FB: f64, FF: f64, FG: f64, FI: f64, FM: f64, FO: f64,
    FT: bool, FU: f64, G9: bool, HS: bool, HT: f64, HV: f64,
    HX: f64, HZ: f64, I1: f64, I2: bool, I4: bool, Ic: f64,
    If: bool, Ig: f64, Ih: f64, In: bool, Ip: f64, Iq: f64,
    Iu: f64, Iw: f64, Iz: f64, IB: f64, IG: bool, IH: f64,
    OJ: f64, Q6: f64, Q9: f64, Qc: f64, Qf: f64, Qi: f64,
    Qm: f64, Qq: f64, Qy: f64, QE: f64, QN: f64, QP: f64,
    R3: f64, R4: f64, Rt: f64, Ru: f64, Rv: f64, Rw: f64,
    TW: f64, TX: f64, TY: f64, YB: f64, YC: f64, YD: f64,
    a0Z: f64, a10: f64, a11: f64, a1G: f64, a1H: f64, a1I: f64,
    a1P: f64, a1Q: f64, a1R: f64, a1Y: f64, a1Z: f64, a20: f64,
    a2w: f64, a2x: f64, a5q: f64, a5r: f64, a5s: f64, a6U: f64,
    a6V: f64, a6W: f64, a6X: f64, a70: f64, a73: f64, a76: f64,
    a79: f64, a7a: f64, a7b: f64, a7c: f64, a7e: f64, a7i: f64,
    a7l: f64, a7T: f64, a7U: f64, a8Q: f64, a8R: f64, ab0: f64,
    ab1: f64, ab2: f64, abV: f64, abW: f64, abX: f64, aca: f64,
    acb: f64, acc: f64, acx: f64, acy: f64, acz: f64, acA: f64,
    acB: f64, acS: f64, acT: f64, acU: f64, acV: f64, acW: f64,
    an8: f64, an9: f64, ana: f64, anb: f64, aow: f64, aox: f64,
    aoy: f64, aoz: f64, aoA: f64, aoB: f64, aoO: f64, aoP: f64,
    aoQ: f64, aoR: f64, aoS: f64, aoT: f64, aoU: f64, aoV: f64,
    ar7: f64, ar8: f64, ar9: f64, ara: f64, arb: f64, arc: f64,
    ard: f64, are: f64, arf: f64, awJ: f64, awK: f64, awL: f64,
    awM: f64, b2x: f64, b2y: f64, b2z: f64, b2A: f64, b2B: f64,
    b2C: f64, b2H: f64, b2I: f64, b2V: f64, b2W: f64, b2X: f64,
    b2Y: f64, b2Z: f64, b30: f64, b35: f64, b36: f64, b3j: f64,
    b3k: f64, b3l: f64, b3m: f64, b3n: f64, b3o: f64, b3t: f64,
    b3u: f64, b3z: f64, b3A: f64, b4n: f64, b4o: f64, b4p: f64,
    b4q: f64, b4r: f64, b4s: f64, b4t: f64, b4u: f64, b4v: f64,
    b51: f64, b52: f64, b53: f64, b54: f64, b5h: f64, b5i: f64,
    b5j: f64, b5k: f64, b5l: f64, b5m: f64, b5n: f64,
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
        let b=1.0;let d=0.0;let N=0.001;let O=2.0;let a3=0.1;let bR=3.0;let gl=1e-6;let gp=0.5;let gB=4.0;let h1=6.0;let lq=ctx.node_voltage(n[6]);let lr=ctx.node_voltage(n[7]);let lt=(sf[0]*(lq-lr));let lu=ctx.node_voltage(n[8]);let lw=(sf[0]*(lq-lu));let lx=ctx.node_voltage(n[4]);let lz=(sf[0]*(lq-lx));let lA=ctx.node_voltage(n[5]);let lC=(sf[0]*(lA-lx));let lE=(sf[0]*(lA-lq));let lH=(sf[0]*(ctx.node_voltage(n[3])-lr));let lJ=(sf[0]*(lr-lu));let lK=ctx.node_voltage(n[2]);let lN=ctx.node_voltage(n[1]);let lP=(sf[0]*(lN-lA));let lU=(sf[0]*(lN-ctx.node_voltage(n[0])));let lV=ctx.node_voltage(n[10]);let lX=(sf[0]*(lV-lr));let m0=(sf[0]*(ctx.node_voltage(n[9])-lV));let m3=(((lw+lE)-lJ)-lX);let m7=((m3+(lP+(-lU)))-m0);let m8=(lU+m7);let m9=(lH-lX);let mb=(sf[409]*lw);let me=(if (mb<sf[215]){b}else{d});let mf=(mb).exp();let mh=(!((me)!=0.0));let mj=(if mh{sf[216]}else{d});let mn=(if mh{(mj*(b+(mb-sf[215])))}else{(if ((me)!=0.0){mf}else{d})});let mo=(sf[409]*lz);let mp=(mo/sf[639]);let mr=(if (mp<sf[215]){b}else{d});let ms=(mp).exp();let mu=(!((mr)!=0.0));let mv=(if mu{sf[216]}else{mj});let mz=(if mu{(mv*(b+(mp-sf[215])))}else{(if ((mr)!=0.0){ms}else{d})});let mA=(sf[409]*m3);let mC=(if (mA<sf[215]){b}else{d});let mD=(mA).exp();let mF=(!((mC)!=0.0));let mG=(if mF{sf[216]}else{mv});let mK=(if mF{(mG*(b+(mA-sf[215])))}else{(if ((mC)!=0.0){mD}else{d})});let mL=(sf[409]*lE);let mN=(if (mL<sf[215]){b}else{d});let mQ=(!((mN)!=0.0));let mR=(if mQ{sf[216]}else{mG});let mW=(sf[409]*m8);let mY=(if (mW<sf[215]){b}else{d});let mZ=(mW).exp();let n1=(!((mY)!=0.0));let n2=(if n1{sf[216]}else{mR});let n6=(if n1{(n2*(b+(mW-sf[215])))}else{(if ((mY)!=0.0){mZ}else{d})});let n7=(sf[409]*lH);let n9=(if (n7<sf[215]){b}else{d});let nc=(!((n9)!=0.0));let nd=(if nc{sf[216]}else{n2});let ni=(sf[409]*(m9-m0));let nk=(if (ni<sf[215]){b}else{d});let nl=(ni).exp();let nn=(!((nk)!=0.0));let no=(if nn{sf[216]}else{nd});let ns=(if nn{(no*(b+(ni-sf[215])))}else{(if ((nk)!=0.0){nl}else{d})});let nt=(sf[409]*m9);let nv=(if (nt<sf[215]){b}else{d});let ny=(!((nv)!=0.0));let nz=(if ny{sf[216]}else{no});let nF=(sf[409]*(m8-sf[497]));let nH=(if (nF<sf[215]){b}else{d});let nI=(nF).exp();let nK=(!((nH)!=0.0));let nL=(if nK{sf[216]}else{nz});let nR=(sf[409]*(m3-sf[497]));let nT=(if (nR<sf[215]){b}else{d});let nU=(nR).exp();let nW=(!((nT)!=0.0));let nX=(if nW{sf[216]}else{nL});let o3=(sf[409]*(lw-sf[497]));let o5=(if (o3<sf[215]){b}else{d});let o6=(o3).exp();let o8=(!((o5)!=0.0));let o9=(if o8{sf[216]}else{nX});let od=(if o8{(o9*(b+(o3-sf[215])))}else{(if ((o5)!=0.0){o6}else{d})});let of=(sf[409]*(lt-sf[497]));let oh=(if (of<sf[215]){b}else{d});let oi=(of).exp();let ok=(!((oh)!=0.0));let ol=(if ok{sf[216]}else{o9});let op=(if ok{(ol*(b+(of-sf[215])))}else{(if ((oh)!=0.0){oi}else{d})});let os=((b+(gB*od))).sqrt();let ov=((b+(gB*op))).sqrt();let ow=(O*op);let ox=(b+ov);let oy=(ow/ox);let oB=(if (oy<sf[217]){b}else{d});let oC=(if ((oB)!=0.0){sf[217]}else{oy});let oE=(b+os);let oF=(oE/ox);let oI=(sf[408]*((os-ov)-(oF).ln()));let oK=((lJ+oI)/sf[615]);let oM=(if (oK>d){b}else{d});let oN=100.0;let oP=(if (lt<oN){b}else{d});let oQ=(((oM)!=0.0)&&((oP)!=0.0));let oT=(((oM)!=0.0)&&(!((oP)!=0.0)));let oV=(b+(lt-oN));let p1=(sf[615]*(gp*oK));let p3=(b+(sf[409]*p1));let p8=(if ((oM)!=0.0){((sf[497]+(sf[862]*(p3).ln()))-(if oT{(oN+(oV).ln())}else{(if oQ{lt}else{d})}))}else{d});let pb=(if ((oM)!=0.0){sf[863]}else{d});let pd=(if ((oM)!=0.0){(pb*pb)}else{gl});let ph=(if (p8<d){b}else{d});let pi=(((oM)!=0.0)&&((ph)!=0.0));let pj=(gp*pd);let pl=((pd+(if ((oM)!=0.0){(p8*p8)}else{sf[667]}))).sqrt();let pm=(pl-p8);let pq=(((oM)!=0.0)&&(!((ph)!=0.0)));let pt=(if pq{(gp*(p8+pl))}else{(if pi{(pj/pm)}else{d})});let px=(pt+sf[220]);let py=(pt*px);let pB=(sf[219]*(pt+sf[864]));let pD=(if ((oM)!=0.0){(py/pB)}else{d});let pF=(if ((oM)!=0.0){(oK/pD)}else{d});let pJ=(if ((oM)!=0.0){((pF-b)/sf[221])}else{sf[646]});let pL=(if (pF<b){b}else{d});let pM=(((oM)!=0.0)&&((pL)!=0.0));let pN=(pJ).exp();let pO=(b+pN);let pU=(((oM)!=0.0)&&(!((pL)!=0.0)));let pW=((-pJ)).exp();let pX=(b+pW);
        let qa=(if ((oM)!=0.0){((if pU{(pF+(sf[221]*(pX).ln()))}else{(if pM{(b+(sf[221]*(pO).ln()))}else{d})})/sf[227])}else{d});let qc=(if ((oM)!=0.0){(pt/sf[220])}else{d});let qd=(gB*qa);let qe=(qc*qd);let qf=(b+qc);let qi=((b+(qe*qf))).sqrt();let qj=(b+qi);let qk=(O*qa);let ql=(qf*qk);let qn=(if ((oM)!=0.0){(qj/ql)}else{d});let qp=(oC*qn);let qq=((b-qn)+qp);let qr=(b+qp);let qt=(if ((oM)!=0.0){(qq/qr)}else{d});let qw=(if ((oM)!=0.0){(sf[409]*(p1*qt))}else{d});let qz=(b+(oC+qw));let qC=(if ((oM)!=0.0){((O*qw)+(oC*qz))}else{d});let qF=(if ((oM)!=0.0){(gp*(qw-b))}else{d});let qI=(if ((oM)!=0.0){(qC+(qF*qF))}else{d});let qK=(if (qw>=b){b}else{d});let qL=(((oM)!=0.0)&&((qK)!=0.0));let qM=(qI).sqrt();let qQ=(((oM)!=0.0)&&(!((qK)!=0.0)));let qR=(qM-qF);let qT=(if qQ{(qC/qR)}else{(if qL{(qF+qM)}else{d})});let qX=(((oM)!=0.0)&&(((if (qT<sf[228]){b}else{d}))!=0.0));let qY=(if qX{sf[228]}else{qT});let qZ=(b+qY);let r8=(if ((oM)!=0.0){(sf[229]*(oK-sf[218]))}else{d});let rf=(((if ((oM)!=0.0){(oK*sf[868])}else{d})+(r8*r8))).sqrt();let rp=(((oM)!=0.0)&&sb[22]);let rq=(O*oK);let rr=(oK+pD);let rw=(oK*sf[218]);let rx=(oK+sf[218]);let rC=(!((oM)!=0.0));let rD=(O*od);let rG=(if rC{mn}else{(if ((oM)!=0.0){((qY*qZ)*sf[866])}else{d})});let rS=(if (((lJ).abs()<sf[870])||((oI).abs()<(sf[871]*(os+ov)))){b}else{d});let rT=(rC&&((rS)!=0.0));let rU=(oC+(if rC{(rD/oE)}else{qY}));let rW=(if rT{(gp*rU)}else{d});let rX=(b+rW);let s1=(rC&&(!((rS)!=0.0)));let s3=((lw+oI)-lt);let s5=(if s1{(oI/s3)}else{(if rT{(rW/rX)}else{qt})});let s7=(if rC{sf[869]}else{(if rp{(sf[535]*(a3+(rq/rr)))}else{(if (((oM)!=0.0)&&((sf[231])!=0.0)){sf[869]}else{d})})});let s8=(if rC{oK}else{(if ((oM)!=0.0){(rw/rx)}else{d})});let sb_=(if rC{(b-(s8/sf[218]))}else{(if ((oM)!=0.0){(sf[218]/rx)}else{d})});let si=((lz-sf[872])/sf[873]);let sk=(if (lz<sf[872]){b}else{d});let sl=(si).exp();let sm=(b+sl);let sr=(!((sk)!=0.0));let st=((-si)).exp();let su=(b+st);let sy=(if sr{(sf[872]-(sf[873]*(su).ln()))}else{(if ((sk)!=0.0){(lz-(sf[873]*(sm).ln()))}else{d})});let sA=(b-(sf[576]*sy));let sC=f64::powf(sA,sf[235]);let sI=((sf[874]*(b-sC))+(bR*(lz-sy)));let sV=(if sb[28]{lw}else{(if sb[26]{(lt+(if rC{lJ}else{(if ((oM)!=0.0){(r8+rf)}else{d})}))}else{(if ((sf[237])!=0.0){lt}else{d})})});let t3=(sV-sf[880]);let t4=(t3/s7);let t6=(if (sV<sf[880]){b}else{d});let t7=(t4).exp();let t8=(b+t7);let t9=(t8).ln();let td=(!((t6)!=0.0));let tf=((-t4)).exp();let tg=(b+tf);let th=(tg).ln();let tk=(if td{(sf[880]-(s7*th))}else{(if ((t6)!=0.0){(sV-(s7*t9))}else{d})});let tm=f64::powf(sb_,sf[240]);let tq=(b-(tk/sf[535]));let tr=f64::powf(tq,sf[241]);let tv=(sf[877]*tm);let tw=(sV-tk);let tB=((sf[876]*((sf[881]*(b-(tm*tr)))+(tv*tw)))+(sf[592]*lt));let tE=(mz*sf[883]);let tG=((b+tE)).sqrt();let tH=(b+tG);let tI=(tE/tH);let tK=f64::powf(rG,sf[884]);let tL=(sf[883]*tK);let tN=((b+tL)).sqrt();let tO=(b+tN);let tP=(tL/tO);let tT=(b+(sI/sf[801]));let tU=(tB/sf[799]);let tV=(tT+tU);let u6=((if sb[30]{(sf[409]*(sf[846]*tT))}else{d})).exp();let u7=((if sb[30]{(sf[409]*(sf[846]*((-tB)/sf[799])))}else{d})).exp();let ud=(if sb[30]{((u6-u7)/sf[887])}else{(if ((sf[242])!=0.0){tV}else{d})});let ue=0.010000000000000002;let uf=(ud*ud);let uh=(if (ud<d){b}else{d});let ui=0.005000000000000001;let uk=((ue+uf)).sqrt();let ul=(uk-ud);let uo=(!((uh)!=0.0));let ur=(if uo{(gp*(ud+uk))}else{(if ((uh)!=0.0){(ui/ul)}else{d})});let uu=(b+(gp*(tI+tP)));let uv=(ur*uu);let uy=(tK*sf[888]);let uz=(sf[684]*mz);let uA=(uz-uy);let uB=(uA/uv);let uC=0.0001;let uD=(lz/uC);let uE=(lz<d);let uF=(if uE{b}else{d});let uG=(uD).exp();let uH=(b+uG);let uL=(!((uF)!=0.0));let uN=((-uD)).exp();let uO=(b+uN);let uS=(if uL{(lz+(uC*(uO).ln()))}else{(if ((uF)!=0.0){(uC*(uH).ln())}else{d})});let uU=(uS/sf[244]);let uW=(if (uU<sf[215]){b}else{d});let uZ=(!((uW)!=0.0));let v0=(if uZ{sf[216]}else{ol});let v9=((lz-sf[245])/N);let vv=(mo/sf[149]);let vx=(if (vv<sf[215]){b}else{d});let vy=(vv).exp();let vA=(!((vx)!=0.0));let vB=(if vA{sf[216]}else{v0});let vF=(if vA{(vB*(b+(vv-sf[215])))}else{(if ((vx)!=0.0){vy}else{uS})});let vH=(sf[409]*(lz-sf[555]));
        let vJ=(if (vH<sf[215]){b}else{d});let vO=(((sf[155])!=0.0)&&(!((vJ)!=0.0)));let vP=(if vO{sf[216]}else{vB});let vW=((uB/sf[684])-1000.0);let vX=40.0;let vZ=(if (vW<vX){b}else{d});let w4=(((sf[155])!=0.0)&&(!((vZ)!=0.0)));let w6=(if w4{2.3538526683702e17}else{vP});let wL=(sf[409]*lC);let wM=(wL/sf[153]);let wO=(if (wM<sf[215]){b}else{d});let wP=(wM).exp();let wR=(!((wO)!=0.0));let wS=(if wR{sf[216]}else{w6});let wW=(if wR{(wS*(b+(wM-sf[215])))}else{(if ((wO)!=0.0){wP}else{vF})});let wY=(sf[409]*(lC-sf[555]));let x0=(if (wY<sf[215]){b}else{d});let x5=(((sf[155])!=0.0)&&(!((x0)!=0.0)));let x6=(if x5{sf[216]}else{wS});let xn=(mo/sf[136]);let xp=(if (xn<sf[215]){b}else{d});let xq=(xn).exp();let xs=(!((xp)!=0.0));let xt=(if xs{sf[216]}else{x6});let xx=(if xs{(xt*(b+(xn-sf[215])))}else{(if ((xp)!=0.0){xq}else{wW})});let xA=(wL/sf[171]);let xC=(if (xA<sf[215]){b}else{d});let xD=(xA).exp();let xF=(!((xC)!=0.0));let xG=(if xF{sf[216]}else{xt});let xK=(if xF{(xG*(b+(xA-sf[215])))}else{(if ((xC)!=0.0){xD}else{xx})});let xN=(mA/sf[142]);let xP=(if (xN<sf[215]){b}else{d});let xQ=(xN).exp();let xS=(!((xP)!=0.0));let xT=(if xS{sf[216]}else{xG});let xX=(if xS{(xT*(b+(xN-sf[215])))}else{(if ((xP)!=0.0){xQ}else{xK})});let y0=(wL/sf[175]);let y2=(if (y0<sf[215]){b}else{d});let y3=(y0).exp();let y5=(!((y2)!=0.0));let y6=(if y5{sf[216]}else{xT});let ya=(if y5{(y6*(b+(y0-sf[215])))}else{(if ((y2)!=0.0){y3}else{xX})});let yh=(if (uE&&sb[38]){b}else{d});let yi=(O*sC);let yl=(sf[766]*(b-(sf[22]/yi)));let yn=(if (yl<sf[215]){b}else{d});let ys=(((yh)!=0.0)&&(!((yn)!=0.0)));let yt=(if ys{sf[216]}else{y6});let yz=(if ((yh)!=0.0){(sf[576]*lz)}else{sf[797]});let yB=1e-30;let yD=(((yz*yz)+yB)).sqrt();let yG=f64::powf(yD,sf[250]);let yO=(h1*yz);let yP=(yz*yO);let yQ=(yz+sf[253]);let yS=((sf[20]*(sf[252]-((bR*yz)*sf[253])))-(yP*yQ));let yU=0.16666666666666666;let yY=(sf[766]*(sf[22]*lz));let yZ=(sf[433]*(if ((yh)!=0.0){((yG*yS)*yU)}else{d}));let z1=(if ((yh)!=0.0){(yY/yZ)}else{yz});let z2=-0.001;let z4=(if (z1<z2){b}else{d});let z6=(if (z1<sf[215]){b}else{d});let z7=(((yh)!=0.0)&&((z4)!=0.0));let zc=(z7&&(!((z6)!=0.0)));let zd=(if zc{sf[216]}else{yt});let zP=(if (sb[41]&&(lt<d)){b}else{d});let zQ=(sf[577]*lt);let zR=(b-zQ);let zT=(if ((zP)!=0.0){f64::powf(zR,sf[241])}else{d});let zU=(O*zT);let zX=(sf[786]*(b-(sf[54]/zU)));let zZ=(if (zX<sf[215]){b}else{d});let A4=(((zP)!=0.0)&&(!((zZ)!=0.0)));let A5=(if A4{sf[216]}else{zd});let Aa=(if ((zP)!=0.0){zQ}else{sf[777]});let Ad=((yB+(Aa*Aa))).sqrt();let Af=f64::powf(Ad,sf[254]);let An=(h1*Aa);let Ao=(Aa*An);let Ap=(Aa+sf[257]);let Ar=((sf[52]*(sf[256]-((bR*Aa)*sf[257])))-(Ao*Ap));let Aw=(sf[786]*(sf[54]*lt));let Ax=(sf[454]*(if ((zP)!=0.0){(yU*(Af*Ar))}else{d}));let Az=(if ((zP)!=0.0){(Aw/Ax)}else{Aa});let AB=(if (Az<z2){b}else{d});let AD=(if (Az<sf[215]){b}else{d});let AE=(((zP)!=0.0)&&((AB)!=0.0));let AJ=(AE&&(!((AD)!=0.0)));let AK=(if AJ{sf[216]}else{A5});let Bf=(mK*sf[883]);let Bg=(gB*(if nW{(nX*(b+(nR-sf[215])))}else{(if ((nT)!=0.0){nU}else{d})}));let Bh=(Bf-sf[883]);let Bj=((b+Bf)).sqrt();let Bk=(b+Bj);let Bn=((b+Bg)).sqrt();let Bo=(b+Bn);let CK=(n6-b);let CL=(sf[903]*CK);let CO=((b+(n6*sf[895]))).sqrt();let CP=(b+CO);let CR=(if ((sf[267])!=0.0){(CL/CP)}else{d});let CX=(sf[904]*(n6-ns));let D4=((b+(sf[906]*(n6+(ns*sf[262]))))).sqrt();let D5=(b+D4);let D9=(CK*sf[904]);let Dc=((b+(n6*sf[906]))).sqrt();let Dd=(b+Dc);let Df=(if sb[48]{(D9/Dd)}else{(if sb[47]{(CX/D5)}else{d})});let Dt=(if sb[50]{(m8-sf[915])}else{d});let Dx=(if sb[50]{(Dt*Dt)}else{uf});let Dz=(if (Dt<d){b}else{d});let DA=(sb[50]&&((Dz)!=0.0));let DD=((sf[272]+Dx)).sqrt();let DE=(DD-Dt);let DI=(sb[50]&&(!((Dz)!=0.0)));let DL=(if DI{(gp*(Dt+DD))}else{(if DA{(sf[273]/DE)}else{d})});let DP=(DL+(sf[910]+(sf[608]*(CR+Df))));let DU=(if sb[52]{b}else{(if sb[50]{(DL/DP)}else{b})});let EX=(if (tV<d){b}else{d});let EZ=((ue+(tV*tV))).sqrt();let F0=(EZ-tV);let F3=(!((EX)!=0.0));let F6=(if F3{(gp*(tV+EZ))}else{(if ((EX)!=0.0){(ui/F0)}else{d})});let Fi=(if (uB>d){b}else{d});let Fo=(if (lt<sf[295]){b}else{d});let Fr=((-uB)/sf[296]);
        let Ft=(if (Fr<sf[215]){b}else{d});let Fv=(((Fo)!=0.0)&&(((Fi)!=0.0)&&((sf[294])!=0.0)));let Fw=(((Ft)!=0.0)&&Fv);let Fx=(Fr).exp();let FA=(Fv&&(!((Ft)!=0.0)));let FB=(if FA{sf[216]}else{AK});let FF=(if FA{(FB*(b+(Fr-sf[215])))}else{(if Fw{Fx}else{d})});let FG=(sf[295]-lt);let FI=(if Fv{(FF*FG)}else{d});let FM=(sf[916]*f64::powf(FI,sf[297]));let FO=(if (FM<sf[215]){b}else{d});let FT=(Fv&&(!((FO)!=0.0)));let FU=(if FT{sf[216]}else{FB});let G9=(((Fi)!=0.0)&&sb[57]);let HS=(((Fo)!=0.0)&&(((sf[312])!=0.0)&&(G9&&sb[61])));let HT=f64::powf(FG,sf[297]);let HV=(uB+sf[313]);let HX=(b-(uB/HV));let HZ=f64::powf(HX,sf[314]);let I1=(if HS{(HT*HZ)}else{d});let I2=(((sf[306])!=0.0)&&HS);let I4=(sb[59]&&HS);let I8=(if I4{((uB-sf[315])/sf[313])}else{d});let Ic=(if I4{((I8-b)/sf[316])}else{v9});let Ie=(if (I8<b){b}else{d});let If=(I4&&((Ie)!=0.0));let Ig=(Ic).exp();let Ih=(b+Ig);let In=(I4&&(!((Ie)!=0.0)));let Ip=((-Ic)).exp();let Iq=(b+Ip);let Iu=(if In{(I8+(sf[316]*(Iq).ln()))}else{(if If{(b+(sf[316]*(Ih).ln()))}else{d})});let Iw=f64::powf(Iu,sf[317]);let Iz=(sf[916]*(if I4{(I1*Iw)}else{(if I2{I1}else{d})}));let IB=(if (Iz<sf[215]){b}else{d});let IG=(HS&&(!((IB)!=0.0)));let IH=(if IG{sf[216]}else{FU});let JH=((lC-sf[872])/sf[873]);let JJ=(if (lC<sf[872]){b}else{d});let JK=(JH).exp();let JL=(b+JK);let JQ=(!((JJ)!=0.0));let JS=((-JH)).exp();let JT=(b+JS);let JX=(if JQ{(sf[872]-(sf[873]*(JT).ln()))}else{(if ((JJ)!=0.0){(lC-(sf[873]*(JL).ln()))}else{d})});let K0=(b-(sf[576]*JX));let Kd=(tI*sf[924]);let Ke=(F6*Kd);let Kf=(tP*sf[924]);let Kg=(F6*Kf);let Ki=((m3-sf[880])/sf[869]);let Kk=(if (m3<sf[880]){b}else{d});let Kl=(Ki).exp();let Km=(b+Kl);let Kr=(!((Kk)!=0.0));let Kt=((-Ki)).exp();let Ku=(b+Kt);let Ky=(if Kr{(sf[880]-(sf[869]*(Ku).ln()))}else{(if ((Kk)!=0.0){(m3-(sf[869]*(Km).ln()))}else{d})});let KA=(b-(Ky/sf[535]));let KP=((m8-sf[880])/sf[869]);let KR=(if (m8<sf[880]){b}else{d});let KS=(KP).exp();let KT=(b+KS);let KY=(!((KR)!=0.0));let L0=((-KP)).exp();let L1=(b+L0);let L5=(if KY{(sf[880]-(sf[869]*(L1).ln()))}else{(if ((KR)!=0.0){(m8-(sf[869]*(KT).ln()))}else{d})});let L7=(b-(L5/sf[535]));let Lq=((lH-sf[926])/sf[925]);let Ls=(if (lH<sf[926]){b}else{d});let Lt=(Lq).exp();let Lu=(b+Lt);let Lz=(!((Ls)!=0.0));let LB=((-Lq)).exp();let LC=(b+LB);let LG=(if Lz{(sf[926]-(sf[925]*(LC).ln()))}else{(if ((Ls)!=0.0){(lH-(sf[925]*(Lu).ln()))}else{d})});let LK=(b-(LG/sf[575]));let LZ=(lz/sf[932]);let M1=(if (LZ<sf[215]){b}else{d});let M2=(LZ).exp();let M4=(!((M1)!=0.0));let M5=(if M4{sf[216]}else{IH});let Ma=(sf[931]*(if M4{(M5*(b+(LZ-sf[215])))}else{(if ((M1)!=0.0){M2}else{ya})}));let Mf=(s5*sf[936]);let Mg=(O+rU);let Mv=(sf[409]*((m3-sf[516])/sf[332]));let Mx=(if (Mv<sf[215]){b}else{d});let Mz=(((Mx)!=0.0)&&sb[66]);let MA=(Mv).exp();let MD=(sb[66]&&(!((Mx)!=0.0)));let ME=(if MD{sf[216]}else{M5});let MK=(mK*sf[938]);let MN=((b+(gB*(if MD{(ME*(b+(Mv-sf[215])))}else{(if Mz{MA}else{d})})))).sqrt();let MO=(b+MN);let MQ=(if sb[66]{(MK/MO)}else{(if ((sf[331])!=0.0){((sf[937]*(((Bh/Bk)*sf[923])+((Bg/Bo)*sf[935])))/sf[830])}else{d})});let MZ=(if sb[70]{(n6*sf[883])}else{d});let N0=(MZ-sf[883]);let N2=((b+MZ)).sqrt();let N3=(b+N2);let N7=(if sb[70]{(gB*(if nK{(nL*(b+(nF-sf[215])))}else{(if ((nH)!=0.0){nI}else{d})}))}else{d});let N9=((b+N7)).sqrt();let Na=(b+N9);let Nm=(sf[409]*(m8-sf[516]));let No=(if (Nm<sf[215]){b}else{d});let Nq=(((No)!=0.0)&&sb[71]);let Nr=(Nm).exp();let Nu=(sb[71]&&(!((No)!=0.0)));let Nv=(if Nu{sf[216]}else{ME});let NB=(n6*sf[940]);let NE=((b+(gB*(if Nu{(Nv*(b+(Nm-sf[215])))}else{(if Nq{Nr}else{d})})))).sqrt();let NF=(b+NE);let NH=(if sb[71]{(NB/NF)}else{(if sb[70]{((sf[939]*((sf[923]*(if sb[70]{(N0/N3)}else{d}))+(sf[935]*(if sb[70]{(N7/Na)}else{d}))))/sf[830])}else{d})});let NQ=(if ((sf[336])!=0.0){(f64::powf(sA,sf[337])-bR)}else{d});let NR=(if ((sf[336])!=0.0){si}else{d});let NT=(if (NR<d){b}else{d});let NU=(((sf[336])!=0.0)&&((NT)!=0.0));let NV=(NR).exp();let NW=(b+NV);let O0=(((sf[336])!=0.0)&&(!((NT)!=0.0)));let O2=((-NR)).exp();let O3=(b+O2);let O5=(if O0{(O2/O3)}else{(if NU{(b/NW)}else{d})});
        let Oc=((sf[409]*tE)/sf[639]);let Od=(gp/tG);let Of=(if ((sf[336])!=0.0){(Oc*Od)}else{d});let Og=(F6*sf[924]);let Ol=(lE*0.2);let On=((if ((sf[336])!=0.0){(Ma/sf[932])}else{d})+((if ((sf[336])!=0.0){(sf[920]*(if ((sf[336])!=0.0){(bR+(NQ*O5))}else{d}))}else{d})+(if ((sf[336])!=0.0){(Of*Og)}else{d})));let Ow=(if ((sf[336])!=0.0){(Ke+(Ma*sf[338]))}else{d});let OF=(if sb[73]{Ke}else{(if ((sf[336])!=0.0){(Ow*sf[341])}else{d})});let OG=(if sb[73]{Kg}else{(if ((sf[336])!=0.0){(Kg+(Ow*sf[340]))}else{d})});let OI=(uy+uz);let OJ=(OI/uv);let OT=(if (OJ>d){b}else{d});let OU=(OF+OG);let OX=(!((OT)!=0.0));let OY=(sf[826]*F6);let P0=(if OX{(uv*OY)}else{(if ((OT)!=0.0){(OU/OJ)}else{d})});let Pf=(if sb[81]{d}else{(if sb[79]{(P0*sf[347])}else{(if ((sf[345])!=0.0){(sf[340]*P0)}else{d})})});let Q5=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (sf[0]*((if sb[73]{Ma}else{(if ((sf[336])!=0.0){(Ma*sf[339])}else{d})})+((sI*sf[920])+OF)))) };let Q6=(sf[15]*Q5);let Q8=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (sf[0]*(sf[921]*((sf[874]*(b-f64::powf(K0,sf[235])))+(bR*(lC-JX)))))) };let Q9=(sf[15]*Q8);let Qb=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (sf[0]*((Mf*Mg)+((tB*sf[922])+OG)))) };let Qc=(sf[15]*Qb);let Qe=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (sf[0]*(sf[585]*((sf[927]*(b-f64::powf(LK,sf[327])))+(O*(lH-LG)))))) };let Qf=(sf[15]*Qe);let Qh=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (sf[0]*(if ((sf[336])!=0.0){(Ol*On)}else{d}))) };let Qi=(sf[15]*Qh);let Ql=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, ((sf[0]*(lN-lK))*sf[350])) };let Qm=(sf[15]*Ql);let Qp=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (lU*sf[351])) };let Qq=(sf[15]*Qp);let Qx=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, (sf[0]*((sf[6]*(sf[323]*(sf[591]*((sf[876]*((sf[881]*(b-f64::powf(L7,sf[241])))+(sf[877]*(m8-L5))))+(sf[592]*m8)))))+(if ((sf[333])!=0.0){(DU*NH)}else{d})))) };let Qy=(sf[15]*Qx);
        let QD=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, (sf[0]*((sf[7]*((sf[591]*((sf[876]*((sf[881]*(b-f64::powf(KA,sf[241])))+(sf[877]*(m3-Ky))))+(sf[592]*m3)))*sf[323]))+(if ((sf[333])!=0.0){(sf[7]*MQ)}else{MQ})))) };let QE=(sf[15]*QD);let QN=ctx.node_voltage(n[11]);let QO=if REACTIVE { 0.0 } else { eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, QN) };let QP=(Pf*QO);let R3=(if mh{(mj*sf[941])}else{(if ((me)!=0.0){(mf*sf[941])}else{d})});let R4=(if mh{(mj*sf[942])}else{(if ((me)!=0.0){(mf*sf[942])}else{d})});let Rd=(if mu{(mv*sf[943])}else{(if ((mr)!=0.0){(ms*sf[943])}else{d})});let Re=(if mu{(mv*sf[944])}else{(if ((mr)!=0.0){(ms*sf[944])}else{d})});let Rt=(if mF{(mG*sf[941])}else{(if ((mC)!=0.0){(mD*sf[941])}else{d})});let Ru=(if mF{(mG*sf[945])}else{(if ((mC)!=0.0){(mD*sf[945])}else{d})});let Rv=(if mF{(mG*sf[946])}else{(if ((mC)!=0.0){(mD*sf[946])}else{d})});let Rw=(if mF{(mG*sf[942])}else{(if ((mC)!=0.0){(mD*sf[942])}else{d})});let RS=(if n1{(n2*sf[945])}else{(if ((mY)!=0.0){(mZ*sf[945])}else{d})});let RT=(if n1{(n2*sf[947])}else{(if ((mY)!=0.0){(mZ*sf[947])}else{d})});let RU=(if n1{(n2*sf[946])}else{(if ((mY)!=0.0){(mZ*sf[946])}else{d})});let RV=(if n1{(n2*sf[942])}else{(if ((mY)!=0.0){(mZ*sf[942])}else{d})});let Sd=(if nn{(no*sf[941])}else{(if ((nk)!=0.0){(nl*sf[941])}else{d})});let Se=(if nn{(no*sf[946])}else{(if ((nk)!=0.0){(nl*sf[946])}else{d})});let Sf=(if nn{(no*sf[942])}else{(if ((nk)!=0.0){(nl*sf[942])}else{d})});let T4=(if o8{(o9*sf[941])}else{(if ((o5)!=0.0){(o6*sf[941])}else{d})});let T5=(if o8{(o9*sf[942])}else{(if ((o5)!=0.0){(o6*sf[942])}else{d})});let Tc=(if ok{(ol*sf[941])}else{(if ((oh)!=0.0){(oi*sf[941])}else{d})});let Td=(if ok{(ol*sf[942])}else{(if ((oh)!=0.0){(oi*sf[942])}else{d})});let Tg=(O*os);let Th=((gB*T4)/Tg);let Ti=((gB*T5)/Tg);let Tl=(O*ov);let Tm=((gB*Tc)/Tl);let Tn=((gB*Td)/Tl);let Tt=(ox*ox);let Tz=(if ((oB)!=0.0){d}else{(((ox*(O*Tc))-(ow*Tm))/Tt)});let TA=(if ((oB)!=0.0){d}else{(((ox*(O*Td))-(ow*Tn))/Tt)});let TR=(sf[408]*((Th-Tm)-((((ox*Th)-(oE*Tm))/Tt)/oF)));let TS=(sf[408]*((-Tn)-(((-(oE*Tn))/Tt)/oF)));let TT=(sf[408]*(Ti-((Ti/ox)/oF)));let TV=(sf[352]+TT);let TW=(TR/sf[615]);let TX=((sf[0]+TS)/sf[615]);let TY=(TV/sf[615]);let U8=(sf[615]*(gp*TW));let U9=(sf[615]*(gp*TX));let Ua=(sf[615]*(gp*TY));let Um=(if ((oM)!=0.0){((sf[862]*((sf[409]*U8)/p3))-(if oT{(sf[0]/oV)}else{(if oQ{sf[0]}else{d})}))}else{d});let Un=(if ((oM)!=0.0){((sf[862]*((sf[409]*U9)/p3))-(if oT{(sf[352]/oV)}else{(if oQ{sf[352]}else{d})}))}else{d});let Uo=(if ((oM)!=0.0){(sf[862]*((sf[409]*Ua)/p3))}else{d});let Up=(p8*Um);let Ur=(p8*Un);let Ut=(p8*Uo);let Uy=(O*pl);let Uz=((if ((oM)!=0.0){(Up+Up)}else{d})/Uy);let UA=((if ((oM)!=0.0){(Ur+Ur)}else{d})/Uy);let UB=((if ((oM)!=0.0){(Ut+Ut)}else{d})/Uy);let UH=(pm*pm);let UY=(if pq{(gp*(Um+Uz))}else{(if pi{((-(pj*(Uz-Um)))/UH)}else{d})});let UZ=(if pq{(gp*(Un+UA))}else{(if pi{((-(pj*(UA-Un)))/UH)}else{d})});let V0=(if pq{(gp*(Uo+UB))}else{(if pi{((-(pj*(UB-Uo)))/UH)}else{d})});let Vg=(pB*pB);let Vq=(if ((oM)!=0.0){(((pB*((px*UY)+(pt*UY)))-(py*(sf[219]*UY)))/Vg)}else{d});let Vr=(if ((oM)!=0.0){(((pB*((px*UZ)+(pt*UZ)))-(py*(sf[219]*UZ)))/Vg)}else{d});let Vs=(if ((oM)!=0.0){(((pB*((px*V0)+(pt*V0)))-(py*(sf[219]*V0)))/Vg)}else{d});let Vw=(pD*pD);let VG=(if ((oM)!=0.0){(((pD*TW)-(oK*Vq))/Vw)}else{d});let VH=(if ((oM)!=0.0){(((pD*TX)-(oK*Vr))/Vw)}else{d});let VI=(if ((oM)!=0.0){(((pD*TY)-(oK*Vs))/Vw)}else{d});let VM=(if ((oM)!=0.0){(VG/sf[221])}else{d});let VN=(if ((oM)!=0.0){(VH/sf[221])}else{d});let VO=(if ((oM)!=0.0){(VI/sf[221])}else{d});
        let Wm=(if ((oM)!=0.0){((if pU{(VG+(sf[221]*((pW*(-VM))/pX)))}else{(if pM{(sf[221]*((pN*VM)/pO))}else{d})})/sf[227])}else{d});let Wn=(if ((oM)!=0.0){((if pU{(VH+(sf[221]*((pW*(-VN))/pX)))}else{(if pM{(sf[221]*((pN*VN)/pO))}else{d})})/sf[227])}else{d});let Wo=(if ((oM)!=0.0){((if pU{(VI+(sf[221]*((pW*(-VO))/pX)))}else{(if pM{(sf[221]*((pN*VO)/pO))}else{d})})/sf[227])}else{d});let Ws=(if ((oM)!=0.0){(UY/sf[220])}else{d});let Wt=(if ((oM)!=0.0){(UZ/sf[220])}else{d});let Wu=(if ((oM)!=0.0){(V0/sf[220])}else{d});let WQ=(O*qi);let X9=(ql*ql);let Xj=(if ((oM)!=0.0){(((ql*(((qf*((qd*Ws)+(qc*(gB*Wm))))+(qe*Ws))/WQ))-(qj*((qk*Ws)+(qf*(O*Wm)))))/X9)}else{d});let Xk=(if ((oM)!=0.0){(((ql*(((qf*((qd*Wt)+(qc*(gB*Wn))))+(qe*Wt))/WQ))-(qj*((qk*Wt)+(qf*(O*Wn)))))/X9)}else{d});let Xl=(if ((oM)!=0.0){(((ql*(((qf*((qd*Wu)+(qc*(gB*Wo))))+(qe*Wu))/WQ))-(qj*((qk*Wu)+(qf*(O*Wo)))))/X9)}else{d});let Xr=((qn*Tz)+(oC*Xj));let Xu=((qn*TA)+(oC*Xk));let Xv=(oC*Xl);let XC=(qr*qr);let XM=(if ((oM)!=0.0){(((qr*((-Xj)+Xr))-(qq*Xr))/XC)}else{d});let XN=(if ((oM)!=0.0){(((qr*((-Xk)+Xu))-(qq*Xu))/XC)}else{d});let XO=(if ((oM)!=0.0){(((qr*((-Xl)+Xv))-(qq*Xv))/XC)}else{d});let Y1=(if ((oM)!=0.0){(sf[409]*((qt*U8)+(p1*XM)))}else{d});let Y2=(if ((oM)!=0.0){(sf[409]*((qt*U9)+(p1*XN)))}else{d});let Y3=(if ((oM)!=0.0){(sf[409]*((qt*Ua)+(p1*XO)))}else{d});let Yj=(if ((oM)!=0.0){((O*Y1)+((qz*Tz)+(oC*(Tz+Y1))))}else{d});let Yk=(if ((oM)!=0.0){((O*Y2)+((qz*TA)+(oC*(TA+Y2))))}else{d});let Yl=(if ((oM)!=0.0){((O*Y3)+(oC*Y3))}else{d});let Yp=(if ((oM)!=0.0){(gp*Y1)}else{d});let Yq=(if ((oM)!=0.0){(gp*Y2)}else{d});let Yr=(if ((oM)!=0.0){(gp*Y3)}else{d});let Ys=(qF*Yp);let Yu=(qF*Yq);let Yw=(qF*Yr);let YB=(if ((oM)!=0.0){(Yj+(Ys+Ys))}else{d});let YC=(if ((oM)!=0.0){(Yk+(Yu+Yu))}else{d});let YD=(if ((oM)!=0.0){(Yl+(Yw+Yw))}else{d});let YE=(O*qM);let YF=(YB/YE);let YG=(YC/YE);let YH=(YD/YE);let YU=(qR*qR);let Z7=(if qX{d}else{(if qQ{(((qR*Yj)-(qC*(YF-Yp)))/YU)}else{(if qL{(Yp+YF)}else{d})})});let Z8=(if qX{d}else{(if qQ{(((qR*Yk)-(qC*(YG-Yq)))/YU)}else{(if qL{(Yq+YG)}else{d})})});let Z9=(if qX{d}else{(if qQ{(((qR*Yl)-(qC*(YH-Yr)))/YU)}else{(if qL{(Yr+YH)}else{d})})});let Zs=(if ((oM)!=0.0){(sf[229]*TW)}else{d});let Zt=(if ((oM)!=0.0){(sf[229]*TX)}else{d});let Zu=(if ((oM)!=0.0){(sf[229]*TY)}else{d});let ZB=(r8*Zs);let ZD=(r8*Zt);let ZF=(r8*Zu);let ZK=(O*rf);let a03=(rr*rr);let a0j=(sf[218]*TW);let a0k=(sf[218]*TX);let a0l=(sf[218]*TY);let a0p=(rx*rx);let a0Q=(oE*oE);let a0Y=(if rC{(((oE*(O*T5))-(rD*Ti))/a0Q)}else{Z9});let a0Z=(if rC{R3}else{(if ((oM)!=0.0){(sf[866]*((qZ*Z7)+(qY*Z7)))}else{d})});let a10=(if rC{d}else{(if ((oM)!=0.0){(sf[866]*((qZ*Z8)+(qY*Z8)))}else{d})});let a11=(if rC{R4}else{(if ((oM)!=0.0){(sf[866]*((qZ*Z9)+(qY*Z9)))}else{d})});let a12=(Tz+(if rC{(((oE*(O*T4))-(rD*Th))/a0Q)}else{Z7}));let a13=(TA+(if rC{d}else{Z8}));let a17=(if rT{(gp*a12)}else{d});let a18=(if rT{(gp*a13)}else{d});let a19=(if rT{(gp*a0Y)}else{d});let a1d=(rX*rX);let a1w=(s3*s3);let a1G=(if s1{(((s3*TR)-(oI*((sf[0]+TR)-sf[0])))/a1w)}else{(if rT{(((rX*a17)-(rW*a17))/a1d)}else{XM})});let a1H=(if s1{(((s3*TS)-(oI*(TS-sf[352])))/a1w)}else{(if rT{(((rX*a18)-(rW*a18))/a1d)}else{XN})});let a1I=(if s1{(((s3*TT)-(oI*TV))/a1w)}else{(if rT{(((rX*a19)-(rW*a19))/a1d)}else{XO})});let a1M=(if rC{d}else{(if rp{(sf[535]*(((rr*(O*TW))-(rq*(TW+Vq)))/a03))}else{d})});let a1N=(if rC{d}else{(if rp{(sf[535]*(((rr*(O*TX))-(rq*(TX+Vr)))/a03))}else{d})});let a1O=(if rC{d}else{(if rp{(sf[535]*(((rr*(O*TY))-(rq*(TY+Vs)))/a03))}else{d})});let a1P=(if rC{TW}else{(if ((oM)!=0.0){(((rx*a0j)-(rw*TW))/a0p)}else{d})});let a1Q=(if rC{TX}else{(if ((oM)!=0.0){(((rx*a0k)-(rw*TX))/a0p)}else{d})});let a1R=(if rC{TY}else{(if ((oM)!=0.0){(((rx*a0l)-(rw*TY))/a0p)}else{d})});let a1Y=(if rC{(-(a1P/sf[218]))}else{(if ((oM)!=0.0){((-a0j)/a0p)}else{d})});let a1Z=(if rC{(-(a1Q/sf[218]))}else{(if ((oM)!=0.0){((-a0k)/a0p)}else{d})});let a20=(if rC{(-(a1R/sf[218]))}else{(if ((oM)!=0.0){((-a0l)/a0p)}else{d})});let a2n=(if sr{(-(sf[873]*((st*sf[950])/su)))}else{(if ((sk)!=0.0){(sf[352]-(sf[873]*((sl*sf[948])/sm)))}else{d})});
        let a2o=(if sr{(-(sf[873]*((st*sf[951])/su)))}else{(if ((sk)!=0.0){(sf[0]-(sf[873]*((sl*sf[949])/sm)))}else{d})});let a2r=(-(sf[576]*a2n));let a2s=(-(sf[576]*a2o));let a2v=(sf[235]*f64::powf(sA,sf[356]));let a2w=(a2r*a2v);let a2x=(a2s*a2v);let a2G=((sf[874]*(-a2w))+(bR*(sf[352]-a2n)));let a2H=((sf[874]*(-a2x))+(bR*(sf[0]-a2o)));let a2P=(if sb[28]{sf[0]}else{(if sb[26]{(sf[0]+(if rC{d}else{(if ((oM)!=0.0){(Zs+(((if ((oM)!=0.0){(sf[868]*TW)}else{d})+(ZB+ZB))/ZK))}else{d})}))}else{sf[357]})});let a2Q=(if sb[28]{d}else{(if sb[26]{(sf[352]+(if rC{sf[0]}else{(if ((oM)!=0.0){(Zt+(((if ((oM)!=0.0){(sf[868]*TX)}else{d})+(ZD+ZD))/ZK))}else{d})}))}else{sf[358]})});let a2R=(if sb[28]{sf[352]}else{(if sb[26]{(if rC{sf[352]}else{(if ((oM)!=0.0){(Zu+(((if ((oM)!=0.0){(sf[868]*TY)}else{d})+(ZF+ZF))/ZK))}else{d})})}else{d})});let a2V=(s7*s7);let a2W=(((s7*a2P)-(t3*a1M))/a2V);let a30=(((s7*a2Q)-(t3*a1N))/a2V);let a34=(((s7*a2R)-(t3*a1O))/a2V);let a3L=(if td{(-((th*a1M)+(s7*((tf*(-a2W))/tg))))}else{(if ((t6)!=0.0){(a2P-((t9*a1M)+(s7*((t7*a2W)/t8))))}else{d})});let a3M=(if td{(-((th*a1N)+(s7*((tf*(-a30))/tg))))}else{(if ((t6)!=0.0){(a2Q-((t9*a1N)+(s7*((t7*a30)/t8))))}else{d})});let a3N=(if td{(-((th*a1O)+(s7*((tf*(-a34))/tg))))}else{(if ((t6)!=0.0){(a2R-((t9*a1O)+(s7*((t7*a34)/t8))))}else{d})});let a3Q=(sf[240]*f64::powf(sb_,sf[359]));let a3R=(a1Y*a3Q);let a3S=(a1Z*a3Q);let a3T=(a20*a3Q);let a42=(sf[241]*f64::powf(tq,sf[360]));let a4F=(sf[876]*((sf[881]*(-((tr*a3T)+(tm*((-(a3N/sf[535]))*a42)))))+((tw*(sf[877]*a3T))+(tv*(a2R-a3N)))));let a4I=((sf[876]*((sf[881]*(-((tr*a3R)+(tm*((-(a3L/sf[535]))*a42)))))+((tw*(sf[877]*a3R))+(tv*(a2P-a3L)))))+sf[952]);let a4J=((sf[876]*((sf[881]*(-((tr*a3S)+(tm*((-(a3M/sf[535]))*a42)))))+((tw*(sf[877]*a3S))+(tv*(a2Q-a3M)))))+sf[953]);let a4K=(sf[883]*Rd);let a4L=(sf[883]*Re);let a4M=(O*tG);let a4N=(a4K/a4M);let a4O=(a4L/a4M);let a4S=(tH*tH);let a4T=(((tH*a4K)-(tE*a4N))/a4S);let a4X=(((tH*a4L)-(tE*a4O))/a4S);let a50=(sf[884]*f64::powf(rG,sf[954]));let a51=(a0Z*a50);let a52=(a10*a50);let a53=(a11*a50);let a54=(sf[883]*a51);let a55=(sf[883]*a52);let a56=(sf[883]*a53);let a57=(O*tN);let a5e=(tO*tO);let a5f=(((tO*a54)-(tL*(a54/a57)))/a5e);let a5j=(((tO*a55)-(tL*(a55/a57)))/a5e);let a5n=(((tO*a56)-(tL*(a56/a57)))/a5e);let a5o=(a2G/sf[801]);let a5p=(a2H/sf[801]);let a5q=(a4I/sf[799]);let a5r=(a4J/sf[799]);let a5s=(a4F/sf[799]);let a5t=(a5p+a5q);let a65=(if sb[30]{((u6*(if sb[30]{(sf[409]*(sf[846]*a5o))}else{d}))/sf[887])}else{(if ((sf[242])!=0.0){a5o}else{d})});let a66=(if sb[30]{(((u6*(if sb[30]{(sf[409]*(sf[846]*a5p))}else{d}))-(u7*(if sb[30]{(sf[409]*(sf[846]*((-a4I)/sf[799])))}else{d})))/sf[887])}else{(if ((sf[242])!=0.0){a5t}else{d})});let a67=(if sb[30]{((-(u7*(if sb[30]{(sf[409]*(sf[846]*((-a4J)/sf[799])))}else{d})))/sf[887])}else{(if ((sf[242])!=0.0){a5r}else{d})});let a68=(if sb[30]{((-(u7*(if sb[30]{(sf[409]*(sf[846]*((-a4F)/sf[799])))}else{d})))/sf[887])}else{(if ((sf[242])!=0.0){a5s}else{d})});let a69=(ud*a65);let a6a=(a69+a69);let a6b=(ud*a66);let a6c=(a6b+a6b);let a6d=(ud*a67);let a6e=(a6d+a6d);let a6f=(ud*a68);let a6g=(a6f+a6f);let a6h=(O*uk);let a6i=(a6a/a6h);let a6j=(a6c/a6h);let a6k=(a6e/a6h);let a6l=(a6g/a6h);let a6s=(ul*ul);let a6U=(gp*a4T);let a6V=(gp*(a4X+a5f));let a6W=(gp*a5j);let a6X=(gp*a5n);let a70=((uu*(if uo{(gp*(a65+a6i))}else{(if ((uh)!=0.0){((-(ui*(a6i-a65)))/a6s)}else{d})}))+(ur*a6U));let a73=((uu*(if uo{(gp*(a66+a6j))}else{(if ((uh)!=0.0){((-(ui*(a6j-a66)))/a6s)}else{d})}))+(ur*a6V));let a76=((uu*(if uo{(gp*(a67+a6k))}else{(if ((uh)!=0.0){((-(ui*(a6k-a67)))/a6s)}else{d})}))+(ur*a6W));let a79=((uu*(if uo{(gp*(a68+a6l))}else{(if ((uh)!=0.0){((-(ui*(a6l-a68)))/a6s)}else{d})}))+(ur*a6X));let a7a=(sf[888]*a51);let a7b=(sf[888]*a52);let a7c=(sf[888]*a53);let a7e=(sf[684]*Re);let a7i=(uv*(sf[684]*Rd));let a7l=(uv*uv);let a7T=(if uL{(sf[352]+(uC*((uN*sf[363])/uO)))}else{(if ((uF)!=0.0){(uC*((uG*sf[361])/uH))}else{d})});let a7U=(if uL{(sf[0]+(uC*((uN*sf[364])/uO)))}else{(if ((uF)!=0.0){(uC*((uG*sf[362])/uH))}else{d})});
        let a8Q=(if vA{(vB*sf[955])}else{(if ((vx)!=0.0){(vy*sf[955])}else{a7T})});let a8R=(if vA{(vB*sf[956])}else{(if ((vx)!=0.0){(vy*sf[956])}else{a7U})});let ab0=(if wR{(wS*sf[957])}else{(if ((wO)!=0.0){(wP*sf[957])}else{a8Q})});let ab1=(if wR{(wS*sf[958])}else{(if ((wO)!=0.0){(wP*sf[958])}else{d})});let ab2=(if wR{d}else{(if ((wO)!=0.0){d}else{a8R})});let abV=(if xs{(xt*sf[959])}else{(if ((xp)!=0.0){(xq*sf[959])}else{ab0})});let abW=(if xs{d}else{(if ((xp)!=0.0){d}else{ab1})});let abX=(if xs{(xt*sf[960])}else{(if ((xp)!=0.0){(xq*sf[960])}else{ab2})});let aca=(if xF{(xG*sf[961])}else{(if ((xC)!=0.0){(xD*sf[961])}else{abV})});let acb=(if xF{(xG*sf[962])}else{(if ((xC)!=0.0){(xD*sf[962])}else{abW})});let acc=(if xF{d}else{(if ((xC)!=0.0){d}else{abX})});let acx=(if xS{d}else{(if ((xP)!=0.0){d}else{aca})});let acy=(if xS{(xT*sf[963])}else{(if ((xP)!=0.0){(xQ*sf[963])}else{acb})});let acz=(if xS{(xT*sf[964])}else{(if ((xP)!=0.0){(xQ*sf[964])}else{acc})});let acA=(if xS{(xT*sf[965])}else{(if ((xP)!=0.0){(xQ*sf[965])}else{d})});let acB=(if xS{(xT*sf[966])}else{(if ((xP)!=0.0){(xQ*sf[966])}else{d})});let acS=(if y5{(y6*sf[967])}else{(if ((y2)!=0.0){(y3*sf[967])}else{acx})});let acT=(if y5{(y6*sf[968])}else{(if ((y2)!=0.0){(y3*sf[968])}else{acy})});let acU=(if y5{d}else{(if ((y2)!=0.0){d}else{acz})});let acV=(if y5{d}else{(if ((y2)!=0.0){d}else{acA})});let acW=(if y5{d}else{(if ((y2)!=0.0){d}else{acB})});let aio=(sf[883]*Rt);let aip=(sf[883]*Ru);let aiq=(sf[883]*Rv);let air=(sf[883]*Rw);let ais=(gB*(if nW{(nX*sf[941])}else{(if ((nT)!=0.0){(nU*sf[941])}else{d})}));let ait=(gB*(if nW{(nX*sf[945])}else{(if ((nT)!=0.0){(nU*sf[945])}else{d})}));let aiu=(gB*(if nW{(nX*sf[946])}else{(if ((nT)!=0.0){(nU*sf[946])}else{d})}));let aiv=(gB*(if nW{(nX*sf[942])}else{(if ((nT)!=0.0){(nU*sf[942])}else{d})}));let aiw=(O*Bj);let aiE=(Bk*Bk);let aiS=(O*Bn);let aj0=(Bo*Bo);let amM=(O*CO);let amU=(CP*CP);let an8=(if ((sf[267])!=0.0){(((CP*(sf[903]*RS))-(CL*((sf[895]*RS)/amM)))/amU)}else{d});let an9=(if ((sf[267])!=0.0){(((CP*(sf[903]*RT))-(CL*((sf[895]*RT)/amM)))/amU)}else{d});let ana=(if ((sf[267])!=0.0){(((CP*(sf[903]*RU))-(CL*((sf[895]*RU)/amM)))/amU)}else{d});let anb=(if ((sf[267])!=0.0){(((CP*(sf[903]*RV))-(CL*((sf[895]*RV)/amM)))/amU)}else{d});let anf=(sf[904]*RS);let ang=(sf[904]*RT);let anj=(sf[904]*RU);let anq=(sf[906]*RS);let anr=(sf[906]*RT);let anu=(sf[906]*RU);let anw=(O*D4);let anG=(D5*D5);let aoa=(O*Dc);let aoi=(Dd*Dd);let aor=(((Dd*anj)-(D9*(anu/aoa)))/aoi);let aow=(if sb[48]{(((Dd*anf)-(D9*(anq/aoa)))/aoi)}else{(if sb[47]{(((D5*anf)-(CX*(anq/anw)))/anG)}else{d})});let aox=(if sb[48]{(((Dd*ang)-(D9*(anr/aoa)))/aoi)}else{(if sb[47]{(((D5*ang)-(CX*(anr/anw)))/anG)}else{d})});let aoy=(if sb[48]{d}else{(if sb[47]{(((D5*(sf[904]*(-Sd)))-(CX*((sf[906]*(sf[262]*Sd))/anw)))/anG)}else{d})});let aoz=(if sb[48]{aor}else{(if sb[47]{(((D5*(sf[904]*(RU-Se)))-(CX*((sf[906]*(RU+(sf[262]*Se)))/anw)))/anG)}else{d})});let aoA=(if sb[48]{aor}else{(if sb[47]{(((D5*anj)-(CX*(anu/anw)))/anG)}else{d})});let aoB=(if sb[48]{(((Dd*(sf[904]*RV))-(D9*((sf[906]*RV)/aoa)))/aoi)}else{(if sb[47]{(((D5*(sf[904]*(RV-Sf)))-(CX*((sf[906]*(RV+(sf[262]*Sf)))/anw)))/anG)}else{d})});let aoG=(Dt*sf[377]);let aoH=(aoG+aoG);let aoI=(Dt*sf[378]);let aoK=(Dt*sf[379]);let aoL=(aoK+aoK);let aoM=(Dt*sf[380]);let aoO=(if sb[50]{aoH}else{d});let aoP=(if sb[50]{(aoI+aoI)}else{d});let aoQ=(if sb[50]{d}else{a6a});let aoR=(if sb[50]{aoH}else{a6c});let aoS=(if sb[50]{aoL}else{a6e});let aoT=(if sb[50]{aoL}else{a6g});let aoU=(if sb[50]{(aoM+aoM)}else{d});let aoV=(if sb[50]{aoL}else{d});let aoW=(O*DD);let aoX=(aoO/aoW);let aoY=(aoP/aoW);let aoZ=(aoQ/aoW);let ap0=(aoR/aoW);let ap1=(aoS/aoW);let ap2=(aoT/aoW);let ap3=(aoU/aoW);let ap4=(aoV/aoW);let ape=(DE*DE);let apY=(if DI{(gp*(sf[377]+aoX))}else{(if DA{((-(sf[273]*(aoX-sf[377])))/ape)}else{d})});let apZ=(if DI{(gp*(sf[378]+aoY))}else{(if DA{((-(sf[273]*(aoY-sf[378])))/ape)}else{d})});let aq0=(if DI{(gp*aoZ)}else{(if DA{((-(sf[273]*aoZ))/ape)}else{d})});
        let aq1=(if DI{(gp*(sf[377]+ap0))}else{(if DA{((-(sf[273]*(ap0-sf[377])))/ape)}else{d})});let aq2=(if DI{(gp*(sf[379]+ap1))}else{(if DA{((-(sf[273]*(ap1-sf[379])))/ape)}else{d})});let aq3=(if DI{(gp*(sf[379]+ap2))}else{(if DA{((-(sf[273]*(ap2-sf[379])))/ape)}else{d})});let aq4=(if DI{(gp*(sf[380]+ap3))}else{(if DA{((-(sf[273]*(ap3-sf[380])))/ape)}else{d})});let aq5=(if DI{(gp*(sf[379]+ap4))}else{(if DA{((-(sf[273]*(ap4-sf[379])))/ape)}else{d})});let aqb=(sf[608]*(an8+aow));let aqe=(sf[608]*(ana+aoz));let aqr=(DP*DP);let ar7=(if sb[52]{d}else{(if sb[50]{(((DP*apY)-(DL*(apY+aqb)))/aqr)}else{d})});let ar8=(if sb[52]{d}else{(if sb[50]{(((DP*apZ)-(DL*(apZ+(sf[608]*(an9+aox)))))/aqr)}else{d})});let ar9=(if sb[52]{d}else{(if sb[50]{((-(DL*(sf[608]*aoy)))/aqr)}else{d})});let ara=(if sb[52]{d}else{(if sb[50]{(((DP*aq0)-(DL*aq0))/aqr)}else{d})});let arb=(if sb[52]{d}else{(if sb[50]{(((DP*aq1)-(DL*(aq1+aqb)))/aqr)}else{d})});let arc=(if sb[52]{d}else{(if sb[50]{(((DP*aq2)-(DL*(aq2+aqe)))/aqr)}else{d})});let ard=(if sb[52]{d}else{(if sb[50]{(((DP*aq3)-(DL*(aq3+(sf[608]*(ana+aoA)))))/aqr)}else{d})});let are=(if sb[52]{d}else{(if sb[50]{(((DP*aq4)-(DL*(aq4+(sf[608]*(anb+aoB)))))/aqr)}else{d})});let arf=(if sb[52]{d}else{(if sb[50]{(((DP*aq5)-(DL*(aq5+aqe)))/aqr)}else{d})});let aw3=(tV*a5o);let aw5=(tV*a5t);let aw7=(tV*a5r);let aw9=(tV*a5s);let awb=(O*EZ);let awc=((aw3+aw3)/awb);let awd=((aw5+aw5)/awb);let awe=((aw7+aw7)/awb);let awf=((aw9+aw9)/awb);let awm=(F0*F0);let awJ=(if F3{(gp*(a5o+awc))}else{(if ((EX)!=0.0){((-(ui*(awc-a5o)))/awm)}else{d})});let awK=(if F3{(gp*(a5t+awd))}else{(if ((EX)!=0.0){((-(ui*(awd-a5t)))/awm)}else{d})});let awL=(if F3{(gp*(a5r+awe))}else{(if ((EX)!=0.0){((-(ui*(awe-a5r)))/awm)}else{d})});let awM=(if F3{(gp*(a5s+awf))}else{(if ((EX)!=0.0){((-(ui*(awf-a5s)))/awm)}else{d})});let aKW=(if JQ{(-(sf[873]*((JS*sf[950])/JT)))}else{(if ((JJ)!=0.0){(sf[352]-(sf[873]*((JK*sf[948])/JL)))}else{d})});let aKX=(if JQ{(-(sf[873]*((JS*sf[951])/JT)))}else{(if ((JJ)!=0.0){(sf[0]-(sf[873]*((JK*sf[949])/JL)))}else{d})});let aL3=(sf[235]*f64::powf(K0,sf[356]));let aLp=((Kd*awJ)+(F6*(sf[924]*a4T)));let aLs=((Kd*awK)+(F6*(sf[924]*a4X)));let aLt=(Kd*awL);let aLu=(Kd*awM);let aLy=(Kf*awJ);let aLB=((Kf*awK)+(F6*(sf[924]*a5f)));let aLE=((Kf*awL)+(F6*(sf[924]*a5j)));let aLH=((Kf*awM)+(F6*(sf[924]*a5n)));let aMq=(if Kr{(-(sf[869]*((Kt*sf[985])/Ku)))}else{(if ((Kk)!=0.0){(sf[0]-(sf[869]*((Kl*sf[981])/Km)))}else{d})});let aMr=(if Kr{(-(sf[869]*((Kt*sf[986])/Ku)))}else{(if ((Kk)!=0.0){(sf[353]-(sf[869]*((Kl*sf[982])/Km)))}else{d})});let aMs=(if Kr{(-(sf[869]*((Kt*sf[987])/Ku)))}else{(if ((Kk)!=0.0){(sf[354]-(sf[869]*((Kl*sf[983])/Km)))}else{d})});let aMt=(if Kr{(-(sf[869]*((Kt*sf[988])/Ku)))}else{(if ((Kk)!=0.0){(sf[352]-(sf[869]*((Kl*sf[984])/Km)))}else{d})});let aMD=(sf[241]*f64::powf(KA,sf[360]));let aO0=(if KY{(-(sf[869]*((L0*sf[986])/L1)))}else{(if ((KR)!=0.0){(sf[353]-(sf[869]*((KS*sf[982])/KT)))}else{d})});let aO1=(if KY{(-(sf[869]*((L0*sf[992])/L1)))}else{(if ((KR)!=0.0){(sf[355]-(sf[869]*((KS*sf[991])/KT)))}else{d})});let aO2=(if KY{(-(sf[869]*((L0*sf[987])/L1)))}else{(if ((KR)!=0.0){(sf[354]-(sf[869]*((KS*sf[983])/KT)))}else{d})});let aO3=(if KY{(-(sf[869]*((L0*sf[988])/L1)))}else{(if ((KR)!=0.0){(sf[352]-(sf[869]*((KS*sf[984])/KT)))}else{d})});let aOd=(sf[241]*f64::powf(L7,sf[360]));let aOT=(sf[6]*(sf[323]*(sf[591]*(sf[989]+(sf[876]*((sf[881]*(-((-(aO0/sf[535]))*aOd)))+(sf[877]*(sf[353]-aO0))))))));let aOV=(sf[6]*(sf[323]*(sf[591]*(sf[990]+(sf[876]*((sf[881]*(-((-(aO2/sf[535]))*aOd)))+(sf[877]*(sf[354]-aO2))))))));let aPj=(if Lz{(-(sf[925]*((LB*sf[996])/LC)))}else{(if ((Ls)!=0.0){(sf[0]-(sf[925]*((Lt*sf[994])/Lu)))}else{d})});let aPk=(if Lz{(-(sf[925]*((LB*sf[997])/LC)))}else{(if ((Ls)!=0.0){(sf[352]-(sf[925]*((Lt*sf[995])/Lu)))}else{d})});let aPr=(sf[327]*f64::powf(LK,sf[390]));let aPW=(sf[931]*(if M4{(M5*sf[998])}else{(if ((M1)!=0.0){(M2*sf[998])}else{acS})}));let aPX=(sf[931]*(if M4{d}else{(if ((M1)!=0.0){d}else{acT})}));let aPY=(sf[931]*(if M4{(M5*sf[999])}else{(if ((M1)!=0.0){(M2*sf[999])}else{acU})}));
        let aPZ=(sf[931]*(if M4{d}else{(if ((M1)!=0.0){d}else{acV})}));let aQ0=(sf[931]*(if M4{d}else{(if ((M1)!=0.0){d}else{acW})}));let aR7=(O*MN);let aRf=(MO*MO);let aRt=(if sb[66]{(((MO*(sf[938]*Rt))-(MK*((gB*(if MD{(ME*sf[1000])}else{(if Mz{(MA*sf[1000])}else{d})}))/aR7)))/aRf)}else{(if ((sf[331])!=0.0){((sf[937]*((sf[923]*(((Bk*aio)-(Bh*(aio/aiw)))/aiE))+(sf[935]*(((Bo*ais)-(Bg*(ais/aiS)))/aj0))))/sf[830])}else{d})});let aRu=(if sb[66]{(((MO*(sf[938]*Ru))-(MK*((gB*(if MD{(ME*sf[1001])}else{(if Mz{(MA*sf[1001])}else{d})}))/aR7)))/aRf)}else{(if ((sf[331])!=0.0){((sf[937]*((sf[923]*(((Bk*aip)-(Bh*(aip/aiw)))/aiE))+(sf[935]*(((Bo*ait)-(Bg*(ait/aiS)))/aj0))))/sf[830])}else{d})});let aRv=(if sb[66]{(((MO*(sf[938]*Rv))-(MK*((gB*(if MD{(ME*sf[1002])}else{(if Mz{(MA*sf[1002])}else{d})}))/aR7)))/aRf)}else{(if ((sf[331])!=0.0){((sf[937]*((sf[923]*(((Bk*aiq)-(Bh*(aiq/aiw)))/aiE))+(sf[935]*(((Bo*aiu)-(Bg*(aiu/aiS)))/aj0))))/sf[830])}else{d})});let aRw=(if sb[66]{(((MO*(sf[938]*Rw))-(MK*((gB*(if MD{(ME*sf[1003])}else{(if Mz{(MA*sf[1003])}else{d})}))/aR7)))/aRf)}else{(if ((sf[331])!=0.0){((sf[937]*((sf[923]*(((Bk*air)-(Bh*(air/aiw)))/aiE))+(sf[935]*(((Bo*aiv)-(Bg*(aiv/aiS)))/aj0))))/sf[830])}else{d})});let aRJ=(if sb[70]{(sf[883]*RS)}else{d});let aRK=(if sb[70]{(sf[883]*RT)}else{d});let aRL=(if sb[70]{(sf[883]*RU)}else{d});let aRM=(if sb[70]{(sf[883]*RV)}else{d});let aRN=(O*N2);let aRV=(N3*N3);let aSh=(if sb[70]{(gB*(if nK{(nL*sf[945])}else{(if ((nH)!=0.0){(nI*sf[945])}else{d})}))}else{d});let aSi=(if sb[70]{(gB*(if nK{(nL*sf[947])}else{(if ((nH)!=0.0){(nI*sf[947])}else{d})}))}else{d});let aSj=(if sb[70]{(gB*(if nK{(nL*sf[946])}else{(if ((nH)!=0.0){(nI*sf[946])}else{d})}))}else{d});let aSk=(if sb[70]{(gB*(if nK{(nL*sf[942])}else{(if ((nH)!=0.0){(nI*sf[942])}else{d})}))}else{d});let aSl=(O*N9);let aSt=(Na*Na);let aTx=(O*NE);let aTF=(NF*NF);let aTY=(DU*(if sb[71]{(((NF*(sf[940]*RS))-(NB*((gB*(if Nu{(Nv*sf[945])}else{(if Nq{(Nr*sf[945])}else{d})}))/aTx)))/aTF)}else{(if sb[70]{((sf[939]*((sf[923]*(if sb[70]{(((N3*aRJ)-(N0*(aRJ/aRN)))/aRV)}else{d}))+(sf[935]*(if sb[70]{(((Na*aSh)-(N7*(aSh/aSl)))/aSt)}else{d}))))/sf[830])}else{d})}));let aU8=(DU*(if sb[71]{(((NF*(sf[940]*RU))-(NB*((gB*(if Nu{(Nv*sf[946])}else{(if Nq{(Nr*sf[946])}else{d})}))/aTx)))/aTF)}else{(if sb[70]{((sf[939]*((sf[923]*(if sb[70]{(((N3*aRL)-(N0*(aRL/aRN)))/aRV)}else{d}))+(sf[935]*(if sb[70]{(((Na*aSj)-(N7*(aSj/aSl)))/aSt)}else{d}))))/sf[830])}else{d})}));let aUs=(sf[337]*f64::powf(sA,sf[395]));let aUC=(NW*NW);let aUK=(O2*sf[1006]);let aUL=(O2*sf[1007]);let aUP=(O3*O3);let aVf=(tG*tG);let aVQ=(if ((sf[336])!=0.0){(aPZ/sf[932])}else{d});let aWt=(sf[338]*aPZ);let aWz=(if ((sf[336])!=0.0){(aLp+(sf[338]*aPW))}else{d});let aWA=(if ((sf[336])!=0.0){(sf[338]*aPX)}else{d});let aWB=(if ((sf[336])!=0.0){(aLs+(sf[338]*aPY))}else{d});let aWC=(if ((sf[336])!=0.0){(aLt+aWt)}else{d});let aWD=(if ((sf[336])!=0.0){(aLu+aWt)}else{d});let aWE=(if ((sf[336])!=0.0){(sf[338]*aQ0)}else{d});let aX7=(if sb[73]{aLp}else{(if ((sf[336])!=0.0){(sf[341]*aWz)}else{d})});let aX8=(if sb[73]{d}else{(if ((sf[336])!=0.0){(sf[341]*aWA)}else{d})});let aX9=(if sb[73]{aLs}else{(if ((sf[336])!=0.0){(sf[341]*aWB)}else{d})});let aXa=(if sb[73]{aLt}else{(if ((sf[336])!=0.0){(sf[341]*aWC)}else{d})});let aXb=(if sb[73]{aLu}else{(if ((sf[336])!=0.0){(sf[341]*aWD)}else{d})});let aXc=(if sb[73]{d}else{(if ((sf[336])!=0.0){(sf[341]*aWE)}else{d})});let aXd=(if sb[73]{aLy}else{(if ((sf[336])!=0.0){(aLy+(sf[340]*aWz))}else{d})});let aXe=(if sb[73]{d}else{(if ((sf[336])!=0.0){(sf[340]*aWA)}else{d})});let aXf=(if sb[73]{aLB}else{(if ((sf[336])!=0.0){(aLB+(sf[340]*aWB))}else{d})});let aXg=(if sb[73]{aLE}else{(if ((sf[336])!=0.0){(aLE+(sf[340]*aWC))}else{d})});let aXh=(if sb[73]{aLH}else{(if ((sf[336])!=0.0){(aLH+(sf[340]*aWD))}else{d})});let aXi=(if sb[73]{d}else{(if ((sf[336])!=0.0){(sf[340]*aWE)}else{d})});let aXm=(if sb[73]{aPZ}else{(if ((sf[336])!=0.0){(sf[339]*aPZ)}else{d})});let aXE=(OJ*OJ);let aYp=(if OX{((OY*a70)+(uv*(sf[826]*awJ)))}else{(if ((OT)!=0.0){(((OJ*(aX7+aXd))-(OU*((a7i-(OI*a70))/a7l)))/aXE)}else{d})});
        let aYq=(if OX{d}else{(if ((OT)!=0.0){((aX8+aXe)/OJ)}else{d})});let aYr=(if OX{((OY*a73)+(uv*(sf[826]*awK)))}else{(if ((OT)!=0.0){(((OJ*(aX9+aXf))-(OU*(((uv*(a7a+a7e))-(OI*a73))/a7l)))/aXE)}else{d})});let aYs=(if OX{((OY*a76)+(uv*(sf[826]*awL)))}else{(if ((OT)!=0.0){(((OJ*(aXa+aXg))-(OU*(((uv*a7b)-(OI*a76))/a7l)))/aXE)}else{d})});let aYt=(if OX{((OY*a79)+(uv*(sf[826]*awM)))}else{(if ((OT)!=0.0){(((OJ*(aXb+aXh))-(OU*(((uv*a7c)-(OI*a79))/a7l)))/aXE)}else{d})});let aYu=(if OX{d}else{(if ((OT)!=0.0){((aXc+aXi)/OJ)}else{d})});let b2q=(if REACTIVE { 1.0 } else { ddt_scale });let b2x=(sf[15]*((sf[0]*((if sb[73]{aPW}else{(if ((sf[336])!=0.0){(sf[339]*aPW)}else{d})})+((sf[920]*a2G)+aX7)))*b2q));let b2y=(sf[15]*((sf[0]*(aX8+(if sb[73]{aPX}else{(if ((sf[336])!=0.0){(sf[339]*aPX)}else{d})})))*b2q));let b2z=(sf[15]*((sf[0]*((if sb[73]{aPY}else{(if ((sf[336])!=0.0){(sf[339]*aPY)}else{d})})+((sf[920]*a2H)+aX9)))*b2q));let b2A=(sf[15]*((sf[0]*(aXa+aXm))*b2q));let b2B=(sf[15]*((sf[0]*(aXb+aXm))*b2q));let b2C=(sf[15]*((sf[0]*(aXc+(if sb[73]{aQ0}else{(if ((sf[336])!=0.0){(sf[339]*aQ0)}else{d})})))*b2q));let b2H=(sf[15]*(b2q*(sf[0]*(sf[921]*((sf[874]*(-((-(sf[576]*aKW))*aL3)))+(bR*(sf[352]-aKW)))))));let b2I=(sf[15]*(b2q*(sf[0]*(sf[921]*((sf[874]*(-((-(sf[576]*aKX))*aL3)))+(bR*(sf[0]-aKX)))))));let b2V=(sf[15]*(b2q*(sf[0]*aXd)));let b2W=(sf[15]*(b2q*(sf[0]*aXe)));let b2X=(sf[15]*(b2q*(sf[0]*(((Mg*(sf[936]*a1G))+(Mf*a12))+((sf[922]*a4I)+aXf)))));let b2Y=(sf[15]*(b2q*(sf[0]*(((Mg*(sf[936]*a1H))+(Mf*a13))+((sf[922]*a4J)+aXg)))));let b2Z=(sf[15]*(b2q*(sf[0]*(((Mg*(sf[936]*a1I))+(Mf*a0Y))+((sf[922]*a4F)+aXh)))));let b30=(sf[15]*(b2q*(sf[0]*aXi)));let b35=(sf[15]*(b2q*(sf[0]*(sf[585]*((sf[927]*(-((-(aPj/sf[575]))*aPr)))+(O*(sf[0]-aPj)))))));let b36=(sf[15]*(b2q*(sf[0]*(sf[585]*((sf[927]*(-((-(aPk/sf[575]))*aPr)))+(O*(sf[352]-aPk)))))));let b3j=(sf[15]*(b2q*(sf[0]*(if ((sf[336])!=0.0){(Ol*((if ((sf[336])!=0.0){(aPW/sf[932])}else{d})+((if ((sf[336])!=0.0){(sf[920]*(if ((sf[336])!=0.0){((O5*(if ((sf[336])!=0.0){(a2r*aUs)}else{d}))+(NQ*(if O0{(((O3*aUK)-(O2*aUK))/aUP)}else{(if NU{((-(NV*sf[1004]))/aUC)}else{d})})))}else{d}))}else{d})+(if ((sf[336])!=0.0){((Og*(if ((sf[336])!=0.0){((Od*((sf[409]*a4K)/sf[639]))+(Oc*((-(gp*a4N))/aVf)))}else{d}))+(Of*(sf[924]*awJ)))}else{d}))))}else{d}))));let b3k=(sf[15]*(b2q*(sf[0]*(if ((sf[336])!=0.0){((On*sf[396])+(Ol*(if ((sf[336])!=0.0){(aPX/sf[932])}else{d})))}else{d}))));let b3l=(sf[15]*(b2q*(sf[0]*(if ((sf[336])!=0.0){((On*sf[397])+(Ol*((if ((sf[336])!=0.0){(aPY/sf[932])}else{d})+((if ((sf[336])!=0.0){(sf[920]*(if ((sf[336])!=0.0){((O5*(if ((sf[336])!=0.0){(a2s*aUs)}else{d}))+(NQ*(if O0{(((O3*aUL)-(O2*aUL))/aUP)}else{(if NU{((-(NV*sf[1005]))/aUC)}else{d})})))}else{d}))}else{d})+(if ((sf[336])!=0.0){((Og*(if ((sf[336])!=0.0){((Od*((sf[409]*a4L)/sf[639]))+(Oc*((-(gp*a4O))/aVf)))}else{d}))+(Of*(sf[924]*awK)))}else{d})))))}else{d}))));let b3m=(sf[15]*(b2q*(sf[0]*(if ((sf[336])!=0.0){(Ol*((if ((sf[336])!=0.0){(Of*(sf[924]*awL))}else{d})+aVQ))}else{d}))));let b3n=(sf[15]*(b2q*(sf[0]*(if ((sf[336])!=0.0){(Ol*((if ((sf[336])!=0.0){(Of*(sf[924]*awM))}else{d})+aVQ))}else{d}))));let b3o=(sf[15]*(b2q*(sf[0]*(if ((sf[336])!=0.0){(Ol*(if ((sf[336])!=0.0){(aQ0/sf[932])}else{d}))}else{d}))));let b3t=(sf[15]*(b2q*sf[400]));let b3u=(sf[15]*(b2q*sf[401]));let b3z=(sf[15]*(b2q*sf[402]));let b3A=(sf[15]*(b2q*sf[403]));let b4n=(sf[15]*(b2q*(sf[0]*(aOT+(if ((sf[333])!=0.0){((NH*ar7)+aTY)}else{d})))));let b4o=(sf[15]*(b2q*(sf[0]*((sf[6]*(sf[323]*(sf[591]*((sf[876]*((sf[881]*(-((-(aO1/sf[535]))*aOd)))+(sf[877]*(sf[355]-aO1))))+sf[993]))))+(if ((sf[333])!=0.0){((NH*ar8)+(DU*(if sb[71]{(((NF*(sf[940]*RT))-(NB*((gB*(if Nu{(Nv*sf[947])}else{(if Nq{(Nr*sf[947])}else{d})}))/aTx)))/aTF)}else{(if sb[70]{((sf[939]*((sf[923]*(if sb[70]{(((N3*aRK)-(N0*(aRK/aRN)))/aRV)}else{d}))+(sf[935]*(if sb[70]{(((Na*aSi)-(N7*(aSi/aSl)))/aSt)}else{d}))))/sf[830])}else{d})})))}else{d})))));let b4p=(sf[15]*(b2q*(sf[0]*(if ((sf[333])!=0.0){(NH*ar9)}else{d}))));let b4q=(sf[15]*(b2q*(sf[0]*(if ((sf[333])!=0.0){(NH*ara)}else{d}))));
        let b4r=(sf[15]*(b2q*(sf[0]*(aOT+(if ((sf[333])!=0.0){(aTY+(NH*arb))}else{d})))));let b4s=(sf[15]*(b2q*(sf[0]*(aOV+(if ((sf[333])!=0.0){((NH*arc)+aU8)}else{d})))));let b4t=(sf[15]*(b2q*(sf[0]*(aOV+(if ((sf[333])!=0.0){(aU8+(NH*ard))}else{d})))));let b4u=(sf[15]*(b2q*(sf[0]*((sf[6]*(sf[323]*(sf[591]*(sf[953]+(sf[876]*((sf[881]*(-((-(aO3/sf[535]))*aOd)))+(sf[877]*(sf[352]-aO3))))))))+(if ((sf[333])!=0.0){((NH*are)+(DU*(if sb[71]{(((NF*(sf[940]*RV))-(NB*((gB*(if Nu{(Nv*sf[942])}else{(if Nq{(Nr*sf[942])}else{d})}))/aTx)))/aTF)}else{(if sb[70]{((sf[939]*((sf[923]*(if sb[70]{(((N3*aRM)-(N0*(aRM/aRN)))/aRV)}else{d}))+(sf[935]*(if sb[70]{(((Na*aSk)-(N7*(aSk/aSl)))/aSt)}else{d}))))/sf[830])}else{d})})))}else{d})))));let b4v=(sf[15]*(b2q*(sf[0]*(aOV+(if ((sf[333])!=0.0){(aU8+(NH*arf))}else{d})))));let b51=(sf[15]*(b2q*(sf[0]*((sf[7]*(sf[323]*(sf[591]*(sf[952]+(sf[876]*((sf[881]*(-((-(aMq/sf[535]))*aMD)))+(sf[877]*(sf[0]-aMq))))))))+(if ((sf[333])!=0.0){(sf[7]*aRt)}else{aRt})))));let b52=(sf[15]*(b2q*(sf[0]*((sf[7]*(sf[323]*(sf[591]*((sf[876]*((sf[881]*(-((-(aMr/sf[535]))*aMD)))+(sf[877]*(sf[353]-aMr))))+sf[989]))))+(if ((sf[333])!=0.0){(sf[7]*aRu)}else{aRu})))));let b53=(sf[15]*(b2q*(sf[0]*((sf[7]*(sf[323]*(sf[591]*((sf[876]*((sf[881]*(-((-(aMs/sf[535]))*aMD)))+(sf[877]*(sf[354]-aMs))))+sf[990]))))+(if ((sf[333])!=0.0){(sf[7]*aRv)}else{aRv})))));
        let b54=(sf[15]*(b2q*(sf[0]*((sf[7]*(sf[323]*(sf[591]*(sf[953]+(sf[876]*((sf[881]*(-((-(aMt/sf[535]))*aMD)))+(sf[877]*(sf[352]-aMt))))))))+(if ((sf[333])!=0.0){(sf[7]*aRw)}else{aRw})))));let b5h=(QO*(if sb[81]{d}else{(if sb[79]{(sf[347]*aYp)}else{(if ((sf[345])!=0.0){(sf[340]*aYp)}else{d})})}));let b5i=(QO*(if sb[81]{d}else{(if sb[79]{(sf[347]*aYq)}else{(if ((sf[345])!=0.0){(sf[340]*aYq)}else{d})})}));let b5j=(QO*(if sb[81]{d}else{(if sb[79]{(sf[347]*aYr)}else{(if ((sf[345])!=0.0){(sf[340]*aYr)}else{d})})}));let b5k=(QO*(if sb[81]{d}else{(if sb[79]{(sf[347]*aYs)}else{(if ((sf[345])!=0.0){(sf[340]*aYs)}else{d})})}));let b5l=(QO*(if sb[81]{d}else{(if sb[79]{(sf[347]*aYt)}else{(if ((sf[345])!=0.0){(sf[340]*aYt)}else{d})})}));let b5m=(QO*(if sb[81]{d}else{(if sb[79]{(sf[347]*aYu)}else{(if ((sf[345])!=0.0){(sf[340]*aYu)}else{d})})}));let b5n=(Pf*b2q);

        CommonStampValues {
            b, d, N, O, a3, bR, gl, gp,
            gB, h1, lt, lx, lz, lE, lH, lK,
            lP, lX, m0, m3, m7, mn, mK, mL,
            mN, mQ, mR, n7, n9, nc, nd, nt,
            nv, ny, nz, oK, qI, rG, s5, s8,
            sb_, sC, tU, uu, uv, uA, uB, uU,
            uW, uZ, v0, v9, vF, vH, vJ, vO,
            vP, vW, vX, vZ, w4, w6, wW, wY,
            x0, x5, x6, xx, xK, xX, ya, yh,
            yi, yl, yn, ys, yt, yz, yD, yG,
            yO, yP, yQ, yS, yU, yY, yZ, z1,
            z4, z6, z7, zc, zd, zP, zR, zT,
            zU, zX, zZ, A4, A5, Aa, Ad, Af,
            An, Ao, Ap, Ar, Aw, Ax, Az, AB,
            AD, AE, AJ, AK, CR, Df, Dx, DU,
            F6, Fi, Fv, Fw, Fx, FA, FB, FF,
            FG, FI, FM, FO, FT, FU, G9, HS,
            HT, HV, HX, HZ, I1, I2, I4, Ic,
            If, Ig, Ih, In, Ip, Iq, Iu, Iw,
            Iz, IB, IG, IH, OJ, Q6, Q9, Qc,
            Qf, Qi, Qm, Qq, Qy, QE, QN, QP,
            R3, R4, Rt, Ru, Rv, Rw, TW, TX,
            TY, YB, YC, YD, a0Z, a10, a11, a1G,
            a1H, a1I, a1P, a1Q, a1R, a1Y, a1Z, a20,
            a2w, a2x, a5q, a5r, a5s, a6U, a6V, a6W,
            a6X, a70, a73, a76, a79, a7a, a7b, a7c,
            a7e, a7i, a7l, a7T, a7U, a8Q, a8R, ab0,
            ab1, ab2, abV, abW, abX, aca, acb, acc,
            acx, acy, acz, acA, acB, acS, acT, acU,
            acV, acW, an8, an9, ana, anb, aow, aox,
            aoy, aoz, aoA, aoB, aoO, aoP, aoQ, aoR,
            aoS, aoT, aoU, aoV, ar7, ar8, ar9, ara,
            arb, arc, ard, are, arf, awJ, awK, awL,
            awM, b2x, b2y, b2z, b2A, b2B, b2C, b2H,
            b2I, b2V, b2W, b2X, b2Y, b2Z, b30, b35,
            b36, b3j, b3k, b3l, b3m, b3n, b3o, b3t,
            b3u, b3z, b3A, b4n, b4o, b4p, b4q, b4r,
            b4s, b4t, b4u, b4v, b51, b52, b53, b54,
            b5h, b5i, b5j, b5k, b5l, b5m, b5n,
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let CommonStampValues {
            b, d, N, O, a3, bR, gl, gp,
            gB, h1, lt, lx, lz, lE, lH, lK,
            lP, lX, m0, m3, m7, mn, mK, mL,
            mN, mQ, mR, n7, n9, nc, nd, nt,
            nv, ny, nz, oK, qI, rG, s5, s8,
            sb_, sC, tU, uu, uv, uA, uB, uU,
            uW, uZ, v0, v9, vF, vH, vJ, vO,
            vP, vW, vX, vZ, w4, w6, wW, wY,
            x0, x5, x6, xx, xK, xX, ya, yh,
            yi, yl, yn, ys, yt, yz, yD, yG,
            yO, yP, yQ, yS, yU, yY, yZ, z1,
            z4, z6, z7, zc, zd, zP, zR, zT,
            zU, zX, zZ, A4, A5, Aa, Ad, Af,
            An, Ao, Ap, Ar, Aw, Ax, Az, AB,
            AD, AE, AJ, AK, CR, Df, Dx, DU,
            F6, Fi, Fv, Fw, Fx, FA, FB, FF,
            FG, FI, FM, FO, FT, FU, G9, HS,
            HT, HV, HX, HZ, I1, I2, I4, Ic,
            If, Ig, Ih, In, Ip, Iq, Iu, Iw,
            Iz, IB, IG, IH, OJ, Q6, Q9, Qc,
            Qf, Qi, Qm, Qq, Qy, QE, QN, QP,
            R3, R4, Rt, Ru, Rv, Rw, TW, TX,
            TY, YB, YC, YD, a0Z, a10, a11, a1G,
            a1H, a1I, a1P, a1Q, a1R, a1Y, a1Z, a20,
            a2w, a2x, a5q, a5r, a5s, a6U, a6V, a6W,
            a6X, a70, a73, a76, a79, a7a, a7b, a7c,
            a7e, a7i, a7l, a7T, a7U, a8Q, a8R, ab0,
            ab1, ab2, abV, abW, abX, aca, acb, acc,
            acx, acy, acz, acA, acB, acS, acT, acU,
            acV, acW, an8, an9, ana, anb, aow, aox,
            aoy, aoz, aoA, aoB, aoO, aoP, aoQ, aoR,
            aoS, aoT, aoU, aoV, ar7, ar8, ar9, ara,
            arb, arc, ard, are, arf, awJ, awK, awL,
            awM, b2x, b2y, b2z, b2A, b2B, b2C, b2H,
            b2I, b2V, b2W, b2X, b2Y, b2Z, b30, b35,
            b36, b3j, b3k, b3l, b3m, b3n, b3o, b3t,
            b3u, b3z, b3A, b4n, b4o, b4p, b4q, b4r,
            b4s, b4t, b4u, b4v, b51, b52, b53, b54,
            b5h, b5i, b5j, b5k, b5l, b5m, b5n,
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
        let w=ctx.simparam_or("gmin", d);let M=(if sb[5]{d}else{(if ((sf[19])!=0.0){w}else{d})});let mO=(mL).exp();let na=(n7).exp();let nh=(if nc{(nd*(b+(n7-sf[215])))}else{(if ((n9)!=0.0){na}else{d})});let nw=(nt).exp();let nD=(if ny{(nz*(b+(nt-sf[215])))}else{(if ((nv)!=0.0){nw}else{d})});let uX=(uU).exp();let v4=(if uZ{(v0*(b+(uU-sf[215])))}else{(if ((uW)!=0.0){uX}else{d})});let vb=(if (lz<sf[245]){b}else{d});let vc=(v9).exp();let vd=(b+vc);let vi=(!((vb)!=0.0));let vk=((-v9)).exp();let vl=(b+vk);let vp=(if vi{(sf[245]-(N*(vl).ln()))}else{(if ((vb)!=0.0){(lz-(N*(vd).ln()))}else{d})});let vr=(vp*sf[246]);let vs=(sf[245]-vp);let vt={let pb=vs;pb*pb};let vK=(((sf[155])!=0.0)&&((vJ)!=0.0));let vL=(vH).exp();let vT=(if vO{(vP*(b+(vH-sf[215])))}else{(if vK{vL}else{uU})});let w0=(((sf[155])!=0.0)&&((vZ)!=0.0));let w1=(vW).exp();let wa=(if w4{(w6*(b+(vW-vX)))}else{(if w0{w1}else{v4})});let wb=(vF-b);let wc=(sf[712]*wb);let we=(wb*sf[889]);let wh=((b+(gB*vT))).sqrt();let wi=(b+wh);let wj=(we/wi);let wk=(b+tU);let wo=(sf[727]*(rG-b));let wp=(wa*wo);let wq=(b+wa);let wG=(sf[247]*((rG+vF)-O));let x1=(((sf[155])!=0.0)&&((x0)!=0.0));let x2=(wY).exp();let xb=(wW-b);let xc=(sf[718]*xb);let xe=(xb*sf[890]);let xh=((b+(gB*(if x5{(x6*(b+(wY-sf[215])))}else{(if x1{x2}else{vT})})))).sqrt();let xi=(b+xh);let xZ=(sf[704]*(xX-b));let yo=(((yh)!=0.0)&&((yn)!=0.0));let yp=(yl).exp();let yx=(if ys{(yt*(b+(yl-sf[215])))}else{(if yo{yp}else{d})});let z8=(((z6)!=0.0)&&z7);let z9=(z1).exp();let zi=(-lz);let zj=(b-(if zc{(zd*(b+(z1-sf[215])))}else{(if z8{z9}else{d})}));let zl=(b+(zj/z1));let zp=(((yh)!=0.0)&&(!((z4)!=0.0)));let zq=(gp*lz);let zr=(z1*zq);let zs=0.3333333333333333;let zt=(z1*zs);let zu=0.25;let zw=(b+(z1*zu));let zy=(b+(zt*zw));let zC=((if zp{(zr*zy)}else{(if z7{(zi*zl)}else{d})})*sf[891]);let zD=(sC*zC);let zI=(!((yh)!=0.0));let A0=(((zP)!=0.0)&&((zZ)!=0.0));let A1=(zX).exp();let A9=(if A4{(A5*(b+(zX-sf[215])))}else{(if A0{A1}else{d})});let AF=(((AD)!=0.0)&&AE);let AG=(Az).exp();let AP=(-lt);let AQ=(b-(if AJ{(AK*(b+(Az-sf[215])))}else{(if AF{AG}else{d})}));let AS=(b+(AQ/Az));let AW=(((zP)!=0.0)&&(!((AB)!=0.0)));let AX=(gp*lt);let AY=(Az*AX);let AZ=(zs*Az);let B1=(b+(zu*Az));let B3=(b+(AZ*B1));let B7=((if AW{(AY*B3)}else{(if AE{(AP*AS)}else{d})})*sf[892]);let B8=(zT*B7);let Bd=(!((zP)!=0.0));let Be=(if Bd{d}else{(if ((zP)!=0.0){(sf[55]*(sf[577]*(A9*B8)))}else{d})});let Br=(mK-b);let Bs=(sf[893]*Br);let Bx=((b+(mK*sf[895]))).sqrt();let By=(b+Bx);let Bz=(Bs/By);let BH=(sf[896]*(mn-nh));let BP=((b+(sf[898]*(mn+(nh*sf[262]))))).sqrt();let BQ=(b+BP);let BX=(sf[899]*(mK-nD));let C2=((b+(sf[898]*(mK+(nD*sf[262]))))).sqrt();let C3=(b+C2);let C8=(sf[896]*(mn-b));let Cb=((b+(mn*sf[898]))).sqrt();let Cc=(b+Cb);let Cf=(Br*sf[899]);let Ci=((b+(mK*sf[898]))).sqrt();let Cj=(b+Ci);let Cl=(if sb[43]{(Cf/Cj)}else{(if ((sf[259])!=0.0){(BX/C3)}else{d})});let Co=(sf[900]*(nh-b));let Cu=((b+(nh*sf[902]))).sqrt();let Cv=(b+Cu);let CF=(if ((sf[267])!=0.0){(sf[7]*Bz)}else{Bz});let DW=(if ((sf[267])!=0.0){(CR*DU)}else{d});let E3=(if ((sf[275])!=0.0){(lt+lE)}else{d});let E5=(-E3);let E9=(if (E5<d){b}else{d});let Ea=(((sf[275])!=0.0)&&((E9)!=0.0));let Ed=((sf[276]+(if ((sf[275])!=0.0){(E3*E3)}else{Dx}))).sqrt();let Ee=(Ed-E5);let Ei=(((sf[275])!=0.0)&&(!((E9)!=0.0)));let El=(if Ei{(gp*(E5+Ed))}else{(if Ea{(sf[277]/Ee)}else{d})});let EC=(if (El<sf[285]){b}else{d});let ED=(((sf[275])!=0.0)&&((EC)!=0.0));let EE=(El/sf[283]);let EG=(b-f64::powf(EE,sf[278]));let EK=(((sf[275])!=0.0)&&(!((EC)!=0.0)));let EQ=(if sb[54]{b}else{(if EK{(sf[282]+(sf[292]*(El-sf[285])))}else{(if ED{(b/EG)}else{d})})});let F7=(uu*F6);let F8=(sf[600]/F7);let Fa=(if (F8<sf[16]){b}else{d});let Fc=(bR*(if ((Fa)!=0.0){sf[16]}else{F8}));let Ff=(lE+(sf[862]*((if mQ{(mR*(b+(mL-sf[215])))}else{(if ((mN)!=0.0){mO}else{d})})-b)));let FP=(Fv&&((FO)!=0.0));let FQ=(FM).exp();let FY=(if FT{(FU*(b+(FM-sf[215])))}else{(if FP{FQ}else{d})});let G1=(FI*sf[917]);let Gb=((((if (lt<sf[497]){b}else{d}))!=0.0)&&(((sf[299])!=0.0)&&G9));let Gh=(if Gb{sf[304]}else{d});let Gi=(sf[497]-lt);
        let Gk=(if Gb{(Gi/sb_)}else{qI});let Gn=(((O*Gk)/Gh)).sqrt();let Go=(if Gb{Gn}else{d});let Gs=(Gb&&((sf[306])!=0.0));let Gv=(Gb&&sb[59]);let Gy=(if Gv{(b-(gp*s5))}else{d});let Gz=(sf[302]*Gy);let GB=(if Gv{(Gy*Gz)}else{(if Gs{sf[302]}else{d})});let GC=(Go*GB);let GG=(((Go*Go)+(GB*GB))).sqrt();let GI=(if Gb{(GC/GG)}else{d});let GK=(if Gb{(Gi/GI)}else{d});let GL=(gp*GI);let GM=(Gh*GL);let GP=(if Gb{(GK+(sb_*GM))}else{d});let H2=(sf[218]*(if Gv{(b+(sf[308]*(b+(O*s5))))}else{d}));let H4=((if Gv{sf[311]}else{d})-(uB/H2));let H7=(if Gv{(GK-(GM*H4))}else{d});let H8=(H7-GP);let Ha=(a3*GK);let Hb=(GK*Ha);let Hh=((if Gv{((H8*H8)+((s8*Hb)/sf[218]))}else{Gk})).sqrt();let Hk=(if Gv{(gp*((GP+H7)+Hh))}else{(if Gs{GP}else{d})});let Hl=(Hk-GK);let Hn=(if Gb{(Hl/Hk)}else{d});let Hr=(if ((Hn).abs()>1e-7){b}else{d});let Hs=(Gb&&((Hr)!=0.0));let Hu=(if Hs{(GL/Hn)}else{d});let Hw=(Hk*sf[918]);let Hx=(Hu*Hw);let Hz=(sf[919]/Hk);let HA=(Hz).exp();let HC=(b+(GB/Hu));let HE=((Hz*HC)).exp();let HF=(HA-HE);let HJ=(Gb&&(!((Hr)!=0.0)));let HK=(sf[4]*GB);let IC=(HS&&((IB)!=0.0));let ID=(Iz).exp();let IL=(if IG{(IH*(b+(Iz-sf[215])))}else{(if IC{ID}else{FY})});let IM=(FG*sf[917]);let IO=(if HS{(IL*IM)}else{(if HJ{(HA*HK)}else{(if Hs{(Hx*HF)}else{(if Fv{(FY*G1)}else{d})})})});let IU=(((Fi)!=0.0)&&(((if (IO>d){b}else{d}))!=0.0));let IV=(((sf[319])!=0.0)&&IU);let IW=(sf[605]+Fc);let IX=(uB*IW);let J4=(if IV{(((sf[408]/IX)+(sf[712]*(uv/sf[684])))+(sf[597]/IW))}else{d});let J5=(((sf[312])!=0.0)&&IV);let J8=(if J5{((IO-J4)/gl)}else{Ic});let Ja=(if (IO<J4){b}else{d});let Jb=(J5&&((Ja)!=0.0));let Jc=(J8).exp();let Jd=(b+Jc);let Jj=(J5&&(!((Ja)!=0.0)));let Jl=((-J8)).exp();let Jm=(b+Jl);let Jq=(if Jj{(J4-(gl*(Jm).ln()))}else{(if Jb{(IO-(gl*(Jd).ln()))}else{IO})});let Jr=(uB*Jq);let Ju=(IV&&sb[63]);let Jv=(J4*Jr);let Jw=(J4+Jq);let JA=(IU&&sb[64]);let JB=(if JA{Jr}else{(if Ju{(Jv/Jw)}else{(if J5{Jr}else{d})})});let OR=(if sb[75]{d}else{(if ((sf[343])!=0.0){((JB/OJ)).abs()}else{d})});let PJ=(sf[15]*(sf[0]*(-(Be*EQ))));let S2=(if nc{(nd*sf[941])}else{(if ((n9)!=0.0){(na*sf[941])}else{d})});let S3=(if nc{(nd*sf[942])}else{(if ((n9)!=0.0){(na*sf[942])}else{d})});let Sp=(if ny{(nz*sf[941])}else{(if ((nv)!=0.0){(nw*sf[941])}else{d})});let Sq=(if ny{(nz*sf[946])}else{(if ((nv)!=0.0){(nw*sf[946])}else{d})});let Sr=(if ny{(nz*sf[942])}else{(if ((nv)!=0.0){(nw*sf[942])}else{d})});let a7m=((a7i-(uA*a70))/a7l);let a7q=(((uv*(a7e-a7a))-(uA*a73))/a7l);let a7u=(((uv*(-a7b))-(uA*a76))/a7l);let a7y=(((uv*(-a7c))-(uA*a79))/a7l);let a7V=(a7T/sf[244]);let a7W=(a7U/sf[244]);let a83=(if uZ{(v0*a7V)}else{(if ((uW)!=0.0){(uX*a7V)}else{d})});let a84=(if uZ{(v0*a7W)}else{(if ((uW)!=0.0){(uX*a7W)}else{d})});let a8t=(if vi{(-(N*((vk*sf[367])/vl)))}else{(if ((vb)!=0.0){(sf[352]-(N*((vc*sf[365])/vd)))}else{d})});let a8u=(if vi{(-(N*((vk*sf[368])/vl)))}else{(if ((vb)!=0.0){(sf[0]-(N*((vc*sf[366])/vd)))}else{d})});let a8z=(O*vs);let a8Y=(if vO{(vP*sf[942])}else{(if vK{(vL*sf[942])}else{a7V})});let a8Z=(if vO{(vP*sf[941])}else{(if vK{(vL*sf[941])}else{a7W})});let a90=(a7m/sf[684]);let a91=(a7q/sf[684]);let a92=(a7u/sf[684]);let a93=(a7y/sf[684]);let a9g=(if w4{(w6*a90)}else{(if w0{(w1*a90)}else{a83})});let a9h=(if w4{(w6*a91)}else{(if w0{(w1*a91)}else{a84})});let a9i=(if w4{(w6*a92)}else{(if w0{(w1*a92)}else{d})});let a9j=(if w4{(w6*a93)}else{(if w0{(w1*a93)}else{d})});let a9k=(sf[712]*a8Q);let a9l=(sf[712]*a8R);let a9q=(O*wh);let a9w=(wi*wi);let aa0=(wq*wq);let abd=(sf[718]*ab0);let abe=(sf[718]*ab1);let abf=(sf[718]*ab2);let abm=(O*xh);let abt=(xi*xi);let ad6=(yi*yi);let add=(sf[766]*(-((-(sf[22]*(O*a2w)))/ad6)));let ade=(sf[766]*(-((-(sf[22]*(O*a2x)))/ad6)));let adp=(if ((yh)!=0.0){sf[969]}else{d});let adq=(if ((yh)!=0.0){sf[970]}else{d});let adr=(yz*adp);let adt=(yz*adq);let adv=(O*yD);let adA=(sf[250]*f64::powf(yD,sf[369]));let aek=(yZ*yZ);let aeq=(if ((yh)!=0.0){(((yZ*sf[971])-(yY*(sf[433]*(if ((yh)!=0.0){(yU*((yS*(((adr+adr)/adv)*adA))+(yG*((sf[20]*(-(sf[253]*(bR*adp))))-((yQ*((yO*adp)+(yz*(h1*adp))))+(yP*adp))))))}else{d}))))/aek)}else{adp});
        let aer=(if ((yh)!=0.0){(((yZ*sf[972])-(yY*(sf[433]*(if ((yh)!=0.0){(yU*((yS*(((adt+adt)/adv)*adA))+(yG*((sf[20]*(-(sf[253]*(bR*adq))))-((yQ*((yO*adq)+(yz*(h1*adq))))+(yP*adq))))))}else{d}))))/aek)}else{adq});let aeF=(z1*z1);let afK=(sf[241]*f64::powf(zR,sf[360]));let afN=(if ((zP)!=0.0){(sf[975]*afK)}else{d});let afO=(if ((zP)!=0.0){(sf[976]*afK)}else{d});let afT=(zU*zU);let ag0=(sf[786]*(-((-(sf[54]*(O*afN)))/afT)));let ag1=(sf[786]*(-((-(sf[54]*(O*afO)))/afT)));let aga=(if ((zP)!=0.0){sf[973]}else{d});let agb=(if ((zP)!=0.0){sf[974]}else{d});let agc=(Aa*aga);let age=(Aa*agb);let agg=(O*Ad);let agl=(sf[254]*f64::powf(Ad,sf[374]));let ah5=(Ax*Ax);let ahb=(if ((zP)!=0.0){(((Ax*sf[977])-(Aw*(sf[454]*(if ((zP)!=0.0){(yU*((Ar*(((agc+agc)/agg)*agl))+(Af*((sf[52]*(-(sf[257]*(bR*aga))))-((Ap*((An*aga)+(Aa*(h1*aga))))+(Ao*aga))))))}else{d}))))/ah5)}else{aga});let ahc=(if ((zP)!=0.0){(((Ax*sf[978])-(Aw*(sf[454]*(if ((zP)!=0.0){(yU*((Ar*(((age+age)/agg)*agl))+(Af*((sf[52]*(-(sf[257]*(bR*agb))))-((Ap*((An*agb)+(Aa*(h1*agb))))+(Ao*agb))))))}else{d}))))/ah5)}else{agb});let ahq=(Az*Az);let ajm=(O*Bx);let aju=(By*By);let ajv=(((By*(sf[893]*Rt))-(Bs*((sf[895]*Rt)/ajm)))/aju);let ajz=(((By*(sf[893]*Ru))-(Bs*((sf[895]*Ru)/ajm)))/aju);let ajD=(((By*(sf[893]*Rv))-(Bs*((sf[895]*Rv)/ajm)))/aju);let ajH=(((By*(sf[893]*Rw))-(Bs*((sf[895]*Rw)/ajm)))/aju);let ajL=(sf[896]*R3);let ajN=(sf[896]*R4);let ajR=(sf[898]*R3);let ajT=(sf[898]*R4);let ajU=(O*BP);let ak2=(BQ*BQ);let ako=(sf[899]*Rt);let akp=(sf[899]*Ru);let akr=(sf[899]*Rv);let akz=(sf[898]*Rt);let akA=(sf[898]*Ru);let akC=(sf[898]*Rv);let akE=(O*C2);let akO=(C3*C3);let alg=(O*Cb);let alm=(Cc*Cc);let aly=(O*Ci);let alG=(Cj*Cj);let alP=(((Cj*akr)-(Cf*(akC/aly)))/alG);let alU=(if sb[43]{d}else{(if ((sf[259])!=0.0){(((C3*(sf[899]*(-Sp)))-(BX*((sf[898]*(sf[262]*Sp))/akE)))/akO)}else{d})});let alV=(if sb[43]{(((Cj*ako)-(Cf*(akz/aly)))/alG)}else{(if ((sf[259])!=0.0){(((C3*ako)-(BX*(akz/akE)))/akO)}else{d})});let alW=(if sb[43]{(((Cj*akp)-(Cf*(akA/aly)))/alG)}else{(if ((sf[259])!=0.0){(((C3*akp)-(BX*(akA/akE)))/akO)}else{d})});let alX=(if sb[43]{alP}else{(if ((sf[259])!=0.0){(((C3*(sf[899]*(Rv-Sq)))-(BX*((sf[898]*(Rv+(sf[262]*Sq)))/akE)))/akO)}else{d})});let alY=(if sb[43]{alP}else{(if ((sf[259])!=0.0){(((C3*akr)-(BX*(akC/akE)))/akO)}else{d})});let alZ=(if sb[43]{(((Cj*(sf[899]*Rw))-(Cf*((sf[898]*Rw)/aly)))/alG)}else{(if ((sf[259])!=0.0){(((C3*(sf[899]*(Rw-Sr)))-(BX*((sf[898]*(Rw+(sf[262]*Sr)))/akE)))/akO)}else{d})});let am4=(O*Cu);let ama=(Cv*Cv);let arg=(DU*an8);let arq=(DU*ana);let arJ=(DU*aow);let arV=(DU*aoz);let asl=(E3*sf[381]);let asn=(E3*sf[382]);let asp=(E3*sf[383]);let asA=(O*Ed);let asB=((if ((sf[275])!=0.0){d}else{aoO})/asA);let asC=((if ((sf[275])!=0.0){d}else{aoP})/asA);let asD=((if ((sf[275])!=0.0){d}else{aoQ})/asA);let asE=((if ((sf[275])!=0.0){(asl+asl)}else{aoO})/asA);let asF=((if ((sf[275])!=0.0){(asn+asn)}else{aoR})/asA);let asG=((if ((sf[275])!=0.0){(asp+asp)}else{aoS})/asA);let asH=((if ((sf[275])!=0.0){d}else{aoT})/asA);let asI=((if ((sf[275])!=0.0){d}else{aoU})/asA);let asJ=((if ((sf[275])!=0.0){d}else{aoV})/asA);let asP=(Ee*Ee);let atA=(if Ei{(gp*asB)}else{(if Ea{((-(sf[277]*asB))/asP)}else{d})});let atB=(if Ei{(gp*asC)}else{(if Ea{((-(sf[277]*asC))/asP)}else{d})});let atC=(if Ei{(gp*asD)}else{(if Ea{((-(sf[277]*asD))/asP)}else{d})});let atD=(if Ei{(gp*(sf[384]+asE))}else{(if Ea{((-(sf[277]*(asE-sf[384])))/asP)}else{d})});let atE=(if Ei{(gp*(sf[385]+asF))}else{(if Ea{((-(sf[277]*(asF-sf[385])))/asP)}else{d})});let atF=(if Ei{(gp*(sf[386]+asG))}else{(if Ea{((-(sf[277]*(asG-sf[386])))/asP)}else{d})});let atG=(if Ei{(gp*asH)}else{(if Ea{((-(sf[277]*asH))/asP)}else{d})});let atH=(if Ei{(gp*asI)}else{(if Ea{((-(sf[277]*asI))/asP)}else{d})});let atI=(if Ei{(gp*asJ)}else{(if Ea{((-(sf[277]*asJ))/asP)}else{d})});let atT=(sf[278]*f64::powf(EE,sf[287]));let au3=(EG*EG);let auE=(if sb[54]{d}else{(if EK{(sf[292]*atA)}else{(if ED{(((atA/sf[283])*atT)/au3)}else{d})})});let auF=(if sb[54]{d}else{(if EK{(sf[292]*atB)}else{(if ED{(((atB/sf[283])*atT)/au3)}else{d})})});
        let auG=(if sb[54]{d}else{(if EK{(sf[292]*atC)}else{(if ED{(((atC/sf[283])*atT)/au3)}else{d})})});let auH=(if sb[54]{d}else{(if EK{(sf[292]*atD)}else{(if ED{(((atD/sf[283])*atT)/au3)}else{d})})});let auI=(if sb[54]{d}else{(if EK{(sf[292]*atE)}else{(if ED{(((atE/sf[283])*atT)/au3)}else{d})})});let auJ=(if sb[54]{d}else{(if EK{(sf[292]*atF)}else{(if ED{(((atF/sf[283])*atT)/au3)}else{d})})});let auK=(if sb[54]{d}else{(if EK{(sf[292]*atG)}else{(if ED{(((atG/sf[283])*atT)/au3)}else{d})})});let auL=(if sb[54]{d}else{(if EK{(sf[292]*atH)}else{(if ED{(((atH/sf[283])*atT)/au3)}else{d})})});let auM=(if sb[54]{d}else{(if EK{(sf[292]*atI)}else{(if ED{(((atI/sf[283])*atT)/au3)}else{d})})});let av9=(EQ*(if ((sf[267])!=0.0){(sf[7]*ajD)}else{ajD}));let avt=(EQ*(sf[704]*acA));let avC=(EQ*(if ((sf[267])!=0.0){(arg+(CR*ar7))}else{d}));let ax1=(F7*F7);let axg=(bR*(if ((Fa)!=0.0){d}else{((-(sf[600]*((F6*a6U)+(uu*awJ))))/ax1)}));let axh=(bR*(if ((Fa)!=0.0){d}else{((-(sf[600]*((F6*a6V)+(uu*awK))))/ax1)}));let axi=(bR*(if ((Fa)!=0.0){d}else{((-(sf[600]*((F6*a6W)+(uu*awL))))/ax1)}));let axj=(bR*(if ((Fa)!=0.0){d}else{((-(sf[600]*((F6*a6X)+(uu*awM))))/ax1)}));let axq=(Fc*Fc);let axH=((-a7m)/sf[296]);let axI=((-a7q)/sf[296]);let axJ=((-a7u)/sf[296]);let axK=((-a7y)/sf[296]);let ay9=(if Fv{(FG*(if FA{(FB*axH)}else{(if Fw{(Fx*axH)}else{d})}))}else{d});let aya=(if Fv{((FG*(if FA{(FB*axI)}else{(if Fw{(Fx*axI)}else{d})}))+(FF*sf[352]))}else{d});let ayb=(if Fv{((FG*(if FA{(FB*axJ)}else{(if Fw{(Fx*axJ)}else{d})}))+(sf[0]*FF))}else{d});let ayc=(if Fv{(FG*(if FA{(FB*axK)}else{(if Fw{(Fx*axK)}else{d})}))}else{d});let ayf=(sf[297]*f64::powf(FI,sf[387]));let ayk=(sf[916]*(ay9*ayf));let ayl=(sf[916]*(aya*ayf));let aym=(sf[916]*(ayb*ayf));let ayn=(sf[916]*(ayc*ayf));let ayA=(if FT{(FU*ayk)}else{(if FP{(FQ*ayk)}else{d})});let ayB=(if FT{(FU*ayl)}else{(if FP{(FQ*ayl)}else{d})});let ayC=(if FT{(FU*aym)}else{(if FP{(FQ*aym)}else{d})});let ayD=(if FT{(FU*ayn)}else{(if FP{(FQ*ayn)}else{d})});let az1=(sb_*sb_);let aza=(if Gb{(((sb_*sf[352])-(Gi*a1Y))/az1)}else{YB});let azb=(if Gb{(((sf[0]*sb_)-(Gi*a1Z))/az1)}else{YC});let azc=(if Gb{((-(Gi*a20))/az1)}else{YD});let azj=(O*Gn);let azn=(if Gb{(((O*aza)/Gh)/azj)}else{d});let azo=(if Gb{(((O*azb)/Gh)/azj)}else{d});let azp=(if Gb{(((O*azc)/Gh)/azj)}else{d});let azw=(if Gv{(-(gp*a1G))}else{d});let azx=(if Gv{(-(gp*a1H))}else{d});let azy=(if Gv{(-(gp*a1I))}else{d});let azL=(if Gv{((Gz*azw)+(Gy*(sf[302]*azw)))}else{d});let azM=(if Gv{((Gz*azx)+(Gy*(sf[302]*azx)))}else{d});let azN=(if Gv{((Gz*azy)+(Gy*(sf[302]*azy)))}else{d});let azX=(Go*azn);let azZ=(Go*azo);let aA1=(Go*azp);let aA3=(GB*azL);let aA5=(GB*azM);let aA7=(GB*azN);let aAc=(O*GG);let aAj=(GG*GG);let aAt=(if Gb{(((GG*((GB*azn)+(Go*azL)))-(GC*(((azX+azX)+(aA3+aA3))/aAc)))/aAj)}else{d});let aAu=(if Gb{(((GG*((GB*azo)+(Go*azM)))-(GC*(((azZ+azZ)+(aA5+aA5))/aAc)))/aAj)}else{d});let aAv=(if Gb{(((GG*((GB*azp)+(Go*azN)))-(GC*(((aA1+aA1)+(aA7+aA7))/aAc)))/aAj)}else{d});let aAz=(GI*GI);let aAI=(if Gb{(((GI*sf[352])-(Gi*aAt))/aAz)}else{d});let aAJ=(if Gb{(((sf[0]*GI)-(Gi*aAu))/aAz)}else{d});let aAK=(if Gb{((-(Gi*aAv))/aAz)}else{d});let aAL=(gp*aAt);let aAM=(gp*aAu);let aAN=(gp*aAv);let aAO=(Gh*aAL);let aAP=(Gh*aAM);let aAQ=(Gh*aAN);let aB3=(if Gb{(aAI+((GM*a1Y)+(sb_*aAO)))}else{d});let aB4=(if Gb{(aAJ+((GM*a1Z)+(sb_*aAP)))}else{d});let aB5=(if Gb{(aAK+((GM*a20)+(sb_*aAQ)))}else{d});let aBp=(H2*H2);let aBR=(if Gv{(-(GM*(-(a7m/H2))))}else{d});let aBS=(if Gv{(aAI-((H4*aAO)+(GM*(-(((H2*a7q)-(uB*(sf[218]*(if Gv{(sf[308]*(O*a1G))}else{d}))))/aBp)))))}else{d});let aBT=(if Gv{(aAJ-((H4*aAP)+(GM*(-(((H2*a7u)-(uB*(sf[218]*(if Gv{(sf[308]*(O*a1H))}else{d}))))/aBp)))))}else{d});let aBU=(if Gv{(aAK-((H4*aAQ)+(GM*(-(((H2*a7y)-(uB*(sf[218]*(if Gv{(sf[308]*(O*a1I))}else{d}))))/aBp)))))}else{d});let aBY=(H8*aBR);let aC0=(H8*(aBS-aB3));let aC2=(H8*(aBT-aB4));let aC4=(H8*(aBU-aB5));let aCE=(O*Hh);let aCR=(if Gv{(gp*(aBR+((if Gv{(aBY+aBY)}else{d})/aCE)))}else{d});
        let aCS=(if Gv{(gp*((aB3+aBS)+((if Gv{((aC0+aC0)+(((Hb*a1P)+(s8*((Ha*aAI)+(GK*(a3*aAI)))))/sf[218]))}else{aza})/aCE)))}else{(if Gs{aB3}else{d})});let aCT=(if Gv{(gp*((aB4+aBT)+((if Gv{((aC2+aC2)+(((Hb*a1Q)+(s8*((Ha*aAJ)+(GK*(a3*aAJ)))))/sf[218]))}else{azb})/aCE)))}else{(if Gs{aB4}else{d})});let aCU=(if Gv{(gp*((aB5+aBU)+((if Gv{((aC4+aC4)+(((Hb*a1R)+(s8*((Ha*aAK)+(GK*(a3*aAK)))))/sf[218]))}else{azc})/aCE)))}else{(if Gs{aB5}else{d})});let aD1=(Hk*Hk);let aDl=(Hn*Hn);let aDz=(if Hs{((-(GL*(if Gb{(((Hk*aCR)-(Hl*aCR))/aD1)}else{d})))/aDl)}else{d});let aDA=(if Hs{(((Hn*aAL)-(GL*(if Gb{(((Hk*(aCS-aAI))-(Hl*aCS))/aD1)}else{d})))/aDl)}else{d});let aDB=(if Hs{(((Hn*aAM)-(GL*(if Gb{(((Hk*(aCT-aAJ))-(Hl*aCT))/aD1)}else{d})))/aDl)}else{d});let aDC=(if Hs{(((Hn*aAN)-(GL*(if Gb{(((Hk*(aCU-aAK))-(Hl*aCU))/aD1)}else{d})))/aDl)}else{d});let aDV=((-(sf[919]*aCR))/aD1);let aDY=((-(sf[919]*aCS))/aD1);let aE1=((-(sf[919]*aCT))/aD1);let aE4=((-(sf[919]*aCU))/aD1);let aE5=(HA*aDV);let aE6=(HA*aDY);let aE7=(HA*aE1);let aE8=(HA*aE4);let aEb=(Hu*Hu);let aFh=(sf[297]*f64::powf(FG,sf[387]));let aFn=(HV*HV);let aFH=(sf[314]*f64::powf(HX,sf[388]));let aFU=(if HS{(HT*((-(((HV*a7m)-(uB*a7m))/aFn))*aFH))}else{d});let aFV=(if HS{((HZ*(sf[352]*aFh))+(HT*((-(((HV*a7q)-(uB*a7q))/aFn))*aFH)))}else{d});let aFW=(if HS{((HZ*(sf[0]*aFh))+(HT*((-(((HV*a7u)-(uB*a7u))/aFn))*aFH)))}else{d});let aFX=(if HS{(HT*((-(((HV*a7y)-(uB*a7y))/aFn))*aFH))}else{d});let aG6=(if I4{(a7m/sf[313])}else{d});let aG7=(if I4{(a7q/sf[313])}else{d});let aG8=(if I4{(a7u/sf[313])}else{d});let aG9=(if I4{(a7y/sf[313])}else{d});let aGe=(if I4{(aG6/sf[316])}else{sf[365]});let aGf=(if I4{(aG7/sf[316])}else{sf[366]});let aGg=(if I4{(aG8/sf[316])}else{d});let aGh=(if I4{(aG9/sf[316])}else{d});let aGY=(sf[317]*f64::powf(Iu,sf[389]));let aHj=(sf[916]*(if I4{((Iw*aFU)+(I1*((if In{(aG6+(sf[316]*((Ip*(-aGe))/Iq)))}else{(if If{(sf[316]*((Ig*aGe)/Ih))}else{d})})*aGY)))}else{(if I2{aFU}else{d})}));let aHk=(sf[916]*(if I4{((Iw*aFV)+(I1*((if In{(aG7+(sf[316]*((Ip*(-aGf))/Iq)))}else{(if If{(sf[316]*((Ig*aGf)/Ih))}else{d})})*aGY)))}else{(if I2{aFV}else{d})}));let aHl=(sf[916]*(if I4{((Iw*aFW)+(I1*((if In{(aG8+(sf[316]*((Ip*(-aGg))/Iq)))}else{(if If{(sf[316]*((Ig*aGg)/Ih))}else{d})})*aGY)))}else{(if I2{aFW}else{d})}));let aHm=(sf[916]*(if I4{((Iw*aFX)+(I1*((if In{(aG9+(sf[316]*((Ip*(-aGh))/Iq)))}else{(if If{(sf[316]*((Ig*aGh)/Ih))}else{d})})*aGY)))}else{(if I2{aFX}else{d})}));let aHN=(if HS{(IM*(if IG{(IH*aHj)}else{(if IC{(ID*aHj)}else{ayA})}))}else{(if HJ{(HK*aE5)}else{(if Hs{((HF*((Hw*aDz)+(Hu*(sf[918]*aCR))))+(Hx*(aE5-(HE*((HC*aDV)+(Hz*((-(GB*aDz))/aEb)))))))}else{(if Fv{((G1*ayA)+(FY*(sf[917]*ay9)))}else{d})})})});let aHO=(if HS{((IM*(if IG{(IH*aHk)}else{(if IC{(ID*aHk)}else{ayB})}))+(IL*sf[979]))}else{(if HJ{((HK*aE6)+(HA*(sf[4]*azL)))}else{(if Hs{((HF*((Hw*aDA)+(Hu*(sf[918]*aCS))))+(Hx*(aE6-(HE*((HC*aDY)+(Hz*(((Hu*azL)-(GB*aDA))/aEb)))))))}else{(if Fv{((G1*ayB)+(FY*(sf[917]*aya)))}else{d})})})});let aHP=(if HS{((IM*(if IG{(IH*aHl)}else{(if IC{(ID*aHl)}else{ayC})}))+(IL*sf[980]))}else{(if HJ{((HK*aE7)+(HA*(sf[4]*azM)))}else{(if Hs{((HF*((Hw*aDB)+(Hu*(sf[918]*aCT))))+(Hx*(aE7-(HE*((HC*aE1)+(Hz*(((Hu*azM)-(GB*aDB))/aEb)))))))}else{(if Fv{((G1*ayC)+(FY*(sf[917]*ayb)))}else{d})})})});let aHQ=(if HS{(IM*(if IG{(IH*aHm)}else{(if IC{(ID*aHm)}else{ayD})}))}else{(if HJ{((HK*aE8)+(HA*(sf[4]*azN)))}else{(if Hs{((HF*((Hw*aDC)+(Hu*(sf[918]*aCU))))+(Hx*(aE8-(HE*((HC*aE4)+(Hz*(((Hu*azN)-(GB*aDC))/aEb)))))))}else{(if Fv{((G1*ayD)+(FY*(sf[917]*ayc)))}else{d})})})});let aI5=(IX*IX);let aIu=(IW*IW);let aIJ=(if IV{((((-(sf[408]*((IW*a7m)+(uB*axg))))/aI5)+(sf[712]*(a70/sf[684])))+((-(sf[597]*axg))/aIu))}else{d});let aIK=(if IV{((((-(sf[408]*((IW*a7q)+(uB*axh))))/aI5)+(sf[712]*(a73/sf[684])))+((-(sf[597]*axh))/aIu))}else{d});let aIL=(if IV{((((-(sf[408]*((IW*a7u)+(uB*axi))))/aI5)+(sf[712]*(a76/sf[684])))+((-(sf[597]*axi))/aIu))}else{d});let aIM=(if IV{((((-(sf[408]*((IW*a7y)+(uB*axj))))/aI5)+(sf[712]*(a79/sf[684])))+((-(sf[597]*axj))/aIu))}else{d});let aIV=(if J5{((aHN-aIJ)/gl)}else{aGe});
        let aIW=(if J5{((aHO-aIK)/gl)}else{aGf});let aIX=(if J5{((aHP-aIL)/gl)}else{aGg});let aIY=(if J5{((aHQ-aIM)/gl)}else{aGh});let aJD=(if Jj{(aIJ-(gl*((Jl*(-aIV))/Jm)))}else{(if Jb{(aHN-(gl*((Jc*aIV)/Jd)))}else{aHN})});let aJE=(if Jj{(aIK-(gl*((Jl*(-aIW))/Jm)))}else{(if Jb{(aHO-(gl*((Jc*aIW)/Jd)))}else{aHO})});let aJF=(if Jj{(aIL-(gl*((Jl*(-aIX))/Jm)))}else{(if Jb{(aHP-(gl*((Jc*aIX)/Jd)))}else{aHP})});let aJG=(if Jj{(aIM-(gl*((Jl*(-aIY))/Jm)))}else{(if Jb{(aHQ-(gl*((Jc*aIY)/Jd)))}else{aHQ})});let aJJ=((Jq*a7m)+(uB*aJD));let aJM=((Jq*a7q)+(uB*aJE));let aJP=((Jq*a7u)+(uB*aJF));let aJS=((Jq*a7y)+(uB*aJG));let aKg=(Jw*Jw);let aZ7=(w*sf[352]);let aZ8=(sf[0]*w);let aZa=(w*sf[354]);let b09=(sf[15]*(sf[0]*(sf[750]*acV)));let b0d=((((if sb[35]{(sf[712]*((sf[249]*a8Q)+(wk*(sf[247]*a8Q))))}else{(if sb[33]{a9k}else{(if ((sf[155])!=0.0){((a9k+(wk*(((wi*(sf[889]*a8Q))-(we*((gB*a8Y)/a9q)))/a9w)))+(((wq*(wo*a9g))-(wp*a9g))/aa0))}else{d})})})+(sf[697]*abV))+aZ7)-(if zI{d}else{(if ((yh)!=0.0){(sf[23]*(sf[576]*((zD*(if ys{(yt*add)}else{(if yo{(yp*add)}else{d})}))+(yx*((zC*a2w)+(sC*(sf[891]*(if zp{((zy*((zq*aeq)+(z1*sf[372])))+(zr*((zw*(zs*aeq))+(zt*(zu*aeq)))))}else{(if z7{((sf[0]*zl)+(zi*(((z1*(-(if zc{(zd*aeq)}else{(if z8{(z9*aeq)}else{d})})))-(zj*aeq))/aeF)))}else{d})}))))))))}else{d})}));let b0e=((((if sb[35]{(sf[712]*((sf[249]*a8R)+((wG*a5q)+(wk*(sf[247]*(a0Z+a8R))))))}else{(if sb[33]{a9l}else{(if ((sf[155])!=0.0){((a9l+((wk*(((wi*(sf[889]*a8R))-(we*((gB*a8Z)/a9q)))/a9w))+(wj*a5q)))+(((wq*((wo*a9h)+(wa*(sf[727]*a0Z))))-(wp*a9h))/aa0))}else{d})})})+(sf[697]*abX))+aZ8)-(if zI{d}else{(if ((yh)!=0.0){(sf[23]*(sf[576]*((zD*(if ys{(yt*ade)}else{(if yo{(yp*ade)}else{d})}))+(yx*((zC*a2x)+(sC*(sf[891]*(if zp{((zy*((zq*aer)+(z1*sf[373])))+(zr*((zw*(zs*aer))+(zt*(zu*aer)))))}else{(if z7{((zl*sf[352])+(zi*(((z1*(-(if zc{(zd*aer)}else{(if z8{(z9*aer)}else{d})})))-(zj*aer))/aeF)))}else{d})}))))))))}else{d})}));let b0L=(sf[15]*(sf[0]*(-(Be*auE))));let b0M=(sf[15]*(sf[0]*(-(Be*auF))));let b0N=(sf[15]*(sf[0]*(-(Be*auG))));let b0O=(sf[15]*(sf[0]*(-(Be*auH))));let b0P=(sf[15]*(sf[0]*(-((EQ*(if Bd{d}else{(if ((zP)!=0.0){(sf[55]*(sf[577]*((B8*(if A4{(A5*ag0)}else{(if A0{(A1*ag0)}else{d})}))+(A9*((B7*afN)+(zT*(sf[892]*(if AW{((B3*((AX*ahb)+(Az*sf[373])))+(AY*((B1*(zs*ahb))+(AZ*(zu*ahb)))))}else{(if AE{((AS*sf[352])+(AP*(((Az*(-(if AJ{(AK*ahb)}else{(if AF{(AG*ahb)}else{d})})))-(AQ*ahb))/ahq)))}else{d})}))))))))}else{d})}))+(Be*auI)))));let b0Q=(sf[15]*(sf[0]*(-((EQ*(if Bd{d}else{(if ((zP)!=0.0){(sf[55]*(sf[577]*((B8*(if A4{(A5*ag1)}else{(if A0{(A1*ag1)}else{d})}))+(A9*((B7*afO)+(zT*(sf[892]*(if AW{((B3*((AX*ahc)+(Az*sf[372])))+(AY*((B1*(zs*ahc))+(AZ*(zu*ahc)))))}else{(if AE{((sf[0]*AS)+(AP*(((Az*(-(if AJ{(AK*ahc)}else{(if AF{(AG*ahc)}else{d})})))-(AQ*ahc))/ahq)))}else{d})}))))))))}else{d})}))+(Be*auJ)))));let b0R=(sf[15]*(sf[0]*(-(Be*auK))));let b0S=(sf[15]*(sf[0]*(-(Be*auL))));let b0T=(sf[15]*(sf[0]*(-(Be*auM))));let b1F=(sf[15]*(sf[0]*(if ((sf[267])!=0.0){(arJ+(Df*ar7))}else{d})));

        stamper.stamp_current_node3_local(
            Some(7),
            Some(8),
            multiplicity * ((sf[15]*(sf[0]*oK))),
            6,
            multiplicity * ((sf[15]*(sf[0]*TW))),
            7,
            multiplicity * ((sf[15]*(sf[0]*TX))),
            8,
            multiplicity * ((sf[15]*(sf[0]*TY))),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(4),
            multiplicity * ((sf[15]*(sf[0]*uB))),
            [4, 6, 7, 8],
            [(sf[15]*(sf[0]*a7m)), (sf[15]*(sf[0]*a7q)), (sf[15]*(sf[0]*a7u)), (sf[15]*(sf[0]*a7y))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(4),
            multiplicity * ((sf[15]*(sf[0]*((sf[750]*(ya-b))+((if sb[32]{xc}else{(if ((sf[155])!=0.0){(xc+(xe/xi))}else{d})})+(sf[744]*(xK-b))))))),
            [4, 5, 6, 7, 8, 10],
            [(sf[15]*(sf[0]*((sf[750]*acS)+((if sb[32]{abd}else{(if ((sf[155])!=0.0){(abd+(((xi*(sf[890]*ab0))-(xe*((gB*(if x5{(x6*sf[942])}else{(if x1{(x2*sf[942])}else{a8Y})}))/abm)))/abt))}else{d})})+(sf[744]*aca))))), (sf[15]*(sf[0]*((sf[750]*acT)+((if sb[32]{abe}else{(if ((sf[155])!=0.0){(abe+(((xi*(sf[890]*ab1))-(xe*((gB*(if x5{(x6*sf[941])}else{(if x1{(x2*sf[941])}else{d})}))/abm)))/abt))}else{d})})+(sf[744]*acb))))), (sf[15]*(sf[0]*((sf[750]*acU)+((if sb[32]{abf}else{(if ((sf[155])!=0.0){(abf+(((xi*(sf[890]*ab2))-(xe*((gB*(if x5{d}else{(if x1{d}else{a8Z})}))/abm)))/abt))}else{d})})+(sf[744]*acc))))), b09, b09, (sf[15]*(sf[0]*(sf[750]*acW)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(4),
            multiplicity * ((sf[15]*(sf[0]*((sf[755]*(v4-b))+((vr*vt)+((((if sb[35]{(sf[712]*((wb*sf[249])+(wk*wG)))}else{(if sb[33]{wc}else{(if ((sf[155])!=0.0){((wc+(wj*wk))+(wp/wq))}else{d})})})+(sf[697]*(xx-b)))+(w*lz))-(if zI{d}else{(if ((yh)!=0.0){(sf[23]*(sf[576]*(yx*zD)))}else{d})}))))))),
            [4, 5, 6, 7, 8],
            [(sf[15]*(sf[0]*((sf[755]*a83)+(((vt*(sf[246]*a8t))+(vr*((-a8t)*a8z)))+b0d)))), (sf[15]*(sf[0]*(sf[697]*abW))), (sf[15]*(sf[0]*((sf[755]*a84)+(((vt*(sf[246]*a8u))+(vr*((-a8u)*a8z)))+b0e)))), (sf[15]*(sf[0]*(if sb[35]{(sf[712]*((wG*a5r)+(wk*(sf[247]*a10))))}else{(if sb[33]{d}else{(if ((sf[155])!=0.0){((wj*a5r)+(((wq*((wo*a9i)+(wa*(sf[727]*a10))))-(wp*a9i))/aa0))}else{d})})}))), (sf[15]*(sf[0]*(if sb[35]{(sf[712]*((wG*a5s)+(wk*(sf[247]*a11))))}else{(if sb[33]{d}else{(if ((sf[155])!=0.0){((wj*a5s)+(((wq*((wo*a9j)+(wa*(sf[727]*a11))))-(wp*a9j))/aa0))}else{d})})})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * ((if ((sf[155])!=0.0){PJ}else{d})),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [(if ((sf[155])!=0.0){b0L}else{d}), (if ((sf[155])!=0.0){b0M}else{d}), (if ((sf[155])!=0.0){b0N}else{d}), (if ((sf[155])!=0.0){b0O}else{d}), (if ((sf[155])!=0.0){b0P}else{d}), (if ((sf[155])!=0.0){b0Q}else{d}), (if ((sf[155])!=0.0){b0R}else{d}), (if ((sf[155])!=0.0){b0S}else{d}), (if ((sf[155])!=0.0){b0T}else{d})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(8),
            multiplicity * ((if sb[32]{PJ}else{d})),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [(if sb[32]{b0L}else{d}), (if sb[32]{b0M}else{d}), (if sb[32]{b0N}else{d}), (if sb[32]{b0O}else{d}), (if sb[32]{b0P}else{d}), (if sb[32]{b0Q}else{d}), (if sb[32]{b0R}else{d}), (if sb[32]{b0S}else{d}), (if sb[32]{b0T}else{d})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*(if ((sf[267])!=0.0){(sf[7]*Cl)}else{Cl})))),
            [3, 5, 6, 7, 8, 10],
            [(sf[15]*(sf[0]*(if ((sf[267])!=0.0){(sf[7]*alU)}else{alU}))), (sf[15]*(sf[0]*(if ((sf[267])!=0.0){(sf[7]*alV)}else{alV}))), (sf[15]*(sf[0]*(if ((sf[267])!=0.0){(sf[7]*alW)}else{alW}))), (sf[15]*(sf[0]*(if ((sf[267])!=0.0){(sf[7]*alX)}else{alX}))), (sf[15]*(sf[0]*(if ((sf[267])!=0.0){(sf[7]*alY)}else{alY}))), (sf[15]*(sf[0]*(if ((sf[267])!=0.0){(sf[7]*alZ)}else{alZ})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*(if sb[43]{(C8/Cc)}else{(if ((sf[259])!=0.0){(BH/BQ)}else{d})})))),
            [3, 6, 7, 8],
            [(sf[15]*(sf[0]*(if sb[43]{d}else{(if ((sf[259])!=0.0){(((BQ*(sf[896]*(-S2)))-(BH*((sf[898]*(sf[262]*S2))/ajU)))/ak2)}else{d})}))), (sf[15]*(sf[0]*(if sb[43]{(((Cc*ajL)-(C8*(ajR/alg)))/alm)}else{(if ((sf[259])!=0.0){(((BQ*ajL)-(BH*(ajR/ajU)))/ak2)}else{d})}))), (sf[15]*(sf[0]*(if sb[43]{d}else{(if ((sf[259])!=0.0){(((BQ*(sf[896]*(-S3)))-(BH*((sf[898]*(sf[262]*S3))/ajU)))/ak2)}else{d})}))), (sf[15]*(sf[0]*(if sb[43]{(((Cc*ajN)-(C8*(ajT/alg)))/alm)}else{(if ((sf[259])!=0.0){(((BQ*ajN)-(BH*(ajT/ajU)))/ak2)}else{d})})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*(if ((sf[267])!=0.0){(Df*DU)}else{d})))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [b1F, (sf[15]*(sf[0]*(if ((sf[267])!=0.0){((DU*aox)+(Df*ar8))}else{d}))), (sf[15]*(sf[0]*(if ((sf[267])!=0.0){((DU*aoy)+(Df*ar9))}else{d}))), (sf[15]*(sf[0]*(if ((sf[267])!=0.0){(Df*ara)}else{d}))), b1F, (sf[15]*(sf[0]*(if ((sf[267])!=0.0){(arJ+(Df*arb))}else{d}))), (sf[15]*(sf[0]*(if ((sf[267])!=0.0){(arV+(Df*arc))}else{d}))), (sf[15]*(sf[0]*(if ((sf[267])!=0.0){((DU*aoA)+(Df*ard))}else{d}))), (sf[15]*(sf[0]*(if ((sf[267])!=0.0){((DU*aoB)+(Df*are))}else{d}))), (sf[15]*(sf[0]*(if ((sf[267])!=0.0){(arV+(Df*arf))}else{d})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * ((sf[15]*(sf[0]*((Co/Cv)+(M*lH))))),
            3,
            multiplicity * ((sf[15]*(sf[0]*((((Cv*(sf[900]*S2))-(Co*((sf[902]*S2)/am4)))/ama)+(sf[0]*M))))),
            7,
            multiplicity * ((sf[15]*(sf[0]*((((Cv*(sf[900]*S3))-(Co*((sf[902]*S3)/am4)))/ama)+(M*sf[352]))))),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * ((sf[15]*(sf[0]*(Ff/Fc)))),
            [4, 5, 6, 7, 8],
            [(sf[15]*(sf[0]*((-(Ff*axg))/axq))), (sf[15]*(sf[0]*((sf[0]+(sf[862]*(if mQ{(mR*sf[941])}else{(if ((mN)!=0.0){(mO*sf[941])}else{d})})))/Fc))), (sf[15]*(sf[0]*(((Fc*(sf[352]+(sf[862]*(if mQ{(mR*sf[942])}else{(if ((mN)!=0.0){(mO*sf[942])}else{d})}))))-(Ff*axh))/axq))), (sf[15]*(sf[0]*((-(Ff*axi))/axq))), (sf[15]*(sf[0]*((-(Ff*axj))/axq)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * ((sf[15]*(sf[0]*(-JB)))),
            [4, 6, 7, 8],
            [(sf[15]*(sf[0]*(-(if JA{aJJ}else{(if Ju{(((Jw*((Jr*aIJ)+(J4*aJJ)))-(Jv*(aIJ+aJD)))/aKg)}else{(if J5{aJJ}else{d})})})))), (sf[15]*(sf[0]*(-(if JA{aJM}else{(if Ju{(((Jw*((Jr*aIK)+(J4*aJM)))-(Jv*(aIK+aJE)))/aKg)}else{(if J5{aJM}else{d})})})))), (sf[15]*(sf[0]*(-(if JA{aJP}else{(if Ju{(((Jw*((Jr*aIL)+(J4*aJP)))-(Jv*(aIL+aJF)))/aKg)}else{(if J5{aJP}else{d})})})))), (sf[15]*(sf[0]*(-(if JA{aJS}else{(if Ju{(((Jw*((Jr*aIM)+(J4*aJS)))-(Jv*(aIM+aJG)))/aKg)}else{(if J5{aJS}else{d})})}))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(4),
            multiplicity * ((sf[15]*((sf[0]*(sf[0]*(lK-lx)))/sf[597]))),
            2,
            multiplicity * (sf[1010]),
            4,
            multiplicity * (sf[1011]),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(5),
            multiplicity * ((sf[15]*((sf[0]*lP)/sf[605]))),
            1,
            multiplicity * (sf[1014]),
            5,
            multiplicity * (sf[1015]),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(4),
            multiplicity * (Q6),
            [4, 5, 6, 7, 8, 10],
            [b2x, b2y, b2z, b2A, b2B, b2C],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(5),
            Some(4),
            multiplicity * (Q9),
            4,
            multiplicity * (b2H),
            5,
            multiplicity * (b2I),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(8),
            multiplicity * (Qc),
            [4, 5, 6, 7, 8, 10],
            [b2V, b2W, b2X, b2Y, b2Z, b30],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * (Qf),
            3,
            multiplicity * (b35),
            7,
            multiplicity * (b36),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (Qi),
            [4, 5, 6, 7, 8, 10],
            [b3j, b3k, b3l, b3m, b3n, b3o],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (Qm),
            1,
            multiplicity * (b3t),
            2,
            multiplicity * (b3u),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (Qq),
            0,
            multiplicity * (b3z),
            1,
            multiplicity * (b3A),
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * ((sf[15]*(sf[0]*(DW*EQ)))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [(sf[15]*(sf[0]*(avC+(DW*auE)))), (sf[15]*(sf[0]*((EQ*(if ((sf[267])!=0.0){((DU*an9)+(CR*ar8))}else{d}))+(DW*auF)))), (sf[15]*(sf[0]*(EQ*(if ((sf[267])!=0.0){(CR*ar9)}else{d})))), (sf[15]*(sf[0]*((EQ*(if ((sf[267])!=0.0){(CR*ara)}else{d}))+(DW*auG)))), (sf[15]*(sf[0]*(avC+(DW*auH)))), (sf[15]*(sf[0]*((EQ*(if ((sf[267])!=0.0){(arg+(CR*arb))}else{d}))+(DW*auI)))), (sf[15]*(sf[0]*((EQ*(if ((sf[267])!=0.0){(arq+(CR*arc))}else{d}))+(DW*auJ)))), (sf[15]*(sf[0]*((EQ*(if ((sf[267])!=0.0){(arq+(CR*ard))}else{d}))+(DW*auK)))), (sf[15]*(sf[0]*((EQ*(if ((sf[267])!=0.0){((DU*anb)+(CR*are))}else{d}))+(DW*auL)))), (sf[15]*(sf[0]*((EQ*(if ((sf[267])!=0.0){(arq+(CR*arf))}else{d}))+(DW*auM))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(9),
            multiplicity * ((sf[15]*(sf[851]*(sf[0]*m7)))),
            [0, 1, 5, 6, 7, 8, 9, 10],
            [sf[1020], sf[1021], sf[1021], sf[1021], sf[1022], sf[1022], sf[1023], sf[1022]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * (Qy),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [b4n, b4o, b4p, b4q, b4n, b4r, b4s, b4t, b4u, b4v],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(10),
            multiplicity * ((sf[15]*(sf[0]*((CF*EQ)+((xZ*EQ)+(w*m3)))))),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [(sf[15]*(sf[0]*((CF*auE)+(xZ*auE)))), (sf[15]*(sf[0]*((CF*auF)+(xZ*auF)))), (sf[15]*(sf[0]*((CF*auG)+((EQ*(sf[704]*acx))+(xZ*auG))))), (sf[15]*(sf[0]*(((EQ*(if ((sf[267])!=0.0){(sf[7]*ajv)}else{ajv}))+(CF*auH))+(((EQ*(sf[704]*acy))+(xZ*auH))+aZ8)))), (sf[15]*(sf[0]*(((EQ*(if ((sf[267])!=0.0){(sf[7]*ajz)}else{ajz}))+(CF*auI))+(((EQ*(sf[704]*acz))+(xZ*auI))+(w*sf[353]))))), (sf[15]*(sf[0]*((av9+(CF*auJ))+((avt+(xZ*auJ))+aZa)))), (sf[15]*(sf[0]*((av9+(CF*auK))+((avt+(xZ*auK))+aZa)))), (sf[15]*(sf[0]*((CF*auL)+(xZ*auL)))), (sf[15]*(sf[0]*(((EQ*(if ((sf[267])!=0.0){(sf[7]*ajH)}else{ajH}))+(CF*auM))+(((EQ*(sf[704]*acB))+(xZ*auM))+aZ7))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(10),
            multiplicity * (QE),
            [5, 6, 7, 8, 10],
            [b51, b52, b53, b53, b54],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(9),
            Some(10),
            multiplicity * ((if ((sf[213])!=0.0){(sf[15]*(sf[856]*(sf[0]*m0)))}else{d})),
            9,
            multiplicity * (sf[1028]),
            10,
            multiplicity * (sf[1029]),
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
            multiplicity * ((if ((sf[214])!=0.0){(sf[15]*(sf[861]*(sf[0]*lX)))}else{d})),
            7,
            multiplicity * (sf[1034]),
            10,
            multiplicity * (sf[1035]),
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
            multiplicity * (QN),
            11,
            multiplicity * (b),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(4),
            multiplicity * (QP),
            [4, 5, 6, 7, 8, 10, 11],
            [b5h, b5i, b5j, b5k, b5l, b5m, b5n],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(8),
            Some(6),
            multiplicity * ((OR*QN)),
            11,
            multiplicity * (OR),
        );
        stamper.stamp_current_node1_local(
            Some(8),
            Some(4),
            multiplicity * (QN),
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
        let CommonStampValues {
            b, d, N, O, a3, bR, gl, gp,
            gB, h1, lt, lx, lz, lE, lH, lK,
            lP, lX, m0, m3, m7, mn, mK, mL,
            mN, mQ, mR, n7, n9, nc, nd, nt,
            nv, ny, nz, oK, qI, rG, s5, s8,
            sb_, sC, tU, uu, uv, uA, uB, uU,
            uW, uZ, v0, v9, vF, vH, vJ, vO,
            vP, vW, vX, vZ, w4, w6, wW, wY,
            x0, x5, x6, xx, xK, xX, ya, yh,
            yi, yl, yn, ys, yt, yz, yD, yG,
            yO, yP, yQ, yS, yU, yY, yZ, z1,
            z4, z6, z7, zc, zd, zP, zR, zT,
            zU, zX, zZ, A4, A5, Aa, Ad, Af,
            An, Ao, Ap, Ar, Aw, Ax, Az, AB,
            AD, AE, AJ, AK, CR, Df, Dx, DU,
            F6, Fi, Fv, Fw, Fx, FA, FB, FF,
            FG, FI, FM, FO, FT, FU, G9, HS,
            HT, HV, HX, HZ, I1, I2, I4, Ic,
            If, Ig, Ih, In, Ip, Iq, Iu, Iw,
            Iz, IB, IG, IH, OJ, Q6, Q9, Qc,
            Qf, Qi, Qm, Qq, Qy, QE, QN, QP,
            R3, R4, Rt, Ru, Rv, Rw, TW, TX,
            TY, YB, YC, YD, a0Z, a10, a11, a1G,
            a1H, a1I, a1P, a1Q, a1R, a1Y, a1Z, a20,
            a2w, a2x, a5q, a5r, a5s, a6U, a6V, a6W,
            a6X, a70, a73, a76, a79, a7a, a7b, a7c,
            a7e, a7i, a7l, a7T, a7U, a8Q, a8R, ab0,
            ab1, ab2, abV, abW, abX, aca, acb, acc,
            acx, acy, acz, acA, acB, acS, acT, acU,
            acV, acW, an8, an9, ana, anb, aow, aox,
            aoy, aoz, aoA, aoB, aoO, aoP, aoQ, aoR,
            aoS, aoT, aoU, aoV, ar7, ar8, ar9, ara,
            arb, arc, ard, are, arf, awJ, awK, awL,
            awM, b2x, b2y, b2z, b2A, b2B, b2C, b2H,
            b2I, b2V, b2W, b2X, b2Y, b2Z, b30, b35,
            b36, b3j, b3k, b3l, b3m, b3n, b3o, b3t,
            b3u, b3z, b3A, b4n, b4o, b4p, b4q, b4r,
            b4s, b4t, b4u, b4v, b51, b52, b53, b54,
            b5h, b5i, b5j, b5k, b5l, b5m, b5n,
        }=self.eval_common_stamp_values::<true>(ctx);
        let p=&(*self.params);
        let m=self.multiplicity;
        let multiplicity=m;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(4),
            &[4, 5, 6, 7, 8, 10],
            &[b2x, b2y, b2z, b2A, b2B, b2C],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2_local(
            Some(5),
            Some(4),
            4,
            multiplicity * (b2H),
            5,
            multiplicity * (b2I),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(8),
            &[4, 5, 6, 7, 8, 10],
            &[b2V, b2W, b2X, b2Y, b2Z, b30],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2_local(
            Some(3),
            Some(7),
            3,
            multiplicity * (b35),
            7,
            multiplicity * (b36),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[4, 5, 6, 7, 8, 10],
            &[b3j, b3k, b3l, b3m, b3n, b3o],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(2),
            1,
            multiplicity * (b3t),
            2,
            multiplicity * (b3u),
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(0),
            0,
            multiplicity * (b3z),
            1,
            multiplicity * (b3A),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(9),
            &[0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            &[b4n, b4o, b4p, b4q, b4n, b4r, b4s, b4t, b4u, b4v],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(10),
            &[5, 6, 7, 8, 10],
            &[b51, b52, b53, b53, b54],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(4),
            &[4, 5, 6, 7, 8, 10, 11],
            &[b5h, b5i, b5j, b5k, b5l, b5m, b5n],
            &[],
            &[],
            multiplicity,
        );
    }
}
