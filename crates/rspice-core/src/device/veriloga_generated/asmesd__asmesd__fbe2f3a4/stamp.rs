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
    v1: f64, vc: f64, vd: f64, vi: f64, vj: f64, vk: f64, 
    vl: f64, vx: f64, vz: f64, v12: f64, v1u: f64, v21: f64, 
    v22: f64, v26: f64, v27: f64, v28: f64, v5g: f64, v5j: f64, 
    v5k: f64, v5l: f64, v5m: f64, v68: f64, v6m: f64, v6p: f64, 
    v7i: f64, v7l: f64, v7q: f64, v7s: f64, v8i: f64, v8p: f64, 
    v8x: f64, v9b: f64, va1: f64, vag: f64, vah: f64, vai: f64, 
    vap: f64, vav: f64, vbl: f64, vbp: f64, vbw: f64, vdi: f64, 
    vef: f64, veg: f64, veh: f64, vfm: f64, vg0: f64, vg7: f64, 
    vmn: f64, vmt: f64, vn2: f64, vo0: f64, vo2: f64, vo4: f64, 
    vo6: f64, vo8: f64, voa: f64, voc: f64, vod: f64, vos: f64, 
    vou: f64, vp9: f64, vvc: f64, vvg: f64, vvq: f64, vvr: f64, 
    vvs: f64, vx3: f64, vx4: f64, vx5: f64, vxx: f64, vxy: f64, 
    vy4: f64, vyr: f64, vys: f64, vyt: f64, vzf: f64, vzg: f64, 
    vzh: f64, v10i: f64, v10j: f64, v10k: f64, v10l: f64, v10m: f64, 
    v10n: f64, v10s: f64, v10t: f64, v10u: f64, v10v: f64, v11g: f64, 
    v11h: f64, v11i: f64, v11j: f64, v12a: f64, v12b: f64, v12c: f64, 
    v12d: f64, v13a: f64, v13b: f64, v13c: f64, v13d: f64, v17g: f64, 
    v17h: f64, v17i: f64, v17j: f64, v17m: f64, v17p: f64, v17s: f64, 
    v17t: f64, v17u: f64, v17x: f64, v180: f64, v1ky: f64, v1kz: f64, 
    v1l0: f64, v1l4: f64, v1l5: f64, v1l6: f64, v1lc: f64, v1ld: f64, 
    v1le: f64, v1lf: f64, v1lg: f64, v1lm: f64, v1ln: f64, v1lo: f64, 
    v1lp: f64, v1lq: f64, v1lv: f64, v1lw: f64, v1lx: f64, v1ly: f64, 
    v1m2: f64, v1m3: f64, v1m4: f64, v1m9: f64, v1ma: f64, v1mb: f64, 
    v1mc: f64, v1md: f64, v1me: f64, v1mf: f64, v1mg: f64, 
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let v1=ctx.node_voltage(nodes[3]);let v4=((ctx.temperature()+v1)+sf[0]);let v6=1300.0;let v7=173.14999999999998;let v8=(v4>v7);let v9=(if v8{v4}else{v7});let va=(v6<v9);let vb=(if va{v6}else{v9});let vc=1.0;let vd=0.0;let vi=ctx.node_voltage(nodes[5]);let vj=ctx.node_voltage(nodes[4]);let vk=(vi-vj);let vl=(sf[4]*vk);let vw=8.6170869e-5;let vx=(vb*vw);let vy=(vb/sf[8]);let vz=(vy).ln();let v12=((vz*sf[9])).exp();let v1r=(vy-vc);let v1s=(sf[25]*v1r);let v1u=((vz*sf[24])+(v1s/vx));let v1y=(v1u).exp();
        let v1z=(sf[27]*v1y);let v21=((vz*sf[26])).exp();let v22=(sf[28]*v21);let v26=((v1u/sf[30])).exp();let v27=(sf[29]*v26);let v28=(v27/v12);let v2j=(sf[33]*(vc+(v1r*sf[34])));let v2o=(sf[35]*(vc+(v1r*sf[36])));let v2t=(sf[37]*(vc+(v1r*sf[38])));let v2y=(sf[39]*(vc+(v1r*sf[40])));let v32=300.15;let v34=(vb/v32);let v36=0.000702;let v37=(vb*v36);let v38=(vb*v37);let v3a=(vb+1108.0);let v3d=(-(1.16-(v38/v3a)));let v3e=1.3806226e-23;let v3g=(v3e*(vb+vb));let v3l=(-(vx+vx));let v3m=1.5;
        let v3p=1.6021918e-19;let v3r=((v3m*(v34).ln())+(((v3d/v3g)+1.3454442398941469e20)*v3p));let v3s=(v3l*v3r);let v3v=((sf[45]-v3s)/sf[44]);let v3w=(sf[45]-v3v);let v3z=0.0004;let v44=(vc+(sf[46]*(sf[48]-(v3w/v3v))));let v45=(sf[41]/v44);let v47=(v3s+(v34*v3v));let v48=(v47-v3v);let v4b=(v3z*(vb-v32));let v4e=(vc+(sf[46]*(v4b-(v48/v3v))));let v4f=(v45*v4e);let v4i=((sf[49]-v3s)/sf[44]);let v4j=(sf[49]-v4i);let v4o=(vc+(sf[50]*(sf[48]-(v4j/v4i))));let v4p=(sf[42]/v4o);let v4r=(v3s+(v34*v4i));
        let v4s=(v4r-v4i);let v4w=(vc+(sf[50]*(v4b-(v4s/v4i))));let v4x=(v4p*v4w);let v50=((sf[51]-v3s)/sf[44]);let v51=(sf[51]-v50);let v56=(vc+(sf[52]*(sf[48]-(v51/v50))));let v57=(sf[43]/v56);let v59=(v3s+(v34*v50));let v5a=(v59-v50);let v5e=(vc+(sf[52]*(v4b-(v5a/v50))));let v5f=(v57*v5e);let v5g=ctx.node_voltage(nodes[2]);let v5i=(sf[4]*(v5g-vj));let v5j=ctx.node_voltage(nodes[6]);let v5k=(vi-v5j);let v5l=(sf[4]*v5k);let v5m=ctx.node_voltage(nodes[1]);let v5o=(sf[4]*(v5m-vj));
        let v5u=(if (v1z>vd){vc}else{vd});let v5w=(vx*sf[53]);let v5y=(if (v5u!=0.0){(v5l/v5w)}else{vd});let v5z=(-v5l);let v60=(v5z-v2o);let v62=(vx*sf[54]);let v64=(if (v5u!=0.0){(v60/v62)}else{vd});let v65=(-v2o);let v67=(if (v5u!=0.0){(v65/v62)}else{vd});let v68=80.0;let v6a=(if (v5y>v68){vc}else{vd});let v6b=((v5u!=0.0)&&(v6a!=0.0));let v6f=(if v6b{v68}else{v5y});let v6h=((v5u!=0.0)&&(!(v6a!=0.0)));let v6i=(if v6h{vc}else{(if v6b{(vc+(v5y-v68))}else{vd})});let v6j=(v6f).exp();
        let v6l=(if (v5u!=0.0){(v6i*v6j)}else{v6i});let v6m=37.0;let v6n=(v64>=v6m);let v6o=(!v6n);let v6p=-37.0;let v6q=(v64<=v6p);let v6s=(v6o&&(!v6q));let v6t=(v64).exp();let v6u=(vc+v6t);let v6w=(v6o&&v6q);let v70=(v67>=v6m);let v71=(!v70);let v72=(v67<=v6p);let v74=(v71&&(!v72));let v75=(v67).exp();let v76=(vc+v75);let v78=(v71&&v72);let v7d=(if (v5u!=0.0){((if v6s{(v6u).ln()}else{(if v6w{v6t}else{(if v6n{v64}else{vd})})})-(if v74{(v76).ln()}else{(if v78{v75}else{(if v70{v67}else{vd})})}))}else{vd});
        let v7e=(v6l-vc);let v7g=(v2j*v7d);let v7i=(v5l).abs();let v7j=f64::powf(v7i,v2t);let v7l=(vc+(sf[55]*v7j));let v7p=(!(v5u!=0.0));let v7q=(if v7p{vd}else{(if (v5u!=0.0){((v1z*v7e)-(v7g/v7l))}else{vd})});let v7s=(if (v22>vd){vc}else{vd});let v7u=(sf[56]-v5l);let v7v=0.001;let v7w=(v7u>v7v);let v7y=(if (v7s!=0.0){(if v7w{v7u}else{v7v})}else{vd});let v80=(v5z*sf[56]);let v82=(vx*sf[57]);let v83=(v7y*v82);let v85=(if (v7s!=0.0){(v80/v83)}else{v6f});let v87=(if (v85>v68){vc}else{vd});
        let v88=((v7s!=0.0)&&(v87!=0.0));let v8c=(if v88{v68}else{v85});let v8e=((v7s!=0.0)&&(!(v87!=0.0)));let v8f=(if v8e{vc}else{(if v88{(vc+(v85-v68))}else{v6l})});let v8g=(v8c).exp();let v8i=(if (v7s!=0.0){(v8f*v8g)}else{v8f});let v8p=(if (v28>vd){vc}else{vd});let v8q=(vx*sf[30]);let v8s=(if (v8p!=0.0){(v5l/v8q)}else{v8c});let v8u=(vx*sf[58]);let v8w=(if (v8p!=0.0){(v60/v8u)}else{v64});let v8x=(v65/v8u);let v8y=(if (v8p!=0.0){v8x}else{v67});let v90=(if (v8s>v68){vc}else{vd});
        let v91=((v8p!=0.0)&&(v90!=0.0));let v95=(if v91{v68}else{v8s});let v97=((v8p!=0.0)&&(!(v90!=0.0)));let v98=(if v97{vc}else{(if v91{(vc+(v8s-v68))}else{v8i})});let v99=(v95).exp();let v9b=(if (v8p!=0.0){(v98*v99)}else{v98});let v9c=(v8w>=v6m);let v9d=(!v9c);let v9e=(v8w<=v6p);let v9g=(v9d&&(!v9e));let v9h=(v8w).exp();let v9i=(vc+v9h);let v9k=(v9d&&v9e);let v9o=(v8y>=v6m);let v9p=(!v9o);let v9q=(v8y<=v6p);let v9s=(v9p&&(!v9q));let v9t=(v8y).exp();let v9u=(vc+v9t);let v9w=(v9p&&v9q);
        let va1=(if (v8p!=0.0){((if v9g{(v9i).ln()}else{(if v9k{v9h}else{(if v9c{v8w}else{vd})})})-(if v9s{(v9u).ln()}else{(if v9w{v9t}else{(if v9o{v8y}else{vd})})}))}else{v7d});let vab=(vx*sf[59]);let vad=(if (v5u!=0.0){(vl/vab)}else{v95});let vaf=((-vl)-v2o);let vag=(vaf/v8u);let vah=(if (v5u!=0.0){vag}else{v8w});let vai=(if (v5u!=0.0){v8x}else{v8y});let vak=(if (vad>v68){vc}else{vd});let val=((v5u!=0.0)&&(vak!=0.0));let vap=(if val{v68}else{vad});let var=((v5u!=0.0)&&(!(vak!=0.0)));
        let vas=(if var{vc}else{(if val{(vc+(vad-v68))}else{v9b})});let vat=(vap).exp();let vav=(if (v5u!=0.0){(vas*vat)}else{vas});let vaw=(vah>=v6m);let vax=(!vaw);let vay=(vah<=v6p);let vb0=(vax&&(!vay));let vb1=(vah).exp();let vb2=(vc+vb1);let vb4=(vax&&vay);let vb8=(vai>=v6m);let vb9=(!vb8);let vba=(vai<=v6p);let vbc=(vb9&&(!vba));let vbd=(vai).exp();let vbe=(vc+vbd);let vbg=(vb9&&vba);
        let vbl=(if (v5u!=0.0){((if vb0{(vb2).ln()}else{(if vb4{vb1}else{(if vaw{vah}else{vd})})})-(if vbc{(vbe).ln()}else{(if vbg{vbd}else{(if vb8{vai}else{vd})})}))}else{va1});let vbm=(vav-vc);let vbo=(v2y*vbl);let vbp=(vl).abs();let vbq=f64::powf(vbp,v2t);let vbs=(vc+(sf[55]*vbq));let vbw=(if v7p{vd}else{(if (v5u!=0.0){((v1z*vbm)-(vbo/vbs))}else{vd})});let vdi=ctx.node_voltage(nodes[9]);let vec=(vc+f64::powf(((vc+(((v7q*(sf[20]*(vc+(vl*sf[60]))))+(sf[23]*vbw))*4.0))).abs(),sf[61]));
        let vef=((((vc-(sf[17]*v5l))-(vl*sf[14]))*2.0)/vec);let veg=(vbw*vef);let veh=(v7q*vef);let vfm=(v5m-v5g);let vg0=(sf[79]*(vc+((f64::powf((vc+f64::powf(((vfm/sf[76])).abs(),sf[77])),sf[78])-vc)*sf[80])));let vg7=ctx.node_voltage(nodes[8]);let vgt=(if (v5i<=vd){vc}else{vd});let vgu=(v59*v5f);let vgx=(vc-(v5i/v59));let vh0=((sf[91]*(vgx).ln())).exp();let vh1=(vc-vh0);let vh5=(!(vgt!=0.0));let vh6=(v5f*v5i);let vh9=(v5i*sf[92]);let vhb=(vc+(vh9/v59));let vhh=(v5l+((-v47)*sf[93]));
        let vhj=(if (vhh>vd){vc}else{vd});let vhp=(if (vhj!=0.0){sf[98]}else{vd});let vhs=(vc-(sf[95]*(sf[95]*vhp)));let vhy=(vhh*sf[100]);let vi0=(sf[95]+(vhy/v47));let vi4=(!(vhj!=0.0));let vi6=(vc-(v5l/v47));let vi9=((sf[99]*(vi6).ln())).exp();let via=(vc-vi9);let vid=(if vi4{((v47*via)/sf[99])}else{(if (vhj!=0.0){((v47*vhs)/sf[99])}else{vd})});let vie=(if vi4{vd}else{(if (vhj!=0.0){(vhp*(vhh*vi0))}else{vd})});let vif=(vid+vie);let vii=(sf[93]*(-v4r));let vij=(v5o+vii);let vil=(if (vij>vd){vc}else{vd});
        let vip=(if (vil!=0.0){sf[103]}else{vhp});let vis=(vc-(sf[95]*(sf[95]*vip)));let viy=(vij*sf[105]);let vj0=(sf[95]+(viy/v4r));let vj4=(!(vil!=0.0));let vj6=(vc-(v5o/v4r));let vj9=((sf[104]*(vj6).ln())).exp();let vja=(vc-vj9);let vjd=(if vj4{((v4r*vja)/sf[104])}else{(if (vil!=0.0){((v4r*vis)/sf[104])}else{vid})});let vje=(if vj4{vd}else{(if (vil!=0.0){(vip*(vij*vj0))}else{vie})});let vjf=(vjd+vje);let vjk=(vl+vii);let vjm=(if (vjk>vd){vc}else{vd});let vjn=(if (vjm!=0.0){sf[103]}else{vip});
        let vjq=(vc-(sf[95]*(sf[95]*vjn)));let vju=(sf[105]*vjk);let vjw=(sf[95]+(vju/v4r));let vk0=(!(vjm!=0.0));let vk2=(vc-(vl/v4r));let vk5=((sf[104]*(vk2).ln())).exp();let vk6=(vc-vk5);let vkb=((if vk0{((v4r*vk6)/sf[104])}else{(if (vjm!=0.0){((v4r*vjq)/sf[104])}else{vjd})})+(if vk0{vd}else{(if (vjm!=0.0){(vjn*(vjk*vjw))}else{vje})}));let vks=(if sb[9]{vd}else{(if (sf[109]!=0.0){(veh*sf[113])}else{vd})});let vmn=(v1*sf[134]);let vmt=ctx.node_voltage(nodes[7]);let vn2=(vmt*sf[135]);
        let vo0=(sf[3]*(sf[4]*(v4f*vif)));let vo2=(sf[3]*(sf[4]*(v7q*vg0)));let vo4=(sf[3]*(sf[4]*((v4x*vjf)*sf[107])));let vo6=(sf[3]*(sf[4]*(sf[106]*(v4x*vkb))));let vo8=(sf[3]*(sf[4]*(veg*sf[81])));let voa=(sf[3]*(sf[4]*(if vh5{(vh6*vhb)}else{(if (vgt!=0.0){((vgu*vh1)/sf[91])}else{vd})})));let voc=(sf[3]*(-vks));let vod=(sf[3]*vks);let vof=(if va{vd}else{(if v8{vc}else{vd})});let vos=(vw*vof);let vot=(vof/sf[8]);let vou=(vot/vy);let vp9=((sf[24]*vou)+(((vx*(sf[25]*vot))-(v1s*vos))/(vx*vx)));
        let vpc=(sf[27]*(v1y*vp9));let vpz=(sf[37]*(sf[38]*vot));let vq2=(vof/v32);let vqr=((v3r*(-(vos+vos)))+(v3l*((v3m*(vq2/v34))+(v3p*(((v3g*(((v3a*((v37*vof)+(vb*(v36*vof))))-(v38*vof))/(v3a*v3a)))-(v3d*(v3e*(vof+vof))))/(v3g*v3g))))));let vqt=((-vqr)/sf[44]);let vqu=(-vqt);let vqy=(v3v*v3v);let vr7=(v34*vqt);let vr9=(vqr+((v3v*vq2)+vr7));let vrf=(v3z*vof);let vro=(v4i*v4i);let vry=(vqr+(vr7+(v4i*vq2)));
        let vs8=((v4w*((-(sf[42]*(sf[50]*(-(((v4i*vqu)-(v4j*vqt))/vro)))))/(v4o*v4o)))+(v4p*(sf[50]*(vrf-(((v4i*(vry-vqt))-(v4s*vqt))/vro)))));let vsc=(v50*v50);let vsm=(vqr+(vr7+(v50*vq2)));let vsw=((v5e*((-(sf[43]*(sf[52]*(-(((v50*vqu)-(v51*vqt))/vsc)))))/(v56*v56)))+(v57*(sf[52]*(vrf-(((v50*(vsm-vqt))-(v5a*vqt))/vsc)))));let vt4=(if (v5u!=0.0){((-(v5l*(sf[53]*vos)))/(v5w*v5w))}else{vd});let vt5=(if (v5u!=0.0){(sf[4]/v5w)}else{vd});let vt6=(if (v5u!=0.0){(sf[136]/v5w)}else{vd});
        let vt7=(-(sf[35]*(sf[36]*vot)));let vt8=(sf[54]*vos);let vt9=(v62*vt7);let vtc=(v62*v62);let vtg=(if (v5u!=0.0){((vt9-(v60*vt8))/vtc)}else{vd});let vth=(if (v5u!=0.0){(sf[136]/v62)}else{vd});let vti=(if (v5u!=0.0){(sf[4]/v62)}else{vd});let vtm=(if (v5u!=0.0){((vt9-(v65*vt8))/vtc)}else{vd});let vtq=(if v6b{vd}else{vt4});let vtr=(if v6b{vd}else{vt5});let vts=(if v6b{vd}else{vt6});let vtt=(if v6h{vd}else{(if v6b{vt4}else{vd})});let vtu=(if v6h{vd}else{(if v6b{vt5}else{vd})});
        let vtv=(if v6h{vd}else{(if v6b{vt6}else{vd})});let vu8=(if (v5u!=0.0){((v6j*vtt)+(v6i*(v6j*vtq)))}else{vtt});let vu9=(if (v5u!=0.0){((v6j*vtu)+(v6i*(v6j*vtr)))}else{vtu});let vua=(if (v5u!=0.0){((v6j*vtv)+(v6i*(v6j*vts)))}else{vtv});let vub=(v6t*vtg);let vuc=(v6t*vth);let vud=(v6t*vti);let vuq=(v75*vtm);let vuw=(if (v5u!=0.0){((if v6s{(vub/v6u)}else{(if v6w{vub}else{(if v6n{vtg}else{vd})})})-(if v74{(vuq/v76)}else{(if v78{vuq}else{(if v70{vtm}else{vd})})}))}else{vd});
        let vux=(if (v5u!=0.0){(if v6s{(vuc/v6u)}else{(if v6w{vuc}else{(if v6n{vth}else{vd})})})}else{vd});let vuy=(if (v5u!=0.0){(if v6s{(vud/v6u)}else{(if v6w{vud}else{(if v6n{vti}else{vd})})})}else{vd});let vvc=(sf[55]*(vpz*(v7j*(v7i).ln())));let vvg=(v7l*v7l);let vvq=(if v7p{vd}else{(if (v5u!=0.0){(((v7e*vpc)+(v1z*vu8))-(((v7l*((v7d*(sf[33]*(sf[34]*vot)))+(v2j*vuw)))-(v7g*vvc))/vvg))}else{vd})});let vvr=(if v7p{vd}else{(if (v5u!=0.0){((v1z*vu9)-((v2j*vux)/v7l))}else{vd})});
        let vvs=(if v7p{vd}else{(if (v5u!=0.0){((v1z*vua)-((v2j*vuy)/v7l))}else{vd})});let vw5=(v83*v83);let vwf=(if (v7s!=0.0){((-(v80*(v7y*(sf[57]*vos))))/vw5)}else{vtq});let vwg=(if (v7s!=0.0){(((v83*sf[138])-(v80*(v82*(if (v7s!=0.0){(if v7w{sf[136]}else{vd})}else{vd}))))/vw5)}else{vtr});let vwh=(if (v7s!=0.0){(((v83*sf[139])-(v80*(v82*(if (v7s!=0.0){(if v7w{sf[4]}else{vd})}else{vd}))))/vw5)}else{vts});let vwl=(if v88{vd}else{vwf});let vwm=(if v88{vd}else{vwg});let vwn=(if v88{vd}else{vwh});
        let vwo=(if v8e{vd}else{(if v88{vwf}else{vu8})});let vwp=(if v8e{vd}else{(if v88{vwg}else{vu9})});let vwq=(if v8e{vd}else{(if v88{vwh}else{vua})});let vx3=(if (v7s!=0.0){((v8g*vwo)+(v8f*(v8g*vwl)))}else{vwo});let vx4=(if (v7s!=0.0){((v8g*vwp)+(v8f*(v8g*vwm)))}else{vwp});let vx5=(if (v7s!=0.0){((v8g*vwq)+(v8f*(v8g*vwn)))}else{vwq});let vxo=(if (v8p!=0.0){((-(v5l*(sf[30]*vos)))/(v8q*v8q))}else{vwl});let vxp=(if (v8p!=0.0){(sf[4]/v8q)}else{vwm});let vxq=(if (v8p!=0.0){(sf[136]/v8q)}else{vwn});
        let vxr=(sf[58]*vos);let vxs=(v8u*vt7);let vxv=(v8u*v8u);let vxx=(sf[136]/v8u);let vxy=(sf[4]/v8u);let vxz=(if (v8p!=0.0){((vxs-(v60*vxr))/vxv)}else{vtg});let vy0=(if (v8p!=0.0){vxx}else{vth});let vy1=(if (v8p!=0.0){vxy}else{vti});let vy4=((vxs-(v65*vxr))/vxv);let vy5=(if (v8p!=0.0){vy4}else{vtm});let vy9=(if v91{vd}else{vxo});let vya=(if v91{vd}else{vxp});let vyb=(if v91{vd}else{vxq});let vyc=(if v97{vd}else{(if v91{vxo}else{vx3})});let vyd=(if v97{vd}else{(if v91{vxp}else{vx4})});
        let vye=(if v97{vd}else{(if v91{vxq}else{vx5})});let vyr=(if (v8p!=0.0){((v99*vyc)+(v98*(v99*vy9)))}else{vyc});let vys=(if (v8p!=0.0){((v99*vyd)+(v98*(v99*vya)))}else{vyd});let vyt=(if (v8p!=0.0){((v99*vye)+(v98*(v99*vyb)))}else{vye});let vyu=(v9h*vxz);let vyv=(v9h*vy0);let vyw=(v9h*vy1);let vz9=(v9t*vy5);let vzf=(if (v8p!=0.0){((if v9g{(vyu/v9i)}else{(if v9k{vyu}else{(if v9c{vxz}else{vd})})})-(if v9s{(vz9/v9u)}else{(if v9w{vz9}else{(if v9o{vy5}else{vd})})}))}else{vuw});
        let vzg=(if (v8p!=0.0){(if v9g{(vyv/v9i)}else{(if v9k{vyv}else{(if v9c{vy0}else{vd})})})}else{vux});let vzh=(if (v8p!=0.0){(if v9g{(vyw/v9i)}else{(if v9k{vyw}else{(if v9c{vy1}else{vd})})})}else{vuy});let v10c=(if (v5u!=0.0){((-(vl*(sf[59]*vos)))/(vab*vab))}else{vy9});let v10d=(if (v5u!=0.0){(sf[136]/vab)}else{vd});let v10e=(if (v5u!=0.0){(sf[4]/vab)}else{vya});let v10f=(if (v5u!=0.0){vd}else{vyb});let v10i=((vxs-(vaf*vxr))/vxv);let v10j=(if (v5u!=0.0){v10i}else{vxz});
        let v10k=(if (v5u!=0.0){vxy}else{vd});let v10l=(if (v5u!=0.0){vxx}else{vy0});let v10m=(if (v5u!=0.0){vd}else{vy1});let v10n=(if (v5u!=0.0){vy4}else{vy5});let v10s=(if val{vd}else{v10c});let v10t=(if val{vd}else{v10d});let v10u=(if val{vd}else{v10e});let v10v=(if val{vd}else{v10f});let v10w=(if var{vd}else{(if val{v10c}else{vyr})});let v10x=(if var{vd}else{(if val{v10d}else{vd})});let v10y=(if var{vd}else{(if val{v10e}else{vys})});let v10z=(if var{vd}else{(if val{v10f}else{vyt})});
        let v11g=(if (v5u!=0.0){((vat*v10w)+(vas*(vat*v10s)))}else{v10w});let v11h=(if (v5u!=0.0){((vat*v10x)+(vas*(vat*v10t)))}else{v10x});let v11i=(if (v5u!=0.0){((vat*v10y)+(vas*(vat*v10u)))}else{v10y});let v11j=(if (v5u!=0.0){((vat*v10z)+(vas*(vat*v10v)))}else{v10z});let v11k=(vb1*v10j);let v11l=(vb1*v10k);let v11m=(vb1*v10l);let v11n=(vb1*v10m);let v124=(vbd*v10n);
        let v12a=(if (v5u!=0.0){((if vb0{(v11k/vb2)}else{(if vb4{v11k}else{(if vaw{v10j}else{vd})})})-(if vbc{(v124/vbe)}else{(if vbg{v124}else{(if vb8{v10n}else{vd})})}))}else{vzf});let v12b=(if (v5u!=0.0){(if vb0{(v11l/vb2)}else{(if vb4{v11l}else{(if vaw{v10k}else{vd})})})}else{vd});let v12c=(if (v5u!=0.0){(if vb0{(v11m/vb2)}else{(if vb4{v11m}else{(if vaw{v10l}else{vd})})})}else{vzg});let v12d=(if (v5u!=0.0){(if vb0{(v11n/vb2)}else{(if vb4{v11n}else{(if vaw{v10m}else{vd})})})}else{vzh});
        let v13a=(if v7p{vd}else{(if (v5u!=0.0){(((vbm*vpc)+(v1z*v11g))-(((vbs*((vbl*(sf[39]*(sf[40]*vot)))+(v2y*v12a)))-(vbo*(sf[55]*(vpz*(vbq*(vbp).ln())))))/(vbs*vbs)))}else{vd})});let v13b=(if v7p{vd}else{(if (v5u!=0.0){((v1z*v11h)-((v2y*v12b)/vbs))}else{vd})});let v13c=(if v7p{vd}else{(if (v5u!=0.0){((v1z*v11i)-((v2y*v12c)/vbs))}else{vd})});let v13d=(if v7p{vd}else{(if (v5u!=0.0){((v1z*v11j)-((v2y*v12d)/vbs))}else{vd})});let v17g=(sf[148]/vec);let v17h=(sf[149]/vec);let v17i=(sf[150]/vec);
        let v17j=(vef*v13a);let v17m=((vef*v13b)+(vbw*v17g));let v17p=((vef*v13c)+(vbw*v17h));let v17s=((vef*v13d)+(vbw*v17i));let v17t=(vef*vvq);let v17u=(v7q*v17g);let v17x=((vef*vvr)+(v7q*v17h));let v180=((vef*vvs)+(v7q*v17i));let v19e=(v59*v59);let v1au=(sf[93]*(-vr9));let v1b4=(v47*v47);let v1cf=(if vi4{(((via*vr9)+(v47*(-(vi9*(sf[99]*((-((-(v5l*vr9))/v1b4))/vi6))))))/sf[99])}else{(if (vhj!=0.0){((vhs*vr9)/sf[99])}else{vd})});
        let v1cg=(if vi4{((v47*(-(vi9*(sf[99]*((-(sf[4]/v47))/vi6)))))/sf[99])}else{vd});let v1ch=(if vi4{((v47*(-(vi9*(sf[99]*((-(sf[136]/v47))/vi6)))))/sf[99])}else{vd});let v1ci=(if vi4{vd}else{(if (vhj!=0.0){(vhp*((vi0*v1au)+(vhh*(((v47*(sf[100]*v1au))-(vhy*vr9))/v1b4))))}else{vd})});let v1cj=(if vi4{vd}else{(if (vhj!=0.0){(vhp*((sf[4]*vi0)+(vhh*(sf[153]/v47))))}else{vd})});let v1ck=(if vi4{vd}else{(if (vhj!=0.0){(vhp*((vi0*sf[136])+(vhh*(sf[154]/v47))))}else{vd})});let v1cu=(sf[93]*(-vry));
        let v1d3=(sf[155]/v4r);let v1d4=(v4r*(sf[105]*v1cu));let v1d7=(v4r*v4r);let v1d9=(sf[156]/v4r);let v1dw=(-(sf[4]/v4r));let v1dy=(-(sf[136]/v4r));let v1ej=(if vj4{((v4r*(-(vj9*(sf[104]*(v1dw/vj6)))))/sf[104])}else{vd});let v1ek=(if vj4{(((vja*vry)+(v4r*(-(vj9*(sf[104]*((-((-(v5o*vry))/v1d7))/vj6))))))/sf[104])}else{(if (vil!=0.0){((vis*vry)/sf[104])}else{v1cf})});let v1el=(if vj4{((v4r*(-(vj9*(sf[104]*(v1dy/vj6)))))/sf[104])}else{vd});let v1em=(if vj4{vd}else{(if (vil!=0.0){vd}else{v1cg})});
        let v1en=(if vj4{vd}else{(if (vil!=0.0){vd}else{v1ch})});let v1eo=(if vj4{vd}else{(if (vil!=0.0){(vip*((sf[4]*vj0)+(vij*v1d3)))}else{vd})});let v1ep=(if vj4{vd}else{(if (vil!=0.0){(vip*((vj0*v1cu)+(vij*((v1d4-(viy*vry))/v1d7))))}else{v1ci})});let v1eq=(if vj4{vd}else{(if (vil!=0.0){(vip*((vj0*sf[136])+(vij*v1d9)))}else{vd})});let v1er=(if vj4{vd}else{(if (vil!=0.0){vd}else{v1cj})});let v1es=(if vj4{vd}else{(if (vil!=0.0){vd}else{v1ck})});
        let v1ho=(if sb[9]{vd}else{(if (sf[109]!=0.0){(sf[113]*v17t)}else{vd})});let v1hp=(if sb[9]{vd}else{(if (sf[109]!=0.0){(sf[113]*v17u)}else{vd})});let v1hq=(if sb[9]{vd}else{(if (sf[109]!=0.0){(sf[113]*v17x)}else{vd})});let v1hr=(if sb[9]{vd}else{(if (sf[109]!=0.0){(sf[113]*v180)}else{vd})});let v1ky=(sf[3]*(sf[4]*((vif*((v4e*((-(sf[41]*(sf[46]*(-(((v3v*vqu)-(v3w*vqt))/vqy)))))/(v44*v44)))+(v45*(sf[46]*(vrf-(((v3v*(vr9-vqt))-(v48*vqt))/vqy))))))+(v4f*(v1cf+v1ci)))));
        let v1kz=(sf[3]*(sf[4]*(v4f*(v1cg+v1cj))));let v1l0=(sf[3]*(sf[4]*(v4f*(v1ch+v1ck))));let v1l4=(sf[3]*(sf[4]*(vg0*vvq)));let v1l5=(sf[3]*(sf[4]*(vg0*vvr)));let v1l6=(sf[3]*(sf[4]*(vg0*vvs)));let v1lc=(sf[3]*(sf[4]*(sf[107]*(v4x*(v1ej+v1eo)))));let v1ld=(sf[3]*(sf[4]*(sf[107]*((vjf*vs8)+(v4x*(v1ek+v1ep))))));let v1le=(sf[3]*(sf[4]*(sf[107]*(v4x*(v1el+v1eq)))));let v1lf=(sf[3]*(sf[4]*(sf[107]*(v4x*(v1em+v1er)))));let v1lg=(sf[3]*(sf[4]*(sf[107]*(v4x*(v1en+v1es)))));
        let v1lm=(sf[3]*(sf[4]*(sf[106]*(v4x*((if vk0{vd}else{(if (vjm!=0.0){vd}else{v1ej})})+(if vk0{vd}else{(if (vjm!=0.0){vd}else{v1eo})}))))));let v1ln=(sf[3]*(sf[4]*(sf[106]*((vkb*vs8)+(v4x*((if vk0{(((vk6*vry)+(v4r*(-(vk5*(sf[104]*((-((-(vl*vry))/v1d7))/vk2))))))/sf[104])}else{(if (vjm!=0.0){((vjq*vry)/sf[104])}else{v1ek})})+(if vk0{vd}else{(if (vjm!=0.0){(vjn*((vjw*v1cu)+(vjk*((v1d4-(vju*vry))/v1d7))))}else{v1ep})})))))));
        let v1lo=(sf[3]*(sf[4]*(sf[106]*(v4x*((if vk0{((v4r*(-(vk5*(sf[104]*(v1dy/vk2)))))/sf[104])}else{(if (vjm!=0.0){vd}else{v1el})})+(if vk0{vd}else{(if (vjm!=0.0){(vjn*((vjw*sf[136])+(vjk*v1d9)))}else{v1eq})}))))));let v1lp=(sf[3]*(sf[4]*(sf[106]*(v4x*((if vk0{((v4r*(-(vk5*(sf[104]*(v1dw/vk2)))))/sf[104])}else{(if (vjm!=0.0){vd}else{v1em})})+(if vk0{vd}else{(if (vjm!=0.0){(vjn*((sf[4]*vjw)+(vjk*v1d3)))}else{v1er})}))))));
        let v1lq=(sf[3]*(sf[4]*(sf[106]*(v4x*((if vk0{vd}else{(if (vjm!=0.0){vd}else{v1en})})+(if vk0{vd}else{(if (vjm!=0.0){vd}else{v1es})}))))));let v1lv=(sf[3]*(sf[4]*(sf[81]*v17j)));let v1lw=(sf[3]*(sf[4]*(sf[81]*v17m)));let v1lx=(sf[3]*(sf[4]*(sf[81]*v17p)));let v1ly=(sf[3]*(sf[4]*(sf[81]*v17s)));let v1m2=(sf[3]*(sf[4]*(if vh5{((vhb*(sf[4]*v5f))+(vh6*(sf[151]/v59)))}else{(if (vgt!=0.0){((vgu*(-(vh0*(sf[91]*((-(sf[4]/v59))/vgx)))))/sf[91])}else{vd})})));
        let v1m3=(sf[3]*(sf[4]*(if vh5{((vhb*(v5i*vsw))+(vh6*((-(vh9*vsm))/v19e)))}else{(if (vgt!=0.0){(((vh1*((v5f*vsm)+(v59*vsw)))+(vgu*(-(vh0*(sf[91]*((-((-(v5i*vsm))/v19e))/vgx))))))/sf[91])}else{vd})})));let v1m4=(sf[3]*(sf[4]*(if vh5{((vhb*(v5f*sf[136]))+(vh6*(sf[152]/v59)))}else{(if (vgt!=0.0){((vgu*(-(vh0*(sf[91]*((-(sf[136]/v59))/vgx)))))/sf[91])}else{vd})})));let v1m9=(sf[3]*(-v1ho));let v1ma=(sf[3]*(-v1hp));let v1mb=(sf[3]*(-v1hq));let v1mc=(sf[3]*(-v1hr));let v1md=(sf[3]*v1ho);
        let v1me=(sf[3]*v1hp);let v1mf=(sf[3]*v1hq);let v1mg=(sf[3]*v1hr);

        CommonStampValues {
            v1, vc, vd, vi, vj, vk, vl, vx, 
            vz, v12, v1u, v21, v22, v26, v27, v28, 
            v5g, v5j, v5k, v5l, v5m, v68, v6m, v6p, 
            v7i, v7l, v7q, v7s, v8i, v8p, v8x, v9b, 
            va1, vag, vah, vai, vap, vav, vbl, vbp, 
            vbw, vdi, vef, veg, veh, vfm, vg0, vg7, 
            vmn, vmt, vn2, vo0, vo2, vo4, vo6, vo8, 
            voa, voc, vod, vos, vou, vp9, vvc, vvg, 
            vvq, vvr, vvs, vx3, vx4, vx5, vxx, vxy, 
            vy4, vyr, vys, vyt, vzf, vzg, vzh, v10i, 
            v10j, v10k, v10l, v10m, v10n, v10s, v10t, v10u, 
            v10v, v11g, v11h, v11i, v11j, v12a, v12b, v12c, 
            v12d, v13a, v13b, v13c, v13d, v17g, v17h, v17i, 
            v17j, v17m, v17p, v17s, v17t, v17u, v17x, v180, 
            v1ky, v1kz, v1l0, v1l4, v1l5, v1l6, v1lc, v1ld, 
            v1le, v1lf, v1lg, v1lm, v1ln, v1lo, v1lp, v1lq, 
            v1lv, v1lw, v1lx, v1ly, v1m2, v1m3, v1m4, v1m9, 
            v1ma, v1mb, v1mc, v1md, v1me, v1mf, v1mg, 
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let nodes = self.nodes;
        let multiplicity = self.multiplicity;
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
            v1, vc, vd, vi, vj, vk, vl, vx, 
            vz, v12, v1u, v21, v22, v26, v27, v28, 
            v5g, v5j, v5k, v5l, v5m, v68, v6m, v6p, 
            v7i, v7l, v7q, v7s, v8i, v8p, v8x, v9b, 
            va1, vag, vah, vai, vap, vav, vbl, vbp, 
            vbw, vdi, vef, veg, veh, vfm, vg0, vg7, 
            vmn, vmt, vn2, vo0, vo2, vo4, vo6, vo8, 
            voa, voc, vod, vos, vou, vp9, vvc, vvg, 
            vvq, vvr, vvs, vx3, vx4, vx5, vxx, vxy, 
            vy4, vyr, vys, vyt, vzf, vzg, vzh, v10i, 
            v10j, v10k, v10l, v10m, v10n, v10s, v10t, v10u, 
            v10v, v11g, v11h, v11i, v11j, v12a, v12b, v12c, 
            v12d, v13a, v13b, v13c, v13d, v17g, v17h, v17i, 
            v17j, v17m, v17p, v17s, v17t, v17u, v17x, v180, 
            v1ky, v1kz, v1l0, v1l4, v1l5, v1l6, v1lc, v1ld, 
            v1le, v1lf, v1lg, v1lm, v1ln, v1lo, v1lp, v1lq, 
            v1lv, v1lw, v1lx, v1ly, v1m2, v1m3, v1m4, v1m9, 
            v1ma, v1mb, v1mc, v1md, v1me, v1mf, v1mg, 
        }=self.eval_common_stamp_values(ctx);
        let vn=(vl<vd);let vp=(-(if vn{vl}else{vd}));let vt=(vc+(sf[5]*f64::powf(vp,sf[6])));let v14=(v12*sf[10]);let v15=(vt*v14);let v17=(v12*sf[11]);let v2c=((v1u/sf[32])).exp();let v2d=(sf[31]*v2c);let v2e=(v2d/v12);let v5p=(v5m-vi);let v5r=(v5g-v5j);let v7z=-1.0;let v8j=(v8i-vc);let v8m=(!(v7s!=0.0));let va2=(v9b-vc);let va4=(vd*va1);let va8=(!(v8p!=0.0));let vby=(if (v2e>vd){vc}else{vd});let vbz=(vx*sf[32]);let vc1=(if (vby!=0.0){(vl/vbz)}else{vap});let vc2=(if (vby!=0.0){vag}else{vah});
        let vc3=(if (vby!=0.0){v8x}else{vai});let vc5=(if (vc1>v68){vc}else{vd});let vc6=((vby!=0.0)&&(vc5!=0.0));let vcc=((vby!=0.0)&&(!(vc5!=0.0)));let vcd=(if vcc{vc}else{(if vc6{(vc+(vc1-v68))}else{vav})});let vce=((if vc6{v68}else{vc1})).exp();let vch=(vc2>=v6m);let vci=(!vch);let vcj=(vc2<=v6p);let vcl=(vci&&(!vcj));let vcm=(vc2).exp();let vcn=(vc+vcm);let vcp=(vci&&vcj);let vct=(vc3>=v6m);let vcu=(!vct);let vcv=(vc3<=v6p);let vcx=(vcu&&(!vcv));let vcy=(vc3).exp();let vcz=(vc+vcy);let vd1=(vcu&&vcv);
        let vd7=((if (vby!=0.0){(vcd*vce)}else{vcd})-vc);let vdc=(vc+(sf[55]*f64::powf(vbp,sf[37])));let vdg=(!(vby!=0.0));let vdl=1e-9;let vdp=(((if (vdi<v5l){vdi}else{v5l})/(if (v7i>vdl){v7i}else{vdl}))).abs();let vdq=(v7q-(if v8m{vd}else{(if (v7s!=0.0){(v22*v8j)}else{vd})}));let vds=((if va8{vd}else{(if (v8p!=0.0){((v28*va2)-(va4/v7l))}else{vd})})+(vdq/v15));
        let vdu=((if vdg{vd}else{(if (vby!=0.0){((v2e*vd7)-((vd*(if (vby!=0.0){((if vcl{(vcn).ln()}else{(if vcp{vcm}else{(if vch{vc2}else{vd})})})-(if vcx{(vcz).ln()}else{(if vd1{vcy}else{(if vct{vc3}else{vd})})}))}else{vbl}))/vdc))}else{vd})})+(vbw/v17));let vem=(v7q*sf[63]);let vf4=((vz*sf[69])).exp();let vf7=f64::powf((vc+f64::powf((((sf[4]*v5p)/sf[64])).abs(),sf[65])),sf[70]);let vf8=((sf[68]*vf4)*vf7);let vfc=((vz*sf[72])).exp();let vfd=(sf[71]*vfc);let vfh=((vz*sf[74])).exp();
        let vfk=f64::powf((vc+f64::powf((((sf[4]*v5r)/sf[66])).abs(),sf[67])),sf[75]);let vfl=((sf[73]*vfh)*vfk);let vgd=(vc+f64::powf(((vg7).abs()/sf[84]),sf[85]));let vgf=(if (sf[83]!=0.0){(vf8/vgd)}else{vf8});let vll=((if (sf[87]!=0.0){(vgf+sf[88])}else{vgf})/sf[3]);let vlq=((if (sf[87]!=0.0){(vfl+sf[90])}else{vfl})/sf[3]);let vlv=((if (sf[87]!=0.0){(vfd+sf[89])}else{vfd})/sf[3]);let vly=1e-6;
        let vm1=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, vdi);
        let vm8=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, vg7);let vme=ctx.node_voltage(nodes[0]);let vmi=((-((vds*vfm)).abs())-((vdu*(v5m-vme))).abs());
        let vmo=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, vmn);
        let vmx=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, vmn);
        let vn3=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, vn2);let vnd=(vll>sf[129]);let vne=(if vnd{vll}else{sf[129]});let vnh=(vlq>sf[129]);let vni=(if vnh{vlq}else{sf[129]});let vnl=(vme-vj);let vnm=(vlv>sf[129]);let vnn=(if vnm{vlv}else{sf[129]});let von=(sf[6]*f64::powf(vp,sf[137]));
        let vow=(v12*(sf[9]*vou));let voy=(vt*(sf[10]*vow));let voz=(v14*(sf[5]*((-(if vn{sf[136]}else{vd}))*von)));let vp0=(v14*(sf[5]*((-(if vn{sf[4]}else{vd}))*von)));let vpl=(v12*v12);let v13l=(if (vby!=0.0){((-(vl*(sf[32]*vos)))/(vbz*vbz))}else{v10s});let v13m=(if (vby!=0.0){(sf[136]/vbz)}else{v10t});let v13n=(if (vby!=0.0){(sf[4]/vbz)}else{v10u});let v13o=(if (vby!=0.0){vd}else{v10v});let v13p=(if (vby!=0.0){v10i}else{v10j});let v13q=(if (vby!=0.0){vxy}else{v10k});
        let v13r=(if (vby!=0.0){vxx}else{v10l});let v13s=(if (vby!=0.0){vd}else{v10m});let v13t=(if (vby!=0.0){vy4}else{v10n});let v142=(if vcc{vd}else{(if vc6{v13l}else{v11g})});let v143=(if vcc{vd}else{(if vc6{v13m}else{v11h})});let v144=(if vcc{vd}else{(if vc6{v13n}else{v11i})});let v145=(if vcc{vd}else{(if vc6{v13o}else{v11j})});let v14q=(vcm*v13p);let v14r=(vcm*v13q);let v14s=(vcm*v13r);let v14t=(vcm*v13s);let v15a=(vcy*v13t);let v16g=(v15*v15);let v18r=(vf7*(sf[68]*(vf4*(sf[69]*vou))));
        let v1hv=ddt_scale;let v1iq=(sf[134]*v1hv);let v1j0=-0.0;

        stamper.stamp_current_node3_local(
            Some(9),
            None,
            multiplicity * ((-(v5l-vdi))),
            5,
            multiplicity * (sf[136]),
            6,
            multiplicity * (sf[4]),
            9,
            multiplicity * (vc),
        );
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * ((vdi*vly)),
            9,
            multiplicity * (vly),
        );
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * ((sf[133]*vm1)),
            9,
            multiplicity * ((sf[133]*v1hv)),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            None,
            multiplicity * ((if (sf[83]!=0.0){(vg0*(-(v7q/v15)))}else{vd})),
            [3, 4, 5, 6],
            [(if (sf[83]!=0.0){(vg0*(-(((v15*vvq)-(v7q*voy))/v16g)))}else{vd}), (if (sf[83]!=0.0){(vg0*(-((-(v7q*voz))/v16g)))}else{vd}), (if (sf[83]!=0.0){(vg0*(-(((v15*vvr)-(v7q*vp0))/v16g)))}else{vd}), (if (sf[83]!=0.0){(vg0*(-(vvs/v15)))}else{vd})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * ((if (sf[83]!=0.0){vg7}else{vd})),
            8,
            multiplicity * (sf[157]),
        );
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * ((if (sf[83]!=0.0){(vg0*vm8)}else{vd})),
            8,
            multiplicity * ((if (sf[83]!=0.0){(vg0*v1hv)}else{vd})),
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            vd,
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * ((if (sf[116]!=0.0){vmi}else{vd})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if (sf[116]!=0.0){(v1/sf[115])}else{vd})),
            3,
            multiplicity * (sf[159]),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if (sf[116]!=0.0){vmo}else{vd})),
            3,
            multiplicity * ((if (sf[116]!=0.0){v1iq}else{vd})),
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            vd,
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * ((if sb[28]{vmi}else{vd})),
        );
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * ((if sb[28]{((v1-vmt)/sf[115])}else{vd})),
            3,
            multiplicity * (sf[161]),
            7,
            multiplicity * (sf[162]),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if sb[28]{vmx}else{vd})),
            3,
            multiplicity * ((if sb[28]{v1iq}else{vd})),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * ((if sb[28]{(vmt/sf[117])}else{vd})),
            7,
            multiplicity * (sf[164]),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * ((if sb[28]{vn3}else{vd})),
            7,
            multiplicity * ((if sb[28]{(sf[135]*v1hv)}else{vd})),
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * ((if sb[31]{vmi}else{vd})),
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            vd,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            vd,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            vd,
        );
        stamper.stamp_current_node1_local(
            Some(5),
            Some(6),
            multiplicity * ((vd*v5k)),
            6,
            multiplicity * (v1j0),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            Some(4),
            multiplicity * ((vd*vk)),
            4,
            multiplicity * (v1j0),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            Some(6),
            multiplicity * ((vd*(vj-v5j))),
            6,
            multiplicity * (v1j0),
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * ((if (sf[130]!=0.0){(v5p/vne)}else{vd})),
            1,
            multiplicity * ((if (sf[130]!=0.0){(vc/vne)}else{vd})),
            3,
            multiplicity * ((if (sf[130]!=0.0){((-(v5p*(if vnd{((if (sf[83]!=0.0){(v18r/vgd)}else{v18r})/sf[3])}else{vd})))/(vne*vne))}else{vd})),
            5,
            multiplicity * ((if (sf[130]!=0.0){(v7z/vne)}else{vd})),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (vd),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(5),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            vd,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(6),
            multiplicity * ((if (sf[131]!=0.0){(v5r/vni)}else{vd})),
            2,
            multiplicity * ((if (sf[131]!=0.0){(vc/vni)}else{vd})),
            3,
            multiplicity * ((if (sf[131]!=0.0){((-(v5r*(if vnh{((vfk*(sf[73]*(vfh*(sf[74]*vou))))/sf[3])}else{vd})))/(vni*vni))}else{vd})),
            6,
            multiplicity * ((if (sf[131]!=0.0){(v7z/vni)}else{vd})),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(6),
            multiplicity * (vd),
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(6),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            vd,
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(4),
            multiplicity * ((if (sf[132]!=0.0){(vnl/vnn)}else{vd})),
            0,
            multiplicity * ((if (sf[132]!=0.0){(vc/vnn)}else{vd})),
            3,
            multiplicity * ((if (sf[132]!=0.0){((-(vnl*(if vnm{((sf[71]*(vfc*(sf[72]*vou)))/sf[3])}else{vd})))/(vnn*vnn))}else{vd})),
            4,
            multiplicity * ((if (sf[132]!=0.0){(v7z/vnn)}else{vd})),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(4),
            multiplicity * (vd),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(4),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            vd,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * ((sf[3]*(sf[4]*vds))),
            [3, 4, 5, 6],
            [(sf[3]*(sf[4]*((if va8{vd}else{(if (v8p!=0.0){(((va2*(((v12*(sf[29]*(v26*(vp9/sf[30]))))-(v27*vow))/vpl))+(v28*vyr))-(((v7l*(vd*vzf))-(va4*vvc))/vvg))}else{vd})})+(((v15*(vvq-(if v8m{vd}else{(if (v7s!=0.0){((v8j*(sf[28]*(v21*(sf[26]*vou))))+(v22*vx3))}else{vd})})))-(vdq*voy))/v16g)))), (sf[3]*(sf[4]*((-(vdq*voz))/v16g))), (sf[3]*(sf[4]*((if va8{vd}else{(if (v8p!=0.0){((v28*vys)-((vd*vzg)/v7l))}else{vd})})+(((v15*(vvr-(if v8m{vd}else{(if (v7s!=0.0){(v22*vx4)}else{vd})})))-(vdq*vp0))/v16g)))), (sf[3]*(sf[4]*((if va8{vd}else{(if (v8p!=0.0){((v28*vyt)-((vd*vzh)/v7l))}else{vd})})+((vvs-(if v8m{vd}else{(if (v7s!=0.0){(v22*vx5)}else{vd})}))/v15))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * ((sf[3]*(sf[4]*vdu))),
            [3, 4, 5, 6],
            [(sf[3]*(sf[4]*((if vdg{vd}else{(if (vby!=0.0){(((vd7*(((v12*(sf[31]*(v2c*(vp9/sf[32]))))-(v2d*vow))/vpl))+(v2e*(if (vby!=0.0){((vce*v142)+(vcd*(vce*(if vc6{vd}else{v13l}))))}else{v142})))-((vd*(if (vby!=0.0){((if vcl{(v14q/vcn)}else{(if vcp{v14q}else{(if vch{v13p}else{vd})})})-(if vcx{(v15a/vcz)}else{(if vd1{v15a}else{(if vct{v13t}else{vd})})}))}else{v12a}))/vdc))}else{vd})})+(((v17*v13a)-(vbw*(sf[11]*vow)))/(v17*v17))))), (sf[3]*(sf[4]*((if vdg{vd}else{(if (vby!=0.0){((v2e*(if (vby!=0.0){((vce*v143)+(vcd*(vce*(if vc6{vd}else{v13m}))))}else{v143}))-((vd*(if (vby!=0.0){(if vcl{(v14r/vcn)}else{(if vcp{v14r}else{(if vch{v13q}else{vd})})})}else{v12b}))/vdc))}else{vd})})+(v13b/v17)))), (sf[3]*(sf[4]*((if vdg{vd}else{(if (vby!=0.0){((v2e*(if (vby!=0.0){((vce*v144)+(vcd*(vce*(if vc6{vd}else{v13n}))))}else{v144}))-((vd*(if (vby!=0.0){(if vcl{(v14s/vcn)}else{(if vcp{v14s}else{(if vch{v13r}else{vd})})})}else{v12c}))/vdc))}else{vd})})+(v13c/v17)))), (sf[3]*(sf[4]*((if vdg{vd}else{(if (vby!=0.0){((v2e*(if (vby!=0.0){((vce*v145)+(vcd*(vce*(if vc6{vd}else{v13o}))))}else{v145}))-((vd*(if (vby!=0.0){(if vcl{(v14t/vcn)}else{(if vcp{v14t}else{(if vch{v13s}else{vd})})})}else{v12d}))/vdc))}else{vd})})+(v13d/v17))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * ((sf[4]*(sf[3]*(-veg)))),
            [3, 4, 5, 6],
            [(sf[4]*(sf[3]*(-v17j))), (sf[4]*(sf[3]*(-v17m))), (sf[4]*(sf[3]*(-v17p))), (sf[4]*(sf[3]*(-v17s)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * ((sf[3]*(sf[4]*(((vdp*veh)*sf[62])+(vef*vem))))),
            [3, 4, 5, 6],
            [(sf[3]*(sf[4]*((sf[62]*(vdp*v17t))+(vef*(sf[63]*vvq))))), (sf[3]*(sf[4]*((sf[62]*(vdp*v17u))+(vem*v17g)))), (sf[3]*(sf[4]*((sf[62]*(vdp*v17x))+((vem*v17h)+(vef*(sf[63]*vvr)))))), (sf[3]*(sf[4]*((sf[62]*(vdp*v180))+((vem*v17i)+(vef*(sf[63]*vvs))))))],
            [],
            [],
            multiplicity,
        );
        let vo0_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, vo0);
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (vo0_ddt),
            3,
            multiplicity * (((v1ky) * ddt_scale)),
            5,
            multiplicity * (((v1kz) * ddt_scale)),
            6,
            multiplicity * (((v1l0) * ddt_scale)),
        );
        let vo2_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, vo2);
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (vo2_ddt),
            3,
            multiplicity * (((v1l4) * ddt_scale)),
            5,
            multiplicity * (((v1l5) * ddt_scale)),
            6,
            multiplicity * (((v1l6) * ddt_scale)),
        );
        let vo4_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, vo4);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(1),
            Some(4),
            multiplicity * (vo4_ddt),
            [1, 3, 4, 5, 6],
            [((v1lc) * ddt_scale), ((v1ld) * ddt_scale), ((v1le) * ddt_scale), ((v1lf) * ddt_scale), ((v1lg) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let vo6_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, vo6);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(4),
            multiplicity * (vo6_ddt),
            [1, 3, 4, 5, 6],
            [((v1lm) * ddt_scale), ((v1ln) * ddt_scale), ((v1lo) * ddt_scale), ((v1lp) * ddt_scale), ((v1lq) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let vo8_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, vo8);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (vo8_ddt),
            [3, 4, 5, 6],
            [((v1lv) * ddt_scale), ((v1lw) * ddt_scale), ((v1lx) * ddt_scale), ((v1ly) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let voa_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, voa);
        stamper.stamp_current_node3_local(
            Some(2),
            Some(4),
            multiplicity * (voa_ddt),
            2,
            multiplicity * (((v1m2) * ddt_scale)),
            3,
            multiplicity * (((v1m3) * ddt_scale)),
            4,
            multiplicity * (((v1m4) * ddt_scale)),
        );
        let voc_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, voc);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (voc_ddt),
            [3, 4, 5, 6],
            [((v1m9) * ddt_scale), ((v1ma) * ddt_scale), ((v1mb) * ddt_scale), ((v1mc) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let vod_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, vod);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (vod_ddt),
            [3, 4, 5, 6],
            [((v1md) * ddt_scale), ((v1me) * ddt_scale), ((v1mf) * ddt_scale), ((v1mg) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (vd),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (vd),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(6),
            multiplicity * (vd),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let CommonStampValues {
            v1, vc, vd, vi, vj, vk, vl, vx, 
            vz, v12, v1u, v21, v22, v26, v27, v28, 
            v5g, v5j, v5k, v5l, v5m, v68, v6m, v6p, 
            v7i, v7l, v7q, v7s, v8i, v8p, v8x, v9b, 
            va1, vag, vah, vai, vap, vav, vbl, vbp, 
            vbw, vdi, vef, veg, veh, vfm, vg0, vg7, 
            vmn, vmt, vn2, vo0, vo2, vo4, vo6, vo8, 
            voa, voc, vod, vos, vou, vp9, vvc, vvg, 
            vvq, vvr, vvs, vx3, vx4, vx5, vxx, vxy, 
            vy4, vyr, vys, vyt, vzf, vzg, vzh, v10i, 
            v10j, v10k, v10l, v10m, v10n, v10s, v10t, v10u, 
            v10v, v11g, v11h, v11i, v11j, v12a, v12b, v12c, 
            v12d, v13a, v13b, v13c, v13d, v17g, v17h, v17i, 
            v17j, v17m, v17p, v17s, v17t, v17u, v17x, v180, 
            v1ky, v1kz, v1l0, v1l4, v1l5, v1l6, v1lc, v1ld, 
            v1le, v1lf, v1lg, v1lm, v1ln, v1lo, v1lp, v1lq, 
            v1lv, v1lw, v1lx, v1ly, v1m2, v1m3, v1m4, v1m9, 
            v1ma, v1mb, v1mc, v1md, v1me, v1mf, v1mg, 
        }=self.eval_common_stamp_values(ctx);
        let vm1=0.0;let vm8=0.0;let vmo=0.0;let vmx=0.0;let vn3=0.0;let v1hv=1.0;let v1iq=(sf[134]*v1hv);

        stamper.stamp_current_reactive_node1(
            Some(nodes[9]),
            None,
            nodes[9],
            multiplicity * ((sf[133]*v1hv)),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[8]),
            None,
            nodes[8],
            multiplicity * ((if (sf[83]!=0.0){(vg0*v1hv)}else{vd})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * ((if (sf[116]!=0.0){v1iq}else{vd})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * ((if sb[28]{v1iq}else{vd})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[7]),
            None,
            nodes[7],
            multiplicity * ((if sb[28]{(sf[135]*v1hv)}else{vd})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes[3],
            multiplicity * (v1ky),
            nodes[5],
            multiplicity * (v1kz),
            nodes[6],
            multiplicity * (v1l0),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes[3],
            multiplicity * (v1l4),
            nodes[5],
            multiplicity * (v1l5),
            nodes[6],
            multiplicity * (v1l6),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            &[nodes[1], nodes[3], nodes[4], nodes[5], nodes[6]],
            &[v1lc, v1ld, v1le, v1lf, v1lg],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &[nodes[1], nodes[3], nodes[4], nodes[5], nodes[6]],
            &[v1lm, v1ln, v1lo, v1lp, v1lq],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &[nodes[3], nodes[4], nodes[5], nodes[6]],
            &[v1lv, v1lw, v1lx, v1ly],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[2]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * (v1m2),
            nodes[3],
            multiplicity * (v1m3),
            nodes[4],
            multiplicity * (v1m4),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &[nodes[3], nodes[4], nodes[5], nodes[6]],
            &[v1m9, v1ma, v1mb, v1mc],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &[nodes[3], nodes[4], nodes[5], nodes[6]],
            &[v1md, v1me, v1mf, v1mg],
            &[],
            &[],
            multiplicity,
        );
    }
}
