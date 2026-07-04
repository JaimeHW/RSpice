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
    b: f64, z: f64, F: f64, cg: f64, cj: f64, ct: f64,
    cz: f64, cI: f64, cL: f64, cP: f64, cS: f64, di: f64,
    dk: f64, do_: f64, ds: f64, ep: f64, er: f64, es: f64,
    ew: f64, ey: f64, ez: f64, eA: f64, eG: f64, eV: f64,
    eY: f64, eZ: f64, f2: f64, f5: f64, fY: f64, h6: f64,
    h9: f64, ha: f64, hb: f64, hc: f64, hd: f64, he: f64,
    hf: f64, hg: f64, hi: f64, hy: f64, in_: f64, k6: f64,
    k9: f64, mk: f64, mx: f64, mJ: f64, mN: f64, r1: f64,
    rJ: f64, rS: bool, sL: f64, sM: f64, sN: f64, sV: f64,
    sW: f64, sX: f64, t5: f64, t6: f64, t7: f64, tf: f64,
    tg: f64, th: f64, tN: f64, tO: f64, tP: f64, tQ: f64,
    tV: f64, tW: f64, tX: f64, tY: f64, ui: f64, uj: f64,
    uk: f64, ul: f64, uG: f64, uH: f64, uI: f64, uJ: f64,
    xB: f64, xC: f64, xD: f64, xE: f64, xH: f64, xK: f64,
    xN: f64, xQ: f64, xR: f64, xS: f64, xT: f64, xU: f64,
    y0: f64, y1: f64, y2: f64, y3: f64, y4: f64, y5: f64,
    y6: f64, y7: f64, y8: f64, y9: f64, ya: f64, yb: f64,
    yc: f64, yd: f64, yq: f64, yr: f64, ys: f64, yt: f64,
    zr: f64, zs: f64, zt: f64, zu: f64, zv: f64, zw: f64,
    zx: f64, zy: f64, zz: f64, zA: f64, zB: f64, zC: f64,
    zQ: f64, zR: f64, zS: f64, zT: f64, A7: f64, A8: f64,
    A9: f64, Aa: f64, Dd: f64, De: f64, Df: f64, Dg: f64,
    H8: f64, H9: f64, Ha: f64, Hb: f64, He: f64, Hh: f64,
    Hk: f64, Hn: f64, Ho: f64, Hp: f64, Hq: f64, Hr: f64,
    Hs: f64, Ht: f64, Hu: f64, Hv: f64, Hw: f64, Hx: f64,
    Hy: f64, Hz: f64, HB: f64, HD: f64, HF: f64, HH: f64,
    HM: f64, HN: f64, HO: f64, HP: f64, YO: f64, acm: f64,
    acp: f64, acs: f64, acv: f64, adI: f64, adL: f64, adO: f64,
    adR: f64, aeZ: f64, af0: f64, af1: f64, af2: f64, aiu: f64,
    aix: f64, aji: f64, ajl: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let b=0.0;let j=3.0;let z=0.5;let F=1.0;let cb=ctx.node_voltage(n[3]);let cg=(sf[20]*(ctx.node_voltage(n[2])-cb));let cj=(sf[20]*(ctx.node_voltage(n[0])-cb));let cm=(if ((cj-cg)<b){F}else{b});let cq=(if (cm!=0.0){cj}else{cg});let cr=(if (cm!=0.0){(if (cm!=0.0){cg}else{b})}else{cj});let ct=(if (!(cm!=0.0)){F}else{(if (cm!=0.0){-1.0}else{b})});let cx=(sf[241]+(sf[219]+(((sf[20]*(ctx.node_voltage(n[1])-cb))-sf[237])-sf[89])));let cz=2.0;let cC=(((cx*cx)+sf[242])).sqrt();let cE=(z*(cx+cC));let cF=(sf[219]+cq);let cI=((sf[188]+(cF*cF))).sqrt();let cL=((z*(cF+cI))).sqrt();let cM=(sf[219]+cr);let cP=((sf[188]+(cM*cM))).sqrt();let cS=((z*(cM+cP))).sqrt();let cY=0.25;let d2=((cE+sf[96])).sqrt();let d3=(cE-sf[219]);let da=((sf[182]+(sf[219]+(d3-(sf[70]*(d2-sf[97])))))).sqrt();let df=((sf[70]-(sf[94]*(cL+cS)))+(sf[92]*da));let di=((sf[182]+(df*df))).sqrt();let dk=(z*(df+di));let dl=(cY*dk);let do_=((cE+(dk*dl))).sqrt();let dq=(do_-(z*dk));let ds=(d3-(dk*dq));let du=(sf[183]*(ds-cq));let dv=-0.35;let dx=(if (du>dv){F}else{b});let dy=1.3;let dA=1.6;let dB=(du+dA);let dD=((du+dy)-(dB).ln());let dF=(if (dx!=0.0){(cz/dD)}else{b});let dG=(cz+dF);let dH=(F+du);let dJ=(dH+(dF).ln());let dL=(if (dx!=0.0){(dG/dJ)}else{b});let dN=(dH+(dL).ln());let dO=(cz+dL);let dR=-15.0;let dT=(if (du>dR){F}else{b});let dU=(!(dx!=0.0));let dV=((dT!=0.0)&&dU);let dW=1.55;let dY=((-du)).exp();let e0=(if dV{(dW+dY)}else{dF});let e1=(cz+e0);let e3=(dH+(e0).ln());let e5=(if dV{(e1/e3)}else{dL});let e7=(dH+(e5).ln());let e8=(cz+e5);let eb=-23.0;let ed=(if (du>eb){F}else{b});let ef=(dU&&(!(dT!=0.0)));let eg=((ed!=0.0)&&ef);let eh=(cz+dY);let el=(ef&&(!(ed!=0.0)));let em=(du).exp();let en=1e-64;let ep=(if el{(em+en)}else{(if eg{(F/eh)}else{(if dV{(e7/e8)}else{(if (dx!=0.0){(dN/dO)}else{b})})})});let eq=(F+ep);let er=(ep*eq);let es=(er).sqrt();let ew=((cY+(es*sf[243]))).sqrt();let ey=(sf[225]*(ew-z));let ez=(cr-cq);let eA=(z*ez);let eG=(sf[188]*((sf[5]*(es-(sf[183]*ey)))+0.015625));let eP=0.75;let eV=((cY+(sf[243]*(es-(eP*(er).ln()))))).sqrt();let eY=(sf[230]+(sf[225]*(eV-z)));let eZ=(eA-eY);let f2=((eG+(eY*eY))).sqrt();let f5=((eG+(eZ*eZ))).sqrt();let fa=(sf[183]*(f5+(((ds-eA)-cq)-f2)));let fc=(if (fa>dv){F}else{b});let fe=(dA+fa);let fg=((dy+fa)-(fe).ln());let fi=(if (fc!=0.0){(cz/fg)}else{e0});let fj=(cz+fi);let fk=(F+fa);let fm=(fk+(fi).ln());let fo=(if (fc!=0.0){(fj/fm)}else{e5});let fq=(fk+(fo).ln());let fr=(cz+fo);let fv=(if (fa>dR){F}else{b});let fw=(!(fc!=0.0));let fx=((fv!=0.0)&&fw);let fz=((-fa)).exp();let fB=(if fx{(dW+fz)}else{fi});let fC=(cz+fB);let fE=(fk+(fB).ln());let fG=(if fx{(fC/fE)}else{fo});let fI=(fk+(fG).ln());let fJ=(cz+fG);let fN=(if (fa>eb){F}else{b});let fP=(fw&&(!(fv!=0.0)));let fQ=((fN!=0.0)&&fP);let fR=(cz+fz);let fV=(fP&&(!(fN!=0.0)));let fW=(fa).exp();let fY=(if fV{(en+fW)}else{(if fQ{(F/fR)}else{(if fx{(fI/fJ)}else{(if (fc!=0.0){(fq/fr)}else{ep})})})});let gi=(sf[183]*(ds-cr));let gk=(if (gi>dv){F}else{b});let gm=(dA+gi);let go=((dy+gi)-(gm).ln());let gq=(if (gk!=0.0){(cz/go)}else{fB});let gr=(cz+gq);let gs=(F+gi);let gu=(gs+(gq).ln());let gw=(if (gk!=0.0){(gr/gu)}else{fG});let gy=(gs+(gw).ln());let gz=(cz+gw);let gD=(if (gi>dR){F}else{b});let gE=(!(gk!=0.0));let gF=((gD!=0.0)&&gE);let gH=((-gi)).exp();let gJ=(if gF{(dW+gH)}else{gq});let gK=(cz+gJ);let gM=(gs+(gJ).ln());let gO=(if gF{(gK/gM)}else{gw});let gQ=(gs+(gO).ln());let gR=(cz+gO);let gV=(if (gi>eb){F}else{b});let gX=(gE&&(!(gD!=0.0)));let gY=((gV!=0.0)&&gX);let gZ=(cz+gH);let h3=(gX&&(!(gV!=0.0)));let h4=(gi).exp();let h6=(if h3{(en+h4)}else{(if gY{(F/gZ)}else{(if gF{(gQ/gR)}else{(if (gk!=0.0){(gy/gz)}else{fY})})})});let h7=(F+h6);let h9=(cY+er);let ha=(cY+(h6*h7));let hb=(h9).sqrt();let hc=(ha).sqrt();let hd=(hb+hc);let he=(hd*hd);let hf=(sf[219]+ds);let hg=(1e-6+hf);let hi=(cz*(hg).sqrt());let hy=-0.5;let im=(cz*es);let in_=4.0;let k6=(cz*hc);let k9=(cz*hb);let lV=(h9*hb);let lW=(ha*hc);let lZ=((sf[219]+(z*ds))).sqrt();let m0=(lZ+lZ);let m5=(-(sf[110]*(sf[181]*(F+(dk/m0)))));let m6=0.266666666;let m8=6.0;let m9=(ha*m8);let mc=(hc*in_);
        let mh=(m6*((((j*lW)+(hb*m9))+(h9*mc))+(cz*lV)));let mj=((mh/he)-z);let mk=(m5*mj);let mm=(h9*m8);let mp=(hb*in_);let mu=(m6*((((j*lV)+(hc*mm))+(ha*mp))+(cz*lW)));let mw=((mu/he)-z);let mx=(m5*mw);let my=(mk+mx);let mz=(dk*hy);let mE=(dk*my);let mF=(dk+m0);let mJ=((-my)-((sf[110]*((cE+(hi*mz))-cx))-(mE/mF)));let mN=(if (F==ct){F}else{b});let qe=(if (cj>b){F}else{b});let qj=(F+(cj/sf[261]));let qm=((sf[163]*(qj).ln())).exp();let qt=(F+(cj/sf[263]));let qw=((sf[165]*(qt).ln())).exp();let qD=(F+(cj/sf[265]));let qG=((sf[167]*(qD).ln())).exp();let qJ=(!(qe!=0.0));let r0=((if qJ{(sf[301]*(F-((cj*sf[166])/sf[265])))}else{(if (qe!=0.0){(sf[301]*qG)}else{b})})+((if qJ{(sf[299]*(F-((cj*sf[162])/sf[261])))}else{(if (qe!=0.0){(sf[299]*qm)}else{b})})+(if qJ{(sf[300]*(F-((cj*sf[164])/sf[263])))}else{(if (qe!=0.0){(sf[300]*qw)}else{b})})));let r1=(cj*r0);let r3=(if (cg>b){F}else{b});let r6=(F+(cg/sf[261]));let r9=((sf[163]*(r6).ln())).exp();let re=(F+(cg/sf[263]));let rh=((sf[165]*(re).ln())).exp();let rl=(F+(cg/sf[265]));let ro=((sf[167]*(rl).ln())).exp();let rr=(!(r3!=0.0));let rI=((if rr{(sf[301]*(F-((cg*sf[166])/sf[265])))}else{(if (r3!=0.0){(sf[301]*ro)}else{b})})+((if rr{(sf[302]*(F-((cg*sf[162])/sf[261])))}else{(if (r3!=0.0){(sf[302]*r9)}else{b})})+(if rr{(sf[303]*(F-((cg*sf[164])/sf[263])))}else{(if (r3!=0.0){(sf[303]*rh)}else{b})})));let rJ=(cg*rI);let rS=(!(mN!=0.0));let so=(if (cm!=0.0){sf[20]}else{b});let sq=(if (cm!=0.0){b}else{sf[20]});let sr=(if (cm!=0.0){so}else{b});let ss=(if (cm!=0.0){(if (cm!=0.0){sf[169]}else{b})}else{sf[169]});let st=(sf[20]*cx);let sv=(cx*sf[169]);let sx=(cz*cC);let sC=(z*(sf[20]+((st+st)/sx)));let sD=(z*(sf[169]+((sv+sv)/sx)));let sE=(cF*so);let sG=(cF*sq);let sI=(cF*sf[169]);let sK=(cz*cI);let sL=((sE+sE)/sK);let sM=((sG+sG)/sK);let sN=((sI+sI)/sK);let sU=(cz*cL);let sV=((z*(so+sL))/sU);let sW=((z*(sq+sM))/sU);let sX=((z*(sf[169]+sN))/sU);let sY=(cM*sq);let t0=(cM*sr);let t2=(cM*ss);let t4=(cz*cP);let t5=((sY+sY)/t4);let t6=((t0+t0)/t4);let t7=((t2+t2)/t4);let te=(cz*cS);let tf=((z*(sq+t5))/te);let tg=((z*(sr+t6))/te);let th=((z*(ss+t7))/te);let ti=(cz*d2);let tp=(cz*da);let ty=(-(sf[94]*(sV+tf)));let tz=(-(sf[94]*(sW+tg)));let tB=(sf[92]*((sC-(sf[70]*(sC/ti)))/tp));let tD=((-(sf[94]*(sX+th)))+(sf[92]*((sD-(sf[70]*(sD/ti)))/tp)));let tE=(df*ty);let tG=(df*tB);let tI=(df*tz);let tK=(df*tD);let tM=(cz*di);let tN=((tE+tE)/tM);let tO=((tG+tG)/tM);let tP=((tI+tI)/tM);let tQ=((tK+tK)/tM);let tV=(z*(ty+tN));let tW=(z*(tB+tO));let tX=(z*(tz+tP));let tY=(z*(tD+tQ));let uh=(cz*do_);let ui=(((dl*tV)+(dk*(cY*tV)))/uh);let uj=((sC+((dl*tW)+(dk*(cY*tW))))/uh);let uk=(((dl*tX)+(dk*(cY*tX)))/uh);let ul=((sD+((dl*tY)+(dk*(cY*tY))))/uh);let uG=(-((dq*tV)+(dk*(ui-(z*tV)))));let uH=(sC-((dq*tW)+(dk*(uj-(z*tW)))));let uI=(-((dq*tX)+(dk*(uk-(z*tX)))));let uJ=(sD-((dq*tY)+(dk*(ul-(z*tY)))));let uN=(sf[183]*(uG-so));let uO=(sf[183]*uH);let uP=(sf[183]*(uI-sq));let uQ=(sf[183]*(uJ-sf[169]));let v1=(dD*dD);let vc=(if (dx!=0.0){((-(cz*(uN-(uN/dB))))/v1)}else{b});let vd=(if (dx!=0.0){((-(cz*(uO-(uO/dB))))/v1)}else{b});let ve=(if (dx!=0.0){((-(cz*(uP-(uP/dB))))/v1)}else{b});let vf=(if (dx!=0.0){((-(cz*(uQ-(uQ/dB))))/v1)}else{b});let vr=(dJ*dJ);let vF=(if (dx!=0.0){(((dJ*vc)-(dG*(uN+(vc/dF))))/vr)}else{b});let vG=(if (dx!=0.0){(((dJ*vd)-(dG*(uO+(vd/dF))))/vr)}else{b});let vH=(if (dx!=0.0){(((dJ*ve)-(dG*(uP+(ve/dF))))/vr)}else{b});let vI=(if (dx!=0.0){(((dJ*vf)-(dG*(uQ+(vf/dF))))/vr)}else{b});let vU=(dO*dO);let wd=(-uO);let wg=(dY*(-uN));let wh=(dY*wd);let wi=(dY*(-uP));let wj=(dY*(-uQ));let wk=(if dV{wg}else{vc});let wl=(if dV{wh}else{vd});let wm=(if dV{wi}else{ve});let wn=(if dV{wj}else{vf});let wz=(e3*e3);let wN=(if dV{(((e3*wk)-(e1*(uN+(wk/e0))))/wz)}else{vF});let wO=(if dV{(((e3*wl)-(e1*(uO+(wl/e0))))/wz)}else{vG});let wP=(if dV{(((e3*wm)-(e1*(uP+(wm/e0))))/wz)}else{vH});let wQ=(if dV{(((e3*wn)-(e1*(uQ+(wn/e0))))/wz)}else{vI});let x2=(e8*e8);let xl=(eh*eh);let xB=(if el{(em*uN)}else{(if eg{((-wg)/xl)}else{(if dV{(((e8*(uN+(wN/e5)))-(e7*wN))/x2)}else{(if (dx!=0.0){(((dO*(uN+(vF/dL)))-(dN*vF))/vU)}else{b})})})});
        let xC=(if el{(em*uO)}else{(if eg{((-wh)/xl)}else{(if dV{(((e8*(uO+(wO/e5)))-(e7*wO))/x2)}else{(if (dx!=0.0){(((dO*(uO+(vG/dL)))-(dN*vG))/vU)}else{b})})})});let xD=(if el{(em*uP)}else{(if eg{((-wi)/xl)}else{(if dV{(((e8*(uP+(wP/e5)))-(e7*wP))/x2)}else{(if (dx!=0.0){(((dO*(uP+(vH/dL)))-(dN*vH))/vU)}else{b})})})});let xE=(if el{(em*uQ)}else{(if eg{((-wj)/xl)}else{(if dV{(((e8*(uQ+(wQ/e5)))-(e7*wQ))/x2)}else{(if (dx!=0.0){(((dO*(uQ+(vI/dL)))-(dN*vI))/vU)}else{b})})})});let xH=((eq*xB)+(ep*xB));let xK=((eq*xC)+(ep*xC));let xN=((eq*xD)+(ep*xD));let xQ=((eq*xE)+(ep*xE));let xR=(xH/im);let xS=(xK/im);let xT=(xN/im);let xU=(xQ/im);let xZ=(cz*ew);let y0=((sf[243]*xR)/xZ);let y1=((sf[243]*xS)/xZ);let y2=((sf[243]*xT)/xZ);let y3=((sf[243]*xU)/xZ);let y4=(sf[225]*y0);let y5=(sf[225]*y1);let y6=(sf[225]*y2);let y7=(sf[225]*y3);let y8=(sq-so);let y9=(sr-sq);let ya=(ss-sf[169]);let yb=(z*y8);let yc=(z*y9);let yd=(z*ya);let yq=(sf[188]*(sf[5]*(xR-(sf[183]*y4))));let yr=(sf[188]*(sf[5]*(xS-(sf[183]*y5))));let ys=(sf[188]*(sf[5]*(xT-(sf[183]*y6))));let yt=(sf[188]*(sf[5]*(xU-(sf[183]*y7))));let zq=(cz*eV);let zr=((sf[243]*(xR-(eP*(xH/er))))/zq);let zs=((sf[243]*(xS-(eP*(xK/er))))/zq);let zt=((sf[243]*(xT-(eP*(xN/er))))/zq);let zu=((sf[243]*(xU-(eP*(xQ/er))))/zq);let zv=(sf[225]*zr);let zw=(sf[225]*zs);let zx=(sf[225]*zt);let zy=(sf[225]*zu);let zz=(yb-zv);let zA=(-zw);let zB=(yc-zx);let zC=(yd-zy);let zD=(eY*zv);let zF=(eY*zw);let zH=(eY*zx);let zJ=(eY*zy);let zP=(cz*f2);let zQ=((yq+(zD+zD))/zP);let zR=((yr+(zF+zF))/zP);let zS=((ys+(zH+zH))/zP);let zT=((yt+(zJ+zJ))/zP);let zU=(eZ*zz);let zW=(eZ*zA);let zY=(eZ*zB);let A0=(eZ*zC);let A6=(cz*f5);let A7=((yq+(zU+zU))/A6);let A8=((yr+(zW+zW))/A6);let A9=((ys+(zY+zY))/A6);let Aa=((yt+(A0+A0))/A6);let Ap=(sf[183]*(A7+(((uG-yb)-so)-zQ)));let Aq=(sf[183]*(A8+(uH-zR)));let Ar=(sf[183]*(A9+(((uI-yc)-sq)-zS)));let As=(sf[183]*(Aa+(((uJ-yd)-sf[169])-zT)));let AD=(fg*fg);let AO=(if (fc!=0.0){((-(cz*(Ap-(Ap/fe))))/AD)}else{wk});let AP=(if (fc!=0.0){((-(cz*(Aq-(Aq/fe))))/AD)}else{wl});let AQ=(if (fc!=0.0){((-(cz*(Ar-(Ar/fe))))/AD)}else{wm});let AR=(if (fc!=0.0){((-(cz*(As-(As/fe))))/AD)}else{wn});let B3=(fm*fm);let Bh=(if (fc!=0.0){(((fm*AO)-(fj*(Ap+(AO/fi))))/B3)}else{wN});let Bi=(if (fc!=0.0){(((fm*AP)-(fj*(Aq+(AP/fi))))/B3)}else{wO});let Bj=(if (fc!=0.0){(((fm*AQ)-(fj*(Ar+(AQ/fi))))/B3)}else{wP});let Bk=(if (fc!=0.0){(((fm*AR)-(fj*(As+(AR/fi))))/B3)}else{wQ});let Bw=(fr*fr);let BS=(fz*(-Ap));let BT=(fz*(-Aq));let BU=(fz*(-Ar));let BV=(fz*(-As));let BW=(if fx{BS}else{AO});let BX=(if fx{BT}else{AP});let BY=(if fx{BU}else{AQ});let BZ=(if fx{BV}else{AR});let Cb=(fE*fE);let Cp=(if fx{(((fE*BW)-(fC*(Ap+(BW/fB))))/Cb)}else{Bh});let Cq=(if fx{(((fE*BX)-(fC*(Aq+(BX/fB))))/Cb)}else{Bi});let Cr=(if fx{(((fE*BY)-(fC*(Ar+(BY/fB))))/Cb)}else{Bj});let Cs=(if fx{(((fE*BZ)-(fC*(As+(BZ/fB))))/Cb)}else{Bk});let CE=(fJ*fJ);let CX=(fR*fR);let Dd=(if fV{(fW*Ap)}else{(if fQ{((-BS)/CX)}else{(if fx{(((fJ*(Ap+(Cp/fG)))-(fI*Cp))/CE)}else{(if (fc!=0.0){(((fr*(Ap+(Bh/fo)))-(fq*Bh))/Bw)}else{xB})})})});let De=(if fV{(fW*Aq)}else{(if fQ{((-BT)/CX)}else{(if fx{(((fJ*(Aq+(Cq/fG)))-(fI*Cq))/CE)}else{(if (fc!=0.0){(((fr*(Aq+(Bi/fo)))-(fq*Bi))/Bw)}else{xC})})})});let Df=(if fV{(fW*Ar)}else{(if fQ{((-BU)/CX)}else{(if fx{(((fJ*(Ar+(Cr/fG)))-(fI*Cr))/CE)}else{(if (fc!=0.0){(((fr*(Ar+(Bj/fo)))-(fq*Bj))/Bw)}else{xD})})})});let Dg=(if fV{(fW*As)}else{(if fQ{((-BV)/CX)}else{(if fx{(((fJ*(As+(Cs/fG)))-(fI*Cs))/CE)}else{(if (fc!=0.0){(((fr*(As+(Bk/fo)))-(fq*Bk))/Bw)}else{xE})})})});let Em=(sf[183]*(uG-sq));let En=(sf[183]*(uI-sr));let Eo=(sf[183]*(uJ-ss));let Ez=(go*go);let EK=(if (gk!=0.0){((-(cz*(Em-(Em/gm))))/Ez)}else{BW});let EL=(if (gk!=0.0){((-(cz*(uO-(uO/gm))))/Ez)}else{BX});let EM=(if (gk!=0.0){((-(cz*(En-(En/gm))))/Ez)}else{BY});let EN=(if (gk!=0.0){((-(cz*(Eo-(Eo/gm))))/Ez)}else{BZ});let EZ=(gu*gu);let Fd=(if (gk!=0.0){(((gu*EK)-(gr*(Em+(EK/gq))))/EZ)}else{Cp});let Fe=(if (gk!=0.0){(((gu*EL)-(gr*(uO+(EL/gq))))/EZ)}else{Cq});let Ff=(if (gk!=0.0){(((gu*EM)-(gr*(En+(EM/gq))))/EZ)}else{Cr});
        let Fg=(if (gk!=0.0){(((gu*EN)-(gr*(Eo+(EN/gq))))/EZ)}else{Cs});let Fs=(gz*gz);let FN=(gH*(-Em));let FO=(gH*wd);let FP=(gH*(-En));let FQ=(gH*(-Eo));let FR=(if gF{FN}else{EK});let FS=(if gF{FO}else{EL});let FT=(if gF{FP}else{EM});let FU=(if gF{FQ}else{EN});let G6=(gM*gM);let Gk=(if gF{(((gM*FR)-(gK*(Em+(FR/gJ))))/G6)}else{Fd});let Gl=(if gF{(((gM*FS)-(gK*(uO+(FS/gJ))))/G6)}else{Fe});let Gm=(if gF{(((gM*FT)-(gK*(En+(FT/gJ))))/G6)}else{Ff});let Gn=(if gF{(((gM*FU)-(gK*(Eo+(FU/gJ))))/G6)}else{Fg});let Gz=(gR*gR);let GS=(gZ*gZ);let H8=(if h3{(h4*Em)}else{(if gY{((-FN)/GS)}else{(if gF{(((gR*(Em+(Gk/gO)))-(gQ*Gk))/Gz)}else{(if (gk!=0.0){(((gz*(Em+(Fd/gw)))-(gy*Fd))/Fs)}else{Dd})})})});let H9=(if h3{(h4*uO)}else{(if gY{((-FO)/GS)}else{(if gF{(((gR*(uO+(Gl/gO)))-(gQ*Gl))/Gz)}else{(if (gk!=0.0){(((gz*(uO+(Fe/gw)))-(gy*Fe))/Fs)}else{De})})})});let Ha=(if h3{(h4*En)}else{(if gY{((-FP)/GS)}else{(if gF{(((gR*(En+(Gm/gO)))-(gQ*Gm))/Gz)}else{(if (gk!=0.0){(((gz*(En+(Ff/gw)))-(gy*Ff))/Fs)}else{Df})})})});let Hb=(if h3{(h4*Eo)}else{(if gY{((-FQ)/GS)}else{(if gF{(((gR*(Eo+(Gn/gO)))-(gQ*Gn))/Gz)}else{(if (gk!=0.0){(((gz*(Eo+(Fg/gw)))-(gy*Fg))/Fs)}else{Dg})})})});let He=((h7*H8)+(h6*H8));let Hh=((h7*H9)+(h6*H9));let Hk=((h7*Ha)+(h6*Ha));let Hn=((h7*Hb)+(h6*Hb));let Ho=(xH/k9);let Hp=(xK/k9);let Hq=(xN/k9);let Hr=(xQ/k9);let Hs=(He/k6);let Ht=(Hh/k6);let Hu=(Hk/k6);let Hv=(Hn/k6);let Hw=(Ho+Hs);let Hx=(Hp+Ht);let Hy=(Hq+Hu);let Hz=(Hr+Hv);let HA=(hd*Hw);let HB=(HA+HA);let HC=(hd*Hx);let HD=(HC+HC);let HE=(hd*Hy);let HF=(HE+HE);let HG=(hd*Hz);let HH=(HG+HG);let HM=(cz*(uG/hi));let HN=(cz*(uH/hi));let HO=(cz*(uI/hi));let HP=(cz*(uJ/hi));let YO=(he*he);let aa8=((hb*xH)+(h9*Ho));let aab=((hb*xK)+(h9*Hp));let aae=((hb*xN)+(h9*Hq));let aah=((hb*xQ)+(h9*Hr));let aak=((hc*He)+(ha*Hs));let aan=((hc*Hh)+(ha*Ht));let aaq=((hc*Hk)+(ha*Hu));let aat=((hc*Hn)+(ha*Hv));let aay=(cz*lZ);let aaz=((z*uG)/aay);let aaA=((z*uH)/aay);let aaB=((z*uI)/aay);let aaC=((z*uJ)/aay);let aaD=(aaz+aaz);let aaE=(aaA+aaA);let aaF=(aaB+aaB);let aaG=(aaC+aaC);let aaK=(m0*m0);let ab6=(-(sf[110]*(sf[181]*(((m0*tV)-(dk*aaD))/aaK))));let ab7=(-(sf[110]*(sf[181]*(((m0*tW)-(dk*aaE))/aaK))));let ab8=(-(sf[110]*(sf[181]*(((m0*tX)-(dk*aaF))/aaK))));let ab9=(-(sf[110]*(sf[181]*(((m0*tY)-(dk*aaG))/aaK))));let acm=((mj*ab6)+(m5*(((he*(m6*((((j*aak)+((m9*Ho)+(hb*(m8*He))))+((mc*xH)+(h9*(in_*Hs))))+(cz*aa8))))-(mh*HB))/YO)));let acp=((mj*ab7)+(m5*(((he*(m6*((((j*aan)+((m9*Hp)+(hb*(m8*Hh))))+((mc*xK)+(h9*(in_*Ht))))+(cz*aab))))-(mh*HD))/YO)));let acs=((mj*ab8)+(m5*(((he*(m6*((((j*aaq)+((m9*Hq)+(hb*(m8*Hk))))+((mc*xN)+(h9*(in_*Hu))))+(cz*aae))))-(mh*HF))/YO)));let acv=((mj*ab9)+(m5*(((he*(m6*((((j*aat)+((m9*Hr)+(hb*(m8*Hn))))+((mc*xQ)+(h9*(in_*Hv))))+(cz*aah))))-(mh*HH))/YO)));let adI=((mw*ab6)+(m5*(((he*(m6*((((j*aa8)+((mm*Hs)+(hc*(m8*xH))))+((mp*He)+(ha*(in_*Ho))))+(cz*aak))))-(mu*HB))/YO)));let adL=((mw*ab7)+(m5*(((he*(m6*((((j*aab)+((mm*Ht)+(hc*(m8*xK))))+((mp*Hh)+(ha*(in_*Hp))))+(cz*aan))))-(mu*HD))/YO)));let adO=((mw*ab8)+(m5*(((he*(m6*((((j*aae)+((mm*Hu)+(hc*(m8*xN))))+((mp*Hk)+(ha*(in_*Hq))))+(cz*aaq))))-(mu*HF))/YO)));let adR=((mw*ab9)+(m5*(((he*(m6*((((j*aah)+((mm*Hv)+(hc*(m8*xQ))))+((mp*Hn)+(ha*(in_*Hr))))+(cz*aat))))-(mu*HH))/YO)));let adS=(acm+adI);let adT=(acp+adL);let adU=(acs+adO);let adV=(acv+adR);let aeD=(mF*mF);let aeZ=((-adS)-((sf[110]*((mz*HM)+(hi*(hy*tV))))-(((mF*((my*tV)+(dk*adS)))-(mE*(tV+aaD)))/aeD)));let af0=((-adT)-((sf[110]*((sC+((mz*HN)+(hi*(hy*tW))))-sf[20]))-(((mF*((my*tW)+(dk*adT)))-(mE*(tW+aaE)))/aeD)));let af1=((-adU)-((sf[110]*((mz*HO)+(hi*(hy*tX))))-(((mF*((my*tX)+(dk*adU)))-(mE*(tX+aaF)))/aeD)));let af2=((-adV)-((sf[110]*((sD+((mz*HP)+(hi*(hy*tY))))-sf[169]))-(((mF*((my*tY)+(dk*adV)))-(mE*(tY+aaG)))/aeD)));let aiu=((sf[20]*r0)+(cj*((if qJ{sf[344]}else{(if (qe!=0.0){(sf[301]*(qG*(sf[167]*(sf[326]/qD))))}else{b})})+((if qJ{sf[332]}else{(if (qe!=0.0){(sf[299]*(qm*(sf[163]*(sf[322]/qj))))}else{b})})+(if qJ{sf[338]}else{(if (qe!=0.0){(sf[300]*(qw*(sf[165]*(sf[324]/qt))))}else{b})})))));
        let aix=((r0*sf[169])+(cj*((if qJ{sf[345]}else{(if (qe!=0.0){(sf[301]*(qG*(sf[167]*(sf[327]/qD))))}else{b})})+((if qJ{sf[333]}else{(if (qe!=0.0){(sf[299]*(qm*(sf[163]*(sf[323]/qj))))}else{b})})+(if qJ{sf[339]}else{(if (qe!=0.0){(sf[300]*(qw*(sf[165]*(sf[325]/qt))))}else{b})})))));let aji=((sf[20]*rI)+(cg*((if rr{sf[344]}else{(if (r3!=0.0){(sf[301]*(ro*(sf[167]*(sf[326]/rl))))}else{b})})+((if rr{sf[346]}else{(if (r3!=0.0){(sf[302]*(r9*(sf[163]*(sf[322]/r6))))}else{b})})+(if rr{sf[348]}else{(if (r3!=0.0){(sf[303]*(rh*(sf[165]*(sf[324]/re))))}else{b})})))));let ajl=((rI*sf[169])+(cg*((if rr{sf[345]}else{(if (r3!=0.0){(sf[301]*(ro*(sf[167]*(sf[327]/rl))))}else{b})})+((if rr{sf[347]}else{(if (r3!=0.0){(sf[302]*(r9*(sf[163]*(sf[323]/r6))))}else{b})})+(if rr{sf[349]}else{(if (r3!=0.0){(sf[303]*(rh*(sf[165]*(sf[325]/re))))}else{b})})))));

        CommonStampValues {
            b, z, F, cg, cj, ct, cz, cI,
            cL, cP, cS, di, dk, do_, ds, ep,
            er, es, ew, ey, ez, eA, eG, eV,
            eY, eZ, f2, f5, fY, h6, h9, ha,
            hb, hc, hd, he, hf, hg, hi, hy,
            in_, k6, k9, mk, mx, mJ, mN, r1,
            rJ, rS, sL, sM, sN, sV, sW, sX,
            t5, t6, t7, tf, tg, th, tN, tO,
            tP, tQ, tV, tW, tX, tY, ui, uj,
            uk, ul, uG, uH, uI, uJ, xB, xC,
            xD, xE, xH, xK, xN, xQ, xR, xS,
            xT, xU, y0, y1, y2, y3, y4, y5,
            y6, y7, y8, y9, ya, yb, yc, yd,
            yq, yr, ys, yt, zr, zs, zt, zu,
            zv, zw, zx, zy, zz, zA, zB, zC,
            zQ, zR, zS, zT, A7, A8, A9, Aa,
            Dd, De, Df, Dg, H8, H9, Ha, Hb,
            He, Hh, Hk, Hn, Ho, Hp, Hq, Hr,
            Hs, Ht, Hu, Hv, Hw, Hx, Hy, Hz,
            HB, HD, HF, HH, HM, HN, HO, HP,
            YO, acm, acp, acs, acv, adI, adL, adO,
            adR, aeZ, af0, af1, af2, aiu, aix, aji,
            ajl,
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
            b, z, F, cg, cj, ct, cz, cI,
            cL, cP, cS, di, dk, do_, ds, ep,
            er, es, ew, ey, ez, eA, eG, eV,
            eY, eZ, f2, f5, fY, h6, h9, ha,
            hb, hc, hd, he, hf, hg, hi, hy,
            in_, k6, k9, mk, mx, mJ, mN, r1,
            rJ, rS, sL, sM, sN, sV, sW, sX,
            t5, t6, t7, tf, tg, th, tN, tO,
            tP, tQ, tV, tW, tX, tY, ui, uj,
            uk, ul, uG, uH, uI, uJ, xB, xC,
            xD, xE, xH, xK, xN, xQ, xR, xS,
            xT, xU, y0, y1, y2, y3, y4, y5,
            y6, y7, y8, y9, ya, yb, yc, yd,
            yq, yr, ys, yt, zr, zs, zt, zu,
            zv, zw, zx, zy, zz, zA, zB, zC,
            zQ, zR, zS, zT, A7, A8, A9, Aa,
            Dd, De, Df, Dg, H8, H9, Ha, Hb,
            He, Hh, Hk, Hn, Ho, Hp, Hq, Hr,
            Hs, Ht, Hu, Hv, Hw, Hx, Hy, Hz,
            HB, HD, HF, HH, HM, HN, HO, HP,
            YO, acm, acp, acs, acv, adI, adL, adO,
            adR, aeZ, af0, af1, af2, aiu, aix, aji,
            ajl,
        }=self.eval_common_stamp_values(ctx);
        let eJ=((eG+(ey*ey))).sqrt();let eK=(eA-ey);let eN=((eG+(eK*eK))).sqrt();let eO=(eJ-eN);let fZ=(F+fY);let g3=(F+((eA-eO)/sf[222]));let g9=((sf[48]-(sf[6]*(g3).ln()))+(sf[221]*(eA+eO)));let ge=(((g9*g9)+sf[99])).sqrt();let gg=(z*(g9+ge));let hj=(sf[70]/hi);let hk=(sf[70]+hi);let hl=(sf[70]/hk);let hm=(F+hj);let ho=(sf[181]*(-hm));let hp=0.66666666;let hq=1.33333332;let hu=(hq*(h9+(ha+(hb*hc))));let hw=((hu/hd)-F);let hx=(ho*hw);let hH=((sf[187]+(ds*ds))).sqrt();let hI=(if (sf[101]!=0.0){hH}else{b});let hN=((if (sf[101]!=0.0){(z*(ds+hI))}else{b})*sf[102]);let hP=(if (sf[101]!=0.0){(F+hN)}else{b});let hQ=(gg*hP);let hU=(((hi*sf[100])-(hl*hx))+(sf[21]*hx));let hW=(if (hU>b){F}else{b});let hY=((hW!=0.0)&&sb[12]);let hZ=(sf[16]*hU);let i3=(sb[12]&&(!(hW!=0.0)));let i5=(if i3{(F-hZ)}else{(if hY{(F+hZ)}else{b})});let ia=(gg*i5);let ic=(if sb[12]{(sf[247]/ia)}else{(if (sf[101]!=0.0){(sf[240]/hQ)}else{b})});let id=(sf[185]+hf);let ie=(id).sqrt();let if_=(cz*ie);let ih=(F+(sf[70]/if_));let ii=(er-(fY*fZ));let ij=(sf[187]*ih);let ik=(ic*ij);let il=(ii*ik);let io=(di+di);let ir=((dk/io)*sf[103]);let is=(cS*ir);let it=(is/cP);let iu=(cL*ir);let iv=(iu/cI);let ix=(-(hf/do_));let iy=(it*ix);let iz=(iv*ix);let iA=(sf[183]*ep);let iB=(iy*iA);let iC=(iz-F);let iD=(iA*iC);let iE=(ew*in_);let iF=(es*iE);let iG=(sf[181]/iF);let iH=(iB*iG);let iI=(iD*iG);let iL=(es+es);let iM=(sf[181]/iL);let iP=(sf[249]*((iB*iM)-iH));let iS=(sf[249]*((iD*iM)-iI));let iT=(F/eJ);let iU=(F/eN);let iW=(iP+(ey*iH));let iY=(z-iH);let j0=(iP+(eK*iY));let j2=((iT*iW)-(iU*j0));let j4=(iS+(ey*iI));let j6=(hy-iI);let j8=(iS+(eK*j6));let ja=((iT*j4)-(iU*j8));let jd=(sf[181]*(es-1.5));let je=(eV*in_);let jf=(er*je);let jg=(jd/jf);let jh=(iB*jg);let ji=(iD*jg);let jj=(sf[183]*fY);let jk=(F/f2);let jl=(F/f5);let jo=(iP+(eY*jh));let jr=(z-jh);let jt=(iP+(eZ*jr));let jv=(((iy-z)-(jk*jo))+(jl*jt));let jz=(iS+(eY*ji));let jC=(hy-ji);let jE=(iS+(eZ*jC));let jG=(((iz-z)-(jk*jz))+(jl*jE));let jJ=((sf[222]+eA)-eO);let jK=(sf[6]/jJ);let jL=(z-j2);let jN=(hy-ja);let jP=(F/ge);let jT=((-(jK*jL))+(sf[221]*(z+j2)));let jY=((-(jK*jN))+(sf[221]*(hy+ja)));let k0=(sf[183]*h6);let k1=(iy-F);let k2=(k0*k1);let k3=(iz*k0);let k4=(ho*hp);let k5=(k4/he);let k7=(hb+k6);let k8=(k5*k7);let ka=(hc+k9);let kb=(k5*ka);let kc=(-hj);let kd=(hx*kc);let kf=(hj+(cz+hj));let kg=(hg*kf);let kh=(kd/kg);let km=(((iy*kh)+(iB*k8))+(k2*kb));let kr=(((iz*kh)+(iD*k8))+(k3*kb));let ks=(cz*hm);let kt=(hg*ks);let kv=(hm-(hx/kt));let kw=(-hl);let ky=(km+(iy*kv));let kB=(kr+(iz*kv));let kD=(hI*hP);let kF=(if (sf[101]!=0.0){(hN/kD)}else{kv});let kK=(-(jP*jT));let kN=(-(jP*jY));let kR=(if sb[12]{(sf[16]/i5)}else{kF});let kT=((kw*ky)+(sf[21]*km));let kY=((kw*kB)+(sf[21]*kr));let l3=(ih*in_);let l4=(ie*l3);let l5=(id*l4);let l6=(sf[104]/l5);let l9=((if sb[12]{(kK+(kR*kT))}else{(if (sf[101]!=0.0){(kK-(if (sf[101]!=0.0){(iy*kF)}else{b}))}else{b})})+(iy*l6));let lc=((iB+(ii*l9))-(jj*jv));let le=(-ik);let lf=((if sb[12]{(kN+(kR*kY))}else{(if (sf[101]!=0.0){(kN-(if (sf[101]!=0.0){(iz*kF)}else{b}))}else{b})})+(iz*l6));let li=((iD+(ii*lf))-(jj*jG));let ls=((F+((le*li)*sf[109]))+((ik*lc)*sf[109]));let lt=(F/ls);let lu=(il*lt);let lw=(ez-(sf[13]*ey));let lA=(if ((lw>b)&&sb[26]){F}else{b});let lF=(if (lA!=0.0){((if (lA!=0.0){(F/lw)}else{b})*sf[250])}else{b});let lG=-35.0;let lJ=((lA!=0.0)&&((if (lF<lG){F}else{b})!=0.0));let lL=((if lJ{lG}else{lF})).exp();let lM=(if (lA!=0.0){lL}else{b});let lN=(sf[224]*lw);let lP=(if (lA!=0.0){(lM*lN)}else{b});let lS=(!(lA!=0.0));let mK=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, mk);let mL=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, mx);let on=(-cj);let oq=((sf[195]*on)/sf[290]);let or=-40.0;
        let ot=(if (oq<or){F}else{b});let oy=((sf[195]*(on+sf[156]))/sf[290]);let oz=70.0;let oB=(if (oy>oz){F}else{b});let oD=(!(oB!=0.0));let oG=((-oy)).exp();let oJ=(if oD{(F+(sf[157]*oG))}else{(if (oB!=0.0){F}else{b})});let oM=(sf[195]*cj);let oQ=((oM/sf[292])*sf[159]);let oR=(cj+sf[159]);let oS=0.001;let oT=(oR>oS);let oU=(if oT{oR}else{oS});let oW=((oQ/oU)).exp();let p2=((oM/sf[293])*sf[160]);let p3=(cj+sf[160]);let p4=(p3>oS);let p5=(if p4{p3}else{oS});let p7=((p2/p5)).exp();let pe=((oM/sf[294])*sf[161]);let pf=(cj+sf[161]);let pg_=(pf>oS);let ph=(if pg_{pf}else{oS});let pj=((pe/ph)).exp();let pr=(-cg);let pt=((sf[195]*pr)/sf[290]);let pv=(if (pt<or){F}else{b});let pz=((sf[195]*(sf[156]+pr))/sf[290]);let pB=(if (pz>oz){F}else{b});let pD=(!(pB!=0.0));let pF=((-pz)).exp();let pI=(if pD{(F+(sf[157]*pF))}else{(if (pB!=0.0){F}else{b})});let pJ=(sf[195]*cg);let pL=(sf[159]*(pJ/sf[292]));let pM=(cg+sf[159]);let pN=(pM>oS);let pO=(if pN{pM}else{oS});let pQ=((pL/pO)).exp();let pU=(sf[160]*(pJ/sf[293]));let pV=(cg+sf[160]);let pW=(pV>oS);let pX=(if pW{pV}else{oS});let pZ=((pU/pX)).exp();let q4=(sf[161]*(pJ/sf[294]));let q5=(cg+sf[161]);let q6=(q5>oS);let q7=(if q6{q5}else{oS});let q9=((q4/q7)).exp();let rK=(sf[20]*ct);let rM=(sf[20]*mK);let rO=(sf[20]*mL);let rQ=(sf[20]*(if lS{b}else{(if (lA!=0.0){(lu*lP)}else{b})}));let rW=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, mJ);let rY=((if (ot!=0.0){or}else{oq})).exp();let s0=(sf[289]*(F-rY));let s8=((if (pv!=0.0){or}else{pt})).exp();let sa=(sf[298]*(F-s8));let sh=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, r1);let sk=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, rJ);let yu=(ey*y4);let yw=(ey*y5);let yy=(ey*y6);let yA=(ey*y7);let yG=(cz*eJ);let yH=((yq+(yu+yu))/yG);let yI=((yr+(yw+yw))/yG);let yJ=((ys+(yy+yy))/yG);let yK=((yt+(yA+yA))/yG);let yL=(yb-y4);let yM=(-y5);let yN=(yc-y6);let yO=(yd-y7);let yP=(eK*yL);let yR=(eK*yM);let yT=(eK*yN);let yV=(eK*yO);let z1=(cz*eN);let z2=((yq+(yP+yP))/z1);let z3=((yr+(yR+yR))/z1);let z4=((ys+(yT+yT))/z1);let z5=((yt+(yV+yV))/z1);let z6=(yH-z2);let z7=(yI-z3);let z8=(yJ-z4);let z9=(yK-z5);let Dt=(yb-z6);let Du=(-z7);let Dv=(yc-z8);let Dw=(yd-z9);let DU=((-(sf[6]*((Dt/sf[222])/g3)))+(sf[221]*(yb+z6)));let DV=((-(sf[6]*((Du/sf[222])/g3)))+(sf[221]*z7));let DW=((-(sf[6]*((Dv/sf[222])/g3)))+(sf[221]*(yc+z8)));let DX=((-(sf[6]*((Dw/sf[222])/g3)))+(sf[221]*(yd+z9)));let DY=(g9*DU);let E0=(g9*DV);let E2=(g9*DW);let E4=(g9*DX);let E6=(cz*ge);let E7=((DY+DY)/E6);let E8=((E0+E0)/E6);let E9=((E2+E2)/E6);let Ea=((E4+E4)/E6);let Ef=(z*(DU+E7));let Eg=(z*(DV+E8));let Eh=(z*(DW+E9));let Ei=(z*(DX+Ea));let HR=(-(sf[70]*HM));let HS=(hi*hi);let HT=(HR/HS);let HV=(-(sf[70]*HN));let HW=(HV/HS);let HY=(-(sf[70]*HO));let HZ=(HY/HS);let I1=(-(sf[70]*HP));let I2=(I1/HS);let I3=(hk*hk);let I4=(HR/I3);let I5=(HV/I3);let I6=(HY/I3);let I7=(I1/I3);let I8=(-HT);let I9=(-HW);let Ia=(-HZ);let Ib=(-I2);let Ic=(sf[181]*I8);let Id=(sf[181]*I9);let Ie=(sf[181]*Ia);let If=(sf[181]*Ib);let IW=((hw*Ic)+(ho*(((hd*(hq*(xH+(He+((hc*Ho)+(hb*Hs))))))-(hu*Hw))/he)));let IZ=((hw*Id)+(ho*(((hd*(hq*(xK+(Hh+((hc*Hp)+(hb*Ht))))))-(hu*Hx))/he)));let J2=((hw*Ie)+(ho*(((hd*(hq*(xN+(Hk+((hc*Hq)+(hb*Hu))))))-(hu*Hy))/he)));let J5=((hw*If)+(ho*(((hd*(hq*(xQ+(Hn+((hc*Hr)+(hb*Hv))))))-(hu*Hz))/he)));let Jq=(ds*uG);let Js=(ds*uH);let Ju=(ds*uI);let Jw=(ds*uJ);let Jy=(cz*hH);let JD=(if (sf[101]!=0.0){((Jq+Jq)/Jy)}else{b});let JE=(if (sf[101]!=0.0){((Js+Js)/Jy)}else{b});let JF=(if (sf[101]!=0.0){((Ju+Ju)/Jy)}else{b});
        let JG=(if (sf[101]!=0.0){((Jw+Jw)/Jy)}else{b});let JT=(sf[102]*(if (sf[101]!=0.0){(z*(uG+JD))}else{b}));let JU=(sf[102]*(if (sf[101]!=0.0){(z*(uH+JE))}else{b}));let JV=(sf[102]*(if (sf[101]!=0.0){(z*(uI+JF))}else{b}));let JW=(sf[102]*(if (sf[101]!=0.0){(z*(uJ+JG))}else{b}));let JX=(if (sf[101]!=0.0){JT}else{b});let JY=(if (sf[101]!=0.0){JU}else{b});let JZ=(if (sf[101]!=0.0){JV}else{b});let K0=(if (sf[101]!=0.0){JW}else{b});let Kf=(hQ*hQ);let KC=(sf[16]*(((sf[100]*HM)-((hx*I4)+(hl*IW)))+(sf[21]*IW)));let KD=(sf[16]*(((sf[100]*HN)-((hx*I5)+(hl*IZ)))+(sf[21]*IZ)));let KE=(sf[16]*(((sf[100]*HO)-((hx*I6)+(hl*J2)))+(sf[21]*J2)));let KF=(sf[16]*(((sf[100]*HP)-((hx*I7)+(hl*J5)))+(sf[21]*J5)));let KO=(if i3{(-KC)}else{(if hY{KC}else{b})});let KP=(if i3{(-KD)}else{(if hY{KD}else{b})});let KQ=(if i3{(-KE)}else{(if hY{KE}else{b})});let KR=(if i3{(-KF)}else{(if hY{KF}else{b})});let L6=(ia*ia);let Ll=(uG/if_);let Lm=(uH/if_);let Ln=(uI/if_);let Lo=(uJ/if_);let Lv=(if_*if_);let Lw=((-(sf[70]*(cz*Ll)))/Lv);let Lz=((-(sf[70]*(cz*Lm)))/Lv);let LC=((-(sf[70]*(cz*Ln)))/Lv);let LF=((-(sf[70]*(cz*Lo)))/Lv);let LG=(xH-((fZ*Dd)+(fY*Dd)));let LH=(xK-((fZ*De)+(fY*De)));let LI=(xN-((fZ*Df)+(fY*Df)));let LJ=(xQ-((fZ*Dg)+(fY*Dg)));let LQ=((ij*(if sb[12]{((-(sf[247]*((i5*Ef)+(gg*KO))))/L6)}else{(if (sf[101]!=0.0){((-(sf[240]*((hP*Ef)+(gg*JX))))/Kf)}else{b})}))+(ic*(sf[187]*Lw)));let LT=((ij*(if sb[12]{((-(sf[247]*((i5*Eg)+(gg*KP))))/L6)}else{(if (sf[101]!=0.0){((-(sf[240]*((hP*Eg)+(gg*JY))))/Kf)}else{b})}))+(ic*(sf[187]*Lz)));let LW=((ij*(if sb[12]{((-(sf[247]*((i5*Eh)+(gg*KQ))))/L6)}else{(if (sf[101]!=0.0){((-(sf[240]*((hP*Eh)+(gg*JZ))))/Kf)}else{b})}))+(ic*(sf[187]*LC)));let LZ=((ij*(if sb[12]{((-(sf[247]*((i5*Ei)+(gg*KR))))/L6)}else{(if (sf[101]!=0.0){((-(sf[240]*((hP*Ei)+(gg*K0))))/Kf)}else{b})}))+(ic*(sf[187]*LF)));let Mj=(io*io);let Mx=(sf[103]*(((io*tV)-(dk*(tN+tN)))/Mj));let My=(sf[103]*(((io*tW)-(dk*(tO+tO)))/Mj));let Mz=(sf[103]*(((io*tX)-(dk*(tP+tP)))/Mj));let MA=(sf[103]*(((io*tY)-(dk*(tQ+tQ)))/Mj));let MO=(cP*cP);let Nc=(cI*cI);let Nq=(do_*do_);let NE=(-(((do_*uG)-(hf*ui))/Nq));let NF=(-(((do_*uH)-(hf*uj))/Nq));let NG=(-(((do_*uI)-(hf*uk))/Nq));let NH=(-(((do_*uJ)-(hf*ul))/Nq));let NK=((ix*(((cP*((ir*tf)+(cS*Mx)))-(is*t5))/MO))+(it*NE));let NN=((ix*((cS*My)/cP))+(it*NF));let NQ=((ix*(((cP*((ir*tg)+(cS*Mz)))-(is*t6))/MO))+(it*NG));let NT=((ix*(((cP*((ir*th)+(cS*MA)))-(is*t7))/MO))+(it*NH));let NW=((ix*(((cI*((ir*sV)+(cL*Mx)))-(iu*sL))/Nc))+(iv*NE));let NZ=((ix*((cL*My)/cI))+(iv*NF));let O2=((ix*(((cI*((ir*sW)+(cL*Mz)))-(iu*sM))/Nc))+(iv*NG));let O5=((ix*(((cI*((ir*sX)+(cL*MA)))-(iu*sN))/Nc))+(iv*NH));let O6=(sf[183]*xB);let O7=(sf[183]*xC);let O8=(sf[183]*xD);let O9=(sf[183]*xE);let Oc=((iA*NK)+(iy*O6));let Of=((iA*NN)+(iy*O7));let Oi=((iA*NQ)+(iy*O8));let Ol=((iA*NT)+(iy*O9));let Oo=((iC*O6)+(iA*NW));let Or=((iC*O7)+(iA*NZ));let Ou=((iC*O8)+(iA*O2));let Ox=((iC*O9)+(iA*O5));let OQ=(iF*iF);let OR=((-(sf[181]*((iE*xR)+(es*(in_*y0)))))/OQ);let OU=((-(sf[181]*((iE*xS)+(es*(in_*y1)))))/OQ);let OX=((-(sf[181]*((iE*xT)+(es*(in_*y2)))))/OQ);let P0=((-(sf[181]*((iE*xU)+(es*(in_*y3)))))/OQ);let P3=((iG*Oc)+(iB*OR));let P6=((iG*Of)+(iB*OU));let P9=((iG*Oi)+(iB*OX));let Pc=((iG*Ol)+(iB*P0));let Pf=((iG*Oo)+(iD*OR));let Pi=((iG*Or)+(iD*OU));let Pl=((iG*Ou)+(iD*OX));let Po=((iG*Ox)+(iD*P0));let Pv=(iL*iL);let Pw=((-(sf[181]*(xR+xR)))/Pv);let Pz=((-(sf[181]*(xS+xS)))/Pv);let PC=((-(sf[181]*(xT+xT)))/Pv);let PF=((-(sf[181]*(xU+xU)))/Pv);let PW=(sf[249]*(((iM*Oc)+(iB*Pw))-P3));let PX=(sf[249]*(((iM*Of)+(iB*Pz))-P6));let PY=(sf[249]*(((iM*Oi)+(iB*PC))-P9));let PZ=(sf[249]*(((iM*Ol)+(iB*PF))-Pc));let Qg=(sf[249]*(((iM*Oo)+(iD*Pw))-Pf));let Qh=(sf[249]*(((iM*Or)+(iD*Pz))-Pi));let Qi=(sf[249]*(((iM*Ou)+(iD*PC))-Pl));let Qj=(sf[249]*(((iM*Ox)+(iD*PF))-Po));let Ql=(eJ*eJ);let Qm=((-yH)/Ql);let Qo=((-yI)/Ql);let Qq=((-yJ)/Ql);let Qs=((-yK)/Ql);let Qu=(eN*eN);let Qv=((-z2)/Qu);let Qx=((-z3)/Qu);let Qz=((-z4)/Qu);let QB=((-z5)/Qu);let RA=(((iW*Qm)+(iT*(PW+((iH*y4)+(ey*P3)))))-((j0*Qv)+(iU*(PW+((iY*yL)+(eK*(-P3)))))));
        let RB=(((iW*Qo)+(iT*(PX+((iH*y5)+(ey*P6)))))-((j0*Qx)+(iU*(PX+((iY*yM)+(eK*(-P6)))))));let RC=(((iW*Qq)+(iT*(PY+((iH*y6)+(ey*P9)))))-((j0*Qz)+(iU*(PY+((iY*yN)+(eK*(-P9)))))));let RD=(((iW*Qs)+(iT*(PZ+((iH*y7)+(ey*Pc)))))-((j0*QB)+(iU*(PZ+((iY*yO)+(eK*(-Pc)))))));let SC=(((j4*Qm)+(iT*(Qg+((iI*y4)+(ey*Pf)))))-((j8*Qv)+(iU*(Qg+((j6*yL)+(eK*(-Pf)))))));let SD=(((j4*Qo)+(iT*(Qh+((iI*y5)+(ey*Pi)))))-((j8*Qx)+(iU*(Qh+((j6*yM)+(eK*(-Pi)))))));let SE=(((j4*Qq)+(iT*(Qi+((iI*y6)+(ey*Pl)))))-((j8*Qz)+(iU*(Qi+((j6*yN)+(eK*(-Pl)))))));let SF=(((j4*Qs)+(iT*(Qj+((iI*y7)+(ey*Po)))))-((j8*QB)+(iU*(Qj+((j6*yO)+(eK*(-Po)))))));let T3=(jf*jf);let T4=(((jf*(sf[181]*xR))-(jd*((je*xH)+(er*(in_*zr)))))/T3);let T8=(((jf*(sf[181]*xS))-(jd*((je*xK)+(er*(in_*zs)))))/T3);let Tc=(((jf*(sf[181]*xT))-(jd*((je*xN)+(er*(in_*zt)))))/T3);let Tg=(((jf*(sf[181]*xU))-(jd*((je*xQ)+(er*(in_*zu)))))/T3);let Tj=((jg*Oc)+(iB*T4));let Tm=((jg*Of)+(iB*T8));let Tp=((jg*Oi)+(iB*Tc));let Ts=((jg*Ol)+(iB*Tg));let Tv=((jg*Oo)+(iD*T4));let Ty=((jg*Or)+(iD*T8));let TB=((jg*Ou)+(iD*Tc));let TE=((jg*Ox)+(iD*Tg));let TF=(sf[183]*Dd);let TG=(sf[183]*De);let TH=(sf[183]*Df);let TI=(sf[183]*Dg);let TK=(f2*f2);let TL=((-zQ)/TK);let TN=((-zR)/TK);let TP=((-zS)/TK);let TR=((-zT)/TK);let TT=(f5*f5);let TU=((-A7)/TT);let TW=((-A8)/TT);let TY=((-A9)/TT);let U0=((-Aa)/TT);let WD=(jJ*jJ);let WE=((-(sf[6]*Dt))/WD);let WH=((-(sf[6]*Du))/WD);let WK=((-(sf[6]*Dv))/WD);let WN=((-(sf[6]*Dw))/WD);let Xl=(ge*ge);let Xm=((-E7)/Xl);let Xo=((-E8)/Xl);let Xq=((-E9)/Xl);let Xs=((-Ea)/Xl);let Yf=(sf[183]*H8);let Yg=(sf[183]*H9);let Yh=(sf[183]*Ha);let Yi=(sf[183]*Hb);let YP=(((he*(hp*Ic))-(k4*HB))/YO);let YT=(((he*(hp*Id))-(k4*HD))/YO);let YX=(((he*(hp*Ie))-(k4*HF))/YO);let Z1=(((he*(hp*If))-(k4*HH))/YO);let Zc=((k7*YP)+(k5*(Ho+(cz*Hs))));let Zf=((k7*YT)+(k5*(Hp+(cz*Ht))));let Zi=((k7*YX)+(k5*(Hq+(cz*Hu))));let Zl=((k7*Z1)+(k5*(Hr+(cz*Hv))));let Zw=((ka*YP)+(k5*(Hs+(cz*Ho))));let Zz=((ka*YT)+(k5*(Ht+(cz*Hp))));let ZC=((ka*YX)+(k5*(Hu+(cz*Hq))));let ZF=((ka*Z1)+(k5*(Hv+(cz*Hr))));let a0b=(kg*kg);let a0c=(((kg*((kc*IW)+(hx*I8)))-(kd*((kf*uG)+(hg*(HT+HT)))))/a0b);let a0g=(((kg*((kc*IZ)+(hx*I9)))-(kd*((kf*uH)+(hg*(HW+HW)))))/a0b);let a0k=(((kg*((kc*J2)+(hx*Ia)))-(kd*((kf*uI)+(hg*(HZ+HZ)))))/a0b);let a0o=(((kg*((kc*J5)+(hx*Ib)))-(kd*((kf*uJ)+(hg*(I2+I2)))))/a0b);let a13=((((kh*NK)+(iy*a0c))+((k8*Oc)+(iB*Zc)))+((kb*((k1*Yf)+(k0*NK)))+(k2*Zw)));let a14=((((kh*NN)+(iy*a0g))+((k8*Of)+(iB*Zf)))+((kb*((k1*Yg)+(k0*NN)))+(k2*Zz)));let a15=((((kh*NQ)+(iy*a0k))+((k8*Oi)+(iB*Zi)))+((kb*((k1*Yh)+(k0*NQ)))+(k2*ZC)));let a16=((((kh*NT)+(iy*a0o))+((k8*Ol)+(iB*Zl)))+((kb*((k1*Yi)+(k0*NT)))+(k2*ZF)));let a1L=((((kh*NW)+(iz*a0c))+((k8*Oo)+(iD*Zc)))+((kb*((k0*NW)+(iz*Yf)))+(k3*Zw)));let a1M=((((kh*NZ)+(iz*a0g))+((k8*Or)+(iD*Zf)))+((kb*((k0*NZ)+(iz*Yg)))+(k3*Zz)));let a1N=((((kh*O2)+(iz*a0k))+((k8*Ou)+(iD*Zi)))+((kb*((k0*O2)+(iz*Yh)))+(k3*ZC)));let a1O=((((kh*O5)+(iz*a0o))+((k8*Ox)+(iD*Zl)))+((kb*((k0*O5)+(iz*Yi)))+(k3*ZF)));let a28=(kt*kt);let a2m=(HT-(((kt*IW)-(hx*((ks*uG)+(hg*(cz*HT)))))/a28));let a2n=(HW-(((kt*IZ)-(hx*((ks*uH)+(hg*(cz*HW)))))/a28));let a2o=(HZ-(((kt*J2)-(hx*((ks*uI)+(hg*(cz*HZ)))))/a28));let a2p=(I2-(((kt*J5)-(hx*((ks*uJ)+(hg*(cz*I2)))))/a28));let a2q=(-I4);let a2r=(-I5);let a2s=(-I6);let a2t=(-I7);let a3D=(kD*kD);let a3R=(if (sf[101]!=0.0){(((kD*JT)-(hN*((hP*JD)+(hI*JX))))/a3D)}else{a2m});let a3S=(if (sf[101]!=0.0){(((kD*JU)-(hN*((hP*JE)+(hI*JY))))/a3D)}else{a2n});let a3T=(if (sf[101]!=0.0){(((kD*JV)-(hN*((hP*JF)+(hI*JZ))))/a3D)}else{a2o});let a3U=(if (sf[101]!=0.0){(((kD*JW)-(hN*((hP*JG)+(hI*K0))))/a3D)}else{a2p});let a4r=(-((jT*Xm)+(jP*((-((jL*WE)+(jK*(-RA))))+(sf[221]*RA)))));let a4s=(-((jT*Xo)+(jP*((-((jL*WH)+(jK*(-RB))))+(sf[221]*RB)))));let a4t=(-((jT*Xq)+(jP*((-((jL*WK)+(jK*(-RC))))+(sf[221]*RC)))));let a4u=(-((jT*Xs)+(jP*((-((jL*WN)+(jK*(-RD))))+(sf[221]*RD)))));let a4D=(-((jY*Xm)+(jP*((-((jN*WE)+(jK*(-SC))))+(sf[221]*SC)))));let a4E=(-((jY*Xo)+(jP*((-((jN*WH)+(jK*(-SD))))+(sf[221]*SD)))));let a4F=(-((jY*Xq)+(jP*((-((jN*WK)+(jK*(-SE))))+(sf[221]*SE)))));
        let a4G=(-((jY*Xs)+(jP*((-((jN*WN)+(jK*(-SF))))+(sf[221]*SF)))));let a4R=(i5*i5);let a52=(if sb[12]{((-(sf[16]*KO))/a4R)}else{a3R});let a53=(if sb[12]{((-(sf[16]*KP))/a4R)}else{a3S});let a54=(if sb[12]{((-(sf[16]*KQ))/a4R)}else{a3T});let a55=(if sb[12]{((-(sf[16]*KR))/a4R)}else{a3U});let a6u=(l5*l5);let a6v=((-(sf[104]*((l4*uG)+(id*((l3*Ll)+(ie*(in_*Lw)))))))/a6u);let a6y=((-(sf[104]*((l4*uH)+(id*((l3*Lm)+(ie*(in_*Lz)))))))/a6u);let a6B=((-(sf[104]*((l4*uI)+(id*((l3*Ln)+(ie*(in_*LC)))))))/a6u);let a6E=((-(sf[104]*((l4*uJ)+(id*((l3*Lo)+(ie*(in_*LF)))))))/a6u);let a8p=((sf[109]*((li*(-LQ))+(le*((Oo+((lf*LG)+(ii*((if sb[12]{(a4D+((kY*a52)+(kR*(((kB*a2q)+(kw*(a1L+((kv*NW)+(iz*a2m)))))+(sf[21]*a1L)))))}else{(if (sf[101]!=0.0){(a4D-(if (sf[101]!=0.0){((kF*NW)+(iz*a3R))}else{b}))}else{b})})+((l6*NW)+(iz*a6v))))))-((jG*TF)+(jj*((NW-((jz*TL)+(jk*(Qg+((ji*zv)+(eY*Tv))))))+((jE*TU)+(jl*(Qg+((jC*zz)+(eZ*(-Tv)))))))))))))+(sf[109]*((lc*LQ)+(ik*((Oc+((l9*LG)+(ii*((if sb[12]{(a4r+((kT*a52)+(kR*(((ky*a2q)+(kw*(a13+((kv*NK)+(iy*a2m)))))+(sf[21]*a13)))))}else{(if (sf[101]!=0.0){(a4r-(if (sf[101]!=0.0){((kF*NK)+(iy*a3R))}else{b}))}else{b})})+((l6*NK)+(iy*a6v))))))-((jv*TF)+(jj*((NK-((jo*TL)+(jk*(PW+((jh*zv)+(eY*Tj))))))+((jt*TU)+(jl*(PW+((jr*zz)+(eZ*(-Tj))))))))))))));let a8q=((sf[109]*((li*(-LT))+(le*((Or+((lf*LH)+(ii*((if sb[12]{(a4E+((kY*a53)+(kR*(((kB*a2r)+(kw*(a1M+((kv*NZ)+(iz*a2n)))))+(sf[21]*a1M)))))}else{(if (sf[101]!=0.0){(a4E-(if (sf[101]!=0.0){((kF*NZ)+(iz*a3S))}else{b}))}else{b})})+((l6*NZ)+(iz*a6y))))))-((jG*TG)+(jj*((NZ-((jz*TN)+(jk*(Qh+((ji*zw)+(eY*Ty))))))+((jE*TW)+(jl*(Qh+((jC*zA)+(eZ*(-Ty)))))))))))))+(sf[109]*((lc*LT)+(ik*((Of+((l9*LH)+(ii*((if sb[12]{(a4s+((kT*a53)+(kR*(((ky*a2r)+(kw*(a14+((kv*NN)+(iy*a2n)))))+(sf[21]*a14)))))}else{(if (sf[101]!=0.0){(a4s-(if (sf[101]!=0.0){((kF*NN)+(iy*a3S))}else{b}))}else{b})})+((l6*NN)+(iy*a6y))))))-((jv*TG)+(jj*((NN-((jo*TN)+(jk*(PX+((jh*zw)+(eY*Tm))))))+((jt*TW)+(jl*(PX+((jr*zA)+(eZ*(-Tm))))))))))))));let a8r=((sf[109]*((li*(-LW))+(le*((Ou+((lf*LI)+(ii*((if sb[12]{(a4F+((kY*a54)+(kR*(((kB*a2s)+(kw*(a1N+((kv*O2)+(iz*a2o)))))+(sf[21]*a1N)))))}else{(if (sf[101]!=0.0){(a4F-(if (sf[101]!=0.0){((kF*O2)+(iz*a3T))}else{b}))}else{b})})+((l6*O2)+(iz*a6B))))))-((jG*TH)+(jj*((O2-((jz*TP)+(jk*(Qi+((ji*zx)+(eY*TB))))))+((jE*TY)+(jl*(Qi+((jC*zB)+(eZ*(-TB)))))))))))))+(sf[109]*((lc*LW)+(ik*((Oi+((l9*LI)+(ii*((if sb[12]{(a4t+((kT*a54)+(kR*(((ky*a2s)+(kw*(a15+((kv*NQ)+(iy*a2o)))))+(sf[21]*a15)))))}else{(if (sf[101]!=0.0){(a4t-(if (sf[101]!=0.0){((kF*NQ)+(iy*a3T))}else{b}))}else{b})})+((l6*NQ)+(iy*a6B))))))-((jv*TH)+(jj*((NQ-((jo*TP)+(jk*(PY+((jh*zx)+(eY*Tp))))))+((jt*TY)+(jl*(PY+((jr*zB)+(eZ*(-Tp))))))))))))));let a8s=((sf[109]*((li*(-LZ))+(le*((Ox+((lf*LJ)+(ii*((if sb[12]{(a4G+((kY*a55)+(kR*(((kB*a2t)+(kw*(a1O+((kv*O5)+(iz*a2p)))))+(sf[21]*a1O)))))}else{(if (sf[101]!=0.0){(a4G-(if (sf[101]!=0.0){((kF*O5)+(iz*a3U))}else{b}))}else{b})})+((l6*O5)+(iz*a6E))))))-((jG*TI)+(jj*((O5-((jz*TR)+(jk*(Qj+((ji*zy)+(eY*TE))))))+((jE*U0)+(jl*(Qj+((jC*zC)+(eZ*(-TE)))))))))))))+(sf[109]*((lc*LZ)+(ik*((Ol+((l9*LJ)+(ii*((if sb[12]{(a4u+((kT*a55)+(kR*(((ky*a2t)+(kw*(a16+((kv*NT)+(iy*a2p)))))+(sf[21]*a16)))))}else{(if (sf[101]!=0.0){(a4u-(if (sf[101]!=0.0){((kF*NT)+(iy*a3U))}else{b}))}else{b})})+((l6*NT)+(iy*a6E))))))-((jv*TI)+(jj*((NT-((jo*TR)+(jk*(PZ+((jh*zy)+(eY*Ts))))))+((jt*U0)+(jl*(PZ+((jr*zC)+(eZ*(-Ts))))))))))))));let a8u=(ls*ls);let a8E=((lt*((ik*LG)+(ii*LQ)))+(il*((-a8p)/a8u)));let a8H=((lt*((ik*LH)+(ii*LT)))+(il*((-a8q)/a8u)));let a8K=((lt*((ik*LI)+(ii*LW)))+(il*((-a8r)/a8u)));let a8N=((lt*((ik*LJ)+(ii*LZ)))+(il*((-a8s)/a8u)));let a8P=(sf[13]*y5);let a8S=(y8-(sf[13]*y4));let a8U=(y9-(sf[13]*y6));let a8V=(ya-(sf[13]*y7));let a8X=(lw*lw);let af3=ddt_scale;let afz=(oU*oU);let afS=(p5*p5);let agd=(ph*ph);let agC=(pO*pO);let agR=(pX*pX);let ah8=(q7*q7);let ajq=(sf[20]*(acm*af3));let ajr=(sf[20]*(acp*af3));let ajs=(sf[20]*(acs*af3));let ajt=(sf[20]*(acv*af3));let ajy=(sf[20]*(adI*af3));let ajz=(sf[20]*(adL*af3));let ajA=(sf[20]*(adO*af3));let ajB=(sf[20]*(adR*af3));
        let ajG=(sf[20]*(if lS{b}else{(if (lA!=0.0){((lP*a8E)+(lu*(if (lA!=0.0){((lN*(if (lA!=0.0){(lL*(if lJ{b}else{(if (lA!=0.0){(sf[250]*(if (lA!=0.0){((-a8S)/a8X)}else{b}))}else{b})}))}else{b}))+(lM*(sf[224]*a8S)))}else{b})))}else{b})}));let ajH=(sf[20]*(if lS{b}else{(if (lA!=0.0){((lP*a8H)+(lu*(if (lA!=0.0){((lN*(if (lA!=0.0){(lL*(if lJ{b}else{(if (lA!=0.0){(sf[250]*(if (lA!=0.0){(a8P/a8X)}else{b}))}else{b})}))}else{b}))+(lM*(sf[224]*(-a8P))))}else{b})))}else{b})}));let ajI=(sf[20]*(if lS{b}else{(if (lA!=0.0){((lP*a8K)+(lu*(if (lA!=0.0){((lN*(if (lA!=0.0){(lL*(if lJ{b}else{(if (lA!=0.0){(sf[250]*(if (lA!=0.0){((-a8U)/a8X)}else{b}))}else{b})}))}else{b}))+(lM*(sf[224]*a8U)))}else{b})))}else{b})}));let ajJ=(sf[20]*(if lS{b}else{(if (lA!=0.0){((lP*a8N)+(lu*(if (lA!=0.0){((lN*(if (lA!=0.0){(lL*(if lJ{b}else{(if (lA!=0.0){(sf[250]*(if (lA!=0.0){((-a8V)/a8X)}else{b}))}else{b})}))}else{b}))+(lM*(sf[224]*a8V)))}else{b})))}else{b})}));

        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * ((lu*rK)),
            &[(rK*a8E),(rK*a8H),(rK*a8K),(rK*a8N)],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * ((if (mN!=0.0){rM}else{b})),
            &[(if (mN!=0.0){ajq}else{b}),(if (mN!=0.0){ajr}else{b}),(if (mN!=0.0){ajs}else{b}),(if (mN!=0.0){ajt}else{b})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * ((if (mN!=0.0){rO}else{b})),
            &[(if (mN!=0.0){ajy}else{b}),(if (mN!=0.0){ajz}else{b}),(if (mN!=0.0){ajA}else{b}),(if (mN!=0.0){ajB}else{b})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * ((if (mN!=0.0){rQ}else{b})),
            &[(if (mN!=0.0){ajG}else{b}),(if (mN!=0.0){ajH}else{b}),(if (mN!=0.0){ajI}else{b}),(if (mN!=0.0){ajJ}else{b})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * ((if rS{rM}else{b})),
            &[(if rS{ajq}else{b}),(if rS{ajr}else{b}),(if rS{ajs}else{b}),(if rS{ajt}else{b})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * ((if rS{rO}else{b})),
            &[(if rS{ajy}else{b}),(if rS{ajz}else{b}),(if rS{ajA}else{b}),(if rS{ajB}else{b})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * ((if rS{rQ}else{b})),
            &[(if rS{ajG}else{b}),(if rS{ajH}else{b}),(if rS{ajI}else{b}),(if rS{ajJ}else{b})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(1),
            Some(3),
            multiplicity * ((sf[20]*rW)),
            &[(sf[20]*(aeZ*af3)),(sf[20]*(af0*af3)),(sf[20]*(af1*af3)),(sf[20]*(af2*af3))],
            &[],
            multiplicity,
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (b),
        );
        stamper.stamp_current_node2_local(
            Some(0),
            Some(3),
            multiplicity * ((sf[90]*(sf[20]*((((sf[291]*(oW-F))-(sf[286]*(p7-F)))-(sf[285]*(pj-F)))+((oJ*s0)+(cj*sf[168])))))),
            0,
            multiplicity * ((sf[90]*(sf[20]*((((sf[291]*(oW*(((oU*sf[312])-(oQ*(if oT{sf[20]}else{b})))/afz)))-(sf[286]*(p7*(((p5*sf[316])-(p2*(if p4{sf[20]}else{b})))/afS))))-(sf[285]*(pj*(((ph*sf[320])-(pe*(if pg_{sf[20]}else{b})))/agd))))+(((s0*(if oD{(sf[157]*(oG*sf[308]))}else{b}))+(oJ*(sf[289]*(-(rY*(if (ot!=0.0){b}else{sf[306]}))))))+sf[176]))))),
            3,
            multiplicity * ((sf[90]*(sf[20]*((((sf[291]*(oW*(((oU*sf[313])-(oQ*(if oT{sf[169]}else{b})))/afz)))-(sf[286]*(p7*(((p5*sf[317])-(p2*(if p4{sf[169]}else{b})))/afS))))-(sf[285]*(pj*(((ph*sf[321])-(pe*(if pg_{sf[169]}else{b})))/agd))))+(((s0*(if oD{(sf[157]*(oG*sf[309]))}else{b}))+(oJ*(sf[289]*(-(rY*(if (ot!=0.0){b}else{sf[307]}))))))+sf[177]))))),
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * ((sf[90]*(sf[20]*((((sf[291]*(pQ-F))-(sf[296]*(pZ-F)))-(sf[295]*(q9-F)))+((pI*sa)+(cg*sf[168])))))),
            2,
            multiplicity * ((sf[90]*(sf[20]*((((sf[291]*(pQ*(((pO*sf[312])-(pL*(if pN{sf[20]}else{b})))/agC)))-(sf[296]*(pZ*(((pX*sf[316])-(pU*(if pW{sf[20]}else{b})))/agR))))-(sf[295]*(q9*(((q7*sf[320])-(q4*(if q6{sf[20]}else{b})))/ah8))))+(sf[176]+((sa*(if pD{(sf[157]*(pF*sf[308]))}else{b}))+(pI*(sf[298]*(-(s8*(if (pv!=0.0){b}else{sf[306]}))))))))))),
            3,
            multiplicity * ((sf[90]*(sf[20]*((((sf[291]*(pQ*(((pO*sf[313])-(pL*(if pN{sf[169]}else{b})))/agC)))-(sf[296]*(pZ*(((pX*sf[317])-(pU*(if pW{sf[169]}else{b})))/agR))))-(sf[295]*(q9*(((q7*sf[321])-(q4*(if q6{sf[169]}else{b})))/ah8))))+(sf[177]+((sa*(if pD{(sf[157]*(pF*sf[309]))}else{b}))+(pI*(sf[298]*(-(s8*(if (pv!=0.0){b}else{sf[307]}))))))))))),
        );
        stamper.stamp_current_node2_local(
            Some(0),
            Some(3),
            multiplicity * ((sf[90]*(sf[20]*sh))),
            0,
            multiplicity * ((sf[90]*(sf[20]*(af3*aiu)))),
            3,
            multiplicity * ((sf[90]*(sf[20]*(af3*aix)))),
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * ((sf[90]*(sf[20]*sk))),
            2,
            multiplicity * ((sf[90]*(sf[20]*(af3*aji)))),
            3,
            multiplicity * ((sf[90]*(sf[20]*(af3*ajl)))),
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
            b, z, F, cg, cj, ct, cz, cI,
            cL, cP, cS, di, dk, do_, ds, ep,
            er, es, ew, ey, ez, eA, eG, eV,
            eY, eZ, f2, f5, fY, h6, h9, ha,
            hb, hc, hd, he, hf, hg, hi, hy,
            in_, k6, k9, mk, mx, mJ, mN, r1,
            rJ, rS, sL, sM, sN, sV, sW, sX,
            t5, t6, t7, tf, tg, th, tN, tO,
            tP, tQ, tV, tW, tX, tY, ui, uj,
            uk, ul, uG, uH, uI, uJ, xB, xC,
            xD, xE, xH, xK, xN, xQ, xR, xS,
            xT, xU, y0, y1, y2, y3, y4, y5,
            y6, y7, y8, y9, ya, yb, yc, yd,
            yq, yr, ys, yt, zr, zs, zt, zu,
            zv, zw, zx, zy, zz, zA, zB, zC,
            zQ, zR, zS, zT, A7, A8, A9, Aa,
            Dd, De, Df, Dg, H8, H9, Ha, Hb,
            He, Hh, Hk, Hn, Ho, Hp, Hq, Hr,
            Hs, Ht, Hu, Hv, Hw, Hx, Hy, Hz,
            HB, HD, HF, HH, HM, HN, HO, HP,
            YO, acm, acp, acs, acv, adI, adL, adO,
            adR, aeZ, af0, af1, af2, aiu, aix, aji,
            ajl,
        }=self.eval_common_stamp_values(ctx);
        let mK=0.0;let mL=0.0;let rM=(sf[20]*mK);let rO=(sf[20]*mL);let rW=0.0;let sh=0.0;let sk=0.0;let af3=1.0;let ajq=(sf[20]*(acm*af3));let ajr=(sf[20]*(acp*af3));let ajs=(sf[20]*(acs*af3));let ajt=(sf[20]*(acv*af3));let ajy=(sf[20]*(adI*af3));let ajz=(sf[20]*(adL*af3));let ajA=(sf[20]*(adO*af3));let ajB=(sf[20]*(adR*af3));

        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[3]),
            &nodes,
            &[(if (mN!=0.0){ajq}else{b}),(if (mN!=0.0){ajr}else{b}),(if (mN!=0.0){ajs}else{b}),(if (mN!=0.0){ajt}else{b})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[3]),
            &nodes,
            &[(if (mN!=0.0){ajy}else{b}),(if (mN!=0.0){ajz}else{b}),(if (mN!=0.0){ajA}else{b}),(if (mN!=0.0){ajB}else{b})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[3]),
            &nodes,
            &[(if rS{ajq}else{b}),(if rS{ajr}else{b}),(if rS{ajs}else{b}),(if rS{ajt}else{b})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[3]),
            &nodes,
            &[(if rS{ajy}else{b}),(if rS{ajz}else{b}),(if rS{ajA}else{b}),(if rS{ajB}else{b})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[3]),
            &nodes,
            &[(sf[20]*(aeZ*af3)),(sf[20]*(af0*af3)),(sf[20]*(af1*af3)),(sf[20]*(af2*af3))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[3]),
            nodes[0],
            multiplicity * ((sf[90]*(sf[20]*(af3*aiu)))),
            nodes[3],
            multiplicity * ((sf[90]*(sf[20]*(af3*aix)))),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[2]),
            Some(nodes[3]),
            nodes[2],
            multiplicity * ((sf[90]*(sf[20]*(af3*aji)))),
            nodes[3],
            multiplicity * ((sf[90]*(sf[20]*(af3*ajl)))),
        );
    }
}
