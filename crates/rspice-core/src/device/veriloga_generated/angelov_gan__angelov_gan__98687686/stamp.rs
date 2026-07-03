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
    v0: f64, v1: f64, v3: f64, v5: f64, v6: f64, v7: f64, 
    v8: f64, v9: f64, vb: f64, vd: f64, vw: f64, vz: f64, 
    v13: f64, v18: f64, v19: f64, v1b: f64, v2c: f64, v3c: bool, 
    v3g: f64, v3n: bool, v3s: bool, v3y: f64, v40: f64, v45: f64, 
    v4b: f64, v4q: f64, v4z: f64, v54: f64, v5e: f64, v5m: f64, 
    v5v: f64, v6u: f64, v6v: f64, v6x: f64, v8s: f64, v9f: f64, 
    va6: f64, vcl: f64, vcz: f64, vg1: f64, vg7: f64, vis: f64, 
    vix: f64, vjz: f64, vkf: f64, vlu: f64, vm0: f64, vma: f64, 
    vmd: f64, vms: f64, vmu: f64, vmw: f64, vmx: f64, vn0: f64, 
    vn1: f64, vn8: f64, vnc: f64, vne: f64, vnu: f64, vo0: f64, 
    vo6: f64, vob: f64, voj: f64, vok: f64, vox: f64, vpl: f64, 
    vpm: f64, vq6: f64, vqb: f64, vqd: f64, vqf: f64, vqh: f64, 
    vqi: f64, vrr: f64, vrs: f64, vrt: f64, vru: f64, vrv: f64, 
    vuk: f64, vul: f64, vum: f64, vun: f64, vuo: f64, vup: f64, 
    vuq: f64, vuz: f64, vv0: f64, vv1: f64, vv2: f64, v14e: f64, 
    v14f: f64, v14g: f64, v14h: f64, v14i: f64, v176: f64, v177: f64, 
    v178: f64, v179: f64, v1by: f64, v1bz: f64, v1c0: f64, v1c1: f64, 
    v1c2: f64, v1si: f64, v1u4: f64, v1yb: f64, v1yc: f64, v1yd: f64, 
    v1ye: f64, v1zq: f64, v1zr: f64, v1zs: f64, v1zt: f64, v20n: f64, 
    v20p: f64, v20q: f64, v20y: f64, v210: f64, v212: f64, v21c: f64, 
    v21l: f64, v21m: f64, v21n: f64, v21o: f64, v21p: f64, v21v: f64, 
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let v0=ctx.node_voltage(nodes[12]);let v1=ctx.node_voltage(nodes[8]);let v2=(v0-v1);let v3=ctx.node_voltage(nodes[10]);let v4=ctx.node_voltage(nodes[5]);let v5=(v3-v4);let v6=(-v5);let v7=(v4-v1);let v8=ctx.node_voltage(nodes[11]);let v9=(v8-v1);let va=ctx.node_voltage(nodes[4]);let vb=(va-v1);let vd=0.0;let vw=ctx.node_voltage(nodes[3]);let vz=(if (sf[10]!=0.0){(sf[218]+(vw).abs())}else{sf[218]});let v13=((vz-sf[9])).abs();let v18=1.0;let v19=(if ((v13>vd)||sb[2]){v18}else{vd});let v1b=(v13).abs();
        let v2c=(v18+(v1b*sf[24]));let v2n=(v18+(v13*sf[28]));let v3c=((v19!=0.0)&&(sf[36]!=0.0));let v3g=(v18+(sf[24]*(v13*v13)));let v3n=((v19!=0.0)&&sb[8]);let v3s=(!(v19!=0.0));let v3v=(if v3s{sf[17]}else{(if (v19!=0.0){(sf[17]*(v18+(v1b*sf[18])))}else{vd})});let v3w=(if v3s{sf[19]}else{(if (v19!=0.0){(sf[19]*(v18+(v1b*sf[20])))}else{vd})});let v3x=(if v3s{sf[21]}else{(if (v19!=0.0){(sf[21]*(v18+(v1b*sf[22])))}else{vd})});let v3y=(if v3s{sf[23]}else{(if (v19!=0.0){(sf[23]*v2c)}else{vd})});
        let v40=(if v3s{sf[38]}else{(if v3n{(v2c*sf[38])}else{(if v3c{(v3g*sf[38])}else{vd})})});let v42=(if v3s{sf[27]}else{(if (v19!=0.0){(sf[27]*v2n)}else{vd})});let v43=(if v3s{sf[29]}else{(if (v19!=0.0){(v2n*sf[29])}else{vd})});let v45=(if v3s{sf[32]}else{(if (v19!=0.0){(sf[32]+(v13*sf[33]))}else{vd})});let v4b=0.5;let v4k=(v7*sf[45]);let v4l=(v4k).cosh();let v4q=1e-12;let v4s=(v4q+(v4l*v4l));let v4y=(v18+(v1b*sf[49]));let v4z=((sf[47]*(v18+(sf[48]/v4s)))*v4y);let v54=(sf[50]*(v18+(v1b*sf[51])));
        let v59=((v7*sf[53])).tanh();let v5e=(v6-v45);let v5f=(sf[54]*v5e);let v5j=(v18+(v1b*sf[26]));let v5k=((((((if v3s{sf[25]}else{(if (v19!=0.0){(sf[25]+(v13*sf[26]))}else{vd})})-sf[52])+(sf[52]*v59))-(vb*sf[46]))-(v5e*v5f))*v5j);let v5l=(v2-v5k);let v5m=(v5l*v5l);let v5r=(v54*v5l);let v5t=(((v4z*v5l)+(v5m*sf[55]))+(v5m*v5r));let v5u=(v5t).tanh();let v5v=(v18+v5u);let v5x=(-v5t);let v61=((v4b*(scalar_limexp(v5t)-scalar_limexp(v5x)))).tanh();let v6b=2.0;let v6u=(v5-v5k);
        let v6v=(if sb[16]{v6u}else{v4l});let v6x=(if sb[16]{(v6v*v6v)}else{v5l});let v8b=(if sb[19]{v5l}else{v6v});let v8d=(if sb[19]{(v8b*v8b)}else{v6x});let v8g=(v54*v8d);let v8i=((v8b+(sf[55]*v8d))+(v8b*v8g));let v8k=(if sb[19]{(v4z*v8i)}else{v5t});let v8m=(-v8k);let v8q=((v4b*(scalar_limexp(v8k)-scalar_limexp(v8m)))).tanh();let v8s=(if sb[19]{(v18+v8q)}else{(v18+v61)});let v9f=(if sb[22]{v5l}else{v8b});let v9h=(if sb[22]{(v9f*v9f)}else{v8d});let v9k=(v54*v9h);let v9m=((v9f+(sf[55]*v9h))+(v9f*v9k));
        let v9o=(if sb[22]{(v4z*v9m)}else{v8k});let va0=(-v9o);let va4=((v4b*(scalar_limexp(v9o)-scalar_limexp(va0)))).tanh();let va6=(if sb[22]{(v18+va4)}else{v8s});let vc5=(v18+v5v);let vci=(v18+va6);let vcl=(if sb[28]{(sf[67]+(v3x/vci))}else{(if (sf[66]!=0.0){(sf[67]+(v3x/vc5))}else{vd})});let vcz=-1.0;let vew=(v7*sf[89]);let vex=((v42+(v9*sf[88]))+vew);let vey=(vex).tanh();let vez=(v18+vey);let vf4=((sf[90]+(v7*sf[91]))).tanh();let vf5=(v18+vf4);let vfa=((sf[92]-(v7*sf[93]))).tanh();
        let vfc=((v18+vfa)-sf[89]);let vfg=((v43+(v5*sf[94]))-vew);let vfh=(vfg).tanh();let vfi=(v18+vfh);let vfy=(v3v*vez);let vg1=(if sb[40]{(sf[100]+(vf5*vfy))}else{sf[101]});let vg7=(if sb[40]{(sf[102]+(v3w*((vfc*vfi)+sf[104])))}else{sf[103]});let vgc=(if sb[43]{(vf5-sf[89])}else{vf5});let vgd=(v42+vew);let vge=(vgd).cosh();let vgf=(if sb[43]{vge}else{vd});let vgh=(if sb[43]{(vgf).ln()}else{vd});let vgi=(vex).cosh();let vgj=(if sb[43]{vgi}else{vd});let vgl=(if sb[43]{(vgj).ln()}else{vd});
        let vgn=(if sb[43]{(vgd+vgh)}else{vd});let vgp=((vex+vgl)-vgn);let vgs=(v9*sf[104]);let vgv=(v9*sf[100]);let vgy=(v43-vew);let vgz=(vgy).cosh();let vh0=(if sb[43]{vgz}else{vgf});let vh2=(if sb[43]{(vh0).ln()}else{vd});let vh3=(vfg).cosh();let vh4=(if sb[43]{vh3}else{vgj});let vh6=(if sb[43]{(vh4).ln()}else{vd});let vh8=(if sb[43]{(vgy+vh2)}else{vd});let vha=((vfg+vh6)-vh8);let vhd=(v5*sf[104]);let vhg=(v5*sf[102]);let vhr=(v9/sf[105]);let vht=(if sb[46]{(vhr-v18)}else{vd});let vhw=(vht*vht);
        let vhx=(sf[107]+vhw);let vhz=f64::powf(vhx,sf[108]);let vi3=(sf[107]+(vhw*sf[110]));let vi9=((v42+(sf[88]*(v9+vew)))).tanh();let vic=(if sb[46]{vf5}else{vgc});let vie=(vfa+sf[111]);let vif=(if sb[46]{vie}else{vfc});let vik=((v43+(sf[94]*(v5+(v7*sf[111]))))).tanh();let vim=(if sb[46]{(v18+vik)}else{vfi});let viq=(v3v*((if sb[46]{(v18+vi9)}else{vez})+((if sb[46]{(vhz*vi3)}else{vd})*sf[112])));let vis=(sf[100]+(vic*viq));let vix=(sf[102]+(v3w*(sf[104]+(vif*vim))));let vj2=(if sb[49]{vge}else{vh0});
        let vj5=(if sb[49]{vgi}else{vh4});let vja=(sf[112]*(v9+sf[105]));let vjb=(vcz+vhr);let vjd=(sf[107]+f64::powf(vjb,v6b));let vjf=f64::powf(vjd,sf[114]);let vjs=(((if sb[49]{(vja*vjf)}else{vd})+((vex+(if sb[49]{(vj5).ln()}else{vgl}))-(if sb[49]{(vgd+(if sb[49]{(vj2).ln()}else{vgh}))}else{vgn})))-sf[119]);let vjt=(vf4+sf[111]);let vjz=(if sb[49]{(vgv+(v3v*(vgs+((vjs*vjt)/sf[88]))))}else{(if sb[43]{((v3v*(((vgc*vgp)/sf[88])+vgs))+vgv)}else{vd})});let vk0=(if sb[49]{vgz}else{vj2});
        let vk3=(if sb[49]{vh3}else{vj5});let vk9=((vfg+(if sb[49]{(vk3).ln()}else{vh6}))-(if sb[49]{(vgy+(if sb[49]{(vk0).ln()}else{vh2}))}else{vh8}));let vkf=(if sb[49]{(vhg+(v3w*(vhd+((vie*vk9)/sf[94]))))}else{(if sb[43]{((v3w*(((vfc*vha)/sf[94])+vhd))+vhg)}else{vd})});let vlu=(if sb[67]{((v3v*((vz*5.5226012e-23)*sf[140]))*sf[142])}else{vd});let vm0=3.141592653589793;let vma=(sf[144]*ctx.node_voltage(nodes[15]));let vmd=(sf[145]*ctx.branch_current(branches[0]));
        let vms=(sf[146]*(ctx.node_voltage(nodes[7])-v4));let vmu=(v7*sf[147]);let vmw=(ctx.node_voltage(nodes[6])-va);let vmx=(v3y*vmw);let vmz=ctx.branch_current(branches[1]);let vn0=(vcl*vmz);let vn1=(sf[121]*vmz);let vn8=(v2*v40);let vnc=ctx.node_voltage(nodes[14]);let vne=(sf[148]*(v8-vnc));let vnu=(sf[149]*ctx.branch_current(branches[10]));let vo0=(sf[150]*ctx.branch_current(branches[14]));let vo6=(sf[151]*ctx.branch_current(branches[18]));let vob=ctx.node_voltage(nodes[17]);
        let voj=(-(if sb[67]{(vlu*vm0)}else{vd}));let vok=(vob*voj);let vox=(vw*sf[152]);let vp4=(v4k).sinh();let vp5=(sf[45]*vp4);let vp6=(sf[153]*vp4);let vp8=(v4l*vp5);let vpa=(v4l*vp6);let vpe=(v4s*v4s);let vpl=(v4y*(sf[47]*((-(sf[48]*(vp8+vp8)))/vpe)));let vpm=(v4y*(sf[47]*((-(sf[48]*(vpa+vpa)))/vpe)));let vpp=(v18-(v59*v59));let vq3=(v5j*((sf[52]*(sf[53]*vpp))-(v5f+v5f)));let vq4=(v5j*((sf[52]*(sf[155]*vpp))-sf[154]));let vq5=(v5j*(-((-v5f)+(v5e*sf[156]))));let vq6=(-(v5j*sf[154]));let vq7=(-vq3);
        let vq8=(vcz-vq4);let vq9=(-vq5);let vqa=(v5l*vq6);let vqb=(vqa+vqa);let vqc=(v5l*vq7);let vqd=(vqc+vqc);let vqe=(v5l*vq8);let vqf=(vqe+vqe);let vqg=(v5l*vq9);let vqh=(vqg+vqg);let vqi=(v5l+v5l);let vrk=(((v4z*vq6)+(sf[55]*vqb))+((v5r*vqb)+(v5m*(v54*vq6))));let vrl=((((v5l*vpl)+(v4z*vq7))+(sf[55]*vqd))+((v5r*vqd)+(v5m*(v54*vq7))));let vrm=((((v5l*vpm)+(v4z*vq8))+(sf[55]*vqf))+((v5r*vqf)+(v5m*(v54*vq8))));let vrn=(((v4z*vq9)+(sf[55]*vqh))+((v5r*vqh)+(v5m*(v54*vq9))));
        let vro=((v4z+(sf[55]*vqi))+((v5r*vqi)+(v54*v5m)));let vrq=(v18-(v5u*v5u));let vrr=(vrk*vrq);let vrs=(vrl*vrq);let vrt=(vrm*vrq);let vru=(vrn*vrq);let vrv=(vro*vrq);let vrw=scalar_limexp_derivative(v5t);let vs7=scalar_limexp_derivative(v5x);let vso=(v18-(v61*v61));let vuk=(vcz-vq3);let vul=(-vq4);let vum=(v18-vq5);let vun=(if sb[16]{vq6}else{vd});let vuo=(if sb[16]{vuk}else{vp5});let vup=(if sb[16]{vul}else{vp6});let vuq=(if sb[16]{vum}else{vd});let vur=(v6v*vun);let vut=(v6v*vuo);let vuv=(v6v*vup);
        let vux=(v6v*vuq);let vuz=(if sb[16]{(vur+vur)}else{vq6});let vv0=(if sb[16]{(vut+vut)}else{vq7});let vv1=(if sb[16]{(vuv+vuv)}else{vq8});let vv2=(if sb[16]{(vux+vux)}else{vq9});let v11j=(if sb[19]{vq6}else{vun});let v11k=(if sb[19]{vq7}else{vuo});let v11l=(if sb[19]{vq8}else{vup});let v11m=(if sb[19]{vq9}else{vuq});let v11o=(v8b*v11j);let v11q=(v8b*v11k);let v11s=(v8b*v11l);let v11u=(v8b*v11m);let v11w=(v8b*sf[161]);let v11y=(if sb[19]{(v11o+v11o)}else{vuz});
        let v11z=(if sb[19]{(v11q+v11q)}else{vv0});let v120=(if sb[19]{(v11s+v11s)}else{vv1});let v121=(if sb[19]{(v11u+v11u)}else{vv2});let v122=(if sb[19]{(v11w+v11w)}else{sf[158]});let v13b=(if sb[19]{(v4z*((v11j+(sf[55]*v11y))+((v8g*v11j)+(v8b*(v54*v11y)))))}else{vrk});let v13c=(if sb[19]{((v8i*vpl)+(v4z*((v11k+(sf[55]*v11z))+((v8g*v11k)+(v8b*(v54*v11z))))))}else{vrl});let v13d=(if sb[19]{((v8i*vpm)+(v4z*((v11l+(sf[55]*v120))+((v8g*v11l)+(v8b*(v54*v120))))))}else{vrm});
        let v13e=(if sb[19]{(v4z*((v11m+(sf[55]*v121))+((v8g*v11m)+(v8b*(v54*v121)))))}else{vrn});let v13f=(if sb[19]{(v4z*((sf[161]+(sf[55]*v122))+((v8g*sf[161])+(v8b*(v54*v122)))))}else{vro});let v13g=scalar_limexp_derivative(v8k);let v13r=scalar_limexp_derivative(v8m);let v148=(v18-(v8q*v8q));let v14e=(if sb[19]{((v4b*((v13b*v13g)-((-v13b)*v13r)))*v148)}else{((v4b*((vrk*vrw)-((-vrk)*vs7)))*vso)});let v14f=(if sb[19]{((v4b*((v13c*v13g)-((-v13c)*v13r)))*v148)}else{((v4b*((vrl*vrw)-((-vrl)*vs7)))*vso)});
        let v14g=(if sb[19]{((v4b*((v13d*v13g)-((-v13d)*v13r)))*v148)}else{((v4b*((vrm*vrw)-((-vrm)*vs7)))*vso)});let v14h=(if sb[19]{((v4b*((v13e*v13g)-((-v13e)*v13r)))*v148)}else{((v4b*((vrn*vrw)-((-vrn)*vs7)))*vso)});let v14i=(if sb[19]{((v4b*((v13f*v13g)-((-v13f)*v13r)))*v148)}else{((v4b*((vro*vrw)-((-vro)*vs7)))*vso)});let v176=(if sb[22]{vq6}else{v11j});let v177=(if sb[22]{vq7}else{v11k});let v178=(if sb[22]{vq8}else{v11l});let v179=(if sb[22]{vq9}else{v11m});let v17b=(v9f*v176);let v17d=(v9f*v177);
        let v17f=(v9f*v178);let v17h=(v9f*v179);let v17j=(v9f*sf[162]);let v17l=(if sb[22]{(v17b+v17b)}else{v11y});let v17m=(if sb[22]{(v17d+v17d)}else{v11z});let v17n=(if sb[22]{(v17f+v17f)}else{v120});let v17o=(if sb[22]{(v17h+v17h)}else{v121});let v17p=(if sb[22]{(v17j+v17j)}else{v122});let v18y=(if sb[22]{(v4z*((v176+(sf[55]*v17l))+((v9k*v176)+(v9f*(v54*v17l)))))}else{v13b});let v18z=(if sb[22]{((v9m*vpl)+(v4z*((v177+(sf[55]*v17m))+((v9k*v177)+(v9f*(v54*v17m))))))}else{v13c});
        let v190=(if sb[22]{((v9m*vpm)+(v4z*((v178+(sf[55]*v17n))+((v9k*v178)+(v9f*(v54*v17n))))))}else{v13d});let v191=(if sb[22]{(v4z*((v179+(sf[55]*v17o))+((v9k*v179)+(v9f*(v54*v17o)))))}else{v13e});let v192=(if sb[22]{(v4z*((sf[162]+(sf[55]*v17p))+((v9k*sf[162])+(v9f*(v54*v17p)))))}else{v13f});let v1b0=scalar_limexp_derivative(v9o);let v1bb=scalar_limexp_derivative(va0);let v1bs=(v18-(va4*va4));let v1by=(if sb[22]{((v4b*((v18y*v1b0)-((-v18y)*v1bb)))*v1bs)}else{v14e});
        let v1bz=(if sb[22]{((v4b*((v18z*v1b0)-((-v18z)*v1bb)))*v1bs)}else{v14f});let v1c0=(if sb[22]{((v4b*((v190*v1b0)-((-v190)*v1bb)))*v1bs)}else{v14g});let v1c1=(if sb[22]{((v4b*((v191*v1b0)-((-v191)*v1bb)))*v1bs)}else{v14h});let v1c2=(if sb[22]{((v4b*((v192*v1b0)-((-v192)*v1bb)))*v1bs)}else{v14i});let v1lv=(vc5*vc5);let v1mq=(vci*vci);let v1pr=(v18-(vey*vey));let v1ps=(sf[89]*v1pr);let v1pt=(sf[177]*v1pr);let v1pu=(sf[88]*v1pr);let v1px=(v18-(vf4*vf4));let v1py=(sf[91]*v1px);let v1pz=(sf[178]*v1px);
        let v1q2=(v18-(vfa*vfa));let v1q3=(sf[179]*v1q2);let v1q4=(sf[93]*v1q2);let v1q8=(v18-(vfh*vfh));let v1q9=(sf[181]*v1q8);let v1qa=(sf[89]*v1q8);let v1qb=(sf[94]*v1q8);let v1r2=(vgd).sinh();let v1r3=(sf[89]*v1r2);let v1r4=(sf[176]*v1r2);let v1r5=(if sb[43]{v1r3}else{vd});let v1r6=(if sb[43]{v1r4}else{vd});let v1r9=(if sb[43]{(v1r5/vgf)}else{vd});let v1ra=(if sb[43]{(v1r6/vgf)}else{vd});let v1rb=(vex).sinh();let v1rc=(sf[89]*v1rb);let v1rd=(sf[177]*v1rb);let v1re=(sf[88]*v1rb);
        let v1rf=(if sb[43]{v1rc}else{vd});let v1rg=(if sb[43]{v1rd}else{vd});let v1rh=(if sb[43]{v1re}else{vd});let v1rl=(if sb[43]{(v1rf/vgj)}else{vd});let v1rm=(if sb[43]{(v1rg/vgj)}else{vd});let v1rn=(if sb[43]{(v1rh/vgj)}else{vd});let v1rq=(if sb[43]{(sf[89]+v1r9)}else{vd});let v1rr=(if sb[43]{(sf[176]+v1ra)}else{vd});let v1si=(if sb[43]{(sf[100]+(v3v*(sf[104]+((vgc*(sf[88]+v1rn))/sf[88]))))}else{vd});let v1sj=(vgy).sinh();let v1sk=(sf[176]*v1sj);let v1sl=(sf[89]*v1sj);
        let v1sm=(if sb[43]{v1sk}else{v1r5});let v1sn=(if sb[43]{v1sl}else{v1r6});let v1sq=(if sb[43]{(v1sm/vh0)}else{vd});let v1sr=(if sb[43]{(v1sn/vh0)}else{vd});let v1ss=(vfg).sinh();let v1st=(sf[181]*v1ss);let v1su=(sf[89]*v1ss);let v1sv=(sf[94]*v1ss);let v1sw=(if sb[43]{v1st}else{v1rf});let v1sx=(if sb[43]{v1su}else{v1rg});let v1sy=(if sb[43]{v1sv}else{vd});let v1sz=(if sb[43]{vd}else{v1rh});let v1t4=(if sb[43]{(v1sw/vh4)}else{vd});let v1t5=(if sb[43]{(v1sx/vh4)}else{vd});
        let v1t6=(if sb[43]{(v1sy/vh4)}else{vd});let v1t7=(if sb[43]{(v1sz/vh4)}else{vd});let v1ta=(if sb[43]{(sf[176]+v1sq)}else{vd});let v1tb=(if sb[43]{(sf[89]+v1sr)}else{vd});let v1u4=(if sb[43]{(sf[102]+(v3w*(sf[104]+((vfc*(sf[94]+v1t6))/sf[94]))))}else{vd});let v1ug=(vht*sf[187]);let v1uh=(v1ug+v1ug);let v1ui=(vht*sf[188]);let v1uj=(v1ui+v1ui);let v1um=(sf[108]*f64::powf(vhx,sf[189]));let v1v3=(v18-(vi9*vi9));let v1vf=(v18-(vik*vik));let v1wg=(if sb[49]{v1r3}else{v1sm});
        let v1wh=(if sb[49]{v1r4}else{v1sn});let v1wm=(if sb[49]{v1rc}else{v1sw});let v1wn=(if sb[49]{v1rd}else{v1sx});let v1wo=(if sb[49]{vd}else{v1sy});let v1wp=(if sb[49]{v1re}else{v1sz});let v1x0=(v6b*f64::powf(vjb,v18));let v1x5=(sf[114]*f64::powf(vjd,sf[198]));
        let v1yb=(if sb[49]{(v3v*(((vjt*((sf[89]+(if sb[49]{(v1wm/vj5)}else{v1rl}))-(if sb[49]{(sf[89]+(if sb[49]{(v1wg/vj2)}else{v1r9}))}else{v1rq})))+(vjs*v1py))/sf[88]))}else{(if sb[43]{(v3v*(((vgp*v1py)+(vgc*((sf[89]+v1rl)-v1rq)))/sf[88]))}else{vd})});
        let v1yc=(if sb[49]{(sf[183]+(v3v*(sf[182]+(((vjt*((if sb[49]{((vjf*sf[197])+(vja*((sf[185]*v1x0)*v1x5)))}else{vd})+((sf[177]+(if sb[49]{(v1wn/vj5)}else{v1rm}))-(if sb[49]{(sf[176]+(if sb[49]{(v1wh/vj2)}else{v1ra}))}else{v1rr}))))+(vjs*v1pz))/sf[88]))))}else{(if sb[43]{((v3v*((((vgp*v1pz)+(vgc*((sf[177]+v1rm)-v1rr)))/sf[88])+sf[182]))+sf[183])}else{vd})});let v1yd=(if sb[49]{(v3v*((vjt*(if sb[49]{(v1wo/vj5)}else{vd}))/sf[88]))}else{vd});
        let v1ye=(if sb[49]{(sf[100]+(v3v*(sf[104]+((vjt*((if sb[49]{((sf[112]*vjf)+(vja*((sf[186]*v1x0)*v1x5)))}else{vd})+(sf[88]+(if sb[49]{(v1wp/vj5)}else{v1rn}))))/sf[88]))))}else{v1si});
        let v1zq=(if sb[49]{(sf[184]+(v3w*(sf[182]+(((vk9*v1q3)+(vie*((sf[181]+(if sb[49]{((if sb[49]{v1st}else{v1wm})/vk3)}else{v1t4}))-(if sb[49]{(sf[176]+(if sb[49]{((if sb[49]{v1sk}else{v1wg})/vk0)}else{v1sq}))}else{v1ta}))))/sf[94]))))}else{(if sb[43]{((v3w*(sf[182]+(((vha*v1q3)+(vfc*((sf[181]+v1t4)-v1ta)))/sf[94])))+sf[184])}else{vd})});
        let v1zr=(if sb[49]{(v3w*(((vk9*v1q4)+(vie*((sf[89]+(if sb[49]{((if sb[49]{v1su}else{v1wn})/vk3)}else{v1t5}))-(if sb[49]{(sf[89]+(if sb[49]{((if sb[49]{v1sl}else{v1wh})/vk0)}else{v1sr}))}else{v1tb}))))/sf[94]))}else{(if sb[43]{(v3w*(((vha*v1q4)+(vfc*((sf[89]+v1t5)-v1tb)))/sf[94]))}else{vd})});let v1zs=(if sb[49]{(sf[102]+(v3w*(sf[104]+((vie*(sf[94]+(if sb[49]{((if sb[49]{v1sv}else{v1wo})/vk3)}else{v1t6})))/sf[94]))))}else{v1u4});
        let v1zt=(if sb[49]{(v3w*((vie*(if sb[49]{((if sb[49]{vd}else{v1wp})/vk3)}else{v1t7}))/sf[94]))}else{(if sb[43]{(v3w*((vfc*v1t7)/sf[94]))}else{vd})});let v20n=(v5*(if sb[49]{vd}else{(if sb[46]{(v3w*((vim*v1q3)+(vif*(if sb[46]{(sf[195]*v1vf)}else{v1q9}))))}else{(if sb[43]{vd}else{(if sb[40]{(v3w*((vfi*v1q3)+(vfc*v1q9)))}else{vd})})})}));
        let v20p=(v5*(if sb[49]{vd}else{(if sb[46]{(v3w*((vim*v1q4)+(vif*(if sb[46]{(sf[196]*v1vf)}else{v1qa}))))}else{(if sb[43]{vd}else{(if sb[40]{(v3w*((vfi*v1q4)+(vfc*v1qa)))}else{vd})})})}));let v20q=(v5*(if sb[49]{vd}else{(if sb[46]{(v3w*(vif*(if sb[46]{(sf[94]*v1vf)}else{v1qb})))}else{(if sb[43]{vd}else{(if sb[40]{(v3w*(vfc*v1qb))}else{vd})})})}));
        let v20y=(v9*(if sb[49]{vd}else{(if sb[46]{((viq*v1py)+(vic*(v3v*(if sb[46]{(sf[191]*v1v3)}else{v1ps}))))}else{(if sb[43]{vd}else{(if sb[40]{((vfy*v1py)+(vf5*(v3v*v1ps)))}else{vd})})})}));let v210=(v9*(if sb[49]{vd}else{(if sb[46]{((viq*v1pz)+(vic*(v3v*((if sb[46]{(sf[192]*v1v3)}else{v1pt})+(sf[112]*(if sb[46]{((vi3*(v1uh*v1um))+(vhz*(sf[110]*v1uh)))}else{vd}))))))}else{(if sb[43]{vd}else{(if sb[40]{((vfy*v1pz)+(vf5*(v3v*v1pt)))}else{vd})})})}));
        let v212=(v9*(if sb[49]{vd}else{(if sb[46]{(vic*(v3v*((if sb[46]{(sf[88]*v1v3)}else{v1pu})+(sf[112]*(if sb[46]{((vi3*(v1uj*v1um))+(vhz*(sf[110]*v1uj)))}else{vd})))))}else{(if sb[43]{vd}else{(if sb[40]{(vf5*(v3v*v1pu))}else{vd})})})}));let v21c=(-v3y);let v21l=(if (sf[122]!=0.0){(vmz*(if sb[28]{((-(v3x*v1by))/v1mq)}else{(if (sf[66]!=0.0){((-(v3x*vrr))/v1lv)}else{vd})}))}else{vd});
        let v21m=(if (sf[122]!=0.0){(vmz*(if sb[28]{((-(v3x*v1bz))/v1mq)}else{(if (sf[66]!=0.0){((-(v3x*vrs))/v1lv)}else{vd})}))}else{vd});let v21n=(if (sf[122]!=0.0){(vmz*(if sb[28]{((-(v3x*v1c0))/v1mq)}else{(if (sf[66]!=0.0){((-(v3x*vrt))/v1lv)}else{vd})}))}else{vd});let v21o=(if (sf[122]!=0.0){(vmz*(if sb[28]{((-(v3x*v1c1))/v1mq)}else{(if (sf[66]!=0.0){((-(v3x*vru))/v1lv)}else{vd})}))}else{vd});
        let v21p=(if (sf[122]!=0.0){(vmz*(if sb[28]{((-(v3x*v1c2))/v1mq)}else{(if (sf[66]!=0.0){((-(v3x*vrv))/v1lv)}else{vd})}))}else{vd});let v21v=(-v40);

        CommonStampValues {
            v0, v1, v3, v5, v6, v7, v8, v9, 
            vb, vd, vw, vz, v13, v18, v19, v1b, 
            v2c, v3c, v3g, v3n, v3s, v3y, v40, v45, 
            v4b, v4q, v4z, v54, v5e, v5m, v5v, v6u, 
            v6v, v6x, v8s, v9f, va6, vcl, vcz, vg1, 
            vg7, vis, vix, vjz, vkf, vlu, vm0, vma, 
            vmd, vms, vmu, vmw, vmx, vn0, vn1, vn8, 
            vnc, vne, vnu, vo0, vo6, vob, voj, vok, 
            vox, vpl, vpm, vq6, vqb, vqd, vqf, vqh, 
            vqi, vrr, vrs, vrt, vru, vrv, vuk, vul, 
            vum, vun, vuo, vup, vuq, vuz, vv0, vv1, 
            vv2, v14e, v14f, v14g, v14h, v14i, v176, v177, 
            v178, v179, v1by, v1bz, v1c0, v1c1, v1c2, v1si, 
            v1u4, v1yb, v1yc, v1yd, v1ye, v1zq, v1zr, v1zs, 
            v1zt, v20n, v20p, v20q, v20y, v210, v212, v21c, 
            v21l, v21m, v21n, v21o, v21p, v21v, 
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
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
            v0, v1, v3, v5, v6, v7, v8, v9, 
            vb, vd, vw, vz, v13, v18, v19, v1b, 
            v2c, v3c, v3g, v3n, v3s, v3y, v40, v45, 
            v4b, v4q, v4z, v54, v5e, v5m, v5v, v6u, 
            v6v, v6x, v8s, v9f, va6, vcl, vcz, vg1, 
            vg7, vis, vix, vjz, vkf, vlu, vm0, vma, 
            vmd, vms, vmu, vmw, vmx, vn0, vn1, vn8, 
            vnc, vne, vnu, vo0, vo6, vob, voj, vok, 
            vox, vpl, vpm, vq6, vqb, vqd, vqf, vqh, 
            vqi, vrr, vrs, vrt, vru, vrv, vuk, vul, 
            vum, vun, vuo, vup, vuq, vuz, vv0, vv1, 
            vv2, v14e, v14f, v14g, v14h, v14i, v176, v177, 
            v178, v179, v1by, v1bz, v1c0, v1c1, v1c2, v1si, 
            v1u4, v1yb, v1yc, v1yd, v1ye, v1zq, v1zr, v1zs, 
            v1zt, v20n, v20p, v20q, v20y, v210, v212, v21c, 
            v21l, v21m, v21n, v21o, v21p, v21v, 
        }=self.eval_common_stamp_values(ctx);
        let vc=ctx.node_voltage(nodes[16]);let v1e=(sf[11]*(v18+(sf[12]*v1b)));let v3t=(if v3s{sf[13]}else{(if (v19!=0.0){(sf[13]*(v18+(v1b*sf[14])))}else{vd})});let v3u=(if v3s{sf[15]}else{(if (v19!=0.0){(sf[15]*(v18+(v1b*sf[16])))}else{vd})});let v3z=(if v3s{sf[37]}else{(if v3n{(v2c*sf[37])}else{(if v3c{(sf[37]*v3g)}else{vd})})});let v44=(if v3s{sf[30]}else{(if (v19!=0.0){(sf[30]+(v13*sf[31]))}else{vd})});let v4i=(if sb[11]{sf[44]}else{(if (sf[41]!=0.0){(sf[43]/(vz*8.617333262145179e-5))}else{vd})});
        let v65=(sf[56]+(sf[53]*v5v));let v67=((v7*v65)).tanh();let v6i=(v3t*v5v);let v6j=(v67*v6i);let v6p=((v18+(v7*sf[62]))+(v3u*scalar_limexp(v5e)));let v6z=(if sb[16]{(v6v*v6x)}else{v5m});let v75=(if sb[16]{(((v4z*v6v)+(sf[55]*v6x))+(v54*v6z))}else{vd});let v76=(v75).tanh();let v78=(if sb[16]{(v18+v76)}else{vd});let v7b=(if sb[16]{(sf[56]+(sf[53]*v78))}else{vd});let v7e=(sf[62]+(v5v*sf[63]));let v7f=(if sb[16]{v7e}else{vd});let v7g=(v18+v67);let v7h=(v6i*v7g);let v7m=(sf[64]*(v7-v45));
        let v7o=(v3u*scalar_limexp(v7m));let v7p=((v18+(v7*v7f))+v7o);let v7r=(if sb[16]{(v7h*v7p)}else{vd});let v7u=(if sb[16]{(sf[62]+(v78*sf[63]))}else{vd});let v7w=((v7*v7b)).tanh();let v7y=(v3t*v78);let v7z=(v18-(if sb[16]{v7w}else{vd}));let v80=(v7y*v7z);let v82=(v18-(v7*v7u));let v84=(if sb[16]{(v80*v82)}else{vd});let v8v=(if sb[19]{(sf[56]+(sf[53]*v8s))}else{vd});let v8x=((v7*v8v)).tanh();let v8y=(if sb[19]{v8x}else{vd});let v91=(if sb[19]{(sf[62]+(sf[63]*v8s))}else{v7f});let v92=(v3t*v8s);
        let v93=(v8y*v92);let v96=(v5e*sf[64]);let v99=((v18+(v7*v91))+(v3u*scalar_limexp(v96)));let v9p=(if sb[22]{v6u}else{v6z});let v9r=(if sb[22]{(v9p*v9p)}else{vd});let v9u=(v54*v9p);let v9w=((v9p+(sf[55]*v9r))+(v9r*v9u));let v9y=(if sb[22]{(v4z*v9w)}else{v75});let va8=(-v9y);let vac=((v4b*(scalar_limexp(v9y)-scalar_limexp(va8)))).tanh();let vae=(if sb[22]{(v18+vac)}else{vd});let vag=(sf[56]+(sf[53]*va6));let vah=(if sb[22]{vag}else{v8v});let vak=(if sb[22]{(sf[56]+(sf[53]*vae))}else{vd});
        let vam=((v7*vah)).tanh();let van=(if sb[22]{vam}else{v8y});let vap=((v7*vak)).tanh();let vat=(if sb[22]{(sf[62]+(sf[63]*vae))}else{vd});let vaw=(if sb[22]{(sf[62]+(sf[63]*va6))}else{vd});let vax=(v3t*va6);let vay=(v18+van);let vaz=(vax*vay);let vb2=(v7o+(v18+(v7*vaw)));let vb5=(v3t*vae);let vb6=(v18-(if sb[22]{vap}else{vd}));let vb7=(vb5*vb6);let vb9=(v18-(v7*vat));let vbi=(if sb[25]{v7e}else{v91});let vbj=(if sb[25]{vag}else{vah});let vbl=((v7*vbj)).tanh();let vbo=((vb*vbj)).tanh();
        let vbs=((if sb[25]{vbl}else{van})+((if sb[25]{vbo}else{vd})*sf[65]));let vbt=(v6i*vbs);let vbv=(v7+(vb*sf[65]));let vby=(v7o+(v18+(vbi*vbv)));let vc0=(if sb[25]{(vbt*vby)}else{(if sb[22]{(v4b*((if sb[22]{(vaz*vb2)}else{v7r})-(if sb[22]{(vb7*vb9)}else{v84})))}else{(if sb[19]{(v93*v99)}else{(if sb[16]{(v4b*(v7r-v84))}else{(if (sf[57]!=0.0){(v6j*v6p)}else{vd})})})})});let vcb=(v5v*sf[69]);let vcm=(va6*sf[69]);let vct=(v18+(v1b*sf[71]));
        let vcu=((if sb[28]{(sf[70]+vcm)}else{(if (sf[66]!=0.0){(vcb+sf[70])}else{vd})})*vct);let vcv=((if sb[28]{(sf[68]+vcm)}else{(if (sf[66]!=0.0){(sf[68]+vcb)}else{vd})})*vct);let vd5=(v9-v44);let vd9=((-v9)-sf[74]);let vdb=(v5-v44);let vde=(v6-sf[75]);let vdk=(if sb[30]{scalar_limexp((v44*(-v4i)))}else{(if (sf[73]!=0.0){scalar_limexp((v4i*((-v44)).tanh()))}else{v9f})});let vdw=(vd5).tanh();let vdy=(vdb).tanh();let ve6=(sf[76]*(if sb[30]{vd9}else{(if (sf[73]!=0.0){vd9}else{vd})}));
        let vea=(v4i*(if sb[34]{vd5}else{(if sb[32]{vdw}else{(if (sf[73]!=0.0){vd5}else{vd})})}));let vei=(sf[85]*((scalar_limexp(vea)-((scalar_limexp(ve6)-sf[80])*sf[87]))-vdk));let vej=(sf[76]*(if sb[30]{vde}else{(if (sf[73]!=0.0){vde}else{vd})}));let vem=(v4i*(if sb[34]{vdb}else{(if sb[32]{vdy}else{(if (sf[73]!=0.0){vdb}else{vd})})}));let vhj=v1si;let vhl=v1u4;let vkg=v1ye;let vkh=(if sb[49]{vkg}else{(if sb[46]{vis}else{(if sb[43]{vhj}else{vg1})})});let vki=v1zs;
        let vkj=(if sb[49]{vki}else{(if sb[46]{vix}else{(if sb[43]{vhl}else{vg7})})});let vly=(if sb[67]{((v18-(vlu*vlu))).sqrt()}else{vd});let vm2=(if sb[67]{((-vlu)*vm0)}else{vd});let vme=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, vkf);
        let vmg=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, vjz);
        let vmk=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (v5*vkj));
        let vmn=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (v9*vkh));
        let vn2=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, vn1);
        let vn9=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, vn8);let vni=ctx.node_voltage(nodes[13]);let vnv=ctx.branch_current(branches[11]);let vo1=ctx.branch_current(branches[15]);let voc=(if sb[67]{vob}else{vd});let vod=ctx.node_voltage(nodes[18]);
        let vol=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, vok);
        let voy=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, vox);let vt8=(v18-(v67*v67));let vte=(v3t*vrr);let vtf=(v3t*vrs);let vtg=(v3t*vrt);let vth=(v3t*vru);let vti=(v3t*vrv);let vtj=(v6i*((v7*(sf[53]*vrr))*vt8));let vtm=(v6i*((v65+(v7*(sf[53]*vrs)))*vt8));let vtp=(v6i*(((-v65)+(v7*(sf[53]*vrt)))*vt8));
        let vts=(v6i*((v7*(sf[53]*vru))*vt8));let vtv=(v6i*((v7*(sf[53]*vrv))*vt8));let vtz=scalar_limexp_derivative(v5e);let vvh=(if sb[16]{((v6x*vun)+(v6v*vuz))}else{vqb});let vvi=(if sb[16]{((v6x*vuo)+(v6v*vv0))}else{vqd});let vvj=(if sb[16]{((v6x*vup)+(v6v*vv1))}else{vqf});let vvk=(if sb[16]{((v6x*vuq)+(v6v*vv2))}else{vqh});let vvl=(if sb[16]{(v6v*sf[158])}else{vqi});let vwd=(if sb[16]{(((v4z*vun)+(sf[55]*vuz))+(v54*vvh))}else{vd});
        let vwe=(if sb[16]{((((v6v*vpl)+(v4z*vuo))+(sf[55]*vv0))+(v54*vvi))}else{vd});let vwf=(if sb[16]{((((v6v*vpm)+(v4z*vup))+(sf[55]*vv1))+(v54*vvj))}else{vd});let vwg=(if sb[16]{(((v4z*vuq)+(sf[55]*vv2))+(v54*vvk))}else{vd});let vwh=(if sb[16]{(sf[159]+(v54*vvl))}else{vd});let vwj=(v18-(v76*v76));let vwp=(if sb[16]{(vwd*vwj)}else{vd});let vwq=(if sb[16]{(vwe*vwj)}else{vd});let vwr=(if sb[16]{(vwf*vwj)}else{vd});let vws=(if sb[16]{(vwg*vwj)}else{vd});let vwt=(if sb[16]{(vwh*vwj)}else{vd});
        let vx4=(sf[63]*vrr);let vx5=(sf[63]*vrs);let vx6=(sf[63]*vrt);let vx7=(sf[63]*vru);let vx8=(sf[63]*vrv);let vx9=(if sb[16]{vx4}else{vd});let vxa=(if sb[16]{vx5}else{vd});let vxb=(if sb[16]{vx6}else{vd});let vxc=(if sb[16]{vx7}else{vd});let vxd=(if sb[16]{vx8}else{vd});let vxx=scalar_limexp_derivative(v7m);let vy0=(v3u*(sf[64]*vxx));let vy1=(v3u*(sf[160]*vxx));let vyj=(if sb[16]{((v7p*(vtj+(v7g*vte)))+(v7h*(v7*vx9)))}else{vd});
        let vyk=(if sb[16]{((v7p*(vtm+(v7g*vtf)))+(v7h*((v7f+(v7*vxa))+vy0)))}else{vd});let vyl=(if sb[16]{((v7p*(vtp+(v7g*vtg)))+(v7h*(((-v7f)+(v7*vxb))+vy1)))}else{vd});let vym=(if sb[16]{((v7p*(vts+(v7g*vth)))+(v7h*(v7*vxc)))}else{vd});let vyn=(if sb[16]{((v7p*(vtv+(v7g*vti)))+(v7h*(v7*vxd)))}else{vd});let vz7=(v18-(v7w*v7w));let v10z=(if sb[16]{((v82*((v7z*(v3t*vwp))+(v7y*(-(if sb[16]{((v7*(if sb[16]{(sf[53]*vwp)}else{vd}))*vz7)}else{vd})))))+(v80*(-(v7*(if sb[16]{(sf[63]*vwp)}else{vd})))))}else{vd});
        let v110=(if sb[16]{((v82*((v7z*(v3t*vwq))+(v7y*(-(if sb[16]{((v7b+(v7*(if sb[16]{(sf[53]*vwq)}else{vd})))*vz7)}else{vd})))))+(v80*(-(v7u+(v7*(if sb[16]{(sf[63]*vwq)}else{vd}))))))}else{vd});let v111=(if sb[16]{((v82*((v7z*(v3t*vwr))+(v7y*(-(if sb[16]{(((-v7b)+(v7*(if sb[16]{(sf[53]*vwr)}else{vd})))*vz7)}else{vd})))))+(v80*(-((-v7u)+(v7*(if sb[16]{(sf[63]*vwr)}else{vd}))))))}else{vd});
        let v112=(if sb[16]{((v82*((v7z*(v3t*vws))+(v7y*(-(if sb[16]{((v7*(if sb[16]{(sf[53]*vws)}else{vd}))*vz7)}else{vd})))))+(v80*(-(v7*(if sb[16]{(sf[63]*vws)}else{vd})))))}else{vd});let v113=(if sb[16]{((v82*((v7z*(v3t*vwt))+(v7y*(-(if sb[16]{((v7*(if sb[16]{(sf[53]*vwt)}else{vd}))*vz7)}else{vd})))))+(v80*(-(v7*(if sb[16]{(sf[63]*vwt)}else{vd})))))}else{vd});let v14o=(if sb[19]{(sf[53]*v14e)}else{vd});let v14p=(if sb[19]{(sf[53]*v14f)}else{vd});let v14q=(if sb[19]{(sf[53]*v14g)}else{vd});
        let v14r=(if sb[19]{(sf[53]*v14h)}else{vd});let v14s=(if sb[19]{(sf[53]*v14i)}else{vd});let v152=(v18-(v8x*v8x));let v158=(if sb[19]{((v7*v14o)*v152)}else{vd});let v159=(if sb[19]{((v8v+(v7*v14p))*v152)}else{vd});let v15a=(if sb[19]{(((-v8v)+(v7*v14q))*v152)}else{vd});let v15b=(if sb[19]{((v7*v14r)*v152)}else{vd});let v15c=(if sb[19]{((v7*v14s)*v152)}else{vd});let v15i=(if sb[19]{(sf[63]*v14e)}else{vx9});let v15j=(if sb[19]{(sf[63]*v14f)}else{vxa});let v15k=(if sb[19]{(sf[63]*v14g)}else{vxb});
        let v15l=(if sb[19]{(sf[63]*v14h)}else{vxc});let v15m=(if sb[19]{(sf[63]*v14i)}else{vxd});let v16f=scalar_limexp_derivative(v96);let v193=(if sb[22]{vq6}else{vvh});let v194=(if sb[22]{vuk}else{vvi});let v195=(if sb[22]{vul}else{vvj});let v196=(if sb[22]{vum}else{vvk});let v197=(if sb[22]{vd}else{vvl});let v198=(v9p*v193);let v19a=(v9p*v194);let v19c=(v9p*v195);let v19e=(v9p*v196);let v19g=(v9p*v197);let v19i=(if sb[22]{(v198+v198)}else{vd});let v19j=(if sb[22]{(v19a+v19a)}else{vd});
        let v19k=(if sb[22]{(v19c+v19c)}else{vd});let v19l=(if sb[22]{(v19e+v19e)}else{vd});let v19m=(if sb[22]{(v19g+v19g)}else{vd});let v1av=(if sb[22]{(v4z*((v193+(sf[55]*v19i))+((v9u*v19i)+(v9r*(v54*v193)))))}else{vwd});let v1aw=(if sb[22]{((v9w*vpl)+(v4z*((v194+(sf[55]*v19j))+((v9u*v19j)+(v9r*(v54*v194))))))}else{vwe});let v1ax=(if sb[22]{((v9w*vpm)+(v4z*((v195+(sf[55]*v19k))+((v9u*v19k)+(v9r*(v54*v195))))))}else{vwf});
        let v1ay=(if sb[22]{(v4z*((v196+(sf[55]*v19l))+((v9u*v19l)+(v9r*(v54*v196)))))}else{vwg});let v1az=(if sb[22]{(v4z*((v197+(sf[55]*v19m))+((v9u*v19m)+(v9r*(v54*v197)))))}else{vwh});let v1c3=scalar_limexp_derivative(v9y);let v1ce=scalar_limexp_derivative(va8);let v1cv=(v18-(vac*vac));let v1d1=(if sb[22]{((v4b*((v1av*v1c3)-((-v1av)*v1ce)))*v1cv)}else{vd});let v1d2=(if sb[22]{((v4b*((v1aw*v1c3)-((-v1aw)*v1ce)))*v1cv)}else{vd});let v1d3=(if sb[22]{((v4b*((v1ax*v1c3)-((-v1ax)*v1ce)))*v1cv)}else{vd});
        let v1d4=(if sb[22]{((v4b*((v1ay*v1c3)-((-v1ay)*v1ce)))*v1cv)}else{vd});let v1d5=(if sb[22]{((v4b*((v1az*v1c3)-((-v1az)*v1ce)))*v1cv)}else{vd});let v1d6=(sf[53]*v1by);let v1d7=(sf[53]*v1bz);let v1d8=(sf[53]*v1c0);let v1d9=(sf[53]*v1c1);let v1da=(sf[53]*v1c2);let v1db=(if sb[22]{v1d6}else{v14o});let v1dc=(if sb[22]{v1d7}else{v14p});let v1dd=(if sb[22]{v1d8}else{v14q});let v1de=(if sb[22]{v1d9}else{v14r});let v1df=(if sb[22]{v1da}else{v14s});let v1dz=(v18-(vam*vam));
        let v1e5=(if sb[22]{((v7*v1db)*v1dz)}else{v158});let v1e6=(if sb[22]{((vah+(v7*v1dc))*v1dz)}else{v159});let v1e7=(if sb[22]{(((-vah)+(v7*v1dd))*v1dz)}else{v15a});let v1e8=(if sb[22]{((v7*v1de)*v1dz)}else{v15b});let v1e9=(if sb[22]{((v7*v1df)*v1dz)}else{v15c});let v1ej=(v18-(vap*vap));
        let v1ip=(if sb[22]{(v4b*((if sb[22]{((vb2*((vay*(v3t*v1bz))+(vax*v1e6)))+(vaz*(vy0+(vaw+(v7*(if sb[22]{(sf[63]*v1bz)}else{vd}))))))}else{vyk})-(if sb[22]{((vb9*((vb6*(v3t*v1d2))+(vb5*(-(if sb[22]{((vak+(v7*(if sb[22]{(sf[53]*v1d2)}else{vd})))*v1ej)}else{vd})))))+(vb7*(-(vat+(v7*(if sb[22]{(sf[63]*v1d2)}else{vd}))))))}else{v110})))}else{(if sb[19]{((v99*((v92*v159)+(v8y*(v3t*v14f))))+(v93*((v91+(v7*v15j))+(v3u*(sf[64]*v16f)))))}else{(if sb[16]{(v4b*(vyk-v110))}else{(if (sf[57]!=0.0){((v6p*(vtm+(v67*vtf)))+(v6j*(sf[62]+(v3u*vtz))))}else{vd})})})});
        let v1iy=(if sb[25]{v1d6}else{v1db});let v1iz=(if sb[25]{v1d7}else{v1dc});let v1j0=(if sb[25]{v1d8}else{v1dd});let v1j1=(if sb[25]{v1d9}else{v1de});let v1j2=(if sb[25]{v1da}else{v1df});let v1j6=(-vbj);let v1jc=(v18-(vbl*vbl));let v1jv=(v18-(vbo*vbo));
        let v1lo=(if sb[25]{((vby*((vbs*vte)+(v6i*((if sb[25]{((v7*v1iy)*v1jc)}else{v1e5})+(sf[65]*(if sb[25]{((vbj+(vb*v1iy))*v1jv)}else{vd}))))))+(vbt*((vbv*(if sb[25]{vx4}else{v15i}))+(vbi*sf[65]))))}else{(if sb[22]{(v4b*((if sb[22]{((vb2*((vay*(v3t*v1by))+(vax*v1e5)))+(vaz*(v7*(if sb[22]{(sf[63]*v1by)}else{vd}))))}else{vyj})-(if sb[22]{((vb9*((vb6*(v3t*v1d1))+(vb5*(-(if sb[22]{((v7*(if sb[22]{(sf[53]*v1d1)}else{vd}))*v1ej)}else{vd})))))+(vb7*(-(v7*(if sb[22]{(sf[63]*v1d1)}else{vd})))))}else{v10z})))}else{(if sb[19]{((v99*((v92*v158)+(v8y*(v3t*v14e))))+(v93*(v7*v15i)))}else{(if sb[16]{(v4b*(vyj-v10z))}else{(if (sf[57]!=0.0){(v6p*(vtj+(v67*vte)))}else{vd})})})})});
        let v1lq=(if sb[25]{((vby*((vbs*vtg)+(v6i*((if sb[25]{((v1j6+(v7*v1j0))*v1jc)}else{v1e7})+(sf[65]*(if sb[25]{((v1j6+(vb*v1j0))*v1jv)}else{vd}))))))+(vbt*(vy1+((vbv*(if sb[25]{vx6}else{v15k}))+(vbi*sf[164])))))}else{(if sb[22]{(v4b*((if sb[22]{((vb2*((vay*(v3t*v1c0))+(vax*v1e7)))+(vaz*(vy1+((-vaw)+(v7*(if sb[22]{(sf[63]*v1c0)}else{vd}))))))}else{vyl})-(if sb[22]{((vb9*((vb6*(v3t*v1d3))+(vb5*(-(if sb[22]{(((-vak)+(v7*(if sb[22]{(sf[53]*v1d3)}else{vd})))*v1ej)}else{vd})))))+(vb7*(-((-vat)+(v7*(if sb[22]{(sf[63]*v1d3)}else{vd}))))))}else{v111})))}else{(if sb[19]{((v99*((v92*v15a)+(v8y*(v3t*v14g))))+(v93*((-v91)+(v7*v15k))))}else{(if sb[16]{(v4b*(vyl-v111))}else{(if (sf[57]!=0.0){((v6p*(vtp+(v67*vtg)))+(v6j*sf[157]))}else{vd})})})})});
        let v1lr=(if sb[25]{((vby*((vbs*vth)+(v6i*((if sb[25]{((v7*v1j1)*v1jc)}else{v1e8})+(sf[65]*(if sb[25]{((vb*v1j1)*v1jv)}else{vd}))))))+(vbt*(vbv*(if sb[25]{vx7}else{v15l}))))}else{(if sb[22]{(v4b*((if sb[22]{((vb2*((vay*(v3t*v1c1))+(vax*v1e8)))+(vaz*(v7*(if sb[22]{(sf[63]*v1c1)}else{vd}))))}else{vym})-(if sb[22]{((vb9*((vb6*(v3t*v1d4))+(vb5*(-(if sb[22]{((v7*(if sb[22]{(sf[53]*v1d4)}else{vd}))*v1ej)}else{vd})))))+(vb7*(-(v7*(if sb[22]{(sf[63]*v1d4)}else{vd})))))}else{v112})))}else{(if sb[19]{((v99*((v92*v15b)+(v8y*(v3t*v14h))))+(v93*((v7*v15l)+(v3u*(sf[160]*v16f)))))}else{(if sb[16]{(v4b*(vym-v112))}else{(if (sf[57]!=0.0){((v6p*(vts+(v67*vth)))+(v6j*(v3u*(-vtz))))}else{vd})})})})});
        let v1ls=(if sb[25]{((vby*((vbs*vti)+(v6i*((if sb[25]{((v7*v1j2)*v1jc)}else{v1e9})+(sf[65]*(if sb[25]{((vb*v1j2)*v1jv)}else{vd}))))))+(vbt*(vbv*(if sb[25]{vx8}else{v15m}))))}else{(if sb[22]{(v4b*((if sb[22]{((vb2*((vay*(v3t*v1c2))+(vax*v1e9)))+(vaz*(v7*(if sb[22]{(sf[63]*v1c2)}else{vd}))))}else{vyn})-(if sb[22]{((vb9*((vb6*(v3t*v1d5))+(vb5*(-(if sb[22]{((v7*(if sb[22]{(sf[53]*v1d5)}else{vd}))*v1ej)}else{vd})))))+(vb7*(-(v7*(if sb[22]{(sf[63]*v1d5)}else{vd})))))}else{v113})))}else{(if sb[19]{((v99*((v92*v15c)+(v8y*(v3t*v14i))))+(v93*(v7*v15m)))}else{(if sb[16]{(v4b*(vyn-v113))}else{(if (sf[57]!=0.0){(v6p*(vtv+(v67*vti)))}else{vd})})})})});
        let v1nj=(vct*(if sb[28]{(sf[69]*v1by)}else{(if (sf[66]!=0.0){(sf[69]*vrr)}else{vd})}));let v1nk=(vct*(if sb[28]{(sf[69]*v1bz)}else{(if (sf[66]!=0.0){(sf[69]*vrs)}else{vd})}));let v1nl=(vct*(if sb[28]{(sf[69]*v1c0)}else{(if (sf[66]!=0.0){(sf[69]*vrt)}else{vd})}));let v1nm=(vct*(if sb[28]{(sf[69]*v1c1)}else{(if (sf[66]!=0.0){(sf[69]*vru)}else{vd})}));let v1nn=(vct*(if sb[28]{(sf[69]*v1c2)}else{(if (sf[66]!=0.0){(sf[69]*vrv)}else{vd})}));let v1nw=(if sb[30]{vd}else{(if (sf[73]!=0.0){vd}else{v177})});
        let v1nx=(if sb[30]{vd}else{(if (sf[73]!=0.0){vd}else{v178})});let v1ny=(if sb[30]{vd}else{(if (sf[73]!=0.0){vd}else{v179})});let v1o1=(v18-(vdw*vdw));let v1o6=(v18-(vdy*vdy));let v1oi=scalar_limexp_derivative(ve6);let v1on=scalar_limexp_derivative(vea);let v1oz=(sf[85]*(-(if sb[30]{vd}else{(if (sf[73]!=0.0){vd}else{v176})})));let v1p5=scalar_limexp_derivative(vej);let v1pa=scalar_limexp_derivative(vem);let v205=ddt_scale;let v21d=-1e-12;

        stamper.stamp_current_sparse_local::<5, 0>(
            Some(15),
            None,
            multiplicity * ((-vc0)),
            [4, 5, 8, 10, 12],
            [(-v1lo), (-(if sb[25]{((vby*((vbs*vtf)+(v6i*((if sb[25]{((vbj+(v7*v1iz))*v1jc)}else{v1e6})+(sf[65]*(if sb[25]{((vb*v1iz)*v1jv)}else{vd}))))))+(vbt*(vy0+(vbi+(vbv*(if sb[25]{vx5}else{v15j}))))))}else{v1ip})), (-v1lq), (-v1lr), (-v1ls)],
            [],
            [],
            multiplicity,
        );
        let vma_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, vma);
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (vma_ddt),
            15,
            multiplicity * (((sf[144]) * ddt_scale)),
        );
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (vc),
            16,
            multiplicity * (v18),
        );
        let vmd_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, vmd);
        stamper.stamp_potential_branch_local(
            Some(15),
            Some(16),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            0,
            vmd_ddt,
            0,
            ((sf[145]) * ddt_scale),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            Some(8),
            multiplicity * (vc),
            16,
            multiplicity * (v18),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(8),
            multiplicity * (vei),
            [4, 5, 8, 10, 11, 12],
            [v1oz, (sf[85]*(-v1nw)), (sf[85]*((((v4i*(if sb[34]{vcz}else{(if sb[32]{(-v1o1)}else{sf[166]})}))*v1on)-(sf[87]*(sf[171]*v1oi)))-v1nx)), (sf[85]*(-v1ny)), (sf[85]*(((v4i*(if sb[34]{v18}else{(if sb[32]{v1o1}else{sf[167]})}))*v1on)-(sf[87]*(sf[172]*v1oi)))), sf[174]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * ((sf[85]*((scalar_limexp(vem)-(sf[87]*(scalar_limexp(vej)-sf[83])))-vdk))),
            [4, 5, 8, 10, 12],
            [v1oz, (sf[85]*((((v4i*(if sb[34]{vcz}else{(if sb[32]{(-v1o6)}else{sf[166]})}))*v1pa)-(sf[87]*(sf[171]*v1p5)))-v1nw)), (sf[85]*(-v1nx)), (sf[85]*((((v4i*(if sb[34]{v18}else{(if sb[32]{v1o6}else{sf[167]})}))*v1pa)-(sf[87]*(sf[172]*v1p5)))-v1ny)), sf[174]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(10),
            Some(5),
            multiplicity * ((if (sf[120]!=0.0){vme}else{vd})),
            [5, 8, 10, 11],
            [(if (sf[120]!=0.0){(v1zq*v205)}else{vd}), (if (sf[120]!=0.0){(v1zr*v205)}else{vd}), (if (sf[120]!=0.0){(v1zs*v205)}else{vd}), (if (sf[120]!=0.0){(v1zt*v205)}else{vd})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(8),
            multiplicity * ((if (sf[120]!=0.0){vmg}else{vd})),
            [5, 8, 10, 11],
            [(if (sf[120]!=0.0){(v1yb*v205)}else{vd}), (if (sf[120]!=0.0){(v1yc*v205)}else{vd}), (if (sf[120]!=0.0){(v1yd*v205)}else{vd}), (if (sf[120]!=0.0){(v1ye*v205)}else{vd})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(10),
            Some(5),
            multiplicity * ((if sb[69]{vmk}else{vd})),
            5,
            multiplicity * ((if sb[69]{(v205*((-vkj)+v20n))}else{vd})),
            8,
            multiplicity * ((if sb[69]{(v205*v20p)}else{vd})),
            10,
            multiplicity * ((if sb[69]{(v205*(vkj+v20q))}else{vd})),
        );
        stamper.stamp_current_node3_local(
            Some(11),
            Some(8),
            multiplicity * ((if sb[69]{vmn}else{vd})),
            5,
            multiplicity * ((if sb[69]{(v205*v20y)}else{vd})),
            8,
            multiplicity * ((if sb[69]{(v205*((-vkh)+v210))}else{vd})),
            11,
            multiplicity * ((if sb[69]{(v205*(vkh+v212))}else{vd})),
        );
        let vms_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, vms);
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (vms_ddt),
            5,
            multiplicity * (((sf[199]) * ddt_scale)),
            7,
            multiplicity * (((sf[146]) * ddt_scale)),
        );
        let vmu_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, vmu);
        stamper.stamp_current_node2_local(
            Some(5),
            Some(8),
            multiplicity * (vmu_ddt),
            5,
            multiplicity * (((sf[147]) * ddt_scale)),
            8,
            multiplicity * (((sf[200]) * ddt_scale)),
        );
        let vmx_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, vmx);
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (vmx_ddt),
            4,
            multiplicity * (((v21c) * ddt_scale)),
            6,
            multiplicity * (((v3y) * ddt_scale)),
        );
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * ((v4q*vmw)),
            4,
            multiplicity * (v21d),
            6,
            multiplicity * (v4q),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            1,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            1,
            (if (sf[122]!=0.0){(vn0+vn2)}else{vd}),
            [4, 5, 8, 10, 12],
            [v21l, v21m, v21n, v21o, v21p],
            [1],
            [(if (sf[122]!=0.0){(vcl+(sf[121]*v205))}else{vd})],
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            vd,
        );
        stamper.stamp_current_node2_local(
            Some(11),
            Some(12),
            multiplicity * ((if (sf[123]!=0.0){((v8-v0)/v3z)}else{vd})),
            11,
            multiplicity * ((if (sf[123]!=0.0){(v18/v3z)}else{vd})),
            12,
            multiplicity * ((if (sf[123]!=0.0){(vcz/v3z)}else{vd})),
        );
        stamper.stamp_current_node2_local(
            Some(12),
            Some(8),
            multiplicity * ((if (sf[123]!=0.0){vn9}else{vd})),
            8,
            multiplicity * ((if (sf[123]!=0.0){(v205*v21v)}else{vd})),
            12,
            multiplicity * ((if (sf[123]!=0.0){(v40*v205)}else{vd})),
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(8),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            vd,
        );
        let vne_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, vne);
        stamper.stamp_current_node2_local(
            Some(11),
            Some(14),
            multiplicity * (vne_ddt),
            11,
            multiplicity * (((sf[148]) * ddt_scale)),
            14,
            multiplicity * (((sf[201]) * ddt_scale)),
        );
        stamper.stamp_current_node2_local(
            Some(14),
            Some(8),
            multiplicity * ((if (sf[125]!=0.0){((vnc-v1)/sf[124])}else{vd})),
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
            vd,
        );
        stamper.stamp_current_node2_local(
            Some(13),
            Some(10),
            multiplicity * ((if (sf[127]!=0.0){((vni-v3)/sf[126])}else{vd})),
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
            vd,
        );
        stamper.stamp_current_const_local(
            Some(13),
            Some(10),
            multiplicity * (vd),
        );
        stamper.stamp_current_node2_local(
            Some(13),
            Some(11),
            multiplicity * ((if (sf[129]!=0.0){((vni-v8)/sf[128])}else{vd})),
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
            vd,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            7,
            (if (sf[131]!=0.0){(sf[130]*ctx.branch_current(branches[7]))}else{vd}),
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
            vd,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            vd,
        );
        let vnu_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, vnu);
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(7),
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            10,
            vnu_ddt,
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
            (if (sf[132]!=0.0){(vcu*vnv)}else{vd}),
            [4, 5, 8, 10, 12],
            [(if (sf[132]!=0.0){(vnv*v1nj)}else{vd}), (if (sf[132]!=0.0){(vnv*v1nk)}else{vd}), (if (sf[132]!=0.0){(vnv*v1nl)}else{vd}), (if (sf[132]!=0.0){(vnv*v1nm)}else{vd}), (if (sf[132]!=0.0){(vnv*v1nn)}else{vd})],
            [11],
            [(if (sf[132]!=0.0){vcu}else{vd})],
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            vd,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            13,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            13,
            vd,
        );
        let vo0_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, vo0);
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(2),
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            14,
            vo0_ddt,
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
            (if (sf[133]!=0.0){(vcv*vo1)}else{vd}),
            [4, 5, 8, 10, 12],
            [(if (sf[133]!=0.0){(vo1*v1nj)}else{vd}), (if (sf[133]!=0.0){(vo1*v1nk)}else{vd}), (if (sf[133]!=0.0){(vo1*v1nl)}else{vd}), (if (sf[133]!=0.0){(vo1*v1nm)}else{vd}), (if (sf[133]!=0.0){(vo1*v1nn)}else{vd})],
            [15],
            [(if (sf[133]!=0.0){vcv}else{vd})],
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            vd,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            17,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            17,
            vd,
        );
        let vo6_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, vo6);
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(0),
            18,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            18,
            vo6_ddt,
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
            multiplicity * (v4q),
        );
        stamper.stamp_current_node2_local(
            Some(12),
            Some(2),
            multiplicity * ((v4q*(v0-ctx.node_voltage(nodes[2])))),
            2,
            multiplicity * (v21d),
            12,
            multiplicity * (v4q),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (vd),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (vd),
        );
        stamper.stamp_current_const_local(
            Some(17),
            None,
            multiplicity * (vd),
        );
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (voc),
            17,
            multiplicity * (sf[215]),
        );
        stamper.stamp_current_const_local(
            Some(18),
            None,
            multiplicity * (vd),
        );
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * ((if sb[67]{vod}else{vd})),
            18,
            multiplicity * (sf[215]),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(8),
            multiplicity * (voc),
            17,
            multiplicity * (sf[215]),
        );
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * ((if sb[67]{((vm2*vob)+(vly*vod))}else{vd})),
            17,
            multiplicity * ((if sb[67]{vm2}else{vd})),
            18,
            multiplicity * ((if sb[67]{vly}else{vd})),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(5),
            multiplicity * ((if sb[67]{vol}else{vd})),
            17,
            multiplicity * ((if sb[67]{(voj*v205)}else{vd})),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (vd),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (vd),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (vd),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (vd),
        );
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (vob),
            17,
            multiplicity * (v18),
        );
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (vod),
            18,
            multiplicity * (v18),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (vd),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (vd),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (vd),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (vd),
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * ((if (sf[143]!=0.0){(-(((v7*vc0)).abs()+((v9*vei)).abs()))}else{vd})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if (sf[143]!=0.0){(vw/v1e)}else{vd})),
            3,
            multiplicity * ((if (sf[143]!=0.0){(v18/v1e)}else{vd})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if (sf[143]!=0.0){voy}else{vd})),
            3,
            multiplicity * ((if (sf[143]!=0.0){(sf[152]*v205)}else{vd})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if sb[70]{(vw*v4q)}else{vd})),
            3,
            multiplicity * (sf[216]),
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
            v0, v1, v3, v5, v6, v7, v8, v9, 
            vb, vd, vw, vz, v13, v18, v19, v1b, 
            v2c, v3c, v3g, v3n, v3s, v3y, v40, v45, 
            v4b, v4q, v4z, v54, v5e, v5m, v5v, v6u, 
            v6v, v6x, v8s, v9f, va6, vcl, vcz, vg1, 
            vg7, vis, vix, vjz, vkf, vlu, vm0, vma, 
            vmd, vms, vmu, vmw, vmx, vn0, vn1, vn8, 
            vnc, vne, vnu, vo0, vo6, vob, voj, vok, 
            vox, vpl, vpm, vq6, vqb, vqd, vqf, vqh, 
            vqi, vrr, vrs, vrt, vru, vrv, vuk, vul, 
            vum, vun, vuo, vup, vuq, vuz, vv0, vv1, 
            vv2, v14e, v14f, v14g, v14h, v14i, v176, v177, 
            v178, v179, v1by, v1bz, v1c0, v1c1, v1c2, v1si, 
            v1u4, v1yb, v1yc, v1yd, v1ye, v1zq, v1zr, v1zs, 
            v1zt, v20n, v20p, v20q, v20y, v210, v212, v21c, 
            v21l, v21m, v21n, v21o, v21p, v21v, 
        }=self.eval_common_stamp_values(ctx);
        let vhj=v1si;let vhl=v1u4;let vkg=v1ye;let vkh=(if sb[49]{vkg}else{(if sb[46]{vis}else{(if sb[43]{vhj}else{vg1})})});let vki=v1zs;let vkj=(if sb[49]{vki}else{(if sb[46]{vix}else{(if sb[43]{vhl}else{vg7})})});let vme=0.0;let vmg=0.0;let vmk=0.0;let vmn=0.0;let vn2=0.0;let vn9=0.0;let vol=0.0;let voy=0.0;let v205=1.0;

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
            &[(if (sf[120]!=0.0){(v1zq*v205)}else{vd}), (if (sf[120]!=0.0){(v1zr*v205)}else{vd}), (if (sf[120]!=0.0){(v1zs*v205)}else{vd}), (if (sf[120]!=0.0){(v1zt*v205)}else{vd})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            &[nodes[5], nodes[8], nodes[10], nodes[11]],
            &[(if (sf[120]!=0.0){(v1yb*v205)}else{vd}), (if (sf[120]!=0.0){(v1yc*v205)}else{vd}), (if (sf[120]!=0.0){(v1yd*v205)}else{vd}), (if (sf[120]!=0.0){(v1ye*v205)}else{vd})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * ((if sb[69]{(v205*((-vkj)+v20n))}else{vd})),
            nodes[8],
            multiplicity * ((if sb[69]{(v205*v20p)}else{vd})),
            nodes[10],
            multiplicity * ((if sb[69]{(v205*(vkj+v20q))}else{vd})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes[5],
            multiplicity * ((if sb[69]{(v205*v20y)}else{vd})),
            nodes[8],
            multiplicity * ((if sb[69]{(v205*((-vkh)+v210))}else{vd})),
            nodes[11],
            multiplicity * ((if sb[69]{(v205*(vkh+v212))}else{vd})),
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
            multiplicity * (v21c),
            nodes[6],
            multiplicity * (v3y),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[8]),
            &[nodes[4], nodes[5], nodes[8], nodes[10], nodes[12]],
            &[v21l, v21m, v21n, v21o, v21p],
            &[branches[1]],
            &[(if (sf[122]!=0.0){(vcl+(sf[121]*v205))}else{vd})],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[12]),
            Some(nodes[8]),
            nodes[8],
            multiplicity * ((if (sf[123]!=0.0){(v205*v21v)}else{vd})),
            nodes[12],
            multiplicity * ((if (sf[123]!=0.0){(v40*v205)}else{vd})),
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
            multiplicity * ((if sb[67]{(voj*v205)}else{vd})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * ((if (sf[143]!=0.0){(sf[152]*v205)}else{vd})),
        );
    }
}
