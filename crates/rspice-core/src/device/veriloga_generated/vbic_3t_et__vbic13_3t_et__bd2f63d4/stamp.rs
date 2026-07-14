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
    b: f64, d: f64, M: f64, c7: f64, f0: f64, fl: f64,
    fm: f64, fn_: f64, gk: f64, gp: f64, gD: f64, hA: f64,
    iB: f64, jH: f64, jL: f64, jM: f64, jO: f64, jP: f64,
    jR: f64, jS: f64, jU: f64, jV: f64, k0: f64, k2: f64,
    k3: f64, k4: f64, k8: f64, kh: f64, ki: f64, ot: f64,
    ox: f64, oJ: f64, oN: f64, oW: f64, p3: f64, p6: f64,
    pb: f64, pg_: f64, pv: f64, pD: f64, pL: f64, pQ: f64,
    pU: f64, qc: f64, yG: f64, yJ: f64, EJ: f64, EL: f64,
    ET: f64, EZ: f64, F4: f64, F7: f64, Fb: f64, Fd: f64,
    Ff: f64, Fh: f64, Fk: f64, Fm: f64, Fo: f64, Fu: f64,
    Fw: f64, Fx: f64, GB: f64, GM: f64, Hi: f64, K7: f64,
    Lz: f64, UO: f64, UQ: f64, UR: f64, US: f64, UT: f64,
    UU: f64, Vg: f64, Vi: f64, Vj: f64, Vk: f64, Vl: f64,
    Vm: f64, VA: f64, W9: f64, Wa: f64, Wb: f64, Wc: f64,
    Wq: f64, Wr: f64, Ws: f64, Wt: f64, WV: f64, WW: f64,
    WX: f64, WY: f64, Xp: f64, Xq: f64, Xr: f64, Xs: f64,
    Y4: f64, Y6: f64, Y7: f64, Y8: f64, Y9: f64, Ya: f64,
    Yx: f64, Yy: f64, Yz: f64, afh: f64, afi: f64, afj: f64,
    afq: f64, afr: f64, afs: f64, avK: f64, avL: f64, avM: f64,
    avN: f64, avO: f64, avP: f64, avQ: f64, awq: f64, awr: f64,
    aws: f64, ax6: f64, ax7: f64, ax8: f64, axf: f64, axg: f64,
    axh: f64, axq: f64, axr: f64, axs: f64, axI: f64, axJ: f64,
    axK: f64, axO: f64, axP: f64, axQ: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let b=0.0;let d=1.0;let M=0.5;let b8=273.15;let bA=1.380662e-23;let bC=1.602189e-19;let c7=4.0;let f0=ctx.node_voltage(n[3]);let f2=((sf[283]+f0)-b8);let f4=(if (f2<sf[74]){d}else{b});let f7=(((f2-sf[73])-d)).exp();let f9=(if ((f4)!=0.0){(sf[73]+f7)}else{f2});let fd=((((if (f9>sf[76]){d}else{b}))!=0.0)&&(!((f4)!=0.0)));let fg=(((sf[75]-f9)-d)).exp();let fj=(b8+(if fd{(sf[75]-fg)}else{f9}));let fl=((bA*fj)/bC);let fm=(fj/sf[71]);let fn_=(fj-sf[71]);let fq=(sf[48]*f64::powf(fm,sf[124]));let gj=(sf[77]*f64::powf(fm,sf[82]));let gk=(d-fm);let gl=(sf[84]*gk);let gm=(sf[81]*fl);let go=((gl/gm)).exp();let gp=(gj*go);let gr=(sf[90]*f64::powf(fm,sf[93]));let gs=(sf[95]*gk);let gt=(sf[92]*fl);let gv=((gs/gt)).exp();let gw=(gr*gv);let gy=(sf[20]*f64::powf(fm,sf[99]));let gz=(sf[101]*gk);let gA=(sf[98]*fl);let gC=((gz/gA)).exp();let gD=(gy*gC);let hc=(d+(fn_*sf[141]));let hd=(sf[81]*hc);let he=(sf[92]*hc);let hA=2.0;let hC=(hA*(fl/fm));let hF=(fm*sf[150]);let hH=((hF/fl)).exp();let hI=-0.5;let hK=(fm*sf[151]);let hM=((hK/fl)).exp();let hN=(hH-hM);let hO=(hN).ln();let hP=(hC*hO);let hR=3.0;let hS=(fl*hR);let hT=(fm).ln();let hU=(hS*hT);let hW=(fm-d);let hY=(((fm*hP)-hU)-(sf[106]*hW));let hZ=(fl*hA);let i0=(-hY);let i2=((i0/fl)).exp();let i5=((d+(c7*i2))).sqrt();let i7=(M*(d+i5));let i8_=(i7).ln();let ia=(hY+(hZ*i8_));let id=(fm*sf[153]);let if_=((id/fl)).exp();let ih=(fm*sf[154]);let ij=((ih/fl)).exp();let ik=(if_-ij);let il=(ik).ln();let im=(hC*il);let iq=(((fm*im)-hU)-(sf[117]*hW));let ir=(-iq);let it=((ir/fl)).exp();let iw=((d+(c7*it))).sqrt();let iy=(M*(d+iw));let iz=(iy).ln();let iB=(iq+(hZ*iz));let iD=(sf[149]/ia);let iG=(sf[155]*f64::powf(iD,sf[156]));let iI=(sf[152]/iB);let iK=f64::powf(iI,sf[158]);let iL=(sf[157]*iK);let iN=(iK*sf[159]);let iQ=(sf[160]*f64::powf(fm,sf[80]));let iS=((gl/fl)).exp();let iT=(iQ*iS);let j6=(sf[163]*(d+(fn_*sf[164])));let jb=(sf[165]*(d+(fn_*sf[166])));let jz=(j6>b);let jB=(if jz{(d/j6)}else{b});let jC=(jb>b);let jE=(if jC{(d/jb)}else{b});let jF=(fq>b);let jH=(if jF{(d/fq)}else{b});let jL=ctx.node_voltage(n[7]);let jM=ctx.node_voltage(n[8]);let jO=(sf[53]*(jL-jM));let jP=ctx.node_voltage(n[6]);let jR=(sf[53]*(jP-jM));let jS=ctx.node_voltage(n[5]);let jU=(sf[53]*(jL-jS));let jV=ctx.node_voltage(n[4]);let jX=(sf[53]*(jL-jV));let k0=ctx.node_voltage(n[9]);let k2=(sf[53]*(jP-k0));let k3=ctx.node_voltage(n[1]);let k4=ctx.node_voltage(n[2]);let k8=ctx.node_voltage(n[0]);let kh=ctx.node_voltage(n[10]);let ki=ctx.node_voltage(n[11]);let kj=(-ia);let kl=(kj*sf[167]);let km=(jO+kl);let kn=(if ((sf[16])!=0.0){km}else{b});let kp=(if (kn>b){d}else{b});let kq=(((sf[16])!=0.0)&&((kp)!=0.0));let ku=(if kq{sf[170]}else{b});let kw=(d-(sf[168]*ku));let kC=(kn*sf[172]);let kD=(ia*sf[168]);let kF=(d+(kC/kD));let kK=(((sf[16])!=0.0)&&(!((kp)!=0.0)));let kM=(d-(jO/ia));let kO=(d-f64::powf(kM,sf[171]));let kR=(if kK{((ia*kO)/sf[171])}else{(if kq{((ia*kw)/sf[171])}else{b})});let l0=(((kl*kl)+sf[174])).sqrt();let l4=(if sb[42]{(hI*(kl+(if sb[42]{l0}else{b})))}else{b});let l6=(d-(l4/ia));let l7=f64::powf(l6,sf[171]);let la=(if sb[42]{((kj*l7)/sf[171])}else{b});let lb=(if sb[42]{km}else{b});let le=((sf[174]+(lb*lb))).sqrt();let lj=(if sb[42]{((M*(lb-(if sb[42]{le}else{b})))-kl)}else{b});let ll=(d-(lj/ia));let lm=f64::powf(ll,sf[171]);let lr=(l4+(jO-lj));let ls=(sf[170]*lr);let lt=(sf[172]*lr);let lv=(d+(lt/kD));let lz=(if sb[42]{(((if sb[42]{((kj*lm)/sf[171])}else{kR})+(ls*lv))-la)}else{(if ((sf[16])!=0.0){(kR+(if kK{b}else{(if kq{(ku*(kn*kF))}else{b})}))}else{b})});let lA=(-iB);let lB=(sf[167]*lA);let lC=(jU+lB);let lD=(if ((sf[1])!=0.0){lC}else{b});let lF=(if (lD>b){d}else{b});let lG=(((sf[1])!=0.0)&&((lF)!=0.0));let lJ=(if lG{sf[176]}else{b});let lM=(d-(sf[168]*(sf[168]*lJ)));let lS=(lD*sf[178]);let lU=(sf[168]+(lS/iB));let m1=(if (sb[6]&&(jU<sf[179])){d}else{b});let m3=(((sf[1])!=0.0)&&(!((lF)!=0.0)));let m4=(((m1)!=0.0)&&m3);let m6=(d+(sf[10]/iB));let m7=f64::powf(m6,sf[177]);let m9=(sf[177]*(sf[10]+jU));let ma=(sf[10]+iB);let mc=(d-(m9/ma));let me=(d-(m7*mc));let mj=(m3&&(!((m1)!=0.0)));let ml=(d-(jU/iB));
        let mn=(d-f64::powf(ml,sf[177]));let mq=(if mj{((iB*mn)/sf[177])}else{(if m4{((iB*me)/sf[177])}else{(if lG{((iB*lM)/sf[177])}else{b})})});let mw=(sf[10]+lB);let mx=(sf[10]-lB);let mz=(if sb[44]{(mw/mx)}else{b});let mA=(hA*mz);let mB=(mz-d);let mG=(((mB*mB)+sf[181])).sqrt();let mH=(d+mz);let mM=(((mH*mH)+sf[183])).sqrt();let mN=(mG+mM);let mP=(if sb[44]{(mA/mN)}else{b});let mU=(if sb[44]{(M*(((mx*mP)-sf[10])-lB))}else{b});let mW=(d-(mU/iB));let mY=(d-f64::powf(mW,sf[177]));let n1=(if sb[44]{((iB*mY)/sf[177])}else{b});let n4=(lB+(sf[10]+(hA*jU)));let n6=(if sb[44]{(n4/mx)}else{b});let n7=(hA*n6);let n8=(n6-d);let nb=((sf[181]+(n8*n8))).sqrt();let nc=(d+n6);let nf=((sf[183]+(nc*nc))).sqrt();let ng=(nb+nf);let ni=(if sb[44]{(n7/ng)}else{b});let nn=(if sb[44]{(M*(((mx*ni)-sf[10])-lB))}else{b});let np=(d-(nn/iB));let nr=(d-f64::powf(np,sf[177]));let nu=(if sb[44]{((iB*nr)/sf[177])}else{mq});let nx=(if sb[44]{(M*(d+ni))}else{b});let nA=(if sb[44]{f64::powf(m6,sf[184])}else{b});let nC=(d+(lB/iB));let nE=(if sb[44]{f64::powf(nC,sf[184])}else{b});let nF=(d-nx);let nJ=(if sb[44]{((nA*nF)+(nx*nE))}else{b});let nL=(mU+(jU-nn));let nV=((sf[181]+(lB*lB))).sqrt();let nZ=(if sb[46]{(hI*(lB+(if sb[46]{nV}else{b})))}else{mU});let o1=(d-(nZ/iB));let o2=f64::powf(o1,sf[177]);let o5=(if sb[46]{((lA*o2)/sf[177])}else{b});let o6=(if sb[46]{lC}else{b});let o9=((sf[181]+(o6*o6))).sqrt();let oe=(if sb[46]{((M*(o6-(if sb[46]{o9}else{b})))-lB)}else{nn});let og=(d-(oe/iB));let oh=f64::powf(og,sf[177]);let or=(if sb[46]{(((if sb[46]{((lA*oh)/sf[177])}else{nu})+(sf[185]*(nZ+(jU-oe))))-o5)}else{(if sb[44]{((nu+(if sb[44]{(nJ*nL)}else{b}))-n1)}else{(if ((sf[1])!=0.0){(mq+(if m3{b}else{(if lG{(lJ*(lD*lU))}else{b})}))}else{b})})});let os=(fl*hd);let ot=(d/os);let ox=((jO*ot)).exp();let oI=(fl*he);let oJ=(d/oI);let oN=((jU*oJ)).exp();let oW=(gp*gw);let p3=0.0001;let p4=(((d+(jE*lz))+(jB*or))-p3);let p6=1e-8;let p8=(((p4*p4)+p6)).sqrt();let pb=(p3+(M*(p4+p8)));let pg_=f64::powf(pb,sf[186]);let pv=(M*(pb+sf[187]));let pD=(M*pb);let pL=(pD*sf[188]);let pQ=(if ((sf[21])!=0.0){(d/gA)}else{oJ});let pU=((k2*pQ)).exp();let qc=((jU*pQ)).exp();let yl=(jU/fl);let yo=(yl).exp();let yp=(!(((if (yl<sf[55]){d}else{b}))!=0.0));let yu=(if yp{(sf[194]*(d+(yl-sf[55])))}else{yo});let yv=(jX/fl);let yy=(yv).exp();let yz=(!(((if (yv<sf[55]){d}else{b}))!=0.0));let yD=(if yz{(sf[194]*(d+(yv-sf[55])))}else{yy});let yG=((d+(iT*yu))).sqrt();let yJ=((d+(iT*yD))).sqrt();let BJ=(jR+kl);let BK=(if ((sf[16])!=0.0){BJ}else{b});let BM=(if (BK>b){d}else{b});let BN=(((sf[16])!=0.0)&&((BM)!=0.0));let BO=(if BN{sf[170]}else{b});let BQ=(d-(sf[168]*BO));let BU=(sf[172]*BK);let BW=(d+(BU/kD));let C1=(((sf[16])!=0.0)&&(!((BM)!=0.0)));let C3=(d-(jR/ia));let C5=(d-f64::powf(C3,sf[171]));let C8=(if C1{((ia*C5)/sf[171])}else{(if BN{((ia*BQ)/sf[171])}else{b})});let Cc=(if sb[42]{BJ}else{b});let Cf=((sf[174]+(Cc*Cc))).sqrt();let Ck=(if sb[42]{((M*(Cc-(if sb[42]{Cf}else{b})))-kl)}else{b});let Cm=(d-(Ck/ia));let Cn=f64::powf(Cm,sf[171]);let Cs=(l4+(jR-Ck));let Ct=(sf[170]*Cs);let Cu=(sf[172]*Cs);let Cw=(d+(Cu/kD));let CA=(if sb[42]{(((if sb[42]{((kj*Cn)/sf[171])}else{C8})+(Ct*Cw))-la)}else{(if ((sf[16])!=0.0){(C8+(if C1{b}else{(if BN{(BO*(BK*BW))}else{b})}))}else{b})});let CB=(k2+lB);let CC=(if ((sf[1])!=0.0){CB}else{b});let CE=(if (CC>b){d}else{b});let CF=(((sf[1])!=0.0)&&((CE)!=0.0));let CG=(if CF{sf[176]}else{b});let CJ=(d-(sf[168]*(sf[168]*CG)));let CN=(sf[178]*CC);let CP=(sf[168]+(CN/iB));let CV=(if (sb[6]&&(k2<sf[179])){d}else{b});let CX=(((sf[1])!=0.0)&&(!((CE)!=0.0)));let CY=(((CV)!=0.0)&&CX);let D0=(sf[177]*(sf[10]+k2));let D2=(d-(D0/ma));let D4=(d-(m7*D2));let D9=(CX&&(!((CV)!=0.0)));let Db=(d-(k2/iB));let Dd=(d-f64::powf(Db,sf[177]));let Dg=(if D9{((iB*Dd)/sf[177])}else{(if CY{((iB*D4)/sf[177])}else{(if CF{((iB*CJ)/sf[177])}else{b})})});let Dm=(lB+(sf[10]+(hA*k2)));let Do=(if sb[44]{(Dm/mx)}else{b});let Dp=(hA*Do);let Dq=(Do-d);let Dt=((sf[181]+(Dq*Dq))).sqrt();let Du=(d+Do);let Dx=((sf[183]+(Du*Du))).sqrt();let Dy=(Dt+Dx);let DA=(if sb[44]{(Dp/Dy)}else{b});
        let DF=(if sb[44]{(M*(((mx*DA)-sf[10])-lB))}else{b});let DH=(d-(DF/iB));let DJ=(d-f64::powf(DH,sf[177]));let DM=(if sb[44]{((iB*DJ)/sf[177])}else{Dg});let DP=(if sb[44]{(M*(d+DA))}else{b});let DQ=(d-DP);let DU=(if sb[44]{((nA*DQ)+(nE*DP))}else{b});let DW=(mU+(k2-DF));let E2=(if sb[46]{CB}else{b});let E5=((sf[181]+(E2*E2))).sqrt();let Ea=(if sb[46]{((M*(E2-(if sb[46]{E5}else{b})))-lB)}else{DF});let Ec=(d-(Ea/iB));let Ed=f64::powf(Ec,sf[177]);let Em=(if sb[46]{(((if sb[46]{((lA*Ed)/sf[177])}else{DM})+(sf[185]*(nZ+(k2-Ea))))-o5)}else{(if sb[44]{((DM+(if sb[44]{(DU*DW)}else{b}))-n1)}else{(if ((sf[1])!=0.0){(Dg+(if CX{b}else{(if CF{(CG*(CC*CP))}else{b})}))}else{b})})});let Ev=((sf[65]*jU)/1.44);let Ex=(if (Ev<sf[55]){d}else{b});let Ey=(Ev).exp();let EA=(!((Ex)!=0.0));let EJ=(sf[205]*(d+(pb*sf[206])));let EL=((if EA{(sf[194]*(d+(Ev-sf[55])))}else{(if ((Ex)!=0.0){Ey}else{yu})})*sf[207]);let ET=(sf[13]*(iG*lz));let EZ=(iL*or);let F4=(yG*sf[209]);let F7=(iN*Em);let Fb=((k3-k4)*sf[210]);let Fd=((k3-k8)*sf[211]);let Ff=(f0*sf[212]);let Fh=(kh*sf[213]);let Fk=((ki*sf[213])*0.3333333333333333);let Fm=(sf[53]*(sf[192]*(iG*CA)));let Fo=(sf[53]*(yJ*sf[209]));let Fq=(if ((f4)!=0.0){f7}else{d});let Fu=(if fd{(-(fg*(-Fq)))}else{Fq});let Fw=((bA*Fu)/bC);let Fx=(Fu/sf[71]);let GB=(-Fx);let GC=(sf[84]*GB);let GM=((go*(sf[77]*(Fx*(sf[82]*f64::powf(fm,sf[223])))))+(gj*(go*(((gm*GC)-(gl*(sf[81]*Fw)))/(gm*gm)))));let H9=(sf[98]*Fw);let Hd=(gA*gA);let Hi=((gC*(sf[20]*(Fx*(sf[99]*f64::powf(fm,sf[225])))))+(gy*(gC*(((gA*(sf[101]*GB))-(gz*H9))/Hd))));let It=(sf[141]*Fu);let IM=(hA*(((fm*Fw)-(fl*Fx))/(fm*fm)));let IR=(fl*fl);let Jc=((hT*(hR*Fw))+(hS*(Fx/fm)));let Jf=((((hP*Fx)+(fm*((hO*IM)+(hC*(((hH*(((fl*(sf[150]*Fx))-(hF*Fw))/IR))-(hM*(((fl*(sf[151]*Fx))-(hK*Fw))/IR)))/hN)))))-Jc)-(sf[106]*Fx));let Jg=(hA*Fw);let Jv=(Jf+((i8_*Jg)+(hZ*((M*((c7*(i2*(((fl*(-Jf))-(i0*Fw))/IR)))/(hA*i5)))/i7))));let JS=((((im*Fx)+(fm*((il*IM)+(hC*(((if_*(((fl*(sf[153]*Fx))-(id*Fw))/IR))-(ij*(((fl*(sf[154]*Fx))-(ih*Fw))/IR)))/ik)))))-Jc)-(sf[117]*Fx));let K7=(JS+((iz*Jg)+(hZ*((M*((c7*(it*(((fl*(-JS))-(ir*Fw))/IR)))/(hA*iw)))/iy))));let Ka=(ia*ia);let Kg=(sf[155]*(((-(sf[149]*Jv))/Ka)*(sf[156]*f64::powf(iD,sf[230]))));let Kj=(iB*iB);let Kn=(((-(sf[152]*K7))/Kj)*(sf[158]*f64::powf(iI,sf[197])));let KC=((iS*(sf[160]*(Fx*(sf[80]*f64::powf(fm,sf[231])))))+(iQ*(iS*(((fl*GC)-(gl*Fw))/IR))));let Lz=(if jF{((-(sf[48]*(Fx*(sf[124]*f64::powf(fm,sf[214])))))/(fq*fq))}else{b});let LF=(-Jv);let LG=(sf[167]*LF);let LH=(if ((sf[16])!=0.0){LG}else{b});let LQ=(sf[168]*Jv);let LR=(kD*(sf[172]*LH));let LU=(kD*kD);let LW=(sf[236]/kD);let LX=(sf[237]/kD);let Mj=(-(sf[53]/ia));let Mk=(-(sf[233]/ia));let Mn=(sf[171]*f64::powf(kM,sf[238]));let MC=(if kK{(((kO*Jv)+(ia*(-((-((-(jO*Jv))/Ka))*Mn))))/sf[171])}else{(if kq{((kw*Jv)/sf[171])}else{b})});let MD=(if kK{((ia*(-(Mj*Mn)))/sf[171])}else{b});let ME=(if kK{((ia*(-(Mk*Mn)))/sf[171])}else{b});let MO=(kl*LG);let MV=(if sb[42]{(hI*(LG+(if sb[42]{((MO+MO)/(hA*l0))}else{b})))}else{b});let N8=(if sb[42]{(((l7*LF)+(kj*((-(((ia*MV)-(l4*Jv))/Ka))*(sf[171]*f64::powf(l6,sf[238])))))/sf[171])}else{b});let N9=(if sb[42]{LG}else{b});let Nc=(lb*N9);let Ne=(lb*sf[239]);let Ng=(lb*sf[240]);let Ni=(hA*le);let Nw=(if sb[42]{((M*(N9-(if sb[42]{((Nc+Nc)/Ni)}else{b})))-LG)}else{b});let Nx=(if sb[42]{(M*(sf[239]-(if sb[42]{((Ne+Ne)/Ni)}else{b})))}else{b});let Ny=(if sb[42]{(M*(sf[240]-(if sb[42]{((Ng+Ng)/Ni)}else{b})))}else{b});let NJ=(sf[171]*f64::powf(ll,sf[238]));let NZ=(sf[53]-Nx);let O0=(sf[233]-Ny);let O1=(MV+(-Nw));let Or=(if sb[42]{(((if sb[42]{(((lm*LF)+(kj*((-(((ia*Nw)-(lj*Jv))/Ka))*NJ)))/sf[171])}else{MC})+((lv*(sf[170]*O1))+(ls*(((kD*(sf[172]*O1))-(lt*LQ))/LU))))-N8)}else{(if ((sf[16])!=0.0){(MC+(if kK{b}else{(if kq{(ku*((kF*LH)+(kn*((LR-(kC*LQ))/LU))))}else{b})}))}else{b})});let Os=(if sb[42]{((if sb[42]{((kj*((-(Nx/ia))*NJ))/sf[171])}else{MD})+((lv*(sf[170]*NZ))+(ls*((sf[172]*NZ)/kD))))}else{(if ((sf[16])!=0.0){(MD+(if kK{b}else{(if kq{(ku*((kF*sf[234])+(kn*LW)))}else{b})}))}else{b})});
        let Ot=(if sb[42]{((if sb[42]{((kj*((-(Ny/ia))*NJ))/sf[171])}else{ME})+((lv*(sf[170]*O0))+(ls*((sf[172]*O0)/kD))))}else{(if ((sf[16])!=0.0){(ME+(if kK{b}else{(if kq{(ku*((kF*sf[235])+(kn*LX)))}else{b})}))}else{b})});let Ou=(-K7);let Ov=(sf[167]*Ou);let Ow=(if ((sf[1])!=0.0){Ov}else{b});let OF=(iB*(sf[178]*Ow));let OJ=(sf[243]/iB);let OK=(sf[244]/iB);let P2=((-(sf[10]*K7))/Kj);let P6=(P2*(sf[177]*f64::powf(m6,sf[245])));let Pb=(ma*ma);let Pw=((iB*(-(m7*(-(sf[246]/ma)))))/sf[177]);let Px=((iB*(-(m7*(-(sf[247]/ma)))))/sf[177]);let PH=(-(sf[233]/iB));let PI=(-(sf[53]/iB));let PK=(sf[177]*f64::powf(ml,sf[245]));let PZ=(if mj{(((mn*K7)+(iB*(-((-((-(jU*K7))/Kj))*PK))))/sf[177])}else{(if m4{(((me*K7)+(iB*(-((mc*P6)+(m7*(-((-(m9*K7))/Pb)))))))/sf[177])}else{(if lG{((lM*K7)/sf[177])}else{b})})});let Q0=(if mj{((iB*(-(PH*PK)))/sf[177])}else{(if m4{Pw}else{b})});let Q1=(if mj{((iB*(-(PI*PK)))/sf[177])}else{(if m4{Px}else{b})});let Qb=(-Ov);let Qc=(mx*Ov);let Qf=(mx*mx);let Qh=(if sb[44]{((Qc-(mw*Qb))/Qf)}else{b});let Qj=(mB*Qh);let Qn=(mH*Qh);let QD=(if sb[44]{(M*(((mP*Qb)+(mx*(if sb[44]{(((mN*(hA*Qh))-(mA*(((Qj+Qj)/(hA*mG))+((Qn+Qn)/(hA*mM)))))/(mN*mN))}else{b})))-Ov))}else{b});let QR=(if sb[44]{(((mY*K7)+(iB*(-((-(((iB*QD)-(mU*K7))/Kj))*(sf[177]*f64::powf(mW,sf[245]))))))/sf[177])}else{b});let QZ=(if sb[44]{((Qc-(n4*Qb))/Qf)}else{b});let R0=(if sb[44]{(sf[248]/mx)}else{b});let R1=(if sb[44]{(sf[249]/mx)}else{b});let R3=(hA*R0);let R4=(hA*R1);let R5=(n8*QZ);let R7=(n8*R0);let R9=(n8*R1);let Rb=(hA*nb);let Rf=(nc*QZ);let Rh=(nc*R0);let Rj=(nc*R1);let Rl=(hA*nf);let Rv=(ng*ng);let RF=(if sb[44]{(((ng*(hA*QZ))-(n7*(((R5+R5)/Rb)+((Rf+Rf)/Rl))))/Rv)}else{b});let RG=(if sb[44]{(((ng*R3)-(n7*(((R7+R7)/Rb)+((Rh+Rh)/Rl))))/Rv)}else{b});let RH=(if sb[44]{(((ng*R4)-(n7*(((R9+R9)/Rb)+((Rj+Rj)/Rl))))/Rv)}else{b});let RR=(if sb[44]{(M*(((ni*Qb)+(mx*RF))-Ov))}else{b});let RS=(if sb[44]{(M*(mx*RG))}else{b});let RT=(if sb[44]{(M*(mx*RH))}else{b});let S4=(sf[177]*f64::powf(np,sf[245]));let Sj=(if sb[44]{(((nr*K7)+(iB*(-((-(((iB*RR)-(nn*K7))/Kj))*S4))))/sf[177])}else{PZ});let Sk=(if sb[44]{((iB*(-((-(RS/iB))*S4)))/sf[177])}else{Q0});let Sl=(if sb[44]{((iB*(-((-(RT/iB))*S4)))/sf[177])}else{Q1});let Sp=(if sb[44]{(M*RF)}else{b});let Sq=(if sb[44]{(M*RG)}else{b});let Sr=(if sb[44]{(M*RH)}else{b});let Sw=(if sb[44]{(P2*(sf[184]*f64::powf(m6,sf[250])))}else{b});let SE=(if sb[44]{((((iB*Ov)-(lB*K7))/Kj)*(sf[184]*f64::powf(nC,sf[250])))}else{b});let Tl=(lB*Ov);let Ts=(if sb[46]{(hI*(Ov+(if sb[46]{((Tl+Tl)/(hA*nV))}else{b})))}else{QD});let TF=(if sb[46]{(((o2*Ou)+(lA*((-(((iB*Ts)-(nZ*K7))/Kj))*(sf[177]*f64::powf(o1,sf[245])))))/sf[177])}else{b});let TG=(if sb[46]{Ov}else{b});let TJ=(o6*TG);let TL=(o6*sf[251]);let TN=(o6*sf[252]);let TP=(hA*o9);let U3=(if sb[46]{((M*(TG-(if sb[46]{((TJ+TJ)/TP)}else{b})))-Ov)}else{RR});let U4=(if sb[46]{(M*(sf[251]-(if sb[46]{((TL+TL)/TP)}else{b})))}else{RS});let U5=(if sb[46]{(M*(sf[252]-(if sb[46]{((TN+TN)/TP)}else{b})))}else{RT});let Ug=(sf[177]*f64::powf(og,sf[245]));let UG=(if sb[46]{(((if sb[46]{(((oh*Ou)+(lA*((-(((iB*U3)-(oe*K7))/Kj))*Ug)))/sf[177])}else{Sj})+(sf[185]*(Ts+(-U3))))-TF)}else{(if sb[44]{((Sj+(if sb[44]{((nL*(if sb[44]{(((nF*Sw)+(nA*(-Sp)))+((nE*Sp)+(nx*SE)))}else{b}))+(nJ*(QD+(-RR))))}else{b}))-QR)}else{(if ((sf[1])!=0.0){(PZ+(if m3{b}else{(if lG{(lJ*((lU*Ow)+(lD*((OF-(lS*K7))/Kj))))}else{b})}))}else{b})})});let UH=(if sb[46]{((if sb[46]{((lA*((-(U4/iB))*Ug))/sf[177])}else{Sk})+(sf[185]*(sf[233]-U4)))}else{(if sb[44]{(Sk+(if sb[44]{((nL*(if sb[44]{((nA*(-Sq))+(nE*Sq))}else{b}))+(nJ*(sf[233]-RS)))}else{b}))}else{(if ((sf[1])!=0.0){(Q0+(if m3{b}else{(if lG{(lJ*((lU*sf[241])+(lD*OJ)))}else{b})}))}else{b})})});let UI=(if sb[46]{((if sb[46]{((lA*((-(U5/iB))*Ug))/sf[177])}else{Sl})+(sf[185]*(sf[53]-U5)))}else{(if sb[44]{(Sl+(if sb[44]{((nL*(if sb[44]{((nA*(-Sr))+(nE*Sr))}else{b}))+(nJ*(sf[53]-RT)))}else{b}))}else{(if ((sf[1])!=0.0){(Q1+(if m3{b}else{(if lG{(lJ*((lU*sf[242])+(lD*OK)))}else{b})}))}else{b})})});let UO=((-((hd*Fw)+(fl*(sf[81]*It))))/(os*os));let UQ=(sf[53]*ot);let UR=(ot*sf[233]);
        let US=(ox*(jO*UO));let UT=(ox*UQ);let UU=(ox*UR);let Vg=((-((he*Fw)+(fl*(sf[92]*It))))/(oI*oI));let Vi=(oJ*sf[233]);let Vj=(sf[53]*oJ);let Vk=(oN*(jU*Vg));let Vl=(oN*Vi);let Vm=(oN*Vj);let VA=((gw*GM)+(gp*((gv*(sf[90]*(Fx*(sf[93]*f64::powf(fm,sf[224])))))+(gr*(gv*(((gt*(sf[95]*GB))-(gs*(sf[92]*Fw)))/(gt*gt)))))));let VK=(jE*Ot);let VO=(jB*UH);let VQ=(((lz*(if jC{((-(sf[165]*(sf[166]*Fu)))/(jb*jb))}else{b}))+(jE*Or))+((or*(if jz{((-(sf[163]*(sf[164]*Fu)))/(j6*j6))}else{b}))+(jB*UG)));let VR=((jE*Os)+(jB*UI));let VS=(p4*VQ);let VU=(p4*VO);let VW=(p4*VR);let VY=(p4*VK);let W0=(hA*p8);let W9=(M*(VQ+((VS+VS)/W0)));let Wa=(M*(VO+((VU+VU)/W0)));let Wb=(M*(VR+((VW+VW)/W0)));let Wc=(M*(VK+((VY+VY)/W0)));let Wp=(sf[186]*f64::powf(pb,sf[253]));let Wq=(W9*Wp);let Wr=(Wa*Wp);let Ws=(Wb*Wp);let Wt=(Wc*Wp);let WV=(M*W9);let WW=(M*Wa);let WX=(M*Wb);let WY=(M*Wc);let Xp=(sf[188]*WV);let Xq=(sf[188]*WW);let Xr=(sf[188]*WX);let Xs=(sf[188]*WY);let Y4=(if ((sf[21])!=0.0){((-H9)/Hd)}else{Vg});let Y6=(sf[53]*pQ);let Y7=(pQ*sf[233]);let Y8=(pU*(k2*Y4));let Y9=(pU*Y6);let Ya=(pU*Y7);let Yx=(qc*(jU*Y4));let Yy=(qc*Y7);let Yz=(qc*Y6);let aeP=((-(jU*Fw))/IR);let aeQ=(sf[233]/fl);let aeR=(sf[53]/fl);let aeW=(sf[194]*aeQ);let aeX=(sf[194]*aeR);let aeY=(if yp{(sf[194]*aeP)}else{(yo*aeP)});let aeZ=(if yp{aeW}else{(yo*aeQ)});let af0=(if yp{aeX}else{(yo*aeR)});let af3=((-(jX*Fw))/IR);let afg=(hA*yG);let afh=(((yu*KC)+(iT*aeY))/afg);let afi=((iT*aeZ)/afg);let afj=((iT*af0)/afg);let afp=(hA*yJ);let afq=(((yD*KC)+(iT*(if yz{(sf[194]*af3)}else{(yy*af3)})))/afp);let afr=((iT*(if yz{aeW}else{(yy*aeQ)}))/afp);let afs=((iT*(if yz{aeX}else{(yy*aeR)}))/afp);let apc=(sf[171]*f64::powf(C3,sf[238]));let apr=(if C1{(((C5*Jv)+(ia*(-((-((-(jR*Jv))/Ka))*apc))))/sf[171])}else{(if BN{((BQ*Jv)/sf[171])}else{b})});let aps=(if C1{((ia*(-(Mj*apc)))/sf[171])}else{b});let apt=(if C1{((ia*(-(Mk*apc)))/sf[171])}else{b});let apD=(Cc*N9);let apF=(Cc*sf[239]);let apH=(Cc*sf[240]);let apJ=(hA*Cf);let apX=(if sb[42]{((M*(N9-(if sb[42]{((apD+apD)/apJ)}else{b})))-LG)}else{b});let apY=(if sb[42]{(M*(sf[239]-(if sb[42]{((apF+apF)/apJ)}else{b})))}else{b});let apZ=(if sb[42]{(M*(sf[240]-(if sb[42]{((apH+apH)/apJ)}else{b})))}else{b});let aqa=(sf[171]*f64::powf(Cm,sf[238]));let aqq=(sf[53]-apY);let aqr=(sf[233]-apZ);let aqs=(MV+(-apX));let arA=(sf[177]*f64::powf(Db,sf[245]));let arP=(if D9{(((Dd*K7)+(iB*(-((-((-(k2*K7))/Kj))*arA))))/sf[177])}else{(if CY{(((D4*K7)+(iB*(-((D2*P6)+(m7*(-((-(D0*K7))/Pb)))))))/sf[177])}else{(if CF{((CJ*K7)/sf[177])}else{b})})});let arQ=(if D9{((iB*(-(PI*arA)))/sf[177])}else{(if CY{Px}else{b})});let arR=(if D9{((iB*(-(PH*arA)))/sf[177])}else{(if CY{Pw}else{b})});let as4=(if sb[44]{((Qc-(Dm*Qb))/Qf)}else{b});let as6=(Dq*as4);let as8=(Dq*R1);let asa=(Dq*R0);let asc=(hA*Dt);let asg=(Du*as4);let asi=(Du*R1);let ask=(Du*R0);let asm=(hA*Dx);let asw=(Dy*Dy);let asG=(if sb[44]{(((Dy*(hA*as4))-(Dp*(((as6+as6)/asc)+((asg+asg)/asm))))/asw)}else{b});let asH=(if sb[44]{(((Dy*R4)-(Dp*(((as8+as8)/asc)+((asi+asi)/asm))))/asw)}else{b});let asI=(if sb[44]{(((Dy*R3)-(Dp*(((asa+asa)/asc)+((ask+ask)/asm))))/asw)}else{b});let asS=(if sb[44]{(M*(((DA*Qb)+(mx*asG))-Ov))}else{b});let asT=(if sb[44]{(M*(mx*asH))}else{b});let asU=(if sb[44]{(M*(mx*asI))}else{b});let at5=(sf[177]*f64::powf(DH,sf[245]));let atk=(if sb[44]{(((DJ*K7)+(iB*(-((-(((iB*asS)-(DF*K7))/Kj))*at5))))/sf[177])}else{arP});let atl=(if sb[44]{((iB*(-((-(asT/iB))*at5)))/sf[177])}else{arQ});let atm=(if sb[44]{((iB*(-((-(asU/iB))*at5)))/sf[177])}else{arR});let atq=(if sb[44]{(M*asG)}else{b});let atr=(if sb[44]{(M*asH)}else{b});let ats=(if sb[44]{(M*asI)}else{b});let au9=(E2*TG);let aub=(E2*sf[252]);let aud=(E2*sf[251]);let auf=(hA*E5);let aut=(if sb[46]{((M*(TG-(if sb[46]{((au9+au9)/auf)}else{b})))-Ov)}else{asS});let auu=(if sb[46]{(M*(sf[252]-(if sb[46]{((aub+aub)/auf)}else{b})))}else{asT});let auv=(if sb[46]{(M*(sf[251]-(if sb[46]{((aud+aud)/auf)}else{b})))}else{asU});let auG=(sf[177]*f64::powf(Ec,sf[245]));let avK=(sf[205]*(sf[206]*W9));let avL=(sf[205]*(sf[206]*Wa));let avM=(sf[205]*(sf[206]*Wb));
        let avN=(sf[205]*(sf[206]*Wc));let avO=(sf[207]*(if EA{b}else{(if ((Ex)!=0.0){b}else{aeY})}));let avP=(sf[207]*(if EA{sf[278]}else{(if ((Ex)!=0.0){(Ey*sf[276])}else{aeZ})}));let avQ=(sf[207]*(if EA{sf[279]}else{(if ((Ex)!=0.0){(Ey*sf[277])}else{af0})}));let awq=(sf[13]*((lz*Kg)+(iG*Or)));let awr=(sf[13]*(iG*Os));let aws=(sf[13]*(iG*Ot));let ax6=((or*(sf[157]*Kn))+(iL*UG));let ax7=(iL*UH);let ax8=(iL*UI);let axf=(sf[209]*afh);let axg=(sf[209]*afi);let axh=(sf[209]*afj);let axq=((Em*(sf[159]*Kn))+(iN*(if sb[46]{(((if sb[46]{(((Ed*Ou)+(lA*((-(((iB*aut)-(Ea*K7))/Kj))*auG)))/sf[177])}else{atk})+(sf[185]*(Ts+(-aut))))-TF)}else{(if sb[44]{((atk+(if sb[44]{((DW*(if sb[44]{(((DQ*Sw)+(nA*(-atq)))+((DP*SE)+(nE*atq)))}else{b}))+(DU*(QD+(-asS))))}else{b}))-QR)}else{(if ((sf[1])!=0.0){(arP+(if CX{b}else{(if CF{(CG*((CP*Ow)+(CC*((OF-(CN*K7))/Kj))))}else{b})}))}else{b})})})));let axr=(iN*(if sb[46]{((if sb[46]{((lA*((-(auu/iB))*auG))/sf[177])}else{atl})+(sf[185]*(sf[53]-auu)))}else{(if sb[44]{(atl+(if sb[44]{((DW*(if sb[44]{((nA*(-atr))+(nE*atr))}else{b}))+(DU*(sf[53]-asT)))}else{b}))}else{(if ((sf[1])!=0.0){(arQ+(if CX{b}else{(if CF{(CG*((CP*sf[242])+(CC*OK)))}else{b})}))}else{b})})}));let axs=(iN*(if sb[46]{((if sb[46]{((lA*((-(auv/iB))*auG))/sf[177])}else{atm})+(sf[185]*(sf[233]-auv)))}else{(if sb[44]{(atm+(if sb[44]{((DW*(if sb[44]{((nA*(-ats))+(nE*ats))}else{b}))+(DU*(sf[233]-asU)))}else{b}))}else{(if ((sf[1])!=0.0){(arR+(if CX{b}else{(if CF{(CG*((CP*sf[241])+(CC*OJ)))}else{b})}))}else{b})})}));let axI=(sf[53]*(sf[192]*((CA*Kg)+(iG*(if sb[42]{(((if sb[42]{(((Cn*LF)+(kj*((-(((ia*apX)-(Ck*Jv))/Ka))*aqa)))/sf[171])}else{apr})+((Cw*(sf[170]*aqs))+(Ct*(((kD*(sf[172]*aqs))-(Cu*LQ))/LU))))-N8)}else{(if ((sf[16])!=0.0){(apr+(if C1{b}else{(if BN{(BO*((BW*LH)+(BK*((LR-(BU*LQ))/LU))))}else{b})}))}else{b})})))));let axJ=(sf[53]*(sf[192]*(iG*(if sb[42]{((if sb[42]{((kj*((-(apY/ia))*aqa))/sf[171])}else{aps})+((Cw*(sf[170]*aqq))+(Ct*((sf[172]*aqq)/kD))))}else{(if ((sf[16])!=0.0){(aps+(if C1{b}else{(if BN{(BO*((BW*sf[234])+(BK*LW)))}else{b})}))}else{b})}))));let axK=(sf[53]*(sf[192]*(iG*(if sb[42]{((if sb[42]{((kj*((-(apZ/ia))*aqa))/sf[171])}else{apt})+((Cw*(sf[170]*aqr))+(Ct*((sf[172]*aqr)/kD))))}else{(if ((sf[16])!=0.0){(apt+(if C1{b}else{(if BN{(BO*((BW*sf[235])+(BK*LX)))}else{b})}))}else{b})}))));let axO=(sf[53]*(sf[209]*afq));let axP=(sf[53]*(sf[209]*afr));let axQ=(sf[53]*(sf[209]*afs));

        CommonStampValues {
            b, d, M, c7, f0, fl, fm, fn_,
            gk, gp, gD, hA, iB, jH, jL, jM,
            jO, jP, jR, jS, jU, jV, k0, k2,
            k3, k4, k8, kh, ki, ot, ox, oJ,
            oN, oW, p3, p6, pb, pg_, pv, pD,
            pL, pQ, pU, qc, yG, yJ, EJ, EL,
            ET, EZ, F4, F7, Fb, Fd, Ff, Fh,
            Fk, Fm, Fo, Fu, Fw, Fx, GB, GM,
            Hi, K7, Lz, UO, UQ, UR, US, UT,
            UU, Vg, Vi, Vj, Vk, Vl, Vm, VA,
            W9, Wa, Wb, Wc, Wq, Wr, Ws, Wt,
            WV, WW, WX, WY, Xp, Xq, Xr, Xs,
            Y4, Y6, Y7, Y8, Y9, Ya, Yx, Yy,
            Yz, afh, afi, afj, afq, afr, afs, avK,
            avL, avM, avN, avO, avP, avQ, awq, awr,
            aws, ax6, ax7, ax8, axf, axg, axh, axq,
            axr, axs, axI, axJ, axK, axO, axP, axQ,
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let CommonStampValues {
            b, d, M, c7, f0, fl, fm, fn_,
            gk, gp, gD, hA, iB, jH, jL, jM,
            jO, jP, jR, jS, jU, jV, k0, k2,
            k3, k4, k8, kh, ki, ot, ox, oJ,
            oN, oW, p3, p6, pb, pg_, pv, pD,
            pL, pQ, pU, qc, yG, yJ, EJ, EL,
            ET, EZ, F4, F7, Fb, Fd, Ff, Fh,
            Fk, Fm, Fo, Fu, Fw, Fx, GB, GM,
            Hi, K7, Lz, UO, UQ, UR, US, UT,
            UU, Vg, Vi, Vj, Vk, Vl, Vm, VA,
            W9, Wa, Wb, Wc, Wq, Wr, Ws, Wt,
            WV, WW, WX, WY, Xp, Xq, Xr, Xs,
            Y4, Y6, Y7, Y8, Y9, Ya, Yx, Yy,
            Yz, afh, afi, afj, afq, afr, afs, avK,
            avL, avM, avN, avO, avP, avQ, awq, awr,
            aws, ax6, ax7, ax8, axf, axg, axh, axq,
            axr, axs, axI, axJ, axK, axO, axP, axQ,
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
        let ae=0.01;let ai=ctx.simparam_or("gmin", 1e-12);let ak=(if sb[21]{ai}else{sf[43]});let an=ctx.simparam_or("pnjmaxi", d);let ap=(if sb[22]{an}else{sf[45]});let bQ=(if sb[36]{b}else{(sf[301]*((sf[303]+(ap/sf[30]))).ln())});let c6=(M*ap);let cp=(if sb[79]{b}else{(if (sb[25]&&(ap>sf[48])){(sf[308]*((d+(f64::powf((c6*sf[87]),sf[89])/sf[311]))).ln())}else{(sf[308]*((d+(ap/sf[311]))).ln())})});let cV=(if sb[82]{b}else{(if (sb[24]&&(ap>sf[47])){(sf[316]*((d+(f64::powf((c6*sf[97]),sf[89])/sf[321]))).ln())}else{(sf[316]*((d+(ap/sf[321]))).ln())})});let dl=(if sb[84]{b}else{(if (sb[23]&&(ap>sf[46])){(sf[325]*((d+((sf[59]*(ap*ap))/sf[328]))).ln())}else{(sf[325]*((d+(ap/sf[328]))).ln())})});let dE=(sf[333]*((d+(ap/sf[336]))).ln());let dG=(if sb[86]{b}else{dE});let dZ=(sf[341]*((d+(ap/sf[344]))).ln());let e1=(if sb[88]{b}else{dZ});let ej=(sf[349]*((d+(ap/sf[352]))).ln());let el=(if sb[90]{b}else{ej});let eD=(sf[357]*((d+(ap/sf[360]))).ln());let eF=(if sb[92]{b}else{eD});let eN=(sf[349]*((d+(ap/sf[363]))).ln());let eX=(sf[357]*((d+(ap/sf[366]))).ln());let fy=f64::powf(fm,sf[127]);let fA=(if sb[37]{(sf[125]*fy)}else{(if ((sf[22])!=0.0){(sf[125]*f64::powf(fm,sf[126]))}else{b})});let fI=(if sb[38]{(fy*sf[128])}else{(if ((sf[39])!=0.0){(sf[128]*f64::powf(fm,sf[129]))}else{b})});let fQ=f64::powf(fm,sf[132]);let fS=(if sb[39]{(sf[130]*fQ)}else{(if ((sf[18])!=0.0){(sf[130]*f64::powf(fm,sf[131]))}else{b})});let g0=(if sb[40]{(fQ*sf[133])}else{(if ((sf[26])!=0.0){(sf[133]*f64::powf(fm,sf[134]))}else{b})});let g4=(sf[135]*f64::powf(fm,sf[136]));let gc=(if sb[41]{(fy*sf[137])}else{(if ((sf[17])!=0.0){(sf[137]*f64::powf(fm,sf[138]))}else{b})});let gh=(sf[139]*(d+(fn_*sf[140])));let gF=(sf[102]*f64::powf(fm,sf[105]));let gG=(sf[107]*gk);let gH=(sf[104]*fl);let gJ=((gG/gH)).exp();let gK=(gF*gJ);let gM=(sf[108]*f64::powf(fm,sf[111]));let gN=(sf[113]*gk);let gO=(sf[110]*fl);let gQ=((gN/gO)).exp();let gR=(gM*gQ);let gS=f64::powf(fm,sf[116]);let gT=(sf[114]*gS);let gU=(sf[118]*gk);let gV=(sf[115]*fl);let gX=((gU/gV)).exp();let gY=(gT*gX);let gZ=f64::powf(fm,sf[121]);let h0=(sf[119]*gZ);let h1=(sf[123]*gk);let h2=(sf[120]*fl);let h4=((h1/h2)).exp();let h5=(h0*h4);let h6=(sf[32]*gS);let h7=(gX*h6);let h8=(sf[33]*gZ);let h9=(h4*h8);let hj=(sf[142]*(d+(fn_*sf[143])));let ho=(sf[144]*(d+(fn_*sf[145])));let hs=(sf[146]+(fn_*sf[147]));let hz=(sf[78]*(d+(fn_*sf[148])));let iX=(sf[161]*f64::powf(fm,sf[162]));let iY=(-(sf[5]*(d+(fn_*hs))));let iZ=(fl*hz);let j1=((iY/iZ)).exp();let jc=0.001;let jd=(fA>jc);let jf=1000.0;let jg=(if jd{(d/fA)}else{jf});let jh=(fI>jc);let jj=(if jh{(d/fI)}else{jf});let jk=(fS>jc);let jm=(if jk{(d/fS)}else{jf});let jn=(g0>jc);let jp=(if jn{(d/g0)}else{jf});let jq=(g4>jc);let js=(if jq{(d/g4)}else{jf});let jt=(gc>jc);let jv=(if jt{(d/gc)}else{jf});let jw=(gh>jc);let jy=(if jw{(d/gh)}else{jf});let jI=(iX>b);let jK=(if jI{(d/iX)}else{b});let jZ=(sf[53]*(jP-jV));let k7=(sf[53]*(jS-jM));let ka=(k8-jV);let kc=(sf[53]*(jV-jS));let kd=(k3-jP);let ke=(jP-jL);let kf=(k4-jM);let kg=(k0-jV);let oy=(!(((if (jO<cp){d}else{b}))!=0.0));let oA=((cp*ot)).exp();let oB=(jO-cp);let oD=(d+(ot*oB));let oG=((if oy{(oA*oD)}else{ox})-d);let oH=(gp*oG);let oK=(jU<cV);let oO=(!(((if oK{d}else{b}))!=0.0));let oQ=((cV*oJ)).exp();let oR=(jU-cV);let oT=(d+(oJ*oR));let oU=(oQ*oT);let oX=((if oO{oU}else{oN})-d);let oY=(oW*oX);let ph=(c7*((jH*oH)+(sf[57]*oY)));let pi=(pg_+ph);let pl=(if ((if ((sf[24])!=0.0){pi}else{b})>p6){d}else{b});let pm=(((sf[24])!=0.0)&&((pl)!=0.0));let ps=(((sf[24])!=0.0)&&(!((pl)!=0.0)));let py=(d+ph);let pB=(if ((if sb[47]{py}else{pi})>p6){d}else{b});let pC=(sb[47]&&((pB)!=0.0));let pF=(d+f64::powf(py,sf[86]));let pJ=(sb[47]&&(!((pB)!=0.0)));let pM=(if pJ{pL}else{(if pC{(pD*pF)}else{(if ps{pv}else{(if pm{(M*(pb+f64::powf(pi,sf[86])))}else{b})})})});let pN=(oY/pM);let pO=(oH/pM);let pR=(k2<dl);let pV=(!oK);let pY=(((sf[21])!=0.0)&&(!(((if pR{d}else{b}))!=0.0)));let q0=((dl*pQ)).exp();let q1=(k2-dl);let q3=(d+(pQ*q1));let q4=(q0*q3);let q5=(sb[11]&&pR);let q6=(if q5{pU}else{(if pV{oU}else{oN})});let q8=(jU<dl);let q9=(if q8{d}else{b});
        let qa=(((sf[21])!=0.0)&&((q9)!=0.0));let qf=(((sf[21])!=0.0)&&(!((q9)!=0.0)));let qg=(jU-dl);let qi=(d+(pQ*qg));let qj=(q0*qi);let qm=(!pR);let qq=(!q8);let qu=(((sf[189]*(if qm{q4}else{pU}))+(sf[190]*(if qq{qj}else{qc})))-d);let qA=((((if pY{q4}else{q6})*sf[189])+((if qf{qj}else{(if qa{qc}else{b})})*sf[190]))-d);let qB=(gD*qA);let qP=((if ((sf[21])!=0.0){(d+(c7*(sf[59]*(if ((sf[21])!=0.0){(gD*qu)}else{b}))))}else{(if sb[48]{py}else{pi})})).sqrt();let qT=(!((d+(c7*(if ((sf[21])!=0.0){(sf[59]*qB)}else{b})))>p6));let qV=(if sb[49]{d}else{(if qT{0.50005}else{(M*(d+qP))})});let qW=(d/gH);let qX=(if ((sf[25])!=0.0){qW}else{pQ});let qY=(jO<dG);let r1=((jO*qX)).exp();let r2=(sb[11]&&qm);let r4=(!(((if qY{d}else{b}))!=0.0));let r5=(((sf[25])!=0.0)&&r4);let r8=(if (!(gK>b)){b}else{dE});let ra=((qX*r8)).exp();let rb=(jO-r8);let rd=(d+(qX*rb));let rf=(jO<r8);let rg=(sb[13]&&rf);let rh=(if rg{r1}else{(if r2{q4}else{q6})});let rj=(d/gO);let rk=(if ((sf[25])!=0.0){rj}else{qX});let rl=(jO<e1);let rm=(if rl{d}else{b});let rn=(((sf[25])!=0.0)&&((rm)!=0.0));let rp=((jO*rk)).exp();let rr=(!((rm)!=0.0));let rs=(((sf[25])!=0.0)&&rr);let rv=(if (!(gR>b)){b}else{dZ});let rx=((rk*rv)).exp();let ry=(jO-rv);let rA=(d+(rk*ry));let rC=(if rs{(rx*rA)}else{(if rn{rp}else{b})});let rG=(d+(sf[28]*(pb-d)));let rH=(gK*rG);let rJ=((dG*qX)).exp();let rK=(jO-dG);let rM=(d+(qX*rK));let rN=(rJ*rM);let rO=(!qY);let rQ=((if rO{rN}else{r1})-d);let rT=((e1*rk)).exp();let rU=(jO-e1);let rW=(d+(rk*rU));let rX=(rT*rW);let rY=(!rl);let s0=((if rY{rX}else{rp})-d);let s1=(gR*s0);let sa=(iY-jO);let sb_=(if sb[53]{sa}else{b});let sc=(d/iZ);let sd=(if sb[53]{sc}else{rk});let sg=((sb_*sd)).exp();let sh=((if r5{(ra*rd)}else{rh})-d);let sj=(rC-d);let sk=(gR*sj);let sr=((bQ*sd)).exp();let ss=(sb_-bQ);let su=(d+(sd*ss));let sw=(!(sb_<bQ));let sF=(if sb[56]{qW}else{sd});let sG=(jR<dG);let sJ=((jR*sF)).exp();let sK=(sb[13]&&rO);let sM=(!(((if sG{d}else{b}))!=0.0));let sN=(sb[56]&&sM);let sP=((r8*sF)).exp();let sQ=(jR-r8);let sS=(d+(sF*sQ));let sW=(jR<r8);let sX=(sb[58]&&sW);let sY=(if sX{sJ}else{(if sK{rN}else{rh})});let t0=(if sb[56]{rj}else{sF});let t1=(jR<e1);let t2=(if t1{d}else{b});let t3=(sb[56]&&((t2)!=0.0));let t5=((jR*t0)).exp();let t7=(!((t2)!=0.0));let t8=(sb[56]&&t7);let ta=((rv*t0)).exp();let tb=(jR-rv);let td=(d+(t0*tb));let tf=(if t8{(ta*td)}else{(if t3{t5}else{rC})});let th=((dG*sF)).exp();let ti=(jR-dG);let tk=(d+(sF*ti));let tl=(th*tk);let tm=(!sG);let to=((if tm{tl}else{sJ})-d);let tr=((e1*t0)).exp();let ts=(jR-e1);let tu=(d+(t0*ts));let tv=(tr*tu);let tw=(!t1);let ty=((if tw{tv}else{t5})-d);let tD=(if sb[59]{sa}else{sb_});let tE=(if sb[59]{sc}else{t0});let tH=((tD*tE)).exp();let tI=((if sN{(sP*sS)}else{sY})-d);let tK=(tf-d);let tO=((bQ*tE)).exp();let tP=(tD-bQ);let tR=(d+(tE*tP));let tT=(!(tD<bQ));let u1=(if sb[61]{qW}else{tE});let u3=((jO*u1)).exp();let u4=(sb[58]&&tm);let u6=(r4&&sb[61]);let u8_=((r8*u1)).exp();let ua=(d+(rb*u1));let ue=(rf&&sb[63]);let uh=(if sb[61]{rj}else{u1});let ui=(((rm)!=0.0)&&sb[61]);let uk=((jO*uh)).exp();let um=(rr&&sb[61]);let uo=((rv*uh)).exp();let uq=(d+(ry*uh));let uv=((dG*u1)).exp();let ux=(d+(rK*u1));let uy=(uv*ux);let uA=((if rO{uy}else{u3})-d);let uD=((e1*uh)).exp();let uF=(d+(rU*uh));let uG=(uD*uF);let uI=((if rY{uG}else{uk})-d);let uJ=(gR*uI);let uT=(if sb[66]{sa}else{tD});let uU=(if sb[66]{sc}else{uh});let uX=((uT*uU)).exp();let uY=((if u6{(u8_*ua)}else{(if ue{u3}else{(if u4{tl}else{sY})})})-d);let v0=((if um{(uo*uq)}else{(if ui{uk}else{tf})})-d);let v1=(gR*v0);let va=((bQ*uU)).exp();let vb=(uT-bQ);let vd=(d+(uU*vb));let vf=(!(uT<bQ));let vk=(if sb[66]{((if sb[54]{(sf[13]*(v1+(gK*uY)))}else{(sf[13]*((rH*uY)+v1))})-(sf[191]*((if vf{(va*vd)}else{uX})-j1)))}else{(if sb[65]{(sf[13]*(uJ+(gK*uA)))}else{(if sb[64]{(sf[13]*((rH*uA)+uJ))}else{(if sb[56]{b}else{(if sb[53]{((if sb[54]{(sk+(gK*sh))}else{((rH*sh)+sk)})-(sf[30]*((if sw{(sr*su)}else{sg})-j1)))}else{(if sb[52]{(s1+(gK*rQ))}else{(if sb[50]{((rH*rQ)+s1)}else{b})})})})})})});let vl=(if sb[61]{qW}else{uU});let vn=((jR*vl)).exp();let vs=(rO&&sb[63]);
        let vu=(sM&&sb[61]);let vw=((r8*vl)).exp();let vy=(d+(sQ*vl));let vA=(sW&&sb[63]);let vD=(if sb[61]{rj}else{vl});let vF=((jR*vD)).exp();let vG=(sb[13]&&rY);let vI=(jR<rv);let vJ=(sb[58]&&vI);let vL=(sb[58]&&tw);let vO=(sb[63]&&(jO<rv));let vQ=(rY&&sb[63]);let vS=(t7&&sb[61]);let vU=((rv*vD)).exp();let vW=(d+(tb*vD));let vY=(sb[63]&&vI);let w3=((dG*vl)).exp();let w5=(d+(ti*vl));let w8=((if tm{(w3*w5)}else{vn})-d);let wb=((e1*vD)).exp();let wd=(d+(ts*vD));let wg=((if tw{(wb*wd)}else{vF})-d);let wl=(if sb[66]{sa}else{uT});let wm=(if sb[66]{sc}else{vD});let wp=((wl*wm)).exp();let wq=((if vu{(vw*vy)}else{(if vA{vn}else{(if vs{uy}else{(if ue{u3}else{(if u4{tl}else{(if sX{sJ}else{(if sK{rN}else{r1})})})})})})})-d);let ws=((if vS{(vU*vW)}else{(if vY{vF}else{(if vQ{uG}else{(if vO{uk}else{(if vL{tv}else{(if vJ{t5}else{(if vG{rX}else{rp})})})})})})})-d);let wy=((bQ*wm)).exp();let wz=(wl-bQ);let wB=(d+(wm*wz));let wD=(!(wl<bQ));let wI=(if sb[66]{((sf[192]*((gK*wq)+(gR*ws)))-(sf[193]*((if wD{(wy*wB)}else{wp})-j1)))}else{(if sb[61]{(sf[192]*((gK*w8)+(gR*wg)))}else{(if sb[59]{(((gK*tI)+(gR*tK))-(sf[30]*((if tT{(tO*tR)}else{tH})-j1)))}else{(if sb[56]{((gK*to)+(gR*ty))}else{b})})})});let wJ=(d/gV);let wK=(jU<el);let wN=((jU*wJ)).exp();let wO=(!(((if wK{d}else{b}))!=0.0));let wR=(if (!(gY>b)){b}else{ej});let wT=((wJ*wR)).exp();let wU=(jU-wR);let wW=(d+(wJ*wU));let wZ=(d/h2);let x0=(jU<eF);let x3=((jU*wZ)).exp();let x4=(!(((if x0{d}else{b}))!=0.0));let x7=(if (!(h5>b)){b}else{eD});let x9=((wZ*x7)).exp();let xa=(jU-x7);let xc=(d+(wZ*xa));let xf=((if wO{(wT*wW)}else{wN})-d);let xh=((if x4{(x9*xc)}else{x3})-d);let xj=((gY*xf)+(h5*xh));let xk=(if ((sf[34])!=0.0){wJ}else{wZ});let xo=((k2*xk)).exp();let xq=((el*wJ)).exp();let xr=(jU-el);let xt=(d+(wJ*xr));let xv=(!wK);let xy=(((sf[34])!=0.0)&&(!(((if (k2<(if sb[94]{b}else{eN})){d}else{b}))!=0.0)));let xB=(if (!(h7>b)){b}else{eN});let xD=((xk*xB)).exp();let xE=(k2-xB);let xG=(d+(xk*xE));let xJ=(sb[18]&&(k2<xB));let xM=(if ((sf[34])!=0.0){wZ}else{xk});let xQ=((k2*xM)).exp();let xS=((eF*wZ)).exp();let xT=(jU-eF);let xV=(d+(wZ*xT));let xX=(!x0);let y0=(((sf[34])!=0.0)&&(!(((if (k2<(if sb[96]{b}else{eX})){d}else{b}))!=0.0)));let y3=(if (!(h9>b)){b}else{eX});let y5=((xM*y3)).exp();let y6=(k2-y3);let y8=(d+(xM*y6));let yb=(sb[18]&&(k2<y3));let yf=((if xy{(xD*xG)}else{(if xJ{xo}else{(if xv{(xq*xt)}else{wN})})})-d);let yh=((if y0{(y5*y8)}else{(if yb{xQ}else{(if xX{(xS*xV)}else{x3})})})-d);let yk=(if sb[67]{b}else{((h7*yf)+(h9*yh))});let yK=(jg*ka);let yL=(d+yG);let yM=(d+yJ);let yN=(yL/yM);let yQ=((yG-yJ)-(yN).ln());let yS=(kc+(fl*yQ));let yT=(jj*yS);let yU=(jK*yT);let yW=(sf[62]*(M*jK));let yZ=((ae+(kc*kc))).sqrt();let z1=(d+(yW*yZ));let z2=(jj*z1);let z3=(yU/z2);let z6=((d+(z3*z3))).sqrt();let z7=(yT/z6);let z8=(jm*kd);let z9=(ke*pM);let za=(jp*z9);let zb=(js*kf);let zc=(kg*qV);let zd=(jv*zc);let ze=0.02;let zg=(ze*(d+hj));let zl=(if ((sf[41])!=0.0){f64::powf(zg,sf[196])}else{b});let zn=((iB-jU)-zl);let zq=((ae+(zn*zn))).sqrt();let zu=(if ((sf[41])!=0.0){(zl+(M*(zn+zq)))}else{b});let zv=(-hj);let zx=f64::powf(zu,sf[197]);let zz=(if ((sf[41])!=0.0){(zv*zx)}else{b});let zB=(if (zz<sf[55]){d}else{b});let zC=(((sf[41])!=0.0)&&((zB)!=0.0));let zD=(zz).exp();let zG=(((sf[41])!=0.0)&&(!((zB)!=0.0)));let zH=(if zG{sf[194]}else{b});let zL=(if zG{(zH*(d+(zz-sf[55])))}else{(if zC{zD}else{b})});let zM=(sf[40]*zu);let zO=(if ((sf[41])!=0.0){(zL*zM)}else{b});let zP=(ki-pN);let zQ=(zP-xj);let zV=(ze*(d+ho));let A0=(if ((sf[36])!=0.0){f64::powf(zV,sf[200])}else{b});let A2=((b-jZ)-A0);let A5=((ae+(A2*A2))).sqrt();let A9=(if ((sf[36])!=0.0){(A0+(M*(A2+A5)))}else{b});let Aa=(-ho);let Ac=f64::powf(A9,sf[201]);let Ae=(if ((sf[36])!=0.0){(Aa*Ac)}else{b});let Ag=(if (Ae<sf[55]){d}else{b});let Ah=(((sf[36])!=0.0)&&((Ag)!=0.0));let Ai=(Ae).exp();let Al=(((sf[36])!=0.0)&&(!((Ag)!=0.0)));let Am=(if Al{sf[194]}else{b});let Aq=(if Al{(Am*(d+(Ae-sf[55])))}else{(if Ah{Ai}else{b})});let Ar=(sf[35]*A9);let At=(if ((sf[36])!=0.0){(Aq*Ar)}else{zO});let Au=(-yK);let AC=0.1;let AE=(if sb[70]{((d-(jU/sf[8]))-AC)}else{b});
        let AH=((p3+(AE*AE))).sqrt();let AQ=(if sb[72]{sf[3]}else{(if sb[70]{(sf[3]*(if sb[70]{(AC+(M*(AE+AH)))}else{AE}))}else{b})});let AS=((pO/AQ)-d);let AZ=((xj-(if sb[68]{b}else{(zO*zQ)}))-(if sb[73]{b}else{(sf[2]*f64::powf(AS,sf[202]))}));let BI=(sf[53]*z7);let Eo=(if (oH>b){d}else{b});let Eq=(sf[68]*(oH*Eo));let Er=(d+Eq);let Es=(Eq/Er);let EN=(sf[69]+(Es*Es));let EQ=(d+(Eo*(EL*EN)));let ER=(EJ*EQ);let EU=(oH*ER);let FM=(Fx*(sf[127]*f64::powf(fm,sf[216])));let G6=(Fx*(sf[132]*f64::powf(fm,sf[219])));let Hp=(sf[104]*Fw);let Ht=(gH*gH);let Hy=((gJ*(sf[102]*(Fx*(sf[105]*f64::powf(fm,sf[226])))))+(gF*(gJ*(((gH*(sf[107]*GB))-(gG*Hp))/Ht))));let HF=(sf[110]*Fw);let HJ=(gO*gO);let HO=((gQ*(sf[108]*(Fx*(sf[111]*f64::powf(fm,sf[227])))))+(gM*(gQ*(((gO*(sf[113]*GB))-(gN*HF))/HJ))));let HS=(Fx*(sf[116]*f64::powf(fm,sf[228])));let HV=(sf[115]*Fw);let HZ=(gV*gV);let I1=(gX*(((gV*(sf[118]*GB))-(gU*HV))/HZ));let I8=(Fx*(sf[121]*f64::powf(fm,sf[229])));let Ib=(sf[120]*Fw);let If=(h2*h2);let Ih=(h4*(((h2*(sf[123]*GB))-(h1*Ib))/If));let Ix=(sf[142]*(sf[143]*Fu));let Iz=(sf[144]*(sf[145]*Fu));let KI=(-(sf[5]*((hs*Fu)+(fn_*(sf[147]*Fu)))));let KL=((hz*Fw)+(fl*(sf[78]*(sf[148]*Fu))));let KP=(iZ*iZ);let KR=(j1*(((iZ*KI)-(iY*KL))/KP));let L3=(if jh{((-(if sb[38]{(sf[128]*FM)}else{(if ((sf[39])!=0.0){(sf[128]*(Fx*(sf[129]*f64::powf(fm,sf[217]))))}else{b})}))/(fI*fI))}else{b});let LD=(if jI{((-(sf[161]*(Fx*(sf[162]*f64::powf(fm,sf[232])))))/(iX*iX))}else{b});let V8=((oG*GM)+(gp*(if oy{((oD*(oA*(cp*UO)))+(oA*(oB*UO)))}else{US})));let V9=(gp*(if oy{(oA*UQ)}else{UT}));let Va=(gp*(if oy{(oA*UR)}else{UU}));let Vs=((oT*(oQ*(cV*Vg)))+(oQ*(oR*Vg)));let Vt=(oQ*Vi);let Vu=(oQ*Vj);let VD=((oX*VA)+(oW*(if oO{Vs}else{Vk})));let VE=(oW*(if oO{Vt}else{Vl}));let VF=(oW*(if oO{Vu}else{Vm}));let Wu=(c7*(((oH*Lz)+(jH*V8))+(sf[57]*VD)));let Wv=(c7*(sf[57]*VE));let Ww=(c7*((jH*V9)+(sf[57]*VF)));let Wx=(c7*(jH*Va));let Wy=(Wq+Wu);let Wz=(Wr+Wv);let WA=(Ws+Ww);let WB=(Wt+Wx);let WE=(sf[86]*f64::powf(pi,sf[254]));let X4=(sf[86]*f64::powf(py,sf[254]));let Xt=(if pJ{Xp}else{(if pC{((pF*WV)+(pD*(Wu*X4)))}else{(if ps{WV}else{(if pm{(M*(W9+(Wy*WE)))}else{b})})})});let Xu=(if pJ{Xq}else{(if pC{((pF*WW)+(pD*(Wv*X4)))}else{(if ps{WW}else{(if pm{(M*(Wa+(Wz*WE)))}else{b})})})});let Xv=(if pJ{Xr}else{(if pC{((pF*WX)+(pD*(Ww*X4)))}else{(if ps{WX}else{(if pm{(M*(Wb+(WA*WE)))}else{b})})})});let Xw=(if pJ{Xs}else{(if pC{((pF*WY)+(pD*(Wx*X4)))}else{(if ps{WY}else{(if pm{(M*(Wc+(WB*WE)))}else{b})})})});let XA=(pM*pM);let XB=(((pM*VD)-(oY*Xt))/XA);let XF=(((pM*VE)-(oY*Xu))/XA);let XJ=(((pM*VF)-(oY*Xv))/XA);let XM=((-(oY*Xw))/XA);let XQ=(((pM*V8)-(oH*Xt))/XA);let XT=((-(oH*Xu))/XA);let XX=(((pM*V9)-(oH*Xv))/XA);let Y1=(((pM*Va)-(oH*Xw))/XA);let Yf=(q0*(dl*Y4));let Yj=((q3*Yf)+(q0*(q1*Y4)));let Yk=(q0*Y6);let Yl=(q0*Y7);let Ym=(if q5{Y8}else{(if pV{Vs}else{Vk})});let Yn=(if q5{b}else{(if pV{Vt}else{Vl})});let Yo=(if q5{Y9}else{b});let Yp=(if q5{b}else{(if pV{Vu}else{Vm})});let Yq=(if q5{Ya}else{b});let YG=((qi*Yf)+(q0*(qg*Y4)));let ZQ=(hA*qP);let a0g=((-Hp)/Ht);let a0h=(if ((sf[25])!=0.0){a0g}else{Y4});let a0j=(sf[53]*qX);let a0k=(qX*sf[233]);let a0l=(r1*(jO*a0h));let a0m=(r1*a0j);let a0n=(r1*a0k);let a0B=(if rg{a0l}else{(if r2{Yj}else{Ym})});let a0C=(if rg{b}else{(if r2{b}else{Yn})});let a0D=(if rg{b}else{(if r2{Yk}else{Yo})});let a0E=(if rg{a0m}else{(if r2{b}else{Yp})});let a0F=(if rg{a0n}else{b});let a0G=(if rg{b}else{(if r2{Yl}else{Yq})});let a0H=(if r5{((rd*(ra*(r8*a0h)))+(ra*(rb*a0h)))}else{a0B});let a0I=(if r5{b}else{a0C});let a0J=(if r5{b}else{a0D});let a0K=(if r5{(ra*a0j)}else{a0E});let a0L=(if r5{(ra*a0k)}else{a0F});let a0M=(if r5{b}else{a0G});let a0O=((-HF)/HJ);let a0P=(if ((sf[25])!=0.0){a0O}else{a0h});let a0R=(sf[53]*rk);let a0S=(rk*sf[233]);let a0T=(rp*(jO*a0P));let a0U=(rp*a0R);let a0V=(rp*a0S);let a17=(if rs{((rA*(rx*(rv*a0P)))+(rx*(ry*a0P)))}else{(if rn{a0T}else{b})});let a18=(if rs{(rx*a0R)}else{(if rn{a0U}else{b})});let a19=(if rs{(rx*a0S)}else{(if rn{a0V}else{b})});let a1g=((rG*Hy)+(gK*(sf[28]*W9)));let a1h=(gK*(sf[28]*Wa));let a1i=(gK*(sf[28]*Wb));
        let a1j=(gK*(sf[28]*Wc));let a1p=((rM*(rJ*(dG*a0h)))+(rJ*(rK*a0h)));let a1q=(rJ*a0j);let a1r=(rJ*a0k);let a1s=(if rO{a1p}else{a0l});let a1t=(if rO{a1q}else{a0m});let a1u=(if rO{a1r}else{a0n});let a1K=((rW*(rT*(e1*a0P)))+(rT*(rU*a0P)));let a1L=(rT*a0R);let a1M=(rT*a0S);let a1S=((s0*HO)+(gR*(if rY{a1K}else{a0T})));let a1T=(gR*(if rY{a1L}else{a0U}));let a1U=(gR*(if rY{a1M}else{a0V}));let a2e=(if sb[53]{KI}else{b});let a2i=((-KL)/KP);let a2j=(if sb[53]{a2i}else{a0P});let a2k=(sd*a2e);let a2n=(sd*sf[255]);let a2o=(sd*sf[256]);let a2I=((sj*HO)+(gR*a17));let a2J=(gR*a18);let a2K=(gR*a19);let a3A=(if sb[56]{a0g}else{a2j});let a3C=(sf[53]*sF);let a3D=(sF*sf[233]);let a3E=(sJ*(jR*a3A));let a3F=(sJ*a3C);let a3G=(sJ*a3D);let a3V=(if sX{a3E}else{(if sK{a1p}else{a0B})});let a3W=(if sX{b}else{(if sK{b}else{a0C})});let a3X=(if sX{a3F}else{(if sK{b}else{a0D})});let a3Y=(if sX{b}else{(if sK{a1q}else{a0E})});let a3Z=(if sX{a3G}else{(if sK{a1r}else{a0F})});let a40=(if sX{b}else{(if sK{b}else{a0G})});let a47=(if sb[56]{a0O}else{a3A});let a49=(sf[53]*t0);let a4a=(t0*sf[233]);let a4b=(t5*(jR*a47));let a4c=(t5*a49);let a4d=(t5*a4a);let a4q=(if t8{((td*(ta*(rv*a47)))+(ta*(tb*a47)))}else{(if t3{a4b}else{a17})});let a4r=(if t8{(ta*a49)}else{(if t3{a4c}else{b})});let a4s=(if t8{b}else{(if t3{b}else{a18})});let a4t=(if t8{(ta*a4a)}else{(if t3{a4d}else{a19})});let a4z=((tk*(th*(dG*a3A)))+(th*(ti*a3A)));let a4A=(th*a3C);let a4B=(th*a3D);let a4P=((tu*(tr*(e1*a47)))+(tr*(ts*a47)));let a4Q=(tr*a49);let a4R=(tr*a4a);let a56=(if sb[59]{KI}else{a2e});let a59=(if sb[59]{a2i}else{a47});let a5a=(tE*a56);let a5d=(tE*sf[257]);let a5e=(tE*sf[258]);let a5Z=(if sb[61]{a0g}else{a59});let a61=(sf[53]*u1);let a62=(u1*sf[233]);let a63=(u3*(jO*a5Z));let a64=(u3*a61);let a65=(u3*a62);let a6q=(if u6{((ua*(u8_*(r8*a5Z)))+(u8_*(rb*a5Z)))}else{(if ue{a63}else{(if u4{a4z}else{a3V})})});let a6r=(if u6{b}else{(if ue{b}else{(if u4{b}else{a3W})})});let a6s=(if u6{b}else{(if ue{b}else{(if u4{a4A}else{a3X})})});let a6t=(if u6{(u8_*a61)}else{(if ue{a64}else{(if u4{b}else{a3Y})})});let a6u=(if u6{(u8_*a62)}else{(if ue{a65}else{(if u4{a4B}else{a3Z})})});let a6v=(if u6{b}else{(if ue{b}else{(if u4{b}else{a40})})});let a6w=(if sb[61]{a0O}else{a5Z});let a6y=(sf[53]*uh);let a6z=(uh*sf[233]);let a6A=(uk*(jO*a6w));let a6B=(uk*a6y);let a6C=(uk*a6z);let a6Y=((ux*(uv*(dG*a5Z)))+(uv*(rK*a5Z)));let a6Z=(uv*a61);let a70=(uv*a62);let a71=(if rO{a6Y}else{a63});let a72=(if rO{a6Z}else{a64});let a73=(if rO{a70}else{a65});let a7j=((uF*(uD*(e1*a6w)))+(uD*(rU*a6w)));let a7k=(uD*a6y);let a7l=(uD*a6z);let a7r=((uI*HO)+(gR*(if rY{a7j}else{a6A})));let a7s=(gR*(if rY{a7k}else{a6B}));let a7t=(gR*(if rY{a7l}else{a6C}));let a7Y=(if sb[66]{KI}else{a56});let a81=(if sb[66]{a2i}else{a6w});let a82=(uU*a7Y);let a85=(uU*sf[259]);let a86=(uU*sf[260]);let a8q=((v0*HO)+(gR*(if um{((uq*(uo*(rv*a6w)))+(uo*(ry*a6w)))}else{(if ui{a6A}else{a4q})})));let a8r=(gR*(if um{b}else{(if ui{b}else{a4r})}));let a8s=(gR*(if um{(uo*a6y)}else{(if ui{a6B}else{a4s})}));let a8t=(gR*(if um{(uo*a6z)}else{(if ui{a6C}else{a4t})}));let a9l=(if sb[66]{((if sb[54]{(sf[13]*(a8q+((uY*Hy)+(gK*a6q))))}else{(sf[13]*(((uY*a1g)+(rH*a6q))+a8q))})-(sf[191]*((if vf{((vd*(va*(bQ*a81)))+(va*(a82+(vb*a81))))}else{(uX*(a82+(uT*a81)))})-KR)))}else{(if sb[65]{(sf[13]*(a7r+((uA*Hy)+(gK*a71))))}else{(if sb[64]{(sf[13]*(((uA*a1g)+(rH*a71))+a7r))}else{(if sb[56]{b}else{(if sb[53]{((if sb[54]{(a2I+((sh*Hy)+(gK*a0H)))}else{(((sh*a1g)+(rH*a0H))+a2I)})-(sf[30]*((if sw{((su*(sr*(bQ*a2j)))+(sr*(a2k+(ss*a2j))))}else{(sg*(a2k+(sb_*a2j)))})-KR)))}else{(if sb[52]{(a1S+((rQ*Hy)+(gK*a1s)))}else{(if sb[50]{(((rQ*a1g)+(rH*a1s))+a1S)}else{b})})})})})})});let a9m=(if sb[66]{(if sb[54]{(sf[13]*(gK*a6r))}else{(sf[13]*((uY*a1h)+(rH*a6r)))})}else{(if sb[65]{b}else{(if sb[64]{(sf[13]*(uA*a1h))}else{(if sb[56]{b}else{(if sb[53]{(if sb[54]{(gK*a0I)}else{((sh*a1h)+(rH*a0I))})}else{(if sb[52]{b}else{(if sb[50]{(rQ*a1h)}else{b})})})})})})});
        let a9n=(if sb[66]{(if sb[54]{(sf[13]*(a8r+(gK*a6s)))}else{(sf[13]*((rH*a6s)+a8r))})}else{(if sb[65]{b}else{(if sb[64]{b}else{(if sb[56]{b}else{(if sb[53]{(if sb[54]{(gK*a0J)}else{(rH*a0J)})}else{b})})})})});let a9o=(if sb[66]{((if sb[54]{(sf[13]*(a8s+(gK*a6t)))}else{(sf[13]*(((uY*a1i)+(rH*a6t))+a8s))})-(sf[191]*(if vf{(va*a85)}else{(uX*a85)})))}else{(if sb[65]{(sf[13]*(a7s+(gK*a72)))}else{(if sb[64]{(sf[13]*(((uA*a1i)+(rH*a72))+a7s))}else{(if sb[56]{b}else{(if sb[53]{((if sb[54]{(a2J+(gK*a0K))}else{(((sh*a1i)+(rH*a0K))+a2J)})-(sf[30]*(if sw{(sr*a2n)}else{(sg*a2n)})))}else{(if sb[52]{(a1T+(gK*a1t))}else{(if sb[50]{(((rQ*a1i)+(rH*a1t))+a1T)}else{b})})})})})})});let a9p=(if sb[66]{((if sb[54]{(sf[13]*(a8t+(gK*a6u)))}else{(sf[13]*(((uY*a1j)+(rH*a6u))+a8t))})-(sf[191]*(if vf{(va*a86)}else{(uX*a86)})))}else{(if sb[65]{(sf[13]*(a7t+(gK*a73)))}else{(if sb[64]{(sf[13]*(((uA*a1j)+(rH*a73))+a7t))}else{(if sb[56]{b}else{(if sb[53]{((if sb[54]{(a2K+(gK*a0L))}else{(((sh*a1j)+(rH*a0L))+a2K)})-(sf[30]*(if sw{(sr*a2o)}else{(sg*a2o)})))}else{(if sb[52]{(a1U+(gK*a1u))}else{(if sb[50]{(((rQ*a1j)+(rH*a1u))+a1U)}else{b})})})})})})});let a9q=(if sb[66]{(if sb[54]{(sf[13]*(gK*a6v))}else{(sf[13]*(rH*a6v))})}else{(if sb[65]{b}else{(if sb[64]{b}else{(if sb[56]{b}else{(if sb[53]{(if sb[54]{(gK*a0M)}else{(rH*a0M)})}else{b})})})})});let a9r=(if sb[61]{a0g}else{a81});let a9t=(sf[53]*vl);let a9u=(vl*sf[233]);let a9v=(vn*(jR*a9r));let a9w=(vn*a9t);let a9x=(vn*a9u);let aa7=(if sb[61]{a0O}else{a9r});let aa9=(sf[53]*vD);let aaa=(vD*sf[233]);let aab=(vF*(jR*aa7));let aac=(vF*aa9);let aad=(vF*aaa);let abp=(if sb[61]{(sf[192]*(((w8*Hy)+(gK*(if tm{((w5*(w3*(dG*a9r)))+(w3*(ti*a9r)))}else{a9v})))+((wg*HO)+(gR*(if tw{((wd*(wb*(e1*aa7)))+(wb*(ts*aa7)))}else{aab})))))}else{(if sb[59]{((((tI*Hy)+(gK*(if sN{((sS*(sP*(r8*a3A)))+(sP*(sQ*a3A)))}else{a3V})))+((tK*HO)+(gR*a4q)))-(sf[30]*((if tT{((tR*(tO*(bQ*a59)))+(tO*(a5a+(tP*a59))))}else{(tH*(a5a+(tD*a59)))})-KR)))}else{(if sb[56]{(((to*Hy)+(gK*(if tm{a4z}else{a3E})))+((ty*HO)+(gR*(if tw{a4P}else{a4b}))))}else{b})})});let aby=(if sb[66]{a2i}else{aa7});let abz=(wm*(if sb[66]{KI}else{a7Y}));let abC=(wm*sf[261]);let abD=(wm*sf[262]);let ack=(if sb[66]{((sf[192]*(((wq*Hy)+(gK*(if vu{((vy*(vw*(r8*a9r)))+(vw*(sQ*a9r)))}else{(if vA{a9v}else{(if vs{a6Y}else{(if ue{a63}else{(if u4{a4z}else{(if sX{a3E}else{(if sK{a1p}else{a0l})})})})})})})))+((ws*HO)+(gR*(if vS{((vW*(vU*(rv*aa7)))+(vU*(tb*aa7)))}else{(if vY{aab}else{(if vQ{a7j}else{(if vO{a6A}else{(if vL{a4P}else{(if vJ{a4b}else{(if vG{a1K}else{a0T})})})})})})})))))-(sf[193]*((if wD{((wB*(wy*(bQ*aby)))+(wy*(abz+(wz*aby))))}else{(wp*(abz+(wl*aby)))})-KR)))}else{abp});let acl=(if sb[66]{b}else{(if sb[61]{b}else{(if sb[59]{(gK*(if sN{b}else{a3W}))}else{b})})});let acm=(if sb[66]{(sf[192]*((gK*(if vu{(vw*a9t)}else{(if vA{a9w}else{(if vs{b}else{(if ue{b}else{(if u4{a4A}else{(if sX{a3F}else{b})})})})})}))+(gR*(if vS{(vU*aa9)}else{(if vY{aac}else{(if vQ{b}else{(if vO{b}else{(if vL{a4Q}else{(if vJ{a4c}else{b})})})})})}))))}else{(if sb[61]{(sf[192]*((gK*(if tm{(w3*a9t)}else{a9w}))+(gR*(if tw{(wb*aa9)}else{aac}))))}else{(if sb[59]{((gK*(if sN{(sP*a3C)}else{a3X}))+(gR*a4r))}else{(if sb[56]{((gK*(if tm{a4A}else{a3F}))+(gR*(if tw{a4Q}else{a4c})))}else{b})})})});let acn=(if sb[66]{((sf[192]*((gK*(if vu{b}else{(if vA{b}else{(if vs{a6Z}else{(if ue{a64}else{(if u4{b}else{(if sX{b}else{(if sK{a1q}else{a0m})})})})})})}))+(gR*(if vS{b}else{(if vY{b}else{(if vQ{a7k}else{(if vO{a6B}else{(if vL{b}else{(if vJ{b}else{(if vG{a1L}else{a0U})})})})})})}))))-(sf[193]*(if wD{(wy*abC)}else{(wp*abC)})))}else{(if sb[61]{b}else{(if sb[59]{(((gK*(if sN{b}else{a3Y}))+(gR*a4s))-(sf[30]*(if tT{(tO*a5d)}else{(tH*a5d)})))}else{b})})});
        let aco=(if sb[66]{((sf[192]*((gK*(if vu{(vw*a9u)}else{(if vA{a9x}else{(if vs{a70}else{(if ue{a65}else{(if u4{a4B}else{(if sX{a3G}else{(if sK{a1r}else{a0n})})})})})})}))+(gR*(if vS{(vU*aaa)}else{(if vY{aad}else{(if vQ{a7l}else{(if vO{a6C}else{(if vL{a4R}else{(if vJ{a4d}else{(if vG{a1M}else{a0V})})})})})})}))))-(sf[193]*(if wD{(wy*abD)}else{(wp*abD)})))}else{(if sb[61]{(sf[192]*((gK*(if tm{(w3*a9u)}else{a9x}))+(gR*(if tw{(wb*aaa)}else{aad}))))}else{(if sb[59]{(((gK*(if sN{(sP*a3D)}else{a3Z}))+(gR*a4t))-(sf[30]*(if tT{(tO*a5e)}else{(tH*a5e)})))}else{(if sb[56]{((gK*(if tm{a4B}else{a3G}))+(gR*(if tw{a4R}else{a4d})))}else{b})})})});let acp=(if sb[66]{b}else{(if sb[61]{b}else{(if sb[59]{(gK*(if sN{b}else{a40}))}else{b})})});let acr=((-HV)/HZ);let act=(wJ*sf[233]);let acu=(sf[53]*wJ);let acv=(wN*(jU*acr));let acw=(wN*act);let acx=(wN*acu);let acK=((-Ib)/If);let acM=(wZ*sf[233]);let acN=(sf[53]*wZ);let acO=(x3*(jU*acK));let acP=(x3*acM);let acQ=(x3*acN);let adc=(((xf*((gX*(sf[114]*HS))+(gT*I1)))+(gY*(if wO{((wW*(wT*(wR*acr)))+(wT*(wU*acr)))}else{acv})))+((xh*((h4*(sf[119]*I8))+(h0*Ih)))+(h5*(if x4{((xc*(x9*(x7*acK)))+(x9*(xa*acK)))}else{acO}))));let add=((gY*(if wO{(wT*act)}else{acw}))+(h5*(if x4{(x9*acM)}else{acP})));let ade=((gY*(if wO{(wT*acu)}else{acx}))+(h5*(if x4{(x9*acN)}else{acQ})));let adf=(if ((sf[34])!=0.0){acr}else{acK});let adh=(sf[53]*xk);let adi=(xk*sf[233]);let adP=(if ((sf[34])!=0.0){acK}else{adf});let adR=(sf[53]*xM);let adS=(xM*sf[233]);let aeI=(if sb[67]{b}else{(((yf*((h6*I1)+(gX*(sf[32]*HS))))+(h7*(if xy{((xG*(xD*(xB*adf)))+(xD*(xE*adf)))}else{(if xJ{(xo*(k2*adf))}else{(if xv{((xt*(xq*(el*acr)))+(xq*(xr*acr)))}else{acv})})})))+((yh*((h8*Ih)+(h4*(sf[33]*I8))))+(h9*(if y0{((y8*(y5*(y3*adP)))+(y5*(y6*adP)))}else{(if yb{(xQ*(k2*adP))}else{(if xX{((xV*(xS*(eF*acK)))+(xS*(xT*acK)))}else{acO})})}))))});let aeJ=(if sb[67]{b}else{((h7*(if xy{b}else{(if xJ{b}else{(if xv{(xq*act)}else{acw})})}))+(h9*(if y0{b}else{(if yb{b}else{(if xX{(xS*acM)}else{acP})})})))});let aeK=(if sb[67]{b}else{((h7*(if xy{(xD*adh)}else{(if xJ{(xo*adh)}else{b})}))+(h9*(if y0{(y5*adR)}else{(if yb{(xQ*adR)}else{b})})))});let aeL=(if sb[67]{b}else{((h7*(if xy{b}else{(if xJ{b}else{(if xv{(xq*acu)}else{acx})})}))+(h9*(if y0{b}else{(if yb{b}else{(if xX{(xS*acN)}else{acQ})})})))});let aeM=(if sb[67]{b}else{((h7*(if xy{(xD*adi)}else{(if xJ{(xo*adi)}else{b})}))+(h9*(if y0{(y5*adS)}else{(if yb{(xQ*adS)}else{b})})))});let aft=(ka*(if jd{((-(if sb[37]{(sf[125]*FM)}else{(if ((sf[22])!=0.0){(sf[125]*(Fx*(sf[126]*f64::powf(fm,sf[215]))))}else{b})}))/(fA*fA))}else{b}));let afu=(-jg);let afy=(yM*yM);let ag3=((yS*L3)+(jj*((yQ*Fw)+(fl*((afh-afq)-((((yM*afh)-(yL*afq))/afy)/yN))))));let ag4=(jj*(sf[53]+(fl*((-afr)-(((-(yL*afr))/afy)/yN)))));let ag5=(jj*(sf[233]+(fl*(afi-((afi/yM)/yN)))));let ag6=(jj*(fl*((afj-afs)-((((yM*afj)-(yL*afs))/afy)/yN))));let agf=(sf[53]*kc);let agh=(kc*sf[233]);let agj=(hA*yZ);let agx=(z2*z2);let agI=(z3*(((z2*((yT*LD)+(jK*ag3)))-(yU*((z1*L3)+(jj*(yZ*(sf[62]*(M*LD)))))))/agx));let agK=(z3*(((z2*(jK*ag4))-(yU*(jj*(yW*((agf+agf)/agj)))))/agx));let agM=(z3*(((z2*(jK*ag5))-(yU*(jj*(yW*((agh+agh)/agj)))))/agx));let agO=(z3*((jK*ag6)/z2));let agQ=(hA*z6);let agY=(z6*z6);let agZ=(((z6*ag3)-(yT*((agI+agI)/agQ)))/agY);let ah3=(((z6*ag4)-(yT*((agK+agK)/agQ)))/agY);let ah7=(((z6*ag5)-(yT*((agM+agM)/agQ)))/agY);let ahb=(((z6*ag6)-(yT*((agO+agO)/agQ)))/agY);let ahc=(kd*(if jk{((-(if sb[39]{(sf[130]*G6)}else{(if ((sf[18])!=0.0){(sf[130]*(Fx*(sf[131]*f64::powf(fm,sf[218]))))}else{b})}))/(fS*fS))}else{b}));let ahd=(-jm);let ahm=((z9*(if jn{((-(if sb[40]{(sf[133]*G6)}else{(if ((sf[26])!=0.0){(sf[133]*(Fx*(sf[134]*f64::powf(fm,sf[220]))))}else{b})}))/(g0*g0))}else{b}))+(jp*(ke*Xt)));let ahn=(jp*(ke*Xu));let aho=(jp*pM);let ahp=(jp*((-pM)+(ke*Xv)));let ahq=(jp*(ke*Xw));let ahr=(kf*(if jq{((-(sf[135]*(Fx*(sf[136]*f64::powf(fm,sf[221])))))/(g4*g4))}else{b}));let ahs=(-js);
        let ahD=((zc*(if jt{((-(if sb[41]{(sf[137]*FM)}else{(if ((sf[17])!=0.0){(sf[137]*(Fx*(sf[138]*f64::powf(fm,sf[222]))))}else{b})}))/(gc*gc))}else{b}))+(jv*(kg*(if sb[49]{b}else{(if qT{b}else{(M*((if ((sf[21])!=0.0){(c7*(sf[59]*(if ((sf[21])!=0.0){((qu*Hi)+(gD*((sf[189]*(if qm{Yj}else{Y8}))+(sf[190]*(if qq{YG}else{Yx})))))}else{b})))}else{(if sb[48]{Wu}else{Wy})})/ZQ))})}))));let ahE=(jv*(-qV));let ahF=(jv*(kg*(if sb[49]{b}else{(if qT{b}else{(M*((if ((sf[21])!=0.0){(c7*(sf[59]*(if ((sf[21])!=0.0){(gD*(sf[190]*(if qq{Yl}else{Yy})))}else{b})))}else{(if sb[48]{Wv}else{Wz})})/ZQ))})})));let ahG=(jv*(kg*(if sb[49]{b}else{(if qT{b}else{(M*((if ((sf[21])!=0.0){(c7*(sf[59]*(if ((sf[21])!=0.0){(gD*(sf[189]*(if qm{Yk}else{Y9})))}else{b})))}else{b})/ZQ))})})));let ahH=(jv*(kg*(if sb[49]{b}else{(if qT{b}else{(M*((if ((sf[21])!=0.0){(c7*(sf[59]*(if ((sf[21])!=0.0){(gD*(sf[190]*(if qq{Yk}else{Yz})))}else{b})))}else{(if sb[48]{Ww}else{WA})})/ZQ))})})));let ahI=(jv*(kg*(if sb[49]{b}else{(if qT{b}else{(M*((if ((sf[21])!=0.0){b}else{(if sb[48]{Wx}else{WB})})/ZQ))})})));let ahJ=(jv*(qV+(kg*(if sb[49]{b}else{(if qT{b}else{(M*((if ((sf[21])!=0.0){(c7*(sf[59]*(if ((sf[21])!=0.0){(gD*(sf[189]*(if qm{Yl}else{Ya})))}else{b})))}else{b})/ZQ))})}))));let ahP=(if ((sf[41])!=0.0){((ze*Ix)*(sf[196]*f64::powf(zg,sf[263])))}else{b});let ahQ=(K7-ahP);let ahR=(zn*ahQ);let ahT=(sf[53]*zn);let ahV=(zn*sf[233]);let ahX=(hA*zq);let ai8=(if ((sf[41])!=0.0){(ahP+(M*(ahQ+((ahR+ahR)/ahX))))}else{b});let ai9=(if ((sf[41])!=0.0){(M*(sf[53]+((ahT+ahT)/ahX)))}else{b});let aia=(if ((sf[41])!=0.0){(M*(sf[233]+((ahV+ahV)/ahX)))}else{b});let aie=(sf[197]*f64::powf(zu,sf[264]));let ain=(if ((sf[41])!=0.0){((zx*(-Ix))+(zv*(ai8*aie)))}else{b});let aio=(if ((sf[41])!=0.0){(zv*(ai9*aie))}else{b});let aip=(if ((sf[41])!=0.0){(zv*(aia*aie))}else{b});let aiO=(if ((sf[41])!=0.0){((zM*(if zG{(zH*ain)}else{(if zC{(zD*ain)}else{b})}))+(zL*(sf[40]*ai8)))}else{b});let aiP=(if ((sf[41])!=0.0){((zM*(if zG{(zH*aio)}else{(if zC{(zD*aio)}else{b})}))+(zL*(sf[40]*ai9)))}else{b});let aiQ=(if ((sf[41])!=0.0){((zM*(if zG{(zH*aip)}else{(if zC{(zD*aip)}else{b})}))+(zL*(sf[40]*aia)))}else{b});let aiR=(-XB);let aiS=(-XF);let aiT=(-XJ);let aiU=(-XM);let aji=(if ((sf[36])!=0.0){((ze*Iz)*(sf[200]*f64::powf(zV,sf[265])))}else{b});let ajj=(-aji);let ajk=(A2*ajj);let ajm=(sf[53]*A2);let ajo=(A2*sf[233]);let ajq=(hA*A5);let ajB=(if ((sf[36])!=0.0){(aji+(M*(ajj+((ajk+ajk)/ajq))))}else{b});let ajC=(if ((sf[36])!=0.0){(M*(sf[53]+((ajm+ajm)/ajq)))}else{b});let ajD=(if ((sf[36])!=0.0){(M*(sf[233]+((ajo+ajo)/ajq)))}else{b});let ajH=(sf[201]*f64::powf(A9,sf[266]));let ajQ=(if ((sf[36])!=0.0){((Ac*(-Iz))+(Aa*(ajB*ajH)))}else{b});let ajR=(if ((sf[36])!=0.0){(Aa*(ajC*ajH))}else{b});let ajS=(if ((sf[36])!=0.0){(Aa*(ajD*ajH))}else{b});let akP=(AE*sf[271]);let akR=(AE*sf[272]);let akT=(hA*AH);let alc=(AQ*AQ);let all=(sf[202]*f64::powf(AS,sf[273]));let alC=(-(if sb[68]{b}else{zO}));let alD=((adc-(if sb[68]{b}else{((zQ*aiO)+(zO*(aiR-adc)))}))-(if sb[73]{b}else{(sf[2]*((XQ/AQ)*all))}));let alE=((add-(if sb[68]{b}else{((zQ*aiP)+(zO*(aiS-add)))}))-(if sb[73]{b}else{(sf[2]*((((AQ*XT)-(pO*(if sb[72]{b}else{(if sb[70]{(sf[3]*(if sb[70]{(M*(sf[271]+((akP+akP)/akT)))}else{sf[271]}))}else{b})})))/alc)*all))}));let alF=((ade-(if sb[68]{b}else{((zQ*aiQ)+(zO*(aiT-ade)))}))-(if sb[73]{b}else{(sf[2]*((((AQ*XX)-(pO*(if sb[72]{b}else{(if sb[70]{(sf[3]*(if sb[70]{(M*(sf[272]+((akR+akR)/akT)))}else{sf[272]}))}else{b})})))/alc)*all))}));let alG=((-(if sb[68]{b}else{(zO*aiU)}))-(if sb[73]{b}else{(sf[2]*((Y1/AQ)*all))}));let ao0=(ak*sf[53]);let ao1=(ak*sf[233]);let avc=(sf[68]*(Eo*V8));let avd=(sf[68]*(Eo*V9));let ave=(sf[68]*(Eo*Va));let avi=(Er*Er);let avR=(Es*(((Er*avc)-(Eq*avc))/avi));let avT=(Es*(((Er*avd)-(Eq*avd))/avi));let avV=(Es*(((Er*ave)-(Eq*ave))/avi));

        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(8),
            multiplicity * ((sf[53]*(vk+(ak*jO)))),
            [3, 5, 6, 7, 8, 9],
            [(sf[53]*a9l), (sf[53]*a9m), (sf[53]*a9n), (sf[53]*(a9o+ao0)), (sf[53]*(a9p+ao1)), (sf[53]*a9q)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(8),
            multiplicity * ((sf[53]*(wI+(ak*jR)))),
            [3, 5, 6, 7, 8, 9],
            [(sf[53]*ack), (sf[53]*acl), (sf[53]*(acm+ao0)), (sf[53]*acn), (sf[53]*(aco+ao1)), (sf[53]*acp)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(5),
            Some(8),
            multiplicity * ((sf[53]*ki)),
            11,
            multiplicity * (sf[53]),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * ((sf[53]*pN)),
            [3, 5, 7, 8],
            [(sf[53]*XB), (sf[53]*XF), (sf[53]*XJ), (sf[53]*XM)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(5),
            multiplicity * ((sf[53]*(AZ+(ak*jU)))),
            [3, 5, 7, 8, 11],
            [(sf[53]*alD), (sf[53]*(alE+ao1)), (sf[53]*(alF+ao0)), (sf[53]*alG), (sf[53]*alC)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(4),
            multiplicity * ((sf[53]*((if sb[69]{b}else{(if ((sf[36])!=0.0){(At*Au)}else{b})})+(ak*jZ)))),
            [0, 3, 4, 5, 6, 7],
            [(sf[53]*(if sb[69]{b}else{(if ((sf[36])!=0.0){(At*afu)}else{b})})), (sf[53]*(if sb[69]{b}else{(if ((sf[36])!=0.0){((Au*(if ((sf[36])!=0.0){((Ar*(if Al{(Am*ajQ)}else{(if Ah{(Ai*ajQ)}else{b})}))+(Aq*(sf[35]*ajB)))}else{aiO}))+(At*(-aft)))}else{b})})), (sf[53]*((if sb[69]{b}else{(if ((sf[36])!=0.0){((Au*(if ((sf[36])!=0.0){((Ar*(if Al{(Am*ajR)}else{(if Ah{(Ai*ajR)}else{b})}))+(Aq*(sf[35]*ajC)))}else{b}))+(jg*At))}else{b})})+ao1)), (sf[53]*(if sb[69]{b}else{(if ((sf[36])!=0.0){(Au*(if ((sf[36])!=0.0){b}else{aiP}))}else{b})})), (sf[53]*((if sb[69]{b}else{(if ((sf[36])!=0.0){(Au*(if ((sf[36])!=0.0){((Ar*(if Al{(Am*ajS)}else{(if Ah{(Ai*ajS)}else{b})}))+(Aq*(sf[35]*ajD)))}else{b}))}else{b})})+ao0)), (sf[53]*(if sb[69]{b}else{(if ((sf[36])!=0.0){(Au*(if ((sf[36])!=0.0){b}else{aiQ}))}else{b})}))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * ((sf[53]*(yk+(ak*k2)))),
            [3, 5, 6, 7, 9],
            [(sf[53]*aeI), (sf[53]*aeJ), (sf[53]*(aeK+ao0)), (sf[53]*aeL), (sf[53]*(aeM+ao1))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(4),
            multiplicity * (yK),
            0,
            multiplicity * (jg),
            3,
            multiplicity * (aft),
            4,
            multiplicity * (afu),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(5),
            multiplicity * (BI),
            [3, 4, 5, 7],
            [(sf[53]*agZ), (sf[53]*ah3), (sf[53]*ah7), (sf[53]*ahb)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(6),
            multiplicity * (z8),
            1,
            multiplicity * (jm),
            3,
            multiplicity * (ahc),
            6,
            multiplicity * (ahd),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (za),
            [3, 5, 6, 7, 8],
            [ahm, ahn, aho, ahp, ahq],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(8),
            multiplicity * (zb),
            2,
            multiplicity * (js),
            3,
            multiplicity * (ahr),
            8,
            multiplicity * (ahs),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(9),
            Some(4),
            multiplicity * (zd),
            [3, 4, 5, 6, 7, 8, 9],
            [ahD, ahE, ahF, ahG, ahH, ahI, ahJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            None,
            multiplicity * ((ki-pO)),
            [3, 5, 7, 8, 11],
            [(-XQ), (-XT), (-XX), (-Y1), d],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(11),
            None,
            multiplicity * ((ki-kh)),
            10,
            multiplicity * (-1.0),
            11,
            multiplicity * (d),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((f0*jy)),
            3,
            multiplicity * ((jy+(f0*(if jw{((-(sf[139]*(sf[140]*Fu)))/(gh*gh))}else{b})))),
        );
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            None,
            multiplicity * (((((((((((((jO*vk)+(jU*AZ))+(k7*zP))+(jR*wI))+(k2*yk))+(ka*yK))+(kc*z7))+(kd*z8))+(ke*za))+(kf*zb))+(kg*zd))*sf[204])),
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11],
            &[(sf[204]*(yK+yK)), (sf[204]*(z8+z8)), (sf[204]*(zb+zb)), (sf[204]*(((((((((((jO*a9l)+(jU*alD))+(k7*aiR))+(jR*ack))+(k2*aeI))+(ka*aft))+(kc*agZ))+(kd*ahc))+(ke*ahm))+(kf*ahr))+(kg*ahD))), (sf[204]*(((Au+(ka*afu))+(BI+(kc*ah3)))+((-zd)+(kg*ahE)))), (sf[204]*((((((((jO*a9m)+((AZ*sf[233])+(jU*alE)))+((sf[53]*zP)+(k7*aiS)))+(jR*acl))+(k2*aeJ))+((z7*sf[233])+(kc*ah7)))+(ke*ahn))+(kg*ahF))), (sf[204]*((((((jO*a9n)+((sf[53]*wI)+(jR*acm)))+((sf[53]*yk)+(k2*aeK)))+((-z8)+(kd*ahd)))+(za+(ke*aho)))+(kg*ahG))), (sf[204]*(((((((((sf[53]*vk)+(jO*a9o))+((sf[53]*AZ)+(jU*alF)))+(k7*aiT))+(jR*acn))+(k2*aeL))+(kc*ahb))+((-za)+(ke*ahp)))+(kg*ahH))), (sf[204]*((((((((vk*sf[233])+(jO*a9p))+(jU*alG))+((zP*sf[233])+(k7*aiU)))+((wI*sf[233])+(jR*aco)))+(ke*ahq))+((-zb)+(kf*ahs)))+(kg*ahI))), (sf[204]*((((jO*a9q)+(jR*acp))+((yk*sf[233])+(k2*aeM)))+(zd+(kg*ahJ)))), (sf[204]*(k7+(jU*alC)))],
            &[],
            &[],
            multiplicity,
        );
        let Fl_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (sf[53]*(ET+(EU/pM))));
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (Fl_ddt),
            [3, 5, 7, 8],
            [(((sf[53]*(awq+(((pM*((ER*V8)+(oH*((EQ*avK)+(EJ*(Eo*((EN*avO)+(EL*(avR+avR)))))))))-(EU*Xt))/XA)))) * ddt_scale), (((sf[53]*(((pM*(oH*((EQ*avL)+(EJ*(Eo*(EN*avP))))))-(EU*Xu))/XA))) * ddt_scale), (((sf[53]*(awr+(((pM*((ER*V9)+(oH*((EQ*avM)+(EJ*(Eo*((EN*avQ)+(EL*(avT+avT)))))))))-(EU*Xv))/XA)))) * ddt_scale), (((sf[53]*(aws+(((pM*((ER*Va)+(oH*((EQ*avN)+(EJ*(Eo*(EL*(avV+avV))))))))-(EU*Xw))/XA)))) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let Fm_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, Fm);
        stamper.stamp_current_node3_local(
            Some(6),
            Some(8),
            multiplicity * (Fm_ddt),
            3,
            multiplicity * (((axI) * ddt_scale)),
            6,
            multiplicity * (((axJ) * ddt_scale)),
            8,
            multiplicity * (((axK) * ddt_scale)),
        );
        let Fn_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (sf[53]*((EZ+(oY*sf[208]))+F4)));
        stamper.stamp_current_node3_local(
            Some(7),
            Some(5),
            multiplicity * (Fn_ddt),
            3,
            multiplicity * ((((sf[53]*((ax6+(sf[208]*VD))+axf))) * ddt_scale)),
            5,
            multiplicity * ((((sf[53]*((ax7+(sf[208]*VE))+axg))) * ddt_scale)),
            7,
            multiplicity * ((((sf[53]*((ax8+(sf[208]*VF))+axh))) * ddt_scale)),
        );
        let Fo_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, Fo);
        stamper.stamp_current_node3_local(
            Some(7),
            Some(4),
            multiplicity * (Fo_ddt),
            3,
            multiplicity * (((axO) * ddt_scale)),
            4,
            multiplicity * (((axP) * ddt_scale)),
            7,
            multiplicity * (((axQ) * ddt_scale)),
        );
        let Fp_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (sf[53]*(F7+((if sb[49]{b}else{qB})*sf[208]))));
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (Fp_ddt),
            [3, 5, 6, 7, 9],
            [(((sf[53]*(axq+(sf[208]*(if sb[49]{b}else{((qA*Hi)+(gD*((sf[189]*(if pY{Yj}else{Ym}))+(sf[190]*(if qf{YG}else{(if qa{Yx}else{b})})))))}))))) * ddt_scale), (((sf[53]*(sf[208]*(if sb[49]{b}else{(gD*((sf[189]*(if pY{b}else{Yn}))+(sf[190]*(if qf{Yl}else{(if qa{Yy}else{b})}))))})))) * ddt_scale), (((sf[53]*(axr+(sf[208]*(if sb[49]{b}else{(gD*(sf[189]*(if pY{Yk}else{Yo})))}))))) * ddt_scale), (((sf[53]*(sf[208]*(if sb[49]{b}else{(gD*((sf[189]*(if pY{b}else{Yp}))+(sf[190]*(if qf{Yk}else{(if qa{Yz}else{b})}))))})))) * ddt_scale), (((sf[53]*(axs+(sf[208]*(if sb[49]{b}else{(gD*(sf[189]*(if pY{Yl}else{Yq})))}))))) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let Fb_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, Fb);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (Fb_ddt),
            1,
            multiplicity * (((sf[210]) * ddt_scale)),
            2,
            multiplicity * (((sf[280]) * ddt_scale)),
        );
        let Fd_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, Fd);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (Fd_ddt),
            0,
            multiplicity * (((sf[281]) * ddt_scale)),
            1,
            multiplicity * (((sf[211]) * ddt_scale)),
        );
        let Fh_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, Fh);
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (Fh_ddt),
            10,
            multiplicity * (((sf[213]) * ddt_scale)),
        );
        let Fk_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, Fk);
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (Fk_ddt),
            11,
            multiplicity * (((sf[282]) * ddt_scale)),
        );
        let Ff_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, Ff);
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (Ff_ddt),
            3,
            multiplicity * (((sf[212]) * ddt_scale)),
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
            b, d, M, c7, f0, fl, fm, fn_,
            gk, gp, gD, hA, iB, jH, jL, jM,
            jO, jP, jR, jS, jU, jV, k0, k2,
            k3, k4, k8, kh, ki, ot, ox, oJ,
            oN, oW, p3, p6, pb, pg_, pv, pD,
            pL, pQ, pU, qc, yG, yJ, EJ, EL,
            ET, EZ, F4, F7, Fb, Fd, Ff, Fh,
            Fk, Fm, Fo, Fu, Fw, Fx, GB, GM,
            Hi, K7, Lz, UO, UQ, UR, US, UT,
            UU, Vg, Vi, Vj, Vk, Vl, Vm, VA,
            W9, Wa, Wb, Wc, Wq, Wr, Ws, Wt,
            WV, WW, WX, WY, Xp, Xq, Xr, Xs,
            Y4, Y6, Y7, Y8, Y9, Ya, Yx, Yy,
            Yz, afh, afi, afj, afq, afr, afs, avK,
            avL, avM, avN, avO, avP, avQ, awq, awr,
            aws, ax6, ax7, ax8, axf, axg, axh, axq,
            axr, axs, axI, axJ, axK, axO, axP, axQ,
        }=self.eval_common_stamp_values(ctx);
        let p=&(*self.params);
        let m=self.multiplicity;
        let multiplicity=m;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let an=ctx.simparam_or("pnjmaxi", d);let ap=(if sb[22]{an}else{sf[45]});let c6=(M*ap);let cp=(if sb[79]{b}else{(if (sb[25]&&(ap>sf[48])){(sf[308]*((d+(f64::powf((c6*sf[87]),sf[89])/sf[311]))).ln())}else{(sf[308]*((d+(ap/sf[311]))).ln())})});let cV=(if sb[82]{b}else{(if (sb[24]&&(ap>sf[47])){(sf[316]*((d+(f64::powf((c6*sf[97]),sf[89])/sf[321]))).ln())}else{(sf[316]*((d+(ap/sf[321]))).ln())})});let dl=(if sb[84]{b}else{(if (sb[23]&&(ap>sf[46])){(sf[325]*((d+((sf[59]*(ap*ap))/sf[328]))).ln())}else{(sf[325]*((d+(ap/sf[328]))).ln())})});let oy=(!(((if (jO<cp){d}else{b}))!=0.0));let oA=((cp*ot)).exp();let oB=(jO-cp);let oD=(d+(ot*oB));let oG=((if oy{(oA*oD)}else{ox})-d);let oH=(gp*oG);let oK=(jU<cV);let oO=(!(((if oK{d}else{b}))!=0.0));let oQ=((cV*oJ)).exp();let oR=(jU-cV);let oT=(d+(oJ*oR));let oU=(oQ*oT);let oX=((if oO{oU}else{oN})-d);let oY=(oW*oX);let ph=(c7*((jH*oH)+(sf[57]*oY)));let pi=(pg_+ph);let pl=(if ((if ((sf[24])!=0.0){pi}else{b})>p6){d}else{b});let pm=(((sf[24])!=0.0)&&((pl)!=0.0));let ps=(((sf[24])!=0.0)&&(!((pl)!=0.0)));let py=(d+ph);let pB=(if ((if sb[47]{py}else{pi})>p6){d}else{b});let pC=(sb[47]&&((pB)!=0.0));let pF=(d+f64::powf(py,sf[86]));let pJ=(sb[47]&&(!((pB)!=0.0)));let pM=(if pJ{pL}else{(if pC{(pD*pF)}else{(if ps{pv}else{(if pm{(M*(pb+f64::powf(pi,sf[86])))}else{b})})})});let pR=(k2<dl);let pV=(!oK);let pY=(((sf[21])!=0.0)&&(!(((if pR{d}else{b}))!=0.0)));let q0=((dl*pQ)).exp();let q1=(k2-dl);let q3=(d+(pQ*q1));let q5=(sb[11]&&pR);let q9=(if (jU<dl){d}else{b});let qa=(((sf[21])!=0.0)&&((q9)!=0.0));let qf=(((sf[21])!=0.0)&&(!((q9)!=0.0)));let qg=(jU-dl);let qi=(d+(pQ*qg));let qA=((((if pY{(q0*q3)}else{(if q5{pU}else{(if pV{oU}else{oN})})})*sf[189])+((if qf{(q0*qi)}else{(if qa{qc}else{b})})*sf[190]))-d);let Eo=(if (oH>b){d}else{b});let Eq=(sf[68]*(oH*Eo));let Er=(d+Eq);let Es=(Eq/Er);let EN=(sf[69]+(Es*Es));let EQ=(d+(Eo*(EL*EN)));let ER=(EJ*EQ);let EU=(oH*ER);let V8=((oG*GM)+(gp*(if oy{((oD*(oA*(cp*UO)))+(oA*(oB*UO)))}else{US})));let V9=(gp*(if oy{(oA*UQ)}else{UT}));let Va=(gp*(if oy{(oA*UR)}else{UU}));let Vs=((oT*(oQ*(cV*Vg)))+(oQ*(oR*Vg)));let Vt=(oQ*Vi);let Vu=(oQ*Vj);let VD=((oX*VA)+(oW*(if oO{Vs}else{Vk})));let VE=(oW*(if oO{Vt}else{Vl}));let VF=(oW*(if oO{Vu}else{Vm}));let Wu=(c7*(((oH*Lz)+(jH*V8))+(sf[57]*VD)));let Wv=(c7*(sf[57]*VE));let Ww=(c7*((jH*V9)+(sf[57]*VF)));let Wx=(c7*(jH*Va));let WE=(sf[86]*f64::powf(pi,sf[254]));let X4=(sf[86]*f64::powf(py,sf[254]));let XA=(pM*pM);let Yf=(q0*(dl*Y4));let Yk=(q0*Y6);let Yl=(q0*Y7);let avc=(sf[68]*(Eo*V8));let avd=(sf[68]*(Eo*V9));let ave=(sf[68]*(Eo*Va));let avi=(Er*Er);let avR=(Es*(((Er*avc)-(Eq*avc))/avi));let avT=(Es*(((Er*avd)-(Eq*avd))/avi));let avV=(Es*(((Er*ave)-(Eq*ave))/avi));

        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(8),
            &[3, 5, 7, 8],
            &[(sf[53]*(awq+(((pM*((ER*V8)+(oH*((EQ*avK)+(EJ*(Eo*((EN*avO)+(EL*(avR+avR)))))))))-(EU*(if pJ{Xp}else{(if pC{((pF*WV)+(pD*(Wu*X4)))}else{(if ps{WV}else{(if pm{(M*(W9+((Wq+Wu)*WE)))}else{b})})})})))/XA))), (sf[53]*(((pM*(oH*((EQ*avL)+(EJ*(Eo*(EN*avP))))))-(EU*(if pJ{Xq}else{(if pC{((pF*WW)+(pD*(Wv*X4)))}else{(if ps{WW}else{(if pm{(M*(Wa+((Wr+Wv)*WE)))}else{b})})})})))/XA)), (sf[53]*(awr+(((pM*((ER*V9)+(oH*((EQ*avM)+(EJ*(Eo*((EN*avQ)+(EL*(avT+avT)))))))))-(EU*(if pJ{Xr}else{(if pC{((pF*WX)+(pD*(Ww*X4)))}else{(if ps{WX}else{(if pm{(M*(Wb+((Ws+Ww)*WE)))}else{b})})})})))/XA))), (sf[53]*(aws+(((pM*((ER*Va)+(oH*((EQ*avN)+(EJ*(Eo*(EL*(avV+avV))))))))-(EU*(if pJ{Xs}else{(if pC{((pF*WY)+(pD*(Wx*X4)))}else{(if ps{WY}else{(if pm{(M*(Wc+((Wt+Wx)*WE)))}else{b})})})})))/XA)))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3_local(
            Some(6),
            Some(8),
            3,
            multiplicity * (axI),
            6,
            multiplicity * (axJ),
            8,
            multiplicity * (axK),
        );
        stamper.stamp_current_reactive_node3_local(
            Some(7),
            Some(5),
            3,
            multiplicity * ((sf[53]*((ax6+(sf[208]*VD))+axf))),
            5,
            multiplicity * ((sf[53]*((ax7+(sf[208]*VE))+axg))),
            7,
            multiplicity * ((sf[53]*((ax8+(sf[208]*VF))+axh))),
        );
        stamper.stamp_current_reactive_node3_local(
            Some(7),
            Some(4),
            3,
            multiplicity * (axO),
            4,
            multiplicity * (axP),
            7,
            multiplicity * (axQ),
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(9),
            &[3, 5, 6, 7, 9],
            &[(sf[53]*(axq+(sf[208]*(if sb[49]{b}else{((qA*Hi)+(gD*((sf[189]*(if pY{((q3*Yf)+(q0*(q1*Y4)))}else{(if q5{Y8}else{(if pV{Vs}else{Vk})})}))+(sf[190]*(if qf{((qi*Yf)+(q0*(qg*Y4)))}else{(if qa{Yx}else{b})})))))})))), (sf[53]*(sf[208]*(if sb[49]{b}else{(gD*((sf[189]*(if pY{b}else{(if q5{b}else{(if pV{Vt}else{Vl})})}))+(sf[190]*(if qf{Yl}else{(if qa{Yy}else{b})}))))}))), (sf[53]*(axr+(sf[208]*(if sb[49]{b}else{(gD*(sf[189]*(if pY{Yk}else{(if q5{Y9}else{b})})))})))), (sf[53]*(sf[208]*(if sb[49]{b}else{(gD*((sf[189]*(if pY{b}else{(if q5{b}else{(if pV{Vu}else{Vm})})}))+(sf[190]*(if qf{Yk}else{(if qa{Yz}else{b})}))))}))), (sf[53]*(axs+(sf[208]*(if sb[49]{b}else{(gD*(sf[189]*(if pY{Yl}else{(if q5{Ya}else{b})})))}))))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(2),
            1,
            multiplicity * (sf[210]),
            2,
            multiplicity * (sf[280]),
        );
        stamper.stamp_current_reactive_node2_local(
            Some(1),
            Some(0),
            0,
            multiplicity * (sf[281]),
            1,
            multiplicity * (sf[211]),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(10),
            None,
            10,
            multiplicity * (sf[213]),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(11),
            None,
            11,
            multiplicity * (sf[282]),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(3),
            None,
            3,
            multiplicity * (sf[212]),
        );
    }
}
