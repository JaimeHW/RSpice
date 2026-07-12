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
    b: f64, d: f64, T: f64, ca: f64, fk: f64, fF: f64,
    fG: f64, fH: f64, gE: f64, h1: f64, h8: f64, hf: f64,
    hm: f64, hU: f64, iV: f64, ji: f64, jj: f64, k5: f64,
    k6: f64, k8: f64, k9: f64, kb: f64, kc: f64, ke: f64,
    kf: f64, kk: f64, km: f64, kn: f64, ko: f64, ks: f64,
    kB: f64, kC: f64, p2: f64, pk: f64, pp: f64, ps: f64,
    px: f64, pV: f64, q8: f64, qM: f64, rm: f64, ro: f64,
    sd: f64, sA: f64, sB: f64, te: f64, tw: f64, tx: f64,
    ud: f64, uu: f64, uv: f64, v5: f64, vo: f64, vp: f64,
    vI: f64, vX: f64, wJ: f64, wM: f64, Dg: f64, Di: f64,
    Dk: f64, Dm: f64, Dp: f64, Dq: f64, Dr: f64, Ds: f64,
    Dt: f64, Du: f64, Dz: f64, DB: f64, DC: f64, EG: f64,
    Fu: f64, Fy: f64, FK: f64, FO: f64, G0: f64, G4: f64,
    Gg: f64, Gk: f64, Ic: f64, IN: f64, IQ: f64, IU: f64,
    Tg: f64, Th: f64, Ti: f64, TQ: f64, TR: f64, TS: f64,
    TT: f64, Un: f64, Uo: f64, Up: f64, Uq: f64, Vn: f64,
    Vo: f64, Vp: f64, Vq: f64, VR: f64, VS: f64, VT: f64,
    VU: f64, VY: f64, Xs: f64, Xt: f64, Xu: f64, Xv: f64,
    Xw: f64, Xx: f64, YU: f64, YV: f64, YW: f64, YX: f64,
    YY: f64, YZ: f64, Z2: f64, a0H: f64, a0I: f64, a0J: f64,
    a0K: f64, a1p: f64, a1q: f64, a1r: f64, a1s: f64, a1t: f64,
    a1u: f64, a1v: f64, a2F: f64, a2G: f64, a2H: f64, a2I: f64,
    a3h: f64, a3i: f64, a3j: f64, a3k: f64, a3l: f64, a3m: f64,
    a3n: f64, a57: f64, a58: f64, a59: f64, a5a: f64, a5J: f64,
    a5K: f64, a5L: f64, a5M: f64, a5N: f64, a5O: f64, a5P: f64,
    a75: f64, a76: f64, a77: f64, a78: f64, a7I: f64, a7J: f64,
    a7K: f64, a7L: f64, a7M: f64, a7N: f64, a7P: f64, a8y: f64,
    a8T: f64, a8U: f64, a8V: f64, a8W: f64, a8X: f64, a8Y: f64,
    aaL: f64, aaM: f64, aaN: f64, aaO: f64, aaP: f64, aaQ: f64,
    aaZ: f64, ab0: f64, ab1: f64, ab2: f64, ab3: f64, av8: f64,
    av9: f64, ava: f64, avb: f64, avc: f64, avd: f64, ave: f64,
    avf: f64, avg: f64, avh: f64, avi: f64, avj: f64, avk: f64,
    avl: f64, avm: f64, avn: f64, avo: f64, avp: f64, avq: f64,
    avr: f64, avs: f64, avt: f64, avu: f64, avv: f64, avw: f64,
    avx: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let b=0.0;let d=1.0;let T=0.5;let b9=273.15;let bB=1.380662e-23;let bD=1.602189e-19;let ca=4.0;let fk=ctx.node_voltage(n[3]);let fm=((sf[298]+fk)-b9);let fo=(if (fm<sf[79]){d}else{b});let fr=(((fm-sf[78])-d)).exp();let ft=(if ((fo)!=0.0){(sf[78]+fr)}else{fm});let fx=((((if (ft>sf[81]){d}else{b}))!=0.0)&&(!((fo)!=0.0)));let fA=(((sf[80]-ft)-d)).exp();let fD=(b9+(if fx{(sf[80]-fA)}else{ft}));let fF=((bB*fD)/bD);let fG=(fD/sf[76]);let fH=(fD-sf[76]);let fK=(sf[52]*f64::powf(fG,sf[137]));let gD=(sf[82]*f64::powf(fG,sf[88]));let gE=(d-fG);let gF=(sf[90]*gE);let gG=(sf[87]*fF);let gI=((gF/gG)).exp();let gJ=(gD*gI);let gL=(sf[99]*f64::powf(fG,sf[102]));let gM=(sf[104]*gE);let gN=(sf[101]*fF);let gP=((gM/gN)).exp();let gQ=(gL*gP);let gS=(sf[31]*f64::powf(fG,sf[110]));let gT=(sf[112]*gE);let gU=(sf[109]*fF);let gW=((gT/gU)).exp();let gX=(gS*gW);let h1=(sf[117]*fF);let h8=(sf[123]*fF);let hf=(sf[128]*fF);let hm=(sf[133]*fF);let hw=(d+(fH*sf[154]));let hx=(sf[87]*hw);let hy=(sf[101]*hw);let hM=(sf[159]+(fH*sf[160]));let hT=(sf[83]*(d+(fH*sf[161])));let hU=2.0;let hW=(hU*(fF/fG));let hZ=(fG*sf[163]);let i1=((hZ/fF)).exp();let i2=-0.5;let i4=(fG*sf[164]);let i6=((i4/fF)).exp();let i7=(i1-i6);let i8_=(i7).ln();let i9=(hW*i8_);let ib=3.0;let ic=(fF*ib);let id=(fG).ln();let ie=(ic*id);let ig=(fG-d);let ii=(((fG*i9)-ie)-(sf[119]*ig));let ij=(fF*hU);let ik=(-ii);let im=((ik/fF)).exp();let ip=((d+(ca*im))).sqrt();let ir=(T*(d+ip));let is=(ir).ln();let iu=(ii+(ij*is));let ix=(fG*sf[166]);let iz=((ix/fF)).exp();let iB=(fG*sf[167]);let iD=((iB/fF)).exp();let iE=(iz-iD);let iF=(iE).ln();let iG=(hW*iF);let iK=(((fG*iG)-ie)-(sf[130]*ig));let iL=(-iK);let iN=((iL/fF)).exp();let iQ=((d+(ca*iN))).sqrt();let iS=(T*(d+iQ));let iT=(iS).ln();let iV=(iK+(ij*iT));let iX=(sf[162]/iu);let j0=(sf[168]*f64::powf(iX,sf[169]));let j2=(sf[165]/iV);let j4=f64::powf(j2,sf[171]);let j5=(sf[170]*j4);let j7=(j4*sf[172]);let ja=(sf[173]*f64::powf(fG,sf[86]));let jc=((gF/fF)).exp();let jd=(ja*jc);let ji=(-(sf[4]*(d+(fH*hM))));let jj=(fF*hT);let jq=(sf[176]*(d+(fH*sf[177])));let jv=(sf[178]*(d+(fH*sf[179])));let jT=(jq>b);let jV=(if jT{(d/jq)}else{b});let jW=(jv>b);let jY=(if jW{(d/jv)}else{b});let jZ=(fK>b);let k1=(if jZ{(d/fK)}else{b});let k5=ctx.node_voltage(n[7]);let k6=ctx.node_voltage(n[8]);let k8=(sf[58]*(k5-k6));let k9=ctx.node_voltage(n[6]);let kb=(sf[58]*(k9-k6));let kc=ctx.node_voltage(n[5]);let ke=(sf[58]*(k5-kc));let kf=ctx.node_voltage(n[4]);let kh=(sf[58]*(k5-kf));let kk=ctx.node_voltage(n[9]);let km=(sf[58]*(k9-kk));let kn=ctx.node_voltage(n[1]);let ko=ctx.node_voltage(n[2]);let ks=ctx.node_voltage(n[0]);let kB=ctx.node_voltage(n[10]);let kC=ctx.node_voltage(n[11]);let kD=(-iu);let kF=(kD*sf[180]);let kG=(k8+kF);let kH=(if ((sf[7])!=0.0){kG}else{b});let kJ=(if (kH>b){d}else{b});let kK=(((sf[7])!=0.0)&&((kJ)!=0.0));let kO=(if kK{sf[183]}else{b});let kQ=(d-(sf[181]*kO));let kW=(kH*sf[185]);let kX=(iu*sf[181]);let kZ=(d+(kW/kX));let l4=(((sf[7])!=0.0)&&(!((kJ)!=0.0)));let l6=(d-(k8/iu));let l8=(d-f64::powf(l6,sf[184]));let lb=(if l4{((iu*l8)/sf[184])}else{(if kK{((iu*kQ)/sf[184])}else{b})});let lk=(((kF*kF)+sf[187])).sqrt();let lo=(if sb[51]{(i2*(kF+(if sb[51]{lk}else{b})))}else{b});let lq=(d-(lo/iu));let lr=f64::powf(lq,sf[184]);let lu=(if sb[51]{((kD*lr)/sf[184])}else{b});let lv=(if sb[51]{kG}else{b});let ly=((sf[187]+(lv*lv))).sqrt();let lD=(if sb[51]{((T*(lv-(if sb[51]{ly}else{b})))-kF)}else{b});let lF=(d-(lD/iu));let lG=f64::powf(lF,sf[184]);let lL=(lo+(k8-lD));let lM=(sf[183]*lL);let lN=(sf[185]*lL);let lP=(d+(lN/kX));let lT=(if sb[51]{(((if sb[51]{((kD*lG)/sf[184])}else{lb})+(lM*lP))-lu)}else{(if ((sf[7])!=0.0){(lb+(if l4{b}else{(if kK{(kO*(kH*kZ))}else{b})}))}else{b})});let lU=(-iV);let lV=(sf[180]*lU);let lW=(ke+lV);let lX=(if ((sf[1])!=0.0){lW}else{b});let lZ=(if (lX>b){d}else{b});let m0=(((sf[1])!=0.0)&&((lZ)!=0.0));let m3=(if m0{sf[189]}else{b});let m6=(d-(sf[181]*(sf[181]*m3)));let mc=(lX*sf[191]);let me=(sf[181]+(mc/iV));let ml=(if (sb[8]&&(ke<sf[192])){d}else{b});let mn=(((sf[1])!=0.0)&&(!((lZ)!=0.0)));let mo=(((ml)!=0.0)&&mn);
        let mq=(d+(sf[14]/iV));let mr=f64::powf(mq,sf[190]);let mt=(sf[190]*(sf[14]+ke));let mu=(sf[14]+iV);let mw=(d-(mt/mu));let my=(d-(mr*mw));let mD=(mn&&(!((ml)!=0.0)));let mF=(d-(ke/iV));let mH=(d-f64::powf(mF,sf[190]));let mK=(if mD{((iV*mH)/sf[190])}else{(if mo{((iV*my)/sf[190])}else{(if m0{((iV*m6)/sf[190])}else{b})})});let mQ=(sf[14]+lV);let mR=(sf[14]-lV);let mT=(if sb[53]{(mQ/mR)}else{b});let mU=(hU*mT);let mV=(mT-d);let n0=(((mV*mV)+sf[194])).sqrt();let n1=(d+mT);let n6=(((n1*n1)+sf[196])).sqrt();let n7=(n0+n6);let n9=(if sb[53]{(mU/n7)}else{b});let ne=(if sb[53]{(T*(((mR*n9)-sf[14])-lV))}else{b});let ng=(d-(ne/iV));let ni=(d-f64::powf(ng,sf[190]));let nl=(if sb[53]{((iV*ni)/sf[190])}else{b});let no=(lV+(sf[14]+(hU*ke)));let nq=(if sb[53]{(no/mR)}else{b});let nr=(hU*nq);let ns=(nq-d);let nv=((sf[194]+(ns*ns))).sqrt();let nw=(d+nq);let nz=((sf[196]+(nw*nw))).sqrt();let nA=(nv+nz);let nC=(if sb[53]{(nr/nA)}else{b});let nH=(if sb[53]{(T*(((mR*nC)-sf[14])-lV))}else{b});let nJ=(d-(nH/iV));let nL=(d-f64::powf(nJ,sf[190]));let nO=(if sb[53]{((iV*nL)/sf[190])}else{mK});let nR=(if sb[53]{(T*(d+nC))}else{b});let nU=(if sb[53]{f64::powf(mq,sf[197])}else{b});let nW=(d+(lV/iV));let nY=(if sb[53]{f64::powf(nW,sf[197])}else{b});let nZ=(d-nR);let o3=(if sb[53]{((nU*nZ)+(nR*nY))}else{b});let o5=(ne+(ke-nH));let of=((sf[194]+(lV*lV))).sqrt();let oj=(if sb[55]{(i2*(lV+(if sb[55]{of}else{b})))}else{ne});let ol=(d-(oj/iV));let om=f64::powf(ol,sf[190]);let op=(if sb[55]{((lU*om)/sf[190])}else{b});let oq=(if sb[55]{lW}else{b});let ot=((sf[194]+(oq*oq))).sqrt();let oy=(if sb[55]{((T*(oq-(if sb[55]{ot}else{b})))-lV)}else{nH});let oA=(d-(oy/iV));let oB=f64::powf(oA,sf[190]);let oL=(if sb[55]{(((if sb[55]{((lU*oB)/sf[190])}else{nO})+(sf[198]*(oj+(ke-oy))))-op)}else{(if sb[53]{((nO+(if sb[53]{(o3*o5)}else{b}))-nl)}else{(if ((sf[1])!=0.0){(mK+(if mn{b}else{(if m0{(m3*(lX*me))}else{b})}))}else{b})})});let oM=(fF*hx);let oN=(d/oM);let oP=(if (k8<sf[343]){d}else{b});let oR=((k8*oN)).exp();let oT=(!((oP)!=0.0));let oV=((sf[343]*oN)).exp();let oW=(k8-sf[343]);let oY=(d+(oN*oW));let p0=(if oT{(oV*oY)}else{(if ((oP)!=0.0){oR}else{b})});let p1=(p0-d);let p2=(gJ*p1);let p3=(fF*hy);let p4=(d/p3);let p6=(if (ke<sf[363]){d}else{b});let p8=((ke*p4)).exp();let pa=(!((p6)!=0.0));let pc=((sf[363]*p4)).exp();let pd=(ke-sf[363]);let pf=(d+(p4*pd));let ph=(if pa{(pc*pf)}else{(if ((p6)!=0.0){p8}else{p0})});let pi=(gJ*gQ);let pj=(ph-d);let pk=(pi*pj);let pp=0.0001;let pq=(((d+(jY*lT))+(jV*oL))-pp);let ps=1e-8;let pu=(((pq*pq)+ps)).sqrt();let px=(pp+(T*(pq+pu)));let pD=(ca*((k1*p2)+(sf[62]*pk)));let pF=(if ((sf[29])!=0.0){(f64::powf(px,sf[199])+pD)}else{b});let pH=(if (pF>ps){d}else{b});let pI=(((sf[29])!=0.0)&&((pH)!=0.0));let pO=(((sf[29])!=0.0)&&(!((pH)!=0.0)));let pV=(if sb[56]{(d+pD)}else{pF});let pX=(if (pV>ps){d}else{b});let pY=(sb[56]&&((pX)!=0.0));let pZ=(T*px);let q1=(d+f64::powf(pV,sf[93]));let q5=(sb[56]&&(!((pX)!=0.0)));let q8=(if q5{(pZ*sf[201])}else{(if pY{(pZ*q1)}else{(if pO{(T*(px+sf[200]))}else{(if pI{(T*(px+f64::powf(pF,sf[93])))}else{b})})})});let qc=(if ((sf[32])!=0.0){(d/gU)}else{p4});let qe=(if (km<sf[382]){d}else{b});let qf=(((sf[32])!=0.0)&&((qe)!=0.0));let qh=((km*qc)).exp();let qk=(((sf[32])!=0.0)&&(!((qe)!=0.0)));let qm=((sf[382]*qc)).exp();let qn=(km-sf[382]);let qp=(d+(qc*qn));let qr=(if qk{(qm*qp)}else{(if qf{qh}else{ph})});let qt=(if (ke<sf[382]){d}else{b});let qu=(((sf[32])!=0.0)&&((qt)!=0.0));let qw=((ke*qc)).exp();let qz=(((sf[32])!=0.0)&&(!((qt)!=0.0)));let qA=(ke-sf[382]);let qC=(d+(qc*qA));let qE=(if qz{(qm*qC)}else{(if qu{qw}else{b})});let qK=(((qr*sf[202])+(qE*sf[203]))-d);let qM=(if ((sf[32])!=0.0){(gX*qK)}else{b});let r6=(d/h1);let r7=(if ((sf[18])!=0.0){r6}else{qc});let r9=(if (k8<sf[396]){d}else{b});let ra=(((sf[18])!=0.0)&&((r9)!=0.0));let rc=((k8*r7)).exp();let re=(!((r9)!=0.0));let rf=(((sf[18])!=0.0)&&re);let rh=((sf[396]*r7)).exp();let ri=(k8-sf[396]);let rk=(d+(r7*ri));let rm=(if rf{(rh*rk)}else{(if ra{rc}else{qr})});let rn=(d/h8);let ro=(if ((sf[18])!=0.0){rn}else{r7});let rV=(ji-k8);
        let rW=(if sb[61]{rV}else{b});let rX=(d/jj);let rY=(if sb[61]{rX}else{ro});let s0=(if (rW<sf[323]){d}else{b});let s1=(sb[61]&&((s0)!=0.0));let s3=((rW*rY)).exp();let s6=(sb[61]&&(!((s0)!=0.0)));let s8=((sf[323]*rY)).exp();let s9=(rW-sf[323]);let sb_=(d+(rY*s9));let sd=(if s6{(s8*sb_)}else{(if s1{s3}else{qE})});let sl=(if sb[63]{r6}else{rY});let sn=(if (kb<sf[396]){d}else{b});let so=(sb[63]&&((sn)!=0.0));let sq=((kb*sl)).exp();let ss=(!((sn)!=0.0));let st=(sb[63]&&ss);let sv=((sf[396]*sl)).exp();let sw=(kb-sf[396]);let sy=(d+(sl*sw));let sA=(if st{(sv*sy)}else{(if so{sq}else{rm})});let sB=(if sb[63]{rn}else{sl});let sY=(if sb[64]{rV}else{rW});let sZ=(if sb[64]{rX}else{sB});let t1=(if (sY<sf[323]){d}else{b});let t2=(sb[64]&&((t1)!=0.0));let t4=((sY*sZ)).exp();let t7=(sb[64]&&(!((t1)!=0.0)));let t9=((sf[323]*sZ)).exp();let ta=(sY-sf[323]);let tc=(d+(sZ*ta));let te=(if t7{(t9*tc)}else{(if t2{t4}else{sd})});let tl=(if sb[66]{r6}else{sZ});let tm=(((r9)!=0.0)&&sb[66]);let to=((k8*tl)).exp();let tq=(re&&sb[66]);let ts=((sf[396]*tl)).exp();let tu=(d+(ri*tl));let tw=(if tq{(ts*tu)}else{(if tm{to}else{sA})});let tx=(if sb[66]{rn}else{tl});let tX=(if sb[69]{rV}else{sY});let tY=(if sb[69]{rX}else{tx});let u0=(if (tX<sf[323]){d}else{b});let u1=(sb[69]&&((u0)!=0.0));let u3=((tX*tY)).exp();let u6=(sb[69]&&(!((u0)!=0.0)));let u8_=((sf[323]*tY)).exp();let u9=(tX-sf[323]);let ub=(d+(tY*u9));let ud=(if u6{(u8_*ub)}else{(if u1{u3}else{te})});let uj=(if sb[66]{r6}else{tY});let uk=(((sn)!=0.0)&&sb[66]);let um=((kb*uj)).exp();let uo=(ss&&sb[66]);let uq=((sf[396]*uj)).exp();let us=(d+(sw*uj));let uu=(if uo{(uq*us)}else{(if uk{um}else{tw})});let uv=(if sb[66]{rn}else{uj});let uP=(if sb[69]{rV}else{tX});let uQ=(if sb[69]{rX}else{uv});let uS=(if (uP<sf[323]){d}else{b});let uT=(sb[69]&&((uS)!=0.0));let uV=((uP*uQ)).exp();let uY=(sb[69]&&(!((uS)!=0.0)));let v0=((sf[323]*uQ)).exp();let v1=(uP-sf[323]);let v3=(d+(uQ*v1));let v5=(if uY{(v0*v3)}else{(if uT{uV}else{ud})});let vb=(d/hf);let vd=(if (ke<sf[424]){d}else{b});let vf=((ke*vb)).exp();let vh=(!((vd)!=0.0));let vj=((sf[424]*vb)).exp();let vk=(ke-sf[424]);let vm=(d+(vb*vk));let vo=(if vh{(vj*vm)}else{(if ((vd)!=0.0){vf}else{uu})});let vp=(d/hm);let vI=(if ((sf[40])!=0.0){vb}else{vp});let vK=(if (km<sf[447]){d}else{b});let vL=(((sf[40])!=0.0)&&((vK)!=0.0));let vN=((km*vI)).exp();let vQ=(((sf[40])!=0.0)&&(!((vK)!=0.0)));let vS=((sf[447]*vI)).exp();let vT=(km-sf[447]);let vV=(d+(vI*vT));let vX=(if vQ{(vS*vV)}else{(if vL{vN}else{vo})});let wm=(ke/fF);let wo=(if (wm<sf[60]){d}else{b});let wp=(wm).exp();let wr=(!((wo)!=0.0));let ww=(if wr{(sf[207]*(d+(wm-sf[60])))}else{(if ((wo)!=0.0){wp}else{vX})});let wx=(kh/fF);let wz=(if (wx<sf[60]){d}else{b});let wA=(wx).exp();let wC=(!((wz)!=0.0));let wG=(if wC{(sf[207]*(d+(wx-sf[60])))}else{(if ((wz)!=0.0){wA}else{v5})});let wJ=((d+(jd*ww))).sqrt();let wM=((d+(jd*wG))).sqrt();let zO=(kb+kF);let zP=(if ((sf[7])!=0.0){zO}else{b});let zR=(if (zP>b){d}else{b});let zS=(((sf[7])!=0.0)&&((zR)!=0.0));let zT=(if zS{sf[183]}else{b});let zV=(d-(sf[181]*zT));let zZ=(sf[185]*zP);let A1=(d+(zZ/kX));let A6=(((sf[7])!=0.0)&&(!((zR)!=0.0)));let A8=(d-(kb/iu));let Aa=(d-f64::powf(A8,sf[184]));let Ad=(if A6{((iu*Aa)/sf[184])}else{(if zS{((iu*zV)/sf[184])}else{b})});let Ah=(if sb[51]{zO}else{b});let Ak=((sf[187]+(Ah*Ah))).sqrt();let Ap=(if sb[51]{((T*(Ah-(if sb[51]{Ak}else{b})))-kF)}else{b});let Ar=(d-(Ap/iu));let As=f64::powf(Ar,sf[184]);let Ax=(lo+(kb-Ap));let Ay=(sf[183]*Ax);let Az=(sf[185]*Ax);let AB=(d+(Az/kX));let AF=(if sb[51]{(((if sb[51]{((kD*As)/sf[184])}else{Ad})+(Ay*AB))-lu)}else{(if ((sf[7])!=0.0){(Ad+(if A6{b}else{(if zS{(zT*(zP*A1))}else{b})}))}else{b})});let AG=(km+lV);let AH=(if ((sf[1])!=0.0){AG}else{b});let AJ=(if (AH>b){d}else{b});let AK=(((sf[1])!=0.0)&&((AJ)!=0.0));let AL=(if AK{sf[189]}else{b});let AO=(d-(sf[181]*(sf[181]*AL)));let AS=(sf[191]*AH);let AU=(sf[181]+(AS/iV));let B0=(if (sb[8]&&(km<sf[192])){d}else{b});let B2=(((sf[1])!=0.0)&&(!((AJ)!=0.0)));let B3=(((B0)!=0.0)&&B2);let B5=(sf[190]*(sf[14]+km));let B7=(d-(B5/mu));let B9=(d-(mr*B7));
        let Be=(B2&&(!((B0)!=0.0)));let Bg=(d-(km/iV));let Bi=(d-f64::powf(Bg,sf[190]));let Bl=(if Be{((iV*Bi)/sf[190])}else{(if B3{((iV*B9)/sf[190])}else{(if AK{((iV*AO)/sf[190])}else{b})})});let Br=(lV+(sf[14]+(hU*km)));let Bt=(if sb[53]{(Br/mR)}else{b});let Bu=(hU*Bt);let Bv=(Bt-d);let By=((sf[194]+(Bv*Bv))).sqrt();let Bz=(d+Bt);let BC=((sf[196]+(Bz*Bz))).sqrt();let BD=(By+BC);let BF=(if sb[53]{(Bu/BD)}else{b});let BK=(if sb[53]{(T*(((mR*BF)-sf[14])-lV))}else{b});let BM=(d-(BK/iV));let BO=(d-f64::powf(BM,sf[190]));let BR=(if sb[53]{((iV*BO)/sf[190])}else{Bl});let BU=(if sb[53]{(T*(d+BF))}else{b});let BV=(d-BU);let BZ=(if sb[53]{((nU*BV)+(nY*BU))}else{b});let C1=(ne+(km-BK));let C7=(if sb[55]{AG}else{b});let Ca=((sf[194]+(C7*C7))).sqrt();let Cf=(if sb[55]{((T*(C7-(if sb[55]{Ca}else{b})))-lV)}else{BK});let Ch=(d-(Cf/iV));let Ci=f64::powf(Ch,sf[190]);let Cr=(if sb[55]{(((if sb[55]{((lU*Ci)/sf[190])}else{BR})+(sf[198]*(oj+(km-Cf))))-op)}else{(if sb[53]{((BR+(if sb[53]{(BZ*C1)}else{b}))-nl)}else{(if ((sf[1])!=0.0){(Bl+(if B2{b}else{(if AK{(AL*(AH*AU))}else{b})}))}else{b})})});let Ct=(if (p2>b){d}else{b});let Cv=(sf[73]*(p2*Ct));let Cw=(d+Cv);let Cx=(Cv/Cw);let CA=((sf[70]*ke)/1.44);let CC=(if (CA<sf[60]){d}else{b});let CD=(CA).exp();let CF=(!((CC)!=0.0));let CO=(sf[218]*(d+(px*sf[219])));let CQ=((if CF{(sf[207]*(d+(CA-sf[60])))}else{(if ((CC)!=0.0){CD}else{ww})})*sf[220]);let CS=(sf[74]+(Cx*Cx));let CV=(d+(Ct*(CQ*CS)));let CW=(CO*CV);let CZ=(p2*CW);let Dg=((kn-ko)*sf[223]);let Di=((kn-ks)*sf[224]);let Dk=(fk*sf[225]);let Dm=(kB*sf[226]);let Dp=((kC*sf[226])*0.3333333333333333);let Dq=(sf[58]*((sf[17]*(j0*lT))+(CZ/q8)));let Dr=(sf[58]*(sf[205]*(j0*AF)));let Ds=(sf[58]*(((j5*oL)+(pk*sf[221]))+(wJ*sf[222])));let Dt=(sf[58]*(wM*sf[222]));let Du=(sf[58]*((j7*Cr)+((if sb[57]{b}else{qM})*sf[221])));let Dv=(if ((fo)!=0.0){fr}else{d});let Dz=(if fx{(-(fA*(-Dv)))}else{Dv});let DB=((bB*Dz)/bD);let DC=(Dz/sf[76]);let EG=(-DC);let EH=(sf[90]*EG);let ER=((gI*(sf[82]*(DC*(sf[88]*f64::powf(fG,sf[236])))))+(gD*(gI*(((gG*EH)-(gF*(sf[87]*DB)))/(gG*gG)))));let Fe=(sf[109]*DB);let Fi=(gU*gU);let Fu=(sf[117]*DB);let Fy=(h1*h1);let FK=(sf[123]*DB);let FO=(h8*h8);let G0=(sf[128]*DB);let G4=(hf*hf);let Gg=(sf[133]*DB);let Gk=(hm*hm);let Gy=(sf[154]*Dz);let GR=(hU*(((fG*DB)-(fF*DC))/(fG*fG)));let GW=(fF*fF);let Hh=((id*(ib*DB))+(ic*(DC/fG)));let Hk=((((i9*DC)+(fG*((i8_*GR)+(hW*(((i1*(((fF*(sf[163]*DC))-(hZ*DB))/GW))-(i6*(((fF*(sf[164]*DC))-(i4*DB))/GW)))/i7)))))-Hh)-(sf[119]*DC));let Hl=(hU*DB);let HA=(Hk+((is*Hl)+(ij*((T*((ca*(im*(((fF*(-Hk))-(ik*DB))/GW)))/(hU*ip)))/ir))));let HX=((((iG*DC)+(fG*((iF*GR)+(hW*(((iz*(((fF*(sf[166]*DC))-(ix*DB))/GW))-(iD*(((fF*(sf[167]*DC))-(iB*DB))/GW)))/iE)))))-Hh)-(sf[130]*DC));let Ic=(HX+((iT*Hl)+(ij*((T*((ca*(iN*(((fF*(-HX))-(iL*DB))/GW)))/(hU*iQ)))/iS))));let If=(iu*iu);let Il=(sf[168]*(((-(sf[162]*HA))/If)*(sf[169]*f64::powf(iX,sf[243]))));let Io=(iV*iV);let Is=(((-(sf[165]*Ic))/Io)*(sf[171]*f64::powf(j2,sf[210])));let IH=((jc*(sf[173]*(DC*(sf[86]*f64::powf(fG,sf[244])))))+(ja*(jc*(((fF*EH)-(gF*DB))/GW))));let IN=(-(sf[4]*((hM*Dz)+(fH*(sf[160]*Dz)))));let IQ=((hT*DB)+(fF*(sf[83]*(sf[161]*Dz))));let IU=(jj*jj);let JK=(-HA);let JL=(sf[180]*JK);let JM=(if ((sf[7])!=0.0){JL}else{b});let JV=(sf[181]*HA);let JW=(kX*(sf[185]*JM));let JZ=(kX*kX);let K1=(sf[249]/kX);let K2=(sf[250]/kX);let Ko=(-(sf[58]/iu));let Kp=(-(sf[246]/iu));let Ks=(sf[184]*f64::powf(l6,sf[251]));let KH=(if l4{(((l8*HA)+(iu*(-((-((-(k8*HA))/If))*Ks))))/sf[184])}else{(if kK{((kQ*HA)/sf[184])}else{b})});let KI=(if l4{((iu*(-(Ko*Ks)))/sf[184])}else{b});let KJ=(if l4{((iu*(-(Kp*Ks)))/sf[184])}else{b});let KT=(kF*JL);let L0=(if sb[51]{(i2*(JL+(if sb[51]{((KT+KT)/(hU*lk))}else{b})))}else{b});let Ld=(if sb[51]{(((lr*JK)+(kD*((-(((iu*L0)-(lo*HA))/If))*(sf[184]*f64::powf(lq,sf[251])))))/sf[184])}else{b});let Le=(if sb[51]{JL}else{b});let Lh=(lv*Le);let Lj=(lv*sf[252]);let Ll=(lv*sf[253]);let Ln=(hU*ly);let LB=(if sb[51]{((T*(Le-(if sb[51]{((Lh+Lh)/Ln)}else{b})))-JL)}else{b});let LC=(if sb[51]{(T*(sf[252]-(if sb[51]{((Lj+Lj)/Ln)}else{b})))}else{b});
        let LD=(if sb[51]{(T*(sf[253]-(if sb[51]{((Ll+Ll)/Ln)}else{b})))}else{b});let LO=(sf[184]*f64::powf(lF,sf[251]));let M4=(sf[58]-LC);let M5=(sf[246]-LD);let M6=(L0+(-LB));let Mw=(if sb[51]{(((if sb[51]{(((lG*JK)+(kD*((-(((iu*LB)-(lD*HA))/If))*LO)))/sf[184])}else{KH})+((lP*(sf[183]*M6))+(lM*(((kX*(sf[185]*M6))-(lN*JV))/JZ))))-Ld)}else{(if ((sf[7])!=0.0){(KH+(if l4{b}else{(if kK{(kO*((kZ*JM)+(kH*((JW-(kW*JV))/JZ))))}else{b})}))}else{b})});let Mx=(if sb[51]{((if sb[51]{((kD*((-(LC/iu))*LO))/sf[184])}else{KI})+((lP*(sf[183]*M4))+(lM*((sf[185]*M4)/kX))))}else{(if ((sf[7])!=0.0){(KI+(if l4{b}else{(if kK{(kO*((kZ*sf[247])+(kH*K1)))}else{b})}))}else{b})});let My=(if sb[51]{((if sb[51]{((kD*((-(LD/iu))*LO))/sf[184])}else{KJ})+((lP*(sf[183]*M5))+(lM*((sf[185]*M5)/kX))))}else{(if ((sf[7])!=0.0){(KJ+(if l4{b}else{(if kK{(kO*((kZ*sf[248])+(kH*K2)))}else{b})}))}else{b})});let Mz=(-Ic);let MA=(sf[180]*Mz);let MB=(if ((sf[1])!=0.0){MA}else{b});let MK=(iV*(sf[191]*MB));let MO=(sf[256]/iV);let MP=(sf[257]/iV);let N7=((-(sf[14]*Ic))/Io);let Nb=(N7*(sf[190]*f64::powf(mq,sf[258])));let Ng=(mu*mu);let NB=((iV*(-(mr*(-(sf[259]/mu)))))/sf[190]);let NC=((iV*(-(mr*(-(sf[260]/mu)))))/sf[190]);let NM=(-(sf[246]/iV));let NN=(-(sf[58]/iV));let NP=(sf[190]*f64::powf(mF,sf[258]));let O4=(if mD{(((mH*Ic)+(iV*(-((-((-(ke*Ic))/Io))*NP))))/sf[190])}else{(if mo{(((my*Ic)+(iV*(-((mw*Nb)+(mr*(-((-(mt*Ic))/Ng)))))))/sf[190])}else{(if m0{((m6*Ic)/sf[190])}else{b})})});let O5=(if mD{((iV*(-(NM*NP)))/sf[190])}else{(if mo{NB}else{b})});let O6=(if mD{((iV*(-(NN*NP)))/sf[190])}else{(if mo{NC}else{b})});let Og=(-MA);let Oh=(mR*MA);let Ok_=(mR*mR);let Om=(if sb[53]{((Oh-(mQ*Og))/Ok_)}else{b});let Oo=(mV*Om);let Os=(n1*Om);let OI=(if sb[53]{(T*(((n9*Og)+(mR*(if sb[53]{(((n7*(hU*Om))-(mU*(((Oo+Oo)/(hU*n0))+((Os+Os)/(hU*n6)))))/(n7*n7))}else{b})))-MA))}else{b});let OW=(if sb[53]{(((ni*Ic)+(iV*(-((-(((iV*OI)-(ne*Ic))/Io))*(sf[190]*f64::powf(ng,sf[258]))))))/sf[190])}else{b});let P4=(if sb[53]{((Oh-(no*Og))/Ok_)}else{b});let P5=(if sb[53]{(sf[261]/mR)}else{b});let P6=(if sb[53]{(sf[262]/mR)}else{b});let P8=(hU*P5);let P9=(hU*P6);let Pa=(ns*P4);let Pc=(ns*P5);let Pe=(ns*P6);let Pg=(hU*nv);let Pk=(nw*P4);let Pm=(nw*P5);let Po=(nw*P6);let Pq=(hU*nz);let PA=(nA*nA);let PK=(if sb[53]{(((nA*(hU*P4))-(nr*(((Pa+Pa)/Pg)+((Pk+Pk)/Pq))))/PA)}else{b});let PL=(if sb[53]{(((nA*P8)-(nr*(((Pc+Pc)/Pg)+((Pm+Pm)/Pq))))/PA)}else{b});let PM=(if sb[53]{(((nA*P9)-(nr*(((Pe+Pe)/Pg)+((Po+Po)/Pq))))/PA)}else{b});let PW=(if sb[53]{(T*(((nC*Og)+(mR*PK))-MA))}else{b});let PX=(if sb[53]{(T*(mR*PL))}else{b});let PY=(if sb[53]{(T*(mR*PM))}else{b});let Q9=(sf[190]*f64::powf(nJ,sf[258]));let Qo=(if sb[53]{(((nL*Ic)+(iV*(-((-(((iV*PW)-(nH*Ic))/Io))*Q9))))/sf[190])}else{O4});let Qp=(if sb[53]{((iV*(-((-(PX/iV))*Q9)))/sf[190])}else{O5});let Qq=(if sb[53]{((iV*(-((-(PY/iV))*Q9)))/sf[190])}else{O6});let Qu=(if sb[53]{(T*PK)}else{b});let Qv=(if sb[53]{(T*PL)}else{b});let Qw=(if sb[53]{(T*PM)}else{b});let QB=(if sb[53]{(N7*(sf[197]*f64::powf(mq,sf[263])))}else{b});let QJ=(if sb[53]{((((iV*MA)-(lV*Ic))/Io)*(sf[197]*f64::powf(nW,sf[263])))}else{b});let Rq=(lV*MA);let Rx=(if sb[55]{(i2*(MA+(if sb[55]{((Rq+Rq)/(hU*of))}else{b})))}else{OI});let RK=(if sb[55]{(((om*Mz)+(lU*((-(((iV*Rx)-(oj*Ic))/Io))*(sf[190]*f64::powf(ol,sf[258])))))/sf[190])}else{b});let RL=(if sb[55]{MA}else{b});let RO=(oq*RL);let RQ=(oq*sf[264]);let RS=(oq*sf[265]);let RU=(hU*ot);let S8=(if sb[55]{((T*(RL-(if sb[55]{((RO+RO)/RU)}else{b})))-MA)}else{PW});let S9=(if sb[55]{(T*(sf[264]-(if sb[55]{((RQ+RQ)/RU)}else{b})))}else{PX});let Sa=(if sb[55]{(T*(sf[265]-(if sb[55]{((RS+RS)/RU)}else{b})))}else{PY});let Sl=(sf[190]*f64::powf(oA,sf[258]));let SL=(if sb[55]{(((if sb[55]{(((oB*Mz)+(lU*((-(((iV*S8)-(oy*Ic))/Io))*Sl)))/sf[190])}else{Qo})+(sf[198]*(Rx+(-S8))))-RK)}else{(if sb[53]{((Qo+(if sb[53]{((o5*(if sb[53]{(((nZ*QB)+(nU*(-Qu)))+((nY*Qu)+(nR*QJ)))}else{b}))+(o3*(OI+(-PW))))}else{b}))-OW)}else{(if ((sf[1])!=0.0){(O4+(if mn{b}else{(if m0{(m3*((me*MB)+(lX*((MK-(mc*Ic))/Io))))}else{b})}))}else{b})})});
        let SM=(if sb[55]{((if sb[55]{((lU*((-(S9/iV))*Sl))/sf[190])}else{Qp})+(sf[198]*(sf[246]-S9)))}else{(if sb[53]{(Qp+(if sb[53]{((o5*(if sb[53]{((nU*(-Qv))+(nY*Qv))}else{b}))+(o3*(sf[246]-PX)))}else{b}))}else{(if ((sf[1])!=0.0){(O5+(if mn{b}else{(if m0{(m3*((me*sf[254])+(lX*MO)))}else{b})}))}else{b})})});let SN=(if sb[55]{((if sb[55]{((lU*((-(Sa/iV))*Sl))/sf[190])}else{Qq})+(sf[198]*(sf[58]-Sa)))}else{(if sb[53]{(Qq+(if sb[53]{((o5*(if sb[53]{((nU*(-Qw))+(nY*Qw))}else{b}))+(o3*(sf[58]-PY)))}else{b}))}else{(if ((sf[1])!=0.0){(O6+(if mn{b}else{(if m0{(m3*((me*sf[255])+(lX*MP)))}else{b})}))}else{b})})});let ST=((-((hx*DB)+(fF*(sf[87]*Gy))))/(oM*oM));let SV=(sf[58]*oN);let SW=(oN*sf[246]);let Tb=(if oT{((oY*(oV*(sf[343]*ST)))+(oV*(oW*ST)))}else{(if ((oP)!=0.0){(oR*(k8*ST))}else{b})});let Tc=(if oT{(oV*SV)}else{(if ((oP)!=0.0){(oR*SV)}else{b})});let Td=(if oT{(oV*SW)}else{(if ((oP)!=0.0){(oR*SW)}else{b})});let Tg=((p1*ER)+(gJ*Tb));let Th=(gJ*Tc);let Ti=(gJ*Td);let To=((-((hy*DB)+(fF*(sf[101]*Gy))))/(p3*p3));let Tq=(p4*sf[246]);let Tr=(sf[58]*p4);let TH=(if pa{((pf*(pc*(sf[363]*To)))+(pc*(pd*To)))}else{(if ((p6)!=0.0){(p8*(ke*To))}else{Tb})});let TI=(if pa{(pc*Tq)}else{(if ((p6)!=0.0){(p8*Tq)}else{b})});let TJ=(if pa{(pc*Tr)}else{(if ((p6)!=0.0){(p8*Tr)}else{Tc})});let TK=(if pa{b}else{(if ((p6)!=0.0){b}else{Td})});let TQ=((pj*((gQ*ER)+(gJ*((gP*(sf[99]*(DC*(sf[102]*f64::powf(fG,sf[237])))))+(gL*(gP*(((gN*(sf[104]*EG))-(gM*(sf[101]*DB)))/(gN*gN))))))))+(pi*TH));let TR=(pi*TI);let TS=(pi*TJ);let TT=(pi*TK);let TY=(jY*My);let U2=(jV*SM);let U4=(((lT*(if jW{((-(sf[178]*(sf[179]*Dz)))/(jv*jv))}else{b}))+(jY*Mw))+((oL*(if jT{((-(sf[176]*(sf[177]*Dz)))/(jq*jq))}else{b}))+(jV*SL)));let U5=((jY*Mx)+(jV*SN));let U6=(pq*U4);let U8=(pq*U2);let Ua=(pq*U5);let Uc=(pq*TY);let Ue=(hU*pu);let Un=(T*(U4+((U6+U6)/Ue)));let Uo=(T*(U2+((U8+U8)/Ue)));let Up=(T*(U5+((Ua+Ua)/Ue)));let Uq=(T*(TY+((Uc+Uc)/Ue)));let UF=(sf[199]*f64::powf(px,sf[266]));let UK=(ca*(((p2*(if jZ{((-(sf[52]*(DC*(sf[137]*f64::powf(fG,sf[227])))))/(fK*fK))}else{b}))+(k1*Tg))+(sf[62]*TQ)));let UL=(ca*(sf[62]*TR));let UM=(ca*((k1*Th)+(sf[62]*TS)));let UN=(ca*((k1*Ti)+(sf[62]*TT)));let US=(if ((sf[29])!=0.0){((Un*UF)+UK)}else{b});let UT=(if ((sf[29])!=0.0){((Uo*UF)+UL)}else{b});let UU=(if ((sf[29])!=0.0){((Up*UF)+UM)}else{b});let UV=(if ((sf[29])!=0.0){((Uq*UF)+UN)}else{b});let UY=(sf[93]*f64::powf(pF,sf[267]));let Vf=(T*Un);let Vg=(T*Uo);let Vh=(T*Up);let Vi=(T*Uq);let Vn=(if sb[56]{UK}else{US});let Vo=(if sb[56]{UL}else{UT});let Vp=(if sb[56]{UM}else{UU});let Vq=(if sb[56]{UN}else{UV});let Vs=(sf[93]*f64::powf(pV,sf[267]));let VR=(if q5{(sf[201]*Vf)}else{(if pY{((q1*Vf)+(pZ*(Vn*Vs)))}else{(if pO{Vf}else{(if pI{(T*(Un+(US*UY)))}else{b})})})});let VS=(if q5{(sf[201]*Vg)}else{(if pY{((q1*Vg)+(pZ*(Vo*Vs)))}else{(if pO{Vg}else{(if pI{(T*(Uo+(UT*UY)))}else{b})})})});let VT=(if q5{(sf[201]*Vh)}else{(if pY{((q1*Vh)+(pZ*(Vp*Vs)))}else{(if pO{Vh}else{(if pI{(T*(Up+(UU*UY)))}else{b})})})});let VU=(if q5{(sf[201]*Vi)}else{(if pY{((q1*Vi)+(pZ*(Vq*Vs)))}else{(if pO{Vi}else{(if pI{(T*(Uq+(UV*UY)))}else{b})})})});let VY=(q8*q8);let Wt=(if ((sf[32])!=0.0){((-Fe)/Fi)}else{To});let Wv=(sf[58]*qc);let Ww=(qc*sf[246]);let WH=(qm*(sf[382]*Wt));let WM=(qm*Wv);let WN=(qm*Ww);let WO=(if qk{((qp*WH)+(qm*(qn*Wt)))}else{(if qf{(qh*(km*Wt))}else{TH})});let WP=(if qk{b}else{(if qf{b}else{TI})});let WQ=(if qk{WM}else{(if qf{(qh*Wv)}else{b})});let WR=(if qk{b}else{(if qf{b}else{TJ})});let WS=(if qk{b}else{(if qf{b}else{TK})});let WT=(if qk{WN}else{(if qf{(qh*Ww)}else{b})});let X5=(if qz{((qC*WH)+(qm*(qA*Wt)))}else{(if qu{(qw*(ke*Wt))}else{b})});let X6=(if qz{WN}else{(if qu{(qw*Ww)}else{b})});let X7=(if qz{WM}else{(if qu{(qw*Wv)}else{b})});let Xs=(if ((sf[32])!=0.0){((qK*((gW*(sf[31]*(DC*(sf[110]*f64::powf(fG,sf[238])))))+(gS*(gW*(((gU*(sf[112]*EG))-(gT*Fe))/Fi)))))+(gX*((sf[202]*WO)+(sf[203]*X5))))}else{b});let Xt=(if ((sf[32])!=0.0){(gX*((sf[202]*WP)+(sf[203]*X6)))}else{b});let Xu=(if ((sf[32])!=0.0){(gX*(sf[202]*WQ))}else{b});let Xv=(if ((sf[32])!=0.0){(gX*((sf[202]*WR)+(sf[203]*X7)))}else{b});
        let Xw=(if ((sf[32])!=0.0){(gX*(sf[202]*WS))}else{b});let Xx=(if ((sf[32])!=0.0){(gX*(sf[202]*WT))}else{b});let Yy=((-Fu)/Fy);let Yz=(if ((sf[18])!=0.0){Yy}else{Wt});let YB=(sf[58]*r7);let YC=(r7*sf[246]);let YU=(if rf{((rk*(rh*(sf[396]*Yz)))+(rh*(ri*Yz)))}else{(if ra{(rc*(k8*Yz))}else{WO})});let YV=(if rf{b}else{(if ra{b}else{WP})});let YW=(if rf{b}else{(if ra{b}else{WQ})});let YX=(if rf{(rh*YB)}else{(if ra{(rc*YB)}else{WR})});let YY=(if rf{(rh*YC)}else{(if ra{(rc*YC)}else{WS})});let YZ=(if rf{b}else{(if ra{b}else{WT})});let Z1=((-FK)/FO);let Z2=(if ((sf[18])!=0.0){Z1}else{Yz});let a0g=(if sb[61]{IN}else{b});let a0k=((-IQ)/IU);let a0l=(if sb[61]{a0k}else{Z2});let a0m=(rY*a0g);let a0p=(rY*sf[268]);let a0q=(rY*sf[269]);let a0H=(if s6{((sb_*(s8*(sf[323]*a0l)))+(s8*(a0m+(s9*a0l))))}else{(if s1{(s3*(a0m+(rW*a0l)))}else{X5})});let a0I=(if s6{b}else{(if s1{b}else{X6})});let a0J=(if s6{(s8*a0p)}else{(if s1{(s3*a0p)}else{X7})});let a0K=(if s6{(s8*a0q)}else{(if s1{(s3*a0q)}else{b})});let a14=(if sb[63]{Yy}else{a0l});let a16=(sf[58]*sl);let a17=(sl*sf[246]);let a1p=(if st{((sy*(sv*(sf[396]*a14)))+(sv*(sw*a14)))}else{(if so{(sq*(kb*a14))}else{YU})});let a1q=(if st{b}else{(if so{b}else{YV})});let a1r=(if st{(sv*a16)}else{(if so{(sq*a16)}else{YW})});let a1s=(if st{b}else{(if so{b}else{YX})});let a1t=(if st{(sv*a17)}else{(if so{(sq*a17)}else{YY})});let a1u=(if st{b}else{(if so{b}else{YZ})});let a1v=(if sb[63]{Z1}else{a14});let a2g=(if sb[64]{IN}else{a0g});let a2j=(if sb[64]{a0k}else{a1v});let a2k=(sZ*a2g);let a2n=(sZ*sf[270]);let a2o=(sZ*sf[271]);let a2F=(if t7{((tc*(t9*(sf[323]*a2j)))+(t9*(a2k+(ta*a2j))))}else{(if t2{(t4*(a2k+(sY*a2j)))}else{a0H})});let a2G=(if t7{b}else{(if t2{b}else{a0I})});let a2H=(if t7{(t9*a2n)}else{(if t2{(t4*a2n)}else{a0J})});let a2I=(if t7{(t9*a2o)}else{(if t2{(t4*a2o)}else{a0K})});let a2W=(if sb[66]{Yy}else{a2j});let a2Y=(sf[58]*tl);let a2Z=(tl*sf[246]);let a3h=(if tq{((tu*(ts*(sf[396]*a2W)))+(ts*(ri*a2W)))}else{(if tm{(to*(k8*a2W))}else{a1p})});let a3i=(if tq{b}else{(if tm{b}else{a1q})});let a3j=(if tq{b}else{(if tm{b}else{a1r})});let a3k=(if tq{(ts*a2Y)}else{(if tm{(to*a2Y)}else{a1s})});let a3l=(if tq{(ts*a2Z)}else{(if tm{(to*a2Z)}else{a1t})});let a3m=(if tq{b}else{(if tm{b}else{a1u})});let a3n=(if sb[66]{Z1}else{a2W});let a4I=(if sb[69]{IN}else{a2g});let a4L=(if sb[69]{a0k}else{a3n});let a4M=(tY*a4I);let a4P=(tY*sf[272]);let a4Q=(tY*sf[273]);let a57=(if u6{((ub*(u8_*(sf[323]*a4L)))+(u8_*(a4M+(u9*a4L))))}else{(if u1{(u3*(a4M+(tX*a4L)))}else{a2F})});let a58=(if u6{b}else{(if u1{b}else{a2G})});let a59=(if u6{(u8_*a4P)}else{(if u1{(u3*a4P)}else{a2H})});let a5a=(if u6{(u8_*a4Q)}else{(if u1{(u3*a4Q)}else{a2I})});let a5o=(if sb[66]{Yy}else{a4L});let a5q=(sf[58]*uj);let a5r=(uj*sf[246]);let a5J=(if uo{((us*(uq*(sf[396]*a5o)))+(uq*(sw*a5o)))}else{(if uk{(um*(kb*a5o))}else{a3h})});let a5K=(if uo{b}else{(if uk{b}else{a3i})});let a5L=(if uo{(uq*a5q)}else{(if uk{(um*a5q)}else{a3j})});let a5M=(if uo{b}else{(if uk{b}else{a3k})});let a5N=(if uo{(uq*a5r)}else{(if uk{(um*a5r)}else{a3l})});let a5O=(if uo{b}else{(if uk{b}else{a3m})});let a5P=(if sb[66]{Z1}else{a5o});let a6J=(if sb[69]{a0k}else{a5P});let a6K=(uQ*(if sb[69]{IN}else{a4I}));let a6N=(uQ*sf[274]);let a6O=(uQ*sf[275]);let a75=(if uY{((v3*(v0*(sf[323]*a6J)))+(v0*(a6K+(v1*a6J))))}else{(if uT{(uV*(a6K+(uP*a6J)))}else{a57})});let a76=(if uY{b}else{(if uT{b}else{a58})});let a77=(if uY{(v0*a6N)}else{(if uT{(uV*a6N)}else{a59})});let a78=(if uY{(v0*a6O)}else{(if uT{(uV*a6O)}else{a5a})});let a7n=((-G0)/G4);let a7p=(vb*sf[246]);let a7q=(sf[58]*vb);let a7I=(if vh{((vm*(vj*(sf[424]*a7n)))+(vj*(vk*a7n)))}else{(if ((vd)!=0.0){(vf*(ke*a7n))}else{a5J})});let a7J=(if vh{(vj*a7p)}else{(if ((vd)!=0.0){(vf*a7p)}else{a5K})});let a7K=(if vh{b}else{(if ((vd)!=0.0){b}else{a5L})});let a7L=(if vh{(vj*a7q)}else{(if ((vd)!=0.0){(vf*a7q)}else{a5M})});let a7M=(if vh{b}else{(if ((vd)!=0.0){b}else{a5N})});let a7N=(if vh{b}else{(if ((vd)!=0.0){b}else{a5O})});let a7P=((-Gg)/Gk);let a8y=(if ((sf[40])!=0.0){a7n}else{a7P});let a8A=(sf[58]*vI);let a8B=(vI*sf[246]);
        let a8T=(if vQ{((vV*(vS*(sf[447]*a8y)))+(vS*(vT*a8y)))}else{(if vL{(vN*(km*a8y))}else{a7I})});let a8U=(if vQ{b}else{(if vL{b}else{a7J})});let a8V=(if vQ{(vS*a8A)}else{(if vL{(vN*a8A)}else{a7K})});let a8W=(if vQ{b}else{(if vL{b}else{a7L})});let a8X=(if vQ{b}else{(if vL{b}else{a7M})});let a8Y=(if vQ{(vS*a8B)}else{(if vL{(vN*a8B)}else{a7N})});let aa0=((-(ke*DB))/GW);let aa1=(sf[246]/fF);let aa2=(sf[58]/fF);let aad=(sf[207]*aa1);let aae=(sf[207]*aa2);let aaf=(if wr{(sf[207]*aa0)}else{(if ((wo)!=0.0){(wp*aa0)}else{a8T})});let aag=(if wr{aad}else{(if ((wo)!=0.0){(wp*aa1)}else{a8U})});let aah=(if wr{b}else{(if ((wo)!=0.0){b}else{a8V})});let aai=(if wr{aae}else{(if ((wo)!=0.0){(wp*aa2)}else{a8W})});let aaj=(if wr{b}else{(if ((wo)!=0.0){b}else{a8X})});let aak=(if wr{b}else{(if ((wo)!=0.0){b}else{a8Y})});let aan=((-(kh*DB))/GW);let aaK=(hU*wJ);let aaL=(((ww*IH)+(jd*aaf))/aaK);let aaM=((jd*aag)/aaK);let aaN=((jd*aah)/aaK);let aaO=((jd*aai)/aaK);let aaP=((jd*aaj)/aaK);let aaQ=((jd*aak)/aaK);let aaY=(hU*wM);let aaZ=(((wG*IH)+(jd*(if wC{(sf[207]*aan)}else{(if ((wz)!=0.0){(wA*aan)}else{a75})})))/aaY);let ab0=((jd*(if wC{aad}else{(if ((wz)!=0.0){(wA*aa1)}else{b})}))/aaY);let ab1=((jd*(if wC{b}else{(if ((wz)!=0.0){b}else{a76})}))/aaY);let ab2=((jd*(if wC{aae}else{(if ((wz)!=0.0){(wA*aa2)}else{a77})}))/aaY);let ab3=((jd*(if wC{b}else{(if ((wz)!=0.0){b}else{a78})}))/aaY);let amd=(sf[184]*f64::powf(A8,sf[251]));let ams=(if A6{(((Aa*HA)+(iu*(-((-((-(kb*HA))/If))*amd))))/sf[184])}else{(if zS{((zV*HA)/sf[184])}else{b})});let amt=(if A6{((iu*(-(Ko*amd)))/sf[184])}else{b});let amu=(if A6{((iu*(-(Kp*amd)))/sf[184])}else{b});let amE=(Ah*Le);let amG=(Ah*sf[252]);let amI=(Ah*sf[253]);let amK=(hU*Ak);let amY=(if sb[51]{((T*(Le-(if sb[51]{((amE+amE)/amK)}else{b})))-JL)}else{b});let amZ=(if sb[51]{(T*(sf[252]-(if sb[51]{((amG+amG)/amK)}else{b})))}else{b});let an0=(if sb[51]{(T*(sf[253]-(if sb[51]{((amI+amI)/amK)}else{b})))}else{b});let anb=(sf[184]*f64::powf(Ar,sf[251]));let anr=(sf[58]-amZ);let ans=(sf[246]-an0);let ant=(L0+(-amY));let aoB=(sf[190]*f64::powf(Bg,sf[258]));let aoQ=(if Be{(((Bi*Ic)+(iV*(-((-((-(km*Ic))/Io))*aoB))))/sf[190])}else{(if B3{(((B9*Ic)+(iV*(-((B7*Nb)+(mr*(-((-(B5*Ic))/Ng)))))))/sf[190])}else{(if AK{((AO*Ic)/sf[190])}else{b})})});let aoR=(if Be{((iV*(-(NN*aoB)))/sf[190])}else{(if B3{NC}else{b})});let aoS=(if Be{((iV*(-(NM*aoB)))/sf[190])}else{(if B3{NB}else{b})});let ap5=(if sb[53]{((Oh-(Br*Og))/Ok_)}else{b});let ap7=(Bv*ap5);let ap9=(Bv*P6);let apb=(Bv*P5);let apd=(hU*By);let aph=(Bz*ap5);let apj=(Bz*P6);let apl=(Bz*P5);let apn=(hU*BC);let apx=(BD*BD);let apH=(if sb[53]{(((BD*(hU*ap5))-(Bu*(((ap7+ap7)/apd)+((aph+aph)/apn))))/apx)}else{b});let apI=(if sb[53]{(((BD*P9)-(Bu*(((ap9+ap9)/apd)+((apj+apj)/apn))))/apx)}else{b});let apJ=(if sb[53]{(((BD*P8)-(Bu*(((apb+apb)/apd)+((apl+apl)/apn))))/apx)}else{b});let apT=(if sb[53]{(T*(((BF*Og)+(mR*apH))-MA))}else{b});let apU=(if sb[53]{(T*(mR*apI))}else{b});let apV=(if sb[53]{(T*(mR*apJ))}else{b});let aq6=(sf[190]*f64::powf(BM,sf[258]));let aql=(if sb[53]{(((BO*Ic)+(iV*(-((-(((iV*apT)-(BK*Ic))/Io))*aq6))))/sf[190])}else{aoQ});let aqm=(if sb[53]{((iV*(-((-(apU/iV))*aq6)))/sf[190])}else{aoR});let aqn=(if sb[53]{((iV*(-((-(apV/iV))*aq6)))/sf[190])}else{aoS});let aqr=(if sb[53]{(T*apH)}else{b});let aqs=(if sb[53]{(T*apI)}else{b});let aqt=(if sb[53]{(T*apJ)}else{b});let ara=(C7*RL);let arc=(C7*sf[265]);let are=(C7*sf[264]);let arg=(hU*Ca);let aru=(if sb[55]{((T*(RL-(if sb[55]{((ara+ara)/arg)}else{b})))-MA)}else{apT});let arv=(if sb[55]{(T*(sf[265]-(if sb[55]{((arc+arc)/arg)}else{b})))}else{apU});let arw=(if sb[55]{(T*(sf[264]-(if sb[55]{((are+are)/arg)}else{b})))}else{apV});let arH=(sf[190]*f64::powf(Ch,sf[258]));let asd=(sf[73]*(Ct*Tg));let ase=(sf[73]*(Ct*Th));let asf=(sf[73]*(Ct*Ti));let asj=(Cw*Cw);let at1=(Cx*(((Cw*asd)-(Cv*asd))/asj));let at3=(Cx*(((Cw*ase)-(Cv*ase))/asj));let at5=(Cx*(((Cw*asf)-(Cv*asf))/asj));
        let av8=(sf[58]*((sf[17]*((lT*Il)+(j0*Mw)))+(((q8*((CW*Tg)+(p2*((CV*(sf[218]*(sf[219]*Un)))+(CO*(Ct*((CS*(sf[220]*(if CF{b}else{(if ((CC)!=0.0){b}else{aaf})})))+(CQ*(at1+at1)))))))))-(CZ*VR))/VY)));let av9=(sf[58]*(((q8*(p2*((CV*(sf[218]*(sf[219]*Uo)))+(CO*(Ct*(CS*(sf[220]*(if CF{sf[293]}else{(if ((CC)!=0.0){(CD*sf[291])}else{aag})}))))))))-(CZ*VS))/VY));let ava=(sf[58]*((p2*(CO*(Ct*(CS*(sf[220]*(if CF{b}else{(if ((CC)!=0.0){b}else{aah})}))))))/q8));let avb=(sf[58]*((sf[17]*(j0*Mx))+(((q8*((CW*Th)+(p2*((CV*(sf[218]*(sf[219]*Up)))+(CO*(Ct*((CS*(sf[220]*(if CF{sf[294]}else{(if ((CC)!=0.0){(CD*sf[292])}else{aai})})))+(CQ*(at3+at3)))))))))-(CZ*VT))/VY)));let avc=(sf[58]*((sf[17]*(j0*My))+(((q8*((CW*Ti)+(p2*((CV*(sf[218]*(sf[219]*Uq)))+(CO*(Ct*((CS*(sf[220]*(if CF{b}else{(if ((CC)!=0.0){b}else{aaj})})))+(CQ*(at5+at5)))))))))-(CZ*VU))/VY)));let avd=(sf[58]*((p2*(CO*(Ct*(CS*(sf[220]*(if CF{b}else{(if ((CC)!=0.0){b}else{aak})}))))))/q8));let ave=(sf[58]*(sf[205]*((AF*Il)+(j0*(if sb[51]{(((if sb[51]{(((As*JK)+(kD*((-(((iu*amY)-(Ap*HA))/If))*anb)))/sf[184])}else{ams})+((AB*(sf[183]*ant))+(Ay*(((kX*(sf[185]*ant))-(Az*JV))/JZ))))-Ld)}else{(if ((sf[7])!=0.0){(ams+(if A6{b}else{(if zS{(zT*((A1*JM)+(zP*((JW-(zZ*JV))/JZ))))}else{b})}))}else{b})})))));let avf=(sf[58]*(sf[205]*(j0*(if sb[51]{((if sb[51]{((kD*((-(amZ/iu))*anb))/sf[184])}else{amt})+((AB*(sf[183]*anr))+(Ay*((sf[185]*anr)/kX))))}else{(if ((sf[7])!=0.0){(amt+(if A6{b}else{(if zS{(zT*((A1*sf[247])+(zP*K1)))}else{b})}))}else{b})}))));let avg=(sf[58]*(sf[205]*(j0*(if sb[51]{((if sb[51]{((kD*((-(an0/iu))*anb))/sf[184])}else{amu})+((AB*(sf[183]*ans))+(Ay*((sf[185]*ans)/kX))))}else{(if ((sf[7])!=0.0){(amu+(if A6{b}else{(if zS{(zT*((A1*sf[248])+(zP*K2)))}else{b})}))}else{b})}))));let avh=(sf[58]*((((oL*(sf[170]*Is))+(j5*SL))+(sf[221]*TQ))+(sf[222]*aaL)));let avi=(sf[58]*(((j5*SM)+(sf[221]*TR))+(sf[222]*aaM)));let avj=(sf[58]*(sf[222]*aaN));let avk=(sf[58]*(((j5*SN)+(sf[221]*TS))+(sf[222]*aaO)));let avl=(sf[58]*((sf[221]*TT)+(sf[222]*aaP)));let avm=(sf[58]*(sf[222]*aaQ));let avn=(sf[58]*(sf[222]*aaZ));let avo=(sf[58]*(sf[222]*ab0));let avp=(sf[58]*(sf[222]*ab1));let avq=(sf[58]*(sf[222]*ab2));let avr=(sf[58]*(sf[222]*ab3));let avs=(sf[58]*(((Cr*(sf[172]*Is))+(j7*(if sb[55]{(((if sb[55]{(((Ci*Mz)+(lU*((-(((iV*aru)-(Cf*Ic))/Io))*arH)))/sf[190])}else{aql})+(sf[198]*(Rx+(-aru))))-RK)}else{(if sb[53]{((aql+(if sb[53]{((C1*(if sb[53]{(((BV*QB)+(nU*(-aqr)))+((BU*QJ)+(nY*aqr)))}else{b}))+(BZ*(OI+(-apT))))}else{b}))-OW)}else{(if ((sf[1])!=0.0){(aoQ+(if B2{b}else{(if AK{(AL*((AU*MB)+(AH*((MK-(AS*Ic))/Io))))}else{b})}))}else{b})})})))+(sf[221]*(if sb[57]{b}else{Xs}))));let avt=(sf[58]*(sf[221]*(if sb[57]{b}else{Xt})));let avu=(sf[58]*((j7*(if sb[55]{((if sb[55]{((lU*((-(arv/iV))*arH))/sf[190])}else{aqm})+(sf[198]*(sf[58]-arv)))}else{(if sb[53]{(aqm+(if sb[53]{((C1*(if sb[53]{((nU*(-aqs))+(nY*aqs))}else{b}))+(BZ*(sf[58]-apU)))}else{b}))}else{(if ((sf[1])!=0.0){(aoR+(if B2{b}else{(if AK{(AL*((AU*sf[255])+(AH*MP)))}else{b})}))}else{b})})}))+(sf[221]*(if sb[57]{b}else{Xu}))));let avv=(sf[58]*(sf[221]*(if sb[57]{b}else{Xv})));let avw=(sf[58]*(sf[221]*(if sb[57]{b}else{Xw})));let avx=(sf[58]*((j7*(if sb[55]{((if sb[55]{((lU*((-(arw/iV))*arH))/sf[190])}else{aqn})+(sf[198]*(sf[246]-arw)))}else{(if sb[53]{(aqn+(if sb[53]{((C1*(if sb[53]{((nU*(-aqt))+(nY*aqt))}else{b}))+(BZ*(sf[246]-apV)))}else{b}))}else{(if ((sf[1])!=0.0){(aoS+(if B2{b}else{(if AK{(AL*((AU*sf[254])+(AH*MO)))}else{b})}))}else{b})})}))+(sf[221]*(if sb[57]{b}else{Xx}))));

        CommonStampValues {
            b, d, T, ca, fk, fF, fG, fH,
            gE, h1, h8, hf, hm, hU, iV, ji,
            jj, k5, k6, k8, k9, kb, kc, ke,
            kf, kk, km, kn, ko, ks, kB, kC,
            p2, pk, pp, ps, px, pV, q8, qM,
            rm, ro, sd, sA, sB, te, tw, tx,
            ud, uu, uv, v5, vo, vp, vI, vX,
            wJ, wM, Dg, Di, Dk, Dm, Dp, Dq,
            Dr, Ds, Dt, Du, Dz, DB, DC, EG,
            Fu, Fy, FK, FO, G0, G4, Gg, Gk,
            Ic, IN, IQ, IU, Tg, Th, Ti, TQ,
            TR, TS, TT, Un, Uo, Up, Uq, Vn,
            Vo, Vp, Vq, VR, VS, VT, VU, VY,
            Xs, Xt, Xu, Xv, Xw, Xx, YU, YV,
            YW, YX, YY, YZ, Z2, a0H, a0I, a0J,
            a0K, a1p, a1q, a1r, a1s, a1t, a1u, a1v,
            a2F, a2G, a2H, a2I, a3h, a3i, a3j, a3k,
            a3l, a3m, a3n, a57, a58, a59, a5a, a5J,
            a5K, a5L, a5M, a5N, a5O, a5P, a75, a76,
            a77, a78, a7I, a7J, a7K, a7L, a7M, a7N,
            a7P, a8y, a8T, a8U, a8V, a8W, a8X, a8Y,
            aaL, aaM, aaN, aaO, aaP, aaQ, aaZ, ab0,
            ab1, ab2, ab3, av8, av9, ava, avb, avc,
            avd, ave, avf, avg, avh, avi, avj, avk,
            avl, avm, avn, avo, avp, avq, avr, avs,
            avt, avu, avv, avw, avx,
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let CommonStampValues {
            b, d, T, ca, fk, fF, fG, fH,
            gE, h1, h8, hf, hm, hU, iV, ji,
            jj, k5, k6, k8, k9, kb, kc, ke,
            kf, kk, km, kn, ko, ks, kB, kC,
            p2, pk, pp, ps, px, pV, q8, qM,
            rm, ro, sd, sA, sB, te, tw, tx,
            ud, uu, uv, v5, vo, vp, vI, vX,
            wJ, wM, Dg, Di, Dk, Dm, Dp, Dq,
            Dr, Ds, Dt, Du, Dz, DB, DC, EG,
            Fu, Fy, FK, FO, G0, G4, Gg, Gk,
            Ic, IN, IQ, IU, Tg, Th, Ti, TQ,
            TR, TS, TT, Un, Uo, Up, Uq, Vn,
            Vo, Vp, Vq, VR, VS, VT, VU, VY,
            Xs, Xt, Xu, Xv, Xw, Xx, YU, YV,
            YW, YX, YY, YZ, Z2, a0H, a0I, a0J,
            a0K, a1p, a1q, a1r, a1s, a1t, a1u, a1v,
            a2F, a2G, a2H, a2I, a3h, a3i, a3j, a3k,
            a3l, a3m, a3n, a57, a58, a59, a5a, a5J,
            a5K, a5L, a5M, a5N, a5O, a5P, a75, a76,
            a77, a78, a7I, a7J, a7K, a7L, a7M, a7N,
            a7P, a8y, a8T, a8U, a8V, a8W, a8X, a8Y,
            aaL, aaM, aaN, aaO, aaP, aaQ, aaZ, ab0,
            ab1, ab2, ab3, av8, av9, ava, avb, avc,
            avd, ave, avf, avg, avh, avi, avj, avk,
            avl, avm, avn, avo, avp, avq, avr, avs,
            avt, avu, avv, avw, avx,
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
        let ae=0.01;let fS=f64::powf(fG,sf[140]);let fU=(if sb[46]{(sf[138]*fS)}else{(if ((sf[33])!=0.0){(sf[138]*f64::powf(fG,sf[139]))}else{b})});let g2=(if sb[47]{(fS*sf[141])}else{(if ((sf[13])!=0.0){(sf[141]*f64::powf(fG,sf[142]))}else{b})});let ga=f64::powf(fG,sf[145]);let gc=(if sb[48]{(sf[143]*ga)}else{(if ((sf[19])!=0.0){(sf[143]*f64::powf(fG,sf[144]))}else{b})});let gk=(if sb[49]{(ga*sf[146])}else{(if ((sf[24])!=0.0){(sf[146]*f64::powf(fG,sf[147]))}else{b})});let go=(sf[148]*f64::powf(fG,sf[149]));let gw=(if sb[50]{(fS*sf[150])}else{(if ((sf[34])!=0.0){(sf[150]*f64::powf(fG,sf[151]))}else{b})});let gB=(sf[152]*(d+(fH*sf[153])));let gZ=(sf[115]*f64::powf(fG,sf[118]));let h0=(sf[120]*gE);let h3=((h0/h1)).exp();let h4=(gZ*h3);let h6=(sf[121]*f64::powf(fG,sf[124]));let h7=(sf[126]*gE);let ha=((h7/h8)).exp();let hb=(h6*ha);let hc=f64::powf(fG,sf[129]);let hd=(sf[127]*hc);let he=(sf[131]*gE);let hh=((he/hf)).exp();let hi=(hd*hh);let hj=f64::powf(fG,sf[134]);let hk=(sf[132]*hj);let hl=(sf[136]*gE);let ho=((hl/hm)).exp();let hp=(hk*ho);let hq=(sf[38]*hc);let hr=(hh*hq);let hs=(sf[39]*hj);let ht=(ho*hs);let hD=(sf[155]*(d+(fH*sf[156])));let hI=(sf[157]*(d+(fH*sf[158])));let jh=(sf[174]*f64::powf(fG,sf[175]));let jl=((ji/jj)).exp();let jw=0.001;let jx=(fU>jw);let jz=1000.0;let jA=(if jx{(d/fU)}else{jz});let jB=(g2>jw);let jD=(if jB{(d/g2)}else{jz});let jE=(gc>jw);let jG=(if jE{(d/gc)}else{jz});let jH=(gk>jw);let jJ=(if jH{(d/gk)}else{jz});let jK=(go>jw);let jM=(if jK{(d/go)}else{jz});let jN=(gw>jw);let jP=(if jN{(d/gw)}else{jz});let jQ=(gB>jw);let jS=(if jQ{(d/gB)}else{jz});let k2=(jh>b);let k4=(if k2{(d/jh)}else{b});let kj=(sf[58]*(k9-kf));let kr=(sf[58]*(kc-k6));let ku=(ks-kf);let kw=(sf[58]*(kf-kc));let kx=(kn-k9);let ky=(k9-k5);let kz=(ko-k6);let kA=(kk-kf);let q9=(pk/q8);let qa=(p2/q8);let qR=(if ((sf[32])!=0.0){(d+(ca*(if ((sf[32])!=0.0){(sf[64]*qM)}else{b})))}else{pV});let qT=(if (qR>ps){d}else{b});let qU=(((sf[32])!=0.0)&&((qT)!=0.0));let qV=(qR).sqrt();let r0=(((sf[32])!=0.0)&&(!((qT)!=0.0)));let r5=(if sb[57]{d}else{(if r0{0.50005}else{(if qU{(T*(d+qV))}else{b})})});let rq=(if (k8<sf[410]){d}else{b});let rr=(((sf[18])!=0.0)&&((rq)!=0.0));let rt=((k8*ro)).exp();let rv=(!((rq)!=0.0));let rw=(((sf[18])!=0.0)&&rv);let ry=((sf[410]*ro)).exp();let rz=(k8-sf[410]);let rB=(d+(ro*rz));let rD=(if rw{(ry*rB)}else{(if rr{rt}else{b})});let rH=(d+(sf[8]*(px-d)));let rI=(h4*rH);let rJ=(rm-d);let rL=(rD-d);let rM=(hb*rL);let rT=(if sb[60]{(rM+(h4*rJ))}else{(if sb[58]{((rI*rJ)+rM)}else{b})});let sD=(if (kb<sf[410]){d}else{b});let sE=(sb[63]&&((sD)!=0.0));let sG=((kb*sB)).exp();let sI=(!((sD)!=0.0));let sJ=(sb[63]&&sI);let sL=((sf[410]*sB)).exp();let sM=(kb-sf[410]);let sO=(d+(sB*sM));let sQ=(if sJ{(sL*sO)}else{(if sE{sG}else{rD})});let sR=(sA-d);let sT=(sQ-d);let sW=(if sb[63]{((h4*sR)+(hb*sT))}else{b});let ty=(((rq)!=0.0)&&sb[66]);let tA=((k8*tx)).exp();let tC=(rv&&sb[66]);let tE=((sf[410]*tx)).exp();let tG=(d+(rz*tx));let tI=(if tC{(tE*tG)}else{(if ty{tA}else{sQ})});let tK=(tw-d);let tM=(tI-d);let tN=(hb*tM);let tV=(if sb[68]{(sf[17]*(tN+(h4*tK)))}else{(if sb[67]{(sf[17]*((rI*tK)+tN))}else{(if sb[63]{b}else{(if sb[61]{(rT-(sf[36]*(sd-jl)))}else{rT})})})});let ui=(if sb[69]{(tV-(sf[204]*(ud-jl)))}else{tV});let uw=(((sD)!=0.0)&&sb[66]);let uy=((kb*uv)).exp();let uA=(sI&&sb[66]);let uC=((sf[410]*uv)).exp();let uE=(d+(sM*uv));let uG=(if uA{(uC*uE)}else{(if uw{uy}else{tI})});let uI=(uu-d);let uK=(uG-d);let uO=(if sb[66]{(sf[205]*((h4*uI)+(hb*uK)))}else{(if sb[64]{(sW-(sf[36]*(te-jl)))}else{sW})});let va=(if sb[69]{(uO-(sf[206]*(v5-jl)))}else{uO});let vr=(if (ke<sf[438]){d}else{b});let vt=((ke*vp)).exp();let vv=(!((vr)!=0.0));let vx=((sf[438]*vp)).exp();let vy=(ke-sf[438]);let vA=(d+(vp*vy));let vC=(if vv{(vx*vA)}else{(if ((vr)!=0.0){vt}else{uG})});let vD=(vo-d);let vF=(vC-d);let vH=((hi*vD)+(hp*vF));let vY=(if ((sf[40])!=0.0){vp}else{vI});let w0=(if (km<sf[456]){d}else{b});let w1=(((sf[40])!=0.0)&&((w0)!=0.0));let w3=((km*vY)).exp();let w6=(((sf[40])!=0.0)&&(!((w0)!=0.0)));let w8=((sf[456]*vY)).exp();let w9=(km-sf[456]);
        let wb=(d+(vY*w9));let we=(vX-d);let wg=((if w6{(w8*wb)}else{(if w1{w3}else{vC})})-d);let wl=(if sb[70]{b}else{(if ((sf[40])!=0.0){((hr*we)+(ht*wg))}else{b})});let wN=(jA*ku);let wO=(d+wJ);let wP=(d+wM);let wQ=(wO/wP);let wT=((wJ-wM)-(wQ).ln());let wV=(kw+(fF*wT));let wW=(jD*wV);let wX=(k4*wW);let wZ=(sf[67]*(T*k4));let x2=((ae+(kw*kw))).sqrt();let x4=(d+(wZ*x2));let x5=(jD*x4);let x6=(wX/x5);let x9=((d+(x6*x6))).sqrt();let xa=(wW/x9);let xb=(jG*kx);let xc=(ky*q8);let xd=(jJ*xc);let xe=(jM*kz);let xf=(kA*r5);let xg=(jP*xf);let xh=0.02;let xj=(xh*(d+hD));let xo=(if ((sf[3])!=0.0){f64::powf(xj,sf[209])}else{b});let xq=((iV-ke)-xo);let xt=((ae+(xq*xq))).sqrt();let xx=(if ((sf[3])!=0.0){(xo+(T*(xq+xt)))}else{b});let xy=(-hD);let xA=f64::powf(xx,sf[210]);let xC=(if ((sf[3])!=0.0){(xy*xA)}else{b});let xE=(if (xC<sf[60]){d}else{b});let xF=(((sf[3])!=0.0)&&((xE)!=0.0));let xG=(xC).exp();let xJ=(((sf[3])!=0.0)&&(!((xE)!=0.0)));let xK=(if xJ{sf[207]}else{b});let xO=(if xJ{(xK*(d+(xC-sf[60])))}else{(if xF{xG}else{b})});let xP=(sf[2]*xx);let xR=(if ((sf[3])!=0.0){(xO*xP)}else{b});let xS=(kC-q9);let xT=(xS-vH);let xZ=(xh*(d+hI));let y4=(if ((sf[21])!=0.0){f64::powf(xZ,sf[213])}else{b});let y6=((b-kj)-y4);let y9=((ae+(y6*y6))).sqrt();let yd=(if ((sf[21])!=0.0){(y4+(T*(y6+y9)))}else{b});let ye=(-hI);let yg=f64::powf(yd,sf[214]);let yi=(if ((sf[21])!=0.0){(ye*yg)}else{b});let yk=(if (yi<sf[60]){d}else{b});let yl=(((sf[21])!=0.0)&&((yk)!=0.0));let ym=(yi).exp();let yp=(((sf[21])!=0.0)&&(!((yk)!=0.0)));let yq=(if yp{sf[207]}else{b});let yu=(if yp{(yq*(d+(yi-sf[60])))}else{(if yl{ym}else{b})});let yv=(sf[20]*yd);let yx=(if ((sf[21])!=0.0){(yu*yv)}else{xR});let yy=(-wN);let yG=0.1;let yI=(if sb[73]{((d-(ke/sf[22]))-yG)}else{b});let yL=((pp+(yI*yI))).sqrt();let yU=(if sb[75]{sf[11]}else{(if sb[73]{(sf[11]*(if sb[73]{(yG+(T*(yI+yL)))}else{yI}))}else{b})});let yW=((qa/yU)-d);let z4=((vH-(if sb[71]{b}else{(if ((sf[3])!=0.0){(xR*xT)}else{b})}))-(if sb[76]{b}else{(if ((sf[12])!=0.0){(sf[10]*f64::powf(yW,sf[215]))}else{b})}));let zN=(sf[58]*xa);let DR=(DC*(sf[140]*f64::powf(fG,sf[229])));let Eb=(DC*(sf[145]*f64::powf(fG,sf[232])));let FD=((h3*(sf[115]*(DC*(sf[118]*f64::powf(fG,sf[239])))))+(gZ*(h3*(((h1*(sf[120]*EG))-(h0*Fu))/Fy))));let FT=((ha*(sf[121]*(DC*(sf[124]*f64::powf(fG,sf[240])))))+(h6*(ha*(((h8*(sf[126]*EG))-(h7*FK))/FO))));let FX=(DC*(sf[129]*f64::powf(fG,sf[241])));let G6=(hh*(((hf*(sf[131]*EG))-(he*G0))/G4));let Gd=(DC*(sf[134]*f64::powf(fG,sf[242])));let Gm=(ho*(((hm*(sf[136]*EG))-(hl*Gg))/Gk));let GC=(sf[155]*(sf[156]*Dz));let GE=(sf[157]*(sf[158]*Dz));let IW=(jl*(((jj*IN)-(ji*IQ))/IU));let J8=(if jB{((-(if sb[47]{(sf[141]*DR)}else{(if ((sf[13])!=0.0){(sf[141]*(DC*(sf[142]*f64::powf(fG,sf[230]))))}else{b})}))/(g2*g2))}else{b});let JI=(if k2{((-(sf[174]*(DC*(sf[175]*f64::powf(fG,sf[245])))))/(jh*jh))}else{b});let VZ=(((q8*TQ)-(pk*VR))/VY);let W3=(((q8*TR)-(pk*VS))/VY);let W7=(((q8*TS)-(pk*VT))/VY);let Wb=(((q8*TT)-(pk*VU))/VY);let Wf=(((q8*Tg)-(p2*VR))/VY);let Wi=((-(p2*VS))/VY);let Wm=(((q8*Th)-(p2*VT))/VY);let Wq=(((q8*Ti)-(p2*VU))/VY);let XW=(hU*qV);let Z4=(sf[58]*ro);let Z5=(ro*sf[246]);let Zk=(if rw{((rB*(ry*(sf[410]*Z2)))+(ry*(rz*Z2)))}else{(if rr{(rt*(k8*Z2))}else{b})});let Zl=(if rw{(ry*Z4)}else{(if rr{(rt*Z4)}else{b})});let Zm=(if rw{(ry*Z5)}else{(if rr{(rt*Z5)}else{b})});let Zt=((rH*FD)+(h4*(sf[8]*Un)));let Zu=(h4*(sf[8]*Uo));let Zv=(h4*(sf[8]*Up));let Zw=(h4*(sf[8]*Uq));let ZN=((rL*FT)+(hb*Zk));let ZO=(hb*Zl);let ZP=(hb*Zm);let a0a=(if sb[60]{(ZN+((rJ*FD)+(h4*YU)))}else{(if sb[58]{(((rJ*Zt)+(rI*YU))+ZN)}else{b})});let a0b=(if sb[60]{(h4*YV)}else{(if sb[58]{((rJ*Zu)+(rI*YV))}else{b})});let a0d=(if sb[60]{(ZO+(h4*YX))}else{(if sb[58]{(((rJ*Zv)+(rI*YX))+ZO)}else{b})});let a0e=(if sb[60]{(ZP+(h4*YY))}else{(if sb[58]{(((rJ*Zw)+(rI*YY))+ZP)}else{b})});let a1x=(sf[58]*sB);let a1y=(sB*sf[246]);let a1O=(if sJ{((sO*(sL*(sf[410]*a1v)))+(sL*(sM*a1v)))}else{(if sE{(sG*(kb*a1v))}else{Zk})});let a1P=(if sJ{(sL*a1x)}else{(if sE{(sG*a1x)}else{b})});let a1Q=(if sJ{b}else{(if sE{b}else{Zl})});
        let a1R=(if sJ{(sL*a1y)}else{(if sE{(sG*a1y)}else{Zm})});let a2a=(if sb[63]{(((sR*FD)+(h4*a1p))+((sT*FT)+(hb*a1O)))}else{b});let a2b=(if sb[63]{(h4*a1q)}else{b});let a2d=(if sb[63]{((h4*a1s)+(hb*a1Q))}else{b});let a2e=(if sb[63]{((h4*a1t)+(hb*a1R))}else{b});let a3p=(sf[58]*tx);let a3q=(tx*sf[246]);let a3G=(if tC{((tG*(tE*(sf[410]*a3n)))+(tE*(rz*a3n)))}else{(if ty{(tA*(k8*a3n))}else{a1O})});let a3H=(if tC{b}else{(if ty{b}else{a1P})});let a3I=(if tC{(tE*a3p)}else{(if ty{(tA*a3p)}else{a1Q})});let a3J=(if tC{(tE*a3q)}else{(if ty{(tA*a3q)}else{a1R})});let a40=((tM*FT)+(hb*a3G));let a41=(hb*a3H);let a42=(hb*a3I);let a43=(hb*a3J);let a4C=(if sb[68]{(sf[17]*(a40+((tK*FD)+(h4*a3h))))}else{(if sb[67]{(sf[17]*(((tK*Zt)+(rI*a3h))+a40))}else{(if sb[63]{b}else{(if sb[61]{(a0a-(sf[36]*(a0H-IW)))}else{a0a})})})});let a4D=(if sb[68]{(sf[17]*(h4*a3i))}else{(if sb[67]{(sf[17]*((tK*Zu)+(rI*a3i)))}else{(if sb[63]{b}else{(if sb[61]{(a0b-(sf[36]*a0I))}else{a0b})})})});let a4E=(if sb[68]{(sf[17]*(a41+(h4*a3j)))}else{(if sb[67]{(sf[17]*((rI*a3j)+a41))}else{(if sb[63]{b}else{(if sb[60]{(h4*YW)}else{(if sb[58]{(rI*YW)}else{b})})})})});let a4F=(if sb[68]{(sf[17]*(a42+(h4*a3k)))}else{(if sb[67]{(sf[17]*(((tK*Zv)+(rI*a3k))+a42))}else{(if sb[63]{b}else{(if sb[61]{(a0d-(sf[36]*a0J))}else{a0d})})})});let a4G=(if sb[68]{(sf[17]*(a43+(h4*a3l)))}else{(if sb[67]{(sf[17]*(((tK*Zw)+(rI*a3l))+a43))}else{(if sb[63]{b}else{(if sb[61]{(a0e-(sf[36]*a0K))}else{a0e})})})});let a4H=(if sb[68]{(sf[17]*(h4*a3m))}else{(if sb[67]{(sf[17]*(rI*a3m))}else{(if sb[63]{b}else{(if sb[60]{(h4*YZ)}else{(if sb[58]{(rI*YZ)}else{b})})})})});let a5k=(if sb[69]{(a4C-(sf[204]*(a57-IW)))}else{a4C});let a5l=(if sb[69]{(a4D-(sf[204]*a58))}else{a4D});let a5m=(if sb[69]{(a4F-(sf[204]*a59))}else{a4F});let a5n=(if sb[69]{(a4G-(sf[204]*a5a))}else{a4G});let a5R=(sf[58]*uv);let a5S=(uv*sf[246]);let a68=(if uA{((uE*(uC*(sf[410]*a5P)))+(uC*(sM*a5P)))}else{(if uw{(uy*(kb*a5P))}else{a3G})});let a69=(if uA{(uC*a5R)}else{(if uw{(uy*a5R)}else{a3H})});let a6a=(if uA{b}else{(if uw{b}else{a3I})});let a6b=(if uA{(uC*a5S)}else{(if uw{(uy*a5S)}else{a3J})});let a6A=(if sb[66]{(sf[205]*(((uI*FD)+(h4*a5J))+((uK*FT)+(hb*a68))))}else{(if sb[64]{(a2a-(sf[36]*(a2F-IW)))}else{a2a})});let a6B=(if sb[66]{(sf[205]*(h4*a5K))}else{(if sb[64]{(a2b-(sf[36]*a2G))}else{a2b})});let a6C=(if sb[66]{(sf[205]*((h4*a5L)+(hb*a69)))}else{(if sb[63]{((h4*a1r)+(hb*a1P))}else{b})});let a6D=(if sb[66]{(sf[205]*((h4*a5M)+(hb*a6a)))}else{(if sb[64]{(a2d-(sf[36]*a2H))}else{a2d})});let a6E=(if sb[66]{(sf[205]*((h4*a5N)+(hb*a6b)))}else{(if sb[64]{(a2e-(sf[36]*a2I))}else{a2e})});let a6F=(if sb[66]{(sf[205]*(h4*a5O))}else{(if sb[63]{(h4*a1u)}else{b})});let a7i=(if sb[69]{(a6A-(sf[206]*(a75-IW)))}else{a6A});let a7j=(if sb[69]{(a6B-(sf[206]*a76))}else{a6B});let a7k=(if sb[69]{(a6D-(sf[206]*a77))}else{a6D});let a7l=(if sb[69]{(a6E-(sf[206]*a78))}else{a6E});let a7R=(vp*sf[246]);let a7S=(sf[58]*vp);let a89=(if vv{((vA*(vx*(sf[438]*a7P)))+(vx*(vy*a7P)))}else{(if ((vr)!=0.0){(vt*(ke*a7P))}else{a68})});let a8a=(if vv{(vx*a7R)}else{(if ((vr)!=0.0){(vt*a7R)}else{b})});let a8b=(if vv{b}else{(if ((vr)!=0.0){b}else{a69})});let a8c=(if vv{(vx*a7S)}else{(if ((vr)!=0.0){(vt*a7S)}else{a6a})});let a8d=(if vv{b}else{(if ((vr)!=0.0){b}else{a6b})});let a8l=(hi*a7N);let a8t=(((vD*((hh*(sf[127]*FX))+(hd*G6)))+(hi*a7I))+((vF*((ho*(sf[132]*Gd))+(hk*Gm)))+(hp*a89)));let a8u=((hi*a7J)+(hp*a8a));let a8v=((hi*a7K)+(hp*a8b));let a8w=((hi*a7L)+(hp*a8c));let a8x=((hi*a7M)+(hp*a8d));let a8Z=(if ((sf[40])!=0.0){a7P}else{a8y});let a91=(sf[58]*vY);let a92=(vY*sf[246]);let a9S=(if sb[70]{b}else{(if ((sf[40])!=0.0){(((we*((hq*G6)+(hh*(sf[38]*FX))))+(hr*a8T))+((wg*((hs*Gm)+(ho*(sf[39]*Gd))))+(ht*(if w6{((wb*(w8*(sf[456]*a8Z)))+(w8*(w9*a8Z)))}else{(if w1{(w3*(km*a8Z))}else{a89})}))))}else{b})});let a9T=(if sb[70]{b}else{(if ((sf[40])!=0.0){((hr*a8U)+(ht*(if w6{b}else{(if w1{b}else{a8a})})))}else{b})});let a9U=(if sb[70]{b}else{(if ((sf[40])!=0.0){((hr*a8V)+(ht*(if w6{(w8*a91)}else{(if w1{(w3*a91)}else{a8b})})))}else{b})});
        let a9V=(if sb[70]{b}else{(if ((sf[40])!=0.0){((hr*a8W)+(ht*(if w6{b}else{(if w1{b}else{a8c})})))}else{b})});let a9W=(if sb[70]{b}else{(if ((sf[40])!=0.0){((hr*a8X)+(ht*(if w6{b}else{(if w1{b}else{a8d})})))}else{b})});let a9X=(if sb[70]{b}else{(if ((sf[40])!=0.0){((hr*a8Y)+(ht*(if w6{(w8*a92)}else{(if w1{(w3*a92)}else{b})})))}else{b})});let ab4=(ku*(if jx{((-(if sb[46]{(sf[138]*DR)}else{(if ((sf[33])!=0.0){(sf[138]*(DC*(sf[139]*f64::powf(fG,sf[228]))))}else{b})}))/(fU*fU))}else{b}));let ab5=(-jA);let ab9=(wP*wP);let abY=((wV*J8)+(jD*((wT*DB)+(fF*((aaL-aaZ)-((((wP*aaL)-(wO*aaZ))/ab9)/wQ))))));let abZ=(jD*(sf[58]+(fF*((-ab0)-(((-(wO*ab0))/ab9)/wQ)))));let ac0=(jD*(sf[246]+(fF*((aaM-ab1)-((((wP*aaM)-(wO*ab1))/ab9)/wQ)))));let ac1=(jD*(fF*(aaN-((aaN/wP)/wQ))));let ac2=(jD*(fF*((aaO-ab2)-((((wP*aaO)-(wO*ab2))/ab9)/wQ))));let ac3=(jD*(fF*((aaP-ab3)-((((wP*aaP)-(wO*ab3))/ab9)/wQ))));let ac4=(jD*(fF*(aaQ-((aaQ/wP)/wQ))));let acg=(sf[58]*kw);let aci=(kw*sf[246]);let ack=(hU*x2);let acy=(x5*x5);let acM=(x6*(((x5*((wW*JI)+(k4*abY)))-(wX*((x4*J8)+(jD*(x2*(sf[67]*(T*JI)))))))/acy));let acO=(x6*(((x5*(k4*abZ))-(wX*(jD*(wZ*((acg+acg)/ack)))))/acy));let acQ=(x6*(((x5*(k4*ac0))-(wX*(jD*(wZ*((aci+aci)/ack)))))/acy));let acS=(x6*((k4*ac1)/x5));let acU=(x6*((k4*ac2)/x5));let acW=(x6*((k4*ac3)/x5));let acY=(x6*((k4*ac4)/x5));let ad0=(hU*x9);let adb=(x9*x9);let adc=(((x9*abY)-(wW*((acM+acM)/ad0)))/adb);let adg=(((x9*abZ)-(wW*((acO+acO)/ad0)))/adb);let adk=(((x9*ac0)-(wW*((acQ+acQ)/ad0)))/adb);let ado=(((x9*ac1)-(wW*((acS+acS)/ad0)))/adb);let ads=(((x9*ac2)-(wW*((acU+acU)/ad0)))/adb);let adw=(((x9*ac3)-(wW*((acW+acW)/ad0)))/adb);let adA=(((x9*ac4)-(wW*((acY+acY)/ad0)))/adb);let adB=(kx*(if jE{((-(if sb[48]{(sf[143]*Eb)}else{(if ((sf[19])!=0.0){(sf[143]*(DC*(sf[144]*f64::powf(fG,sf[231]))))}else{b})}))/(gc*gc))}else{b}));let adC=(-jG);let adL=((xc*(if jH{((-(if sb[49]{(sf[146]*Eb)}else{(if ((sf[24])!=0.0){(sf[146]*(DC*(sf[147]*f64::powf(fG,sf[233]))))}else{b})}))/(gk*gk))}else{b}))+(jJ*(ky*VR)));let adM=(jJ*(ky*VS));let adN=(jJ*q8);let adO=(jJ*((-q8)+(ky*VT)));let adP=(jJ*(ky*VU));let adQ=(kz*(if jK{((-(sf[148]*(DC*(sf[149]*f64::powf(fG,sf[234])))))/(go*go))}else{b}));let adR=(-jM);let ae2=((xf*(if jN{((-(if sb[50]{(sf[150]*DR)}else{(if ((sf[34])!=0.0){(sf[150]*(DC*(sf[151]*f64::powf(fG,sf[235]))))}else{b})}))/(gw*gw))}else{b}))+(jP*(kA*(if sb[57]{b}else{(if r0{b}else{(if qU{(T*((if ((sf[32])!=0.0){(ca*(if ((sf[32])!=0.0){(sf[64]*Xs)}else{b}))}else{Vn})/XW))}else{b})})}))));let ae3=(jP*(-r5));let ae4=(jP*(kA*(if sb[57]{b}else{(if r0{b}else{(if qU{(T*((if ((sf[32])!=0.0){(ca*(if ((sf[32])!=0.0){(sf[64]*Xt)}else{b}))}else{Vo})/XW))}else{b})})})));let ae5=(jP*(kA*(if sb[57]{b}else{(if r0{b}else{(if qU{(T*((if ((sf[32])!=0.0){(ca*(if ((sf[32])!=0.0){(sf[64]*Xu)}else{b}))}else{b})/XW))}else{b})})})));let ae6=(jP*(kA*(if sb[57]{b}else{(if r0{b}else{(if qU{(T*((if ((sf[32])!=0.0){(ca*(if ((sf[32])!=0.0){(sf[64]*Xv)}else{b}))}else{Vp})/XW))}else{b})})})));let ae7=(jP*(kA*(if sb[57]{b}else{(if r0{b}else{(if qU{(T*((if ((sf[32])!=0.0){(ca*(if ((sf[32])!=0.0){(sf[64]*Xw)}else{b}))}else{Vq})/XW))}else{b})})})));let ae8=(jP*(r5+(kA*(if sb[57]{b}else{(if r0{b}else{(if qU{(T*((if ((sf[32])!=0.0){(ca*(if ((sf[32])!=0.0){(sf[64]*Xx)}else{b}))}else{b})/XW))}else{b})})}))));let aee=(if ((sf[3])!=0.0){((xh*GC)*(sf[209]*f64::powf(xj,sf[276])))}else{b});let aef=(Ic-aee);let aeg=(xq*aef);let aei=(sf[58]*xq);let aek=(xq*sf[246]);let aem=(hU*xt);let aex=(if ((sf[3])!=0.0){(aee+(T*(aef+((aeg+aeg)/aem))))}else{b});let aey=(if ((sf[3])!=0.0){(T*(sf[58]+((aei+aei)/aem)))}else{b});let aez=(if ((sf[3])!=0.0){(T*(sf[246]+((aek+aek)/aem)))}else{b});let aeD=(sf[210]*f64::powf(xx,sf[277]));let aeM=(if ((sf[3])!=0.0){((xA*(-GC))+(xy*(aex*aeD)))}else{b});let aeN=(if ((sf[3])!=0.0){(xy*(aey*aeD))}else{b});let aeO=(if ((sf[3])!=0.0){(xy*(aez*aeD))}else{b});let afd=(if ((sf[3])!=0.0){((xP*(if xJ{(xK*aeM)}else{(if xF{(xG*aeM)}else{b})}))+(xO*(sf[2]*aex)))}else{b});let afe=(if ((sf[3])!=0.0){((xP*(if xJ{(xK*aeN)}else{(if xF{(xG*aeN)}else{b})}))+(xO*(sf[2]*aey)))}else{b});
        let aff=(if ((sf[3])!=0.0){((xP*(if xJ{(xK*aeO)}else{(if xF{(xG*aeO)}else{b})}))+(xO*(sf[2]*aez)))}else{b});let afg=(-VZ);let afh=(-W3);let afi=(-W7);let afj=(-Wb);let afV=(if ((sf[21])!=0.0){((xh*GE)*(sf[213]*f64::powf(xZ,sf[278])))}else{b});let afW=(-afV);let afX=(y6*afW);let afZ=(sf[58]*y6);let ag1=(y6*sf[246]);let ag3=(hU*y9);let age=(if ((sf[21])!=0.0){(afV+(T*(afW+((afX+afX)/ag3))))}else{b});let agf=(if ((sf[21])!=0.0){(T*(sf[58]+((afZ+afZ)/ag3)))}else{b});let agg=(if ((sf[21])!=0.0){(T*(sf[246]+((ag1+ag1)/ag3)))}else{b});let agk=(sf[214]*f64::powf(yd,sf[279]));let agt=(if ((sf[21])!=0.0){((yg*(-GE))+(ye*(age*agk)))}else{b});let agu=(if ((sf[21])!=0.0){(ye*(agf*agk))}else{b});let agv=(if ((sf[21])!=0.0){(ye*(agg*agk))}else{b});let ahs=(yI*sf[284]);let ahu=(yI*sf[285]);let ahw=(hU*yL);let ahP=(yU*yU);let ahY=(sf[215]*f64::powf(yW,sf[286]));let aih=(a8v-(if sb[71]{b}else{(if ((sf[3])!=0.0){(xR*(-a8v))}else{b})}));let aik=(a8l-(if sb[71]{b}else{(if ((sf[3])!=0.0){(xR*(-a8l))}else{b})}));let ail=(-(if sb[71]{b}else{(if ((sf[3])!=0.0){xR}else{b})}));let aim=((a8t-(if sb[71]{b}else{(if ((sf[3])!=0.0){((xT*afd)+(xR*(afg-a8t)))}else{b})}))-(if sb[76]{b}else{(if ((sf[12])!=0.0){(sf[10]*((Wf/yU)*ahY))}else{b})}));let ain=((a8u-(if sb[71]{b}else{(if ((sf[3])!=0.0){((xT*afe)+(xR*(afh-a8u)))}else{b})}))-(if sb[76]{b}else{(if ((sf[12])!=0.0){(sf[10]*((((yU*Wi)-(qa*(if sb[75]{b}else{(if sb[73]{(sf[11]*(if sb[73]{(T*(sf[284]+((ahs+ahs)/ahw)))}else{sf[284]}))}else{b})})))/ahP)*ahY))}else{b})}));let aio=((a8w-(if sb[71]{b}else{(if ((sf[3])!=0.0){((xT*aff)+(xR*(afi-a8w)))}else{b})}))-(if sb[76]{b}else{(if ((sf[12])!=0.0){(sf[10]*((((yU*Wm)-(qa*(if sb[75]{b}else{(if sb[73]{(sf[11]*(if sb[73]{(T*(sf[285]+((ahu+ahu)/ahw)))}else{sf[285]}))}else{b})})))/ahP)*ahY))}else{b})}));let aip=((a8x-(if sb[71]{b}else{(if ((sf[3])!=0.0){(xR*(afj-a8x))}else{b})}))-(if sb[76]{b}else{(if ((sf[12])!=0.0){(sf[10]*((Wq/yU)*ahY))}else{b})}));

        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(8),
            multiplicity * ((sf[58]*(ui+(sf[44]*k8)))),
            [3, 5, 6, 7, 8, 9],
            [(sf[58]*a5k), (sf[58]*a5l), (sf[58]*a4E), (sf[58]*(a5m+sf[287])), (sf[58]*(a5n+sf[288])), (sf[58]*a4H)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(8),
            multiplicity * ((sf[58]*(va+(sf[44]*kb)))),
            [3, 5, 6, 7, 8, 9],
            [(sf[58]*a7i), (sf[58]*a7j), (sf[58]*(a6C+sf[287])), (sf[58]*a7k), (sf[58]*(a7l+sf[288])), (sf[58]*a6F)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(5),
            Some(8),
            multiplicity * ((sf[58]*kC)),
            11,
            multiplicity * (sf[58]),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * ((sf[58]*q9)),
            [3, 5, 7, 8],
            [(sf[58]*VZ), (sf[58]*W3), (sf[58]*W7), (sf[58]*Wb)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(5),
            multiplicity * ((sf[58]*(z4+(sf[44]*ke)))),
            [3, 5, 6, 7, 8, 9, 11],
            [(sf[58]*aim), (sf[58]*(ain+sf[288])), (sf[58]*aih), (sf[58]*(aio+sf[287])), (sf[58]*aip), (sf[58]*aik), (sf[58]*ail)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(4),
            multiplicity * ((sf[58]*((if sb[72]{b}else{(if ((sf[21])!=0.0){(yx*yy)}else{b})})+(sf[44]*kj)))),
            [0, 3, 4, 5, 6, 7],
            [(sf[58]*(if sb[72]{b}else{(if ((sf[21])!=0.0){(yx*ab5)}else{b})})), (sf[58]*(if sb[72]{b}else{(if ((sf[21])!=0.0){((yy*(if ((sf[21])!=0.0){((yv*(if yp{(yq*agt)}else{(if yl{(ym*agt)}else{b})}))+(yu*(sf[20]*age)))}else{afd}))+(yx*(-ab4)))}else{b})})), (sf[58]*((if sb[72]{b}else{(if ((sf[21])!=0.0){((yy*(if ((sf[21])!=0.0){((yv*(if yp{(yq*agu)}else{(if yl{(ym*agu)}else{b})}))+(yu*(sf[20]*agf)))}else{b}))+(jA*yx))}else{b})})+sf[288])), (sf[58]*(if sb[72]{b}else{(if ((sf[21])!=0.0){(yy*(if ((sf[21])!=0.0){b}else{afe}))}else{b})})), (sf[58]*((if sb[72]{b}else{(if ((sf[21])!=0.0){(yy*(if ((sf[21])!=0.0){((yv*(if yp{(yq*agv)}else{(if yl{(ym*agv)}else{b})}))+(yu*(sf[20]*agg)))}else{b}))}else{b})})+sf[287])), (sf[58]*(if sb[72]{b}else{(if ((sf[21])!=0.0){(yy*(if ((sf[21])!=0.0){b}else{aff}))}else{b})}))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(9),
            multiplicity * ((sf[58]*(wl+(sf[44]*km)))),
            [3, 5, 6, 7, 8, 9],
            [(sf[58]*a9S), (sf[58]*a9T), (sf[58]*(a9U+sf[287])), (sf[58]*a9V), (sf[58]*a9W), (sf[58]*(a9X+sf[288]))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(4),
            multiplicity * (wN),
            0,
            multiplicity * (jA),
            3,
            multiplicity * (ab4),
            4,
            multiplicity * (ab5),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(4),
            Some(5),
            multiplicity * (zN),
            [3, 4, 5, 6, 7, 8, 9],
            [(sf[58]*adc), (sf[58]*adg), (sf[58]*adk), (sf[58]*ado), (sf[58]*ads), (sf[58]*adw), (sf[58]*adA)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(6),
            multiplicity * (xb),
            1,
            multiplicity * (jG),
            3,
            multiplicity * (adB),
            6,
            multiplicity * (adC),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (xd),
            [3, 5, 6, 7, 8],
            [adL, adM, adN, adO, adP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(8),
            multiplicity * (xe),
            2,
            multiplicity * (jM),
            3,
            multiplicity * (adQ),
            8,
            multiplicity * (adR),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(9),
            Some(4),
            multiplicity * (xg),
            [3, 4, 5, 6, 7, 8, 9],
            [ae2, ae3, ae4, ae5, ae6, ae7, ae8],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            None,
            multiplicity * ((kC-qa)),
            [3, 5, 7, 8, 11],
            [(-Wf), (-Wi), (-Wm), (-Wq), d],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(11),
            None,
            multiplicity * ((kC-kB)),
            10,
            multiplicity * (-1.0),
            11,
            multiplicity * (d),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((fk*jS)),
            3,
            multiplicity * ((jS+(fk*(if jQ{((-(sf[152]*(sf[153]*Dz)))/(gB*gB))}else{b})))),
        );
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            None,
            multiplicity * (((((((((((((k8*ui)+(ke*z4))+(kr*xS))+(kb*va))+(km*wl))+(ku*wN))+(kw*xa))+(kx*xb))+(ky*xd))+(kz*xe))+(kA*xg))*sf[217])),
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11],
            &[(sf[217]*(wN+wN)), (sf[217]*(xb+xb)), (sf[217]*(xe+xe)), (sf[217]*(((((((((((k8*a5k)+(ke*aim))+(kr*afg))+(kb*a7i))+(km*a9S))+(ku*ab4))+(kw*adc))+(kx*adB))+(ky*adL))+(kz*adQ))+(kA*ae2))), (sf[217]*(((yy+(ku*ab5))+(zN+(kw*adg)))+((-xg)+(kA*ae3)))), (sf[217]*((((((((k8*a5l)+((z4*sf[246])+(ke*ain)))+((sf[58]*xS)+(kr*afh)))+(kb*a7j))+(km*a9T))+((xa*sf[246])+(kw*adk)))+(ky*adM))+(kA*ae4))), (sf[217]*((((((((k8*a4E)+(ke*aih))+((sf[58]*va)+(kb*a6C)))+((sf[58]*wl)+(km*a9U)))+(kw*ado))+((-xb)+(kx*adC)))+(xd+(ky*adN)))+(kA*ae5))), (sf[217]*(((((((((sf[58]*ui)+(k8*a5m))+((sf[58]*z4)+(ke*aio)))+(kr*afi))+(kb*a7k))+(km*a9V))+(kw*ads))+((-xd)+(ky*adO)))+(kA*ae6))), (sf[217]*((((((((((ui*sf[246])+(k8*a5n))+(ke*aip))+((xS*sf[246])+(kr*afj)))+((va*sf[246])+(kb*a7l)))+(km*a9W))+(kw*adw))+(ky*adP))+((-xe)+(kz*adR)))+(kA*ae7))), (sf[217]*((((((k8*a4H)+(ke*aik))+(kb*a6F))+((wl*sf[246])+(km*a9X)))+(kw*adA))+(xg+(kA*ae8)))), (sf[217]*(kr+(ke*ail)))],
            &[],
            &[],
            multiplicity,
        );
        let Dq_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, Dq);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(8),
            multiplicity * (Dq_ddt),
            [3, 5, 6, 7, 8, 9],
            [((av8) * ddt_scale), ((av9) * ddt_scale), ((ava) * ddt_scale), ((avb) * ddt_scale), ((avc) * ddt_scale), ((avd) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let Dr_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, Dr);
        stamper.stamp_current_node3_local(
            Some(6),
            Some(8),
            multiplicity * (Dr_ddt),
            3,
            multiplicity * (((ave) * ddt_scale)),
            6,
            multiplicity * (((avf) * ddt_scale)),
            8,
            multiplicity * (((avg) * ddt_scale)),
        );
        let Ds_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, Ds);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (Ds_ddt),
            [3, 5, 6, 7, 8, 9],
            [((avh) * ddt_scale), ((avi) * ddt_scale), ((avj) * ddt_scale), ((avk) * ddt_scale), ((avl) * ddt_scale), ((avm) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let Dt_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, Dt);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(4),
            multiplicity * (Dt_ddt),
            [3, 4, 5, 7, 8],
            [((avn) * ddt_scale), ((avo) * ddt_scale), ((avp) * ddt_scale), ((avq) * ddt_scale), ((avr) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let Du_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, Du);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(9),
            multiplicity * (Du_ddt),
            [3, 5, 6, 7, 8, 9],
            [((avs) * ddt_scale), ((avt) * ddt_scale), ((avu) * ddt_scale), ((avv) * ddt_scale), ((avw) * ddt_scale), ((avx) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let Dg_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, Dg);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (Dg_ddt),
            1,
            multiplicity * (((sf[223]) * ddt_scale)),
            2,
            multiplicity * (((sf[295]) * ddt_scale)),
        );
        let Di_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, Di);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (Di_ddt),
            0,
            multiplicity * (((sf[296]) * ddt_scale)),
            1,
            multiplicity * (((sf[224]) * ddt_scale)),
        );
        let Dm_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, Dm);
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (Dm_ddt),
            10,
            multiplicity * (((sf[226]) * ddt_scale)),
        );
        let Dp_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, Dp);
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (Dp_ddt),
            11,
            multiplicity * (((sf[297]) * ddt_scale)),
        );
        let Dk_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, Dk);
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (Dk_ddt),
            3,
            multiplicity * (((sf[225]) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(8),
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(8),
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(9),
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(9),
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(4),
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(6),
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(7),
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(8),
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(4),
            multiplicity * (b),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        let br=self.branches;
        let branches=br;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let CommonStampValues {
            b, d, T, ca, fk, fF, fG, fH,
            gE, h1, h8, hf, hm, hU, iV, ji,
            jj, k5, k6, k8, k9, kb, kc, ke,
            kf, kk, km, kn, ko, ks, kB, kC,
            p2, pk, pp, ps, px, pV, q8, qM,
            rm, ro, sd, sA, sB, te, tw, tx,
            ud, uu, uv, v5, vo, vp, vI, vX,
            wJ, wM, Dg, Di, Dk, Dm, Dp, Dq,
            Dr, Ds, Dt, Du, Dz, DB, DC, EG,
            Fu, Fy, FK, FO, G0, G4, Gg, Gk,
            Ic, IN, IQ, IU, Tg, Th, Ti, TQ,
            TR, TS, TT, Un, Uo, Up, Uq, Vn,
            Vo, Vp, Vq, VR, VS, VT, VU, VY,
            Xs, Xt, Xu, Xv, Xw, Xx, YU, YV,
            YW, YX, YY, YZ, Z2, a0H, a0I, a0J,
            a0K, a1p, a1q, a1r, a1s, a1t, a1u, a1v,
            a2F, a2G, a2H, a2I, a3h, a3i, a3j, a3k,
            a3l, a3m, a3n, a57, a58, a59, a5a, a5J,
            a5K, a5L, a5M, a5N, a5O, a5P, a75, a76,
            a77, a78, a7I, a7J, a7K, a7L, a7M, a7N,
            a7P, a8y, a8T, a8U, a8V, a8W, a8X, a8Y,
            aaL, aaM, aaN, aaO, aaP, aaQ, aaZ, ab0,
            ab1, ab2, ab3, av8, av9, ava, avb, avc,
            avd, ave, avf, avg, avh, avi, avj, avk,
            avl, avm, avn, avo, avp, avq, avr, avs,
            avt, avu, avv, avw, avx,
        }=self.eval_common_stamp_values(ctx);
        let p=&(*self.params);
        let m=self.multiplicity;
        let multiplicity=m;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(8),
            &[3, 5, 6, 7, 8, 9],
            &[av8, av9, ava, avb, avc, avd],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3_local(
            Some(6),
            Some(8),
            3,
            multiplicity * (ave),
            6,
            multiplicity * (avf),
            8,
            multiplicity * (avg),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[3, 5, 6, 7, 8, 9],
            &[avh, avi, avj, avk, avl, avm],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(4),
            &[3, 4, 5, 7, 8],
            &[avn, avo, avp, avq, avr],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(9),
            &[3, 5, 6, 7, 8, 9],
            &[avs, avt, avu, avv, avw, avx],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(2),
            1,
            multiplicity * (sf[223]),
            2,
            multiplicity * (sf[295]),
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(0),
            0,
            multiplicity * (sf[296]),
            1,
            multiplicity * (sf[224]),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(10),
            None,
            10,
            multiplicity * (sf[226]),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(11),
            None,
            11,
            multiplicity * (sf[297]),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(3),
            None,
            3,
            multiplicity * (sf[225]),
        );
    }
}
