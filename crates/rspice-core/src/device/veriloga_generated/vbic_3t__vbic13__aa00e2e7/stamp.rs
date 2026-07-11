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
    a: f64, b: f64, c: f64, s: f64, A: f64, K: f64,
    N: f64, X: f64, aM: f64, bb: f64, bp: f64, br_: f64,
    eK: f64, f5: f64, f6: f64, f7: f64, g9: f64, gw: f64,
    gD: f64, gK: f64, gR: f64, hp: f64, iq: f64, iN: f64,
    iO: f64, jA: f64, jB: f64, jD: f64, jE: f64, jG: f64,
    jH: f64, jJ: f64, jK: f64, jP: f64, jR: f64, jS: f64,
    jT: f64, jX: f64, k6: f64, k7: f64, oJ: f64, p1: f64,
    p6: f64, p9: f64, pe: f64, pF: f64, pS: f64, qy: f64,
    rb: f64, rd: f64, s7: f64, sw: f64, sx: f64, ta: f64,
    ts: f64, tt: f64, u9: f64, uq: f64, ur: f64, v1: f64,
    vk: f64, vl: f64, vI: f64, vX: f64, ws: f64, wJ: f64,
    wM: f64, Dv: f64, Dx: f64, Dz: f64, DB: f64, DE: f64,
    DF: f64, DG: f64, DH: f64, DI: f64, DJ: f64, DO: f64,
    DQ: f64, DR: f64, EV: f64, FJ: f64, FN: f64, FZ: f64,
    G3: f64, Gf: f64, Gj: f64, Gv: f64, Gz: f64, Ir: f64,
    J2: f64, J5: f64, J9: f64, JY: f64, Tv: f64, Tw: f64,
    Tx: f64, U5: f64, U6: f64, U7: f64, U8: f64, UC: f64,
    UD: f64, UE: f64, UF: f64, VC: f64, VD: f64, VE: f64,
    VF: f64, W6: f64, W7: f64, W8: f64, W9: f64, Wd: f64,
    XH: f64, XI: f64, XJ: f64, XK: f64, XL: f64, XM: f64,
    Z9: f64, Za: f64, Zb: f64, Zc: f64, Zd: f64, Ze: f64,
    Zh: f64, a0W: f64, a0X: f64, a0Y: f64, a0Z: f64, a1E: f64,
    a1F: f64, a1G: f64, a1H: f64, a1I: f64, a1J: f64, a1K: f64,
    a2U: f64, a2V: f64, a2W: f64, a2X: f64, a3w: f64, a3x: f64,
    a3y: f64, a3z: f64, a3A: f64, a3B: f64, a3C: f64, a5m: f64,
    a5n: f64, a5o: f64, a5p: f64, a5Y: f64, a5Z: f64, a60: f64,
    a61: f64, a62: f64, a63: f64, a64: f64, a7k: f64, a7l: f64,
    a7m: f64, a7n: f64, a7X: f64, a7Y: f64, a7Z: f64, a80: f64,
    a81: f64, a82: f64, a84: f64, a8N: f64, a98: f64, a99: f64,
    a9a: f64, a9b: f64, a9c: f64, a9d: f64, ab0: f64, ab1: f64,
    ab2: f64, ab3: f64, ab4: f64, ab5: f64, abe: f64, abf: f64,
    abg: f64, abh: f64, abi: f64, avn: f64, avo: f64, avp: f64,
    avq: f64, avr: f64, avs: f64, avt: f64, avu: f64, avv: f64,
    avw: f64, avx: f64, avy: f64, avz: f64, avA: f64, avB: f64,
    avC: f64, avD: f64, avE: f64, avF: f64, avG: f64, avH: f64,
    avI: f64, avJ: f64, avK: f64, avL: f64, avM: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let a=if ctx.analysis_initial_step(){1.0}else{0.0};let b=0.0;let c=1.0;let s=(if (((a)!=0.0)&&sb[1]){c}else{(if (((a)!=0.0)&&((sf[2])!=0.0)){sf[3]}else{b})});let y=(((a)!=0.0)&&sb[2]);let A=-1.0;let E=(y&&sb[3]);let K=(if (E&&sb[4]){c}else{(if (((sf[6])!=0.0)&&E){sf[7]}else{(if (((sf[5])!=0.0)&&y){A}else{(if (((a)!=0.0)&&((sf[4])!=0.0)){c}else{b})})})});let N=(if ((a)!=0.0){sf[9]}else{b});let S=(if ((a)!=0.0){sf[12]}else{b});let X=(if ((a)!=0.0){sf[15]}else{b});let a7=(if ((a)!=0.0){sf[21]}else{b});let ac=(if ((a)!=0.0){sf[24]}else{b});let af=273.15;let ai=(if ((a)!=0.0){sf[27]}else{b});let aI=1.380662e-23;let aK=1.602189e-19;let aM=(sf[253]/ai);let b3=(if sb[11]{b}else{(if ((sf[35])!=0.0){(sf[256]*((sf[258]+(s/sf[34]))).ln())}else{b})});let bb=(c-aM);let bg=((sf[33]*f64::powf(aM,sf[41]))*(((sf[43]*bb)/sf[259])).exp());let bh=(bg>b);let bi=(if bh{c}else{b});let bn=(if (sb[12]&&(s>sf[44])){c}else{b});let bp=0.5;let bq=(s*bp);let br_=4.0;let bM=(if (!((bi)!=0.0)){b}else{(if (((bi)!=0.0)&&(!((bn)!=0.0))){(sf[259]*((c+(s/bg))).ln())}else{(if (((bi)!=0.0)&&((bn)!=0.0)){(sf[259]*((c+(f64::powf((bq*sf[47]),sf[49])/bg))).ln())}else{b})})});let bZ=((sf[50]*f64::powf(aM,sf[53]))*(((bb*sf[55])/sf[260])).exp());let c2=(if (bh&&(bZ>b)){c}else{b});let c5=(if (sb[5]&&(s>sf[10])){c}else{b});let cb=(bg*bZ);let cp=(if (!((c2)!=0.0)){b}else{(if (((c2)!=0.0)&&(!((c5)!=0.0))){(sf[260]*((c+(s/cb))).ln())}else{(if (((c2)!=0.0)&&((c5)!=0.0)){(sf[260]*((c+(f64::powf((bq*sf[57]),sf[49])/cb))).ln())}else{b})})});let cB=((sf[58]*f64::powf(aM,sf[60]))*(((bb*sf[62])/sf[261])).exp());let cD=(if (cB>b){c}else{b});let cG=(if (sb[6]&&(s>sf[13])){c}else{b});let cX=(if (!((cD)!=0.0)){b}else{(if (((cD)!=0.0)&&(!((cG)!=0.0))){(sf[261]*((c+(s/cB))).ln())}else{(if (((cD)!=0.0)&&((cG)!=0.0)){(sf[261]*((c+((X*(s*s))/cB))).ln())}else{b})})});let da=((sf[63]*f64::powf(aM,sf[66]))*(((bb*sf[68])/sf[262])).exp());let dc=(if (da>b){c}else{b});let dj=(if (!((dc)!=0.0)){b}else{(if ((dc)!=0.0){(sf[262]*((c+(s/da))).ln())}else{b})});let dJ=f64::powf(aM,sf[77]);let dQ=(((bb*sf[79])/sf[264])).exp();let dR=((sf[75]*dJ)*dQ);let dT=(if (dR>b){c}else{b});let e0=(if (!((dT)!=0.0)){b}else{(if ((dT)!=0.0){(sf[264]*((c+(s/dR))).ln())}else{b})});let eo=(dQ*(dJ*sf[85]));let eq=(if (eo>b){c}else{b});let ex=(if (!((eq)!=0.0)){b}else{(if ((eq)!=0.0){(sf[264]*((c+(s/eo))).ln())}else{b})});let eK=ctx.node_voltage(n[3]);let eM=((sf[239]+eK)-af);let eO=(if (eM<sf[30]){c}else{b});let eR=(((eM-sf[29])-c)).exp();let eT=(if ((eO)!=0.0){(sf[29]+eR)}else{eM});let eX=((((if (eT>sf[32]){c}else{b}))!=0.0)&&(!((eO)!=0.0)));let f0=(((sf[31]-eT)-c)).exp();let f3=(af+(if eX{(sf[31]-f0)}else{eT}));let f5=((aI*f3)/aK);let f6=(f3/ai);let f7=(f3-ai);let fa=(sf[44]*f64::powf(f6,sf[87]));let g8=(sf[33]*f64::powf(f6,sf[41]));let g9=(c-f6);let ga=(sf[43]*g9);let gb=(sf[40]*f5);let gd=((ga/gb)).exp();let ge=(g8*gd);let gg=(sf[50]*f64::powf(f6,sf[53]));let gh=(sf[55]*g9);let gi=(sf[52]*f5);let gk=((gh/gi)).exp();let gl=(gg*gk);let gn=(sf[58]*f64::powf(f6,sf[60]));let go=(sf[62]*g9);let gp=(sf[59]*f5);let gr=((go/gp)).exp();let gs=(gn*gr);let gw=(sf[65]*f5);let gD=(sf[71]*f5);let gK=(sf[76]*f5);let gR=(sf[81]*f5);let h1=(c+(f7*sf[109]));let h2=(sf[40]*h1);let h3=(sf[52]*h1);let hh=(sf[114]+(f7*sf[115]));let ho=(sf[36]*(c+(f7*sf[116])));let hp=2.0;let hr=(hp*(f5/f6));let hu=(f6*sf[118]);let hw=((hu/f5)).exp();let hx=-0.5;let hz=(f6*sf[119]);let hB=((hz/f5)).exp();let hC=(hw-hB);let hD=(hC).ln();let hE=(hr*hD);let hG=3.0;let hH=(f5*hG);let hI=(f6).ln();let hJ=(hH*hI);let hL=(f6-c);let hN=(((f6*hE)-hJ)-(sf[67]*hL));let hO=(f5*hp);let hP=(-hN);let hR=((hP/f5)).exp();let hU=((c+(br_*hR))).sqrt();let hW=(bp*(c+hU));let hX=(hW).ln();let hZ=(hN+(hO*hX));let i2=(f6*sf[121]);let i4=((i2/f5)).exp();let i6=(f6*sf[122]);let i8_=((i6/f5)).exp();let i9=(i4-i8_);let ia=(i9).ln();let ib=(hr*ia);let if_=(((f6*ib)-hJ)-(sf[78]*hL));let ig=(-if_);let ii=((ig/f5)).exp();let il=((c+(br_*ii))).sqrt();let in_=(bp*(c+il));let io=(in_).ln();let iq=(if_+(hO*io));let is=(sf[117]/hZ);let iv=(sf[123]*f64::powf(is,sf[124]));
        let ix=(sf[120]/iq);let iz=f64::powf(ix,sf[126]);let iA=(sf[125]*iz);let iC=(iz*sf[127]);let iF=(sf[128]*f64::powf(f6,sf[39]));let iH=((ga/f5)).exp();let iI=(iF*iH);let iN=(-(sf[37]*(c+(f7*hh))));let iO=(f5*ho);let iV=(sf[131]*(c+(f7*sf[132])));let j0=(sf[133]*(c+(f7*sf[134])));let jo=(iV>b);let jq=(if jo{(c/iV)}else{b});let jr=(j0>b);let jt=(if jr{(c/j0)}else{b});let ju=(fa>b);let jw=(if ju{(c/fa)}else{b});let jA=ctx.node_voltage(n[7]);let jB=ctx.node_voltage(n[8]);let jD=(K*(jA-jB));let jE=ctx.node_voltage(n[6]);let jG=(K*(jE-jB));let jH=ctx.node_voltage(n[5]);let jJ=(K*(jA-jH));let jK=ctx.node_voltage(n[4]);let jM=(K*(jA-jK));let jP=ctx.node_voltage(n[9]);let jR=(K*(jE-jP));let jS=ctx.node_voltage(n[1]);let jT=ctx.node_voltage(n[2]);let jX=ctx.node_voltage(n[0]);let k6=ctx.node_voltage(n[10]);let k7=ctx.node_voltage(n[11]);let k8=(-hZ);let ka=(k8*sf[135]);let ke=(jD+ka);let kf=(if ((sf[137])!=0.0){ke}else{b});let kh=(if (kf>b){c}else{b});let ki=(((sf[137])!=0.0)&&((kh)!=0.0));let km=(if ki{sf[140]}else{b});let ko=(c-(sf[138]*km));let ku=(kf*sf[142]);let kv=(hZ*sf[138]);let kx=(c+(ku/kv));let kC=(((sf[137])!=0.0)&&(!((kh)!=0.0)));let kE=(c-(jD/hZ));let kG=(c-f64::powf(kE,sf[141]));let kJ=(if kC{((hZ*kG)/sf[141])}else{(if ki{((hZ*ko)/sf[141])}else{b})});let kS=(((ka*ka)+sf[144])).sqrt();let kW=(if sb[19]{(hx*(ka+(if sb[19]{kS}else{b})))}else{b});let kY=(c-(kW/hZ));let kZ=f64::powf(kY,sf[141]);let l2=(if sb[19]{((k8*kZ)/sf[141])}else{b});let l3=(if sb[19]{ke}else{b});let l6=((sf[144]+(l3*l3))).sqrt();let lb=(if sb[19]{((bp*(l3-(if sb[19]{l6}else{b})))-ka)}else{b});let ld=(c-(lb/hZ));let le=f64::powf(ld,sf[141]);let lj=(kW+(jD-lb));let lk=(sf[140]*lj);let ll=(sf[142]*lj);let ln=(c+(ll/kv));let lr=(if sb[19]{(((if sb[19]{((k8*le)/sf[141])}else{kJ})+(lk*ln))-l2)}else{(if ((sf[137])!=0.0){(kJ+(if kC{b}else{(if ki{(km*(kf*kx))}else{b})}))}else{b})});let ls=(-iq);let lt=(sf[135]*ls);let lx=(jJ+lt);let ly=(if ((sf[146])!=0.0){lx}else{b});let lA=(if (ly>b){c}else{b});let lB=(((sf[146])!=0.0)&&((lA)!=0.0));let lE=(if lB{sf[148]}else{b});let lH=(c-(sf[138]*(sf[138]*lE)));let lN=(ly*sf[150]);let lP=(sf[138]+(lN/iq));let lY=(if (sb[21]&&(jJ<sf[152])){c}else{b});let m0=(((sf[146])!=0.0)&&(!((lA)!=0.0)));let m1=(((lY)!=0.0)&&m0);let m3=(c+(sf[151]/iq));let m4=f64::powf(m3,sf[149]);let m6=(sf[149]*(jJ+sf[151]));let m7=(iq+sf[151]);let m9=(c-(m6/m7));let mb=(c-(m4*m9));let mg=(m0&&(!((lY)!=0.0)));let mi=(c-(jJ/iq));let mk=(c-f64::powf(mi,sf[149]));let mn=(if mg{((iq*mk)/sf[149])}else{(if m1{((iq*mb)/sf[149])}else{(if lB{((iq*lH)/sf[149])}else{b})})});let mx=(lt+sf[151]);let my=(sf[151]-lt);let mA=(if sb[25]{(mx/my)}else{b});let mB=(hp*mA);let mC=(mA-c);let mH=(((mC*mC)+sf[156])).sqrt();let mI=(c+mA);let mN=(((mI*mI)+sf[158])).sqrt();let mO=(mH+mN);let mQ=(if sb[25]{(mB/mO)}else{b});let mV=(if sb[25]{(bp*(((my*mQ)-sf[151])-lt))}else{b});let mX=(c-(mV/iq));let mZ=(c-f64::powf(mX,sf[149]));let n2=(if sb[25]{((iq*mZ)/sf[149])}else{b});let n5=(lt+(sf[151]+(hp*jJ)));let n7=(if sb[25]{(n5/my)}else{b});let n8=(hp*n7);let n9=(n7-c);let nc=((sf[156]+(n9*n9))).sqrt();let nd=(c+n7);let ng=((sf[158]+(nd*nd))).sqrt();let nh=(nc+ng);let nj=(if sb[25]{(n8/nh)}else{b});let no=(if sb[25]{(bp*(((my*nj)-sf[151])-lt))}else{b});let nq=(c-(no/iq));let ns=(c-f64::powf(nq,sf[149]));let nv=(if sb[25]{((iq*ns)/sf[149])}else{mn});let ny=(if sb[25]{(bp*(c+nj))}else{b});let nB=(if sb[25]{f64::powf(m3,sf[159])}else{b});let nD=(c+(lt/iq));let nF=(if sb[25]{f64::powf(nD,sf[159])}else{b});let nG=(c-ny);let nK=(if sb[25]{((nB*nG)+(ny*nF))}else{b});let nM=(mV+(jJ-no));let nW=((sf[156]+(lt*lt))).sqrt();let o0=(if sb[27]{(hx*(lt+(if sb[27]{nW}else{b})))}else{mV});let o2=(c-(o0/iq));let o3=f64::powf(o2,sf[149]);let o6=(if sb[27]{((ls*o3)/sf[149])}else{b});let o7=(if sb[27]{lx}else{b});let oa=((sf[156]+(o7*o7))).sqrt();let of=(if sb[27]{((bp*(o7-(if sb[27]{oa}else{b})))-lt)}else{no});let oh=(c-(of/iq));let oi=f64::powf(oh,sf[149]);
        let os=(if sb[27]{(((if sb[27]{((ls*oi)/sf[149])}else{nv})+(sf[160]*(o0+(jJ-of))))-o6)}else{(if sb[25]{((nv+(if sb[25]{(nK*nM)}else{b}))-n2)}else{(if ((sf[146])!=0.0){(mn+(if m0{b}else{(if lB{(lE*(ly*lP))}else{b})}))}else{b})})});let ot=(f5*h2);let ou=(c/ot);let ow=(if (jD<bM){c}else{b});let oy=((jD*ou)).exp();let oA=(!((ow)!=0.0));let oC=((bM*ou)).exp();let oD=(jD-bM);let oF=(c+(ou*oD));let oH=(if oA{(oC*oF)}else{(if ((ow)!=0.0){oy}else{b})});let oI=(oH-c);let oJ=(ge*oI);let oK=(f5*h3);let oL=(c/oK);let oN=(if (jJ<cp){c}else{b});let oP=((jJ*oL)).exp();let oR=(!((oN)!=0.0));let oT=((cp*oL)).exp();let oU=(jJ-cp);let oW=(c+(oL*oU));let oY=(if oR{(oT*oW)}else{(if ((oN)!=0.0){oP}else{oH})});let oZ=(ge*gl);let p0=(oY-c);let p1=(oZ*p0);let p6=0.0001;let p7=(((c+(jt*lr))+(jq*os))-p6);let p9=1e-8;let pb=(((p7*p7)+p9)).sqrt();let pe=(p6+(bp*(p7+pb)));let pn=(br_*((jw*oJ)+(S*p1)));let pp=(if ((sf[162])!=0.0){(f64::powf(pe,sf[163])+pn)}else{b});let pr=(if (pp>p9){c}else{b});let ps=(((sf[162])!=0.0)&&((pr)!=0.0));let py=(((sf[162])!=0.0)&&(!((pr)!=0.0)));let pF=(if sb[29]{(c+pn)}else{pp});let pH=(if (pF>p9){c}else{b});let pI=(sb[29]&&((pH)!=0.0));let pJ=(bp*pe);let pL=(c+f64::powf(pF,sf[46]));let pP=(sb[29]&&(!((pH)!=0.0)));let pS=(if pP{(pJ*sf[165])}else{(if pI{(pJ*pL)}else{(if py{(bp*(pe+sf[164]))}else{(if ps{(bp*(pe+f64::powf(pp,sf[46])))}else{b})})})});let pY=(if ((sf[166])!=0.0){(c/gp)}else{oL});let q0=(if (jR<cX){c}else{b});let q1=(((sf[166])!=0.0)&&((q0)!=0.0));let q3=((jR*pY)).exp();let q6=(((sf[166])!=0.0)&&(!((q0)!=0.0)));let q8=((cX*pY)).exp();let q9=(jR-cX);let qb=(c+(pY*q9));let qd=(if q6{(q8*qb)}else{(if q1{q3}else{oY})});let qf=(if (jJ<cX){c}else{b});let qg=(((sf[166])!=0.0)&&((qf)!=0.0));let qi=((jJ*pY)).exp();let ql=(((sf[166])!=0.0)&&(!((qf)!=0.0)));let qm=(jJ-cX);let qo=(c+(pY*qm));let qq=(if ql{(q8*qo)}else{(if qg{qi}else{b})});let qw=(((qd*sf[167])+(qq*sf[168]))-c);let qy=(if ((sf[166])!=0.0){(gs*qw)}else{b});let qV=(c/gw);let qW=(if ((sf[170])!=0.0){qV}else{pY});let qY=(if (jD<dj){c}else{b});let qZ=(((sf[170])!=0.0)&&((qY)!=0.0));let r1=((jD*qW)).exp();let r3=(!((qY)!=0.0));let r4=(((sf[170])!=0.0)&&r3);let r6=((dj*qW)).exp();let r7=(jD-dj);let r9=(c+(qW*r7));let rb=(if r4{(r6*r9)}else{(if qZ{r1}else{qd})});let rc=(c/gD);let rd=(if ((sf[170])!=0.0){rc}else{qW});let rP=(iN-jD);let rQ=(if sb[38]{rP}else{b});let rR=(c/iO);let rS=(if sb[38]{rR}else{rd});let rU=(if (rQ<b3){c}else{b});let rV=(sb[38]&&((rU)!=0.0));let rX=((rQ*rS)).exp();let s0=(sb[38]&&(!((rU)!=0.0)));let s2=((b3*rS)).exp();let s3=(rQ-b3);let s5=(c+(rS*s3));let s7=(if s0{(s2*s5)}else{(if rV{rX}else{qq})});let sh=(if sb[41]{qV}else{rS});let sj=(if (jG<dj){c}else{b});let sk=(sb[41]&&((sj)!=0.0));let sm=((jG*sh)).exp();let so=(!((sj)!=0.0));let sp=(sb[41]&&so);let sr=((dj*sh)).exp();let ss=(jG-dj);let su=(c+(sh*ss));let sw=(if sp{(sr*su)}else{(if sk{sm}else{rb})});let sx=(if sb[41]{rc}else{sh});let sU=(if sb[42]{rP}else{rQ});let sV=(if sb[42]{rR}else{sx});let sX=(if (sU<b3){c}else{b});let sY=(sb[42]&&((sX)!=0.0));let t0=((sU*sV)).exp();let t3=(sb[42]&&(!((sX)!=0.0)));let t5=((b3*sV)).exp();let t6=(sU-b3);let t8=(c+(sV*t6));let ta=(if t3{(t5*t8)}else{(if sY{t0}else{s7})});let th=(if sb[44]{qV}else{sV});let ti=(((qY)!=0.0)&&sb[44]);let tk=((jD*th)).exp();let tm=(r3&&sb[44]);let to=((dj*th)).exp();let tq=(c+(r7*th));let ts=(if tm{(to*tq)}else{(if ti{tk}else{sw})});let tt=(if sb[44]{rc}else{th});let tT=(if sb[47]{rP}else{sU});let tU=(if sb[47]{rR}else{tt});let tW=(if (tT<b3){c}else{b});let tX=(sb[47]&&((tW)!=0.0));let tZ=((tT*tU)).exp();let u2=(sb[47]&&(!((tW)!=0.0)));let u4=((b3*tU)).exp();let u5=(tT-b3);let u7=(c+(tU*u5));let u9=(if u2{(u4*u7)}else{(if tX{tZ}else{ta})});let uf=(if sb[44]{qV}else{tU});let ug=(((sj)!=0.0)&&sb[44]);let ui=((jG*uf)).exp();let uk=(so&&sb[44]);let um=((dj*uf)).exp();let uo=(c+(ss*uf));let uq=(if uk{(um*uo)}else{(if ug{ui}else{ts})});let ur=(if sb[44]{rc}else{uf});let uL=(if sb[47]{rP}else{tT});let uM=(if sb[47]{rR}else{ur});let uO=(if (uL<b3){c}else{b});let uP=(sb[47]&&((uO)!=0.0));let uR=((uL*uM)).exp();
        let uU=(sb[47]&&(!((uO)!=0.0)));let uW=((b3*uM)).exp();let uX=(uL-b3);let uZ=(c+(uM*uX));let v1=(if uU{(uW*uZ)}else{(if uP{uR}else{u9})});let v7=(c/gK);let v9=(if (jJ<e0){c}else{b});let vb=((jJ*v7)).exp();let vd=(!((v9)!=0.0));let vf=((e0*v7)).exp();let vg=(jJ-e0);let vi=(c+(v7*vg));let vk=(if vd{(vf*vi)}else{(if ((v9)!=0.0){vb}else{uq})});let vl=(c/gR);let vI=(if ((sf[178])!=0.0){v7}else{vl});let vK=(if (jR<ex){c}else{b});let vL=(((sf[178])!=0.0)&&((vK)!=0.0));let vN=((jR*vI)).exp();let vQ=(((sf[178])!=0.0)&&(!((vK)!=0.0)));let vS=((ex*vI)).exp();let vT=(jR-ex);let vV=(c+(vI*vT));let vX=(if vQ{(vS*vV)}else{(if vL{vN}else{vk})});let wm=(jJ/f5);let wo=(if (wm<N){c}else{b});let wp=(wm).exp();let wr=(!((wo)!=0.0));let ws=(N).exp();let ww=(if wr{(ws*(c+(wm-N)))}else{(if ((wo)!=0.0){wp}else{vX})});let wx=(jM/f5);let wz=(if (wx<N){c}else{b});let wA=(wx).exp();let wC=(!((wz)!=0.0));let wG=(if wC{(ws*(c+(wx-N)))}else{(if ((wz)!=0.0){wA}else{v1})});let wJ=((c+(iI*ww))).sqrt();let wM=((c+(iI*wG))).sqrt();let A3=(jG+ka);let A4=(if ((sf[137])!=0.0){A3}else{b});let A6=(if (A4>b){c}else{b});let A7=(((sf[137])!=0.0)&&((A6)!=0.0));let A8=(if A7{sf[140]}else{b});let Aa=(c-(sf[138]*A8));let Ae=(sf[142]*A4);let Ag=(c+(Ae/kv));let Al=(((sf[137])!=0.0)&&(!((A6)!=0.0)));let An=(c-(jG/hZ));let Ap=(c-f64::powf(An,sf[141]));let As=(if Al{((hZ*Ap)/sf[141])}else{(if A7{((hZ*Aa)/sf[141])}else{b})});let Aw=(if sb[19]{A3}else{b});let Az=((sf[144]+(Aw*Aw))).sqrt();let AE=(if sb[19]{((bp*(Aw-(if sb[19]{Az}else{b})))-ka)}else{b});let AG=(c-(AE/hZ));let AH=f64::powf(AG,sf[141]);let AM=(kW+(jG-AE));let AN=(sf[140]*AM);let AO=(sf[142]*AM);let AQ=(c+(AO/kv));let AU=(if sb[19]{(((if sb[19]{((k8*AH)/sf[141])}else{As})+(AN*AQ))-l2)}else{(if ((sf[137])!=0.0){(As+(if Al{b}else{(if A7{(A8*(A4*Ag))}else{b})}))}else{b})});let AV=(jR+lt);let AW=(if ((sf[146])!=0.0){AV}else{b});let AY=(if (AW>b){c}else{b});let AZ=(((sf[146])!=0.0)&&((AY)!=0.0));let B0=(if AZ{sf[148]}else{b});let B3=(c-(sf[138]*(sf[138]*B0)));let B7=(sf[150]*AW);let B9=(sf[138]+(B7/iq));let Bf=(if (sb[21]&&(jR<sf[152])){c}else{b});let Bh=(((sf[146])!=0.0)&&(!((AY)!=0.0)));let Bi=(((Bf)!=0.0)&&Bh);let Bk=(sf[149]*(jR+sf[151]));let Bm=(c-(Bk/m7));let Bo=(c-(m4*Bm));let Bt=(Bh&&(!((Bf)!=0.0)));let Bv=(c-(jR/iq));let Bx=(c-f64::powf(Bv,sf[149]));let BA=(if Bt{((iq*Bx)/sf[149])}else{(if Bi{((iq*Bo)/sf[149])}else{(if AZ{((iq*B3)/sf[149])}else{b})})});let BG=(lt+(sf[151]+(hp*jR)));let BI=(if sb[25]{(BG/my)}else{b});let BJ=(hp*BI);let BK=(BI-c);let BN=((sf[156]+(BK*BK))).sqrt();let BO=(c+BI);let BR=((sf[158]+(BO*BO))).sqrt();let BS=(BN+BR);let BU=(if sb[25]{(BJ/BS)}else{b});let BZ=(if sb[25]{(bp*(((my*BU)-sf[151])-lt))}else{b});let C1=(c-(BZ/iq));let C3=(c-f64::powf(C1,sf[149]));let C6=(if sb[25]{((iq*C3)/sf[149])}else{BA});let C9=(if sb[25]{(bp*(c+BU))}else{b});let Ca=(c-C9);let Ce=(if sb[25]{((nB*Ca)+(nF*C9))}else{b});let Cg=(mV+(jR-BZ));let Cm=(if sb[27]{AV}else{b});let Cp=((sf[156]+(Cm*Cm))).sqrt();let Cu=(if sb[27]{((bp*(Cm-(if sb[27]{Cp}else{b})))-lt)}else{BZ});let Cw=(c-(Cu/iq));let Cx=f64::powf(Cw,sf[149]);let CG=(if sb[27]{(((if sb[27]{((ls*Cx)/sf[149])}else{C6})+(sf[160]*(o0+(jR-Cu))))-o6)}else{(if sb[25]{((C6+(if sb[25]{(Ce*Cg)}else{b}))-n2)}else{(if ((sf[146])!=0.0){(BA+(if Bh{b}else{(if AZ{(B0*(AW*B9))}else{b})}))}else{b})})});let CI=(if (oJ>b){c}else{b});let CK=(ac*(oJ*CI));let CL=(c+CK);let CM=(CK/CL);let CO=1.44;let CP=((a7*jJ)/CO);let CR=(if (CP<N){c}else{b});let CS=(CP).exp();let CU=(!((CR)!=0.0));let D3=(sf[198]*(c+(pe*sf[199])));let D5=((if CU{(ws*(c+(CP-N)))}else{(if ((CR)!=0.0){CS}else{ww})})*sf[200]);let D7=((if ((a)!=0.0){sf[25]}else{b})+(CM*CM));let Da=(c+(CI*(D5*D7)));let Db=(D3*Da);let De=(oJ*Db);let Dv=((jS-jT)*sf[203]);let Dx=((jS-jX)*sf[204]);let Dz=(eK*sf[205]);let DB=(k6*sf[206]);let DE=((k7*sf[206])*0.3333333333333333);let DF=(K*((sf[169]*(iv*lr))+(De/pS)));let DG=(K*(sf[176]*(iv*AU)));let DH=(K*(((iA*os)+(p1*sf[201]))+(wJ*sf[202])));let DI=(K*(wM*sf[202]));let DJ=(K*((iC*CG)+((if sb[31]{b}else{qy})*sf[201])));let DK=(if ((eO)!=0.0){eR}else{c});
        let DO=(if eX{(-(f0*(-DK)))}else{DK});let DQ=((aI*DO)/aK);let DR=(DO/ai);let EV=(-DR);let EW=(sf[43]*EV);let F6=((gd*(sf[33]*(DR*(sf[41]*f64::powf(f6,sf[216])))))+(g8*(gd*(((gb*EW)-(ga*(sf[40]*DQ)))/(gb*gb)))));let Ft=(sf[59]*DQ);let Fx=(gp*gp);let FJ=(sf[65]*DQ);let FN=(gw*gw);let FZ=(sf[71]*DQ);let G3=(gD*gD);let Gf=(sf[76]*DQ);let Gj=(gK*gK);let Gv=(sf[81]*DQ);let Gz=(gR*gR);let GN=(sf[109]*DO);let H6=(hp*(((f6*DQ)-(f5*DR))/(f6*f6)));let Hb=(f5*f5);let Hw=((hI*(hG*DQ))+(hH*(DR/f6)));let Hz=((((hE*DR)+(f6*((hD*H6)+(hr*(((hw*(((f5*(sf[118]*DR))-(hu*DQ))/Hb))-(hB*(((f5*(sf[119]*DR))-(hz*DQ))/Hb)))/hC)))))-Hw)-(sf[67]*DR));let HA=(hp*DQ);let HP=(Hz+((hX*HA)+(hO*((bp*((br_*(hR*(((f5*(-Hz))-(hP*DQ))/Hb)))/(hp*hU)))/hW))));let Ic=((((ib*DR)+(f6*((ia*H6)+(hr*(((i4*(((f5*(sf[121]*DR))-(i2*DQ))/Hb))-(i8_*(((f5*(sf[122]*DR))-(i6*DQ))/Hb)))/i9)))))-Hw)-(sf[78]*DR));let Ir=(Ic+((io*HA)+(hO*((bp*((br_*(ii*(((f5*(-Ic))-(ig*DQ))/Hb)))/(hp*il)))/in_))));let Iu=(hZ*hZ);let IA=(sf[123]*(((-(sf[117]*HP))/Iu)*(sf[124]*f64::powf(is,sf[223]))));let ID=(iq*iq);let IH=(((-(sf[120]*Ir))/ID)*(sf[126]*f64::powf(ix,sf[183])));let IW=((iH*(sf[128]*(DR*(sf[39]*f64::powf(f6,sf[224])))))+(iF*(iH*(((f5*EW)-(ga*DQ))/Hb))));let J2=(-(sf[37]*((hh*DO)+(f7*(sf[115]*DO)))));let J5=((ho*DQ)+(f5*(sf[36]*(sf[116]*DO))));let J9=(iO*iO);let JY=(-K);let JZ=(-HP);let K0=(sf[135]*JZ);let K1=(if ((sf[137])!=0.0){K0}else{b});let K2=(if ((sf[137])!=0.0){K}else{b});let K3=(if ((sf[137])!=0.0){JY}else{b});let Ka=(sf[138]*HP);let Kb=(kv*(sf[142]*K1));let Ke=(kv*kv);let Kg=((sf[142]*K2)/kv);let Kh=((sf[142]*K3)/kv);let KD=(-(K/hZ));let KE=(-(JY/hZ));let KH=(sf[141]*f64::powf(kE,sf[226]));let KW=(if kC{(((kG*HP)+(hZ*(-((-((-(jD*HP))/Iu))*KH))))/sf[141])}else{(if ki{((ko*HP)/sf[141])}else{b})});let KX=(if kC{((hZ*(-(KD*KH)))/sf[141])}else{b});let KY=(if kC{((hZ*(-(KE*KH)))/sf[141])}else{b});let L8=(ka*K0);let Lf=(if sb[19]{(hx*(K0+(if sb[19]{((L8+L8)/(hp*kS))}else{b})))}else{b});let Ls=(if sb[19]{(((kZ*JZ)+(k8*((-(((hZ*Lf)-(kW*HP))/Iu))*(sf[141]*f64::powf(kY,sf[226])))))/sf[141])}else{b});let Lt=(if sb[19]{K0}else{b});let Lu=(if sb[19]{K}else{b});let Lv=(if sb[19]{JY}else{b});let Lw=(l3*Lt);let Ly=(l3*Lu);let LA=(l3*Lv);let LC=(hp*l6);let LQ=(if sb[19]{((bp*(Lt-(if sb[19]{((Lw+Lw)/LC)}else{b})))-K0)}else{b});let LR=(if sb[19]{(bp*(Lu-(if sb[19]{((Ly+Ly)/LC)}else{b})))}else{b});let LS=(if sb[19]{(bp*(Lv-(if sb[19]{((LA+LA)/LC)}else{b})))}else{b});let M3=(sf[141]*f64::powf(ld,sf[226]));let Mj=(K-LR);let Mk=(JY-LS);let Ml=(Lf+(-LQ));let ML=(if sb[19]{(((if sb[19]{(((le*JZ)+(k8*((-(((hZ*LQ)-(lb*HP))/Iu))*M3)))/sf[141])}else{KW})+((ln*(sf[140]*Ml))+(lk*(((kv*(sf[142]*Ml))-(ll*Ka))/Ke))))-Ls)}else{(if ((sf[137])!=0.0){(KW+(if kC{b}else{(if ki{(km*((kx*K1)+(kf*((Kb-(ku*Ka))/Ke))))}else{b})}))}else{b})});let MM=(if sb[19]{((if sb[19]{((k8*((-(LR/hZ))*M3))/sf[141])}else{KX})+((ln*(sf[140]*Mj))+(lk*((sf[142]*Mj)/kv))))}else{(if ((sf[137])!=0.0){(KX+(if kC{b}else{(if ki{(km*((kx*K2)+(kf*Kg)))}else{b})}))}else{b})});let MN=(if sb[19]{((if sb[19]{((k8*((-(LS/hZ))*M3))/sf[141])}else{KY})+((ln*(sf[140]*Mk))+(lk*((sf[142]*Mk)/kv))))}else{(if ((sf[137])!=0.0){(KY+(if kC{b}else{(if ki{(km*((kx*K3)+(kf*Kh)))}else{b})}))}else{b})});let MO=(-Ir);let MP=(sf[135]*MO);let MQ=(if ((sf[146])!=0.0){MP}else{b});let MR=(if ((sf[146])!=0.0){JY}else{b});let MS=(if ((sf[146])!=0.0){K}else{b});let MZ=(iq*(sf[150]*MQ));let N3=((sf[150]*MR)/iq);let N4=((sf[150]*MS)/iq);let Nm=((-(sf[151]*Ir))/ID);let Nq=(Nm*(sf[149]*f64::powf(m3,sf[227])));let Nv=(m7*m7);let NQ=((iq*(-(m4*(-((sf[149]*JY)/m7)))))/sf[149]);let NR=((iq*(-(m4*(-((K*sf[149])/m7)))))/sf[149]);let O1=(-(JY/iq));let O2=(-(K/iq));let O4=(sf[149]*f64::powf(mi,sf[227]));let Oj=(if mg{(((mk*Ir)+(iq*(-((-((-(jJ*Ir))/ID))*O4))))/sf[149])}else{(if m1{(((mb*Ir)+(iq*(-((m9*Nq)+(m4*(-((-(m6*Ir))/Nv)))))))/sf[149])}else{(if lB{((lH*Ir)/sf[149])}else{b})})});let Ok_=(if mg{((iq*(-(O1*O4)))/sf[149])}else{(if m1{NQ}else{b})});let Ol=(if mg{((iq*(-(O2*O4)))/sf[149])}else{(if m1{NR}else{b})});let Ov=(-MP);let Ow=(my*MP);let Oz=(my*my);
        let OB=(if sb[25]{((Ow-(mx*Ov))/Oz)}else{b});let OD=(mC*OB);let OH=(mI*OB);let OX=(if sb[25]{(bp*(((mQ*Ov)+(my*(if sb[25]{(((mO*(hp*OB))-(mB*(((OD+OD)/(hp*mH))+((OH+OH)/(hp*mN)))))/(mO*mO))}else{b})))-MP))}else{b});let Pb=(if sb[25]{(((mZ*Ir)+(iq*(-((-(((iq*OX)-(mV*Ir))/ID))*(sf[149]*f64::powf(mX,sf[227]))))))/sf[149])}else{b});let Pj=(if sb[25]{((Ow-(n5*Ov))/Oz)}else{b});let Pk=(if sb[25]{((hp*JY)/my)}else{b});let Pl=(if sb[25]{((K*hp)/my)}else{b});let Pn=(hp*Pk);let Po=(hp*Pl);let Pp=(n9*Pj);let Pr=(n9*Pk);let Pt=(n9*Pl);let Pv=(hp*nc);let Pz=(nd*Pj);let PB=(nd*Pk);let PD=(nd*Pl);let PF=(hp*ng);let PP=(nh*nh);let PZ=(if sb[25]{(((nh*(hp*Pj))-(n8*(((Pp+Pp)/Pv)+((Pz+Pz)/PF))))/PP)}else{b});let Q0=(if sb[25]{(((nh*Pn)-(n8*(((Pr+Pr)/Pv)+((PB+PB)/PF))))/PP)}else{b});let Q1=(if sb[25]{(((nh*Po)-(n8*(((Pt+Pt)/Pv)+((PD+PD)/PF))))/PP)}else{b});let Qb=(if sb[25]{(bp*(((nj*Ov)+(my*PZ))-MP))}else{b});let Qc=(if sb[25]{(bp*(my*Q0))}else{b});let Qd=(if sb[25]{(bp*(my*Q1))}else{b});let Qo=(sf[149]*f64::powf(nq,sf[227]));let QD=(if sb[25]{(((ns*Ir)+(iq*(-((-(((iq*Qb)-(no*Ir))/ID))*Qo))))/sf[149])}else{Oj});let QE=(if sb[25]{((iq*(-((-(Qc/iq))*Qo)))/sf[149])}else{Ok_});let QF=(if sb[25]{((iq*(-((-(Qd/iq))*Qo)))/sf[149])}else{Ol});let QJ=(if sb[25]{(bp*PZ)}else{b});let QK=(if sb[25]{(bp*Q0)}else{b});let QL=(if sb[25]{(bp*Q1)}else{b});let QQ=(if sb[25]{(Nm*(sf[159]*f64::powf(m3,sf[228])))}else{b});let QY=(if sb[25]{((((iq*MP)-(lt*Ir))/ID)*(sf[159]*f64::powf(nD,sf[228])))}else{b});let RF=(lt*MP);let RM=(if sb[27]{(hx*(MP+(if sb[27]{((RF+RF)/(hp*nW))}else{b})))}else{OX});let RZ=(if sb[27]{(((o3*MO)+(ls*((-(((iq*RM)-(o0*Ir))/ID))*(sf[149]*f64::powf(o2,sf[227])))))/sf[149])}else{b});let S0=(if sb[27]{MP}else{b});let S1=(if sb[27]{JY}else{b});let S2=(if sb[27]{K}else{b});let S3=(o7*S0);let S5=(o7*S1);let S7=(o7*S2);let S9=(hp*oa);let Sn=(if sb[27]{((bp*(S0-(if sb[27]{((S3+S3)/S9)}else{b})))-MP)}else{Qb});let So=(if sb[27]{(bp*(S1-(if sb[27]{((S5+S5)/S9)}else{b})))}else{Qc});let Sp=(if sb[27]{(bp*(S2-(if sb[27]{((S7+S7)/S9)}else{b})))}else{Qd});let SA=(sf[149]*f64::powf(oh,sf[227]));let T0=(if sb[27]{(((if sb[27]{(((oi*MO)+(ls*((-(((iq*Sn)-(of*Ir))/ID))*SA)))/sf[149])}else{QD})+(sf[160]*(RM+(-Sn))))-RZ)}else{(if sb[25]{((QD+(if sb[25]{((nM*(if sb[25]{(((nG*QQ)+(nB*(-QJ)))+((nF*QJ)+(ny*QY)))}else{b}))+(nK*(OX+(-Qb))))}else{b}))-Pb)}else{(if ((sf[146])!=0.0){(Oj+(if m0{b}else{(if lB{(lE*((lP*MQ)+(ly*((MZ-(lN*Ir))/ID))))}else{b})}))}else{b})})});let T1=(if sb[27]{((if sb[27]{((ls*((-(So/iq))*SA))/sf[149])}else{QE})+(sf[160]*(JY-So)))}else{(if sb[25]{(QE+(if sb[25]{((nM*(if sb[25]{((nB*(-QK))+(nF*QK))}else{b}))+(nK*(JY-Qc)))}else{b}))}else{(if ((sf[146])!=0.0){(Ok_+(if m0{b}else{(if lB{(lE*((lP*MR)+(ly*N3)))}else{b})}))}else{b})})});let T2=(if sb[27]{((if sb[27]{((ls*((-(Sp/iq))*SA))/sf[149])}else{QF})+(sf[160]*(K-Sp)))}else{(if sb[25]{(QF+(if sb[25]{((nM*(if sb[25]{((nB*(-QL))+(nF*QL))}else{b}))+(nK*(K-Qd)))}else{b}))}else{(if ((sf[146])!=0.0){(Ol+(if m0{b}else{(if lB{(lE*((lP*MS)+(ly*N4)))}else{b})}))}else{b})})});let T8=((-((h2*DQ)+(f5*(sf[40]*GN))))/(ot*ot));let Ta=(K*ou);let Tb=(ou*JY);let Tq=(if oA{((oF*(oC*(bM*T8)))+(oC*(oD*T8)))}else{(if ((ow)!=0.0){(oy*(jD*T8))}else{b})});let Tr=(if oA{(oC*Ta)}else{(if ((ow)!=0.0){(oy*Ta)}else{b})});let Ts=(if oA{(oC*Tb)}else{(if ((ow)!=0.0){(oy*Tb)}else{b})});let Tv=((oI*F6)+(ge*Tq));let Tw=(ge*Tr);let Tx=(ge*Ts);let TD=((-((h3*DQ)+(f5*(sf[52]*GN))))/(oK*oK));let TF=(oL*JY);let TG=(K*oL);let TW=(if oR{((oW*(oT*(cp*TD)))+(oT*(oU*TD)))}else{(if ((oN)!=0.0){(oP*(jJ*TD))}else{Tq})});let TX=(if oR{(oT*TF)}else{(if ((oN)!=0.0){(oP*TF)}else{b})});let TY=(if oR{(oT*TG)}else{(if ((oN)!=0.0){(oP*TG)}else{Tr})});let TZ=(if oR{b}else{(if ((oN)!=0.0){b}else{Ts})});let U5=((p0*((gl*F6)+(ge*((gk*(sf[50]*(DR*(sf[53]*f64::powf(f6,sf[217])))))+(gg*(gk*(((gi*(sf[55]*EV))-(gh*(sf[52]*DQ)))/(gi*gi))))))))+(oZ*TW));let U6=(oZ*TX);let U7=(oZ*TY);let U8=(oZ*TZ);let Ud=(jt*MN);let Uh=(jq*T1);
        let Uj=(((lr*(if jr{((-(sf[133]*(sf[134]*DO)))/(j0*j0))}else{b}))+(jt*ML))+((os*(if jo{((-(sf[131]*(sf[132]*DO)))/(iV*iV))}else{b}))+(jq*T0)));let Uk=((jt*MM)+(jq*T2));let Ul=(p7*Uj);let Un=(p7*Uh);let Up=(p7*Uk);let Ur=(p7*Ud);let Ut=(hp*pb);let UC=(bp*(Uj+((Ul+Ul)/Ut)));let UD=(bp*(Uh+((Un+Un)/Ut)));let UE=(bp*(Uk+((Up+Up)/Ut)));let UF=(bp*(Ud+((Ur+Ur)/Ut)));let UU=(sf[163]*f64::powf(pe,sf[229]));let UZ=(br_*(((oJ*(if ju{((-(sf[44]*(DR*(sf[87]*f64::powf(f6,sf[207])))))/(fa*fa))}else{b}))+(jw*Tv))+(S*U5)));let V0=(br_*(S*U6));let V1=(br_*((jw*Tw)+(S*U7)));let V2=(br_*((jw*Tx)+(S*U8)));let V7=(if ((sf[162])!=0.0){((UC*UU)+UZ)}else{b});let V8=(if ((sf[162])!=0.0){((UD*UU)+V0)}else{b});let V9=(if ((sf[162])!=0.0){((UE*UU)+V1)}else{b});let Va=(if ((sf[162])!=0.0){((UF*UU)+V2)}else{b});let Vd=(sf[46]*f64::powf(pp,sf[230]));let Vu=(bp*UC);let Vv=(bp*UD);let Vw=(bp*UE);let Vx=(bp*UF);let VC=(if sb[29]{UZ}else{V7});let VD=(if sb[29]{V0}else{V8});let VE=(if sb[29]{V1}else{V9});let VF=(if sb[29]{V2}else{Va});let VH=(sf[46]*f64::powf(pF,sf[230]));let W6=(if pP{(sf[165]*Vu)}else{(if pI{((pL*Vu)+(pJ*(VC*VH)))}else{(if py{Vu}else{(if ps{(bp*(UC+(V7*Vd)))}else{b})})})});let W7=(if pP{(sf[165]*Vv)}else{(if pI{((pL*Vv)+(pJ*(VD*VH)))}else{(if py{Vv}else{(if ps{(bp*(UD+(V8*Vd)))}else{b})})})});let W8=(if pP{(sf[165]*Vw)}else{(if pI{((pL*Vw)+(pJ*(VE*VH)))}else{(if py{Vw}else{(if ps{(bp*(UE+(V9*Vd)))}else{b})})})});let W9=(if pP{(sf[165]*Vx)}else{(if pI{((pL*Vx)+(pJ*(VF*VH)))}else{(if py{Vx}else{(if ps{(bp*(UF+(Va*Vd)))}else{b})})})});let Wd=(pS*pS);let WI=(if ((sf[166])!=0.0){((-Ft)/Fx)}else{TD});let WK=(K*pY);let WL=(pY*JY);let WW=(q8*(cX*WI));let X1=(q8*WK);let X2=(q8*WL);let X3=(if q6{((qb*WW)+(q8*(q9*WI)))}else{(if q1{(q3*(jR*WI))}else{TW})});let X4=(if q6{b}else{(if q1{b}else{TX})});let X5=(if q6{X1}else{(if q1{(q3*WK)}else{b})});let X6=(if q6{b}else{(if q1{b}else{TY})});let X7=(if q6{b}else{(if q1{b}else{TZ})});let X8=(if q6{X2}else{(if q1{(q3*WL)}else{b})});let Xk=(if ql{((qo*WW)+(q8*(qm*WI)))}else{(if qg{(qi*(jJ*WI))}else{b})});let Xl=(if ql{X2}else{(if qg{(qi*WL)}else{b})});let Xm=(if ql{X1}else{(if qg{(qi*WK)}else{b})});let XH=(if ((sf[166])!=0.0){((qw*((gr*(sf[58]*(DR*(sf[60]*f64::powf(f6,sf[218])))))+(gn*(gr*(((gp*(sf[62]*EV))-(go*Ft))/Fx)))))+(gs*((sf[167]*X3)+(sf[168]*Xk))))}else{b});let XI=(if ((sf[166])!=0.0){(gs*((sf[167]*X4)+(sf[168]*Xl)))}else{b});let XJ=(if ((sf[166])!=0.0){(gs*(sf[167]*X5))}else{b});let XK=(if ((sf[166])!=0.0){(gs*((sf[167]*X6)+(sf[168]*Xm)))}else{b});let XL=(if ((sf[166])!=0.0){(gs*(sf[167]*X7))}else{b});let XM=(if ((sf[166])!=0.0){(gs*(sf[167]*X8))}else{b});let YN=((-FJ)/FN);let YO=(if ((sf[170])!=0.0){YN}else{WI});let YQ=(K*qW);let YR=(qW*JY);let Z9=(if r4{((r9*(r6*(dj*YO)))+(r6*(r7*YO)))}else{(if qZ{(r1*(jD*YO))}else{X3})});let Za=(if r4{b}else{(if qZ{b}else{X4})});let Zb=(if r4{b}else{(if qZ{b}else{X5})});let Zc=(if r4{(r6*YQ)}else{(if qZ{(r1*YQ)}else{X6})});let Zd=(if r4{(r6*YR)}else{(if qZ{(r1*YR)}else{X7})});let Ze=(if r4{b}else{(if qZ{b}else{X8})});let Zg=((-FZ)/G3);let Zh=(if ((sf[170])!=0.0){Zg}else{YO});let a0v=(if sb[38]{J2}else{b});let a0w=(if sb[38]{JY}else{b});let a0x=(if sb[38]{K}else{b});let a0z=((-J5)/J9);let a0A=(if sb[38]{a0z}else{Zh});let a0B=(rS*a0v);let a0E=(rS*a0w);let a0F=(rS*a0x);let a0W=(if s0{((s5*(s2*(b3*a0A)))+(s2*(a0B+(s3*a0A))))}else{(if rV{(rX*(a0B+(rQ*a0A)))}else{Xk})});let a0X=(if s0{b}else{(if rV{b}else{Xl})});let a0Y=(if s0{(s2*a0E)}else{(if rV{(rX*a0E)}else{Xm})});let a0Z=(if s0{(s2*a0F)}else{(if rV{(rX*a0F)}else{b})});let a1j=(if sb[41]{YN}else{a0A});let a1l=(K*sh);let a1m=(sh*JY);let a1E=(if sp{((su*(sr*(dj*a1j)))+(sr*(ss*a1j)))}else{(if sk{(sm*(jG*a1j))}else{Z9})});let a1F=(if sp{b}else{(if sk{b}else{Za})});let a1G=(if sp{(sr*a1l)}else{(if sk{(sm*a1l)}else{Zb})});let a1H=(if sp{b}else{(if sk{b}else{Zc})});let a1I=(if sp{(sr*a1m)}else{(if sk{(sm*a1m)}else{Zd})});let a1J=(if sp{b}else{(if sk{b}else{Ze})});let a1K=(if sb[41]{Zg}else{a1j});let a2v=(if sb[42]{J2}else{a0v});let a2w=(if sb[42]{JY}else{a0w});let a2x=(if sb[42]{K}else{a0x});
        let a2y=(if sb[42]{a0z}else{a1K});let a2z=(sV*a2v);let a2C=(sV*a2w);let a2D=(sV*a2x);let a2U=(if t3{((t8*(t5*(b3*a2y)))+(t5*(a2z+(t6*a2y))))}else{(if sY{(t0*(a2z+(sU*a2y)))}else{a0W})});let a2V=(if t3{b}else{(if sY{b}else{a0X})});let a2W=(if t3{(t5*a2C)}else{(if sY{(t0*a2C)}else{a0Y})});let a2X=(if t3{(t5*a2D)}else{(if sY{(t0*a2D)}else{a0Z})});let a3b=(if sb[44]{YN}else{a2y});let a3d=(K*th);let a3e=(th*JY);let a3w=(if tm{((tq*(to*(dj*a3b)))+(to*(r7*a3b)))}else{(if ti{(tk*(jD*a3b))}else{a1E})});let a3x=(if tm{b}else{(if ti{b}else{a1F})});let a3y=(if tm{b}else{(if ti{b}else{a1G})});let a3z=(if tm{(to*a3d)}else{(if ti{(tk*a3d)}else{a1H})});let a3A=(if tm{(to*a3e)}else{(if ti{(tk*a3e)}else{a1I})});let a3B=(if tm{b}else{(if ti{b}else{a1J})});let a3C=(if sb[44]{Zg}else{a3b});let a4X=(if sb[47]{J2}else{a2v});let a4Y=(if sb[47]{JY}else{a2w});let a4Z=(if sb[47]{K}else{a2x});let a50=(if sb[47]{a0z}else{a3C});let a51=(tU*a4X);let a54=(tU*a4Y);let a55=(tU*a4Z);let a5m=(if u2{((u7*(u4*(b3*a50)))+(u4*(a51+(u5*a50))))}else{(if tX{(tZ*(a51+(tT*a50)))}else{a2U})});let a5n=(if u2{b}else{(if tX{b}else{a2V})});let a5o=(if u2{(u4*a54)}else{(if tX{(tZ*a54)}else{a2W})});let a5p=(if u2{(u4*a55)}else{(if tX{(tZ*a55)}else{a2X})});let a5D=(if sb[44]{YN}else{a50});let a5F=(K*uf);let a5G=(uf*JY);let a5Y=(if uk{((uo*(um*(dj*a5D)))+(um*(ss*a5D)))}else{(if ug{(ui*(jG*a5D))}else{a3w})});let a5Z=(if uk{b}else{(if ug{b}else{a3x})});let a60=(if uk{(um*a5F)}else{(if ug{(ui*a5F)}else{a3y})});let a61=(if uk{b}else{(if ug{b}else{a3z})});let a62=(if uk{(um*a5G)}else{(if ug{(ui*a5G)}else{a3A})});let a63=(if uk{b}else{(if ug{b}else{a3B})});let a64=(if sb[44]{Zg}else{a5D});let a6Y=(if sb[47]{a0z}else{a64});let a6Z=(uM*(if sb[47]{J2}else{a4X}));let a72=(uM*(if sb[47]{JY}else{a4Y}));let a73=(uM*(if sb[47]{K}else{a4Z}));let a7k=(if uU{((uZ*(uW*(b3*a6Y)))+(uW*(a6Z+(uX*a6Y))))}else{(if uP{(uR*(a6Z+(uL*a6Y)))}else{a5m})});let a7l=(if uU{b}else{(if uP{b}else{a5n})});let a7m=(if uU{(uW*a72)}else{(if uP{(uR*a72)}else{a5o})});let a7n=(if uU{(uW*a73)}else{(if uP{(uR*a73)}else{a5p})});let a7C=((-Gf)/Gj);let a7E=(v7*JY);let a7F=(K*v7);let a7X=(if vd{((vi*(vf*(e0*a7C)))+(vf*(vg*a7C)))}else{(if ((v9)!=0.0){(vb*(jJ*a7C))}else{a5Y})});let a7Y=(if vd{(vf*a7E)}else{(if ((v9)!=0.0){(vb*a7E)}else{a5Z})});let a7Z=(if vd{b}else{(if ((v9)!=0.0){b}else{a60})});let a80=(if vd{(vf*a7F)}else{(if ((v9)!=0.0){(vb*a7F)}else{a61})});let a81=(if vd{b}else{(if ((v9)!=0.0){b}else{a62})});let a82=(if vd{b}else{(if ((v9)!=0.0){b}else{a63})});let a84=((-Gv)/Gz);let a8N=(if ((sf[178])!=0.0){a7C}else{a84});let a8P=(K*vI);let a8Q=(vI*JY);let a98=(if vQ{((vV*(vS*(ex*a8N)))+(vS*(vT*a8N)))}else{(if vL{(vN*(jR*a8N))}else{a7X})});let a99=(if vQ{b}else{(if vL{b}else{a7Y})});let a9a=(if vQ{(vS*a8P)}else{(if vL{(vN*a8P)}else{a7Z})});let a9b=(if vQ{b}else{(if vL{b}else{a80})});let a9c=(if vQ{b}else{(if vL{b}else{a81})});let a9d=(if vQ{(vS*a8Q)}else{(if vL{(vN*a8Q)}else{a82})});let aaf=((-(jJ*DQ))/Hb);let aag=(JY/f5);let aah=(K/f5);let aas=(ws*aag);let aat=(ws*aah);let aau=(if wr{(ws*aaf)}else{(if ((wo)!=0.0){(wp*aaf)}else{a98})});let aav=(if wr{aas}else{(if ((wo)!=0.0){(wp*aag)}else{a99})});let aaw=(if wr{b}else{(if ((wo)!=0.0){b}else{a9a})});let aax=(if wr{aat}else{(if ((wo)!=0.0){(wp*aah)}else{a9b})});let aay=(if wr{b}else{(if ((wo)!=0.0){b}else{a9c})});let aaz=(if wr{b}else{(if ((wo)!=0.0){b}else{a9d})});let aaC=((-(jM*DQ))/Hb);let aaZ=(hp*wJ);let ab0=(((ww*IW)+(iI*aau))/aaZ);let ab1=((iI*aav)/aaZ);let ab2=((iI*aaw)/aaZ);let ab3=((iI*aax)/aaZ);let ab4=((iI*aay)/aaZ);let ab5=((iI*aaz)/aaZ);let abd=(hp*wM);let abe=(((wG*IW)+(iI*(if wC{(ws*aaC)}else{(if ((wz)!=0.0){(wA*aaC)}else{a7k})})))/abd);let abf=((iI*(if wC{aas}else{(if ((wz)!=0.0){(wA*aag)}else{b})}))/abd);let abg=((iI*(if wC{b}else{(if ((wz)!=0.0){b}else{a7l})}))/abd);let abh=((iI*(if wC{aat}else{(if ((wz)!=0.0){(wA*aah)}else{a7m})}))/abd);let abi=((iI*(if wC{b}else{(if ((wz)!=0.0){b}else{a7n})}))/abd);let ams=(sf[141]*f64::powf(An,sf[226]));
        let amH=(if Al{(((Ap*HP)+(hZ*(-((-((-(jG*HP))/Iu))*ams))))/sf[141])}else{(if A7{((Aa*HP)/sf[141])}else{b})});let amI=(if Al{((hZ*(-(KD*ams)))/sf[141])}else{b});let amJ=(if Al{((hZ*(-(KE*ams)))/sf[141])}else{b});let amT=(Aw*Lt);let amV=(Aw*Lu);let amX=(Aw*Lv);let amZ=(hp*Az);let and=(if sb[19]{((bp*(Lt-(if sb[19]{((amT+amT)/amZ)}else{b})))-K0)}else{b});let ane=(if sb[19]{(bp*(Lu-(if sb[19]{((amV+amV)/amZ)}else{b})))}else{b});let anf=(if sb[19]{(bp*(Lv-(if sb[19]{((amX+amX)/amZ)}else{b})))}else{b});let anq=(sf[141]*f64::powf(AG,sf[226]));let anG=(K-ane);let anH=(JY-anf);let anI=(Lf+(-and));let aoQ=(sf[149]*f64::powf(Bv,sf[227]));let ap5=(if Bt{(((Bx*Ir)+(iq*(-((-((-(jR*Ir))/ID))*aoQ))))/sf[149])}else{(if Bi{(((Bo*Ir)+(iq*(-((Bm*Nq)+(m4*(-((-(Bk*Ir))/Nv)))))))/sf[149])}else{(if AZ{((B3*Ir)/sf[149])}else{b})})});let ap6=(if Bt{((iq*(-(O2*aoQ)))/sf[149])}else{(if Bi{NR}else{b})});let ap7=(if Bt{((iq*(-(O1*aoQ)))/sf[149])}else{(if Bi{NQ}else{b})});let apk=(if sb[25]{((Ow-(BG*Ov))/Oz)}else{b});let apm=(BK*apk);let apo=(BK*Pl);let apq=(BK*Pk);let aps=(hp*BN);let apw=(BO*apk);let apy=(BO*Pl);let apA=(BO*Pk);let apC=(hp*BR);let apM=(BS*BS);let apW=(if sb[25]{(((BS*(hp*apk))-(BJ*(((apm+apm)/aps)+((apw+apw)/apC))))/apM)}else{b});let apX=(if sb[25]{(((BS*Po)-(BJ*(((apo+apo)/aps)+((apy+apy)/apC))))/apM)}else{b});let apY=(if sb[25]{(((BS*Pn)-(BJ*(((apq+apq)/aps)+((apA+apA)/apC))))/apM)}else{b});let aq8=(if sb[25]{(bp*(((BU*Ov)+(my*apW))-MP))}else{b});let aq9=(if sb[25]{(bp*(my*apX))}else{b});let aqa=(if sb[25]{(bp*(my*apY))}else{b});let aql=(sf[149]*f64::powf(C1,sf[227]));let aqA=(if sb[25]{(((C3*Ir)+(iq*(-((-(((iq*aq8)-(BZ*Ir))/ID))*aql))))/sf[149])}else{ap5});let aqB=(if sb[25]{((iq*(-((-(aq9/iq))*aql)))/sf[149])}else{ap6});let aqC=(if sb[25]{((iq*(-((-(aqa/iq))*aql)))/sf[149])}else{ap7});let aqG=(if sb[25]{(bp*apW)}else{b});let aqH=(if sb[25]{(bp*apX)}else{b});let aqI=(if sb[25]{(bp*apY)}else{b});let arp=(Cm*S0);let arr=(Cm*S2);let art=(Cm*S1);let arv=(hp*Cp);let arJ=(if sb[27]{((bp*(S0-(if sb[27]{((arp+arp)/arv)}else{b})))-MP)}else{aq8});let arK=(if sb[27]{(bp*(S2-(if sb[27]{((arr+arr)/arv)}else{b})))}else{aq9});let arL=(if sb[27]{(bp*(S1-(if sb[27]{((art+art)/arv)}else{b})))}else{aqa});let arW=(sf[149]*f64::powf(Cw,sf[227]));let ass=(ac*(CI*Tv));let ast=(ac*(CI*Tw));let asu=(ac*(CI*Tx));let asy=(CL*CL);let asK=((a7*JY)/CO);let asL=((K*a7)/CO);let atg=(CM*(((CL*ass)-(CK*ass))/asy));let ati=(CM*(((CL*ast)-(CK*ast))/asy));let atk=(CM*(((CL*asu)-(CK*asu))/asy));let avn=(K*((sf[169]*((lr*IA)+(iv*ML)))+(((pS*((Db*Tv)+(oJ*((Da*(sf[198]*(sf[199]*UC)))+(D3*(CI*((D7*(sf[200]*(if CU{b}else{(if ((CR)!=0.0){b}else{aau})})))+(D5*(atg+atg)))))))))-(De*W6))/Wd)));let avo=(K*(((pS*(oJ*((Da*(sf[198]*(sf[199]*UD)))+(D3*(CI*(D7*(sf[200]*(if CU{(ws*asK)}else{(if ((CR)!=0.0){(CS*asK)}else{aav})}))))))))-(De*W7))/Wd));let avp=(K*((oJ*(D3*(CI*(D7*(sf[200]*(if CU{b}else{(if ((CR)!=0.0){b}else{aaw})}))))))/pS));let avq=(K*((sf[169]*(iv*MM))+(((pS*((Db*Tw)+(oJ*((Da*(sf[198]*(sf[199]*UE)))+(D3*(CI*((D7*(sf[200]*(if CU{(ws*asL)}else{(if ((CR)!=0.0){(CS*asL)}else{aax})})))+(D5*(ati+ati)))))))))-(De*W8))/Wd)));let avr=(K*((sf[169]*(iv*MN))+(((pS*((Db*Tx)+(oJ*((Da*(sf[198]*(sf[199]*UF)))+(D3*(CI*((D7*(sf[200]*(if CU{b}else{(if ((CR)!=0.0){b}else{aay})})))+(D5*(atk+atk)))))))))-(De*W9))/Wd)));let avs=(K*((oJ*(D3*(CI*(D7*(sf[200]*(if CU{b}else{(if ((CR)!=0.0){b}else{aaz})}))))))/pS));let avt=(K*(sf[176]*((AU*IA)+(iv*(if sb[19]{(((if sb[19]{(((AH*JZ)+(k8*((-(((hZ*and)-(AE*HP))/Iu))*anq)))/sf[141])}else{amH})+((AQ*(sf[140]*anI))+(AN*(((kv*(sf[142]*anI))-(AO*Ka))/Ke))))-Ls)}else{(if ((sf[137])!=0.0){(amH+(if Al{b}else{(if A7{(A8*((Ag*K1)+(A4*((Kb-(Ae*Ka))/Ke))))}else{b})}))}else{b})})))));let avu=(K*(sf[176]*(iv*(if sb[19]{((if sb[19]{((k8*((-(ane/hZ))*anq))/sf[141])}else{amI})+((AQ*(sf[140]*anG))+(AN*((sf[142]*anG)/kv))))}else{(if ((sf[137])!=0.0){(amI+(if Al{b}else{(if A7{(A8*((Ag*K2)+(A4*Kg)))}else{b})}))}else{b})}))));
        let avv=(K*(sf[176]*(iv*(if sb[19]{((if sb[19]{((k8*((-(anf/hZ))*anq))/sf[141])}else{amJ})+((AQ*(sf[140]*anH))+(AN*((sf[142]*anH)/kv))))}else{(if ((sf[137])!=0.0){(amJ+(if Al{b}else{(if A7{(A8*((Ag*K3)+(A4*Kh)))}else{b})}))}else{b})}))));let avw=(K*((((os*(sf[125]*IH))+(iA*T0))+(sf[201]*U5))+(sf[202]*ab0)));let avx=(K*(((iA*T1)+(sf[201]*U6))+(sf[202]*ab1)));let avy=(K*(sf[202]*ab2));let avz=(K*(((iA*T2)+(sf[201]*U7))+(sf[202]*ab3)));let avA=(K*((sf[201]*U8)+(sf[202]*ab4)));let avB=(K*(sf[202]*ab5));let avC=(K*(sf[202]*abe));let avD=(K*(sf[202]*abf));let avE=(K*(sf[202]*abg));let avF=(K*(sf[202]*abh));let avG=(K*(sf[202]*abi));let avH=(K*(((CG*(sf[127]*IH))+(iC*(if sb[27]{(((if sb[27]{(((Cx*MO)+(ls*((-(((iq*arJ)-(Cu*Ir))/ID))*arW)))/sf[149])}else{aqA})+(sf[160]*(RM+(-arJ))))-RZ)}else{(if sb[25]{((aqA+(if sb[25]{((Cg*(if sb[25]{(((Ca*QQ)+(nB*(-aqG)))+((C9*QY)+(nF*aqG)))}else{b}))+(Ce*(OX+(-aq8))))}else{b}))-Pb)}else{(if ((sf[146])!=0.0){(ap5+(if Bh{b}else{(if AZ{(B0*((B9*MQ)+(AW*((MZ-(B7*Ir))/ID))))}else{b})}))}else{b})})})))+(sf[201]*(if sb[31]{b}else{XH}))));let avI=(K*(sf[201]*(if sb[31]{b}else{XI})));let avJ=(K*((iC*(if sb[27]{((if sb[27]{((ls*((-(arK/iq))*arW))/sf[149])}else{aqB})+(sf[160]*(K-arK)))}else{(if sb[25]{(aqB+(if sb[25]{((Cg*(if sb[25]{((nB*(-aqH))+(nF*aqH))}else{b}))+(Ce*(K-aq9)))}else{b}))}else{(if ((sf[146])!=0.0){(ap6+(if Bh{b}else{(if AZ{(B0*((B9*MS)+(AW*N4)))}else{b})}))}else{b})})}))+(sf[201]*(if sb[31]{b}else{XJ}))));let avK=(K*(sf[201]*(if sb[31]{b}else{XK})));let avL=(K*(sf[201]*(if sb[31]{b}else{XL})));let avM=(K*((iC*(if sb[27]{((if sb[27]{((ls*((-(arL/iq))*arW))/sf[149])}else{aqC})+(sf[160]*(JY-arL)))}else{(if sb[25]{(aqC+(if sb[25]{((Cg*(if sb[25]{((nB*(-aqI))+(nF*aqI))}else{b}))+(Ce*(JY-aqa)))}else{b}))}else{(if ((sf[146])!=0.0){(ap7+(if Bh{b}else{(if AZ{(B0*((B9*MR)+(AW*N3)))}else{b})}))}else{b})})}))+(sf[201]*(if sb[31]{b}else{XM}))));

        CommonStampValues {
            a, b, c, s, A, K, N, X,
            aM, bb, bp, br_, eK, f5, f6, f7,
            g9, gw, gD, gK, gR, hp, iq, iN,
            iO, jA, jB, jD, jE, jG, jH, jJ,
            jK, jP, jR, jS, jT, jX, k6, k7,
            oJ, p1, p6, p9, pe, pF, pS, qy,
            rb, rd, s7, sw, sx, ta, ts, tt,
            u9, uq, ur, v1, vk, vl, vI, vX,
            ws, wJ, wM, Dv, Dx, Dz, DB, DE,
            DF, DG, DH, DI, DJ, DO, DQ, DR,
            EV, FJ, FN, FZ, G3, Gf, Gj, Gv,
            Gz, Ir, J2, J5, J9, JY, Tv, Tw,
            Tx, U5, U6, U7, U8, UC, UD, UE,
            UF, VC, VD, VE, VF, W6, W7, W8,
            W9, Wd, XH, XI, XJ, XK, XL, XM,
            Z9, Za, Zb, Zc, Zd, Ze, Zh, a0W,
            a0X, a0Y, a0Z, a1E, a1F, a1G, a1H, a1I,
            a1J, a1K, a2U, a2V, a2W, a2X, a3w, a3x,
            a3y, a3z, a3A, a3B, a3C, a5m, a5n, a5o,
            a5p, a5Y, a5Z, a60, a61, a62, a63, a64,
            a7k, a7l, a7m, a7n, a7X, a7Y, a7Z, a80,
            a81, a82, a84, a8N, a98, a99, a9a, a9b,
            a9c, a9d, ab0, ab1, ab2, ab3, ab4, ab5,
            abe, abf, abg, abh, abi, avn, avo, avp,
            avq, avr, avs, avt, avu, avv, avw, avx,
            avy, avz, avA, avB, avC, avD, avE, avF,
            avG, avH, avI, avJ, avK, avL, avM,
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let CommonStampValues {
            a, b, c, s, A, K, N, X,
            aM, bb, bp, br_, eK, f5, f6, f7,
            g9, gw, gD, gK, gR, hp, iq, iN,
            iO, jA, jB, jD, jE, jG, jH, jJ,
            jK, jP, jR, jS, jT, jX, k6, k7,
            oJ, p1, p6, p9, pe, pF, pS, qy,
            rb, rd, s7, sw, sx, ta, ts, tt,
            u9, uq, ur, v1, vk, vl, vI, vX,
            ws, wJ, wM, Dv, Dx, Dz, DB, DE,
            DF, DG, DH, DI, DJ, DO, DQ, DR,
            EV, FJ, FN, FZ, G3, Gf, Gj, Gv,
            Gz, Ir, J2, J5, J9, JY, Tv, Tw,
            Tx, U5, U6, U7, U8, UC, UD, UE,
            UF, VC, VD, VE, VF, W6, W7, W8,
            W9, Wd, XH, XI, XJ, XK, XL, XM,
            Z9, Za, Zb, Zc, Zd, Ze, Zh, a0W,
            a0X, a0Y, a0Z, a1E, a1F, a1G, a1H, a1I,
            a1J, a1K, a2U, a2V, a2W, a2X, a3w, a3x,
            a3y, a3z, a3A, a3B, a3C, a5m, a5n, a5o,
            a5p, a5Y, a5Z, a60, a61, a62, a63, a64,
            a7k, a7l, a7m, a7n, a7X, a7Y, a7Z, a80,
            a81, a82, a84, a8N, a98, a99, a9a, a9b,
            a9c, a9d, ab0, ab1, ab2, ab3, ab4, ab5,
            abe, abf, abg, abh, abi, avn, avo, avp,
            avq, avr, avs, avt, avu, avv, avw, avx,
            avy, avz, avA, avB, avC, avD, avE, avF,
            avG, avH, avI, avJ, avK, avL, avM,
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
        let d=0.01;let l=(if (((a)!=0.0)&&sb[0]){1e-12}else{(if (((a)!=0.0)&&((sf[0])!=0.0)){sf[1]}else{b})});let a2=(if ((a)!=0.0){sf[18]}else{b});let dw=((sf[69]*f64::powf(aM,sf[72]))*(((bb*sf[74])/sf[263])).exp());let dy=(if (dw>b){c}else{b});let dF=(if (!((dy)!=0.0)){b}else{(if ((dy)!=0.0){(sf[263]*((c+(s/dw))).ln())}else{b})});let e4=f64::powf(aM,sf[82]);let eb=(((bb*sf[84])/sf[265])).exp();let ec=((sf[80]*e4)*eb);let ee=(if (ec>b){c}else{b});let el=(if (!((ee)!=0.0)){b}else{(if ((ee)!=0.0){(sf[265]*((c+(s/ec))).ln())}else{b})});let eA=(eb*(e4*sf[86]));let eC=(if (eA>b){c}else{b});let eJ=(if (!((eC)!=0.0)){b}else{(if ((eC)!=0.0){(sf[265]*((c+(s/eA))).ln())}else{b})});let fj=f64::powf(f6,sf[91]);let fl=(if sb[13]{(sf[89]*fj)}else{(if ((sf[88])!=0.0){(sf[89]*f64::powf(f6,sf[90]))}else{b})});let fu=(if sb[14]{(fj*sf[93])}else{(if ((sf[92])!=0.0){(sf[93]*f64::powf(f6,sf[94]))}else{b})});let fD=f64::powf(f6,sf[98]);let fF=(if sb[15]{(sf[96]*fD)}else{(if ((sf[95])!=0.0){(sf[96]*f64::powf(f6,sf[97]))}else{b})});let fO=(if sb[16]{(fD*sf[100])}else{(if ((sf[99])!=0.0){(sf[100]*f64::powf(f6,sf[101]))}else{b})});let fS=(sf[102]*f64::powf(f6,sf[103]));let g1=(if sb[17]{(fj*sf[105])}else{(if ((sf[104])!=0.0){(sf[105]*f64::powf(f6,sf[106]))}else{b})});let g6=(sf[107]*(c+(f7*sf[108])));let gu=(sf[63]*f64::powf(f6,sf[66]));let gv=(sf[68]*g9);let gy=((gv/gw)).exp();let gz=(gu*gy);let gB=(sf[69]*f64::powf(f6,sf[72]));let gC=(sf[74]*g9);let gF=((gC/gD)).exp();let gG=(gB*gF);let gH=f64::powf(f6,sf[77]);let gI=(sf[75]*gH);let gJ=(sf[79]*g9);let gM=((gJ/gK)).exp();let gN=(gI*gM);let gO=f64::powf(f6,sf[82]);let gP=(sf[80]*gO);let gQ=(sf[84]*g9);let gT=((gQ/gR)).exp();let gU=(gP*gT);let gV=(sf[85]*gH);let gW=(gM*gV);let gX=(sf[86]*gO);let gY=(gT*gX);let h8=(sf[110]*(c+(f7*sf[111])));let hd=(sf[112]*(c+(f7*sf[113])));let iM=(sf[129]*f64::powf(f6,sf[130]));let iQ=((iN/iO)).exp();let j1=0.001;let j2=(fl>j1);let j4=1000.0;let j5=(if j2{(c/fl)}else{j4});let j6=(fu>j1);let j8=(if j6{(c/fu)}else{j4});let j9=(fF>j1);let jb=(if j9{(c/fF)}else{j4});let jc=(fO>j1);let je=(if jc{(c/fO)}else{j4});let jf=(fS>j1);let jh=(if jf{(c/fS)}else{j4});let ji=(g1>j1);let jk=(if ji{(c/g1)}else{j4});let jl=(g6>j1);let jn=(if jl{(c/g6)}else{j4});let jx=(iM>b);let jz=(if jx{(c/iM)}else{b});let jO=(K*(jE-jK));let jW=(K*(jH-jB));let jZ=(jX-jK);let k1=(K*(jK-jH));let k2=(jS-jE);let k3=(jE-jA);let k4=(jT-jB);let k5=(jP-jK);let pT=(p1/pS);let pU=(oJ/pS);let qD=(if ((sf[166])!=0.0){(c+(br_*(if ((sf[166])!=0.0){(X*qy)}else{b})))}else{pF});let qF=(if (qD>p9){c}else{b});let qG=(((sf[166])!=0.0)&&((qF)!=0.0));let qH=(qD).sqrt();let qM=(((sf[166])!=0.0)&&(!((qF)!=0.0)));let qR=(if sb[31]{c}else{(if qM{0.50005}else{(if qG{(bp*(c+qH))}else{b})})});let rf=(if (jD<dF){c}else{b});let rg=(((sf[170])!=0.0)&&((rf)!=0.0));let ri=((jD*rd)).exp();let rk=(!((rf)!=0.0));let rl=(((sf[170])!=0.0)&&rk);let rn=((dF*rd)).exp();let ro=(jD-dF);let rq=(c+(rd*ro));let rs=(if rl{(rn*rq)}else{(if rg{ri}else{b})});let rz=(c+(sf[171]*(pe-c)));let rA=(gz*rz);let rB=(rb-c);let rD=(rs-c);let rE=(gG*rD);let rL=(if sb[36]{(rE+(gz*rB))}else{(if sb[34]{((rA*rB)+rE)}else{b})});let sz=(if (jG<dF){c}else{b});let sA=(sb[41]&&((sz)!=0.0));let sC=((jG*sx)).exp();let sE=(!((sz)!=0.0));let sF=(sb[41]&&sE);let sH=((dF*sx)).exp();let sI=(jG-dF);let sK=(c+(sx*sI));let sM=(if sF{(sH*sK)}else{(if sA{sC}else{rs})});let sN=(sw-c);let sP=(sM-c);let sS=(if sb[41]{((gz*sN)+(gG*sP))}else{b});let tu=(((rf)!=0.0)&&sb[44]);let tw=((jD*tt)).exp();let ty=(rk&&sb[44]);let tA=((dF*tt)).exp();let tC=(c+(ro*tt));let tE=(if ty{(tA*tC)}else{(if tu{tw}else{sM})});let tG=(ts-c);let tI=(tE-c);let tJ=(gG*tI);let tR=(if sb[46]{(sf[169]*(tJ+(gz*tG)))}else{(if sb[45]{(sf[169]*((rA*tG)+tJ))}else{(if sb[41]{b}else{(if sb[38]{(rL-(sf[34]*(s7-iQ)))}else{rL})})})});let ue=(if sb[47]{(tR-(sf[175]*(u9-iQ)))}else{tR});let us=(((sz)!=0.0)&&sb[44]);let uu=((jG*ur)).exp();let uw=(sE&&sb[44]);let uy=((dF*ur)).exp();let uA=(c+(sI*ur));let uC=(if uw{(uy*uA)}else{(if us{uu}else{tE})});let uE=(uq-c);let uG=(uC-c);
        let uK=(if sb[44]{(sf[176]*((gz*uE)+(gG*uG)))}else{(if sb[42]{(sS-(sf[34]*(ta-iQ)))}else{sS})});let v6=(if sb[47]{(uK-(sf[177]*(v1-iQ)))}else{uK});let vn=(if (jJ<el){c}else{b});let vp=((jJ*vl)).exp();let vr=(!((vn)!=0.0));let vt=((el*vl)).exp();let vu=(jJ-el);let vw=(c+(vl*vu));let vy=(if vr{(vt*vw)}else{(if ((vn)!=0.0){vp}else{uC})});let vz=(vk-c);let vB=(vy-c);let vD=((gN*vz)+(gU*vB));let vY=(if ((sf[178])!=0.0){vl}else{vI});let w0=(if (jR<eJ){c}else{b});let w1=(((sf[178])!=0.0)&&((w0)!=0.0));let w3=((jR*vY)).exp();let w6=(((sf[178])!=0.0)&&(!((w0)!=0.0)));let w8=((eJ*vY)).exp();let w9=(jR-eJ);let wb=(c+(vY*w9));let we=(vX-c);let wg=((if w6{(w8*wb)}else{(if w1{w3}else{vy})})-c);let wl=(if sb[51]{b}else{(if ((sf[178])!=0.0){((gW*we)+(gY*wg))}else{b})});let wN=(j5*jZ);let wO=(c+wJ);let wP=(c+wM);let wQ=(wO/wP);let wT=((wJ-wM)-(wQ).ln());let wV=(k1+(f5*wT));let wW=(j8*wV);let wX=(jz*wW);let wZ=(a2*(bp*jz));let x2=((d+(k1*k1))).sqrt();let x4=(c+(wZ*x2));let x5=(j8*x4);let x6=(wX/x5);let x9=((c+(x6*x6))).sqrt();let xa=(wW/x9);let xb=(jb*k2);let xc=(k3*pS);let xd=(je*xc);let xe=(jh*k4);let xf=(k5*qR);let xg=(jk*xf);let xk=0.02;let xm=(xk*(c+h8));let xr=(if ((sf[180])!=0.0){f64::powf(xm,sf[182])}else{b});let xt=((iq-jJ)-xr);let xw=((d+(xt*xt))).sqrt();let xA=(if ((sf[180])!=0.0){(xr+(bp*(xt+xw)))}else{b});let xB=(-h8);let xD=f64::powf(xA,sf[183]);let xF=(if ((sf[180])!=0.0){(xB*xD)}else{b});let xH=(if (xF<N){c}else{b});let xI=(((sf[180])!=0.0)&&((xH)!=0.0));let xJ=(xF).exp();let xM=(((sf[180])!=0.0)&&(!((xH)!=0.0)));let xN=(if xM{ws}else{b});let xR=(if xM{(xN*(c+(xF-N)))}else{(if xI{xJ}else{b})});let xS=(sf[179]*xA);let xU=(if ((sf[180])!=0.0){(xR*xS)}else{b});let xV=(k7-pT);let xW=(xV-vD);let y5=(xk*(c+hd));let ya=(if ((sf[185])!=0.0){f64::powf(y5,sf[188])}else{b});let yc=((b-jO)-ya);let yf=((d+(yc*yc))).sqrt();let yj=(if ((sf[185])!=0.0){(ya+(bp*(yc+yf)))}else{b});let yk=(-hd);let ym=f64::powf(yj,sf[189]);let yo=(if ((sf[185])!=0.0){(yk*ym)}else{b});let yq=(if (yo<N){c}else{b});let yr=(((sf[185])!=0.0)&&((yq)!=0.0));let ys=(yo).exp();let yv=(((sf[185])!=0.0)&&(!((yq)!=0.0)));let yw=(if yv{ws}else{b});let yA=(if yv{(yw*(c+(yo-N)))}else{(if yr{ys}else{b})});let yB=(sf[184]*yj);let yD=(if ((sf[185])!=0.0){(yA*yB)}else{xU});let yE=(-wN);let yV=0.1;let yX=(if sb[60]{((c-(jJ/sf[193]))-yV)}else{b});let z0=((p6+(yX*yX))).sqrt();let z9=(if sb[62]{sf[191]}else{(if sb[60]{(sf[191]*(if sb[60]{(yV+(bp*(yX+z0)))}else{yX}))}else{b})});let zb=((pU/z9)-c);let zj=((vD-(if sb[53]{b}else{(if ((sf[180])!=0.0){(xU*xW)}else{b})}))-(if sb[63]{b}else{(if ((sf[192])!=0.0){(sf[190]*f64::powf(zb,sf[195]))}else{b})}));let A2=(K*xa);let E6=(DR*(sf[91]*f64::powf(f6,sf[209])));let Eq=(DR*(sf[98]*f64::powf(f6,sf[212])));let FS=((gy*(sf[63]*(DR*(sf[66]*f64::powf(f6,sf[219])))))+(gu*(gy*(((gw*(sf[68]*EV))-(gv*FJ))/FN))));let G8=((gF*(sf[69]*(DR*(sf[72]*f64::powf(f6,sf[220])))))+(gB*(gF*(((gD*(sf[74]*EV))-(gC*FZ))/G3))));let Gc=(DR*(sf[77]*f64::powf(f6,sf[221])));let Gl=(gM*(((gK*(sf[79]*EV))-(gJ*Gf))/Gj));let Gs=(DR*(sf[82]*f64::powf(f6,sf[222])));let GB=(gT*(((gR*(sf[84]*EV))-(gQ*Gv))/Gz));let GR=(sf[110]*(sf[111]*DO));let GT=(sf[112]*(sf[113]*DO));let Jb=(iQ*(((iO*J2)-(iN*J5))/J9));let Jn=(if j6{((-(if sb[14]{(sf[93]*E6)}else{(if ((sf[92])!=0.0){(sf[93]*(DR*(sf[94]*f64::powf(f6,sf[210]))))}else{b})}))/(fu*fu))}else{b});let JX=(if jx{((-(sf[129]*(DR*(sf[130]*f64::powf(f6,sf[225])))))/(iM*iM))}else{b});let We=(((pS*U5)-(p1*W6))/Wd);let Wi=(((pS*U6)-(p1*W7))/Wd);let Wm=(((pS*U7)-(p1*W8))/Wd);let Wq=(((pS*U8)-(p1*W9))/Wd);let Wu=(((pS*Tv)-(oJ*W6))/Wd);let Wx=((-(oJ*W7))/Wd);let WB=(((pS*Tw)-(oJ*W8))/Wd);let WF=(((pS*Tx)-(oJ*W9))/Wd);let Yb=(hp*qH);let Zj=(K*rd);let Zk=(rd*JY);let Zz=(if rl{((rq*(rn*(dF*Zh)))+(rn*(ro*Zh)))}else{(if rg{(ri*(jD*Zh))}else{b})});let ZA=(if rl{(rn*Zj)}else{(if rg{(ri*Zj)}else{b})});let ZB=(if rl{(rn*Zk)}else{(if rg{(ri*Zk)}else{b})});let ZI=((rz*FS)+(gz*(sf[171]*UC)));let ZJ=(gz*(sf[171]*UD));let ZK=(gz*(sf[171]*UE));let ZL=(gz*(sf[171]*UF));let a02=((rD*G8)+(gG*Zz));let a03=(gG*ZA);let a04=(gG*ZB);
        let a0p=(if sb[36]{(a02+((rB*FS)+(gz*Z9)))}else{(if sb[34]{(((rB*ZI)+(rA*Z9))+a02)}else{b})});let a0q=(if sb[36]{(gz*Za)}else{(if sb[34]{((rB*ZJ)+(rA*Za))}else{b})});let a0s=(if sb[36]{(a03+(gz*Zc))}else{(if sb[34]{(((rB*ZK)+(rA*Zc))+a03)}else{b})});let a0t=(if sb[36]{(a04+(gz*Zd))}else{(if sb[34]{(((rB*ZL)+(rA*Zd))+a04)}else{b})});let a1M=(K*sx);let a1N=(sx*JY);let a23=(if sF{((sK*(sH*(dF*a1K)))+(sH*(sI*a1K)))}else{(if sA{(sC*(jG*a1K))}else{Zz})});let a24=(if sF{(sH*a1M)}else{(if sA{(sC*a1M)}else{b})});let a25=(if sF{b}else{(if sA{b}else{ZA})});let a26=(if sF{(sH*a1N)}else{(if sA{(sC*a1N)}else{ZB})});let a2p=(if sb[41]{(((sN*FS)+(gz*a1E))+((sP*G8)+(gG*a23)))}else{b});let a2q=(if sb[41]{(gz*a1F)}else{b});let a2s=(if sb[41]{((gz*a1H)+(gG*a25))}else{b});let a2t=(if sb[41]{((gz*a1I)+(gG*a26))}else{b});let a3E=(K*tt);let a3F=(tt*JY);let a3V=(if ty{((tC*(tA*(dF*a3C)))+(tA*(ro*a3C)))}else{(if tu{(tw*(jD*a3C))}else{a23})});let a3W=(if ty{b}else{(if tu{b}else{a24})});let a3X=(if ty{(tA*a3E)}else{(if tu{(tw*a3E)}else{a25})});let a3Y=(if ty{(tA*a3F)}else{(if tu{(tw*a3F)}else{a26})});let a4f=((tI*G8)+(gG*a3V));let a4g=(gG*a3W);let a4h=(gG*a3X);let a4i=(gG*a3Y);let a4R=(if sb[46]{(sf[169]*(a4f+((tG*FS)+(gz*a3w))))}else{(if sb[45]{(sf[169]*(((tG*ZI)+(rA*a3w))+a4f))}else{(if sb[41]{b}else{(if sb[38]{(a0p-(sf[34]*(a0W-Jb)))}else{a0p})})})});let a4S=(if sb[46]{(sf[169]*(gz*a3x))}else{(if sb[45]{(sf[169]*((tG*ZJ)+(rA*a3x)))}else{(if sb[41]{b}else{(if sb[38]{(a0q-(sf[34]*a0X))}else{a0q})})})});let a4T=(if sb[46]{(sf[169]*(a4g+(gz*a3y)))}else{(if sb[45]{(sf[169]*((rA*a3y)+a4g))}else{(if sb[41]{b}else{(if sb[36]{(gz*Zb)}else{(if sb[34]{(rA*Zb)}else{b})})})})});let a4U=(if sb[46]{(sf[169]*(a4h+(gz*a3z)))}else{(if sb[45]{(sf[169]*(((tG*ZK)+(rA*a3z))+a4h))}else{(if sb[41]{b}else{(if sb[38]{(a0s-(sf[34]*a0Y))}else{a0s})})})});let a4V=(if sb[46]{(sf[169]*(a4i+(gz*a3A)))}else{(if sb[45]{(sf[169]*(((tG*ZL)+(rA*a3A))+a4i))}else{(if sb[41]{b}else{(if sb[38]{(a0t-(sf[34]*a0Z))}else{a0t})})})});let a4W=(if sb[46]{(sf[169]*(gz*a3B))}else{(if sb[45]{(sf[169]*(rA*a3B))}else{(if sb[41]{b}else{(if sb[36]{(gz*Ze)}else{(if sb[34]{(rA*Ze)}else{b})})})})});let a5z=(if sb[47]{(a4R-(sf[175]*(a5m-Jb)))}else{a4R});let a5A=(if sb[47]{(a4S-(sf[175]*a5n))}else{a4S});let a5B=(if sb[47]{(a4U-(sf[175]*a5o))}else{a4U});let a5C=(if sb[47]{(a4V-(sf[175]*a5p))}else{a4V});let a66=(K*ur);let a67=(ur*JY);let a6n=(if uw{((uA*(uy*(dF*a64)))+(uy*(sI*a64)))}else{(if us{(uu*(jG*a64))}else{a3V})});let a6o=(if uw{(uy*a66)}else{(if us{(uu*a66)}else{a3W})});let a6p=(if uw{b}else{(if us{b}else{a3X})});let a6q=(if uw{(uy*a67)}else{(if us{(uu*a67)}else{a3Y})});let a6P=(if sb[44]{(sf[176]*(((uE*FS)+(gz*a5Y))+((uG*G8)+(gG*a6n))))}else{(if sb[42]{(a2p-(sf[34]*(a2U-Jb)))}else{a2p})});let a6Q=(if sb[44]{(sf[176]*(gz*a5Z))}else{(if sb[42]{(a2q-(sf[34]*a2V))}else{a2q})});let a6R=(if sb[44]{(sf[176]*((gz*a60)+(gG*a6o)))}else{(if sb[41]{((gz*a1G)+(gG*a24))}else{b})});let a6S=(if sb[44]{(sf[176]*((gz*a61)+(gG*a6p)))}else{(if sb[42]{(a2s-(sf[34]*a2W))}else{a2s})});let a6T=(if sb[44]{(sf[176]*((gz*a62)+(gG*a6q)))}else{(if sb[42]{(a2t-(sf[34]*a2X))}else{a2t})});let a6U=(if sb[44]{(sf[176]*(gz*a63))}else{(if sb[41]{(gz*a1J)}else{b})});let a7x=(if sb[47]{(a6P-(sf[177]*(a7k-Jb)))}else{a6P});let a7y=(if sb[47]{(a6Q-(sf[177]*a7l))}else{a6Q});let a7z=(if sb[47]{(a6S-(sf[177]*a7m))}else{a6S});let a7A=(if sb[47]{(a6T-(sf[177]*a7n))}else{a6T});let a86=(vl*JY);let a87=(K*vl);let a8o=(if vr{((vw*(vt*(el*a84)))+(vt*(vu*a84)))}else{(if ((vn)!=0.0){(vp*(jJ*a84))}else{a6n})});let a8p=(if vr{(vt*a86)}else{(if ((vn)!=0.0){(vp*a86)}else{b})});let a8q=(if vr{b}else{(if ((vn)!=0.0){b}else{a6o})});let a8r=(if vr{(vt*a87)}else{(if ((vn)!=0.0){(vp*a87)}else{a6p})});let a8s=(if vr{b}else{(if ((vn)!=0.0){b}else{a6q})});let a8A=(gN*a82);let a8I=(((vz*((gM*(sf[75]*Gc))+(gI*Gl)))+(gN*a7X))+((vB*((gT*(sf[80]*Gs))+(gP*GB)))+(gU*a8o)));let a8J=((gN*a7Y)+(gU*a8p));let a8K=((gN*a7Z)+(gU*a8q));let a8L=((gN*a80)+(gU*a8r));let a8M=((gN*a81)+(gU*a8s));let a9e=(if ((sf[178])!=0.0){a84}else{a8N});let a9g=(K*vY);let a9h=(vY*JY);
        let aa7=(if sb[51]{b}else{(if ((sf[178])!=0.0){(((we*((gV*Gl)+(gM*(sf[85]*Gc))))+(gW*a98))+((wg*((gX*GB)+(gT*(sf[86]*Gs))))+(gY*(if w6{((wb*(w8*(eJ*a9e)))+(w8*(w9*a9e)))}else{(if w1{(w3*(jR*a9e))}else{a8o})}))))}else{b})});let aa8=(if sb[51]{b}else{(if ((sf[178])!=0.0){((gW*a99)+(gY*(if w6{b}else{(if w1{b}else{a8p})})))}else{b})});let aa9=(if sb[51]{b}else{(if ((sf[178])!=0.0){((gW*a9a)+(gY*(if w6{(w8*a9g)}else{(if w1{(w3*a9g)}else{a8q})})))}else{b})});let aaa=(if sb[51]{b}else{(if ((sf[178])!=0.0){((gW*a9b)+(gY*(if w6{b}else{(if w1{b}else{a8r})})))}else{b})});let aab=(if sb[51]{b}else{(if ((sf[178])!=0.0){((gW*a9c)+(gY*(if w6{b}else{(if w1{b}else{a8s})})))}else{b})});let aac=(if sb[51]{b}else{(if ((sf[178])!=0.0){((gW*a9d)+(gY*(if w6{(w8*a9h)}else{(if w1{(w3*a9h)}else{b})})))}else{b})});let abj=(jZ*(if j2{((-(if sb[13]{(sf[89]*E6)}else{(if ((sf[88])!=0.0){(sf[89]*(DR*(sf[90]*f64::powf(f6,sf[208]))))}else{b})}))/(fl*fl))}else{b}));let abk=(-j5);let abo=(wP*wP);let acd=((wV*Jn)+(j8*((wT*DQ)+(f5*((ab0-abe)-((((wP*ab0)-(wO*abe))/abo)/wQ))))));let ace=(j8*(K+(f5*((-abf)-(((-(wO*abf))/abo)/wQ)))));let acf=(j8*(JY+(f5*((ab1-abg)-((((wP*ab1)-(wO*abg))/abo)/wQ)))));let acg=(j8*(f5*(ab2-((ab2/wP)/wQ))));let ach=(j8*(f5*((ab3-abh)-((((wP*ab3)-(wO*abh))/abo)/wQ))));let aci=(j8*(f5*((ab4-abi)-((((wP*ab4)-(wO*abi))/abo)/wQ))));let acj=(j8*(f5*(ab5-((ab5/wP)/wQ))));let acv=(K*k1);let acx=(k1*JY);let acz=(hp*x2);let acN=(x5*x5);let ad1=(x6*(((x5*((wW*JX)+(jz*acd)))-(wX*((x4*Jn)+(j8*(x2*(a2*(bp*JX)))))))/acN));let ad3=(x6*(((x5*(jz*ace))-(wX*(j8*(wZ*((acv+acv)/acz)))))/acN));let ad5=(x6*(((x5*(jz*acf))-(wX*(j8*(wZ*((acx+acx)/acz)))))/acN));let ad7=(x6*((jz*acg)/x5));let ad9=(x6*((jz*ach)/x5));let adb=(x6*((jz*aci)/x5));let add=(x6*((jz*acj)/x5));let adf=(hp*x9);let adq=(x9*x9);let adr=(((x9*acd)-(wW*((ad1+ad1)/adf)))/adq);let adv=(((x9*ace)-(wW*((ad3+ad3)/adf)))/adq);let adz=(((x9*acf)-(wW*((ad5+ad5)/adf)))/adq);let adD=(((x9*acg)-(wW*((ad7+ad7)/adf)))/adq);let adH=(((x9*ach)-(wW*((ad9+ad9)/adf)))/adq);let adL=(((x9*aci)-(wW*((adb+adb)/adf)))/adq);let adP=(((x9*acj)-(wW*((add+add)/adf)))/adq);let adQ=(k2*(if j9{((-(if sb[15]{(sf[96]*Eq)}else{(if ((sf[95])!=0.0){(sf[96]*(DR*(sf[97]*f64::powf(f6,sf[211]))))}else{b})}))/(fF*fF))}else{b}));let adR=(-jb);let ae0=((xc*(if jc{((-(if sb[16]{(sf[100]*Eq)}else{(if ((sf[99])!=0.0){(sf[100]*(DR*(sf[101]*f64::powf(f6,sf[213]))))}else{b})}))/(fO*fO))}else{b}))+(je*(k3*W6)));let ae1=(je*(k3*W7));let ae2=(je*pS);let ae3=(je*((-pS)+(k3*W8)));let ae4=(je*(k3*W9));let ae5=(k4*(if jf{((-(sf[102]*(DR*(sf[103]*f64::powf(f6,sf[214])))))/(fS*fS))}else{b}));let ae6=(-jh);let aeh=((xf*(if ji{((-(if sb[17]{(sf[105]*E6)}else{(if ((sf[104])!=0.0){(sf[105]*(DR*(sf[106]*f64::powf(f6,sf[215]))))}else{b})}))/(g1*g1))}else{b}))+(jk*(k5*(if sb[31]{b}else{(if qM{b}else{(if qG{(bp*((if ((sf[166])!=0.0){(br_*(if ((sf[166])!=0.0){(X*XH)}else{b}))}else{VC})/Yb))}else{b})})}))));let aei=(jk*(-qR));let aej=(jk*(k5*(if sb[31]{b}else{(if qM{b}else{(if qG{(bp*((if ((sf[166])!=0.0){(br_*(if ((sf[166])!=0.0){(X*XI)}else{b}))}else{VD})/Yb))}else{b})})})));let aek=(jk*(k5*(if sb[31]{b}else{(if qM{b}else{(if qG{(bp*((if ((sf[166])!=0.0){(br_*(if ((sf[166])!=0.0){(X*XJ)}else{b}))}else{b})/Yb))}else{b})})})));let ael=(jk*(k5*(if sb[31]{b}else{(if qM{b}else{(if qG{(bp*((if ((sf[166])!=0.0){(br_*(if ((sf[166])!=0.0){(X*XK)}else{b}))}else{VE})/Yb))}else{b})})})));let aem=(jk*(k5*(if sb[31]{b}else{(if qM{b}else{(if qG{(bp*((if ((sf[166])!=0.0){(br_*(if ((sf[166])!=0.0){(X*XL)}else{b}))}else{VF})/Yb))}else{b})})})));let aen=(jk*(qR+(k5*(if sb[31]{b}else{(if qM{b}else{(if qG{(bp*((if ((sf[166])!=0.0){(br_*(if ((sf[166])!=0.0){(X*XM)}else{b}))}else{b})/Yb))}else{b})})}))));let aet=(if ((sf[180])!=0.0){((xk*GR)*(sf[182]*f64::powf(xm,sf[231])))}else{b});let aeu=(Ir-aet);let aev=(xt*aeu);let aex=(K*xt);let aez=(xt*JY);let aeB=(hp*xw);let aeM=(if ((sf[180])!=0.0){(aet+(bp*(aeu+((aev+aev)/aeB))))}else{b});let aeN=(if ((sf[180])!=0.0){(bp*(K+((aex+aex)/aeB)))}else{b});let aeO=(if ((sf[180])!=0.0){(bp*(JY+((aez+aez)/aeB)))}else{b});
        let aeS=(sf[183]*f64::powf(xA,sf[232]));let af1=(if ((sf[180])!=0.0){((xD*(-GR))+(xB*(aeM*aeS)))}else{b});let af2=(if ((sf[180])!=0.0){(xB*(aeN*aeS))}else{b});let af3=(if ((sf[180])!=0.0){(xB*(aeO*aeS))}else{b});let afs=(if ((sf[180])!=0.0){((xS*(if xM{(xN*af1)}else{(if xI{(xJ*af1)}else{b})}))+(xR*(sf[179]*aeM)))}else{b});let aft=(if ((sf[180])!=0.0){((xS*(if xM{(xN*af2)}else{(if xI{(xJ*af2)}else{b})}))+(xR*(sf[179]*aeN)))}else{b});let afu=(if ((sf[180])!=0.0){((xS*(if xM{(xN*af3)}else{(if xI{(xJ*af3)}else{b})}))+(xR*(sf[179]*aeO)))}else{b});let afv=(-We);let afw=(-Wi);let afx=(-Wm);let afy=(-Wq);let aga=(if ((sf[185])!=0.0){((xk*GT)*(sf[188]*f64::powf(y5,sf[233])))}else{b});let agb=(-aga);let agc=(yc*agb);let age=(K*yc);let agg=(yc*JY);let agi=(hp*yf);let agt=(if ((sf[185])!=0.0){(aga+(bp*(agb+((agc+agc)/agi))))}else{b});let agu=(if ((sf[185])!=0.0){(bp*(K+((age+age)/agi)))}else{b});let agv=(if ((sf[185])!=0.0){(bp*(JY+((agg+agg)/agi)))}else{b});let agz=(sf[189]*f64::powf(yj,sf[234]));let agI=(if ((sf[185])!=0.0){((ym*(-GT))+(yk*(agt*agz)))}else{b});let agJ=(if ((sf[185])!=0.0){(yk*(agu*agz))}else{b});let agK=(if ((sf[185])!=0.0){(yk*(agv*agz))}else{b});let ahF=(if sb[60]{(-(JY/sf[193]))}else{b});let ahG=(if sb[60]{(-(K/sf[193]))}else{b});let ahH=(yX*ahF);let ahJ=(yX*ahG);let ahL=(hp*z0);let ai4=(z9*z9);let aid=(sf[195]*f64::powf(zb,sf[235]));let aiw=(a8K-(if sb[53]{b}else{(if ((sf[180])!=0.0){(xU*(-a8K))}else{b})}));let aiz=(a8A-(if sb[53]{b}else{(if ((sf[180])!=0.0){(xU*(-a8A))}else{b})}));let aiA=(-(if sb[53]{b}else{(if ((sf[180])!=0.0){xU}else{b})}));let aiB=((a8I-(if sb[53]{b}else{(if ((sf[180])!=0.0){((xW*afs)+(xU*(afv-a8I)))}else{b})}))-(if sb[63]{b}else{(if ((sf[192])!=0.0){(sf[190]*((Wu/z9)*aid))}else{b})}));let aiC=((a8J-(if sb[53]{b}else{(if ((sf[180])!=0.0){((xW*aft)+(xU*(afw-a8J)))}else{b})}))-(if sb[63]{b}else{(if ((sf[192])!=0.0){(sf[190]*((((z9*Wx)-(pU*(if sb[62]{b}else{(if sb[60]{(sf[191]*(if sb[60]{(bp*(ahF+((ahH+ahH)/ahL)))}else{ahF}))}else{b})})))/ai4)*aid))}else{b})}));let aiD=((a8L-(if sb[53]{b}else{(if ((sf[180])!=0.0){((xW*afu)+(xU*(afx-a8L)))}else{b})}))-(if sb[63]{b}else{(if ((sf[192])!=0.0){(sf[190]*((((z9*WB)-(pU*(if sb[62]{b}else{(if sb[60]{(sf[191]*(if sb[60]{(bp*(ahG+((ahJ+ahJ)/ahL)))}else{ahG}))}else{b})})))/ai4)*aid))}else{b})}));let aiE=((a8M-(if sb[53]{b}else{(if ((sf[180])!=0.0){(xU*(afy-a8M))}else{b})}))-(if sb[63]{b}else{(if ((sf[192])!=0.0){(sf[190]*((WF/z9)*aid))}else{b})}));let ala=(l*K);let alb=(l*JY);

        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(8),
            multiplicity * ((K*(ue+(l*jD)))),
            [3, 5, 6, 7, 8, 9],
            [(K*a5z), (K*a5A), (K*a4T), (K*(a5B+ala)), (K*(a5C+alb)), (K*a4W)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(8),
            multiplicity * ((K*(v6+(l*jG)))),
            [3, 5, 6, 7, 8, 9],
            [(K*a7x), (K*a7y), (K*(a6R+ala)), (K*a7z), (K*(a7A+alb)), (K*a6U)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(5),
            Some(8),
            multiplicity * ((K*k7)),
            11,
            multiplicity * (K),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * ((K*pT)),
            [3, 5, 7, 8],
            [(K*We), (K*Wi), (K*Wm), (K*Wq)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(5),
            multiplicity * ((K*(zj+(l*jJ)))),
            [3, 5, 6, 7, 8, 9, 11],
            [(K*aiB), (K*(aiC+alb)), (K*aiw), (K*(aiD+ala)), (K*aiE), (K*aiz), (K*aiA)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(4),
            multiplicity * ((K*((if sb[55]{b}else{(if ((sf[185])!=0.0){(yD*yE)}else{b})})+(l*jO)))),
            [0, 3, 4, 5, 6, 7],
            [(K*(if sb[55]{b}else{(if ((sf[185])!=0.0){(yD*abk)}else{b})})), (K*(if sb[55]{b}else{(if ((sf[185])!=0.0){((yE*(if ((sf[185])!=0.0){((yB*(if yv{(yw*agI)}else{(if yr{(ys*agI)}else{b})}))+(yA*(sf[184]*agt)))}else{afs}))+(yD*(-abj)))}else{b})})), (K*((if sb[55]{b}else{(if ((sf[185])!=0.0){((yE*(if ((sf[185])!=0.0){((yB*(if yv{(yw*agJ)}else{(if yr{(ys*agJ)}else{b})}))+(yA*(sf[184]*agu)))}else{b}))+(j5*yD))}else{b})})+alb)), (K*(if sb[55]{b}else{(if ((sf[185])!=0.0){(yE*(if ((sf[185])!=0.0){b}else{aft}))}else{b})})), (K*((if sb[55]{b}else{(if ((sf[185])!=0.0){(yE*(if ((sf[185])!=0.0){((yB*(if yv{(yw*agK)}else{(if yr{(ys*agK)}else{b})}))+(yA*(sf[184]*agv)))}else{b}))}else{b})})+ala)), (K*(if sb[55]{b}else{(if ((sf[185])!=0.0){(yE*(if ((sf[185])!=0.0){b}else{afu}))}else{b})}))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(9),
            multiplicity * ((K*(wl+(l*jR)))),
            [3, 5, 6, 7, 8, 9],
            [(K*aa7), (K*aa8), (K*(aa9+ala)), (K*aaa), (K*aab), (K*(aac+alb))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(4),
            multiplicity * (wN),
            0,
            multiplicity * (j5),
            3,
            multiplicity * (abj),
            4,
            multiplicity * (abk),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(4),
            Some(5),
            multiplicity * (A2),
            [3, 4, 5, 6, 7, 8, 9],
            [(K*adr), (K*adv), (K*adz), (K*adD), (K*adH), (K*adL), (K*adP)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(6),
            multiplicity * (xb),
            1,
            multiplicity * (jb),
            3,
            multiplicity * (adQ),
            6,
            multiplicity * (adR),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (xd),
            [3, 5, 6, 7, 8],
            [ae0, ae1, ae2, ae3, ae4],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(8),
            multiplicity * (xe),
            2,
            multiplicity * (jh),
            3,
            multiplicity * (ae5),
            8,
            multiplicity * (ae6),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(9),
            Some(4),
            multiplicity * (xg),
            [3, 4, 5, 6, 7, 8, 9],
            [aeh, aei, aej, aek, ael, aem, aen],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            None,
            multiplicity * ((k7-pU)),
            [3, 5, 7, 8, 11],
            [(-Wu), (-Wx), (-WB), (-WF), c],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(11),
            None,
            multiplicity * ((k7-k6)),
            10,
            multiplicity * (A),
            11,
            multiplicity * (c),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((eK*jn)),
            3,
            multiplicity * ((jn+(eK*(if jl{((-(sf[107]*(sf[108]*DO)))/(g6*g6))}else{b})))),
        );
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            None,
            multiplicity * (((((((((((((jD*ue)+(jJ*zj))+(jW*xV))+(jG*v6))+(jR*wl))+(jZ*wN))+(k1*xa))+(k2*xb))+(k3*xd))+(k4*xe))+(k5*xg))*sf[197])),
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11],
            &[(sf[197]*(wN+wN)), (sf[197]*(xb+xb)), (sf[197]*(xe+xe)), (sf[197]*(((((((((((jD*a5z)+(jJ*aiB))+(jW*afv))+(jG*a7x))+(jR*aa7))+(jZ*abj))+(k1*adr))+(k2*adQ))+(k3*ae0))+(k4*ae5))+(k5*aeh))), (sf[197]*(((yE+(jZ*abk))+(A2+(k1*adv)))+((-xg)+(k5*aei)))), (sf[197]*((((((((jD*a5A)+((zj*JY)+(jJ*aiC)))+((K*xV)+(jW*afw)))+(jG*a7y))+(jR*aa8))+((xa*JY)+(k1*adz)))+(k3*ae1))+(k5*aej))), (sf[197]*((((((((jD*a4T)+(jJ*aiw))+((K*v6)+(jG*a6R)))+((K*wl)+(jR*aa9)))+(k1*adD))+((-xb)+(k2*adR)))+(xd+(k3*ae2)))+(k5*aek))), (sf[197]*(((((((((K*ue)+(jD*a5B))+((K*zj)+(jJ*aiD)))+(jW*afx))+(jG*a7z))+(jR*aaa))+(k1*adH))+((-xd)+(k3*ae3)))+(k5*ael))), (sf[197]*((((((((((ue*JY)+(jD*a5C))+(jJ*aiE))+((xV*JY)+(jW*afy)))+((v6*JY)+(jG*a7A)))+(jR*aab))+(k1*adL))+(k3*ae4))+((-xe)+(k4*ae6)))+(k5*aem))), (sf[197]*((((((jD*a4W)+(jJ*aiz))+(jG*a6U))+((wl*JY)+(jR*aac)))+(k1*adP))+(xg+(k5*aen)))), (sf[197]*(jW+(jJ*aiA)))],
            &[],
            &[],
            multiplicity,
        );
        let DF_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, DF);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(8),
            multiplicity * (DF_ddt),
            [3, 5, 6, 7, 8, 9],
            [((avn) * ddt_scale), ((avo) * ddt_scale), ((avp) * ddt_scale), ((avq) * ddt_scale), ((avr) * ddt_scale), ((avs) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let DG_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, DG);
        stamper.stamp_current_node3_local(
            Some(6),
            Some(8),
            multiplicity * (DG_ddt),
            3,
            multiplicity * (((avt) * ddt_scale)),
            6,
            multiplicity * (((avu) * ddt_scale)),
            8,
            multiplicity * (((avv) * ddt_scale)),
        );
        let DH_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, DH);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (DH_ddt),
            [3, 5, 6, 7, 8, 9],
            [((avw) * ddt_scale), ((avx) * ddt_scale), ((avy) * ddt_scale), ((avz) * ddt_scale), ((avA) * ddt_scale), ((avB) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let DI_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, DI);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(4),
            multiplicity * (DI_ddt),
            [3, 4, 5, 7, 8],
            [((avC) * ddt_scale), ((avD) * ddt_scale), ((avE) * ddt_scale), ((avF) * ddt_scale), ((avG) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let DJ_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, DJ);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(9),
            multiplicity * (DJ_ddt),
            [3, 5, 6, 7, 8, 9],
            [((avH) * ddt_scale), ((avI) * ddt_scale), ((avJ) * ddt_scale), ((avK) * ddt_scale), ((avL) * ddt_scale), ((avM) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let Dv_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, Dv);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (Dv_ddt),
            1,
            multiplicity * (((sf[203]) * ddt_scale)),
            2,
            multiplicity * (((sf[236]) * ddt_scale)),
        );
        let Dx_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, Dx);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (Dx_ddt),
            0,
            multiplicity * (((sf[237]) * ddt_scale)),
            1,
            multiplicity * (((sf[204]) * ddt_scale)),
        );
        let DB_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, DB);
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (DB_ddt),
            10,
            multiplicity * (((sf[206]) * ddt_scale)),
        );
        let DE_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, DE);
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (DE_ddt),
            11,
            multiplicity * (((sf[238]) * ddt_scale)),
        );
        let Dz_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, Dz);
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (Dz_ddt),
            3,
            multiplicity * (((sf[205]) * ddt_scale)),
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
            a, b, c, s, A, K, N, X,
            aM, bb, bp, br_, eK, f5, f6, f7,
            g9, gw, gD, gK, gR, hp, iq, iN,
            iO, jA, jB, jD, jE, jG, jH, jJ,
            jK, jP, jR, jS, jT, jX, k6, k7,
            oJ, p1, p6, p9, pe, pF, pS, qy,
            rb, rd, s7, sw, sx, ta, ts, tt,
            u9, uq, ur, v1, vk, vl, vI, vX,
            ws, wJ, wM, Dv, Dx, Dz, DB, DE,
            DF, DG, DH, DI, DJ, DO, DQ, DR,
            EV, FJ, FN, FZ, G3, Gf, Gj, Gv,
            Gz, Ir, J2, J5, J9, JY, Tv, Tw,
            Tx, U5, U6, U7, U8, UC, UD, UE,
            UF, VC, VD, VE, VF, W6, W7, W8,
            W9, Wd, XH, XI, XJ, XK, XL, XM,
            Z9, Za, Zb, Zc, Zd, Ze, Zh, a0W,
            a0X, a0Y, a0Z, a1E, a1F, a1G, a1H, a1I,
            a1J, a1K, a2U, a2V, a2W, a2X, a3w, a3x,
            a3y, a3z, a3A, a3B, a3C, a5m, a5n, a5o,
            a5p, a5Y, a5Z, a60, a61, a62, a63, a64,
            a7k, a7l, a7m, a7n, a7X, a7Y, a7Z, a80,
            a81, a82, a84, a8N, a98, a99, a9a, a9b,
            a9c, a9d, ab0, ab1, ab2, ab3, ab4, ab5,
            abe, abf, abg, abh, abi, avn, avo, avp,
            avq, avr, avs, avt, avu, avv, avw, avx,
            avy, avz, avA, avB, avC, avD, avE, avF,
            avG, avH, avI, avJ, avK, avL, avM,
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
            &[avn, avo, avp, avq, avr, avs],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3_local(
            Some(6),
            Some(8),
            3,
            multiplicity * (avt),
            6,
            multiplicity * (avu),
            8,
            multiplicity * (avv),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[3, 5, 6, 7, 8, 9],
            &[avw, avx, avy, avz, avA, avB],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(4),
            &[3, 4, 5, 7, 8],
            &[avC, avD, avE, avF, avG],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(9),
            &[3, 5, 6, 7, 8, 9],
            &[avH, avI, avJ, avK, avL, avM],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(2),
            1,
            multiplicity * (sf[203]),
            2,
            multiplicity * (sf[236]),
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(0),
            0,
            multiplicity * (sf[237]),
            1,
            multiplicity * (sf[204]),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(10),
            None,
            10,
            multiplicity * (sf[206]),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(11),
            None,
            11,
            multiplicity * (sf[238]),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(3),
            None,
            3,
            multiplicity * (sf[205]),
        );
    }
}
