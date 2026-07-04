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
    a: f64, b: f64, d: f64, f: f64, g: f64, h: f64,
    i: f64, j: f64, l: f64, n_: f64, G: f64, J: f64,
    N: f64, S: f64, T: f64, V: f64, aw: f64, b6: bool,
    ba: f64, bh: bool, bm: bool, bs: f64, bu: f64, bz: f64,
    bF: f64, bU: f64, c3: f64, c8: f64, ci: f64, cq: f64,
    cz: f64, d8: f64, d9: f64, db: f64, eg: f64, eD: f64,
    f4: f64, gt: f64, gH: f64, it: f64, iz: f64, k4: f64,
    k9: f64, kL: f64, l1: f64, lQ: f64, lW: f64, m6: f64,
    m9: f64, mo: f64, mq: f64, ms: f64, mt: f64, mw: f64,
    mx: f64, mE: f64, mI: f64, mK: f64, n0: f64, n6: f64,
    nc: f64, nh: f64, np: f64, nq: f64, nD: f64, o1: f64,
    o2: f64, om: f64, or: f64, ot: f64, ov: f64, ox: f64,
    oy: f64, ph: f64, pi: f64, pj: f64, pk: f64, pl: f64,
    qU: f64, qV: f64, qW: f64, qX: f64, qY: f64, qZ: f64,
    r0: f64, r9: f64, ra: f64, rb: f64, rc: f64, wC: f64,
    wD: f64, wE: f64, wF: f64, wG: f64, ye: f64, yf: f64,
    yg: f64, yh: f64, B0: f64, B1: f64, B2: f64, B3: f64,
    B4: f64, KC: f64, Ly: f64, NY: f64, NZ: f64, O0: f64,
    O1: f64, ON: f64, OO: f64, OP: f64, OQ: f64, Pk: f64,
    Pm: f64, Pn: f64, Pv: f64, Px: f64, Pz: f64, PJ: f64,
    PS: f64, PT: f64, PU: f64, PV: f64, PW: f64, Q2: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let n=self.nodes;
        let nodes=n;
        let br=self.branches;
        let branches=br;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let a=ctx.node_voltage(n[12]);let b=ctx.node_voltage(n[8]);let c=(a-b);let d=ctx.node_voltage(n[10]);let e=ctx.node_voltage(n[5]);let f=(d-e);let g=(-f);let h=(e-b);let i=ctx.node_voltage(n[11]);let j=(i-b);let k=ctx.node_voltage(n[4]);let l=(k-b);let n_=0.0;let G=ctx.node_voltage(n[3]);let J=(if (sf[10]!=0.0){(sf[218]+(G).abs())}else{sf[218]});let N=((J-sf[9])).abs();let S=1.0;let T=(if ((N>n_)||sb[2]){S}else{n_});let V=(N).abs();let aw=(S+(V*sf[24]));let aH=(S+(N*sf[28]));let b6=((T!=0.0)&&(sf[36]!=0.0));let ba=(S+(sf[24]*(N*N)));let bh=((T!=0.0)&&sb[8]);let bm=(!(T!=0.0));let bp=(if bm{sf[17]}else{(if (T!=0.0){(sf[17]*(S+(V*sf[18])))}else{n_})});let bq=(if bm{sf[19]}else{(if (T!=0.0){(sf[19]*(S+(V*sf[20])))}else{n_})});let br_=(if bm{sf[21]}else{(if (T!=0.0){(sf[21]*(S+(V*sf[22])))}else{n_})});let bs=(if bm{sf[23]}else{(if (T!=0.0){(sf[23]*aw)}else{n_})});let bu=(if bm{sf[38]}else{(if bh{(aw*sf[38])}else{(if b6{(ba*sf[38])}else{n_})})});let bw=(if bm{sf[27]}else{(if (T!=0.0){(sf[27]*aH)}else{n_})});let bx=(if bm{sf[29]}else{(if (T!=0.0){(aH*sf[29])}else{n_})});let bz=(if bm{sf[32]}else{(if (T!=0.0){(sf[32]+(N*sf[33]))}else{n_})});let bF=0.5;let bO=(h*sf[45]);let bP=(bO).cosh();let bU=1e-12;let bW=(bU+(bP*bP));let c2=(S+(V*sf[49]));let c3=((sf[47]*(S+(sf[48]/bW)))*c2);let c8=(sf[50]*(S+(V*sf[51])));let cd=((h*sf[53])).tanh();let ci=(g-bz);let cj=(sf[54]*ci);let cn=(S+(V*sf[26]));let co=((((((if bm{sf[25]}else{(if (T!=0.0){(sf[25]+(N*sf[26]))}else{n_})})-sf[52])+(sf[52]*cd))-(l*sf[46]))-(ci*cj))*cn);let cp=(c-co);let cq=(cp*cp);let cv=(c8*cp);let cx=(((c3*cp)+(cq*sf[55]))+(cq*cv));let cy=(cx).tanh();let cz=(S+cy);let cB=(-cx);let cF=((bF*(scalar_limexp(cx)-scalar_limexp(cB)))).tanh();let cP=2.0;let d8=(f-co);let d9=(if sb[16]{d8}else{bP});let db=(if sb[16]{(d9*d9)}else{cp});let dZ=(if sb[19]{cp}else{d9});let e1=(if sb[19]{(dZ*dZ)}else{db});let e4=(c8*e1);let e6=((dZ+(sf[55]*e1))+(dZ*e4));let e8=(if sb[19]{(c3*e6)}else{cx});let ea=(-e8);let ee=((bF*(scalar_limexp(e8)-scalar_limexp(ea)))).tanh();let eg=(if sb[19]{(S+ee)}else{(S+cF)});let eD=(if sb[22]{cp}else{dZ});let eF=(if sb[22]{(eD*eD)}else{e1});let eI=(c8*eF);let eK=((eD+(sf[55]*eF))+(eD*eI));let eM=(if sb[22]{(c3*eK)}else{e8});let eY=(-eM);let f2=((bF*(scalar_limexp(eM)-scalar_limexp(eY)))).tanh();let f4=(if sb[22]{(S+f2)}else{eg});let gd=(S+cz);let gq=(S+f4);let gt=(if sb[28]{(sf[67]+(br_/gq))}else{(if (sf[66]!=0.0){(sf[67]+(br_/gd))}else{n_})});let gH=-1.0;let hO=(h*sf[89]);let hP=((bw+(j*sf[88]))+hO);let hQ=(hP).tanh();let hR=(S+hQ);let hW=((sf[90]+(h*sf[91]))).tanh();let hX=(S+hW);let i2=((sf[92]-(h*sf[93]))).tanh();let i4=((S+i2)-sf[89]);let i8_=((bx+(f*sf[94]))-hO);let i9=(i8_).tanh();let ia=(S+i9);let iq=(bp*hR);let it=(if sb[40]{(sf[100]+(hX*iq))}else{sf[101]});let iz=(if sb[40]{(sf[102]+(bq*((i4*ia)+sf[104])))}else{sf[103]});let iE=(if sb[43]{(hX-sf[89])}else{hX});let iF=(bw+hO);let iG=(iF).cosh();let iH=(if sb[43]{iG}else{n_});let iJ=(if sb[43]{(iH).ln()}else{n_});let iK=(hP).cosh();let iL=(if sb[43]{iK}else{n_});let iN=(if sb[43]{(iL).ln()}else{n_});let iP=(if sb[43]{(iF+iJ)}else{n_});let iR=((hP+iN)-iP);let iU=(j*sf[104]);let iX=(j*sf[100]);let j0=(bx-hO);let j1=(j0).cosh();let j2=(if sb[43]{j1}else{iH});let j4=(if sb[43]{(j2).ln()}else{n_});let j5=(i8_).cosh();let j6=(if sb[43]{j5}else{iL});let j8=(if sb[43]{(j6).ln()}else{n_});let ja=(if sb[43]{(j0+j4)}else{n_});let jc=((i8_+j8)-ja);let jf=(f*sf[104]);let ji=(f*sf[102]);let jt=(j/sf[105]);let jv=(if sb[46]{(jt-S)}else{n_});let jy=(jv*jv);let jz=(sf[107]+jy);let jB=f64::powf(jz,sf[108]);let jF=(sf[107]+(jy*sf[110]));let jL=((bw+(sf[88]*(j+hO)))).tanh();let jO=(if sb[46]{hX}else{iE});let jQ=(i2+sf[111]);let jR=(if sb[46]{jQ}else{i4});let jW=((bx+(sf[94]*(f+(h*sf[111]))))).tanh();let jY=(if sb[46]{(S+jW)}else{ia});let k2=(bp*((if sb[46]{(S+jL)}else{hR})+((if sb[46]{(jB*jF)}else{n_})*sf[112])));let k4=(sf[100]+(jO*k2));let k9=(sf[102]+(bq*(sf[104]+(jR*jY))));let ke=(if sb[49]{iG}else{j2});let kh=(if sb[49]{iK}else{j6});let km=(sf[112]*(j+sf[105]));let kn=(gH+jt);let kp=(sf[107]+{let pb=kn;pb*pb});
        let kr=f64::powf(kp,sf[114]);let kE=(((if sb[49]{(km*kr)}else{n_})+((hP+(if sb[49]{(kh).ln()}else{iN}))-(if sb[49]{(iF+(if sb[49]{(ke).ln()}else{iJ}))}else{iP})))-sf[119]);let kF=(hW+sf[111]);let kL=(if sb[49]{(iX+(bp*(iU+((kE*kF)/sf[88]))))}else{(if sb[43]{((bp*(((iE*iR)/sf[88])+iU))+iX)}else{n_})});let kM=(if sb[49]{j1}else{ke});let kP=(if sb[49]{j5}else{kh});let kV=((i8_+(if sb[49]{(kP).ln()}else{j8}))-(if sb[49]{(j0+(if sb[49]{(kM).ln()}else{j4}))}else{ja}));let l1=(if sb[49]{(ji+(bq*(jf+((jQ*kV)/sf[94]))))}else{(if sb[43]{((bq*(((i4*jc)/sf[94])+jf))+ji)}else{n_})});let lQ=(if sb[67]{((bp*((J*5.5226012e-23)*sf[140]))*sf[142])}else{n_});let lW=3.141592653589793;let m6=(sf[144]*ctx.node_voltage(n[15]));let m9=(sf[145]*ctx.branch_current(br[0]));let mo=(sf[146]*(ctx.node_voltage(n[7])-e));let mq=(h*sf[147]);let ms=(ctx.node_voltage(n[6])-k);let mt=(bs*ms);let mv=ctx.branch_current(br[1]);let mw=(gt*mv);let mx=(sf[121]*mv);let mE=(c*bu);let mI=ctx.node_voltage(n[14]);let mK=(sf[148]*(i-mI));let n0=(sf[149]*ctx.branch_current(br[10]));let n6=(sf[150]*ctx.branch_current(br[14]));let nc=(sf[151]*ctx.branch_current(br[18]));let nh=ctx.node_voltage(n[17]);let np=(-(if sb[67]{(lQ*lW)}else{n_}));let nq=(nh*np);let nD=(G*sf[152]);let nK=(bO).sinh();let nL=(sf[45]*nK);let nM=(sf[153]*nK);let nO=(bP*nL);let nQ=(bP*nM);let nU=(bW*bW);let o1=(c2*(sf[47]*((-(sf[48]*(nO+nO)))/nU)));let o2=(c2*(sf[47]*((-(sf[48]*(nQ+nQ)))/nU)));let o5=(S-(cd*cd));let oj=(cn*((sf[52]*(sf[53]*o5))-(cj+cj)));let ok=(cn*((sf[52]*(sf[155]*o5))-sf[154]));let ol=(cn*(-((-cj)+(ci*sf[156]))));let om=(-(cn*sf[154]));let on=(-oj);let oo=(gH-ok);let op=(-ol);let oq=(cp*om);let or=(oq+oq);let os=(cp*on);let ot=(os+os);let ou=(cp*oo);let ov=(ou+ou);let ow=(cp*op);let ox=(ow+ow);let oy=(cp+cp);let pa=(((c3*om)+(sf[55]*or))+((cv*or)+(cq*(c8*om))));let pb=((((cp*o1)+(c3*on))+(sf[55]*ot))+((cv*ot)+(cq*(c8*on))));let pc=((((cp*o2)+(c3*oo))+(sf[55]*ov))+((cv*ov)+(cq*(c8*oo))));let pd=(((c3*op)+(sf[55]*ox))+((cv*ox)+(cq*(c8*op))));let pe=((c3+(sf[55]*oy))+((cv*oy)+(c8*cq)));let pg_=(S-(cy*cy));let ph=(pa*pg_);let pi=(pb*pg_);let pj=(pc*pg_);let pk=(pd*pg_);let pl=(pe*pg_);let pm=scalar_limexp_derivative(cx);let px=scalar_limexp_derivative(cB);let pO=(S-(cF*cF));let qU=(gH-oj);let qV=(-ok);let qW=(S-ol);let qX=(if sb[16]{om}else{n_});let qY=(if sb[16]{qU}else{nL});let qZ=(if sb[16]{qV}else{nM});let r0=(if sb[16]{qW}else{n_});let r1=(d9*qX);let r3=(d9*qY);let r5=(d9*qZ);let r7=(d9*r0);let r9=(if sb[16]{(r1+r1)}else{om});let ra=(if sb[16]{(r3+r3)}else{on});let rb=(if sb[16]{(r5+r5)}else{oo});let rc=(if sb[16]{(r7+r7)}else{op});let uX=(if sb[19]{om}else{qX});let uY=(if sb[19]{on}else{qY});let uZ=(if sb[19]{oo}else{qZ});let v0=(if sb[19]{op}else{r0});let v2=(dZ*uX);let v4=(dZ*uY);let v6=(dZ*uZ);let v8=(dZ*v0);let va=(dZ*sf[161]);let vc=(if sb[19]{(v2+v2)}else{r9});let vd=(if sb[19]{(v4+v4)}else{ra});let ve=(if sb[19]{(v6+v6)}else{rb});let vf=(if sb[19]{(v8+v8)}else{rc});let vg=(if sb[19]{(va+va)}else{sf[158]});let vZ=(if sb[19]{(c3*((uX+(sf[55]*vc))+((e4*uX)+(dZ*(c8*vc)))))}else{pa});let w0=(if sb[19]{((e6*o1)+(c3*((uY+(sf[55]*vd))+((e4*uY)+(dZ*(c8*vd))))))}else{pb});let w1=(if sb[19]{((e6*o2)+(c3*((uZ+(sf[55]*ve))+((e4*uZ)+(dZ*(c8*ve))))))}else{pc});let w2=(if sb[19]{(c3*((v0+(sf[55]*vf))+((e4*v0)+(dZ*(c8*vf)))))}else{pd});let w3=(if sb[19]{(c3*((sf[161]+(sf[55]*vg))+((e4*sf[161])+(dZ*(c8*vg)))))}else{pe});let w4=scalar_limexp_derivative(e8);let wf=scalar_limexp_derivative(ea);let ww=(S-(ee*ee));let wC=(if sb[19]{((bF*((vZ*w4)-((-vZ)*wf)))*ww)}else{((bF*((pa*pm)-((-pa)*px)))*pO)});let wD=(if sb[19]{((bF*((w0*w4)-((-w0)*wf)))*ww)}else{((bF*((pb*pm)-((-pb)*px)))*pO)});let wE=(if sb[19]{((bF*((w1*w4)-((-w1)*wf)))*ww)}else{((bF*((pc*pm)-((-pc)*px)))*pO)});let wF=(if sb[19]{((bF*((w2*w4)-((-w2)*wf)))*ww)}else{((bF*((pd*pm)-((-pd)*px)))*pO)});let wG=(if sb[19]{((bF*((w3*w4)-((-w3)*wf)))*ww)}else{((bF*((pe*pm)-((-pe)*px)))*pO)});let ye=(if sb[22]{om}else{uX});let yf=(if sb[22]{on}else{uY});let yg=(if sb[22]{oo}else{uZ});let yh=(if sb[22]{op}else{v0});let yj=(eD*ye);
        let yl=(eD*yf);let yn=(eD*yg);let yp=(eD*yh);let yr=(eD*sf[162]);let yt=(if sb[22]{(yj+yj)}else{vc});let yu=(if sb[22]{(yl+yl)}else{vd});let yv=(if sb[22]{(yn+yn)}else{ve});let yw=(if sb[22]{(yp+yp)}else{vf});let yx=(if sb[22]{(yr+yr)}else{vg});let zg=(if sb[22]{(c3*((ye+(sf[55]*yt))+((eI*ye)+(eD*(c8*yt)))))}else{vZ});let zh=(if sb[22]{((eK*o1)+(c3*((yf+(sf[55]*yu))+((eI*yf)+(eD*(c8*yu))))))}else{w0});let zi=(if sb[22]{((eK*o2)+(c3*((yg+(sf[55]*yv))+((eI*yg)+(eD*(c8*yv))))))}else{w1});let zj=(if sb[22]{(c3*((yh+(sf[55]*yw))+((eI*yh)+(eD*(c8*yw)))))}else{w2});let zk=(if sb[22]{(c3*((sf[162]+(sf[55]*yx))+((eI*sf[162])+(eD*(c8*yx)))))}else{w3});let As=scalar_limexp_derivative(eM);let AD=scalar_limexp_derivative(eY);let AU=(S-(f2*f2));let B0=(if sb[22]{((bF*((zg*As)-((-zg)*AD)))*AU)}else{wC});let B1=(if sb[22]{((bF*((zh*As)-((-zh)*AD)))*AU)}else{wD});let B2=(if sb[22]{((bF*((zi*As)-((-zi)*AD)))*AU)}else{wE});let B3=(if sb[22]{((bF*((zj*As)-((-zj)*AD)))*AU)}else{wF});let B4=(if sb[22]{((bF*((zk*As)-((-zk)*AD)))*AU)}else{wG});let GL=(gd*gd);let Hg=(gq*gq);let J1=(S-(hQ*hQ));let J2=(sf[89]*J1);let J3=(sf[177]*J1);let J4=(sf[88]*J1);let J7=(S-(hW*hW));let J8=(sf[91]*J7);let J9=(sf[178]*J7);let Jc=(S-(i2*i2));let Jd=(sf[179]*Jc);let Je=(sf[93]*Jc);let Ji=(S-(i9*i9));let Jj=(sf[181]*Ji);let Jk=(sf[89]*Ji);let Jl=(sf[94]*Ji);let JM=(iF).sinh();let JN=(sf[89]*JM);let JO=(sf[176]*JM);let JP=(if sb[43]{JN}else{n_});let JQ=(if sb[43]{JO}else{n_});let JT=(if sb[43]{(JP/iH)}else{n_});let JU=(if sb[43]{(JQ/iH)}else{n_});let JV=(hP).sinh();let JW=(sf[89]*JV);let JX=(sf[177]*JV);let JY=(sf[88]*JV);let JZ=(if sb[43]{JW}else{n_});let K0=(if sb[43]{JX}else{n_});let K1=(if sb[43]{JY}else{n_});let K5=(if sb[43]{(JZ/iL)}else{n_});let K6=(if sb[43]{(K0/iL)}else{n_});let K7=(if sb[43]{(K1/iL)}else{n_});let Ka=(if sb[43]{(sf[89]+JT)}else{n_});let Kb=(if sb[43]{(sf[176]+JU)}else{n_});let KC=(if sb[43]{(sf[100]+(bp*(sf[104]+((iE*(sf[88]+K7))/sf[88]))))}else{n_});let KD=(j0).sinh();let KE=(sf[176]*KD);let KF=(sf[89]*KD);let KG=(if sb[43]{KE}else{JP});let KH=(if sb[43]{KF}else{JQ});let KK=(if sb[43]{(KG/j2)}else{n_});let KL=(if sb[43]{(KH/j2)}else{n_});let KM=(i8_).sinh();let KN=(sf[181]*KM);let KO=(sf[89]*KM);let KP=(sf[94]*KM);let KQ=(if sb[43]{KN}else{JZ});let KR=(if sb[43]{KO}else{K0});let KS=(if sb[43]{KP}else{n_});let KT=(if sb[43]{n_}else{K1});let KY=(if sb[43]{(KQ/j6)}else{n_});let KZ=(if sb[43]{(KR/j6)}else{n_});let L0=(if sb[43]{(KS/j6)}else{n_});let L1=(if sb[43]{(KT/j6)}else{n_});let L4=(if sb[43]{(sf[176]+KK)}else{n_});let L5=(if sb[43]{(sf[89]+KL)}else{n_});let Ly=(if sb[43]{(sf[102]+(bq*(sf[104]+((i4*(sf[94]+L0))/sf[94]))))}else{n_});let LK=(jv*sf[187]);let LL=(LK+LK);let LM=(jv*sf[188]);let LN=(LM+LM);let LQ=(sf[108]*f64::powf(jz,sf[189]));let M7=(S-(jL*jL));let Mj=(S-(jW*jW));let MU=(if sb[49]{JN}else{KG});let MV=(if sb[49]{JO}else{KH});let N0=(if sb[49]{JW}else{KQ});let N1=(if sb[49]{JX}else{KR});let N2=(if sb[49]{n_}else{KS});let N3=(if sb[49]{JY}else{KT});let Nd=(cP*kn);let Ni=(sf[114]*f64::powf(kp,sf[198]));let NY=(if sb[49]{(bp*(((kF*((sf[89]+(if sb[49]{(N0/kh)}else{K5}))-(if sb[49]{(sf[89]+(if sb[49]{(MU/ke)}else{JT}))}else{Ka})))+(kE*J8))/sf[88]))}else{(if sb[43]{(bp*(((iR*J8)+(iE*((sf[89]+K5)-Ka)))/sf[88]))}else{n_})});let NZ=(if sb[49]{(sf[183]+(bp*(sf[182]+(((kF*((if sb[49]{((kr*sf[197])+(km*((sf[185]*Nd)*Ni)))}else{n_})+((sf[177]+(if sb[49]{(N1/kh)}else{K6}))-(if sb[49]{(sf[176]+(if sb[49]{(MV/ke)}else{JU}))}else{Kb}))))+(kE*J9))/sf[88]))))}else{(if sb[43]{((bp*((((iR*J9)+(iE*((sf[177]+K6)-Kb)))/sf[88])+sf[182]))+sf[183])}else{n_})});let O0=(if sb[49]{(bp*((kF*(if sb[49]{(N2/kh)}else{n_}))/sf[88]))}else{n_});let O1=(if sb[49]{(sf[100]+(bp*(sf[104]+((kF*((if sb[49]{((sf[112]*kr)+(km*((sf[186]*Nd)*Ni)))}else{n_})+(sf[88]+(if sb[49]{(N3/kh)}else{K7}))))/sf[88]))))}else{KC});
        let ON=(if sb[49]{(sf[184]+(bq*(sf[182]+(((kV*Jd)+(jQ*((sf[181]+(if sb[49]{((if sb[49]{KN}else{N0})/kP)}else{KY}))-(if sb[49]{(sf[176]+(if sb[49]{((if sb[49]{KE}else{MU})/kM)}else{KK}))}else{L4}))))/sf[94]))))}else{(if sb[43]{((bq*(sf[182]+(((jc*Jd)+(i4*((sf[181]+KY)-L4)))/sf[94])))+sf[184])}else{n_})});let OO=(if sb[49]{(bq*(((kV*Je)+(jQ*((sf[89]+(if sb[49]{((if sb[49]{KO}else{N1})/kP)}else{KZ}))-(if sb[49]{(sf[89]+(if sb[49]{((if sb[49]{KF}else{MV})/kM)}else{KL}))}else{L5}))))/sf[94]))}else{(if sb[43]{(bq*(((jc*Je)+(i4*((sf[89]+KZ)-L5)))/sf[94]))}else{n_})});let OP=(if sb[49]{(sf[102]+(bq*(sf[104]+((jQ*(sf[94]+(if sb[49]{((if sb[49]{KP}else{N2})/kP)}else{L0})))/sf[94]))))}else{Ly});let OQ=(if sb[49]{(bq*((jQ*(if sb[49]{((if sb[49]{n_}else{N3})/kP)}else{L1}))/sf[94]))}else{(if sb[43]{(bq*((i4*L1)/sf[94]))}else{n_})});let Pk=(f*(if sb[49]{n_}else{(if sb[46]{(bq*((jY*Jd)+(jR*(if sb[46]{(sf[195]*Mj)}else{Jj}))))}else{(if sb[43]{n_}else{(if sb[40]{(bq*((ia*Jd)+(i4*Jj)))}else{n_})})})}));let Pm=(f*(if sb[49]{n_}else{(if sb[46]{(bq*((jY*Je)+(jR*(if sb[46]{(sf[196]*Mj)}else{Jk}))))}else{(if sb[43]{n_}else{(if sb[40]{(bq*((ia*Je)+(i4*Jk)))}else{n_})})})}));let Pn=(f*(if sb[49]{n_}else{(if sb[46]{(bq*(jR*(if sb[46]{(sf[94]*Mj)}else{Jl})))}else{(if sb[43]{n_}else{(if sb[40]{(bq*(i4*Jl))}else{n_})})})}));let Pv=(j*(if sb[49]{n_}else{(if sb[46]{((k2*J8)+(jO*(bp*(if sb[46]{(sf[191]*M7)}else{J2}))))}else{(if sb[43]{n_}else{(if sb[40]{((iq*J8)+(hX*(bp*J2)))}else{n_})})})}));let Px=(j*(if sb[49]{n_}else{(if sb[46]{((k2*J9)+(jO*(bp*((if sb[46]{(sf[192]*M7)}else{J3})+(sf[112]*(if sb[46]{((jF*(LL*LQ))+(jB*(sf[110]*LL)))}else{n_}))))))}else{(if sb[43]{n_}else{(if sb[40]{((iq*J9)+(hX*(bp*J3)))}else{n_})})})}));let Pz=(j*(if sb[49]{n_}else{(if sb[46]{(jO*(bp*((if sb[46]{(sf[88]*M7)}else{J4})+(sf[112]*(if sb[46]{((jF*(LN*LQ))+(jB*(sf[110]*LN)))}else{n_})))))}else{(if sb[43]{n_}else{(if sb[40]{(hX*(bp*J4))}else{n_})})})}));let PJ=(-bs);let PS=(if (sf[122]!=0.0){(mv*(if sb[28]{((-(br_*B0))/Hg)}else{(if (sf[66]!=0.0){((-(br_*ph))/GL)}else{n_})}))}else{n_});let PT=(if (sf[122]!=0.0){(mv*(if sb[28]{((-(br_*B1))/Hg)}else{(if (sf[66]!=0.0){((-(br_*pi))/GL)}else{n_})}))}else{n_});let PU=(if (sf[122]!=0.0){(mv*(if sb[28]{((-(br_*B2))/Hg)}else{(if (sf[66]!=0.0){((-(br_*pj))/GL)}else{n_})}))}else{n_});let PV=(if (sf[122]!=0.0){(mv*(if sb[28]{((-(br_*B3))/Hg)}else{(if (sf[66]!=0.0){((-(br_*pk))/GL)}else{n_})}))}else{n_});let PW=(if (sf[122]!=0.0){(mv*(if sb[28]{((-(br_*B4))/Hg)}else{(if (sf[66]!=0.0){((-(br_*pl))/GL)}else{n_})}))}else{n_});let Q2=(-bu);

        CommonStampValues {
            a, b, d, f, g, h, i, j,
            l, n_, G, J, N, S, T, V,
            aw, b6, ba, bh, bm, bs, bu, bz,
            bF, bU, c3, c8, ci, cq, cz, d8,
            d9, db, eg, eD, f4, gt, gH, it,
            iz, k4, k9, kL, l1, lQ, lW, m6,
            m9, mo, mq, ms, mt, mw, mx, mE,
            mI, mK, n0, n6, nc, nh, np, nq,
            nD, o1, o2, om, or, ot, ov, ox,
            oy, ph, pi, pj, pk, pl, qU, qV,
            qW, qX, qY, qZ, r0, r9, ra, rb,
            rc, wC, wD, wE, wF, wG, ye, yf,
            yg, yh, B0, B1, B2, B3, B4, KC,
            Ly, NY, NZ, O0, O1, ON, OO, OP,
            OQ, Pk, Pm, Pn, Pv, Px, Pz, PJ,
            PS, PT, PU, PV, PW, Q2,
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        let br=self.branches;
        let branches=br;
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
            a, b, d, f, g, h, i, j,
            l, n_, G, J, N, S, T, V,
            aw, b6, ba, bh, bm, bs, bu, bz,
            bF, bU, c3, c8, ci, cq, cz, d8,
            d9, db, eg, eD, f4, gt, gH, it,
            iz, k4, k9, kL, l1, lQ, lW, m6,
            m9, mo, mq, ms, mt, mw, mx, mE,
            mI, mK, n0, n6, nc, nh, np, nq,
            nD, o1, o2, om, or, ot, ov, ox,
            oy, ph, pi, pj, pk, pl, qU, qV,
            qW, qX, qY, qZ, r0, r9, ra, rb,
            rc, wC, wD, wE, wF, wG, ye, yf,
            yg, yh, B0, B1, B2, B3, B4, KC,
            Ly, NY, NZ, O0, O1, ON, OO, OP,
            OQ, Pk, Pm, Pn, Pv, Px, Pz, PJ,
            PS, PT, PU, PV, PW, Q2,
        }=self.eval_common_stamp_values(ctx);
        let m_=ctx.node_voltage(n[16]);let Y=(sf[11]*(S+(sf[12]*V)));let bn=(if bm{sf[13]}else{(if (T!=0.0){(sf[13]*(S+(V*sf[14])))}else{n_})});let bo=(if bm{sf[15]}else{(if (T!=0.0){(sf[15]*(S+(V*sf[16])))}else{n_})});let bt=(if bm{sf[37]}else{(if bh{(aw*sf[37])}else{(if b6{(sf[37]*ba)}else{n_})})});let by=(if bm{sf[30]}else{(if (T!=0.0){(sf[30]+(N*sf[31]))}else{n_})});let bM=(if sb[11]{sf[44]}else{(if (sf[41]!=0.0){(sf[43]/(J*8.617333262145179e-5))}else{n_})});let cJ=(sf[56]+(sf[53]*cz));let cL=((h*cJ)).tanh();let cW=(bn*cz);let cX=(cL*cW);let d3=((S+(h*sf[62]))+(bo*scalar_limexp(ci)));let dd=(if sb[16]{(d9*db)}else{cq});let dj=(if sb[16]{(((c3*d9)+(sf[55]*db))+(c8*dd))}else{n_});let dk=(dj).tanh();let dm=(if sb[16]{(S+dk)}else{n_});let dp=(if sb[16]{(sf[56]+(sf[53]*dm))}else{n_});let ds=(sf[62]+(cz*sf[63]));let dt=(if sb[16]{ds}else{n_});let du=(S+cL);let dv=(cW*du);let dA=(sf[64]*(h-bz));let dC=(bo*scalar_limexp(dA));let dD=((S+(h*dt))+dC);let dF=(if sb[16]{(dv*dD)}else{n_});let dI=(if sb[16]{(sf[62]+(dm*sf[63]))}else{n_});let dK=((h*dp)).tanh();let dM=(bn*dm);let dN=(S-(if sb[16]{dK}else{n_}));let dO=(dM*dN);let dQ=(S-(h*dI));let dS=(if sb[16]{(dO*dQ)}else{n_});let ej=(if sb[19]{(sf[56]+(sf[53]*eg))}else{n_});let el=((h*ej)).tanh();let em=(if sb[19]{el}else{n_});let ep=(if sb[19]{(sf[62]+(sf[63]*eg))}else{dt});let eq=(bn*eg);let er=(em*eq);let eu=(ci*sf[64]);let ex=((S+(h*ep))+(bo*scalar_limexp(eu)));let eN=(if sb[22]{d8}else{dd});let eP=(if sb[22]{(eN*eN)}else{n_});let eS=(c8*eN);let eU=((eN+(sf[55]*eP))+(eP*eS));let eW=(if sb[22]{(c3*eU)}else{dj});let f6=(-eW);let fa=((bF*(scalar_limexp(eW)-scalar_limexp(f6)))).tanh();let fc=(if sb[22]{(S+fa)}else{n_});let fe=(sf[56]+(sf[53]*f4));let ff=(if sb[22]{fe}else{ej});let fi=(if sb[22]{(sf[56]+(sf[53]*fc))}else{n_});let fk=((h*ff)).tanh();let fl=(if sb[22]{fk}else{em});let fn_=((h*fi)).tanh();let fr=(if sb[22]{(sf[62]+(sf[63]*fc))}else{n_});let fu=(if sb[22]{(sf[62]+(sf[63]*f4))}else{n_});let fv=(bn*f4);let fw=(S+fl);let fx=(fv*fw);let fA=(dC+(S+(h*fu)));let fD=(bn*fc);let fE=(S-(if sb[22]{fn_}else{n_}));let fF=(fD*fE);let fH=(S-(h*fr));let fQ=(if sb[25]{ds}else{ep});let fR=(if sb[25]{fe}else{ff});let fT=((h*fR)).tanh();let fW=((l*fR)).tanh();let g0=((if sb[25]{fT}else{fl})+((if sb[25]{fW}else{n_})*sf[65]));let g1=(cW*g0);let g3=(h+(l*sf[65]));let g6=(dC+(S+(fQ*g3)));let g8=(if sb[25]{(g1*g6)}else{(if sb[22]{(bF*((if sb[22]{(fx*fA)}else{dF})-(if sb[22]{(fF*fH)}else{dS})))}else{(if sb[19]{(er*ex)}else{(if sb[16]{(bF*(dF-dS))}else{(if (sf[57]!=0.0){(cX*d3)}else{n_})})})})});let gj=(cz*sf[69]);let gu=(f4*sf[69]);let gB=(S+(V*sf[71]));let gC=((if sb[28]{(sf[70]+gu)}else{(if (sf[66]!=0.0){(gj+sf[70])}else{n_})})*gB);let gD=((if sb[28]{(sf[68]+gu)}else{(if (sf[66]!=0.0){(sf[68]+gj)}else{n_})})*gB);let gN=(j-by);let gR=((-j)-sf[74]);let gT=(f-by);let gW=(g-sf[75]);let h2=(if sb[30]{scalar_limexp((by*(-bM)))}else{(if (sf[73]!=0.0){scalar_limexp((bM*((-by)).tanh()))}else{eD})});let he=(gN).tanh();let hg=(gT).tanh();let ho=(sf[76]*(if sb[30]{gR}else{(if (sf[73]!=0.0){gR}else{n_})}));let hs=(bM*(if sb[34]{gN}else{(if sb[32]{he}else{(if (sf[73]!=0.0){gN}else{n_})})}));let hA=(sf[85]*((scalar_limexp(hs)-((scalar_limexp(ho)-sf[80])*sf[87]))-h2));let hB=(sf[76]*(if sb[30]{gW}else{(if (sf[73]!=0.0){gW}else{n_})}));let hE=(bM*(if sb[34]{gT}else{(if sb[32]{hg}else{(if (sf[73]!=0.0){gT}else{n_})})}));let jl=KC;let jn=Ly;let l2=O1;let l3=(if sb[49]{l2}else{(if sb[46]{k4}else{(if sb[43]{jl}else{it})})});let l4=OP;let l5=(if sb[49]{l4}else{(if sb[46]{k9}else{(if sb[43]{jn}else{iz})})});let lU=(if sb[67]{((S-(lQ*lQ))).sqrt()}else{n_});let lY=(if sb[67]{((-lQ)*lW)}else{n_});let ma=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, l1);
        let mc=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, kL);let mg=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (f*l5));let mj=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (j*l3));let my=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, mx);let mF=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, mE);let mO=ctx.node_voltage(n[13]);let n1=ctx.branch_current(br[11]);let n7=ctx.branch_current(br[15]);let ni=(if sb[67]{nh}else{n_});let nj=ctx.node_voltage(n[18]);let nr=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, nq);let nE=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, nD);let q8=(S-(cL*cL));let qe=(bn*ph);let qf=(bn*pi);let qg=(bn*pj);let qh=(bn*pk);let qi=(bn*pl);let qj=(cW*((h*(sf[53]*ph))*q8));let qm=(cW*((cJ+(h*(sf[53]*pi)))*q8));let qp=(cW*(((-cJ)+(h*(sf[53]*pj)))*q8));let qs=(cW*((h*(sf[53]*pk))*q8));let qv=(cW*((h*(sf[53]*pl))*q8));let qz=scalar_limexp_derivative(ci);let rr=(if sb[16]{((db*qX)+(d9*r9))}else{or});let rs=(if sb[16]{((db*qY)+(d9*ra))}else{ot});let rt=(if sb[16]{((db*qZ)+(d9*rb))}else{ov});let ru=(if sb[16]{((db*r0)+(d9*rc))}else{ox});let rv=(if sb[16]{(d9*sf[158])}else{oy});let rX=(if sb[16]{(((c3*qX)+(sf[55]*r9))+(c8*rr))}else{n_});let rY=(if sb[16]{((((d9*o1)+(c3*qY))+(sf[55]*ra))+(c8*rs))}else{n_});let rZ=(if sb[16]{((((d9*o2)+(c3*qZ))+(sf[55]*rb))+(c8*rt))}else{n_});let s0=(if sb[16]{(((c3*r0)+(sf[55]*rc))+(c8*ru))}else{n_});let s1=(if sb[16]{(sf[159]+(c8*rv))}else{n_});let s3=(S-(dk*dk));let s9=(if sb[16]{(rX*s3)}else{n_});let sa=(if sb[16]{(rY*s3)}else{n_});let sb_=(if sb[16]{(rZ*s3)}else{n_});let sc=(if sb[16]{(s0*s3)}else{n_});let sd=(if sb[16]{(s1*s3)}else{n_});let so=(sf[63]*ph);let sp=(sf[63]*pi);let sq=(sf[63]*pj);let sr=(sf[63]*pk);let ss=(sf[63]*pl);let st=(if sb[16]{so}else{n_});let su=(if sb[16]{sp}else{n_});let sv=(if sb[16]{sq}else{n_});let sw=(if sb[16]{sr}else{n_});let sx=(if sb[16]{ss}else{n_});let sR=scalar_limexp_derivative(dA);let sU=(bo*(sf[64]*sR));let sV=(bo*(sf[160]*sR));let td=(if sb[16]{((dD*(qj+(du*qe)))+(dv*(h*st)))}else{n_});let te=(if sb[16]{((dD*(qm+(du*qf)))+(dv*((dt+(h*su))+sU)))}else{n_});let tf=(if sb[16]{((dD*(qp+(du*qg)))+(dv*(((-dt)+(h*sv))+sV)))}else{n_});let tg=(if sb[16]{((dD*(qs+(du*qh)))+(dv*(h*sw)))}else{n_});let th=(if sb[16]{((dD*(qv+(du*qi)))+(dv*(h*sx)))}else{n_});let tB=(S-(dK*dK));let uD=(if sb[16]{((dQ*((dN*(bn*s9))+(dM*(-(if sb[16]{((h*(if sb[16]{(sf[53]*s9)}else{n_}))*tB)}else{n_})))))+(dO*(-(h*(if sb[16]{(sf[63]*s9)}else{n_})))))}else{n_});let uE=(if sb[16]{((dQ*((dN*(bn*sa))+(dM*(-(if sb[16]{((dp+(h*(if sb[16]{(sf[53]*sa)}else{n_})))*tB)}else{n_})))))+(dO*(-(dI+(h*(if sb[16]{(sf[63]*sa)}else{n_}))))))}else{n_});
        let uF=(if sb[16]{((dQ*((dN*(bn*sb_))+(dM*(-(if sb[16]{(((-dp)+(h*(if sb[16]{(sf[53]*sb_)}else{n_})))*tB)}else{n_})))))+(dO*(-((-dI)+(h*(if sb[16]{(sf[63]*sb_)}else{n_}))))))}else{n_});let uG=(if sb[16]{((dQ*((dN*(bn*sc))+(dM*(-(if sb[16]{((h*(if sb[16]{(sf[53]*sc)}else{n_}))*tB)}else{n_})))))+(dO*(-(h*(if sb[16]{(sf[63]*sc)}else{n_})))))}else{n_});let uH=(if sb[16]{((dQ*((dN*(bn*sd))+(dM*(-(if sb[16]{((h*(if sb[16]{(sf[53]*sd)}else{n_}))*tB)}else{n_})))))+(dO*(-(h*(if sb[16]{(sf[63]*sd)}else{n_})))))}else{n_});let wM=(if sb[19]{(sf[53]*wC)}else{n_});let wN=(if sb[19]{(sf[53]*wD)}else{n_});let wO=(if sb[19]{(sf[53]*wE)}else{n_});let wP=(if sb[19]{(sf[53]*wF)}else{n_});let wQ=(if sb[19]{(sf[53]*wG)}else{n_});let x0=(S-(el*el));let x6=(if sb[19]{((h*wM)*x0)}else{n_});let x7=(if sb[19]{((ej+(h*wN))*x0)}else{n_});let x8=(if sb[19]{(((-ej)+(h*wO))*x0)}else{n_});let x9=(if sb[19]{((h*wP)*x0)}else{n_});let xa=(if sb[19]{((h*wQ)*x0)}else{n_});let xg=(if sb[19]{(sf[63]*wC)}else{st});let xh=(if sb[19]{(sf[63]*wD)}else{su});let xi=(if sb[19]{(sf[63]*wE)}else{sv});let xj=(if sb[19]{(sf[63]*wF)}else{sw});let xk=(if sb[19]{(sf[63]*wG)}else{sx});let xN=scalar_limexp_derivative(eu);let zl=(if sb[22]{om}else{rr});let zm=(if sb[22]{qU}else{rs});let zn=(if sb[22]{qV}else{rt});let zo=(if sb[22]{qW}else{ru});let zp=(if sb[22]{n_}else{rv});let zq=(eN*zl);let zs=(eN*zm);let zu=(eN*zn);let zw=(eN*zo);let zy=(eN*zp);let zA=(if sb[22]{(zq+zq)}else{n_});let zB=(if sb[22]{(zs+zs)}else{n_});let zC=(if sb[22]{(zu+zu)}else{n_});let zD=(if sb[22]{(zw+zw)}else{n_});let zE=(if sb[22]{(zy+zy)}else{n_});let An=(if sb[22]{(c3*((zl+(sf[55]*zA))+((eS*zA)+(eP*(c8*zl)))))}else{rX});let Ao=(if sb[22]{((eU*o1)+(c3*((zm+(sf[55]*zB))+((eS*zB)+(eP*(c8*zm))))))}else{rY});let Ap=(if sb[22]{((eU*o2)+(c3*((zn+(sf[55]*zC))+((eS*zC)+(eP*(c8*zn))))))}else{rZ});let Aq=(if sb[22]{(c3*((zo+(sf[55]*zD))+((eS*zD)+(eP*(c8*zo)))))}else{s0});let Ar=(if sb[22]{(c3*((zp+(sf[55]*zE))+((eS*zE)+(eP*(c8*zp)))))}else{s1});let B5=scalar_limexp_derivative(eW);let Bg=scalar_limexp_derivative(f6);let Bx=(S-(fa*fa));let BD=(if sb[22]{((bF*((An*B5)-((-An)*Bg)))*Bx)}else{n_});let BE=(if sb[22]{((bF*((Ao*B5)-((-Ao)*Bg)))*Bx)}else{n_});let BF=(if sb[22]{((bF*((Ap*B5)-((-Ap)*Bg)))*Bx)}else{n_});let BG=(if sb[22]{((bF*((Aq*B5)-((-Aq)*Bg)))*Bx)}else{n_});let BH=(if sb[22]{((bF*((Ar*B5)-((-Ar)*Bg)))*Bx)}else{n_});let BI=(sf[53]*B0);let BJ=(sf[53]*B1);let BK=(sf[53]*B2);let BL=(sf[53]*B3);let BM=(sf[53]*B4);let BN=(if sb[22]{BI}else{wM});let BO=(if sb[22]{BJ}else{wN});let BP=(if sb[22]{BK}else{wO});let BQ=(if sb[22]{BL}else{wP});let BR=(if sb[22]{BM}else{wQ});let Cb=(S-(fk*fk));let Ch=(if sb[22]{((h*BN)*Cb)}else{x6});let Ci=(if sb[22]{((ff+(h*BO))*Cb)}else{x7});let Cj=(if sb[22]{(((-ff)+(h*BP))*Cb)}else{x8});let Ck=(if sb[22]{((h*BQ)*Cb)}else{x9});let Cl=(if sb[22]{((h*BR)*Cb)}else{xa});let Cv=(S-(fn_*fn_));let EV=(if sb[22]{(bF*((if sb[22]{((fA*((fw*(bn*B1))+(fv*Ci)))+(fx*(sU+(fu+(h*(if sb[22]{(sf[63]*B1)}else{n_}))))))}else{te})-(if sb[22]{((fH*((fE*(bn*BE))+(fD*(-(if sb[22]{((fi+(h*(if sb[22]{(sf[53]*BE)}else{n_})))*Cv)}else{n_})))))+(fF*(-(fr+(h*(if sb[22]{(sf[63]*BE)}else{n_}))))))}else{uE})))}else{(if sb[19]{((ex*((eq*x7)+(em*(bn*wD))))+(er*((ep+(h*xh))+(bo*(sf[64]*xN)))))}else{(if sb[16]{(bF*(te-uE))}else{(if (sf[57]!=0.0){((d3*(qm+(cL*qf)))+(cX*(sf[62]+(bo*qz))))}else{n_})})})});let F4=(if sb[25]{BI}else{BN});let F5=(if sb[25]{BJ}else{BO});let F6=(if sb[25]{BK}else{BP});let F7=(if sb[25]{BL}else{BQ});let F8=(if sb[25]{BM}else{BR});let Fc=(-fR);let Fi=(S-(fT*fT));let FB=(S-(fW*fW));
        let GE=(if sb[25]{((g6*((g0*qe)+(cW*((if sb[25]{((h*F4)*Fi)}else{Ch})+(sf[65]*(if sb[25]{((fR+(l*F4))*FB)}else{n_}))))))+(g1*((g3*(if sb[25]{so}else{xg}))+(fQ*sf[65]))))}else{(if sb[22]{(bF*((if sb[22]{((fA*((fw*(bn*B0))+(fv*Ch)))+(fx*(h*(if sb[22]{(sf[63]*B0)}else{n_}))))}else{td})-(if sb[22]{((fH*((fE*(bn*BD))+(fD*(-(if sb[22]{((h*(if sb[22]{(sf[53]*BD)}else{n_}))*Cv)}else{n_})))))+(fF*(-(h*(if sb[22]{(sf[63]*BD)}else{n_})))))}else{uD})))}else{(if sb[19]{((ex*((eq*x6)+(em*(bn*wC))))+(er*(h*xg)))}else{(if sb[16]{(bF*(td-uD))}else{(if (sf[57]!=0.0){(d3*(qj+(cL*qe)))}else{n_})})})})});let GG=(if sb[25]{((g6*((g0*qg)+(cW*((if sb[25]{((Fc+(h*F6))*Fi)}else{Cj})+(sf[65]*(if sb[25]{((Fc+(l*F6))*FB)}else{n_}))))))+(g1*(sV+((g3*(if sb[25]{sq}else{xi}))+(fQ*sf[164])))))}else{(if sb[22]{(bF*((if sb[22]{((fA*((fw*(bn*B2))+(fv*Cj)))+(fx*(sV+((-fu)+(h*(if sb[22]{(sf[63]*B2)}else{n_}))))))}else{tf})-(if sb[22]{((fH*((fE*(bn*BF))+(fD*(-(if sb[22]{(((-fi)+(h*(if sb[22]{(sf[53]*BF)}else{n_})))*Cv)}else{n_})))))+(fF*(-((-fr)+(h*(if sb[22]{(sf[63]*BF)}else{n_}))))))}else{uF})))}else{(if sb[19]{((ex*((eq*x8)+(em*(bn*wE))))+(er*((-ep)+(h*xi))))}else{(if sb[16]{(bF*(tf-uF))}else{(if (sf[57]!=0.0){((d3*(qp+(cL*qg)))+(cX*sf[157]))}else{n_})})})})});let GH=(if sb[25]{((g6*((g0*qh)+(cW*((if sb[25]{((h*F7)*Fi)}else{Ck})+(sf[65]*(if sb[25]{((l*F7)*FB)}else{n_}))))))+(g1*(g3*(if sb[25]{sr}else{xj}))))}else{(if sb[22]{(bF*((if sb[22]{((fA*((fw*(bn*B3))+(fv*Ck)))+(fx*(h*(if sb[22]{(sf[63]*B3)}else{n_}))))}else{tg})-(if sb[22]{((fH*((fE*(bn*BG))+(fD*(-(if sb[22]{((h*(if sb[22]{(sf[53]*BG)}else{n_}))*Cv)}else{n_})))))+(fF*(-(h*(if sb[22]{(sf[63]*BG)}else{n_})))))}else{uG})))}else{(if sb[19]{((ex*((eq*x9)+(em*(bn*wF))))+(er*((h*xj)+(bo*(sf[160]*xN)))))}else{(if sb[16]{(bF*(tg-uG))}else{(if (sf[57]!=0.0){((d3*(qs+(cL*qh)))+(cX*(bo*(-qz))))}else{n_})})})})});let GI=(if sb[25]{((g6*((g0*qi)+(cW*((if sb[25]{((h*F8)*Fi)}else{Cl})+(sf[65]*(if sb[25]{((l*F8)*FB)}else{n_}))))))+(g1*(g3*(if sb[25]{ss}else{xk}))))}else{(if sb[22]{(bF*((if sb[22]{((fA*((fw*(bn*B4))+(fv*Cl)))+(fx*(h*(if sb[22]{(sf[63]*B4)}else{n_}))))}else{th})-(if sb[22]{((fH*((fE*(bn*BH))+(fD*(-(if sb[22]{((h*(if sb[22]{(sf[53]*BH)}else{n_}))*Cv)}else{n_})))))+(fF*(-(h*(if sb[22]{(sf[63]*BH)}else{n_})))))}else{uH})))}else{(if sb[19]{((ex*((eq*xa)+(em*(bn*wG))))+(er*(h*xk)))}else{(if sb[16]{(bF*(th-uH))}else{(if (sf[57]!=0.0){(d3*(qv+(cL*qi)))}else{n_})})})})});let HJ=(gB*(if sb[28]{(sf[69]*B0)}else{(if (sf[66]!=0.0){(sf[69]*ph)}else{n_})}));let HK=(gB*(if sb[28]{(sf[69]*B1)}else{(if (sf[66]!=0.0){(sf[69]*pi)}else{n_})}));let HL=(gB*(if sb[28]{(sf[69]*B2)}else{(if (sf[66]!=0.0){(sf[69]*pj)}else{n_})}));let HM=(gB*(if sb[28]{(sf[69]*B3)}else{(if (sf[66]!=0.0){(sf[69]*pk)}else{n_})}));let HN=(gB*(if sb[28]{(sf[69]*B4)}else{(if (sf[66]!=0.0){(sf[69]*pl)}else{n_})}));let HW=(if sb[30]{n_}else{(if (sf[73]!=0.0){n_}else{yf})});let HX=(if sb[30]{n_}else{(if (sf[73]!=0.0){n_}else{yg})});let HY=(if sb[30]{n_}else{(if (sf[73]!=0.0){n_}else{yh})});let I1=(S-(he*he));let I6=(S-(hg*hg));let Ii=scalar_limexp_derivative(ho);let In=scalar_limexp_derivative(hs);let Iz=(sf[85]*(-(if sb[30]{n_}else{(if (sf[73]!=0.0){n_}else{ye})})));let IF=scalar_limexp_derivative(hB);let IK=scalar_limexp_derivative(hE);let P2=ddt_scale;let PK=-1e-12;

        stamper.stamp_current_sparse_local::<5, 0>(
            Some(15),
            None,
            multiplicity * ((-g8)),
            [4, 5, 8, 10, 12],
            [(-GE), (-(if sb[25]{((g6*((g0*qf)+(cW*((if sb[25]{((fR+(h*F5))*Fi)}else{Ci})+(sf[65]*(if sb[25]{((l*F5)*FB)}else{n_}))))))+(g1*(sU+(fQ+(g3*(if sb[25]{sp}else{xh}))))))}else{EV})), (-GG), (-GH), (-GI)],
            [],
            [],
            multiplicity,
        );
        let m6_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, m6);
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (m6_ddt),
            15,
            multiplicity * (((sf[144]) * ddt_scale)),
        );
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (m_),
            16,
            multiplicity * (S),
        );
        let m9_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, m9);
        stamper.stamp_potential_branch_local(
            Some(15),
            Some(16),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            0,
            m9_ddt,
            0,
            ((sf[145]) * ddt_scale),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            Some(8),
            multiplicity * (m_),
            16,
            multiplicity * (S),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(8),
            multiplicity * (hA),
            [4, 5, 8, 10, 11, 12],
            [Iz, (sf[85]*(-HW)), (sf[85]*((((bM*(if sb[34]{gH}else{(if sb[32]{(-I1)}else{sf[166]})}))*In)-(sf[87]*(sf[171]*Ii)))-HX)), (sf[85]*(-HY)), (sf[85]*(((bM*(if sb[34]{S}else{(if sb[32]{I1}else{sf[167]})}))*In)-(sf[87]*(sf[172]*Ii)))), sf[174]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * ((sf[85]*((scalar_limexp(hE)-(sf[87]*(scalar_limexp(hB)-sf[83])))-h2))),
            [4, 5, 8, 10, 12],
            [Iz, (sf[85]*((((bM*(if sb[34]{gH}else{(if sb[32]{(-I6)}else{sf[166]})}))*IK)-(sf[87]*(sf[171]*IF)))-HW)), (sf[85]*(-HX)), (sf[85]*((((bM*(if sb[34]{S}else{(if sb[32]{I6}else{sf[167]})}))*IK)-(sf[87]*(sf[172]*IF)))-HY)), sf[174]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(10),
            Some(5),
            multiplicity * ((if (sf[120]!=0.0){ma}else{n_})),
            [5, 8, 10, 11],
            [(if (sf[120]!=0.0){(ON*P2)}else{n_}), (if (sf[120]!=0.0){(OO*P2)}else{n_}), (if (sf[120]!=0.0){(OP*P2)}else{n_}), (if (sf[120]!=0.0){(OQ*P2)}else{n_})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(8),
            multiplicity * ((if (sf[120]!=0.0){mc}else{n_})),
            [5, 8, 10, 11],
            [(if (sf[120]!=0.0){(NY*P2)}else{n_}), (if (sf[120]!=0.0){(NZ*P2)}else{n_}), (if (sf[120]!=0.0){(O0*P2)}else{n_}), (if (sf[120]!=0.0){(O1*P2)}else{n_})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(10),
            Some(5),
            multiplicity * ((if sb[69]{mg}else{n_})),
            5,
            multiplicity * ((if sb[69]{(P2*((-l5)+Pk))}else{n_})),
            8,
            multiplicity * ((if sb[69]{(P2*Pm)}else{n_})),
            10,
            multiplicity * ((if sb[69]{(P2*(l5+Pn))}else{n_})),
        );
        stamper.stamp_current_node3_local(
            Some(11),
            Some(8),
            multiplicity * ((if sb[69]{mj}else{n_})),
            5,
            multiplicity * ((if sb[69]{(P2*Pv)}else{n_})),
            8,
            multiplicity * ((if sb[69]{(P2*((-l3)+Px))}else{n_})),
            11,
            multiplicity * ((if sb[69]{(P2*(l3+Pz))}else{n_})),
        );
        let mo_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, mo);
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (mo_ddt),
            5,
            multiplicity * (((sf[199]) * ddt_scale)),
            7,
            multiplicity * (((sf[146]) * ddt_scale)),
        );
        let mq_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, mq);
        stamper.stamp_current_node2_local(
            Some(5),
            Some(8),
            multiplicity * (mq_ddt),
            5,
            multiplicity * (((sf[147]) * ddt_scale)),
            8,
            multiplicity * (((sf[200]) * ddt_scale)),
        );
        let mt_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, mt);
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (mt_ddt),
            4,
            multiplicity * (((PJ) * ddt_scale)),
            6,
            multiplicity * (((bs) * ddt_scale)),
        );
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * ((bU*ms)),
            4,
            multiplicity * (PK),
            6,
            multiplicity * (bU),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            1,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            1,
            (if (sf[122]!=0.0){(mw+my)}else{n_}),
            [4, 5, 8, 10, 12],
            [PS, PT, PU, PV, PW],
            [1],
            [(if (sf[122]!=0.0){(gt+(sf[121]*P2))}else{n_})],
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            n_,
        );
        stamper.stamp_current_node2_local(
            Some(11),
            Some(12),
            multiplicity * ((if (sf[123]!=0.0){((i-a)/bt)}else{n_})),
            11,
            multiplicity * ((if (sf[123]!=0.0){(S/bt)}else{n_})),
            12,
            multiplicity * ((if (sf[123]!=0.0){(gH/bt)}else{n_})),
        );
        stamper.stamp_current_node2_local(
            Some(12),
            Some(8),
            multiplicity * ((if (sf[123]!=0.0){mF}else{n_})),
            8,
            multiplicity * ((if (sf[123]!=0.0){(P2*Q2)}else{n_})),
            12,
            multiplicity * ((if (sf[123]!=0.0){(bu*P2)}else{n_})),
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(8),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            n_,
        );
        let mK_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, mK);
        stamper.stamp_current_node2_local(
            Some(11),
            Some(14),
            multiplicity * (mK_ddt),
            11,
            multiplicity * (((sf[148]) * ddt_scale)),
            14,
            multiplicity * (((sf[201]) * ddt_scale)),
        );
        stamper.stamp_current_node2_local(
            Some(14),
            Some(8),
            multiplicity * ((if (sf[125]!=0.0){((mI-b)/sf[124])}else{n_})),
            8,
            multiplicity * (sf[204]),
            14,
            multiplicity * (sf[205]),
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            n_,
        );
        stamper.stamp_current_node2_local(
            Some(13),
            Some(10),
            multiplicity * ((if (sf[127]!=0.0){((mO-d)/sf[126])}else{n_})),
            10,
            multiplicity * (sf[208]),
            13,
            multiplicity * (sf[209]),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(10),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            n_,
        );
        stamper.stamp_current_const_local(
            Some(13),
            Some(10),
            multiplicity * (n_),
        );
        stamper.stamp_current_node2_local(
            Some(13),
            Some(11),
            multiplicity * ((if (sf[129]!=0.0){((mO-i)/sf[128])}else{n_})),
            11,
            multiplicity * (sf[212]),
            13,
            multiplicity * (sf[213]),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(11),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            n_,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            7,
            (if (sf[131]!=0.0){(sf[130]*ctx.branch_current(br[7]))}else{n_}),
            7,
            sf[214],
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            8,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            8,
            n_,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            n_,
        );
        let n0_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, n0);
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(7),
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            10,
            n0_ddt,
            10,
            ((sf[149]) * ddt_scale),
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            11,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            11,
            (if (sf[132]!=0.0){(gC*n1)}else{n_}),
            [4, 5, 8, 10, 12],
            [(if (sf[132]!=0.0){(n1*HJ)}else{n_}), (if (sf[132]!=0.0){(n1*HK)}else{n_}), (if (sf[132]!=0.0){(n1*HL)}else{n_}), (if (sf[132]!=0.0){(n1*HM)}else{n_}), (if (sf[132]!=0.0){(n1*HN)}else{n_})],
            [11],
            [(if (sf[132]!=0.0){gC}else{n_})],
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            n_,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            13,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            13,
            n_,
        );
        let n6_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, n6);
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(2),
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            14,
            n6_ddt,
            14,
            ((sf[150]) * ddt_scale),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            15,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            15,
            (if (sf[133]!=0.0){(gD*n7)}else{n_}),
            [4, 5, 8, 10, 12],
            [(if (sf[133]!=0.0){(n7*HJ)}else{n_}), (if (sf[133]!=0.0){(n7*HK)}else{n_}), (if (sf[133]!=0.0){(n7*HL)}else{n_}), (if (sf[133]!=0.0){(n7*HM)}else{n_}), (if (sf[133]!=0.0){(n7*HN)}else{n_})],
            [15],
            [(if (sf[133]!=0.0){gD}else{n_})],
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            n_,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            17,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            17,
            n_,
        );
        let nc_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, nc);
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(0),
            18,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            18,
            nc_ddt,
            18,
            ((sf[151]) * ddt_scale),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (1e-15),
        );
        stamper.stamp_current_const_local(
            Some(14),
            Some(2),
            multiplicity * (bU),
        );
        stamper.stamp_current_node2_local(
            Some(12),
            Some(2),
            multiplicity * ((bU*(a-ctx.node_voltage(n[2])))),
            2,
            multiplicity * (PK),
            12,
            multiplicity * (bU),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (n_),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (n_),
        );
        stamper.stamp_current_const_local(
            Some(17),
            None,
            multiplicity * (n_),
        );
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (ni),
            17,
            multiplicity * (sf[215]),
        );
        stamper.stamp_current_const_local(
            Some(18),
            None,
            multiplicity * (n_),
        );
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * ((if sb[67]{nj}else{n_})),
            18,
            multiplicity * (sf[215]),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(8),
            multiplicity * (ni),
            17,
            multiplicity * (sf[215]),
        );
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * ((if sb[67]{((lY*nh)+(lU*nj))}else{n_})),
            17,
            multiplicity * ((if sb[67]{lY}else{n_})),
            18,
            multiplicity * ((if sb[67]{lU}else{n_})),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(5),
            multiplicity * ((if sb[67]{nr}else{n_})),
            17,
            multiplicity * ((if sb[67]{(np*P2)}else{n_})),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (n_),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (n_),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (n_),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (n_),
        );
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (nh),
            17,
            multiplicity * (S),
        );
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (nj),
            18,
            multiplicity * (S),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (n_),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (n_),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (n_),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (n_),
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * ((if (sf[143]!=0.0){(-(((h*g8)).abs()+((j*hA)).abs()))}else{n_})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if (sf[143]!=0.0){(G/Y)}else{n_})),
            3,
            multiplicity * ((if (sf[143]!=0.0){(S/Y)}else{n_})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if (sf[143]!=0.0){nE}else{n_})),
            3,
            multiplicity * ((if (sf[143]!=0.0){(sf[152]*P2)}else{n_})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if sb[70]{(G*bU)}else{n_})),
            3,
            multiplicity * (sf[216]),
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
            a, b, d, f, g, h, i, j,
            l, n_, G, J, N, S, T, V,
            aw, b6, ba, bh, bm, bs, bu, bz,
            bF, bU, c3, c8, ci, cq, cz, d8,
            d9, db, eg, eD, f4, gt, gH, it,
            iz, k4, k9, kL, l1, lQ, lW, m6,
            m9, mo, mq, ms, mt, mw, mx, mE,
            mI, mK, n0, n6, nc, nh, np, nq,
            nD, o1, o2, om, or, ot, ov, ox,
            oy, ph, pi, pj, pk, pl, qU, qV,
            qW, qX, qY, qZ, r0, r9, ra, rb,
            rc, wC, wD, wE, wF, wG, ye, yf,
            yg, yh, B0, B1, B2, B3, B4, KC,
            Ly, NY, NZ, O0, O1, ON, OO, OP,
            OQ, Pk, Pm, Pn, Pv, Px, Pz, PJ,
            PS, PT, PU, PV, PW, Q2,
        }=self.eval_common_stamp_values(ctx);
        let jl=KC;let jn=Ly;let l2=O1;let l3=(if sb[49]{l2}else{(if sb[46]{k4}else{(if sb[43]{jl}else{it})})});let l4=OP;let l5=(if sb[49]{l4}else{(if sb[46]{k9}else{(if sb[43]{jn}else{iz})})});let ma=0.0;let mc=0.0;let mg=0.0;let mj=0.0;let my=0.0;let mF=0.0;let nr=0.0;let nE=0.0;let P2=1.0;

        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (sf[144]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[15]),
            Some(nodes[16]),
            branches[0],
            multiplicity * (sf[145]),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            &[nodes[5], nodes[8], nodes[10], nodes[11]],
            &[(if (sf[120]!=0.0){(ON*P2)}else{n_}), (if (sf[120]!=0.0){(OO*P2)}else{n_}), (if (sf[120]!=0.0){(OP*P2)}else{n_}), (if (sf[120]!=0.0){(OQ*P2)}else{n_})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            &[nodes[5], nodes[8], nodes[10], nodes[11]],
            &[(if (sf[120]!=0.0){(NY*P2)}else{n_}), (if (sf[120]!=0.0){(NZ*P2)}else{n_}), (if (sf[120]!=0.0){(O0*P2)}else{n_}), (if (sf[120]!=0.0){(O1*P2)}else{n_})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * ((if sb[69]{(P2*((-l5)+Pk))}else{n_})),
            nodes[8],
            multiplicity * ((if sb[69]{(P2*Pm)}else{n_})),
            nodes[10],
            multiplicity * ((if sb[69]{(P2*(l5+Pn))}else{n_})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes[5],
            multiplicity * ((if sb[69]{(P2*Pv)}else{n_})),
            nodes[8],
            multiplicity * ((if sb[69]{(P2*((-l3)+Px))}else{n_})),
            nodes[11],
            multiplicity * ((if sb[69]{(P2*(l3+Pz))}else{n_})),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * (sf[199]),
            nodes[7],
            multiplicity * (sf[146]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            Some(nodes[8]),
            nodes[5],
            multiplicity * (sf[147]),
            nodes[8],
            multiplicity * (sf[200]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes[4],
            multiplicity * (PJ),
            nodes[6],
            multiplicity * (bs),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[8]),
            &[nodes[4], nodes[5], nodes[8], nodes[10], nodes[12]],
            &[PS, PT, PU, PV, PW],
            &[branches[1]],
            &[(if (sf[122]!=0.0){(gt+(sf[121]*P2))}else{n_})],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[12]),
            Some(nodes[8]),
            nodes[8],
            multiplicity * ((if (sf[123]!=0.0){(P2*Q2)}else{n_})),
            nodes[12],
            multiplicity * ((if (sf[123]!=0.0){(bu*P2)}else{n_})),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[11]),
            Some(nodes[14]),
            nodes[11],
            multiplicity * (sf[148]),
            nodes[14],
            multiplicity * (sf[201]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[1]),
            Some(nodes[7]),
            branches[10],
            multiplicity * (sf[149]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[9]),
            Some(nodes[2]),
            branches[14],
            multiplicity * (sf[150]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[6]),
            Some(nodes[0]),
            branches[18],
            multiplicity * (sf[151]),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[17],
            multiplicity * ((if sb[67]{(np*P2)}else{n_})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * ((if (sf[143]!=0.0){(sf[152]*P2)}else{n_})),
        );
    }
}
