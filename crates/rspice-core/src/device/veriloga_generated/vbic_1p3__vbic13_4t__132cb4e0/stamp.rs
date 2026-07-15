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
    g: f64, i: f64, C: f64, cj: f64, fO: f64, g9: f64,
    ga: f64, gb: f64, hc: f64, hh: f64, hv: f64, iG: f64,
    jH: f64, ll: f64, lp: f64, lq: f64, ls: f64, lt: f64,
    lv: f64, lw: f64, ly: f64, lz: f64, lE: f64, lG: f64,
    lH: f64, lI: f64, lM: f64, lV: f64, lX: f64, m2: f64,
    m3: f64, qe: f64, qi: f64, qu: f64, qy: f64, qH: f64,
    qO: f64, qR: f64, qW: f64, r1: f64, rg: f64, ro: f64,
    rw: f64, rB: f64, rF: f64, rX: f64, AH: bool, AJ: f64,
    AO: f64, B1: f64, B4: f64, Jp: f64, Jr: f64, Jz: f64,
    JF: f64, JK: f64, JN: f64, JV: f64, JX: f64, JZ: f64,
    K1: f64, K4: f64, K6: f64, K8: f64, Ka: f64, Kf: f64,
    Kh: f64, Ki: f64, Lr: f64, LC: f64, M8: f64, Pt: f64,
    RK: f64, a0Z: f64, a11: f64, a12: f64, a13: f64, a14: f64,
    a15: f64, a1r: f64, a1t: f64, a1u: f64, a1v: f64, a1w: f64,
    a1x: f64, a1L: f64, a2k: f64, a2l: f64, a2m: f64, a2n: f64,
    a2B: f64, a2C: f64, a2D: f64, a2E: f64, a36: f64, a37: f64,
    a38: f64, a39: f64, a3A: f64, a3B: f64, a3C: f64, a3D: f64,
    a4f: f64, a4h: f64, a4i: f64, a4j: f64, a4k: f64, a4l: f64,
    a4I: f64, a4J: f64, a4K: f64, ana: f64, anb: f64, anc: f64,
    and: f64, ane: f64, anf: f64, anz: f64, anA: f64, anB: f64,
    anI: f64, anJ: f64, anK: f64, aJi: f64, aJj: f64, aJk: f64,
    aJl: f64, aJm: f64, aJn: f64, aJV: f64, aJW: f64, aJX: f64,
    aKB: f64, aKC: f64, aKD: f64, aKK: f64, aKL: f64, aKM: f64,
    aKV: f64, aKW: f64, aKX: f64, aLm: f64, aLn: f64, aLo: f64,
    aLs: f64, aLt: f64, aLu: f64, aLA: f64, aLB: f64, aLC: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let g=0.0;let i=1.0;let C=0.5;let bk=273.15;let bM=1.380662e-23;let bO=1.602189e-19;let cj=4.0;let fO=ctx.node_voltage(n[4]);let fQ=((sf[324]+fO)-bk);let fS=(if (fQ<sf[81]){i}else{g});let fV=(((fQ-sf[80])-i)).exp();let fX=(if ((fS)!=0.0){(sf[80]+fV)}else{fQ});let g1=((((if (fX>sf[83]){i}else{g}))!=0.0)&&(!((fS)!=0.0)));let g4=(((sf[82]-fX)-i)).exp();let g7=(bk+(if g1{(sf[82]-g4)}else{fX}));let g9=((bM*g7)/bO);let ga=(g7/sf[78]);let gb=(g7-sf[78]);let ge=(sf[53]*f64::powf(ga,sf[139]));let hb=(sf[84]*f64::powf(ga,sf[89]));let hc=(i-ga);let hd=(sf[91]*hc);let he=(sf[88]*g9);let hg=((hd/he)).exp();let hh=(hb*hg);let hj=(sf[97]*f64::powf(ga,sf[100]));let hk=(sf[102]*hc);let hl=(sf[99]*g9);let hn=((hk/hl)).exp();let ho=(hj*hn);let hq=(sf[21]*f64::powf(ga,sf[106]));let hr=(sf[108]*hc);let hs=(sf[105]*g9);let hu=((hr/hs)).exp();let hv=(hq*hu);let ii=(i+(gb*sf[158]));let ij=(sf[88]*ii);let ik=(sf[99]*ii);let iG=2.0;let iI=(iG*(g9/ga));let iL=(ga*sf[167]);let iN=((iL/g9)).exp();let iO=-0.5;let iQ=(ga*sf[168]);let iS=((iQ/g9)).exp();let iT=(iN-iS);let iU=(iT).ln();let iV=(iI*iU);let iX=3.0;let iY=(g9*iX);let iZ=(ga).ln();let j0=(iY*iZ);let j2=(ga-i);let j4=(((ga*iV)-j0)-(sf[113]*j2));let j5=(g9*iG);let j6=(-j4);let j8=((j6/g9)).exp();let jb=((i+(cj*j8))).sqrt();let jd=(C*(i+jb));let je=(jd).ln();let jg=(j4+(j5*je));let jj=(ga*sf[170]);let jl=((jj/g9)).exp();let jn=(ga*sf[171]);let jp=((jn/g9)).exp();let jq=(jl-jp);let jr=(jq).ln();let js=(iI*jr);let jw=(((ga*js)-j0)-(sf[124]*j2));let jx=(-jw);let jz=((jx/g9)).exp();let jC=((i+(cj*jz))).sqrt();let jE=(C*(i+jC));let jF=(jE).ln();let jH=(jw+(j5*jF));let jK=(ga*sf[173]);let jM=((jK/g9)).exp();let jO=(ga*sf[174]);let jQ=((jO/g9)).exp();let jR=(jM-jQ);let jS=(jR).ln();let jT=(iI*jS);let jX=(((ga*jT)-j0)-(sf[133]*j2));let jY=(-jX);let k0=((jY/g9)).exp();let k3=((i+(cj*k0))).sqrt();let k5=(C*(i+k3));let k6=(k5).ln();let k8=(jX+(j5*k6));let ka=(sf[166]/jg);let kd=(sf[175]*f64::powf(ka,sf[176]));let kf=(sf[169]/jH);let kh=f64::powf(kf,sf[178]);let ki=(sf[177]*kh);let kk=(kh*sf[179]);let kl=(sf[172]/k8);let ko=(sf[45]*f64::powf(kl,sf[180]));let kr=(sf[181]*f64::powf(ga,sf[87]));let kt=((hd/g9)).exp();let ku=(kr*kt);let kH=(sf[184]*(i+(gb*sf[185])));let kM=(sf[186]*(i+(gb*sf[187])));let ld=(kH>g);let lf=(if ld{(i/kH)}else{g});let lg=(kM>g);let li=(if lg{(i/kM)}else{g});let lj=(ge>g);let ll=(if lj{(i/ge)}else{g});let lp=ctx.node_voltage(n[8]);let lq=ctx.node_voltage(n[9]);let ls=(sf[60]*(lp-lq));let lt=ctx.node_voltage(n[7]);let lv=(sf[60]*(lt-lq));let lw=ctx.node_voltage(n[6]);let ly=(sf[60]*(lp-lw));let lz=ctx.node_voltage(n[5]);let lB=(sf[60]*(lp-lz));let lE=ctx.node_voltage(n[10]);let lG=(sf[60]*(lt-lE));let lH=ctx.node_voltage(n[1]);let lI=ctx.node_voltage(n[2]);let lM=ctx.node_voltage(n[0]);let lV=ctx.node_voltage(n[11]);let lX=(sf[60]*(lV-lE));let m2=ctx.node_voltage(n[12]);let m3=ctx.node_voltage(n[13]);let m4=(-jg);let m6=(m4*sf[188]);let m7=(ls+m6);let m8=(if ((sf[13])!=0.0){m7}else{g});let ma=(if (m8>g){i}else{g});let mb=(((sf[13])!=0.0)&&((ma)!=0.0));let mf=(if mb{sf[191]}else{g});let mh=(i-(sf[189]*mf));let mn=(m8*sf[193]);let mo=(jg*sf[189]);let mq=(i+(mn/mo));let mv=(((sf[13])!=0.0)&&(!((ma)!=0.0)));let mx=(i-(ls/jg));let mz=(i-f64::powf(mx,sf[192]));let mC=(if mv{((jg*mz)/sf[192])}else{(if mb{((jg*mh)/sf[192])}else{g})});let mL=(((m6*m6)+sf[195])).sqrt();let mP=(if sb[47]{(iO*(m6+(if sb[47]{mL}else{g})))}else{g});let mR=(i-(mP/jg));let mS=f64::powf(mR,sf[192]);let mV=(if sb[47]{((m4*mS)/sf[192])}else{g});let mW=(if sb[47]{m7}else{g});let mZ=((sf[195]+(mW*mW))).sqrt();let n4=(if sb[47]{((C*(mW-(if sb[47]{mZ}else{g})))-m6)}else{g});let n6=(i-(n4/jg));let n7=f64::powf(n6,sf[192]);let nc=(mP+(ls-n4));let nd=(sf[191]*nc);let ne=(sf[193]*nc);let ng=(i+(ne/mo));let nk=(if sb[47]{(((if sb[47]{((m4*n7)/sf[192])}else{mC})+(nd*ng))-mV)}else{(if ((sf[13])!=0.0){(mC+(if mv{g}else{(if mb{(mf*(m8*mq))}else{g})}))}else{g})});let nl=(-jH);let nm=(sf[188]*nl);let nn=(ly+nm);let no=(if ((sf[15])!=0.0){nn}else{g});let nq=(if (no>g){i}else{g});let nr=(((sf[15])!=0.0)&&((nq)!=0.0));
        let nu=(if nr{sf[197]}else{g});let nx=(i-(sf[189]*(sf[189]*nu)));let nD=(no*sf[199]);let nF=(sf[189]+(nD/jH));let nM=(if (sb[3]&&(ly<sf[200])){i}else{g});let nO=(((sf[15])!=0.0)&&(!((nq)!=0.0)));let nP=(((nM)!=0.0)&&nO);let nR=(i+(sf[16]/jH));let nS=f64::powf(nR,sf[198]);let nU=(sf[198]*(sf[16]+ly));let nV=(sf[16]+jH);let nX=(i-(nU/nV));let nZ=(i-(nS*nX));let o4=(nO&&(!((nM)!=0.0)));let o6=(i-(ly/jH));let o8=(i-f64::powf(o6,sf[198]));let ob=(if o4{((jH*o8)/sf[198])}else{(if nP{((jH*nZ)/sf[198])}else{(if nr{((jH*nx)/sf[198])}else{g})})});let oh=(sf[16]+nm);let oi=(sf[16]-nm);let ok=(if sb[49]{(oh/oi)}else{g});let ol=(iG*ok);let om=(ok-i);let or=(((om*om)+sf[202])).sqrt();let os=(i+ok);let ox=(((os*os)+sf[204])).sqrt();let oy=(or+ox);let oA=(if sb[49]{(ol/oy)}else{g});let oF=(if sb[49]{(C*(((oi*oA)-sf[16])-nm))}else{g});let oH=(i-(oF/jH));let oJ=(i-f64::powf(oH,sf[198]));let oM=(if sb[49]{((jH*oJ)/sf[198])}else{g});let oP=(nm+(sf[16]+(iG*ly)));let oR=(if sb[49]{(oP/oi)}else{g});let oS=(iG*oR);let oT=(oR-i);let oW=((sf[202]+(oT*oT))).sqrt();let oX=(i+oR);let p0=((sf[204]+(oX*oX))).sqrt();let p1=(oW+p0);let p3=(if sb[49]{(oS/p1)}else{g});let p8=(if sb[49]{(C*(((oi*p3)-sf[16])-nm))}else{g});let pa=(i-(p8/jH));let pc=(i-f64::powf(pa,sf[198]));let pf=(if sb[49]{((jH*pc)/sf[198])}else{ob});let pi=(if sb[49]{(C*(i+p3))}else{g});let pl=(if sb[49]{f64::powf(nR,sf[205])}else{g});let pn=(i+(nm/jH));let pp=(if sb[49]{f64::powf(pn,sf[205])}else{g});let pq=(i-pi);let pu=(if sb[49]{((pl*pq)+(pi*pp))}else{g});let pw=(oF+(ly-p8));let pG=((sf[202]+(nm*nm))).sqrt();let pK=(if sb[51]{(iO*(nm+(if sb[51]{pG}else{g})))}else{oF});let pM=(i-(pK/jH));let pN=f64::powf(pM,sf[198]);let pQ=(if sb[51]{((nl*pN)/sf[198])}else{g});let pR=(if sb[51]{nn}else{g});let pU=((sf[202]+(pR*pR))).sqrt();let pZ=(if sb[51]{((C*(pR-(if sb[51]{pU}else{g})))-nm)}else{p8});let q1=(i-(pZ/jH));let q2=f64::powf(q1,sf[198]);let qc=(if sb[51]{(((if sb[51]{((nl*q2)/sf[198])}else{pf})+(sf[206]*(pK+(ly-pZ))))-pQ)}else{(if sb[49]{((pf+(if sb[49]{(pu*pw)}else{g}))-oM)}else{(if ((sf[15])!=0.0){(ob+(if nO{g}else{(if nr{(nu*(no*nF))}else{g})}))}else{g})})});let qd=(g9*ij);let qe=(i/qd);let qi=((ls*qe)).exp();let qt=(g9*ik);let qu=(i/qt);let qy=((ly*qu)).exp();let qH=(hh*ho);let qO=0.0001;let qP=(((i+(li*nk))+(lf*qc))-qO);let qR=1e-8;let qT=(((qP*qP)+qR)).sqrt();let qW=(qO+(C*(qP+qT)));let r1=f64::powf(qW,sf[207]);let rg=(C*(qW+sf[208]));let ro=(C*qW);let rw=(ro*sf[209]);let rB=(if ((sf[22])!=0.0){(i/hs)}else{qu});let rF=((lG*rB)).exp();let rX=((ly*rB)).exp();let AG=(ly/g9);let AH=(AG<sf[62]);let AJ=(AG).exp();let AK=(!(((if AH{i}else{g}))!=0.0));let AO=(sf[215]*(i+(AG-sf[62])));let AP=(if AK{AO}else{AJ});let AQ=(lB/g9);let AT=(AQ).exp();let AU=(!(((if (AQ<sf[62]){i}else{g}))!=0.0));let AY=(if AU{(sf[215]*(i+(AQ-sf[62])))}else{AT});let B1=((i+(ku*AP))).sqrt();let B4=((i+(ku*AY))).sqrt();let F6=(-k8);let F8=(if ((sf[46])!=0.0){(sf[188]*F6)}else{g});let Fa=(lX+F8);let Fb=(if sb[80]{Fa}else{g});let Fd=(if (Fb>g){i}else{g});let Fe=(sb[80]&&((Fd)!=0.0));let Fh=(if Fe{sf[227]}else{g});let Fj=(i-(sf[189]*Fh));let Fp=(Fb*sf[229]);let Fq=(k8*sf[189]);let Fs=(i+(Fp/Fq));let Fx=(sb[80]&&(!((Fd)!=0.0)));let Fz=(i-(lX/k8));let FB=(i-f64::powf(Fz,sf[228]));let FE=(if Fx{((k8*FB)/sf[228])}else{(if Fe{((k8*Fj)/sf[228])}else{g})});let FO=(((F8*F8)+sf[231])).sqrt();let FS=(if sb[82]{(iO*(F8+(if sb[82]{FO}else{g})))}else{g});let FU=(i-(FS/k8));let FV=f64::powf(FU,sf[228]);let FZ=(if sb[82]{Fa}else{g});let G2=((sf[231]+(FZ*FZ))).sqrt();let G7=(if sb[82]{((C*(FZ-(if sb[82]{G2}else{g})))-F8)}else{g});let G9=(i-(G7/k8));let Ga=f64::powf(G9,sf[228]);let Gf=(FS+(lX-G7));let Gg=(sf[227]*Gf);let Gh=(sf[229]*Gf);let Gj=(i+(Gh/Fq));let Gp=(if sb[83]{g}else{(if sb[82]{(((if sb[82]{((F6*Ga)/sf[228])}else{FE})+(Gg*Gj))-(if sb[82]{((F6*FV)/sf[228])}else{g}))}else{(if sb[80]{(FE+(if Fx{g}else{(if Fe{(Fh*(Fb*Fs))}else{g})}))}else{g})})});let Gq=(lv+m6);let Gr=(if ((sf[13])!=0.0){Gq}else{g});let Gt=(if (Gr>g){i}else{g});let Gu=(((sf[13])!=0.0)&&((Gt)!=0.0));let Gv=(if Gu{sf[191]}else{g});let Gx=(i-(sf[189]*Gv));
        let GB=(sf[193]*Gr);let GD=(i+(GB/mo));let GI=(((sf[13])!=0.0)&&(!((Gt)!=0.0)));let GK=(i-(lv/jg));let GM=(i-f64::powf(GK,sf[192]));let GP=(if GI{((jg*GM)/sf[192])}else{(if Gu{((jg*Gx)/sf[192])}else{g})});let GT=(if sb[47]{Gq}else{g});let GW=((sf[195]+(GT*GT))).sqrt();let H1=(if sb[47]{((C*(GT-(if sb[47]{GW}else{g})))-m6)}else{g});let H3=(i-(H1/jg));let H4=f64::powf(H3,sf[192]);let H9=(mP+(lv-H1));let Ha=(sf[191]*H9);let Hb=(sf[193]*H9);let Hd=(i+(Hb/mo));let Hh=(if sb[47]{(((if sb[47]{((m4*H4)/sf[192])}else{GP})+(Ha*Hd))-mV)}else{(if ((sf[13])!=0.0){(GP+(if GI{g}else{(if Gu{(Gv*(Gr*GD))}else{g})}))}else{g})});let Hi=(lG+nm);let Hj=(if ((sf[15])!=0.0){Hi}else{g});let Hl=(if (Hj>g){i}else{g});let Hm=(((sf[15])!=0.0)&&((Hl)!=0.0));let Hn=(if Hm{sf[197]}else{g});let Hq=(i-(sf[189]*(sf[189]*Hn)));let Hu=(sf[199]*Hj);let Hw=(sf[189]+(Hu/jH));let HC=(if (sb[3]&&(lG<sf[200])){i}else{g});let HE=(((sf[15])!=0.0)&&(!((Hl)!=0.0)));let HF=(((HC)!=0.0)&&HE);let HH=(sf[198]*(sf[16]+lG));let HJ=(i-(HH/nV));let HL=(i-(nS*HJ));let HQ=(HE&&(!((HC)!=0.0)));let HS=(i-(lG/jH));let HU=(i-f64::powf(HS,sf[198]));let HX=(if HQ{((jH*HU)/sf[198])}else{(if HF{((jH*HL)/sf[198])}else{(if Hm{((jH*Hq)/sf[198])}else{g})})});let I3=(nm+(sf[16]+(iG*lG)));let I5=(if sb[49]{(I3/oi)}else{g});let I6=(iG*I5);let I7=(I5-i);let Ia=((sf[202]+(I7*I7))).sqrt();let Ib=(i+I5);let Ie=((sf[204]+(Ib*Ib))).sqrt();let If=(Ia+Ie);let Ih=(if sb[49]{(I6/If)}else{g});let Im=(if sb[49]{(C*(((oi*Ih)-sf[16])-nm))}else{g});let Io=(i-(Im/jH));let Iq=(i-f64::powf(Io,sf[198]));let It=(if sb[49]{((jH*Iq)/sf[198])}else{HX});let Iw=(if sb[49]{(C*(i+Ih))}else{g});let Ix=(i-Iw);let IB=(if sb[49]{((pl*Ix)+(pp*Iw))}else{g});let ID=(oF+(lG-Im));let IJ=(if sb[51]{Hi}else{g});let IM=((sf[202]+(IJ*IJ))).sqrt();let IR=(if sb[51]{((C*(IJ-(if sb[51]{IM}else{g})))-nm)}else{Im});let IT=(i-(IR/jH));let IU=f64::powf(IT,sf[198]);let J3=(if sb[51]{(((if sb[51]{((nl*IU)/sf[198])}else{It})+(sf[206]*(pK+(lG-IR))))-pQ)}else{(if sb[49]{((It+(if sb[49]{(IB*ID)}else{g}))-oM)}else{(if ((sf[15])!=0.0){(HX+(if HE{g}else{(if Hm{(Hn*(Hj*Hw))}else{g})}))}else{g})})});let Jc=((sf[72]*ly)/1.44);let Jf=(Jc).exp();let Jg=(!(((if (Jc<sf[62]){i}else{g}))!=0.0));let Jp=(sf[232]*(i+(qW*sf[233])));let Jr=((if Jg{(sf[215]*(i+(Jc-sf[62])))}else{Jf})*sf[234]);let Jz=(sf[23]*(kd*nk));let JF=(ki*qc);let JK=(B1*sf[236]);let JN=(kk*J3);let JV=((lH-lI)*sf[238]);let JX=((lH-lM)*sf[239]);let JZ=(fO*sf[240]);let K1=(m2*sf[241]);let K4=((m3*sf[241])*0.3333333333333333);let K6=(sf[60]*(sf[213]*(kd*Hh)));let K8=(sf[60]*(B4*sf[236]));let Ka=(sf[60]*((ko*Gp)+(lX*sf[237])));let Kb=(if ((fS)!=0.0){fV}else{i});let Kf=(if g1{(-(g4*(-Kb)))}else{Kb});let Kh=((bM*Kf)/bO);let Ki=(Kf/sf[78]);let Lr=(-Ki);let Ls=(sf[91]*Lr);let LC=((hg*(sf[84]*(Ki*(sf[89]*f64::powf(ga,sf[252])))))+(hb*(hg*(((he*Ls)-(hd*(sf[88]*Kh)))/(he*he)))));let LZ=(sf[105]*Kh);let M3=(hs*hs);let M8=((hu*(sf[21]*(Ki*(sf[106]*f64::powf(ga,sf[254])))))+(hq*(hu*(((hs*(sf[108]*Lr))-(hr*LZ))/M3))));let NP=(sf[158]*Kf);let O8=(iG*(((ga*Kh)-(g9*Ki))/(ga*ga)));let Od=(g9*g9);let Oy=((iZ*(iX*Kh))+(iY*(Ki/ga)));let OB=((((iV*Ki)+(ga*((iU*O8)+(iI*(((iN*(((g9*(sf[167]*Ki))-(iL*Kh))/Od))-(iS*(((g9*(sf[168]*Ki))-(iQ*Kh))/Od)))/iT)))))-Oy)-(sf[113]*Ki));let OC=(iG*Kh);let OR=(OB+((je*OC)+(j5*((C*((cj*(j8*(((g9*(-OB))-(j6*Kh))/Od)))/(iG*jb)))/jd))));let Pe=((((js*Ki)+(ga*((jr*O8)+(iI*(((jl*(((g9*(sf[170]*Ki))-(jj*Kh))/Od))-(jp*(((g9*(sf[171]*Ki))-(jn*Kh))/Od)))/jq)))))-Oy)-(sf[124]*Ki));let Pt=(Pe+((jF*OC)+(j5*((C*((cj*(jz*(((g9*(-Pe))-(jx*Kh))/Od)))/(iG*jC)))/jE))));let PQ=((((jT*Ki)+(ga*((jS*O8)+(iI*(((jM*(((g9*(sf[173]*Ki))-(jK*Kh))/Od))-(jQ*(((g9*(sf[174]*Ki))-(jO*Kh))/Od)))/jR)))))-Oy)-(sf[133]*Ki));let Q5=(PQ+((k6*OC)+(j5*((C*((cj*(k0*(((g9*(-PQ))-(jY*Kh))/Od)))/(iG*k3)))/k5))));let Q8=(jg*jg);let Qe=(sf[175]*(((-(sf[166]*OR))/Q8)*(sf[176]*f64::powf(ka,sf[261]))));let Qh=(jH*jH);let Ql=(((-(sf[169]*Pt))/Qh)*(sf[178]*f64::powf(kf,sf[218])));let Qq=(k8*k8);let QJ=((kt*(sf[181]*(Ki*(sf[87]*f64::powf(ga,sf[263])))))+(kr*(kt*(((g9*Ls)-(hd*Kh))/Od))));
        let RK=(if lj{((-(sf[53]*(Ki*(sf[139]*f64::powf(ga,sf[242])))))/(ge*ge))}else{g});let RQ=(-OR);let RR=(sf[188]*RQ);let RS=(if ((sf[13])!=0.0){RR}else{g});let S1=(sf[189]*OR);let S2=(mo*(sf[193]*RS));let S5=(mo*mo);let S7=(sf[268]/mo);let S8=(sf[269]/mo);let Su=(-(sf[60]/jg));let Sv=(-(sf[265]/jg));let Sy=(sf[192]*f64::powf(mx,sf[270]));let SN=(if mv{(((mz*OR)+(jg*(-((-((-(ls*OR))/Q8))*Sy))))/sf[192])}else{(if mb{((mh*OR)/sf[192])}else{g})});let SO=(if mv{((jg*(-(Su*Sy)))/sf[192])}else{g});let SP=(if mv{((jg*(-(Sv*Sy)))/sf[192])}else{g});let SZ=(m6*RR);let T6=(if sb[47]{(iO*(RR+(if sb[47]{((SZ+SZ)/(iG*mL))}else{g})))}else{g});let Tj=(if sb[47]{(((mS*RQ)+(m4*((-(((jg*T6)-(mP*OR))/Q8))*(sf[192]*f64::powf(mR,sf[270])))))/sf[192])}else{g});let Tk=(if sb[47]{RR}else{g});let Tn=(mW*Tk);let Tp=(mW*sf[271]);let Tr=(mW*sf[272]);let Tt=(iG*mZ);let TH=(if sb[47]{((C*(Tk-(if sb[47]{((Tn+Tn)/Tt)}else{g})))-RR)}else{g});let TI=(if sb[47]{(C*(sf[271]-(if sb[47]{((Tp+Tp)/Tt)}else{g})))}else{g});let TJ=(if sb[47]{(C*(sf[272]-(if sb[47]{((Tr+Tr)/Tt)}else{g})))}else{g});let TU=(sf[192]*f64::powf(n6,sf[270]));let Ua=(sf[60]-TI);let Ub=(sf[265]-TJ);let Uc=(T6+(-TH));let UC=(if sb[47]{(((if sb[47]{(((n7*RQ)+(m4*((-(((jg*TH)-(n4*OR))/Q8))*TU)))/sf[192])}else{SN})+((ng*(sf[191]*Uc))+(nd*(((mo*(sf[193]*Uc))-(ne*S1))/S5))))-Tj)}else{(if ((sf[13])!=0.0){(SN+(if mv{g}else{(if mb{(mf*((mq*RS)+(m8*((S2-(mn*S1))/S5))))}else{g})}))}else{g})});let UD=(if sb[47]{((if sb[47]{((m4*((-(TI/jg))*TU))/sf[192])}else{SO})+((ng*(sf[191]*Ua))+(nd*((sf[193]*Ua)/mo))))}else{(if ((sf[13])!=0.0){(SO+(if mv{g}else{(if mb{(mf*((mq*sf[266])+(m8*S7)))}else{g})}))}else{g})});let UE=(if sb[47]{((if sb[47]{((m4*((-(TJ/jg))*TU))/sf[192])}else{SP})+((ng*(sf[191]*Ub))+(nd*((sf[193]*Ub)/mo))))}else{(if ((sf[13])!=0.0){(SP+(if mv{g}else{(if mb{(mf*((mq*sf[267])+(m8*S8)))}else{g})}))}else{g})});let UF=(-Pt);let UG=(sf[188]*UF);let UH=(if ((sf[15])!=0.0){UG}else{g});let UQ=(jH*(sf[199]*UH));let UU=(sf[275]/jH);let UV=(sf[276]/jH);let Vd=((-(sf[16]*Pt))/Qh);let Vh=(Vd*(sf[198]*f64::powf(nR,sf[277])));let Vm=(nV*nV);let VH=((jH*(-(nS*(-(sf[278]/nV)))))/sf[198]);let VI=((jH*(-(nS*(-(sf[279]/nV)))))/sf[198]);let VS=(-(sf[265]/jH));let VT=(-(sf[60]/jH));let VV=(sf[198]*f64::powf(o6,sf[277]));let Wa=(if o4{(((o8*Pt)+(jH*(-((-((-(ly*Pt))/Qh))*VV))))/sf[198])}else{(if nP{(((nZ*Pt)+(jH*(-((nX*Vh)+(nS*(-((-(nU*Pt))/Vm)))))))/sf[198])}else{(if nr{((nx*Pt)/sf[198])}else{g})})});let Wb=(if o4{((jH*(-(VS*VV)))/sf[198])}else{(if nP{VH}else{g})});let Wc=(if o4{((jH*(-(VT*VV)))/sf[198])}else{(if nP{VI}else{g})});let Wm=(-UG);let Wn=(oi*UG);let Wq=(oi*oi);let Ws=(if sb[49]{((Wn-(oh*Wm))/Wq)}else{g});let Wu=(om*Ws);let Wy=(os*Ws);let WO=(if sb[49]{(C*(((oA*Wm)+(oi*(if sb[49]{(((oy*(iG*Ws))-(ol*(((Wu+Wu)/(iG*or))+((Wy+Wy)/(iG*ox)))))/(oy*oy))}else{g})))-UG))}else{g});let X2=(if sb[49]{(((oJ*Pt)+(jH*(-((-(((jH*WO)-(oF*Pt))/Qh))*(sf[198]*f64::powf(oH,sf[277]))))))/sf[198])}else{g});let Xa=(if sb[49]{((Wn-(oP*Wm))/Wq)}else{g});let Xb=(if sb[49]{(sf[280]/oi)}else{g});let Xc=(if sb[49]{(sf[281]/oi)}else{g});let Xe=(iG*Xb);let Xf=(iG*Xc);let Xg=(oT*Xa);let Xi=(oT*Xb);let Xk=(oT*Xc);let Xm=(iG*oW);let Xq=(oX*Xa);let Xs=(oX*Xb);let Xu=(oX*Xc);let Xw=(iG*p0);let XG=(p1*p1);let XQ=(if sb[49]{(((p1*(iG*Xa))-(oS*(((Xg+Xg)/Xm)+((Xq+Xq)/Xw))))/XG)}else{g});let XR=(if sb[49]{(((p1*Xe)-(oS*(((Xi+Xi)/Xm)+((Xs+Xs)/Xw))))/XG)}else{g});let XS=(if sb[49]{(((p1*Xf)-(oS*(((Xk+Xk)/Xm)+((Xu+Xu)/Xw))))/XG)}else{g});let Y2=(if sb[49]{(C*(((p3*Wm)+(oi*XQ))-UG))}else{g});let Y3=(if sb[49]{(C*(oi*XR))}else{g});let Y4=(if sb[49]{(C*(oi*XS))}else{g});let Yf=(sf[198]*f64::powf(pa,sf[277]));let Yu=(if sb[49]{(((pc*Pt)+(jH*(-((-(((jH*Y2)-(p8*Pt))/Qh))*Yf))))/sf[198])}else{Wa});let Yv=(if sb[49]{((jH*(-((-(Y3/jH))*Yf)))/sf[198])}else{Wb});let Yw=(if sb[49]{((jH*(-((-(Y4/jH))*Yf)))/sf[198])}else{Wc});let YA=(if sb[49]{(C*XQ)}else{g});let YB=(if sb[49]{(C*XR)}else{g});let YC=(if sb[49]{(C*XS)}else{g});let YH=(if sb[49]{(Vd*(sf[205]*f64::powf(nR,sf[282])))}else{g});
        let YP=(if sb[49]{((((jH*UG)-(nm*Pt))/Qh)*(sf[205]*f64::powf(pn,sf[282])))}else{g});let Zw=(nm*UG);let ZD=(if sb[51]{(iO*(UG+(if sb[51]{((Zw+Zw)/(iG*pG))}else{g})))}else{WO});let ZQ=(if sb[51]{(((pN*UF)+(nl*((-(((jH*ZD)-(pK*Pt))/Qh))*(sf[198]*f64::powf(pM,sf[277])))))/sf[198])}else{g});let ZR=(if sb[51]{UG}else{g});let ZU=(pR*ZR);let ZW=(pR*sf[283]);let ZY=(pR*sf[284]);let a00=(iG*pU);let a0e=(if sb[51]{((C*(ZR-(if sb[51]{((ZU+ZU)/a00)}else{g})))-UG)}else{Y2});let a0f=(if sb[51]{(C*(sf[283]-(if sb[51]{((ZW+ZW)/a00)}else{g})))}else{Y3});let a0g=(if sb[51]{(C*(sf[284]-(if sb[51]{((ZY+ZY)/a00)}else{g})))}else{Y4});let a0r=(sf[198]*f64::powf(q1,sf[277]));let a0R=(if sb[51]{(((if sb[51]{(((q2*UF)+(nl*((-(((jH*a0e)-(pZ*Pt))/Qh))*a0r)))/sf[198])}else{Yu})+(sf[206]*(ZD+(-a0e))))-ZQ)}else{(if sb[49]{((Yu+(if sb[49]{((pw*(if sb[49]{(((pq*YH)+(pl*(-YA)))+((pp*YA)+(pi*YP)))}else{g}))+(pu*(WO+(-Y2))))}else{g}))-X2)}else{(if ((sf[15])!=0.0){(Wa+(if nO{g}else{(if nr{(nu*((nF*UH)+(no*((UQ-(nD*Pt))/Qh))))}else{g})}))}else{g})})});let a0S=(if sb[51]{((if sb[51]{((nl*((-(a0f/jH))*a0r))/sf[198])}else{Yv})+(sf[206]*(sf[265]-a0f)))}else{(if sb[49]{(Yv+(if sb[49]{((pw*(if sb[49]{((pl*(-YB))+(pp*YB))}else{g}))+(pu*(sf[265]-Y3)))}else{g}))}else{(if ((sf[15])!=0.0){(Wb+(if nO{g}else{(if nr{(nu*((nF*sf[273])+(no*UU)))}else{g})}))}else{g})})});let a0T=(if sb[51]{((if sb[51]{((nl*((-(a0g/jH))*a0r))/sf[198])}else{Yw})+(sf[206]*(sf[60]-a0g)))}else{(if sb[49]{(Yw+(if sb[49]{((pw*(if sb[49]{((pl*(-YC))+(pp*YC))}else{g}))+(pu*(sf[60]-Y4)))}else{g}))}else{(if ((sf[15])!=0.0){(Wc+(if nO{g}else{(if nr{(nu*((nF*sf[274])+(no*UV)))}else{g})}))}else{g})})});let a0Z=((-((ij*Kh)+(g9*(sf[88]*NP))))/(qd*qd));let a11=(sf[60]*qe);let a12=(qe*sf[265]);let a13=(qi*(ls*a0Z));let a14=(qi*a11);let a15=(qi*a12);let a1r=((-((ik*Kh)+(g9*(sf[99]*NP))))/(qt*qt));let a1t=(qu*sf[265]);let a1u=(sf[60]*qu);let a1v=(qy*(ly*a1r));let a1w=(qy*a1t);let a1x=(qy*a1u);let a1L=((ho*LC)+(hh*((hn*(sf[97]*(Ki*(sf[100]*f64::powf(ga,sf[253])))))+(hj*(hn*(((hl*(sf[102]*Lr))-(hk*(sf[99]*Kh)))/(hl*hl)))))));let a1V=(li*UE);let a1Z=(lf*a0S);let a21=(((nk*(if lg{((-(sf[186]*(sf[187]*Kf)))/(kM*kM))}else{g}))+(li*UC))+((qc*(if ld{((-(sf[184]*(sf[185]*Kf)))/(kH*kH))}else{g}))+(lf*a0R)));let a22=((li*UD)+(lf*a0T));let a23=(qP*a21);let a25=(qP*a1Z);let a27=(qP*a22);let a29=(qP*a1V);let a2b=(iG*qT);let a2k=(C*(a21+((a23+a23)/a2b)));let a2l=(C*(a1Z+((a25+a25)/a2b)));let a2m=(C*(a22+((a27+a27)/a2b)));let a2n=(C*(a1V+((a29+a29)/a2b)));let a2A=(sf[207]*f64::powf(qW,sf[285]));let a2B=(a2k*a2A);let a2C=(a2l*a2A);let a2D=(a2m*a2A);let a2E=(a2n*a2A);let a36=(C*a2k);let a37=(C*a2l);let a38=(C*a2m);let a39=(C*a2n);let a3A=(sf[209]*a36);let a3B=(sf[209]*a37);let a3C=(sf[209]*a38);let a3D=(sf[209]*a39);let a4f=(if ((sf[22])!=0.0){((-LZ)/M3)}else{a1r});let a4h=(sf[60]*rB);let a4i=(rB*sf[265]);let a4j=(rF*(lG*a4f));let a4k=(rF*a4h);let a4l=(rF*a4i);let a4I=(rX*(ly*a4f));let a4J=(rX*a4i);let a4K=(rX*a4h);let an7=((-(ly*Kh))/Od);let an8=(sf[265]/g9);let an9=(sf[60]/g9);let ana=(AJ*an7);let anb=(AJ*an8);let anc=(AJ*an9);let and=(sf[215]*an7);let ane=(sf[215]*an8);let anf=(sf[215]*an9);let anl=((-(lB*Kh))/Od);let any=(iG*B1);let anz=(((AP*QJ)+(ku*(if AK{and}else{ana})))/any);let anA=((ku*(if AK{ane}else{anb}))/any);let anB=((ku*(if AK{anf}else{anc}))/any);let anH=(iG*B4);let anI=(((AY*QJ)+(ku*(if AU{(sf[215]*anl)}else{(AT*anl)})))/anH);let anJ=((ku*(if AU{ane}else{(AT*an8)}))/anH);let anK=((ku*(if AU{anf}else{(AT*an9)}))/anH);let azv=(-Q5);let azx=(if ((sf[46])!=0.0){(sf[188]*azv)}else{g});let azy=(if sb[80]{azx}else{g});let azH=(sf[189]*Q5);let azL=(Fq*Fq);let aAe=(sf[228]*f64::powf(Fz,sf[310]));let aAt=(if Fx{(((FB*Q5)+(k8*(-((-((-(lX*Q5))/Qq))*aAe))))/sf[228])}else{(if Fe{((Fj*Q5)/sf[228])}else{g})});let aAu=(if Fx{((k8*(-((-(sf[265]/k8))*aAe)))/sf[228])}else{g});let aAv=(if Fx{((k8*(-((-(sf[60]/k8))*aAe)))/sf[228])}else{g});let aAF=(F8*azx);let aAM=(if sb[82]{(iO*(azx+(if sb[82]{((aAF+aAF)/(iG*FO))}else{g})))}else{g});let aB0=(if sb[82]{azx}else{g});let aB3=(FZ*aB0);let aB5=(FZ*sf[311]);let aB7=(FZ*sf[312]);
        let aB9=(iG*G2);let aBn=(if sb[82]{((C*(aB0-(if sb[82]{((aB3+aB3)/aB9)}else{g})))-azx)}else{g});let aBo=(if sb[82]{(C*(sf[311]-(if sb[82]{((aB5+aB5)/aB9)}else{g})))}else{g});let aBp=(if sb[82]{(C*(sf[312]-(if sb[82]{((aB7+aB7)/aB9)}else{g})))}else{g});let aBA=(sf[228]*f64::powf(G9,sf[310]));let aBQ=(sf[265]-aBo);let aBR=(sf[60]-aBp);let aBS=(aAM+(-aBn));let aCO=(sf[192]*f64::powf(GK,sf[270]));let aD3=(if GI{(((GM*OR)+(jg*(-((-((-(lv*OR))/Q8))*aCO))))/sf[192])}else{(if Gu{((Gx*OR)/sf[192])}else{g})});let aD4=(if GI{((jg*(-(Su*aCO)))/sf[192])}else{g});let aD5=(if GI{((jg*(-(Sv*aCO)))/sf[192])}else{g});let aDf=(GT*Tk);let aDh=(GT*sf[271]);let aDj=(GT*sf[272]);let aDl=(iG*GW);let aDz=(if sb[47]{((C*(Tk-(if sb[47]{((aDf+aDf)/aDl)}else{g})))-RR)}else{g});let aDA=(if sb[47]{(C*(sf[271]-(if sb[47]{((aDh+aDh)/aDl)}else{g})))}else{g});let aDB=(if sb[47]{(C*(sf[272]-(if sb[47]{((aDj+aDj)/aDl)}else{g})))}else{g});let aDM=(sf[192]*f64::powf(H3,sf[270]));let aE2=(sf[60]-aDA);let aE3=(sf[265]-aDB);let aE4=(T6+(-aDz));let aFc=(sf[198]*f64::powf(HS,sf[277]));let aFr=(if HQ{(((HU*Pt)+(jH*(-((-((-(lG*Pt))/Qh))*aFc))))/sf[198])}else{(if HF{(((HL*Pt)+(jH*(-((HJ*Vh)+(nS*(-((-(HH*Pt))/Vm)))))))/sf[198])}else{(if Hm{((Hq*Pt)/sf[198])}else{g})})});let aFs=(if HQ{((jH*(-(VT*aFc)))/sf[198])}else{(if HF{VI}else{g})});let aFt=(if HQ{((jH*(-(VS*aFc)))/sf[198])}else{(if HF{VH}else{g})});let aFG=(if sb[49]{((Wn-(I3*Wm))/Wq)}else{g});let aFI=(I7*aFG);let aFK=(I7*Xc);let aFM=(I7*Xb);let aFO=(iG*Ia);let aFS=(Ib*aFG);let aFU=(Ib*Xc);let aFW=(Ib*Xb);let aFY=(iG*Ie);let aG8=(If*If);let aGi=(if sb[49]{(((If*(iG*aFG))-(I6*(((aFI+aFI)/aFO)+((aFS+aFS)/aFY))))/aG8)}else{g});let aGj=(if sb[49]{(((If*Xf)-(I6*(((aFK+aFK)/aFO)+((aFU+aFU)/aFY))))/aG8)}else{g});let aGk=(if sb[49]{(((If*Xe)-(I6*(((aFM+aFM)/aFO)+((aFW+aFW)/aFY))))/aG8)}else{g});let aGu=(if sb[49]{(C*(((Ih*Wm)+(oi*aGi))-UG))}else{g});let aGv=(if sb[49]{(C*(oi*aGj))}else{g});let aGw=(if sb[49]{(C*(oi*aGk))}else{g});let aGH=(sf[198]*f64::powf(Io,sf[277]));let aGW=(if sb[49]{(((Iq*Pt)+(jH*(-((-(((jH*aGu)-(Im*Pt))/Qh))*aGH))))/sf[198])}else{aFr});let aGX=(if sb[49]{((jH*(-((-(aGv/jH))*aGH)))/sf[198])}else{aFs});let aGY=(if sb[49]{((jH*(-((-(aGw/jH))*aGH)))/sf[198])}else{aFt});let aH2=(if sb[49]{(C*aGi)}else{g});let aH3=(if sb[49]{(C*aGj)}else{g});let aH4=(if sb[49]{(C*aGk)}else{g});let aHL=(IJ*ZR);let aHN=(IJ*sf[284]);let aHP=(IJ*sf[283]);let aHR=(iG*IM);let aI5=(if sb[51]{((C*(ZR-(if sb[51]{((aHL+aHL)/aHR)}else{g})))-UG)}else{aGu});let aI6=(if sb[51]{(C*(sf[284]-(if sb[51]{((aHN+aHN)/aHR)}else{g})))}else{aGv});let aI7=(if sb[51]{(C*(sf[283]-(if sb[51]{((aHP+aHP)/aHR)}else{g})))}else{aGw});let aIi=(sf[198]*f64::powf(IT,sf[277]));let aJi=(sf[232]*(sf[233]*a2k));let aJj=(sf[232]*(sf[233]*a2l));let aJk=(sf[232]*(sf[233]*a2m));let aJl=(sf[232]*(sf[233]*a2n));let aJm=(sf[234]*(if Jg{sf[317]}else{(Jf*sf[315])}));let aJn=(sf[234]*(if Jg{sf[318]}else{(Jf*sf[316])}));let aJV=(sf[23]*((nk*Qe)+(kd*UC)));let aJW=(sf[23]*(kd*UD));let aJX=(sf[23]*(kd*UE));let aKB=((qc*(sf[177]*Ql))+(ki*a0R));let aKC=(ki*a0S);let aKD=(ki*a0T);let aKK=(sf[236]*anz);let aKL=(sf[236]*anA);let aKM=(sf[236]*anB);let aKV=((J3*(sf[179]*Ql))+(kk*(if sb[51]{(((if sb[51]{(((IU*UF)+(nl*((-(((jH*aI5)-(IR*Pt))/Qh))*aIi)))/sf[198])}else{aGW})+(sf[206]*(ZD+(-aI5))))-ZQ)}else{(if sb[49]{((aGW+(if sb[49]{((ID*(if sb[49]{(((Ix*YH)+(pl*(-aH2)))+((Iw*YP)+(pp*aH2)))}else{g}))+(IB*(WO+(-aGu))))}else{g}))-X2)}else{(if ((sf[15])!=0.0){(aFr+(if HE{g}else{(if Hm{(Hn*((Hw*UH)+(Hj*((UQ-(Hu*Pt))/Qh))))}else{g})}))}else{g})})})));let aKW=(kk*(if sb[51]{((if sb[51]{((nl*((-(aI6/jH))*aIi))/sf[198])}else{aGX})+(sf[206]*(sf[60]-aI6)))}else{(if sb[49]{(aGX+(if sb[49]{((ID*(if sb[49]{((pl*(-aH3))+(pp*aH3))}else{g}))+(IB*(sf[60]-aGv)))}else{g}))}else{(if ((sf[15])!=0.0){(aFs+(if HE{g}else{(if Hm{(Hn*((Hw*sf[274])+(Hj*UV)))}else{g})}))}else{g})})}));
        let aKX=(kk*(if sb[51]{((if sb[51]{((nl*((-(aI7/jH))*aIi))/sf[198])}else{aGY})+(sf[206]*(sf[265]-aI7)))}else{(if sb[49]{(aGY+(if sb[49]{((ID*(if sb[49]{((pl*(-aH4))+(pp*aH4))}else{g}))+(IB*(sf[265]-aGw)))}else{g}))}else{(if ((sf[15])!=0.0){(aFt+(if HE{g}else{(if Hm{(Hn*((Hw*sf[273])+(Hj*UU)))}else{g})}))}else{g})})}));let aLm=(sf[60]*(sf[213]*((Hh*Qe)+(kd*(if sb[47]{(((if sb[47]{(((H4*RQ)+(m4*((-(((jg*aDz)-(H1*OR))/Q8))*aDM)))/sf[192])}else{aD3})+((Hd*(sf[191]*aE4))+(Ha*(((mo*(sf[193]*aE4))-(Hb*S1))/S5))))-Tj)}else{(if ((sf[13])!=0.0){(aD3+(if GI{g}else{(if Gu{(Gv*((GD*RS)+(Gr*((S2-(GB*S1))/S5))))}else{g})}))}else{g})})))));let aLn=(sf[60]*(sf[213]*(kd*(if sb[47]{((if sb[47]{((m4*((-(aDA/jg))*aDM))/sf[192])}else{aD4})+((Hd*(sf[191]*aE2))+(Ha*((sf[193]*aE2)/mo))))}else{(if ((sf[13])!=0.0){(aD4+(if GI{g}else{(if Gu{(Gv*((GD*sf[266])+(Gr*S7)))}else{g})}))}else{g})}))));let aLo=(sf[60]*(sf[213]*(kd*(if sb[47]{((if sb[47]{((m4*((-(aDB/jg))*aDM))/sf[192])}else{aD5})+((Hd*(sf[191]*aE3))+(Ha*((sf[193]*aE3)/mo))))}else{(if ((sf[13])!=0.0){(aD5+(if GI{g}else{(if Gu{(Gv*((GD*sf[267])+(Gr*S8)))}else{g})}))}else{g})}))));let aLs=(sf[60]*(sf[236]*anI));let aLt=(sf[60]*(sf[236]*anJ));let aLu=(sf[60]*(sf[236]*anK));let aLA=(sf[60]*((Gp*(sf[45]*(((-(sf[172]*Q5))/Qq)*(sf[180]*f64::powf(kl,sf[262])))))+(ko*(if sb[83]{g}else{(if sb[82]{(((if sb[82]{(((Ga*azv)+(F6*((-(((k8*aBn)-(G7*Q5))/Qq))*aBA)))/sf[228])}else{aAt})+((Gj*(sf[227]*aBS))+(Gg*(((Fq*(sf[229]*aBS))-(Gh*azH))/azL))))-(if sb[82]{(((FV*azv)+(F6*((-(((k8*aAM)-(FS*Q5))/Qq))*(sf[228]*f64::powf(FU,sf[310])))))/sf[228])}else{g}))}else{(if sb[80]{(aAt+(if Fx{g}else{(if Fe{(Fh*((Fs*azy)+(Fb*(((Fq*(sf[229]*azy))-(Fp*azH))/azL))))}else{g})}))}else{g})})}))));let aLB=(sf[60]*((ko*(if sb[83]{g}else{(if sb[82]{((if sb[82]{((F6*((-(aBo/k8))*aBA))/sf[228])}else{aAu})+((Gj*(sf[227]*aBQ))+(Gg*((sf[229]*aBQ)/Fq))))}else{(if sb[80]{(aAu+(if Fx{g}else{(if Fe{(Fh*((Fs*sf[306])+(Fb*(sf[308]/Fq))))}else{g})}))}else{g})})}))+sf[319]));let aLC=(sf[60]*((ko*(if sb[83]{g}else{(if sb[82]{((if sb[82]{((F6*((-(aBp/k8))*aBA))/sf[228])}else{aAv})+((Gj*(sf[227]*aBR))+(Gg*((sf[229]*aBR)/Fq))))}else{(if sb[80]{(aAv+(if Fx{g}else{(if Fe{(Fh*((Fs*sf[307])+(Fb*(sf[309]/Fq))))}else{g})}))}else{g})})}))+sf[320]));

        CommonStampValues {
            g, i, C, cj, fO, g9, ga, gb,
            hc, hh, hv, iG, jH, ll, lp, lq,
            ls, lt, lv, lw, ly, lz, lE, lG,
            lH, lI, lM, lV, lX, m2, m3, qe,
            qi, qu, qy, qH, qO, qR, qW, r1,
            rg, ro, rw, rB, rF, rX, AH, AJ,
            AO, B1, B4, Jp, Jr, Jz, JF, JK,
            JN, JV, JX, JZ, K1, K4, K6, K8,
            Ka, Kf, Kh, Ki, Lr, LC, M8, Pt,
            RK, a0Z, a11, a12, a13, a14, a15, a1r,
            a1t, a1u, a1v, a1w, a1x, a1L, a2k, a2l,
            a2m, a2n, a2B, a2C, a2D, a2E, a36, a37,
            a38, a39, a3A, a3B, a3C, a3D, a4f, a4h,
            a4i, a4j, a4k, a4l, a4I, a4J, a4K, ana,
            anb, anc, and, ane, anf, anz, anA, anB,
            anI, anJ, anK, aJi, aJj, aJk, aJl, aJm,
            aJn, aJV, aJW, aJX, aKB, aKC, aKD, aKK,
            aKL, aKM, aKV, aKW, aKX, aLm, aLn, aLo,
            aLs, aLt, aLu, aLA, aLB, aLC,
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let CommonStampValues {
            g, i, C, cj, fO, g9, ga, gb,
            hc, hh, hv, iG, jH, ll, lp, lq,
            ls, lt, lv, lw, ly, lz, lE, lG,
            lH, lI, lM, lV, lX, m2, m3, qe,
            qi, qu, qy, qH, qO, qR, qW, r1,
            rg, ro, rw, rB, rF, rX, AH, AJ,
            AO, B1, B4, Jp, Jr, Jz, JF, JK,
            JN, JV, JX, JZ, K1, K4, K6, K8,
            Ka, Kf, Kh, Ki, Lr, LC, M8, Pt,
            RK, a0Z, a11, a12, a13, a14, a15, a1r,
            a1t, a1u, a1v, a1w, a1x, a1L, a2k, a2l,
            a2m, a2n, a2B, a2C, a2D, a2E, a36, a37,
            a38, a39, a3A, a3B, a3C, a3D, a4f, a4h,
            a4i, a4j, a4k, a4l, a4I, a4J, a4K, ana,
            anb, anc, and, ane, anf, anz, anA, anB,
            anI, anJ, anK, aJi, aJj, aJk, aJl, aJm,
            aJn, aJV, aJW, aJX, aKB, aKC, aKD, aKK,
            aKL, aKM, aKV, aKW, aKX, aLm, aLn, aLo,
            aLs, aLt, aLu, aLA, aLB, aLC,
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
        let aq=0.01;let au=ctx.simparam_or("gmin", 1e-12);let aw=(if sb[26]{au}else{sf[50]});let az=ctx.simparam_or("pnjmaxi", i);let aB=(if sb[27]{az}else{sf[52]});let c2=(if sb[41]{g}else{(sf[342]*((sf[344]+(aB/sf[5]))).ln())});let ci=(C*aB);let cB=(if sb[89]{g}else{(if (sb[28]&&(aB>sf[53])){(sf[349]*((i+(f64::powf((ci*sf[94]),sf[96])/sf[352]))).ln())}else{(sf[349]*((i+(aB/sf[352]))).ln())})});let d7=(if sb[92]{g}else{(if (sb[29]&&(aB>sf[54])){(sf[357]*((i+(f64::powf((ci*sf[104]),sf[96])/sf[362]))).ln())}else{(sf[357]*((i+(aB/sf[362]))).ln())})});let dx=(if sb[94]{g}else{(if (sb[30]&&(aB>sf[55])){(sf[366]*((i+((sf[66]*(aB*aB))/sf[369]))).ln())}else{(sf[366]*((i+(aB/sf[369]))).ln())})});let dQ=(sf[374]*((i+(aB/sf[377]))).ln());let dS=(if sb[96]{g}else{dQ});let eb=(sf[382]*((i+(aB/sf[385]))).ln());let ed=(if sb[98]{g}else{eb});let ev=(sf[390]*((i+(aB/sf[393]))).ln());let ex=(if sb[100]{g}else{ev});let eP=(sf[398]*((i+(aB/sf[401]))).ln());let eR=(if sb[102]{g}else{eP});let eZ=(sf[390]*((i+(aB/sf[404]))).ln());let f9=(sf[398]*((i+(aB/sf[407]))).ln());let fb=(if sb[106]{g}else{f9});let fs=(sf[412]*((i+(aB/sf[415]))).ln());let fL=(sf[420]*((i+(aB/sf[423]))).ln());let gm=f64::powf(ga,sf[142]);let go=(if sb[42]{(sf[140]*gm)}else{(if ((sf[7])!=0.0){(sf[140]*f64::powf(ga,sf[141]))}else{g})});let gw=(if sb[43]{(gm*sf[143])}else{(if ((sf[8])!=0.0){(sf[143]*f64::powf(ga,sf[144]))}else{g})});let gE=f64::powf(ga,sf[147]);let gG=(if sb[44]{(sf[145]*gE)}else{(if ((sf[9])!=0.0){(sf[145]*f64::powf(ga,sf[146]))}else{g})});let gO=(if sb[45]{(gE*sf[148])}else{(if ((sf[10])!=0.0){(sf[148]*f64::powf(ga,sf[149]))}else{g})});let gS=(sf[150]*f64::powf(ga,sf[151]));let gW=(sf[152]*f64::powf(ga,sf[153]));let h4=(if sb[46]{(gm*sf[154])}else{(if ((sf[11])!=0.0){(sf[154]*f64::powf(ga,sf[155]))}else{g})});let h9=(sf[156]*(i+(gb*sf[157])));let hx=(sf[109]*f64::powf(ga,sf[112]));let hy=(sf[114]*hc);let hz=(sf[111]*g9);let hB=((hy/hz)).exp();let hC=(hx*hB);let hE=(sf[115]*f64::powf(ga,sf[118]));let hF=(sf[120]*hc);let hG=(sf[117]*g9);let hI=((hF/hG)).exp();let hJ=(hE*hI);let hK=f64::powf(ga,sf[123]);let hL=(sf[121]*hK);let hM=(sf[125]*hc);let hN=(sf[122]*g9);let hP=((hM/hN)).exp();let hQ=(hL*hP);let hR=f64::powf(ga,sf[128]);let hS=(sf[126]*hR);let hT=(sf[130]*hc);let hU=(sf[127]*g9);let hW=((hT/hU)).exp();let hX=(hS*hW);let hY=(sf[30]*hK);let hZ=(hP*hY);let i0=(sf[31]*hR);let i1=(hW*i0);let i3=(sf[42]*f64::powf(ga,sf[132]));let i4=(sf[134]*hc);let i5=(sf[131]*g9);let i7=((i4/i5)).exp();let i8_=(i3*i7);let ia=(sf[43]*f64::powf(ga,sf[136]));let ib=(sf[138]*hc);let ic=(sf[135]*g9);let ie=((ib/ic)).exp();let if_=(ia*ie);let ip=(sf[159]*(i+(gb*sf[160])));let iu=(sf[161]*(i+(gb*sf[162])));let iy=(sf[163]+(gb*sf[164]));let iF=(sf[85]*(i+(gb*sf[165])));let ky=(sf[182]*f64::powf(ga,sf[183]));let kz=(-(sf[27]*(i+(gb*iy))));let kA=(g9*iF);let kC=((kz/kA)).exp();let kN=0.001;let kO=(go>kN);let kQ=1000.0;let kR=(if kO{(i/go)}else{kQ});let kS=(gw>kN);let kU=(if kS{(i/gw)}else{kQ});let kV=(gG>kN);let kX=(if kV{(i/gG)}else{kQ});let kY=(gO>kN);let l0=(if kY{(i/gO)}else{kQ});let l1=(gS>kN);let l3=(if l1{(i/gS)}else{kQ});let l4=(h4>kN);let l6=(if l4{(i/h4)}else{kQ});let l7=(gW>kN);let l9=(if l7{(i/gW)}else{kQ});let la=(h9>kN);let lc=(if la{(i/h9)}else{kQ});let lm=(ky>g);let lo=(if lm{(i/ky)}else{g});let lD=(sf[60]*(lt-lz));let lL=(sf[60]*(lw-lq));let lO=(lM-lz);let lQ=(sf[60]*(lz-lw));let lR=(lH-lt);let lS=(lt-lp);let lT=(lI-lq);let lU=(lE-lz);let lZ=(sf[60]*(lt-lV));let m1=(ctx.node_voltage(n[3])-lV);let qj=(!(((if (ls<cB){i}else{g}))!=0.0));let ql=((cB*qe)).exp();let qm=(ls-cB);let qo=(i+(qe*qm));let qr=((if qj{(ql*qo)}else{qi})-i);let qs=(hh*qr);let qv=(ly<d7);let qz=(!(((if qv{i}else{g}))!=0.0));let qB=((d7*qu)).exp();let qC=(ly-d7);let qE=(i+(qu*qC));let qF=(qB*qE);let qI=((if qz{qF}else{qy})-i);let qJ=(qH*qI);let r2=(cj*((ll*qs)+(sf[64]*qJ)));let r3=(r1+r2);let r6=(if ((if ((sf[20])!=0.0){r3}else{g})>qR){i}else{g});let r7=(((sf[20])!=0.0)&&((r6)!=0.0));let rd=(((sf[20])!=0.0)&&(!((r6)!=0.0)));let rj=(i+r2);let rm=(if ((if sb[52]{rj}else{r3})>qR){i}else{g});
        let rn=(sb[52]&&((rm)!=0.0));let rq=(i+f64::powf(rj,sf[93]));let ru=(sb[52]&&(!((rm)!=0.0)));let rx=(if ru{rw}else{(if rn{(ro*rq)}else{(if rd{rg}else{(if r7{(C*(qW+f64::powf(r3,sf[93])))}else{g})})})});let ry=(qJ/rx);let rz=(qs/rx);let rC=(lG<dx);let rG=(!qv);let rJ=(((sf[22])!=0.0)&&(!(((if rC{i}else{g}))!=0.0)));let rL=((dx*rB)).exp();let rM=(lG-dx);let rO=(i+(rB*rM));let rP=(rL*rO);let rQ=(sb[7]&&rC);let rR=(if rQ{rF}else{(if rG{qF}else{qy})});let rT=(ly<dx);let rU=(if rT{i}else{g});let rV=(((sf[22])!=0.0)&&((rU)!=0.0));let s0=(((sf[22])!=0.0)&&(!((rU)!=0.0)));let s1=(ly-dx);let s3=(i+(rB*s1));let s4=(rL*s3);let s7=(!rC);let sb_=(!rT);let sf_=(((sf[210]*(if s7{rP}else{rF}))+(sf[211]*(if sb_{s4}else{rX})))-i);let sh=(if ((sf[22])!=0.0){(hv*sf_)}else{g});let sl=((((if rJ{rP}else{rR})*sf[210])+((if s0{s4}else{(if rV{rX}else{g})})*sf[211]))-i);let sm=(hv*sl);let su=(if ((sf[22])!=0.0){(i+(cj*(sf[66]*sh)))}else{(if sb[53]{rj}else{r3})});let sw=(if (su>qR){i}else{g});let sx=(((sf[22])!=0.0)&&((sw)!=0.0));let sz=(i+(cj*(if ((sf[22])!=0.0){(sf[66]*sm)}else{g})));let sA=(sz).sqrt();let sF=(((sf[22])!=0.0)&&(!((sw)!=0.0)));let sG=0.50005;let sH=(if sF{sG}else{(if sx{(C*(i+sA))}else{g})});let sI=(lX<dx);let sK=((lX*rB)).exp();let sL=(sb[7]&&s7);let sN=(lX-dx);let sP=(i+(rB*sN));let sQ=(rL*sP);let sR=(sb[7]&&sI);let sT=(!sI);let sV=((if sT{sQ}else{sK})-i);let sY=(su).sqrt();let t2=(!(sz>qR));let t6=(if sb[54]{i}else{(if t2{sG}else{(C*(i+sY))})});let t7=(sh-(if ((sf[22])!=0.0){(hv*sV)}else{g}));let t9=(if sb[54]{g}else{(t7/sH)});let ta=(i/hz);let tb=(if ((sf[24])!=0.0){ta}else{rB});let tc=(ls<dS);let tf=((ls*tb)).exp();let tg=(sb[7]&&sT);let ti=(!(((if tc{i}else{g}))!=0.0));let tj=(((sf[24])!=0.0)&&ti);let tm=(if (!(hC>g)){g}else{dQ});let to=((tb*tm)).exp();let tp=(ls-tm);let tr=(i+(tb*tp));let tt=(ls<tm);let tu=(sb[8]&&tt);let tv=(if tu{tf}else{(if tg{sQ}else{(if sR{sK}else{(if sL{rP}else{rR})})})});let tx=(i/hG);let ty=(if ((sf[24])!=0.0){tx}else{tb});let tz=(ls<ed);let tA=(if tz{i}else{g});let tB=(((sf[24])!=0.0)&&((tA)!=0.0));let tD=((ls*ty)).exp();let tF=(!((tA)!=0.0));let tG=(((sf[24])!=0.0)&&tF);let tJ=(if (!(hJ>g)){g}else{eb});let tL=((ty*tJ)).exp();let tM=(ls-tJ);let tO=(i+(ty*tM));let tQ=(if tG{(tL*tO)}else{(if tB{tD}else{g})});let tU=(i+(sf[25]*(qW-i)));let tV=(hC*tU);let tX=((dS*tb)).exp();let tY=(ls-dS);let u0=(i+(tb*tY));let u1=(tX*u0);let u2=(!tc);let u4=((if u2{u1}else{tf})-i);let u7=((ed*ty)).exp();let u8_=(ls-ed);let ua=(i+(ty*u8_));let ub=(u7*ua);let uc=(!tz);let ue=((if uc{ub}else{tD})-i);let uf=(hJ*ue);let uo=(kz-ls);let up=(if sb[58]{uo}else{g});let uq=(i/kA);let ur=(if sb[58]{uq}else{ty});let uu=((up*ur)).exp();let uv=((if tj{(to*tr)}else{tv})-i);let ux=(tQ-i);let uy=(hJ*ux);let uF=((c2*ur)).exp();let uG=(up-c2);let uI=(i+(ur*uG));let uK=(!(up<c2));let uT=(if sb[61]{ta}else{ur});let uU=(lv<dS);let uX=((lv*uT)).exp();let uY=(sb[8]&&u2);let v0=(!(((if uU{i}else{g}))!=0.0));let v1=(sb[61]&&v0);let v3=((tm*uT)).exp();let v4=(lv-tm);let v6=(i+(uT*v4));let va=(lv<tm);let vb=(sb[63]&&va);let vc=(if vb{uX}else{(if uY{u1}else{tv})});let ve=(if sb[61]{tx}else{uT});let vf=(lv<ed);let vg=(if vf{i}else{g});let vh=(sb[61]&&((vg)!=0.0));let vj=((lv*ve)).exp();let vl=(!((vg)!=0.0));let vm=(sb[61]&&vl);let vo=((tJ*ve)).exp();let vp=(lv-tJ);let vr=(i+(ve*vp));let vt=(if vm{(vo*vr)}else{(if vh{vj}else{tQ})});let vv=((dS*uT)).exp();let vw=(lv-dS);let vy=(i+(uT*vw));let vz=(vv*vy);let vA=(!uU);let vC=((if vA{vz}else{uX})-i);let vF=((ed*ve)).exp();let vG=(lv-ed);let vI=(i+(ve*vG));let vJ=(vF*vI);let vK=(!vf);let vM=((if vK{vJ}else{vj})-i);let vR=(if sb[64]{uo}else{up});let vS=(if sb[64]{uq}else{ve});let vV=((vR*vS)).exp();let vW=((if v1{(v3*v6)}else{vc})-i);let vY=(vt-i);let w2=((c2*vS)).exp();let w3=(vR-c2);let w5=(i+(vS*w3));let w7=(!(vR<c2));let wf=(if sb[66]{ta}else{vS});let wh=((ls*wf)).exp();let wi=(sb[63]&&vA);let wk=(ti&&sb[66]);let wm=((tm*wf)).exp();let wo=(i+(tp*wf));let ws=(tt&&sb[68]);let wv=(if sb[66]{tx}else{wf});let ww=(((tA)!=0.0)&&sb[66]);let wy=((ls*wv)).exp();let wA=(tF&&sb[66]);let wC=((tJ*wv)).exp();
        let wE=(i+(tM*wv));let wJ=((dS*wf)).exp();let wL=(i+(tY*wf));let wM=(wJ*wL);let wO=((if u2{wM}else{wh})-i);let wR=((ed*wv)).exp();let wT=(i+(u8_*wv));let wU=(wR*wT);let wW=((if uc{wU}else{wy})-i);let wX=(hJ*wW);let x7=(if sb[71]{uo}else{vR});let x8=(if sb[71]{uq}else{wv});let xb=((x7*x8)).exp();let xc=((if wk{(wm*wo)}else{(if ws{wh}else{(if wi{vz}else{vc})})})-i);let xe=((if wA{(wC*wE)}else{(if ww{wy}else{vt})})-i);let xf=(hJ*xe);let xo=((c2*x8)).exp();let xp=(x7-c2);let xr=(i+(x8*xp));let xt=(!(x7<c2));let xy=(if sb[71]{((if sb[59]{(sf[23]*(xf+(hC*xc)))}else{(sf[23]*((tV*xc)+xf))})-(sf[212]*((if xt{(xo*xr)}else{xb})-kC)))}else{(if sb[70]{(sf[23]*(wX+(hC*wO)))}else{(if sb[69]{(sf[23]*((tV*wO)+wX))}else{(if sb[61]{g}else{(if sb[58]{((if sb[59]{(uy+(hC*uv))}else{((tV*uv)+uy)})-(sf[5]*((if uK{(uF*uI)}else{uu})-kC)))}else{(if sb[57]{(uf+(hC*u4))}else{(if sb[55]{((tV*u4)+uf)}else{g})})})})})})});let xz=(if sb[66]{ta}else{x8});let xB=((lv*xz)).exp();let xG=(u2&&sb[68]);let xI=(v0&&sb[66]);let xK=((tm*xz)).exp();let xM=(i+(v4*xz));let xO=(va&&sb[68]);let xR=(if sb[66]{tx}else{xz});let xT=((lv*xR)).exp();let xU=(sb[8]&&uc);let xW=(lv<tJ);let xX=(sb[63]&&xW);let xZ=(sb[63]&&vK);let y2=(sb[68]&&(ls<tJ));let y4=(uc&&sb[68]);let y6=(vl&&sb[66]);let y8=((tJ*xR)).exp();let ya=(i+(vp*xR));let yc=(sb[68]&&xW);let yh=((dS*xz)).exp();let yj=(i+(vw*xz));let ym=((if vA{(yh*yj)}else{xB})-i);let yp=((ed*xR)).exp();let yr=(i+(vG*xR));let yu=((if vK{(yp*yr)}else{xT})-i);let yz=(if sb[71]{uo}else{x7});let yA=(if sb[71]{uq}else{xR});let yD=((yz*yA)).exp();let yE=((if xI{(xK*xM)}else{(if xO{xB}else{(if xG{wM}else{(if ws{wh}else{(if wi{vz}else{(if vb{uX}else{(if uY{u1}else{tf})})})})})})})-i);let yG=((if y6{(y8*ya)}else{(if yc{xT}else{(if y4{wU}else{(if y2{wy}else{(if xZ{vJ}else{(if xX{vj}else{(if xU{ub}else{tD})})})})})})})-i);let yM=((c2*yA)).exp();let yN=(yz-c2);let yP=(i+(yA*yN));let yR=(!(yz<c2));let yW=(if sb[71]{((sf[213]*((hC*yE)+(hJ*yG)))-(sf[214]*((if yR{(yM*yP)}else{yD})-kC)))}else{(if sb[66]{(sf[213]*((hC*ym)+(hJ*yu)))}else{(if sb[64]{(((hC*vW)+(hJ*vY))-(sf[5]*((if w7{(w2*w5)}else{vV})-kC)))}else{(if sb[61]{((hC*vC)+(hJ*vM))}else{g})})})});let yX=(i/hN);let yY=(ly<ex);let z1=((ly*yX)).exp();let z2=(!(((if yY{i}else{g}))!=0.0));let z5=(if (!(hQ>g)){g}else{ev});let z7=((yX*z5)).exp();let z8=(ly-z5);let za=(i+(yX*z8));let zd=(i/hU);let ze=(ly<eR);let zh=((ly*zd)).exp();let zi=(!(((if ze{i}else{g}))!=0.0));let zl=(if (!(hX>g)){g}else{eP});let zn=((zd*zl)).exp();let zo=(ly-zl);let zq=(i+(zd*zo));let zt=((if z2{(z7*za)}else{z1})-i);let zv=((if zi{(zn*zq)}else{zh})-i);let zx=((hQ*zt)+(hX*zv));let zy=(if ((sf[32])!=0.0){yX}else{zd});let zC=((lG*zy)).exp();let zE=((ex*yX)).exp();let zF=(ly-ex);let zH=(i+(yX*zF));let zJ=(!yY);let zM=(((sf[32])!=0.0)&&(!(((if (lG<(if sb[104]{g}else{eZ})){i}else{g}))!=0.0)));let zP=(if (!(hZ>g)){g}else{eZ});let zR=((zy*zP)).exp();let zS=(lG-zP);let zU=(i+(zy*zS));let zX=(sb[14]&&(lG<zP));let A0=(if ((sf[32])!=0.0){zd}else{zy});let A1=(lG<fb);let A4=((lG*A0)).exp();let A6=((eR*zd)).exp();let A7=(ly-eR);let A9=(i+(zd*A7));let Ab=(!ze);let Ae=(((sf[32])!=0.0)&&(!(((if A1{i}else{g}))!=0.0)));let Ah=(if (!(i1>g)){g}else{f9});let Aj=((A0*Ah)).exp();let Ak=(lG-Ah);let Am=(i+(A0*Ak));let Ap=(sb[14]&&(lG<Ah));let Aq=(if Ap{A4}else{(if Ab{(A6*A9)}else{zh})});let At=((fb*A0)).exp();let Au=(lG-fb);let Aw=(i+(A0*Au));let AA=((if zM{(zR*zU)}else{(if zX{zC}else{(if zJ{(zE*zH)}else{z1})})})-i);let AC=((if Ae{(Aj*Am)}else{Aq})-i);let AF=(if sb[72]{g}else{((hZ*AA)+(i1*AC))});let B5=(kR*lO);let B6=(i+B1);let B7=(i+B4);let B8=(B6/B7);let Bb=((B1-B4)-(B8).ln());let Bd=(lQ+(g9*Bb));let Be=(kU*Bd);let Bf=(lo*Be);let Bh=(sf[69]*(C*lo));let Bk=((aq+(lQ*lQ))).sqrt();let Bm=(i+(Bh*Bk));let Bn=(kU*Bm);let Bo=(Bf/Bn);let Br=((i+(Bo*Bo))).sqrt();let Bs=(Be/Br);let Bt=(kX*lR);let Bu=(lS*rx);let Bv=(l0*Bu);let Bw=(l3*lT);let Bx=(lU*t6);let By=(l6*Bx);let Bz=(l9*m1);let BA=0.02;let BC=(BA*(i+ip));let BH=(if ((sf[34])!=0.0){f64::powf(BC,sf[217])}else{g});let BJ=((jH-ly)-BH);let BM=((aq+(BJ*BJ))).sqrt();
        let BQ=(if ((sf[34])!=0.0){(BH+(C*(BJ+BM)))}else{g});let BR=(-ip);let BT=f64::powf(BQ,sf[218]);let BV=(if ((sf[34])!=0.0){(BR*BT)}else{g});let BX=(if (BV<sf[62]){i}else{g});let BY=(((sf[34])!=0.0)&&((BX)!=0.0));let BZ=(BV).exp();let C2=(((sf[34])!=0.0)&&(!((BX)!=0.0)));let C3=(if C2{sf[215]}else{g});let C7=(if C2{(C3*(i+(BV-sf[62])))}else{(if BY{BZ}else{g})});let C8=(sf[33]*BQ);let Ca=(if ((sf[34])!=0.0){(C7*C8)}else{g});let Cb=(m3-ry);let Cc=(Cb-zx);let Ch=(BA*(i+iu));let Cm=(if ((sf[36])!=0.0){f64::powf(Ch,sf[221])}else{g});let Co=((g-lD)-Cm);let Cr=((aq+(Co*Co))).sqrt();let Cv=(if ((sf[36])!=0.0){(Cm+(C*(Co+Cr)))}else{g});let Cw=(-iu);let Cy=f64::powf(Cv,sf[222]);let CA=(if ((sf[36])!=0.0){(Cw*Cy)}else{g});let CC=(if (CA<sf[62]){i}else{g});let CD=(((sf[36])!=0.0)&&((CC)!=0.0));let CE=(CA).exp();let CH=(((sf[36])!=0.0)&&(!((CC)!=0.0)));let CI=(if CH{sf[215]}else{g});let CM=(if CH{(CI*(i+(CA-sf[62])))}else{(if CD{CE}else{g})});let CN=(sf[35]*Cv);let CP=(if ((sf[36])!=0.0){(CM*CN)}else{Ca});let CQ=(-B5);let CY=0.1;let D0=(if sb[75]{((i-(ly/sf[40]))-CY)}else{g});let D3=((qO+(D0*D0))).sqrt();let Dc=(if sb[77]{sf[38]}else{(if sb[75]{(sf[38]*(if sb[75]{(CY+(C*(D0+D3)))}else{D0}))}else{g})});let De=((rz/Dc)-i);let Dl=((zx-(if sb[73]{g}else{(Ca*Cc)}))-(if sb[78]{g}else{(sf[37]*f64::powf(De,sf[223]))}));let Dn=(if ((sf[44])!=0.0){(i/i5)}else{A0});let Dr=((lX*Dn)).exp();let Dt=(((sf[44])!=0.0)&&(!(((if (lX<(if sb[108]{g}else{fs})){i}else{g}))!=0.0)));let Dw=(if (!(i8_>g)){g}else{fs});let Dy=((Dn*Dw)).exp();let Dz=(lX-Dw);let DB=(i+(Dn*Dz));let DD=(!AH);let DG=(sb[23]&&(lX<Dw));let DK=(if ((sf[44])!=0.0){(i/ic)}else{Dn});let DO=((lX*DK)).exp();let DP=(sb[14]&&(!A1));let DS=(((sf[44])!=0.0)&&(!(((if (lX<(if sb[110]{g}else{fL})){i}else{g}))!=0.0)));let DV=(if (!(if_>g)){g}else{fL});let DX=((DK*DV)).exp();let DY=(lX-DV);let E0=(i+(DK*DY));let E3=(sb[23]&&(lX<DV));let E7=((if Dt{(Dy*DB)}else{(if DG{Dr}else{(if DD{AO}else{AJ})})})-i);let E9=((if DS{(DX*E0)}else{(if E3{DO}else{(if DP{(At*Aw)}else{Aq})})})-i);let Ec=(if sb[79]{g}else{((i8_*E7)+(if_*E9))});let F3=(sf[60]*Bs);let F5=(sf[60]*t9);let J5=(if (qs>g){i}else{g});let J7=(sf[75]*(qs*J5));let J8=(i+J7);let J9=(J7/J8);let Jt=(sf[76]+(J9*J9));let Jw=(i+(J5*(Jr*Jt)));let Jx=(Jp*Jw);let JA=(qs*Jx);let Kx=(Ki*(sf[142]*f64::powf(ga,sf[244])));let KR=(Ki*(sf[147]*f64::powf(ga,sf[247])));let Mf=(sf[111]*Kh);let Mj=(hz*hz);let Mo=((hB*(sf[109]*(Ki*(sf[112]*f64::powf(ga,sf[255])))))+(hx*(hB*(((hz*(sf[114]*Lr))-(hy*Mf))/Mj))));let Mv=(sf[117]*Kh);let Mz=(hG*hG);let ME=((hI*(sf[115]*(Ki*(sf[118]*f64::powf(ga,sf[256])))))+(hE*(hI*(((hG*(sf[120]*Lr))-(hF*Mv))/Mz))));let MI=(Ki*(sf[123]*f64::powf(ga,sf[257])));let ML=(sf[122]*Kh);let MP=(hN*hN);let MR=(hP*(((hN*(sf[125]*Lr))-(hM*ML))/MP));let MY=(Ki*(sf[128]*f64::powf(ga,sf[258])));let N1=(sf[127]*Kh);let N5=(hU*hU);let N7=(hW*(((hU*(sf[130]*Lr))-(hT*N1))/N5));let Np=(sf[131]*Kh);let Nt=(i5*i5);let NF=(sf[135]*Kh);let NJ=(ic*ic);let NT=(sf[159]*(sf[160]*Kf));let NV=(sf[161]*(sf[162]*Kf));let QP=(-(sf[27]*((iy*Kf)+(gb*(sf[164]*Kf)))));let QS=((iF*Kh)+(g9*(sf[85]*(sf[165]*Kf))));let QW=(kA*kA);let QY=(kC*(((kA*QP)-(kz*QS))/QW));let Ra=(if kS{((-(if sb[43]{(sf[143]*Kx)}else{(if ((sf[8])!=0.0){(sf[143]*(Ki*(sf[144]*f64::powf(ga,sf[245]))))}else{g})}))/(gw*gw))}else{g});let RO=(if lm{((-(sf[182]*(Ki*(sf[183]*f64::powf(ga,sf[264])))))/(ky*ky))}else{g});let a1j=((qr*LC)+(hh*(if qj{((qo*(ql*(cB*a0Z)))+(ql*(qm*a0Z)))}else{a13})));let a1k=(hh*(if qj{(ql*a11)}else{a14}));let a1l=(hh*(if qj{(ql*a12)}else{a15}));let a1D=((qE*(qB*(d7*a1r)))+(qB*(qC*a1r)));let a1E=(qB*a1t);let a1F=(qB*a1u);let a1O=((qI*a1L)+(qH*(if qz{a1D}else{a1v})));let a1P=(qH*(if qz{a1E}else{a1w}));let a1Q=(qH*(if qz{a1F}else{a1x}));let a2F=(cj*(((qs*RK)+(ll*a1j))+(sf[64]*a1O)));let a2G=(cj*(sf[64]*a1P));let a2H=(cj*((ll*a1k)+(sf[64]*a1Q)));let a2I=(cj*(ll*a1l));let a2J=(a2B+a2F);let a2K=(a2C+a2G);let a2L=(a2D+a2H);let a2M=(a2E+a2I);let a2P=(sf[93]*f64::powf(r3,sf[286]));let a3f=(sf[93]*f64::powf(rj,sf[286]));
        let a3E=(if ru{a3A}else{(if rn{((rq*a36)+(ro*(a2F*a3f)))}else{(if rd{a36}else{(if r7{(C*(a2k+(a2J*a2P)))}else{g})})})});let a3F=(if ru{a3B}else{(if rn{((rq*a37)+(ro*(a2G*a3f)))}else{(if rd{a37}else{(if r7{(C*(a2l+(a2K*a2P)))}else{g})})})});let a3G=(if ru{a3C}else{(if rn{((rq*a38)+(ro*(a2H*a3f)))}else{(if rd{a38}else{(if r7{(C*(a2m+(a2L*a2P)))}else{g})})})});let a3H=(if ru{a3D}else{(if rn{((rq*a39)+(ro*(a2I*a3f)))}else{(if rd{a39}else{(if r7{(C*(a2n+(a2M*a2P)))}else{g})})})});let a3L=(rx*rx);let a3M=(((rx*a1O)-(qJ*a3E))/a3L);let a3Q=(((rx*a1P)-(qJ*a3F))/a3L);let a3U=(((rx*a1Q)-(qJ*a3G))/a3L);let a3X=((-(qJ*a3H))/a3L);let a41=(((rx*a1j)-(qs*a3E))/a3L);let a44=((-(qs*a3F))/a3L);let a48=(((rx*a1k)-(qs*a3G))/a3L);let a4c=(((rx*a1l)-(qs*a3H))/a3L);let a4q=(rL*(dx*a4f));let a4u=((rO*a4q)+(rL*(rM*a4f)));let a4v=(rL*a4h);let a4w=(rL*a4i);let a4x=(if rQ{a4j}else{(if rG{a1D}else{a1v})});let a4y=(if rQ{g}else{(if rG{a1E}else{a1w})});let a4z=(if rQ{a4k}else{g});let a4A=(if rQ{g}else{(if rG{a1F}else{a1x})});let a4B=(if rQ{a4l}else{g});let a4R=((s3*a4q)+(rL*(s1*a4f)));let a5f=(if ((sf[22])!=0.0){((sf_*M8)+(hv*((sf[210]*(if s7{a4u}else{a4j}))+(sf[211]*(if sb_{a4R}else{a4I})))))}else{g});let a5g=(if ((sf[22])!=0.0){(hv*(sf[211]*(if sb_{a4w}else{a4J})))}else{g});let a5h=(if ((sf[22])!=0.0){(hv*(sf[210]*(if s7{a4v}else{a4k})))}else{g});let a5i=(if ((sf[22])!=0.0){(hv*(sf[211]*(if sb_{a4v}else{a4K})))}else{g});let a5j=(if ((sf[22])!=0.0){(hv*(sf[210]*(if s7{a4w}else{a4l})))}else{g});let a5x=((sl*M8)+(hv*((sf[210]*(if rJ{a4u}else{a4x}))+(sf[211]*(if s0{a4R}else{(if rV{a4I}else{g})})))));let a5y=(hv*((sf[210]*(if rJ{g}else{a4y}))+(sf[211]*(if s0{a4w}else{(if rV{a4J}else{g})}))));let a5z=(hv*(sf[210]*(if rJ{a4v}else{a4z})));let a5A=(hv*((sf[210]*(if rJ{g}else{a4A}))+(sf[211]*(if s0{a4v}else{(if rV{a4K}else{g})}))));let a5B=(hv*(sf[210]*(if rJ{a4w}else{a4B})));let a6b=(iG*sA);let a6x=(sK*(lX*a4f));let a6y=(sK*a4i);let a6z=(sK*a4h);let a6I=((sP*a4q)+(rL*(sN*a4f)));let a70=(iG*sY);let a7A=(sH*sH);let a7T=(if sb[54]{g}else{(((sH*(a5f-(if ((sf[22])!=0.0){((sV*M8)+(hv*(if sT{a6I}else{a6x})))}else{g})))-(t7*(if sF{g}else{(if sx{(C*((cj*(if ((sf[22])!=0.0){(sf[66]*a5x)}else{g}))/a6b))}else{g})})))/a7A)});let a7U=(if sb[54]{g}else{(((sH*a5g)-(t7*(if sF{g}else{(if sx{(C*((cj*(if ((sf[22])!=0.0){(sf[66]*a5y)}else{g}))/a6b))}else{g})})))/a7A)});let a7V=(if sb[54]{g}else{(((sH*a5h)-(t7*(if sF{g}else{(if sx{(C*((cj*(if ((sf[22])!=0.0){(sf[66]*a5z)}else{g}))/a6b))}else{g})})))/a7A)});let a7W=(if sb[54]{g}else{(((sH*a5i)-(t7*(if sF{g}else{(if sx{(C*((cj*(if ((sf[22])!=0.0){(sf[66]*a5A)}else{g}))/a6b))}else{g})})))/a7A)});let a7X=(if sb[54]{g}else{(((sH*(a5j-(if ((sf[22])!=0.0){(hv*(if sT{a4w}else{a6y}))}else{g})))-(t7*(if sF{g}else{(if sx{(C*((cj*(if ((sf[22])!=0.0){(sf[66]*a5B)}else{g}))/a6b))}else{g})})))/a7A)});let a7Y=(if sb[54]{g}else{((-(if ((sf[22])!=0.0){(hv*(if sT{a4v}else{a6z}))}else{g}))/sH)});let a80=((-Mf)/Mj);let a81=(if ((sf[24])!=0.0){a80}else{a4f});let a83=(sf[60]*tb);let a84=(tb*sf[265]);let a85=(tf*(ls*a81));let a86=(tf*a83);let a87=(tf*a84);let a8m=(if tu{a85}else{(if tg{a6I}else{(if sR{a6x}else{(if sL{a4u}else{a4x})})})});let a8n=(if tu{g}else{(if tg{g}else{(if sR{g}else{(if sL{g}else{a4y})})})});let a8o=(if tu{g}else{(if tg{g}else{(if sR{g}else{(if sL{a4v}else{a4z})})})});let a8p=(if tu{a86}else{(if tg{g}else{(if sR{g}else{(if sL{g}else{a4A})})})});let a8q=(if tu{a87}else{g});let a8r=(if tu{g}else{(if tg{a4w}else{(if sR{a6y}else{(if sL{a4w}else{a4B})})})});let a8s=(if tu{g}else{(if tg{a4v}else{(if sR{a6z}else{g})})});let a8t=(if tj{((tr*(to*(tm*a81)))+(to*(tp*a81)))}else{a8m});let a8u=(if tj{g}else{a8n});let a8v=(if tj{g}else{a8o});let a8w=(if tj{(to*a83)}else{a8p});let a8x=(if tj{(to*a84)}else{a8q});let a8y=(if tj{g}else{a8r});let a8z=(if tj{g}else{a8s});let a8B=((-Mv)/Mz);let a8C=(if ((sf[24])!=0.0){a8B}else{a81});let a8E=(sf[60]*ty);let a8F=(ty*sf[265]);let a8G=(tD*(ls*a8C));let a8H=(tD*a8E);let a8I=(tD*a8F);let a8U=(if tG{((tO*(tL*(tJ*a8C)))+(tL*(tM*a8C)))}else{(if tB{a8G}else{g})});
        let a8V=(if tG{(tL*a8E)}else{(if tB{a8H}else{g})});let a8W=(if tG{(tL*a8F)}else{(if tB{a8I}else{g})});let a93=((tU*Mo)+(hC*(sf[25]*a2k)));let a94=(hC*(sf[25]*a2l));let a95=(hC*(sf[25]*a2m));let a96=(hC*(sf[25]*a2n));let a9c=((u0*(tX*(dS*a81)))+(tX*(tY*a81)));let a9d=(tX*a83);let a9e=(tX*a84);let a9f=(if u2{a9c}else{a85});let a9g=(if u2{a9d}else{a86});let a9h=(if u2{a9e}else{a87});let a9x=((ua*(u7*(ed*a8C)))+(u7*(u8_*a8C)));let a9y=(u7*a8E);let a9z=(u7*a8F);let a9F=((ue*ME)+(hJ*(if uc{a9x}else{a8G})));let a9G=(hJ*(if uc{a9y}else{a8H}));let a9H=(hJ*(if uc{a9z}else{a8I}));let aa1=(if sb[58]{QP}else{g});let aa5=((-QS)/QW);let aa6=(if sb[58]{aa5}else{a8C});let aa7=(ur*aa1);let aaa=(ur*sf[287]);let aab=(ur*sf[288]);let aaw=((ux*ME)+(hJ*a8U));let aax=(hJ*a8V);let aay=(hJ*a8W);let abs=(if sb[61]{a80}else{aa6});let abu=(sf[60]*uT);let abv=(uT*sf[265]);let abw=(uX*(lv*abs));let abx=(uX*abu);let aby=(uX*abv);let abO=(if vb{abw}else{(if uY{a9c}else{a8m})});let abP=(if vb{g}else{(if uY{g}else{a8n})});let abQ=(if vb{abx}else{(if uY{g}else{a8o})});let abR=(if vb{g}else{(if uY{a9d}else{a8p})});let abS=(if vb{aby}else{(if uY{a9e}else{a8q})});let abT=(if vb{g}else{(if uY{g}else{a8r})});let abU=(if vb{g}else{(if uY{g}else{a8s})});let ac2=(if sb[61]{a8B}else{abs});let ac4=(sf[60]*ve);let ac5=(ve*sf[265]);let ac6=(vj*(lv*ac2));let ac7=(vj*ac4);let ac8=(vj*ac5);let acl=(if vm{((vr*(vo*(tJ*ac2)))+(vo*(vp*ac2)))}else{(if vh{ac6}else{a8U})});let acm=(if vm{(vo*ac4)}else{(if vh{ac7}else{g})});let acn=(if vm{g}else{(if vh{g}else{a8V})});let aco=(if vm{(vo*ac5)}else{(if vh{ac8}else{a8W})});let acu=((vy*(vv*(dS*abs)))+(vv*(vw*abs)));let acv=(vv*abu);let acw=(vv*abv);let acK=((vI*(vF*(ed*ac2)))+(vF*(vG*ac2)));let acL=(vF*ac4);let acM=(vF*ac5);let ad1=(if sb[64]{QP}else{aa1});let ad4=(if sb[64]{aa5}else{ac2});let ad5=(vS*ad1);let ad8=(vS*sf[289]);let ad9=(vS*sf[290]);let adW=(if sb[66]{a80}else{ad4});let adY=(sf[60]*wf);let adZ=(wf*sf[265]);let ae0=(wh*(ls*adW));let ae1=(wh*adY);let ae2=(wh*adZ);let aep=(if wk{((wo*(wm*(tm*adW)))+(wm*(tp*adW)))}else{(if ws{ae0}else{(if wi{acu}else{abO})})});let aeq=(if wk{g}else{(if ws{g}else{(if wi{g}else{abP})})});let aer=(if wk{g}else{(if ws{g}else{(if wi{acv}else{abQ})})});let aes=(if wk{(wm*adY)}else{(if ws{ae1}else{(if wi{g}else{abR})})});let aet=(if wk{(wm*adZ)}else{(if ws{ae2}else{(if wi{acw}else{abS})})});let aeu=(if wk{g}else{(if ws{g}else{(if wi{g}else{abT})})});let aev=(if wk{g}else{(if ws{g}else{(if wi{g}else{abU})})});let aew=(if sb[66]{a8B}else{adW});let aey=(sf[60]*wv);let aez=(wv*sf[265]);let aeA=(wy*(ls*aew));let aeB=(wy*aey);let aeC=(wy*aez);let aeY=((wL*(wJ*(dS*adW)))+(wJ*(tY*adW)));let aeZ=(wJ*adY);let af0=(wJ*adZ);let af1=(if u2{aeY}else{ae0});let af2=(if u2{aeZ}else{ae1});let af3=(if u2{af0}else{ae2});let afj=((wT*(wR*(ed*aew)))+(wR*(u8_*aew)));let afk=(wR*aey);let afl=(wR*aez);let afr=((wW*ME)+(hJ*(if uc{afj}else{aeA})));let afs=(hJ*(if uc{afk}else{aeB}));let aft=(hJ*(if uc{afl}else{aeC}));let ag0=(if sb[71]{QP}else{ad1});let ag3=(if sb[71]{aa5}else{aew});let ag4=(x8*ag0);let ag7=(x8*sf[291]);let ag8=(x8*sf[292]);let agt=((xe*ME)+(hJ*(if wA{((wE*(wC*(tJ*aew)))+(wC*(tM*aew)))}else{(if ww{aeA}else{acl})})));let agu=(hJ*(if wA{g}else{(if ww{g}else{acm})}));let agv=(hJ*(if wA{(wC*aey)}else{(if ww{aeB}else{acn})}));let agw=(hJ*(if wA{(wC*aez)}else{(if ww{aeC}else{aco})}));let ahs=(if sb[71]{((if sb[59]{(sf[23]*(agt+((xc*Mo)+(hC*aep))))}else{(sf[23]*(((xc*a93)+(tV*aep))+agt))})-(sf[212]*((if xt{((xr*(xo*(c2*ag3)))+(xo*(ag4+(xp*ag3))))}else{(xb*(ag4+(x7*ag3)))})-QY)))}else{(if sb[70]{(sf[23]*(afr+((wO*Mo)+(hC*af1))))}else{(if sb[69]{(sf[23]*(((wO*a93)+(tV*af1))+afr))}else{(if sb[61]{g}else{(if sb[58]{((if sb[59]{(aaw+((uv*Mo)+(hC*a8t)))}else{(((uv*a93)+(tV*a8t))+aaw)})-(sf[5]*((if uK{((uI*(uF*(c2*aa6)))+(uF*(aa7+(uG*aa6))))}else{(uu*(aa7+(up*aa6)))})-QY)))}else{(if sb[57]{(a9F+((u4*Mo)+(hC*a9f)))}else{(if sb[55]{(((u4*a93)+(tV*a9f))+a9F)}else{g})})})})})})});
        let aht=(if sb[71]{(if sb[59]{(sf[23]*(hC*aeq))}else{(sf[23]*((xc*a94)+(tV*aeq)))})}else{(if sb[70]{g}else{(if sb[69]{(sf[23]*(wO*a94))}else{(if sb[61]{g}else{(if sb[58]{(if sb[59]{(hC*a8u)}else{((uv*a94)+(tV*a8u))})}else{(if sb[57]{g}else{(if sb[55]{(u4*a94)}else{g})})})})})})});let ahu=(if sb[71]{(if sb[59]{(sf[23]*(agu+(hC*aer)))}else{(sf[23]*((tV*aer)+agu))})}else{(if sb[70]{g}else{(if sb[69]{g}else{(if sb[61]{g}else{(if sb[58]{(if sb[59]{(hC*a8v)}else{(tV*a8v)})}else{g})})})})});let ahv=(if sb[71]{((if sb[59]{(sf[23]*(agv+(hC*aes)))}else{(sf[23]*(((xc*a95)+(tV*aes))+agv))})-(sf[212]*(if xt{(xo*ag7)}else{(xb*ag7)})))}else{(if sb[70]{(sf[23]*(afs+(hC*af2)))}else{(if sb[69]{(sf[23]*(((wO*a95)+(tV*af2))+afs))}else{(if sb[61]{g}else{(if sb[58]{((if sb[59]{(aax+(hC*a8w))}else{(((uv*a95)+(tV*a8w))+aax)})-(sf[5]*(if uK{(uF*aaa)}else{(uu*aaa)})))}else{(if sb[57]{(a9G+(hC*a9g))}else{(if sb[55]{(((u4*a95)+(tV*a9g))+a9G)}else{g})})})})})})});let ahw=(if sb[71]{((if sb[59]{(sf[23]*(agw+(hC*aet)))}else{(sf[23]*(((xc*a96)+(tV*aet))+agw))})-(sf[212]*(if xt{(xo*ag8)}else{(xb*ag8)})))}else{(if sb[70]{(sf[23]*(aft+(hC*af3)))}else{(if sb[69]{(sf[23]*(((wO*a96)+(tV*af3))+aft))}else{(if sb[61]{g}else{(if sb[58]{((if sb[59]{(aay+(hC*a8x))}else{(((uv*a96)+(tV*a8x))+aay)})-(sf[5]*(if uK{(uF*aab)}else{(uu*aab)})))}else{(if sb[57]{(a9H+(hC*a9h))}else{(if sb[55]{(((u4*a96)+(tV*a9h))+a9H)}else{g})})})})})})});let ahx=(if sb[71]{(if sb[59]{(sf[23]*(hC*aeu))}else{(sf[23]*(tV*aeu))})}else{(if sb[70]{g}else{(if sb[69]{g}else{(if sb[61]{g}else{(if sb[58]{(if sb[59]{(hC*a8y)}else{(tV*a8y)})}else{g})})})})});let ahy=(if sb[71]{(if sb[59]{(sf[23]*(hC*aev))}else{(sf[23]*(tV*aev))})}else{(if sb[70]{g}else{(if sb[69]{g}else{(if sb[61]{g}else{(if sb[58]{(if sb[59]{(hC*a8z)}else{(tV*a8z)})}else{g})})})})});let ahz=(if sb[66]{a80}else{ag3});let ahB=(sf[60]*xz);let ahC=(xz*sf[265]);let ahD=(xB*(lv*ahz));let ahE=(xB*ahB);let ahF=(xB*ahC);let aif=(if sb[66]{a8B}else{ahz});let aih=(sf[60]*xR);let aii=(xR*sf[265]);let aij=(xT*(lv*aif));let aik=(xT*aih);let ail=(xT*aii);let ajx=(if sb[66]{(sf[213]*(((ym*Mo)+(hC*(if vA{((yj*(yh*(dS*ahz)))+(yh*(vw*ahz)))}else{ahD})))+((yu*ME)+(hJ*(if vK{((yr*(yp*(ed*aif)))+(yp*(vG*aif)))}else{aij})))))}else{(if sb[64]{((((vW*Mo)+(hC*(if v1{((v6*(v3*(tm*abs)))+(v3*(v4*abs)))}else{abO})))+((vY*ME)+(hJ*acl)))-(sf[5]*((if w7{((w5*(w2*(c2*ad4)))+(w2*(ad5+(w3*ad4))))}else{(vV*(ad5+(vR*ad4)))})-QY)))}else{(if sb[61]{(((vC*Mo)+(hC*(if vA{acu}else{abw})))+((vM*ME)+(hJ*(if vK{acK}else{ac6}))))}else{g})})});let ajH=(if sb[71]{aa5}else{aif});let ajI=(yA*(if sb[71]{QP}else{ag0}));let ajL=(yA*sf[293]);let ajM=(yA*sf[294]);let akt=(if sb[71]{((sf[213]*(((yE*Mo)+(hC*(if xI{((xM*(xK*(tm*ahz)))+(xK*(v4*ahz)))}else{(if xO{ahD}else{(if xG{aeY}else{(if ws{ae0}else{(if wi{acu}else{(if vb{abw}else{(if uY{a9c}else{a85})})})})})})})))+((yG*ME)+(hJ*(if y6{((ya*(y8*(tJ*aif)))+(y8*(vp*aif)))}else{(if yc{aij}else{(if y4{afj}else{(if y2{aeA}else{(if xZ{acK}else{(if xX{ac6}else{(if xU{a9x}else{a8G})})})})})})})))))-(sf[214]*((if yR{((yP*(yM*(c2*ajH)))+(yM*(ajI+(yN*ajH))))}else{(yD*(ajI+(yz*ajH)))})-QY)))}else{ajx});let aku=(if sb[71]{g}else{(if sb[66]{g}else{(if sb[64]{(hC*(if v1{g}else{abP}))}else{g})})});let akv=(if sb[71]{(sf[213]*((hC*(if xI{(xK*ahB)}else{(if xO{ahE}else{(if xG{g}else{(if ws{g}else{(if wi{acv}else{(if vb{abx}else{g})})})})})}))+(hJ*(if y6{(y8*aih)}else{(if yc{aik}else{(if y4{g}else{(if y2{g}else{(if xZ{acL}else{(if xX{ac7}else{g})})})})})}))))}else{(if sb[66]{(sf[213]*((hC*(if vA{(yh*ahB)}else{ahE}))+(hJ*(if vK{(yp*aih)}else{aik}))))}else{(if sb[64]{((hC*(if v1{(v3*abu)}else{abQ}))+(hJ*acm))}else{(if sb[61]{((hC*(if vA{acv}else{abx}))+(hJ*(if vK{acL}else{ac7})))}else{g})})})});
        let akw=(if sb[71]{((sf[213]*((hC*(if xI{g}else{(if xO{g}else{(if xG{aeZ}else{(if ws{ae1}else{(if wi{g}else{(if vb{g}else{(if uY{a9d}else{a86})})})})})})}))+(hJ*(if y6{g}else{(if yc{g}else{(if y4{afk}else{(if y2{aeB}else{(if xZ{g}else{(if xX{g}else{(if xU{a9y}else{a8H})})})})})})}))))-(sf[214]*(if yR{(yM*ajL)}else{(yD*ajL)})))}else{(if sb[66]{g}else{(if sb[64]{(((hC*(if v1{g}else{abR}))+(hJ*acn))-(sf[5]*(if w7{(w2*ad8)}else{(vV*ad8)})))}else{g})})});let akx=(if sb[71]{((sf[213]*((hC*(if xI{(xK*ahC)}else{(if xO{ahF}else{(if xG{af0}else{(if ws{ae2}else{(if wi{acw}else{(if vb{aby}else{(if uY{a9e}else{a87})})})})})})}))+(hJ*(if y6{(y8*aii)}else{(if yc{ail}else{(if y4{afl}else{(if y2{aeC}else{(if xZ{acM}else{(if xX{ac8}else{(if xU{a9z}else{a8I})})})})})})}))))-(sf[214]*(if yR{(yM*ajM)}else{(yD*ajM)})))}else{(if sb[66]{(sf[213]*((hC*(if vA{(yh*ahC)}else{ahF}))+(hJ*(if vK{(yp*aii)}else{ail}))))}else{(if sb[64]{(((hC*(if v1{(v3*abv)}else{abS}))+(hJ*aco))-(sf[5]*(if w7{(w2*ad9)}else{(vV*ad9)})))}else{(if sb[61]{((hC*(if vA{acw}else{aby}))+(hJ*(if vK{acM}else{ac8})))}else{g})})})});let aky=(if sb[71]{g}else{(if sb[66]{g}else{(if sb[64]{(hC*(if v1{g}else{abT}))}else{g})})});let akz=(if sb[71]{g}else{(if sb[66]{g}else{(if sb[64]{(hC*(if v1{g}else{abU}))}else{g})})});let akB=((-ML)/MP);let akD=(yX*sf[265]);let akE=(sf[60]*yX);let akF=(z1*(ly*akB));let akG=(z1*akD);let akH=(z1*akE);let akU=((-N1)/N5);let akW=(zd*sf[265]);let akX=(sf[60]*zd);let akY=(zh*(ly*akU));let akZ=(zh*akW);let al0=(zh*akX);let alm=(((zt*((hP*(sf[121]*MI))+(hL*MR)))+(hQ*(if z2{((za*(z7*(z5*akB)))+(z7*(z8*akB)))}else{akF})))+((zv*((hW*(sf[126]*MY))+(hS*N7)))+(hX*(if zi{((zq*(zn*(zl*akU)))+(zn*(zo*akU)))}else{akY}))));let aln=((hQ*(if z2{(z7*akD)}else{akG}))+(hX*(if zi{(zn*akW)}else{akZ})));let alo=((hQ*(if z2{(z7*akE)}else{akH}))+(hX*(if zi{(zn*akX)}else{al0})));let alp=(if ((sf[32])!=0.0){akB}else{akU});let alr=(sf[60]*zy);let als=(zy*sf[265]);let alZ=(if ((sf[32])!=0.0){akU}else{alp});let am1=(sf[60]*A0);let am2=(A0*sf[265]);let amp=(if Ap{(A4*(lG*alZ))}else{(if Ab{((A9*(A6*(eR*akU)))+(A6*(A7*akU)))}else{akY})});let amq=(if Ap{g}else{(if Ab{(A6*akW)}else{akZ})});let amr=(if Ap{(A4*am1)}else{g});let ams=(if Ap{g}else{(if Ab{(A6*akX)}else{al0})});let amt=(if Ap{(A4*am2)}else{g});let an0=(if sb[72]{g}else{(((AA*((hY*MR)+(hP*(sf[30]*MI))))+(hZ*(if zM{((zU*(zR*(zP*alp)))+(zR*(zS*alp)))}else{(if zX{(zC*(lG*alp))}else{(if zJ{((zH*(zE*(ex*akB)))+(zE*(zF*akB)))}else{akF})})})))+((AC*((i0*N7)+(hW*(sf[31]*MY))))+(i1*(if Ae{((Am*(Aj*(Ah*alZ)))+(Aj*(Ak*alZ)))}else{amp}))))});let an1=(if sb[72]{g}else{((hZ*(if zM{g}else{(if zX{g}else{(if zJ{(zE*akD)}else{akG})})}))+(i1*(if Ae{g}else{amq})))});let an2=(if sb[72]{g}else{((hZ*(if zM{(zR*alr)}else{(if zX{(zC*alr)}else{g})}))+(i1*(if Ae{(Aj*am1)}else{amr})))});let an3=(if sb[72]{g}else{((hZ*(if zM{g}else{(if zX{g}else{(if zJ{(zE*akE)}else{akH})})}))+(i1*(if Ae{g}else{ams})))});let an4=(if sb[72]{g}else{((hZ*(if zM{(zR*als)}else{(if zX{(zC*als)}else{g})}))+(i1*(if Ae{(Aj*am2)}else{amt})))});let anL=(lO*(if kO{((-(if sb[42]{(sf[140]*Kx)}else{(if ((sf[7])!=0.0){(sf[140]*(Ki*(sf[141]*f64::powf(ga,sf[243]))))}else{g})}))/(go*go))}else{g}));let anM=(-kR);let anQ=(B7*B7);let aol=((Bd*Ra)+(kU*((Bb*Kh)+(g9*((anz-anI)-((((B7*anz)-(B6*anI))/anQ)/B8))))));let aom=(kU*(sf[60]+(g9*((-anJ)-(((-(B6*anJ))/anQ)/B8)))));let aon=(kU*(sf[265]+(g9*(anA-((anA/B7)/B8)))));let aoo=(kU*(g9*((anB-anK)-((((B7*anB)-(B6*anK))/anQ)/B8))));let aox=(sf[60]*lQ);let aoz=(lQ*sf[265]);let aoB=(iG*Bk);let aoP=(Bn*Bn);let ap0=(Bo*(((Bn*((Be*RO)+(lo*aol)))-(Bf*((Bm*Ra)+(kU*(Bk*(sf[69]*(C*RO)))))))/aoP));let ap2=(Bo*(((Bn*(lo*aom))-(Bf*(kU*(Bh*((aox+aox)/aoB)))))/aoP));let ap4=(Bo*(((Bn*(lo*aon))-(Bf*(kU*(Bh*((aoz+aoz)/aoB)))))/aoP));let ap6=(Bo*((lo*aoo)/Bn));let ap8=(iG*Br);let apg=(Br*Br);let aph=(((Br*aol)-(Be*((ap0+ap0)/ap8)))/apg);let apl=(((Br*aom)-(Be*((ap2+ap2)/ap8)))/apg);let app=(((Br*aon)-(Be*((ap4+ap4)/ap8)))/apg);let apt=(((Br*aoo)-(Be*((ap6+ap6)/ap8)))/apg);
        let apu=(lR*(if kV{((-(if sb[44]{(sf[145]*KR)}else{(if ((sf[9])!=0.0){(sf[145]*(Ki*(sf[146]*f64::powf(ga,sf[246]))))}else{g})}))/(gG*gG))}else{g}));let apv=(-kX);let apE=((Bu*(if kY{((-(if sb[45]{(sf[148]*KR)}else{(if ((sf[10])!=0.0){(sf[148]*(Ki*(sf[149]*f64::powf(ga,sf[248]))))}else{g})}))/(gO*gO))}else{g}))+(l0*(lS*a3E)));let apF=(l0*(lS*a3F));let apG=(l0*rx);let apH=(l0*((-rx)+(lS*a3G)));let apI=(l0*(lS*a3H));let apJ=(lT*(if l1{((-(sf[150]*(Ki*(sf[151]*f64::powf(ga,sf[249])))))/(gS*gS))}else{g}));let apK=(-l3);let apV=((Bx*(if l4{((-(if sb[46]{(sf[154]*Kx)}else{(if ((sf[11])!=0.0){(sf[154]*(Ki*(sf[155]*f64::powf(ga,sf[251]))))}else{g})}))/(h4*h4))}else{g}))+(l6*(lU*(if sb[54]{g}else{(if t2{g}else{(C*((if ((sf[22])!=0.0){(cj*(sf[66]*a5f))}else{(if sb[53]{a2F}else{a2J})})/a70))})}))));let apW=(l6*(-t6));let apX=(l6*(lU*(if sb[54]{g}else{(if t2{g}else{(C*((if ((sf[22])!=0.0){(cj*(sf[66]*a5g))}else{(if sb[53]{a2G}else{a2K})})/a70))})})));let apY=(l6*(lU*(if sb[54]{g}else{(if t2{g}else{(C*((if ((sf[22])!=0.0){(cj*(sf[66]*a5h))}else{g})/a70))})})));let apZ=(l6*(lU*(if sb[54]{g}else{(if t2{g}else{(C*((if ((sf[22])!=0.0){(cj*(sf[66]*a5i))}else{(if sb[53]{a2H}else{a2L})})/a70))})})));let aq0=(l6*(lU*(if sb[54]{g}else{(if t2{g}else{(C*((if ((sf[22])!=0.0){g}else{(if sb[53]{a2I}else{a2M})})/a70))})})));let aq1=(l6*(t6+(lU*(if sb[54]{g}else{(if t2{g}else{(C*((if ((sf[22])!=0.0){(cj*(sf[66]*a5j))}else{g})/a70))})}))));let aq2=(m1*(if l7{((-(sf[152]*(Ki*(sf[153]*f64::powf(ga,sf[250])))))/(gW*gW))}else{g}));let aq3=(-l9);let aq9=(if ((sf[34])!=0.0){((BA*NT)*(sf[217]*f64::powf(BC,sf[295])))}else{g});let aqa=(Pt-aq9);let aqb=(BJ*aqa);let aqd=(sf[60]*BJ);let aqf=(BJ*sf[265]);let aqh=(iG*BM);let aqs=(if ((sf[34])!=0.0){(aq9+(C*(aqa+((aqb+aqb)/aqh))))}else{g});let aqt=(if ((sf[34])!=0.0){(C*(sf[60]+((aqd+aqd)/aqh)))}else{g});let aqu=(if ((sf[34])!=0.0){(C*(sf[265]+((aqf+aqf)/aqh)))}else{g});let aqy=(sf[218]*f64::powf(BQ,sf[296]));let aqH=(if ((sf[34])!=0.0){((BT*(-NT))+(BR*(aqs*aqy)))}else{g});let aqI=(if ((sf[34])!=0.0){(BR*(aqt*aqy))}else{g});let aqJ=(if ((sf[34])!=0.0){(BR*(aqu*aqy))}else{g});let ar8=(if ((sf[34])!=0.0){((C8*(if C2{(C3*aqH)}else{(if BY{(BZ*aqH)}else{g})}))+(C7*(sf[33]*aqs)))}else{g});let ar9=(if ((sf[34])!=0.0){((C8*(if C2{(C3*aqI)}else{(if BY{(BZ*aqI)}else{g})}))+(C7*(sf[33]*aqt)))}else{g});let ara=(if ((sf[34])!=0.0){((C8*(if C2{(C3*aqJ)}else{(if BY{(BZ*aqJ)}else{g})}))+(C7*(sf[33]*aqu)))}else{g});let arb=(-a3M);let arc=(-a3Q);let ard=(-a3U);let are=(-a3X);let arC=(if ((sf[36])!=0.0){((BA*NV)*(sf[221]*f64::powf(Ch,sf[297])))}else{g});let arD=(-arC);let arE=(Co*arD);let arG=(sf[60]*Co);let arI=(Co*sf[265]);let arK=(iG*Cr);let arV=(if ((sf[36])!=0.0){(arC+(C*(arD+((arE+arE)/arK))))}else{g});let arW=(if ((sf[36])!=0.0){(C*(sf[60]+((arG+arG)/arK)))}else{g});let arX=(if ((sf[36])!=0.0){(C*(sf[265]+((arI+arI)/arK)))}else{g});let as1=(sf[222]*f64::powf(Cv,sf[298]));let asa=(if ((sf[36])!=0.0){((Cy*(-NV))+(Cw*(arV*as1)))}else{g});let asb=(if ((sf[36])!=0.0){(Cw*(arW*as1))}else{g});let asc=(if ((sf[36])!=0.0){(Cw*(arX*as1))}else{g});let at9=(D0*sf[303]);let atb=(D0*sf[304]);let atd=(iG*D3);let atw=(Dc*Dc);let atF=(sf[223]*f64::powf(De,sf[305]));let atW=(-(if sb[73]{g}else{Ca}));let atX=((alm-(if sb[73]{g}else{((Cc*ar8)+(Ca*(arb-alm)))}))-(if sb[78]{g}else{(sf[37]*((a41/Dc)*atF))}));let atY=((aln-(if sb[73]{g}else{((Cc*ar9)+(Ca*(arc-aln)))}))-(if sb[78]{g}else{(sf[37]*((((Dc*a44)-(rz*(if sb[77]{g}else{(if sb[75]{(sf[38]*(if sb[75]{(C*(sf[303]+((at9+at9)/atd)))}else{sf[303]}))}else{g})})))/atw)*atF))}));let atZ=((alo-(if sb[73]{g}else{((Cc*ara)+(Ca*(ard-alo)))}))-(if sb[78]{g}else{(sf[37]*((((Dc*a48)-(rz*(if sb[77]{g}else{(if sb[75]{(sf[38]*(if sb[75]{(C*(sf[304]+((atb+atb)/atd)))}else{sf[304]}))}else{g})})))/atw)*atF))}));let au0=((-(if sb[73]{g}else{(Ca*are)}))-(if sb[78]{g}else{(sf[37]*((a4c/Dc)*atF))}));let au3=(if ((sf[44])!=0.0){((-Np)/Nt)}else{alZ});let au5=(Dn*sf[265]);let au6=(sf[60]*Dn);let aux=(if ((sf[44])!=0.0){((-NF)/NJ)}else{au3});let auz=(DK*sf[265]);let auA=(sf[60]*DK);
        let avi=(((E7*((i7*(sf[42]*(Ki*(sf[132]*f64::powf(ga,sf[259])))))+(i3*(i7*(((i5*(sf[134]*Lr))-(i4*Np))/Nt)))))+(i8_*(if Dt{((DB*(Dy*(Dw*au3)))+(Dy*(Dz*au3)))}else{(if DG{(Dr*(lX*au3))}else{(if DD{and}else{ana})})})))+((E9*((ie*(sf[43]*(Ki*(sf[136]*f64::powf(ga,sf[260])))))+(ia*(ie*(((ic*(sf[138]*Lr))-(ib*NF))/NJ)))))+(if_*(if DS{((E0*(DX*(DV*aux)))+(DX*(DY*aux)))}else{(if E3{(DO*(lX*aux))}else{(if DP{((Aw*(At*(fb*alZ)))+(At*(Au*alZ)))}else{amp})})}))));let avn=(if sb[79]{g}else{avi});let avo=(if sb[79]{g}else{((i8_*(if Dt{g}else{(if DG{g}else{(if DD{ane}else{anb})})}))+(if_*(if DS{g}else{(if E3{g}else{(if DP{g}else{amq})})})))});let avp=(if sb[79]{g}else{(if_*(if DS{g}else{(if E3{g}else{(if DP{(At*am1)}else{amr})})}))});let avq=(if sb[79]{g}else{((i8_*(if Dt{g}else{(if DG{g}else{(if DD{anf}else{anc})})}))+(if_*(if DS{g}else{(if E3{g}else{(if DP{g}else{ams})})})))});let avr=(if sb[79]{g}else{((i8_*(if Dt{(Dy*au5)}else{(if DG{(Dr*au5)}else{g})}))+(if_*(if DS{(DX*auz)}else{(if E3{(DO*auz)}else{(if DP{(At*am2)}else{amt})})})))});let avs=(if sb[79]{g}else{((i8_*(if Dt{(Dy*au6)}else{(if DG{(Dr*au6)}else{g})}))+(if_*(if DS{(DX*auA)}else{(if E3{(DO*auA)}else{g})})))});let ayt=(aw*sf[60]);let ayu=(aw*sf[265]);let aIO=(sf[75]*(J5*a1j));let aIP=(sf[75]*(J5*a1k));let aIQ=(sf[75]*(J5*a1l));let aIU=(J8*J8);let aJo=(J9*(((J8*aIO)-(J7*aIO))/aIU));let aJq=(J9*(((J8*aIP)-(J7*aIP))/aIU));let aJs=(J9*(((J8*aIQ)-(J7*aIQ))/aIU));

        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(9),
            multiplicity * ((sf[60]*(xy+(aw*ls)))),
            [4, 6, 7, 8, 9, 10, 11],
            [(sf[60]*ahs), (sf[60]*aht), (sf[60]*ahu), (sf[60]*(ahv+ayt)), (sf[60]*(ahw+ayu)), (sf[60]*ahx), (sf[60]*ahy)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * ((sf[60]*(yW+(aw*lv)))),
            [4, 6, 7, 8, 9, 10, 11],
            [(sf[60]*akt), (sf[60]*aku), (sf[60]*(akv+ayt)), (sf[60]*akw), (sf[60]*(akx+ayu)), (sf[60]*aky), (sf[60]*akz)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(6),
            Some(9),
            multiplicity * ((sf[60]*m3)),
            13,
            multiplicity * (sf[60]),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(9),
            Some(6),
            multiplicity * ((sf[60]*ry)),
            [4, 6, 8, 9],
            [(sf[60]*a3M), (sf[60]*a3Q), (sf[60]*a3U), (sf[60]*a3X)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * ((sf[60]*(Dl+(aw*ly)))),
            [4, 6, 8, 9, 13],
            [(sf[60]*atX), (sf[60]*(atY+ayu)), (sf[60]*(atZ+ayt)), (sf[60]*au0), (sf[60]*atW)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * ((sf[60]*((if sb[74]{g}else{(if ((sf[36])!=0.0){(CP*CQ)}else{g})})+(aw*lD)))),
            [0, 4, 5, 6, 7, 8],
            [(sf[60]*(if sb[74]{g}else{(if ((sf[36])!=0.0){(CP*anM)}else{g})})), (sf[60]*(if sb[74]{g}else{(if ((sf[36])!=0.0){((CQ*(if ((sf[36])!=0.0){((CN*(if CH{(CI*asa)}else{(if CD{(CE*asa)}else{g})}))+(CM*(sf[35]*arV)))}else{ar8}))+(CP*(-anL)))}else{g})})), (sf[60]*((if sb[74]{g}else{(if ((sf[36])!=0.0){((CQ*(if ((sf[36])!=0.0){((CN*(if CH{(CI*asb)}else{(if CD{(CE*asb)}else{g})}))+(CM*(sf[35]*arW)))}else{g}))+(kR*CP))}else{g})})+ayu)), (sf[60]*(if sb[74]{g}else{(if ((sf[36])!=0.0){(CQ*(if ((sf[36])!=0.0){g}else{ar9}))}else{g})})), (sf[60]*((if sb[74]{g}else{(if ((sf[36])!=0.0){(CQ*(if ((sf[36])!=0.0){((CN*(if CH{(CI*asc)}else{(if CD{(CE*asc)}else{g})}))+(CM*(sf[35]*arX)))}else{g}))}else{g})})+ayt)), (sf[60]*(if sb[74]{g}else{(if ((sf[36])!=0.0){(CQ*(if ((sf[36])!=0.0){g}else{ara}))}else{g})}))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * ((sf[60]*(AF+(aw*lG)))),
            [4, 6, 7, 8, 10],
            [(sf[60]*an0), (sf[60]*an1), (sf[60]*(an2+ayt)), (sf[60]*an3), (sf[60]*(an4+ayu))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(5),
            multiplicity * (B5),
            0,
            multiplicity * (kR),
            4,
            multiplicity * (anL),
            5,
            multiplicity * (anM),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (F3),
            [4, 5, 6, 8],
            [(sf[60]*aph), (sf[60]*apl), (sf[60]*app), (sf[60]*apt)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(7),
            multiplicity * (Bt),
            1,
            multiplicity * (kX),
            4,
            multiplicity * (apu),
            7,
            multiplicity * (apv),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (Bv),
            [4, 6, 7, 8, 9],
            [apE, apF, apG, apH, apI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(9),
            multiplicity * (Bw),
            2,
            multiplicity * (l3),
            4,
            multiplicity * (apJ),
            9,
            multiplicity * (apK),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(10),
            Some(5),
            multiplicity * (By),
            [4, 5, 6, 7, 8, 9, 10],
            [apV, apW, apX, apY, apZ, aq0, aq1],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(10),
            multiplicity * ((sf[60]*(Ec+(aw*lX)))),
            [4, 6, 7, 8, 10, 11],
            [(sf[60]*avn), (sf[60]*avo), (sf[60]*avp), (sf[60]*avq), (sf[60]*(avr+ayu)), (sf[60]*(avs+ayt))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(11),
            multiplicity * (F5),
            [4, 6, 7, 8, 10, 11],
            [(sf[60]*a7T), (sf[60]*a7U), (sf[60]*a7V), (sf[60]*a7W), (sf[60]*a7X), (sf[60]*a7Y)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(3),
            Some(11),
            multiplicity * (Bz),
            3,
            multiplicity * (l9),
            4,
            multiplicity * (aq2),
            11,
            multiplicity * (aq3),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(12),
            None,
            multiplicity * ((m3-rz)),
            [4, 6, 8, 9, 13],
            [(-a41), (-a44), (-a48), (-a4c), i],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(13),
            None,
            multiplicity * ((m3-m2)),
            12,
            multiplicity * (-1.0),
            13,
            multiplicity * (i),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((fO*lc)),
            4,
            multiplicity * ((lc+(fO*(if la{((-(sf[156]*(sf[157]*Kf)))/(h9*h9))}else{g})))),
        );
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * ((((((((((((((((ls*xy)+(ly*Dl))+(lL*Cb))+(lv*yW))+(lG*AF))+(m1*Bz))+(lX*Ec))+(lZ*t9))+(lO*B5))+(lQ*Bs))+(lR*Bt))+(lS*Bv))+(lT*Bw))+(lU*By))*sf[225])),
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13],
            &[(sf[225]*(B5+B5)), (sf[225]*(Bt+Bt)), (sf[225]*(Bw+Bw)), (sf[225]*(Bz+Bz)), (sf[225]*((((((((((((((ls*ahs)+(ly*atX))+(lL*arb))+(lv*akt))+(lG*an0))+(m1*aq2))+(lX*avn))+(lZ*a7T))+(lO*anL))+(lQ*aph))+(lR*apu))+(lS*apE))+(lT*apJ))+(lU*apV))), (sf[225]*(((CQ+(lO*anM))+(F3+(lQ*apl)))+((-By)+(lU*apW)))), (sf[225]*((((((((((ls*aht)+((Dl*sf[265])+(ly*atY)))+((sf[60]*Cb)+(lL*arc)))+(lv*aku))+(lG*an1))+(lX*avo))+(lZ*a7U))+((Bs*sf[265])+(lQ*app)))+(lS*apF))+(lU*apX))), (sf[225]*((((((((ls*ahu)+((sf[60]*yW)+(lv*akv)))+((sf[60]*AF)+(lG*an2)))+(lX*avp))+(F5+(lZ*a7V)))+((-Bt)+(lR*apv)))+(Bv+(lS*apG)))+(lU*apY))), (sf[225]*(((((((((((sf[60]*xy)+(ls*ahv))+((sf[60]*Dl)+(ly*atZ)))+(lL*ard))+(lv*akw))+(lG*an3))+(lX*avq))+(lZ*a7W))+(lQ*apt))+((-Bv)+(lS*apH)))+(lU*apZ))), (sf[225]*((((((((xy*sf[265])+(ls*ahw))+(ly*au0))+((Cb*sf[265])+(lL*are)))+((yW*sf[265])+(lv*akx)))+(lS*apI))+((-Bw)+(lT*apK)))+(lU*aq0))), (sf[225]*((((((ls*ahx)+(lv*aky))+((AF*sf[265])+(lG*an4)))+((Ec*sf[265])+(lX*avr)))+(lZ*a7X))+(By+(lU*aq1)))), (sf[225]*(((((ls*ahy)+(lv*akz))+((-Bz)+(m1*aq3)))+((sf[60]*Ec)+(lX*avs)))+((t9*sf[265])+(lZ*a7Y)))), (sf[225]*(lL+(ly*atW)))],
            &[],
            &[],
            multiplicity,
        );
        let K5_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (sf[60]*(Jz+(JA/rx))));
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(9),
            multiplicity * (K5_ddt),
            [4, 6, 8, 9],
            [(((sf[60]*(aJV+(((rx*((Jx*a1j)+(qs*((Jw*aJi)+(Jp*(J5*(Jr*(aJo+aJo))))))))-(JA*a3E))/a3L)))) * ddt_scale), (((sf[60]*(((rx*(qs*((Jw*aJj)+(Jp*(J5*(Jt*aJm))))))-(JA*a3F))/a3L))) * ddt_scale), (((sf[60]*(aJW+(((rx*((Jx*a1k)+(qs*((Jw*aJk)+(Jp*(J5*((Jt*aJn)+(Jr*(aJq+aJq)))))))))-(JA*a3G))/a3L)))) * ddt_scale), (((sf[60]*(aJX+(((rx*((Jx*a1l)+(qs*((Jw*aJl)+(Jp*(J5*(Jr*(aJs+aJs))))))))-(JA*a3H))/a3L)))) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let K6_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, K6);
        stamper.stamp_current_node3_local(
            Some(7),
            Some(9),
            multiplicity * (K6_ddt),
            4,
            multiplicity * (((aLm) * ddt_scale)),
            7,
            multiplicity * (((aLn) * ddt_scale)),
            9,
            multiplicity * (((aLo) * ddt_scale)),
        );
        let K7_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (sf[60]*((JF+(qJ*sf[235]))+JK)));
        stamper.stamp_current_node3_local(
            Some(8),
            Some(6),
            multiplicity * (K7_ddt),
            4,
            multiplicity * ((((sf[60]*((aKB+(sf[235]*a1O))+aKK))) * ddt_scale)),
            6,
            multiplicity * ((((sf[60]*((aKC+(sf[235]*a1P))+aKL))) * ddt_scale)),
            8,
            multiplicity * ((((sf[60]*((aKD+(sf[235]*a1Q))+aKM))) * ddt_scale)),
        );
        let K8_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, K8);
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * (K8_ddt),
            4,
            multiplicity * (((aLs) * ddt_scale)),
            5,
            multiplicity * (((aLt) * ddt_scale)),
            8,
            multiplicity * (((aLu) * ddt_scale)),
        );
        let K9_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (sf[60]*(JN+((if sb[54]{g}else{sm})*sf[235]))));
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * (K9_ddt),
            [4, 6, 7, 8, 10],
            [(((sf[60]*(aKV+(sf[235]*(if sb[54]{g}else{a5x}))))) * ddt_scale), (((sf[60]*(sf[235]*(if sb[54]{g}else{a5y})))) * ddt_scale), (((sf[60]*(aKW+(sf[235]*(if sb[54]{g}else{a5z}))))) * ddt_scale), (((sf[60]*(sf[235]*(if sb[54]{g}else{a5A})))) * ddt_scale), (((sf[60]*(aKX+(sf[235]*(if sb[54]{g}else{a5B}))))) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let JV_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, JV);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (JV_ddt),
            1,
            multiplicity * (((sf[238]) * ddt_scale)),
            2,
            multiplicity * (((sf[321]) * ddt_scale)),
        );
        let JX_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, JX);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (JX_ddt),
            0,
            multiplicity * (((sf[322]) * ddt_scale)),
            1,
            multiplicity * (((sf[239]) * ddt_scale)),
        );
        let Ka_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, Ka);
        stamper.stamp_current_node3_local(
            Some(11),
            Some(10),
            multiplicity * (Ka_ddt),
            4,
            multiplicity * (((aLA) * ddt_scale)),
            10,
            multiplicity * (((aLB) * ddt_scale)),
            11,
            multiplicity * (((aLC) * ddt_scale)),
        );
        let K1_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, K1);
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (K1_ddt),
            12,
            multiplicity * (((sf[241]) * ddt_scale)),
        );
        let K4_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, K4);
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (K4_ddt),
            13,
            multiplicity * (((sf[323]) * ddt_scale)),
        );
        let JZ_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, JZ);
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (JZ_ddt),
            4,
            multiplicity * (((sf[240]) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(9),
            multiplicity * (g),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(9),
            multiplicity * (g),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (g),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (g),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(9),
            multiplicity * (g),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(10),
            multiplicity * (g),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(10),
            multiplicity * (g),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(5),
            multiplicity * (g),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (g),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(7),
            multiplicity * (g),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (g),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(9),
            multiplicity * (g),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (g),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(11),
            multiplicity * (g),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(11),
            multiplicity * (g),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        let br=self.branches;
        let branches=br;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let CommonStampValues {
            g, i, C, cj, fO, g9, ga, gb,
            hc, hh, hv, iG, jH, ll, lp, lq,
            ls, lt, lv, lw, ly, lz, lE, lG,
            lH, lI, lM, lV, lX, m2, m3, qe,
            qi, qu, qy, qH, qO, qR, qW, r1,
            rg, ro, rw, rB, rF, rX, AH, AJ,
            AO, B1, B4, Jp, Jr, Jz, JF, JK,
            JN, JV, JX, JZ, K1, K4, K6, K8,
            Ka, Kf, Kh, Ki, Lr, LC, M8, Pt,
            RK, a0Z, a11, a12, a13, a14, a15, a1r,
            a1t, a1u, a1v, a1w, a1x, a1L, a2k, a2l,
            a2m, a2n, a2B, a2C, a2D, a2E, a36, a37,
            a38, a39, a3A, a3B, a3C, a3D, a4f, a4h,
            a4i, a4j, a4k, a4l, a4I, a4J, a4K, ana,
            anb, anc, and, ane, anf, anz, anA, anB,
            anI, anJ, anK, aJi, aJj, aJk, aJl, aJm,
            aJn, aJV, aJW, aJX, aKB, aKC, aKD, aKK,
            aKL, aKM, aKV, aKW, aKX, aLm, aLn, aLo,
            aLs, aLt, aLu, aLA, aLB, aLC,
        }=self.eval_common_stamp_values(ctx);
        let p=&(*self.params);
        let m=self.multiplicity;
        let multiplicity=m;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let az=ctx.simparam_or("pnjmaxi", i);let aB=(if sb[27]{az}else{sf[52]});let ci=(C*aB);let cB=(if sb[89]{g}else{(if (sb[28]&&(aB>sf[53])){(sf[349]*((i+(f64::powf((ci*sf[94]),sf[96])/sf[352]))).ln())}else{(sf[349]*((i+(aB/sf[352]))).ln())})});let d7=(if sb[92]{g}else{(if (sb[29]&&(aB>sf[54])){(sf[357]*((i+(f64::powf((ci*sf[104]),sf[96])/sf[362]))).ln())}else{(sf[357]*((i+(aB/sf[362]))).ln())})});let dx=(if sb[94]{g}else{(if (sb[30]&&(aB>sf[55])){(sf[366]*((i+((sf[66]*(aB*aB))/sf[369]))).ln())}else{(sf[366]*((i+(aB/sf[369]))).ln())})});let qj=(!(((if (ls<cB){i}else{g}))!=0.0));let ql=((cB*qe)).exp();let qm=(ls-cB);let qo=(i+(qe*qm));let qr=((if qj{(ql*qo)}else{qi})-i);let qs=(hh*qr);let qv=(ly<d7);let qz=(!(((if qv{i}else{g}))!=0.0));let qB=((d7*qu)).exp();let qC=(ly-d7);let qE=(i+(qu*qC));let qF=(qB*qE);let qI=((if qz{qF}else{qy})-i);let qJ=(qH*qI);let r2=(cj*((ll*qs)+(sf[64]*qJ)));let r3=(r1+r2);let r6=(if ((if ((sf[20])!=0.0){r3}else{g})>qR){i}else{g});let r7=(((sf[20])!=0.0)&&((r6)!=0.0));let rd=(((sf[20])!=0.0)&&(!((r6)!=0.0)));let rj=(i+r2);let rm=(if ((if sb[52]{rj}else{r3})>qR){i}else{g});let rn=(sb[52]&&((rm)!=0.0));let rq=(i+f64::powf(rj,sf[93]));let ru=(sb[52]&&(!((rm)!=0.0)));let rx=(if ru{rw}else{(if rn{(ro*rq)}else{(if rd{rg}else{(if r7{(C*(qW+f64::powf(r3,sf[93])))}else{g})})})});let rC=(lG<dx);let rG=(!qv);let rJ=(((sf[22])!=0.0)&&(!(((if rC{i}else{g}))!=0.0)));let rL=((dx*rB)).exp();let rM=(lG-dx);let rO=(i+(rB*rM));let rQ=(sb[7]&&rC);let rU=(if (ly<dx){i}else{g});let rV=(((sf[22])!=0.0)&&((rU)!=0.0));let s0=(((sf[22])!=0.0)&&(!((rU)!=0.0)));let s1=(ly-dx);let s3=(i+(rB*s1));let sl=((((if rJ{(rL*rO)}else{(if rQ{rF}else{(if rG{qF}else{qy})})})*sf[210])+((if s0{(rL*s3)}else{(if rV{rX}else{g})})*sf[211]))-i);let J5=(if (qs>g){i}else{g});let J7=(sf[75]*(qs*J5));let J8=(i+J7);let J9=(J7/J8);let Jt=(sf[76]+(J9*J9));let Jw=(i+(J5*(Jr*Jt)));let Jx=(Jp*Jw);let JA=(qs*Jx);let a1j=((qr*LC)+(hh*(if qj{((qo*(ql*(cB*a0Z)))+(ql*(qm*a0Z)))}else{a13})));let a1k=(hh*(if qj{(ql*a11)}else{a14}));let a1l=(hh*(if qj{(ql*a12)}else{a15}));let a1D=((qE*(qB*(d7*a1r)))+(qB*(qC*a1r)));let a1E=(qB*a1t);let a1F=(qB*a1u);let a1O=((qI*a1L)+(qH*(if qz{a1D}else{a1v})));let a1P=(qH*(if qz{a1E}else{a1w}));let a1Q=(qH*(if qz{a1F}else{a1x}));let a2F=(cj*(((qs*RK)+(ll*a1j))+(sf[64]*a1O)));let a2G=(cj*(sf[64]*a1P));let a2H=(cj*((ll*a1k)+(sf[64]*a1Q)));let a2I=(cj*(ll*a1l));let a2P=(sf[93]*f64::powf(r3,sf[286]));let a3f=(sf[93]*f64::powf(rj,sf[286]));let a3L=(rx*rx);let a4q=(rL*(dx*a4f));let a4v=(rL*a4h);let a4w=(rL*a4i);let aIO=(sf[75]*(J5*a1j));let aIP=(sf[75]*(J5*a1k));let aIQ=(sf[75]*(J5*a1l));let aIU=(J8*J8);let aJo=(J9*(((J8*aIO)-(J7*aIO))/aIU));let aJq=(J9*(((J8*aIP)-(J7*aIP))/aIU));let aJs=(J9*(((J8*aIQ)-(J7*aIQ))/aIU));

        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(9),
            &[4, 6, 8, 9],
            &[(sf[60]*(aJV+(((rx*((Jx*a1j)+(qs*((Jw*aJi)+(Jp*(J5*(Jr*(aJo+aJo))))))))-(JA*(if ru{a3A}else{(if rn{((rq*a36)+(ro*(a2F*a3f)))}else{(if rd{a36}else{(if r7{(C*(a2k+((a2B+a2F)*a2P)))}else{g})})})})))/a3L))), (sf[60]*(((rx*(qs*((Jw*aJj)+(Jp*(J5*(Jt*aJm))))))-(JA*(if ru{a3B}else{(if rn{((rq*a37)+(ro*(a2G*a3f)))}else{(if rd{a37}else{(if r7{(C*(a2l+((a2C+a2G)*a2P)))}else{g})})})})))/a3L)), (sf[60]*(aJW+(((rx*((Jx*a1k)+(qs*((Jw*aJk)+(Jp*(J5*((Jt*aJn)+(Jr*(aJq+aJq)))))))))-(JA*(if ru{a3C}else{(if rn{((rq*a38)+(ro*(a2H*a3f)))}else{(if rd{a38}else{(if r7{(C*(a2m+((a2D+a2H)*a2P)))}else{g})})})})))/a3L))), (sf[60]*(aJX+(((rx*((Jx*a1l)+(qs*((Jw*aJl)+(Jp*(J5*(Jr*(aJs+aJs))))))))-(JA*(if ru{a3D}else{(if rn{((rq*a39)+(ro*(a2I*a3f)))}else{(if rd{a39}else{(if r7{(C*(a2n+((a2E+a2I)*a2P)))}else{g})})})})))/a3L)))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3_local(
            Some(7),
            Some(9),
            4,
            multiplicity * (aLm),
            7,
            multiplicity * (aLn),
            9,
            multiplicity * (aLo),
        );
        stamper.stamp_current_reactive_node3_local(
            Some(8),
            Some(6),
            4,
            multiplicity * ((sf[60]*((aKB+(sf[235]*a1O))+aKK))),
            6,
            multiplicity * ((sf[60]*((aKC+(sf[235]*a1P))+aKL))),
            8,
            multiplicity * ((sf[60]*((aKD+(sf[235]*a1Q))+aKM))),
        );
        stamper.stamp_current_reactive_node3_local(
            Some(8),
            Some(5),
            4,
            multiplicity * (aLs),
            5,
            multiplicity * (aLt),
            8,
            multiplicity * (aLu),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(10),
            &[4, 6, 7, 8, 10],
            &[(sf[60]*(aKV+(sf[235]*(if sb[54]{g}else{((sl*M8)+(hv*((sf[210]*(if rJ{((rO*a4q)+(rL*(rM*a4f)))}else{(if rQ{a4j}else{(if rG{a1D}else{a1v})})}))+(sf[211]*(if s0{((s3*a4q)+(rL*(s1*a4f)))}else{(if rV{a4I}else{g})})))))})))), (sf[60]*(sf[235]*(if sb[54]{g}else{(hv*((sf[210]*(if rJ{g}else{(if rQ{g}else{(if rG{a1E}else{a1w})})}))+(sf[211]*(if s0{a4w}else{(if rV{a4J}else{g})}))))}))), (sf[60]*(aKW+(sf[235]*(if sb[54]{g}else{(hv*(sf[210]*(if rJ{a4v}else{(if rQ{a4k}else{g})})))})))), (sf[60]*(sf[235]*(if sb[54]{g}else{(hv*((sf[210]*(if rJ{g}else{(if rQ{g}else{(if rG{a1F}else{a1x})})}))+(sf[211]*(if s0{a4v}else{(if rV{a4K}else{g})}))))}))), (sf[60]*(aKX+(sf[235]*(if sb[54]{g}else{(hv*(sf[210]*(if rJ{a4w}else{(if rQ{a4l}else{g})})))}))))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(2),
            1,
            multiplicity * (sf[238]),
            2,
            multiplicity * (sf[321]),
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(0),
            0,
            multiplicity * (sf[322]),
            1,
            multiplicity * (sf[239]),
        );
        stamper.stamp_current_reactive_node3_local(
            Some(11),
            Some(10),
            4,
            multiplicity * (aLA),
            10,
            multiplicity * (aLB),
            11,
            multiplicity * (aLC),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(12),
            None,
            12,
            multiplicity * (sf[241]),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(13),
            None,
            13,
            multiplicity * (sf[323]),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(4),
            None,
            4,
            multiplicity * (sf[240]),
        );
    }
}
