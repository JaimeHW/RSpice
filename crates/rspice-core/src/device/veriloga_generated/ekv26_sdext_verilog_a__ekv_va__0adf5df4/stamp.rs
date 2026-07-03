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
    v1: f64, vp: f64, vv: f64, v5c: f64, v5f: f64, v5p: f64, 
    v5v: f64, v64: f64, v67: f64, v6b: f64, v6e: f64, v74: f64, 
    v76: f64, v7a: f64, v7e: f64, v91: f64, v93: f64, v94: f64, 
    v98: f64, v9a: f64, v9b: f64, v9c: f64, v9i: f64, v9x: f64, 
    va0: f64, va1: f64, va4: f64, va7: f64, vbq: f64, vdo: f64, 
    vdr: f64, vds: f64, vdt: f64, vdu: f64, vdv: f64, vdw: f64, 
    vdx: f64, vdy: f64, ve0: f64, veg: f64, vfv: f64, viu: f64, 
    vix: f64, vmo: f64, vn1: f64, vnd: f64, vnh: f64, vur: f64, 
    vvz: f64, vw8: bool, vxr: f64, vxs: f64, vxt: f64, vy1: f64, 
    vy2: f64, vy3: f64, vyb: f64, vyc: f64, vyd: f64, vyl: f64, 
    vym: f64, vyn: f64, vzj: f64, vzk: f64, vzl: f64, vzm: f64, 
    vzr: f64, vzs: f64, vzt: f64, vzu: f64, v10e: f64, v10f: f64, 
    v10g: f64, v10h: f64, v112: f64, v113: f64, v114: f64, v115: f64, 
    v163: f64, v164: f64, v165: f64, v166: f64, v169: f64, v16c: f64, 
    v16f: f64, v16i: f64, v16j: f64, v16k: f64, v16l: f64, v16m: f64, 
    v16s: f64, v16t: f64, v16u: f64, v16v: f64, v16w: f64, v16x: f64, 
    v16y: f64, v16z: f64, v170: f64, v171: f64, v172: f64, v173: f64, 
    v174: f64, v175: f64, v17i: f64, v17j: f64, v17k: f64, v17l: f64, 
    v199: f64, v19a: f64, v19b: f64, v19c: f64, v19d: f64, v19e: f64, 
    v19f: f64, v19g: f64, v19h: f64, v19i: f64, v19j: f64, v19k: f64, 
    v19y: f64, v19z: f64, v1a0: f64, v1a1: f64, v1af: f64, v1ag: f64, 
    v1ah: f64, v1ai: f64, v1fr: f64, v1fs: f64, v1ft: f64, v1fu: f64, 
    v1mi: f64, v1mj: f64, v1mk: f64, v1ml: f64, v1mo: f64, v1mr: f64, 
    v1mu: f64, v1mx: f64, v1my: f64, v1mz: f64, v1n0: f64, v1n1: f64, 
    v1n2: f64, v1n3: f64, v1n4: f64, v1n5: f64, v1n6: f64, v1n7: f64, 
    v1n8: f64, v1n9: f64, v1nb: f64, v1nd: f64, v1nf: f64, v1nh: f64, 
    v1nm: f64, v1nn: f64, v1no: f64, v1np: f64, v2gy: f64, v34a: f64, 
    v34d: f64, v34g: f64, v34j: f64, v36m: f64, v36p: f64, v36s: f64, 
    v36v: f64, v38t: f64, v38u: f64, v38v: f64, v38w: f64, v3eu: f64, 
    v3ex: f64, v3g8: f64, v3gb: f64, 
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let v1=0.0;let v9=3.0;let vp=0.5;let vv=1.0;let v57=ctx.node_voltage(nodes[3]);let v5c=(sf[20]*(ctx.node_voltage(nodes[2])-v57));let v5f=(sf[20]*(ctx.node_voltage(nodes[0])-v57));let v5i=(if ((v5f-v5c)<v1){vv}else{v1});let v5m=(if (v5i!=0.0){v5f}else{v5c});let v5n=(if (v5i!=0.0){(if (v5i!=0.0){v5c}else{v1})}else{v5f});let v5p=(if (!(v5i!=0.0)){vv}else{(if (v5i!=0.0){-1.0}else{v1})});let v5t=(sf[241]+(sf[219]+(((sf[20]*(ctx.node_voltage(nodes[1])-v57))-sf[237])-sf[89])));let v5v=2.0;let v5y=(((v5t*v5t)+sf[242])).sqrt();let v60=(vp*(v5t+v5y));let v61=(sf[219]+v5m);let v64=((sf[188]+(v61*v61))).sqrt();let v67=((vp*(v61+v64))).sqrt();let v68=(sf[219]+v5n);let v6b=((sf[188]+(v68*v68))).sqrt();let v6e=((vp*(v68+v6b))).sqrt();let v6k=0.25;let v6o=((v60+sf[96])).sqrt();let v6p=(v60-sf[219]);let v6w=((sf[182]+(sf[219]+(v6p-(sf[70]*(v6o-sf[97])))))).sqrt();let v71=((sf[70]-(sf[94]*(v67+v6e)))+(sf[92]*v6w));let v74=((sf[182]+(v71*v71))).sqrt();let v76=(vp*(v71+v74));let v77=(v6k*v76);let v7a=((v60+(v76*v77))).sqrt();let v7c=(v7a-(vp*v76));let v7e=(v6p-(v76*v7c));let v7g=(sf[183]*(v7e-v5m));let v7h=-0.35;let v7j=(if (v7g>v7h){vv}else{v1});let v7k=1.3;let v7m=1.6;let v7n=(v7g+v7m);let v7p=((v7g+v7k)-(v7n).ln());let v7r=(if (v7j!=0.0){(v5v/v7p)}else{v1});let v7s=(v5v+v7r);let v7t=(vv+v7g);let v7v=(v7t+(v7r).ln());let v7x=(if (v7j!=0.0){(v7s/v7v)}else{v1});let v7z=(v7t+(v7x).ln());let v80=(v5v+v7x);let v83=-15.0;let v85=(if (v7g>v83){vv}else{v1});let v86=(!(v7j!=0.0));let v87=((v85!=0.0)&&v86);let v88=1.55;let v8a=((-v7g)).exp();let v8c=(if v87{(v88+v8a)}else{v7r});let v8d=(v5v+v8c);let v8f=(v7t+(v8c).ln());let v8h=(if v87{(v8d/v8f)}else{v7x});let v8j=(v7t+(v8h).ln());let v8k=(v5v+v8h);let v8n=-23.0;let v8p=(if (v7g>v8n){vv}else{v1});let v8r=(v86&&(!(v85!=0.0)));let v8s=((v8p!=0.0)&&v8r);let v8t=(v5v+v8a);let v8x=(v8r&&(!(v8p!=0.0)));let v8y=(v7g).exp();let v8z=1e-64;let v91=(if v8x{(v8y+v8z)}else{(if v8s{(vv/v8t)}else{(if v87{(v8j/v8k)}else{(if (v7j!=0.0){(v7z/v80)}else{v1})})})});let v92=(vv+v91);let v93=(v91*v92);let v94=(v93).sqrt();let v98=((v6k+(v94*sf[243]))).sqrt();let v9a=(sf[225]*(v98-vp));let v9b=(v5n-v5m);let v9c=(vp*v9b);let v9i=(sf[188]*((sf[5]*(v94-(sf[183]*v9a)))+0.015625));let v9r=0.75;let v9x=((v6k+(sf[243]*(v94-(v9r*(v93).ln()))))).sqrt();let va0=(sf[230]+(sf[225]*(v9x-vp)));let va1=(v9c-va0);let va4=((v9i+(va0*va0))).sqrt();let va7=((v9i+(va1*va1))).sqrt();let vac=(sf[183]*(va7+(((v7e-v9c)-v5m)-va4)));let vae=(if (vac>v7h){vv}else{v1});let vag=(v7m+vac);let vai=((v7k+vac)-(vag).ln());let vak=(if (vae!=0.0){(v5v/vai)}else{v8c});let val=(v5v+vak);let vam=(vv+vac);let vao=(vam+(vak).ln());let vaq=(if (vae!=0.0){(val/vao)}else{v8h});let vas=(vam+(vaq).ln());let vat=(v5v+vaq);let vax=(if (vac>v83){vv}else{v1});let vay=(!(vae!=0.0));let vaz=((vax!=0.0)&&vay);let vb1=((-vac)).exp();let vb3=(if vaz{(v88+vb1)}else{vak});let vb4=(v5v+vb3);let vb6=(vam+(vb3).ln());let vb8=(if vaz{(vb4/vb6)}else{vaq});let vba=(vam+(vb8).ln());let vbb=(v5v+vb8);let vbf=(if (vac>v8n){vv}else{v1});let vbh=(vay&&(!(vax!=0.0)));let vbi=((vbf!=0.0)&&vbh);let vbj=(v5v+vb1);let vbn=(vbh&&(!(vbf!=0.0)));let vbo=(vac).exp();let vbq=(if vbn{(v8z+vbo)}else{(if vbi{(vv/vbj)}else{(if vaz{(vba/vbb)}else{(if (vae!=0.0){(vas/vat)}else{v91})})})});let vca=(sf[183]*(v7e-v5n));let vcc=(if (vca>v7h){vv}else{v1});let vce=(v7m+vca);let vcg=((v7k+vca)-(vce).ln());let vci=(if (vcc!=0.0){(v5v/vcg)}else{vb3});let vcj=(v5v+vci);let vck=(vv+vca);let vcm=(vck+(vci).ln());let vco=(if (vcc!=0.0){(vcj/vcm)}else{vb8});let vcq=(vck+(vco).ln());let vcr=(v5v+vco);let vcv=(if (vca>v83){vv}else{v1});let vcw=(!(vcc!=0.0));let vcx=((vcv!=0.0)&&vcw);let vcz=((-vca)).exp();let vd1=(if vcx{(v88+vcz)}else{vci});let vd2=(v5v+vd1);let vd4=(vck+(vd1).ln());let vd6=(if vcx{(vd2/vd4)}else{vco});let vd8=(vck+(vd6).ln());let vd9=(v5v+vd6);let vdd=(if (vca>v8n){vv}else{v1});let vdf=(vcw&&(!(vcv!=0.0)));let vdg=((vdd!=0.0)&&vdf);let vdh=(v5v+vcz);let vdl=(vdf&&(!(vdd!=0.0)));let vdm=(vca).exp();
        let vdo=(if vdl{(v8z+vdm)}else{(if vdg{(vv/vdh)}else{(if vcx{(vd8/vd9)}else{(if (vcc!=0.0){(vcq/vcr)}else{vbq})})})});let vdp=(vv+vdo);let vdr=(v6k+v93);let vds=(v6k+(vdo*vdp));let vdt=(vdr).sqrt();let vdu=(vds).sqrt();let vdv=(vdt+vdu);let vdw=(vdv*vdv);let vdx=(sf[219]+v7e);let vdy=(1e-6+vdx);let ve0=(v5v*(vdy).sqrt());let veg=-0.5;let vfu=(v5v*v94);let vfv=4.0;let viu=(v5v*vdu);let vix=(v5v*vdt);let vlz=(vdr*vdt);let vm0=(vds*vdu);let vm3=((sf[219]+(vp*v7e))).sqrt();let vm4=(vm3+vm3);let vm9=(-(sf[110]*(sf[181]*(vv+(v76/vm4)))));let vma=0.266666666;let vmc=6.0;let vmd=(vds*vmc);let vmg=(vdu*vfv);let vml=(vma*((((v9*vm0)+(vdt*vmd))+(vdr*vmg))+(v5v*vlz)));let vmn=((vml/vdw)-vp);let vmo=(vm9*vmn);let vmq=(vdr*vmc);let vmt=(vdt*vfv);let vmy=(vma*((((v9*vlz)+(vdu*vmq))+(vds*vmt))+(v5v*vm0)));let vn0=((vmy/vdw)-vp);let vn1=(vm9*vn0);let vn2=(vmo+vn1);let vn3=(v76*veg);let vn8=(v76*vn2);let vn9=(v76+vm4);let vnd=((-vn2)-((sf[110]*((v60+(ve0*vn3))-v5t))-(vn8/vn9)));let vnh=(if (vv==v5p){vv}else{v1});let vte=(if (v5f>v1){vv}else{v1});let vtj=(vv+(v5f/sf[261]));let vtm=((sf[163]*(vtj).ln())).exp();let vtt=(vv+(v5f/sf[263]));let vtw=((sf[165]*(vtt).ln())).exp();let vu3=(vv+(v5f/sf[265]));let vu6=((sf[167]*(vu3).ln())).exp();let vu9=(!(vte!=0.0));let vuq=((if vu9{(sf[301]*(vv-((v5f*sf[166])/sf[265])))}else{(if (vte!=0.0){(sf[301]*vu6)}else{v1})})+((if vu9{(sf[299]*(vv-((v5f*sf[162])/sf[261])))}else{(if (vte!=0.0){(sf[299]*vtm)}else{v1})})+(if vu9{(sf[300]*(vv-((v5f*sf[164])/sf[263])))}else{(if (vte!=0.0){(sf[300]*vtw)}else{v1})})));let vur=(v5f*vuq);let vut=(if (v5c>v1){vv}else{v1});let vuw=(vv+(v5c/sf[261]));let vuz=((sf[163]*(vuw).ln())).exp();let vv4=(vv+(v5c/sf[263]));let vv7=((sf[165]*(vv4).ln())).exp();let vvb=(vv+(v5c/sf[265]));let vve=((sf[167]*(vvb).ln())).exp();let vvh=(!(vut!=0.0));let vvy=((if vvh{(sf[301]*(vv-((v5c*sf[166])/sf[265])))}else{(if (vut!=0.0){(sf[301]*vve)}else{v1})})+((if vvh{(sf[302]*(vv-((v5c*sf[162])/sf[261])))}else{(if (vut!=0.0){(sf[302]*vuz)}else{v1})})+(if vvh{(sf[303]*(vv-((v5c*sf[164])/sf[263])))}else{(if (vut!=0.0){(sf[303]*vv7)}else{v1})})));let vvz=(v5c*vvy);let vw8=(!(vnh!=0.0));let vx4=(if (v5i!=0.0){sf[20]}else{v1});let vx6=(if (v5i!=0.0){v1}else{sf[20]});let vx7=(if (v5i!=0.0){vx4}else{v1});let vx8=(if (v5i!=0.0){(if (v5i!=0.0){sf[169]}else{v1})}else{sf[169]});let vx9=(sf[20]*v5t);let vxb=(v5t*sf[169]);let vxd=(v5v*v5y);let vxi=(vp*(sf[20]+((vx9+vx9)/vxd)));let vxj=(vp*(sf[169]+((vxb+vxb)/vxd)));let vxk=(v61*vx4);let vxm=(v61*vx6);let vxo=(v61*sf[169]);let vxq=(v5v*v64);let vxr=((vxk+vxk)/vxq);let vxs=((vxm+vxm)/vxq);let vxt=((vxo+vxo)/vxq);let vy0=(v5v*v67);let vy1=((vp*(vx4+vxr))/vy0);let vy2=((vp*(vx6+vxs))/vy0);let vy3=((vp*(sf[169]+vxt))/vy0);let vy4=(v68*vx6);let vy6=(v68*vx7);let vy8=(v68*vx8);let vya=(v5v*v6b);let vyb=((vy4+vy4)/vya);let vyc=((vy6+vy6)/vya);let vyd=((vy8+vy8)/vya);let vyk=(v5v*v6e);let vyl=((vp*(vx6+vyb))/vyk);let vym=((vp*(vx7+vyc))/vyk);let vyn=((vp*(vx8+vyd))/vyk);let vyo=(v5v*v6o);let vyv=(v5v*v6w);let vz4=(-(sf[94]*(vy1+vyl)));let vz5=(-(sf[94]*(vy2+vym)));let vz7=(sf[92]*((vxi-(sf[70]*(vxi/vyo)))/vyv));let vz9=((-(sf[94]*(vy3+vyn)))+(sf[92]*((vxj-(sf[70]*(vxj/vyo)))/vyv)));let vza=(v71*vz4);let vzc=(v71*vz7);let vze=(v71*vz5);let vzg=(v71*vz9);let vzi=(v5v*v74);let vzj=((vza+vza)/vzi);let vzk=((vzc+vzc)/vzi);let vzl=((vze+vze)/vzi);let vzm=((vzg+vzg)/vzi);let vzr=(vp*(vz4+vzj));let vzs=(vp*(vz7+vzk));let vzt=(vp*(vz5+vzl));let vzu=(vp*(vz9+vzm));let v10d=(v5v*v7a);let v10e=(((v77*vzr)+(v76*(v6k*vzr)))/v10d);let v10f=((vxi+((v77*vzs)+(v76*(v6k*vzs))))/v10d);let v10g=(((v77*vzt)+(v76*(v6k*vzt)))/v10d);let v10h=((vxj+((v77*vzu)+(v76*(v6k*vzu))))/v10d);let v112=(-((v7c*vzr)+(v76*(v10e-(vp*vzr)))));let v113=(vxi-((v7c*vzs)+(v76*(v10f-(vp*vzs)))));let v114=(-((v7c*vzt)+(v76*(v10g-(vp*vzt)))));let v115=(vxj-((v7c*vzu)+(v76*(v10h-(vp*vzu)))));let v119=(sf[183]*(v112-vx4));let v11a=(sf[183]*v113);let v11b=(sf[183]*(v114-vx6));let v11c=(sf[183]*(v115-sf[169]));let v11n=(v7p*v7p);let v11y=(if (v7j!=0.0){((-(v5v*(v119-(v119/v7n))))/v11n)}else{v1});
        let v11z=(if (v7j!=0.0){((-(v5v*(v11a-(v11a/v7n))))/v11n)}else{v1});let v120=(if (v7j!=0.0){((-(v5v*(v11b-(v11b/v7n))))/v11n)}else{v1});let v121=(if (v7j!=0.0){((-(v5v*(v11c-(v11c/v7n))))/v11n)}else{v1});let v12d=(v7v*v7v);let v12r=(if (v7j!=0.0){(((v7v*v11y)-(v7s*(v119+(v11y/v7r))))/v12d)}else{v1});let v12s=(if (v7j!=0.0){(((v7v*v11z)-(v7s*(v11a+(v11z/v7r))))/v12d)}else{v1});let v12t=(if (v7j!=0.0){(((v7v*v120)-(v7s*(v11b+(v120/v7r))))/v12d)}else{v1});let v12u=(if (v7j!=0.0){(((v7v*v121)-(v7s*(v11c+(v121/v7r))))/v12d)}else{v1});let v136=(v80*v80);let v13p=(-v11a);let v13s=(v8a*(-v119));let v13t=(v8a*v13p);let v13u=(v8a*(-v11b));let v13v=(v8a*(-v11c));let v13w=(if v87{v13s}else{v11y});let v13x=(if v87{v13t}else{v11z});let v13y=(if v87{v13u}else{v120});let v13z=(if v87{v13v}else{v121});let v14b=(v8f*v8f);let v14p=(if v87{(((v8f*v13w)-(v8d*(v119+(v13w/v8c))))/v14b)}else{v12r});let v14q=(if v87{(((v8f*v13x)-(v8d*(v11a+(v13x/v8c))))/v14b)}else{v12s});let v14r=(if v87{(((v8f*v13y)-(v8d*(v11b+(v13y/v8c))))/v14b)}else{v12t});let v14s=(if v87{(((v8f*v13z)-(v8d*(v11c+(v13z/v8c))))/v14b)}else{v12u});let v154=(v8k*v8k);let v15n=(v8t*v8t);let v163=(if v8x{(v8y*v119)}else{(if v8s{((-v13s)/v15n)}else{(if v87{(((v8k*(v119+(v14p/v8h)))-(v8j*v14p))/v154)}else{(if (v7j!=0.0){(((v80*(v119+(v12r/v7x)))-(v7z*v12r))/v136)}else{v1})})})});let v164=(if v8x{(v8y*v11a)}else{(if v8s{((-v13t)/v15n)}else{(if v87{(((v8k*(v11a+(v14q/v8h)))-(v8j*v14q))/v154)}else{(if (v7j!=0.0){(((v80*(v11a+(v12s/v7x)))-(v7z*v12s))/v136)}else{v1})})})});let v165=(if v8x{(v8y*v11b)}else{(if v8s{((-v13u)/v15n)}else{(if v87{(((v8k*(v11b+(v14r/v8h)))-(v8j*v14r))/v154)}else{(if (v7j!=0.0){(((v80*(v11b+(v12t/v7x)))-(v7z*v12t))/v136)}else{v1})})})});let v166=(if v8x{(v8y*v11c)}else{(if v8s{((-v13v)/v15n)}else{(if v87{(((v8k*(v11c+(v14s/v8h)))-(v8j*v14s))/v154)}else{(if (v7j!=0.0){(((v80*(v11c+(v12u/v7x)))-(v7z*v12u))/v136)}else{v1})})})});let v169=((v92*v163)+(v91*v163));let v16c=((v92*v164)+(v91*v164));let v16f=((v92*v165)+(v91*v165));let v16i=((v92*v166)+(v91*v166));let v16j=(v169/vfu);let v16k=(v16c/vfu);let v16l=(v16f/vfu);let v16m=(v16i/vfu);let v16r=(v5v*v98);let v16s=((sf[243]*v16j)/v16r);let v16t=((sf[243]*v16k)/v16r);let v16u=((sf[243]*v16l)/v16r);let v16v=((sf[243]*v16m)/v16r);let v16w=(sf[225]*v16s);let v16x=(sf[225]*v16t);let v16y=(sf[225]*v16u);let v16z=(sf[225]*v16v);let v170=(vx6-vx4);let v171=(vx7-vx6);let v172=(vx8-sf[169]);let v173=(vp*v170);let v174=(vp*v171);let v175=(vp*v172);let v17i=(sf[188]*(sf[5]*(v16j-(sf[183]*v16w))));let v17j=(sf[188]*(sf[5]*(v16k-(sf[183]*v16x))));let v17k=(sf[188]*(sf[5]*(v16l-(sf[183]*v16y))));let v17l=(sf[188]*(sf[5]*(v16m-(sf[183]*v16z))));let v198=(v5v*v9x);let v199=((sf[243]*(v16j-(v9r*(v169/v93))))/v198);let v19a=((sf[243]*(v16k-(v9r*(v16c/v93))))/v198);let v19b=((sf[243]*(v16l-(v9r*(v16f/v93))))/v198);let v19c=((sf[243]*(v16m-(v9r*(v16i/v93))))/v198);let v19d=(sf[225]*v199);let v19e=(sf[225]*v19a);let v19f=(sf[225]*v19b);let v19g=(sf[225]*v19c);let v19h=(v173-v19d);let v19i=(-v19e);let v19j=(v174-v19f);let v19k=(v175-v19g);let v19l=(va0*v19d);let v19n=(va0*v19e);let v19p=(va0*v19f);let v19r=(va0*v19g);let v19x=(v5v*va4);let v19y=((v17i+(v19l+v19l))/v19x);let v19z=((v17j+(v19n+v19n))/v19x);let v1a0=((v17k+(v19p+v19p))/v19x);let v1a1=((v17l+(v19r+v19r))/v19x);let v1a2=(va1*v19h);let v1a4=(va1*v19i);let v1a6=(va1*v19j);let v1a8=(va1*v19k);let v1ae=(v5v*va7);let v1af=((v17i+(v1a2+v1a2))/v1ae);let v1ag=((v17j+(v1a4+v1a4))/v1ae);let v1ah=((v17k+(v1a6+v1a6))/v1ae);let v1ai=((v17l+(v1a8+v1a8))/v1ae);let v1ax=(sf[183]*(v1af+(((v112-v173)-vx4)-v19y)));let v1ay=(sf[183]*(v1ag+(v113-v19z)));let v1az=(sf[183]*(v1ah+(((v114-v174)-vx6)-v1a0)));let v1b0=(sf[183]*(v1ai+(((v115-v175)-sf[169])-v1a1)));let v1bb=(vai*vai);let v1bm=(if (vae!=0.0){((-(v5v*(v1ax-(v1ax/vag))))/v1bb)}else{v13w});let v1bn=(if (vae!=0.0){((-(v5v*(v1ay-(v1ay/vag))))/v1bb)}else{v13x});let v1bo=(if (vae!=0.0){((-(v5v*(v1az-(v1az/vag))))/v1bb)}else{v13y});let v1bp=(if (vae!=0.0){((-(v5v*(v1b0-(v1b0/vag))))/v1bb)}else{v13z});let v1c1=(vao*vao);
        let v1cf=(if (vae!=0.0){(((vao*v1bm)-(val*(v1ax+(v1bm/vak))))/v1c1)}else{v14p});let v1cg=(if (vae!=0.0){(((vao*v1bn)-(val*(v1ay+(v1bn/vak))))/v1c1)}else{v14q});let v1ch=(if (vae!=0.0){(((vao*v1bo)-(val*(v1az+(v1bo/vak))))/v1c1)}else{v14r});let v1ci=(if (vae!=0.0){(((vao*v1bp)-(val*(v1b0+(v1bp/vak))))/v1c1)}else{v14s});let v1cu=(vat*vat);let v1dg=(vb1*(-v1ax));let v1dh=(vb1*(-v1ay));let v1di=(vb1*(-v1az));let v1dj=(vb1*(-v1b0));let v1dk=(if vaz{v1dg}else{v1bm});let v1dl=(if vaz{v1dh}else{v1bn});let v1dm=(if vaz{v1di}else{v1bo});let v1dn=(if vaz{v1dj}else{v1bp});let v1dz=(vb6*vb6);let v1ed=(if vaz{(((vb6*v1dk)-(vb4*(v1ax+(v1dk/vb3))))/v1dz)}else{v1cf});let v1ee=(if vaz{(((vb6*v1dl)-(vb4*(v1ay+(v1dl/vb3))))/v1dz)}else{v1cg});let v1ef=(if vaz{(((vb6*v1dm)-(vb4*(v1az+(v1dm/vb3))))/v1dz)}else{v1ch});let v1eg=(if vaz{(((vb6*v1dn)-(vb4*(v1b0+(v1dn/vb3))))/v1dz)}else{v1ci});let v1es=(vbb*vbb);let v1fb=(vbj*vbj);let v1fr=(if vbn{(vbo*v1ax)}else{(if vbi{((-v1dg)/v1fb)}else{(if vaz{(((vbb*(v1ax+(v1ed/vb8)))-(vba*v1ed))/v1es)}else{(if (vae!=0.0){(((vat*(v1ax+(v1cf/vaq)))-(vas*v1cf))/v1cu)}else{v163})})})});let v1fs=(if vbn{(vbo*v1ay)}else{(if vbi{((-v1dh)/v1fb)}else{(if vaz{(((vbb*(v1ay+(v1ee/vb8)))-(vba*v1ee))/v1es)}else{(if (vae!=0.0){(((vat*(v1ay+(v1cg/vaq)))-(vas*v1cg))/v1cu)}else{v164})})})});let v1ft=(if vbn{(vbo*v1az)}else{(if vbi{((-v1di)/v1fb)}else{(if vaz{(((vbb*(v1az+(v1ef/vb8)))-(vba*v1ef))/v1es)}else{(if (vae!=0.0){(((vat*(v1az+(v1ch/vaq)))-(vas*v1ch))/v1cu)}else{v165})})})});let v1fu=(if vbn{(vbo*v1b0)}else{(if vbi{((-v1dj)/v1fb)}else{(if vaz{(((vbb*(v1b0+(v1eg/vb8)))-(vba*v1eg))/v1es)}else{(if (vae!=0.0){(((vat*(v1b0+(v1ci/vaq)))-(vas*v1ci))/v1cu)}else{v166})})})});let v1hq=(sf[183]*(v112-vx6));let v1hr=(sf[183]*(v114-vx7));let v1hs=(sf[183]*(v115-vx8));let v1i3=(vcg*vcg);let v1ie=(if (vcc!=0.0){((-(v5v*(v1hq-(v1hq/vce))))/v1i3)}else{v1dk});let v1if=(if (vcc!=0.0){((-(v5v*(v11a-(v11a/vce))))/v1i3)}else{v1dl});let v1ig=(if (vcc!=0.0){((-(v5v*(v1hr-(v1hr/vce))))/v1i3)}else{v1dm});let v1ih=(if (vcc!=0.0){((-(v5v*(v1hs-(v1hs/vce))))/v1i3)}else{v1dn});let v1it=(vcm*vcm);let v1j7=(if (vcc!=0.0){(((vcm*v1ie)-(vcj*(v1hq+(v1ie/vci))))/v1it)}else{v1ed});let v1j8=(if (vcc!=0.0){(((vcm*v1if)-(vcj*(v11a+(v1if/vci))))/v1it)}else{v1ee});let v1j9=(if (vcc!=0.0){(((vcm*v1ig)-(vcj*(v1hr+(v1ig/vci))))/v1it)}else{v1ef});let v1ja=(if (vcc!=0.0){(((vcm*v1ih)-(vcj*(v1hs+(v1ih/vci))))/v1it)}else{v1eg});let v1jm=(vcr*vcr);let v1k7=(vcz*(-v1hq));let v1k8=(vcz*v13p);let v1k9=(vcz*(-v1hr));let v1ka=(vcz*(-v1hs));let v1kb=(if vcx{v1k7}else{v1ie});let v1kc=(if vcx{v1k8}else{v1if});let v1kd=(if vcx{v1k9}else{v1ig});let v1ke=(if vcx{v1ka}else{v1ih});let v1kq=(vd4*vd4);let v1l4=(if vcx{(((vd4*v1kb)-(vd2*(v1hq+(v1kb/vd1))))/v1kq)}else{v1j7});let v1l5=(if vcx{(((vd4*v1kc)-(vd2*(v11a+(v1kc/vd1))))/v1kq)}else{v1j8});let v1l6=(if vcx{(((vd4*v1kd)-(vd2*(v1hr+(v1kd/vd1))))/v1kq)}else{v1j9});let v1l7=(if vcx{(((vd4*v1ke)-(vd2*(v1hs+(v1ke/vd1))))/v1kq)}else{v1ja});let v1lj=(vd9*vd9);let v1m2=(vdh*vdh);let v1mi=(if vdl{(vdm*v1hq)}else{(if vdg{((-v1k7)/v1m2)}else{(if vcx{(((vd9*(v1hq+(v1l4/vd6)))-(vd8*v1l4))/v1lj)}else{(if (vcc!=0.0){(((vcr*(v1hq+(v1j7/vco)))-(vcq*v1j7))/v1jm)}else{v1fr})})})});let v1mj=(if vdl{(vdm*v11a)}else{(if vdg{((-v1k8)/v1m2)}else{(if vcx{(((vd9*(v11a+(v1l5/vd6)))-(vd8*v1l5))/v1lj)}else{(if (vcc!=0.0){(((vcr*(v11a+(v1j8/vco)))-(vcq*v1j8))/v1jm)}else{v1fs})})})});let v1mk=(if vdl{(vdm*v1hr)}else{(if vdg{((-v1k9)/v1m2)}else{(if vcx{(((vd9*(v1hr+(v1l6/vd6)))-(vd8*v1l6))/v1lj)}else{(if (vcc!=0.0){(((vcr*(v1hr+(v1j9/vco)))-(vcq*v1j9))/v1jm)}else{v1ft})})})});let v1ml=(if vdl{(vdm*v1hs)}else{(if vdg{((-v1ka)/v1m2)}else{(if vcx{(((vd9*(v1hs+(v1l7/vd6)))-(vd8*v1l7))/v1lj)}else{(if (vcc!=0.0){(((vcr*(v1hs+(v1ja/vco)))-(vcq*v1ja))/v1jm)}else{v1fu})})})});let v1mo=((vdp*v1mi)+(vdo*v1mi));let v1mr=((vdp*v1mj)+(vdo*v1mj));let v1mu=((vdp*v1mk)+(vdo*v1mk));let v1mx=((vdp*v1ml)+(vdo*v1ml));let v1my=(v169/vix);let v1mz=(v16c/vix);let v1n0=(v16f/vix);let v1n1=(v16i/vix);let v1n2=(v1mo/viu);let v1n3=(v1mr/viu);let v1n4=(v1mu/viu);
        let v1n5=(v1mx/viu);let v1n6=(v1my+v1n2);let v1n7=(v1mz+v1n3);let v1n8=(v1n0+v1n4);let v1n9=(v1n1+v1n5);let v1na=(vdv*v1n6);let v1nb=(v1na+v1na);let v1nc=(vdv*v1n7);let v1nd=(v1nc+v1nc);let v1ne=(vdv*v1n8);let v1nf=(v1ne+v1ne);let v1ng=(vdv*v1n9);let v1nh=(v1ng+v1ng);let v1nm=(v5v*(v112/ve0));let v1nn=(v5v*(v113/ve0));let v1no=(v5v*(v114/ve0));let v1np=(v5v*(v115/ve0));let v2gy=(vdw*vdw);let v30g=((vdt*v169)+(vdr*v1my));let v30j=((vdt*v16c)+(vdr*v1mz));let v30m=((vdt*v16f)+(vdr*v1n0));let v30p=((vdt*v16i)+(vdr*v1n1));let v30s=((vdu*v1mo)+(vds*v1n2));let v30v=((vdu*v1mr)+(vds*v1n3));let v30y=((vdu*v1mu)+(vds*v1n4));let v311=((vdu*v1mx)+(vds*v1n5));let v316=(v5v*vm3);let v317=((vp*v112)/v316);let v318=((vp*v113)/v316);let v319=((vp*v114)/v316);let v31a=((vp*v115)/v316);let v31b=(v317+v317);let v31c=(v318+v318);let v31d=(v319+v319);let v31e=(v31a+v31a);let v31i=(vm4*vm4);let v324=(-(sf[110]*(sf[181]*(((vm4*vzr)-(v76*v31b))/v31i))));let v325=(-(sf[110]*(sf[181]*(((vm4*vzs)-(v76*v31c))/v31i))));let v326=(-(sf[110]*(sf[181]*(((vm4*vzt)-(v76*v31d))/v31i))));let v327=(-(sf[110]*(sf[181]*(((vm4*vzu)-(v76*v31e))/v31i))));let v34a=((vmn*v324)+(vm9*(((vdw*(vma*((((v9*v30s)+((vmd*v1my)+(vdt*(vmc*v1mo))))+((vmg*v169)+(vdr*(vfv*v1n2))))+(v5v*v30g))))-(vml*v1nb))/v2gy)));let v34d=((vmn*v325)+(vm9*(((vdw*(vma*((((v9*v30v)+((vmd*v1mz)+(vdt*(vmc*v1mr))))+((vmg*v16c)+(vdr*(vfv*v1n3))))+(v5v*v30j))))-(vml*v1nd))/v2gy)));let v34g=((vmn*v326)+(vm9*(((vdw*(vma*((((v9*v30y)+((vmd*v1n0)+(vdt*(vmc*v1mu))))+((vmg*v16f)+(vdr*(vfv*v1n4))))+(v5v*v30m))))-(vml*v1nf))/v2gy)));let v34j=((vmn*v327)+(vm9*(((vdw*(vma*((((v9*v311)+((vmd*v1n1)+(vdt*(vmc*v1mx))))+((vmg*v16i)+(vdr*(vfv*v1n5))))+(v5v*v30p))))-(vml*v1nh))/v2gy)));let v36m=((vn0*v324)+(vm9*(((vdw*(vma*((((v9*v30g)+((vmq*v1n2)+(vdu*(vmc*v169))))+((vmt*v1mo)+(vds*(vfv*v1my))))+(v5v*v30s))))-(vmy*v1nb))/v2gy)));let v36p=((vn0*v325)+(vm9*(((vdw*(vma*((((v9*v30j)+((vmq*v1n3)+(vdu*(vmc*v16c))))+((vmt*v1mr)+(vds*(vfv*v1mz))))+(v5v*v30v))))-(vmy*v1nd))/v2gy)));let v36s=((vn0*v326)+(vm9*(((vdw*(vma*((((v9*v30m)+((vmq*v1n4)+(vdu*(vmc*v16f))))+((vmt*v1mu)+(vds*(vfv*v1n0))))+(v5v*v30y))))-(vmy*v1nf))/v2gy)));let v36v=((vn0*v327)+(vm9*(((vdw*(vma*((((v9*v30p)+((vmq*v1n5)+(vdu*(vmc*v16i))))+((vmt*v1mx)+(vds*(vfv*v1n1))))+(v5v*v311))))-(vmy*v1nh))/v2gy)));let v36w=(v34a+v36m);let v36x=(v34d+v36p);let v36y=(v34g+v36s);let v36z=(v34j+v36v);let v387=(vn9*vn9);let v38t=((-v36w)-((sf[110]*((vn3*v1nm)+(ve0*(veg*vzr))))-(((vn9*((vn2*vzr)+(v76*v36w)))-(vn8*(vzr+v31b)))/v387)));let v38u=((-v36x)-((sf[110]*((vxi+((vn3*v1nn)+(ve0*(veg*vzs))))-sf[20]))-(((vn9*((vn2*vzs)+(v76*v36x)))-(vn8*(vzs+v31c)))/v387)));let v38v=((-v36y)-((sf[110]*((vn3*v1no)+(ve0*(veg*vzt))))-(((vn9*((vn2*vzt)+(v76*v36y)))-(vn8*(vzt+v31d)))/v387)));let v38w=((-v36z)-((sf[110]*((vxj+((vn3*v1np)+(ve0*(veg*vzu))))-sf[169]))-(((vn9*((vn2*vzu)+(v76*v36z)))-(vn8*(vzu+v31e)))/v387)));let v3eu=((sf[20]*vuq)+(v5f*((if vu9{sf[344]}else{(if (vte!=0.0){(sf[301]*(vu6*(sf[167]*(sf[326]/vu3))))}else{v1})})+((if vu9{sf[332]}else{(if (vte!=0.0){(sf[299]*(vtm*(sf[163]*(sf[322]/vtj))))}else{v1})})+(if vu9{sf[338]}else{(if (vte!=0.0){(sf[300]*(vtw*(sf[165]*(sf[324]/vtt))))}else{v1})})))));let v3ex=((vuq*sf[169])+(v5f*((if vu9{sf[345]}else{(if (vte!=0.0){(sf[301]*(vu6*(sf[167]*(sf[327]/vu3))))}else{v1})})+((if vu9{sf[333]}else{(if (vte!=0.0){(sf[299]*(vtm*(sf[163]*(sf[323]/vtj))))}else{v1})})+(if vu9{sf[339]}else{(if (vte!=0.0){(sf[300]*(vtw*(sf[165]*(sf[325]/vtt))))}else{v1})})))));let v3g8=((sf[20]*vvy)+(v5c*((if vvh{sf[344]}else{(if (vut!=0.0){(sf[301]*(vve*(sf[167]*(sf[326]/vvb))))}else{v1})})+((if vvh{sf[346]}else{(if (vut!=0.0){(sf[302]*(vuz*(sf[163]*(sf[322]/vuw))))}else{v1})})+(if vvh{sf[348]}else{(if (vut!=0.0){(sf[303]*(vv7*(sf[165]*(sf[324]/vv4))))}else{v1})})))));
        let v3gb=((vvy*sf[169])+(v5c*((if vvh{sf[345]}else{(if (vut!=0.0){(sf[301]*(vve*(sf[167]*(sf[327]/vvb))))}else{v1})})+((if vvh{sf[347]}else{(if (vut!=0.0){(sf[302]*(vuz*(sf[163]*(sf[323]/vuw))))}else{v1})})+(if vvh{sf[349]}else{(if (vut!=0.0){(sf[303]*(vv7*(sf[165]*(sf[325]/vv4))))}else{v1})})))));

        CommonStampValues {
            v1, vp, vv, v5c, v5f, v5p, v5v, v64, 
            v67, v6b, v6e, v74, v76, v7a, v7e, v91, 
            v93, v94, v98, v9a, v9b, v9c, v9i, v9x, 
            va0, va1, va4, va7, vbq, vdo, vdr, vds, 
            vdt, vdu, vdv, vdw, vdx, vdy, ve0, veg, 
            vfv, viu, vix, vmo, vn1, vnd, vnh, vur, 
            vvz, vw8, vxr, vxs, vxt, vy1, vy2, vy3, 
            vyb, vyc, vyd, vyl, vym, vyn, vzj, vzk, 
            vzl, vzm, vzr, vzs, vzt, vzu, v10e, v10f, 
            v10g, v10h, v112, v113, v114, v115, v163, v164, 
            v165, v166, v169, v16c, v16f, v16i, v16j, v16k, 
            v16l, v16m, v16s, v16t, v16u, v16v, v16w, v16x, 
            v16y, v16z, v170, v171, v172, v173, v174, v175, 
            v17i, v17j, v17k, v17l, v199, v19a, v19b, v19c, 
            v19d, v19e, v19f, v19g, v19h, v19i, v19j, v19k, 
            v19y, v19z, v1a0, v1a1, v1af, v1ag, v1ah, v1ai, 
            v1fr, v1fs, v1ft, v1fu, v1mi, v1mj, v1mk, v1ml, 
            v1mo, v1mr, v1mu, v1mx, v1my, v1mz, v1n0, v1n1, 
            v1n2, v1n3, v1n4, v1n5, v1n6, v1n7, v1n8, v1n9, 
            v1nb, v1nd, v1nf, v1nh, v1nm, v1nn, v1no, v1np, 
            v2gy, v34a, v34d, v34g, v34j, v36m, v36p, v36s, 
            v36v, v38t, v38u, v38v, v38w, v3eu, v3ex, v3g8, 
            v3gb, 
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
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
            v1, vp, vv, v5c, v5f, v5p, v5v, v64, 
            v67, v6b, v6e, v74, v76, v7a, v7e, v91, 
            v93, v94, v98, v9a, v9b, v9c, v9i, v9x, 
            va0, va1, va4, va7, vbq, vdo, vdr, vds, 
            vdt, vdu, vdv, vdw, vdx, vdy, ve0, veg, 
            vfv, viu, vix, vmo, vn1, vnd, vnh, vur, 
            vvz, vw8, vxr, vxs, vxt, vy1, vy2, vy3, 
            vyb, vyc, vyd, vyl, vym, vyn, vzj, vzk, 
            vzl, vzm, vzr, vzs, vzt, vzu, v10e, v10f, 
            v10g, v10h, v112, v113, v114, v115, v163, v164, 
            v165, v166, v169, v16c, v16f, v16i, v16j, v16k, 
            v16l, v16m, v16s, v16t, v16u, v16v, v16w, v16x, 
            v16y, v16z, v170, v171, v172, v173, v174, v175, 
            v17i, v17j, v17k, v17l, v199, v19a, v19b, v19c, 
            v19d, v19e, v19f, v19g, v19h, v19i, v19j, v19k, 
            v19y, v19z, v1a0, v1a1, v1af, v1ag, v1ah, v1ai, 
            v1fr, v1fs, v1ft, v1fu, v1mi, v1mj, v1mk, v1ml, 
            v1mo, v1mr, v1mu, v1mx, v1my, v1mz, v1n0, v1n1, 
            v1n2, v1n3, v1n4, v1n5, v1n6, v1n7, v1n8, v1n9, 
            v1nb, v1nd, v1nf, v1nh, v1nm, v1nn, v1no, v1np, 
            v2gy, v34a, v34d, v34g, v34j, v36m, v36p, v36s, 
            v36v, v38t, v38u, v38v, v38w, v3eu, v3ex, v3g8, 
            v3gb, 
        }=self.eval_common_stamp_values(ctx);
        let v9l=((v9i+(v9a*v9a))).sqrt();let v9m=(v9c-v9a);let v9p=((v9i+(v9m*v9m))).sqrt();let v9q=(v9l-v9p);let vbr=(vv+vbq);let vbv=(vv+((v9c-v9q)/sf[222]));let vc1=((sf[48]-(sf[6]*(vbv).ln()))+(sf[221]*(v9c+v9q)));let vc6=(((vc1*vc1)+sf[99])).sqrt();let vc8=(vp*(vc1+vc6));let ve1=(sf[70]/ve0);let ve2=(sf[70]+ve0);let ve3=(sf[70]/ve2);let ve4=(vv+ve1);let ve6=(sf[181]*(-ve4));let ve7=0.66666666;let ve8=1.33333332;let vec=(ve8*(vdr+(vds+(vdt*vdu))));let vee=((vec/vdv)-vv);let vef=(ve6*vee);let vep=((sf[187]+(v7e*v7e))).sqrt();let veq=(if (sf[101]!=0.0){vep}else{v1});let vev=((if (sf[101]!=0.0){(vp*(v7e+veq))}else{v1})*sf[102]);let vex=(if (sf[101]!=0.0){(vv+vev)}else{v1});let vey=(vc8*vex);let vf2=(((ve0*sf[100])-(ve3*vef))+(sf[21]*vef));let vf4=(if (vf2>v1){vv}else{v1});let vf6=((vf4!=0.0)&&sb[12]);let vf7=(sf[16]*vf2);let vfb=(sb[12]&&(!(vf4!=0.0)));let vfd=(if vfb{(vv-vf7)}else{(if vf6{(vv+vf7)}else{v1})});let vfi=(vc8*vfd);let vfk=(if sb[12]{(sf[247]/vfi)}else{(if (sf[101]!=0.0){(sf[240]/vey)}else{v1})});let vfl=(sf[185]+vdx);let vfm=(vfl).sqrt();let vfn=(v5v*vfm);let vfp=(vv+(sf[70]/vfn));let vfq=(v93-(vbq*vbr));let vfr=(sf[187]*vfp);let vfs=(vfk*vfr);let vft=(vfq*vfs);let vfw=(v74+v74);let vfz=((v76/vfw)*sf[103]);let vg0=(v6e*vfz);let vg1=(vg0/v6b);let vg2=(v67*vfz);let vg3=(vg2/v64);let vg5=(-(vdx/v7a));let vg6=(vg1*vg5);let vg7=(vg3*vg5);let vg8=(sf[183]*v91);let vg9=(vg6*vg8);let vga=(vg7-vv);let vgb=(vg8*vga);let vgc=(v98*vfv);let vgd=(v94*vgc);let vge=(sf[181]/vgd);let vgf=(vg9*vge);let vgg=(vgb*vge);let vgj=(v94+v94);let vgk=(sf[181]/vgj);let vgn=(sf[249]*((vg9*vgk)-vgf));let vgq=(sf[249]*((vgb*vgk)-vgg));let vgr=(vv/v9l);let vgs=(vv/v9p);let vgu=(vgn+(v9a*vgf));let vgw=(vp-vgf);let vgy=(vgn+(v9m*vgw));let vh0=((vgr*vgu)-(vgs*vgy));let vh2=(vgq+(v9a*vgg));let vh4=(veg-vgg);let vh6=(vgq+(v9m*vh4));let vh8=((vgr*vh2)-(vgs*vh6));let vhb=(sf[181]*(v94-1.5));let vhc=(v9x*vfv);let vhd=(v93*vhc);let vhe=(vhb/vhd);let vhf=(vg9*vhe);let vhg=(vgb*vhe);let vhh=(sf[183]*vbq);let vhi=(vv/va4);let vhj=(vv/va7);let vhm=(vgn+(va0*vhf));let vhp=(vp-vhf);let vhr=(vgn+(va1*vhp));let vht=(((vg6-vp)-(vhi*vhm))+(vhj*vhr));let vhx=(vgq+(va0*vhg));let vi0=(veg-vhg);let vi2=(vgq+(va1*vi0));let vi4=(((vg7-vp)-(vhi*vhx))+(vhj*vi2));let vi7=((sf[222]+v9c)-v9q);let vi8=(sf[6]/vi7);let vi9=(vp-vh0);let vib=(veg-vh8);let vid=(vv/vc6);let vih=((-(vi8*vi9))+(sf[221]*(vp+vh0)));let vim=((-(vi8*vib))+(sf[221]*(veg+vh8)));let vio=(sf[183]*vdo);let vip=(vg6-vv);let viq=(vio*vip);let vir=(vg7*vio);let vis=(ve6*ve7);let vit=(vis/vdw);let viv=(vdt+viu);let viw=(vit*viv);let viy=(vdu+vix);let viz=(vit*viy);let vj0=(-ve1);let vj1=(vef*vj0);let vj3=(ve1+(v5v+ve1));let vj4=(vdy*vj3);let vj5=(vj1/vj4);let vja=(((vg6*vj5)+(vg9*viw))+(viq*viz));let vjf=(((vg7*vj5)+(vgb*viw))+(vir*viz));let vjg=(v5v*ve4);let vjh=(vdy*vjg);let vjj=(ve4-(vef/vjh));let vjk=(-ve3);let vjm=(vja+(vg6*vjj));let vjp=(vjf+(vg7*vjj));let vjr=(veq*vex);let vjt=(if (sf[101]!=0.0){(vev/vjr)}else{vjj});let vjy=(-(vid*vih));let vk1=(-(vid*vim));let vk5=(if sb[12]{(sf[16]/vfd)}else{vjt});let vk7=((vjk*vjm)+(sf[21]*vja));let vkc=((vjk*vjp)+(sf[21]*vjf));let vkh=(vfp*vfv);let vki=(vfm*vkh);let vkj=(vfl*vki);let vkk=(sf[104]/vkj);let vkn=((if sb[12]{(vjy+(vk5*vk7))}else{(if (sf[101]!=0.0){(vjy-(if (sf[101]!=0.0){(vg6*vjt)}else{v1}))}else{v1})})+(vg6*vkk));let vkq=((vg9+(vfq*vkn))-(vhh*vht));let vks=(-vfs);let vkt=((if sb[12]{(vk1+(vk5*vkc))}else{(if (sf[101]!=0.0){(vk1-(if (sf[101]!=0.0){(vg7*vjt)}else{v1}))}else{v1})})+(vg7*vkk));let vkw=((vgb+(vfq*vkt))-(vhh*vi4));let vl6=((vv+((vks*vkw)*sf[109]))+((vfs*vkq)*sf[109]));let vl7=(vv/vl6);let vl8=(vft*vl7);let vla=(v9b-(sf[13]*v9a));let vle=(if ((vla>v1)&&sb[26]){vv}else{v1});let vlj=(if (vle!=0.0){((if (vle!=0.0){(vv/vla)}else{v1})*sf[250])}else{v1});let vlk=-35.0;let vln=((vle!=0.0)&&((if (vlj<vlk){vv}else{v1})!=0.0));let vlp=((if vln{vlk}else{vlj})).exp();let vlq=(if (vle!=0.0){vlp}else{v1});let vlr=(sf[224]*vla);let vlt=(if (vle!=0.0){(vlq*vlr)}else{v1});let vlw=(!(vle!=0.0));
        let vne=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, vmo);let vnf=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, vn1);let vq7=(-v5f);let vqa=((sf[195]*vq7)/sf[290]);let vqb=-40.0;let vqd=(if (vqa<vqb){vv}else{v1});let vqi=((sf[195]*(vq7+sf[156]))/sf[290]);let vqj=70.0;let vql=(if (vqi>vqj){vv}else{v1});let vqn=(!(vql!=0.0));let vqq=((-vqi)).exp();let vqt=(if vqn{(vv+(sf[157]*vqq))}else{(if (vql!=0.0){vv}else{v1})});let vqw=(sf[195]*v5f);let vr0=((vqw/sf[292])*sf[159]);let vr1=(v5f+sf[159]);let vr2=0.001;let vr3=(vr1>vr2);let vr4=(if vr3{vr1}else{vr2});let vr6=((vr0/vr4)).exp();let vrc=((vqw/sf[293])*sf[160]);let vrd=(v5f+sf[160]);let vre=(vrd>vr2);let vrf=(if vre{vrd}else{vr2});let vrh=((vrc/vrf)).exp();let vro=((vqw/sf[294])*sf[161]);let vrp=(v5f+sf[161]);let vrq=(vrp>vr2);let vrr=(if vrq{vrp}else{vr2});let vrt=((vro/vrr)).exp();let vs1=(-v5c);let vs3=((sf[195]*vs1)/sf[290]);let vs5=(if (vs3<vqb){vv}else{v1});let vs9=((sf[195]*(sf[156]+vs1))/sf[290]);let vsb=(if (vs9>vqj){vv}else{v1});let vsd=(!(vsb!=0.0));let vsf=((-vs9)).exp();let vsi=(if vsd{(vv+(sf[157]*vsf))}else{(if (vsb!=0.0){vv}else{v1})});let vsj=(sf[195]*v5c);let vsl=(sf[159]*(vsj/sf[292]));let vsm=(v5c+sf[159]);let vsn=(vsm>vr2);let vso=(if vsn{vsm}else{vr2});let vsq=((vsl/vso)).exp();let vsu=(sf[160]*(vsj/sf[293]));let vsv=(v5c+sf[160]);let vsw=(vsv>vr2);let vsx=(if vsw{vsv}else{vr2});let vsz=((vsu/vsx)).exp();let vt4=(sf[161]*(vsj/sf[294]));let vt5=(v5c+sf[161]);let vt6=(vt5>vr2);let vt7=(if vt6{vt5}else{vr2});let vt9=((vt4/vt7)).exp();let vw0=(sf[20]*v5p);let vw2=(sf[20]*vne);let vw4=(sf[20]*vnf);let vw6=(sf[20]*(if vlw{v1}else{(if (vle!=0.0){(vl8*vlt)}else{v1})}));let vwc=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, vnd);let vwe=((if (vqd!=0.0){vqb}else{vqa})).exp();let vwg=(sf[289]*(vv-vwe));let vwo=((if (vs5!=0.0){vqb}else{vs3})).exp();let vwq=(sf[298]*(vv-vwo));let vwx=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, vur);let vx0=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, vvz);let v17m=(v9a*v16w);let v17o=(v9a*v16x);let v17q=(v9a*v16y);let v17s=(v9a*v16z);let v17y=(v5v*v9l);let v17z=((v17i+(v17m+v17m))/v17y);let v180=((v17j+(v17o+v17o))/v17y);let v181=((v17k+(v17q+v17q))/v17y);let v182=((v17l+(v17s+v17s))/v17y);let v183=(v173-v16w);let v184=(-v16x);let v185=(v174-v16y);let v186=(v175-v16z);let v187=(v9m*v183);let v189=(v9m*v184);let v18b=(v9m*v185);let v18d=(v9m*v186);let v18j=(v5v*v9p);let v18k=((v17i+(v187+v187))/v18j);let v18l=((v17j+(v189+v189))/v18j);let v18m=((v17k+(v18b+v18b))/v18j);let v18n=((v17l+(v18d+v18d))/v18j);let v18o=(v17z-v18k);let v18p=(v180-v18l);let v18q=(v181-v18m);let v18r=(v182-v18n);let v1g7=(v173-v18o);let v1g8=(-v18p);let v1g9=(v174-v18q);let v1ga=(v175-v18r);let v1gy=((-(sf[6]*((v1g7/sf[222])/vbv)))+(sf[221]*(v173+v18o)));let v1gz=((-(sf[6]*((v1g8/sf[222])/vbv)))+(sf[221]*v18p));let v1h0=((-(sf[6]*((v1g9/sf[222])/vbv)))+(sf[221]*(v174+v18q)));let v1h1=((-(sf[6]*((v1ga/sf[222])/vbv)))+(sf[221]*(v175+v18r)));let v1h2=(vc1*v1gy);let v1h4=(vc1*v1gz);let v1h6=(vc1*v1h0);let v1h8=(vc1*v1h1);let v1ha=(v5v*vc6);let v1hb=((v1h2+v1h2)/v1ha);
        let v1hc=((v1h4+v1h4)/v1ha);let v1hd=((v1h6+v1h6)/v1ha);let v1he=((v1h8+v1h8)/v1ha);let v1hj=(vp*(v1gy+v1hb));let v1hk=(vp*(v1gz+v1hc));let v1hl=(vp*(v1h0+v1hd));let v1hm=(vp*(v1h1+v1he));let v1nr=(-(sf[70]*v1nm));let v1ns=(ve0*ve0);let v1nt=(v1nr/v1ns);let v1nv=(-(sf[70]*v1nn));let v1nw=(v1nv/v1ns);let v1ny=(-(sf[70]*v1no));let v1nz=(v1ny/v1ns);let v1o1=(-(sf[70]*v1np));let v1o2=(v1o1/v1ns);let v1o3=(ve2*ve2);let v1o4=(v1nr/v1o3);let v1o5=(v1nv/v1o3);let v1o6=(v1ny/v1o3);let v1o7=(v1o1/v1o3);let v1o8=(-v1nt);let v1o9=(-v1nw);let v1oa=(-v1nz);let v1ob=(-v1o2);let v1oc=(sf[181]*v1o8);let v1od=(sf[181]*v1o9);let v1oe=(sf[181]*v1oa);let v1of=(sf[181]*v1ob);let v1pm=((vee*v1oc)+(ve6*(((vdv*(ve8*(v169+(v1mo+((vdu*v1my)+(vdt*v1n2))))))-(vec*v1n6))/vdw)));let v1pp=((vee*v1od)+(ve6*(((vdv*(ve8*(v16c+(v1mr+((vdu*v1mz)+(vdt*v1n3))))))-(vec*v1n7))/vdw)));let v1ps=((vee*v1oe)+(ve6*(((vdv*(ve8*(v16f+(v1mu+((vdu*v1n0)+(vdt*v1n4))))))-(vec*v1n8))/vdw)));let v1pv=((vee*v1of)+(ve6*(((vdv*(ve8*(v16i+(v1mx+((vdu*v1n1)+(vdt*v1n5))))))-(vec*v1n9))/vdw)));let v1qg=(v7e*v112);let v1qi=(v7e*v113);let v1qk=(v7e*v114);let v1qm=(v7e*v115);let v1qo=(v5v*vep);let v1qt=(if (sf[101]!=0.0){((v1qg+v1qg)/v1qo)}else{v1});let v1qu=(if (sf[101]!=0.0){((v1qi+v1qi)/v1qo)}else{v1});let v1qv=(if (sf[101]!=0.0){((v1qk+v1qk)/v1qo)}else{v1});let v1qw=(if (sf[101]!=0.0){((v1qm+v1qm)/v1qo)}else{v1});let v1r9=(sf[102]*(if (sf[101]!=0.0){(vp*(v112+v1qt))}else{v1}));let v1ra=(sf[102]*(if (sf[101]!=0.0){(vp*(v113+v1qu))}else{v1}));let v1rb=(sf[102]*(if (sf[101]!=0.0){(vp*(v114+v1qv))}else{v1}));let v1rc=(sf[102]*(if (sf[101]!=0.0){(vp*(v115+v1qw))}else{v1}));let v1rd=(if (sf[101]!=0.0){v1r9}else{v1});let v1re=(if (sf[101]!=0.0){v1ra}else{v1});let v1rf=(if (sf[101]!=0.0){v1rb}else{v1});let v1rg=(if (sf[101]!=0.0){v1rc}else{v1});let v1rv=(vey*vey);let v1si=(sf[16]*(((sf[100]*v1nm)-((vef*v1o4)+(ve3*v1pm)))+(sf[21]*v1pm)));let v1sj=(sf[16]*(((sf[100]*v1nn)-((vef*v1o5)+(ve3*v1pp)))+(sf[21]*v1pp)));let v1sk=(sf[16]*(((sf[100]*v1no)-((vef*v1o6)+(ve3*v1ps)))+(sf[21]*v1ps)));let v1sl=(sf[16]*(((sf[100]*v1np)-((vef*v1o7)+(ve3*v1pv)))+(sf[21]*v1pv)));let v1su=(if vfb{(-v1si)}else{(if vf6{v1si}else{v1})});let v1sv=(if vfb{(-v1sj)}else{(if vf6{v1sj}else{v1})});let v1sw=(if vfb{(-v1sk)}else{(if vf6{v1sk}else{v1})});let v1sx=(if vfb{(-v1sl)}else{(if vf6{v1sl}else{v1})});let v1tc=(vfi*vfi);let v1tr=(v112/vfn);let v1ts=(v113/vfn);let v1tt=(v114/vfn);let v1tu=(v115/vfn);let v1u1=(vfn*vfn);let v1u2=((-(sf[70]*(v5v*v1tr)))/v1u1);let v1u5=((-(sf[70]*(v5v*v1ts)))/v1u1);let v1u8=((-(sf[70]*(v5v*v1tt)))/v1u1);let v1ub=((-(sf[70]*(v5v*v1tu)))/v1u1);let v1uc=(v169-((vbr*v1fr)+(vbq*v1fr)));let v1ud=(v16c-((vbr*v1fs)+(vbq*v1fs)));let v1ue=(v16f-((vbr*v1ft)+(vbq*v1ft)));let v1uf=(v16i-((vbr*v1fu)+(vbq*v1fu)));let v1um=((vfr*(if sb[12]{((-(sf[247]*((vfd*v1hj)+(vc8*v1su))))/v1tc)}else{(if (sf[101]!=0.0){((-(sf[240]*((vex*v1hj)+(vc8*v1rd))))/v1rv)}else{v1})}))+(vfk*(sf[187]*v1u2)));let v1up=((vfr*(if sb[12]{((-(sf[247]*((vfd*v1hk)+(vc8*v1sv))))/v1tc)}else{(if (sf[101]!=0.0){((-(sf[240]*((vex*v1hk)+(vc8*v1re))))/v1rv)}else{v1})}))+(vfk*(sf[187]*v1u5)));let v1us=((vfr*(if sb[12]{((-(sf[247]*((vfd*v1hl)+(vc8*v1sw))))/v1tc)}else{(if (sf[101]!=0.0){((-(sf[240]*((vex*v1hl)+(vc8*v1rf))))/v1rv)}else{v1})}))+(vfk*(sf[187]*v1u8)));let v1uv=((vfr*(if sb[12]{((-(sf[247]*((vfd*v1hm)+(vc8*v1sx))))/v1tc)}else{(if (sf[101]!=0.0){((-(sf[240]*((vex*v1hm)+(vc8*v1rg))))/v1rv)}else{v1})}))+(vfk*(sf[187]*v1ub)));let v1vf=(vfw*vfw);let v1vt=(sf[103]*(((vfw*vzr)-(v76*(vzj+vzj)))/v1vf));let v1vu=(sf[103]*(((vfw*vzs)-(v76*(vzk+vzk)))/v1vf));let v1vv=(sf[103]*(((vfw*vzt)-(v76*(vzl+vzl)))/v1vf));let v1vw=(sf[103]*(((vfw*vzu)-(v76*(vzm+vzm)))/v1vf));let v1wa=(v6b*v6b);let v1wy=(v64*v64);let v1xc=(v7a*v7a);let v1xq=(-(((v7a*v112)-(vdx*v10e))/v1xc));let v1xr=(-(((v7a*v113)-(vdx*v10f))/v1xc));let v1xs=(-(((v7a*v114)-(vdx*v10g))/v1xc));let v1xt=(-(((v7a*v115)-(vdx*v10h))/v1xc));let v1xw=((vg5*(((v6b*((vfz*vyl)+(v6e*v1vt)))-(vg0*vyb))/v1wa))+(vg1*v1xq));let v1xz=((vg5*((v6e*v1vu)/v6b))+(vg1*v1xr));
        let v1y2=((vg5*(((v6b*((vfz*vym)+(v6e*v1vv)))-(vg0*vyc))/v1wa))+(vg1*v1xs));let v1y5=((vg5*(((v6b*((vfz*vyn)+(v6e*v1vw)))-(vg0*vyd))/v1wa))+(vg1*v1xt));let v1y8=((vg5*(((v64*((vfz*vy1)+(v67*v1vt)))-(vg2*vxr))/v1wy))+(vg3*v1xq));let v1yb=((vg5*((v67*v1vu)/v64))+(vg3*v1xr));let v1ye=((vg5*(((v64*((vfz*vy2)+(v67*v1vv)))-(vg2*vxs))/v1wy))+(vg3*v1xs));let v1yh=((vg5*(((v64*((vfz*vy3)+(v67*v1vw)))-(vg2*vxt))/v1wy))+(vg3*v1xt));let v1yi=(sf[183]*v163);let v1yj=(sf[183]*v164);let v1yk=(sf[183]*v165);let v1yl=(sf[183]*v166);let v1yo=((vg8*v1xw)+(vg6*v1yi));let v1yr=((vg8*v1xz)+(vg6*v1yj));let v1yu=((vg8*v1y2)+(vg6*v1yk));let v1yx=((vg8*v1y5)+(vg6*v1yl));let v1z0=((vga*v1yi)+(vg8*v1y8));let v1z3=((vga*v1yj)+(vg8*v1yb));let v1z6=((vga*v1yk)+(vg8*v1ye));let v1z9=((vga*v1yl)+(vg8*v1yh));let v1zs=(vgd*vgd);let v1zt=((-(sf[181]*((vgc*v16j)+(v94*(vfv*v16s)))))/v1zs);let v1zw=((-(sf[181]*((vgc*v16k)+(v94*(vfv*v16t)))))/v1zs);let v1zz=((-(sf[181]*((vgc*v16l)+(v94*(vfv*v16u)))))/v1zs);let v202=((-(sf[181]*((vgc*v16m)+(v94*(vfv*v16v)))))/v1zs);let v205=((vge*v1yo)+(vg9*v1zt));let v208=((vge*v1yr)+(vg9*v1zw));let v20b=((vge*v1yu)+(vg9*v1zz));let v20e=((vge*v1yx)+(vg9*v202));let v20h=((vge*v1z0)+(vgb*v1zt));let v20k=((vge*v1z3)+(vgb*v1zw));let v20n=((vge*v1z6)+(vgb*v1zz));let v20q=((vge*v1z9)+(vgb*v202));let v20x=(vgj*vgj);let v20y=((-(sf[181]*(v16j+v16j)))/v20x);let v211=((-(sf[181]*(v16k+v16k)))/v20x);let v214=((-(sf[181]*(v16l+v16l)))/v20x);let v217=((-(sf[181]*(v16m+v16m)))/v20x);let v21o=(sf[249]*(((vgk*v1yo)+(vg9*v20y))-v205));let v21p=(sf[249]*(((vgk*v1yr)+(vg9*v211))-v208));let v21q=(sf[249]*(((vgk*v1yu)+(vg9*v214))-v20b));let v21r=(sf[249]*(((vgk*v1yx)+(vg9*v217))-v20e));let v228=(sf[249]*(((vgk*v1z0)+(vgb*v20y))-v20h));let v229=(sf[249]*(((vgk*v1z3)+(vgb*v211))-v20k));let v22a=(sf[249]*(((vgk*v1z6)+(vgb*v214))-v20n));let v22b=(sf[249]*(((vgk*v1z9)+(vgb*v217))-v20q));let v22d=(v9l*v9l);let v22e=((-v17z)/v22d);let v22g=((-v180)/v22d);let v22i=((-v181)/v22d);let v22k=((-v182)/v22d);let v22m=(v9p*v9p);let v22n=((-v18k)/v22m);let v22p=((-v18l)/v22m);let v22r=((-v18m)/v22m);let v22t=((-v18n)/v22m);let v24i=(((vgu*v22e)+(vgr*(v21o+((vgf*v16w)+(v9a*v205)))))-((vgy*v22n)+(vgs*(v21o+((vgw*v183)+(v9m*(-v205)))))));let v24j=(((vgu*v22g)+(vgr*(v21p+((vgf*v16x)+(v9a*v208)))))-((vgy*v22p)+(vgs*(v21p+((vgw*v184)+(v9m*(-v208)))))));let v24k=(((vgu*v22i)+(vgr*(v21q+((vgf*v16y)+(v9a*v20b)))))-((vgy*v22r)+(vgs*(v21q+((vgw*v185)+(v9m*(-v20b)))))));let v24l=(((vgu*v22k)+(vgr*(v21r+((vgf*v16z)+(v9a*v20e)))))-((vgy*v22t)+(vgs*(v21r+((vgw*v186)+(v9m*(-v20e)))))));let v26a=(((vh2*v22e)+(vgr*(v228+((vgg*v16w)+(v9a*v20h)))))-((vh6*v22n)+(vgs*(v228+((vh4*v183)+(v9m*(-v20h)))))));let v26b=(((vh2*v22g)+(vgr*(v229+((vgg*v16x)+(v9a*v20k)))))-((vh6*v22p)+(vgs*(v229+((vh4*v184)+(v9m*(-v20k)))))));let v26c=(((vh2*v22i)+(vgr*(v22a+((vgg*v16y)+(v9a*v20n)))))-((vh6*v22r)+(vgs*(v22a+((vh4*v185)+(v9m*(-v20n)))))));let v26d=(((vh2*v22k)+(vgr*(v22b+((vgg*v16z)+(v9a*v20q)))))-((vh6*v22t)+(vgs*(v22b+((vh4*v186)+(v9m*(-v20q)))))));let v271=(vhd*vhd);let v272=(((vhd*(sf[181]*v16j))-(vhb*((vhc*v169)+(v93*(vfv*v199)))))/v271);let v276=(((vhd*(sf[181]*v16k))-(vhb*((vhc*v16c)+(v93*(vfv*v19a)))))/v271);let v27a=(((vhd*(sf[181]*v16l))-(vhb*((vhc*v16f)+(v93*(vfv*v19b)))))/v271);let v27e=(((vhd*(sf[181]*v16m))-(vhb*((vhc*v16i)+(v93*(vfv*v19c)))))/v271);let v27h=((vhe*v1yo)+(vg9*v272));let v27k=((vhe*v1yr)+(vg9*v276));let v27n=((vhe*v1yu)+(vg9*v27a));let v27q=((vhe*v1yx)+(vg9*v27e));let v27t=((vhe*v1z0)+(vgb*v272));let v27w=((vhe*v1z3)+(vgb*v276));let v27z=((vhe*v1z6)+(vgb*v27a));let v282=((vhe*v1z9)+(vgb*v27e));let v283=(sf[183]*v1fr);let v284=(sf[183]*v1fs);let v285=(sf[183]*v1ft);let v286=(sf[183]*v1fu);let v288=(va4*va4);let v289=((-v19y)/v288);let v28b=((-v19z)/v288);let v28d=((-v1a0)/v288);let v28f=((-v1a1)/v288);let v28h=(va7*va7);let v28i=((-v1af)/v28h);let v28k=((-v1ag)/v28h);let v28m=((-v1ah)/v28h);let v28o=((-v1ai)/v28h);let v2d7=(vi7*vi7);let v2d8=((-(sf[6]*v1g7))/v2d7);let v2db=((-(sf[6]*v1g8))/v2d7);let v2de=((-(sf[6]*v1g9))/v2d7);let v2dh=((-(sf[6]*v1ga))/v2d7);
        let v2ef=(vc6*vc6);let v2eg=((-v1hb)/v2ef);let v2ei=((-v1hc)/v2ef);let v2ek=((-v1hd)/v2ef);let v2em=((-v1he)/v2ef);let v2fz=(sf[183]*v1mi);let v2g0=(sf[183]*v1mj);let v2g1=(sf[183]*v1mk);let v2g2=(sf[183]*v1ml);let v2gz=(((vdw*(ve7*v1oc))-(vis*v1nb))/v2gy);let v2h3=(((vdw*(ve7*v1od))-(vis*v1nd))/v2gy);let v2h7=(((vdw*(ve7*v1oe))-(vis*v1nf))/v2gy);let v2hb=(((vdw*(ve7*v1of))-(vis*v1nh))/v2gy);let v2hm=((viv*v2gz)+(vit*(v1my+(v5v*v1n2))));let v2hp=((viv*v2h3)+(vit*(v1mz+(v5v*v1n3))));let v2hs=((viv*v2h7)+(vit*(v1n0+(v5v*v1n4))));let v2hv=((viv*v2hb)+(vit*(v1n1+(v5v*v1n5))));let v2i6=((viy*v2gz)+(vit*(v1n2+(v5v*v1my))));let v2i9=((viy*v2h3)+(vit*(v1n3+(v5v*v1mz))));let v2ic=((viy*v2h7)+(vit*(v1n4+(v5v*v1n0))));let v2if=((viy*v2hb)+(vit*(v1n5+(v5v*v1n1))));let v2jb=(vj4*vj4);let v2jc=(((vj4*((vj0*v1pm)+(vef*v1o8)))-(vj1*((vj3*v112)+(vdy*(v1nt+v1nt)))))/v2jb);let v2jg=(((vj4*((vj0*v1pp)+(vef*v1o9)))-(vj1*((vj3*v113)+(vdy*(v1nw+v1nw)))))/v2jb);let v2jk=(((vj4*((vj0*v1ps)+(vef*v1oa)))-(vj1*((vj3*v114)+(vdy*(v1nz+v1nz)))))/v2jb);let v2jo=(((vj4*((vj0*v1pv)+(vef*v1ob)))-(vj1*((vj3*v115)+(vdy*(v1o2+v1o2)))))/v2jb);let v2kt=((((vj5*v1xw)+(vg6*v2jc))+((viw*v1yo)+(vg9*v2hm)))+((viz*((vip*v2fz)+(vio*v1xw)))+(viq*v2i6)));let v2ku=((((vj5*v1xz)+(vg6*v2jg))+((viw*v1yr)+(vg9*v2hp)))+((viz*((vip*v2g0)+(vio*v1xz)))+(viq*v2i9)));let v2kv=((((vj5*v1y2)+(vg6*v2jk))+((viw*v1yu)+(vg9*v2hs)))+((viz*((vip*v2g1)+(vio*v1y2)))+(viq*v2ic)));let v2kw=((((vj5*v1y5)+(vg6*v2jo))+((viw*v1yx)+(vg9*v2hv)))+((viz*((vip*v2g2)+(vio*v1y5)))+(viq*v2if)));let v2m1=((((vj5*v1y8)+(vg7*v2jc))+((viw*v1z0)+(vgb*v2hm)))+((viz*((vio*v1y8)+(vg7*v2fz)))+(vir*v2i6)));let v2m2=((((vj5*v1yb)+(vg7*v2jg))+((viw*v1z3)+(vgb*v2hp)))+((viz*((vio*v1yb)+(vg7*v2g0)))+(vir*v2i9)));let v2m3=((((vj5*v1ye)+(vg7*v2jk))+((viw*v1z6)+(vgb*v2hs)))+((viz*((vio*v1ye)+(vg7*v2g1)))+(vir*v2ic)));let v2m4=((((vj5*v1yh)+(vg7*v2jo))+((viw*v1z9)+(vgb*v2hv)))+((viz*((vio*v1yh)+(vg7*v2g2)))+(vir*v2if)));let v2mo=(vjh*vjh);let v2n2=(v1nt-(((vjh*v1pm)-(vef*((vjg*v112)+(vdy*(v5v*v1nt)))))/v2mo));let v2n3=(v1nw-(((vjh*v1pp)-(vef*((vjg*v113)+(vdy*(v5v*v1nw)))))/v2mo));let v2n4=(v1nz-(((vjh*v1ps)-(vef*((vjg*v114)+(vdy*(v5v*v1nz)))))/v2mo));let v2n5=(v1o2-(((vjh*v1pv)-(vef*((vjg*v115)+(vdy*(v5v*v1o2)))))/v2mo));let v2n6=(-v1o4);let v2n7=(-v1o5);let v2n8=(-v1o6);let v2n9=(-v1o7);let v2p9=(vjr*vjr);let v2pn=(if (sf[101]!=0.0){(((vjr*v1r9)-(vev*((vex*v1qt)+(veq*v1rd))))/v2p9)}else{v2n2});let v2po=(if (sf[101]!=0.0){(((vjr*v1ra)-(vev*((vex*v1qu)+(veq*v1re))))/v2p9)}else{v2n3});let v2pp=(if (sf[101]!=0.0){(((vjr*v1rb)-(vev*((vex*v1qv)+(veq*v1rf))))/v2p9)}else{v2n4});let v2pq=(if (sf[101]!=0.0){(((vjr*v1rc)-(vev*((vex*v1qw)+(veq*v1rg))))/v2p9)}else{v2n5});let v2qn=(-((vih*v2eg)+(vid*((-((vi9*v2d8)+(vi8*(-v24i))))+(sf[221]*v24i)))));let v2qo=(-((vih*v2ei)+(vid*((-((vi9*v2db)+(vi8*(-v24j))))+(sf[221]*v24j)))));let v2qp=(-((vih*v2ek)+(vid*((-((vi9*v2de)+(vi8*(-v24k))))+(sf[221]*v24k)))));let v2qq=(-((vih*v2em)+(vid*((-((vi9*v2dh)+(vi8*(-v24l))))+(sf[221]*v24l)))));let v2qz=(-((vim*v2eg)+(vid*((-((vib*v2d8)+(vi8*(-v26a))))+(sf[221]*v26a)))));let v2r0=(-((vim*v2ei)+(vid*((-((vib*v2db)+(vi8*(-v26b))))+(sf[221]*v26b)))));let v2r1=(-((vim*v2ek)+(vid*((-((vib*v2de)+(vi8*(-v26c))))+(sf[221]*v26c)))));let v2r2=(-((vim*v2em)+(vid*((-((vib*v2dh)+(vi8*(-v26d))))+(sf[221]*v26d)))));let v2rd=(vfd*vfd);let v2ro=(if sb[12]{((-(sf[16]*v1su))/v2rd)}else{v2pn});let v2rp=(if sb[12]{((-(sf[16]*v1sv))/v2rd)}else{v2po});let v2rq=(if sb[12]{((-(sf[16]*v1sw))/v2rd)}else{v2pp});let v2rr=(if sb[12]{((-(sf[16]*v1sx))/v2rd)}else{v2pq});let v2u6=(vkj*vkj);let v2u7=((-(sf[104]*((vki*v112)+(vfl*((vkh*v1tr)+(vfm*(vfv*v1u2)))))))/v2u6);let v2ua=((-(sf[104]*((vki*v113)+(vfl*((vkh*v1ts)+(vfm*(vfv*v1u5)))))))/v2u6);let v2ud=((-(sf[104]*((vki*v114)+(vfl*((vkh*v1tt)+(vfm*(vfv*v1u8)))))))/v2u6);let v2ug=((-(sf[104]*((vki*v115)+(vfl*((vkh*v1tu)+(vfm*(vfv*v1ub)))))))/v2u6);
        let v2xh=((sf[109]*((vkw*(-v1um))+(vks*((v1z0+((vkt*v1uc)+(vfq*((if sb[12]{(v2qz+((vkc*v2ro)+(vk5*(((vjp*v2n6)+(vjk*(v2m1+((vjj*v1y8)+(vg7*v2n2)))))+(sf[21]*v2m1)))))}else{(if (sf[101]!=0.0){(v2qz-(if (sf[101]!=0.0){((vjt*v1y8)+(vg7*v2pn))}else{v1}))}else{v1})})+((vkk*v1y8)+(vg7*v2u7))))))-((vi4*v283)+(vhh*((v1y8-((vhx*v289)+(vhi*(v228+((vhg*v19d)+(va0*v27t))))))+((vi2*v28i)+(vhj*(v228+((vi0*v19h)+(va1*(-v27t)))))))))))))+(sf[109]*((vkq*v1um)+(vfs*((v1yo+((vkn*v1uc)+(vfq*((if sb[12]{(v2qn+((vk7*v2ro)+(vk5*(((vjm*v2n6)+(vjk*(v2kt+((vjj*v1xw)+(vg6*v2n2)))))+(sf[21]*v2kt)))))}else{(if (sf[101]!=0.0){(v2qn-(if (sf[101]!=0.0){((vjt*v1xw)+(vg6*v2pn))}else{v1}))}else{v1})})+((vkk*v1xw)+(vg6*v2u7))))))-((vht*v283)+(vhh*((v1xw-((vhm*v289)+(vhi*(v21o+((vhf*v19d)+(va0*v27h))))))+((vhr*v28i)+(vhj*(v21o+((vhp*v19h)+(va1*(-v27h))))))))))))));let v2xi=((sf[109]*((vkw*(-v1up))+(vks*((v1z3+((vkt*v1ud)+(vfq*((if sb[12]{(v2r0+((vkc*v2rp)+(vk5*(((vjp*v2n7)+(vjk*(v2m2+((vjj*v1yb)+(vg7*v2n3)))))+(sf[21]*v2m2)))))}else{(if (sf[101]!=0.0){(v2r0-(if (sf[101]!=0.0){((vjt*v1yb)+(vg7*v2po))}else{v1}))}else{v1})})+((vkk*v1yb)+(vg7*v2ua))))))-((vi4*v284)+(vhh*((v1yb-((vhx*v28b)+(vhi*(v229+((vhg*v19e)+(va0*v27w))))))+((vi2*v28k)+(vhj*(v229+((vi0*v19i)+(va1*(-v27w)))))))))))))+(sf[109]*((vkq*v1up)+(vfs*((v1yr+((vkn*v1ud)+(vfq*((if sb[12]{(v2qo+((vk7*v2rp)+(vk5*(((vjm*v2n7)+(vjk*(v2ku+((vjj*v1xz)+(vg6*v2n3)))))+(sf[21]*v2ku)))))}else{(if (sf[101]!=0.0){(v2qo-(if (sf[101]!=0.0){((vjt*v1xz)+(vg6*v2po))}else{v1}))}else{v1})})+((vkk*v1xz)+(vg6*v2ua))))))-((vht*v284)+(vhh*((v1xz-((vhm*v28b)+(vhi*(v21p+((vhf*v19e)+(va0*v27k))))))+((vhr*v28k)+(vhj*(v21p+((vhp*v19i)+(va1*(-v27k))))))))))))));let v2xj=((sf[109]*((vkw*(-v1us))+(vks*((v1z6+((vkt*v1ue)+(vfq*((if sb[12]{(v2r1+((vkc*v2rq)+(vk5*(((vjp*v2n8)+(vjk*(v2m3+((vjj*v1ye)+(vg7*v2n4)))))+(sf[21]*v2m3)))))}else{(if (sf[101]!=0.0){(v2r1-(if (sf[101]!=0.0){((vjt*v1ye)+(vg7*v2pp))}else{v1}))}else{v1})})+((vkk*v1ye)+(vg7*v2ud))))))-((vi4*v285)+(vhh*((v1ye-((vhx*v28d)+(vhi*(v22a+((vhg*v19f)+(va0*v27z))))))+((vi2*v28m)+(vhj*(v22a+((vi0*v19j)+(va1*(-v27z)))))))))))))+(sf[109]*((vkq*v1us)+(vfs*((v1yu+((vkn*v1ue)+(vfq*((if sb[12]{(v2qp+((vk7*v2rq)+(vk5*(((vjm*v2n8)+(vjk*(v2kv+((vjj*v1y2)+(vg6*v2n4)))))+(sf[21]*v2kv)))))}else{(if (sf[101]!=0.0){(v2qp-(if (sf[101]!=0.0){((vjt*v1y2)+(vg6*v2pp))}else{v1}))}else{v1})})+((vkk*v1y2)+(vg6*v2ud))))))-((vht*v285)+(vhh*((v1y2-((vhm*v28d)+(vhi*(v21q+((vhf*v19f)+(va0*v27n))))))+((vhr*v28m)+(vhj*(v21q+((vhp*v19j)+(va1*(-v27n))))))))))))));let v2xk=((sf[109]*((vkw*(-v1uv))+(vks*((v1z9+((vkt*v1uf)+(vfq*((if sb[12]{(v2r2+((vkc*v2rr)+(vk5*(((vjp*v2n9)+(vjk*(v2m4+((vjj*v1yh)+(vg7*v2n5)))))+(sf[21]*v2m4)))))}else{(if (sf[101]!=0.0){(v2r2-(if (sf[101]!=0.0){((vjt*v1yh)+(vg7*v2pq))}else{v1}))}else{v1})})+((vkk*v1yh)+(vg7*v2ug))))))-((vi4*v286)+(vhh*((v1yh-((vhx*v28f)+(vhi*(v22b+((vhg*v19g)+(va0*v282))))))+((vi2*v28o)+(vhj*(v22b+((vi0*v19k)+(va1*(-v282)))))))))))))+(sf[109]*((vkq*v1uv)+(vfs*((v1yx+((vkn*v1uf)+(vfq*((if sb[12]{(v2qq+((vk7*v2rr)+(vk5*(((vjm*v2n9)+(vjk*(v2kw+((vjj*v1y5)+(vg6*v2n5)))))+(sf[21]*v2kw)))))}else{(if (sf[101]!=0.0){(v2qq-(if (sf[101]!=0.0){((vjt*v1y5)+(vg6*v2pq))}else{v1}))}else{v1})})+((vkk*v1y5)+(vg6*v2ug))))))-((vht*v286)+(vhh*((v1y5-((vhm*v28f)+(vhi*(v21r+((vhf*v19g)+(va0*v27q))))))+((vhr*v28o)+(vhj*(v21r+((vhp*v19k)+(va1*(-v27q))))))))))))));let v2xm=(vl6*vl6);let v2xw=((vl7*((vfs*v1uc)+(vfq*v1um)))+(vft*((-v2xh)/v2xm)));let v2xz=((vl7*((vfs*v1ud)+(vfq*v1up)))+(vft*((-v2xi)/v2xm)));let v2y2=((vl7*((vfs*v1ue)+(vfq*v1us)))+(vft*((-v2xj)/v2xm)));let v2y5=((vl7*((vfs*v1uf)+(vfq*v1uv)))+(vft*((-v2xk)/v2xm)));let v2y7=(sf[13]*v16x);let v2ya=(v170-(sf[13]*v16w));let v2yc=(v171-(sf[13]*v16y));let v2yd=(v172-(sf[13]*v16z));let v2yf=(vla*vla);let v38x=ddt_scale;let v39t=(vr4*vr4);let v3ac=(vrf*vrf);let v3ax=(vrr*vrr);let v3bm=(vso*vso);let v3c1=(vsx*vsx);let v3ci=(vt7*vt7);let v3gg=(sf[20]*(v34a*v38x));let v3gh=(sf[20]*(v34d*v38x));let v3gi=(sf[20]*(v34g*v38x));let v3gj=(sf[20]*(v34j*v38x));let v3go=(sf[20]*(v36m*v38x));let v3gp=(sf[20]*(v36p*v38x));
        let v3gq=(sf[20]*(v36s*v38x));let v3gr=(sf[20]*(v36v*v38x));let v3gw=(sf[20]*(if vlw{v1}else{(if (vle!=0.0){((vlt*v2xw)+(vl8*(if (vle!=0.0){((vlr*(if (vle!=0.0){(vlp*(if vln{v1}else{(if (vle!=0.0){(sf[250]*(if (vle!=0.0){((-v2ya)/v2yf)}else{v1}))}else{v1})}))}else{v1}))+(vlq*(sf[224]*v2ya)))}else{v1})))}else{v1})}));let v3gx=(sf[20]*(if vlw{v1}else{(if (vle!=0.0){((vlt*v2xz)+(vl8*(if (vle!=0.0){((vlr*(if (vle!=0.0){(vlp*(if vln{v1}else{(if (vle!=0.0){(sf[250]*(if (vle!=0.0){(v2y7/v2yf)}else{v1}))}else{v1})}))}else{v1}))+(vlq*(sf[224]*(-v2y7))))}else{v1})))}else{v1})}));let v3gy=(sf[20]*(if vlw{v1}else{(if (vle!=0.0){((vlt*v2y2)+(vl8*(if (vle!=0.0){((vlr*(if (vle!=0.0){(vlp*(if vln{v1}else{(if (vle!=0.0){(sf[250]*(if (vle!=0.0){((-v2yc)/v2yf)}else{v1}))}else{v1})}))}else{v1}))+(vlq*(sf[224]*v2yc)))}else{v1})))}else{v1})}));let v3gz=(sf[20]*(if vlw{v1}else{(if (vle!=0.0){((vlt*v2y5)+(vl8*(if (vle!=0.0){((vlr*(if (vle!=0.0){(vlp*(if vln{v1}else{(if (vle!=0.0){(sf[250]*(if (vle!=0.0){((-v2yd)/v2yf)}else{v1}))}else{v1})}))}else{v1}))+(vlq*(sf[224]*v2yd)))}else{v1})))}else{v1})}));

        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * ((vl8*vw0)),
            &[(vw0*v2xw),(vw0*v2xz),(vw0*v2y2),(vw0*v2y5)],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * ((if (vnh!=0.0){vw2}else{v1})),
            &[(if (vnh!=0.0){v3gg}else{v1}),(if (vnh!=0.0){v3gh}else{v1}),(if (vnh!=0.0){v3gi}else{v1}),(if (vnh!=0.0){v3gj}else{v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * ((if (vnh!=0.0){vw4}else{v1})),
            &[(if (vnh!=0.0){v3go}else{v1}),(if (vnh!=0.0){v3gp}else{v1}),(if (vnh!=0.0){v3gq}else{v1}),(if (vnh!=0.0){v3gr}else{v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * ((if (vnh!=0.0){vw6}else{v1})),
            &[(if (vnh!=0.0){v3gw}else{v1}),(if (vnh!=0.0){v3gx}else{v1}),(if (vnh!=0.0){v3gy}else{v1}),(if (vnh!=0.0){v3gz}else{v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * ((if vw8{vw2}else{v1})),
            &[(if vw8{v3gg}else{v1}),(if vw8{v3gh}else{v1}),(if vw8{v3gi}else{v1}),(if vw8{v3gj}else{v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * ((if vw8{vw4}else{v1})),
            &[(if vw8{v3go}else{v1}),(if vw8{v3gp}else{v1}),(if vw8{v3gq}else{v1}),(if vw8{v3gr}else{v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * ((if vw8{vw6}else{v1})),
            &[(if vw8{v3gw}else{v1}),(if vw8{v3gx}else{v1}),(if vw8{v3gy}else{v1}),(if vw8{v3gz}else{v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(1),
            Some(3),
            multiplicity * ((sf[20]*vwc)),
            &[(sf[20]*(v38t*v38x)),(sf[20]*(v38u*v38x)),(sf[20]*(v38v*v38x)),(sf[20]*(v38w*v38x))],
            &[],
            multiplicity,
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (v1),
        );
        stamper.stamp_current_node2_local(
            Some(0),
            Some(3),
            multiplicity * ((sf[90]*(sf[20]*((((sf[291]*(vr6-vv))-(sf[286]*(vrh-vv)))-(sf[285]*(vrt-vv)))+((vqt*vwg)+(v5f*sf[168])))))),
            0,
            multiplicity * ((sf[90]*(sf[20]*((((sf[291]*(vr6*(((vr4*sf[312])-(vr0*(if vr3{sf[20]}else{v1})))/v39t)))-(sf[286]*(vrh*(((vrf*sf[316])-(vrc*(if vre{sf[20]}else{v1})))/v3ac))))-(sf[285]*(vrt*(((vrr*sf[320])-(vro*(if vrq{sf[20]}else{v1})))/v3ax))))+(((vwg*(if vqn{(sf[157]*(vqq*sf[308]))}else{v1}))+(vqt*(sf[289]*(-(vwe*(if (vqd!=0.0){v1}else{sf[306]}))))))+sf[176]))))),
            3,
            multiplicity * ((sf[90]*(sf[20]*((((sf[291]*(vr6*(((vr4*sf[313])-(vr0*(if vr3{sf[169]}else{v1})))/v39t)))-(sf[286]*(vrh*(((vrf*sf[317])-(vrc*(if vre{sf[169]}else{v1})))/v3ac))))-(sf[285]*(vrt*(((vrr*sf[321])-(vro*(if vrq{sf[169]}else{v1})))/v3ax))))+(((vwg*(if vqn{(sf[157]*(vqq*sf[309]))}else{v1}))+(vqt*(sf[289]*(-(vwe*(if (vqd!=0.0){v1}else{sf[307]}))))))+sf[177]))))),
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * ((sf[90]*(sf[20]*((((sf[291]*(vsq-vv))-(sf[296]*(vsz-vv)))-(sf[295]*(vt9-vv)))+((vsi*vwq)+(v5c*sf[168])))))),
            2,
            multiplicity * ((sf[90]*(sf[20]*((((sf[291]*(vsq*(((vso*sf[312])-(vsl*(if vsn{sf[20]}else{v1})))/v3bm)))-(sf[296]*(vsz*(((vsx*sf[316])-(vsu*(if vsw{sf[20]}else{v1})))/v3c1))))-(sf[295]*(vt9*(((vt7*sf[320])-(vt4*(if vt6{sf[20]}else{v1})))/v3ci))))+(sf[176]+((vwq*(if vsd{(sf[157]*(vsf*sf[308]))}else{v1}))+(vsi*(sf[298]*(-(vwo*(if (vs5!=0.0){v1}else{sf[306]}))))))))))),
            3,
            multiplicity * ((sf[90]*(sf[20]*((((sf[291]*(vsq*(((vso*sf[313])-(vsl*(if vsn{sf[169]}else{v1})))/v3bm)))-(sf[296]*(vsz*(((vsx*sf[317])-(vsu*(if vsw{sf[169]}else{v1})))/v3c1))))-(sf[295]*(vt9*(((vt7*sf[321])-(vt4*(if vt6{sf[169]}else{v1})))/v3ci))))+(sf[177]+((vwq*(if vsd{(sf[157]*(vsf*sf[309]))}else{v1}))+(vsi*(sf[298]*(-(vwo*(if (vs5!=0.0){v1}else{sf[307]}))))))))))),
        );
        stamper.stamp_current_node2_local(
            Some(0),
            Some(3),
            multiplicity * ((sf[90]*(sf[20]*vwx))),
            0,
            multiplicity * ((sf[90]*(sf[20]*(v38x*v3eu)))),
            3,
            multiplicity * ((sf[90]*(sf[20]*(v38x*v3ex)))),
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * ((sf[90]*(sf[20]*vx0))),
            2,
            multiplicity * ((sf[90]*(sf[20]*(v38x*v3g8)))),
            3,
            multiplicity * ((sf[90]*(sf[20]*(v38x*v3gb)))),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let CommonStampValues {
            v1, vp, vv, v5c, v5f, v5p, v5v, v64, 
            v67, v6b, v6e, v74, v76, v7a, v7e, v91, 
            v93, v94, v98, v9a, v9b, v9c, v9i, v9x, 
            va0, va1, va4, va7, vbq, vdo, vdr, vds, 
            vdt, vdu, vdv, vdw, vdx, vdy, ve0, veg, 
            vfv, viu, vix, vmo, vn1, vnd, vnh, vur, 
            vvz, vw8, vxr, vxs, vxt, vy1, vy2, vy3, 
            vyb, vyc, vyd, vyl, vym, vyn, vzj, vzk, 
            vzl, vzm, vzr, vzs, vzt, vzu, v10e, v10f, 
            v10g, v10h, v112, v113, v114, v115, v163, v164, 
            v165, v166, v169, v16c, v16f, v16i, v16j, v16k, 
            v16l, v16m, v16s, v16t, v16u, v16v, v16w, v16x, 
            v16y, v16z, v170, v171, v172, v173, v174, v175, 
            v17i, v17j, v17k, v17l, v199, v19a, v19b, v19c, 
            v19d, v19e, v19f, v19g, v19h, v19i, v19j, v19k, 
            v19y, v19z, v1a0, v1a1, v1af, v1ag, v1ah, v1ai, 
            v1fr, v1fs, v1ft, v1fu, v1mi, v1mj, v1mk, v1ml, 
            v1mo, v1mr, v1mu, v1mx, v1my, v1mz, v1n0, v1n1, 
            v1n2, v1n3, v1n4, v1n5, v1n6, v1n7, v1n8, v1n9, 
            v1nb, v1nd, v1nf, v1nh, v1nm, v1nn, v1no, v1np, 
            v2gy, v34a, v34d, v34g, v34j, v36m, v36p, v36s, 
            v36v, v38t, v38u, v38v, v38w, v3eu, v3ex, v3g8, 
            v3gb, 
        }=self.eval_common_stamp_values(ctx);
        let vne=0.0;let vnf=0.0;let vw2=(sf[20]*vne);let vw4=(sf[20]*vnf);let vwc=0.0;let vwx=0.0;let vx0=0.0;let v38x=1.0;let v3gg=(sf[20]*(v34a*v38x));let v3gh=(sf[20]*(v34d*v38x));let v3gi=(sf[20]*(v34g*v38x));let v3gj=(sf[20]*(v34j*v38x));let v3go=(sf[20]*(v36m*v38x));let v3gp=(sf[20]*(v36p*v38x));let v3gq=(sf[20]*(v36s*v38x));let v3gr=(sf[20]*(v36v*v38x));

        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[3]),
            &nodes,
            &[(if (vnh!=0.0){v3gg}else{v1}),(if (vnh!=0.0){v3gh}else{v1}),(if (vnh!=0.0){v3gi}else{v1}),(if (vnh!=0.0){v3gj}else{v1})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[3]),
            &nodes,
            &[(if (vnh!=0.0){v3go}else{v1}),(if (vnh!=0.0){v3gp}else{v1}),(if (vnh!=0.0){v3gq}else{v1}),(if (vnh!=0.0){v3gr}else{v1})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[3]),
            &nodes,
            &[(if vw8{v3gg}else{v1}),(if vw8{v3gh}else{v1}),(if vw8{v3gi}else{v1}),(if vw8{v3gj}else{v1})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[3]),
            &nodes,
            &[(if vw8{v3go}else{v1}),(if vw8{v3gp}else{v1}),(if vw8{v3gq}else{v1}),(if vw8{v3gr}else{v1})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[3]),
            &nodes,
            &[(sf[20]*(v38t*v38x)),(sf[20]*(v38u*v38x)),(sf[20]*(v38v*v38x)),(sf[20]*(v38w*v38x))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[3]),
            nodes[0],
            multiplicity * ((sf[90]*(sf[20]*(v38x*v3eu)))),
            nodes[3],
            multiplicity * ((sf[90]*(sf[20]*(v38x*v3ex)))),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[2]),
            Some(nodes[3]),
            nodes[2],
            multiplicity * ((sf[90]*(sf[20]*(v38x*v3g8)))),
            nodes[3],
            multiplicity * ((sf[90]*(sf[20]*(v38x*v3gb)))),
        );
    }
}
