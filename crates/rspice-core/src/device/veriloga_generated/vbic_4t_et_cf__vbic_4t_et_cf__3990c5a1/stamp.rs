#![allow(dead_code, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

const LIMEXP_MAX: f64 = 5.54062238439351e34;

#[inline]
fn scalar_limexp(arg: f64) -> f64 {
    if arg < 80.0 { arg.exp() } else { LIMEXP_MAX * (1.0 + arg - 80.0) }
}

#[inline]
fn scalar_limexp_derivative(arg: f64) -> f64 {
    if arg < 80.0 { arg.exp() } else { LIMEXP_MAX }
}

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
    g: f64, l: f64, m_: f64, n_: f64, Z: f64, a0: f64,
    aw: f64, c6: f64, c9: f64, cx: f64, d9: f64, e4: f64,
    eD: f64, eE: f64, eF: f64, eG: f64, eH: f64, eI: f64,
    eJ: f64, eK: f64, eM: f64, eN: f64, f0: f64, mD: f64,
    mE: f64, nC: f64, nX: f64, o3: f64, oD: f64, oI: f64,
    oK: f64, oM: f64, oO: f64, oQ: f64, oY: f64, s5: f64,
    sb_: f64, sc: f64, sh: f64, sk: f64, sT: f64, t9: f64,
    ui: f64, uk: f64, ur: f64, us: f64, uv: f64, uz: f64,
    uC: f64, uF: f64, vi: f64, vj: f64, w8: f64, wO: f64,
    zQ: f64, ZH: f64, ZI: f64, ZJ: f64, ZK: f64, ZL: f64,
    ZM: f64, ZN: f64, a1r: f64, a1s: f64, a1t: f64, a1M: f64,
    a1N: f64, a1O: f64, a3B: f64, a3C: f64, a3D: f64, a3E: f64,
    a3F: f64, a3J: f64, a4k: f64, a4n: f64, a4o: f64, a4p: f64,
    a4q: f64, a4r: f64, a4s: f64, a4z: f64, a4A: f64, a4B: f64,
    a4C: f64, a4D: f64, a4H: f64, a4M: f64, a4N: f64, a4O: f64,
    a57: f64, a58: f64, a59: f64, a5a: f64, a5b: f64, ahY: f64,
    ahZ: f64, ai0: f64, ai2: f64, ai3: f64, ai4: f64, aii: f64,
    aij: f64, aik: f64, air: f64, ais: f64, ait: f64, aoL: f64,
    aoP: f64, aoY: f64, aoZ: f64, ap0: f64, ap7: f64, ap8: f64,
    ap9: f64, apa: f64, apf: f64, aph: f64, apr: f64, aps: f64,
    apt: f64, apu: f64, apv: f64, apw: f64, apD: f64, apK: f64,
    apL: f64, apM: f64, apN: f64, apO: f64, apR: f64, apS: f64,
    apT: f64, apU: f64, apV: f64, apZ: f64, aq0: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let g=ctx.node_voltage(n[4]);let h=(sf[321]+g);let l=((h*1.3806503e-23)/1.602176462e-19);let m_=(h/sf[1]);let n_=(h-sf[1]);let r=(sf[3]*f64::powf(m_,sf[4]));let W=f64::powf(m_,sf[20]);let Z=1.0;let a0=(Z-m_);let a1=(sf[22]*a0);let a3=((a1/l)).exp();let a4=(W*a3);let a8=(sf[19]*f64::powf(a4,sf[24]));let ab=f64::powf(m_,sf[26]);let ae=(a0*sf[28]);let ag=((ae/l)).exp();let ah=(ab*ag);let al=(sf[25]*f64::powf(ah,sf[30]));let ap=(a0*sf[33]);let ar=((ap/l)).exp();let as_=(W*ar);let aw=(sf[31]*f64::powf(as_,sf[35]));let bL=(Z+(n_*sf[70]));let bM=(sf[23]*bL);let bN=(sf[29]*bL);let c6=2.0;let c8=(c6*(l/m_));let c9=0.5;let cc=(m_*sf[79]);let ce=((cc/l)).exp();let cf=-0.5;let ch=(m_*sf[80]);let cj=((ch/l)).exp();let ck=(ce-cj);let cl=(ck).ln();let cm=(c8*cl);let cp=(l*3.0);let cq=(m_).ln();let cr=(cp*cq);let ct=(m_-Z);let cv=(((m_*cm)-cr)-(sf[38]*ct));let cw=(l*c6);let cx=4.0;let cy=(-cv);let cA=((cy/l)).exp();let cD=((Z+(cx*cA))).sqrt();let cF=(c9*(Z+cD));let cG=(cF).ln();let cI=(cv+(cw*cG));let cL=(m_*sf[82]);let cN=((cL/l)).exp();let cP=(m_*sf[83]);let cR=((cP/l)).exp();let cS=(cN-cR);let cT=(cS).ln();let cU=(c8*cT);let cY=(((m_*cU)-cr)-(sf[49]*ct));let cZ=(-cY);let d1=((cZ/l)).exp();let d4=((Z+(cx*d1))).sqrt();let d6=(c9*(Z+d4));let d7=(d6).ln();let d9=(cY+(cw*d7));let dc=(m_*sf[85]);let de=((dc/l)).exp();let dg=(m_*sf[86]);let di=((dg/l)).exp();let dj=(de-di);let dk=(dj).ln();let dl=(c8*dk);let dp=(((m_*dl)-cr)-(sf[61]*ct));let dq=(-dp);let ds=((dq/l)).exp();let dv=((Z+(cx*ds))).sqrt();let dx=(c9*(Z+dv));let dy=(dx).ln();let dA=(dp+(cw*dy));let dC=(sf[78]/cI);let dF=(sf[87]*f64::powf(dC,sf[88]));let dH=(sf[81]/d9);let dJ=f64::powf(dH,sf[90]);let dK=(sf[89]*dJ);let dM=(dJ*sf[91]);let dO=(sf[84]/dA);let dR=(sf[92]*f64::powf(dO,sf[93]));let dT=(W*sf[94]);let dU=(a3*dT);let e4=0.0;let ee=(if sb[2]{(Z/r)}else{e4});let eD=ctx.node_voltage(n[8]);let eE=ctx.node_voltage(n[9]);let eF=(eD-eE);let eG=ctx.node_voltage(n[7]);let eH=(eG-eE);let eI=ctx.node_voltage(n[6]);let eJ=(eD-eI);let eK=ctx.node_voltage(n[5]);let eL=(eD-eK);let eM=ctx.node_voltage(n[10]);let eN=(eG-eM);let eO=(-cI);let eQ=(eO*sf[119]);let eU=(eF+eQ);let eV=(if (sf[121]!=0.0){eU}else{e4});let eX=(if (eV>e4){Z}else{e4});let eY=((sf[121]!=0.0)&&(eX!=0.0));let f0=-1.0;let f3=(if eY{sf[124]}else{e4});let f6=(Z-(sf[122]*(sf[122]*f3)));let fc=(eV*sf[126]);let fe=(sf[122]+(fc/cI));let fj=((sf[121]!=0.0)&&(!(eX!=0.0)));let fl=(Z-(eF/cI));let fn_=(Z-f64::powf(fl,sf[125]));let fq=(if fj{((cI*fn_)/sf[125])}else{(if eY{((cI*f6)/sf[125])}else{e4})});let fr=(if fj{e4}else{(if eY{(f3*(eV*fe))}else{e4})});let fz=(((eQ*eQ)+sf[128])).sqrt();let fA=(if sb[10]{fz}else{e4});let fD=(if sb[10]{(cf*(eQ+fA))}else{e4});let fF=(Z-(fD/cI));let fG=f64::powf(fF,sf[125]);let fJ=(if sb[10]{((eO*fG)/sf[125])}else{e4});let fK=(if sb[10]{eU}else{e4});let fN=((sf[128]+(fK*fK))).sqrt();let fO=(if sb[10]{fN}else{e4});let fS=(if sb[10]{((c9*(fK-fO))-eQ)}else{e4});let fU=(Z-(fS/cI));let fV=f64::powf(fU,sf[125]);let fY=(if sb[10]{((eO*fV)/sf[125])}else{fq});let g6=(if sb[10]{((fY+(sf[130]*(fD+(eF-fS))))-fJ)}else{(if (sf[121]!=0.0){(fq+fr)}else{e4})});let g7=(eH+eQ);let g8=(if (sf[121]!=0.0){g7}else{eV});let ga=(if (g8>e4){Z}else{e4});let gb=((sf[121]!=0.0)&&(ga!=0.0));let gc=(if gb{sf[124]}else{f3});let gf=(Z-(sf[122]*(sf[122]*gc)));let gj=(sf[126]*g8);let gl=(sf[122]+(gj/cI));let gq=((sf[121]!=0.0)&&(!(ga!=0.0)));let gs=(Z-(eH/cI));let gu=(Z-f64::powf(gs,sf[125]));let gx=(if gq{((cI*gu)/sf[125])}else{(if gb{((cI*gf)/sf[125])}else{fY})});let gy=(if gq{e4}else{(if gb{(gc*(g8*gl))}else{fr})});let gB=(if sb[10]{fz}else{fA});let gE=(if sb[10]{(cf*(eQ+gB))}else{fD});let gG=(Z-(gE/cI));let gH=f64::powf(gG,sf[125]);let gK=(if sb[10]{((eO*gH)/sf[125])}else{fJ});let gL=(if sb[10]{g7}else{fK});let gO=((sf[128]+(gL*gL))).sqrt();let gP=(if sb[10]{gO}else{fO});let gT=(if sb[10]{((c9*(gL-gP))-eQ)}else{fS});let gV=(Z-(gT/cI));let gW=f64::powf(gV,sf[125]);let gZ=(if sb[10]{((eO*gW)/sf[125])}else{gx});let h5=(if sb[10]{((gZ+(sf[130]*(gE+(eH-gT))))-gK)}else{(if (sf[121]!=0.0){(gx+gy)}else{e4})});let h6=(-d9);
        let h7=(sf[119]*h6);let hb=(eJ+h7);let hc=(if (sf[132]!=0.0){hb}else{g8});let he=(if (hc>e4){Z}else{e4});let hf=((sf[132]!=0.0)&&(he!=0.0));let hi=(if hf{sf[134]}else{gc});let hl=(Z-(sf[122]*(sf[122]*hi)));let hr=(hc*sf[136]);let ht=(sf[122]+(hr/d9));let hC=(if (sb[12]&&(eJ<sf[138])){Z}else{e4});let hE=((sf[132]!=0.0)&&(!(he!=0.0)));let hF=((hC!=0.0)&&hE);let hH=(Z+(sf[137]/d9));let hI=f64::powf(hH,sf[135]);let hK=(sf[135]*(eJ+sf[137]));let hL=(d9+sf[137]);let hN=(Z-(hK/hL));let hP=(Z-(hI*hN));let hU=(hE&&(!(hC!=0.0)));let hW=(Z-(eJ/d9));let hY=(Z-f64::powf(hW,sf[135]));let i1=(if hU{((d9*hY)/sf[135])}else{(if hF{((d9*hP)/sf[135])}else{(if hf{((d9*hl)/sf[135])}else{gZ})})});let i2=(if hE{e4}else{(if hf{(hi*(hc*ht))}else{gy})});let ib=(h7+sf[137]);let ic=(sf[137]-h7);let id=(ib/ic);let ie=(if sb[16]{id}else{e4});let if_=(c6*ie);let ig=(ie-Z);let il=(((ig*ig)+sf[142])).sqrt();let im=(Z+ie);let ir=(((im*im)+sf[144])).sqrt();let is=(il+ir);let iu=(if sb[16]{(if_/is)}else{e4});let iz=(if sb[16]{(c9*(((ic*iu)-sf[137])-h7))}else{gE});let iB=(Z-(iz/d9));let iD=(Z-f64::powf(iB,sf[135]));let iG=(if sb[16]{((d9*iD)/sf[135])}else{e4});let iJ=(h7+(sf[137]+(c6*eJ)));let iL=(if sb[16]{(iJ/ic)}else{e4});let iM=(c6*iL);let iN=(iL-Z);let iQ=((sf[142]+(iN*iN))).sqrt();let iR=(Z+iL);let iU=((sf[144]+(iR*iR))).sqrt();let iV=(iQ+iU);let iX=(if sb[16]{(iM/iV)}else{e4});let j2=(if sb[16]{(c9*(((ic*iX)-sf[137])-h7))}else{gT});let j4=(Z-(j2/d9));let j6=(Z-f64::powf(j4,sf[135]));let j9=(if sb[16]{((d9*j6)/sf[135])}else{i1});let jc=(if sb[16]{(c9*(Z+iX))}else{e4});let je=f64::powf(hH,sf[145]);let jf=(if sb[16]{je}else{e4});let jh=(Z+(h7/d9));let ji=f64::powf(jh,sf[145]);let jj=(if sb[16]{ji}else{e4});let jk=(Z-jc);let jo=(if sb[16]{((jf*jk)+(jc*jj))}else{e4});let jq=(iz+(eJ-j2));let js=(if sb[16]{(jo*jq)}else{e4});let jA=((sf[142]+(h7*h7))).sqrt();let jB=(if sb[18]{jA}else{gB});let jE=(if sb[18]{(cf*(h7+jB))}else{iz});let jG=(Z-(jE/d9));let jH=f64::powf(jG,sf[135]);let jK=(if sb[18]{((h6*jH)/sf[135])}else{gK});let jL=(if sb[18]{hb}else{gL});let jO=((sf[142]+(jL*jL))).sqrt();let jP=(if sb[18]{jO}else{gP});let jT=(if sb[18]{((c9*(jL-jP))-h7)}else{j2});let jV=(Z-(jT/d9));let jW=f64::powf(jV,sf[135]);let jZ=(if sb[18]{((h6*jW)/sf[135])}else{j9});let k6=(if sb[18]{((jZ+(sf[146]*(jE+(eJ-jT))))-jK)}else{(if sb[16]{((j9+js)-iG)}else{(if (sf[132]!=0.0){(i1+i2)}else{e4})})});let k7=(eN+h7);let k8=(if (sf[132]!=0.0){k7}else{hc});let ka=(if (k8>e4){Z}else{e4});let kb=((sf[132]!=0.0)&&(ka!=0.0));let kc=(if kb{sf[134]}else{hi});let kf=(Z-(sf[122]*(sf[122]*kc)));let kj=(sf[136]*k8);let kl=(sf[122]+(kj/d9));let kr=(if (sb[12]&&(eN<sf[138])){Z}else{e4});let kt=((sf[132]!=0.0)&&(!(ka!=0.0)));let ku=((kr!=0.0)&&kt);let kw=(sf[135]*(eN+sf[137]));let ky=(Z-(kw/hL));let kA=(Z-(hI*ky));let kF=(kt&&(!(kr!=0.0)));let kH=(Z-(eN/d9));let kJ=(Z-f64::powf(kH,sf[135]));let kM=(if kF{((d9*kJ)/sf[135])}else{(if ku{((d9*kA)/sf[135])}else{(if kb{((d9*kf)/sf[135])}else{jZ})})});let kN=(if kt{e4}else{(if kb{(kc*(k8*kl))}else{i2})});let kQ=(if sb[16]{id}else{ie});let kR=(c6*kQ);let kS=(kQ-Z);let kV=((sf[142]+(kS*kS))).sqrt();let kW=(Z+kQ);let kZ=((sf[144]+(kW*kW))).sqrt();let l0=(kV+kZ);let l2=(if sb[16]{(kR/l0)}else{iu});let l7=(if sb[16]{(c9*(((ic*l2)-sf[137])-h7))}else{jE});let l9=(Z-(l7/d9));let lb=(Z-f64::powf(l9,sf[135]));let lh=(h7+(sf[137]+(c6*eN)));let lj=(if sb[16]{(lh/ic)}else{iL});let lk=(c6*lj);let ll=(lj-Z);let lo=((sf[142]+(ll*ll))).sqrt();let lp=(Z+lj);let ls=((sf[144]+(lp*lp))).sqrt();let lt=(lo+ls);let lv=(if sb[16]{(lk/lt)}else{iX});let lA=(if sb[16]{(c9*(((ic*lv)-sf[137])-h7))}else{jT});let lC=(Z-(lA/d9));let lE=(Z-f64::powf(lC,sf[135]));let lH=(if sb[16]{((d9*lE)/sf[135])}else{kM});let lK=(if sb[16]{(c9*(Z+lv))}else{jc});let lL=(if sb[16]{je}else{jf});let lM=(if sb[16]{ji}else{jj});let lN=(Z-lK);let lR=(if sb[16]{((lL*lN)+(lK*lM))}else{jo});let lT=(l7+(eN-lA));let lZ=(if sb[18]{jA}else{jB});let m2=(if sb[18]{(cf*(h7+lZ))}else{l7});let m4=(Z-(m2/d9));let m5=f64::powf(m4,sf[135]);let m8=(if sb[18]{((h6*m5)/sf[135])}else{jK});let m9=(if sb[18]{k7}else{jL});
        let mc=((sf[142]+(m9*m9))).sqrt();let md=(if sb[18]{mc}else{jP});let mh=(if sb[18]{((c9*(m9-md))-h7)}else{lA});let mj=(Z-(mh/d9));let mk=f64::powf(mj,sf[135]);let mn=(if sb[18]{((h6*mk)/sf[135])}else{lH});let mt=(if sb[18]{((mn+(sf[146]*(m2+(eN-mh))))-m8)}else{(if sb[16]{((lH+(if sb[16]{(lR*lT)}else{js}))-(if sb[16]{((d9*lb)/sf[135])}else{iG}))}else{(if (sf[132]!=0.0){(kM+kN)}else{e4})})});let mw=(-dA);let my=(if (sf[147]!=0.0){(sf[119]*mw)}else{h7});let mD=ctx.node_voltage(n[11]);let mE=(mD-eM);let mF=(my+mE);let mG=(if sb[21]{mF}else{k8});let mI=(if (mG>e4){Z}else{e4});let mJ=(sb[21]&&(mI!=0.0));let mM=(if mJ{sf[151]}else{kc});let mP=(Z-(sf[122]*(sf[122]*mM)));let mV=(mG*sf[153]);let mX=(sf[122]+(mV/dA));let n2=(sb[21]&&(!(mI!=0.0)));let n4=(Z-(mE/dA));let n6=(Z-f64::powf(n4,sf[152]));let n9=(if n2{((dA*n6)/sf[152])}else{(if mJ{((dA*mP)/sf[152])}else{mn})});let nj=(((my*my)+sf[155])).sqrt();let nn=(if sb[23]{(cf*(my+(if sb[23]{nj}else{lZ})))}else{m2});let np=(Z-(nn/dA));let nq=f64::powf(np,sf[152]);let nu=(if sb[23]{mF}else{m9});let nx=((sf[155]+(nu*nu))).sqrt();let nC=(if sb[23]{((c9*(nu-(if sb[23]{nx}else{md})))-my)}else{mh});let nE=(Z-(nC/dA));let nF=f64::powf(nE,sf[152]);let nS=(if sb[24]{e4}else{(if sb[23]{(((if sb[23]{((mw*nF)/sf[152])}else{n9})+(sf[157]*(nn+(mE-nC))))-(if sb[23]{((mw*nq)/sf[152])}else{m8}))}else{(if sb[21]{(n9+(if n2{e4}else{(if mJ{(mM*(mG*mX))}else{kN})}))}else{e4})})});let nT=(l*bM);let nU=(eF/nT);let nW=(scalar_limexp(nU)-Z);let nX=(a8*nW);let nY=(l*bN);let nZ=(eJ/nY);let o0=scalar_limexp(nZ);let o1=(a8*al);let o2=(o0-Z);let o3=(o1*o2);let o7=((Z+(sf[102]*g6))+(sf[99]*k6));let o8=0.0001;let o9=(o7-o8);let od=(((o9*o9)+1e-8)).sqrt();let oh=(o8+(c9*((o7+od)-o8)));let or=(cx*((ee*nX)+(sf[105]*o3)));let os=(f64::powf(oh,sf[161])+or);let oy=(c9*oh);let oz=(Z+or);let oB=(Z+f64::powf(oz,sf[160]));let oD=(if sb[26]{(oy*oB)}else{(if (sf[159]!=0.0){(c9*(oh+f64::powf(os,sf[160])))}else{e4})});let oI=(l*sf[34]);let oK=(if (sf[162]!=0.0){(eN/oI)}else{nZ});let oM=(if (sf[162]!=0.0){scalar_limexp(oK)}else{o0});let oO=(if (sf[162]!=0.0){(eJ/oI)}else{e4});let oQ=(if (sf[162]!=0.0){scalar_limexp(oO)}else{e4});let oW=(((oM*sf[163])+(oQ*sf[164]))-Z);let oY=(if (sf[162]!=0.0){(aw*oW)}else{e4});let s5=ctx.node_voltage(n[0]);let sb_=(eJ/l);let sc=scalar_limexp(sb_);let sd=(eL/l);let se=scalar_limexp(sd);let sh=((Z+(dU*sc))).sqrt();let sk=((Z+(dU*se))).sqrt();let sT=ctx.node_voltage(n[1]);let t9=ctx.node_voltage(n[2]);let tS=(if (nX>e4){Z}else{e4});let tU=(sf[117]*(nX*tS));let tV=(Z+tU);let tW=(tU/tV);let u1=(sf[183]*(Z+(oh*sf[184])));let u5=((sf[114]*eJ)/1.44);let u7=(sf[185]*scalar_limexp(u5));let u9=(sf[118]+(tW*tW));let uc=(Z+(tS*(u7*u9)));let ud=(u1*uc);let ug=(nX*ud);let ui=((sf[165]*(dF*g6))+(ug/oD));let uk=(sf[170]*(dF*h5));let ur=(((dK*k6)+(o3*sf[186]))+(sh*sf[187]));let us=(sk*sf[187]);let uv=((dM*mt)+((if sb[28]{e4}else{oY})*sf[186]));let uz=((dR*nS)+(mE*sf[188]));let uC=((sT-t9)*sf[189]);let uF=((sT-s5)*sf[190]);let vi=(g*sf[193]);let vj=8.617342301212761e-5;let w2=(sf[194]*(sf[20]*f64::powf(m_,sf[203])));let w8=(l*l);let wa=(a3*(((l*sf[205])-(a1*vj))/w8));let wi=(sf[19]*(((a3*w2)+(W*wa))*(sf[24]*f64::powf(a4,sf[206]))));let wO=(sf[31]*(((ar*w2)+(W*(ar*(((l*sf[210])-(ap*vj))/w8))))*(sf[35]*f64::powf(as_,sf[211]))));let yw=(c6*(((m_*vj)-(l*sf[194]))/(m_*m_)));let yV=((cq*0.00025852026903638284)+(cp*(sf[194]/m_)));let yY=((((cm*sf[194])+(m_*((cl*yw)+(c8*(((ce*(((l*sf[230])-(cc*vj))/w8))-(cj*(((l*sf[231])-(ch*vj))/w8)))/ck)))))-yV)-sf[232]);let yZ=0.00017234684602425522;let ze=(yY+((cG*yZ)+(cw*((c9*((cx*(cA*(((l*(-yY))-(cy*vj))/w8)))/(c6*cD)))/cF))));let zB=((((cU*sf[194])+(m_*((cT*yw)+(c8*(((cN*(((l*sf[233])-(cL*vj))/w8))-(cR*(((l*sf[234])-(cP*vj))/w8)))/cS)))))-yV)-sf[235]);let zQ=(zB+((d7*yZ)+(cw*((c9*((cx*(d1*(((l*(-zB))-(cZ*vj))/w8)))/(c6*d4)))/d6))));let Ad=((((dl*sf[194])+(m_*((dk*yw)+(c8*(((de*(((l*sf[236])-(dc*vj))/w8))-(di*(((l*sf[237])-(dg*vj))/w8)))/dj)))))-yV)-sf[238]);let As=(Ad+((dy*yZ)+(cw*((c9*((cx*(ds*(((l*(-Ad))-(dq*vj))/w8)))/(c6*dv)))/dx))));let Av=(cI*cI);
        let AB=(sf[87]*(((-(sf[78]*ze))/Av)*(sf[88]*f64::powf(dC,sf[239]))));let AE=(d9*d9);let AI=(((-(sf[81]*zQ))/AE)*(sf[90]*f64::powf(dH,sf[174])));let AN=(dA*dA);let AX=((dT*wa)+(a3*(sf[94]*w2)));let Bl=(-ze);let Bm=(sf[119]*Bl);let Bn=(if (sf[121]!=0.0){Bm}else{e4});let BA=(sf[244]/cI);let BX=(-(Z/cI));let BY=(-(f0/cI));let C1=(sf[125]*f64::powf(fl,sf[246]));let Cg=(if fj{(((fn_*ze)+(cI*(-((-((-(eF*ze))/Av))*C1))))/sf[125])}else{(if eY{((f6*ze)/sf[125])}else{e4})});let Ch=(if fj{((cI*(-(BX*C1)))/sf[125])}else{e4});let Ci=(if fj{((cI*(-(BY*C1)))/sf[125])}else{e4});let Cj=(if fj{e4}else{(if eY{(f3*((fe*Bn)+(eV*(((cI*(sf[126]*Bn))-(fc*ze))/Av))))}else{e4})});let Ck=(if fj{e4}else{(if eY{(f3*((fe*sf[242])+(eV*BA)))}else{e4})});let Cl=(if fj{e4}else{(if eY{(f3*((fe*sf[243])+(eV*(sf[245]/cI))))}else{e4})});let Cs=(eQ*Bm);let Cv=((Cs+Cs)/(c6*fz));let Cw=(if sb[10]{Cv}else{e4});let Cz=(if sb[10]{(cf*(Bm+Cw))}else{e4});let CM=(if sb[10]{(((fG*Bl)+(eO*((-(((cI*Cz)-(fD*ze))/Av))*(sf[125]*f64::powf(fF,sf[246])))))/sf[125])}else{e4});let CN=(if sb[10]{Bm}else{e4});let CQ=(fK*CN);let CS=(fK*sf[247]);let CU=(fK*sf[248]);let CW=(c6*fN);let D0=(if sb[10]{((CQ+CQ)/CW)}else{e4});let D1=(if sb[10]{((CS+CS)/CW)}else{e4});let D2=(if sb[10]{((CU+CU)/CW)}else{e4});let Da=(if sb[10]{((c9*(CN-D0))-Bm)}else{e4});let Db=(if sb[10]{(c9*(sf[247]-D1))}else{e4});let Dc=(if sb[10]{(c9*(sf[248]-D2))}else{e4});let Dn=(sf[125]*f64::powf(fU,sf[246]));let Dz=(if sb[10]{(((fV*Bl)+(eO*((-(((cI*Da)-(fS*ze))/Av))*Dn)))/sf[125])}else{Cg});let DA=(if sb[10]{((eO*((-(Db/cI))*Dn))/sf[125])}else{Ch});let DB=(if sb[10]{((eO*((-(Dc/cI))*Dn))/sf[125])}else{Ci});let DN=(if sb[10]{((Dz+(sf[130]*(Cz+(-Da))))-CM)}else{(if (sf[121]!=0.0){(Cg+Cj)}else{e4})});let DO=(if sb[10]{(DA+(sf[130]*(Z-Db)))}else{(if (sf[121]!=0.0){(Ch+Ck)}else{e4})});let DP=(if sb[10]{(DB+(sf[130]*(f0-Dc)))}else{(if (sf[121]!=0.0){(Ci+Cl)}else{e4})});let DQ=(if (sf[121]!=0.0){Bm}else{Bn});let Ew=(sf[125]*f64::powf(gs,sf[246]));let EL=(if gq{(((gu*ze)+(cI*(-((-((-(eH*ze))/Av))*Ew))))/sf[125])}else{(if gb{((gf*ze)/sf[125])}else{Dz})});let EM=(if gq{((cI*(-(BX*Ew)))/sf[125])}else{e4});let EN=(if gq{e4}else{(if gb{e4}else{DA})});let EO=(if gq{((cI*(-(BY*Ew)))/sf[125])}else{(if gb{e4}else{DB})});let EP=(if gq{e4}else{(if gb{(gc*((gl*DQ)+(g8*(((cI*(sf[126]*DQ))-(gj*ze))/Av))))}else{Cj})});let EQ=(if gq{e4}else{(if gb{(gc*((gl*sf[242])+(g8*BA)))}else{e4})});let ER=(if gq{e4}else{(if gb{(gc*((gl*sf[249])+(g8*(sf[251]/cI))))}else{Ck})});let ES=(if gq{e4}else{(if gb{(gc*((gl*sf[250])+(g8*(sf[252]/cI))))}else{Cl})});let F1=(if sb[10]{Cv}else{Cw});let F4=(if sb[10]{(cf*(Bm+F1))}else{Cz});let Fh=(if sb[10]{(((gH*Bl)+(eO*((-(((cI*F4)-(gE*ze))/Av))*(sf[125]*f64::powf(gG,sf[246])))))/sf[125])}else{CM});let Fi=(if sb[10]{Bm}else{CN});let Fl=(gL*Fi);let Fn=(gL*sf[247]);let Fp=(gL*sf[253]);let Fr=(gL*sf[254]);let Ft=(c6*gO);let Fy=(if sb[10]{((Fl+Fl)/Ft)}else{D0});let Fz=(if sb[10]{((Fn+Fn)/Ft)}else{e4});let FA=(if sb[10]{((Fp+Fp)/Ft)}else{D1});let FB=(if sb[10]{((Fr+Fr)/Ft)}else{D2});let FL=(if sb[10]{((c9*(Fi-Fy))-Bm)}else{Da});let FM=(if sb[10]{(c9*(sf[247]-Fz))}else{e4});let FN=(if sb[10]{(c9*(sf[253]-FA))}else{Db});let FO=(if sb[10]{(c9*(sf[254]-FB))}else{Dc});let G1=(sf[125]*f64::powf(gV,sf[246]));let Gg=(if sb[10]{(((gW*Bl)+(eO*((-(((cI*FL)-(gT*ze))/Av))*G1)))/sf[125])}else{EL});let Gh=(if sb[10]{((eO*((-(FM/cI))*G1))/sf[125])}else{EM});let Gi=(if sb[10]{((eO*((-(FN/cI))*G1))/sf[125])}else{EN});let Gj=(if sb[10]{((eO*((-(FO/cI))*G1))/sf[125])}else{EO});let GC=(-zQ);let GD=(sf[119]*GC);let GE=(if (sf[132]!=0.0){GD}else{DQ});let GY=(sf[259]/d9);let Ht=((-(sf[137]*zQ))/AE);let Hx=(Ht*(sf[135]*f64::powf(hH,sf[263])));let HB=(hL*hL);let HW=((d9*(-(hI*(-(sf[264]/hL)))))/sf[135]);let HX=((d9*(-(hI*(-(sf[135]/hL)))))/sf[135]);let I9=(-(f0/d9));let Ia=(-(Z/d9));let Ic=(sf[135]*f64::powf(hW,sf[263]));let Ir=(if hU{(((hY*zQ)+(d9*(-((-((-(eJ*zQ))/AE))*Ic))))/sf[135])}else{(if hF{(((hP*zQ)+(d9*(-((hN*Hx)+(hI*(-((-(hK*zQ))/HB)))))))/sf[135])}else{(if hf{((hl*zQ)/sf[135])}else{Gg})})});
        let Is=(if hU{((d9*(-(I9*Ic)))/sf[135])}else{(if hF{HW}else{e4})});let It=(if hU{e4}else{(if hF{e4}else{(if hf{e4}else{Gh})})});let Iu=(if hU{((d9*(-(Ia*Ic)))/sf[135])}else{(if hF{HX}else{(if hf{e4}else{Gi})})});let Iv=(if hU{e4}else{(if hF{e4}else{(if hf{e4}else{Gj})})});let Iw=(if hE{e4}else{(if hf{(hi*((ht*GE)+(hc*(((d9*(sf[136]*GE))-(hr*zQ))/AE))))}else{EP})});let Ix=(if hE{e4}else{(if hf{(hi*((ht*sf[255])+(hc*GY)))}else{e4})});let Iy=(if hE{e4}else{(if hf{(hi*((ht*sf[256])+(hc*(sf[260]/d9))))}else{EQ})});let Iz=(if hE{e4}else{(if hf{(hi*((ht*sf[257])+(hc*(sf[261]/d9))))}else{ER})});let IA=(if hE{e4}else{(if hf{(hi*((ht*sf[258])+(hc*(sf[262]/d9))))}else{ES})});let IL=(-GD);let IM=(ic*GD);let IP=(ic*ic);let IQ=((IM-(ib*IL))/IP);let IR=(if sb[16]{IQ}else{e4});let IT=(ig*IR);let IX=(im*IR);let J7=(if sb[16]{(((is*(c6*IR))-(if_*(((IT+IT)/(c6*il))+((IX+IX)/(c6*ir)))))/(is*is))}else{e4});let Jd=(if sb[16]{(c9*(((iu*IL)+(ic*J7))-GD))}else{F4});let Jr=(if sb[16]{(((iD*zQ)+(d9*(-((-(((d9*Jd)-(iz*zQ))/AE))*(sf[135]*f64::powf(iB,sf[263]))))))/sf[135])}else{e4});let Jy=(if sb[16]{((IM-(iJ*IL))/IP)}else{e4});let Jz=(if sb[16]{(-2.0/ic)}else{e4});let JA=(if sb[16]{(c6/ic)}else{e4});let JC=(c6*Jz);let JD=(c6*JA);let JE=(iN*Jy);let JG=(iN*Jz);let JI=(iN*JA);let JK=(c6*iQ);let JO=(iR*Jy);let JQ=(iR*Jz);let JS=(iR*JA);let JU=(c6*iU);let K4=(iV*iV);let Ke=(if sb[16]{(((iV*(c6*Jy))-(iM*(((JE+JE)/JK)+((JO+JO)/JU))))/K4)}else{e4});let Kf=(if sb[16]{(((iV*JC)-(iM*(((JG+JG)/JK)+((JQ+JQ)/JU))))/K4)}else{e4});let Kg=(if sb[16]{(((iV*JD)-(iM*(((JI+JI)/JK)+((JS+JS)/JU))))/K4)}else{e4});let Kq=(if sb[16]{(c9*(((iX*IL)+(ic*Ke))-GD))}else{FL});let Kr=(if sb[16]{(c9*(ic*Kf))}else{e4});let Ks=(if sb[16]{e4}else{FM});let Kt=(if sb[16]{(c9*(ic*Kg))}else{FN});let Ku=(if sb[16]{e4}else{FO});let KJ=(sf[135]*f64::powf(j4,sf[263]));let L6=(if sb[16]{(((j6*zQ)+(d9*(-((-(((d9*Kq)-(j2*zQ))/AE))*KJ))))/sf[135])}else{Ir});let L7=(if sb[16]{((d9*(-((-(Kr/d9))*KJ)))/sf[135])}else{Is});let L8=(if sb[16]{((d9*(-((-(Ks/d9))*KJ)))/sf[135])}else{It});let L9=(if sb[16]{((d9*(-((-(Kt/d9))*KJ)))/sf[135])}else{Iu});let La=(if sb[16]{((d9*(-((-(Ku/d9))*KJ)))/sf[135])}else{Iv});let Le=(if sb[16]{(c9*Ke)}else{e4});let Lf=(if sb[16]{(c9*Kf)}else{e4});let Lg=(if sb[16]{(c9*Kg)}else{e4});let Lk=(Ht*(sf[145]*f64::powf(hH,sf[265])));let Ll=(if sb[16]{Lk}else{e4});let Ls=((((d9*GD)-(h7*zQ))/AE)*(sf[145]*f64::powf(jh,sf[265])));let Lt=(if sb[16]{Ls}else{e4});let LK=(if sb[16]{(((jk*Ll)+(jf*(-Le)))+((jj*Le)+(jc*Lt)))}else{e4});let LL=(if sb[16]{((jf*(-Lf))+(jj*Lf))}else{e4});let LM=(if sb[16]{((jf*(-Lg))+(jj*Lg))}else{e4});let M4=(if sb[16]{((jq*LK)+(jo*(Jd+(-Kq))))}else{e4});let M5=(if sb[16]{((jq*LL)+(jo*(f0-Kr)))}else{e4});let M6=(if sb[16]{(jo*(-Ks))}else{e4});let M7=(if sb[16]{((jq*LM)+(jo*(Z-Kt)))}else{e4});let M8=(if sb[16]{(jo*(-Ku))}else{e4});let Mk=(h7*GD);let Mn=((Mk+Mk)/(c6*jA));let Mo=(if sb[18]{Mn}else{F1});let Mr=(if sb[18]{(cf*(GD+Mo))}else{Jd});let ME=(if sb[18]{(((jH*GC)+(h6*((-(((d9*Mr)-(jE*zQ))/AE))*(sf[135]*f64::powf(jG,sf[263])))))/sf[135])}else{Fh});let MF=(if sb[18]{GD}else{Fi});let MK=(jL*MF);let MM=(jL*sf[266]);let MO=(jL*sf[267]);let MQ=(jL*sf[268]);let MS=(jL*sf[269]);let MU=(c6*jO);let N0=(if sb[18]{((MK+MK)/MU)}else{Fy});let N1=(if sb[18]{((MM+MM)/MU)}else{e4});let N2=(if sb[18]{((MO+MO)/MU)}else{Fz});let N3=(if sb[18]{((MQ+MQ)/MU)}else{FA});let N4=(if sb[18]{((MS+MS)/MU)}else{FB});let Ng=(if sb[18]{((c9*(MF-N0))-GD)}else{Kq});let Nh=(if sb[18]{(c9*(sf[266]-N1))}else{Kr});let Ni=(if sb[18]{(c9*(sf[267]-N2))}else{Ks});let Nj=(if sb[18]{(c9*(sf[268]-N3))}else{Kt});let Nk=(if sb[18]{(c9*(sf[269]-N4))}else{Ku});let Nz=(sf[135]*f64::powf(jV,sf[263]));let NR=(if sb[18]{(((jW*GC)+(h6*((-(((d9*Ng)-(jT*zQ))/AE))*Nz)))/sf[135])}else{L6});let NS=(if sb[18]{((h6*((-(Nh/d9))*Nz))/sf[135])}else{L7});let NT=(if sb[18]{((h6*((-(Ni/d9))*Nz))/sf[135])}else{L8});let NU=(if sb[18]{((h6*((-(Nj/d9))*Nz))/sf[135])}else{L9});let NV=(if sb[18]{((h6*((-(Nk/d9))*Nz))/sf[135])}else{La});
        let Od=(if sb[18]{((NR+(sf[146]*(Mr+(-Ng))))-ME)}else{(if sb[16]{((L6+M4)-Jr)}else{(if (sf[132]!=0.0){(Ir+Iw)}else{e4})})});let Oe=(if sb[18]{(NS+(sf[146]*(f0-Nh)))}else{(if sb[16]{(L7+M5)}else{(if (sf[132]!=0.0){(Is+Ix)}else{e4})})});let Of=(if sb[18]{(NT+(sf[146]*(-Ni)))}else{(if sb[16]{(L8+M6)}else{(if (sf[132]!=0.0){(It+Iy)}else{e4})})});let Og=(if sb[18]{(NU+(sf[146]*(Z-Nj)))}else{(if sb[16]{(L9+M7)}else{(if (sf[132]!=0.0){(Iu+Iz)}else{e4})})});let Oh=(if sb[18]{(NV+(sf[146]*(-Nk)))}else{(if sb[16]{(La+M8)}else{(if (sf[132]!=0.0){(Iv+IA)}else{e4})})});let Oi=(if (sf[132]!=0.0){GD}else{GE});let Py=(sf[135]*f64::powf(kH,sf[263]));let PN=(if kF{(((kJ*zQ)+(d9*(-((-((-(eN*zQ))/AE))*Py))))/sf[135])}else{(if ku{(((kA*zQ)+(d9*(-((ky*Hx)+(hI*(-((-(kw*zQ))/HB)))))))/sf[135])}else{(if kb{((kf*zQ)/sf[135])}else{NR})})});let PO=(if kF{e4}else{(if ku{e4}else{(if kb{e4}else{NS})})});let PP=(if kF{((d9*(-(Ia*Py)))/sf[135])}else{(if ku{HX}else{(if kb{e4}else{NT})})});let PQ=(if kF{e4}else{(if ku{e4}else{(if kb{e4}else{NU})})});let PR=(if kF{e4}else{(if ku{e4}else{(if kb{e4}else{NV})})});let PS=(if kF{((d9*(-(I9*Py)))/sf[135])}else{(if ku{HW}else{e4})});let PT=(if kt{e4}else{(if kb{(kc*((kl*Oi)+(k8*(((d9*(sf[136]*Oi))-(kj*zQ))/AE))))}else{Iw})});let PU=(if kt{e4}else{(if kb{(kc*((kl*sf[270])+(k8*(sf[274]/d9))))}else{Ix})});let PV=(if kt{e4}else{(if kb{(kc*((kl*sf[271])+(k8*(sf[275]/d9))))}else{Iy})});let PW=(if kt{e4}else{(if kb{(kc*((kl*sf[272])+(k8*(sf[276]/d9))))}else{Iz})});let PX=(if kt{e4}else{(if kb{(kc*((kl*sf[273])+(k8*(sf[277]/d9))))}else{IA})});let PY=(if kt{e4}else{(if kb{(kc*((kl*sf[255])+(k8*GY)))}else{e4})});let Qb=(if sb[16]{IQ}else{IR});let Qd=(kS*Qb);let Qh=(kW*Qb);let Qx=(if sb[16]{(c9*(((l2*IL)+(ic*(if sb[16]{(((l0*(c6*Qb))-(kR*(((Qd+Qd)/(c6*kV))+((Qh+Qh)/(c6*kZ)))))/(l0*l0))}else{J7})))-GD))}else{Mr});let QP=(if sb[16]{((IM-(lh*IL))/IP)}else{Jy});let QQ=(if sb[16]{e4}else{Jz});let QR=(if sb[16]{e4}else{JA});let QV=(ll*QP);let QX=(ll*QQ);let QZ=(ll*JA);let R1=(ll*QR);let R3=(ll*Jz);let R5=(c6*lo);let Rb=(lp*QP);let Rd=(lp*QQ);let Rf=(lp*JA);let Rh=(lp*QR);let Rj=(lp*Jz);let Rl=(c6*ls);let Rz=(lt*lt);let RR=(if sb[16]{(((lt*(c6*QP))-(lk*(((QV+QV)/R5)+((Rb+Rb)/Rl))))/Rz)}else{Ke});let RS=(if sb[16]{(((lt*(c6*QQ))-(lk*(((QX+QX)/R5)+((Rd+Rd)/Rl))))/Rz)}else{Kf});let RT=(if sb[16]{(((lt*JD)-(lk*(((QZ+QZ)/R5)+((Rf+Rf)/Rl))))/Rz)}else{e4});let RU=(if sb[16]{(((lt*(c6*QR))-(lk*(((R1+R1)/R5)+((Rh+Rh)/Rl))))/Rz)}else{Kg});let RV=(if sb[16]{(((lt*JC)-(lk*(((R3+R3)/R5)+((Rj+Rj)/Rl))))/Rz)}else{e4});let S9=(if sb[16]{(c9*(((lv*IL)+(ic*RR))-GD))}else{Ng});let Sa=(if sb[16]{(c9*(ic*RS))}else{Nh});let Sb=(if sb[16]{(c9*(ic*RT))}else{Ni});let Sc=(if sb[16]{(c9*(ic*RU))}else{Nj});let Sd=(if sb[16]{e4}else{Nk});let Se=(if sb[16]{(c9*(ic*RV))}else{e4});let Sv=(sf[135]*f64::powf(lC,sf[263]));let SW=(if sb[16]{(((lE*zQ)+(d9*(-((-(((d9*S9)-(lA*zQ))/AE))*Sv))))/sf[135])}else{PN});let SX=(if sb[16]{((d9*(-((-(Sa/d9))*Sv)))/sf[135])}else{PO});let SY=(if sb[16]{((d9*(-((-(Sb/d9))*Sv)))/sf[135])}else{PP});let SZ=(if sb[16]{((d9*(-((-(Sc/d9))*Sv)))/sf[135])}else{PQ});let T0=(if sb[16]{((d9*(-((-(Sd/d9))*Sv)))/sf[135])}else{PR});let T1=(if sb[16]{((d9*(-((-(Se/d9))*Sv)))/sf[135])}else{PS});let T7=(if sb[16]{(c9*RR)}else{Le});let T8=(if sb[16]{(c9*RS)}else{Lf});let T9=(if sb[16]{(c9*RT)}else{e4});let Ta=(if sb[16]{(c9*RU)}else{Lg});let Tb=(if sb[16]{(c9*RV)}else{e4});let Un=(if sb[18]{Mn}else{Mo});let Uq=(if sb[18]{(cf*(GD+Un))}else{Qx});let UD=(if sb[18]{(((m5*GC)+(h6*((-(((d9*Uq)-(m2*zQ))/AE))*(sf[135]*f64::powf(m4,sf[263])))))/sf[135])}else{ME});let UE=(if sb[18]{GD}else{MF});let UJ=(m9*UE);let UL=(m9*sf[278]);let UN=(m9*sf[279]);let UP=(m9*sf[280]);let UR=(m9*sf[281]);let UT=(m9*sf[266]);let UV=(c6*mc);let V2=(if sb[18]{((UJ+UJ)/UV)}else{N0});let V3=(if sb[18]{((UL+UL)/UV)}else{N1});let V4=(if sb[18]{((UN+UN)/UV)}else{N2});let V5=(if sb[18]{((UP+UP)/UV)}else{N3});let V6=(if sb[18]{((UR+UR)/UV)}else{N4});let V7=(if sb[18]{((UT+UT)/UV)}else{e4});let Vl=(if sb[18]{((c9*(UE-V2))-GD)}else{S9});
        let Vm=(if sb[18]{(c9*(sf[278]-V3))}else{Sa});let Vn=(if sb[18]{(c9*(sf[279]-V4))}else{Sb});let Vo=(if sb[18]{(c9*(sf[280]-V5))}else{Sc});let Vp=(if sb[18]{(c9*(sf[281]-V6))}else{Sd});let Vq=(if sb[18]{(c9*(sf[266]-V7))}else{Se});let VH=(sf[135]*f64::powf(mj,sf[263]));let W2=(if sb[18]{(((mk*GC)+(h6*((-(((d9*Vl)-(mh*zQ))/AE))*VH)))/sf[135])}else{SW});let W3=(if sb[18]{((h6*((-(Vm/d9))*VH))/sf[135])}else{SX});let W4=(if sb[18]{((h6*((-(Vn/d9))*VH))/sf[135])}else{SY});let W5=(if sb[18]{((h6*((-(Vo/d9))*VH))/sf[135])}else{SZ});let W6=(if sb[18]{((h6*((-(Vp/d9))*VH))/sf[135])}else{T0});let W7=(if sb[18]{((h6*((-(Vq/d9))*VH))/sf[135])}else{T1});let Wy=(-As);let WA=(if (sf[147]!=0.0){(sf[119]*Wy)}else{GD});let WB=(if sb[21]{WA}else{Oi});let XQ=(sf[152]*f64::powf(n4,sf[294]));let Y5=(if n2{(((n6*As)+(dA*(-((-((-(mE*As))/AN))*XQ))))/sf[152])}else{(if mJ{((mP*As)/sf[152])}else{W2})});let Y6=(if n2{e4}else{(if mJ{e4}else{W3})});let Y7=(if n2{e4}else{(if mJ{e4}else{W4})});let Y8=(if n2{e4}else{(if mJ{e4}else{W5})});let Y9=(if n2{e4}else{(if mJ{e4}else{W6})});let Ya=(if n2{((dA*(-((-(f0/dA))*XQ)))/sf[152])}else{(if mJ{e4}else{W7})});let Yb=(if n2{((dA*(-((-(Z/dA))*XQ)))/sf[152])}else{e4});let Yx=(my*WA);let YE=(if sb[23]{(cf*(WA+(if sb[23]{((Yx+Yx)/(c6*nj))}else{Un})))}else{Uq});let YS=(if sb[23]{WA}else{UE});let YZ=(nu*YS);let Z1=(nu*sf[295]);let Z3=(nu*sf[296]);let Z5=(nu*sf[297]);let Z7=(nu*sf[298]);let Z9=(nu*sf[299]);let Zb=(nu*sf[300]);let Zd=(c6*nx);let ZH=(if sb[23]{((c9*(YS-(if sb[23]{((YZ+YZ)/Zd)}else{V2})))-WA)}else{Vl});let ZI=(if sb[23]{(c9*(sf[295]-(if sb[23]{((Z1+Z1)/Zd)}else{V3})))}else{Vm});let ZJ=(if sb[23]{(c9*(sf[296]-(if sb[23]{((Z3+Z3)/Zd)}else{V4})))}else{Vn});let ZK=(if sb[23]{(c9*(sf[297]-(if sb[23]{((Z5+Z5)/Zd)}else{V5})))}else{Vo});let ZL=(if sb[23]{(c9*(sf[298]-(if sb[23]{((Z7+Z7)/Zd)}else{V6})))}else{Vp});let ZM=(if sb[23]{(c9*(sf[299]-(if sb[23]{((Z9+Z9)/Zd)}else{V7})))}else{Vq});let ZN=(if sb[23]{(c9*(sf[300]-(if sb[23]{((Zb+Zb)/Zd)}else{e4})))}else{e4});let a06=(sf[152]*f64::powf(nE,sf[294]));let a1l=scalar_limexp_derivative(nU);let a1r=((nW*wi)+(a8*(((-(eF*((bM*vj)+(l*sf[226]))))/(nT*nT))*a1l)));let a1s=(a8*((Z/nT)*a1l));let a1t=(a8*((f0/nT)*a1l));let a1A=((-(eJ*((bN*vj)+(l*sf[227]))))/(nY*nY));let a1B=(f0/nY);let a1C=(Z/nY);let a1D=scalar_limexp_derivative(nZ);let a1E=(a1A*a1D);let a1F=(a1B*a1D);let a1G=(a1C*a1D);let a1M=((o2*((al*wi)+(a8*(sf[25]*(((ag*(sf[194]*(sf[26]*f64::powf(m_,sf[207]))))+(ab*(ag*(((l*sf[208])-(ae*vj))/w8))))*(sf[30]*f64::powf(ah,sf[209])))))))+(o1*a1E));let a1N=(o1*a1F);let a1O=(o1*a1G);let a1T=(sf[99]*Oe);let a1U=(sf[99]*Of);let a1X=((sf[102]*DN)+(sf[99]*Od));let a1Y=((sf[102]*DO)+(sf[99]*Og));let a1Z=((sf[102]*DP)+(sf[99]*Oh));let a20=(o9*a1X);let a22=(o9*a1T);let a24=(o9*a1U);let a26=(o9*a1Y);let a28=(o9*a1Z);let a2a=(c6*od);let a2l=(c9*(a1X+((a20+a20)/a2a)));let a2m=(c9*(a1T+((a22+a22)/a2a)));let a2n=(c9*(a1U+((a24+a24)/a2a)));let a2o=(c9*(a1Y+((a26+a26)/a2a)));let a2p=(c9*(a1Z+((a28+a28)/a2a)));let a2C=(sf[161]*f64::powf(oh,sf[301]));let a2I=(cx*(((nX*(if sb[2]{((-(sf[3]*(sf[194]*(sf[4]*f64::powf(m_,sf[195])))))/(r*r))}else{e4}))+(ee*a1r))+(sf[105]*a1M)));let a2J=(cx*(sf[105]*a1N));let a2K=(cx*((ee*a1s)+(sf[105]*a1O)));let a2L=(cx*(ee*a1t));let a2S=(sf[160]*f64::powf(os,sf[302]));let a3j=(sf[160]*f64::powf(oz,sf[302]));let a3B=(if sb[26]{((oB*(c9*a2l))+(oy*(a2I*a3j)))}else{(if (sf[159]!=0.0){(c9*(a2l+(((a2l*a2C)+a2I)*a2S)))}else{e4})});let a3C=(if sb[26]{((oB*(c9*a2m))+(oy*(a2J*a3j)))}else{(if (sf[159]!=0.0){(c9*(a2m+(((a2m*a2C)+a2J)*a2S)))}else{e4})});let a3D=(if sb[26]{(oB*(c9*a2n))}else{(if (sf[159]!=0.0){(c9*(a2n+((a2n*a2C)*a2S)))}else{e4})});let a3E=(if sb[26]{((oB*(c9*a2o))+(oy*(a2K*a3j)))}else{(if (sf[159]!=0.0){(c9*(a2o+(((a2o*a2C)+a2K)*a2S)))}else{e4})});let a3F=(if sb[26]{((oB*(c9*a2p))+(oy*(a2L*a3j)))}else{(if (sf[159]!=0.0){(c9*(a2p+(((a2p*a2C)+a2L)*a2S)))}else{e4})});let a3J=(oD*oD);let a4k=(oI*oI);let a4n=(f0/oI);let a4o=(if (sf[162]!=0.0){((-(eN*sf[303]))/a4k)}else{a1A});let a4p=(if (sf[162]!=0.0){e4}else{a1B});
        let a4q=(if (sf[162]!=0.0){(Z/oI)}else{e4});let a4r=(if (sf[162]!=0.0){e4}else{a1C});let a4s=(if (sf[162]!=0.0){a4n}else{e4});let a4t=scalar_limexp_derivative(oK);let a4z=(if (sf[162]!=0.0){(a4o*a4t)}else{a1E});let a4A=(if (sf[162]!=0.0){(a4p*a4t)}else{a1F});let a4B=(if (sf[162]!=0.0){(a4q*a4t)}else{e4});let a4C=(if (sf[162]!=0.0){(a4r*a4t)}else{a1G});let a4D=(if (sf[162]!=0.0){(a4s*a4t)}else{e4});let a4H=(if (sf[162]!=0.0){((-(eJ*sf[303]))/a4k)}else{e4});let a4I=scalar_limexp_derivative(oO);let a4M=(if (sf[162]!=0.0){(a4H*a4I)}else{e4});let a4N=(if (sf[162]!=0.0){(a4s*a4I)}else{e4});let a4O=(if (sf[162]!=0.0){(a4q*a4I)}else{e4});let a57=(if (sf[162]!=0.0){((oW*wO)+(aw*((sf[163]*a4z)+(sf[164]*a4M))))}else{e4});let a58=(if (sf[162]!=0.0){(aw*((sf[163]*a4A)+(sf[164]*a4N)))}else{e4});let a59=(if (sf[162]!=0.0){(aw*(sf[163]*a4B))}else{e4});let a5a=(if (sf[162]!=0.0){(aw*((sf[163]*a4C)+(sf[164]*a4O)))}else{e4});let a5b=(if (sf[162]!=0.0){(aw*(sf[163]*a4D))}else{e4});let ahY=((-(eJ*vj))/w8);let ahZ=(f0/l);let ai0=(Z/l);let ai1=scalar_limexp_derivative(sb_);let ai2=(ahY*ai1);let ai3=(ahZ*ai1);let ai4=(ai0*ai1);let ai8=scalar_limexp_derivative(sd);let aih=(c6*sh);let aii=(((sc*AX)+(dU*ai2))/aih);let aij=((dU*ai3)/aih);let aik=((dU*ai4)/aih);let aiq=(c6*sk);let air=(((se*AX)+(dU*(((-(eL*vj))/w8)*ai8)))/aiq);let ais=((dU*(ahZ*ai8))/aiq);let ait=((dU*(ai0*ai8))/aiq);let ank=(sf[117]*(tS*a1r));let anl=(sf[117]*(tS*a1s));let anm=(sf[117]*(tS*a1t));let anq=(tV*tV);let anN=scalar_limexp_derivative(u5);let anS=(tW*(((tV*ank)-(tU*ank))/anq));let anU=(tW*(((tV*anl)-(tU*anl))/anq));let anW=(tW*(((tV*anm)-(tU*anm))/anq));let aoL=(((oD*(nX*((uc*(sf[183]*(sf[184]*a2m)))+(u1*(tS*(u9*(sf[185]*(sf[313]*anN))))))))-(ug*a3C))/a3J);let aoP=(((oD*(nX*(uc*(sf[183]*(sf[184]*a2n)))))-(ug*a3D))/a3J);let aoY=((sf[165]*((g6*AB)+(dF*DN)))+(((oD*((ud*a1r)+(nX*((uc*(sf[183]*(sf[184]*a2l)))+(u1*(tS*(u7*(anS+anS))))))))-(ug*a3B))/a3J));let aoZ=((sf[165]*(dF*DO))+(((oD*((ud*a1s)+(nX*((uc*(sf[183]*(sf[184]*a2o)))+(u1*(tS*((u9*(sf[185]*(sf[314]*anN)))+(u7*(anU+anU)))))))))-(ug*a3E))/a3J));let ap0=((sf[165]*(dF*DP))+(((oD*((ud*a1t)+(nX*((uc*(sf[183]*(sf[184]*a2p)))+(u1*(tS*(u7*(anW+anW))))))))-(ug*a3F))/a3J));let ap7=(sf[170]*((h5*AB)+(dF*(if sb[10]{((Gg+(sf[130]*(F4+(-FL))))-Fh)}else{(if (sf[121]!=0.0){(EL+EP)}else{e4})}))));let ap8=(sf[170]*(dF*(if sb[10]{(Gh+(sf[130]*(Z-FM)))}else{(if (sf[121]!=0.0){(EM+EQ)}else{e4})})));let ap9=(sf[170]*(dF*(if sb[10]{(Gi+(sf[130]*(-FN)))}else{(if (sf[121]!=0.0){(EN+ER)}else{e4})})));let apa=(sf[170]*(dF*(if sb[10]{(Gj+(sf[130]*(f0-FO)))}else{(if (sf[121]!=0.0){(EO+ES)}else{e4})})));let apf=(dK*Of);let aph=(dK*Oh);let apr=((((k6*(sf[89]*AI))+(dK*Od))+(sf[186]*a1M))+(sf[187]*aii));let aps=(((dK*Oe)+(sf[186]*a1N))+(sf[187]*aij));let apt=(((dK*Og)+(sf[186]*a1O))+(sf[187]*aik));let apu=(sf[187]*air);let apv=(sf[187]*ais);let apw=(sf[187]*ait);let apD=(dM*(if sb[18]{(W6+(sf[146]*(-Vp)))}else{(if sb[16]{(T0+(if sb[16]{(lR*(-Sd))}else{M8}))}else{(if (sf[132]!=0.0){(PR+PX)}else{e4})})}));let apK=(((mt*(sf[91]*AI))+(dM*(if sb[18]{((W2+(sf[146]*(Uq+(-Vl))))-UD)}else{(if sb[16]{((SW+(if sb[16]{((lT*(if sb[16]{(((lN*(if sb[16]{Lk}else{Ll}))+(lL*(-T7)))+((lM*T7)+(lK*(if sb[16]{Ls}else{Lt}))))}else{LK}))+(lR*(Qx+(-S9))))}else{M4}))-(if sb[16]{(((lb*zQ)+(d9*(-((-(((d9*Qx)-(l7*zQ))/AE))*(sf[135]*f64::powf(l9,sf[263]))))))/sf[135])}else{Jr}))}else{(if (sf[132]!=0.0){(PN+PT)}else{e4})})})))+(sf[186]*(if sb[28]{e4}else{a57})));let apL=((dM*(if sb[18]{(W3+(sf[146]*(-Vm)))}else{(if sb[16]{(SX+(if sb[16]{((lT*(if sb[16]{((lL*(-T8))+(lM*T8))}else{LL}))+(lR*(-Sa)))}else{M5}))}else{(if (sf[132]!=0.0){(PO+PU)}else{e4})})}))+(sf[186]*(if sb[28]{e4}else{a58})));let apM=((dM*(if sb[18]{(W4+(sf[146]*(Z-Vn)))}else{(if sb[16]{(SY+(if sb[16]{((lT*(if sb[16]{((lL*(-T9))+(lM*T9))}else{e4}))+(lR*(Z-Sb)))}else{M6}))}else{(if (sf[132]!=0.0){(PP+PV)}else{e4})})}))+(sf[186]*(if sb[28]{e4}else{a59})));
        let apN=((dM*(if sb[18]{(W5+(sf[146]*(-Vo)))}else{(if sb[16]{(SZ+(if sb[16]{((lT*(if sb[16]{((lL*(-Ta))+(lM*Ta))}else{LM}))+(lR*(-Sc)))}else{M7}))}else{(if (sf[132]!=0.0){(PQ+PW)}else{e4})})}))+(sf[186]*(if sb[28]{e4}else{a5a})));let apO=((dM*(if sb[18]{(W7+(sf[146]*(f0-Vq)))}else{(if sb[16]{(T1+(if sb[16]{((lT*(if sb[16]{((lL*(-Tb))+(lM*Tb))}else{e4}))+(lR*(f0-Se)))}else{e4}))}else{(if (sf[132]!=0.0){(PS+PY)}else{e4})})}))+(sf[186]*(if sb[28]{e4}else{a5b})));let apR=((nS*(sf[92]*(((-(sf[84]*As))/AN)*(sf[93]*f64::powf(dO,sf[240])))))+(dR*(if sb[24]{e4}else{(if sb[23]{(((if sb[23]{(((nF*Wy)+(mw*((-(((dA*ZH)-(nC*As))/AN))*a06)))/sf[152])}else{Y5})+(sf[157]*(YE+(-ZH))))-(if sb[23]{(((nq*Wy)+(mw*((-(((dA*YE)-(nn*As))/AN))*(sf[152]*f64::powf(np,sf[294])))))/sf[152])}else{UD}))}else{(if sb[21]{(Y5+(if n2{e4}else{(if mJ{(mM*((mX*WB)+(mG*(((dA*(sf[153]*WB))-(mV*As))/AN))))}else{PT})}))}else{e4})})})));let apS=(dR*(if sb[24]{e4}else{(if sb[23]{((if sb[23]{((mw*((-(ZI/dA))*a06))/sf[152])}else{Y6})+(sf[157]*(-ZI)))}else{(if sb[21]{(Y6+(if n2{e4}else{(if mJ{(mM*((mX*sf[282])+(mG*(sf[288]/dA))))}else{PU})}))}else{e4})})}));let apT=(dR*(if sb[24]{e4}else{(if sb[23]{((if sb[23]{((mw*((-(ZJ/dA))*a06))/sf[152])}else{Y7})+(sf[157]*(-ZJ)))}else{(if sb[21]{(Y7+(if n2{e4}else{(if mJ{(mM*((mX*sf[283])+(mG*(sf[289]/dA))))}else{PV})}))}else{e4})})}));let apU=(dR*(if sb[24]{e4}else{(if sb[23]{((if sb[23]{((mw*((-(ZK/dA))*a06))/sf[152])}else{Y8})+(sf[157]*(-ZK)))}else{(if sb[21]{(Y8+(if n2{e4}else{(if mJ{(mM*((mX*sf[284])+(mG*(sf[290]/dA))))}else{PW})}))}else{e4})})}));let apV=(dR*(if sb[24]{e4}else{(if sb[23]{((if sb[23]{((mw*((-(ZL/dA))*a06))/sf[152])}else{Y9})+(sf[157]*(-ZL)))}else{(if sb[21]{(Y9+(if n2{e4}else{(if mJ{(mM*((mX*sf[285])+(mG*(sf[291]/dA))))}else{PX})}))}else{e4})})}));let apZ=((dR*(if sb[24]{e4}else{(if sb[23]{((if sb[23]{((mw*((-(ZM/dA))*a06))/sf[152])}else{Ya})+(sf[157]*(f0-ZM)))}else{(if sb[21]{(Ya+(if n2{e4}else{(if mJ{(mM*((mX*sf[286])+(mG*(sf[292]/dA))))}else{PY})}))}else{e4})})}))+sf[315]);let aq0=(sf[188]+(dR*(if sb[24]{e4}else{(if sb[23]{((if sb[23]{((mw*((-(ZN/dA))*a06))/sf[152])}else{Yb})+(sf[157]*(Z-ZN)))}else{(if sb[21]{(Yb+(if n2{e4}else{(if mJ{(mM*((mX*sf[287])+(mG*(sf[293]/dA))))}else{e4})}))}else{e4})})})));

        CommonStampValues {
            g, l, m_, n_, Z, a0, aw, c6,
            c9, cx, d9, e4, eD, eE, eF, eG,
            eH, eI, eJ, eK, eM, eN, f0, mD,
            mE, nC, nX, o3, oD, oI, oK, oM,
            oO, oQ, oY, s5, sb_, sc, sh, sk,
            sT, t9, ui, uk, ur, us, uv, uz,
            uC, uF, vi, vj, w8, wO, zQ, ZH,
            ZI, ZJ, ZK, ZL, ZM, ZN, a1r, a1s,
            a1t, a1M, a1N, a1O, a3B, a3C, a3D, a3E,
            a3F, a3J, a4k, a4n, a4o, a4p, a4q, a4r,
            a4s, a4z, a4A, a4B, a4C, a4D, a4H, a4M,
            a4N, a4O, a57, a58, a59, a5a, a5b, ahY,
            ahZ, ai0, ai2, ai3, ai4, aii, aij, aik,
            air, ais, ait, aoL, aoP, aoY, aoZ, ap0,
            ap7, ap8, ap9, apa, apf, aph, apr, aps,
            apt, apu, apv, apw, apD, apK, apL, apM,
            apN, apO, apR, apS, apT, apU, apV, apZ,
            aq0,
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let CommonStampValues {
            g, l, m_, n_, Z, a0, aw, c6,
            c9, cx, d9, e4, eD, eE, eF, eG,
            eH, eI, eJ, eK, eM, eN, f0, mD,
            mE, nC, nX, o3, oD, oI, oK, oM,
            oO, oQ, oY, s5, sb_, sc, sh, sk,
            sT, t9, ui, uk, ur, us, uv, uz,
            uC, uF, vi, vj, w8, wO, zQ, ZH,
            ZI, ZJ, ZK, ZL, ZM, ZN, a1r, a1s,
            a1t, a1M, a1N, a1O, a3B, a3C, a3D, a3E,
            a3F, a3J, a4k, a4n, a4o, a4p, a4q, a4r,
            a4s, a4z, a4A, a4B, a4C, a4D, a4H, a4M,
            a4N, a4O, a57, a58, a59, a5a, a5b, ahY,
            ahZ, ai0, ai2, ai3, ai4, aii, aij, aik,
            air, ais, ait, aoL, aoP, aoY, aoZ, ap0,
            ap7, ap8, ap9, apa, apf, aph, apr, aps,
            apt, apu, apv, apw, apD, apK, apL, apM,
            apN, apO, apR, apS, apT, apU, apV, apZ,
            aq0,
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
        let v=(sf[5]*f64::powf(m_,sf[6]));let z=(sf[7]*f64::powf(m_,sf[8]));let D=(sf[9]*f64::powf(m_,sf[10]));let H=(sf[11]*f64::powf(m_,sf[12]));let L=(sf[13]*f64::powf(m_,sf[14]));let P=(sf[15]*f64::powf(m_,sf[16]));let T=(sf[17]*f64::powf(m_,sf[18]));let az=f64::powf(m_,sf[37]);let aC=(a0*sf[39]);let aE=((aC/l)).exp();let aF=(az*aE);let aJ=(sf[36]*f64::powf(aF,sf[41]));let aM=f64::powf(m_,sf[43]);let aP=(a0*sf[45]);let aR=((aP/l)).exp();let aS=(aM*aR);let aW=(sf[42]*f64::powf(aS,sf[47]));let b0=(a0*sf[50]);let b2=((b0/l)).exp();let b3=(az*b2);let b6=f64::powf(b3,sf[52]);let b7=(sf[48]*b6);let bb=(a0*sf[55]);let bd=((bb/l)).exp();let be=(aM*bd);let bh=f64::powf(be,sf[57]);let bi=(sf[53]*bh);let bk=(b6*sf[58]);let bm=(bh*sf[59]);let bq=(a0*sf[62]);let bs=((bq/l)).exp();let bt=(az*bs);let bx=(sf[60]*f64::powf(bt,sf[64]));let bB=(a0*sf[67]);let bD=((bB/l)).exp();let bE=(aM*bD);let bI=(sf[65]*f64::powf(bE,sf[69]));let bW=(n_*sf[75]);let bX=(sf[74]+bW);let c5=(sf[76]*(Z+(n_*sf[77])));let dY=(sf[95]*f64::powf(m_,sf[96]));let dZ=(-(sf[73]*(Z+(n_*bX))));let e0=(l*c5);let e2=((dZ/e0)).exp();let ep=(if sb[5]{(Z/dY)}else{e4});let oE=(o3/oD);let oF=(nX/oD);let p3=((Z+(cx*(if (sf[162]!=0.0){(sf[108]*oY)}else{e4})))).sqrt();let p6=(if (sf[162]!=0.0){(c9*(Z+p3))}else{e4});let p8=(if (sf[162]!=0.0){(mE/oI)}else{oK});let pa=(if (sf[162]!=0.0){scalar_limexp(p8)}else{oM});let pb=(pa-Z);let pe=(oY-(if (sf[162]!=0.0){(aw*pb)}else{e4}));let pj=(if sb[28]{Z}else{p6});let pk=(if sb[28]{e4}else{(if (sf[162]!=0.0){(pe/p6)}else{e4})});let po=(l*sf[40]);let pp=(eF/po);let pq=(if (sf[166]!=0.0){pp}else{p8});let ps=(if (sf[166]!=0.0){scalar_limexp(pq)}else{pa});let pt=(l*sf[46]);let pu=(eF/pt);let pv=(if (sf[166]!=0.0){pu}else{e4});let px=(if (sf[166]!=0.0){scalar_limexp(pv)}else{e4});let pB=(dZ-eF);let pC=(pB/e0);let pD=(if sb[31]{pC}else{oO});let pF=(if sb[31]{scalar_limexp(pD)}else{oQ});let pG=(ps-Z);let pI=(px-Z);let pK=((aJ*pG)+(aW*pI));let pY=(eH/po);let pZ=(if sb[36]{pY}else{pq});let q1=(if sb[36]{scalar_limexp(pZ)}else{ps});let q2=(eH/pt);let q3=(if sb[36]{q2}else{pv});let q5=(if sb[36]{scalar_limexp(q3)}else{px});let q7=(dZ-eH);let q8=(q7/e0);let q9=(if sb[37]{q8}else{pD});let qb=(if sb[37]{scalar_limexp(q9)}else{pF});let qc=(q1-Z);let qe=(q5-Z);let qg=((aJ*qc)+(aW*qe));let qp=(if sb[40]{pp}else{pZ});let qr=(if sb[40]{scalar_limexp(qp)}else{q1});let qs=(if sb[40]{pu}else{q3});let qu=(if sb[40]{scalar_limexp(qs)}else{q5});let qw=(if sb[41]{pC}else{q9});let qy=(if sb[41]{scalar_limexp(qw)}else{qb});let qz=(qr-Z);let qB=(qu-Z);let qD=((aJ*qz)+(aW*qB));let qL=(if sb[42]{(sf[165]*qD)}else{(if sb[41]{(sf[165]*(qD-(sf[168]*(qy-e2))))}else{(if sb[36]{e4}else{(if sb[33]{pK}else{(if sb[31]{(pK-(sf[168]*(pF-e2)))}else{e4})})})})});let qM=(if sb[40]{pY}else{qp});let qP=(if sb[40]{q2}else{qs});let qS=(if sb[41]{q8}else{qw});let qW=((if sb[40]{scalar_limexp(qM)}else{qr})-Z);let qY=((if sb[40]{scalar_limexp(qP)}else{qu})-Z);let r0=((aJ*qW)+(aW*qY));let r7=(if sb[42]{(sf[170]*r0)}else{(if sb[41]{(sf[170]*(r0-(sf[168]*((if sb[41]{scalar_limexp(qS)}else{qy})-e2))))}else{(if sb[38]{qg}else{(if sb[37]{(qg-(sf[168]*(qb-e2)))}else{e4})})})});let r8=(l*sf[51]);let r9=(eJ/r8);let ra=scalar_limexp(r9);let rb=(l*sf[56]);let rc=(eJ/rb);let rd=scalar_limexp(rc);let re=(ra-Z);let rg=(rd-Z);let ri=((b7*re)+(bi*rg));let ro=(if (sf[171]!=0.0){(eN/r8)}else{r9});let rs=(if (sf[171]!=0.0){(eN/rb)}else{rc});let ru=(if (sf[171]!=0.0){scalar_limexp(rs)}else{rd});let rv=((if (sf[171]!=0.0){scalar_limexp(ro)}else{ra})-Z);let rx=(ru-Z);let rC=(if sb[46]{e4}else{(if (sf[171]!=0.0){((bk*rv)+(bm*rx))}else{e4})});let rG=(d9-eJ);let rI=0.01;let rK=(((rG*rG)+rI)).sqrt();let rN=(if (sf[173]!=0.0){(c9*(rG+rK))}else{nC});let rO=(sf[172]*rN);let rP=(-(sf[71]*(Z+(n_*sf[72]))));let rR=f64::powf(rN,sf[174]);let rS=(rP*rR);let rT=scalar_limexp(rS);let rV=(if (sf[173]!=0.0){(rO*rT)}else{e4});let rW=(oF-oE);let rX=(rW-ri);let s2=(ri-(if sb[48]{e4}else{(if (sf[173]!=0.0){(rV*rX)}else{e4})}));let s6=(s5-eK);let sa=(if sb[50]{e4}else{(if (sf[175]!=0.0){(s6/v)}else{e4})});let sn=(Z+sh);
        let so=(Z+sk);let sq=(if (sf[176]!=0.0){(sn/so)}else{e4});let sr=(eK-eI);let su=((sh-sk)-(sq).ln());let sw=(sr+(l*su));let sy=(if (sf[176]!=0.0){(sw/z)}else{e4});let sz=(z*ep);let sA=(sy*sz);let sC=(sf[111]*(c9*ep));let sF=((rI+(sr*sr))).sqrt();let sH=(Z+(sC*sF));let sJ=(if (sf[176]!=0.0){(sA/sH)}else{e4});let sM=((Z+(sJ*sJ))).sqrt();let sQ=(if sb[52]{e4}else{(if (sf[176]!=0.0){(sy/sM)}else{e4})});let sU=(sT-eG);let sY=(if sb[54]{e4}else{(if (sf[177]!=0.0){(sU/D)}else{e4})});let t1=(eG-eD);let t2=(oD*t1);let t6=(if sb[56]{e4}else{(if (sf[178]!=0.0){(t2/H)}else{e4})});let ta=(t9-eE);let te=(if sb[58]{e4}else{(if (sf[179]!=0.0){(ta/L)}else{e4})});let th=(eM-eK);let ti=(pj*th);let tm=(if sb[60]{e4}else{(if (sf[180]!=0.0){(ti/T)}else{e4})});let tr=(l*sf[63]);let tt=(if (sf[181]!=0.0){(mE/tr)}else{sb_});let tw=(l*sf[68]);let ty=(if (sf[181]!=0.0){(mE/tw)}else{rs});let tB=((if (sf[181]!=0.0){scalar_limexp(tt)}else{sc})-Z);let tD=((if (sf[181]!=0.0){scalar_limexp(ty)}else{ru})-Z);let tI=(if sb[64]{e4}else{(if (sf[181]!=0.0){((bx*tB)+(bI*tD))}else{e4})});let tM=(ctx.node_voltage(n[3])-mD);let tQ=(if sb[66]{e4}else{(if (sf[182]!=0.0){(tM/P)}else{e4})});let uJ=(eI-eE);let uU=(eG-mD);let vz=(sf[7]*(sf[194]*(sf[8]*f64::powf(m_,sf[197]))));let wS=(sf[194]*(sf[37]*f64::powf(m_,sf[212])));let x6=(sf[36]*(((aE*wS)+(az*(aE*(((l*sf[213])-(aC*vj))/w8))))*(sf[41]*f64::powf(aF,sf[214]))));let xa=(sf[194]*(sf[43]*f64::powf(m_,sf[215])));let xo=(sf[42]*(((aR*xa)+(aM*(aR*(((l*sf[216])-(aP*vj))/w8))))*(sf[47]*f64::powf(aS,sf[217]))));let xB=(((b2*wS)+(az*(b2*(((l*sf[218])-(b0*vj))/w8))))*(sf[52]*f64::powf(b3,sf[219])));let xP=(((bd*xa)+(aM*(bd*(((l*sf[220])-(bb*vj))/w8))))*(sf[57]*f64::powf(be,sf[221])));let B6=((c5*vj)+(l*sf[229]));let B7=(e0*(-(sf[73]*(bW+bX))));let Ba=(e0*e0);let Bc=(e2*((B7-(dZ*B6))/Ba));let Bk=(if sb[5]{((-(sf[95]*(sf[194]*(sf[96]*f64::powf(m_,sf[241])))))/(dY*dY))}else{e4});let a3K=(((oD*a1M)-(o3*a3B))/a3J);let a3O=(((oD*a1N)-(o3*a3C))/a3J);let a3R=((-(o3*a3D))/a3J);let a3V=(((oD*a1O)-(o3*a3E))/a3J);let a3Y=((-(o3*a3F))/a3J);let a42=(((oD*a1r)-(nX*a3B))/a3J);let a45=((-(nX*a3C))/a3J);let a48=((-(nX*a3D))/a3J);let a4c=(((oD*a1s)-(nX*a3E))/a3J);let a4g=(((oD*a1t)-(nX*a3F))/a3J);let a5r=(c6*p3);let a5C=(if (sf[162]!=0.0){(c9*((cx*(if (sf[162]!=0.0){(sf[108]*a57)}else{e4}))/a5r))}else{e4});let a5D=(if (sf[162]!=0.0){(c9*((cx*(if (sf[162]!=0.0){(sf[108]*a58)}else{e4}))/a5r))}else{e4});let a5E=(if (sf[162]!=0.0){(c9*((cx*(if (sf[162]!=0.0){(sf[108]*a59)}else{e4}))/a5r))}else{e4});let a5F=(if (sf[162]!=0.0){(c9*((cx*(if (sf[162]!=0.0){(sf[108]*a5a)}else{e4}))/a5r))}else{e4});let a5G=(if (sf[162]!=0.0){(c9*((cx*(if (sf[162]!=0.0){(sf[108]*a5b)}else{e4}))/a5r))}else{e4});let a5K=(if (sf[162]!=0.0){((-(mE*sf[303]))/a4k)}else{a4o});let a5L=(if (sf[162]!=0.0){e4}else{a4p});let a5M=(if (sf[162]!=0.0){e4}else{a4q});let a5N=(if (sf[162]!=0.0){e4}else{a4r});let a5O=(if (sf[162]!=0.0){a4n}else{a4s});let a5P=scalar_limexp_derivative(p8);let a5W=(if (sf[162]!=0.0){(a5K*a5P)}else{a4z});let a5X=(if (sf[162]!=0.0){(a5L*a5P)}else{a4A});let a5Y=(if (sf[162]!=0.0){(a5M*a5P)}else{a4B});let a5Z=(if (sf[162]!=0.0){(a5N*a5P)}else{a4C});let a60=(if (sf[162]!=0.0){(a5O*a5P)}else{a4D});let a61=(if (sf[162]!=0.0){(a4q*a5P)}else{e4});let a6p=(p6*p6);let a6Y=(if sb[28]{e4}else{(if (sf[162]!=0.0){(((p6*(a57-(if (sf[162]!=0.0){((pb*wO)+(aw*a5W))}else{e4})))-(pe*a5C))/a6p)}else{e4})});let a6Z=(if sb[28]{e4}else{(if (sf[162]!=0.0){(((p6*(a58-(if (sf[162]!=0.0){(aw*a5X)}else{e4})))-(pe*a5D))/a6p)}else{e4})});let a70=(if sb[28]{e4}else{(if (sf[162]!=0.0){(((p6*(a59-(if (sf[162]!=0.0){(aw*a5Y)}else{e4})))-(pe*a5E))/a6p)}else{e4})});let a71=(if sb[28]{e4}else{(if (sf[162]!=0.0){(((p6*(a5a-(if (sf[162]!=0.0){(aw*a5Z)}else{e4})))-(pe*a5F))/a6p)}else{e4})});let a72=(if sb[28]{e4}else{(if (sf[162]!=0.0){(((p6*(a5b-(if (sf[162]!=0.0){(aw*a60)}else{e4})))-(pe*a5G))/a6p)}else{e4})});let a73=(if sb[28]{e4}else{(if (sf[162]!=0.0){((-(if (sf[162]!=0.0){(aw*a61)}else{e4}))/p6)}else{e4})});let a77=(po*po);let a78=((-(eF*sf[304]))/a77);let a79=(Z/po);let a7a=(f0/po);
        let a7b=(if (sf[166]!=0.0){a78}else{a5K});let a7c=(if (sf[166]!=0.0){e4}else{a5L});let a7d=(if (sf[166]!=0.0){e4}else{a5M});let a7e=(if (sf[166]!=0.0){a79}else{a5N});let a7f=(if (sf[166]!=0.0){a7a}else{e4});let a7g=(if (sf[166]!=0.0){e4}else{a5O});let a7h=(if (sf[166]!=0.0){e4}else{a4q});let a7i=scalar_limexp_derivative(pq);let a7q=(if (sf[166]!=0.0){(a7b*a7i)}else{a5W});let a7r=(if (sf[166]!=0.0){(a7c*a7i)}else{a5X});let a7s=(if (sf[166]!=0.0){(a7d*a7i)}else{a5Y});let a7t=(if (sf[166]!=0.0){(a7e*a7i)}else{a5Z});let a7u=(if (sf[166]!=0.0){(a7f*a7i)}else{e4});let a7v=(if (sf[166]!=0.0){(a7g*a7i)}else{a60});let a7w=(if (sf[166]!=0.0){(a7h*a7i)}else{a61});let a7A=(pt*pt);let a7B=((-(eF*sf[305]))/a7A);let a7C=(Z/pt);let a7D=(f0/pt);let a7E=(if (sf[166]!=0.0){a7B}else{e4});let a7F=(if (sf[166]!=0.0){a7C}else{e4});let a7G=(if (sf[166]!=0.0){a7D}else{e4});let a7H=scalar_limexp_derivative(pv);let a7L=(if (sf[166]!=0.0){(a7E*a7H)}else{e4});let a7M=(if (sf[166]!=0.0){(a7F*a7H)}else{e4});let a7N=(if (sf[166]!=0.0){(a7G*a7H)}else{e4});let a7Q=((B7-(pB*B6))/Ba);let a7R=(f0/e0);let a7S=(Z/e0);let a7T=(if sb[31]{a7Q}else{a4H});let a7U=(if sb[31]{e4}else{a4s});let a7V=(if sb[31]{a7R}else{a4q});let a7W=(if sb[31]{a7S}else{e4});let a7X=scalar_limexp_derivative(pD);let a82=(if sb[31]{(a7T*a7X)}else{a4M});let a83=(if sb[31]{(a7U*a7X)}else{a4N});let a84=(if sb[31]{(a7V*a7X)}else{a4O});let a85=(if sb[31]{(a7W*a7X)}else{e4});let a89=(aJ*a7r);let a8a=(aJ*a7s);let a8d=(aJ*a7v);let a8e=(aJ*a7w);let a8k=(((pG*x6)+(aJ*a7q))+((pI*xo)+(aW*a7L)));let a8l=((aJ*a7t)+(aW*a7M));let a8m=((aJ*a7u)+(aW*a7N));let a8T=((-(eH*sf[304]))/a77);let a8U=(if sb[36]{a8T}else{a7b});let a8V=(if sb[36]{e4}else{a7c});let a8W=(if sb[36]{a79}else{a7d});let a8X=(if sb[36]{e4}else{a7e});let a8Y=(if sb[36]{a7a}else{a7f});let a8Z=(if sb[36]{e4}else{a7g});let a90=(if sb[36]{e4}else{a7h});let a91=scalar_limexp_derivative(pZ);let a99=(if sb[36]{(a8U*a91)}else{a7q});let a9a=(if sb[36]{(a8V*a91)}else{a7r});let a9b=(if sb[36]{(a8W*a91)}else{a7s});let a9c=(if sb[36]{(a8X*a91)}else{a7t});let a9d=(if sb[36]{(a8Y*a91)}else{a7u});let a9e=(if sb[36]{(a8Z*a91)}else{a7v});let a9f=(if sb[36]{(a90*a91)}else{a7w});let a9i=((-(eH*sf[305]))/a7A);let a9j=(if sb[36]{a9i}else{a7E});let a9k=(if sb[36]{a7C}else{e4});let a9l=(if sb[36]{e4}else{a7F});let a9m=(if sb[36]{a7D}else{a7G});let a9n=scalar_limexp_derivative(q3);let a9s=(if sb[36]{(a9j*a9n)}else{a7L});let a9t=(if sb[36]{(a9k*a9n)}else{e4});let a9u=(if sb[36]{(a9l*a9n)}else{a7M});let a9v=(if sb[36]{(a9m*a9n)}else{a7N});let a9y=((B7-(q7*B6))/Ba);let a9z=(if sb[37]{a9y}else{a7T});let a9A=(if sb[37]{e4}else{a7U});let a9B=(if sb[37]{a7R}else{e4});let a9C=(if sb[37]{e4}else{a7V});let a9D=(if sb[37]{a7S}else{a7W});let a9E=scalar_limexp_derivative(q9);let a9K=(if sb[37]{(a9z*a9E)}else{a82});let a9L=(if sb[37]{(a9A*a9E)}else{a83});let a9M=(if sb[37]{(a9B*a9E)}else{e4});let a9N=(if sb[37]{(a9C*a9E)}else{a84});let a9O=(if sb[37]{(a9D*a9E)}else{a85});let a9S=(aJ*a9a);let a9W=(aJ*a9e);let a9X=(aJ*a9f);let aa4=(((qc*x6)+(aJ*a99))+((qe*xo)+(aW*a9s)));let aa5=((aJ*a9b)+(aW*a9t));let aa6=((aJ*a9c)+(aW*a9u));let aa7=((aJ*a9d)+(aW*a9v));let aax=(if sb[40]{a78}else{a8U});let aay=(if sb[40]{e4}else{a8V});let aaz=(if sb[40]{e4}else{a8W});let aaA=(if sb[40]{a79}else{a8X});let aaB=(if sb[40]{a7a}else{a8Y});let aaC=(if sb[40]{e4}else{a8Z});let aaD=(if sb[40]{e4}else{a90});let aaE=scalar_limexp_derivative(qp);let aaM=(if sb[40]{(aax*aaE)}else{a99});let aaN=(if sb[40]{(aay*aaE)}else{a9a});let aaO=(if sb[40]{(aaz*aaE)}else{a9b});let aaP=(if sb[40]{(aaA*aaE)}else{a9c});let aaQ=(if sb[40]{(aaB*aaE)}else{a9d});let aaR=(if sb[40]{(aaC*aaE)}else{a9e});let aaS=(if sb[40]{(aaD*aaE)}else{a9f});let aaT=(if sb[40]{a7B}else{a9j});let aaU=(if sb[40]{e4}else{a9k});let aaV=(if sb[40]{a7C}else{a9l});let aaW=(if sb[40]{a7D}else{a9m});let aaX=scalar_limexp_derivative(qs);let ab2=(if sb[40]{(aaT*aaX)}else{a9s});let ab3=(if sb[40]{(aaU*aaX)}else{a9t});let ab4=(if sb[40]{(aaV*aaX)}else{a9u});let ab5=(if sb[40]{(aaW*aaX)}else{a9v});let ab6=(if sb[41]{a7Q}else{a9z});
        let ab7=(if sb[41]{e4}else{a9A});let ab8=(if sb[41]{e4}else{a9B});let ab9=(if sb[41]{a7R}else{a9C});let aba=(if sb[41]{a7S}else{a9D});let abb=scalar_limexp_derivative(qw);let abh=(if sb[41]{(ab6*abb)}else{a9K});let abi=(if sb[41]{(ab7*abb)}else{a9L});let abj=(if sb[41]{(ab8*abb)}else{a9M});let abk=(if sb[41]{(ab9*abb)}else{a9N});let abl=(if sb[41]{(aba*abb)}else{a9O});let abp=(aJ*aaN);let abB=(((qz*x6)+(aJ*aaM))+((qB*xo)+(aW*ab2)));let abC=((aJ*aaO)+(aW*ab3));let abD=((aJ*aaP)+(aW*ab4));let abE=((aJ*aaQ)+(aW*ab5));let abV=(sf[165]*(aJ*aaR));let abW=(sf[165]*(aJ*aaS));let ac9=(if sb[42]{(sf[165]*abB)}else{(if sb[41]{(sf[165]*(abB-(sf[168]*(abh-Bc))))}else{(if sb[36]{e4}else{(if sb[33]{a8k}else{(if sb[31]{(a8k-(sf[168]*(a82-Bc)))}else{e4})})})})});let aca=(if sb[42]{(sf[165]*abp)}else{(if sb[41]{(sf[165]*(abp-(sf[168]*abi)))}else{(if sb[36]{e4}else{(if sb[33]{a89}else{(if sb[31]{(a89-(sf[168]*a83))}else{e4})})})})});let acb=(if sb[42]{(sf[165]*abC)}else{(if sb[41]{(sf[165]*(abC-(sf[168]*abj)))}else{(if sb[36]{e4}else{(if sb[33]{a8a}else{(if sb[31]{a8a}else{e4})})})})});let acc=(if sb[42]{(sf[165]*abD)}else{(if sb[41]{(sf[165]*(abD-(sf[168]*abk)))}else{(if sb[36]{e4}else{(if sb[33]{a8l}else{(if sb[31]{(a8l-(sf[168]*a84))}else{e4})})})})});let acd=(if sb[42]{(sf[165]*abE)}else{(if sb[41]{(sf[165]*(abE-(sf[168]*abl)))}else{(if sb[36]{e4}else{(if sb[33]{a8m}else{(if sb[31]{(a8m-(sf[168]*a85))}else{e4})})})})});let ace=(if sb[42]{abV}else{(if sb[41]{abV}else{(if sb[36]{e4}else{(if sb[33]{a8d}else{(if sb[31]{a8d}else{e4})})})})});let acf=(if sb[42]{abW}else{(if sb[41]{abW}else{(if sb[36]{e4}else{(if sb[33]{a8e}else{(if sb[31]{a8e}else{e4})})})})});let acn=scalar_limexp_derivative(qM);let acG=scalar_limexp_derivative(qP);let acU=scalar_limexp_derivative(qS);let ad8=(aJ*(if sb[40]{((if sb[40]{e4}else{aay})*acn)}else{aaN}));let adk=(((qW*x6)+(aJ*(if sb[40]{((if sb[40]{a8T}else{aax})*acn)}else{aaM})))+((qY*xo)+(aW*(if sb[40]{((if sb[40]{a9i}else{aaT})*acG)}else{ab2}))));let adl=((aJ*(if sb[40]{((if sb[40]{a79}else{aaz})*acn)}else{aaO}))+(aW*(if sb[40]{((if sb[40]{a7C}else{aaU})*acG)}else{ab3})));let adm=((aJ*(if sb[40]{((if sb[40]{e4}else{aaA})*acn)}else{aaP}))+(aW*(if sb[40]{((if sb[40]{e4}else{aaV})*acG)}else{ab4})));let adn=((aJ*(if sb[40]{((if sb[40]{a7a}else{aaB})*acn)}else{aaQ}))+(aW*(if sb[40]{((if sb[40]{a7D}else{aaW})*acG)}else{ab5})));let adE=(sf[170]*(aJ*(if sb[40]{((if sb[40]{e4}else{aaC})*acn)}else{aaR})));let adF=(sf[170]*(aJ*(if sb[40]{((if sb[40]{e4}else{aaD})*acn)}else{aaS})));let adS=(if sb[42]{(sf[170]*adk)}else{(if sb[41]{(sf[170]*(adk-(sf[168]*((if sb[41]{((if sb[41]{a9y}else{ab6})*acU)}else{abh})-Bc))))}else{(if sb[38]{aa4}else{(if sb[37]{(aa4-(sf[168]*(a9K-Bc)))}else{e4})})})});let adT=(if sb[42]{(sf[170]*ad8)}else{(if sb[41]{(sf[170]*(ad8-(sf[168]*(if sb[41]{((if sb[41]{e4}else{ab7})*acU)}else{abi}))))}else{(if sb[38]{a9S}else{(if sb[37]{(a9S-(sf[168]*a9L))}else{e4})})})});let adU=(if sb[42]{(sf[170]*adl)}else{(if sb[41]{(sf[170]*(adl-(sf[168]*(if sb[41]{((if sb[41]{a7R}else{ab8})*acU)}else{abj}))))}else{(if sb[38]{aa5}else{(if sb[37]{(aa5-(sf[168]*a9M))}else{e4})})})});let adV=(if sb[42]{(sf[170]*adm)}else{(if sb[41]{(sf[170]*(adm-(sf[168]*(if sb[41]{((if sb[41]{e4}else{ab9})*acU)}else{abk}))))}else{(if sb[38]{aa6}else{(if sb[37]{(aa6-(sf[168]*a9N))}else{e4})})})});let adW=(if sb[42]{(sf[170]*adn)}else{(if sb[41]{(sf[170]*(adn-(sf[168]*(if sb[41]{((if sb[41]{a7S}else{aba})*acU)}else{abl}))))}else{(if sb[38]{aa7}else{(if sb[37]{(aa7-(sf[168]*a9O))}else{e4})})})});let adX=(if sb[42]{adE}else{(if sb[41]{adE}else{(if sb[38]{a9W}else{(if sb[37]{a9W}else{e4})})})});let adY=(if sb[42]{adF}else{(if sb[41]{adF}else{(if sb[38]{a9X}else{(if sb[37]{a9X}else{e4})})})});let ae2=(r8*r8);let ae3=((-(eJ*sf[306]))/ae2);let ae4=(f0/r8);let ae5=(Z/r8);let ae6=scalar_limexp_derivative(r9);let ae7=(ae3*ae6);let ae8=(ae4*ae6);let ae9=(ae5*ae6);let aed=(rb*rb);let aee=((-(eJ*sf[307]))/aed);let aef=(f0/rb);let aeg=(Z/rb);let aeh=scalar_limexp_derivative(rc);let aei=(aee*aeh);let aej=(aef*aeh);let aek=(aeg*aeh);
        let aev=(((re*(sf[48]*xB))+(b7*ae7))+((rg*(sf[53]*xP))+(bi*aei)));let aew=((b7*ae8)+(bi*aej));let aex=((b7*ae9)+(bi*aek));let aeG=scalar_limexp_derivative(ro);let aeU=(if (sf[171]!=0.0){((-(eN*sf[307]))/aed)}else{aee});let aeV=(if (sf[171]!=0.0){e4}else{aef});let aeW=(if (sf[171]!=0.0){aeg}else{e4});let aeX=(if (sf[171]!=0.0){e4}else{aeg});let aeY=(if (sf[171]!=0.0){aef}else{e4});let aeZ=scalar_limexp_derivative(rs);let af5=(if (sf[171]!=0.0){(aeU*aeZ)}else{aei});let af6=(if (sf[171]!=0.0){(aeV*aeZ)}else{aej});let af7=(if (sf[171]!=0.0){(aeW*aeZ)}else{e4});let af8=(if (sf[171]!=0.0){(aeX*aeZ)}else{aek});let af9=(if (sf[171]!=0.0){(aeY*aeZ)}else{e4});let afy=(if sb[46]{e4}else{(if (sf[171]!=0.0){(((rv*(sf[58]*xB))+(bk*(if (sf[171]!=0.0){((if (sf[171]!=0.0){((-(eN*sf[306]))/ae2)}else{ae3})*aeG)}else{ae7})))+((rx*(sf[59]*xP))+(bm*af5)))}else{e4})});let afz=(if sb[46]{e4}else{(if (sf[171]!=0.0){((bk*(if (sf[171]!=0.0){((if (sf[171]!=0.0){e4}else{ae4})*aeG)}else{ae8}))+(bm*af6))}else{e4})});let afA=(if sb[46]{e4}else{(if (sf[171]!=0.0){((bk*(if (sf[171]!=0.0){((if (sf[171]!=0.0){ae5}else{e4})*aeG)}else{e4}))+(bm*af7))}else{e4})});let afB=(if sb[46]{e4}else{(if (sf[171]!=0.0){((bk*(if (sf[171]!=0.0){((if (sf[171]!=0.0){e4}else{ae5})*aeG)}else{ae9}))+(bm*af8))}else{e4})});let afC=(if sb[46]{e4}else{(if (sf[171]!=0.0){((bk*(if (sf[171]!=0.0){((if (sf[171]!=0.0){ae4}else{e4})*aeG)}else{e4}))+(bm*af9))}else{e4})});let afD=(rG*zQ);let afG=(-rG);let afI=(c6*rK);let afS=(if (sf[173]!=0.0){(c9*(zQ+((afD+afD)/afI)))}else{ZH});let afT=(if (sf[173]!=0.0){(c9*(Z+((rG+rG)/afI)))}else{ZI});let afU=(if (sf[173]!=0.0){e4}else{ZJ});let afV=(if (sf[173]!=0.0){(c9*(f0+((afG+afG)/afI)))}else{ZK});let afW=(if (sf[173]!=0.0){e4}else{ZL});let afX=(if (sf[173]!=0.0){e4}else{ZM});let afY=(if (sf[173]!=0.0){e4}else{ZN});let ag9=(sf[174]*f64::powf(rN,sf[309]));let agq=scalar_limexp_derivative(rS);let ah0=(a42-a3K);let ah1=(a45-a3O);let ah2=(a48-a3R);let ah3=(a4c-a3V);let ah4=(a4g-a3Y);let ahD=(aev-(if sb[48]{e4}else{(if (sf[173]!=0.0){((rX*(if (sf[173]!=0.0){((rT*(sf[172]*afS))+(rO*(((rR*sf[308])+(rP*(afS*ag9)))*agq)))}else{e4}))+(rV*(ah0-aev)))}else{e4})}));let ahE=(aew-(if sb[48]{e4}else{(if (sf[173]!=0.0){((rX*(if (sf[173]!=0.0){((rT*(sf[172]*afT))+(rO*((rP*(afT*ag9))*agq)))}else{e4}))+(rV*(ah1-aew)))}else{e4})}));let ahF=(-(if sb[48]{e4}else{(if (sf[173]!=0.0){((rX*(if (sf[173]!=0.0){((rT*(sf[172]*afU))+(rO*((rP*(afU*ag9))*agq)))}else{e4}))+(rV*ah2))}else{e4})}));let ahG=(aex-(if sb[48]{e4}else{(if (sf[173]!=0.0){((rX*(if (sf[173]!=0.0){((rT*(sf[172]*afV))+(rO*((rP*(afV*ag9))*agq)))}else{e4}))+(rV*(ah3-aex)))}else{e4})}));let ahH=(-(if sb[48]{e4}else{(if (sf[173]!=0.0){((rX*(if (sf[173]!=0.0){((rT*(sf[172]*afW))+(rO*((rP*(afW*ag9))*agq)))}else{e4}))+(rV*ah4))}else{e4})}));let ahI=(-(if sb[48]{e4}else{(if (sf[173]!=0.0){(rX*(if (sf[173]!=0.0){((rT*(sf[172]*afX))+(rO*((rP*(afX*ag9))*agq)))}else{e4}))}else{e4})}));let ahJ=(-(if sb[48]{e4}else{(if (sf[173]!=0.0){(rX*(if (sf[173]!=0.0){((rT*(sf[172]*afY))+(rO*((rP*(afY*ag9))*agq)))}else{e4}))}else{e4})}));let ahT=(if sb[50]{e4}else{(if (sf[175]!=0.0){(Z/v)}else{e4})});let ahU=(if sb[50]{e4}else{(if (sf[175]!=0.0){((-(s6*(sf[5]*(sf[194]*(sf[6]*f64::powf(m_,sf[196]))))))/(v*v))}else{e4})});let ahV=(if sb[50]{e4}else{(if (sf[175]!=0.0){(f0/v)}else{e4})});let aix=(so*so);let ajc=(if (sf[176]!=0.0){(((z*((su*vj)+(l*((aii-air)-((if (sf[176]!=0.0){(((so*aii)-(sn*air))/aix)}else{e4})/sq)))))-(sw*vz))/(z*z))}else{e4});let ajd=(if (sf[176]!=0.0){((Z+(l*((-ais)-((if (sf[176]!=0.0){((-(sn*ais))/aix)}else{e4})/sq))))/z)}else{e4});let aje=(if (sf[176]!=0.0){((f0+(l*(aij-((if (sf[176]!=0.0){(aij/so)}else{e4})/sq))))/z)}else{e4});let ajf=(if (sf[176]!=0.0){((l*((aik-ait)-((if (sf[176]!=0.0){(((so*aik)-(sn*ait))/aix)}else{e4})/sq)))/z)}else{e4});let ajs=(-sr);let aju=(c6*sF);let ajD=(sH*sH);let ajS=(sJ*(if (sf[176]!=0.0){(((sH*((sz*ajc)+(sy*((ep*vz)+(z*Bk)))))-(sA*(sF*(sf[111]*(c9*Bk)))))/ajD)}else{e4}));let ajU=(sJ*(if (sf[176]!=0.0){(((sH*(sz*ajd))-(sA*(sC*((sr+sr)/aju))))/ajD)}else{e4}));
        let ajW=(sJ*(if (sf[176]!=0.0){(((sH*(sz*aje))-(sA*(sC*((ajs+ajs)/aju))))/ajD)}else{e4}));let ajY=(sJ*(if (sf[176]!=0.0){((sz*ajf)/sH)}else{e4}));let ak0=(c6*sM);let ak8=(sM*sM);let akq=(if sb[52]{e4}else{(if (sf[176]!=0.0){(((sM*ajc)-(sy*((ajS+ajS)/ak0)))/ak8)}else{e4})});let akr=(if sb[52]{e4}else{(if (sf[176]!=0.0){(((sM*ajd)-(sy*((ajU+ajU)/ak0)))/ak8)}else{e4})});let aks=(if sb[52]{e4}else{(if (sf[176]!=0.0){(((sM*aje)-(sy*((ajW+ajW)/ak0)))/ak8)}else{e4})});let akt=(if sb[52]{e4}else{(if (sf[176]!=0.0){(((sM*ajf)-(sy*((ajY+ajY)/ak0)))/ak8)}else{e4})});let akD=(if sb[54]{e4}else{(if (sf[177]!=0.0){(Z/D)}else{e4})});let akE=(if sb[54]{e4}else{(if (sf[177]!=0.0){((-(sU*(sf[9]*(sf[194]*(sf[10]*f64::powf(m_,sf[198]))))))/(D*D))}else{e4})});let akF=(if sb[54]{e4}else{(if (sf[177]!=0.0){(f0/D)}else{e4})});let al2=(if sb[56]{e4}else{(if (sf[178]!=0.0){(((H*(t1*a3B))-(t2*(sf[11]*(sf[194]*(sf[12]*f64::powf(m_,sf[199]))))))/(H*H))}else{e4})});let al3=(if sb[56]{e4}else{(if (sf[178]!=0.0){((t1*a3C)/H)}else{e4})});let al4=(if sb[56]{e4}else{(if (sf[178]!=0.0){((oD+(t1*a3D))/H)}else{e4})});let al5=(if sb[56]{e4}else{(if (sf[178]!=0.0){(((t1*a3E)+(-oD))/H)}else{e4})});let al6=(if sb[56]{e4}else{(if (sf[178]!=0.0){((t1*a3F)/H)}else{e4})});let alg=(if sb[58]{e4}else{(if (sf[179]!=0.0){(Z/L)}else{e4})});let alh=(if sb[58]{e4}else{(if (sf[179]!=0.0){((-(ta*(sf[13]*(sf[194]*(sf[14]*f64::powf(m_,sf[200]))))))/(L*L))}else{e4})});let ali=(if sb[58]{e4}else{(if (sf[179]!=0.0){(f0/L)}else{e4})});let alG=(if sb[60]{e4}else{(if (sf[180]!=0.0){(((T*(th*(if sb[28]{e4}else{a5C})))-(ti*(sf[17]*(sf[194]*(sf[18]*f64::powf(m_,sf[202]))))))/(T*T))}else{e4})});let alH=(if sb[60]{e4}else{(if (sf[180]!=0.0){((-pj)/T)}else{e4})});let alI=(if sb[60]{e4}else{(if (sf[180]!=0.0){((th*(if sb[28]{e4}else{a5D}))/T)}else{e4})});let alJ=(if sb[60]{e4}else{(if (sf[180]!=0.0){((th*(if sb[28]{e4}else{a5E}))/T)}else{e4})});let alK=(if sb[60]{e4}else{(if (sf[180]!=0.0){((th*(if sb[28]{e4}else{a5F}))/T)}else{e4})});let alL=(if sb[60]{e4}else{(if (sf[180]!=0.0){((pj+(th*(if sb[28]{e4}else{a5G})))/T)}else{e4})});let alY=scalar_limexp_derivative(tt);let amm=scalar_limexp_derivative(ty);let amZ=(if sb[64]{e4}else{(if (sf[181]!=0.0){(((tB*(sf[60]*(((bs*wS)+(az*(bs*(((l*sf[222])-(bq*vj))/w8))))*(sf[64]*f64::powf(bt,sf[223])))))+(bx*(if (sf[181]!=0.0){((if (sf[181]!=0.0){((-(mE*sf[310]))/(tr*tr))}else{ahY})*alY)}else{ai2})))+((tD*(sf[65]*(((bD*xa)+(aM*(bD*(((l*sf[224])-(bB*vj))/w8))))*(sf[69]*f64::powf(bE,sf[225])))))+(bI*(if (sf[181]!=0.0){((if (sf[181]!=0.0){((-(mE*sf[311]))/(tw*tw))}else{aeU})*amm)}else{af5}))))}else{e4})});let an0=(if sb[64]{e4}else{(if (sf[181]!=0.0){((bx*(if (sf[181]!=0.0){((if (sf[181]!=0.0){e4}else{ahZ})*alY)}else{ai3}))+(bI*(if (sf[181]!=0.0){((if (sf[181]!=0.0){e4}else{aeV})*amm)}else{af6})))}else{e4})});let an1=(if sb[64]{e4}else{(if (sf[181]!=0.0){(bI*(if (sf[181]!=0.0){((if (sf[181]!=0.0){e4}else{aeW})*amm)}else{af7}))}else{e4})});let an2=(if sb[64]{e4}else{(if (sf[181]!=0.0){((bx*(if (sf[181]!=0.0){((if (sf[181]!=0.0){e4}else{ai0})*alY)}else{ai4}))+(bI*(if (sf[181]!=0.0){((if (sf[181]!=0.0){e4}else{aeX})*amm)}else{af8})))}else{e4})});let an3=(if sb[64]{e4}else{(if (sf[181]!=0.0){((bx*(if (sf[181]!=0.0){((if (sf[181]!=0.0){(f0/tr)}else{e4})*alY)}else{e4}))+(bI*(if (sf[181]!=0.0){((if (sf[181]!=0.0){(f0/tw)}else{aeY})*amm)}else{af9})))}else{e4})});let an4=(if sb[64]{e4}else{(if (sf[181]!=0.0){((bx*(if (sf[181]!=0.0){((if (sf[181]!=0.0){(Z/tr)}else{e4})*alY)}else{e4}))+(bI*(if (sf[181]!=0.0){((if (sf[181]!=0.0){(Z/tw)}else{e4})*amm)}else{e4})))}else{e4})});let ane=(if sb[66]{e4}else{(if (sf[182]!=0.0){(Z/P)}else{e4})});let anf=(if sb[66]{e4}else{(if (sf[182]!=0.0){((-(tM*(sf[15]*(sf[194]*(sf[16]*f64::powf(m_,sf[201]))))))/(P*P))}else{e4})});let ang=(if sb[66]{e4}else{(if (sf[182]!=0.0){(f0/P)}else{e4})});

        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(9),
            multiplicity * (qL),
            [4, 6, 7, 8, 9, 10, 11],
            [ac9, aca, acb, acc, acd, ace, acf],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (r7),
            [4, 6, 7, 8, 9, 10, 11],
            [adS, adT, adU, adV, adW, adX, adY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (oF),
            [4, 6, 7, 8, 9],
            [a42, a45, a48, a4c, a4g],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(6),
            multiplicity * (oE),
            [4, 6, 7, 8, 9],
            [a3K, a3O, a3R, a3V, a3Y],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(6),
            multiplicity * (s2),
            [4, 6, 7, 8, 9, 10, 11],
            [ahD, ahE, ahF, ahG, ahH, ahI, ahJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * (rC),
            [4, 6, 7, 8, 10],
            [afy, afz, afA, afB, afC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(5),
            multiplicity * (sa),
            0,
            multiplicity * (ahT),
            4,
            multiplicity * (ahU),
            5,
            multiplicity * (ahV),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (sQ),
            [4, 5, 6, 8],
            [akq, akr, aks, akt],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(7),
            multiplicity * (sY),
            1,
            multiplicity * (akD),
            4,
            multiplicity * (akE),
            7,
            multiplicity * (akF),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (t6),
            [4, 6, 7, 8, 9],
            [al2, al3, al4, al5, al6],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(9),
            multiplicity * (te),
            2,
            multiplicity * (alg),
            4,
            multiplicity * (alh),
            9,
            multiplicity * (ali),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(5),
            multiplicity * (tm),
            [4, 5, 6, 7, 8, 10],
            [alG, alH, alI, alJ, alK, alL],
            [],
            [],
            multiplicity,
        );
        let ui_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, ui);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(9),
            multiplicity * (ui_ddt),
            [4, 6, 7, 8, 9],
            [((aoY) * ddt_scale), ((aoL) * ddt_scale), ((aoP) * ddt_scale), ((aoZ) * ddt_scale), ((ap0) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let uk_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, uk);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(9),
            multiplicity * (uk_ddt),
            [4, 7, 8, 9],
            [((ap7) * ddt_scale), ((ap8) * ddt_scale), ((ap9) * ddt_scale), ((apa) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let ur_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, ur);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (ur_ddt),
            [4, 6, 7, 8, 9],
            [((apr) * ddt_scale), ((aps) * ddt_scale), ((apf) * ddt_scale), ((apt) * ddt_scale), ((aph) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let us_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, us);
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * (us_ddt),
            4,
            multiplicity * (((apu) * ddt_scale)),
            5,
            multiplicity * (((apv) * ddt_scale)),
            8,
            multiplicity * (((apw) * ddt_scale)),
        );
        let uv_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, uv);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(10),
            multiplicity * (uv_ddt),
            [4, 6, 7, 8, 9, 10],
            [((apK) * ddt_scale), ((apL) * ddt_scale), ((apM) * ddt_scale), ((apN) * ddt_scale), ((apD) * ddt_scale), ((apO) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let uC_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, uC);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (uC_ddt),
            1,
            multiplicity * (((sf[189]) * ddt_scale)),
            2,
            multiplicity * (((sf[316]) * ddt_scale)),
        );
        let uF_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, uF);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (uF_ddt),
            0,
            multiplicity * (((sf[317]) * ddt_scale)),
            1,
            multiplicity * (((sf[190]) * ddt_scale)),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(10),
            multiplicity * (tI),
            [4, 6, 7, 8, 10, 11],
            [amZ, an0, an1, an2, an3, an4],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(11),
            multiplicity * (pk),
            [4, 6, 7, 8, 10, 11],
            [a6Y, a6Z, a70, a71, a72, a73],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(3),
            Some(11),
            multiplicity * (tQ),
            3,
            multiplicity * (ane),
            4,
            multiplicity * (anf),
            11,
            multiplicity * (ang),
        );
        let uz_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, uz);
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(10),
            multiplicity * (uz_ddt),
            [4, 6, 7, 8, 9, 10, 11],
            [((apR) * ddt_scale), ((apS) * ddt_scale), ((apT) * ddt_scale), ((apU) * ddt_scale), ((apV) * ddt_scale), ((apZ) * ddt_scale), ((aq0) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((if sb[68]{e4}else{(if (sf[192]!=0.0){(g/sf[191])}else{e4})})),
            4,
            multiplicity * (sf[320]),
        );
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * ((-((((((((((((((eF*qL)+(eJ*s2))+(rW*uJ))+(eH*r7))+(eN*rC))+(tM*tQ))+(mE*tI))+(pk*uU))+(s6*sa))+(sr*sQ))+(sU*sY))+(t1*t6))+(ta*te))+(th*tm)))),
            &[(-(sa+(s6*ahT))),(-(sY+(sU*akD))),(-(te+(ta*alg))),(-(tQ+(tM*ane))),(-((((((((((((((eF*ac9)+(eJ*ahD))+(uJ*ah0))+(eH*adS))+(eN*afy))+(tM*anf))+(mE*amZ))+(uU*a6Y))+(s6*ahU))+(sr*akq))+(sU*akE))+(t1*al2))+(ta*alh))+(th*alG))),(-((((-sa)+(s6*ahV))+(sQ+(sr*akr)))+((-tm)+(th*alH)))),(-((((((((((eF*aca)+((-s2)+(eJ*ahE)))+(rW+(uJ*ah1)))+(eH*adT))+(eN*afz))+(mE*an0))+(uU*a6Z))+((-sQ)+(sr*aks)))+(t1*al3))+(th*alI))),(-((((((((((eF*acb)+(eJ*ahF))+(uJ*ah2))+(r7+(eH*adU)))+(rC+(eN*afA)))+(mE*an1))+(pk+(uU*a70)))+((-sY)+(sU*akF)))+(t6+(t1*al4)))+(th*alJ))),(-((((((((((qL+(eF*acc))+(s2+(eJ*ahG)))+(uJ*ah3))+(eH*adV))+(eN*afB))+(mE*an2))+(uU*a71))+(sr*akt))+((-t6)+(t1*al5)))+(th*alK))),(-(((((((-qL)+(eF*acd))+(eJ*ahH))+((uJ*ah4)+(-rW)))+((-r7)+(eH*adW)))+(t1*al6))+((-te)+(ta*ali)))),(-(((((((eF*ace)+(eJ*ahI))+(eH*adX))+((-rC)+(eN*afC)))+((-tI)+(mE*an3)))+(uU*a72))+(tm+(th*alL)))),(-((((((eF*acf)+(eJ*ahJ))+(eH*adY))+((-tQ)+(tM*ang)))+(tI+(mE*an4)))+((uU*a73)+(-pk))))],
            &[],
            multiplicity,
        );
        let vi_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, vi);
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (vi_ddt),
            4,
            multiplicity * (((sf[193]) * ddt_scale)),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        let br=self.branches;
        let branches=br;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let CommonStampValues {
            g, l, m_, n_, Z, a0, aw, c6,
            c9, cx, d9, e4, eD, eE, eF, eG,
            eH, eI, eJ, eK, eM, eN, f0, mD,
            mE, nC, nX, o3, oD, oI, oK, oM,
            oO, oQ, oY, s5, sb_, sc, sh, sk,
            sT, t9, ui, uk, ur, us, uv, uz,
            uC, uF, vi, vj, w8, wO, zQ, ZH,
            ZI, ZJ, ZK, ZL, ZM, ZN, a1r, a1s,
            a1t, a1M, a1N, a1O, a3B, a3C, a3D, a3E,
            a3F, a3J, a4k, a4n, a4o, a4p, a4q, a4r,
            a4s, a4z, a4A, a4B, a4C, a4D, a4H, a4M,
            a4N, a4O, a57, a58, a59, a5a, a5b, ahY,
            ahZ, ai0, ai2, ai3, ai4, aii, aij, aik,
            air, ais, ait, aoL, aoP, aoY, aoZ, ap0,
            ap7, ap8, ap9, apa, apf, aph, apr, aps,
            apt, apu, apv, apw, apD, apK, apL, apM,
            apN, apO, apR, apS, apT, apU, apV, apZ,
            aq0,
        }=self.eval_common_stamp_values(ctx);
        let p=&(*self.params);
        let m=self.multiplicity;
        let multiplicity=m;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(9),
            &[4, 6, 7, 8, 9],
            &[aoY, aoL, aoP, aoZ, ap0],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(9),
            &[4, 7, 8, 9],
            &[ap7, ap8, ap9, apa],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(6),
            &[4, 6, 7, 8, 9],
            &[apr, aps, apf, apt, aph],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3_local(
            Some(8),
            Some(5),
            4,
            multiplicity * (apu),
            5,
            multiplicity * (apv),
            8,
            multiplicity * (apw),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(10),
            &[4, 6, 7, 8, 9, 10],
            &[apK, apL, apM, apN, apD, apO],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(2),
            1,
            multiplicity * (sf[189]),
            2,
            multiplicity * (sf[316]),
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(0),
            0,
            multiplicity * (sf[317]),
            1,
            multiplicity * (sf[190]),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(10),
            &[4, 6, 7, 8, 9, 10, 11],
            &[apR, apS, apT, apU, apV, apZ, aq0],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node1_local(
            Some(4),
            None,
            4,
            multiplicity * (sf[193]),
        );
    }
}
